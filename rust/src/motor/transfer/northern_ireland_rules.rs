use super::*;

/// Constructs the Northern Ireland transfer rules (rule 16): one window, from 6 May (moved to the Saturday [inference])
/// to 20 March.
///
/// `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES`; SEH frame only for the `new`, no destructor on the normal path.
/// The Northern Ireland transfer rules (rule 16): the base (`exe::new_transfer_rules`), the vtable 0x96df9c, 1 window from
/// `NORTHERN_IRELAND_TRANSFER_WINDOWS`. Returns the object.
/// Original: `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES` (0x7936d0, northern_ireland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7936d0 } (thiscall, ret 0x14, northern_ireland_rules.cpp, `NORTHERN_IRELAND_RULES::NORTHERN_IRELAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência da Irlanda do Norte (regra 16): a base (0x8e84a0), a vtable 0x96df9c, 1 janela(s) de `NORTHERN_IRELAND_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_northern_ireland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_2 northern_ireland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96df9c, 1, &NORTHERN_IRELAND_TRANSFER_WINDOWS, 0xa697f4, 0x4e);
    this
}
