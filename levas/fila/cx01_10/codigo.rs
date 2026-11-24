// ---------- cx01_10: the Brazilian Champions Cup: constructor, entrants and predicted positions (bra_champ_cup.cpp) ----------

/// The Champions Cup's vtable.
const BRA_CHAMP_CUP_VTABLE: usize = 0x967c44;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\cups\bra_champ_cup.cpp", for the error boxes.
const BRA_CHAMP_CUP_SOURCE: usize = 0x98a1e8;
/// `BraChampCup::set_up_rules` (0x426230, bra_champ_cup.cpp, thiscall, ret 0; 0 on failure).
const FN_BRA_CHAMP_CUP_SET_UP_RULES: usize = 0x426230;
/// `compare_clubs_by_reputation_desc_nulls_last` (0x4b07d0, comp_util.cpp, cdecl): the qsort order by reputation.
const FN_COMPARE_CLUBS_BY_REPUTATION_DESC_NULLS_LAST: usize = 0x4b07d0;

/// The nine places of the Champions Cup and the long names of the clubs named for each; the name tests run in this
/// order for every Brazilian club, a later match overwriting the place (Vasco and Vila Nova both go to place 0).
const BRA_CHAMP_CUP_NAMED_CLUBS: [(usize, usize); 10] = [
    (0x98a398, 0), // "Associação Desportiva Vasco da Gama"
    (0x98a380, 0), // "Vila Nova Futebol Clube"
    (0x98a364, 1), // "Clube Náutico Capibaribe"
    (0x98a344, 2), // "São Raimundo Esporte Clube (AM)"
    (0x98a328, 3), // "América Futebol Clube (MG)"
    (0x98a304, 4), // "Grêmio Foot-Ball Porto-Alegrense"
    (0x98a2e4, 5), // "Clube de Regatas do Flamengo"
    (0x98a2c8, 6), // "Clube Atlético Paranaense"
    (0x98a2a8, 7), // "Sport Club Corinthians Paulista"
    (0x98a294, 8), // "Esporte Clube Bahia"
];

/// The Brazilian state championships whose strongest club fills an empty place, with that place.
const BRA_CHAMP_CUP_STATE_PLACES: [(usize, usize); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 8),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 6),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 1),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 2),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 3),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 1),
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 4),
];

/// The predicted final positions of the nine entrants, in entrant order.
const BRA_CHAMP_CUP_PREDICTED: [u8; 9] = [1, 2, 3, 3, 5, 5, 7, 7, 9];

/// The place of a state championship id in `BRA_CHAMP_CUP_STATE_PLACES`.
unsafe fn bra_champ_cup_state_place(id: u32) -> Option<usize> {
    BRA_CHAMP_CUP_STATE_PLACES.iter().find(|&&(key, _)| read_u32(key) == id).map(|&(_, place)| place)
}

/// Builds the Brazilian Champions Cup: League::League (`exe::new_league`), the competition record, the vtable, the
/// year (word), 3 substitutions allowed, 7 substitutes named, discipline rule 6 and importance 12. A saved state
/// (`exe::league_load_from_temp_file` with 1) ends it. Otherwise the league flags 0x28, the main stage (0xff), the
/// current stage −1, three stages and their vector (`exe::crt_malloc` of 12, unchecked), the entrants
/// (`bra_champ_cup_select_entrants`), the statistics (operator new 0x5ce and `exe::new_comp_stats`; none: the error box
/// of line 0x53) and the rules (`BraChampCup::set_up_rules`; 0: the box of line 0x5a); with them, the competition is
/// not active yet and has no table nor clubs. Returns the object.
/// Original: `BraChampCup::BraChampCup` (0x4259a0, bra_champ_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_champ_cup(this: usize, year: u32, record: usize) -> usize {
    let cup = League(this);
    exe::new_league(this);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, BRA_CHAMP_CUP_VTABLE);
    cup.set_year(year as u16 as i16);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_rule(6);
    cup.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    covered!("cx01_10 new champions cup");
    cup.set_league_flags(0x28);
    cup.set_stage_index(stage_index::MAIN);
    write_i32(this + league::CURRENT_STAGE, -1);
    cup.set_stage_count(3);
    cup.set_stages(exe::crt_malloc(0xc));
    bra_champ_cup_select_entrants(this);
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    cup.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x53);
        return this;
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRA_CHAMP_CUP_SET_UP_RULES);
    if rules(this) == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x5a);
        return this;
    }
    cup.set_active(0);
    cup.set_table(0);
    cup.set_club_count(0);
    this
}

/// Picks the nine entrants of the Brazilian Champions Cup into a new entrant book.
///
/// First, in table order and up to 200, the clubs whose state championship (`club::RESERVE_DIVISION`) is one of the
/// twelve of `BRA_CHAMP_CUP_STATE_PLACES`, sorted by reputation (`exe::crt_qsort` with
/// `compare_clubs_by_reputation_desc_nulls_last`). The book: `ENTRANTS_ARE_QUALIFIERS` = 0xff, capacity and count 9,
/// nine 6-byte entries (`exe::crt_calloc`; none: the error box of line 0x3d7). Every Brazilian club (`club::NATION`
/// id = Brazil) whose long name is one of `BRA_CHAMP_CUP_NAMED_CLUBS` takes that place. If a place is still empty,
/// each listed club in order takes its state's place when that place is empty.
/// Original: `BraChampCup::select_entrants` (0x426d80, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_select_entrants(this: usize) {
    let cup = League(this);
    let mut listed = [0usize; 0xc8];
    let mut n = 0usize;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) && n < 0xc8 {
        let c = club_at(i as isize);
        let state = c.reserve_division();
        if !state.is_null() && bra_champ_cup_state_place(read_u32(state.addr())).is_some() {
            listed[n] = c.addr();
            n += 1;
        }
        i += 1;
    }
    exe::crt_qsort(listed.as_mut_ptr() as usize, n as u32, 4, FN_COMPARE_CLUBS_BY_REPUTATION_DESC_NULLS_LAST);
    cup.set_entrants_are_qualifiers(0xff);
    cup.set_entrant_capacity(9);
    cup.set_entrant_count(9);
    let book = exe::crt_calloc(6, 9);
    cup.set_entrant_book(book);
    if book == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x3d7);
        return;
    }
    let brazil = read_u32(KEY_NATION_BRAZIL);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        if !c.nation().is_null() && read_u32(c.nation().addr()) == brazil {
            let name = c_string_bytes(c.name_addr());
            for &(named, place) in &BRA_CHAMP_CUP_NAMED_CLUBS {
                if name == c_string_bytes(named) {
                    covered!("cx01_10 named entrant");
                    write_usize(book + 6 * place, c.addr());
                }
            }
        }
        i += 1;
    }
    if (0..9).all(|k| read_usize(book + 6 * k) != 0) {
        return;
    }
    for &c in &listed[..n] {
        if let Some(place) = bra_champ_cup_state_place(read_u32(Club(c).reserve_division().addr())) {
            if read_usize(book + 6 * place) == 0 {
                covered!("cx01_10 state entrant");
                write_usize(book + 6 * place, c);
            }
        }
    }
}

/// Stores the predicted final positions of the nine entrants (`BRA_CHAMP_CUP_PREDICTED`, in entrant order) in the
/// competition statistics (`exe::comp_stats_set_club_final_position`). Without statistics the error box of line 0x319;
/// the clubs are first copied into a list of nine (`exe::crt_malloc`; none: the box of line 0x321), freed at the end.
/// Original: `BraChampCup::set_predicted_positions` (0x426ab0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_predicted_positions(this: usize) {
    let cup = League(this);
    if cup.comp_stats() == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x319);
        return;
    }
    let list = exe::crt_malloc(0x24);
    if list == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x321);
        return;
    }
    covered!("cx01_10 predicted positions");
    for k in 0..9 {
        write_usize(list + 4 * k, read_usize(cup.entrant_book() + 6 * k));
    }
    for (k, &position) in BRA_CHAMP_CUP_PREDICTED.iter().enumerate() {
        exe::comp_stats_set_club_final_position(cup.comp_stats(), read_usize(list + 4 * k), position as u32);
    }
    exe::crt_free(list);
}

/// Comparer for `construct_bra_champ_cup`: only the original runs (it allocates the stages, the book and the
/// statistics); the return, the vtable, the year, the record and the bench and discipline values must be the ones the
/// copy writes.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_champ_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let cup = League(this);
    let ok = r == this && read_usize(this) == BRA_CHAMP_CUP_VTABLE && cup.year() == year as u16 as i16 && cup.competition().addr() == record
        && cup.substitutions_allowed() == 3 && cup.substitutes_named() == 7 && cup.discipline_rule() == 6 && cup.importance() == 0xc;
    SCOREBOARD.record(ok, || format!("construct_bra_champ_cup({:#x}, {}, {:#x}): returned {:#x}, vtable {:#x}, year {}, record {:#x}, bench {}/{}, discipline {}, importance {}",
        this, year as u16, record, r, read_usize(this), cup.year(), cup.competition().addr(), cup.substitutes_named(), cup.substitutions_allowed(), cup.discipline_rule(), cup.importance()));
    r
}

/// Comparer for `bra_champ_cup_select_entrants`: the original builds its book, then the copy builds another; the nine
/// entries and the book fields must match. The copy's book is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_select_entrants(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let cup = League(this);
    let book = cup.entrant_book();
    if book == 0 {
        return;
    }
    let fields = |c: League| (c.entrants_are_qualifiers(), c.entrant_capacity(), c.entrant_count());
    let original = (fields(cup), core::slice::from_raw_parts(book as *const u8, 6 * 9).to_vec());
    bra_champ_cup_select_entrants(this);
    let mine = cup.entrant_book();
    let copy = (fields(cup), if mine != 0 { core::slice::from_raw_parts(mine as *const u8, 6 * 9).to_vec() } else { Vec::new() });
    if mine != 0 {
        exe::crt_free(mine);
    }
    cup.set_entrant_book(book);
    SCOREBOARD.record(original == copy, || format!("bra_champ_cup_select_entrants({:#x}): original {:x?} rust {:x?}", this, original, copy));
}

/// Comparer for `bra_champ_cup_set_predicted_positions`: only the original runs (it may add the clubs to the
/// statistics); each entrant's final position must then be the copy's (`BRA_CHAMP_CUP_PREDICTED`).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let cup = League(this);
    if cup.comp_stats() == 0 || cup.entrant_book() == 0 {
        return;
    }
    let got: Vec<u8> = (0..9).map(|k| {
        let e = exe::comp_stats_find_club_entry(cup.comp_stats(), read_usize(cup.entrant_book() + 6 * k));
        if e != 0 { CompStatsClub(e).final_position() } else { 0 }
    }).collect();
    SCOREBOARD.record(got == BRA_CHAMP_CUP_PREDICTED, || format!("bra_champ_cup_set_predicted_positions({:#x}): {:?}", this, got));
}
