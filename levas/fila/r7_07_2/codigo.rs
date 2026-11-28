// --- r7_07_2: the player search's option mask and the test of one candidate (search_filters.cpp, 0x806630 and
// 0x807450; the rest of SearchFilters is the sx20_13's) ---

/// The text search of the filters: a person's names, a nation, a continent, a club and a division against the text at
/// `SEARCH_TEXT` (the routines of the sx19b_10/sx19b_11, called by their addresses).
const FN_SEARCH_TEXT_MATCHES_STAFF: usize = 0x806080; // cdecl(person, text) -> matches
const FN_SEARCH_TEXT_MATCHES_NATION: usize = 0x805fb0; // cdecl(nation, text)
const FN_SEARCH_TEXT_MATCHES_CONTINENT: usize = 0x805f10; // cdecl(continent, text)
const FN_SEARCH_TEXT_MATCHES_CLUB: usize = 0x806030; // cdecl(club, text)
const FN_SEARCH_TEXT_MATCHES_DIVISION: usize = 0x8061f0; // cdecl(division, text)
/// `Cash::set_from_preset_index` (0x43f620, thiscall(cash, index) ret 4 -> the amount): the value bands' tops.
const FN_CASH_SET_FROM_PRESET_INDEX: usize = 0x43f620;
/// The free text of the filters (zero: no text search).
const SEARCH_TEXT: usize = 0x7f;
/// The option mask the defaults leave (`search_filters_set_defaults`): every candidate passes.
const SEARCH_DEFAULT_LOW: u32 = 0x14140081;
const SEARCH_DEFAULT_HIGH: u32 = 0x44aaa884;

/// The human manager on the screen's slot (`G_SCREEN_RECORDS_MANAGER`, 0x300 bytes per slot).
unsafe fn search_screen_manager() -> Staff {
    Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize).wrapping_mul(0x300))))
}

/// The nation the manager's club plays in, else the manager's nation (null when neither).
unsafe fn search_manager_home_nation(manager: Staff) -> usize {
    let club = manager.club_job();
    if !club.is_null() && read_usize(club.addr() + club::NATION) != 0 {
        return read_usize(club.addr() + club::NATION);
    }
    manager.nation().addr()
}

/// The manager's home nation for the candidate tests: his club's nation, else his national side's, else his own.
unsafe fn search_filter_home_nation(manager: Staff) -> usize {
    let club = manager.club_job();
    if !club.is_null() && read_usize(club.addr() + club::NATION) != 0 {
        return read_usize(club.addr() + club::NATION);
    }
    let side = manager.national_job();
    if side.is_null() { manager.nation().addr() } else { read_usize(side.addr() + club::NATION) }
}

/// An empty nation filter (-1) takes the human manager's: his club's nation, else his own; without a manager the error
/// box at line 0x38b and -1.
unsafe fn search_fill_nation(this: usize, field: usize) {
    if read_i32(this + field) != -1 {
        return;
    }
    let manager = search_screen_manager();
    if manager.is_null() {
        show_error_at(SEARCH_FILTERS_SOURCE, 0x38b);
        write_i32(this + field, -1);
        return;
    }
    let nation = search_manager_home_nation(manager);
    write_i32(this + field, if nation == 0 { -1 } else { read_i32(nation) });
}

/// An empty region filter (-1) takes the human manager's nation's region (`nation::ACTUAL_REGION`), as above; line
/// 0x3a9.
unsafe fn search_fill_region(this: usize, field: usize) {
    if read_i8(this + field) != -1 {
        return;
    }
    let manager = search_screen_manager();
    if manager.is_null() {
        show_error_at(SEARCH_FILTERS_SOURCE, 0x3a9);
        write_i8(this + field, -1);
        return;
    }
    let nation = search_manager_home_nation(manager);
    write_i8(this + field, if nation == 0 { -1 } else { read_i8(nation + nation::ACTUAL_REGION) });
}

/// The bits an option clears before it is set (low and high dwords to keep), for the options without their own rule.
fn search_option_group(low: u32, high: u32) -> Option<(u32, u32)> {
    let keep = match (low, high) {
        (0x80 | 0x100 | 0x200 | 0x400 | 0x800 | 0x1000 | 0x2000 | 0x4000 | 0x8000 | 0x10000 | 0x20000, 0) => (0xfffc_007f, !0), // age bands
        (0x40000 | 0x80000, 0) => (0xfff3_ffff, !0),
        (0x4000000 | 0x8000000, 0) => (0xf3ff_ffff, !0),
        (0x10000000 | 0x20000000 | 0x40000000, 0) => (0x8fff_ffff, !0),
        (0, 4 | 8 | 0x10 | 0x20 | 0x40) => (!0, 0xffff_ff83),
        (0, 0x80 | 0x100 | 0x200 | 0x400) => (!0, 0xffff_f87f),
        (0, 0x800 | 0x1000) => (!0, !0x1800),
        (0, 0x2000 | 0x4000) => (!0, !0x6000),
        (0, 0x8000 | 0x10000) => (!0, 0xfffe_7fff),
        (0, 0x20000 | 0x40000) => (!0, 0xfff9_ffff),
        (0, 0x80000 | 0x100000) => (!0, 0xffe7_ffff),
        (0, 0x200000 | 0x400000) => (!0, 0xff9f_ffff),
        (0, 0x800000 | 0x1000000 | 0x2000000) => (!0, 0xfc7f_ffff),
        (0, 0x4000000 | 0x8000000 | 0x10000000 | 0x20000000) => (!0, 0xc3ff_ffff),
        (0, 0x40000000 | 0x80000000) => (!0, 0x3fff_ffff),
        _ => return None,
    };
    Some(keep)
}

/// Sets one option of the player search's 64-bit mask (`low`, `high`), clearing the other options of its group;
/// returns whether the mask changed.
///
/// `SearchFilters::set_option_flag(low, high)`: an option is one bit (any other value changes nothing). The
/// nationality options (low 1..0x40) clear low bits 0..5; 4, 8, 0x10 and 0x20 are remembered (+8/+0xc) and 0x40 sets
/// the remembered one instead; with 0x10 an empty nationality nation (`SEARCH_NATIONALITY_NATION`) and with 0x20 an
/// empty region (`SEARCH_NATIONALITY_REGION`) take the human manager's (`search_fill_nation`/`search_fill_region`).
/// The based-in options (low 0x100000..0x1000000) clear low bits 20..24; 0x800000 and 0x1000000 are remembered
/// (+0x10/+0x14) and 0x2000000 sets the remembered one; 0x800000 and 0x1000000 fill the based-in nation and region
/// the same way. Low 0x80000000 and high 1 and 2 clear low bit 31 and high bits 0..1; high 3 with low 0x80000000 only
/// clears them. The other groups (`search_option_group`): age bands (low 0x80..0x20000), low 0x40000/0x80000,
/// 0x4000000/0x8000000, 0x10000000..0x40000000 and the high pairs and groups up to 0x80000000.
/// Original: `SearchFilters::set_option_flag` (0x806630, search_filters.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn search_filters_set_option_flag(this: usize, low: u32, high: u32) -> u32 {
    let (old_low, old_high) = (read_u32(this), read_u32(this + 4));
    match (low, high) {
        (1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40, 0) => {
            covered!("r7_07_2 nationality option");
            let (low, high) = if low == 0x40 {
                (read_u32(this + 8), read_u32(this + 0xc))
            } else {
                if matches!(low, 4 | 8 | 0x10 | 0x20) {
                    write_u32(this + 8, low);
                    write_u32(this + 0xc, 0);
                }
                (low, high)
            };
            write_u32(this, old_low & 0xffff_ffc0 | low);
            write_u32(this + 4, old_high | high);
            match (low, high) {
                (0x10, 0) => search_fill_nation(this, SEARCH_NATIONALITY_NATION),
                (0x20, 0) => search_fill_region(this, SEARCH_NATIONALITY_REGION),
                _ => {}
            }
        }
        (0x100000 | 0x200000 | 0x400000 | 0x800000 | 0x1000000 | 0x2000000, 0) => {
            covered!("r7_07_2 based in option");
            let (low, high) = if low == 0x2000000 {
                (read_u32(this + 0x10), read_u32(this + 0x14))
            } else {
                if matches!(low, 0x800000 | 0x1000000) {
                    write_u32(this + 0x10, low);
                    write_u32(this + 0x14, 0);
                }
                (low, high)
            };
            write_u32(this, old_low & 0xfe0f_ffff | low);
            write_u32(this + 4, old_high | high);
            match (low, high) {
                (0x800000, 0) => search_fill_nation(this, SEARCH_BASED_NATION),
                (0x1000000, 0) => search_fill_region(this, SEARCH_BASED_REGION),
                _ => {}
            }
        }
        (0x80000000, 0) | (0, 1) | (0, 2) | (0x80000000, 3) => {
            write_u32(this, old_low & 0x7fff_ffff);
            write_u32(this + 4, old_high & 0xffff_fffc);
            if (low, high) != (0x80000000, 3) {
                write_u32(this, read_u32(this) | low);
                write_u32(this + 4, read_u32(this + 4) | high);
            }
        }
        _ => {
            if let Some((keep_low, keep_high)) = search_option_group(low, high) {
                covered!("r7_07_2 option group");
                write_u32(this, old_low & keep_low | low);
                write_u32(this + 4, old_high & keep_high | high);
            }
        }
    }
    (read_u32(this) != old_low || read_u32(this + 4) != old_high) as u32
}

/// The nationality options (low 2..0x20) and, with a national side, the eligibility for it.
unsafe fn search_nationality_matches(this: usize, low: u32, p: Staff, manager: Staff, home: usize) -> bool {
    let (nation, second) = (p.nation().addr(), p.second_nation().addr());
    let either = |test: &dyn Fn(usize) -> bool| (nation != 0 && test(nation)) || (second != 0 && test(second));
    if low & 2 != 0 && nation != home && second != home {
        return false; // the manager's nation
    }
    if low & 4 != 0 && !either(&|n| read_i8(n + nation::GROUP_MEMBERSHIP) == 2) {
        return false;
    }
    if low & 8 != 0 && (second == 0 || read_u32(second) != read_u32(KEY_NATION_PAYS_BASQUE)) {
        return false;
    }
    if low & 0x10 != 0 && !either(&|n| read_i32(n) == read_i32(this + SEARCH_NATIONALITY_NATION)) {
        return false;
    }
    if low & 0x20 != 0 && !either(&|n| read_i8(n + nation::ACTUAL_REGION) == read_i8(this + SEARCH_NATIONALITY_REGION)) {
        return false;
    }
    let side = manager.national_job();
    if side.is_null() || p.player().is_null() {
        return true;
    }
    if low & 2 != 0 && home == read_usize(side.addr() + club::NATION) && exe::national_teams_player_is_eligible(read_usize(G_NATIONAL_TEAMS), p.addr(), manager.addr()) == 0 {
        return false;
    }
    if read_u32(this) & 0x10 != 0 && read_i32(this + SEARCH_NATIONALITY_NATION) == read_i32(read_usize(manager.national_job().addr() + club::NATION))
        && exe::national_teams_player_is_eligible(read_usize(G_NATIONAL_TEAMS), p.addr(), manager.addr()) == 0
    {
        return false;
    }
    true
}

/// The age bands (low 0x100..0x20000), on the age as a signed byte.
fn search_age_matches(low: u32, age: i8) -> bool {
    const BANDS: [(u32, i8, i8); 10] = [(0x100, i8::MIN, 21), (0x200, 19, 25), (0x400, 20, 30), (0x800, 25, 32), (0x1000, 30, i8::MAX), (0x2000, i8::MIN, 30),
        (0x4000, 30, 40), (0x8000, 40, 50), (0x10000, 50, 60), (0x20000, 60, i8::MAX)];
    BANDS.iter().all(|&(bit, lo, hi)| low & bit == 0 || (lo..=hi).contains(&age))
}

/// The language filter (`SEARCH_LANGUAGE` 1..3: the manager's home nation's first, second or third language, the
/// second and third only when set, 1..0x40): the candidate's fluency in it at least 4.
unsafe fn search_language_matches(this: usize, p: Staff, home: usize) -> bool {
    let fluent = |language: i8| exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p.addr(), 0, language as u8 as u32, 1) as i8 >= 4;
    match read_u8(this + SEARCH_LANGUAGE) {
        1 => fluent(read_i8(home + nation::FIRST_LANGUAGE)),
        2 => {
            let l = read_i8(home + nation::SECOND_LANGUAGE);
            !(1..=0x40).contains(&l) || fluent(l)
        }
        3 => {
            let l = read_i8(home + nation::THIRD_LANGUAGE);
            !(1..=0x40).contains(&l) || fluent(l)
        }
        _ => true,
    }
}

/// The based-in options (low 0x200000..0x1000000) on the nation the candidate plays in: his club's nation, else his
/// national side's (unless his job there is 0xb), else his own.
unsafe fn search_based_in_matches(this: usize, low: u32, p: Staff, home: usize) -> bool {
    let club = p.club_job();
    let based = if !club.is_null() && read_usize(club.addr() + club::NATION) != 0 {
        read_usize(club.addr() + club::NATION)
    } else if !p.national_job().is_null() && p.job_for_nation() != 0xb {
        read_usize(p.national_job().addr() + club::NATION)
    } else {
        p.nation().addr()
    };
    !(low & 0x200000 != 0 && based != home)
        && !(low & 0x400000 != 0 && based == home)
        && !(low & 0x800000 != 0 && (based == 0 || read_i32(based) != read_i32(this + SEARCH_BASED_NATION)))
        && !(low & 0x1000000 != 0 && (based == 0 || read_i8(based + nation::ACTUAL_REGION) != read_i8(this + SEARCH_BASED_REGION)))
}

/// A value band's top (`Cash::set_from_preset_index` on a zeroed amount).
unsafe fn search_band_amount(band: i8) -> i32 {
    let mut cash = 0u32;
    let preset: unsafe extern "thiscall" fn(*mut u32, u32) -> i32 = core::mem::transmute(FN_CASH_SET_FROM_PRESET_INDEX);
    preset(&mut cash, band as i32 as u32)
}

/// The value filter (low 0x8000000, `SEARCH_VALUE_BAND`): -1 only rolling contracts in nations with free transfers;
/// 0..6 a value up to the band's top; 7 a value of at least band 6's.
unsafe fn search_value_matches(this: usize, p: Staff, contract: usize) -> bool {
    let band = read_i8(this + SEARCH_VALUE_BAND);
    if band == -1 && contract != 0 {
        if exe::staff_contract_is_rolling(contract) == 0 {
            return false;
        }
        if exe::nation_allows_free_transfers(read_usize(club_at(read_i32(contract + 4) as isize).addr() + club::NATION)) == 0 {
            return false;
        }
    }
    let value = read_i32(p.addr() + staff::VALUE);
    if (0..7).contains(&band) && value > search_band_amount(band) {
        return false;
    }
    !(band == 7 && value < search_band_amount(6))
}

/// The contract options: high 1 a rolling contract, high 2 a fixed one ending within a year at a club, low 0x80000000
/// out of FIFA's protected period against the manager's club.
unsafe fn search_contract_matches(low: u32, high: u32, p: Staff, manager: Staff, contract: usize) -> bool {
    if high & 1 != 0 && contract != 0 && exe::staff_contract_is_rolling(contract) == 0 {
        return false;
    }
    if high & 2 != 0 {
        if p.club_job().is_null() || contract == 0 {
            return false;
        }
        if exe::staff_contract_is_rolling(contract) != 0 || exe::staff_contract_days_to_end_date(contract) >= 0x16d {
            return false;
        }
    }
    if low & 0x80000000 != 0 && (contract == 0 || exe::staff_contract_check_fifa_protected_period(contract, manager.club_job().addr(), 0, 0) != 0) {
        return false;
    }
    true
}

/// The position options on `formation_get_position_mask` of the player sheet: high 8 goalkeeper (bit 0), 0x10 defence
/// (6), 0x20 (0x38), 0x40 (0x40), 0x100 (0x80), 0x200 (0x100/0x200/0x400), 0x400 (0x800). The mask is read with
/// high 4 clear, and again with 0x80 clear while it is still 0xffff (its starting value, or read as that).
unsafe fn search_position_matches(high: u32, p: Staff) -> bool {
    let read = || exe::formation_get_position_mask(p.player().addr());
    let mut mask = 0xffffu16;
    if high & 4 == 0 {
        mask = read();
        let m = mask;
        if (high & 8 != 0 && m & 1 == 0) || (high & 0x10 != 0 && m & 6 == 0) || (high & 0x20 != 0 && m & 0x38 == 0) || (high & 0x40 != 0 && m & 0x40 == 0) {
            return false;
        }
    }
    if high & 0x80 == 0 {
        if mask == 0xffff {
            mask = read();
        }
        let m = mask;
        if (high & 0x100 != 0 && m & 0x80 == 0) || (high & 0x200 != 0 && m & 0x700 == 0) || (high & 0x400 != 0 && m & 0x800 == 0) {
            return false;
        }
    }
    true
}

/// The candidate's job for the job filter: at his club, else at his national side (unless that is 0xb), else 0 with
/// job filter 9 or his club job byte.
unsafe fn search_candidate_job(this: usize, p: Staff) -> u8 {
    if !p.club_job().is_null() {
        return p.job_for_club();
    }
    if !p.national_job().is_null() && p.job_for_nation() != 0xb {
        return p.job_for_nation();
    }
    if read_u8(this + SEARCH_JOB) == 9 { 0 } else { p.job_for_club() }
}

/// The job filter (`SEARCH_JOB` 1..9) on the candidate's job.
fn search_job_matches(filter: u8, job: u8) -> bool {
    match filter {
        1 => job == 1,
        2 => matches!(job, 2 | 3 | 4), // r7_07_2: coaches
        3 => matches!(job, 5 | 0xc),
        4 => matches!(job, 6 | 7 | 0xd),
        5 => matches!(job, 8 | 0xf),
        6 => job == 9,
        7 => job == 0xa,
        8 => matches!(job, 0xb | 0xc | 0xd | 0xf),
        9 => job == 0,
        _ => true,
    }
}

/// The interest option (high 0x10000 with 0x8000 clear): the manager has a club; a player of reputation up to the
/// club's + 5000 interested in joining (`exe::shortlist_manager_staff_interest_in_joining_club`), a non-player up to
/// the same in the job (`exe::staff_interest_in_job_at_club`, with the job); interest below 3.15 fails (and NaN).
unsafe fn search_interest_matches(p: Staff, manager: Staff, job: u8) -> bool {
    let club = manager.club_job();
    if club.is_null() {
        return false;
    }
    let ceiling = read_i16(club.addr() + club::REPUTATION) as i32 + 5000;
    let shortlist = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    let interest = if !p.player().is_null() {
        if read_i16(p.player().addr() + 0xd) as i32 > ceiling {
            return false; // +0xd: the player's current reputation
        }
        exe::shortlist_manager_staff_interest_in_joining_club(shortlist, p.addr(), club.addr(), 1, 0, 1, 0)
    } else if !p.non_player().is_null() {
        if read_i16(p.non_player().addr() + 0xc) as i32 > ceiling {
            return false; // +0xc: the non-player's current reputation
        }
        exe::staff_interest_in_job_at_club(shortlist, p.addr(), club.addr(), 1, job as u32, 0)
    } else {
        return true;
    };
    interest >= read_f64(rdata::F64_3_15)
}

/// The free text (`SEARCH_TEXT`): the candidate's names, his nation or its continent, his club, its division, its
/// nation or that nation's continent.
unsafe fn search_text_matches(this: usize, p: Staff) -> bool {
    let text = this + SEARCH_TEXT;
    let test = |f: usize, x: usize| {
        let matches: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(f);
        x != 0 && matches(x, text) != 0
    };
    let continent = |n: usize| if n == 0 { 0 } else { read_usize(n + nation::CONTINENT) };
    let nation = p.nation().addr();
    let club = p.club_job().addr();
    let club_nation = if club == 0 { 0 } else { read_usize(club + club::NATION) };
    test(FN_SEARCH_TEXT_MATCHES_STAFF, p.addr())
        || test(FN_SEARCH_TEXT_MATCHES_NATION, nation)
        || test(FN_SEARCH_TEXT_MATCHES_CONTINENT, continent(nation))
        || test(FN_SEARCH_TEXT_MATCHES_CLUB, club)
        || test(FN_SEARCH_TEXT_MATCHES_DIVISION, if club == 0 { 0 } else { read_usize(club + club::DIVISION) })
        || test(FN_SEARCH_TEXT_MATCHES_NATION, club_nation)
        || test(FN_SEARCH_TEXT_MATCHES_CONTINENT, continent(club_nation))
}

/// One attribute range table: of the attributes 1..count whose range is not the whole 1..20, how many hold a value in
/// range (stopping when `needed` are or when `set` were seen) must reach `needed`. `value` gives the attribute or None
/// when the manager cannot see it (the candidate fails).
unsafe fn search_ranges_match(this: usize, minimums: usize, count: usize, set: usize, needed: usize, value: impl Fn(u8) -> Option<i8>) -> bool {
    let (mut seen, mut hits) = (0i8, 0i8);
    for a in 1..=count as u8 {
        let (lo, hi) = (read_i8(this + minimums + a as usize), read_i8(this + minimums + count + a as usize));
        if lo <= 1 && hi >= SEARCH_ATTRIBUTE_TOP as i8 {
            continue;
        }
        let Some(v) = value(a) else { return false };
        if (lo..=hi).contains(&v) {
            hits = hits.wrapping_add(1);
            if hits.wrapping_sub(1) as i32 == read_i8(this + needed) as i32 - 1 {
                break;
            }
        }
        seen = seen.wrapping_add(1);
        if seen.wrapping_sub(1) as i32 == read_i8(this + set) as i32 - 1 {
            break;
        }
    }
    hits >= read_i8(this + needed)
}

/// Whether a player search candidate passes every active filter.
///
/// `SearchFilters::player_matches_filter(candidate)`: no candidate, the error box (line 0x1f2) and 0; the default mask
/// (`search_filters_set_defaults`) passes everyone. Needs the human manager on the screen's slot (else box 0x1ff) and
/// his home nation (his club's, his national side's or his own; else box 0x20e). Then, each test skipped when its
/// "any" bit is set: the nationality (low bit 0 clear, `search_nationality_matches`), the age (low 0x80 clear,
/// `search_age_matches`), the language (low 0x40000 clear, `search_language_matches`), where he plays (low 0x100000
/// clear), the value (low 0x4000000 clear, `search_value_matches`, with the contract `exe::contract_manager_get_contract`),
/// listed for transfer (low 0x20000000) or loan (low 0x40000000) unless low 0x10000000, the scout requests (high
/// 0x40000000 clear), the contract (`search_contract_matches`), the positions (`search_position_matches`), the job
/// (high 0x800 clear), the injury (high 0x2000 clear; with 0x4000, injury status 1 or 2 fails), the interest
/// (`search_interest_matches`), the free text (high 0x200000 clear), the candidate's flags +0x29 (high 0x1000000) and
/// +0x2a (high 0x2000000) unless high 0x800000, and the player attribute ranges (high 0x20000 clear: visible to the
/// manager, `exe::staff_get_player_attribute`) and the non-player ones (high 0x80000 clear).
/// The original builds a STAFF_CONTRACT local (0x85e410) that nothing reads; the copy leaves it out. It passes the
/// language, the job and the attribute as dwords whose upper bytes are left from other values; the callees read the
/// byte.
/// Original: `SearchFilters::player_matches_filter` (0x807450, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_player_matches_filter(this: usize, candidate: usize) -> u32 {
    if candidate == 0 {
        show_error_at(SEARCH_FILTERS_SOURCE, 0x1f2);
        return 0;
    }
    let (low, high) = (read_u32(this), read_u32(this + 4));
    if low == SEARCH_DEFAULT_LOW && high == SEARCH_DEFAULT_HIGH {
        return 1;
    }
    let manager = search_screen_manager();
    if manager.is_null() {
        show_error_at(SEARCH_FILTERS_SOURCE, 0x1ff);
        return 0;
    }
    let home = search_filter_home_nation(manager);
    if home == 0 {
        show_error_at(SEARCH_FILTERS_SOURCE, 0x20e);
        return 0;
    }
    covered!("r7_07_2 candidate");
    let p = staff_at(read_i32(candidate + 0xc) as isize); // +0xc: the candidate's person id
    if low & 1 == 0 && !search_nationality_matches(this, low, p, manager, home) {
        return 0;
    }
    if low & 0x80 == 0 && !search_age_matches(low, read_i8(p.addr() + staff::AGE)) {
        return 0;
    }
    if low & 0x40000 == 0 && !search_language_matches(this, p, home) {
        return 0;
    }
    if low & 0x100000 == 0 && !search_based_in_matches(this, low, p, home) {
        return 0;
    }
    let mut contract = 0usize;
    if low & 0x4000000 == 0 {
        contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p.addr());
        if low & 0x8000000 != 0 && !search_value_matches(this, p, contract) {
            return 0;
        }
    }
    if low & 0x10000000 == 0 {
        if low & 0x20000000 != 0 && exe::staff_is_transfer_listed_or_requested(p.addr(), 0) == 0 {
            return 0;
        }
        if low & 0x40000000 != 0 && exe::staff_is_loan_listed(p.addr(), 0) == 0 {
            return 0;
        }
    }
    if high & 0x40000000 == 0 && exe::scout_manager_has_scout_request(read_usize(G_SCOUT_MANAGER), p.addr(), 0, 0) == 0 {
        return 0;
    }
    if low & 0x80000000 != 0 || high & 3 != 0 {
        if contract == 0 {
            contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p.addr());
        }
        if !search_contract_matches(low, high, p, manager, contract) {
            return 0;
        }
    }
    if !search_position_matches(high, p) {
        return 0;
    }
    let job = search_candidate_job(this, p);
    if high & 0x800 == 0 && !search_job_matches(read_u8(this + SEARCH_JOB), job) {
        return 0;
    }
    if high & 0x2000 == 0 {
        let status = exe::injury_get_player_injury_status(read_usize(G_INJURY), p.addr());
        if high & 0x4000 != 0 && (status == 2 || status == 1) {
            return 0;
        }
    }
    if high & 0x8000 == 0 && high & 0x10000 != 0 && !search_interest_matches(p, manager, job) {
        return 0;
    }
    if high & 0x200000 == 0 && read_u8(this + SEARCH_TEXT) != 0 && !search_text_matches(this, p) {
        return 0;
    }
    if high & 0x800000 == 0 && ((high & 0x1000000 != 0 && read_u8(candidate + 0x29) == 0) || (high & 0x2000000 != 0 && read_u8(candidate + 0x2a) == 0)) {
        return 0; // +0x29, +0x2a: the candidate's two flags
    }
    if high & 0x20000 == 0
        && !search_ranges_match(this, SEARCH_PLAYER_MINIMUMS, SEARCH_PLAYER_ATTRIBUTES, SEARCH_PLAYER_RANGES_SET, SEARCH_PLAYER_RANGES_SHOWN, |a| {
            (exe::staff_is_player_attribute_visible(manager.addr(), p.addr(), a as u32) != 0).then(|| exe::staff_get_player_attribute(p.addr(), a as u32))
        })
    {
        return 0;
    }
    if high & 0x80000 == 0
        && !search_ranges_match(this, SEARCH_STAFF_MINIMUMS, SEARCH_STAFF_ATTRIBUTES, SEARCH_STAFF_RANGES_SET, SEARCH_STAFF_RANGES_SHOWN, |a| {
            Some(exe::staff_get_non_player_attribute(p.addr(), a as u32))
        })
    {
        return 0;
    }
    covered!("r7_07_2 candidate passes");
    1
}

/// Comparer for `search_filters_set_option_flag`: side by side on the filter record and the answer
/// (`search_filters_compare`); without a human manager when a nation or region would be filled (the error box), only
/// the original.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_option_flag(this: usize, low: u32, high: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I0%%);
    let fills = matches!((low, high), (0x10 | 0x20 | 0x40, 0) | (0x800000 | 0x1000000 | 0x2000000, 0));
    if fills && search_screen_manager().is_null() {
        return o(this, low, high);
    }
    search_filters_compare("set_option_flag", this, || o(this, low, high), || search_filters_set_option_flag(this, low, high))
}

/// Comparer for `search_filters_player_matches_filter`: the original and the copy (they only read) and the answers;
/// the error boxes (no candidate, manager or home nation) only the original.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_player_matches_filter(this: usize, candidate: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    let manager = search_screen_manager();
    if candidate == 0 || manager.is_null() || search_filter_home_nation(manager) == 0 {
        return o(this, candidate);
    }
    let r = o(this, candidate);
    let n = search_filters_player_matches_filter(this, candidate);
    SCOREBOARD.record(r == n, || format!("player_matches_filter(person {}, mask {:#x}/{:#x}) original={} rust={}", read_i32(candidate + 0xc), read_u32(this), read_u32(this + 4), r, n));
    r
}
