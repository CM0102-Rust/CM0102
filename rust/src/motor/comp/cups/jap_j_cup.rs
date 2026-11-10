use super::*;

/// Writes one cup round (a 0x68-byte record, docs/.md §2.4).
///
/// The draw (day, month with 0 = January, years to add to the cup year +0x40, weekday or −1) goes through `cup_set_round_draw_date`; the
/// match (the same, plus the leg at +6 and the venue at +0x13) through `cup_set_round_match_date`.
///
/// Then the fields: +7 the round name, +9 the index, +0xb the draw bits, +0xd and +0xf the tie-breakers (single match
/// and two legs), +0x17 the category copied into each match, +0x18/+0x1a/+0x1c/+0x1e teams, matches, new teams and
/// entrants already in, +0x20/+0x21/+0x22 replay, matches per tie and days between them, +0x5c/+0x60/+0x64 the prize
/// money (reaching, winning, losing).
///
/// [pt] Uma rodada de copa (registro de 0x68 bytes, docs/.md §2.4): o sorteio (dia, mês com 0 =
/// janeiro, anos a somar ao ano da copa +0x40, dia da semana ou −1) pela 0x521e60; o jogo (o mesmo, mais o turno em +6
/// e o mando em +0x13) pela 0x521eb0; depois os campos: +7 o nome da rodada, +9 o índice, +0xb os bits do sorteio, +0xd
/// e +0xf os desempates (jogo único e ida e volta), +0x17 a categoria copiada em cada jogo, +0x18/+0x1a/+0x1c/+0x1e
/// times, jogos, times novos e inscritos que já entraram, +0x20/+0x21/+0x22 replay, jogos por confronto e dias entre
/// eles, +0x5c/+0x60/+0x64 os prêmios (chegar, vencer, perder).
#[derive(Clone, Copy)]
pub(crate) struct JapJCupCupRound {
    pub(crate) draw: (u32, u32, u32, u32),
    pub(crate) game: (u32, u32, u32, u32, u32, u32),
    pub(crate) code: u16,
    pub(crate) bits: (u16, u16, u16),
    pub(crate) category: u8,
    pub(crate) teams: (u16, u16, u16, u16),
    pub(crate) tie: (u8, u8, u8),
    pub(crate) prizes: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
pub(crate) const fn cqn11r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> JapJCupCupRound {
    JapJCupCupRound { draw: sorteio, game: jogo, code: codigo, bits, category: categoria, teams: times, tie: confronto, prizes: premios }
}

/// Writes cup round i of table t (0x68 bytes): draw and match dates (`cup_set_round_draw_date`, `cup_set_round_match_date`) plus code, bits, category,
/// team counts, tie settings and prizes from a static record.
///
/// [pt] Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
pub(crate) unsafe fn jap_j_cup_write_cup_round(t: usize, i: u32, r: &JapJCupCupRound, year: u32) {
    exe::cup_set_round_draw_date(t, i, r.draw.0, r.draw.1, r.draw.2, r.draw.3, year);
    exe::cup_set_round_match_date(t, i, r.game.0, r.game.1, r.game.2, r.game.3, r.game.4, year, r.game.5);
    let e = t + i as usize * 0x68;
    let w = |o: usize, v: u16| write_u16(e + o, v);
    let d = |o: usize, v: u32| write_u32(e + o, v);
    w(7, r.code);
    w(9, i as u16);
    w(0xb, r.bits.0);
    w(0xd, r.bits.1);
    w(0xf, r.bits.2);
    write_u8(e + 0x17, r.category);
    w(0x18, r.teams.0);
    w(0x1a, r.teams.1);
    w(0x1c, r.teams.2);
    w(0x1e, r.teams.3);
    write_u8(e + 0x20, r.tie.0);
    write_u8(e + 0x21, r.tie.1);
    write_u8(e + 0x22, r.tie.2);
    d(0x5c, r.prizes.0);
    d(0x60, r.prizes.1);
    d(0x64, r.prizes.2);
}

/// Shared body of virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) of the
/// single-stage cups: only stage −1 builds the round table.
///
/// Outside stage −1 (the byte), returns 0 without writing anything. In −1: saida (if requested) = 0, rodadas = the
/// table size, extra = 0; the table (`exe::crt_malloc` of 0x68 × rodadas; when out of memory, the error box `linha` of `arquivo`
/// and returns 0) is filled with each round.
///
/// Returns the table.
///
/// [pt] O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela.
pub(crate) unsafe fn build_single_stage_cup_rounds(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[JapJCupCupRound], file: usize, row: u32, name: &'static str) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = table.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    crate::comparison::branch(name);
    let year = this.year() as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        jap_j_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// Side-by-side comparer of create_round_table: outputs and written round bytes must match; frees the port's table;
/// other stages only run the original and expect 0.
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada).
pub(crate) unsafe fn compare_cup_round_table(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if stage as u8 != 0xff {
        // simplification: outside stage −1 neither of the two writes anything; the outputs may not even exist
        let a = o();
        SCOREBOARD.record(a == 0, || format!("{}(stage {}) returned {:#x}", name, stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): saídas {} ({} x {}) table {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Copa da Liga japonesa (J League Cup): cinco rodadas de fevereiro a novembro, as quatro primeiras em ida e volta (7
/// dias entre os jogos, desempate de ida e volta 0x83).
pub(crate) const J_LEAGUE_CUP_ROUNDS: [JapJCupCupRound; 5] = [
    cqn11r((0xa, 1, 0, 0xffff_ffff), (0xc, 3, 0, 2, 2, 0), 0xa, (0, 0, 0x83), 4, (0x18, 0xc, 0x18, 0), (0, 2, 7), (0, 0xffff_ffff, 0)), // cqn11: the 1st round of the Japanese League Cup
    cqn11r((0x14, 3, 0, 3), (5, 6, 0, 2, 2, 0), 0x14, (0, 0, 0x83), 4, (0x10, 8, 4, 0x18), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((0xd, 6, 0, 3), (0x1e, 7, 0, 2, 2, 0), 0x78, (0, 0, 0x83), 4, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((7, 8, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x82, (0, 0, 0x83), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cqn11r((0x13, 9, 0, 3), (4, 0xa, 0, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// rule: the J League Cup has 5 rounds, the first four over two legs (7 days apart): 1st round (24 teams) drawn on 10/2
/// (no fixed weekday) and played on 12/4; 2nd (16 teams, 4 enter) 20/4 and 5/7; quarter-finals 13/7 and 30/8;
/// semi-final 7/9 and 11/10; single-match final, drawn 19/10, played 4/11 at a neutral ground [inference: venue 4]; no
/// prize per round (−1); category 4 up to the quarter-finals and 6 after
/// Original: `JapJCup::create_round_table` (0x665640, jap_j_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x665640 } (thiscall, ret 0x10, jap_j_cup.cpp; a virtual +0x3c da Copa da Liga japonesa): `JapJCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0x9e sem memória.
pub(crate) unsafe extern "thiscall" fn jap_j_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_single_stage_cup_rounds(Cup(this), stage, rounds, extra, out, &J_LEAGUE_CUP_ROUNDS, 0x9eaf14, 0x9e, "cqn_11 tabela da copa da liga japonesa")
}

/// A Copa do Imperador: sete rodadas em jogo único de novembro ao começo de janeiro do ano seguinte, sorteio por sementes
/// com grupos separados (+0xd = 0x83).
pub(crate) const EMPERORS_CUP_ROUNDS: [JapJCupCupRound; 7] = [
    cqn11r((1, 0xa, 0, 6), (0x1d, 0xa, 0, 6, 1, 0), 0xa, (0, 0x83, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cqn11: the 1st round of the Emperor’s Cup
    cqn11r((0x1e, 0xa, 0, 0), (6, 0xb, 0, 6, 1, 0), 0x14, (0, 0x83, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((7, 0xb, 0, 0), (0xd, 0xb, 0, 6, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x10, 0x40), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xe, 0xb, 0, 0), (0x14, 0xb, 0, 6, 1, 0), 0x28, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x15, 0xb, 0, 0), (0x17, 0xb, 0, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x18, 0xb, 0, 3), (0x1b, 0xb, 0, 6, 1, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x1c, 0xb, 0, 0), (2, 0, 1, 5, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The US Open Cup: five single-match rounds from June to October; in the 1st round the draw also carries bit 0x400
/// (+0xd = 0x483).
///
/// [pt] A Copa dos EUA (US Open Cup): cinco rodadas em jogo único de junho a outubro; na 1ª rodada o sorteio leva também o
/// bit 0x400 (+0xd = 0x483).
pub(crate) const US_OPEN_CUP_ROUNDS: [JapJCupCupRound; 5] = [
    cqn11r((1, 5, 0, 3), (0xe, 5, 0, 2, 2, 0), 0x14, (0, 0x483, 0), 4, (0x20, 0x10, 0x20, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xf, 5, 0, 3), (0x19, 5, 0, 6, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x10, 8, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0x1a, 5, 0, 0), (9, 7, 0, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cqn11r((0xa, 7, 0, 3), (0x17, 7, 0, 2, 2, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqn11: the US Cup semifinal
    cqn11r((0x18, 7, 0, 3), (0x16, 9, 0, 6, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// Club record i in the club table (0x245 bytes each).
///
/// [pt] O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
pub(crate) unsafe fn jap_j_cup_club_by_index(i: i32) -> usize {
    club_at(i as isize).addr()
}

/// Random 0..n-1 (`exe::utils_random`).
///
/// [pt] O sorteio de 0 a n − 1 (0x90cfc0: push n; add esp, 4 -> cdecl).
pub(crate) unsafe fn jap_j_cup_random(n: i32) -> i32 {
    exe::utils_random(n)
}

/// Sorts n clubs by reputation and shuffles the top `escolhas` (comp_util_sort_and_shuffle_top_clubs `comp_util_sort_and_shuffle_top_clubs`).
///
/// [pt] Ordena `n` clubes do vetor pela reputação e embaralha os melhores (0x4afbd0: push escolhas; push n; push vetor; add
/// esp, 0xc -> cdecl; já em Rust).
pub(crate) unsafe fn sort_and_shuffle_top_clubs(v: *mut usize, n: i32, escolhas: u32) {
    // simplification: the original pushes the count with garbage in the high 16 bits; 0x4afbd0 reads the word
    exe::comp_util_sort_and_shuffle_top_clubs(v as usize, n as i16 as u16 as u32, escolhas);
}

/// CRT qsort of n 6-byte entrants with comparator `cup_compare_entries_by_level_desc`.
///
/// [pt] O qsort de `n` inscritos a partir de `base` (0x945333 com 0x522190: push 0x522190; push 6; push n; push base; add
/// esp, 0x10 -> cdecl; 0x522190 já em Rust).
pub(crate) unsafe fn sort_entrants(base: usize, n: u32) {
    exe::crt_qsort(base, n, 6, 0x522190);
}

/// `JapJCup::collect_entrants()` (`jap_j_cup_collect_entrants`, thiscall, ret 0, jap_j_cup.cpp): 28 entrants (+0x3e = 28); the list
/// (`exe::crt_malloc` of 0xa8 at +0xa7; out of memory, the box for line 0x11a and −1).
///
/// The J2 clubs ([0x9cf768]) while they fit, class 0; the J1 clubs ([0x9cf65c]) while they fit, each with a random
/// class from 0 to 19 (`exe::utils_random`(20), after writing the club); qsort (`cup_compare_entries_by_level_desc`) of the 16 entries from the 11th (+0x42).
///
/// Returns 1 with the list full, otherwise 0 (as the result and count pair).
///
/// [pt] { 0x665a00 } (thiscall, ret 0, jap_j_cup.cpp): `JapJCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de 0xa8 em
/// +0xa7; sem memória, a caixa da linha 0x11a e −1). Os clubes da J2 ([0x9cf768]) enquanto cabem, classe 0; os da J1
/// ([0x9cf65c]) enquanto cabem, cada um com a classe sorteada de 0 a 19 (0x90cfc0(20), depois de gravar o clube); o
/// qsort (0x522190) das 16 inscrições a partir da 11ª (+0x42). Devolve 1 com a lista cheia, senão 0.
pub(crate) unsafe fn collect_j_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x1c);
    let l = exe::crt_malloc(0xa8);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0x9eaf14, 0x11a);
        return (-1, 0);
    }
    let mut n = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if comp_id_at(c, 0x57) == Some(read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2)) && n < this.team_count() as i32 {
            eng_cc_cup_write_entrant(this, n, c, 0);
            n += 1;
            if n == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if comp_id_at(c, 0x57) == Some(read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1)) && n < this.team_count() as i32 {
            let e = this.team_table() + n as usize * 6;
            write_u32(e, c as u32);
            crate::comparison::branch("cqn_12 j1 seed");
            let s = jap_j_cup_random(0x14); // cqn12: the drawn class of the J1 clubs
            write_u8(e + 4, s as u8);
            write_u8(e + 5, 0);
            n += 1;
            if n == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    sort_entrants(this.team_table() + 0x42, 0x10);
    ((n == this.team_count() as i32) as i32, n)
}

/// rule: the J League Cup takes 28 clubs: the J2 ones, class 0, and the J1 ones, each with a class drawn from 0 to 19
/// (the seed); the 16 entries from the 11th on are sorted by class (ties by reputation); with an incomplete list it
/// returns 0
/// Original: `JapJCup::collect_entrants` (0x665a00, jap_j_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Liga japonesa 28 clubes: os da J2, classe 0, e os da J1, cada um com uma classe sorteada de
/// 0 a 19 (a semente); as 16 inscrições a partir da 11ª vão ao qsort pela classe (no empate a reputação); com a lista
/// incompleta, devolve 0
pub(crate) unsafe extern "thiscall" fn jap_j_cup_collect_entrants(this: usize) -> i32 {
    collect_j_cup_entrants(Cup(this)).0
}

/// Side-by-side comparer for routines that allocate the entrant list +0xa7 and draw randoms: compares +0x3e,
/// [0xb67a34], RNG state and the entrants written; frees the port's list.
///
/// [pt] Lado a lado para as rotinas que alocam a lista +0xa7 e sorteiam: o +0x3e, o [0xb67a34] e o estado do sorteador a
/// partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xa7, a cópia faz a sua,
/// comparada nos inscritos que ela gravou e liberada.
pub(crate) unsafe fn jap_j_cup_compare_entrant_list(name: &str, this: Cup, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let (r, n) = crate::rng::without_counting(c);
        lr.set(this.team_table());
        nr.set(n);
        this.set_team_table((lo.get() as u32) as usize);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 6;
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", name, this.addr(), a, b, igual, list, k));
    a
}

/// `UsaOpenCup::collect_entrants()` (`usa_open_cup_collect_entrants`, thiscall, ret 0, usa_open_cup.cpp): 32 entrants.
///
/// +0x3e = 32. Clubs whose division (+0x57, the pointer) is the database competition of [0x9cf594] or that of
/// [0x9cf598] go into two local arrays of up to 50 each (the pass stops when both are full); each array is sorted and
/// shuffled (`comp_util_sort_and_shuffle_top_clubs`, 10 picks).
///
/// The list (`exe::crt_calloc`, the calloc of +0x3e × 6, at +0xa7; out of memory, the error box for line 0x138 and −1) gets the
/// first 10 of each array with class 2 and then all the MLS clubs ([0x9cf590]) with class 1 (without checking the space
/// left). Returns 1.
///
/// [pt] { 0x90b530 } (thiscall, ret 0, usa_open_cup.cpp): `UsaOpenCup::collect_entrants()`: +0x3e = 32. Os clubes cuja
/// divisão (+0x57, o ponteiro) é a competição do banco de [0x9cf594] ou a de [0x9cf598] vão para dois vetores locais de
/// até 50 (a passada para quando os dois enchem); cada vetor é ordenado e embaralhado (0x4afbd0, 10 escolhas). A lista
/// (0x945501, o calloc de +0x3e × 6, em +0xa7; sem memória, a caixa da linha 0x138 e −1) leva os 10 primeiros de cada
/// vetor com a classe 2 e depois todos os clubes da MLS ([0x9cf590]) com a classe 1 (sem conferir o espaço). Devolve 1.
pub(crate) unsafe fn collect_us_open_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x20);
    let comp = |id: usize| read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(id).wrapping_mul(0x6b));
    let (a, b) = (comp(KEY_CLUB_COMP_AMERICAN_A_LEAGUE), comp(KEY_CLUB_COMP_AMERICAN_USISL));
    // simplification: the original’s local vectors start with whatever was on the stack; with fewer than 10 clubs in one of the
    // leagues it would copy that leftover (and 0x4afbd0 would already show the box): the copy starts with zero
    let mut va = [0usize; 50];
    let mut vb = [0usize; 50];
    let (mut na, mut nb) = (0i8, 0i8);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if na == 0x32 && nb == 0x32 {
            break;
        }
        let c = jap_j_cup_club_by_index(i);
        let d = read_usize(c + 0x57);
        if d == a && na < 0x32 {
            va[na as usize] = c;
            na += 1;
        } else if d == b && nb < 0x32 {
            vb[nb as usize] = c;
            nb += 1;
        }
        i += 1;
    }
    sort_and_shuffle_top_clubs(va.as_mut_ptr(), na as i32, 10);
    sort_and_shuffle_top_clubs(vb.as_mut_ptr(), nb as i32, 10);
    let l = exe::crt_calloc(this.team_count() as i32 as u32, 6);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0xaa20e0, 0x138);
        return (-1, 0);
    }
    let mut k = 0i8;
    for &c in va.iter().take(10).chain(vb.iter().take(10)) {
        eng_cc_cup_write_entrant(this, k as i32, c, 2); // cqn12: the 10 from each lower league
        k = k.wrapping_add(1);
    }
    crate::comparison::branch("cqn_12 us cup entrants");
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let c = jap_j_cup_club_by_index(i);
        if comp_id_at(c, 0x57) == Some(read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) {
            eng_cc_cup_write_entrant(this, k as i32, c, 1);
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    (1, k as i32)
}

/// `JapEmpCup::collect_entrants()` (`jap_emp_cup_collect_entrants`, thiscall, ret 0, jap_emp_cup.cpp): the 80 entrants of the Emperor's Cup,
/// from the Japanese lower divisions (shuffled), then the J2 and J1 clubs.
///
/// Sets +0x3e = 80 and allocates the list (`exe::crt_malloc` of 0x1e0 at +0xa7; when out of memory, the error box for line 0x147
/// and −1).
///
/// From the Japanese clubs (+0x53 = [0x9cf36c]) that have a division, into local vectors: those of [0x9cf678] (up to
/// 100), those of [0x9cf674] (up to 30), those of [0x9cf670] (up to 30) and those of divisions other than J2
/// ([0x9cf768]) and J1 ([0x9cf65c]) (up to 6); if there are none of the latter, the last club of [0x9cf678] is moved
/// from there to here. With fewer than 47, 2, 3 or 1 in each of them, the error box for line 0x178 and 0.
///
/// Each vector is sorted and shuffled (`comp_util_sort_and_shuffle_top_clubs` with 47, 2, 3 and 1 picks); the list takes the first 47, 2 and 3 (the
/// last vector is not used), class 0; then the J2 clubs while they fit (class 0) and the J1 clubs while they fit, with
/// class = last season's position (+0x5f), until the list is full; then qsort (`cup_compare_entries_by_level_desc`) of the last 16 entries.
///
/// Returns 1 with the list full, otherwise 0.
///
/// [pt] { 0x6624f0 } (thiscall, ret 0, jap_emp_cup.cpp): `JapEmpCup::collect_entrants()`: +0x3e = 80; a lista (0x944e46 de
/// 0x1e0 em +0xa7; sem memória, a caixa da linha 0x147 e −1). Dos clubes japoneses (+0x53 = [0x9cf36c]) com divisão, em
/// vetores locais: os da [0x9cf678] (até 100), os da [0x9cf674] (até 30), os da [0x9cf670] (até 30) e os de outras
/// divisões que não a J2 ([0x9cf768]) nem a J1 ([0x9cf65c]) (até 6); sem nenhum destes, o último da [0x9cf678] sai de lá
/// para cá. Com menos de 47, 2, 3 ou 1 em cada um, a caixa da linha 0x178 e 0. Cada vetor é ordenado e embaralhado
/// (0x4afbd0 com 47, 2, 3 e 1 escolhas); a lista leva os 47, os 2 e os 3 primeiros (o último vetor não entra), classe 0;
/// depois os da J2 enquanto cabem (classe 0) e os da J1 enquanto cabem, com a classe = a posição da temporada anterior
/// (+0x5f), até encher; o qsort (0x522190) das 16 últimas inscrições. Devolve 1 com a lista cheia, senão 0.
pub(crate) unsafe fn collect_emperors_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x50);
    let l = exe::crt_malloc(0x1e0);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0x9eab90, 0x147);
        return (-1, 0);
    }
    let mut va = [0usize; 100];
    let mut vb = [0usize; 30];
    let mut vc = [0usize; 30];
    let mut vd = [0usize; 6];
    let (mut na, mut nb, mut nc, mut nd) = (0i8, 0i8, 0i8, 0i8);
    if read_usize(G_CLUB_COUNT) as i32 > 0 {
        let mut i: i16 = 0;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let c = jap_j_cup_club_by_index(i as i32);
            if read_usize(c + 0x57) != 0 && comp_id_at(c, 0x53) == Some(read_usize(KEY_NATION_JAPAN)) {
                let d = read_usize(read_usize(c + 0x57));
                if d == read_usize(KEY_CLUB_COMP_JAPANESE_ADMINISTRATIVE_DIVISION) {
                    if na < 0x64 {
                        va[na as usize] = c;
                        na += 1;
                    }
                } else if d == read_usize(KEY_CLUB_COMP_JAPANESE_UNIVERSITY_LEAGUE) {
                    if nb < 0x1e {
                        vb[nb as usize] = c;
                        nb += 1;
                    }
                } else if d == read_usize(KEY_CLUB_COMP_JAPANESE_FOOTBALL_LEAGUE) {
                    if nc < 0x1e {
                        vc[nc as usize] = c;
                        nc += 1;
                    }
                } else if d != read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2) && d != read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1) && nd < 6 {
                    vd[nd as usize] = c;
                    nd += 1;
                }
            }
            i = i.wrapping_add(1);
        }
        if nd == 0 {
            na = na.wrapping_sub(1);
            nd = 1;
            vd[0] = if na >= 0 { va[na as usize] } else { 0 };
        }
    } else {
        // simplification: with no clubs the original reads the vector out of place (index −1); the copy uses zero and falls into the box below
        na = na.wrapping_sub(1);
        nd = 1;
        vd[0] = 0;
    }
    if na < 0x2f || nb < 2 || nc < 3 || nd < 1 {
        show_error_at(0x9eab90, 0x178);
        return (0, 0);
    }
    sort_and_shuffle_top_clubs(va.as_mut_ptr(), na as i32, 0x2f);
    sort_and_shuffle_top_clubs(vb.as_mut_ptr(), nb as i32, 2);
    sort_and_shuffle_top_clubs(vc.as_mut_ptr(), nc as i32, 3);
    sort_and_shuffle_top_clubs(vd.as_mut_ptr(), nd as i32, 1);
    crate::comparison::branch("cqn_12 emperor cup entrants");
    let mut n = 0i32;
    for &c in va.iter().take(0x2f).chain(vb.iter().take(2)).chain(vc.iter().take(3)) {
        eng_cc_cup_write_entrant(this, n, c, 0); // cqn12: those drawn from the lower divisions
        n += 1;
    }
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if comp_id_at(c, 0x57) == Some(read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2)) && n < this.team_count() as i32 {
            eng_cc_cup_write_entrant(this, n, c, 0);
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = jap_j_cup_club_by_index(i as i32);
        if comp_id_at(c, 0x57) == Some(read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1)) && n < this.team_count() as i32 {
            crate::comparison::branch("cqn_12 j1 class in emperor cup");
            eng_cc_cup_write_entrant(this, n, c, read_u8(c + 0x5f)); // cqn12: the class of the J1 clubs in the Emperor’s Cup
            n += 1;
            if n == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    sort_entrants(this.team_table().wrapping_add(((n * 3 - 0x30) * 2) as isize as usize), 0x10);
    ((n == this.team_count() as i32) as i32, n)
}

/// rule: the same turn in the Japanese League Cup (J-League Cup): tables freed, history archived, year +1, the 28
/// entrants (JapJCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `JapJCup::start_new_season` (0x665bc0, jap_j_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x665bc0 } (thiscall, ret 0, jap_j_cup.cpp; a virtual +0x08 da Copa da Liga japonesa): `JapJCup::start_new_season()`,
/// a virada (`start_new_cup_season`) com os inscritos de 0x665a00 (thiscall, ret 0) e as caixas do jap_j_cup.cpp nas linhas 0x1a3,
/// 0x1aa e 0x1b1.
pub(crate) unsafe extern "thiscall" fn jap_j_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x665a00, 0x9eaf14, [0x1a3, 0x1aa, 0x1b1], 1, "cqr_5 virada da copa da liga japonesa") // cqr5: the Japanese League Cup
}

