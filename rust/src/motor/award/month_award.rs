use super::*;








/// Club record with the given id (0x245 bytes each).
///
/// [pt] O clube `id` ([0xae23b4], 0x245 bytes cada).
pub(crate) unsafe fn month_award_club_by_id(id: i32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE))
}

/// O ensaio dos comparadores: a cópia roda sem a notícia (0x767330), sem o registro do prêmio (0x86c8c0) e sem as
/// listas dos clubes (0x842c00), que alocam e não podem rodar duas vezes.
pub(crate) static mut MONTH_AWARD_DRY_RUN: bool = false;

/// the best of the month ratings (0x768130) in the award league and with the requested age becomes the month winner,
/// with his club
/// Original: `MONTH_AWARD::award_best_rated_player` (0x767150, month_award.cpp, thiscall, ret 8).
///
/// [pt] { 0x767150 } (thiscall, ret 8, month_award.cpp): `MONTH_AWARD::award_best_rated_player(notas, idade)`, na virada do
/// mês pelos prêmios das nações (os da Inglaterra em 0x579a37..0x579adc). O melhor das notas (0x768130) na liga do
/// prêmio ([+9], ou −1) e com a idade pedida; achou: grava o vencedor (0x7671c0) com o clube da pessoa (+0x39, ou −1),
/// o mês (+0xc) e o ano (+0xd) das notas. Os bytes altos do mês e do ano vão como a original os empurra (da tabela de
/// pessoas no registrador).
pub(crate) unsafe extern "thiscall" fn award_best_rated_player(this: usize, notas: usize, age: u32) {
    let league = {
        let l = read_usize(this + 9);
        if l != 0 {
            read_i32(l)
        } else {
            -1
        }
    };
    let best: unsafe extern "thiscall" fn(usize, i32, u32) -> usize = core::mem::transmute(FN_MONTH_RATINGS_FIND_BEST_PLAYER);
    let r = best(notas, league, age);
    if r == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_6 premiou o melhor do mes");
    let id = read_i32(r);
    let table = read_usize(G_STAFF_TABLE) as u32;
    let clube = read_usize(month_ratings_staff_by_id(id) + 0x39);
    let year = (table & 0xffff0000) | read_i16(notas + 0xd) as u16 as u32;
    let month = read_u8(notas + 0xc) as u32;
    let write: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) = core::mem::transmute(FN_MONTH_AWARD_SET_WINNER);
    if clube != 0 {
        write(this, id, read_i32(clube), (year & 0xffffff00) | month, year);
    } else {
        write(this, id, -1, month, year);
    }
}

// simplification: o trabalho dela é chamar o melhor do mês e a vaga do vencedor, que têm comparador próprio (lado a lado e em
// ensaio); a vaga chama a notícia e o registro do prêmio, que alocam: só a original; a cópia leva o ramo.

/// Raises the award winner's reputation: the port directly in dry run, else the original 0x412f00.
///
/// [pt] A reputação do premiado: no ensaio, a cópia direto (sem as listas dos clubes); senão pelo endereço (0x412f00).
pub(crate) unsafe fn raise_winner_reputation(person: usize, clube: usize, nac: u32, mundo: u32, player: u32) {
    if MONTH_AWARD_DRY_RUN {
        return award_raise_winner_reputation(person, clube, nac, mundo, player);
    }
    let f: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_AWARD_RAISE_WINNER_REPUTATION);
    f(person, clube, nac, mundo, player);
}

/// finds the month slot, stores the person and the club, raises the winner reputation, sends the news and records the
/// award in the history
/// Original: `MONTH_AWARD::set_winner` (0x7671c0, month_award.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7671c0 } (thiscall, ret 0x10, month_award.cpp): `MONTH_AWARD::set_winner(pessoa, clube, mês, ano)`. Acha a vaga
/// do mês (12 vagas de 11 bytes: mês em +0x25, ano em +0x26); sem ela, nada. Grava a pessoa (+0x1d) e o clube (+0x21);
/// com pessoa, a reputação do premiado (0x412f00, com as reputações +0x15 e +0x17 do prêmio e a marca de jogador
/// +0x19); a notícia (0x767330) e o registro do prêmio (0x86c8c0 em [0xdd7f00], com o +5 do prêmio).
pub(crate) unsafe extern "thiscall" fn set_winner(this: usize, id: i32, club_id: i32, month: u32, year: u32) {
    let mut slot = None;
    for k in 0..12usize {
        let v = this + k * 11;
        if read_u8(v + 0x25) == month as u8 && read_i16(v + 0x26) == year as u16 as i16 {
            slot = Some(k);
            break;
        }
    }
    let k = match slot {
        Some(k) => k,
        None => return,
    };
    let v = this + k * 11;
    write_u32(v + 0x1d, id as u32);
    write_u32(v + 0x21, club_id as u32);
    if id > -1 {
        let clube = if club_id > -1 { month_award_club_by_id(club_id) } else { 0 };
        // os bytes altos das reputações vêm do dword +0x19 no registrador, como a original empurra
        let mark = read_usize(this + 0x19) as u32;
        let high = mark & 0xffff0000;
        crate::comparacao::ramo("cpw_6 vencedor do mes");
        raise_winner_reputation(month_ratings_staff_by_id(id), clube, high | read_i16(this + 0x15) as u16 as u32, high | read_i16(this + 0x17) as u16 as u32, mark);
    }
    if MONTH_AWARD_DRY_RUN {
        return;
    }
    let noticia: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MONTH_AWARD_POST_WINNER_NEWS);
    noticia(this, (this as u32 & 0xffffff00) | k as u32);
    let cid = read_i32(v + 0x21);
    let clube = if cid != -1 { month_award_club_by_id(cid) } else { 0 };
    let record: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_MONTH_AWARD_RECORD);
    record(read_usize(G_STAFF_RECORDS), month_ratings_staff_by_id(read_i32(v + 0x1d)), clube, read_usize(this + 5) as u32, month, year);
}

/// Adds the regions the winner's reputation writes: player record +9..+0xe, non-player record +8..+0xd and the club's
/// board row word (+6 of 73 bytes).
///
/// [pt] As regiões que a reputação do premiado grava: o registro do jogador (+0x61: +9..+0xe), o do não jogador (+0x69:
/// +8..+0xd) e a linha da diretoria do clube (73 bytes em [[0xb67a44]], palavra +6).
pub(crate) unsafe fn winner_reputation_regions(regions: &mut Vec<(usize, usize)>, person: usize, clube: usize) {
    let p = read_usize(person + staff::PLAYER);
    if p != 0 {
        regions.push((p + 9, 6));
    }
    let np = read_usize(person + staff::NON_PLAYER);
    if np != 0 {
        regions.push((np + 8, 6));
    }
    if clube != 0 {
        let rows = read_usize(read_usize(G_MANAGER_MANAGER));
        regions.push((rows.wrapping_add((read_i32(clube) as usize).wrapping_mul(73)).wrapping_add(6), 2));
    }
}

/// Dry-run comparer: runs the port without allocating calls, then restores and runs the original from the same region
/// state, comparing the regions.
///
/// [pt] O comparador em ensaio: a cópia sem as chamadas que alocam e a original a partir do mesmo estado das `regioes`.
pub(crate) unsafe fn compare_month_award(name: &str, regions: &[(usize, usize)], original: impl FnOnce(), copy: impl FnOnce()) {
    let before = crate::comparacao::Memoria::ler(regions);
    MONTH_AWARD_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(copy);
    MONTH_AWARD_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(regions);
    before.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(regions);
    PLACAR.registrar(mo == mr, || format!("{} original={:x?} rust={:x?}", name, mo, mr));
}


/// Adds the low 16 bits of v to the 16-bit word at a.
///
/// [pt] A palavra `a` mais o valor do _ftol (16 bits baixos, como o `add word` da original).
pub(crate) unsafe fn add_to_word(a: usize, v: i32) {
    write_i16(a, read_i16(a).wrapping_add(v as i16));
}

/// Clamps the signed 16-bit word at a to at most 10000 and at least `piso`.
///
/// [pt] Teto de 10000 e piso `piso` na palavra `a` (as comparações com sinal da original).
pub(crate) unsafe fn clamp_word(a: usize, floor: i16) {
    if read_i16(a) > 10000 {
        write_i16(a, 10000);
    }
    if read_i16(a) <= floor {
        write_i16(a, floor);
    }
}

/// player: the world, home and current reputations rise by a fraction of the award ones, up to 10000 and at least them;
/// non-player: they rise up to them and the board confidence gains 1000 (between 6500 and 10000)
/// Original: `award_raise_winner_reputation` (0x412f00, award_manager.cpp, cdecl).
///
/// [pt] { 0x412f00 } (cdecl, award_manager.cpp): `award_raise_winner_reputation(pessoa, clube, rep_nacional, rep_mundial,
/// de_jogador)`, pela vaga do vencedor do mês (0x7671c0) e pelos prêmios do ano e das seleções. Jogador (+0x61) num
/// prêmio de jogador: mundial (+0xd) + 5% da mundial do prêmio, até 10000 e pelo menos ela; nacional (+9): do clube da
/// nação dele (+0x53 = +0x1a da pessoa), + 10% da nacional e pelo menos ela; senão + 2,5% da soma e pelo menos a média;
/// atual (+0xb) + 10% da nacional, pelo menos ela; põe o jogador nas listas dos clubes (0x842c00, com 1) se houver o
/// gerente das listas. Não jogador (+0x69) num prêmio que não é de jogador: as três sobem até os pisos (+0xc mundial,
/// +8 pela nação do clube, +0xa nacional) e, com clube, a confiança da diretoria (+6 da linha de 73 bytes) + 1000, entre
/// 6500 e 10000. Só as palavras baixas das reputações valem.
pub(crate) unsafe extern "C" fn award_raise_winner_reputation(person: usize, clube: usize, nac: u32, mundo: u32, player: u32) {
    let nac = nac as u16 as i16;
    let mundo = mundo as u16 as i16;
    let p = read_usize(person + staff::PLAYER);
    if p != 0 && player != 0 {
        crate::comparacao::ramo("cpw_6 premio de jogador");
        add_to_word(p + 0xd, match_pl_ftol(mundo as i32 as f64 * read_f64_at(0x967888))); // cpw6: o aumento mundial; 0x967888 = 0.05
        if read_i16(p + 0xd) > 10000 {
            write_i16(p + 0xd, 10000);
        }
        if read_i16(p + 0xd) <= mundo { // cpw6: o piso mundial
            write_i16(p + 0xd, mundo);
        }
        if clube != 0 && read_usize(clube + club::NATION) == read_usize(person + staff::NATION) { // cpw6: o clube da nação do jogador
            add_to_word(p + 9, match_pl_ftol(nac as i32 as f64 * read_f64_at(0x967880))); // 0x967880 = 0.1
            clamp_word(p + 9, nac);
        } else {
            add_to_word(p + 9, match_pl_ftol((nac as i32 + mundo as i32) as f64 * read_f64_at(0x967878))); // 0x967878 = 0.025
            if read_i16(p + 9) > 10000 {
                write_i16(p + 9, 10000);
            }
            let floor = nac as i32 / 2 + mundo as i32 / 2;
            if (read_i16(p + 9) as i32) <= floor {
                write_i16(p + 9, floor as i16);
            }
        }
        add_to_word(p + 0xb, match_pl_ftol(nac as i32 as f64 * read_f64_at(0x967880))); // 0x967880 = 0.1
        clamp_word(p + 0xb, nac);
        let lists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
        if lists != 0 && !MONTH_AWARD_DRY_RUN {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
            f(lists, person, 1);
        }
        return;
    }
    let np = read_usize(person + staff::NON_PLAYER);
    if np == 0 || player != 0 {
        return;
    }
    if read_i16(np + 0xc) <= mundo {
        write_i16(np + 0xc, mundo);
    }
    if clube != 0 && read_usize(clube + club::NATION) == read_usize(person + staff::NATION) {
        if read_i16(np + 8) <= nac {
            write_i16(np + 8, nac);
        }
    } else {
        let floor = nac as i32 / 2 + mundo as i32 / 2;
        if (read_i16(np + 8) as i32) <= floor {
            write_i16(np + 8, floor as i16);
        }
    }
    if read_i16(np + 0xa) <= nac {
        write_i16(np + 0xa, nac);
    }
    if clube == 0 {
        return;
    }
    let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as usize).wrapping_mul(73)).wrapping_add(6);
    let v = read_i16(row).wrapping_add(1000);
    let v = if v < 1 {
        1
    } else if v > 10000 {
        10000
    } else {
        v
    };
    write_i16(row, if v < 6500 { 6500 } else { v });
}


