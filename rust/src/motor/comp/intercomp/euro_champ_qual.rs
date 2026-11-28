use super::*;

/// `EuroChampQual::set_up_rules()` (0x57e8f0, thiscall, ret 0, euro_champ_qual.cpp; virtual +0x8c, from the constructor
/// in 2001-000; SEH frame for the local Date, no destructor): sets points, rounds, tie-breakers, qualification places
/// and the draw date, and builds the round table; returns 1.
///
/// 3 points for a win (+0xc2) and 1 for a draw (+0xc3), +0x4a = 3, two rounds (+0x3c = 2, word), match category 0xa
/// (+0xc4), +0x43 = 2, tie-breakers 1, 2, 4, 5 (+0xc5..+0xc8), one direct place (+0xbe = 1) and one play-off place
/// (+0xbf = 1), no relegation (+0xc0 = +0xc1 = 0), +0x49 = 7, +0xa7 = 0xffff (word).
///
/// The draw date (+0xb6, the day of the year) = Date(10, 2, year +0x40, 5) (`exe::date_date_549f70`: push 5; push year; push 2; push
/// 0xa; lea ecx, local -> thiscall, ret 0x10: 10 March moved to the Friday [inference]); +0xb8 = 0 (word); the round
/// table from virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) at +0xba.
///
/// Returns 1.
pub(crate) unsafe fn set_up_rules(this: League) -> u32 {
    this.set_win_points(3);
    this.set_substitutions_allowed(3);
    this.set_legs(2); // cqs8: dois turnos
    this.set_draw_points(1);
    this.set_match_category(match_category::NATIONAL_TEAMS);
    this.set_comp_type(comp_type::LEAGUE);
    for (k, v) in [1u8, 2, 4, 5].into_iter().enumerate() {
        write_u8(this.addr() + league::TIEBREAKS + k, v);
    }
    this.set_promoted_count(1);
    this.set_promotion_playoff_count(1);
    this.set_relegation_playoff_count(0);
    this.set_relegated_count(0);
    this.set_substitutes_named(7);
    this.set_games_per_club(0xffffu16 as i16);
    let mut data = [0u32; 2];
    // simplification: the original pushes the year in ecx with the high 16 bits of the cup pointer; 0x549f70 reads the word
    exe::date_date_549f70((&mut data) as *mut [u32; 2] as *mut [u16; 4], 0xa, 2, this.year() as u16 as u32, 5); // cqs8: the draw date
    this.set_next_draw_day((data[0] as u16) as i16);
    this.set_next_draw_year_offset((0) as i16);
    covered!("cqs_8 euro rules");
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = rounds(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    this.set_round_dates((t as u32) as usize);
    1
}

/// EuroChampQual::set_up_rules hook entry: sets the European Championship qualifying rules (points, two legs,
/// tiebreaks, one direct and one play-off place, draw date) and builds the round table (see set_up_rules); returns 1.
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_set_up_rules(this: usize) -> u32 {
    set_up_rules(League(this))
}

/// `g.get_group_team_count()` (`exe::eurochampqual_get_group_team_count`, thiscall, ret 4, euro_champ_qual.cpp): the number of teams in
/// group `g`, 6 or 5 depending on the format (+0xf6).
///
/// With format 1 (9 groups [inference]), groups 0, 2 and 5 have 6 national teams and the others 5; with format 2 (10
/// groups [inference]), groups 0, 2, 5 and 7 have 6 and the others 5; any other format shows the error box for line
/// 0x245 and returns 5. `g` is the argument's byte.
pub(crate) unsafe fn group_team_count(this: usize, g: u32) -> i16 {
    let g = g as u8;
    match read_i8(this + euro_champ_qual::GROUP_FORMAT) {
        1 => if matches!(g, 0 | 2 | 5) { 6 } else { 5 }, // cqs8: the groups of 6 in format 1
        2 => if matches!(g, 0 | 2 | 5 | 7) { 6 } else { 5 },
        _ => {
            show_error_at(0x9c4674, 0x245);
            5
        }
    }
}

/// g.get_group_team_count(): number of nations in qualifying group g of the European Championship (6 or 5
/// depending on the format at +0xf6; see group_team_count).
pub(crate) unsafe extern "thiscall" fn get_group_team_count(this: usize, g: u32) -> i16 {
    group_team_count(this, g)
}

/// `clube.get_qualification_status()` (`0x580b50`, thiscall, ret 4, `euro_champ_qual.cpp`; the virtual
/// giving a club's status): −2 if the low byte of the club's stat `0x12` is below 14, otherwise −3.
///
/// Reads the club's stat `0x12` from the cup statistics (Comp_Stats::get_club_stat `0x4a2e10`: push 0x12; push club;
/// mov ecx, [this+8] -> thiscall, ret 8, returns a float), truncated (_ftol `0x945640`): with the low byte below 14
/// (signed), −2; otherwise −3 (word).
pub(crate) unsafe fn qualification_status(this: usize, club: usize) -> i16 {
    let x = ftol(exe::comp_stats_get_club_stat(read_usize(this + 8), club, 0x12));
    ((((x as u8 as i8) < 0xe) as i32) - 3) as i16 // cqs8: the situation by statistic 0x12
}

/// club.get_qualification_status(): the club's qualification status in the European Championship
/// qualifiers, from its competition stat 0x12 (Comp_Stats): -2 if that value (low byte, signed) is below 14, else -3.
pub(crate) unsafe extern "thiscall" fn get_qualification_status(this: usize, club: usize) -> i16 {
    qualification_status(this, club)
}

/// EuroChampQual::init_group_one_table(): sets up group 1 of the European Championship qualifiers (the cup itself):
/// stores the group size (+0x3e), allocates the league table (one 0x3b-byte row per team at +0xb1), sets the cached
/// match count (+0xa7) to (teams - 1) x rounds (+0x3c), fills one table row per national team from the list at +0xf7
/// and sends the group-draw news. Returns 1, or 0 out of memory.
pub(crate) unsafe extern "thiscall" fn init_group_one_table(this: usize) -> u32 {
    let this = League(this);
    let n = exe::eurochampqual_get_group_team_count(this, 0xffff_ffff);
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 * 0x3b) as u32);
    this.set_table((t as u32) as usize);
    if t == 0 {
        show_error_at(0x9c4674, 0x2d2);
        return 0;
    }
    let n = this.club_count();
    this.set_games_per_club(n.wrapping_sub(1).wrapping_mul(this.legs())); // cqs8: the group 1 rounds
    covered!("cqs_8 euro group 1");
    let mut i: i8 = 0;
    if n > 0 {
        loop {
            let c = read_usize(read_usize(this.addr() + euro_champ_qual::DRAWN_TEAMS) + i as isize as usize * 4);
            exe::league_init_table_row(this, i as u8 as u32, c, 0, 0);
            i = i.wrapping_add(1);
            if (i as i16) >= this.club_count() {
                break;
            }
        }
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Rule: the European Championship qualifying entrants are drawn into the groups: in 1998 by a fixed table, otherwise
/// by up to six pots in which the group order is shuffled and each group in turn takes the next entrant while it has
/// room.
///
/// `this.draw_groups()`: allocates the drawn array (`euro_champ_qual::DRAWN_TEAMS`, 6 dwords per group, groups = +0xff
/// + 2: `exe::crt_malloc`, cdecl; out of memory: the error box at line 0x262 and nothing more) and fills it
/// (`draw_groups_into`). The slots no entrant reaches keep what the allocation left.
/// Original: `EuroChampQual::draw_groups` (0x57dab0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_draw_groups(this: usize) {
    let groups = read_u8(this + 0xff).wrapping_add(2);
    let a = exe::crt_malloc(((groups.wrapping_mul(6) as i8 as i32) << 2) as u32);
    write_usize(this + euro_champ_qual::DRAWN_TEAMS, a);
    if a == 0 {
        show_error_at(0x9c4674, 0x262);
        return;
    }
    draw_groups_into(this, a);
}

/// Comparer for `euro_champ_qual_draw_groups`: the original runs for real (its array stays); from the generator state
/// before it, the copy draws again into a local array without counting the draws; the generator state after both and
/// every dword the copy wrote must equal the original's array. (In 1998 the copy also rewrites the entrants with the same
/// national teams.)
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_draw_groups(this: usize) {
    let rng_before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EURO_CHAMP_QUAL_DRAW_GROUPS)(this);
    let a = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
    if a == 0 {
        return;
    }
    let rng_after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_before.write_back();
    let slots = (read_u8(this + 0xff).wrapping_add(2) as usize * 6).max(54);
    let local = vec![0u32; slots];
    let written = crate::rng::pause_trace(|| crate::rng::without_counting(|| draw_groups_into(this, local.as_ptr() as usize)));
    let rng_copy = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_after.write_back();
    let bad = written.iter().find(|&&k| local[k] as usize != read_usize(a + 4 * k)).copied();
    SCOREBOARD.record(bad.is_none() && rng_copy == rng_after, || format!("euro_champ_qual_draw_groups({:#x}): {} slots written, first different {:?}, generator equal {}",
        this, written.len(), bad, rng_copy == rng_after));
}

/// The work of `draw_groups` over the drawn array `drawn` (6 dwords per group): returns the indices of the dwords it
/// wrote, in order.
///
/// Edition year (+0x40, word) 1998: the fixed draw `EURO_QUAL_1998_DRAW`: entrant k (the entrants array
/// `+0xee`) becomes the national team of the nation of its key (`exe::database_get_national_team`([`G_NATION_TABLE`] +
/// `nation::SIZE` × key), cdecl) and goes into its slot. Other years: groups = +0xff + 2 (a signed byte; the
/// constructor sets +0xff to 7, so 9); a group order 0..8 and a fill count per group (zero); entrant i = 0. Up to 6 pot
/// rounds, while i is below the entrant count (+0xf2, dword): the order is shuffled by 20 swaps of two positions drawn
/// with `exe::utils_random`(groups) (cdecl); then, for each position j of the order (while i is below the count), its
/// group g (skipped when g ≥ groups) takes entrant i when its fill count is below
/// `EuroChampQual::get_group_team_count`(g − 1) (`exe::eurochampqual_get_group_team_count`: push; ecx = this ->
/// thiscall, ret 4; the byte g − 1, a defect of the original: the capacity of the previous group, so in format 1 the
/// groups of six are 1, 3 and 6, and group 0 asks for group −1), at slot g × 6 + fill; then i + 1 and fill + 1.
pub(crate) unsafe fn draw_groups_into(this: usize, drawn: usize) -> Vec<usize> {
    let mut written = Vec::new();
    let entrants = read_usize(this + 0xee);
    if read_u16(this + 0x40) == 0x7ce {
        covered!("gra_22 the fixed 1998 draw");
        for (k, &(key, slot)) in EURO_QUAL_1998_DRAW.iter().enumerate() {
            let team = exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(nation::SIZE)));
            write_usize(entrants + 4 * k, team);
            write_usize(drawn + 4 * slot as usize, read_usize(entrants + 4 * k));
            written.push(slot as usize);
        }
        return written;
    }
    let groups = read_u8(this + 0xff).wrapping_add(2) as i8;
    // ponytail: 16-entry tables; the original has 9 (groups is always 9; above 9 it reads stack leftovers)
    let mut order: [i8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0];
    let mut fill = [0i8; 16];
    let mut i: i8 = 0;
    let count = read_i32(this + 0xf2);
    for _pot in 0..6 {
        if i as i32 >= count {
            break;
        }
        for _ in 0..0x14 { // gra22: 20 swaps per pot
            let a = exe::utils_random(groups as i32) as i8 as usize;
            let b = exe::utils_random(groups as i32) as i8 as usize;
            order.swap(a, b);
        }
        covered!("gra_22 pot drawn");
        let mut j: i8 = 0;
        while j < groups {
            if i as i32 >= count {
                break;
            }
            let g = order[j as usize];
            if g < groups {
                let c = fill[g as usize];
                let cap = exe::eurochampqual_get_group_team_count(this, g.wrapping_sub(1) as u8 as u32); // gra22: the capacity asked for g - 1
                if (c as i16) < cap {
                    let slot = (c as i32 + g as i32 * 6) as usize;
                    write_usize(drawn + 4 * slot, read_usize(entrants + 4 * i as usize));
                    written.push(slot);
                    i += 1;
                    fill[g as usize] = c + 1;
                }
            }
            j += 1;
        }
    }
    written
}

/// The fixed draw of the 1998 edition (the qualifiers of Euro 2000): for each entrant k, its nation key and its slot in
/// the drawn array (group × 6 + place; 9 groups, groups 1, 3, 6 and 8 with six teams).
pub(crate) const EURO_QUAL_1998_DRAW: [(usize, u8); 49] = [
    (KEY_NATION_BELARUS, 0), (KEY_NATION_WALES, 1), (KEY_NATION_SWITZERLAND, 2), (KEY_NATION_DENMARK, 3), (KEY_NATION_ITALY, 4),
    (KEY_NATION_SLOVENIA, 6), (KEY_NATION_LATVIA, 7), (KEY_NATION_GEORGIA, 8), (KEY_NATION_GREECE, 9), (KEY_NATION_NORWAY, 10),
    (KEY_NATION_ALBANIA, 11), (KEY_NATION_MOLDOVA, 12), (KEY_NATION_NORTHERN_IRELAND, 13), (KEY_NATION_FINLAND, 14),
    (KEY_NATION_TURKEY, 15), (KEY_NATION_GERMANY, 16), (KEY_NATION_ARMENIA, 18), (KEY_NATION_ICELAND, 19),
    (KEY_NATION_UKRAINE, 20), (KEY_NATION_FRANCE, 21), (KEY_NATION_RUSSIA, 22), (KEY_NATION_ANDORRA, 23),
    (KEY_NATION_LUXEMBOURG, 24), (KEY_NATION_POLAND, 25), (KEY_NATION_SWEDEN, 26), (KEY_NATION_BULGARIA, 27),
    (KEY_NATION_ENGLAND, 28), (KEY_NATION_SAN_MARINO, 30), (KEY_NATION_CYPRUS, 31), (KEY_NATION_ISRAEL, 32),
    (KEY_NATION_AUSTRIA, 33), (KEY_NATION_SPAIN, 34), (KEY_NATION_LIECHTENSTEIN, 36), (KEY_NATION_HUNGARY, 37),
    (KEY_NATION_SLOVAKIA, 38), (KEY_NATION_PORTUGAL, 39), (KEY_NATION_ROMANIA, 40), (KEY_NATION_AZERBAIJAN, 41),
    (KEY_NATION_MALTA, 42), (KEY_NATION_FYR_OF_MACEDONIA, 43), (KEY_NATION_REPUBLIC_OF_IRELAND, 44), (KEY_NATION_CROATIA, 45),
    (KEY_NATION_YUGOSLAVIA, 46), (KEY_NATION_SCOTLAND, 48), (KEY_NATION_FAROE_ISLANDS, 49),
    (KEY_NATION_BOSNIA_HERZEGOVINA, 50), (KEY_NATION_LITHUANIA, 51), (KEY_NATION_ESTONIA, 52), (KEY_NATION_CZECH_REPUBLIC, 53),
];

/// Builds the European Championship qualifiers on the League base: League::League (`exe::new_league`), the competition
/// record in `league::COMPETITION`, the vtable 0x96a0d4, `club_sides` 4, `upper_division_id` and `lower_division_id`
/// −1, `active` 0, the drawn teams, the entrants, their count and the qualified flag (+0xfb) 0, the hosts count
/// (`euro_champ_qual::GROUP_FORMAT`) 1, `discipline_rule` 0, the last group stage index (+0xff) 7, the runners-up stage
/// index (+0x100) 8 and the play-off stage index (+0x101) 9, the edition `year` (the next finals year from `year` − 1,
/// `euro_qual_edition`), `league_flags` 0x24, the stored group results (+0x10, a new `LLIST` of 0xe bytes:
/// `exe::crt_operator_new` and `exe::new_llist`, or null) and `importance` 0xf. If the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1) nonzero), returns this.
///
/// Otherwise: the rules (`euro_champ_qual_set_up_rules`; 0: the box at line 0x5e and this), the statistics
/// (`exe::crt_operator_new`(0x5ce) and `exe::new_comp_stats`; null: the box at line 0x66 and this), `stage_index`
/// `stage_index::MAIN`, `current_stage` −1, `stage_count` 10 and the stage vector (`exe::crt_malloc`(0x28),
/// unchecked). When the season `year` (signed word) is after the edition (only a 1999 start: the 1998 edition), the
/// Euro 2000 groups and results (EuroChampQual::create_groups_and_results 0x580bd0). When the edition is more than two
/// years after `year`, `active` 0 again. Returns this.
/// Original: `EuroChampQual::EuroChampQual` (0x57ce10, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_euro_champ_qual(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let l = this;
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96a0d4); // the qualifiers' vtable
    l.set_club_sides(4);
    l.set_upper_division_id(-1);
    l.set_lower_division_id(0xffff_ffff);
    l.set_active(0);
    write_usize(this.addr() + euro_champ_qual::DRAWN_TEAMS, 0);
    write_usize(this.addr() + 0xee, 0); // the entrants
    write_u32(this.addr() + 0xf2, 0); // their count
    write_u32(this.addr() + 0xfb, 0); // the qualified flag
    write_u8(this.addr() + euro_champ_qual::GROUP_FORMAT, 1);
    l.set_discipline_rule(0);
    write_u8(this.addr() + 0xff, 7); // the last group stage index
    write_u8(this.addr() + 0x100, 8); // the runners-up stage index
    write_u8(this.addr() + 0x101, 9); // the play-off stage index
    let season = year as u16 as i16;
    l.set_year(euro_qual_edition(season.wrapping_sub(1)));
    l.set_league_flags(0x24);
    let p = exe::crt_operator_new(0xe);
    write_usize(this.addr() + 0x10, if p != 0 { exe::new_llist(p) } else { 0 }); // the stored group results
    l.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    if euro_champ_qual_set_up_rules(this.addr()) == 0 {
        show_error_at(0x9c4674, 0x5e); // euro_champ_qual.cpp
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    l.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(0x9c4674, 0x66);
        return this.addr();
    }
    covered!("cx06_3 euro qualifiers built");
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(10);
    l.set_stages(exe::crt_malloc(0x28));
    if season > l.year() {
        covered!("cx06_3 the 1998 edition");
        let groups_and_results: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROCHAMPQUAL_CREATE_GROUPS_AND_RESULTS); // EuroChampQual::create_groups_and_results
        groups_and_results(this.addr());
    }
    if (l.year() as i32) - (season as i32) > 2 {
        l.set_active(0);
    }
    this.addr()
}

/// Lists the entrants of the European Championship qualifiers: the European nations' national teams but the hosts.
///
/// `euro_champ_qual::GROUP_FORMAT` counts the European hosts of the finals: without the host table
/// (`G_HOST_COUNTRY`), the box at line 0x4c3; otherwise HostCountry::find_hosts (`exe::hostcountry_find_hosts`: the
/// competition id in `KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`, the edition `year()` + 2, the two host ids, 1) and
/// each host (up to the count, stopping at −1) whose nation's continent (`nation::CONTINENT`, its id) is
/// `KEY_CONTINENT_EUROPE` adds one. Below 0 or above 2: the box at line 0x4dd and 1. The entrant count (+0xf2) is 51 −
/// hosts; the old list (+0xee) is freed and a new one allocated (`exe::crt_malloc`; out of memory: the box at line 0x4ec
/// and 0); every European nation in table order (`G_NATION_TABLE`, `G_NATION_COUNT`), while the list has room, gives
/// its national team (`exe::database_get_national_team`) unless its id is one of the two hosts. A short list: the box at
/// line 0x506 and 0. The list is sorted by reputation (`exe::crt_qsort` with 0x4b07d0). Returns 1.
/// Original: `EuroChampQual::collect_entrants` (0x57f600, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_collect_entrants(this: usize) -> u32 {
    let l = League(this);
    write_u32(this + 0xf2, 0x33); // the entrant count
    write_u8(this + euro_champ_qual::GROUP_FORMAT, 0);
    let mut hosts = [-1i32; 2];
    let is_european = |nation: usize| {
        let continent = read_usize(nation + nation::CONTINENT);
        continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_EUROPE)
    };
    let nation_at = |id: usize| read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE));
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0x9c4674, 0x4c3); // euro_champ_qual.cpp
    } else {
        let (first, second) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) as usize,
            l.year().wrapping_add(2) as u16 as u32, &mut first[0], &mut second[0], 1) as i8;
        let mut k = 0i8;
        while k < n {
            let Some(&host) = hosts.get(k as usize) else { break };
            if host == -1 {
                break;
            }
            if is_european(nation_at(host as usize)) {
                covered!("cx06_3 european host");
                write_u8(this + euro_champ_qual::GROUP_FORMAT, read_u8(this + euro_champ_qual::GROUP_FORMAT).wrapping_add(1));
            }
            k += 1;
        }
    }
    let european_hosts = read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8;
    if !(0..=2).contains(&european_hosts) {
        show_error_at(0x9c4674, 0x4dd);
        write_u8(this + euro_champ_qual::GROUP_FORMAT, 1);
    }
    let count = 0x33 - read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8 as i32;
    write_i32(this + 0xf2, count);
    if read_usize(this + 0xee) != 0 {
        exe::crt_free(read_usize(this + 0xee));
    }
    let list = exe::crt_malloc((count as u32).wrapping_mul(4));
    write_usize(this + 0xee, list); // the entrants
    if list == 0 {
        show_error_at(0x9c4674, 0x4ec);
        return 0;
    }
    let mut added = 0i32;
    let mut k: i16 = 0;
    while (k as i32) < read_i32(G_NATION_COUNT) {
        let nation = nation_at(k as usize);
        if is_european(nation) && added < read_i32(this + 0xf2) {
            let team = exe::database_get_national_team(nation);
            let id = read_i32(nation);
            if id != hosts[0] && id != hosts[1] {
                write_usize(read_usize(this + 0xee) + 4 * added as usize, team);
                added += 1;
            }
        }
        k = k.wrapping_add(1);
    }
    if added < read_i32(this + 0xf2) {
        show_error_at(0x9c4674, 0x506);
        return 0;
    }
    covered!("cx06_3 qualifiers entrants");
    exe::crt_qsort(read_usize(this + 0xee), added as u32, 4, 0x4b07d0usize); // by reputation, descending
    1
}

/// Sets the predicted final positions of the qualifiers' entrants: without statistics, the box at line 0x6f3. The
/// entrants (+0xee, count +0xf2) sorted by reputation (`exe::crt_qsort` with 0x4b07d0); the national teams freed from
/// their European competitions (NationalTeams::release_competitions_of_continent
/// `exe::nationalteams_release_competitions_of_continent` on the continent `KEY_CONTINENT_EUROPE`, 0xc6-byte records of
/// `G_CONTINENT_TABLE`, with 1); then each entrant k, in order, its band (`euro_qual_predicted_band`, Comp_Stats
/// set_club_final_position `exe::comp_stats_set_club_final_position`) and the qualifiers as its nation's current
/// competition (NationalTeams::set_current_competition `exe::nationalteams_set_current_competition`).
/// Original: `EuroChampQual::set_predicted_positions` (0x5806d0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        show_error_at(0x9c4674, 0x6f3); // euro_champ_qual.cpp
        return;
    }
    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x4b07d0usize);
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add((read_u32(KEY_CONTINENT_EUROPE) as usize).wrapping_mul(0xc6)); // continent records
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 1);
    let mut k: i16 = 0;
    while (k as i32) < read_i32(this + 0xf2) {
        let team = Club(read_usize(read_usize(this + 0xee) + 4 * k as usize));
        if let Some(band) = euro_qual_predicted_band(read_u8(this + euro_champ_qual::GROUP_FORMAT), k) {
            covered!("cx06_3 qualifiers predicted band");
            exe::comp_stats_set_club_final_position(l.comp_stats(), team, band);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation(), l.competition());
        k = k.wrapping_add(1);
    }
}

/// Comparer for `construct_euro_champ_qual`: only the original runs (it allocates and loads); the plan: this returned,
/// the vtable, the record, the edition year (`euro_qual_edition`; any year when loading a saved game) and importance
/// 0xf (`compare_nations_tournament_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_euro_champ_qual(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EURO_CHAMP_QUAL);
    compare_nations_tournament_ctor("construct_euro_champ_qual", League(this), record, 0x96a0d4, euro_qual_edition((year as u16 as i16).wrapping_sub(1)), || o(this, year, record))
}

/// Comparer for `euro_champ_qual_collect_entrants`: only the original runs (it frees and allocates the list); the plan:
/// on success the count is 51 minus the hosts (0 to 2), every entrant a national team, sorted by reputation
/// (`club::REPUTATION`, descending).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_QUAL_COLLECT_ENTRANTS)(this);
    if r != 1 {
        return r;
    }
    let (list, count, hosts) = (read_usize(this + 0xee), read_i32(this + 0xf2), read_u8(this + euro_champ_qual::GROUP_FORMAT) as i32);
    let teams: Vec<Club> = (0..count.clamp(0, 64) as usize).map(|k| Club(read_usize(list + 4 * k))).collect();
    let sorted = teams.windows(2).all(|w| w[0].reputation() >= w[1].reputation());
    let ok = count == 0x33 - hosts && (0..=2).contains(&hosts) && teams.iter().all(|t| t.addr() != 0) && sorted;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_collect_entrants({:#x}): {} entrants, {} hosts, sorted {}", this, count, hosts, sorted));
    r
}

/// Comparer for `euro_champ_qual_set_predicted_positions`: only the original runs (the national teams' competitions
/// change); the plan: by reputation the entrants' final positions make the bands (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_set_predicted_positions(this: usize) {
    let (list, count) = (read_usize(this + 0xee), read_i32(this + 0xf2));
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&count) { (0..count as usize).map(|k| read_usize(list + 4 * k)).collect() } else { Vec::new() };
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EURO_CHAMP_QUAL_SET_PREDICTED_POSITIONS)(this);
    let format = read_u8(this + euro_champ_qual::GROUP_FORMAT);
    let bands: Vec<u32> = (0..clubs.len() as i16).filter_map(|k| euro_qual_predicted_band(format, k)).collect();
    if bands.len() == clubs.len() {
        predicted_by_reputation_check("euro_champ_qual_set_predicted_positions", this, &clubs, &bands);
    }
}

/// Builds the round table of a qualifiers' stage (`euro_qual_phase` with the round-table machine `build_phase_table`):
/// ten group matchdays for each group, one play-off round of two legs for 6 or 8 nations. With the play-off and a
/// format other than 1 or 2, the box at line 0x11b after the table (which is still returned).
/// Original: `EuroChampQual::create_schedule` (0x57d190, euro_champ_qual.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let plan = euro_qual_phase(this, stage);
    let unknown_format = plan.is_some_and(|p| p.league == 0) && !matches!(read_u8(this + euro_champ_qual::GROUP_FORMAT), 1 | 2);
    let table = build_phase_table(League(this), plan, rounds, extra, league, 0x9c4674);
    if table != 0 && unknown_format {
        show_error_at(0x9c4674, 0x11b); // euro_champ_qual.cpp
    }
    table
}

/// Creates the League_Stage of qualifying group `g` + 1 (stage `g`).
///
/// The round table of stage `g` (`comp_vtable::CREATE_SCHEDULE`: g, &rounds, &id, 0). Without a team list, the group's
/// size (`exe::eurochampqual_get_group_team_count`(g)) and a list of its drawn teams (`euro_champ_qual::DRAWN_TEAMS`,
/// slots 6 × (g + 1) on; `exe::crt_malloc`; out of memory: the box at line 0x3b4 and 0); with `list`, the round table
/// is freed (the stage gets none) and `count` teams of `list` are used. The stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the qualifiers, the teams, the
/// list, 2, `.competition()`, the round table, the rounds, the win, draw and category bytes, &tie-breaks, &promoted,
/// the year, g, the id, the importance, 4, 0, `.league_flags()`, −1, 0, 2) into `.stages()`[g]; none: the box at line
/// 0x3d0 and 0 (the list and the round table are not freed). Then the own list and the round table freed,
/// `.current_stage()` = g, the group draw news (`exe::league_send_group_draw_news` on the stage) and 1.
/// Original: `EuroChampQual::create_group_stage` (0x57ed10, euro_champ_qual.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_group_stage(this: usize, g: u32, list: usize, count: u32) -> u32 {
    let this = League(this);
    let l = this;
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let mut table = schedule(this.addr(), g, &mut rounds, &mut id, 0);
    let (teams, n) = if list == 0 {
        let n = exe::eurochampqual_get_group_team_count(this, g);
        let teams = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
        if teams == 0 {
            show_error_at(0x9c4674, 0x3b4); // euro_champ_qual.cpp
            return 0;
        }
        covered!("cx06_4 group drawn teams");
        let drawn = read_usize(this.addr() + euro_champ_qual::DRAWN_TEAMS);
        for k in 0..n.max(0) as usize {
            write_usize(teams + 4 * k, read_usize(drawn + 4 * (6 * (g as usize + 1) + k)));
        }
        (teams, n as u16 as u32)
    } else {
        if table != 0 {
            exe::crt_free(table);
        }
        table = 0;
        (list, count as u16 as u32)
    };
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), n, teams, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, l.year() as u16 as u32, g, id as u16 as u32,
            l.importance() as u32, 4, 0, l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * g as usize, stage);
    if read_usize(l.stages() + 4 * g as usize) == 0 {
        show_error_at(0x9c4674, 0x3d0);
        return 0;
    }
    if list == 0 {
        exe::crt_free(teams);
    }
    if table != 0 {
        exe::crt_free(table);
    }
    l.set_current_stage(g as usize);
    exe::league_send_group_draw_news(read_usize(l.stages() + 4 * g as usize));
    1
}

/// The qualifiers' day. Before the draw date (`oceania_nations_draw_date`: `.next_draw_day()` of `.year()` +
/// `.next_draw_year_offset()`; today's year unsigned, its day signed) nothing happens and 0 is returned (al; the
/// League day does not run).
///
/// On the draw date itself, while not `active` and with `full`: the entrants (EuroChampQual::collect_entrants 0x57f600),
/// the groups draw (`euro_champ_qual_draw_groups`), group 1's table (`init_group_one_table`; 0: the box at line 0x5a8 and
/// 0), its fixtures (`exe::league_create_fixtures`; 0: the box at line 0x5af and 0), its table sorted
/// (`exe::league_sort_table`(this, 0); 0: the box at line 0x5b6 and 0), the stages of groups 2 to 9
/// (`euro_champ_qual_create_group_stage`(g, 0, 6), g = 0..7), the drawn teams freed and cleared, the runners-up stage
/// (EuroChampQual::create_runners_up_stage 0x57f920), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`),
/// the entrants freed and cleared with their count, and `active` 1. On other days, when the competition has a fixture
/// on the date (`fix_man_comp_has_fixture_on_date` on a copy of today), the runners-up table
/// (EuroChampQual::update_runners_up_table 0x57fa10). Then the League day (`exe::league_update_for_date`(this, today,
/// `full`)), whose result it returns.
/// Original: `EuroChampQual::update_for_date` (0x57fd60, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let (draw_day, draw_year) = oceania_nations_draw_date(l);
    let (day, year) = (read_i16(today), read_u16(today + 2));
    if year < draw_year || (year == draw_year && day < draw_day as i16) {
        return 0;
    }
    if year == draw_year && day as u16 == draw_day && l.active() == 0 && full != 0 {
        covered!("cx06_4 qualifiers draw day");
        let collect_entrants: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_EUROCHAMPQUAL_COLLECT_ENTRANTS); // EuroChampQual::collect_entrants
        collect_entrants(this);
        euro_champ_qual_draw_groups(this);
        if init_group_one_table(this) == 0 {
            show_error_at(0x9c4674, 0x5a8); // euro_champ_qual.cpp
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0x9c4674, 0x5af);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(0x9c4674, 0x5b6);
            return 0;
        }
        for g in 0..8 {
            euro_champ_qual_create_group_stage(this, g, 0, 6);
        }
        exe::crt_free(read_usize(this + euro_champ_qual::DRAWN_TEAMS));
        write_usize(this + euro_champ_qual::DRAWN_TEAMS, 0);
        let create_runners_up_stage: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROCHAMPQUAL_CREATE_RUNNERS_UP_STAGE); // EuroChampQual::create_runners_up_stage
        create_runners_up_stage(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xee)); // the entrants
        write_usize(this + 0xee, 0);
        write_u32(this + 0xf2, 0);
        l.set_active(1);
    } else {
        let date = [read_u32(today), read_u32(today + 4)];
        if fix_man_comp_has_fixture_on_date(date.as_ptr() as usize, read_u32(l.competition().addr())) != 0 {
            covered!("cx06_4 qualifiers match day");
            let update_runners_up_table: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROCHAMPQUAL_UPDATE_RUNNERS_UP_TABLE); // EuroChampQual::update_runners_up_table
            update_runners_up_table(this);
        }
    }
    exe::league_update_for_date(this, today, full) as u32
}

/// Comparer for `euro_champ_qual_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes
/// written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EURO_CHAMP_QUAL_CREATE_SCHEDULE);
    compare_phase_table("euro_champ_qual_create_schedule", stage, euro_qual_phase(this, stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| euro_champ_qual_create_schedule(this, stage, r, e, l))
}

/// Comparer for `euro_champ_qual_create_group_stage`: it allocates the stage and sends the draw news (it cannot run
/// twice), so only the original runs; the plan: on success the stage at `.stages()`[g] with the group's teams (the
/// drawn ones or the list, as a set: the stage sorts its table), `.current_stage()` = g.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_group_stage(this: usize, g: u32, list: usize, count: u32) -> u32 {
    let l = League(this);
    let expected: Vec<usize> = if list == 0 {
        let drawn = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
        let n = exe::eurochampqual_get_group_team_count(this, g).clamp(0, 6) as usize;
        if drawn != 0 { (0..n).map(|k| read_usize(drawn + 4 * (6 * (g as usize + 1) + k))).collect() } else { Vec::new() }
    } else {
        (0..(count as u16 as i16).clamp(0, 8) as usize).map(|k| read_usize(list + 4 * k)).collect()
    };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(IDX_EURO_CHAMP_QUAL_CREATE_GROUP_STAGE)(this, g, list, count);
    if r != 1 {
        return r;
    }
    let stage = League(read_usize(l.stages() + 4 * g as usize));
    let mut got: Vec<usize> = (0..stage.club_count().clamp(0, 8) as usize).map(|k| read_usize(stage.table() + k * league_table_row::SIZE)).collect();
    let mut want = expected.clone();
    got.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(got == want && l.current_stage() == g as usize, || format!("euro_champ_qual_create_group_stage({:#x}, {}): teams {:x?} expected {:x?}", this, g, got, want));
    r
}

/// Comparer for `euro_champ_qual_update_for_date`: the day draws, allocates and updates the league (it cannot run
/// twice), so only the original runs; the plan: after the draw day, `active` 1, the drawn teams and the entrants
/// cleared and group 1's table built.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let draws = full != 0 && l.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(l);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_EURO_CHAMP_QUAL_UPDATE_FOR_DATE)(this, today, full);
    if draws {
        let ok = l.active() == 1 && read_usize(this + euro_champ_qual::DRAWN_TEAMS) == 0 && read_usize(this + 0xee) == 0 && l.table() != 0;
        SCOREBOARD.record(ok, || format!("euro_champ_qual_update_for_date({:#x}): the draw day left active {}, table {:#x}", this, l.active(), l.table()));
    }
    r
}

/// The qualifiers' edition: from `start`, up one year at a time while the remainder of (year − 1990) by 4 is not 0
/// (signed, as the original's `and 0x80000003` with its sign adjustment): the year of the next finals.
pub(crate) fn euro_qual_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7c6) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The predicted final position of the `k`-th entrant (by reputation) of the qualifiers, by the number of European
/// hosts (`format`): one host, 9 group winners (1), then the best runners-up and the play-off bands 10, 13 and 16 (3
/// each), then 19, 28 and 37 (9 each) and 46; two hosts, 1 (9), 10 (1), 11 (4), 15 (4), 19, 28, 37 (9 each) and 46. No
/// position for any other format.
pub(crate) fn euro_qual_predicted_band(format: u8, k: i16) -> Option<u32> {
    let bands: &[(i16, u32)] = match format {
        1 => &[(9, 1), (0xc, 0xa), (0xf, 0xd), (0x12, 0x10), (0x1b, 0x13), (0x24, 0x1c), (0x2d, 0x25)],
        2 => &[(9, 1), (0xa, 0xa), (0xe, 0xb), (0x12, 0xf), (0x1b, 0x13), (0x24, 0x1c), (0x2d, 0x25)],
        _ => return None,
    };
    Some(bands.iter().find(|b| k < b.0).map_or(0x2e, |b| b.1))
}

/// The round table of the qualifiers' stage `stage` (signed byte): up to the last group stage (+0xff) the group table
/// (a league, 10 rounds, the stage id 0x3e9 + stage, 0x28a bytes, box line 0xbb); the play-off stage (+0x101, unless
/// it is also the runners-up stage +0x100) the play-off round (not a league, 1 round, id 0, 0x68 bytes, box line 0xf5)
/// with the size of the format (`euro_champ_qual::GROUP_FORMAT` 1 or 2); any other stage none.
pub(crate) unsafe fn euro_qual_phase(this: usize, stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f <= read_u8(this + 0xff) as i8 { // the last group stage
        return Some(GoldcupStagePlan { league: 1, rounds: 10, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0x28a, row: 0xbb,
            ops: &EURO_QUAL_GROUP_OPS, ops2: &[], name: "cx06_4 qualifiers group rounds" });
    }
    if f == read_u8(this + 0x100) as i8 || f != read_u8(this + 0x101) as i8 { // the runners-up / play-off stages
        return None;
    }
    let ops2: &'static [GoldcupScheduleOp] = match read_u8(this + euro_champ_qual::GROUP_FORMAT) {
        1 => &EURO_QUAL_PLAYOFF_SIX,
        2 => &EURO_QUAL_PLAYOFF_EIGHT,
        _ => &[],
    };
    Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0xf5, ops: &EURO_QUAL_PLAYOFF_OPS, ops2,
        name: "cx06_4 qualifiers play-off round" })
}

/// The qualifiers' group round table: ten matchdays (`D`: day, month from 0 = January, years after the edition year,
/// weekday, code 1 or 2, value 0), each with its first match slot empty (`L`): 2 and 6 September, 7 and 11 October,
/// 24 and 28 March, 6 June, 2 and 6 September and 7 October.
pub(crate) const EURO_QUAL_GROUP_OPS: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 2, 8, 0, 5, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 6, 8, 0, 2, 2, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 7, 9, 0, 5, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0xb, 9, 0, 2, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 0x18, 2, 1, 5, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x1c, 2, 1, 2, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 6, 5, 1, 2, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 2, 8, 1, 5, 1, 0), Cqp12L(7, 0, -1, -1, -1, 0),
    Cqp12D(8, 6, 8, 1, 2, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0), Cqp12D(9, 7, 9, 1, 5, 1, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];

/// The qualifiers' play-off round: the draw on 11 October of the year after the edition year (`S`, weekday 2), the
/// two legs from 11 November (`J`, weekday 5, time 1), round code 0xa0, tie rules 3, category 10, two legs (+0x21 2),
/// +0x22 4 and the prize words cleared.
pub(crate) const EURO_QUAL_PLAYOFF_OPS: [GoldcupScheduleOp; 15] = [
    Cqp12S(0, 0xb, 9, 1, 2), Cqp12J(0, 0xb, 0xa, 1, 5, 1, 0), Cqp12W(0x7, 0xa0), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0),
    Cqp12B(0x17, 0xa), Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 2), Cqp12B(0x22, 4), Cqp12X(0x5c, 0),
    Cqp12X(0x60, 0), Cqp12X(0x64, 0),
];

/// The play-off size with one European host: 6 nations, 3 ties.
pub(crate) const EURO_QUAL_PLAYOFF_SIX: [GoldcupScheduleOp; 3] = [Cqp12W(0x18, 6), Cqp12W(0x1a, 3), Cqp12W(0x1c, 6)];

/// The play-off size with two European hosts: 8 nations, 4 ties.
pub(crate) const EURO_QUAL_PLAYOFF_EIGHT: [GoldcupScheduleOp; 3] = [Cqp12W(0x18, 8), Cqp12W(0x1a, 4), Cqp12W(0x1c, 8)];
