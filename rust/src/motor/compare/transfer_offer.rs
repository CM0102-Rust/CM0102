use super::*;

// The comparers of the routines in ../transfer_offer.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer_offer.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `transfer_offer_closed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "system" fn comparar_transfer_offer_closed(o: usize) -> u32 {
    let r = original_fn::<unsafe extern "system" fn(usize) -> u32>(IDX_TRANSFER_OFFER_CLOSED)(o);
    let n = transfer_offer_closed(o);
    SCOREBOARD.record(r == n, || format!("transfer_offer_closed(estado {:#x}, {:#x}) original={} rust={}", *((o + 0x2c) as *const u8), *((o + 0x2e) as *const u8), r, n));
    r
}
