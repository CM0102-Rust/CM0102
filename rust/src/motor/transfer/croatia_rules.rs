use super::*;

/// Constructs the Croatia transfer rules (rule 5): one window, from 1 June to 30 March.
///
/// `CROATIA_RULES::CROATIA_RULES`; SEH frame only for the `new`, no destructor on the normal path: the transfer rules
/// of Croatia (rule 5): the base (0x8e84a0), the vtable 0x969394, 1 window from `CROATIA_TRANSFER_WINDOWS`. Returns the
/// object.
/// Original: `CROATIA_RULES::CROATIA_RULES` (0x518500, croatia_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x518500 } (thiscall, ret 0x14, croatia_rules.cpp, `CROATIA_RULES::CROATIA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Croácia (regra 5): a base (0x8e84a0), a vtable 0x969394, 1 janela(s) de `CROATIA_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_croatia_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_1 croacia");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x969394, 1, &CROATIA_TRANSFER_WINDOWS, 0x9b5570, 0x4e);
    this
}
