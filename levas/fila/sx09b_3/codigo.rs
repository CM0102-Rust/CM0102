// --- sx09b_3: the scope text of a ban (discipline.cpp) ---

/// Writes which ruling bodies a ban covers and returns the grammatical form of the text (the `{}` codes of the
/// templates).
///
/// `get_scope_text(rules, out, size, everywhere, form)`: `rules` are the ban's four rules (`ban_record::RULES`, one per
/// person record slot; 0xff none). A `size` below 70 gives the "Error" box (line 0x1475) and returns 0xff. Three flags
/// start set: world-wide, continental and domestic; the first rule present (in slot order) is the one that speaks.
/// Slot 0 or slot 1 missing clears all three; slot 2 present clears domestic, missing clears world-wide and
/// continental. With slot 3 (the FIFA's) present: `everywhere` set gives "global" and returns 0xff; otherwise "world-wide"
/// when that flag survived, else the speaking rule's own text. Without slot 3: domestic and not continental gives
/// "domestic"; continental gives "continental"; neither, the speaking rule's own text (its vtable slot 0x10 with `out`,
/// `size` and `form`). The form returned is the speaking rule's: by its record slot (`ruling_body::RECORD_SLOT`) 1 →
/// 9, 3 → 1, 0 → 8 when the byte +0x20 is not 0xff (else 0); otherwise by its index (`ruling_body::RULE_SET_INDEX`)
/// 1 → 2, 2 → 6, 35 → 7, 41 → 3, any other → 0xff.
/// Original: `Discipline::get_scope_text` (0x55ce40, discipline.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_scope_text(this: usize, rules: usize, out: usize, size: i32, everywhere: u32, form: u32) -> u8 {
    if size < 0x46 {
        show_error_at(0x9b8f18, 0x1475);
        return 0xff;
    }
    let rule = |k: usize| read_u8(rules + k);
    let (mut world, mut continental, mut domestic) = (true, true, true);
    let mut speaker = 0xffu8;
    for k in 0..2 {
        if rule(k) == 0xff {
            (world, continental, domestic) = (false, false, false);
        } else if speaker == 0xff {
            speaker = rule(k);
        }
    }
    if rule(2) != 0xff {
        if speaker == 0xff {
            speaker = rule(2);
        }
        domestic = false;
    } else {
        (world, continental) = (false, false);
    }
    let body = |s: u8| read_usize(read_usize(this).wrapping_add((s as i8 as isize as usize).wrapping_mul(4)));
    let own_text = |s: u8| {
        let f: unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(body(s)) + 0x10)); // vtable +0x10: the rule's scope text
        f(body(s), out, size, form);
    };
    if rule(3) != 0xff {
        if everywhere != 0 {
            covered!("sx09b_3 global");
            story_lang!(out, 0x9c3478); // "global"
            return 0xff;
        }
        if speaker == 0xff {
            speaker = rule(3);
        }
        if world {
            covered!("sx09b_3 world-wide");
            story_lang!(out, 0x9c346c); // "world-wide"
        } else {
            own_text(speaker);
        }
    } else if domestic && !continental {
        covered!("sx09b_3 domestic");
        story_lang!(out, 0x9c3454); // "domestic"
    } else if continental {
        covered!("sx09b_3 continental");
        story_lang!(out, 0x9c3460); // "continental"
    } else {
        covered!("sx09b_3 the rule's own text");
        own_text(speaker);
    }
    let b = body(speaker);
    match read_u8(b + ruling_body::RECORD_SLOT) {
        1 => 9,
        0 => if read_u8(b + 0x20) != 0xff { 8 } else { 0 }, // sx09b_3: +0x20 of the ruling body (unnamed byte)
        3 => 1,
        _ => match read_u8(b + ruling_body::RULE_SET_INDEX) as i8 {
            1 => 2,
            2 => 6,
            35 => 7,
            41 => 3,
            _ => 0xff,
        },
    }
}

/// Comparer for `get_scope_text`: side by side over the output buffer and the form returned. A short buffer (the
/// "Error" box) runs the original only.
pub(crate) unsafe extern "thiscall" fn compare_get_scope_text(this: usize, rules: usize, out: usize, size: i32, everywhere: u32, form: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32) -> u8>(%%I0%%);
    if size < 0x46 {
        return o(this, rules, out, size, everywhere, form);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, size as usize)], || o(this, rules, out, size, everywhere, form), || get_scope_text(this, rules, out, size, everywhere, form));
    SCOREBOARD.record(same && r == n, || {
        format!(
            "get_scope_text(rules {:02x?}, everywhere {}, form {}) original={} rust={} {:?}",
            (0..4).map(|k| read_u8(rules + k)).collect::<Vec<_>>(),
            everywhere,
            form,
            r,
            n,
            first_differences(&mo, &mr)
        )
    });
    r
}
