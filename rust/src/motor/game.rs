use super::*;

// ---------- 256a leva: os coeficientes das nações (o bônus de dezembro e a história de cinco anos), o dia das aposentadorias e a soma presa de dois bytes (game.cpp) ----------






/// A história do coeficiente da nação: +0xb0, +0xb8, +0xc0, +0xc8, +0xd0 (cinco anos) e +0xd8 (o ano corrente).
pub(crate) unsafe fn cpt6_historia(na: usize) -> [f64; 6] {
    let mut h = [0f64; 6];
    for (k, x) in h.iter_mut().enumerate() {
        *x = read_f64(na + 0xb0 + k * 8);
    }
    h
}

/// O mês da data do jogo (0x54cfb0 sobre 0xae2c90, o al com sinal).
pub(crate) unsafe fn cpt6_mes() -> i32 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    mes(0xae2c90) as u8 as i8 as i32
}

/// Dezembro na história `h` da nação `na` (`marca` = +0x121): com marca < 12, o ano corrente ganha
/// reputação (+0x8e) × 0,05 × (12 − marca) × 3; a história anda um ano, o ano corrente e a marca zeram.
pub(crate) unsafe fn cpt6_dezembro(na: usize, h: &mut [f64; 6], marca: &mut u8) {
    crate::comparacao::ramo("cpt_6 dezembro");
    if (*marca as i8) < 0xc {
        crate::comparacao::ramo("cpt_6 bonus de dezembro");
        h[5] = read_i16(na + 0x8e) as i32 as f64 * f64_em(0x967888) * (0xc - *marca as i8 as i32) as f64 * f64_em(0x968930) + h[5]; // cpt6: o bônus de dezembro
    }
    h[0] = h[1];
    h[1] = h[2];
    h[2] = h[3];
    h[3] = h[4];
    h[4] = h[5];
    h[5] = 0.0; // cpt6: o ano corrente zerado
    *marca = 0;
}

/// the first time, each nation history gets the coefficient at +0xa8; on the table day of the month, the weighted
/// history sum in an array that is sorted and freed; on the December day, the current year gets reputation × 0.05 × (12
/// − mark) × 3 (mark < 12), the history moves one year and the current year and the mark are cleared
/// Original: `game_update_nation_coefficients` (0x5d5f20, game.cpp, cdecl).
///
/// [pt] { 0x5d5f20 } (cdecl, game.cpp): `game_update_nation_coefficients()`, todo dia pelo laço do dia do jogo (0x5cda10).
/// Na primeira vez ([0x9cd218]), a história de cada nação (+0xb0..+0xd0) recebe o coeficiente de +0xa8. No dia do
/// mês da tabela 0x9cd200 (palavra pelo mês 0x54cfb0): um vetor (malloc de 12 bytes por nação: a nação e a soma
/// 0,6·c0 + 0,4·b8 + 0,2·b0 + 0,8·c8 + d8 + d0), ordenado (qsort 0x945333 com 0x4afba0) e solto; no dia de dezembro
/// ([0x9cd216]) cada nação passa também pelo bônus e pelo deslocamento da história (`cpt6_dezembro`).
pub(crate) unsafe extern "C" fn coeficientes_das_nacoes() {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let qsort: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(FN_CRT_QSORT);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u8(0x9cd218) != 0 {
        if (read_usize(0xae2350) as i32) > 0 {
            let mut p = read_usize(0xae23a8) + 0xd0;
            let mut i = 0i16;
            loop {
                let v = read_f64(p - 0x28);
                for k in 0..5usize {
                    write_f64(p - k * 8, v);
                }
                i = i.wrapping_add(1);
                p += 0x122;
                if i as i32 >= read_usize(0xae2350) as i32 {
                    break;
                }
            }
        }
        *(0x9cd218 as *mut u8) = 0;
    }
    let hoje = read_usize(0xae2c90) as i16;
    if read_i16(0x9cd200 + (cpt6_mes() * 2) as usize) != hoje {
        return;
    }
    let vetor = malloc(read_usize(0xae2350).wrapping_mul(0xc));
    if vetor == 0 {
        aviso_em(0x9cd2a4, 0x156d);
        return;
    }
    let dez = read_i16(0x9cd216) == read_i16(0xae2c90); // cpt6: o dia de dezembro
    let mut n = read_usize(0xae2350) as i32;
    if n > 0 {
        let mut na = read_usize(0xae23a8);
        let mut i = 0i16;
        loop {
            let e = vetor + i as i32 as usize * 0xc;
            *(e as *mut usize) = na;
            let s = read_f64(na + 0xc0) * f64_em(0x968e48) + read_f64(na + 0xb8) * f64_em(0x968e00) + read_f64(na + 0xb0) * f64_em(0x968918)
                + read_f64(na + 0xc8) * f64_em(0x9689b0) + read_f64(na + 0xd8) + read_f64(na + 0xd0);
            write_f64(e + 4, s);
            if dez {
                let mut h = cpt6_historia(na);
                let mut marca = read_u8(na + 0x121);
                cpt6_dezembro(na, &mut h, &mut marca);
                for (k, x) in h.iter().enumerate() {
                    write_f64(na + 0xb0 + k * 8, *x);
                }
                *((na + 0x121) as *mut u8) = marca;
            }
            i = i.wrapping_add(1);
            na += 0x122;
            n = read_usize(0xae2350) as i32;
            if i as i32 >= n {
                break;
            }
        }
    }
    qsort(vetor, n as usize, 0xc, 0x4afba0);
    free(vetor);
}

/// Comparador: a original aloca e solta o vetor (e o ordena), então roda só ela, conferida contra o plano feito antes
/// pelas mesmas contas da cópia: a história (+0xb0..+0xd8) e a marca (+0x121) de cada nação e [0x9cd218] zerado.
pub(crate) unsafe extern "C" fn comparar_coeficientes_das_nacoes() {
    let o = orig::<unsafe extern "C" fn()>(1170);
    let iniciar = read_u8(0x9cd218) != 0;
    let hoje = read_usize(0xae2c90) as i16;
    let dez = read_i16(0x9cd200 + (cpt6_mes() * 2) as usize) == hoje && read_i16(0x9cd216) == hoje;
    let n = read_usize(0xae2350) as i32;
    let base = read_usize(0xae23a8);
    let mut plano = Vec::new();
    for k in 0..n.max(0) as usize {
        let na = base + k * 0x122;
        let mut h = cpt6_historia(na);
        if iniciar {
            let v = read_f64(na + 0xa8);
            for x in h.iter_mut().take(5) {
                *x = v;
            }
        }
        let mut marca = read_u8(na + 0x121);
        if dez {
            cpt6_dezembro(na, &mut h, &mut marca);
        }
        plano.push((h, marca));
    }
    o();
    let mut erro = None;
    for (k, (h, marca)) in plano.iter().enumerate() {
        let na = base + k * 0x122;
        let g = cpt6_historia(na);
        if (0..6).any(|j| g[j].to_bits() != h[j].to_bits()) || read_u8(na + 0x121) != *marca {
            erro = Some(format!("nação {} ({:x}): {:?}/{} x plano {:?}/{}", k, na, g, read_u8(na + 0x121), h, marca));
            break;
        }
    }
    if read_u8(0x9cd218) != 0 {
        erro = Some(format!("[0x9cd218] = {}", read_u8(0x9cd218)));
    }
    PLACAR.registrar(erro.is_none(), || format!("coeficientes_das_nacoes(dezembro {}): {}", dez, erro.clone().unwrap_or_default()));
}

/// 1 on days 0, 80, 173 and 250 of the year (the retirement scan days), otherwise 0
/// Original: `is_player_regen_day` (0x5d6250, game.cpp, cdecl).
///
/// [pt] { 0x5d6250 } (cdecl, game.cpp): `is_player_regen_day(dia)`: 1 nos dias 0, 80, 173 e 250 do ano (a busca das
/// aposentadorias), senão 0. Pelo laço do dia do jogo (0x5cda97, com a data inteira: o dia é a palavra baixa).
/// A original só devolve o al (o ah fica com o byte alto do dia).
pub(crate) unsafe extern "C" fn dia_das_aposentadorias(dia: u32) -> u32 {
    let d = dia as u16;
    (d == 0 || d == 0x50 || d == 0xad || d == 0xfa) as u32
}

/// Comparador: pelo al (o único byte que os chamadores leem).
pub(crate) unsafe extern "C" fn comparar_dia_das_aposentadorias(dia: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(u32) -> u32>(1171)(dia);
    let n = dia_das_aposentadorias(dia);
    PLACAR.registrar(r as u8 == n as u8, || format!("dia_das_aposentadorias({}) {} x {}", dia as u16, r as u8, n as u8));
    r
}

/// the sum of two signed bytes clamped between the given minimum and maximum
/// Original: `clamp_char_sum` (0x5d61d0, game.cpp, cdecl).
///
/// [pt] { 0x5d61d0 } (cdecl, game.cpp): `clamp_char_sum(a, b, min, max)`: a soma dos dois bytes com sinal (16 bits),
/// presa entre os outros dois. Devolve o ax da original: `min` ou `max` estendidos, ou o byte alto do `max`
/// estendido com a soma no al (os 26 chamadores, no diário dos jogadores, só gravam o al).
pub(crate) unsafe extern "C" fn soma_presa(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let s = (a as u8 as i8 as i16).wrapping_add(b as u8 as i8 as i16);
    let lo = min as u8 as i8 as i16;
    let hi = max as u8 as i8 as i16;
    if lo > s {
        lo as u16 as u32
    } else if hi < s {
        hi as u16 as u32
    } else {
        ((hi as u16 & 0xff00) | (s as u16 & 0xff)) as u32
    }
}

/// Comparador: pelo ax (os 16 bits altos do eax da original são os do chamador).
pub(crate) unsafe extern "C" fn comparar_soma_presa(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(1172)(a, b, min, max);
    let n = soma_presa(a, b, min, max);
    PLACAR.registrar(r as u16 == n as u16, || format!("soma_presa({}, {}, {}, {}) {:x} x {:x}", a as u8 as i8, b as u8 as i8, min as u8 as i8, max as u8 as i8, r as u16, n as u16));
    r
}

// ---------- 255a leva: as relações do elenco no dia a dia: as brigas e preferências entre colegas, o atrito com o técnico e o moral espalhado pela liderança (game.cpp) ----------


pub(crate) unsafe fn cps5_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}


pub(crate) unsafe fn cps5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// O clube de índice `i` (registros de 0x245 bytes em [0xae23b4]; a conta em 32 bits, como a original).
pub(crate) unsafe fn cps5_clube(i: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
pub(crate) unsafe fn cps5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O clube do contrato (pelo id em +4), ou 0 sem contrato.
pub(crate) unsafe fn cps5_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cps5_clube(read_usize(c + 4))
    }
}

/// O vínculo da pessoa com o clube (0x4cf5f0).
pub(crate) unsafe fn cps5_vinculo(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    f(read_usize(0xae1050), p, clube)
}

/// `pessoa` não gosta de `alvo` (0x547840)?
pub(crate) unsafe fn cps5_nao_gosta(alvo: usize, pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    f(alvo, pessoa)
}

/// O fim comum do atrito (0x5d6d10 e 0x5d7190): o primeiro colega do elenco (+0xd7, 50 vagas) de quem a pessoa não
/// gosta; achado, e só quando dia % 14 + id da pessoa dá zero (na prática a pessoa de id 0 a cada 14 dias), a queixa
/// (0x587c60) contra o técnico (+0xcf, tipo 0), o auxiliar (+0xd3, 1) ou outro (2) e a marca no vínculo `k` (0x867700
/// com 0x2000, 0x1000 ou 0x800).
pub(crate) unsafe fn cps5_atrito(p: usize, clube: usize, k: usize) {
    let mut j = 0usize;
    loop {
        if j >= 0x32 {
            return;
        }
        let q = read_usize(clube + 0xd7 + j * 4);
        if q != p && q != 0 && cps5_nao_gosta(q, p) != 0 {
            break;
        }
        j += 1;
    }
    let q = read_usize(clube + 0xd7 + j * 4);
    if q == 0 {
        return;
    }
    if ((read_i16(0xae2c90) as i32) % 0xe).wrapping_add(read_usize(p) as i32) != 0 {
        return;
    }
    crate::comparacao::ramo("cps_5 atrito com o tecnico");
    let queixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_NEWS_POST_DRESSING_ROOM_UNREST);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    let (tipo, marca) = if q == read_usize(clube + 0xcf) {
        (0, 0x2000)
    } else if q == read_usize(clube + 0xd3) {
        (1, 0x1000)
    } else {
        (2, 0x800)
    };
    queixa(p, q, tipo);
    marcar(k, marca, 0);
}

/// ten random pairs of team-mates: a disliked one stops being disliked and may fight (0x5888a0); otherwise becomes
/// disliked, liked or stops being liked; every 50 days, random clubs go through the dressing room friction; on the
/// other days each player, by attribute draws, may reach the friction with the manager
/// Original: `game_update_squad_relationships` (0x5d6d10, game.cpp, cdecl).
///
/// [pt] { 0x5d6d10 } (cdecl, game.cpp): `game_update_squad_relationships()`, todo dia pelo laço do dia do jogo. (1) Dez
/// vezes: um clube sorteado e duas vagas do elenco sorteadas (50); pessoas diferentes e sorteio(+0x5b da primeira) = 0:
/// se a primeira não gosta da segunda, deixa de não gostar (0x547c50) e, no mesmo clube de contrato e com o vínculo
/// marcado em +0x45 & 0x800, a briga (0x5888a0); senão, 1/3 passa a não gostar (0x547af0), 1/3 do resto passa a gostar
/// (0x547a10) e o resto deixa de gostar (0x547c10). (2) Dia múltiplo de 50: [0xae235c]/100 clubes sorteados passam
/// pelo atrito no vestiário (0x5d7190). Nos outros dias, cada pessoa (menos as 17 últimas) com ficha de jogador e
/// preferências, contrato e o vínculo do clube com a pessoa de +0xff: três sorteios (+0x5b, +0x5d, +0x5a, cada um + 5)
/// filtram; a soma de cinco atributos menos 3, 5, 15, 15 por sorteios zerados e 5 se a reputação do jogador (+0xb)
/// passa de 4500 e da do clube (+0x80) dá o sorteio que, zerado, leva ao atrito com o técnico.
pub(crate) unsafe extern "C" fn relacoes_do_elenco() {
    let tirar_rejeitada: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_STAFF_REMOVE_DISLIKED_STAFF);
    let briga: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_NEWS_POST_FEUD_SETTLED);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let favorecer: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_STAFF);
    let tirar_favorita: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_STAFF_REMOVE_FAVOURED_STAFF);
    for _ in 0..10 {
        let clube = cps5_clube(cps5_sortear(read_usize(0xae235c) as i32) as usize);
        let a = read_usize(clube.wrapping_add(cps5_sortear(0x32) as usize * 4) + 0xd7);
        let b = read_usize(clube.wrapping_add(cps5_sortear(0x32) as usize * 4) + 0xd7);
        if a == 0 || b == 0 || a == b {
            continue;
        }
        if cps5_sortear(cps5_sb(a + 0x5b)) != 0 {
            continue;
        }
        if cps5_nao_gosta(b, a) != 0 {
            tirar_rejeitada(a, b);
            let c = cps5_contrato(a);
            if c == 0 || cps5_clube_do_contrato(c) == 0 {
                continue;
            }
            let ca = cps5_clube_do_contrato(cps5_contrato(a));
            let cb = cps5_clube_do_contrato(cps5_contrato(b));
            if ca != cb {
                continue;
            }
            let k = cps5_vinculo(a, cps5_clube_do_contrato(cps5_contrato(a)));
            if k == 0 || read_usize(k + 0x45) & 0x800 == 0 {
                continue;
            }
            briga(a, b);
        } else if cps5_sortear(3) == 0 {
            rejeitar(a, b, 1);
        } else if cps5_sortear(3) == 0 {
            favorecer(a, b, 1);
        } else {
            tirar_favorita(a, b);
        }
    }
    if (read_i16(0xae2c90) as i32) % 0x32 == 0 {
        let atrito: unsafe extern "C" fn(usize) = core::mem::transmute(FN_GAME_CHECK_CLUB_DRESSING_ROOM_UNREST);
        let mut i = 0i32;
        while i < read_usize(0xae235c) as i32 / 100 {
            atrito(cps5_clube(cps5_sortear(read_usize(0xae235c) as i32) as usize));
            i += 1;
        }
        return;
    }
    let base = read_usize(0xae23bc);
    let fim = base.wrapping_add((read_usize(0xae2364).wrapping_sub(17)).wrapping_mul(0x6e));
    let mut p = base;
    while p < fim {
        cps5_pessoa_e_tecnico(p);
        p += 0x6e;
    }
}

/// Uma pessoa do laço de fora dos dias múltiplos de 50 (0x5d6f15..0x5d7115).
pub(crate) unsafe fn cps5_pessoa_e_tecnico(p: usize) {
    if read_usize(p + 0x61) == 0 || read_usize(p + 0x65) == 0 {
        return;
    }
    let c = cps5_contrato(p);
    if c == 0 {
        return;
    }
    let clube = cps5_clube_do_contrato(c);
    if clube == 0 {
        return;
    }
    let k = cps5_vinculo(read_usize(clube + 0xff), clube);
    if k == 0 {
        return;
    }
    if cps5_sortear(cps5_sb(p + 0x5b) + 5) != 0 && cps5_sortear(cps5_sb(p + 0x5d) + 5) != 0 && cps5_sortear(cps5_sb(p + 0x5a) + 5) != 0 {
        return;
    }
    let mut e = cps5_sb(p + 0x5b) + (cps5_sb(p + 0x5a) + cps5_sb(p + 0x59)) + (cps5_sb(p + 0x5c) + cps5_sb(p + 0x18));
    crate::comparacao::ramo("cps_5 tecnico sorteado");
    if cps5_sortear(cps5_sb(p + 0x59)) == 0 { // cps5: o primeiro desconto
        e -= 3;
    }
    if cps5_sortear(cps5_sb(p + 0x5a)) == 0 {
        e -= 5;
    }
    if read_usize(k + 0x45) != 0 || cps5_sortear(0x64) == 0 {
        if cps5_sortear(cps5_sb(p + 0x5b)) == 0 {
            e -= 0xf;
        }
        if cps5_sortear(cps5_sb(p + 0x5d)) == 0 {
            e -= 0xf;
        }
    }
    let rep = read_i16(read_usize(p + 0x61) + 0xb);
    if rep > 0x1194 && rep > read_i16(clube + 0x80) {
        e -= 5;
    }
    if cps5_sortear(e) >= 1 {
        return;
    }
    if read_usize(p + 0x65) == 0 {
        return;
    }
    cps5_atrito(p, clube, k);
}

// ponytail: sorteia e mexe nas preferências (0x547a10, 0x547af0, 0x547c10, 0x547c50), briga e queixa: só a original;
// a cópia leva os ramos (roda todo dia).
pub(crate) unsafe extern "C" fn comparar_relacoes_do_elenco() {
    orig::<unsafe extern "C" fn()>(1128)()
}

/// each squad player with a contract and preferences: the first team-mate he dislikes; with day % 14 + id = 0, the
/// complaint against the manager, the assistant or another (0x587c60) and the flag on the contract (0x867700)
/// Original: `game_check_club_dressing_room_unrest` (0x5d7190, game.cpp, cdecl).
///
/// [pt] { 0x5d7190 } (cdecl, game.cpp): `game_check_club_dressing_room_unrest(clube)`: cada pessoa do elenco (+0xd7, 50
/// vagas) com vínculo com o clube (0x4cf5f0) e preferências (+0x65) passa pelo atrito com o técnico (o primeiro colega
/// de quem não gosta; a queixa só com dia % 14 + id = 0). Pelas relações do elenco (0x5d6d10) nos dias múltiplos de 50.
pub(crate) unsafe extern "C" fn atrito_com_o_tecnico(clube: usize) {
    for i in 0..0x32usize {
        let p = read_usize(clube + 0xd7 + i * 4);
        if p == 0 {
            continue;
        }
        let k = cps5_vinculo(p, clube);
        if k == 0 {
            continue;
        }
        if read_usize(p + 0x65) == 0 {
            continue;
        }
        cps5_atrito(p, clube, k);
    }
}

// ponytail: a queixa (0x587c60) e a marca no vínculo (0x867700) não rodam duas vezes: só a original; a cópia leva o
// ramo do atrito.
pub(crate) unsafe extern "C" fn comparar_atrito_com_o_tecnico(clube: usize) {
    orig::<unsafe extern "C" fn(usize)>(1129)(clube)
}

/// A pessoa conta para a liderança do clube: id válido (< [0xae2364]), contrato com o clube (+0x39) e ficha de jogador.
pub(crate) unsafe fn cps5_do_elenco(p: usize, clube: usize) -> bool {
    p != 0 && (read_usize(p) as i32) < read_usize(0xae2364) as i32 && read_usize(p + 0x39) == clube && read_usize(p + 0x61) != 0
}

/// one club in seven each day (not national teams): squad leadership and leadership × morale summed; each player draws
/// from them and his own morale and the morale goes up or down by one (0x544710)
/// Original: `game_spread_squad_morale_by_leadership` (0x5ce0b0, game.cpp, cdecl).
///
/// [pt] { 0x5ce0b0 } (cdecl, game.cpp): `game_spread_squad_morale_by_leadership()`, todo dia pelo laço do dia do jogo. Cada
/// clube que não é seleção (0x53b900), com sorteio(7) = 0: soma a liderança (jogador +0x2f) e liderança × moral do
/// vínculo (+0x44) do elenco (+0xd7, 50 vagas); menos de 16 jogadores contam 10 cada um que falta. Com soma não nula,
/// cada jogador sorteia (2 × Σ liderança × moral) / (liderança dele + soma) mais sorteio(moral / 12), em byte com sinal:
/// negativo, sorteio(s) < sorteio(−5) baixa o moral (0x544710 com −1); positivo, sorteio(s) > sorteio(5) sobe (+1).
pub(crate) unsafe extern "C" fn moral_pela_lideranca() {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let moral: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let n = read_usize(0xae235c);
    let mut clube = read_usize(0xae23b4);
    let fim = clube.wrapping_add(n.wrapping_mul(0x245));
    while clube < fim {
        let vagas = clube + 0xd7;
        if e_selecao(clube) == 0 && cps5_sortear(7) == 0 {
            let (mut soma, mut lideranca, mut q) = (0i32, 0i32, 0i32);
            for i in 0..0x32usize {
                let p = read_usize(vagas + i * 4);
                if !cps5_do_elenco(p, clube) {
                    continue;
                }
                let c = cps5_vinculo(p, clube);
                if c == 0 {
                    continue;
                }
                let l = cps5_sb(read_usize(p + 0x61) + 0x2f);
                soma = soma.wrapping_add(cps5_sb(c + 0x44) * l);
                lideranca += l;
                q += 1;
            }
            if q < 0x10 {
                lideranca += (0x10 - q) * 10;
            }
            for i in 0..0x32usize {
                if lideranca == 0 {
                    break;
                }
                let p = read_usize(vagas + i * 4);
                if !cps5_do_elenco(p, clube) {
                    continue;
                }
                let c = cps5_vinculo(p, clube);
                let m = if c == 0 { 0 } else { cps5_sb(c + 0x44) };
                let l = cps5_sb(read_usize(p + 0x61) + 0x2f);
                crate::comparacao::ramo("cps_5 moral sorteado");
                let r1 = cps5_sortear(soma.wrapping_mul(2) / (l + lideranca));
                let r2 = cps5_sortear(m / 12); // cps5: a parte do moral
                let s = (r1 as u8).wrapping_add(r2 as u8) as i8;
                if s < 0 {
                    let e = cps5_sortear(s as i32);
                    if e < cps5_sortear(-5) {
                        moral(p, -1, 1);
                    }
                } else if s > 0 {
                    let e = cps5_sortear(s as i32);
                    if e > cps5_sortear(5) {
                        crate::comparacao::ramo("cps_5 moral sobe");
                        moral(p, 1, 1); // cps5: sobe o moral
                    }
                }
            }
        }
        clube = clube.wrapping_add(0x245);
    }
}

// ponytail: sorteia e muda o moral (0x544710, que também sorteia): só a original; a cópia leva os ramos.
pub(crate) unsafe extern "C" fn comparar_moral_pela_lideranca() {
    orig::<unsafe extern "C" fn()>(1130)()
}

// ---------- 255a leva: o dia do jogo: as faltas ao treino, as competições do dia e a confiança da diretoria no clube (game.cpp) ----------



pub(crate) unsafe fn cps6_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}



pub(crate) unsafe fn cps6_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
pub(crate) unsafe fn cps6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O clube do contrato (registros de 0x245 bytes em [0xae23b4], pelo id em +4), ou 0 sem contrato.
pub(crate) unsafe fn cps6_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        read_usize(0xae23b4).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
    }
}

/// Dias desde a data de +0x3e da pessoa até hoje (0x54cdb0 sobre a data do jogo 0xae2c90, a outra por valor).
pub(crate) unsafe fn cps6_dias_desde(p: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    f(0xae2c90, read_usize(p + 0x3e) as u32, read_usize(p + 0x42) as u32)
}

/// n/250 consecutive people from a random one: a present player, with training and without a serious injury misses
/// training (0x41a360) by a draw on the sum of attributes, morale and likes of the manager (more likely when newly
/// arrived) or by the contract flags
/// Original: `game_check_players_missing_training` (0x5d6aa0, game.cpp, cdecl).
///
/// [pt] { 0x5d6aa0 } (cdecl, game.cpp): `game_check_players_missing_training()`, todo dia pelo laço do dia do jogo. A partir
/// de uma pessoa sorteada (sorteio(n − n/250 − 1)), n/250 pessoas seguidas: jogador (+0x61, função +0x3d = 11) com
/// contrato e clube, presente (0x418f60), com o registro de treino (0x8b39f0, +0x15) e sem lesão grave (0x62c400 ≠ 2).
/// (1) Com +0x5b < 8, jogador +0x2f < 8 e a soma (jogador +0x2f, +0x18, +0x5b, vínculo +0x44, −5 se +0x5b ou o moral
/// valem 1, −5 se não gosta do técnico, +2 se gosta) < 43: sorteio(+0x5b + 100, 50 ou 25 conforme os dias desde +0x3e:
/// < 30, < 90, < 180) zerado é falta. (2) Senão, com +0x5b < 10 e vínculo com o clube: as marcas do vínculo (+0x45:
/// 0x8000, 0x30000; com +0x5b < 5, 0x7800) dão sorteios(+0x5b) que, zerados, são falta. A falta: 0x41a360.
pub(crate) unsafe extern "C" fn faltas_ao_treino() {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
    let treino: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_GET_PLAYER_SCHEDULE);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let falta: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_MISS_TRAINING);
    let n = read_usize(0xae2364) as i32;
    let q = n / 250;
    crate::comparacao::ramo("cps_6 faltas do dia");
    let r = cps6_sortear(n - q - 1); // cps6: a primeira pessoa
    let mut atual = read_usize(0xae23bc).wrapping_add((r as usize).wrapping_mul(0x6e));
    for _ in 0..q {
        let p = atual;
        atual = atual.wrapping_add(0x6e);
        if read_usize(p + 0x61) == 0 || read_u8(p + 0x3d) != 0xb {
            continue;
        }
        let c = cps6_contrato(p);
        if c == 0 {
            continue;
        }
        let clube = cps6_clube_do_contrato(c);
        if clube == 0 {
            continue;
        }
        if ausente(read_usize(0xad9c6c), p, 0) != 0 {
            continue;
        }
        let t = treino(read_usize(0xde1f44), p);
        if t == 0 || read_u8(t + 0x15) == 0 {
            continue;
        }
        if lesao(read_usize(0xb63d28), p) == 2 {
            continue;
        }
        let jog = read_usize(p + 0x61);
        let cl = read_u8(p + 0x5b);
        let dl = read_u8(jog + 0x44);
        let mut bl = read_u8(jog + 0x2f).wrapping_add(read_u8(p + 0x18)).wrapping_add(cl).wrapping_add(dl);
        if cl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if dl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if nao_gosta(read_usize(clube + 0xcf), p) != 0 {
            bl = bl.wrapping_add(0xfb);
        }
        if gosta(read_usize(clube + 0xcf), p) != 0 {
            bl = bl.wrapping_add(2);
        }
        let mut e_falta = false;
        let a = read_u8(p + 0x5b);
        if (a as i8) < 8 && (read_u8(read_usize(p + 0x61) + 0x2f) as i8) < 8 && (bl as i8) < 0x2b {
            let mut s = a;
            if cps6_dias_desde(p) < 0x1e {
                s = s.wrapping_add(0x64);
            } else if cps6_dias_desde(p) < 0x5a {
                s = s.wrapping_add(0x32);
            } else if cps6_dias_desde(p) < 0xb4 {
                s = s.wrapping_add(0x19);
            }
            crate::comparacao::ramo("cps_6 falta pela soma");
            if cps6_sortear(s as i8 as i32) == 0 { // cps6: o sorteio da soma
                e_falta = true;
            }
        }
        if !e_falta {
            if cps6_sb(p + 0x5b) >= 0xa {
                continue;
            }
            let clube = cps6_clube_do_contrato(cps6_contrato(p));
            let v = vinculo(read_usize(0xae1050), p, clube);
            if v == 0 {
                continue;
            }
            if read_usize(v + 0x45) & 0x8000 != 0 && cps6_sortear(cps6_sb(p + 0x5b)) == 0 {
                e_falta = true;
            } else if read_usize(v + 0x45) & 0x30000 != 0 && cps6_sortear(cps6_sb(p + 0x5b)) == 0 {
                e_falta = true;
            } else {
                let a = cps6_sb(p + 0x5b);
                if a >= 5 || read_usize(v + 0x45) & 0x7800 == 0 || cps6_sortear(a) != 0 {
                    continue;
                }
                e_falta = true;
            }
        }
        if e_falta {
            crate::comparacao::ramo("cps_6 falta ao treino");
            falta(read_usize(0xad9c6c), p); // cps6: a falta
        }
    }
}

// ponytail: sorteia e a falta (0x41a360) multa, sorteia e mexe na forma: só a original; a cópia leva os ramos.
pub(crate) unsafe extern "C" fn comparar_faltas_ao_treino() {
    orig::<unsafe extern "C" fn()>(1131)()
}

/// each competition in the list gets the day (virtual +0x14 with the game date and the flag); with the flag, the next
/// step of [0xae2a58] (0x5aaf00)
/// Original: `game_update_comps_for_date` (0x5ce060, game.cpp, cdecl).
///
/// [pt] { 0x5ce060 } (cdecl, game.cpp): `game_update_comps_for_date(marca)`: cada competição da lista ([0xadadfc], conta
/// [0xadadf8] relida a cada volta) recebe o dia (virtual +0x14 com a data do jogo 0xae2c90 e a marca); com a marca, o
/// passo seguinte do [0xae2a58] (0x5aaf00, que a original chama em salto de cauda). Pelo laço do dia do jogo (com 1).
pub(crate) unsafe extern "C" fn competicoes_do_dia(marca: u32) {
    let m = marca as u8 as i8;
    let mut i = 0i32;
    while i < read_usize(0xadadf8) as i32 {
        let c = read_usize(read_usize(0xadadfc) + i as usize * 4);
        if c != 0 {
            let dia: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(read_usize(read_usize(c) + 0x14));
            dia(c, 0xae2c90, m as i32);
        }
        i += 1;
    }
    if m != 0 {
        crate::comparacao::ramo("cps_6 competicoes do dia");
        let seguinte: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIXTURE_MANAGER_DAILY_UPDATE);
        seguinte(read_usize(0xae2a58));
    }
}

// ponytail: passa o dia de todas as competições (virtuais) e do [0xae2a58]: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "C" fn comparar_competicoes_do_dia(marca: u32) {
    orig::<unsafe extern "C" fn(u32)>(1132)(marca)
}

/// A confiança da diretoria no clube: a palavra +6 da linha do clube (73 bytes por id em [this]).
pub(crate) unsafe fn cps6_confianca(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49)) + 6
}

/// adds the delta to the board confidence in the club (+6 of the 73-byte club row), held to 1..10000
/// Original: `manager_manager_adjust_board_confidence` (0x5cdde0, game.cpp, thiscall, ret 8).
///
/// [pt] { 0x5cdde0 } (thiscall, ret 8, game.cpp): `manager_manager_adjust_board_confidence(clube, delta)`: soma `delta`
/// (palavra) à confiança da diretoria no clube (+6 da linha do MANAGER_MANAGER [0xb67a44]), presa em 1..10000. Clube
/// nulo: nada. Pelo laço do dia do jogo (com −5).
pub(crate) unsafe extern "thiscall" fn mudar_confianca_da_diretoria(this: usize, clube: usize, delta: u32) {
    if clube == 0 {
        return;
    }
    let c = cps6_confianca(this, clube);
    write_i16(c, read_i16(c).wrapping_add(delta as u16 as i16));
    let v = read_i16(c);
    if v < 1 {
        write_i16(c, 1);
    } else if v > 0x2710 {
        write_i16(c, 0x2710);
    }
}

/// Comparador: lado a lado pela confiança (2 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_mudar_confianca_da_diretoria(this: usize, clube: usize, delta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1133);
    if clube == 0 {
        return o(this, clube, delta);
    }
    let c = cps6_confianca(this, clube);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 2)], || o(this, clube, delta), || mudar_confianca_da_diretoria(this, clube, delta));
    PLACAR.registrar(igual, || format!("mudar_confianca_da_diretoria(clube {:x}, {}) {:x?} x {:x?}", clube, delta as i32, mo, mr));
}

// ---------- 188a leva: a lista de observados do gerente de transferências, a entrada do treino e a carga de treino (game.cpp, injury.cpp) ----------

/// the shortlist manager kept in the transfer manager
/// Original: `transfer_manager_get_shortlist_manager` (0x5d5ab0, game.cpp, thiscall).
///
/// [pt] { 0x5d5ab0 } (thiscall, game.cpp): o gerente da lista de observados guardado em +0x856 do gerente
/// de transferências.
pub(crate) unsafe extern "thiscall" fn lista_de_observados(gerente: usize) -> usize {
    core::ptr::read_unaligned((gerente + 0x856) as *const usize)
}

/// the player's 6-byte entry in the training table
/// Original: `training_manager_get_player_entry` (0x5d5ac0, game.cpp, thiscall).
///
/// [pt] { 0x5d5ac0 } (thiscall, ret 4, game.cpp): a entrada de 6 bytes do jogador na tabela do treino
/// ([treino+0x651d]), pelo id dos dados do jogador ([[jogador+0x61]]).
pub(crate) unsafe extern "thiscall" fn entrada_do_treino(treino: usize, jogador: usize) -> usize {
    let tabela = core::ptr::read_unaligned((treino + 0x651d) as *const usize);
    let id = *(core::ptr::read_unaligned((jogador + 0x61) as *const usize) as *const i32);
    (tabela as isize + id as isize * 6) as usize
}

/// O byte da carga de treino do jogador `*id` no registro de condição (31 bytes por jogador, +0x1e).
pub(crate) unsafe fn byte_da_carga(lesoes: usize, id: *const i32) -> *mut u8 {
    (*(lesoes as *const usize) as isize + *id as isize * 31 + 0x1e) as *mut u8
}

/// stores the player's training load in the fitness record
/// Original: `Injury::set_training_load` (0x62c660, injury.cpp, thiscall).
///
/// [pt] { 0x62c660 } (thiscall, ret 8, injury.cpp): grava a carga de treino `valor` do jogador `*id`.
pub(crate) unsafe extern "thiscall" fn gravar_carga_de_treino(lesoes: usize, id: *const i32, valor: u8) {
    *byte_da_carga(lesoes, id) = valor;
}

pub(crate) unsafe extern "thiscall" fn comparar_lista_de_observados(gerente: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> usize>(436)(gerente);
    let n = lista_de_observados(gerente);
    PLACAR.registrar(r == n, || format!("lista_de_observados original={:#x} rust={:#x}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_entrada_do_treino(treino: usize, jogador: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(437)(treino, jogador);
    let n = entrada_do_treino(treino, jogador);
    PLACAR.registrar(r == n, || format!("entrada_do_treino original={:#x} rust={:#x}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_carga_de_treino(lesoes: usize, id: *const i32, valor: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, u8)>(438);
    let a = byte_da_carga(lesoes, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(lesoes, id, valor), || gravar_carga_de_treino(lesoes, id, valor));
    PLACAR.registrar(igual, || format!("gravar_carga_de_treino({}, {}) {:x?} x {:x?}", *id, valor, mo, mr));
}

// ---------- 178a leva: a media do jogo, o valor do contrato e o nivel para a selecao ----------

/// the f64 average +0x20/+0x28 or +0x10/+0x18 (zero divisor: 6.8)
/// Original: `game_average` (0x5d5a60, game.cpp, thiscall).
///
/// [pt] { 0x5d5a60 } (thiscall, ret 4, game.cpp): a média +0x20/+0x28 (com `segunda`) ou +0x10/+0x18 (f64,
/// como a original deixa no x87); com divisor zero (ou não comparável), 6.8.
pub(crate) unsafe extern "thiscall" fn media_do_jogo(t: usize, segunda: u8) -> f64 {
    let d = |o: usize| core::ptr::read_unaligned((t + o) as *const f64);
    let (n, q) = if segunda != 0 { (d(0x20), d(0x28)) } else { (d(0x10), d(0x18)) };
    if q == f64_em(0x9689a0) || q.is_nan() {
        return f64_em(0x968998);
    }
    n / q
}

/// dword +0xc of the person's contract record
/// Original: `contract_value` (0x4d0c20, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0c20 } (thiscall, ret 4, contract_manager.cpp): o dword +0xc da ficha de contrato (0x50 bytes
/// a partir de [this]) da pessoa `*p`, pelo índice em +0 do registro de 79 bytes em [0xae2d0c]; 0 sem
/// pessoa, com id fora de [0xae2364], índice negativo ou ficha no endereço 0.
pub(crate) unsafe extern "thiscall" fn valor_do_contrato(t: usize, p: *const i32) -> u32 {
    if p.is_null() || *p >= *(0xae2364 as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(0xae2d0c as *const usize)).wrapping_add((*p as usize).wrapping_mul(79)) as *const i32);
    if k < 0 {
        return 0;
    }
    let ficha = (*(t as *const usize)).wrapping_add(k as usize * 0x50);
    if ficha == 0 {
        return 0;
    }
    core::ptr::read_unaligned((ficha + 0xc) as *const u32)
}

/// the player's level 1..3 for the national team by reputation and position ratings
/// Original: `national_team_level` (0x76d810, national_teams.cpp, stdcall).
///
/// [pt] { 0x76d810 } (stdcall, ret 0xc, national_teams.cpp): o nível 1..3 do jogador `j` para a seleção
/// (0 com reputação `*rep` abaixo de 100, ou 50 com `base`, ou goleiro de nota 15): 1 para líbero ou
/// zagueiro, 2 para volante, meia ou meia-atacante (nota 15 em +0x10..+0x14 da ficha), senão 3.
/// Devolve só `al`.
pub(crate) unsafe extern "stdcall" fn nivel_para_a_selecao(j: usize, rep: *const i16, base: u8) -> u8 {
    if *rep < if base != 0 { 0x32 } else { 0x64 } {
        return 0;
    }
    let ficha = core::ptr::read_unaligned((j + 0x61) as *const usize);
    let n = |o: usize| *((ficha + o) as *const i8) >= 0xf;
    if n(0xf) {
        0
    } else if n(0x10) || n(0x11) {
        1
    } else if n(0x12) || n(0x13) || n(0x14) {
        2
    } else {
        3
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_media_do_jogo(t: usize, segunda: u8) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8) -> f64>(406)(t, segunda);
    let n = media_do_jogo(t, segunda);
    PLACAR.registrar(o.to_bits() == n.to_bits(), || format!("media_do_jogo({:#x}, {}) original={} rust={}", t, segunda, o, n));
    o
}

comparar_valor!(comparar_valor_do_contrato, valor_do_contrato, 407, (p: *const i32) -> u32);

pub(crate) unsafe extern "stdcall" fn comparar_nivel_para_a_selecao(j: usize, rep: *const i16, base: u8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8>(408)(j, rep, base);
    let n = nivel_para_a_selecao(j, rep, base);
    PLACAR.registrar(r == n, || format!("nivel_para_a_selecao({:#x}, {}, {}) original={} rust={}", j, *rep, base, r, n));
    r
}

