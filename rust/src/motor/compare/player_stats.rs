use super::*;

// The comparers of the routines in ../player_stats.rs.
// [pt] Os comparadores das rotinas de ../player_stats.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `player_statistic`.
pub(crate) unsafe extern "stdcall" fn compare_player_statistic(r: *const u8, which: i8, weight: f32) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(IDX_PLAYER_STATISTIC)(r, which, weight);
    let n = player_statistic(r, which, weight);
    SCOREBOARD.record(o.to_bits() == n.to_bits(), || format!("player_statistic({:p}, {}, {}) original={} rust={}", r, which, weight, o, n));
    o
}

/// Comparer for `archive_player_stats` (comparison mode: see the header of motor.rs), as a dry run (model
/// RARE_FINANCE_EVENT_DRY_RUN).
///
/// The copy runs first without writing to the file (the nodes it would write go into the sum of matches and goals),
/// then the original routine from the same state; compares the return value, the six 32-byte category records and the
/// matches and goals for the club (+0x20/+0x22 of the player sheet's record).
///
/// The history positions (+0x18, +0x1c), which only the original's write moves, are left out. Null person or no player
/// sheet: only the original.
///
/// [pt] Comparador em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem gravar no arquivo (os nós que gravaria entram na
/// soma dos jogos e gols), depois a original a partir do mesmo estado; compara a devolução, os seis registros de 32 bytes
/// das categorias e os jogos e gols pelo clube (+0x20/+0x22 do registro da ficha). As posições do histórico (+0x18,
/// +0x1c), que só a gravação da original move, ficam fora. Pessoa nula ou sem ficha: só a original.
pub(crate) unsafe extern "thiscall" fn compare_archive_player_stats(this: usize, p: usize, value: u32, loan: u32, end: u32, club_id: u32, other: u32) -> u32 {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_ARCHIVE_PLAYER_STATS);
    if p.is_null() || p.player().is_null() {
        return o(this, p.addr(), value, loan, end, club_id, other);
    }
    let reg = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add((read_i32(p.player().addr()) as isize * 0x27) as usize);
    let mut regions = vec![(reg + player_stats_entry::CLUB_LEAGUE_GAMES, 4)];
    for cat in 0..6usize {
        let id = read_i32(reg + 4 * cat);
        if id != -1 {
            regions.push((read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add((id as isize * 0x20) as usize), 32));
        }
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    ARCHIVE_STATS_DRY_RUN = true;
    ARCHIVE_STATS_PENDING_TOTALS = (0, 0);
    ARCHIVE_STATS_CLUB_ID = club_id as i32;
    let n = crate::rng::pause_trace(|| archive_player_stats(this, p.addr(), value, loan, end, club_id, other));
    ARCHIVE_STATS_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let r = o(this, p.addr(), value, loan, end, club_id, other);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    SCOREBOARD.record(r == n && mo == mr, || format!("archive_player_stats(pessoa {:x}, valor {}, clube {}) original={} rust={} {:x?} x {:x?}",
        p.addr(), value as i32, club_id as i32, r, n, mo.0, mr.0));
    r
}

/// Comparer for `build_career_history` (comparison mode: see the header of motor.rs): both build the table (the
/// original routine and the copy, from the same state; the history file is only read) and compare the count and the
/// entries.
///
/// The copy's table is freed (`0x9452ca`) and the game carries on with the original's. The error cases (box): only the
/// original routine.
///
/// [pt] Comparador: as duas montam a tabela (a original e a cópia, a partir do mesmo estado; o arquivo de histórico só é
/// lido) e comparam a conta e as entradas; a da cópia é liberada (0x9452ca) e o jogo segue com a da original. Os casos
/// de erro (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_build_career_history(this: usize, p: usize, count: *mut i32) -> usize {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize>(IDX_BUILD_CAREER_HISTORY);
    if p.is_null() || (read_i32(p.addr()) < read_i32(G_STAFF_COUNT) && p.player().is_null()) {
        return o(this, p.addr(), count);
    }
    let r = o(this, p.addr(), count);
    let mut n = 0i32;
    let t = build_career_history(this, p.addr(), &mut n);
    let same = if r == 0 || t == 0 {
        r == t && *count == n
    } else {
        *count == n
            && core::slice::from_raw_parts(r as *const u8, n.max(0) as usize * HISTORY_ENTRY_SIZE)
                == core::slice::from_raw_parts(t as *const u8, n.max(0) as usize * HISTORY_ENTRY_SIZE)
    };
    SCOREBOARD.record(same, || {
        let k = (*count).max(n).max(0) as usize;
        let dif = (0..k).find(|&i| {
            r == 0 || t == 0 || core::slice::from_raw_parts((r + i * HISTORY_ENTRY_SIZE) as *const u8, HISTORY_ENTRY_SIZE) != core::slice::from_raw_parts((t + i * HISTORY_ENTRY_SIZE) as *const u8, HISTORY_ENTRY_SIZE)
        });
        format!("build_career_history(pessoa {:x}) conta original={} rust={}, primeira entrada diferente {:?}", p.addr(), *count, n, dif)
    });
    if t != 0 {
        exe::crt_free(t);
    }
    r
}

/// Comparer for `player_stats_years_since_at_club`.
///
/// [pt] Comparador: a devolução (16 bits); a tabela é montada e liberada pelas duas. Os casos de erro: só a original.
pub(crate) unsafe extern "stdcall" fn compare_player_stats_years_since_at_club(p: usize, club: usize) -> i16 {
    let p = Staff(p);
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> i16>(IDX_PLAYER_STATS_YEARS_SINCE_AT_CLUB);
    if p.is_null() || club == 0 || (read_i32(p.addr()) < read_i32(G_STAFF_COUNT) && p.club_job() != Club(club) && p.player().is_null()) {
        return o(p.addr(), club);
    }
    let r = o(p.addr(), club);
    let n = player_stats_years_since_at_club(p.addr(), club);
    SCOREBOARD.record(r == n, || format!("player_stats_years_since_at_club(pessoa {:x}, clube {:x}) original={} rust={}", p.addr(), club, r, n));
    r
}

/// Comparer for `player_stats_compare_history_by_staff`.
///
/// [pt] Comparador: a devolução (só lê).
pub(crate) unsafe extern "C" fn compare_player_stats_compare_history_by_staff(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_STATS_COMPARE_HISTORY_BY_STAFF)(a, b);
    let n = player_stats_compare_history_by_staff(a, b);
    SCOREBOARD.record(r == n, || format!("player_stats_compare_history_by_staff({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `get_stat`.
pub(crate) unsafe extern "thiscall" fn compare_get_stat(this: usize, player: usize, category: u32, which: u32, default: f32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64>(IDX_GET_STAT);
    if player == 0 || (read_usize(player) as i32) >= read_usize(G_PLAYER_DATA_COUNT) as i32 {
        return o(this, player, category, which, default);
    }
    let c = category as u8 as i8;
    let simples = which as u8 != 0x13 && c <= 6;
    // simplification: category outside 1..6 falls into the box of 0x7c3920; in the combined ones without record 4 or 5, locals without a value
    if (simples && !(1..=6).contains(&c))
        || (!simples && (100..=102).contains(&(c as i32)) && (exe::playerstats_get_category_record(this, player, 4) == 0 || exe::playerstats_get_category_record(this, player, 5) == 0))
    {
        return o(this, player, category, which, default);
    }
    let r = o(this, player, category, which, default);
    let n = get_stat(this, player, category, which, default);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || {
        format!("get_stat(jogador {}, categoria {}, qual {}, padrão {}) original={} rust={}", read_usize(player) as i32, c, which as u8, default, r, n)
    });
    r
}

/// Comparer for `player_stats_compare_language_entries`.
pub(crate) unsafe extern "C" fn compare_player_stats_compare_language_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_STATS_COMPARE_LANGUAGE_ENTRIES)(a, b);
    let n = player_stats_compare_language_entries(a, b);
    SCOREBOARD.record(r == n, || format!("player_stats_compare_language_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `player_stats_compare_history_by_club`.
pub(crate) unsafe extern "C" fn compare_player_stats_compare_history_by_club(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_PLAYER_STATS_COMPARE_HISTORY_BY_CLUB)(a, b);
    let n = player_stats_compare_history_by_club(a, b);
    SCOREBOARD.record(r == n, || format!("player_stats_compare_history_by_club({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `player_stats_get_career_goals`.
///
/// [pt] Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "stdcall" fn compare_player_stats_get_career_goals(person: usize) -> u16 {
    let person = Staff(person);
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(IDX_PLAYER_STATS_GET_CAREER_GOALS);
    if person.is_null() || (read_i32(person.addr()) < read_i32(G_STAFF_COUNT) && person.player().is_null()) {
        return o(person.addr());
    }
    let r = o(person.addr());
    let n = player_stats_get_career_goals(person.addr());
    SCOREBOARD.record(r == n, || format!("player_stats_get_career_goals(pessoa {:x}) original={} rust={}", person.addr(), r, n));
    r
}

/// Comparer for `get_category_record`.
///
/// [pt] Comparador: pelo valor (só lê); os casos de erro só na original.
pub(crate) unsafe extern "thiscall" fn compare_get_category_record(this: usize, player: usize, category: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CATEGORY_RECORD)(this, player, category);
    if category_slot(this, player, category).is_none() {
        return r;
    }
    let n = get_category_record(this, player, category);
    SCOREBOARD.record(r == n, || format!("get_category_record({:x}, {:x}, {}) original={:x} rust={:x}", this, player, category as u8 as i8, r, n));
    r
}

/// Comparer for `create_stats`.
///
/// The original routine reallocates the vector, so only it runs; beforehand, the plan (the index the player will get,
/// the count and the free ones afterwards); afterwards, the returned record, the index in the slot, the count, the free
/// ones and the record zeroed when it is new.
///
/// Simplification: the error cases and running out of memory only in the original, unchecked.
///
/// [pt] Comparador: a original realoca o vetor, então só ela roda; antes, o plano (o índice que o jogador vai ter, a
/// conta e os soltos depois); depois, o registro devolvido, o índice na vaga, a conta, os soltos e o registro
/// zerado quando é novo. simplification: os casos de erro e a falta de memória só na original, sem conferir.
pub(crate) unsafe extern "thiscall" fn compare_create_stats(this: usize, player: usize, category: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_CREATE_STATS);
    let slot = match category_slot(this, player, category) {
        Some(v) => v,
        _ => return o(this, player, category),
    };
    let current = read_usize(slot) as i32;
    let (n, soltos) = (read_usize(this + player_stats_fields::CATEGORY_COUNT), read_usize(this + player_stats_fields::FREE_CATEGORY_COUNT));
    let solto = if current == -1 && soltos != 0 {
        (0..n as i32).find(|&i| core::ptr::read_unaligned((read_usize(this + player_stats_fields::CATEGORY_RECORDS) + ((i as usize) << 5) + 0xe) as *const u16) == 0xffff)
    } else {
        None
    };
    let (indice, n2, soltos2) = match (current, solto) {
        (-1, Some(i)) => (i, n, soltos.wrapping_sub(1)),
        (-1, None) => (n as i32, n.wrapping_add(1), soltos),
        (i, _) => (i, n, soltos),
    };
    let r = o(this, player, category);
    if r == 0 {
        return r;
    }
    let expected = read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add((indice as usize) << 5);
    let zerado = current != -1 || (0..0x20).all(|k| *((r + k) as *const u8) == 0);
    let same = r == expected && read_usize(slot) as i32 == indice && read_usize(this + player_stats_fields::CATEGORY_COUNT) == n2 && read_usize(this + player_stats_fields::FREE_CATEGORY_COUNT) == soltos2 && zerado;
    SCOREBOARD.record(same, || format!("create_stats({:x}, {:x}, {}) original={:x} plano={:x} índice {} x {} conta {} x {} soltos {} x {} zerado={}",
        this, player, category as u8 as i8, r, expected, read_usize(slot) as i32, indice, read_usize(this + player_stats_fields::CATEGORY_COUNT), n2, read_usize(this + player_stats_fields::FREE_CATEGORY_COUNT), soltos2, zerado));
    r
}

/// Comparer for `add_language` (`add_language`; comparison mode: see the header of motor.rs).
///
/// It allocates and reallocates the table (it cannot run twice): the plan beforehand (the count and capacity
/// afterwards, the fluency), then the original runs; it checks the count, the capacity and the new record (at the end
/// when not sorting; with `ordenar`, somewhere in the table).
///
/// [pt] Comparador: a 0x7c6a30 aloca e realoca a tabela (não pode rodar duas vezes): o plano antes (a conta e a
/// capacidade depois, a fluência), a original roda; confere a conta, a capacidade e o registro novo (no fim sem ordenar;
/// com `ordenar`, em algum lugar da tabela).
pub(crate) unsafe extern "thiscall" fn compare_add_language(this: usize, person: usize, language: u32, fluencia: u32, sort: u32) {
    let (n, cap) = if read_usize(this + player_stats_fields::LANGUAGES) == 0 { (read_usize(this + player_stats_fields::LANGUAGE_COUNT), 100) } else { (read_usize(this + player_stats_fields::LANGUAGE_COUNT), read_usize(this + player_stats_fields::LANGUAGE_CAPACITY)) };
    let cap = if cap == n { cap.wrapping_add(100) } else { cap };
    let f = fluencia as u8;
    let new = (read_usize(person) as u32, language as u8, if (f as i8) > 10 { 10 } else { f });
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_ADD_LANGUAGE)(this, person, language, fluencia, sort);
    let base = read_usize(this + player_stats_fields::LANGUAGES);
    let e = |i: usize| {
        let r = base.wrapping_add(i * 6);
        (read_usize(r) as u32, *((r + language_entry::LANGUAGE) as *const u8), *((r + language_entry::FLUENCY) as *const u8))
    };
    let count = read_usize(this + player_stats_fields::LANGUAGE_COUNT);
    let mut ok = count == n.wrapping_add(1) && read_usize(this + player_stats_fields::LANGUAGE_CAPACITY) == cap;
    if ok {
        ok = if sort == 0 { e(n) == new } else { (0..count).any(|i| e(i) == new) };
    }
    SCOREBOARD.record(ok, || format!("add_language({:?}, ordenar {}) conta {} (esperava {}) capacidade {} (esperava {})",
        new, sort, count, n.wrapping_add(1), read_usize(this + player_stats_fields::LANGUAGE_CAPACITY), cap));
}

/// Comparer for `improve_language`.
///
/// [pt] Comparador: sem tabela ou sem o registro, só a original (o acréscimo 0x7c6a30 tem o comparador dele); com o
/// registro, lado a lado nos 6 bytes dele.
pub(crate) unsafe extern "thiscall" fn compare_improve_language(this: usize, person: usize, language: u32, aumento: u32, sort: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_IMPROVE_LANGUAGE);
    let r = if read_usize(this + player_stats_fields::LANGUAGES) == 0 { 0 } else { find_language_record(this, person, language as u8) };
    if r == 0 {
        return o(this, person, language, aumento, sort);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r, 6)],
        || o(this, person, language, aumento, sort), || improve_language(this, person, language, aumento, sort));
    SCOREBOARD.record(igual, || format!("improve_language({}, idioma {}, +{}) {:x?} x {:x?}", read_usize(person), language as u8, aumento as u8, mo, mr));
}

/// Comparer for `player_stats_reset_player`.
///
/// [pt] Comparador: lado a lado no registro de 39 bytes, na conta dos livres (+0x38) e na marca +0xe dos registros de
/// categoria liberados (a 0x58f110 só grava +0x1c, dentro do registro). Pessoa nula ou não jogadora: só a original.
pub(crate) unsafe extern "thiscall" fn compare_player_stats_reset_player(this: usize, person: usize) -> usize {
    let person = Staff(person);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_STATS_RESET_PLAYER);
    if person.is_null() || person.player().is_null() {
        return o(this, person.addr());
    }
    let r = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add(read_usize(person.player().addr()).wrapping_mul(39)); // the id of the player profile (+0x61), not the person’s
    let mut reg = vec![(r, 39), (this + player_stats_fields::FREE_CATEGORY_COUNT, 4)];
    for k in 0..6usize {
        let i = read_usize(r + 4 * k);
        if i as u32 != 0xffff_ffff {
            reg.push((read_usize(this + player_stats_fields::CATEGORY_RECORDS).wrapping_add(i << 5) + 0xe, 2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person.addr()), || player_stats_reset_player(this, person.addr()));
    SCOREBOARD.record(igual && a == b, || format!("player_stats_reset_player({}) {:x?} x {:x?}", read_usize(person.addr()), mo, mr));
    a
}

/// Comparer for `get_language_fluency`.
///
/// [pt] Comparador: pelo valor (a 0x4cf5f0 e a busca só leem).
pub(crate) unsafe extern "thiscall" fn compare_get_language_fluency(this: usize, person: usize, club: usize, language: u32, by_contract: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(IDX_GET_LANGUAGE_FLUENCY)(this, person, club, language, by_contract);
    let n = get_language_fluency(this, person, club, language, by_contract);
    SCOREBOARD.record(r == n, || format!("get_language_fluency({:x}, {:x}, {}, {}) original={} rust={}", person, club, language as u8, by_contract, r, n));
    r
}

/// Comparer for `player_stats_record_match` (comparison mode: see the header of motor.rs): side-by-side comparer.
///
/// Compares the double sums (`+0x10`..`+0x30`), the category records of each row that counts, the range
/// `+0x20`..`+0x27` of the player's record and `+0x3b` of the contract; in the simulated match (competition −2), also
/// the random-draw state (and the two possible categories), with the copy's random draw left out of the count. A null
/// match or fixture, the box of a competition with no known type, or a category record still to be created (`0x7c5d80`
/// grows the vector): only the original routine.
///
/// [pt] Comparador lado a lado: as somas em double (+0x10..+0x30), os registros das categorias de cada linha que conta, o
/// trecho +0x20..+0x27 do registro do jogador e o +0x3b do contrato; no jogo simulado (competição −2), também o estado
/// do sorteio (e as duas categorias possíveis), com o sorteio da cópia fora da contagem. Partida ou jogo nulos, a caixa
/// de uma competição sem tipo conhecido ou registro de categoria a criar (0x7c5d80 cresce o vetor): só a original.
pub(crate) unsafe extern "thiscall" fn compare_player_stats_record_match(this: usize, p: usize, game: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PLAYER_STATS_RECORD_MATCH);
    if p == 0 || game == 0 {
        return o(this, p, game); // simplification: the boxes, original only
    }
    let simulado = read_usize(p) as i32 == -2;
    let mut regions = vec![(this + player_stats_fields::RATING_SUM, 0x20)];
    for lado in 0..2usize {
        let cats: Vec<u8> = if simulado {
            vec![2, 3, 6]
        } else {
            let mut error = false;
            let c = match_stats_category(Fixture(p), lado as i32, false, &mut error);
            if error {
                return o(this, p, game); // simplification: the box, original only
            }
            stats_categories_for(c).into_iter().flatten().collect()
        };
        for k in 0..20usize {
            let Some((e, pessoa, dados)) = match_lineup_row(game, lado, k) else { continue };
            for &c in &cats {
                let r = player_category_record(this, dados, c);
                if r == 0 {
                    return o(this, p, game); // simplification: the record would be created, original only
                }
                regions.push((r, 0x20));
            }
            regions.push((read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add(read_usize(e + lineup_row::PLAYER_ID).wrapping_mul(0x27)) + 0x20, 7));
            let ct = contract_of_staff(pessoa);
            if ct != 0 {
                regions.push((ct + 0x3b, 2));
            }
        }
    }
    if simulado {
        regions.extend(crate::rng::STATE_REGIONS);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, p, game), || {
        crate::rng::without_counting(|| player_stats_record_match(this, p, game))
    });
    SCOREBOARD.record(igual && ro == rr, || format!("player_stats_record_match({:x}, {:x}, {:x}) original={} {:x?} rust={} {:x?}", this, p, game, ro, mo, rr, mr));
    ro
}

/// Comparer for `end_season_for_key_nation`.
///
/// [pt] Comparador: a 0x7c5fe0 move as estatísticas para o histórico (aloca): roda só a original e confere a devolução
/// (1, ou 0 sem nação).
pub(crate) unsafe extern "thiscall" fn compare_end_season_for_key_nation(this: usize, nation: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_END_SEASON_FOR_KEY_NATION)(this, nation);
    SCOREBOARD.record(r == (nation != 0) as u32, || format!("end_season_for_key_nation({:x}) devolveu {}", nation, r));
    r
}

/// Comparer for `remove_language`.
pub(crate) unsafe extern "thiscall" fn compare_remove_language(stats: usize, sheet: usize, language: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8)>(IDX_REMOVE_LANGUAGE);
    let table = *((stats + player_stats_fields::LANGUAGES) as *const usize);
    let count = *((stats + player_stats_fields::LANGUAGE_COUNT) as *const i32);
    if table == 0 || count <= 0 {
        return o(stats, sheet, language);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(stats + player_stats_fields::LANGUAGE_COUNT, 4), (table, count as usize * 6)],
        || o(stats, sheet, language), || remove_language(stats, sheet, language));
    SCOREBOARD.record(igual, || format!("remove_language({}) contagem {:x?} x {:x?}", language, mo.0[0].1, mr.0[0].1));
}

/// Comparer for `get_club_league_stat`.
pub(crate) unsafe extern "thiscall" fn compare_get_club_league_stat(stats: usize, player: usize, field: i8) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f32>(IDX_GET_CLUB_LEAGUE_STAT)(stats, player, field);
    if player == 0 || !(1..=2).contains(&field) {
        return r;
    }
    let n = get_club_league_stat(stats, player, field);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("get_club_league_stat({}) original={} rust={}", field, r, n));
    r
}

/// Comparer for `get_last_match_rating`.
pub(crate) unsafe extern "thiscall" fn compare_get_last_match_rating(stats: usize, id: *const i32) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(IDX_GET_LAST_MATCH_RATING)(stats, id);
    if id.is_null() {
        return r;
    }
    let n = get_last_match_rating(stats, id);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("get_last_match_rating({}) original={} rust={}", *id, r, n));
    r
}

/// Comparer for `player_stats_get_goals_for_club`.
///
/// [pt] Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "stdcall" fn compare_player_stats_get_goals_for_club(person: usize, club: usize) -> u16 {
    let person = Staff(person);
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u16>(IDX_PLAYER_STATS_GET_GOALS_FOR_CLUB);
    if person.is_null() || club == 0 || (read_i32(person.addr()) < read_i32(G_STAFF_COUNT) && person.player().is_null()) {
        return o(person.addr(), club);
    }
    let r = o(person.addr(), club);
    let n = player_stats_get_goals_for_club(person.addr(), club);
    SCOREBOARD.record(r == n, || format!("player_stats_get_goals_for_club(pessoa {:x}, clube {:x}) original={} rust={}", person.addr(), club, r, n));
    r
}

/// Comparer for `competition_weight`.
pub(crate) unsafe extern "stdcall" fn compare_competition_weight(p: usize, code: i8) -> u16 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, i8) -> u16>(IDX_COMPETITION_WEIGHT)(p, code);
    let n = competition_weight(p, code);
    SCOREBOARD.record(r == n, || format!("competition_weight({:#x}, {}) original={} rust={}", p, code, r, n));
    r
}
