use super::*;

/// whether the transfer offer is closed (by its state and, in state 0xf, by its phase)
/// Original: `transfer_offer_closed` (0x8d0d90, transfer_offer.cpp, stdcall).
pub(crate) unsafe extern "system" fn transfer_offer_closed(o: usize) -> u32 {
    let o = Offer(o);
    match o.status() {
        2 | 3 | 4 | 0x10 | 0x14 => 1,
        0xf => (o.response_round() >= 3) as u32,
        _ => 0,
    }
}
