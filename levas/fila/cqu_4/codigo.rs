// ---------- %%LEVA%%a leva: os calendários MLS, J2 (as datas e os jogos de cada rodada da fase de pontos corridos e do mata-mata) (usa_mls.cpp, jap_j2.cpp) ----------

/// A MLS, a liga: 12 rodadas (da 29/fev à 30/jul), 780 bytes, 24 operações.
const CQU4_MLS_0: [Cqp11Op; 24] = [
    Cqp11D(0, 0x1d, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 3, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 3, 4, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x11, 4, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x1f, 4, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0xa, 5, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x15, 5, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xc, 6, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 2, 7, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x10, 7, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x14, 7, 0, 2, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1e, 7, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
];

/// A MLS, a liga: 22 rodadas (da 22/fev à 6/ago), 1430 bytes, 44 operações.
const CQU4_MLS_1: [Cqp11Op; 44] = [
    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 5, 3, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 3, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x1a, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0xa, 4, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x16, 4, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x18, 4, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1c, 4, 0, 2, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 7, 5, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0xe, 5, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x1c, 5, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 2, 6, 0, 2, 2, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 6, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0xa, 6, 0, 2, 2, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x13, 6, 0, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x17, 6, 0, 2, 2, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x1a, 6, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0x1d, 6, 0, 2, 2, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 6, 7, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 9, 7, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x17, 7, 0, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 6, 8, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0),
];

/// A MLS, a liga: 32 rodadas (da 22/fev à 6/ago), 2080 bytes, 64 operações.
const CQU4_MLS_2: [Cqp11Op; 64] = [
    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x1d, 2, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 5, 3, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0xc, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x13, 3, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x1a, 3, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 3, 4, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xa, 4, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x11, 4, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x18, 4, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1c, 4, 0, 2, 2, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1f, 4, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 7, 5, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0xa, 5, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0xe, 5, 0, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x15, 5, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0x1c, 5, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 2, 6, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 6, 6, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0xc, 6, 0, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0x13, 6, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x1a, 6, 0, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x1d, 6, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 2, 7, 0, 5, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 6, 7, 0, 2, 2, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 9, 7, 0, 5, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0x10, 7, 0, 5, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0x14, 7, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x17, 7, 0, 5, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x1e, 7, 0, 5, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 6, 8, 0, 5, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0),
];

/// A MLS, a fase 2: 2 rodadas (da 7/ago à 30/ago), 208 bytes, 38 operações.
const CQU4_MLS_3: [Cqp11Op; 38] = [
    Cqp11S(0, 7, 8, 0, 6), Cqp11J(0, 0x10, 8, 0, 5, 1, 0), Cqp11W(0x7, 0x78), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x230), Cqp11B(0x17, 5), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x30),
    Cqp11W(0x11, 0x20), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 0x15, 8, 0, 3), Cqp11J(1, 0x1e, 8, 0, 5, 1, 0), Cqp11W(0x6f, 0x82),
    Cqp11W(0x71, 1), Cqp11W(0x73, 0), Cqp11W(0x75, 0x230), Cqp11B(0x7f, 5), Cqp11W(0x80, 4), Cqp11W(0x82, 2),
    Cqp11W(0x84, 0), Cqp11W(0x77, 0x30), Cqp11W(0x79, 0x20), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 7), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0), Cqp11X(0xcc, 0),
];

/// A MLS, a fase 3: 1 rodadas (da 8/set à 15/set), 104 bytes, 18 operações.
const CQU4_MLS_4: [Cqp11Op; 18] = [
    Cqp11S(0, 8, 9, 0, 6), Cqp11J(0, 0xf, 9, 0, 6, 2, 4), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11W(0x7, 0x96),
    Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x220), Cqp11B(0x17, 5), Cqp11W(0x1a, 1), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0),
];

/// As fases da tabela de rodadas da MLS: a fase -3 (é liga): 12 rodadas, da 29/fev à 30/jul; a fase -2 (é liga): 22 rodadas, da 22/fev à 6/ago; a fase de pontos corridos (−1) (é liga): 32 rodadas, da 22/fev à 6/ago; a fase 2 (mata-mata ou fase final): 2 rodadas, da 7/ago à 30/ago; a fase 3 (mata-mata ou fase final): 1 rodadas, da 8/set à 15/set.
fn cqu4_mls_fase(fase: u32) -> Option<Cqp11Fase> {
    match fase as u8 as i8 {
        -3 => Some(Cqp11Fase { liga: 1, rodadas: 12, extra: 0, extra_checa: false, tamanho: 0x30c, linha: 0xb3,
            ops: &CQU4_MLS_0, nome: "cqu_4 mls liga" }),
        -2 => Some(Cqp11Fase { liga: 1, rodadas: 22, extra: 0, extra_checa: false, tamanho: 0x596, linha: 0xeb,
            ops: &CQU4_MLS_1, nome: "cqu_4 mls liga" }),
        -1 => Some(Cqp11Fase { liga: 1, rodadas: 32, extra: 0, extra_checa: false, tamanho: 0x820, linha: 0x141,
            ops: &CQU4_MLS_2, nome: "cqu_4 mls liga" }),
        2 => Some(Cqp11Fase { liga: 0, rodadas: 2, extra: 0xa0, extra_checa: false, tamanho: 0xd0, linha: 0x1b6,
            ops: &CQU4_MLS_3, nome: "cqu_4 mls fase 2" }),
        3 => Some(Cqp11Fase { liga: 0, rodadas: 1, extra: 0x417, extra_checa: false, tamanho: 0x68, linha: 0x1f2,
            ops: &CQU4_MLS_4, nome: "cqu_4 mls fase 3" }),
        _ => None,
    }
}

/// { 0x9072d0 } (thiscall, ret 0x10, usa_mls.cpp; a virtual +0x3c): `UsaMls::create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase
/// de `cqu4_mls_fase` (a máquina da cqp_11: as saídas, a tabela de 0x41 bytes por rodada e as datas e jogos de cada rodada).
unsafe extern "thiscall" fn cqu4_calendario_mls(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqu4_mls_fase(fase), rodadas, extra, liga, 0xaa1fb4)
}

unsafe extern "thiscall" fn comparar_cqu4_calendario_mls(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cqp11_comparar("cqu4_calendario_mls", fase, cqu4_mls_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqu4_calendario_mls(this, fase, r, e, l))
}

/// A J2, a liga: 44 rodadas (da 12/fev à 26/out), 2860 bytes, 88 operações.
const CQU4_J2_0: [Cqp11Op; 88] = [
    Cqp11D(0, 0xc, 2, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 2, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x1a, 2, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 2, 3, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 9, 3, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x10, 3, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x17, 3, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1b, 3, 0, 3, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1f, 3, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 3, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 7, 4, 0, 6, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0xe, 4, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x15, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0x1b, 4, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 5, 0, 6, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 2, 6, 0, 6, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 9, 6, 0, 6, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0xf, 6, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 0x1e, 6, 0, 6, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 2, 7, 0, 2, 2, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 6, 7, 0, 6, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0xd, 7, 0, 6, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x14, 7, 0, 6, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x17, 7, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 0x1b, 7, 0, 6, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 3, 8, 0, 6, 1, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 6, 8, 0, 2, 2, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0xa, 8, 0, 6, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0xd, 8, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x11, 8, 0, 6, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x18, 8, 0, 6, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 1, 9, 0, 6, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0), Cqp11D(0x20, 4, 9, 0, 2, 2, 0), Cqp11L(0x20, 0, -1, -1, -1, 0),
    Cqp11D(0x21, 7, 9, 0, 5, 1, 0), Cqp11L(0x21, 0, -1, -1, -1, 0), Cqp11D(0x22, 0xb, 9, 0, 2, 2, 0),
    Cqp11L(0x22, 0, -1, -1, -1, 0), Cqp11D(0x23, 0xf, 9, 0, 6, 1, 0), Cqp11L(0x23, 0, -1, -1, -1, 0),
    Cqp11D(0x24, 0x16, 9, 0, 6, 1, 0), Cqp11L(0x24, 0, -1, -1, -1, 0), Cqp11D(0x25, 0x1d, 9, 0, 6, 1, 0),
    Cqp11L(0x25, 0, -1, -1, -1, 0), Cqp11D(0x26, 1, 0xa, 0, 2, 2, 0), Cqp11L(0x26, 0, -1, -1, -1, 0),
    Cqp11D(0x27, 5, 0xa, 0, 6, 1, 0), Cqp11L(0x27, 0, -1, -1, -1, 0), Cqp11D(0x28, 8, 0xa, 0, 2, 2, 0),
    Cqp11L(0x28, 0, -1, -1, -1, 0), Cqp11D(0x29, 0xc, 0xa, 0, 6, 1, 0), Cqp11L(0x29, 0, -1, -1, -1, 0),
    Cqp11D(0x2a, 0x13, 0xa, 0, 6, 1, 0), Cqp11L(0x2a, 0, -1, -1, -1, 0), Cqp11D(0x2b, 0x1a, 0xa, 0, 6, 1, 0),
    Cqp11L(0x2b, 0, -1, -1, -1, 0),
];

/// As fases da tabela de rodadas da J2: a fase de pontos corridos (−1) (é liga): 44 rodadas, da 12/fev à 26/out.
fn cqu4_j2_fase(fase: u32) -> Option<Cqp11Fase> {
    match fase as u8 as i8 {
        -1 => Some(Cqp11Fase { liga: 1, rodadas: 44, extra: 0, extra_checa: false, tamanho: 0xb2c, linha: 0x9d,
            ops: &CQU4_J2_0, nome: "cqu_4 j2 liga" }),
        _ => None,
    }
}

/// { 0x664780 } (thiscall, ret 0x10, jap_j2.cpp; a virtual +0x3c): `JapJ2::create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase
/// de `cqu4_j2_fase` (a máquina da cqp_11: as saídas, a tabela de 0x41 bytes por rodada e as datas e jogos de cada rodada).
unsafe extern "thiscall" fn cqu4_calendario_j2(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqu4_j2_fase(fase), rodadas, extra, liga, 0x9eaedc)
}

unsafe extern "thiscall" fn comparar_cqu4_calendario_j2(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    cqp11_comparar("cqu4_calendario_j2", fase, cqu4_j2_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqu4_calendario_j2(this, fase, r, e, l))
}
