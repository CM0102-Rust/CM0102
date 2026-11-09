use super::*;

/// The constructor of the match statistics: checks for the CM01/02 CD and builds the statistics: loads from the save or
/// creates the index, the buckets and the zeroed temporary file. Returns `this`.
///
/// It checks the CD: among the drives (GetLogicalDriveStringsA, 200) the first CD drive (GetDriveTypeA 5) with
/// "<root>cm0102.exe" (`0x4c0aa0`; on the others it closes the search, `0x4c0c10`); without it, the "Insert CD" box and
/// it tries again, forever.
///
/// With `zip` and the save loaded (`0x72aa80`), done; otherwise the index of `n` entries (`+4`, `[this]` with n × 12
/// bytes; out of memory, error `0x62` and the game closes), `+0x1c` = n, the four bucket cursors (`+8`: k × n / 4) and
/// "match_stats_matches.tmp" (CM3_TEMP, "wb") with `n` zeroed records of `0xd7a` bytes (error `0x71` if it cannot open,
/// `0x7e` if the write fails), `+0x18` = 0. The local record is created and destroyed by the runtime's vector iterators
/// (`0x9463af`/`0x9462bb`) as in the original.
/// Original: `Match_Stats::Match_Stats` (0x729000, match_stats.cpp, thiscall).
///
/// [pt] { 0x729000 } (thiscall, ret 8, match_stats.cpp): o construtor das estatísticas da partida.
/// Confere o CD: entre as unidades (GetLogicalDriveStringsA, 200) a primeira de CD (GetDriveTypeA 5)
/// com "<raiz>cm0102.exe" (0x4c0aa0; nas outras fecha a busca, 0x4c0c10); sem ela, a caixa "Insert
/// CD" e tenta de novo, para sempre. Com `zip` e a carga do save (0x72aa80) pronto; senão o índice de
/// `n` entradas (+4, [this] com n × 12 bytes; sem memória, erro 0x62 e o jogo fecha), +0x1c = n, os
/// quatro cursores dos baldes (+8: k × n / 4) e o "match_stats_matches.tmp" (CM3_TEMP, "wb") com `n`
/// registros zerados de 0xd7a bytes (erro 0x71 sem abrir, 0x7e se a escrita falha), +0x18 = 0. O
/// registro local nasce e morre pelos iteradores de vetor do runtime (0x9463af/0x9462bb) como na
/// original. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construct_match_stats(stats: usize, zip: usize, n: i32) -> usize {
    crate::comparison::branch("statistics build");
    let units_digit: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(*(0x9670f8 as *const usize));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(*(0x967100 as *const usize));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let d = |o: usize| (stats + o) as *mut i32;

    let mut record = [0u8; 0xd7a];
    let r = record.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(r + match_record::HOME_TEAM, 0x625, 2, 0x453c30, 0x453c10);

    // the working exe (cm0102-1080) comes without the CD requirement: the jne on the drive type (0x729089)
    // became a nop and the found-check test (0x7290cb, 85 c0) became xor (31 c0 in cm0102-1080; 33 c0 is the other
    // encoding); the copy reads the bytes of the exe itself
    let any_unit = *(0x729089 as *const u8) == 0x90;
    let always_found = *(0x7290cb as *const u8) != 0x85; // the original has `test eax, eax` (85 c0); anything else is a no-CD patch (xor 31 c0 or 33 c0)
    'cd: loop {
        // simplification: the buffer starts zeroed; the original would only read garbage with more than 200 bytes of units
        // (more than 50 letters), impossible
        let mut list = [0u8; 0x100];
        if units_digit(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let raiz = list.as_ptr() as usize + i;
                if any_unit || kind(raiz) == 5 {
                    let mut name = [0u8; 0x104];
                    let mut found = [0u8; 0x114];
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, raiz, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let ach = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, ach, 0) == 0 || always_found {
                        break 'cd;
                    }
                    exe::file_find_close(ach);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32 units
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let mut title = [0u8; 0x100];
        let mut msg = [0u8; 0x100];
        text(title.as_mut_ptr() as usize, 0x987df4);
        text(msg.as_mut_ptr() as usize, 0x987dc4);
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }

    if zip != 0 && exe::match_stats_load(stats, zip) != 0 {
        exe::crt_vector_destructor_iterator(r + match_record::HOME_TEAM, 0x625, 2, 0x453c10);
        return stats;
    }
    *d(4) = n;
    let t = exe::crt_operator_new((n as u32).wrapping_mul(12));
    *(stats as *mut usize) = t;
    if t == 0 {
        fatal_error_at(0xa17ba0, 0x62);
    }
    *d(match_stats::CAPACITY) = n;
    let mut sum = 0i32;
    for k in 0..4 {
        *d(8 + 4 * k) = sum / 4;
        sum = sum.wrapping_add(n);
    }
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xa17b88), 0x9a7f28);
    if f == 0 {
        show_error_at(0xa17ba0, 0x71);
    } else {
        core::ptr::write_bytes(r as *mut u8, 0, 0xd7a);
        let mut ok = true;
        let mut i = 0;
        while i < n {
            if exe::crt_fwrite(r, 0xd7a, 1, f) != 1 {
                show_error_at(0xa17ba0, 0x7e);
                exe::crt_fclose(f);
                ok = false;
                break;
            }
            i += 1;
        }
        if ok {
            exe::crt_fclose(f);
            *d(match_stats::EVENT_FILE_NUMBER) = 0;
        }
    }
    exe::crt_vector_destructor_iterator(r + match_record::HOME_TEAM, 0x625, 2, 0x453c10);
    stats
}

/// frees the record index and clears its pointer and count
/// Original: `Match_Stats::~Match_Stats` (0x7293c0, match_stats.cpp, thiscall).
///
/// [pt] { 0x7293c0 } (thiscall, match_stats.cpp): o destrutor das estatísticas da partida: libera o índice
/// ([this]) e zera o ponteiro e a quantidade (+4).
pub(crate) unsafe extern "thiscall" fn destroy_match_stats(stats: usize) {
    crate::comparison::branch("statistics tear down");
    let t = *(stats as *const usize);
    if t != 0 {
        exe::crt_operator_delete(t);
        *(stats as *mut usize) = 0;
        *((stats + match_stats::COUNT) as *mut u32) = 0;
    }
}

// simplification: the constructor checks the CD (a box that waits for the click), allocates the index and recreates the file
// temporary; the destructor frees the index: running twice is not possible, so the comparers only call the
// original; the copies are proved in Rust mode (the branches: the automatic career creates the game and goes through the
// construtor).

/// Fills the match statistics record: clubs, dates, attendance, gate receipts, weather, sorted events, formation,
/// captain and the 20 players of each side. Returns 1.
///
/// Convention: thiscall, ret `0xc`, `match_stats.cpp`; `this` is not used. Fills statistics record `reg` of match `m`
/// (fixture in `[m+0x4792]`): clubs, dates, attendance (`+0x34`) and gate receipts (`+0x38`, `0x59af40` only without
/// `sem_renda` (no receipts)), the score by half, the stadium, the referee, the minute / 11, the weather (`0x923010` at
/// `+0xd76`), up to 50 events of 4 bytes at `+0x57` (sorted by `0x72b160`, terminated by `0xff`) and, per team
/// (`+0x11f` + `0x625` × t), the statistics, the formation (`0x5b2ae0`), the captain (`0x897590` with `sem_renda`) and
/// the 20 players of `0x49` bytes (shirt number via `0x85a760`, assigning one with `0x85a470` when missing). Null match
/// or record: the original routine runs (error box).
/// Original: `Match_Stats::fill_match_record` (0x729ff0, match_screens.cpp, thiscall).
///
/// [pt] { 0x729ff0 } (thiscall, ret 0xc, match_stats.cpp; `this` não é usado): preenche o registro de
/// estatística `reg` da partida `m` (jogo em [m+0x4792]): clubes, datas, público (+0x34) e renda
/// (+0x38, 0x59af40 só sem `sem_renda`), o placar por tempo, o estádio, o árbitro, o minuto / 11, o
/// clima (0x923010 em +0xd76), até 50 eventos de 4 bytes em +0x57 (ordenados por 0x72b160, fim 0xff)
/// e, por time (+0x11f + 0x625 × t), as estatísticas, a formação (0x5b2ae0), o capitão (0x897590 com
/// `sem_renda`) e os 20 jogadores de 0x49 bytes (número da camisa por 0x85a760, dando um com
/// 0x85a470 quando falta). Partida ou registro nulos: a original (caixa de erro). Devolve 1.
pub(crate) unsafe extern "thiscall" fn fill_match_record(stats: usize, m: usize, reg: usize, sem_renda: u32) -> u32 {
    if m == 0 || reg == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_FILL_MATCH_RECORD)(stats, m, reg, sem_renda);
    }
    let b = |p: usize| *(p as *const u8);
    let w = |p: usize| core::ptr::read_unaligned(p as *const u16);
    let d = |p: usize| core::ptr::read_unaligned(p as *const u32);
    let pb = |p: usize, v: u8| *(p as *mut u8) = v;
    let pw = |p: usize, v: u16| core::ptr::write_unaligned(p as *mut u16, v);
    let pd = |p: usize, v: u32| core::ptr::write_unaligned(p as *mut u32, v);
    // the static date (day 0x29 of 1972) of the first call: the mark before construction, as in the original
    if b(G_MATCH_STATS_FILL_RECORD_STATIC_DATE_GUARD) & 1 == 0 {
        pb(G_MATCH_STATS_FILL_RECORD_STATIC_DATE_GUARD, b(G_MATCH_STATS_FILL_RECORD_STATIC_DATE_GUARD) | 1);
        exe::new_date(G_MATCH_STATS_FILL_RECORD_STATIC_DATE, 0x29, 0x7b4);
        exe::crt_atexit(0x72aa70);
    }
    let game = || read_u32(m + engine::MATCH) as usize;
    pd(reg, d(game() + 0xc));
    pd(reg + match_record::AWAY_CLUB_ID, d(game() + 0x10));
    pw(reg + match_record::DAY, w(game() + 0x2a));
    pw(reg + match_record::YEAR, w(game() + 0x28));
    pd(reg + match_record::ATTENDANCE, read_u32(m + engine::ATTENDANCE));
    for k in 0..5 {
        pw(reg + match_record::BALL_TIME_THIRD_1 + 2 * k, w(m + engine::BALL_TIME_THIRD_1 + 2 * k));
        pw(reg + match_record::BALL_TIME_THIRD_2 + 2 * k, w(m + engine::BALL_TIME_THIRD_2 + 2 * k));
        pw(reg + match_record::BALL_TIME_THIRD_3 + 2 * k, w(m + engine::BALL_TIME_THIRD_3 + 2 * k));
        pw(reg + match_record::BALL_IN_PLAY_TIME + 2 * k, w(m + engine::BALL_IN_PLAY_TIME + 2 * k));
    }
    pd(reg + match_record::GATE_RECEIPTS, 0);
    let stadium = read_u32(m + engine::ADJUSTED_PLAYER_ID) as i32;
    let p = if stadium > -1 { d((d(G_STAFF_TABLE) as i32 + stadium * 0x6e + 0x61) as usize) as usize } else { 0 };
    pd(reg + match_record::MAN_OF_THE_MATCH_ID, if p != 0 { d(p) } else { u32::MAX });
    let q = read_u32(m + engine::REFEREE_ID) as usize;
    pd(reg + match_record::REFEREE_ID, if q != 0 && d(q) as i32 > -1 { d(q) } else { u32::MAX });
    pw(reg + match_record::MINUTE, (read_u16(m + engine::CLOCK) as i16 as i32 / 11) as u16);
    pw(reg + match_record::PERIOD_END, read_u16(m + engine::PERIOD_END));
    pw(reg + match_record::SUB_TICK, read_u16(m + engine::SUB_TICK));
    pb(reg + match_record::NO_STATISTICS, b(m));
    pd(reg + match_record::SHOOTOUT, matches!(read_u8(m + engine::SHOOTOUT_STATE), 1 | 4) as u32);
    exe::weather_get_match_weather(d(G_WEATHER) as usize, reg + match_record::WEATHER, game());
    if sem_renda == 0 {
        pd(reg + match_record::GATE_RECEIPTS, exe::financetable_calc_gate_receipts(d(G_FINANCE_TABLE) as usize, game(), read_u32(reg + match_record::ATTENDANCE)));
    }
    // the events: move code → type (1..8); 7 and 8 mark +0x53 and store the order in place of the minute
    let ev = read_u32(m + engine::COMMENTATOR) as usize;
    let mut n: u32 = 0;
    if ev != 0 && (read_u16(ev + match_events_fields::COUNT) as i16) > 0 {
        let mut i: i32 = 0;
        while i < read_u16(ev + match_events_fields::COUNT) as i16 as i32 {
            let e = ev + match_events_fields::ENTRIES + (i as i16 as i32 * 14) as usize;
            let c = w(e) as i16 as i32;
            let kind: u8 = match c {
                0x1f56 | 0x1f57 => 6,
                0x1fcc | 0x2013..=0x201b | 0x2114 => 1,
                0x1fcd => 2,
                0x1fce => 4,
                0x1fcf => 3,
                0x1ff5..=0x1ff8 => 5,
                0x2000 => 7,
                0x2008 => 8,
                _ => 0,
            };
            if kind != 0 && n != 0x32 {
                let special = kind >= 7;
                if special {
                    pd(reg + match_record::SHOOTOUT, 1);
                }
                let s = reg + match_record::EVENTS + 4 * n as usize;
                pb(s, (b(e + 6) != 0) as u8 | ((n as u8).wrapping_add(1) << 1));
                pb(s + 1, b(e + 5));
                pb(s + 2, kind);
                pb(s + 3, if special { n as u8 } else { b(e + 4) });
                n += 1;
            }
            i += 1;
        }
        if n > 0 {
            exe::crt_qsort(reg + match_record::EVENTS, n, 4, 0x72b160);
            for k in 0..n as usize {
                pb(reg + match_record::EVENTS + 4 * k, b(reg + match_record::EVENTS + 4 * k) & 1);
            }
        }
    }
    if n < 0x32 {
        pb(reg + match_record::EVENTS + 4 * n as usize, 0xff);
    }
    for t in 0..2usize {
        let tm = reg + 0x625 * t + 0x11f;
        let s = m + 0x36 * t + 0xf5bc;
        let id = d(reg + match_record::AWAY_CLUB_ID * t) as i32;
        let club = (d(G_CLUB_TABLE) as i32 + id * 0x245) as usize;
        let uses = b(game() + 0x40 + t) != 0 && exe::squad_manager_club_uses_squad_numbers(d(G_SQUAD_MANAGER) as usize, club, 0) != 0;
        for (o, de) in [(0, 1), (1, 0), (3, 4), (4, 5), (2, 6), (5, 7), (6, 8), (7, 9), (8, 2), (9, 3), (0xa, 0xb),
                        (0xb, 0xa), (0xc, 0xc), (0xd, 0xd), (0xe, 0xe), (0xf, 0x10), (0x10, 0xf)] {
            pb(tm + o, b(s + de));
        }
        for (o, de) in [(0x11, 0x26), (0x13, 0x28), (0x15, 0x2a), (0x17, 0x2c), (0x19, 0x2e), (0x1b, 0x30), (0x1d, 0x34)] {
            pw(tm + o, w(s + de));
        }
        for k in 0..5 {
            pb(tm + match_record_team::POSSESSION + k, b(s + 0x12 + k));
            pb(tm + match_record_team::ZONE_1 + k, b(s + 0x21 + k));
            pb(tm + match_record_team::ZONE_2 + k, b(s + 0x1c + k));
            pb(tm + match_record_team::ZONE_3 + k, b(s + 0x17 + k));
        }
        pb(tm + match_record_team::KIT, b(m + engine::SIDE_KIT + t));
        if sem_renda != 0 {
            let away = (b(game() + 0x40 + t) == 0) as i32;
            let c = exe::tactics_find_captain_slot(d(G_TACTICS) as usize, d(game() + 0x1c + 4 * t) as usize, (away) as u32, (0) as u32) as i32;
            pb(tm + match_record_team::CAPTAIN, b((m as isize + (c + t as i32 * 20) as isize * 0x1be + 0x4796) as usize));
        } else {
            pb(tm + match_record_team::CAPTAIN, b(s + 0x11));
        }
        let f = m + 0x18e3 * t;
        for k in 0..11 {
            pw(tm + match_record_team::FORMATION_X + 2 * k, exe::formation_get_slot_position(f + 0xc3a8, (k as i32) as u32, (0) as u32));
            pw(tm + match_record_team::FORMATION_Y + 2 * k, exe::formation_get_slot_position(f + 0xc3a8, (k as i32) as u32, (1) as u32));
        }
        pd(tm + 0x61, d(f + 0xc92c));
        let g = m + 0xd0 * t;
        let tecnico = read_u32(g + engine::SIDE0_MANAGER) as usize;
        let last = if tecnico != 0 {
            pd(tm + match_record_team::MANAGER_ID, d(tecnico));
            pw(tm + match_record_team::MANAGER_CURRENT_REPUTATION, read_u16(g + engine::MATCH_MANAGERS));
            pw(tm + match_record_team::MANAGER_HOME_REPUTATION, read_u16(g + engine::SIDE0_MANAGER_HOME_REPUTATION));
            read_u16(g + engine::SIDE0_MANAGER_WORLD_REPUTATION)
        } else {
            pd(tm + match_record_team::MANAGER_ID, u32::MAX);
            pw(tm + match_record_team::MANAGER_CURRENT_REPUTATION, 0xffff);
            pw(tm + match_record_team::MANAGER_HOME_REPUTATION, 0xffff);
            0xffff
        };
        pw(tm + match_record_team::MANAGER_WORLD_REPUTATION, last);
        pw(tm + match_record_team::SIDE_REPUTATION, w(m + 2 * t + 0x1d6));
        for k in 0..20usize {
            let x = tm + 0x8d + 0x49 * k;
            let y = m + 0x22d8 * t + 0x47ff + 0x1be * k;
            let manager = || d(G_SQUAD_MANAGER) as usize;
            if uses {
                if d(y + 4) == 0 {
                    pb(x, 0);
                } else {
                    let mut v = (exe::squad_manager_get_squad_number(manager(), d(y) as usize, club) as i32).wrapping_abs() as u8;
                    pb(x, v);
                    if v == 0 {
                        exe::squad_manager_allocate_squad_number(manager(), club, d(y) as usize);
                        v = (exe::squad_manager_get_squad_number(manager(), d(y) as usize, club) as i32).wrapping_abs() as u8;
                        pb(x, v);
                    }
                }
            } else {
                pb(x, if d(y) == 0 { 0 } else { (k as u8) + 1 });
            }
            pd(x - 0x1c, if d(y) != 0 { d(d(y) as usize) } else { u32::MAX });
            pb(x + 1, b(y - 0x50));
            pb(x + 2, b(y - 0x4f));
            if d(x - 0x1c) as i32 <= -1 {
                continue;
            }
            pd(x - 0x18, if d(y + 4) != 0 { d(d(y + 4) as usize) } else { u32::MAX });
            pd(x - 0x14, d(y - 0xc));
            pd(x - 0x10, d(y - 8));
            pd(x - 0xc, d(y - 4));
            let mut copy = [0u8; 8];
            exe::date_operator_3d(x - 8, copy.as_mut_ptr() as usize, if d(y) != 0 { d(y) as usize + 0x10 } else { G_MATCH_STATS_FILL_RECORD_STATIC_DATE });
            pb(x + 3, b(y - 0x4d));
            // simplification: fld f32 × fmul f64 (0.01) in 80 bits on the x87; in f64 it gives the same truncation (f32 × 0.01 never falls within 2^-53 of an integer)
            let rating = core::ptr::read_unaligned((y - 0x1c) as *const f32) as f64 * core::ptr::read_unaligned(0x967898 as *const f64);
            pb(x + 4, ftol(rating) as u8);
            for (o, de) in [(5, 0x5d), (6, 0x5a), (7, 0x5c), (8, 0x5b), (9, 0x5f), (0xa, 0x5e), (0xb, 0x59), (0xc, 0x58),
                            (0xd, 0x55), (0xe, 0x54), (0xf, 0x56), (0x10, 0x45), (0x11, 0x46), (0x12, 0x44), (0x13, 0x43),
                            (0x16, 0x4a), (0x17, 0x47), (0x14, 0x49)] {
                pb(x + o, b(y - de));
            }
            pw(x + 0x1a, ftol(core::ptr::read_unaligned((y - 0x18) as *const f32) as f64) as u16);
            pb(x + 0x15, b(y - 0x52));
            pb(x + 0x18, b(y - 0x48));
            pw(x + 0x1c, w(y - 0x22));
            pw(x + 0x1e, w(y - 0x20));
            pw(x + 0x20, w(y - 0x1e));
            pb(x + 0x19, b(y - 0x4e));
            for k2 in 0..9 {
                pb(x + 0x22 + k2, b(y - 0x68 + k2));
            }
            pb(x + 0x2b, b(y - 0x57));
        }
    }
    1
}

/// qsort comparer of the 4-byte match events: orders two events by type group, then by minute and arrival order.
///
/// Same group (types 1..3 together, 7..8 together, or the same type +2): tie broken by the minute (`+3`, unsigned) and
/// then by arrival order (`+0` signed, half); different groups: the difference of the types.
/// Original: `match_stats_compare_events` (0x72b160, match_screens.cpp, cdecl).
///
/// [pt] { 0x72b160 } (cdecl, match_stats.cpp, comparador do qsort dos eventos de 4 bytes): mesmo grupo
/// (tipos 1..3 entre si, 7..8 entre si, ou o mesmo tipo +2) desempata pelo minuto (+3, sem sinal) e
/// depois pela ordem de chegada (+0 com sinal, metade); grupos diferentes: a diferença dos tipos.
pub(crate) unsafe extern "C" fn match_stats_compare_events(a: *const u8, b: *const u8) -> i32 {
    let (ta, tb) = (*a.add(2), *b.add(2));
    let same = (matches!(ta, 1..=3) && matches!(tb, 1..=3)) || (matches!(ta, 7 | 8) && matches!(tb, 7 | 8));
    if !same {
        let r = ta as i8 as i32 - tb as i8 as i32;
        if r != 0 {
            return r;
        }
    }
    let r = *a.add(3) as i32 - *b.add(3) as i32;
    if r != 0 {
        return r;
    }
    (*a as i8 as i32 >> 1) - (*b as i8 as i32 >> 1)
}

/// Os 25 registros de aba (0xd4 bytes: +4 o título, +0xcc a marca, +0xd0 o valor) que a barra de abas
/// (`exe::gui_add_tab_bar`) lê, construídos (0x4568f0) e destruídos (0x5a53c0) pelos iteradores do runtime
/// (`exe::crt_vector_constructor_iterator`/`exe::crt_vector_destructor_iterator`) como na original.
pub(crate) type MatchTabs = [u32; 0x14b4 / 4];

/// Constructs the 25 match tab objects (0xd4 bytes each) with the CRT vector constructor.
pub(crate) unsafe fn construct_match_tabs(tabs: &mut MatchTabs) {
    exe::crt_vector_constructor_iterator(tabs.as_mut_ptr() as usize, 0xd4, 0x19, 0x4568f0, 0x5a53c0);
}

/// Destroys the 25 match tab objects with the CRT vector destructor.
pub(crate) unsafe fn destruct_match_tabs(tabs: &mut MatchTabs) {
    exe::crt_vector_destructor_iterator(tabs.as_mut_ptr() as usize, 0xd4, 0x19, 0x5a53c0);
}

/// Address of the title text (+4) of match tab k.
///
/// [pt] O título da aba `k` (texto em +4) e a marca e o valor dela.
pub(crate) unsafe fn match_tab_title(tabs: &mut MatchTabs, k: usize) -> usize {
    tabs.as_mut_ptr() as usize + k * 0xd4 + 4
}
/// Sets the flag and value dwords of match tab k.
pub(crate) unsafe fn set_match_tab_flags(tabs: &mut MatchTabs, k: usize, mark: u32, value: u32) {
    tabs[k * 53 + 51] = mark;
    tabs[k * 53 + 52] = value;
}

/// Frees the match state copy in screen param 0x1d (destroys its two team blocks, deletes it) and clears the param.
///
/// [pt] Libera a foto da partida do parâmetro 0x1d (0xd7a bytes; o vetor de 2 x 0x625 em +0x11f, 0x453c10)
/// e zera o parâmetro.
pub(crate) unsafe fn free_match_snapshot() {
    if get_screen_param(0x1d) != 0 {
        let f = get_screen_param(0x1d);
        if f != 0 {
            exe::crt_vector_destructor_iterator(f + match_record::HOME_TEAM, 0x625, 2, 0x453c10);
            exe::crt_operator_delete(f);
        }
        set_screen_param(0x1d, 0);
    }
}

/// Match screen background: stadium picture, else competition, else club then random; returns the last call's eax.
///
/// [pt] O fundo das telas da partida: a foto (0x41c670), senão o da competição (0x41c410), senão o do clube
/// (0x41c250 e 0x41bfb0). Devolve o `eax` da última chamada (a original empilha o byte da cor do
/// uniforme por cima dele).
pub(crate) unsafe fn set_match_background(photo: usize, competition: usize, club: usize) -> usize {
    if photo != 0 {
        let r = exe::background_set_stadium_picture(photo, 0, 1);
        if r != 0 {
            return r;
        }
    }
    let r = (exe::background_set_comp_picture(competition, 0, 1) as usize);
    if r != 0 {
        return r;
    }
    exe::background_set_club_picture(club, 0, 1);
    exe::background_set_random_picture()
}

/// Shows the match_screens.cpp "Error" box at the given line (game continues) and clears the left mouse flag.
///
/// [pt] A caixa "Error" de match_screens.cpp na linha `linha` (sem parar o jogo) e [0xb67a34] = 0.
pub(crate) unsafe fn match_screens_error_box(row: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa161d4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
}

/// Formats the team name into 0xde1f64 for the match header: name only, or with B-team / reserves suffix.
///
/// [pt] O nome do time no texto 0xde1f64 do cabeçalho da partida: só o nome ("  %s") com `so_o_nome`;
/// senão a seleção B ("'B'", 0x53b900) ou o time reserva ("Res.").
pub(crate) unsafe fn format_match_team_name(team: Club, so_name: u8) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if so_name != 0 {
        format(0xde1f64, 0x987ff0, team.addr() + club::SHORT_NAME);
    } else if exe::database_is_national_team(team) != 0 {
        let c = team.gender_name() as i32;
        text(0xde1f64, 0xa164a4, c, c, team.addr() + club::NAME);
    } else {
        let c = (team.short_gender_name() as i8) as i32;
        text(0xde1f64, 0xa16478, c, c, team.addr() + club::SHORT_NAME);
    }
}

/// Adds a commentary speed row `valor` to list `lista`, flagged (0x2002) when it is the current one `v` (the return
/// value of `exe::human_manager_get_commentary_speed`).
///
/// The original pushes one of the three colours (0xae31aa, 0xaebdec, 0xaebdea, in that order) in the `eax` left over
/// from `exe::human_manager_get_commentary_speed` (with the high 16 bits known) and the others in dirty registers (extended here); `sujo` says which, as
/// (flagged, not flagged).
///
/// [pt] Uma linha da velocidade da narração `valor` na lista `lista`, marcada (0x2002) quando é a atual `v`
/// (a devolução de 0x5fcff0). A original empilha uma das três cores (0xae31aa, 0xaebdec, 0xaebdea, nessa
/// ordem) no `eax` que sobrou de 0x5fcff0 (os 16 bits de cima conhecidos) e as outras em registradores
/// sujos (aqui estendidas); `sujo` diz qual, (marcada, não marcada).
pub(crate) unsafe fn add_commentary_speed_row(list: usize, value: u16, v: u32, sujo: (usize, usize)) -> usize {
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let marked = v as u16 == value;
    let k = if marked { sujo.0 } else { sujo.1 };
    let enderecos = [0xae31aausize, G_GUI_COLOUR_YELLOW, G_GUI_COLOUR_BLACK];
    let colour = |i: usize| if i == k { colour_with_high_bits(v as usize, enderecos[i]) } else { colour_word(enderecos[i]) };
    let mark = if marked { 0x2002 } else { 2 };
    row(current_human_display(), mark, 0, 0, 0, 0, 0, 0, 0x10, colour(0), colour(1), 1, 1, colour(2), 0xde1f64, 0, 0xc, value as usize, list)
}

/// Builds the live match screen: team headers and score, commentary, clock, control, tactics buttons, commentary
/// speeds, tabs, the selected tab panel, the stadium and the commentary panel.
///
/// Convention: cdecl, SEH, `match_screens.cpp`, ~5 KB. It sets flag `+0xea` of the human's record (`0x765480`) and
/// frees the snapshot of param `0x1d`. Mode 1 (param 1): only the empty background figure (`0x41c0d0`).
///
/// Otherwise: it removes the "left" mark (−911) of the human among the spectators (`[partida+0x5b]` `+0xc5` + 4h,
/// counting in `+0x108`); the background; the headers of the two teams (kit colours `0x53b640`, name, club box, score
/// or "A" for abandoned) and the aggregate (`[+0x4f]` ≥ 0; added to the score outside mode 2 of `[+0x67]`); the
/// commentary area (background "match.mbr"), the clock area (minute `[+0x4d]` in param 3), the "<<<"/">>>" arrows (or
/// the symbols of font `[0x9cc2c4]`), the control (`0x725a70`, param 4) and the tactics buttons of both teams (params 5
/// and 6; the one in param `0x15` marked); with the human in charge of one of the teams (`0x600c90`) or with no human
/// at all (`0x600a50`), the commentary speeds (`0x5fcff0`; the current one marked, focus `[Display+0x12f4ff]` and the
/// commentator volume `[+0x2db0]`).
///
/// Finally record `+0` = 3, the tabs (Overview, Stats, Zones, Report, the team statistics, Ratings, Latest Scores and,
/// for a real league without national teams, League Table), −1 in param 8, the tab panel (param 2), the stadium and,
/// with param 8 at −1, the commentary panel (with the highlight of param 7 narrated by `0x6d7b00`) in param 8.
/// Original: `build_match_screen` (0x718e90, match_screens.cpp, cdecl).
///
/// [pt] { 0x718e90 } (cdecl, SEH, match_screens.cpp, ~5 KB): monta a tela da partida ao vivo. Liga a marca
/// +0xea da ficha do humano (0x765480) e libera a foto do parâmetro 0x1d. Modo 1 (parâmetro 1): só a
/// figura de fundo vazia (0x41c0d0). Senão: tira a marca de "saiu" (−911) do humano nos espectadores
/// ([partida+0x5b] +0xc5 + 4h, contando em +0x108); o fundo; os cabeçalhos dos dois times (cores do
/// uniforme 0x53b640, nome, caixa do clube, placar ou "A" de abandonada) e o agregado ([+0x4f] ≥ 0;
/// somado ao placar fora do modo 2 de [+0x67]); a área da narração (fundo "match.mbr"), a do relógio
/// (minuto [+0x4d] no parâmetro 3), as setas "<<<"/">>>" (ou os símbolos da fonte [0x9cc2c4]), o
/// controle (0x725a70, parâmetro 4) e os botões táticos dos dois times (parâmetros 5 e 6; o do
/// parâmetro 0x15 marcado); com o humano no comando de um dos times (0x600c90) ou sem humano nenhum
/// (0x600a50), as velocidades da narração (0x5fcff0; a atual marcada, o foco [Display+0x12f4ff] e o
/// volume do narrador [+0x2db0]). Por fim a ficha +0 = 3, as abas (Overview, Stats, Zones, Report, as
/// estatísticas dos times, Ratings, Latest Scores e, para liga de verdade sem seleções, League Table),
/// −1 no parâmetro 8, o painel da aba (parâmetro 2), o estádio e, com o parâmetro 8 em −1, o painel de
/// narração (com o lance do parâmetro 7 narrado por 0x6d7b00) no parâmetro 8.
pub(crate) unsafe extern "C" fn build_match_screen() {
    crate::comparison::branch("screens build the match");
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    const MENOS1: usize = usize::MAX;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text_of_manager: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let colours: unsafe extern "C" fn(usize, usize, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let sx = |a: usize| *(a as *const i8) as i32;
    let velocidade = || {
        (exe::human_manager_get_commentary_speed(*(G_HUMAN_MANAGER as *const usize), 0) as u32)
    };
    let write_item = |k: u32, r: usize| set_screen_param(k, r as i16 as isize as usize);

    let mut tabs: MatchTabs = [0; 0x14b4 / 4];
    construct_match_tabs(&mut tabs);
    *((exe::get_human_manager_record(0) + 0xea) as *mut u32) = 1;
    free_match_snapshot();
    let p = get_screen_param(0);
    let mode = get_screen_param(1) as u8;
    let aba = get_screen_param(2) as u8;
    let p15 = get_screen_param(0x15);
    get_screen_param(0xe);
    if mode == 1 {
        exe::background_set_picture(0);
        destruct_match_tabs(&mut tabs);
        return;
    }
    let espectadores = read_usize(p + live_match::SPECTATORS);
    let st = read_usize(p + live_match::RECORD);
    let h = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let mark = (espectadores as isize + 0xc5 + h * 4) as *mut i32;
    if core::ptr::read_unaligned(mark) == -911 {
        core::ptr::write_unaligned(mark, 0);
        let n = (espectadores + match_spectators::COUNT) as *mut u8;
        *n = (*n).wrapping_sub(1);
    }
    set_match_background(read_usize(p + live_match::STADIUM), read_usize(p + live_match::COMPETITION), read_usize(p + live_match::HOME_CLUB));

    // the headers: the u16 colours (the rest of the original’s variables is stack garbage, here 0); the byte of the
    // uniforme vai num registrador sujo (estendido)
    let (mut cor_a, mut cor_b) = (0u32, 0u32);
    colours(read_usize(p + live_match::HOME_CLUB), read_u8(st + match_record::HOME_KIT) as usize, &mut cor_a, &mut cor_b);
    format_match_team_name(Club(read_usize(p + live_match::HOME_CLUB)), read_u8(p + live_match::HOME_SIDE_MARK));
    text_of_manager(sm, 0x64, 0xa, 0x1bc, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x64, 0xa, 0x1bc, 0x46, 1, 0, 1, 7, 0, 0, 1, read_usize(p + live_match::HOME_CLUB));
    let r = if read_u8(st + match_record::HOME_TEAM) == 0xfd { format(BUF, 0x9b760c) } else { format(BUF, 0x98806c, read_u8(st + match_record::HOME_TEAM) as u32) };
    text_of_manager(sm, 0x18f, 0xf, 0x1b7, 0x41, 0x30, colour_word(G_GUI_COLOUR_PALE_GREY), 0xc, 6, colour_with_high_bits(r as usize, G_GUI_COLOUR_BLACK), BUF, 1);
    colours(read_usize(p + live_match::AWAY_CLUB), read_u8(st + match_record::AWAY_KIT) as usize, &mut cor_a, &mut cor_b);
    format_match_team_name(Club(read_usize(p + live_match::AWAY_CLUB)), read_u8(p + live_match::AWAY_SIDE_MARK));
    text_of_manager(sm, 0x1be, 0xa, 0x316, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x1be, 0xa, 0x316, 0x46, 1, 0, 1, 7, 0, 0, 1, read_usize(p + live_match::AWAY_CLUB));
    let r = if read_u8(st + match_record::HOME_TEAM) == 0xfd { format(BUF, 0x9b760c) } else { format(BUF, 0x98806c, read_u8(st + match_record::AWAY_TEAM) as u32) };
    text_of_manager(sm, 0x2e9, 0xf, 0x311, 0x41, 0x30, colour_word(G_GUI_COLOUR_PALE_GREY), 0xc, 6, colour_with_high_bits(r as usize, G_GUI_COLOUR_BLACK), BUF, 1);
    let aggregate = sx(p + live_match::HOME_AGGREGATE);
    if aggregate > -1 {
        let (a, b) = if read_u8(p + live_match::STATE) != 2 {
            (aggregate + read_u8(st + match_record::HOME_TEAM) as i32, sx(p + live_match::AWAY_AGGREGATE) + read_u8(st + match_record::AWAY_TEAM) as i32)
        } else {
            (aggregate, sx(p + live_match::AWAY_AGGREGATE))
        };
        text(BUF, 0xa16454, a, b);
        text_of_manager(sm, 0x1c3, 0x37, 0x2e4, 0x41, 1, 0, 0x40, 1, cor_a as usize, BUF, 1);
    }

    // the commentary, the clock and the arrows
    // y2 and the push immediate at 0x719345: 0x257 in the 800x600 exe, 0x437 in cm0102-1080 (resolucao.py)
    let y2 = core::ptr::read_unaligned(0x719346 as *const u32) as usize;
    let narracao = list(current_human_display(), 0, 0, 0x59, y2, 1, 0, 1, 0, 1, 0, MENOS1);
    let bottom = exe::utils_build_file_path(0x9b5348, 0xa16448);
    row(current_human_display(), 0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, bottom, 0, 0, 0, narracao);
    let clock = list(current_human_display(), 5, 0xa, 0x55, 0x24e, 1, 0, 0xd, 0, 1, 0, MENOS1);
    format(BUF, 0x98806c, *((st + match_record::MINUTE) as *const i16) as i32);
    let r = text_of_cell(sm, clock, 0, 0, 0x1061, 0, 0xc, 5, colour_word(G_GUI_COLOUR_YELLOW), BUF);
    write_item(3, r);
    let setas = list(current_human_display(), 5, 0xa, 0x55, 0x24e, 2, 0, 0xd, 0, 1, 0, MENOS1);
    if *(G_FONT_MODE as *const i32) != -1 {
        format(BUF, 0x99ec1c, 0x28);
        cell(sm, setas, 0, 1, 0x1021, 0, 0xc, 0, colour_word(G_GUI_COLOUR_YELLOW), BUF, (-2isize) as usize, 0);
    } else {
        // the original pushes eax with [0x9cc2c4] = −1 above the colour
        cell(sm, setas, 0, 1, 0x1021, 0, 0xc, 1, colour_with_high_bits(MENOS1, G_GUI_COLOUR_PALE_GREY), 0xa16444, (-2isize) as usize, 0);
    }
    if *(G_FONT_MODE as *const i32) != -1 {
        format(BUF, 0x99ec1c, 0x29);
        cell(sm, setas, 1, 1, 0x1021, 0, 0xc, 0, colour_word(G_GUI_COLOUR_YELLOW), BUF, (-3isize) as usize, 0);
    } else {
        cell(sm, setas, 1, 1, 0x1021, 0, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0xa16440, (-3isize) as usize, 0);
    }

    // the match control (0x725a70 reads only the mode byte; the original’s color stays in the low 16 bits
    // of the variable that held the attendance)
    let mut marks: u32 = 0;
    exe::match_screens_get_match_control_label(mode as u32, espectadores, BUF, 0x7d0, &mut marks);
    let r = row(current_human_display(), 2, 0, 0, 0, 0, 0, 2, marks as usize, 0, 0, 0xc, 1, colour_with_high_bits(espectadores, G_GUI_COLOUR_PALE_GREY), BUF, 0, 2, 0, clock);
    write_item(4, r);

    // the tactical buttons of both teams (the one of parameter 0x15 marked)
    for (k, side, param) in [(3usize, 0x20usize, 5u32), (4, 0x24, 6)] {
        let t = Club(rd(p + side));
        let c = sx(t.addr() + club::SHORT_GENDER_NAME);
        text(BUF, 0xa16428, c, c, t.addr() + club::SHORT_NAME);
        let r = if t.addr() == p15 {
            // for the second team the original pushes the colour over the team pointer (eax)
            let colour = if k == 4 { colour_with_high_bits(t.addr(), G_GUI_COLOUR_PALE_GREY) } else { colour_word(G_GUI_COLOUR_PALE_GREY) };
            cell(sm, clock, 0, k, 0x1061, 0, 0xc, 1, colour, BUF, 1, t.addr())
        } else {
            cell(sm, clock, 0, k, 0x1021, 0, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 1, t.addr())
        };
        write_item(param, r);
    }

    // the commentary speeds
    let mostrar = exe::human_manager_current_controls_club(read_usize(p + live_match::HOME_CLUB), 0, 1) != 0
        || exe::human_manager_current_controls_club(read_usize(p + live_match::AWAY_CLUB), 0, 1) != 0
        || (exe::human_manager_is_human_club(read_usize(p + live_match::HOME_CLUB), 0, 1, 0, 0) == 0 && exe::human_manager_is_human_club(read_usize(p + live_match::AWAY_CLUB), 0, 1, 0, 0) == 0);
    if mostrar {
        text(BUF, 0xa16414);
        let r = row(current_human_display(), 0x82, 0, 0, 0, 0, 0, 5, 0x1021, 0, 0, 0xc, 1, colour_with_high_bits(p15, G_GUI_COLOUR_PALE_GREY), BUF, 0, 0, 0, clock);
        let v = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), r);
        text(BUF, 0xa16408);
        let muito_rapida = add_commentary_speed_row(v, 0x14, velocidade(), (0, 1));
        text(BUF, 0xa16400);
        let fast = add_commentary_speed_row(v, 0x37, velocidade(), (2, 0));
        text(BUF, 0xa163cc, 0xad9c64);
        let average = add_commentary_speed_row(v, 0x5a, velocidade(), (1, 2));
        text(BUF, 0xa163c4);
        let lenta = add_commentary_speed_row(v, 0x7d, velocidade(), (0, 1));
        text(BUF, 0xa163b8);
        let muito_lenta = add_commentary_speed_row(v, 0x96, velocidade(), (2, 0));
        // the focus goes to the row above the current one (very slow to very fast and those outside the table)
        let foco = match velocidade() as i16 {
            0x37 => muito_rapida,
            0x5a => fast,
            0x7d => average,
            0x96 => lenta,
            _ => muito_lenta,
        };
        let d = current_human_display();
        core::ptr::write_unaligned((d + 0x12f4ff) as *mut u16, foco as u16);
        *((d + 0x12f4fe) as *mut u8) = 0x20;
        let commentator = rd(read_usize(p + live_match::ENGINE) + 0x8f7a);
        *((commentator + match_events_fields::COMMENTARY_SPEED) as *mut u8) = velocidade() as u8;
    }
    *(exe::get_human_manager_record(0) as *mut u8) = 3;

    // the tabs
    text(match_tab_title(&mut tabs, 0), 0xa163a8);
    set_match_tab_flags(&mut tabs, 0, 3, 1);
    text(match_tab_title(&mut tabs, 1), 0xa1639c);
    set_match_tab_flags(&mut tabs, 1, 3, 2);
    text(match_tab_title(&mut tabs, 2), 0xa1638c);
    set_match_tab_flags(&mut tabs, 2, 3, 3);
    text(match_tab_title(&mut tabs, 3), 0xa1637c);
    set_match_tab_flags(&mut tabs, 3, 3, 4);
    let t1 = Club(read_usize(p + live_match::HOME_CLUB));
    text(match_tab_title(&mut tabs, 4), 0xa16364, sx(t1.addr() + club::SHORT_GENDER_NAME), sx(t1.addr() + club::SHORT_GENDER_NAME), t1.addr() + club::SHORT_NAME);
    set_match_tab_flags(&mut tabs, 4, 3, 6);
    text(match_tab_title(&mut tabs, 5), 0xa16354);
    set_match_tab_flags(&mut tabs, 5, 3, 7);
    text(match_tab_title(&mut tabs, 6), 0xa16328, 0xad9c64);
    set_match_tab_flags(&mut tabs, 6, 3, 8);
    let mut n: usize = 7;
    // the league table of the match (for national teams by the continent of the human’s club; cup match by stage)
    let ligas = |k: isize| rd((rd(G_COMP_OBJECT_TABLE) as isize + k * 4) as usize);
    let comp = read_usize(p + live_match::FIXTURE) as i32 as isize;
    let entry = ligas(comp);
    let mut league: usize = 0;
    if entry != 0 {
        let national_teams = *(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as *const i32) as isize;
        let candidata = if comp == national_teams {
            if sx(p + live_match::STAGE_INDEX) >= 2 {
                None
            } else {
                let slot = rd((0xb74c26 + h * 0x300) as usize);
                let side = if rd(read_usize(p + live_match::AWAY_CLUB) + 0xcf) == slot { read_usize(p + live_match::AWAY_CLUB) } else { read_usize(p + live_match::HOME_CLUB) };
                let national_teams = || *(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as *const i32) as isize;
                Some(if (exe::usa_mls_get_club_division(side) as u32) as u16 == 0x414 {
                    rd(rd(ligas(national_teams()) + 0xc) + 4)
                } else if (exe::usa_mls_get_club_division(side) as u32) as u16 == 0x47b {
                    rd(rd(ligas(national_teams()) + 0xc))
                } else {
                    ligas(national_teams())
                })
            }
        } else {
            let stage = *((p + live_match::STAGE_INDEX) as *const i8);
            Some(if stage == -1 { entry } else { rd((rd(entry + 0xc) as isize + stage as isize * 4) as usize) })
        };
        if let Some(l) = candidata {
            league = l;
            if l != 0 && rb(l + 0x43) == 2 && exe::comp_is_reserve_or_minor_comp(*(G_RESERVE_COMP as *const usize), rd(l + 4)) == 0 {
                text(match_tab_title(&mut tabs, 7), 0x99c280);
                set_match_tab_flags(&mut tabs, 7, 3, 0xa);
                n = 8;
            }
        }
    }
    let t2 = Club(read_usize(p + live_match::AWAY_CLUB));
    text(match_tab_title(&mut tabs, n), 0xa16364, sx(t2.addr() + club::SHORT_GENDER_NAME), sx(t2.addr() + club::SHORT_GENDER_NAME), t2.addr() + club::SHORT_NAME);
    set_match_tab_flags(&mut tabs, n, 3, 9);
    let (mut top, mut base) = (0x50i32, 0x221i32);
    let aba = aba as i8 as i32;
    exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, n as u32 + 1, aba, &mut top, &mut base, 0);
    set_screen_param(8, MENOS1);
    let panel = |f: usize, top: *mut i32, base: *mut i32| {
        let g: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(f);
        g(top, base)
    };
    match aba {
        1 => panel(0x71c270, &mut top, &mut base),
        2 => panel(0x71e040, &mut top, &mut base),
        3 => panel(0x71f500, &mut top, &mut base),
        4 => panel(0x721000, &mut top, &mut base),
        6 => exe::draw_team_stats_panel(0, &mut top, &mut base),
        7 => panel(0x722ed0, &mut top, &mut base),
        8 => panel(0x723d20, &mut top, &mut base),
        9 => exe::draw_team_stats_panel(1, &mut top, &mut base),
        10 => {
            exe::match_screens_draw_live_league_table(league, top, base);
        }
        _ => {}
    }

    // the stadium (”stadium, city”; without a city only the name, without a stadium ”Unknown Stadium”)
    let stadium = read_usize(p + live_match::STADIUM);
    let r: usize = if stadium != 0 {
        let city = read_usize(stadium + stadium::CITY);
        if city != 0 {
            let (a, b) = (sx(stadium + stadium::GENDER_NAME), sx(city + city::GENDER_NAME));
            text(BUF, 0x9902a4, a, a, b, b, stadium + stadium::NAME, city + 4) as usize
        } else {
            // the original’s strcpy leaves eax and ecx at 0
            copy_to_work_text(stadium + stadium::NAME);
            0
        }
    } else {
        text(BUF, 0x99c1a4) as usize
    };
    text_of_manager(sm, 0x64, 0x22b, 0x316, 0x24e, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 3, colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY), BUF, 1);

    // the commentary panel
    if get_screen_param(8) as u16 == 0xffff {
        if get_screen_param(7) != 0 {
            let commentator = rd(read_usize(p + live_match::ENGINE) + 0x8f7a);
            let r = text_of_manager(sm, 0x64, 0x22b, 0x316, 0x24e, 1, 0, 0xc, 3, 0, 0, 1);
            exe::match_events_show_event_text(commentator, rd(read_usize(p + live_match::SPECTATORS) + 1), st, read_usize(p + live_match::FIXTURE), r, 0x2b2, 0, 3, 0);
            write_item(8, r);
        } else {
            let r = text_of_manager(sm, 0x64, 0x22b, 0x316, 0x24e, 1, 0, 0xc, 3, 0, 0, 1);
            write_item(8, r);
        }
    }
    destruct_match_tabs(&mut tabs);
}

/// Handles input on the live match screen for item `item` (−1: the passing of time): commentary volume and speed, the
/// new event (clock, commentary, modes), watching or not, a team's tactics, switching tabs and each tab's input.
///
/// On a network (`[0xb77c84]`) or with the match to be confirmed in mode 1 without confirmation: 0; spectators closed
/// (`[+0x119]`): −9; mode 1: confirms over the network (`0x6b5260`) and, once confirmed, moves to mode 2 and −11. The
/// item with id `0xc` changes the commentator volume (`[+0x2db0]`); a volume different from the manager's speed
/// (`0x5fcff0`) passes it on (`0x5fd100`) and −11.
///
/// No item: with the human connected (`0x5fd210`), modes 2 and 3 become 4 (stops watching) and 5 stops the commentary
/// (`0x802a50`, −9); in mode 4 with a new event (`0x6b55c0`) it reads the match menu flags (`+0`: 1 and 2 redo
/// everything, 4 the clock, 8 narrates the highlight (`0x6d7b00`) and, on substitution ones, marks the human's team in
/// param `0x15`, `0x20` and `0x40` modes 3 and 5); the "waiting" flag (param `0x24`) and the control label
/// (`0x725a70`); with no team marked, it looks for the human's (params `0x2f`/`0x30`).
///
/// With a team to open (the marked one, or the item with id 1): opens its tactics (`0x899a50`, −4) if the match allows
/// it (`[+0x8eb2]` with no highlight), otherwise re-marks the buttons (`0x800d30`) and param `0x15`. Item id 2: watch
/// or not (modes 2/3/4), 5 stops (−9); id 3: changes the tab (param 2). The menu bar (`0x7643d0`; −4 marks the human as
/// leaving, −911) and then the tab decides (the panels `0x71d9e0`, `0x71e750`, `0x720320` (id 4 changes the period of
/// param `0xa`), `0x723350`; the handlers `0x722dc0` and `0x725860`; tab 10 converts param `0x26` into `0x25`). Finally
/// −11 with `[0xb6b7b0]` or the redraw (`0x802d00`) with `[0xb6b7a8]`.
/// Original: `handle_match_screen_input` (0x71a240, match_screens.cpp, cdecl).
///
/// [pt] { 0x71a240 } (cdecl, match_screens.cpp): a resposta da tela da partida ao vivo ao item `item` (−1:
/// a passagem do tempo). Em rede ([0xb77c84]) ou com a partida a confirmar no modo 1 sem confirmação:
/// 0; espectadores fechados ([+0x119]): −9; modo 1: confirma pela rede (0x6b5260) e, confirmado, passa
/// ao modo 2 e −11. O item de id 0xc troca o volume do narrador ([+0x2db0]); volume diferente da
/// velocidade do técnico (0x5fcff0) repassa (0x5fd100) e −11. Sem item: com o humano conectado
/// (0x5fd210), os modos 2 e 3 viram 4 (deixa de assistir) e o 5 para a narração (0x802a50, −9); no modo
/// 4 com lance novo (0x6b55c0) lê as marcas do menu da partida (+0: 1 e 2 refazem tudo, 4 o relógio, 8
/// narra o lance (0x6d7b00) e, nos de substituição, marca o time do humano no parâmetro 0x15, 0x20 e
/// 0x40 os modos 3 e 5); a marca de "esperando" (parâmetro 0x24) e o rótulo do controle (0x725a70); sem
/// time marcado, procura o do humano (parâmetros 0x2f/0x30). Com um time a abrir (o marcado, ou o
/// item de id 1): abre a tática dele (0x899a50, −4) se o jogo permite ([+0x8eb2] sem lance), senão
/// remarca os botões (0x800d30) e o parâmetro 0x15. Item de id 2: assistir ou não (modos 2/3/4), 5
/// para (−9); id 3: troca a aba (parâmetro 2). A barra de menu (0x7643d0; −4 marca o humano como
/// saindo, −911) e depois a aba decide (os painéis 0x71d9e0, 0x71e750, 0x720320 (id 4 troca o período
/// do parâmetro 0xa), 0x723350; as respostas 0x722dc0 e 0x725860; a 10 converte o parâmetro 0x26 no
/// 0x25). Por fim −11 com [0xb6b7b0] ou o redesenho (0x802d00) com [0xb6b7a8].
pub(crate) unsafe extern "C" fn handle_match_screen_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond match");
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    let g = |a: usize| *(a as *const u32);
    let por = |a: usize, v: u32| *(a as *mut u32) = v;
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let h = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16);
    if g(0xb77c84) != 0 {
        return 0;
    }
    let p = get_screen_param(0);
    let mut mode = get_screen_param(1) as u8;
    let aba = get_screen_param(2) as u8;
    let p3 = get_screen_param(3);
    let p4 = get_screen_param(4);
    let p5 = get_screen_param(5);
    let p6 = get_screen_param(6);
    let p8 = get_screen_param(8);
    let mut p15 = get_screen_param(0x15);
    let m = read_usize(p + live_match::SPECTATORS);
    if read_usize(m + match_spectators::CLOSED) != 0 {
        return -9;
    }
    if mode == 1 {
        if read_usize(p + live_match::ENGINE) == 0 {
            exe::match_day_start_watched_match(*(G_MATCH_DAY as *const usize), p);
        }
        if read_usize(p + live_match::ENGINE) == 0 {
            return 0;
        }
        set_screen_param(1, 2);
        return -11;
    }
    let pm = read_usize(p + live_match::ENGINE);
    let st = read_usize(p + live_match::RECORD);
    let item16 = item as u16 as i16;
    let record = |i: i16| (current_human_display() as isize + i as isize * 0x18c) as usize;
    if item_id_or_default(item16) == 0xc {
        let v = if item16 != -1 { rb(record(item16) + 0xba9a6) } else { 0 };
        *((read_usize(pm + engine::COMMENTATOR) + 0x2db0) as *mut u8) = v;
    }
    let volume = rb(read_usize(pm + engine::COMMENTATOR) + 0x2db0);
    if volume as u16 != exe::human_manager_get_commentary_speed(*(G_HUMAN_MANAGER as *const usize), 0) as u16 {
        // the original pushes ecx with the upper 16 bits of what was left from 0x5fcff0 (extended here)
        exe::human_manager_set_commentary_speed(*(G_HUMAN_MANAGER as *const usize), rb(read_usize(pm + engine::COMMENTATOR) + 0x2db0) as u32, 0);
        return -11;
    }
    por(G_MATCH_SCREEN_LIVE_DATA_FLAG, 0);
    por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 0);
    por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 0);
    let espectador = || (m as isize + 0xc5 + h() as isize * 4) as *mut i32;
    let how_many = (m + match_spectators::COUNT) as *mut u8;
    let assistir = || {
        if core::ptr::read_unaligned(espectador()) == 0 {
            core::ptr::write_unaligned(espectador(), 1);
            *how_many = (*how_many).wrapping_add(1);
        }
    };
    let deixar_de_assistir = || {
        if core::ptr::read_unaligned(espectador()) != 0 {
            core::ptr::write_unaligned(espectador(), 0);
            *how_many = (*how_many).wrapping_sub(1);
        }
    };
    let t_do_menu = |d: usize| {
        let x = read_usize(pm + engine::MATCH);
        if x != 0 { rd(x + d) } else { 0 }
    };
    // is team `t` that of the current human (0x600a50 and his slot at 0x7fffa0)? then it is the one of parameter 0x15
    let mark_se_of_human = |t: usize, p15: &mut usize| {
        if exe::human_manager_is_human_club(t, 0, 1, 0, 0) != 0 && exe::screenmanager_find_manager_channel(sm, exe::database_get_club_manager(t)) == h() as u16 {
            *p15 = t;
            set_screen_param(0x15, t);
        }
    };

    // the team to open (Some) or move on to the item
    let target: Option<usize> = if item16 == -1 {
        let mut to_play = false;
        if rd((0xb74c26 + h() as isize * 0x300) as usize) != 0 {
            if exe::human_manager_is_on_holiday(*(G_HUMAN_MANAGER as *const usize), 0) != 0 {
                if mode == 2 || mode == 3 {
                    set_screen_param(1, 4);
                    mode = 4;
                    deixar_de_assistir();
                    *((p + live_match::STATE) as *mut u8) = 1;
                    por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                    to_play = true;
                } else if mode == 5 {
                    exe::screenmanager_stop_sound(sm, (-1) as i32 as u32);
                    return -9;
                }
            }
        }
        if !to_play && mode == 4 {
            to_play = true;
        }
        let lance: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_STEP_LIVE_MATCH);
        if to_play && lance(*(G_MATCH_DAY as *const usize), p) != 0 {
            let marks = || rb(m);
            if marks() & 1 != 0 {
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
            }
            if marks() & 2 != 0 {
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
            }
            if marks() & 4 != 0 && g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(BUF, 0x98806c, *((st + match_record::MINUTE) as *const i16) as i32);
                exe::screenmanager_set_widget_text(sm, p3 as u32, BUF);
                por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 1);
            }
            if marks() & 8 != 0 {
                if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                    let commentator = read_usize(pm + engine::COMMENTATOR);
                    // (width, wrap, font, mode); the original’s mode is a seven in dl over a dirty edx
                    let (largura, quebrar, fonte, modo_n) = if aba == 1 && read_usize(st + match_record::SHOOTOUT) == 0 { (0x262, 1, 5, 0) } else { (0x2b2, 0, 3, (aba == 5) as u32) };
                    let comp = rd(read_usize(pm + engine::MATCH));
                    exe::match_events_show_event_text(commentator, rd(m + 1), st, comp, p8, largura, quebrar, fonte, modo_n);
                    por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 1);
                }
                set_screen_param(7, 1);
                let lance = rd(m + 1);
                let code = *(lance as *const u16);
                if matches!(code, 0x1ff8 | 0x1ff7 | 0x1ff5 | 0x1ff6 | 0x1f56) {
                    let t = t_do_menu(0x1c);
                    if read_usize(pm + engine::MATCH) != 0 && t != 0 && rb(rd(m + 1) + 6) == 0 {
                        mark_se_of_human(t, &mut p15);
                    }
                    let t = t_do_menu(0x20);
                    if read_usize(pm + engine::MATCH) != 0 && t != 0 && rb(rd(m + 1) + 6) == 1 {
                        mark_se_of_human(t, &mut p15);
                    }
                }
            } else if get_screen_param(7) != 0 {
                if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                    exe::screenmanager_set_widget_style(sm, p8 as u32, 1);
                    exe::screenmanager_set_widget_text(sm, p8 as u32, 0xad9c64);
                    por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 1);
                }
                set_screen_param(7, 0);
            }
            if marks() & 0x20 != 0 {
                set_screen_param(1, 3);
                mode = 3;
                assistir();
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
            }
            if marks() & 0x40 != 0 {
                set_screen_param(1, 5);
                mode = 5;
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
            }
            por(G_MATCH_SCREEN_LIVE_DATA_FLAG, 1);
        }
        // the ”waiting” flag (parameter 0x24): the human left the match others are watching
        let esperando = *how_many != 0 && core::ptr::read_unaligned(espectador()) == 0;
        if esperando == (get_screen_param(0x24) == 0) {
            if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                let mut colour: u32 = 0;
                exe::match_screens_get_match_control_label(mode as u32, m, BUF, 0x7d0, &mut colour);
                exe::screenmanager_set_widget_style(sm, p4 as u32, colour);
                exe::screenmanager_set_widget_text(sm, p4 as u32, BUF);
                por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 1);
            }
            set_screen_param(0x24, esperando as usize);
        }
        if p15 == 0 {
            let stage = read_u8(pm + engine::SHOOTOUT_STATE);
            if stage == 1 || stage == 4 {
                let t = t_do_menu(0x1c);
                if read_usize(pm + engine::MATCH) != 0 && t != 0 && get_screen_param(0x2f) == 0 {
                    mark_se_of_human(t, &mut p15);
                }
                let t = t_do_menu(0x20);
                if read_usize(pm + engine::MATCH) != 0 && t != 0 && get_screen_param(0x30) == 0 {
                    mark_se_of_human(t, &mut p15);
                }
            }
        }
        if p15 != 0 { Some(p15) } else { None }
    } else if display_item_id(item16) == 1 {
        Some(rd(record(item16) + 0xba9a6))
    } else {
        None
    };

    if let Some(target) = target {
        if get_screen_param(7) == 0 && read_u8(pm + engine::RESTART_TYPE) != 0 {
            // opens the team’s tactics
            let mut side: u32 = 0;
            if exe::human_manager_is_human_club(target, 0, 1, 0, 0) != 0 {
                let slot = || rd((0xb74c26 + h() as isize * 0x300) as usize);
                if exe::database_get_club_manager(target) != slot() {
                    let v = slot();
                    let u = exe::database_get_club_manager(target);
                    let swap: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SCREENMANAGER_SWAP_MANAGER_KEYS);
                    swap(sm, v, u);
                }
            }
            if (target == read_usize(p + live_match::HOME_CLUB) && read_u8(p + live_match::HOME_SIDE_MARK) == 0) || (target == read_usize(p + live_match::AWAY_CLUB) && read_u8(p + live_match::AWAY_SIDE_MARK) == 0) {
                side = 1;
            }
            set_screen_param(0x15, 0);
            assistir();
            *((exe::get_human_manager_record(0) + 0xea) as *mut u32) = 0;
            let stage = read_u8(pm + engine::SHOOTOUT_STATE);
            if (stage == 1 || stage == 4) && exe::human_manager_is_human_club(target, 0, 1, 0, 0) != 0 && exe::screenmanager_find_manager_channel(sm, exe::database_get_club_manager(target)) == h() as u16 {
                let k = if target == t_do_menu(0x1c) { 0x2f } else { 0x30 };
                set_screen_param(k, 1);
                exe::tactics_screen_open(target, side, pm, 0, -1, 1);
                return -4;
            }
            let state = read_u8(p + live_match::STATE);
            if state == 0xff || state == 1 || state == 4 {
                exe::tactics_screen_open(target, side, pm, 1, -1, 0);
            } else {
                exe::tactics_screen_open(target, side, pm, 0, -1, 0);
            }
            return -4;
        }
        if target != p15 {
            if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                exe::screenmanager_set_widget_style(sm, p5 as u32, if t_do_menu(0x1c) == target { 0x1061 } else { 0x1021 });
                exe::screenmanager_set_widget_style(sm, p6 as u32, if t_do_menu(0x20) == target { 0x1061 } else { 0x1021 });
                por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 1);
            }
            set_screen_param(0x15, target);
        }
    }

    if item16 != -1 {
        if display_item_id(item16) == 2 {
            match mode {
                2 | 3 => {
                    set_screen_param(1, 4);
                    mode = 4;
                    deixar_de_assistir();
                    *((p + live_match::STATE) as *mut u8) = 1;
                    por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                }
                4 => {
                    if core::ptr::read_unaligned(espectador()) == 0 {
                        core::ptr::write_unaligned(espectador(), 1);
                        *how_many = (*how_many).wrapping_add(1);
                    } else {
                        core::ptr::write_unaligned(espectador(), 0);
                        *how_many = (*how_many).wrapping_sub(1);
                    }
                    por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                }
                5 => {
                    exe::screenmanager_stop_sound(sm, (-1) as i32 as u32);
                    return -9;
                }
                _ => {}
            }
        }
        if display_item_id(item16) == 3 && rd(record(item16) + 0xba9a6) != get_screen_param(2) {
            set_screen_param(2, rd(record(item16) + 0xba9a6));
            por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
        }
    }

    if exe::menubar_handle_object_link(item) == -4 {
        if core::ptr::read_unaligned(espectador()) == 0 {
            core::ptr::write_unaligned(espectador(), -911);
            *how_many = (*how_many).wrapping_add(1);
        }
        *((exe::get_human_manager_record(0) + 0xea) as *mut u32) = 0;
        return -4;
    }
    // the original reads parameter 0 before looking at [0xb6b7ac]
    let menu_da_partida = |bit: u8| {
        let q = get_screen_param(0);
        g(G_MATCH_SCREEN_LIVE_DATA_FLAG) != 0 && rb(read_usize(q + live_match::SPECTATORS)) & bit != 0
    };
    let redo = |f: usize| {
        let r: unsafe extern "C" fn() = core::mem::transmute(f);
        r();
        por(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 1);
    };
    match aba as i8 {
        1 => {
            let q = get_screen_param(0);
            if g(G_MATCH_SCREEN_LIVE_DATA_FLAG) != 0 {
                let f = read_usize(q + live_match::SPECTATORS);
                if rb(f) & 0x20 != 0 {
                    por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                }
                if rb(f) & 0x10 != 0 {
                    por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                }
                if rb(f) & 4 != 0 {
                    if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
                        return -11;
                    }
                    redo(0x71d9e0);
                }
            }
        }
        2 => {
            if menu_da_partida(4) {
                if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
                    return -11;
                }
                redo(0x71e750);
            }
        }
        3 => {
            if menu_da_partida(4) && g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                redo(0x720320);
            }
            if item16 != -1 && display_item_id(item16) == 4 && rd(record(item16) + 0xba9a6) != get_screen_param(0xa) {
                set_screen_param(0xa, rd(record(item16) + 0xba9a6));
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                return -11;
            }
        }
        4 => {
            if menu_da_partida(0x20) {
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                return -11;
            }
        }
        6 | 9 => {
            let r = exe::handle_team_stats_input(item);
            if r != 0 {
                return r;
            }
        }
        7 => {
            if menu_da_partida(4) {
                if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
                    return -11;
                }
                redo(0x723350);
            }
        }
        8 => {
            let swap: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MATCH_SCREENS_HANDLE_LATEST_SCORES_CYCLE);
            let r = swap(item);
            if r != 0 {
                return r;
            }
        }
        10 => {
            let v = get_screen_param(0x26);
            let r = exe::screenmanager_get_area_scroll_row(sm, v);
            set_screen_param(0x25, r);
            if mode != 5 && rb(m) & 4 != 0 {
                por(G_MATCH_SCREEN_VIEW_CHANGED_FLAG, 1);
                return -11;
            }
        }
        _ => {}
    }
    if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
        return -11;
    }
    if g(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG) != 0 {
        exe::screenmanager_redraw_screen(sm, 0, 0);
    }
    0
}

/// Builds the details screen of a stored match: loads the match record and snapshot from the match stats, the team
/// headers, the 7 tabs and the selected tab panel.
///
/// Convention: cdecl, SEH, `match_screens.cpp`. It frees the snapshot of param `0x1d`; without the record (param
/// `0x1f`) it reads that of the match in param `0x1e` from the statistics (`0x729e30`, `0x4f` bytes) and without the
/// snapshot (`0x20`) it builds it (`0x729600`, `0xd7a` bytes; on error: "Error" box and nothing more); the background,
/// the headers of the two teams (colours `0x53b640`, name, club box, score or "Match Abandoned"), the aggregate
/// (`[+0x4b]` ≥ 0), the menu bar, the 7 tabs, the back and forward buttons and the tab panel (param 2).
/// Original: `build_match_details_screen` (0x71b630, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b630 } (cdecl, SEH, match_screens.cpp): monta a tela de detalhes de uma partida guardada:
/// libera a foto do parâmetro 0x1d; sem o registro (parâmetro 0x1f) lê o da partida do parâmetro 0x1e
/// das estatísticas (0x729e30, 0x4f bytes) e sem a foto (0x20) a monta (0x729600, 0xd7a bytes; erro:
/// caixa "Error" e nada mais); o fundo, os cabeçalhos dos dois times (cores 0x53b640, nome, caixa do
/// clube, placar ou "Match Abandoned"), o agregado ([+0x4b] ≥ 0), a barra de menu, as 7 abas, os
/// botões de voltar e avançar e o painel da aba (parâmetro 2).
pub(crate) unsafe extern "C" fn build_match_details_screen() {
    crate::comparison::branch("screens build the details");
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text_of_manager: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let colours: unsafe extern "C" fn(usize, usize, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let sx = |a: usize| *(a as *const i8) as i32;

    let mut tabs: MatchTabs = [0; 0x14b4 / 4];
    construct_match_tabs(&mut tabs);
    free_match_snapshot();
    let aba = get_screen_param(2) as u8;
    let index = get_screen_param(0x1e);
    let mut reg = get_screen_param(0x1f);
    let mut photo = get_screen_param(0x20);
    let stats = *(G_MATCH_STATS as *const usize);
    if reg == 0 {
        reg = exe::crt_operator_new((0x4f) as u32);
        if exe::match_stats_find_record_fixture(stats, index, reg) == 0 {
            match_screens_error_box(0x90a);
            exe::crt_operator_delete(reg);
            destruct_match_tabs(&mut tabs);
            return;
        }
        set_screen_param(0x1f, reg);
    }
    if photo == 0 {
        let f = exe::crt_operator_new((0xd7a) as u32);
        if f != 0 {
            exe::crt_vector_constructor_iterator(f + match_record::HOME_TEAM, 0x625, 2, 0x453c30, 0x453c10);
        }
        if exe::match_stats_read_match_record(*(G_MATCH_STATS as *const usize), (index) as u32, f) == 0 {
            match_screens_error_box(0x91a);
            if f != 0 {
                exe::crt_vector_destructor_iterator(f + match_record::HOME_TEAM, 0x625, 2, 0x453c10);
                exe::crt_operator_delete(f);
            }
            destruct_match_tabs(&mut tabs);
            return;
        }
        set_screen_param(0x20, f);
        photo = f;
    }
    let r = set_match_background(read_usize(reg + fixture::STADIUM), read_usize(reg + fixture::COMPETITION), read_usize(reg + fixture::HOME_CLUB));
    // the u16 colours (the rest of the original’s variables is stack garbage, here 0)
    let (mut cor_a, mut cor_b) = (0u32, 0u32);
    colours(read_usize(reg + fixture::HOME_CLUB), (r & !0xff) | read_u8(photo + match_record::HOME_KIT) as usize, &mut cor_a, &mut cor_b);
    format(BUF, 0x987ff0, read_usize(reg + fixture::HOME_CLUB) + 0x38);
    text_of_manager(sm, 0x64, 0xa, 0x1bc, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x64, 0xa, 0x1bc, 0x46, 1, 0, 1, 7, 0, 0, 2, read_usize(reg + fixture::HOME_CLUB));
    let r = if read_u8(photo + match_record::HOME_TEAM) == 0xfd { text(BUF, 0xa164c4, 0xad9c64) } else { format(BUF, 0x98806c, read_u8(photo + match_record::HOME_TEAM) as u32) };
    text_of_manager(sm, 0x18f, 0xf, 0x1b7, 0x41, 0x30, colour_word(G_GUI_COLOUR_PALE_GREY), 0xc, 6, colour_with_high_bits(r as usize, G_GUI_COLOUR_BLACK), BUF, 1);
    // the second team’s kit byte goes in a dirty (extended) ecx
    colours(read_usize(reg + fixture::AWAY_CLUB), read_u8(photo + match_record::AWAY_KIT) as usize, &mut cor_a, &mut cor_b);
    format(BUF, 0x987ff0, read_usize(reg + fixture::AWAY_CLUB) + 0x38);
    text_of_manager(sm, 0x1be, 0xa, 0x316, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x1be, 0xa, 0x316, 0x46, 1, 0, 1, 7, 0, 0, 2, read_usize(reg + fixture::AWAY_CLUB));
    let r = if read_u8(photo + match_record::HOME_TEAM) == 0xfd { text(BUF, 0xa164c4, 0xad9c64) } else { format(BUF, 0x98806c, read_u8(photo + match_record::AWAY_TEAM) as u32) };
    text_of_manager(sm, 0x2e9, 0xf, 0x311, 0x41, 0x30, colour_with_high_bits(r as usize, G_GUI_COLOUR_PALE_GREY), 0xc, 6, colour_word(G_GUI_COLOUR_BLACK), BUF, 1);
    let aggregate = sx(reg + fixture::HOME_AGGREGATE);
    if aggregate > -1 {
        text(BUF, 0xa16454, aggregate, sx(reg + fixture::AWAY_AGGREGATE));
        text_of_manager(sm, 0x1c3, 0x37, 0x2e4, 0x41, 1, 0, 0x40, 1, cor_a as usize, BUF, 1);
    }
    exe::menubar_build(4, 0);

    text(match_tab_title(&mut tabs, 0), 0xa163a8);
    set_match_tab_flags(&mut tabs, 0, 1, 1);
    text(match_tab_title(&mut tabs, 1), 0xa1639c);
    set_match_tab_flags(&mut tabs, 1, 1, 2);
    text(match_tab_title(&mut tabs, 2), 0xa1638c);
    set_match_tab_flags(&mut tabs, 2, 1, 3);
    text(match_tab_title(&mut tabs, 3), 0xa1637c);
    set_match_tab_flags(&mut tabs, 3, 1, 4);
    let t1 = Club(read_usize(reg + fixture::HOME_CLUB));
    text(match_tab_title(&mut tabs, 4), 0xa16364, sx(t1.addr() + club::SHORT_GENDER_NAME), sx(t1.addr() + club::SHORT_GENDER_NAME), t1.addr() + club::SHORT_NAME);
    set_match_tab_flags(&mut tabs, 4, 1, 5);
    text(match_tab_title(&mut tabs, 5), 0xa16354);
    set_match_tab_flags(&mut tabs, 5, 1, 6);
    let t2 = Club(read_usize(reg + fixture::AWAY_CLUB));
    text(match_tab_title(&mut tabs, 6), 0xa16364, sx(t2.addr() + club::SHORT_GENDER_NAME), sx(t2.addr() + club::SHORT_GENDER_NAME), t2.addr() + club::SHORT_NAME);
    set_match_tab_flags(&mut tabs, 6, 1, 7);
    let (mut top, mut base) = (0x50i32, 0x221i32);
    let aba = aba as i8 as i32;
    exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 7, aba, &mut top, &mut base, 1);
    let advance = (exe::screenmanager_is_forward_unavailable(sm) == 0) as u32;
    let go_back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(go_back, advance);
    let panel = |f: usize, top: *mut i32, base: *mut i32| {
        let g: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(f);
        g(top, base)
    };
    match aba {
        1 => panel(0x71c270, &mut top, &mut base),
        2 => panel(0x71e040, &mut top, &mut base),
        3 => panel(0x71f500, &mut top, &mut base),
        4 => panel(0x721000, &mut top, &mut base),
        5 => exe::draw_team_stats_panel(0, &mut top, &mut base),
        6 => panel(0x722ed0, &mut top, &mut base),
        7 => exe::draw_team_stats_panel(1, &mut top, &mut base),
        _ => {}
    }
    destruct_match_tabs(&mut tabs);
}

// simplification: the answer touches screens, menu, network, commentary (sound) and tactics (does not run twice): in
// comparison only the original runs; the copy is proved in Lucas’s game in Rust mode (”telas ...” counter)

/// Redraws the overview possession bar when the share changes (percentages and the colour of the club with more of the
/// ball) and stores the match state in the param `0x1d` copy.
///
/// In the match (param 0), with no penalty shoot-out (`[estado + 0x53]` = 0), it takes the colours of the two clubs
/// (`0x53b640` with kits `+0x152` and `+0x777`) and, if possession (`+0x142`) changed since the copy in param `0x1d`
/// (or there is no copy), it redraws the overview possession bar: the percentage (50 with no value or without flag
/// `+0x767`) as "%d%%" in items 3 and 4 from param `0x21`, split from `0x115` to `0x2c9` at the height of param `0x22`,
/// and the colour of the club with more possession in item 5. At the end it stores the state in the copy (which is
/// created here if it does not exist).
/// Original: `refresh_possession_bar` (0x71d9e0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71d9e0 } (cdecl, match_screens.cpp): na partida (parâmetro 0), sem disputa de pênaltis
/// ([estado + 0x53] = 0), pega as cores dos dois clubes (0x53b640 com os uniformes +0x152 e +0x777) e,
/// se a posse (+0x142) mudou desde a cópia do parâmetro 0x1d (ou sem cópia), redesenha a barra da posse
/// da visão geral: o percentual (50 sem valor ou sem a marca +0x767) em "%d%%" nos itens 3 e 4 a partir
/// do parâmetro 0x21, repartidos de 0x115 a 0x2c9 na altura do parâmetro 0x22, e a cor do clube que tem
/// mais posse no item 5. No fim guarda o estado na cópia (que nasce aqui se não existe).
pub(crate) unsafe extern "C" fn refresh_possession_bar() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let copy = get_screen_param(0x1d);
    let item = get_screen_param(0x21) as u32;
    let height = get_screen_param(0x22) as i32;
    if p0 == 0 {
        return;
    }
    let e = core::ptr::read_unaligned((p0 + live_match::RECORD) as *const usize);
    if core::ptr::read_unaligned((e + match_record::SHOOTOUT) as *const u32) == 0 {
        // the colours: 0x53b640 writes u16; the rest of the original’s variable is stack garbage (0x801040 only reads the word)
        let club = |k: usize| (*(G_CLUB_TABLE as *const usize)).wrapping_add(core::ptr::read_unaligned((e + k) as *const usize).wrapping_mul(club::SIZE));
        let (mut c1, mut c1b, mut c2, mut c2b) = (0u32, 0u32, 0u32, 0u32);
        exe::database_get_club_kit_colours(club(0), *((e + match_record::HOME_KIT) as *const u8) as u32, &mut c1, &mut c1b);
        exe::database_get_club_kit_colours(club(4), *((e + match_record::AWAY_KIT) as *const u8) as u32, &mut c2, &mut c2b);
        let v = *((e + match_record::HOME_RECENT_POSSESSION) as *const u8);
        if copy == 0 || v != *((copy + match_record::HOME_RECENT_POSSESSION) as *const u8) {
            let p = if v == 0 || *((e + match_record::AWAY_RECENT_POSSESSION) as *const u8) == 0 { 0x32 } else { v as i32 };
            format(BUF, FORMAT_PCT, p);
            let x = p * 0x1b4 / 100 + 0x115;
            exe::screenmanager_set_widget_rect(sm, item.wrapping_add(3), 0x115, height - 0x23, x, height - 0x14);
            exe::screenmanager_set_widget_text(sm, item.wrapping_add(3), BUF);
            format(BUF, FORMAT_PCT, 100 - p);
            exe::screenmanager_set_widget_rect(sm, item.wrapping_add(4), x, height - 0x23, 0x2c9, height - 0x14);
            exe::screenmanager_set_widget_text(sm, item.wrapping_add(4), BUF);
            exe::screenmanager_set_widget_colour(sm, item.wrapping_add(5), if p < 0x32 { c2 } else { c1 });
        }
    }
    save_match_snapshot(copy, e);
}

/// Address of the player row (0x49 bytes) idx of side in match state e.
///
/// [pt] A linha (0x49 bytes) do jogador `idx` do lado `lado` no estado da partida `e` (+0x11f + 0x71).
pub(crate) fn match_player_row(e: usize, side: u8, idx: u8) -> usize {
    (e as isize + side as i8 as isize * 0x625 + idx as i8 as isize * 0x49) as usize
}

/// Would the match overview show its error box (the match_screens.cpp "Error" box, after which it carries on)?
///
/// Yes when, in the match, the period [state + 0x4f] is outside the six known ones, or when an incident (up to 50, up
/// to side 0xff) has a type outside 1 to 8. In these cases the copy and the comparer run the original routine.
///
/// [pt] A visão geral avisa erro (caixa "Error" de match_screens.cpp e segue)? Na partida, com o período
/// [estado + 0x4f] fora dos seis conhecidos; ou com um lance (até 50, até o lado 0xff) de tipo fora de
/// 1 a 8. Nesses casos a cópia e o comparador rodam a original.
pub(crate) unsafe fn overview_would_error() -> bool {
    let p0 = get_screen_param(0);
    let e = if p0 != 0 { core::ptr::read_unaligned((p0 + live_match::RECORD) as *const usize) } else { get_screen_param(0x20) };
    if p0 != 0 && ![0x1ef, 0x3de, 0, 0x483, 0x528, 1].contains(&core::ptr::read_unaligned((e + match_record::PERIOD_END) as *const u16)) {
        return true;
    }
    for k in 0..50 {
        let ev = e + match_record::EVENTS + 4 * k;
        if *(ev as *const u8) == 0xff {
            break;
        }
        if !(1..=8).contains(&*((ev + 2) as *const u8)) {
            return true;
        }
    }
    false
}

/// Draws the match overview from `*topo` to `*base` (`*base` −= `0x28` right away): period, possession bar and
/// commentary, date, score with half time/extra time/penalties, competition, scorers and cards, shoot-out, referee,
/// attendance, weather and abandoned match.
///
/// Convention: cdecl, `match_screens.cpp`, ~6 KB. In the match (param 0; state at `+0x5f`): the period title (by
/// `[estado + 0x4f]`, with the names of param 1 = 4 or the others) and, with no penalty shoot-out (`[estado + 0x53]` =
/// 0), the possession bar (frame, "Last 5 Mins", the club colours; the item in param `0x21`, the height in `0x22`;
/// `*base` −= `0x37`) and the commentary box (with param 7 the commentator writes the last highlight in it, `0x6d7b00`;
/// the item in param 8). Outside the match: the stadium and the city of param `0x1f`, with the state of param `0x20`.
///
/// Then: the date, the score (`0x4bf930`) with half time, extra time and penalties, the competition (with shortcut
/// `0x7e5` when it is neither a friendly nor a national team one), the list of scorers in two columns (goals, own goal,
/// missed penalty, sending off, injury; the minutes of the same player and type together, breaking the line with more
/// than 10 letters), the penalty shoot-out, the referee, the attendance, the weather, "Match Abandoned" and the
/// possession bar (`0x71d9e0`).
/// Original: `draw_match_overview_panel` (0x71c270, match_screens.cpp, cdecl).
///
/// [pt] { 0x71c270 } (cdecl, match_screens.cpp, ~6 KB): desenha a visão geral da partida de `*topo` a
/// `*base` (`*base` −= 0x28 logo de saída). Na partida (parâmetro 0; estado em +0x5f): o título do
/// período (pelo [estado + 0x4f], com os nomes do parâmetro 1 = 4 ou os outros) e, sem disputa de
/// pênaltis ([estado + 0x53] = 0), a barra da posse (moldura, "Last 5 Mins", as cores dos clubes; o item
/// no parâmetro 0x21, a altura no 0x22; `*base` −= 0x37) e a caixa do comentário (com o parâmetro 7 o
/// narrador escreve o último lance nela, 0x6d7b00; o item no parâmetro 8). Fora dela: o estádio e a
/// cidade do parâmetro 0x1f, com o estado do parâmetro 0x20. Depois: a data, o placar (0x4bf930) com o
/// intervalo, a prorrogação e os pênaltis, a competição (com o atalho 0x7e5 quando não é amistoso nem
/// de seleção), a lista dos autores em duas colunas (gols, gol contra, pênalti perdido, expulsão,
/// lesão; os minutos do mesmo jogador e tipo juntos, quebrando a linha com mais de 10 letras), a
/// disputa de pênaltis, o árbitro, o público, o tempo, "Match Abandoned" e a barra da posse (0x71d9e0).
pub(crate) unsafe extern "C" fn draw_match_overview_panel(top: *mut i32, base: *mut i32) {
    if overview_would_error() {
        return original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_OVERVIEW_PANEL)(top, base);
    }
    let txt: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let b8 = |a: usize| *(a as *const u8);
    let i8_ = |a: usize| *(a as *const i8) as i32;
    let z = |v: i32| v as usize;
    let item16 = |r: usize| r as u16 as i16 as i32 as usize;

    let mut colunas3: [u8; 4] = [1, 1, 1, 0];
    let mut colunas4: [u8; 4] = [2, 1, 2, 1];
    let p0 = get_screen_param(0);
    let period = get_screen_param(1) as u8;
    let base0 = *base;
    *base = base0 - 0x28;
    let (part, e);
    if p0 != 0 {
        *(BUF as *mut u8) = 0;
        e = read_usize(p0 + live_match::RECORD);
        let w = core::ptr::read_unaligned((e + match_record::PERIOD_END) as *const u16);
        let rotulo = if period == 4 {
            match w {
                0x1ef => 0xa167ec,
                0x3de => 0xa167e0,
                0 | 1 => 0xa167b8,
                0x483 => 0xa167a0,
                0x528 => 0xa16788,
                _ => 0,
            }
        } else {
            match w {
                0x1ef => 0xa1677c,
                0x3de => 0xa16770,
                0 | 1 => 0xa167b8,
                0x483 => 0xa1675c,
                0x528 => 0xa16744,
                _ => 0,
            }
        };
        match rotulo {
            0 => show_error_at(0xa161d4, if period == 4 { 0x1d49 } else { 0x1d5e }),
            0xa167b8 => {
                text(BUF, rotulo, EMPTY_TEXT);
            }
            _ => {
                text(BUF, rotulo);
            }
        }
        let t = *top;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), BUF, 1);
        *top += 0x28;
        part = p0 + live_match::FIXTURE;
        if core::ptr::read_unaligned((e + match_record::SHOOTOUT) as *const u32) == 0 {
            // the possession bar
            let b = *base;
            list(current_human_display(), 0x6e, z(b - 0x36), 0x30c, z(b), 1, 0, 1, 0, 2, 0, MAX);
            // 0x53b640 writes u16; the rest of the original’s variable is stack garbage (only the word counts)
            let club = |k: usize| (*(G_CLUB_TABLE as *const usize)).wrapping_add(u(e + k).wrapping_mul(club::SIZE));
            let (mut c68, mut c58, mut c5c, mut c64) = (0u32, 0u32, 0u32, 0u32);
            exe::database_get_club_kit_colours(club(0), read_u8(e + match_record::HOME_KIT) as u32, &mut c68, &mut c58);
            exe::database_get_club_kit_colours(club(4), read_u8(e + match_record::AWAY_KIT) as u32, &mut c5c, &mut c64);
            let b = *base;
            let r = txt(sm, 0x96, z(b - 0x2d), 0x2e4, z(b - 0xa), 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0, 1);
            set_screen_param(0x21, item16(r));
            text(BUF, 0xa16738);
            let b = *base;
            txt(sm, 0xaf, z(b - 0x2d), 0xfa, z(b - 0xa), 1, 0, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 1);
            let b = *base;
            txt(sm, 0x113, z(b - 0x25), 0x2cb, z(b - 0x12), 0x60, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_with_high_bits(z(b), G_GUI_COLOUR_PALE_GREY), 0, 1);
            txt(sm, 0x115, z(b - 0x23), 0x1ef, z(b - 0x14), 0x10, c58 as usize, 0xc, 1, c68 as usize, 0, 1);
            let r = txt(sm, 0x1ef, z(b - 0x23), 0x2c9, z(b - 0x14), 0x10, c64 as usize, 0xc, 1, c5c as usize, 0, 1);
            row(current_human_display(), 1, 0x1ef, z(b - 0x23), 0x1ef, z(b - 0x20), 0, 0, 0x400, colour_with_high_bits(r, G_GUI_COLOUR_BLACK), 0, 0, 0, 0, 0, 0, 0, 0, MAX);
            set_screen_param(0x22, z(*base));
            *base -= 0x37;
            // the commentary box
            let t = *top;
            let y = (*base - t - 0x46) / 2 + t;
            if get_screen_param(7) != 0 {
                crate::comparison::branch("screens overview with the commentator");
                let rel = u(read_usize(p0 + live_match::ENGINE) + 0x8f7a);
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                let m = read_usize(p0 + live_match::ENGINE);
                exe::match_events_show_event_text(rel, u(read_usize(p0 + live_match::SPECTATORS) + 1), e, u(read_usize(m + engine::MATCH)), r, 0x262, 1, 5, 0);
                set_screen_param(8, item16(r));
            } else {
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                set_screen_param(8, item16(r));
            }
        }
    } else {
        let g = Fixture(get_screen_param(0x1f));
        let st = g.stadium();
        if st != 0 {
            let city = read_usize(st + stadium::CITY);
            if city != 0 {
                let (a, c) = (i8_(st + stadium::GENDER_NAME), i8_(city + city::GENDER_NAME));
                text(BUF, 0x9902a4, a, a, c, c, st + stadium::NAME, city + 4);
            } else {
                copy_to_work_text(st + stadium::NAME);
            }
        } else {
            text(BUF, 0x99c1a4);
        }
        let t = *top;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), BUF, 1);
        *top += 0x28;
        e = get_screen_param(0x20);
        part = g.addr();
    }

    // the date, the score and the competition
    let mut data = [0u8; 8];
    exe::new_date(data.as_mut_ptr() as usize, core::ptr::read_unaligned((part + fixture::DAY) as *const u16) as u32, core::ptr::read_unaligned((part + fixture::YEAR) as *const u16) as u32);
    exe::date_get_long_date_string(data.as_mut_ptr() as usize, BUF, 0x7d0, 0xff);
    let mut w = exe::font_text_width(1, (BUF) as *const u8) + 0xf;
    if w <= 0x7d {
        w = 0x7d;
    }
    let t = *top;
    txt(sm, 0x6e, z(t), z(w + 0x6e), z(t + 0x14), 2, 0, 0xc, 1, colour_word(0xaebde8), BUF, 1);
    exe::comp_util_format_fixture_round_label(part, 3, BUF, 0x7d0);
    let mut aux = [0u8; 0x200];
    let pa = aux.as_mut_ptr() as usize;
    if i8_(part + fixture::HOME_GOALS_HALF_TIME) >= 0 {
        text(pa, 0xa16714, i8_(part + fixture::HOME_GOALS_HALF_TIME), i8_(part + fixture::AWAY_GOALS_HALF_TIME));
        append_to_text_buffer(aux.as_ptr());
        if i8_(part + fixture::HOME_GOALS_EXTRA_TIME) >= 0 {
            text(pa, 0xa166d4, i8_(part + fixture::HOME_GOALS), i8_(part + fixture::AWAY_GOALS), i8_(part + fixture::HOME_GOALS_EXTRA_TIME), i8_(part + fixture::AWAY_GOALS_EXTRA_TIME));
            append_to_text_buffer(aux.as_ptr());
        }
        let pens = if read_u8(part + fixture::TIE_RULES) & 0x20 != 0 { 0xa166b0 } else { 0xa1668c };
        if core::ptr::read_unaligned((e + match_record::SHOOTOUT) as *const u32) != 0 {
            text(pa, pens, read_u8(e + match_record::HOME_SHOOTOUT_GOALS) as u32, read_u8(e + match_record::AWAY_SHOOTOUT_GOALS) as u32);
            append_to_text_buffer(aux.as_ptr());
        } else if i8_(part + fixture::HOME_PENALTIES) >= 0 {
            text(pa, pens, i8_(part + fixture::HOME_PENALTIES), i8_(part + fixture::AWAY_PENALTIES));
            append_to_text_buffer(aux.as_ptr());
        }
    }
    let mut x = exe::font_text_width(1, (BUF) as *const u8) + 0xf;
    if x <= 0x7d {
        x = 0x7d;
    }
    let comp = read_usize(part + fixture::COMPETITION);
    if exe::friendly_is_own_comp_record(*(G_FRIENDLY as *const usize), comp) == 0 && exe::comp_is_reserve_or_minor_comp(*(G_RESERVE_COMP as *const usize), read_usize(part + fixture::COMPETITION)) == 0 {
        let key = (((*(read_usize(part + fixture::COMPETITION) as *const u32)) << 11 | (core::ptr::read_unaligned((part + fixture::ROUND_PREFIX) as *const u16) as u32 & 0x7ff)) << 11)
            | (core::ptr::read_unaligned((part + fixture::ROUND_NAME) as *const u16) as u32 & 0x7ff);
        let t = *top;
        caixa(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, colour_with_high_bits(key as usize, 0xaebde8), BUF, 0x7e5, key as usize);
    } else {
        let t = *top;
        txt(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, colour_word(0xaebde8), BUF, 1);
    }

    // the scorers: two columns (home 0, away 2), one row per player and type with the minutes alongside
    *top += 0x19;
    let (ini, fim) = (*top, *base);
    let a = exe::font_get_height(4);
    let n = ((fim - ini - 0x10) / a) as u8;
    let lst = list(current_human_display(), 0x6e, z(ini), 0x30c, z(fim), 4, colunas4.as_mut_ptr() as usize, n as usize, 0, 2, 0, MAX);
    let mut marks = [0u32; 50];
    let mut rows = [0i32; 2];
    let mut side_of_row = 0usize;
    let mut col: u32;
    // [esp+0x24] of the original: the fixed colours replace only the low word, the green one (0x5e4800) all of it
    let mut colour = 0u32;
    let word = |c: u32, a: usize| (c & 0xffff_0000) | *(a as *const u16) as u32;
    let mut name = [0u8; 200];
    let mut junto = [0u8; 0x200];
    let pj = junto.as_mut_ptr() as usize;
    let persons = G_STAFF_TABLE as *const usize;
    // the name cell: with the player checked (0x53e990), the cell with the shortcut to the person
    // (0x7dc national-team club, 0x7d7 otherwise); otherwise only the text
    let cell_of_name = |queue: usize, col: u32, colour: u32, side: u8, f198: u32, f19c: u32, f1a0: u32, l: i32| {
        let pid = core::ptr::read_unaligned((queue + match_record::ROW_STAFF_ID) as *const i32);
        if pid >= 0 && pid < *(G_STAFF_COUNT as *const i32) && exe::database_staff_matches_names((*persons).wrapping_add(pid as usize * 110), f198, f19c, f1a0, queue + match_record::ROW_DATE_OF_BIRTH) != 0 {
            let club = if side == 0 { u(e) } else { read_usize(e + match_record::AWAY_CLUB_ID) } as i32;
            let atalho = if club >= *(G_CLUB_COUNT as *const i32) - 2 * *(G_NATION_COUNT as *const i32) { 0x7dc } else { 0x7d7 };
            cell(sm, lst, col as usize, z(l), 1, 0, 1, 4, colour as usize, BUF, atalho, (*persons).wrapping_add(pid as usize * 110));
        } else {
            text_of_cell(sm, lst, col as usize, z(l), 1, 0, 1, 4, colour as usize, BUF);
        }
        pid
    };
    for i in 1..=50usize {
        let ev = e + match_record::EVENTS + 4 * (i - 1);
        let side = b8(ev);
        if side == 0xff {
            break;
        }
        if marks[i - 1] != 0 {
            continue;
        }
        let idx = b8(ev + 1);
        let queue = match_player_row(e, side, idx);
        let (f198, f19c, f1a0) = (read_usize(queue + match_record::ROW_FIRST_NAME_ID) as u32, read_usize(queue + match_record::ROW_SECOND_NAME_ID) as u32, read_usize(queue + match_record::ROW_COMMON_NAME_ID) as u32);
        let kind = b8(ev + 2);
        match kind {
            1..=6 => {
                // the own goal goes to the other side
                let other = (kind == 2) != (side != 0);
                col = if other { 2 } else { 0 };
                side_of_row = other as usize;
                exe::match_events_format_player_name(e, idx as i8 as i32, side as i8 as i32, name.as_mut_ptr(), 0xc8, 0, 0, 2);
                let np = name.as_ptr();
                match kind {
                    1 | 3 => {
                        format(BUF, 0x987ff0, np);
                        colour = word(colour, G_GUI_COLOUR_PALE_GREY);
                    }
                    2 => {
                        text(BUF, 0xa16670, 0x64, np);
                        colour = word(colour, 0xaebdd4);
                    }
                    5 => {
                        text(BUF, 0xa16630, 0x64, np);
                        colour = word(colour, G_GUI_COLOUR_RED);
                    }
                    _ => {
                        text(BUF, if kind == 4 { 0xa16650 } else { 0xa16610 }, 0x64, np);
                        colour = exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0);
                    }
                }
            }
            7 | 8 => continue,
            _ => {
                show_error_at(0xa161d4, 0xc04);
                continue;
            }
        }
        let pid = cell_of_name(queue, col, colour, side, f198, f19c, f1a0, rows[side_of_row]);
        let minute = |ev: usize| {
            let m = b8(ev + 3) as u32 + 1;
            if b8(ev + 2) == 3 {
                text(BUF, 0xa165fc, m);
            } else {
                format(BUF, FORMAT_D, m);
            }
        };
        minute(ev);
        if i < 50 {
            for j in i..50 {
                let evj = e + match_record::EVENTS + 4 * j;
                let lj = b8(evj);
                if lj == 0xff {
                    break;
                }
                if marks[j] != 0 {
                    continue;
                }
                if core::ptr::read_unaligned((match_player_row(e, lj, b8(evj + 1)) + 0x190) as *const i32) != pid {
                    continue;
                }
                let tj = b8(evj + 2);
                if tj != kind && !((tj == 1 || tj == 3) && (kind == 1 || kind == 3)) {
                    continue;
                }
                let mj = b8(evj + 3) as u32 + 1;
                if tj == 3 {
                    text(pj, 0xa165d8, BUF, mj);
                } else {
                    format(pj, 0xa165d0, BUF, mj);
                }
                if junto.iter().position(|&c| c == 0).unwrap_or(0) > 10 {
                    append_to_text_buffer(0xa165cc as *const u8);
                    let l = rows[side_of_row];
                    text_of_cell(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, colour_word(G_GUI_COLOUR_YELLOW), BUF);
                    rows[side_of_row] = l + 1;
                    minute(evj);
                } else {
                    copy_to_work_text(pj);
                }
                marks[j] = 1;
            }
        }
        let l = rows[side_of_row];
        text_of_cell(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, colour_word(G_GUI_COLOUR_YELLOW), BUF);
        rows[side_of_row] = l + 1;
        marks[i - 1] = 1;
    }

    // the penalty shoot-out
    if core::ptr::read_unaligned((e + match_record::SHOOTOUT) as *const u32) != 0 {
        let m = if rows[0] > rows[1] { rows[0] } else { rows[1] };
        rows[side_of_row] = m;
        text_of_cell(sm, lst, 0, z(m), 1, 0, 1, 3, colour_word(G_GUI_COLOUR_YELLOW), EMPTY_TEXT);
        let r = m + 1;
        rows[side_of_row] = r;
        text(BUF, if read_u8(part + fixture::TIE_RULES) & 0x20 != 0 { 0xa165b8 } else { 0xa165a4 });
        let mut w = exe::font_text_width(3, (BUF) as *const u8) + 0x73;
        if w >= 0x2f1 {
            w = 0x2f1;
        }
        rows[side_of_row] = r + 1;
        // the original pushes edx = the row pointer with the colour in dx; 0x55f990 only reads the word
        row(current_human_display(), 1, 0x73, 0, z(w), 0, 0, z(r), 1, 0, 0, 1, 3, colour_word(G_GUI_COLOUR_YELLOW), BUF, 0, 0, 0, lst);
        row(current_human_display(), 1, 0x73, 0, z(w), 0, 0, z(r + 1), 0xc400, colour_word(G_GUI_COLOUR_YELLOW), 0, 0, 0, 0, 0, 0, 0, 0, lst);
        rows[side_of_row] = r + 2;
        rows[0] = r + 2;
        rows[1] = rows[side_of_row];
        for k in 0..50 {
            let ev = e + match_record::EVENTS + 4 * k;
            let side = b8(ev);
            if side == 0xff {
                break;
            }
            let idx = b8(ev + 1);
            let queue = match_player_row(e, side, idx);
            let (f198, f19c, f1a0) = (read_usize(queue + match_record::ROW_FIRST_NAME_ID) as u32, read_usize(queue + match_record::ROW_SECOND_NAME_ID) as u32, read_usize(queue + match_record::ROW_COMMON_NAME_ID) as u32);
            let kind = b8(ev + 2);
            if kind != 7 && kind != 8 {
                continue;
            }
            col = if side != 0 { 2 } else { 0 };
            side_of_row = (side != 0) as usize;
            exe::match_events_format_player_name(e, idx as i8 as i32, side as i8 as i32, name.as_mut_ptr(), 0xc8, 0, 0, 2);
            if kind == 7 {
                text(BUF, 0xa16588, name.as_ptr());
                colour = word(colour, 0xaebde8);
            } else {
                text(BUF, 0xa1656c, name.as_ptr());
                colour = exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0);
            }
            let l = rows[side_of_row];
            cell_of_name(queue, col, colour, side, f198, f19c, f1a0, l);
            rows[side_of_row] = l + 1;
        }
    }

    // the referee, the attendance and the weather
    let base23 = base0 - 0x23;
    list(current_human_display(), 0x6e, z(base23), 0x30c, z(base0), 1, 0, 1, 0, 2, 0, MAX);
    let l2 = list(current_human_display(), 0x78, z(base23), 0x302, z(base0), 3, colunas3.as_mut_ptr() as usize, 1, 0, 1, 0, MAX);
    let referee = core::ptr::read_unaligned((part + fixture::REFEREE) as *const i16);
    if referee > -1 {
        exe::database_format_official_name((*(G_OFFICIAL_TABLE as *const usize)).wrapping_add(referee as usize * 0x2b), aux.as_mut_ptr(), 0xc8, 1);
        text(BUF, 0xa16550, aux.as_ptr());
    } else {
        text(BUF, 0xa1653c);
    }
    text_of_cell(sm, l2, 0, 0, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_YELLOW), BUF);
    let attendance = core::ptr::read_unaligned((e + match_record::ATTENDANCE) as *const u32);
    if attendance != 0 {
        let w = core::ptr::read_unaligned((e + match_record::PERIOD_END) as *const i16);
        if w >= 0x3de || w == 0 || w == 1 {
            text(BUF, 0xa1651c, attendance);
            text_of_cell(sm, l2, 1, 0, 1, 0, 0xc, 2, colour_word(0xaebde8), BUF);
        }
    }
    exe::weather_format_description(*(G_WEATHER as *const usize), pa, e + match_record::WEATHER);
    text(BUF, 0xa164f8, aux.as_ptr());
    text_of_cell(sm, l2, 2, 0, 1, 0, 0x40, 2, colour_word(G_GUI_COLOUR_YELLOW), BUF);
    if read_u8(e + match_record::HOME_TEAM) == 0xfd {
        let t = *top;
        let y = (*base - t - 0x46) / 2 + t;
        text(BUF, 0xa164e8);
        txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 0x30, colour_word(G_GUI_COLOUR_BLACK), 0xc, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 1);
    }
    exe::refresh_possession_bar();
}

/// Draws the live league table (league, top, bottom) on a copy of the league with the day's scores applied (or the draw
/// date notice): title, headers, separators, position, up/down arrow, club and columns.
///
/// Convention: cdecl, `match_screens.cpp`, ~3.4 KB; the map says `(void)`, but the original reads three arguments: the
/// league, `topo` and `base`, by value. It makes a local copy of the league (League `0x682200` + `0x682380`, the
/// temporary destroyed with `0x682300`) and, in the match with live scores (`[[p0+0x57]+0x8ed0]`), applies to it
/// (`0x685f60`) the day's matches of the group in param `0xe` of its competition: the human's match with the score from
/// the state (`+0x11f`/`+0x744`, penalties `+0x120`/`+0x745`), the others with no result yet as 0-0, those that already
/// have a score by the count up to the minute (`0x728e90`); sorts the copy.
///
/// With the draw date (`[liga+0xb6]`, `[+0xb8]`+`[+0x40]`) still to come: only the competition name and "The draw will
/// be made on ..." (`0x5ed070`). Otherwise: the title, the 11-column table (the rows by the promotion and relegation
/// places or by the space), the headers, the league with `[+0xad]` = param `0x28` sorted (`0x6827d0`) for the position
/// arrows, one row per club (separators `0x6400`, position, up/down arrow, name with shortcut `0x7d5` and the human's
/// colour, and the record columns by param `0x28`), the scrolling (param `0x25`) and the table item in `0x26`. No
/// league or no param `0xe` ("Error" box): the original routine runs.
/// Original: `match_screens_draw_live_league_table` (0x7280e0, match_screens.cpp, cdecl).
///
/// [pt] { 0x7280e0 } (cdecl, match_screens.cpp, ~3,4 KB; o mapa diz `(void)`, mas a original lê três
/// argumentos: a liga, `topo` e `base`, por valor): desenha a tabela da liga ao vivo. Faz uma cópia
/// local da liga (League 0x682200 + 0x682380, o temporário destruído com 0x682300) e, na partida com
/// placares ao vivo ([[p0+0x57]+0x8ed0]), aplica nela (0x685f60) os jogos do dia do grupo do parâmetro
/// 0xe da competição dele: a partida do humano com o placar do estado (+0x11f/+0x744, pênaltis
/// +0x120/+0x745), as outras sem resultado ainda com 0 a 0, as que já têm placar pela contagem até o
/// minuto (0x728e90); ordena a cópia. Com a data do sorteio ([liga+0xb6], [+0xb8]+[+0x40]) ainda por
/// vir: só o nome da competição e "The draw will be made on ..." (0x5ed070). Senão: o título, a tabela
/// de 11 colunas (as linhas pelas vagas de acesso e rebaixamento ou pelo espaço), os cabeçalhos, a
/// liga com [+0xad] = parâmetro 0x28 ordenada (0x6827d0) para as setas de posição, uma linha por clube
/// (separadores 0x6400, posição, seta de subida/descida, nome com atalho 0x7d5 e cor do humano, e as
/// colunas do registro pelo parâmetro 0x28), a rolagem (parâmetro 0x25) e o item da tabela no 0x26.
/// Sem liga ou sem o parâmetro 0xe (caixa "Error"): a original.
pub(crate) unsafe extern "C" fn match_screens_draw_live_league_table(league: usize, top: i32, base: i32) {
    let league = League(league);
    if league.is_null() || get_screen_param(0xe) == 0 {
        return original_fn::<unsafe extern "C" fn(usize, i32, i32)>(IDX_MATCH_SCREENS_DRAW_LIVE_LEAGUE_TABLE)(league.addr(), top, base);
    }
    let txt: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let w16 = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let b8 = |a: usize| *(a as *const u8);
    let z = |v: i32| v as usize;

    // the original’s frame ([esp+N] after the four registers): the variables in the same places
    // (+0x1c columns, +0x30 draw date, +0x38 today / league record, +0x40 and +0x180 matches,
    // +0x90 the local league, +0x1d0 the temporary one, +0x3d0 the date in words, +0xba0 the title)
    let mut quadro = [0u32; 0x1370 / 4];
    let s = quadro.as_mut_ptr() as usize;
    let gravar8 = |a: usize, v: u8| *(a as *mut u8) = v;
    for (k, &v) in [4u8, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4].iter().enumerate() {
        gravar8(s + 0x1c + k, v);
    }
    let local = League(s + 0x90);
    exe::new_league(local);
    exe::league_copy_for_display(local, s + 0x1d0, league);
    exe::drop_league(s + 0x1d0);
    let p0 = get_screen_param(0);
    let pe = get_screen_param(0xe);
    *((s + 0x14) as *mut usize) = pe;

    // the group’s matches of the day in the local copy
    let m = read_usize(p0 + live_match::ENGINE);
    if m != 0 && read_u16(m + engine::CLOCK) != 0 {
        let mut k = core::ptr::read_unaligned((pe + 4) as *const i32);
        while k <= core::ptr::read_unaligned((pe + 8) as *const i32) {
            let game = Fixture(u(*(G_MATCH_DAY as *const usize) + 0x19).wrapping_add((k as usize).wrapping_mul(0x69)) + 4);
            if u(pe) == u(game.addr()) && (game.home_goals() as u8) != 0xfd {
                if (game.home_goals() as u8) == 0xff {
                    let a = s + 0x40;
                    core::ptr::copy_nonoverlapping(game.addr() as *const u8, a as *mut u8, 0x4f);
                    if read_usize(a + fixture::HOME_CLUB_ID) == read_usize(p0 + live_match::HOME_CLUB_ID) {
                        let e = read_usize(p0 + live_match::RECORD);
                        gravar8(a + fixture::HOME_GOALS, read_u8(e + match_record::HOME_TEAM));
                        gravar8(a + fixture::AWAY_GOALS, read_u8(e + match_record::AWAY_TEAM));
                        gravar8(a + fixture::HOME_AGGREGATE, 0xff);
                        gravar8(a + fixture::AWAY_AGGREGATE, 0xff);
                        gravar8(a + fixture::HOME_PENALTIES, read_u8(e + match_record::HOME_SHOOTOUT_GOALS));
                        gravar8(a + fixture::AWAY_PENALTIES, read_u8(e + match_record::AWAY_SHOOTOUT_GOALS));
                    } else {
                        gravar8(a + fixture::HOME_GOALS, 0);
                        gravar8(a + fixture::AWAY_GOALS, 0);
                        gravar8(a + fixture::HOME_AGGREGATE, 0xff);
                        gravar8(a + fixture::AWAY_AGGREGATE, 0xff);
                        gravar8(a + fixture::HOME_PENALTIES, 0xff);
                        gravar8(a + fixture::AWAY_PENALTIES, 0xff);
                    }
                    gravar8(a + fixture::AWAY_GOALS_HALF_TIME, 0);
                    gravar8(a + fixture::HOME_GOALS_HALF_TIME, 0);
                    exe::league_apply_result_to_table(local, a, local.table(), read_usize(local.addr() + league::CLUB_COUNT) as u32, 1, 0, 0);
                } else {
                    let a = s + 0x180;
                    core::ptr::copy_nonoverlapping(game.addr() as *const u8, a as *mut u8, 0x4f);
                    exe::match_screens_tally_live_score(p0, k as u32, a);
                    exe::league_apply_result_to_table(local, a, local.table(), read_usize(local.addr() + league::CLUB_COUNT) as u32, 1, 0, 0);
                }
            }
            k += 1;
        }
    }
    exe::league_sort_table(local, 0);
    get_screen_param(0x27);
    let column = get_screen_param(0x28);
    get_screen_param(0x29);

    // the random draw still to come
    let sorteio = (league.next_draw_day() as u16);
    if sorteio != 0xffff {
        exe::new_date(s + 0x30, sorteio as u32, (league.next_draw_year_offset() as u16).wrapping_add((league.year() as u16)) as u32);
        core::ptr::copy_nonoverlapping(G_CURRENT_DATE as *const u8, (s + 0x38) as *mut u8, 8);
        let (hoje, dia) = (w16(s + 0x3a), w16(s + 0x32));
        if hoje < dia || (hoje == dia && (w16(s + 0x38) as i16) < (w16(s + 0x30) as i16)) {
            let mut y = top;
            let c = league.stage_id();
            if c != 0 {
                let mut k = exe::comp_util_get_parent_stage_id(c as i32);
                if k == -1 {
                    k = c as i32;
                }
                exe::comp_util_get_stage_name((BUF) as usize as *mut u8, (0x7d0) as u32 as i32, k as u32, 0);
                txt(sm, 0x6e, z(top), 0x30c, z(top + 0x23), 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), BUF, 1);
                y = top + 0x28;
            }
            let a = exe::font_get_height(3);
            let n = ((base - y - 0x10) / a) as u8;
            exe::date_get_long_date_string(s + 0x30, s + 0x3d0, 0x7d0, 0xff);
            let d = (exe::date_get_weekday(s + 0x30) as u32);
            let g = (exe::date_get_weekday_gender((exe::langlib_get_language() as u32), d) as u32);
            text(BUF, 0x99c2e8, g as u8 as i8 as i32, s + 0x3d0);
            mensagem(0x6e, z(y), 0x30c, z(base), 2, 0, n as usize, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0);
            exe::drop_league(local);
            return;
        }
    }

    // the title and the table
    let title = s + 0xba0;
    gravar8(s + 0x3d0, 0);
    let c = (league.stage_id() as u16);
    if c != 0 {
        exe::comp_util_get_stage_name((title) as usize as *mut u8, (0x7d0) as u32 as i32, c as u32, 0);
    } else {
        text(title, 0xa17b74);
    }
    txt(sm, 0x6e, z(top), 0x30c, z(top + 0x23), 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_YELLOW), title, 1);
    let y = top + 0x28;
    let clubs = league.club_count();
    let (alto, n);
    if clubs <= 8 && league.legs() > 0 && u(league.competition().addr()) != u(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        let mut k = (clubs as u8).wrapping_add(1);
        if league.promoted_count() != 0 || (league.league_flags() as u8) & 1 != 0 {
            k = k.wrapping_add(1);
        }
        for o in [0xbf, 0xc0, 0xc1] {
            if b8(league.addr() + o) != 0 {
                k = k.wrapping_add(1);
            }
        }
        n = k;
        alto = exe::font_get_height(3) * n as i32 + 0x10;
    } else {
        alto = base - y;
        let a = exe::font_get_height(3);
        n = ((alto - 0x10) / a) as u8;
    }
    let lst = list(current_human_display(), 0x6e, z(y), 0x30c, z(alto + y), 0xb, s + 0x1c, n as usize, 0, 2, 0, MAX);
    let cabecalho = |col: usize| {
        text_of_cell(sm, lst, col, 0, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
    };
    text(BUF, 0x990b3c);
    cabecalho(4);
    text(BUF, 0x990b1c, EMPTY_TEXT);
    cabecalho(5);
    let rule = (league.match_rules() as u16) as u8;
    text(BUF, if rule & 0x20 != 0 { 0x990b18 } else if rule & 1 != 0 { 0x99c24c } else { 0x990b14 });
    cabecalho(6);
    text(BUF, 0x990af4, EMPTY_TEXT);
    cabecalho(7);
    text(BUF, 0x990af0);
    cabecalho(8);
    text(BUF, 0x990aec);
    cabecalho(9);
    text(BUF, 0x990ae8);
    cabecalho(0xa);

    // the real league sorted by the column of parameter 0x28, for the position arrows
    league.set_table_column(column);
    exe::league_sort_table(league, 0);
    let mut queue = 1usize;
    let c8 = |o: usize| b8(local.addr() + o) as i8 as i32;
    let mut i = 0i32;
    while i < local.club_count() as i32 {
        let total = local.club_count() as i32;
        // the separators: top places (+0xbe, or 1 with +0xd9 & 1), playoff places (+0xbf), bottom places (+0xc0, +0xc1)
        let (v0, v1, v2, v3) = (c8(league::PROMOTED_COUNT), c8(league::PROMOTION_PLAYOFF_COUNT), c8(league::RELEGATION_PLAYOFF_COUNT), c8(league::RELEGATED_COUNT));
        let separador = (|| {
            if v0 == 0 {
                if (local.league_flags() as u8) & 1 != 0 && i == 1 {
                    return Some(G_GUI_COLOUR_YELLOW);
                }
            } else if v0 > 0 && i == v0 {
                return Some(G_GUI_COLOUR_YELLOW);
            }
            if v1 > 0 && i == v1 + v0 {
                return Some(0xaebdd4);
            }
            if v2 > 0 && i == total - v3 - v2 {
                return Some(0xaebdd4);
            }
            if v3 > 0 && i == total - v3 {
                return Some(G_GUI_COLOUR_YELLOW);
            }
            None
        })();
        if let Some(a) = separador {
            // [esp+0x14] of the original: the colour in the low word, parameter 0xe in the high one
            *((s + 0x14) as *mut u16) = *(a as *const u16);
            let x2 = if total as i16 <= n as i16 { 0x30a } else { 0x2f4 };
            row(current_human_display(), 1, 0x70, 0, x2, 0, 0, queue, 0x6400, u(s + 0x14), 0, 0, 0, 0, 0, 0, 0, 0, lst);
            queue += 1;
        }
        let r = exe::league_get_table_row(local, i);
        let club = Club(u(r));
        let nl = league.club_count() as i32;
        if nl > 0 {
            let mut j = 0;
            loop {
                let x = exe::league_get_table_row(league, j);
                *((s + 0x38) as *mut usize) = x;
                if club.addr() == u(x) {
                    break;
                }
                j += 1;
                if j >= league.club_count() as i32 {
                    break;
                }
            }
        }
        let pos = exe::utils_format_ordinal(b8(r + 4) as i8 as i32 + 1, 3);
        let ret = if league.legs() != 0 {
            cell(sm, lst, 0, queue, 0x30, colour_with_high_bits(pos, G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), pos, 7, r)
        } else {
            text_of_cell(sm, lst, 0, queue, 0x30, colour_with_high_bits(pos, G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), pos)
        };
        let x = u(s + 0x38);
        *(BUF as *mut u8) = 0;
        let (agora, antes) = (b8(r + 4) as i8, b8(x + 4) as i8);
        if agora < antes {
            row(current_human_display(), 0x20001, 0, 0, 0, 0, 1, queue, 1, colour_with_high_bits(ret, 0xaebdcc), 0, 0xc, 0, colour_word(G_GUI_COLOUR_YELLOW), 0, 0, 0, 0, lst);
        } else if agora > antes {
            row(current_human_display(), 0x40001, 0, 0, 0, 0, 1, queue, 1, colour_with_high_bits(ret, 0xaebdcc), 0, 0xc, 0, colour_word(G_GUI_COLOUR_RED), 0, 0, 0, 0, lst);
        }
        let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { colour_word(G_GUI_COLOUR_YELLOW) } else { colour_word(G_GUI_COLOUR_PALE_GREY) };
        cell(sm, lst, 2, queue, 1, 0, 1, 3, colour, club.addr() + club::SHORT_NAME, 0x7d5, club.addr());
        let d = r + column.wrapping_mul(14);
        for (o, col) in [(0x11usize, 4usize), (0x12, 5), (0x13, 6), (0x14, 7)] {
            format(BUF, FORMAT_D, b8(d + o) as i8 as i32);
            text_of_cell(sm, lst, col, queue, 1, 0, 0xc, 2, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
        }
        for (o, col) in [(0x15usize, 8usize), (0x17, 9)] {
            format(BUF, FORMAT_D, core::ptr::read_unaligned((d + o) as *const i16) as i32);
            text_of_cell(sm, lst, col, queue, 1, 0, 0xc, 2, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
        }
        format(BUF, FORMAT_D, core::ptr::read_unaligned((d + 0x19) as *const i16) as i32);
        text_of_cell(sm, lst, 0xa, queue, 0x30, colour_word(0xae3192), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
        queue += 1;
        i += 1;
    }

    let rolagem = get_screen_param(0x25);
    let disp = current_human_display();
    let id = lst as u16 as i16;
    if id != -1 {
        exe::area_set_scroll_row(disp.wrapping_add((id as isize * 0xbf1) as usize) + 4, rolagem);
    }
    set_screen_param(0x26, id as i32 as usize);
    let t = local.table();
    if t != 0 {
        exe::league_free_table(local, t);
    }
    let v = local.round_dates();
    if v != 0 {
        exe::crt_free(v);
    }
    exe::drop_league(local);
}

/// C strcpy.
///
/// [pt] strcpy de C (a original copia com `rep movs`).
pub(crate) unsafe fn match_screens_c_strcpy(d: *mut u8, s: *const u8) {
    let mut i = 0;
    loop {
        let c = *s.add(i);
        *d.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// C strcat.
///
/// [pt] strcat de C.
pub(crate) unsafe fn match_screens_c_strcat(d: *mut u8, s: *const u8) {
    let mut n = 0;
    while *d.add(n) != 0 {
        n += 1;
    }
    match_screens_c_strcpy(d.add(n), s);
}

/// Sets the team selection subtitle ("Team Selection" / "Please Confirm") via langlib into 0xde1f64 and the screen
/// manager.
///
/// [pt] O subtítulo da conferência ("Team Selection" ou "Please Confirm"): o texto traduzido em 0xde1f64 e
/// 0x803f80 no gerente.
pub(crate) unsafe fn set_selection_subtitle(t: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(0xde1f64, t);
    exe::screenmanager_add_screen_subtitle(SCREEN_MANAGER, 0xde1f64, 1);
}

/// Joins the names of n players into 0xde1f64 ("a", "a and b", "a, b, and c") and adds a text link for each.
///
/// [pt] Os nomes dos `n` jogadores de `lista` juntos em 0xde1f64 ("a", "a and b", "a, b, and c" pelos
/// textos traduzidos) e, para cada um, a ligação no Text `texto_obj` (0x4ae8a0, tipo 0x7dc na seleção,
/// senão 0x7d7).
pub(crate) unsafe fn join_selection_names(text_obj: usize, list: &[usize], n: i8, club: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut name = [0u8; 0xc8];
    let mut juntos = [0u8; 0x7d0];
    let mut k: i8 = 0;
    while k < n {
        let p = list[k as usize];
        exe::database_format_staff_name(p, (name.as_mut_ptr()) as *mut u8 as usize, 0xc8, 1);
        if k == 0 {
            match_screens_c_strcpy(juntos.as_mut_ptr(), name.as_ptr());
        } else {
            let template: usize = if k as i32 == n as i32 - 1 { if n == 2 { 0xa17b40 } else { 0xa17b18 } } else { 0xa17af4 };
            text(juntos.as_mut_ptr() as usize, template, 0xde1f64usize, name.as_ptr() as usize);
        }
        match_screens_c_strcpy(0xde1f64 as *mut u8, juntos.as_ptr());
        let kind = if exe::database_is_national_team(club) != 0 { 0x7dc } else { 0x7d7 };
        exe::text_add_link(text_obj, name.as_ptr(), kind, p);
        k += 1;
    }
}

/// The selection check warning: message `msg` in the Text (0x4ae660) in an area (0x5ed200) plus the "Ok" button
/// (0x55f990).
///
/// `caixa_sobre`/`ok_sobre`: where the original pushes the box colour / the button text colour with the high 16 bits
/// from the previous call's return value (varies by branch).
///
/// [pt] O aviso da conferência: a mensagem `msg` no Text (0x4ae660) numa área (0x5ed200) e o botão "Ok"
/// (0x55f990). `caixa_sobre`/`ok_sobre`: onde a original empurra a cor da caixa / a cor do texto do
/// botão com os 16 bits de cima da devolução da chamada anterior (varia por ramo).
pub(crate) unsafe fn show_selection_warning(text_obj: usize, msg: *const u8, caixa_sobre: bool, ok_sobre: bool) {
    let area: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA_FROM_TEXT);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let r = exe::text_set_string(text_obj, msg);
    let colour = if caixa_sobre { colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY) } else { colour_word(G_GUI_COLOUR_PALE_GREY) };
    area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, colour, text_obj);
    let r = text(0xde1f64, 0x990f7c) as usize;
    let colour_of_text = if ok_sobre { colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY) } else { colour_word(G_GUI_COLOUR_PALE_GREY) };
    row(current_human_display(), 2, 0x64, 0x22b, 0x316, 0x24e, 0, 0, 0x30, colour_with_high_bits(r, G_GUI_COLOUR_GREY), colour_word(G_GUI_COLOUR_GREY), 0xc, 4, colour_of_text, 0xde1f64, 0, 1, 0, usize::MAX);
}

/// Draws the team selection check for the club (param 0), the match (param 1) and the squad (param 2): warns (with
/// linked names) about players on international duty, on loan, unavailable, ineligible, banned, injured or without a
/// work permit, the bench limit or missing starters; otherwise asks for confirmation (captain, set piece takers,
/// substitutes) with "No" and "Yes".
///
/// Convention: cdecl, `match_screens.cpp`, ~7 KB with the pieces up to `0x727c2e`. It goes through the 20 slots of the
/// tactic (`0x8968f0`): counts starters (< 11), substitutes (< 11 + the bench `[jogo+0x4d]` & `0xf`) and those left
/// out; of those in the team, it gathers those called up by the national team (`0x7757d0`), those loaned to another
/// club (`0x4cf6d0`/`0x4cf5a0`, outside the national team), the ineligible ones with the same reason as the first
/// (`0x8cf5e0`), the suspended (`0x55d270`), the injured and the unavailable (`0x62c400`/`0x62ed70`: an injury of type
/// `0x4d` to `0x51` in language 0 counts as unavailable) and those without a work permit (`0x8c7560`, outside
/// competition `[0x9cf684]` and friendlies).
///
/// Then the club background, the menu, the colours and the title with the name. The first non-empty group, in that
/// order, becomes the "Team Selection" warning with the (linked) names and the "Ok" button; otherwise the day's bench
/// limit (`0x8cf880`); otherwise fewer than 11 starters; otherwise "Please Confirm": the chosen captain (`0x727e20`)
/// when there is no valid one, the set piece takers outside the 11, the missing substitutes, those outside the squad,
/// "Proceed to the match ?" and the list with "No" and "Yes".
/// Original: `match_screens_draw_team_selection` (0x7260f0, match_screens.cpp, cdecl).
///
/// [pt] { 0x7260f0 } (cdecl, match_screens.cpp, ~7 KB com os pedaços até 0x727c2e): desenha a conferência
/// da escalação do clube (parâmetro 0) para o jogo (parâmetro 1) e a equipe (parâmetro 2). Passa pelas
/// 20 vagas da tática (0x8968f0): conta titulares (< 11), reservas (< 11 + o banco [jogo+0x4d] & 0xf)
/// e os de fora; dos que estão no time, junta os convocados pela seleção (0x7757d0), os emprestados a
/// outro clube (0x4cf6d0/0x4cf5a0, fora da seleção), os inelegíveis com o mesmo motivo do primeiro
/// (0x8cf5e0), os suspensos (0x55d270), os lesionados e os indisponíveis (0x62c400/0x62ed70: lesão de
/// tipo 0x4d a 0x51 no idioma 0 conta como indisponível) e os sem permissão de trabalho (0x8c7560, fora
/// da competição [0x9cf684] e de amistoso). Depois o fundo do clube, o menu, as cores e o título com o
/// nome. O primeiro grupo não vazio, nessa ordem, vira o aviso "Team Selection" com os nomes (ligados)
/// e o botão "Ok"; senão o limite do banco do dia (0x8cf880); senão menos de 11 titulares; senão
/// "Please Confirm": o capitão escolhido (0x727e20) quando não há um válido, os cobradores de bola
/// parada fora dos 11, os reservas que faltam, os de fora do grupo, "Proceed to the match ?" e a lista
/// com "No" e "Yes".
pub(crate) unsafe extern "C" fn match_screens_draw_team_selection() {
    crate::comparison::branch("screens line-up check");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let area: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA_FROM_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let g = |a: usize| *(a as *const usize);
    const BUF: usize = 0xde1f64;
    const SEPARADOR: usize = 0x9ed918; // "\n\n"

    let mut obj = [0usize; 2];
    let t = obj.as_mut_ptr();
    let tu = t as usize;
    let mut reason: u8 = 0;
    exe::new_text(t, 0);
    let columns: [u8; 2] = [3, 1];
    let club = get_screen_param(0);
    let game = Fixture(get_screen_param(1));
    let equipe = get_screen_param(2);
    let bench = || ((game.squad_rules() as u8) & 0xf) as i8;
    let competition = || game.competition();

    let (mut starters, mut reservas, mut fora) = (0i8, 0i8, 0i8);
    let mut convocados = ([0usize; 20], 0i8);
    let mut emprestados = ([0usize; 20], 0i8);
    let mut indisponiveis = ([0usize; 20], 0i8);
    let mut inelegiveis = ([0usize; 20], 0i8);
    let mut suspensos = ([0usize; 20], 0i8);
    let mut lesionados = ([0usize; 20], 0i8);
    let mut sem_visto = ([0usize; 20], 0i8);
    let por = |g: &mut ([usize; 20], i8), p: usize| {
        g.0[g.1 as usize] = p;
        g.1 += 1;
    };
    for vaga in 0..0x14u8 {
        // simplification: the original pushes the slot as a dword with the top 3 bytes meaningless (the variable is a
        // char); the call uses the char, here it is zero-extended
        let p = exe::tactics_get_selected_player(g(G_TACTICS), club, vaga as u32, (equipe) as i32);
        if p == 0 {
            continue;
        }
        let v = vaga as i8;
        if v < 11 {
            starters += 1;
        } else if v < bench() + 0xb {
            reservas += 1;
        } else {
            fora += 1;
        }
        if v >= bench() + 0xb {
            continue;
        }
        if (exe::nationalteams_get_international_duty_club(g(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, club, (0) as u32, (0) as u32) as usize) != 0 {
            por(&mut convocados, p);
        }
        if exe::contract_manager_get_loan_contract(g(G_CONTRACT_MANAGER), p) != 0
            && exe::contract_manager_get_contract(g(G_CONTRACT_MANAGER), p) != 0
            && *((exe::contract_manager_get_contract(g(G_CONTRACT_MANAGER), p) + 4) as *const i32) == *(club as *const i32)
            && exe::database_is_national_team(club) == 0
        {
            por(&mut emprestados, p);
        }
        let m = exe::transfer_manager_get_ineligibility_code(g(G_TRANSFER_MANAGER), p, game);
        if m != 0 && (inelegiveis.1 == 0 || m == reason) {
            reason = m;
            por(&mut inelegiveis, p);
        }
        if exe::discipline_is_player_banned(g(G_DISCIPLINE), p, competition(), 1, 0) != 0 {
            por(&mut suspensos, p);
        }
        let tipo_ruim = |l: usize| l != 0 && (0x4d..=0x51).contains(&*(l as *const i8));
        if exe::injury_get_player_injury_status(g(G_INJURY), p) == 1 {
            let l = exe::injury_get_player_injury(g(G_INJURY), p);
            if tipo_ruim(l) {
                if exe::langlib_get_language() == 0 {
                    por(&mut indisponiveis, p);
                } else {
                    por(&mut lesionados, p);
                }
            }
        } else if exe::injury_get_player_injury_status(g(G_INJURY), p) == 2 {
            if exe::langlib_get_language() != 0 {
                por(&mut lesionados, p);
            } else if tipo_ruim(exe::injury_get_player_injury(g(G_INJURY), p)) {
                por(&mut indisponiveis, p);
            } else {
                por(&mut lesionados, p);
            }
        }
        if exe::transfer_manager_player_needs_work_permit(g(G_TRANSFER_MANAGER), p, club, 0) != 0
            && *(competition() as *const i32) != *(KEY_CLUB_COMP_RESERVE as *const i32)
            && exe::friendly_is_friendly_competition(g(G_FRIENDLY), competition()) == 0
        {
            por(&mut sem_visto, p);
        }
    }

    (exe::background_set_club_picture(club, 0, 0) as i32);
    exe::menubar_build(4, 0);
    // 0x53b640 writes u16; the rest of the original’s variable is stack garbage, here 0
    let (mut frente, mut fundo_da_cor) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut frente, &mut fundo_da_cor);
    exe::club_get_national_team_name(club, equipe, BUF, 0x7d0, 3, 1);
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, (fundo_da_cor as usize) as u32, (frente as usize) as u32, BUF);

    // simplification: an ineligibility reason outside 1 to 11 leaves the original’s message with whatever
    // was left on the stack; here it starts empty
    let mut msg = [0u8; 0x2710];
    let m = msg.as_mut_ptr();
    let mu = m as usize;
    let group = |g: &([usize; 20], i8), plural: usize, singular: usize, translate: bool| {
        set_selection_subtitle(0xa17b64);
        join_selection_names(tu, &g.0, g.1, club);
        let template = if g.1 > 1 { plural } else { singular };
        if translate {
            text(mu, template, BUF);
        } else {
            format(mu, template, BUF);
        }
    };
    if convocados.1 != 0 {
        group(&convocados, 0xa17a9c, 0xa17a4c, true);
        show_selection_warning(tu, m, false, false);
    } else if emprestados.1 != 0 {
        set_selection_subtitle(0xa17b64);
        join_selection_names(tu, &emprestados.0, emprestados.1, club);
        if emprestados.1 > 1 {
            text(mu, 0xa17a10, BUF);
        } else {
            let c = core::ptr::read_unaligned((emprestados.0[0] + 0x39) as *const usize);
            let gender = *((c + club::SHORT_GENDER_NAME) as *const i8) as i32;
            text(mu, 0xa179c8, gender, gender, BUF, c + club::SHORT_NAME);
        }
        show_selection_warning(tu, m, false, true);
    } else if indisponiveis.1 != 0 {
        group(&indisponiveis, 0xa1799c, 0xa17970, false);
        show_selection_warning(tu, m, false, false);
    } else if inelegiveis.1 != 0 {
        set_selection_subtitle(0xa17b64);
        join_selection_names(tu, &inelegiveis.0, inelegiveis.1, club);
        const PLURAL: [usize; 11] = [0xa1793c, 0xa17908, 0xa178c0, 0xa17878, 0xa1782c, 0xa177dc, 0xa1778c, 0xa1773c, 0xa176f4, 0xa17694, 0xa1763c];
        const SINGULAR: [usize; 11] = [0xa17610, 0xa175e0, 0xa175a0, 0xa1755c, 0xa17514, 0xa174cc, 0xa17484, 0xa1743c, 0xa173fc, 0xa173a8, 0xa17358];
        if (1..=11).contains(&reason) {
            let template = if inelegiveis.1 > 1 { PLURAL[reason as usize - 1] } else { SINGULAR[reason as usize - 1] };
            text(mu, template, BUF);
        }
        show_selection_warning(tu, m, true, false);
    } else if suspensos.1 != 0 {
        group(&suspensos, 0xa17328, 0xa17300, true);
        show_selection_warning(tu, m, false, false);
    } else if lesionados.1 != 0 {
        group(&lesionados, 0xa172b8, 0xa17278, true);
        show_selection_warning(tu, m, false, false);
    } else if sem_visto.1 != 0 {
        group(&sem_visto, 0xa17224, 0xa171d4, true);
        show_selection_warning(tu, m, false, false);
    } else if exe::transfer_manager_check_matchday_squad_limit(g(G_TRANSFER_MANAGER), club, game, (m) as u32, 0, 2, 0) != 0 {
        set_selection_subtitle(0xa17b64);
        show_selection_warning(tu, m, true, false);
    } else if starters < 11 {
        set_selection_subtitle(0xa17b64);
        let (mut maximo, mut quantos) = ([0u8; 0xc8], [0u8; 0xc8]);
        exe::utils_number_to_words((bench() + 0xb) as i32, maximo.as_mut_ptr(), 0xc8, 1);
        exe::utils_number_to_words(starters as i32, quantos.as_mut_ptr(), 0xc8, 1);
        match starters {
            0 => text(mu, 0xa17150, maximo.as_ptr()),
            1 => text(mu, 0xa170cc, maximo.as_ptr()),
            _ => text(mu, 0xa17038, quantos.as_ptr(), maximo.as_ptr()),
        };
        show_selection_warning(tu, m, false, true);
    } else {
        set_selection_subtitle(0x9909b8);
        *m = 0;
        let append = |s: usize| {
            if *m != 0 {
                match_screens_c_strcat(m, SEPARADOR as *const u8);
            }
            match_screens_c_strcat(m, s as *const u8);
        };
        let mut name = [0u8; 0xc8];
        let mut other = [0u8; 0xc8];
        // the captain: with no captain slot (negative) or outside the 11, the best starter (0x727e20)
        let c = (exe::tactics_find_captain_slot(g(G_TACTICS), club, (equipe) as u32, 1) as u8) as i8;
        if c < 0 || (exe::tactics_find_captain_slot(g(G_TACTICS), club, (equipe) as u32, 1) as u8) as i8 >= 0xb {
            let s = exe::match_screens_find_best_rated_starter_slot(club, equipe) as i8;
            if s > -1 {
                // simplification: as in the slot above, the top 3 bytes of the pushed dword are uninitialized
                let p = exe::tactics_get_selected_player(g(G_TACTICS), club, s as u8 as u32, (equipe) as i32);
                if p != 0 {
                    exe::database_format_staff_name(p, (name.as_mut_ptr()) as *mut u8 as usize, 0xc8, 1);
                    text(BUF, 0xa16fec, name.as_ptr());
                    append(BUF);
                    let kind = if exe::database_is_national_team(club) != 0 { 0x7dc } else { 0x7d7 };
                    exe::text_add_link(tu, name.as_ptr(), kind, p);
                }
            }
        }
        // the set-piece takers (5 roles × 2 choices) who are not in the 11, without repeating; the choice
        // goes in the low byte of the team’s dword (the original reuses the variable)
        let mut cobradores = ([0usize; 20], 0i8);
        for papel in 0..5usize {
            for escolha in 0..2usize {
                let p = exe::tactics_get_set_piece_taker(g(G_TACTICS), club, (papel) as i32, ((equipe & !0xff) | escolha) as i32, (equipe) as i32);
                if p != 0 {
                    let s = exe::tactics_find_selection_slot(g(G_TACTICS), club, p, (equipe) as u32) as i8;
                    if !(0..=10).contains(&s) && !cobradores.0[..cobradores.1 as usize].contains(&p) {
                        por(&mut cobradores, p);
                    }
                }
            }
        }
        if cobradores.1 > 0 {
            join_selection_names(tu, &cobradores.0, cobradores.1, club);
            let mut aviso = [0u8; 0x7c0];
            text(aviso.as_mut_ptr() as usize, if cobradores.1 > 1 { 0xa16f8c } else { 0xa16f34 }, BUF);
            append(aviso.as_ptr() as usize);
        }
        if reservas < bench() {
            exe::utils_number_to_words(bench() as i32, name.as_mut_ptr(), 0xc8, 1);
            exe::utils_number_to_words(reservas as i32, other.as_mut_ptr(), 0xc8, 1);
            match reservas {
                0 => text(BUF, 0xa16eec, name.as_ptr()),
                1 => text(BUF, 0xa16ea4, name.as_ptr()),
                _ => text(BUF, 0xa16e50, other.as_ptr(), name.as_ptr()),
            };
            append(BUF);
        }
        if fora > 0 {
            exe::utils_number_to_words(fora as i32, name.as_mut_ptr(), 0xc8, 1);
            if fora == 1 {
                text(BUF, 0xa16e14);
            } else {
                text(BUF, 0xa16dcc, name.as_ptr());
            }
            append(BUF);
        }
        text(BUF, 0xa16db4);
        append(BUF);
        exe::text_set_string(tu, m);
        area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, colour_word(G_GUI_COLOUR_PALE_GREY), tu);
        let id = list(current_human_display(), 0x64, 0x22b, 0x316, 0x24e, 2, columns.as_ptr() as usize, 1, 0, 1, 0, usize::MAX);
        let r = text(BUF, 0x98e8cc) as usize;
        cell(SCREEN_MANAGER, id, 0, 0, 0x30, colour_with_high_bits(r, G_GUI_COLOUR_GREY), 0xc, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 2, 0);
        text(BUF, 0x98e8d0);
        cell(SCREEN_MANAGER, id, 1, 0, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 3, 0);
    }
    exe::drop_text(t);
}

/// Reads the record index, the four bucket cursors and the counts from `match_stats.dat` in the saved game. Returns 1
/// if it read.
///
/// Reads from the save's "match_stats.dat" (`0x9326e0` in zip `zip`) the count at `+4`, the index (12 bytes each,
/// allocated in `[this]`), the four bucket cursors (`+8`), `+0x1c` and `+0x18`. Null zip: the original routine runs
/// (error box). No file: error `0x49e`; read or allocation failure: error
/// `0x4a5`/`0x4ae`/`0x4b6`/`0x4be`/`0x4c6`/`0x4ce` and it closes.
/// Original: `Match_Stats::load` (0x72aa80, match_stats.cpp, thiscall).
///
/// [pt] { 0x72aa80 } (thiscall, ret 4, match_stats.cpp): lê do "match_stats.dat" do save (0x9326e0 no zip
/// `zip`) a quantidade em +4, o índice (12 bytes cada, alocado em [this]), os quatro cursores dos
/// baldes (+8), +0x1c e +0x18. Zip nulo: a original (caixa de erro). Sem o arquivo: erro 0x49e; falha
/// de leitura ou de alocação: erro 0x4a5/0x4ae/0x4b6/0x4be/0x4c6/0x4ce e fecha. Devolve 1 se leu.
pub(crate) unsafe extern "thiscall" fn match_screens_load(stats: usize, zip: usize) -> u32 {
    if zip == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_LOAD)(stats, zip);
    }
    crate::comparison::branch("screens statistics load");
    let mut h = exe::zopen(zip, 0xa17bd0, 0, 0x16, 4);
    if h == 0 {
        show_error_at(0xa17ba0, 0x49e);
        return 0;
    }
    let falha = |row: u32, mut h: usize| {
        show_error_at(0xa17ba0, row);
        exe::zclose(&mut h);
        0
    };
    let n = (stats + match_stats::COUNT) as *mut i32;
    if exe::zread(n as usize, 4, 1, h) != 1 {
        return falha(0x4a5, h);
    }
    let table = exe::crt_operator_new((*n).wrapping_mul(12) as u32);
    *(stats as *mut usize) = table;
    if table == 0 {
        return falha(0x4ae, h);
    }
    if exe::zread(table, 0xc, *n as u32, h) != *n as u32 {
        return falha(0x4b6, h);
    }
    if exe::zread(stats + match_stats::BUCKET_CURSORS, 4, 4, h) != 4 {
        return falha(0x4be, h);
    }
    if exe::zread(stats + match_stats::CAPACITY, 4, 1, h) != 1 {
        return falha(0x4c6, h);
    }
    if exe::zread(stats + match_stats::EVENT_FILE_NUMBER, 4, 1, h) != 1 {
        return falha(0x4ce, h);
    }
    exe::zclose(&mut h);
    1
}

/// Writes the record index, the four bucket cursors and the counts to `match_stats.dat` in the saved game. Returns 1 if
/// it wrote.
///
/// Writes to the save's "match_stats.dat" (`0x9326e0` for writing) the count (`+4`), the index `[this]`, the four
/// cursors (`+8`), `+0x1c` and `+0x18`. Null zip: the original routine runs (error box); if it cannot open, the game
/// stops (error `0x4f1`); write failure: error `0x4f8`/`0x500`/`0x508`/`0x510`/`0x518` and it closes.
/// Original: `Match_Stats::save` (0x72adf0, match_stats.cpp, thiscall).
///
/// [pt] { 0x72adf0 } (thiscall, ret 4, match_stats.cpp): grava no "match_stats.dat" do save (0x9326e0 em
/// escrita) a quantidade (+4), o índice [this], os quatro cursores (+8), +0x1c e +0x18. Zip nulo: a
/// original (caixa de erro); sem abrir, o jogo para (erro 0x4f1); falha de escrita: erro
/// 0x4f8/0x500/0x508/0x510/0x518 e fecha. Devolve 1 se gravou.
pub(crate) unsafe extern "thiscall" fn match_screens_save(stats: usize, zip: usize) -> u32 {
    if zip == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_SAVE)(stats, zip);
    }
    crate::comparison::branch("screens statistics save");
    let mut h = exe::zopen(zip, 0xa17bd0, 1, 0x16, 4);
    if h == 0 {
        fatal_error_at(0xa17ba0, 0x4f1);
    }
    let falha = |row: u32, mut h: usize| {
        show_error_at(0xa17ba0, row);
        exe::zclose(&mut h);
        0
    };
    let n = (stats + match_stats::COUNT) as *const i32;
    if exe::zwrite(n as usize, 4, 1, h) != 1 {
        return falha(0x4f8, h);
    }
    if exe::zwrite(*(stats as *const usize), 0xc, *n as u32, h) != *n as u32 {
        return falha(0x500, h);
    }
    if exe::zwrite(stats + match_stats::BUCKET_CURSORS, 4, 4, h) != 4 {
        return falha(0x508, h);
    }
    if exe::zwrite(stats + match_stats::CAPACITY, 4, 1, h) != 1 {
        return falha(0x510, h);
    }
    if exe::zwrite(stats + match_stats::EVENT_FILE_NUMBER, 4, 1, h) != 1 {
        return falha(0x518, h);
    }
    exe::zclose(&mut h);
    1
}

// simplification: loading and saving touch the save file (and loading allocates the index): running twice
// would read/write twice, so the comparer only calls the original; the copies are proven in Rust mode.

/// Os registros de estatística da partida têm 0xd7a bytes no "match_stats_matches.tmp" de CM3_TEMP.
pub(crate) const MATCH_STATS_RECORD_SIZE: u32 = 0xd7a;

/// Reads one match statistics record (`0xd7a` bytes) from `match_stats_matches.tmp` by index. Returns 1 if it read.
///
/// Reads record `k` (`0xd7a` bytes, at position k × `0xd7a`) of "match_stats_matches.tmp" (CM3_TEMP, "rb") into
/// `saida`. Index −1: returns 0 silently; outside 0..`[this+0x1c]` or null output: the original routine runs (error
/// box). Cannot open: error `0x128`; fseek or fread fail: error `0x12f`/`0x137` and it closes.
/// Original: `Match_Stats::read_match_record` (0x729600, match_stats.cpp, thiscall).
///
/// [pt] { 0x729600 } (thiscall, ret 8, match_stats.cpp): lê o registro `k` (0xd7a bytes, na posição
/// k × 0xd7a) do "match_stats_matches.tmp" (CM3_TEMP, "rb") para `saida`. Índice −1: devolve 0 calado;
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro). Sem abrir: erro 0x128; fseek ou
/// fread falham: erro 0x12f/0x137 e fecha. Devolve 1 se leu.
pub(crate) unsafe extern "thiscall" fn read_match_record(stats: usize, k: i32, out: usize) -> u32 {
    if k < 0 || k > (*((stats + match_stats::CAPACITY) as *const i32)).wrapping_sub(1) || out == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_READ_MATCH_RECORD)(stats, k, out);
    }
    crate::comparison::branch("statistics read record");
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xa17b88), 0x98f928);
    if f == 0 {
        show_error_at(0xa17ba0, 0x128);
        return 0;
    }
    if exe::crt_fseek(f, k.wrapping_mul(MATCH_STATS_RECORD_SIZE as i32), 0) != 0 {
        show_error_at(0xa17ba0, 0x12f);
        exe::crt_fclose(f);
        return 0;
    }
    if exe::crt_fread(out, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {
        show_error_at(0xa17ba0, 0x137);
        exe::crt_fclose(f);
        return 0;
    }
    exe::crt_fclose(f);
    1
}

/// Records a finished match: fills its record and writes it, except for excluded matches. Returns 1.
///
/// Records finished match `m`: fills record `r` (`0x729ff0`, without skipping the gate receipts) and, except for a
/// match of competition `[0x9cf684]` with `+0x4c` in the record, writes it (`0x729820`) with fixture `[m+0x4792]` and
/// events `[m+0x8f7a]`. Null match or record: the original routine runs (error box, returns 0).
/// Original: `Match_Stats::record_match` (0x7293f0, match_stats.cpp, thiscall).
///
/// [pt] { 0x7293f0 } (thiscall, ret 8, match_stats.cpp): registra a partida `m` que terminou: preenche o
/// registro `r` (0x729ff0, sem pular a renda) e, salvo jogo da competição [0x9cf684] com +0x4c no
/// registro, grava (0x729820) com o jogo [m+0x4792] e os eventos [m+0x8f7a]. Partida ou registro
/// nulos: a original (caixa de erro, devolve 0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn match_screens_record_match(stats: usize, m: usize, r: usize) -> u32 {
    if m == 0 || r == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_MATCH_SCREENS_RECORD_MATCH)(stats, m, r);
    }
    crate::comparison::branch("statistics record match");
    exe::match_stats_fill_match_record(stats, m, r, 0);
    let game = core::ptr::read_unaligned((m + engine::MATCH) as *const usize);
    if *(game as *const i32) != *(KEY_CLUB_COMP_RESERVE as *const i32) || *((r + match_record::NO_STATISTICS) as *const u8) == 0 {
        exe::match_stats_write_match_record(stats, game, r, core::ptr::read_unaligned((m + engine::COMMENTATOR) as *const usize));
    }
    1
}

/// Writes the events and the record into the fixture's slot (or the bucket cursor's slot, which wraps around) and
/// updates the index. Returns 1.
///
/// Writes record `r` of fixture `jogo` to "match_stats_matches.tmp" ("r+b"): zeroes the 13 bytes from `+0xd69`, writes
/// the events (`0x6d98b0` of `eventos`, human if one of the clubs `+0x1c`/`+0x20` is human-managed, `0x600a50`) and
/// writes the record into slot `[jogo+8]`; with no slot (−1), into the slot of the bucket cursor (`0x729bd0`) at
/// `[this+8+4b]`, which advances and goes back to the start of the bucket on reaching its end ((b+1) × `[this+0x1c]` /
/// 4).
///
/// At the end, entry `[jogo+8]` of the index (12 bytes in `[this]`) receives `+0xc`, `+0x10` and the dates
/// `+0x2a`/`+0x28`. Null argument: the original routine runs (error box). File failures: error
/// `0x172`/`0x17f`/`0x18d`/`0x1a3` (closing, as the original does).
/// Original: `Match_Stats::write_match_record` (0x729820, match_stats.cpp, thiscall).
///
/// [pt] { 0x729820 } (thiscall, ret 0xc, match_stats.cpp): grava o registro `r` do jogo `jogo` no
/// "match_stats_matches.tmp" ("r+b"): zera os 13 bytes de +0xd69, grava os eventos (0x6d98b0 de
/// `eventos`, humano se um dos clubes +0x1c/+0x20 é de humano, 0x600a50) e escreve o registro na vaga
/// [jogo+8]; sem vaga (−1), na vaga do cursor do balde (0x729bd0) em [this+8+4b], que anda e volta ao
/// começo do balde ao chegar ao fim ((b+1) × [this+0x1c] / 4). No fim, a entrada [jogo+8] do índice
/// (12 bytes em [this]) recebe +0xc, +0x10 e as datas +0x2a/+0x28. Argumento nulo: a original (caixa
/// de erro). Falhas do arquivo: erro 0x172/0x17f/0x18d/0x1a3 (fechando, como a original). Devolve 1.
pub(crate) unsafe extern "thiscall" fn write_match_record(stats: usize, game: usize, r: usize, events: usize) -> u32 {
    let game = Fixture(game);
    if game.is_null() || r == 0 || events == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_WRITE_MATCH_RECORD)(stats, game.addr(), r, events);
    }
    crate::comparison::branch("statistics write record");
    let d = |o: usize| (stats + o) as *mut i32;
    let j = |o: usize| (game.addr() + o) as *mut i32;

    let desc = r + match_record::EVENT_LOG;
    core::ptr::write_bytes(desc as *mut u8, 0, 13);
    let h = (exe::human_manager_is_human_club(game.home_club(), 1, 1, 0, 0) != 0 || exe::human_manager_is_human_club(game.away_club(), 1, 1, 0, 0) != 0) as u32;
    exe::match_events_save_event_log(events, desc, h);
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xa17b88), 0x9c7848);
    if f == 0 {
        show_error_at(0xa17ba0, 0x172);
        return 0;
    }
    let slot = *j(8);
    if slot != -1 {
        if exe::crt_fseek(f, slot.wrapping_mul(MATCH_STATS_RECORD_SIZE as i32), 0) != 0 {
            show_error_at(0xa17ba0, 0x17f);
            exe::crt_fclose(f);
            return 0;
        }
    } else {
        let b = exe::match_stats_get_record_bucket(stats, game, r) as i8 as isize;
        let cursor = d(8 + 4 * b as usize);
        if exe::crt_fseek(f, (*cursor).wrapping_mul(MATCH_STATS_RECORD_SIZE as i32), 0) != 0 {
            show_error_at(0xa17ba0, 0x18d);
            exe::crt_fclose(f);
            return 0;
        }
        *j(8) = *cursor;
        let c = (*cursor).wrapping_add(1);
        *cursor = c;
        let total = *d(match_stats::CAPACITY);
        if c == (b as i32 + 1).wrapping_mul(total) / 4 {
            *cursor = c - total / 4;
        }
    }
    if exe::crt_fwrite(r, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {
        show_error_at(0xa17ba0, 0x1a3);
        exe::crt_fclose(f);
        return 0;
    }
    exe::crt_fclose(f);
    let e = (*(stats as *const usize) as isize + *j(8) as isize * 12) as usize;
    *(e as *mut i32) = *j(fixture::HOME_CLUB_ID);
    *((e + match_stats_entry::AWAY_CLUB_ID) as *mut i32) = *j(fixture::AWAY_CLUB_ID);
    *((e + match_stats_entry::DAY) as *mut u16) = game.day();
    *((e + match_stats_entry::YEAR) as *mut u16) = game.year();
    1
}

// simplification: all three touch the temporary file (and saving also the events and the index): running
// twice would write the record twice, so the comparer only calls the original; the copies are
// prove in Rust mode (the branches) and by the career with the mutations.

/// 16-bit colour stored at a, as u32.
///
/// [pt] Cor (16 bits) guardada em `a`. A original empilha registradores com os 16 bits de cima sujos; quem
/// recebe (0x55f990 pelas células, 0x801960/0x801040) só usa a palavra de baixo (ver o painel 0x723d20).
pub(crate) unsafe fn colour_word_u32(a: usize) -> u32 {
    *(a as *const u16) as u32
}

/// The regions a match-screen routine touches: those of the screen, the network buffer position ([0xae290c]) and, in
/// the updates (`exe::screenmanager_set_widget_text` and company, which become a packet for the `Display`), the manager's local `Display`
/// (+0x3844).
///
/// An update that goes over the network (slot with [+0x7d4] ≠ 0): `None` (the packet would be sent twice).
///
/// [pt] As regiões que uma rotina das telas da partida mexe: as da tela, a posição do buffer de rede
/// ([0xae290c]) e, nas atualizações (0x801c70 e companhia, que viram um pacote para o `Display`), o
/// `Display` local do gerente (+0x3844). Atualização que vai pela rede (vaga com [+0x7d4] ≠ 0): `None`
/// (o pacote seria enviado duas vezes).
pub(crate) unsafe fn screen_update_regions(montagem: bool) -> Option<Vec<(usize, usize)>> {
    let mut r = screen_build_regions();
    r.push((0xae290c, 4));
    if montagem {
        return Some(r);
    }
    let slot = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    if core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7d4 + slot * 0x300) as *const u16) != 0 {
        return None;
    }
    let local = SCREEN_MANAGER + 0x3844;
    if !r.iter().any(|x| x.0 == local) {
        r.push((local, 0x12f550));
    }
    Some(r)
}

/// First differing byte of each memory region (comparer message).
///
/// [pt] O primeiro byte diferente de cada região (para a mensagem do comparador).
pub(crate) fn first_differences(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<String> {
    mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
        let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
        Some(format!("{:#x}+{:#x}", a, k))
    }).collect()
}

/// Appends text to the 0xde1f64 buffer (the original's strcat).
///
/// [pt] Acrescenta o texto em `origem` ao fim do texto 0xde1f64 (o strcat da original).
pub(crate) unsafe fn append_to_text_buffer(source: *const u8) {
    let mut end = 0xde1f64usize;
    while *(end as *const u8) != 0 {
        end += 1;
    }
    let mut i = 0;
    loop {
        let b = *source.add(i);
        *((end + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Colunas 5 a 24 das estatísticas do time: a cor e o campo (byte da linha do jogador) mostrado.
pub(crate) const TEAM_STATS_FIELDS: [(usize, usize); 20] = [
    (G_GUI_COLOUR_YELLOW, 0x1ce), (G_GUI_COLOUR_YELLOW, 0x1cf), (G_GUI_COLOUR_YELLOW, 0x1d0), (0xaebdd4, 0x1d1), (0xaebdd4, 0x1d2),
    (0xaebdd4, 0x1d3), (G_GUI_COLOUR_YELLOW, 0x1d4), (G_GUI_COLOUR_YELLOW, 0x1d5), (G_GUI_COLOUR_YELLOW, 0x1d6), (0xaebdd4, 0x1b7),
    (0xaebdd4, 0x1b5), (0xaebdd4, 0x1b6), (G_GUI_COLOUR_YELLOW, 0x1d7), (G_GUI_COLOUR_YELLOW, 0x1b8), (G_GUI_COLOUR_YELLOW, 0x1bb),
    (0xaebdd4, 0x1b9), (0xaebdd4, 0x1ba), (0xaebdd4, 0x1b0), (0xaebde8, 0x1c5), (G_GUI_COLOUR_YELLOW, 0x1b1),
];

/// Writes the 20 player rows of the team stats table of side `[param 0x1b]` (22 columns from the param `0x1c` table),
/// building the cells or updating what changed.
///
/// Convention: cdecl, `match_screens.cpp`, ~4.2 KB. Data: in the match `[p0 + 0x5f]` and p0 + 4, outside it params
/// `0x20` and `0x1f`; row i at data + 1573·side + 73·i, empty with `0xfb` at `+0x1ad`. The 22 columns come from the
/// table of param `0x1c` (1: `0xa16178`, 2: `0xa1618c`): number (or i + 1), two markers, the name ("(c)" for the
/// captain; button `0x7dc`/`0x7d7` when the person checks out, `0x53e990`), the event (injury, sending off, coming on,
/// going off) and the numbers.
///
/// With param `0x1d` (the previous snapshot), an unchanged cell gets no text. `modo` ≠ 0 builds the cells in list
/// `area` (`0x8040a0` button, `0x804100` text; the first item in param `0x1a` if it is at −1); `modo` 0 updates the
/// items from param `0x1a` (`0x801c70`, `0x800d30`, `0x801960`). Param `0x1c` other than 1 and 2 (the original reads
/// the table with no value): the original routine runs.
/// Original: `fill_team_stats_rows` (0x721cd0, match_screens.cpp, cdecl).
///
/// [pt] { 0x721cd0 } (cdecl, match_screens.cpp, ~4,2 KB): as 20 linhas de jogador das estatísticas do time
/// do lado [parâmetro 0x1b] (dados: na partida [p0 + 0x5f] e p0 + 4, fora dela os parâmetros 0x20 e
/// 0x1f; linha i em dados + 1573·lado + 73·i, vazia com 0xfb em +0x1ad). As 22 colunas vêm da tabela
/// do parâmetro 0x1c (1: 0xa16178, 2: 0xa1618c): número (ou i + 1), dois marcadores, o nome ("(c)" no
/// capitão; botão 0x7dc/0x7d7 quando a pessoa confere, 0x53e990), o evento (lesão, expulsão, entrada,
/// saída) e os números. Com o parâmetro 0x1d (a foto anterior), célula sem mudança fica sem texto.
/// `modo` ≠ 0 monta as células na lista `area` (0x8040a0 botão, 0x804100 texto; o primeiro item no
/// parâmetro 0x1a se estiver em −1); `modo` 0 atualiza os itens a partir do parâmetro 0x1a (0x801c70,
/// 0x800d30, 0x801960). Parâmetro 0x1c fora de 1 e 2 (a original lê a tabela sem valor): a original.
pub(crate) unsafe extern "C" fn fill_team_stats_rows(mode: u32, area: usize) {
    type T9 = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, u32, u32, u32, usize) -> u32;
    type T11 = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, u32, u32, u32, usize, u32, usize) -> u32;
    let cell: T11 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: T9 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;

    let side = get_screen_param(0x1b) as u8 as i8;
    let p0 = get_screen_param(0);
    let before = get_screen_param(0x1d);
    let table = match get_screen_param(0x1c) as u8 {
        1 => G_TEAM_STATS_COLUMN_IDS_VIEW_1,
        2 => G_TEAM_STATS_COLUMN_IDS_VIEW_2,
        _ => return original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_FILL_TEAM_STATS_ROWS)(mode, area),
    };
    let data = if p0 != 0 {
        core::ptr::read_unaligned((p0 + live_match::RECORD) as *const usize)
    } else {
        let d = get_screen_param(0x20);
        get_screen_param(0x1f);
        d
    };
    let mut item = get_screen_param(0x1a) as u32;
    let base_side = (side as i32).wrapping_mul(1573);
    // the cell state carries from one to the next, as in the original (the column goes back to 1 only in column 0)
    let (mut coluna, mut cor, mut fundo, mut fonte, mut marcas, mut estilo) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    for i in 0..20i32 {
        let row = base_side.wrapping_add(73 * i) as usize;
        let reg = data.wrapping_add(row);
        let b = |o: usize| *((reg + o) as *const u8);
        let bi = |o: usize| *((reg + o) as *const i8);
        for j in 0..22usize {
            let mut botao: (u32, usize) = (0, 0);
            *(BUF as *mut u8) = 0;
            let photo = if before != 0 { row.wrapping_add(before).wrapping_add(0x190) } else { 0 };
            // the row’s `o` field equal to the previous snapshot’s
            let same = |o: usize| *((photo + o - 0x190) as *const u8) == b(o);
            let id = core::ptr::read_unaligned((reg + match_record::ROW_STAFF_ID) as *const i32);
            if b(match_record::ROW_POSITION) == 0xfb {
                continue;
            }
            let code = *((table + j) as *const u8) as usize;
            if code > 0x18 {
                continue;
            }
            match code {
                0 => {
                    cor = colour_word_u32(G_GUI_COLOUR_PALE_GREY);
                    coluna = 1;
                    fonte = 0xc;
                    fundo = if i < 0xb { colour_word_u32(G_GUI_COLOUR_DARK_BLUE) } else { exe::graphics_rgb_to_pixel(0, 0, 0x40, 0) };
                    let n = bi(0x1ac);
                    marcas = 0x30;
                    estilo = 1;
                    format(BUF, 0x98806c, if n > 0 { n as i32 } else { i + 1 });
                    // simplification: the original compares i with ([info + 0x4d] & 0xf) + 0xb, but both branches give 0x30
                }
                1 | 2 => {
                    cor = colour_word_u32(G_GUI_COLOUR_GREY);
                    marcas = 1;
                    coluna = code as u32 + 2;
                    fonte = 0xc;
                    fundo = 0;
                    estilo = 1;
                    if !(photo != 0 && same(0x1bc) && same(0x1bd)) {
                        let (a, d) = (b(0x1bc), b(0x1bd));
                        // the card colour goes in the cell background ([esp+0x1c] in the original), not in the text
                        if code == 1 {
                            if a != 0 {
                                marcas = 0x20030;
                                fundo = colour_word_u32(G_GUI_COLOUR_YELLOW);
                            } else if d != 0 {
                                marcas = 0x20030;
                                fundo = colour_word_u32(G_GUI_COLOUR_RED);
                            }
                        } else if a != 0 && d != 0 {
                            fundo = colour_word_u32(G_GUI_COLOUR_RED);
                            marcas = 0x40030;
                        }
                    }
                }
                3 => {
                    let (n1, n2, n3) = (
                        core::ptr::read_unaligned((reg + match_record::ROW_FIRST_NAME_ID) as *const u32),
                        core::ptr::read_unaligned((reg + match_record::ROW_SECOND_NAME_ID) as *const u32),
                        core::ptr::read_unaligned((reg + match_record::ROW_COMMON_NAME_ID) as *const u32),
                    );
                    marcas = 1;
                    estilo = 2;
                    coluna = 5;
                    fonte = 1;
                    fundo = 0;
                    let mut name = [0u8; 0xc8];
                    exe::database_format_name_from_ids(n1, n2, n3, name.as_mut_ptr(), 0xc8, 1, id as u32);
                    let capitao = *((data.wrapping_add(base_side as usize) + 0x153) as *const i8) as i32;
                    if i == capitao {
                        text(BUF, 0xa16a7c, name.as_ptr());
                    } else {
                        copy_to_work_text(name.as_ptr() as usize);
                    }
                    if id >= 0 && id < *(G_STAFF_COUNT as *const i32) {
                        let person = (*(G_STAFF_TABLE as *const usize)).wrapping_add(id as usize * 110);
                        if exe::database_staff_matches_names(person, n1, n2, n3, reg + match_record::ROW_DATE_OF_BIRTH) != 0 {
                            let v = if side == 0 { *(data as *const i32) } else { *((data + match_record::AWAY_CLUB_ID) as *const i32) };
                            let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
                            botao = (if v >= limit { 0x7dc } else { 0x7d7 }, person);
                        }
                    }
                    if core::ptr::read_unaligned((reg + match_record::ROW_PLAYER_ID) as *const u32) == core::ptr::read_unaligned((data + match_record::MAN_OF_THE_MATCH_ID) as *const u32) {
                        cor = if *(G_PLAYER_SETUP_COLOURBLIND_FLAG as *const u8) != 0 { colour_word_u32(G_GUI_COLOUR_YELLOW) } else { colour_word_u32(0xaebde8) };
                    } else if bi(0x1c3) > -1 || bi(match_record::ROW_MINUTE_LEFT_PITCH) > -1 || bi(match_record::ROW_MINUTE_OFF) > -1 {
                        cor = colour_word_u32(G_GUI_COLOUR_GREY);
                    } else if i <= 0xa || bi(0x1c2) != -1 {
                        cor = colour_word_u32(G_GUI_COLOUR_PALE_GREY);
                    } else {
                        cor = colour_word_u32(G_GUI_COLOUR_GREY);
                    }
                }
                4 => {
                    cor = colour_word_u32(0xae3154);
                    estilo = 1;
                    coluna = 6;
                    fonte = 1;
                    fundo = 0;
                    marcas = 1;
                    if !(i >= 0xb && bi(0x1c2) <= -1)
                        && !(photo != 0 && same(0x1c3) && same(0x1c4) && same(0x1c0) && same(0x1c2))
                    {
                        if bi(0x1c3) > -1 {
                            text(BUF, 0xa16a68, bi(0x1c3) as i32 + 1);
                        } else if bi(match_record::ROW_MINUTE_LEFT_PITCH) > -1 {
                            text(BUF, 0xa16a54, bi(match_record::ROW_MINUTE_LEFT_PITCH) as i32 + 1);
                        } else if bi(match_record::ROW_MINUTE_OFF) > -1 {
                            text(BUF, 0xa16a40, bi(match_record::ROW_MINUTE_OFF) as i32 + 1);
                        } else if bi(0x1c2) > -1 {
                            text(BUF, 0xa16a2c, bi(0x1c2) as i32 + 1);
                            cor = colour_word_u32(0xaebdd4);
                        }
                    }
                }
                _ => {
                    let (endereco_da_cor, campo) = TEAM_STATS_FIELDS[code - 5];
                    coluna = coluna.wrapping_add(1);
                    cor = colour_word_u32(endereco_da_cor);
                    marcas = 1;
                    fonte = 0xc;
                    fundo = 0;
                    estilo = 1;
                    if !(i >= 0xb && bi(0x1c2) <= -1) && !(photo != 0 && same(campo) && same(0x1c2)) {
                        match code {
                            22 => {
                                format(BUF, 0x98806c, bi(campo) as i32);
                                cor = colour_word_u32(0xaebdde);
                            }
                            24 => {
                                if b(campo) != 0 {
                                    format(BUF, 0x98806c, b(campo) as u32);
                                }
                            }
                            _ => {
                                format(BUF, 0x98806c, b(campo) as u32);
                            }
                        }
                    }
                }
            }
            if mode != 0 {
                let r = if botao.1 != 0 {
                    cell(sm, area, coluna, i + 1, marcas, fundo, fonte, estilo, cor, BUF, botao.0, botao.1)
                } else {
                    text_of_cell(sm, area, coluna, i + 1, marcas, fundo, fonte, estilo, cor, BUF)
                };
                if item as u16 == 0xffff {
                    set_screen_param(0x1a, r as u16 as i16 as i32 as usize);
                    item = r;
                }
            } else {
                exe::screenmanager_set_widget_text(sm, item, BUF);
                exe::screenmanager_set_widget_style(sm, item, marcas);
                exe::screenmanager_set_widget_text_colour(sm, item, cor);
                item = item.wrapping_add(1);
            }
        }
    }
}

/// Rewrites the latest scores rows in pages: stage, fixture, score, period, event scorers, aggregate and the page
/// footer.
///
/// Convention: cdecl, `match_screens.cpp`, ~4.9 KB. Params: 0 the match or null, `0xe` the day's entry, `0xf` how many
/// rows, `0x10` the first item, 6 items per row, `0x11` the page shown, `0x12` paused. It clears the rows, then splits
/// the entry's matches (`[+4]` to `[+8]`, 105 bytes in `[[0xb67a50]+0x19]`) into pages (`0x13` = how many): per match
/// the stage header (`0x4bffc0`, when it changes), the fixture row (teams, score counted from the events, period
/// `0x6b6570`) and the event scorers (goal, penalty, own goal, missed penalty, sending off, injury; minutes of the same
/// player together) and the aggregate. Only the page shown is written (second pass). In the footer "<< Page x/y >>" (or
/// "Hold") with more than one page; at the end the time in param `0x14`.
/// Original: `refresh_latest_scores_rows` (0x724520, match_screens.cpp, cdecl).
///
/// [pt] { 0x724520 } (cdecl, match_screens.cpp, ~4,9 KB): reescreve as linhas do painel dos últimos placares
/// (parâmetros 0 a partida ou nulo, 0xe a entrada do dia, 0xf quantas linhas, 0x10 o primeiro item, 6
/// itens por linha, 0x11 a página mostrada, 0x12 parado). Limpa as linhas, depois reparte as partidas
/// da entrada ([+4] a [+8], 105 bytes em [[0xb67a50]+0x19]) em páginas (0x13 = quantas): por partida o
/// cabeçalho da fase (0x4bffc0, quando muda), a linha do jogo (times, placar contado pelos eventos,
/// período 0x6b6570) e os autores dos eventos (gol, pênalti, contra, pênalti perdido, expulsão, lesão;
/// minutos do mesmo jogador juntos) e o agregado. Só a página mostrada é escrita (segunda passada).
/// No rodapé "<< Página x/y >>" (ou "Hold") com mais de uma página; no fim a hora no parâmetro 0x14.
pub(crate) unsafe extern "C" fn refresh_latest_scores_rows() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    const BUF: usize = 0xde1f64;
    let sm = SCREEN_MANAGER;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let i32_em = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let i16_em = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let b = |a: usize| *(a as *const u8);

    let p0 = get_screen_param(0);
    let entry = get_screen_param(0xe);
    let n = get_screen_param(0xf) as u8 as i32;
    let first = get_screen_param(0x10) as u32;
    let pagina = get_screen_param(0x11) as u8 as i8;
    let parado = get_screen_param(0x12);
    let reference = if p0 != 0 { p0 } else { u(*(G_MATCH_DAY as *const usize) + 0x34) };
    let item = |row: i32, desvio: u32| first.wrapping_add((row as u32).wrapping_mul(6)).wrapping_add(desvio);

    for r in 0..n {
        let it = item(r, 0);
        exe::screenmanager_set_widget_rect(sm, it, (0) as i32, (0) as i32, 0, (0) as i32);
        exe::screenmanager_set_widget_style(sm, it, 1);
        exe::screenmanager_set_widget_font(sm, it, (3) as usize);
        exe::screenmanager_set_widget_text(sm, it, 0xad9c64);
        exe::screenmanager_set_widget_id(sm, it, 0);
        exe::screenmanager_set_widget_text(sm, it + 1, 0xad9c64);
        exe::screenmanager_set_widget_text(sm, it + 2, 0xad9c64);
        exe::screenmanager_set_widget_text(sm, it + 3, 0xad9c64);
        exe::screenmanager_set_widget_font(sm, it + 4, (3) as usize);
        exe::screenmanager_set_widget_text(sm, it + 4, 0xad9c64);
        exe::screenmanager_set_widget_id(sm, it + 4, 0);
        exe::screenmanager_set_widget_text(sm, it + 5, 0xad9c64);
    }
    set_screen_param(0x13, 0);
    let mut paginas: i8 = 0;
    let mut k = i32_em(entry + 4);
    let end = i32_em(entry + 8);
    // the high 16 bits of eax in the footer index (`movzx ax` over the match index)
    let mut high = k as u32 & 0xffff0000;
    if k <= end {
        let mut nova_pagina = true;
        let mut previous = 0usize;
        let (mut row, mut folga) = (0i32, 0i32);
        let abandonada = |partida: usize| {
            let e = read_usize(partida + live_match::ENGINE);
            e != 0 && exe::cm3_match_engine_is_match_abandoned(e) != 0
        };
        loop {
            let deslocamento = k.wrapping_mul(105) as usize;
            let partida = u(*(G_MATCH_DAY as *const usize) + 0x19).wrapping_add(deslocamento);
            let m4 = Fixture(partida + live_match::FIXTURE);
            let (ebp, extras) = 'refaz: loop {
                if nova_pagina {
                    paginas = paginas.wrapping_add(1);
                    previous = 0;
                    row = 0;
                    folga = 0;
                    set_screen_param(0x13, paginas as i32 as usize);
                    nova_pagina = false;
                }
                let (mut g1, mut g2): (u8, u8) = (0, 0);
                if abandonada(partida) {
                    g1 = 0xfd;
                    g2 = 0xfd;
                }
                let passes = if paginas == pagina { 2 } else { 1 };
                let mut result = (0i32, 0i32);
                for passe in 1..=passes {
                    let mut ebp = folga + row;
                    if passe != 2 && ebp > n {
                        nova_pagina = true;
                        continue 'refaz;
                    }
                    let (w32, w30) = (m4.round_prefix(), m4.round_name());
                    let same = previous != 0 && w32 == read_i16(previous + fixture::ROUND_PREFIX) && w30 == read_i16(previous + fixture::ROUND_NAME);
                    if !same && !(w32 == 0 && w30 == 0) {
                        if passe == 2 {
                            exe::comp_util_format_stage_round_name(w32 as i32, w30 as i32, BUF, 0x7d0, 0);
                            let mut x = exe::font_text_width(3, (BUF) as *const u8) + 0x7d;
                            if x >= 0x2f1 {
                                x = 0x2f1;
                            }
                            let comp = m4.competition();
                            let it = item(ebp, 0);
                            if exe::friendly_is_own_comp_record(*(G_FRIENDLY as *const usize), comp) == 0 && exe::comp_is_reserve_or_minor_comp(*(G_RESERVE_COMP as *const usize), comp) == 0 {
                                exe::screenmanager_set_widget_rect(sm, it, (0x7d) as i32, (0) as i32, x, (0) as i32);
                                exe::screenmanager_set_widget_text_colour(sm, it, colour_word_u32(G_GUI_COLOUR_YELLOW));
                                exe::screenmanager_set_widget_text(sm, it, BUF);
                                exe::screenmanager_set_widget_id(sm, it, 0x7e5);
                                let key = ((*(comp as *const u32) << 11 | (w32 as u16 as u32 & 0x7ff)) << 11) | (w30 as u16 as u32 & 0x7ff);
                                exe::screenmanager_set_widget_data(sm, it, key as usize);
                            } else {
                                exe::screenmanager_set_widget_rect(sm, it, (0x7d) as i32, (0) as i32, x, (0) as i32);
                                exe::screenmanager_set_widget_text_colour(sm, it, colour_word_u32(G_GUI_COLOUR_YELLOW));
                                exe::screenmanager_set_widget_text(sm, it, BUF);
                            }
                            ebp += 1;
                            let it = item(ebp, 0);
                            exe::screenmanager_set_widget_rect(sm, it, (0x7d) as i32, (0) as i32, x, (0) as i32);
                            exe::screenmanager_set_widget_style(sm, it, 0xc400);
                            exe::screenmanager_set_widget_colour(sm, it, colour_word_u32(G_GUI_COLOUR_YELLOW));
                            ebp += 1;
                        } else {
                            if ebp + 2 > n {
                                nova_pagina = true;
                                continue 'refaz;
                            }
                            ebp += 2;
                        }
                    }
                    if passe == 2 {
                        // the match row
                        let it = item(ebp, 0);
                        for (side, desvio) in [(0x1cusize, 0u32), (0x20, 4)] {
                            let team = Club(u(m4.addr() + side));
                            format(BUF, 0x987ff0, team.addr() + club::SHORT_NAME);
                            let h = exe::human_manager_is_human_club(team, 0, 1, 0, 0);
                            exe::screenmanager_set_widget_text_colour(sm, it + desvio, if h != 0 { colour_word_u32(G_GUI_COLOUR_YELLOW) } else { colour_word_u32(0xaebde8) });
                            exe::screenmanager_set_widget_text(sm, it + desvio, BUF);
                            exe::screenmanager_set_widget_id(sm, it + desvio, if b(m4.addr() + fixture::HOME_SIDE_MARK + desvio as usize / 4) != 0 { 0x7d5 } else { 0x7d6 });
                            exe::screenmanager_set_widget_data(sm, it + desvio, team);
                            if desvio == 0 {
                                match g1 {
                                    0xfe => text(BUF, 0x99c9e4, 0xad9c64),
                                    0xfd => text(BUF, 0x99c9c4, 0xad9c64),
                                    _ => format(BUF, 0xa16d1c, g1 as i8 as i32),
                                };
                                exe::screenmanager_set_widget_text(sm, it + 1, BUF);
                                exe::screenmanager_set_widget_text(sm, it + 2, 0x989adc);
                                // the original decides the away score from the home score (0xfe/0xfd)
                                match g1 {
                                    0xfe => text(BUF, 0x99c9a4, 0xad9c64),
                                    0xfd => text(BUF, 0x99c984, 0xad9c64),
                                    _ => format(BUF, 0xa16d14, g2 as i8 as i32),
                                };
                                exe::screenmanager_set_widget_text(sm, it + 3, BUF);
                            }
                        }
                        // the period
                        let rotulo = (|| -> usize {
                            if read_u8(reference + live_match::STATE) == 0xff {
                                return 0x98ba0c;
                            }
                            let side_p = read_u8(partida + live_match::STATE);
                            if side_p == 0xff {
                                return 0x98ba0c;
                            }
                            let er = read_usize(reference + live_match::RECORD);
                            if er == 0 {
                                return 0x98ba0c;
                            }
                            let ep = read_usize(partida + live_match::RECORD);
                            if ep == 0 {
                                return 0x98ba0c;
                            }
                            let x = exe::match_day_get_period_rank(*((er + match_record::PERIOD_END) as *const u16) as u32, read_u8(reference + live_match::STATE) as u32) as i8;
                            let y = exe::match_day_get_period_rank(*((ep + match_record::PERIOD_END) as *const u16) as u32, side_p as u32) as i8;
                            let e = if x < y { reference } else { partida };
                            let cl = b(e + 0x67);
                            let w = || *((u(e + 0x5f) + 0x4f) as *const u16);
                            if cl == 1 && w() == 0x1ef {
                                0xa16d10
                            } else if cl == 4 && w() == 0x3de {
                                0xa16d0c
                            } else if cl == 1 && w() == 0x3de {
                                0xa16d08
                            } else if cl == 2 && w() == 0 {
                                0xa16d04
                            } else if cl == 1 && w() == 0x483 {
                                0xa16cfc
                            } else if cl == 4 && w() == 0x528 {
                                0xa16cf4
                            } else if cl == 1 && w() == 0x528 {
                                0xa16cec
                            } else if cl == 2 && w() == 1 {
                                0xa16ce4
                            } else {
                                0xa16ce0
                            }
                        })();
                        text(BUF, rotulo);
                        exe::screenmanager_set_widget_text(sm, it + 5, BUF);
                    } else if ebp + 2 > n {
                        nova_pagina = true;
                        continue 'refaz;
                    }

                    // the events (up to 50, 4 bytes at [match + 0x5f] + 0x57: side, player, type, minute)
                    let mut usados = [0u32; 0x32];
                    let ev = u(u(*(G_MATCH_DAY as *const usize) + 0x19).wrapping_add(deslocamento) + 0x5f);
                    ebp += 1;
                    let (mut casa, mut fora) = (ebp, ebp);
                    let mut extras = 0i32;
                    let (mut r24, mut cor4c, mut coluna44) = (0i32, 0u32, 0u32);
                    let (mut n98, mut na0, mut na4) = (0u32, 0u32, 0u32);
                    let valido = |minute: u8| {
                        if read_u8(reference + live_match::STATE) == 0xff {
                            return false;
                        }
                        let e = read_usize(reference + live_match::RECORD);
                        e != 0 && (minute as i16) <= i16_em(e + 0x4d)
                    };
                    let record = |side: u8, player: u8| {
                        ev.wrapping_add((side as i8 as i32 * 1573) as usize).wrapping_add((player as i8 as i32 * 73) as usize)
                    };
                    let mut j = 0usize;
                    while ev != 0 && j < 0x32 {
                        let e = ev + match_record::AWAY_CLUB_ID * j;
                        let side = b(e + 0x57);
                        if side == 0xff {
                            break;
                        }
                        if usados[j] != 0 || !valido(b(e + 0x5a)) {
                            j += 1;
                            continue;
                        }
                        let minute = b(e + 0x5a) as u32;
                        let player = b(e + 0x58);
                        if passe == 2 {
                            let reg = record(side, player);
                            n98 = *((reg + match_record::ROW_FIRST_NAME_ID) as *const u32);
                            na4 = *((reg + match_record::ROW_COMMON_NAME_ID) as *const u32);
                            na0 = *((reg + match_record::ROW_SECOND_NAME_ID) as *const u32);
                        }
                        let kind = b(e + 0x59);
                        if kind == 7 || kind == 8 {
                            j += 1;
                            continue;
                        }
                        if !(1..=6).contains(&kind) {
                            show_error_at(0xa161d4, 0x1b9c);
                            j += 1;
                            continue;
                        }
                        // the side’s cursor (the own goal goes to the other side)
                        let of_home = (side == 0) != (kind == 2);
                        let r2c;
                        if of_home {
                            r24 = casa;
                            casa += 1;
                            r2c = casa;
                        } else {
                            r24 = fora;
                            fora += 1;
                            r2c = fora;
                        }
                        if passe == 2 {
                            coluna44 = if of_home { 0 } else { 2 };
                            let mut name = [0u8; 0xc8];
                            exe::match_events_format_player_name(ev, player as i8 as i32, side as i8 as i32, name.as_mut_ptr(), 0xc8, 0, 0, 2);
                            let (template, c) = match kind {
                                1 => (0xa16be8, colour_word_u32(G_GUI_COLOUR_PALE_GREY)),
                                3 => (0xa16c0c, colour_word_u32(G_GUI_COLOUR_PALE_GREY)),
                                2 => (0xa16cb8, colour_word_u32(0xaebdd4)),
                                4 => (0xa16c8c, 0),
                                5 => (0xa16c60, colour_word_u32(G_GUI_COLOUR_RED)),
                                _ => (0xa16c34, 0),
                            };
                            text(BUF, template, name.as_ptr(), minute + 1);
                            cor4c = if kind == 4 || kind == 6 { exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) } else { c };
                        } else if kind <= 3 {
                            if abandonada(partida) {
                                g1 = 0xfd;
                                g2 = 0xfd;
                            } else if of_home {
                                g1 = g1.wrapping_add(1);
                            } else {
                                g2 = g2.wrapping_add(1);
                            }
                        }
                        // the other events of the same player and the same type (goal and penalty together)
                        let reg = record(side, player);
                        let id = read_i32(reg + match_record::ROW_STAFF_ID);
                        let goal = |t: u8| t == 1 || t == 3;
                        for m in (j + 1)..0x32 {
                            let em = ev + match_record::AWAY_CLUB_ID * m;
                            let side_m = read_u8(em + match_record::EVENTS);
                            if side_m == 0xff {
                                break;
                            }
                            if usados[m] != 0 || !valido(read_u8(em + match_record::EVENT_MINUTE)) {
                                continue;
                            }
                            let minute_m = read_u8(em + match_record::EVENT_MINUTE) as u32;
                            if i32_em(record(side_m, read_u8(em + match_record::EVENT_PLAYER)) + 0x190) != id {
                                continue;
                            }
                            let tm = read_u8(em + match_record::EVENT_KIND);
                            if tm != kind && !(goal(tm) && goal(kind)) {
                                continue;
                            }
                            if passe == 2 {
                                let mut more = [0u8; 0x100];
                                if tm == 3 {
                                    text(more.as_mut_ptr() as usize, 0xa16bd4, minute_m + 1);
                                } else {
                                    format(more.as_mut_ptr() as usize, 0x990b40, minute_m + 1);
                                }
                                append_to_text_buffer(more.as_ptr());
                            } else if abandonada(partida) {
                                g1 = 0xfd;
                                g2 = 0xfd;
                            } else if goal(tm) {
                                if side_m == 0 { g1 = g1.wrapping_add(1) } else { g2 = g2.wrapping_add(1) }
                            } else if tm == 2 {
                                if side_m != 0 { g1 = g1.wrapping_add(1) } else { g2 = g2.wrapping_add(1) }
                            }
                            usados[m] = 1;
                        }
                        if passe == 2 {
                            let it = item(r24, coluna44 * 2);
                            exe::screenmanager_set_widget_text_colour(sm, it, cor4c);
                            exe::screenmanager_set_widget_text(sm, it, BUF);
                            if id >= 0 && id < *(G_STAFF_COUNT as *const i32) {
                                let person = (*(G_STAFF_TABLE as *const usize)).wrapping_add(id as usize * 110);
                                if exe::database_staff_matches_names(person, n98, na0, na4, reg + match_record::ROW_DATE_OF_BIRTH) != 0 {
                                    let v = if side == 0 { i32_em(ev) } else { read_i32(ev + match_record::AWAY_CLUB_ID) };
                                    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
                                    exe::screenmanager_set_widget_id(sm, it, if v >= limit { 0x7dc } else { 0x7d7 });
                                    exe::screenmanager_set_widget_data(sm, it, person);
                                }
                            }
                        } else if r2c > n {
                            nova_pagina = true;
                            continue 'refaz;
                        }
                        usados[j] = 1;
                        if (side == 0 && casa > fora) || (side == 1 && fora > casa) {
                            extras += 1;
                        }
                        j += 1;
                    }

                    // o agregado (ida e volta)
                    if (m4.home_aggregate() as u8) as i8 > -1 {
                        let (r, do_fora, prox);
                        if casa <= fora {
                            r = casa;
                            casa += 1;
                            do_fora = false;
                            prox = casa;
                        } else {
                            r = fora;
                            fora += 1;
                            do_fora = true;
                            prox = fora;
                        }
                        if passe == 2 {
                            let first_leg = Fixture(u(partida));
                            text(BUF, 0xa16bb0, (first_leg.home_aggregate() as u8) as i8 as i32 + g1 as i8 as i32, (first_leg.away_aggregate() as u8) as i8 as i32 + g2 as i8 as i32);
                            let it = item(r, if do_fora { 4 } else { 0 });
                            exe::screenmanager_set_widget_font(sm, it, (2) as usize);
                            exe::screenmanager_set_widget_text_colour(sm, it, colour_word_u32(G_GUI_COLOUR_YELLOW));
                            exe::screenmanager_set_widget_text(sm, it, BUF);
                        } else if prox > n {
                            nova_pagina = true;
                            continue 'refaz;
                        }
                        if (!do_fora && casa > fora) || (do_fora && fora > casa) {
                            extras += 1;
                        }
                    }
                    result = (casa.max(fora), extras);
                }
                break result;
            };
            row = ebp;
            folga = (3 - extras).max(1);
            previous = m4.addr();
            k = k.wrapping_add(1);
            high = k as u32 & 0xffff0000;
            if k > end {
                break;
            }
        }
    }

    let it = (first as usize).wrapping_add(((high | n as u32) as usize).wrapping_mul(6)) as u32;
    if paginas > 1 {
        exe::screenmanager_set_widget_style(sm, it, 0x30);
        if *(G_FONT_MODE as *const i32) != -1 {
            format(BUF, 0x99ec1c, 0x2a);
            exe::screenmanager_set_widget_text(sm, it, BUF);
        } else {
            exe::screenmanager_set_widget_text(sm, it, 0x98e7c4);
        }
        let template = if parado != 0 { 0xa16b8c } else { 0xa16b68 };
        text(BUF, template, pagina as i32, paginas as i32);
        exe::screenmanager_set_widget_style(sm, it + 1, 2);
        exe::screenmanager_set_widget_text(sm, it + 1, BUF);
        exe::screenmanager_set_widget_style(sm, it + 2, 0x30);
        if *(G_FONT_MODE as *const i32) != -1 {
            format(BUF, 0x99ec1c, 0x2b);
            exe::screenmanager_set_widget_text(sm, it + 2, BUF);
        } else {
            exe::screenmanager_set_widget_text(sm, it + 2, 0x98e7a8);
        }
    } else {
        for d in 0..3 {
            exe::screenmanager_set_widget_style(sm, it + d, 1);
            exe::screenmanager_set_widget_text(sm, it + d, 0xad9c64);
        }
    }
    set_screen_param(0x14, exe::utils_get_time_ms() as usize);
}

/// Handles the waiting screen of a network game: waits for the other humans of the same fixture, or closes the screens,
/// shows the notice and opens the match screen.
///
/// With no record for the current human (`0x765480`) or no match in it (`+0xe2`): −6. Another human (`0x7ffd80`) not
/// absent (`0x7ffdd0`) with the same match: stores his key in `[0xb77c4a]` and −6. Otherwise it closes the screens
/// (`0x803d80`; blocked: −11), shows "Waiting for other players..." (`0x802f80`) and opens the match screen
/// (`0x718aa0`) in the human's slot: −4.
/// Original: `match_screens_handle_waiting_screen` (0x727fb0, match_screens.cpp, cdecl).
///
/// [pt] { 0x727fb0 } (cdecl, match_screens.cpp): a resposta da tela de espera do jogo em rede. Sem ficha do
/// humano da vez (0x765480) ou sem partida nela (+0xe2): −6. Outro humano (0x7ffd80) não ausente
/// (0x7ffdd0) com a mesma partida: guarda a chave dele em [0xb77c4a] e −6. Senão fecha as telas
/// (0x803d80; bloqueado: −11), mostra "Waiting for other players..." (0x802f80) e abre a tela da
/// partida (0x718aa0) na vaga do humano: −4.
pub(crate) unsafe extern "C" fn match_screens_handle_waiting_screen() -> i32 {
    crate::comparison::branch("screens wait for other players");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = SCREEN_MANAGER;
    let h = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16);
    let key_current = || core::ptr::read_unaligned((0xb74c26 + h() as isize * 0x300) as *const usize);
    if exe::get_human_manager_record(0) == 0 {
        return -6;
    }
    let partida = core::ptr::read_unaligned((exe::get_human_manager_record(0) + 0xe2) as *const usize);
    if partida == 0 {
        return -6;
    }
    let mut keys = [0usize; 16];
    let mut how_many: i16 = 0;
    exe::screenmanager_get_channel_manager_keys(sm, keys.as_mut_ptr(), &mut how_many, h() as i32);
    let mut k = 0usize;
    while (k as i32) < how_many as i32 {
        let c = keys[k];
        if c != key_current() && exe::screenmanager_is_manager_away(sm, c) == 0 && core::ptr::read_unaligned((exe::get_human_manager_record(c) + 0xe2) as *const usize) == partida {
            *(G_MATCH_WAITING_MANAGER as *mut usize) = c;
            return -6;
        }
        k += 1;
    }
    if exe::screenmanager_close_all_screens_803d80(sm) == 0 {
        return -11;
    }
    text(0xde1f64, 0xa16d94);
    exe::screenmanager_send_network_status_message(sm, 0xde1f64, 1, 0, h() as i32);
    exe::open_match_screen(key_current(), partida);
    -4
}

// simplification: the wait closes the screens, shows the message and opens the match screen (network); does not run twice
// twice: in comparison mode original only (the copy is proven in Rust mode by the branch)

pub(crate) type DisplayLineFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type ManagerBoxFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type ManagerTextFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// Colour word at a combined with the high 16 bits of x (reproduces registers pushed with known upper halves).
///
/// [pt] A cor (u16) em `a` com os 16 bits de cima de `x`: onde a original troca só o `ax`/`cx`/`dx` de um
/// registrador que tem um valor conhecido (a devolução de uma chamada, uma conta) e o empurra inteiro.
pub(crate) unsafe fn colour_with_high_bits(x: usize, a: usize) -> usize {
    (x & 0xffff_0000) | colour_word(a)
}

/// "Nothing to show" message of match panels: text area sized to the lines that fit in font 3.
///
/// [pt] A mensagem de "não há" dos painéis da partida (0x5ed070 com as linhas que cabem na fonte 3).
pub(crate) unsafe fn show_panel_empty_message(top: *mut i32, base: *mut i32, text_id: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let a = exe::font_get_height(3);
    let n = ((*base - *top - 0x10) / a) as u8;
    text(0xde1f64, text_id);
    mensagem(0x6e, *top as usize, 0x30c, *base as usize, 2, 0, n as usize, 3, colour_word(G_GUI_COLOUR_PALE_GREY), 0xde1f64, 0);
}

/// Draws the action zones from `*topo` to `*base`: the period selector, the pitch split into three zones with both
/// clubs' colour bars, or the no-zones message.
///
/// Outside the match (param 0 null) and with no data (`[param 0x20 + 0x4c]` ≠ 0): the message "There are no action
/// zones available.". Otherwise: the "View" box with the list of the five periods (the first one's item in param
/// `0x23`; the text of the period of param `0xa` is stored), `*topo` += `0x19`, the pitch (`0x8ac600`, which returns
/// the rectangle), the two lines that split it into three, the period title, the three zones (button of param `0xb` on
/// the first), in each zone the panel with the colour bars of the two clubs (`0x53b640`) and the middle line, the
/// bottom panel the same, the rectangle in params `0x16` to `0x19`, `*base` in `0xc` and the values (`0x720320`).
/// Original: `draw_action_zones_panel` (0x71f500, match_screens.cpp, cdecl).
///
/// [pt] { 0x71f500 } (cdecl, match_screens.cpp): desenha as zonas de ação de `*topo` a `*base`. Fora da
/// partida (parâmetro 0 nulo) e sem dados ([parâmetro 0x20 + 0x4c] ≠ 0): a mensagem "There are no
/// action zones available.". Senão: a caixa "View" com a lista dos cinco períodos (o item do primeiro no
/// parâmetro 0x23; o texto do período do parâmetro 0xa fica guardado), `*topo` += 0x19, o campo
/// (0x8ac600, que devolve o retângulo), as duas linhas que o dividem em três, o título do período, as
/// três zonas (botão do parâmetro 0xb na primeira), em cada zona o painel com as barras das cores dos
/// dois clubes (0x53b640) e a linha do meio, o painel de baixo igual, o retângulo nos parâmetros 0x16 a
/// 0x19, `*base` no 0xc e os valores (0x720320).
pub(crate) unsafe extern "C" fn draw_action_zones_panel(top: *mut i32, base: *mut i32) {
    crate::comparison::branch("screens action zones panel");
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let txt: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let guarda = G_ACTION_ZONES_STATIC_GUARD as *mut u8;
    if *guarda & 1 == 0 {
        *guarda |= 1;
        *(G_ACTION_ZONES_STATIC_COLOUR as *mut u16) = *(G_GUI_COLOUR_YELLOW as *const u16);
    }
    let p0 = get_screen_param(0);
    let vista = get_screen_param(0xa) as u8;
    let data = if p0 != 0 {
        core::ptr::read_unaligned((p0 + live_match::RECORD) as *const usize)
    } else {
        let d = get_screen_param(0x20);
        if *((d + match_record::NO_STATISTICS) as *const u8) != 0 {
            return show_panel_empty_message(top, base, 0xa16880);
        }
        d
    };

    text(BUF, 0x988004);
    let t = *top as usize;
    let c = caixa(sm, 0x6e, t, 0xeb, t + 0x14, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0, 0);
    let periodos = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(0xae31aa), c);
    // simplification: with the view outside 0 to 4 the original passes the text of a local variable with no value; here empty
    let mut title = [0u8; 200];
    for (k, &(rotulo, item)) in [(0xa16738usize, 4u8), (0xa167ec, 0), (0xa167e0, 1), (0xa168a8, 2), (0x98800c, 3)].iter().enumerate() {
        text(BUF, rotulo);
        let r = row(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x90, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), 0x21, 1, colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 0, item as usize, periodos);
        if k == 0 {
            set_screen_param(0x23, r as u16 as i16 as i32 as usize);
        }
        if vista == item {
            for i in 0..200 {
                let b = *((BUF + i) as *const u8);
                title[i] = b;
                if b == 0 {
                    break;
                }
            }
        }
    }

    *top += 0x19;
    list(current_human_display(), 0x6e, *top as usize, 0x30c, *base as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    let (mut x0, mut y0, mut x1, mut y1) = (0i32, 0i32, 0i32, 0i32);
    let field: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) =
        core::mem::transmute(FN_DRAW_PITCH);
    field(0xb9, *top + 10, 0x2c1, *base - 0x37, &mut x0, &mut y0, &mut x1, &mut y1, 1, -1, 0x4080_0000);
    let w = x1 - x0;
    let third = w / 3;
    let two = w.wrapping_mul(2) / 3;
    let u = |v: i32| v as usize;

    row(current_human_display(), 1, u(x0 + third), u(y0), u(x0 + third), u(y1), 0, 0, 0x2400, colour_with_high_bits(u(w), G_ACTION_ZONES_STATIC_COLOUR), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    row(current_human_display(), 1, u(x0 + two), u(y0), u(x0 + two), u(y1), 0, 0, 0x2400, colour_with_high_bits(u(w.wrapping_mul(2)), G_ACTION_ZONES_STATIC_COLOUR), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    txt(sm, u(x0 + third + 10), u(y0 + 5), u(x0 + two - 10), u(y0 + 0x19), 2, 0, 0xc, 2, colour_word(G_GUI_COLOUR_PALE_GREY), title.as_ptr() as usize, 1);
    let r = txt(sm, u(x0), u(y0), u(x0 + third), u(y1), 1, 0, 0xc, 4, colour_word(G_GUI_COLOUR_YELLOW), 0, 1);
    set_screen_param(0xb, r as u16 as i16 as i32 as usize);
    txt(sm, u(x0 + third), u(y0), u(x0 + two), u(y1), 1, 0, 0xc, 4, colour_word(G_GUI_COLOUR_YELLOW), 0, 1);
    txt(sm, u(x0 + two), u(y0), u(x1), u(y1), 1, 0, 0xc, 4, colour_word(G_GUI_COLOUR_YELLOW), 0, 1);
    txt(sm, u(x0 + two + 10), u(y0 + 5), u(x1 - 10), u(y0 + 0x19), 1, 0, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), 0, 1);

    // the two clubs’ colors (0x53b640 writes u16; the rest of the original’s variable is stack garbage, here 0)
    let (mut c1a, mut c1b, mut c2a, mut c2b) = (0u32, 0u32, 0u32, 0u32);
    let clubs = *(G_CLUB_TABLE as *const usize);
    let id1 = core::ptr::read_unaligned(data as *const i32);
    exe::database_get_club_kit_colours((clubs as isize + id1 as isize * 0x245) as usize, *((data + match_record::HOME_KIT) as *const u8) as u32, &mut c1a, &mut c1b);
    let id2 = core::ptr::read_unaligned((data + match_record::AWAY_CLUB_ID) as *const i32);
    exe::database_get_club_kit_colours((clubs as isize + id2 as isize * 0x245) as usize, *((data + match_record::AWAY_KIT) as *const u8) as u32, &mut c2a, &mut c2b);

    // a zone: the frame, the panel from `l` to `r`, the bars of the two colors and the middle line; returns that of the line
    let zona = |a: (i32, i32), colour: usize, l: i32, r: i32, colour_b: (usize, usize)| -> usize {
        txt(sm, u(a.0), u(y1 - 0x19), u(a.1), u(y1 - 5), 0x30, colour, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0, 1);
        txt(sm, u(l), u(y1 - 0x14), u(r), u(y1 - 10), 0x60, colour_b.0, 0xc, 1, colour_b.1, 0, 1);
        let d = r - l;
        let middle = (d * 25 - 100) * 2 / 100 + l + 2;
        txt(sm, u(l + 2), u(y1 - 0x12), u(middle), u(y1 - 12), 0x10, c1b as usize, 0xc, 1, c1a as usize, 0, 1);
        txt(sm, u(middle), u(y1 - 0x12), u(r - 2), u(y1 - 12), 0x10, c2b as usize, 0xc, 1, c2a as usize, 0, 1);
        let xm = l + d / 2;
        let sign = if d < 0 { 0xffff_0000 } else { 0 };
        row(current_human_display(), 1, u(xm), u(y1 - 0x12), u(xm), u(y1 - 0x10), 0, 0, 0x400, sign | colour_word(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX)
    };
    let r1 = zona((x0 + 5, x0 + third - 5), colour_word(G_GUI_COLOUR_GREY), x0 + 10, x0 + third - 10, (colour_with_high_bits(u(y1), G_GUI_COLOUR_GREY), colour_word(G_GUI_COLOUR_PALE_GREY)));
    let r2 = zona((x0 + third + 5, x0 + two - 5), colour_with_high_bits(r1, G_GUI_COLOUR_GREY), x0 + third + 10, x0 + two - 10, (colour_with_high_bits(u(third), G_GUI_COLOUR_GREY), colour_with_high_bits(u(w), G_GUI_COLOUR_PALE_GREY)));
    zona((x0 + two + 5, x1 - 5), colour_with_high_bits(r2, G_GUI_COLOUR_GREY), x0 + two + 10, x1 - 10, (colour_word(G_GUI_COLOUR_GREY), colour_with_high_bits(u(two), G_GUI_COLOUR_PALE_GREY)));

    let b = *base;
    let r = txt(sm, 0xb9, u(b - 0x2d), 0x2c1, u(b - 10), 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0, 1);
    txt(sm, 0xd2, u(b - 0x25), 0x2a8, u(b - 0x12), 0x60, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY), 0, 1);
    txt(sm, 0xd4, u(b - 0x23), 0x1bd, u(b - 0x14), 0x10, c1b as usize, 0xc, 1, c1a as usize, 0, 1);
    let r = txt(sm, 0x1bd, u(b - 0x23), 0x2a6, u(b - 0x14), 0x10, c2b as usize, 0xc, 1, c2a as usize, 0, 1);
    row(current_human_display(), 1, 0x1bd, u(b - 0x23), 0x1bd, u(b - 0x20), 0, 0, 0x400, colour_with_high_bits(r, G_GUI_COLOUR_BLACK), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    set_screen_param(0x16, u(x0));
    set_screen_param(0x17, u(y0));
    set_screen_param(0x18, u(x1));
    set_screen_param(0x19, u(y1));
    set_screen_param(0xc, u(*base));
    exe::refresh_action_zones_values();
}

/// Draws the match report from `*topo` to `*base`: the minutes and the event texts (building the report outside the
/// match), or the no-report message.
///
/// A 2-column table (1, 5) with one row per text of each event in the report (`0x6d8d10`, " %s", colour by event type)
/// and the minute (`0x66f4e0` or, in language 2, `0x90c310`) on the first row of each event with a new minute; padded
/// with empty rows up to 20.
///
/// The report: in the match, `[[p0 + 0x57] + 0x8f7a]`; outside it, the one in param `0x2d`, which is created here
/// (`0x944cf1` of `0x2db1` bytes, `0x6d5bf0`, built from data + `0xd69` in `0x6d9c40`; if it cannot be built, it is
/// destroyed and the message is shown). The events with code `[dados + 0x4f]` `0x1ef`, `0x3de` (minute ≥ 45), `0x483`
/// (≥ 90) and `0x528` (≥ 105) are left out. With no data (`[param 0x20 + 0x4c]`) or no rows: "There is no match report
/// available.". Null report or out of memory: the original routine runs (error box).
/// Original: `draw_match_report_panel` (0x721000, match_screens.cpp, cdecl).
///
/// [pt] { 0x721000 } (cdecl, match_screens.cpp): desenha o relatório da partida de `*topo` a `*base`: uma
/// tabela de 2 colunas (1, 5) com uma linha por texto de cada lance do relatório (0x6d8d10, " %s", cor
/// pelo tipo do lance) e o minuto (0x66f4e0 ou, no idioma 2, 0x90c310) na primeira linha de cada lance
/// de minuto novo; completa com linhas vazias até 20. O relatório: na partida, [[p0 + 0x57] + 0x8f7a];
/// fora dela, o do parâmetro 0x2d, que nasce aqui (0x944cf1 de 0x2db1 bytes, 0x6d5bf0, montado pelos
/// dados + 0xd69 em 0x6d9c40; se não monta, é destruído e sai a mensagem). Os lances do código
/// [dados + 0x4f] 0x1ef, 0x3de (minuto ≥ 45), 0x483 (≥ 90) e 0x528 (≥ 105) ficam de fora. Sem dados
/// ([parâmetro 0x20 + 0x4c]) ou sem linhas: "There is no match report available.". Relatório nulo ou
/// sem memória: a original (caixa de erro).
pub(crate) unsafe extern "C" fn draw_match_report_panel(top: *mut i32, base: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_REPORT_PANEL);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    const SEM_RELATORIO: usize = 0xa16900;

    let p0 = get_screen_param(0);
    let (dados, partida, relatorio);
    if p0 == 0 {
        dados = get_screen_param(0x20);
        partida = get_screen_param(0x1f);
        if *((dados + match_record::NO_STATISTICS) as *const u8) != 0 {
            crate::comparison::branch("screens match report panel");
            return show_panel_empty_message(top, base, SEM_RELATORIO);
        }
        if get_screen_param(0x2d) == 0 {
            let memoria = exe::crt_operator_new(0x2db1);
            if memoria == 0 {
                // simplification: out of memory the original shows the error (and tries to allocate again, to no effect)
                return o(top, base);
            }
            crate::comparison::branch("screens match report panel (build)");
            // simplification: the original’s SEH frame only frees the memory if the constructor throws an exception, which it does not
            let r = exe::new_match_events(memoria, (0) as u8);
            set_screen_param(0x2d, r);
            if (exe::match_events_load_event_log(r, dados + match_record::EVENT_LOG) as u32) as u8 == 0 {
                show_panel_empty_message(top, base, SEM_RELATORIO);
                let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
                destroy(r);
                exe::crt_operator_delete(r);
                set_screen_param(0x2d, 0);
                return;
            }
            relatorio = r;
        } else {
            relatorio = get_screen_param(0x2d);
        }
    } else {
        dados = core::ptr::read_unaligned((get_screen_param(0) + 0x5f) as *const usize);
        partida = get_screen_param(0) + 4;
        relatorio = core::ptr::read_unaligned((core::ptr::read_unaligned((get_screen_param(0) + 0x57) as *const usize) + 0x8f7a) as *const usize);
    }
    if relatorio == 0 {
        return o(top, base);
    }
    crate::comparison::branch("screens match report panel");

    let lances = |r: usize| *((r + 6) as *const i16);
    let mut columns: [u8; 2] = [1, 5];
    let mut last_minute: u8 = 0xff;
    let mut rows: i32 = 0;
    let mut table: usize = 0;
    // the row colour: the u16 colours swap only the low 16 bits (the high ones start with garbage in the original)
    let mut colour: usize = 0;
    if lances(relatorio) <= 0 {
        return show_panel_empty_message(top, base, SEM_RELATORIO);
    }
    let mut i: i16 = 0;
    loop {
        let entry = if i >= 0 && i < lances(relatorio) { relatorio + match_events_fields::ENTRIES + i as usize * 14 } else { 0 };
        let code = core::ptr::read_unaligned((dados + match_record::PERIOD_END) as *const u16);
        let skip = code == 0x1ef
            || (code == 0x3de && *((entry + 4) as *const u8) >= 0x2d)
            || (code == 0x483 && last_minute >= 0x5a)
            || (code == 0x528 && last_minute >= 0x69);
        if !skip {
            let mut first: i32 = -1;
            let mut j: u32 = 0;
            loop {
                let id = if partida != 0 { *(partida as *const u32) } else { u32::MAX };
                let t = exe::match_events_build_report_text(relatorio, entry, dados, id, j);
                j += 1;
                if t == 0 {
                    break;
                }
                if rows == 0 {
                    let (b, a) = (*base, *top);
                    let n = ((b - a) / exe::font_get_height(2)) as u8;
                    // simplification: the top 3 bytes of the rows are stack garbage in the original; here 0
                    table = list(current_human_display(), 0x6e, a as usize, 0x30c, b as usize, 2, columns.as_mut_ptr() as usize, n as usize, 0, 1, 0, usize::MAX);
                }
                let k = *(entry as *const i16) as i32;
                let word = |a: usize| (colour & 0xffff_0000) | colour_word(a);
                colour = match k {
                    0x1f56 | 0x1f57 | 0x1fce | 0x2008 => (exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as usize),
                    0x1fcc | 0x1fcf | 0x2000 | 0x2013..=0x201b | 0x2114 => (exe::graphics_rgb_to_pixel(0, 0x96, 0xff, 0) as usize),
                    0x1fcd => word(0xaebdd4),
                    0x1ff5..=0x1ff8 => word(G_GUI_COLOUR_RED),
                    0x1ff9 => word(G_GUI_COLOUR_YELLOW),
                    _ => word(0xae3154),
                };
                if first == -1 {
                    first = rows;
                }
                format(BUF, 0x98ed14, t);
                text_of_cell(sm, table, 1, rows as usize, 2, 0, 1, 2, colour, BUF);
                rows += 1;
            }
            if first != -1 {
                let minute = *((entry + 4) as *const u8);
                let mut k = first;
                if last_minute != minute {
                    last_minute = minute;
                    if exe::langlib_get_language() != 2 {
                        text(BUF, 0xa168ec, minute as u32 + 1);
                    } else {
                        format(BUF, 0xa168e4, exe::utils_format_ordinal((minute.wrapping_add(1) as u32) as i32, (2) as i32));
                    }
                    text_of_cell(sm, table, 0, k as usize, 2, 0, 0xc, 2, colour_word(G_GUI_COLOUR_YELLOW), BUF);
                    k += 1;
                }
                while k < rows {
                    text_of_cell(sm, table, 0, k as usize, 2, 0, 0xc, 2, colour_word(G_GUI_COLOUR_PALE_GREY), 0xad9c64);
                    k += 1;
                }
            }
        }
        i = i.wrapping_add(1);
        if i >= lances(relatorio) {
            break;
        }
    }
    if rows == 0 {
        return show_panel_empty_message(top, base, SEM_RELATORIO);
    }
    while rows < 0x14 {
        let r = text_of_cell(sm, table, 0, rows as usize, 2, 0, 0xc, 3, colour_word(G_GUI_COLOUR_PALE_GREY), 0xad9c64);
        text_of_cell(sm, table, 1, rows as usize, 2, 0, 1, 2, colour_with_high_bits(r, G_GUI_COLOUR_PALE_GREY), 0xad9c64);
        rows += 1;
    }
}

/// Builds the day fixtures screen of the group in param 1 (`+4` to `+8` the day's matches in `[[0xb67a50]+0x19]`, 105
/// bytes each): title, the day's competition tree, the rounds and matches table and the scroll to the human's match.
///
/// The background and the competition colours (`0x41c410`, `0x53b7c0`), the title with the name (Friendlies/Reserves
/// with more than one match) and the badge (outside friendlies and national teams), the menu bar (`0x75db00` with 4),
/// the title "Fixtures"/"Results" (singular or plural; the day from the date `0x54d040`/`0x54d130`), with more than one
/// round (`[+0x15]` > 1) the tree of the day's competition groups (group header and one row per competition), the
/// "Competition" badge, the 9-column table with the header of each new round and one row per match (`0x49db00`), the
/// back and forward buttons, the scroll to the human's match (param 4) or to the one in param 2 and the current human's
/// seen mark in the group (`+0x14` + 4h).
/// Original: `build_day_fixtures_screen` (0x717db0, match_screens.cpp, cdecl).
///
/// [pt] { 0x717db0 } (cdecl, match_screens.cpp): monta a tela dos jogos do dia do grupo do parâmetro 1 (+4 a
/// +8 as partidas do dia em [[0xb67a50]+0x19], 105 bytes cada): o fundo e as cores da competição
/// (0x41c410, 0x53b7c0), o título com o nome (Friendlies/Reserves com mais de uma partida) e o selo
/// (fora dos amistosos e das seleções), a barra de menu (0x75db00 com 4), o título "Fixtures"/"Results"
/// (singular ou plural; o dia pela data 0x54d040/0x54d130), com mais de uma rodada ([+0x15] > 1) a
/// árvore dos grupos de competições do dia (cabeçalho do grupo e uma linha por competição), o selo
/// "Competition", a tabela de 9 colunas com o cabeçalho de cada rodada nova e uma linha por partida
/// (0x49db00), os botões de voltar e avançar, a rolagem até a partida do humano (parâmetro 4) ou à do
/// parâmetro 2 e a marca de vista do humano da vez no grupo (+0x14 + 4h).
pub(crate) unsafe extern "C" fn build_day_fixtures_screen() {
    crate::comparison::branch("screens build the day matches");
    let caixa: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let row: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sem_selo = |c: usize| {
        exe::friendly_is_own_comp_record(*(G_FRIENDLY as *const usize), c) == 0 && exe::comp_is_reserve_or_minor_comp(*(G_RESERVE_COMP as *const usize), c) == 0
    };
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rd32 = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let mut columns: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
    let group = get_screen_param(1);
    let comp = (*(G_CLUB_COMP_TABLE as *const usize) as isize + rd32(group) as isize * 107) as usize;
    exe::background_set_comp_picture(comp, 0, 0);
    // the competition’s colors (0x53b7c0 writes u16; the rest of the original’s variable is stack garbage, here 0)
    let (mut ca, mut cb) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut ca, &mut cb);
    let more_of_one = rd32(group + 8) > rd32(group + 4);
    let key = *(comp as *const usize);
    if key == *(KEY_CLUB_COMP_FRIENDLY as *const usize) && more_of_one {
        text(BUF, 0xa1631c);
    } else if key == *(KEY_CLUB_COMP_RESERVE as *const usize) && more_of_one {
        text(BUF, 0xa16310);
    } else {
        copy_to_work_text(comp + 4);
    }
    exe::screenmanager_add_screen_title(sm, cb, ca, BUF);
    if sem_selo(comp) {
        row(current_human_display(), 6, 0x7d, 0xa, 0x316, 0x46, 0, 0, 1, 0, 0, 0xc, 7, 0, 0, 0, 0x7d0, comp, usize::MAX);
    }
    exe::menubar_build(4, 0);

    // the day’s title; the date byte goes whole (the routines only read the byte)
    let md = rd(G_MATCH_DAY);
    let start = rd32(group + 4);
    let jogado = *((rd(read_usize(md + match_day_fields::MATCHES) + start as usize * 105) + 0x43) as *const u8) != 0xff;
    let formato = match (jogado, rd32(group + 8) > start) {
        (false, true) => 0xa162f0usize,
        (false, false) => 0xa162d0,
        (true, true) => 0xa162b0,
        (true, false) => 0xa16290,
    };
    let r1 = exe::date_get_time_of_day_name(*(rd(G_MATCH_DAY) as *const u8) as u32, 1);
    let r2 = (exe::date_get_time_of_day_gender(*(rd(G_MATCH_DAY) as *const u8) as u32) as u32) as u8 as i8 as i32;
    let r3 = (exe::date_get_time_of_day_gender(*(rd(G_MATCH_DAY) as *const u8) as u32) as u32) as u8 as i8 as i32;
    text(BUF, formato, r3, r2, r1);
    exe::screenmanager_add_screen_subtitle(sm, BUF, 1);

    // the top 3 bytes of the table’s row count: the group counter the original left on the stack
    let mut rest: usize = 0;
    if rd32(rd(G_MATCH_DAY) + 0x15) > 1 {
        let cabeca = caixa(sm, 0x69, 0xf, 0x78, 0x23, 0x30, cb as usize, 0xc, 1, ca as usize, 0, 0, 0);
        let raiz = list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), cabeca);
        let mut i: i32 = 0;
        loop {
            let md = rd(G_MATCH_DAY);
            let n = read_i32(md + match_day_fields::GROUP_COUNT);
            if i >= n {
                break;
            }
            let e = read_usize(md + match_day_fields::GROUPS) + i as usize * 0x18;
            let parent = if n == 1 {
                raiz
            } else {
                let kind = *(e as *const i32);
                match kind {
                    0 => {
                        text(BUF, 0xa16284);
                    }
                    1 => {
                        text(BUF, 0xa16248, 0xad9c64);
                    }
                    2 => {
                        text(BUF, 0xa16210, 0xad9c64);
                    }
                    3..=7 => {
                        let mut name = [0u8; 0x64];
                        exe::get_continent_adjective(*((0xb63d64 + kind as isize * 72) as *const usize), name.as_mut_ptr(), 0x64, 6);
                        copy_to_work_text(name.as_ptr() as usize);
                    }
                    _ => copy_to_work_text(*((0xb63d60 + kind as isize * 72) as *const usize) + 4),
                }
                if *((e + 0x14) as *const u32) != 0 {
                    let r = row(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_with_high_bits(raiz, 0xae31aa), colour_word(G_GUI_COLOUR_BLACK), 1, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0, 0, 0, raiz);
                    list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_word(0xae31aa), r)
                } else {
                    let r = row(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_with_high_bits(raiz, 0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), 1, 1, colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 0, 0, raiz);
                    list(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour_with_high_bits(r, 0xae31aa), r)
                }
            };
            let mut k = rd32(e + 4);
            while k <= rd32(e + 8) {
                let m = (rd(rd(G_MATCH_DAY) + 0xd) as isize + k as isize * 0x54) as usize;
                let c = (*(G_CLUB_COMP_TABLE as *const usize) as isize + *(m as *const i32) as isize * 107) as usize;
                panel_competition_name(Competition(c), m);
                let v = *((m + 0x10) as *const u32) as usize;
                if v != 0 {
                    row(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_BLACK), 1, 1, colour_with_high_bits(v, G_GUI_COLOUR_PALE_GREY), BUF, 0, 1, m, parent);
                } else {
                    row(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(0xae31aa), colour_word(G_GUI_COLOUR_YELLOW), 1, 1, colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 1, m, parent);
                }
                k += 1;
            }
            i += 1;
        }
        rest = i as usize;
    }

    let mut esquerda = 0x91usize;
    if sem_selo(comp) {
        let r = text(BUF, 0xa16204) as u32 as usize;
        caixa(sm, 0x28f, 0x91, 0x30c, 0xa5, 0x30, colour_with_high_bits(r, G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0x7d0, comp);
        esquerda = 0xaa;
    }
    let n = ((0x207 - esquerda as i32) / exe::font_get_height(3)) as u8;
    let table = list(current_human_display(), 0x6e, esquerda, 0x30c, 0x217, 9, columns.as_mut_ptr() as usize, (rest & !0xff) | n as usize, 0, 2, 0, usize::MAX);

    let mut k: i32 = 0;
    let mut target: i32 = -1;
    let mut previous: usize = 0;
    let mut j = rd32(group + 4);
    while j <= rd32(group + 8) {
        let partidas = rd(rd(G_MATCH_DAY) + 0x19);
        let fx = Fixture(rd(partidas + j as usize * 105));
        let w30 = fx.round_name();
        let w32 = fx.round_prefix();
        let same_round = previous != 0 && w32 == *((previous + fixture::ROUND_PREFIX) as *const i16) && w30 == *((previous + fixture::ROUND_NAME) as *const i16);
        if !same_round && !(w32 == 0 && w30 == 0) {
            if previous != 0 {
                // the original pushes edx = the matches pointer with the color in dx
                text_of_cell(sm, table, 0, k as usize, 1, 0, 1, 1, colour_with_high_bits(partidas, G_GUI_COLOUR_PALE_GREY), 0);
                k += 1;
            }
            let round: unsafe extern "C" fn(i32, i32, usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_FORMAT_STAGE_ROUND_NAME);
            round(w32 as i32, w30 as i32, BUF, 0x7d0, 0);
            let mut width = exe::font_text_width(3, BUF as *const u8) + 0x73;
            if width >= 0x2f1 {
                width = 0x2f1;
            }
            let width = width as usize;
            if exe::comp_is_reserve_or_minor_comp(*(G_RESERVE_COMP as *const usize), comp) == 0 {
                let base_id = *(fx.competition() as *const u32);
                let id = (((base_id << 11) | (w32 as u32 & 0x7ff)) << 11) | (w30 as u32 & 0x7ff);
                row(current_human_display(), 2, 0x73, 0, width, 0, 0, k as usize, 1, 0, 0, 1, 3, colour_with_high_bits(id as usize, G_GUI_COLOUR_YELLOW), BUF, 0, 0x7e5, id as usize, table);
            } else {
                row(current_human_display(), 1, 0x73, 0, width, 0, 0, k as usize, 1, 0, 0, 1, 3, colour_word(G_GUI_COLOUR_YELLOW), BUF, 0, 0, 0, table);
            }
            row(current_human_display(), 1, 0x73, 0, width, 0, 0, (k + 1) as usize, 0xc400, colour_word(G_GUI_COLOUR_YELLOW), 0, 0, 0, 0, 0, 0, 0, 0, table);
            k += 2;
        }
        let (c1, c2) = (fx.home_club(), fx.away_club());
        if exe::human_manager_current_controls_club(c1, 0, 1) != 0 || exe::human_manager_current_controls_club(c2, 0, 1) != 0 {
            target = k;
        } else if target == -1 && (exe::human_manager_is_human_club(c1, 0, 1, 0, 0) != 0 || exe::human_manager_is_human_club(c2, 0, 1, 0, 0) != 0) {
            target = k;
        }
        exe::comp_screens_add_fixture_row(fx, table, &mut k, 2);
        previous = fx.addr();
        j += 1;
    }

    let advance = (exe::screenmanager_is_forward_unavailable(sm) == 0) as u32;
    let go_back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(go_back, advance);

    let item = table as u16 as i16;
    let rolar = |pos: usize| {
        if item != -1 {
            exe::area_set_scroll_row((current_human_display() as isize + item as isize * 3057 + 4) as usize, pos);
        }
    };
    if get_screen_param(4) != 0 {
        if target > -1 {
            let p = target - (n >> 1) as i32;
            rolar(if p <= 0 { 0 } else { p as usize });
        } else {
            rolar(get_screen_param(2));
        }
        set_screen_param(3, item as i32 as usize);
        set_screen_param(4, 0);
    } else {
        rolar(get_screen_param(2));
        set_screen_param(3, item as i32 as usize);
    }
    *((group as isize + 0x14 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as *mut u32) = 1;
}

/// Saves into `copia` (screen parameter 0x1d) the match state `estado` (0xd7a bytes), as the three values routines do
/// at the end (the copy is `match_team_block_assign` / `match_player_row_assign` inlined).
///
/// Fields of 4, 2 and 1 bytes up to 0x11f, the two team blocks of 0x625 bytes and the tail 0xd69..0xd7a. With no copy
/// yet: allocates it (`exe::crt_operator_new`), constructs the two blocks (`exe::crt_vector_constructor_iterator` with 0x453c30) and writes it into parameter 0x1d.
///
/// [pt] Guarda em `copia` (o parâmetro 0x1d) o estado da partida `estado` (0xd7a bytes), como as três
/// rotinas de valores fazem no fim (a cópia é a 0x71dd70 / 0x71df00 em linha): campos de 4, 2 e 1
/// bytes até 0x11f, os dois blocos de time de 0x625 bytes e o fim 0xd69..0xd7a. Sem cópia ainda:
/// aloca (0x944cf1), constrói os dois blocos (0x9463af com 0x453c30) e grava no parâmetro 0x1d.
pub(crate) unsafe fn save_match_snapshot(mut copy: usize, state: usize) {
    if copy == 0 {
        crate::comparison::branch("screens create the match state copy");
        copy = exe::crt_operator_new(0xd7a);
        if copy != 0 {
            exe::crt_vector_constructor_iterator(copy + match_record::HOME_TEAM, 0x625, 2, 0x453c30, 0x453c10);
        }
        set_screen_param(0x1d, copy);
    }
    copy_fields(copy, state, 0, 8, 4);
    copy_fields(copy, state, 8, 0x34, 2);
    copy_fields(copy, state, 0x34, 0x4c, 4);
    copy_fields(copy, state, 0x4c, 0x4d, 1);
    copy_fields(copy, state, 0x4d, 0x53, 2);
    copy_fields(copy, state, 0x53, 0x11f, 4);
    for t in [0x11f, 0x11f + 0x625] {
        let (d, s) = (copy + t, state + t);
        copy_fields(d, s, 0, 0x11, 1);
        copy_fields(d, s, 0x11, 0x1f, 2);
        copy_fields(d, s, 0x1f, 0x35, 1);
        copy_fields(d, s, 0x35, 0x61, 2);
        copy_fields(d, s, 0x61, 0x69, 4);
        copy_fields(d, s, 0x69, 0x71, 2);
        for j in 0..20 {
            match_player_row_assign(d + match_record_team::ROWS + j * 0x49, s + match_record_team::ROWS + j * 0x49);
        }
    }
    copy_fields(copy, state, 0xd69, 0xd75, 4);
    copy_fields(copy, state, 0xd75, 0xd76, 1);
    copy_fields(copy, state, 0xd76, 0xd7a, 4);
}

/// Match state the value routines display: [param0 + 0x5f] during the match, else screen param 0x20.
///
/// [pt] O estado da partida que as rotinas de valores mostram: na partida (parâmetro 0) [p0 + 0x5f], fora
/// dela o parâmetro 0x20 (lido depois dos outros, como a original).
pub(crate) unsafe fn displayed_match_state(p0: usize) -> usize {
    if p0 != 0 {
        core::ptr::read_unaligned((p0 + live_match::RECORD) as *const usize)
    } else {
        get_screen_param(0x20)
    }
}

pub(crate) const EMPTY_TEXT: usize = 0xad9c64;
pub(crate) const FORMAT_D: usize = 0x98806c; // "%d"
pub(crate) const FORMAT_PCT: usize = 0x98e8d4; // "%d%%"

/// Sends the match stats cells (from the item in param 9) the state values that changed since the match state copy
/// (param `0x1d`; with no copy, all) and updates the copy.
///
/// The bytes of the two teams ("%d"; `+0x127..` and `+0x74c..`), the two differences `+0x127` − `+0x128` and `+0x74c` −
/// `+0x74d` and six percentages (num × 100 / den, "%d%%", or "-" with den 0; the last one compares `+0x75d` but shows
/// 100 − the one of `+0x138`, as the original does). At the end it stores the state in the copy.
/// Original: `refresh_match_stats_values` (0x71e750, match_screens.cpp, cdecl).
///
/// [pt] { 0x71e750 } (cdecl, match_screens.cpp): passa às células das estatísticas da partida (a partir do
/// item do parâmetro 9) os valores do estado que mudaram desde a cópia do parâmetro 0x1d (sem cópia,
/// todos): os bytes dos dois times ("%d"; +0x127.. e +0x74c..), as duas diferenças +0x127 − +0x128 e
/// +0x74c − +0x74d e seis percentuais (num × 100 / den, "%d%%", ou "-" com den 0; o último compara
/// +0x75d mas mostra 100 − o do +0x138, como a original). No fim guarda o estado na cópia.
pub(crate) unsafe extern "C" fn refresh_match_stats_values() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let item = get_screen_param(9) as u32;
    let copy = get_screen_param(0x1d);
    let e = displayed_match_state(p0);
    let byte = |k: usize, d: u32| {
        if copy == 0 || (read_u8(e + k) as i32) != (read_u8(copy + k) as i32) {
            format(BUF, FORMAT_D, (read_u8(e + k) as i32));
            exe::screenmanager_set_widget_text(sm, item.wrapping_add(d), BUF);
        }
    };
    let diferenca = |a: usize, b: usize, d: u32| {
        if copy == 0 || (read_u8(e + a) as i32) - (read_u8(e + b) as i32) != (read_u8(copy + a) as i32) - (read_u8(copy + b) as i32) {
            format(BUF, FORMAT_D, (read_u8(e + a) as i32) - (read_u8(e + b) as i32));
            exe::screenmanager_set_widget_text(sm, item.wrapping_add(d), BUF);
        }
    };
    let write_percent_pair = |muda: usize, den: usize, num: usize, d: u32, inverter: bool| {
        if copy == 0 || (read_i16(e + muda) as i32) != (read_i16(copy + muda) as i32) {
            if (read_i16(e + den) as i32) == 0 {
                *(BUF as *mut [u8; 2]) = *b"-\0";
            } else {
                let x = (read_i16(e + num) as i32) * 100 / (read_i16(e + den) as i32);
                format(BUF, FORMAT_PCT, if inverter { 100 - x } else { x });
            }
            exe::screenmanager_set_widget_text(sm, item.wrapping_add(d), BUF);
        }
    };

    read_i8(0x127 + 0);
    read_i8(0x74c + 2);
    read_i8(0x128 + 3);
    read_i8(0x74d + 5);
    diferenca(0x127, 0x128, 6);
    diferenca(0x74c, 0x74d, 8);
    for (k, d) in [(0x129, 9), (0x74e, 0xb), (0x12a, 0xc), (0x74f, 0xe), (0x12b, 0xf), (0x750, 0x11), (0x12c, 0x12), (0x751, 0x14), (0x12d, 0x15), (0x752, 0x17)] {
        read_i8(k + d);
    }
    write_percent_pair(0x130, 0x130, 0x132, 0x18, false);
    write_percent_pair(0x755, 0x755, 0x757, 0x1a, false);
    write_percent_pair(0x134, 0x134, 0x136, 0x1b, false);
    write_percent_pair(0x759, 0x759, 0x75b, 0x1d, false);
    write_percent_pair(0x138, 0x138, 0x13a, 0x1e, false);
    write_percent_pair(0x75d, 0x138, 0x13a, 0x20, true);
    for (k, d) in [(0x12e, 0x21), (0x753, 0x23), (0x12f, 0x24), (0x754, 0x26)] {
        read_i8(k + d);
    }
    save_match_snapshot(copy, e);
}

/// Updates the action zones cells with the values that changed since the copy in param `0x1d` (with no copy, all):
/// markers, possession, ball-in-play time, the three zone bars and the possession bar in the club colours; updates the
/// state copy.
///
/// The five markers (param `0x23` + 0..4; `0x801350` 1 and `0x801fc0` 4) when the sum of the bytes `+0x767`/`+0x142`,
/// `+0x763`/`+0x13e` .. `+0x766`/`+0x141` is not zero (the first only in the match). By the side of param `0xa` (i8):
/// possession in three (`+0xc`, `+0x16`, `+0x20`, "%d%%" truncated to i8; the third is 100 − the others) in the item of
/// param `0xb`; the ball-in-play time (`+0x2a` × 60 / 11 seconds, empty on side 4); the colours of the two clubs
/// (`0x53b640` with kits `+0x152` and `+0x777`); the three pitch zones (`+0x143`, `+0x148`, `+0x14d`; 50 with no value
/// or without flag `+0x768`/`+0x76d`/`+0x772`) as bars between params `0x16` and `0x18` at the height of `0x19`; and
/// the possession bar (`+0x13e`/`+0x763`) from `0xd4` to `0x2a6` at the height of param `0xc`. At the end it stores the
/// state.
/// Original: `refresh_action_zones_values` (0x720320, match_screens.cpp, cdecl).
///
/// [pt] { 0x720320 } (cdecl, match_screens.cpp): passa às células das zonas de ação os valores que mudaram
/// desde a cópia do parâmetro 0x1d (sem cópia, todos). Os cinco marcadores (parâmetro 0x23 + 0..4;
/// 0x801350 1 e 0x801fc0 4) quando a soma dos bytes +0x767/+0x142, +0x763/+0x13e .. +0x766/+0x141 não é
/// zero (o primeiro só na partida). Pelo lado do parâmetro 0xa (i8): a posse em três (+0xc, +0x16,
/// +0x20, "%d%%" truncado a i8; o terceiro é 100 − os outros) no item do parâmetro 0xb; o tempo de bola
/// em jogo (+0x2a × 60 / 11 segundos, vazio no lado 4); as cores dos dois clubes (0x53b640 com o
/// uniforme +0x152 e +0x777); as três zonas do campo (+0x143, +0x148, +0x14d; 50 sem valor ou sem a
/// marca +0x768/+0x76d/+0x772) em barras entre os parâmetros 0x16 e 0x18 na altura do 0x19; e a barra
/// da posse (+0x13e/+0x763) de 0xd4 a 0x2a6 na altura do parâmetro 0xc. No fim guarda o estado.
pub(crate) unsafe extern "C" fn refresh_action_zones_values() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzido: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let side = get_screen_param(0xa) as u8 as i8 as isize;
    let b = get_screen_param(0xb) as u32;
    let height = get_screen_param(0xc) as i32;
    let esq = get_screen_param(0x16) as i32;
    let dir = get_screen_param(0x18) as i32;
    let base = get_screen_param(0x19) as i32;
    let copy = get_screen_param(0x1d);
    let marks = get_screen_param(0x23) as u32;
    let e = displayed_match_state(p0);
    let it = |d: u32| b.wrapping_add(d);

    for (i, a, c) in [(0u32, 0x767, 0x142), (1, 0x763, 0x13e), (2, 0x764, 0x13f), (3, 0x765, 0x140), (4, 0x766, 0x141)] {
        if (i > 0 || p0 != 0) && (read_u8(e + a) as u32) + (read_u8(e + c) as u32) != 0 {
            exe::screenmanager_set_widget_text_flags(sm, marks.wrapping_add(i), 1);
            exe::screenmanager_set_widget_id(sm, marks.wrapping_add(i), 4);
        }
    }

    let s = |x: usize, k: isize| core::ptr::read_unaligned((x as isize + side * 2 + k) as *const i16) as i32;
    if copy == 0 || s(e, 0xc) != s(copy, 0xc) || s(e, 0x16) != s(copy, 0x16) || s(e, 0x20) != s(copy, 0x20) {
        let total = || s(e, 0xc) + s(e, 0x16) + s(e, 0x20);
        let a = if total() == 0 { 0 } else { (s(e, 0xc) * 100 / total()) as i8 as i32 };
        format(BUF, FORMAT_PCT, a);
        exe::screenmanager_set_widget_text(sm, b, BUF);
        let c = if total() == 0 { 0 } else { (s(e, 0x20) * 100 / total()) as i8 as i32 };
        format(BUF, FORMAT_PCT, c);
        exe::screenmanager_set_widget_text(sm, it(2), BUF);
        format(BUF, FORMAT_PCT, if total() == 0 { 0 } else { 100 - c - a });
        exe::screenmanager_set_widget_text(sm, it(1), BUF);
    }
    if copy == 0 || s(e, 0x2a) != s(copy, 0x2a) {
        if side == 4 {
            *(BUF as *mut u8) = 0;
        } else {
            let x = s(e, 0x2a) * 60 / 11;
            traduzido(BUF, 0xa168b4, x / 60, x % 60);
        }
        exe::screenmanager_set_widget_text(sm, it(3), BUF);
    }

    // the club colors: the first output (+0x3c on the original’s stack) is the bar color; the second
    // each call’s output is not used. Only the low 16 bits count (0x801040 reads a word).
    let club = |k: usize| (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((e + k) as *const usize)).wrapping_mul(club::SIZE));
    let (mut cor1, mut cor2, mut resto1, mut resto2) = ([0u8; 2], [0u8; 2], [0u8; 2], [0u8; 2]);
    exe::database_get_club_kit_colours(club(0), (read_u8(e + match_record::HOME_KIT) as u32), (cor1.as_mut_ptr()) as *mut u32, (resto1.as_mut_ptr()) as *mut u32);
    exe::database_get_club_kit_colours(club(4), (read_u8(e + match_record::AWAY_KIT) as u32), (cor2.as_mut_ptr()) as *mut u32, (resto2.as_mut_ptr()) as *mut u32);
    let colour = |pct: u32| u16::from_le_bytes(if pct < 0x32 { cor2 } else { cor1 }) as u32;
    let value = |field: isize| *((e as isize + side + field) as *const u8);
    let changed = |field: isize| copy == 0 || value(field) != *((copy as isize + side + field) as *const u8);
    let pct = |field: isize, mark: isize| if value(field) == 0 || value(mark) == 0 { 0x32 } else { value(field) as u32 };
    let zona = |field: isize, mark: isize, start: i32, end: i32, d: u32| {
        let p = pct(field, mark);
        let x = (end - start - 4) * p as i32 / 100 + start + 2;
        exe::screenmanager_set_widget_rect(sm, it(d), start + 2, base - 0x12, x, base - 0xc);
        exe::screenmanager_set_widget_rect(sm, it(d + 1), x, base - 0x12, end - 2, base - 0xc);
        exe::screenmanager_set_widget_colour(sm, it(d + 2), colour(p));
    };
    if changed(0x143) {
        zona(0x143, 0x768, esq + 10, (dir - esq) / 3 + esq - 10, 6);
    }
    if changed(0x148) {
        zona(0x148, 0x76d, (dir - esq) / 3 + esq + 10, (dir - esq) * 2 / 3 + esq - 10, 0xb);
    }
    if changed(0x14d) {
        zona(0x14d, 0x772, (dir - esq) * 2 / 3 + esq + 10, dir - 10, 0x10);
    }
    if changed(0x13e) {
        let p = pct(0x13e, 0x763);
        format(BUF, FORMAT_PCT, p);
        let w = (p * 0x1d2 / 100) as i32 + 0xd4;
        exe::screenmanager_set_widget_rect(sm, it(0x15), 0xd4, height - 0x23, w, height - 0x14);
        exe::screenmanager_set_widget_text(sm, it(0x15), BUF);
        format(BUF, FORMAT_PCT, 100u32.wrapping_sub(p));
        exe::screenmanager_set_widget_rect(sm, it(0x16), w, height - 0x23, 0x2a6, height - 0x14);
        exe::screenmanager_set_widget_text(sm, it(0x16), BUF);
        exe::screenmanager_set_widget_colour(sm, it(0x17), colour(p));
    }
    save_match_snapshot(copy, e);
}

/// Updates the player ratings rows with the values that changed since the copy in param `0x1d` (with no copy, all):
/// cards, injury/sending-off/substitution minute, rating, goals and name colour; updates the state copy.
///
/// 7 items per row from param `0xd`; 20 rows per team, the row only if item + 6 < `0x4b0`, otherwise it skips without
/// advancing. Per row (73 bytes at `+0x190`): empty (`0xfb` at `+0x1d`) clears the items; otherwise the
/// borders/backgrounds of the two cards (`+0x2c` and `+0x2d`), the minute of injury, sending off, going off or coming
/// on (`+0x33`, `+0x34`, `+0x30`, `+0x32`, "inj/off/sub/on <%d - minute>", with colour), the rating (`+0x35`) and the
/// goals (`+0x21`; empty for a substitute who did not come on) and the colour of the name (the player at `+4` equal to
/// `[estado + 0x3c]`, a player who went off, a substitute who did not come on). At the end it stores the state.
/// Original: `refresh_player_ratings_rows` (0x723350, match_screens.cpp, cdecl).
///
/// [pt] { 0x723350 } (cdecl, match_screens.cpp): passa às linhas das notas dos jogadores (7 itens por linha a
/// partir do parâmetro 0xd; 20 linhas por time, a linha só se o item + 6 < 0x4b0, senão pula sem andar)
/// os valores que mudaram desde a cópia do parâmetro 0x1d (sem cópia, todos). Por linha (73 bytes em
/// +0x190): vazia (0xfb em +0x1d) limpa os itens; senão as margens/fundos dos dois cartões (+0x2c e
/// +0x2d), o minuto de lesão, expulsão, saída ou entrada (+0x33, +0x34, +0x30, +0x32, "inj/off/sub/on
/// <%d - minute>", com cor), a nota (+0x35) e os gols (+0x21; vazios para o reserva que não entrou) e a
/// cor do nome (o jogador em +4 igual ao [estado + 0x3c], quem saiu, o reserva que não entrou). No fim
/// guarda o estado.
pub(crate) unsafe extern "C" fn refresh_player_ratings_rows() {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzido: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let p0 = get_screen_param(0);
    let mut item = get_screen_param(0xd) as u32;
    let copy = get_screen_param(0x1d);
    let e = displayed_match_state(p0);
    let current = core::ptr::read_unaligned((e + match_record::MAN_OF_THE_MATCH_ID) as *const u32);
    for t in 0..2usize {
        for j in 0..20usize {
            if item as i16 as i32 + 6 > 0x4af {
                continue;
            }
            let row = j * 0x49;
            let i = item.wrapping_add(1);
            let r = e + t * 0x625 + row + 0x190;
            let c = if copy != 0 { copy + t * 0x625 + row + 0x190 } else { 0 };
            let er = |k: usize| *((r + k) as *const u8);
            let cr = |k: usize| *((c + k) as *const u8);
            let difere = |ks: &[usize]| c == 0 || ks.iter().any(|&k| er(k) != cr(k));
            if er(match_record_row::POSITION) == 0xfb {
                if c == 0 {
                    exe::screenmanager_set_widget_style(sm, i, 1);
                    exe::screenmanager_set_widget_style(sm, i + 1, 1);
                    exe::screenmanager_set_widget_text(sm, i + 3, EMPTY_TEXT);
                    exe::screenmanager_set_widget_text(sm, i + 4, EMPTY_TEXT);
                    exe::screenmanager_set_widget_text(sm, i + 5, EMPTY_TEXT);
                }
            } else {
                if difere(&[0x2c, 0x2d]) {
                    let (a, d) = (er(match_record_row::YELLOW_CARDS), er(match_record_row::RED_CARD));
                    if a != 0 && d != 0 {
                        exe::screenmanager_set_widget_style(sm, i, 0x20030);
                        exe::screenmanager_set_widget_colour(sm, i, colour_word(G_GUI_COLOUR_YELLOW) as u32);
                        exe::screenmanager_set_widget_style(sm, i + 1, 0x40030);
                        exe::screenmanager_set_widget_colour(sm, i + 1, colour_word(G_GUI_COLOUR_RED) as u32);
                    } else if a == 0 && d != 0 {
                        exe::screenmanager_set_widget_style(sm, i, 0x20030);
                        exe::screenmanager_set_widget_colour(sm, i, colour_word(G_GUI_COLOUR_RED) as u32);
                        exe::screenmanager_set_widget_style(sm, i + 1, 1);
                    } else if a != 0 {
                        exe::screenmanager_set_widget_style(sm, i, 0x20030);
                        exe::screenmanager_set_widget_colour(sm, i, colour_word(G_GUI_COLOUR_YELLOW) as u32);
                        exe::screenmanager_set_widget_style(sm, i + 1, 1);
                    } else {
                        exe::screenmanager_set_widget_style(sm, i, 1);
                        exe::screenmanager_set_widget_style(sm, i + 1, 1);
                    }
                }
                let m = |k: usize| er(k) as i8;
                if difere(&[0x33, 0x34, 0x30, 0x32]) {
                    let minute = if m(0x33) >= 0 {
                        Some((0xa16a68usize, m(0x33), 0xae3154usize)) // "inj <%d - minute>"
                    } else if m(0x34) >= 0 {
                        Some((0xa16a54, m(0x34), 0xae3154)) // "off <%d - minute>"
                    } else if m(0x30) >= 0 {
                        Some((0xa16a40, m(0x30), 0xae3154)) // "sub <%d - minute>"
                    } else if m(0x32) >= 0 {
                        Some((0xa16a2c, m(0x32), 0xaebdd4)) // "on <%d - minute>"
                    } else {
                        None
                    };
                    match minute {
                        Some((f, v, colour)) => {
                            traduzido(BUF, f, v as i32 + 1);
                            exe::screenmanager_set_widget_text_colour(sm, i + 3, colour_word(colour) as u32);
                            exe::screenmanager_set_widget_text(sm, i + 3, BUF);
                        }
                        None => exe::screenmanager_set_widget_text(sm, i + 3, EMPTY_TEXT),
                    }
                }
                if row >= 0x323 && m(0x32) < 0 {
                    // the substitute who did not come on: only clears, and only without a copy
                    if c == 0 {
                        exe::screenmanager_set_widget_text(sm, i + 4, EMPTY_TEXT);
                        exe::screenmanager_set_widget_text(sm, i + 5, EMPTY_TEXT);
                    }
                } else {
                    if difere(&[0x35, 0x32]) {
                        format(BUF, FORMAT_D, er(match_record_row::MATCH_GRADE) as i32);
                        exe::screenmanager_set_widget_text(sm, i + 4, BUF);
                    }
                    if difere(&[0x21, 0x32]) {
                        if er(match_record_row::GOALS) != 0 {
                            format(BUF, FORMAT_D, er(match_record_row::GOALS) as i32);
                            exe::screenmanager_set_widget_text(sm, i + 5, BUF);
                        } else {
                            exe::screenmanager_set_widget_text(sm, i + 5, EMPTY_TEXT);
                        }
                    }
                }
                let colour = if core::ptr::read_unaligned((r + match_record_row::PLAYER_ID) as *const u32) == current {
                    0xaebde8
                } else if m(0x33) >= 0 || m(0x34) >= 0 || m(0x30) >= 0 || (row > 0x2da && m(0x32) == -1) {
                    G_GUI_COLOUR_GREY
                } else {
                    G_GUI_COLOUR_PALE_GREY
                };
                if difere(&[0x33, 0x34, 0x30, 0x32]) || current != core::ptr::read_unaligned((copy + match_record::MAN_OF_THE_MATCH_ID) as *const u32) {
                    exe::screenmanager_set_widget_text_colour(sm, i + 2, colour_word(colour) as u32);
                }
            }
            item = item.wrapping_add(7);
        }
    }
    save_match_snapshot(copy, e);
}

pub(crate) type ListCellFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type ListCellTextFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type DisplayListFn = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// The current human's Display ([0xb74c4c + 0x300 x slot]).
///
/// [pt] O `Display` do humano da vez ([0xb74c4c + 0x300 x (i16)[0xb77c7a]]).
pub(crate) unsafe fn current_human_display() -> usize {
    *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize)
}

/// 16-bit colour stored at a, zero-extended.
///
/// [pt] A cor (u16) guardada em `a`, estendida com zero. Onde a original empurra um registrador inteiro com
/// lixo imprevisível nos 16 bits de cima (o que sobrou de uma chamada), vai o valor estendido.
pub(crate) unsafe fn colour_word(a: usize) -> usize {
    *(a as *const u16) as usize
}

/// Draws one team's stats table from `*topo` to `*base` (page header, arrows, column titles) or the no-stats message.
///
/// Outside the match (param 0 null) and with no statistics (`[param 0x20 + 0x4c]` ≠ 0): only the message "There are no
/// team stats available." (`0x5ed070`, as many lines as fit in font 3). Otherwise: the frame (`0x55fba0`), the
/// 22-column header table, the page arrows in columns 4 and 5 ("<<" and ">>", or '*' and '+' with the symbol font
/// `[0x9cc2c4]` ≠ -1; the item of column 4 in param `0x1a`), the arrow of the current page (param `0x1c`: 1 or 2)
/// disabled (`0x800580`), the titles of columns 7 to `0x13` from the page's table (`0xa16178` / `0xa1618c`; the text at
/// `[0xa16118 + 4k]`, with each page's own comment titles), the values table, side `lado` in param `0x1b`, -1 in `0x1a`
/// and the filling (`0x721cd0`).
/// Original: `draw_team_stats_panel` (0x7217a0, match_screens.cpp, cdecl).
///
/// [pt] { 0x7217a0 } (cdecl, match_screens.cpp): desenha a tabela das estatísticas de um time de `*topo` a
/// `*base`. Fora da partida (parâmetro 0 nulo) e sem estatísticas ([parâmetro 0x20 + 0x4c] ≠ 0): só a
/// mensagem "There are no team stats available." (0x5ed070, as linhas que cabem na fonte 3). Senão: a
/// moldura (0x55fba0), a tabela do cabeçalho de 22 colunas, as setas de página nas colunas 4 e 5 ("<<"
/// e ">>", ou '*' e '+' pela fonte de símbolos [0x9cc2c4] ≠ -1; o item da 4 no parâmetro 0x1a), a seta
/// da página atual (parâmetro 0x1c: 1 ou 2) desligada (0x800580), os títulos das colunas 7 a 0x13 pela
/// tabela da página (0xa16178 / 0xa1618c; o texto em [0xa16118 + 4k], com os títulos de comentário
/// próprios de cada página), a tabela dos valores, o lado `lado` no parâmetro 0x1b, -1 no 0x1a e o
/// preenchimento (0x721cd0).
pub(crate) unsafe extern "C" fn draw_team_stats_panel(side: u32, top: *mut i32, base: *mut i32) {
    crate::comparison::branch("screens team statistics panel");
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let pagina = get_screen_param(0x1c) as u8;
    let mut cabecalho: [u8; 22] = [1, 1, 1, 0x49, 8, 8, 3, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xe, 0];
    let mut valores: [u8; 22] = [0, 0xe, 1, 5, 1, 0x35, 0x15, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 7, 7];
    if get_screen_param(0) == 0 && *((get_screen_param(0x20) + 0x4c) as *const u8) != 0 {
        let a = exe::font_get_height(3);
        let n = ((*base - *top - 0x10) / a) as u8;
        text(BUF, 0xa16a08);
        let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
            core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
        mensagem(0x6e, *top as usize, 0x30c, *base as usize, 2, 0, n as usize, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0);
        return;
    }

    list(current_human_display(), 0x6e, *top as usize, 0x30c, *base as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    let table = list(current_human_display(), 0x6e, *top as usize, 0x30c, *base as usize, 0x16, cabecalho.as_mut_ptr() as usize, 0x15, 0, 1, 0, usize::MAX);
    // the ”<<” arrow: with the normal font the original pushes eax = [0x9cc2c4] (-1) with the colour in ax
    let r = if *(G_FONT_MODE as *const i32) != -1 {
        format(BUF, 0x99ec1c, 0x2a);
        cell(sm, table, 4, 0, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 0, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 9, 0)
    } else {
        cell(sm, table, 4, 0, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, 0xffff_0000 | colour_word(G_GUI_COLOUR_PALE_GREY), 0x98e7c4, 9, 0)
    };
    set_screen_param(0x1a, r as u16 as i16 as i32 as usize);
    if *(G_FONT_MODE as *const i32) != -1 {
        format(BUF, 0x99ec1c, 0x2b);
        cell(sm, table, 5, 0, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 0, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0xa, 0);
    } else {
        cell(sm, table, 5, 0, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0x98e7a8, 0xa, 0);
    }
    // the page’s column table; on a page other than 1 and 2 the original reads by the pointer’s value
    // `base` (the argument slot is what receives the table), and no title is output
    let (colunas, seta) = match pagina {
        1 => (G_TEAM_STATS_COLUMN_IDS_VIEW_1, r),
        2 => (G_TEAM_STATS_COLUMN_IDS_VIEW_2, r.wrapping_add(1)),
        _ => (base as usize, r.wrapping_add(1)),
    };
    exe::screenmanager_set_widget_flags(sm, seta, 1);
    for c in 7..0x14usize {
        let k = *((colunas + c - 2) as *const u8) as usize;
        let rotulo = *((0xa16118 + k * 4) as *const usize);
        if rotulo == 0 {
            continue;
        }
        let own = match pagina {
            1 => match c {
                0xb => Some(0xa16968usize),
                0xe => Some(0xa169e8),
                0x11 => Some(0xa169cc),
                0x12 => Some(0xa169a8),
                0x13 => Some(0xa16988),
                _ => None,
            },
            2 => match c {
                9 => Some(0xa16968usize),
                0xc => Some(0xa169cc),
                0xd => Some(0xa169a8),
                0xe => Some(0xa16944),
                0x10 => Some(0xa16924),
                0x13 => Some(0xa16988),
                _ => None,
            },
            _ => continue,
        };
        match own {
            Some(f) => text(BUF, f, 0xad9c64),
            None => text(BUF, rotulo, 0),
        };
        text_of_cell(sm, table, c, 0, 0x30, colour_word(G_GUI_COLOUR_GREY), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF);
    }
    let tabela_dos_valores = list(current_human_display(), 0x6e, *top as usize, 0x30c, *base as usize, 0x16, valores.as_mut_ptr() as usize, 0x15, 0, 1, 0, usize::MAX);
    set_screen_param(0x1b, side as u8 as i8 as i32 as usize);
    set_screen_param(0x1a, usize::MAX);
    exe::fill_team_stats_rows(1, tabela_dos_valores);
}

/// Draws both teams' player ratings from `*topo` to `*base`: shirt number, name (button, with the captain) and rating.
///
/// A 16-column table (8 per team) and 20 rows per team. The data: in the match (param 0) `[p0 + 0x5f]` and the match p0
/// + 4; outside it params `0x20` and `0x1f`. For row `j` of team `i` (73-byte record at data + 1573i + 73j + `0x190`):
/// the shirt number (or j + 1) with a background by starter/substitute (the first 11 and up to `0xb` +
/// `[partida+0x4d]`&`0xf`; the item of the first in param `0xd`), two separators, the name (`0x53e140`; "(c)" for the
/// captain, `[dados + 1573i + 0x153]`) as a button (`0x8040a0`, item `0x7dc` or `0x7d7` by the balance of `[dados]` or
/// `[dados+4]`) when the person is valid (`0x53e990`), otherwise text; empty (`0xfb` at `+0x1d`) with no name; and the
/// three rating cells. At the end, `0x723350`.
/// Original: `draw_player_ratings_panel` (0x722ed0, match_screens.cpp, cdecl).
///
/// [pt] { 0x722ed0 } (cdecl, match_screens.cpp): desenha as notas dos jogadores dos dois times de `*topo` a
/// `*base`: uma tabela de 16 colunas (8 por time) e 20 linhas por time. Os dados: na partida (parâmetro
/// 0) [p0 + 0x5f] e a partida p0 + 4; fora dela os parâmetros 0x20 e 0x1f. Por linha `j` do time `i`
/// (registro de 73 bytes em dados + 1573i + 73j + 0x190): o número da camisa (ou j + 1) com fundo por
/// titular/reserva (as 11 primeiras e até 0xb + [partida+0x4d]&0xf; o item da primeira no parâmetro
/// 0xd), dois separadores, o nome (0x53e140; "(c)" no capitão, [dados + 1573i + 0x153]) como botão
/// (0x8040a0, item 0x7dc ou 0x7d7 pelo saldo de [dados] ou [dados+4]) quando a pessoa vale (0x53e990),
/// senão texto; vazio (0xfb em +0x1d) sem nome; e as três células da nota. No fim, 0x723350.
pub(crate) unsafe extern "C" fn draw_player_ratings_panel(top: *mut i32, base: *mut i32) {
    crate::comparison::branch("screens player ratings panel");
    let cell: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let mut columns: [u8; 16] = [0xe, 1, 5, 1, 0x46, 0x15, 7, 7, 0xe, 1, 5, 1, 0x46, 0x15, 7, 7];
    let p0 = get_screen_param(0);
    let (dados, partida) = if p0 != 0 {
        (core::ptr::read_unaligned((p0 + live_match::RECORD) as *const usize), p0 + live_match::FIXTURE)
    } else {
        let d = get_screen_param(0x20);
        (d, get_screen_param(0x1f))
    };
    let table = list(current_human_display(), 0x6e, *top as usize, 0x30c, *base as usize, 0x10, columns.as_mut_ptr() as usize, 0x14, 0, 2, 0, usize::MAX);
    for i in 0..2usize {
        let col = 8 * i;
        for j in 0..0x14usize {
            let reg = dados + 1573 * i + 73 * j + 0x190;
            let camisa = *((reg + match_record_row::SHIRT_NUMBER) as *const i8);
            format(BUF, 0x98806c, if camisa > 0 { camisa as i32 } else { j as i32 + 1 });
            let starters = (*((partida + fixture::SQUAD_RULES) as *const u8) & 0xf) as usize + 0xb;
            // the first 11: the original pushes eax = the match with the color in ax
            let bottom = if j < 0xb { (partida & 0xffff_0000) | colour_word(G_GUI_COLOUR_DARK_BLUE) } else { (exe::graphics_rgb_to_pixel(0, 0, 0x40, 0) as usize) };
            let r = if j < starters {
                text_of_cell(sm, table, col, j, 0x30, bottom, 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), BUF)
            } else {
                text_of_cell(sm, table, col, j, 0x30, bottom, 0x2c, 1, colour_word(G_GUI_COLOUR_PALE_GREY), 0xad9c64)
            };
            if i == 0 && j == 0 {
                set_screen_param(0xd, r as u16 as i16 as i32 as usize);
            }
            text_of_cell(sm, table, col + 2, j, 1, colour_word(G_GUI_COLOUR_YELLOW), 0xc, 1, colour_word(G_GUI_COLOUR_BLACK), 0);
            text_of_cell(sm, table, col + 2, j, 1, colour_word(G_GUI_COLOUR_RED), 0xc, 1, colour_word(G_GUI_COLOUR_BLACK), 0);
            if *((reg + match_record_row::POSITION) as *const u8) == 0xfb {
                text_of_cell(sm, table, col + 4, j, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_GREY), 0);
            } else {
                let id = core::ptr::read_unaligned(reg as *const i32);
                let a = core::ptr::read_unaligned((reg + match_record_row::FIRST_NAME_ID) as *const u32);
                let b = core::ptr::read_unaligned((reg + match_record_row::SECOND_NAME_ID) as *const u32);
                let c = core::ptr::read_unaligned((reg + match_record_row::COMMON_NAME_ID) as *const u32);
                let mut name = [0u8; 0xc8];
                exe::database_format_name_from_ids(a, b, c, (name.as_mut_ptr() as usize) as *mut u8, 0xc8, 1, (id) as u32);
                let capitao = *((dados + 1573 * i + 0x153) as *const i8) as i32;
                if j as i32 == capitao {
                    text(BUF, 0xa16a7c, name.as_ptr() as usize);
                } else {
                    copy_to_work_text(name.as_ptr() as usize);
                }
                let mut botao = false;
                if id >= 0 && id < *(G_STAFF_COUNT as *const i32) {
                    let person = (*(G_STAFF_TABLE as *const usize) as isize + id as isize * 110) as usize;
                    if exe::database_staff_matches_names(person, a, b, c, reg + match_record_row::DATE_OF_BIRTH) != 0 {
                        botao = true;
                        let goal_difference = if i == 0 { *(dados as *const i32) } else { *((dados + match_record::AWAY_CLUB_ID) as *const i32) };
                        let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
                        let item = if goal_difference >= limit { 0x7dc } else { 0x7d7 };
                        cell(sm, table, col + 4, j, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_GREY), BUF, item, person);
                    }
                }
                if !botao {
                    text_of_cell(sm, table, col + 4, j, 1, 0, 1, 2, colour_word(G_GUI_COLOUR_GREY), BUF);
                }
            }
            text_of_cell(sm, table, col + 5, j, 1, 0, 1, 1, colour_word(0xae3154), 0);
            text_of_cell(sm, table, col + 6, j, 1, 0, 0xc, 2, colour_word(0xaebde8), 0);
            text_of_cell(sm, table, col + 7, j, 1, 0, 0xc, 1, colour_word(G_GUI_COLOUR_YELLOW), 0);
        }
    }
    exe::refresh_player_ratings_rows();
}

/// The labels of the 13 rows of the match statistics and their colours.
///
/// [pt] Os rótulos das 13 linhas das estatísticas da partida e as cores deles.
pub(crate) const MATCH_STATS_LABELS: [(usize, usize); 13] = [
    (0xa16870, G_GUI_COLOUR_PALE_GREY), // Shots On Goal
    (0xa16864, G_GUI_COLOUR_PALE_GREY), // On Target
    (0xa16858, G_GUI_COLOUR_PALE_GREY), // Off Target
    (0xa16850, G_GUI_COLOUR_PALE_GREY), // Corners
    (0xa16844, G_GUI_COLOUR_PALE_GREY), // Free Kicks
    (0xa16838, G_GUI_COLOUR_PALE_GREY), // Throw-Ins
    (0xa16830, G_GUI_COLOUR_PALE_GREY), // Fouls
    (0xa16824, G_GUI_COLOUR_PALE_GREY), // Offsides
    (0xa16810, G_GUI_COLOUR_PALE_GREY), // Passes Completed
    (0xa16804, G_GUI_COLOUR_PALE_GREY), // Tackles Won
    (0xa167f8, G_GUI_COLOUR_PALE_GREY), // Headers Won
    (0x99caf4, G_GUI_COLOUR_YELLOW), // Yellow Cards
    (0x99cae8, G_GUI_COLOUR_RED), // Red Cards
];

/// Draws the match stats from `*topo` to `*base`: 13 labelled rows (shots, corners, fouls, passes, tackles, headers,
/// cards).
///
/// The 3-column table (1, 5, 1) and 13 rows, each with the cells of the two teams (columns 0 and 2; the first one's
/// item in param 9) and the label in the middle; then the values (`0x71e750`).
/// Original: `draw_match_stats_panel` (0x71e040, match_screens.cpp, cdecl).
///
/// [pt] { 0x71e040 } (cdecl, match_screens.cpp): desenha as estatísticas da partida de `*topo` a `*base`: a
/// tabela de 3 colunas (1, 5, 1) e 13 linhas, cada uma com as células dos dois times (colunas 0 e 2;
/// o item da primeira no parâmetro 9) e o rótulo no meio; depois os valores (0x71e750).
pub(crate) unsafe extern "C" fn draw_match_stats_panel(top: *mut i32, base: *mut i32) {
    crate::comparison::branch("screens match statistics panel");
    let text_of_cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let list: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;

    let mut columns: [u8; 3] = [1, 5, 1];
    let table = list(current_human_display(), 0x6e, *top as usize, 0x30c, *base as usize, 3, columns.as_mut_ptr() as usize, 0xd, 0, 2, 0, usize::MAX);
    for (k, &(rotulo, cor)) in MATCH_STATS_LABELS.iter().enumerate() {
        let r = text_of_cell(sm, table, 0, k, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 3, colour_word(0xaebde8), 0);
        if k == 0 {
            set_screen_param(9, r as u16 as i16 as i32 as usize);
        }
        text(BUF, rotulo);
        text_of_cell(sm, table, 1, k, 1, 0, 0xc, 4, colour_word(cor), BUF);
        text_of_cell(sm, table, 2, k, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 3, colour_word(0xaebde8), 0);
    }
    exe::refresh_match_stats_values();
}

/// Value at [0xb74c26 + 0x300 x current slot]: the current human's slot entry [inference: his person pointer].
///
/// [pt] A vaga de tela do humano da vez ([0xb74c26 + 0x300 × humano]).
pub(crate) unsafe fn current_human_screen_person() -> usize {
    core::ptr::read_unaligned((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize)
}

/// Id of display item (or the default [0xdd6bde] when item is -1).
///
/// [pt] O id do item `item` (−1 nenhum: o padrão [0xdd6bde]), como as respostas de match_screens.cpp leem.
pub(crate) unsafe fn item_id_or_default(item: i16) -> u16 {
    if item == -1 { *(G_CURRENT_COMMAND_ID as *const u16) } else { display_item_id(item) }
}

/// Handles the match Continue (item `item`, −1 none): the pending team selection command (online, the club's free slot
/// or waiting for the others), the network matches and team selection.
///
/// On a network (`[0xb77c84]`): with pending command 1 (`[0xdd6be0]`, cleared with `[0xdd6be4]`) and choice 3, it looks
/// in the manager's slot list (`0x7ffd80`) for another free slot (`0x7ffdd0`) of the human's club (`0x765480` `+0xe2`):
/// if found, stores it in `[0xb77c4a]` and returns −6; otherwise, with other players waiting (`0x803d80`), shows
/// "Waiting for other players..." (`0x802f80`), opens the match screen (`0x718aa0`) and returns −4, or −11. Pending
/// command `0x43`: opens the team selection (`0x725f70`) and returns −4.
///
/// Then, only the item with id `0x3e8`: with the match network (`[0xb67a48]`) it opens the first entry of list
/// `[0xb67a50]` (`+0xd`, `0x54` bytes; first those with `+0x10`, then those with `+0xc`) that the human has not yet
/// marked (`0x717cc0`, −4) or returns −8; without it, with a record and a club, it opens the team selection if
/// `0x474e00` refuses and returns −4.
/// Original: `match_screens_handle_continue_command` (0x725ca0, match_screens.cpp, cdecl).
///
/// [pt] { 0x725ca0 } (cdecl, match_screens.cpp): a resposta ao Continuar da partida (item `item`, −1 nenhum).
/// Em rede ([0xb77c84]): com o comando pendente 1 ([0xdd6be0], zerado com [0xdd6be4]) e a escolha 3,
/// procura na lista de vagas do gerente (0x7ffd80) outra vaga livre (0x7ffdd0) do mesmo clube do humano
/// (0x765480 +0xe2): achou, guarda em [0xb77c4a] e devolve −6; senão, com outros jogadores a esperar
/// (0x803d80), avisa "Waiting for other players..." (0x802f80), abre a tela da partida (0x718aa0) e
/// devolve −4, ou −11. Comando pendente 0x43: abre a escalação (0x725f70) e devolve −4. Depois, só o
/// item de id 0x3e8: com a rede da partida ([0xb67a48]) abre a primeira entrada da lista [0xb67a50]
/// (+0xd, 0x54 bytes; primeiro as de +0x10, depois as de +0xc) que o humano ainda não marcou (0x717cc0,
/// −4) ou devolve −8; sem ela, com ficha e clube, abre a escalação se 0x474e00 negar e devolve −4.
pub(crate) unsafe extern "C" fn match_screens_handle_continue_command(item: u32) -> i32 {
    crate::comparison::branch("screens respond on continue");
    let g = |a: usize| *(a as *const u32);
    let human = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16);
    let r = exe::get_human_manager_record(0);
    let club = if r != 0 { core::ptr::read_unaligned((r + human_manager_sheet::LIVE_MATCH) as *const usize) } else { 0 };
    if g(0xb77c84) != 0 {
        let comando = g(G_PENDING_COMMAND_CODE);
        if comando == 1 {
            let choice = g(G_PENDING_COMMAND_ARG);
            *(G_PENDING_COMMAND_CODE as *mut u32) = 0;
            *(G_PENDING_COMMAND_ARG as *mut u32) = 0;
            if choice == 3 {
                let (mut n, mut lista) = (0i16, [0usize; 16]);
                // the original pushes the whole eax: the high 16 bits are those of the record returned by 0x765480
                exe::screenmanager_get_channel_manager_keys(SCREEN_MANAGER, lista.as_mut_ptr(), &mut n, ((r as u32 & 0xffff_0000) | human() as u16 as u32) as i32);
                let mut i: i8 = 0;
                if n > 0 {
                    loop {
                        let e = *lista.as_ptr().offset(i as isize);
                        if e != current_human_screen_person()
                            && exe::screenmanager_is_manager_away(SCREEN_MANAGER, e) == 0
                            && core::ptr::read_unaligned((exe::get_human_manager_record(e) + 0xe2) as *const usize) == club
                        {
                            core::ptr::write_unaligned(G_MATCH_WAITING_MANAGER as *mut usize, e);
                            return -6;
                        }
                        i = i.wrapping_add(1);
                        if i as i16 >= n {
                            break;
                        }
                    }
                }
                if exe::screenmanager_close_all_screens_803d80(SCREEN_MANAGER) == 0 {
                    return -11;
                }
                let format_text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                format_text(0xde1f64, 0xa16d94);
                // simplification: the original pushes ecx with high 16 bits left over from 0x66f4e0 (unpredictable); here zero
                exe::screenmanager_send_network_status_message(SCREEN_MANAGER, 0xde1f64, (0) as i32, 0, (human() as u16 as u32) as i32);
                exe::open_match_screen(current_human_screen_person(), club);
                return -4;
            }
        } else if comando == 0x43 {
            *(G_PENDING_COMMAND_CODE as *mut u32) = 0;
            exe::match_screens_open_team_selection();
            return -4;
        }
    }
    if item_id_or_default(item as u16 as i16) != 0x3e8 {
        return 0;
    }
    if g(G_MATCH_DAY_SCREEN_SHOWING_FLAG) != 0 {
        let n = core::ptr::read_unaligned((*(G_MATCH_DAY as *const usize) + 0x15) as *const i32);
        for modo in 1..=2u8 {
            let base = core::ptr::read_unaligned((*(G_MATCH_DAY as *const usize) + 0xd) as *const usize);
            let field = if modo == 1 { 0x10 } else { 0xc };
            let mut i: i8 = 0;
            if n > 0 {
                loop {
                    let e = (base as isize + i as isize * 0x54) as usize;
                    if core::ptr::read_unaligned((e + field) as *const u32) != 0
                        && core::ptr::read_unaligned((e as isize + human() as isize * 4 + 0x14) as *const u32) == 0
                    {
                        exe::open_day_fixtures_screen((current_human_screen_person()) as u32, e);
                        return -4;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n {
                        break;
                    }
                }
            }
        }
        return -8;
    }
    if r != 0 && club != 0 {
        if exe::open_squad_numbers_screen_for_match(r) == 0 {
            exe::match_screens_open_team_selection();
        }
        return -4;
    }
    0
}

/// Opens the Team Selection screen for the human's next match and stores the club, the match and the home flag.
///
/// Opens the screen (`0x7fe600` with the builder `0x7260f0` and the handler `0x727c30`) and writes the params: 0 the
/// club (record `0x765480` `+0xe6`), 1 the match (`[ficha+0xe2]` + 4) and 2 whether the club plays at home (`+0x1c`
/// without flag `+0x40`, or `+0x20` without `+0x41`). With no record, club or match: the error box (lines `0x1e1c`,
/// `0x1e24`, `0x1e2c`), as in the original (the screen has already been opened).
/// Original: `match_screens_open_team_selection` (0x725f70, match_screens.cpp, cdecl).
///
/// [pt] { 0x725f70 } (cdecl, match_screens.cpp): abre a tela da escalação da próxima partida do humano
/// (0x7fe600 com a montagem 0x7260f0 e a resposta 0x727c30) e grava os parâmetros: 0 o clube (ficha
/// 0x765480 +0xe6), 1 a partida ([ficha+0xe2] + 4) e 2 se o clube joga em casa (+0x1c sem a marca +0x40,
/// ou +0x20 sem a +0x41). Sem ficha, clube ou partida: a caixa de erro (linhas 0x1e1c, 0x1e24, 0x1e2c),
/// como a original (a tela já ficou aberta).
pub(crate) unsafe extern "C" fn match_screens_open_team_selection() {
    crate::comparison::branch("screens open the line-up");
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x7260f0, 0x727c30, 1, 0, 0) == 0 {
        return;
    }
    const FONTE: usize = 0xa161d4; // match_screens.cpp
    let r = exe::get_human_manager_record(0);
    if r == 0 {
        return show_error_at(FONTE, 0x1e1c);
    }
    let club = core::ptr::read_unaligned((r + human_manager_sheet::CLUB) as *const usize);
    if club == 0 {
        return show_error_at(FONTE, 0x1e24);
    }
    let partida = core::ptr::read_unaligned((r + human_manager_sheet::LIVE_MATCH) as *const usize).wrapping_add(4);
    if partida == 0 {
        return show_error_at(FONTE, 0x1e2c);
    }
    let d = |o: usize| core::ptr::read_unaligned((partida + o) as *const usize);
    let b = |o: usize| *((partida + o) as *const u8);
    let in_home = (club == d(fixture::HOME_CLUB) && b(fixture::HOME_SIDE_MARK) == 0) || (club == d(fixture::AWAY_CLUB) && b(fixture::AWAY_SIDE_MARK) == 0);
    set_screen_param(0, club);
    set_screen_param(1, partida);
    set_screen_param(2, in_home as usize);
}

/// Handles the Team Selection buttons (item `item`, −1 none): appoints the captain (the best starter) when missing and
/// stores the pending command; the rest go to the common handler.
///
/// Ids other than 1, 2 and 3: the common handler (`0x7615d0` and, if it returns 0, `0x7643d0`) with the full argument.
/// Id 3 (confirm): if tactic `[0xde1d08]` has no captain (`0x897590` with 1 negative) or he is not a starter
/// (`0x897590` with 0 ≥ 11), it appoints the best starter (`0x727e20`) via `0x897760`. For 1, 2 and 3 it leaves pending
/// command 1 (`[0xdd6be0]`) with the choice (the id, `[0xdd6be4]`) and returns −9.
/// Original: `match_screens_handle_team_selection_input` (0x727c30, match_screens.cpp, cdecl).
///
/// [pt] { 0x727c30 } (cdecl, match_screens.cpp): a resposta da tela da escalação ao item `item` (−1 nenhum).
/// Ids fora de 1, 2 e 3: a resposta comum (0x7615d0 e, se der 0, 0x7643d0) com o argumento inteiro. Id 3
/// (confirmar): se a tática [0xde1d08] não tem capitão (0x897590 com 1 negativo) ou ele não é titular
/// (0x897590 com 0 ≥ 11), nomeia o melhor titular (0x727e20) por 0x897760. Para 1, 2 e 3 deixa o
/// comando pendente 1 ([0xdd6be0]) com a escolha (o id, [0xdd6be4]) e devolve −9.
pub(crate) unsafe extern "C" fn match_screens_handle_team_selection_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond line-up");
    let club = get_screen_param(0);
    let p2 = get_screen_param(2);
    let item16 = item as u16 as i16;
    let id = item_id_or_default(item16);
    if id != 1 && id != 2 && id != 3 {
        let r = exe::menubar_handle_command(item);
        if r != 0 {
            return r;
        }
        return exe::menubar_handle_object_link(item);
    }
    if id == 3 {
        let tactic = || *(G_TACTICS as *const usize);
        if ((exe::tactics_find_captain_slot(tactic(), club, p2 as u32, 1) as i8)) < 0 || ((exe::tactics_find_captain_slot(tactic(), club, p2 as u32, 0) as i8)) >= 11 {
            let m = (exe::match_screens_find_best_rated_starter_slot(club, p2) as u32) as u8;
            if (m as i8) > -1 {
                // the original writes the byte over the argument and pushes the whole word
                exe::tactics_set_captain_from_slot(tactic(), club, ((item & 0xffff_ff00) | m as u32) as u32 as i8, p2 as u32);
            }
        }
    }
    *(G_PENDING_COMMAND_CODE as *mut u32) = 1;
    *(G_PENDING_COMMAND_ARG as *mut i32) = item_id_or_default(item16) as i16 as i32;
    -9
}

// simplification: all three open screens, touch the network or the captain (they do not run twice): in comparison mode
// original only; the copy is proven in Rust mode by the branches

/// Copies fields of size 1, 2 or 4 bytes from s to d over [ini, fim) in ascending order like the original.
///
/// [pt] Copia de `s` para `d` os campos de `tam` bytes (1, 2 ou 4) de `ini` a `fim`, em ordem crescente, como a original.
pub(crate) unsafe fn copy_fields(d: usize, s: usize, start: usize, end: usize, size: usize) {
    let mut k = start;
    while k < end {
        match size {
            1 => *((d + k) as *mut u8) = *((s + k) as *const u8),
            2 => core::ptr::write_unaligned((d + k) as *mut u16, core::ptr::read_unaligned((s + k) as *const u16)),
            _ => core::ptr::write_unaligned((d + k) as *mut u32, core::ptr::read_unaligned((s + k) as *const u32)),
        }
        k += size;
    }
}

/// Copies the match team block (`0x625` bytes) field by field, with its 20 player rows. Returns `this`.
///
/// Bytes 0..`0x11`, words `0x11`..`0x1f`, the four 5-byte arrays and bytes `0x33`/`0x34`, the two lists of 11 words
/// (`0x35`, `0x4b`), the doubles `0x61`/`0x65`, the words `0x69`..`0x71` and the 20 player rows of `0x49` bytes
/// (`0x71df00`).
/// Original: `match_team_block_assign` (0x71dd70, match_screens.cpp, thiscall).
///
/// [pt] { 0x71dd70 } (thiscall, match_screens.cpp): copia o bloco do time da partida (0x625 bytes) campo a
/// campo: bytes 0..0x11, palavras 0x11..0x1f, os quatro vetores de 5 bytes e os bytes 0x33/0x34, as duas
/// listas de 11 palavras (0x35, 0x4b), as duplas 0x61/0x65, as palavras 0x69..0x71 e as 20 linhas de
/// jogador de 0x49 bytes (0x71df00). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn match_team_block_assign(this: usize, src: usize) -> usize {
    copy_fields(this, src, 0, 0x11, 1);
    copy_fields(this, src, 0x11, 0x1f, 2);
    copy_fields(this, src, 0x1f, 0x35, 1);
    copy_fields(this, src, 0x35, 0x61, 2);
    copy_fields(this, src, 0x61, 0x69, 4);
    copy_fields(this, src, 0x69, 0x71, 2);
    for i in 0..20 {
        exe::match_player_row_assign(this + match_record_team::ROWS + i * 0x49, src + match_record_team::ROWS + i * 0x49);
    }
    this
}

/// Copies one player row of the team block (`0x49` bytes). Returns `this`.
///
/// Doubles 0..`0x14`, the date at `+0x14` through Date::operator= (`0x417c10`, with the return value in a local), bytes
/// `0x1c`..`0x36`, words `0x36`..`0x3e` and bytes `0x3e`..`0x49`.
/// Original: `match_player_row_assign` (0x71df00, match_screens.cpp, thiscall).
///
/// [pt] { 0x71df00 } (thiscall, match_screens.cpp): copia a linha de jogador do bloco do time (0x49 bytes):
/// duplas 0..0x14, a data em +0x14 pela Date::operator= (0x417c10, com o retorno numa local), bytes
/// 0x1c..0x36, palavras 0x36..0x3e e bytes 0x3e..0x49. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn match_player_row_assign(this: usize, src: usize) -> usize {
    copy_fields(this, src, 0, 0x14, 4);
    let mut volta = [0u8; 8];
    exe::date_operator_3d(this + match_record_row::DATE_OF_BIRTH, (&mut volta) as *mut [u8; 8] as usize, src + match_record_row::DATE_OF_BIRTH);
    copy_fields(this, src, 0x1c, 0x36, 1);
    copy_fields(this, src, 0x36, 0x3e, 2);
    copy_fields(this, src, 0x3e, 0x49, 1);
    this
}

/// Counts into result `res` the goals and shoot-out goals of day match `indice` up to the current minute of the live
/// match `partida`.
///
/// The day match is `[[0xb67a50]+0x19]` + `0x69` x index + `0x5f`; events of 4 bytes from `+0x57`, up to 50 or side
/// `0xff`; the minute is `[[partida+0x5f]+0x4d]`, signed 16 bits; with no live match, `[partida+0x67]` = `0xff` or
/// `[partida+0x5f]` null, it counts nothing. Type 1/3: goal for the side (0 goes to `+0x43`, otherwise `+0x44`); 2: own
/// goal (inverted); 7: shoot-out goal (`+0x49`/`+0x4a`, which start at `0xff` and go to 1 on the first); 8 and the
/// others: nothing. It first zeroes `+0x43..+0x46` and `+0x4b`/`+0x4c`. Null result or match: nothing.
/// Original: `match_screens_tally_live_score` (0x728e90, match_screens.cpp, cdecl).
///
/// [pt] { 0x728e90 } (cdecl, match_screens.cpp): conta no resultado `res` os gols da partida `indice` do dia
/// ([[0xb67a50]+0x19] + 0x69 x indice + 0x5f; lances de 4 bytes a partir de +0x57, até 50 ou lado 0xff)
/// até o minuto atual da partida `partida` ([[partida+0x5f]+0x4d], 16 bits com sinal; sem partida ao vivo,
/// [partida+0x67] = 0xff ou [partida+0x5f] nulo, não conta nada). Tipo 1/3: gol do lado (0 vai a +0x43,
/// senão +0x44); 2: gol contra (invertido); 7: gol nos pênaltis (+0x49/+0x4a, que começam em 0xff e vão
/// a 1 no primeiro); 8 e os outros: nada. Zera antes +0x43..+0x46 e +0x4b/+0x4c. Resultado ou partida
/// nulos: nada.
pub(crate) unsafe extern "C" fn match_screens_tally_live_score(partida: usize, index: u32, res: usize) {
    if res == 0 || partida == 0 {
        return;
    }
    let b = |k: usize| (res + k) as *mut u8;
    for k in [0x43, 0x44, 0x4b, 0x4c] {
        *b(k) = 0;
    }
    *b(fixture::HOME_PENALTIES) = 0xff;
    *b(fixture::AWAY_PENALTIES) = 0xff;
    *b(fixture::AWAY_GOALS_HALF_TIME) = 0;
    *b(fixture::HOME_GOALS_HALF_TIME) = 0;
    let day = core::ptr::read_unaligned((*(G_MATCH_DAY as *const usize) + 0x19) as *const u32);
    let game = core::ptr::read_unaligned(day.wrapping_add(index.wrapping_mul(0x69)).wrapping_add(0x5f) as usize as *const usize);
    if game == 0 {
        return;
    }
    let mut lance = game + match_record::EVENT_KIND;
    for _ in 0..0x32 {
        let side = *((lance - 2) as *const u8);
        if side == 0xff {
            break;
        }
        let ao_vivo = core::ptr::read_unaligned((partida + live_match::RECORD) as *const usize);
        if *((partida + live_match::STATE) as *const u8) != 0xff
            && ao_vivo != 0
            && *((lance + 1) as *const u8) as i16 <= core::ptr::read_unaligned((ao_vivo + match_record::MINUTE) as *const i16)
        {
            let kind = *(lance as *const u8);
            let field = match kind {
                1 | 3 => Some(if side == 0 { 0x43 } else { 0x44 }),
                2 => Some(if side == 0 { 0x44 } else { 0x43 }),
                _ => None,
            };
            if let Some(k) = field {
                *b(k) = (*b(k)).wrapping_add(1);
            } else if kind == 7 {
                let k = if side == 0 { 0x49 } else { 0x4a };
                *b(k) = if *b(k) == 0xff { 1 } else { *b(k) + 1 };
            }
        }
        lance += 4;
    }
}

/// reports whether a match statistics entry holds the two given clubs in either order and the given date
/// Original: `Match_Stats::fixture_entry_matches` (0x729500, match_screens.cpp, thiscall).
///
/// [pt] { 0x729500 } (thiscall, ret 0x14, match_stats.cpp): a entrada `k` (12 bytes em [this]) tem os dois
/// clubes em qualquer ordem (+0/+4) e as datas (+8/+0xa, palavras) dadas? Índice −1: não; abaixo de −1
/// ou além de [this+0x1c]: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn fixture_entry_matches(stats: usize, k: i32, a: i32, b: i32, data_a: i16, data_b: i16) -> i32 {
    if k < -1 || k > (*((stats + match_stats::CAPACITY) as *const i32)).wrapping_sub(1) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i16, i16) -> i32>(IDX_FIXTURE_ENTRY_MATCHES)(stats, k, a, b, data_a, data_b);
    }
    if k == -1 {
        return 0;
    }
    let e = (*(stats as *const usize) as isize + k as isize * 12) as usize;
    let d = |o: usize| *((e + o) as *const i32);
    let w = |o: usize| *((e + o) as *const i16);
    let clubs = (d(0) == a && d(4) == b) || (d(0) == b && d(4) == a);
    (clubs && w(8) == data_a && w(match_stats_entry::YEAR) == data_b) as i32
}

/// Returns the bucket (0 to 3) where the match statistics record is stored, by competition type, national team and
/// fixture flags.
///
/// 0 if one of the clubs (`+0x1c`/`+0x20`) has its record (`[+0xcf]`) among the last 16 of the base with `+0x5f`; 3 if
/// the record has `+0x4c` and `[0xa5a390]`; without `+0x40`, 3 if the type (`+0x3a`) is 1, otherwise 2; with `+0x40`
/// and a national team club (`0x53b900`): 0 if the club's word `+0x80` exceeds 4999 or `0x157b`, otherwise 2; otherwise
/// by the type (jump table at `0x729d38`) and the exe's byte tables at `0x729d64` and `0x729d94`.
/// Original: `match_stats_get_record_bucket` (0x729bd0, match_screens.cpp, stdcall).
///
/// [pt] { 0x729bd0 } (stdcall, ret 8, match_stats.cpp): o balde (0..3) do registro de estatística do jogo:
/// 0 se um dos clubes (+0x1c/+0x20) tem a ficha ([+0xcf]) entre as 16 últimas da base com +0x5f; 3 se o
/// registro tem +0x4c e [0xa5a390]; sem +0x40, 3 se o tipo (+0x3a) é 1, senão 2; com +0x40 e clube de
/// seleção (0x53b900): 0 se a palavra +0x80 do clube passa de 4999 ou de 0x157b, senão 2; senão pelo tipo
/// (tabela de saltos em 0x729d38) e as tabelas de bytes do exe em 0x729d64 e 0x729d94.
pub(crate) unsafe extern "stdcall" fn match_stats_get_record_bucket(game: usize, record: usize) -> u8 {
    let game = Fixture(game);
    let limit = *(G_STAFF_COUNT as *const i32);
    for o in [0x1c, 0x20] {
        let p = Staff(core::ptr::read_unaligned((*((game.addr() + o) as *const usize) + 0xcf) as *const usize));
        if !p.is_null() {
            let id = *(p.addr() as *const i32);
            if id >= limit.wrapping_sub(0x10) && id < limit && p.classification() != 0 {
                return 0;
            }
        }
    }
    if *((record + match_record::NO_STATISTICS) as *const u8) != 0 && *(G_MATCH_ENGINE_ENABLED as *const u8) != 0 {
        return 3;
    }
    if game.home_side_mark() == 0 {
        return (game.category() == 1) as u8 + 2;
    }
    if exe::database_is_national_team(game.home_club()) != 0 {
        if game.home_side_mark() == 0 {
            return 2;
        }
        let v = core::ptr::read_unaligned((game.home_club() + 0x80) as *const i16);
        return if v as i32 * 2 >= 10000 || v >= 0x157c { 0 } else { 2 };
    }
    // simplification: the two byte tables of the jumps are read from the exe itself (the hook only replaces the start of the routine)
    let table = |end: usize, i: i32| *((end + i as usize) as *const u8);
    match ((game.category() as i8) as i32).wrapping_sub(1) {
        0 => 3,
        4 | 8 => 1,
        5 => {
            let c = (game.round_name() as i32);
            if c as u32 > 0x9b {
                return 1;
            }
            match table(0x729d94, c) {
                0 => 0,
                1 => (core::ptr::read_unaligned((game.competition() + 0x69) as *const i16) <= 0xc) as u8,
                _ => 1,
            }
        }
        7 => {
            if (0x41f..=0x426).contains(&(game.round_prefix() as i32)) {
                return 0;
            }
            let c = (game.round_name() as i32) - 0x78;
            if c as u32 > 0x23 { 1 } else { (table(0x729d64, c) != 0) as u8 }
        }
        _ => {
            let n = *((game.competition() + 0x69) as *const i8);
            if n > 0xd { 1 } else { (n <= 6) as u8 + 2 }
        }
    }
}

/// Opens the day fixtures screen in slot `vaga` with the fixture group (error box without a group).
///
/// `0x7fe4a0` with the builder `0x717db0` and the handler `0x718930`, no close handler; it writes its params
/// (`0x7ff130`): 1 the fixture group, 2 and 3 zero, 4 one. Null group: the original routine runs (error box and
/// `[0xb67a34]` = 0).
/// Original: `open_day_fixtures_screen` (0x717cc0, match_screens.cpp, cdecl).
///
/// [pt] { 0x717cc0 } (cdecl, match_screens.cpp): abre a tela dos jogos do dia na vaga `vaga` (0x7fe4a0 com a
/// montagem 0x717db0 e a resposta 0x718930, sem fechamento) e grava os parâmetros dela (0x7ff130): 1 o
/// grupo de jogos, 2 e 3 zero, 4 um. Grupo nulo: a original (caixa de erro e [0xb67a34] = 0).
pub(crate) unsafe extern "C" fn open_day_fixtures_screen(slot: u32, group: usize) {
    if group == 0 {
        return original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_DAY_FIXTURES_SCREEN)(slot, group);
    }
    crate::comparison::branch("screens open the day matches");
    if exe::screenmanager_open_screen_for_each_record(SCREEN_MANAGER, slot, 0x717db0, 0x718930, 0, 0, 0) == 0 {
        return;
    }
    for (k, v) in [(1u32, group), (2, 0), (3, 0), (4, 1)] {
        exe::screenmanager_set_screen_param_for_each_record(SCREEN_MANAGER, slot, k, v, 0);
    }
}

/// Handles input on the day fixtures screen (item `item`, −1 = none): another fixture group, the competition progress
/// or the menu bar.
///
/// With an item: param 2 receives `0x803320` of param 3. By the item id (0 becomes the default `[0xdd6bde]`): 1 opens
/// another fixture group (`0x717cc0` in slot `[0xb74c26]` of the current human with the item's data `+0xba9a6`) and 2
/// the competition progress (`0x49b830` with the data), both −11; otherwise the menu bar (`0x7615d0`, otherwise
/// `0x7643d0`). The item goes whole (32 bits) to the bar, as in the original.
/// Original: `handle_day_fixtures_input` (0x718930, match_screens.cpp, cdecl).
///
/// [pt] { 0x718930 } (cdecl, match_screens.cpp): a resposta da tela dos jogos do dia ao item `item` (−1 =
/// nenhum). Com item: o parâmetro 2 recebe 0x803320 do parâmetro 3. Pelo id do item (0 vira o padrão
/// [0xdd6bde]): 1 abre outro grupo de jogos (0x717cc0 na vaga [0xb74c26] do humano da vez com o dado
/// +0xba9a6 do item) e 2 o andamento da competição (0x49b830 com o dado), ambos −11; senão a barra de
/// menu (0x7615d0, senão 0x7643d0). O item vai inteiro (32 bits) para a barra, como na original.
pub(crate) unsafe extern "C" fn handle_day_fixtures_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond day matches");
    let i = item as i16;
    let id = |i: i16| if i == -1 { *(G_CURRENT_COMMAND_ID as *const u16) } else { display_item_id(i) };
    let given = |i: i16| {
        if i == -1 {
            return 0usize;
        }
        let display = *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
        core::ptr::read_unaligned((display as isize + i as isize * 0x18c + 0xba9a6) as *const usize)
    };
    if i != -1 {
        set_screen_param(2, exe::screenmanager_get_area_scroll_row(SCREEN_MANAGER, get_screen_param(3)));
    }
    match id(i) {
        1 => {
            let slot = *((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const u32);
            exe::open_day_fixtures_screen(slot, given(i));
            -11
        }
        2 => {
            exe::comp_open_league_progress_for_fixture(given(i));
            -11
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 {
                return r;
            }
            exe::menubar_handle_object_link(item)
        }
    }
}

/// Returns the starting-eleven slot (0..10) of tactic `tatica` with the highest staff rating (the default captain), or
/// −1 (`0xff`) for none.
///
/// `0x8968f0` on `[0xde1d08]` with `contexto`, rating `0x6e1cf0(pessoa, 0, tatica)`, i16 comparison; on a tie the first
/// one stays. Null tactic: the original routine runs (error box). Callers only use `al`.
/// Original: `match_screens_find_best_rated_starter_slot` (0x727e20, match_screens.cpp, cdecl).
///
/// [pt] { 0x727e20 } (cdecl, match_screens.cpp): a vaga (0..10) dos onze da tática `tatica` com a maior nota
/// da comissão (0x8968f0 em [0xde1d08] com `contexto`, nota 0x6e1cf0(pessoa, 0, tatica), comparação i16;
/// no empate fica a primeira), o capitão padrão; −1 (0xff) sem ninguém. Tática nula: a original (caixa
/// de erro). Quem chama só usa `al`.
pub(crate) unsafe extern "C" fn match_screens_find_best_rated_starter_slot(tactic: usize, contexto: i32) -> u8 {
    if tactic == 0 {
        return original_fn::<unsafe extern "C" fn(usize, i32) -> u8>(IDX_MATCH_SCREENS_FIND_BEST_RATED_STARTER_SLOT)(tactic, contexto);
    }
    let mut slot = 0xffu8;
    let mut best = 0i16;
    for k in 0..11u32 {
        let p = exe::tactics_get_selected_player(*(G_TACTICS as *const usize), tactic, k, contexto);
        if p != 0 {
            let n = exe::match_man_get_captain_score(p, 0, tactic);
            if slot == 0xff || n > best {
                slot = k as u8;
                best = n;
            }
        }
    }
    slot
}

// simplification: opening and replying touch screens and the menu (they do not run twice): in comparison mode only the
// original runs; the copies are proved in Lucas’s game in Rust mode (”telas ...” counters in ddraw.log)

/// Regions of the current screen's parameter block and the marks at 0xb6b7a8.
///
/// [pt] O bloco de parâmetros da tela atual ([gerente + 0x7e2 + 0x300 x vaga]; o parâmetro k em +0x14 + 8k,
/// a marca de "liberar ao trocar" em +0x18 + 8k, 60 parâmetros) e as marcas 0xb6b7a8/ac/b0.
pub(crate) unsafe fn screen_param_regions() -> Vec<(usize, usize)> {
    let slot = *((SCREEN_MANAGER + 0x382a) as *const i16) as isize;
    let block = core::ptr::read_unaligned((SCREEN_MANAGER as isize + 0x7e2 + slot * 0x300) as *const usize);
    let mut r = vec![(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 12)];
    if block != 0 {
        r.push((block, 0x200));
    }
    r
}

/// Id of item in the current human's display list (0 becomes the default [0xdd6bde]).
///
/// [pt] O id do item `item` na lista de exibição do humano da vez (0 vira o padrão [0xdd6bde]).
pub(crate) unsafe fn display_item_id(item: i16) -> u16 {
    let display = *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
    let w = core::ptr::read_unaligned((display as isize + item as isize * 0x18c + 0xba966) as *const u16);
    if w == 0 { *(G_CURRENT_COMMAND_ID as *const u16) } else { w }
}

/// Opens the live match screen for match `partida` in slot `vaga`, writes its params and marks the current human as a
/// spectator.
///
/// `0x7fe4a0` with the builder `0x718e90`, the handler `0x71a240` and the close handler `0x71b120`; it writes the
/// params (`0x7ff130`; param `0xe` is `[partida+0x63]`); marks the current human as a spectator
/// (`[[partida+0x5b]+0xc5+4h]`, counting in `+0x108`) and, with no confirmation (`[partida+0x57]`), confirms over the
/// network (`0x6b5260`) and, if confirmed, writes 2 in param 1. Null slot or match: the original routine runs (error
/// box).
/// Original: `open_match_screen` (0x718aa0, match_screens.cpp, cdecl).
///
/// [pt] { 0x718aa0 } (cdecl, match_screens.cpp): abre a tela da partida `partida` na vaga `vaga` (0x7fe4a0
/// com a montagem 0x718e90, a resposta 0x71a240 e o fechamento 0x71b120) e grava os parâmetros dela
/// (0x7ff130; o 0xe é [partida+0x63]); marca o humano da vez como espectador ([[partida+0x5b]+0xc5+4h],
/// contando em +0x108) e, sem confirmação ([partida+0x57]), confirma pela rede (0x6b5260) e, se
/// confirmou, grava 2 no parâmetro 1. Vaga ou partida nula: a original (caixa de erro).
pub(crate) unsafe extern "C" fn open_match_screen(slot: u32, partida: usize) {
    if slot == 0 || partida == 0 {
        return original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_MATCH_SCREEN)(slot, partida);
    }
    crate::comparison::branch("screens open the match");
    if exe::screenmanager_open_screen_for_each_record(SCREEN_MANAGER, slot, 0x718e90, 0x71a240, 1, 0x71b120, 0) == 0 {
        return;
    }
    const M: usize = usize::MAX;
    let e = *((partida + live_match::DAY_ENTRY) as *const usize);
    for (k, v) in [
        (0u32, partida), (1, 1), (2, 1), (3, M), (4, M), (5, M), (6, M), (7, 0), (8, M), (9, M), (0xa, 3), (0xb, M), (0xc, 0), (0xd, M),
        (0xe, e), (0xf, 0), (0x10, M), (0x11, 1), (0x12, 0), (0x13, 1), (0x14, 0), (0x15, 0), (0x16, 0), (0x17, 0), (0x18, 0), (0x19, 0),
        (0x1a, M), (0x1b, 0), (0x1c, 1), (0x1d, 0), (0x21, M), (0x22, 0), (0x23, M), (0x24, 0), (0x25, 0), (0x26, 0), (0x27, 0),
        (0x28, 0), (0x29, 0), (0x2d, 0), (0x2f, 0), (0x30, 0),
    ] {
        exe::screenmanager_set_screen_param_for_each_record(SCREEN_MANAGER, slot, k, v, 0);
    }
    let espectadores = *((partida + live_match::SPECTATORS) as *const usize);
    let mark = (espectadores as isize + 0xc5 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as *mut u32;
    if *mark == 0 {
        *mark = 1;
        let n = (espectadores + match_spectators::COUNT) as *mut u8;
        *n = (*n).wrapping_add(1);
    }
    if *((partida + live_match::ENGINE) as *const u32) == 0 {
        exe::match_day_start_watched_match(*(G_MATCH_DAY as *const usize), partida);
        if *((partida + live_match::ENGINE) as *const u32) != 0 {
            exe::screenmanager_set_screen_param(SCREEN_MANAGER, 1, 2, 0);
        }
    }
}

/// Handles the latest scores panel of the match screen (item `item`): the page switch every 5 s and the clicks
/// (competition, pages, pause). Returns 0.
///
/// Every 5 s (param `0x14` + 5000 ms, `0x90c510`) it moves to the next page (param `0x11`, from 1 up to `0x13`, unless
/// paused by `0x12`) and, without `[0xb6b7b0]`, redraws the panel (`0x724520`) and sets `[0xb6b7a8]`. Items: 5 changes
/// the competition (`0xe`) and goes back to page 1, 6 and 8 the previous and the next page (in a circle), 7 stops or
/// releases the automatic switch; each one sets `[0xb6b7b0]`.
/// Original: `match_screens_handle_latest_scores_cycle` (0x725860, match_screens.cpp, cdecl).
///
/// [pt] { 0x725860 } (cdecl, match_screens.cpp): a resposta do painel dos últimos placares ao item `item`.
/// A cada 5 s (parâmetro 0x14 + 5000 ms, 0x90c510) passa à página seguinte (parâmetro 0x11, de 1 até
/// o 0x13, se não estiver parado pelo 0x12) e, sem [0xb6b7b0], redesenha o painel (0x724520) e liga
/// [0xb6b7a8]. Itens: 5 troca a competição (0xe) e volta à página 1, 6 e 8 a página anterior e a
/// seguinte (em círculo), 7 para ou solta a troca automática; cada um liga [0xb6b7b0]. Devolve 0.
pub(crate) unsafe extern "C" fn match_screens_handle_latest_scores_cycle(item: u32) -> i32 {
    crate::comparison::branch("screens respond scores panel");
    let pagina = get_screen_param(0x11) as u8 as i8;
    let parado = get_screen_param(0x12);
    let paginas = get_screen_param(0x13) as u8 as i8;
    let quando = get_screen_param(0x14) as u32;
    let next = if pagina == paginas { 1 } else { (pagina as i32 + 1) as usize };
    if exe::utils_get_time_ms() >= quando.wrapping_add(5000) {
        if paginas > 1 && parado == 0 {
            set_screen_param(0x11, next);
        }
        if *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *const u32) == 0 {
            exe::refresh_latest_scores_rows();
            *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 1;
        }
    }
    let item16 = item as u16 as i16;
    if item16 == -1 {
        return 0;
    }
    let b0 = G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32;
    if display_item_id(item16) == 5 {
        let display = *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
        let given = core::ptr::read_unaligned((display as isize + item16 as isize * 0x18c + 0xba9a6) as *const usize);
        set_screen_param(0xe, given);
        set_screen_param(0x11, 1);
        *b0 = 1;
    }
    if display_item_id(item16) == 6 {
        // the page read at the start (the original does not reread after item 5)
        set_screen_param(0x11, if pagina == 1 { paginas as i32 as usize } else { (pagina as i32 - 1) as usize });
        *b0 = 1;
    }
    if display_item_id(item16) == 7 {
        set_screen_param(0x12, (parado == 0) as usize);
        *b0 = 1;
    }
    if display_item_id(item16) == 8 {
        set_screen_param(0x11, next);
        *b0 = 1;
    }
    0
}

/// Handles the team stats panel (item `item`): the redraw with new data and the clicks that move the row up and down.
/// Returns 0.
///
/// With new data (`[0xb6b7ac]`) and flag 4 of the match menu (`[[param 0]+0x5b]`), without `[0xb6b7b0]`, it redraws the
/// panel (`0x721cd0` with 0 and −1) and sets `[0xb6b7a8]`; items 9 and `0xa` move the row of param `0x1c` down and up
/// and set `[0xb6b7b0]`.
/// Original: `handle_team_stats_input` (0x722dc0, match_screens.cpp, cdecl).
///
/// [pt] { 0x722dc0 } (cdecl, match_screens.cpp): a resposta das estatísticas do time ao item `item`. Com
/// dados novos ([0xb6b7ac]) e a marca 4 do menu da partida ([[parâmetro 0]+0x5b]), sem [0xb6b7b0],
/// redesenha o painel (0x721cd0 com 0 e −1) e liga [0xb6b7a8]; os itens 9 e 0xa descem e sobem a
/// linha do parâmetro 0x1c e ligam [0xb6b7b0]. Devolve 0.
pub(crate) unsafe extern "C" fn handle_team_stats_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond team statistics");
    let p0 = get_screen_param(0);
    let row = get_screen_param(0x1c) as u8 as i8;
    if *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *const u32) != 0 && *(*((p0 + live_match::SPECTATORS) as *const usize) as *const u8) & 4 != 0 && *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *const u32) == 0 {
        exe::fill_team_stats_rows(0, (-1) as i32 as usize);
        *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 1;
    }
    let item16 = item as u16 as i16;
    if item16 == -1 {
        return 0;
    }
    if display_item_id(item16) == 9 {
        set_screen_param(0x1c, (row as i32 - 1) as usize);
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
    }
    if display_item_id(item16) == 0xa {
        set_screen_param(0x1c, (row as i32 + 1) as usize);
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
    }
    0
}

// simplification: opening the screen touches screens and the network (does not run twice): in comparison mode original only

/// Reads parameter k of the current screen (screen manager `exe::screenmanager_get_screen_param`).
///
/// [pt] Lê e grava os parâmetros da tela atual do gerente de telas [0xb74450] (0x7ff010, 0x7ff270).
pub(crate) unsafe fn get_screen_param(k: u32) -> usize {
    exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, k)
}
/// Writes parameter k of the current screen (screen manager `exe::screenmanager_set_screen_param`).
pub(crate) unsafe fn set_screen_param(k: u32, v: usize) {
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, v, 0);
}

/// opens the details screen of a match and writes its parameters
/// Original: `open_match_details` (0x71b4a0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b4a0 } (cdecl, match_screens.cpp): abre a tela de detalhes da partida `partida` (0x7fe600 com a
/// montagem 0x71b630, a resposta 0x71bdc0 e o fechamento 0x71c170) e grava os parâmetros dela (a
/// partida em 0x1e; a aba 2 começa em 6).
pub(crate) unsafe extern "C" fn open_match_details(partida: usize) {
    crate::comparison::branch("screens open the details");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x71b630, 0x71bdc0, 0, 0x71c170, 0) == 0 {
        return;
    }
    const MENOS1: usize = usize::MAX;
    for (k, v) in [
        (0u32, 0usize), (2, 6), (9, MENOS1), (0xa, 3), (0xb, MENOS1), (0xc, 0), (0xd, MENOS1), (0x16, 0), (0x17, 0), (0x18, 0), (0x19, 0),
        (0x1a, MENOS1), (0x1b, 0), (0x1c, 1), (0x1d, 0), (0x1e, partida), (0x1f, 0), (0x20, 0), (0x21, MENOS1), (0x22, 0), (0x23, MENOS1), (0x2d, 0),
    ] {
        set_screen_param(k, v);
    }
}

/// The match details screen's response to item `item` (−1 none): tab switch, club tactics, tab redraw, menu.
///
/// Without `[0xb77c84]`: clears `0xb6b7a8`/ac/b0; an item with id 1 (or `[0xdd6bde]`) changes the tab (param 2) to the
/// item's data (`+0x48` of the item in the current human's `Display`) and asks for −11; id 2 opens the tactics of the
/// item's club (`0x899a50`) and returns −4. Then, by tab: 1 possession (`0x71d9e0`), 2 statistics (`0x71e750`), 3
/// action zones (`0x720320`; and an item with id 4 changes the action of param `0xa`), 4 report, 5 and 7 the team
/// statistics (`0x722dc0`), 6 the ratings (`0x723350`); the redraws only with flag 4 of the match menu (`[[param
/// 0]+0x5b]`) and `[0xb6b7ac]`. Finally −11, the redraw (``exe::screenmanager_redraw_screen``) and the menu bar.
/// Original: `match_details_input` (0x71bdc0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71bdc0 } (cdecl, match_screens.cpp): a resposta da tela de detalhes ao item `item` (−1 nenhum).
/// Sem [0xb77c84]: zera 0xb6b7a8/ac/b0; um item de id 1 (ou [0xdd6bde]) troca a aba (parâmetro 2) pelo
/// dado do item (+0x48 do item no `Display` do humano da vez) e pede −11; de id 2 abre a tática do clube
/// do item (0x899a50) e devolve −4. Depois, pela aba: 1 posse (0x71d9e0), 2 estatísticas (0x71e750), 3
/// zonas de ação (0x720320; e um item de id 4 troca a ação do parâmetro 0xa), 4 relatório, 5 e 7 as
/// estatísticas do time (0x722dc0), 6 as notas (0x723350); os redesenhos só com a marca 4 do menu da
/// partida ([[parâmetro 0]+0x5b]) e [0xb6b7ac]. Por fim −11, o redesenho (0x802d00) e a barra de menu.
pub(crate) unsafe extern "C" fn match_details_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond details");
    let g = |a: usize| *(a as *const u32);
    let item16 = item as u16 as i16;
    let current_human_display = || *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
    let id_ou_default = |w: u16| if w == 0 { *(G_CURRENT_COMMAND_ID as *const u16) } else { w };
    if g(0xb77c84) == 0 {
        let aba = get_screen_param(2) as u8 as i8;
        let v1e = get_screen_param(0x1e);
        let p1f = Fixture(get_screen_param(0x1f));
        *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 0;
        if item16 != -1 {
            let deslocamento = item16 as isize * 0x18c;
            let record = |d: usize| (d as isize + deslocamento) as usize;
            let e = record(current_human_display());
            if id_ou_default(core::ptr::read_unaligned((e + 0xba966) as *const u16)) == 1 {
                let given = core::ptr::read_unaligned((e + 0xba9a6) as *const usize);
                if given != get_screen_param(2) {
                    set_screen_param(2, given);
                    *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                }
            }
            let e = record(current_human_display());
            if id_ou_default(core::ptr::read_unaligned((e + 0xba966) as *const u16)) == 2 {
                let club = core::ptr::read_unaligned((e + 0xba9a6) as *const usize);
                let mark = if club == p1f.home_club() { p1f.home_side_mark() } else { (p1f.away_side_mark() as u8) };
                exe::tactics_screen_open(club, (mark == 0) as u32, (0) as usize, 0, (v1e) as i32, 0);
                return -4;
            }
        }
        let menu_da_partida = |bit: u8| {
            let p = get_screen_param(0);
            g(G_MATCH_SCREEN_LIVE_DATA_FLAG) != 0 && *(*((p + live_match::SPECTATORS) as *const usize) as *const u8) & bit != 0
        };
        let redo = |f: usize| -> bool {
            // the tab redraws: with flag 4; with [0xb6b7b0] on the answer is −11
            if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
                return true;
            }
            let r: unsafe extern "C" fn() = core::mem::transmute(f);
            r();
            *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 1;
            false
        };
        match aba {
            1 => {
                let p = get_screen_param(0);
                if g(G_MATCH_SCREEN_LIVE_DATA_FLAG) != 0 {
                    let m = *(*((p + live_match::SPECTATORS) as *const usize) as *const u8);
                    if m & 0x20 != 0 || m & 0x10 != 0 {
                        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                    }
                    if m & 4 != 0 && redo(0x71d9e0) {
                        return -11;
                    }
                }
            }
            2 => {
                if menu_da_partida(4) && redo(0x71e750) {
                    return -11;
                }
            }
            3 => {
                if menu_da_partida(4) && g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) == 0 {
                    redo(0x720320);
                }
                if item16 != -1 {
                    let d = current_human_display();
                    if id_ou_default(exe::display_get_widget_id(d, item)) == 4 {
                        let given: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_DISPLAY_GET_WIDGET_DATA);
                        if given(d, item) != get_screen_param(0xa) {
                            set_screen_param(0xa, exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, item));
                            *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                            return -11;
                        }
                    }
                }
            }
            4 => {
                if menu_da_partida(0x20) {
                    *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                    return -11;
                }
            }
            5 | 7 => {
                let r = exe::handle_team_stats_input(item);
                if r != 0 {
                    return r;
                }
            }
            6 => {
                if menu_da_partida(4) && redo(0x723350) {
                    return -11;
                }
            }
            _ => {}
        }
        if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
            return -11;
        }
        if g(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG) != 0 {
            exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        }
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(item)
}

/// closes the details screen: destroys and frees the objects kept in the parameters
/// Original: `close_match_details` (0x71c170, match_screens.cpp, cdecl).
///
/// [pt] { 0x71c170 } (cdecl, match_screens.cpp): fecha a tela de detalhes: os parâmetros 0x2d (0x5a8f60),
/// 0x1d e 0x20 (0x453bc0) são destruídos e liberados, o 0x1f só liberado (0x944c94), e todos zerados.
pub(crate) unsafe extern "C" fn close_match_details() {
    crate::comparison::branch("screens close the details");
    for (k, destrutor) in [(0x2du32, Some(0x5a8f60usize)), (0x1d, Some(0x453bc0)), (0x1f, None), (0x20, Some(0x453bc0))] {
        if get_screen_param(k) == 0 {
            continue;
        }
        let p = get_screen_param(k);
        match destrutor {
            Some(d) => {
                if p != 0 {
                    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(d);
                    destroy(p);
                    exe::crt_operator_delete(p);
                }
            }
            None => exe::crt_operator_delete(p),
        }
        set_screen_param(k, 0);
    }
}

// simplification: all three touch screens, menus and memory (they do not run twice): in comparison mode only the
// original runs; the copies are proved in Lucas’s game in Rust mode (”telas ...” counters in ddraw.log)

/// O gerente de telas (o objeto [0xb74450], que tem a vaga da tela atual em +0x382a e um `Display` por
/// vaga em +0x7fc + 0x300 x vaga).
pub(crate) const SCREEN_MANAGER: usize = G_SCREEN_MANAGER;

/// Regions building a screen touches: the screen manager (0x3840 bytes) and the current screen's Display (0x12f550
/// bytes).
///
/// [pt] As regiões que a montagem de uma tela mexe: o gerente de telas (0x3840 bytes) e o `Display` da tela
/// atual (a lista de exibição: itens de 0x18c bytes em +0xba95e, quantos em +0x12e9a0; 0x12f550 bytes).
pub(crate) unsafe fn screen_build_regions() -> Vec<(usize, usize)> {
    let slot = *((SCREEN_MANAGER + 0x382a) as *const i16);
    let mut r = vec![(SCREEN_MANAGER, 0x3840)];
    if (0..64).contains(&slot) {
        let display = core::ptr::read_unaligned((SCREEN_MANAGER + 0x7fc + slot as usize * 0x300) as *const usize);
        if display != 0 {
            r.push((display, 0x12f550));
        }
    }
    r
}

/// Opens the latest scores screen of competition `entrada` in slot `vaga` and writes its params.
///
/// `0x7fe4a0` with the builder `0x71b300` and the handler `0x71b3d0`; params (`0x7ff130`): 0 and `0xe` (the entry) and
/// `0xf..0x14` (0, −1, 1, 0, 1, 0). Null slot or entry: the original routine runs (error box).
/// Original: `open_latest_scores` (0x71b1b0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b1b0 } (cdecl, match_screens.cpp): abre a tela dos últimos placares da competição `entrada` na
/// vaga `vaga` (0x7fe4a0 com a montagem 0x71b300 e a resposta 0x71b3d0) e grava os parâmetros dela
/// (0x7ff130): 0 e 0xe (a entrada) e 0xf..0x14 (0, −1, 1, 0, 1, 0). Vaga ou entrada nula: a original
/// (caixa de erro).
pub(crate) unsafe extern "C" fn open_latest_scores(slot: u32, entry: usize) {
    if slot == 0 || entry == 0 {
        return original_fn::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_LATEST_SCORES)(slot, entry);
    }
    crate::comparison::branch("screens open the latest scores");
    if exe::screenmanager_open_screen_for_each_record(SCREEN_MANAGER, slot, 0x71b300, 0x71b3d0, 0, 0, 0) == 0 {
        return;
    }
    for (k, v) in [(0u32, 0usize), (0xe, entry), (0xf, 0), (0x10, usize::MAX), (0x11, 1), (0x12, 0), (0x13, 1), (0x14, 0)] {
        exe::screenmanager_set_screen_param_for_each_record(SCREEN_MANAGER, slot, k, v, 0);
    }
}

/// Builds the latest scores screen: background, title, menu, panel and back and next buttons.
///
/// The competition background (107-byte record in `[0xae23d0]` by the id in param `0xe`, `0x41c410`), the title "Latest
/// Scores" (`0x66f4e0` into `0xde1f64`, colours `0x839590`, `0x803f30`), the menu bar (`0x75db00` with 4), the scores
/// panel (`0x723d20`, from `0x50` to `0x217`) and the back and forward buttons (`0x5edcc0`, disabled when the manager
/// has nowhere to go).
/// Original: `build_latest_scores` (0x71b300, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b300 } (cdecl, match_screens.cpp): monta a tela dos últimos placares: o fundo da competição
/// (registro de 107 bytes em [0xae23d0] pelo id do parâmetro 0xe, 0x41c410), o título "Latest Scores"
/// (0x66f4e0 em 0xde1f64, cores 0x839590, 0x803f30), a barra de menu (0x75db00 com 4), o painel dos
/// placares (0x723d20, de 0x50 a 0x217) e os botões de voltar e avançar (0x5edcc0, desligados quando o
/// gerente não tem para onde ir).
pub(crate) unsafe extern "C" fn build_latest_scores() {
    crate::comparison::branch("screens build the latest scores");
    let entry = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0xe);
    let id = *(entry as *const i32);
    exe::background_set_comp_picture((*(G_CLUB_COMP_TABLE as *const usize) as isize + id as isize * 107) as usize, 0, 0);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(0xde1f64, 0xa16328, 0xad9c64);
    let (c1, c0) = ((exe::get_heading_colour(1) as u32), (exe::get_heading_colour(0) as u32));
    exe::screenmanager_add_screen_title(SCREEN_MANAGER, c0, c1, 0xde1f64);
    exe::menubar_build(4, 0);
    let (mut topo, mut base) = (0x50i32, 0x217i32);
    exe::draw_latest_scores_panel(&mut topo, &mut base);
    let advance = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
    let go_back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(go_back, advance);
}

/// The latest scores screen's response to item `item`: competition switch, redraw, menu.
///
/// Without `[0xb77c84]`: clears `0xb6b7a8`/ac/b0; outside mode 2 of the match (`[[0xb67a50]+0x34]+0x67`) and with the
/// current human manager connected (`0x5fd210`), it stamps the time (`0x5fdd30` with `0x90c510`); then the competition
/// switch (`0x725860`) decides; with no decision, −11 with `[0xb6b7b0]` or a redraw with `[0xb6b7a8]` (`0x802d00`).
/// Finally the menu bar (`0x7615d0`, otherwise `0x7643d0`).
/// Original: `latest_scores_input` (0x71b3d0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b3d0 } (cdecl, match_screens.cpp): a resposta da tela dos últimos placares ao item `item`.
/// Sem [0xb77c84]: zera 0xb6b7a8/ac/b0; fora do modo 2 da partida ([[0xb67a50]+0x34]+0x67) e com o
/// técnico humano da vez conectado (0x5fd210), marca a hora (0x5fdd30 com 0x90c510); depois a troca de
/// competição (0x725860) decide; sem decisão, −11 com [0xb6b7b0] ou redesenha com [0xb6b7a8]
/// (0x802d00). Por fim a barra de menu (0x7615d0, senão 0x7643d0).
pub(crate) unsafe extern "C" fn latest_scores_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond latest scores");
    let g = |a: usize| *(a as *const u32);
    if g(0xb77c84) == 0 {
        let m = g(G_MATCH_DAY) as usize;
        *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 0;
        if *((*((m + match_day_fields::HUMAN_MATCH) as *const usize) + 0x67) as *const u8) != 2 {
            let human = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
            if *((0xb74c26 + human * 0x300) as *const u32) != 0 {
                let tecnicos = g(G_HUMAN_MANAGER) as usize;
                let conectado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
                if conectado(tecnicos, 0) != 0 {
                    exe::human_manager_set_holiday_stamp(tecnicos, exe::utils_get_time_ms(), 0);
                }
            }
        }
        let r = exe::match_screens_handle_latest_scores_cycle(item);
        if r != 0 {
            return r;
        }
        if g(G_MATCH_SCREEN_VIEW_CHANGED_FLAG) != 0 {
            return -11;
        }
        if g(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG) != 0 {
            exe::screenmanager_redraw_screen(SCREEN_MANAGER, 0, 0);
        }
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(item)
}

// simplification: opening and answering touch screens, menu and time (they do not run twice): in comparison mode only the
// original runs; the copies are proved in Lucas’s game in Rust mode (”telas ...” counters in ddraw.log)

/// Writes the match control button label for state `estado` (1 waiting, 2 kick off, 3 restart, 4 pause/resume, 5
/// continue or wait for the players) into `saida` and whether it is disabled. Returns 1.
///
/// Via `0x66f4e0` (the text goes through the language), with flags `0x1021` (plus `0x40` when the button is disabled).
/// With no match, no output, fewer than 100 bytes or the state out of range: the original routine runs (error box).
/// Original: `match_control_label` (0x725a70, match_screens.cpp, cdecl).
///
/// [pt] { 0x725a70 } (cdecl, match_screens.cpp): o rótulo do botão de controle da partida para o estado
/// `estado` (1 esperando, 2 começar, 3 recomeçar, 4 pausar/retomar, 5 continuar ou esperar os
/// jogadores) em `saida` pela 0x66f4e0 (o texto passa pelo idioma), com as marcas 0x1021 (mais 0x40
/// quando o botão fica desligado). Sem partida, sem saída, com menos de 100 bytes ou estado fora: a
/// original (caixa de erro). Devolve 1.
pub(crate) unsafe extern "C" fn match_control_label(state: u8, m: usize, out: *mut u8, size: i32, marks: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u8, usize, *mut u8, i32, *mut u32) -> u32>(IDX_MATCH_CONTROL_LABEL);
    if m == 0 || out.is_null() || size < 100 || !(1..=5).contains(&state) {
        return o(state, m, out, size, marks);
    }
    let text: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    *out = 0;
    *marks = 0x1021;
    let human = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let (formato, desligado) = match state {
        1 => (0xa16d8c, false),
        2 => (0xa16d7c, false),
        3 => (0xa16d6c, false),
        4 => {
            if core::ptr::read_unaligned((m as isize + 0xc5 + 4 * human) as *const u32) != 0 {
                (0xa16d5c, false)
            } else if *((m + match_spectators::COUNT) as *const u8) != 0 {
                (0xa16d54, true)
            } else {
                (0xa16d48, false)
            }
        }
        _ => {
            let player = *((0xb74c26 + human * 0x300) as *const u32);
            if exe::screenmanager_is_manager_done(G_SCREEN_MANAGER, player) != 0 { (0xa16d34, true) } else { (0xa16d24, false) }
        }
    };
    crate::comparison::branch("screens control label");
    text(out, formato);
    if desligado {
        *marks |= 0x40;
    }
    1
}

/// closes the match screen: destroys and frees screen manager parameters 0x2d and 0x1d
/// Original: `close_match_screen` (0x71b120, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b120 } (cdecl, match_screens.cpp): fecha a tela da partida: os parâmetros 0x2d e 0x1d do
/// gerente de telas [0xb74450] (0x7ff010), se houver, são destruídos (0x5a8f60 e 0x453bc0), liberados
/// (0x944c94) e zerados (0x7ff270).
pub(crate) unsafe extern "C" fn close_match_screen() {
    crate::comparison::branch("screens close the match");
    for (k, destrutor) in [(0x2du32, 0x5a8f60usize), (0x1d, 0x453bc0)] {
        if exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, k) == 0 {
            continue;
        }
        let p = exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, k);
        if p != 0 {
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
            destroy(p);
            exe::crt_operator_delete(p);
        }
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, 0, 0);
    }
}

/// opens the human manager's waiting screen in the screen manager
/// Original: `open_waiting_screen` (0x727f00, match_screens.cpp, cdecl).
///
/// [pt] { 0x727f00 } (cdecl, match_screens.cpp): abre a tela de espera na vaga `vaga` do gerente de telas
/// (0x7fe4a0 com o desenho 0x727fa0 e a resposta 0x727fb0). Vaga 0: a original (caixa de erro).
pub(crate) unsafe extern "C" fn open_waiting_screen(slot: u32) {
    if slot == 0 {
        return original_fn::<unsafe extern "C" fn(u32)>(IDX_OPEN_WAITING_SCREEN)(slot);
    }
    crate::comparison::branch("screens open the wait");
    exe::screenmanager_open_screen_for_each_record(G_SCREEN_MANAGER, slot, 0x727fa0, 0x727fb0, 0, 0, 0);
}

// simplification: closing and opening touch the screen manager and memory (they do not run twice): in the
// comparison only the original runs; the copies are proven in Lucas’s game in Rust mode (JOGAR.bat), with
// the ”telas ...” counters in ddraw.log

/// Draws the latest scores panel from `*topo` to `*base`: title, badge, the day's groups and matches, table and
/// buttons.
///
/// Title (inside the match, param 0 ≠ 0: "<competition> Latest" at left `0x6e` and width `0x30c`, `*topo` += `0x28`; on
/// the stand-alone screen the name at `0x64`/`0x316`, += `0x41`) and the competition badge (`0x804030`, outside
/// friendlies `0x5c2990` and national teams `0x7e6f20`). With more than one round on the day (`[[0xb67a50]+0x2a]` > 1)
/// the list of the day's groups (`[+1]`, `0x18` bytes, `[+9]` how many) with the matches of each (`[+0xd]`, 84 bytes,
/// from `+4` to `+8` of the group): group header (World Cup, International, Continental or the nation's name) and one
/// row per match (`0x55f990`).
///
/// Then the 4-column table with the rows that fit (`0x5e5cd0`, the height of font 3; how many in param `0xf`, the first
/// item in `0x10`), the three buttons at the top (`0x804030`, 6 to 8) and the scores redraw (`0x724520`).
/// Original: `draw_latest_scores_panel` (0x723d20, match_screens.cpp, cdecl).
///
/// [pt] { 0x723d20 } (cdecl, match_screens.cpp): desenha o painel dos últimos placares a partir de `*topo`
/// até `*base`. Título (dentro da partida, parâmetro 0 ≠ 0: "<competição> Latest" à esquerda 0x6e e
/// largura 0x30c, `*topo` += 0x28; na tela avulsa o nome em 0x64/0x316, += 0x41) e o selo da competição
/// (0x804030, fora dos amistosos 0x5c2990 e das seleções 0x7e6f20). Com mais de uma rodada no dia
/// ([[0xb67a50]+0x2a] > 1) a lista dos grupos do dia ([+1], 0x18 bytes, [+9] quantos) com as partidas
/// de cada um ([+0xd], 84 bytes, de +4 a +8 do grupo): cabeçalho do grupo (Copa do Mundo, Internacional,
/// Continental ou o nome da nação) e uma linha por partida (0x55f990). Depois a tabela de 4 colunas
/// com as linhas que cabem (0x5e5cd0, a altura da fonte 3; quantas no parâmetro 0xf, o primeiro item no 0x10),
/// os três botões do alto (0x804030, 6 a 8) e o redesenho dos placares (0x724520).
pub(crate) unsafe extern "C" fn draw_latest_scores_panel(top: *mut i32, base: *mut i32) {
    crate::comparison::branch("screens latest scores panel");
    type T9 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T11 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T12 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T18 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let title: T11 = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: T12 = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let cell: T11 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let text_of_cell: T9 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let row: T18 = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let list: T11 = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let colour = |a: usize| *(a as *const u16) as usize;
    let display = || *((0xb74c4c + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize);
    let sm = SCREEN_MANAGER;
    const BUF: usize = 0xde1f64;
    let sem_selo = |c: usize| {
        exe::friendly_is_own_comp_record(*(G_FRIENDLY as *const usize), c) == 0 && exe::comp_is_reserve_or_minor_comp(*(G_RESERVE_COMP as *const usize), c) == 0
    };

    let partida = get_screen_param(0);
    let entry = get_screen_param(0xe);
    let comp = Competition((*(G_CLUB_COMP_TABLE as *const usize) as isize + *(entry as *const i32) as isize * 107) as usize);
    let t = *top as usize;
    let (esquerda, largura);
    if partida != 0 {
        esquerda = 0x6e;
        largura = 0x30c;
        let more_of_one = core::ptr::read_unaligned((entry + 8) as *const i32) > core::ptr::read_unaligned((entry + 4) as *const i32);
        let key = *(comp.addr() as *const usize);
        if key == *(KEY_CLUB_COMP_FRIENDLY as *const usize) && more_of_one {
            text(BUF, 0xa16b28, 0xad9c64);
        } else if key == *(KEY_CLUB_COMP_RESERVE as *const usize) && more_of_one {
            text(BUF, 0xa16aec, 0xad9c64);
        } else {
            let g = (comp.short_gender_name() as i8) as i32;
            text(BUF, 0xa16ac8, g, g, comp.addr() + competition::SHORT_NAME);
        }
        title(sm, 0x6e, t, 0x30c, t + 0x23, 2, 0, 0xc, 3, colour(G_GUI_COLOUR_YELLOW), BUF, 1);
        if sem_selo(comp.addr()) {
            caixa(sm, 0x9b, t, 0x276, t + 0x23, 1, 0, 0xc, 3, colour(G_GUI_COLOUR_YELLOW), 0, 0x7d0, comp.addr());
        }
        *top += 0x28;
    } else {
        esquerda = 0x64;
        largura = 0x316;
        panel_competition_name(comp, entry);
        title(sm, 0x64, t, 0x316, t + 0x2d, 2, 0, 0xc, 6, colour(G_GUI_COLOUR_YELLOW), BUF, 1);
        if sem_selo(comp.addr()) {
            caixa(sm, 0x91, t, 0x280, t + 0x2d, 1, 0, 0xc, 6, colour(G_GUI_COLOUR_YELLOW), 0, 0x7d0, comp.addr());
        }
        *top += 0x41;
    }

    let md = *(G_MATCH_DAY as *const usize);
    if *((md + match_day_fields::MARKED_COMPETITION_COUNT) as *const i8) > 1 {
        let cabeca = caixa(sm, esquerda + 5, t + 5, esquerda + 0x14, t + 0x19, 0x30, colour(G_GUI_COLOUR_BLACK), 0xc, 1, colour(G_GUI_COLOUR_PALE_GREY), 0, 0, 0);
        let h = list(display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour(0xae31aa), cabeca);
        let groups = core::ptr::read_unaligned((md + match_day_fields::GROUP_COUNT) as *const i32);
        for i in 0..groups.max(0) as usize {
            let g = core::ptr::read_unaligned((md + match_day_fields::GROUPS) as *const usize) + i * 0x18;
            if *((g + 0x14) as *const u32) == 0 {
                continue;
            }
            let parent = if *((md + match_day_fields::MARKED_GROUP_COUNT) as *const u8) == 1 {
                h
            } else {
                let kind = *(g as *const i32);
                match kind {
                    0 => {
                        text(BUF, 0xa16284);
                    }
                    1 => {
                        text(BUF, 0xa16248, 0xad9c64);
                    }
                    3..=7 => {
                        text(BUF, 0xa16a90, 0xad9c64);
                    }
                    _ => {
                        let nation = *((0xb63d60 + kind as isize * 72) as *const usize);
                        if nation != 0 {
                            copy_to_work_text(nation + 4);
                        } else {
                            text(BUF, 0xa16a90, 0xad9c64);
                        }
                    }
                }
                let r = row(display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour(0xae31aa), colour(G_GUI_COLOUR_YELLOW), 1, 1, colour(G_GUI_COLOUR_BLACK), BUF, 0, 0, 0, h);
                list(display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, colour(0xae31aa), r)
            };
            let mut k = *((g + 4) as *const i32);
            while k <= *((g + 8) as *const i32) {
                let m = (core::ptr::read_unaligned((md + match_day_fields::COMPETITIONS) as *const usize) as isize + k as isize * 84) as usize;
                if *((m + 0x10) as *const u32) != 0 {
                    let c = (*(G_CLUB_COMP_TABLE as *const usize) as isize + *(m as *const i32) as isize * 107) as usize;
                    panel_competition_name(Competition(c), m);
                    row(display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour(0xae31aa), colour(G_GUI_COLOUR_YELLOW), 1, 1, colour(G_GUI_COLOUR_BLACK), BUF, 0, 5, m, parent);
                }
                k += 1;
            }
        }
    }

    let a = exe::font_get_height(3);
    let n = ((*base - *top - 0x10) / a) as u8;
    set_screen_param(0xf, n as usize);
    let mut columns: [u8; 4] = [0x16, 4, 0x16, 4];
    // the original passes the row byte over the `base` pointer on the stack (the top 3 bytes remain)
    let rows = (base as usize & !0xff) | n as usize;
    let table = list(display(), 0x6e, *top as usize, 0x30c, *base as usize, 4, columns.as_mut_ptr() as usize, rows, 0, 2, 0, usize::MAX);
    for s in 0..n as usize {
        let r = cell(sm, table, 0, s, 1, 0, 1, 3, colour(0xaebde8), 0, 0, 0) as u16 as i16;
        if s == 0 {
            set_screen_param(0x10, r as i32 as usize);
        }
        for _ in 0..3 {
            text_of_cell(sm, table, 1, s, 1, 0, 0xc, 3, colour(G_GUI_COLOUR_YELLOW), 0);
        }
        cell(sm, table, 2, s, 1, 0, 1, 3, colour(0xaebde8), 0, 0, 0);
        text_of_cell(sm, table, 3, s, 1, 0, 0xc, 2, colour(G_GUI_COLOUR_PALE_GREY), 0);
    }
    let mark = if *(G_FONT_MODE as *const i32) != -1 { 0 } else { 1 };
    caixa(sm, largura - 0x7c, t + 5, largura - 0x68, t + 0x19, 1, colour(G_GUI_COLOUR_DARK_BLUE), 0xc, mark, colour(G_GUI_COLOUR_PALE_GREY), 0, 6, 0);
    caixa(sm, largura - 0x66, t + 5, largura - 0x1b, t + 0x19, 1, 0, 0xc, 2, colour(G_GUI_COLOUR_YELLOW), 0, 7, 0);
    caixa(sm, largura - 0x19, t + 5, largura - 5, t + 0x19, 1, colour(G_GUI_COLOUR_DARK_BLUE), 0xc, mark, colour(G_GUI_COLOUR_PALE_GREY), 0, 8, 0);
    exe::refresh_latest_scores_rows();
}

