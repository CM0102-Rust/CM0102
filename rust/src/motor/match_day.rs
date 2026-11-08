use super::*;

/// Does the club play in today's fixture list ([this+0x19], count at +0x21, 0x69 bytes per match, home at +0x20 and
/// away at +0x24)? With `so_marcados`, only fixtures with +0x5b set count. Returns 1 or 0 (0 for a null club).
///
/// [pt] { 0x6b59a0 } (match_day.cpp): o clube tem jogo na lista do dia ([this + 0x19], quantidade em
/// 0x21, 0x69 bytes por jogo; mandante em 0x20, visitante em 0x24)? Com `so_marcados`, so os jogos
/// com 0x5b preenchido. Clube nulo: o original mostra uma caixa de erro e devolve 0 (aqui, so 0).
pub(crate) unsafe extern "thiscall" fn has_match(this: usize, clube: u32, so_marcados: u32) -> u32 {
    if clube == 0 {
        return 0;
    }
    let n = *((this + 0x21) as *const i32);
    let base = *((this + 0x19) as *const usize) + 0x20;
    for k in 0..n.max(0) as usize {
        let jogo = base + 0x69 * k;
        if so_marcados != 0 && *((jogo + 0x3b) as *const u32) == 0 {
            continue;
        }
        if *(jogo as *const u32) == clube || *((jogo + 4) as *const u32) == clube {
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
pub(crate) unsafe extern "C" fn won_the_match(jogo: usize, clube: u32) -> u8 {
    if jogo == 0 || clube == 0 {
        return 0;
    }
    let mandante = *((jogo + 0x1c) as *const u32) == clube;
    if !mandante && *((jogo + 0x20) as *const u32) != clube {
        return 0;
    }
    for (a, b) in [(0x49, 0x4a), (0x47, 0x48), (0x43, 0x44)] {
        let (pa, pb) = (byte(jogo, a), byte(jogo, b));
        if pa > -1 {
            return (if mandante { pa > pb } else { pa < pb }) as u8;
        }
    }
    0
}

/// Shows the "v%s %s.%s %d" error box for match_man.cpp at a line and exits the game, as the original.
///
/// [pt] A caixa de erro "v%s %s.%s %d" de match_man.cpp na linha `linha`, e o jogo para (como a original).
pub(crate) unsafe fn match_man_fatal_error(linha: u32) -> ! {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa15da4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    fechar1();
    fechar2();
    sair(-1)
}


/// frees the global list, the day's vectors and, for each match, the engine (0x6b97a0), the summary and the +0x5f
/// object
/// Original: `end_day` (0x6b4190, match_day.cpp, thiscall).
///
/// [pt] { 0x6b4190 } (match_day.cpp): o fim do dia de jogos. Libera a lista global [0xb67dc8], os dois
/// vetores do dia (+1, +0xd) e, para cada partida (+0x19, 0x69 bytes cada, +0x21 delas), o motor
/// (0x6b97a0 e liberado; a partida sai de [0xde2f70] por 0x90d6b0), o resumo (+0x5b) e o objeto de
/// +0x5f (0x453bc0); por fim o vetor das partidas.
pub(crate) unsafe extern "thiscall" fn end_day(dia: usize) {
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let liberar_vetor: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_VIRTUAL_STAFF_RELEASE_FIXTURE);
    let destruir_5f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_TEAM_PAIR_DESTRUCTOR);
    let destroy_engine: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_DESTRUCTOR_CM3_MATCH_ENGINE);
    let d = |o: usize| (dia + o) as *mut usize;
    let lista = *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *const usize);
    if lista != 0 {
        destruir_lista(lista);
        liberar(lista);
        *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *mut usize) = 0;
    }
    for o in [1usize, 0xd] {
        if *d(o) != 0 {
            liberar_vetor(*d(o));
            *d(o) = 0;
            *d(o + 4) = 0;
            *d(o + 8) = 0;
        }
    }
    if *d(0x19) == 0 {
        return;
    }
    let mut k = 0i32;
    while k < *d(0x21) as i32 {
        let p = *d(0x19) + k as usize * 0x69;
        let q = |o: usize| (p + o) as *mut usize;
        let motor = *q(0x57);
        if motor != 0 {
            destroy_engine(motor);
            liberar(motor);
            *q(0x57) = 0;
            tirar(*(G_VIRTUAL_STAFF as *const usize), p + 4);
        }
        if *q(0x5b) != 0 {
            liberar(*q(0x5b));
            *q(0x5b) = 0;
        }
        let x = *q(0x5f);
        if x != 0 {
            destruir_5f(x);
            liberar(x);
            *q(0x5f) = 0;
        }
        k += 1;
    }
    liberar_vetor(*d(0x19));
    *d(0x19) = 0;
    *d(0x1d) = 0;
    *d(0x21) = 0;
}

/// clears each of the day's matches' fields and picks the human's (or the first marked) for the screen (0x717cc0,
/// 0x7fc7e0)
/// Original: `pick_day_match` (0x6b42a0, match_day.cpp, thiscall).
///
/// [pt] { 0x6b42a0 } (match_day.cpp): com partidas no dia (+0x2b), limpa os 0x40 bytes de cada uma
/// (+0xd, 0x54 bytes, +0x15 delas) a partir de +0x14 e escolhe a do humano (+0x10; senao a primeira
/// com +0xc) para a tela (0x717cc0 e 0x7fc7e0 com [0xb67a48] ligado). Devolve 1 se havia uma do
/// humano.
pub(crate) unsafe extern "thiscall" fn pick_day_match(dia: usize) -> u32 {
    if *((dia + 0x2b) as *const usize) == 0 {
        return 0;
    }
    let mut achou = 0u32;
    let mut escolhida = 0usize;
    let n = *((dia + 0x15) as *const i32);
    let mut k = 0i32;
    while k < n {
        let e = *((dia + 0xd) as *const usize) + k as usize * 0x54;
        core::ptr::write_bytes((e + 0x14) as *mut u8, 0, 0x40);
        let e = *((dia + 0xd) as *const usize) + k as usize * 0x54;
        if *((e + 0x10) as *const usize) != 0 {
            if escolhida == 0 || *((escolhida + 0x10) as *const usize) == 0 {
                escolhida = e;
            }
            achou = 1;
        } else if *((e + 0xc) as *const usize) != 0 && escolhida == 0 {
            escolhida = e;
        }
        k += 1;
    }
    let mostrar: unsafe extern "C" fn(u32, usize) = core::mem::transmute(FN_OPEN_DAY_FIXTURES_SCREEN);
    let atualizar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_MONITOR);
    *(G_MATCH_DAY_SCREEN_SHOWING_FLAG as *mut u32) = 1;
    mostrar(0, escolhida);
    atualizar(G_SCREEN_MANAGER);
    *(G_MATCH_DAY_SCREEN_SHOWING_FLAG as *mut u32) = 0;
    achou
}

// ponytail: a escalacao de cada lado chama a automatica (0x6dbc50, que aloca a lista e le a pilha) e o
// dia de jogos libera motores e listas: sem comparacao propria, validados pela carreira (Rust x
// original), como o nascimento e o fim do motor. As rotinas que chamam seguem comparadas.
/// Comparer for `end_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_end_day(dia: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_END_DAY)(dia)
}
/// Comparer for `pick_day_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pick_day_match(dia: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_PICK_DAY_MATCH)(dia)
}

/// starts the day's next match after the current one (+0x2f): skips the decided ones, those of a human club and those
/// already started; creates the engine (0x6b7e50) and prepares it (0x6b7f20); in the 1st round with none, moves to the
/// 2nd. Returns 1 if it started one
/// Original: `start_next_match` (0x6b5a70, match_day.cpp, thiscall).
///
/// [pt] { 0x6b5a70 } (thiscall, match_day.cpp): comeca a proxima partida do dia (+0x19, 0x69 bytes cada,
/// +0x21 delas) depois da atual (+0x2f; -2 = acabou). A atual ainda com motor (+0x57) e erro 0x68d.
/// Pula as ja decididas (+0x47 -2), as sem jogo marcado na 2a volta (+0x33 1 e +0x63.+0x10 zero), as
/// de clube humano (0x600a50 com +0x44/+0x45) e as ja comecadas (+0x47 != -1, +0x57, +0x53). Na
/// escolhida: tira da fila [0xde2f70] (0x90d2f0 com -3), cria o motor (0xf63b bytes, 0x6b7e50; sem
/// ele erro 0x6b2 e segue) e o prepara (0x6b7f20 com a rapida de 0x6b66a0); +0x2f = ela, devolve 1.
/// Sem nenhuma: na 1a volta (+0x33 1) passa a 2 e recomeca do inicio; senao +0x2f = -2 e devolve 0.
pub(crate) unsafe extern "thiscall" fn start_next_match(dia: usize) -> u32 {
    let ler = |o: usize| *((dia + o) as *const i32);
    let por = |o: usize, v: i32| *((dia + o) as *mut i32) = v;
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let tirar: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_FILL_FIXTURE_SQUADS);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_CM3_MATCH_ENGINE);
    let rapida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_CAN_QUICK_SIMULATE);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_INIT_MATCH);
    let atual = ler(0x2f);
    if atual == -2 {
        return 0;
    }
    if atual > -1 && u(u(dia, 0x19).wrapping_add((atual as isize * 0x69) as usize), 0x57) != 0 {
        show_error_at(0xa15708, 0x68d);
        return 0;
    }
    loop {
        let mut k = ler(0x2f) + 1;
        while k < ler(0x21) {
            let p = u(dia, 0x19).wrapping_add((k as isize * 0x69) as usize);
            if byte(p, 0x47) != -2
                && (byte(dia, 0x33) != 1 || u(u(p, 0x63), 0x10) != 0)
                && humano(u(p, 0x20), 0, *((p + 0x44) as *const u8) as u32, 0, 0) == 0
                && humano(u(p, 0x24), 0, *((p + 0x45) as *const u8) as u32, 0, 0) == 0
                && byte(p, 0x47) == -1
                && u(p, 0x57) == 0
                && u(p, 0x53) == 0
            {
                tirar(u(G_VIRTUAL_STAFF, 0), p + 4, -3, 0);
                let mem = novo(0xf63b);
                let motor = if mem != 0 { construir(mem) } else { 0 };
                *((p + 0x57) as *mut usize) = motor;
                if motor != 0 {
                    let r = rapida(p + 4);
                    preparar(motor, p + 4, r, 0, 0);
                    por(0x2f, k);
                    crate::comparacao::ramo("dia partida iniciada");
                    return 1;
                }
                show_error_at(0xa15708, 0x6b2);
            }
            k += 1;
        }
        if byte(dia, 0x33) != 1 {
            por(0x2f, -2);
            crate::comparacao::ramo("dia sem mais partidas");
            return 0;
        }
        *((dia + 0x33) as *mut u8) = 2;
        crate::comparacao::ramo("dia segunda volta");
        por(0x2f, -1);
    }
}

/// one engine step of the current match (1 in 3 or 6 calls, by the modes in [0xae3138]) or to the end; when it
/// finishes, creates the summary, records it (0x7293f0), frees the engine and takes it off the queue. Returns 1 if it
/// finished
/// Original: `step_day_match` (0x6b5ce0, match_day.cpp, thiscall, ret 4).
///
/// [pt] { 0x6b5ce0 } (thiscall, ret 4, match_day.cpp): faz andar a partida atual do dia (+0x2f). Sem
/// `ate_o_fim`, so 1 de cada 3 (0x40000 em [0xae3138]+4) ou 6 (0x80000) chamadas, pelo contador
/// [0xb67a54] (0..100), e um passo do motor (0x6b98c0 com 1); com `ate_o_fim`, passos (-1) ate o
/// fim. O estado vai em +0x67; ao terminar (2 ou 8) conta em +0x25, cria o resumo (+0x5f, 0xd7a
/// bytes com 2 objetos de 0x625 em +0x11f; sem ele erro 0x713), registra (0x7293f0), libera o motor
/// e tira a partida da fila [0xde2f70] (0x90d6b0). Devolve 1 se a partida terminou.
pub(crate) unsafe extern "thiscall" fn step_day_match(dia: usize, ate_o_fim: u32) -> u32 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_RUN_MATCH_STEPS);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MATCH_STATS_RECORD_MATCH);
    let destroy_engine: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_DESTRUCTOR_CM3_MATCH_ENGINE);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_VIRTUAL_STAFF_RELEASE_FIXTURE);
    let contador = G_STEP_BACKGROUND_MATCH_COUNTER as *mut i32;
    *contador += 1;
    if *contador > 100 {
        *contador = 0;
    }
    if ate_o_fim == 0 {
        let modo = u(u(G_CONFIG, 0), 4);
        if modo & 0x40000 != 0 && *contador % 3 > 0 {
            return 0;
        }
        if modo & 0x80000 != 0 && *contador % 6 > 0 {
            return 0;
        }
    }
    let atual = *((dia + 0x2f) as *const i32);
    if atual < 0 {
        return 0;
    }
    let p = u(dia, 0x19).wrapping_add((atual as isize * 0x69) as usize);
    if u(p, 0x57) == 0 {
        return 0;
    }
    let estado = (p + 0x67) as *mut u8;
    crate::comparacao::ramo("andar com motor");
    let fim = |e: u8| e == 2 || e == 8;
    let step_towards = |n: i32| {
        let e = laco(u(p, 0x57), n) as u8;
        *estado = e;
        if fim(e) {
            *((dia + 0x25) as *mut i32) += 1;
        }
    };
    if ate_o_fim != 0 {
        crate::comparacao::ramo("andar ate o fim");
        while !fim(*estado) {
            step_towards(-1);
        }
    } else {
        step_towards(1);
        crate::comparacao::ramo("andar um step_towards");
    }
    if !fim(*estado) {
        return 0;
    }
    crate::comparacao::ramo("andar partida terminada");
    let mem = novo(0xd7a);
    if mem != 0 {
        vetor(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
    }
    *((p + 0x5f) as *mut usize) = mem;
    if mem == 0 {
        show_error_at(0xa15708, 0x713);
        return 0;
    }
    registrar(u(G_MATCH_STATS, 0), u(p, 0x57), mem);
    let motor = u(p, 0x57);
    if motor != 0 {
        destroy_engine(motor);
        liberar(motor);
    }
    *((p + 0x57) as *mut usize) = 0;
    tirar(u(G_VIRTUAL_STAFF, 0), p + 4);
    1
}

/// copies the day's match into the match, records the result in the competition (method 1, or 0x924050 and
/// 0x5ab510/0x5aa680 for the away game) and runs the post-match: players, events, contracts, suspensions, states, the
/// history and each side's club (0x6b79a0)
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
pub(crate) unsafe extern "thiscall" fn close_day_match(_dia: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    if p == 0 {
        show_error_at(0xa15708, 0x73f);
        return 0;
    }
    let jogo = u(p, 0);
    core::ptr::copy((p + 4) as *const u8, jogo as *mut u8, 0x4f);
    let resumo = u(p, 0x5f);
    let mut rodada = [0u8; 4];
    let competicao = |id: i32| *((u(G_COMP_OBJECT_TABLE, 0) as isize + 4 * id as isize) as *const usize);
    if u(jogo, 0x14) != 0 {
        crate::comparacao::ramo("fechar com competicao");
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
        for k in 0..2 {
            let c = u(jogo, 0x20);
            if c == 0 || humano(c, 0, 1, 0, 0) == 0 || u(c, 0x57) == 0 {
                continue;
            }
            let id = *(u(c, 0x57) as *const i32);
            if id != *(jogo as *const i32) {
                continue;
            }
            let comp = competicao(*(u(jogo, 0x14) as *const i32));
            if comp == 0 || u(comp, 8) == 0 {
                continue;
            }
            let r = busca(u(competicao(id), 8), c);
            if r != 0 && *((r + 0x2d) as *const i32) > 1 {
                rodada[k] = *((r + 0x2d) as *const u8);
                crate::comparacao::ramo("fechar rodada do humano");
            }
        }
        if byte(p, 0x47) == -3 {
            crate::comparacao::ramo("fechar jogo de fora");
            let mut copia = [0u8; 0x100];
            core::ptr::copy((p + 4) as *const u8, copia.as_mut_ptr(), 0x4f);
            for o in 0x43..=0x4a {
                copia[o] = 0xff;
            }
            let w = core::ptr::read_unaligned(copia.as_ptr().add(0x4d) as *const u16) & 0xcfff;
            core::ptr::write_unaligned(copia.as_mut_ptr().add(0x4d) as *mut u16, w);
            let fora: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_WEATHER_POST_ABANDONED_NEWS);
            let gravar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_NEW_FIXTURE_DATE);
            let juntar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
            fora(u(G_WEATHER, 0), p + 4, ((*((jogo + 0x4d) as *const u16) >> 12) & 3) as u32);
            gravar(u(G_FIXTURE_MANAGER, 0), copia.as_mut_ptr());
            juntar(u(G_FIXTURE_MANAGER, 0), copia.as_mut_ptr(), 1);
        } else {
            let c = competicao(*(jogo as *const i32));
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(u(u(c, 0), 4));
            metodo(c, jogo, resumo);
            crate::comparacao::ramo("fechar metodo da competicao");
        }
    }
    *(G_GAME_UNSAVED_CHANGES_FLAG as *mut u32) = 1;
    let jogadores: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_MATCH_DAY_APPLY_POST_MATCH_PLAYER_CHANGES);
    let eventos: unsafe extern "C" fn(usize) = core::mem::transmute(FN_MATCH_DAY_PROCESS_PLAYER_INCIDENTS);
    jogadores(jogo, resumo);
    eventos(jogo);
    let dois = |this: usize, f: usize| {
        let g: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(f);
        g(this, jogo, resumo);
    };
    dois(u(G_CONTRACT_MANAGER, 0), 0x4ca6d0);
    dois(u(G_DISCIPLINE, 0), 0x55ca00);
    if byte(p, 0x47) != -3 {
        dois(u(G_NATIONAL_TEAMS, 0), 0x76b600);
    }
    dois(u(G_INJURY, 0), 0x62c080);
    if byte(p, 0x47) == -3 {
        return 1;
    }
    if *(jogo as *const u32) != *(KEY_CLUB_COMP_RESERVE as *const u32) {
        let h = u(G_RESERVE_COMP, 0);
        dois(h, u(u(h, 0), 4));
        crate::comparacao::ramo("fechar historia");
    }
    for (this, f) in [(G_PLAYER_STATS, 0x7c0d70), (G_TRANSFER_MANAGER, 0x8c39f0), (G_CLUB_RECORDS, 0x444930), (G_STAFF_RECORDS, 0x868830), (G_AWARD_MANAGER, 0x4134c0), (G_MANAGER_MANAGER, 0x6951e0), (G_FINANCE_TABLE, 0x59a5d0), (G_SCOUT_MANAGER, 0x7f8c00)] {
        dois(u(this, 0), f);
    }
    let clube: unsafe extern "C" fn(usize, usize, usize, u32, usize) = core::mem::transmute(FN_MATCH_DAY_POST_PLAYER_REACTION_NEWS);
    // ponytail: a rodada vai como dword com os altos zerados; a original passa sobra da pilha neles (lidos como byte)
    clube(u(jogo, 0x1c), u(jogo, 0x20), jogo, rodada[0] as u32, resumo);
    clube(u(jogo, 0x20), u(jogo, 0x1c), jogo, rodada[1] as u32, resumo);
    1
}

// ponytail: o dia de jogos cria e libera motores e resumos e chama dezenas de rotinas ainda
// originais; sem comparacao propria, validadas pela carreira (Rust x original), como as da 149a.
/// Comparer for `start_next_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_start_next_match(dia: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_START_NEXT_MATCH)(dia)
}
/// Comparer for `step_day_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_step_day_match(dia: usize, ate_o_fim: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_STEP_DAY_MATCH)(dia, ate_o_fim)
}
/// Comparer for `close_day_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_close_day_match(dia: usize, p: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLOSE_DAY_MATCH)(dia, p)
}

/// plays the day. On a network, ties the humans to their matches (0x11d-byte scoreboard, 0x765480, 0x5fcff0/0x5fd100)
/// and plays them to the end; then each match without an engine: with a human on a network, steps the current one
/// (0x6b5ce0) and restarts; otherwise creates the engine, plays to the end, creates the summary, records it and writes
/// the score with the percentage on the bar (0x66f4e0, 0x802f80). Finally closes each match (0x6b5f00), redoes the
/// humans' stored tactics and frees the summaries
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
pub(crate) unsafe extern "thiscall" fn play_the_day(dia: usize) -> u32 {
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let por = |p: usize, o: usize, v: usize| core::ptr::write_unaligned((p + o) as *mut usize, v);
    let ler = |o: usize| *((dia + o) as *const i32);
    let rede = || *(G_MATCH_ENGINE_ENABLED as *const u8) != 0;
    let gerente = || u(G_HUMAN_MANAGER, 0);
    let partida = |k: i32| u(dia, 0x19).wrapping_add((k as isize * 0x69) as usize);
    let humano_a = |i: i32| u(G_STAFF_TABLE, 0).wrapping_add(((*(G_STAFF_COUNT as *const i32) - i - 1) as isize * 0x6e) as usize);
    let humano_b = |i: i32| u(G_STAFF_TABLE, 0).wrapping_add(((*(G_STAFF_COUNT as *const i32) + i - 0x10) as isize * 0x6e) as usize);
    let iniciar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_START_NEXT_BACKGROUND_MATCH);
    let andar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_DAY_STEP_BACKGROUND_MATCH);
    let fechar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_APPLY_MATCH_RESULT);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let usa: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let tirar_da_fila: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_FILL_FIXTURE_SQUADS);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_VIRTUAL_STAFF_RELEASE_FIXTURE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let avisar_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FN_OPEN_MANAGER_CLUB_SCREEN);
    let nivel: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_HUMAN_MANAGER_GET_COMMENTARY_SPEED);
    // 0x5fe290 (ret 8): a partida e o humano (a original empilha o humano antes do teste)
    let em_rede: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_WATCHING_FIXTURE);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let f5fd2f0: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_ON_HOLIDAY);
    let f727f00: unsafe extern "C" fn(usize) = core::mem::transmute(FN_MATCH_SCREENS_OPEN_WAITING_SCREEN_FOR_SLOT);
    let repassar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMMENTARY_SPEED);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_SCREENMANAGER_FIND_MANAGER_CHANNEL);
    let fora_do_dia: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_OPEN_LATEST_SCORES_SCREEN);
    let redesenhar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SCREENMANAGER_MONITOR);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_RUN_MATCH_STEPS);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MATCH_STATS_RECORD_MATCH);
    let destroy_engine: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_DESTRUCTOR_CM3_MATCH_ENGINE);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_CM3_MATCH_ENGINE);
    let rapida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_CAN_QUICK_SIMULATE);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_INIT_MATCH);
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_GET_CURSOR_MODE);
    let ajustar_relogio: unsafe extern "C" fn(u32) = core::mem::transmute(FN_SET_CURSOR_MODE);
    let f68d520: unsafe extern "C" fn() = core::mem::transmute(FN_PUMP_MESSAGES);
    let f788010: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_NEWS_SCREENS_OPEN_NEWS_SCREEN);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mostrar: unsafe extern "thiscall" fn(usize, usize, i32, u32, i32) = core::mem::transmute(FN_SCREENMANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let preparar_linhas: unsafe extern "thiscall" fn(usize, i32, usize, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SELECT_TACTIC_SLOT);
    let guardadas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_ACTIVE_TACTIC);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FORMATION_CLEAR_MARKING_TARGETS);
    let data_texto: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_DATE_GET_LONG_DATE_STRING);

    let terminou = |e: u8| e == 2 || e == 8;
    // joga ate o fim (passos de -1), contando as terminadas em +0x25
    let ate_o_fim = |p: usize| {
        while !terminou(*((p + 0x67) as *const u8)) {
            let e = laco(u(p, 0x57), -1) as u8;
            *((p + 0x67) as *mut u8) = e;
            if terminou(e) {
                *((dia + 0x25) as *mut i32) += 1;
            }
        }
    };
    // libera o motor e tira a partida da fila
    let sem_motor = |p: usize| {
        let m = u(p, 0x57);
        if m != 0 {
            destroy_engine(m);
            liberar(m);
        }
        por(p, 0x57, 0);
        tirar(u(G_VIRTUAL_STAFF, 0), p + 4);
    };
    let novo_motor = |p: usize| {
        let mem = novo(0xf63b);
        let m = if mem != 0 { construir(mem) } else { 0 };
        por(p, 0x57, m);
        m
    };
    let novo_resumo = |p: usize| {
        let mem = novo(0xd7a);
        if mem != 0 {
            vetor(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        por(p, 0x5f, mem);
        mem
    };
    let b = |p: usize, o: usize| *((p + o) as *const u8);

    'reinicio: loop {
        iniciar(dia);
        let mut vistos = [0u32; 16];
        let mut algum = false;
        if rede() {
            crate::comparacao::ramo("rede");
            let mut k = 0i32;
            while k < ler(0x21) {
                let p = partida(k);
                let e = byte(p, 0x47);
                if e != -2 && e <= -1 && u(p, 0x53) != 0 {
                    tirar_da_fila(u(G_VIRTUAL_STAFF, 0), p + 4, -1, 0);
                    let r = novo(0x11d);
                    por(p, 0x5b, r);
                    if r == 0 {
                        show_error_at(0xa15708, 0x250);
                    } else {
                        *(r as *mut u8) = 0;
                        for o in [5usize, 0x45, 0x85, 0xc5] {
                            core::ptr::write_bytes((u(p, 0x5b) + o) as *mut u8, 0, 0x40);
                        }
                        let (mut s13, mut s12) = (0u8, 0u8);
                        for o in 0x105..=0x108 {
                            *((u(p, 0x5b) + o) as *mut u8) = 0;
                        }
                        for (o, v) in [(0x109usize, 0usize), (0x10d, 0), (0x111, 1), (0x115, 0), (0x119, 0)] {
                            por(u(p, 0x5b), o, v);
                        }
                        // o maior nivel (0x5fcff0) entre o guardado e o deste humano
                        let maior = |s: u8, h: usize| if s as i16 > nivel(gerente(), h) as i16 { s } else { nivel(gerente(), h) as u8 };
                        for i in 0..16 {
                            let h = humano_a(i);
                            if b(h, 0x5f) == 0 {
                                continue;
                            }
                            let lado = if humano(u(p, 0x20), 0, b(p, 0x44) as u32, h, 0) != 0 {
                                Some((0x20, 0x44))
                            } else if humano(u(p, 0x24), 0, b(p, 0x45) as u32, h, 0) != 0 {
                                Some((0x24, 0x45))
                            } else {
                                None
                            };
                            match lado {
                                Some((oc, ob)) => {
                                    por(ficha(h), 0xe2, p);
                                    por(ficha(h), 0xe6, u(p, oc));
                                    avisar_humano(h, (b(p, ob) == 0) as u32, 0, 0, 0);
                                    s13 = maior(s13, h);
                                }
                                None => {
                                    if em_rede(gerente(), p + 4, h) == 0 {
                                        continue;
                                    }
                                    if t9a01(gerente(), h) != 0 {
                                        f5fd2f0(gerente(), 0, h);
                                    }
                                    por(ficha(h), 0xe2, p);
                                    por(ficha(h), 0xe6, 0);
                                    f727f00(h);
                                    s12 = maior(s12, h);
                                }
                            }
                            let x = indice(G_SCREEN_MANAGER, h) as i16 as isize;
                            let placar = u(p, 0x5b);
                            if core::ptr::read_unaligned((placar as isize + 5 + 4 * x) as *const u32) == 0 {
                                let x = indice(G_SCREEN_MANAGER, h) as i16 as isize;
                                core::ptr::write_unaligned((u(p, 0x5b) as isize + 5 + 4 * x) as *mut u32, 1);
                                *((u(p, 0x5b) + 0x105) as *mut u8) += 1;
                            }
                            // ponytail: o indice e de 0 a 15 (os humanos); fora disso a original escreveria na pilha
                            let x = indice(G_SCREEN_MANAGER, h) as i16;
                            if (0..16).contains(&x) {
                                vistos[x as usize] = 1;
                            }
                        }
                        for i in 0..16 {
                            let h = humano_b(i);
                            if b(h, 0x5f) != 0 && u(ficha(h), 0xe2) == p {
                                repassar(gerente(), if s13 > 0 { s13 } else { s12 } as u32, h);
                            }
                        }
                        if u(dia, 0x34) == 0 {
                            por(dia, 0x34, p);
                        }
                        algum = true;
                        crate::comparacao::ramo("rede partida com humanos");
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
                let h = humano_b(i);
                if indice(G_SCREEN_MANAGER, h) != 0xffff {
                    let x = indice(G_SCREEN_MANAGER, h) as i16;
                    if (0..16).contains(&x) && vistos[x as usize] == 0 {
                        fora_do_dia(h, u(u(dia, 0x34), 0x63));
                    }
                }
            }
            redesenhar(G_SCREEN_MANAGER);
            for i in 0..16 {
                let h = humano_b(i);
                por(ficha(h), 0xe2, 0);
                por(ficha(h), 0xe6, 0);
            }
            let mut k = 0i32;
            while k < ler(0x21) {
                let p = partida(k);
                if u(p, 0x53) != 0 && u(p, 0x57) != 0 {
                    ate_o_fim(p);
                    registrar(u(G_MATCH_STATS, 0), u(p, 0x57), u(p, 0x5f));
                    sem_motor(p);
                    liberar(u(p, 0x5b));
                    por(p, 0x5b, 0);
                }
                k += 1;
            }
        }

        // as partidas sem humano, uma a uma
        *(G_MATCH_DAY_PLAYING_FLAG as *mut u32) = 1;
        let mut placar_escrito = false;
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        let hora = relogio();
        let mut k = 0i32;
        while k < ler(0x21) {
            if *(G_INTERACTIVE_MODE_FLAG as *const u32) != 0 {
                ajustar_relogio(2);
                f68d520();
                if *(G_MOUSE_LEFT_BUTTON_DOWN as *const u32) != 0 {
                    let mut redesenha = false;
                    for i in 0..16 {
                        let h = humano_a(i);
                        if b(h, 0x5f) != 0 {
                            if u(h, 0x39) == 0 && u(h, 0x24) == 0 {
                                f788010(h, 0);
                            } else {
                                avisar_humano(h, 0, 0, 0, 0);
                            }
                            redesenha = true;
                        }
                    }
                    if redesenha {
                        redesenhar(G_SCREEN_MANAGER);
                    }
                    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                }
            }
            let p = partida(k);
            if byte(p, 0x47) == -1 && u(p, 0x57) == 0 {
                let com_humano = humano(u(p, 0x20), 0, b(p, 0x44) as u32, 0, 0) != 0 || humano(u(p, 0x24), 0, b(p, 0x45) as u32, 0, 0) != 0;
                if com_humano && rede() {
                    andar(dia, 1);
                    *(G_MATCH_DAY_PLAYING_FLAG as *mut u32) = 0;
                    crate::comparacao::ramo("humano em rede recomeca");
                    continue 'reinicio;
                }
                tirar_da_fila(u(G_VIRTUAL_STAFF, 0), p + 4, -2, 0);
                let m = novo_motor(p);
                if m == 0 {
                    show_error_at(0xa15708, 0x39d);
                } else {
                    let r = rapida(p + 4);
                    crate::comparacao::ramo("partida sem humano");
                    preparar(m, p + 4, r, 0, 0);
                    let rapido = *(u(p, 0x57) as *const i8) != 0;
                    ate_o_fim(p);
                    let resumo = novo_resumo(p);
                    if resumo == 0 {
                        show_error_at(0xa15708, 0x3b5);
                    } else {
                        registrar(u(G_MATCH_STATS, 0), u(p, 0x57), resumo);
                        sem_motor(p);
                        if !rapido {
                            crate::comparacao::ramo("placar na barra");
                            // o placar com a porcentagem do dia na barra
                            let n = ler(0x21);
                            let pct = if n != 0 { ler(0x25).wrapping_mul(100) / n } else { 0 } as i8 as i32;
                            let (c, casa, fora) = (u(p, 0x18), u(p, 0x20), u(p, 0x24));
                            let (a52, b52) = (byte(casa, 0x52) as i32, byte(fora, 0x52) as i32);
                            let (g1, g2) = if byte(p, 0x4b) > -1 { (byte(p, 0x4b), byte(p, 0x4c)) } else { (byte(p, 0x47), byte(p, 0x48)) };
                            if c != 0 {
                                crate::comparacao::ramo("placar com competicao");
                                let c37 = byte(c, 0x37) as i32;
                                texto(0xde1f64, 0xa157b4, c37, c37, a52, a52, b52, b52, c + 4, casa + 0x38, g1 as i32, fora + 0x38, g2 as i32, pct, 0xa15850usize);
                            } else {
                                texto(0xde1f64, 0xa15734, a52, a52, b52, b52, casa + 0x38, g1 as i32, fora + 0x38, g2 as i32, pct, 0xa15850usize);
                            }
                            mostrar(G_SCREEN_MANAGER, 0xde1f64, -1, 0, -1);
                            placar_escrito = true;
                        }
                    }
                }
            }
            k += 1;
        }

        // o fechamento de cada partida
        ajustar_relogio(hora);
        andar(dia, 1);
        *(G_MATCH_DAY_PLAYING_FLAG as *mut u32) = 0;
        let mut k = 0i32;
        while k < ler(0x21) {
            let p = partida(k);
            if byte(p, 0x47) != -2 {
                if u(p, 0x5f) == 0 {
                    show_error_at(0xa15708, 0x3fb);
                } else {
                    fechar(dia, p);
                    crate::comparacao::ramo("match_wrap_up");
                    for (oc, ob) in [(0x20usize, 0x44usize), (0x24, 0x45)] {
                        if humano(u(p, oc), 0, 1, 0, 0) != 0 && usa(u(p, oc)) != 0 {
                            let x = usa(u(p, oc));
                            crate::comparacao::ramo("linhas do humano refeitas");
                            preparar_linhas(gerente(), (b(p, ob) == 0) as i32, u(p, oc), x);
                            limpar(guardadas(gerente(), u(u(p, oc), 0xcf)));
                        }
                    }
                    let s = u(p, 0x5f);
                    if s != 0 {
                        fim_do_vetor(s + 0x11f, 0x625, 2, 0x453c10);
                        liberar(s);
                    }
                    por(p, 0x5f, 0);
                }
            }
            k += 1;
        }
        if placar_escrito {
            crate::comparacao::ramo("data na barra");
            data_texto(G_CURRENT_DATE, 0xde1f64, 0x7d0, *(G_CURRENT_DAY_SLICE as *const u8) as u32);
            mostrar(G_SCREEN_MANAGER, 0xde1f64, -1, 0, -1);
        }
        return 1;
    }
}

// ponytail: o laco do dia cria e libera motores e resumos e mexe na tela; sem comparacao propria,
// validado pela carreira (Rust x original), como as da 151a.
/// Comparer for `play_the_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_play_the_day(dia: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_PLAY_THE_DAY)(dia)
}

/// sets up the day (date, mode 0..2): zeroes the vectors, gathers the pending matches of each list (0x5ae6a0), records
/// the already decided ones (0x923630/0x923cc0), groups by list and by competition (qsort 0x6b6510), points each match
/// to its competition (+0x63), creates the global list [0xb67dc8] and works out the day's interest (0x6b5040). Returns
/// the day
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
pub(crate) unsafe extern "thiscall" fn set_up_the_day(dia: usize, data: usize, modo: u32) -> usize {
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ler = |o: usize| core::ptr::read_unaligned((dia + o) as *const i32);
    let por = |p: usize, o: usize, v: u32| core::ptr::write_unaligned((p + o) as *mut u32, v);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let preparar_historia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_RESERVECOMP_CANCEL_UNPLAYABLE_RESERVE_FIXTURES);
    let lista_do_dia: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let decidida: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_CHECK_POSTPONEMENT);
    let marcar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_WEATHER_POST_POSTPONED_NEWS);
    let gravar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_NEW_FIXTURE_DATE);
    let juntar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
    let avisar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_OFFICIALSMANAGER_CHOOSE_MATCH_OFFICIAL);
    let grupo_novo: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_KEY_NATION_ENTRY_FROM_ACTIVE_INDEX);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let interesse: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_REFRESH_HUMAN_INTEREST_FLAGS);
    let modo8 = modo as u8;
    *(dia as *mut u8) = modo8;
    for o in (1..=0x25).step_by(4) {
        por(dia, o, 0);
    }
    *((dia + 0x29) as *mut u8) = 0;
    *((dia + 0x2a) as *mut u8) = 0;
    por(dia, 0x2b, 0);
    por(dia, 0x2f, u32::MAX);
    *((dia + 0x33) as *mut u8) = 1;
    por(dia, 0x34, 0);
    *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *mut usize) = 0;
    if data == 0 || modo8 > 2 {
        show_error_at(0xa15708, 0x93);
        return dia;
    }
    preparar_historia(u(G_RESERVE_COMP, 0));
    // cresce um vetor (ponteiro +o, capacidade +o+4, contagem +o+8) de 5 em 5; false sem memoria
    let crescer = |o: usize, tam: u32| -> bool {
        if ler(o + 8) == ler(o + 4) {
            let cap = ler(o + 4) + 5;
            por(dia, o + 4, cap as u32);
            let p = realocar(u(dia, o), (cap as u32).wrapping_mul(tam));
            por(dia, o, p as u32);
            if p == 0 {
                return false;
            }
        }
        true
    };
    let mut comp_atual = 0usize;
    let mut i = 0i32;
    while i < *(G_KEY_NATION_ACTIVE_COUNT as *const i8) as i32 {
        let lista = lista_do_dia(u(G_FIXTURE_MANAGER, 0), data, modo, i as u32);
        if lista != 0 && *(lista as *const i16) > 0 {
            por(lista, 0xa, u(lista, 2) as u32);
            let mut grupo = 0usize;
            let mut x = proximo(lista);
            while x != 0 {
                let e = byte(x, 0x43);
                if e <= -1 && e != -2 {
                    let r = decidida(u(G_WEATHER, 0), x);
                    if r & 1 != 0 {
                        crate::comparacao::ramo("montagem decidida antes");
                        let mut copia = [0u8; 0x100];
                        core::ptr::copy(x as *const u8, copia.as_mut_ptr(), 0x4f);
                        *((x + 0x43) as *mut u8) = 0xfe;
                        // ponytail: o 2o argumento vai so com o byte; a original passa sobra da pilha nos altos (lidos como byte)
                        marcar(u(G_WEATHER, 0), x, r as u32);
                        gravar(u(G_FIXTURE_MANAGER, 0), copia.as_mut_ptr());
                        juntar(u(G_FIXTURE_MANAGER, 0), copia.as_mut_ptr(), 1);
                    } else {
                        avisar(u(G_OFFICIALS_MANAGER, 0), x, x + 0x2e);
                        crate::comparacao::ramo("montagem avisada");
                    }
                    if grupo == 0 {
                        crate::comparacao::ramo("montagem grupo novo");
                        if !crescer(1, 24) {
                            show_error_at(0xa15708, 0xd6);
                            return dia;
                        }
                        grupo = u(dia, 1) + ler(9) as usize * 24;
                        por(grupo, 0, grupo_novo(i as u32));
                        for o in [4usize, 8, 0xc, 0x10] {
                            por(grupo, o, u32::MAX);
                        }
                        por(grupo, 0x14, 0);
                        por(dia, 9, (ler(9) + 1) as u32);
                    }
                    if !crescer(0x19, 0x69) {
                        show_error_at(0xa15708, 0xf4);
                        return dia;
                    }
                    let p = u(dia, 0x19) + ler(0x21) as usize * 0x69;
                    por(p, 0, x as u32);
                    core::ptr::copy(x as *const u8, (p + 4) as *mut u8, 0x4f);
                    for o in [0x53usize, 0x57, 0x5b, 0x5f, 0x63] {
                        por(p, o, 0);
                    }
                    *((p + 0x67) as *mut u8) = 0xff;
                    if u(grupo, 0xc) as i32 == -1 {
                        por(grupo, 0xc, ler(0x21) as u32);
                    }
                    por(grupo, 0x10, ler(0x21) as u32);
                    por(dia, 0x21, (ler(0x21) + 1) as u32);
                }
                x = proximo(lista);
            }
            if grupo != 0 {
                let (de, ate) = (u(grupo, 0xc) as i32, u(grupo, 0x10) as i32);
                ordenar(u(dia, 0x19) + de as usize * 0x69, (ate - de + 1) as u32, 0x69, 0x6b6510);
                let mut ultima = -1i32;
                let mut k = u(grupo, 0xc) as i32;
                while k <= u(grupo, 0x10) as i32 {
                    let p = u(dia, 0x19) + k as usize * 0x69;
                    let id = u(p, 4) as i32;
                    if id != ultima {
                        crate::comparacao::ramo("montagem competicao nova");
                        if !crescer(0xd, 0x54) {
                            show_error_at(0xa15708, 0x140);
                            return dia;
                        }
                        let c = u(dia, 0xd) + ler(0x15) as usize * 0x54;
                        comp_atual = c;
                        por(c, 0, id as u32);
                        por(c, 4, u32::MAX);
                        por(c, 8, u32::MAX);
                        por(c, 0xc, 0);
                        por(c, 0x10, 0);
                        core::ptr::write_bytes((c + 0x14) as *mut u8, 0, 0x40);
                        if u(grupo, 4) as i32 == -1 {
                            por(grupo, 4, ler(0x15) as u32);
                        }
                        por(grupo, 8, ler(0x15) as u32);
                        por(dia, 0x15, (ler(0x15) + 1) as u32);
                        ultima = u(p, 4) as i32;
                    }
                    let c = comp_atual;
                    if u(c, 4) as i32 == -1 {
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
    while i < ler(0x15) {
        let c = u(dia, 0xd) + i as usize * 0x54;
        let mut k = u(c, 4) as i32;
        while k <= u(c, 8) as i32 {
            por(u(dia, 0x19) + k as usize * 0x69, 0x63, c as u32);
            k += 1;
        }
        i += 1;
    }
    let mem = novo(0xe);
    *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *mut usize) = if mem != 0 { lista_nova(mem) } else { 0 };
    interesse(dia);
    dia
}



// ponytail: a montagem do dia aloca os vetores e chama rotinas ainda originais; o lado e os sons sao
// so leitura e tela. Sem comparacao propria, validadas pela carreira (Rust x original).
/// Comparer for `set_up_the_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_up_the_day(dia: usize, data: usize, modo: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_SET_UP_THE_DAY)(dia, data, modo)
}

/// Memory regions 0x6b5040 marks for a match day: day +0x29..+0x2e, each group +0x14, each competition +0xc..+0x13 and
/// each match +0x53.
///
/// [pt] As regioes que a 0x6b5040 marca no dia `d`: +0x29 a +0x2e do dia, +0x14 de cada grupo, +0xc a
/// +0x13 de cada competicao e +0x53 de cada partida.
pub(crate) unsafe fn match_day_marked_regions(d: usize) -> Vec<(usize, usize)> {
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let mut v = vec![(d + 0x29, 6)];
    for gi in 0..ler(d + 9).max(0) as usize {
        let grp = (ler(d + 1) as usize).wrapping_add(gi * 0x18);
        v.push((grp + 0x14, 4));
        for ci in ler(grp + 4)..=ler(grp + 8) {
            let comp = (ler(d + 0xd) as isize + ci as isize * 0x54) as usize;
            v.push((comp + 0xc, 8));
            for mi in ler(comp + 4)..=ler(comp + 8) {
                v.push(((ler(d + 0x19) as isize + mi as isize * 0x69) as usize + 0x53, 4));
            }
        }
    }
    v
}

/// Marks on match day `d` what interests the human managers: clears the counters (+0x29 groups, +0x2a competitions,
/// +0x2b any), then for each group and competition flags the competition (+0xc) when one of the 16 human managers'
/// clubs (+0x39 or +0x24) plays in it, and flags each fixture (+0x53) involving a human club or watched by a human,
/// counting the groups and competitions with such fixtures; other fixtures of human interest (0x6b61e0) only flag the
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
    let lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_HUMAN_MANAGER_CLEAR_INVALID_WATCHED_FIXTURES);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_INCLUDES_CLUB);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let f5fe420: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_FIXTURE_WATCHED);
    let f6b61e0: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_MATCH_DAY_IS_FIXTURE_OF_HUMAN_INTEREST);
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let pr = |a: usize, v: i32| core::ptr::write_unaligned(a as *mut i32, v);
    lista(g(G_HUMAN_MANAGER));
    *((d + 0x29) as *mut u8) = 0;
    *((d + 0x2a) as *mut u8) = 0;
    pr(d + 0x2b, 0);
    let mut gi = 0i32;
    while gi < ler(d + 9) {
        let grp = (ler(d + 1) as usize).wrapping_add(gi as usize * 0x18);
        pr(grp + 0x14, 0);
        let mut ci = ler(grp + 4);
        while ci <= ler(grp + 8) {
            let comp = (ler(d + 0xd) as isize + ci as isize * 0x54) as usize;
            pr(comp + 0xc, 0);
            pr(comp + 0x10, 0);
            if ler(comp) > -1 {
                let objeto = g(g(G_COMP_OBJECT_TABLE) + ler(comp) as usize * 4);
                if (0..16).filter(|k| g((g(G_STAFF_TABLE) as isize + (ler(0xae2364) + k - 16) as isize * 0x6e) as usize + 0x39) != 0).count() >= 2 {
                    crate::comparacao::ramo("interesse dois humanos com clube");
                }
                for k in 0..16 {
                    let pe = (g(G_STAFF_TABLE) as isize + (ler(0xae2364) + k - 16) as isize * 0x6e) as usize;
                    let clube = g(pe + 0x39);
                    let outro = g(pe + 0x24);
                    if (clube != 0 && na_competicao(objeto, clube) != 0) || (outro != 0 && na_competicao(objeto, outro) != 0) {
                        crate::comparacao::ramo("interesse humano na competicao");
                        pr(comp + 0xc, 1);
                        pr(d + 0x2b, 1);
                    }
                }
            }
            let mut mi = ler(comp + 4);
            while mi <= ler(comp + 8) {
                let m = (ler(d + 0x19) as isize + mi as isize * 0x69) as usize;
                pr(m + 0x53, 0);
                let marcada = humano(g(m + 0x20), 0, *((m + 0x44) as *const u8) as u32, 0, 0) != 0
                    || humano(g(m + 0x24), 0, *((m + 0x45) as *const u8) as u32, 0, 0) != 0
                    || f5fe420(g(G_HUMAN_MANAGER), m + 4) != 0;
                if marcada {
                    crate::comparacao::ramo("interesse partida marcada");
                    pr(m + 0x53, 1);
                    if ler(grp + 0x14) == 0 {
                        *((d + 0x29) as *mut u8) = (*((d + 0x29) as *const u8)).wrapping_add(1);
                    }
                    if ler(comp + 0x10) == 0 {
                        *((d + 0x2a) as *mut u8) = (*((d + 0x2a) as *const u8)).wrapping_add(1);
                    }
                    pr(grp + 0x14, 1);
                    pr(comp + 0x10, 1);
                    pr(comp + 0xc, 1);
                    pr(d + 0x2b, 1);
                } else if f6b61e0(m + 4) != 0 {
                    crate::comparacao::ramo("interesse pela 6b61e0");
                    pr(comp + 0xc, 1);
                    pr(d + 0x2b, 1);
                }
                mi += 1;
            }
            ci += 1;
        }
        gi += 1;
    }
    1
}

/// Comparer for `match_day_interest` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_day_interest(d: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_MATCH_DAY_INTEREST);
    if is_recording_top_level() {
        return o(d);
    }
    // ponytail: a 0x5fe8d0 (lista dos humanos) roda nas duas; ela refaz a lista do zero
    let reg = match_day_marked_regions(d);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(d), || match_day_interest(d));
    PLACAR.registrar(igual && r == n, || format!("match_day_interest original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Addresses of the national-team marks (0xb67a58 + id) of human managers with flag bit 2 and a national team.
///
/// [pt] As marcas das selecoes dos tecnicos humanos (0xb67a58 + id), que a 0x6b66a0 poe uma vez
/// (com [0xa15704] ligado): para cada vaga de humano em 0xb63d60 (de 0x48 em 0x48 ate 0xb64738) com
/// +0x11c bit 2 e selecao (+0x71).
pub(crate) unsafe fn human_national_team_marks() -> Vec<usize> {
    let mut v = Vec::new();
    for e in (0xb63d60usize..0xb64738).step_by(0x48) {
        let p = *(e as *const usize);
        if p != 0 && *((p + 0x11c) as *const u8) & 2 != 0 {
            let q = *((p + 0x71) as *const usize);
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
/// is involved (0x7f9720). First marks the human managers' national teams once.
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
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let ligado: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let f5c2990: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_FRIENDLY_IS_OWN_COMP_RECORD);
    let vaga_de: unsafe extern "C" fn(u32, usize) -> u32 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
    let f7f9720: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_SCOUT_MANAGER_FIXTURE_HAS_SCOUTED_PLAYER);
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16) as i32;
    if g(0xa15704) != 0 {
        for e in human_national_team_marks() {
            crate::comparacao::ramo("sem humano selecao de humano");
            *(e as *mut u8) = 1;
        }
        *(G_QUICK_SIM_COMP_FLAGS_PENDING as *mut u32) = 0;
    }
    if g(0xdd7118) == 0 {
        return 1;
    }
    let (c1, c2) = (g(partida + 0x1c), g(partida + 0x20));
    if humano(c1, 1, 1, 0, 0) != 0 || humano(c2, 1, 1, 0, 0) != 0 {
        crate::comparacao::ramo("sem humano clube humano em campo");
        return 0;
    }
    for c in [c1, c2] {
        if ligado(c, 0, 1) != 0 && humano(ligado(c, 0, 1), 1, 1, 0, 0) != 0 {
            crate::comparacao::ramo("sem humano clube ligado");
            return 0;
        }
    }
    let final_ = || (f7f9720(g(G_SCOUT_MANAGER), partida) == 0) as u32;
    let comp = ler(partida);
    if comp == ler(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) || comp == ler(KEY_CLUB_COMP_WORLD_CLUB_CUP) {
        return 0;
    }
    if comp == ler(KEY_CLUB_COMP_WELSH_PREMIER_CUP) {
        let base = g(G_NATION_TABLE);
        for k in [ler(KEY_NATION_ENGLAND), ler(KEY_NATION_WALES)] {
            if *((base as isize + k as isize * 0x122 + 0x11c) as *const u8) & 2 != 0 {
                return 0;
            }
        }
    }
    if comp == ler(KEY_CLUB_COMP_RESERVE) && humano(c1, 1, 1, 0, 0) == 0 && humano(c2, 1, 1, 0, 0) == 0 {
        crate::comparacao::ramo("sem humano competicao direta");
        return final_();
    }
    // os 16 humanos no fim da tabela de pessoas
    let base = g(G_STAFF_TABLE);
    for k in 0..16isize {
        let pe = (base as isize + (ler(0xae2364) as isize - 1 - k) * 0x6e) as usize;
        if pe != 0 && *((pe + 0x5f) as *const u8) == 1 && g(pe + 0x39) != 0 {
            let nacao = ler(g(pe + 0x39) + 0x57);
            if nacao == ler(c1 + 0x57) || nacao == ler(c2 + 0x57) {
                crate::comparacao::ramo("sem humano tecnico da nacao");
                return 0;
            }
        }
    }
    if (selecao(c2) == 0 && f5c2990(g(G_FRIENDLY), ler(partida + 0x14) as u32) != 0) || comp == ler(KEY_CLUB_COMP_INTER_TOTO_CUP) {
        crate::comparacao::ramo("sem humano pela injury_word_0c");
        return if w(c2 + 0x80) + w(c1 + 0x80) > 0x2ee0 { 0 } else { final_() };
    }
    let mut x = vaga_de(ler(partida + 0x14) as u32, partida) as u8;
    if let Some(k) = (0xb63d74usize..G_KEY_NATION_ACTIVE_SLOT_LOOP_END).step_by(0x48).position(|e| *(e as *const u8) == x) {
        x = k as u8;
    }
    if x == 0 || x == 1 {
        return 0;
    }
    if matches!(x, 3..=7) {
        let q = g(g(partida + 0x14) + 0x59);
        if q != 0 && *((0xb67a58 + g(q)) as *const u8) != 0 {
            crate::comparacao::ramo("sem humano selecao marcada");
            return 0;
        }
    }
    if (x as i8) > -1 {
        let e = g(0xb63d60 + x as usize * 0x48);
        if e != 0 && *((e + 0x11c) as *const u8) & 2 != 0 {
            return 0;
        }
    }
    crate::comparacao::ramo("sem humano 7f9720");
    final_()
}

/// Comparer for `match_without_human` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_without_human(partida: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_MATCH_WITHOUT_HUMAN);
    if is_recording_top_level() {
        return o(partida);
    }
    let mut reg = vec![(0xa15704usize, 4usize)];
    if *(G_QUICK_SIM_COMP_FLAGS_PENDING as *const usize) != 0 {
        reg.extend(human_national_team_marks().into_iter().map(|e| (e, 1)));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(partida), || match_without_human(partida));
    PLACAR.registrar(igual && r == n, || format!("match_without_human original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// 0x6b79a0 with calls injected: a club's post-match news after a match against `outro` (turnaround story for a senior
/// player, or a pleased youngster favouring the club).
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
pub(crate) unsafe fn post_match_club_with(clube: usize, outro: usize, partida: usize, n: u32, t: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let teste_547d60: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let ch = core::cell::RefCell::new(chamar);
    let saida = |s: usize, c: usize, o: usize, n: i32| (ch.borrow_mut())(0x5896d0, 0, [s as u32, c as u32, o as u32, n as u32, 0]);
    let f589bc0 = |p: usize, c: usize| (ch.borrow_mut())(0x589bc0, 0, [p as u32, c as u32, 0, 0, 0]);
    let f547d90 = |p: usize, c: usize, um: u32| (ch.borrow_mut())(0x547d90, 0, [p as u32, c as u32, um, 0, 0]);
    let teste_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let teste_6b7cf0 = |pa: usize, c: usize| (ch.borrow_mut())(0x6b7cf0, 0, [pa as u32, c as u32, 0, 0, 0]) as u8;
    let fora = |c: usize| (ch.borrow_mut())(0x53b900, 0, [c as u32, 0, 0, 0, 0]) as i32;
    let teste_7c3a40 = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x7c3a40, n, [p as u32, c as u32, 0, 0, 0]) as u16;
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let ub = |a: usize| *(a as *const u8);
    if clube == 0 || partida == 0 || outro == 0 {
        // ponytail: a original mostra a caixa "Error" (match_day.cpp, linhas 0xc45/0xc4a/0xc4f); nao
        // acontece no jogo
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        return;
    }
    if g(clube + club::MANAGER) == 0 {
        return;
    }
    if teste_600a50(clube, 0, 1, 0, 0) == 0 && w(clube + club::REPUTATION) < 0x128e && sortear(100) != 0 {
        return;
    }
    crate::comparacao::ramo("clube segue");
    let n = n as u8 as i8;
    if teste_6b7cf0(partida, clube) != 0 && ub(partida + 0x40) != 0 && ub(partida + 0x41) != 0 && fora(clube) == 0 {
        for k in 0..50 {
            let s = g(clube + club::SQUAD + 4 * k);
            if s == 0 {
                continue;
            }
            let c = contrato(g(G_CONTRACT_MANAGER), s);
            let livre = c != 0 && ler(c + 0x45) == 0;
            if livre {
                crate::comparacao::ramo("clube elenco");
            }
            if livre
                && g(s + 0x61) != 0
                && w(g(s + 0x61) + 0xb) > 0x109a
                && byte(s, 0x59) > 5
                && byte(s, 0x57) > 0xf
                && byte(s, 0x58) > 0xa
                && n > 2
                && sortear(n as i32) != 0
            {
                crate::comparacao::ramo("clube 5896d0");
                saida(s, clube, outro, n as i32);
                return;
            }
        }
    }
    let lado = (ler(clube) != ler(t)) as usize;
    for k in 0..20 {
        let linha = t + 0x625 * lado + 0x1ce + k * 0x49;
        let id = ler(linha - 0x3e);
        if id == -1 || ub(linha) == 0 {
            continue;
        }
        let p = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
        if g(p + 0x61) == 0 || (p != 0 && ler(p) >= ler(0xae2364)) || byte(p, 0x18) > 0x14 {
            continue;
        }
        let c = contrato(g(G_CONTRACT_MANAGER), p);
        if c == 0 || ler(c + 0x45) != 0 || ub(partida + 0x40) == 0 || ub(partida + 0x41) == 0 {
            continue;
        }
        if ler(partida + 0x14) == 0 || ler(partida + 0x14) != ler(clube + club::DIVISION) || fora(clube) != 0 || byte(p, 0x59) <= 0xf {
            continue;
        }
        crate::comparacao::ramo("clube jovem");
        let r = sortear(5);
        if byte(p, 0x57) <= (r as u8).wrapping_add(10) as i8 || g(p + 0x61) == 0 || w(g(p + 0x61) + 0xb) >= 0x109a {
            continue;
        }
        if teste_7c3a40(g(G_PLAYER_STATS), p, clube) == 0 {
            crate::comparacao::ramo("clube 589bc0");
            f589bc0(p, clube);
            if teste_547d60(p, clube) == 0 {
                crate::comparacao::ramo("clube 547d90");
                f547d90(p, clube, 1);
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
pub(crate) unsafe extern "C" fn post_match_club(clube: usize, outro: usize, partida: usize, n: u32, t: usize) {
    post_match_club_with(clube, outro, partida, n, t, &mut |r, tt, a| dispatch_post_match_call(r, tt, a))
}

/// Calls one of the post-match routines with effects by game address, with the right convention and argument count.
///
/// [pt] As rotinas do banco com efeitos chamadas pelo pos-jogo, pelo endereco do jogo (`this` 0: cdecl).
pub(crate) unsafe fn dispatch_post_match_call(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    type T1 = unsafe extern "thiscall" fn(usize, u32) -> u32;
    type T2 = unsafe extern "thiscall" fn(usize, u32, u32) -> u32;
    type T3 = unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32;
    type T4 = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32;
    type T5 = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32;
    type C4 = unsafe extern "C" fn(u32, u32, u32, u32) -> u32;
    type C5 = unsafe extern "C" fn(u32, u32, u32, u32, u32) -> u32;
    match rotina {
        0x4cf5f0 | 0x7c3a40 => core::mem::transmute::<usize, T2>(rotina)(this, a[0], a[1]),
        0x600a50 => core::mem::transmute::<usize, C5>(rotina)(a[0], a[1], a[2], a[3], a[4]),
        0x8676d0 | 0x8674e0 | 0x867560 => core::mem::transmute::<usize, T1>(rotina)(this, a[0]),
        0x867700 | 0x733a60 | 0x5afe00 => core::mem::transmute::<usize, T2>(rotina)(this, a[0], a[1]),
        0x759610 => core::mem::transmute::<usize, T3>(rotina)(this, a[0], a[1], a[2]),
        0x744240 => core::mem::transmute::<usize, T4>(rotina)(this, a[0], a[1], a[2], a[3]),
        0x72c1f0 => core::mem::transmute::<usize, T5>(rotina)(this, a[0], a[1], a[2], a[3], a[4]),
        // cdecl: quem chama limpa a pilha, os argumentos a mais nao sao lidos
        _ => core::mem::transmute::<usize, C4>(rotina)(a[0], a[1], a[2], a[3]),
    }
}

// --- portas do pos-jogo: as 14 rotinas do banco com efeitos (e a consulta 0x7c3a40), gravadas na fita so durante as
// comparacoes do pos-jogo (RECORD_POST_MATCH); para os outros chamadores do jogo so repassam. Devolvem
// edx:eax inteiro (nenhuma devolve ponto flutuante: conferido na 132a leva).
pub(crate) static RECORD_POST_MATCH: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static mut PORTAS_POS: [usize; 15] = [0; 15];
pub(crate) const POS_PROLOGUES: [Prologo; 15] = [
    Prologo { endereco: 0x8676d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x40] },
    Prologo { endereco: 0x8674e0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x867560, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x867700, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x733a60, bytes: &[0x6a, 0xff, 0x68, 0xe6, 0xd6, 0x95, 0x00] },
    Prologo { endereco: 0x5afe00, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] },
    Prologo { endereco: 0x759610, bytes: &[0x6a, 0xff, 0x68, 0x83, 0xd9, 0x95, 0x00] },
    Prologo { endereco: 0x744240, bytes: &[0x6a, 0xff, 0x68, 0x06, 0xd8, 0x95, 0x00] },
    Prologo { endereco: 0x72c1f0, bytes: &[0x6a, 0xff, 0x68, 0x86, 0xd6, 0x95, 0x00] },
    Prologo { endereco: 0x7daf50, bytes: &[0x6a, 0xff, 0x68, 0x70, 0xfd, 0x95, 0x00] },
    Prologo { endereco: 0x547af0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x5896d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x589bc0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x547d90, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x7c3a40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
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
pub(crate) unsafe fn pass_or_record_post_match(rotina: usize, this: usize, a: [u32; 5], f: impl FnOnce() -> u64) -> u64 {
    use std::sync::atomic::Ordering::Relaxed;
    if RECORD_POST_MATCH.load(Relaxed) && is_recording() {
        let mut r = 0u64;
        call_recording(rotina, this, a, || {
            r = f();
            r as u32
        });
        return r;
    }
    f()
}

/// Gate for STAFF_CONTRACT::clear_flags (0x8676d0) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_clear_contract_flags(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[0]);
    pass_or_record_post_match(0x8676d0, c, [a, 0, 0, 0, 0], || f(c, a))
}
/// Gate for STAFF_CONTRACT::clear_grievances (0x8674e0) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_clear_grievances(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[1]);
    pass_or_record_post_match(0x8674e0, c, [a, 0, 0, 0, 0], || f(c, a))
}
/// Gate for STAFF_CONTRACT::set_flags (0x867560) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_set_contract_flags(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[2]);
    pass_or_record_post_match(0x867560, c, [a, 0, 0, 0, 0], || f(c, a))
}
/// Gate for STAFF_CONTRACT::apply_event_flags (0x867700) in post-match recording.
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
/// Gate for Media::create_player_media_story (0x72c1f0) in post-match recording.
pub(crate) unsafe extern "thiscall" fn gate_player_media_story(c: usize, a: u32, b: u32, d: u32, e: u32, g: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[8]);
    pass_or_record_post_match(0x72c1f0, c, [a, b, d, e, g], || f(c, a, b, d, e, g))
}
/// Gate for apply_obscene_gesture_ban (0x7daf50) in post-match recording.
pub(crate) unsafe extern "C" fn gate_obscene_gesture_ban(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[9]);
    pass_or_record_post_match(0x7daf50, 0, [a, b, d, 0, 0], || f(a, b, d))
}
/// Gate for staff_add_disliked_staff (0x547af0) in post-match recording.
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
/// Gate for staff_add_favoured_club (0x547d90) in post-match recording.
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

/// Comparer for `pos` (comparison mode: see the header of motor.rs).
///
/// [pt] Roda a comparacao de uma rotina do pos-jogo pela fita: as 14 do banco entram como unidades.
pub(crate) unsafe fn comparar_pos(nome: &str, reg: Vec<(usize, usize)>, original: impl FnOnce(), rust: impl FnOnce(&mut dyn FnMut(usize, usize, [u32; 5]) -> u32)) {
    use std::sync::atomic::Ordering::Relaxed;
    let gravar = RECORD_POST_MATCH.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, original, |chamar| rust(&mut |r, t, a| chamar(r, t, a, [u32::MAX; 5])));
    RECORD_POST_MATCH.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("{} ({} chamadas): {}", nome, k, tape_detail()));
}

/// The flags (+0x40..+0x48) of the person's contract at a club as a comparer region, or None.
///
/// [pt] As flags do contrato (+0x40 a +0x48) de `pessoa` no `clube`, como regiao.
pub(crate) unsafe fn contract_flags_region(pessoa: usize, clube: usize) -> Option<(usize, usize)> {
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = contrato(*(G_CONTRACT_MANAGER as *const usize), pessoa, clube);
    (c != 0).then_some((c + 0x40, 9))
}

/// Comparer for `post_match_events` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_post_match_events(partida: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_POST_MATCH_EVENTS);
    // o cursor da lista, as linhas dos dois clubes em [[0xb67a44]] e as flags dos contratos dos eventos
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let lista = *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *const usize);
    if lista != 0 {
        reg.push((lista, 0xe));
        let pessoa = |id: i32| (*(G_STAFF_TABLE as *const usize) as isize + id as isize * 0x6e) as usize;
        let mut no = core::ptr::read_unaligned((lista + 2) as *const usize);
        let mut k = 0;
        while no != 0 && k < core::ptr::read_unaligned(lista as *const i16) as i32 {
            let ev = core::ptr::read_unaligned(no as *const usize);
            no = core::ptr::read_unaligned((no + 6) as *const usize);
            k += 1;
            if ev == 0 {
                break;
            }
            let clube = (*(G_CLUB_TABLE as *const usize) as isize + core::ptr::read_unaligned((ev + 4) as *const i32) as isize * 0x245) as usize;
            if clube == *((partida + 0x1c) as *const usize) || clube == *((partida + 0x20) as *const usize) {
                reg.extend(contract_flags_region(pessoa(core::ptr::read_unaligned(ev as *const i32)), clube));
            }
        }
    }
    let tabela = *(G_MANAGER_MANAGER as *const usize);
    if tabela != 0 {
        for o in [0x1c, 0x20] {
            let clube = *((partida + o) as *const usize);
            if clube != 0 {
                reg.push(((*(tabela as *const usize) as isize + *(clube as *const i32) as isize * 0x49) as usize, 0x49));
            }
        }
    }
    comparar_pos("post_match_events", reg, || o(partida), |chamar| post_match_events_injected(partida, chamar));
}

/// Comparer for `post_match_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_post_match_players(partida: usize, t: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_POST_MATCH_PLAYERS);
    // as fichas dos jogadores (+0x45, +9 a +0xe), os contratos, +0x80 dos clubes e as fichas do
    // tecnico e dos auxiliares (+8 a +0xd)
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for lado in 0..2usize {
        let clube = (g(0xae23b4) as isize + ler(t + 4 * lado) as isize * 0x245) as usize;
        let ts = t + lado * 0x625;
        reg.push((clube + club::REPUTATION, 2));
        for k in 0..20usize {
            let id = ler(ts + 0x1c5 + k * 0x49 - 0x35);
            if id == -1 {
                continue;
            }
            let p = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            let f = g(p + 0x61);
            if f != 0 {
                reg.extend([(f + 0x45, 1), (f + 9, 6)]);
                reg.extend(contract_flags_region(p, clube));
            }
        }
        let id = ler(ts + 0x184);
        if id > -1 {
            let tec = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            let mut fichas = vec![g(tec + 0x69)];
            for a in [g(clube + club::ASSISTANT_MANAGER), g(clube + club::COACHES), g(clube + 0x1a3), g(clube + 0x1a7), g(clube + 0x1ab), g(clube + 0x1af)] {
                if a != 0 {
                    fichas.push(g(a + 0x69));
                }
            }
            for f in fichas.into_iter().filter(|&f| f != 0) {
                reg.push((f + 8, 6));
            }
        }
    }
    comparar_pos("post_match_players", reg, || o(partida, t), |chamar| post_match_players_with(partida, t, chamar));
}

/// Comparer for `post_match_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_post_match_club(clube: usize, outro: usize, partida: usize, n: u32, t: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, usize)>(IDX_POST_MATCH_CLUB);
    // a copia nao escreve no banco: so as chamadas e o sorteio
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    comparar_pos("post_match_club", reg, || o(clube, outro, partida, n, t), |chamar| post_match_club_with(clube, outro, partida, n, t, chamar));
}

/// the current human leaves the match he was watching; with no other human watching, the match runs to the end, is
/// recorded and leaves the queue
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
    crate::comparacao::ramo("telas largar a partida assistida");
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_SCREENMANAGER_FIND_MANAGER_CHANNEL);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_CM3_MATCH_ENGINE);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_INIT_MATCH);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_RUN_MATCH_STEPS);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MATCH_STATS_RECORD_MATCH);
    let destroy_engine: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_DESTRUCTOR_CM3_MATCH_ENGINE);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_VIRTUAL_STAFF_RELEASE_FIXTURE);
    let vez = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;

    let p = u((0xb74c26 + vez() * 0x300) as usize, 0);
    if p == 0 {
        return;
    }
    let a = ficha(p);
    if a == 0 {
        return;
    }
    let jogo = u(a, 0xe2);
    if jogo == 0 {
        return;
    }
    let mut sozinho = true;
    for i in 0..0x10usize {
        let idx = (*(G_STAFF_COUNT as *const usize)).wrapping_add(i).wrapping_sub(0x10);
        let q = u(G_STAFF_TABLE, 0).wrapping_add(idx.wrapping_mul(110));
        // como a original, sem conferir a ficha nula do outro humano
        if *((q + 0x5f) as *const u8) != 0 && q != p && u(ficha(q), 0xe2) == jogo {
            let minha = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const u16);
            if indice(G_SCREEN_MANAGER, q) == minha {
                return;
            }
            sozinho = false;
        }
    }
    let resumo = u(jogo, 0x5b);
    if resumo == 0 {
        return;
    }
    for (k, o) in [5isize, 0x45, 0x85, 0xc5].into_iter().enumerate() {
        let c = (resumo as isize + vez() * 4 + o) as *mut u32;
        if core::ptr::read_unaligned(c) != 0 {
            core::ptr::write_unaligned(c, 0);
            let n = (resumo + 0x105 + k) as *mut u8;
            *n = (*n).wrapping_sub(1);
        }
    }
    if !sozinho {
        return;
    }
    let motor = (jogo + 0x57) as *mut usize;
    if core::ptr::read_unaligned(motor) == 0 {
        let mem = novo(0xf63b);
        let m = if mem != 0 { construir(mem) } else { 0 };
        core::ptr::write_unaligned(motor, m);
        if m == 0 {
            show_error_at(0xa15708, 0xa4b);
            return;
        }
        preparar(m, jogo + 4, 0, 0, 0);
    }
    let estado = (jogo + 0x67) as *mut u8;
    while *estado != 2 && *estado != 8 {
        let e = laco(core::ptr::read_unaligned(motor), -1) as u8;
        *estado = e;
        if e == 2 || e == 8 {
            *((u(G_MATCH_DAY, 0) + 0x25) as *mut i32) += 1;
        }
    }
    if *estado != 8 {
        let mem = novo(0xd7a);
        if mem != 0 {
            vetor(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        core::ptr::write_unaligned((jogo + 0x5f) as *mut usize, mem);
        if mem == 0 {
            show_error_at(0xa15708, 0xa68);
            return;
        }
        registrar(u(G_MATCH_STATS, 0), core::ptr::read_unaligned(motor), mem);
    }
    let m = core::ptr::read_unaligned(motor);
    if m != 0 {
        destroy_engine(m);
        liberar(m);
    }
    core::ptr::write_unaligned(motor, 0);
    tirar(u(G_VIRTUAL_STAFF, 0), jogo + 4);
}

/// finds in the fixture list the match of a statistics record (date, competition and club) and copies the fixture
/// Original: `Match_Stats::find_record_fixture` (0x729e30, match_stats.cpp, thiscall).
///
/// [pt] { 0x729e30 } (thiscall, ret 8, match_stats.cpp): a ficha do jogo (0x4f bytes) do registro `k` do
/// índice de estatísticas ([this], 12 bytes: +0 competição, +4 um dos clubes, +8/+0xa a data): pede
/// à agenda [0xae2a58] (0x5acbe0) os jogos daquela data e competição numa lista local (0x68d020) e copia
/// para `saida` o primeiro com o clube +4 na casa (+0xc) ou fora (+0x10). Devolve 1 se achou. Índice
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn find_record_fixture(estat: usize, k: i32, saida: usize) -> u32 {
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_FIND_RECORD_FIXTURE)(estat, k, saida);
    }
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    // ponytail: a lista nasce zerada aqui, que é tudo o que o construtor 0x68d020 faz
    let mut lista = [0u8; 14];
    let l = lista.as_mut_ptr() as usize;
    let tab = *(estat as *const usize);
    let e = tab.wrapping_add((k as usize).wrapping_mul(12));
    // a original empurra os registradores inteiros: em cima, os 16 bits de [this] e de k × 3
    let d1 = (tab as u32 & 0xffff_0000) | *((e + 8) as *const u16) as u32;
    let d2 = ((k as u32).wrapping_mul(3) & 0xffff_0000) | *((e + 0xa) as *const u16) as u32;
    jogos(*(G_FIXTURE_MANAGER as *const usize), d1, d2, d1, d2, *(e as *const u32), l, u32::MAX, 0xffff_fffe, u32::MAX);
    // o cursor (+0xa) começa na cabeça (+2)
    core::ptr::copy(lista.as_ptr().add(2), lista.as_mut_ptr().add(0xa), 4);
    let clube = *((e + 4) as *const u32);
    let mut f = proximo(l);
    while f != 0 {
        if clube == *((f + 0xc) as *const u32) || clube == *((f + 0x10) as *const u32) {
            core::ptr::copy_nonoverlapping(f as *const u8, saida as *mut u8, 0x4f);
            destruir_lista(l);
            return 1;
        }
        f = proximo(l);
    }
    destruir_lista(l);
    0
}

// ponytail: a partida deixada cria e libera o motor, anda a partida até o fim e a registra no arquivo
// temporário: rodar duas vezes não dá, então o comparador só chama a original; a cópia se prova no
// modo Rust (o ramo), na sessão do Lucas com rede (só um jogador da rede saindo a chama).
/// Comparer for `match_day_drop_viewer_and_finish_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_day_drop_viewer_and_finish_match() {
    original_fn::<unsafe extern "C" fn()>(IDX_MATCH_DAY_DROP_VIEWER_AND_FINISH_MATCH)()
}

/// Comparer for `find_record_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_record_fixture(estat: usize, k: i32, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_FIND_RECORD_FIXTURE);
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return o(estat, k, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 0x4f)], || o(estat, k, saida), || find_record_fixture(estat, k, saida));
    PLACAR.registrar(igual && r == n, || format!("find_record_fixture(k {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

/// one step of the human's match on a network, by the clock and the humans' scoreboard (everyone sees each state;
/// sounds, events, engine step, end, half time, goals, minute); records it (0x729ff0) and picks the day's human match
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
pub(crate) unsafe extern "thiscall" fn network_step(dia: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| core::ptr::read_unaligned((q + o) as *const usize);
    let por = |q: usize, o: usize, v: u32| core::ptr::write_unaligned((q + o) as *mut u32, v);
    let andar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_DAY_STEP_BACKGROUND_MATCH);
    let iniciar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_START_NEXT_BACKGROUND_MATCH);
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    let sons: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MATCH_EVENTS_IS_LINE_FINISHED);
    let next_event: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_GET_NEXT_EVENT);
    let tabela: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_MATCH_EVENTS_GET_EVENT_PRIORITY);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_RUN_MATCH_STEPS);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MATCH_STATS_FILL_MATCH_RECORD);
    let competicao: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(FN_MATCH_DAY_GET_PERIOD_RANK);
    let seguir = || {
        if andar(dia, 0) != 0 {
            iniciar(dia);
        }
        0u32
    };
    if p == 0 {
        return seguir();
    }
    let pl = u(p, 0x5b);
    crate::comparacao::ramo("step_towards em rede");
    let motor = u(p, 0x57);
    let estado = pl as *mut u8;
    let eu = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let visto = |i: isize| ((pl as isize) + 0x85 + 4 * i) as *mut u32;
    // todos viram de novo: so o humano da vez marca e espera
    let rever = || {
        core::ptr::write_bytes((pl + 0x85) as *mut u8, 0, 0x40);
        core::ptr::write_unaligned(visto(eu), 1);
        *((pl + 0x107) as *mut u8) = 1;
    };
    let mostrar_evento = |ev: usize| {
        if tabela(u(motor, 0x8f7a), *(ev as *const u16) as u32) == 2 {
            por(pl, 0x115, 1);
        }
        por(pl, 1, ev as u32);
        *estado = 8;
        rever();
        por(pl, 0x109, 0);
    };
    if *estado != 0 && *((pl + 0x107) as *const u8) != *((pl + 0x105) as *const u8) {
        if core::ptr::read_unaligned(visto(eu)) != 0 {
            return 0;
        }
        core::ptr::write_unaligned(visto(eu), 1);
        *((pl + 0x107) as *mut u8) += 1;
        por(pl, 0x10d, relogio());
        return 1;
    }
    if *((pl + 0x108) as *const u8) != 0 {
        return seguir();
    }
    if *estado & 8 != 0 && sons(u(motor, 0x8f7a), pl + 5) == 0 {
        return seguir();
    }
    let espera = if *((u(motor, 0x8f7a) + 0x2db0) as *const u8) == 0x14 { 0 } else { u(pl, 0x109) as u32 };
    if relogio() < (u(pl, 0x10d) as u32).wrapping_add(espera) {
        return seguir();
    }
    if *estado & 8 != 0 {
        let ev = next_event(u(motor, 0x8f7a), 0);
        if ev != 0 {
            mostrar_evento(ev);
            por(pl, 0x10d, relogio());
            return 1;
        }
    }
    if u(pl, 0x111) != 0 {
        por(pl, 0x115, 0);
        let r = laco(motor, 1) as u8;
        *((p + 0x67) as *mut u8) = r;
        if r == 2 || r == 8 {
            *((dia + 0x25) as *mut i32) += 1;
        }
        if *((u(motor, 0x8f7a) + 5) as *const u8) != 0 {
            let ev = next_event(u(motor, 0x8f7a), 0);
            if ev != 0 {
                mostrar_evento(ev);
                por(pl, 0x10d, relogio());
                por(pl, 0x111, 0);
                return 1;
            }
        }
    }
    *estado = 0;
    let fim = *((p + 0x67) as *const u8);
    let resumo = u(p, 0x5f);
    let marcar = |bit: u8| {
        por(pl, 0x109, 0);
        *(pl as *mut u8) |= bit;
    };
    if fim == 2 {
        *estado = 0x40;
        por(pl, 0x109, 0);
    }
    if *((p + 0x67) as *const u8) == 8 {
        marcar(0x40);
    }
    if *((p + 0x67) as *const u8) == 4 {
        marcar(0x20);
    }
    if byte(motor, 0xf5bd) as i32 > *((resumo + 0x11f) as *const u8) as i32 {
        marcar(1);
    }
    if byte(motor, 0xf5f3) as i32 > *((u(p, 0x5f) + 0x744) as *const u8) as i32 {
        marcar(2);
    }
    if u(pl, 0x115) != 0 {
        marcar(0x10);
    }
    if *((motor + 0x8eb2) as *const u8) != 0 {
        marcar(0x80);
    }
    let r = u(p, 0x5f);
    if *((motor + 0x8ed0) as *const i16) as i32 / 11 > *((r + 0x4d) as *const i16) as i32 {
        *estado |= 4;
        por(pl, 0x109, (0x226 - 50 * *((u(p, 0x5f) + 0x51) as *const i16) as i32) as u32);
    }
    por(pl, 0x111, 1);
    if *estado == 0 {
        return 0;
    }
    rever();
    por(pl, 0x10d, relogio());
    registrar(u(G_MATCH_STATS, 0), motor, u(p, 0x5f), 1);
    let s = u(u(dia, 0x34), 0x5f);
    let trocar = if s != 0 {
        let a = competicao(*((u(p, 0x5f) + 0x4f) as *const u16) as u32, 1) as i8;
        let b = competicao(*((s + 0x4f) as *const u16) as u32, 1) as i8;
        a > b || *((u(p, 0x5f) + 0x4d) as *const i16) > *((u(u(dia, 0x34), 0x5f) + 0x4d) as *const i16)
    } else {
        true
    };
    if trocar {
        por(dia, 0x34, p as u32);
    }
    1
}

/// Comparer for `network_step` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_network_step(dia: usize, p: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NETWORK_STEP)(dia, p)
}

/// each human confirms the network match; with all of them, creates the engine and prepares it for watching, or plays
/// it to the end and records it
/// Original: `confirm_network_match` (0x6b5260, match_day.cpp, thiscall, ret 4).
///
/// [pt] { 0x6b5260 } (thiscall, ret 4, match_day.cpp): o humano da vez ([0xb77c7a]) confirma a partida
/// `p` em rede (placar dos humanos +0x5b: +0x45 quem confirmou, +0x105 humanos, +0x106 confirmados).
/// Com todos: cria o motor (0x6b7e50) e o prepara para ver (0x6b7f20 com o ultimo 1). Se ele nao
/// for de ver (devolve 0), joga ate o fim (contando em [0xb67a50]+0x25), cria o resumo, registra
/// (0x7293f0), libera o motor, tira da fila e marca +0x119; se for, avisa o narrador (+0x8f7a = 3),
/// cria o resumo e registra para ver (0x729ff0) e devolve 1. Erros 0x4b8 (sem partida), 0x4d0, 0x4ea,
/// 0x511 (sem memoria).
pub(crate) unsafe extern "thiscall" fn confirm_network_match(_dia: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| core::ptr::read_unaligned((q + o) as *const usize);
    let por = |q: usize, o: usize, v: usize| core::ptr::write_unaligned((q + o) as *mut usize, v);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_CM3_MATCH_ENGINE);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_INIT_MATCH);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_RUN_MATCH_STEPS);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MATCH_STATS_RECORD_MATCH);
    let registrar_para_ver: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MATCH_STATS_FILL_MATCH_RECORD);
    let destroy_engine: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_DESTRUCTOR_CM3_MATCH_ENGINE);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_VIRTUAL_STAFF_RELEASE_FIXTURE);
    const FONTE: usize = 0xa15708; // match_day.cpp
    if p == 0 {
        show_error_at(FONTE, 0x4b8);
        return 0;
    }
    let pl = u(p, 0x5b);
    let eu = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let confirmou = (pl as isize + eu * 4 + 0x45) as *mut u32;
    if core::ptr::read_unaligned(confirmou) == 0 {
        core::ptr::write_unaligned(confirmou, 1);
        *((pl + 0x106) as *mut u8) = (*((pl + 0x106) as *const u8)).wrapping_add(1);
    }
    if *((pl + 0x106) as *const u8) != *((pl + 0x105) as *const u8) {
        return 0;
    }
    crate::comparacao::ramo("partida em rede confirmada");
    let mem = novo(0xf63b);
    let motor = if mem != 0 { construir(mem) } else { 0 };
    por(p, 0x57, motor);
    if motor == 0 {
        show_error_at(FONTE, 0x4d0);
        return 0;
    }
    let novo_resumo = || {
        let r = novo(0xd7a);
        if r != 0 {
            vetor(r + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        por(p, 0x5f, r);
        r
    };
    if preparar(motor, p + 4, 0, 0, 1) as i8 == 0 {
        let terminou = |e: u8| e == 2 || e == 8;
        let estado = (p + 0x67) as *mut u8;
        while !terminou(*estado) {
            let e = laco(u(p, 0x57), -1) as u8;
            *estado = e;
            if terminou(e) {
                *((u(G_MATCH_DAY, 0) + 0x25) as *mut i32) += 1;
            }
        }
        let r = novo_resumo();
        if r == 0 {
            show_error_at(FONTE, 0x4ea);
            return 0;
        }
        registrar(u(G_MATCH_STATS, 0), u(p, 0x57), r);
        let m = u(p, 0x57);
        if m != 0 {
            destroy_engine(m);
            liberar(m);
        }
        por(p, 0x57, 0);
        tirar(u(G_VIRTUAL_STAFF, 0), p + 4);
        por(pl, 0x119, 1);
        return 0;
    }
    let narrador = u(u(p, 0x57), 0x8f7a);
    if narrador != 0 {
        *(narrador as *mut u8) = 3;
    }
    let r = novo_resumo();
    if r == 0 {
        show_error_at(FONTE, 0x511);
        return 0;
    }
    registrar_para_ver(u(G_MATCH_STATS, 0), u(p, 0x57), r, 1);
    1
}

/// Comparer for `confirm_network_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_confirm_network_match(dia: usize, p: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CONFIRM_NETWORK_MATCH)(dia, p)
}

/// the number by code (0: 5/4, 1: 11/10, 0x1ef: 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; the first with the second
/// argument 1); any other code: the original (error box). Compared by the result
/// Original: `competition_number` (0x6b6570, match_day.cpp (304 bytes), cdecl).
///
/// [pt] { 0x6b6570 } (cdecl, match_day.cpp): o numero da competicao pelo codigo (0: 5/4, 1: 11/10, 0x1ef:
/// 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; o primeiro com `segunda` 1). Outro codigo: a original
/// (caixa de erro).
pub(crate) unsafe extern "C" fn competition_number(codigo: u32, segunda: u32) -> u32 {
    let s = (segunda as u8 == 1) as u32;
    crate::comparacao::ramo("numero da competicao");
    match codigo as u16 as i16 {
        0 => 4 + s,
        1 => 10 + s,
        0x1ef => s,
        0x3de => 2 + s,
        0x483 => 6 + s,
        0x528 => 8 + s,
        _ => original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_COMPETITION_NUMBER)(codigo, segunda),
    }
}

/// Comparer for `competition_number` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_competition_number(codigo: u32, segunda: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_COMPETITION_NUMBER);
    let r = o(codigo, segunda);
    if matches!(codigo as u16 as i16, 0 | 1 | 0x1ef | 0x3de | 0x483 | 0x528) {
        let n = competition_number(codigo, segunda);
        PLACAR.registrar(r as u8 == n as u8, || format!("competition_number({:#x}, {}) original={} rust={}", codigo as u16, segunda as u8, r as u8, n as u8));
    }
    r
}
