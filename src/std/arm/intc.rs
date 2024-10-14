use crate::core::model::dev::*;
use crate::core::model::proc::ProcInfo;
use crate::std::arm::cpu::ArmCpu;

struct ArmCpuIntrs {
    fiq: IntrInput,
    irq: IntrInput,
    serr: IntrInput,
}

pub struct Gicv2 {
    bus: MemParent,
    base: usize,
    ram: Vec<u8>,
    ctlr: [u8; 4],
    pending: Option<u32>,
    active: Option<u32>,
    cpus: Vec<ArmCpuIntrs>,
}

impl Gicv2 {
    pub fn new<'a>(
        bus: MemParent,
        base: usize,
        cpus: impl IntoIterator<Item = &'a ProcInfo<ArmCpu>>,
    ) -> Self {
        let cpus = cpus
            .into_iter()
            .map(|p| ArmCpuIntrs {
                fiq: p.intr_in("FIQ", 0),
                irq: p.intr_in("IRQ", 0),
                serr: p.intr_in("SError", 0),
            })
            .collect::<Vec<ArmCpuIntrs>>();

        if cpus.len() < 1 || cpus.len() > 8 {
            panic!(
                "GICv2 controls between 1 and 8 ARM CPUs. This GICv2 was initialized for {0} ARM CPUs.",
                cpus.len()
            )
        }

        Self {
            bus,
            base,
            ram: vec![0; 0x3000],
            ctlr: [0; 4],
            pending: None,
            active: None,
            cpus,
        }
    }

    // const TYPER: [u8; 4] = [0b0, 0b0, 0b100, 0b00000111];
    const TYPER: [u8; 4] = [0b00000111, 0b100, 0b0, 0b0];
    // const IIDR: [u8; 4] = [0x00, 0x02, 0x04, 0x3B];
    const IIDR: [u8; 4] = [0x3B, 0x04, 0x02, 0x00];
    // const PENDING_NONE: [u8; 4] = [0x00, 0x00, 0x03, 0xff];
    const PENDING_NONE: [u8; 4] = [0xff, 0x03, 0x00, 0x00];
    const CNTP_ID: [u8; 4] = [0x1d, 0x00, 0x00, 0x00];
    const CNTV_ID: [u8; 4] = [0x1b, 0x00, 0x00, 0x00];
    const CNTHP_ID: [u8; 4] = [0x1a, 0x00, 0x00, 0x00];

    fn read_ram(&self, offset: usize) -> Option<&[u8]> {
        self.ram.get(offset..offset + 4)
    }

    fn read_only(&self, offset: usize) {
        // eprintln!("[GIC] Write to RO offset: 0x{offset:x}");
    }

    fn interrupt_id(&self, intid: u32) -> [u8; 4] {
        // SGI Interrupts are 0-15, PPI interrupts are 16-31, and SPI interrupts have an offset of 32.
        // let PPI_OFFSET = 16;
        match intid {
            13 => Gicv2::CNTP_ID,  // PPI_OFFSET + 13
            11 => Gicv2::CNTHP_ID, // PPI_OFFSET + 11
            14 => Gicv2::CNTV_ID,  // PPI_OFFSET + 14
            _ => Gicv2::PENDING_NONE,
        }
    }

    // fn is_interrupt_pending(&self) -> bool {
    //     self.pending.is_some()
    // }

    fn acknowledge_interrupt(&mut self) -> [u8; 4] {
        // Return current pending interrupt, if any, and move it to active
        if let Some(intid) = self.pending {
            self.pending = None;
            self.active = Some(intid);
            self.interrupt_id(intid)
        } else {
            Gicv2::PENDING_NONE
        }
    }

    // fn clear_pending_interrupt(&mut self, intid: [u8; 4]) {
    //     if let Some(pending_intid) = self.pending {
    //         if pending_intid.to_le_bytes() == intid {
    //             self.pending = None;
    //         }
    //     }
    // }

    fn clear_active_interrupt(&mut self, intid: [u8; 4]) {
        if let Some(active_intid) = self.active {
            // if active_intid.to_le_bytes() == intid {
            //     self.active = None;
            // }
            self.active = None;
        }
    }

    // Temporary helper for dealing with the current DevDecl-MMIO interface
    fn to_result(&self, data: Option<[u8; 4]>) -> MemReadResult {
        let res = data.and_then(|data| {
            let mut res = [0; 16];
            (&mut res[0..4]).copy_from_slice(&data);
            Some(res)
        });
        MemReadResult(res)
    }

    // MMIO interfaces

    const DISTRIB_MMIO_SIZE: usize = 0x1000;

    pub fn distrib_read(&mut self, offset: usize, _size: usize, _: &mut DevCtx) -> MemReadResult {
        match offset {
            0x0004 => self.to_result(Some(Gicv2::TYPER)),
            // Send all interrupts to CPU interface 0
            0x0800 => self.to_result(Some([0xff, 0xff, 0xff, 0xff])),
            // Linux timer
            0x0C04 => {
                let data = self.read_ram(0x1C04).unwrap();
                let data = data.try_into().unwrap();
                // eprintln!("[GIC] Read 1C04: 0x{0:x}", u32::from_le_bytes(data));
                self.to_result(Some(data))
            }
            _ => {
                // eprintln!("[GIC] Read offset: 0x{offset:x}");
                self.to_result(Some([0; 4]))
            }
        }
    }

    pub fn distrib_write(&mut self, offset: usize, data: &[u8], _: &mut DevCtx) -> MemWriteResult {
        let data_bytes = data.try_into().unwrap();
        let data_u32 = u32::from_le_bytes(data_bytes);

        match offset {
            0x0004 => self.read_only(offset),
            0x0100 => {
                // eprintln!("[GIC] Registering interrupts 0x{data_u32:x}");
                // let intID = HighestSetBit(data);
                let int_id = 31 - data_u32.leading_zeros();
                // eprintln!("[GIC] Registering interrupt {int_id}");
            }
            0x0800 => self.read_only(offset),
            // We don't exhaustively model the GIC, so log and forward unrecognised writes to memory
            _ => {
                // eprintln!("[GIC] Unknown write offset: 0x{offset:x}");
                // eprintln!("[GIC] Unknown write data: 0x{data_u32:x}");
            }
        };
        MemWriteResult(Some(()))
    }

    const CPU_IFACE_MMIO_SIZE: usize = 0x2000;

    pub fn cpu_iface_read(&mut self, offset: usize, _size: usize, _: &mut DevCtx) -> MemReadResult {
        match offset {
            0x0000 => {
                // eprintln!(
                //     "[GIC] Read GICC_CTLR 0x{0:x}",
                //     u32::from_le_bytes(self.ctlr)
                // );
                self.to_result(Some(self.ctlr))
            }
            0x000C => {
                let intid = self.acknowledge_interrupt();
                // eprintln!(
                //     "[GIC] Acknowledged interrupt {0}",
                //     u32::from_le_bytes(intid)
                // );
                self.to_result(Some(intid))
            }
            0x00FC => self.to_result(Some(Gicv2::IIDR)),
            _ => {
                // eprintln!("[GIC] Read offset: 0x{offset:x}");
                self.to_result(Some([0; 4]))
            }
        }
    }

    pub fn cpu_iface_write(
        &mut self,
        offset: usize,
        data: &[u8],
        ctx: &mut DevCtx,
    ) -> MemWriteResult {
        let data_bytes = data.try_into().unwrap();
        let data_u32 = u32::from_le_bytes(data_bytes);

        match offset {
            0x0000 => {
                // eprintln!("[GIC] GICC_CTLR = 0x{data_u32:x}");
                self.ctlr = data_bytes;
            }
            0x000C => self.read_only(offset),
            0x00FC => self.read_only(offset),
            0x0010 => {
                // eprintln!("[GIC] End of interrupt = 0x{data_u32:x}");
                self.clear_active_interrupt(data_bytes);
                // TODO: assumes only one cpu
                ctx.set_intr(self.cpus.first().unwrap().irq, false);
            }
            0x1000 => {
                // eprintln!("[GIC] Deactivating interrupt 0x{data_u32:x}");
                self.clear_active_interrupt(data_bytes);
                // TODO: assumes only one cpu
                ctx.set_intr(self.cpus.first().unwrap().irq, false);
            }
            // We don't exhaustively model the GIC, so log and forward unrecognised writes to memory
            _ => {
                // eprintln!("[GIC] Unknown write offset: 0x{offset:x}");
                // eprintln!("[GIC] Unknown write data: 0x{data_u32:x}");
            }
        };
        MemWriteResult(Some(()))
    }

    // Interrupt input interfaces

    pub fn handle_spi(&mut self, n: u32, level: bool, ctx: &mut DevCtx) {
        todo!()
    }

    pub fn handle_ppi(&mut self, n: u32, level: bool, ctx: &mut DevCtx) {
        if level && self.active != Some(n) {
            self.pending = Some(n);
            ctx.set_intr(self.cpus.first().unwrap().irq, level)
        }
    }
}

impl Device for Gicv2 {
    fn declare(&self, decl: &mut DevDecl<Self>) {
        let decl = decl
            .mmio(
                "GICv2 Distributor Regs",
                self.bus,
                self.base,
                Gicv2::DISTRIB_MMIO_SIZE,
                Gicv2::distrib_read,
                Gicv2::distrib_write,
            )
            .mmio(
                "GICv2 CPU Interface Regs",
                self.bus,
                self.base + Gicv2::DISTRIB_MMIO_SIZE,
                Gicv2::CPU_IFACE_MMIO_SIZE,
                Gicv2::cpu_iface_read,
                Gicv2::cpu_iface_write,
            )
            .intr_in("SPI", 988, Gicv2::handle_spi)
            .intr_in("PPI", 16, Gicv2::handle_ppi);

        for cpu in &self.cpus {
            decl.intr_conn(cpu.fiq);
            decl.intr_conn(cpu.irq);
            decl.intr_conn(cpu.serr);
        }
    }
}
