// ---------- %%LEVA%%a leva: o fechamento do motor (4): as ordens das partidas e dos jogadores e a troca do evento ----------

/// { 0x6b6510 } (cdecl, qsort): a ordem das partidas do dia (registros de 0x69 bytes): pela
/// competição (+4), depois +0x46 (byte), +0x36, +0x38 e +0x34 (palavras) e, empatando tudo, pelo nome
/// do primeiro clube (+0x20 → +0x38) na comparação de textos 0x90c1d0 (sem maiúsculas).
unsafe extern "C" fn day_match_order(a: usize, b: usize) -> i32 {
    let d = core::ptr::read_unaligned((a + 4) as *const i32).wrapping_sub(core::ptr::read_unaligned((b + 4) as *const i32));
    if d != 0 {
        return d;
    }
    let d = *((a + 0x46) as *const i8) as i32 - *((b + 0x46) as *const i8) as i32;
    if d != 0 {
        return d;
    }
    for o in [0x36, 0x38, 0x34] {
        let d = core::ptr::read_unaligned((a + o) as *const i16) as i32 - core::ptr::read_unaligned((b + o) as *const i16) as i32;
        if d != 0 {
            return d;
        }
    }
    let textos: unsafe extern "C" fn(usize, usize, i32, u32) -> i32 = core::mem::transmute(0x90c1d0usize);
    let nome = |x: usize| core::ptr::read_unaligned((x + 0x20) as *const usize).wrapping_add(0x38);
    textos(nome(a), nome(b), 1, 0)
}

/// { 0x6bd840 } (cdecl, qsort, match_eng.cpp): a ordem dos jogadores na vez do motor ([0xb67a64]):
/// com +0x8eb2 ligado, o de +0xf582 vem primeiro; depois o de +0x91de; os demais pela força
/// (+0x104 + +0x105) × (+0x3b × 0,005 + 0,5) × +0x4d (f32), truncada, do maior para o menor.
unsafe extern "C" fn action_order(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    let m = *(0xb67a64 as *const usize);
    if *((m + 0x8eb2) as *const u8) != 0 {
        let primeiro = core::ptr::read_unaligned((m + 0xf582) as *const usize);
        if pa == primeiro {
            return -1;
        }
        if pb == primeiro {
            return 1;
        }
    }
    let c = core::ptr::read_unaligned((m + 0x91de) as *const usize);
    if c == pa {
        return -1;
    }
    if c == pb {
        return 1;
    }
    let forca = |p: usize| {
        let s = *((p + 0x105) as *const i8) as i32 + *((p + 0x104) as *const i8) as i32;
        let w = core::ptr::read_unaligned((p + 0x3b) as *const i16) as f64;
        match_pl_ftol(s as f64 * (w * read_f64_at(0x968ef0) + read_f64_at(0x968968)) * core::ptr::read_unaligned((p + 0x4d) as *const f32) as f64)
    };
    forca(pb).wrapping_sub(forca(pa))
}

/// { 0x6d5d80 } (stdcall, ret 0xc, match_events.cpp): troca o evento da narração por uma variante:
/// com sorteio(3) ≠ 0 fica o mesmo; senão, pelo tipo do lance `tipo` (1, 2, 8, 0x10, 0x20 — este com
/// outro sorteio(3)) há tabelas de troca; os lances 2..4 (byte baixo de `lados`) têm as suas, e o
/// byte de cima 3..5 troca 0x1f4b por 0x217e.
unsafe extern "system" fn event_variant(e: *const i16, lados: u32, tipo: u8) -> i16 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    if sorteio(3) != 0 {
        return *e;
    }
    let v = *e;
    let tabela: &[(i16, i16)] = match tipo {
        1 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fee, 0x218d), (0x1f49, 0x2182), (0x205a, 0x2190), (0x205b, 0x2191), (0x2079, 0x2192)],
        2 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fe6, 0x2193), (0x1fee, 0x218d), (0x1f48, 0x2181), (0x1f41, 0x2180), (0x1f44, 0x2187), (0x1f86, 0x2189), (0x1fa2, 0x218a)],
        8 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fe6, 0x2193), (0x1fee, 0x218d), (0x205a, 0x218e), (0x205b, 0x218f), (0x1f4b, 0x2183), (0x1f41, 0x2184), (0x1f4a, 0x2188)],
        0x10 => &[(0x1f48, 0x2185)],
        0x20 if sorteio(3) == 0 => &[(0x216f, 0x2199), (0x1fee, 0x218d), (0x1f42, 0x217c), (0x1f4b, 0x217d), (0x1f48, 0x2186)],
        _ => &[],
    };
    if let Some(&(_, n)) = tabela.iter().find(|&&(de, _)| de == v) {
        return n;
    }
    if matches!(lados as u8, 2..=4) {
        let lance: &[(i16, i16)] = &[(0x2112, 0x2196), (0x1fa6, 0x2194), (0x1f8b, 0x218b), (0x1fbf, 0x218c), (0x211f, 0x217f)];
        if let Some(&(_, n)) = lance.iter().find(|&&(de, _)| de == v) {
            return n;
        }
    }
    if matches!((lados >> 8) as u8, 3..=5) && v == 0x1f4b {
        return 0x217e;
    }
    v
}

unsafe extern "C" fn comparar_day_match_order(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = day_match_order(a, b);
    SCOREBOARD.record(r == n, || format!("day_match_order({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

unsafe extern "C" fn comparar_action_order(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(%%I1%%)(a, b);
    let n = action_order(a, b);
    SCOREBOARD.record(r == n, || format!("action_order({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

unsafe extern "system" fn comparar_event_variant(e: *const i16, lados: u32, tipo: u8) -> i16 {
    let o = original_fn::<unsafe extern "system" fn(*const i16, u32, u8) -> i16>(%%I2%%);
    if is_recording_top_level() {
        return o(e, lados, tipo);
    }
    let reg = crate::rng::STATE_REGIONS.to_vec();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(e, lados, tipo), || crate::rng::without_counting(|| event_variant(e, lados, tipo)));
    SCOREBOARD.record(igual && r == n, || format!("event_variant({:#x}, {:#x}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", *e, lados, tipo, r, n, mo, mr));
    r
}
