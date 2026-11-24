// --- sx09b_7: the reason of a fine as text (Fine.cpp) ---

/// Writes the reason of a fine: 1 "Unprofessional Behaviour", 2 "Dismissal in prior match", 3 "Violent Behaviour", 4
/// "Poor performance", any other "No Reason"; in lower case with `lower` set.
///
/// `fine_reason_text(reason, out, size, lower)`: no output: nothing; a `size` (signed word) below 50 only ends the
/// output (an empty string).
/// Original: `fine_reason_text` (0x5a73a0, Fine.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fine_reason_text(_this: usize, reason: u32, out: usize, size: u32, lower: u32) {
    if out == 0 {
        return;
    }
    if (size as u16 as i16) < 0x32 {
        covered!("sx09b_7 fine reason in a short buffer");
        write_u8(out, 0);
        return;
    }
    let text = if lower as u8 == 0 {
        covered!("sx09b_7 fine reason in capitals");
        match reason as u8 as i8 {
            1 => 0x9c3480, // "Unprofessional Behaviour"
            2 => 0x9ca2a8, // "Dismissal in prior match"
            3 => 0x9ca294, // sx09b_7: "Violent Behaviour"
            4 => 0x9ca280, // "Poor performance"
            _ => 0x9ca274, // "No Reason"
        }
    } else {
        covered!("sx09b_7 fine reason in lower case");
        match reason as u8 as i8 {
            1 => 0x9ca258, // "unprofessional behaviour"
            2 => 0x9ca23c, // "dismissal in prior match"
            3 => 0x9ca228, // "violent behaviour"
            4 => 0x9ca214, // "poor performance"
            _ => 0x9ca208, // "no reason"
        }
    };
    story_lang!(out, text);
}

/// Comparer for `fine_reason_text`: side by side over the output buffer.
pub(crate) unsafe extern "thiscall" fn compare_fine_reason_text(this: usize, reason: u32, out: usize, size: u32, lower: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32, u32)>(%%I0%%);
    if out == 0 {
        return o(this, reason, out, size, lower);
    }
    let len = (size as u16 as i16).max(1) as usize;
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(out, len)], || o(this, reason, out, size, lower), || fine_reason_text(this, reason, out, size, lower));
    SCOREBOARD.record(same, || format!("discipline screen: fine_reason_text(reason {}, lower {}) {:?}", reason as u8 as i8, lower as u8, first_differences(&mo, &mr)));
}
