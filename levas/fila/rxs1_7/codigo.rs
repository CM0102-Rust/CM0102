/// Writes up to two reasons why a player refuses a move into `out` from the reason flags, joined with "and", and
/// returns how many it wrote.
///
/// `transfer_manager_build_refusal_reasons_text(seller, buyer, _, _, flags, interest, out)`: the clubs by id (−1 none);
/// "same nation" when both exist and their `.nation()` are equal. Flag 0x80000000 alone: "wants to wait until the end
/// of the current season before deciding his future" (0xa9e17c) straight into `out`, 1. Otherwise, in order, while
/// fewer than two: one of 8 "revealed that he would find it very difficult to leave <seller>" (0xa9e124), 0x20 "is not
/// prepared to leave <seller> for a rival club" (0xa9e0d8), 0x400 "is currently very happy at <seller>" (0xa9e09c),
/// 0x1000 "is currently very happy with his..." (0xa9e050), 0x100000 "has only recently joined current club <seller>"
/// (0xa9e008) (all with a seller), or 1 with `interest` ≥ 2.75 "is not yet ready to come to a decision regarding his
/// future" (0xa9dfcc); 0x400000 with a seller in a continental competition (`club::CONTINENTAL_COMP_ID`, or its
/// `G_CLUB_STATE_TABLE` entry) "does not want to lose the opportunity of playing in the <competition>" (0xa9df68);
/// 0x80000 outside the same nation "is not prepared to move abroad to play in the reserves" (0xa9df30), or 0x8000
/// "doesn't want to drop down to a lower division without the guarantee of first team football" (0xa9ded4, both
/// divisions in the same nation and the seller's division `.reputation()` above the buyer's) or "is not prepared to
/// move unless he is guaranteed first team football" (0xa9de90); 0x1000000 outside the same nation, with the buyer's
/// nation, "is unwilling to move to a club in <nation>" (0xa9de4c), or 0x2000000 with the buyer's nation's continent
/// "... in <continent>" (0xa9de08), or 0x10000 outside the same nation "is concerned about the difference in language
/// and lifestyle" (0xa9ddcc); 0x20000000 "is unsure whether the move would benefit him in the long term" (0xa9dd8c).
/// One reason is copied (`FN_CRT_SPRINTF` with "%s"), two are joined by 0x9a9f28; none leaves `out` as it was.
/// Original: `transfer_manager_build_refusal_reasons_text` (0x8e4ae0, transfer_manager.cpp, stdcall, ret 0x20).
pub(crate) unsafe extern "stdcall" fn transfer_manager_build_refusal_reasons_text(seller: i32, buyer: i32, _a: u32, _b: u32, flags: u32, interest: f64, out: usize) -> u8 {
    let fmt = news_text_format();
    let mut reasons = [[0u8; 0xfa]; 5];
    let mut n = 0usize;
    let club = |id: i32| club_at(id as isize);
    let same_nation = buyer > -1 && seller > -1 && club(seller).nation().addr() == club(buyer).nation().addr();
    if flags & 0x80000000 != 0 {
        fmt(out, 0xa9e17c); // "wants to wait until the end of the current season before deciding his future"
        return 1;
    }
    let bit = |b: u32| flags & b != 0;
    let seller_template = if seller < 0 {
        None
    } else if bit(8) {
        Some(0xa9e124) // "revealed that he would find it very difficult to leave <club>"
    } else if bit(0x20) {
        Some(0xa9e0d8) // "is not prepared to leave <club> for a rival club"
    } else if bit(0x400) {
        Some(0xa9e09c) // "is currently very happy at <club>"
    } else if bit(0x1000) {
        Some(0xa9e050) // "is currently very happy with his ..."
    } else if bit(0x100000) {
        Some(0xa9e008) // "has only recently joined current club <club>"
    } else {
        None
    };
    if let Some(t) = seller_template {
        let s = club(seller);
        let g = s.short_gender_name() as i8 as i32;
        fmt(reasons[0].as_mut_ptr() as usize, t, g, g, s.addr() + club::SHORT_NAME);
        n = 1;
    } else if bit(1) && interest >= read_f64(rdata::F64_2_75) {
        fmt(reasons[0].as_mut_ptr() as usize, 0xa9dfcc); // "is not yet ready to come to a decision regarding his future"
        n = 1;
    }
    if bit(0x400000) && seller > -1 {
        let mut comp = read_i32(club(seller).addr() + club::CONTINENTAL_COMP_ID);
        if comp <= -1 {
            comp = read_i32(read_usize(G_CLUB_STATE_TABLE) + seller as usize * 6);
        }
        if comp > -1 {
            let c = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp as usize).wrapping_mul(0x6b)));
            let g = c.gender_name() as i8 as i32;
            covered!("rxs1_7 refusal for the continental competition");
            fmt(reasons[n].as_mut_ptr() as usize, 0xa9df68, g, g, c.name_addr()); // rxs1_7: continental competition
            n += 1;
        }
    }
    if n < 2 {
        let t = if !same_nation && bit(0x80000) {
            Some(0xa9df30) // "is not prepared to move abroad to play in the reserves"
        } else if bit(0x8000) {
            let lower = buyer > -1 && seller > -1 && {
                let (from, to) = (club(seller).division(), club(buyer).division());
                !from.is_null() && !to.is_null() && read_u32(to.addr() + competition::NATION) == read_u32(from.addr() + competition::NATION)
                    && read_i16(from.addr() + competition::REPUTATION) > read_i16(to.addr() + competition::REPUTATION)
            };
            Some(if lower { 0xa9ded4 } else { 0xa9de90 }) // a lower division / first team football
        } else {
            None
        };
        if let Some(t) = t {
            fmt(reasons[n].as_mut_ptr() as usize, t);
            n += 1;
        }
    }
    if n < 2 {
        let buyer_nation = if buyer > -1 { club(buyer).nation() } else { Nation(0) };
        if !same_nation && bit(0x1000000) && !buyer_nation.is_null() {
            let g = read_i8(buyer_nation.addr() + nation::GENDER_NAME) as i32;
            fmt(reasons[n].as_mut_ptr() as usize, 0xa9de4c, g, g, buyer_nation.addr() + 4); // "... a club in <nation>"
            n += 1;
        } else if bit(0x2000000) && !buyer_nation.is_null() && read_usize(buyer_nation.addr() + nation::CONTINENT) != 0 {
            let continent = read_usize(buyer_nation.addr() + nation::CONTINENT);
            let g = read_i8(continent + 0x1e) as i32; // the continent name's gender
            fmt(reasons[n].as_mut_ptr() as usize, 0xa9de08, g, g, continent + 4); // "... a club in <continent>"
            n += 1;
        } else if !same_nation && bit(0x10000) {
            fmt(reasons[n].as_mut_ptr() as usize, 0xa9ddcc); // "is concerned about the difference in language and lifestyle"
            n += 1;
        }
    }
    if n < 2 && bit(0x20000000) {
        fmt(reasons[n].as_mut_ptr() as usize, 0xa9dd8c); // "is unsure whether the move would benefit him in the long term"
        n += 1;
    }
    match n {
        1 => {
            let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            crt_format(out, 0x991944, reasons[0].as_ptr() as usize); // "%s"
        }
        2 => {
            fmt(out, 0x9a9f28, reasons[0].as_ptr() as usize, reasons[1].as_ptr() as usize); // "<1> and <2>"
        }
        _ => {}
    }
    n as u8
}

/// Comparer for `transfer_manager_build_refusal_reasons_text`: side by side over the string and the count (the copy
/// writes into a local buffer that starts as a copy of the caller's string, the original into `out`).
pub(crate) unsafe extern "stdcall" fn compare_transfer_manager_build_refusal_reasons_text(seller: i32, buyer: i32, a: u32, b: u32, flags: u32, interest: f64, out: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(i32, i32, u32, u32, u32, f64, usize) -> u8>(%%I0%%);
    let c_string = |p: usize| core::slice::from_raw_parts(p as *const u8, (0..0x400).find(|&k| read_u8(p + k) == 0).unwrap_or(0x3ff)).to_vec();
    let mut mine = [0u8; 0x400];
    let before = c_string(out);
    mine[..before.len()].copy_from_slice(&before);
    let r = o(seller, buyer, a, b, flags, interest, out);
    let rr = transfer_manager_build_refusal_reasons_text(seller, buyer, a, b, flags, interest, mine.as_mut_ptr() as usize);
    let (theirs, ours) = (c_string(out), c_string(mine.as_ptr() as usize));
    SCOREBOARD.record(r == rr && theirs == ours, || {
        format!("transfer_manager_build_refusal_reasons_text(flags {:#x}): original {} {:?} rust {} {:?}", flags, r,
            String::from_utf8_lossy(&theirs), rr, String::from_utf8_lossy(&ours))
    });
    r
}
