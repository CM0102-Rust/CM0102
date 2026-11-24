// ---------- cx09_9: the Belgian Third Division period winners and club progress (bel_third.cpp) ----------

/// The Belgian Third Division's period flags: three pairs (+0xfe, +0x100, +0x102), one byte per group (A, then B);
/// 1 once that group's period has its winner.
const BEL_THIRD_PERIOD_WON: [usize; 3] = [0xfe, 0x100, 0x102];

/// Updates the Belgian Third Division tables (League::update_positions_and_status 0x684cd0, through its detour), then
/// for group A (this league) and group B (stage 0): the next period still open (the group's first flag of
/// `BEL_THIRD_PERIOD_WON` at 0; all three set: nothing more for either group, a defect of the original: group B is not
/// looked at once group A has its three winners); its stage (BelThird::find_or_create_period_stage 0x422cf0 with the
/// group's `league::STAGE_ID`, without creating; none: the next group); when that period table's first row has status
/// 1 (decided), its club: each row of the group table with that club and status 0xff gets status 1, the period winner
/// news (BelThird::add_period_winner_news 0x4242e0) and its progress (virtual `comp_vtable::SET_CLUB_PROGRESS` with the
/// league's `league::STAGE_INDEX`, 2, 2 and 7); then the group's first open flag becomes 1.
/// Original: `BelThird::update_positions_and_status` (0x422f70, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_update_positions_and_status(this: usize) {
    let l = League(this);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684cd0usize); // League::update_positions_and_status
    base(this);
    let find: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> i8 = core::mem::transmute(0x422cf0usize);
    let news: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4242e0usize);
    for g in 0..2usize {
        let group = if g == 0 { l } else { League(read_usize(l.stages())) };
        let Some(period) = BEL_THIRD_PERIOD_WON.iter().position(|&f| read_u8(this + f + g) == 0) else {
            return;
        };
        let period = period as u32 + 1;
        let i = find(this, period, group.stage_id() as u16 as u32, 0);
        if i == -2 {
            continue;
        }
        let table = League(read_usize(l.stages() + 4 * i as isize as usize)).table();
        if LeagueTableRow(table).status() != 1 {
            continue;
        }
        let leader = LeagueTableRow(table).club() as usize;
        let mut k: i32 = 0;
        while k < group.club_count() as i32 {
            let row = LeagueTableRow(group.table() + k as usize * league_table_row::SIZE);
            if row.club() as usize == leader && row.status() == 0xff {
                covered!("cx09_9 belgian third division period winner");
                row.set_status(1);
                news(this, row.club() as usize, period);
                let progress: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
                progress(this, row.club() as usize, l.stage_index() as u32, 2, 2, 7);
            }
            k += 1;
        }
        let open = BEL_THIRD_PERIOD_WON.iter().position(|&f| read_u8(this + f + g) == 0).unwrap_or(2);
        write_u8(this + BEL_THIRD_PERIOD_WON[open] + g, 1);
    }
}

/// Sends the news of a Belgian Third Division period winner: a local NewsItem (`exe::new_newsitem`(0x11, 0)) with the
/// club's id (slot 0), the period (slot 1, its signed byte) and the competition's id (slot 2), sent to the managers of
/// the division (`exe::news_send_to_division_managers` on `G_NEWS`, with the competition record). The same body as
/// BelSecond's period winner news.
/// Original: `BelThird::add_period_winner_news` (0x4242e0, bel_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bel_third_add_period_winner_news(this: usize, club: usize, period: u32) {
    let comp = League(this).competition().addr();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x11, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club));
    exe::newsitem_set_slot(n, 1, period as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 2, read_u32(comp));
    covered!("cx09_9 belgian third division period news");
    exe::news_send_to_division_managers(read_usize(G_NEWS), n, comp);
}

/// Stores a club's three progress codes in its Comp_Stats entry (`exe::comp_stats_find_club_entry`, unchecked): in the
/// league and group stages (the stage as a signed byte below 2, the league's 0xff included) each code becomes
/// 3 × code − 2 (byte arithmetic), elsewhere as they come; into `comp_stats_club::CURRENT_POSITION`, `BEST_POSITION`
/// and `WORST_POSITION`.
/// Original: `BelThird::set_club_progress` (0x4241b0, bel_third.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bel_third_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 2 {
        covered!("cx09_9 belgian third division league progress");
        (c, b, w) = (c.wrapping_mul(3).wrapping_sub(2), b.wrapping_mul(3).wrapping_sub(2), w.wrapping_mul(3).wrapping_sub(2));
    }
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `bel_third_update_positions_and_status`: only the original (it updates the tables and sends news).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_update_positions_and_status(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparer for `bel_third_add_period_winner_news`: only the original (it sends the news).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_add_period_winner_news(this: usize, club: usize, period: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, club, period)
}

/// Comparer for `bel_third_set_club_progress`: side by side on the 3 bytes (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("bel_third_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bel_third_set_club_progress(this, club, stage, current, best, worst));
}
