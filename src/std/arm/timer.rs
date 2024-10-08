use crate::model::{dev::*, proc::ProcId};
use crate::std::arm::cpu::registers::aarch64;

#[derive(Clone)]
struct GTimerImpl {
    tval: u64,
    cval: u64,
    oval: u64,
    ctl: u64,
    intr_out: IntrInput,
    timer_id: Option<TimerId>,
    idx: usize,
}

impl GTimerImpl {
    fn new(intr_out: IntrInput, idx: usize) -> Self {
        GTimerImpl {
            tval: 0,
            cval: 0,
            oval: 0,
            ctl: 0,
            intr_out,
            timer_id: None,
            idx,
        }
    }

    fn enable(&self) -> bool {
        &self.ctl & 0b1 != 0
    }

    fn imask(&self) -> bool {
        &self.ctl & 0b10 != 0
    }

    fn istatus(&self) -> bool {
        &self.ctl & 0b100 != 0
    }

    fn set_istatus(&mut self, val: bool) {
        self.ctl |= if val { 0b100 } else { 0 };
    }

    fn recalc_timer(&mut self, ctx: &mut DevCtx) {
        // println!("recalc_timer()");
        if self.enable() {
            // println!("Timer enabled.");
            let istatus = ctx.count() - self.oval >= self.cval;
            self.set_istatus(istatus);

            let new_expiry;
            if istatus {
                self.timer_id = None;
                if self.oval > ctx.count() {
                    new_expiry = self.oval;
                } else {
                    new_expiry = u64::MAX;
                }
            } else {
                new_expiry = self.cval.saturating_add(self.oval);
            }
            // TODO: u64::MAX / CNTFRQ_EL0 case. Right now ticks = time, but that will change
            let new_timer_id;
            if let Some(timer_id) = self.timer_id {
                // println!("Modifying existing timer to expire at {new_expiry}.");
                new_timer_id = ctx.mod_timer(timer_id, new_expiry);
            } else {
                // println!("Creating new timer to expire at {new_expiry}.");
                // TODO: this whole index thing is a work-around to interior self-referencing. It's kind of ugly
                let idx = self.idx.clone();
                new_timer_id = ctx.new_timer(
                    move |gtimer: &mut GTimer, ctx: &mut DevCtx| {
                        gtimer.timer_impl(idx).recalc_timer(ctx)
                    },
                    new_expiry,
                );
            }
            self.timer_id = Some(new_timer_id);
        } else {
            // println!("Timer not enabled.");
            if let Some(timer_id) = self.timer_id {
                // println!("Deleting active timer.");
                ctx.del_timer(timer_id);
            }
            self.timer_id = None;
        }
        // TODO: a sysreg can override IMASK
        ctx.set_intr(self.intr_out, self.istatus() && !self.imask());
    }
}

#[derive(Clone)]
pub struct GTimer {
    proc: ProcId,
    phys: GTimerImpl,
    virt: GTimerImpl,
    hyp_phys: GTimerImpl,
    hyp_virt: GTimerImpl,
}

impl GTimer {
    pub fn new(
        proc: ProcId,
        intc: &DevInfo,
        phys: u32,
        virt: u32,
        hyp_phys: u32,
        hyp_virt: u32,
    ) -> Self {
        Self {
            proc,
            phys: GTimerImpl::new(intc.intr_in("PPI", phys), 0),
            virt: GTimerImpl::new(intc.intr_in("PPI", virt), 1),
            hyp_phys: GTimerImpl::new(intc.intr_in("PPI", hyp_phys), 2),
            hyp_virt: GTimerImpl::new(intc.intr_in("PPI", hyp_virt), 3),
        }
    }

    fn timer_impl(&mut self, idx: usize) -> &mut GTimerImpl {
        match idx {
            0 => &mut self.phys,
            1 => &mut self.virt,
            2 => &mut self.hyp_phys,
            3 => &mut self.hyp_virt,
            _ => panic!("Unreachable"),
        }
    }
}

impl Device for GTimer {
    fn declare(&self, decl: &mut DevDecl<Self>)
    where
        Self: Sized,
    {
        decl.intr_conn(self.phys.intr_out)
            // TODO: this only maps in the virtual timer used in the sail example
            .intr_conn(self.virt.intr_out)
            .map_reg::<aarch64::CNTV_CVAL_EL0, _, _>(
                self.proc,
                |dev, _| dev.virt.cval,
                |dev, val, ctx| {
                    dev.virt.cval = val;
                    dev.virt.recalc_timer(ctx);
                },
            )
            .map_reg::<aarch64::CNTV_CTL_EL0, _, _>(
                self.proc,
                |dev, _| dev.virt.ctl,
                |dev, val, ctx| {
                    dev.virt.ctl = val;
                    dev.virt.recalc_timer(ctx);
                },
            )
            .intr_conn(self.hyp_phys.intr_out)
            .intr_conn(self.hyp_virt.intr_out);
    }
}
