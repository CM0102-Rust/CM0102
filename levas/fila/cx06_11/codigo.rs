// ---------- cx06_11: the European Championship qualifiers' play-off and qualified nations (euro_champ_qual.cpp) ----------

/// The fixed Euro 2000 play-off (the 1998 edition, before the game starts), in pairs: Scotland × England, Israel ×
/// Denmark, Slovenia × Ukraine, Republic of Ireland × Turkey.
pub(crate) const EURO_2000_PLAYOFF: [usize; 8] = [KEY_NATION_SCOTLAND, KEY_NATION_ENGLAND, KEY_NATION_ISRAEL, KEY_NATION_DENMARK, KEY_NATION_SLOVENIA,
    KEY_NATION_UKRAINE, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_TURKEY];

/// The league of group stage `g` of the qualifiers: −1 the qualifiers themselves (group 1), otherwise `.stages()`[g].
unsafe fn euro_qual_group(this: League, g: i32) -> League {
    if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) }
}

/// A nation that failed to qualify: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record`: round
/// 0x41c, weight 0x64) and its national team's competition ended (`clear_national_team_competition`).
unsafe fn euro_qual_failed(this: League, club: usize) {
    european_cup_add_failed_to_qualify_record(club, this.competition().addr(), 0x41c, 0x64);
    clear_national_team_competition(Club(club));
}

/// Moves the qualifiers to the next stage: past the last one (`.current_stage()` ≥ `.stage_count()` − 1, signed) the
/// euro_champ_qual.cpp box at line 0x38b and nothing else; otherwise `.current_stage()` + 1 and, entering the runners-up
/// stage (+0x100) or the play-off stage (+0x101), the play-off (`euro_champ_qual_create_playoff_stage`).
/// Original: `EuroChampQual::advance_stage` (0x57ec60, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c4674, 0x38b); // euro_champ_qual.cpp
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    if stage == read_u8(this + 0x100) as i8 as i32 || stage == read_u8(this + 0x101) as i8 as i32 { // the runners-up / play-off stages
        covered!("cx06_11 qualifiers play-off stage");
        euro_champ_qual_create_playoff_stage(this);
    }
}

/// Ends the qualifying groups and creates the play-off.
///
/// Without a runners-up stage (+0x100 = −1): every group row (the qualifiers and `.stages()`[0..=+0xff]) not marked
/// qualified (`league_table_row::STATUS` 7) fails (`euro_qual_failed`), and nothing else.
///
/// Otherwise: the best runners-up (the first 3 rows of the runners-up table with one European host, 1 with two) are
/// found in their groups, marked qualified (status 7, announced through `comp_vtable::ANNOUNCE_TEAM_STATUS` with 7, the
/// runners-up stage index, 0, 0) and their group gets `.promoted_count()` 2 and `.promotion_playoff_count()` 0; the
/// group scan stops when all of them are found. The play-off list (`exe::crt_malloc`; out of memory: the box at line
/// 0x441 and return) takes the last 6 (one host) or 8 (two) rows of the runners-up table; in every group those nations
/// get status 1 and the qualified-for-round record (round 0xa0, weight 0x1e), and the others not marked qualified fail
/// (`euro_qual_failed`). Then the progress 1, 1, 1 at stage −1 (`comp_vtable::SET_CLUB_PROGRESS`) for the first
/// (rows − play-off size) rows of the last group scanned: a defect of the original (the best runners-up of the runners-up
/// table were meant; the last group has 5 rows, fewer than the play-off, so nothing happens).
///
/// The play-off's round table (`comp_vtable::CREATE_SCHEDULE` with the play-off stage index) and its stage
/// (`exe::crt_operator_new`(0xb2) and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`: the qualifiers, the play-off size,
/// the list, the rounds, `.competition()`, the round table, the year, the stage index, 4, 0, 0x14, the pairing mode, 0,
/// 0, no seeds) into `.stages()`[+0x101]. Before the game start date (`G_GAME_START_DATE` after `G_CURRENT_DATE`: the
/// 1998 edition) the list is the fixed `EURO_2000_PLAYOFF` (8 nations even with a list of 6, a defect of the original),
/// the round's byte +0xe gets bit 2 and the pairs are kept (mode 1); otherwise they are drawn (mode 0). The list and the
/// round table freed; no stage: the box at line 0x4a3 and return; with it, its first round set up (`exe::cup_set_up_round`
/// with 0) and `.current_stage()` the play-off stage index.
/// Original: `EuroChampQual::create_playoff_stage` (0x57ef50, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_playoff_stage(this: usize) {
    let l = League(this);
    let last = read_u8(this + 0xff) as i8 as i32; // the last group stage
    let row = |group: League, r: i32| exe::league_get_table_row(group, r);
    if read_u8(this + 0x100) as i8 == -1 { // no runners-up stage
        let mut g = -1;
        while g <= last {
            let group = euro_qual_group(l, g);
            for r in 0..group.club_count() as i32 {
                let at = group.table() + r as usize * league_table_row::SIZE;
                if read_u8(at + league_table_row::STATUS) != 7 {
                    euro_qual_failed(l, read_usize(at));
                }
            }
            g += 1;
        }
        return;
    }
    let one_host = read_u8(this + euro_champ_qual::GROUP_FORMAT) == 1;
    let best_runners_up: i8 = if one_host { 3 } else { 1 };
    let runners_up = || League(read_usize(l.stages() + 4 * read_u8(this + 0x100) as i8 as isize as usize));
    let mut qualified = 0;
    let mut g = -1;
    while g <= last {
        let group = euro_qual_group(l, g);
        for r in 0..group.club_count() as i32 {
            for k in 0..best_runners_up {
                let candidate = row(runners_up(), k as i32);
                if read_usize(row(group, r)) == read_usize(candidate) {
                    covered!("cx06_11 best runner-up qualified");
                    write_u8(row(group, r) + league_table_row::STATUS, 7);
                    let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ANNOUNCE_TEAM_STATUS));
                    announce(this, read_usize(row(group, r)), 7, read_u8(this + 0x100) as u32, 0, 0);
                    group.set_promoted_count(2);
                    group.set_promotion_playoff_count(0);
                    qualified += 1;
                    break;
                }
            }
        }
        if qualified == best_runners_up as i32 {
            break;
        }
        g += 1;
    }
    let size: i16 = if one_host { 6 } else { 8 };
    let list = exe::crt_malloc((size as u32) * 4);
    if list == 0 {
        return show_error_at(0x9c4674, 0x441); // euro_champ_qual.cpp
    }
    let table_of = runners_up();
    let mut k = 0;
    for r in (table_of.club_count() as i32 - size as i32)..table_of.club_count() as i32 {
        write_usize(list + 4 * k, read_usize(row(table_of, r)));
        k += 1;
    }
    let mut scanned = table_of;
    let mut g = -1;
    while g <= last {
        let group = euro_qual_group(l, g);
        scanned = group;
        for r in 0..group.club_count() as i32 {
            let at = group.table() + r as usize * league_table_row::SIZE;
            if (0..size as usize).any(|k| read_usize(list + 4 * k) == read_usize(at)) {
                covered!("cx06_11 play-off nation");
                write_u8(at + league_table_row::STATUS, 1);
                european_cup_add_qualified_record(read_usize(group.table() + r as usize * league_table_row::SIZE), l.competition().addr(), 0, 0xa0, 0x1e);
                continue;
            }
            if read_u8(at + league_table_row::STATUS) != 7 {
                euro_qual_failed(l, read_usize(at));
            }
        }
        g += 1;
    }
    let progress: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    let mut r = 0i32;
    while r < scanned.club_count() as i32 - size as i32 {
        progress(this, read_usize(scanned.table() + r as usize * league_table_row::SIZE), -1, 1, 1, 1);
        r += 1;
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let index = read_u8(this + 0x101); // the play-off stage index
    let table = schedule(this, index as u32, &mut rounds, &mut id, 0);
    let (start_day, start_year) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
    let (day, year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let historic = start_year > year || (start_year == year && start_day > day);
    if historic {
        covered!("cx06_11 the euro 2000 play-off");
        for (k, &key) in EURO_2000_PLAYOFF.iter().enumerate() {
            write_usize(list + 4 * k, national_team_of_id(read_u32(key) as usize));
        }
        write_u8(table + 0xe, read_u8(table + 0xe) | 2); // the round's fixed-pairs bit
    }
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, size as u16 as u32, list, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, index as u32, 4, 0, 0x14,
            historic as u32, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(l.stages() + 4 * index as i8 as isize as usize, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    let stage = read_usize(l.stages() + 4 * index as i8 as isize as usize);
    if stage == 0 {
        return show_error_at(0x9c4674, 0x4a3);
    }
    exe::cup_set_up_round(stage, 0);
    l.set_current_stage(index as i8 as isize as usize);
}

/// Marks the nations qualified through the play-off, once (the qualified flag +0xfb set first): in every group (the
/// qualifiers and `.stages()`[0..=+0xff]) the row of each nation flagged as through (+5 = 1) in the play-off's team
/// table (`.stages()`[+0x101], 6-byte entries) gets status 7 (`league_table_row::STATUS`).
/// Original: `EuroChampQual::mark_qualified_teams` (0x580040, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_mark_qualified_teams(this: usize) {
    if read_u32(this + 0xfb) != 0 { // already marked
        return;
    }
    write_u32(this + 0xfb, 1);
    let l = League(this);
    let last = read_u8(this + 0xff) as i8 as i32;
    let playoff = || Cup(read_usize(l.stages() + 4 * read_u8(this + 0x101) as i8 as isize as usize));
    let mut g = -1;
    while g <= last {
        let group = euro_qual_group(l, g);
        let mut i = 0i32;
        while i < playoff().team_count() as i32 {
            let entry = playoff().team_table() + 6 * i as usize;
            if read_u8(entry + 5) == 1 { // through
                for r in 0..group.club_count() as i32 {
                    if read_usize(exe::league_get_table_row(group, r)) == read_usize(entry) {
                        covered!("cx06_11 play-off winner marked");
                        write_u8(exe::league_get_table_row(group, r) + league_table_row::STATUS, 7);
                        break;
                    }
                }
            }
            i += 1;
        }
        g += 1;
    }
}

/// Comparer for `euro_champ_qual_advance_stage`: the play-off allocates and draws (it cannot run twice), so only the
/// original runs; the plan: `.current_stage()` one up (unchanged past the last stage; the play-off sets it again).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_advance_stage(this: usize) {
    let l = League(this);
    let before = l.current_stage() as i32;
    let last = (l.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let now = l.current_stage() as i32;
    let ok = if before >= last { now == before } else { now == before + 1 || now == read_u8(this + 0x101) as i8 as i32 };
    SCOREBOARD.record(ok, || format!("euro_champ_qual_advance_stage({:#x}): stage {} -> {}", this, before, now));
}

/// Comparer for `euro_champ_qual_create_playoff_stage`: it writes records, announces and allocates the play-off (it
/// cannot run twice), so only the original runs; the plan: with a runners-up stage, the play-off stage built with 6 or 8
/// nations and `.current_stage()` its index.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_playoff_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let l = League(this);
    if read_u8(this + 0x100) as i8 == -1 {
        return;
    }
    let index = read_u8(this + 0x101) as i8 as isize as usize;
    let stage = Cup(read_usize(l.stages() + 4 * index));
    let size = if read_u8(this + euro_champ_qual::GROUP_FORMAT) == 1 { 6 } else { 8 };
    let ok = stage.addr() != 0 && stage.team_count() == size && l.current_stage() == index;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_create_playoff_stage({:#x}): stage {:#x}, current stage {}", this, stage.addr(), l.current_stage() as i32));
}

/// Comparer for `euro_champ_qual_mark_qualified_teams`: side by side on the qualified flag and every group table (the
/// statuses).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_mark_qualified_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let l = League(this);
    let mut regions = vec![(this + 0xfb, 4usize)];
    for g in -1..=read_u8(this + 0xff) as i8 as i32 {
        let group = euro_qual_group(l, g);
        if group.addr() == 0 || group.table() == 0 {
            return o(this);
        }
        regions.push((group.table(), group.club_count().clamp(0, 8) as usize * league_table_row::SIZE));
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || euro_champ_qual_mark_qualified_teams(this));
    SCOREBOARD.record(same, || format!("euro_champ_qual_mark_qualified_teams({:#x}): {}", this, first_differences(&mo, &mr).join(", ")));
}
