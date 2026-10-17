// ---------- centesima quinquagesima sexta leva: o texto dos eventos e o comeco da partida assistida (0x6d8d10, 0x6b5260) ----------

/// { 0x6d8d10 } (stdcall, ret 0x10, match_events.cpp): o texto do evento `ev` (+0 codigo, +2 qual
/// som, +5/+6 e +7/+8 os dois jogadores (lado, indice), +9 um numero) da partida `ctx` (+0/+4 os
/// clubes, +0x40 o arbitro), com `comp` a competicao (-1 sem) e `variante` o trecho entre `|` (0 o
/// primeiro). Le o texto `R` (+0x10c) do som escolhido (no maximo o ultimo) e troca as marcas:
/// `<P1>`/`<P>` jogador (0x6db2d0), `<T1>`/`<T2>` clube, `<C>` competicao, `<F>` a fase dela
/// (0x4bfa30/0x4bff70), `<D>`/`<D1>` o numero (+1), `<N>` o numero por extenso (0x90c310), `<rf>`/`<Rf>`
/// o arbitro ("Mr ..." ou "the referee" conforme o idioma e um resto de divisao por 5), `<O>`/`<Y>` nada.
/// `{...}` antes ou depois da marca sao prefixo e sufixo (0x6701f0, que aplica artigo e genero);
/// a letra seguinte `C` poe em maiusculas (0x90c2d0). Escreve em [0xb67ea0] (e devolve), recontando
/// o tamanho por `strlen` depois de cada marca, como a original; sem evento, sem texto ou sem o
/// trecho pedido, 0. Marca desconhecida avisa 0xb10.
unsafe extern "stdcall" fn texto_do_evento(ev: usize, ctx: usize, comp: i32, variante: u32) -> usize {
    const BUF: usize = 0xb67ea0;
    const TEMP: usize = 0xb67dd8;
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x6db2d0usize);
    let compor: unsafe extern "C" fn(usize, *const u8, *const u8, u32, u32) -> usize = core::mem::transmute(0x6701f0usize);
    let maiuscula: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x90c2d0usize);
    let por_extenso: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(0x90c310usize);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let nome_do_arbitro: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x53eab0usize);
    let fase: unsafe extern "C" fn(usize, u32, u32, u32, *mut u8, u32) = core::mem::transmute(0x4bfa30usize);
    let criterio: unsafe extern "C" fn(u32, u32, u32, u32) -> u8 = core::mem::transmute(0x4bff70usize);
    if ev == 0 || ctx == 0 {
        return 0;
    }
    let k = (((*(ev as *const u16) & 0x7fff) as i32 - 0x1f40) as i16) as isize;
    let e = (0xb68788isize + k * 18) as usize;
    let sons = core::ptr::read_unaligned((e + 8) as *const usize);
    if sons == 0 {
        return 0;
    }
    let n = core::ptr::read_unaligned((e + 0xc) as *const i16);
    let qual = (ev + 2) as *mut i16;
    if *qual as i32 > n as i32 - 1 {
        *qual = n.wrapping_sub(1);
    }
    let mut s = core::ptr::read_unaligned((sons as isize + *qual as isize * 0x110 + 0x10c) as *const usize);
    if s == 0 {
        return 0;
    }
    let v = variante as u16 as i16;
    if v > 0 {
        let mut resta = v as u8 as i8;
        let mut c = *(s as *const u8);
        if c != 0 {
            loop {
                if c == b'|' {
                    resta = resta.wrapping_sub(1);
                    if resta <= 0 {
                        s += 1;
                        break;
                    }
                }
                c = *((s + 1) as *const u8);
                s += 1;
                if c == 0 {
                    break;
                }
            }
        }
        if *(s as *const u8) == 0 {
            return 0;
        }
    }
    crate::comparacao::ramo("texto do evento");
    let t = |i: i16| *((s as isize + i as isize) as *const u8);
    let dest = |o: i16| (BUF as isize + o as isize) as usize;
    let b = |o: usize| *((ev + o) as *const u8);
    let mut pos: i16 = 0;
    let mut out: i16 = 0;
    // o prefixo `{...}` logo antes da marca
    let prefixo = |pos: i16, p: &mut [u8; 0x100]| {
        p[0] = 0;
        if pos > 1 && (t(pos - 1) == b'}' || t(pos - 1) == b']') {
            let mut a = pos - 2;
            while a >= 0 && t(a) != b'{' && t(a) != b'[' {
                a -= 1;
            }
            let j = a as i32;
            let fim = pos as i32 - j - 1;
            let mut c = 1i32;
            while c < fim {
                p[(c - 1) as usize] = t((j + c) as i16);
                c += 1;
            }
            p[(c - 1) as usize] = 0;
        }
    };
    // o sufixo `{...}` logo depois do `>` da marca (a procura comeca em `inicio`)
    let sufixo = |inicio: i16, x: &mut [u8; 0x100]| {
        x[0] = 0;
        let mut fe = inicio;
        while t(fe) != b'>' && t(fe) != 0 {
            fe += 1;
        }
        if t(fe + 1) == b'{' || t(fe + 1) == b'[' {
            let mut j = fe + 2;
            while t(j) != b'}' && t(j) != 0 {
                x[(j - fe - 2) as usize] = t(j);
                j += 1;
            }
            x[(j - fe - 2) as usize] = 0;
        }
    };
    let escrever = |out: i16, r: usize| {
        formatar(dest(out), 0x991944, r);
    };
    let em_maiusculas = |out: i16, r: usize| {
        let mut j: i16 = 0;
        while *((r as isize + j as isize) as *const u8) != 0 {
            let c = maiuscula(*((r as isize + j as isize) as *const u8) as i32);
            *(dest(out + j) as *mut u8) = c;
            j += 1;
        }
        *(dest(out + j) as *mut u8) = 0;
    };
    let mut pre = [0u8; 0x100];
    let mut suf = [0u8; 0x100];
    let mut nome = [0u8; 0xd0];
    let mut arb = [0u8; 0xd0];
    let mut texto_fase = [0u8; 0x800];

    if t(0) != 0 {
        loop {
            let c = t(pos);
            if c == b'|' {
                break;
            }
            if c != b'<' {
                if c != b'{' && c != b'[' {
                    *(dest(out) as *mut u8) = c;
                    out += 1;
                } else {
                    // `{...}`/`[...]` solto: pula ate o fecho
                    let mut j = pos;
                    let mut a = c;
                    loop {
                        if a == b']' || a == 0 {
                            break;
                        }
                        j += 1;
                        a = t(j);
                        if a == b'}' {
                            break;
                        }
                    }
                    pos = j;
                }
            } else {
                let x = t(pos + 1) as i8 as i32;
                match x as u8 {
                    b'P' | b'p' => {
                        let (lado, idx) = if t(pos + 2) == b'1' && b(5) != 0xff { (b(5), b(6)) } else { (b(7), b(8)) };
                        if lado != 0xff {
                            nome_do_jogador(ctx, lado as i8 as i32, idx as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 1);
                            sufixo(pos + 3, &mut suf);
                            let r = compor(nome.as_ptr() as usize, 0xad9c64 as *const u8, suf.as_ptr(), 0x64, 0x64);
                            if t(pos + 3) == b'C' || t(pos + 3) == b'c' {
                                em_maiusculas(out, r);
                            } else {
                                escrever(out, r);
                            }
                        }
                    }
                    b'T' | b't' => {
                        let dois = t(pos + 2) == b'2';
                        let casa = b(6) != 0;
                        let id = if dois == casa { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                        let clube = (*(0xae23b4 as *const usize)).wrapping_add(id.wrapping_mul(0x245));
                        prefixo(pos, &mut pre);
                        sufixo(pos + 2, &mut suf);
                        let g = *((clube + 0x52) as *const u8) as u32;
                        let r = compor(clube + 0x38, pre.as_ptr(), suf.as_ptr(), g, g);
                        if t(pos + 2) == b'C' || t(pos + 2) == b'c' {
                            em_maiusculas(out, r);
                        } else {
                            escrever(out, r);
                        }
                    }
                    b'C' | b'c' | b'F' | b'f' => {
                        if comp >= 0 {
                            let cp = (*(0xae23d0 as *const usize)).wrapping_add((comp as usize).wrapping_mul(0x6b));
                            prefixo(pos, &mut pre);
                            sufixo(pos + 2, &mut suf);
                            let r = if x as u8 == b'C' || x as u8 == b'c' {
                                let g = *((cp + 0x52) as *const u8) as u32;
                                compor(cp + 0x38, pre.as_ptr(), suf.as_ptr(), g, g)
                            } else {
                                fase(cp, 0, 0x96, 3, texto_fase.as_mut_ptr(), 0x7d0);
                                let g = criterio(0, 0, 0x96, 3) as u32;
                                compor(texto_fase.as_ptr() as usize, pre.as_ptr(), suf.as_ptr(), g, g)
                            };
                            escrever(out, r);
                        }
                    }
                    b'D' | b'd' => {
                        let numero = core::ptr::read_unaligned((ev + 9) as *const i32);
                        formatar(dest(out), 0x98806c, if t(pos + 2) == b'1' { numero.wrapping_add(1) } else { numero });
                    }
                    b'N' | b'n' => {
                        let r = por_extenso(core::ptr::read_unaligned((ev + 9) as *const i32), -1);
                        escrever(out, r);
                    }
                    b'R' | b'r' => {
                        if t(pos + 2) == b'f' {
                            let fixo = if x as u8 == b'R' { 0xa15a14usize } else { 0xa159e0 };
                            let arbitro = core::ptr::read_unaligned((ctx + 0x40) as *const i32);
                            if arbitro <= -1 {
                                formatar_texto(dest(out), fixo);
                            } else {
                                let resto = ((b(7) as i8 as i32 + b(5) as i8 as i32 + core::ptr::read_unaligned((ctx + 4) as *const i32)) % 5) as i8;
                                let ficha = (*(0xae23e8 as *const usize)).wrapping_add((arbitro as usize).wrapping_mul(0x2b));
                                let modo = if idioma() == 4 && resto == 0 {
                                    1
                                } else if resto == 0 {
                                    5
                                } else if resto > 0 && resto <= 2 {
                                    1
                                } else {
                                    0
                                };
                                match modo {
                                    1 => {
                                        nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 1);
                                        formatar_texto(TEMP, 0xa15a20, arb.as_ptr());
                                    }
                                    5 => {
                                        nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 5);
                                        formatar_texto(TEMP, 0xa159ec, arb.as_ptr());
                                    }
                                    _ => {
                                        formatar_texto(TEMP, if x as u8 == b'R' { 0xa15a14 } else { 0xa159e0 });
                                    }
                                }
                                // como na original: o texto pronto vira o formato
                                formatar(dest(out), TEMP);
                            }
                        }
                    }
                    b'O' | b'o' | b'Y' | b'y' => {}
                    _ => aviso_em(0xa159b0, 0xb10),
                }
                // fim da marca: ate o `>`; o tamanho e recontado
                if t(pos) != b'>' {
                    let mut j = pos;
                    loop {
                        if t(j) == 0 {
                            break;
                        }
                        j += 1;
                        if t(j) == b'>' {
                            break;
                        }
                    }
                    pos = j;
                }
                if t(pos) == 0 {
                    pos -= 1;
                }
                let mut l = 0usize;
                while *((BUF + l) as *const u8) != 0 {
                    l += 1;
                }
                out = l as i16;
            }
            pos += 1;
            if t(pos) == 0 {
                break;
            }
        }
    }
    *(dest(out) as *mut u8) = 0;
    BUF
}

/// { 0x6b5260 } (thiscall, ret 4, match_day.cpp): o humano da vez ([0xb77c7a]) confirma a partida
/// `p` em rede (placar dos humanos +0x5b: +0x45 quem confirmou, +0x105 humanos, +0x106 confirmados).
/// Com todos: cria o motor (0x6b7e50) e o prepara para ver (0x6b7f20 com o ultimo 1). Se ele nao
/// for de ver (devolve 0), joga ate o fim (contando em [0xb67a50]+0x25), cria o resumo, registra
/// (0x7293f0), libera o motor, tira da fila e marca +0x119; se for, avisa o narrador (+0x8f7a = 3),
/// cria o resumo e registra para ver (0x729ff0) e devolve 1. Erros 0x4b8 (sem partida), 0x4d0, 0x4ea,
/// 0x511 (sem memoria).
unsafe extern "thiscall" fn confirmar_partida_em_rede(_dia: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| core::ptr::read_unaligned((q + o) as *const usize);
    let por = |q: usize, o: usize, v: usize| core::ptr::write_unaligned((q + o) as *mut usize, v);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x6b7e50usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x6b7f20usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x6b98c0usize);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7293f0usize);
    let registrar_para_ver: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x729ff0usize);
    let destruir_motor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6b97a0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x90d6b0usize);
    const FONTE: usize = 0xa15708; // match_day.cpp
    if p == 0 {
        aviso_em(FONTE, 0x4b8);
        return 0;
    }
    let pl = u(p, 0x5b);
    let eu = *(0xb77c7a as *const i16) as isize;
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
        aviso_em(FONTE, 0x4d0);
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
                *((u(0xb67a50, 0) + 0x25) as *mut i32) += 1;
            }
        }
        let r = novo_resumo();
        if r == 0 {
            aviso_em(FONTE, 0x4ea);
            return 0;
        }
        registrar(u(0xb6b7c0, 0), u(p, 0x57), r);
        let m = u(p, 0x57);
        if m != 0 {
            destruir_motor(m);
            liberar(m);
        }
        por(p, 0x57, 0);
        tirar(u(0xde2f70, 0), p + 4);
        por(pl, 0x119, 1);
        return 0;
    }
    let narrador = u(u(p, 0x57), 0x8f7a);
    if narrador != 0 {
        *(narrador as *mut u8) = 3;
    }
    let r = novo_resumo();
    if r == 0 {
        aviso_em(FONTE, 0x511);
        return 0;
    }
    registrar_para_ver(u(0xb6b7c0, 0), u(p, 0x57), r, 1);
    1
}

/// O texto do evento so escreve nos buffers de texto ([0xb67ea0], [0xb67dd8]) e no `qual` do evento:
/// a original e a copia montam o mesmo texto e o resultado e conferido.
unsafe extern "stdcall" fn comparar_texto_do_evento(ev: usize, ctx: usize, comp: i32, variante: u32) -> usize {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, i32, u32) -> usize>(343);
    if ev == 0 {
        return o(ev, ctx, comp, variante);
    }
    let reg = [(ev + 2, 2), (0xb67ea0, 0x400), (0xb67dd8, 0xc8)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(ev, ctx, comp, variante) as u32, || texto_do_evento(ev, ctx, comp, variante) as u32);
    PLACAR.registrar(igual && r == rr, || format!("texto_do_evento(codigo {:#x} variante {}) original={:#x} rust={:#x} {:x?} x {:x?}", *(ev as *const u16), variante, r, rr, mo, mr));
    r as usize
}
// ponytail: so roda em jogo em rede (cria e libera motor e resumo); sem comparacao propria.
unsafe extern "thiscall" fn comparar_confirmar_partida_em_rede(dia: usize, p: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(344)(dia, p)
}
