use super::*;

// The comparers of the routines in ../transfer_offer.rs.
// [pt] Os comparadores das rotinas de ../transfer_offer.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `transfer_offer_closed`.
pub(crate) unsafe extern "system" fn compare_transfer_offer_closed(o: usize) -> u32 {
    let r = original_fn::<unsafe extern "system" fn(usize) -> u32>(IDX_TRANSFER_OFFER_CLOSED)(o);
    let n = transfer_offer_closed(o);
    SCOREBOARD.record(r == n, || format!("transfer_offer_closed(estado {:#x}, {:#x}) original={} rust={}", *((o + offer::STATUS) as *const u8), *((o + offer::RESPONSE_ROUND) as *const u8), r, n));
    r
}
