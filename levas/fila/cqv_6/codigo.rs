// ---------- %%LEVA%%a leva: the Copa do Brasil: the constructor and the 71 entrants (bra_cup.cpp) ----------

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
unsafe fn bra_cup_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
unsafe fn bra_cup_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
unsafe fn bra_cup_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

const BRA_CUP_VTABLE: usize = 0x967cf8;

/// The body of the Copa do Brasil constructor (see `construct_bra_cup`).
unsafe fn construct_bra_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, BRA_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(1);
    cup.set_discipline_index(6);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_6 copa do brasil new");
    let select_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x427f00usize);
    select_entrants(this);
    let build_team_table: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x427c20usize);
    if build_team_table(this) == 0 {
        show_error_at(0x98a3bc, 0x4b);
        return this;
    }
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_prize_fund(0x7a120);
    bra_cup_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) == 0 {
        show_error_at(0x98a3bc, 0x59);
        return this;
    }
    if !bra_cup_ctor_create_stats(this) {
        show_error_at(0x98a3bc, 0x60);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the Copa do Brasil constructor writes on a new cup (offset, width, value).
const BRA_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x7a120), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x6), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Copa do Brasil constructor `BraCup::BraCup(year, record)`, run once at career creation. On the Cup::Cup base
/// (`exe::new_cup`): the competition record at +4, the vtable (0x967cf8), the year (`cup::YEAR`), `current_round()` =
/// 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 1 (clubs of one nation),
/// discipline rule 6 and `run_by_stages()` = 0. If the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this,
/// 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the 71 entrants (BraCup::select_entrants
/// 0x427f00, thiscall, ret 0, unchecked); the 71-team table (BraCup::build_team_table 0x427c20, thiscall, ret 0; the
/// error box of line 0x4b if it returns 0); 7 substitutes named, 3 substitutions allowed and the prize fund
/// (`cup::PRIZE_FUND`) = 500,000; the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree
/// (Cup::map_fixture_tree 0x518790, thiscall, ret 0; the error box of line 0x59 if it returns 0); the statistics at +8
/// (the error box of line 0x60 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0,
/// unchecked). Every failure returns this. [the bench, substitutions, discipline and type fields read from the
/// competitions reference, §2.1]
/// Original: `BraCup::BraCup` (0x427430, bra_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_bra_cup(this: usize, year: u32, record: usize) -> usize {
    construct_bra_cup_body(this, year, record) // cqv6: the Copa do Brasil
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`bra_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_bra_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = bra_cup_ctor_plan_error(this, r, year, record, BRA_CUP_VTABLE, &BRA_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_bra_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The 71 fixed Copa do Brasil names, in slot order (texts in the exe, read from there like the original): Vasco (0),
/// Remo, Paysandu, River, Sampaio Corrêa, Nacional (AM), ABC, América (RN), Ceará, Confiança, CRB, Sergipe, Comercial
/// Bandeirantes, Comercial, Juventus, Serra, Tocantinópolis, União, Camaçari, Fluminense de Feira, Juazeiro, Vitória,
/// Caxias, Internacional, Juventude, Pelotas, Anapolina, Anápolis, Atlético Goianiense, Goiânia, Atlético Mineiro,
/// Caldense, Uberlândia, Villa Nova, Paraná, Londrina, Malutrom, Coritiba, Central, Ferroviário, Porto, Santa Cruz,
/// América (RJ), Americano, Bangu, Botafogo, Avaí, Criciúma, Figueirense, Tubarão, Botafogo (SP), Palmeiras, Rio
/// Branco, Santos, Sport, Ponte Preta, São Paulo, Gama, São Raimundo, Fluminense, Goiás, Vila Nova, Náutico, Bahia,
/// Fortaleza, Joinville, América (MG), Grêmio, Flamengo, Atlético Paranaense and Corinthians (70).
const BRA_CUP_ENTRANT_NAMES: [usize; 71] = [0x98a398, 0x98aa20, 0x98aa0c, 0x98a9f4, 0x98a9d4, 0x98a9b8, 0x98a9a4, 0x98a988, 0x98a974, 0x98a954, 0x98a93c, 0x98a924, 0x98a8f8, 0x98a8e0, 0x98a8b4, 0x98a8a0, 0x98a880, 0x98a86c, 0x98a854, 0x98a830, 0x98a818, 0x98a800, 0x98a7dc, 0x98a7c0, 0x98a7a8, 0x98a790, 0x98a770, 0x98a758, 0x98a73c, 0x98a724, 0x98a70c, 0x98a6ec, 0x98a6d0, 0x98a6b4, 0x98a6a4, 0x98a68c, 0x98a67c, 0x98a664, 0x98a650, 0x98a634, 0x98a61c, 0x98a600, 0x98a5e4, 0x98a5c4, 0x98a5ac, 0x98a58c, 0x98a578, 0x98a560, 0x98a544, 0x98a52c, 0x98a510, 0x98a4f0, 0x98a4d4, 0x98a4bc, 0x98a4a4, 0x98a484, 0x98a46c, 0x98a450, 0x98a344, 0x98a434, 0x98a420, 0x98a380, 0x98a364, 0x98a294, 0x98a408, 0x98a3f0, 0x98a328, 0x98a304, 0x98a2e4, 0x98a2c8, 0x98a2a8];

/// Is the club's name (`club::NAME`) equal to the text `t` (the original's inline comparison, byte by byte up to the
/// terminating zero)?
unsafe fn bra_cup_club_name_is(c: usize, t: usize) -> bool {
    let mut i = 0usize;
    loop {
        let a = read_u8(c + club::NAME + i);
        if a != read_u8(t + i) {
            return false;
        }
        if a == 0 {
            return true;
        }
        i += 1;
    }
}

/// The twelve Copa do Brasil state leagues: the key, the control byte (the best-club mark in step 2, later the count
/// of the others), the slot group (0: slots 0x36..; 1: 0x3d..; 2: 0x42..) and the ceiling for the others (5 or 7). The
/// order is the original's test order; the indices are the original's stack bytes minus 0x13. [inference: the three
/// with ceiling 7 are the biggest state leagues]
const BRA_CUP_STATE_LEAGUES: [(usize, usize, usize, i8); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 0xc, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0xb, 0, 7),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 0x7, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0xd, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0x9, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0xf, 0, 7),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0xe, 1, 7),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0x4, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 0xa, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 0x5, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 0x6, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 0x8, 2, 5),
];

/// The club's state league (`club::RESERVE_DIVISION`; the record's +0 is its id), in the original's test order.
unsafe fn bra_cup_state_league_of(c: usize) -> Option<usize> {
    let id = read_usize(read_usize(c + club::RESERVE_DIVISION));
    BRA_CUP_STATE_LEAGUES.iter().position(|e| id == read_usize(e.0))
}

/// The body of `bra_cup_select_entrants` (also run by its comparer).
unsafe fn bra_cup_select_entrants_body(this: usize) {
    write_u8(this + 0x4b, 0xff);
    write_u16(this + cup::ENTRANT_CAPACITY, 0x47);
    write_u16(this + cup::ENTRANT_COUNT, 0x47);
    let book = exe::crt_calloc(6, 0x47);
    write_u32(this + cup::ENTRANT_BOOK, book as u32);
    if book == 0 {
        show_error_at(0x98a3bc, 0x1e0);
        return;
    }
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        let nation = read_usize(c + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_BRAZIL) {
            for (k, &t) in BRA_CUP_ENTRANT_NAMES.iter().enumerate() { // cqv6: the 71 names
                if bra_cup_club_name_is(c, t) {
                    write_u32(read_usize(this + cup::ENTRANT_BOOK) + k * 6, c as u32); // cqv6: the slot of the name
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let mut k: i16 = 0;
    while k < 0x47 {
        if read_usize(read_usize(this + cup::ENTRANT_BOOK) + k as usize * 6) == 0 {
            break;
        }
        k += 1;
    }
    if k == 0x47 {
        covered!("cqv_6 the 71 names");
        return;
    }
    covered!("cqv_6 fill from the state leagues");
    // the original's counters (stack bytes, zeroed at the start): [0] the next slot for the others, [1..3] the count of
    // each group (0x3d, 0x42, 0x36), [4..0xf] the mark and count of each state league, [0x10] those of the
    // KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION division
    let mut b = [0i8; 0x11];
    let group_count = |g: usize| [3usize, 1, 2][g];
    let group_first_slot = |g: usize| [0x36i32, 0x3d, 0x42][g];
    let mut list = [0usize; 200];
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if n >= 0xc8 {
            break;
        }
        let c = base + i as u16 as usize * club::SIZE;
        if read_usize(c + club::RESERVE_DIVISION) != 0 && bra_cup_state_league_of(c).is_some() {
            list[n as usize] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x4b07d0usize);
    for j in 0..n.max(0) as usize {
        let c = list[j];
        if let Some(e) = bra_cup_state_league_of(c) {
            let (_, mark, g, _) = BRA_CUP_STATE_LEAGUES[e];
            if b[mark] == 0 {
                let cg = group_count(g);
                write_u32(read_usize(this + cup::ENTRANT_BOOK) + (group_first_slot(g) + b[cg] as i32) as usize * 6, c as u32);
                b[mark] = 1;
                b[cg] = b[cg].wrapping_add(1);
                list[j] = 0;
            }
        }
        let c = list[j];
        let d = if c != 0 { read_usize(c + club::DIVISION) } else { 0 };
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION) && b[0x10] < 5 {
            write_u32(read_usize(this + cup::ENTRANT_BOOK) + (0x36 + b[3] as i32) as usize * 6, c as u32);
            b[3] = b[3].wrapping_add(1);
            b[0x10] = b[0x10].wrapping_add(1);
            list[j] = 0;
        }
    }
    for j in 0..n.max(0) as usize {
        let c = list[j];
        if c == 0 {
            continue;
        }
        if let Some(e) = bra_cup_state_league_of(c) {
            let (_, mark, _, ceiling) = BRA_CUP_STATE_LEAGUES[e];
            if b[mark] < ceiling {
                write_u32(read_usize(this + cup::ENTRANT_BOOK) + b[0] as i32 as usize * 6, c as u32);
                b[0] = b[0].wrapping_add(1);
                b[mark] = b[mark].wrapping_add(1);
            }
        }
    }
    if !(b[0] == 0x36 && b[3] == 7 && b[1] == 5 && b[2] == 5) {
        show_error_at(0x98a3bc, 0x3ce);
    }
}

/// The 71 Copa do Brasil entrants, chosen in the cup constructor (0x427430) at career creation. The entrant book
/// (`cup::ENTRANT_BOOK`, `exe::crt_calloc`(6, 0x47), cdecl: 0x47 six-byte records; out of memory, the error box of line
/// 0x1e0), `cup::ENTRANT_COUNT` = `cup::ENTRANT_CAPACITY` = 0x47 and +0x4b = 0xff.
/// 1) In club table order, each Brazilian club (`club::NATION` = [`KEY_NATION_BRAZIL`]) whose name (`club::NAME`) is
///    one of the 71 fixed ones (`BRA_CUP_ENTRANT_NAMES`) goes to that name's slot. With all 71 slots filled, done.
/// 2) Otherwise (an edited database): up to 200 clubs of the twelve state leagues (`club::RESERVE_DIVISION`,
///    `BRA_CUP_STATE_LEAGUES`) in a list sorted by reputation (`exe::crt_qsort` with 0x4b07d0, cdecl); the best of each
///    state league goes to its group (slots 0x36.., 0x3d.. or 0x42..), and up to 5 from the
///    [`KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION`] division (`club::DIVISION`) to group 0x36; then the others,
///    up to 5 per state league (7 in the three biggest [inference]), from slot 0 on. Without 54 + 7 + 5 + 5, the error
///    box of line 0x3ce.
/// Original: `BraCup::select_entrants` (0x427f00, bra_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_cup_select_entrants(this: usize) {
    bra_cup_select_entrants_body(this)
}

/// Side by side from the same state (`cup::ENTRANT_COUNT`/`cup::ENTRANT_CAPACITY`, +0x4b, [0xb67a34]; it draws
/// nothing): the original leaves its book in `cup::ENTRANT_BOOK`, the copy builds its own, compared over the 0x47
/// six-byte records and freed.
unsafe extern "thiscall" fn compare_bra_cup_select_entrants(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let before = read_usize(this + cup::ENTRANT_BOOK);
    let book_original = core::cell::Cell::new(0usize);
    let book_rust = core::cell::Cell::new(0usize);
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(this + cup::ENTRANT_COUNT, 4usize), (this + 0x4b, 1), (0xb67a34, 4)], || {
        o(this);
        book_original.set(read_usize(this + cup::ENTRANT_BOOK));
        write_u32(this + cup::ENTRANT_BOOK, before as u32);
    }, || {
        bra_cup_select_entrants_body(this);
        book_rust.set(read_usize(this + cup::ENTRANT_BOOK));
        write_u32(this + cup::ENTRANT_BOOK, book_original.get() as u32);
    });
    let (lo, lr) = (book_original.get(), book_rust.get());
    let book = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x47 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x47 * 6);
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && book, || format!("bra_cup_select_entrants({:#x}) memory {} book {}", this, same, book));
}
