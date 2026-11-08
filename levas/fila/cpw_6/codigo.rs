// ---------- %%LEVA%%a leva: o prêmio do mês: o melhor das notas, a vaga do vencedor e a reputação do premiado (month_award.cpp, award_manager.cpp) ----------

unsafe fn cpw6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw6_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpw6_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A pessoa `id` ([0xae23bc], 0x6e bytes cada).
unsafe fn cpw6_pessoa(id: i32) -> usize {
    cpw6_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e))
}

/// O clube `id` ([0xae23b4], 0x245 bytes cada).
unsafe fn month_award_club_by_id(id: i32) -> usize {
    cpw6_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O ensaio dos comparadores: a cópia roda sem a notícia (0x767330), sem o registro do prêmio (0x86c8c0) e sem as
/// listas dos clubes (0x842c00), que alocam e não podem rodar duas vezes.
static mut MONTH_AWARD_DRY_RUN: bool = false;

/// { 0x767150 } (thiscall, ret 8, month_award.cpp): `MONTH_AWARD::award_best_rated_player(notas, idade)`, na virada do
/// mês pelos prêmios das nações (os da Inglaterra em 0x579a37..0x579adc). O melhor das notas (0x768130) na liga do
/// prêmio ([+9], ou −1) e com a idade pedida; achou: grava o vencedor (0x7671c0) com o clube da pessoa (+0x39, ou −1),
/// o mês (+0xc) e o ano (+0xd) das notas. Os bytes altos do mês e do ano vão como a original os empurra (da tabela de
/// pessoas no registrador).
unsafe extern "thiscall" fn award_best_rated_player(this: usize, notas: usize, idade: u32) {
    let liga = {
        let l = cpw6_d(this + 9);
        if l != 0 {
            cpw6_i(l)
        } else {
            -1
        }
    };
    let melhor: unsafe extern "thiscall" fn(usize, i32, u32) -> usize = core::mem::transmute(0x768130usize);
    let r = melhor(notas, liga, idade);
    if r == 0 {
        return;
    }
    crate::comparison::branch("cpw_6 premiou o melhor do mes");
    let id = cpw6_i(r);
    let tabela = cpw6_d(0xae23bc) as u32;
    let clube = cpw6_d(cpw6_pessoa(id) + 0x39);
    let ano = (tabela & 0xffff0000) | cpw6_w(notas + 0xd) as u16 as u32;
    let mes = cpw6_b(notas + 0xc) as u32;
    let gravar: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) = core::mem::transmute(0x7671c0usize);
    if clube != 0 {
        gravar(this, id, cpw6_i(clube), (ano & 0xffffff00) | mes, ano);
    } else {
        gravar(this, id, -1, mes, ano);
    }
}

// ponytail: o trabalho dela é chamar o melhor do mês e a vaga do vencedor, que têm comparador próprio (lado a lado e em
// ensaio); a vaga chama a notícia e o registro do prêmio, que alocam: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_award_best_rated_player(this: usize, notas: usize, idade: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, notas, idade)
}

/// A reputação do premiado: no ensaio, a cópia direto (sem as listas dos clubes); senão pelo endereço (0x412f00).
unsafe fn raise_winner_reputation(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    if MONTH_AWARD_DRY_RUN {
        return award_raise_winner_reputation(pessoa, clube, nac, mundo, jogador);
    }
    let f: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x412f00usize);
    f(pessoa, clube, nac, mundo, jogador);
}

/// { 0x7671c0 } (thiscall, ret 0x10, month_award.cpp): `MONTH_AWARD::set_winner(pessoa, clube, mês, ano)`. Acha a vaga
/// do mês (12 vagas de 11 bytes: mês em +0x25, ano em +0x26); sem ela, nada. Grava a pessoa (+0x1d) e o clube (+0x21);
/// com pessoa, a reputação do premiado (0x412f00, com as reputações +0x15 e +0x17 do prêmio e a marca de jogador
/// +0x19); a notícia (0x767330) e o registro do prêmio (0x86c8c0 em [0xdd7f00], com o +5 do prêmio).
unsafe extern "thiscall" fn set_winner(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let mut vaga = None;
    for k in 0..12usize {
        let v = this + k * 11;
        if cpw6_b(v + 0x25) == mes as u8 && cpw6_w(v + 0x26) == ano as u16 as i16 {
            vaga = Some(k);
            break;
        }
    }
    let k = match vaga {
        Some(k) => k,
        None => return,
    };
    let v = this + k * 11;
    cpw6_pd(v + 0x1d, id as u32);
    cpw6_pd(v + 0x21, clube_id as u32);
    if id > -1 {
        let clube = if clube_id > -1 { month_award_club_by_id(clube_id) } else { 0 };
        // os bytes altos das reputações vêm do dword +0x19 no registrador, como a original empurra
        let marca = cpw6_d(this + 0x19) as u32;
        let alto = marca & 0xffff0000;
        crate::comparison::branch("cpw_6 vencedor do mes");
        raise_winner_reputation(cpw6_pessoa(id), clube, alto | cpw6_w(this + 0x15) as u16 as u32, alto | cpw6_w(this + 0x17) as u16 as u32, marca);
    }
    if MONTH_AWARD_DRY_RUN {
        return;
    }
    let noticia: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x767330usize);
    noticia(this, (this as u32 & 0xffffff00) | k as u32);
    let cid = cpw6_i(v + 0x21);
    let clube = if cid != -1 { month_award_club_by_id(cid) } else { 0 };
    let registro: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c8c0usize);
    registro(cpw6_d(0xdd7f00), cpw6_pessoa(cpw6_i(v + 0x1d)), clube, cpw6_d(this + 5) as u32, mes, ano);
}

/// As regiões que a reputação do premiado grava: o registro do jogador (+0x61: +9..+0xe), o do não jogador (+0x69:
/// +8..+0xd) e a linha da diretoria do clube (73 bytes em [[0xb67a44]], palavra +6).
unsafe fn winner_reputation_regions(regioes: &mut Vec<(usize, usize)>, pessoa: usize, clube: usize) {
    let p = cpw6_d(pessoa + 0x61);
    if p != 0 {
        regioes.push((p + 9, 6));
    }
    let np = cpw6_d(pessoa + 0x69);
    if np != 0 {
        regioes.push((np + 8, 6));
    }
    if clube != 0 {
        let linhas = cpw6_d(cpw6_d(0xb67a44));
        regioes.push((linhas.wrapping_add((cpw6_i(clube) as usize).wrapping_mul(73)).wrapping_add(6), 2));
    }
}

/// O comparador em ensaio: a cópia sem as chamadas que alocam e a original a partir do mesmo estado das `regioes`.
unsafe fn compare_month_award(nome: &str, regioes: &[(usize, usize)], original: impl FnOnce(), copia: impl FnOnce()) {
    let antes = crate::comparison::MemorySnapshot::read(regioes);
    MONTH_AWARD_DRY_RUN = true;
    crate::rng::pause_trace(copia);
    MONTH_AWARD_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(regioes);
    antes.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(regioes);
    SCOREBOARD.record(mo == mr, || format!("{} original={:x?} rust={:x?}", nome, mo, mr));
}

unsafe extern "thiscall" fn compare_set_winner(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, u32)>(%%I1%%);
    let mut regioes = vec![(this + 0x1d, 12 * 11)];
    if id > -1 {
        winner_reputation_regions(&mut regioes, cpw6_pessoa(id), if clube_id > -1 { month_award_club_by_id(clube_id) } else { 0 });
    }
    compare_month_award("set_winner", &regioes, || o(this, id, clube_id, mes, ano), || set_winner(this, id, clube_id, mes, ano));
}

/// A palavra `a` mais o valor do _ftol (16 bits baixos, como o `add word` da original).
unsafe fn add_to_word(a: usize, v: i32) {
    cpw6_pw(a, cpw6_w(a).wrapping_add(v as i16));
}

/// Teto de 10000 e piso `piso` na palavra `a` (as comparações com sinal da original).
unsafe fn clamp_word(a: usize, piso: i16) {
    if cpw6_w(a) > 10000 {
        cpw6_pw(a, 10000);
    }
    if cpw6_w(a) <= piso {
        cpw6_pw(a, piso);
    }
}

/// { 0x412f00 } (cdecl, award_manager.cpp): `award_raise_winner_reputation(pessoa, clube, rep_nacional, rep_mundial,
/// de_jogador)`, pela vaga do vencedor do mês (0x7671c0) e pelos prêmios do ano e das seleções. Jogador (+0x61) num
/// prêmio de jogador: mundial (+0xd) + 5% da mundial do prêmio, até 10000 e pelo menos ela; nacional (+9): do clube da
/// nação dele (+0x53 = +0x1a da pessoa), + 10% da nacional e pelo menos ela; senão + 2,5% da soma e pelo menos a média;
/// atual (+0xb) + 10% da nacional, pelo menos ela; põe o jogador nas listas dos clubes (0x842c00, com 1) se houver o
/// gerente das listas. Não jogador (+0x69) num prêmio que não é de jogador: as três sobem até os pisos (+0xc mundial,
/// +8 pela nação do clube, +0xa nacional) e, com clube, a confiança da diretoria (+6 da linha de 73 bytes) + 1000, entre
/// 6500 e 10000. Só as palavras baixas das reputações valem.
unsafe extern "C" fn award_raise_winner_reputation(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let nac = nac as u16 as i16;
    let mundo = mundo as u16 as i16;
    let p = cpw6_d(pessoa + 0x61);
    if p != 0 && jogador != 0 {
        crate::comparison::branch("cpw_6 premio de jogador");
        add_to_word(p + 0xd, match_pl_ftol(mundo as i32 as f64 * read_f64_at(0x967888))); // cpw6: o aumento mundial
        if cpw6_w(p + 0xd) > 10000 {
            cpw6_pw(p + 0xd, 10000);
        }
        if cpw6_w(p + 0xd) <= mundo { // cpw6: o piso mundial
            cpw6_pw(p + 0xd, mundo);
        }
        if clube != 0 && cpw6_d(clube + 0x53) == cpw6_d(pessoa + 0x1a) { // cpw6: o clube da nação do jogador
            add_to_word(p + 9, match_pl_ftol(nac as i32 as f64 * read_f64_at(0x967880)));
            clamp_word(p + 9, nac);
        } else {
            add_to_word(p + 9, match_pl_ftol((nac as i32 + mundo as i32) as f64 * read_f64_at(0x967878)));
            if cpw6_w(p + 9) > 10000 {
                cpw6_pw(p + 9, 10000);
            }
            let piso = nac as i32 / 2 + mundo as i32 / 2;
            if (cpw6_w(p + 9) as i32) <= piso {
                cpw6_pw(p + 9, piso as i16);
            }
        }
        add_to_word(p + 0xb, match_pl_ftol(nac as i32 as f64 * read_f64_at(0x967880)));
        clamp_word(p + 0xb, nac);
        let listas = cpw6_d(cpw6_d(0xde1f54) + 0x856);
        if listas != 0 && !MONTH_AWARD_DRY_RUN {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
            f(listas, pessoa, 1);
        }
        return;
    }
    let np = cpw6_d(pessoa + 0x69);
    if np == 0 || jogador != 0 {
        return;
    }
    if cpw6_w(np + 0xc) <= mundo {
        cpw6_pw(np + 0xc, mundo);
    }
    if clube != 0 && cpw6_d(clube + 0x53) == cpw6_d(pessoa + 0x1a) {
        if cpw6_w(np + 8) <= nac {
            cpw6_pw(np + 8, nac);
        }
    } else {
        let piso = nac as i32 / 2 + mundo as i32 / 2;
        if (cpw6_w(np + 8) as i32) <= piso {
            cpw6_pw(np + 8, piso as i16);
        }
    }
    if cpw6_w(np + 0xa) <= nac {
        cpw6_pw(np + 0xa, nac);
    }
    if clube == 0 {
        return;
    }
    let linha = cpw6_d(cpw6_d(0xb67a44)).wrapping_add((cpw6_i(clube) as usize).wrapping_mul(73)).wrapping_add(6);
    let v = cpw6_w(linha).wrapping_add(1000);
    let v = if v < 1 {
        1
    } else if v > 10000 {
        10000
    } else {
        v
    };
    cpw6_pw(linha, if v < 6500 { 6500 } else { v });
}

unsafe extern "C" fn compare_award_raise_winner_reputation(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(%%I2%%);
    let mut regioes = Vec::new();
    winner_reputation_regions(&mut regioes, pessoa, clube);
    compare_month_award("award_raise_winner_reputation", &regioes, || o(pessoa, clube, nac, mundo, jogador), || award_raise_winner_reputation(pessoa, clube, nac, mundo, jogador));
}
