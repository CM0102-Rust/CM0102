use super::*;




/// fills the match record as seen by the club: date, fixture id, competition, venue, club, opponent and goals for and
/// against (the extra-time score, if any)
/// Original: `fill_match_record_from_fixture` (0x7e4750, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e4750 } (cdecl, record_utils.cpp): `fill_match_record_from_fixture(jogo, clube, saida)`, da estatística da tática
/// (0x896290, no fim de cada partida sem humano) e dos recordes. Jogo, clube ou saída nulos: a caixa (linhas 0x29, 0x31,
/// 0x39) e 0. Senão a data (+0, 0x417c10 da Date(dia +0x2a, ano +0x28) da 0x549ef0), +8 o id do jogo (+0), +0xc a
/// palavra +0x32, +0xe a +0x30, +0x10 o lado (0x4afa80, cdecl(clube, jogo)), +0x11 o id do clube, +0x1b o dword +8 do
/// jogo; mandante (+0x1c): +0x15 o +0x10 do jogo e o placar (+0x19 pró, +0x1a contra) pelo da prorrogação (+0x47/+0x48)
/// se ele não for negativo, senão o normal (+0x43/+0x44); visitante: +0x15 o +0xc e o placar ao contrário. Devolve 1.
pub(crate) unsafe extern "C" fn fill_match_record_from_fixture(jogo: usize, clube: usize, out: usize) -> u32 {
    if jogo == 0 {
        show_error_at(0xa7a088, 0x29);
        return 0;
    }
    if clube == 0 {
        show_error_at(0xa7a088, 0x31);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa7a088, 0x39);
        return 0;
    }
    let data_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let side: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_FIXTURE_VENUE_SIDE);
    let (mut t0, mut t1) = ([0u8; 8], [0u8; 8]);
    let (p0, p1) = (t0.as_mut_ptr() as usize, t1.as_mut_ptr() as usize);
    let d = data_new(p0, read_u16(jogo + 0x2a) as u32, read_u16(jogo + 0x28) as u32);
    copy(out, p1, d);
    let s = |o: usize| (out + o) as *mut u8;
    core::ptr::write_unaligned(s(8) as *mut u32, read_u32(jogo));
    core::ptr::write_unaligned(s(0xc) as *mut u16, read_u16(jogo + 0x32));
    core::ptr::write_unaligned(s(0xe) as *mut u16, read_u16(jogo + 0x30));
    *s(0x10) = side(clube, jogo);
    core::ptr::write_unaligned(s(0x11) as *mut u32, read_u32(clube));
    core::ptr::write_unaligned(s(0x1b) as *mut u32, read_u32(jogo + 8));
    if clube as u32 == read_u32(jogo + 0x1c) { // cpz7: o clube é o mandante
        core::ptr::write_unaligned(s(0x15) as *mut u32, read_u32(jogo + 0x10));
        if read_u8(jogo + 0x47) as i8 > -1 {
            *s(0x19) = read_u8(jogo + 0x47);
            *s(0x1a) = read_u8(jogo + 0x48);
        } else {
            crate::comparacao::ramo("cpz_7 placar do mandante");
            *s(0x19) = read_u8(jogo + 0x43); // cpz7: os gols do mandante
            *s(0x1a) = read_u8(jogo + 0x44);
        }
    } else {
        core::ptr::write_unaligned(s(0x15) as *mut u32, read_u32(jogo + 0xc));
        if read_u8(jogo + 0x48) as i8 > -1 {
            *s(0x19) = read_u8(jogo + 0x48);
            *s(0x1a) = read_u8(jogo + 0x47);
        } else {
            crate::comparacao::ramo("cpz_7 placar do visitante");
            *s(0x19) = read_u8(jogo + 0x44);
            *s(0x1a) = read_u8(jogo + 0x43); // cpz7: os gols sofridos pelo visitante
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

/// stores the three name ids (-1 when missing), the person's id and birth date in a compact reference
/// Original: `fill_staff_name_ref` (0x7e4920, record_utils.cpp, cdecl).
///
/// [pt] { 0x7e4920 } (cdecl, record_utils.cpp): `fill_staff_name_ref(pessoa, saida)`, dos recordes do clube e da competição,
/// da notícia da suspensão, do prêmio e da busca de jogadores. Pessoa ou saída nulas: a caixa (linhas 0x80, 0x88) e 0.
/// Senão +0, +4, +8 os ids dos nomes (+4, +8, +0xc da pessoa; −1 sem nome), +0xc o id da pessoa e +0x10 a data de
/// nascimento (+0x10, 0x417c10); devolve 1.
pub(crate) unsafe extern "C" fn fill_staff_name_ref(person: usize, out: usize) -> u32 {
    if person == 0 {
        show_error_at(0xa7a088, 0x80);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa7a088, 0x88);
        return 0;
    }
    for k in 0..3 {
        core::ptr::write_unaligned((out + 4 * k) as *mut u32, name_id(read_u32(person + 4 + 4 * k)));
    }
    core::ptr::write_unaligned((out + 0xc) as *mut u32, read_u32(person));
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copy(out + 0x10, t.as_mut_ptr() as usize, person + staff::DATE_OF_BIRTH);
    1
}


/// the result code of the record: 3 win, 2 score draw, 1 goalless draw, -3 defeat
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




/// Body of Staff_Records::add_year_award_record (0x86c390); without gravar (the comparer's copy) skips the history file
/// entry.
///
/// [pt] O corpo da 0x86c390; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
pub(crate) unsafe fn add_year_award(this: usize, p: usize, clube: usize, prize: usize, a4: u32, a5: u32, credito: u32, write: bool) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x34;
    put_i32(&mut e, 8, if clube != 0 { read_usize(clube) as i32 } else { -1 });
    put_i32(&mut e, 0xd, read_usize(prize) as i32);
    put_i32(&mut e, 0x11, (a4 & 0xffff) as i32);
    put_i32(&mut e, 0x15, a5 as u8 as i8 as i32);
    put_i32(&mut e, 0x19, credito as u8 as i8 as i32);
    append_person_history_by_id(write, this, read_usize(p), &e);
    let (t3d, t28) = (read_u8(p + 0x3d), read_u8(p + 0x28));
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = staff_records_manager_record(this, p);
        if r != 0 {
            crate::comparacao::ramo("cpy_10 prêmio do técnico");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
        }
        return;
    }
    if read_usize(p + 0x61) == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        return;
    }
    if read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE)) == 0 {
        return;
    }
    let of_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let k = of_club(read_usize(G_CONTRACT_MANAGER), p, contract_club_except_type5(p));
    if k == 0 {
        return;
    }
    if read_usize(G_TRANSFER_MANAGER) != 0 {
        let days: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        if days(G_CURRENT_DATE, read_usize(k + 0x25), read_usize(k + 0x29)) > 0x12c { // cpy10: mais de 300 dias
            let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
            let owner = contract_club_except_type5(p);
            let x = entry(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), owner, p, 0, 0);
            if x != 0 && read_usize(x + 0xf) == read_usize(p) {
                crate::comparacao::ramo("cpy_10 renovação");
                offer_renewal(p);
            }
        }
    }
    if credito as u8 != 1 {
        crate::comparacao::ramo("cpy_10 sem crédito");
        return; // cpy10: só o creditado
    }
    if read_usize(k + 0x45) != 0 {
        return;
    }
    let v59 = read_u8(p + 0x59) as i8;
    if v59 <= 0xf { // cpy10: o +0x59
        return;
    }
    if read_u8(p + 0x5b) as i8 <= 0xa {
        return;
    }
    crate::comparacao::ramo("cpy_10 sorteio do agradecimento");
    if sortear(v59 as i32) <= 5 {
        return;
    }
    let agradece: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_POST_PLAYER_CREDITS_MANAGER_FOR_AWARD);
    agradece(p, clube, prize);
}



/// the match record is a win by a larger margin than the other one, or the same with more goals scored
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


/// the match record is a defeat by a larger margin than the other one, or the same with more goals conceded
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


