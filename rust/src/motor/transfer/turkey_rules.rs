use super::*;

// ---------- 277a leva: as regras de transferência da Turquia, da Polônia e da Finlândia: as janelas de transferência (finland_rules.cpp, poland_rules.cpp, turkey_rules.cpp) ----------



/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn cqm3_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk1_janelas(this, n, t, arquivo, linha);
}


/// As janelas da Turquia: uma janela, de 1º de junho a 31 de janeiro.
pub(crate) const CQM3_TURQUIA: [[u8; 6]; 2] = [
    [0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia
    [0x18, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// the Turkey rule (24): one window, from 1 June to 31 January; +0x13 = 8 (base default 50) [inference: the foreign
/// limit]; +0x14 = 8 (default 30)
/// Original: `TURKEY_RULES::TURKEY_RULES` (0x9020a0, turkey_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x9020a0 } (thiscall, ret 0x14, turkey_rules.cpp, `TURKEY_RULES::TURKEY_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Turquia (regra 24): a base (0x8e84a0), a vtable 0x97075c, 1 janela(s) de `CQM3_TURQUIA`; o +0x13 = 8 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 8 (o padrão é 30). Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_turquia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 turquia");
    cqm3_construir(this, [a1, a2, a3, a4, a5], 0x97075c, 1, &CQM3_TURQUIA, 0xaa19a8, 0x52);
    *((this + 0x13) as *mut u8) = 8;
    *((this + 0x14) as *mut u8) = 8;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_turquia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1813);
    let mut r = 0usize;
    cqk1_comparar("regras_turquia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_turquia(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Polônia: uma janela, de 1º de julho a 20 de fevereiro.
pub(crate) const CQM3_POLONIA: [[u8; 6]; 2] = [
    [0x12, 0x00, 0xff, 0x01, 0x06, 0x01], // cqm3: a Polônia
    [0x12, 0x00, 0xff, 0x14, 0x01, 0x00],
];

/// the Poland rule (18): one window, from 1 July to 20 February; +0x13 = 5 (base default 50) [inference: the foreign
/// limit]; +0x14 = 5 (default 30)
/// Original: `POLAND_RULES::POLAND_RULES` (0x7cd830, poland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7cd830 } (thiscall, ret 0x14, poland_rules.cpp, `POLAND_RULES::POLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Polônia (regra 18): a base (0x8e84a0), a vtable 0x96e61c, 1 janela(s) de `CQM3_POLONIA`; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30). Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_polonia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 polonia");
    cqm3_construir(this, [a1, a2, a3, a4, a5], 0x96e61c, 1, &CQM3_POLONIA, 0xa786b8, 0x51);
    *((this + 0x13) as *mut u8) = 5;
    *((this + 0x14) as *mut u8) = 5;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_polonia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1814);
    let mut r = 0usize;
    cqk1_comparar("regras_polonia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_polonia(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Finlândia: uma janela, de 16 de novembro a 15 de agosto.
pub(crate) const CQM3_FINLANDIA: [[u8; 6]; 2] = [
    [0x08, 0x00, 0xff, 0x10, 0x0a, 0x01],
    [0x08, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqm3: a Finlândia
];

/// the Finland rule (8): one window, from 16 November to 15 August; dword +0x15 = 1
/// Original: `FINLAND_RULES::FINLAND_RULES` (0x5a9950, finland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5a9950 } (thiscall, ret 0x14, finland_rules.cpp, `FINLAND_RULES::FINLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Finlândia (regra 8): a base (0x8e84a0), a vtable 0x96a75c, 1 janela(s) de `CQM3_FINLANDIA`; o dword +0x15 = 1. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_finlandia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_3 finlandia");
    cqm3_construir(this, [a1, a2, a3, a4, a5], 0x96a75c, 1, &CQM3_FINLANDIA, 0x9cc134, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_finlandia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1815);
    let mut r = 0usize;
    cqk1_comparar("regras_finlandia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_finlandia(s, a1, a2, a3, a4, a5);
    });
    r
}

