use super::*;



/// Constructor of a nation's transfer rules: base 0x8e84a0 with the caller's 5 arguments, the nation's vtable and the
/// transfer windows.
///
/// [pt] O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn turkey_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    set_transfer_windows(this, n, t, arquivo, linha);
}


/// As janelas da Turquia: uma janela, de 1º de junho a 31 de janeiro.
pub(crate) const TURKEY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia
    [0x18, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// the Turkey rule (24): one window, from 1 June to 31 January; +0x13 = 8 (base default 50) [inference: the foreign
/// limit]; +0x14 = 8 (default 30)
/// Original: `TURKEY_RULES::TURKEY_RULES` (0x9020a0, turkey_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x9020a0 } (thiscall, ret 0x14, turkey_rules.cpp, `TURKEY_RULES::TURKEY_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Turquia (regra 24): a base (0x8e84a0), a vtable 0x97075c, 1 janela(s) de `TURKEY_TRANSFER_WINDOWS`; o +0x13 = 8 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 8 (o padrão é 30). Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_turkey_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 turquia");
    turkey_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x97075c, 1, &TURKEY_TRANSFER_WINDOWS, 0xaa19a8, 0x52);
    *((this + 0x13) as *mut u8) = 8;
    *((this + 0x14) as *mut u8) = 8;
    this
}

/// Comparer for `construct_turkey_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_turkey_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_TURKEY_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_turkey_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_turkey_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Polônia: uma janela, de 1º de julho a 20 de fevereiro.
pub(crate) const POLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x12, 0x00, 0xff, 0x01, 0x06, 0x01], // cqm3: a Polônia
    [0x12, 0x00, 0xff, 0x14, 0x01, 0x00],
];

/// the Poland rule (18): one window, from 1 July to 20 February; +0x13 = 5 (base default 50) [inference: the foreign
/// limit]; +0x14 = 5 (default 30)
/// Original: `POLAND_RULES::POLAND_RULES` (0x7cd830, poland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7cd830 } (thiscall, ret 0x14, poland_rules.cpp, `POLAND_RULES::POLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Polônia (regra 18): a base (0x8e84a0), a vtable 0x96e61c, 1 janela(s) de `POLAND_TRANSFER_WINDOWS`; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30). Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 polonia");
    turkey_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96e61c, 1, &POLAND_TRANSFER_WINDOWS, 0xa786b8, 0x51);
    *((this + 0x13) as *mut u8) = 5;
    *((this + 0x14) as *mut u8) = 5;
    this
}

/// Comparer for `construct_poland_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_POLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_poland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_poland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Finlândia: uma janela, de 16 de novembro a 15 de agosto.
pub(crate) const FINLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x08, 0x00, 0xff, 0x10, 0x0a, 0x01],
    [0x08, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqm3: a Finlândia
];

/// the Finland rule (8): one window, from 16 November to 15 August; dword +0x15 = 1
/// Original: `FINLAND_RULES::FINLAND_RULES` (0x5a9950, finland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5a9950 } (thiscall, ret 0x14, finland_rules.cpp, `FINLAND_RULES::FINLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Finlândia (regra 8): a base (0x8e84a0), a vtable 0x96a75c, 1 janela(s) de `FINLAND_TRANSFER_WINDOWS`; o dword +0x15 = 1. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_finland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 finlandia");
    turkey_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96a75c, 1, &FINLAND_TRANSFER_WINDOWS, 0x9cc134, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// Comparer for `construct_finland_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_finland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_FINLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_finland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_finland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

