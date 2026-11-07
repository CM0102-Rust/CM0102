// ---------- octogesima primeira leva: casa da vaga, byte de criterio da competicao ----------

/// { 0x5b2ae0 } (ret 8, formation.cpp): a casa da vaga 0..10 (palavra +0x12d, ou +0x14e com
/// `segunda`). Vaga fora da faixa: a original (caixa de erro e 1). Devolve so `ax`: no caminho +0x12d
/// a original deixa em cima de `eax` o que o chamador tinha ali.
unsafe extern "thiscall" fn casa_da_vaga(this: usize, vaga: i8, segunda: u32) -> u16 {
    if !(0..=10).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u16>(223)(this, vaga, segunda);
    }
    let base = if segunda != 0 { 0x14e } else { 0x12d };
    core::ptr::read_unaligned((this + base + 2 * vaga as usize) as *const u16)
}

/// { 0x4bff70 } (comp_util.cpp): com a palavra baixa de `b` (ou, sem ela, a de `a`), o resultado
/// de 0x4ba8f0(palavra, 0); senao o byte +0x37 (`tipo` 1) ou +0x52 do registro; sem registro, 1.
unsafe extern "C" fn criterio_da_competicao(registro: usize, a: u32, b: u32, tipo: u8) -> u8 {
    let texto: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x4ba8f0usize);
    if b as u16 != 0 {
        return texto(b, 0);
    }
    if a as u16 != 0 {
        return texto(a, 0);
    }
    if registro == 0 {
        return 1;
    }
    *((registro + if tipo == 1 { 0x37 } else { 0x52 }) as *const u8)
}

// ---------- octogesima leva: esquecer a pessoa e quadro do tecnico (nevoa), casas da formacao ----------

/// { 0x5afd00 } (ret 4, fog_of_war.cpp): com a nevoa ligada (0x9cc270) e a pessoa valida, zera o
/// nivel dela (registro de 9 bytes) nos 16 quadros de [this] que existem e estao ativos (+4).
unsafe extern "thiscall" fn esquecer_pessoa(this: usize, pessoa: usize) {
    let valida = || *(pessoa as *const i32) < *(0xae2364 as *const i32) && *(0x9cc270 as *const u8) != 0;
    if pessoa == 0 || !valida() {
        return;
    }
    for k in 0..16usize {
        let q = *((this + 4 * k) as *const usize);
        if q != 0 && *((q + 4) as *const u32) != 0 && valida() {
            *((*(q as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9)) as *mut u8) = 0;
        }
    }
}

/// { 0x5afde0 } (ret 4, fog_of_war.cpp): o quadro do tecnico, [this + (0xae2364 - id) x 4 - 4];
/// sem tecnico, 0.
unsafe extern "thiscall" fn quadro_do_tecnico(this: usize, tecnico: usize) -> usize {
    if tecnico == 0 {
        return 0;
    }
    let k = (*(0xae2364 as *const i32)).wrapping_sub(*(tecnico as *const i32)) as usize;
    *(this.wrapping_add(k.wrapping_mul(4)).wrapping_sub(4) as *const usize)
}

/// { 0x5b2e80 } (ret 8, formation.cpp): a primeira das 11 casas (+0x12d) com o setor da mascara
/// (0x5b73a0) e o bit 0x200; sem ela, 0. Sem `jogadores`, 1; senao 1 se algum dos 20 (+0x19, ou
/// +0x1a com 0xfe em +0x19) esta naquela vaga.
unsafe extern "thiscall" fn casa_do_setor_ocupada(this: usize, mascara: u32, jogadores: usize) -> u32 {
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    let (mut setor, mut lado) = (0u16, 0u16);
    separar(mascara, &mut setor, &mut lado);
    let casa = |k: usize| core::ptr::read_unaligned((this + 0x12d + 2 * k) as *const u16);
    let k = match (0..11usize).find(|&k| casa(k) & setor != 0 && casa(k) & 0x200 != 0) {
        Some(k) => k as u8,
        None => return 0,
    };
    if jogadores == 0 {
        return 1;
    }
    (0..20usize).any(|i| {
        let p = jogadores + 0x19 + i * 0x1be;
        let c = *(p as *const u8);
        c == k || (c == 0xfe && *((p + 1) as *const u8) == k)
    }) as u32
}

/// { 0x5b2f10 } (ret 4, formation.cpp): a vaga (0..10) da casa `codigo` entre as 11 de +0x12d; 0xff
/// se nao estiver.
unsafe extern "thiscall" fn vaga_da_casa(this: usize, codigo: u16) -> u32 {
    (0..11u32).find(|&k| core::ptr::read_unaligned((this + 0x12d + 2 * k as usize) as *const u16) == codigo).unwrap_or(0xff)
}

/// { 0x5b5960 } (formation.cpp): as 11 marcas (+0x5b4) com 4 e 8 viram 2, e as fichas designadas
/// (+0x5bf) voltam a -1. Devolve 1.
unsafe extern "thiscall" fn limpar_designados(this: usize) -> u32 {
    for k in 0..11usize {
        let b = (this + 0x5b4 + k) as *mut u8;
        if *b & 4 != 0 && *b & 8 != 0 {
            *b = 2;
        }
        core::ptr::write_unaligned((this + 0x5bf + 4 * k) as *mut i32, -1);
    }
    1
}

/// { 0x5b59a0 } (formation.cpp): a mascara da formacao (+0x187b) = OU dos bits das 11 casas
/// (0x5b5a60), com onze mascaras trocadas por quatro formacoes de referencia. Devolve a mascara
/// antes da troca (o eax da original).
unsafe extern "thiscall" fn mascara_da_formacao(this: usize) -> u32 {
    let bit: unsafe extern "stdcall" fn(i16) -> u32 = core::mem::transmute(0x5b5a60usize);
    let m = (this + 0x187b) as *mut u32;
    core::ptr::write_unaligned(m, 0);
    for k in 0..11usize {
        let c = core::ptr::read_unaligned((this + 0x12d + 2 * k) as *const i16);
        let v = core::ptr::read_unaligned(m) | bit(c);
        core::ptr::write_unaligned(m, v);
    }
    let v = core::ptr::read_unaligned(m);
    let troca = match v {
        0x101176c1 | 0x101b26c1 => 0x101f06c1,
        0x38115381 | 0x3a242381 => 0x381b0381,
        0x280e07c1 | 0x091a07c1 | 0x2884ab81 | 0x280aab81 | 0x288c07c1 | 0x281507c1 => 0x280e8b81,
        0x288a8b81 => 0x281b2381,
        _ => v,
    };
    core::ptr::write_unaligned(m, troca);
    v
}

// ---------- quinquagesima nona leva: marcado designado ----------

/// { 0x5b58e0 } (formation.cpp): a pessoa designada para a posicao (indice em +0x5bf + 4 x posicao,
/// no vetor de pessoas de 0xae23bc, 110 bytes cada); indice negativo: 0.
unsafe extern "thiscall" fn designado_da_posicao(this: usize, posicao: i8) -> usize {
    let i = *((this as isize + 0x5bf + 4 * posicao as isize) as *const i32);
    if i > -1 {
        (*(0xae23bc as *const usize)).wrapping_add(110 * i as usize)
    } else {
        0
    }
}

/// { 0x7160f0 } (match_pl.cpp): o adversario que o jogador marca individualmente, ou 0. Precisa
/// da marca 4 na casa da posicao (0x5b57c0, formacao do lado em motor + 0x91e2); procura entre os
/// adversarios ativos a ficha designada para a posicao (0x5b58e0). Com a marca 0x10 ainda: se a
/// formacao adversaria tem o bit 0x10000000 em +0x187b ou o byte +0xaf do outro lado passa de 1, o
/// jogador com 0x106 nao marca; e, se o marcado tem as tabelas A e B diferentes e sem a marca 0x40
/// na B, o jogador com 0x106 tambem nao marca.
unsafe extern "thiscall" fn marcado_designado(this: usize) -> usize {
    let marca: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    let designado: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(0x5b58e0usize);
    let motor = *((this + 0x19e) as *const usize);
    let lado = byte(this, 0x27);
    let formacao = (motor as isize + 0x91e2 + 0x18e3 * lado as isize) as usize;
    if marca(formacao, byte(this, 0x19)) & 4 == 0 {
        return 0;
    }
    let ficha = designado(formacao, byte(this, 0x19));
    let outro = (lado == 0) as i8;
    let alvo = match (0..20).map(|k| jogador_do_bloco(motor, outro, k)).find(|&q| byte(q, 0x19) >= 0 && *((q + 0x69) as *const usize) == ficha) {
        Some(q) => q,
        None => return 0,
    };
    if marca(formacao, byte(this, 0x19)) & 0x10 == 0 {
        return alvo;
    }
    let o = outro as usize;
    let fechada = *((motor + 0xaa5d + 0x18e3 * o) as *const u32) & 0x1000_0000 != 0 || byte(motor + o, 0xaf) > 1;
    if fechada && byte(this, 0x106) != 0 {
        return 0;
    }
    let m_alvo = *((alvo + 0x19e) as *const usize);
    let b = if m_alvo != 0 { tabela_b(m_alvo, alvo) } else { 0 };
    let a = if m_alvo != 0 { tabela_a(m_alvo, alvo) } else { 0 };
    if b == a || b & 0x40 != 0 || byte(this, 0x106) == 0 {
        alvo
    } else {
        0
    }
}

// ---------- quinquagesima oitava leva: casas da formacao e craques ----------

/// { 0x5b5290 } (formation.cpp): liga o bit `mascara` (um so) na mascara de casas da posicao
/// (+0x188b + 8 x posicao, 64 bits), apagando antes o grupo dele (bits que se excluem), e zera
/// +0x187f. Posicao fora de 0..=10 ou bit desconhecido: o original mostra uma caixa de erro (sem a
/// caixa aqui), zera 0xb67a34 e devolve 0.
unsafe extern "thiscall" fn gravar_casa(this: usize, posicao: i8, mascara: u64) -> u32 {
    let grupo: u64 = match mascara {
        0x1 | 0x2 | 0x4 | 0x8 | 0x10 => 0x1f,
        0x20 | 0x40 | 0x80 | 0x100 => 0x1e0,
        0x200 | 0x400 | 0x800 | 0x1000 => 0x1e00,
        0x2000 | 0x4000 | 0x8000 => 0xe000,
        0x10000 | 0x20000 | 0x4000_0000 => 0x4003_0000,
        0x40000 | 0x80000 | 0x8000_0000 => 0x800c_0000,
        0x10_0000 | 0x20_0000 => 0x30_0000,
        0x40_0000 | 0x80_0000 => 0xc0_0000,
        0x100_0000 | 0x200_0000 => 0x300_0000,
        0x400_0000 | 0x800_0000 => 0xc00_0000,
        0x1000_0000 | 0x2000_0000 => 0x3000_0000,
        0x1_0000_0000 | 0x2_0000_0000 => 0x3_0000_0000,
        0x4_0000_0000 | 0x8_0000_0000 => 0xc_0000_0000,
        _ => 0,
    };
    if !(0..=10).contains(&posicao) || grupo == 0 {
        *(0xb67a34 as *mut u32) = 0;
        return 0;
    }
    let casa = (this + 0x188b + 8 * posicao as usize) as *mut u64;
    *casa = (*casa & !grupo) | mascara;
    *((this + 0x187f) as *mut u32) = 0;
    1
}

/// { 0x6d10a0 } (match_eng.cpp): marca (bit 0x800000 da formacao do lado, 0x5b5290) os jogadores
/// ativos cuja nota passa de 8500: 10 x (+0x43 + +0x33 + +0x27 da pessoa) + forma (+0x4d), com o
/// papel (0x717410): goleiro fica de fora; 2 +500; 4 -1000 (-500 com 0x800 ou 0x80); 8 -300 (sem
/// 0x800 e 0x80); 0x10 +1000; 0x20 +2000.
unsafe extern "thiscall" fn marcar_craques(this: usize, lado: i8) {
    for k in 0..20 {
        let j = jogador_do_bloco(this, lado, k);
        if byte(j, 0x19) < 0 {
            continue;
        }
        let pd = *((j + 0x6d) as *const usize);
        let soma = 10 * (byte(pd, 0x43) as i32 + byte(pd, 0x33) as i32 + byte(pd, 0x27) as i32);
        let mut v = ftol(soma as f64 + f32_em(j + 0x4d) as f64);
        // ponytail: o original chama 0x717410 a cada teste; ela so le o jogador
        let r = f717410(j);
        if r & 1 != 0 {
            continue;
        }
        if r & 2 != 0 {
            v += 500;
        } else if r & 4 != 0 {
            v -= if r & 0x800 != 0 || r & 0x80 != 0 { 500 } else { 1000 };
        } else if r & 8 != 0 {
            if r & 0x800 == 0 && r & 0x80 == 0 {
                v -= 300;
            }
        } else if r & 0x10 != 0 {
            v += 1000;
        } else if r & 0x20 != 0 {
            v += 2000;
        }
        if v as i16 > 8500 {
            let gravar: unsafe extern "thiscall" fn(usize, i8, u64) -> u32 = core::mem::transmute(0x5b5290usize);
            gravar((this as isize + 0x91e2 + 0x18e3 * lado as isize) as usize, byte(j, 0x19), 0x80_0000);
        }
    }
}

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

// ---------- 257a leva: as casas da formação: gravar a coordenada da casa e escolher as posições, a posição repetida e o tipo de marcação (formation.cpp) ----------



/// { 0x5b3560 } (thiscall, ret 0x10, formation.cpp): `Formation::set_slot_coordinate(casa, fase, faixa, coord)`:
/// grava a coordenada (dword: x 0..0x1a, y 0..0x23) da casa na fase e na faixa da bola (+0x164, 11 × 2 × 3 × 4
/// dwords) e, para cada fase, acha as posições da tabela 0x9cc368 (palavras até 0xffff) mais perto das 12 coordenadas
/// da casa nessa fase (|Δx·9/27| + |Δy·12/36| contra a casa de cada posição pela 0x5b7e60; dobrada nas posições com o
/// bit 2; os empates entram na lista da fase, até 11 em [esp+0x64] contíguas). A posição geral (+0x12d) fica se
/// estiver numa das listas; senão vira a primeira da fase de menor distância (a 0x5b59a0 refaz a máscara, a casa
/// ganha a marca +0x143 e as outras casas com a mesma posição a perdem). A posição extra (+0x14e) sai da lista da
/// outra fase: a mesma posição, ou a de mesmo lado (0x5b73a0), ou a primeira, ou com duas o meio dos bits das duas.
/// Zera +0x187f. Argumentos fora da faixa: a original (caixa e 0). Devolve 1.
unsafe extern "thiscall" fn coordenada_da_casa(this: usize, casa: u32, fase: u32, faixa: u32, coord: u32) -> u32 {
    let (s, f) = (casa as u8 as i8, fase as u8 as i8);
    let (zx, zy) = (faixa as u16 as i16, (faixa >> 16) as u16 as i16);
    let (cx, cy) = (coord as u16 as i16, (coord >> 16) as u16 as i16);
    if !(0..=10).contains(&s) || !(0..=1).contains(&f) || !(0..=2).contains(&zx) || !(0..=3).contains(&zy)
        || !(0..=0x1a).contains(&cx) || !(0..=0x23).contains(&cy)
    {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(1224)(this, casa, fase, faixa, coord);
    }
    let casa_no: unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32 = core::mem::transmute(0x5b7e60usize);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
    let su = s as usize;
    let cel = this + 0x164 + 4 * (zy as usize + 4 * (zx as usize + 3 * (f as usize + 2 * su)));
    crate::comparacao::ramo("cpu_11 coordenada gravada");
    core::ptr::write_unaligned(cel as *mut u32, coord); // cpu11: a coordenada
    // a saída da 0x5b7e60 é o próprio argumento coord na original (uma vaga só, para a função inteira)
    let mut saida: [i16; 2] = [cx, cy];
    // ponytail: as listas das duas fases ficam contíguas como em [esp+0x64]; os índices além de 0xea não acontecem
    let mut cand = [0u16; 0xea];
    let mut conta = [0i8; 2]; // na original sem valor inicial; a primeira posição sempre a zera
    let mut melhor = [0i32; 2];
    let mut ultimo_dy: i32 = 0; // [esp+0x60], lido só pelos índices fora da tabela no fim
    for p in 0..2usize {
        melhor[p] = 0x7fffffff;
        let mut lista = 0x9cc368usize;
        while read_i16(lista) != -1 {
            let pos = read_i16(lista) as u16;
            let base = this + 0x164 + su * 0x60 + p * 0x30;
            let mut dist: i32 = 0;
            for zx in 0..3u32 {
                for zy in 0..4u32 {
                    // a máscara vai com a altura da faixa nos 16 bits de cima (o ecx da original); a 0x5b7e60 não os lê
                    casa_no(pos as u32 | zy << 16, p as i8, zx | zy << 16, saida.as_mut_ptr());
                    let c = base + 4 * (zx as usize * 4 + zy as usize);
                    let dy = ((read_i16(c + 2) as i32 * 12) / 36 - (saida[1] as i32 * 12) / 36).wrapping_abs();
                    let dx = ((read_i16(c) as i32 * 9) / 27 - (saida[0] as i32 * 9) / 27).wrapping_abs();
                    ultimo_dy = dy;
                    dist = dx.wrapping_add(dist).wrapping_add(dy);
                }
            }
            if pos & 2 != 0 {
                crate::comparacao::ramo("cpu_11 distância dobrada");
                dist = dist.wrapping_add(dist); // cpu11: o dobro
            }
            if dist <= melhor[p] { // cpu11: o empate entra na lista
                if dist < melhor[p] {
                    conta[p] = 0;
                } else {
                    crate::comparacao::ramo("cpu_11 empate na lista");
                }
                melhor[p] = dist;
                cand[(conta[p] as i32 + p as i32 * 11) as usize] = pos;
                conta[p] = conta[p].wrapping_add(1);
            }
            lista += 2;
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    let geral = this + 0x12d + su * 2;
    let mut sf: i8 = (melhor[0] > melhor[1]) as i8;
    let mut si: i8 = 0;
    for q in 0..2i8 {
        let mut a: i8 = 0;
        while a < conta[q as usize] {
            if cand[(a as i32 + q as i32 * 11) as usize] == read_i16(geral) as u16 {
                sf = q;
                si = a;
                break;
            }
            a += 1;
        }
    }
    let novo = cand[(sf as i32 * 11 + si as i32) as usize];
    if read_i16(geral) as u16 != novo {
        write_u16(geral, novo);
        mascara(this);
        *((this + 0x143 + su) as *mut u8) = 1;
        for k in 0..11i8 {
            if k != s && read_i16(this + 0x12d + k as usize * 2) as u16 == novo {
                *((this + 0x143 + k as usize) as *mut u8) = 0;
            }
        }
    }
    let extra = this + 0x14e + su * 2;
    let o = (1 - sf as i32) * 11; // o começo da lista da outra fase
    let no = conta[(1 - sf) as usize];
    let atual = read_i16(geral) as u16;
    for a in 0..no.max(0) as i32 {
        if cand[(o + a) as usize] == atual {
            write_u16(extra, cand[(o + a) as usize]);
            return 1;
        }
    }
    // ponytail: as saídas da 0x5b73a0 ficam sem valor inicial na original se ela falhar (posição sem lado); aqui 0
    let (mut setor, mut lado_c, mut lado_a) = (0u16, 0u16, 0u16);
    for b in 0..no.max(0) as i32 {
        separar(cand[(o + b) as usize] as u32, &mut setor, &mut lado_c);
        separar(read_i16(geral) as u16 as u32, &mut setor, &mut lado_a);
        if lado_c == lado_a {
            write_u16(extra, cand[(o + b) as usize]);
            return 1;
        }
    }
    if no != 2 {
        write_u16(extra, cand[o as usize]);
        return 1;
    }
    // duas na outra fase: o meio dos bits de setor (1..0x40) e de lado (0x80..0x800) das duas. As palavras do quadro a
    // partir de [esp+0x38]: a tabela dos lados e 0xffff, a faixa (3) e 0, o ponteiro da melhor distância (sem valor
    // reproduzível: 0), this, a tabela dos setores e 0xffff, o último |Δy| e as listas.
    let (c0, c1) = (cand[o as usize], cand[(o + 1) as usize]);
    let mut q = [0u16; 0x100];
    q[..6].copy_from_slice(&[0x80, 0x100, 0x200, 0x400, 0x800, 0xffff]);
    q[6] = 3;
    q[10] = this as u16;
    q[11] = (this >> 16) as u16;
    q[12..20].copy_from_slice(&[1, 2, 4, 8, 0x10, 0x20, 0x40, 0xffff]);
    q[20] = ultimo_dy as u16;
    q[21] = (ultimo_dy >> 16) as u16;
    q[22..].copy_from_slice(&cand[..0xea]);
    let mut v1a: i8 = conta[0]; // [esp+0x1a]
    let mut v12: i8 = no; // [esp+0x12]
    for k in 0..7usize {
        if q[12 + k] & c0 != 0 {
            v1a = k as i8;
        }
        if q[12 + k] & c1 != 0 {
            v12 = k as i8;
        }
    }
    let mut v13: i8 = si; // [esp+0x13]
    let mut dl: i8 = v1a;
    for k in 0..5usize {
        if q[k] & c0 != 0 {
            v13 = k as i8;
        }
        if q[k] & c1 != 0 {
            dl = k as i8;
        }
    }
    let ib = (v13 as i32 + dl as i32) / 2;
    let ia = (v12 as i32 + v1a as i32) / 2;
    write_u16(extra, q[ib as usize & 0xff] | q[(12 + ia) as usize & 0xff]);
    1
}

/// { 0x5b3460 } (thiscall, ret 8, formation.cpp): `Formation::check_duplicate_position(casa, *saida)`: zera a saída;
/// a primeira outra casa com a mesma posição geral (+0x12d) decide: com marca (+0x143, byte com sinal) menor que a da
/// casa, saída 1 e devolve 1; senão devolve 0. Sem repetida, 1. Casa fora de 0..10: a original (caixa e 0).
unsafe extern "thiscall" fn posicao_repetida_na_formacao(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let s = casa as u8 as i8;
    if !(0..=10).contains(&s) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(1225)(this, casa, saida);
    }
    *saida = 0;
    let su = s as usize;
    for k in 0..11usize {
        if k != su && read_i16(this + 0x12d + k * 2) == read_i16(this + 0x12d + su * 2) {
            if *((this + 0x143 + k) as *const i8) < *((this + 0x143 + su) as *const i8) {
                *saida = 1;
                return 1;
            }
            return 0;
        }
    }
    1
}

/// { 0x5b57d0 } (thiscall, ret 8, formation.cpp): `Formation::set_marking_type(casa, marca)`: as marcas 1, 2 e 4
/// trocam os bits 0..2 do byte +0x5b4 da casa; 8 e 0x10, os bits 3..4; zera +0x187f e devolve 1. Outra marca: a
/// original (caixa e 0). A casa não é conferida.
unsafe extern "thiscall" fn tipo_de_marcacao(this: usize, casa: u32, marca: u32) -> u32 {
    let m = marca as u8;
    let limpa = match m {
        1 | 2 | 4 => 0xf8u8,
        8 | 0x10 => 0xe7,
        _ => return orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(1226)(this, casa, marca),
    };
    let b = (this as isize + casa as u8 as i8 as isize + 0x5b4) as *mut u8;
    *b = *b & limpa | m;
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// As regiões da formação que as três gravam: as posições, as marcas, as extras e as coordenadas (+0x12d..+0x584),
/// os tipos de marcação (+0x5b4, 11) e a máscara com +0x187f (+0x187b, 8).
fn cpu11_regioes(this: usize) -> [(usize, usize); 3] {
    [(this + 0x12d, 0x584 - 0x12d), (this + 0x5b4, 11), (this + 0x187b, 8)]
}

/// Comparador: grava só a formação (a 0x5b7e60 e a 0x5b73a0 são puras; a 0x5b59a0 refaz a máscara): lado a lado.
unsafe extern "thiscall" fn comparar_coordenada_da_casa(this: usize, casa: u32, fase: u32, faixa: u32, coord: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(1224);
    let reg = cpu11_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, fase, faixa, coord), || coordenada_da_casa(this, casa, fase, faixa, coord));
    PLACAR.registrar(igual && r == n, || format!("coordenada_da_casa({:x}, {}, {}, {:x}, {:x}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, fase as u8 as i8, faixa, coord, r, n, mo, mr));
    r
}

/// Comparador: pura (grava só a saída): as duas sobre saídas próprias.
unsafe extern "thiscall" fn comparar_posicao_repetida_na_formacao(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(1225);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, saida); // ponytail: a caixa, só a original
    }
    let (mut so, mut sr) = (0xdead_u32, 0xbeef_u32);
    let r = o(this, casa, &mut so);
    let n = posicao_repetida_na_formacao(this, casa, &mut sr);
    PLACAR.registrar(r == n && so == sr, || format!("posicao_repetida_na_formacao({:x}, {}) {}/{} saída {}/{}", this, casa as u8 as i8, r, n, so, sr));
    *saida = so;
    r
}

/// Comparador: grava um byte e +0x187f: lado a lado.
unsafe extern "thiscall" fn comparar_tipo_de_marcacao(this: usize, casa: u32, marca: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(1226);
    if !matches!(marca as u8, 1 | 2 | 4 | 8 | 0x10) {
        return o(this, casa, marca); // ponytail: a caixa, só a original
    }
    let reg = [(this as isize + casa as u8 as i8 as isize + 0x5b4) as usize, 1usize];
    let reg = [(reg[0], reg[1]), (this + 0x187f, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, marca), || tipo_de_marcacao(this, casa, marca));
    PLACAR.registrar(igual && r == n, || format!("tipo_de_marcacao({:x}, {}, {}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, marca as u8, r, n, mo, mr));
    r
}

// ---------- 214a leva: o mapa de casas da formação, a nota do clube para o jovem e a partida assistida pelo humano (formation.cpp, player_regen.cpp, human_manager.cpp) ----------

/// { 0x5b7160 } (thiscall, formation.cpp): o mapa de casas da formação. Para cada uma das 22 linhas
/// k (11 casas × 2 fases) e bloco 3 × 4 (a18, a14) com a posição (i = 12k + 4·a18 + a14) em +0x164
/// (coluna, palavra) e +0x166 (linha), as 3 × 3 células do bloco ganham a coluna × 9 / 27 no nibble
/// baixo e a linha × 12 / 36 no alto da tabela +0x5eb (12 por fila, 9 filas por k); o espelho em +0xf9e
/// (fila e coluna invertidas) recebe (0xb0 − linha × 16) com (0xf8 − coluna) & 0xf. Devolve 1.
unsafe extern "thiscall" fn mapa_de_casas_da_formacao(this: usize) -> u32 {
    for k in 0..22i32 {
        for a18 in 0..3i32 {
            for a14 in 0..4i32 {
                let i = (12 * k + 4 * a18 + a14) as usize;
                for y in 3 * a18..3 * a18 + 3 {
                    for x in 3 * a14..3 * a14 + 3 {
                        let coluna = core::ptr::read_unaligned((this + i * 4 + 0x164) as *const i16) as i32;
                        let c = (coluna * 9 / 27) as u8;
                        let p = (this as isize + 0x5eb + ((9 * k + y) * 12 + x) as isize) as *mut u8;
                        *p = (*p & 0xf0) | (c & 0xf);
                        let linha = core::ptr::read_unaligned((this + i * 4 + 0x166) as *const i16) as i32;
                        let d = ((linha * 12 / 36) as u8) << 4;
                        *p = (c & 0xf) | d;
                        let q = (this as isize + 0xf9e + ((9 * k - y) * 12 - x) as isize) as *mut u8;
                        *q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho
                    }
                }
            }
        }
    }
    1
}

/// { 0x7ad220 } (stdcall, ret 0xc, player_regen.cpp): a nota do `clube` para o jovem `jogador`
/// (sorteio(nota)): 5000 + reputação (+0x80) menos as distâncias dela à habilidade (ficha +0x61: +0xb,
/// e +7, +5 × 50); com +7 acima de sorteio(20) + 100, o bônus de juniores (+0x7f, 20 = garantido,
/// senão sorteio(+0x7f) > sorteio(20)); clube de competição ativa (0xadadfc) com +7 > 50: o estado do
/// clube humano (0xae2d14) e a reputação contra o elenco (`elenco`); o treinador de juniores (+0xcf
/// → +0x69) do clube do computador (0x600a50) ou o bônus do humano; a mesma nação (+0x1a/+0x53) e os
/// clubes galeses (Swansea, Cardiff, Wrexham) e a Escócia. Só os 16 bits baixos da nota contam.
unsafe extern "stdcall" fn nota_do_clube_para_o_jovem(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let ficha = u(jogador, 0x61);
    let rep = w(clube, 0x80) as i32;
    let mut s = rep
        .wrapping_sub((w(ficha, 0xb) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 7).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_sub((w(ficha, 5).wrapping_mul(0x32) as i32 - rep).wrapping_abs())
        .wrapping_add(0x1388);
    let juniores = *((clube + 0x7f) as *const i8);
    if w(ficha, 7) as i32 > sorteio(0x14) + 0x64 {
        if juniores == 0x14 {
            s = s.wrapping_add(rep / 2 + 0x1b58);
        } else if sorteio(juniores as i32) > sorteio(0x14) {
            s = s.wrapping_add(rep / 2 + (juniores as i32 + 0x64) * 50);
        }
    }
    let comp = u(clube, 0x57);
    if w(ficha, 7) > 0x32 && comp != 0 && *((*(0xadadfc as *const usize) + 4 * *(comp as *const u32) as usize) as *const usize) != 0 {
        let estados = *(0xae2d14 as *const usize);
        if humano(clube, 1, 1, 0, 0) != 0 && estados != 0 && w(ficha, 7) > 0x78 {
            let e = *((estados + 6 * *(clube as *const u32) as usize + 4) as *const i8);
            if e < 0 {
                s = s.wrapping_add(0x1f4);
            } else if e == 0 {
                s = s.wrapping_add(0xfa);
            }
        }
        let r = w(clube, 0x80);
        s = s.wrapping_add(if r > 0x1676 {
            if elenco < 0x28 { 0x3e8 } else if elenco < 0x2d { 0x2ee } else { 0xfa }
        } else if r > 0x109a {
            if elenco < 0x23 { 0x2ee } else if elenco < 0x28 { 0x1f4 } else { 0xfa }
        } else {
            0xfa
        });
    }
    let tecnico = u(clube, 0xcf);
    let x = if tecnico != 0 { u(tecnico, 0x69) } else { 0 };
    if x != 0 && humano(clube, 1, 1, 0, 0) == 0 {
        s = s.wrapping_add((*((x + 0x22) as *const i8) as i32 - 10) * 30);
        if w(ficha, 7) as i32 > sorteio(0x4b) + 0x64 {
            let v = chance(x, *((x + 0x18) as *const i8)) as i8;
            s = s.wrapping_add((v as i32 - 10) * 60);
        }
        let v = chance(x, *((x + 0x18) as *const i8)) as i8;
        let t = v as i32 + *((x + 0x22) as *const i8) as i32 + juniores as i32;
        if t > 0x28 && w(clube, 0x80) > 0x109a && w(ficha, 7) > 0x50 {
            if (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            } else {
                s = s.wrapping_add(0xfa);
            }
        }
    } else if humano(clube, 1, 1, 0, 0) != 0 {
        s = s.wrapping_add(0xc8);
        if w(ficha, 7) > 0x78 {
            if juniores == 0x14 && (s as i16) < w(clube, 0x80) {
                s = w(clube, 0x80) as i32;
            }
            s = s.wrapping_add(juniores as i32 * 25);
        }
    }
    let nacao = u(jogador, 0x1a);
    let nacao_do_clube = u(clube, 0x53);
    if nacao == nacao_do_clube {
        s = s.wrapping_add(0x3e8); // mesma nação
    }
    let g = |a: usize| *(a as *const u32);
    let (gales, inglaterra, escocia) = (g(0x9cf510), g(0x9cf2e4), g(0x9cf45c));
    let id = g(clube);
    if id == g(0x9d0430) || id == g(0x9d0434) || (id == g(0x9d043c) && nacao != 0) {
        let n = g(nacao);
        if n == gales {
            s = s.wrapping_add(0x3e8);
        } else if n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        }
    }
    if g(nacao_do_clube) == escocia {
        let n = g(nacao);
        if n == gales || n == inglaterra {
            s = s.wrapping_sub(0x2ee);
        } else if n == escocia {
            s = s.wrapping_add(0x3e8);
        }
    }
    sorteio(s as i16 as i32)
}

/// O registro (42735 bytes em [this]) da partida que o `humano` (ou o da tela, 0xb74c26 + 0x300 ×
/// [0xb77c7a]) assiste: Ok(0) sem humano; Err se o índice (id − [0xae2364] + 0x10) sai de 0..[this+4].
unsafe fn registro_da_partida_assistida(this: usize, humano: usize) -> Result<usize, ()> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return Ok(0);
    }
    let i = (*(h as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(());
    }
    Ok((*(this as *const usize)).wrapping_add((i as usize).wrapping_mul(42735)))
}

/// { 0x5fe290 } (thiscall, ret 8, human_manager.cpp): se a `partida` é a que o humano escolheu
/// assistir: o registro dele (+0x9ad4..) com a mesma competição (+0x3f), datas (+0x2a, +0x28), clube
/// (+0), +0x34 e os dois lados (+0xc, +0x10). Partida nula ou humano fora da faixa: a original (caixa
/// de erro e 0).
unsafe extern "thiscall" fn partida_assistida_pelo_humano(this: usize, partida: usize, humano: usize) -> u32 {
    let r = match registro_da_partida_assistida(this, humano) {
        Err(()) => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542)(this, partida, humano),
        Ok(_) if partida == 0 => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542)(this, partida, humano),
        Ok(0) => return 0,
        Ok(r) => r,
    };
    let b = |p: usize, o: usize| *((p + o) as *const u8);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let d = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u32);
    let mesma = b(partida, 0x3f) == b(r, 0x9b13)
        && w(partida, 0x2a) == w(r, 0x9afe)
        && w(partida, 0x28) == w(r, 0x9afc)
        && d(partida, 0) == d(r, 0x9ad4)
        && w(partida, 0x34) == w(r, 0x9b08)
        && d(partida, 0xc) == d(r, 0x9ae0)
        && d(partida, 0x10) == d(r, 0x9ae4);
    if mesma { 1 } else { 0 } // outra partida
}

unsafe extern "thiscall" fn comparar_mapa_de_casas_da_formacao(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(540);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x5eb, 0x187b - 0x5eb)], || o(this), || mapa_de_casas_da_formacao(this));
    PLACAR.registrar(igual && r == n, || format!("mapa_de_casas_da_formacao {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "stdcall" fn comparar_nota_do_clube_para_o_jovem(jogador: usize, clube: usize, elenco: i8) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, i8) -> i32>(541);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(jogador, clube, elenco), || crate::aleatorio::sem_contar(|| nota_do_clube_para_o_jovem(jogador, clube, elenco)));
    PLACAR.registrar(igual && r == n, || format!("nota_do_clube_para_o_jovem(elenco {}) original={} rust={} sorteio igual={}", elenco, r, n, igual));
    r
}

unsafe extern "thiscall" fn comparar_partida_assistida_pelo_humano(this: usize, partida: usize, humano: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(542);
    let r = o(this, partida, humano);
    // ponytail: os casos de erro (caixa) só na original
    if partida == 0 || registro_da_partida_assistida(this, humano).is_err() {
        return r;
    }
    let n = partida_assistida_pelo_humano(this, partida, humano);
    PLACAR.registrar(r == n, || format!("partida_assistida_pelo_humano original={} rust={}", r, n));
    r
}

// ---------- centesima sexagesima terceira leva: a formacao (marcas e casas) e o reinicio dos recordes do clube ----------

/// { 0x5b3c70 } (thiscall, ret 0x10, formation.cpp): a casa guardada da vaga `vaga` 0..10 com
/// `ataque` 0/1 e `faixa` (palavra baixa 0..2, alta 0..3): o dword +0x164 + 4×(alta + 4×(baixa +
/// 3×(ataque + 2×vaga))) vai para `*saida`. Devolve 1. Fora das faixas: a original (caixa de erro).
unsafe extern "thiscall" fn casa_guardada(t: usize, vaga: i8, ataque: i8, faixa: u32, saida: *mut u32) -> u32 {
    let baixo = faixa as u16 as i16;
    let alto = (faixa >> 16) as u16 as i16;
    if !(0..=10).contains(&vaga) || !(0..=1).contains(&ataque) || !(0..=2).contains(&baixo) || !(0..=3).contains(&alto) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(361)(t, vaga, ataque, faixa, saida);
    }
    let i = alto as usize + 4 * (baixo as usize + 3 * (ataque as usize + 2 * vaga as usize));
    *saida = core::ptr::read_unaligned((t + 0x164 + 4 * i) as *const u32);
    1
}

/// { 0x5b50d0 } (thiscall, ret 4, formation.cpp): liga a marca `m` em +0x584 apagando antes as do
/// mesmo grupo (1/2/4/8; 0x10/0x20/0x40; 0x80/0x100; 0x200/0x400; 0x800/0x1000; 0x2000/0x4000;
/// 0x8000/0x10000/0x20000) e zera +0x187f. Devolve 1. Marca fora dos grupos: a original (caixa de
/// erro).
unsafe extern "thiscall" fn ligar_marca_da_formacao(t: usize, m: u32) -> u32 {
    let grupo: u32 = match m {
        1 | 2 | 4 | 8 => 0xf,
        0x10 | 0x20 | 0x40 => 0x70,
        0x80 | 0x100 => 0x180,
        0x200 | 0x400 => 0x600,
        0x800 | 0x1000 => 0x1800,
        0x2000 | 0x4000 => 0x6000,
        0x8000 | 0x10000 | 0x20000 => 0x38000,
        _ => return orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(362)(t, m),
    };
    let marcas = (t + 0x584) as *mut u32;
    let v = core::ptr::read_unaligned(marcas) & !grupo | m;
    core::ptr::write_unaligned((t + 0x187f) as *mut u32, 0);
    core::ptr::write_unaligned(marcas, v);
    1
}

/// { 0x452fe0 } (cdecl, club_records.cpp): reinicia os recordes do clube `r`: −1.0f em +0x1f1 e −1
/// nos campos de id (dwords, palavras e bytes). Devolve 1. `r` nulo: a original (caixa de erro).
unsafe extern "C" fn reiniciar_recordes(r: usize) -> u32 {
    if r == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(363)(r);
    }
    let d = |o: usize| core::ptr::write_unaligned((r + o) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((r + 0x1f1) as *mut u32, 0xbf800000);
    for o in [1, 8, 0xe, 0x31, 0x54, 0x77, 0x85, 0xa4, 0xc3, 0xe2, 0x101, 0x118, 0x12c, 0x140, 0x154] {
        d(o);
    }
    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210, 0x22b] {
        *((r + o) as *mut u8) = 0xff;
    }
    for o in [0x247, 0x261] {
        core::ptr::write_unaligned((r + o) as *mut u16, 0xffff);
    }
    d(0x2a3);
    d(0x2e2);
    core::ptr::write_unaligned((r + 0x2f9) as *mut u16, 0xffff);
    for o in [0x313, 0x33c, 0x365, 0x36b, 0x37d] {
        d(o);
    }
    1
}

unsafe extern "thiscall" fn comparar_casa_guardada(t: usize, vaga: i8, ataque: i8, faixa: u32, saida: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(361);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, vaga, ataque, faixa, saida), || casa_guardada(t, vaga, ataque, faixa, saida));
    PLACAR.registrar(igual && r == v, || format!("casa_guardada({:#x}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, vaga, ataque, faixa, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_ligar_marca_da_formacao(t: usize, m: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(362);
    let reg = [(t + 0x584, 4), (t + 0x187f, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, m), || ligar_marca_da_formacao(t, m));
    PLACAR.registrar(igual && r == v, || format!("ligar_marca_da_formacao({:#x}, {:#x}) original={} {:x?} rust={} {:x?}", t, m, r, mo, v, mr));
    r
}

unsafe extern "C" fn comparar_reiniciar_recordes(r: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(363);
    let reg = if r == 0 { vec![] } else { vec![(r, 0x381)] };
    let (a, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(r), || reiniciar_recordes(r));
    PLACAR.registrar(igual && a == v, || format!("reiniciar_recordes({:#x}) original={} {:x?} rust={} {:x?}", r, a, mo, v, mr));
    a
}

