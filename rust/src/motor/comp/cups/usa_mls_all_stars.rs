use super::*;

/// The season rollover of a cup (`start_new_season`, slot 3, `+0x08`): frees and rebuilds entrants, rounds and ties.
///
/// This is the template for knockout cups, checked instruction by instruction: only the source file of the boxes, the
/// lines and the entrants routine change.
///
/// `+0x4c` = 0; the entrant list (`+0xa7`) and the round list (`+0xa3`) are freed (`0x9452ca`, cdecl) and zeroed; the
/// ties per round (`+0xad`: one vector per round, `+0x3c` rounds, signed word) are freed and zeroed one by one and the
/// vector itself freed and zeroed; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`, push 1;
/// mov ecx -> thiscall, ret 4); the year (`+0x40`, word) goes up by `ano` (1); `+0xab` = 0 and `+0xb1` = 0 (bytes).
///
/// Then, each with its own box if it returns negative: the entrants (`inscritos`, mov ecx, esi; direct call ->
/// thiscall, ret 0), the rounds (virtual `+0x8c`) and the tie tree (virtual `+0x94`).
///
/// Finally, with `previstas`, the predicted positions (virtual `+0x5c`), whose eax it returns (the All-Star Game has
/// none: it returns the eax of the tie tree).
///
/// [pt] A virada da temporada de uma copa (`start_new_season`, slot 3, +0x08; o molde das copas de mata-mata, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c) e a árvore dos confrontos
/// (virtual +0x94); por fim, com `previstas`, as posições previstas (virtual +0x5c), cujo eax ela devolve (o Jogo das
/// Estrelas não as tem: devolve o eax da árvore).
pub(crate) unsafe fn usa_mls_all_stars_cup_start_new_season(this: Cup, entrants: usize, file: usize, rows: [u32; 3], year: i16, predicted: bool, ramo: &'static str) -> u32 {
    this.set_season_closed(0);
    for o in [0xa7usize, 0xa3] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_usize(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    let e = read_usize(this.addr() + 8);
    if e != 0 {
        exe::comp_stats_archive_season_and_reset(e, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    crate::comparison::branch(ramo);
    this.set_current_round(0);
    this.set_run_by_stages(0);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(entrants);
    // simplification: in the boxes the original exits with the box’s eax (0x5e8290); the caller of the turnover does not read the return
    if entrants(this.addr()) < 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    let r = call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if !predicted {
        return r as u32;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Rule: the MLS All-Star Game season turn: tables freed, history archived, year +1, the two all-star teams, the rounds
/// and the tree (no predicted positions).
///
/// `UsaMlsAllStars::start_new_season()` (virtual +0x08 of the All-Star Game): the turn
/// (`usa_mls_all_stars_cup_start_new_season`) with the two teams from UsaMlsAllStars::set_up_teams (`set_up_teams`) and the
/// usa_mls_all_stars.cpp error boxes at lines 0x133, 0x13a and 0x141, without the predicted positions.
/// Original: `UsaMlsAllStars::start_new_season` (0x90a760, usa_mls_all_stars.cpp, thiscall, ret 0).
///
/// [pt] { 0x90a760 } (thiscall, ret 0, usa_mls_all_stars.cpp; a virtual +0x08 do Jogo das Estrelas): `UsaMlsAllStars::
/// start_new_season()`, a virada (`usa_mls_all_stars_cup_start_new_season`) com os dois times de UsaMlsAllStars::set_up_teams (0x90a540) e as caixas
/// do usa_mls_all_stars.cpp nas linhas 0x133, 0x13a e 0x141, sem as posições previstas.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x90a540, 0xaa20a0, [0x133, 0x13a, 0x141], 1, false, "cqr_6 virada do jogo das estrelas") // cqr6: the All-Star Game
}

/// Club record i of G_CLUB_TABLE (0x245 bytes each).
///
/// [pt] O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
pub(crate) unsafe fn usa_mls_all_stars_club_by_index(i: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(i.wrapping_mul(club::SIZE))
}

/// `UsaMlsAllStars::set_up_teams()` (`set_up_teams`, thiscall, ret 0, usa_mls_all_stars.cpp): the two teams of the All-Star
/// Game; returns 1.
///
/// +0x3e = 2; the list (`exe::crt_malloc` of 0xc, cdecl; out of memory, the error box for line 0xc6 and −1) with the two
/// All-Star clubs ([0x9d00a4] and [0x9d00a8], indices into the club table): in an odd year (+0x40, signed) the one in
/// [0x9d00a4] is first (home side) and the stadium conference is 0x413; in an even year the one in [0x9d00a8] is first
/// and the conference is 0x414.
///
/// A random draw from 0 to 5 (`exe::utils_random`(6), cdecl) picks the k-th MLS club ([0x9cf590] as the club's division, +0x57) of
/// that conference (usa_mls_get_club_division `exe::usa_mls_get_club_division`, push club; add esp -> cdecl, signed word), in table order: the
/// first All-Star team takes its stadium (+0x69) and +0x6d = 0 (if there is no k-th club, nothing changes).
///
/// Both get the nation [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, the USA [inference]) and the four list bytes (+4, +5,
/// +0xa, +0xb) are zeroed.
///
/// [pt] { 0x90a540 } (thiscall, ret 0, usa_mls_all_stars.cpp): `UsaMlsAllStars::set_up_teams()`, os dois times do Jogo das
/// Estrelas: +0x3e = 2; a lista (0x944e46 de 0xc, cdecl; sem memória, a caixa da linha 0xc6 e −1) com os dois clubes das
/// estrelas ([0x9d00a4] e [0x9d00a8], índices na tabela dos clubes): no ano ímpar (+0x40, com sinal) o de [0x9d00a4] é o
/// primeiro (o mandante) e a conferência dos estádios é 0x413; no ano par o de [0x9d00a8] é o primeiro e a conferência é
/// 0x414. Um sorteio de 0 a 5 (0x90cfc0(6), cdecl) escolhe o k-ésimo clube da MLS ([0x9cf590] na divisão do clube, +0x57)
/// dessa conferência (usa_mls_get_club_division 0x909ec0, push clube; add esp -> cdecl, palavra com sinal), pela ordem
/// da tabela: o primeiro time das estrelas fica com o estádio dele (+0x69) e +0x6d = 0 (sem o k-ésimo, nada muda). Os
/// dois ficam com a nação [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, os EUA [inferência]) e os quatro bytes da lista (+4,
/// +5, +0xa, +0xb) zerados. Devolve 1.
pub(crate) unsafe fn set_up_all_star_teams(this: Cup) -> i32 {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xaa20a0, 0xc6);
        return -1;
    }
    let (a, b) = (read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST), read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST));
    let conferencia = if this.year() % 2 != 0 {
        write_usize(l, usa_mls_all_stars_club_by_index(a));
        write_usize(this.team_table() + 6, usa_mls_all_stars_club_by_index(b));
        0x413
    } else {
        write_usize(l + 6, usa_mls_all_stars_club_by_index(a));
        write_usize(this.team_table(), usa_mls_all_stars_club_by_index(b));
        0x414
    };
    let mut k = exe::utils_random(6) as u8; // cqr6: the stadium drawn among 6
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = usa_mls_all_stars_club_by_index(i);
        let league = read_usize(c + club::DIVISION);
        if league != 0 && read_usize(league) == read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && ((exe::usa_mls_get_club_division(c) as u32) as i16) as i32 == conferencia {
            if k == 0 {
                crate::comparison::branch("cqr_6 all-star game stadium");
                let first = read_usize(this.team_table());
                write_usize(first + club::STADIUM, read_usize(c + club::STADIUM)); // cqr6: the stadium of the drawn club
                write_u8(read_usize(this.team_table()) + club::OWN_STADIUM, 0);
                break;
            }
            k = k.wrapping_sub(1);
        }
        i += 1;
    }
    let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_UNITED_STATES).wrapping_mul(nation::SIZE));
    let l = this.team_table();
    write_usize(read_usize(l + 6) + club::NATION, nation);
    write_usize(read_usize(l) + club::NATION, read_usize(read_usize(l + 6) + club::NATION));
    for o in [4usize, 5, 0xa, 0xb] {
        write_u8(this.team_table() + o, 0);
    }
    1
}

/// rule: the MLS All-Star Game pits the two conference all-star teams ([0x9d00a4] and [0x9d00a8], clubs in the table);
/// in odd years the [0x9d00a4] team is at home, in even years the [0x9d00a8] one; the stadium is that of an MLS club
/// drawn (1 in 6, in table order) among the home conference's clubs (0x413 in odd years, 0x414 in even ones); both
/// teams get the USA nation [inference]
/// Original: `UsaMlsAllStars::set_up_teams` (0x90a540, usa_mls_all_stars.cpp, thiscall, ret 0).
///
/// [pt] regra: o Jogo das Estrelas da MLS opõe as duas seleções das conferências ([0x9d00a4] e [0x9d00a8], clubes da
/// tabela); no ano ímpar a de [0x9d00a4] manda o jogo, no par a de [0x9d00a8]; o estádio é o de um clube da MLS
/// sorteado (1 em 6, pela ordem da tabela) entre os da conferência do mandante (0x413 no ano ímpar, 0x414 no par); os
/// dois times ficam com a nação dos EUA [inferência]
pub(crate) unsafe extern "thiscall" fn set_up_teams(this: usize) -> i32 {
    set_up_all_star_teams(Cup(this))
}

/// rule: the two MLS All-Star selections are called up on the day of Date(26, 6, year, 6): 26 July moved to weekday 6
/// [inference: the following Saturday]; the match is on 2 August (see cqm_12); then, every day, the cup update
/// Original: `UsaMlsAllStars::update_for_date` (0x90a900, usa_mls_all_stars.cpp, thiscall, ret 8).
///
/// [pt] { 0x90a900 } (thiscall, ret 8, usa_mls_all_stars.cpp; a virtual +0x14 do Jogo das Estrelas; quadro SEH pela Date
/// local, sem destrutor): `UsaMlsAllStars::update_for_date(hoje, fase)`: com a fase diferente de 0, no dia de Date(26, 6,
/// ano de hoje, dia da semana 6) (o sábado de 26 de julho em diante [inferência]), a convocação das duas seleções
/// (0x90a980, mov ecx -> thiscall, ret 0); depois a atualização da copa.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    // simplification: the year goes in a register with garbage in the high 16 bits; 0x549f70 reads it as a word
    let year = read_i16(today + 2) as u16 as u32;
    let d = make_date_dword(0x1a, 6, year, 6); // cqn10: the All-Star Game call-up
    if stage != 0 && is_date_today(d, today) {
        crate::comparison::branch("cqn_10 all-star game call-up");
        exe::usamlsallstars_select_squads(this);
    }
    eng_cc_cup_cup_update_for_date(Cup(this), today, stage)
}

/// Rule: the MLS All-Star game is a single match between the two conference selections, drawn on 15/6 and played on 2/8
/// (moved to weekday 6) at the borrowed home ground [inference: venue 0], with no bracket draw (+0xd = 0x20
/// [inference]); category 8; no prize.
///
/// `UsaMlsAllStars::create_round_table(stage, &rounds, &extra, &out)` (virtual +0x3c of the All-Star Game): outside
/// stage −1 (the byte), 0. On −1: out (if requested) = 0, rounds = 1, extra = 0; the table (`exe::crt_malloc` of 0x68; out of
/// memory, the error box at line 0x9a and 0) with the single round: the draw on day 15 of month 5 (June) moved to
/// weekday 5, the match on day 2 of month 7 (August) moved to weekday 6, leg 1, venue 0 (`cup_set_round_draw_date` and `cup_set_round_match_date`, cdecl,
/// with the year +0x40); name 0, +0xd = 0x20, category 8, 2 teams, 1 match, 2 new, 1 match per tie, no prize.
/// Original: `UsaMlsAllStars::create_round_table` (0x90a400, usa_mls_all_stars.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x90a400 } (thiscall, ret 0x10, usa_mls_all_stars.cpp; a virtual +0x3c do Jogo das Estrelas): `UsaMlsAllStars::
/// create_round_table(fase, &rodadas, &extra, &saida)`: fora da fase −1 (o byte), 0. Na −1: saida (se pedida) = 0,
/// rodadas = 1, extra = 0; a tabela (0x944e46 de 0x68; sem memória, a caixa da linha 0x9a e 0) com a rodada única: o
/// sorteio em 15 do mês 5 (junho) levado ao dia da semana 5, o jogo em 2 do mês 7 (agosto) levado ao dia 6, turno 1,
/// mando 0 (0x521e60 e 0x521eb0, cdecl, com o ano +0x40); nome 0, +0xd = 0x20, categoria 8, 2 times, 1 jogo, 2 novos,
/// 1 jogo por confronto, sem prêmio.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 1;
    *extra = 0;
    let t = CupRound(exe::crt_malloc(0x68));
    if t.is_null() {
        show_error_at(0xaa20a0, 0x9a);
        return 0;
    }
    crate::comparison::branch("cqm_12 all-star game fixture");
    let year = this.year() as i32 as u32;
    exe::cup_set_round_draw_date(t.addr(), 0, 0xf, 5, 0, 5, year);
    exe::cup_set_round_match_date(t.addr(), 0, 2, 7, 0, 6, 1, year, 0); // cqm12: the All-Star Game on 2 August
    let w = |o: usize, v: u16| write_u16(t.addr() + o, v);
    let d = |o: usize, v: u32| write_u32(t.addr() + o, v);
    w(0x18, 2);
    w(0x1c, 2);
    w(7, 0);
    w(9, 0);
    w(0xb, 0);
    w(0xd, 0x20);
    t.set_match_category(match_category::CONTINENTAL_LATE_ROUND);
    w(0x1a, 1);
    w(0xf, 0);
    w(0x1e, 0);
    t.set_single_leg_flag(0);
    t.set_legs(1);
    t.set_leg_gap_days(0);
    d(0x5c, 0);
    d(0x60, 0);
    d(0x64, 0);
    t.addr()
}

