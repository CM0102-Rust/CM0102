use super::*;

/// whether the transfer offer is closed (by its state and, in state 0xf, by its phase)
/// Original: `transfer_offer_closed` (0x8d0d90, transfer_offer.cpp, stdcall).
///
/// [pt] { 0x8d0d90 } (stdcall, transfer_offer.cpp): a oferta de transferência está encerrada? Estados
/// (+0x2c) 2, 3, 4, 0x10 e 0x14 sim; 0xf só com +0x2e a partir de 3; os demais não.
pub(crate) unsafe extern "system" fn transfer_offer_closed(o: usize) -> u32 {
    match *((o + offer::STATUS) as *const u8) {
        2 | 3 | 4 | 0x10 | 0x14 => 1,
        0xf => (*((o + offer::RESPONSE_ROUND) as *const u8) >= 3) as u32,
        _ => 0,
    }
}
