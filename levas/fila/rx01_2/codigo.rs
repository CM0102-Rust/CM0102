// ---------- rx01_2: the T2K TrueType interpreter: the round functions and the stretched readers (library_without_asserts, T2K fnt) ----------

/// The round function signature: (state, 26.6 value, engine compensation) -> the rounded value.
pub(crate) type TtRoundFn = unsafe extern "C" fn(TtLocalState, i32, i32) -> i32;

/// The common shape of the round functions: a non-negative `x` gives `up(x + engine)`, a negative one
/// `−up(engine − x)`; when the result's sign differs from a non-zero `x`, `flipped` is returned instead.
fn round_symmetric(x: i32, engine: i32, up: impl Fn(i32) -> i32, flipped: i32) -> i32 {
    let r = if x >= 0 { up(x.wrapping_add(engine)) } else { up(engine.wrapping_sub(x)).wrapping_neg() };
    if (r ^ x) < 0 && x != 0 {
        flipped
    } else {
        r
    }
}

/// Rounds to the nearest half pixel (multiple of 32), symmetric about zero; 0 when the sign would flip.
/// Original: `tt_round_to_double_grid` (0x936e80, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_round_to_double_grid(_gs: TtLocalState, x: i32, engine: i32) -> i32 {
    round_symmetric(x, engine, |v| v.wrapping_add(0x10) & !0x1f, 0)
}

/// Rounds toward zero to a whole pixel (multiple of 64); 0 when the sign would flip.
/// Original: `tt_round_down_to_grid` (0x936eb0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_round_down_to_grid(_gs: TtLocalState, x: i32, engine: i32) -> i32 {
    round_symmetric(x, engine, |v| v & !0x3f, 0)
}

/// Rounds away from zero to a whole pixel; 0 when the sign would flip.
/// Original: `tt_round_up_to_grid` (0x936ee0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_round_up_to_grid(_gs: TtLocalState, x: i32, engine: i32) -> i32 {
    round_symmetric(x, engine, |v| v.wrapping_add(0x3f) & !0x3f, 0)
}

/// Rounds to the nearest whole pixel; 0 when the sign would flip.
/// Original: `tt_round_to_grid` (0x936f10, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_round_to_grid(_gs: TtLocalState, x: i32, engine: i32) -> i32 {
    round_symmetric(x, engine, |v| v.wrapping_add(0x20) & !0x3f, 0)
}

/// Rounds to the centre of the pixel (a multiple of 64 plus 32); when the sign would flip, ±32 with the sign of `x`.
/// Original: `tt_round_to_half_grid` (0x936f40, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_round_to_half_grid(_gs: TtLocalState, x: i32, engine: i32) -> i32 {
    round_symmetric(x, engine, |v| (v & !0x3f).wrapping_add(0x20), if x > 0 { 0x20 } else { -0x20 })
}

/// No rounding: `x` unchanged.
/// Original: `tt_round_off` (0x936f80, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_round_off(_gs: TtLocalState, x: i32, _engine: i32) -> i32 {
    x
}

/// SROUND rounding: `((threshold − phase + |x| + engine) & period mask) + phase`, with the sign of `x`; when the sign
/// would flip, the phase with the sign of `x`.
/// Original: `tt_super_round` (0x936f90, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_super_round(gs: TtLocalState, x: i32, engine: i32) -> i32 {
    let g = gs.global();
    let (phase, threshold, mask) = (g.phase() as i32, g.threshold() as i32, g.period_mask());
    let up = |v: i32| (threshold.wrapping_sub(phase).wrapping_add(v) & mask).wrapping_add(phase);
    round_symmetric(x, engine, up, if x > 0 { phase } else { phase.wrapping_neg() })
}

/// S45ROUND rounding: like `tt_super_round` with a period of `period_45 >> 14` (a √2 multiple): the value is divided by
/// the period (`fixed_div`), cut to a whole pixel, multiplied back (`fixed_mul`) and the phase added; when the sign
/// would flip, the phase with the sign of `x`.
/// Original: `tt_super_round_45` (0x936ff0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_super_round_45(gs: TtLocalState, x: i32, engine: i32) -> i32 {
    let g = gs.global();
    let (phase, threshold) = (g.phase() as i32, g.threshold() as i32);
    let up = |v: i32| {
        let period = g.period_45() >> 14;
        let steps = lib_fixed_div(threshold.wrapping_sub(phase).wrapping_add(v), period) & !0x3f;
        lib_fixed_mul(steps, g.period_45() >> 14).wrapping_add(phase)
    };
    round_symmetric(x, engine, up, if x > 0 { phase } else { phase.wrapping_neg() })
}

/// The stretch of the projection vector for non-square pixels: the x stretch along x, the y stretch along y, else
/// `y_stretch × py² + x_stretch × px²` (2.14 squares, rounded).
pub(crate) unsafe fn projection_stretch(gs: TtLocalState) -> i32 {
    let g = gs.global();
    let (px, py) = gs.proj();
    if py == 0 {
        return g.x_stretch();
    }
    if px == 0 {
        return g.y_stretch();
    }
    let square = |v: i16| ((v as i32 * v as i32 + 0x2000) >> 14) as i16;
    lib_f2dot14_mul(g.y_stretch(), square(py)).wrapping_add(lib_f2dot14_mul(g.x_stretch(), square(px)))
}

/// Control value `n` scaled by the projection's stretch (non-square pixels): `fixed_mul(cvt[n], stretch)`.
/// Original: `tt_read_cvt_stretched` (0x9372c0, library_without_asserts, T2K fnt.c, cdecl).
pub(crate) unsafe extern "C" fn tt_read_cvt_stretched(gs: TtLocalState, n: i32) -> i32 {
    let stretch = projection_stretch(gs);
    lib_fixed_mul(read_i32(gs.global().cvt().wrapping_add((n as usize).wrapping_mul(4))), stretch)
}

/// The single width scaled by the projection's stretch (non-square pixels).
/// Original: `tt_single_width_stretched` (0x937360, library_without_asserts, T2K fnt.c, cdecl; "tt_current_ppem_stretched"
/// in the first survey).
pub(crate) unsafe extern "C" fn tt_single_width_stretched(gs: TtLocalState) -> i32 {
    let stretch = projection_stretch(gs);
    lib_fixed_mul(gs.global().single_width(), stretch)
}

/// Comparers: value against value (pure: they only read the state).
unsafe fn compare_tt_rounder(name: &str, original: TtRoundFn, copy: TtRoundFn, gs: TtLocalState, x: i32, engine: i32) -> i32 {
    let r = original(gs, x, engine);
    let c = copy(gs, x, engine);
    SCOREBOARD.record(r == c, || format!("{}({}, {}) original={} rust={}", name, x, engine, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_round_to_double_grid(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_round_to_double_grid", original_fn::<TtRoundFn>(%%I0%%), tt_round_to_double_grid, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_round_down_to_grid(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_round_down_to_grid", original_fn::<TtRoundFn>(%%I1%%), tt_round_down_to_grid, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_round_up_to_grid(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_round_up_to_grid", original_fn::<TtRoundFn>(%%I2%%), tt_round_up_to_grid, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_round_to_grid(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_round_to_grid", original_fn::<TtRoundFn>(%%I3%%), tt_round_to_grid, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_round_to_half_grid(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_round_to_half_grid", original_fn::<TtRoundFn>(%%I4%%), tt_round_to_half_grid, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_round_off(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_round_off", original_fn::<TtRoundFn>(%%I5%%), tt_round_off, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_super_round(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_super_round", original_fn::<TtRoundFn>(%%I6%%), tt_super_round, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_super_round_45(gs: TtLocalState, x: i32, e: i32) -> i32 {
    compare_tt_rounder("tt_super_round_45", original_fn::<TtRoundFn>(%%I7%%), tt_super_round_45, gs, x, e)
}

pub(crate) unsafe extern "C" fn compare_tt_read_cvt_stretched(gs: TtLocalState, n: i32) -> i32 {
    let r = original_fn::<TtReadFn>(%%I8%%)(gs, n);
    let c = tt_read_cvt_stretched(gs, n);
    SCOREBOARD.record(r == c, || format!("tt_read_cvt_stretched({}) original={} rust={}", n, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_tt_single_width_stretched(gs: TtLocalState) -> i32 {
    let r = original_fn::<TtValueFn>(%%I9%%)(gs);
    let c = tt_single_width_stretched(gs);
    SCOREBOARD.record(r == c, || format!("tt_single_width_stretched original={} rust={}", r, c));
    r
}
