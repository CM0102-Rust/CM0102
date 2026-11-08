use super::*;

// ---------- 259a leva: o prêmio do mês: o melhor das notas, a vaga do vencedor e a reputação do premiado (month_award.cpp, award_manager.cpp) ----------








/// O clube `id` ([0xae23b4], 0x245 bytes cada).
pub(crate) unsafe fn cpw6_clube(id: i32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O ensaio dos comparadores: a cópia roda sem a notícia (0x767330), sem o registro do prêmio (0x86c8c0) e sem as
/// listas dos clubes (0x842c00), que alocam e não podem rodar duas vezes.
pub(crate) static mut CPW6_ENSAIO: bool = false;

/// the best of the month ratings (0x768130) in the award league and with the requested age becomes the month winner,
/// with his club
/// Original: `MONTH_AWARD::award_best_rated_player` (0x767150, month_award.cpp, thiscall, ret 8).
///
/// [pt] { 0x767150 } (thiscall, ret 8, month_award.cpp): `MONTH_AWARD::award_best_rated_player(notas, idade)`, na virada do
/// mês pelos prêmios das nações (os da Inglaterra em 0x579a37..0x579adc). O melhor das notas (0x768130) na liga do
/// prêmio ([+9], ou −1) e com a idade pedida; achou: grava o vencedor (0x7671c0) com o clube da pessoa (+0x39, ou −1),
/// o mês (+0xc) e o ano (+0xd) das notas. Os bytes altos do mês e do ano vão como a original os empurra (da tabela de
/// pessoas no registrador).
pub(crate) unsafe extern "thiscall" fn premiar_o_melhor_do_mes(this: usize, notas: usize, idade: u32) {
    let liga = {
        let l = read_usize(this + 9);
        if l != 0 {
            read_i32(l)
        } else {
            -1
        }
    };
    let melhor: unsafe extern "thiscall" fn(usize, i32, u32) -> usize = core::mem::transmute(FN_MONTH_RATINGS_FIND_BEST_PLAYER);
    let r = melhor(notas, liga, idade);
    if r == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_6 premiou o melhor do mes");
    let id = read_i32(r);
    let tabela = read_usize(G_STAFF_TABLE) as u32;
    let clube = read_usize(cpw5_pessoa(id) + 0x39);
    let ano = (tabela & 0xffff0000) | read_i16(notas + 0xd) as u16 as u32;
    let mes = read_u8(notas + 0xc) as u32;
    let gravar: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) = core::mem::transmute(FN_MONTH_AWARD_SET_WINNER);
    if clube != 0 {
        gravar(this, id, read_i32(clube), (ano & 0xffffff00) | mes, ano);
    } else {
        gravar(this, id, -1, mes, ano);
    }
}

// ponytail: o trabalho dela é chamar o melhor do mês e a vaga do vencedor, que têm comparador próprio (lado a lado e em
// ensaio); a vaga chama a notícia e o registro do prêmio, que alocam: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_premiar_o_melhor_do_mes(this: usize, notas: usize, idade: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1287)(this, notas, idade)
}

/// A reputação do premiado: no ensaio, a cópia direto (sem as listas dos clubes); senão pelo endereço (0x412f00).
pub(crate) unsafe fn cpw6_reputacao(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    if CPW6_ENSAIO {
        return reputacao_do_premiado(pessoa, clube, nac, mundo, jogador);
    }
    let f: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_AWARD_RAISE_WINNER_REPUTATION);
    f(pessoa, clube, nac, mundo, jogador);
}

/// finds the month slot, stores the person and the club, raises the winner reputation, sends the news and records the
/// award in the history
/// Original: `MONTH_AWARD::set_winner` (0x7671c0, month_award.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7671c0 } (thiscall, ret 0x10, month_award.cpp): `MONTH_AWARD::set_winner(pessoa, clube, mês, ano)`. Acha a vaga
/// do mês (12 vagas de 11 bytes: mês em +0x25, ano em +0x26); sem ela, nada. Grava a pessoa (+0x1d) e o clube (+0x21);
/// com pessoa, a reputação do premiado (0x412f00, com as reputações +0x15 e +0x17 do prêmio e a marca de jogador
/// +0x19); a notícia (0x767330) e o registro do prêmio (0x86c8c0 em [0xdd7f00], com o +5 do prêmio).
pub(crate) unsafe extern "thiscall" fn gravar_vencedor_do_mes(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let mut vaga = None;
    for k in 0..12usize {
        let v = this + k * 11;
        if read_u8(v + 0x25) == mes as u8 && read_i16(v + 0x26) == ano as u16 as i16 {
            vaga = Some(k);
            break;
        }
    }
    let k = match vaga {
        Some(k) => k,
        None => return,
    };
    let v = this + k * 11;
    write_u32(v + 0x1d, id as u32);
    write_u32(v + 0x21, clube_id as u32);
    if id > -1 {
        let clube = if clube_id > -1 { cpw6_clube(clube_id) } else { 0 };
        // os bytes altos das reputações vêm do dword +0x19 no registrador, como a original empurra
        let marca = read_usize(this + 0x19) as u32;
        let alto = marca & 0xffff0000;
        crate::comparacao::ramo("cpw_6 vencedor do mes");
        cpw6_reputacao(cpw5_pessoa(id), clube, alto | read_i16(this + 0x15) as u16 as u32, alto | read_i16(this + 0x17) as u16 as u32, marca);
    }
    if CPW6_ENSAIO {
        return;
    }
    let noticia: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MONTH_AWARD_POST_WINNER_NEWS);
    noticia(this, (this as u32 & 0xffffff00) | k as u32);
    let cid = read_i32(v + 0x21);
    let clube = if cid != -1 { cpw6_clube(cid) } else { 0 };
    let registro: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_MONTH_AWARD_RECORD);
    registro(read_usize(G_STAFF_RECORDS), cpw5_pessoa(read_i32(v + 0x1d)), clube, read_usize(this + 5) as u32, mes, ano);
}

/// As regiões que a reputação do premiado grava: o registro do jogador (+0x61: +9..+0xe), o do não jogador (+0x69:
/// +8..+0xd) e a linha da diretoria do clube (73 bytes em [[0xb67a44]], palavra +6).
pub(crate) unsafe fn cpw6_regioes_da_reputacao(regioes: &mut Vec<(usize, usize)>, pessoa: usize, clube: usize) {
    let p = read_usize(pessoa + staff::PLAYER);
    if p != 0 {
        regioes.push((p + 9, 6));
    }
    let np = read_usize(pessoa + staff::NON_PLAYER);
    if np != 0 {
        regioes.push((np + 8, 6));
    }
    if clube != 0 {
        let linhas = read_usize(read_usize(G_MANAGER_MANAGER));
        regioes.push((linhas.wrapping_add((read_i32(clube) as usize).wrapping_mul(73)).wrapping_add(6), 2));
    }
}

/// O comparador em ensaio: a cópia sem as chamadas que alocam e a original a partir do mesmo estado das `regioes`.
pub(crate) unsafe fn cpw6_comparar(nome: &str, regioes: &[(usize, usize)], original: impl FnOnce(), copia: impl FnOnce()) {
    let antes = crate::comparacao::Memoria::ler(regioes);
    CPW6_ENSAIO = true;
    crate::aleatorio::pausar_rastro(copia);
    CPW6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(regioes);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(regioes);
    PLACAR.registrar(mo == mr, || format!("{} original={:x?} rust={:x?}", nome, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_vencedor_do_mes(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, u32, u32)>(1288);
    let mut regioes = vec![(this + 0x1d, 12 * 11)];
    if id > -1 {
        cpw6_regioes_da_reputacao(&mut regioes, cpw5_pessoa(id), if clube_id > -1 { cpw6_clube(clube_id) } else { 0 });
    }
    cpw6_comparar("gravar_vencedor_do_mes", &regioes, || o(this, id, clube_id, mes, ano), || gravar_vencedor_do_mes(this, id, clube_id, mes, ano));
}

/// A palavra `a` mais o valor do _ftol (16 bits baixos, como o `add word` da original).
pub(crate) unsafe fn cpw6_somar(a: usize, v: i32) {
    write_i16(a, read_i16(a).wrapping_add(v as i16));
}

/// Teto de 10000 e piso `piso` na palavra `a` (as comparações com sinal da original).
pub(crate) unsafe fn cpw6_teto_e_piso(a: usize, piso: i16) {
    if read_i16(a) > 10000 {
        write_i16(a, 10000);
    }
    if read_i16(a) <= piso {
        write_i16(a, piso);
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
pub(crate) unsafe extern "C" fn reputacao_do_premiado(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let nac = nac as u16 as i16;
    let mundo = mundo as u16 as i16;
    let p = read_usize(pessoa + staff::PLAYER);
    if p != 0 && jogador != 0 {
        crate::comparacao::ramo("cpw_6 premio de jogador");
        cpw6_somar(p + 0xd, ftol(mundo as i32 as f64 * f64_em(0x967888))); // cpw6: o aumento mundial
        if read_i16(p + 0xd) > 10000 {
            write_i16(p + 0xd, 10000);
        }
        if read_i16(p + 0xd) <= mundo { // cpw6: o piso mundial
            write_i16(p + 0xd, mundo);
        }
        if clube != 0 && read_usize(clube + club::NATION) == read_usize(pessoa + staff::NATION) { // cpw6: o clube da nação do jogador
            cpw6_somar(p + 9, ftol(nac as i32 as f64 * f64_em(0x967880)));
            cpw6_teto_e_piso(p + 9, nac);
        } else {
            cpw6_somar(p + 9, ftol((nac as i32 + mundo as i32) as f64 * f64_em(0x967878)));
            if read_i16(p + 9) > 10000 {
                write_i16(p + 9, 10000);
            }
            let piso = nac as i32 / 2 + mundo as i32 / 2;
            if (read_i16(p + 9) as i32) <= piso {
                write_i16(p + 9, piso as i16);
            }
        }
        cpw6_somar(p + 0xb, ftol(nac as i32 as f64 * f64_em(0x967880)));
        cpw6_teto_e_piso(p + 0xb, nac);
        let listas = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
        if listas != 0 && !CPW6_ENSAIO {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
            f(listas, pessoa, 1);
        }
        return;
    }
    let np = read_usize(pessoa + staff::NON_PLAYER);
    if np == 0 || jogador != 0 {
        return;
    }
    if read_i16(np + 0xc) <= mundo {
        write_i16(np + 0xc, mundo);
    }
    if clube != 0 && read_usize(clube + club::NATION) == read_usize(pessoa + staff::NATION) {
        if read_i16(np + 8) <= nac {
            write_i16(np + 8, nac);
        }
    } else {
        let piso = nac as i32 / 2 + mundo as i32 / 2;
        if (read_i16(np + 8) as i32) <= piso {
            write_i16(np + 8, piso as i16);
        }
    }
    if read_i16(np + 0xa) <= nac {
        write_i16(np + 0xa, nac);
    }
    if clube == 0 {
        return;
    }
    let linha = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as usize).wrapping_mul(73)).wrapping_add(6);
    let v = read_i16(linha).wrapping_add(1000);
    let v = if v < 1 {
        1
    } else if v > 10000 {
        10000
    } else {
        v
    };
    write_i16(linha, if v < 6500 { 6500 } else { v });
}

pub(crate) unsafe extern "C" fn comparar_reputacao_do_premiado(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(1289);
    let mut regioes = Vec::new();
    cpw6_regioes_da_reputacao(&mut regioes, pessoa, clube);
    cpw6_comparar("reputacao_do_premiado", &regioes, || o(pessoa, clube, nac, mundo, jogador), || reputacao_do_premiado(pessoa, clube, nac, mundo, jogador));
}

