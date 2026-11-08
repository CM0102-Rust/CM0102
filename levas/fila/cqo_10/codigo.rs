// ---------- %%LEVA%%a leva: o calendário das copas italianas: as rodadas da Copa da Itália, da Copa da Itália da Série C e das duas Supercopas (ita_cup.cpp, ita_c_cup.cpp, ita_super.cpp, ita_c1_super.cpp) ----------

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
type Cqo10G = (u32, u32, u32, u32);

/// As duas fases das copas italianas (a virtual +0x3c, `create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret
/// 0x10; a fase lida como byte com sinal). Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1, rodadas
/// = o número de rodadas de grupo, extra = fase + `id` (16 bits); a tabela de 0x41 bytes por rodada (0x944e46; sem
/// memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0, rodadas =
/// o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as rodadas pela
/// `eng_cc_cup_write_cup_round` com o ano +0x40 da copa (com sinal). Acima de `limite`: devolve 0 sem gravar nada.
unsafe fn create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limit: i8, id: i16, grupos: &[Cqo10G],
    mata: &[Cql10R], arquivo: usize, linha_g: u32, linha_m: u32, nome_g: &'static str, nome: &'static str) -> usize {
    let f = fase as u8 as i8;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if f < limit {
        let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
        let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
        if !liga.is_null() {
            *liga = 1;
        }
        *rodadas = grupos.len() as i16;
        *extra = (f as i16).wrapping_add(id);
        let t = alocar(grupos.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(arquivo, linha_g);
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
    if f != limit {
        return 0;
    }
    if !liga.is_null() {
        *liga = 0;
    }
    *rodadas = mata.len() as i16;
    *extra = 0;
    let t = alocar(mata.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(arquivo, linha_m);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in mata.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, ano);
    }
    t
}

/// Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
unsafe fn written_group_schedule_bytes(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (de grupos ou de mata-mata, pelo é_liga
/// da cópia) têm de ser iguais; a tabela da cópia é liberada (0x9452ca).
unsafe fn ita_cup_compare_schedule(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize, limit: i8) -> usize {
    if fase as u8 as i8 > limit {
        // ponytail: acima do mata-mata nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let grupos = (fase as u8 as i8) < limit;
    let tabela = if a != 0 && b != 0 {
        if grupos { written_group_schedule_bytes(a, *rodadas) == written_group_schedule_bytes(b, r2) } else { eng_cc_cup_written_round_bytes(a, *rodadas) == eng_cc_cup_written_round_bytes(b, r2) }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Itália, grupos: três rodadas em agosto (12 e 19, sábado [inferência: dia da semana 6], e 29, terça [2]).
const CQO10_COPA_DA_ITALIA_GRUPOS: [Cqo10G; 3] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x1d, 7, 2, 2)];

/// A Copa da Itália, mata-mata: cinco rodadas em ida e volta, de setembro a março do ano seguinte.
const CQO10_COPA_DA_ITALIA: [Cql10R; 5] = [
    cql10r((0x1e, 7, 0, 3), (0x13, 8, 0, 2, 2, 0), 0xa, (0, 0x400, 3), 4, (0x10, 8, 0x10, 0), (0, 2, 0x23), (0, 0xffff_ffff, 0)), // cqo10: a 1ª rodada do mata-mata da Copa da Itália
    cql10r((0x19, 9, 0, 3), (0xb, 0xa, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 4, (0x10, 8, 8, 0x10), (0, 2, 0x11), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 0xa, 0, 3), (0xc, 0xb, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0x1c), (0, 0xffff_ffff, 0)),
    cql10r((0xa, 0, 1, 3), (0x17, 0, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((7, 1, 1, 3), (6, 2, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 0x39), (0, 0xffff_ffff, 0)),
];

/// { 0x63ec50 } (thiscall, ret 0x10, ita_cup.cpp; a virtual +0x3c da Copa da Itália): `ItaCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (extra = fase + 0x3e9; a caixa da linha 0xc1), o mata-mata na
/// fase 7 (a caixa da linha 0xe1).
unsafe extern "thiscall" fn ita_cup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_schedule(this, fase, rodadas, extra, liga, 7, 0x3e9, &CQO10_COPA_DA_ITALIA_GRUPOS, &CQO10_COPA_DA_ITALIA, 0x9ea23c, 0xc1, 0xe1,
        "cqo_10 grupos da copa da italia", "cqo_10 mata-mata da copa da italia")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    ita_cup_compare_schedule("ita_cup_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| ita_cup_create_schedule(this, fase, r, e, l), 7)
}

/// A Copa da Itália da Série C, grupos: cinco rodadas em agosto (12, 19, 22, 26 e 29).
const CQO10_COPA_C_GRUPOS: [Cqo10G; 5] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x16, 7, 2, 2), (0x1a, 7, 6, 1), (0x1d, 7, 2, 2)]; // cqo10: os grupos da Copa da Série C

/// A Copa da Itália da Série C, mata-mata: cinco rodadas em ida e volta, de setembro a maio do ano seguinte (as
/// semifinais em abril).
const CQO10_COPA_C: [Cql10R; 5] = [
    cql10r((0x1e, 7, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x14, (0, 0, 3), 4, (0x20, 0x10, 0x20, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 9, 0, 3), (1, 0xa, 0, 2, 2, 0), 0x1e, (0, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0xf, 0xa, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0xa, 0, 3), (0xb, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
];

/// { 0x63cf90 } (thiscall, ret 0x10, ita_c_cup.cpp; a virtual +0x3c da Copa da Série C): `ItaCCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 0xf (extra = fase + 0x420; a caixa da linha 0xbc), o mata-mata na
/// fase 0xf (a caixa da linha 0xe3).
unsafe extern "thiscall" fn ita_cup_create_schedule_63cf90(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_schedule(this, fase, rodadas, extra, liga, 0xf, 0x420, &CQO10_COPA_C_GRUPOS, &CQO10_COPA_C, 0x9ea204, 0xbc, 0xe3,
        "cqo_10 grupos da copa da serie c", "cqo_10 mata-mata da copa da serie c")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_schedule_63cf90(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    ita_cup_compare_schedule("ita_cup_create_schedule_63cf90", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| ita_cup_create_schedule_63cf90(this, fase, r, e, l), 0xf)
}

/// A Supercopa da Itália: um jogo único em 19 de agosto (sorteio em 5 de julho), mando neutro [inferência: o +0x13 da
/// data fica 0], sem prêmios.
const CQO10_SUPERCOPA: [Cql10R; 1] = [
    cql10r((5, 6, 0, 5), (0x13, 7, 0, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqo10: a Supercopa da Itália
];

/// { 0x65f920 } (thiscall, ret 0x10, ita_super.cpp; a virtual +0x3c da Supercopa da Itália): `ItaSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
unsafe extern "thiscall" fn ita_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &CQO10_SUPERCOPA, 0x9eac34, 0x99, "cqo_10 tabela da supercopa")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    eng_cc_cup_compare_round_table("ita_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| ita_cup_create_round_table(this, fase, r, e, s))
}

/// A Supercopa da Série C1: um jogo único em 21 de maio do ano seguinte (sorteio em 15 de maio), sem prêmios.
const CQO10_SUPERCOPA_C1: [Cql10R; 1] = [
    cql10r((0xf, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)),
];

/// { 0x63c890 } (thiscall, ret 0x10, ita_c1_super.cpp; a virtual +0x3c da Supercopa da Série C1): `ItaC1Super::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
unsafe extern "thiscall" fn ita_cup_create_round_table_63c890(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &CQO10_SUPERCOPA_C1, 0x9ea1c8, 0x99, "cqo_10 tabela da supercopa c1")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_round_table_63c890(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I3%%);
    eng_cc_cup_compare_round_table("ita_cup_create_round_table_63c890", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| ita_cup_create_round_table_63c890(this, fase, r, e, s))
}
