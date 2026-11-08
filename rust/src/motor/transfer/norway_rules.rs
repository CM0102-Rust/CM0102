use super::*;



/// Constructs a nation's transfer rules: base TRANSFER_RULES ctor, the nation vtable and the transfer windows.
///
/// [pt] O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
pub(crate) unsafe fn construct_nation_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TRANSFER_RULES_TRANSFER_RULES);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    set_transfer_windows(this, n, t, arquivo, linha);
}


/// As janelas da Noruega: uma janela, de 1º de setembro a 31 de agosto.
pub(crate) const NORWAY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x11, 0x00, 0xff, 0x01, 0x08, 0x01],
    [0x11, 0x00, 0xff, 0x1f, 0x07, 0x00], // cqm2: a Noruega
];

/// the Norway rule (17): one window, from 1 September to 31 August
/// Original: `NORWAY_RULES::NORWAY_RULES` (0x7944f0, norway_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7944f0 } (thiscall, ret 0x14, norway_rules.cpp, `NORWAY_RULES::NORWAY_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Noruega (regra 17): a base (0x8e84a0), a vtable 0x96dfe4, 1 janela(s) de `NORWAY_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_norway_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_2 noruega");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96dfe4, 1, &NORWAY_TRANSFER_WINDOWS, 0xa69870, 0x51);
    this
}


/// As janelas da Irlanda do Norte: uma janela, de 6 de maio (andando para o sábado [inferência]) a 20 de março.
pub(crate) const NORTHERN_IRELAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x10, 0x00, 0x06, 0x06, 0x04, 0x01], // cqm2: a Irlanda do Norte
    [0x10, 0x00, 0xff, 0x14, 0x02, 0x00],
];

/// the Northern Ireland rule (16): one window, from 6 May (moved to the Saturday [inference]) to 20 March
/// Original: `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES` (0x7936d0, northern_ireland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7936d0 } (thiscall, ret 0x14, northern_ireland_rules.cpp, `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda do Norte (regra 16): a base (0x8e84a0), a vtable 0x96df9c, 1 janela(s) de `NORTHERN_IRELAND_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_northern_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_2 irlanda_do_norte");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96df9c, 1, &NORTHERN_IRELAND_TRANSFER_WINDOWS, 0xa697f4, 0x4e);
    this
}


/// As janelas do País de Gales: uma janela, de 7 de maio (andando para o domingo [inferência]) a 20 de março.
pub(crate) const WALES_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x1a, 0x00, 0x00, 0x07, 0x04, 0x01], // cqm2: a País de Gales
    [0x1a, 0x00, 0xff, 0x14, 0x02, 0x00],
];

/// the Wales rule (26): one window, from 7 May (moved to the Sunday [inference]) to 20 March
/// Original: `WALES_RULES::WALES_RULES` (0x90f080, wales_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x90f080 } (thiscall, ret 0x14, wales_rules.cpp, `WALES_RULES::WALES_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência do País de Gales (regra 26): a base (0x8e84a0), a vtable 0x970a9c, 1 janela(s) de `WALES_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_wales_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqm_2 pais_de_gales");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x970a9c, 1, &WALES_TRANSFER_WINDOWS, 0xad4404, 0x4e);
    this
}


