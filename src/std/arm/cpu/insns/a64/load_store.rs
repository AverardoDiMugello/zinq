// TODO: Compare and swap pair

// TODO: Advanced SIMD load/store multiple structures

// TODO: Advanced SIMD load/store multiple structures (post-indexed)

// TODO: Advanced SIMD load/store single structure

// TODO: Advanced SIMD load/store single structure (post-indexed)

// TODO: RCW compare and swap

// TODO: RCW compare and swap pair

// TODO: 128-bit atomic memory operations

// TODO: GCS load/store

// TODO: Load/store memory tags

// TODO: Load/store exclusive pair

pub mod ldst_exclusive_reg {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($ctx:ident, $rs:ident, $rn:ident, $rt:ident, $el:ident, $pstate_sp:ident, $ops:literal => $s:ident, $n:ident, $t:ident, $tagchecked:ident, $address:ident, $block:ident, $graph:ident) => {
            let $s: u32 = $rs.load();
            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            let $tagchecked = $n != 31;

            let mut $graph = IRGraph::with_capacity(1, 0, 1 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
        };
    }

    pub fn stxrb<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlxrb<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: true,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldxrb<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldaxrb<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stxrh<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x16(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlxrh<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x16(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: true,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldxrh<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldaxrh<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stxr32<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x32(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlxr32<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x32(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: true,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldxr32<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldaxr32<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stxr64<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x64(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlxr64<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = x64(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::ExLDST {
                acqrel: true,
                tagchecked,
                s,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldxr64<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldaxr64<'a>(
        ctx: &ArmIRCtx,
        rs: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rs, rn, rt, el, pstate_sp, 2 => s, n, t, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::ExLDST {
                acqrel: false,
                tagchecked,
                s,
            },
        );
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_ordered {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($ctx:ident, $rn:ident, $rt:ident, $el:ident, $pstate_sp:ident, $ops:literal => $n:ident, $t:ident, $tagchecked:ident, $address:ident, $block:ident, $graph:ident) => {
            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            let $tagchecked = $n != 31;

            let mut $graph = IRGraph::with_capacity(1, 0, 1 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
        };
    }

    pub fn stllrb<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem8(address, data, ArmLdstAttrs::LOR { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlrb<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem8(address, data, ArmLdstAttrs::AcqRel { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldlarb<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem8(address, ArmLdstAttrs::LOR { tagchecked });
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }

        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldarb<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem8(address, ArmLdstAttrs::AcqRel { tagchecked });
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stllrh<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x16(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem16(address, data, ArmLdstAttrs::LOR { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlrh<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x16(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem16(address, data, ArmLdstAttrs::AcqRel { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldlarh<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem16(address, ArmLdstAttrs::AcqRel { tagchecked });
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldarh<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem16(address, ArmLdstAttrs::AcqRel { tagchecked });
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stllr32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x32(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem32(address, data, ArmLdstAttrs::LOR { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlr32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x32(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem32(address, data, ArmLdstAttrs::AcqRel { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldlar32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem32(address, ArmLdstAttrs::AcqRel { tagchecked });
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldar32<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem32(address, ArmLdstAttrs::AcqRel { tagchecked });
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stllr64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x64(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem64(address, data, ArmLdstAttrs::LOR { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stlr64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = x64(t, ctx, graph.block_mut(block));
        graph
            .block_mut(block)
            .write_mem64(address, data, ArmLdstAttrs::AcqRel { tagchecked });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldlar64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem64(address, ArmLdstAttrs::AcqRel { tagchecked });
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldar64<'a>(
        ctx: &ArmIRCtx,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, el, pstate_sp, 2 => n, t, tagchecked, address, block, graph);

        let data = graph
            .block_mut(block)
            .read_mem64(address, ArmLdstAttrs::AcqRel { tagchecked });
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

// TODO: Compare and swap

// TODO: LDIAPP/STILP

// TODO: LDAPR/STLR (writeback)

// TODO: LDAPR/STLR (unscaled immediate)

// TODO: LDAPR/STLR (SIMD&FP)

pub mod ld_reg_lit {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! literal {
        ($ctx:ident, $rt:ident, $imm19:ident, $ops:literal => $t:ident, $addr:ident, $block:ident, $graph:ident) => {
            let $t: u32 = $rt.load();
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[2..21]).copy_from_bitslice($imm19);
            (&mut offset[21..]).fill(*$imm19.last().unwrap());
            let offset = offset.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 3 + $ops);
            let $block = $graph.root();

            let pc = $graph.block_mut($block).read_reg64($ctx.reg64::<PC>());
            let ($addr, _) = $graph.block_mut($block).uadd_i64(pc, Bv64::Lit(offset));
        };
    }

    pub fn ldr32<'a>(ctx: &ArmIRCtx, imm19: &BitSlice<u32>, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        literal!(ctx, rt, imm19, 2 => t, addr, block, graph);
        let result = graph.block_mut(block).read_mem32(
            addr,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: false,
                unpriv_instr: false,
            },
        );
        let result = graph.block_mut(block).zext_i32_to_i64(result);
        if t != 31 {
            graph
                .block_mut(block)
                .write_reg64(ctx.arr64::<X>(t), result);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        literal!(ctx, rt, imm19, 2 => t, addr, block, graph);

        let result = graph.block_mut(block).read_mem32(
            addr,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: false,
            },
        );
        let result = graph.block_mut(block).zext_i32_to_i128(result);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), result);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64<'a>(ctx: &ArmIRCtx, imm19: &BitSlice<u32>, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        literal!(ctx, rt, imm19, 2 => t, addr, block, graph);

        let result = graph.block_mut(block).read_mem64(
            addr,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: false,
                unpriv_instr: false,
            },
        );
        if t != 31 {
            graph
                .block_mut(block)
                .write_reg64(ctx.arr64::<X>(t), result);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        literal!(ctx, rt, imm19, 2 => t, addr, block, graph);

        let result = graph.block_mut(block).read_mem64(
            addr,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: false,
            },
        );
        let result = graph.block_mut(block).zext_i64_to_i128(result);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), result);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsw<'a>(ctx: &ArmIRCtx, imm19: &BitSlice<u32>, rt: &BitSlice<u32>) -> IRGraph<ArmCpu> {
        literal!(ctx, rt, imm19, 3 => t, addr, block, graph);

        let result = graph.block_mut(block).read_mem32(
            addr,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: false,
                unpriv_instr: false,
            },
        );
        let result = graph.block_mut(block).sext_i32_to_i64(result);
        if t != 31 {
            graph
                .block_mut(block)
                .write_reg64(ctx.arr64::<X>(t), result);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm19: &BitSlice<u32>,
        rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        literal!(ctx, rt, imm19, 2 => t, addr, block, graph);

        let result = graph.block_mut(block).read_mem128(
            addr,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: false,
            },
        );
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), result);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn prfm<'a>(
        ctx: &ArmIRCtx,
        _imm19: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
    ) -> IRGraph<ArmCpu> {
        // Hints aren't implemented for now
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        let block = graph.block_mut(graph.root());
        block.nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

// TODO: Memory Copy and Memory Set

pub mod ldst_noalloc_pair_offset {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! pair_offset {
        ($ctx:ident, $rn:ident, $rt:ident, $rt2:ident, $imm7:ident, $scale:literal, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $t2:ident, $address:ident, $block:ident, $graph:ident => $insns:block) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..$scale]).fill(false);
            (&mut offset[$scale..($scale + 7)]).copy_from_bitslice($imm7);
            (&mut offset[($scale + 7)..]).fill(*$imm7.last().unwrap());
            let offset = offset.load();

            let $n: u32 = $rn.load();
            let $t: u32 = $rt.load();
            let $t2: u32 = $rt2.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 5 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);

            $insns
        };
    }

    pub fn stnp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x32(t, ctx, graph.block_mut(block));
            let data2 = x32(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldnp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});

            let data1 = graph.block_mut(block).zext_i32_to_i64(data1);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            let data2 = graph.block_mut(block).zext_i32_to_i64(data2);
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stnp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
            let data2 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldnp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
        let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
        let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
        let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});

        let data1 = graph.block_mut(block).zext_i32_to_i128(data1);
        graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
        let data2 = graph.block_mut(block).zext_i32_to_i128(data2);
        graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stnp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
            let data2 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldnp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});

            let data1 = graph.block_mut(block).zext_i64_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i64_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stnp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x64(t, ctx, graph.block_mut(block));
            let data2 = x64(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldnp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::GPR{nontemporal: true, tagchecked: n != 31, unpriv_instr: false});

            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stnp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
            let data2 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            graph.block_mut(block).write_mem128(address, data1, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
            graph.block_mut(block).write_mem128(address2, data2, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldnp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            let data1 = graph.block_mut(block).read_mem128(address, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});
            let data2 = graph.block_mut(block).read_mem128(address2, ArmLdstAttrs::ASIMD{nontemporal: true, tagchecked: n != 31});

            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_pair_post {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! pair_post {
        ($ctx:ident, $rn:ident, $rt:ident, $rt2:ident, $imm7:ident, $scale:literal, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $t2:ident, $address:ident, $block:ident, $graph:ident => $insns:block) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..$scale]).fill(false);
            (&mut offset[$scale..($scale+7)]).copy_from_bitslice($imm7);
            (&mut offset[($scale+7)..]).fill(*$imm7.last().unwrap());
            let offset = offset.load();

            let $n: u32 = $rn.load();
            let $t: u32 = $rt.load();
            let $t2: u32 = $rt2.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 5 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }

            $insns

            // Wback
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);
            if $n == 31 {
                $graph
                    .block_mut($block)
                    .write_reg64(if !$pstate_sp { $ctx.sp(EL::EL0) } else {$ctx.sp($el)}, $address);
            } else {
                $graph.block_mut($block).write_reg64($ctx.arr64::<X>($n), $address);
            }
        };
    }

    pub fn stp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x32(t, ctx, graph.block_mut(block));
            let data2 = x32(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});

            let data1 = graph.block_mut(block).zext_i32_to_i64(data1);
            if t != 31 {
                graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);
            }
            let data2 = graph.block_mut(block).zext_i32_to_i64(data2);
            if t2 != 31 {
                graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);
            }
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
            let data2 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});

            let data1 = graph.block_mut(block).zext_i32_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i32_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stgp<'a>(
        _ctx: &ArmIRCtx,
        _imm7: &BitSlice<u32>,
        _rt2: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn ldpsw<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 7 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});

            let data1 = graph.block_mut(block).sext_i32_to_i64(data1);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            let data2 = graph.block_mut(block).sext_i32_to_i64(data2);
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
            let data2 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});

            let data1 = graph.block_mut(block).zext_i64_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i64_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x64(t, ctx, graph.block_mut(block));
            let data2 = x64(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});

            if t != 31 {
                graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);
            }
            if t2 != 31 {
                graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);
            }
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
            let data2 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            graph.block_mut(block).write_mem128(address, data1, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_mem128(address2, data2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_post!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            let data1 = graph.block_mut(block).read_mem128(address, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            let data2 = graph.block_mut(block).read_mem128(address2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});

            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_pair_offset {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! pair_offset {
        ($ctx:ident, $rn:ident, $rt:ident, $rt2:ident, $imm7:ident, $scale:literal, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $t2:ident, $address:ident, $block:ident, $graph:ident => $insns:block) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..$scale]).fill(false);
            (&mut offset[$scale..($scale + 7)]).copy_from_bitslice($imm7);
            (&mut offset[($scale + 7)..]).fill(*$imm7.last().unwrap());
            let offset = offset.load();

            let $n: u32 = $rn.load();
            let $t: u32 = $rt.load();
            let $t2: u32 = $rt2.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 5 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);

            $insns
        };
    }

    pub fn stp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x32(t, ctx, graph.block_mut(block));
            let data2 = x32(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});

            let data1 = graph.block_mut(block).zext_i32_to_i64(data1);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            let data2 = graph.block_mut(block).zext_i32_to_i64(data2);
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
            let data2 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});

            let data1 = graph.block_mut(block).zext_i32_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i32_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stgp<'a>(
        _ctx: &ArmIRCtx,
        _imm7: &BitSlice<u32>,
        _rt2: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn ldpsw<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 7 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});

            let data1 = graph.block_mut(block).sext_i32_to_i64(data1);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            let data2 = graph.block_mut(block).sext_i32_to_i64(data2);
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
            let data2 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});

            let data1 = graph.block_mut(block).zext_i64_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i64_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x64(t, ctx, graph.block_mut(block));
            let data2 = x64(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: n != 31, unpriv_instr: false});

            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
            let data2 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            graph.block_mut(block).write_mem128(address, data1, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
            graph.block_mut(block).write_mem128(address2, data2, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_offset!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            let data1 = graph.block_mut(block).read_mem128(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});
            let data2 = graph.block_mut(block).read_mem128(address2, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: n != 31});

            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_pair_pre {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! pair_pre {
        ($ctx:ident, $rn:ident, $rt:ident, $rt2:ident, $imm7:ident, $scale:literal, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $t2:ident, $address:ident, $block:ident, $graph:ident => $insns:block) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..$scale]).fill(false);
            (&mut offset[$scale..($scale+7)]).copy_from_bitslice($imm7);
            (&mut offset[($scale+7)..]).fill(*$imm7.last().unwrap());
            let offset = offset.load();

            let $n: u32 = $rn.load();
            let $t: u32 = $rt.load();
            let $t2: u32 = $rt2.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 5 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);

            $insns

            // Wback
            if $n == 31 {
                $graph
                    .block_mut($block)
                    .write_reg64(if !$pstate_sp { $ctx.sp(EL::EL0) } else {$ctx.sp($el)}, $address);
            } else {
                $graph.block_mut($block).write_reg64($ctx.arr64::<X>($n), $address);
            }
        };
    }

    pub fn stp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x32(t, ctx, graph.block_mut(block));
            let data2 = x32(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp32<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});

            let data1 = graph.block_mut(block).zext_i32_to_i64(data1);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            let data2 = graph.block_mut(block).zext_i32_to_i64(data2);
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
            let data2 = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t2));

            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            graph.block_mut(block).write_mem32(address, data1, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_mem32(address2, data2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});

            let data1 = graph.block_mut(block).zext_i32_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i32_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stgp<'a>(
        _ctx: &ArmIRCtx,
        _imm7: &BitSlice<u32>,
        _rt2: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        todo!()
    }

    pub fn ldpsw<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 2, el, pstate_sp, cpa2, 7 => n, t, t2, address, block, graph => {
            let (address2, _) = graph.block_mut(block).uadd_i64(address, Addr::Lit(4));
            let data1 = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem32(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});

            let data1 = graph.block_mut(block).sext_i32_to_i64(data1);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            let data2 = graph.block_mut(block).sext_i32_to_i64(data2);
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
            let data2 = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t2));

            let (address2,_) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2,_) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});

            let data1 = graph.block_mut(block).zext_i64_to_i128(data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            let data2 = graph.block_mut(block).zext_i64_to_i128(data2);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = x64(t, ctx, graph.block_mut(block));
            let data2 = x64(t2, ctx, graph.block_mut(block));

            // TODO: BigEndian, lse, is_pair = true
            let (address2,_) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            graph.block_mut(block).write_mem64(address, data1, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            graph.block_mut(block).write_mem64(address2, data2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp64<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
        _lse2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 3, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            // TODO: BigEndian, lse, is_pair = true
            let (address2,_) = graph.block_mut(block).uadd_i64(address, Addr::Lit(8));
            let data1 = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data2 = graph.block_mut(block).read_mem64(address2, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});

            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data1);}
            if t2 != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t2), data2);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let data1 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
            let data2 = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t2));

            let (address2,_) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            graph.block_mut(block).write_mem128(address, data1, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_mem128(address2, data2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldp128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm7: &BitSlice<u32>,
        rt2: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pair_pre!(ctx, rn, rt, rt2, imm7, 4, el, pstate_sp, cpa2, 5 => n, t, t2, address, block, graph => {
            let (address2,_) = graph.block_mut(block).uadd_i64(address, Addr::Lit(16));
            let data1 = graph.block_mut(block).read_mem128(address, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});
            let data2 = graph.block_mut(block).read_mem128(address2, ArmLdstAttrs::ASIMD{ nontemporal: false, tagchecked: true});

            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data1);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t2), data2);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_usc_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! usc_imm {
        ($ctx:ident, $rn:ident, $rt:ident, $imm9:ident, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $address:ident, $block:ident, $graph:ident) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..9]).copy_from_bitslice($imm9);
            (&mut offset[9..]).fill(*$imm9.last().unwrap());
            let offset = offset.load();

            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            // TODO: tagchecked

            let mut $graph = IRGraph::with_capacity(1, 0, 3 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);
        };
    }

    pub fn sturb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldurb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldursb64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldursb32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg8(ctx.arr8::<B>(t));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
        graph.block_mut(block).write_mem128(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem128(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn sturh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x16(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldurh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldursh64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldursh32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg16(ctx.arr16::<H>(t));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x32(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldursw<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x64(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: n != 31,
                unpriv_instr: false,
            },
        );
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn prfum<'a>(
        ctx: &ArmIRCtx,
        _imm9: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        // Hints aren't implemented for now
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        let block = graph.block_mut(graph.root());
        block.nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn stur64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldur64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usc_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i64_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_post_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! post_imm {
        ($ctx:ident, $rn:ident, $rt:ident, $imm9:ident, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $address:ident, $block:ident, $graph:ident => $insns:block) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..9]).copy_from_bitslice($imm9);
            (&mut offset[9..]).fill(*$imm9.last().unwrap());
            let offset = offset.load();

            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            // TODO: tagchecked

            let mut $graph = IRGraph::with_capacity(1, 0, 5 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }

            $insns

            // Wback
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);
            if $n == 31 {
                $graph
                    .block_mut($block)
                    .write_reg64(if !$pstate_sp { $ctx.sp(EL::EL0) } else {$ctx.sp($el)}, $address);
            } else {
                $graph.block_mut($block).write_reg64($ctx.arr64::<X>($n), $address);
            }
        };
    }

    pub fn strb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x8(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem8(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).zext_i8_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).sext_i8_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).sext_i8_to_i32(data);
            let data = graph.block_mut(block).zext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg8(ctx.arr8::<B>(t));
            graph.block_mut(block).write_mem8(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i8_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
            graph.block_mut(block).write_mem128(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem128(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn strh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x16(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem16(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).zext_i16_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).sext_i16_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).sext_i16_to_i32(data);
            let data = graph.block_mut(block).zext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg16(ctx.arr16::<H>(t));
            graph.block_mut(block).write_mem16(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i16_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x32(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem32(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).zext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsw<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            let data = graph.block_mut(block).sext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
            graph.block_mut(block).write_mem32(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i32_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x64(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem64(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr: false});
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
            graph.block_mut(block).write_mem64(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        post_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i64_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_unpriv {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! decode {
        ($ctx:ident, $rn:ident, $rt:ident, $imm9:ident, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $nontemporal:ident, $tagchecked:ident, $address:ident, $block:ident, $graph:ident) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..9]).copy_from_bitslice($imm9);
            (&mut offset[9..]).fill(*$imm9.last().unwrap());
            let offset = offset.load();

            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            let $nontemporal = false;
            let $tagchecked = $n != 31;

            let mut $graph = IRGraph::with_capacity(1, 0, 3 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);
        };
    }

    pub fn sttrb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrsb64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrsb32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn sttrh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = x16(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrsh64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrsh32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn sttr32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = x32(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtr32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtrsw<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        let data = graph.block_mut(block).sext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn sttr64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = x64(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldtr64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, nontemporal, tagchecked, address, block, graph);
        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::GPR {
                nontemporal,
                tagchecked,
                unpriv_instr: true,
            },
        );
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

pub mod ldst_reg_pre_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! pre_imm {
        ($ctx:ident, $rn:ident, $rt:ident, $imm9:ident, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $address:ident, $block:ident, $graph:ident => $insns:block) => {
            let mut offset = bitarr!(u32, Lsb0; 0; 64);
            (&mut offset[0..9]).copy_from_bitslice($imm9);
            (&mut offset[9..]).fill(*$imm9.last().unwrap());
            let offset = offset.load();

            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 4 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);

            $insns

            // Wback
            if $n == 31 {
                $graph
                    .block_mut($block)
                    .write_reg64(if !$pstate_sp { $ctx.sp(EL::EL0) } else {$ctx.sp($el)}, $address);
            } else {
                $graph.block_mut($block).write_reg64($ctx.arr64::<X>($n), $address);
            }
        };
    }

    pub fn strb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x8(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem8(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrb<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).zext_i8_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).sext_i8_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).sext_i8_to_i32(data);
            let data = graph.block_mut(block).zext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg8(ctx.arr8::<B>(t));
            graph.block_mut(block).write_mem8(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem8(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i8_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
            graph.block_mut(block).write_mem128(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem128(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn strh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x16(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem16(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrh<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).zext_i16_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).sext_i16_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).sext_i16_to_i32(data);
            let data = graph.block_mut(block).zext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg16(ctx.arr16::<H>(t));
            graph.block_mut(block).write_mem16(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem16(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i16_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x32(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem32(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).zext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsw<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 3 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            let data = graph.block_mut(block).sext_i32_to_i64(data);
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
            graph.block_mut(block).write_mem32(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem32(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i32_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = x64(t, ctx, graph.block_mut(block));
            graph.block_mut(block).write_mem64(address, data, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::GPR{nontemporal: false, tagchecked: true, unpriv_instr:false});
            if t != 31{
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);}
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
            graph.block_mut(block).write_mem64(address, data, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm9: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        pre_imm!(ctx, rn, rt, imm9, el, pstate_sp, cpa2, 2 => n, t, address, block, graph => {
            let data = graph.block_mut(block).read_mem64(address, ArmLdstAttrs::ASIMD{nontemporal: false, tagchecked: true});
            let data = graph.block_mut(block).zext_i64_to_i128(data);
            graph.block_mut(block).write_reg128(ctx.arr128::<Q>(t), data);
        });
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

// TODO: Atomic memory operations

pub mod ldst_reg_regoff {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! decode_and_address {
        ($ctx:ident, $rm:ident, $option:ident, $rn:ident, $rt:ident, $shift:expr, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $t:ident, $n:ident, $m:ident, $address:ident, $block:ident, $graph:ident) => {
            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();
            let $m: u32 = $rm.load();
            let extend_type = ExtendType::decode($option[2], $option[1], $option[0]);

            let mut $graph = IRGraph::with_capacity(1, 0, 9 + $ops);
            let $block = $graph.root();

            let offset = extend_reg_i64($m, extend_type, $shift, $ctx, $graph.block_mut($block));
            let address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add(address, offset, $cpa2, $block, &mut $graph);
        };
    }

    pub fn strb<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, 0, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = x8(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrb<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, 0, el, pstate_sp, cpa2, 3 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, 0, el, pstate_sp, cpa2, 3 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, 0, el, pstate_sp, cpa2, 3 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str8_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, 0, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_reg8(ctx.arr8::<B>(t));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr8_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, 0, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str128_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {4} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
        graph.block_mut(block).write_mem128(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr128_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {4} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem128(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn strh<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {1} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = x16(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrh<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {1} else {0}, el, pstate_sp, cpa2, 3 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {1} else {0}, el, pstate_sp, cpa2, 3 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {1} else {0}, el, pstate_sp, cpa2, 3 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str16_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {1} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_reg16(ctx.arr16::<H>(t));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr16_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {1} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {2} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = x32(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {2} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsw<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {2} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        let data = graph.block_mut(block).sext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {2} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {2} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {3} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = x64(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {3} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                tagchecked: true,
                unpriv_instr: false,
            },
        );
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn pfrm<'a>(
        ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _option: &BitSlice<u32>,
        _s: bool,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        // Hints aren't implemented for now
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        let block = graph.block_mut(graph.root());
        block.nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn rprfm<'a>(
        ctx: &ArmIRCtx,
        _rm: &BitSlice<u32>,
        _option: &BitSlice<u32>,
        _s: bool,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        // Hints aren't implemented for now
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        let block = graph.block_mut(graph.root());
        block.nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {3} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64_simdfp<'a>(
        ctx: &ArmIRCtx,
        rm: &BitSlice<u32>,
        option: &BitSlice<u32>,
        s: bool,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        decode_and_address!(ctx, rm, option, rn, rt, if s {3} else {0}, el, pstate_sp, cpa2, 2 => t, n, m, address, block, graph);
        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: true,
            },
        );
        let data = graph.block_mut(block).zext_i64_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}

// TODO: Load/store register (pac)

pub mod ldst_reg_usn_imm {
    use crate::core::model::ir::*;
    use crate::std::arm::cpu::insns::helpers::*;
    use crate::std::arm::cpu::pseudoc::*;
    use crate::std::arm::cpu::registers::*;
    use crate::std::arm::cpu::{ArmCpu, ArmIRCtx, ArmLdstAttrs};
    use bitvec::prelude::*;

    macro_rules! usn_imm {
        ($ctx:ident, $rn:ident, $rt:ident, $imm12:ident, $lsl:literal, $el:ident, $pstate_sp:ident, $cpa2:ident, $ops:literal => $n:ident, $t:ident, $address:ident, $block:ident, $graph:ident) => {
            let offset = $imm12.load::<u64>() << $lsl;
            let $t: u32 = $rt.load();
            let $n: u32 = $rn.load();

            let mut $graph = IRGraph::with_capacity(1, 0, 3 + $ops);
            let $block = $graph.root();

            let $address;
            if $n == 31 {
                // TODO: CheckSPAlignment
                $address = sp($el, $pstate_sp, $ctx, $graph.block_mut($block));
            } else {
                $address = x64($n, $ctx, $graph.block_mut($block));
            }
            let $address = address_add($address, Bv64::Lit(offset), $cpa2, $block, &mut $graph);
        };
    }

    pub fn strb<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 0, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x8(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrb<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 0, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb64<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 0, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsb32<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 0, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).sext_i8_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 0, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg8(ctx.arr8::<B>(t));
        graph.block_mut(block).write_mem8(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr8_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 0, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem8(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i8_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 4, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg128(ctx.arr128::<Q>(t));
        graph.block_mut(block).write_mem128(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr128_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 4, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem128(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn strh<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 1, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x16(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrh<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 1, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh64<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 1, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsh32<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 1, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).sext_i16_to_i32(data);
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 1, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg16(ctx.arr16::<H>(t));
        graph.block_mut(block).write_mem16(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr16_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 1, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem16(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i16_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 2, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x32(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 2, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldrsw<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 2, el, pstate_sp, cpa2, 3 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).sext_i32_to_i64(data);
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 2, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg32(ctx.arr32::<S>(t));
        graph.block_mut(block).write_mem32(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr32_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 2, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem32(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i32_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 3, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = x64(t, ctx, graph.block_mut(block));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 3, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::GPR {
                nontemporal: false,
                unpriv_instr: false,
                tagchecked: n != 31,
            },
        );
        if t != 31 {
            graph.block_mut(block).write_reg64(ctx.arr64::<X>(t), data);
        }
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn prfm<'a>(
        ctx: &ArmIRCtx,
        _imm12: &BitSlice<u32>,
        _rn: &BitSlice<u32>,
        _rt: &BitSlice<u32>,
        _el: EL,
        _pstate_sp: bool,
        _cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        // Hints aren't implemented for now
        let mut graph = IRGraph::with_capacity(1, 0, 1);
        let block = graph.block_mut(graph.root());
        block.nop();
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn str64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 3, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_reg64(ctx.arr64::<D>(t));
        graph.block_mut(block).write_mem64(
            address,
            data,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }

    pub fn ldr64_simdfp<'a>(
        ctx: &ArmIRCtx,
        imm12: &BitSlice<u32>,
        rn: &BitSlice<u32>,
        rt: &BitSlice<u32>,
        el: EL,
        pstate_sp: bool,
        cpa2: bool,
    ) -> IRGraph<ArmCpu> {
        usn_imm!(ctx, rn, rt, imm12, 3, el, pstate_sp, cpa2, 2 => n, t, address, block, graph);

        let data = graph.block_mut(block).read_mem64(
            address,
            ArmLdstAttrs::ASIMD {
                nontemporal: false,
                tagchecked: n != 31,
            },
        );
        let data = graph.block_mut(block).zext_i64_to_i128(data);
        graph
            .block_mut(block)
            .write_reg128(ctx.arr128::<Q>(t), data);
        inc_pc(ctx, graph.block_mut(graph.root()));
        graph
    }
}
