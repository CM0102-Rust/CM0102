use super::*;

/// Writes one cup round (a 0x68-byte record, docs/.md §2.4).
///
/// The draw (day, month with 0 = January, years to add to the cup year +0x40, weekday or −1) goes through 0x521e60; the
/// match (the same, plus the leg at +6 and the venue at +0x13) through 0x521eb0.
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
pub(crate) struct FraCupCupRound {
    pub(crate) sorteio: (u32, u32, u32, u32),
    pub(crate) jogo: (u32, u32, u32, u32, u32, u32),
    pub(crate) codigo: u16,
    pub(crate) bits: (u16, u16, u16),
    pub(crate) categoria: u8,
    pub(crate) times: (u16, u16, u16, u16),
    pub(crate) confronto: (u8, u8, u8),
    pub(crate) premios: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
pub(crate) const fn cql11r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> FraCupCupRound {
    FraCupCupRound { sorteio, jogo, codigo, bits, categoria, times, confronto, premios }
}

/// Writes round i of cup round table t: draw and match dates (0x521e60/0x521eb0) and the round fields.
///
/// [pt] Grava a rodada `i` da tabela `t` (ano-base `ano`, a palavra +0x40 da copa com sinal): as duas datas (0x521e60 e
/// 0x521eb0, cdecl, 7 e 9 argumentos; add esp, 0x1c / 0x24) e os campos.
pub(crate) unsafe fn fra_cup_write_cup_round(t: usize, i: u32, r: &FraCupCupRound, year: u32) {
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    sorteio(t, i, r.sorteio.0, r.sorteio.1, r.sorteio.2, r.sorteio.3, year);
    jogo(t, i, r.jogo.0, r.jogo.1, r.jogo.2, r.jogo.3, r.jogo.4, year, r.jogo.5);
    let e = t + i as usize * 0x68;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((e + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((e + o) as *mut u32, v);
    w(7, r.codigo);
    w(9, i as u16);
    w(0xb, r.bits.0);
    w(0xd, r.bits.1);
    w(0xf, r.bits.2);
    *((e + 0x17) as *mut u8) = r.categoria;
    w(0x18, r.times.0);
    w(0x1a, r.times.1);
    w(0x1c, r.times.2);
    w(0x1e, r.times.3);
    *((e + 0x20) as *mut u8) = r.confronto.0;
    *((e + 0x21) as *mut u8) = r.confronto.1;
    *((e + 0x22) as *mut u8) = r.confronto.2;
    d(0x5c, r.premios.0);
    d(0x60, r.premios.1);
    d(0x64, r.premios.2);
}

/// Shared body of virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) of the
/// single-stage cups: only stage −1 builds the round table.
///
/// Outside stage −1 (the byte), returns 0 without writing anything. In −1: saida (if requested) = 0, rodadas = the
/// table size, extra = 0; the table (0x944e46 of 0x68 × rodadas; when out of memory, the error box `linha` of `arquivo`
/// and returns 0) is filled with each round.
///
/// Returns the table.
///
/// [pt] O corpo comum da virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) das copas
/// de uma fase só: fora da fase −1 (o byte), devolve 0 sem gravar nada. Na −1: saida (se pedida) = 0, rodadas = o
/// tamanho da tabela, extra = 0; a tabela (0x944e46 de 0x68 × rodadas; sem memória, a caixa `linha` de `arquivo` e
/// devolve 0) com cada rodada; devolve a tabela.
pub(crate) unsafe fn single_stage_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[FraCupCupRound], file: usize, row: u32, name: &'static str) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = table.len() as i16;
    *extra = 0;
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    crate::comparison::branch(name);
    let year = core::ptr::read_unaligned((this + cup::YEAR) as *const i16) as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        fra_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// A Copa da França (FraCup): nove rodadas de outubro a maio do ano seguinte, a partir da 6ª rodada nacional (código
/// 0x3c); o clube da divisão mais baixa joga em casa (+0xb = 1) da 7ª rodada à semifinal.
pub(crate) const FRENCH_CUP_ROUNDS: [FraCupCupRound; 9] = [
    cql11r((0xa, 9, 0, 0xffff_ffff), (6, 0xa, 0, 5, 2, 0), 0x3c, (0, 3, 0), 4, (0x28, 0x14, 0x28, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0xc, 0xa, 0, 4), (0x1b, 0xa, 0, 5, 2, 0), 0x46, (1, 3, 0), 4, (0xb8, 0x5c, 0xa4, 0x28), (0, 1, 0), (0, 0, 0)), // cql11: the 7th round of the Coupe de France
    cql11r((3, 0xb, 0, 4), (0x12, 0xb, 0, 5, 2, 0), 0x50, (1, 3, 0), 4, (0x5c, 0x2e, 0, 0xcc), (0, 1, 0), (0, 0, 0)),
    cql11r((6, 0, 1, 3), (0x16, 0, 1, 5, 2, 0), 0x5a, (1, 3, 0), 4, (0x40, 0x20, 0x12, 0xcc), (0, 1, 0), (0, 0, 0)),
    cql11r((0x1b, 0, 1, 3), (0xc, 1, 1, 5, 2, 0), 0x64, (1, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql11r((0x14, 1, 1, 6), (4, 2, 1, 5, 2, 0), 0x6e, (1, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0, 0x186a0)),
    cql11r((5, 2, 1, 6), (0x13, 2, 1, 6, 2, 0), 0x78, (1, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0, 0x30d40)),
    cql11r((0x14, 2, 1, 0), (0xc, 3, 1, 2, 2, 0), 0x82, (1, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0, 0x61a80)),
    cql11r((0xd, 3, 1, 3), (6, 4, 1, 5, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xf4240, 0xc3500)),
];

/// rule: the Coupe de France has 9 single-leg rounds, from the 6th national round (40 teams) to the final: 6th drawn
/// 10/10 (no fixed weekday), played 6/11; 7th 12/11 and 27/11 (164 clubs enter, 184 teams); 8th 3/12 and 18/12; last
/// 128 6/1 and 22/1 (18 enter); last 64 27/1 and 12/2; last 16 20/2 and 4/3; quarter-finals 5/3 and 19/3; semi-final
/// 20/3 and 12/4; final drawn 13/4, played 6/5 at a neutral ground [inference: venue 4]; from the 7th round to the
/// semi-final the lower-division club plays at home (+0xb = 1); the prize (+0x64) of 100 thousand in the last 16, 200
/// thousand in the quarter-finals, 400 thousand in the semi-final, and in the final 1 million (+0x60) and 800 thousand
/// (+0x64); category 4 up to the last 16 and 6 after
/// Original: `FraCup::create_round_table` (0x5ba3f0, fra_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5ba3f0 } (thiscall, ret 0x10, fra_cup.cpp; a virtual +0x3c da Copa da França): `FraCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 9 rodadas; a caixa da linha 0xa6 sem memória.
pub(crate) unsafe extern "thiscall" fn fra_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(this, stage, rounds, extra, out, &FRENCH_CUP_ROUNDS, 0x9ccb28, 0xa6, "cql_11 tabela da copa da franca")
}

/// A Copa da Liga francesa (FraLgeCup): sete rodadas de setembro a abril do ano seguinte.
pub(crate) const FRENCH_LEAGUE_CUP_ROUNDS: [FraCupCupRound; 7] = [
    cql11r((1, 8, 0, 3), (5, 9, 0, 1, 2, 0), 0xe6, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((0xf, 9, 0, 4), (0x10, 0xa, 0, 1, 2, 0), 0xa, (0, 3, 0), 4, (0x1c, 0xe, 0x14, 0x10), (0, 1, 0), (0, 0x493e0, 0)), // cql11: the 1st round of the French League Cup
    cql11r((0x1a, 0xa, 0, 4), (8, 0, 1, 5, 2, 0), 0x14, (0, 3, 0), 4, (0x20, 0x10, 0x12, 0x24), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((0xe, 0, 1, 4), (0x1d, 0, 1, 5, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0, 0x24), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((3, 1, 1, 4), (0x13, 1, 1, 5, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((4, 2, 1, 4), (1, 3, 1, 5, 2, 0), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0x493e0, 0)),
    cql11r((2, 3, 1, 6), (0x16, 3, 1, 5, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x16e360, 0)),
];

/// The Copa del Rey (SpaCup): seven rounds from August to May of the following year.
///
/// The club from the lower division plays at home and clubs from the same nation are kept apart (`+0xb` = 3) up to the
/// semi-final; from the 3rd round to the semi-final, ties are two-legged.
///
/// [pt] A Copa do Rei (SpaCup): sete rodadas de agosto a maio do ano seguinte; o clube da divisão mais baixa joga em casa e
/// os clubes da mesma nação se separam (+0xb = 3) até a semifinal; da 3ª rodada à semifinal, ida e volta.
pub(crate) const COPA_DEL_REY_ROUNDS: [FraCupCupRound; 7] = [
    cql11r((0x12, 7, 0, 4), (0x1e, 7, 0, 2, 2, 0), 0xe6, (3, 3, 0), 4, (0x26, 0x13, 0x26, 0), (0, 1, 7), (0, 0xffff_ffff, 0)),
    cql11r((8, 8, 0, 4), (0xd, 0xb, 0, 2, 2, 0), 0xa, (3, 3, 0), 4, (0x40, 0x20, 0x2d, 0x26), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql11r((0xf, 0xb, 0, 4), (3, 0, 1, 2, 2, 0), 0x14, (3, 3, 0), 4, (0x20, 0x10, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // cql11: the 2nd round of the Copa del Rey
    cql11r((5, 0, 1, 4), (0xa, 0, 1, 2, 2, 0), 0x1e, (3, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql11r((0x13, 0, 1, 4), (0x1f, 0, 1, 2, 2, 0), 0x78, (3, 0, 3), 6, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql11r((0x17, 1, 1, 4), (0xb, 3, 1, 2, 2, 0), 0x82, (3, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql11r((0x1a, 3, 1, 3), (0x1a, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Id of the competition pointed to by c + o, or None.
///
/// [pt] O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
pub(crate) unsafe fn comp_id_at(c: usize, o: usize) -> Option<usize> {
    let d = read_u32_as_usize(c + o);
    if d == 0 { None } else { Some(read_u32_as_usize(d)) }
}

/// Writes entrant n (club, class 0, status 0) in the 6-byte list at +0xa7 and counts it.
///
/// [pt] Grava a inscrição `n` da lista (+0xa7, 6 bytes: o clube, +4 classe 0, +5 situação 0) e conta.
pub(crate) unsafe fn add_entrant(this: usize, n: &mut i32, club: usize) {
    let e = read_u32_as_usize(this + cup::TEAM_TABLE) + *n as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, club as u32);
    *((e + 4) as *mut u8) = 0;
    *((e + 5) as *mut u8) = 0;
    *n += 1;
}

/// One pass over the first clubs adding those accepted by `entra`; optionally stops when the list fills (+0x3e).
///
/// [pt] Uma passada pelos clubes (os `ate` primeiros, índice em 16 bits) que passam em `entra(clube, n)`; com `para`, a
/// passada acaba quando a lista enche (+0x3e).
pub(crate) unsafe fn enter_clubs_pass(this: usize, n: &mut i32, ate: impl Fn() -> i32, para: bool, enters: impl Fn(usize, i32) -> bool) {
    let mut i: i16 = 0;
    while (i as i32) < ate() {
        let c = eng_cc_cup_club_by_index(i);
        if enters(c, *n) {
            add_entrant(this, n, c);
            if para && *n == read_i16(this + cup::TEAM_COUNT) as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// Adds clubs of division [id] while they fit (n < +0x3e).
///
/// [pt] Os clubes da divisão [id] enquanto cabem (n < +0x3e), até encher.
pub(crate) unsafe fn fra_cup_enter_division_clubs(this: usize, n: &mut i32, id: usize) {
    enter_clubs_pass(this, n, || read_u32_as_usize(G_CLUB_COUNT) as i32, true, |c, n| comp_id_at(c, 0x57) == Some(read_u32_as_usize(id)) && n < read_i16(this + cup::TEAM_COUNT) as i32);
}

/// Amateur entrant limit of the French Cup: 0xb8 in 2001, 0xb6 otherwise.
///
/// [pt] O limite dos amadores da Copa da França: 0xb8 em 2001 (+0x40 = 0x7d1), 0xb6 nos outros anos.
pub(crate) unsafe fn french_cup_amateur_limit(this: usize) -> i32 {
    if read_i16(this + cup::YEAR) == 0x7d1 { 0xb8 } else { 0xb6 }
}

/// `FraCup::collect_entrants()` (0x5baa50, thiscall, ret 0, fra_cup.cpp): builds the 222-entrant French Cup list;
/// returns (full flag, count).
///
/// +0x3e = 222; the list is allocated (0x944e46 of 0x534 bytes at +0xa7; out of memory, the error box for line 0x16f
/// and −1).
///
/// In this order: all clubs of division [0x9cf5d4] (without checking the space left); the clubs of division [0x9cf764]
/// linked (+0x60) to [0x9cf5d8]; the French clubs (+0x53 = [0x9cf300]) of [0x9cf764] with no link; the French clubs
/// with no division that are not national teams (0x53b900), among the clubs before the 2 × [0xae2350] national teams at
/// the end — these last three groups up to `french_cup_amateur_limit`; then the Second ([0x9cf5d0]) and the First
/// ([0x9cf5cc]) while they fit.
///
/// Returns 1 if the list is full, otherwise 0 (and how many entered, for the comparer).
///
/// [pt] { 0x5baa50 } (thiscall, ret 0, fra_cup.cpp): `FraCup::collect_entrants()`: +0x3e = 222; a lista (0x944e46 de 0x534 em
/// +0xa7; sem memória, a caixa da linha 0x16f e −1). Na ordem: todos os da divisão [0x9cf5d4] (sem conferir o espaço); os
/// da divisão [0x9cf764] ligados (+0x60) à [0x9cf5d8]; os franceses (+0x53 = [0x9cf300]) da [0x9cf764] sem ligação; os
/// franceses sem divisão que não são seleções (0x53b900), entre os clubes antes das 2 × [0xae2350] seleções do fim — estas
/// três até `french_cup_amateur_limit`; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem. Devolve 1 com a lista
/// cheia, senão 0 (e quantos entraram, para o comparador).
pub(crate) unsafe fn french_cup_collect_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 0xde);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alloc(0x534);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9ccb28, 0x16f);
        return (-1, 0);
    }
    let mut n = 0i32;
    let all = || read_u32_as_usize(G_CLUB_COUNT) as i32;
    enter_clubs_pass(this, &mut n, all, false, |c, _| comp_id_at(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_NATIONAL)));
    enter_clubs_pass(this, &mut n, all, true, |c, n| {
        comp_id_at(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) && comp_id_at(c, 0x60) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA)) && n < french_cup_amateur_limit(this)
    });
    enter_clubs_pass(this, &mut n, all, true, |c, n| {
        comp_id_at(c, 0x53) == Some(read_u32_as_usize(KEY_NATION_FRANCE)) && comp_id_at(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) && read_u32_as_usize(c + 0x60) == 0
            && n < french_cup_amateur_limit(this)
    });
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    enter_clubs_pass(this, &mut n, || read_u32_as_usize(G_CLUB_COUNT) as i32 - 2 * read_u32_as_usize(G_NATION_COUNT) as i32, true, |c, n| {
        read_u32_as_usize(c + 0x53) != 0 && national_team(c) == 0 && read_u32_as_usize(c + 0x57) == 0 && comp_id_at(c, 0x53) == Some(read_u32_as_usize(KEY_NATION_FRANCE))
            && n < french_cup_amateur_limit(this)
    });
    crate::comparison::branch("cqm_11 inscritos da copa da franca");
    fra_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION); // cqm11: the French Second Division in the Coupe de France
    fra_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION); // cqm11: the French First Division in the Coupe de France
    ((n == read_i16(this + cup::TEAM_COUNT) as i32) as i32, n)
}

/// rule: the Coupe de France takes 222 clubs, in this order (the order decides the entry round: the first 40 in the 6th
/// round, the next 164 in the 7th and the last 18, the First Division, in the last 128): all clubs of division
/// [0x9cf5d4] (the National [inference]); those of [0x9cf764] linked (+0x60) to [0x9cf5d8] (the CFA [inference]); the
/// French ones of [0x9cf764] without a link; the French clubs with no division that are not national teams — these
/// three groups up to 184 entrants in 2001 and 182 in other years; the Second and the First Division; with an
/// incomplete list it returns 0
/// Original: `FraCup::collect_entrants` (0x5baa50, fra_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da França 222 clubes, nesta ordem (a ordem decide a rodada de entrada: os 40 primeiros na 6ª
/// rodada, os 164 seguintes na 7ª e os 18 últimos, a Primeira, nos 64 avos): todos os da divisão [0x9cf5d4] (o National
/// [inferência]); os da [0x9cf764] ligados (+0x60) à [0x9cf5d8] (o CFA [inferência]); os franceses da [0x9cf764] sem
/// ligação; os franceses sem divisão que não são seleções — estes três grupos até 184 inscritos em 2001 e 182 nos
/// outros anos; a Segunda e a Primeira; com a lista incompleta, devolve 0
pub(crate) unsafe extern "thiscall" fn fra_cup_collect_entrants(this: usize) -> i32 {
    french_cup_collect_entrants(this).0
}

/// `FraLgeCup::collect_entrants()` (`0x5bd160`, thiscall, ret 0, `fra_lge_cup.cpp`): builds the list of 54 French
/// League Cup entrants.
///
/// Sets `+0x3e` = 54; allocates the list (`0x944e46` of 0x144 into `+0xa7`; out of memory, the box of line `0x146` and
/// −1). In order: the clubs of division [`0x9cf5d4`] that did not come (`+0x5b`, with competition) from [`0x9cf5d8`],
/// up to 16 in 2001 (`+0x40` = 0x7d1) and 14 in other years; the clubs of [`0x9cf5d4`] that came from [`0x9cf5d8`]
/// while the list has fewer than 16; then the Second division ([`0x9cf5d0`]) and the First ([`0x9cf5cc`]) while they
/// fit.
///
/// Returns 1 with the list full, otherwise 0.
///
/// [pt] { 0x5bd160 } (thiscall, ret 0, fra_lge_cup.cpp): `FraLgeCup::collect_entrants()`: +0x3e = 54; a lista (0x944e46 de
/// 0x144 em +0xa7; sem memória, a caixa da linha 0x146 e −1). Na ordem: os da divisão [0x9cf5d4] que não vieram (+0x5b,
/// com competição) da [0x9cf5d8], até 16 em 2001 (+0x40 = 0x7d1) e 14 nos outros anos; os da [0x9cf5d4] que vieram da
/// [0x9cf5d8] enquanto a lista tem menos de 16; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem.
/// Devolve 1 com a lista cheia, senão 0.
pub(crate) unsafe fn french_league_cup_collect_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 0x36);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alloc(0x144);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9ccb98, 0x146);
        return (-1, 0);
    }
    let mut n = 0i32;
    let all = || read_u32_as_usize(G_CLUB_COUNT) as i32;
    let limit = if read_i16(this + cup::YEAR) == 0x7d1 { 0x10 } else { 0xe };
    enter_clubs_pass(this, &mut n, all, true, |c, n| {
        comp_id_at(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_NATIONAL)) && read_u32_as_usize(c + 0x5b) != 0 && comp_id_at(c, 0x5b) != Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA)) && n < limit
    });
    // the original tests the size before each club: the pass stops when the list reaches 16
    let mut i: i16 = 0;
    while (i as i32) < read_u32_as_usize(G_CLUB_COUNT) as i32 && n * 6 < 0x60 {
        let c = eng_cc_cup_club_by_index(i);
        if comp_id_at(c, 0x57) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_NATIONAL)) && comp_id_at(c, 0x5b) == Some(read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA)) {
            add_entrant(this, &mut n, c);
            if n == read_i16(this + cup::TEAM_COUNT) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparison::branch("cqm_11 inscritos da copa da liga francesa");
    fra_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION); // cqm11: the French Second Division in the League Cup
    fra_cup_enter_division_clubs(this, &mut n, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION); // cqm11: the French First Division in the League Cup
    ((n == read_i16(this + cup::TEAM_COUNT) as i32) as i32, n)
}

/// `FraSuper::select_teams()` (0x5bf770, thiscall, ret 0, fra_super.cpp): picks the First Division champion and the
/// French Cup winner, or the First Division runner-up when they are the same club.
///
/// Sets +0x3e = 2 and allocates the list (0x944e46 of 0xc at +0xa7; when out of memory, the error box for line 0xc4 and
/// −1). The First Division champion ([0x9cf5cc]; if missing, error box 0xcd and 0) and the French Cup winner
/// ([0x9cf70c]; if missing, 0xd6 and 0); if they are the same club, the second one becomes the First Division runner-up
/// (if missing, 0xe2 and 0).
///
/// Class and status 0 for both; returns 1.
///
/// [pt] { 0x5bf770 } (thiscall, ret 0, fra_super.cpp): `FraSuper::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc4 e −1). O campeão da Primeira ([0x9cf5cc]; sem ele, a caixa 0xcd e 0) e o
/// da Copa da França ([0x9cf70c]; sem ele, 0xd6 e 0); se é o mesmo clube, o segundo vira o vice da Primeira (sem ele, 0xe2
/// e 0). Classe e situação 0 nos dois; devolve 1.
pub(crate) unsafe fn french_super_cup_select_teams(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 2);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alloc(0xc);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9ccc48, 0xc4);
        return (-1, 0);
    }
    let a = latest_placed_club(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, 0); // cqm11: o campeão francês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        show_error_at(0x9ccc48, 0xcd);
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_FRENCH_CUP, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        show_error_at(0x9ccc48, 0xd6);
        return (0, 0);
    }
    if a == b {
        crate::comparison::branch("cqm_11 dobradinha francesa");
        let b = latest_placed_club(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, 1);
        core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
        if b == 0 {
            show_error_at(0x9ccc48, 0xe2);
            return (0, 0);
        }
    }
    crate::comparison::branch("cqm_11 clubes da supercopa francesa");
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

