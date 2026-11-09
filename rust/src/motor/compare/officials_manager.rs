use super::*;

// The comparers of the routines in ../officials_manager.rs.
// [pt] Os comparadores das rotinas de ../officials_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `record_referee_grade`.
pub(crate) unsafe extern "thiscall" fn compare_record_referee_grade(this: usize, r: *const u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const u8)>(IDX_RECORD_REFEREE_GRADE);
    if is_recording_top_level() || r.is_null() {
        return o(this, r);
    }
    let reg = official_record_region(this, *(r as *const i32));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, r), || record_referee_grade(this, r));
    SCOREBOARD.record(igual, || format!("record_referee_grade(nota {}) {:x?} x {:x?}", *r.add(6), mo, mr));
}

/// Comparer for `set_up_officials`.
///
/// [pt] Comparador: só a original (simplification: aloca, libera e troca a tabela dos árbitros, e as chamadas sorteiam: não pode
/// rodar duas vezes), com o plano: os blocos por nação (this+0xc), a conta (this+4) igual à da tabela e as marcas (this+8).
pub(crate) unsafe extern "thiscall" fn compare_set_up_officials(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SET_UP_OFFICIALS)(this);
    SCOREBOARD.record(read_usize(this + officials_manager_fields::NATION_BLOCKS) != 0 && read_i32(this + officials_manager_fields::BUSY_COUNT) == read_i32(G_OFFICIAL_COUNT) && read_usize(this + officials_manager_fields::BUSY_FLAGS) != 0,
        || format!("set_up_officials: blocos {:x}, conta {} x {}", read_usize(this + officials_manager_fields::NATION_BLOCKS), read_i32(this + officials_manager_fields::BUSY_COUNT), read_i32(G_OFFICIAL_COUNT)));
}

/// Comparer for `officials_manager_compare_comp_level`.
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn compare_officials_manager_compare_comp_level(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_OFFICIALS_MANAGER_COMPARE_COMP_LEVEL)(a, b);
    let n = officials_manager_compare_comp_level(a, b);
    SCOREBOARD.record(r == n, || format!("officials_manager_compare_comp_level({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `officials_manager_compare_nation_then_quality_desc`.
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn compare_officials_manager_compare_nation_then_quality_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_OFFICIALS_MANAGER_COMPARE_NATION_THEN_QUALITY_DESC)(a, b);
    let n = officials_manager_compare_nation_then_quality_desc(a, b);
    SCOREBOARD.record(r == n, || format!("officials_manager_compare_nation_then_quality_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `officials_manager_compare_nation_then_quality_asc`.
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn compare_officials_manager_compare_nation_then_quality_asc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_OFFICIALS_MANAGER_COMPARE_NATION_THEN_QUALITY_ASC)(a, b);
    let n = officials_manager_compare_nation_then_quality_asc(a, b);
    SCOREBOARD.record(r == n, || format!("officials_manager_compare_nation_then_quality_asc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `choose_match_official`.
///
/// [pt] Comparador: lado a lado pela palavra de saída e pelo estado do sorteio, e pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_choose_match_official(this: usize, game: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CHOOSE_MATCH_OFFICIAL);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((out, 2));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, game, out),
        || crate::rng::without_counting(|| choose_match_official(this, game, out)));
    SCOREBOARD.record(igual && r == n, || format!("choose_match_official(jogo {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", game, r, mo, n, mr));
    r
}

/// Comparer for `clear_busy_flags`.
///
/// [pt] Comparador: lado a lado pelos bytes de ocupado e pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_clear_busy_flags(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CLEAR_BUSY_FLAGS);
    let n = read_i32(this + officials_manager_fields::BUSY_COUNT);
    let reg = if n > 0 && read_usize(this + officials_manager_fields::BUSY_FLAGS) != 0 { vec![(read_usize(this + officials_manager_fields::BUSY_FLAGS), n as usize)] } else { Vec::new() };
    let (r, m, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this), || clear_busy_flags(this));
    SCOREBOARD.record(igual && r == m, || format!("clear_busy_flags(conta {}) original={} rust={} memoria igual {}", n, r, m, igual));
    r
}

/// Comparer for `renew_officials`.
///
/// [pt] Comparador: lado a lado pelas fichas de todos os árbitros ([0xae2390] × 0x2b), pelas linhas de estatísticas
/// ([0xae2390] × 17), pelo estado do sorteio e por [0xb67a34], e pela devolução. Sem tabela de árbitros (a caixa a cada
/// ficha): só a original.
pub(crate) unsafe extern "thiscall" fn compare_renew_officials(this: usize, comp: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_RENEW_OFFICIALS);
    let total = read_i32(G_OFFICIAL_COUNT).max(0) as usize;
    if read_usize(G_OFFICIAL_TABLE) == 0 || read_usize(this) == 0 {
        return o(this, comp);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(read_usize(G_OFFICIAL_TABLE), total * 0x2b), (read_usize(this), total * 0x11), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, comp),
        || crate::rng::without_counting(|| renew_officials(this, comp)));
    SCOREBOARD.record(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("renew_officials(comp {:#x}) original={} rust={}, diferencas em {:x?}", comp, r, n, dif)
    });
    r
}

/// Comparer for `officials_manager_add_random_officials`.
///
/// The routine reallocates and sorts (it cannot run twice): only the original routine, after a dry run. The dry run
/// generates the copy's `conta` sheets (generate_official, without counting the random draws) in its own area from the
/// same random generator state, which is restored; then the original runs.
///
/// It checks the final random state, the new count ([0xae2390] + conta) and that each sheet from the dry run is in the
/// sorted table (fields the original writes). Non-positive count or nation without a continent: only the original.
///
/// [pt] Comparador: realoca e ordena (não pode rodar duas vezes): só a original, depois de um ensaio. O ensaio gera as
/// `conta` fichas da cópia (generate_official, sem contar os sorteios) numa área própria a partir do mesmo estado do sorteio,
/// que é reposto; depois roda a original. Confere o estado final do sorteio, a conta nova ([0xae2390] + conta) e que
/// cada ficha do ensaio esteja na tabela ordenada (campos que a original grava). Conta não positiva ou nação sem
/// continente: só a original.
pub(crate) unsafe extern "thiscall" fn compare_officials_manager_add_random_officials(this: usize, count: i32, nation: usize) {
    let nation = Nation(nation);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(IDX_OFFICIALS_MANAGER_ADD_RANDOM_OFFICIALS);
    if count <= 0 || (!nation.is_null() && nation.continent() == 0) {
        return o(this, count, nation.addr());
    }
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let mut novas = vec![[0u8; 0x2b]; count as usize];
    crate::rng::pause_trace(|| crate::rng::without_counting(|| {
        for g in novas.iter_mut() {
            generate_official(g.as_mut_ptr() as usize, nation);
        }
    }));
    let state_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    state.write_back();
    let before = read_i32(G_OFFICIAL_COUNT);
    o(this, count, nation.addr());
    let estado_orig = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let total = read_i32(G_OFFICIAL_COUNT);
    let t = read_usize(G_OFFICIAL_TABLE);
    let mut usada = vec![false; total.max(0) as usize];
    let mut missing = Vec::new();
    for (k, g) in novas.iter().enumerate() {
        match (0..usada.len()).find(|&j| !usada[j] && same_official_record(g, t + j * 0x2b)) {
            Some(j) => usada[j] = true,
            None => missing.push(k),
        }
    }
    let same = estado_orig == state_rust && total == before.wrapping_add(count) && missing.is_empty();
    SCOREBOARD.record(same, || format!("officials_manager_add_random_officials(conta {}, nacao {:#x}) total {} (antes {}), sorteio igual {}, fichas da copia ausentes {:?}",
        count, nation.addr(), total, before, estado_orig == state_rust, missing));
}

/// Comparer for `officials_manager_validate_officials`.
///
/// [pt] Comparador: lado a lado pelas fichas de todos os árbitros e pelo estado do sorteio.
pub(crate) unsafe extern "C" fn compare_officials_manager_validate_officials() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_OFFICIALS_MANAGER_VALIDATE_OFFICIALS);
    let total = read_i32(G_OFFICIAL_COUNT).max(0) as usize;
    if read_usize(G_OFFICIAL_TABLE) == 0 {
        return o();
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((read_usize(G_OFFICIAL_TABLE), total * 0x2b));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| officials_manager_validate_officials()));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("officials_manager_validate_officials ({} fichas) diferencas em {:x?}", total, dif)
    });
}

/// Comparer for `officials_manager_operator`.
///
/// [pt] Comparador: lado a lado pela ficha de destino e pela devolução.
pub(crate) unsafe extern "thiscall" fn compare_officials_manager_operator(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_OFFICIALS_MANAGER_OPERATOR);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x2b)], || o(this, src), || officials_manager_operator(this, src));
    SCOREBOARD.record(igual && r == n, || format!("officials_manager_operator(src {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", src, r, mo, n, mr));
    r
}

/// Comparer for `save_official_stats`.
///
/// [pt] Comparador da 0x79bb80: só a original.
pub(crate) unsafe extern "thiscall" fn compare_save_official_stats(this: usize, save: usize) -> u8 {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_SAVE_OFFICIAL_STATS)(this, save)
}
