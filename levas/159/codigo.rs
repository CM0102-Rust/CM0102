// ---------- centesima quinquagesima nona leva: a formacao (formation.cpp), posicoes do jogador e casas no campo ----------

/// { 0x5b7d20 } (cdecl, formation.cpp): a máscara das posições fortes do jogador `p` pelas notas de
/// posição (bytes com sinal): a partir de 15, descendo até 10, o primeiro nível em que alguma passa
/// dá os bits do setor (+0xf goleiro 1, +0x10 líbero 2, +0x11 zagueiro 4, +0x12 volante 8 ou, sem
/// ele, +0x14 meia 0x20 ou +0x13 0x10, +0x15 atacante 0x40); os lados (+0x17 0x800, +0x18 0x80,
/// +0x19 0x200) só contam com nota 15 ou mais (a original repete o mesmo teste a cada nível).
/// Devolve só `ax` (os chamadores usam 16 bits). `p` nulo: a original (caixa de erro e 0).
unsafe extern "C" fn mascara_de_posicoes(p: usize) -> u16 {
    if p == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u16>(349)(p);
    }
    let b = |o: usize| *((p + o) as *const i8);
    let mut m = 0u16;
    let mut nivel = 15i8;
    loop {
        if b(0xf) >= nivel {
            m |= 1;
        }
        if b(0x11) >= nivel {
            m |= 4;
        }
        if b(0x10) >= nivel {
            m |= 2;
        }
        if b(0x12) >= nivel {
            m |= 8;
        } else if b(0x14) >= nivel {
            m |= 0x20;
        } else if b(0x13) >= nivel {
            m |= 0x10;
        }
        if b(0x15) >= nivel {
            m |= 0x40;
        }
        nivel -= 1;
        if nivel < 10 || m != 0 {
            break;
        }
    }
    if b(0x17) >= 15 {
        m |= 0x800;
    }
    if b(0x18) >= 15 {
        m |= 0x80;
    }
    if b(0x19) >= 15 {
        m |= 0x200;
    }
    m
}

/// { 0x5b7e60 } (cdecl, formation.cpp): a casa no campo (`saida[0]` = coluna 0..26, `saida[1]` =
/// linha 0..35) da posição `mascara` (separada pela 0x5b73a0 em setor e lado), com `ataque` 0/1 e
/// `faixa` = palavra baixa 0..2 (esquerda, meio, direita) e palavra alta 0..3 (a altura na
/// formação). A linha vem do setor (tabelas de 4 pela altura; no ataque, os laterais 0x80/0x800
/// sobem 1, ou 1/4 nos setores 2 e 4); a coluna, do lado (tabelas de 5); depois os encostos nas
/// faixas e os limites. Argumentos fora da faixa: a original (caixa de erro e 0). Devolve 1.
unsafe extern "C" fn casa_no_campo(mascara: u32, ataque: i8, faixa: u32, saida: *mut i16) -> u32 {
    let baixo = faixa as u16 as i16;
    let alto = (faixa >> 16) as u16 as i16;
    if !(0..=1).contains(&ataque) || !(0..=2).contains(&baixo) || !(0..=3).contains(&alto) {
        return orig::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(350)(mascara, ataque, faixa, saida);
    }
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    let (mut setor, mut lado) = (0u16, 0u16);
    separar(mascara, &mut setor, &mut lado);
    let a = alto as usize;
    let lateral = lado & 0x80 != 0 || lado & 0x800 != 0;
    let at = ataque != 0;
    // a linha (saida[1])
    let linha: i16 = if setor & 1 != 0 {
        if at { [0, 2, 4, 6][a] } else { [0, 1, 2, 3][a] }
    } else if setor & 2 != 0 {
        if !at {
            [1, 3, 5, 9][a]
        } else {
            let v = [2, 4, 13, 15][a];
            if lateral { v + if alto != 3 { 1 } else { 4 } } else { v }
        }
    } else if setor & 4 != 0 {
        if !at {
            [2, 7, 9, 13][a]
        } else {
            let v = [4, 9, 17, 17][a];
            if lateral { v + if alto != 3 { 1 } else { 4 } } else { v }
        }
    } else {
        let (sem, com): ([i16; 4], [i16; 4]) = if setor & 8 != 0 {
            ([4, 9, 13, 18], [7, 11, 20, 22])
        } else if setor & 0x10 != 0 {
            ([6, 11, 16, 22], [9, 14, 23, 26])
        } else if setor & 0x20 != 0 {
            ([9, 13, 19, 26], [12, 19, 27, 30])
        } else {
            ([13, 15, 23, 31], [15, 24, 31, 34])
        };
        if !at {
            sem[a]
        } else if lateral {
            com[a] + 1
        } else {
            com[a]
        }
    };
    *saida.add(1) = linha;
    // a coluna (saida[0]), pelo lado
    let l = if lado & 0x80 != 0 {
        0
    } else if lado & 0x100 != 0 {
        1
    } else if lado & 0x200 != 0 {
        2
    } else if lado & 0x400 != 0 {
        3
    } else {
        4
    };
    let tabela: [i16; 5] = if setor & 1 != 0 {
        [13; 5]
    } else if setor & 6 != 0 || setor & 0x38 != 0 {
        match (at, alto == 0, setor & 6 != 0) {
            (false, true, _) => [9, 11, 13, 15, 17],
            (false, false, _) => [7, 11, 13, 15, 19],
            (true, true, true) => [6, 11, 13, 15, 20],
            (true, _, _) => [3, 10, 13, 16, 23],
        }
    } else if !at {
        [7, 11, 13, 15, 19]
    } else if alto == 0 || alto == 1 {
        [2, 8, 13, 18, 24]
    } else if alto == 2 {
        [3, 10, 13, 16, 23]
    } else {
        [6, 11, 13, 15, 20]
    };
    *saida = tabela[l];
    // os encostos: com `perto`, um passo na faixa; sem, cinco (fora do goleiro)
    let perto = (setor & 1 != 0 && alto > 0)
        || (setor & 6 != 0 && alto == 0)
        || (setor & 0x40 != 0 && lado & 0x700 != 0 && alto == 3 && ataque == 1);
    let passo = if perto { 1 } else if setor & 1 == 0 { 5 } else { 0 };
    if passo != 0 {
        let v = *saida as i32;
        if baixo == 0 {
            *saida = (v - passo).clamp(2, 0x11) as i16;
        } else if baixo == 2 {
            *saida = (v + passo).clamp(9, 0x18) as i16;
        }
    }
    *saida = (*saida).clamp(0, 0x1a);
    *saida.add(1) = (*saida.add(1)).clamp(0, 0x23);
    1
}

/// { 0x5b8960 } (thiscall, formation.cpp): as 11 vagas da formação `this` sem conflito: devolve 0
/// se alguma vaga com lado aberto (0x880 na casa +0x12d ou +0x14e, ver 0x5b2ae0) é também de
/// atacante (0x40, nas duas), de meia (0x20 na +0x12d e na +0x14e pela 0x5b2ae0) ou de volante e
/// meia-atacante pela 0x5b2ae0 (8 e 0x10); senão 1. Devolve só `al`.
unsafe extern "thiscall" fn vagas_sem_conflito(this: usize) -> u8 {
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let w = |o: usize| core::ptr::read_unaligned((this + o) as *const u16);
    for i in 0..=10u32 {
        let a = w(0x12d + 2 * i as usize);
        let b = w(0x14e + 2 * i as usize);
        if a & 0x880 != 0 && a & 0x40 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 0x40 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 0x20 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && casa(this, i, 1) & 0x20 != 0 {
            return 0;
        }
        if casa(this, i, 0) & 0x880 != 0 && casa(this, i, 0) & 8 != 0 {
            return 0;
        }
        if casa(this, i, 0) & 0x880 != 0 && casa(this, i, 0) & 0x10 != 0 {
            return 0;
        }
        if casa(this, i, 1) & 0x880 != 0 && casa(this, i, 1) & 0x10 != 0 {
            return 0;
        }
    }
    1
}

unsafe extern "C" fn comparar_mascara_de_posicoes(p: usize) -> u16 {
    let r = orig::<unsafe extern "C" fn(usize) -> u16>(349)(p);
    let n = mascara_de_posicoes(p);
    PLACAR.registrar(r == n, || format!("mascara_de_posicoes({:#x}) original={:#x} rust={:#x}", p, r, n));
    r
}

unsafe extern "C" fn comparar_casa_no_campo(mascara: u32, ataque: i8, faixa: u32, saida: *mut i16) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(350);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(mascara, ataque, faixa, saida), || casa_no_campo(mascara, ataque, faixa, saida));
    PLACAR.registrar(igual && r == v, || format!("casa_no_campo({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", mascara, ataque, faixa, r, mo, v, mr));
    r
}

comparar_valor!(comparar_vagas_sem_conflito, vagas_sem_conflito, 351, () -> u8);
