// ---------- sx05b_24: the money values the screens set and show (cash.cpp) ----------

/// The seven preset money amounts in pounds the search filters offer (the dword table at 0x98b1e0: 100,000 to
/// 10,000,000).
const CASH_PRESETS: usize = 0x98b1e0;
/// The short names of the 26 currencies by id (`cash_get_currency_name`): £, Peso, Aus$, BFr, Real, Kuna, DKr, Euro,
/// Markka, FFr, DM, DFl, Drachma, IR£, Lira, ¥, NKr, Zloty, Escudo, Ruble, Pta, SKr, SwF, TLira, $ and the won (in the
/// game's hangul code).
const CURRENCY_NAMES: [usize; 26] = [0x98b254, 0x98b47c, 0x98b474, 0x98b470, 0x98b468, 0x98b460, 0x98b3a4, 0x98b458, 0x98b450, 0x98b44c,
    0x98b448, 0x98b444, 0x98b43c, 0x98b438, 0x98b430, 0x98b2dc, 0x98b42c, 0x98b424, 0x98b41c, 0x98b414, 0x98b410, 0x98b40c, 0x98b408,
    0x98b400, 0x98b3fc, 0x98b310];

/// The length of the display currency's prefix text (`CURRENCY_PREFIX`).
/// Original: `cash_currency_prefix_length` (0x43f440, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_currency_prefix_length() -> u32 {
    let mut n = 0;
    while read_u8(CURRENCY_PREFIX + n) != 0 {
        n += 1;
    }
    n as u32
}

/// Stores a display currency amount as a cash value: divided by the exchange rate and truncated (`cash_from_display`);
/// returns it.
/// Original: `Cash::set_from_display` (0x43f4c0, cash.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn cash_set_from_display(cash: usize, amount: i64) -> u32 {
    let v = cash_from_display(amount);
    write_u32(cash, v);
    v
}

/// Stores a whole pounds amount as the display currency shows it: times the round rate (`CURRENCY_ROUND_RATE`,
/// truncated to 64 bits), then back by the exchange rate (`cash_from_display`); returns it.
unsafe fn cash_set_round_amount(cash: usize, pounds: i32) -> u32 {
    let v = cash_from_display((pounds as f64 * read_f64(CURRENCY_ROUND_RATE)) as i64);
    write_u32(cash, v);
    v
}

/// Stores preset amount `index` (0 to 6 of `CASH_PRESETS`, signed) as the display currency rounds it
/// (`cash_set_round_amount`); another index shows the "Error" box (line 0xaa) and leaves the value. Returns the value.
/// Original: `Cash::set_from_preset_index` (0x43f620, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_set_from_preset_index(cash: usize, index: u32) -> u32 {
    let k = index as u8 as i8;
    if !(0..7).contains(&k) {
        show_error_at(CASH_CPP, 0xaa);
        return read_u32(cash);
    }
    covered!("sx05b_24 preset amount");
    cash_set_round_amount(cash, read_i32(CASH_PRESETS + 4 * k as usize))
}

/// Stores amount `index` of a caller's dword table of `count` amounts as the display currency rounds it
/// (`cash_set_round_amount`); a null table or an index (signed byte) outside 0..count (signed word) shows the "Error"
/// box (line 0xc0) and leaves the value. Returns the value.
/// Original: `Cash::set_from_amount_table` (0x43f720, cash.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn cash_set_from_amount_table(cash: usize, index: u32, table: usize, count: u32) -> u32 {
    let k = index as u8 as i8;
    if table == 0 || k as i16 >= count as u16 as i16 || k < 0 {
        show_error_at(CASH_CPP, 0xc0);
        return read_u32(cash);
    }
    covered!("sx05b_24 table amount");
    cash_set_round_amount(cash, read_i32(table + 4 * k as usize))
}

/// Copies the short name of currency `id` (0 to 25, signed byte) into `buf`; 1. A null buffer or a size (signed word)
/// below 50 shows the "Error" box (line 0x285), a bad id the box of line 0x2db; then 0.
/// Original: `cash_get_currency_name` (0x4415f0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_get_currency_name(buf: usize, size: u32, id: u32) -> u32 {
    if buf == 0 || (size as u16 as i16) < 0x32 {
        show_error_at(CASH_CPP, 0x285);
        return 0;
    }
    let k = id as u8 as i8 as i32;
    if !(0..26).contains(&k) {
        show_error_at(CASH_CPP, 0x2db);
        return 0;
    }
    tel26_copy_text(buf as *mut u8, CURRENCY_NAMES[k as usize] as *const u8);
    1
}

/// Comparer for `cash_currency_prefix_length`: the value side by side.
pub(crate) unsafe extern "C" fn compare_cash_currency_prefix_length() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I0%%)();
    let v = cash_currency_prefix_length();
    SCOREBOARD.record(r == v, || format!("cash_currency_prefix_length (Club screen money) original={} rust={}", r, v));
    r
}

/// Comparer for `cash_set_from_display`: side by side over the cash value and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_set_from_display(cash: usize, amount: i64) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i64) -> u32>(%%I1%%);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, amount), || cash_set_from_display(cash, amount));
    SCOREBOARD.record(equal && r == v, || format!("cash_set_from_display({}) original={} rust={} {:?}", amount, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `cash_set_from_preset_index`: side by side; a bad index (the error box) only the original.
pub(crate) unsafe extern "thiscall" fn compare_cash_set_from_preset_index(cash: usize, index: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    if !(0..7).contains(&(index as u8 as i8)) {
        return o(cash, index);
    }
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, index), || cash_set_from_preset_index(cash, index));
    SCOREBOARD.record(equal && r == v, || format!("cash_set_from_preset_index({}) (Player Search filters) original={} rust={} {:?}", index as u8 as i8, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `cash_set_from_amount_table`: side by side; the error box only the original.
pub(crate) unsafe extern "thiscall" fn compare_cash_set_from_amount_table(cash: usize, index: u32, table: usize, count: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(%%I3%%);
    let k = index as u8 as i8;
    if table == 0 || k as i16 >= count as u16 as i16 || k < 0 {
        return o(cash, index, table, count);
    }
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(cash, 4)], || o(cash, index, table, count), || cash_set_from_amount_table(cash, index, table, count));
    SCOREBOARD.record(equal && r == v, || format!("cash_set_from_amount_table({}) (contract/transfer screens) original={} rust={} {:?}", k, r, v, first_differences(&a, &b)));
    r
}

/// Comparer for `cash_get_currency_name`: side by side over the first 50 bytes of the buffer; the error boxes only the
/// original.
pub(crate) unsafe extern "C" fn compare_cash_get_currency_name(buf: usize, size: u32, id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I4%%);
    if buf == 0 || (size as u16 as i16) < 0x32 || !(0..26).contains(&(id as u8 as i8)) {
        return o(buf, size, id);
    }
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(buf, 0x32)], || o(buf, size, id), || cash_get_currency_name(buf, size, id));
    SCOREBOARD.record(equal && r == v, || format!("cash_get_currency_name({}) (Preferences screen) {:?}", id as u8 as i8, first_differences(&a, &b)));
    r
}
