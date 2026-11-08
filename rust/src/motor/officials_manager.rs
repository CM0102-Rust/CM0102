use super::*;

/// adds the grade to the referee's record (17 bytes at [this] + id × 17, id in 0..[0xae2390]): +0xf the cards, words +8
/// and +0xa the fouls and the grade, +0x10 the last grade, +0xc the matches and +4 the average (+0xa / +0xc, float).
/// Compared side by side (the record)
/// Original: `record_referee_grade` (0x79ce10, officials_manager.cpp (208 bytes), thiscall, ret 4).
///
/// [pt] { 0x79ce10 } (ret 4, officials_manager.cpp): soma a nota `r` (dword id, bytes: cartoes, faltas,
/// nota) na ficha do arbitro (17 bytes em [this] + id x 17, id em 0..[0xae2390]): +0xf os cartoes,
/// as palavras +8 e +0xa as faltas e a nota, +0x10 a ultima nota, +0xc os jogos e +4 a media
/// (+0xa / +0xc, float).
pub(crate) unsafe extern "thiscall" fn record_referee_grade(this: usize, r: *const u8) {
    let base = *(this as *const usize);
    if base == 0 || r.is_null() {
        return;
    }
    let id = *(r as *const i32);
    if id < 0 || id >= *(G_OFFICIAL_COUNT as *const i32) {
        return;
    }
    let f = base + id as usize * 17;
    let b = |o: usize| (f + o) as *mut u8;
    let w = |o: usize| (f + o) as *mut i16;
    *b(0xf) = (*b(0xf)).wrapping_add(*r.add(4));
    *w(8) = (*w(8)).wrapping_add(*r.add(5) as i8 as i16);
    *w(0xa) = (*w(0xa)).wrapping_add(*r.add(6) as i8 as i16);
    *b(0x10) = *r.add(6);
    *w(0xc) = (*w(0xc)).wrapping_add(1);
    if *w(0xc) != 0 {
        *((f + 4) as *mut f32) = (*w(0xa) as f64 / *w(0xc) as f64) as f32;
    }
}

/// Region of the official's 17-byte record that 0x79ce10 touches (empty if id invalid).
///
/// [pt] A ficha do arbitro que a 0x79ce10 mexe (vazia se o id nao vale).
pub(crate) unsafe fn official_record_region(manager: usize, id: i32) -> Vec<(usize, usize)> {
    let base = *(manager as *const usize);
    if base == 0 || id < 0 || id >= *(G_OFFICIAL_COUNT as *const i32) {
        return Vec::new();
    }
    vec![(base + id as usize * 17, 17)]
}

/// Record of official i (0x2b bytes in [0xae23e8]).
///
/// [pt] O registro do árbitro `i` (0x2b bytes em [0xae23e8], [0xae2390] deles).
pub(crate) unsafe fn official_record(i: i32) -> usize {
    read_u32_as_usize(G_OFFICIAL_TABLE).wrapping_add((i as isize * 0x2b) as usize)
}

/// Original's fatal out-of-memory end: box at officials_manager.cpp line, cleanup and exit(-1).
///
/// [pt] O fim fatal da original sem memória: a caixa (linha `linha` de officials_manager.cpp, 0xa6a22c), 0x5cc7d0, 0x5a8f60
/// e exit(−1) (0x945934).
pub(crate) unsafe fn officials_out_of_memory(row: u32) -> ! {
    show_error_at(0xa6a22c, row);
    let f1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let f2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    f1();
    f2();
    exit(-1)
}

/// Calls the game's qsort (0x945333) with the given comparison routine.
///
/// [pt] O qsort do jogo (0x945333) pela rotina de comparação `cmp`.
pub(crate) unsafe fn officials_manager_crt_qsort(base: usize, n: u32, size: u32, cmp: usize) {
    let q: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    q(base, n, size, cmp);
}

/// Calls the game's calloc (0x945501).
///
/// [pt] calloc (0x945501, cdecl).
pub(crate) unsafe fn crt_calloc(n: u32, size: u32) -> usize {
    let f: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    f(n, size)
}

/// Calls the game's free (0x9452ca).
///
/// [pt] free (0x9452ca, cdecl).
pub(crate) unsafe fn crt_free(p: usize) {
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    f(p);
}

/// Sets up the referees on a new game: reviews them, sorts them by nation and quality, keeps 6 per nation (61 in some
/// nations), builds the per-nation blocks with one slot per league, draws the missing referees and shares the nation's
/// referees among the slots.
///
/// `OfficialsManager::set_up_officials()`, from the constructor (0x79b580) on a new game. It reviews the records
/// (0x79caa0), sorts them by nation and quality (qsort 0x79d4e0) and, per nation, keeps the first 6 (nation with +0x11c
/// zero) or 61 (the first of each nation after the first one is not kept: the count restarts on it), copied (0x79c530)
/// into a new table; the per-nation blocks (this+0xc, 0x102 bytes: +0 the nation, +4 the referees, +8 the first, +0xc
/// the demand, +0x10 the competitions and 15 slots of 16 bytes at +0x12): each league (0x4b0700, not a friendly
/// 0x5c6710, with nation +0x5d) gets a slot with the demand (+0x3e of the competition) / 2 + 10; a nation with
/// competitions asks for the largest × the number and, if short, draws the missing ones (0x79c5c0); without
/// competitions and with fewer than 5, it draws up to the nation's +0x8e (1..5). The indexes (+0), the first of each
/// nation and, per nation, the slots sorted (qsort 0x79d410) with each one's share; this+4 the count and this+8 the
/// flags (calloc).
/// Original: `OfficialsManager::set_up_officials` (0x79bf60, officials_manager.cpp, thiscall).
///
/// [pt] { 0x79bf60 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::set_up_officials()`, pelo construtor
/// (0x79b580) no jogo novo. Revê os registros (0x79caa0), ordena por nação e pela qualidade (qsort 0x79d4e0) e, por
/// nação, fica com os 6 primeiros (nação com +0x11c zero) ou 61 (o primeiro de cada nação depois da primeira não fica:
/// a conta recomeça nele), copiados (0x79c530) numa tabela nova; os blocos por nação (this+0xc, 0x102 bytes: +0 a nação,
/// +4 os árbitros, +8 o primeiro, +0xc o pedido, +0x10 as competições e 15 vagas de 16 bytes em +0x12): cada liga
/// (0x4b0700, não amistosa 0x5c6710, com nação +0x5d) ganha uma vaga com o pedido (+0x3e da competição) / 2 + 10; a nação
/// com competições pede o maior × a quantidade e, faltando, sorteia os que faltam (0x79c5c0); sem competições e menos de
/// 5, sorteia até o +0x8e da nação (1..5). Os índices (+0), o primeiro de cada nação e, por nação, as vagas ordenadas
/// (qsort 0x79d410) com a parte de cada uma; this+4 a conta e this+8 as marcas (calloc).
pub(crate) unsafe extern "thiscall" fn set_up_officials(this: usize) {
    let rever: unsafe extern "C" fn() = core::mem::transmute(FN_OFFICIALS_MANAGER_VALIDATE_OFFICIALS);
    rever();
    officials_manager_crt_qsort(read_u32_as_usize(G_OFFICIAL_TABLE), read_i32(G_OFFICIAL_COUNT) as u32, 0x2b, 0x79d4e0);
    let n = read_i32(G_OFFICIAL_COUNT);
    let marks = crt_calloc(n as u32, 1);
    let mut ficam = 0i32;
    if n > 0 {
        let mut current = read_u32_as_usize(official_record(0) + 0x16);
        let mut group = 0i32;
        for i in 0..n {
            let nation = read_u32_as_usize(official_record(i) + 0x16);
            if nation != current {
                group = 0;
                current = nation;
                continue;
            }
            let m = (marks + i as usize) as *mut u8;
            if nation == 0 {
                *m = 0;
                continue;
            }
            let limit = if *((nation + nation::LEAGUE_SELECTED) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: the referees who stay per nation
            if group <= limit {
                ficam += 1;
                *m = 1;
                crate::comparison::branch("cqg_9 arbitro mantido");
            } else {
                *m = 0;
            }
            group += 1;
        }
    }
    let new = crt_calloc(ficam as u32, 0x2b);
    let mut dst = new;
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_OFFICIAL_OPERATOR_3D);
    let mut i = 0i32;
    while i < read_i32(G_OFFICIAL_COUNT) {
        if *((marks + i as usize) as *const u8) != 0 {
            copy(dst, official_record(i));
            dst += 0x2b;
        }
        i += 1;
    }
    crt_free(read_u32_as_usize(G_OFFICIAL_TABLE));
    write_u32(G_OFFICIAL_TABLE, new as u32);
    write_u32(G_OFFICIAL_COUNT, ficam as u32);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let nations = read_i32(G_NATION_COUNT);
    let blocos = malloc((nations as u32).wrapping_mul(0x102));
    write_u32(this + officials_manager_fields::NATION_BLOCKS, blocos as u32);
    if blocos == 0 {
        officials_out_of_memory(0x1ff);
    }
    for k in 0..nations.max(0) {
        let b = blocos + k as usize * 0x102;
        write_i16(b + officials_nation_block::COMP_COUNT, 0);
        write_u32(b + officials_nation_block::REFEREE_COUNT, 0);
        write_u32(b + officials_nation_block::FIRST_REFEREE, 1_000_000);
        write_u32(b + officials_nation_block::DEMAND, 0);
        write_u32(b, k as u32);
        for j in 0..0xf {
            let v = b + officials_nation_block::COMP_SLOTS + 16 * j;
            write_u32(v, 1_000_000);
            write_u32(v + 4, 0);
            write_u32(v + 8, 0);
            write_u32(v + 0xc, 0xffff_ffff);
        }
    }
    let active: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    for k in 0..read_i32(G_COMP_OBJECT_COUNT).max(0) {
        if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + 4 * k as usize) == 0 || active(k) == 0 {
            continue;
        }
        let comp = read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(k as usize * 0x6b);
        if friendly(read_u32_as_usize(G_FRIENDLY), comp) != 0 {
            continue;
        }
        let nation = read_u32_as_usize(comp + competition::NATION);
        if nation == 0 {
            continue;
        }
        let b = blocos.wrapping_add((read_i32(nation) as isize * 0x102) as usize);
        let v = b.wrapping_add((read_i16(b + officials_nation_block::COMP_COUNT) as isize * 0x10) as usize);
        write_u32(v + 0x1e, k as u32);
        let request = read_i16(read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xa; // cqg9: the slot request
        crate::comparison::branch("cqg_9 vaga de competicao");
        write_u32(v + 0x1a, request as u32);
        if read_i32(v + 0x1a) > read_i32(b + officials_nation_block::DEMAND) {
            write_u32(b + officials_nation_block::DEMAND, read_u32_as_usize(v + 0x1a) as u32);
        }
        write_i16(b + officials_nation_block::COMP_COUNT, read_i16(b + officials_nation_block::COMP_COUNT).wrapping_add(1));
    }
    for i in 0..read_i32(G_OFFICIAL_COUNT).max(0) {
        let b = blocos.wrapping_add((read_i32(read_u32_as_usize(official_record(i) + 0x16)) as isize * 0x102) as usize);
        write_u32(b + officials_nation_block::REFEREE_COUNT, read_u32_as_usize(b + officials_nation_block::REFEREE_COUNT).wrapping_add(1) as u32);
    }
    let novos: unsafe extern "stdcall" fn(i32, usize) = core::mem::transmute(FN_OFFICIALS_MANAGER_ADD_RANDOM_OFFICIALS);
    for k in 0..read_i32(G_NATION_COUNT).max(0) {
        let b = blocos + k as usize * 0x102;
        let nation = read_u32_as_usize(G_NATION_TABLE).wrapping_add(k as usize * nation::SIZE);
        let c = read_i16(b + officials_nation_block::COMP_COUNT);
        if c > 0 {
            write_u32(b + officials_nation_block::DEMAND, (c as i32).wrapping_mul(read_i32(b + officials_nation_block::DEMAND)) as u32);
            if read_i32(b + officials_nation_block::DEMAND) > read_i32(b + officials_nation_block::REFEREE_COUNT) {
                novos(read_i32(b + officials_nation_block::DEMAND) - read_i32(b + officials_nation_block::REFEREE_COUNT), nation);
                write_u32(b + officials_nation_block::REFEREE_COUNT, read_u32_as_usize(b + officials_nation_block::DEMAND) as u32);
            }
        } else if read_i32(b + officials_nation_block::REFEREE_COUNT) < 5 {
            if read_i16(nation + nation::REPUTATION) == 0 {
                write_i16(nation + nation::REPUTATION, 1);
            }
            novos((read_i16(nation + nation::REPUTATION) as i32).min(5), nation);
            write_u32(b + officials_nation_block::REFEREE_COUNT, (read_i16(read_u32_as_usize(G_NATION_TABLE).wrapping_add(k as usize * nation::SIZE) + 0x8e) as i32).min(5) as u32);
        }
    }
    for i in 0..read_i32(G_OFFICIAL_COUNT).max(0) {
        write_u32(official_record(i), i as u32);
    }
    for i in 0..read_i32(G_OFFICIAL_COUNT).max(0) {
        let r = official_record(i);
        let b = blocos.wrapping_add((read_i32(read_u32_as_usize(r + official::NATION)) as isize * 0x102) as usize);
        if read_i32(r) < read_i32(b + officials_nation_block::FIRST_REFEREE) {
            write_u32(b + officials_nation_block::FIRST_REFEREE, read_u32_as_usize(r) as u32);
        }
    }
    for k in 0..read_i32(G_NATION_COUNT).max(0) {
        let b = read_u32_as_usize(this + officials_manager_fields::NATION_BLOCKS) + k as usize * 0x102;
        let c = read_i16(b + officials_nation_block::COMP_COUNT);
        if c == 0 {
            continue;
        }
        officials_manager_crt_qsort(b + officials_nation_block::COMP_SLOTS, c as i32 as u32, 0x10, 0x79d410);
        let mut j = 0i32;
        while j < read_i16(b + officials_nation_block::COMP_COUNT) as i32 {
            let c = read_i16(b + officials_nation_block::COMP_COUNT) as i32;
            crate::comparison::branch("cqg_9 parte da vaga");
            write_u32(b + 0x16 + 16 * j as usize, (read_i32(b + officials_nation_block::REFEREE_COUNT) / c) as u32); // cqg9: the share of each slot
            let c = read_i16(b + officials_nation_block::COMP_COUNT) as i32;
            write_u32(b + officials_nation_block::COMP_SLOTS + 16 * j as usize, (read_i32(b + officials_nation_block::REFEREE_COUNT).wrapping_mul(j) / c).wrapping_add(read_i32(b + officials_nation_block::FIRST_REFEREE)) as u32);
            j += 1;
        }
    }
    let total = read_i32(G_OFFICIAL_COUNT);
    write_u32(this + officials_manager_fields::BUSY_COUNT, total as u32);
    let m = crt_calloc(total as u32, 1);
    write_u32(this + officials_manager_fields::BUSY_FLAGS, m as u32);
    if m == 0 {
        officials_out_of_memory(0x2dd);
    }
    crt_free(marks);
}

/// Orders a nation's referee slots by competition level (the invalid ones last or first), tie by id.
///
/// `officials_manager_compare_comp_level(a, b)` (cdecl), the order of a nation's competition slots (qsort of 0x79bf60,
/// 16-byte entries with the competition at +0xc): `a`'s competition outside 0..[0xae2378]: 1; `b`'s outside: −1; by
/// level (+0x69, word, of the 0x6b-byte competition in [0xae23d0]); same level: the difference of the ids.
/// Original: `officials_manager_compare_comp_level` (0x79d410, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79d410 } (cdecl, officials_manager.cpp): `officials_manager_compare_comp_level(a, b)`, a ordem das vagas de
/// competição de uma nação (qsort da 0x79bf60, entradas de 16 bytes com a competição em +0xc): competição de `a` fora de
/// 0..[0xae2378]: 1; a de `b` fora: −1; pelo nível (+0x69, palavra, da competição de 0x6b bytes em [0xae23d0]); mesmo
/// nível: a diferença dos ids.
pub(crate) unsafe extern "C" fn officials_manager_compare_comp_level(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_i32(a + officials_comp_slot::COMP_ID), read_i32(b + officials_comp_slot::COMP_ID));
    let max = read_i32(G_CLUB_COMP_COUNT);
    if ca < 0 || ca > max {
        return 1;
    }
    if cb < 0 || cb > max {
        return -1;
    }
    let base = read_i32(G_CLUB_COMP_TABLE) as u32 as usize;
    let sa = read_i16(base.wrapping_add((ca as isize * 0x6b + 0x69) as usize));
    let signed_byte_at = read_i16(base.wrapping_add((cb as isize * 0x6b + 0x69) as usize));
    if sa == signed_byte_at {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - signed_byte_at as i32
}

/// Official's sorting quality: byte +0x28 x 50 + word +0x22, in 16 bits.
///
/// [pt] A qualidade do árbitro na ordem: o byte +0x28 × 50 + a palavra +0x22, em 16 bits.
pub(crate) unsafe fn official_quality(r: usize) -> i16 {
    (*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(read_i16(r + official::RATING)) // cqh7: the quality weight
}

/// Nation id of the official ([+0x16] + 0).
///
/// [pt] O id da nação do árbitro ([+0x16] + 0).
pub(crate) unsafe fn official_nation_id(r: usize) -> i32 {
    read_i32(read_i32(r + official::NATION) as u32 as usize)
}

/// Orders the referees by nation and, within it, by quality (+0x28 × 50 + +0x22) from highest to lowest; tie by index,
/// descending.
///
/// `officials_manager_compare_nation_then_quality_desc(a, b)` (cdecl), the qsort order of 0x79bf60 (those kept per
/// nation): by nation (id) and, within it, by quality from highest to lowest; tie: `b`'s index +0 minus `a`'s.
/// Original: `officials_manager_compare_nation_then_quality_desc` (0x79d4e0, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79d4e0 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_desc(a, b)`, a ordem do
/// qsort da 0x79bf60 (os que ficam por nação): pela nação (id) e, na mesma, pela qualidade da maior para a menor; empate:
/// o índice +0 de `b` menos o de `a`.
pub(crate) unsafe extern "C" fn officials_manager_compare_nation_then_quality_desc(a: usize, b: usize) -> i32 {
    let (na, nb) = (official_nation_id(a), official_nation_id(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (official_quality(a), official_quality(b));
    crate::comparison::branch("cqh_7 ordem decrescente");
    if ka == kb {
        return read_i32(b).wrapping_sub(read_i32(a)); // cqh7: the tie by the index
    }
    kb as i32 - ka as i32
}

/// Orders the referees by nation and by quality from lowest to highest; tie by index, ascending.
///
/// `officials_manager_compare_nation_then_quality_asc(a, b)` (cdecl), the qsort order of 0x79c5c0 (after the new
/// referees): by nation and by quality from lowest to highest; tie: `a`'s index +0 minus `b`'s.
/// Original: `officials_manager_compare_nation_then_quality_asc` (0x79d480, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79d480 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_asc(a, b)`, a ordem do
/// qsort da 0x79c5c0 (depois dos árbitros novos): pela nação e pela qualidade da menor para a maior; empate: o índice +0
/// de `a` menos o de `b`.
pub(crate) unsafe extern "C" fn officials_manager_compare_nation_then_quality_asc(a: usize, b: usize) -> i32 {
    let (na, nb) = (official_nation_id(a), official_nation_id(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (official_quality(a), official_quality(b));
    crate::comparison::branch("cqh_7 ordem crescente");
    if ka == kb {
        return read_i32(a).wrapping_sub(read_i32(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// The nation's officials block in the manager (0x102 bytes in [this+0xc]: counts, first index, league bands).
///
/// [pt] O bloco de árbitros da nação `nacao` no gerenciador (0x102 bytes em [this+0xc]: +4 conta, +8 primeiro índice, +0x10
/// palavra com o número de faixas de liga, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição).
pub(crate) unsafe fn nation_officials_block(this: usize, nation: i32) -> usize {
    read_usize(this + officials_manager_fields::NATION_BLOCKS).wrapping_add(nation.wrapping_mul(0x102) as usize)
}

/// Nation id of the club (+0x53), or None without nation.
///
/// [pt] O id da nação do clube (+0x53, ponteiro) ou, sem nação, `sem`.
pub(crate) unsafe fn officials_manager_club_nation_id(club: usize) -> Option<i32> {
    let n = read_usize(club + club::NATION);
    if n != 0 { Some(read_i32(n)) } else { None }
}

/// Chooses a match referee: the referee range for the competition, a random one in it and the next free one.
///
/// `OfficialsManager::choose_match_official(match, out)`, from setting up the day (0x6b3c10, output at +0x2e of the
/// match). Removed match (−2): 0 (eax 0xffffff00, as in the original). The referee range: without a competition, the
/// home club's nation (+0x1c, +0x53) or a random one; a friendly (0x5c6710 on [0xae2c74]), the home club's; a
/// competition without an object ([0xadadfc]), as without a competition; a competition of types 2 and 4 (+0x42), the
/// top quarter (ftol of 0.75 and 0.25 of the count) of the first nation, starting from a random one, of the continent
/// (+0x59 of the competition, or +0x71 of the home club's nation, of the away club's, or a random one) that is not that
/// of either club (a neutral nation); in the others, the competition's range in its nation's block (+0x5d; or the home
/// club's, the away club's, or a random one), the last that matches, or the whole block. Count 1: the first; otherwise
/// first + random(count − 1). Busy (the byte of the referee's id in [this+8]): the next in the range, wrapping around;
/// all busy, the drawn one. Writes the index (word) and returns 1 in al (the rest of eax with the index, as in the
/// original).
/// Original: `OfficialsManager::choose_match_official` (0x79c7b0, officials_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x79c7b0 } (thiscall, ret 8, officials_manager.cpp): `OfficialsManager::choose_match_official(jogo, saida)`, pela
/// montagem do dia (0x6b3c10, saída em +0x2e do jogo). Jogo tirado (−2): 0 (eax 0xffffff00, como a original). A faixa
/// de árbitros: sem competição, a nação do mandante (+0x1c, +0x53) ou uma sorteada; amistoso (0x5c6710 em [0xae2c74]),
/// a do mandante; competição sem objeto ([0xadadfc]), como sem competição; competição dos tipos 2 e 4 (+0x42), o quarto
/// de cima (ftol de 0,75 e 0,25 da conta) da primeira nação, a partir de uma sorteada, do continente (+0x59 da
/// competição, ou +0x71 da nação do mandante, do visitante, ou um sorteado) que não seja a de nenhum dos dois clubes;
/// nas outras, a faixa da competição no bloco da nação dela (+0x5d; ou do mandante, do visitante, ou sorteada), a última
/// que bater, ou o bloco inteiro. Conta 1: o primeiro; senão primeiro + sorteio(conta − 1). Ocupado (o byte do id do
/// árbitro em [this+8]): o seguinte da faixa, dando a volta; todos ocupados, o sorteado. Grava o índice (palavra) e
/// devolve 1 no al (o resto do eax com o índice, como a original).
pub(crate) unsafe extern "thiscall" fn choose_match_official(this: usize, game: usize, out: usize) -> u32 {
    if read_i32(game) == -2 {
        return 0xffff_ff00;
    }
    let nations = read_i32(G_NATION_COUNT);
    let home = read_usize(game + fixture::HOME_CLUB);
    let away = read_usize(game + fixture::AWAY_CLUB);
    let comp = read_usize(game + fixture::COMPETITION);
    let band_of_nation = |nacao: i32| {
        let b = nation_officials_block(this, nacao);
        (read_i32(b + officials_nation_block::FIRST_REFEREE), read_i32(b + officials_nation_block::REFEREE_COUNT))
    };
    let (ini, n) = if comp == 0 {
        band_of_nation(officials_manager_club_nation_id(home).unwrap_or_else(|| random(nations)))
    } else {
        let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
        if friendly(read_usize(G_FRIENDLY), comp) != 0 {
            band_of_nation(read_i32(read_usize(home + club::NATION))) // like the original, without checking the nation
        } else {
            let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(comp).wrapping_mul(4) as usize));
            if c == 0 {
                band_of_nation(officials_manager_club_nation_id(home).unwrap_or_else(|| random(nations)))
            } else {
                let kind = read_u8(c + cup::CLUB_SIDES) as i8;
                if kind == 2 || kind == 4 {
                    let continent_of = |club: usize| {
                        let n = read_usize(club + club::NATION);
                        if n != 0 && read_usize(n + nation::CONTINENT) != 0 { Some(read_i32(read_usize(n + nation::CONTINENT))) } else { None }
                    };
                    let continent = if read_usize(comp + competition::CONTINENT) != 0 {
                        read_i32(read_usize(comp + competition::CONTINENT))
                    } else if let Some(k) = continent_of(home) {
                        k
                    } else if let Some(k) = continent_of(away) {
                        k
                    } else {
                        random(read_i32(G_CONTINENT_COUNT))
                    };
                    let start = random(nations);
                    let mut k = start;
                    loop {
                        let nation = read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize);
                        let cont = read_usize(nation + nation::CONTINENT);
                        if cont != 0 && read_i32(cont) == continent
                            && officials_manager_club_nation_id(home) != Some(read_i32(nation))
                            && officials_manager_club_nation_id(away) != Some(read_i32(nation))
                        {
                            break;
                        }
                        k += 1;
                        if k >= read_i32(G_NATION_COUNT) {
                            k = 0;
                        }
                        if k == start {
                            break;
                        }
                    }
                    let b = nation_officials_block(this, k);
                    let count = read_i32(b + officials_nation_block::REFEREE_COUNT) as f64;
                    crate::comparison::branch("cpy_8 arbitro neutro");
                    (read_i32(b + officials_nation_block::FIRST_REFEREE).wrapping_sub(match_pl_ftol(count * -0.75)), match_pl_ftol(count * 0.25)) // cpy8: the top quarter
                } else {
                    let nacao = if read_usize(comp + competition::NATION) != 0 {
                        read_i32(read_usize(comp + competition::NATION))
                    } else if let Some(k) = officials_manager_club_nation_id(home) {
                        k
                    } else if let Some(k) = officials_manager_club_nation_id(away) {
                        k
                    } else {
                        random(nations)
                    };
                    let b = nation_officials_block(this, nacao);
                    let id = read_i32(comp);
                    let mut found = None;
                    for f in 0..read_i16(b + officials_nation_block::COMP_COUNT).max(0) as usize {
                        let e = b + officials_nation_block::COMP_SLOTS + f * 0x10;
                        if read_i32(e + officials_comp_slot::COMP_ID) == id {
                            found = Some((read_i32(e), read_i32(e + officials_comp_slot::SHARE)));
                        }
                    }
                    match found {
                        Some(band) => {
                            crate::comparison::branch("cpy_8 band da liga");
                            band // cpy8: the league band
                        }
                        None => (read_i32(b + officials_nation_block::FIRST_REFEREE), read_i32(b + officials_nation_block::REFEREE_COUNT)),
                    }
                }
            }
        }
    };
    let drawn = if n == 1 {
        ini
    } else {
        crate::comparison::branch("cpy_8 arbitro sorteado");
        ini.wrapping_add(random(n.wrapping_sub(1))) // cpy8: the random draw within the band
    };
    let ocupado = |k: i32| {
        let sheet = read_usize(G_OFFICIAL_TABLE).wrapping_add(k.wrapping_mul(0x2b) as usize);
        read_u8(read_usize(this + officials_manager_fields::BUSY_FLAGS).wrapping_add(read_usize(sheet))) != 0
    };
    let mut k = drawn;
    if ocupado(k) {
        let end = n.wrapping_add(ini);
        loop {
            k += 1;
            if k >= end {
                k = ini;
            }
            if k == drawn || !ocupado(k) {
                break;
            }
        }
    }
    core::ptr::write_unaligned(out as *mut i16, k as i16);
    (k as u32 & 0xffff_ff00) | 1
}

/// Clears the daily referee busy bytes.
///
/// `OfficialsManager::clear_busy_flags`, in the day loop (0x5cccd0) and in the run up to the start date (0x5cc850):
/// zeroes the [this+4] busy bytes at [this+8] (the count and the vector re-read at each step). Returns the count (0 if
/// not positive), like the original's eax.
/// Original: `OfficialsManager::clear_busy_flags` (0x79bf40, officials_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x79bf40 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::clear_busy_flags`, no laço do dia
/// (0x5cccd0) e na corrida até a data de início (0x5cc850): zera os [this+4] bytes de ocupado em [this+8] (a conta e o
/// vetor relidos a cada passo). Devolve a conta (0 se não positiva), como o eax da original.
pub(crate) unsafe extern "thiscall" fn clear_busy_flags(this: usize) -> u32 {
    let mut i = 0i32;
    while i < read_i32(this + officials_manager_fields::BUSY_COUNT) {
        *(read_usize(this + officials_manager_fields::BUSY_FLAGS).wrapping_add(i as usize) as *mut u8) = 0;
        i += 1;
    }
    i as u32
}

/// At each league's season start, replaces the referees of its range over 45 (or over 40, one in three) with new random
/// ones and clears the range's statistics rows.
///
/// `OfficialsManager::renew_officials(comp)`, at the start of each league's season (83 callers). Without a competition,
/// without an object ([0xadadfc]), of types 4 and 2 (+0x42), or without a count (0x4b0700): 0. The competition's range
/// in its nation's block (+0x5d, or a random one), the first that matches, or the whole block. Each referee of the
/// range (0x2b-byte records in [0xae23e8]; null record: the "Error" box): over 45 years old (the year as a byte, minus
/// 1 before his birthday), or over 40 and random(3) zero, becomes another: two names from his nation (0x539c60, +4 and
/// +8), +0x25 random(16) + 5 (or, with random(10) zero, random(11) + random(11)), the year (+0xe) game year − 27 +
/// random(5) and the day (+0xc) random(340). Then it zeroes the statistics rows (17 bytes at [this]) of each referee of
/// the range (+4, +8, +0xa, +0xc, +0xf; the `add eax, 0x11` at 0x79d149 stays inside the loop). Returns 1 (only al
/// counts).
/// Original: `OfficialsManager::renew_officials` (0x79cee0, officials_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x79cee0 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::renew_officials(comp)`, no começo da
/// temporada de cada liga (83 chamadores). Sem competição, sem objeto ([0xadadfc]), dos tipos 4 e 2 (+0x42) ou sem
/// conta (0x4b0700): 0. A faixa da competição no bloco da nação dela (+0x5d, ou uma sorteada), a primeira que bater,
/// ou o bloco inteiro. Cada árbitro da faixa (fichas de 0x2b bytes em [0xae23e8]; ficha nula: a caixa "Error"): com
/// mais de 45 anos (o ano em byte, menos 1 antes do dia dele), ou mais de 40 e sorteio(3) zero, vira outro: dois nomes
/// da nação dele (0x539c60, +4 e +8), +0x25 sorteio(16) + 5 (ou, com sorteio(10) zero, sorteio(11) + sorteio(11)), o
/// ano (+0xe) ano do jogo − 27 + sorteio(5) e o dia (+0xc) sorteio(340). Depois zera as linhas de estatísticas (17
/// bytes em [this]) de cada árbitro da faixa (+4, +8, +0xa, +0xc, +0xf; o `add eax, 0x11` em 0x79d149 fica dentro do
/// laço). Devolve 1 (só o al vale).
pub(crate) unsafe extern "thiscall" fn renew_officials(this: usize, comp: usize) -> u8 {
    if comp == 0 {
        return 0;
    }
    let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(comp).wrapping_mul(4) as usize));
    if c == 0 || read_u8(c + cup::CLUB_SIDES) == 4 || read_u8(c + cup::CLUB_SIDES) == 2 {
        return 0;
    }
    let count: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    if count(read_i32(comp)) == 0 {
        return 0;
    }
    let nacao = if read_usize(comp + competition::NATION) != 0 { read_i32(read_usize(comp + competition::NATION)) } else { random(read_i32(G_NATION_COUNT)) };
    let b = nation_officials_block(this, nacao);
    let mut band = None;
    for f in 0..read_i16(b + officials_nation_block::COMP_COUNT).max(0) as usize {
        let e = b + officials_nation_block::COMP_SLOTS + f * 0x10;
        if read_i32(e + officials_comp_slot::COMP_ID) == read_i32(comp) {
            band = Some((read_i32(e), read_i32(e + officials_comp_slot::SHARE)));
            break;
        }
    }
    let (ini, n) = band.unwrap_or((read_i32(b + officials_nation_block::FIRST_REFEREE), read_i32(b + officials_nation_block::REFEREE_COUNT)));
    let name: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    for k in 0..n.max(0) {
        let sheet = read_usize(G_OFFICIAL_TABLE).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x2b) as usize);
        if sheet == 0 {
            show_error_at(0xa6a22c, 0xa61);
            continue;
        }
        let year = read_u8(G_CURRENT_DATE_YEAR);
        let dif = (read_i16(sheet + official::BIRTH_DAY) as i32).wrapping_sub(read_i16(G_CURRENT_DATE) as i32);
        let age = if dif > 0 { year.wrapping_sub(read_u8(sheet + official::BIRTH_YEAR)).wrapping_sub(1) } else { year.wrapping_sub(read_u8(sheet + official::BIRTH_YEAR)) } as i8;
        if age <= 0x28 || (age <= 0x2d && random(3) != 0) {
            continue;
        }
        crate::comparison::branch("cpy_8 arbitro renovado");
        let nation = read_usize(sheet + player_sheet::WING_BACK);
        let n1 = name(nation);
        core::ptr::write_unaligned((sheet + 4) as *mut usize, n1);
        let n2 = name(read_usize(sheet + player_sheet::WING_BACK));
        core::ptr::write_unaligned((sheet + 8) as *mut usize, n2);
        *((sheet + player_sheet::DIRTINESS) as *mut u8) = if random(10) != 0 {
            (random(0x10) as u8).wrapping_add(5)
        } else {
            let a = random(0xb) as u8;
            a.wrapping_add(random(0xb) as u8)
        };
        let r5 = random(5);
        let base = (read_i16(G_CURRENT_DATE_YEAR) as u16).wrapping_sub(0x1b);
        core::ptr::write_unaligned((sheet + official::BIRTH_YEAR) as *mut u16, (r5 as u16).wrapping_add(base));
        core::ptr::write_unaligned((sheet + official::BIRTH_DAY) as *mut u16, random(0x154) as u16);
    }
    for k in 0..n.max(0) {
        let row = read_usize(this).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x11) as usize);
        *((row + official_stats::CARDS) as *mut u8) = 0;
        core::ptr::write_unaligned((row + official_stats::FOULS) as *mut u16, 0);
        core::ptr::write_unaligned((row + official_stats::GRADE_SUM) as *mut u16, 0);
        core::ptr::write_unaligned((row + official_stats::MATCHES) as *mut u16, 0);
        core::ptr::write_unaligned((row + official_stats::AVERAGE_GRADE) as *mut u32, 0);
    }
    1
}

/// Random byte: random(n) + k.
///
/// [pt] Um byte sorteado: sorteio(n) + k no al.
pub(crate) unsafe fn random_byte_plus(n: i32, k: u8) -> u8 {
    (random(n) as u8).wrapping_add(k)
}

/// A new referee record in `r` (0x2b bytes) for nation `nacao`, in the order of `0x79c6aa`.
///
/// First and second name by weight (`0x539b90`, `0x53a0e0`), the nation (`+0x16`), the words `+0x22` random draw(80) +
/// 1, `+0x1e` and `+0x20` random draw(80) + 40 (the byte extended), bytes `+0x24`..`+0x26` random draw(10) + 5 and
/// `+0x27`..`+0x2a` random draw(10) + 7, `+0x1a` zero, the year (`+0xe`) 1960 + random draw(10) and the day (`+0xc`)
/// random draw(300) + 1.
///
/// It does not write `+0` and `+0x10`..`+0x15` (the original routine leaves whatever the realloc brought).
///
/// [pt] Uma ficha de árbitro nova em `r` (0x2b bytes) da nação `nacao`, na ordem da 0x79c6aa: primeiro e segundo nome pelo
/// peso (0x539b90, 0x53a0e0), a nação (+0x16), as palavras +0x22 sorteio(80) + 1, +0x1e e +0x20 sorteio(80) + 40 (o
/// byte estendido), os bytes +0x24..+0x26 sorteio(10) + 5 e +0x27..+0x2a sorteio(10) + 7, +0x1a zero, o ano (+0xe)
/// 1960 + sorteio(10) e o dia (+0xc) sorteio(300) + 1. Não grava +0, +0x10..+0x15 (a original deixa o que o realloc
/// trouxe).
pub(crate) unsafe fn generate_official(r: usize, nation: usize) {
    let first: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_BY_WEIGHT);
    let second: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_BY_WEIGHT);
    write_usize(r + 4, first(nation));
    write_usize(r + 8, second(nation));
    write_usize(r + official::NATION, nation);
    write_i16(r + official::RATING, random_byte_plus(0x50, 1) as i8 as i16); // cpy9: a nota +0x22
    write_i16(r + official::CURRENT_ABILITY, random_byte_plus(0x50, 0x28) as i8 as i16);
    write_i16(r + official::POTENTIAL_ABILITY, random_byte_plus(0x50, 0x28) as i8 as i16);
    for o in [0x24usize, 0x25, 0x26] {
        write_u8(r + o, random_byte_plus(10, 5));
    }
    write_u8(r + 0x27, random_byte_plus(10, 7));
    write_u8(r + 0x28, random_byte_plus(10, 7)); // cpy9: o atributo +0x28
    write_u8(r + 0x29, random_byte_plus(10, 7));
    write_u8(r + 0x2a, random_byte_plus(10, 7));
    write_usize(r + 0x1a, 0);
    write_i16(r + official::BIRTH_YEAR, random(10).wrapping_add(0x7a8) as i16);
    write_i16(r + official::BIRTH_DAY, random(0x12c).wrapping_add(1) as i16);
}

/// Reallocates the referee table, generates `count` new referees of the nation and sorts the table by nation and
/// quality.
///
/// `officials_manager_add_random_officials(count, nation)` (thiscall with this unused, ret 8), from setting up the
/// referees of a new game (0x79bf60, in nations with few referees). A nation without a continent (+0x71): nothing. It
/// reallocates the table ([0xae23e8], 0x945667) to [0xae2390] + count records (out of memory: error and the game
/// stops), generates `count` new records at the end (generate_official: names by weight, random ratings and attributes,
/// born in the 1960s), adds the count to [0xae2390] and sorts the whole table (qsort 0x945333 with 0x79d480: nation,
/// +0x28 × 50 + +0x22, id).
/// Original: `officials_manager_add_random_officials` (0x79c5c0, officials_manager.cpp, thiscall (this sem uso), ret 8).
///
/// [pt] { 0x79c5c0 } (thiscall com o this sem uso, ret 8, officials_manager.cpp): `officials_manager_add_random_officials(
/// conta, nacao)`, pela montagem dos árbitros do jogo novo (0x79bf60, nas nações com poucos árbitros). Nação sem
/// continente (+0x71): nada. Realoca a tabela ([0xae23e8], 0x945667) para [0xae2390] + conta fichas (sem memória: erro
/// e o jogo para), gera `conta` fichas novas no fim (generate_official), soma a conta em [0xae2390] e ordena a tabela inteira
/// (qsort 0x945333 com 0x79d480: nação, +0x28 × 50 + +0x22, id).
pub(crate) unsafe extern "thiscall" fn officials_manager_add_random_officials(_this: usize, count: i32, nation: usize) {
    if nation != 0 && read_usize(nation + nation::CONTINENT) == 0 {
        return;
    }
    let realloc: unsafe extern "C" fn(usize, i32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let before = read_i32(G_OFFICIAL_COUNT);
    let t = realloc(read_usize(G_OFFICIAL_TABLE), before.wrapping_add(count).wrapping_mul(0x2b));
    *(G_OFFICIAL_TABLE as *mut usize) = t;
    if t == 0 {
        fatal_error_at(0xa6a22c, 0x342);
    }
    let mut r = t.wrapping_add(before.wrapping_mul(0x2b) as usize);
    for _ in 0..count.max(0) {
        crate::comparison::branch("cpy_9 arbitro novo");
        generate_official(r, nation);
        r += 0x2b;
    }
    let n = read_i32(G_OFFICIAL_COUNT).wrapping_add(count);
    *(G_OFFICIAL_COUNT as *mut i32) = n;
    let sort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    crate::comparison::branch("cpy_9 ordenar");
    sort(read_usize(G_OFFICIAL_TABLE), n, 0x2b, 0x79d480); // cpy9: ordenar a tabela
}

/// Does the dry-run generated record match the table record in the fields 0x79c5c0 writes (+4..+0x10, +0x16..+0x2b)?
///
/// [pt] A ficha `g` (gerada no ensaio) bate com a `f` da tabela nos campos que a 0x79c5c0 grava (+4..+0x10, +0x16..+0x2b)?
pub(crate) unsafe fn same_official_record(g: &[u8; 0x2b], f: usize) -> bool {
    let t = core::slice::from_raw_parts(f as *const u8, 0x2b);
    g[4..0x10] == t[4..0x10] && g[0x16..] == t[0x16..]
}

/// If an attribute byte is outside 1..20, replaces it with the value from f.
///
/// [pt] O ajuste de um atributo em byte (+0x24..+0x2a) fora de 1..20: o valor sorteado por `f` (lê a ficha antes).
pub(crate) unsafe fn fix_official_attribute(e: usize, o: usize, f: impl FnOnce() -> u8) {
    let v = read_u8(e + o) as i8;
    if v <= 0 || v > 0x14 {
        write_u8(e + o, f());
    }
}

/// Repairs each referee record: the nation from the names, out-of-range ratings and attributes drawn again, and a
/// too-old birth date.
///
/// `officials_manager_validate_officials()` (cdecl), at the start of setting up the referees of a new game (0x79bf60).
/// For each of the [0xae2390] records (0x2b bytes in [0xae23e8]): without a nation (+0x16), the one from the nation
/// table by the index (+0x37) of the first name (+4), of the second (+8), or a random one; the words +0x22, +0x1e and
/// +0x20 outside 1..200: random(80) + 1 plus the nation's byte +0x8e (+0x22) or 4 times it (+0x1e, +0x20; the redone
/// +0x20 is at least +0x1e); the bytes +0x24..+0x2a outside 1..20 drawn by the bands of +0x1e (above 120, above 80);
/// born before the date 0x549f70(1, 1, 1910, −1) (year unsigned, then the day signed), year 1960 + random(10) and day
/// random(300) + 1.
/// Original: `officials_manager_validate_officials` (0x79caa0, officials_manager.cpp, cdecl).
///
/// [pt] { 0x79caa0 } (cdecl, officials_manager.cpp): `officials_manager_validate_officials()`, no começo da montagem dos
/// árbitros do jogo novo (0x79bf60). Para cada uma das [0xae2390] fichas (0x2b bytes em [0xae23e8]): sem nação (+0x16),
/// a da tabela das nações pelo índice (+0x37) do primeiro nome (+4), do segundo (+8) ou sorteado; as palavras +0x22,
/// +0x1e e +0x20 fora de 1..200: sorteio(80) + 1 mais o byte +0x8e da nação (+0x22) ou 4 vezes ele (+0x1e, +0x20; a
/// +0x20 refeita fica no mínimo na +0x1e); os bytes +0x24..+0x2a fora de 1..20 sorteados pelas faixas de +0x1e (acima de
/// 120, de 80); nascido antes da data 0x549f70(1, 1, 1910, −1) (ano sem sinal, depois o dia com sinal), ano 1960 +
/// sorteio(10) e dia sorteio(300) + 1.
pub(crate) unsafe extern "C" fn officials_manager_validate_officials() {
    let base = read_usize(G_OFFICIAL_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_OFFICIAL_COUNT) {
        let e = base.wrapping_add((i as usize).wrapping_mul(0x2b));
        if read_usize(e + official::NATION) == 0 {
            let (n1, n2) = (read_usize(e + 4), read_usize(e + 8));
            let k = if read_i32(n1 + 0x37) >= 0 {
                read_i32(n1 + 0x37)
            } else if read_i32(n2 + 0x37) >= 0 {
                read_i32(n2 + 0x37)
            } else {
                random(read_i32(G_NATION_COUNT))
            };
            write_usize(e + official::NATION, read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize));
        }
        let away = |o: usize| {
            let v = read_i16(e + o);
            v <= 0 || v > 0xc8
        };
        if away(0x22) {
            let nation = read_usize(e + official::NATION);
            let v = if nation != 0 {
                let s = random(0x50) as u8;
                s.wrapping_add(read_u8(read_usize(e + official::NATION) + 0x8e)).wrapping_add(1)
            } else {
                (random(0x50) as u8).wrapping_add(1)
            };
            write_i16(e + official::RATING, v as i8 as i16);
        }
        for o in [0x1eusize, 0x20] {
            if away(o) {
                crate::comparison::branch("cpy_9 nota consertada");
                let nation = read_usize(e + official::NATION);
                let v = if nation != 0 {
                    let s = random(0x50) as u8;
                    s.wrapping_add(read_u8(read_usize(e + official::NATION) + 0x8e) << 2).wrapping_add(1)
                } else {
                    (random(0x50) as u8).wrapping_add(1)
                };
                write_i16(e + o, v as i8 as i16);
                if o == 0x20 && read_i16(e + official::POTENTIAL_ABILITY) < read_i16(e + official::CURRENT_ABILITY) {
                    write_i16(e + official::POTENTIAL_ABILITY, read_i16(e + official::CURRENT_ABILITY));
                }
            }
        }
        let band = |high: (i32, u8), medio: (i32, u8), low: (i32, u8)| {
            let w = read_i16(e + official::CURRENT_ABILITY);
            let (n, k) = if w > 0x78 { high } else if w > 0x50 { medio } else { low };
            random_byte_plus(n, k)
        };
        fix_official_attribute(e, 0x24, || if read_i16(e + official::CURRENT_ABILITY) > 0x64 { random_byte_plus(0xc, 7) } else { random_byte_plus(0xa, 5) });
        fix_official_attribute(e, 0x25, || band((0xe, 7), (0xf, 5), (0xa, 5)));
        fix_official_attribute(e, 0x26, || band((0xb, 0xa), (0xf, 5), (0xa, 5)));
        fix_official_attribute(e, 0x27, || if read_i16(e + official::CURRENT_ABILITY) > 0x50 { random_byte_plus(0xf, 6) } else { random_byte_plus(0x14, 1) });
        fix_official_attribute(e, 0x28, || band((0xa, 0xb), (0xf, 5), (0xa, 7)));
        fix_official_attribute(e, 0x29, || band((0xd, 8), (0xe, 7), (0xf, 6)));
        fix_official_attribute(e, 0x2a, || if random(0x64) != 0 { random_byte_plus(0xa, 0xb) } else { random_byte_plus(0xa, 5) });
        let mut limit = [0u8; 8];
        let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> *const u8 = core::mem::transmute(FN_DATE_DATE_549F70);
        let d = data(limit.as_mut_ptr(), 1, 1, 0x776, -1);
        let (dia, ano) = (read_i16(d as usize), read_i16(d as usize + 2) as u16);
        let year_and = read_i16(e + official::BIRTH_YEAR) as u16;
        if ano > year_and || (ano == year_and && dia > read_i16(e + official::BIRTH_DAY)) {
            write_i16(e + official::BIRTH_YEAR, random(10).wrapping_add(0x7a8) as i16);
            write_i16(e + official::BIRTH_DAY, random(0x12c).wrapping_add(1) as i16);
        }
        i += 1;
    }
}

/// Copies a 0x2b-byte referee record field by field, the date through the date copy.
///
/// `Official::operator=(src)`, from setting up the referees of a new game (0x79bf60, the records that are kept). Copies
/// the 0x2b-byte record field by field (+0, +4, +8; the date at +0xc through 0x417c10; +0x14, +0x16, +0x1a, +0x1e,
/// +0x20, +0x22, the bytes +0x24..+0x2a). Returns `this`.
/// Original: `Official::operator=` (0x79c530, officials_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x79c530 } (thiscall, ret 4, officials_manager.cpp): `Official::operator=(src)`, pela montagem dos árbitros do
/// jogo novo (0x79bf60, as fichas que ficam). Copia a ficha de 0x2b bytes campo a campo (+0, +4, +8; a data de +0xc
/// pela 0x417c10; +0x14, +0x16, +0x1a, +0x1e, +0x20, +0x22, os bytes +0x24..+0x2a). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn officials_manager_operator(this: usize, src: usize) -> usize {
    for o in [0usize, 4, 8] {
        write_usize(this + o, read_usize(src + o));
    }
    let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copy(this + official::BIRTH_DAY, t.as_mut_ptr(), src + official::BIRTH_DAY);
    write_i16(this + 0x14, read_i16(src + 0x14));
    write_usize(this + official::NATION, read_usize(src + official::NATION));
    write_usize(this + 0x1a, read_usize(src + 0x1a));
    for o in [0x1eusize, 0x20, 0x22] {
        write_i16(this + o, read_i16(src + o));
    }
    for o in 0x24usize..0x2b {
        write_u8(this + o, read_u8(src + o));
    }
    this
}

/// Writes the referees and their statistics into the off.dat item of the save.
///
/// `OfficialsManager::save_official_stats(save)`: writes into the `off.dat` item the number of referees [0xae2390], the
/// 0x11-byte records from [+0] (one per referee), the number of nations [0xae2350], the 0x102-byte records from [+0xc]
/// (one per nation) and one byte per referee from [+8]; returns 1 (in the low byte). No container or an error on
/// opening: the "Error" box (officials_manager.cpp 0x142, 0x14a) and 0; no referees or an error on writing: the box
/// (0x150, 0x156, 0x15c, 0x164, 0x16c, 0x174), it closes the item and 0.
/// Original: `OfficialsManager::save_official_stats` (0x79bb80, officials_manager.cpp, thiscall).
///
/// [pt] { 0x79bb80 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::save_official_stats(save)`: grava no item
/// `off.dat` a quantidade de árbitros [0xae2390], os registros de 0x11 bytes de [+0] (um por árbitro), a quantidade de
/// nações [0xae2350], os registros de 0x102 bytes de [+0xc] (um por nação) e um byte por árbitro de [+8]; devolve 1
/// (no byte baixo). Sem contêiner ou erro ao abrir: a caixa "Error" (officials_manager.cpp 0x142, 0x14a) e 0; sem
/// árbitros ou erro ao gravar: a caixa (0x150, 0x156, 0x15c, 0x164, 0x16c, 0x174), fecha o item e 0.
pub(crate) unsafe extern "thiscall" fn save_official_stats(this: usize, save: usize) -> u8 {
    const FONTE: usize = 0xa6a22c;
    let arbitros = || *(G_OFFICIAL_COUNT as *const u32);
    let nations = || *(G_NATION_COUNT as *const u32);
    crate::comparison::branch("cpx_7 gravar estatisticas dos arbitros");
    if save == 0 {
        show_error_at(FONTE, 0x142);
        return 0;
    }
    let mut z = zip_open_item(save, 0xa6a260);
    if z == 0 {
        show_error_at(FONTE, 0x14a);
        return 0;
    }
    if zip_write(0xae2390, 4, 1, z) != 1 {
        return officials_save_error(0x150, &mut z);
    }
    if arbitros() == 0 {
        return officials_save_error(0x156, &mut z);
    }
    if zip_write(read_usize(this), 0x11, arbitros(), z) != arbitros() {
        return officials_save_error(0x15c, &mut z);
    }
    if zip_write(0xae2350, 4, 1, z) != 1 {
        return officials_save_error(0x164, &mut z);
    }
    if zip_write(read_usize(this + officials_manager_fields::NATION_BLOCKS), 0x102, nations(), z) != nations() { // cpx7: the records of the nations
        return officials_save_error(0x16c, &mut z);
    }
    if zip_write(read_usize(this + officials_manager_fields::BUSY_FLAGS), 1, arbitros(), z) != arbitros() {
        return officials_save_error(0x174, &mut z);
    }
    zip_close(&mut z);
    1
}

