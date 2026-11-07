// ---------- 276a leva: o calendário das copas do Japão e dos EUA: as rodadas da Copa da Liga japonesa, da Copa do Imperador e da Copa dos EUA (jap_j_cup.cpp, jap_emp_cup.cpp, usa_open_cup.cpp) ----------


/// Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
struct Cqn11R {
    sorteio: (u32, u32, u32, u32),
    jogo: (u32, u32, u32, u32, u32, u32),
    codigo: u16,
    bits: (u16, u16, u16),
    categoria: u8,
    times: (u16, u16, u16, u16),
    confronto: (u8, u8, u8),
    premios: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
const fn cqn11r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> Cqn11R {
    Cqn11R { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
unsafe fn cqn11_rodada(t: usize, i: u32, r: &Cqn11R, ano: u32) {
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    sorteio(t, i, r.sorteio.0, r.sorteio.1, r.sorteio.2, r.sorteio.3, ano);
    jogo(t, i, r.jogo.0, r.jogo.1, r.jogo.2, r.jogo.3, r.jogo.4, ano, r.jogo.5);
    let e = t + i as usize * 0x68;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((e + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((e + o) as *mut u32, v);
    w(7, r.codigo);
    w(9, i as u16);
    w(0xb, r.bits.0);
    w(0xd, r.bits.1);
    w(0xf, r.bits.2);
    *((e + 0x17) as *mut u8) = r.categoria;
    w(0x18, r.times.0);
    w(0x1a, r.times.1);
    w(0x1c, r.times.2);
    w(0x1e, r.times.3);
    *((e + 0x20) as *mut u8) = r.confronto.0;
    *((e + 0x21) as *mut u8) = r.confronto.1;
    *((e + 0x22) as *mut u8) = r.confronto.2;
    d(0x5c, r.premios.0);
    d(0x60, r.premios.1);
    d(0x64, r.premios.2);
}

/// O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela.
unsafe fn cqn11_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32, tabela: &[Cqn11R], arquivo: usize, linha: u32, nome: &'static str) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = tabela.len() as i16;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(tabela.len() as u32 * 0x68);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in tabela.iter().enumerate() {
        cqn11_rodada(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava em cada rodada (o resto da memória alocada fica como veio): +0..+0x10 (as datas e os
/// campos até o desempate de ida e volta), +0x13..+0x22 (o mando até os dias entre os jogos) e +0x5c..+0x67 (os prêmios).
unsafe fn cqn11_gravados(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x68;
        for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
            v.extend_from_slice(core::slice::from_raw_parts((e + a) as *const u8, b - a));
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada).
unsafe fn cqn11_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if fase as u8 != 0xff {
        // ponytail: fora da fase −1 nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut s2 = if saida.is_null() { 0 } else { *saida };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if saida.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (saida.is_null() || *saida == s2);
    let tabela = if a != 0 && b != 0 { cqn11_gravados(a, *rodadas) == cqn11_gravados(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Liga japonesa (J League Cup): cinco rodadas de fevereiro a novembro, as quatro primeiras em ida e volta (7
/// dias entre os jogos, desempate de ida e volta 0x83).
const CQN11_COPA_DA_LIGA_JAPONESA: [Cqn11R; 5] = [
    cqn11r((0xa, 1, 0, 0xffff_ffff), (0xc, 3, 0, 2, 2, 0), 0xa, (0, 0, 0x83), 4, (0x18, 0xc, 0x18, 0), (0, 2, 7), (0, 0xffff_ffff, 0)), // cqn11: a 1ª rodada da Copa da Liga japonesa
    cqn11r((0x14, 3, 0, 3), (5, 6, 0, 2, 2, 0), 0x14, (0, 0, 0x83), 4, (0x10, 8, 4, 0x18), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((0xd, 6, 0, 3), (0x1e, 7, 0, 2, 2, 0), 0x78, (0, 0, 0x83), 4, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((7, 8, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x82, (0, 0, 0x83), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((0x13, 9, 0, 3), (4, 0xa, 0, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x665640 } (thiscall, ret 0x10, jap_j_cup.cpp; a virtual +0x3c da Copa da Liga japonesa): `JapJCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0x9e sem memória.
unsafe extern "thiscall" fn cqn11_copa_da_liga_japonesa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cqn11_tabela(this, fase, rodadas, extra, saida, &CQN11_COPA_DA_LIGA_JAPONESA, 0x9eaf14, 0x9e, "cqn_11 tabela da copa da liga japonesa")
}

unsafe extern "thiscall" fn comparar_cqn11_copa_da_liga_japonesa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1793);
    cqn11_comparar("cqn11_copa_da_liga_japonesa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqn11_copa_da_liga_japonesa_tabela(this, fase, r, e, s))
}

/// A Copa do Imperador: sete rodadas em jogo único de novembro ao começo de janeiro do ano seguinte, sorteio por sementes
/// com grupos separados (+0xd = 0x83).
const CQN11_COPA_DO_IMPERADOR: [Cqn11R; 7] = [
    cqn11r((1, 0xa, 0, 6), (0x1d, 0xa, 0, 6, 1, 0), 0xa, (0, 0x83, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cqn11: a 1ª rodada da Copa do Imperador
    cqn11r((0x1e, 0xa, 0, 0), (6, 0xb, 0, 6, 1, 0), 0x14, (0, 0x83, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((7, 0xb, 0, 0), (0xd, 0xb, 0, 6, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x10, 0x40), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xe, 0xb, 0, 0), (0x14, 0xb, 0, 6, 1, 0), 0x28, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x15, 0xb, 0, 0), (0x17, 0xb, 0, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x18, 0xb, 0, 3), (0x1b, 0xb, 0, 6, 1, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x1c, 0xb, 0, 0), (2, 0, 1, 5, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x661ff0 } (thiscall, ret 0x10, jap_emp_cup.cpp; a virtual +0x3c da Copa do Imperador): `JapEmpCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa0 sem memória.
unsafe extern "thiscall" fn cqn11_copa_do_imperador_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cqn11_tabela(this, fase, rodadas, extra, saida, &CQN11_COPA_DO_IMPERADOR, 0x9eab90, 0xa0, "cqn_11 tabela da copa do imperador")
}

unsafe extern "thiscall" fn comparar_cqn11_copa_do_imperador_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1794);
    cqn11_comparar("cqn11_copa_do_imperador_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqn11_copa_do_imperador_tabela(this, fase, r, e, s))
}

/// A Copa dos EUA (US Open Cup): cinco rodadas em jogo único de junho a outubro; na 1ª rodada o sorteio leva também o
/// bit 0x400 (+0xd = 0x483).
const CQN11_COPA_DOS_EUA: [Cqn11R; 5] = [
    cqn11r((1, 5, 0, 3), (0xe, 5, 0, 2, 2, 0), 0x14, (0, 0x483, 0), 4, (0x20, 0x10, 0x20, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xf, 5, 0, 3), (0x19, 5, 0, 6, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x1a, 5, 0, 0), (9, 7, 0, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xa, 7, 0, 3), (0x17, 7, 0, 2, 2, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqn11: a semifinal da Copa dos EUA
    cqn11r((0x18, 7, 0, 3), (0x16, 9, 0, 6, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x90b170 } (thiscall, ret 0x10, usa_open_cup.cpp; a virtual +0x3c da Copa dos EUA): `UsaOpenCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0x9c sem memória.
unsafe extern "thiscall" fn cqn11_copa_dos_eua_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cqn11_tabela(this, fase, rodadas, extra, saida, &CQN11_COPA_DOS_EUA, 0xaa20e0, 0x9c, "cqn_11 tabela da copa dos eua")
}

unsafe extern "thiscall" fn comparar_cqn11_copa_dos_eua_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1795);
    cqn11_comparar("cqn11_copa_dos_eua_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqn11_copa_dos_eua_tabela(this, fase, r, e, s))
}
// ---------- 276a leva: os inscritos das copas do Japão e dos EUA: a Copa da Liga japonesa, a Copa dos EUA e a Copa do Imperador (jap_j_cup.cpp, usa_open_cup.cpp, jap_emp_cup.cpp) ----------

unsafe fn cqn12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqn12_clube(i: i32) -> usize {
    cqn12_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize)
}

/// O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
unsafe fn cqn12_comp(c: usize, o: usize) -> Option<usize> {
    let d = cqn12_d(c + o);
    if d == 0 { None } else { Some(cqn12_d(d)) }
}

/// Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação 0).
unsafe fn cqn12_inscrever(this: usize, k: i32, clube: usize, classe: u8) {
    let e = cqn12_d(this + 0xa7) + k as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = classe;
    *((e + 5) as *mut u8) = 0;
}

/// O sorteio de 0 a n − 1 (0x90cfc0: push n; add esp, 4 -> cdecl).
unsafe fn cqn12_sortear(n: i32) -> i32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    sortear(n)
}

/// Ordena `n` clubes do vetor pela reputação e embaralha os melhores (0x4afbd0: push escolhas; push n; push vetor; add
/// esp, 0xc -> cdecl; já em Rust).
unsafe fn cqn12_embaralhar(v: *mut usize, n: i32, escolhas: u32) {
    let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    // ponytail: a original empurra a conta com lixo nos 16 bits altos; a 0x4afbd0 lê a palavra
    f(v as usize, n as i16 as u16 as u32, escolhas);
}

/// O qsort de `n` inscritos a partir de `base` (0x945333 com 0x522190: push 0x522190; push 6; push n; push base; add
/// esp, 0x10 -> cdecl; 0x522190 já em Rust).
unsafe fn cqn12_ordenar(base: usize, n: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(base, n, 6, 0x522190);
}

/// { 0x665a00 } (thiscall, ret 0, jap_j_cup.cpp): `JapJCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de 0xa8 em
/// +0xa7; sem memória, a caixa da linha 0x11a e −1). Os clubes da J2 ([0x9cf768]) enquanto cabem, classe 0; os da J1
/// ([0x9cf65c]) enquanto cabem, cada um com a classe sorteada de 0 a 19 (0x90cfc0(20), depois de gravar o clube); o
/// qsort (0x522190) das 16 inscrições a partir da 11ª (+0x42). Devolve 1 com a lista cheia, senão 0.
unsafe fn cqn12_copa_da_liga(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x1c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xa8);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9eaf14, 0x11a);
        return (-1, 0);
    }
    let mut n = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf768)) && n < cqn12_w(this + 0x3e) as i32 {
            cqn12_inscrever(this, n, c, 0);
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf65c)) && n < cqn12_w(this + 0x3e) as i32 {
            let e = cqn12_d(this + 0xa7) + n as usize * 6;
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            crate::comparacao::ramo("cqn_12 semente da j1");
            let s = cqn12_sortear(0x14); // cqn12: a classe sorteada dos clubes da J1
            *((e + 4) as *mut u8) = s as u8;
            *((e + 5) as *mut u8) = 0;
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    cqn12_ordenar(cqn12_d(this + 0xa7) + 0x42, 0x10);
    ((n == cqn12_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn cqn12_copa_da_liga_inscritos(this: usize) -> i32 {
    cqn12_copa_da_liga(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 e sorteiam: o +0x3e, o [0xb67a34] e o estado do sorteador a
/// partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xa7, a cópia faz a sua,
/// comparada nos inscritos que ela gravou e liberada.
unsafe fn cqn12_comparar(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqn12_d(this + 0xa7));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(c);
        lr.set(cqn12_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 6;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", nome, this, a, b, igual, lista, k));
    a
}

unsafe extern "thiscall" fn comparar_cqn12_copa_da_liga_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1796);
    cqn12_comparar("cqn12_copa_da_liga_inscritos", this, || o(this), || cqn12_copa_da_liga(this))
}

/// { 0x90b530 } (thiscall, ret 0, usa_open_cup.cpp): `UsaOpenCup::collect_entrants()`: +0x3e = 32. Os clubes cuja
/// divisão (+0x57, o ponteiro) é a competição do banco de [0x9cf594] ou a de [0x9cf598] vão para dois vetores locais de
/// até 50 (a passada para quando os dois enchem); cada vetor é ordenado e embaralhado (0x4afbd0, 10 escolhas). A lista
/// (0x945501, o calloc de +0x3e × 6, em +0xa7; sem memória, a caixa da linha 0x138 e −1) leva os 10 primeiros de cada
/// vetor com a classe 2 e depois todos os clubes da MLS ([0x9cf590]) com a classe 1 (sem conferir o espaço). Devolve 1.
unsafe fn cqn12_copa_dos_eua(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x20);
    let comp = |id: usize| cqn12_d(0xae23d0).wrapping_add(cqn12_d(id).wrapping_mul(0x6b));
    let (a, b) = (comp(0x9cf594), comp(0x9cf598));
    // ponytail: os vetores locais da original começam com o que estava na pilha; com menos de 10 clubes numa das
    // ligas ela copiaria essa sobra (e a 0x4afbd0 já daria a caixa): a cópia começa com zero
    let mut va = [0usize; 50];
    let mut vb = [0usize; 50];
    let (mut na, mut nb) = (0i8, 0i8);
    let mut i = 0i32;
    while i < cqn12_d(0xae235c) as i32 {
        if na == 0x32 && nb == 0x32 {
            break;
        }
        let c = cqn12_clube(i);
        let d = cqn12_d(c + 0x57);
        if d == a && na < 0x32 {
            va[na as usize] = c;
            na += 1;
        } else if d == b && nb < 0x32 {
            vb[nb as usize] = c;
            nb += 1;
        }
        i += 1;
    }
    cqn12_embaralhar(va.as_mut_ptr(), na as i32, 10);
    cqn12_embaralhar(vb.as_mut_ptr(), nb as i32, 10);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let l = calloc(cqn12_w(this + 0x3e) as i32 as u32, 6);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0xaa20e0, 0x138);
        return (-1, 0);
    }
    let mut k = 0i8;
    for &c in va.iter().take(10).chain(vb.iter().take(10)) {
        cqn12_inscrever(this, k as i32, c, 2); // cqn12: os 10 de cada liga de baixo
        k = k.wrapping_add(1);
    }
    crate::comparacao::ramo("cqn_12 inscritos da copa dos eua");
    let mut i = 0i32;
    while i < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf590)) {
            cqn12_inscrever(this, k as i32, c, 1);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    (1, k as i32)
}

unsafe extern "thiscall" fn cqn12_copa_dos_eua_inscritos(this: usize) -> i32 {
    cqn12_copa_dos_eua(this).0
}

unsafe extern "thiscall" fn comparar_cqn12_copa_dos_eua_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1797);
    cqn12_comparar("cqn12_copa_dos_eua_inscritos", this, || o(this), || cqn12_copa_dos_eua(this))
}

/// { 0x6624f0 } (thiscall, ret 0, jap_emp_cup.cpp): `JapEmpCup::collect_entrants()`: +0x3e = 80; a lista (0x944e46 de
/// 0x1e0 em +0xa7; sem memória, a caixa da linha 0x147 e −1). Dos clubes japoneses (+0x53 = [0x9cf36c]) com divisão, em
/// vetores locais: os da [0x9cf678] (até 100), os da [0x9cf674] (até 30), os da [0x9cf670] (até 30) e os de outras
/// divisões que não a J2 ([0x9cf768]) nem a J1 ([0x9cf65c]) (até 6); sem nenhum destes, o último da [0x9cf678] sai de lá
/// para cá. Com menos de 47, 2, 3 ou 1 em cada um, a caixa da linha 0x178 e 0. Cada vetor é ordenado e embaralhado
/// (0x4afbd0 com 47, 2, 3 e 1 escolhas); a lista leva os 47, os 2 e os 3 primeiros (o último vetor não entra), classe 0;
/// depois os da J2 enquanto cabem (classe 0) e os da J1 enquanto cabem, com a classe = a posição da temporada anterior
/// (+0x5f), até encher; o qsort (0x522190) das 16 últimas inscrições. Devolve 1 com a lista cheia, senão 0.
unsafe fn cqn12_copa_do_imperador(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x50);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x1e0);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9eab90, 0x147);
        return (-1, 0);
    }
    let mut va = [0usize; 100];
    let mut vb = [0usize; 30];
    let mut vc = [0usize; 30];
    let mut vd = [0usize; 6];
    let (mut na, mut nb, mut nc, mut nd) = (0i8, 0i8, 0i8, 0i8);
    if cqn12_d(0xae235c) as i32 > 0 {
        let mut i: i16 = 0;
        while (i as i32) < cqn12_d(0xae235c) as i32 {
            let c = cqn12_clube(i as i32);
            if cqn12_d(c + 0x57) != 0 && cqn12_comp(c, 0x53) == Some(cqn12_d(0x9cf36c)) {
                let d = cqn12_d(cqn12_d(c + 0x57));
                if d == cqn12_d(0x9cf678) {
                    if na < 0x64 {
                        va[na as usize] = c;
                        na += 1;
                    }
                } else if d == cqn12_d(0x9cf674) {
                    if nb < 0x1e {
                        vb[nb as usize] = c;
                        nb += 1;
                    }
                } else if d == cqn12_d(0x9cf670) {
                    if nc < 0x1e {
                        vc[nc as usize] = c;
                        nc += 1;
                    }
                } else if d != cqn12_d(0x9cf768) && d != cqn12_d(0x9cf65c) && nd < 6 {
                    vd[nd as usize] = c;
                    nd += 1;
                }
            }
            i = i.wrapping_add(1);
        }
        if nd == 0 {
            na = na.wrapping_sub(1);
            nd = 1;
            vd[0] = if na >= 0 { va[na as usize] } else { 0 };
        }
    } else {
        // ponytail: sem clubes a original lê o vetor fora do lugar (índice −1); a cópia usa zero e cai na caixa abaixo
        na = na.wrapping_sub(1);
        nd = 1;
        vd[0] = 0;
    }
    if na < 0x2f || nb < 2 || nc < 3 || nd < 1 {
        aviso_em(0x9eab90, 0x178);
        return (0, 0);
    }
    cqn12_embaralhar(va.as_mut_ptr(), na as i32, 0x2f);
    cqn12_embaralhar(vb.as_mut_ptr(), nb as i32, 2);
    cqn12_embaralhar(vc.as_mut_ptr(), nc as i32, 3);
    cqn12_embaralhar(vd.as_mut_ptr(), nd as i32, 1);
    crate::comparacao::ramo("cqn_12 inscritos da copa do imperador");
    let mut n = 0i32;
    for &c in va.iter().take(0x2f).chain(vb.iter().take(2)).chain(vc.iter().take(3)) {
        cqn12_inscrever(this, n, c, 0); // cqn12: os sorteados das divisões de baixo
        n += 1;
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf768)) && n < cqn12_w(this + 0x3e) as i32 {
            cqn12_inscrever(this, n, c, 0);
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < cqn12_d(0xae235c) as i32 {
        let c = cqn12_clube(i as i32);
        if cqn12_comp(c, 0x57) == Some(cqn12_d(0x9cf65c)) && n < cqn12_w(this + 0x3e) as i32 {
            crate::comparacao::ramo("cqn_12 classe da j1 na copa do imperador");
            cqn12_inscrever(this, n, c, *((c + 0x5f) as *const u8)); // cqn12: a classe dos clubes da J1 na Copa do Imperador
            n += 1;
            if n == cqn12_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    cqn12_ordenar(cqn12_d(this + 0xa7).wrapping_add(((n * 3 - 0x30) * 2) as isize as usize), 0x10);
    ((n == cqn12_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn cqn12_copa_do_imperador_inscritos(this: usize) -> i32 {
    cqn12_copa_do_imperador(this).0
}

unsafe extern "thiscall" fn comparar_cqn12_copa_do_imperador_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1798);
    cqn12_comparar("cqn12_copa_do_imperador_inscritos", this, || o(this), || cqn12_copa_do_imperador(this))
}

