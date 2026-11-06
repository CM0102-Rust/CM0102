// ---------- %%LEVA%%a leva: as copas sul-americanas de clubes: o calendário da Libertadores e da Mercosul e os convidados da Mercosul (conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
type Cqo13G = (u32, u32, u32, u32);

/// As duas fases da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10; a fase lida
/// como byte com sinal) das copas sul-americanas. Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1,
/// rodadas = o número de rodadas de grupo, extra = fase + 0x3e9 (16 bits); a tabela de 0x41 bytes por rodada (0x944e46;
/// sem memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0,
/// rodadas = o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as
/// rodadas pela `cql10_rodada` com o ano +0x40 da copa (com sinal) e as palavras de `a_mais` (rodada, deslocamento,
/// valor). Acima de `limite`: devolve 0 sem gravar nada.
unsafe fn cqo13_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, grupos: &[Cqo13G],
    mata: &[Cql10R], a_mais: &[(usize, usize, u16)], arquivo: usize, linha_g: u32, linha_m: u32, nome_g: &'static str, nome: &'static str) -> usize {
    let f = fase as u8 as i8;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if f < limite {
        let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
        let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
        if !liga.is_null() {
            *liga = 1;
        }
        *rodadas = grupos.len() as i16;
        *extra = (f as i16).wrapping_add(0x3e9);
        let t = alocar(grupos.len() as u32 * 0x41);
        if t == 0 {
            aviso_em(arquivo, linha_g);
            return 0;
        }
        crate::comparacao::ramo(nome_g);
        // o ano vai com lixo nos bytes altos na original (o registrador de antes com a palavra +0x40 por cima); a
        // 0x68a160 só usa os 16 bits de baixo: a cópia passa o ano limpo
        let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
            data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, ano, 0);
            jogo(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limite {
        return 0;
    }
    if !liga.is_null() {
        *liga = 0;
    }
    *rodadas = mata.len() as i16;
    *extra = 0;
    let t = alocar(mata.len() as u32 * 0x68);
    if t == 0 {
        aviso_em(arquivo, linha_m);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in mata.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    for &(r, o, v) in a_mais {
        core::ptr::write_unaligned((t + r * 0x68 + o) as *mut u16, v);
    }
    t
}

/// Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
unsafe fn cqo13_gravados_grupos(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (grupos ou mata-mata, este com as
/// palavras de `a_mais`) têm de ser iguais; a tabela da cópia é liberada (0x9452ca). Acima do mata-mata só a original.
unsafe fn cqo13_comparar(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limite: i8, a_mais: &[(usize, usize, u16)],
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = fase as u8 as i8;
    if f > limite {
        // ponytail: acima do mata-mata nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, f, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let mais = |t: usize| a_mais.iter().flat_map(|&(r, o, _)| core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2).to_vec()).collect::<Vec<u8>>();
    let tabela = if a != 0 && b != 0 {
        if f < limite {
            cqo13_gravados_grupos(a, *rodadas) == cqo13_gravados_grupos(b, r2)
        } else {
            cql10_gravados(a, *rodadas) == cql10_gravados(b, r2) && mais(a) == mais(b)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, f, saidas, *rodadas, r2, tabela));
    a
}

/// A Libertadores, grupos: seis rodadas às terças [inferência: dia da semana 2], de 25 de fevereiro a 1º de abril.
const CQO13_LIBERTADORES_GRUPOS: [Cqo13G; 6] = [(0x19, 1, 2, 2), (4, 2, 2, 2), (0xb, 2, 2, 2), (0x12, 2, 2, 2), (0x19, 2, 2, 2), (1, 3, 2, 2)]; // cqo13: os grupos da Libertadores

/// A Libertadores, mata-mata: quatro rodadas em ida e volta, de abril a junho (o jogo de volta 14 dias depois nas
/// oitavas e 7 nas outras).
const CQO13_LIBERTADORES: [Cql10R; 4] = [
    cql10r((2, 3, 0, 3), (0xf, 3, 0, 2, 2, 0), 0x14, (0, 4, 1), 7, (0x10, 8, 0x10, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo13: as oitavas da Libertadores
    cql10r((0x1e, 3, 0, 3), (0x14, 4, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x1c, 4, 0, 3), (3, 5, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 5, 0, 3), (0x11, 5, 0, 2, 2, 0), 0x96, (0, 4, 1), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0xc3500)),
];

/// { 0x4c31c0 } (thiscall, ret 0x10, conmebol_liber.cpp; a virtual +0x3c da Libertadores): `ConmebolLiber::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (a caixa da linha 0x9f), o mata-mata na
/// fase 7 (a caixa da linha 0xc5).
unsafe extern "thiscall" fn cqo13_libertadores_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo13_tabela(this, fase, rodadas, extra, liga, 7, &CQO13_LIBERTADORES_GRUPOS, &CQO13_LIBERTADORES, &[], 0x9a7de4, 0x9f, 0xc5,
        "cqo_13 grupos da libertadores", "cqo_13 mata-mata da libertadores")
}

unsafe extern "thiscall" fn comparar_cqo13_libertadores_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cqo13_comparar("cqo13_libertadores_tabela", fase, rodadas, extra, liga, 7, &[], || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo13_libertadores_tabela(this, fase, r, e, l))
}

/// A Mercosul, grupos: seis rodadas às terças, de 29 de julho a 14 de outubro.
const CQO13_MERCOSUL_GRUPOS: [Cqo13G; 6] = [(0x1d, 6, 2, 2), (0x13, 7, 2, 2), (2, 8, 2, 2), (0x10, 8, 2, 2), (0x1e, 8, 2, 2), (0xe, 9, 2, 2)]; // cqo13: os grupos da Mercosul

/// A Mercosul, mata-mata: quartas, semifinal e final em ida e volta, de outubro a dezembro (a volta 3 dias depois nas
/// quartas e na semifinal, 7 na final).
const CQO13_MERCOSUL: [Cql10R; 3] = [
    cql10r((0xf, 9, 0, 3), (0x1c, 9, 0, 2, 2, 0), 0x78, (0, 4, 1), 8, (8, 4, 8, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((5, 0xa, 0, 3), (0xb, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 1), 8, (4, 2, 0, 0), (0, 2, 3), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 0xa, 0, 3), (2, 0xb, 0, 2, 2, 0), 0x96, (0, 0x2000, 0x2000), 8, (2, 1, 0, 0), (0, 2, 7), (0, 0x1e8480, 0)),
];

/// A final da Mercosul grava ainda a palavra +0x11 = 3 [inferência: o desempate da final, pênaltis].
const CQO13_MERCOSUL_A_MAIS: [(usize, usize, u16); 1] = [(2, 0x11, 3)];

/// { 0x4c4930 } (thiscall, ret 0x10, conmebol_merc.cpp; a virtual +0x3c da Mercosul): `ConmebolMerc::create_schedule(
/// fase, &rodadas, &extra, &é_liga)`: grupos abaixo da fase 4 (a caixa da linha 0x97), o mata-mata na fase 4 (a caixa
/// da linha 0xbd).
unsafe extern "thiscall" fn cqo13_mercosul_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqo13_tabela(this, fase, rodadas, extra, liga, 4, &CQO13_MERCOSUL_GRUPOS, &CQO13_MERCOSUL, &CQO13_MERCOSUL_A_MAIS, 0x9a7e5c, 0x97, 0xbd,
        "cqo_13 grupos da mercosul", "cqo_13 mata-mata da mercosul")
}

unsafe extern "thiscall" fn comparar_cqo13_mercosul_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    cqo13_comparar("cqo13_mercosul_tabela", fase, rodadas, extra, liga, 4, &CQO13_MERCOSUL_A_MAIS, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqo13_mercosul_tabela(this, fase, r, e, l))
}

unsafe fn cqo13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// { 0x4c5850 } (thiscall, ret 0, conmebol_merc.cpp): `ConmebolMerc::select_invited_clubs()`: +0xf2 = 20 e a lista de
/// ponteiros (0x944e46 de 0x50 em +0xee; sem memória, a caixa da linha 0x2c6 e 0); cada clube da tabela ([0xae23b4],
/// [0xae235c] − 2 × [0xae2350]) que as regras ([0xae1000], 0x4c6890: thiscall com o clube) convidam entra na lista, sem
/// conferir o espaço; a lista é embaralhada (0x4afb40 com +0xf2). Devolve 1.
unsafe fn cqo13_convidados(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x14);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x50);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(0x9a7e5c, 0x2c6);
        return (0, 0);
    }
    let convidado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4c6890usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    let clubes = || (cqo13_d(0xae235c) as i32).wrapping_sub((cqo13_d(0xae2350) as i32).wrapping_mul(2));
    let mut k = 0i16;
    let mut i = 0i32;
    while i < clubes() {
        let c = cqo13_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize);
        if convidado(cqo13_d(0xae1000), c) != 0 {
            core::ptr::write_unaligned((cqo13_d(this + 0xee) + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    crate::comparacao::ramo("cqo_13 convidados da mercosul");
    // ponytail: a original empurra a palavra +0xf2 com lixo nos 16 bits altos (o eax de antes); a 0x4afb40 lê a palavra
    embaralhar(cqo13_d(this + 0xee), core::ptr::read_unaligned((this + 0xf2) as *const u16) as u32); // cqo13: embaralha os convidados
    (1, k as i32)
}

unsafe extern "thiscall" fn cqo13_mercosul_convidados(this: usize) -> i32 {
    cqo13_convidados(this).0
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nos convidados que ela gravou e liberada.
unsafe extern "thiscall" fn comparar_cqo13_mercosul_convidados(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqo13_d(this + 0xee));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(|| cqo13_convidados(this));
        lr.set(cqo13_d(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqo13_mercosul_convidados({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", this, a, b, igual, lista, k));
    a
}
