use super::*;

/// Writes the count (+0) and the 0x44-byte records from `[+4]` into the simulated_stats.dat item; on error the "Error"
/// box without closing the item.
///
/// `SimulatedStats::save(save)`: writes into the `simulated_stats.dat` item the count (+0, u32) and the 0x44-byte
/// records of `[+4]`. Error on open or write: the "Error" box (simulated_stats.cpp 0x209, 0x210, 0x218), without
/// closing the item. Does not check for a null container.
/// Original: `SimulatedStats::save` (0x84c680, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_save(this: usize, save: usize) {
    const FONTE: usize = 0xa8278c;
    covered!("cpx_7 save simulated stats");
    let mut z = zip_open_item(save, 0xa827c0);
    if z == 0 {
        show_error_at(FONTE, 0x209);
        return;
    }
    if zip_write(this, 4, 1, z) != 1 {
        show_error_at(FONTE, 0x210);
        return;
    }
    if zip_write(read_usize(this + 4), 0x44, read_usize(this) as u32, z) != read_usize(this) as u32 { // cpx7: os registros simulados
        show_error_at(FONTE, 0x218);
        return;
    }
    zip_close(&mut z);
}
