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
