// ---------- sbD_6: the Brazilian Cup and Champions Cup entrants (bra_nat_first.cpp) ----------

/// The twelve state leagues in the order the Brazilian Cup reads them, with how many clubs each may send besides its
/// champion: São Paulo, Rio, Minas Gerais, Paraná, Rio Grande do Sul, Goiás, Pernambuco and Bahia 5; Northeast 7;
/// Santa Catarina 5; Centre-West and North 7.
pub(crate) const BRAZILIAN_CUP_STATE_LEAGUES: [(usize, u8); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 5),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 5),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 7),
];

/// The last entrant slot of the Brazilian Cup: the slots are filled from 70 down to 0.
const BRAZILIAN_CUP_LAST_SLOT: i16 = 0x46;

/// Writes `club` into entrant slot `slot` (6-byte entries) of a competition's entrant book; a negative slot writes
/// before the book, as the original's arithmetic does.
unsafe fn write_entrant_slot(comp: League, slot: i16, club: u32) {
    write_u32(comp.entrant_book().wrapping_add((slot as isize as usize).wrapping_mul(6)), club);
}

/// Fills the Brazilian Cup entrant book at the Brazilian turnover (`this` is not read). Left out: the champion and the
/// runner-up of Series A, the Brazilian Cup winner and the Champions Cup winner (`exe::comp_util_get_latest_placed_club`
/// of their competition records). The slots go from 70 down to 0, each step one slot lower:
/// 1. each state league (`BRAZILIAN_CUP_STATE_LEAGUES` order) sends the first club of its table by position
///    (`exe::league_get_table_row`) that is not left out; a league with none still uses up its slot;
/// 2. Series A sends its clubs from the third row of its table on (`league::TABLE`, rows of 0x3b bytes), skipping the
///    clubs left out and those already in a higher slot;
/// 3. then, row by row from the second to the tenth, each state league in turn sends the club of that row of its table,
///    if it has not yet sent its quota, the club is not left out nor already in.
/// It stops when the slots run out. Defect of the original: step 3 starts without checking the slot, so it would write
/// one club before the book if steps 1 and 2 had used up all the slots (not with the real leagues).
/// Original: `bra_nat_first_fill_brazilian_cup_entrants` (0x42b760, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_fill_brazilian_cup_entrants(_this: usize) {
    let cup = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CUP));
    let serie_a = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION));
    let leagues: [League; 12] = core::array::from_fn(|k| League(comp_by_global(BRAZILIAN_CUP_STATE_LEAGUES[k].0)));
    let champions_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP).wrapping_mul(0x6b));
    let left_out = [
        exe::comp_util_get_latest_placed_club(serie_a.competition(), 0) as u32,
        exe::comp_util_get_latest_placed_club(serie_a.competition(), 1) as u32,
        exe::comp_util_get_latest_placed_club(cup.competition(), 0) as u32,
        exe::comp_util_get_latest_placed_club(champions_cup, 0) as u32,
    ];
    let already_in = |club: u32, slot: i16| (slot + 1..=BRAZILIAN_CUP_LAST_SLOT).any(|s| read_u32(cup.entrant_book() + s as usize * 6) == club);
    let table_club = |league: League, row: i8| read_u32(league.table().wrapping_add((row as isize as usize).wrapping_mul(0x3b)));
    let mut slot = BRAZILIAN_CUP_LAST_SLOT;
    covered!("sbD_6 brazilian cup entrants");
    for &league in &leagues {
        if slot < 0 {
            break;
        }
        let champion = (0..league.club_count()).map(|r| read_u32(exe::league_get_table_row(league, r as i32))).find(|c| !left_out.contains(c));
        if let Some(club) = champion {
            write_entrant_slot(cup, slot, club);
        }
        slot -= 1;
    }
    let mut row: i8 = 2;
    if serie_a.club_count() > 2 {
        while slot >= 0 {
            let club = table_club(serie_a, row);
            if !already_in(club, slot) && !left_out.contains(&club) {
                write_entrant_slot(cup, slot, club);
                slot -= 1;
            }
            row = row.wrapping_add(1);
            if row as i16 >= serie_a.club_count() {
                break;
            }
        }
    }
    let mut sent = [0u8; 12];
    for row in 1..10i8 {
        for (k, &league) in leagues.iter().enumerate() {
            if sent[k] as i8 >= BRAZILIAN_CUP_STATE_LEAGUES[k].1 as i8 || row as i16 >= league.club_count() {
                continue;
            }
            let club = table_club(league, row);
            if left_out.contains(&club) || already_in(club, slot) {
                continue;
            }
            write_entrant_slot(cup, slot, club);
            slot -= 1;
            if slot < 0 {
                return;
            }
            sent[k] += 1;
        }
        if slot < 0 {
            return;
        }
    }
}

/// BraChampCup's season set-up (bra_champ_cup.cpp, thiscall, ret 0): virtual `comp_vtable::SET_PREDICTED_POSITIONS`
/// (rank), the table, the fixtures and the order (each with its error box), then `league::ACTIVE` = 1.
pub(crate) const FN_BRA_CHAMP_CUP_SET_UP: usize = 0x427320;

/// The twelve state leagues in the order the Champions Cup reads their champions: São Paulo, Rio, Minas Gerais, Paraná,
/// Centre-West, Pernambuco, Bahia, Santa Catarina, Goiás, North, Northeast and Rio Grande do Sul.
pub(crate) const CHAMPIONS_CUP_STATE_LEAGUES: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP,
    KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS,
    KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS,
    KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP,
];

/// The champion of a state league and his points per game (`f32`, as the original stores them): the table row whose
/// club has the best final position (`comp_stats_club::CURRENT_POSITION` of its statistics entry,
/// `exe::comp_stats_find_club_entry` on the league's statistics) below 100, the first of equals. With no such row the
/// original leaves both unset (stack garbage); the copy gives (0, 0.0).
unsafe fn state_league_champion(league: League) -> (u32, f32) {
    let (mut best, mut champion) = (100u16, (0u32, 0f32));
    for r in 0..league.club_count().max(0) as usize {
        let row = league.table() + r * 0x3b;
        let entry = exe::comp_stats_find_club_entry(league.comp_stats(), read_usize(row));
        if entry != 0 && (read_u8(entry + comp_stats_club::CURRENT_POSITION) as u16) < best {
            best = read_u8(entry + comp_stats_club::CURRENT_POSITION) as u16;
            let points = read_i16(row + league_table_row::POINTS) as f64;
            let played = read_i8(row + league_table_row::PLAYED) as f64;
            champion = (read_u32(row), (points / played) as f32);
        }
    }
    champion
}

/// The nine Champions Cup entrants as (slot, club), in the order the original writes them, or None while a state
/// league is not complete (virtual `comp_vtable::CHECK_COMPLETE`(1, 0)) or the cup is already active
/// (`league::ACTIVE`). Slots 3 to 7: the champions of Rio, São Paulo, Minas Gerais, Rio Grande do Sul and Paraná;
/// slot 0: the better of the North and Northeast champions by points per game (the Northeast on a tie) if strictly
/// better than the Centre-West champion, otherwise the Centre-West one; slot 1: the Goiás champion if strictly better
/// than the Santa Catarina one, otherwise the latter; slots 8 and 2: the Bahia and Pernambuco champions, the strictly
/// better of the two in slot 8.
pub(crate) unsafe fn champions_cup_entrants() -> Option<[(usize, u32); 9]> {
    let leagues: [League; 12] = core::array::from_fn(|k| League(comp_by_global(CHAMPIONS_CUP_STATE_LEAGUES[k])));
    for league in leagues {
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(league.addr()) + comp_vtable::CHECK_COMPLETE));
        if complete(league.addr(), 1, 0) == 0 {
            return None;
        }
    }
    if League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP)).active() != 0 {
        return None;
    }
    let c: [(u32, f32); 12] = core::array::from_fn(|k| state_league_champion(leagues[k]));
    let (sao_paulo, rio, minas, parana, centre_west, pernambuco, bahia, santa_catarina, goias, north, northeast, gaucho) =
        (c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10], c[11]);
    let north_best = if north.1 > northeast.1 { north } else { northeast };
    let first = if north_best.1 > centre_west.1 { north_best.0 } else { centre_west.0 };
    let second = if goias.1 > santa_catarina.1 { goias.0 } else { santa_catarina.0 };
    let (better, other) = if bahia.1 > pernambuco.1 { (bahia.0, pernambuco.0) } else { (pernambuco.0, bahia.0) };
    Some([(3, rio.0), (4, sao_paulo.0), (5, minas.0), (6, gaucho.0), (7, parana.0), (0, first), (1, second), (8, better), (2, other)])
}

/// Puts the state champions into the Champions Cup entrant book once all twelve state leagues are complete
/// (`champions_cup_entrants`), then sets the cup up (`FN_BRA_CHAMP_CUP_SET_UP`: rank, table, fixtures; active = 1).
/// Defect of the original: a state league with no club of final position below 100 leaves its champion and ratio as
/// stack garbage.
/// Original: `bra_nat_first_fill_champions_cup_entrants` (0x42bba0, bra_nat_first.cpp, cdecl).
pub(crate) unsafe extern "C" fn bra_nat_first_fill_champions_cup_entrants() {
    let Some(entrants) = champions_cup_entrants() else {
        return;
    };
    covered!("sbD_6 champions cup entrants");
    let cup = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP));
    for (slot, club) in entrants {
        write_u32(cup.entrant_book() + slot * 6, club);
    }
    let set_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRA_CHAMP_CUP_SET_UP);
    set_up(cup.addr());
}

/// Comparer for `bra_nat_first_fill_brazilian_cup_entrants`: side by side over the 71 entrant slots of the Brazilian
/// Cup (the routine only writes there).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_first_fill_brazilian_cup_entrants(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let book = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CUP)).entrant_book();
    let reg = [(book, (BRAZILIAN_CUP_LAST_SLOT as usize + 1) * 6)];
    let (_, _, same, a, b) = crate::comparison::side_by_side(&reg, || o(this), || bra_nat_first_fill_brazilian_cup_entrants(this));
    SCOREBOARD.record(same, || format!("bra_nat_first_fill_brazilian_cup_entrants: entrant slots {:?} x {:?}", a, b));
}

/// Comparer for `bra_nat_first_fill_champions_cup_entrants`: only the original runs (it sets the cup up); the copy's
/// entrants, worked out before, must be the nine slots the original wrote (nothing when the copy expects nothing).
pub(crate) unsafe extern "C" fn compare_bra_nat_first_fill_champions_cup_entrants() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let plan = champions_cup_entrants();
    let book = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP)).entrant_book();
    let before: [u32; 9] = core::array::from_fn(|s| if book == 0 { 0 } else { read_u32(book + s * 6) });
    o();
    let after: [u32; 9] = core::array::from_fn(|s| if book == 0 { 0 } else { read_u32(book + s * 6) });
    let expected = match plan {
        Some(e) => {
            let mut v = before;
            for (s, c) in e {
                v[s] = c;
            }
            v
        }
        None => before,
    };
    SCOREBOARD.record(after == expected, || format!("bra_nat_first_fill_champions_cup_entrants: original {:x?} copy {:x?}", after, expected));
}
