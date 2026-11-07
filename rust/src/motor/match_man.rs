// ---------- centesima decima setima leva: escalar (0x6e2000) ----------

/// { 0x6e2000 } (cdecl, match_man.cpp, ~3,9 KB, recursiva): distribui as `n` pessoas de `pessoas`
/// pelas `m` vagas (+ `x` extras) e grava em `saida` a pessoa de cada vaga. Cada par (pessoa, vaga)
/// vale a nota da 0x6e2f20 (codigo da vaga `codigos`, `marcas`, `bytes`, `curtos`, modo, clube,
/// comissao...). Com menos pessoas que vagas, cada pessoa livre procura a vaga onde rende mais do que
/// o ocupante (ate 10000 voltas) e no fim saem as vagas pares com +0xf da ficha 20 e as impares com
/// +0xf abaixo de 10. Com mais pessoas, cada vaga pega a melhor pessoa e os repetidos sao resolvidos
/// trocando pela pessoa de menor perda (ate 10000 voltas). As extras recebem, pela propria 0x6e2000
/// (modo | 0x200), as pessoas que sobraram. Por fim troca pares de vagas enquanto a soma melhorar (ate
/// 100 voltas). A memoria de trabalho e a do jogo (0x945501/0x9452ca), na mesma ordem. Devolve 1, ou 0
/// sem pessoas ou sem vagas.
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn escalar(
    pessoas: usize, a2: u32, codigos: usize, a4: u32, a5: u32, saida: usize, modo: u32, clube: usize, comissao: usize,
    a10: u32, _a11: u32, mut marcas: usize, mut bytes: usize, a14: u32, mut curtos: usize,
) -> u8 {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let nota_fn: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let recursiva: Escalar = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let tecnico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_GET_SAVED_LINEUP);
    let n = a2 as u8 as i8 as i32;
    let m = a4 as u8 as i8 as i32;
    let x = a5 as u8 as i8 as i32;
    let tot = m + x;
    let pes = |k: i32| *((pessoas + 4 * k as usize) as *const usize);
    let sai = |i: i32| (saida + 4 * i as usize) as *mut usize;
    let (mut al12, mut al13, mut al15) = (false, false, false);
    if marcas == 0 {
        marcas = if tot != 0 { calloc(tot as u32, 8) } else { 0 };
        al12 = true;
    }
    if bytes == 0 {
        bytes = if tot != 0 { calloc(tot as u32, 1) } else { 0 };
        al13 = true;
    }
    if curtos == 0 {
        curtos = if tot != 0 { calloc(tot as u32, 2) } else { 0 };
        al15 = true;
    }
    let liberar_param = |free: unsafe extern "C" fn(usize)| {
        if al13 && bytes != 0 {
            free(bytes);
        }
        if al12 && marcas != 0 {
            free(marcas);
        }
        if al15 && curtos != 0 {
            free(curtos);
        }
    };
    if n < 1 {
        for i in 0..tot {
            *sai(i) = 0;
        }
        liberar_param(free);
        return 0;
    }
    let jog = calloc(n as u32, 4);
    let jg = |k: i32| *((jog + 4 * k as usize) as *const usize);
    if comissao != 0 {
        for k in 0..n {
            let mut j = *((comissao + 0x23) as *const usize);
            let mut achou = 0usize;
            for _ in 0..20 {
                if *((j + 0x69) as *const usize) == pes(k) {
                    achou = j;
                    break;
                }
                j += 0x1be;
            }
            *((jog + 4 * k as usize) as *mut usize) = achou;
        }
    }
    let mut lc = 0u32;
    if modo & 0x400000 != 0 && clube != 0 {
        let p = *((clube + 0xcf) as *const usize);
        let q = *(0xae2364 as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
            lc = tecnico(*(0xb63c98 as *const usize), p);
        }
    }
    let usado = calloc(n as u32, 1);
    let us = |k: i32| (usado + k as usize) as *mut i8;
    for i in 0..tot {
        *sai(i) = 0;
    }
    if m == 0 && x == 0 {
        free(usado);
        liberar_param(free);
        free(jog);
        return 0;
    }
    // a nota de `pessoa` (com o jogador `j`) na vaga `i`
    let nota = |pessoa: usize, j: usize, i: i32| -> i32 {
        let i = i as usize;
        nota_fn(
            pessoa, *((pessoa + 0x61) as *const usize), j, *((codigos + 2 * i) as *const u16) as u32, modo, clube, comissao, a10, u32::MAX,
            *((marcas + 8 * i) as *const u32), *((marcas + 8 * i + 4) as *const u32), *((bytes + i) as *const u8) as u32, a14,
            *((curtos + 2 * i) as *const u16) as u32, lc,
        )
    };
    let soltar = |alvo: usize| {
        for kk in 0..n {
            if pes(kk) == alvo {
                *us(kk) = 0;
            }
        }
    };
    if n < m {
        // menos pessoas que vagas: cada pessoa livre procura a melhor vaga
        let mut colocadas = 0i8;
        let mut voltas = 0;
        while (colocadas as i32) < n && voltas < 10000 {
            voltas += 1;
            for k in 0..n {
                if *us(k) != 0 {
                    continue;
                }
                let cand = pes(k);
                if cand == 0 || *((cand + 0x61) as *const usize) == 0 {
                    continue;
                }
                let (mut melhor, mut melhor_nota) = (-1i32, 0i32);
                for i in 0..m {
                    let r = nota(cand, jg(k), i);
                    if melhor == -1 || r > melhor_nota {
                        let ocupante = *sai(i);
                        if ocupante == 0 || nota(ocupante, jg(k), i) < r {
                            melhor = i;
                            melhor_nota = r;
                        }
                    }
                }
                if melhor > -1 {
                    let ocupante = *sai(melhor);
                    if ocupante != 0 {
                        for kk in 0..n {
                            if pes(kk) == *sai(melhor) {
                                *us(kk) = 0;
                                colocadas = colocadas.wrapping_sub(1);
                            }
                        }
                    }
                    *sai(melhor) = cand;
                    colocadas = colocadas.wrapping_add(1);
                    *us(k) = 1;
                }
            }
        }
        for i in 0..m {
            let ocupante = *sai(i);
            if ocupante == 0 {
                continue;
            }
            let f = byte(*((ocupante + 0x61) as *const usize), 0xf);
            let sair = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 10 } else { f == 20 };
            if sair {
                soltar(ocupante);
                *sai(i) = 0;
            }
        }
    } else {
        // mais pessoas: cada vaga pega a melhor; os repetidos sao resolvidos depois
        let mut conflitos = 0i8;
        let mut melhor_nota = 0i32;
        for i in 0..m {
            let (mut melhor, mut mk) = (0usize, 0i32);
            for k in 0..n {
                let cand = pes(k);
                if cand == 0 || *((cand + 0x61) as *const usize) == 0 {
                    continue;
                }
                let r = nota(cand, jg(k), i);
                if melhor == 0 || r > melhor_nota {
                    melhor = cand;
                    melhor_nota = r;
                    mk = k;
                }
            }
            if melhor != 0 {
                *us(mk) = (*us(mk)).wrapping_add(1);
                if *us(mk) > 1 {
                    conflitos = conflitos.wrapping_add(1);
                }
            }
            *sai(i) = melhor;
        }
        let mut menor_perda = 10000i32;
        let mut voltas = 0;
        let mut k2_atual = 0i32;
        while conflitos > 0 && voltas < 10000 {
            voltas += 1;
            let (mut vaga_t, mut k2_t) = (-1i32, -1i32);
            for k in 0..n {
                let mut troca = 0usize;
                let cand = pes(k);
                if m > 0 {
                    for j in 0..m {
                        if cand == 0 || *us(k) <= 1 {
                            break;
                        }
                        if cand != *sai(j) {
                            continue;
                        }
                        let r_atual = nota(cand, jg(k), j);
                        let (mut outra, mut outra_nota) = (0usize, 0i32);
                        for k2 in 0..n {
                            let o = pes(k2);
                            if o == 0 || *((o + 0x61) as *const usize) == 0 || o == cand {
                                continue;
                            }
                            let r = nota(o, jg(k2), j);
                            if (outra == 0 || r > outra_nota) && (r < r_atual || *us(k2) == 0) {
                                outra_nota = r;
                                outra = o;
                                k2_atual = k2;
                            }
                        }
                        if troca == 0 || r_atual - outra_nota < menor_perda {
                            vaga_t = j;
                            troca = outra;
                            k2_t = k2_atual;
                            menor_perda = (r_atual - outra_nota) as i16 as i32;
                        }
                    }
                    if troca != 0 {
                        *us(k2_t) = (*us(k2_t)).wrapping_add(1);
                        if *us(k2_t) > 1 {
                            conflitos = conflitos.wrapping_add(1);
                        }
                        *sai(vaga_t) = troca;
                        *us(k) = (*us(k)).wrapping_sub(1);
                        conflitos = conflitos.wrapping_sub(1);
                    }
                }
            }
        }
    }
    // as extras: as pessoas que sobraram, pela propria 0x6e2000
    if x != 0 {
        let mut resto = [0usize; 128];
        let mut c = 0usize;
        for k in 0..n {
            if *us(k) == 0 {
                resto[c] = pes(k);
                c += 1;
            }
        }
        recursiva(
            resto.as_ptr() as usize, c as u32, codigos + 2 * m as usize, x as u32, 0, saida + 4 * m as usize, modo | 0x200, clube, comissao, a10, 0,
            marcas, bytes, a14, curtos + 2 * m as usize,
        );
    }
    // trocas de pares enquanto a soma melhorar
    let jogador_de = |p: usize| -> usize {
        if comissao == 0 {
            return 0;
        }
        let mut j = *((comissao + 0x23) as *const usize);
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == p {
                return j;
            }
            j += 0x1be;
        }
        0
    };
    let mut voltas = 0;
    loop {
        let mut mudou = false;
        let mut i = 0;
        while i < m && !mudou {
            let a = *sai(i);
            if a != 0 && *((a + 0x61) as *const usize) != 0 {
                let ja = jogador_de(a);
                let a_i = nota(a, ja, i);
                let mut j = i + 1;
                while j < tot && !mudou {
                    let b = *sai(j);
                    if b != 0 && *((b + 0x61) as *const usize) != 0 {
                        let jb = jogador_de(b);
                        let b_i = nota(b, jb, i);
                        let trocar = if j < m {
                            let a_j = nota(a, ja, j);
                            let b_j = nota(b, jb, j);
                            b_j.wrapping_add(a_i) < a_j.wrapping_add(b_i)
                        } else {
                            a_i < b_i
                        };
                        if trocar {
                            mudou = true;
                            let t = *sai(j);
                            *sai(j) = *sai(i);
                            *sai(i) = t;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        voltas += 1;
        if !mudou || voltas >= 100 {
            break;
        }
    }
    free(usado);
    free(jog);
    liberar_param(free);
    1
}

// ---------- centesima decima oitava leva: a postura do time (0x6df460) ----------

/// the stance the staff ask for (1 all-out attack … 5 park the bus). Starts from the sides' strength (with the engine
/// running, +0x8ea5, the sum of the 0x6e86d0 scores of those on the pitch × 50 / 11; otherwise the +0x80 reputation),
/// possession (+0x1fc/+0x206/+0x210), time (+0x8ed0), sendings-off (+0x1ad, 0x6cbbb0) and the aggregate score
/// (0x6ce310); adjusts for the league (table positions through 0x4a4850 and dates through 0x54cdb0), the manager (+0xe)
/// and the end of the match (+0x8ed4 = 0x3de). Stores the factor in engine+0x476a[side] (10 and −1 at the extremes);
/// with the engine running and the factor within 0.5 of the stored one, keeps the current stance (+0x1ab)
/// Original: `stance` (0x6df460, match_man.cpp (~3 KB), thiscall).
///
/// [pt] { 0x6df460 } (thiscall, match_man.cpp): a postura que a comissao `c` (+0x12 motor, +0x18 lado,
/// +0x1f tecnico, +0x2f clube, +0x33 adversario, +0x37 partida) pede ao time: 1 ataque total, 2
/// ataque, 3 equilibrio, 4 defesa, 5 retranca. Parte da forca dos dois lados (com o motor rodando
/// (+0x8ea5), a soma das notas 0x6e86d0 dos que estao em campo * 50 / 11; senao a reputacao +0x80
/// dos clubes), da posse (+0x1fc/+0x206/+0x210), do tempo (+0x8ed0), dos expulsos (+0x1ad) e do
/// placar agregado (0x6ce310), e ajusta pela liga (posicoes na tabela, 0x4a4850), pelo tecnico e pelo
/// fim do jogo (+0x8ed4 = 0x3de). Guarda o fator em motor+0x476a[lado] (10 e -1 nos extremos); com o
/// motor rodando e o fator a menos de 0,5 do guardado, mantem a postura atual (+0x1ab).
unsafe extern "thiscall" fn postura(c: usize) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let i = |p: usize, o: usize| *((p + o) as *const i32);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let m = u(c, 0x12);
    let lado = byte(c, 0x18);
    let lu = lado as isize as usize;
    let (clube_a, clube_b) = (u(c, 0x2f), u(c, 0x33));
    let (rep_a, rep_b) = (w(clube_a, 0x80), w(clube_b, 0x80));
    let rodando = byte(m, 0x8ea5) != 0;
    // a forca de cada lado (so os 16 bits contam, como no original)
    let (mut proprio, mut outro) = (rep_a, rep_b);
    if rodando {
        let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
        let mut soma = [0i32; 2];
        for (l, s) in soma.iter_mut().enumerate() {
            for k in 0..20 {
                let j = m + 0x22d8 * l + 0x4796 + 0x1be * k;
                if byte(j, 0x19) >= 0 {
                    *s = s.wrapping_add(nota(u(c, 0x1b), u(j, 0x69), clube_a, 2, u32::MAX) as i32);
                }
            }
        }
        let (a, b) = if lado == 0 { (soma[0], soma[1]) } else { (soma[1], soma[0]) };
        proprio = a.wrapping_mul(50) / 11;
        outro = b.wrapping_mul(50) / 11;
    }
    let (mut x11, mut x12, mut x14, mut x16, mut x17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    let agregado: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> i8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOAL_MARGIN);
    agregado(m, lado, &mut x11, &mut x12, &mut x16, &mut x17, &mut x14, 0);
    let proprio = proprio as i16 as i32;
    let outro = outro as i16 as i32;
    let f = (proprio - outro) as f64 / ((1.5 - proprio as f64 * 0.0001) * 6000.0) + 1.0;
    // a posse
    let (p1, p2, p3) = (w(m, 0x1fc), w(m, 0x206), w(m, 0x210));
    let tot = p1 + p2 + p3;
    let posse_b = if tot != 0 { (p3 * 100 / tot) as i8 } else { 0 };
    let posse_a = if tot != 0 { (p1 * 100 / tot) as i8 } else { 0 };
    let r = if lado == 0 && posse_a != 0 {
        Some(posse_b as f64 / posse_a as f64)
    } else if lado == 1 && posse_b != 0 {
        Some(posse_a as f64 / posse_b as f64)
    } else {
        None
    };
    let r = match r {
        Some(q) if q > 1.0 => q + q - 1.0,
        Some(q) => q * 1.29 - 0.29,
        None => f,
    };
    let r = if 0.05 > r { 0.05 } else if 6.0 < r { 6.0 } else { r };
    let mut t = ((w(m, 0x8ed0) << 2) / 99) as i8;
    if t > 20 {
        t = 20;
    }
    let t = t as f64;
    let k = byte(m, 0x1ad + lu);
    let g = f * (25.0 - t) + t * r;
    let mut h = g * 0.04 * (1.0 - k as f64 * 0.07);
    if !(h > 0.0) {
        h = 0.01;
    }
    if x14 != 0 {
        if lado == 1 && x11 != 0 {
            x11 = x11.wrapping_add(2);
        } else if lado == 0 {
            if x12 != 0 {
                x12 = x12.wrapping_add(2);
            } else {
                x11 = x11.wrapping_add(1);
            }
        }
    }
    // os expulsos: 0x6cbbb0 conta os ativos de cada lado
    let mut x13 = 0i8;
    if rodando {
        let contar: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_PLAYERS_ON_PITCH);
        let b = contar(m, (lado == 0) as u32);
        let a = contar(m, lado as u8 as u32);
        x13 = (a as i8).wrapping_sub(b as i8);
        if x13 != 0 {
            x11 = x11.wrapping_add(x13.wrapping_mul(-2));
        }
    }
    let tt = (w(m, 0x8ed0) / 11) as f64;
    if x14 == 0 && x17 == 0 && x11 as i32 + x12 as i32 == 0 {
        let (d, e) = (byte(m, 0x1ad + (lado == 0) as usize), byte(m, 0x1ad + lu));
        if d > e {
            h *= tt * 0.014925373134328358 + 1.25;
        } else if d < e {
            h *= 0.75;
        }
    }
    h *= 1.0 - (x11 as i32 - x12 as i32) as f64 * 0.1;
    if u(clube_a, 0x53) != u(clube_b, 0x53) {
        h *= 0.9;
    } else {
        let jogo = u(c, 0x37);
        let p14 = u(jogo, 0x14);
        if p14 != 0 {
            h += h;
            let comp = if p14 == u(clube_a, 0x57) { u(u(0xadadfc, 0), 4 * u(jogo, 0)) } else { 0 };
            if comp != 0 && u(comp, 8) != 0 {
                // a liga: quem esta acima na tabela e quanto falta
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
                let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
                let e = busca(u(comp, 8), clube_a);
                if e != 0 {
                    let mut x18 = 0f32;
                    let mut feito = false;
                    if i(e, 0x55) > 1 && dias(0xae2c90, u(e, 0x61) as u32, u(e, 0x65) as u32) < 30 {
                        let (a, b) = (i(e, 0x55).wrapping_neg(), i(e, 0x2d).wrapping_neg().wrapping_shl(1));
                        x18 = (if a < b { a } else { b }) as f32;
                        feito = true;
                    }
                    if !feito && i(e, 0x41) > 1 && dias(0xae2c90, u(e, 0x4d) as u32, u(e, 0x51) as u32) < 10 {
                        let (a, b) = (i(e, 0x41), i(e, 0x19).wrapping_shl(1));
                        x18 = (if a > b { a } else { b }) as f32;
                    }
                    x18 = ((x18 as f64).abs() * x18 as f64) as f32;
                    if k == 0 && 0.0f32 > x18 {
                        x18 = 0.0;
                    }
                    let mut base = h;
                    let pe = *((e + 0x72) as *const u8);
                    if pe < 5 && (x11 as i32 - 3) < x12 as i32 {
                        let e2 = busca(u(u(u(0xadadfc, 0), 4 * u(jogo, 0)), 8), clube_b);
                        if e2 != 0 && *((e2 + 0x72) as *const u8) > pe {
                            let comp = u(u(0xadadfc, 0), 4 * u(jogo, 0));
                            let cx = w(comp, 0x3e);
                            let mut x30 = if cx > 0 { 22.0 / cx as f64 } else { 1.0 };
                            let ax = w(u(jogo, 0x14), 0x69);
                            if ax >= 10 {
                                let lim = ax * 500;
                                if rep_a > lim {
                                    x30 *= ((rep_a - lim) as f64 + (rep_a - 5000) as f64) * 0.0002 + 1.0;
                                }
                            }
                            let p = pow_do_jogo(x30, 1.5) * (*((e2 + 0x72) as *const u8) as i32 - pe as i32) as f64;
                            let p3 = p * p * p;
                            let q = p3 / ((k as i32 * k as i32) as f64 * 50.0 + 250.0) + 1.0;
                            let q = if 1.0 > q { 1.0 } else if 10.0 < q { 10.0 } else { q };
                            base = q * h;
                        }
                    }
                    let z = x18 as f64 * 0.013333333333333334 + 1.0;
                    let z = if 0.25 > z { 0.25 } else if 5.0 < z { 5.0 } else { z };
                    h = base * z;
                }
            }
        }
    }
    let soma = x11 as i32 + x12 as i32;
    if soma > 6 {
        h *= 0.825;
    } else if soma > 4 {
        h *= 0.875;
    }
    let kb = k;
    if kb != 0 || x13 < 0 {
        let pressao = (rep_b > 7500 && rep_b - rep_a > 2000) || (rep_b > 8500 && rep_b - rep_a > 750) || (rep_b > 8000 && x11 as i32 - 1 > x12 as i32);
        if pressao {
            let d = x12 as i32 - x11 as i32;
            if (d < 2 && w(m, 0x8ed4) == 0x1ef) || x12 <= x11 {
                h *= 0.3;
            }
        }
    }
    let d = x11 as i32 - x12 as i32;
    h *= if kb == 0 { 1.0 - d as f64 * 0.1 } else { 0.967 - (kb as i32 + 2 * d) as f64 * 0.067 };
    if x11 > x12 || (x11 == x12 && (x16 != 0 || lado == 1)) {
        let a = if 1.0 > tt { 1.0 } else { tt };
        h *= if 1.0 < 30.0 / a { 1.0 } else { 30.0 / a };
    } else if x11 < x12 || kb == 0 {
        let v = tt * 0.03333333333333333;
        h *= if 1.0 > v { 1.0 } else { v };
    }
    let d2 = x12 as i32 - x11 as i32;
    if d2 > 2 && d2 < 6 {
        h *= 1.01 - d2 as f64 * 0.2;
    }
    let ra_maior = w(clube_a, 0x80) > w(clube_b, 0x80);
    if ((soma < 5 && x13 > -1 && x11 == x12 && x14 == 0 && ra_maior) || x13 > 0) && !(h > 1.0) {
        h = 1.0;
    }
    if soma < 4 && x13 > -1 && x11 as i32 - 2 >= x12 as i32 && ra_maior && !(h > 1.15) {
        h = 1.15;
    }
    let tecnico = u(c, 0x1f);
    if tecnico != 0 && x13 > -1 {
        let e = byte(tecnico, 0xe) as i32;
        let mut f1 = (1.0 + (e - 10) as f64 * 0.06666666666666667) * h;
        let e2 = pow_do_jogo(e as f64, 2.0);
        if f1 < e2 * 0.005 {
            f1 = e2 * 0.0025 + f1 * 0.5;
        }
        let hm1 = h - 1.0;
        h = if f1 < hm1 { hm1 } else { f1 };
    }
    if (rep_b > 7500 || x13 < 0) && kb != 0 && d.abs() < 2 && (x11 >= x12 || tt < 75.0 || soma > 3) {
        let g = tt * 0.02857142857142857 + 0.2;
        if !(h < g) {
            h = g;
        }
    }
    if x17 != 0 && x14 == 0 && x16 == 0 && (x11 == x12 || x11 as i32 + 1 == x12 as i32) && kb == 0 {
        let g = (if tt < 45.0 { tt } else { 45.0 }) * 0.02857142857142857 + 0.2;
        if !(h > g) {
            h = g;
        }
    }
    if x12 > 1 && x11 > 1 {
        let cai = if x14 == 0 && x17 != 0 { (tt + 90.0) / soma as f64 } else { tt / soma as f64 };
        if cai < 18.0 {
            h *= 0.3;
        }
    }
    let fator = (m + 0x476a + 4 * lu) as *mut f32;
    let tempo = w(m, 0x8ed4);
    if tt > 85.0 - h * 10.0 && tempo == 0x3de && (x11 as i32 + 1 == x12 as i32 || (x17 != 0 && x14 == 0 && x16 == 0 && x11 == x12)) {
        *fator = 10.0;
        return 1;
    }
    if soma < 5 && tt > 82.0 - h * 10.0 && tempo == 0x3de && x11 == x12 && x16 == 0 && (h > 1.0 || byte(m, 0x1ab + lu) == 1) {
        *fator = 10.0;
        return 1;
    }
    if h < 1.4 && kb != 0 && d > 1 {
        h = 0.2;
    }
    let retranca = (d > 3 && h > 1.0)
        || (tt > 30.0 && x11 >= x12 && 10.0 / h < ((d + 1) * (d + 1)) as f64 && clube_a != u(clube_b, 0xb3) && clube_a != u(clube_b, 0xb7) && clube_a != u(clube_b, 0xbb));
    if retranca {
        *fator = -1.0;
        return 5;
    }
    if rodando && (h - *fator as f64).abs() < 0.5 {
        return byte(m, 0x1ab + lu) as u8;
    }
    *fator = h as f32;
    if !(h > 0.3) {
        4
    } else if h < 0.7 {
        3
    } else if h > 1.5 {
        1
    } else {
        2
    }
}

// ---------- centesima decima nona leva: a formacao (0x6e0230) ----------

/// { 0x6e0230 } (thiscall, ret 0x10, match_man.cpp): a comissao `c` escolhe a formacao do clube
/// (+0x2f) e carrega a tatica no motor. Com o motor em +0 sorteia a formacao; com `modo` != 2 so
/// recarrega; com tecnico humano a ser consultado (0x5fd210/0x5fea50) usa a tatica dele (0x5fbe20,
/// 0x5fbd00). Senao da nota (0x6e2f20) a cada setor (7, bits 1..0x40) e faixa (5, bits 0x80..0x800)
/// pelas duas melhores das `n` pessoas de `fichas` (o goleiro so na faixa do meio; com menos de 11, o
/// ultimo setor vale metade e as pontas dos setores 1 a 3 ficam com a menor), e julga cada formacao
/// do banco de taticas (0xde1d08): o tempo (+2, ate 3000) e a postura (0x6df460) pesam a nota da
/// formacao (+0x10/+0x18/+0x20, ate 3), o tecnico ajusta (+0x1c, +0xe, +0x1f; a favorita +0x43 vale
/// x1,1 ou ate x3,75, as outras x0,1) e cada vaga soma a nota do seu setor e faixa (0x5b2ae0) e as
/// marcas (0x5b5270/0x5b57c0); o equilibrio dos setores entra pela postura. Grava a melhor (0x8967d0),
/// copia a tatica (0x896660) para o motor (+0x91e2 do lado) e para +0x2b e acerta as instrucoes
/// (0x6ddab0). As notas das formacoes ficam numa lista da CRT do jogo (0x945501/0x9452ca). Devolve 0.
unsafe extern "thiscall" fn escolher_formacao(c: usize, fichas: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let postura: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_MENTALITY);
    let definir: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CLUB_TACTIC_INDEX);
    let carregar: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTIC);
    let instrucoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_SET_COMPUTER_INSTRUCTIONS);
    let pp = postura(c) as i8;
    let clube = u(c, 0x2f);
    let mut escala = w(clube, 0x80) << 3;
    if escala <= 2000 {
        escala = 2000;
    }
    let m = u(c, 0x12);
    let humano = byte(c, 0x19) as i32;
    let tat = || u(0xde1d08, 0);
    let quantas = || (*((tat() + 0x5fd) as *const i32)).wrapping_add(*((tat() + 0x5f5) as *const i32));
    let melhor: i32;
    let tatica = 'tatica: {
        if byte(m, 0) != 0 {
            melhor = sortear(quantas()) as i16 as i32;
        } else if a4 as u8 != 2 {
            break 'tatica carregar(tat(), clube, humano);
        } else {
            let p = u(clube, 0xcf);
            let q = *(0xae2364 as *const i32);
            if humano == 0 && p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
                let gerente = u(0xb63c98, 0);
                let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
                if consultar(gerente, p) != 0 && opcao(gerente, 2, p) != 0 {
                    let preparar: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
                    let dele: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_ACTIVE_TACTIC);
                    preparar(gerente, 0, clube, p);
                    break 'tatica dele(u(0xb63c98, 0), u(clube, 0xcf));
                }
            }
            melhor = julgar_formacoes(c, fichas, a2, a3, pp, escala, calloc, free);
        }
        definir(tat(), clube, melhor as i8 as i16 as u16 as u32, humano);
        carregar(tat(), clube, humano)
    };
    let lado = byte(c, 0x18) as isize;
    core::ptr::copy_nonoverlapping(tatica as *const u8, (m as isize + 0x91e2 + 0x18e3 * lado) as *mut u8, 0x18e3);
    // ponytail: sem tatica (ou sem +0x2b) o original para o jogo com erro (match_man.cpp 0xc1d); aqui so nao copia
    let copia = u(c, 0x2b);
    if tatica != 0 && copia != 0 {
        core::ptr::copy_nonoverlapping(tatica as *const u8, copia as *mut u8, 0x18e3);
        instrucoes(c);
    }
    0
}

/// O miolo da 0x6e0230: as notas por setor e faixa e o julgamento de cada formacao; devolve a melhor.
#[allow(clippy::too_many_arguments)]
unsafe fn julgar_formacoes(
    c: usize, fichas: usize, a2: u32, a3: u32, pp: i8, escala: i32, calloc: unsafe extern "C" fn(u32, u32) -> usize, free: unsafe extern "C" fn(usize),
) -> i32 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let d = |p: usize, o: usize| *((p + o) as *const f64);
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let pegar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_TACTIC_BLOCK);
    let posicao: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let marcas: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let marca_b: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let clube = u(c, 0x2f);
    let n = a2 as u8 as i8 as i32;
    // as notas de cada setor (g) e faixa (s): a soma das duas melhores pessoas
    let mut tab = [0i32; 35];
    for g in 0..7 {
        for s in 0..5 {
            let faixa = if g == 0 {
                if s != 2 {
                    continue;
                }
                0
            } else {
                0x80u32 << s
            };
            let modo = faixa | 1 << g;
            let (mut a, mut b, mut quantos) = (-10000i32, -10000i32, 0);
            for k in 0..n {
                let p = u(fichas + 4 * k as usize, 0);
                let r = nota(p, u(p, 0x61), 0, modo, a3, clube, c, u(c, 0x1b) as u32, u32::MAX, 0, 0, 0, 0, 0, 0).max(-10000);
                match quantos {
                    0 => {
                        a = r;
                        quantos = 1;
                    }
                    1 => {
                        if r > a {
                            b = a;
                            a = r;
                        } else {
                            b = r;
                        }
                        quantos = 2;
                    }
                    _ => {
                        if r > a {
                            b = a;
                            a = r;
                        } else if r > b {
                            b = r;
                        }
                    }
                }
            }
            let soma = a.wrapping_add(b);
            tab[g * 5 + s] = if n < 11 && g == 6 { soma / 2 } else { soma };
        }
    }
    if n < 11 {
        for (x, y) in [(5, 9), (10, 14), (15, 19)] {
            let v = tab[x].min(tab[y]);
            tab[x] = v;
            tab[y] = v;
        }
    }
    let tat = || u(0xde1d08, 0);
    let quantas = || (*((tat() + 0x5fd) as *const i32)).wrapping_add(*((tat() + 0x5f5) as *const i32));
    let lista = calloc(quantas() as u32, 8);
    let (mut melhor, mut melhor_r) = (-1i32, 0f64);
    let tec = u(c, 0x1f);
    let mut idx = 0i32;
    while idx < quantas() {
        let f = u(tat(), 0) + 0x28 * idx as usize;
        let mut t = w(f, 2);
        if t as f64 > 3000.0 {
            t = 3000;
        }
        let v = match pp {
            1 => d(f, 0x10),
            3..=5 => d(f, 0x20),
            _ => d(f, 0x18),
        };
        let v = if v > 3.0 { 3.0 } else { v };
        let mut x = ((3000.0 - t as f64) * 5.0 + v * t as f64) * 0.005;
        let tatica = pegar(tat(), *(f as *const u16) as u32, 0);
        let flags = *((tatica + 0x584) as *const u32);
        if tec != 0 {
            if flags & 0x400 != 0 {
                x += (byte(tec, 0x1c) as i32 - 10) as f64;
            }
            if flags & 0x1000 != 0 {
                x += (10 - byte(tec, 0xe) as i32) as f64;
            }
            if flags & 0x100 != 0 {
                x += (byte(tec, 0x1f) as i32 - 10) as f64;
            }
            let fav = byte(tec, 0x43);
            let favorita = fav > 0 && fav < 10 && {
                let q = u(tat(), 8 + 4 * fav as usize);
                q != 0 && q == u(tatica, 0x187b)
            };
            x *= if !favorita {
                0.1
            } else if n >= 11 {
                let y = n as f64 * 0.2 - 0.2;
                if 3.75 < y { 3.75 } else { y }
            } else {
                1.1
            };
        }
        let (mut f18, mut f20, mut f28, mut f44, mut f54) = (0f32, 0f32, 0f32, 0f32, 0f32);
        let mut soma = 0i32;
        for vaga in 0..11u32 {
            let p1 = posicao(tatica, vaga, 0);
            let p2 = posicao(tatica, vaga, 1) as u8;
            if tec != 0 {
                let r = marcas(tatica, vaga) as u32;
                let fl = marca_b(tatica, vaga);
                if r & 0x8000 != 0 && flags & 0x100 == 0 {
                    x += byte(tec, 0x1f) as f64 * 0.25 - 2.5;
                }
                if r & 0x200000 != 0 {
                    x += byte(tec, 0x15) as f64 - 10.0;
                }
                if r & 0x800000 != 0 {
                    x += byte(tec, 0xe) as f64 * 0.5 - 5.0;
                }
                if fl & 4 != 0 {
                    x += byte(tec, 0x1a) as f64 * 1.5 - 15.0;
                }
            }
            let mais = |v: &mut f32, q: f64| *v = (*v as f64 + q) as f32;
            let (mut setor, mut faixa) = (0usize, -1i32);
            if p1 & 1 != 0 {
                faixa = 2;
            } else if p1 & 2 != 0 {
                mais(&mut f18, 1.0);
                setor = 1;
            } else if p1 & 4 != 0 {
                mais(&mut f18, 1.0);
                setor = 2;
            } else if p1 & 8 != 0 {
                mais(&mut f28, 1.0);
                setor = 3;
            } else if p1 & 0x10 != 0 {
                mais(&mut f54, 1.0);
                setor = 4;
            } else if p1 & 0x20 != 0 {
                mais(&mut f20, 1.0);
                setor = 5;
            } else {
                mais(&mut f44, 1.0);
                setor = 6;
            }
            if p2 & 2 != 0 {
                mais(&mut f18, 0.5);
                setor = 1;
            } else if p2 & 4 != 0 {
                mais(&mut f18, 0.5);
                setor = 2;
            } else if p2 & 8 != 0 {
                mais(&mut f28, 0.5);
                setor = 3;
            } else if p2 & 0x10 != 0 {
                mais(&mut f54, 0.5);
                setor = 4;
            } else if p2 & 0x20 != 0 {
                mais(&mut f20, 0.5);
                setor = 5;
            } else if p2 & 0x40 != 0 {
                mais(&mut f44, 0.5);
                setor = 6;
            }
            if faixa == -1 {
                faixa = if p1 & 0x80 != 0 {
                    0
                } else if p1 & 0x100 != 0 {
                    1
                } else if p1 & 0x200 != 0 {
                    2
                } else if p1 & 0x400 != 0 {
                    3
                } else {
                    4
                };
            }
            soma = soma.wrapping_add(tab[setor * 5 + faixa as usize]);
        }
        // o equilibrio dos setores
        let z = (f18 as f64 - f44 as f64) as f32 as f64;
        let q = |v: f32| pow_do_jogo(v as f64, 2.0);
        let s_ = (q(f44) + q(f44) + q(f20)) - (q(f18) + q(f18)) - q(f28);
        let e = 44.0 - ((32.0 - s_) - (z - f20 as f64 * 0.5).abs() * (z - f20 as f64)) * 0.8;
        let y = match pp {
            1 => (e * 0.015 + 1.0) * x,
            3 => (1.0 - e * 0.015) * x,
            4 => (1.0 - e * 0.025) * x,
            _ => x,
        };
        let r = y * (soma as f64 / escala as f64);
        *((lista + 8 * idx as usize) as *mut f64) = r;
        if melhor == -1 || r > melhor_r {
            melhor_r = r;
            melhor = idx;
        }
        idx += 1;
    }
    if lista != 0 {
        free(lista);
    }
    melhor
}

// ---------- centesima vigesima leva: as instrucoes individuais (0x6ddab0) ----------

/// the staff set the instructions of the tactic copied at +0x2b (0x5b50d0 general, 0x5b2ba0 per position, 0x5b5290
/// 64-bit marks, 0x5b5910 man-marking in the engine's tactic block) and in the tactics bank (0x897ee0). With a human
/// manager to consult it does nothing. At home (or by a draw against the reputation gap in competitions +0x3a = 1 or 9)
/// it asks for 0x20; the manager (+0x13, +0x1f, +0x1c) picks tempo and marking. A factor (reputations / 17000, manager
/// +0xe and the remainder of the ids' sum by 13 − 0x544410/2) is compared with the attributes (0x5431e0) of each of the
/// 20 on the side on the pitch: wing runs, man-marking the opponent from 0x6d4330 (with 0x7c1e30 for midfielders),
/// passing, long shots, dribbling, tackling, offside, set pieces (minus the accumulated value of those with high
/// +0x2e), crosses by table and the set-piece taker (0x897ee0 for whoever is at +6/+0xa/+0xe of the staff). Quirk kept:
/// the marked player's side comes from the player's +0x19 (position 0 becomes the other side)
/// Original: `instructions` (0x6ddab0, match_man.cpp (~6.4 KB), thiscall).
///
/// [pt] { 0x6ddab0 } (thiscall, match_man.cpp, ~6,4 KB): a comissao `c` acerta as instrucoes da tatica
/// copiada em +0x2b (0x5b50d0 gerais, 0x5b2ba0 posicao, 0x5b5290 marcas de 64 bits, 0x5b5910 marcacao
/// individual no motor) e no banco de taticas (0x897ee0). Com tecnico humano a ser consultado nao
/// mexe. Em casa (ou com sorteio contra a diferenca de reputacao nas competicoes +0x3a = 1 ou 9) pede
/// 0x20; o tecnico (+0x13, +0x1f, +0x1c) escolhe o ritmo e a marcacao. Um fator (reputacoes dos dois
/// clubes / 17000, o tecnico +0xe e um resto pelos ids) e comparado com os atributos (0x5431e0) de cada
/// um dos 20 do lado em campo: avanco pelas pontas, marcacao individual do adversario de 0x6d4330
/// (com 0x7c1e30 para os meias), passes, chutes de longe, dribles, desarmes, impedimento, cobrancas
/// (acumulado dos que tem +0x2e alto), cruzamentos por tabela e o especialista de bola parada (0x897ee0
/// para quem esta em +6/+0xa/+0xe da comissao).
unsafe extern "thiscall" fn instrucoes(c: usize) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let attr: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_STAFF_GET_PLAYER_ATTRIBUTE);
    let attr_tec: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_STAFF_GET_NON_PLAYER_ATTRIBUTE);
    let attr_tec2: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let tabela_b: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_SECONDARY_POSITION);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FORMATION_SET_TEAM_INSTRUCTION);
    let pos: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_FORMATION_SET_SLOT_POSITION);
    let marca: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_FORMATION_SET_PLAYER_INSTRUCTION);
    let marcas: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let marcar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_FORMATION_SET_MARKING_TARGET);
    let banco: unsafe extern "thiscall" fn(usize, usize, u32, usize, i32) = core::mem::transmute(FN_TACTICS_SET_PLAYMAKER);
    let quem_marca: unsafe extern "thiscall" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_FIND_MARKING_TARGET);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f32 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let (ca, cb) = (u(c, 0x2f), u(c, 0x33));
    let mut acc = 0i8;
    let f40 = (w(ca, 0x80) as i32 as f64 + w(cb, 0x80) as i32 as f64) * 5.882352941176471e-05;
    let humano = byte(c, 0x19);
    if humano == 0 {
        let p = u(ca, 0xcf);
        let q = *(0xae2364 as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
            let gerente = u(0xb63c98, 0);
            let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
            if consultar(gerente, p) != 0 && opcao(u(0xb63c98, 0), 2, u(u(c, 0x2f), 0xcf)) != 0 {
                return;
            }
        }
    }
    let t = || u(c, 0x2b);
    let em_casa = humano != 0 || {
        let comp = byte(u(c, 0x37), 0x3a);
        (comp == 1 || comp == 9) && {
            let r = sortear(2000).wrapping_add(sortear(-4000));
            (w(u(c, 0x2f), 0x80) as i32 - w(u(c, 0x33), 0x80) as i32) > r
        }
    };
    if em_casa {
        geral(t(), 0x20);
    }
    let tec = u(c, 0x1f);
    if tec == 0 {
        return;
    }
    let f40 = (1.0 - (byte(tec, 0xe) as f64 - 10.0) * 0.03333333333333333) * f40;
    let ritmo = byte(tec, 0x13);
    let r = if ritmo <= 8 {
        2
    } else if ritmo <= 14 {
        1
    } else if ritmo <= 18 {
        4
    } else if ritmo <= 20 {
        8
    } else {
        0
    };
    if r != 0 {
        geral(t(), r);
    }
    let m1f = byte(u(c, 0x1f), 0x1f);
    if m1f >= 15 {
        geral(t(), 0x100);
    } else if m1f <= 5 {
        geral(t(), 0x80);
    }
    if 19.0 - w(u(c, 0x2f), 0x80) as i32 as f64 * 0.0003 < byte(u(c, 0x1f), 0x1c) as f64 && *((t() + 0x187b) as *const u8) & 8 == 0 {
        geral(t(), 0x400);
    }
    let tat = || u(0xde1d08, 0);
    let hum = byte(c, 0x19) as i32;
    banco(tat(), u(c, 0x2f), 0, 0, hum);
    banco(tat(), u(c, 0x2f), 1, 0, byte(c, 0x19) as i32);
    let m = u(c, 0x12);
    let bloco = |k: usize| m + 0x22d8 * byte(c, 0x18) as isize as usize + 0x4796 + 0x1be * k;
    // o acumulado dos que sabem cobrar
    for k in 0..20 {
        let j = bloco(k);
        if byte(j, 0x19) >= 0 && tabela(j) & 0x40 != 0 && tabela(j) & 0x200 != 0 {
            let f = u(j, 0x6d);
            let v = byte(f, 0x2e);
            if v > 14 {
                acc = acc.wrapping_add(ftol(pow_do_jogo((v as i32 - 14) as f64, 1.5)) as i8);
                let v40 = byte(f, 0x40);
                if v40 < 8 {
                    acc = acc.wrapping_add(8 - v40);
                }
            }
        }
    }
    let tc = |o: usize| byte(u(c, 0x1f), o);
    for k in 0..20 {
        let j = bloco(k);
        if byte(j, 0x19) < 0 {
            continue;
        }
        let f = u(j, 0x6d);
        let pos_j = *((j + 0x19) as *const u8) as u32;
        let a = |k: u32| attr(u(j, 0x69), k) as i32;
        let tb = || tabela(j);
        let fb = |o: usize| byte(u(j, 0x6d), o);
        let div = 13 - (attr_tec2(u(c, 0x1f), 0xb) as i32) / 2;
        let resto = (*(u(c, 0x1f) as *const i32)).wrapping_add(*(f as *const i32)) % div;
        let mut f38 = (resto as f64 * 0.05 + 1.0) * f40 * 1.2;
        if f38 < 0.6 {
            f38 = 0.6;
        }
        // avanco pelas pontas
        if *((t() + 0x187b) as *const u32) & 0x1c00000 == 0 {
            let lat = |faixa: u16, ref_: usize| tb() & faixa != 0 && fb(0x14) as i32 >= fb(ref_) as i32 - 1 && fb(0x19) == 20;
            if tb() & 0x10 != 0 && tb() & 0x200 != 0 && fb(0x14) as i32 > fb(0x13) as i32 - 1 && fb(0x19) == 20 && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x220, 1);
            } else if tb() & 0x10 != 0 && tabela_b(j) & 0x20 == 0 && lat(0x100, 0x13) && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 1);
            } else if tb() & 0x10 != 0 && tabela_b(j) & 0x20 == 0 && lat(0x400, 0x13) && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 1);
            } else if tb() & 0x40 != 0 && lat(0x200, 0x15) && f38 * 17.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x240, 1);
            } else if tb() & 0x40 != 0 && lat(0x100, 0x15) && f38 * 17.0 < a(7) as f64 {
                pos(t(), pos_j, 0x120, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x140, 1);
            } else if tb() & 0x40 != 0 && lat(0x400, 0x15) && f38 * 17.0 < a(7) as f64 {
                pos(t(), pos_j, 0x420, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x440, 1);
            }
        }
        let fl = |v: f32| *((j + 0x4d) as *const f32) as f64 > v as f64;
        if (tb() & 2 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0)
            && fl(8250.0)
            && fb(0x3c) > 12
            && f38 * 17.0 < a(7) as f64
            && fb(0x1a) > 15
            && f38 * 40.0 < (a(0x14) + (tc(0x15) as i32 - 10) * 3) as f64
        {
            marca(t(), pos_j, 0x200000, 0);
            if (*((j + 0x4d) as *const f32) as f64) < 8500.0 || fb(0x3c) < 18 || fb(0x44) > 16 {
                marca(t(), byte(j, 0x19) as u8 as u32, 0x4000, 0);
            }
        } else {
            let b = a(0x14) + a(0xd) + a(7) + (tc(0x15) as i32 + fb(0x1a) as i32 - 20) * 3;
            if f38 * 40.0 > b as f64 {
                marca(t(), pos_j, 0x100000, 0);
            }
        }
        // 0x5b5270 bit 1 e +0x584 bit 8: 2, senao 4
        let dois_ou_quatro = || {
            let v = if marcas(t(), byte(j, 0x19) as u8 as u32) & 1 != 0 && *((t() + 0x584) as *const u8) & 8 != 0 { 2 } else { 4 };
            marca(t(), byte(j, 0x19) as u8 as u32, v, 0);
        };
        if tb() & 0x20 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            dois_ou_quatro();
        }
        if (tb() & 8 != 0 || tb() & 0x10 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && f38 * 16.0 > a(7) as f64 {
            dois_ou_quatro();
            let r = quem_marca(m, *((j + 0x27) as *const u8) as u32, byte(j, 0x19) as u8 as u32);
            if r != -1 {
                let lado_o = (byte(j, 0x19) == 0) as usize;
                let o = m + 0x1be * (r as isize as usize).wrapping_add(20 * lado_o) + 0x4796;
                let blk = || (u(c, 0x12) as isize + 0x91e2 + 0x18e3 * byte(j, 0x27) as isize) as usize;
                let tbo = || tabela(o);
                let reps = || {
                    let ra = w(u(c, 0x2f), 0x80);
                    ra < w(u(c, 0x33), 0x80).wrapping_add(0xfa) || w(u(o, 0x6d), 0xb).wrapping_add(0xfa) > ra
                };
                let lados = || tbo() & 0x200 != 0 || tbo() & 0x100 != 0 || tbo() & 0x400 != 0;
                let resto5 = || *(u(o, 0x69) as *const i32) % 5;
                if u(o, 0x69) != 0 {
                    if (tbo() & 0x20 != 0 || tabela_b(o) & 0x20 != 0)
                        && lados()
                        && reps()
                        && a(0x1c) > 12
                        && a(0x11) > 10
                        && a(0x1f) > 10
                        && fb(0x12) == 20
                        && tb() & 0x10 != 0
                        && (tb() & 0x200 != 0 || tb() & 0x100 != 0 || tb() & 0x400 != 0)
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(10) as i32
                    {
                        if u(u(u(c, 0x12), 0x4792), 0x1c) != u(c, 0x2f) || a(0x1a) > 15 {
                            pos(t(), byte(j, 0x19) as u8 as u32, 0x208, 1);
                        }
                        marcar(blk(), byte(j, 0x19) as u8 as u32, u(o, 0x69));
                    } else if u(o, 0x69) != 0
                        && tb() & 8 != 0
                        && (tb() & 0x200 != 0 || tb() & 0x100 != 0 || tb() & 0x400 != 0)
                        && (tbo() & 0x20 != 0 || tabela_b(o) & 0x20 != 0)
                        && tbo() & 0x200 != 0
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 15
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(7) as i32
                    {
                        marcar(blk(), byte(j, 0x19) as u8 as u32, u(o, 0x69));
                    } else if u(o, 0x69) != 0
                        && (tbo() & 0x10 != 0 || tabela_b(o) & 0x10 != 0)
                        && lados()
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 12
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(8) as i32
                    {
                        let fo = u(o, 0x6d);
                        let g = u(0xb74350, 0);
                        let m1 = media(g, fo, 6, 1, 6.5);
                        let m2 = media(u(0xb74350, 0), u(o, 0x6d), 6, 2, 6.5);
                        let m3 = media(u(0xb74350, 0), u(o, 0x6d), 6, 5, 6.5);
                        let (mut st0, mut r3) = (0.0f64, 0.0f32);
                        if m1 as f64 > 2.0 {
                            st0 = m2 as f64 / m1 as f64;
                            r3 = (m3 as f64 / m1 as f64) as f32;
                        }
                        if st0 > 0.35 || r3 as f64 > 0.4 {
                            if (tbo() & 0x20 != 0 || tabela_b(o) & 0x20 != 0) && (fb(0x12) == 20 || fb(0x13) == 20) && a(0x1a) > 14 {
                                pos(t(), byte(j, 0x19) as u8 as u32, 0x208, 1);
                            }
                            marcar(blk(), byte(j, 0x19) as u8 as u32, u(o, 0x69));
                        }
                    }
                }
            }
        }
        // os passes
        let q = f38 * 10.0;
        if (a(0x12) as f64) < q {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x10000000, 0);
        } else if f38 * 13.0 < a(0x12) as f64 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x20000000, 0);
        }
        // chutes de longe
        if (tb() & 8 != 0 || tb() & 0x10 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            let a2 = a(2);
            if (tc(0x1f) as i32 / 5) as f64 + q > a2 as f64 {
                let a3 = a(0x1f);
                let pj = byte(j, 0x19) as u8 as u32;
                if (tc(0x1f) as i32 / 5) as f64 + f38 * 5.0 > a3 as f64 {
                    let v = if marcas(t(), pj) & 0x200 != 0 && *((t() + 0x584) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                    marca(t(), pj, v, 0);
                    marca(t(), byte(j, 0x19) as u8 as u32, 0x8000, 0);
                } else {
                    let a3 = a(0x1f);
                    if (tc(0x1f) as i32 / 5) as f64 + q > a3 as f64 {
                        let v = if marcas(t(), pj) & 0x200 != 0 && *((t() + 0x584) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                        marca(t(), byte(j, 0x19) as u8 as u32, v, 0);
                    }
                }
            }
        }
        if byte(j, 0x24) != 0 && a(2) > tc(0x1f) as i32 / 4 + 8 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x800, 0);
        }
        // dribles
        if tb() & 8 != 0 || tb() & 0x10 != 0 || (tc(0xe) > 16 && tb() & 0x40 != 0) {
            let f28 = f38 * 14.0;
            let te4 = tc(0xe) as i32 / 4;
            let pj = || byte(j, 0x19) as u8 as u32;
            let fl8000 = || *((j + 0x4d) as *const f32) as f64;
            if f28 - (te4 as f64) < a(0x14) as f64 && f28 - ((tc(0xe) as i32 / 4) as f64) < a(0xc) as f64 && fl8000() > 8000.0 {
                marca(t(), pj(), 0x800000, 0);
            } else {
                let te4 = tc(0xe) as i32 / 4;
                let lim = q - te4 as f64;
                let alto = lim > a(0x14) as f64 && lim < a(0xc) as f64;
                if alto || fl8000() < 8000.0 {
                    marca(t(), pj(), 0x400000, 0);
                }
            }
        }
        // desarmes: os de meio e ataque (ou quem desarma bem) entram pela nota mais alta
        {
            let pj = || byte(j, 0x19) as u8 as u32;
            let entra = tb() & 0x10 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0 || f38 * 16.0 < a(0xb) as f64;
            let forte = entra && (f38 * 14.0 < a(0xb) as f64 || f38 * 34.0 - ((tc(0xe) as i32 / 4) as f64) < (a(0x15) + a(0xb)) as f64);
            if forte {
                marca(t(), pj(), 0x2000000, 0);
            } else if f38 * 25.0 - (tc(0xe) as i32 / 4) as f64 > (a(0x15) + a(0xb)) as f64 {
                marca(t(), pj(), 0x1000000, 0);
            }
        }
        if tb() & 0x10 != 0 && fb(0x1a) > 12 && fb(0x14) >= fb(0x13) {
            if tb() & 0x80 != 0 && fb(0x19) > fb(0x18) {
                pos(t(), byte(j, 0x19) as u8 as u32, 0x120, 1);
            } else if tb() & 0x800 != 0 && fb(0x19) > fb(0x17) {
                pos(t(), byte(j, 0x19) as u8 as u32, 0x420, 1);
            }
        }
        // impedimento
        let d = || a(0x1b) - a(0x15) - a(0x14);
        if tb() & 0x40 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 && d() > 0 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x8000000, 0);
        } else if d() < -5 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x4000000, 0);
        }
        // cobrancas
        let l28 = acc as f64;
        let pj = || byte(j, 0x19) as u8 as u32;
        let (k1, k2) = if tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            (f38 * 16.0, q)
        } else if tb() & 8 == 0 && tb() & 0x10 == 0 && tb() & 0x20 == 0 {
            (f38 * 13.0, f38 * 7.0)
        } else {
            (q, f38 * 5.0)
        };
        if k1 - l28 < a(8) as f64 {
            marca(t(), pj(), 0, 4);
        } else if k2 - l28 > a(8) as f64 {
            marca(t(), pj(), 0, 8);
        }
        // cruzamentos pela tabela
        let tv = tb() as i16 as i32;
        let b = || a(0x16) + a(7);
        match tv {
            0x210 | 0x110 | 0xa0 | 0x120 | 0x420 | 0x220 | 0x410 | 0x820 => {
                if f38 * 23.0 < b() as f64 {
                    marca(t(), pj(), 0, 1);
                }
            }
            0x90 | 0x108 | 0x202 | 0x208 | 0x408 | 0x810 => {
                if f38 * 27.0 < b() as f64 {
                    marca(t(), pj(), 0, 1);
                }
            }
            _ => {
                if f38 * 31.0 < b() as f64 {
                    marca(t(), pj(), 0, 1);
                } else if f38 * 20.0 > b() as f64 {
                    marca(t(), pj(), 0, 2);
                }
            }
        }
        // bola parada
        if (tb() & 0x10 != 0 || tb() & 8 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && tabela_b(j) & 0x40 == 0 {
            let ca = u(c, 0x2f);
            let s = a(0x16) + a(9) + a(7);
            if ((w(ca, 0x80) as i32 / 500) as f64 + f38 * 30.0) < s as f64 {
                let pj = u(j, 0x69);
                if u(c, 6) == pj || u(c, 0xa) == pj || u(c, 0xe) == pj {
                    banco(tat(), ca, 0, pj, byte(c, 0x19) as i32);
                }
            }
        }
    }
}

// ---------- centesima vigesima primeira leva: as pecas pequenas da comissao ----------

/// { 0x6e0d00 } (thiscall, ret 0xc, match_man.cpp): poe `pessoa` na vaga `pos` (0..19) do bloco da
/// comissao (+0x23) pela 0x6eaea0 (motor +0x12, lado +0x18, `extra`). Sem pessoa devolve 0; senao 1.
/// ponytail: vaga fora de 0..19 para o jogo com erro no original (match_man.cpp 0xc39); aqui so devolve 0.
/// A 0x6eaea0 (inicia o jogador, com sorteios e efeitos fora do motor) vem injetada: na comparacao
/// ela e unidade da fita (a original grava; a copia Rust confere e nao roda de novo).
unsafe fn colocar_na_vaga_com(c: usize, pos: u32, pessoa: usize, extra: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let p = pos as u8 as i8;
    if pessoa == 0 || !(0..=19).contains(&p) {
        return 0;
    }
    let j = *((c + 0x23) as *const usize) + 0x1be * p as usize;
    chamar(0x6eaea0, j, [*((c + 0x12) as *const usize) as u32, pessoa as u32, *((c + 0x18) as *const u8) as u32, pos, extra]);
    1
}

unsafe extern "thiscall" fn colocar_na_vaga(c: usize, pos: u32, pessoa: usize, extra: u32) -> u8 {
    colocar_na_vaga_com(c, pos, pessoa, extra, &mut |_, this, a| {
        let iniciar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_PLAYER_SETUP);
        iniciar(this, a[0], a[1], a[2], a[3], a[4]);
        0
    })
}

// Porta de 0x6eaea0 (inicia o jogador na vaga): unidade da fita na comparacao.
static mut PORTA_6EAEA0: usize = 0;
const PROLOGO_6EAEA0: Prologo = Prologo { endereco: 0x6eaea0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c] };

unsafe extern "thiscall" fn porta_6eaea0(j: usize, m: u32, pessoa: u32, lado: u32, pos: u32, extra: u32) {
    let o: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(PORTA_6EAEA0);
    if !gravando() {
        return o(j, m, pessoa, lado, pos, extra);
    }
    chamar_gravando(0x6eaea0, j, [m, pessoa, lado, pos, extra], || {
        o(j, m, pessoa, lado, pos, extra);
        0
    });
}

/// As duas pontas das ordens da substituicao: sem pessoa (+0x6d) vai para o fim; +0x19 = -2 vem
/// primeiro; goleiro (tabela 1) com ficha +0xf abaixo de 20 tambem. Devolve None quando o resto decide.
unsafe fn ordem_pontas(a: usize, b: usize) -> Option<i32> {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    if a == 0 || u(a, 0x6d) == 0 {
        return Some(1);
    }
    if b == 0 || u(b, 0x6d) == 0 {
        return Some(-1);
    }
    if byte(a, 0x19) == -2 {
        return Some(-1);
    }
    if byte(b, 0x19) == -2 {
        return Some(1);
    }
    if tabela(a) == 1 && byte(u(a, 0x6d), 0xf) < 20 {
        return Some(-1);
    }
    if tabela(b) == 1 && byte(u(b, 0x6d), 0xf) < 20 {
        return Some(1);
    }
    None
}

/// +0x35 (curto) + +0x4d (float), truncado.
unsafe fn condicao(j: usize) -> i32 {
    ftol(*((j + 0x35) as *const i16) as f64 + *((j + 0x4d) as *const f32) as f64)
}

/// { 0x6e0df0 } (cdecl, comparador do qsort da substituicao): pela condicao (+0x35 + +0x4d), pesada
/// pela tabela de `a` (2 e 4: x1,35; 8: x1,2 em `a` e x1,25 em `b`; 0x10: x1,1) nos dois lados.
unsafe extern "C" fn ordem_por_tabela(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = (*pa, *pb);
    if let Some(r) = ordem_pontas(a, b) {
        return r;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let pesar = |v: i32, oito: f64| -> i32 {
        match tabela(a) {
            4 | 2 => ftol(v as f64 * 1.35),
            8 => ftol(v as f64 * oito),
            0x10 => ftol(v as f64 * 1.1),
            _ => v,
        }
    };
    let va = pesar(condicao(a), 1.2);
    let vb = pesar(condicao(b), 1.25);
    va.wrapping_sub(vb)
}

/// { 0x6e0fe0 } (cdecl, comparador do qsort da substituicao): so pela condicao (+0x35 + +0x4d).
unsafe extern "C" fn ordem_por_condicao(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = (*pa, *pb);
    if let Some(r) = ordem_pontas(a, b) {
        return r;
    }
    let vb = condicao(b);
    condicao(a).wrapping_sub(vb)
}

/// for each place from 0 to 10 + (match +0x4d & 0xf), the person from the club's tactic (0x8968f0) comes in (0x6e0d00)
/// if able to play (0x62c400 ≠ 2, 0x55d270 and 0x7757d0 without impediment); returns how many
/// Original: `starters` (0x6dbb80, match_man.cpp, thiscall).
///
/// [pt] { 0x6dbb80 } (thiscall, match_man.cpp): os titulares do inicio do jogo. Para cada vaga de 0 a
/// 10 + (partida +0x4d & 0xf), a pessoa da tatica do clube (0x8968f0) entra (0x6e0d00) se puder jogar:
/// 0x62c400 diferente de 2, e (partida sem competicao, -2, ou 0x55d270 sem impedimento) e 0x7757d0 sem
/// impedimento. Devolve quantos entraram.
unsafe extern "thiscall" fn titulares(c: usize) -> u8 {
    titulares_com(c, &mut |_, this, a| {
        let colocar: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM);
        colocar(this, a[0], a[1] as usize, a[2]) as u32
    })
}

/// `titulares` com a 0x6e0d00 (poe na vaga) injetada.
unsafe fn titulares_com(c: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let total = || (*((u(c, 0x37) + 0x4d) as *const u8) & 0xf) as i32 + 11;
    let mut n = 0u8;
    let mut k = 0i8;
    while (k as i32) < total() {
        let pos = k as u8 as u32;
        let p = da_tatica(u(0xde1d08, 0), u(c, 0x2f), pos, byte(c, 0x19) as i32);
        if p != 0 && estado(u(0xb63d28, 0), p) != 2 {
            let jogo = u(c, 0x37);
            let livre = *(jogo as *const i32) == -2 || suspenso(u(0xae28f0, 0), p, u(jogo, 0x14), 1, 0) == 0;
            if livre && impedido(u(0xb74318, 0), p, 0xae2c90, u(c, 0x2f), 0, 0) == 0 && chamar(0x6e0d00, c, [pos, p as u32, pos, 0, 0]) as u8 != 0 {
                n = n.wrapping_add(1);
            }
        }
        k = k.wrapping_add(1);
    }
    n
}

/// stores the staff's tactic (+0x3f) in the bank (0x8984d0); with a human manager only when 0x600a50 denies or 0x5fd480
/// confirms
/// Original: `store_tactic` (0x6dbb10, match_man.cpp, thiscall, virtual).
///
/// [pt] { 0x6dbb10 } (thiscall, match_man.cpp, virtual): guarda a tatica da comissao (+0x3f) no banco de
/// taticas (0x8984d0). Com tecnico humano (+0x19) so quando 0x600a50 nega ou 0x5fd480 confirma.
unsafe extern "thiscall" fn guardar_tatica(c: usize) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let clube = u(c, 0x2f);
    if clube == 0 || u(0xde1d08, 0) == 0 {
        return;
    }
    if byte(c, 0x19) != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        let confirma: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
        if humano(clube, 1, 1, 0, 0) != 0 && confirma(u(0xb63c98, 0), u(u(c, 0x2f), 0xcf)) == 0 {
            return;
        }
    }
    let guardar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TACTICS_STORE_SELECTION_RECORD);
    guardar(u(0xde1d08, 0), u(c, 0x2f), c + 0x3f, byte(c, 0x19) as i32);
}

// ---------- centesima quinquagesima leva: a escalacao automatica (0x6dbc50) ----------

/// A caixa de erro "v%s %s.%s %d" do fonte `arquivo` (0xa15da4 match_man.cpp, 0xa15708
/// match_day.cpp) na linha `linha` (o jogo segue).
unsafe fn aviso_em(arquivo: usize, linha: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(arquivo, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// the staff selects the team against the opponent. Outside the human, weighs today's match against the next one on the
/// list (0x5acbe0: competition reputation, rival, same city, days until it) to rest players (+0x1a 0/1/2 and the slot
/// masks); filters the 50 of the lines (state, suspension, already played, barred, on loan), selects (0x6e2000), picks
/// the formation (0x6e0230), the three references (+6/+0xa/+0xe), the rotation (0x8cf880/0x6e8910), the slots
/// (0x6e0d00), the captain (0x897760), the instructions (0x6ddab0) and the general orders by the stance (0x5b50d0). The
/// locals live in a frame with the original stack offsets (zeroed, like the determinised original)
/// Original: `select_automatically` (0x6dbc50, match_man.cpp (~7.7 KB), thiscall, ret 8).
///
/// [pt] { 0x6dbc50 } (thiscall, ret 8, match_man.cpp, ~7,7 KB): a escalacao automatica da comissao `t`
/// (+0x12 motor, +0x18 lado, +0x19 humano, +0x1a peso do jogo, +0x1b tecnico, +0x1f ficha, +0x27
/// linhas, +0x2b tatica, +0x2f clube, +0x37 partida) contra `outro`; `modo` 1 respeita as vagas
/// marcadas na tatica do clube (0x8972c0), 2 refaz tudo. Fora do humano, com clube de reputacao
/// (+0x80 >= 6000) e forca (0x53c590) > 30, mede o jogo contra o proximo da lista (0x5acbe0) para
/// poupar (+0x1a 0/1/2, com as mascaras de 15 vagas). Filtra os 50 das linhas (estado 0x62c400,
/// suspensao 0x55d270, ja jogou 0x8cf5e0, impedido 0x7757d0, emprestado 0x4cf6d0/0x4cf5a0), escala
/// (0x6e2000), escolhe a formacao (0x6e0230), as tres referencias (+6/+0xa/+0xe pela ficha +0xd
/// + +0xb), o rodizio (0x8cf880/0x6e8910), poe nas vagas (0x6e0d00, 0x896a90), o capitao (0x897760),
/// as instrucoes (0x6ddab0) e as ordens gerais pela postura (0x6df460, 0x5b50d0). Devolve quantos
/// escalou. Os locais vivem num quadro com os deslocamentos da pilha original (zerado, como a
/// original determinizada): os vetores se sobrepoem como la (o curto da 1a vaga extra, as vagas
/// marcadas alem de 20).
unsafe extern "thiscall" fn escalar_automatico(t: usize, outro: usize, modo: u32) -> i8 {
    let mut quadro = [0u8; 0x750];
    let f = quadro.as_mut_ptr() as usize;
    let b = |o: usize| (f + o) as *mut u8;
    let rd = |o: usize| core::ptr::read_unaligned((f + o) as *const u32);
    let wd = |o: usize, v: u32| core::ptr::write_unaligned((f + o) as *mut u32, v);
    let rw = |o: usize| core::ptr::read_unaligned((f + o) as *const i16);
    let ww = |o: usize, v: i16| core::ptr::write_unaligned((f + o) as *mut i16, v);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let h16 = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);
    let tb = |o: usize| (t + o) as *mut u8;

    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mais_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let forca: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let rival_de: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_CLUB_STILL_IN_CONTENTION);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let melhor: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_MATCH_MAN_CHOOSE_CAPTAIN);
    let escalar: Escalar = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let guardar_instrucoes: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_TACTICS_CLEAR_OR_RESTORE_SELECTION);
    let vaga_marcada: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(FN_TACTICS_FIND_SELECTION_SLOT);
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let formacao: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC);
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let marcas: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let marca_b: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let rodizio: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
    #[allow(clippy::type_complexity)]
    let reavaliar: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, u32) = core::mem::transmute(FN_MATCH_MAN_APPLY_SQUAD_RESTRICTION);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let colocar: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(FN_TACTICS_SET_SELECTED_PLAYER);
    let capitao: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
    let instrucoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_SET_COMPUTER_INSTRUCTIONS);
    let postura: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_MENTALITY);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FORMATION_SET_TEAM_INSTRUCTION);

    let clube = u(t, 0x2f);
    let jogo = u(t, 0x37);
    let m = u(t, 0x12);
    let lado = byte(t, 0x18) as isize;
    let peso_do_lado = || *(((m as isize) + 0x1ad + lado) as *const i8);
    let competicao = |id: i32| *((u(0xadadfc, 0) as isize + 4 * id as isize) as *const usize);
    let tatica = *(0xde1d08 as *const usize);
    let gerente = *(0xb63c98 as *const usize);
    let mut modo = modo;

    lista_nova(f + 0x80);
    data_padrao(f + 0x38);
    mais_dias(0xae2c90, f + 0xac, 2);
    if outro == 0 {
        erro_de_match_man(0x174);
    }
    *b(0x13) = (*((u(m, 0x4792) + 0x4d) as *const u8) & 0xf).wrapping_add(0xb);
    let forte = forca(clube, 0, 1) as i8;
    *b(0x12) = forte as u8;
    *tb(0x16) = 0;

    // fora do humano: pesa o jogo de hoje contra o proximo da lista (poupar para o mais importante?)
    if byte(t, 0x19) == 0 && h16(clube, 0x80) >= 0x1770 && *(jogo as *const i32) != -2 && humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 && forte > 0x1e {
        mais_dias(0xae2c90, f + 0xa4, 1);
        mais_dias(0xae2c90, f + 0x40, 7);
        let r = jogos(u(0xae2a58, 0), rd(0xa4), rd(0xa6), rd(0x40), rd(0x42), *(clube as *const u32), f + 0x80, u32::MAX, 0xffff_fffe, 1);
        if r as u16 != 0 {
            // 333 x a reputacao da competicao (.69; a da [0x9cf6f8] na tabela [0xae23d0]) + reputacao/3
            let peso = |comp: usize, rival: usize| -> i32 {
                let c = if *(comp as *const u32) == *(KEY_CLUB_COMP_INTER_TOTO_CUP as *const u32) { u(0xae23d0, 0) + u(KEY_CLUB_COMP_UEFA_CUP, 0).wrapping_mul(0x6b) } else { comp };
                (h16(c, 0x69) as i32 * 333).wrapping_add(h16(rival, 0x80) as i32 / 3)
            };
            let comp = u(jogo, 0x14);
            let id = *(comp as *const u32);
            let mut l14 = peso(comp, outro);
            if id != *(KEY_CLUB_COMP_INTER_TOTO_CUP as *const u32) {
                if byte(jogo, 0x3a) == 8 {
                    let k = *((jogo + 0x30) as *const u16);
                    if byte(t, 0x18) == 1 || (0x78..=0x7c).contains(&k) || (0x82..=0x87).contains(&k) || (0x96..=0x9b).contains(&k) {
                        *tb(0x16) = 1;
                    }
                } else if id == *(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as *const u32) && byte(jogo, 0x42) == -1 {
                    *tb(0x16) = 1;
                }
            }
            let mut fator = peso_do_lado() as f64 * f64_em(0x96d438) + f64_em(0x967890);
            if fator > f64_em(0x968dc0) {
                fator = f64_em(0x968dc0);
            }
            let mut esi = ftol(l14 as f64 * fator);
            l14 = esi;
            if rival_de(outro, clube) != 0 {
                esi = ftol(l14 as f64 * f64_em(0x968f98));
                l14 = esi;
                *tb(0x16) = 1;
            } else if outro == u(clube, 0xb3) || outro == u(clube, 0xb7) || outro == u(clube, 0xbb) {
                esi = ftol(l14 as f64 * f64_em(0x968940));
                l14 = esi;
            }
            if na_competicao(competicao(*(jogo as *const i32)), clube) == 0 {
                *tb(0x16) = 0;
                let tec = u(t, 0x1b);
                l14 = if tec != 0 { ftol(byte(tec, 0x5c) as f64 * f64_em(0x968a90) * l14 as f64) } else { esi / 2 };
            }
            wd(0x14, l14 as u32);
            wd(0x8a, rd(0x82));
            let x = proximo(f + 0x80);
            wd(0x20, x as u32);
            if x != 0 && *(x as *const i32) != -2 {
                let d = data_nova(f + 0x24, *((x + 0x2a) as *const u16) as u32, *((x + 0x28) as *const u16) as u32);
                copiar(f + 0x38, f + 0x18, d);
                let rival = if clube == u(x, 0x1c) { u(x, 0x20) } else { u(x, 0x1c) };
                let mut l18 = peso(u(x, 0x14), rival);
                let mut esi = l18;
                if rival_de(rival, clube) != 0 {
                    esi = ftol(l18 as f64 * f64_em(0x968940));
                    l18 = esi;
                } else if rival == u(clube, 0xb3) || rival == u(clube, 0xb7) || rival == u(clube, 0xbb) {
                    esi = ftol(l18 as f64 * f64_em(0x969588));
                    l18 = esi;
                }
                if u(x, 0x18) != u(clube, 0x69) {
                    let c = if u(clube, 0x53) != u(rival, 0x53) { f64_em(0x968978) } else { f64_em(0x9689c0) };
                    esi = ftol(l18 as f64 * c);
                    l18 = esi;
                }
                if na_competicao(competicao(*(x as *const i32)), clube) == 0 {
                    l18 = esi / 3;
                } else if conta(*(jogo as *const i32)) != 0 && (byte(x, 0x3a) == 8 || byte(x, 0x3a) == 7) && *(x as *const i32) > -1 {
                    let c = competicao(*(x as *const i32));
                    if c != 0 && u(c, 8) != 0 {
                        let r = busca(u(c, 8), clube);
                        let k = if r != 0 { *((r + 0x72) as *const u8) } else { 0 };
                        if k == 0 {
                            l18 = esi / 2;
                        } else if k > 6 {
                            l18 = esi.wrapping_mul(6) / k as i32;
                        }
                    }
                }
                wd(0x18, l18 as u32);
                // o proximo jogo daqui a 1..7 dias pesa mais (o de 4 dias, nada)
                for (k, c) in [Some(0x96d4c0usize), Some(0x96d4b8), Some(0x96d4b0), None, Some(0x96d4a8), Some(0x96d4a0), Some(0x96d498)].iter().enumerate() {
                    let p = mais_dias(0xae2c90, f + 0x48, k as u32 + 1);
                    wd(0x24, *(p as *const u32));
                    wd(0x28, *((p + 4) as *const u32));
                    if rw(0x26) == rw(0x3a) && rw(0x24) == rw(0x38) {
                        if let Some(c) = c {
                            l18 = ftol(l18 as f64 * f64_em(*c));
                            wd(0x18, l18 as u32);
                        }
                        break;
                    }
                }
                let (a, b2) = (l18 as f64, l14 as f64);
                if a > b2 + b2 && forte > 0x28 && l14 < 0x1b58 {
                    *tb(0x1a) = 0;
                } else if a > b2 * f64_em(0x968dc0) && forte > 0x23 && l14 < 0x2134 {
                    *tb(0x1a) = 1;
                } else if a > b2 * f64_em(0x9689c0) && l14 < 0x2710 {
                    *tb(0x1a) = 2;
                }
            }
        }
    }

    // poupando: as mascaras das vagas e a primeira escala, com os que podem jogar
    let emprestado_fora = |p: usize| -> bool {
        // emprestado (0x4cf6d0) a outro clube que nao o parceiro (0x540a50)?
        let dono = |p: usize| {
            let c = contrato(u(0xae1050, 0), p);
            *(0xae1080 as *mut usize) = c;
            if c != 0 { u(0xae23b4, 0).wrapping_add(u(c, 4).wrapping_mul(0x245)) } else { 0 }
        };
        if emprestado(u(0xae1050, 0), p) == 0 {
            return false;
        }
        if dono(p) == clube {
            return true;
        }
        dono(p) == parceiro(clube, 0, 1)
    };
    if byte(t, 0x1a) != 3 && humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 {
        match byte(t, 0x1a) {
            0 => {
                for (o, v) in [(0x50, 1), (0x52, 0x804), (0x54, 0x84), (0x56, 0x204), (0x58, 0x204), (0x5a, 0x204), (0x5c, 0x810), (0x5e, 0x90), (0x60, 0x210), (0x62, 0x210), (0x64, 0x210), (0x66, 0x40), (0x68, 0x40), (0x6a, 0x40), (0x6c, 0x40)] {
                    ww(o, v);
                }
                *b(0x30) = 0xe;
            }
            1 => {
                for (o, v) in [(0x50, 4), (0x52, 4), (0x54, 0x204), (0x56, 0x10), (0x58, 0x10), (0x5a, 0x210), (0x5c, 0x40), (0x5e, 0x40)] {
                    ww(o, v);
                }
                *b(0x30) = 7;
            }
            2 => {
                for (o, v) in [(0x50, 4), (0x52, 0x10), (0x54, 0x40)] {
                    ww(o, v);
                }
                *b(0x30) = 2;
            }
            _ => {}
        }
        let x = rd(0x20) as usize;
        *b(0x18) = 0;
        for k in 0..50usize {
            let p = u(u(t, 0x27), 4 * k);
            if p == 0 || u(p, 0x61) == 0 {
                continue;
            }
            if byte(p, 0x18) < 0x10 {
                let c = u(jogo, 0x14);
                if c != 0 && h16(c, 0x69) > 10 {
                    continue;
                }
            }
            if estado(u(0xb63d28, 0), p) == 2 {
                continue;
            }
            if byte(t, 0x19) != 0 {
                if *(jogo as *const i32) != -2 && suspenso(u(0xae28f0, 0), p, u(jogo, 0x14), 1, 0) != 0 {
                    continue;
                }
            } else if x != 0 {
                if u(jogo, 0x14) != u(x, 0x14) && suspenso(u(0xae28f0, 0), p, u(x, 0x14), 1, 0) != 0 {
                    continue;
                }
                if ja_jogou(u(0xde1f54, 0), p, x) != 0 || impedido(u(0xb74318, 0), p, f + 0x38, clube, 0, 0) != 0 {
                    continue;
                }
            }
            let dia = if *(jogo as *const u32) == *(KEY_CLUB_COMP_RESERVE as *const u32) { f + 0xac } else { 0xae2c90 };
            if impedido(u(0xb74318, 0), p, dia, clube, 0, 0) != 0 || emprestado_fora(p) {
                continue;
            }
            let c = *b(0x18) as usize;
            wd(0x278 + 4 * c, p as u32);
            *b(0x18) = (c + 1) as u8;
        }
        if *b(0x18) != 0 {
            let r = melhor(t, f + 0x278, rd(0x18));
            *((t + 0x3b) as *mut usize) = r;
            if r == 0 {
                erro_de_match_man(0x302);
            }
        }
        let cf = u(clube, 0xcf);
        let a7 = if byte(t, 0x19) == 0 {
            0xa520c2
        } else if humano(clube, 1, 1, 0, 0) != 0 && cf != 0 && consultar(gerente, cf) != 0 && opcao(gerente, 3, u(clube, 0xcf)) != 0 {
            0xc520c2
        } else {
            0x8520c2
        };
        escalar(f + 0x278, rd(0x18), f + 0x50, rd(0x30), 0, f + 0x56c, a7, clube, t, u(t, 0x1b) as u32, 0, 0, 0, 0, 0);
    }

    // os candidatos: quem pode jogar entra na lista; no modo 1 os das vagas marcadas so as ocupam
    let humano19 = byte(t, 0x19) as i32 as u32;
    let bl = modo as u8;
    if bl == 2 {
        guardar_instrucoes(tatica, clube, humano19, 0, 0);
    }
    let vaga = |p: usize| vaga_marcada(tatica, clube, p, humano19) as i8;
    let marcar_se = |p: usize| {
        if bl == 1 && vaga(p) > -1 {
            marcar(tatica, clube, p, humano19, 1);
        }
    };
    *b(0x18) = 0;
    for k in 0..50usize {
        wd(0x20, 4 * k as u32);
        let p = u(u(t, 0x27), 4 * k);
        if p == 0 || u(p, 0x61) == 0 {
            continue;
        }
        if byte(t, 0x19) == 0 && byte(p, 0x18) < 0x10 {
            let c = u(jogo, 0x14);
            if c != 0 && h16(c, 0x69) > 10 {
                continue;
            }
        }
        let m5e = |p: usize| *((p + 0x5e) as *const u8);
        let segue = if byte(t, 0x1a) == 3 {
            !(selecao(clube) != 0 && m5e(p) & 8 != 0)
        } else if humano(clube, 0, 1, 0, 0) != 0 {
            if selecao(clube) != 0 {
                m5e(p) & 8 != 0
            } else if byte(t, 0x19) != 0 && m5e(p) & 1 != 0 && vaga(p) == -1 {
                false
            } else {
                byte(t, 0x19) != 0 || m5e(p) & 2 == 0
            }
        } else if *(p as *const i32) >= *(0xae2364 as *const i32) {
            true
        } else if selecao(clube) != 0 {
            m5e(p) & 4 == 0
        } else {
            let mut fora = false;
            let mut j = 0i8;
            while j < *b(0x30) as i8 {
                if rd(0x56c + 4 * j as usize) as usize == p {
                    fora = true;
                    break;
                }
                j += 1;
            }
            !fora
        };
        if !segue {
            continue;
        }
        if selecao(clube) == 0 && cedido(u(0xde1f54, 0), p, clube, 0) != 0 {
            marcar_se(p);
            continue;
        }
        if estado(u(0xb63d28, 0), p) == 2 || (byte(t, 0x19) != 0 && estado(u(0xb63d28, 0), p) != 0) {
            marcar_se(p);
            continue;
        }
        let fora = (*(jogo as *const i32) != -2 && suspenso(u(0xae28f0, 0), p, u(jogo, 0x14), 1, 0) != 0)
            || impedido(u(0xb74318, 0), p, 0xae2c90, clube, 0, 0) != 0
            || (*(jogo as *const i32) != -2 && ja_jogou(u(0xde1f54, 0), p, jogo) != 0)
            || emprestado_fora(p);
        if fora {
            marcar_se(p);
            continue;
        }
        if bl == 1 {
            let v = vaga(p);
            if v > -1 && v < *b(0x13) as i8 {
                *b(0x2e) = 1;
                *b(0x90 + v as usize) = 1;
                continue;
            }
        }
        let c = *b(0x18) as usize;
        wd(0x278 + 4 * c, p as u32);
        *b(0x18) = (c + 1) as u8;
    }
    wd(0x20, 0xc8);
    let mut bl = bl;
    if bl == 1 && *b(0x2e) == 0 {
        bl = 2;
        modo = (modo & !0xff) | 2;
    }

    // as marcas da escala
    let mut mascara: u32 = if selecao(clube) != 0 { 0x208a09c } else { 0xca0ca };
    if byte(t, 0x19) != 0 {
        mascara = (mascara | 0x20100) ^ 0x80000;
    } else {
        let dif = || h16(clube, 0x80) as i32 - h16(outro, 0x80) as i32;
        let a = byte(jogo, 0x3a);
        let mut feito = false;
        if a == 1 || a == 9 {
            let lim = sorteio(2000).wrapping_add(sorteio(-4000));
            if dif() > lim {
                mascara |= 0x100;
                feito = true;
            }
        }
        if !feito {
            let lim = sorteio(3000) + 3000;
            if dif() > lim {
                mascara |= 0x100;
                if selecao(clube) == 0 {
                    mascara |= 0x10000;
                }
            } else {
                let j0 = *(jogo as *const i32);
                if byte(t, 0x1a) != 3 || (j0 > -1 && competicao(j0) != 0 && na_competicao(competicao(j0), clube) == 0) {
                    mascara = (mascara | 0x100) ^ 0x80000;
                } else if selecao(clube) == 0 {
                    mascara |= 0x10000;
                }
            }
        }
    }
    if byte(t, 0x16) != 0 {
        mascara |= 0x800000;
    }
    wd(0x14, mascara);

    // as tres referencias (a maior ficha +0xd + +0xb, sem repetir)
    if *b(0x18) != 0 && bl == 2 {
        let r = melhor(t, f + 0x278, rd(0x18));
        *((t + 0x3b) as *mut usize) = r;
        if r == 0 {
            erro_de_match_man(0x42b);
        }
        if mascara & 0x80000 != 0 {
            for s in 0..3usize {
                ww(0x40 + 2 * s, 0xd8f0u16 as i16);
                ww(0x24 + 2 * s, -1);
                let mut c = 0i8;
                while c < *b(0x18) as i8 {
                    let p = rd(0x278 + 4 * c as usize) as usize;
                    let c16 = c as i16;
                    let pula = p == 0
                        || *(p as *const i32) >= *(0xae2364 as *const i32)
                        || u(p, 0x61) == 0
                        || (s == 1 && rw(0x24) == c16)
                        || (s == 2 && (rw(0x24) == c16 || rw(0x26) == c16));
                    if !pula {
                        let fi = u(p, 0x61);
                        let v = h16(fi, 0xd).wrapping_add(h16(fi, 0xb));
                        if v > rw(0x40 + 2 * s) {
                            ww(0x24 + 2 * s, c16);
                            ww(0x40 + 2 * s, v);
                        }
                    }
                    c += 1;
                }
                let a = rw(0x24 + 2 * s);
                if a > -1 {
                    core::ptr::write_unaligned((t + 6 + 4 * s) as *mut u32, rd(0x278 + 4 * a as usize));
                }
            }
        }
    }
    formacao(t, f + 0x278, rd(0x18), rd(0x14), modo);

    // as vagas: as 11 da formacao (casas, marcas) e as extras sorteadas
    let n = *b(0x13) as i8 as i32;
    wd(0x30, n as u32);
    *b(0x12) = 0;
    *b(0x20) = 0;
    *b(0x40) = 0;
    let tat = || u(t, 0x2b);
    let extra = |o: usize, k: usize, primeiro: i32| {
        if k == 12 && *b(0x2f) == 0 && (*b(0x13) as i8 > 14 || sorteio(primeiro) == 0) {
            *b(0x2f) = 1;
            true
        } else {
            let v = rw(o);
            ww(o, if v < 3 { 4 } else if v >= 6 { 0x40 } else { 0x10 });
            false
        }
    };
    for k in 0..n.max(0) as usize {
        if modo as u8 == 1 {
            if *b(0x90 + k) != 0 {
                continue;
            }
            let c = *b(0x12) as i8 as usize;
            if k <= 10 {
                ww(0xe8 + 2 * c, casa(tat(), k as u32, 0) as i16);
                ww(0x214 + 2 * c, casa(tat(), k as u32, 1) as i16);
                let q = marcas(tat(), k as u32);
                wd(0x5bc + 8 * c, q as u32);
                wd(0x5c0 + 8 * c, (q >> 32) as u32);
                *b(0xb4 + c) = marca_b(tat(), k as u32);
                *b(0x20) = (*b(0x20)).wrapping_add(1);
            } else {
                wd(0x5bc + 8 * c, 0);
                wd(0x5c0 + 8 * c, 0);
                *b(0xb4 + c) = 0;
                ww(0xe8 + 2 * c, sorteio(10) as i16);
                ww(0x214 + 2 * c, 0);
                if extra(0xe8 + 2 * c, k, 0x14) {
                    ww(0xe8 + 2 * c, 1);
                }
                *b(0x40) = (*b(0x40)).wrapping_add(1);
            }
            *b(0x12) = (*b(0x12)).wrapping_add(1);
        } else if k <= 10 {
            *b(0x12) = (*b(0x12)).wrapping_add(1);
            *b(0x20) = (*b(0x20)).wrapping_add(1);
            ww(0xe8 + 2 * k, casa(tat(), k as u32, 0) as i16);
            ww(0x214 + 2 * k, casa(tat(), k as u32, 1) as i16);
            let q = marcas(tat(), k as u32);
            wd(0x5bc + 8 * k, q as u32);
            wd(0x5c0 + 8 * k, (q >> 32) as u32);
            *b(0xb4 + k) = marca_b(tat(), k as u32);
        } else {
            *b(0x40) = (*b(0x40)).wrapping_add(1);
            *b(0x12) = (*b(0x12)).wrapping_add(1);
            wd(0x5bc + 8 * k, 0);
            wd(0x5c0 + 8 * k, 0);
            *b(0xb4 + k) = 0;
            ww(0xe8 + 2 * k, sorteio(10) as i16);
            // como na original: zera o curto da vaga seguinte (o desta fica com o que havia)
            ww(0x214 + 2 * (*b(0x12) as i8 as usize), 0);
            if extra(0xe8 + 2 * k, k, 10) {
                ww(0x100, 1);
            }
        }
    }
    let cf = u(clube, 0xcf);
    if humano(clube, 1, 1, 0, 0) != 0 && byte(t, 0x19) == 0 && cf != 0 && consultar(gerente, cf) != 0 && opcao(gerente, 3, u(clube, 0xcf)) != 0 {
        wd(0x14, rd(0x14) | 0x400000);
    }
    escalar(f + 0x278, rd(0x18), f + 0xe8, rd(0x20), rd(0x40), f + 0x14c, rd(0x14), clube, t, u(t, 0x1b) as u32, 0, f + 0x5bc, f + 0xb4, *((u(t, 0x2b) + 0x584) as *const u32), f + 0x214);
    let n = rd(0x30) as i32;
    if n - 1 > 0 {
        for i in 0..(n - 1) as usize {
            for j in i + 1..n as usize {
                let a = rd(0x14c + 4 * i);
                if a != 0 && a == rd(0x14c + 4 * j) {
                    aviso_em(0xa15da4, 0x4c9);
                }
            }
        }
        wd(0x20, n as u32);
    }

    // o rodizio (0x8cf880 aponta quem descansa; 0x6e8910 reavalia), no maximo duas voltas
    if *(jogo as *const i32) != -2 {
        *b(0x2e) = 0;
        let mut r = rodizio(u(0xde1f54, 0), clube, jogo, 0, f + 0x14c, 2, f + 0x78);
        while r != 0 && *b(0x2e) != 2 {
            let nb = *b(0x13);
            *b(0x2e) = (*b(0x2e)).wrapping_add(1);
            core::ptr::copy(b(0x14c), b(0x340), 44);
            let v = if nb as i8 > 0xb {
                let k = rd(0x30) - 0xb;
                let v = calloc(k, 4);
                core::ptr::copy(b(0x178), v as *mut u8, 4 * k as usize);
                v
            } else {
                0
            };
            wd(0x24, v as u32);
            *b(0x20) = 0;
            let w = if *b(0x18) as i8 > nb as i8 {
                let w = calloc(0x32, 4);
                for c in 0..(*b(0x18) as i8).max(0) as usize {
                    let p = rd(0x278 + 4 * c);
                    if p == 0 {
                        continue;
                    }
                    let dentro = nb as i8 > 0 && (0..rd(0x30) as usize).any(|j| rd(0x14c + 4 * j) == p);
                    if !dentro {
                        *((w + 4 * *b(0x20) as i8 as usize) as *mut u32) = p;
                        *b(0x20) = (*b(0x20)).wrapping_add(1);
                    }
                }
                w
            } else {
                0
            };
            // ponytail: o 6o argumento leva so o byte (n - 11); a original passa sobra de ebx nos altos (lidos como byte)
            reavaliar(clube, u(t, 0x1b), f + 0x78, rd(0x14), 0xb, nb.wrapping_sub(0xb) as u32, rd(0x20), f + 0x340, v, w, f + 0x14c, f + 0xe8, f + 0x214, f + 0x5bc, f + 0xb4, 0);
            free(v);
            if w != 0 {
                free(w);
            }
            r = rodizio(u(0xde1f54, 0), clube, jogo, 0, f + 0x14c, 2, f + 0x78);
        }
    }
    if modo as u8 == 1 {
        *b(0x12) = 0;
        *b(0x24) = 0;
        let nn = *b(0x13) as i8;
        if nn > 0 {
            for k in 0..nn as usize {
                let p = if *b(0x90 + k) != 0 {
                    da_tatica(tatica, clube, rd(0x24), byte(t, 0x19) as i32) as u32
                } else {
                    let c = *b(0x12) as i8 as usize;
                    *b(0x12) = (*b(0x12)).wrapping_add(1);
                    rd(0x14c + 4 * c)
                };
                wd(0x36c + 4 * k, p);
                *b(0x24) = (k + 1) as u8;
            }
            core::ptr::copy(b(0x36c), b(0x14c), 4 * rd(0x30) as usize);
        }
    }

    // nas vagas, o capitao, as instrucoes
    let cap = melhor(t, f + 0x14c, 0xb);
    let n = rd(0x30) as i32;
    for k in 0..n.max(0) as usize {
        colocar(t, k as u32, rd(0x14c + 4 * k) as usize, k as u32);
        let p = rd(0x14c + 4 * k) as usize;
        if p != 0 {
            gravar(tatica, clube, k as u32, p, humano19);
            *b(0x37) = (*b(0x37)).wrapping_add(1);
            if rd(0x14c + 4 * k) as usize == cap {
                capitao(tatica, clube, k as u32, humano19);
            }
        }
    }
    let n_final = n.max(0) as usize;
    instrucoes(t);
    let nome = if u(t, 0x2b) != 0 { u(t, 0x2b) } else { 0xa15934 };
    let nome = core::ffi::CStr::from_ptr(nome as *const core::ffi::c_char).to_bytes();
    let especial = nome != b"3-4-3" && nome != b"3-5-2 Attacking";

    // as ordens gerais pela postura (0x5b50d0)
    let g = |x: u32| geral(u(t, 0x2b), x);
    let a = |x: u32| {
        g(x);
        g(0x800);
        g(0x2000);
    };
    let bx = |x: u32| {
        g(x);
        g(0x2000);
    };
    let cx = || {
        g(0x800);
        g(0x2000);
    };
    let ficha = u(t, 0x1f);
    let lim = |base: i32| if peso_do_lado() > 0 { base + 3 } else { base };
    let disposicao = |i: isize| *(((m as isize) + 0x476a + 4 * i) as *const f32) as f64;
    let e = |o: usize| byte(ficha, o) as i32;
    match postura(t) as i8 {
        1 => {
            let d = disposicao(lado);
            let x = if ficha != 0 && !(d >= f64_em(0x968e18)) {
                if e(0xe) > 6 && especial { 0x10000 } else { 0x20000 }
            } else if especial || d > f64_em(0x9688b0) {
                0x10000
            } else {
                0x20000
            };
            a(x);
        }
        3 => {
            if ficha == 0 {
                g(0x8000);
                bx(0x1000);
            } else {
                g(if e(0xe) > 0x10 { 0x20000 } else { 0x8000 });
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        4 => {
            if ficha != 0 {
                g(if e(0xe) > 0x12 { 0x20000 } else { 0x8000 });
            } else {
                g(0x8000);
            }
            if ficha != 0 && e(0xe) > 0xc {
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            } else {
                g(0x800);
                g(0x4000);
            }
        }
        5 => {
            if ficha == 0 {
                g(0x20000);
                bx(0x1000);
            } else {
                g(if e(0xe) > 0x10 { 0x10000 } else if e(0xe) > 0xc { 0x20000 } else { 0x8000 });
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        _ => {
            if ficha == 0 {
                a(0x20000);
            } else if e(0xe) > lim(0xf) && especial {
                a(0x10000);
            } else if e(0xe) <= 5 && peso_do_lado() != 0 && !(disposicao(n_final as isize) > f64_em(0x967890)) {
                // como na original: le a disposicao no indice do total de vagas (o ebx do laco), nao do lado
                g(0x8000);
                if e(0x13) < lim(4) { bx(0x1000) } else { cx() }
            } else {
                a(0x20000);
            }
        }
    }
    destruir_lista(f + 0x80);
    *b(0x37) as i8
}

// ponytail: aloca a lista e os vetores do rodizio e chama 24 rotinas ainda originais; sem comparacao
// propria, validada pela carreira (Rust x original), como as da 145a, 148a e 149a.
unsafe extern "thiscall" fn comparar_escalar_automatico(t: usize, outro: usize, modo: u32) -> i8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i8>(331)(t, outro, modo)
}

// ---------- centesima decima sexta leva: a substituicao (0x6e1090) ----------

/// Minuto do relogio (+0x8ed0 / 11), no maximo o fim do periodo (+0x8ed4: 45, 90, 105, 120).
unsafe fn minuto_do_periodo(m: usize) -> i8 {
    let min = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
    match *((m + 0x8ed4) as *const i16) {
        0x1ef => min.min(0x2d),
        0x3de => min.min(0x5a),
        0x483 => min.min(0x69),
        0x528 => min.min(0x78),
        _ => min,
    }
}

/// { 0x6e1090 } (ret 4, match_man.cpp) com as chamadas que mexem no jogo injetadas (0x6e2000 escala,
/// 0x6d6f70 narra, 0x897760 grava o capitao, 0x6e1aa0 refaz o time): a comissao `c` faz ate +0x17
/// trocas. Os elegiveis (em campo, ou -2 sem +0x1e) vao para uma lista ordenada pelo qsort da CRT
/// (0x945333 com 0x6e0df0, ou 0x6e0fe0 com 11 ou mais); os `m` primeiros dao as vagas (tabelas
/// 0x717430 para os -2, 0x717410/0x717450; posicao +0x19 ou +0x1a; a primeira e 0 com `goleiro`;
/// marcas 0x5b5270 e byte 0x5b57c0 da posicao); depois delas os reservas livres (+0x19 = -1) do 12o em diante, ate
/// 11 + (+0x4d da competicao & 0xf), entram como candidatos. O modo da escala sai do clube (0x53b900),
/// da forca contra o adversario (+0x80, com sorteios), da competicao (+0x3a, +0x1a = 3 e 0x48f150),
/// de +0x16 e do tecnico humano. Quem ganha vaga e estava no banco entra (0x6cdd00, -1 em +0x17,
/// minuto em +0x1f) com a posicao da vaga; quem fica sem vaga sai (0x6cdd50 com -1, +0x19 = -4, +0x1e
/// = 1, minuto em +0x20). Recompoe as posicoes (0x6ce0f0, 0x6ce110) e narra as trocas (0x1f59 mais de
/// 2, 0x1f5c duas, 0x1f5a uma; sem troca para), cada uma com os comentarios de cansaco (0x211d por
/// sorteio(+0x31) abaixo de -10) e de lesao (0x211e) do que sai e do que entra, e 0x20d7 ou 0x1f5b
/// (+0x22 negativo do que sai). Por fim o mais bem avaliado (0x6e1cf0) vira capitao e o time e refeito.
unsafe fn substituir_com(c: usize, goleiro: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let tabela_c: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_ALT_SLOT_POSITION);
    let tabela_b: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_SECONDARY_POSITION);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_CLUB_STILL_IN_CONTENTION);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let t9b23: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let entrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_PLACE_PLAYER_AT_ENTRY_CELL);
    let tirar: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REMOVE_PLAYER_FROM_PITCH);
    let recompor: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_CLEAR_GOALKEEPER);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let u = |o: usize| *((c + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let trocas = byte(c, 0x17);
    if trocas == 0 {
        return;
    }
    let mut lista = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        let e = byte(j, 0x19);
        if e >= 0 || (e == -2 && byte(j, 0x1e) == 0) {
            lista[n] = j;
            n += 1;
        }
    }
    qsort(lista.as_mut_ptr() as usize, n as u32, 4, if n < 11 { 0x6e0df0 } else { 0x6e0fe0 });
    let vagas = (trocas as i32).min(n as i32).max(0) as usize;
    let mut tab = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut pos = [0i8; 20];
    let mut marca = [0u64; 20];
    let mut bpos = [0u8; 20];
    for i in 0..vagas {
        let p = lista[i];
        if byte(p, 0x19) == -2 {
            tab[i] = tabela_c(p);
            tab_b[i] = 0;
        } else {
            tab[i] = tabela(p);
            tab_b[i] = tabela_b(p);
        }
        pos[i] = if i == 0 && goleiro != 0 {
            0
        } else if byte(p, 0x19) == -2 {
            byte(p, 0x1a)
        } else {
            byte(p, 0x19)
        };
        marca[i] = marcas(u(0x2b), pos[i]);
        bpos[i] = byte_pos(u(0x2b), pos[i]);
    }
    let comp = u(0x37);
    // os reservas entram logo depois das vagas (o resto da lista ordenada fica de fora)
    let mut n1 = vagas;
    let limite = (*((comp + 0x4d) as *const u8) & 0xf) as i32 + 0xb;
    if limite > 0xb {
        let mut p = u(0x23) + 0x132a;
        let mut k = 0xb;
        while k < limite {
            if p != 0 && byte(p, 0x19) == -1 {
                lista[n1] = p;
                n1 += 1;
            }
            k += 1;
            p += 0x1be;
        }
    }
    let mut fichas = [0usize; 20];
    for i in 0..n1 {
        fichas[i] = *((lista[i] + 0x69) as *const usize);
    }
    // o modo da escala
    let clube = u(0x2f);
    let forca = |o: usize| w(*((c + o) as *const usize), 0x80) as i32;
    let mut modo: u32 = if selecao(clube) != 0 { 0x1ffffd2 + 0x8a0cb } else { 0x8a0cb };
    if byte(c, 0x19) != 0 {
        modo = (modo | 0x20100) ^ 0x80000;
    } else {
        let mut feito = false;
        if matches!(byte(comp, 0x3a), 1 | 9) {
            let r = sortear(0x7d0).wrapping_add(sortear(-4000));
            if forca(0x2f) - forca(0x33) > r {
                modo |= 0x100;
                feito = true;
            }
        }
        if !feito {
            let r = sortear(0xbb8) + 0xbb8;
            let checa_selecao = if forca(0x2f) - forca(0x33) > r {
                modo |= 0x100;
                true
            } else if byte(c, 0x1a) != 3 {
                false
            } else {
                let id = *(comp as *const i32);
                id <= -1 || na_competicao(*((*(0xadadfc as *const usize) + id as usize * 4) as *const usize), clube) != 0
            };
            if checa_selecao {
                if selecao(clube) == 0 {
                    modo |= 0x10000;
                }
            } else {
                modo = (modo | 0x100) ^ 0x80000;
            }
        }
    }
    if byte(c, 0x16) != 0 {
        modo |= 0x800000;
    }
    if humano(clube, 1, 1, 0, 0) != 0 && byte(c, 0x19) == 0 {
        let p = *((clube + 0xcf) as *const usize);
        if p != 0 && t9a01(*(0xb63c98 as *const usize), p) != 0 && t9b23(*(0xb63c98 as *const usize), 3, p) != 0 {
            modo |= 0x400000;
        }
    }
    let mut lugar = [0usize; 20];
    let extra = *((u(0x2b) + 0x584) as *const u32);
    let args: [usize; 15] = [
        fichas.as_ptr() as usize, n1, tab.as_ptr() as usize, vagas, 0, lugar.as_mut_ptr() as usize, modo as usize, clube, c, u(0x1b), 0,
        marca.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    pedir_vetores(
        &[(args[0], 4 * n1), (args[2], 2 * vagas), (args[11], 8 * vagas), (args[12], vagas), (args[14], 2 * vagas)],
        &[(args[5], 4 * vagas)],
    );
    chamar(0x6e2000, 0, [args.as_ptr() as u32, n1 as u32, modo, clube as u32, c as u32]);
    let m = u(0x12);
    let mut entraram = [0usize; 20];
    let mut sairam = [0usize; 20];
    let (mut ne, mut ns) = (0usize, 0usize);
    for i in 0..n1 {
        let mut j = u(0x23);
        let mut achou = false;
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == fichas[i] {
                achou = true;
                break;
            }
            j += 0x1be;
        }
        if !achou || j == 0 {
            *(0xb67a34 as *mut u32) = 0;
            continue;
        }
        let vaga = (0..vagas).find(|&k| lugar[k] == *((j + 0x69) as *const usize));
        match vaga {
            Some(k) => {
                if byte(j, 0x19) == -1 {
                    *((c + 0x17) as *mut i8) = byte(c, 0x17).wrapping_sub(1);
                    entrar(m, j);
                    *((j + 0x1f) as *mut i8) = minuto_do_periodo(m);
                    entraram[ne] = j;
                    ne += 1;
                }
                let e = byte(j, 0x19);
                if e >= 0 || e == -1 {
                    *((j + 0x19) as *mut i8) = pos[k];
                }
            }
            None => {
                if byte(j, 0x19) == -1 {
                    continue;
                }
                tirar(m, j, -1);
                *((j + 0x19) as *mut i8) = -4;
                *((j + 0x1e) as *mut u8) = 1;
                *((j + 0x20) as *mut i8) = minuto_do_periodo(m);
                sairam[ns] = j;
                ns += 1;
            }
        }
    }
    recompor(m, byte(c, 0x18));
    manter(m, byte(c, 0x18));
    let narrador = *((m + 0x8f7a) as *const usize);
    let lado_c = byte(c, 0x18) as u8 as u32;
    let codigo = match ne {
        0 => return,
        1 => 0x1f5a,
        2 => 0x1f5c,
        _ => 0x1f59,
    };
    chamar(0x6d6f70, narrador, [codigo, m as u32, u32::MAX, lado_c, u32::MAX]);
    for i in 0..ne {
        let (sai, entra) = (sairam[i], entraram[i]);
        let comentarios = byte(m, 0x8ebb) != 0;
        // cansaco: primeiro o que sai, depois o que entra
        let cansado = if comentarios && sortear(w(sai, 0x31) as i32) < -10 {
            Some(sai)
        } else if comentarios && sortear(w(entra, 0x31) as i32) < -10 {
            Some(entra)
        } else {
            None
        };
        if let Some(p) = cansado {
            chamar(0x6d6f70, narrador, [0x211d, m as u32, u32::MAX, (byte(p, 0x27) == 0) as u32, u32::MAX]);
        }
        let machucado = if comentarios && {
            let r = sortear(w(sai, 0x35) as i32 / 30 + w(sai, 0x33) as i32 - 0xe9);
            r > sortear(0x1e)
        } {
            Some(sai)
        } else if comentarios && {
            let r = sortear(w(entra, 0x33) as i32);
            r > sortear(0x1e)
        } {
            Some(entra)
        } else {
            None
        };
        if let Some(p) = machucado {
            chamar(0x6d6f70, narrador, [0x211e, m as u32, u32::MAX, byte(p, 0x27) as u8 as u32, u32::MAX]);
        }
        let cod = if byte(sai, 0x22) > -1 { 0x20d7 } else { 0x1f5b };
        let a = [cod, m as u32, byte(entra, 0x2e) as u8 as u32, byte(entra, 0x27) as u8 as u32, byte(sai, 0x2e) as u8 as u32];
        PROXIMO_SEXTO.store(byte(sai, 0x27) as u8 as u32, std::sync::atomic::Ordering::Relaxed);
        chamar(0x6d6f70, narrador, a);
    }
    // o capitao
    let (mut melhor, mut nota) = (-1i8, 0i16);
    let mut j = u(0x23);
    for k in 0..20i8 {
        if byte(j, 0x19) >= 0 {
            let v = avaliar(0, j, clube);
            if melhor == -1 || v > nota {
                nota = v;
                melhor = k;
            }
        }
        j += 0x1be;
    }
    chamar(0x897760, *(0xde1d08 as *const usize), [clube as u32, melhor as u8 as u32, byte(c, 0x19) as i32 as u32, 0, 0]);
    chamar(0x6e1aa0, c, [0, 0, 0, 0, 0]);
}

/// O 6o argumento da proxima narracao (o lado de quem sai em 0x20d7/0x1f5b; -1 nas outras).
static PROXIMO_SEXTO: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);

/// { 0x6e1090 } (ret 4, match_man.cpp): ver `substituir_com`.
unsafe extern "thiscall" fn substituir(c: usize, goleiro: u32) {
    substituir_com(c, goleiro as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e2000 => {
                let f: Escalar = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
                let v = *(a[0] as usize as *const [usize; 15]);
                return f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14]);
            }
            0x897760 => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            _ => {
                let sexto = PROXIMO_SEXTO.swap(u32::MAX, std::sync::atomic::Ordering::Relaxed);
                comentar(this, [a[0], a[1], a[2], a[3], a[4], sexto, 0]);
            }
        }
        0
    })
}

// ---------- centesima decima quinta leva: a comissao refaz o time (0x6e1aa0) ----------

/// Argumentos da 0x6e2000 (escalar: distribui as pessoas pelas posicoes, recursiva, aloca), cdecl.
type Escalar = unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u32;

/// { 0x6e1aa0 } (match_man.cpp) com as chamadas que mexem no jogo injetadas (0x6e0230 julga a
/// escalacao, 0x6e2000 escala, 0x6d6f70 narra): a comissao (`c`: +0x12 motor, +0x23 primeiro dos 20
/// jogadores, +0x2b tatica, +0x2f clube) refaz o time. Com exatamente 11 em campo e a escalacao
/// reprovada (0x6e0230 com 0x8c1, 2) narra 0x2007. Depois junta os de linha em campo (fora a tabela
/// 1): posicao +0x19, tabelas 0x717410/0x717450, marcas (0x5b5270) e byte (0x5b57c0) da posicao e a
/// ficha +0x69; escala (0x6e2000 com 0x8800, ou 0x408800 com tecnico humano a ser consultado:
/// 0x600a50, +0x19 da comissao, +0xcf do clube, 0x5fd210, 0x5fea50 com 3) e devolve a cada ficha
/// escalada a posicao do lugar que ela ganhou, refazendo o desempenho (0x6eecb0).
unsafe fn refazer_time_com(c: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let tabela_b: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_SECONDARY_POSITION);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let t9b23: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    let u = |o: usize| *((c + o) as *const usize);
    let mut fichas = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        if byte(j, 0x19) >= 0 {
            fichas[n] = *((j + 0x69) as *const usize);
            n += 1;
        }
    }
    if n == 11 {
        pedir_vetores(&[(fichas.as_ptr() as usize, 44)], &[]);
        if chamar(0x6e0230, c, [fichas.as_ptr() as u32, 0xb, 0x8c1, 2, 0]) as u8 != 0 {
            let m = u(0x12);
            chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [0x2007, m as u32, u32::MAX, byte(c, 0x18) as u8 as u32, u32::MAX]);
        }
    }
    let mut pos = [0i8; 20];
    let mut tab = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut marca = [0u64; 20];
    let mut bpos = [0u8; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        if byte(j, 0x19) < 0 || tabela(j) == 1 {
            continue;
        }
        pos[n] = byte(j, 0x19);
        tab[n] = tabela(j);
        tab_b[n] = tabela_b(j);
        marca[n] = marcas(u(0x2b), byte(j, 0x19));
        bpos[n] = byte_pos(u(0x2b), byte(j, 0x19));
        fichas[n] = *((j + 0x69) as *const usize);
        n += 1;
    }
    let clube = u(0x2f);
    let mut modo = 0x8800u32;
    if humano(clube, 1, 1, 0, 0) != 0 && byte(c, 0x19) == 0 {
        let p = *((clube + 0xcf) as *const usize);
        if p != 0 && t9a01(*(0xb63c98 as *const usize), p) != 0 && t9b23(*(0xb63c98 as *const usize), 3, p) != 0 {
            modo = 0x408800;
        }
    }
    let mut lugar = [0usize; 20];
    let extra = *((u(0x2b) + 0x584) as *const u32);
    let args: [usize; 15] = [
        fichas.as_ptr() as usize, n, tab.as_ptr() as usize, n, 0, lugar.as_mut_ptr() as usize, modo as usize, clube, c, u(0x1b), 0,
        marca.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    pedir_vetores(
        &[(args[0], 4 * n), (args[2], 2 * n), (args[11], 8 * n), (args[12], n), (args[14], 2 * n)],
        &[(args[5], 4 * n)],
    );
    chamar(0x6e2000, 0, [args.as_ptr() as u32, n as u32, modo, clube as u32, c as u32]);
    for i in 0..n {
        let mut j = u(0x23);
        let mut achou = false;
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == fichas[i] {
                achou = true;
                break;
            }
            j += 0x1be;
        }
        if !achou {
            j = 0;
        }
        let f = *((j + 0x69) as *const usize);
        if let Some(k) = (0..n).find(|&k| lugar[k] == f) {
            *((j + 0x19) as *mut i8) = pos[k];
            desempenho(j, 0);
        }
    }
}

/// the staff redo the team. With 11 on the pitch and the line-up rejected (0x6e0230 with 0x8c1, 2) narrates 0x2007.
/// Gathers the outfield players on the pitch (position +0x19, tables 0x717410/0x717450, position marks 0x5b5270 and
/// byte 0x5b57c0, record +0x69), picks the team (0x6e2000 with 0x8800, or 0x408800 when a human manager is to be
/// consulted: 0x600a50, 0x5fd210, 0x5fea50) and gives each picked record the position of the place it won, redoing
/// performance (0x6eecb0). 0x6e0230 and 0x6e2000 (pick, recursive, allocates) are units with vectors: the tape records
/// the bytes of the input and output vectors passed by pointer, checks the Rust copy's inputs and writes its outputs
/// (`chamar_gravando_vetores`, `pedir_vetores`). The provisional 0x6e1aa0 port from batch 114 is gone
/// Original: `redo_team` (0x6e1aa0, match_man.cpp, thiscall).
///
/// [pt] { 0x6e1aa0 } (match_man.cpp): ver `refazer_time_com`.
unsafe extern "thiscall" fn refazer_time(c: usize) {
    refazer_time_com(c, &mut |rotina, this, a| match rotina {
        0x6e0230 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC);
            f(this, a[0] as usize, a[1], a[2], a[3]) as u32
        }
        0x6e2000 => {
            let f: Escalar = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
            let v = *(a[0] as usize as *const [usize; 15]);
            f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14])
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}



// ---------- centesima decima terceira leva: o mais bem avaliado da lista (0x6e1940) ----------

/// { 0x6e1940 } (ret 8, match_man.cpp): entre as `n` pessoas de `lista` (com ficha +0x61), a mais
/// bem avaliada pela comissao (0x6e1cf0 com o clube +0x2f); 0 com `n` <= 0 ou nenhuma. Com o objeto
/// +0x1f, se o valor dele (0x544410 com o byte +0x17) ficar abaixo do id da pessoa mod 20, a nota
/// cai para (1,1 - 0,01 x (endereco do objeto mod 20)) x nota, como no original. Empate fica com
/// a primeira.
unsafe extern "thiscall" fn melhor_da_lista(this: usize, lista: *const usize, n: i8) -> usize {
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let valor: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let (mut melhor, mut nota) = (0usize, 0i16);
    for i in 0..n.max(0) as usize {
        let p = *lista.add(i);
        if p == 0 || *((p + 0x61) as *const usize) == 0 {
            continue;
        }
        let mut v = avaliar(p, 0, *((this + 0x2f) as *const usize)) as i32;
        let x = *((this + 0x1f) as *const usize);
        if x != 0 && (valor(x, byte(x, 0x17) as u8 as u32) as i32) < *(p as *const i32) % 20 {
            let m = (x as i32) % 20;
            v = ftol((f64_em(0x9689c0) - m as f64 * f64_em(0x967898)) * v as i16 as f64);
        }
        if melhor == 0 || v as i16 > nota {
            melhor = p;
            nota = v as i16;
        }
    }
    melhor
}

// ---------- centesima decima segunda leva: o mais bem avaliado do lado (0x6e1a40) ----------

/// the index (0..20) of the active player (+0x19 ≥ 0) best rated by the staff (0x6e1cf0 with club +0x2f) among the 20
/// in the block starting at +0x23; the first active one counts even with a negative rating, then only strictly higher;
/// −1 with none active
/// Original: `best_rated` (0x6e1a40, match_man.cpp, thiscall).
///
/// [pt] { 0x6e1a40 } (match_man.cpp): o indice (0..20) do jogador ativo (+0x19 >= 0) mais bem avaliado
/// pela comissao (0x6e1cf0 com o clube +0x2f) entre os 20 do bloco que comeca em +0x23 (de 0x1be em
/// 0x1be); o primeiro ativo entra mesmo com nota negativa, depois so nota estritamente maior. -1 sem
/// nenhum ativo.
unsafe extern "thiscall" fn melhor_avaliado(this: usize) -> i8 {
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let clube = *((this + 0x2f) as *const usize);
    let mut p = *((this + 0x23) as *const usize);
    let (mut melhor, mut nota) = (-1i8, 0i16);
    for k in 0..20i8 {
        if byte(p, 0x19) >= 0 {
            let v = avaliar(0, p, clube);
            if melhor == -1 || v > nota {
                nota = v;
                melhor = k;
            }
        }
        p += 0x1be;
    }
    melhor
}

// ---------- centesima decima leva: a avaliacao do jogador pela comissao (0x6e1cf0) ----------

/// { 0x6e1cf0 } (cdecl, match_man.cpp): a nota (i16) que a comissao da a `pessoa` (ou a pessoa do
/// jogador `jogador` +0x69) para o `clube`; -10000 sem pessoa. Soma o encaixe (0x53ce90) em
/// (encaixe^2 + 5) x 0,1 negativo, a diferenca da nota +0xd da ficha para a reputacao do clube
/// (x 0,002 + 1,5, entre -4 e 4), o contrato (0x4cf5f0: +0x44 / 75 + 0,5, ajustado pelo tipo em +0x4f
/// >> 4: -10, +2,5, +2, -3, -15, -20, -50) e a ficha +0x2f vezes o fator da idade (dias de vida pela
/// data de hoje 0xae2c90 e a de nascimento +0x3e, 0x54cdb0; nas selecoes, +0x22 x 75): acima de 1000
/// dias, 1 + (dias - 1000) x 0,000125. Depois pela idade +0x18 (x0,5 abaixo de 18, x0,75 abaixo de
/// 21, x0,9 abaixo de 24, x1,05 acima de 25), x0,9 se os dias ficarem abaixo de (50 - +0x59) x 10 e
/// x(0,01 x +0x5b + 0,9) com +0x5b abaixo de 10.
unsafe extern "C" fn avaliar_jogador(pessoa: usize, jogador: usize, clube: usize) -> i16 {
    let encaixe: unsafe extern "C" fn(usize, usize) -> i8 = core::mem::transmute(FN_DATABASE_GET_CONTRACT_RATING_MODIFIER);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let d8 = |a: usize| f64_em(a);
    let p = if jogador != 0 { *((jogador + 0x69) as *const usize) } else { pessoa };
    if p == 0 {
        return -10000;
    }
    let ficha = *((p + 0x61) as *const usize);
    let enc = encaixe(p, clube) as f64;
    let mut y = (*((ficha + 0xd) as *const i16) as i32 - *((clube + 0x80) as *const i16) as i32) as f64 * d8(0x969048) + d8(0x968940);
    if !(d8(0x96d220) <= y) {
        y = -4.0;
    } else if d8(0x968938) < y {
        y = 4.0;
    }
    let mut x = 0.0f64;
    let e = contrato(*(0xae1050 as *const usize), p, clube);
    if e != 0 {
        x = byte(e, 0x44) as f64 * d8(0x968ef8) + d8(0x968968);
        let ajuste = match byte(e, 0x4f) as u8 >> 4 {
            0 | 5 => -d8(0x9688b0),
            1 => d8(0x968e18),
            2 => d8(0x968950),
            4 => -d8(0x968930),
            6 => -d8(0x968e08),
            7 => -d8(0x968ea8),
            8 => -d8(0x968ff8),
            _ => 0.0,
        };
        if ajuste != 0.0 {
            x += ajuste;
        }
    }
    let a = if selecao(clube) != 0 {
        *((p + 0x22) as *const u8) as f64 * d8(0x96d520)
    } else {
        dias(0xae2c90, *((p + 0x3e) as *const u32), *((p + 0x42) as *const u32)) as f64
    };
    let f = if a > d8(0x968908) { (a - d8(0x968908)) * d8(0x96a6b8) + d8(0x967890) } else { 1.0 };
    let base = byte(ficha, 0x2f) as f64 * f - (enc * enc + d8(0x9688a0)) * d8(0x967880);
    let mut v = ftol(base + y + x + d8(0x968968)) as i16;
    let idade = byte(p, 0x18);
    let k = if idade < 0x12 {
        Some(d8(0x968968))
    } else if idade < 0x15 {
        Some(d8(0x969040))
    } else if idade < 0x18 {
        Some(d8(0x9689d8))
    } else if idade > 0x19 {
        Some(d8(0x9689b8))
    } else {
        None
    };
    if let Some(k) = k {
        v = ftol(v as f64 * k) as i16;
    }
    if a < ((0x32 - byte(p, 0x59) as i32) * 10) as f64 {
        v = ftol(v as f64 * d8(0x9689d8)) as i16;
    }
    let moral = byte(p, 0x5b);
    if moral < 10 {
        v = ftol((moral as f64 * d8(0x967898) + d8(0x9689d8)) * v as f64) as i16;
    }
    v
}

// ---------- 173a leva: o fechamento do motor (7): a regra do elenco e as notas do jogador (match_man.cpp) ----------

/// applies the squad rule (foreigners, non-EU or age): swaps restricted players for the candidates with the smallest
/// score loss
/// Original: `apply_squad_rule` (0x6e8910, match_man.cpp, cdecl).
///
/// [pt] { 0x6e8910 } (cdecl, match_man.cpp): cumpre a `regra` do elenco (byte 0: 1 estrangeiro de fora da
/// comunidade, 2 estrangeiro, 3 acima da idade em +1; +2 máximo em campo, +3 máximo no banco, +4 e +5
/// o permitido e o atual) na escalação do `clube` (nação +0x53; 0x53d980/0x53d740): tira os restritos
/// da lista de reserva (`reserva`, `nr`); se há mais restritos em campo que o máximo (+2), escolhe entre
/// os titulares restritos e os candidatos (banco não restrito + reserva) pela 0x6e2000 e troca, um a
/// um, o de menor perda na nota (0x6e2f20), refazendo o banco (0x6e2000) se um titular saiu dele;
/// depois faz o mesmo no banco (`saida` + `nt`) com o máximo +3 (ou +4 − +5).
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn cumprir_regra_do_elenco(
    clube: usize, tecnico: usize, regra: usize, modo: u32, nt: u32, nb: u32, nr: u32, titulares: usize, banco: usize, reserva: usize,
    saida: usize, codigos: usize, curtos: usize, marcas: usize, bytes: usize, lado: usize,
) {
    let r = |k: usize| *((regra + k) as *const i8);
    if r(0) == 0 || r(5) >= r(4) {
        return;
    }
    let escalar: Escalar = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let restrito = |p: usize| -> bool {
        let estrangeiro: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        let comunitario: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
        let nacao = *((clube + 0x53) as *const u32);
        match r(0) {
            1 => p != 0 && estrangeiro(p, nacao, 1) != 0 && comunitario(p, nacao) == 0,
            2 => p != 0 && estrangeiro(p, nacao, 1) != 0,
            3 => p != 0 && r(1) < *((p + 0x18) as *const i8),
            _ => false,
        }
    };
    let (nt, nb) = (nt as u8 as i8, nb as u8 as i8);
    let mut nr = nr as u8 as i8;
    let res = |k: i8| (reserva as isize + 4 * k as isize) as *mut usize;
    let sai = |k: i32| (saida as isize + 4 * k as isize) as *mut usize;
    let w = |base: usize, k: i32| *((base as isize + 2 * k as isize) as *const u16);
    // tira `alvo` (ou a posição `k`) da reserva, pondo o último no lugar
    let tirar = |k: i8, nr: &mut i8| {
        if (k as i32) < *nr as i32 - 1 {
            *res(k) = *res(*nr - 1);
            *res(*nr - 1) = 0;
        } else {
            *res(k) = 0;
        }
        *nr -= 1;
    };
    let tirar_alvo = |alvo: usize, nr: &mut i8| {
        let mut k = 0i8;
        if 0 < *nr {
            loop {
                if *res(k) == alvo {
                    tirar(k, nr);
                    break;
                }
                k += 1;
                if k >= *nr {
                    break;
                }
            }
        }
    };
    let marca = if lado != 0 { *((*((lado + 0x2b) as *const usize) + 0x584) as *const u32) } else { 0 };
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserva == 0 {
                break;
            }
            if restrito(*res(k)) {
                let ultimo = (k as i32) < nr as i32 - 1;
                tirar(k, &mut nr);
                if ultimo {
                    k -= 1;
                }
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
    }
    let mut vaga = [0i8; 52];
    let mut cod = [0u16; 50];
    let mut cod2 = [0u16; 50];
    let mut m8 = [0u32; 100];
    let mut b1 = [0u8; 52];
    let mut cand = [0usize; 50];
    let mut escolha = [0usize; 50];
    let mut falta = r(5);
    if falta < r(2) {
        let mut trocas = r(2) - falta;
        falta = r(3);
        if trocas != 0 {
            let mut nv = 0i8;
            let mut k = 0i8;
            while k < nt {
                if restrito(*((titulares + 4 * k as usize) as *const usize)) {
                    let i = nv as usize;
                    vaga[i] = k;
                    cod[i] = w(codigos, k as i32);
                    cod2[i] = if curtos == 0 { 0 } else { w(curtos, k as i32) };
                    (m8[2 * i], m8[2 * i + 1]) = if marcas == 0 { (0, 0) } else { (*((marcas + 8 * k as usize) as *const u32), *((marcas + 8 * k as usize + 4) as *const u32)) };
                    b1[i] = if bytes == 0 { 0 } else { *((bytes + k as usize) as *const u8) };
                    nv += 1;
                }
                k += 1;
            }
            let mut nc = 0i8;
            for k in 0..nb.max(0) as usize {
                let q = *((banco + 4 * k) as *const usize);
                if q != 0 && !restrito(q) {
                    cand[nc as usize] = q;
                    nc += 1;
                }
            }
            let mut k = 0i8;
            if 0 < nr {
                loop {
                    if reserva == 0 {
                        break;
                    }
                    cand[nc as usize] = *res(k);
                    nc += 1;
                    k += 1;
                    if k >= nr {
                        break;
                    }
                }
            }
            if nc != 0 {
                escalar(cand.as_ptr() as usize, nc as u8 as u32, cod.as_ptr() as usize, nv as u8 as u32, 0, escolha.as_mut_ptr() as usize, modo, clube, lado,
                    tecnico as u32, 0, m8.as_ptr() as usize, b1.as_ptr() as usize, marca, cod2.as_ptr() as usize);
                let mut melhor = 0i32;
                let mut saiu_do_banco = false;
                loop {
                    if nv == 0 || nv < 1 {
                        break;
                    }
                    let mut esc = -1i32;
                    for k in 0..nv as usize {
                        if vaga[k] == -1 {
                            continue;
                        }
                        let atual = *sai(vaga[k] as i32);
                        if atual == 0 {
                            continue;
                        }
                        let valer = |p: usize| nota(p, *((p + 0x61) as *const usize), 0, cod[k] as u32, modo, clube, lado, tecnico as u32, u32::MAX,
                            m8[2 * k], m8[2 * k + 1], b1[k] as u32, marca, cod2[k] as u32, 0);
                        let a = valer(atual);
                        let b = if escolha[k] == 0 { -1000000 } else { valer(escolha[k]) };
                        if esc == -1 || a.wrapping_sub(b) < melhor {
                            melhor = a.wrapping_sub(b);
                            esc = k as i32;
                        }
                    }
                    if esc < 0 {
                        break;
                    }
                    let e = esc as usize;
                    *sai(vaga[e] as i32) = escolha[e];
                    tirar_alvo(escolha[e], &mut nr);
                    for k in 0..nb.max(0) as usize {
                        let q = (banco + 4 * k) as *mut usize;
                        if escolha[e] != 0 && *q == escolha[e] {
                            saiu_do_banco = true;
                            *q = 0;
                        }
                    }
                    vaga[e] = -1;
                    trocas -= 1;
                    if trocas == 0 {
                        break;
                    }
                }
                if saiu_do_banco {
                    let mut n = 0i8;
                    for k in 0..nb.max(0) as usize {
                        let q = *((banco + 4 * k) as *const usize);
                        if q != 0 {
                            escolha[n as usize] = q;
                            n += 1;
                        }
                    }
                    for k in 0..nr.max(0) {
                        if *res(k) != 0 {
                            escolha[n as usize] = *res(k);
                            n += 1;
                        }
                    }
                    if n != 0 {
                        escalar(escolha.as_ptr() as usize, n as u8 as u32, codigos + 2 * nt as isize as usize, nb as u8 as u32, 0, sai(nt as i32) as usize, modo,
                            clube, lado, tecnico as u32, 0, 0, 0, 0, curtos.wrapping_add(2 * nt as isize as usize));
                    }
                }
            }
        }
    } else {
        falta = r(4) - falta;
    }
    if falta == 0 {
        return;
    }
    let mut nv = 0i8;
    let mut k = 0i8;
    while k < nb {
        let j = nt as i32 + k as i32;
        if restrito(*sai(j)) {
            let i = nv as usize;
            cod[i] = w(codigos, j);
            cod2[i] = w(curtos, j);
            vaga[i] = k;
            nv += 1;
        }
        k += 1;
    }
    let mut nc = 0i8;
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserva == 0 {
                break;
            }
            let q = *res(k);
            if !(0..nb.max(0) as i32).any(|j| q == *sai(j + nt as i32)) {
                cand[nc as usize] = q;
                nc += 1;
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
        if nc != 0 {
            escalar(cand.as_ptr() as usize, nc as u8 as u32, cod.as_ptr() as usize, nv as u8 as u32, 0, escolha.as_mut_ptr() as usize, modo, clube, lado,
                tecnico as u32, 0, 0, 0, 0, cod2.as_ptr() as usize);
            let mut melhor = 0i32;
            loop {
                if nv == 0 || nv < 1 {
                    return;
                }
                let mut esc = -1i32;
                for k in 0..nv as usize {
                    if vaga[k] == -1 {
                        continue;
                    }
                    let atual = *sai(vaga[k] as i32 + nt as i32);
                    if atual == 0 {
                        continue;
                    }
                    let valer = |p: usize| nota(p, *((p + 0x61) as *const usize), 0, cod[k] as u32, modo, clube, lado, tecnico as u32, u32::MAX, 0, 0, 0, 0, cod2[k] as u32, 0);
                    let a = valer(atual);
                    let b = if escolha[k] == 0 { -1000000 } else { valer(escolha[k]) };
                    if esc == -1 || a.wrapping_sub(b) < melhor {
                        melhor = a.wrapping_sub(b);
                        esc = k as i32;
                    }
                }
                if esc < 0 {
                    return;
                }
                let e = esc as usize;
                *sai(vaga[e] as i32 + nt as i32) = escolha[e];
                tirar_alvo(escolha[e], &mut nr);
                vaga[e] = -1;
                falta -= 1;
                if falta == 0 {
                    break;
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_cumprir_regra_do_elenco(
    clube: usize, tecnico: usize, regra: usize, modo: u32, nt: u32, nb: u32, nr: u32, titulares: usize, banco: usize, reserva: usize,
    saida: usize, codigos: usize, curtos: usize, marcas: usize, bytes: usize, lado: usize,
) {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, usize)>(391);
    if gravando_no_topo() {
        return o(clube, tecnico, regra, modo, nt, nb, nr, titulares, banco, reserva, saida, codigos, curtos, marcas, bytes, lado);
    }
    let n = |v: u32| (v as u8 as i8).max(0) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 4 * (n(nt) + n(nb))));
    if banco != 0 {
        reg.push((banco, 4 * n(nb)));
    }
    if reserva != 0 {
        reg.push((reserva, 4 * n(nr)));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || o(clube, tecnico, regra, modo, nt, nb, nr, titulares, banco, reserva, saida, codigos, curtos, marcas, bytes, lado),
        || crate::aleatorio::sem_contar(|| cumprir_regra_do_elenco(clube, tecnico, regra, modo, nt, nb, nr, titulares, banco, reserva, saida, codigos, curtos, marcas, bytes, lado)),
    );
    PLACAR.registrar(igual, || format!("cumprir_regra_do_elenco(regra {:x?}) {:x?} x {:x?}", core::slice::from_raw_parts(regra as *const u8, 6), mo, mr));
}
// ---- as notas do jogador do match_man.cpp (0x6e6340 fora da partida, 0x6e2f20 para uma vaga): partes comuns ----

/// A data (dia +0, ano +2) de `a` vem antes da de `b`? (comparação do ano sem sinal, do dia com sinal)
unsafe fn data_antes(a: usize, b: usize) -> bool {
    let (aa, ab) = (*((a + 2) as *const u16), *((b + 2) as *const u16));
    aa < ab || (aa == ab && *(a as *const i16) < *(b as *const i16))
}

/// Os dias desde a data de `p` (+0x3e) até hoje ([0xae2c90]), contando 365 por ano.
unsafe fn dias_desde(p: usize) -> i32 {
    let anos = *(0xae2c92 as *const u16) as i32 - *((p + 0x40) as *const u16) as i32;
    anos * 365 - *((p + 0x3e) as *const i16) as i32 + *(0xae2c90 as *const i16) as i32
}

/// O observador `o` (+0x3e) chegou antes da data do jogador `p` (+0x3e) e esta data é de mais de um
/// ano atrás (0x54c8e0 com 365)?
unsafe fn observado_ha_mais_de_um_ano(o: usize, p: usize) -> bool {
    if !data_antes(o + 0x3e, p + 0x3e) {
        return false;
    }
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> *const [u8; 8] = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut d = [0u8; 8];
    let r = datar(0xae2c90, &mut d, 0x16d);
    data_antes(r as usize, p + 0x3e)
}

/// Byte com sinal em `p + k`.
unsafe fn sb(p: usize, k: usize) -> i32 {
    *((p + k) as *const i8) as i32
}

/// A 0x7c1e30 das notas ([0xb74350]) da ficha `f`.
unsafe fn nota_externa(f: usize, k: u32, c: u32, x: f32) -> f64 {
    let g: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    g(*(0xb74350 as *const usize), f, k, c, x)
}

/// A nota da posição (máscara `m`) pelos atributos da ficha `f` na tabela `tab` (0xa15d50 ou
/// 0xa15cfc). A posição principal tem os casos de `m` igual a 8 ou 0x10 e a exceção de lateral
/// (0x800/0x80) na faixa 0x10; a secundária da 0x6e2f20 não.
unsafe fn nota_da_posicao(f: usize, m: u32, masc: u32, tab: usize, principal: bool) -> i32 {
    let fb = |k: usize| sb(f, k);
    let t = |k: i32| *((tab as isize + 4 * k as isize) as *const i32);
    let m16 = m as u16;
    let sem_ala = m & 0x800 == 0 && m & 0x80 == 0;
    if m & 1 != 0 {
        fb(0xf) * fb(0xf) - 300
    } else if masc & 1 != 0 {
        -200
    } else if m & 2 != 0 {
        t(fb(0x10))
    } else if m & 4 != 0 {
        t(fb(0x11))
    } else if principal && m16 == 8 {
        t(fb(0x16).max(fb(0x11)).max(fb(0x12)).max(fb(0x13)))
    } else if m & 8 != 0 {
        t(if sem_ala { fb(0x12) } else { fb(0x16) })
    } else if m & 0x10 != 0 {
        if !principal || sem_ala {
            if principal && m16 == 0x10 {
                t(fb(0x14).max(fb(0x12)).max(fb(0x13)))
            } else {
                let (a, c, d) = (fb(0x14) - 5, fb(0x13), fb(0x12) - 5);
                let al = if a.max(c) > d { if a > c { a } else { c } } else { d };
                t(al as i8 as i32)
            }
        } else {
            t(fb(0x14).max(fb(0x13)))
        }
    } else if m & 0x20 != 0 {
        if sem_ala {
            let a = fb(0x14) as f64;
            let meio = (fb(0x14) + fb(0x1a)) as f64 * f64_em(0x968968);
            t(ftol(if a <= meio { meio } else { a }))
        } else {
            t(fb(0x14))
        }
    } else if m & 0x40 != 0 {
        t(fb(0x15))
    } else {
        100
    }
}

/// O ajuste da posição (máscara `m`) pelos atributos de jogo da ficha `f`; 100 sem nenhum.
unsafe fn ajuste_da_posicao(f: usize, m: u32) -> i32 {
    let fb = |k: usize| sb(f, k);
    if m & 0x1000 != 0 {
        (fb(0x1a) - 10) * 10
    } else if m & 0x80 != 0 {
        (fb(0x18) - 10) * 10
    } else if m & 0x100 != 0 {
        fb(0x30) + ((fb(0x18) / 2).max(fb(0x19)) * 5 - 0x3c) * 2
    } else if m & 0x200 != 0 {
        (fb(0x19) - 10) * 10
    } else if m & 0x400 != 0 {
        fb(0x3b) + ((fb(0x17) / 2).max(fb(0x19)) * 5 - 0x3c) * 2
    } else if m & 0x800 != 0 {
        (fb(0x17) - 10) * 10
    } else {
        100
    }
}

/// O acréscimo da média das notas (0x7c1950 x 0,2; com o observador `quem` de mais de um ano no
/// clube do jogador, pesada pelos dias) à soma, por `cont`.
unsafe fn bonus_da_media(p: usize, f: usize, clube: usize, quem: usize, cont: i8) -> i32 {
    let media: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(FN_PLAYERSTATS_GET_RECENT_FORM);
    let mut x = (media(*(0xb74350 as *const usize), f) as f64 * f64_em(0x968918)) as f32;
    if clube == *((p + 0x39) as *const usize) && quem != 0 && observado_ha_mais_de_um_ano(quem, p) {
        let d = dias_desde(p) as f64;
        x = (((f64_em(0x969070) - d) * f64_em(0x968eb0) + d * x as f64) * f64_em(0x968e10)) as f32;
    }
    let x = x as f64;
    if x > f64_em(0x968998) {
        ftol((x * f64_em(0x96d638) - f64_em(0x96d630)) * cont as f64) as i8 as i32
    } else if x > f64_em(0x9689a0) {
        ftol((x * f64_em(0x96d628) - f64_em(0x96d620)) * cont as f64) as i8 as i32
    } else {
        0
    }
}

/// O acréscimo da forma (flag 2) à soma: a nota 0x7c1e30(6) de um teto pela média (ou por `l3c`),
/// limitado para os de mais de 33 anos quando `teto_veterano`, pesado pelos dias com o observador
/// `quem`, e as notas 0x7c1e30(1) e 0x7c1e30(6, 0x14, `ultimo`) no clube (ou mesma nação).
#[allow(clippy::too_many_arguments)]
unsafe fn bonus_da_forma(p: usize, f: usize, clube: usize, quem: usize, idade: i8, cont: i8, soma: i32, l3c: i32, teto_veterano: bool, ultimo: u32) -> i32 {
    let notas = *(0xb74350 as *const usize);
    let teto = |v: i32| {
        let y = v as f64 * f32_em(0x96d61c) as f64 + f32_em(0x96d618) as f64;
        (if y > f64_em(0x96d610) { f64_em(0x96d610) } else { y }) as f32
    };
    let razao = || {
        let (m10, m18) = (*((notas + 0x10) as *const f64), *((notas + 0x18) as *const f64));
        if m18 == f64_em(0x9689a0) { f64_em(0x968998) } else { m10 / m18 }
    };
    let mut x = f32::from_bits(0x40d9999a);
    if cont != 0 && soma / cont as i32 > 0x96 {
        x = teto(soma / cont as i32 - 0x96);
    } else if l3c as i16 > 0x96 {
        x = teto(l3c as i16 as i32 - 0x96);
    }
    let mut x = nota_externa(f, 6, 0x12, x) as f32;
    if teto_veterano && idade > 0x21 {
        let r = razao() + f64_em(0x968fd0);
        if (x as f64) > r {
            x = r as f32;
        }
    }
    if clube == *((p + 0x39) as *const usize) && quem != 0 && observado_ha_mais_de_um_ano(quem, p) {
        let d = dias_desde(p) as f64;
        x = (((f64_em(0x969070) - d) * f64_em(0x968eb0) + x as f64 * d) * f64_em(0x968e10)) as f32;
        let fb_p = sb(*((p + 0x61) as *const usize), 0xf);
        if idade as i32 > fb_p / 10 + 0x22 && x as f64 > razao() {
            x = ((razao() + x as f64) * f32_em(0x968f18) as f64) as f32;
        }
    }
    let xf = x as f64;
    let c = cont as f64;
    let mut soma = soma;
    if xf > f64_em(0x968f10) || (xf <= f64_em(0x968998) && xf > f64_em(0x9689a0)) {
        soma += ftol((xf * f32_em(0x96af88) as f64 - f32_em(0x96d608) as f64) * c) as i8 as i32;
    } else if xf > f64_em(0x968998) {
        soma += ftol((xf * f64_em(0x96afc8) - f64_em(0x96d510)) * c) as i8 as i32;
    }
    let dono = *((p + 0x39) as *const usize);
    if clube != 0 && (clube == dono || (dono != 0 && *((clube + 0x53) as *const usize) == *((dono + 0x53) as *const usize))) {
        let xk = (xf * f64_em(0x9689d8)) as f32;
        let r = nota_externa(f, 1, 0x12, xk);
        if r > f64_em(0x968998) {
            soma += ftol((r * f64_em(0x96d628) - f64_em(0x96d600)) * c) as i8 as i32;
        } else if r < xk as f64 {
            soma += ftol((r * f32_em(0x96d5f8) as f64 / xk as f64 - f32_em(0x96d5f8) as f64) * c) as i8 as i32;
        }
        soma += ftol((nota_externa(f, 6, 0x14, f32::from_bits(ultimo)) * f64_em(0x968e30) - f64_em(0x96d5f0)) * c);
    }
    soma
}

/// O desconto do histórico do jogador (0x62ed70, +0x28).
unsafe fn desconto_do_historico(p: usize, l4c: i32) -> i32 {
    let historico: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let h = historico(*(0xb63d28 as *const usize), p);
    if h == 0 {
        return l4c;
    }
    let w = core::ptr::read_unaligned((h + 0x28) as *const i16) as i32;
    l4c + if w <= 10 { -2 * w } else { -20 - w }
}

/// A reputação do jogador (0x62c3d0 pelo id).
unsafe fn reputacao(p: usize) -> i16 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_INJURY_GET_FATIGUE);
    g(*(0xb63d28 as *const usize), *(p as *const u32))
}

/// O fim da flag 0x4000 (comum às duas notas): a idade com +0x100, a soma das metades dos
/// atributos (sem goleiro), +0x2f e o fator do estrangeiro (0x7c6830 e a tabela por continente).
#[allow(clippy::too_many_arguments)]
unsafe fn fim_da_idade(p: usize, f: usize, clube: usize, flags: u32, pos: u32, masc: u32, idade: i8, cont: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| sb(f, k);
    if flags & 0x100 != 0 {
        let cl = if masc & 1 != 0 { 0x1d } else { 0x19 };
        let y = f64_em(0x968e38) - cont as f64 * f64_em(0x968f80);
        if idade as i32 > cl && y > f64_em(0x9689a0) {
            l4c -= ftol((idade as i32 - cl) as f64 * y);
        }
    }
    if pos & 1 == 0 {
        let s: i32 = (0x10..=0x19).map(|k| fb(k) / 2).sum();
        if s > 0x23 {
            l4c += (s - 20) * (s - 20) / 250;
        }
    }
    l4c += ftol((fb(0x2f) - 10) as f64 * f64_em(0x96a670));
    let nac = *((clube + 0x53) as *const usize);
    let pais = *((p + 0x1a) as *const usize);
    if nac == 0 || *((nac + 0x77) as *const u8) == *((pais + 0x77) as *const u8) {
        return l4c;
    }
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_PLAYERSTATS_GET_LANGUAGE_FLUENCY);
    let mut c = g(*(0xb74350 as *const usize), p, clube, (nac as u32 & !0xff) | *((nac + 0x77) as *const u8) as u32, 1) as u8 as i8;
    if flags & 0x100 != 0 {
        let k = (idade as u8).wrapping_sub((*((p + 0x56) as *const i8) / 2) as u8).wrapping_add(5) as i8;
        let m = f64_em(0x96d5d0) - k as f64 * f64_em(0x96d0e8);
        c = ftol(if c as f64 > m { c as f64 } else { m }) as i8;
    } else {
        let y = l4c as f64 * f64_em(0x96d1e0) - f64_em(0x9688a0);
        if (c as f64) < y {
            c = ftol(y) as i8;
        }
    }
    let b22 = *((p + 0x22) as *const u8) as i32;
    if b22 != 0 && b22 * b22 + core::ptr::read_unaligned((pais + 0x8e) as *const i16) as i32 > 5000 && c <= 5 {
        c = 5;
    }
    if (0..=9).contains(&c) {
        const IGUAL: [usize; 10] = [0x96a6a8, 0x96d588, 0x96d590, 0x96d598, 0x96d5a0, 0x96d5a8, 0x96d5b0, 0x96d5b8, 0x96d5c0, 0x96d5c8];
        const OUTRO: [usize; 10] = [0x96d588, 0x96d590, 0x96d598, 0x96d5a0, 0x96d5a8, 0x96d5b0, 0x96d5b8, 0x96d5c0, 0x96d5c8, 0x968e50];
        let t = if *((pais + 0x76) as *const u8) == 0xe { &IGUAL } else { &OUTRO };
        l4c = ftol(l4c as f64 * f64_em(t[c as usize]));
    }
    l4c
}

/// O ajuste pelo contrato (flag 0x2000000): sem clube (+0x39) há mais de 60 dias, o salário (+0x80)
/// do clube contra o do dono e a satisfação (+0x3d do contrato), a idade e o goleiro, a lesão
/// (0x5444a0) e a idade com os jogos (+0x22).
unsafe fn ajuste_do_contrato(p: usize, f: usize, clube: usize, contrato: usize, idade: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| sb(f, k);
    let mul = |l: i32, k: f64| ftol(l as f64 * k);
    let dono = *((p + 0x39) as *const usize);
    if dono == 0 {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        if dias(0xae2c90, *((p + 0x3e) as *const u32), *((p + 0x42) as *const u32)) > 0x3c {
            l4c = mul(l4c, f64_em(0x969040));
        }
    } else {
        let lim = core::ptr::read_unaligned((dono + 0x80) as *const i16) as i32 - 0x4e2;
        let dx = core::ptr::read_unaligned((clube + 0x80) as *const i16);
        let ax = *((contrato + 0x3d) as *const i8) as i16;
        let cl = idade;
        let k = if dx as i32 > lim {
            let ax = ax.wrapping_add(0x64);
            if ax >= 0x7d {
                None
            } else {
                let mut st = f32_em(0x96d248) as f64;
                if dx != 0 {
                    st = lim as f64 / dx as f64 * ax as f64 * f32_em(0x96d5d8) as f64;
                    if st > f64_em(0x967890) {
                        st = f32_em(0x968e70) as f64;
                    } else if st < f64_em(0x969040) {
                        st = f32_em(0x96d248) as f64;
                    }
                }
                if cl > 0x22 {
                    st *= f64_em(0x969040);
                } else if cl > 0x1e {
                    st *= f64_em(0x969520);
                } else if cl > 0x1b {
                    st *= f64_em(0x9689d0);
                }
                if fb(0xf) == 0x14 {
                    st *= f64_em(if cl < 0x12 { 0x969040 } else if cl < 0x14 { 0x969520 } else { 0x9689d0 });
                }
                Some(st)
            }
        } else if ax >= 0x19 || cl <= 0x1e {
            None
        } else {
            let mut st = f32_em(if cl > 0x25 { 0x968f18 } else if cl > 0x22 { 0x96d248 } else { 0x96d364 }) as f64;
            if dx > 0x1482 && cl < 0x14 {
                if ax < -0x4b {
                    st = f32_em(0x968f18) as f64;
                } else if ax < -0x19 {
                    st = f32_em(0x96d320) as f64;
                } else if ax < 0 {
                    st = f32_em(0x96d364) as f64;
                }
            }
            if fb(0xf) == 0x14 {
                st *= f64_em(if cl < 0x12 { 0x969040 } else if cl < 0x15 { 0x969520 } else { 0x9689d0 });
            }
            Some(st)
        };
        if let Some(st) = k {
            l4c = mul(l4c, st);
        }
        let lesao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
        if lesao(p, 0) != 0 {
            l4c = mul(l4c, f64_em(0x9689d0));
        }
    }
    let (cl, al) = (idade, *((p + 0x22) as *const u8));
    // Some(0): a nota vira 5
    let fator: Option<usize> = if cl > 0x22 {
        if al == 0 {
            Some(0x9689e0)
        } else if al < 8 {
            Some(0x968968)
        } else if cl > 0x23 && al < 0x19 {
            Some(0x96a5d0)
        } else if al < 0xf {
            Some(0x969040)
        } else if al < 0x23 {
            Some(0x969520)
        } else {
            None
        }
    } else if cl > 0x1b {
        if al == 0 {
            Some(0x968968)
        } else if al < 8 {
            Some(0x96a5d0)
        } else if al < 0xf {
            Some(0x9689b0)
        } else {
            None
        }
    } else if cl > 0x18 {
        if al == 0 {
            Some(0x96a5d0)
        } else if al < 8 {
            Some(0x969040)
        } else if al < 0xf {
            Some(0x969520)
        } else if al > 0x14 {
            Some(0)
        } else {
            None
        }
    } else if cl > 0x15 {
        if al == 0 {
            Some(0x969520)
        } else if al < 6 {
            Some(0x9689d8)
        } else if al < 0xa {
            Some(0x9689d0)
        } else if al > 0x14 {
            Some(0)
        } else {
            None
        }
    } else if al > 0xa {
        Some(0)
    } else {
        None
    };
    match fator {
        Some(0) => 5,
        Some(k) => mul(l4c, f64_em(k)),
        None => l4c,
    }
}

/// O bônus do observador (flag 0x1000): o conhecimento `conhece` (+0x22 dos jovens, +0x14, +0x0e,
/// +0x1a, +0x1f, +0x13), o gosto do jogador pelo clube e por `quem`, e os atributos de cada faixa de
/// posição; `craque_meia` liga o bônus dos craques também na faixa 0x20 (só na 0x6e6340).
#[allow(clippy::too_many_arguments)]
unsafe fn bonus_do_observador(p: usize, f: usize, clube: usize, conhece: usize, obs: usize, quem: usize, pos: u32, masc: u32, idade: i8, mut l4c: i32, craque_meia: bool) -> i32 {
    let fb = |k: usize| sb(f, k);
    let kb = |k: usize| sb(conhece, k);
    if idade <= 0x17 {
        l4c += ftol((kb(0x22) - 10) as f64 * f64_em(0x96a670));
    }
    let g = |a: usize| -> unsafe extern "C" fn(usize, usize) -> u32 { core::mem::transmute(a) };
    if g(0x547d30)(p, clube) != 0 {
        let m = l4c as f64 * f64_em(0x969040);
        let x = (l4c - 10) as f64;
        l4c = ftol(if x < m { x } else { m });
    } else if g(0x547d60)(p, clube) != 0 {
        let m = l4c as f64 * f64_em(0x9689c0);
        let x = (l4c + 10) as f64;
        l4c = ftol(if x > m { x } else { m });
    }
    if g(0x547840)(p, quem) != 0 {
        l4c -= 50;
    } else if g(0x547810)(p, quem) != 0 {
        let x = l4c as f64 * f64_em(0x968928);
        let m = (l4c + 20) as f64;
        l4c = ftol(if x > m { x } else { m });
    }
    let f44 = fb(0x44);
    let k14 = kb(0x14);
    let mut ecx = (k14 * (f44 - 10) / 3).min(10) + ((sb(p, 0x5b) - 10) * k14 / 3).min(10) + ((sb(p, 0x59) - 10) * sb(obs, 0x59) / 3).min(10);
    if pos & 1 == 0 {
        let fora = pos & 0x80 == 0 && pos & 0x800 == 0;
        let e = kb(0xe);
        let k1a = kb(0x1a) * fb(0x32);
        let k1f = kb(0x1f) * (f44 - 10);
        // os craques: (+0x26 / 3) x +0x1b x +0x36 / 1500, com +0x26 > 30 e +0x36 > 15
        let craque = |ecx: i32| if fb(0x26) > 0x1e && fb(0x36) > 0xf { ecx + fb(0x26) / 3 * fb(0x1b) * fb(0x36) / 1500 } else { ecx };
        if pos & 6 != 0 {
            ecx += k1a / 40 + (fb(0x28) - 10) * e / 10 + fb(0x26) * e / 500 + fb(0x27) * e / 350 + fb(0x43) * e / 250 + k1f / 8 - 10;
            if fora {
                ecx += fb(0x2e) - 15;
            }
        } else if pos & 8 != 0 {
            ecx += k1a / 40 + (fb(0x28) - 10) * e / 8 + fb(0x26) * e / 325 + fb(0x43) * e / 150 + k1f / 6 + fb(0x27) / 15 - 10;
            if fora {
                ecx += fb(0x3e) / 15 + f44 - 15;
            }
        } else if pos & 0x10 != 0 {
            ecx += k1a / 60 + (fb(0x28) - 10) * e / 5 + fb(0x26) * e / 150 + fb(0x43) * e / 100 + k1f / 6 + fb(0x27) / 10 - 10;
            if fora {
                ecx += fb(0x3e) / 15 + fb(0x37) / 15 + f44 - 15;
            } else {
                ecx += fb(0x23) / 15;
            }
        } else if pos & 0x20 != 0 {
            ecx += k1a / 150 + (fb(0x28) - 10) * e / 4 + k1f / 8 + fb(0x43) * e / 75 + fb(0x26) * e / 150 + fb(0x27) / 8 - 10;
            if fora {
                ecx += fb(0x43) / 15 + fb(0x1a) - 15;
            } else {
                ecx += fb(0x23) / 15;
            }
            if craque_meia {
                ecx = craque(ecx);
            }
        } else if pos & 0x40 != 0 {
            ecx += k1a / 400 + (fb(0x28) - 10) * e / 3 + fb(0x43) * e / 200 + k1f / 10 + fb(0x26) * e / 175 + fb(0x27) / 5 - 10;
            if fora {
                ecx += fb(0x33) / 15 + fb(0x36) + fb(0x40) + fb(0x27) / 15 - 0x14;
                ecx = craque(ecx);
            }
        }
    }
    if masc & 0x40 != 0 && pos & 0x40 != 0 {
        ecx += (fb(0x36) * fb(0x2e) - 200) * kb(0x13) / 500;
    }
    if ecx != 0 {
        l4c += ftol((f64_em(0x967890) - l4c as f64 * f64_em(0x96d580)) * ecx as f64).clamp(-0x50, 0x50);
    }
    l4c
}

/// Na lista de transferências ([0xde1f54], 0x8c7260/0x8c7560 com o clube do contrato, que fica em
/// [0xae1080]): a porcentagem +4 / (+4 + +6) em byte (0 sem nenhum); None fora da lista ou de 70 em
/// diante.
unsafe fn na_lista_de_transferencias(p: usize) -> Option<i8> {
    let lista = *(0xde1f54 as *const usize);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_WORK_PERMIT);
    let x = busca(lista, p);
    if x == 0 {
        return None;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato_de(*(0xae1050 as *const usize), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c == 0 { 0 } else { *(0xae23b4 as *const usize) + *((c + 4) as *const usize) * 0x245 };
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if pode(lista, p, dono, 1) == 0 {
        return None;
    }
    let (a, b) = (*((x + 4) as *const u16) as i32, *((x + 6) as *const u16) as i32);
    if a + b == 0 {
        return Some(0);
    }
    let bl = (a * 100 / (a + b)) as i8;
    if bl >= 0x46 { None } else { Some(bl) }
}

/// O extra do contrato (flag 0x10000): +0x44 pelas bandeiras +0x45, o tipo (+0x4f >> 4) e a lista
/// de transferências.
unsafe fn extra_do_contrato(p: usize, contrato: usize, flags: u32, l28: i32) -> i32 {
    let e = *((contrato + 0x45) as *const u32);
    let s44 = *((contrato + 0x44) as *const i8) as f64;
    let mut extra = 0i32;
    if e & 0x2808 != 0 {
        let x = s44 * f64_em(0x96a670);
        extra = ftol(if f64_em(0x96d578) < x { f64_em(0x96d578) } else { x });
    } else if e != 0 {
        let x = s44 * f64_em(0x968e00);
        extra = ftol(if f64_em(0x96d410) < x { f64_em(0x96d410) } else { x });
    }
    let t = (*((contrato + 0x4f) as *const u8) >> 4) as i8 as i32 - 1;
    let lista = match t {
        0 | 1 => {
            extra += if t == 0 { 0x19 } else { 0xf };
            flags & 0x800000 == 0
        }
        2 => {
            if flags & 0x800000 != 0 {
                extra += 10;
                false
            } else {
                let d = (*((contrato + 0x3e) as *const i8) as i32 - *((contrato + 0x3f) as *const i8) as i32) * 5;
                extra += ftol(d as f64 + (l28 as i16 as f64 * f64_em(0x968e18) - f64_em(0x96d570)));
                true
            }
        }
        6 => {
            extra -= 0x14;
            false
        }
        _ => false,
    };
    if lista {
        if let Some(bl) = na_lista_de_transferencias(p) {
            extra += 0x32 - bl as i32 / 2;
        }
    }
    extra
}

/// O vínculo (flag 0x20000) pelo tipo do contrato `c`: -25 a +25, ou pela lista de transferências.
unsafe fn vinculo(p: usize, c: usize, flags: u32) -> i32 {
    let t = (*((c + 0x4f) as *const u8) >> 4) as i8 as i32 - 1;
    match t {
        0..=2 => {
            if flags & 0x800000 != 0 {
                -25
            } else {
                match na_lista_de_transferencias(p) {
                    Some(bl) => bl as i32 / 2 - 0x4b,
                    None => -25,
                }
            }
        }
        3 | 4 | 7 => 25,
        _ => 0,
    }
}

/// O ajuste pela reputação (0x62c3d0) abaixo de -1000 ou acima de 500.
unsafe fn ajuste_da_reputacao(p: usize, l4c: i32) -> i32 {
    if reputacao(p) < -1000 {
        l4c - ftol(reputacao(p) as f64 * f64_em(0x96a740) + f64_em(0x96afc8))
    } else if reputacao(p) > 0x1f4 {
        l4c + ftol(f64_em(0x968890) - reputacao(p) as f64 * f64_em(0x967888))
    } else {
        l4c
    }
}

/// Os jogos (+0x22, flag 0x100000) ou o clube (flag 4: o dono +0x39 e o +0x69 do treinador dele).
unsafe fn ajuste_dos_jogos(p: usize, f: usize, flags: u32, da_base: bool, l4c: i32) -> i32 {
    if flags & 0x100000 != 0 {
        return l4c - (*((p + 0x22) as *const u8) as i32 * 5).min(0x32);
    }
    if flags & 4 == 0 || !da_base {
        return l4c;
    }
    let dono = *((p + 0x39) as *const usize);
    if dono == 0 {
        return if core::ptr::read_unaligned((f + 9) as *const i16) < 0x1388 { l4c - 0x1e } else { l4c };
    }
    let s = *((dono + 0x57) as *const usize);
    if s == 0 {
        return l4c;
    }
    l4c + ftol((core::ptr::read_unaligned((s + 0x69) as *const i16) as i32 - 0x10) as f64 * f64_em(0x9688a0))
}

/// No próprio clube (+0x39): a nota sobe 5 ou 5%.
unsafe fn no_proprio_clube(p: usize, clube: usize, l4c: i32) -> i32 {
    if *((p + 0x39) as *const usize) != clube {
        return l4c;
    }
    let x = (l4c + 5) as f64;
    let m = l4c as f64 * f64_em(0x9689b8);
    ftol(if x > m { x } else { m })
}

/// A combinação da nota da posição `l40` com o ajuste `l4c` (só `l40` com as flags 0x800 sozinhas).
fn combinar_nota(flags: u32, l40: i32, l4c: i32) -> i32 {
    if flags == 0x800 {
        l40
    } else if l40 < 0 {
        (500 - l4c) * l40
    } else if l4c < 1 {
        l4c + l40
    } else {
        l40 * l4c
    }
}

/// the player's score for a position outside a match (transfers and scouting): position, form, perceived ability,
/// contract, age, scout knowledge
/// Original: `off_match_score` (0x6e6340, match_man.cpp, cdecl).
///
/// [pt] { 0x6e6340 } (cdecl, match_man.cpp): a nota do jogador `p` para a posição `pos` fora da partida
/// (transferências, observação), pelas `flags`, para o `clube`, com o `limite` de corte e o
/// observador `o`: a nota da posição (tabela 0xa15d50) vezes o ajuste da reputação, forma,
/// habilidade vista, idade, contrato e conhecimento do observador. -10000 sem ficha (+0x61) ou
/// abaixo do limite.
unsafe extern "C" fn nota_fora_da_partida(p: usize, pos: u32, flags: u32, clube: usize, limite: i32, o: usize) -> i32 {
    if p == 0 {
        return -10000;
    }
    let f = *((p + 0x61) as *const usize);
    if f == 0 {
        return -10000;
    }
    let fb = |k: usize| sb(f, k);
    let bd = *(0xb63d28 as *const usize);
    let da_base = *(p as *const i32) < *(0xae2364 as *const i32);
    let ativo = limite > -1;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    let sem_valor = flags & 0x800;
    if sem_valor == 0 || flags & 0x210000 != 0 {
        let valor: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_INJURY_GET_CONDITION);
        l24 = valor(bd, p, 0);
        l28 = l24 as i16 as i32 / 100;
    }
    let idade = *((p + 0x18) as *const i8);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = if *(0xae1050 as *const usize) != 0 { contrato_com(*(0xae1050 as *const usize), p, clube) } else { 0 };
    let (conhece, obs) = if o != 0 {
        (*((o + 0x69) as *const usize), o)
    } else if clube != 0 && *((clube + 0xcf) as *const usize) != 0 {
        let t = *((clube + 0xcf) as *const usize);
        (*((t + 0x69) as *const usize), t)
    } else {
        (0, 0)
    };
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let mut l3c = relacao(p, obs, 0);
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let masc = mascara(f);
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    edi += if flags & 0x400 != 0 { 100 } else { nota_da_posicao(f, pos, masc, 0xa15d50, true) };
    let l40 = edi + if flags & 0x200 != 0 { 100 } else { ajuste_da_posicao(f, pos) };
    let mut cont: i8 = 0;
    let mut soma: i32 = 0;
    let corte = |soma: i32, cont: i8| ativo && soma * 200 / (cont as i32) < limite;
    if flags & 0x40 != 0 {
        cont = 1;
        soma = core::ptr::read_unaligned((f + 0xb) as *const i16) as i32 / 50;
        if clube == *((p + 0x39) as *const usize) && o != 0 && observado_ha_mais_de_um_ano(o, p) {
            soma = ftol((f64_em(0x968fa8) - dias_desde(p) as f64 * f64_em(0x96a350)) * soma as f64);
        }
        if ativo && soma * 200 < limite {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        cont += 1;
        let a = core::ptr::read_unaligned((f + 9) as *const i16) as f64;
        let b = core::ptr::read_unaligned((*((p + 0x1a) as *const usize) + 0x8e) as *const i16) as f64;
        soma += ftol(a / ((f64_em(0x96d640) - b * f64_em(0x96a5c0)) * f64_em(0x968ff8)));
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        cont += 1;
        soma += core::ptr::read_unaligned((f + 0xd) as *const i16) as i32 / 50;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 4 != 0 && da_base {
        let v = ftol(nota_externa(f, 5, 0x12, f32::from_bits(0x40d00000)) * f64_em(0x968ea8));
        if v != 0 {
            cont += 1;
            soma += v;
        }
    }
    let habilidade: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if cont != 0 {
            l3c = soma * 50 / cont as i32;
        }
        l34 = habilidade(obs, p, clube, 2, l3c as i16) as u16 as i32;
        cont += 1;
        soma += l34 as i16 as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if cont != 0 {
            l3c = do_contrato_ou_media(p, f, clube, soma, cont);
        }
        let mut r = habilidade(obs, p, clube, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        cont += 1;
        soma += r as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    // daqui em diante o original reaproveita o lugar do limite para contas: não corta mais
    if flags & 0x40000 != 0 && da_base {
        soma += bonus_da_media(p, f, clube, o, cont);
    }
    if flags & 2 != 0 && da_base {
        soma = bonus_da_forma(p, f, clube, o, idade, cont, soma, l3c, false, 0x3fb66666);
    }
    let mut l4c: i32 = 0;
    if cont != 0 {
        l4c = soma / cont as i32;
        if l4c > 0x96 {
            l4c = ftol((l4c - 0x96) as f64 * f64_em(0x968940) + f64_em(0x968f50));
        }
    }
    if flags & 8 != 0 && da_base {
        l4c += ftol(fb(0x45) as f64 * f64_em(0x968940) - f64_em(0x968e08));
    }
    if flags & 0x4000 != 0 {
        let q = (0x1b58 - core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) / 100;
        let k = (if q < 10 { 10 } else { q }) as i8 as i32;
        let lim = if fb(0xf) == 0x14 { 0x24 } else if fb(0x15) == 0x14 || fb(0x14) == 0x14 { 0x22 } else { 0x23 };
        if idade as i32 > lim {
            let y = (f64_em(0x967890) - (idade as i32 - lim) as f64 / k as f64) * l4c as f64;
            let m = (l4c - 5) as f64;
            l4c = ftol(if y < m { y } else { m });
        }
        let historico: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
        let h = historico(bd, p);
        if h != 0 {
            l4c -= core::ptr::read_unaligned((h + 0x2c) as *const i16) as i32;
        }
        l4c = fim_da_idade(p, f, clube, flags, pos, masc, idade, cont, l4c);
    }
    if contrato != 0 && clube != 0 && flags & 0x2000000 != 0 && da_base {
        l4c = ajuste_do_contrato(p, f, clube, contrato, idade, l4c);
    }
    if sem_valor == 0 {
        let mut v = l24 as i16 as i32;
        if flags & 0x200000 == 0 && reputacao(p) > 1000 {
            v += 500 - reputacao(p) as i32;
        }
        if v < 5000 {
            return v + (l40 * 5 - 10000) * 20;
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = desconto_do_historico(p, l4c);
    }
    if flags & 0x200000 != 0 {
        l4c = ajuste_da_fama(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && clube != 0 && l40 >= 0x64 {
        l4c = bonus_do_observador(p, f, clube, conhece, obs, o, pos, masc, idade, l4c, true);
    }
    l4c = fora_da_nacao(p, clube, flags, l4c);
    if flags & 0x10000 != 0 {
        if contrato != 0 {
            l4c += extra_do_contrato(p, contrato, flags, l28);
        }
    } else if flags & 0x20000 != 0 {
        let c = contrato_com(*(0xae1050 as *const usize), p, clube);
        if c != 0 {
            l4c += vinculo(p, c, flags);
        }
        if sem_valor == 0 {
            l4c = ajuste_da_reputacao(p, l4c);
        }
    }
    l4c = ajuste_dos_jogos(p, f, flags, da_base, l4c);
    if !da_base {
        l4c -= 0x1e;
    }
    l4c = no_proprio_clube(p, clube, l4c);
    combinar_nota(flags, l40, l4c)
}

/// O valor para a habilidade potencial (flag 0x100): no clube do contrato (0x4cf5a0, que fica em
/// [0xae1080]) a média x 25 mais o potencial (+7) x 25 (em 16 bits); fora dele, a média x 50.
unsafe fn do_contrato_ou_media(p: usize, f: usize, clube: usize, soma: i32, cont: i8) -> i32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato_de(*(0xae1050 as *const usize), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c == 0 { 0 } else { *(0xae23b4 as *const usize) + *((c + 4) as *const usize) * 0x245 };
    if dono == clube {
        let q = soma * 25 / cont as i32;
        let _ = f;
        let pot = core::ptr::read_unaligned((*((p + 0x61) as *const usize) + 7) as *const i16).wrapping_mul(0x19) as u16 as u32;
        q.wrapping_add(((q as u32 & 0xffff0000) | pot) as i32)
    } else {
        soma * 50 / cont as i32
    }
}

/// A fama (flag 0x200000): -200 para os famosos (valor/100 >= 99) de +0x3c alto, +0x2d baixo e
/// reputação abaixo de 500; senão 100 - valor/100.
unsafe fn ajuste_da_fama(f: usize, p: usize, l28: i32, l4c: i32) -> i32 {
    let v = l28 as i16 as i32;
    if v >= 0x63 && sb(f, 0x3c) > 0xe && sb(f, 0x2d) < 8 && reputacao(p) < 0x1f4 {
        l4c - 200
    } else {
        l4c + 100 - v
    }
}

/// Fora da nação do clube (flag 0x1000000): -50 sem nenhuma das duas nacionalidades (+0x1a, +0x1e).
unsafe fn fora_da_nacao(p: usize, clube: usize, flags: u32, l4c: i32) -> i32 {
    if flags & 0x1000000 == 0 || clube == 0 {
        return l4c;
    }
    let nac = *((clube + 0x53) as *const usize);
    if *((p + 0x1a) as *const usize) != nac && *((p + 0x1e) as *const usize) != nac {
        l4c - 50
    } else {
        l4c
    }
}

/// the player's score for a lineup slot: like the off-match score, plus the second position, in-match stamina, tactics,
/// veterans and holidays
/// Original: `lineup_slot_score` (0x6e2f20, match_man.cpp, cdecl).
///
/// [pt] { 0x6e2f20 } (cdecl, match_man.cpp): a nota do jogador `p` (ficha `f`) para uma vaga da escalação
/// (`pos`, e a segunda posição `pos2` com peso 1/3), pelas `flags`, no `clube`, com o lado `lado`
/// (+0x1b observador, +0x1f conhecimento), o técnico `tecnico`, o `limite`, as marcas da tática
/// (`marca` e `bytes`, `tatica`) e a lista das férias `ferias` (16 pares id, posição). Como a
/// 0x6e6340 (tabela 0xa15cfc), mais a condição na partida (`reg`, o jogador da partida), a tática,
/// a idade dos veteranos e o bônus das férias. -10000 sem pessoa ou ficha; -1000000 vetado.
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn nota_para_a_vaga(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, clube: usize, lado: usize, tecnico: usize, limite: i32, marca: u32, _marca_alta: u32,
    bytes: u32, tatica: u32, pos2: u32, ferias: usize,
) -> i32 {
    if p == 0 || f == 0 {
        return -10000;
    }
    let fb = |k: usize| sb(f, k);
    let bd = *(0xb63d28 as *const usize);
    let da_base = *(p as *const i32) < *(0xae2364 as *const i32);
    let idade = *((p + 0x18) as *const i8);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = if *(0xae1050 as *const usize) != 0 { contrato_com(*(0xae1050 as *const usize), p, clube) } else { 0 };
    let sem_valor = flags & 0x800;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    if sem_valor == 0 || flags & 0x210000 != 0 {
        let valor: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_INJURY_GET_CONDITION);
        l24 = valor(bd, p, 0);
        l28 = l24 as i16 as i32 / 100;
    }
    let limite = if ferias != 0 { 0 } else { limite };
    let ativo = ferias != 0 || limite > -1;
    let (conhece, obs) = if lado != 0 {
        (*((lado + 0x1f) as *const usize), *((lado + 0x1b) as *const usize))
    } else if tecnico != 0 {
        (*((tecnico + 0x69) as *const usize), tecnico)
    } else if clube != 0 && *((clube + 0xcf) as *const usize) != 0 {
        let t = *((clube + 0xcf) as *const usize);
        (*((t + 0x69) as *const usize), t)
    } else {
        (0, 0)
    };
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let mut l3c = relacao(p, obs, 0);
    let mascara: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let masc = mascara(f);
    let segunda = pos2 as u16 != 0 && pos2 as u16 != pos as u16;
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    let mut ebp: i32 = 0;
    if flags & 0x400 != 0 {
        edi += 100;
    } else {
        edi += nota_da_posicao(f, pos, masc, 0xa15cfc, true);
        if segunda {
            ebp = nota_da_posicao(f, pos2, masc, 0xa15cfc, false);
        }
    }
    let l40 = if flags & 0x200 != 0 {
        edi + 100
    } else {
        edi += ajuste_da_posicao(f, pos);
        if segunda { (ebp + ajuste_da_posicao(f, pos2) + 2 * edi) / 3 } else { edi }
    };
    let mut cont: i8 = 0;
    let mut soma: i32 = 0;
    let corte = |soma: i32, cont: i8| ativo && soma * 200 / (cont as i32) < limite;
    if flags & 0x40 != 0 {
        cont = 1;
        soma = core::ptr::read_unaligned((f + 0xb) as *const i16) as i32 / 50;
        if clube == *((p + 0x39) as *const usize) && tecnico != 0 && observado_ha_mais_de_um_ano(tecnico, p) {
            soma = ftol((f64_em(0x968fa8) - dias_desde(p) as f64 * f64_em(0x96a350)) * soma as f64);
        }
        if ativo && soma * 200 < limite {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        cont += 1;
        let a = core::ptr::read_unaligned((f + 9) as *const i16) as f64;
        let b = core::ptr::read_unaligned((*((p + 0x1a) as *const usize) + 0x8e) as *const i16) as f64;
        soma += ftol(a / ((f64_em(0x968950) - b * f64_em(0x96a5c0)) * f64_em(0x968ff8)));
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        cont += 1;
        soma += core::ptr::read_unaligned((f + 0xd) as *const i16) as i32 / 50;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 1 != 0 && reg != 0 && masc & 1 == 0 {
        cont += 1;
        soma += (core::ptr::read_unaligned((reg + 0x35) as *const i16) as i32 - folego_na_partida(reg, fb(0xf))) / 50;
    }
    if flags & 4 != 0 && da_base {
        let v = ftol(nota_externa(f, 5, 0x12, f32::from_bits(0x40d00000)) * f64_em(0x968ea8));
        if v != 0 {
            cont += 1;
            soma += v;
        }
    }
    let habilidade: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if cont != 0 {
            l3c = soma * 50 / cont as i32;
        }
        l34 = habilidade(obs, p, clube, 2, l3c as i16) as u16 as i32;
        cont += 1;
        soma += l34 as i16 as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if cont != 0 {
            l3c = do_contrato_ou_media(p, f, clube, soma, cont);
        }
        let mut r = habilidade(obs, p, clube, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        let fp = *((p + 0x61) as *const usize);
        if fp != 0 {
            let (d, c) = (*((fp + 0x1b) as *const i8), *((fp + 0x36) as *const i8));
            r = r.wrapping_sub(if d < 6 { 10 } else if d < 10 { 5 } else { 0 });
            r = r.wrapping_sub(if c < 6 { 15 } else if c < 10 { 10 } else { 0 });
        }
        cont += 1;
        soma += r as i32;
        if corte(soma, cont) {
            return -10000;
        }
    }
    if flags & 0x40000 != 0 && da_base {
        soma += bonus_da_media(p, f, clube, tecnico, cont);
    }
    if flags & 2 != 0 && da_base {
        soma = bonus_da_forma(p, f, clube, tecnico, idade, cont, soma, l3c, true, 0x3faccccd);
    }
    let mut l4c: i32 = 0;
    if cont != 0 {
        l4c = soma / cont as i32;
        if l4c > 0x96 {
            l4c = ftol((l4c - 0x96) as f64 * f64_em(0x968940) + f64_em(0x968f50));
        }
    }
    if flags & 8 != 0 && da_base {
        l4c += ftol((fb(0x45) - 10) as f64 * f64_em(0x9689b0));
    }
    let marcada = marca & 0x8000 != 0 || (marca & 0x2000 != 0 && tatica & 0x100 != 0);
    if flags & 0x8000 != 0 {
        l4c = ajuste_da_tatica(f, marca, bytes as u8, tatica, marcada, l4c);
    }
    if contrato != 0 && clube != 0 && flags & 0x2000000 != 0 && da_base {
        l4c = ajuste_do_contrato(p, f, clube, contrato, idade, l4c);
    }
    if flags & 0x4000 != 0 {
        l4c = desconto_do_historico_2c(p, l4c);
        let id = *(p as *const i32);
        let mut edi = l4c;
        if *((*(0xae2d0c as *const usize) as isize + id as isize * 79 + 0xb) as *const u8) & 0x30 != 0 {
            if idade > 0x28 {
                return -1000000;
            }
            edi -= if idade > 0x22 { 0x4b } else { 0x19 };
            l4c = edi;
            if sb(*((p + 0x61) as *const usize), 0x3c) < 0xa {
                edi -= 0x32;
                l4c = edi;
            }
        }
        if idade > 0x2a {
            return -1000000;
        }
        let i = idade as i32;
        if i > 0x22 && masc & 1 == 0 {
            l4c = edi + (0x22 - i) * 10;
        } else if i < 0x17 && masc & 1 != 0 {
            l4c = edi + (i - 0x17) * 10;
        } else if i > 0x20 && pos & 0x40 != 0 {
            l4c = edi + (0x20 - i) * 5;
        }
        l4c = fim_da_idade(p, f, clube, flags, pos, masc, idade, cont, l4c);
    }
    let mut valor: i32 = 0;
    if sem_valor == 0 {
        let mut v: i32;
        if reg != 0 {
            if *((reg + 0x19) as *const i8) == -2 {
                return -1000000;
            }
            let x = core::ptr::read_unaligned((reg + 0x59) as *const f32) as f64;
            v = ftol(x + x + core::ptr::read_unaligned((reg + 0x4d) as *const f32) as f64) as i16 as i32;
            if core::ptr::read_unaligned((*((reg + 0x19e) as *const usize) + 0x8ed4) as *const i16) == 0x1ef {
                v = (v + 0x5dc).min(0x2710);
            }
        } else {
            v = l24 as i16 as i32;
        }
        if flags & 0x200000 == 0 && reputacao(p) > 1000 {
            v += 500 - reputacao(p) as i32;
        }
        if v < 0x1676 {
            let r = v + (l40 * 5 - 0x4e2) * 20;
            return if v >= 0xdac { r } else { r - 0x7a120 };
        }
        if marcada {
            v = ftol(v as f64 * f64_em(0x96a5c0) * v as f64);
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = desconto_do_historico(p, l4c);
        valor = v;
    }
    if flags & 0x200000 != 0 {
        l4c = ajuste_da_fama(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && clube != 0 && l40 >= 0x64 {
        l4c = bonus_do_observador(p, f, clube, conhece, obs, tecnico, pos, masc, idade, l4c, false);
    }
    if flags & 0x2000 != 0 && lado != 0 && *((lado + 0x3b) as *const usize) == p {
        l4c += 100;
    }
    l4c = fora_da_nacao(p, clube, flags, l4c);
    if flags & 0x80000 != 0 && lado != 0 {
        let em = |o: usize| *((lado + o) as *const usize) == p;
        l4c += if em(6) { 0x1e } else if em(0xa) { 0x14 } else if em(0xe) { 0xa } else { 0 };
    }
    // o cansaço do minuto da partida reduz o extra
    let pelo_minuto = |v: i32| {
        if reg == 0 {
            v
        } else {
            let minuto = core::ptr::read_unaligned((*((reg + 0x19e) as *const usize) + 0x8ed0) as *const i16) as f64;
            ftol((f64_em(0x967890) - minuto * f64_em(0x96d268)) * v as f64)
        }
    };
    if flags & 0x10000 != 0 {
        if contrato != 0 {
            let mut extra = pelo_minuto(extra_do_contrato(p, contrato, flags, l28));
            if ferias != 0 {
                extra = ftol(extra as f64 * f64_em(0x9689e0));
            }
            l4c += extra;
        }
    } else if flags & 0x20000 != 0 {
        if contrato != 0 {
            l4c += pelo_minuto(vinculo(p, contrato, flags));
        }
        if sem_valor == 0 {
            l4c = ajuste_da_reputacao(p, l4c);
        }
    }
    l4c = ajuste_dos_jogos(p, f, flags, da_base, l4c);
    let mut base = l40;
    if !da_base && clube != 0 {
        let nac = *((clube + 0x53) as *const usize);
        if nac != 0 && *((nac + 0x11c) as *const u8) != 0 {
            l4c -= 0x46;
            base = l40 - 0x78;
        }
    }
    l4c = no_proprio_clube(p, clube, l4c);
    l4c = combinar_nota(flags, base, l4c);
    if ferias != 0 && (pos & 1 == 0 || masc & 1 != 0) {
        l4c = bonus_das_ferias(p, clube, ferias, pos, base, sem_valor, valor, l4c);
    }
    l4c
}

/// O desconto da flag 0x4000 pelo histórico (0x62ed70, +0x2c).
unsafe fn desconto_do_historico_2c(p: usize, l4c: i32) -> i32 {
    let historico: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let h = historico(*(0xb63d28 as *const usize), p);
    if h == 0 { l4c } else { l4c - core::ptr::read_unaligned((h + 0x2c) as *const i16) as i32 }
}

/// O cansaço do jogador da partida `reg` a descontar do fôlego (+0x35): -2000 para o técnico humano
/// (+0x904d do lado, sem 0x5fd210, seleção ou 0x5fd480) com o jogador abaixo de 11 (+0); fora de
/// campo (+0x19 = -1) os minutos que faltam x 150 (até 3000; goleiro só +3500, os demais pelo menos
/// -2000); em campo, os minutos jogados (minuto - +0x1f) abaixo de 60 x 40.
unsafe fn folego_na_partida(reg: usize, goleiro: i32) -> i32 {
    let m = *((reg + 0x19e) as *const usize);
    let humano = if core::ptr::read_unaligned((m + 0x8ed4) as *const i16) == 0x1ef {
        true
    } else {
        let lado = *((reg + 0x27) as *const i8);
        let base = (m as isize + lado as isize * 0xd0) as usize;
        let t = *((base + 0x904d) as *const usize);
        let n = *(0xae2364 as *const i32);
        if t == 0 || *(t as *const i32) < n - 0x10 || *(t as *const i32) >= n || *((t + 0x5f) as *const u8) == 0 {
            false
        } else {
            let tecnicos = *(0xb63c98 as *const usize);
            let g9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            if g9a01(tecnicos, t) != 0 {
                false
            } else if *((base + 0x904b) as *const u8) != 0 {
                let c = *((*((m + 0x4792) as *const usize) + if lado == 0 { 0x1c } else { 0x20 }) as *const usize);
                let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
                if selecao(c) != 0 {
                    true
                } else {
                    let g9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
                    g9a05(tecnicos, *((base + 0x904d) as *const usize)) != 0
                }
            } else {
                true
            }
        }
    };
    let extra = if humano && *(reg as *const i8) < 0xb { -2000 } else { 0 };
    let estado = *((reg + 0x19) as *const i8);
    let minuto = core::ptr::read_unaligned((m + 0x8ed0) as *const i16) as i32 / 11;
    if estado == -1 {
        let e = extra + ((60 - minuto) * 150).min(3000);
        if goleiro > 1 { e + 0xdac } else { e.max(-2000) }
    } else if estado >= 0 {
        let jogados = (minuto as u8).wrapping_sub(*((reg + 0x1f) as *const u8)) as i8;
        if jogados < 0x3c { extra + (jogados as i32 - 60) * 40 } else { extra }
    } else {
        extra
    }
}

/// As marcas da tática (flag 0x8000): `marca` (o u64 das instruções, parte baixa), `bytes` (o
/// byte das instruções) e `tatica` contra os atributos da ficha.
unsafe fn ajuste_da_tatica(f: usize, lo: u32, b1: u8, tatica: u32, marcada: bool, mut l4c: i32) -> i32 {
    let fb = |k: usize| sb(f, k);
    let x = |k: usize, c: usize| ftol((fb(k) - 10) as f64 * f64_em(c));
    let t13 = tatica as u8;
    let um = lo & 1 != 0;
    if lo & 4 != 0 || (um && t13 & 2 != 0) {
        l4c += fb(0x37) / 7 - x(0x40, 0x96d5e8);
    } else if lo & 8 != 0 || (um && t13 & 4 != 0) {
        l4c += x(0x36, 0x968e00) - x(0x3c, 0x968fe8) - x(0x2e, 0x968fe8);
    } else if lo & 0x10 != 0 || (um && t13 & 8 != 0) {
        l4c += x(0x36, 0x96a670) - x(0x3c, 0x96d5e8) - x(0x2e, 0x96d5e8);
    }
    if marcada {
        let c = fb(0x3c) - 15;
        let d = fb(0x44) - 15;
        let x1 = ftol((c.abs() * c) as f64 * f64_em(0x968fe8));
        let x2 = ftol((d.abs() * d) as f64 * f64_em(0x968e00));
        l4c += fb(0x1e) / 10 + (x2 - x1);
    } else if b1 & 1 != 0 {
        l4c += fb(0x33) / 12 + (fb(0x43) / 12 - x(0x28, 0x96d5e8));
    }
    if b1 & 2 != 0 {
        l4c += fb(0x39) / 7;
    } else if b1 & 4 != 0 {
        l4c += fb(0x32) / 7;
    }
    if lo & 0x40 != 0 {
        l4c += ftol((fb(0x3b) - 20) as f64 * f64_em(0x96d5e0));
    } else if lo & 0x100 != 0 {
        l4c += ftol((fb(0x30) - 20) as f64 * f64_em(0x96d5e0));
    }
    if lo & 0x200000 != 0 {
        l4c += fb(0x1a) + fb(0x43) / 10 + fb(0x33) / 9 - 20;
    }
    if lo & 0x20000 != 0 {
        l4c += (fb(0x2e) - 10) / 4;
    } else if lo & 0x40000000 != 0 {
        l4c += (10 - fb(0x2e)) / 4;
    }
    if lo & 0x80000000 != 0 {
        l4c += (10 - fb(0x2e)) / 4;
    } else if lo & 0x80000 != 0 {
        l4c += (fb(0x2e) - 10) / 4;
    }
    if lo & 0x800000 != 0 {
        l4c += (fb(0x3c) - 10) / 2 + fb(0x43) / 15 + fb(0x27) / 15;
    }
    if lo & 0x2000000 != 0 {
        l4c += fb(0x26) / 7;
    }
    if lo & 0x8000000 != 0 {
        l4c += (fb(0x40) - 10) / 3 + (fb(0x3d) - 10) / 2;
    }
    if lo & 0x20000000 != 0 {
        l4c += (fb(0x28) - 10) / 2 + fb(0x31) / 14 + fb(0x27) / 14;
    }
    l4c
}

/// O bônus das férias: o jogador na lista (16 pares id, posição) ganha 40000 + 200 x a nota da
/// posição (pelo menos 10000) se estiver entre os 11 e o técnico (0x5fea50) aceitar a posição; senão
/// 200 x a nota (nunca negativa), com a nota negativa pela metade; pesado pelo valor do jogador.
#[allow(clippy::too_many_arguments)]
unsafe fn bonus_das_ferias(p: usize, clube: usize, ferias: usize, pos: u32, base: i32, sem_valor: u32, valor: i32, mut l4c: i32) -> i32 {
    let id = *(p as *const i32);
    let Some(k) = (0..16usize).find(|&k| *((ferias + 8 * k) as *const i32) == id) else {
        return l4c;
    };
    let aceita = || {
        let g: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
        if g(*(0xb63c98 as *const usize), 2, *((clube + 0xcf) as *const usize)) == 0 {
            return true;
        }
        let w = *((ferias + 8 * k + 4) as *const i32);
        w == pos as i16 as i32 || w == -1
    };
    let mut bonus;
    if k < 0xb && aceita() {
        bonus = (base * 200).max(10000) + 40000;
        if l4c < 0 {
            l4c = 0;
        }
    } else {
        bonus = (base * 200).max(0);
        if l4c < 0 {
            l4c /= 2;
        }
    }
    if sem_valor == 0 {
        let v = valor as f64;
        let y = v * f64_em(0x968eb8);
        let z = if valor >= 0x251c {
            y * f64_em(0x96d568)
        } else if valor >= 0x222e {
            y * v * f64_em(0x968910)
        } else {
            v * (y * v)
        };
        bonus = ftol((z * f64_em(0x96a600) - f64_em(0x967890)) * bonus as f64);
    }
    if bonus > 0 {
        l4c += bonus;
    }
    l4c
}

unsafe extern "C" fn comparar_nota_fora_da_partida(p: usize, pos: u32, flags: u32, clube: usize, limite: i32, o: usize) -> i32 {
    let f = orig::<unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32>(392);
    if gravando_no_topo() {
        return f(p, pos, flags, clube, limite, o);
    }
    // ponytail: sem as regiões do sorteador. A nota não sorteia: o único sorteio vem da 0x62ed70, que
    // atualiza o físico do jogador (0x62c680) só na primeira consulta e guarda; a cópia, depois da
    // original, não o refaz. Devolver o sorteador entre as duas acusava essa diferença de cache.
    let reg = vec![(0xae1080usize, 4usize)];
    let rastros = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || {
            let (r, v) = crate::aleatorio::rastrear(|| f(p, pos, flags, clube, limite, o));
            rastros.borrow_mut().0 = v;
            r
        },
        || {
            let (r, v) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(|| nota_fora_da_partida(p, pos, flags, clube, limite, o)));
            rastros.borrow_mut().1 = v;
            r
        },
    );
    PLACAR.registrar(igual && r == n, || format!("nota_fora_da_partida({:#x}, pos {:#x}, flags {:#x}, limite {}) original={} rust={} sorteios {:?} x {:?} {:x?} x {:x?}", p, pos as u16, flags, limite, r, n, rastros.borrow().0, rastros.borrow().1, mo, mr));
    r
}

#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_nota_para_a_vaga(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, clube: usize, lado: usize, tecnico: usize, limite: i32, marca: u32, marca_alta: u32,
    bytes: u32, tatica: u32, pos2: u32, ferias: usize,
) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32>(393);
    if gravando_no_topo() {
        return o(p, f, reg, pos, flags, clube, lado, tecnico, limite, marca, marca_alta, bytes, tatica, pos2, ferias);
    }
    // sem as regiões do sorteador, pelo mesmo motivo da nota fora da partida (0x62ed70 com cache)
    let regioes = vec![(0xae1080usize, 4usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(p, f, reg, pos, flags, clube, lado, tecnico, limite, marca, marca_alta, bytes, tatica, pos2, ferias),
        || crate::aleatorio::sem_contar(|| nota_para_a_vaga(p, f, reg, pos, flags, clube, lado, tecnico, limite, marca, marca_alta, bytes, tatica, pos2, ferias)),
    );
    PLACAR.registrar(igual && r == n, || format!("nota_para_a_vaga({:#x}, pos {:#x}/{:#x}, flags {:#x}, marca {:#x}) original={} rust={} {:x?} x {:x?}", p, pos as u16, pos2 as u16, flags, marca, r, n, mo, mr));
    r
}

