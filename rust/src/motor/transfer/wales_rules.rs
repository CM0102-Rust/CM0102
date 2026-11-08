use super::*;

/// Constructs the Wales transfer rules (rule 26): one window, from 7 May (moved to the Sunday [inference]) to 20 March.
///
/// `WALES_RULES::WALES_RULES`; SEH frame only for the `new`, no destructor on the normal path. The Wales transfer rules
/// (rule 26): the base (0x8e84a0), the vtable 0x970a9c, 1 window from `WALES_TRANSFER_WINDOWS`. Returns the object.
/// Original: `WALES_RULES::WALES_RULES` (0x90f080, wales_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x90f080 } (thiscall, ret 0x14, wales_rules.cpp, `WALES_RULES::WALES_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de transferência do País de Gales (regra 26): a base (0x8e84a0), a vtable 0x970a9c, 1 janela(s) de `WALES_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_wales_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqm_2 pais_de_gales");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x970a9c, 1, &WALES_TRANSFER_WINDOWS, 0xad4404, 0x4e);
    this
}
