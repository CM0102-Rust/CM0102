use super::*;



/// Constructor of a nation's transfer rules: base 0x8e84a0 with the caller's 5 arguments, the nation's vtable and the
/// transfer windows.
///
/// [pt] O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn argentina_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    set_transfer_windows(this, n, t, arquivo, linha);
}


/// As janelas da Argentina: duas janelas, de 24 de julho (andando para o domingo [inferência]) a 6 de agosto (andando para a quarta-feira [inferência]); de 14 de dezembro (andando para o domingo [inferência]) a 4 de março (andando para a quarta-feira [inferência]).
pub(crate) const ARGENTINA_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x01, 0x00, 0x00, 0x18, 0x06, 0x01],
    [0x01, 0x00, 0x03, 0x06, 0x07, 0x00], // cqn1: a Argentina
    [0x01, 0x01, 0x00, 0x0e, 0x0b, 0x01],
    [0x01, 0x01, 0x03, 0x04, 0x02, 0x00],
];

/// the Argentina rule (1): two windows, from 24 July (moved to the Sunday [inference]) to 6 August (moved to the
/// Wednesday [inference]); from 14 December (moved to the Sunday [inference]) to 4 March (moved to the Wednesday
/// [inference])
/// Original: `ARGENTINA_RULES::init_transfer_windows` (0x40a620, argentina_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x40a620 } (thiscall, ret 0, argentina_rules.cpp, `ARGENTINA_RULES::init_transfer_windows`; chamada pelo construtor das regras da Argentina (regra 1), depois da base e da vtable): as
/// 2 janelas de transferência de `ARGENTINA_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn argentina_rules_init_transfer_windows(this: usize) {
    crate::comparacao::ramo("cqn_1 argentina");
    set_transfer_windows(this, 2, &ARGENTINA_TRANSFER_WINDOWS, 0x987960, 0x50);
}

/// Comparer for `argentina_rules_init_transfer_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_argentina_rules_init_transfer_windows(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ARGENTINA_RULES_INIT_TRANSFER_WINDOWS);
    compare_transfer_rules_ctor("argentina_rules_init_transfer_windows", this, || o(this), |s| argentina_rules_init_transfer_windows(s));
}

/// As janelas de Portugal: duas janelas, de 1º de junho a 1º de setembro; de 15 de dezembro a 15 de janeiro.
pub(crate) const PORTUGAL_TRANSFER_WINDOWS: [[u8; 6]; 4] = [
    [0x13, 0x00, 0xff, 0x01, 0x05, 0x01],
    [0x13, 0x00, 0xff, 0x01, 0x08, 0x00], // cqn1: a Portugal
    [0x13, 0x01, 0xff, 0x0f, 0x0b, 0x01],
    [0x13, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// the Portugal rule (19): two windows, from 1 June to 1 September; from 15 December to 15 January
/// Original: `PORTUGAL_RULES::init_transfer_windows` (0x7d3df0, portugal_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x7d3df0 } (thiscall, ret 0, portugal_rules.cpp, `PORTUGAL_RULES::init_transfer_windows`; chamada pelo construtor das regras de Portugal (regra 19), depois da base e da vtable): as
/// 2 janelas de transferência de `PORTUGAL_TRANSFER_WINDOWS`.
pub(crate) unsafe extern "thiscall" fn argentina_rules_init_transfer_windows_7d3df0(this: usize) {
    crate::comparacao::ramo("cqn_1 portugal");
    set_transfer_windows(this, 2, &PORTUGAL_TRANSFER_WINDOWS, 0xa78844, 0x4f);
}

/// Comparer for `argentina_rules_init_transfer_windows_7d3df0` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_argentina_rules_init_transfer_windows_7d3df0(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ARGENTINA_RULES_INIT_TRANSFER_WINDOWS_7D3DF0);
    compare_transfer_rules_ctor("argentina_rules_init_transfer_windows_7d3df0", this, || o(this), |s| argentina_rules_init_transfer_windows_7d3df0(s));
}

/// As janelas da Coreia do Sul: uma janela, de 25 de novembro (andando para o domingo [inferência]) a 25 de julho.
pub(crate) const SOUTH_KOREA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x1b, 0x00, 0x00, 0x19, 0x0a, 0x01],
    [0x1b, 0x00, 0xff, 0x19, 0x06, 0x00], // cqn1: a Coreia do Sul
];

/// the South Korea rule (27): one window, from 25 November (moved to the Sunday [inference]) to 25 July; +0x13 = 7
/// (base default 50) [inference: the foreign limit]
/// Original: `KOREA_RULES::KOREA_RULES` (0x66e630, korea_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x66e630 } (thiscall, ret 0x14, korea_rules.cpp, `KOREA_RULES::KOREA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Coreia do Sul (regra 27): a base (0x8e84a0), a vtable 0x96cfec, 1 janela(s) de `SOUTH_KOREA_TRANSFER_WINDOWS`; o +0x13 = 7 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_korea_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqn_1 coreia");
    argentina_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96cfec, 1, &SOUTH_KOREA_TRANSFER_WINDOWS, 0x9eb20c, 0x51);
    *((this + 0x13) as *mut u8) = 7;
    this
}

/// Comparer for `construct_korea_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_korea_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_KOREA_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_korea_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_korea_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

