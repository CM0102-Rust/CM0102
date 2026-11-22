// ---------- batch %%LEVA%%: the printouts (4): a club's printout, its squad statistics and its transfers (printouts.cpp) ----------

/// The dashes under the transfers' column titles (0xa78bc0) and under the squad's (0xa78884).
const PRINTOUT_TRANSFER_DASHES: usize = 0xa78bc0;
const PRINTOUT_SQUAD_DASHES: usize = 0xa78884;

/// `TRANSFER_MANAGER::get_past_transfers` (0x8cc030, thiscall, ret 0x18): the start month and year, the end month
/// and year, the array out (malloc'd 0x25-byte records) and the count out.
unsafe fn printout_past_transfers(start_month: u32, start_year: u32, end_month: u32, end_year: u32, list: *mut usize, count: *mut i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32) -> u32 = core::mem::transmute(0x8cc030usize);
    f(read_usize(G_TRANSFER_MANAGER), start_month, start_year, end_month, end_year, list, count);
}

/// `key_nation_format_club_season` (0x669630, cdecl): the year, the club, the buffer and its size.
unsafe fn printout_club_season(year: u32, club: usize, out: usize, size: u32) {
    let f: unsafe extern "C" fn(u32, usize, usize, u32) -> u32 = core::mem::transmute(0x669630usize);
    f(year, club, out, size);
}

/// `Date::get_short_date_string` (0x54a650, thiscall, ret 0xc): the date, the buffer, its size and the time of day
/// (0xff: none).
unsafe fn printout_short_date(date: usize, out: usize, size: u32, time: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x54a650usize);
    f(date, out, size, time);
}

/// The size of a past transfer record (`get_past_transfers`; its first dword is the staff index the name routine
/// takes) [inference from this routine].
const PAST_TRANSFER_SIZE: usize = 0x25;

/// Rule: a club's transfers in the printout's period, the ones it bought and then the ones it sold, by date.
///
/// `write_club_transfers(club, reserves, file)`; a national team writes nothing and returns 0. The transfers of the
/// period (`get_past_transfers` from the months and years of [`PRINTOUT_PERIOD_START`] and [`PRINTOUT_PERIOD_END`])
/// that neither bought nor sold by the club are dropped (the last record moved into the place, the place looked at
/// again) and the rest sorted by date (`qsort` with `printouts_compare_by_date`). The heading " <%s - season>
/// Transfers" takes the club's season of the start year. Then two passes: the players in ("Date Player In From Fee",
/// the club the buyer) and out ("... Player Out To ...", the club the seller); each row: the short date (8 wide), the
/// name (style 1, 25 wide), the other club's short name, or "Free Transfer" in / "Released" out without one (25 wide)
/// and the fee: with a fee, the money (flags 5), "<money> p/x" for an exchange; without one, "Exch", "Loan", "Bos" or
/// "Free" (8 wide). A pass without rows prints "None". Each pass ends with a newline; the array is freed and the
/// section ends. Returns 1.
/// Original: `printouts_write_club_transfers` (0x7d60c0, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_club_transfers(club: usize, _reserves: u32, file: usize) -> u32 {
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    let (mut list, mut count) = (0usize, 0i32);
    let end_month = exe::date_get_month(PRINTOUT_PERIOD_END) as u32;
    let start_month = exe::date_get_month(PRINTOUT_PERIOD_START) as u32;
    printout_past_transfers(start_month, read_u16(PRINTOUT_PERIOD_START + 2) as u32, end_month, read_u16(PRINTOUT_PERIOD_END + 2) as u32, &mut list, &mut count);
    let id = read_u32(club);
    let mut k = 0i32;
    while k < count {
        let r = list + k as usize * PAST_TRANSFER_SIZE;
        if read_u32(r + past_transfer::TO_CLUB_ID) != id && read_u32(r + past_transfer::FROM_CLUB_ID) != id {
            count -= 1;
            let last = list + count as usize * PAST_TRANSFER_SIZE;
            core::ptr::copy(last as *const u8, r as *mut u8, past_transfer::DATE);
            let mut out = [0u8; 8];
            exe::date_operator_3d(r + past_transfer::DATE, out.as_mut_ptr() as usize, last + past_transfer::DATE);
            k -= 1;
        }
        k += 1;
    }
    if count != 0 {
        exe::crt_qsort(list, count as u32, PAST_TRANSFER_SIZE as u32, 0x7da130usize);
    }
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_club_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, club, t, 0xc8);
    comp_lang_format()(PRINTOUT_LINE, 0xa78c90, t);
    printout_write_heading(file);
    for pass in 1..=2u8 {
        printout_write_titles(file, if pass == 1 { 0xa78c4c } else { 0xa78c08 }, 0, PRINTOUT_TRANSFER_DASHES);
        let mut any = false;
        for k in 0..count.max(0) {
            let r = list + k as usize * PAST_TRANSFER_SIZE;
            let ours = if pass == 1 { read_u32(r + past_transfer::TO_CLUB_ID) } else { read_u32(r + past_transfer::FROM_CLUB_ID) };
            if ours != id {
                continue;
            }
            write_u8(PRINTOUT_LINE, 0);
            printout_short_date(r + past_transfer::DATE, t, 0xc8, 0xff);
            printout_column(t, 8);
            exe::database_format_name_from_ids(read_u32(r + past_transfer::FIRST_NAME_ID), read_u32(r + past_transfer::SECOND_NAME_ID), read_u32(r + past_transfer::COMMON_NAME_ID), t as *mut u8, 0xc8, 1, read_u32(r));
            printout_column(t, 0x19);
            let other = if pass == 1 { read_i32(r + past_transfer::FROM_CLUB_ID) } else { read_i32(r + past_transfer::TO_CLUB_ID) };
            if other > -1 {
                printout_copy(t, read_usize(G_CLUB_TABLE) + other as usize * club::SIZE + club::SHORT_NAME);
            } else if pass == 1 {
                comp_lang_format()(t, 0x98e604);
            } else {
                comp_lang_format()(t, 0xa78b88, PRINTOUT_NO_TEXT);
            }
            printout_column(t, 0x19);
            let fee = read_u32(r + past_transfer::FEE);
            let kind = read_u8(r + past_transfer::MOVE_TYPE);
            if fee != 0 {
                let cash = fee;
                let amount = exe::cash_get_display_amount(&cash, 5);
                if kind == 1 {
                    covered!("tel25_8 exchange with a fee");
                    exe::cash_format_money(PRINTOUT_SCRATCH_TEXT, 0x7d0, amount, 5);
                    comp_lang_format()(t, 0x98e628, PRINTOUT_SCRATCH_TEXT);
                } else {
                    exe::cash_format_money(t, 0xc8, amount, 5);
                }
            } else {
                let word = match kind {
                    1 => 0x98e620,
                    2 => 0x98e5fc,
                    4 => 0x98e61c,
                    _ => 0x98e614,
                };
                comp_lang_format()(t, word);
            }
            printout_column(t, 8);
            printout_write_row(file);
            any = true;
        }
        if !any {
            covered!("tel25_8 no transfers");
            comp_lang_format()(PRINTOUT_LINE, 0xa78b64, PRINTOUT_NO_TEXT);
            printout_write_row(file);
        }
        printout_write(file, PRINTOUT_NEWLINE);
    }
    if list != 0 {
        exe::crt_free(list);
    }
    printout_end_section(file);
    1
}

/// Comparer for `printouts_write_club_transfers`: side by side over the text written (`compare_printout_text`).
pub(crate) unsafe extern "C" fn compare_printouts_write_club_transfers(club: usize, reserves: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I0%%);
    compare_printout_text(file, || format!("club transfers printout ({:#x})", club), |f| o(club, reserves, f), |f| printouts_write_club_transfers(club, reserves, f))
}

/// `LLIST` (14 bytes): its constructor, the item under the cursor without moving it (0x68d2e0) and the rewind (the
/// cursor `+0xa` back to the head `+2`).
unsafe fn printout_list_peek(list: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
    f(list)
}

unsafe fn printout_list_rewind(list: usize) {
    write_u32(list + 0xa, read_u32(list + 2));
}

/// The squad number column of a squad row: the absolute number when the club uses squad numbers and the player has
/// one (`get_squad_number`, asked twice as in the original), else "-"; 2 wide.
unsafe fn printout_squad_number(club: usize, person: usize, t: usize) {
    let manager = read_usize(G_SQUAD_MANAGER);
    if exe::squad_manager_club_uses_squad_numbers(manager, club, 0) != 0 && exe::squad_manager_get_squad_number(manager, person, club) != 0 {
        let n = exe::squad_manager_get_squad_number(manager, person, club) as i32;
        comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, n.abs());
    } else {
        comp_crt_format()(t, PRINTOUT_DASH);
    }
    printout_column(t, 2);
}

/// `PlayerStats` (at [`G_PLAYER_STATS`]): the scope's label (0x7c14b0, ret 0xc: the scope, the buffer, its size) and
/// a statistic of a player (0x7c25b0, ret 0x20: the player, the scope, the statistic, the buffer, its size and three
/// options) [names from player_stats.cpp's asserts; the options' meaning unknown].
unsafe fn printout_stats_scope_label(scope: u32, out: usize, size: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x7c14b0usize);
    f(read_usize(G_PLAYER_STATS), scope, out, size);
}

unsafe fn printout_player_stat(player: usize, scope: u32, stat: u32, out: usize, options: [u32; 3]) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7c25b0usize);
    f(read_usize(G_PLAYER_STATS), player, scope, stat, out, 0xc8, options[0], options[1], options[2]);
}

/// The statistics columns of the squad statistics printout: (statistic, options, width): appearances, goals,
/// conceded, penalties, assists, yellow cards, red cards, man of the match and the average rating.
const PRINTOUT_SQUAD_STATS: [(u32, [u32; 3], u32); 9] = [
    (1, [1, 0, 1], 7), (2, [0, 0, 1], 5), (3, [0, 0, 1], 5), (4, [1, 0, 1], 7), (5, [0, 0, 1], 5),
    (0xa, [0, 0, 1], 5), (0xb, [0, 0, 1], 5), (0xc, [0, 0, 1], 5), (0x11, [0, 0, 1], 5),
];

/// The scope of a club's statistics: 5 for a national team, 6 for a club [inference: the label 0x7c14b0 names it].
const PRINTOUT_SCOPE_NATIONAL: u8 = 5;
const PRINTOUT_SCOPE_CLUB: u8 = 6;

/// Rule: a club's squad statistics, one row per player of the squad by name.
///
/// `write_club_squad_stats(club, reserves, file)`. The squad list by name (`build_squad_list_by_name_for_printout`
/// 0x4849c0 into a local `LLIST`); the scope is 5 for a national team, 6 otherwise. The heading " <%s - season> <%s -
/// scope> Stats" takes the club's season of the start year and the scope's label (into the money text buffer); then
/// the column titles and dashes. Each player of the list: the squad number (2 wide), the name (style 3, 25 wide) and
/// nine statistics of the player record (`+0x61`) in the scope: appearances (7 wide), goals, conceded (5), penalties
/// (7), assists, yellow and red cards, man of the match and the average rating (5 each). The section ends and the list
/// is freed. Returns 1.
/// Original: `printouts_write_club_squad_stats` (0x7d5280, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_club_squad_stats(club: usize, _reserves: u32, file: usize) -> u32 {
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let build: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x4849c0usize);
    build(club, l);
    let scope = if exe::database_is_national_team(club) != 0 { PRINTOUT_SCOPE_NATIONAL } else { PRINTOUT_SCOPE_CLUB } as u32;
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_club_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, club, t, 0xc8);
    printout_stats_scope_label(scope, PRINTOUT_SCRATCH_TEXT, 0x7d0);
    comp_lang_format()(PRINTOUT_LINE, 0xa78b40, t, PRINTOUT_SCRATCH_TEXT);
    printout_write_heading(file);
    printout_write_titles(file, 0xa78ae0, 0, PRINTOUT_WIDE_DASHES);
    printout_list_rewind(l);
    while printout_list_peek(l) != 0 {
        covered!("tel25_8 squad statistics row");
        let person = read_usize(exe::llist_get_next(l));
        write_u8(PRINTOUT_LINE, 0);
        printout_squad_number(club, person, t);
        exe::database_format_staff_name(person, t, 0xc8, 3);
        printout_column(t, 0x19);
        let player = read_usize(person + staff::PLAYER);
        for (stat, options, width) in PRINTOUT_SQUAD_STATS {
            printout_player_stat(player, scope, stat, t, options);
            printout_column(t, width);
        }
        printout_write_row(file);
    }
    printout_end_section(file);
    exe::drop_llist(l);
    1
}

/// Comparer for `printouts_write_club_squad_stats`: side by side over the text written (`compare_printout_text`).
pub(crate) unsafe extern "C" fn compare_printouts_write_club_squad_stats(club: usize, reserves: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I1%%);
    compare_printout_text(file, || format!("club squad statistics printout ({:#x})", club), |f| o(club, reserves, f), |f| printouts_write_club_squad_stats(club, reserves, f))
}

/// The period of a club's printout: (day, year) of its start, from the club's nation's season end day
/// (`key_nation_record::SEASON_END_DAY` of the entry of [`G_LEAGUE_TABLE`] for `key_nation_index_of_club`) when the
/// club is no national team and the entry has `SPLIT_SEASON_FLAG`: the last such day not after today (today before
/// it: last year's); otherwise day 0 of this year.
unsafe fn printout_club_period_start(club: usize) -> (u32, u32) {
    let entry = G_LEAGUE_TABLE + exe::key_nation_index_of_club(club) as i8 as isize as usize * 0x48;
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    if exe::database_is_national_team(club) != 0 || read_u32(entry + key_nation_record::SPLIT_SEASON_FLAG) == 0 {
        return (0, year);
    }
    let day = read_u16(entry + key_nation_record::SEASON_END_DAY);
    let before_the_end = (read_i16(G_CURRENT_DATE) as i32) < day as i32;
    (day as u32, if before_the_end { year.wrapping_sub(1) & 0xffff } else { year })
}

/// The squad section of a club's printout (bit 1): the squad by name, one row per player with his number, name,
/// positions, nation, birth date, age, caps, international goals, wages, contract end and value.
unsafe fn printout_club_squad(club: usize, file: usize) {
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let build: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x4849c0usize);
    build(club, l);
    comp_lang_format()(PRINTOUT_LINE, 0xa789b4);
    printout_write_heading(file);
    printout_write_titles(file, 0xa788ec, 0, PRINTOUT_SQUAD_DASHES);
    printout_list_rewind(l);
    let pad: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x7da070usize);
    let column = |t: usize, width: u32| {
        let s = pad(t, width);
        printout_append(PRINTOUT_LINE, s);
        printout_append(PRINTOUT_LINE, PRINTOUT_GAP);
    };
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    while printout_list_peek(l) != 0 {
        covered!("tel25_8 club printout squad row");
        let person = read_usize(exe::llist_get_next(l));
        let manager = read_usize(G_CONTRACT_MANAGER);
        let mut contract = exe::contract_manager_get_contract(manager, person);
        if contract != 0 && read_u32(club) != read_u32(contract + 4) && exe::contract_manager_get_loan_contract(manager, person) != 0
            && read_u32(exe::contract_manager_get_loan_contract(manager, person) + 4) == read_u32(club) {
            covered!("tel25_8 loan contract");
            contract = exe::contract_manager_get_loan_contract(manager, person);
        }
        write_u8(PRINTOUT_LINE, 0);
        // the squad number: the original pads it with the inlined pad_string, the rest through the call
        printout_squad_number(club, person, t);
        exe::database_format_staff_name(person, t, 0xc8, 3);
        column(t, 0x19);
        database_format_player_positions(read_usize(person + staff::PLAYER), t, 0xc8, 1);
        column(t, 0xb);
        let nation = read_usize(person + staff::NATION);
        printout_copy(t, if nation != 0 { nation + nation::THREE_LETTER_NAME } else { PRINTOUT_DASH });
        column(t, 3);
        printout_short_date(person + staff::DATE_OF_BIRTH, t, 0xc8, 0xff);
        column(t, 8);
        comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, read_i8(person + staff::AGE) as i32);
        column(t, 3);
        for field in [staff::INT_APPS, staff::INT_GOALS] {
            let n = read_u8(person + field);
            if n != 0 {
                comp_crt_format()(t, PRINTOUT_FORMAT_NUMBER, n as u32);
            } else {
                printout_copy(t, PRINTOUT_DASH);
            }
            column(t, 3);
        }
        if contract != 0 {
            printout_contract_wage(contract, t, 0, 2);
            printout_contract_wage(contract, PRINTOUT_SCRATCH_TEXT, 0, 1);
            if printout_text_length(PRINTOUT_SCRATCH_TEXT) <= printout_text_length(t) {
                printout_copy(t, PRINTOUT_SCRATCH_TEXT);
            }
        } else {
            printout_copy(t, PRINTOUT_DASH);
        }
        column(t, 8);
        if contract != 0 {
            let expiry: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x85e8f0usize);
            expiry(contract, t);
        } else {
            printout_copy(t, PRINTOUT_DASH);
        }
        column(t, 8);
        let value = read_u32(person + staff::VALUE);
        if read_usize(person + staff::CLUB_JOB) != 0 && value != 0 {
            let cash = value;
            let format: unsafe extern "thiscall" fn(*const u32, usize, u32, u32) = core::mem::transmute(0x450aa0usize);
            format(&cash, t, 0xc8, 5);
        } else {
            comp_lang_format()(t, 0x98e614);
        }
        column(t, 8);
        printout_write_row(file);
    }
    printout_end_section(file);
    exe::drop_llist(l);
}

/// A contract's wage as text (0x85e750, staff_contracts.cpp, thiscall, ret 0xc): the buffer, a second argument (0
/// at both calls) and the style (2 and 1) [inference: the printout keeps the shorter of the two].
unsafe fn printout_contract_wage(contract: usize, out: usize, second: u32, style: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x85e750usize);
    f(contract, out, second, style);
}

/// Rule: a club's printout: the squad, its statistics, its transfers and its fixtures written to a file (by default
/// "~printout.txt"), then sent to the printer when asked; the temporary file is removed.
///
/// `print_club_report(club, reserves, kind, file, print)`. No club: the printouts.cpp 88 box and 0. The period is set
/// first: [`PRINTOUT_PERIOD_START`] from `printout_club_period_start` and [`PRINTOUT_PERIOD_END`] a year later (both
/// through `Date::Date` and `Date::operator=`). The file opens for writing ("w"; failing: the 113 box and 0). The
/// heading " <%s - club> - {}<%s - date>" (langlib_sprintf with 0x64, the club's name from
/// `club_get_national_team_name(club, 0, .., 0xc8, 3, 1)` and today's long date in the money text buffer) gets a row
/// of asterisks, a newline, the heading, a newline, the asterisks and three newlines. Then by the bits of `kind`: 1
/// the squad (the rows of `printout_club_squad`: number 2 wide with "-" for none, name style 3 25 wide, positions 11,
/// the nation's three letters or "-" 3, the birth date 8, the age 3, the caps and international goals or "-" 3, the
/// shorter of the two wage texts or "-" 8, the contract's end or "-" 8, the value for a player with a club and a value
/// or "Free" 8; the contract is the loan one when the main contract is another club's and the loan is this club's), 2
/// the squad statistics (0x7d5280), 4 the transfers (0x7d60c0) and 8 the fixtures (0x7d69f0), each with (club,
/// reserves, file); any of them returning 0 makes the result 0 (otherwise 1). The file is closed. With `print`: it opens
/// again for reading ("r"; failing: the 175 box and 0), a print job of 0x8c starts and each line read (0xc8 bytes,
/// its final newline cut) becomes a page break when it is a single space, otherwise a printed line; then the job is
/// printed and ended and the file closed. Without a file given, "~printout.txt" is removed. Returns the result.
/// Original: `printouts_print_club_report` (0x7d4460, printouts.cpp, cdecl) [name: none in the map]
pub(crate) unsafe extern "C" fn printouts_print_club_report(club: usize, reserves: u32, kind: u32, file: usize, print: u32) -> u32 {
    if club == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x58);
        return 0;
    }
    let (day, year) = printout_club_period_start(club);
    let (mut date, mut out) = ([0u8; 8], [0u8; 8]);
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    exe::date_operator_3d(PRINTOUT_PERIOD_START, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    exe::new_date(date.as_mut_ptr() as usize, read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2).wrapping_add(1) as u32);
    exe::date_operator_3d(PRINTOUT_PERIOD_END, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    let name = if file != 0 { file } else { PRINTOUT_TEMP_FILE };
    let f = exe::crt_fopen(name, 0x99c370usize);
    if f == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x71);
        return 0;
    }
    covered!("tel25_8 club printout");
    let mut club_name = [0u8; 0xc8];
    let national_name: unsafe extern "C" fn(usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x542100usize);
    national_name(club, 0, club_name.as_mut_ptr() as usize, 0xc8, 3, 1);
    exe::date_get_long_date_string(G_CURRENT_DATE, PRINTOUT_SCRATCH_TEXT, 0x7d0, 0xff);
    comp_lang_format()(PRINTOUT_LINE, 0xa78a20, 0x64u32, club_name.as_ptr(), PRINTOUT_SCRATCH_TEXT);
    printout_write_all(f, &[PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_LINE, PRINTOUT_NEWLINE, PRINTOUT_STARS, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE, PRINTOUT_NEWLINE]);
    let kind = kind as u8;
    let mut result = 1;
    if kind & 1 != 0 {
        printout_club_squad(club, f);
    }
    for (bit, writer) in [(2u8, 0x7d5280usize), (4, 0x7d60c0), (8, 0x7d69f0)] {
        if kind & bit != 0 {
            let w: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(writer);
            if w(club, reserves, f) == 0 {
                result = 0;
            }
        }
    }
    exe::crt_fclose(f);
    if print != 0 {
        let f = exe::crt_fopen(name, 0x9899fcusize);
        if f == 0 {
            tel26_error_box(PRINTOUTS_CPP, 0xaf);
            return 0;
        }
        covered!("tel25_8 print the club printout");
        let begin: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8033e0usize);
        let line: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x803710usize);
        let page_break: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x803930usize);
        let print_job: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x803a90usize);
        let end: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8035b0usize);
        begin(G_SCREEN_MANAGER, 0x8c);
        while printout_get_line(PRINTOUT_LINE, 0xc8, f) != 0 {
            let n = printout_text_length(PRINTOUT_LINE);
            if n != 0 && read_u8(PRINTOUT_LINE + n - 1) == b'\n' {
                write_u8(PRINTOUT_LINE + n - 1, 0);
            }
            if read_u8(PRINTOUT_LINE) == b' ' && read_u8(PRINTOUT_LINE + 1) == 0 {
                page_break(G_SCREEN_MANAGER);
            } else {
                line(G_SCREEN_MANAGER, PRINTOUT_LINE);
            }
        }
        print_job(G_SCREEN_MANAGER);
        end(G_SCREEN_MANAGER);
        exe::crt_fclose(f);
    }
    if file == 0 {
        exe::crt_remove(PRINTOUT_TEMP_FILE as *const u8);
    }
    result
}

/// The file the comparer's copy of a club printout writes to.
const PRINTOUT_CLUB_COPY_FILE: &[u8] = b"~printout_club_copy.txt\0";

/// The whole text of a closed file by name ("rb"), or None.
unsafe fn printout_named_file_text(name: usize) -> Option<Vec<u8>> {
    let f = exe::crt_fopen(name, b"rb\0".as_ptr() as usize);
    if f == 0 {
        return None;
    }
    let t = printout_file_text(f);
    exe::crt_fclose(f);
    Some(t)
}

/// Comparer for `printouts_print_club_report`. With a file given and no printing: the copy first writes the same
/// printout into a scratch file (its writers 0x7d5280, 0x7d60c0 and 0x7d69f0 through their own comparers), then the
/// original writes the real one, and the two files (and the returns and the period) must match; the scratch file is
/// removed. Otherwise (the temporary file or the printer) only the original runs, and the period it leaves
/// ([`PRINTOUT_PERIOD_START`], [`PRINTOUT_PERIOD_END`]) is checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_printouts_print_club_report(club: usize, reserves: u32, kind: u32, file: usize, print: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32>(%%I2%%);
    if club == 0 {
        return o(club, reserves, kind, file, print);
    }
    let (day, year) = printout_club_period_start(club);
    let plan = (day, year, day, (year + 1) & 0xffff);
    let period = || (read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2) as u32,
                     read_u16(PRINTOUT_PERIOD_END) as u32, read_u16(PRINTOUT_PERIOD_END + 2) as u32);
    if file == 0 || print != 0 {
        let r = o(club, reserves, kind, file, print);
        let got = period();
        SCOREBOARD.record(got == plan, || format!("club printout ({:#x}, kind {:#x}): period {:?} x plan {:?}", club, kind, got, plan));
        return r;
    }
    let copy_name = PRINTOUT_CLUB_COPY_FILE.as_ptr() as usize;
    let rr = printouts_print_club_report(club, reserves, kind, copy_name, 0);
    let got_r = period();
    let ro = o(club, reserves, kind, file, print);
    let got = period();
    let (to, tr) = (printout_named_file_text(file), printout_named_file_text(copy_name));
    exe::crt_remove(copy_name as *const u8);
    let (to, tr) = (to.unwrap_or_default(), tr.unwrap_or_default());
    let k = to.iter().zip(tr.iter()).position(|(x, y)| x != y).unwrap_or(to.len().min(tr.len()));
    SCOREBOARD.record(ro == rr && got == plan && got_r == plan && to == tr, || {
        format!("club printout ({:#x}, kind {:#x}): return {} x {}, period {:?} / {:?} x plan {:?}, text {} x {} bytes, first difference at {} ({:?} x {:?})",
                club, kind, ro, rr, got, got_r, plan, to.len(), tr.len(), k, printout_snippet(&to, k), printout_snippet(&tr, k))
    });
    ro
}
