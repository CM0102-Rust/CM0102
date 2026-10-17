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
