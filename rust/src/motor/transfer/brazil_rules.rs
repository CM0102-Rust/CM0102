use super::*;

// ---------- 273a leva: as regras de transferência do Brasil, da Alemanha e da Escócia: as janelas de transferência (brazil_rules.cpp, germany_rules.cpp, scotland_rules.cpp) ----------



/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn cqk3_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk1_janelas(this, n, t, arquivo, linha);
}


/// A janela do Brasil: de 11 de dezembro (andando para a quarta-feira [inferência]) a 8 de novembro (andando para a sexta-feira [inferência]).
pub(crate) const CQK3_BRASIL: [[u8; 6]; 2] = [
    [0x04, 0x00, 0x03, 0x0b, 0x0b, 0x01], // cqk3: o Brasil abre em dezembro
    [0x04, 0x00, 0x05, 0x08, 0x0a, 0x00], // cqk3: o Brasil fecha em novembro
];

/// the Brazilian rule (4): one window, from 11 December (moved to the Wednesday [inference]) to 8 November (moved to
/// the Friday [inference]): closed only from November to December; +0x13 = 3 (default 50) [inference: the foreign
/// signing limit]
/// Original: `BRAZIL_RULES::BRAZIL_RULES` (0x43f0e0, brazil_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x43f0e0 } (thiscall, ret 0x14, brazil_rules.cpp, `BRAZIL_RULES::BRAZIL_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência do Brasil (regra 4): a base (0x8e84a0), a vtable 0x968838, 1 janela(s) de `CQK3_BRASIL` e o +0x13 = 3 (o padrão da base é 50) [inferência: o limite de estrangeiros contratados]. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn regras_brasil(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_3 brasil");
    cqk3_construir(this, [a1, a2, a3, a4, a5], 0x968838, 1, &CQK3_BRASIL, 0x98b104, 0x50);
    *((this + 0x13) as *mut u8) = 3; // cqk3: o limite do Brasil
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_regras_brasil(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1682);
    let mut r = 0usize;
    cqk1_comparar("regras_brasil", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_brasil(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Alemanha: de 1º de julho a 15 de agosto e de 1º de julho a 15 de janeiro.
pub(crate) const CQK3_ALEMANHA: [[u8; 6]; 4] = [
    [0x0a, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x00, 0xff, 0x0f, 0x07, 0x00],
    [0x0a, 0x01, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// the German rule (10): two windows, from 1 July to 15 August and from 1 July to 15 January [inference: the second for
/// free agents]
/// Original: `GERMANY_RULES::init_transfer_windows` (0x5dffe0, germany_rules.cpp, thiscall, ret 0).
///
/// [pt] { 0x5dffe0 } (thiscall, ret 0, germany_rules.cpp, `GERMANY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Alemanha (regra 10), depois da base e da vtable): as
/// 2 janelas de transferência de `CQK3_ALEMANHA`.
pub(crate) unsafe extern "thiscall" fn janelas_alemanha(this: usize) {
    crate::comparacao::ramo("cqk_3 alemanha");
    cqk1_janelas(this, 2, &CQK3_ALEMANHA, 0x9cd538, 0x4d);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
pub(crate) unsafe extern "thiscall" fn comparar_janelas_alemanha(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1683);
    cqk1_comparar("janelas_alemanha", this, || o(this), |s| janelas_alemanha(s));
}

/// A janela da Escócia: de 30 de maio (andando para o sábado [inferência]) a 30 de março (andando para a quinta-feira [inferência]).
pub(crate) const CQK3_ESCOCIA: [[u8; 6]; 2] = [
    [0x15, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x15, 0x00, 0x04, 0x1e, 0x02, 0x00],
];



