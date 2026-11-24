// ---------- cx16_2: the round tables of the CONCACAF and European World Cup qualifying (wc_concacaf_cup.cpp, wc_europe_league.cpp) ----------

/// Builds a qualifying round table whose plan has fields that depend on the competition: the stage plan by
/// `build_phase_table` (outputs, table, the dates in order), then `fields` (only `Cqp12B`/`Cqp12W`/`Cqp12X`) written
/// into the table. The calls of each round touch only that round's bytes and the original writes every field after
/// the calls of its round, so the bytes end the same.
pub(crate) unsafe fn build_phase_table_with_fields(this: League, plan: Option<GoldcupStagePlan>, fields: &[GoldcupScheduleOp], rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize) -> usize {
    let t = build_phase_table(this, plan, rounds, extra, league, file);
    if t != 0 {
        for op in fields {
            match *op {
                Cqp12B(o, v) => write_u8(t + o, v),
                Cqp12W(o, v) => write_u16(t + o, v),
                Cqp12X(o, v) => write_u32(t + o, v),
                _ => {}
            }
        }
    }
    t
}

/// Side-by-side comparer of `build_phase_table_with_fields`: as `compare_phase_table`, with the bytes of the plan's
/// operations and of `fields` in both tables.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_phase_table_with_fields(name: &str, stage: u32, plan: Option<GoldcupStagePlan>, fields: &[GoldcupScheduleOp], rounds: *mut i16, extra: *mut i16, league: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let Some(p) = plan else {
        let a = o();
        SCOREBOARD.record(a == 0, || format!("{}(stage {}) returned {:#x}", name, stage as u8 as i8, a));
        return a;
    };
    let (mut r2, mut e2, mut l2) = (*rounds, *extra, if league.is_null() { 0 } else { *league });
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], o, || c(&mut r2, &mut e2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { phase_table_written_bytes(p.ops, fields, a) == phase_table_written_bytes(p.ops, fields, b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} ({} x {}, {} x {}) table {}", name, stage as u8 as i8, outs, *rounds, r2, *extra, e2, table));
    a
}

/// The CONCACAF qualifying stage plan of `stage` (signed byte), with `others` the exempt non-Caribbean count (+0xbc):
/// −1 the Caribbean knockout (3 cup rounds, label 0x40f); 0, 1 and below −1 a Central American group league (label
/// 0x473, or 0x474 when not stage 0) of 2 rounds (when the exempt count is 2, or 1 for stage 1) or 6; 2 the play-off
/// cup round; 3 to 5 the semi-final groups (6 rounds, label 0x409 + stage); 6 the final group (10 rounds, label 0x96);
/// above, none.
pub(crate) fn wc_concacaf_cup_stage_plan(stage: u32, others: i8) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    let name = "cx16_2 concacaf qualifying round table";
    match f {
        -1 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0x40f, extra_check: false, size: 0x138, row: 0xd4, ops: &WC_CONCACAF_KNOCKOUT_OPS, ops2: &[], name }),
        -128..=1 => {
            let short = (others == 1 && f == others) || others == 2; // cx16_2: two rounds when Central America has exempt nations
            let ops: &'static [GoldcupScheduleOp] = if short { &WC_CONCACAF_GROUP_OPS[..4] } else { &WC_CONCACAF_GROUP_OPS };
            let rounds = if short { 2 } else { 6 };
            Some(GoldcupStagePlan { league: 1, rounds, extra: 0x473 + (f != 0) as i16, extra_check: false, size: rounds as u32 * league_round_date::SIZE as u32, row: 0x136, ops, ops2: &[], name })
        }
        2 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x162, ops: &WC_CONCACAF_PLAY_OFF_OPS, ops2: &[], name }),
        3..=5 => Some(GoldcupStagePlan { league: 1, rounds: 6, extra: 0x409 + f as i16, extra_check: false, size: 0x186, row: 0x18c, ops: &WC_CONCACAF_SEMI_FINAL_OPS, ops2: &[], name }),
        6 => Some(GoldcupStagePlan { league: 1, rounds: 0xa, extra: 0x96, extra_check: false, size: 0x28a, row: 0x1b5, ops: &WC_CONCACAF_FINAL_GROUP_OPS, ops2: &[], name }),
        7..=127 => None,
    }
}

/// The fields of the CONCACAF knockout's three rounds (stage −1) that depend on the competition: the exempt Caribbean
/// count `caribbean` (+0xbb) sets the teams, ties and new teams of the first two rounds, and the 2000 campaign has the
/// draw rule 0x200 in every round; the rest of each round (round code, legs, prizes...) as the original writes it.
pub(crate) fn wc_concacaf_cup_knockout_fields(caribbean: i8, year: i16) -> Vec<GoldcupScheduleOp> {
    let draw = if year == 0x7d0 { 0x200 } else { 0 }; // cx16_2: the 2000 campaign's draw rule
    let first = (0xc - caribbean as i16) as u16;
    vec![
        Cqp12W(7, 0xa), Cqp12W(9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, draw), Cqp12B(0x17, 0xa), Cqp12W(0x18, first.wrapping_mul(2)),
        Cqp12W(0x1a, first), Cqp12W(0x1c, first.wrapping_mul(2)), Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 2),
        Cqp12B(0x22, 0xe), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
        Cqp12W(0x6f, 0x14), Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, draw), Cqp12W(0x77, 3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0xc),
        Cqp12W(0x82, 6), Cqp12W(0x84, caribbean as i16 as u16), Cqp12B(0x88, 0), Cqp12W(0x86, first.wrapping_mul(2)), Cqp12B(0x89, 2),
        Cqp12B(0x8a, 0xe), Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0),
        Cqp12W(0xd7, 0x1e), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, draw), Cqp12W(0xdf, 3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 6),
        Cqp12W(0xea, 3), Cqp12W(0xec, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 2), Cqp12B(0xf2, 0xe), Cqp12X(0x12c, 0),
        Cqp12X(0x130, 0), Cqp12X(0x134, 0),
    ]
}

/// The competition-dependent fields of a CONCACAF stage: the knockout's (stage −1), none for the others.
pub(crate) unsafe fn wc_concacaf_cup_stage_fields(this: usize, stage: u32) -> Vec<GoldcupScheduleOp> {
    if stage as u8 == 0xff {
        wc_concacaf_cup_knockout_fields(read_u8(this + 0xbb) as i8, Cup(this).year())
    } else {
        Vec::new()
    }
}

/// rule: the CONCACAF World Cup qualifying plays a Caribbean knockout of three two-legged rounds (January to April:
/// 24 teams less the exempt Caribbean nations, then 12 with the exempt joining, then 6), two Central American groups
/// (February to April, six rounds, or two when Central America has exempt nations), a play-off round (May), three
/// semi-final groups of six rounds (June to October) and a final group of ten rounds over the next year (February to
/// October); later stages have no table
///
/// The stage plan of `wc_concacaf_cup_stage_plan` and, for the knockout, the fields of `wc_concacaf_cup_knockout_fields`
/// (`build_phase_table_with_fields`): each date is a call of the original (league_round_set_date 0x68a160,
/// league_round_set_match_slot 0x68a1c0, cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0: pushes;
/// add esp -> cdecl) in the original's order (months from 0 = January); the outputs (`rounds`, `extra` always written,
/// `league` only when not null; 0 for the cup stages, 1 for the groups), the table size and the error-box line when out
/// of memory per stage. The original pushes the year as a word with garbage in the high bytes to 0x68a160, which reads
/// the word.
/// Original: `WcConcacafCup::create_round_table` (0x914de0, wc_concacaf_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    covered!("cx16_2 concacaf qualifying round table");
    let plan = wc_concacaf_cup_stage_plan(stage, read_u8(this + 0xbc) as i8);
    build_phase_table_with_fields(League(this), plan, &wc_concacaf_cup_stage_fields(this, stage), rounds, extra, league, wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP)
}

/// Comparer for `wc_concacaf_cup_create_round_table`: side by side (`compare_phase_table_with_fields`).
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    let plan = wc_concacaf_cup_stage_plan(stage, read_u8(this + 0xbc) as i8);
    let fields = wc_concacaf_cup_stage_fields(this, stage);
    compare_phase_table_with_fields("wc_concacaf_cup_create_round_table", stage, plan, &fields, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_concacaf_cup_create_round_table(this, stage, r, e, l))
}

/// The CONCACAF knockout's three rounds: the draw and the first-leg dates of each (10 Jan / 4 Mar, 19 Mar / 2 Apr, 17 Apr / 7 May).
pub(crate) const WC_CONCACAF_KNOCKOUT_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12S(0, 0xa, 1, 0, 4), Cqp12J(0, 4, 2, 0, 5, 1, 0), Cqp12S(1, 0x13, 2, 0, 6), Cqp12J(1, 2, 3, 0, 5, 1, 0),
    Cqp12S(2, 0x11, 3, 0, 6), Cqp12J(2, 7, 4, 0, 6, 1, 0),
];

/// The Central American group rounds (the first four operations when the group has two rounds).
pub(crate) const WC_CONCACAF_GROUP_OPS: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 5, 2, 0, 6, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x13, 2, 0, 6, 1, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 2, 3, 0, 6, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0x10, 3, 0, 6, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 7, 4, 0, 6, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x13, 4, 0, 6, 1, 0), Cqp12L(5, 0, -1, -1, -1, 0),
];

/// The CONCACAF play-off cup round (stage 2): drawn 20 May, played 3 Jun, six teams in three two-legged ties.
pub(crate) const WC_CONCACAF_PLAY_OFF_OPS: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x14, 4, 0, 0), Cqp12J(0, 3, 5, 0, 5, 1, 0), Cqp12W(0x18, 6), Cqp12W(0x1c, 6), Cqp12W(7, 0x172), Cqp12W(9, 0),
    Cqp12W(0xb, 0), Cqp12W(0xd, 0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 3), Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0),
    Cqp12B(0x21, 2), Cqp12B(0x22, 0xe), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
];

/// The CONCACAF semi-final group rounds (stages 3 to 5).
pub(crate) const WC_CONCACAF_SEMI_FINAL_OPS: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xc, 6, 0, 2, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x1a, 6, 0, 2, 2, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 0x10, 7, 0, 2, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 2, 8, 0, 5, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 0xb, 9, 0, 2, 2, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0xf, 0xa, 0, 2, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
];

/// The CONCACAF final group rounds (stage 6), the year after.
pub(crate) const WC_CONCACAF_FINAL_GROUP_OPS: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 2, 2, 1, 6, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x10, 2, 1, 6, 1, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 0x17, 2, 1, 6, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0x14, 3, 1, 6, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 0x1d, 5, 1, 6, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 9, 8, 1, 2, 1, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 1, 9, 1, 2, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 2, 0xa, 1, 6, 1, 0), Cqp12L(7, 0, -1, -1, -1, 0),
    Cqp12D(8, 9, 0xa, 1, 6, 1, 0), Cqp12L(8, 0, -1, -1, -1, 0), Cqp12D(9, 0x10, 0xa, 1, 6, 1, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];

/// The European qualifying stage plan of `stage` (signed byte), on the tables of the European Championship qualifiers
/// (the same dates): up to the last group stage (+0xff, signed byte) a group of 10 rounds over two years
/// (`EURO_QUAL_GROUP_OPS`, label 0x3e9 + stage, box line 0xdb); the runners-up stage (+0x100) none; the play-off stage
/// (+0x101) one cup round of two legs in November of the next year (`EURO_QUAL_PLAYOFF_OPS`, box line 0x115) of eight
/// nations (four ties) with none or one exempt nation (`WC_EUROPE_EXEMPT`), else six; any other, none.
pub(crate) unsafe fn wc_europe_league_stage_plan(this: usize, stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    let name = "cx16_2 european qualifying round table";
    if f <= read_u8(this + 0xff) as i8 {
        let label = (f as i16).wrapping_add(0x3e9); // cx16_2: the European group label
        return Some(GoldcupStagePlan { league: 1, rounds: 0xa, extra: label, extra_check: false, size: 0x28a, row: 0xdb, ops: &EURO_QUAL_GROUP_OPS, ops2: &[], name });
    }
    if f == read_u8(this + 0x100) as i8 || f != read_u8(this + 0x101) as i8 {
        return None;
    }
    let teams: &'static [GoldcupScheduleOp] = if read_u8(this + WC_EUROPE_EXEMPT) <= 1 { &EURO_QUAL_PLAYOFF_EIGHT } else { &EURO_QUAL_PLAYOFF_SIX }; // cx16_2: eight play-off teams with at most one exempt
    Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x115, ops: &EURO_QUAL_PLAYOFF_OPS, ops2: teams, name })
}

/// rule: the European World Cup qualifying groups play ten rounds from September of the draw year to September of the
/// next (2 and 6 Sep, 7 and 11 Oct, 24 and 28 Mar, 6 Jun, 2 and 6 Sep, 7 Oct), the dates of the European Championship
/// qualifiers; the play-off is one two-legged round drawn in October and played in November of the next year, with
/// eight teams (four ties) when at most one nation is exempt, else six
///
/// The stage plan of `wc_europe_league_stage_plan` by the round-table machine (`build_phase_table`): the dates are calls
/// of the original (0x68a160, 0x68a1c0, 0x521e60, 0x521eb0, cdecl) in the original's order; the outputs (`rounds`,
/// `extra` always written, `league` only when not null), the table size and the error-box line when out of memory per
/// stage; the runners-up stage and any stage past the play-off return 0 without writing anything.
/// Original: `WcEuropeLeague::create_schedule` (0x9185c0, wc_europe_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    covered!("cx16_2 european qualifying round table");
    build_phase_table(League(this), wc_europe_league_stage_plan(this, stage), rounds, extra, league, wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP)
}

/// Comparer for `wc_europe_league_create_schedule`: side by side (`compare_phase_table`).
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_phase_table("wc_europe_league_create_schedule", stage, wc_europe_league_stage_plan(this, stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_europe_league_create_schedule(this, stage, r, e, l))
}
