use super::*;

/// Fills the match record as seen by the club: date, fixture id, competition, venue, club, opponent and goals for and
/// against (the extra-time score, if any).
///
/// `fill_match_record_from_fixture(match, club, out)`, from the tactic statistics (`record_tactic_result`, at the end of each match
/// without a human) and from the records. Null match, club or output: the box (lines 0x29, 0x31, 0x39) and 0. Otherwise
/// the date (+0, `exe::date_operator_3d` of the Date(day +0x2a, year +0x28) from 0x549ef0), +8 the match id (+0), +0xc word +0x32,
/// +0xe word +0x30, +0x10 the side (`comp_util_get_fixture_venue_side`, cdecl(club, match)), +0x11 the club id, +0x1b dword +8 of the match; home
/// side (+0x1c): +0x15 the match's +0x10 and the score (+0x19 for, +0x1a against) from the extra-time one (+0x47/+0x48)
/// if it is not negative, otherwise the normal one (+0x43/+0x44); away side: +0x15 the +0xc and the score reversed.
/// Returns 1.
/// Original: `fill_match_record_from_fixture` (0x7e4750, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e4750 } (cdecl, record_utils.cpp): `fill_match_record_from_fixture(jogo, clube, saida)`, da estatística da tática
/// (0x896290, no fim de cada partida sem humano) e dos recordes. Jogo, clube ou saída nulos: a caixa (linhas 0x29, 0x31,
/// 0x39) e 0. Senão a data (+0, 0x417c10 da Date(dia +0x2a, ano +0x28) da 0x549ef0), +8 o id do jogo (+0), +0xc a
/// palavra +0x32, +0xe a +0x30, +0x10 o lado (0x4afa80, cdecl(clube, jogo)), +0x11 o id do clube, +0x1b o dword +8 do
/// jogo; mandante (+0x1c): +0x15 o +0x10 do jogo e o placar (+0x19 pró, +0x1a contra) pelo da prorrogação (+0x47/+0x48)
/// se ele não for negativo, senão o normal (+0x43/+0x44); visitante: +0x15 o +0xc e o placar ao contrário. Devolve 1.
pub(crate) unsafe extern "C" fn fill_match_record_from_fixture(game: usize, club: usize, out: usize) -> u32 {
    let game = Fixture(game);
    if game.is_null() {
        show_error_at(0xa7a088, 0x29);
        return 0;
    }
    if club == 0 {
        show_error_at(0xa7a088, 0x31);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa7a088, 0x39);
        return 0;
    }
    let (mut t0, mut t1) = ([0u8; 8], [0u8; 8]);
    let (p0, p1) = (t0.as_mut_ptr() as usize, t1.as_mut_ptr() as usize);
    let d = exe::new_date(p0, game.day() as u32, game.year() as u32);
    exe::date_operator_3d(out, p1, d);
    core::ptr::write_unaligned(((out + 8) as *mut u8) as *mut u32, read_u32(game.addr()));
    core::ptr::write_unaligned(((out + 0xc) as *mut u8) as *mut u16, (game.round_prefix() as u16));
    core::ptr::write_unaligned(((out + 0xe) as *mut u8) as *mut u16, (game.round_name() as u16));
    *((out + 0x10) as *mut u8) = exe::comp_util_get_fixture_venue_side(club, game.addr());
    core::ptr::write_unaligned(((out + 0x11) as *mut u8) as *mut u32, read_u32(club));
    core::ptr::write_unaligned(((out + 0x1b) as *mut u8) as *mut u32, (game.match_stats_slot() as u32));
    if club as u32 == (game.home_club().addr() as u32) { // cpz7: the club is the home team
        core::ptr::write_unaligned(((out + 0x15) as *mut u8) as *mut u32, (game.away_club_id() as u32));
        if (game.home_goals_extra_time() as u8) as i8 > -1 {
            *((out + 0x19) as *mut u8) = (game.home_goals_extra_time() as u8);
            *((out + 0x1a) as *mut u8) = (game.away_goals_extra_time() as u8);
        } else {
            crate::comparison::branch("cpz_7 home side score");
            *((out + 0x19) as *mut u8) = (game.home_goals() as u8); // cpz7: the home team goals
            *((out + 0x1a) as *mut u8) = (game.away_goals() as u8);
        }
    } else {
        core::ptr::write_unaligned(((out + 0x15) as *mut u8) as *mut u32, (game.home_club_id() as u32));
        if (game.away_goals_extra_time() as u8) as i8 > -1 {
            *((out + 0x19) as *mut u8) = (game.away_goals_extra_time() as u8);
            *((out + 0x1a) as *mut u8) = (game.home_goals_extra_time() as u8);
        } else {
            crate::comparison::branch("cpz_7 away side score");
            *((out + 0x19) as *mut u8) = (game.away_goals() as u8);
            *((out + 0x1a) as *mut u8) = (game.home_goals() as u8); // cpz7: the goals conceded by the away team
        }
    }
    1
}

/// Name id (+0x33 of a name record), or -1 without name.
///
/// [pt] O id do nome (+0x33 do registro de nome) ou −1 sem nome.
pub(crate) unsafe fn name_id(p: u32) -> u32 {
    if p != 0 { read_u32(p as usize + 0x33) } else { u32::MAX }
}

/// Stores the three name ids (−1 when missing), the person's id and birth date in a compact reference.
///
/// `fill_staff_name_ref(person, out)`, from the club and competition records, the suspension news, the award and the
/// player search. Null person or output: the box (lines 0x80, 0x88) and 0. Otherwise +0, +4, +8 the name ids (+4, +8,
/// +0xc of the person; −1 with no name), +0xc the person's id and +0x10 the birth date (+0x10, `exe::date_operator_3d`); returns 1.
/// Original: `fill_staff_name_ref` (0x7e4920, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e4920 } (cdecl, record_utils.cpp): `fill_staff_name_ref(pessoa, saida)`, dos recordes do clube e da competição,
/// da notícia da suspensão, do prêmio e da busca de jogadores. Pessoa ou saída nulas: a caixa (linhas 0x80, 0x88) e 0.
/// Senão +0, +4, +8 os ids dos nomes (+4, +8, +0xc da pessoa; −1 sem nome), +0xc o id da pessoa e +0x10 a data de
/// nascimento (+0x10, 0x417c10); devolve 1.
pub(crate) unsafe extern "C" fn fill_staff_name_ref(person: usize, out: usize) -> u32 {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0xa7a088, 0x80);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa7a088, 0x88);
        return 0;
    }
    for k in 0..3 {
        core::ptr::write_unaligned((out + 4 * k) as *mut u32, name_id(read_u32(person.addr() + staff::FIRST_NAME + 4 * k)));
    }
    core::ptr::write_unaligned((out + 0xc) as *mut u32, read_u32(person.addr()));
    let mut t = [0u8; 8];
    exe::date_operator_3d(out + 0x10, t.as_mut_ptr() as usize, person.addr() + staff::DATE_OF_BIRTH);
    1
}

/// Returns the result code of the record: 3 win, 2 score draw, 1 goalless draw, −3 defeat.
///
/// `match_record_result_code(record)`, from the competition statistics (`exe::comp_stats_record_match_result`, on each match). Null record: the box
/// (line 0x25a) and 0. For (+0x19) > against (+0x1a), signed: 3; draw with goals: 2; goalless draw: 1; defeat: −3 (only
/// the byte counts: the caller reads al).
/// Original: `match_record_result_code` (0x7e56d0, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e56d0 } (cdecl, record_utils.cpp): `match_record_result_code(registro)`, da estatística da competição
/// (0x49f570, a cada partida). Registro nulo: a caixa (linha 0x25a) e 0. Pró (+0x19) > contra (+0x1a), com sinal: 3;
/// empate com gols: 2; empate sem gols: 1; derrota: −3 (só o byte conta: quem chama lê al).
pub(crate) unsafe extern "C" fn match_record_result_code(record: usize) -> u8 {
    if record == 0 {
        show_error_at(0xa7a088, 0x25a);
        return 0;
    }
    let (p, c) = (read_u8(record + 0x19) as i8, read_u8(record + 0x1a) as i8);
    if p > c {
        3
    } else if p == c && p > 0 {
        2
    } else if p == c {
        1
    } else {
        (-3i8) as u8
    }
}

/// Body of Staff_Records::add_year_award_record (`add_year_award_record`); without gravar (the comparer's copy) skips the history file
/// entry.
///
/// [pt] O corpo da 0x86c390; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
pub(crate) unsafe fn add_year_award(this: usize, p: Staff, club: usize, prize: usize, a4: u32, a5: u32, credito: u32, write: bool) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x34;
    put_i32(&mut e, 8, if club != 0 { read_usize(club) as i32 } else { -1 });
    put_i32(&mut e, 0xd, read_usize(prize) as i32);
    put_i32(&mut e, 0x11, (a4 & 0xffff) as i32);
    put_i32(&mut e, 0x15, a5 as u8 as i8 as i32);
    put_i32(&mut e, 0x19, credito as u8 as i8 as i32);
    append_person_history_by_id(write, this, read_usize(p.addr()), &e);
    let (t3d, t28) = (p.job_for_club(), p.job_for_nation());
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = staff_records_manager_record(this, p);
        if r != 0 {
            crate::comparison::branch("cpy_10 manager award");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
        }
        return;
    }
    if p.player().is_null() {
        return;
    }
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c.addr();
    if c.is_null() || c.contract_type() & 0x3f == 5 {
        return;
    }
    if read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE)) == 0 {
        return;
    }
    let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, contract_club_except_type5(p)));
    if k.is_null() {
        return;
    }
    if read_usize(G_TRANSFER_MANAGER) != 0 {
        if exe::date_days_since(G_CURRENT_DATE, (k.date_started()) as u32, (read_usize(k.addr() + contract::DATE_STARTED + 4)) as u32) > 0x12c { // cpy10: mais de 300 dias
            let owner = contract_club_except_type5(p);
            let x = exe::shortlist_manager_find_club_shortlist_entry(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), owner, p, 0, 0);
            if x != 0 && read_usize(x + 0xf) == read_usize(p.addr()) {
                crate::comparison::branch("cpy_10 renewal");
                offer_renewal(p.addr());
            }
        }
    }
    if credito as u8 != 1 {
        crate::comparison::branch("cpy_10 without credit");
        return; // cpy10: só o creditado
    }
    if (k.grievances() as usize) != 0 {
        return;
    }
    let v59 = (p.loyalty() as u8) as i8;
    if v59 <= 0xf { // cpy10: o +0x59
        return;
    }
    if (p.professionalism() as u8) as i8 <= 0xa {
        return;
    }
    crate::comparison::branch("cpy_10 thanks draw");
    if random(v59 as i32) <= 5 {
        return;
    }
    exe::news_post_player_credits_manager_for_award(p, club, prize);
}

/// Tells whether the match record is a win by a larger margin than the other one, or the same margin with more goals
/// scored.
///
/// `match_record_is_bigger_win(a, b)`, from the club records after each match (0x444930) and from the competition ones
/// (0x49fb93): 1 if `a` is a win (goals +0x19 > +0x1a, signed bytes) by a larger margin than that of `b`, or equal with
/// more goals scored. Null record: the original routine ("Error" box, 0).
/// Original: `match_record_is_bigger_win` (0x7e4a30, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e4a30 } (cdecl, record_utils.cpp): `match_record_is_bigger_win(a, b)`, pelos recordes do clube depois de cada
/// jogo (0x444930) e pelos da competição (0x49fb93): 1 se `a` é vitória (gols +0x19 > +0x1a, bytes com sinal) por
/// diferença maior que a de `b`, ou igual com mais gols feitos. Registro nulo: a original (caixa "Error", 0).
pub(crate) unsafe extern "C" fn match_record_is_bigger_win(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_WIN)(a, b);
    }
    let (fa, sa, fb, signed_byte_at) = (read_u8(a + 0x19) as i8, read_u8(a + 0x1a) as i8, read_u8(b + 0x19) as i8, read_u8(b + 0x1a) as i8);
    if fa <= sa {
        return 0;
    }
    let (da, db) = (fa as i32 - sa as i32, fb as i32 - signed_byte_at as i32);
    (da > db || (da == db && fa > fb)) as u32
}

/// Tells whether the match record is a defeat by a larger margin than the other one, or the same margin with more goals
/// conceded.
///
/// `match_record_is_bigger_defeat(a, b)`, from the club records after each match (0x444930): 1 if `a` is a defeat
/// (+0x19 < +0x1a) by a larger margin than that of `b`, or equal with more goals conceded. Null record: the original
/// routine ("Error" box, 0).
/// Original: `match_record_is_bigger_defeat` (0x7e4b00, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e4b00 } (cdecl, record_utils.cpp): `match_record_is_bigger_defeat(a, b)`, pelos recordes do clube depois de cada
/// jogo (0x444930): 1 se `a` é derrota (+0x19 < +0x1a) por diferença maior que a de `b`, ou igual com mais gols
/// sofridos. Registro nulo: a original (caixa "Error", 0).
pub(crate) unsafe extern "C" fn match_record_is_bigger_defeat(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_DEFEAT)(a, b);
    }
    let (fa, sa, fb, signed_byte_at) = (read_u8(a + 0x19) as i8, read_u8(a + 0x1a) as i8, read_u8(b + 0x19) as i8, read_u8(b + 0x1a) as i8);
    if fa >= sa {
        return 0;
    }
    let (da, db) = (sa as i32 - fa as i32, signed_byte_at as i32 - fb as i32);
    (da > db || (da == db && sa > signed_byte_at)) as u32
}

