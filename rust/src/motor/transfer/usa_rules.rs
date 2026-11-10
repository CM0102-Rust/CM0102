use super::*;

/// Constructs the USA transfer rules (rule 25): one window, from 7 January (moved to the Wednesday [inference]) to 15
/// August; +0x13 = 4 (default 50) [inference: the foreign squad limit].
///
/// `USA_RULES::USA_RULES`; SEH frame only for the `new`, no destructor on the normal path: the transfer rules of the
/// USA (rule 25): the base (`exe::new_transfer_rules`), the vtable 0x970a38, 1 window from `USA_TRANSFER_WINDOWS` and +0x13 = 4 (the
/// base default is 50) [inference: the foreign player limit]. Returns the object.
/// Original: `USA_RULES::USA_RULES` (0x90b940, usa_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x90b940 } (thiscall, ret 0x14, usa_rules.cpp, `USA_RULES::USA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência dos EUA (regra 25): a base (0x8e84a0), a vtable 0x970a38, 1 janela(s) de `USA_TRANSFER_WINDOWS` e o +0x13 = 4 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_usa_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_1 usa");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x970a38, 1, &USA_TRANSFER_WINDOWS, 0xaa211c, 0x54);
    write_u8(this + 0x13, 4);
    this
}
