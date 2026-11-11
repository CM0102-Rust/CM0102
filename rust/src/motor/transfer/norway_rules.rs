use super::*;

/// Constructs a nation's transfer rules: base TRANSFER_RULES ctor, the nation vtable and the transfer windows.
pub(crate) unsafe fn construct_nation_rules(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], file: usize, row: u32) {
    exe::new_transfer_rules(this, a[0], a[1], a[2], a[3], a[4]);
    write_u32(this, vtable);
    set_transfer_windows(this, n, t, file, row);
}

/// As janelas da Noruega: uma janela, de 1º de setembro a 31 de agosto.
pub(crate) const NORWAY_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x11, 0x00, 0xff, 0x01, 0x08, 0x01],
    [0x11, 0x00, 0xff, 0x1f, 0x07, 0x00], // cqm2: a Noruega
];

/// Constructs the Norway transfer rules (rule 17): one window, from 1 September to 31 August.
///
/// `NORWAY_RULES::NORWAY_RULES`; SEH frame only for the `new`, no destructor on the normal path: the transfer rules of
/// Norway (rule 17): the base (`exe::new_transfer_rules`), the vtable 0x96dfe4, 1 window from `NORWAY_TRANSFER_WINDOWS`. Returns the
/// object.
/// Original: `NORWAY_RULES::NORWAY_RULES` (0x7944f0, norway_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_norway_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("cqm_2 norway");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96dfe4, 1, &NORWAY_TRANSFER_WINDOWS, 0xa69870, 0x51);
    this
}

/// Northern Ireland's transfer windows: one window, from 6 May (moved to the Saturday [inference]) to 20 March.
pub(crate) const NORTHERN_IRELAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x10, 0x00, 0x06, 0x06, 0x04, 0x01], // cqm2: Northern Ireland
    [0x10, 0x00, 0xff, 0x14, 0x02, 0x00],
];

/// The transfer windows of Wales: one window, from 7 May (moving to the Sunday [inference]) to 20 March.
pub(crate) const WALES_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x1a, 0x00, 0x00, 0x07, 0x04, 0x01], // cqm2: Wales
    [0x1a, 0x00, 0xff, 0x14, 0x02, 0x00],
];

