// ---------- rx09_16: the Asian World Cup qualifying first round draw (wc_asia_league.cpp) ----------

/// The Asian qualifying's exempt nation count (+0xee, signed byte; the hosts and the last winner when Asian).
pub(crate) const WC_ASIA_EXEMPT: usize = 0xee;
/// The Asian qualifying's entrant count (+0xf3, dword) of the list at `wc_turnover_consts::ENTRANT_LIST`.
pub(crate) const WC_ASIA_ENTRANT_COUNT: usize = 0xf3;
/// The Asian qualifying's first round draw (+0xf7, a pointer: ten groups of four national team slots).
pub(crate) const WC_ASIA_DRAW: usize = 0xf7;

/// The team count of an Asian first round group (the group −1 to 8; −1 is the league's own table), by the exempt count
/// (`WC_ASIA_EXEMPT`): none exempt, four in every group; one, three in group 1; two, three in groups 3 and 6; three,
/// three in groups 1, 3 and 5; otherwise four. Any count above 3 or negative shows the box at line 0x37f first.
/// Original: `WcAsiaLeague::get_first_round_group_size` (0x912db0, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_get_first_round_group_size(this: usize, group: u32) -> i16 {
    let g = group as u8;
    let three = match read_u8(this + WC_ASIA_EXEMPT) as i8 {
        0 => false,
        1 => g == 1,
        2 => g == 3 || g == 6, // rx09_16: two exempt, groups 3 and 6 of three
        3 => g == 1 || g == 3 || g == 5,
        _ => {
            show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x37f);
            false
        }
    };
    if three { 3 } else { 4 }
}

/// Comparer for `wc_asia_league_get_first_round_group_size`: the answers side by side (the box only on the original's
/// side for a bad exempt count).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_get_first_round_group_size(this: usize, group: u32) -> i16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(%%I0%%)(this, group);
    let e = read_u8(this + WC_ASIA_EXEMPT) as i8;
    if (0..=3).contains(&e) {
        let c = wc_asia_league_get_first_round_group_size(this, group);
        SCOREBOARD.record(r == c, || format!("wc_asia_league_get_first_round_group_size(exempt {}, group {}) original={} rust={}", e, group as u8 as i8, r, c));
    }
    r
}

/// The team count of Asian first round group g (−1 to 8), through the hooked routine.
pub(crate) unsafe fn wc_asia_group_size(this: usize, g: i8) -> i16 {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WCASIALEAGUE_GET_FIRST_ROUND_GROUP_SIZE);
    size(this, g as u8 as u32)
}

/// `WcAsiaLeague::get_first_round_group_size` (0x912db0, thiscall, ret 4), through its hook.
const FN_WCASIALEAGUE_GET_FIRST_ROUND_GROUP_SIZE: usize = 0x912db0;

/// Draws the ten Asian first round groups: the group order 0..9 (a local array) shuffled by 20 swaps of two randomly
/// drawn places (`exe::utils_random`(10) twice: the first place, then the second); the draw (`exe::crt_malloc` of 0xa0,
/// ten groups of four dwords, into `WC_ASIA_DRAW`; none: the box at line 0x340 and nothing more). Then up to four pots
/// while fewer than the entrant count (`WC_ASIA_ENTRANT_COUNT`, signed byte against the dword) are placed: for each
/// group in the shuffled order, while some remain, group g takes the next entrant (`wc_turnover_consts::ENTRANT_LIST`)
/// into its next slot if it has fewer than its size (`wc_asia_group_size`(g − 1)).
/// Original: `WcAsiaLeague::draw_first_round_groups` (0x912bd0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_draw_first_round_groups(this: usize) {
    let mut taken = [0u8; 10];
    let mut order: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 0..0x14 {
        let a = exe::utils_random(0xa) as u8 as i8 as isize as usize;
        let b = exe::utils_random(0xa) as u8 as i8 as isize as usize;
        order.swap(a, b); // rx09_16: the group order shuffled
    }
    let draw = exe::crt_malloc(0xa0);
    write_usize(this + WC_ASIA_DRAW, draw);
    if draw == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x340);
    }
    covered!("rx09_16 asian first round draw");
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let mut placed: i8 = 0;
    for _pot in 0..4 {
        if placed as i32 >= read_i32(this + WC_ASIA_ENTRANT_COUNT) {
            break;
        }
        for &g in &order {
            if placed as i32 >= read_i32(this + WC_ASIA_ENTRANT_COUNT) {
                break;
            }
            if g >= 0xa {
                continue;
            }
            let c = taken[g as usize];
            if (c as i8 as i16) < wc_asia_group_size(this, (g as i8).wrapping_sub(1)) {
                write_usize(draw + 4 * (c as i8 as usize + 4 * g as usize), read_usize(list + 4 * placed as usize));
                placed = placed.wrapping_add(1);
                taken[g as usize] = c.wrapping_add(1);
            }
        }
    }
}

/// The national teams of the Asian draw that the plan can check: each of the first entrants (up to the slots of the
/// ten groups) must be in exactly one used slot (group g, slot c below the group's size).
pub(crate) unsafe fn compare_wc_asia_draw(name: &str, this: usize) {
    let draw = read_usize(this + WC_ASIA_DRAW);
    if draw == 0 {
        return;
    }
    let sizes: Vec<usize> = (0..10).map(|g| wc_asia_group_size(this, g as i8 - 1).max(0) as usize).collect();
    let n = (read_i32(this + WC_ASIA_ENTRANT_COUNT).max(0) as usize).min(sizes.iter().sum());
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let slots: Vec<usize> = (0..10).flat_map(|g| (0..sizes[g]).map(move |c| (g, c))).map(|(g, c)| read_usize(draw + 4 * (c + 4 * g))).collect();
    let missing: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).filter(|t| slots.iter().filter(|s| *s == t).count() != 1).collect();
    SCOREBOARD.record(missing.is_empty(), || format!("{}({:#x}): {} entrants, not drawn once {:x?}", name, this, n, missing));
}

/// Comparer for `wc_asia_league_draw_first_round_groups`: only the original runs (it draws and allocates); every
/// entrant drawn once into a slot within its group's size (`compare_wc_asia_draw`).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_draw_first_round_groups(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    compare_wc_asia_draw("wc_asia_league_draw_first_round_groups", this);
}

/// Builds the table of the first Asian first round group (the league's own, group −1): `club_count` = its size
/// (`wc_asia_group_size`(−1)), the table (`exe::crt_malloc` of 0x3b a team; none: the box at line 0x30d and 0) and a row
/// for each of the group's drawn national teams (League::init_table_row `exe::league_init_table_row`(this, k, slot k of
/// the draw, 0, 0)); then the group's draw news (League::send_group_draw_news `exe::league_send_group_draw_news`).
/// Returns 1.
/// Original: `WcAsiaLeague::init_first_group_table` (0x912ad0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_init_first_group_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(wc_asia_group_size(this, -1));
    let t = exe::crt_malloc((l.club_count() as i32 * league_table_row::SIZE as i32) as u32);
    l.set_table(t);
    if t == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x30d);
        return 0;
    }
    covered!("rx09_16 asian first group table");
    let draw = read_usize(this + WC_ASIA_DRAW);
    let mut k: u8 = 0;
    while (k as i8 as i16) < l.club_count() {
        exe::league_init_table_row(l, k as u32, read_usize(draw + 4 * k as i8 as usize), 0, 0); // rx09_16: the k-th team of the first group
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(l);
    1
}

/// Comparer for `wc_asia_league_init_first_group_table`: only the original runs (it allocates the table and sends the
/// news); the club count, the result and the rows against the first group of the draw.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_init_first_group_table(this: usize) -> u32 {
    let n = wc_asia_group_size(this, -1).max(0) as usize;
    let draw = read_usize(this + WC_ASIA_DRAW);
    let plan: Vec<usize> = (0..n).map(|k| read_usize(draw + 4 * k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let l = League(this);
    let found: Vec<usize> = if l.table() != 0 { (0..n).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    SCOREBOARD.record(r == 1 && l.club_count() as usize == n && found == plan, || format!("wc_asia_league_init_first_group_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}
