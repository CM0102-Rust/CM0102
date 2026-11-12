// ---------- batch %%LEVA%%: loading the saved game's temporary files: the CONMEBOL and UEFA seedings and the stadium bookings (conmebol_seeding.cpp, uefa_seeding.cpp, stadium.cpp) ----------

/// Opens `name` in CM3_TEMP for reading (`exe::utils_build_file_path`(0x987da0 "CM3_TEMP", name): 2 pushes; add esp,
/// 8 -> cdecl; fopen `exe::crt_fopen`(path, 0x98f928 "rb"): cdecl).
unsafe fn temp_file_open_for_read(name: usize) -> usize {
    exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), 0x98f928)
}

/// The absolute value of the saved game's data version ([`G_GAME_DATA_VERSION`], cdq; xor; sub).
unsafe fn game_data_version() -> i32 {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs()
}

/// Rule: loading a saved game reads back the CONMEBOL seeding: each nation's places, the last season's year and (from
/// data version 0x3e010 on) the Copa dos Campeões winner, absent in older saves (−1).
///
/// `ConmebolSeeding::load_from_temp_file()`. Without CM3_TEMP\conmebol_seeding.tmp (a new game): 0. Otherwise, with
/// fread (`exe::crt_fread`, cdecl): the nation count (`conmebol_seeding::NATION_COUNT`, 1 byte; short: the error box at
/// line 0x2aa), that many 10-byte nation records from +0 (the count as a signed byte; short: the box at line 0x2b3),
/// the year (`conmebol_seeding::LAST_SEASON_YEAR`, 2 bytes; short: the box at line 0x2bb); each record's nation pointer
/// (+4) rebuilt from its id (+0): [`G_NATION_TABLE`] + `nation::SIZE` × id. Then, with the data version
/// (`game_data_version`) below 0x3e010, `conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID` = −1; from it on, read (4 bytes;
/// short: the box at line 0x2d2). The file closed (`exe::crt_fclose`) and 1. On the boxes the file stays open and it
/// returns 0 (a defect of the original: the handle leaks).
/// Original: `ConmebolSeeding::load_from_temp_file` (0x4c6970, conmebol_seeding.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_seeding_load_from_temp_file(this: usize) -> u32 {
    let f = temp_file_open_for_read(0x9a7f10);
    if f == 0 {
        covered!("cqx_13 conmebol seeding without the file");
        return 0;
    }
    if exe::crt_fread(this + conmebol_seeding::NATION_COUNT, 1, 1, f) != 1 {
        show_error_at(0x9a7ed8, 0x2aa);
        return 0;
    }
    let n = read_u8(this + conmebol_seeding::NATION_COUNT) as i8 as i32;
    if exe::crt_fread(this, 0xa, n as u32, f) as i32 != read_u8(this + conmebol_seeding::NATION_COUNT) as i8 as i32 {
        show_error_at(0x9a7ed8, 0x2b3);
        return 0;
    }
    if exe::crt_fread(this + conmebol_seeding::LAST_SEASON_YEAR, 2, 1, f) != 1 {
        show_error_at(0x9a7ed8, 0x2bb);
        return 0;
    }
    covered!("cqx_13 conmebol seeding read");
    let mut k = 0i8;
    while k < read_u8(this + conmebol_seeding::NATION_COUNT) as i8 {
        let e = this + 0xa * k as usize;
        write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add(read_usize(e).wrapping_mul(nation::SIZE))); // cqx13: the nation of the CONMEBOL record
        k += 1;
    }
    if game_data_version() < 0x3e010 {
        write_u32(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID, u32::MAX);
    } else if exe::crt_fread(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID, 4, 1, f) != 1 {
        show_error_at(0x9a7ed8, 0x2d2);
        return 0;
    }
    exe::crt_fclose(f);
    1
}

/// Comparer for `conmebol_seeding_load_from_temp_file`, side by side over the same object: the original first, then
/// the copy, which reads the same file again into the same fields (it allocates nothing); the 0x75 bytes of the
/// seeding and the return must be equal. Without the file (a new game) both return 0.
unsafe extern "thiscall" fn compare_conmebol_seeding_load_from_temp_file(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let original = crate::comparison::MemorySnapshot::read(&[(this, 0x75)]);
    let c = conmebol_seeding_load_from_temp_file(this);
    let copy = crate::comparison::MemorySnapshot::read(&[(this, 0x75)]);
    SCOREBOARD.record(r == c && copy == original, || format!("conmebol_seeding_load_from_temp_file({:#x}): original {} copy {}, bytes equal {}", this, r, c, copy == original));
    r
}

/// Rule: loading a saved game reads back the UEFA seeding: each nation's coefficients, the season year, the extra
/// list of up to 30 values and the 50 domestic cup semi-final slots.
///
/// `UefaSeeding::load_from_temp_file()`. Without CM3_TEMP\uefa_seeding.tmp (a new game): 0. Otherwise, with fread
/// (`exe::crt_fread`, cdecl): the nation count (+0x438, 1 byte; short: the error box at line 0x5f4), that many
/// 0x12-byte nation records from +0 (the count as a signed byte; the box at line 0x5fd), the year (+0x439, 2 bytes; the
/// box at line 0x605); each record's nation pointer (+4) rebuilt from its id (+0): [`G_NATION_TABLE`] + `nation::SIZE`
/// × id; the extra count (+0x43f, 1 byte; the box at line 0x613); with it nonzero, the list +0x43b freed (when there,
/// `exe::crt_free`) and made again (`exe::crt_malloc`(0x78); out of memory: the box at line 0x623) and that many dwords
/// read into it (the box at line 0x62b); the 50 slots of 12 bytes at +0x440 (the box at line 0x634). The file closed
/// (`exe::crt_fclose`) and 1. On the boxes the file stays open and it returns 0 (a defect of the original: the handle
/// leaks).
/// Original: `UefaSeeding::load_from_temp_file` (0x905230, uefa_seeding.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn uefa_seeding_load_from_temp_file(this: usize) -> u32 {
    let f = temp_file_open_for_read(0xaa1a50);
    if f == 0 {
        covered!("cqx_13 uefa seeding without the file");
        return 0;
    }
    if exe::crt_fread(this + 0x438, 1, 1, f) != 1 {
        show_error_at(0xaa1a1c, 0x5f4);
        return 0;
    }
    if exe::crt_fread(this, 0x12, read_u8(this + 0x438) as i8 as i32 as u32, f) as i32 != read_u8(this + 0x438) as i8 as i32 {
        show_error_at(0xaa1a1c, 0x5fd);
        return 0;
    }
    if exe::crt_fread(this + 0x439, 2, 1, f) != 1 {
        show_error_at(0xaa1a1c, 0x605);
        return 0;
    }
    covered!("cqx_13 uefa seeding read");
    let mut k = 0i8;
    while k < read_u8(this + 0x438) as i8 {
        let e = this + 0x12 * k as usize;
        write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add(read_usize(e).wrapping_mul(nation::SIZE))); // cqx13: the nation of the UEFA record
        k += 1;
    }
    if exe::crt_fread(this + 0x43f, 1, 1, f) != 1 {
        show_error_at(0xaa1a1c, 0x613);
        return 0;
    }
    if read_u8(this + 0x43f) != 0 {
        if read_usize(this + 0x43b) != 0 {
            exe::crt_free(read_usize(this + 0x43b));
        }
        let l = exe::crt_malloc(0x78);
        write_usize(this + 0x43b, l);
        if l == 0 {
            show_error_at(0xaa1a1c, 0x623);
            return 0;
        }
        if exe::crt_fread(l, 4, read_u8(this + 0x43f) as i8 as i32 as u32, f) as i32 != read_u8(this + 0x43f) as i8 as i32 {
            show_error_at(0xaa1a1c, 0x62b);
            return 0;
        }
    }
    if exe::crt_fread(this + 0x440, 0xc, 0x32, f) != 0x32 {
        show_error_at(0xaa1a1c, 0x634);
        return 0;
    }
    exe::crt_fclose(f);
    1
}

/// Comparer for `uefa_seeding_load_from_temp_file`, side by side over the same object: the original first, then the
/// copy, which reads the same file again (freeing the extra list the original made and making its own); the 0x698
/// bytes of the seeding (without the list pointer +0x43b) and the list's dwords (+0x43f of them) must be equal, and the
/// return.
unsafe extern "thiscall" fn compare_uefa_seeding_load_from_temp_file(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    let read = |this: usize| -> Vec<u8> {
        let mut v: Vec<u8> = (0..0x698usize).filter(|k| !(0x43b..0x43f).contains(k)).map(|k| read_u8(this + k)).collect();
        let l = read_usize(this + 0x43b);
        if l != 0 {
            v.extend((0..4 * (read_u8(this + 0x43f) as i8).max(0) as usize).map(|k| read_u8(l + k)));
        }
        v
    };
    let original = read(this);
    let c = uefa_seeding_load_from_temp_file(this);
    let copy = read(this);
    SCOREBOARD.record(r == c && copy == original, || format!("uefa_seeding_load_from_temp_file({:#x}): original {} copy {}, bytes equal {}", this, r, c, copy == original));
    r
}

/// Rule: loading a saved game reads back the stadium bookings (the dates each venue is taken) and the venue list, and
/// (from data version 0x3e029 on) the stashed venue id.
///
/// `Stadium::load_from_temp_file()`. Without CM3_TEMP\Stadium.tmp (a new game): 0. Otherwise, with fread
/// (`exe::crt_fread`, cdecl): the booking capacity (+8, 4 bytes; short: the error box at line 0xf2) and count (+4; the
/// box at line 0xf9); with a capacity, the booking array +0 made (`exe::crt_malloc`(12 × capacity); out of memory: the
/// box at line 0x105) and, with a count, that many 12-byte bookings read into it (the box at line 0x113); the venue
/// capacity (+0x14; the box at line 0x11d), count (+0x10; the box at line 0x124) and flag (+0x18, 1 byte; the box at
/// line 0x12b); with a capacity, the venue array +0xc made (16 × capacity; the box at line 0x137) and, with a count,
/// that many 16-byte venues read into it (the box at line 0x145), each venue's stadium (+0xc) rebuilt from its id: −1
/// gives null, otherwise [`G_STADIUM_TABLE`] + 0x4e × id. With the data version (`game_data_version`) from 0x3e029 on,
/// [`G_STADIUM_STASHED_VENUE_ID`] read (4 bytes; short: the box at line 0x15c, the file left open). The file closed
/// (`exe::crt_fclose`) and 1; on the other boxes the file is closed and it returns 0. (The old arrays are not freed
/// before the new ones, as the original.)
/// Original: `Stadium::load_from_temp_file` (0x85ba40, stadium.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn stadium_load_from_temp_file(this: usize) -> u32 {
    let f = temp_file_open_for_read(0xa835e0);
    if f == 0 {
        covered!("cqx_13 stadium without the file");
        return 0;
    }
    let fail = |line: u32| -> u32 {
        show_error_at(0xa835b0, line);
        exe::crt_fclose(f);
        0
    };
    if exe::crt_fread(this + 8, 4, 1, f) != 1 {
        return fail(0xf2);
    }
    if exe::crt_fread(this + 4, 4, 1, f) != 1 {
        return fail(0xf9);
    }
    if read_u32(this + 8) != 0 {
        let b = exe::crt_malloc(read_u32(this + 8).wrapping_mul(12));
        write_usize(this, b);
        if b == 0 {
            return fail(0x105);
        }
    }
    if read_u32(this + 4) != 0 && exe::crt_fread(read_usize(this), 0xc, read_u32(this + 4), f) != read_u32(this + 4) {
        return fail(0x113);
    }
    if exe::crt_fread(this + 0x14, 4, 1, f) != 1 {
        return fail(0x11d);
    }
    if exe::crt_fread(this + 0x10, 4, 1, f) != 1 {
        return fail(0x124);
    }
    if exe::crt_fread(this + 0x18, 1, 1, f) != 1 {
        return fail(0x12b);
    }
    if read_u32(this + 0x14) != 0 {
        let v = exe::crt_malloc(read_u32(this + 0x14) << 4);
        write_usize(this + 0xc, v);
        if v == 0 {
            return fail(0x137);
        }
    }
    if read_u32(this + 0x10) != 0 {
        if exe::crt_fread(read_usize(this + 0xc), 0x10, read_u32(this + 0x10), f) != read_u32(this + 0x10) {
            return fail(0x145);
        }
        covered!("cqx_13 stadium venues read");
        let mut i = 0i32;
        while i < read_i32(this + 0x10) {
            let p = read_usize(this + 0xc) + 0x10 * i as usize + 0xc;
            let id = read_i32(p);
            write_usize(p, if id == -1 { 0 } else { read_usize(G_STADIUM_TABLE).wrapping_add((id as usize).wrapping_mul(0x4e)) }); // cqx13: the venue's stadium
            i += 1;
        }
    }
    if game_data_version() >= 0x3e029 && exe::crt_fread(G_STADIUM_STASHED_VENUE_ID, 4, 1, f) != 1 {
        show_error_at(0xa835b0, 0x15c);
        return 0;
    }
    exe::crt_fclose(f);
    1
}

/// Comparer for `stadium_load_from_temp_file`, side by side over the same object: the original first, then the copy,
/// which reads the same file again into new arrays; the counts, capacities and flag (+4..+0x19 without the venue
/// pointer), the bookings, the venues and the stashed venue id must be equal, and the return; the copy's arrays are
/// freed and the original's put back.
unsafe extern "thiscall" fn compare_stadium_load_from_temp_file(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let read = |this: usize| -> Vec<u8> {
        let mut v: Vec<u8> = (4..0xcusize).chain(0x10..0x19).map(|k| read_u8(this + k)).collect();
        let (b, n) = (read_usize(this), read_i32(this + 4).clamp(0, 0x10000) as usize);
        if b != 0 {
            v.extend((0..12 * n).map(|k| read_u8(b + k)));
        }
        let (s, m) = (read_usize(this + 0xc), read_i32(this + 0x10).clamp(0, 0x10000) as usize);
        if s != 0 {
            v.extend((0..16 * m).map(|k| read_u8(s + k)));
        }
        v.extend_from_slice(&read_u32(G_STADIUM_STASHED_VENUE_ID).to_le_bytes());
        v
    };
    let original = read(this);
    let (b, s) = (read_usize(this), read_usize(this + 0xc));
    let c = stadium_load_from_temp_file(this);
    let copy = read(this);
    for (o, mine) in [(0usize, b), (0xc, s)] {
        let p = read_usize(this + o);
        if p != mine && p != 0 {
            exe::crt_free(p);
        }
        write_usize(this + o, mine);
    }
    SCOREBOARD.record(r == c && copy == original, || format!("stadium_load_from_temp_file({:#x}): original {} copy {}, bytes equal {}", this, r, c, copy == original));
    r
}
