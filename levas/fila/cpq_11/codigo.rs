// ---------- %%LEVA%%a leva: a copa: o sorteio dos times da rodada, os confrontos no estádio compartilhado e a escala pela rodada (Cup.cpp, comp_util.cpp) ----------

unsafe fn cpq11_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn read_ptr(a: usize) -> usize {
    cpq11_d(a) as usize
}

unsafe fn cpq11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq11_por_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// `base + i × tam` (o índice com sinal, como o movsx do original).
fn index_addr(base: usize, i: i32, tam: usize) -> usize {
    base.wrapping_add((i as isize).wrapping_mul(tam as isize) as usize)
}

/// O confronto `k` (0x35 bytes) da rodada `r` da copa: [[copa + 0xad] + 4r] + 0x35k.
unsafe fn tie_of_round(copa: usize, r: i32, k: i32) -> usize {
    index_addr(read_ptr(index_addr(read_ptr(copa + 0xad), r, 4)), k, 0x35)
}

/// A entrada da rodada `r` (0x68 bytes) em [copa + 0xa3]: +0x18 vagas de times, +0x1a confrontos.
unsafe fn round_entry(copa: usize, r: i32) -> usize {
    index_addr(read_ptr(copa + 0xa3), r, 0x68)
}

/// O clube do time `t` da copa (6 bytes em [copa + 0xa7]: +0 clube, +4 nível).
unsafe fn cup_team_entry(copa: usize, t: i16) -> usize {
    index_addr(read_ptr(copa + 0xa7), t as i32, 6)
}

/// Os confrontos de todas as rodadas da copa (rodadas em +0x3c), para os comparadores.
unsafe fn all_ties_regions(copa: usize) -> Vec<(usize, usize)> {
    let mut v = vec![(0xb67a34, 4)];
    for r in 0..(cpq11_w(copa + 0x3c) as i32).max(0) {
        let f = read_ptr(index_addr(read_ptr(copa + 0xad), r, 4));
        let n = cpq11_w(round_entry(copa, r) + 0x1a) as i32;
        if f != 0 && n > 0 {
            v.push((f, n as usize * 0x35));
        }
    }
    v
}

/// A caixa "Memory Allocation Error" ("v%s %s.%s %d (%s)" de Cup.cpp na linha `linha`) e o jogo para (como a original).
unsafe fn fatal_memory_error_box(linha: u32, texto: usize) -> ! {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b55ac, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
    *(0xb67a34 as *mut u32) = 0;
    fechar1();
    fechar2();
    sair(-1)
}

/// { 0x51e220 } (thiscall, ret 4, Cup.cpp): `Cup::randomize_round_teams(rodada)`: o sorteio dos times da rodada.
/// 1) Tira os times dos confrontos (+0x1a da rodada) para um vetor de 7 bytes por vaga (malloc de 7 × vagas +0x18;
///    sem memória, caixa da linha 0xe7f e o jogo para): o mandante (+0, com o vínculo +0xe/+0xa) e o visitante (+2,
///    com +0x10/+0xc), cada um com o nível do time (+4 do time); a vaga do confronto vira −2 (livre); sem time (−1),
///    a entrada fica −1 (com 0xff no nível).
/// 2) Embaralha: para cada posição, troca com a entrada sorteada entre todas (0x90cfc0(n)).
/// 3) Cada entrada com time vai a um confronto sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira
///    volta só confrontos com as duas vagas vazias (< 0); depois qualquer um com vaga livre (−2); dando a volta
///    inteira na segunda, caixa da linha 0xf4c e sai (a marca da segunda volta não volta a zero). Com as duas vagas
///    livres, sorteia o lado (0x90cfc0(2): ≠ 0 mandante). O time leva o vínculo (+0xe/+0xa ou +0x10/+0xc) e, com
///    vínculo, o confronto da rodada seguinte ganha a rodada (+0x12), o confronto sorteado (+6) e o lado (+8: 1
///    mandante, 0 visitante).
/// Libera o vetor (0x9452ca).
unsafe extern "thiscall" fn randomize_round_teams(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let vagas = cpq11_w(round_entry(copa, r) + 0x18) as i32;
    let vetor = alocar(vagas.wrapping_mul(7) as u32);
    if vetor == 0 {
        fatal_memory_error_box(0xe7f, 0x9b574c); // "CUP::randomize_round_teams() : Cannot allocate memory"
    }
    let entrada = |i: i16| index_addr(vetor, i as i32, 7);
    // 1) os times saem dos confrontos
    let mut n: i16 = 0;
    let mut k: i32 = 0;
    while (k as i16) < cpq11_w(round_entry(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = tie_of_round(copa, r, k as i16 as i32);
            let e = entrada(n);
            n = n.wrapping_add(1);
            let time = cpq11_w(t + lado);
            if time != -1 {
                cpq11_por_w(e, time as u16);
                cpq11_por_w(e + 2, cpq11_w(t + ligado) as u16);
                cpq11_por_w(e + 4, cpq11_w(t + jogo) as u16);
                *((e + 6) as *mut u8) = *((cup_team_entry(copa, time) + 4) as *const u8); // o nível do time
                cpq11_por_w(t + lado, 0xfffe); // a vaga fica livre
            } else {
                cpq11_por_w(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    // 2) o embaralhamento
    if n > 0 {
        let mut p = vetor;
        for _ in 0..n as i32 {
            let q = entrada(sorteio(n as i32) as i16);
            let (a, b, c) = (cpq11_d(q), cpq11_w(q + 4), *((q + 6) as *const u8));
            core::ptr::write_unaligned(q as *mut u32, cpq11_d(p));
            cpq11_por_w(q + 4, cpq11_w(p + 4) as u16);
            *((q + 6) as *mut u8) = *((p + 6) as *const u8);
            core::ptr::write_unaligned(p as *mut u32, a);
            cpq11_por_w(p + 4, b as u16);
            *((p + 6) as *mut u8) = c;
            p += 7;
        }
    }
    // 3) cada time vai a um confronto sorteado
    let mut segunda_volta = false;
    let mut i: i16 = 0;
    while i < n {
        let e = entrada(i);
        i = i.wrapping_add(1);
        if cpq11_w(e) == -1 {
            continue;
        }
        let mut s = sorteio(cpq11_w(round_entry(copa, r) + 0x1a) as i32) as i16;
        let inicio = s;
        let mandante = loop {
            let t = tie_of_round(copa, r, s as i32);
            let (casa, fora) = (cpq11_w(t), cpq11_w(t + 2));
            if segunda_volta || (casa < 0 && fora < 0) {
                let casa_livre = casa == -2;
                if fora == -2 {
                    if casa_livre && sorteio(2) != 0 {
                        crate::comparison::branch("cpq_11 sorteio mandante");
                        break true;
                    }
                    break false;
                }
                if casa_livre {
                    break true;
                }
            }
            s = s.wrapping_add(1);
            if s >= cpq11_w(round_entry(copa, r) + 0x1a) {
                s = 0;
            }
            if s == inicio {
                if segunda_volta {
                    show_error_at(0x9b55ac, 0xf4c);
                    liberar(vetor);
                    return;
                }
                segunda_volta = true;
            }
        };
        let t = tie_of_round(copa, r, s as i32);
        let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
        cpq11_por_w(t + lado, cpq11_w(e) as u16);
        cpq11_por_w(t + ligado, cpq11_w(e + 2) as u16);
        cpq11_por_w(t + jogo, cpq11_w(e + 4) as u16);
        let seguinte = cpq11_w(e + 2);
        if seguinte != -1 {
            let v = tie_of_round(copa, seguinte as i32, cpq11_w(e + 4) as i32);
            cpq11_por_w(v + 0x12, rodada as u16);
            cpq11_por_w(v + 6, s as u16);
            cpq11_por_w(v + 8, mandante as u16);
        }
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
unsafe extern "thiscall" fn comparar_randomize_round_teams(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(copa, rodada),
        || crate::rng::without_counting(|| randomize_round_teams(copa, rodada)));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("randomize_round_teams(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Os pares de palavras do confronto `t` que a 0x522610 troca: mandante/visitante (+0/+2), +0xa/+0xc e +0xe/+0x10.
unsafe fn swap_tie_sides(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xe, 0x10), (0xa, 0xc)] {
        let (x, y) = (cpq11_w(t + a), cpq11_w(t + b));
        cpq11_por_w(t + a, y as u16);
        cpq11_por_w(t + b, x as u16);
    }
}

/// { 0x522610 } (thiscall, ret 4, Cup.cpp): `Cup::separate_shared_stadium_ties(rodada)`: para cada confronto `i` da
/// rodada, procura o primeiro `j > i` cujo mandante (ou, senão, o visitante) divide o estádio (+0x69 do clube, não
/// nulo) com o mandante (ou o visitante) de `i`; achando, inverte o confronto `j` (mandante e visitante, +0xa/+0xc e
/// +0xe/+0x10) e acerta o lado (+8) dos confrontos ligados na rodada seguinte: o de +0xe/+0xa fica 1 (mandante), o de
/// +0x10/+0xc fica 0. Os dois jogos no mesmo estádio deixam de cair no mesmo lado.
unsafe extern "thiscall" fn separate_shared_stadium_ties(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let estadio = |t: i16| cpq11_d(read_ptr(cup_team_entry(copa, t)) as usize + 0x69);
    let mut n = cpq11_w(round_entry(copa, r) + 0x1a);
    let mut i: i32 = 0;
    while (i as i16) < n {
        let fi = tie_of_round(copa, r, i as i16 as i32);
        let casa_i = cpq11_w(fi);
        let mut j = i + 1;
        while (j as i16) < n {
            let fj = tie_of_round(copa, r, j as i16 as i32);
            let mut dividem = false;
            for lado in [0usize, 2] {
                let a = if lado == 0 { casa_i } else { cpq11_w(fi + 2) };
                let b = cpq11_w(fj + lado);
                if a != -1 && b != -1 {
                    crate::comparison::branch("cpq_11 estadio comparado");
                    let (sa, signed_byte_at) = (estadio(a), estadio(b));
                    if sa == signed_byte_at && sa != 0 {
                        dividem = true;
                        break;
                    }
                }
            }
            if dividem {
                swap_tie_sides(fj);
                let seguinte = cpq11_w(fj + 0xe);
                if seguinte != -1 {
                    cpq11_por_w(tie_of_round(copa, seguinte as i32, cpq11_w(fj + 0xa) as i32) + 8, 1);
                }
                let seguinte = cpq11_w(fj + 0x10);
                if seguinte != -1 {
                    cpq11_por_w(tie_of_round(copa, seguinte as i32, cpq11_w(fj + 0xc) as i32) + 8, 0);
                }
                break;
            }
            j += 1;
        }
        n = cpq11_w(round_entry(copa, r) + 0x1a);
        i += 1;
    }
}

/// Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
unsafe extern "thiscall" fn comparar_separate_shared_stadium_ties(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    let reg = all_ties_regions(copa);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(copa, rodada), || separate_shared_stadium_ties(copa, rodada));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("separate_shared_stadium_ties(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// A conta da 0x4bf850 sem efeitos: o eax da original e o byte devolvido. Com uma rodada só, (baixo + alto) / 2
/// (truncada para o zero) sobre a soma; senão o quociente (alto − baixo) × rodada / (rodadas − 1), sem somar o baixo.
fn interpolate_by_round(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> (u32, i8) {
    let (b, a) = (baixo as u8 as i8 as i32, alto as u8 as i8 as i32);
    let n = rodadas as u16 as i16 as i32;
    if n == 1 {
        let s = b.wrapping_add(a);
        let t = s.wrapping_sub(s >> 31); // cdq; sub eax, edx
        let v = (t >> 1) as i8;
        ((t as u32 & 0xffff_ff00) | v as u8 as u32, v)
    } else {
        let q = a.wrapping_sub(b).wrapping_mul(rodada as u16 as i16 as i32).wrapping_div(n - 1); // a média por rodada
        (q as u32, q as i8)
    }
}

/// { 0x4bf850 } (cdecl, comp_util.cpp): `comp_util_scale_by_round(baixo, alto, rodada, rodadas)` (bytes e palavras com
/// sinal; as palavras chegam em registradores com lixo acima, só a palavra é lida): com uma rodada só, a média
/// (baixo + alto) / 2; senão (alto − baixo) × rodada / (rodadas − 1). Fora de 0..=100: a original (caixa da linha
/// 0x256e, devolve 0 ou 100). Devolve o eax inteiro da original (quem chama empurra o registrador todo para
/// 0x86bdd0/0x86c000): o byte em al sobre a soma (rodada única) ou o quociente.
unsafe extern "C" fn comp_util_scale_by_round(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let (eax, v) = interpolate_by_round(baixo, alto, rodada, rodadas);
    if !(0..=100).contains(&v) {
        return original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(%%I2%%)(baixo, alto, rodada, rodadas);
    }
    if rodadas as u16 != 1 {
        crate::comparison::branch("cpq_11 escala pela rodada");
    }
    eax
}

/// Comparador: pura (pelo eax inteiro). Fora da faixa (a caixa de erro) só a original.
unsafe extern "C" fn comparar_comp_util_scale_by_round(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(%%I2%%);
    if !(0..=100).contains(&interpolate_by_round(baixo, alto, rodada, rodadas).1) {
        return o(baixo, alto, rodada, rodadas);
    }
    let r = o(baixo, alto, rodada, rodadas);
    let n = comp_util_scale_by_round(baixo, alto, rodada, rodadas);
    SCOREBOARD.record(r == n, || format!("comp_util_scale_by_round({}, {}, {}, {}) original={:x} rust={:x}",
        baixo as u8 as i8, alto as u8 as i8, rodada as u16 as i16, rodadas as u16 as i16, r, n));
    r
}