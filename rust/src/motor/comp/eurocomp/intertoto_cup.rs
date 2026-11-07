// ---------- 281a leva: o calendário das copas intercontinentais e da Intertoto: a Intertoto, a Copa Intercontinental, a Copa Interamericana e a Copa dos Campeões da CONCACAF (intertoto_cup.cpp, world_club_cup.cpp, inter_amer_cup.cpp, con_champ.cpp) ----------

/// A Intertoto: cinco rodadas em ida e volta de junho a agosto (a 1ª com 40 times; 12 novos na 2ª, 8 na 3ª).
const CQP13_INTERTOTO: [Cql10R; 5] = [
    cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: a 1ª rodada da Intertoto
    cql10r((1, 6, 0, 3), (3, 6, 0, 5, 1, 0), 0x14, (0, 0, 3), 7, (0x20, 0x10, 0xc, 0x28), (0, 2, 7), (0, 0, 0)),
    cql10r((0xb, 6, 0, 6), (0x11, 6, 0, 5, 1, 0), 0x1e, (0, 0, 3), 7, (0x18, 0xc, 8, 0x34), (0, 2, 7), (0, 0, 0)),
    cql10r((0x19, 6, 0, 6), (0x1c, 6, 0, 2, 2, 0), 0x82, (0, 0, 3), 8, (0xc, 6, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((5, 7, 0, 3), (0xa, 7, 0, 1, 2, 0), 0x96, (0, 0, 3), 8, (6, 3, 0, 0), (0, 2, 0xe), (0, 0, 0)),
];

/// { 0x633300 } (thiscall, ret 0x10, intertoto_cup.cpp; a virtual +0x3c): `IntertotoCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0xe4 sem memória.
unsafe extern "thiscall" fn cqp13_intertoto_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERTOTO, 0x9e9e14, 0xe4, "cqp_13 tabela da intertoto")
}

unsafe extern "thiscall" fn comparar_cqp13_intertoto_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1973);
    cql10_comparar("cqp13_intertoto_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_intertoto_tabela(this, fase, r, e, s))
}

/// A Copa Intercontinental: jogo único em 1º de dezembro (sorteio em 10 de outubro), mando 4 (Tóquio [inferência]).
const CQP13_INTERCONTINENTAL: [Cql10R; 1] = [
    cql10r((0xa, 9, 0, 5), (1, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp13: a Copa Intercontinental
];

/// { 0x92b770 } (thiscall, ret 0x10, world_club_cup.cpp; a virtual +0x3c): `WorldClubCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
unsafe extern "thiscall" fn cqp13_intercontinental_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERCONTINENTAL, 0xad5848, 0x9a, "cqp_13 tabela da intercontinental")
}

unsafe extern "thiscall" fn comparar_cqp13_intercontinental_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1974);
    cql10_comparar("cqp13_intercontinental_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_intercontinental_tabela(this, fase, r, e, s))
}

/// A Copa Interamericana: ida e volta a partir de 26 de janeiro do ano seguinte (sorteio em 10 de outubro), 7 dias.
const CQP13_INTERAMERICANA: [Cql10R; 1] = [
    cql10r((0xa, 9, 0, 5), (0x1a, 0, 1, 2, 2, 0), 0, (0, 0, 3), 8, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)),
];

/// { 0x632340 } (thiscall, ret 0x10, inter_amer_cup.cpp; a virtual +0x3c): `InterAmerCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
unsafe extern "thiscall" fn cqp13_interamericana_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERAMERICANA, 0x9e9d94, 0x9a, "cqp_13 tabela da interamericana")
}

unsafe extern "thiscall" fn comparar_cqp13_interamericana_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1975);
    cql10_comparar("cqp13_interamericana_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_interamericana_tabela(this, fase, r, e, s))
}

/// A Copa dos Campeões da CONCACAF: quartas (28/8), semifinal (1/9) e final (4/9) em jogo único, 8 times.
const CQP13_CONCACAF: [Cql10R; 3] = [
    cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: as quartas da CONCACAF
    cql10r((0x1d, 8, 0, 4), (1, 9, 0, 6, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((2, 9, 0, 0), (4, 9, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x4c0f00 } (thiscall, ret 0x10, con_champ.cpp; a virtual +0x3c): `ConChamp::create_round_table(fase, &rodadas,
/// &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 3, extra = 0; a tabela (0x944e46 de 0x138; sem
/// memória, a caixa da linha 0x97 e 0) com as 3 rodadas e, nas duas primeiras, o dia 0 do confronto vazio (0x521f10
/// com −1, −1, −1 e mando 0; a original o chama entre o jogo e os campos, que não se sobrepõem). Fora da −1, 0.
unsafe extern "thiscall" fn cqp13_concacaf_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 3;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x138);
    if t == 0 {
        aviso_em(0x9a7d30, 0x97);
        return 0;
    }
    crate::comparacao::ramo("cqp_13 tabela da concacaf");
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in CQP13_CONCACAF.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
        if i < 2 {
            elo(t, i as i32, 0, -1, -1, -1, 0);
        }
    }
    t
}

unsafe extern "thiscall" fn comparar_cqp13_concacaf_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1976);
    let a = cql10_comparar("cqp13_concacaf_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp13_concacaf_tabela(this, fase, r, e, s));
    // ponytail: a cql10_comparar não olha os dias dos confrontos (+0x23..); o dia 0 das duas primeiras rodadas é
    // conferido aqui contra o valor fixo que a cópia grava
    if a != 0 {
        let ok = (0..2usize).all(|r| {
            let e = a + r * 0x68 + 0x23;
            core::ptr::read_unaligned(e as *const u32) == 0 && core::slice::from_raw_parts((e + 4) as *const u8, 3) == [0xff, 0xff, 0xff]
        });
        PLACAR.registrar(ok, || format!("cqp13_concacaf_tabela: o dia 0 dos confrontos da original difere"));
    }
    a
}

// ---------- 281a leva: a Copa da UEFA e a Intertoto: as rodadas da Copa da UEFA, os inscritos da Intertoto e a lista dos seus clubes (uefa_cup.cpp, intertoto_cup.cpp) ----------

/// As rodadas fixas da Copa da UEFA (a preliminar e os campos que dependem de +0xb2 vêm à parte, em
/// `cqq10_uefa_rodadas`): a 1ª rodada de qualificação em agosto, a 2ª no fim de agosto, a 1ª rodada em setembro, a 2ª
/// em novembro, a 3ª em dezembro, as oitavas em fevereiro, as quartas e a semifinal em março e abril e a final em
/// maio, todas em ida e volta menos a final.
const CQQ10_UEFA: [Cql10R; 8] = [
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
const CQQ10_UEFA_PRELIMINAR: Cql10R =
    cql10r((0x1c, 5, 0, 2), (6, 6, 0, 3, 2, 0), 0xa0, (0, 0x200, 3), 7, (0, 0, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0));

/// As rodadas da Copa da UEFA com `b` = +0xb2 (byte com sinal; as metades truncam como a original, cdq/sar): com b > 0
/// a preliminar (b times, b/2 jogos, b novos) e depois as oito; na 1ª rodada de qualificação 0x52 − b/2 novos e b já
/// inscritos, na 2ª b/2 + 0x52 inscritos, na 1ª rodada b/2 + 0x89 inscritos.
fn cqq10_uefa_rodadas(b: i8) -> ([Cql10R; 9], usize) {
    let mut t = [CQQ10_UEFA_PRELIMINAR; 9];
    let (b, m) = (b as i32, b as i32 / 2);
    let mut u = CQQ10_UEFA;
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
/// sem memória, a caixa da linha 0xe0 e 0) com as rodadas de `cqq10_uefa_rodadas` e rodadas = as gravadas. Fora da −1,
/// 0.
unsafe extern "thiscall" fn cqq10_uefa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
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
        aviso_em(0xaa19e0, 0xe0);
        return 0;
    }
    crate::comparacao::ramo("cqq_10 tabela da uefa");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let (tab, n) = cqq10_uefa_rodadas(*((this + 0xb2) as *const i8));
    for (i, r) in tab[..n].iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    *rodadas = n as i16;
    t
}

unsafe extern "thiscall" fn comparar_cqq10_uefa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1977);
    cql10_comparar("cqq10_uefa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqq10_uefa_tabela(this, fase, r, e, s))
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
unsafe fn cqq10_intertoto(this: usize) -> (i32, i32) {
    const F: usize = 0x9e9e14;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x3c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0xf0);
    if v == 0 {
        aviso_em(F, 0x161);
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
        aviso_em(F, 0x172);
        return (0, 0);
    }
    let l = alocar((n * 6) as u32);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(F, 0x17c);
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

unsafe extern "thiscall" fn cqq10_intertoto_inscritos(this: usize) -> i32 {
    cqq10_intertoto(this).0
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a lista dela em +0xa7, a cópia faz a
/// sua, comparada nos inscritos (com a devolução 1) e liberada.
unsafe extern "thiscall" fn comparar_cqq10_intertoto_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1978);
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
        let (r, n) = cqq10_intertoto(this);
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
    PLACAR.registrar(igual && a == b && lista, || format!("cqq10_intertoto_inscritos({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x633f70 } (thiscall, ret 8, intertoto_cup.cpp; uma virtual): `IntertotoCup::collect_club_list(&saida, ativos)`:
/// uma lista de ponteiros (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2fb, saida = 0 e 0) com os clubes da
/// lista +0xa7 (não nulos, até +0x3e): com `ativos`, só os de situação (+5) 0 ou 1. Grava a lista em saida e devolve a
/// conta (palavra).
unsafe fn cqq10_lista(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let n = cqq10_w(this + 0x3e);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar((n as i32 * 4) as u32);
    if l == 0 {
        aviso_em(0x9e9e14, 0x2fb);
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

unsafe extern "thiscall" fn cqq10_intertoto_lista(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    cqq10_lista(this, saida, ativos)
}

/// Lado a lado: a original grava a lista dela em `saida` (de quem chama, que a libera); a cópia faz a sua numa saída
/// própria, comparada na conta e nos ponteiros, e a libera.
unsafe extern "thiscall" fn comparar_cqq10_intertoto_lista(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(1979);
    let mut s2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, saida, ativos), || cqq10_lista(this, &mut s2, ativos));
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
    PLACAR.registrar(igual && a == b && lista, || format!("cqq10_intertoto_lista({:#x}, {}) devolveu {} x {} lista {}", this, ativos as u8, a, b, lista));
    a
}

// ---------- 273a leva: a Copa Intertoto e a Copa da UEFA: os três vencedores do Intertoto na Copa da UEFA e o destino de cada clube nas duas (intertoto_cup.cpp, uefa_cup.cpp) ----------

unsafe fn cqi10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqi10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqi10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O objeto da Copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]), ou 0.
unsafe fn cqi10_uefa() -> usize {
    cqi10_d(cqi10_d(0xadadfc).wrapping_add(cqi10_d(0x9cf6f0).wrapping_mul(4)))
}

/// Os vencedores do Intertoto (+5 = 1 no registro de 6 bytes de +0xa7) que cabem nas três últimas vagas da UEFA, na
/// ordem em que a 0x633d40 os põe: as vagas vão de +0x3e da UEFA − 27 até + 3 (palavras com sinal).
unsafe fn cqi10_vencedores(this: usize, uefa: usize) -> ([usize; 3], usize) {
    let mut v = [0usize; 3];
    let mut k = 0usize;
    let mut vaga = cqi10_w(uefa + 0x3e).wrapping_sub(0x1b);
    let limite = vaga.wrapping_add(3);
    let mut i = 0i16;
    if cqi10_w(this + 0x3e) > 0 {
        loop {
            if vaga >= limite {
                break;
            }
            let e = cqi10_d(this + 0xa7).wrapping_add((i as isize * 6) as usize);
            if cqi10_b(e + 5) == 1 {
                v[k] = cqi10_d(e);
                k += 1;
                vaga = vaga.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            if i >= cqi10_w(this + 0x3e) {
                break;
            }
        }
    }
    (v, k)
}

/// { 0x633d40 } (thiscall, ret 8, intertoto_cup.cpp; a virtual +0x14 do Intertoto): `IntertotoCup::update_for_date(hoje,
/// fase)`. Uma vez (+0xb2 = 0), quando a copa acabou (virtual +0x10 com 0, 1: push 1; push 0; mov ecx, this ->
/// thiscall, ret 8; o byte de baixo): sem a Copa da UEFA, a caixa da linha 0x29e; com ela, cada time do Intertoto com
/// +5 = 1 (os vencedores das finais) vai, em ordem, para as vagas de +0x3e − 27 em diante da tabela +0xa7 da UEFA (até
/// três: o clube, +4 e +5 zerados) e para o livro de inscritos dela (+0x14, 6 bytes: o clube e a situação 4 em +4;
/// +0x36 sobe); depois as posições previstas da UEFA (virtual +0x5c: mov ecx, uefa -> thiscall, ret 0), +0xb2 = 1 e de
/// novo as posições previstas (a original relê a UEFA). Sempre, no fim, a atualização da copa (0x51f890: push fase;
/// push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn cqi10_intertoto_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqi10_d(this + 0xb2) == 0 {
        let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqi10_d(cqi10_d(this) + 0x10));
        if acabou(this, 0, 1) as u8 != 0 {
            let uefa = cqi10_uefa();
            if uefa == 0 {
                aviso_em(0x9e9e14, 0x29e);
            } else {
                let mut vaga = cqi10_w(uefa + 0x3e).wrapping_sub(0x1b);
                let limite = vaga.wrapping_add(3);
                let mut i = 0i16;
                if cqi10_w(this + 0x3e) > 0 {
                    loop {
                        if vaga >= limite {
                            break;
                        }
                        let e = cqi10_d(this + 0xa7).wrapping_add((i as isize * 6) as usize);
                        if cqi10_b(e + 5) == 1 {
                            crate::comparacao::ramo("cqi_10 vencedor na uefa");
                            let s = cqi10_d(uefa + 0xa7).wrapping_add((vaga as isize * 6) as usize);
                            core::ptr::write_unaligned(s as *mut u32, cqi10_d(e) as u32);
                            *((s + 4) as *mut u8) = 0;
                            *((s + 5) as *mut u8) = 0;
                            let l = cqi10_d(uefa + 0x14).wrapping_add((cqi10_w(uefa + 0x36) as isize * 6) as usize);
                            core::ptr::write_unaligned(l as *mut u32, cqi10_d(s) as u32);
                            *((l + 4) as *mut u8) = 4; // cqi10: a situação do vencedor do Intertoto no livro da UEFA
                            core::ptr::write_unaligned((uefa + 0x36) as *mut i16, cqi10_w(uefa + 0x36).wrapping_add(1));
                            vaga = vaga.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if i >= cqi10_w(this + 0x3e) {
                            break;
                        }
                    }
                }
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi10_d(cqi10_d(uefa) + 0x5c));
                prever(uefa);
                core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
                let uefa = cqi10_uefa();
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi10_d(cqi10_d(uefa) + 0x5c));
                prever(uefa);
                let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
                return base(this, hoje, fase);
            }
            core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

// ponytail: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano, feito
// antes: se +0xb2 passa de 0 a 1 com a Copa da UEFA, o livro dela (+0x14) ganha os vencedores (cqi10_vencedores), em
// ordem e com a situação 4, e +0x36 sobe tanto quanto; senão o livro não cresce.
unsafe extern "thiscall" fn comparar_cqi10_intertoto_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1685);
    let uefa = cqi10_uefa();
    if uefa == 0 || cqi10_d(this + 0xb2) != 0 {
        return o(this, hoje, fase);
    }
    let (v, k) = cqi10_vencedores(this, uefa);
    let n36 = cqi10_w(uefa + 0x36);
    let r = o(this, hoje, fase);
    let passou = cqi10_d(this + 0xb2) == 1;
    let previsto = if passou { n36.wrapping_add(k as i16) } else { n36 };
    let mut igual = cqi10_w(uefa + 0x36) == previsto;
    if passou && igual {
        for (j, &c) in v.iter().enumerate().take(k) {
            let l = cqi10_d(uefa + 0x14).wrapping_add(((n36 as isize) + j as isize) as usize * 6);
            igual &= cqi10_d(l) == c && cqi10_b(l + 4) == 4;
        }
    }
    PLACAR.registrar(igual, || format!("cqi10_intertoto_dia({:#x}) passou={} livro {} -> {} previsto {} vencedores {:x?}", this, passou, n36, cqi10_w(uefa + 0x36), previsto, &v[..k]));
    r
}

/// { 0x633c10 } (thiscall, ret 0x18, intertoto_cup.cpp; a virtual +0x44 do Intertoto): `IntertotoCup::
/// update_staff_records_for_status(clube, desfecho, _, _, jogo, _)`; devolve 0. Peso = 0x4bf850(baixo, alto, rodada do
/// jogo +0x34, rodadas +0x3c) (cdecl, 4 pushes; add esp, 0x10).
/// - 1 (venceu a final): a classificação (0x86bdd0) para a Copa da UEFA (a competição [0xae23d0] + 0x6b × [0x9cf6f0]),
///   fase 0, rodada 0xa, peso(0x3c, 0x64), e o clube passa à Copa da UEFA (+0x1db = [0x9cf6f0]).
/// - 0 (passou de rodada): se a rodada seguinte existe (+0x34 + 1 < +0x3c), a classificação no Intertoto (+4) com a
///   fase +0x32 do jogo, o +7 da rodada seguinte (+0xa3, 0x68 bytes) e o peso rodada × 99 / (rodadas − 1) + 1 (só o
///   byte de baixo sobe: inc al).
/// - 0xff ou 2 (caiu): a eliminação (0x86c000) no Intertoto, fase 0, rodada +0x30 do jogo, peso(0x14, 0x50), e o clube
///   sai da competição europeia (+0x1db = −1).
unsafe extern "thiscall" fn cqi10_intertoto_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    match desfecho as u8 {
        1 => {
            let peso = cpz4_escala(0x3c, 0x64, cqi10_w(jogo + 0x34), cqi10_w(this + 0x3c) as u16 as u32);
            let uefa = cqi10_d(0xae23d0).wrapping_add(cqi10_d(0x9cf6f0).wrapping_mul(0x6b));
            cpz4_classificado(clube, uefa, 0, 0xa, peso);
            crate::comparacao::ramo("cqi_10 vencedor do intertoto");
            core::ptr::write_unaligned((clube + 0x1db) as *mut u32, cqi10_d(0x9cf6f0) as u32); // cqi10: o vencedor passa à UEFA
        }
        0 => {
            let r = cqi10_w(jogo + 0x34);
            let n = cqi10_w(this + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1686)(this, clube, desfecho, a3, a4, jogo, a6); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let rodada = core::ptr::read_unaligned((cqi10_d(this + 0xa3).wrapping_add((seguinte as isize * 0x68) as usize) + 7) as *const u16) as u32;
            // ponytail: a fase vai num registrador cujos 16 bits altos são os do movsx da rodada seguinte (zero); limpa
            cpz4_classificado(clube, cqi10_d(this + 4), core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, rodada, peso);
        }
        0xff | 2 => {
            let peso = cpz4_escala(0x14, 0x50, cqi10_w(jogo + 0x34), cqi10_w(this + 0x3c) as u16 as u32);
            // ponytail: a rodada +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
            cpz4_eliminado(clube, cqi10_d(this + 4), 0, core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32, peso);
            crate::comparacao::ramo("cqi_10 caiu no intertoto");
            cpz4_sair(clube); // cqi10: quem cai no Intertoto sai da competição europeia
        }
        _ => {}
    }
    0
}

// ponytail: os registros vão para o histórico das pessoas (reputação, moral): não rodam duas vezes. Só a original roda;
// o plano confere o +0x1db do clube: [0x9cf6f0] para o vencedor, −1 para quem cai, intacto nos outros.
unsafe extern "thiscall" fn comparar_cqi10_intertoto_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1686);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let antes = cqi10_d(clube + 0x1db);
    let previsto = match desfecho as u8 {
        1 => cqi10_d(0x9cf6f0),
        0xff | 2 => 0xffff_ffff,
        _ => antes,
    };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = cqi10_d(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cqi10_intertoto_registros({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, desfecho as u8, antes, depois, previsto, r));
    r
}

/// { 0x9030e0 } (thiscall, ret 0x18, uefa_cup.cpp; a virtual +0x44 da Copa da UEFA): `UefaCup::
/// update_staff_records_for_status(clube, desfecho, a3, a4, jogo, a6)`: com o desfecho (byte) 1, 2 ou 0xff (campeão,
/// vice ou eliminado) o clube sai da competição europeia (+0x1db = −1); depois os registros da base (0x519890: os seis
/// argumentos como chegaram, o desfecho como dword; mov ecx, this -> thiscall, ret 0x18), cuja devolução sai.
unsafe extern "thiscall" fn cqi10_uefa_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let d = desfecho as u8;
    if d == 1 || d == 2 || d == 0xff {
        if d == 0xff {
            crate::comparacao::ramo("cqi_10 caiu na uefa");
        }
        core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1); // cqi10: quem termina na UEFA sai da competição europeia
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x519890usize);
    base(this, clube, desfecho, a3, a4, jogo, a6)
}

// ponytail: a base grava os registros (histórico, reputação, moral) e pode dar a vaga da UEFA ao campeão: não roda duas
// vezes. Só a original roda; o plano confere o +0x1db do clube (−1 nos desfechos 1, 2 e 0xff, intacto nos outros).
unsafe extern "thiscall" fn comparar_cqi10_uefa_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1687);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let d = desfecho as u8;
    let antes = cqi10_d(clube + 0x1db);
    let previsto = if d == 1 || d == 2 || d == 0xff { 0xffff_ffff } else { antes };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = cqi10_d(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cqi10_uefa_registros({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, d, antes, depois, previsto, r));
    r
}

