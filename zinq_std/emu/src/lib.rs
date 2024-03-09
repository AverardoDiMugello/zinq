mod step_emu;
pub use step_emu::StepEmu;

/*
 * StepEmu:
 *
 * loop {
 *      let insn = cpu.fetch_decode(cpu.ip(), mem);
 *      let ir = insn.semantics(&cpu);
 *      for stmt in ir {
 *          match stmt {
 *              Assign => universal(stmt),
 *              WriteProc => universal(stmt),
 *              MemTx => step_memtx(stmt),
 *              Branch => step_branch(stmt),
 *          }
 *      }
 *      cpu.set_ip(cpu.ip() + insn.size());
 * }
 *
 * QuickEmu:
 *
 * let icount = 0;
 * loop {
 *
 *      // Accumulate basic-block of instructions
 *      let block = IrCtx::new();
 *      loop {
 *          let insn = cpu.fetch_decode(cpu.ip(), mem);
 *          block.extend(insn.semantics(&cpu));
 *
 *          icount += 1;
 *          if self.check_stop(icount) {
 *              break;
 *          }
 *
 *          if let Stmt::Branch(stmt) = block.back() {
 *              break;
 *          }
 *      }
 *
 *      for stmt in block {
 *          match stmt {
 *              Assign => universal(stmt)
 *              WriteProc => universal(stmt),
 *              MemTx => universal(stmt),
 *              Branch => {
 *                  if let Some(on_bb) = self.on_bb {
 *                      on_bb(args?)
 *                  }
 *                  universal?(stmt);
 *              }
 *          }
 *      }
 * }
 */
