// ---------- sx05b_21: setting the display currency (cash.cpp) ----------

/// The display currency's prefix text (before the amount) and the round exchange rate (f64) the preset amounts use;
/// the id of the display currency (byte).
pub(crate) const CURRENCY_PREFIX: usize = 0xad9ca8;
pub(crate) const CURRENCY_ROUND_RATE: usize = 0xad9da0;
pub(crate) const CURRENCY_ID: usize = 0x98b140;

/// One display currency as `cash_set_currency` sets it: the exchange rate, the round rate and the scale (f64 bits),
/// the prefix and suffix texts and the unit texts of thousands, millions, billions and thousands of billions (copied
/// from these addresses of the exe), the thousands and decimal separators and the no-separator-under-10,000 flag.
pub(crate) struct Currency {
    rate: u64,
    round_rate: u64,
    scale: u64,
    prefix: usize,
    suffix: usize,
    thousands: u8,
    no_separator_under_10000: u32,
    decimal: u8,
    units: [usize; 4],
}

/// The 26 currencies by id, as the original's jump table sets them.
pub(crate) const CURRENCIES: [Currency; 26] = [
    Currency { rate: 0x3ff0000000000000, round_rate: 0x3ff0000000000000, scale: 0x3ff0000000000000, prefix: 0x98b254, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 0 pound
    Currency { rate: 0x3ff7d70a3d70a3d7, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 1 peso
    Currency { rate: 0x4003eb851eb851ec, round_rate: 0x4004000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 2 Australian dollar
    Currency { rate: 0x405010a3d70a3d71, round_rate: 0x4049000000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b3e8, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3e0, 0x98b3d8, 0x98b3d0, 0x98b3c8] }, // 3 Belgian franc
    Currency { rate: 0x4005d70a3d70a3d7, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3c4, suffix: 0x98b3c0, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 4 real
    Currency { rate: 0x40283851eb851eb8, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b3bc, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b3b8, 0x98b3b0, 0x98b3ac, 0x98b3a8] }, // 5 kuna
    Currency { rate: 0x4027c28f5c28f5c3, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b3a4, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b39c, 0x98b394, 0x98b38c, 0x98b384] }, // 6 Danish krone
    Currency { rate: 0x3ff8cccccccccccd, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b380, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 7 euro
    Currency { rate: 0x4022f0a3d70a3d71, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b37c, thousands: 0x2c, no_separator_under_10000: 0x1, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 8 markka
    Currency { rate: 0x4024e66666666666, round_rate: 0x4024000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b378, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b374, 0x98b370, 0x98b3e8, 0x98b36c] }, // 9 French franc
    Currency { rate: 0x4008e147ae147ae1, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b368, thousands: 0x2e, no_separator_under_10000: 0x1, decimal: 0x2c, units: [0x98b360, 0x98b358, 0x98b350, 0x98b348] }, // 10 Deutschmark
    Currency { rate: 0x400c147ae147ae14, round_rate: 0x4008000000000000, scale: 0x3ff0000000000000, prefix: 0x98b344, suffix: 0x98b340, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 11 guilder
    Currency { rate: 0x4080b70a3d70a3d7, round_rate: 0x4081300000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b33c, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b334, 0x98b330, 0x98b3f0, 0x98b3ec] }, // 12 drachma
    Currency { rate: 0x3ff4000000000000, round_rate: 0x3ff0000000000000, scale: 0x3ff0000000000000, prefix: 0x98b328, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b324, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 13 Irish pound
    Currency { rate: 0x40a817dc28f5c28f, round_rate: 0x40a7700000000000, scale: 0x408f400000000000, prefix: 0x98b31c, suffix: 0xad9c64, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3f8, 0x98b318, 0x98b3f4, 0x98b3ec] }, // 14 lira
    Currency { rate: 0x4063be147ae147ae, round_rate: 0x4062c00000000000, scale: 0x4059000000000000, prefix: 0x98b2dc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 15 yen
    Currency { rate: 0x402a28f5c28f5c29, round_rate: 0x402e000000000000, scale: 0x4024000000000000, prefix: 0x98b2d8, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b3b8, 0x98b2d4, 0x98b3ac, 0x98b3a8] }, // 16 Norwegian krone
    Currency { rate: 0x401a5c28f5c28f5c, round_rate: 0x4014000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b2d0, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b2c8, 0x98b2c0, 0x98b3ac, 0x98b3a8] }, // 17 zloty
    Currency { rate: 0x3fd460807357e671, round_rate: 0x3fd0000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b2bc, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b2b8, 0x98b2b4, 0x98b2b0, 0x98b2ac] }, // 18 escudo
    Currency { rate: 0x4044dae147ae147b, round_rate: 0x4049000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b2a8, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b2a0, 0x98b294, 0x98b3ac, 0x98b3a8] }, // 19 ruble
    Currency { rate: 0x4070887ae147ae14, round_rate: 0x406f400000000000, scale: 0x4059000000000000, prefix: 0xad9c64, suffix: 0x98b28c, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b284, 0x98b27c, 0xad9c64, 0x98b274] }, // 20 peseta
    Currency { rate: 0x4030000000000000, round_rate: 0x402e000000000000, scale: 0x4024000000000000, prefix: 0xad9c64, suffix: 0x98b270, thousands: 0x20, no_separator_under_10000: 0x0, decimal: 0x2c, units: [0x98b268, 0x98b260, 0x98b258, 0x98b268] }, // 21 Swedish krona
    Currency { rate: 0x4003ae147ae147ae, round_rate: 0x4004000000000000, scale: 0x3ff0000000000000, prefix: 0xad9c64, suffix: 0x98b24c, thousands: 0x2c, no_separator_under_10000: 0x1, decimal: 0x2e, units: [0x98b3b8, 0x98b244, 0x98b3ac, 0x98b3a8] }, // 22 Swiss franc
    Currency { rate: 0x412c77273d70a3d7, round_rate: 0x412e848000000000, scale: 0x412e848000000000, prefix: 0xad9c64, suffix: 0x98b240, thousands: 0x2e, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3b8, 0x98b238, 0x98b230, 0x98b224] }, // 23 Turkish lira
    Currency { rate: 0x3ff9eb851eb851ec, round_rate: 0x3ff8000000000000, scale: 0x3ff0000000000000, prefix: 0x98b3fc, suffix: 0xad9c64, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b3f8, 0x98b3f4, 0x98b3f0, 0x98b3ec] }, // 24 US dollar
    Currency { rate: 0x409dc00000000000, round_rate: 0x409f400000000000, scale: 0x409f400000000000, prefix: 0xad9c64, suffix: 0x98b310, thousands: 0x2c, no_separator_under_10000: 0x0, decimal: 0x2e, units: [0x98b308, 0x98b2f8, 0x98b2e8, 0x98b2e0] }, // 25 won
];

/// Sets the display currency `id` (0 to 25, signed): its exchange rate (`CURRENCY_RATE`), round rate
/// (`CURRENCY_ROUND_RATE`) and scale (`CURRENCY_SCALE`), its prefix (`CURRENCY_PREFIX`) and suffix (`CURRENCY_SUFFIX`),
/// the unit texts (`CURRENCY_UNIT_THOUSANDS`, `_MILLIONS`, `_BILLIONS`, `_TRILLIONS`), the separators and the
/// no-separator flag, and the id itself (`CURRENCY_ID`); 1. Any other id: the "Error" box (line 0x265) and 0, nothing
/// changed.
/// Original: `cash_set_currency` (0x43f7f0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_set_currency(id: u32) -> u32 {
    let k = id as u8 as i8 as i32;
    if !(0..26).contains(&k) {
        show_error_at(CASH_CPP, 0x265);
        return 0;
    }
    let c = &CURRENCIES[k as usize];
    covered!("sx05b_21 currency set");
    write_u64(CURRENCY_RATE, c.rate);
    write_u64(CURRENCY_ROUND_RATE, c.round_rate);
    write_u64(CURRENCY_SCALE, c.scale);
    tel26_copy_text(CURRENCY_PREFIX as *mut u8, c.prefix as *const u8);
    tel26_copy_text(CURRENCY_SUFFIX as *mut u8, c.suffix as *const u8);
    for (dst, &src) in [CURRENCY_UNIT_THOUSANDS, CURRENCY_UNIT_MILLIONS, CURRENCY_UNIT_BILLIONS, CURRENCY_UNIT_TRILLIONS].iter().zip(c.units.iter()) {
        tel26_copy_text(*dst as *mut u8, src as *const u8);
    }
    write_u8(CURRENCY_THOUSANDS_SEPARATOR, c.thousands);
    write_u8(CURRENCY_DECIMAL_SEPARATOR, c.decimal);
    write_u32(CURRENCY_NO_SEPARATOR_UNDER_10000, c.no_separator_under_10000);
    write_u8(CURRENCY_ID, id as u8);
    1
}

/// Comparer for `cash_set_currency`: side by side over the currency globals (0xad9c70 to the round rate, and the id).
pub(crate) unsafe extern "C" fn compare_cash_set_currency(id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I0%%);
    let regions = [(CURRENCY_SUFFIX, CURRENCY_ROUND_RATE + 8 - CURRENCY_SUFFIX), (CURRENCY_ID, 1)];
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&regions, || o(id), || cash_set_currency(id));
    SCOREBOARD.record(equal && r == v, || format!("cash_set_currency({}) original={} rust={} {:?}", id as u8 as i8, r, v, first_differences(&a, &b)));
    r
}
