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
