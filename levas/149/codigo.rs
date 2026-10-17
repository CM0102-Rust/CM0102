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
