// ---------- batch %%LEVA%%: each game result of the Minas Gerais, Paraná and Rio state leagues and J1 (the overall and the half table) (bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, bra_reg_rio.cpp, jap_j1.cpp) ----------

/// The result of a game in a two-half league (the same 128-byte template in all four, checked instruction by
/// instruction: only the round where the second half starts, 11 in the state leagues and 15 in J1, and the half routine
/// change).
///
/// 1) The game goes into the overall table (League::add_result `exe::league_add_result`, thiscall(this, game, stats),
///    ret 8).
/// 2) If the game has no stage yet (`fixture::STAGE_INDEX` = 0xff): its stage is the half, 0 up to round `second` − 1
///    and 1 from it on (the round is the signed word `fixture::ROUND_NUMBER`), written to `fixture::STAGE_INDEX`; while
///    the league's current stage (`league::CURRENT_STAGE`) is below it, the next half is built (the half routine `half`,
///    e.g. `jap_j1_create_half_season_stage`: thiscall(this, current stage + 1), ret 4; the original pushes the dword
///    with the byte in the low part and register garbage above; the half routine only uses the byte: the copy passes the
///    clean byte); then the game also goes into the half table (the `comp_vtable::ADD_RESULT` slot of stage
///    [`league::STAGES`][stage]: thiscall(stage, game, stats), ret 8) with the league's statistics block
///    (`league::COMP_STATS`) saved and zeroed (so the game is not counted twice) and restored; the game's
///    `fixture::STAGE_INDEX` goes back to 0xff.
///
/// Returns 1 on both paths.
pub(crate) unsafe fn bra_reg_minas_gerais_add_two_half_result(this: League, game: usize, stats: u32, second: i16, half: usize, branch: &'static str) -> u32 {
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(half);
    exe::league_add_result(this, game, stats);
    if read_u8(game + fixture::STAGE_INDEX) != 0xff {
        return 1;
    }
    let round = read_i16(game + fixture::ROUND_NUMBER);
    let stage: i8 = (round >= second) as i8; // cqu3: the half of the game
    write_i8(game + fixture::STAGE_INDEX, stage);
    while (this.current_stage() as i32) < stage as i32 {
        build(this.addr(), read_u8(this.addr() + league::CURRENT_STAGE).wrapping_add(1) as u32);
    }
    covered!(branch);
    let stored = this.comp_stats();
    this.set_comp_stats(0);
    let f = read_usize(this.stages() + 4 * read_i8(game + fixture::STAGE_INDEX) as i32 as usize);
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::ADD_RESULT));
    add(f, game, stats); // cqu3: the game in the half table
    this.set_comp_stats(stored);
    write_u8(game + fixture::STAGE_INDEX, 0xff); // cqu3: the game mark back
    1
}

/// Comparer of the four: only the original runs (the game goes into the tables and the half may be built; it cannot run
/// twice), with the plan made before: the return value 1, the game's `fixture::STAGE_INDEX` as it was (0xff back, if it
/// was), the league's `league::COMP_STATS` unchanged and, when the game had no stage, the league's current stage
/// (`league::CURRENT_STAGE`) already at the game's half, with that stage built.
pub(crate) unsafe fn bra_reg_minas_gerais_compare_two_half_result(name: &str, this: League, game: usize, second: i16, original: impl FnOnce() -> u32) -> u32 {
    let mark = read_u8(game + fixture::STAGE_INDEX);
    let block = this.comp_stats();
    let stage = (read_i16(game + fixture::ROUND_NUMBER) >= second) as i32;
    let r = original();
    let mut ok = r == 1 && read_u8(game + fixture::STAGE_INDEX) == mark && this.comp_stats() == block;
    if mark == 0xff {
        ok = ok && this.current_stage() as i32 >= stage && read_usize(this.stages() + 4 * stage as usize) != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {:#x}): returned {}, mark {:#x} -> {:#x}, +0x30 = {}, game stage {}", name, this.addr(), game, r, mark, read_u8(game + fixture::STAGE_INDEX), this.current_stage() as i32, stage));
    r
}

/// Each game result of the Minas Gerais state league: the game goes into the overall table and, if it has no stage
/// yet, gets its half's (the first up to round 10, the second from round 11); the half is built when its first game
/// arrives (`bra_reg_minas_gerais_create_half_season_stage`, 0x4351b0), and the game also goes into the half table, without
/// counting again in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegMinasGerais::add_result` (0x435130, bra_reg_minas_gerais.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x4351b0, "cqu_3 minas gerais half") // cqu3: the second-half round of the Minas Gerais league
}

/// Each game result of the Paraná state league: the game goes into the overall table and, if it has no stage yet, gets
/// its half's (the first up to round 10, the second from round 11); the half is built when its first game arrives
/// (BraRegParana::create_half_stage, `create_half_stage`, 0x438760), and the game also goes into the half table, without counting
/// again in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegParana::add_result` (0x4386e0, bra_reg_parana.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x438760, "cqu_3 parana half") // cqu3: the Paraná league
}

/// Each game result of the Rio state league: the game goes into the overall table and, if it has no stage yet, gets its
/// half's (the first up to round 10, the second from round 11); the half is built when its first game arrives
/// (`bra_reg_rio_create_half_season_stage`, 0x43afa0), and the game also goes into the half table, without counting again
/// in the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `BraRegRio::add_result` (0x43af20, bra_reg_rio.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xb, 0x43afa0, "cqu_3 rio half") // cqu3: the Rio league
}

/// Each game result of J1: the game goes into the overall table and, if it has no stage yet, gets its half's (the first
/// up to round 14, the second from round 15); the half is built when its first game arrives
/// (`jap_j1_create_half_season_stage`, 0x663c20), and the game also goes into the half table, without counting again in
/// the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `JapJ1::add_result` (0x663940, jap_j1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn jap_j1_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xf, 0x663c20, "cqu_3 j1 half") // cqu3: the second-half round of J1
}

/// Comparer for `bra_reg_minas_gerais_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_minas_gerais_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// Comparer for `bra_reg_parana_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_parana_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// Comparer for `bra_reg_rio_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_rio_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    bra_reg_minas_gerais_compare_two_half_result("bra_reg_rio_add_result", League(this), game, 0xb, || o(this, game, stats))
}

/// Comparer for `jap_j1_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I3%%);
    bra_reg_minas_gerais_compare_two_half_result("jap_j1_add_result", League(this), game, 0xf, || o(this, game, stats))
}
