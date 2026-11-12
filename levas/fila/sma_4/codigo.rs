// ---------- batch %%LEVA%%: the national team tables of a new game (national_teams.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\national_teams.cpp", the file of the error box.
const NATIONAL_TEAMS_CPP: usize = 0xa5b2a4;

/// Builds the per-nation table of a new game: counts each nation's people, sizes the senior and under-21 reserve
/// pools, allocates them, builds the pools and picks the first squads.
///
/// `NationalTeams::init_tables()`. The table (`national_teams_fields::NATION_RECORDS`): `G_NATION_COUNT` records of
/// 0x6e bytes (`exe::crt_calloc`; failing, the error box at line 0xd9 and nothing else); the queued competitions
/// (`national_teams_fields::NEXT_COMPETITIONS`, `exe::crt_malloc`, one dword per nation) and the Olympic marks
/// (`national_teams_fields::OLYMPIC_NATION_MARKS`, a byte per nation). For each staff member with a nation
/// (`.nation()`) the nation's count (word +0) goes up; a national job (`.national_job()`) held in a player role
/// (`exe::database_is_player_job` of `.job_for_nation()`) is left (`exe::database_remove_staff_from_club`, flags 0, 1,
/// 0). For each nation: no current or queued competition (−1); the senior pool starts
/// (`national_squad_record::RESERVE_START`) at the running total and takes the count within 10..100 (byte +0xc), its
/// first level that many (`national_squad_record::RESERVE_LEVEL_CAPACITY`) or, with 100, 10 and the other three levels
/// (+0x12, +0x16, +0x1a) 30 each; the under-21 pool starts (+0x1d) at its own running total and takes the count within
/// 10..50 (+0x21), its first level (+0x23) the senior size or, with 50, 5 and the other three (+0x27, +0x2b, +0x2f)
/// 15 each. The two pools (`national_teams_fields::RESERVE_POOLS`, sizes at +0x14 and +0x18, a dword per slot; failing,
/// the error box at line 0x12c or 0x137). Then the pools are built (`exe::nationalteams_build_selection_pools`) and,
/// for each nation from the second one with a continent (`.continent()`), the senior and under-21 squads are picked
/// (`exe::nationalteams_select_squad`, types 0 and 2, today), the called-up marks cleared
/// (`national_squad_record::CALLED_UP_MARK`, `national_squad_record::U21_CALLED_UP_MARK`, +0x57) and the B squad's
/// largest size (+0x56) set to 24. Both pools are sized by the senior total (the under-21 one is larger than it needs;
/// the under-21 total only sets the starts).
/// Original: `NationalTeams::init_tables` (0x76a840, national_teams.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nationalteams_init_tables(this: usize) {
    use national_squad_record as r;
    let n = read_i32(G_NATION_COUNT);
    let t = exe::crt_calloc(n as u32, 0x6e);
    write_usize(this + national_teams_fields::NATION_RECORDS, t);
    if t == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0xd9);
        return;
    }
    write_usize(this + national_teams_fields::NEXT_COMPETITIONS, exe::crt_malloc((n as u32).wrapping_mul(4)));
    write_usize(this + national_teams_fields::OLYMPIC_NATION_MARKS, exe::crt_calloc(1, read_u32(G_NATION_COUNT)));
    for i in 0..read_i32(G_STAFF_COUNT) {
        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE)));
        let nation = s.nation();
        if !nation.is_null() {
            let c = read_usize(this + national_teams_fields::NATION_RECORDS) + read_u32(nation.addr()) as usize * 0x6e;
            write_u16(c, read_u16(c).wrapping_add(1));
        }
        if !s.national_job().is_null() && exe::database_is_player_job(s.job_for_nation() as i8) != 0 {
            exe::database_remove_staff_from_club(s.national_job(), s, 0usize, 1, 0);
        }
    }
    let (mut senior, mut u21) = (0u32, 0u32);
    for i in 0..read_i32(G_NATION_COUNT) as usize {
        let rec = |o: usize| read_usize(this + national_teams_fields::NATION_RECORDS) + i * 0x6e + o;
        write_i32(rec(r::CURRENT_COMPETITION), -1);
        write_i32(read_usize(this + national_teams_fields::NEXT_COMPETITIONS) + i * 4, -1);
        write_u32(rec(r::RESERVE_START), senior);
        let count = read_i16(rec(0));
        let size = count.clamp(10, 100) as u8; // sma4: the senior pool size
        write_u8(rec(0xc), size);
        senior += size as u32;
        if size < 100 {
            write_u8(rec(r::RESERVE_LEVEL_CAPACITY), size);
        } else {
            covered!("sma_4 full senior pool");
            write_u8(rec(r::RESERVE_LEVEL_CAPACITY), 10);
            for o in [0x12, 0x16, 0x1a] {
                write_u8(rec(o), 0x1e);
            }
        }
        write_u32(rec(0x1d), u21);
        let size21 = count.clamp(10, 0x32) as u8; // sma4: the under-21 pool size
        write_u8(rec(0x21), size21);
        u21 += size21 as u32;
        if size21 < 0x32 {
            write_u8(rec(0x23), read_u8(rec(0xc)));
        } else {
            write_u8(rec(0x23), 5);
            for o in [0x27, 0x2b, 0x2f] {
                write_u8(rec(o), 0xf);
            }
        }
    }
    write_u32(this + 0x14, senior);
    let p = exe::crt_calloc(senior, 4);
    write_usize(this + national_teams_fields::RESERVE_POOLS, p);
    if p == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0x12c);
        return;
    }
    write_u32(this + 0x18, senior);
    let p = exe::crt_calloc(senior, 4);
    write_usize(this + national_teams_fields::RESERVE_POOLS + 4, p);
    if p == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0x137);
        return;
    }
    exe::nationalteams_build_selection_pools(this);
    for i in 1..read_i32(G_NATION_COUNT) as usize {
        let nation = Nation(read_usize(G_NATION_TABLE) + i * 0x122);
        if nation.continent() == 0 {
            continue;
        }
        covered!("sma_4 first squads");
        exe::nationalteams_select_squad(this, nation, 0, G_CURRENT_DATE, 0usize, 0);
        exe::nationalteams_select_squad(this, Nation(read_usize(G_NATION_TABLE) + i * 0x122), 2, G_CURRENT_DATE, 0usize, 0);
        let rec = read_usize(this + national_teams_fields::NATION_RECORDS) + i * 0x6e;
        write_u8(rec + r::CALLED_UP_MARK, 0);
        write_u8(rec + r::U21_CALLED_UP_MARK, 0);
        write_u8(rec + 0x56, 0x18); // sma4: the B squad's largest size
        write_u8(rec + 0x57, 0);
    }
}

/// Comparer for `nationalteams_init_tables`.
///
/// The tables are allocated once per game: only the original runs, and the sizes it leaves are checked against the
/// rule (each nation's senior pool within 10..100 and under-21 within 10..50 of its count, the starts the running
/// totals, both pool sizes the senior total).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_init_tables(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let t = read_usize(this + national_teams_fields::NATION_RECORDS);
    if t == 0 || read_usize(this + national_teams_fields::RESERVE_POOLS + 4) == 0 {
        return;
    }
    let (mut senior, mut u21, mut ok) = (0u32, 0u32, true);
    for i in 0..read_i32(G_NATION_COUNT) as usize {
        let rec = t + i * 0x6e;
        let count = read_i16(rec);
        ok &= read_u32(rec + national_squad_record::RESERVE_START) == senior && read_u32(rec + 0x1d) == u21;
        ok &= read_u8(rec + 0xc) == count.clamp(10, 100) as u8 && read_u8(rec + 0x21) == count.clamp(10, 0x32) as u8;
        senior += read_u8(rec + 0xc) as u32;
        u21 += read_u8(rec + 0x21) as u32;
    }
    ok &= read_u32(this + 0x14) == senior && read_u32(this + 0x18) == senior;
    SCOREBOARD.record(ok, || format!("nationalteams_init_tables: totals {} / {} against {} / {}", read_u32(this + 0x14), read_u32(this + 0x18), senior, u21));
}
