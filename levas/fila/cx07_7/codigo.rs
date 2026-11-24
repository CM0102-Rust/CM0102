// ---------- cx07_7: the Olympic football group A table, the group stages and the predicted positions (olympics.cpp) ----------

/// The Olympic football finalists drawn: a list of 16 club pointers (the groups one after the other, four each).
const OLYMPICS_DRAWN_CLUBS: usize = 0xee;
/// The count of the drawn list (dword).
const OLYMPICS_DRAWN_COUNT: usize = 0xf2;

/// The predicted positions of the 16 Olympic finalists, by reputation.
const OLYMPICS_POSITIONS: [u32; 16] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13];

/// Rule: group A of the Olympic football has 4 nations, drawn entries 0 to 3 (the list holds the groups one after the
/// other), each in the row of its order; human managers get the group draw news.
///
/// 4 teams (`.set_club_count()`), the table `.set_table()` (`exe::crt_malloc`(0xec): 4 rows of 0x3b bytes; out of
/// memory, the error box at line 0x1e3 and returns 0), each row i with drawn entry i (`exe::league_init_table_row`:
/// push 0; push 0; push national team; push i; mov ecx, this -> thiscall, ret 0x10; the index a dword whose high bytes
/// are stack leftovers, read as a byte), the group draw news (`exe::league_send_group_draw_news`) and returns 1.
/// Original: `Olympics::init_group_a_table` (0x7a0140, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_init_group_a_table(this: usize) -> u32 {
    let league = League(this);
    league.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    league.set_table(t);
    if t == 0 {
        show_error_at(OLYMPICS_SOURCE, 0x1e3);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        covered!("cx07_7 olympics group a row");
        let drawn = read_usize(read_usize(this + OLYMPICS_DRAWN_CLUBS) + i as usize * 4); // cx07_7: drawn entry i in row i
        exe::league_init_table_row(league, i as u8 as u32, drawn, 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

/// Comparer for `olympics_init_group_a_table`: only the original (the table and each row's history allocate, the news
/// goes out); then the plan: returns 1, 4 teams and in row i the drawn entry i.
pub(crate) unsafe extern "thiscall" fn compare_olympics_init_group_a_table(this: usize) -> u32 {
    let list = read_usize(this + OLYMPICS_DRAWN_CLUBS);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    if list == 0 {
        return o(this);
    }
    let plan: [usize; 4] = core::array::from_fn(|i| read_usize(list + i * 4));
    let r = o(this);
    let league = League(this);
    let t = league.table();
    let read: [usize; 4] = core::array::from_fn(|i| if t != 0 { read_usize(t + i * league_table_row::SIZE) } else { 0 });
    SCOREBOARD.record(r == 1 && league.club_count() == 4 && read == plan, || format!("olympics_init_group_a_table({:#x}): returned {}, rows {:x?} plan {:x?}", this, r, read, plan));
    r
}

/// Group g + 1 (g = 0 to 2, groups B to D) of the Olympic football as a League_Stage: a vector of 4
/// (`exe::crt_malloc`(0x10); out of memory: the box at line 0x2c5 and 0) with drawn entries 4(g + 1) to 4(g + 1) + 3,
/// then `nations_group_stage` of cx05_3 with 4 teams (no stage: the box at line 0x2dc).
/// Original: `Olympics::create_group_stage` (0x7a05d0, olympics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn olympics_create_group_stage(this: usize, g: i32) -> u32 {
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(OLYMPICS_SOURCE, 0x2c5);
        return 0;
    }
    covered!("cx07_7 olympics group stage");
    let first = (g + 1) as usize * 4; // cx07_7: the group's first drawn entry
    for k in 0..4usize {
        write_usize(teams + k * 4, read_usize(read_usize(this + OLYMPICS_DRAWN_CLUBS) + (first + k) * 4));
    }
    nations_group_stage(League(this), g, 4, teams, OLYMPICS_SOURCE, 0x2dc)
}

/// Comparer for `olympics_create_group_stage`: only the original (the stage allocates, the news goes out); then, when
/// it returns 1, the plan: the stage at [+0xc][g], +0x30 = g, four teams, the cup's year and, in its table, the drawn
/// 4(g + 1) to 4(g + 1) + 3 (in any order: the constructor sorts the table).
pub(crate) unsafe extern "thiscall" fn compare_olympics_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I1%%);
    let list = read_usize(this + OLYMPICS_DRAWN_CLUBS);
    if list == 0 || !(0..3).contains(&g) {
        return o(this, g);
    }
    let mut plan: Vec<usize> = (0..4).map(|k| read_usize(list + ((g + 1) as usize * 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let league = League(this);
        let f = League(read_usize(league.stages() + g as usize * 4));
        let t = if !f.is_null() { f.table() } else { 0 };
        let mut read: Vec<usize> = if t != 0 { (0..4).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        read.sort_unstable();
        plan.sort_unstable();
        let ok = !f.is_null() && league.current_stage() as i32 == g && f.club_count() == 4 && f.year() == league.year() && read == plan;
        SCOREBOARD.record(ok, || format!("olympics_create_group_stage({}): the stage does not match the plan ({:x?} read {:x?})", g, plan, read));
    }
    r
}

/// The predicted positions of the Olympic football: the 16 finalists (`OLYMPICS_DRAWN_CLUBS`, count
/// `OLYMPICS_DRAWN_COUNT` dword) by reputation (`predicted_by_reputation_of_copy`): 1, 2, two 3s, four 5s, four 9s and
/// four 13s (boxes at lines 0x44d and 0x455).
/// Original: `Olympics::set_predicted_positions` (0x7a13e0, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_set_predicted_positions(this: usize) {
    covered!("cx07_7 olympics predicted positions");
    predicted_by_reputation_of_copy(this, read_usize(this + OLYMPICS_DRAWN_CLUBS), read_i32(this + OLYMPICS_DRAWN_COUNT), &OLYMPICS_POSITIONS, OLYMPICS_SOURCE, (0x44d, 0x455));
}

/// Comparer for `olympics_set_predicted_positions`: only the original (the statistics entries are created), then the
/// positions read back against the plan (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_olympics_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (list, n) = (read_usize(this + OLYMPICS_DRAWN_CLUBS), read_i32(this + OLYMPICS_DRAWN_COUNT));
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    predicted_by_reputation_check("olympics_set_predicted_positions", this, &clubs, &OLYMPICS_POSITIONS);
}
