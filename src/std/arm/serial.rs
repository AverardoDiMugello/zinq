use crate::core::model::dev::*;

pub struct PL011 {
    bus: MemParent,
    base: usize,
    intr_out: IntrInput,
    zeros: Vec<u8>,
}

impl PL011 {
    const MMIO_SIZE: usize = 0x1000;

    pub fn new(bus: MemParent, base: usize, intr_out: IntrInput) -> Self {
        Self {
            bus,
            base,
            intr_out,
            zeros: vec![0; Self::MMIO_SIZE],
        }
    }

    pub fn mmio_read(&mut self, offset: usize, size: usize, _: &mut DevCtx) -> MemReadResult {
        if let Some(mem) = self.zeros.get(0..size) {
            // eprintln!("[UART] Unknown read offset: {offset}");
            let mut res = [0; 16];
            (&mut res[0..size]).copy_from_slice(mem);
            MemReadResult(Some(res))
        } else {
            panic!("Tried to read outside of UART size: {0}", size);
        }
    }

    pub fn mmio_write(&mut self, offset: usize, data: &[u8], _: &mut DevCtx) -> MemWriteResult {
        if offset == 0 {
            // TODO: this should be a configurable host representation of a character device
            print!("{0}", data.first().and_then(|c| c.as_ascii()).unwrap());
        } else {
            // eprintln!("[UART] Unknown write offset: {offset}");
            // eprintln!("[UART] Unknown write data: {0}", data.first().unwrap());
        }
        MemWriteResult(Some(()))
    }
}

impl Device for PL011 {
    fn declare(&self, decl: &mut DevDecl<Self>)
    where
        Self: Sized,
    {
        decl.mmio(
            "PL011-regs",
            self.bus,
            self.base,
            PL011::MMIO_SIZE,
            PL011::mmio_read,
            PL011::mmio_write,
        )
        .intr_conn(self.intr_out);
        // .host_resource(self.backend); // TODO
    }
}
