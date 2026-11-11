use super::*;

/// Club (non-null and without a place, +0x1db = -1) enters the Copa Libertadores at stage `fase`; returns whether it
/// entered.
pub(crate) unsafe fn enter_libertadores(c: Club, stage: u8) -> bool {
    if c.is_null() || c.continental_comp_id() != u32::MAX {
        return false;
    }
    c.set_continental_comp_id(read_u32(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES));
    c.set_euro_seeding(stage);
    true
}

/// the Libertadores rule: the last winner enters round 1 and the others round 2 [inference: groups and preliminary];
/// the 11 nations of the seeding table (with Brazil, Mexico and the South American ones), each with its quota; Brazil,
/// up to 2002, with Palmeiras and Cruzeiro fixed, from 2003 with the Copa do Brasil winner (or runner-up) and the Copa
/// dos Campeões winner; and always with the Brasileirão champion and runner-up; each nation fills its quota with the
/// top clubs of the table still without a place (no second try: an already qualified club makes the place be lost)
/// Original: `ConmebolSeeding::assign_libertadores_places` (0x4c6430, conmebol_seeding.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn assign_libertadores_places(this: usize, clubs: usize, count: u32) {
    let row = |key: usize| (read_u32(G_CLUB_COMP_TABLE) as usize).wrapping_add(read_u32(key).wrapping_mul(0x6b) as usize);
    let h = Club(exe::comp_util_get_latest_placed_club(row(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES), 0));
    if !h.is_null() && h.continental_comp_id() == u32::MAX {
        h.set_euro_seeding(1);
        h.set_continental_comp_id(read_u32(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES));
    } else {
        show_error_at(0x9a7ed8, 0x114);
    }
    for k in 0..0xbusize {
        let e = this + k * 10;
        let mut n = 0u8;
        if read_u32(e) == read_u32(KEY_NATION_BRAZIL) {
            if read_u16(G_CURRENT_DATE_YEAR) > 0x7d2 {
                let mut c = exe::comp_util_get_latest_placed_club(row(KEY_CLUB_COMP_BRAZILIAN_CUP), 0);
                if c == 0 || read_u32(c + club::CONTINENTAL_COMP_ID) != u32::MAX {
                    c = exe::comp_util_get_latest_placed_club(row(KEY_CLUB_COMP_BRAZILIAN_CUP), 1);
                }
                if enter_libertadores(Club(c), 2) {
                    n = 1;
                }
                let id = read_u32(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID);
                if id != u32::MAX && enter_libertadores(contract_manager_club_by_id(id), 2) {
                    n = n.wrapping_add(1);
                }
            } else {
                if enter_libertadores(contract_manager_club_by_id(read_u32(G_KEY_CLUB_SOCIEDADE_ESPORTIVA_PALMEIRAS)), 2) {
                    covered!("cqj_1 brazil fixed");
                    n = 1; // cqj1: o Palmeiras
                }
                if enter_libertadores(contract_manager_club_by_id(read_u32(G_KEY_CLUB_CRUZEIRO_ESPORTE_CLUBE)), 2) {
                    n = n.wrapping_add(1);
                }
            }
            for lugar in [0u32, 1] {
                if enter_libertadores(Club(exe::comp_util_get_latest_placed_club(row(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION), lugar)), 2) {
                    n = n.wrapping_add(1);
                }
            }
        }
        while (n as i8) < read_i8(e + 8) {
            let c = exe::conmebolseeding_find_nth_unqualified_club_of_nation(this, clubs, count, read_u32(e), 1, 1, 0);
            if c != 0 && read_u32(c + club::CONTINENTAL_COMP_ID) == u32::MAX {
                covered!("cqj_1 nation quota");
                enter_libertadores(Club(c), 2); // cqj1: the quota slot
            }
            n = n.wrapping_add(1);
        }
    }
}

/// Finds the nth club of a nation in the South American club table, optionally skipping clubs already qualified and
/// clubs with a human manager.
///
/// `ConmebolSeeding::find_nth_unqualified_club_of_nation`; this is not read; the same code as UEFA's `uefa_find_nth_unplaced_club_of_nation`. In the
/// South American club table (12-byte records sorted by nation: +0 the club, +4 the nation id), a binary search
/// (bsearch `exe::crt_bsearch` with 0x905ce0; the key holds only the nation and +8 = 0xff) finds a club of the nation; it steps
/// back to the first club of the nation, then walks to the `n`th (char, 1 = the first) that counts, skipping, on
/// request, those that already have a place (+0x1db ≠ −1) and those with a human manager (`exe::human_manager_is_human_club`(club, 0, 1, 0, 0)).
/// The table is sorted by nation and, within it, by division, position and reputation. The last record of the table is
/// never examined by the loop (if the loop reaches it with the count at zero, it is chosen without the tests). Returns
/// the club if it belongs to the nation, otherwise 0.
/// Original: `ConmebolSeeding::find_nth_unqualified_club_of_nation` (0x4c6730, conmebol_seeding.cpp, thiscall, ret 0x18 (this não lido)).
pub(crate) unsafe extern "thiscall" fn find_nth_unqualified_club_of_nation(_this: usize, clubs: usize, count: u32, nation: u32, n: u32, without_slot: u32, without_human: u32) -> usize {
    let k = count as u16 as i16 as i32;
    let last = clubs.wrapping_add((k * 12 - 12) as usize);
    let mut key = [0u8; 12];
    key[4..8].copy_from_slice(&nation.to_le_bytes());
    key[8] = 0xff;
    let mut p = exe::crt_bsearch((key.as_ptr()) as usize, clubs, (k) as u32, 0xc, 0x905ce0);
    if p == 0 {
        return 0;
    }
    while p > clubs && read_u32(p - 8) == nation {
        p -= 12;
    }
    let mut foul = (n as u8).wrapping_sub(1);
    let mut found = false;
    if p < last {
        loop {
            let c = Club(read_u32(p) as usize);
            if without_slot != 0 && !c.is_null() && c.continental_comp_id() != u32::MAX {
                covered!("cqj_1 already with a place");
                p += 12; // cqj1: skips the club that already has a slot
            } else if without_human != 0 && !c.is_null() && exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 {
                p += 12;
            } else {
                if foul == 0 {
                    found = true;
                    break;
                }
                p += 12;
                foul = foul.wrapping_sub(1);
            }
            if p >= last {
                break;
            }
        }
    }
    if !found && foul != 0 {
        return 0;
    }
    if read_u32(p + 4) == nation {
        return read_u32(p) as usize;
    }
    0
}

/// Os 20 clubes convidados da Copa Mercosul (as chaves fixas: River Plate, Boca Juniors, Independiente, San Lorenzo,
/// Vélez Sarsfield, Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Rosario Central, Vasco, Colo-Colo,
/// Universidad de Chile, Universidad Católica, Peñarol, Nacional, Olimpia, Cerro Porteño e Grêmio), na ordem da original.
pub(crate) const MERCOSUR_INVITED_CLUBS: [usize; 20] = [
    0x9d00ac, 0x9d00d0, 0x9d00b4, 0x9d00d8, 0x9d00b0, G_KEY_CLUB_CRUZEIRO_ESPORTE_CLUBE, G_KEY_CLUB_SAO_PAULO_FUTEBOL_CLUBE, G_KEY_CLUB_SOCIEDADE_ESPORTIVA_PALMEIRAS, 0x9d0278, 0x9d0358,
    0x9d00e0, 0x9d0354, 0x9d0280, 0x9d0284, 0x9d0288, 0x9d0298, 0x9d028c, 0x9d0290, 0x9d0294, 0x9d027c,
];

/// the Copa Mercosur rule: the 20 fixed guests (River Plate, Boca Juniors, Independiente, San Lorenzo, Vélez Sarsfield,
/// Rosario Central; Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Vasco, Grêmio; Colo-Colo, Universidad de
/// Chile, Universidad Católica; Peñarol, Nacional; Olimpia, Cerro Porteño)
/// Original: `ConmebolSeeding::is_mercosur_invited_club` (0x4c6890, conmebol_seeding.cpp, thiscall, ret 4 (this não lido)).
pub(crate) unsafe extern "thiscall" fn is_mercosur_invited_club(_this: usize, club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    let id = read_u32(club);
    for &chave in MERCOSUR_INVITED_CLUBS.iter() {
        if id == read_u32(chave) {
            return 1; // cqj1: convidado
        }
    }
    0
}

/// Builds the CONMEBOL club table: South American and Mexican clubs (national teams excluded), each stripped of its
/// continental place, sorted by nation.
///
/// `collect_conmebol_clubs` (stdcall, ret 4). It allocates 12 bytes per club in the game (`exe::crt_malloc`; out of memory, the
/// box for line 0xbb and returns 0) and stores the pointer in `*out`. For each club with a nation that is not a
/// national team (`exe::database_is_national_team`) and is from Mexico ([0x9cf3d0]) or from a South American nation (continent +0x71 equal to
/// the id of [0x9cfa1c]): it removes the continental place (+0x1db = −1) and writes the record (+0 the club, +4 the
/// nation id, +8 the reputation +0x80 / 500, +9 the +0x69 of the division +0x5b, +0xb the +0x5f; without a division −1
/// and 0x7f; with a division belonging to a competition without +0x11c, the division and the club's +0x5f are zeroed).
/// Within a nation the order is by division, position and reputation [inference: the division +0x69 and club +0x5f
/// fields]. It sorts by nation (qsort `exe::crt_qsort` with 0x905ce0). Returns the number of clubs (only the low 16 bits
/// count).
/// Original: `collect_conmebol_clubs` (0x4c6280, conmebol_seeding.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn collect_conmebol_clubs(out: *mut usize) -> u32 {
    let t = exe::crt_malloc((read_u32(G_CLUB_COUNT) as i32).wrapping_mul(12) as u32);
    *out = t;
    if t == 0 {
        show_error_at(0x9a7ed8, 0xbb);
        return 0;
    }
    let mut k = 0u32;
    let mut c = read_u32(G_CLUB_TABLE) as usize;
    let mut i = 0u32;
    if read_u32(G_CLUB_COUNT) as i32 > 0 {
        loop {
            let nation = Nation(read_u32(c + club::NATION) as usize);
            if !nation.is_null() && exe::database_is_national_team(c) == 0 {
                let mexico = read_u32(nation.addr()) == read_u32(KEY_NATION_MEXICO); // cqj2: o México entra
                let count = if mexico { 0 } else { (nation.continent() as u32) as usize };
                if mexico || (count != 0 && read_u32(count) == read_u32(KEY_CONTINENT_SOUTH_AMERICA)) {
                    if mexico {
                        covered!("cqj_2 mexican club");
                    }
                    covered!("cqj_2 removes the place");
                    write_u32(c + club::CONTINENTAL_COMP_ID, u32::MAX); // cqj2: no continental place
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    write_u32(r, c as u32);
                    write_u32(r + 4, read_u32(nation.addr()));
                    write_u8(r + 8, (read_i16(c + club::REPUTATION) as i32 / 500) as u8);
                    let division = Competition(read_u32(c + club::LAST_DIVISION) as usize);
                    if !division.is_null() {
                        covered!("cqj_2 division level");
                        write_u16(r + 9, division.reputation() as u16); // cqj2: the division level
                        write_u8(r + european_club_entry::LAST_POSITION, read_u8(c + club::LAST_POSITION));
                        let comp = Nation(read_u32(read_u32(c + club::LAST_DIVISION) as usize + competition::NATION) as usize);
                        if comp.is_null() || comp.league_selected() == 0 {
                            write_u32(c + club::LAST_DIVISION, 0);
                            write_u8(c + club::LAST_POSITION, 0);
                        }
                    } else {
                        write_u16(r + 9, 0xffff);
                        write_u8(r + european_club_entry::LAST_POSITION, 0x7f);
                    }
                    k += 1;
                }
            }
            c += club::SIZE;
            i += 1;
            if (i as u16 as i16 as i32) >= read_u32(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    exe::crt_qsort(t, (k as u16 as i16 as i32) as u32, 0xc, 0x905ce0);
    // simplification: the original returns ax with the high bytes of eax left over from qsort (the caller reads ax); the copy, zero
    k as u16 as u32
}

/// Regions compared: the whole club table, [0xb67a34] and the club competition history table.
pub(crate) unsafe fn club_seeding_regions() -> Vec<(usize, usize)> {
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    vec![(read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)]
}

/// The Libertadores calendar: every new game year it builds the CONMEBOL club table and gives out the places.
///
/// `ConmebolSeeding::update_for_new_season`, called by the Libertadores and by the Mercosur. Without `skip` and with
/// the game year ([0xae2c92]) different from +0x6f: it stores the year, builds the club table (`collect_conmebol_clubs`; if empty, the
/// box for line 0x9b and it returns without freeing), gives the Libertadores places (`assign_libertadores_places`) and frees the table.
/// Original: `ConmebolSeeding::update_for_new_season` (0x4c61b0, conmebol_seeding.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_for_new_season(this: usize, skip: u32) {
    if skip != 0 {
        return;
    }
    let year = read_u16(G_CURRENT_DATE_YEAR);
    if year == read_u16(this + conmebol_seeding::LAST_SEASON_YEAR) {
        return;
    }
    covered!("cqj_2 year rollover");
    write_u16(this + conmebol_seeding::LAST_SEASON_YEAR, year);
    let mut t = 0usize;
    let n = exe::collect_conmebol_clubs(&mut t) as u32;
    if n as u16 == 0 {
        show_error_at(0x9a7ed8, 0x9b);
        return;
    }
    exe::conmebolseeding_assign_libertadores_places(this, t, n);
    exe::crt_free(t);
}

/// stores the Copa dos Campeões winner: from 2003 on it gets a Libertadores place
/// Original: `ConmebolSeeding::set_bra_champ_cup_winner` (0x4c6960, conmebol_seeding.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_bra_champ_cup_winner(this: usize, club: usize) {
    write_u32(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID, read_u32(club));
}

/// writes CM3_TEMP\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0, +0x6f (u16) and
/// +0x71 (u32); on error the "Error" box without closing the file
/// Original: `ConmebolSeeding::save_to_temp_file` (0x4c6b70, conmebol_seeding.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn conmebol_seeding_save_to_temp_file(this: usize) {
    const FONTE: usize = 0x9a7ed8;
    covered!("cpx_6 save CONMEBOL seeds");
    let f = open_temp_file_for_write(0x9a7f10);
    if f == 0 {
        show_error_at(FONTE, 0x2ee);
        return;
    }
    if crt_fwrite(this + conmebol_seeding::NATION_COUNT, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x2f6);
        return;
    }
    if crt_fwrite(this, 0xa, signed_byte_count(this + conmebol_seeding::NATION_COUNT), f) != signed_byte_count(this + conmebol_seeding::NATION_COUNT) { // cpx6: the country records
        show_error_at(FONTE, 0x2ff);
        return;
    }
    if crt_fwrite(this + conmebol_seeding::LAST_SEASON_YEAR, 2, 1, f) != 1 {
        show_error_at(FONTE, 0x307);
        return;
    }
    if crt_fwrite(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID, 4, 1, f) != 1 {
        show_error_at(FONTE, 0x30f);
        return;
    }
    uefa_seeding_crt_fclose(f);
}

