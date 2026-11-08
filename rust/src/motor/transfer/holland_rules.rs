use super::*;



/// Constructor of a nation's transfer rules: TransferRules base with the caller's 5 arguments, the nation's vtable and
/// its transfer windows.
///
/// [pt] O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn holland_rules_construct_nation_transfer_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    set_transfer_windows(this, n, t, arquivo, linha);
}


/// As janelas da Holanda: uma janela, de 8 de junho (andando para o sábado [inferência]) a 4 de abril (andando para a sexta-feira [inferência]).
pub(crate) const CQM1_HOLANDA: [[u8; 6]; 2] = [
    [0x0c, 0x00, 0x06, 0x08, 0x05, 0x01], // cqm1: a Holanda
    [0x0c, 0x00, 0x05, 0x04, 0x03, 0x00],
];

/// the Holland rule (12): one window, from 8 June (moved to the Saturday [inference]) to 4 April (moved to the Friday
/// [inference]); +0x13 = 50 (base default 50) [inference: the foreign limit]; +0x14 = 50 (default 30)
/// Original: `HOLLAND_RULES::HOLLAND_RULES` (0x5f9690, holland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5f9690 } (thiscall, ret 0x14, holland_rules.cpp, `HOLLAND_RULES::HOLLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Holanda (regra 12): a base (0x8e84a0), a vtable 0x96ba88, 1 janela(s) de `CQM1_HOLANDA`; o +0x13 = 50 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 50 (o padrão é 30). Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_holland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 holanda");
    holland_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96ba88, 1, &CQM1_HOLANDA, 0x9cf04c, 0x52);
    *((this + 0x13) as *mut u8) = 50;
    *((this + 0x14) as *mut u8) = 50;
    this
}

/// Comparer for `construct_holland_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_holland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_HOLLAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_holland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_holland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Croácia: uma janela, de 1º de junho a 30 de março.
pub(crate) const CQM1_CROACIA: [[u8; 6]; 2] = [
    [0x05, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm1: a Croácia
    [0x05, 0x00, 0xff, 0x1e, 0x02, 0x00],
];

/// the Croatia rule (5): one window, from 1 June to 30 March
/// Original: `CROATIA_RULES::CROATIA_RULES` (0x518500, croatia_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x518500 } (thiscall, ret 0x14, croatia_rules.cpp, `CROATIA_RULES::CROATIA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Croácia (regra 5): a base (0x8e84a0), a vtable 0x969394, 1 janela(s) de `CQM1_CROACIA`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_croatia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 croacia");
    holland_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x969394, 1, &CQM1_CROACIA, 0x9b5570, 0x4e);
    this
}

/// Comparer for `construct_croatia_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_croatia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_CROATIA_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_croatia_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_croatia_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Irlanda: uma janela, de 10 de maio a 31 de janeiro.
pub(crate) const CQM1_IRLANDA: [[u8; 6]; 2] = [
    [0x0d, 0x00, 0xff, 0x0a, 0x04, 0x01], // cqm1: a Irlanda
    [0x0d, 0x00, 0xff, 0x1f, 0x00, 0x00],
];

/// the Ireland rule (13): one window, from 10 May to 31 January
/// Original: `IRELAND_RULES::IRELAND_RULES` (0x63c210, ireland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x63c210 } (thiscall, ret 0x14, ireland_rules.cpp, `IRELAND_RULES::IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda (regra 13): a base (0x8e84a0), a vtable 0x96c190, 1 janela(s) de `CQM1_IRLANDA`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_1 irlanda");
    holland_rules_construct_nation_transfer_rules(this, [a1, a2, a3, a4, a5], 0x96c190, 1, &CQM1_IRLANDA, 0x9ea12c, 0x4e);
    this
}

/// Comparer for `construct_ireland_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] A original no objeto de quem chama, a cópia num rascunho (ver `compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn comparar_construct_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_IRELAND_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_ireland_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_ireland_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

