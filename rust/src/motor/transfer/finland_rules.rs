use super::*;

/// Constructs the Finland transfer rules (rule 8): one window, from 16 November to 15 August; dword +0x15 = 1.
///
/// `FINLAND_RULES::FINLAND_RULES`; SEH frame only for the `new`, no destructor on the normal path. The Finland transfer
/// rules (rule 8): the base (`exe::new_transfer_rules`), the vtable 0x96a75c, 1 window from `FINLAND_TRANSFER_WINDOWS`; dword +0x15 = 1.
/// Returns the object.
/// Original: `FINLAND_RULES::FINLAND_RULES` (0x5a9950, finland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5a9950 } (thiscall, ret 0x14, finland_rules.cpp, `FINLAND_RULES::FINLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Finlândia (regra 8): a base (0x8e84a0), a vtable 0x96a75c, 1 janela(s) de `FINLAND_TRANSFER_WINDOWS`; o dword +0x15 = 1. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_finland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_3 finland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96a75c, 1, &FINLAND_TRANSFER_WINDOWS, 0x9cc134, 0x50);
    write_u32(this + 0x15, 1);
    this
}
