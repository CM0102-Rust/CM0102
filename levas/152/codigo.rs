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
    let em_rede: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fe290usize);
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
                                    if em_rede(gerente(), p + 4) == 0 {
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
