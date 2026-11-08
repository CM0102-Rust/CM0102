use super::*;

/// Assigns the `n` people in `pessoas` to `m` positions (+ `x` extras) and writes each position's person to `saida`,
/// scoring each (person, position) pair (0x6e2f20: position code, mode, club, staff). With fewer people than positions
/// each free person takes the position where he beats the holder; with more, each position takes the best and
/// duplicates are resolved by least loss; extras get the leftovers recursively (mode | 0x200); finally pairs are
/// swapped while the total improves. Returns 1, or 0 with no people or positions [inference: picking the line-up].
///
/// [pt] { 0x6e2000 } (cdecl, match_man.cpp, ~3,9 KB, recursiva): distribui as `n` pessoas de `pessoas`
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
pub(crate) unsafe extern "C" fn pick(
    persons: usize, a2: u32, codigos: usize, a4: u32, a5: u32, out: usize, mode: u32, clube: usize, comissao: usize,
    a10: u32, _a11: u32, mut marks: usize, mut bytes: usize, a14: u32, mut curtos: usize,
) -> u8 {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let rating_fn: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let recursiva: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let manager: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_GET_SAVED_LINEUP);
    let n = a2 as u8 as i8 as i32;
    let m = a4 as u8 as i8 as i32;
    let x = a5 as u8 as i8 as i32;
    let tot = m + x;
    let pes = |k: i32| *((persons + 4 * k as usize) as *const usize);
    let sai = |i: i32| (out + 4 * i as usize) as *mut usize;
    let (mut al12, mut al13, mut al15) = (false, false, false);
    if marks == 0 {
        marks = if tot != 0 { calloc(tot as u32, 8) } else { 0 };
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
        if al12 && marks != 0 {
            free(marks);
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
            let mut found = 0usize;
            for _ in 0..20 {
                if *((j + 0x69) as *const usize) == pes(k) {
                    found = j;
                    break;
                }
                j += 0x1be;
            }
            *((jog + 4 * k as usize) as *mut usize) = found;
        }
    }
    let mut lc = 0u32;
    if mode & 0x400000 != 0 && clube != 0 {
        let p = *((clube + club::MANAGER) as *const usize);
        let q = *(G_STAFF_COUNT as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
            lc = manager(*(G_HUMAN_MANAGER as *const usize), p);
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
    let rating = |person: usize, j: usize, i: i32| -> i32 {
        let i = i as usize;
        rating_fn(
            person, *((person + staff::PLAYER) as *const usize), j, *((codigos + 2 * i) as *const u16) as u32, mode, clube, comissao, a10, u32::MAX,
            *((marks + 8 * i) as *const u32), *((marks + 8 * i + 4) as *const u32), *((bytes + i) as *const u8) as u32, a14,
            *((curtos + 2 * i) as *const u16) as u32, lc,
        )
    };
    let soltar = |target: usize| {
        for kk in 0..n {
            if pes(kk) == target {
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
                let (mut melhor, mut best_rating) = (-1i32, 0i32);
                for i in 0..m {
                    let r = rating(cand, jg(k), i);
                    if melhor == -1 || r > best_rating {
                        let ocupante = *sai(i);
                        if ocupante == 0 || rating(ocupante, jg(k), i) < r {
                            melhor = i;
                            best_rating = r;
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
            let exit = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 10 } else { f == 20 };
            if exit {
                soltar(ocupante);
                *sai(i) = 0;
            }
        }
    } else {
        // mais pessoas: cada vaga pega a melhor; os repetidos sao resolvidos depois
        let mut conflitos = 0i8;
        let mut best_rating = 0i32;
        for i in 0..m {
            let (mut melhor, mut mk) = (0usize, 0i32);
            for k in 0..n {
                let cand = pes(k);
                if cand == 0 || *((cand + 0x61) as *const usize) == 0 {
                    continue;
                }
                let r = rating(cand, jg(k), i);
                if melhor == 0 || r > best_rating {
                    melhor = cand;
                    best_rating = r;
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
        let mut k2_current = 0i32;
        while conflitos > 0 && voltas < 10000 {
            voltas += 1;
            let (mut vaga_t, mut k2_t) = (-1i32, -1i32);
            for k in 0..n {
                let mut swap = 0usize;
                let cand = pes(k);
                if m > 0 {
                    for j in 0..m {
                        if cand == 0 || *us(k) <= 1 {
                            break;
                        }
                        if cand != *sai(j) {
                            continue;
                        }
                        let r_current = rating(cand, jg(k), j);
                        let (mut outra, mut outra_nota) = (0usize, 0i32);
                        for k2 in 0..n {
                            let o = pes(k2);
                            if o == 0 || *((o + 0x61) as *const usize) == 0 || o == cand {
                                continue;
                            }
                            let r = rating(o, jg(k2), j);
                            if (outra == 0 || r > outra_nota) && (r < r_current || *us(k2) == 0) {
                                outra_nota = r;
                                outra = o;
                                k2_current = k2;
                            }
                        }
                        if swap == 0 || r_current - outra_nota < menor_perda {
                            vaga_t = j;
                            swap = outra;
                            k2_t = k2_current;
                            menor_perda = (r_current - outra_nota) as i16 as i32;
                        }
                    }
                    if swap != 0 {
                        *us(k2_t) = (*us(k2_t)).wrapping_add(1);
                        if *us(k2_t) > 1 {
                            conflitos = conflitos.wrapping_add(1);
                        }
                        *sai(vaga_t) = swap;
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
            resto.as_ptr() as usize, c as u32, codigos + 2 * m as usize, x as u32, 0, out + 4 * m as usize, mode | 0x200, clube, comissao, a10, 0,
            marks, bytes, a14, curtos + 2 * m as usize,
        );
    }
    // trocas de pares enquanto a soma melhorar
    let player_of = |p: usize| -> usize {
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
                let ja = player_of(a);
                let a_i = rating(a, ja, i);
                let mut j = i + 1;
                while j < tot && !mudou {
                    let b = *sai(j);
                    if b != 0 && *((b + 0x61) as *const usize) != 0 {
                        let jb = player_of(b);
                        let b_i = rating(b, jb, i);
                        let trocar = if j < m {
                            let a_j = rating(a, ja, j);
                            let b_j = rating(b, jb, j);
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
/// ataque, 3 equilibrio, 4 defesa, 5 retranca. RoleBitEffect da forca dos dois lados (com o motor rodando
/// (+0x8ea5), a soma das notas 0x6e86d0 dos que estao em campo * 50 / 11; senao a reputacao +0x80
/// dos clubes), da posse (+0x1fc/+0x206/+0x210), do tempo (+0x8ed0), dos expulsos (+0x1ad) e do
/// placar agregado (0x6ce310), e ajusta pela liga (posicoes na tabela, 0x4a4850), pelo tecnico e pelo
/// fim do jogo (+0x8ed4 = 0x3de). Guarda o fator em motor+0x476a[lado] (10 e -1 nos extremos); com o
/// motor rodando e o fator a menos de 0,5 do guardado, mantem a postura atual (+0x1ab).
pub(crate) unsafe extern "thiscall" fn stance(c: usize) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let i = |p: usize, o: usize| *((p + o) as *const i32);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let m = u(c, 0x12);
    let side = byte(c, 0x18);
    let lu = side as isize as usize;
    let (clube_a, clube_b) = (u(c, 0x2f), u(c, 0x33));
    let (rep_a, rep_b) = (w(clube_a, 0x80), w(clube_b, 0x80));
    let rodando = byte(m, 0x8ea5) != 0;
    // a forca de cada lado (so os 16 bits contam, como no original)
    let (mut own, mut other) = (rep_a, rep_b);
    if rodando {
        let rating: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
        let mut sum = [0i32; 2];
        for (l, s) in sum.iter_mut().enumerate() {
            for k in 0..20 {
                let j = m + 0x22d8 * l + 0x4796 + 0x1be * k;
                if byte(j, 0x19) >= 0 {
                    *s = s.wrapping_add(rating(u(c, 0x1b), u(j, 0x69), clube_a, 2, u32::MAX) as i32);
                }
            }
        }
        let (a, b) = if side == 0 { (sum[0], sum[1]) } else { (sum[1], sum[0]) };
        own = a.wrapping_mul(50) / 11;
        other = b.wrapping_mul(50) / 11;
    }
    let (mut x11, mut x12, mut x14, mut x16, mut x17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    let agregado: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> i8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOAL_MARGIN);
    agregado(m, side, &mut x11, &mut x12, &mut x16, &mut x17, &mut x14, 0);
    let own = own as i16 as i32;
    let other = other as i16 as i32;
    let f = (own - other) as f64 / ((1.5 - own as f64 * 0.0001) * 6000.0) + 1.0;
    // a posse
    let (p1, p2, p3) = (w(m, 0x1fc), w(m, 0x206), w(m, 0x210));
    let tot = p1 + p2 + p3;
    let posse_b = if tot != 0 { (p3 * 100 / tot) as i8 } else { 0 };
    let posse_a = if tot != 0 { (p1 * 100 / tot) as i8 } else { 0 };
    let r = if side == 0 && posse_a != 0 {
        Some(posse_b as f64 / posse_a as f64)
    } else if side == 1 && posse_b != 0 {
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
        if side == 1 && x11 != 0 {
            x11 = x11.wrapping_add(2);
        } else if side == 0 {
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
        let count: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_PLAYERS_ON_PITCH);
        let b = count(m, (side == 0) as u32);
        let a = count(m, side as u8 as u32);
        x13 = (a as i8).wrapping_sub(b as i8);
        if x13 != 0 {
            x11 = x11.wrapping_add(x13.wrapping_mul(-2));
        }
    }
    let tt = (w(m, 0x8ed0) / 11) as f64;
    if x14 == 0 && x17 == 0 && x11 as i32 + x12 as i32 == 0 {
        let (d, e) = (byte(m, 0x1ad + (side == 0) as usize), byte(m, 0x1ad + lu));
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
            let comp = if p14 == u(clube_a, 0x57) { u(u(G_COMP_OBJECT_TABLE, 0), 4 * u(jogo, 0)) } else { 0 };
            if comp != 0 && u(comp, 8) != 0 {
                // a liga: quem esta acima na tabela e quanto falta
                let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
                let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
                let e = search(u(comp, 8), clube_a);
                if e != 0 {
                    let mut x18 = 0f32;
                    let mut done = false;
                    if i(e, 0x55) > 1 && days(G_CURRENT_DATE, u(e, 0x61) as u32, u(e, 0x65) as u32) < 30 {
                        let (a, b) = (i(e, 0x55).wrapping_neg(), i(e, 0x2d).wrapping_neg().wrapping_shl(1));
                        x18 = (if a < b { a } else { b }) as f32;
                        done = true;
                    }
                    if !done && i(e, 0x41) > 1 && days(G_CURRENT_DATE, u(e, 0x4d) as u32, u(e, 0x51) as u32) < 10 {
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
                        let e2 = search(u(u(u(G_COMP_OBJECT_TABLE, 0), 4 * u(jogo, 0)), 8), clube_b);
                        if e2 != 0 && *((e2 + 0x72) as *const u8) > pe {
                            let comp = u(u(G_COMP_OBJECT_TABLE, 0), 4 * u(jogo, 0));
                            let cx = w(comp, 0x3e);
                            let mut x30 = if cx > 0 { 22.0 / cx as f64 } else { 1.0 };
                            let ax = w(u(jogo, 0x14), 0x69);
                            if ax >= 10 {
                                let lim = ax * 500;
                                if rep_a > lim {
                                    x30 *= ((rep_a - lim) as f64 + (rep_a - 5000) as f64) * 0.0002 + 1.0;
                                }
                            }
                            let p = game_pow(x30, 1.5) * (*((e2 + 0x72) as *const u8) as i32 - pe as i32) as f64;
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
    let sum = x11 as i32 + x12 as i32;
    if sum > 6 {
        h *= 0.825;
    } else if sum > 4 {
        h *= 0.875;
    }
    let kb = k;
    if kb != 0 || x13 < 0 {
        let pressure = (rep_b > 7500 && rep_b - rep_a > 2000) || (rep_b > 8500 && rep_b - rep_a > 750) || (rep_b > 8000 && x11 as i32 - 1 > x12 as i32);
        if pressure {
            let d = x12 as i32 - x11 as i32;
            if (d < 2 && w(m, 0x8ed4) == 0x1ef) || x12 <= x11 {
                h *= 0.3;
            }
        }
    }
    let d = x11 as i32 - x12 as i32;
    h *= if kb == 0 { 1.0 - d as f64 * 0.1 } else { 0.967 - (kb as i32 + 2 * d) as f64 * 0.067 };
    if x11 > x12 || (x11 == x12 && (x16 != 0 || side == 1)) {
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
    let ra_greater = w(clube_a, 0x80) > w(clube_b, 0x80);
    if ((sum < 5 && x13 > -1 && x11 == x12 && x14 == 0 && ra_greater) || x13 > 0) && !(h > 1.0) {
        h = 1.0;
    }
    if sum < 4 && x13 > -1 && x11 as i32 - 2 >= x12 as i32 && ra_greater && !(h > 1.15) {
        h = 1.15;
    }
    let manager = u(c, 0x1f);
    if manager != 0 && x13 > -1 {
        let e = byte(manager, 0xe) as i32;
        let mut f1 = (1.0 + (e - 10) as f64 * 0.06666666666666667) * h;
        let e2 = game_pow(e as f64, 2.0);
        if f1 < e2 * 0.005 {
            f1 = e2 * 0.0025 + f1 * 0.5;
        }
        let hm1 = h - 1.0;
        h = if f1 < hm1 { hm1 } else { f1 };
    }
    if (rep_b > 7500 || x13 < 0) && kb != 0 && d.abs() < 2 && (x11 >= x12 || tt < 75.0 || sum > 3) {
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
        let cai = if x14 == 0 && x17 != 0 { (tt + 90.0) / sum as f64 } else { tt / sum as f64 };
        if cai < 18.0 {
            h *= 0.3;
        }
    }
    let factor = (m + 0x476a + 4 * lu) as *mut f32;
    let tempo = w(m, 0x8ed4);
    if tt > 85.0 - h * 10.0 && tempo == 0x3de && (x11 as i32 + 1 == x12 as i32 || (x17 != 0 && x14 == 0 && x16 == 0 && x11 == x12)) {
        *factor = 10.0;
        return 1;
    }
    if sum < 5 && tt > 82.0 - h * 10.0 && tempo == 0x3de && x11 == x12 && x16 == 0 && (h > 1.0 || byte(m, 0x1ab + lu) == 1) {
        *factor = 10.0;
        return 1;
    }
    if h < 1.4 && kb != 0 && d > 1 {
        h = 0.2;
    }
    let retranca = (d > 3 && h > 1.0)
        || (tt > 30.0 && x11 >= x12 && 10.0 / h < ((d + 1) * (d + 1)) as f64 && clube_a != u(clube_b, 0xb3) && clube_a != u(clube_b, 0xb7) && clube_a != u(clube_b, 0xbb));
    if retranca {
        *factor = -1.0;
        return 5;
    }
    if rodando && (h - *factor as f64).abs() < 0.5 {
        return byte(m, 0x1ab + lu) as u8;
    }
    *factor = h as f32;
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

/// The staff object `c` picks the club's formation and loads the tactic into the engine: a random one in the engine's
/// random mode, just a reload when `modo` != 2, the human manager's own tactic when he is on holiday with that option;
/// otherwise rates each sector and band from the two best of the players in `fichas`, scores every formation of the
/// tactics bank (time, mentality, manager preferences, favourite) and keeps the best. Copies the tactic into the engine
/// (+0x91e2 per side) and +0x2b and sets the instructions. Returns 0.
///
/// [pt] { 0x6e0230 } (thiscall, ret 0x10, match_man.cpp): a comissao `c` escolhe a formacao do clube
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
pub(crate) unsafe extern "thiscall" fn choose_formation(c: usize, fichas: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let stance: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_MENTALITY);
    let definir: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CLUB_TACTIC_INDEX);
    let load: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTIC);
    let instructions: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_SET_COMPUTER_INSTRUCTIONS);
    let pp = stance(c) as i8;
    let clube = u(c, 0x2f);
    let mut escala = w(clube, 0x80) << 3;
    if escala <= 2000 {
        escala = 2000;
    }
    let m = u(c, 0x12);
    let human = byte(c, 0x19) as i32;
    let tat = || u(G_TACTICS, 0);
    let how_many = || (*((tat() + 0x5fd) as *const i32)).wrapping_add(*((tat() + 0x5f5) as *const i32));
    let best: i32;
    let tactic = 'tactic: {
        if byte(m, 0) != 0 {
            best = sortear(how_many()) as i16 as i32;
        } else if a4 as u8 != 2 {
            break 'tactic load(tat(), clube, human);
        } else {
            let p = u(clube, 0xcf);
            let q = *(G_STAFF_COUNT as *const i32);
            if human == 0 && p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
                let manager = u(G_HUMAN_MANAGER, 0);
                let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
                if consultar(manager, p) != 0 && opcao(manager, 2, p) != 0 {
                    let prepare: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
                    let dele: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_ACTIVE_TACTIC);
                    prepare(manager, 0, clube, p);
                    break 'tactic dele(u(G_HUMAN_MANAGER, 0), u(clube, 0xcf));
                }
            }
            best = judge_formations(c, fichas, a2, a3, pp, escala, calloc, free);
        }
        definir(tat(), clube, best as i8 as i16 as u16 as u32, human);
        load(tat(), clube, human)
    };
    let side = byte(c, 0x18) as isize;
    core::ptr::copy_nonoverlapping(tactic as *const u8, (m as isize + 0x91e2 + 0x18e3 * side) as *mut u8, 0x18e3);
    // simplification: sem tatica (ou sem +0x2b) o original para o jogo com erro (match_man.cpp 0xc1d); aqui so nao copia
    let copy = u(c, 0x2b);
    if tactic != 0 && copy != 0 {
        core::ptr::copy_nonoverlapping(tactic as *const u8, copy as *mut u8, 0x18e3);
        instructions(c);
    }
    0
}

/// Core of choose_formation (0x6e0230): rates each sector/band by the best two players and judges each formation;
/// returns the best.
///
/// [pt] O miolo da 0x6e0230: as notas por setor e faixa e o julgamento de cada formacao; devolve a melhor.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn judge_formations(
    c: usize, fichas: usize, a2: u32, a3: u32, pp: i8, escala: i32, calloc: unsafe extern "C" fn(u32, u32) -> usize, free: unsafe extern "C" fn(usize),
) -> i32 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let d = |p: usize, o: usize| *((p + o) as *const f64);
    let rating: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let pegar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_TACTIC_BLOCK);
    let position: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let marks: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let mark_b: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let clube = u(c, 0x2f);
    let n = a2 as u8 as i8 as i32;
    // as notas de cada setor (g) e faixa (s): a soma das duas melhores pessoas
    let mut table = [0i32; 35];
    for g in 0..7 {
        for s in 0..5 {
            let band = if g == 0 {
                if s != 2 {
                    continue;
                }
                0
            } else {
                0x80u32 << s
            };
            let mode = band | 1 << g;
            let (mut a, mut b, mut quantos) = (-10000i32, -10000i32, 0);
            for k in 0..n {
                let p = u(fichas + 4 * k as usize, 0);
                let r = rating(p, u(p, 0x61), 0, mode, a3, clube, c, u(c, 0x1b) as u32, u32::MAX, 0, 0, 0, 0, 0, 0).max(-10000);
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
            let sum = a.wrapping_add(b);
            table[g * 5 + s] = if n < 11 && g == 6 { sum / 2 } else { sum };
        }
    }
    if n < 11 {
        for (x, y) in [(5, 9), (10, 14), (15, 19)] {
            let v = table[x].min(table[y]);
            table[x] = v;
            table[y] = v;
        }
    }
    let tat = || u(G_TACTICS, 0);
    let how_many = || (*((tat() + 0x5fd) as *const i32)).wrapping_add(*((tat() + 0x5f5) as *const i32));
    let list = calloc(how_many() as u32, 8);
    let (mut melhor, mut melhor_r) = (-1i32, 0f64);
    let manager = u(c, 0x1f);
    let mut idx = 0i32;
    while idx < how_many() {
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
        let tactic = pegar(tat(), *(f as *const u16) as u32, 0);
        let flags = *((tactic + 0x584) as *const u32);
        if manager != 0 {
            if flags & 0x400 != 0 {
                x += (byte(manager, 0x1c) as i32 - 10) as f64;
            }
            if flags & 0x1000 != 0 {
                x += (10 - byte(manager, 0xe) as i32) as f64;
            }
            if flags & 0x100 != 0 {
                x += (byte(manager, 0x1f) as i32 - 10) as f64;
            }
            let fav = byte(manager, 0x43);
            let favorita = fav > 0 && fav < 10 && {
                let q = u(tat(), 8 + 4 * fav as usize);
                q != 0 && q == u(tactic, 0x187b)
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
        let mut sum = 0i32;
        for vaga in 0..11u32 {
            let p1 = position(tactic, vaga, 0);
            let p2 = position(tactic, vaga, 1) as u8;
            if manager != 0 {
                let r = marks(tactic, vaga) as u32;
                let fl = mark_b(tactic, vaga);
                if r & 0x8000 != 0 && flags & 0x100 == 0 {
                    x += byte(manager, 0x1f) as f64 * 0.25 - 2.5;
                }
                if r & 0x200000 != 0 {
                    x += byte(manager, 0x15) as f64 - 10.0;
                }
                if r & 0x800000 != 0 {
                    x += byte(manager, 0xe) as f64 * 0.5 - 5.0;
                }
                if fl & 4 != 0 {
                    x += byte(manager, 0x1a) as f64 * 1.5 - 15.0;
                }
            }
            let mais = |v: &mut f32, q: f64| *v = (*v as f64 + q) as f32;
            let (mut setor, mut band) = (0usize, -1i32);
            if p1 & 1 != 0 {
                band = 2;
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
            if band == -1 {
                band = if p1 & 0x80 != 0 {
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
            sum = sum.wrapping_add(table[setor * 5 + band as usize]);
        }
        // o equilibrio dos setores
        let z = (f18 as f64 - f44 as f64) as f32 as f64;
        let q = |v: f32| game_pow(v as f64, 2.0);
        let s_ = (q(f44) + q(f44) + q(f20)) - (q(f18) + q(f18)) - q(f28);
        let e = 44.0 - ((32.0 - s_) - (z - f20 as f64 * 0.5).abs() * (z - f20 as f64)) * 0.8;
        let y = match pp {
            1 => (e * 0.015 + 1.0) * x,
            3 => (1.0 - e * 0.015) * x,
            4 => (1.0 - e * 0.025) * x,
            _ => x,
        };
        let r = y * (sum as f64 / escala as f64);
        *((list + 8 * idx as usize) as *mut f64) = r;
        if melhor == -1 || r > melhor_r {
            melhor_r = r;
            melhor = idx;
        }
        idx += 1;
    }
    if list != 0 {
        free(list);
    }
    melhor
}

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
pub(crate) unsafe extern "thiscall" fn instructions(c: usize) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let attr: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_STAFF_GET_PLAYER_ATTRIBUTE);
    let attr_tec: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_STAFF_GET_NON_PLAYER_ATTRIBUTE);
    let attr_tec2: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let table: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let table_b: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_SECONDARY_POSITION);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FORMATION_SET_TEAM_INSTRUCTION);
    let pos: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_FORMATION_SET_SLOT_POSITION);
    let mark: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_FORMATION_SET_PLAYER_INSTRUCTION);
    let marks: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let marcar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_FORMATION_SET_MARKING_TARGET);
    let bench: unsafe extern "thiscall" fn(usize, usize, u32, usize, i32) = core::mem::transmute(FN_TACTICS_SET_PLAYMAKER);
    let who_marks: unsafe extern "thiscall" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_FIND_MARKING_TARGET);
    let average: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f32 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let (ca, cb) = (u(c, 0x2f), u(c, 0x33));
    let mut acc = 0i8;
    let f40 = (w(ca, 0x80) as i32 as f64 + w(cb, 0x80) as i32 as f64) * 5.882352941176471e-05;
    let human = byte(c, 0x19);
    if human == 0 {
        let p = u(ca, 0xcf);
        let q = *(G_STAFF_COUNT as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
            let manager = u(G_HUMAN_MANAGER, 0);
            let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
            if consultar(manager, p) != 0 && opcao(u(G_HUMAN_MANAGER, 0), 2, u(u(c, 0x2f), 0xcf)) != 0 {
                return;
            }
        }
    }
    let t = || u(c, 0x2b);
    let in_home = human != 0 || {
        let comp = byte(u(c, 0x37), 0x3a);
        (comp == 1 || comp == 9) && {
            let r = sortear(2000).wrapping_add(sortear(-4000));
            (w(u(c, 0x2f), 0x80) as i32 - w(u(c, 0x33), 0x80) as i32) > r
        }
    };
    if in_home {
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
    let tat = || u(G_TACTICS, 0);
    let hum = byte(c, 0x19) as i32;
    bench(tat(), u(c, 0x2f), 0, 0, hum);
    bench(tat(), u(c, 0x2f), 1, 0, byte(c, 0x19) as i32);
    let m = u(c, 0x12);
    let block = |k: usize| m + 0x22d8 * byte(c, 0x18) as isize as usize + 0x4796 + 0x1be * k;
    // o acumulado dos que sabem cobrar
    for k in 0..20 {
        let j = block(k);
        if byte(j, 0x19) >= 0 && table(j) & 0x40 != 0 && table(j) & 0x200 != 0 {
            let f = u(j, 0x6d);
            let v = byte(f, 0x2e);
            if v > 14 {
                acc = acc.wrapping_add(match_pl_ftol(game_pow((v as i32 - 14) as f64, 1.5)) as i8);
                let v40 = byte(f, 0x40);
                if v40 < 8 {
                    acc = acc.wrapping_add(8 - v40);
                }
            }
        }
    }
    let tc = |o: usize| byte(u(c, 0x1f), o);
    for k in 0..20 {
        let j = block(k);
        if byte(j, 0x19) < 0 {
            continue;
        }
        let f = u(j, 0x6d);
        let pos_j = *((j + 0x19) as *const u8) as u32;
        let a = |k: u32| attr(u(j, 0x69), k) as i32;
        let tb = || table(j);
        let fb = |o: usize| byte(u(j, 0x6d), o);
        let div = 13 - (attr_tec2(u(c, 0x1f), 0xb) as i32) / 2;
        let resto = (*(u(c, 0x1f) as *const i32)).wrapping_add(*(f as *const i32)) % div;
        let mut f38 = (resto as f64 * 0.05 + 1.0) * f40 * 1.2;
        if f38 < 0.6 {
            f38 = 0.6;
        }
        // avanco pelas pontas
        if *((t() + 0x187b) as *const u32) & 0x1c00000 == 0 {
            let lat = |band: u16, ref_: usize| tb() & band != 0 && fb(0x14) as i32 >= fb(ref_) as i32 - 1 && fb(0x19) == 20;
            if tb() & 0x10 != 0 && tb() & 0x200 != 0 && fb(0x14) as i32 > fb(0x13) as i32 - 1 && fb(0x19) == 20 && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x220, 1);
            } else if tb() & 0x10 != 0 && table_b(j) & 0x20 == 0 && lat(0x100, 0x13) && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 1);
            } else if tb() & 0x10 != 0 && table_b(j) & 0x20 == 0 && lat(0x400, 0x13) && f38 * 16.0 < a(7) as f64 {
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
            mark(t(), pos_j, 0x200000, 0);
            if (*((j + 0x4d) as *const f32) as f64) < 8500.0 || fb(0x3c) < 18 || fb(0x44) > 16 {
                mark(t(), byte(j, 0x19) as u8 as u32, 0x4000, 0);
            }
        } else {
            let b = a(0x14) + a(0xd) + a(7) + (tc(0x15) as i32 + fb(0x1a) as i32 - 20) * 3;
            if f38 * 40.0 > b as f64 {
                mark(t(), pos_j, 0x100000, 0);
            }
        }
        // 0x5b5270 bit 1 e +0x584 bit 8: 2, senao 4
        let dois_ou_quatro = || {
            let v = if marks(t(), byte(j, 0x19) as u8 as u32) & 1 != 0 && *((t() + 0x584) as *const u8) & 8 != 0 { 2 } else { 4 };
            mark(t(), byte(j, 0x19) as u8 as u32, v, 0);
        };
        if tb() & 0x20 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            dois_ou_quatro();
        }
        if (tb() & 8 != 0 || tb() & 0x10 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && f38 * 16.0 > a(7) as f64 {
            dois_ou_quatro();
            let r = who_marks(m, *((j + 0x27) as *const u8) as u32, byte(j, 0x19) as u8 as u32);
            if r != -1 {
                let side = (byte(j, 0x19) == 0) as usize;
                let o = m + 0x1be * (r as isize as usize).wrapping_add(20 * side) + 0x4796;
                let blk = || (u(c, 0x12) as isize + 0x91e2 + 0x18e3 * byte(j, 0x27) as isize) as usize;
                let tbo = || table(o);
                let reps = || {
                    let ra = w(u(c, 0x2f), 0x80);
                    ra < w(u(c, 0x33), 0x80).wrapping_add(0xfa) || w(u(o, 0x6d), 0xb).wrapping_add(0xfa) > ra
                };
                let lados = || tbo() & 0x200 != 0 || tbo() & 0x100 != 0 || tbo() & 0x400 != 0;
                let resto5 = || *(u(o, 0x69) as *const i32) % 5;
                if u(o, 0x69) != 0 {
                    if (tbo() & 0x20 != 0 || table_b(o) & 0x20 != 0)
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
                        && (tbo() & 0x20 != 0 || table_b(o) & 0x20 != 0)
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
                        && (tbo() & 0x10 != 0 || table_b(o) & 0x10 != 0)
                        && lados()
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 12
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(8) as i32
                    {
                        let fo = u(o, 0x6d);
                        let g = u(G_PLAYER_STATS, 0);
                        let m1 = average(g, fo, 6, 1, 6.5);
                        let m2 = average(u(G_PLAYER_STATS, 0), u(o, 0x6d), 6, 2, 6.5);
                        let m3 = average(u(G_PLAYER_STATS, 0), u(o, 0x6d), 6, 5, 6.5);
                        let (mut st0, mut r3) = (0.0f64, 0.0f32);
                        if m1 as f64 > 2.0 {
                            st0 = m2 as f64 / m1 as f64;
                            r3 = (m3 as f64 / m1 as f64) as f32;
                        }
                        if st0 > 0.35 || r3 as f64 > 0.4 {
                            if (tbo() & 0x20 != 0 || table_b(o) & 0x20 != 0) && (fb(0x12) == 20 || fb(0x13) == 20) && a(0x1a) > 14 {
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
            mark(t(), byte(j, 0x19) as u8 as u32, 0x10000000, 0);
        } else if f38 * 13.0 < a(0x12) as f64 {
            mark(t(), byte(j, 0x19) as u8 as u32, 0x20000000, 0);
        }
        // chutes de longe
        if (tb() & 8 != 0 || tb() & 0x10 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            let a2 = a(2);
            if (tc(0x1f) as i32 / 5) as f64 + q > a2 as f64 {
                let a3 = a(0x1f);
                let pj = byte(j, 0x19) as u8 as u32;
                if (tc(0x1f) as i32 / 5) as f64 + f38 * 5.0 > a3 as f64 {
                    let v = if marks(t(), pj) & 0x200 != 0 && *((t() + 0x584) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                    mark(t(), pj, v, 0);
                    mark(t(), byte(j, 0x19) as u8 as u32, 0x8000, 0);
                } else {
                    let a3 = a(0x1f);
                    if (tc(0x1f) as i32 / 5) as f64 + q > a3 as f64 {
                        let v = if marks(t(), pj) & 0x200 != 0 && *((t() + 0x584) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                        mark(t(), byte(j, 0x19) as u8 as u32, v, 0);
                    }
                }
            }
        }
        if byte(j, 0x24) != 0 && a(2) > tc(0x1f) as i32 / 4 + 8 {
            mark(t(), byte(j, 0x19) as u8 as u32, 0x800, 0);
        }
        // dribles
        if tb() & 8 != 0 || tb() & 0x10 != 0 || (tc(0xe) > 16 && tb() & 0x40 != 0) {
            let f28 = f38 * 14.0;
            let te4 = tc(0xe) as i32 / 4;
            let pj = || byte(j, 0x19) as u8 as u32;
            let fl8000 = || *((j + 0x4d) as *const f32) as f64;
            if f28 - (te4 as f64) < a(0x14) as f64 && f28 - ((tc(0xe) as i32 / 4) as f64) < a(0xc) as f64 && fl8000() > 8000.0 {
                mark(t(), pj(), 0x800000, 0);
            } else {
                let te4 = tc(0xe) as i32 / 4;
                let lim = q - te4 as f64;
                let high = lim > a(0x14) as f64 && lim < a(0xc) as f64;
                if high || fl8000() < 8000.0 {
                    mark(t(), pj(), 0x400000, 0);
                }
            }
        }
        // desarmes: os de meio e ataque (ou quem desarma bem) entram pela nota mais alta
        {
            let pj = || byte(j, 0x19) as u8 as u32;
            let entra = tb() & 0x10 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0 || f38 * 16.0 < a(0xb) as f64;
            let strong = entra && (f38 * 14.0 < a(0xb) as f64 || f38 * 34.0 - ((tc(0xe) as i32 / 4) as f64) < (a(0x15) + a(0xb)) as f64);
            if strong {
                mark(t(), pj(), 0x2000000, 0);
            } else if f38 * 25.0 - (tc(0xe) as i32 / 4) as f64 > (a(0x15) + a(0xb)) as f64 {
                mark(t(), pj(), 0x1000000, 0);
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
            mark(t(), byte(j, 0x19) as u8 as u32, 0x8000000, 0);
        } else if d() < -5 {
            mark(t(), byte(j, 0x19) as u8 as u32, 0x4000000, 0);
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
            mark(t(), pj(), 0, 4);
        } else if k2 - l28 > a(8) as f64 {
            mark(t(), pj(), 0, 8);
        }
        // cruzamentos pela tabela
        let tv = tb() as i16 as i32;
        let b = || a(0x16) + a(7);
        match tv {
            0x210 | 0x110 | 0xa0 | 0x120 | 0x420 | 0x220 | 0x410 | 0x820 => {
                if f38 * 23.0 < b() as f64 {
                    mark(t(), pj(), 0, 1);
                }
            }
            0x90 | 0x108 | 0x202 | 0x208 | 0x408 | 0x810 => {
                if f38 * 27.0 < b() as f64 {
                    mark(t(), pj(), 0, 1);
                }
            }
            _ => {
                if f38 * 31.0 < b() as f64 {
                    mark(t(), pj(), 0, 1);
                } else if f38 * 20.0 > b() as f64 {
                    mark(t(), pj(), 0, 2);
                }
            }
        }
        // bola parada
        if (tb() & 0x10 != 0 || tb() & 8 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && table_b(j) & 0x40 == 0 {
            let ca = u(c, 0x2f);
            let s = a(0x16) + a(9) + a(7);
            if ((w(ca, 0x80) as i32 / 500) as f64 + f38 * 30.0) < s as f64 {
                let pj = u(j, 0x69);
                if u(c, 6) == pj || u(c, 0xa) == pj || u(c, 0xe) == pj {
                    bench(tat(), ca, 0, pj, byte(c, 0x19) as i32);
                }
            }
        }
    }
}

/// Port of 0x6e0d00: puts the person in slot pos (0..19) of the staff block (+0x23) via injected 0x6eaea0; returns 1,
/// or 0 without person/bad slot.
///
/// [pt] { 0x6e0d00 } (thiscall, ret 0xc, match_man.cpp): poe `pessoa` na vaga `pos` (0..19) do bloco da
/// comissao (+0x23) pela 0x6eaea0 (motor +0x12, lado +0x18, `extra`). Sem pessoa devolve 0; senao 1.
/// simplification: vaga fora de 0..19 para o jogo com erro no original (match_man.cpp 0xc39); aqui so devolve 0.
/// A 0x6eaea0 (inicia o jogador, com sorteios e efeitos fora do motor) vem injetada: na comparacao
/// ela e unidade da fita (a original grava; a copia Rust confere e nao roda de novo).
pub(crate) unsafe fn place_in_slot_with(c: usize, pos: u32, person: usize, extra: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let p = pos as u8 as i8;
    if person == 0 || !(0..=19).contains(&p) {
        return 0;
    }
    let j = *((c + 0x23) as *const usize) + 0x1be * p as usize;
    call(FN_MATCH_PLAYER_SETUP, j, [*((c + 0x12) as *const usize) as u32, person as u32, *((c + 0x18) as *const u8) as u32, pos, extra]);
    1
}

/// Hook entry for 0x6e0d00 (place_in_slot_with): puts `pessoa` in slot `pos` (0..19) of the staff's player block
/// (+0x23), setting up the match player (0x6eaea0) with the engine, side and `extra`; returns 1, or 0 without a person
/// or with a bad slot.
pub(crate) unsafe extern "thiscall" fn put_in_place(c: usize, pos: u32, person: usize, extra: u32) -> u8 {
    place_in_slot_with(c, pos, person, extra, &mut |_, this, a| {
        let iniciar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_PLAYER_SETUP);
        iniciar(this, a[0], a[1], a[2], a[3], a[4]);
        0
    })
}

// Porta de 0x6eaea0 (inicia o jogador na vaga): unidade da fita na comparacao.
pub(crate) static mut INIT_MATCH_PLAYER_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_6EAEA0: Prologo = Prologo { endereco: 0x6eaea0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c] };

/// Hook entry for 0x6eaea0 (initialise match player): runs the original directly, or records it as a tape unit while
/// recording.
pub(crate) unsafe extern "thiscall" fn init_player_hook(j: usize, m: u32, person: u32, side: u32, pos: u32, extra: u32) {
    let o: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(INIT_MATCH_PLAYER_TRAMPOLINE);
    if !is_recording() {
        return o(j, m, person, side, pos, extra);
    }
    call_recording(FN_MATCH_PLAYER_SETUP, j, [m, person, side, pos, extra], || {
        o(j, m, person, side, pos, extra);
        0
    });
}

/// Comparator ends for the substitution sort: players without person go last, +0x19 = -2 and goalkeepers with +0xf < 20
/// first; None when the rest decides.
///
/// [pt] As duas pontas das ordens da substituicao: sem pessoa (+0x6d) vai para o fim; +0x19 = -2 vem
/// primeiro; goleiro (tabela 1) com ficha +0xf abaixo de 20 tambem. Devolve None quando o resto decide.
pub(crate) unsafe fn substitution_order_ends(a: usize, b: usize) -> Option<i32> {
    let table: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
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
    if table(a) == 1 && byte(u(a, 0x6d), 0xf) < 20 {
        return Some(-1);
    }
    if table(b) == 1 && byte(u(b, 0x6d), 0xf) < 20 {
        return Some(1);
    }
    None
}

/// Player's condition: +0x35 (i16) + +0x4d (f32), truncated.
///
/// [pt] +0x35 (curto) + +0x4d (float), truncado.
pub(crate) unsafe fn match_condition(j: usize) -> i32 {
    match_pl_ftol(*((j + 0x35) as *const i16) as f64 + *((j + 0x4d) as *const f32) as f64)
}

/// qsort comparator for substitutions: compares two match players by condition (+0x35 + +0x4d), weighted by the first
/// player's position (x1.35 for 2 and 4, x1.2/x1.25 for 8, x1.1 for 0x10). Returns a - b.
///
/// [pt] { 0x6e0df0 } (cdecl, comparador do qsort da substituicao): pela condicao (+0x35 + +0x4d), pesada
/// pela tabela de `a` (2 e 4: x1,35; 8: x1,2 em `a` e x1,25 em `b`; 0x10: x1,1) nos dois lados.
pub(crate) unsafe extern "C" fn order_by_table(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = (*pa, *pb);
    if let Some(r) = substitution_order_ends(a, b) {
        return r;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let pesar = |v: i32, oito: f64| -> i32 {
        match table(a) {
            4 | 2 => match_pl_ftol(v as f64 * 1.35),
            8 => match_pl_ftol(v as f64 * oito),
            0x10 => match_pl_ftol(v as f64 * 1.1),
            _ => v,
        }
    };
    let va = pesar(match_condition(a), 1.2);
    let vb = pesar(match_condition(b), 1.25);
    va.wrapping_sub(vb)
}

/// qsort comparator for substitutions: after the fixed ends (players without person last, goalkeepers first...), orders
/// by match condition (+0x35 + +0x4d), ascending.
///
/// [pt] { 0x6e0fe0 } (cdecl, comparador do qsort da substituicao): so pela condicao (+0x35 + +0x4d).
pub(crate) unsafe extern "C" fn order_by_condition(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = (*pa, *pb);
    if let Some(r) = substitution_order_ends(a, b) {
        return r;
    }
    let vb = match_condition(b);
    match_condition(a).wrapping_sub(vb)
}

/// for each place from 0 to 10 + (match +0x4d & 0xf), the person from the club's tactic (0x8968f0) comes in (0x6e0d00)
/// if able to play (0x62c400 ≠ 2, 0x55d270 and 0x7757d0 without impediment); returns how many
/// Original: `starters` (0x6dbb80, match_man.cpp, thiscall).
///
/// [pt] { 0x6dbb80 } (thiscall, match_man.cpp): os titulares do inicio do jogo. Para cada vaga de 0 a
/// 10 + (partida +0x4d & 0xf), a pessoa da tatica do clube (0x8968f0) entra (0x6e0d00) se puder jogar:
/// 0x62c400 diferente de 2, e (partida sem competicao, -2, ou 0x55d270 sem impedimento) e 0x7757d0 sem
/// impedimento. Devolve quantos entraram.
pub(crate) unsafe extern "thiscall" fn starters(c: usize) -> u8 {
    starters_with(c, &mut |_, this, a| {
        let colocar: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM);
        colocar(this, a[0], a[1] as usize, a[2]) as u32
    })
}

/// Port of `starters` with 0x6e0d00 injected: places each tactic-selected player who is not injured, banned or on
/// international duty; returns how many.
///
/// [pt] `titulares` com a 0x6e0d00 (poe na vaga) injetada.
pub(crate) unsafe fn starters_with(c: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let of_tactic: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let state: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let total = || (*((u(c, 0x37) + 0x4d) as *const u8) & 0xf) as i32 + 11;
    let mut n = 0u8;
    let mut k = 0i8;
    while (k as i32) < total() {
        let pos = k as u8 as u32;
        let p = of_tactic(u(G_TACTICS, 0), u(c, 0x2f), pos, byte(c, 0x19) as i32);
        if p != 0 && state(u(G_INJURY, 0), p) != 2 {
            let jogo = u(c, 0x37);
            let free = *(jogo as *const i32) == -2 || suspenso(u(G_DISCIPLINE, 0), p, u(jogo, 0x14), 1, 0) == 0;
            if free && impedido(u(G_NATIONAL_TEAMS, 0), p, G_CURRENT_DATE, u(c, 0x2f), 0, 0) == 0 && call(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM, c, [pos, p as u32, pos, 0, 0]) as u8 != 0 {
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
pub(crate) unsafe extern "thiscall" fn store_tactic(c: usize) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let clube = u(c, 0x2f);
    if clube == 0 || u(G_TACTICS, 0) == 0 {
        return;
    }
    if byte(c, 0x19) != 0 {
        let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        let confirma: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
        if human(clube, 1, 1, 0, 0) != 0 && confirma(u(G_HUMAN_MANAGER, 0), u(u(c, 0x2f), 0xcf)) == 0 {
            return;
        }
    }
    let store: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TACTICS_STORE_SELECTION_RECORD);
    store(u(G_TACTICS, 0), u(c, 0x2f), c + 0x3f, byte(c, 0x19) as i32);
}

/// Shows the game's "v%s %s.%s %d" error box for source file `arquivo` and line (game continues), clearing
/// G_MOUSE_LEFT_BUTTON_DOWN.
///
/// [pt] A caixa de erro "v%s %s.%s %d" do fonte `arquivo` (0xa15da4 match_man.cpp, 0xa15708
/// match_day.cpp) na linha `linha` (o jogo segue).
pub(crate) unsafe fn show_error_at(file: usize, row: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    split(file, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
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
pub(crate) unsafe extern "thiscall" fn select_automatically(t: usize, other: usize, mode: u32) -> i8 {
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

    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy_list: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mais_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let data_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let strength: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let games: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let rival_de: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let in_competition: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_CLUB_STILL_IN_CONTENTION);
    let count: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let state: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let partner: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let best: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_MATCH_MAN_CHOOSE_CAPTAIN);
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let store_instructions: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_TACTICS_CLEAR_OR_RESTORE_SELECTION);
    let marked_slot: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(FN_TACTICS_FIND_SELECTION_SLOT);
    let mark: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let formacao: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC);
    let home: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let marks: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let mark_b: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let rodizio: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
    #[allow(clippy::type_complexity)]
    let reavaliar: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, u32) = core::mem::transmute(FN_MATCH_MAN_APPLY_SQUAD_RESTRICTION);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let of_tactic: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let colocar: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM);
    let write: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(FN_TACTICS_SET_SELECTED_PLAYER);
    let capitao: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
    let instructions: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_SET_COMPUTER_INSTRUCTIONS);
    let stance: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_MENTALITY);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FORMATION_SET_TEAM_INSTRUCTION);

    let clube = u(t, 0x2f);
    let jogo = u(t, 0x37);
    let m = u(t, 0x12);
    let side = byte(t, 0x18) as isize;
    let weight_of_side = || *(((m as isize) + 0x1ad + side) as *const i8);
    let competicao = |id: i32| *((u(G_COMP_OBJECT_TABLE, 0) as isize + 4 * id as isize) as *const usize);
    let tactic = *(G_TACTICS as *const usize);
    let manager = *(G_HUMAN_MANAGER as *const usize);
    let mut mode = mode;

    list_new(f + 0x80);
    data_default(f + 0x38);
    mais_dias(G_CURRENT_DATE, f + 0xac, 2);
    if other == 0 {
        match_man_fatal_error(0x174);
    }
    *b(0x13) = (*((u(m, 0x4792) + 0x4d) as *const u8) & 0xf).wrapping_add(0xb);
    let strong = strength(clube, 0, 1) as i8;
    *b(0x12) = strong as u8;
    *tb(0x16) = 0;

    // fora do humano: pesa o jogo de hoje contra o proximo da lista (poupar para o mais importante?)
    if byte(t, 0x19) == 0 && h16(clube, 0x80) >= 0x1770 && *(jogo as *const i32) != -2 && human(clube, 0, 1, 0, 0) == 0 && national_team(clube) == 0 && strong > 0x1e {
        mais_dias(G_CURRENT_DATE, f + 0xa4, 1);
        mais_dias(G_CURRENT_DATE, f + 0x40, 7);
        let r = games(u(G_FIXTURE_MANAGER, 0), rd(0xa4), rd(0xa6), rd(0x40), rd(0x42), *(clube as *const u32), f + 0x80, u32::MAX, 0xffff_fffe, 1);
        if r as u16 != 0 {
            // 333 x a reputacao da competicao (.69; a da [0x9cf6f8] na tabela [0xae23d0]) + reputacao/3
            let weight = |comp: usize, rival: usize| -> i32 {
                let c = if *(comp as *const u32) == *(KEY_CLUB_COMP_INTER_TOTO_CUP as *const u32) { u(G_CLUB_COMP_TABLE, 0) + u(KEY_CLUB_COMP_UEFA_CUP, 0).wrapping_mul(0x6b) } else { comp };
                (h16(c, 0x69) as i32 * 333).wrapping_add(h16(rival, 0x80) as i32 / 3)
            };
            let comp = u(jogo, 0x14);
            let id = *(comp as *const u32);
            let mut l14 = weight(comp, other);
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
            let mut factor = weight_of_side() as f64 * read_f64_at(0x96d438) + read_f64_at(0x967890); // 0x96d438 = 0.06666666667, 0x967890 = 1
            if factor > read_f64_at(0x968dc0) { // 0x968dc0 = 1.4
                factor = read_f64_at(0x968dc0); // 0x968dc0 = 1.4
            }
            let mut esi = match_pl_ftol(l14 as f64 * factor);
            l14 = esi;
            if rival_de(other, clube) != 0 {
                esi = match_pl_ftol(l14 as f64 * read_f64_at(0x968f98)); // 0x968f98 = 1.75
                l14 = esi;
                *tb(0x16) = 1;
            } else if other == u(clube, 0xb3) || other == u(clube, 0xb7) || other == u(clube, 0xbb) {
                esi = match_pl_ftol(l14 as f64 * read_f64_at(0x968940)); // 0x968940 = 1.5
                l14 = esi;
            }
            if in_competition(competicao(*(jogo as *const i32)), clube) == 0 {
                *tb(0x16) = 0;
                let tec = u(t, 0x1b);
                l14 = if tec != 0 { match_pl_ftol(byte(tec, 0x5c) as f64 * read_f64_at(0x968a90) * l14 as f64) } else { esi / 2 }; // 0x968a90 = 0.04
            }
            wd(0x14, l14 as u32);
            wd(0x8a, rd(0x82));
            let x = next(f + 0x80);
            wd(0x20, x as u32);
            if x != 0 && *(x as *const i32) != -2 {
                let d = data_new(f + 0x24, *((x + 0x2a) as *const u16) as u32, *((x + 0x28) as *const u16) as u32);
                copiar(f + 0x38, f + 0x18, d);
                let rival = if clube == u(x, 0x1c) { u(x, 0x20) } else { u(x, 0x1c) };
                let mut l18 = weight(u(x, 0x14), rival);
                let mut esi = l18;
                if rival_de(rival, clube) != 0 {
                    esi = match_pl_ftol(l18 as f64 * read_f64_at(0x968940)); // 0x968940 = 1.5
                    l18 = esi;
                } else if rival == u(clube, 0xb3) || rival == u(clube, 0xb7) || rival == u(clube, 0xbb) {
                    esi = match_pl_ftol(l18 as f64 * read_f64_at(0x969588)); // 0x969588 = 1.35
                    l18 = esi;
                }
                if u(x, 0x18) != u(clube, 0x69) {
                    let c = if u(clube, 0x53) != u(rival, 0x53) { read_f64_at(0x968978) } else { read_f64_at(0x9689c0) }; // 0x968978 = 1.2, 0x9689c0 = 1.1
                    esi = match_pl_ftol(l18 as f64 * c);
                    l18 = esi;
                }
                if in_competition(competicao(*(x as *const i32)), clube) == 0 {
                    l18 = esi / 3;
                } else if count(*(jogo as *const i32)) != 0 && (byte(x, 0x3a) == 8 || byte(x, 0x3a) == 7) && *(x as *const i32) > -1 {
                    let c = competicao(*(x as *const i32));
                    if c != 0 && u(c, 8) != 0 {
                        let r = search(u(c, 8), clube);
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
                    let p = mais_dias(G_CURRENT_DATE, f + 0x48, k as u32 + 1);
                    wd(0x24, *(p as *const u32));
                    wd(0x28, *((p + 4) as *const u32));
                    if rw(0x26) == rw(0x3a) && rw(0x24) == rw(0x38) {
                        if let Some(c) = c {
                            l18 = match_pl_ftol(l18 as f64 * read_f64_at(*c));
                            wd(0x18, l18 as u32);
                        }
                        break;
                    }
                }
                let (a, b2) = (l18 as f64, l14 as f64);
                if a > b2 + b2 && strong > 0x28 && l14 < 0x1b58 {
                    *tb(0x1a) = 0;
                } else if a > b2 * read_f64_at(0x968dc0) && strong > 0x23 && l14 < 0x2134 { // 0x968dc0 = 1.4
                    *tb(0x1a) = 1;
                } else if a > b2 * read_f64_at(0x9689c0) && l14 < 0x2710 { // 0x9689c0 = 1.1
                    *tb(0x1a) = 2;
                }
            }
        }
    }

    // poupando: as mascaras das vagas e a primeira escala, com os que podem jogar
    let emprestado_fora = |p: usize| -> bool {
        // emprestado (0x4cf6d0) a outro clube que nao o parceiro (0x540a50)?
        let owner = |p: usize| {
            let c = contrato(u(G_CONTRACT_MANAGER, 0), p);
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            if c != 0 { u(G_CLUB_TABLE, 0).wrapping_add(u(c, 4).wrapping_mul(club::SIZE)) } else { 0 }
        };
        if emprestado(u(G_CONTRACT_MANAGER, 0), p) == 0 {
            return false;
        }
        if owner(p) == clube {
            return true;
        }
        owner(p) == partner(clube, 0, 1)
    };
    if byte(t, 0x1a) != 3 && human(clube, 0, 1, 0, 0) == 0 && national_team(clube) == 0 {
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
            if state(u(G_INJURY, 0), p) == 2 {
                continue;
            }
            if byte(t, 0x19) != 0 {
                if *(jogo as *const i32) != -2 && suspenso(u(G_DISCIPLINE, 0), p, u(jogo, 0x14), 1, 0) != 0 {
                    continue;
                }
            } else if x != 0 {
                if u(jogo, 0x14) != u(x, 0x14) && suspenso(u(G_DISCIPLINE, 0), p, u(x, 0x14), 1, 0) != 0 {
                    continue;
                }
                if ja_jogou(u(G_TRANSFER_MANAGER, 0), p, x) != 0 || impedido(u(G_NATIONAL_TEAMS, 0), p, f + 0x38, clube, 0, 0) != 0 {
                    continue;
                }
            }
            let day = if *(jogo as *const u32) == *(KEY_CLUB_COMP_RESERVE as *const u32) { f + 0xac } else { G_CURRENT_DATE };
            if impedido(u(G_NATIONAL_TEAMS, 0), p, day, clube, 0, 0) != 0 || emprestado_fora(p) {
                continue;
            }
            let c = *b(0x18) as usize;
            wd(0x278 + 4 * c, p as u32);
            *b(0x18) = (c + 1) as u8;
        }
        if *b(0x18) != 0 {
            let r = best(t, f + 0x278, rd(0x18));
            *((t + 0x3b) as *mut usize) = r;
            if r == 0 {
                match_man_fatal_error(0x302);
            }
        }
        let cf = u(clube, 0xcf);
        let a7 = if byte(t, 0x19) == 0 {
            0xa520c2
        } else if human(clube, 1, 1, 0, 0) != 0 && cf != 0 && consultar(manager, cf) != 0 && opcao(manager, 3, u(clube, 0xcf)) != 0 {
            0xc520c2
        } else {
            0x8520c2
        };
        pick(f + 0x278, rd(0x18), f + 0x50, rd(0x30), 0, f + 0x56c, a7, clube, t, u(t, 0x1b) as u32, 0, 0, 0, 0, 0);
    }

    // os candidatos: quem pode jogar entra na lista; no modo 1 os das vagas marcadas so as ocupam
    let humano19 = byte(t, 0x19) as i32 as u32;
    let bl = mode as u8;
    if bl == 2 {
        store_instructions(tactic, clube, humano19, 0, 0);
    }
    let slot = |p: usize| marked_slot(tactic, clube, p, humano19) as i8;
    let mark_se = |p: usize| {
        if bl == 1 && slot(p) > -1 {
            mark(tactic, clube, p, humano19, 1);
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
            !(national_team(clube) != 0 && m5e(p) & 8 != 0)
        } else if human(clube, 0, 1, 0, 0) != 0 {
            if national_team(clube) != 0 {
                m5e(p) & 8 != 0
            } else if byte(t, 0x19) != 0 && m5e(p) & 1 != 0 && slot(p) == -1 {
                false
            } else {
                byte(t, 0x19) != 0 || m5e(p) & 2 == 0
            }
        } else if *(p as *const i32) >= *(G_STAFF_COUNT as *const i32) {
            true
        } else if national_team(clube) != 0 {
            m5e(p) & 4 == 0
        } else {
            let mut away = false;
            let mut j = 0i8;
            while j < *b(0x30) as i8 {
                if rd(0x56c + 4 * j as usize) as usize == p {
                    away = true;
                    break;
                }
                j += 1;
            }
            !away
        };
        if !segue {
            continue;
        }
        if national_team(clube) == 0 && cedido(u(G_TRANSFER_MANAGER, 0), p, clube, 0) != 0 {
            mark_se(p);
            continue;
        }
        if state(u(G_INJURY, 0), p) == 2 || (byte(t, 0x19) != 0 && state(u(G_INJURY, 0), p) != 0) {
            mark_se(p);
            continue;
        }
        let away = (*(jogo as *const i32) != -2 && suspenso(u(G_DISCIPLINE, 0), p, u(jogo, 0x14), 1, 0) != 0)
            || impedido(u(G_NATIONAL_TEAMS, 0), p, G_CURRENT_DATE, clube, 0, 0) != 0
            || (*(jogo as *const i32) != -2 && ja_jogou(u(G_TRANSFER_MANAGER, 0), p, jogo) != 0)
            || emprestado_fora(p);
        if away {
            mark_se(p);
            continue;
        }
        if bl == 1 {
            let v = slot(p);
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
        mode = (mode & !0xff) | 2;
    }

    // as marcas da escala
    let mut mask: u32 = if national_team(clube) != 0 { 0x208a09c } else { 0xca0ca };
    if byte(t, 0x19) != 0 {
        mask = (mask | 0x20100) ^ 0x80000;
    } else {
        let dif = || h16(clube, 0x80) as i32 - h16(other, 0x80) as i32;
        let a = byte(jogo, 0x3a);
        let mut done = false;
        if a == 1 || a == 9 {
            let lim = sorteio(2000).wrapping_add(sorteio(-4000));
            if dif() > lim {
                mask |= 0x100;
                done = true;
            }
        }
        if !done {
            let lim = sorteio(3000) + 3000;
            if dif() > lim {
                mask |= 0x100;
                if national_team(clube) == 0 {
                    mask |= 0x10000;
                }
            } else {
                let j0 = *(jogo as *const i32);
                if byte(t, 0x1a) != 3 || (j0 > -1 && competicao(j0) != 0 && in_competition(competicao(j0), clube) == 0) {
                    mask = (mask | 0x100) ^ 0x80000;
                } else if national_team(clube) == 0 {
                    mask |= 0x10000;
                }
            }
        }
    }
    if byte(t, 0x16) != 0 {
        mask |= 0x800000;
    }
    wd(0x14, mask);

    // as tres referencias (a maior ficha +0xd + +0xb, sem repetir)
    if *b(0x18) != 0 && bl == 2 {
        let r = best(t, f + 0x278, rd(0x18));
        *((t + 0x3b) as *mut usize) = r;
        if r == 0 {
            match_man_fatal_error(0x42b);
        }
        if mask & 0x80000 != 0 {
            for s in 0..3usize {
                ww(0x40 + 2 * s, 0xd8f0u16 as i16);
                ww(0x24 + 2 * s, -1);
                let mut c = 0i8;
                while c < *b(0x18) as i8 {
                    let p = rd(0x278 + 4 * c as usize) as usize;
                    let c16 = c as i16;
                    let pula = p == 0
                        || *(p as *const i32) >= *(G_STAFF_COUNT as *const i32)
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
    formacao(t, f + 0x278, rd(0x18), rd(0x14), mode);

    // as vagas: as 11 da formacao (casas, marcas) e as extras sorteadas
    let n = *b(0x13) as i8 as i32;
    wd(0x30, n as u32);
    *b(0x12) = 0;
    *b(0x20) = 0;
    *b(0x40) = 0;
    let tat = || u(t, 0x2b);
    let extra = |o: usize, k: usize, first: i32| {
        if k == 12 && *b(0x2f) == 0 && (*b(0x13) as i8 > 14 || sorteio(first) == 0) {
            *b(0x2f) = 1;
            true
        } else {
            let v = rw(o);
            ww(o, if v < 3 { 4 } else if v >= 6 { 0x40 } else { 0x10 });
            false
        }
    };
    for k in 0..n.max(0) as usize {
        if mode as u8 == 1 {
            if *b(0x90 + k) != 0 {
                continue;
            }
            let c = *b(0x12) as i8 as usize;
            if k <= 10 {
                ww(0xe8 + 2 * c, home(tat(), k as u32, 0) as i16);
                ww(0x214 + 2 * c, home(tat(), k as u32, 1) as i16);
                let q = marks(tat(), k as u32);
                wd(0x5bc + 8 * c, q as u32);
                wd(0x5c0 + 8 * c, (q >> 32) as u32);
                *b(0xb4 + c) = mark_b(tat(), k as u32);
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
            ww(0xe8 + 2 * k, home(tat(), k as u32, 0) as i16);
            ww(0x214 + 2 * k, home(tat(), k as u32, 1) as i16);
            let q = marks(tat(), k as u32);
            wd(0x5bc + 8 * k, q as u32);
            wd(0x5c0 + 8 * k, (q >> 32) as u32);
            *b(0xb4 + k) = mark_b(tat(), k as u32);
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
    if human(clube, 1, 1, 0, 0) != 0 && byte(t, 0x19) == 0 && cf != 0 && consultar(manager, cf) != 0 && opcao(manager, 3, u(clube, 0xcf)) != 0 {
        wd(0x14, rd(0x14) | 0x400000);
    }
    pick(f + 0x278, rd(0x18), f + 0xe8, rd(0x20), rd(0x40), f + 0x14c, rd(0x14), clube, t, u(t, 0x1b) as u32, 0, f + 0x5bc, f + 0xb4, *((u(t, 0x2b) + 0x584) as *const u32), f + 0x214);
    let n = rd(0x30) as i32;
    if n - 1 > 0 {
        for i in 0..(n - 1) as usize {
            for j in i + 1..n as usize {
                let a = rd(0x14c + 4 * i);
                if a != 0 && a == rd(0x14c + 4 * j) {
                    show_error_at(0xa15da4, 0x4c9);
                }
            }
        }
        wd(0x20, n as u32);
    }

    // o rodizio (0x8cf880 aponta quem descansa; 0x6e8910 reavalia), no maximo duas voltas
    if *(jogo as *const i32) != -2 {
        *b(0x2e) = 0;
        let mut r = rodizio(u(G_TRANSFER_MANAGER, 0), clube, jogo, 0, f + 0x14c, 2, f + 0x78);
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
                    let inside = nb as i8 > 0 && (0..rd(0x30) as usize).any(|j| rd(0x14c + 4 * j) == p);
                    if !inside {
                        *((w + 4 * *b(0x20) as i8 as usize) as *mut u32) = p;
                        *b(0x20) = (*b(0x20)).wrapping_add(1);
                    }
                }
                w
            } else {
                0
            };
            // simplification: o 6o argumento leva so o byte (n - 11); a original passa sobra de ebx nos altos (lidos como byte)
            reavaliar(clube, u(t, 0x1b), f + 0x78, rd(0x14), 0xb, nb.wrapping_sub(0xb) as u32, rd(0x20), f + 0x340, v, w, f + 0x14c, f + 0xe8, f + 0x214, f + 0x5bc, f + 0xb4, 0);
            free(v);
            if w != 0 {
                free(w);
            }
            r = rodizio(u(G_TRANSFER_MANAGER, 0), clube, jogo, 0, f + 0x14c, 2, f + 0x78);
        }
    }
    if mode as u8 == 1 {
        *b(0x12) = 0;
        *b(0x24) = 0;
        let nn = *b(0x13) as i8;
        if nn > 0 {
            for k in 0..nn as usize {
                let p = if *b(0x90 + k) != 0 {
                    of_tactic(tactic, clube, rd(0x24), byte(t, 0x19) as i32) as u32
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
    let cap = best(t, f + 0x14c, 0xb);
    let n = rd(0x30) as i32;
    for k in 0..n.max(0) as usize {
        colocar(t, k as u32, rd(0x14c + 4 * k) as usize, k as u32);
        let p = rd(0x14c + 4 * k) as usize;
        if p != 0 {
            write(tactic, clube, k as u32, p, humano19);
            *b(0x37) = (*b(0x37)).wrapping_add(1);
            if rd(0x14c + 4 * k) as usize == cap {
                capitao(tactic, clube, k as u32, humano19);
            }
        }
    }
    let n_final = n.max(0) as usize;
    instructions(t);
    let name = if u(t, 0x2b) != 0 { u(t, 0x2b) } else { 0xa15934 };
    let name = core::ffi::CStr::from_ptr(name as *const core::ffi::c_char).to_bytes();
    let special = name != b"3-4-3" && name != b"3-5-2 Attacking";

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
    let sheet = u(t, 0x1f);
    let lim = |base: i32| if weight_of_side() > 0 { base + 3 } else { base };
    let disposicao = |i: isize| *(((m as isize) + 0x476a + 4 * i) as *const f32) as f64;
    let e = |o: usize| byte(sheet, o) as i32;
    match stance(t) as i8 {
        1 => {
            let d = disposicao(side);
            let x = if sheet != 0 && !(d >= read_f64_at(0x968e18)) { // 0x968e18 = 2.5
                if e(0xe) > 6 && special { 0x10000 } else { 0x20000 }
            } else if special || d > read_f64_at(0x9688b0) { // 0x9688b0 = 10
                0x10000
            } else {
                0x20000
            };
            a(x);
        }
        3 => {
            if sheet == 0 {
                g(0x8000);
                bx(0x1000);
            } else {
                g(if e(0xe) > 0x10 { 0x20000 } else { 0x8000 });
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        4 => {
            if sheet != 0 {
                g(if e(0xe) > 0x12 { 0x20000 } else { 0x8000 });
            } else {
                g(0x8000);
            }
            if sheet != 0 && e(0xe) > 0xc {
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            } else {
                g(0x800);
                g(0x4000);
            }
        }
        5 => {
            if sheet == 0 {
                g(0x20000);
                bx(0x1000);
            } else {
                g(if e(0xe) > 0x10 { 0x10000 } else if e(0xe) > 0xc { 0x20000 } else { 0x8000 });
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        _ => {
            if sheet == 0 {
                a(0x20000);
            } else if e(0xe) > lim(0xf) && special {
                a(0x10000);
            } else if e(0xe) <= 5 && weight_of_side() != 0 && !(disposicao(n_final as isize) > read_f64_at(0x967890)) { // 0x967890 = 1
                // como na original: le a disposicao no indice do total de vagas (o ebx do laco), nao do lado
                g(0x8000);
                if e(0x13) < lim(4) { bx(0x1000) } else { cx() }
            } else {
                a(0x20000);
            }
        }
    }
    destroy_list(f + 0x80);
    *b(0x37) as i8
}

// simplification: aloca a lista e os vetores do rodizio e chama 24 rotinas ainda originais; sem comparacao
// propria, validada pela carreira (Rust x original), como as da 145a, 148a e 149a.

/// Match clock minute (+0x8ed0 / 11) capped at the end of the current period (+0x8ed4: 45, 90, 105, 120).
///
/// [pt] Minuto do relogio (+0x8ed0 / 11), no maximo o fim do periodo (+0x8ed4: 45, 90, 105, 120).
pub(crate) unsafe fn minute_in_period(m: usize) -> i8 {
    let min = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
    match *((m + 0x8ed4) as *const i16) {
        0x1ef => min.min(0x2d),
        0x3de => min.min(0x5a),
        0x483 => min.min(0x69),
        0x528 => min.min(0x78),
        _ => min,
    }
}

/// Port of 0x6e1090 with game-affecting calls injected: the staff makes up to +0x17 substitutions, narrates them, picks
/// the captain and rebuilds the team.
///
/// [pt] { 0x6e1090 } (ret 4, match_man.cpp) com as chamadas que mexem no jogo injetadas (0x6e2000 escala,
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
pub(crate) unsafe fn substitute_with(c: usize, goalkeeper: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let table_c: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_ALT_SLOT_POSITION);
    let table_b: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_SECONDARY_POSITION);
    let marks: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let in_competition: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_CLUB_STILL_IN_CONTENTION);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let t9b23: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let entrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_PLACE_PLAYER_AT_ENTRY_CELL);
    let remove: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REMOVE_PLAYER_FROM_PITCH);
    let recompor: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_CLEAR_GOALKEEPER);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let u = |o: usize| *((c + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let trocas = byte(c, 0x17);
    if trocas == 0 {
        return;
    }
    let mut list = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        let e = byte(j, 0x19);
        if e >= 0 || (e == -2 && byte(j, 0x1e) == 0) {
            list[n] = j;
            n += 1;
        }
    }
    qsort(list.as_mut_ptr() as usize, n as u32, 4, if n < 11 { 0x6e0df0 } else { 0x6e0fe0 });
    let slots = (trocas as i32).min(n as i32).max(0) as usize;
    let mut table = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut pos = [0i8; 20];
    let mut mark = [0u64; 20];
    let mut bpos = [0u8; 20];
    for i in 0..slots {
        let p = list[i];
        if byte(p, 0x19) == -2 {
            table[i] = table_c(p);
            tab_b[i] = 0;
        } else {
            table[i] = tabela(p);
            tab_b[i] = table_b(p);
        }
        pos[i] = if i == 0 && goalkeeper != 0 {
            0
        } else if byte(p, 0x19) == -2 {
            byte(p, 0x1a)
        } else {
            byte(p, 0x19)
        };
        mark[i] = marks(u(0x2b), pos[i]);
        bpos[i] = byte_pos(u(0x2b), pos[i]);
    }
    let comp = u(0x37);
    // os reservas entram logo depois das vagas (o resto da lista ordenada fica de fora)
    let mut n1 = slots;
    let limit = (*((comp + 0x4d) as *const u8) & 0xf) as i32 + 0xb;
    if limit > 0xb {
        let mut p = u(0x23) + 0x132a;
        let mut k = 0xb;
        while k < limit {
            if p != 0 && byte(p, 0x19) == -1 {
                list[n1] = p;
                n1 += 1;
            }
            k += 1;
            p += 0x1be;
        }
    }
    let mut fichas = [0usize; 20];
    for i in 0..n1 {
        fichas[i] = *((list[i] + 0x69) as *const usize);
    }
    // o modo da escala
    let clube = u(0x2f);
    let strength = |o: usize| w(*((c + o) as *const usize), 0x80) as i32;
    let mut mode: u32 = if national_team(clube) != 0 { 0x1ffffd2 + 0x8a0cb } else { 0x8a0cb };
    if byte(c, 0x19) != 0 {
        mode = (mode | 0x20100) ^ 0x80000;
    } else {
        let mut done = false;
        if matches!(byte(comp, 0x3a), 1 | 9) {
            let r = sortear(0x7d0).wrapping_add(sortear(-4000));
            if strength(0x2f) - strength(0x33) > r {
                mode |= 0x100;
                done = true;
            }
        }
        if !done {
            let r = sortear(0xbb8) + 0xbb8;
            let checa_selecao = if strength(0x2f) - strength(0x33) > r {
                mode |= 0x100;
                true
            } else if byte(c, 0x1a) != 3 {
                false
            } else {
                let id = *(comp as *const i32);
                id <= -1 || in_competition(*((*(G_COMP_OBJECT_TABLE as *const usize) + id as usize * 4) as *const usize), clube) != 0
            };
            if checa_selecao {
                if national_team(clube) == 0 {
                    mode |= 0x10000;
                }
            } else {
                mode = (mode | 0x100) ^ 0x80000;
            }
        }
    }
    if byte(c, 0x16) != 0 {
        mode |= 0x800000;
    }
    if human(clube, 1, 1, 0, 0) != 0 && byte(c, 0x19) == 0 {
        let p = *((clube + club::MANAGER) as *const usize);
        if p != 0 && t9a01(*(G_HUMAN_MANAGER as *const usize), p) != 0 && t9b23(*(G_HUMAN_MANAGER as *const usize), 3, p) != 0 {
            mode |= 0x400000;
        }
    }
    let mut place = [0usize; 20];
    let extra = *((u(0x2b) + 0x584) as *const u32);
    let args: [usize; 15] = [
        fichas.as_ptr() as usize, n1, table.as_ptr() as usize, slots, 0, place.as_mut_ptr() as usize, mode as usize, clube, c, u(0x1b), 0,
        mark.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    request_buffers(
        &[(args[0], 4 * n1), (args[2], 2 * slots), (args[11], 8 * slots), (args[12], slots), (args[14], 2 * slots)],
        &[(args[5], 4 * slots)],
    );
    call(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS, 0, [args.as_ptr() as u32, n1 as u32, mode, clube as u32, c as u32]);
    let m = u(0x12);
    let mut entraram = [0usize; 20];
    let mut sairam = [0usize; 20];
    let (mut ne, mut ns) = (0usize, 0usize);
    for i in 0..n1 {
        let mut j = u(0x23);
        let mut found = false;
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == fichas[i] {
                found = true;
                break;
            }
            j += 0x1be;
        }
        if !found || j == 0 {
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            continue;
        }
        let slot = (0..slots).find(|&k| place[k] == *((j + 0x69) as *const usize));
        match slot {
            Some(k) => {
                if byte(j, 0x19) == -1 {
                    *((c + 0x17) as *mut i8) = byte(c, 0x17).wrapping_sub(1);
                    entrar(m, j);
                    *((j + 0x1f) as *mut i8) = minute_in_period(m);
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
                remove(m, j, -1);
                *((j + 0x19) as *mut i8) = -4;
                *((j + 0x1e) as *mut u8) = 1;
                *((j + 0x20) as *mut i8) = minute_in_period(m);
                sairam[ns] = j;
                ns += 1;
            }
        }
    }
    recompor(m, byte(c, 0x18));
    manter(m, byte(c, 0x18));
    let commentator = *((m + 0x8f7a) as *const usize);
    let side_c = byte(c, 0x18) as u8 as u32;
    let code = match ne {
        0 => return,
        1 => 0x1f5a,
        2 => 0x1f5c,
        _ => 0x1f59,
    };
    call(FN_MATCH_EVENTS_ADD_EVENT, commentator, [code, m as u32, u32::MAX, side_c, u32::MAX]);
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
            call(FN_MATCH_EVENTS_ADD_EVENT, commentator, [0x211d, m as u32, u32::MAX, (byte(p, 0x27) == 0) as u32, u32::MAX]);
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
            call(FN_MATCH_EVENTS_ADD_EVENT, commentator, [0x211e, m as u32, u32::MAX, byte(p, 0x27) as u8 as u32, u32::MAX]);
        }
        let cod = if byte(sai, 0x22) > -1 { 0x20d7 } else { 0x1f5b };
        let a = [cod, m as u32, byte(entra, 0x2e) as u8 as u32, byte(entra, 0x27) as u8 as u32, byte(sai, 0x2e) as u8 as u32];
        NEXT_SIXTH_ARG.store(byte(sai, 0x27) as u8 as u32, std::sync::atomic::Ordering::Relaxed);
        call(FN_MATCH_EVENTS_ADD_EVENT, commentator, a);
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
    call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, *(G_TACTICS as *const usize), [clube as u32, melhor as u8 as u32, byte(c, 0x19) as i32 as u32, 0, 0]);
    call(FN_MATCH_MAN_REASSIGN_POSITIONS, c, [0, 0, 0, 0, 0]);
}

/// O 6o argumento da proxima narracao (o lado de quem sai em 0x20d7/0x1f5b; -1 nas outras).
pub(crate) static NEXT_SIXTH_ARG: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);

/// Makes the AI substitutions for a side (choosing who comes on and off, reassigning positions, commentary, new
/// captain); thin wrapper over substitute_with with the real calls.
///
/// [pt] { 0x6e1090 } (ret 4, match_man.cpp): ver `substitute_with`.
pub(crate) unsafe extern "thiscall" fn substitute(c: usize, goalkeeper: u32) {
    substitute_with(c, goalkeeper as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e2000 => {
                let f: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
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
                let sexto = NEXT_SIXTH_ARG.swap(u32::MAX, std::sync::atomic::Ordering::Relaxed);
                add_match_event(this, [a[0], a[1], a[2], a[3], a[4], sexto, 0]);
            }
        }
        0
    })
}

/// Argumentos da 0x6e2000 (escalar: distribui as pessoas pelas posicoes, recursiva, aloca), cdecl.
pub(crate) type AssignPositionsFn = unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u32;

/// Port of 0x6e1aa0 with calls injected: rebuilds the team (formation check, selection, new positions and ratings for
/// each player).
///
/// [pt] { 0x6e1aa0 } (match_man.cpp) com as chamadas que mexem no jogo injetadas (0x6e0230 julga a
/// escalacao, 0x6e2000 escala, 0x6d6f70 narra): a comissao (`c`: +0x12 motor, +0x23 primeiro dos 20
/// jogadores, +0x2b tatica, +0x2f clube) refaz o time. Com exatamente 11 em campo e a escalacao
/// reprovada (0x6e0230 com 0x8c1, 2) narra 0x2007. Depois junta os de linha em campo (fora a tabela
/// 1): posicao +0x19, tabelas 0x717410/0x717450, marcas (0x5b5270) e byte (0x5b57c0) da posicao e a
/// ficha +0x69; escala (0x6e2000 com 0x8800, ou 0x408800 com tecnico humano a ser consultado:
/// 0x600a50, +0x19 da comissao, +0xcf do clube, 0x5fd210, 0x5fea50 com 3) e devolve a cada ficha
/// escalada a posicao do lugar que ela ganhou, refazendo o desempenho (0x6eecb0).
pub(crate) unsafe fn rebuild_team_with(c: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let table_b: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_SECONDARY_POSITION);
    let marks: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
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
        request_buffers(&[(fichas.as_ptr() as usize, 44)], &[]);
        if call(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC, c, [fichas.as_ptr() as u32, 0xb, 0x8c1, 2, 0]) as u8 != 0 {
            let m = u(0x12);
            call(FN_MATCH_EVENTS_ADD_EVENT, *((m + 0x8f7a) as *const usize), [0x2007, m as u32, u32::MAX, byte(c, 0x18) as u8 as u32, u32::MAX]);
        }
    }
    let mut pos = [0i8; 20];
    let mut table = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut mark = [0u64; 20];
    let mut bpos = [0u8; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        if byte(j, 0x19) < 0 || tabela(j) == 1 {
            continue;
        }
        pos[n] = byte(j, 0x19);
        table[n] = tabela(j);
        tab_b[n] = table_b(j);
        mark[n] = marks(u(0x2b), byte(j, 0x19));
        bpos[n] = byte_pos(u(0x2b), byte(j, 0x19));
        fichas[n] = *((j + 0x69) as *const usize);
        n += 1;
    }
    let clube = u(0x2f);
    let mut mode = 0x8800u32;
    if human(clube, 1, 1, 0, 0) != 0 && byte(c, 0x19) == 0 {
        let p = *((clube + club::MANAGER) as *const usize);
        if p != 0 && t9a01(*(G_HUMAN_MANAGER as *const usize), p) != 0 && t9b23(*(G_HUMAN_MANAGER as *const usize), 3, p) != 0 {
            mode = 0x408800;
        }
    }
    let mut place = [0usize; 20];
    let extra = *((u(0x2b) + 0x584) as *const u32);
    let args: [usize; 15] = [
        fichas.as_ptr() as usize, n, table.as_ptr() as usize, n, 0, place.as_mut_ptr() as usize, mode as usize, clube, c, u(0x1b), 0,
        mark.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    request_buffers(
        &[(args[0], 4 * n), (args[2], 2 * n), (args[11], 8 * n), (args[12], n), (args[14], 2 * n)],
        &[(args[5], 4 * n)],
    );
    call(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS, 0, [args.as_ptr() as u32, n as u32, mode, clube as u32, c as u32]);
    for i in 0..n {
        let mut j = u(0x23);
        let mut found = false;
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == fichas[i] {
                found = true;
                break;
            }
            j += 0x1be;
        }
        if !found {
            j = 0;
        }
        let f = *((j + 0x69) as *const usize);
        if let Some(k) = (0..n).find(|&k| place[k] == f) {
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
/// (`call_recording_buffers`, `request_buffers`). The provisional 0x6e1aa0 port from batch 114 is gone
/// Original: `redo_team` (0x6e1aa0, match_man.cpp, thiscall).
///
/// [pt] { 0x6e1aa0 } (match_man.cpp): ver `rebuild_team_with`.
pub(crate) unsafe extern "thiscall" fn redo_team(c: usize) {
    rebuild_team_with(c, &mut |rotina, this, a| match rotina {
        0x6e0230 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_COMPUTER_TACTIC);
            f(this, a[0] as usize, a[1], a[2], a[3]) as u32
        }
        0x6e2000 => {
            let f: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
            let v = *(a[0] as usize as *const [usize; 15]);
            f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14])
        }
        _ => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}



/// Among the `n` people in `lista` with a player record (+0x61), returns the one best rated by the staff of club +0x2f
/// (captain score 0x6e1cf0); with the object at +0x1f, a rating may drop to (1.1 - 0.01 x (object address mod 20)) x
/// rating when its attribute is below the person's id mod 20, as in the original. Ties keep the first; 0 if none.
///
/// [pt] { 0x6e1940 } (ret 8, match_man.cpp): entre as `n` pessoas de `lista` (com ficha +0x61), a mais
/// bem avaliada pela comissao (0x6e1cf0 com o clube +0x2f); 0 com `n` <= 0 ou nenhuma. Com o objeto
/// +0x1f, se o valor dele (0x544410 com o byte +0x17) ficar abaixo do id da pessoa mod 20, a nota
/// cai para (1,1 - 0,01 x (endereco do objeto mod 20)) x nota, como no original. Empate fica com
/// a primeira.
pub(crate) unsafe extern "thiscall" fn best_in_list(this: usize, list: *const usize, n: i8) -> usize {
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let value: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let (mut melhor, mut nota) = (0usize, 0i16);
    for i in 0..n.max(0) as usize {
        let p = *list.add(i);
        if p == 0 || *((p + 0x61) as *const usize) == 0 {
            continue;
        }
        let mut v = avaliar(p, 0, *((this + 0x2f) as *const usize)) as i32;
        let x = *((this + 0x1f) as *const usize);
        if x != 0 && (value(x, byte(x, 0x17) as u8 as u32) as i32) < *(p as *const i32) % 20 {
            let m = (x as i32) % 20;
            v = match_pl_ftol((read_f64_at(0x9689c0) - m as f64 * read_f64_at(0x967898)) * v as i16 as f64); // 0x9689c0 = 1.1, 0x967898 = 0.01
        }
        if melhor == 0 || v as i16 > nota {
            melhor = p;
            nota = v as i16;
        }
    }
    melhor
}

/// the index (0..20) of the active player (+0x19 ≥ 0) best rated by the staff (0x6e1cf0 with club +0x2f) among the 20
/// in the block starting at +0x23; the first active one counts even with a negative rating, then only strictly higher;
/// −1 with none active
/// Original: `best_rated` (0x6e1a40, match_man.cpp, thiscall).
///
/// [pt] { 0x6e1a40 } (match_man.cpp): o indice (0..20) do jogador ativo (+0x19 >= 0) mais bem avaliado
/// pela comissao (0x6e1cf0 com o clube +0x2f) entre os 20 do bloco que comeca em +0x23 (de 0x1be em
/// 0x1be); o primeiro ativo entra mesmo com nota negativa, depois so nota estritamente maior. -1 sem
/// nenhum ativo.
pub(crate) unsafe extern "thiscall" fn best_rated(this: usize) -> i8 {
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

/// The rating (i16) the staff give `pessoa` (or the person of match player `jogador`) for `clube`; -10000 with no
/// person. Combines the contract fit modifier, the player's reputation versus the club's, the contract (squad status,
/// type), current ability (+0x2f) times an age factor, then age, experience and +0x5b multipliers.
///
/// [pt] { 0x6e1cf0 } (cdecl, match_man.cpp): a nota (i16) que a comissao da a `pessoa` (ou a pessoa do
/// jogador `jogador` +0x69) para o `clube`; -10000 sem pessoa. Soma o encaixe (0x53ce90) em
/// (encaixe^2 + 5) x 0,1 negativo, a diferenca da nota +0xd da ficha para a reputacao do clube
/// (x 0,002 + 1,5, entre -4 e 4), o contrato (0x4cf5f0: +0x44 / 75 + 0,5, ajustado pelo tipo em +0x4f
/// >> 4: -10, +2,5, +2, -3, -15, -20, -50) e a ficha +0x2f vezes o fator da idade (dias de vida pela
/// data de hoje 0xae2c90 e a de nascimento +0x3e, 0x54cdb0; nas selecoes, +0x22 x 75): acima de 1000
/// dias, 1 + (dias - 1000) x 0,000125. Depois pela idade +0x18 (x0,5 abaixo de 18, x0,75 abaixo de
/// 21, x0,9 abaixo de 24, x1,05 acima de 25), x0,9 se os dias ficarem abaixo de (50 - +0x59) x 10 e
/// x(0,01 x +0x5b + 0,9) com +0x5b abaixo de 10.
pub(crate) unsafe extern "C" fn rate_player(person: usize, player: usize, clube: usize) -> i16 {
    let encaixe: unsafe extern "C" fn(usize, usize) -> i8 = core::mem::transmute(FN_DATABASE_GET_CONTRACT_RATING_MODIFIER);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let d8 = |a: usize| read_f64_at(a);
    let p = if player != 0 { *((player + 0x69) as *const usize) } else { person };
    if p == 0 {
        return -10000;
    }
    let sheet = *((p + 0x61) as *const usize);
    let enc = encaixe(p, clube) as f64;
    let mut y = (*((sheet + 0xd) as *const i16) as i32 - *((clube + club::REPUTATION) as *const i16) as i32) as f64 * d8(0x969048) + d8(0x968940);
    if !(d8(0x96d220) <= y) {
        y = -4.0;
    } else if d8(0x968938) < y {
        y = 4.0;
    }
    let mut x = 0.0f64;
    let e = contrato(*(G_CONTRACT_MANAGER as *const usize), p, clube);
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
    let a = if national_team(clube) != 0 {
        *((p + 0x22) as *const u8) as f64 * d8(0x96d520)
    } else {
        days(G_CURRENT_DATE, *((p + 0x3e) as *const u32), *((p + 0x42) as *const u32)) as f64
    };
    let f = if a > d8(0x968908) { (a - d8(0x968908)) * d8(0x96a6b8) + d8(0x967890) } else { 1.0 };
    let base = byte(sheet, 0x2f) as f64 * f - (enc * enc + d8(0x9688a0)) * d8(0x967880);
    let mut v = match_pl_ftol(base + y + x + d8(0x968968)) as i16;
    let age = byte(p, 0x18);
    let k = if age < 0x12 {
        Some(d8(0x968968))
    } else if age < 0x15 {
        Some(d8(0x969040))
    } else if age < 0x18 {
        Some(d8(0x9689d8))
    } else if age > 0x19 {
        Some(d8(0x9689b8))
    } else {
        None
    };
    if let Some(k) = k {
        v = match_pl_ftol(v as f64 * k) as i16;
    }
    if a < ((0x32 - byte(p, 0x59) as i32) * 10) as f64 {
        v = match_pl_ftol(v as f64 * d8(0x9689d8)) as i16;
    }
    let morale = byte(p, 0x5b);
    if morale < 10 {
        v = match_pl_ftol((morale as f64 * d8(0x967898) + d8(0x9689d8)) * v as f64) as i16;
    }
    v
}

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
pub(crate) unsafe extern "C" fn apply_squad_rule(
    clube: usize, manager: usize, rule: usize, mode: u32, nt: u32, nb: u32, nr: u32, starters: usize, bench: usize, reserve: usize,
    out: usize, codigos: usize, curtos: usize, marks: usize, bytes: usize, side: usize,
) {
    let r = |k: usize| *((rule + k) as *const i8);
    if r(0) == 0 || r(5) >= r(4) {
        return;
    }
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let rating: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    let restrito = |p: usize| -> bool {
        let estrangeiro: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        let comunitario: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
        let nacao = *((clube + club::NATION) as *const u32);
        match r(0) {
            1 => p != 0 && estrangeiro(p, nacao, 1) != 0 && comunitario(p, nacao) == 0,
            2 => p != 0 && estrangeiro(p, nacao, 1) != 0,
            3 => p != 0 && r(1) < *((p + 0x18) as *const i8),
            _ => false,
        }
    };
    let (nt, nb) = (nt as u8 as i8, nb as u8 as i8);
    let mut nr = nr as u8 as i8;
    let res = |k: i8| (reserve as isize + 4 * k as isize) as *mut usize;
    let sai = |k: i32| (out as isize + 4 * k as isize) as *mut usize;
    let w = |base: usize, k: i32| *((base as isize + 2 * k as isize) as *const u16);
    // tira `alvo` (ou a posição `k`) da reserva, pondo o último no lugar
    let remove = |k: i8, nr: &mut i8| {
        if (k as i32) < *nr as i32 - 1 {
            *res(k) = *res(*nr - 1);
            *res(*nr - 1) = 0;
        } else {
            *res(k) = 0;
        }
        *nr -= 1;
    };
    let remove_target = |target: usize, nr: &mut i8| {
        let mut k = 0i8;
        if 0 < *nr {
            loop {
                if *res(k) == target {
                    remove(k, nr);
                    break;
                }
                k += 1;
                if k >= *nr {
                    break;
                }
            }
        }
    };
    let mark = if side != 0 { *((*((side + 0x2b) as *const usize) + 0x584) as *const u32) } else { 0 };
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserve == 0 {
                break;
            }
            if restrito(*res(k)) {
                let last = (k as i32) < nr as i32 - 1;
                remove(k, &mut nr);
                if last {
                    k -= 1;
                }
            }
            k += 1;
            if k >= nr {
                break;
            }
        }
    }
    let mut slot = [0i8; 52];
    let mut cod = [0u16; 50];
    let mut cod2 = [0u16; 50];
    let mut m8 = [0u32; 100];
    let mut b1 = [0u8; 52];
    let mut cand = [0usize; 50];
    let mut choice = [0usize; 50];
    let mut foul = r(5);
    if foul < r(2) {
        let mut trocas = r(2) - foul;
        foul = r(3);
        if trocas != 0 {
            let mut nv = 0i8;
            let mut k = 0i8;
            while k < nt {
                if restrito(*((starters + 4 * k as usize) as *const usize)) {
                    let i = nv as usize;
                    slot[i] = k;
                    cod[i] = w(codigos, k as i32);
                    cod2[i] = if curtos == 0 { 0 } else { w(curtos, k as i32) };
                    (m8[2 * i], m8[2 * i + 1]) = if marks == 0 { (0, 0) } else { (*((marks + 8 * k as usize) as *const u32), *((marks + 8 * k as usize + 4) as *const u32)) };
                    b1[i] = if bytes == 0 { 0 } else { *((bytes + k as usize) as *const u8) };
                    nv += 1;
                }
                k += 1;
            }
            let mut nc = 0i8;
            for k in 0..nb.max(0) as usize {
                let q = *((bench + 4 * k) as *const usize);
                if q != 0 && !restrito(q) {
                    cand[nc as usize] = q;
                    nc += 1;
                }
            }
            let mut k = 0i8;
            if 0 < nr {
                loop {
                    if reserve == 0 {
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
                pick(cand.as_ptr() as usize, nc as u8 as u32, cod.as_ptr() as usize, nv as u8 as u32, 0, choice.as_mut_ptr() as usize, mode, clube, side,
                    manager as u32, 0, m8.as_ptr() as usize, b1.as_ptr() as usize, mark, cod2.as_ptr() as usize);
                let mut best = 0i32;
                let mut saiu_do_banco = false;
                loop {
                    if nv == 0 || nv < 1 {
                        break;
                    }
                    let mut esc = -1i32;
                    for k in 0..nv as usize {
                        if slot[k] == -1 {
                            continue;
                        }
                        let current = *sai(slot[k] as i32);
                        if current == 0 {
                            continue;
                        }
                        let valer = |p: usize| rating(p, *((p + 0x61) as *const usize), 0, cod[k] as u32, mode, clube, side, manager as u32, u32::MAX,
                            m8[2 * k], m8[2 * k + 1], b1[k] as u32, mark, cod2[k] as u32, 0);
                        let a = valer(current);
                        let b = if choice[k] == 0 { -1000000 } else { valer(choice[k]) };
                        if esc == -1 || a.wrapping_sub(b) < best {
                            best = a.wrapping_sub(b);
                            esc = k as i32;
                        }
                    }
                    if esc < 0 {
                        break;
                    }
                    let e = esc as usize;
                    *sai(slot[e] as i32) = choice[e];
                    remove_target(choice[e], &mut nr);
                    for k in 0..nb.max(0) as usize {
                        let q = (bench + 4 * k) as *mut usize;
                        if choice[e] != 0 && *q == choice[e] {
                            saiu_do_banco = true;
                            *q = 0;
                        }
                    }
                    slot[e] = -1;
                    trocas -= 1;
                    if trocas == 0 {
                        break;
                    }
                }
                if saiu_do_banco {
                    let mut n = 0i8;
                    for k in 0..nb.max(0) as usize {
                        let q = *((bench + 4 * k) as *const usize);
                        if q != 0 {
                            choice[n as usize] = q;
                            n += 1;
                        }
                    }
                    for k in 0..nr.max(0) {
                        if *res(k) != 0 {
                            choice[n as usize] = *res(k);
                            n += 1;
                        }
                    }
                    if n != 0 {
                        pick(choice.as_ptr() as usize, n as u8 as u32, codigos + 2 * nt as isize as usize, nb as u8 as u32, 0, sai(nt as i32) as usize, mode,
                            clube, side, manager as u32, 0, 0, 0, 0, curtos.wrapping_add(2 * nt as isize as usize));
                    }
                }
            }
        }
    } else {
        foul = r(4) - foul;
    }
    if foul == 0 {
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
            slot[i] = k;
            nv += 1;
        }
        k += 1;
    }
    let mut nc = 0i8;
    let mut k = 0i8;
    if 0 < nr {
        loop {
            if reserve == 0 {
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
            pick(cand.as_ptr() as usize, nc as u8 as u32, cod.as_ptr() as usize, nv as u8 as u32, 0, choice.as_mut_ptr() as usize, mode, clube, side,
                manager as u32, 0, 0, 0, 0, cod2.as_ptr() as usize);
            let mut best = 0i32;
            loop {
                if nv == 0 || nv < 1 {
                    return;
                }
                let mut esc = -1i32;
                for k in 0..nv as usize {
                    if slot[k] == -1 {
                        continue;
                    }
                    let current = *sai(slot[k] as i32 + nt as i32);
                    if current == 0 {
                        continue;
                    }
                    let valer = |p: usize| rating(p, *((p + 0x61) as *const usize), 0, cod[k] as u32, mode, clube, side, manager as u32, u32::MAX, 0, 0, 0, 0, cod2[k] as u32, 0);
                    let a = valer(current);
                    let b = if choice[k] == 0 { -1000000 } else { valer(choice[k]) };
                    if esc == -1 || a.wrapping_sub(b) < best {
                        best = a.wrapping_sub(b);
                        esc = k as i32;
                    }
                }
                if esc < 0 {
                    return;
                }
                let e = esc as usize;
                *sai(slot[e] as i32 + nt as i32) = choice[e];
                remove_target(choice[e], &mut nr);
                slot[e] = -1;
                foul -= 1;
                if foul == 0 {
                    break;
                }
            }
        }
    }
}

// ---- as notas do jogador do match_man.cpp (0x6e6340 fora da partida, 0x6e2f20 para uma vaga): partes comuns ----

/// Date `a` (day +0 signed, year +2 unsigned) is before date `b`.
///
/// [pt] A data (dia +0, ano +2) de `a` vem antes da de `b`? (comparação do ano sem sinal, do dia com sinal)
pub(crate) unsafe fn match_man_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (*((a + 2) as *const u16), *((b + 2) as *const u16));
    aa < ab || (aa == ab && *(a as *const i16) < *(b as *const i16))
}

/// Days from the person's date (+0x3e/+0x40) to today, counting 365 per year.
///
/// [pt] Os dias desde a data de `p` (+0x3e) até hoje ([0xae2c90]), contando 365 por ano.
pub(crate) unsafe fn match_man_days_since_date(p: usize) -> i32 {
    let years = *(G_CURRENT_DATE_YEAR as *const u16) as i32 - *((p + 0x40) as *const u16) as i32;
    years * 365 - *((p + 0x3e) as *const i16) as i32 + *(G_CURRENT_DATE as *const i16) as i32
}

/// Observer's date (+0x3e) is before the player's, and the player's date is more than a year ago.
///
/// [pt] O observador `o` (+0x3e) chegou antes da data do jogador `p` (+0x3e) e esta data é de mais de um
/// ano atrás (0x54c8e0 com 365)?
pub(crate) unsafe fn scouted_over_a_year_ago(o: usize, p: usize) -> bool {
    if !match_man_date_is_before(o + 0x3e, p + 0x3e) {
        return false;
    }
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> *const [u8; 8] = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut d = [0u8; 8];
    let r = datar(G_CURRENT_DATE, &mut d, 0x16d);
    match_man_date_is_before(r as usize, p + 0x3e)
}

/// Signed byte at p + k as i32.
///
/// [pt] Byte com sinal em `p + k`.
pub(crate) unsafe fn signed_byte_at(p: usize, k: usize) -> i32 {
    *((p + k) as *const i8) as i32
}

/// PLAYERSTATS get_stat (0x7c1e30 on G_PLAYER_STATS) for player data `f`.
///
/// [pt] A 0x7c1e30 das notas ([0xb74350]) da ficha `f`.
pub(crate) unsafe fn player_stat(f: usize, k: u32, c: u32, x: f32) -> f64 {
    let g: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    g(*(G_PLAYER_STATS as *const usize), f, k, c, x)
}

/// Rating of player data `f` for position mask `m` from its attributes via table `tab`, with the main-position special
/// cases.
///
/// [pt] A nota da posição (máscara `m`) pelos atributos da ficha `f` na tabela `tab` (0xa15d50 ou
/// 0xa15cfc). A posição principal tem os casos de `m` igual a 8 ou 0x10 e a exceção de lateral
/// (0x800/0x80) na faixa 0x10; a secundária da 0x6e2f20 não.
pub(crate) unsafe fn position_rating(f: usize, m: u32, masc: u32, table: usize, principal: bool) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let t = |k: i32| *((table as isize + 4 * k as isize) as *const i32);
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
            let middle = (fb(0x14) + fb(0x1a)) as f64 * read_f64_at(0x968968); // 0x968968 = 0.5
            t(match_pl_ftol(if a <= middle { middle } else { a }))
        } else {
            t(fb(0x14))
        }
    } else if m & 0x40 != 0 {
        t(fb(0x15))
    } else {
        100
    }
}

/// Adjustment of the position mask by the player's game attributes; 100 when no bit matches.
///
/// [pt] O ajuste da posição (máscara `m`) pelos atributos de jogo da ficha `f`; 100 sem nenhum.
pub(crate) unsafe fn position_attribute_adjust(f: usize, m: u32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
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

/// Bonus from the recent-form average (x0.2, weighted by days when the observer has known the player over a year) added
/// to the sum, scaled by `cont`.
///
/// [pt] O acréscimo da média das notas (0x7c1950 x 0,2; com o observador `quem` de mais de um ano no
/// clube do jogador, pesada pelos dias) à soma, por `cont`.
pub(crate) unsafe fn recent_form_bonus(p: usize, f: usize, clube: usize, who: usize, cont: i8) -> i32 {
    let average: unsafe extern "thiscall" fn(usize, usize) -> i8 = core::mem::transmute(FN_PLAYERSTATS_GET_RECENT_FORM);
    let mut x = (average(*(G_PLAYER_STATS as *const usize), f) as f64 * read_f64_at(0x968918)) as f32; // 0x968918 = 0.2
    if clube == *((p + 0x39) as *const usize) && who != 0 && scouted_over_a_year_ago(who, p) {
        let d = match_man_days_since_date(p) as f64;
        x = (((read_f64_at(0x969070) - d) * read_f64_at(0x968eb0) + d * x as f64) * read_f64_at(0x968e10)) as f32; // 0x969070 = 365, 0x968eb0 = 7.5, 0x968e10 = 0.002739726027
    }
    let x = x as f64;
    if x > read_f64_at(0x968998) { // 0x968998 = 6.8
        match_pl_ftol((x * read_f64_at(0x96d638) - read_f64_at(0x96d630)) * cont as f64) as i8 as i32 // 0x96d638 = 3.125, 0x96d630 = 21
    } else if x > read_f64_at(0x9689a0) { // 0x9689a0 = 0
        match_pl_ftol((x * read_f64_at(0x96d628) - read_f64_at(0x96d620)) * cont as f64) as i8 as i32 // 0x96d628 = 6.25, 0x96d620 = 42
    } else {
        0
    }
}

/// Form bonus (flag 2): stat 6 capped by the average, limited for veterans, weighted by observer days, plus stats 1 and
/// 6 at the club.
///
/// [pt] O acréscimo da forma (flag 2) à soma: a nota 0x7c1e30(6) de um teto pela média (ou por `l3c`),
/// limitado para os de mais de 33 anos quando `teto_veterano`, pesado pelos dias com o observador
/// `quem`, e as notas 0x7c1e30(1) e 0x7c1e30(6, 0x14, `ultimo`) no clube (ou mesma nação).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn form_bonus(p: usize, f: usize, clube: usize, who: usize, age: i8, cont: i8, sum: i32, l3c: i32, teto_veterano: bool, last: u32) -> i32 {
    let notas = *(G_PLAYER_STATS as *const usize);
    let ceiling = |v: i32| {
        let y = v as f64 * read_f32_at(0x96d61c) as f64 + read_f32_at(0x96d618) as f64; // 0x96d61c = 0.05000000075, 0x96d618 = 6.800000191
        (if y > read_f64_at(0x96d610) { read_f64_at(0x96d610) } else { y }) as f32 // 0x96d610 = 7.3
    };
    let razao = || {
        let (m10, m18) = (*((notas + 0x10) as *const f64), *((notas + 0x18) as *const f64));
        if m18 == read_f64_at(0x9689a0) { read_f64_at(0x968998) } else { m10 / m18 } // 0x9689a0 = 0, 0x968998 = 6.8
    };
    let mut x = f32::from_bits(0x40d9999a);
    if cont != 0 && sum / cont as i32 > 0x96 {
        x = ceiling(sum / cont as i32 - 0x96);
    } else if l3c as i16 > 0x96 {
        x = ceiling(l3c as i16 as i32 - 0x96);
    }
    let mut x = player_stat(f, 6, 0x12, x) as f32;
    if teto_veterano && age > 0x21 {
        let r = razao() + read_f64_at(0x968fd0); // 0x968fd0 = 0.125
        if (x as f64) > r {
            x = r as f32;
        }
    }
    if clube == *((p + 0x39) as *const usize) && who != 0 && scouted_over_a_year_ago(who, p) {
        let d = match_man_days_since_date(p) as f64;
        x = (((read_f64_at(0x969070) - d) * read_f64_at(0x968eb0) + x as f64 * d) * read_f64_at(0x968e10)) as f32; // 0x969070 = 365, 0x968eb0 = 7.5, 0x968e10 = 0.002739726027
        let fb_p = signed_byte_at(*((p + 0x61) as *const usize), 0xf);
        if age as i32 > fb_p / 10 + 0x22 && x as f64 > razao() {
            x = ((razao() + x as f64) * read_f32_at(0x968f18) as f64) as f32; // 0x968f18 = 0.5
        }
    }
    let xf = x as f64;
    let c = cont as f64;
    let mut sum = sum;
    if xf > read_f64_at(0x968f10) || (xf <= read_f64_at(0x968998) && xf > read_f64_at(0x9689a0)) { // 0x968f10 = 8, 0x968998 = 6.8, 0x9689a0 = 0
        sum += match_pl_ftol((xf * read_f32_at(0x96af88) as f64 - read_f32_at(0x96d608) as f64) * c) as i8 as i32; // 0x96af88 = 25, 0x96d608 = 170
    } else if xf > read_f64_at(0x968998) { // 0x968998 = 6.8
        sum += match_pl_ftol((xf * read_f64_at(0x96afc8) - read_f64_at(0x96d510)) * c) as i8 as i32; // 0x96afc8 = 12.5, 0x96d510 = 85
    }
    let owner = *((p + 0x39) as *const usize);
    if clube != 0 && (clube == owner || (owner != 0 && *((clube + club::NATION) as *const usize) == *((owner + 0x53) as *const usize))) {
        let xk = (xf * read_f64_at(0x9689d8)) as f32; // 0x9689d8 = 0.9
        let r = player_stat(f, 1, 0x12, xk);
        if r > read_f64_at(0x968998) { // 0x968998 = 6.8
            sum += match_pl_ftol((r * read_f64_at(0x96d628) - read_f64_at(0x96d600)) * c) as i8 as i32; // 0x96d628 = 6.25, 0x96d600 = 43
        } else if r < xk as f64 {
            sum += match_pl_ftol((r * read_f32_at(0x96d5f8) as f64 / xk as f64 - read_f32_at(0x96d5f8) as f64) * c) as i8 as i32; // 0x96d5f8 = 85
        }
        sum += match_pl_ftol((player_stat(f, 6, 0x14, f32::from_bits(last)) * read_f64_at(0x968e30) - read_f64_at(0x96d5f0)) * c); // 0x968e30 = 40, 0x96d5f0 = 60
    }
    sum
}

/// Penalty from the player's injury record (+0x28) subtracted from l4c.
///
/// [pt] O desconto do histórico do jogador (0x62ed70, +0x28).
pub(crate) unsafe fn injury_history_penalty(p: usize, l4c: i32) -> i32 {
    let history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let h = history(*(G_INJURY as *const usize), p);
    if h == 0 {
        return l4c;
    }
    let w = core::ptr::read_unaligned((h + 0x28) as *const i16) as i32;
    l4c + if w <= 10 { -2 * w } else { -20 - w }
}

/// Value from INJURY 0x62c3d0 by person id (exe map: get_fatigue / word_0c; the code comment calls it reputation)
/// [inference].
///
/// [pt] A reputação do jogador (0x62c3d0 pelo id).
pub(crate) unsafe fn injury_word_0c(p: usize) -> i16 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_INJURY_GET_FATIGUE);
    g(*(G_INJURY as *const usize), *(p as *const u32))
}

/// End of flag 0x4000: age penalty (flag 0x100), half-attribute sum bonus, +0x2f and the foreign player's
/// language/continent factor.
///
/// [pt] O fim da flag 0x4000 (comum às duas notas): a idade com +0x100, a soma das metades dos
/// atributos (sem goleiro), +0x2f e o fator do estrangeiro (0x7c6830 e a tabela por continente).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn age_and_foreigner_adjust(p: usize, f: usize, clube: usize, flags: u32, pos: u32, masc: u32, age: i8, cont: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    if flags & 0x100 != 0 {
        let cl = if masc & 1 != 0 { 0x1d } else { 0x19 };
        let y = read_f64_at(0x968e38) - cont as f64 * read_f64_at(0x968f80); // 0x968e38 = 3.5, 0x968f80 = 0.67
        if age as i32 > cl && y > read_f64_at(0x9689a0) { // 0x9689a0 = 0
            l4c -= match_pl_ftol((age as i32 - cl) as f64 * y);
        }
    }
    if pos & 1 == 0 {
        let s: i32 = (0x10..=0x19).map(|k| fb(k) / 2).sum();
        if s > 0x23 {
            l4c += (s - 20) * (s - 20) / 250;
        }
    }
    l4c += match_pl_ftol((fb(0x2f) - 10) as f64 * read_f64_at(0x96a670)); // 0x96a670 = 0.6666666667
    let nac = *((clube + club::NATION) as *const usize);
    let pais = *((p + 0x1a) as *const usize);
    if nac == 0 || *((nac + 0x77) as *const u8) == *((pais + 0x77) as *const u8) {
        return l4c;
    }
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_PLAYERSTATS_GET_LANGUAGE_FLUENCY);
    let mut c = g(*(G_PLAYER_STATS as *const usize), p, clube, (nac as u32 & !0xff) | *((nac + 0x77) as *const u8) as u32, 1) as u8 as i8;
    if flags & 0x100 != 0 {
        let k = (age as u8).wrapping_sub((*((p + 0x56) as *const i8) / 2) as u8).wrapping_add(5) as i8;
        let m = read_f64_at(0x96d5d0) - k as f64 * read_f64_at(0x96d0e8); // 0x96d5d0 = 26, 0x96d0e8 = 0.83
        c = match_pl_ftol(if c as f64 > m { c as f64 } else { m }) as i8;
    } else {
        let y = l4c as f64 * read_f64_at(0x96d1e0) - read_f64_at(0x9688a0); // 0x96d1e0 = 0.06, 0x9688a0 = 5
        if (c as f64) < y {
            c = match_pl_ftol(y) as i8;
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
        l4c = match_pl_ftol(l4c as f64 * read_f64_at(t[c as usize]));
    }
    l4c
}

/// Adjustment by contract (flag 0x2000000): days without club, wage vs owner and satisfaction, age, goalkeeper, injury
/// and appearances.
///
/// [pt] O ajuste pelo contrato (flag 0x2000000): sem clube (+0x39) há mais de 60 dias, o salário (+0x80)
/// do clube contra o do dono e a satisfação (+0x3d do contrato), a idade e o goleiro, a lesão
/// (0x5444a0) e a idade com os jogos (+0x22).
pub(crate) unsafe fn contract_adjust(p: usize, f: usize, clube: usize, contrato: usize, age: i8, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let mul = |l: i32, k: f64| match_pl_ftol(l as f64 * k);
    let owner = *((p + 0x39) as *const usize);
    if owner == 0 {
        let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        if days(G_CURRENT_DATE, *((p + 0x3e) as *const u32), *((p + 0x42) as *const u32)) > 0x3c {
            l4c = mul(l4c, read_f64_at(0x969040)); // 0x969040 = 0.75
        }
    } else {
        let lim = core::ptr::read_unaligned((owner + 0x80) as *const i16) as i32 - 0x4e2;
        let dx = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
        let ax = *((contrato + 0x3d) as *const i8) as i16;
        let cl = age;
        let k = if dx as i32 > lim {
            let ax = ax.wrapping_add(0x64);
            if ax >= 0x7d {
                None
            } else {
                let mut st = read_f32_at(0x96d248) as f64; // 0x96d248 = 0.75
                if dx != 0 {
                    st = lim as f64 / dx as f64 * ax as f64 * read_f32_at(0x96d5d8) as f64; // 0x96d5d8 = 0.00800000038
                    if st > read_f64_at(0x967890) { // 0x967890 = 1
                        st = read_f32_at(0x968e70) as f64; // 0x968e70 = 1
                    } else if st < read_f64_at(0x969040) { // 0x969040 = 0.75
                        st = read_f32_at(0x96d248) as f64; // 0x96d248 = 0.75
                    }
                }
                if cl > 0x22 {
                    st *= read_f64_at(0x969040); // 0x969040 = 0.75
                } else if cl > 0x1e {
                    st *= read_f64_at(0x969520); // 0x969520 = 0.85
                } else if cl > 0x1b {
                    st *= read_f64_at(0x9689d0); // 0x9689d0 = 0.95
                }
                if fb(0xf) == 0x14 {
                    st *= read_f64_at(if cl < 0x12 { 0x969040 } else if cl < 0x14 { 0x969520 } else { 0x9689d0 });
                }
                Some(st)
            }
        } else if ax >= 0x19 || cl <= 0x1e {
            None
        } else {
            let mut st = read_f32_at(if cl > 0x25 { 0x968f18 } else if cl > 0x22 { 0x96d248 } else { 0x96d364 }) as f64;
            if dx > 0x1482 && cl < 0x14 {
                if ax < -0x4b {
                    st = read_f32_at(0x968f18) as f64; // 0x968f18 = 0.5
                } else if ax < -0x19 {
                    st = read_f32_at(0x96d320) as f64; // 0x96d320 = 0.6499999762
                } else if ax < 0 {
                    st = read_f32_at(0x96d364) as f64; // 0x96d364 = 0.8500000238
                }
            }
            if fb(0xf) == 0x14 {
                st *= read_f64_at(if cl < 0x12 { 0x969040 } else if cl < 0x15 { 0x969520 } else { 0x9689d0 });
            }
            Some(st)
        };
        if let Some(st) = k {
            l4c = mul(l4c, st);
        }
        let lesao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
        if lesao(p, 0) != 0 {
            l4c = mul(l4c, read_f64_at(0x9689d0)); // 0x9689d0 = 0.95
        }
    }
    let (cl, al) = (age, *((p + 0x22) as *const u8));
    // Some(0): a nota vira 5
    let factor: Option<usize> = if cl > 0x22 {
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
    match factor {
        Some(0) => 5,
        Some(k) => mul(l4c, read_f64_at(k)),
        None => l4c,
    }
}

/// Observer bonus (flag 0x1000): scout knowledge, player/club and player/manager likes, and attributes by position
/// band.
///
/// [pt] O bônus do observador (flag 0x1000): o conhecimento `conhece` (+0x22 dos jovens, +0x14, +0x0e,
/// +0x1a, +0x1f, +0x13), o gosto do jogador pelo clube e por `quem`, e os atributos de cada faixa de
/// posição; `craque_meia` liga o bônus dos craques também na faixa 0x20 (só na 0x6e6340).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn scout_knowledge_bonus(p: usize, f: usize, clube: usize, conhece: usize, obs: usize, who: usize, pos: u32, masc: u32, age: i8, mut l4c: i32, craque_meia: bool) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let kb = |k: usize| signed_byte_at(conhece, k);
    if age <= 0x17 {
        l4c += match_pl_ftol((kb(0x22) - 10) as f64 * read_f64_at(0x96a670)); // 0x96a670 = 0.6666666667
    }
    let g = |a: usize| -> unsafe extern "C" fn(usize, usize) -> u32 { core::mem::transmute(a) };
    if g(0x547d30)(p, clube) != 0 {
        let m = l4c as f64 * read_f64_at(0x969040); // 0x969040 = 0.75
        let x = (l4c - 10) as f64;
        l4c = match_pl_ftol(if x < m { x } else { m });
    } else if g(0x547d60)(p, clube) != 0 {
        let m = l4c as f64 * read_f64_at(0x9689c0); // 0x9689c0 = 1.1
        let x = (l4c + 10) as f64;
        l4c = match_pl_ftol(if x > m { x } else { m });
    }
    if g(0x547840)(p, who) != 0 {
        l4c -= 50;
    } else if g(0x547810)(p, who) != 0 {
        let x = l4c as f64 * read_f64_at(0x968928); // 0x968928 = 1.25
        let m = (l4c + 20) as f64;
        l4c = match_pl_ftol(if x > m { x } else { m });
    }
    let f44 = fb(0x44);
    let k14 = kb(0x14);
    let mut ecx = (k14 * (f44 - 10) / 3).min(10) + ((signed_byte_at(p, 0x5b) - 10) * k14 / 3).min(10) + ((signed_byte_at(p, 0x59) - 10) * signed_byte_at(obs, 0x59) / 3).min(10);
    if pos & 1 == 0 {
        let away = pos & 0x80 == 0 && pos & 0x800 == 0;
        let e = kb(0xe);
        let k1a = kb(0x1a) * fb(0x32);
        let k1f = kb(0x1f) * (f44 - 10);
        // os craques: (+0x26 / 3) x +0x1b x +0x36 / 1500, com +0x26 > 30 e +0x36 > 15
        let craque = |ecx: i32| if fb(0x26) > 0x1e && fb(0x36) > 0xf { ecx + fb(0x26) / 3 * fb(0x1b) * fb(0x36) / 1500 } else { ecx };
        if pos & 6 != 0 {
            ecx += k1a / 40 + (fb(0x28) - 10) * e / 10 + fb(0x26) * e / 500 + fb(0x27) * e / 350 + fb(0x43) * e / 250 + k1f / 8 - 10;
            if away {
                ecx += fb(0x2e) - 15;
            }
        } else if pos & 8 != 0 {
            ecx += k1a / 40 + (fb(0x28) - 10) * e / 8 + fb(0x26) * e / 325 + fb(0x43) * e / 150 + k1f / 6 + fb(0x27) / 15 - 10;
            if away {
                ecx += fb(0x3e) / 15 + f44 - 15;
            }
        } else if pos & 0x10 != 0 {
            ecx += k1a / 60 + (fb(0x28) - 10) * e / 5 + fb(0x26) * e / 150 + fb(0x43) * e / 100 + k1f / 6 + fb(0x27) / 10 - 10;
            if away {
                ecx += fb(0x3e) / 15 + fb(0x37) / 15 + f44 - 15;
            } else {
                ecx += fb(0x23) / 15;
            }
        } else if pos & 0x20 != 0 {
            ecx += k1a / 150 + (fb(0x28) - 10) * e / 4 + k1f / 8 + fb(0x43) * e / 75 + fb(0x26) * e / 150 + fb(0x27) / 8 - 10;
            if away {
                ecx += fb(0x43) / 15 + fb(0x1a) - 15;
            } else {
                ecx += fb(0x23) / 15;
            }
            if craque_meia {
                ecx = craque(ecx);
            }
        } else if pos & 0x40 != 0 {
            ecx += k1a / 400 + (fb(0x28) - 10) * e / 3 + fb(0x43) * e / 200 + k1f / 10 + fb(0x26) * e / 175 + fb(0x27) / 5 - 10;
            if away {
                ecx += fb(0x33) / 15 + fb(0x36) + fb(0x40) + fb(0x27) / 15 - 0x14;
                ecx = craque(ecx);
            }
        }
    }
    if masc & 0x40 != 0 && pos & 0x40 != 0 {
        ecx += (fb(0x36) * fb(0x2e) - 200) * kb(0x13) / 500;
    }
    if ecx != 0 {
        l4c += match_pl_ftol((read_f64_at(0x967890) - l4c as f64 * read_f64_at(0x96d580)) * ecx as f64).clamp(-0x50, 0x50); // 0x967890 = 1, 0x96d580 = 0.00375
    }
    l4c
}

/// If the player has a work-permit record and needs a permit at the contract club: percentage +4/(+4 + +6) as a byte (0
/// if none); None otherwise or at 70+.
///
/// [pt] Na lista de transferências ([0xde1f54], 0x8c7260/0x8c7560 com o clube do contrato, que fica em
/// [0xae1080]): a porcentagem +4 / (+4 + +6) em byte (0 sem nenhum); None fora da lista ou de 70 em
/// diante.
pub(crate) unsafe fn work_permit_appearance_pct(p: usize) -> Option<i8> {
    let list = *(G_TRANSFER_MANAGER as *const usize);
    let search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_WORK_PERMIT);
    let x = search(list, p);
    if x == 0 {
        return None;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(*(G_CONTRACT_MANAGER as *const usize), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let owner = if c == 0 { 0 } else { *(G_CLUB_TABLE as *const usize) + *((c + 4) as *const usize) * club::SIZE };
    let can: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if can(list, p, owner, 1) == 0 {
        return None;
    }
    let (a, b) = (*((x + 4) as *const u16) as i32, *((x + 6) as *const u16) as i32);
    if a + b == 0 {
        return Some(0);
    }
    let bl = (a * 100 / (a + b)) as i8;
    if bl >= 0x46 { None } else { Some(bl) }
}

/// Contract extra (flag 0x10000): +0x44 by the +0x45 flags, the squad status (+0x4f >> 4) and the work-permit
/// percentage.
///
/// [pt] O extra do contrato (flag 0x10000): +0x44 pelas bandeiras +0x45, o tipo (+0x4f >> 4) e a lista
/// de transferências.
pub(crate) unsafe fn contract_extra(p: usize, contrato: usize, flags: u32, l28: i32) -> i32 {
    let e = *((contrato + 0x45) as *const u32);
    let s44 = *((contrato + 0x44) as *const i8) as f64;
    let mut extra = 0i32;
    if e & 0x2808 != 0 {
        let x = s44 * read_f64_at(0x96a670); // 0x96a670 = 0.6666666667
        extra = match_pl_ftol(if read_f64_at(0x96d578) < x { read_f64_at(0x96d578) } else { x }); // 0x96d578 = -25
    } else if e != 0 {
        let x = s44 * read_f64_at(0x968e00); // 0x968e00 = 0.4
        extra = match_pl_ftol(if read_f64_at(0x96d410) < x { read_f64_at(0x96d410) } else { x }); // 0x96d410 = -5
    }
    let t = (*((contrato + contract::SQUAD_STATUS) as *const u8) >> 4) as i8 as i32 - 1;
    let list = match t {
        0 | 1 => {
            extra += if t == 0 { 0x19 } else { 0xf };
            flags & 0x800000 == 0
        }
        2 => {
            if flags & 0x800000 != 0 {
                extra += 10;
                false
            } else {
                let d = (*((contrato + contract::UNKNOWN18_2) as *const i8) as i32 - *((contrato + 0x3f) as *const i8) as i32) * 5;
                extra += match_pl_ftol(d as f64 + (l28 as i16 as f64 * read_f64_at(0x968e18) - read_f64_at(0x96d570))); // 0x968e18 = 2.5, 0x96d570 = 225
                true
            }
        }
        6 => {
            extra -= 0x14;
            false
        }
        _ => false,
    };
    if list {
        if let Some(bl) = work_permit_appearance_pct(p) {
            extra += 0x32 - bl as i32 / 2;
        }
    }
    extra
}

/// Bond (flag 0x20000) by the contract's squad status: -25 to +25, or by the work-permit percentage.
///
/// [pt] O vínculo (flag 0x20000) pelo tipo do contrato `c`: -25 a +25, ou pela lista de transferências.
pub(crate) unsafe fn contract_bond(p: usize, c: usize, flags: u32) -> i32 {
    let t = (*((c + 0x4f) as *const u8) >> 4) as i8 as i32 - 1;
    match t {
        0..=2 => {
            if flags & 0x800000 != 0 {
                -25
            } else {
                match work_permit_appearance_pct(p) {
                    Some(bl) => bl as i32 / 2 - 0x4b,
                    None => -25,
                }
            }
        }
        3 | 4 | 7 => 25,
        _ => 0,
    }
}

/// Adjusts l4c when the 0x62c3d0 value is below -1000 or above 500.
///
/// [pt] O ajuste pela reputação (0x62c3d0) abaixo de -1000 ou acima de 500.
pub(crate) unsafe fn injury_word_0c_adjust(p: usize, l4c: i32) -> i32 {
    if injury_word_0c(p) < -1000 {
        l4c - match_pl_ftol(injury_word_0c(p) as f64 * read_f64_at(0x96a740) + read_f64_at(0x96afc8)) // 0x96a740 = 0.0125, 0x96afc8 = 12.5
    } else if injury_word_0c(p) > 0x1f4 {
        l4c + match_pl_ftol(read_f64_at(0x968890) - injury_word_0c(p) as f64 * read_f64_at(0x967888)) // 0x968890 = 25, 0x967888 = 0.05
    } else {
        l4c
    }
}

/// Appearances penalty (+0x22, flag 0x100000) or club adjustment (flag 4: owner +0x39 and their manager's +0x69).
///
/// [pt] Os jogos (+0x22, flag 0x100000) ou o clube (flag 4: o dono +0x39 e o +0x69 do treinador dele).
pub(crate) unsafe fn appearances_or_club_adjust(p: usize, f: usize, flags: u32, da_base: bool, l4c: i32) -> i32 {
    if flags & 0x100000 != 0 {
        return l4c - (*((p + 0x22) as *const u8) as i32 * 5).min(0x32);
    }
    if flags & 4 == 0 || !da_base {
        return l4c;
    }
    let owner = *((p + 0x39) as *const usize);
    if owner == 0 {
        return if core::ptr::read_unaligned((f + 9) as *const i16) < 0x1388 { l4c - 0x1e } else { l4c };
    }
    let s = *((owner + 0x57) as *const usize);
    if s == 0 {
        return l4c;
    }
    l4c + match_pl_ftol((core::ptr::read_unaligned((s + 0x69) as *const i16) as i32 - 0x10) as f64 * read_f64_at(0x9688a0)) // 0x9688a0 = 5
}

/// At the player's own club (+0x39): rating up by 5 or 5%, whichever is larger.
///
/// [pt] No próprio clube (+0x39): a nota sobe 5 ou 5%.
pub(crate) unsafe fn own_club_bonus(p: usize, clube: usize, l4c: i32) -> i32 {
    if *((p + 0x39) as *const usize) != clube {
        return l4c;
    }
    let x = (l4c + 5) as f64;
    let m = l4c as f64 * read_f64_at(0x9689b8); // 0x9689b8 = 1.05
    match_pl_ftol(if x > m { x } else { m })
}

/// Combines the position rating l40 with the adjustment l4c (only l40 when flags == 0x800).
///
/// [pt] A combinação da nota da posição `l40` com o ajuste `l4c` (só `l40` com as flags 0x800 sozinhas).
pub(crate) fn combine_rating(flags: u32, l40: i32, l4c: i32) -> i32 {
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
pub(crate) unsafe extern "C" fn off_match_score(p: usize, pos: u32, flags: u32, clube: usize, limit: i32, o: usize) -> i32 {
    if p == 0 {
        return -10000;
    }
    let f = *((p + 0x61) as *const usize);
    if f == 0 {
        return -10000;
    }
    let fb = |k: usize| signed_byte_at(f, k);
    let bd = *(G_INJURY as *const usize);
    let da_base = *(p as *const i32) < *(G_STAFF_COUNT as *const i32);
    let active = limit > -1;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    let without_value = flags & 0x800;
    if without_value == 0 || flags & 0x210000 != 0 {
        let value: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_INJURY_GET_CONDITION);
        l24 = value(bd, p, 0);
        l28 = l24 as i16 as i32 / 100;
    }
    let age = *((p + 0x18) as *const i8);
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = if *(G_CONTRACT_MANAGER as *const usize) != 0 { contract_with(*(G_CONTRACT_MANAGER as *const usize), p, clube) } else { 0 };
    let (conhece, obs) = if o != 0 {
        (*((o + 0x69) as *const usize), o)
    } else if clube != 0 && *((clube + club::MANAGER) as *const usize) != 0 {
        let t = *((clube + club::MANAGER) as *const usize);
        (*((t + 0x69) as *const usize), t)
    } else {
        (0, 0)
    };
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let mut l3c = relacao(p, obs, 0);
    let mask: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let masc = mask(f);
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    edi += if flags & 0x400 != 0 { 100 } else { position_rating(f, pos, masc, 0xa15d50, true) };
    let l40 = edi + if flags & 0x200 != 0 { 100 } else { position_attribute_adjust(f, pos) };
    let mut cont: i8 = 0;
    let mut sum: i32 = 0;
    let corte = |sum: i32, cont: i8| active && sum * 200 / (cont as i32) < limit;
    if flags & 0x40 != 0 {
        cont = 1;
        sum = core::ptr::read_unaligned((f + 0xb) as *const i16) as i32 / 50;
        if clube == *((p + 0x39) as *const usize) && o != 0 && scouted_over_a_year_ago(o, p) {
            sum = match_pl_ftol((read_f64_at(0x968fa8) - match_man_days_since_date(p) as f64 * read_f64_at(0x96a350)) * sum as f64); // 0x968fa8 = 1.375, 0x96a350 = 0.001
        }
        if active && sum * 200 < limit {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        cont += 1;
        let a = core::ptr::read_unaligned((f + 9) as *const i16) as f64;
        let b = core::ptr::read_unaligned((*((p + 0x1a) as *const usize) + 0x8e) as *const i16) as f64;
        sum += match_pl_ftol(a / ((read_f64_at(0x96d640) - b * read_f64_at(0x96a5c0)) * read_f64_at(0x968ff8))); // 0x96d640 = 1.8, 0x96a5c0 = 0.0001, 0x968ff8 = 50
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        cont += 1;
        sum += core::ptr::read_unaligned((f + 0xd) as *const i16) as i32 / 50;
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 4 != 0 && da_base {
        let v = match_pl_ftol(player_stat(f, 5, 0x12, f32::from_bits(0x40d00000)) * read_f64_at(0x968ea8)); // 0x968ea8 = 20
        if v != 0 {
            cont += 1;
            sum += v;
        }
    }
    let ability: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if cont != 0 {
            l3c = sum * 50 / cont as i32;
        }
        l34 = ability(obs, p, clube, 2, l3c as i16) as u16 as i32;
        cont += 1;
        sum += l34 as i16 as i32;
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if cont != 0 {
            l3c = potential_or_average_value(p, f, clube, sum, cont);
        }
        let mut r = ability(obs, p, clube, 1, l3c as i16);
        if l34 as i16 > -1 && r < l34 as i16 {
            r = l34 as i16;
        }
        cont += 1;
        sum += r as i32;
        if corte(sum, cont) {
            return -10000;
        }
    }
    // daqui em diante o original reaproveita o lugar do limite para contas: não corta mais
    if flags & 0x40000 != 0 && da_base {
        sum += recent_form_bonus(p, f, clube, o, cont);
    }
    if flags & 2 != 0 && da_base {
        sum = form_bonus(p, f, clube, o, age, cont, sum, l3c, false, 0x3fb66666);
    }
    let mut l4c: i32 = 0;
    if cont != 0 {
        l4c = sum / cont as i32;
        if l4c > 0x96 {
            l4c = match_pl_ftol((l4c - 0x96) as f64 * read_f64_at(0x968940) + read_f64_at(0x968f50)); // 0x968940 = 1.5, 0x968f50 = 150
        }
    }
    if flags & 8 != 0 && da_base {
        l4c += match_pl_ftol(fb(0x45) as f64 * read_f64_at(0x968940) - read_f64_at(0x968e08)); // 0x968940 = 1.5, 0x968e08 = 15
    }
    if flags & 0x4000 != 0 {
        let q = (0x1b58 - core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32) / 100;
        let k = (if q < 10 { 10 } else { q }) as i8 as i32;
        let lim = if fb(0xf) == 0x14 { 0x24 } else if fb(0x15) == 0x14 || fb(0x14) == 0x14 { 0x22 } else { 0x23 };
        if age as i32 > lim {
            let y = (read_f64_at(0x967890) - (age as i32 - lim) as f64 / k as f64) * l4c as f64; // 0x967890 = 1
            let m = (l4c - 5) as f64;
            l4c = match_pl_ftol(if y < m { y } else { m });
        }
        let history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
        let h = history(bd, p);
        if h != 0 {
            l4c -= core::ptr::read_unaligned((h + 0x2c) as *const i16) as i32;
        }
        l4c = age_and_foreigner_adjust(p, f, clube, flags, pos, masc, age, cont, l4c);
    }
    if contrato != 0 && clube != 0 && flags & 0x2000000 != 0 && da_base {
        l4c = contract_adjust(p, f, clube, contrato, age, l4c);
    }
    if without_value == 0 {
        let mut v = l24 as i16 as i32;
        if flags & 0x200000 == 0 && injury_word_0c(p) > 1000 {
            v += 500 - injury_word_0c(p) as i32;
        }
        if v < 5000 {
            return v + (l40 * 5 - 10000) * 20;
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = injury_history_penalty(p, l4c);
    }
    if flags & 0x200000 != 0 {
        l4c = fame_adjust(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && clube != 0 && l40 >= 0x64 {
        l4c = scout_knowledge_bonus(p, f, clube, conhece, obs, o, pos, masc, age, l4c, true);
    }
    l4c = outside_nation_penalty(p, clube, flags, l4c);
    if flags & 0x10000 != 0 {
        if contrato != 0 {
            l4c += contract_extra(p, contrato, flags, l28);
        }
    } else if flags & 0x20000 != 0 {
        let c = contract_with(*(G_CONTRACT_MANAGER as *const usize), p, clube);
        if c != 0 {
            l4c += contract_bond(p, c, flags);
        }
        if without_value == 0 {
            l4c = injury_word_0c_adjust(p, l4c);
        }
    }
    l4c = appearances_or_club_adjust(p, f, flags, da_base, l4c);
    if !da_base {
        l4c -= 0x1e;
    }
    l4c = own_club_bonus(p, clube, l4c);
    combine_rating(flags, l40, l4c)
}

/// Value for potential ability (flag 0x100): at the contract club, average x25 plus potential (+7) x25 (16-bit);
/// elsewhere average x50.
///
/// [pt] O valor para a habilidade potencial (flag 0x100): no clube do contrato (0x4cf5a0, que fica em
/// [0xae1080]) a média x 25 mais o potencial (+7) x 25 (em 16 bits); fora dele, a média x 50.
pub(crate) unsafe fn potential_or_average_value(p: usize, f: usize, clube: usize, sum: i32, cont: i8) -> i32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(*(G_CONTRACT_MANAGER as *const usize), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let owner = if c == 0 { 0 } else { *(G_CLUB_TABLE as *const usize) + *((c + 4) as *const usize) * club::SIZE };
    if owner == clube {
        let q = sum * 25 / cont as i32;
        let _ = f;
        let pot = core::ptr::read_unaligned((*((p + 0x61) as *const usize) + 7) as *const i16).wrapping_mul(0x19) as u16 as u32;
        q.wrapping_add(((q as u32 & 0xffff0000) | pot) as i32)
    } else {
        sum * 50 / cont as i32
    }
}

/// Fame (flag 0x200000): -200 for famous players (value/100 >= 99) with high +0x3c, low +0x2d and 0x62c3d0 value < 500;
/// else 100 - value/100.
///
/// [pt] A fama (flag 0x200000): -200 para os famosos (valor/100 >= 99) de +0x3c alto, +0x2d baixo e
/// reputação abaixo de 500; senão 100 - valor/100.
pub(crate) unsafe fn fame_adjust(f: usize, p: usize, l28: i32, l4c: i32) -> i32 {
    let v = l28 as i16 as i32;
    if v >= 0x63 && signed_byte_at(f, 0x3c) > 0xe && signed_byte_at(f, 0x2d) < 8 && injury_word_0c(p) < 0x1f4 {
        l4c - 200
    } else {
        l4c + 100 - v
    }
}

/// Flag 0x1000000: -50 if neither nationality (+0x1a, +0x1e) is the club's nation.
///
/// [pt] Fora da nação do clube (flag 0x1000000): -50 sem nenhuma das duas nacionalidades (+0x1a, +0x1e).
pub(crate) unsafe fn outside_nation_penalty(p: usize, clube: usize, flags: u32, l4c: i32) -> i32 {
    if flags & 0x1000000 == 0 || clube == 0 {
        return l4c;
    }
    let nac = *((clube + club::NATION) as *const usize);
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
pub(crate) unsafe extern "C" fn lineup_slot_score(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, clube: usize, side: usize, manager: usize, limit: i32, mark: u32, _marca_alta: u32,
    bytes: u32, tactic: u32, pos2: u32, ferias: usize,
) -> i32 {
    if p == 0 || f == 0 {
        return -10000;
    }
    let fb = |k: usize| signed_byte_at(f, k);
    let bd = *(G_INJURY as *const usize);
    let da_base = *(p as *const i32) < *(G_STAFF_COUNT as *const i32);
    let age = *((p + 0x18) as *const i8);
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = if *(G_CONTRACT_MANAGER as *const usize) != 0 { contract_with(*(G_CONTRACT_MANAGER as *const usize), p, clube) } else { 0 };
    let without_value = flags & 0x800;
    let mut l24: i32 = 10000;
    let mut l28: i32 = 100;
    if without_value == 0 || flags & 0x210000 != 0 {
        let value: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_INJURY_GET_CONDITION);
        l24 = value(bd, p, 0);
        l28 = l24 as i16 as i32 / 100;
    }
    let limit = if ferias != 0 { 0 } else { limit };
    let active = ferias != 0 || limit > -1;
    let (conhece, obs) = if side != 0 {
        (*((side + 0x1f) as *const usize), *((side + 0x1b) as *const usize))
    } else if manager != 0 {
        (*((manager + 0x69) as *const usize), manager)
    } else if clube != 0 && *((clube + club::MANAGER) as *const usize) != 0 {
        let t = *((clube + club::MANAGER) as *const usize);
        (*((t + 0x69) as *const usize), t)
    } else {
        (0, 0)
    };
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let mut l3c = relacao(p, obs, 0);
    let mask: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let masc = mask(f);
    let second = pos2 as u16 != 0 && pos2 as u16 != pos as u16;
    let mut edi: i32 = if masc as u16 == 0 { -100 } else { 0 };
    let mut ebp: i32 = 0;
    if flags & 0x400 != 0 {
        edi += 100;
    } else {
        edi += position_rating(f, pos, masc, 0xa15cfc, true);
        if second {
            ebp = position_rating(f, pos2, masc, 0xa15cfc, false);
        }
    }
    let l40 = if flags & 0x200 != 0 {
        edi + 100
    } else {
        edi += position_attribute_adjust(f, pos);
        if second { (ebp + position_attribute_adjust(f, pos2) + 2 * edi) / 3 } else { edi }
    };
    let mut cont: i8 = 0;
    let mut sum: i32 = 0;
    let corte = |sum: i32, cont: i8| active && sum * 200 / (cont as i32) < limit;
    if flags & 0x40 != 0 {
        cont = 1;
        sum = core::ptr::read_unaligned((f + 0xb) as *const i16) as i32 / 50;
        if clube == *((p + 0x39) as *const usize) && manager != 0 && scouted_over_a_year_ago(manager, p) {
            sum = match_pl_ftol((read_f64_at(0x968fa8) - match_man_days_since_date(p) as f64 * read_f64_at(0x96a350)) * sum as f64); // 0x968fa8 = 1.375, 0x96a350 = 0.001
        }
        if active && sum * 200 < limit {
            return -10000;
        }
    }
    if flags & 0x10 != 0 {
        cont += 1;
        let a = core::ptr::read_unaligned((f + 9) as *const i16) as f64;
        let b = core::ptr::read_unaligned((*((p + 0x1a) as *const usize) + 0x8e) as *const i16) as f64;
        sum += match_pl_ftol(a / ((read_f64_at(0x968950) - b * read_f64_at(0x96a5c0)) * read_f64_at(0x968ff8))); // 0x968950 = 2, 0x96a5c0 = 0.0001, 0x968ff8 = 50
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 0x20 != 0 {
        cont += 1;
        sum += core::ptr::read_unaligned((f + 0xd) as *const i16) as i32 / 50;
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 1 != 0 && reg != 0 && masc & 1 == 0 {
        cont += 1;
        sum += (core::ptr::read_unaligned((reg + 0x35) as *const i16) as i32 - match_fatigue_offset(reg, fb(0xf))) / 50;
    }
    if flags & 4 != 0 && da_base {
        let v = match_pl_ftol(player_stat(f, 5, 0x12, f32::from_bits(0x40d00000)) * read_f64_at(0x968ea8)); // 0x968ea8 = 20
        if v != 0 {
            cont += 1;
            sum += v;
        }
    }
    let ability: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let mut l34: i32 = -1;
    if flags & 0x80 != 0 {
        if cont != 0 {
            l3c = sum * 50 / cont as i32;
        }
        l34 = ability(obs, p, clube, 2, l3c as i16) as u16 as i32;
        cont += 1;
        sum += l34 as i16 as i32;
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 0x100 != 0 {
        if cont != 0 {
            l3c = potential_or_average_value(p, f, clube, sum, cont);
        }
        let mut r = ability(obs, p, clube, 1, l3c as i16);
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
        sum += r as i32;
        if corte(sum, cont) {
            return -10000;
        }
    }
    if flags & 0x40000 != 0 && da_base {
        sum += recent_form_bonus(p, f, clube, manager, cont);
    }
    if flags & 2 != 0 && da_base {
        sum = form_bonus(p, f, clube, manager, age, cont, sum, l3c, true, 0x3faccccd);
    }
    let mut l4c: i32 = 0;
    if cont != 0 {
        l4c = sum / cont as i32;
        if l4c > 0x96 {
            l4c = match_pl_ftol((l4c - 0x96) as f64 * read_f64_at(0x968940) + read_f64_at(0x968f50)); // 0x968940 = 1.5, 0x968f50 = 150
        }
    }
    if flags & 8 != 0 && da_base {
        l4c += match_pl_ftol((fb(0x45) - 10) as f64 * read_f64_at(0x9689b0)); // 0x9689b0 = 0.8
    }
    let marcada = mark & 0x8000 != 0 || (mark & 0x2000 != 0 && tactic & 0x100 != 0);
    if flags & 0x8000 != 0 {
        l4c = tactic_instruction_adjust(f, mark, bytes as u8, tactic, marcada, l4c);
    }
    if contrato != 0 && clube != 0 && flags & 0x2000000 != 0 && da_base {
        l4c = contract_adjust(p, f, clube, contrato, age, l4c);
    }
    if flags & 0x4000 != 0 {
        l4c = injury_history_penalty_2c(p, l4c);
        let id = *(p as *const i32);
        let mut edi = l4c;
        if *((*(G_STAFF_STATE_TABLE as *const usize) as isize + id as isize * 79 + 0xb) as *const u8) & 0x30 != 0 {
            if age > 0x28 {
                return -1000000;
            }
            edi -= if age > 0x22 { 0x4b } else { 0x19 };
            l4c = edi;
            if signed_byte_at(*((p + 0x61) as *const usize), 0x3c) < 0xa {
                edi -= 0x32;
                l4c = edi;
            }
        }
        if age > 0x2a {
            return -1000000;
        }
        let i = age as i32;
        if i > 0x22 && masc & 1 == 0 {
            l4c = edi + (0x22 - i) * 10;
        } else if i < 0x17 && masc & 1 != 0 {
            l4c = edi + (i - 0x17) * 10;
        } else if i > 0x20 && pos & 0x40 != 0 {
            l4c = edi + (0x20 - i) * 5;
        }
        l4c = age_and_foreigner_adjust(p, f, clube, flags, pos, masc, age, cont, l4c);
    }
    let mut value: i32 = 0;
    if without_value == 0 {
        let mut v: i32;
        if reg != 0 {
            if *((reg + 0x19) as *const i8) == -2 {
                return -1000000;
            }
            let x = core::ptr::read_unaligned((reg + 0x59) as *const f32) as f64;
            v = match_pl_ftol(x + x + core::ptr::read_unaligned((reg + 0x4d) as *const f32) as f64) as i16 as i32;
            if core::ptr::read_unaligned((*((reg + 0x19e) as *const usize) + 0x8ed4) as *const i16) == 0x1ef {
                v = (v + 0x5dc).min(0x2710);
            }
        } else {
            v = l24 as i16 as i32;
        }
        if flags & 0x200000 == 0 && injury_word_0c(p) > 1000 {
            v += 500 - injury_word_0c(p) as i32;
        }
        if v < 0x1676 {
            let r = v + (l40 * 5 - 0x4e2) * 20;
            return if v >= 0xdac { r } else { r - 0x7a120 };
        }
        if marcada {
            v = match_pl_ftol(v as f64 * read_f64_at(0x96a5c0) * v as f64); // 0x96a5c0 = 0.0001
        }
        l4c += if v < 9000 { v * 3 / 100 - 0x118 } else { (10000 - v) / -100 };
        l4c = injury_history_penalty(p, l4c);
        value = v;
    }
    if flags & 0x200000 != 0 {
        l4c = fame_adjust(f, p, l28, l4c);
    }
    if flags & 0x1000 != 0 && conhece != 0 && clube != 0 && l40 >= 0x64 {
        l4c = scout_knowledge_bonus(p, f, clube, conhece, obs, manager, pos, masc, age, l4c, false);
    }
    if flags & 0x2000 != 0 && side != 0 && *((side + 0x3b) as *const usize) == p {
        l4c += 100;
    }
    l4c = outside_nation_penalty(p, clube, flags, l4c);
    if flags & 0x80000 != 0 && side != 0 {
        let em = |o: usize| *((side + o) as *const usize) == p;
        l4c += if em(6) { 0x1e } else if em(0xa) { 0x14 } else if em(0xe) { 0xa } else { 0 };
    }
    // o cansaço do minuto da partida reduz o extra
    let by_minute = |v: i32| {
        if reg == 0 {
            v
        } else {
            let minute = core::ptr::read_unaligned((*((reg + 0x19e) as *const usize) + 0x8ed0) as *const i16) as f64;
            match_pl_ftol((read_f64_at(0x967890) - minute * read_f64_at(0x96d268)) * v as f64) // 0x967890 = 1, 0x96d268 = 0.0008333333333
        }
    };
    if flags & 0x10000 != 0 {
        if contrato != 0 {
            let mut extra = by_minute(contract_extra(p, contrato, flags, l28));
            if ferias != 0 {
                extra = match_pl_ftol(extra as f64 * read_f64_at(0x9689e0)); // 0x9689e0 = 0.25
            }
            l4c += extra;
        }
    } else if flags & 0x20000 != 0 {
        if contrato != 0 {
            l4c += by_minute(contract_bond(p, contrato, flags));
        }
        if without_value == 0 {
            l4c = injury_word_0c_adjust(p, l4c);
        }
    }
    l4c = appearances_or_club_adjust(p, f, flags, da_base, l4c);
    let mut base = l40;
    if !da_base && clube != 0 {
        let nac = *((clube + club::NATION) as *const usize);
        if nac != 0 && *((nac + 0x11c) as *const u8) != 0 {
            l4c -= 0x46;
            base = l40 - 0x78;
        }
    }
    l4c = own_club_bonus(p, clube, l4c);
    l4c = combine_rating(flags, base, l4c);
    if ferias != 0 && (pos & 1 == 0 || masc & 1 != 0) {
        l4c = holiday_selection_bonus(p, clube, ferias, pos, base, without_value, value, l4c);
    }
    l4c
}

/// Flag 0x4000 penalty from the injury record (+0x2c).
///
/// [pt] O desconto da flag 0x4000 pelo histórico (0x62ed70, +0x2c).
pub(crate) unsafe fn injury_history_penalty_2c(p: usize, l4c: i32) -> i32 {
    let history: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let h = history(*(G_INJURY as *const usize), p);
    if h == 0 { l4c } else { l4c - core::ptr::read_unaligned((h + 0x2c) as *const i16) as i32 }
}

/// Fatigue to subtract from stamina (+0x35) for a match player: -2000 for human-managed below index 11, off-pitch
/// remaining minutes x150, on-pitch minutes played x40.
///
/// [pt] O cansaço do jogador da partida `reg` a descontar do fôlego (+0x35): -2000 para o técnico humano
/// (+0x904d do lado, sem 0x5fd210, seleção ou 0x5fd480) com o jogador abaixo de 11 (+0); fora de
/// campo (+0x19 = -1) os minutos que faltam x 150 (até 3000; goleiro só +3500, os demais pelo menos
/// -2000); em campo, os minutos jogados (minuto - +0x1f) abaixo de 60 x 40.
pub(crate) unsafe fn match_fatigue_offset(reg: usize, goalkeeper: i32) -> i32 {
    let m = *((reg + 0x19e) as *const usize);
    let human = if core::ptr::read_unaligned((m + 0x8ed4) as *const i16) == 0x1ef {
        true
    } else {
        let side = *((reg + 0x27) as *const i8);
        let base = (m as isize + side as isize * 0xd0) as usize;
        let t = *((base + 0x904d) as *const usize);
        let n = *(G_STAFF_COUNT as *const i32);
        if t == 0 || *(t as *const i32) < n - 0x10 || *(t as *const i32) >= n || *((t + 0x5f) as *const u8) == 0 {
            false
        } else {
            let tecnicos = *(G_HUMAN_MANAGER as *const usize);
            let g9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            if g9a01(tecnicos, t) != 0 {
                false
            } else if *((base + 0x904b) as *const u8) != 0 {
                let c = *((*((m + 0x4792) as *const usize) + if side == 0 { 0x1c } else { 0x20 }) as *const usize);
                let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
                if national_team(c) != 0 {
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
    let extra = if human && *(reg as *const i8) < 0xb { -2000 } else { 0 };
    let state = *((reg + 0x19) as *const i8);
    let minute = core::ptr::read_unaligned((m + 0x8ed0) as *const i16) as i32 / 11;
    if state == -1 {
        let e = extra + ((60 - minute) * 150).min(3000);
        if goalkeeper > 1 { e + 0xdac } else { e.max(-2000) }
    } else if state >= 0 {
        let jogados = (minute as u8).wrapping_sub(*((reg + 0x1f) as *const u8)) as i8;
        if jogados < 0x3c { extra + (jogados as i32 - 60) * 40 } else { extra }
    } else {
        extra
    }
}

/// Adjustment by tactic instructions (flag 0x8000): instruction bits, marking and tactic vs the player's attributes.
///
/// [pt] As marcas da tática (flag 0x8000): `marca` (o u64 das instruções, parte baixa), `bytes` (o
/// byte das instruções) e `tatica` contra os atributos da ficha.
pub(crate) unsafe fn tactic_instruction_adjust(f: usize, lo: u32, b1: u8, tactic: u32, marcada: bool, mut l4c: i32) -> i32 {
    let fb = |k: usize| signed_byte_at(f, k);
    let x = |k: usize, c: usize| match_pl_ftol((fb(k) - 10) as f64 * read_f64_at(c));
    let t13 = tactic as u8;
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
        let x1 = match_pl_ftol((c.abs() * c) as f64 * read_f64_at(0x968fe8)); // 0x968fe8 = -0.4
        let x2 = match_pl_ftol((d.abs() * d) as f64 * read_f64_at(0x968e00)); // 0x968e00 = 0.4
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
        l4c += match_pl_ftol((fb(0x3b) - 20) as f64 * read_f64_at(0x96d5e0)); // 0x96d5e0 = 0.2857142857
    } else if lo & 0x100 != 0 {
        l4c += match_pl_ftol((fb(0x30) - 20) as f64 * read_f64_at(0x96d5e0)); // 0x96d5e0 = 0.2857142857
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

/// Holiday bonus: a player in the 16-pair holiday list gets 40000 + 200 x rating if in the 11 and position accepted,
/// else 200 x rating; weighted by player value.
///
/// [pt] O bônus das férias: o jogador na lista (16 pares id, posição) ganha 40000 + 200 x a nota da
/// posição (pelo menos 10000) se estiver entre os 11 e o técnico (0x5fea50) aceitar a posição; senão
/// 200 x a nota (nunca negativa), com a nota negativa pela metade; pesado pelo valor do jogador.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn holiday_selection_bonus(p: usize, clube: usize, ferias: usize, pos: u32, base: i32, without_value: u32, value: i32, mut l4c: i32) -> i32 {
    let id = *(p as *const i32);
    let Some(k) = (0..16usize).find(|&k| *((ferias + 8 * k) as *const i32) == id) else {
        return l4c;
    };
    let aceita = || {
        let g: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
        if g(*(G_HUMAN_MANAGER as *const usize), 2, *((clube + club::MANAGER) as *const usize)) == 0 {
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
    if without_value == 0 {
        let v = value as f64;
        let y = v * read_f64_at(0x968eb8); // 0x968eb8 = 1e-08
        let z = if value >= 0x251c {
            y * read_f64_at(0x96d568) // 0x96d568 = 100000000
        } else if value >= 0x222e {
            y * v * read_f64_at(0x968910) // 0x968910 = 10000
        } else {
            v * (y * v)
        };
        bonus = match_pl_ftol((z * read_f64_at(0x96a600) - read_f64_at(0x967890)) * bonus as f64); // 0x96a600 = 0.0002, 0x967890 = 1
    }
    if bonus > 0 {
        l4c += bonus;
    }
    l4c
}



/// sets up one side's tactics (manager, players, club, opponent, match, home side) and selects: at the human's club the
/// stored lines (0x5fbe20/0x5fbd00) and the automatic 0x6dbc50 mode 1 or the regulars (0x6dbb80); otherwise the
/// automatic mode 2 and the count of the home players; the substitutes by the bench and the club's tactical line.
/// Returns how many it selected
/// Original: `select_side` (0x6db500, match_man.cpp, thiscall, ret 0x24).
///
/// [pt] { 0x6db500 } (ret 0x24, match_man.cpp): monta a tatica `t` de um lado (+0x9032 ou +0x9102 do
/// motor) e escala o time: o tecnico e a ficha dele, os jogadores do bloco, o lado, o clube, o
/// adversario e a partida; o modo (+0x19 humano/+0x1a 3) pelo mando. No clube do humano, as linhas
/// taticas guardadas (0x5fbe20/0x5fbd00) e, sem o tecnico humano, a escalacao automatica no modo 1
/// (0x6dbc50, com o tecnico do clube 0x600e90) ou os titulares (0x6dbb80); fora isso, a automatica no
/// modo 2 e a conta dos da casa. Os reservas (+0x17) pelo banco da partida; le a linha tatica do
/// clube. Devolve quantos foram escalados; em `*vagas`, o tamanho do time.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn select_side(t: usize, m: usize, tecnico: usize, rows: usize, players: usize, side: u32, clube: usize, other: usize, jogo: usize, slots: *mut i8) -> i8 {
    let b = |o: usize| (t + o) as *mut u8;
    let d = |o: usize| (t + o) as *mut usize;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    if m == 0 {
        match_man_fatal_error(0x69);
    }
    if rows == 0 {
        match_man_fatal_error(0x6b);
    }
    if players == 0 {
        match_man_fatal_error(0x6d);
    }
    if clube == 0 {
        match_man_fatal_error(0x6f);
    }
    if other == 0 {
        match_man_fatal_error(0x71);
    }
    if tecnico != 0 {
        *d(0x1b) = tecnico;
        *d(0x1f) = u(tecnico, 0x69);
    } else {
        *d(0x1b) = 0;
        *d(0x1f) = 0;
    }
    *d(0x23) = players;
    *b(0x18) = side as u8;
    *d(0x27) = rows;
    *d(0x2f) = clube;
    *d(0x12) = m;
    *d(0x37) = jogo;
    *d(0x3b) = 0;
    *d(0xe) = 0;
    *d(0xa) = 0;
    *d(6) = 0;
    *d(0x2b) = m + 0x91e2 + (side as u8 as i8 as isize * 0x18e3) as usize;
    *d(0x33) = other;
    let home = (clube == u(jogo, 0x1c) && byte(jogo, 0x40) == 0) || (clube == u(jogo, 0x20) && byte(jogo, 0x41) == 0);
    if home {
        *b(0x19) = 1;
        *b(0x1a) = 0;
    } else {
        *b(0x19) = 0;
        *b(0x1a) = 3;
    }
    let sheet = *d(0x1f);
    if sheet != 0 {
        *((t) as *mut u16) = *((sheet + 0xa) as *const u16);
        *((t + 2) as *mut u16) = *((sheet + 8) as *const u16);
        *((t + 4) as *mut u16) = *((sheet + 0xc) as *const u16);
    }
    let human_usa: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let manager_of_club: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_STAND_IN_MANAGER);
    let automatica: unsafe extern "thiscall" fn(usize, usize, u32) -> i8 = core::mem::transmute(FN_MATCH_MAN_PICK_TEAM);
    let manager = *(G_HUMAN_MANAGER as *const usize);
    let n: i8;
    if *(m as *const u8) == 0 && human_usa(clube) != 0 && human_club(clube, 0, 1, 0, 0) != 0 {
        if is_second_national_team(clube) != 0 {
            *d(0x1b) = u(main_national_team(u(clube, 0x53)), 0xcf);
        }
        let prepare: unsafe extern "thiscall" fn(usize, i32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
        let guardadas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_ACTIVE_TACTIC);
        prepare(manager, byte(t, 0x19) as i32, clube, u(clube, 0xcf));
        let r = guardadas(manager, u(clube, 0xcf));
        if r == 0 || *d(0x2b) == 0 {
            match_man_fatal_error(0xbc);
        }
        core::ptr::copy_nonoverlapping(r as *const u8, *d(0x2b) as *mut u8, 0x18e3);
        let without_manager = |t: usize| {
            let x = manager_of_club(*((t + 0x2f) as *const usize));
            *((t + 0x1b) as *mut usize) = x;
            *((t + 0x1f) as *mut usize) = if x != 0 { *((x + 0x69) as *const usize) } else { 0 };
            automatica(t, other, 1)
        };
        n = if *b(0x19) == 0 {
            starters(t) as i8
        } else if is_national_team(clube) == 0 {
            if manager_9a05(manager, u(clube, 0xcf)) == 0 { without_manager(t) } else { starters(t) as i8 }
        } else if manager_9a06(manager, u(clube, 0xcf), core::ptr::null_mut()) == 0 {
            without_manager(t)
        } else {
            starters(t) as i8
        };
        *slots = ((*((jogo + 0x4d) as *const u8) & 0xf) + 0xb) as i8;
    } else {
        let x = *d(0x1b);
        if x != 0 {
            let base = *(G_STAFF_COUNT as *const i32);
            let id = *(x as *const i32);
            if id >= base - 0x10 && id < base && byte(x, 0x5f) != 0 {
                let y = manager_of_club(*d(0x2f));
                *d(0x1b) = y;
                *d(0x1f) = if y != 0 { u(y, 0x69) } else { 0 };
            }
        }
        n = automatica(t, other, 2);
        *slots = 0;
        let total = (*((jogo + 0x4d) as *const u8) & 0xf) + 0xb;
        let mut k = 0u8;
        while (k as i8) < total as i8 {
            let p = u(players + k as usize * 0x1be, 0x69);
            if p != 0 && *(p as *const i32) < *(G_STAFF_COUNT as *const i32) {
                *slots = (*slots).wrapping_add(1);
            }
            k += 1;
        }
    }
    let bench = (*((jogo + 0x4d) as *const u8) >> 4) & 0xf;
    let sobra = (n as u8).wrapping_sub(0xb) as i8;
    let r = if bench as i8 >= sobra { sobra } else { bench as i8 };
    *b(0x17) = if r <= 0 { 0 } else if (bench as i8) < sobra { bench } else { sobra as u8 };
    read_tactics_row(*(G_TACTICS as *const usize), *d(0x2f), (t + 0x3f) as *mut u8, byte(t, 0x19) as i32 as u32);
    n
}


/// a player's current or potential ability as an observer sees it (reputation, noise and the observer's knowledge)
/// Original: `perceived_ability` (0x6e86d0, match_man.cpp, cdecl).
///
/// [pt] { 0x6e86d0 } (cdecl, match_man.cpp): a habilidade do jogador `p` (2 = atual, senão potencial) como
/// o observador `o` (0 = o técnico do clube `c`, +0xcf) a vê: a reputação `rep` (negativa: pela
/// 0x5407e0) / 50 mistura com a habilidade verdadeira (ficha +0x61: +5 atual, +7 potencial) e um
/// ruído pseudoaleatório (id do jogador + [0xdd7120] + id do observador) que encolhe com o
/// conhecimento do observador (0x544410 da ficha +0x69, níveis +0x17/+0x18, até 20). Com a ficha
/// do observador, conhecimento alto e a semente favorável, a habilidade atual exata.
pub(crate) unsafe extern "C" fn perceived_ability(o: usize, p: usize, c: usize, kind: i8, rep: i16) -> i16 {
    let mut semente = (*(p as *const i32)).wrapping_add(*(G_GAME_RANDOM_SEED as *const i32));
    let mut rep = if rep > -1 {
        rep / 50
    } else {
        let relacao: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
        relacao(p, if o == 0 { *((c + 0xcf) as *const usize) } else { o }, 0) / 50
    };
    let sheet = *((p + 0x61) as *const usize);
    let age = if kind == 2 {
        0
    } else {
        (*((p + 0x18) as *const u8)).wrapping_sub(if *((sheet + 0xf) as *const u8) == 0x14 { 0x1a } else { 0x17 }) as i8
    };
    let mut conhece = 0usize;
    if o != 0 {
        conhece = *((o + 0x69) as *const usize);
        semente = semente.wrapping_add(*(o as *const i32));
        if conhece != 0 && kind == 2 && rep > 0x32 {
            let k = chance_to_know(conhece, *((conhece + 0x17) as *const i8)) as i8 as i32;
            if rep as i32 + (k - 10) * 10 > semente % 200 {
                return core::ptr::read_unaligned((sheet + 5) as *const i16);
            }
        }
    }
    let mut certeza: i8 = 1;
    if conhece != 0 {
        let f = ((semente % 101) as f64 * read_f64_at(0x967898)) as f32 as f64; // 0x967898 = 0.01
        let x = if semente % 10 == 0 {
            f * read_f32_at(0x96afb0) as f64 // 0x96afb0 = 20
        } else {
            let level = *((conhece + if kind == 2 { 0x17 } else { 0x18 }) as *const i8);
            (chance_to_know(conhece, level) as i8 as f64) * f
        };
        certeza = (match_pl_ftol(x + read_f64_at(0x968968)) as i8).min(0x14); // 0x968968 = 0.5
    }
    let c32 = certeza as i32;
    let ruido = (semente % ((0x15 - c32) * 2) + c32 - 0x15) as i16 as i32;
    let verdade = if kind == 2 {
        core::ptr::read_unaligned((sheet + 5) as *const i16)
    } else {
        if age > 0 {
            rep = rep.wrapping_sub((age as i16).wrapping_mul(age as i16));
        }
        core::ptr::read_unaligned((sheet + 7) as *const i16)
    } as i32;
    let weight = (c32 / 2 + 10) as i8 as i32;
    (((verdade + ruido) * weight + (20 - weight) * rep as i32) / 20) as i16
}















