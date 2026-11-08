// ---------- %%LEVA%%a leva: a Copa da UEFA e a Intertoto: as rodadas da Copa da UEFA, os inscritos da Intertoto e a lista dos seus clubes (uefa_cup.cpp, intertoto_cup.cpp) ----------

/// As rodadas fixas da Copa da UEFA (a preliminar e os campos que dependem de +0xb2 vêm à parte, em
/// `uefa_cup_rounds`): a 1ª rodada de qualificação em agosto, a 2ª no fim de agosto, a 1ª rodada em setembro, a 2ª
/// em novembro, a 3ª em dezembro, as oitavas em fevereiro, as quartas e a semifinal em março e abril e a final em
/// maio, todas em ida e volta menos a final.
const UEFA_CUP_ROUNDS: [EngCcCupCupRound; 8] = [
    cql10r((0x1c, 6, 0, 4), (0xa, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqq10: a 1ª rodada de qualificação da UEFA
    cql10r((0x19, 7, 0, 4), (0xe, 8, 0, 3, 2, 0), 0xa, (4, 0x400, 3), 7, (0x60, 0x30, 0x37, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 8, 0, 4), (0x13, 9, 0, 3, 2, 0), 0x14, (4, 0, 3), 7, (0x30, 0x18, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xa, 0, 4), (0x17, 0xa, 0, 3, 2, 0), 0x1e, (0, 0, 3), 8, (0x20, 0x10, 8, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 0xb, 0, 4), (1, 2, 1, 3, 2, 0), 0x28, (0, 0, 3), 8, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 2, 1, 4), (0xf, 2, 1, 3, 2, 0), 0x78, (0, 0, 3), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 2, 1, 4), (5, 3, 1, 3, 2, 0), 0x82, (0, 0, 3), 8, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 4), (0x10, 4, 1, 2, 2, 4), 0x96, (0, 0x83, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0x16e360, 0)),
];

/// A preliminar (só com `b` = +0xb2 > 0): sorteio em 28/6, jogo em 6/7; b times, b/2 jogos, b novos.
const UEFA_CUP_PRELIMINARY_ROUND: EngCcCupCupRound =
    cql10r((0x1c, 5, 0, 2), (6, 6, 0, 3, 2, 0), 0xa0, (0, 0x200, 3), 7, (0, 0, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0));

/// As rodadas da Copa da UEFA com `b` = +0xb2 (byte com sinal; as metades truncam como a original, cdq/sar): com b > 0
/// a preliminar (b times, b/2 jogos, b novos) e depois as oito; na 1ª rodada de qualificação 0x52 − b/2 novos e b já
/// inscritos, na 2ª b/2 + 0x52 inscritos, na 1ª rodada b/2 + 0x89 inscritos.
fn uefa_cup_rounds(b: i8) -> ([EngCcCupCupRound; 9], usize) {
    let mut t = [UEFA_CUP_PRELIMINARY_ROUND; 9];
    let (b, m) = (b as i32, b as i32 / 2);
    let mut u = UEFA_CUP_ROUNDS;
    u[0].times.2 = (0x52 - m) as u16;
    u[0].times.3 = b as u16;
    u[1].times.3 = (m + 0x52) as u16;
    u[3].times.3 = (m + 0x89) as u16;
    let mut n = 0;
    if b > 0 {
        crate::comparacao::ramo("cqq_10 preliminar da uefa");
        t[0].times = (b as u16, m as u16, b as u16, 0);
        n = 1;
    }
    for r in u {
        t[n] = r;
        n += 1;
    }
    (t, n)
}

/// { 0x9025c0 } (thiscall, ret 0x10, uefa_cup.cpp; a virtual +0x3c da Copa da UEFA): `UefaCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 9, extra = 0; a tabela (0x944e46 de 0x3a8;
/// sem memória, a caixa da linha 0xe0 e 0) com as rodadas de `uefa_cup_rounds` e rodadas = as gravadas. Fora da −1,
/// 0.
unsafe extern "thiscall" fn uefa_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 9;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x3a8);
    if t == 0 {
        show_error_at(0xaa19e0, 0xe0);
        return 0;
    }
    crate::comparacao::ramo("cqq_10 tabela da uefa");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let (tab, n) = uefa_cup_rounds(*((this + 0xb2) as *const i8));
    for (i, r) in tab[..n].iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, ano);
    }
    *rodadas = n as i16;
    t
}

unsafe extern "thiscall" fn comparar_uefa_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    eng_cc_cup_compare_round_table("uefa_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| uefa_cup_create_round_table(this, fase, r, e, s))
}

unsafe fn cqq10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqq10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x6336c0 } (thiscall, ret 0, intertoto_cup.cpp): `IntertotoCup::collect_entrants()`: +0x3e = 60; um vetor local de
/// 60 ponteiros (0x944e46 de 0xf0; sem memória, a caixa da linha 0x161 e 0) com os clubes da tabela ([0xae23b4], até
/// [0xae235c]) cuja copa europeia (+0x1db) é a Intertoto ([0x9cf6f8]), até 60; faltando, a caixa da linha 0x172 e 0 (o
/// vetor fica alocado, como na original). A lista (0x944e46 de 6 × 60 em +0xa7; sem memória, a caixa 0x17c e −1) leva
/// cada clube com a classe = o seu coeficiente (+0x1df) e a situação 0, vai ao qsort (0x522190) e o vetor é liberado.
/// Devolve 1 com os 60.
unsafe fn collect_intertoto_entrants(this: usize) -> (i32, i32) {
    const F: usize = 0x9e9e14;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x3c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0xf0);
    if v == 0 {
        show_error_at(F, 0x161);
        return (0, 0);
    }
    let mut k = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < cqq10_d(0xae235c) as i32 {
        if k >= cqq10_w(this + 0x3e) as i32 {
            break;
        }
        let c = cqq10_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize);
        if cqq10_d(c + 0x1db) == cqq10_d(0x9cf6f8) {
            core::ptr::write_unaligned((v + k as usize * 4) as *mut u32, c as u32);
            k += 1;
        }
        i = i.wrapping_add(1);
    }
    let n = cqq10_w(this + 0x3e) as i32;
    if k < n {
        show_error_at(F, 0x172);
        return (0, 0);
    }
    let l = alocar((n * 6) as u32);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(F, 0x17c);
        return (-1, 0);
    }
    crate::comparacao::ramo("cqq_10 inscritos da intertoto");
    let mut j: i16 = 0;
    while j < cqq10_w(this + 0x3e) {
        let c = cqq10_d(v + j as usize * 4);
        let e = cqq10_d(this + 0xa7) + j as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = *((c + 0x1df) as *const u8); // cqq10: a classe pelo coeficiente
        *((e + 5) as *mut u8) = 0;
        j = j.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqq10_d(this + 0xa7), cqq10_w(this + 0x3e) as i32 as u32, 6, 0x522190);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    soltar(v);
    ((k == cqq10_w(this + 0x3e) as i32) as i32, cqq10_w(this + 0x3e) as i32)
}

unsafe extern "thiscall" fn intertoto_cup_collect_entrants(this: usize) -> i32 {
    collect_intertoto_entrants(this).0
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a lista dela em +0xa7, a cópia faz a
/// sua, comparada nos inscritos (com a devolução 1) e liberada.
unsafe extern "thiscall" fn comparar_intertoto_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let antes = cqq10_d(this + 0xa7);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqq10_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, antes as u32);
        r
    }, || {
        let (r, n) = collect_intertoto_entrants(this);
        lr.set(cqq10_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { nr.get().max(0) as usize * 6 } else { 0 };
    let lista = if lo != antes && lr != antes && lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == lr || k == 0
    };
    if lr != 0 && lr != lo && lr != antes {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("intertoto_cup_collect_entrants({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x633f70 } (thiscall, ret 8, intertoto_cup.cpp; uma virtual): `IntertotoCup::collect_club_list(&saida, ativos)`:
/// uma lista de ponteiros (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2fb, saida = 0 e 0) com os clubes da
/// lista +0xa7 (não nulos, até +0x3e): com `ativos`, só os de situação (+5) 0 ou 1. Grava a lista em saida e devolve a
/// conta (palavra).
unsafe fn collect_club_list(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let n = cqq10_w(this + 0x3e);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar((n as i32 * 4) as u32);
    if l == 0 {
        show_error_at(0x9e9e14, 0x2fb);
        *saida = 0;
        return 0;
    }
    crate::comparacao::ramo("cqq_10 lista da intertoto");
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    while i < cqq10_w(this + 0x3e) {
        let e = cqq10_d(this + 0xa7) + i as usize * 6;
        let s = *((e + 5) as *const u8);
        let c = cqq10_d(e);
        if (ativos as u8 == 0 || s == 1 || s == 0) && c != 0 && k < n { // cqq10: os clubes da lista
            core::ptr::write_unaligned((l + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    *saida = l;
    k as u16
}

unsafe extern "thiscall" fn intertoto_cup_collect_club_list(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    collect_club_list(this, saida, ativos)
}

/// Lado a lado: a original grava a lista dela em `saida` (de quem chama, que a libera); a cópia faz a sua numa saída
/// própria, comparada na conta e nos ponteiros, e a libera.
unsafe extern "thiscall" fn comparar_intertoto_cup_collect_club_list(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(%%I2%%);
    let mut s2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, saida, ativos), || collect_club_list(this, &mut s2, ativos));
    let lo = *saida;
    let lista = if lo != 0 && s2 != 0 {
        core::slice::from_raw_parts(lo as *const u8, a as usize * 4) == core::slice::from_raw_parts(s2 as *const u8, b as usize * 4)
    } else {
        lo == 0 && s2 == 0
    };
    if s2 != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(s2);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("intertoto_cup_collect_club_list({:#x}, {}) devolveu {} x {} lista {}", this, ativos as u8, a, b, lista));
    a
}
