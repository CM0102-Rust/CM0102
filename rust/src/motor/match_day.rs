// ---------- trigesima oitava leva: jogos do dia (match_day.cpp) ----------

/// { 0x6b59a0 } (match_day.cpp): o clube tem jogo na lista do dia ([this + 0x19], quantidade em
/// 0x21, 0x69 bytes por jogo; mandante em 0x20, visitante em 0x24)? Com `so_marcados`, so os jogos
/// com 0x5b preenchido. Clube nulo: o original mostra uma caixa de erro e devolve 0 (aqui, so 0).
unsafe extern "thiscall" fn tem_jogo(this: usize, clube: u32, so_marcados: u32) -> u32 {
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

/// { 0x6b7cf0 } (match_day.cpp, cdecl): o clube venceu o jogo? Decide pelos penaltis (0x49/0x4a),
/// senao pela prorrogacao (0x47/0x48), senao pelo tempo normal (0x43/0x44); placar -1 nao conta.
/// Jogo ou clube nulo, ou clube fora do jogo: o original mostra uma caixa de erro e devolve 0.
unsafe extern "C" fn venceu_o_jogo(jogo: usize, clube: u32) -> u8 {
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

// ---------- centesima quadragesima nona leva: a escalacao de cada lado e o dia de jogos (0x6db500, 0x6b4190, 0x6b42a0) ----------

/// A caixa de erro "v%s %s.%s %d" de match_man.cpp na linha `linha`, e o jogo para (como a original).
unsafe fn erro_de_match_man(linha: u32) -> ! {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa15da4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
    fechar1();
    fechar2();
    sair(-1)
}

/// { 0x6db500 } (ret 0x24, match_man.cpp): monta a tatica `t` de um lado (+0x9032 ou +0x9102 do
/// motor) e escala o time: o tecnico e a ficha dele, os jogadores do bloco, o lado, o clube, o
/// adversario e a partida; o modo (+0x19 humano/+0x1a 3) pelo mando. No clube do humano, as linhas
/// taticas guardadas (0x5fbe20/0x5fbd00) e, sem o tecnico humano, a escalacao automatica no modo 1
/// (0x6dbc50, com o tecnico do clube 0x600e90) ou os titulares (0x6dbb80); fora isso, a automatica no
/// modo 2 e a conta dos da casa. Os reservas (+0x17) pelo banco da partida; le a linha tatica do
/// clube. Devolve quantos foram escalados; em `*vagas`, o tamanho do time.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn escalar_lado(t: usize, m: usize, tecnico: usize, linhas: usize, jogadores: usize, lado: u32, clube: usize, outro: usize, jogo: usize, vagas: *mut i8) -> i8 {
    let b = |o: usize| (t + o) as *mut u8;
    let d = |o: usize| (t + o) as *mut usize;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    if m == 0 {
        erro_de_match_man(0x69);
    }
    if linhas == 0 {
        erro_de_match_man(0x6b);
    }
    if jogadores == 0 {
        erro_de_match_man(0x6d);
    }
    if clube == 0 {
        erro_de_match_man(0x6f);
    }
    if outro == 0 {
        erro_de_match_man(0x71);
    }
    if tecnico != 0 {
        *d(0x1b) = tecnico;
        *d(0x1f) = u(tecnico, 0x69);
    } else {
        *d(0x1b) = 0;
        *d(0x1f) = 0;
    }
    *d(0x23) = jogadores;
    *b(0x18) = lado as u8;
    *d(0x27) = linhas;
    *d(0x2f) = clube;
    *d(0x12) = m;
    *d(0x37) = jogo;
    *d(0x3b) = 0;
    *d(0xe) = 0;
    *d(0xa) = 0;
    *d(6) = 0;
    *d(0x2b) = m + 0x91e2 + (lado as u8 as i8 as isize * 0x18e3) as usize;
    *d(0x33) = outro;
    let mandante = (clube == u(jogo, 0x1c) && byte(jogo, 0x40) == 0) || (clube == u(jogo, 0x20) && byte(jogo, 0x41) == 0);
    if mandante {
        *b(0x19) = 1;
        *b(0x1a) = 0;
    } else {
        *b(0x19) = 0;
        *b(0x1a) = 3;
    }
    let ficha = *d(0x1f);
    if ficha != 0 {
        *((t) as *mut u16) = *((ficha + 0xa) as *const u16);
        *((t + 2) as *mut u16) = *((ficha + 8) as *const u16);
        *((t + 4) as *mut u16) = *((ficha + 0xc) as *const u16);
    }
    let humano_usa: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53ca90usize);
    let tecnico_do_clube: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x600e90usize);
    let automatica: unsafe extern "thiscall" fn(usize, usize, u32) -> i8 = core::mem::transmute(0x6dbc50usize);
    let gerente = *(0xb63c98 as *const usize);
    let n: i8;
    if *(m as *const u8) == 0 && humano_usa(clube) != 0 && clube_humano(clube, 0, 1, 0, 0) != 0 {
        if e_segunda_selecao(clube) != 0 {
            *d(0x1b) = u(selecao_principal(u(clube, 0x53)), 0xcf);
        }
        let preparar: unsafe extern "thiscall" fn(usize, i32, usize, usize) = core::mem::transmute(0x5fbe20usize);
        let guardadas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fbd00usize);
        preparar(gerente, byte(t, 0x19) as i32, clube, u(clube, 0xcf));
        let r = guardadas(gerente, u(clube, 0xcf));
        if r == 0 || *d(0x2b) == 0 {
            erro_de_match_man(0xbc);
        }
        core::ptr::copy_nonoverlapping(r as *const u8, *d(0x2b) as *mut u8, 0x18e3);
        let sem_tecnico = |t: usize| {
            let x = tecnico_do_clube(*((t + 0x2f) as *const usize));
            *((t + 0x1b) as *mut usize) = x;
            *((t + 0x1f) as *mut usize) = if x != 0 { *((x + 0x69) as *const usize) } else { 0 };
            automatica(t, outro, 1)
        };
        n = if *b(0x19) == 0 {
            titulares(t) as i8
        } else if e_selecao(clube) == 0 {
            if tecnico_9a05(gerente, u(clube, 0xcf)) == 0 { sem_tecnico(t) } else { titulares(t) as i8 }
        } else if tecnico_9a06(gerente, u(clube, 0xcf), core::ptr::null_mut()) == 0 {
            sem_tecnico(t)
        } else {
            titulares(t) as i8
        };
        *vagas = ((*((jogo + 0x4d) as *const u8) & 0xf) + 0xb) as i8;
    } else {
        let x = *d(0x1b);
        if x != 0 {
            let base = *(0xae2364 as *const i32);
            let id = *(x as *const i32);
            if id >= base - 0x10 && id < base && byte(x, 0x5f) != 0 {
                let y = tecnico_do_clube(*d(0x2f));
                *d(0x1b) = y;
                *d(0x1f) = if y != 0 { u(y, 0x69) } else { 0 };
            }
        }
        n = automatica(t, outro, 2);
        *vagas = 0;
        let total = (*((jogo + 0x4d) as *const u8) & 0xf) + 0xb;
        let mut k = 0u8;
        while (k as i8) < total as i8 {
            let p = u(jogadores + k as usize * 0x1be, 0x69);
            if p != 0 && *(p as *const i32) < *(0xae2364 as *const i32) {
                *vagas = (*vagas).wrapping_add(1);
            }
            k += 1;
        }
    }
    let banco = (*((jogo + 0x4d) as *const u8) >> 4) & 0xf;
    let sobra = (n as u8).wrapping_sub(0xb) as i8;
    let r = if banco as i8 >= sobra { sobra } else { banco as i8 };
    *b(0x17) = if r <= 0 { 0 } else if (banco as i8) < sobra { banco } else { sobra as u8 };
    ler_linha_tatica(*(0xde1d08 as *const usize), *d(0x2f), (t + 0x3f) as *mut u8, byte(t, 0x19) as i32 as u32);
    n
}

/// { 0x6b4190 } (match_day.cpp): o fim do dia de jogos. Libera a lista global [0xb67dc8], os dois
/// vetores do dia (+1, +0xd) e, para cada partida (+0x19, 0x69 bytes cada, +0x21 delas), o motor
/// (0x6b97a0 e liberado; a partida sai de [0xde2f70] por 0x90d6b0), o resumo (+0x5b) e o objeto de
/// +0x5f (0x453bc0); por fim o vetor das partidas.
unsafe extern "thiscall" fn encerrar_dia(dia: usize) {
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let liberar_vetor: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d6b0usize);
    let destruir_5f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x453bc0usize);
    let destruir_motor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6b97a0usize);
    let d = |o: usize| (dia + o) as *mut usize;
    let lista = *(0xb67dc8 as *const usize);
    if lista != 0 {
        destruir_lista(lista);
        liberar(lista);
        *(0xb67dc8 as *mut usize) = 0;
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
            destruir_motor(motor);
            liberar(motor);
            *q(0x57) = 0;
            tirar(*(0xde2f70 as *const usize), p + 4);
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

/// { 0x6b42a0 } (match_day.cpp): com partidas no dia (+0x2b), limpa os 0x40 bytes de cada uma
/// (+0xd, 0x54 bytes, +0x15 delas) a partir de +0x14 e escolhe a do humano (+0x10; senao a primeira
/// com +0xc) para a tela (0x717cc0 e 0x7fc7e0 com [0xb67a48] ligado). Devolve 1 se havia uma do
/// humano.
unsafe extern "thiscall" fn escolher_partida_do_dia(dia: usize) -> u32 {
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
    let mostrar: unsafe extern "C" fn(u32, usize) = core::mem::transmute(0x717cc0usize);
    let atualizar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7fc7e0usize);
    *(0xb67a48 as *mut u32) = 1;
    mostrar(0, escolhida);
    atualizar(0xb74450);
    *(0xb67a48 as *mut u32) = 0;
    achou
}

// ponytail: a escalacao de cada lado chama a automatica (0x6dbc50, que aloca a lista e le a pilha) e o
// dia de jogos libera motores e listas: sem comparacao propria, validados pela carreira (Rust x
// original), como o nascimento e o fim do motor. As rotinas que chamam seguem comparadas.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_escalar_lado(t: usize, m: usize, tecnico: usize, linhas: usize, jogadores: usize, lado: u32, clube: usize, outro: usize, jogo: usize, vagas: *mut i8) -> i8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize, usize, usize, *mut i8) -> i8>(328)(t, m, tecnico, linhas, jogadores, lado, clube, outro, jogo, vagas)
}
unsafe extern "thiscall" fn comparar_encerrar_dia(dia: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(329)(dia)
}
unsafe extern "thiscall" fn comparar_escolher_partida_do_dia(dia: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(330)(dia)
}

// ---------- centesima quinquagesima primeira leva: o andamento do dia de jogos (0x6b5a70, 0x6b5ce0, 0x6b5f00) ----------

/// { 0x6b5a70 } (thiscall, match_day.cpp): comeca a proxima partida do dia (+0x19, 0x69 bytes cada,
/// +0x21 delas) depois da atual (+0x2f; -2 = acabou). A atual ainda com motor (+0x57) e erro 0x68d.
/// Pula as ja decididas (+0x47 -2), as sem jogo marcado na 2a volta (+0x33 1 e +0x63.+0x10 zero), as
/// de clube humano (0x600a50 com +0x44/+0x45) e as ja comecadas (+0x47 != -1, +0x57, +0x53). Na
/// escolhida: tira da fila [0xde2f70] (0x90d2f0 com -3), cria o motor (0xf63b bytes, 0x6b7e50; sem
/// ele erro 0x6b2 e segue) e o prepara (0x6b7f20 com a rapida de 0x6b66a0); +0x2f = ela, devolve 1.
/// Sem nenhuma: na 1a volta (+0x33 1) passa a 2 e recomeca do inicio; senao +0x2f = -2 e devolve 0.
unsafe extern "thiscall" fn iniciar_proxima_partida(dia: usize) -> u32 {
    let ler = |o: usize| *((dia + o) as *const i32);
    let por = |o: usize, v: i32| *((dia + o) as *mut i32) = v;
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(0x90d2f0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x6b7e50usize);
    let rapida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6b66a0usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x6b7f20usize);
    let atual = ler(0x2f);
    if atual == -2 {
        return 0;
    }
    if atual > -1 && u(u(dia, 0x19).wrapping_add((atual as isize * 0x69) as usize), 0x57) != 0 {
        aviso_em(0xa15708, 0x68d);
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
                tirar(u(0xde2f70, 0), p + 4, -3, 0);
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
                aviso_em(0xa15708, 0x6b2);
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

/// { 0x6b5ce0 } (thiscall, ret 4, match_day.cpp): faz andar a partida atual do dia (+0x2f). Sem
/// `ate_o_fim`, so 1 de cada 3 (0x40000 em [0xae3138]+4) ou 6 (0x80000) chamadas, pelo contador
/// [0xb67a54] (0..100), e um passo do motor (0x6b98c0 com 1); com `ate_o_fim`, passos (-1) ate o
/// fim. O estado vai em +0x67; ao terminar (2 ou 8) conta em +0x25, cria o resumo (+0x5f, 0xd7a
/// bytes com 2 objetos de 0x625 em +0x11f; sem ele erro 0x713), registra (0x7293f0), libera o motor
/// e tira a partida da fila [0xde2f70] (0x90d6b0). Devolve 1 se a partida terminou.
unsafe extern "thiscall" fn andar_partida_do_dia(dia: usize, ate_o_fim: u32) -> u32 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x6b98c0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7293f0usize);
    let destruir_motor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6b97a0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d6b0usize);
    let contador = 0xb67a54 as *mut i32;
    *contador += 1;
    if *contador > 100 {
        *contador = 0;
    }
    if ate_o_fim == 0 {
        let modo = u(u(0xae3138, 0), 4);
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
    let passo = |n: i32| {
        let e = laco(u(p, 0x57), n) as u8;
        *estado = e;
        if fim(e) {
            *((dia + 0x25) as *mut i32) += 1;
        }
    };
    if ate_o_fim != 0 {
        crate::comparacao::ramo("andar ate o fim");
        while !fim(*estado) {
            passo(-1);
        }
    } else {
        passo(1);
        crate::comparacao::ramo("andar um passo");
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
        aviso_em(0xa15708, 0x713);
        return 0;
    }
    registrar(u(0xb6b7c0, 0), u(p, 0x57), mem);
    let motor = u(p, 0x57);
    if motor != 0 {
        destruir_motor(motor);
        liberar(motor);
    }
    *((p + 0x57) as *mut usize) = 0;
    tirar(u(0xde2f70, 0), p + 4);
    1
}

/// { 0x6b5f00 } (thiscall, ret 4, match_day.cpp): o fechamento de uma partida do dia `p` (0x69
/// bytes): copia os 0x4f bytes de p+4 para a partida (+0) e, com competicao (+0x14), le o numero da
/// rodada de um clube humano ([adadfc] da competicao +8, 0x4a4850, +0x2d acima de 1; como na
/// original, olha so o visitante +0x20 nas duas voltas) e registra o resultado: p+0x47 = -3 (jogo de
/// fora, 0x924050 com os bits 12-13 de +0x4d e a copia com placares -1 em 0x5ab510/0x5aa680) ou o
/// metodo 1 da competicao. Depois o pos-jogo com o resumo (p+0x5f): jogadores (0x6b6cc0), eventos
/// (0x6b74a0), contratos (0x4ca6d0), suspensoes (0x55ca00), 0x76b600 (fora do -3), estados
/// (0x62c080) e, fora do -3, a historia ([0xb74434] metodo 1, fora da [0x9cf684]), 0x7c0d70,
/// 0x8c39f0, 0x444930, 0x868830, 0x4134c0, 0x6951e0, 0x59a5d0, 0x7f8c00 e o clube de cada lado
/// (0x6b79a0 com a rodada). Sem `p`, erro 0x73f e 0.
unsafe extern "thiscall" fn fechar_partida_do_dia(_dia: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    if p == 0 {
        aviso_em(0xa15708, 0x73f);
        return 0;
    }
    let jogo = u(p, 0);
    core::ptr::copy((p + 4) as *const u8, jogo as *mut u8, 0x4f);
    let resumo = u(p, 0x5f);
    let mut rodada = [0u8; 4];
    let competicao = |id: i32| *((u(0xadadfc, 0) as isize + 4 * id as isize) as *const usize);
    if u(jogo, 0x14) != 0 {
        crate::comparacao::ramo("fechar com competicao");
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
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
            let fora: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x924050usize);
            let gravar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(0x5ab510usize);
            let juntar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x5aa680usize);
            fora(u(0xde2f74, 0), p + 4, ((*((jogo + 0x4d) as *const u16) >> 12) & 3) as u32);
            gravar(u(0xae2a58, 0), copia.as_mut_ptr());
            juntar(u(0xae2a58, 0), copia.as_mut_ptr(), 1);
        } else {
            let c = competicao(*(jogo as *const i32));
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(u(u(c, 0), 4));
            metodo(c, jogo, resumo);
            crate::comparacao::ramo("fechar metodo da competicao");
        }
    }
    *(0xa801bc as *mut u32) = 1;
    let jogadores: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x6b6cc0usize);
    let eventos: unsafe extern "C" fn(usize) = core::mem::transmute(0x6b74a0usize);
    jogadores(jogo, resumo);
    eventos(jogo);
    let dois = |this: usize, f: usize| {
        let g: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(f);
        g(this, jogo, resumo);
    };
    dois(u(0xae1050, 0), 0x4ca6d0);
    dois(u(0xae28f0, 0), 0x55ca00);
    if byte(p, 0x47) != -3 {
        dois(u(0xb74318, 0), 0x76b600);
    }
    dois(u(0xb63d28, 0), 0x62c080);
    if byte(p, 0x47) == -3 {
        return 1;
    }
    if *(jogo as *const u32) != *(0x9cf684 as *const u32) {
        let h = u(0xb74434, 0);
        dois(h, u(u(h, 0), 4));
        crate::comparacao::ramo("fechar historia");
    }
    for (this, f) in [(0xb74350, 0x7c0d70), (0xde1f54, 0x8c39f0), (0xad9dac, 0x444930), (0xdd7f00, 0x868830), (0xad9c60, 0x4134c0), (0xb67a44, 0x6951e0), (0xae2a38, 0x59a5d0), (0xb74448, 0x7f8c00)] {
        dois(u(this, 0), f);
    }
    let clube: unsafe extern "C" fn(usize, usize, usize, u32, usize) = core::mem::transmute(0x6b79a0usize);
    // ponytail: a rodada vai como dword com os altos zerados; a original passa sobra da pilha neles (lidos como byte)
    clube(u(jogo, 0x1c), u(jogo, 0x20), jogo, rodada[0] as u32, resumo);
    clube(u(jogo, 0x20), u(jogo, 0x1c), jogo, rodada[1] as u32, resumo);
    1
}

// ponytail: o dia de jogos cria e libera motores e resumos e chama dezenas de rotinas ainda
// originais; sem comparacao propria, validadas pela carreira (Rust x original), como as da 149a.
unsafe extern "thiscall" fn comparar_iniciar_proxima_partida(dia: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(332)(dia)
}
unsafe extern "thiscall" fn comparar_andar_partida_do_dia(dia: usize, ate_o_fim: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(333)(dia, ate_o_fim)
}
unsafe extern "thiscall" fn comparar_fechar_partida_do_dia(dia: usize, p: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(334)(dia, p)
}

// ---------- centesima quinquagesima segunda leva: o laco do dia de jogos (0x6b4360) ----------

/// { 0x6b4360 } (thiscall, match_day.cpp, ~3,3 KB): joga o dia `dia` (+0x19 partidas de 0x69 bytes,
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
unsafe extern "thiscall" fn jogar_o_dia(dia: usize) -> u32 {
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let por = |p: usize, o: usize, v: usize| core::ptr::write_unaligned((p + o) as *mut usize, v);
    let ler = |o: usize| *((dia + o) as *const i32);
    let rede = || *(0xa5a390 as *const u8) != 0;
    let gerente = || u(0xb63c98, 0);
    let partida = |k: i32| u(dia, 0x19).wrapping_add((k as isize * 0x69) as usize);
    let humano_a = |i: i32| u(0xae23bc, 0).wrapping_add(((*(0xae2364 as *const i32) - i - 1) as isize * 0x6e) as usize);
    let humano_b = |i: i32| u(0xae23bc, 0).wrapping_add(((*(0xae2364 as *const i32) + i - 0x10) as isize * 0x6e) as usize);
    let iniciar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6b5a70usize);
    let andar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6b5ce0usize);
    let fechar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6b5f00usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let usa: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let tirar_da_fila: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(0x90d2f0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d6b0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let avisar_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x4542f0usize);
    let nivel: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x5fcff0usize);
    // 0x5fe290 (ret 8): a partida e o humano (a original empilha o humano antes do teste)
    let em_rede: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5fe290usize);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let f5fd2f0: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fd2f0usize);
    let f727f00: unsafe extern "C" fn(usize) = core::mem::transmute(0x727f00usize);
    let repassar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fd100usize);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x7fffa0usize);
    let fora_do_dia: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x71b1b0usize);
    let redesenhar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7fc7e0usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x6b98c0usize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7293f0usize);
    let destruir_motor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6b97a0usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x6b7e50usize);
    let rapida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6b66a0usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x6b7f20usize);
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x5e8160usize);
    let ajustar_relogio: unsafe extern "C" fn(u32) = core::mem::transmute(0x5e8170usize);
    let f68d520: unsafe extern "C" fn() = core::mem::transmute(0x68d520usize);
    let f788010: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x788010usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mostrar: unsafe extern "thiscall" fn(usize, usize, i32, u32, i32) = core::mem::transmute(0x802f80usize);
    let preparar_linhas: unsafe extern "thiscall" fn(usize, i32, usize, usize) = core::mem::transmute(0x5fbe20usize);
    let guardadas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fbd00usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5b5960usize);
    let data_texto: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x54ba20usize);

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
            destruir_motor(m);
            liberar(m);
        }
        por(p, 0x57, 0);
        tirar(u(0xde2f70, 0), p + 4);
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
                    tirar_da_fila(u(0xde2f70, 0), p + 4, -1, 0);
                    let r = novo(0x11d);
                    por(p, 0x5b, r);
                    if r == 0 {
                        aviso_em(0xa15708, 0x250);
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
                            let x = indice(0xb74450, h) as i16 as isize;
                            let placar = u(p, 0x5b);
                            if core::ptr::read_unaligned((placar as isize + 5 + 4 * x) as *const u32) == 0 {
                                let x = indice(0xb74450, h) as i16 as isize;
                                core::ptr::write_unaligned((u(p, 0x5b) as isize + 5 + 4 * x) as *mut u32, 1);
                                *((u(p, 0x5b) + 0x105) as *mut u8) += 1;
                            }
                            // ponytail: o indice e de 0 a 15 (os humanos); fora disso a original escreveria na pilha
                            let x = indice(0xb74450, h) as i16;
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
                if indice(0xb74450, h) != 0xffff {
                    let x = indice(0xb74450, h) as i16;
                    if (0..16).contains(&x) && vistos[x as usize] == 0 {
                        fora_do_dia(h, u(u(dia, 0x34), 0x63));
                    }
                }
            }
            redesenhar(0xb74450);
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
                    registrar(u(0xb6b7c0, 0), u(p, 0x57), u(p, 0x5f));
                    sem_motor(p);
                    liberar(u(p, 0x5b));
                    por(p, 0x5b, 0);
                }
                k += 1;
            }
        }

        // as partidas sem humano, uma a uma
        *(0xb67a4c as *mut u32) = 1;
        let mut placar_escrito = false;
        *(0xb67a34 as *mut u32) = 0;
        let hora = relogio();
        let mut k = 0i32;
        while k < ler(0x21) {
            if *(0xdd7118 as *const u32) != 0 {
                ajustar_relogio(2);
                f68d520();
                if *(0xb67a34 as *const u32) != 0 {
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
                        redesenhar(0xb74450);
                    }
                    *(0xb67a34 as *mut u32) = 0;
                }
            }
            let p = partida(k);
            if byte(p, 0x47) == -1 && u(p, 0x57) == 0 {
                let com_humano = humano(u(p, 0x20), 0, b(p, 0x44) as u32, 0, 0) != 0 || humano(u(p, 0x24), 0, b(p, 0x45) as u32, 0, 0) != 0;
                if com_humano && rede() {
                    andar(dia, 1);
                    *(0xb67a4c as *mut u32) = 0;
                    crate::comparacao::ramo("humano em rede recomeca");
                    continue 'reinicio;
                }
                tirar_da_fila(u(0xde2f70, 0), p + 4, -2, 0);
                let m = novo_motor(p);
                if m == 0 {
                    aviso_em(0xa15708, 0x39d);
                } else {
                    let r = rapida(p + 4);
                    crate::comparacao::ramo("partida sem humano");
                    preparar(m, p + 4, r, 0, 0);
                    let rapido = *(u(p, 0x57) as *const i8) != 0;
                    ate_o_fim(p);
                    let resumo = novo_resumo(p);
                    if resumo == 0 {
                        aviso_em(0xa15708, 0x3b5);
                    } else {
                        registrar(u(0xb6b7c0, 0), u(p, 0x57), resumo);
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
                            mostrar(0xb74450, 0xde1f64, -1, 0, -1);
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
        *(0xb67a4c as *mut u32) = 0;
        let mut k = 0i32;
        while k < ler(0x21) {
            let p = partida(k);
            if byte(p, 0x47) != -2 {
                if u(p, 0x5f) == 0 {
                    aviso_em(0xa15708, 0x3fb);
                } else {
                    fechar(dia, p);
                    crate::comparacao::ramo("fechamento");
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
            data_texto(0xae2c90, 0xde1f64, 0x7d0, *(0xae2c88 as *const u8) as u32);
            mostrar(0xb74450, 0xde1f64, -1, 0, -1);
        }
        return 1;
    }
}

// ponytail: o laco do dia cria e libera motores e resumos e mexe na tela; sem comparacao propria,
// validado pela carreira (Rust x original), como as da 151a.
unsafe extern "thiscall" fn comparar_jogar_o_dia(dia: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(335)(dia)
}

// ---------- centesima quinquagesima terceira leva: a montagem do dia de jogos (0x6b3c10, 0x714d30, 0x6d8c70) ----------

/// { 0x6b3c10 } (thiscall, ret 8, match_day.cpp): monta o dia `dia` com as partidas da data `data`
/// no modo `modo` (0, 1 ou 2; fora disso ou sem data, erro 0x93). Zera o dia (+0 modo; vetores +1
/// grupos de 24 bytes, +0xd competicoes de 0x54, +0x19 partidas de 0x69, cada um com capacidade e
/// contagem; +0x2f -1, +0x33 1). Para cada uma das [0xb64738] listas (0x5ae6a0): as partidas
/// pendentes (+0x43 <= -1, fora -2) entram no dia (copia de 0x4f bytes; +0x67 -1), as decididas
/// antes (0x923630 bit 0) registradas e marcadas -2 (0x923cc0, 0x5ab510/0x5aa680), as outras
/// avisadas (0x79c7b0); o grupo da lista (0x48dcd0) guarda a faixa das partidas, que e ordenada
/// (qsort com 0x6b6510) e repartida por competicao (+4 da copia), e cada partida aponta a sua
/// (+0x63). Por fim a lista global [0xb67dc8] (0x68d020) e o interesse do dia (0x6b5040). Os vetores
/// crescem de 5 em 5 (0x945667; sem memoria, erros 0xd6, 0xf4, 0x140). Devolve `dia`.
unsafe extern "thiscall" fn montar_o_dia(dia: usize, data: usize, modo: u32) -> usize {
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ler = |o: usize| core::ptr::read_unaligned((dia + o) as *const i32);
    let por = |p: usize, o: usize, v: u32| core::ptr::write_unaligned((p + o) as *mut u32, v);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let preparar_historia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7e6a70usize);
    let lista_do_dia: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let decidida: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x923630usize);
    let marcar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x923cc0usize);
    let gravar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(0x5ab510usize);
    let juntar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x5aa680usize);
    let avisar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x79c7b0usize);
    let grupo_novo: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x48dcd0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interesse: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6b5040usize);
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
    *(0xb67dc8 as *mut usize) = 0;
    if data == 0 || modo8 > 2 {
        aviso_em(0xa15708, 0x93);
        return dia;
    }
    preparar_historia(u(0xb74434, 0));
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
    while i < *(0xb64738 as *const i8) as i32 {
        let lista = lista_do_dia(u(0xae2a58, 0), data, modo, i as u32);
        if lista != 0 && *(lista as *const i16) > 0 {
            por(lista, 0xa, u(lista, 2) as u32);
            let mut grupo = 0usize;
            let mut x = proximo(lista);
            while x != 0 {
                let e = byte(x, 0x43);
                if e <= -1 && e != -2 {
                    let r = decidida(u(0xde2f74, 0), x);
                    if r & 1 != 0 {
                        crate::comparacao::ramo("montagem decidida antes");
                        let mut copia = [0u8; 0x100];
                        core::ptr::copy(x as *const u8, copia.as_mut_ptr(), 0x4f);
                        *((x + 0x43) as *mut u8) = 0xfe;
                        // ponytail: o 2o argumento vai so com o byte; a original passa sobra da pilha nos altos (lidos como byte)
                        marcar(u(0xde2f74, 0), x, r as u32);
                        gravar(u(0xae2a58, 0), copia.as_mut_ptr());
                        juntar(u(0xae2a58, 0), copia.as_mut_ptr(), 1);
                    } else {
                        avisar(u(0xb74340, 0), x, x + 0x2e);
                        crate::comparacao::ramo("montagem avisada");
                    }
                    if grupo == 0 {
                        crate::comparacao::ramo("montagem grupo novo");
                        if !crescer(1, 24) {
                            aviso_em(0xa15708, 0xd6);
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
                        aviso_em(0xa15708, 0xf4);
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
                            aviso_em(0xa15708, 0x140);
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
    *(0xb67dc8 as *mut usize) = if mem != 0 { lista_nova(mem) } else { 0 };
    interesse(dia);
    dia
}

/// { 0x714d30 } (thiscall, match_pl.cpp): o lado do jogador `j` (+0x27).
unsafe extern "thiscall" fn lado_do_jogador(j: usize) -> u8 {
    *((j + 0x27) as *const u8)
}

/// { 0x6d8c70 } (thiscall, ret 4, match_events.cpp): os sons dos eventos `ativos` (16 marcas) ja
/// tocaram? Para cada marcado, o canal (+0x10 + k; -1 livre, -2 espera) que ainda toca (0x8029f0 em
/// 0xb74450) segura; o que acabou e parado (0x802a50) e liberado. Devolve 1 so se todos os marcados
/// ja passaram da janela (+0x2d6c + 4k ate + [+0x2dac], pelo relogio 0x90c510).
unsafe extern "thiscall" fn sons_terminados(e: usize, ativos: *const u32) -> u8 {
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
    let tocando: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x8029f0usize);
    let parar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x802a50usize);
    let agora = relogio();
    crate::comparacao::ramo("sons dos eventos");
    for k in 0..16usize {
        if *ativos.add(k) == 0 {
            continue;
        }
        let ini = *((e + 0x2d6c + 4 * k) as *const u32);
        let fim = (*((e + 0x2dac) as *const u32)).wrapping_add(ini);
        let passou = agora > fim || agora < ini;
        let canal = (e + 0x10 + k) as *mut u8;
        if *canal != 0xff {
            if *canal == 0xfe {
                return 0;
            }
            if tocando(0xb74450, (ini & !0xff) | *canal as u32) != 0 {
                return 0;
            }
            parar(0xb74450, *canal as u32);
            *canal = 0xff;
        }
        if !passou {
            return 0;
        }
    }
    1
}

// ponytail: a montagem do dia aloca os vetores e chama rotinas ainda originais; o lado e os sons sao
// so leitura e tela. Sem comparacao propria, validadas pela carreira (Rust x original).
unsafe extern "thiscall" fn comparar_montar_o_dia(dia: usize, data: usize, modo: u32) -> usize {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(336)(dia, data, modo)
}
unsafe extern "thiscall" fn comparar_lado_do_jogador(j: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize) -> u8>(337)(j)
}
unsafe extern "thiscall" fn comparar_sons_terminados(e: usize, ativos: *const u32) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, *const u32) -> u8>(338)(e, ativos)
}

// ---------- centesima trigesima quarta leva: o interesse do humano no dia de jogos (0x6b5040) ----------

/// As regioes que a 0x6b5040 marca no dia `d`: +0x29 a +0x2e do dia, +0x14 de cada grupo, +0xc a
/// +0x13 de cada competicao e +0x53 de cada partida.
unsafe fn marcas_do_dia(d: usize) -> Vec<(usize, usize)> {
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

/// { 0x6b5040 } (thiscall, match_day.cpp): marca no dia de jogos `d` o que interessa ao humano.
/// Primeiro a lista dos humanos (0x5fe8d0 em [0xb63c98]); zera +0x29, +0x2a e +0x2b. Para cada grupo
/// (+1, de 0x18 em 0x18, quantos em +9) e cada competicao dele (+4 a +8; em +0xd, de 0x54 em 0x54):
/// a competicao interessa (+0xc) se um dos 16 humanos no fim da tabela de pessoas tem o clube (+0x39)
/// ou o outro vinculo (+0x24) nela (0x48db80 da competicao em [[0xadadfc] + id x 4]). Cada partida
/// dela (+4 a +8; em +0x19, de 0x69 em 0x69): com clube humano (0x600a50 com +0x20/+0x44 ou
/// +0x24/+0x45) ou pela 0x5fe420 a partida e marcada (+0x53) e conta o grupo (+0x29) e a competicao
/// (+0x2a) na primeira vez (+0x14 do grupo, +0x10 da competicao); senao, pela 0x6b61e0, so a
/// competicao interessa. Qualquer marca liga +0x2b do dia. Devolve 1.
unsafe extern "thiscall" fn interesse_do_dia(d: usize) -> u32 {
    let lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5fe8d0usize);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x48db80usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x600a50usize);
    let f5fe420: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x5fe420usize);
    let f6b61e0: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x6b61e0usize);
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let pr = |a: usize, v: i32| core::ptr::write_unaligned(a as *mut i32, v);
    lista(g(0xb63c98));
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
                let objeto = g(g(0xadadfc) + ler(comp) as usize * 4);
                if (0..16).filter(|k| g((g(0xae23bc) as isize + (ler(0xae2364) + k - 16) as isize * 0x6e) as usize + 0x39) != 0).count() >= 2 {
                    crate::comparacao::ramo("interesse dois humanos com clube");
                }
                for k in 0..16 {
                    let pe = (g(0xae23bc) as isize + (ler(0xae2364) + k - 16) as isize * 0x6e) as usize;
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
                    || f5fe420(g(0xb63c98), m + 4) != 0;
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

unsafe extern "thiscall" fn comparar_interesse_do_dia(d: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(299);
    if gravando_no_topo() {
        return o(d);
    }
    // ponytail: a 0x5fe8d0 (lista dos humanos) roda nas duas; ela refaz a lista do zero
    let reg = marcas_do_dia(d);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(d), || interesse_do_dia(d));
    PLACAR.registrar(igual && r == n, || format!("interesse_do_dia original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- centesima trigesima terceira leva: a partida sem o humano (0x6b66a0) ----------

/// As marcas das selecoes dos tecnicos humanos (0xb67a58 + id), que a 0x6b66a0 poe uma vez
/// (com [0xa15704] ligado): para cada vaga de humano em 0xb63d60 (de 0x48 em 0x48 ate 0xb64738) com
/// +0x11c bit 2 e selecao (+0x71).
unsafe fn marcas_das_selecoes() -> Vec<usize> {
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

/// { 0x6b66a0 } (cdecl, match_day.cpp): 1 se a `partida` nao envolve o humano, pela 0x7f9720. Antes,
/// uma vez ([0xa15704]), marca as selecoes dos humanos em 0xb67a58. Sem [0xdd7118]: 1. Devolve 0 se:
/// algum dos clubes (+0x1c, +0x20) e de humano (0x600a50), ou o clube ligado a ele (0x540a50); a
/// competicao (+0) e uma de duas fixas ([0x9cf7bc], [0x9cf6e4]); e a de [0x9cf958] com um dos dois
/// clubes fixos ([0x9cf2e4], [0x9cf510], linhas de 0x122 em [0xae23a8]) humano (+0x11c bit 2); um dos
/// 16 humanos no fim da tabela de pessoas (+0x5f = 1) treina clube da mesma nacao (+0x57) de um dos
/// dois. Na competicao [0x9cf684] sem clube humano, direto a 0x7f9720. Fora de selecao (0x53b900) com
/// 0x5c2990, ou na competicao [0x9cf6f8]: 0 se o +0x80 dos dois somar mais de 12000. Senao a vaga da
/// 0x48dd70 (pela tabela em 0xb63d74): 0 ou 1 da 0; nas 3 a 7 com a selecao da competicao marcada, 0;
/// vaga de humano com +0x11c bit 2, 0. No fim: 1 se a 0x7f9720 der 0.
unsafe extern "C" fn partida_sem_humano(partida: usize) -> u32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x600a50usize);
    let ligado: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x53b900usize);
    let f5c2990: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x5c2990usize);
    let vaga_de: unsafe extern "C" fn(u32, usize) -> u32 = core::mem::transmute(0x48dd70usize);
    let f7f9720: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x7f9720usize);
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16) as i32;
    if g(0xa15704) != 0 {
        for e in marcas_das_selecoes() {
            crate::comparacao::ramo("sem humano selecao de humano");
            *(e as *mut u8) = 1;
        }
        *(0xa15704 as *mut u32) = 0;
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
    let final_ = || (f7f9720(g(0xb74448), partida) == 0) as u32;
    let comp = ler(partida);
    if comp == ler(0x9cf7bc) || comp == ler(0x9cf6e4) {
        return 0;
    }
    if comp == ler(0x9cf958) {
        let base = g(0xae23a8);
        for k in [ler(0x9cf2e4), ler(0x9cf510)] {
            if *((base as isize + k as isize * 0x122 + 0x11c) as *const u8) & 2 != 0 {
                return 0;
            }
        }
    }
    if comp == ler(0x9cf684) && humano(c1, 1, 1, 0, 0) == 0 && humano(c2, 1, 1, 0, 0) == 0 {
        crate::comparacao::ramo("sem humano competicao direta");
        return final_();
    }
    // os 16 humanos no fim da tabela de pessoas
    let base = g(0xae23bc);
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
    if (selecao(c2) == 0 && f5c2990(g(0xae2c74), ler(partida + 0x14) as u32) != 0) || comp == ler(0x9cf6f8) {
        crate::comparacao::ramo("sem humano pela reputacao");
        return if w(c2 + 0x80) + w(c1 + 0x80) > 0x2ee0 { 0 } else { final_() };
    }
    let mut x = vaga_de(ler(partida + 0x14) as u32, partida) as u8;
    if let Some(k) = (0xb63d74usize..0xb6474c).step_by(0x48).position(|e| *(e as *const u8) == x) {
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

unsafe extern "C" fn comparar_partida_sem_humano(partida: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(298);
    if gravando_no_topo() {
        return o(partida);
    }
    let mut reg = vec![(0xa15704usize, 4usize)];
    if *(0xa15704 as *const usize) != 0 {
        reg.extend(marcas_das_selecoes().into_iter().map(|e| (e, 1)));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(partida), || partida_sem_humano(partida));
    PLACAR.registrar(igual && r == n, || format!("partida_sem_humano original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- centesima trigesima segunda leva: o clube no pos-jogo (0x6b79a0) ----------

/// { 0x6b79a0 } (cdecl, match_day.cpp; chamada duas vezes em 0x6b61a9/0x6b61bd, uma por clube): o
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
unsafe fn clube_do_pos_jogo_com(clube: usize, outro: usize, partida: usize, n: u32, t: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let teste_547d60: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x547d60usize);
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
        *(0xb67a34 as *mut u32) = 0;
        return;
    }
    if g(clube + 0xcf) == 0 {
        return;
    }
    if teste_600a50(clube, 0, 1, 0, 0) == 0 && w(clube + 0x80) < 0x128e && sortear(100) != 0 {
        return;
    }
    crate::comparacao::ramo("clube segue");
    let n = n as u8 as i8;
    if teste_6b7cf0(partida, clube) != 0 && ub(partida + 0x40) != 0 && ub(partida + 0x41) != 0 && fora(clube) == 0 {
        for k in 0..50 {
            let s = g(clube + 0xd7 + 4 * k);
            if s == 0 {
                continue;
            }
            let c = contrato(g(0xae1050), s);
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
        let p = (g(0xae23bc) as isize + id as isize * 0x6e) as usize;
        if g(p + 0x61) == 0 || (p != 0 && ler(p) >= ler(0xae2364)) || byte(p, 0x18) > 0x14 {
            continue;
        }
        let c = contrato(g(0xae1050), p);
        if c == 0 || ler(c + 0x45) != 0 || ub(partida + 0x40) == 0 || ub(partida + 0x41) == 0 {
            continue;
        }
        if ler(partida + 0x14) == 0 || ler(partida + 0x14) != ler(clube + 0x57) || fora(clube) != 0 || byte(p, 0x59) <= 0xf {
            continue;
        }
        crate::comparacao::ramo("clube jovem");
        let r = sortear(5);
        if byte(p, 0x57) <= (r as u8).wrapping_add(10) as i8 || g(p + 0x61) == 0 || w(g(p + 0x61) + 0xb) >= 0x109a {
            continue;
        }
        if teste_7c3a40(g(0xb74350), p, clube) == 0 {
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

/// { 0x6b74a0 }: ver `eventos_do_pos_jogo_com`.
unsafe extern "C" fn eventos_do_pos_jogo(partida: usize) {
    eventos_do_pos_jogo_com(partida, &mut |r, t, a| despachar_pos(r, t, a))
}

/// { 0x6b6cc0 }: ver `jogadores_do_pos_jogo_com`.
unsafe extern "C" fn jogadores_do_pos_jogo(partida: usize, t: usize) {
    jogadores_do_pos_jogo_com(partida, t, &mut |r, tt, a| despachar_pos(r, tt, a))
}

/// { 0x6b79a0 }: ver `clube_do_pos_jogo_com`.
unsafe extern "C" fn clube_do_pos_jogo(clube: usize, outro: usize, partida: usize, n: u32, t: usize) {
    clube_do_pos_jogo_com(clube, outro, partida, n, t, &mut |r, tt, a| despachar_pos(r, tt, a))
}

/// As rotinas do banco com efeitos chamadas pelo pos-jogo, pelo endereco do jogo (`this` 0: cdecl).
unsafe fn despachar_pos(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
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
// comparacoes do pos-jogo (GRAVAR_POS); para os outros chamadores do jogo so repassam. Devolvem
// edx:eax inteiro (nenhuma devolve ponto flutuante: conferido na 132a leva).
static GRAVAR_POS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static mut PORTAS_POS: [usize; 15] = [0; 15];
const PROLOGOS_POS: [Prologo; 15] = [
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

/// Dentro de uma comparacao do pos-jogo, com a fita aberta.
fn gravando_pos() -> bool {
    GRAVAR_POS.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// Repassa ou grava (durante uma comparacao do pos-jogo, no nivel de cima da fita).
unsafe fn passar_pos(rotina: usize, this: usize, a: [u32; 5], f: impl FnOnce() -> u64) -> u64 {
    use std::sync::atomic::Ordering::Relaxed;
    if GRAVAR_POS.load(Relaxed) && gravando() {
        let mut r = 0u64;
        chamar_gravando(rotina, this, a, || {
            r = f();
            r as u32
        });
        return r;
    }
    f()
}

unsafe extern "thiscall" fn porta_8676d0(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[0]);
    passar_pos(0x8676d0, c, [a, 0, 0, 0, 0], || f(c, a))
}
unsafe extern "thiscall" fn porta_8674e0(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[1]);
    passar_pos(0x8674e0, c, [a, 0, 0, 0, 0], || f(c, a))
}
unsafe extern "thiscall" fn porta_867560(c: usize, a: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(PORTAS_POS[2]);
    passar_pos(0x867560, c, [a, 0, 0, 0, 0], || f(c, a))
}
unsafe extern "thiscall" fn porta_867700(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[3]);
    passar_pos(0x867700, c, [a, b, 0, 0, 0], || f(c, a, b))
}
unsafe extern "thiscall" fn porta_733a60(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[4]);
    passar_pos(0x733a60, c, [a, b, 0, 0, 0], || f(c, a, b))
}
unsafe extern "thiscall" fn porta_5afe00(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[5]);
    passar_pos(0x5afe00, c, [a, b, 0, 0, 0], || f(c, a, b))
}
unsafe extern "thiscall" fn porta_759610(c: usize, a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[6]);
    passar_pos(0x759610, c, [a, b, d, 0, 0], || f(c, a, b, d))
}
unsafe extern "thiscall" fn porta_744240(c: usize, a: u32, b: u32, d: u32, e: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[7]);
    passar_pos(0x744240, c, [a, b, d, e, 0], || f(c, a, b, d, e))
}
unsafe extern "thiscall" fn porta_72c1f0(c: usize, a: u32, b: u32, d: u32, e: u32, g: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[8]);
    passar_pos(0x72c1f0, c, [a, b, d, e, g], || f(c, a, b, d, e, g))
}
unsafe extern "C" fn porta_7daf50(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[9]);
    passar_pos(0x7daf50, 0, [a, b, d, 0, 0], || f(a, b, d))
}
unsafe extern "C" fn porta_547af0(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[10]);
    passar_pos(0x547af0, 0, [a, b, d, 0, 0], || f(a, b, d))
}
unsafe extern "C" fn porta_5896d0(a: u32, b: u32, d: u32, e: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[11]);
    passar_pos(0x5896d0, 0, [a, b, d, e, 0], || f(a, b, d, e))
}
unsafe extern "C" fn porta_589bc0(a: u32, b: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[12]);
    passar_pos(0x589bc0, 0, [a, b, 0, 0, 0], || f(a, b))
}
unsafe extern "C" fn porta_547d90(a: u32, b: u32, d: u32) -> u64 {
    let f: unsafe extern "C" fn(u32, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[13]);
    passar_pos(0x547d90, 0, [a, b, d, 0, 0], || f(a, b, d))
}

unsafe extern "thiscall" fn porta_7c3a40(c: usize, a: u32, b: u32) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u64 = core::mem::transmute(PORTAS_POS[14]);
    passar_pos(0x7c3a40, c, [a, b, 0, 0, 0], || f(c, a, b))
}

fn portas_pos() -> [usize; 15] {
    [
        porta_8676d0 as *const () as usize, porta_8674e0 as *const () as usize, porta_867560 as *const () as usize,
        porta_867700 as *const () as usize, porta_733a60 as *const () as usize, porta_5afe00 as *const () as usize,
        porta_759610 as *const () as usize, porta_744240 as *const () as usize, porta_72c1f0 as *const () as usize,
        porta_7daf50 as *const () as usize, porta_547af0 as *const () as usize, porta_5896d0 as *const () as usize,
        porta_589bc0 as *const () as usize, porta_547d90 as *const () as usize, porta_7c3a40 as *const () as usize,
    ]
}

/// Roda a comparacao de uma rotina do pos-jogo pela fita: as 14 do banco entram como unidades.
unsafe fn comparar_pos(nome: &str, reg: Vec<(usize, usize)>, original: impl FnOnce(), rust: impl FnOnce(&mut dyn FnMut(usize, usize, [u32; 5]) -> u32)) {
    use std::sync::atomic::Ordering::Relaxed;
    let gravar = GRAVAR_POS.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, original, |chamar| rust(&mut |r, t, a| chamar(r, t, a, [u32::MAX; 5])));
    GRAVAR_POS.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("{} ({} chamadas): {}", nome, k, detalhe_fita()));
}

/// As flags do contrato (+0x40 a +0x48) de `pessoa` no `clube`, como regiao.
unsafe fn regiao_do_contrato(pessoa: usize, clube: usize) -> Option<(usize, usize)> {
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa, clube);
    (c != 0).then_some((c + 0x40, 9))
}

unsafe extern "C" fn comparar_eventos_do_pos_jogo(partida: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(295);
    // o cursor da lista, as linhas dos dois clubes em [[0xb67a44]] e as flags dos contratos dos eventos
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let lista = *(0xb67dc8 as *const usize);
    if lista != 0 {
        reg.push((lista, 0xe));
        let pessoa = |id: i32| (*(0xae23bc as *const usize) as isize + id as isize * 0x6e) as usize;
        let mut no = core::ptr::read_unaligned((lista + 2) as *const usize);
        let mut k = 0;
        while no != 0 && k < core::ptr::read_unaligned(lista as *const i16) as i32 {
            let ev = core::ptr::read_unaligned(no as *const usize);
            no = core::ptr::read_unaligned((no + 6) as *const usize);
            k += 1;
            if ev == 0 {
                break;
            }
            let clube = (*(0xae23b4 as *const usize) as isize + core::ptr::read_unaligned((ev + 4) as *const i32) as isize * 0x245) as usize;
            if clube == *((partida + 0x1c) as *const usize) || clube == *((partida + 0x20) as *const usize) {
                reg.extend(regiao_do_contrato(pessoa(core::ptr::read_unaligned(ev as *const i32)), clube));
            }
        }
    }
    let tabela = *(0xb67a44 as *const usize);
    if tabela != 0 {
        for o in [0x1c, 0x20] {
            let clube = *((partida + o) as *const usize);
            if clube != 0 {
                reg.push(((*(tabela as *const usize) as isize + *(clube as *const i32) as isize * 0x49) as usize, 0x49));
            }
        }
    }
    comparar_pos("eventos_do_pos_jogo", reg, || o(partida), |chamar| eventos_do_pos_jogo_com(partida, chamar));
}

unsafe extern "C" fn comparar_jogadores_do_pos_jogo(partida: usize, t: usize) {
    let o = orig::<unsafe extern "C" fn(usize, usize)>(296);
    // as fichas dos jogadores (+0x45, +9 a +0xe), os contratos, +0x80 dos clubes e as fichas do
    // tecnico e dos auxiliares (+8 a +0xd)
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for lado in 0..2usize {
        let clube = (g(0xae23b4) as isize + ler(t + 4 * lado) as isize * 0x245) as usize;
        let ts = t + lado * 0x625;
        reg.push((clube + 0x80, 2));
        for k in 0..20usize {
            let id = ler(ts + 0x1c5 + k * 0x49 - 0x35);
            if id == -1 {
                continue;
            }
            let p = (g(0xae23bc) as isize + id as isize * 0x6e) as usize;
            let f = g(p + 0x61);
            if f != 0 {
                reg.extend([(f + 0x45, 1), (f + 9, 6)]);
                reg.extend(regiao_do_contrato(p, clube));
            }
        }
        let id = ler(ts + 0x184);
        if id > -1 {
            let tec = (g(0xae23bc) as isize + id as isize * 0x6e) as usize;
            let mut fichas = vec![g(tec + 0x69)];
            for a in [g(clube + 0xd3), g(clube + 0x19f), g(clube + 0x1a3), g(clube + 0x1a7), g(clube + 0x1ab), g(clube + 0x1af)] {
                if a != 0 {
                    fichas.push(g(a + 0x69));
                }
            }
            for f in fichas.into_iter().filter(|&f| f != 0) {
                reg.push((f + 8, 6));
            }
        }
    }
    comparar_pos("jogadores_do_pos_jogo", reg, || o(partida, t), |chamar| jogadores_do_pos_jogo_com(partida, t, chamar));
}

unsafe extern "C" fn comparar_clube_do_pos_jogo(clube: usize, outro: usize, partida: usize, n: u32, t: usize) {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, u32, usize)>(297);
    // a copia nao escreve no banco: so as chamadas e o sorteio
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    comparar_pos("clube_do_pos_jogo", reg, || o(clube, outro, partida, n, t), |chamar| clube_do_pos_jogo_com(clube, outro, partida, n, t, chamar));
}

// ---------- 215a leva: o fim das telas da partida: a partida que o humano deixa e a ficha do jogo de um registro de estatística (match_day.cpp, match_screens.cpp) ----------

/// { 0x6b6970 } (cdecl, match_day.cpp): o humano da vez ([0xb77c7a]) deixa a partida que assistia
/// (chamada só quando um jogador da rede sai, 0x838c00). A partida dele ([0xb74c26 + 0x300 × vez],
/// 0x765480 → +0xe2): se outro dos 16 humanos (as últimas 16 fichas de [0xae23bc], 110 bytes, +0x5f
/// ativo) assiste a mesma e é a vez dele (0x7fffa0 no gerente), sai calado; senão tira a vez dos
/// quatro vetores do resumo (+0x5b: +5/+0x45/+0x85/+0xc5, os contadores +0x105..+0x108). Ninguém mais
/// assistindo: cria o motor (+0x57, 0xf63b bytes, 0x6b7e50 + 0x6b7f20; sem ele erro 0xa4b), anda até
/// o fim (0x6b98c0 com −1; estado +0x67, 2 ou 8 contam em [[0xb67a50]+0x25]), fora do 8 cria o resumo
/// (+0x5f, 0xd7a bytes com 2 objetos de 0x625 em +0x11f; sem ele erro 0xa68) e registra (0x7293f0),
/// libera o motor e tira a partida da fila [0xde2f70] (0x90d6b0).
unsafe extern "C" fn largar_partida_assistida() {
    crate::comparacao::ramo("telas largar a partida assistida");
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x7fffa0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x6b7e50usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x6b7f20usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x6b98c0usize);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7293f0usize);
    let destruir_motor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6b97a0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d6b0usize);
    let vez = || *(0xb77c7a as *const i16) as isize;

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
        let idx = (*(0xae2364 as *const usize)).wrapping_add(i).wrapping_sub(0x10);
        let q = u(0xae23bc, 0).wrapping_add(idx.wrapping_mul(110));
        // como a original, sem conferir a ficha nula do outro humano
        if *((q + 0x5f) as *const u8) != 0 && q != p && u(ficha(q), 0xe2) == jogo {
            let minha = *(0xb77c7a as *const u16);
            if indice(0xb74450, q) == minha {
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
            aviso_em(0xa15708, 0xa4b);
            return;
        }
        preparar(m, jogo + 4, 0, 0, 0);
    }
    let estado = (jogo + 0x67) as *mut u8;
    while *estado != 2 && *estado != 8 {
        let e = laco(core::ptr::read_unaligned(motor), -1) as u8;
        *estado = e;
        if e == 2 || e == 8 {
            *((u(0xb67a50, 0) + 0x25) as *mut i32) += 1;
        }
    }
    if *estado != 8 {
        let mem = novo(0xd7a);
        if mem != 0 {
            vetor(mem + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        core::ptr::write_unaligned((jogo + 0x5f) as *mut usize, mem);
        if mem == 0 {
            aviso_em(0xa15708, 0xa68);
            return;
        }
        registrar(u(0xb6b7c0, 0), core::ptr::read_unaligned(motor), mem);
    }
    let m = core::ptr::read_unaligned(motor);
    if m != 0 {
        destruir_motor(m);
        liberar(m);
    }
    core::ptr::write_unaligned(motor, 0);
    tirar(u(0xde2f70, 0), jogo + 4);
}

/// { 0x729e30 } (thiscall, ret 8, match_stats.cpp): a ficha do jogo (0x4f bytes) do registro `k` do
/// índice de estatísticas ([this], 12 bytes: +0 competição, +4 um dos clubes, +8/+0xa a data): pede
/// à agenda [0xae2a58] (0x5acbe0) os jogos daquela data e competição numa lista local (0x68d020) e copia
/// para `saida` o primeiro com o clube +4 na casa (+0xc) ou fora (+0x10). Devolve 1 se achou. Índice
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro, 0).
unsafe extern "thiscall" fn ficha_do_registro(estat: usize, k: i32, saida: usize) -> u32 {
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(544)(estat, k, saida);
    }
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    // ponytail: a lista nasce zerada aqui, que é tudo o que o construtor 0x68d020 faz
    let mut lista = [0u8; 14];
    let l = lista.as_mut_ptr() as usize;
    let tab = *(estat as *const usize);
    let e = tab.wrapping_add((k as usize).wrapping_mul(12));
    // a original empurra os registradores inteiros: em cima, os 16 bits de [this] e de k × 3
    let d1 = (tab as u32 & 0xffff_0000) | *((e + 8) as *const u16) as u32;
    let d2 = ((k as u32).wrapping_mul(3) & 0xffff_0000) | *((e + 0xa) as *const u16) as u32;
    jogos(*(0xae2a58 as *const usize), d1, d2, d1, d2, *(e as *const u32), l, u32::MAX, 0xffff_fffe, u32::MAX);
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
unsafe extern "C" fn comparar_largar_partida_assistida() {
    orig::<unsafe extern "C" fn()>(543)()
}

unsafe extern "thiscall" fn comparar_ficha_do_registro(estat: usize, k: i32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(544);
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return o(estat, k, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 0x4f)], || o(estat, k, saida), || ficha_do_registro(estat, k, saida));
    PLACAR.registrar(igual && r == n, || format!("ficha_do_registro(k {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

// ---------- 170a leva: o fechamento do motor (4): as ordens das partidas e dos jogadores e a troca do evento ----------

/// { 0x6b6510 } (cdecl, qsort): a ordem das partidas do dia (registros de 0x69 bytes): pela
/// competição (+4), depois +0x46 (byte), +0x36, +0x38 e +0x34 (palavras) e, empatando tudo, pelo nome
/// do primeiro clube (+0x20 → +0x38) na comparação de textos 0x90c1d0 (sem maiúsculas).
unsafe extern "C" fn ordem_das_partidas_do_dia(a: usize, b: usize) -> i32 {
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
unsafe extern "C" fn ordem_de_acao(a: *const usize, b: *const usize) -> i32 {
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
        ftol(s as f64 * (w * f64_em(0x968ef0) + f64_em(0x968968)) * core::ptr::read_unaligned((p + 0x4d) as *const f32) as f64)
    };
    forca(pb).wrapping_sub(forca(pa))
}

/// { 0x6d5d80 } (stdcall, ret 0xc, match_events.cpp): troca o evento da narração por uma variante:
/// com sorteio(3) ≠ 0 fica o mesmo; senão, pelo tipo do lance `tipo` (1, 2, 8, 0x10, 0x20 — este com
/// outro sorteio(3)) há tabelas de troca; os lances 2..4 (byte baixo de `lados`) têm as suas, e o
/// byte de cima 3..5 troca 0x1f4b por 0x217e.
unsafe extern "system" fn variante_do_evento(e: *const i16, lados: u32, tipo: u8) -> i16 {
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

unsafe extern "C" fn comparar_ordem_das_partidas_do_dia(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(382)(a, b);
    let n = ordem_das_partidas_do_dia(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_partidas_do_dia({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

unsafe extern "C" fn comparar_ordem_de_acao(a: *const usize, b: *const usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(383)(a, b);
    let n = ordem_de_acao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_de_acao({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

unsafe extern "system" fn comparar_variante_do_evento(e: *const i16, lados: u32, tipo: u8) -> i16 {
    let o = orig::<unsafe extern "system" fn(*const i16, u32, u8) -> i16>(384);
    if gravando_no_topo() {
        return o(e, lados, tipo);
    }
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(e, lados, tipo), || crate::aleatorio::sem_contar(|| variante_do_evento(e, lados, tipo)));
    PLACAR.registrar(igual && r == n, || format!("variante_do_evento({:#x}, {:#x}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", *e, lados, tipo, r, n, mo, mr));
    r
}

