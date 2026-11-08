use super::*;

/// Constructs the Poland transfer rules (rule 18): one window, from 1 July to 20 February; +0x13 = 5 (base default 50)
/// [inference: the foreign player limit]; +0x14 = 5 (default 30).
///
/// Ret 0x14, `POLAND_RULES::POLAND_RULES`; SEH frame only for the `new`, no destructor on the normal path: the base
/// (`exe::new_transfer_rules`), vtable 0x96e61c, 1 window from `POLAND_TRANSFER_WINDOWS`; +0x13 = 5 (the base default is 50) [inference:
/// the foreign player limit]; +0x14 = 5 (the default is 30). Returns the object.
/// Original: `POLAND_RULES::POLAND_RULES` (0x7cd830, poland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7cd830 } (thiscall, ret 0x14, poland_rules.cpp, `POLAND_RULES::POLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Polônia (regra 18): a base (0x8e84a0), a vtable 0x96e61c, 1 janela(s) de `POLAND_TRANSFER_WINDOWS`; o +0x13 = 5 (o padrão da base é 50) [inferência: o limite de estrangeiros]; o +0x14 = 5 (o padrão é 30). Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_poland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_3 poland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96e61c, 1, &POLAND_TRANSFER_WINDOWS, 0xa786b8, 0x51);
    *((this + 0x13) as *mut u8) = 5;
    *((this + 0x14) as *mut u8) = 5;
    this
}
