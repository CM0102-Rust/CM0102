// ---------- centesima quinquagesima quarta leva: os eventos gravados e o passo em rede (0x6d98b0, 0x6d9c40, 0x6b55c0) ----------

/// Os dois arquivos de eventos (humano ou computador) em CM3_TEMP.
fn arquivos_de_eventos(humano: bool) -> (usize, usize) {
    if humano { (0xa15a8c, 0xa15a78) } else { (0xa15a60, 0xa15a48) }
}

/// { 0x6d98b0 } (thiscall, ret 8, match_events.cpp): grava os +6 eventos (0xe bytes cada, em +0x720)
/// do registro `e` num dos dois arquivos "events1/2.tmp" (humano ou computador). Cabecalho: versao 1
/// (senao erro 0xb5c/0xb7b), numero, quantos. Usa o primeiro com menos de 2500; cheios os dois,
/// recria o mais antigo com o numero seguinte de [0xb6b7c0]+0x18 (sem abrir, erro 0xba4). Em `desc`:
/// posicao, quantos, numero e o byte `humano`. Devolve se gravou.
unsafe extern "thiscall" fn gravar_eventos(e: usize, desc: *mut u32, humano: u8) -> u8 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(*mut i32, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(0x946a22usize);
    let posicao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x946aaeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    if desc.is_null() {
        return 0;
    }
    let (um, dois) = arquivos_de_eventos(humano != 0);
    let (mut versao, mut quantos, mut numero) = (0i32, 0i32, 0i32);
    let mut ids = [-1i32, -1];
    let mut aberto = 0usize;
    for (k, nome) in [um, dois].into_iter().enumerate() {
        let f = abrir(caminho(0x987da0, nome), 0x9c7848);
        if f == 0 {
            continue;
        }
        buscar(f, 0, 0);
        ler(&mut versao, 4, 1, f);
        ler(&mut ids[k], 4, 1, f);
        ler(&mut quantos, 4, 1, f);
        numero = ids[k];
        if versao != 1 {
            aviso_em(0xa159b0, if k == 0 { 0xb5c } else { 0xb7b });
            fechar(f);
            return 0;
        }
        if quantos < 0x9c4 {
            crate::comparacao::ramo("eventos arquivo com espaco");
            aberto = f;
            break;
        }
        fechar(f);
    }
    let f = if aberto != 0 {
        aberto
    } else {
        let nome = if ids[0] == -1 || (ids[1] != -1 && ids[0] <= ids[1]) { um } else { dois };
        let f = abrir(caminho(0x987da0, nome), 0x9a7f28);
        if f == 0 {
            aviso_em(0xa159b0, 0xba4);
            return 0;
        }
        versao = 1;
        crate::comparacao::ramo("eventos arquivo novo");
        quantos = 0;
        let r = *(0xb6b7c0 as *const usize) + 0x18;
        numero = *(r as *const i32) + 1;
        *(r as *mut i32) = numero;
        escrever(&versao as *const i32 as usize, 4, 1, f);
        escrever(&numero as *const i32 as usize, 4, 1, f);
        escrever(&quantos as *const i32 as usize, 4, 1, f);
        f
    };
    buscar(f, 0, 2);
    *desc = posicao(f);
    let n = *((e + 6) as *const i16) as i32;
    *desc.add(1) = n as u32;
    *desc.add(2) = numero as u32;
    *(desc.add(3) as *mut u8) = humano;
    let r = escrever(e + 0x720, 0xe, *((e + 6) as *const i16) as i32 as u32, f);
    quantos += 1;
    buscar(f, 8, 0);
    escrever(&quantos as *const i32 as usize, 4, 1, f);
    fechar(f);
    (r != 0) as u8
}

/// { 0x6d9c40 } (thiscall, ret 4, match_events.cpp): le de volta os eventos descritos em `desc`
/// (posicao, quantos, numero, humano) para +0x720 do registro `e` (+6 = quantos): procura o arquivo
/// com o numero (versao 1, senao erro 0xbf6/0xc11, com o arquivo aberto como na original). Devolve
/// se leu.
unsafe extern "thiscall" fn ler_eventos(e: usize, desc: *const u32) -> u8 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(0x946a22usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    if desc.is_null() {
        return 0;
    }
    let (um, dois) = arquivos_de_eventos(*(desc.add(3) as *const u8) != 0);
    let (mut versao, mut numero) = (0i32, 0i32);
    for (k, nome) in [um, dois].into_iter().enumerate() {
        let f = abrir(caminho(0x987da0, nome), 0x98f928);
        if f == 0 {
            if k == 0 {
                continue;
            }
            return 0;
        }
        ler(&mut versao as *mut i32 as usize, 4, 1, f);
        ler(&mut numero as *mut i32 as usize, 4, 1, f);
        if versao != 1 {
            // como na original: o arquivo fica aberto
            aviso_em(0xa159b0, if k == 0 { 0xbf6 } else { 0xc11 });
            return 0;
        }
        if numero as u32 == *desc.add(2) {
            crate::comparacao::ramo("eventos lidos");
            buscar(f, *desc as i32, 0);
            let r = ler(e + 0x720, 0xe, *desc.add(1), f);
            *((e + 6) as *mut u16) = *desc.add(1) as u16;
            fechar(f);
            return (r != 0) as u8;
        }
        fechar(f);
    }
    0
}

/// { 0x6b55c0 } (thiscall, ret 4, match_day.cpp): um passo da partida do humano `p` em rede, pelo
/// relogio (0x90c510) e pelo placar dos humanos (+0x5b: +0 estado, +1 evento, +0x85 quem ja viu,
/// +0x105 humanos, +0x107 vistos, +0x108 pausa, +0x109 espera, +0x10d hora, +0x111 pode andar,
/// +0x115 lance). Sem `p`, anda a atual (0x6b5ce0) e comeca a proxima (0x6b5a70). Espera todos verem
/// o estado; depois os sons (0x6d8c70) e o tempo; mostra o proximo evento (0x6d78b0, lance pela
/// tabela 0x6d5cd0) ou da um passo do motor (0x6b98c0 com 1) e marca o estado: fim (2/8), intervalo
/// (4), gols de cada lado (+0xf5bd/+0xf5f3 contra o resumo), lance, +0x8eb2, o minuto (+0x8ed0 / 11)
/// contra o resumo (espera 0x226 - 50 x +0x51); com estado, registra (0x729ff0) e a do humano do dia
/// (+0x34) passa a ser a de maior competicao (0x6b6570) ou mais adiantada. Devolve 1 se mudou.
unsafe extern "thiscall" fn passo_em_rede(dia: usize, p: usize) -> u32 {
    let u = |q: usize, o: usize| core::ptr::read_unaligned((q + o) as *const usize);
    let por = |q: usize, o: usize, v: u32| core::ptr::write_unaligned((q + o) as *mut u32, v);
    let andar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6b5ce0usize);
    let iniciar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6b5a70usize);
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
    let sons: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6d8c70usize);
    let proximo_evento: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x6d78b0usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x6d5cd0usize);
    let laco: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x6b98c0usize);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x729ff0usize);
    let competicao: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x6b6570usize);
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
    crate::comparacao::ramo("passo em rede");
    let motor = u(p, 0x57);
    let estado = pl as *mut u8;
    let eu = *(0xb77c7a as *const i16) as isize;
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
        let ev = proximo_evento(u(motor, 0x8f7a), 0);
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
            let ev = proximo_evento(u(motor, 0x8f7a), 0);
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
    registrar(u(0xb6b7c0, 0), motor, u(p, 0x5f), 1);
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

// ponytail: os eventos gravados mexem em arquivos e o passo em rede so roda em jogo em rede; sem
// comparacao propria, validados pela carreira (Rust x original).
unsafe extern "thiscall" fn comparar_gravar_eventos(e: usize, desc: *mut u32, humano: u8) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, *mut u32, u8) -> u8>(339)(e, desc, humano)
}
unsafe extern "thiscall" fn comparar_ler_eventos(e: usize, desc: *const u32) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, *const u32) -> u8>(340)(e, desc)
}
unsafe extern "thiscall" fn comparar_passo_em_rede(dia: usize, p: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(341)(dia, p)
}
