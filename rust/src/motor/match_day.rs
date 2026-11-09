use super::*;

/// Does the club play in today's fixture list ([this+0x19], count at +0x21, 0x69 bytes per match, home at +0x20 and
/// away at +0x24)? With `so_marcados`, only fixtures with +0x5b set count. Returns 1 or 0 (0 for a null club).
///
/// [pt] { 0x6b59a0 } (match_day.cpp): o clube tem jogo na lista do dia ([this + 0x19], quantidade em
/// 0x21, 0x69 bytes por jogo; mandante em 0x20, visitante em 0x24)? Com `so_marcados`, so os jogos
/// com 0x5b preenchido. Clube nulo: o original mostra uma caixa de erro e devolve 0 (aqui, so 0).
pub(crate) unsafe extern "thiscall" fn has_match(this: usize, club: u32, so_marked: u32) -> u32 {
    if club == 0 {
        return 0;
    }
    let n = *((this + match_day_fields::MATCH_COUNT) as *const i32);
    let base = *((this + match_day_fields::MATCHES) as *const usize) + 0x20;
    for k in 0..n.max(0) as usize {
        let game = base + 0x69 * k;
        if so_marked != 0 && *((game + 0x3b) as *const u32) == 0 {
            continue;
        }
        if *(game as *const u32) == club || *((game + 4) as *const u32) == club {
            return 1;
        }
    }
    0
}

/// Did the club win the match? Decides on penalties (+0x49/+0x4a), else extra time (+0x47/+0x48), else normal time
/// (+0x43/+0x44), using the first score that is not -1. Returns 1 or 0; a null match or club, or a club not in the
/// match, gives 0.
///
/// [pt] { 0x6b7cf0 } (match_day.cpp, cdecl): o clube venceu o jogo? Decide pelos penaltis (0x49/0x4a),
/// senao pela prorrogacao (0x47/0x48), senao pelo tempo normal (0x43/0x44); placar -1 nao conta.
/// Jogo ou clube nulo, ou clube fora do jogo: o original mostra uma caixa de erro e devolve 0.
pub(crate) unsafe extern "C" fn won_the_match(game: usize, club: u32) -> u8 {
    if game == 0 || club == 0 {
        return 0;
    }
    let home = *((game + fixture::HOME_CLUB) as *const u32) == club;
    if !home && *((game + fixture::AWAY_CLUB) as *const u32) != club {
        return 0;
    }
    for (a, b) in [(0x49, 0x4a), (0x47, 0x48), (0x43, 0x44)] {
        let (pa, pb) = (read_i8(game + a), read_i8(game + b));
        if pa > -1 {
            return (if home { pa > pb } else { pa < pb }) as u8;
        }
    }
    0
}

/// Shows the "v%s %s.%s %d" error box for match_man.cpp at a line and exits the game, as the original.
///
/// [pt] A caixa de erro "v%s %s.%s %d" de match_man.cpp na linha `linha`, e o jogo para (como a original).
pub(crate) unsafe fn match_man_fatal_error(row: u32) -> ! {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0xa15da4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = join(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// The end of the match day: frees the global list, the day's arrays and, for each match, the engine, the summary and
/// the `+0x5f` object.
///
/// Frees the global list `[0xb67dc8]`, the two day arrays (`+1`, `+0xd`) and, for each match (`+0x19`, `0x69` bytes
/// each, `+0x21` of them), the engine (`0x6b97a0` and freed; the match leaves `[0xde2f70]` via `0x90d6b0`), the summary
/// (`+0x5b`) and the object at `+0x5f` (`0x453bc0`); finally the match array.
/// Original: `end_day` (0x6b4190, match_day.cpp, thiscall).
///
/// [pt] { 0x6b4190 } (match_day.cpp): o fim do dia de jogos. Libera a lista global [0xb67dc8], os dois
/// vetores do dia (+1, +0xd) e, para cada partida (+0x19, 0x69 bytes cada, +0x21 delas), o motor
/// (0x6b97a0 e liberado; a partida sai de [0xde2f70] por 0x90d6b0), o resumo (+0x5b) e o objeto de
/// +0x5f (0x453bc0); por fim o vetor das partidas.
pub(crate) unsafe extern "thiscall" fn end_day(day: usize) {
    let d = |o: usize| (day + o) as *mut usize;
    let list = *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *const usize);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *mut usize) = 0;
    }
    for o in [1usize, 0xd] {
        if *d(o) != 0 {
            exe::crt_free(*d(o));
            *d(o) = 0;
            *d(o + 4) = 0;
            *d(o + 8) = 0;
        }
    }
    if *d(match_day_fields::MATCHES) == 0 {
        return;
    }
    let mut k = 0i32;
    while k < *d(match_day_fields::MATCH_COUNT) as i32 {
        let p = *d(match_day_fields::MATCHES) + k as usize * 0x69;
        let q = |o: usize| (p + o) as *mut usize;
        let engine = *q(day_match::ENGINE);
        if engine != 0 {
            exe::drop_cm3_match_engine(engine);
            exe::crt_operator_delete(engine);
            *q(day_match::ENGINE) = 0;
            exe::virtual_staff_release_fixture(*(G_VIRTUAL_STAFF as *const usize), p + day_match::FIXTURE_COPY);
        }
        if *q(day_match::SCOREBOARD) != 0 {
            exe::crt_operator_delete(*q(day_match::SCOREBOARD));
            *q(day_match::SCOREBOARD) = 0;
        }
        let x = *q(day_match::SUMMARY);
        if x != 0 {
            exe::match_team_pair_destructor(x);
            exe::crt_operator_delete(x);
            *q(day_match::SUMMARY) = 0;
        }
        k += 1;
    }
    exe::crt_free(*d(match_day_fields::MATCHES));
    *d(match_day_fields::MATCHES) = 0;
    *d(match_day_fields::MATCH_CAPACITY) = 0;
    *d(match_day_fields::MATCH_COUNT) = 0;
}

/// Clears each of the day's matches' fields and picks the human's match (or the first marked one) for the screen.
/// Returns 1 if there was one of the human's.
///
/// With matches on the day (`+0x2b`), it clears the `0x40` bytes of each one (`+0xd`, `0x54` bytes, `+0x15` of them)
/// from `+0x14` and picks the human's (`+0x10`; otherwise the first with `+0xc`) for the screen (`0x717cc0` and
/// `0x7fc7e0` with `[0xb67a48]` on).
/// Original: `pick_day_match` (0x6b42a0, match_day.cpp, thiscall).
///
/// [pt] { 0x6b42a0 } (match_day.cpp): com partidas no dia (+0x2b), limpa os 0x40 bytes de cada uma
/// (+0xd, 0x54 bytes, +0x15 delas) a partir de +0x14 e escolhe a do humano (+0x10; senao a primeira
/// com +0xc) para a tela (0x717cc0 e 0x7fc7e0 com [0xb67a48] ligado). Devolve 1 se havia uma do
/// humano.
pub(crate) unsafe extern "thiscall" fn pick_day_match(day: usize) -> u32 {
    if *((day + match_day_fields::HAS_INTEREST) as *const usize) == 0 {
        return 0;
    }
    let mut found = 0u32;
    let mut escolhida = 0usize;
    let n = *((day + match_day_fields::COMPETITION_COUNT) as *const i32);
    let mut k = 0i32;
    while k < n {
        let e = *((day + match_day_fields::COMPETITIONS) as *const usize) + k as usize * 0x54;
        core::ptr::write_bytes((e + day_competition::PANEL) as *mut u8, 0, 0x40);
        let e = *((day + match_day_fields::COMPETITIONS) as *const usize) + k as usize * 0x54;
        if *((e + day_competition::HAS_MARKED) as *const usize) != 0 {
            if escolhida == 0 || *((escolhida + day_competition::HAS_MARKED) as *const usize) == 0 {
                escolhida = e;
            }
            found = 1;
        } else if *((e + day_competition::INTERESTING) as *const usize) != 0 && escolhida == 0 {
            escolhida = e;
        }
        k += 1;
    }
    *(G_MATCH_DAY_SCREEN_SHOWING_FLAG as *mut u32) = 1;
    exe::open_day_fixtures_screen(0, escolhida);
    exe::screenmanager_monitor(G_SCREEN_MANAGER);
    *(G_MATCH_DAY_SCREEN_SHOWING_FLAG as *mut u32) = 0;
    found
}

// simplification: each side’s line-up calls the automatic one (0x6dbc50, which allocates the list and reads the stack) and the
// match day releases engines and lists: no comparison of its own, validated by the career (Rust x
// original), like the creation and the end of the engine. The calling routines are still compared.

/// Starts the day's next match after the current one (`+0x2f`): skips the decided ones, those of a human club and those
/// already started; creates the engine and prepares it; in the 1st pass with none, moves to the 2nd. Returns 1 if it
/// started one.
///
/// Starts the next match of the day (`+0x19`, `0x69` bytes each, `+0x21` of them) after the current one (`+0x2f`; -2 =
/// finished). The current one still with an engine (`+0x57`): error `0x68d`. It skips those already decided (`+0x47`
/// -2), those with no match set in the 2nd pass (`+0x33` 1 and `+0x63.+0x10` zero), those of a human club (`0x600a50`
/// with `+0x44`/`+0x45`) and those already started (`+0x47` != -1, `+0x57`, `+0x53`).
///
/// On the chosen one: takes it off queue `[0xde2f70]` (`0x90d2f0` with -3), creates the engine (`0xf63b` bytes,
/// `0x6b7e50`; without it error `0x6b2` and it carries on) and prepares it (`0x6b7f20` with the fast mode from
/// `0x6b66a0`); `+0x2f` = it, returns 1. With none: in the 1st pass (`+0x33` 1) it moves to 2 and starts again from the
/// beginning; otherwise `+0x2f` = -2 and returns 0.
/// Original: `start_next_match` (0x6b5a70, match_day.cpp, thiscall).
///
/// [pt] { 0x6b5a70 } (thiscall, match_day.cpp): comeca a proxima partida do dia (+0x19, 0x69 bytes cada,
/// +0x21 delas) depois da atual (+0x2f; -2 = acabou). A atual ainda com motor (+0x57) e erro 0x68d.
/// Pula as ja decididas (+0x47 -2), as sem jogo marcado na 2a volta (+0x33 1 e +0x63.+0x10 zero), as
/// de clube humano (0x600a50 com +0x44/+0x45) e as ja comecadas (+0x47 != -1, +0x57, +0x53). Na
/// escolhida: tira da fila [0xde2f70] (0x90d2f0 com -3), cria o motor (0xf63b bytes, 0x6b7e50; sem
/// ele erro 0x6b2 e segue) e o prepara (0x6b7f20 com a rapida de 0x6b66a0); +0x2f = ela, devolve 1.
/// Sem nenhuma: na 1a volta (+0x33 1) passa a 2 e recomeca do inicio; senao +0x2f = -2 e devolve 0.
pub(crate) unsafe extern "thiscall" fn start_next_match(day: usize) -> u32 {
    let read = |o: usize| *((day + o) as *const i32);
    let por = |o: usize, v: i32| *((day + o) as *mut i32) = v;
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let current = read(match_day_fields::CURRENT_MATCH);
    if current == -2 {
        return 0;
    }
    if current > -1 && u(u(day, match_day_fields::MATCHES).wrapping_add((current as isize * 0x69) as usize), 0x57) != 0 {
        show_error_at(0xa15708, 0x68d);
        return 0;
    }
    loop {
        let mut k = read(match_day_fields::CURRENT_MATCH) + 1;
        while k < read(match_day_fields::MATCH_COUNT) {
            let p = u(day, match_day_fields::MATCHES).wrapping_add((k as isize * 0x69) as usize);
            if read_i8(p + day_match::HOME_GOALS) != -2
                && (read_i8(day + match_day_fields::PASS) != 1 || u(u(p, day_match::DAY_COMPETITION), 0x10) != 0)
                && exe::human_manager_is_human_club(u(p, day_match::HOME_CLUB), 0, *((p + day_match::HOME_SIDE_MARK) as *const u8) as u32, 0, 0) == 0
                && exe::human_manager_is_human_club(u(p, day_match::AWAY_CLUB), 0, *((p + day_match::AWAY_SIDE_MARK) as *const u8) as u32, 0, 0) == 0
                && read_i8(p + day_match::HOME_GOALS) == -1
                && u(p, day_match::ENGINE) == 0
                && u(p, day_match::MARKED) == 0
            {
                exe::virtual_staff_fill_fixture_squads(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -3, 0);
                let mem = exe::crt_operator_new(0xf63b);
                let engine = if mem != 0 { exe::new_cm3_match_engine(mem) } else { 0 };
                *((p + day_match::ENGINE) as *mut usize) = engine;
                if engine != 0 {
                    let r = exe::match_day_can_quick_simulate(p + day_match::FIXTURE_COPY);
                    exe::cm3_match_engine_init_match(engine, p + day_match::FIXTURE_COPY, r, 0, 0);
                    por(0x2f, k);
                    crate::comparison::branch("day match started");
                    return 1;
                }
                show_error_at(0xa15708, 0x6b2);
            }
            k += 1;
        }
        if read_i8(day + match_day_fields::PASS) != 1 {
            por(0x2f, -2);
            crate::comparison::branch("day no more matches");
            return 0;
        }
        *((day + match_day_fields::PASS) as *mut u8) = 2;
        crate::comparison::branch("day second leg");
        por(0x2f, -1);
    }
}

/// Moves the day's current match (`+0x2f`) forward: one engine step (1 in 3 or 6 calls, by the modes in `[0xae3138]`)
/// or to the end; when it finishes, creates the summary, records it, frees the engine and takes it off the queue.
/// Returns 1 if the match finished.
///
/// Without `ate_o_fim` (to the end), only 1 in every 3 (`0x40000` in `[0xae3138]+4`) or 6 (`0x80000`) calls, by counter
/// `[0xb67a54]` (0..100), and one engine step (`0x6b98c0` with 1); with `ate_o_fim`, steps (-1) to the end. The state
/// goes to `+0x67`; on finishing (2 or 8) it counts in `+0x25`, creates the summary (`+0x5f`, `0xd7a` bytes with 2
/// objects of `0x625` at `+0x11f`; without it error `0x713`), records it (`0x7293f0`), frees the engine and takes the
/// match off queue `[0xde2f70]` (`0x90d6b0`).
/// Original: `step_day_match` (0x6b5ce0, match_day.cpp, thiscall, ret 4).
///
/// [pt] { 0x6b5ce0 } (thiscall, ret 4, match_day.cpp): faz andar a partida atual do dia (+0x2f). Sem
/// `ate_o_fim`, so 1 de cada 3 (0x40000 em [0xae3138]+4) ou 6 (0x80000) chamadas, pelo contador
/// [0xb67a54] (0..100), e um passo do motor (0x6b98c0 com 1); com `ate_o_fim`, passos (-1) ate o
/// fim. O estado vai em +0x67; ao terminar (2 ou 8) conta em +0x25, cria o resumo (+0x5f, 0xd7a
/// bytes com 2 objetos de 0x625 em +0x11f; sem ele erro 0x713), registra (0x7293f0), libera o motor
/// e tira a partida da fila [0xde2f70] (0x90d6b0). Devolve 1 se a partida terminou.
pub(crate) unsafe extern "thiscall" fn step_day_match(day: usize, until_end: u32) -> u32 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let counter = G_STEP_BACKGROUND_MATCH_COUNTER as *mut i32;
    *counter += 1;
    if *counter > 100 {
        *counter = 0;
    }
    if until_end == 0 {
        let mode = u(u(G_CONFIG, 0), 4);
        if mode & 0x40000 != 0 && *counter % 3 > 0 {
            return 0;
        }
        if mode & 0x80000 != 0 && *counter % 6 > 0 {
            return 0;
        }
    }
    let current = *((day + match_day_fields::CURRENT_MATCH) as *const i32);
    if current < 0 {
        return 0;
    }
    let p = u(day, match_day_fields::MATCHES).wrapping_add((current as isize * 0x69) as usize);
    if u(p, day_match::ENGINE) == 0 {
        return 0;
    }
    let state = (p + day_match::STATE) as *mut u8;
    crate::comparison::branch("walk with engine");
    let end = |e: u8| e == 2 || e == 8;
    let step_towards = |n: i32| {
        let e = (exe::cm3_match_engine_run_match_steps(u(p, day_match::ENGINE), n) as u32) as u8;
        *state = e;
        if end(e) {
            *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;
        }
    };
    if until_end != 0 {
        crate::comparison::branch("walk to the end");
        while !end(*state) {
            step_towards(-1);
        }
    } else {
        step_towards(1);
        crate::comparison::branch("walk one step_towards");
    }
    if !end(*state) {
        return 0;
    }
    crate::comparison::branch("walk match finished");
    let mem = exe::crt_operator_new(0xd7a);
    if mem != 0 {
        exe::crt_vector_constructor_iterator(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
    }
    *((p + day_match::SUMMARY) as *mut usize) = mem;
    if mem == 0 {
        show_error_at(0xa15708, 0x713);
        return 0;
    }
    exe::match_stats_record_match(u(G_MATCH_STATS, 0), u(p, day_match::ENGINE), mem);
    let engine = u(p, day_match::ENGINE);
    if engine != 0 {
        exe::drop_cm3_match_engine(engine);
        exe::crt_operator_delete(engine);
    }
    *((p + day_match::ENGINE) as *mut usize) = 0;
    exe::virtual_staff_release_fixture(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY);
    1
}

/// Closes one of the day's matches: copies the day's match into the fixture, records the result in the competition
/// (method 1, or the away-game path) and runs the post-match: players, events, contracts, suspensions, states, the
/// history and each side's club.
///
/// The closing of day match `p` (`0x69` bytes): copies the `0x4f` bytes from p+4 to the fixture (`+0`) and, with a
/// competition (`+0x14`), reads the round number of a human club (`[adadfc]` of the competition `+8`, `0x4a4850`,
/// `+0x2d` above 1; as in the original, it looks only at the away club `+0x20` on both passes) and records the result:
/// p+`0x47` = -3 (away game, `0x924050` with bits 12-13 of `+0x4d` and the copy with scores -1 in
/// `0x5ab510`/`0x5aa680`) or method 1 of the competition.
///
/// Then the post-match with the summary (p+`0x5f`): players (`0x6b6cc0`), events (`0x6b74a0`), contracts (`0x4ca6d0`),
/// suspensions (`0x55ca00`), `0x76b600` (except for -3), states (`0x62c080`) and, except for -3, the history
/// (`[0xb74434]` method 1, outside `[0x9cf684]`), `0x7c0d70`, `0x8c39f0`, `0x444930`, `0x868830`, `0x4134c0`,
/// `0x6951e0`, `0x59a5d0`, `0x7f8c00` and the club of each side (`0x6b79a0` with the round). Without `p`, error `0x73f`
/// and 0.
/// Original: `close_day_match` (0x6b5f00, match_day.cpp, thiscall, ret 4).
///
/// [pt] { 0x6b5f00 } (thiscall, ret 4, match_day.cpp): o fechamento de uma partida do dia `p` (0x69
/// bytes): copia os 0x4f bytes de p+4 para a partida (+0) e, com competicao (+0x14), le o numero da
/// rodada de um clube humano ([adadfc] da competicao +8, 0x4a4850, +0x2d acima de 1; como na
/// original, olha so o visitante +0x20 nas duas voltas) e registra o resultado: p+0x47 = -3 (jogo de
/// fora, 0x924050 com os bits 12-13 de +0x4d e a copia com placares -1 em 0x5ab510/0x5aa680) ou o
/// metodo 1 da competicao. Depois o pos-jogo com o resumo (p+0x5f): jogadores (0x6b6cc0), eventos
/// (0x6b74a0), contratos (0x4ca6d0), suspensoes (0x55ca00), 0x76b600 (fora do -3), estados
/// (0x62c080) e, fora do -3, a historia ([0xb74434] metodo 1, fora da [0x9cf684]), 0x7c0d70,
/// 0x8c39f0, 0x444930, 0x868830, 0x4134c0, 0x6951e0, 0x59a5d0, 0x7f8c00 e o clube de cada lado
/// (0x6b79a0 com a rodada). Sem `p`, erro 0x73f e 0.
pub(crate) unsafe extern "thiscall" fn close_day_match(day: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    if p == 0 {
        show_error_at(0xa15708, 0x73f);
        return 0;
    }
    let game = u(p, 0);
    core::ptr::copy((p + day_match::FIXTURE_COPY) as *const u8, game as *mut u8, 0x4f);
    let summary = u(p, day_match::SUMMARY);
    let mut round = [0u8; 4];
    let competition = |id: i32| *((u(G_COMP_OBJECT_TABLE, 0) as isize + 4 * id as isize) as *const usize);
    if u(game, fixture::COMPETITION) != 0 {
        crate::comparison::branch("close with competition");
        for k in 0..2 {
            let c = u(game, fixture::AWAY_CLUB);
            if c == 0 || exe::human_manager_is_human_club(c, 0, 1, 0, 0) == 0 || u(c, club::DIVISION) == 0 {
                continue;
            }
            let id = *(u(c, club::DIVISION) as *const i32);
            if id != *(game as *const i32) {
                continue;
            }
            let comp = competition(*(u(game, fixture::COMPETITION) as *const i32));
            if comp == 0 || u(comp, 8) == 0 {
                continue;
            }
            let r = exe::comp_stats_find_club_entry(u(competition(id), 8), c);
            if r != 0 && *((r + 0x2d) as *const i32) > 1 {
                round[k] = *((r + 0x2d) as *const u8);
                crate::comparison::branch("close human round");
            }
        }
        if read_i8(p + day_match::HOME_GOALS) == -3 {
            crate::comparison::branch("close away match");
            let mut copia = [0u8; 0x100];
            core::ptr::copy((p + day_match::FIXTURE_COPY) as *const u8, copia.as_mut_ptr(), 0x4f);
            for o in 0x43..=0x4a {
                copia[o] = 0xff;
            }
            let w = core::ptr::read_unaligned(copia.as_ptr().add(0x4d) as *const u16) & 0xcfff;
            core::ptr::write_unaligned(copia.as_mut_ptr().add(0x4d) as *mut u16, w);
            exe::weather_post_abandoned_news(u(G_WEATHER, 0), p + day_match::FIXTURE_COPY, ((*((game + fixture::SQUAD_RULES) as *const u16) >> 12) & 3) as u32);
            exe::fixture_manager_find_new_fixture_date(u(G_FIXTURE_MANAGER, 0), (copia.as_mut_ptr()) as *mut u8 as usize);
            exe::fixture_manager_add_fixture(u(G_FIXTURE_MANAGER, 0), (copia.as_mut_ptr()) as *mut u8 as usize, 1);
        } else {
            let c = competition(*(game as *const i32));
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(u(u(c, 0), 4));
            metodo(c, game, summary);
            crate::comparison::branch("close competition method");
        }
    }
    *(G_GAME_UNSAVED_CHANGES_FLAG as *mut u32) = 1;
    exe::match_day_apply_post_match_player_changes(game, summary);
    exe::match_day_process_player_incidents(game);
    let two = |this: usize, f: usize| {
        let g: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(f);
        g(this, game, summary);
    };
    two(u(G_CONTRACT_MANAGER, 0), 0x4ca6d0);
    two(u(G_DISCIPLINE, 0), 0x55ca00);
    if read_i8(p + day_match::HOME_GOALS) != -3 {
        two(u(G_NATIONAL_TEAMS, 0), 0x76b600);
    }
    two(u(G_INJURY, 0), 0x62c080);
    if read_i8(p + day_match::HOME_GOALS) == -3 {
        return 1;
    }
    if *(game as *const u32) != *(KEY_CLUB_COMP_RESERVE as *const u32) {
        let h = u(G_RESERVE_COMP, 0);
        two(h, u(u(h, 0), 4));
        crate::comparison::branch("close story");
    }
    for (this, f) in [(G_PLAYER_STATS, 0x7c0d70), (G_TRANSFER_MANAGER, 0x8c39f0), (G_CLUB_RECORDS, 0x444930), (G_STAFF_RECORDS, 0x868830), (G_AWARD_MANAGER, 0x4134c0), (G_MANAGER_MANAGER, 0x6951e0), (G_FINANCE_TABLE, 0x59a5d0), (G_SCOUT_MANAGER, 0x7f8c00)] {
        two(u(this, 0), f);
    }
    let club: unsafe extern "C" fn(usize, usize, usize, u32, usize) = core::mem::transmute(FN_MATCH_DAY_POST_PLAYER_REACTION_NEWS);
    // simplification: the round goes as a dword with the high bytes zeroed; the original passes stack leftovers in them (read as a byte)
    club(u(game, fixture::HOME_CLUB), u(game, fixture::AWAY_CLUB), game, round[0] as u32, summary);
    club(u(game, fixture::AWAY_CLUB), u(game, fixture::HOME_CLUB), game, round[1] as u32, summary);
    1
}

// simplification: match day creates and frees engines and summaries and calls dozens of routines still
// originals; no comparison of their own, validated by the career (Rust x original), like those of the 149th.

/// Plays the match day: on a network it ties the humans to their matches and plays them to the end; then plays each
/// match without an engine; finally closes each match, redoes the humans' stored tactics and frees the summaries.
/// Returns 1.
///
/// Convention: thiscall, `match_day.cpp`, ~3.3 KB. Plays day `dia` (`+0x19` matches of `0x69` bytes, `+0x21` of them,
/// `+0x25` finished, `+0x34` the human's). It starts the next one (`0x6b5a70`). On a network (`[0xa5a390]`): for each
/// pending match with a human (`+0x53`) it creates the humans' scoreboard (`+0x5b`, `0x11d` bytes), ties each human to
/// his side (`0x765480` `+0xe2`/`+0xe6`, `0x4542f0`, `0x727f00`), stores the highest level (`0x5fcff0`) and passes it
/// on (`0x5fd100`); then it notifies those not playing (`0x71b1b0`), releases the humans and plays their matches to the
/// end.
///
/// Then, each match still without an engine (`+0x47` -1): with a human on a network, it moves the current one to the
/// end (`0x6b5ce0`) and starts everything again; otherwise it creates the engine (`0x6b7e50`, `0x6b7f20`), plays to the
/// end (`0x6b98c0` -1, counting in `+0x25`), creates the summary (`0xd7a` bytes), records it (`0x7293f0`), frees the
/// engine and, if the engine was not in fast mode (`+0` of the engine zero), writes the score with the day's percentage
/// on the bar (`0x66f4e0`, `0x802f80`).
///
/// Finally it moves the current one (`0x6b5ce0` with 1), closes each match (`0x6b5f00`), redoes the humans' stored
/// tactics (`0x5fbe20`, `0x5fbd00`, `0x5b5960`) and frees the summaries; with a score written, the date on the bar
/// (`0x54ba20`).
/// Original: `play_the_day` (0x6b4360, match_day.cpp (~3.3 KB), thiscall).
///
/// [pt] { 0x6b4360 } (thiscall, match_day.cpp, ~3,3 KB): joga o dia `dia` (+0x19 partidas de 0x69 bytes,
/// +0x21 delas, +0x25 terminadas, +0x34 a do humano). Comeca a proxima (0x6b5a70). Em rede
/// ([0xa5a390]): para cada partida pendente com humano (+0x53) cria o placar dos humanos (+0x5b,
/// 0x11d bytes), liga cada humano ao seu lado (0x765480 +0xe2/+0xe6, 0x4542f0, 0x727f00), guarda o
/// maior nivel (0x5fcff0) e o repassa (0x5fd100); depois avisa os que nao jogam (0x71b1b0), solta os
/// humanos e joga ate o fim as partidas deles. Depois, cada partida ainda sem motor (+0x47 -1): com
/// humano em rede, anda a atual ate o fim (0x6b5ce0) e recomeca tudo; senao cria o motor (0x6b7e50,
/// 0x6b7f20), joga ate o fim (0x6b98c0 -1, contando em +0x25), cria o resumo (0xd7a bytes), registra
/// (0x7293f0), libera o motor e, se o motor nao era rapido (+0 do motor zero), escreve o placar com
/// a porcentagem do dia na barra (0x66f4e0, 0x802f80). Por fim anda a atual (0x6b5ce0 com 1), fecha
/// cada partida (0x6b5f00), refaz as taticas guardadas dos humanos (0x5fbe20, 0x5fbd00, 0x5b5960) e
/// libera os resumos; com placar escrito, a data na barra (0x54ba20). Devolve 1.
pub(crate) unsafe extern "thiscall" fn play_the_day(day: usize) -> u32 {
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let por = |p: usize, o: usize, v: usize| core::ptr::write_unaligned((p + o) as *mut usize, v);
    let read = |o: usize| *((day + o) as *const i32);
    let rede = || *(G_MATCH_ENGINE_ENABLED as *const u8) != 0;
    let manager = || u(G_HUMAN_MANAGER, 0);
    let partida = |k: i32| u(day, match_day_fields::MATCHES).wrapping_add((k as isize * 0x69) as usize);
    let humano_a = |i: i32| u(G_STAFF_TABLE, 0).wrapping_add(((*(G_STAFF_COUNT as *const i32) - i - 1) as isize * 0x6e) as usize);
    let human_b = |i: i32| u(G_STAFF_TABLE, 0).wrapping_add(((*(G_STAFF_COUNT as *const i32) + i - 0x10) as isize * 0x6e) as usize);
    let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    // 0x5fe290 (ret 8): the match and the human (the original pushes the human before the test)
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);

    let terminou = |e: u8| e == 2 || e == 8;
    // plays to the end (steps of -1), counting those finished in +0x25
    let until_end = |p: usize| {
        while !terminou(*((p + day_match::STATE) as *const u8)) {
            let e = (exe::cm3_match_engine_run_match_steps(u(p, day_match::ENGINE), -1) as u32) as u8;
            *((p + day_match::STATE) as *mut u8) = e;
            if terminou(e) {
                *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;
            }
        }
    };
    // frees the engine and removes the match from the queue
    let without_engine = |p: usize| {
        let m = u(p, day_match::ENGINE);
        if m != 0 {
            exe::drop_cm3_match_engine(m);
            exe::crt_operator_delete(m);
        }
        por(p, 0x57, 0);
        exe::virtual_staff_release_fixture(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY);
    };
    let new_engine = |p: usize| {
        let mem = exe::crt_operator_new(0xf63b);
        let m = if mem != 0 { exe::new_cm3_match_engine(mem) } else { 0 };
        por(p, 0x57, m);
        m
    };
    let new_summary = |p: usize| {
        let mem = exe::crt_operator_new(0xd7a);
        if mem != 0 {
            exe::crt_vector_constructor_iterator(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        por(p, 0x5f, mem);
        mem
    };
    let b = |p: usize, o: usize| *((p + o) as *const u8);

    'reinicio: loop {
        exe::match_day_start_next_background_match(day);
        let mut vistos = [0u32; 16];
        let mut algum = false;
        if rede() {
            crate::comparison::branch("rede");
            let mut k = 0i32;
            while k < read(match_day_fields::MATCH_COUNT) {
                let p = partida(k);
                let e = read_i8(p + day_match::HOME_GOALS);
                if e != -2 && e <= -1 && u(p, day_match::MARKED) != 0 {
                    exe::virtual_staff_fill_fixture_squads(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -1, 0);
                    let r = exe::crt_operator_new(0x11d);
                    por(p, 0x5b, r);
                    if r == 0 {
                        show_error_at(0xa15708, 0x250);
                    } else {
                        *(r as *mut u8) = 0;
                        for o in [5usize, 0x45, 0x85, 0xc5] {
                            core::ptr::write_bytes((u(p, day_match::SCOREBOARD) + o) as *mut u8, 0, 0x40);
                        }
                        let (mut s13, mut s12) = (0u8, 0u8);
                        for o in 0x105..=0x108 {
                            *((u(p, day_match::SCOREBOARD) + o) as *mut u8) = 0;
                        }
                        for (o, v) in [(0x109usize, 0usize), (0x10d, 0), (0x111, 1), (0x115, 0), (0x119, 0)] {
                            por(u(p, day_match::SCOREBOARD), o, v);
                        }
                        // the highest level (0x5fcff0) between the stored one and that of this human
                        let greater = |s: u8, h: usize| if s as i16 > exe::human_manager_get_commentary_speed(manager(), h as u32) { s } else { (exe::human_manager_get_commentary_speed(manager(), h as u32) as u8) };
                        for i in 0..16 {
                            let h = humano_a(i);
                            if b(h, staff::CLASSIFICATION) == 0 {
                                continue;
                            }
                            let side = if human(u(p, day_match::HOME_CLUB), 0, b(p, day_match::HOME_SIDE_MARK) as u32, h, 0) != 0 {
                                Some((0x20, 0x44))
                            } else if human(u(p, day_match::AWAY_CLUB), 0, b(p, day_match::AWAY_SIDE_MARK) as u32, h, 0) != 0 {
                                Some((0x24, 0x45))
                            } else {
                                None
                            };
                            match side {
                                Some((oc, ob)) => {
                                    por(exe::get_human_manager_record(h), 0xe2, p);
                                    por(exe::get_human_manager_record(h), 0xe6, u(p, oc));
                                    exe::open_manager_club_screen(h, (b(p, ob) == 0) as u32, 0, 0, 0);
                                    s13 = greater(s13, h);
                                }
                                None => {
                                    if exe::human_manager_is_watching_fixture(manager(), p + day_match::FIXTURE_COPY, h) == 0 {
                                        continue;
                                    }
                                    if exe::human_manager_is_on_holiday(manager(), h) != 0 {
                                        exe::human_manager_set_on_holiday(manager(), 0, h);
                                    }
                                    por(exe::get_human_manager_record(h), 0xe2, p);
                                    por(exe::get_human_manager_record(h), 0xe6, 0);
                                    exe::match_screens_open_waiting_screen_for_slot(h);
                                    s12 = greater(s12, h);
                                }
                            }
                            let x = exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, h) as i16 as isize;
                            let scoreboard = u(p, day_match::SCOREBOARD);
                            if core::ptr::read_unaligned((scoreboard as isize + 5 + 4 * x) as *const u32) == 0 {
                                let x = exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, h) as i16 as isize;
                                core::ptr::write_unaligned((u(p, day_match::SCOREBOARD) as isize + 5 + 4 * x) as *mut u32, 1);
                                *((u(p, day_match::SCOREBOARD) + 0x105) as *mut u8) += 1;
                            }
                            // simplification: the index is 0 to 15 (the humans); outside that the original would write to the stack
                            let x = exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, h) as i16;
                            if (0..16).contains(&x) {
                                vistos[x as usize] = 1;
                            }
                        }
                        for i in 0..16 {
                            let h = human_b(i);
                            if b(h, staff::CLASSIFICATION) != 0 && u(exe::get_human_manager_record(h), 0xe2) == p {
                                exe::human_manager_set_commentary_speed(manager(), if s13 > 0 { s13 } else { s12 } as u32, h);
                            }
                        }
                        if u(day, match_day_fields::HUMAN_MATCH) == 0 {
                            por(day, 0x34, p);
                        }
                        algum = true;
                        crate::comparison::branch("network match with humans");
                    }
                }
                k += 1;
                if !rede() {
                    break;
                }
            }
        }
        if algum {
            for i in 0..16 {
                let h = human_b(i);
                if exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, h) != 0xffff {
                    let x = exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, h) as i16;
                    if (0..16).contains(&x) && vistos[x as usize] == 0 {
                        exe::open_latest_scores_screen(h, u(u(day, match_day_fields::HUMAN_MATCH), 0x63));
                    }
                }
            }
            exe::screenmanager_monitor(G_SCREEN_MANAGER);
            for i in 0..16 {
                let h = human_b(i);
                por(exe::get_human_manager_record(h), 0xe2, 0);
                por(exe::get_human_manager_record(h), 0xe6, 0);
            }
            let mut k = 0i32;
            while k < read(match_day_fields::MATCH_COUNT) {
                let p = partida(k);
                if u(p, day_match::MARKED) != 0 && u(p, day_match::ENGINE) != 0 {
                    until_end(p);
                    exe::match_stats_record_match(u(G_MATCH_STATS, 0), u(p, day_match::ENGINE), u(p, day_match::SUMMARY));
                    without_engine(p);
                    exe::crt_operator_delete(u(p, day_match::SCOREBOARD));
                    por(p, 0x5b, 0);
                }
                k += 1;
            }
        }

        // the matches without a human, one by one
        *(G_MATCH_DAY_PLAYING_FLAG as *mut u32) = 1;
        let mut placar_escrito = false;
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        let hour = exe::get_cursor_mode();
        let mut k = 0i32;
        while k < read(match_day_fields::MATCH_COUNT) {
            if *(G_INTERACTIVE_MODE_FLAG as *const u32) != 0 {
                exe::set_cursor_mode(2);
                exe::pump_messages();
                if *(G_MOUSE_LEFT_BUTTON_DOWN as *const u32) != 0 {
                    let mut redesenha = false;
                    for i in 0..16 {
                        let h = humano_a(i);
                        if b(h, staff::CLASSIFICATION) != 0 {
                            if u(h, staff::CLUB_JOB) == 0 && u(h, staff::NATIONAL_JOB) == 0 {
                                exe::news_screens_open_news_screen(h, 0);
                            } else {
                                exe::open_manager_club_screen(h, 0, 0, 0, 0);
                            }
                            redesenha = true;
                        }
                    }
                    if redesenha {
                        exe::screenmanager_monitor(G_SCREEN_MANAGER);
                    }
                    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                }
            }
            let p = partida(k);
            if read_i8(p + day_match::HOME_GOALS) == -1 && u(p, day_match::ENGINE) == 0 {
                let with_human = human(u(p, day_match::HOME_CLUB), 0, b(p, day_match::HOME_SIDE_MARK) as u32, 0, 0) != 0 || human(u(p, day_match::AWAY_CLUB), 0, b(p, day_match::AWAY_SIDE_MARK) as u32, 0, 0) != 0;
                if with_human && rede() {
                    exe::match_day_step_background_match(day, 1);
                    *(G_MATCH_DAY_PLAYING_FLAG as *mut u32) = 0;
                    crate::comparison::branch("networked human restarts");
                    continue 'reinicio;
                }
                exe::virtual_staff_fill_fixture_squads(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY, -2, 0);
                let m = new_engine(p);
                if m == 0 {
                    show_error_at(0xa15708, 0x39d);
                } else {
                    let r = exe::match_day_can_quick_simulate(p + day_match::FIXTURE_COPY);
                    crate::comparison::branch("match without human");
                    exe::cm3_match_engine_init_match(m, p + day_match::FIXTURE_COPY, r, 0, 0);
                    let rapido = *(u(p, day_match::ENGINE) as *const i8) != 0;
                    until_end(p);
                    let summary = new_summary(p);
                    if summary == 0 {
                        show_error_at(0xa15708, 0x3b5);
                    } else {
                        exe::match_stats_record_match(u(G_MATCH_STATS, 0), u(p, day_match::ENGINE), summary);
                        without_engine(p);
                        if !rapido {
                            crate::comparison::branch("score in the bar");
                            // the scoreboard with the day’s percentage on the bar
                            let n = read(match_day_fields::MATCH_COUNT);
                            let pct = if n != 0 { read(match_day_fields::FINISHED_COUNT).wrapping_mul(100) / n } else { 0 } as i8 as i32;
                            let (c, casa, fora) = (u(p, day_match::COMPETITION), u(p, day_match::HOME_CLUB), u(p, day_match::AWAY_CLUB));
                            let (a52, b52) = (read_i8(casa + club::SHORT_GENDER_NAME) as i32, read_i8(fora + club::SHORT_GENDER_NAME) as i32);
                            let (g1, g2) = if read_i8(p + day_match::HOME_AGGREGATE) > -1 { (read_i8(p + day_match::HOME_AGGREGATE), read_i8(p + day_match::AWAY_AGGREGATE)) } else { (read_i8(p + day_match::HOME_GOALS), read_i8(p + day_match::AWAY_GOALS)) };
                            if c != 0 {
                                crate::comparison::branch("score with competition");
                                let c37 = read_i8(c + 0x37) as i32;
                                text(0xde1f64, 0xa157b4, c37, c37, a52, a52, b52, b52, c + 4, casa + club::SHORT_NAME, g1 as i32, fora + club::SHORT_NAME, g2 as i32, pct, 0xa15850usize);
                            } else {
                                text(0xde1f64, 0xa15734, a52, a52, b52, b52, casa + club::SHORT_NAME, g1 as i32, fora + club::SHORT_NAME, g2 as i32, pct, 0xa15850usize);
                            }
                            exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, 0xde1f64, -1, 0, -1);
                            placar_escrito = true;
                        }
                    }
                }
            }
            k += 1;
        }

        // the closing of each match
        exe::set_cursor_mode(hour);
        exe::match_day_step_background_match(day, 1);
        *(G_MATCH_DAY_PLAYING_FLAG as *mut u32) = 0;
        let mut k = 0i32;
        while k < read(match_day_fields::MATCH_COUNT) {
            let p = partida(k);
            if read_i8(p + day_match::HOME_GOALS) != -2 {
                if u(p, day_match::SUMMARY) == 0 {
                    show_error_at(0xa15708, 0x3fb);
                } else {
                    exe::match_day_apply_match_result(day, p);
                    crate::comparison::branch("match_wrap_up");
                    for (oc, ob) in [(0x20usize, 0x44usize), (0x24, 0x45)] {
                        if human(u(p, oc), 0, 1, 0, 0) != 0 && exe::database_get_club_manager(u(p, oc)) != 0 {
                            let x = exe::database_get_club_manager(u(p, oc));
                            crate::comparison::branch("human lines redone");
                            exe::human_manager_select_tactic_slot(manager(), (b(p, ob) == 0) as i32, u(p, oc), x);
                            exe::formation_clear_marking_targets(exe::human_manager_get_active_tactic(manager(), u(u(p, oc), 0xcf)));
                        }
                    }
                    let s = u(p, day_match::SUMMARY);
                    if s != 0 {
                        exe::crt_vector_destructor_iterator(s + 0x11f, 0x625, 2, 0x453c10);
                        exe::crt_operator_delete(s);
                    }
                    por(p, 0x5f, 0);
                }
            }
            k += 1;
        }
        if placar_escrito {
            crate::comparison::branch("date in the bar");
            exe::date_get_long_date_string(G_CURRENT_DATE, 0xde1f64, 0x7d0, *(G_CURRENT_DAY_SLICE as *const u8) as u32);
            exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, 0xde1f64, -1, 0, -1);
        }
        return 1;
    }
}

// simplification: the day loop creates and frees engines and summaries and touches the screen; no comparison of its own,
// validated by the career (Rust x original), like those of the 151st.

/// Sets up the match day (date, mode 0..2): zeroes the arrays, gathers the pending matches of each list, records the
/// already decided ones, groups by list and by competition, points each match to its competition, creates the global
/// list and works out the day's interest. Returns the day.
///
/// Sets up day `dia` with the matches of date `data` in mode `modo` (0, 1 or 2; otherwise or with no date, error
/// `0x93`). Zeroes the day (`+0` mode; arrays `+1` groups of 24 bytes, `+0xd` competitions of `0x54`, `+0x19` matches
/// of `0x69`, each with capacity and count; `+0x2f` -1, `+0x33` 1).
///
/// For each of the `[0xb64738]` lists (`0x5ae6a0`): the pending matches (`+0x43` <= -1, except -2) go into the day
/// (copy of `0x4f` bytes; `+0x67` -1), those decided earlier (`0x923630` bit 0) are recorded and marked -2 (`0x923cc0`,
/// `0x5ab510`/`0x5aa680`), the others notified (`0x79c7b0`); the list's group (`0x48dcd0`) stores the range of its
/// matches, which is sorted (qsort with `0x6b6510`) and split by competition (`+4` of the copy), and each match points
/// to its own (`+0x63`).
///
/// Finally the global list `[0xb67dc8]` (`0x68d020`) and the day's interest (`0x6b5040`). The arrays grow 5 at a time
/// (`0x945667`; out of memory, errors `0xd6`, `0xf4`, `0x140`).
/// Original: `set_up_the_day` (0x6b3c10, match_day.cpp, thiscall, ret 8).
///
/// [pt] { 0x6b3c10 } (thiscall, ret 8, match_day.cpp): monta o dia `dia` com as partidas da data `data`
/// no modo `modo` (0, 1 ou 2; fora disso ou sem data, erro 0x93). Zera o dia (+0 modo; vetores +1
/// grupos de 24 bytes, +0xd competicoes de 0x54, +0x19 partidas de 0x69, cada um com capacidade e
/// contagem; +0x2f -1, +0x33 1). Para cada uma das [0xb64738] listas (0x5ae6a0): as partidas
/// pendentes (+0x43 <= -1, fora -2) entram no dia (copia de 0x4f bytes; +0x67 -1), as decididas
/// antes (0x923630 bit 0) registradas e marcadas -2 (0x923cc0, 0x5ab510/0x5aa680), as outras
/// avisadas (0x79c7b0); o grupo da lista (0x48dcd0) guarda a faixa das partidas, que e ordenada
/// (qsort com 0x6b6510) e repartida por competicao (+4 da copia), e cada partida aponta a sua
/// (+0x63). Por fim a lista global [0xb67dc8] (0x68d020) e o interesse do dia (0x6b5040). Os vetores
/// crescem de 5 em 5 (0x945667; sem memoria, erros 0xd6, 0xf4, 0x140). Devolve `dia`.
pub(crate) unsafe extern "thiscall" fn set_up_the_day(day: usize, data: usize, mode: u32) -> usize {
    let por = |p: usize, o: usize, v: u32| core::ptr::write_unaligned((p + o) as *mut u32, v);
    let modo8 = mode as u8;
    *(day as *mut u8) = modo8;
    for o in (1..=0x25).step_by(4) {
        por(day, o, 0);
    }
    *((day + match_day_fields::MARKED_GROUP_COUNT) as *mut u8) = 0;
    *((day + match_day_fields::MARKED_COMPETITION_COUNT) as *mut u8) = 0;
    por(day, 0x2b, 0);
    por(day, 0x2f, u32::MAX);
    *((day + match_day_fields::PASS) as *mut u8) = 1;
    por(day, 0x34, 0);
    *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *mut usize) = 0;
    if data == 0 || modo8 > 2 {
        show_error_at(0xa15708, 0x93);
        return day;
    }
    exe::reservecomp_cancel_unplayable_reserve_fixtures(read_usize(G_RESERVE_COMP + 0));
    // grows an array (pointer +o, capacity +o+4, count +o+8) in steps of 5; false when out of memory
    let crescer = |o: usize, size: u32| -> bool {
        if read_i32(day + (o + 8)) == read_i32(day + (o + 4)) {
            let cap = read_i32(day + (o + 4)) + 5;
            por(day, o + 4, cap as u32);
            let p = exe::crt_realloc(read_usize(day + o), (cap as u32).wrapping_mul(size));
            por(day, o, p as u32);
            if p == 0 {
                return false;
            }
        }
        true
    };
    let mut comp_current = 0usize;
    let mut i = 0i32;
    while i < *(G_KEY_NATION_ACTIVE_COUNT as *const i8) as i32 {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER + 0), data, mode, i as u32);
        if list != 0 && *(list as *const i16) > 0 {
            por(list, 0xa, read_usize(list + 2) as u32);
            let mut group = 0usize;
            let mut x = exe::llist_get_next(list);
            while x != 0 {
                let e = read_i8(x + fixture::HOME_GOALS);
                if e <= -1 && e != -2 {
                    let r = exe::weather_check_postponement(read_usize(G_WEATHER + 0), x);
                    if r & 1 != 0 {
                        crate::comparison::branch("setup decided before");
                        let mut copia = [0u8; 0x100];
                        core::ptr::copy(x as *const u8, copia.as_mut_ptr(), 0x4f);
                        *((x + fixture::HOME_GOALS) as *mut u8) = 0xfe;
                        // simplification: the 2nd argument goes with only the byte; the original passes stack leftovers in the high part (read as a byte)
                        exe::weather_post_postponed_news(read_usize(G_WEATHER + 0), x, r as u32);
                        exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER + 0), (copia.as_mut_ptr()) as *mut u8 as usize);
                        exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER + 0), (copia.as_mut_ptr()) as *mut u8 as usize, 1);
                    } else {
                        exe::officialsmanager_choose_match_official(read_usize(G_OFFICIALS_MANAGER + 0), x, x + fixture::REFEREE);
                        crate::comparison::branch("setup notified");
                    }
                    if group == 0 {
                        crate::comparison::branch("setup new group");
                        if !crescer(1, 24) {
                            show_error_at(0xa15708, 0xd6);
                            return day;
                        }
                        group = read_usize(day + match_day_fields::GROUPS) + read_i32(day + match_day_fields::GROUP_COUNT) as usize * 24;
                        por(group, 0, exe::comp_key_nation_entry_from_active_index(i as u32));
                        for o in [4usize, 8, 0xc, 0x10] {
                            por(group, o, u32::MAX);
                        }
                        por(group, 0x14, 0);
                        por(day, 9, (read_i32(day + match_day_fields::GROUP_COUNT) + 1) as u32);
                    }
                    if !crescer(0x19, 0x69) {
                        show_error_at(0xa15708, 0xf4);
                        return day;
                    }
                    let p = read_usize(day + match_day_fields::MATCHES) + read_i32(day + match_day_fields::MATCH_COUNT) as usize * 0x69;
                    por(p, 0, x as u32);
                    core::ptr::copy(x as *const u8, (p + 4) as *mut u8, 0x4f);
                    for o in [0x53usize, 0x57, 0x5b, 0x5f, 0x63] {
                        por(p, o, 0);
                    }
                    *((p + 0x67) as *mut u8) = 0xff;
                    if read_usize(group + day_group::FIRST_MATCH) as i32 == -1 {
                        por(group, 0xc, read_i32(day + match_day_fields::MATCH_COUNT) as u32);
                    }
                    por(group, 0x10, read_i32(day + match_day_fields::MATCH_COUNT) as u32);
                    por(day, 0x21, (read_i32(day + match_day_fields::MATCH_COUNT) + 1) as u32);
                }
                x = exe::llist_get_next(list);
            }
            if group != 0 {
                let (de, ate) = (read_usize(group + day_group::FIRST_MATCH) as i32, read_usize(group + day_group::LAST_MATCH) as i32);
                exe::crt_qsort(read_usize(day + match_day_fields::MATCHES) + de as usize * 0x69, (ate - de + 1) as u32, 0x69, 0x6b6510);
                let mut last = -1i32;
                let mut k = read_usize(group + day_group::FIRST_MATCH) as i32;
                while k <= read_usize(group + day_group::LAST_MATCH) as i32 {
                    let p = read_usize(day + match_day_fields::MATCHES) + k as usize * 0x69;
                    let id = read_usize(p + 4) as i32;
                    if id != last {
                        crate::comparison::branch("setup new competition");
                        if !crescer(0xd, 0x54) {
                            show_error_at(0xa15708, 0x140);
                            return day;
                        }
                        let c = read_usize(day + match_day_fields::COMPETITIONS) + read_i32(day + match_day_fields::COMPETITION_COUNT) as usize * 0x54;
                        comp_current = c;
                        por(c, 0, id as u32);
                        por(c, 4, u32::MAX);
                        por(c, 8, u32::MAX);
                        por(c, 0xc, 0);
                        por(c, 0x10, 0);
                        core::ptr::write_bytes((c + 0x14) as *mut u8, 0, 0x40);
                        if read_usize(group + day_group::FIRST_COMPETITION) as i32 == -1 {
                            por(group, 4, read_i32(day + match_day_fields::COMPETITION_COUNT) as u32);
                        }
                        por(group, 8, read_i32(day + match_day_fields::COMPETITION_COUNT) as u32);
                        por(day, 0x15, (read_i32(day + match_day_fields::COMPETITION_COUNT) + 1) as u32);
                        last = read_usize(p + 4) as i32;
                    }
                    let c = comp_current;
                    if read_usize(c + 4) as i32 == -1 {
                        por(c, 4, k as u32);
                    }
                    por(c, 8, k as u32);
                    k += 1;
                }
            }
        }
        i += 1;
    }
    let mut i = 0i32;
    while i < read_i32(day + match_day_fields::COMPETITION_COUNT) {
        let c = read_usize(day + match_day_fields::COMPETITIONS) + i as usize * 0x54;
        let mut k = read_usize(c + 4) as i32;
        while k <= read_usize(c + 8) as i32 {
            por(read_usize(day + match_day_fields::MATCHES) + k as usize * 0x69, 0x63, c as u32);
            k += 1;
        }
        i += 1;
    }
    let mem = exe::crt_operator_new(0xe);
    *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *mut usize) = if mem != 0 { exe::new_llist(mem) } else { 0 };
    exe::match_day_refresh_human_interest_flags(day);
    day
}

// simplification: the day’s setup allocates the vectors and calls routines still original; the side and the sounds are
// read-only and screen. No comparison of its own, validated by the career (Rust x original).

/// Memory regions `exe::match_day_refresh_human_interest_flags` marks for a match day: day +0x29..+0x2e, each group +0x14, each competition +0xc..+0x13 and
/// each match +0x53.
///
/// [pt] As regioes que a 0x6b5040 marca no dia `d`: +0x29 a +0x2e do dia, +0x14 de cada grupo, +0xc a
/// +0x13 de cada competicao e +0x53 de cada partida.
pub(crate) unsafe fn match_day_marked_regions(d: usize) -> Vec<(usize, usize)> {
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let mut v = vec![(d + match_day_fields::MARKED_GROUP_COUNT, 6)];
    for gi in 0..read(d + match_day_fields::GROUP_COUNT).max(0) as usize {
        let grp = (read(d + match_day_fields::GROUPS) as usize).wrapping_add(gi * 0x18);
        v.push((grp + 0x14, 4));
        for ci in read(grp + 4)..=read(grp + 8) {
            let comp = (read(d + match_day_fields::COMPETITIONS) as isize + ci as isize * 0x54) as usize;
            v.push((comp + 0xc, 8));
            for mi in read(comp + 4)..=read(comp + 8) {
                v.push(((read(d + match_day_fields::MATCHES) as isize + mi as isize * 0x69) as usize + 0x53, 4));
            }
        }
    }
    v
}

/// Marks on match day `d` what interests the human managers: clears the counters (+0x29 groups, +0x2a competitions,
/// +0x2b any), then for each group and competition flags the competition (+0xc) when one of the 16 human managers'
/// clubs (+0x39 or +0x24) plays in it, and flags each fixture (+0x53) involving a human club or watched by a human,
/// counting the groups and competitions with such fixtures; other fixtures of human interest (`exe::match_day_is_fixture_of_human_interest`) only flag the
/// competition. Any flag sets +0x2b. Returns 1.
///
/// [pt] { 0x6b5040 } (thiscall, match_day.cpp): marca no dia de jogos `d` o que interessa ao humano.
/// Primeiro a lista dos humanos (0x5fe8d0 em [0xb63c98]); zera +0x29, +0x2a e +0x2b. Para cada grupo
/// (+1, de 0x18 em 0x18, quantos em +9) e cada competicao dele (+4 a +8; em +0xd, de 0x54 em 0x54):
/// a competicao interessa (+0xc) se um dos 16 humanos no fim da tabela de pessoas tem o clube (+0x39)
/// ou o outro vinculo (+0x24) nela (0x48db80 da competicao em [[0xadadfc] + id x 4]). Cada partida
/// dela (+4 a +8; em +0x19, de 0x69 em 0x69): com clube humano (0x600a50 com +0x20/+0x44 ou
/// +0x24/+0x45) ou pela 0x5fe420 a partida e marcada (+0x53) e conta o grupo (+0x29) e a competicao
/// (+0x2a) na primeira vez (+0x14 do grupo, +0x10 da competicao); senao, pela 0x6b61e0, so a
/// competicao interessa. Qualquer marca liga +0x2b do dia. Devolve 1.
pub(crate) unsafe extern "thiscall" fn match_day_interest(d: usize) -> u32 {
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let pr = |a: usize, v: i32| core::ptr::write_unaligned(a as *mut i32, v);
    exe::human_manager_clear_invalid_watched_fixtures(g(G_HUMAN_MANAGER));
    *((d + match_day_fields::MARKED_GROUP_COUNT) as *mut u8) = 0;
    *((d + match_day_fields::MARKED_COMPETITION_COUNT) as *mut u8) = 0;
    pr(d + match_day_fields::HAS_INTEREST, 0);
    let mut gi = 0i32;
    while gi < read(d + match_day_fields::GROUP_COUNT) {
        let grp = (read(d + match_day_fields::GROUPS) as usize).wrapping_add(gi as usize * 0x18);
        pr(grp + day_group::HAS_MARKED, 0);
        let mut ci = read(grp + day_group::FIRST_COMPETITION);
        while ci <= read(grp + day_group::LAST_COMPETITION) {
            let comp = (read(d + match_day_fields::COMPETITIONS) as isize + ci as isize * 0x54) as usize;
            pr(comp + day_competition::INTERESTING, 0);
            pr(comp + day_competition::HAS_MARKED, 0);
            if read(comp) > -1 {
                let object = g(g(G_COMP_OBJECT_TABLE) + read(comp) as usize * 4);
                if (0..16).filter(|k| g((g(G_STAFF_TABLE) as isize + (read(G_STAFF_COUNT) + k - 16) as isize * 0x6e) as usize + 0x39) != 0).count() >= 2 {
                    crate::comparison::branch("interest two humans with club");
                }
                for k in 0..16 {
                    let pe = Staff((g(G_STAFF_TABLE) as isize + (read(G_STAFF_COUNT) + k - 16) as isize * 0x6e) as usize);
                    let club = g(pe.addr() + staff::CLUB_JOB);
                    let other = g(pe.addr() + staff::NATIONAL_JOB);
                    if (club != 0 && exe::comp_includes_club(object, club) != 0) || (other != 0 && exe::comp_includes_club(object, other) != 0) {
                        crate::comparison::branch("interest human in the competition");
                        pr(comp + day_competition::INTERESTING, 1);
                        pr(d + match_day_fields::HAS_INTEREST, 1);
                    }
                }
            }
            let mut mi = read(comp + day_competition::FIRST_MATCH);
            while mi <= read(comp + day_competition::LAST_MATCH) {
                let m = (read(d + match_day_fields::MATCHES) as isize + mi as isize * 0x69) as usize;
                pr(m + day_match::MARKED, 0);
                let marked = (exe::human_manager_is_human_club(g(m + day_match::HOME_CLUB), 0, *((m + day_match::HOME_SIDE_MARK) as *const u8) as u32, 0, 0) as i32) != 0
                    || (exe::human_manager_is_human_club(g(m + day_match::AWAY_CLUB), 0, *((m + day_match::AWAY_SIDE_MARK) as *const u8) as u32, 0, 0) as i32) != 0
                    || exe::human_manager_is_fixture_watched(g(G_HUMAN_MANAGER), m + day_match::FIXTURE_COPY) != 0;
                if marked {
                    crate::comparison::branch("interest match scheduled");
                    pr(m + day_match::MARKED, 1);
                    if read(grp + day_group::HAS_MARKED) == 0 {
                        *((d + match_day_fields::MARKED_GROUP_COUNT) as *mut u8) = (*((d + match_day_fields::MARKED_GROUP_COUNT) as *const u8)).wrapping_add(1);
                    }
                    if read(comp + day_competition::HAS_MARKED) == 0 {
                        *((d + match_day_fields::MARKED_COMPETITION_COUNT) as *mut u8) = (*((d + match_day_fields::MARKED_COMPETITION_COUNT) as *const u8)).wrapping_add(1);
                    }
                    pr(grp + day_group::HAS_MARKED, 1);
                    pr(comp + day_competition::HAS_MARKED, 1);
                    pr(comp + day_competition::INTERESTING, 1);
                    pr(d + match_day_fields::HAS_INTEREST, 1);
                } else if exe::match_day_is_fixture_of_human_interest(m + day_match::FIXTURE_COPY) != 0 {
                    crate::comparison::branch("interest via 6b61e0");
                    pr(comp + day_competition::INTERESTING, 1);
                    pr(d + match_day_fields::HAS_INTEREST, 1);
                }
                mi += 1;
            }
            ci += 1;
        }
        gi += 1;
    }
    1
}

/// Addresses of the national-team marks (0xb67a58 + id) of human managers with flag bit 2 and a national team.
///
/// [pt] As marcas das selecoes dos tecnicos humanos (0xb67a58 + id), que a 0x6b66a0 poe uma vez
/// (com [0xa15704] ligado): para cada vaga de humano em 0xb63d60 (de 0x48 em 0x48 ate 0xb64738) com
/// +0x11c bit 2 e selecao (+0x71).
pub(crate) unsafe fn human_national_team_marks() -> Vec<usize> {
    let mut v = Vec::new();
    for e in (0xb63d60usize..G_KEY_NATION_ACTIVE_COUNT).step_by(0x48) {
        let p = *(e as *const usize);
        if p != 0 && *((p + nation::LEAGUE_SELECTED) as *const u8) & 2 != 0 {
            let q = *((p + nation::CONTINENT) as *const usize);
            if q != 0 {
                v.push(0xb67a58 + *(q as *const usize));
            }
        }
    }
    v
}

/// Decides whether a fixture can be played without the human (quick simulation): returns 0 when a human-managed or
/// linked club plays, for certain fixed competitions (world club competitions, Welsh Premier Cup with England/Wales
/// flagged), when a human manages a club of the same nation, in some cases when the clubs' reputations (+0x80) add up
/// to more than 12000, or when the competition's national team is flagged for a human; otherwise 1 if no scouted player
/// is involved (`fixture_has_scouted_player`). First marks the human managers' national teams once.
///
/// [pt] { 0x6b66a0 } (cdecl, match_day.cpp): 1 se a `partida` nao envolve o humano, pela 0x7f9720. Antes,
/// uma vez ([0xa15704]), marca as selecoes dos humanos em 0xb67a58. Sem [0xdd7118]: 1. Devolve 0 se:
/// algum dos clubes (+0x1c, +0x20) e de humano (0x600a50), ou o clube ligado a ele (0x540a50); a
/// competicao (+0) e uma de duas fixas ([0x9cf7bc], [0x9cf6e4]); e a de [0x9cf958] com um dos dois
/// clubes fixos ([0x9cf2e4], [0x9cf510], linhas de 0x122 em [0xae23a8]) humano (+0x11c bit 2); um dos
/// 16 humanos no fim da tabela de pessoas (+0x5f = 1) treina clube da mesma nacao (+0x57) de um dos
/// dois. Na competicao [0x9cf684] sem clube humano, direto a 0x7f9720. Fora de selecao (0x53b900) com
/// 0x5c2990, ou na competicao [0x9cf6f8]: 0 se o +0x80 dos dois somar mais de 12000. Senao a vaga da
/// 0x48dd70 (pela tabela em 0xb63d74): 0 ou 1 da 0; nas 3 a 7 com a selecao da competicao marcada, 0;
/// vaga de humano com +0x11c bit 2, 0. No fim: 1 se a 0x7f9720 der 0.
pub(crate) unsafe extern "C" fn match_without_human(partida: usize) -> u32 {
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16) as i32;
    if g(G_QUICK_SIM_COMP_FLAGS_PENDING) != 0 {
        for e in human_national_team_marks() {
            crate::comparison::branch("no human human national team");
            *(e as *mut u8) = 1;
        }
        *(G_QUICK_SIM_COMP_FLAGS_PENDING as *mut u32) = 0;
    }
    if g(G_INTERACTIVE_MODE_FLAG) == 0 {
        return 1;
    }
    let (c1, c2) = (g(partida + fixture::HOME_CLUB), g(partida + fixture::AWAY_CLUB));
    if human(c1, 1, 1, 0, 0) != 0 || human(c2, 1, 1, 0, 0) != 0 {
        crate::comparison::branch("no human human club on the pitch");
        return 0;
    }
    for c in [c1, c2] {
        if exe::club_get_linked_b_team(c, (0) as *mut u32, 1) != 0 && human(exe::club_get_linked_b_team(c, (0) as *mut u32, 1), 1, 1, 0, 0) != 0 {
            crate::comparison::branch("no human linked club");
            return 0;
        }
    }
    let final_ = || (exe::scout_manager_fixture_has_scouted_player(g(G_SCOUT_MANAGER), partida) == 0) as u32;
    let comp = read(partida);
    if comp == read(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) || comp == read(KEY_CLUB_COMP_WORLD_CLUB_CUP) {
        return 0;
    }
    if comp == read(KEY_CLUB_COMP_WELSH_PREMIER_CUP) {
        let base = g(G_NATION_TABLE);
        for k in [read(KEY_NATION_ENGLAND), read(KEY_NATION_WALES)] {
            if *((base as isize + k as isize * 0x122 + 0x11c) as *const u8) & 2 != 0 {
                return 0;
            }
        }
    }
    if comp == read(KEY_CLUB_COMP_RESERVE) && human(c1, 1, 1, 0, 0) == 0 && human(c2, 1, 1, 0, 0) == 0 {
        crate::comparison::branch("no human direct competition");
        return final_();
    }
    // the 16 humans at the end of the people table
    let base = g(G_STAFF_TABLE);
    for k in 0..16isize {
        let pe = Staff((base as isize + (read(G_STAFF_COUNT) as isize - 1 - k) * 0x6e) as usize);
        if !pe.is_null() && pe.classification() == 1 && g(pe.addr() + staff::CLUB_JOB) != 0 {
            let nation = read(g(pe.addr() + staff::CLUB_JOB) + 0x57);
            if nation == read(c1 + club::DIVISION) || nation == read(c2 + club::DIVISION) {
                crate::comparison::branch("no human nation manager");
                return 0;
            }
        }
    }
    if ((exe::database_is_national_team(c2) as i32) == 0 && (exe::friendly_is_own_comp_record(g(G_FRIENDLY), (read(partida + fixture::COMPETITION) as u32) as usize) as i32) != 0) || comp == read(KEY_CLUB_COMP_INTER_TOTO_CUP) {
        crate::comparison::branch("no human by injury_word_0c");
        return if w(c2 + club::REPUTATION) + w(c1 + club::REPUTATION) > 0x2ee0 { 0 } else { final_() };
    }
    let mut x = (exe::comp_active_key_nation_of_fixture((read(partida + fixture::COMPETITION) as u32) as u32 as usize, partida) as u32) as u8;
    if let Some(k) = (0xb63d74usize..G_KEY_NATION_ACTIVE_SLOT_LOOP_END).step_by(0x48).position(|e| *(e as *const u8) == x) {
        x = k as u8;
    }
    if x == 0 || x == 1 {
        return 0;
    }
    if matches!(x, 3..=7) {
        let q = g(g(partida + fixture::COMPETITION) + 0x59);
        if q != 0 && *((0xb67a58 + g(q)) as *const u8) != 0 {
            crate::comparison::branch("no human national team marked");
            return 0;
        }
    }
    if (x as i8) > -1 {
        let e = g(G_LEAGUE_TABLE + x as usize * 0x48);
        if e != 0 && *((e + nation::LEAGUE_SELECTED) as *const u8) & 2 != 0 {
            return 0;
        }
    }
    crate::comparison::branch("no human 7f9720");
    final_()
}

/// `exe::match_day_post_player_reaction_news` (cdecl, match_day.cpp; called twice at 0x6b61a9/0x6b61bd, once per club) with calls injected: the
/// post-match news of `clube` after the match against `outro`.
///
/// No club, other club or match: error box. No manager (+0xcf): nothing; without `exe::human_manager_is_human_club`, with +0x80 below 4750 and
/// random(100) != 0: nothing.
///
/// With 0x6b7cf0, match +0x40 and +0x41 and the club not out (`exe::database_is_national_team`): among the 50 of +0xd7, with a contract
/// (`exe::contract_manager_get_contract`) without +0x45, the player sheet's +0xb above 4250, +0x59 above 5, +0x57 above 15, +0x58 above 10, `n`
/// above 2 and random(n) != 0: 0x5896d0(person, club, other, n) and end (the turnaround story for a senior player).
///
/// Then, the 20 of the club's row in record `t` (side by club at t+0; rows of 0x49 from `engine::SIDE_STRENGTH`, person at -0x3e, only
/// those with +0 in the row): player sheet, youth product, age (+0x18) up to 20, contract without +0x45, the match of
/// the club's competition (+0x14 = +0x57, with +0x40/+0x41), the club not out, +0x59 above 15, +0x57 above random(5) +
/// 10, the player sheet's +0xb below 4250 and 0x7c3a40 zero: 0x589bc0 and, without `exe::staff_favours_club`, `exe::staff_add_favoured_club`(person, club, 1),
/// and end (a pleased youngster favouring the club).
///
/// [pt] { 0x6b79a0 } (cdecl, match_day.cpp; chamada duas vezes em 0x6b61a9/0x6b61bd, uma por clube): o
/// `clube` depois da partida contra `outro`. Sem clube, outro ou partida: caixa de erro. Sem tecnico
/// (+0xcf) nada; sem 0x600a50, com +0x80 abaixo de 4750 e sorteio(100) != 0, nada. Com 0x6b7cf0,
/// partida +0x40 e +0x41 e o clube nao fora (0x53b900): entre os 50 de +0xd7, com contrato (0x4cf5a0)
/// sem +0x45, +0xb da ficha acima de 4250, +0x59 acima de 5, +0x57 acima de 15, +0x58 acima de 10,
/// `n` acima de 2 e sorteio(n) != 0: 0x5896d0(pessoa, clube, outro, n) e fim. Depois, os 20 da
/// linha do clube no registro `t` (lado pelo clube em t+0; linhas de 0x49 a partir de +0x1ce, pessoa
/// em -0x3e, so os com +0 na linha): ficha, da base, idade (+0x18) ate 20, contrato sem +0x45, a
/// partida da competicao do clube (+0x14 = +0x57, com +0x40/+0x41), o clube nao fora, +0x59 acima
/// de 15, +0x57 acima de sorteio(5) + 10, +0xb da ficha abaixo de 4250 e 0x7c3a40 zero: 0x589bc0 e,
/// sem 0x547d60, 0x547d90(pessoa, clube, 1), e fim.
pub(crate) unsafe fn post_match_club_with(club: Club, other: usize, partida: usize, n: u32, t: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let ch = core::cell::RefCell::new(call);
    let out = |s: usize, c: usize, o: usize, n: i32| (ch.borrow_mut())(0x5896d0, 0, [s as u32, c as u32, o as u32, n as u32, 0]);
    let f589bc0 = |p: usize, c: usize| (ch.borrow_mut())(0x589bc0, 0, [p as u32, c as u32, 0, 0, 0]);
    let f547d90 = |p: usize, c: usize, um: u32| (ch.borrow_mut())(0x547d90, 0, [p as u32, c as u32, um, 0, 0]);
    let test_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let test_6b7cf0 = |pa: usize, c: usize| (ch.borrow_mut())(0x6b7cf0, 0, [pa as u32, c as u32, 0, 0, 0]) as u8;
    let away = |c: usize| (ch.borrow_mut())(0x53b900, 0, [c as u32, 0, 0, 0, 0]) as i32;
    let test_7c3a40 = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x7c3a40, n, [p as u32, c as u32, 0, 0, 0]) as u16;
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let ub = |a: usize| *(a as *const u8);
    if club.is_null() || partida == 0 || other == 0 {
        // simplification: the original routine shows the “Error” box (match_day.cpp, lines 0xc45/0xc4a/0xc4f); no
        // happens in the game
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        return;
    }
    if g(club.addr() + club::MANAGER) == 0 {
        return;
    }
    if test_600a50(club.addr(), 0, 1, 0, 0) == 0 && w(club.addr() + club::REPUTATION) < 0x128e && random(100) != 0 {
        return;
    }
    crate::comparison::branch("club continues");
    let n = n as u8 as i8;
    if test_6b7cf0(partida, club.addr()) != 0 && ub(partida + fixture::HOME_SIDE_MARK) != 0 && ub(partida + fixture::AWAY_SIDE_MARK) != 0 && away(club.addr()) == 0 {
        for k in 0..50 {
            let s = g(club.addr() + club::SQUAD + 4 * k);
            if s == 0 {
                continue;
            }
            let c = exe::contract_manager_get_contract(g(G_CONTRACT_MANAGER), s);
            let free = c != 0 && read(c + 0x45) == 0;
            if free {
                crate::comparison::branch("club squad");
            }
            if free
                && g(s + staff::PLAYER) != 0
                && w(g(s + staff::PLAYER) + 0xb) > 0x109a
                && read_i8(s + staff::LOYALTY) > 5
                && read_i8(s + staff::AMBITION) > 0xf
                && read_i8(s + staff::DETERMINATION) > 0xa
                && n > 2
                && random(n as i32) != 0
            {
                crate::comparison::branch("club 5896d0");
                out(s, club.addr(), other, n as i32);
                return;
            }
        }
    }
    let side = (read(club.addr()) != read(t)) as usize;
    for k in 0..20 {
        let row = t + 0x625 * side + 0x1ce + k * 0x49;
        let id = read(row - 0x3e);
        if id == -1 || ub(row) == 0 {
            continue;
        }
        let p = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
        if g(p + staff::PLAYER) == 0 || (p != 0 && read(p) >= read(G_STAFF_COUNT)) || read_i8(p + staff::YEAR_OF_BIRTH) > 0x14 {
            continue;
        }
        let c = exe::contract_manager_get_contract(g(G_CONTRACT_MANAGER), p);
        if c == 0 || read(c + 0x45) != 0 || ub(partida + fixture::HOME_SIDE_MARK) == 0 || ub(partida + fixture::AWAY_SIDE_MARK) == 0 {
            continue;
        }
        if read(partida + fixture::COMPETITION) == 0 || read(partida + fixture::COMPETITION) != read(club.addr() + club::DIVISION) || away(club.addr()) != 0 || read_i8(p + staff::LOYALTY) <= 0xf {
            continue;
        }
        crate::comparison::branch("club youth");
        let r = random(5);
        if read_i8(p + staff::AMBITION) <= (r as u8).wrapping_add(10) as i8 || g(p + staff::PLAYER) == 0 || w(g(p + staff::PLAYER) + 0xb) >= 0x109a {
            continue;
        }
        if test_7c3a40(g(G_PLAYER_STATS), p, club.addr()) == 0 {
            crate::comparison::branch("club 589bc0");
            f589bc0(p, club.addr());
            if (exe::staff_favours_club(p, club) as i32) == 0 {
                crate::comparison::branch("club 547d90");
                f547d90(p, club.addr(), 1);
            }
            return;
        }
    }
}

/// After a match, processes the post-match events of `partida` (injuries, news, morale and contract effects per event
/// type); thin wrapper over post_match_events_injected with the real calls.
///
/// [pt] { 0x6b74a0 }: ver `post_match_events_injected`.
pub(crate) unsafe extern "C" fn post_match_events(partida: usize) {
    post_match_events_injected(partida, &mut |r, t, a| dispatch_post_match_call(r, t, a))
}

/// After a match: updates the players and staff of team `t` in fixture `partida` (form and ratings copied to the player
/// records, possible contract clauses, the manager's and assistants' stats and the club's reputation +0x80). Wrapper of
/// `post_match_players_with` with the real outside calls.
///
/// [pt] { 0x6b6cc0 }: ver `post_match_players_with`.
pub(crate) unsafe extern "C" fn post_match_players(partida: usize, t: usize) {
    post_match_players_with(partida, t, &mut |r, tt, a| dispatch_post_match_call(r, tt, a))
}

/// Post-match news for `clube` after the match `partida` against `outro` (turnaround story for a senior player, or a
/// pleased youngster favouring the club); calls post_match_club_with with the game's real calls.
///
/// [pt] { 0x6b79a0 }: ver `post_match_club_with`.
pub(crate) unsafe extern "C" fn post_match_club(club: usize, other: usize, partida: usize, n: u32, t: usize) {
    post_match_club_with(Club(club), other, partida, n, t, &mut |r, tt, a| dispatch_post_match_call(r, tt, a))
}

/// Calls one of the post-match routines with effects by game address, with the right convention and argument count.
///
/// [pt] As rotinas do banco com efeitos chamadas pelo pos-jogo, pelo endereco do jogo (`this` 0: cdecl).
pub(crate) unsafe fn dispatch_post_match_call(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    type T1 = unsafe extern "thiscall" fn(usize, u32) -> u32;
    type T2 = unsafe extern "thiscall" fn(usize, u32, u32) -> u32;
    type T3 = unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32;
    type T4 = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32;
    type T5 = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32;
    type C4 = unsafe extern "C" fn(u32, u32, u32, u32) -> u32;
    type C5 = unsafe extern "C" fn(u32, u32, u32, u32, u32) -> u32;
    match routine {
        0x4cf5f0 | 0x7c3a40 => core::mem::transmute::<usize, T2>(routine)(this, a[0], a[1]),
        0x600a50 => core::mem::transmute::<usize, C5>(routine)(a[0], a[1], a[2], a[3], a[4]),
        0x8676d0 | 0x8674e0 | 0x867560 => core::mem::transmute::<usize, T1>(routine)(this, a[0]),
        0x867700 | 0x733a60 | 0x5afe00 => core::mem::transmute::<usize, T2>(routine)(this, a[0], a[1]),
        0x759610 => core::mem::transmute::<usize, T3>(routine)(this, a[0], a[1], a[2]),
        0x744240 => core::mem::transmute::<usize, T4>(routine)(this, a[0], a[1], a[2], a[3]),
        0x72c1f0 => core::mem::transmute::<usize, T5>(routine)(this, a[0], a[1], a[2], a[3], a[4]),
        // cdecl: the caller cleans the stack, the extra arguments are not read
        _ => core::mem::transmute::<usize, C4>(routine)(a[0], a[1], a[2], a[3]),
    }
}

// --- post-match ports: the 14 database routines with side effects (and the query 0x7c3a40), recorded on the tape only during the
// post-match comparisons (RECORD_POST_MATCH); for the game’s other callers they just pass through. They return
// edx:eax integer (none returns floating point: checked in the 132nd batch).
pub(crate) static RECORD_POST_MATCH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static mut PORTAS_POS: [usize; 15] = [0; 15];
pub(crate) const POS_PROLOGUES: [Prologue; 15] = [
    Prologue { address: 0x8676d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x40] },
    Prologue { address: 0x8674e0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologue { address: 0x867560, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologue { address: 0x867700, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] },
    Prologue { address: 0x733a60, bytes: &[0x6a, 0xff, 0x68, 0xe6, 0xd6, 0x95, 0x00] },
    Prologue { address: 0x5afe00, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] },
    Prologue { address: 0x759610, bytes: &[0x6a, 0xff, 0x68, 0x83, 0xd9, 0x95, 0x00] },
    Prologue { address: 0x744240, bytes: &[0x6a, 0xff, 0x68, 0x06, 0xd8, 0x95, 0x00] },
    Prologue { address: 0x72c1f0, bytes: &[0x6a, 0xff, 0x68, 0x86, 0xd6, 0x95, 0x00] },
    Prologue { address: 0x7daf50, bytes: &[0x6a, 0xff, 0x68, 0x70, 0xfd, 0x95, 0x00] },
    Prologue { address: 0x547af0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologue { address: 0x5896d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologue { address: 0x589bc0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologue { address: 0x547d90, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologue { address: 0x7c3a40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
];

/// True inside a post-match comparison with the tape open.
///
/// [pt] Dentro de uma comparacao do pos-jogo, com a fita aberta.
pub(crate) fn is_recording_post_match() -> bool {
    RECORD_POST_MATCH.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// Runs a post-match call directly, or records it on the tape during a post-match comparison.
///
/// [pt] Repassa ou grava (durante uma comparacao do pos-jogo, no nivel de cima da fita).
pub(crate) unsafe fn pass_or_record_post_match(routine: usize, this: usize, a: [u32; 5], f: impl FnOnce() -> u64) -> u64 {
    use std::sync::atomic::Ordering::Relaxed;
    if RECORD_POST_MATCH.load(Relaxed) && is_recording() {
        let mut r = 0u64;
        call_recording(routine, this, a, || {
            r = f();
            r as u32
        });
        return r;
    }
    f()
}

/// Gate for STAFF_CONTRACT::clear_flags (`exe::staff_contract_clear_flags`) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_clear_contract_flags(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[0]);
    pass_or_record_post_match(0x8676d0, c, [a, 0, 0, 0, 0], || f(c, a))
}
/// Gate for STAFF_CONTRACT::clear_grievances (`exe::staff_contract_clear_grievances`) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_clear_grievances(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[1]);
    pass_or_record_post_match(0x8674e0, c, [a, 0, 0, 0, 0], || f(c, a))
}
/// Gate for STAFF_CONTRACT::set_flags (`set_flags`) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_set_contract_flags(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[2]);
    pass_or_record_post_match(0x867560, c, [a, 0, 0, 0, 0], || f(c, a))
}
/// Gate for STAFF_CONTRACT::apply_event_flags (`exe::staff_contract_apply_event_flags`) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_apply_event_flags(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[3]);
    pass_or_record_post_match(0x867700, c, [a, b, 0, 0, 0], || f(c, a, b))
}
/// Gate for Media::create_player_outburst_story (0x733a60) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_player_outburst_story(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[4]);
    pass_or_record_post_match(0x733a60, c, [a, b, 0, 0, 0], || f(c, a, b))
}
/// Gate for fog_of_war_reveal_match_players (0x5afe00) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_reveal_match_players(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[5]);
    pass_or_record_post_match(0x5afe00, c, [a, b, 0, 0, 0], || f(c, a, b))
}
/// Gate for Media::queue_player_sent_off_story (0x759610) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_sent_off_story(c: usize, a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[6]);
    pass_or_record_post_match(0x759610, c, [a, b, d, 0, 0], || f(c, a, b, d))
}
/// Gate for Media::post_player_praise_story (0x744240) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_player_praise_story(c: usize, a: u32, b: u32, d: u32, e: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[7]);
    pass_or_record_post_match(0x744240, c, [a, b, d, e, 0], || f(c, a, b, d, e))
}
/// Gate for Media::create_player_media_story (`exe::media_create_player_media_story`) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_player_media_story(c: usize, a: u32, b: u32, d: u32, e: u32, g: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[8]);
    pass_or_record_post_match(0x72c1f0, c, [a, b, d, e, g], || f(c, a, b, d, e, g))
}
/// Gate for apply_obscene_gesture_ban (0x7daf50) in post-match recording.
pub(crate) unsafe extern "C" fn gate_obscene_gesture_ban(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[9]);
    pass_or_record_post_match(0x7daf50, 0, [a, b, d, 0, 0], || f(a, b, d))
}
/// Gate for staff_add_disliked_staff (`exe::staff_add_disliked_staff`) in post-match recording.
pub(crate) unsafe extern "C" fn gate_add_disliked_staff(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[10]);
    pass_or_record_post_match(0x547af0, 0, [a, b, d, 0, 0], || f(a, b, d))
}
/// Gate for news_post_player_ecstatic_at_turnaround (0x5896d0) in post-match recording.
pub(crate) unsafe extern "C" fn gate_news_ecstatic_turnaround(a: u32, b: u32, d: u32, e: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[11]);
    pass_or_record_post_match(0x5896d0, 0, [a, b, d, e, 0], || f(a, b, d, e))
}
/// Gate for news_post_player_pleased_with_chance (0x589bc0) in post-match recording.
pub(crate) unsafe extern "C" fn gate_news_pleased_with_chance(a: u32, b: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[12]);
    pass_or_record_post_match(0x589bc0, 0, [a, b, 0, 0, 0], || f(a, b))
}
/// Gate for staff_add_favoured_club (`exe::staff_add_favoured_club`) in post-match recording.
pub(crate) unsafe extern "C" fn gate_add_favoured_club(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[13]);
    pass_or_record_post_match(0x547d90, 0, [a, b, d, 0, 0], || f(a, b, d))
}

/// Gate for player_stats_get_apps_for_club (0x7c3a40) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_apps_for_club(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[14]);
    pass_or_record_post_match(0x7c3a40, c, [a, b, 0, 0, 0], || f(c, a, b))
}

/// Addresses of the 15 post-match gate functions, in PORTAS_POS order.
pub(crate) fn post_match_gates() -> [usize; 15] {
    [
        gate_clear_contract_flags as *const () as usize, gate_clear_grievances as *const () as usize, gate_set_contract_flags as *const () as usize,
        gate_apply_event_flags as *const () as usize, gate_player_outburst_story as *const () as usize, gate_reveal_match_players as *const () as usize,
        gate_sent_off_story as *const () as usize, gate_player_praise_story as *const () as usize, gate_player_media_story as *const () as usize,
        gate_obscene_gesture_ban as *const () as usize, gate_add_disliked_staff as *const () as usize, gate_news_ecstatic_turnaround as *const () as usize,
        gate_news_pleased_with_chance as *const () as usize, gate_add_favoured_club as *const () as usize, gate_apps_for_club as *const () as usize,
    ]
}

/// The flags (+0x40..+0x48) of the person's contract at a club as a comparer region, or None.
///
/// [pt] As flags do contrato (+0x40 a +0x48) de `pessoa` no `clube`, como regiao.
pub(crate) unsafe fn contract_flags_region(person: usize, club: usize) -> Option<(usize, usize)> {
    let c = exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), person, club);
    (c != 0).then_some((c + 0x40, 9))
}

/// The current human leaves the match he was watching; with no other human watching, the match runs to the end, is
/// recorded and leaves the queue.
///
/// The current human (`[0xb77c7a]`) leaves the match he was watching (called only when a network player leaves,
/// `0x838c00`). His match (`[0xb74c26 + 0x300 × turn]`, `0x765480` → `+0xe2`): if another of the 16 humans (the last 16
/// records of `[0xae23bc]`, 110 bytes, `+0x5f` active) watches the same one and it is his turn (`0x7fffa0` on the
/// manager), it leaves silently; otherwise it removes the turn from the four arrays of the summary (`+0x5b`:
/// `+5`/`+0x45`/`+0x85`/`+0xc5`, the counters `+0x105..+0x108`).
///
/// Nobody else watching: it creates the engine (`+0x57`, `0xf63b` bytes, `0x6b7e50` + `0x6b7f20`; without it error
/// `0xa4b`), runs to the end (`0x6b98c0` with −1; state `+0x67`, 2 or 8 count in `[[0xb67a50]+0x25]`), except for 8
/// creates the summary (`+0x5f`, `0xd7a` bytes with 2 objects of `0x625` at `+0x11f`; without it error `0xa68`) and
/// records it (`0x7293f0`), frees the engine and takes the match off queue `[0xde2f70]` (`0x90d6b0`).
/// Original: `match_day_drop_viewer_and_finish_match` (0x6b6970, match_day.cpp, cdecl).
///
/// [pt] { 0x6b6970 } (cdecl, match_day.cpp): o humano da vez ([0xb77c7a]) deixa a partida que assistia
/// (chamada só quando um jogador da rede sai, 0x838c00). A partida dele ([0xb74c26 + 0x300 × vez],
/// 0x765480 → +0xe2): se outro dos 16 humanos (as últimas 16 fichas de [0xae23bc], 110 bytes, +0x5f
/// ativo) assiste a mesma e é a vez dele (0x7fffa0 no gerente), sai calado; senão tira a vez dos
/// quatro vetores do resumo (+0x5b: +5/+0x45/+0x85/+0xc5, os contadores +0x105..+0x108). Ninguém mais
/// assistindo: cria o motor (+0x57, 0xf63b bytes, 0x6b7e50 + 0x6b7f20; sem ele erro 0xa4b), anda até
/// o fim (0x6b98c0 com −1; estado +0x67, 2 ou 8 contam em [[0xb67a50]+0x25]), fora do 8 cria o resumo
/// (+0x5f, 0xd7a bytes com 2 objetos de 0x625 em +0x11f; sem ele erro 0xa68) e registra (0x7293f0),
/// libera o motor e tira a partida da fila [0xde2f70] (0x90d6b0).
pub(crate) unsafe extern "C" fn match_day_drop_viewer_and_finish_match() {
    crate::comparison::branch("screens leave the watched match");
    let team = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;

    let p = read_usize(((0xb74c26 + team() * 0x300) as usize) + 0);
    if p == 0 {
        return;
    }
    let a = exe::get_human_manager_record(p);
    if a == 0 {
        return;
    }
    let game = read_usize(a + 0xe2);
    if game == 0 {
        return;
    }
    let mut sozinho = true;
    for i in 0..0x10usize {
        let idx = (*(G_STAFF_COUNT as *const usize)).wrapping_add(i).wrapping_sub(0x10);
        let q = Staff(read_usize(G_STAFF_TABLE + 0).wrapping_add(idx.wrapping_mul(110)));
        // like the original, without checking the other human’s null record
        if q.classification() != 0 && q.addr() != p && read_usize((exe::get_human_manager_record(q)) + 0xe2) == game {
            let minha = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const u16);
            if exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, q) == minha {
                return;
            }
            sozinho = false;
        }
    }
    let summary = read_usize(game + day_match::SCOREBOARD);
    if summary == 0 {
        return;
    }
    for (k, o) in [5isize, 0x45, 0x85, 0xc5].into_iter().enumerate() {
        let c = (summary as isize + team() * 4 + o) as *mut u32;
        if core::ptr::read_unaligned(c) != 0 {
            core::ptr::write_unaligned(c, 0);
            let n = (summary + 0x105 + k) as *mut u8;
            *n = (*n).wrapping_sub(1);
        }
    }
    if !sozinho {
        return;
    }
    let engine = (game + day_match::ENGINE) as *mut usize;
    if core::ptr::read_unaligned(engine) == 0 {
        let mem = exe::crt_operator_new(0xf63b);
        let m = if mem != 0 { exe::new_cm3_match_engine(mem) } else { 0 };
        core::ptr::write_unaligned(engine, m);
        if m == 0 {
            show_error_at(0xa15708, 0xa4b);
            return;
        }
        exe::cm3_match_engine_init_match(m, game + day_match::FIXTURE_COPY, 0, 0, 0);
    }
    let state = (game + day_match::STATE) as *mut u8;
    while *state != 2 && *state != 8 {
        let e = (exe::cm3_match_engine_run_match_steps(core::ptr::read_unaligned(engine), -1) as u32) as u8;
        *state = e;
        if e == 2 || e == 8 {
            *((read_usize(G_MATCH_DAY + 0) + 0x25) as *mut i32) += 1;
        }
    }
    if *state != 8 {
        let mem = exe::crt_operator_new(0xd7a);
        if mem != 0 {
            exe::crt_vector_constructor_iterator(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        core::ptr::write_unaligned((game + day_match::SUMMARY) as *mut usize, mem);
        if mem == 0 {
            show_error_at(0xa15708, 0xa68);
            return;
        }
        exe::match_stats_record_match(read_usize(G_MATCH_STATS + 0), core::ptr::read_unaligned(engine), mem);
    }
    let m = core::ptr::read_unaligned(engine);
    if m != 0 {
        exe::drop_cm3_match_engine(m);
        exe::crt_operator_delete(m);
    }
    core::ptr::write_unaligned(engine, 0);
    exe::virtual_staff_release_fixture(read_usize(G_VIRTUAL_STAFF + 0), game + day_match::FIXTURE_COPY);
}

/// Finds in the fixture list the match of a statistics record (date, competition and club) and copies the fixture.
/// Returns 1 if found.
///
/// The fixture (`0x4f` bytes) of record `k` of the statistics index (`[this]`, 12 bytes: `+0` competition, `+4` one of
/// the clubs, `+8`/`+0xa` the date): it asks the schedule `[0xae2a58]` (`0x5acbe0`) for the matches of that date and
/// competition in a local list (`0x68d020`) and copies to `saida` the first one with club `+4` at home (`+0xc`) or away
/// (`+0x10`). Index outside 0..`[this+0x1c]` or null output: the original routine runs (error box, 0).
/// Original: `Match_Stats::find_record_fixture` (0x729e30, match_stats.cpp, thiscall).
///
/// [pt] { 0x729e30 } (thiscall, ret 8, match_stats.cpp): a ficha do jogo (0x4f bytes) do registro `k` do
/// índice de estatísticas ([this], 12 bytes: +0 competição, +4 um dos clubes, +8/+0xa a data): pede
/// à agenda [0xae2a58] (0x5acbe0) os jogos daquela data e competição numa lista local (0x68d020) e copia
/// para `saida` o primeiro com o clube +4 na casa (+0xc) ou fora (+0x10). Devolve 1 se achou. Índice
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn find_record_fixture(stats: usize, k: i32, out: usize) -> u32 {
    if k < 0 || k > (*((stats + match_stats::CAPACITY) as *const i32)).wrapping_sub(1) || out == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_FIND_RECORD_FIXTURE)(stats, k, out);
    }
    // simplification: the list is born zeroed here, which is all the constructor 0x68d020 does
    let mut list = [0u8; 14];
    let l = list.as_mut_ptr() as usize;
    let table = *(stats as *const usize);
    let e = table.wrapping_add((k as usize).wrapping_mul(12));
    // the original pushes the whole registers: on top, the 16 bits of [this] and of k × 3
    let d1 = (table as u32 & 0xffff_0000) | *((e + 8) as *const u16) as u32;
    let d2 = ((k as u32).wrapping_mul(3) & 0xffff_0000) | *((e + 0xa) as *const u16) as u32;
    (exe::fixture_manager_find_fixtures(*(G_FIXTURE_MANAGER as *const usize), d1, d2, d1, d2, *(e as *const u32), l, u32::MAX, 0xffff_fffe, u32::MAX) as u32);
    // the cursor (+0xa) starts at the head (+2)
    core::ptr::copy(list.as_ptr().add(2), list.as_mut_ptr().add(0xa), 4);
    let club = *((e + 4) as *const u32);
    let mut f = exe::llist_get_next(l);
    while f != 0 {
        if club == *((f + fixture::HOME_CLUB_ID) as *const u32) || club == *((f + fixture::AWAY_CLUB_ID) as *const u32) {
            core::ptr::copy_nonoverlapping(f as *const u8, out as *mut u8, 0x4f);
            exe::drop_llist(l);
            return 1;
        }
        f = exe::llist_get_next(l);
    }
    exe::drop_llist(l);
    0
}

// simplification: the abandoned match creates and frees the engine, plays the match to the end and records it in the file
// temporary: running twice is not possible, so the comparer only calls the original; the copy proves itself in the
// Rust mode (the branch), in Lucas’s network session (only a network player leaving calls it).

/// One step of the human's match on a network, by the clock and the humans' scoreboard (everyone sees each state;
/// sounds, events, engine step, end, half time, goals, minute); records it and picks the day's human match. Returns 1
/// if it changed.
///
/// One step of human match `p` on a network, by the clock (`0x90c510`) and the humans' scoreboard (`+0x5b`: `+0` state,
/// `+1` event, `+0x85` who has already seen it, `+0x105` humans, `+0x107` seen, `+0x108` pause, `+0x109` wait, `+0x10d`
/// time, `+0x111` may advance, `+0x115` highlight). Without `p`, it moves the current one (`0x6b5ce0`) and starts the
/// next (`0x6b5a70`).
///
/// It waits for everyone to see the state; then the sounds (`0x6d8c70`) and the time; it shows the next event
/// (`0x6d78b0`, highlight by table `0x6d5cd0`) or takes one engine step (`0x6b98c0` with 1) and sets the state: end
/// (2/8), half time (4), goals on each side (`+0xf5bd`/`+0xf5f3` against the summary), highlight, `+0x8eb2`, the minute
/// (`+0x8ed0` / 11) against the summary (wait `0x226` - 50 x `+0x51`); with a state, it records it (`0x729ff0`) and the
/// day's human match (`+0x34`) becomes the one with the highest competition (`0x6b6570`) or the furthest along.
/// Original: `network_step` (0x6b55c0, match_day.cpp, thiscall, ret 4).
///
/// [pt] { 0x6b55c0 } (thiscall, ret 4, match_day.cpp): um passo da partida do humano `p` em rede, pelo
/// relogio (0x90c510) e pelo placar dos humanos (+0x5b: +0 estado, +1 evento, +0x85 quem ja viu,
/// +0x105 humanos, +0x107 vistos, +0x108 pausa, +0x109 espera, +0x10d hora, +0x111 pode andar,
/// +0x115 lance). Sem `p`, anda a atual (0x6b5ce0) e comeca a proxima (0x6b5a70). Espera todos verem
/// o estado; depois os sons (0x6d8c70) e o tempo; mostra o proximo evento (0x6d78b0, lance pela
/// tabela 0x6d5cd0) ou da um passo do motor (0x6b98c0 com 1) e marca o estado: fim (2/8), intervalo
/// (4), gols de cada lado (+0xf5bd/+0xf5f3 contra o resumo), lance, +0x8eb2, o minuto (+0x8ed0 / 11)
/// contra o resumo (espera 0x226 - 50 x +0x51); com estado, registra (0x729ff0) e a do humano do dia
/// (+0x34) passa a ser a de maior competicao (0x6b6570) ou mais adiantada. Devolve 1 se mudou.
pub(crate) unsafe extern "thiscall" fn network_step(day: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| core::ptr::read_unaligned((q + o) as *const usize);
    let por = |q: usize, o: usize, v: u32| core::ptr::write_unaligned((q + o) as *mut u32, v);
    let follow = || {
        if exe::match_day_step_background_match(day, 0) != 0 {
            exe::match_day_start_next_background_match(day);
        }
        0u32
    };
    if p == 0 {
        return follow();
    }
    let pl = u(p, day_match::SCOREBOARD);
    crate::comparison::branch("step_towards in network");
    let engine = u(p, day_match::ENGINE);
    let state = pl as *mut u8;
    let eu = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let visto = |i: isize| ((pl as isize) + 0x85 + 4 * i) as *mut u32;
    // everyone turns again: only the current human marks and waits
    let rever = || {
        core::ptr::write_bytes((pl + 0x85) as *mut u8, 0, 0x40);
        core::ptr::write_unaligned(visto(eu), 1);
        *((pl + 0x107) as *mut u8) = 1;
    };
    let mostrar_evento = |ev: usize| {
        if exe::match_events_get_event_priority(u(engine, engine::COMMENTATOR), *(ev as *const u16) as u32) == 2 {
            por(pl, 0x115, 1);
        }
        por(pl, 1, ev as u32);
        *state = 8;
        rever();
        por(pl, 0x109, 0);
    };
    if *state != 0 && *((pl + 0x107) as *const u8) != *((pl + 0x105) as *const u8) {
        if core::ptr::read_unaligned(visto(eu)) != 0 {
            return 0;
        }
        core::ptr::write_unaligned(visto(eu), 1);
        *((pl + 0x107) as *mut u8) += 1;
        por(pl, 0x10d, exe::utils_get_time_ms());
        return 1;
    }
    if *((pl + 0x108) as *const u8) != 0 {
        return follow();
    }
    if *state & 8 != 0 && exe::match_events_is_line_finished(u(engine, engine::COMMENTATOR), pl + 5) == 0 {
        return follow();
    }
    let espera = if *((u(engine, engine::COMMENTATOR) + 0x2db0) as *const u8) == 0x14 { 0 } else { u(pl, 0x109) as u32 };
    if exe::utils_get_time_ms() < (u(pl, 0x10d) as u32).wrapping_add(espera) {
        return follow();
    }
    if *state & 8 != 0 {
        let ev = exe::match_events_get_next_event(u(engine, engine::COMMENTATOR), 0);
        if ev != 0 {
            mostrar_evento(ev);
            por(pl, 0x10d, exe::utils_get_time_ms());
            return 1;
        }
    }
    if u(pl, 0x111) != 0 {
        por(pl, 0x115, 0);
        let r = (exe::cm3_match_engine_run_match_steps(engine, 1) as u32) as u8;
        *((p + day_match::STATE) as *mut u8) = r;
        if r == 2 || r == 8 {
            *((day + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;
        }
        if *((u(engine, engine::COMMENTATOR) + 5) as *const u8) != 0 {
            let ev = exe::match_events_get_next_event(u(engine, engine::COMMENTATOR), 0);
            if ev != 0 {
                mostrar_evento(ev);
                por(pl, 0x10d, exe::utils_get_time_ms());
                por(pl, 0x111, 0);
                return 1;
            }
        }
    }
    *state = 0;
    let end = *((p + day_match::STATE) as *const u8);
    let summary = u(p, day_match::SUMMARY);
    let mark = |bit: u8| {
        por(pl, 0x109, 0);
        *(pl as *mut u8) |= bit;
    };
    if end == 2 {
        *state = 0x40;
        por(pl, 0x109, 0);
    }
    if *((p + day_match::STATE) as *const u8) == 8 {
        mark(0x40);
    }
    if *((p + day_match::STATE) as *const u8) == 4 {
        mark(0x20);
    }
    if read_i8(engine + engine::SCORE) as i32 > *((summary + match_summary::SIDES) as *const u8) as i32 {
        mark(1);
    }
    if read_i8(engine + engine::SCORE_SIDE1) as i32 > *((u(p, day_match::SUMMARY) + 0x744) as *const u8) as i32 {
        mark(2);
    }
    if u(pl, 0x115) != 0 {
        mark(0x10);
    }
    if *((engine + engine::RESTART_TYPE) as *const u8) != 0 {
        mark(0x80);
    }
    let r = u(p, day_match::SUMMARY);
    if *((engine + engine::CLOCK) as *const i16) as i32 / 11 > *((r + 0x4d) as *const i16) as i32 {
        *state |= 4;
        por(pl, 0x109, (0x226 - 50 * *((u(p, day_match::SUMMARY) + 0x51) as *const i16) as i32) as u32);
    }
    por(pl, 0x111, 1);
    if *state == 0 {
        return 0;
    }
    rever();
    por(pl, 0x10d, exe::utils_get_time_ms());
    exe::match_stats_fill_match_record(u(G_MATCH_STATS, 0), engine, u(p, day_match::SUMMARY), 1);
    let s = u(u(day, match_day_fields::HUMAN_MATCH), 0x5f);
    let swap = if s != 0 {
        let a = exe::match_day_get_period_rank(*((u(p, day_match::SUMMARY) + 0x4f) as *const u16) as u32, 1) as i8;
        let b = exe::match_day_get_period_rank(*((s + 0x4f) as *const u16) as u32, 1) as i8;
        a > b || *((u(p, day_match::SUMMARY) + 0x4d) as *const i16) > *((u(u(day, match_day_fields::HUMAN_MATCH), 0x5f) + 0x4d) as *const i16)
    } else {
        true
    };
    if swap {
        por(day, 0x34, p as u32);
    }
    1
}

/// Each human confirms the network match; with all of them confirmed, creates the engine and prepares it for watching,
/// or plays it to the end and records it.
///
/// The current human (`[0xb77c7a]`) confirms match `p` on a network (humans' scoreboard `+0x5b`: `+0x45` who confirmed,
/// `+0x105` humans, `+0x106` confirmed). With all: it creates the engine (`0x6b7e50`) and prepares it for watching
/// (`0x6b7f20` with the last argument 1). If it is not one to be watched (returns 0), it plays to the end (counting in
/// `[0xb67a50]+0x25`), creates the summary, records it (`0x7293f0`), frees the engine, takes it off the queue and sets
/// `+0x119`; if it is, it notifies the commentator (`+0x8f7a` = 3), creates the summary, records it for watching
/// (`0x729ff0`) and returns 1. Errors `0x4b8` (no match), `0x4d0`, `0x4ea`, `0x511` (out of memory).
/// Original: `confirm_network_match` (0x6b5260, match_day.cpp, thiscall, ret 4).
///
/// [pt] { 0x6b5260 } (thiscall, ret 4, match_day.cpp): o humano da vez ([0xb77c7a]) confirma a partida
/// `p` em rede (placar dos humanos +0x5b: +0x45 quem confirmou, +0x105 humanos, +0x106 confirmados).
/// Com todos: cria o motor (0x6b7e50) e o prepara para ver (0x6b7f20 com o ultimo 1). Se ele nao
/// for de ver (devolve 0), joga ate o fim (contando em [0xb67a50]+0x25), cria o resumo, registra
/// (0x7293f0), libera o motor, tira da fila e marca +0x119; se for, avisa o narrador (+0x8f7a = 3),
/// cria o resumo e registra para ver (0x729ff0) e devolve 1. Erros 0x4b8 (sem partida), 0x4d0, 0x4ea,
/// 0x511 (sem memoria).
pub(crate) unsafe extern "thiscall" fn confirm_network_match(day: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| core::ptr::read_unaligned((q + o) as *const usize);
    let por = |q: usize, o: usize, v: usize| core::ptr::write_unaligned((q + o) as *mut usize, v);
    const FONTE: usize = 0xa15708; // match_day.cpp
    if p == 0 {
        show_error_at(FONTE, 0x4b8);
        return 0;
    }
    let pl = u(p, day_match::SCOREBOARD);
    let eu = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let confirmou = (pl as isize + eu * 4 + 0x45) as *mut u32;
    if core::ptr::read_unaligned(confirmou) == 0 {
        core::ptr::write_unaligned(confirmou, 1);
        *((pl + 0x106) as *mut u8) = (*((pl + 0x106) as *const u8)).wrapping_add(1);
    }
    if *((pl + 0x106) as *const u8) != *((pl + 0x105) as *const u8) {
        return 0;
    }
    crate::comparison::branch("network match confirmed");
    let mem = exe::crt_operator_new(0xf63b);
    let engine = if mem != 0 { exe::new_cm3_match_engine(mem) } else { 0 };
    por(p, 0x57, engine);
    if engine == 0 {
        show_error_at(FONTE, 0x4d0);
        return 0;
    }
    let new_summary = || {
        let r = exe::crt_operator_new(0xd7a);
        if r != 0 {
            exe::crt_vector_constructor_iterator(r + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        por(p, 0x5f, r);
        r
    };
    if exe::cm3_match_engine_init_match(engine, p + day_match::FIXTURE_COPY, 0, 0, 1) as i8 == 0 {
        let terminou = |e: u8| e == 2 || e == 8;
        let state = (p + day_match::STATE) as *mut u8;
        while !terminou(*state) {
            let e = (exe::cm3_match_engine_run_match_steps(u(p, day_match::ENGINE), -1) as u32) as u8;
            *state = e;
            if terminou(e) {
                *((u(G_MATCH_DAY, 0) + 0x25) as *mut i32) += 1;
            }
        }
        let r = new_summary();
        if r == 0 {
            show_error_at(FONTE, 0x4ea);
            return 0;
        }
        exe::match_stats_record_match(u(G_MATCH_STATS, 0), u(p, day_match::ENGINE), r);
        let m = u(p, day_match::ENGINE);
        if m != 0 {
            exe::drop_cm3_match_engine(m);
            exe::crt_operator_delete(m);
        }
        por(p, 0x57, 0);
        exe::virtual_staff_release_fixture(u(G_VIRTUAL_STAFF, 0), p + day_match::FIXTURE_COPY);
        por(pl, 0x119, 1);
        return 0;
    }
    let commentator = u(u(p, day_match::ENGINE), 0x8f7a);
    if commentator != 0 {
        *(commentator as *mut u8) = 3;
    }
    let r = new_summary();
    if r == 0 {
        show_error_at(FONTE, 0x511);
        return 0;
    }
    exe::match_stats_fill_match_record(u(G_MATCH_STATS, 0), u(p, day_match::ENGINE), r, 1);
    1
}

/// the number by code (0: 5/4, 1: 11/10, 0x1ef: 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; the first with the second
/// argument 1); any other code: the original (error box). Compared by the result
/// Original: `competition_number` (0x6b6570, match_day.cpp (304 bytes), cdecl).
///
/// [pt] { 0x6b6570 } (cdecl, match_day.cpp): o numero da competicao pelo codigo (0: 5/4, 1: 11/10, 0x1ef:
/// 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; o primeiro com `segunda` 1). Outro codigo: a original
/// (caixa de erro).
pub(crate) unsafe extern "C" fn competition_number(code: u32, second: u32) -> u32 {
    let s = (second as u8 == 1) as u32;
    crate::comparison::branch("competition number");
    match code as u16 as i16 {
        0 => 4 + s,
        1 => 10 + s,
        0x1ef => s,
        0x3de => 2 + s,
        0x483 => 6 + s,
        0x528 => 8 + s,
        _ => original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_COMPETITION_NUMBER)(code, second),
    }
}

