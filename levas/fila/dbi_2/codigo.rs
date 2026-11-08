// ---------- %%LEVA%%a leva: as ordens dos clubes pela reputação e dos elencos pela habilidade (Database.cpp) ----------

/// { 0x540020 } (cdecl, Database.cpp): comparação do qsort do vetor de ids de clube (0x53db40): pela reputação
/// (+0x80, palavra com sinal) do clube ([0xae23b4] + id × 581; id −1 ou menor = reputação 0), a maior primeiro:
/// a de `a` menor que a de `b`: 1; maior: −1; iguais: 0.
unsafe extern "C" fn qsort_comparator_club_ids_by_reputation(a: usize, b: usize) -> i32 {
    let clubes = *(0xae23b4 as *const usize);
    let injury_word_0c = |p: usize| {
        let id = *(p as *const i32);
        if id > -1 { core::ptr::read_unaligned(clubes.wrapping_add((id as usize).wrapping_mul(581)).wrapping_add(0x80) as *const i16) } else { 0 }
    };
    let (ra, rb) = (injury_word_0c(a), injury_word_0c(b));
    if ra < rb {
        1 // a de menor reputação depois
    } else if ra > rb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn compare_qsort_comparator_club_ids_by_reputation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = qsort_comparator_club_ids_by_reputation(a, b);
    SCOREBOARD.record(r == n, || format!("qsort_comparator_club_ids_by_reputation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x5402f0 } (cdecl, Database.cpp): comparação do qsort dos elencos (corte do conjunto A+B em 0x541e60 e as
/// seleções da MLS em 0x90a980): ponteiros de pessoa; nula vai para o fim (a nula e b não: 1; a não e b nula: −1;
/// as duas: 0); depois pela palavra +0xb do jogador (pessoa +0x61; sem jogador, 0), a maior primeiro (b − a).
unsafe extern "C" fn qsort_comparator_squad_by_player_0xb(a: usize, b: usize) -> i32 {
    let pa = *(a as *const usize);
    let pb = *(b as *const usize);
    if pa == 0 {
        return (pb != 0) as i32;
    }
    if pb == 0 {
        return -1;
    }
    let habilidade = |p: usize| {
        let j = core::ptr::read_unaligned((p + 0x61) as *const usize);
        if j != 0 { core::ptr::read_unaligned((j + 0xb) as *const i16) } else { 0 }
    };
    habilidade(pb) as i32 - habilidade(pa) as i32 // a maior primeiro
}

unsafe extern "C" fn compare_qsort_comparator_squad_by_player_0xb(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = qsort_comparator_squad_by_player_0xb(a, b);
    SCOREBOARD.record(r == n, || format!("qsort_comparator_squad_by_player_0xb({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x549c30 } (cdecl, Database.cpp): comparação do qsort do elenco na força do time (club_squad_strength_band
/// 0x549a30): ponteiros de pessoa; sem pessoa ou sem jogador (+0x61) em `a`: 1 (mesmo com `b` igual); em `b`:
/// −1; depois pela habilidade atual (jogador +5, palavra com sinal) e, empatada, pela palavra +0xb, a maior
/// primeiro (b − a).
unsafe extern "C" fn compare_staff_by_player_ability_desc(a: usize, b: usize) -> i32 {
    let jogador = |p: usize| {
        let s = *(p as *const usize);
        if s == 0 { 0 } else { core::ptr::read_unaligned((s + 0x61) as *const usize) }
    };
    let ja = jogador(a);
    if ja == 0 {
        return 1;
    }
    let jb = jogador(b);
    if jb == 0 {
        return -1;
    }
    let palavra = |j: usize, d: usize| core::ptr::read_unaligned((j + d) as *const i16) as i32;
    if palavra(ja, 5) != palavra(jb, 5) {
        palavra(jb, 5) - palavra(ja, 5) // a habilidade atual maior primeiro
    } else {
        palavra(jb, 0xb) - palavra(ja, 0xb)
    }
}

unsafe extern "C" fn compare_compare_staff_by_player_ability_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_staff_by_player_ability_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_staff_by_player_ability_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
