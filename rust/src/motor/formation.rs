use super::*;

// ---------- octogesima primeira leva: casa da vaga, byte de criterio da competicao ----------

/// { 0x5b2ae0 } (ret 8, formation.cpp): a casa da vaga 0..10 (palavra +0x12d, ou +0x14e com
/// `segunda`). Vaga fora da faixa: a original (caixa de erro e 1). Devolve so `ax`: no caminho +0x12d
/// a original deixa em cima de `eax` o que o chamador tinha ali.
pub(crate) unsafe extern "thiscall" fn casa_da_vaga(this: usize, vaga: i8, segunda: u32) -> u16 {
    if !(0..=10).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u16>(223)(this, vaga, segunda);
    }
    let base = if segunda != 0 { 0x14e } else { 0x12d };
    core::ptr::read_unaligned((this + base + 2 * vaga as usize) as *const u16)
}

/// { 0x4bff70 } (comp_util.cpp): com a palavra baixa de `b` (ou, sem ela, a de `a`), o resultado
/// de 0x4ba8f0(palavra, 0); senao o byte +0x37 (`tipo` 1) ou +0x52 do registro; sem registro, 1.
pub(crate) unsafe extern "C" fn criterio_da_competicao(registro: usize, a: u32, b: u32, tipo: u8) -> u8 {
    let texto: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_STAGE_NAME_GENDER);
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
pub(crate) unsafe extern "thiscall" fn esquecer_pessoa(this: usize, pessoa: usize) {
    let valida = || *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0;
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
pub(crate) unsafe extern "thiscall" fn quadro_do_tecnico(this: usize, tecnico: usize) -> usize {
    if tecnico == 0 {
        return 0;
    }
    let k = (*(G_STAFF_COUNT as *const i32)).wrapping_sub(*(tecnico as *const i32)) as usize;
    *(this.wrapping_add(k.wrapping_mul(4)).wrapping_sub(4) as *const usize)
}

/// { 0x5b2e80 } (ret 8, formation.cpp): a primeira das 11 casas (+0x12d) com o setor da mascara
/// (0x5b73a0) e o bit 0x200; sem ela, 0. Sem `jogadores`, 1; senao 1 se algum dos 20 (+0x19, ou
/// +0x1a com 0xfe em +0x19) esta naquela vaga.
pub(crate) unsafe extern "thiscall" fn casa_do_setor_ocupada(this: usize, mascara: u32, jogadores: usize) -> u32 {
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(FN_FORMATION_SPLIT_POSITION_CODE);
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
pub(crate) unsafe extern "thiscall" fn vaga_da_casa(this: usize, codigo: u16) -> u32 {
    (0..11u32).find(|&k| core::ptr::read_unaligned((this + 0x12d + 2 * k as usize) as *const u16) == codigo).unwrap_or(0xff)
}

/// marks (+0x5b4) with 4 and 8 become 2; assigned records (+0x5bf) go back to −1; returns 1
/// Original: `clear_assigned` (0x5b5960, formation.cpp, thiscall).
///
/// [pt] { 0x5b5960 } (formation.cpp): as 11 marcas (+0x5b4) com 4 e 8 viram 2, e as fichas designadas
/// (+0x5bf) voltam a -1. Devolve 1.
pub(crate) unsafe extern "thiscall" fn limpar_designados(this: usize) -> u32 {
    for k in 0..11usize {
        let b = (this + 0x5b4 + k) as *mut u8;
        if *b & 4 != 0 && *b & 8 != 0 {
            *b = 2;
        }
        core::ptr::write_unaligned((this + 0x5bf + 4 * k) as *mut i32, -1);
    }
    1
}

/// +0x187b = OR of the bits of the 11 slots (0x5b5a60), with 11 masks swapped for 4 reference formations
/// Original: `formation_mask` (0x5b59a0, formation.cpp, thiscall).
///
/// [pt] { 0x5b59a0 } (formation.cpp): a mascara da formacao (+0x187b) = OU dos bits das 11 casas
/// (0x5b5a60), com onze mascaras trocadas por quatro formacoes de referencia. Devolve a mascara
/// antes da troca (o eax da original).
pub(crate) unsafe extern "thiscall" fn mascara_da_formacao(this: usize) -> u32 {
    let bit: unsafe extern "stdcall" fn(i16) -> u32 = core::mem::transmute(FN_FORMATION_POSITION_TO_SHAPE_BIT);
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
pub(crate) unsafe extern "thiscall" fn designado_da_posicao(this: usize, posicao: i8) -> usize {
    let i = *((this as isize + 0x5bf + 4 * posicao as isize) as *const i32);
    if i > -1 {
        (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)
    } else {
        0
    }
}


// ---------- quinquagesima oitava leva: casas da formacao e craques ----------

/// { 0x5b5290 } (formation.cpp): liga o bit `mascara` (um so) na mascara de casas da posicao
/// (+0x188b + 8 x posicao, 64 bits), apagando antes o grupo dele (bits que se excluem), e zera
/// +0x187f. Posicao fora de 0..=10 ou bit desconhecido: o original mostra uma caixa de erro (sem a
/// caixa aqui), zera 0xb67a34 e devolve 0.
pub(crate) unsafe extern "thiscall" fn gravar_casa(this: usize, posicao: i8, mascara: u64) -> u32 {
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
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
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
pub(crate) unsafe extern "thiscall" fn marcar_craques(this: usize, lado: i8) {
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
            let gravar: unsafe extern "thiscall" fn(usize, i8, u64) -> u32 = core::mem::transmute(FN_FORMATION_SET_PLAYER_INSTRUCTION);
            gravar((this as isize + 0x91e2 + 0x18e3 * lado as isize) as usize, byte(j, 0x19), 0x80_0000);
        }
    }
}

// ---------- centesima quinquagesima nona leva: a formacao (formation.cpp), posicoes do jogador e casas no campo ----------

/// the strong-position mask by the ratings (+0xf..+0x15, level 15 down to 10) and the sides rated 15 (+0x17..+0x19);
/// returns only `ax`
/// Original: `position_mask` (0x5b7d20, formation.cpp, cdecl).
///
/// [pt] { 0x5b7d20 } (cdecl, formation.cpp): a máscara das posições fortes do jogador `p` pelas notas de
/// posição (bytes com sinal): a partir de 15, descendo até 10, o primeiro nível em que alguma passa
/// dá os bits do setor (+0xf goleiro 1, +0x10 líbero 2, +0x11 zagueiro 4, +0x12 volante 8 ou, sem
/// ele, +0x14 meia 0x20 ou +0x13 0x10, +0x15 atacante 0x40); os lados (+0x17 0x800, +0x18 0x80,
/// +0x19 0x200) só contam com nota 15 ou mais (a original repete o mesmo teste a cada nível).
/// Devolve só `ax` (os chamadores usam 16 bits). `p` nulo: a original (caixa de erro e 0).
pub(crate) unsafe extern "C" fn mascara_de_posicoes(p: usize) -> u16 {
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

/// column 0..26 and row 0..35 of the position (sector and side via 0x5b73a0) by attack and band (tables of 4 and 5,
/// nudges of 1 or 5 in the bands, limits)
/// Original: `pitch_cell` (0x5b7e60, formation.cpp, cdecl).
///
/// [pt] { 0x5b7e60 } (cdecl, formation.cpp): a casa no campo (`saida[0]` = coluna 0..26, `saida[1]` =
/// linha 0..35) da posição `mascara` (separada pela 0x5b73a0 em setor e lado), com `ataque` 0/1 e
/// `faixa` = palavra baixa 0..2 (esquerda, meio, direita) e palavra alta 0..3 (a altura na
/// formação). A linha vem do setor (tabelas de 4 pela altura; no ataque, os laterais 0x80/0x800
/// sobem 1, ou 1/4 nos setores 2 e 4); a coluna, do lado (tabelas de 5); depois os encostos nas
/// faixas e os limites. Argumentos fora da faixa: a original (caixa de erro e 0). Devolve 1.
pub(crate) unsafe extern "C" fn casa_no_campo(mascara: u32, ataque: i8, faixa: u32, saida: *mut i16) -> u32 {
    let baixo = faixa as u16 as i16;
    let alto = (faixa >> 16) as u16 as i16;
    if !(0..=1).contains(&ataque) || !(0..=2).contains(&baixo) || !(0..=3).contains(&alto) {
        return orig::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(350)(mascara, ataque, faixa, saida);
    }
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(FN_FORMATION_SPLIT_POSITION_CODE);
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

/// 0 if any of the 11 slots with an open side (0x880) is a striker's, midfielder's, defensive or attacking midfielder's
/// (cells +0x12d/+0x14e and 0x5b2ae0); returns only `al`
/// Original: `slots_without_conflict` (0x5b8960, formation.cpp, thiscall).
///
/// [pt] { 0x5b8960 } (thiscall, formation.cpp): as 11 vagas da formação `this` sem conflito: devolve 0
/// se alguma vaga com lado aberto (0x880 na casa +0x12d ou +0x14e, ver 0x5b2ae0) é também de
/// atacante (0x40, nas duas), de meia (0x20 na +0x12d e na +0x14e pela 0x5b2ae0) ou de volante e
/// meia-atacante pela 0x5b2ae0 (8 e 0x10); senão 1. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn vagas_sem_conflito(this: usize) -> u8 {
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
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

pub(crate) unsafe extern "C" fn comparar_mascara_de_posicoes(p: usize) -> u16 {
    let r = orig::<unsafe extern "C" fn(usize) -> u16>(349)(p);
    let n = mascara_de_posicoes(p);
    PLACAR.registrar(r == n, || format!("mascara_de_posicoes({:#x}) original={:#x} rust={:#x}", p, r, n));
    r
}

pub(crate) unsafe extern "C" fn comparar_casa_no_campo(mascara: u32, ataque: i8, faixa: u32, saida: *mut i16) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(350);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(mascara, ataque, faixa, saida), || casa_no_campo(mascara, ataque, faixa, saida));
    PLACAR.registrar(igual && r == v, || format!("casa_no_campo({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", mascara, ataque, faixa, r, mo, v, mr));
    r
}

comparar_valor!(comparar_vagas_sem_conflito, vagas_sem_conflito, 351, () -> u8);

// ---------- 257a leva: as casas da formação: gravar a coordenada da casa e escolher as posições, a posição repetida e o tipo de marcação (formation.cpp) ----------



/// stores the slot coordinate for the phase and ball zone and picks the general and extra positions closest to the
/// slot's 12 coordinates
/// Original: `Formation::set_slot_coordinate` (0x5b3560, formation.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5b3560 } (thiscall, ret 0x10, formation.cpp): `Formation::set_slot_coordinate(casa, fase, faixa, coord)`:
/// grava a coordenada (dword: x 0..0x1a, y 0..0x23) da casa na fase e na faixa da bola (+0x164, 11 × 2 × 3 × 4
/// dwords) e, para cada fase, acha as posições da tabela 0x9cc368 (palavras até 0xffff) mais perto das 12 coordenadas
/// da casa nessa fase (|Δx·9/27| + |Δy·12/36| contra a casa de cada posição pela 0x5b7e60; dobrada nas posições com o
/// bit 2; os empates entram na lista da fase, até 11 em [esp+0x64] contíguas). A posição geral (+0x12d) fica se
/// estiver numa das listas; senão vira a primeira da fase de menor distância (a 0x5b59a0 refaz a máscara, a casa
/// ganha a marca +0x143 e as outras casas com a mesma posição a perdem). A posição extra (+0x14e) sai da lista da
/// outra fase: a mesma posição, ou a de mesmo lado (0x5b73a0), ou a primeira, ou com duas o meio dos bits das duas.
/// Zera +0x187f. Argumentos fora da faixa: a original (caixa e 0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn coordenada_da_casa(this: usize, casa: u32, fase: u32, faixa: u32, coord: u32) -> u32 {
    let (s, f) = (casa as u8 as i8, fase as u8 as i8);
    let (zx, zy) = (faixa as u16 as i16, (faixa >> 16) as u16 as i16);
    let (cx, cy) = (coord as u16 as i16, (coord >> 16) as u16 as i16);
    if !(0..=10).contains(&s) || !(0..=1).contains(&f) || !(0..=2).contains(&zx) || !(0..=3).contains(&zy)
        || !(0..=0x1a).contains(&cx) || !(0..=0x23).contains(&cy)
    {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(1224)(this, casa, fase, faixa, coord);
    }
    let casa_no: unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32 = core::mem::transmute(FN_FORMATION_GET_DEFAULT_COORDINATE);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(FN_FORMATION_SPLIT_POSITION_CODE);
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_UPDATE_SHAPE_CODE);
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

/// finds another slot with the same general position and sets the output when its flag is lower
/// Original: `Formation::check_duplicate_position` (0x5b3460, formation.cpp, thiscall, ret 8).
///
/// [pt] { 0x5b3460 } (thiscall, ret 8, formation.cpp): `Formation::check_duplicate_position(casa, *saida)`: zera a saída;
/// a primeira outra casa com a mesma posição geral (+0x12d) decide: com marca (+0x143, byte com sinal) menor que a da
/// casa, saída 1 e devolve 1; senão devolve 0. Sem repetida, 1. Casa fora de 0..10: a original (caixa e 0).
pub(crate) unsafe extern "thiscall" fn posicao_repetida_na_formacao(this: usize, casa: u32, saida: *mut u32) -> u32 {
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

/// sets one marking type of the slot (bits 0..2 or 3..4 of byte +0x5b4) and clears +0x187f
/// Original: `Formation::set_marking_type` (0x5b57d0, formation.cpp, thiscall, ret 8).
///
/// [pt] { 0x5b57d0 } (thiscall, ret 8, formation.cpp): `Formation::set_marking_type(casa, marca)`: as marcas 1, 2 e 4
/// trocam os bits 0..2 do byte +0x5b4 da casa; 8 e 0x10, os bits 3..4; zera +0x187f e devolve 1. Outra marca: a
/// original (caixa e 0). A casa não é conferida.
pub(crate) unsafe extern "thiscall" fn tipo_de_marcacao(this: usize, casa: u32, marca: u32) -> u32 {
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
pub(crate) fn cpu11_regioes(this: usize) -> [(usize, usize); 3] {
    [(this + 0x12d, 0x584 - 0x12d), (this + 0x5b4, 11), (this + 0x187b, 8)]
}

/// Comparador: grava só a formação (a 0x5b7e60 e a 0x5b73a0 são puras; a 0x5b59a0 refaz a máscara): lado a lado.
pub(crate) unsafe extern "thiscall" fn comparar_coordenada_da_casa(this: usize, casa: u32, fase: u32, faixa: u32, coord: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(1224);
    let reg = cpu11_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, fase, faixa, coord), || coordenada_da_casa(this, casa, fase, faixa, coord));
    PLACAR.registrar(igual && r == n, || format!("coordenada_da_casa({:x}, {}, {}, {:x}, {:x}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, fase as u8 as i8, faixa, coord, r, n, mo, mr));
    r
}

/// Comparador: pura (grava só a saída): as duas sobre saídas próprias.
pub(crate) unsafe extern "thiscall" fn comparar_posicao_repetida_na_formacao(this: usize, casa: u32, saida: *mut u32) -> u32 {
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
pub(crate) unsafe extern "thiscall" fn comparar_tipo_de_marcacao(this: usize, casa: u32, marca: u32) -> u32 {
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

/// expands each formation slot's positions into the pitch cell tables and their mirrored copy
/// Original: `Formation::build_position_lookup` (0x5b7160, formation.cpp, thiscall).
///
/// [pt] { 0x5b7160 } (thiscall, formation.cpp): o mapa de casas da formação. Para cada uma das 22 linhas
/// k (11 casas × 2 fases) e bloco 3 × 4 (a18, a14) com a posição (i = 12k + 4·a18 + a14) em +0x164
/// (coluna, palavra) e +0x166 (linha), as 3 × 3 células do bloco ganham a coluna × 9 / 27 no nibble
/// baixo e a linha × 12 / 36 no alto da tabela +0x5eb (12 por fila, 9 filas por k); o espelho em +0xf9e
/// (fila e coluna invertidas) recebe (0xb0 − linha × 16) com (0xf8 − coluna) & 0xf. Devolve 1.
pub(crate) unsafe extern "thiscall" fn mapa_de_casas_da_formacao(this: usize) -> u32 {
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


/// O registro (42735 bytes em [this]) da partida que o `humano` (ou o da tela, 0xb74c26 + 0x300 ×
/// [0xb77c7a]) assiste: Ok(0) sem humano; Err se o índice (id − [0xae2364] + 0x10) sai de 0..[this+4].
pub(crate) unsafe fn registro_da_partida_assistida(this: usize, humano: usize) -> Result<usize, ()> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return Ok(0);
    }
    let i = (*(h as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(());
    }
    Ok((*(this as *const usize)).wrapping_add((i as usize).wrapping_mul(42735)))
}


pub(crate) unsafe extern "thiscall" fn comparar_mapa_de_casas_da_formacao(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(540);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x5eb, 0x187b - 0x5eb)], || o(this), || mapa_de_casas_da_formacao(this));
    PLACAR.registrar(igual && r == n, || format!("mapa_de_casas_da_formacao {:x?} x {:x?}", mo, mr));
    r
}



// ---------- centesima sexagesima terceira leva: a formacao (marcas e casas) e o reinicio dos recordes do clube ----------

/// the stored cell of the slot by attack and band
/// Original: `stored_cell` (0x5b3c70, formation.cpp, thiscall).
///
/// [pt] { 0x5b3c70 } (thiscall, ret 0x10, formation.cpp): a casa guardada da vaga `vaga` 0..10 com
/// `ataque` 0/1 e `faixa` (palavra baixa 0..2, alta 0..3): o dword +0x164 + 4×(alta + 4×(baixa +
/// 3×(ataque + 2×vaga))) vai para `*saida`. Devolve 1. Fora das faixas: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn casa_guardada(t: usize, vaga: i8, ataque: i8, faixa: u32, saida: *mut u32) -> u32 {
    let baixo = faixa as u16 as i16;
    let alto = (faixa >> 16) as u16 as i16;
    if !(0..=10).contains(&vaga) || !(0..=1).contains(&ataque) || !(0..=2).contains(&baixo) || !(0..=3).contains(&alto) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(361)(t, vaga, ataque, faixa, saida);
    }
    let i = alto as usize + 4 * (baixo as usize + 3 * (ataque as usize + 2 * vaga as usize));
    *saida = core::ptr::read_unaligned((t + 0x164 + 4 * i) as *const u32);
    1
}

/// sets the flag in +0x584 clearing the same group and zeroes +0x187f
/// Original: `set_formation_flag` (0x5b50d0, formation.cpp, thiscall).
///
/// [pt] { 0x5b50d0 } (thiscall, ret 4, formation.cpp): liga a marca `m` em +0x584 apagando antes as do
/// mesmo grupo (1/2/4/8; 0x10/0x20/0x40; 0x80/0x100; 0x200/0x400; 0x800/0x1000; 0x2000/0x4000;
/// 0x8000/0x10000/0x20000) e zera +0x187f. Devolve 1. Marca fora dos grupos: a original (caixa de
/// erro).
pub(crate) unsafe extern "thiscall" fn ligar_marca_da_formacao(t: usize, m: u32) -> u32 {
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


pub(crate) unsafe extern "thiscall" fn comparar_casa_guardada(t: usize, vaga: i8, ataque: i8, faixa: u32, saida: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(361);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, vaga, ataque, faixa, saida), || casa_guardada(t, vaga, ataque, faixa, saida));
    PLACAR.registrar(igual && r == v, || format!("casa_guardada({:#x}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, vaga, ataque, faixa, r, mo, v, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_ligar_marca_da_formacao(t: usize, m: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(362);
    let reg = [(t + 0x584, 4), (t + 0x187f, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, m), || ligar_marca_da_formacao(t, m));
    PLACAR.registrar(igual && r == v, || format!("ligar_marca_da_formacao({:#x}, {:#x}) original={} {:x?} rust={} {:x?}", t, m, r, mo, v, mr));
    r
}

/// reads a formation from a .pct file (the identifier and version, the inverted password, the creator, the general and
/// extra positions, the coordinates per phase and zone, the team and player instructions, the marking), fixes the
/// instructions of the old versions, redoes the positions of the 11 slots from the coordinates and builds the
/// coordinate map
/// Original: `Formation::load_from_disk` (0x5b5d10, formation.cpp, thiscall).
///
/// [pt] { 0x5b5d10 } (thiscall, ret 0xc, formation.cpp): `Formation::load_from_disk(caminho, quieto, ofuscado)`, pela
/// criação das táticas (0x893dc0: as formações da IA e as pré-definidas, 45 na carreira) e pelas telas de tática.
/// Caminho nulo (linha 0x8d8) ou o arquivo que não abre (0x8e0): 0 (sem `quieto`, a caixa). Lê o identificador (4 bytes;
/// invertido se o marcador do arquivo 0x90c140 não é o nativo; menos 123456789 se ofuscado): fora de 0x98ec54..0x98ec5e
/// "não é formação" (0x8f7), abaixo de 0x98ec59 "versão antiga" (0x900). Depois, na ordem: a senha (+0x33, 50 bytes,
/// cada byte invertido, 0xff fica), o criador (+0x65, 200), a máscara (+0x187b, a partir de 0x98ec5a; antes 0), as 11
/// posições gerais (+0x12d, palavras) e a máscara refeita (0x5b59a0), a ordem de exibição (+0x143, 11 bytes, a partir de
/// 0x98ec5c; antes zeros), as 11 posições extras (+0x14e), as casas (+0x164, 0x420 bytes = 0x210 palavras), nas versões
/// 0x98ec59/0x98ec5a 0x108 bytes descartados, as instruções do time (+0x584, dword), as instruções dos jogadores (abaixo
/// de 0x98ec5e 11 dwords, cada um com 10 ao lado, em +0x188b de 8 em 8; depois 0x58 bytes em +0x188b) e os tipos de
/// marcação (+0x5b4, 11 bytes); as três últimas, se falham, só mostram a caixa. As instruções do time ganham 0x2000
/// (sem 0x4000) e, abaixo de 0x98ec5d, 0x20000. As 11 vagas refazem as posições pela casa guardada (0x5b3c70 e
/// 0x5b3560 com a fase 0 e a faixa 0); o nome (+0) é o do arquivo (_splitpath 0x944cff); o mapa das casas (0x5b7160);
/// +0x187f = 1, +0x1883 = senha não vazia, +0x1887 = 1. Fecha e devolve 1. Os arquivos não nativos trocam a ordem dos
/// bytes (0x43f2d0 palavras, 0x43f300 dwords, 0x43f350 qwords).
pub(crate) unsafe extern "thiscall" fn ler_formacao_do_disco(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    if caminho == 0 {
        if quieto == 0 {
            cqg8_erro(0x8d8);
        }
        return 0;
    }
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let f = fopen(caminho, 0x98f928);
    if f == 0 {
        if quieto == 0 {
            cqg8_erro(0x8e0);
        }
        return 0;
    }
    let marcador: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_UTILS_READ_FILE_FORMAT_MARKER);
    let nativo = marcador(f) != 0xf1ff_fff1;
    let mut id_ = [0u8; 4];
    if cqg8_ler(id_.as_mut_ptr() as usize, 4, f) != 1 {
        return cqg8_falhou(quieto, 0x8ea, f);
    }
    let mut id = u32::from_le_bytes(id_);
    if !nativo {
        id = cqg8_inverter(id);
    }
    if ofuscado != 0 {
        id = id.wrapping_sub(0x75bcd15);
    }
    let id = id as i32;
    if !(0x98ec54..=0x98ec5e).contains(&id) {
        return cqg8_falhou(quieto, 0x8f7, f);
    }
    if id < 0x98ec59 {
        return cqg8_falhou(quieto, 0x900, f);
    }
    let senha = this + 0x33;
    if cqg8_ler(senha, 0x32, f) != 1 {
        return cqg8_falhou(quieto, 0x909, f);
    }
    let strlen = |a: usize| (0..).take_while(|&k| *((a + k) as *const u8) != 0).count();
    if strlen(senha) != 0 {
        let mut k = 0usize;
        loop {
            let b = *((senha + k) as *const u8);
            *((senha + k) as *mut u8) = if b == 0xff { 0xff } else { !b }; // cqg8: a senha invertida
            k += 1;
            if k >= strlen(senha) {
                break;
            }
        }
    }
    if cqg8_ler(this + 0x65, 200, f) != 1 {
        return cqg8_falhou(quieto, 0x915, f);
    }
    let troca16: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_SWAP_BYTES_16_ARRAY);
    if id >= 0x98ec5a {
        if cqg8_ler(this + 0x187b, 4, f) != 1 {
            return cqg8_falhou(quieto, 0x920, f);
        }
        if !nativo {
            write_u32(this + 0x187b, cqg8_inverter(read_u32(this + 0x187b)));
        }
    } else {
        write_u32(this + 0x187b, 0);
    }
    if cqg8_ler(this + 0x12d, 0x16, f) != 1 {
        return cqg8_falhou(quieto, 0x930, f);
    }
    if !nativo {
        troca16(this + 0x12d, 0xb);
    }
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_UPDATE_SHAPE_CODE);
    mascara(this);
    if id >= 0x98ec5c {
        if cqg8_ler(this + 0x143, 0xb, f) != 1 {
            return cqg8_falhou(quieto, 0x940, f);
        }
    } else {
        core::ptr::write_bytes((this + 0x143) as *mut u8, 0, 0xb);
    }
    if cqg8_ler(this + 0x14e, 0x16, f) != 1 {
        return cqg8_falhou(quieto, 0x94f, f);
    }
    if !nativo {
        troca16(this + 0x14e, 0xb);
    }
    if cqg8_ler(this + 0x164, 0x420, f) != 1 {
        return cqg8_falhou(quieto, 0x959, f);
    }
    if !nativo {
        troca16(this + 0x164, 0x210);
    }
    if id == 0x98ec59 || id == 0x98ec5a {
        let mut descarte = [0u8; 0x108];
        if cqg8_ler(descarte.as_mut_ptr() as usize, 0x108, f) != 1 {
            return cqg8_falhou(quieto, 0x968, f);
        }
        if !nativo {
            troca16(descarte.as_mut_ptr() as usize, 0x84);
        }
    }
    let time = this + 0x584;
    if cqg8_ler(time, 4, f) == 1 {
        if !nativo {
            write_u32(time, cqg8_inverter(read_u32(time)));
        }
    } else if quieto == 0 {
        cqg8_erro(0x971);
    }
    if id < 0x98ec5e {
        // ponytail: a original copia o vetor local mesmo quando a leitura falha (sobra de pilha num arquivo truncado);
        // a cópia começa com zeros
        let mut v = [0u32; 11];
        if cqg8_ler(v.as_mut_ptr() as usize, 0x2c, f) == 1 {
            if !nativo {
                let troca32: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_SWAP_BYTES_32_ARRAY);
                troca32(v.as_mut_ptr() as usize, 0xb);
            }
        } else if quieto == 0 {
            cqg8_erro(0x97b);
        }
        for (k, x) in v.iter().enumerate() {
            write_u32(this + 0x188b + 8 * k, *x);
            write_u32(this + 0x188b + 8 * k + 4, 0xa); // cqg8: a instrução padrão das versões antigas
        }
        crate::comparacao::ramo("cqg_8 instrucoes antigas");
    } else if cqg8_ler(this + 0x188b, 0x58, f) == 1 {
        if !nativo {
            let troca64: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_SWAP_BYTES_64_ARRAY);
            troca64(this + 0x188b, 0xb);
        }
    } else if quieto == 0 {
        cqg8_erro(0x988);
    }
    if cqg8_ler(this + 0x5b4, 0xb, f) != 1 && quieto == 0 {
        cqg8_erro(0x98f);
    }
    let mut v = read_u32(time);
    if v & 0x4000 == 0 {
        v |= 0x2000; // cqg8: a instrução 0x2000 sem a 0x4000
        write_u32(time, v);
        crate::comparacao::ramo("cqg_8 instrucao 0x2000");
    }
    if id < 0x98ec5d {
        write_u32(time, read_u32(time) | 0x20000);
    }
    let guardada: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32 = core::mem::transmute(FN_FORMATION_GET_SLOT_COORDINATE);
    let refazer: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) -> u32 = core::mem::transmute(FN_FORMATION_SET_SLOT_COORDINATE);
    let mut c = 0u32;
    for vaga in 0..0xbi32 {
        guardada(this, vaga as i8, 0, 0, &mut c); // cqg8: a casa guardada da fase 0
        crate::comparacao::ramo("cqg_8 posicoes refeitas");
        refazer(this, vaga, 0, 0, c); // cqg8: as posições refeitas pela casa
    }
    let separar: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(FN_CRT_STRTOK);
    separar(caminho, 0, 0, this, 0);
    let mapa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_BUILD_POSITION_LOOKUP);
    mapa(this);
    write_u32(this + 0x187f, 1);
    write_u32(this + 0x1883, (*((this + 0x33) as *const u8) != 0) as u32);
    write_u32(this + 0x1887, 1);
    cqg8_fechar(f);
    1
}

/// Comparador: lado a lado na formação (0x18e3 bytes: até as instruções dos jogadores em +0x188b + 0x58) e na
/// devolução; o arquivo é aberto e lido pelas duas (só leitura).
pub(crate) unsafe extern "thiscall" fn comparar_ler_formacao_do_disco(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1632);
    if this == 0 {
        return o(this, caminho, quieto, ofuscado);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x18e3)], || o(this, caminho, quieto, ofuscado),
        || ler_formacao_do_disco(this, caminho, quieto, ofuscado));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).take(8).collect();
        format!("ler_formacao_do_disco(this {:x}) original={} rust={}, bytes diferentes {:x?}", this, r, n, dif)
    });
    r
}

/// 0 if any slot with an open side is a centre-back's or defensive midfielder's
/// Original: `centre_backs_without_conflict` (0x5b8dc0, formation.cpp, thiscall).
///
/// [pt] { 0x5b8dc0 } (thiscall, formation.cpp): como a 0x5b8960, para os zagueiros: devolve 0 se alguma
/// das 11 vagas com lado aberto (0x880) é de zagueiro (4, na casa +0x12d ou +0x14e) ou de volante
/// (8 na +0x12d; na +0x14e pela 0x5b2ae0); senão 1. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn zagueiros_sem_conflito(this: usize) -> u8 {
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let w = |o: usize| core::ptr::read_unaligned((this + o) as *const u16);
    for i in 0..=10u32 {
        let a = w(0x12d + 2 * i as usize);
        let b = w(0x14e + 2 * i as usize);
        if a & 0x880 != 0 && a & 4 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 4 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 8 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && casa(this, i, 1) & 8 != 0 {
            return 0;
        }
    }
    1
}

/// stores the general or extra position of the slot, the marks, and recomputes the pitch cells without and with the
/// ball (0x5b7e60)
/// Original: `Formation::set_slot_position` (0x5b2ba0, formation.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5b2ba0 } (thiscall, ret 0xc, formation.cpp): `Formation::set_slot_position(casa, posicao, extra)`: sem `extra`,
/// a posição geral da casa (+0x12d, palavras) muda para `posicao` (se mudar: a máscara 0x5b59a0, a marca +0x143 da casa
/// = 1 e a das outras casas com a mesma posição = 0); com ou sem, a posição extra (+0x14e) = `posicao`. Depois, para
/// cada faixa 0..2 e altura 0..3 (a palavra baixa e a alta do argumento da 0x5b7e60), as casas no campo sem e com a
/// bola (+0x164 e +0x194 + 4 × (altura + 4 × faixa + 24 × casa)): com geral = extra, as duas da geral; senão, pelos
/// setores (0x5b73a0): extra de setor abaixo do da geral, sem a bola a extra e com a bola a geral; senão o contrário.
/// Por fim +0x187f = 0. Devolve 1. Casa fora de 0..10: a original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn posicao_na_formacao(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let k = casa as u8 as i8;
    if !(0..=10).contains(&k) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(903)(this, casa, posicao, extra); // caixa: só a original
    }
    let s = k as usize;
    let p = posicao as u16 as i16;
    let geral = this + 0x12d + 2 * s;
    let extra_da_casa = this + 0x14e + 2 * s;
    if extra == 0 && read_i16(geral) != p {
        core::ptr::write_unaligned(geral as *mut i16, p);
        let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_UPDATE_SHAPE_CODE);
        mascara(this);
        *((this + 0x143 + s) as *mut u8) = 1;
        for j in 0..11usize {
            if j != s && read_i16(this + 0x12d + 2 * j) == p {
                *((this + 0x143 + j) as *mut u8) = 0; // a outra casa com a mesma posição
            }
        }
    }
    core::ptr::write_unaligned(extra_da_casa as *mut i16, p);
    // a 0x5b7e60 e a 0x5b73a0 só leem a palavra baixa da máscara (a original empurra o eax com o alto que estiver)
    let casa_no_campo: unsafe extern "C" fn(u32, u32, u32, *mut u32) -> u32 = core::mem::transmute(FN_FORMATION_GET_DEFAULT_COORDINATE);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(FN_FORMATION_SPLIT_POSITION_CODE);
    // as saídas valem de uma volta para a outra, como as locais da original (sem valor só se a 0x5b73a0 falhar
    // logo na primeira: zero aqui)
    let (mut setor_geral, mut setor_extra, mut lado, mut saida) = (0u16, 0u16, 0u16, 0u32);
    if read_i16(geral) == read_i16(extra_da_casa) {
        crate::comparacao::ramo("cpk_7 formacao mesma posicao");
    } else {
        crate::comparacao::ramo("cpk_7 formacao posicao extra");
    }
    for faixa in 0..3usize {
        for altura in 0..4usize {
            let arg = faixa as u32 | (altura as u32) << 16;
            let i = altura + 4 * (faixa + 6 * s);
            let (sem_bola, com_bola) = (this + 0x164 + 4 * i, this + 0x194 + 4 * i);
            let (g, e) = (read_i16(geral) as u16 as u32, read_i16(extra_da_casa) as u16 as u32);
            let (a, b) = if g == e {
                (g, g) // a mesma posição com e sem a bola
            } else {
                separar(g, &mut setor_geral, &mut lado);
                separar(e, &mut setor_extra, &mut lado);
                if (setor_extra as i16) < (setor_geral as i16) { // a extra mais atrás
                    (e, g)
                } else {
                    (g, e)
                }
            };
            casa_no_campo(a, 0, arg, &mut saida);
            core::ptr::write_unaligned(sem_bola as *mut u32, saida);
            casa_no_campo(b, 1, arg, &mut saida);
            core::ptr::write_unaligned(com_bola as *mut u32, saida);
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// Comparador: lado a lado pela formação (+0x12d a +0x584: posições, marcas e casas no campo; a máscara +0x187b e
/// +0x187f) e [0xb67a34] (a 0x5b73a0 zera quando falha).
pub(crate) unsafe extern "thiscall" fn comparar_posicao_na_formacao(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(903);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, posicao, extra);
    }
    let reg = [(this + 0x12d, 0x584 - 0x12d), (this + 0x187b, 8), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, posicao, extra), || posicao_na_formacao(this, casa, posicao, extra));
    PLACAR.registrar(igual && r == n, || format!("posicao_na_formacao({}, {:#x}, {}) {:x?} x {:x?}", casa as u8 as i8, posicao as u16, extra, mo, mr));
    r
}
