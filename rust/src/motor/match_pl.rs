use super::*;

// ---------- ponto flutuante ----------
//
// O x87 do jogo roda em precisao dupla (padrao do Windows): cada operacao arredonda para 53 bits,
// como f64 em SSE2. O que importa reproduzir e onde o original arredonda para float (32 bits):
// `fstp`+`fld` compara o valor ja arredondado; `fst` seguido de comparacao compara o de 53 bits.
// As constantes sao lidas do proprio .rdata do jogo.

pub(crate) unsafe fn f32_em(a: usize) -> f32 {
    *(a as *const f32)
}
pub(crate) unsafe fn f64_em(a: usize) -> f64 {
    *(a as *const f64)
}

/// { 0x6f56c0 } (match_pl.cpp): desgaste do jogador, float em 0x4d (nunca abaixo de zero).
/// codigo -1: `x` x (12, 10, 8 ou 6 pela faixa do valor atual: acima de 9500, 8750, 7750) / 11.
/// Outros codigos: (200 - 8,5 x float de 0xd5) x fator do codigo (switch do original).
pub(crate) unsafe extern "thiscall" fn desgastar(this: usize, x: f32, codigo: i16) {
    let campo = (this + 0x4d) as *mut f32;
    let v = *campo;
    let zero = f32_em(0x968970) as f64;
    if codigo == -1 {
        let k = if v > f32_em(0x96d78c) {
            0x96d690
        } else if v > f32_em(0x96d788) {
            0x968a78
        } else if v > f32_em(0x96b040) {
            0x96d784
        } else {
            0x969084
        };
        let t = x as f64 * f32_em(k) as f64 * f32_em(0x96d780) as f64;
        *campo = (v as f64 - t) as f32;
        if (*campo as f64) < zero {
            *campo = 0.0; // mov dword ptr [ecx + 0x4d], 0
        }
    } else {
        let fator = match codigo {
            19 | 33 => 0x96d770,
            21 | 36 | 46 => 0x96d778,
            _ => 0x96d768,
        };
        let t = (f64_em(0x96a648) - f32_em(this + 0xd5) as f64 * f64_em(0x96a790)) * f64_em(fator);
        let r = v as f64 - t;
        *campo = r as f32;
        if r < zero {
            *campo = 0.0;
        }
    }
}

/// `_ftol` { 0x945640 }: trunca para inteiro (64 bits; o jogo usa a parte baixa). Fora da faixa
/// (infinito, NaN, |x| >= 2^63) o `fistp` do x87 grava o inteiro indefinido, de parte baixa 0.
pub(crate) fn ftol(x: f64) -> i32 {
    if x.is_finite() && x.abs() < 9.223372036854775808e18 {
        x as i64 as i32
    } else {
        0
    }
}

/// { 0x6fa140 } (match_pl.cpp): o jogador esta virado para (a, b)? Compara a direcao dele (0x19a)
/// com o angulo da tabela, com tolerancia de 0x10c x 2,25 + 45 graus.
pub(crate) unsafe extern "thiscall" fn virado_para(this: usize, a: i8, b: i8) -> u8 {
    let (dl, dc) = (a.wrapping_sub(byte(this, 0x102)), b.wrapping_sub(byte(this, 0x103)));
    if dl == 0 && dc == 0 {
        return 0;
    }
    let dir = *((this + 0x19a) as *const i16);
    let alvo = *((TABELA_GLOBAL as isize + 2 * (dl as isize * 24 + dc as isize)) as *const i16);
    if dir == alvo {
        return 1;
    }
    let tolerancia = ftol(byte(this, 0x10c) as f64 * f64_em(0x968ec8) + f64_em(0x968d68)) as i16;
    let mut d = (dir as i32 - alvo as i32).abs();
    if d as i16 > 0xb4 {
        d = 0x168 - d;
    }
    ((d as i16) < tolerancia) as u8
}

// ---------- sexta leva: distancias ----------

/// Celula da tabela de distancias (0xa15e44, 9 por linha). Diferencas com o mesmo calculo do
/// original: valor absoluto em 32 bits e depois o byte baixo com sinal.
pub(crate) unsafe fn distancia_tabela(dl: i32, dc: i32) -> f32 {
    let lin = dl.abs() as i8 as isize;
    let col = dc.abs() as i8 as isize;
    *((0xa15e44 + 4 * (lin * 9 + col)) as *const f32)
}

/// { 0x6f09b0 } (match_pl.cpp): distancia (tabela) do jogador ao ponto (a, b).
pub(crate) unsafe extern "thiscall" fn distancia_ao_ponto(this: usize, a: i8, b: i8) -> f32 {
    distancia_tabela(byte(this, 0x103) as i32 - b as i32, byte(this, 0x102) as i32 - a as i32)
}

/// to (4, 11) for side 0, (4, 0) for the other
/// Original: `distance_to_goal` (0x6fa1e0, thiscall, `float` in st0).
///
/// [pt] { 0x6fa1e0 } (match_pl.cpp): distancia ao gol que o jogador ataca (coluna 4; linha 11 do lado 0, 0 do outro).
pub(crate) unsafe extern "thiscall" fn distancia_ao_gol(this: usize) -> f32 {
    let alvo = if lado(this) == 0 { 0xb } else { 0 };
    distancia_tabela(byte(this, 0x103) as i32 - alvo, byte(this, 0x102) as i32 - 4)
}

/// { 0x6cb780 } (match_eng.cpp): 0 se o jogador esta impedido de seguir (0x70dfa0, marca do
/// motor na casa dele) ou se algum adversario ativo esta a distancia do gol <= limite; 2 se algum
/// colega ativo esta estritamente mais perto; senao 1. Limite: distancia dele ao gol + 0,5, e mais
/// 1,0 fora da area (arredondado para float a cada passo, como o `fstp` do original). Os blocos
/// de 20 jogadores de cada lado ficam em motor + 0x4796 + 8920 x lado, de 0x1be em 0x1be.
pub(crate) unsafe extern "thiscall" fn mais_perto_do_gol(this: usize, j: usize) -> u8 {
    let f6f4650: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_CAN_SEE_OPPONENT_GOAL);
    let f70dfa0: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_OWN_HALF);
    let f6fa1e0: unsafe extern "thiscall" fn(usize) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_OPPONENT_GOAL);
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let f6f09b0: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    if f70dfa0(j) != 0 {
        return 0;
    }
    let s = lado(j);
    let outro = (s == 0) as isize;
    let casa = this as isize + outro + 2 * 45 * (byte(j, 0x103) as isize + 12 * byte(j, 0x102) as isize) + 0x21b6;
    if *(casa as *const u8) != 0 {
        return 0;
    }
    let mut limite = (f6fa1e0(j) as f64 + f64_em(0x968968)) as f32;
    if f6f4650(j) == 0 {
        limite = (limite as f64 + f64_em(0x967890)) as f32;
    }
    let linha: i8 = if s == 0 { 0xb } else { 0 };
    let bloco = |l: isize| (this as isize + 8920 * l + 0x4796) as usize;
    for k in 0..20 {
        let p = bloco(outro) + k * 0x1be;
        if byte(p, 0x19) >= 0 && f717410(p) != 1 && !(f6f09b0(p, 4, linha) > limite) {
            return 0; // C0|C3: menor, igual ou sem ordem
        }
    }
    for k in 0..20 {
        let p = bloco(s as isize) + k * 0x1be;
        if byte(p, 0x19) >= 0 && f717410(p) != 1 && !(f6f09b0(p, 4, linha) >= limite) {
            return 2; // C0: menor ou sem ordem
        }
    }
    1
}

// ---------- oitava leva: posicionamento ----------

/// { 0x5b5270 }: marcas taticas (64 bits) de uma posicao: [this + 0x188b + 8 x posicao].
pub(crate) unsafe extern "thiscall" fn marcas_da_posicao(this: usize, posicao: i8) -> u64 {
    *((this as isize + 0x188b + 8 * posicao as isize) as *const u64)
}

/// Marcas da posicao do jogador no bloco do lado dele (0x91e2 + 6371 x lado), por 0x5b5270.
pub(crate) unsafe fn marcas_do_jogador(j: usize) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let bloco = (*((j + 0x19e) as *const usize) as isize + 0x91e2 + 6371 * lado(j) as isize) as usize;
    f(bloco, byte(j, 0x19))
}

/// `tabela_a` do motor do jogador e 1 (pelo endereco do jogo); sem motor, nao.
pub(crate) unsafe fn tabela_e_um(j: usize) -> bool {
    let motor = *((j + 0x19e) as *const usize);
    let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    motor != 0 && f(motor, j) == 1
}

/// 1 (0x20000); 0 if `tabela_a` = 1; 2 (0x40000000); 0
/// Original: `attacking_role` (0x716270, the player's marks (block 0x91e2 + 6371 × side), thiscall, `al`).
///
/// [pt] { 0x716270 } (match_pl.cpp): papel no ataque pelas marcas da posicao: 1 (marca 0x20000), 0 se
/// `tabela_a` = 1, 2 (marca 0x40000000) ou 0.
pub(crate) unsafe extern "thiscall" fn papel_no_ataque(j: usize) -> u8 {
    if marcas_do_jogador(j) & 0x20000 != 0 {
        return 1;
    }
    if tabela_e_um(j) {
        return 0;
    }
    if marcas_do_jogador(j) & 0x4000_0000 != 0 { 2 } else { 0 }
}

/// 0 if `tabela_a` = 1; 3 (0x80000000); 4 (0x80000); 0
/// Original: `defending_role` (0x716310, thiscall, `al`).
///
/// [pt] { 0x716310 } (match_pl.cpp): papel na defesa: 0 se `tabela_a` = 1; 3 (marca 0x80000000),
/// 4 (marca 0x80000) ou 0.
pub(crate) unsafe extern "thiscall" fn papel_na_defesa(j: usize) -> u8 {
    if tabela_e_um(j) {
        return 0;
    }
    let m = marcas_do_jogador(j);
    if m & 0x8000_0000 != 0 {
        3
    } else if m & 0x80000 != 0 {
        4
    } else {
        0
    }
}

/// { 0x6c2be0 } (match_eng.cpp): onde o jogador deve se posicionar (saidas x e y). Pela bola
/// (0x8ea7/0x8ea8, limitada a 9 x 12), pelo lado e se o time dele esta atacando, le a casa na
/// tabela de formacao do lado (x no meio byte baixo, y no alto). Com 0x8eb1 > 0, a posicao atual
/// quando o papel (ataque ou defesa) manda ficar, e tudo limitado a grade. Senao, ajustes pelas
/// marcas do lado (0x9766 + 6371 x lado): um passo a frente ou atras, e acompanhar a linha da bola.
pub(crate) unsafe extern "thiscall" fn posicionar(this: usize, j: usize, saida_x: *mut i8, saida_y: *mut i8, _a4: u32) {
    let bola_x = byte(this, 0x8ea7);
    let bola_y = byte(this, 0x8ea8);
    let bx = if bola_x < 0 { 0 } else if bola_x >= 9 { 8 } else { bola_x };
    let by = if bola_y < 0 { 0 } else if bola_y >= 12 { 11 } else { bola_y };
    let lado1 = (lado(j) == 1) as i8;
    let s = lado(j);
    let posse = byte(this, 0x8eae);
    // atacando: o time dele tem a posse (ou o jogador guardado em 0xf57a e do lado dele), ou a
    // bola esta na entrada da area do adversario com 0x8ea9 marcado
    let atacando: i8 = if posse as i32 == (s == 0) as i32 {
        0
    } else if s == posse {
        1
    } else {
        let guardado = *((this + 0xf57a) as *const usize);
        if guardado != 0 && byte(guardado, 0x27) == s {
            1
        } else {
            let faixa_y = if s == 0 { 10..=11 } else { 0..=1 };
            ((2..=6).contains(&bola_x) && faixa_y.contains(&bola_y) && byte(this, 0x8ea9) != -1) as i8
        }
    };
    let formacao = |cel: usize| -> u8 {
        let tabela = *((this + 0xf56e + 4 * lado1 as usize) as *const usize);
        let e = 9 * (atacando as isize + 2 * byte(j, 0x19) as isize) + bx as isize;
        *((tabela as isize + 12 * e + cel as isize) as *const u8)
    };
    let limitar_y = |v: i8| if v < 0 { 0 } else if v >= 12 { 11 } else { v };
    let limitar_x = |v: i8| if v < 0 { 0 } else if v >= 9 { 8 } else { v };
    if byte(this, 0x8eb1) > 0 {
        let papel = if atacando != 0 { papel_por_endereco(0x716270, j) } else { papel_por_endereco(0x716310, j) };
        if papel != 0 {
            *saida_x = byte(j, 0x102);
            *saida_y = byte(j, 0x103);
        } else {
            let c = formacao(by as usize);
            *saida_x = (c & 0xf) as i8;
            *saida_y = (c >> 4) as i8;
        }
        *saida_y = limitar_y(*saida_y);
        *saida_x = limitar_x(*saida_x);
        return;
    }
    let c = formacao(by as usize);
    *saida_x = (c & 0xf) as i8;
    *saida_y = (c >> 4) as i8;
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    if f717410(j) == 1 {
        return;
    }
    let marcas = (this as isize + 6371 * lado1 as isize + 0x9766) as *const u32;
    let parado = byte(this, lado1 as usize + 6) == 0;
    let passo = |para_frente: bool| {
        let d: i8 = if para_frente { 1 } else { -1 };
        *saida_y = limitar_y((*saida_y).wrapping_add(d));
        *saida_x = limitar_x(*saida_x);
    };
    let m = *marcas;
    if m & 0x1000 != 0 && parado {
        passo(lado1 != 0);
    } else if m & 0x10000 != 0 && atacando != 0 {
        passo(lado1 == 0);
    } else if m & 0x8000 != 0 && atacando == 0 {
        passo(lado1 != 0);
    }
    if *marcas & 0x4000 == 0 {
        return;
    }
    if atacando != 0 && parado {
        if lado1 != 0 {
            let v = (*saida_y).wrapping_add(2);
            *saida_y = if v > 0xb { 0xb } else { v };
        } else {
            let v = (*saida_y).wrapping_add(-2);
            *saida_y = if v < 0 { 0 } else { v };
        }
        return;
    }
    let linha = bola_y as i32;
    let atual = *saida_y as i32;
    let v = if lado1 != 0 {
        let v = (linha + 2).max(atual) as i8;
        let v = if v >= 9 { v } else { 9 };
        let v = (v as i32).min(linha + 4) as i8;
        if v > 0xb { 0xb } else { v }
    } else {
        let v = (linha - 2).min(atual) as i8;
        let v = if v <= 2 { v } else { 2 };
        let v = (v as i32).max(linha - 4) as i8;
        if v < 0 { 0 } else { v }
    };
    *saida_y = v;
}

/// side 1, largest y (0x103) among the active ones starting from 6, plus 1, at 0x8eaf; side 0, smallest y starting from
/// 5, minus 1, at 0x8eb0 (the fields read by `diferenca_lado` and `diferenca_jogador`). Active: position >= 0 and
/// `tabela_do_jogador` != 1. The original has both 20-player loops unrolled (1.7 KB)
/// Original: `defensive_lines` (0x6c3aa0, offside, thiscall).
///
/// [pt] { 0x6c3aa0 } (match_eng.cpp): as duas linhas defensivas (impedimento). Lado 1: maior y (0x103)
/// entre os ativos, partindo de 6, mais 1, em 0x8eaf. Lado 0: menor y, partindo de 5, menos 1, em
/// 0x8eb0. Ativo: posicao >= 0 e `tabela_do_jogador` (pelo endereco) diferente de 1. O original tem
/// os dois lacos de 20 jogadores desenrolados.
pub(crate) unsafe extern "thiscall" fn linhas_defensivas(this: usize) {
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let ativos = |lado: usize| {
        (0..20).map(move |k| (this + 0x4796 + 8920 * lado + k * 0x1be) as usize).filter(move |&p| byte(p, 0x19) >= 0 && f717410(p) != 1)
    };
    let maior = ativos(1).fold(6i8, |m, p| m.max(byte(p, 0x103)));
    *((this + 0x8eaf) as *mut i8) = maior.wrapping_add(1);
    let menor = ativos(0).fold(5i8, |m, p| m.min(byte(p, 0x103)));
    *((this + 0x8eb0) as *mut i8) = menor.wrapping_sub(1);
}

// ---------- decima leva: atualizacao do jogador a cada lance ----------

/// ranges by the attributes and, by the tactical role (bits 1, 2, 4, 8, 0x10, 0x20, 0x40 of `tabela_a`; "strong" = bits
/// 0x880 of the whole word), draws from 1/75 to 1/1000, floating-point ratios between attributes and the side's state
/// (0xf5bd/0xf5f3). Transcribed block by block, with the original's addresses in the comments; the shared endings
/// became local functions
/// Original: `update_player` (0x6f4aa0, 2.8 KB, 908 instructions. Counter 0x2b (decreases down to 0); float 0x1a2 (× 0.85, floor 2 × (25 − the person's attribute 0x5d), ceiling 100); word 0x35 (disposition, ~4,000 to ~8,750), thiscall).
///
/// [pt] { 0x6f4aa0 } (match_pl.cpp): atualiza, a cada lance, o contador 0x2b (decresce ate 0), o float
/// 0x1a2 (x 0,85, com piso 2 x (25 - atributo 0x5d da pessoa) e teto 100) e a palavra 0x35
/// (disposicao): faixas pelos atributos e, pelo papel tatico (bits de `tabela_a`), sorteios de
/// 1/75 a 1/1000, razoes em ponto flutuante entre atributos e o estado do lado (0xf5bd/0xf5f3).
/// Transcrita bloco a bloco (enderecos do original nos comentarios); os finais compartilhados
/// viram as funcoes locais `ate`.
pub(crate) unsafe extern "thiscall" fn atualizar_jogador(this: usize) {
    let u = |o: usize| *((this + o) as *const u8);
    let i = |o: usize| *((this + o) as *const i8);
    let w35 = (this + 0x35) as *mut i16;
    // 0x6f4aa8: contador 0x2b
    let c = i(0x2b);
    if c > 0 {
        *((this + 0x2b) as *mut i8) = c - 1;
    } else if c < 0 {
        *((this + 0x2b) as *mut i8) = 0;
    }
    // 0x6f4abc: float 0x1a2
    let f1a2 = (this + 0x1a2) as *mut f32;
    let mut st = *f1a2 as f64 * f32_em(0x96d364) as f64;
    *f1a2 = st as f32;
    let pessoa = *((this + 0x69) as *const usize);
    let piso = ((0x19 - *((pessoa + 0x5d) as *const i8) as i32) * 2) as f32;
    if !(st > piso as f64) {
        st = piso as f64;
    }
    *f1a2 = st as f32;
    if st > f64_em(0x9688a8) {
        *f1a2 = f32::from_bits(0x42c80000);
    }
    // 0x6f4b18
    if (u(0x10) as i32 + u(0xc) as i32) < 2 && u(6) < 7 && u(3) < 4 && u(0x11) < 6 {
        let edi = *w35 as i32;
        if edi > sortear(0x5dc) + 0x203a {
            *w35 = ftol((edi - 0x203a) as f64 * f64_em(0x96d5c0) + f64_em(0x96d760)) as i16;
        }
    }
    // 0x6f4b78
    let a1 = u(1) as i32;
    if a1 > 0x32 {
        let mut cx = (*w35 as i32).max((a1 + 20 * u(3) as i32 + 0x113) * 20) as i16;
        *w35 = cx;
        let a2 = u(2) as i32;
        if a2 != 0 {
            let razao = a1 as f64 / a2 as f64;
            if razao > f64_em(0x969040) && razao * f32_em(0x96d2dc) as f64 + f32_em(0x96d75c) as f64 > cx as f64 {
                cx = cx.wrapping_add(5);
                *w35 = cx;
            }
        }
    }
    // 0x6f4bfd / 0x6f4c2b
    let d10 = u(0x10) as i32;
    if d10 != 0 {
        *w35 = (425 * d10 + 0x1770).max(*w35 as i32) as i16;
    }
    let dc = u(0xc) as i32;
    if dc != 0 && dc + d10 > 1 {
        *w35 = (375 * (dc + d10) + 0x1964).max(*w35 as i32) as i16;
    }
    // finais compartilhados
    let ate = move |lim: i16, passo: i16| {
        let ax = *w35;
        if ax < lim {
            *w35 = ax + passo;
        }
    };
    let acima = move |lim: i16, passo: i16| {
        let ax = *w35;
        if ax > lim {
            *w35 = ax - passo;
        }
    };
    let mais5_ou_1 = move || {
        let ax = *w35;
        if ax >= 0x1676 {
            if ax < 0x1a5e {
                *w35 = ax + 1;
            }
        } else {
            *w35 = ax + 5;
        }
    };
    let soma_5c = move |c: i32| {
        if (*w35 as i32) < c * 500 + 0x1a5e {
            *w35 = (*w35).wrapping_add((5 * c) as i16);
        }
    };
    let quieto = || u(6) == 0 && u(9) == 0 && u(3) == 0 && u(0x10) == 0;
    let motor = *((this + 0x19e) as *const usize);
    let f6c38c0: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let papel = if motor != 0 { f6c38c0(motor, this) as u8 } else { 0 };
    let estado_lado = || {
        if lado(this) != 0 { *((motor + 0xf5bd) as *const i8) } else { *((motor + 0xf5f3) as *const i8) }
    };
    let byte_lado = |motor: usize| *((motor + 54 * (lado(this) == 0) as usize + 0xf5bf) as *const u8) as i32;
    if papel & 1 != 0 {
        // 0x6f4c7e
        let f = u(0xf) as i32;
        if sortear(f * f) == 0 && *w35 < 0x1cb6 {
            *w35 += 1;
        }
        let a1f = i(0x1f);
        if a1f != -1 && a1f > 0x1e {
            if u(0xf) == 0 {
                ate(0x1b58, 1);
            }
            return;
        }
        // 0x6f4cc7
        let b = byte_lado(motor);
        *w35 = ((b + 0xe) * 500).min(*w35 as i32) as i16;
        let cf = u(0xf);
        if cf == 0 {
            *w35 = ((b + 0x14) * 300).max(*w35 as i32) as i16;
            let lim = (if (b as u8) < 2 { b } else { 2 } + 0x1c) * 0xfa;
            if (*w35 as i32) < lim {
                *w35 += 1;
            }
            return;
        }
        // 0x6f4d5f
        if *w35 <= 0x109a {
            return;
        }
        if cf > 1 && sortear(cf as i32) != 0 {
            if u(0xf) > 2 && *w35 > 0x1e46 {
                *w35 = 0x1e46;
                return;
            }
            let ax = *w35;
            *w35 = if ax > 0x222e {
                ax - 0x19
            } else if ax > 0x1c52 {
                ax - u(0xf) as i16
            } else {
                ax - 1
            };
            return;
        }
        // 0x6f4dcc
        if (*w35 as i32) <= (byte_lado(motor) + 0x19) * 0xfa {
            return;
        }
        let q = *((motor + 0x8ed0) as *const i16) as i32 / 11;
        if sortear(0x5a - q) == 0 {
            *w35 -= 1;
        }
        return;
    }
    if papel & 2 != 0 {
        // 0x6f4e53
        if sortear(0x4b) == 0 {
            *w35 -= 1;
        }
        if quieto() {
            acima(0x1d4c, 10);
        }
        if estado_lado() != 0 {
            return;
        }
        ate(0x1676, 1);
        return;
    }
    if papel & 4 != 0 {
        // 0x6f4ee4
        if sortear(0x4b) == 0 {
            *w35 -= 1;
        }
        let (a4, a7) = (u(4) as i32, u(7) as i32);
        if a4 + a7 > 9 {
            let mut aa = ((u(5) as i32 + u(8) as i32) as f64 / (a4 + a7) as f64) as f32;
            let s96 = u(9) as i32 + u(6) as i32;
            if s96 != 0 {
                aa = (s96 as f64 * f64_em(0x967878) + aa as f64) as f32;
            }
            let ajustar = |aa: f32, num: i32, den: i32, fator_forte: usize| -> f32 {
                let st = num as f64 / den as f64;
                let guardado = st as f32;
                if st < f64_em(0x96a5d0) || st.is_nan() {
                    let k = if papel_word(motor, this) & 0x880 != 0 { fator_forte } else { 0x9689e0 };
                    (aa as f64 - (guardado as f64 - f64_em(0x96a5d0)) * f64_em(k)) as f32
                } else {
                    aa
                }
            };
            if a4 > 4 {
                aa = ajustar(aa, u(5) as i32, a4, 0x968918);
            }
            if a7 > 4 {
                aa = ajustar(aa, u(8) as i32, a7, 0x967880);
            }
            // 0x6f502a
            let v = estado_lado() as i16;
            let mut cx = 0x1di16.wrapping_sub(v).wrapping_mul(0xfa);
            if u(0xc) != 0 {
                cx = cx.wrapping_add((u(0xc) as i16).wrapping_mul(0x1f4));
            }
            if u(0x10) != 0 {
                cx = cx.wrapping_add((u(0x10) as i16).wrapping_mul(0xfa));
            }
            let a = aa as f64;
            if a < f64_em(0x968fe0) || a.is_nan() {
                if *w35 > cx {
                    *w35 = cx;
                }
            } else if a > f64_em(0x96afb8) && *w35 < cx {
                *w35 = cx;
            }
        }
        // 0x6f50b7
        let parado6 = u(6) == 0;
        if quieto() {
            acima(0x1d4c, 10);
        }
        let v = estado_lado();
        let sem_nada = parado6 && u(9) == 0 && u(3) == 0 && u(0x10) == 0 && u(0xc) == 0;
        if v == 0 {
            if sem_nada {
                let a2 = u(2) as i32;
                if a2 <= 0x14 {
                    ate(0x1676, 1);
                    return;
                }
                let r = u(1) as i32 as f64 / a2 as f64;
                if !(r > f64_em(0x969040)) {
                    ate(0x1676, 1);
                    return;
                }
            }
            mais5_ou_1();
            return;
        }
        // 0x6f5195
        if v == 1 {
            if sem_nada {
                acima(0x1c52, 1);
            } else {
                ate(0x186a, 1);
            }
            return;
        }
        // 0x6f51f9
        let lim = (0x1fi16.wrapping_sub(v as i16).wrapping_mul(0xfa)).max(0x1a5e);
        if *w35 <= lim {
            return;
        }
        let forte = papel_word(motor, this) & 0x880 != 0;
        if forte && !sem_nada {
            *w35 -= 1;
        } else if forte || !sem_nada {
            *w35 = (*w35).wrapping_sub(v as i16);
        } else {
            *w35 = (*w35).wrapping_sub((v as i16).wrapping_mul(5));
        }
        return;
    }
    if papel & 8 != 0 {
        // 0x6f5332
        if sortear(0x1f4) == 0 {
            *w35 -= 1;
        }
        let tres = u(6) == 0 && u(9) == 0 && u(3) == 0;
        if tres {
            acima(0x1d4c, 10);
        }
        if estado_lado() != 0 {
            return;
        }
        if tres && u(0x10) == 0 {
            ate(0x186a, 1);
        } else {
            mais5_ou_1();
        }
        return;
    }
    if papel & 0x10 != 0 {
        // 0x6f540f
        if sortear(0x3e8) == 0 {
            *w35 -= 1;
        }
        if u(0xc) != 0 || u(0x10) != 0 {
            return;
        }
        if u(3) != 0 || u(0xb) != 0 {
            acima(0x1c52, 5);
        } else {
            acima(0x1c52, 10);
        }
        return;
    }
    if papel & 0x20 != 0 {
        // 0x6f5469
        if sortear(0x1f4) == 0 {
            *w35 -= 1;
        }
        let c = u(0xc);
        if c != 0 {
            soma_5c(c as i32);
        } else if u(0x10) != 0 {
            // 0x6f54ba: al = 0
        } else if u(3) != 0 || u(0xb) != 0 {
            acima(0x1c52, 5);
        } else {
            acima(0x1c52, 10);
        }
        return;
    }
    if papel & 0x40 != 0 {
        // 0x6f5515
        if sortear(0x96) == 0 {
            *w35 -= 1;
        }
        let c = u(0xc);
        if c != 0 {
            soma_5c(c as i32);
        } else if u(0x10) != 0 || u(0xb) != 0 {
            // 0x6f557d: al = 0
        } else if u(3) != 0 || u(9) != 0 {
            acima(0x1e46, 5);
        } else {
            acima(0x1c52, 10);
        }
    }
}

// ---------- decima segunda leva: acessores do estado da partida e extremos ----------

leitor!(ler_8ea7, 0x8ea7); // { 0x700320 } bola x
leitor!(ler_8ea8, 0x8ea8); // { 0x700330 } bola y
leitor!(ler_8ea9, 0x8ea9); // { 0x700340 }
leitor!(ler_8eaa, 0x8eaa); // { 0x700380 }
leitor!(ler_8eb2, 0x8eb2); // { 0x700400 }
gravador!(gravar_8ea9, 0x8ea9); // { 0x700350 }
gravador!(gravar_8ea7, 0x8ea7); // { 0x700360 }
gravador!(gravar_8ea8, 0x8ea8); // { 0x700370 }
gravador!(gravar_8eaa, 0x8eaa); // { 0x700390 }

/// { 0x7003a0 }: grava a palavra 0x8ed8.
pub(crate) unsafe extern "thiscall" fn gravar_8ed8(this: usize, v: u16) {
    *((this + 0x8ed8) as *mut u16) = v;
}

/// { 0x700410 }: le o ponteiro 0x8f7a.
pub(crate) unsafe extern "thiscall" fn ler_8f7a(this: usize) -> u32 {
    *((this + 0x8f7a) as *const u32)
}

/// { 0x7003b0 } (match_pl.cpp): registra o lado com a posse (0x8eab). Exceto -1, soma 1 ao
/// contador do lado (0x8d46 + lado) e tira 10 do contador do outro (0x8d46 + (lado == 0)), se positivo.
pub(crate) unsafe extern "thiscall" fn registrar_posse(this: usize, v: i8) {
    *((this + 0x8eab) as *mut i8) = v;
    if v == -1 {
        return;
    }
    let meu = (this as isize + 0x8d46 + v as isize) as *mut i8;
    *meu = (*meu).wrapping_add(1);
    let outro = (this + 0x8d46 + (v == 0) as usize) as *mut i8;
    if *outro > 0 {
        *outro = (*outro).wrapping_sub(10);
    }
}

/// { 0x714c60 } (match_pl.cpp): o ponteiro esta na lista do objeto (0x1ae, quantidade em 0x101)?
pub(crate) unsafe extern "thiscall" fn na_lista(this: usize, alvo: usize) -> u8 {
    let n = *((this + 0x101) as *const i8);
    (0..n.max(0) as usize).any(|k| *((this + 0x1ae + 4 * k) as *const usize) == alvo) as u8
}

/// players with the smallest and largest y on each side (0xf58e, 0xf596, 0xf592, 0xf59a); with no candidate the
/// previous value stays
/// Original: `store_extremes` (0x6cd660, thiscall).
///
/// [pt] { 0x6cd660 } (match_eng.cpp): os jogadores extremos de cada lado entre os ativos (posicao >= 0 e,
/// em 0..=10, `tabela_a` diferente de 1): lado 0, menor y em 0xf58e e maior em 0xf596; lado 1,
/// maior y em 0xf592 e menor em 0xf59a. Sem candidato, o valor anterior fica.
pub(crate) unsafe extern "thiscall" fn guardar_extremos(this: usize) {
    for (lado_b, campo_menor, campo_maior) in [(0usize, 0xf58eusize, 0xf596usize), (1, 0xf59a, 0xf592)] {
        let (mut menor, mut maior) = (12i8, -1i8);
        for k in 0..20 {
            let p = this + 0x4796 + 8920 * lado_b + k * 0x1be;
            let pos = byte(p, 0x19);
            if pos < 0 || (pos <= 10 && tabela_na(this, p, pos) == 1) {
                continue;
            }
            let y = byte(p, 0x103);
            if y < menor {
                menor = y;
                *((this + campo_menor) as *mut usize) = p;
            }
            if y > maior {
                maior = y;
                *((this + campo_maior) as *mut usize) = p;
            }
        }
    }
}

// ---------- decima quinta leva: acessores pequenos ----------

/// the player's word 0x19a
/// Original: `direction` (0x6f4a30, thiscall, `ax`).
///
/// [pt] { 0x6f4a30 } (match_pl.cpp): direcao do jogador (palavra 0x19a).
pub(crate) unsafe extern "thiscall" fn direcao(this: usize) -> u16 {
    *((this + 0x19a) as *const u16)
}

/// { 0x6ce0f0 } (match_eng.cpp): zera o guardado do lado (0xf59e para o lado 0, senao 0xf5a2).
pub(crate) unsafe extern "thiscall" fn zerar_guardado(this: usize, lado_b: u8) {
    *((this + if lado_b == 0 { 0xf59e } else { 0xf5a2 }) as *mut u32) = 0;
}

/// x (0x102) outside 2..=6
/// Original: `x_off_centre` (0x70df20, thiscall, `al`).
///
/// [pt] { 0x70df20 } (match_pl.cpp): 1 com o x do jogador (0x102) fora de 2..=6.
pub(crate) unsafe extern "thiscall" fn x_fora_do_meio(this: usize) -> u8 {
    !(2..=6).contains(&byte(this, 0x102)) as u8
}

/// player == motor[0xf582]
/// Original: `is_the_stored` (0x7103c0, thiscall, `al`).
///
/// [pt] { 0x7103c0 } (match_pl.cpp): o jogador e o guardado em 0xf582 do motor dele.
pub(crate) unsafe extern "thiscall" fn e_o_guardado(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    (this == *((motor + 0xf582) as *const usize)) as u8
}

/// 0xff with no move; otherwise the move's player is on side 0
/// Original: `side_of_move` (0x6bba50, thiscall, `al`).
///
/// [pt] { 0x6bba50 } (match_eng.cpp): lado 0 do jogador em 0x4761 (1 se lado 0, senao 0); 0xff sem
/// lance (0x475a zerado) ou sem jogador.
pub(crate) unsafe extern "thiscall" fn lado_do_lance(this: usize) -> u8 {
    let p = *((this + 0x4761) as *const usize);
    if byte(this, 0x475a) == 0 || p == 0 {
        return 0xff;
    }
    (byte(p, 0x27) == 0) as u8
}

/// { 0x6c2fe0 } (match_eng.cpp): endereco do jogador `k` do bloco do `lado`.
pub(crate) unsafe extern "thiscall" fn jogador_no_bloco(this: usize, lado_b: i8, k: i8) -> usize {
    (this as isize + 0x4796 + 0x1be * (lado_b as isize * 20 + k as isize)) as usize
}

/// { 0x6c3920 } (match_eng.cpp): como `tabela_a`, pela posicao em 0x1a.
pub(crate) unsafe extern "thiscall" fn tabela_a_1a(this: usize, jogador: usize) -> u16 {
    let posicao = byte(jogador, 0x1a);
    if !(0..=10).contains(&posicao) {
        return 0;
    }
    let tabela = *((this as isize + 0x8ebc + byte(jogador, 0x27) as isize * 4) as *const usize) as *const u16;
    *tabela.add(posicao as usize)
}

/// 0x8ed4 outside 0x1ef/0x3de and bit 0x80 of the match's byte 0x38
/// Original: `marked_moment` (0x6d4250, thiscall, `eax`).
///
/// [pt] { 0x6d4250 } (match_eng.cpp): 1 com o momento (0x8ed4) fora de 0x1ef e 0x3de e o bit 0x80 do
/// byte 0x38 da partida (0x4792) ligado.
pub(crate) unsafe extern "thiscall" fn momento_marcado(this: usize) -> u32 {
    let m = *((this + 0x8ed4) as *const u16);
    let partida = *((this + 0x4792) as *const usize);
    (m != 0x1ef && m != 0x3de && byte(partida, 0x38) as u8 & 0x80 != 0) as u32
}

/// { 0x6f4a40 } (match_pl.cpp): estado do proprio lado (lado 0: 0xf5bd; outros: 0xf5f3).
pub(crate) unsafe extern "thiscall" fn estado_do_lado(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    byte(motor, if lado(this) != 0 { 0xf5f3 } else { 0xf5bd }) as u8
}

/// { 0x6f4a70 } (match_pl.cpp): estado do outro lado.
pub(crate) unsafe extern "thiscall" fn estado_do_adversario(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    byte(motor, if lado(this) != 0 { 0xf5bd } else { 0xf5f3 }) as u8
}

/// { 0x70a890 } (match_pl.cpp): grava 0x19c limitado a -200..=200.
pub(crate) unsafe extern "thiscall" fn gravar_19c(this: usize, v: i16) {
    *((this + 0x19c) as *mut i16) = v.clamp(-200, 200);
}

/// { 0x70a8c0 } (match_pl.cpp, sem `this`): codigos 12, 13, 21, 37, 46 e 59.
pub(crate) unsafe extern "stdcall" fn codigo_de_bola_parada(c: i16) -> u8 {
    matches!(c, 12 | 13 | 21 | 37 | 46 | 59) as u8
}

/// y < 4; side 0: y > 7
/// Original: `near_goal` (0x70df70, side 1, thiscall, `al`).
///
/// [pt] { 0x70df70 } (match_pl.cpp): perto do gol adversario (lado 1: y < 4; lado 0: y > 7).
pub(crate) unsafe extern "thiscall" fn perto_do_gol(this: usize) -> u8 {
    let (l, y) = (lado(this), byte(this, 0x103));
    ((l == 1 && y < 4) || (l == 0 && y > 7)) as u8
}

/// 0x475a..=0x4769
/// Original: `clear_move` (0x6bba10, thiscall).
///
/// [pt] { 0x6bba10 } (match_eng.cpp): limpa o lance (0x475a..=0x4769).
pub(crate) unsafe extern "thiscall" fn limpar_lance(this: usize) {
    *((this + 0x475f) as *mut u16) = 0xffff;
    *((this + 0x4761) as *mut u32) = 0;
    *((this + 0x4765) as *mut u32) = 0;
    *((this + 0x4769) as *mut u8) = 0;
    *((this + 0x475a) as *mut u8) = 0;
    for o in 0x475b..=0x475e {
        *((this + o) as *mut u8) = 0xff;
    }
}

/// { 0x6cf710 } (match_eng.cpp): soma `delta` a disposicao (0x35) de todos os ativos do lado.
pub(crate) unsafe extern "thiscall" fn somar_disposicao(this: usize, lado_b: i8, delta: i16) {
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) >= 0 {
            let w = (p + 0x35) as *mut i16;
            *w = (*w).wrapping_add(delta);
        }
    }
}

// ---------- vigesima primeira leva: rotinas sem arquivo no mapa ----------

/// word 0x1bc
/// Original: `read_1bc` (0x717400, thiscall, `ax`).
///
/// [pt] { 0x717400 }: palavra 0x1bc do jogador.
pub(crate) unsafe extern "thiscall" fn ler_1bc(this: usize) -> u16 {
    *((this + 0x1bc) as *const u16)
}

/// { 0x7160d0 }: soma ao byte 0x2b do jogador.
pub(crate) unsafe extern "thiscall" fn somar_2b(this: usize, v: u8) {
    let b = (this + 0x2b) as *mut u8;
    *b = (*b).wrapping_add(v);
}

/// { 0x7160e0 }: grava o byte 0x2b do jogador.
pub(crate) unsafe extern "thiscall" fn gravar_2b(this: usize, v: u8) {
    *((this + 0x2b) as *mut u8) = v;
}

/// Chama uma rotina (motor, jogador) do motor do jogador pelo endereco do jogo; sem motor, None.
pub(crate) unsafe fn pelo_motor<R>(j: usize, rotina: usize) -> Option<R> {
    let motor = *((j + 0x19e) as *const usize);
    if motor == 0 {
        return None;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> R = core::mem::transmute(rotina);
    Some(f(motor, j))
}

/// { 0x717430 }: `tabela_a_1a` (0x6c3920) do motor do jogador, ou 0.
pub(crate) unsafe extern "thiscall" fn tabela_a_1a_do_jogador(this: usize) -> u16 {
    pelo_motor(this, 0x6c3920).unwrap_or(0)
}

/// { 0x717450 }: `tabela_b` (0x6c38f0) do motor do jogador, ou 0.
pub(crate) unsafe extern "thiscall" fn tabela_b_do_jogador(this: usize) -> u16 {
    pelo_motor(this, 0x6c38f0).unwrap_or(0)
}

/// { 0x715f30 }: `diferenca_jogador` (0x6c4130) do motor do jogador, ou 0xff.
pub(crate) unsafe extern "thiscall" fn diferenca_do_jogador(this: usize) -> u8 {
    pelo_motor(this, 0x6c4130).unwrap_or(0xff)
}

/// already in Rust, not hooked; now hooked (prologue with a relative `jne` rebuilt in the trampoline)
/// Original: `ball_in_attack` (0x714d00, match_pl.cpp, thiscall).
///
/// [pt] { 0x714d00 } (o prologo curto termina num jne relativo, refeito no trampolim: RELATIVAS em
/// desvios.rs): bola (y em 0x8ea8) no campo de ataque do lado (lado 1: y >= 6; outros: y < 6).
pub(crate) unsafe extern "thiscall" fn bola_no_ataque(this: usize) -> u8 {
    let y = byte(*((this + 0x19e) as *const usize), 0x8ea8);
    (if lado(this) == 1 { y >= 6 } else { y < 6 }) as u8
}

/// 0x5b5270 of the position
/// Original: `player_marks_rot` (0x7163c0, thiscall, `edx:eax`).
///
/// [pt] { 0x7163c0 }: marcas taticas (0x5b5270) da posicao do jogador. Devolve edx:eax inteiro.
pub(crate) unsafe extern "thiscall" fn marcas_do_jogador_rot(this: usize) -> u64 {
    marcas_do_jogador(this)
}

/// byte 6 + side; otherwise bit 0x800000 of the marks
/// Original: `mark_800000` (0x7163f0, bit 0x1000 of the block, thiscall, `al`).
///
/// [pt] { 0x7163f0 }: com o bit 0x1000 do bloco do lado (0x9766), o byte 6 + lado do motor; senao, o
/// bit 0x800000 das marcas da posicao.
pub(crate) unsafe extern "thiscall" fn marca_800000(this: usize) -> u8 {
    let l = lado(this) as isize;
    let motor = *((this + 0x19e) as *const usize) as isize;
    let bloco = motor + 6371 * l;
    if *((bloco + 0x9766) as *const u32) & 0x1000 != 0 {
        return *((motor + 6 + l) as *const u8);
    }
    (marcas_do_jogador(this) as u32 & 0x800000 != 0) as u8
}

/// y < 3; side 1: y > 8
/// Original: `near_own_goal` (0x70def0, side 0, thiscall, `al`).
///
/// [pt] { 0x70def0 }: perto do proprio gol (lado 0: y < 3; lado 1: y > 8).
pub(crate) unsafe extern "thiscall" fn perto_do_proprio_gol(this: usize) -> u8 {
    let (l, y) = (lado(this), byte(this, 0x103));
    ((l == 0 && y < 3) || (l == 1 && y > 8)) as u8
}

/// 0x880 in `tabela_a` and `tabela_b`
/// Original: `role_880` (0x717c70, thiscall, `al`).
///
/// [pt] { 0x717c70 }: papel 0x880 em `tabela_a` e em `tabela_b` do jogador (sem motor, 0).
pub(crate) unsafe extern "thiscall" fn papel_880(this: usize) -> u8 {
    let a: u16 = pelo_motor(this, 0x6c38c0).unwrap_or(0);
    if a & 0x880 == 0 {
        return 0;
    }
    let b: u16 = pelo_motor(this, 0x6c38f0).unwrap_or(0);
    (b & 0x880 != 0) as u8
}

/// zeroes the owner and the move state
/// Original: `release_ball` (0x6f0970, thiscall).
///
/// [pt] { 0x6f0970 }: solta a bola: zera 0x4786, 0xf582 e 0x1a7 do motor (0x8eae = 0xff) e as palavras
/// 0x29 e 0x19c do jogador. O motor e relido a cada gravacao.
pub(crate) unsafe extern "thiscall" fn soltar_bola(this: usize) {
    let motor = || *((this + 0x19e) as *const usize);
    *((motor() + 0x4786) as *mut u32) = 0;
    *((motor() + 0x8eae) as *mut u8) = 0xff;
    *((motor() + 0xf582) as *mut u32) = 0;
    *((motor() + 0x1a7) as *mut u8) = 0;
    *((this + 0x29) as *mut u16) = 0;
    *((this + 0x19c) as *mut u16) = 0;
}

/// { 0x6e1cd0 }: o jogador (dos 20 a partir de [this + 0x23]) com a ficha dada, ou 0.
pub(crate) unsafe extern "thiscall" fn jogador_pela_ficha(this: usize, ficha: usize) -> usize {
    let base = *((this + 0x23) as *const usize);
    (0..20).map(|k| base + 0x1be * k).find(|&p| *((p + 0x69) as *const usize) == ficha).unwrap_or(0)
}

/// { 0x6d5cd0 } (sem `this`): byte da tabela de 18 bytes por codigo em 0xb68788 (codigos
/// 0x1f40..0x21e4, bit 15 ignorado); fora, 0.
pub(crate) unsafe extern "stdcall" fn byte_do_codigo(c: u32) -> i32 {
    let i = ((c & 0x7fff) as i32 - 0x1f40) as i16;
    if !(0..0x2a5).contains(&i) {
        return 0;
    }
    *((0xb68788 + 18 * i as usize) as *const i8) as i32
}

// ---------- vigesima segunda leva ----------

/// { 0x715f50 }: valor ajustado pelo papel e pela pessoa (0x6d). Papel 1, ou 2 x 0x10a >=
/// sorteio(21): o proprio `base`. Senao soma tres parcelas (cada uma `base`, ou um valor fixo por
/// lado quando o atributo da pessoa vence sorteio(41) e o papel nao o impede) e divide por 3.
pub(crate) unsafe extern "thiscall" fn nota_ajustada(this: usize, base: i8) -> i32 {
    let papel = || pelo_motor::<u16>(this, 0x6c38c0).unwrap_or(0);
    if papel() & 1 != 0 {
        return base as u8 as i32;
    }
    if (byte(this, 0x10a) as i32) * 2 >= sortear(0x15) {
        return base as u8 as i32;
    }
    let pessoa = *((this + 0x6d) as *const usize);
    let vence = |o: usize| {
        let r = sortear(byte(pessoa, o) as i32);
        r > sortear(0x29)
    };
    let lado0 = lado(this) == 0;
    let mut v: i8 = if papel() & 0x80 == 0 && vence(0x18) { if lado0 { 1 } else { 7 } } else { base };
    v = v.wrapping_add(if papel() & 0x700 == 0 && vence(0x19) { 4 } else { base });
    v = v.wrapping_add(if papel() & 0x800 == 0 && vence(0x17) { if lado0 { 7 } else { 1 } } else { base });
    v as i32 / 3
}

/// { 0x7172d0 }: ajusta o byte 0x17 do jogador por `delta` (com sorteios perto do limite 10), com
/// piso pela disposicao (0x35 >= 7500) e faixa 1..=20; devolve se mudou. Com `delta` 0 e 0x477a
/// do motor em -1, so age se a disposicao passar de 7500 e o byte estiver abaixo de 10.
pub(crate) unsafe extern "thiscall" fn ajustar_17(this: usize, delta: i8) -> u8 {
    let disp = *((this + 0x35) as *const i16);
    if delta == 0 {
        let motor = *((this + 0x19e) as *const usize);
        if *((motor + 0x477a) as *const i32) == -1 && !(disp > 0x1d4c && byte(this, 0x17) < 10) {
            return 0;
        }
    }
    let v = byte(this, 0x17);
    let t = v as i32 + delta as i32;
    let mut a = if delta > 0 {
        if t <= 10 || sortear(0x15) >= t { v.wrapping_add(delta) } else { (sortear(delta as i32 + 1) as i8).wrapping_add(v) }
    } else if delta < 0 {
        if t >= 10 || sortear(0xb) <= t { v.wrapping_add(delta) } else { (sortear(delta as i32 - 1) as i8).wrapping_add(v) }
    } else {
        v
    };
    let disp = *((this + 0x35) as *const i16);
    if disp >= 0x1d4c {
        let x = a as f64;
        let y = disp as f64 * f64_em(0x968d60) - f64_em(0x96d868);
        a = ftol(if x > y { x } else { y }) as i8;
    }
    let a = a.clamp(1, 20);
    *((this + 0x17) as *mut i8) = a;
    (a != v) as u8
}

// ---------- vigesima terceira leva: posse de bola ----------

/// releases the previous owner's, credits the pass (counter and disposition by zone), records possession and the ball
/// on his square
/// Original: `take_ball` (0x6f0630, the player takes the ball, thiscall).
///
/// [pt] { 0x6f0630 } (match_pl.cpp): o jogador pega a bola. Solta a do dono anterior (0xf582, mesmas
/// gravacoes de `soltar_bola`), limpa o estado do lance, credita o passe (0xf576, do mesmo lado:
/// contador 0x2 e disposicao + 0x6c/0x60/0x96/0x78 conforme papel e zona), registra o novo dono,
/// a bola na casa dele, o lado com a posse (0x1a5 + lado) e a contagem de posse (0x8d46 + lado).
/// O motor e relido a cada acesso, como no original.
pub(crate) unsafe extern "thiscall" fn pegar_bola(this: usize) {
    let m = || *((this + 0x19e) as *const usize);
    let b = |o: usize| (m() + o) as *mut u8;
    let w = |o: usize| (m() + o) as *mut u32;
    if m() == 0 {
        return;
    }
    let antigo = *w(0xf582) as usize;
    if antigo != 0 {
        soltar_bola(antigo);
    }
    *b(0x1a4) = 0;
    *b(0x1a7) = 0;
    *b(0x1cd) = 0;
    *w(0x4786) = 0;
    let tabela_a = || {
        let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
        f(m(), this)
    };
    let passe = *w(0xf576) as usize;
    if passe != 0 && lado(this) == lado(passe) {
        *((passe + 2) as *mut u8) = (*((passe + 2) as *const u8)).wrapping_add(1);
        let (l, y) = (lado(this), byte(this, 0x103));
        let soma: i16 = if tabela_a() == 1 {
            0x6c
        } else if (l == 1 && y >= 6) || (l == 0 && y < 6) {
            0x60
        } else if (l == 1 && y < 3) || (l == 0 && y > 8) {
            0x96
        } else {
            0x78
        };
        let disp = (*w(0xf576) as usize + 0x35) as *mut i16;
        *disp = (*disp).wrapping_add(soma);
        *w(0x4782) = (*w(0x4782)).wrapping_add(0xf0);
        let anterior = *w(0xf576);
        *w(0xf5aa) = this as u32;
        *w(0xf5a6) = anterior;
    }
    *w(0xf576) = 0;
    *w(0xf57a) = 0;
    *w(0x477e) = 0;
    if tabela_a() == 1 {
        let (l, x, y) = (lado(this), byte(this, 0x102), byte(this, 0x103));
        let area = if l == 1 { 10..=11 } else { 0..=1 };
        if (2..=6).contains(&x) && area.contains(&y) {
            let mm = m() as isize;
            if *((mm + 0x1a5 + l as isize) as *const u8) == 0 {
                *((this + 0x19a) as *mut u16) = if l != 0 { 0xb4 } else { 0 };
                *((mm + 0x8ea9) as *mut u8) = 0xff;
            }
        }
    }
    let (l, y) = (lado(this), byte(this, 0x103));
    if (l == 1 && y >= 6) || (l == 0 && y < 6) {
        *b(0x8eb1) = 0;
    }
    *((m() as isize + 0x1a5 + lado(this) as isize) as *mut u8) = 1;
    *((m() + 0x1a5 + (lado(this) == 0) as usize) as *mut u8) = 0;
    *b(0x8ea7) = byte(this, 0x102) as u8;
    *b(0x8ea8) = byte(this, 0x103) as u8;
    *b(0x8eae) = lado(this) as u8;
    *w(0xf582) = this as u32;
    *((m() + 0x8ed8) as *mut u16) = *((this + 0x19a) as *const u16);
    *b(0x8eaa) = 0;
    let mm = m();
    if byte(mm, 0x8eab) != lado(this) {
        *((mm + 0x4782) as *mut u32) = 0;
    }
    let l = lado(this);
    let mm = m();
    *((mm + 0x8eab) as *mut i8) = l;
    if l != -1 {
        let c = (mm as isize + 0x8d46 + l as isize) as *mut u8;
        *c = (*c).wrapping_add(1);
        let o = (mm + 0x8d46 + (l == 0) as usize) as *mut i8;
        if *o > 0 {
            *o -= 10;
        }
    }
    *((this + 0x29) as *mut u16) = 0;
    *((this + 0x19c) as *mut u16) = 0;
}

// ---------- trigesima terceira leva: defesa do goleiro ----------

/// Sorteio da defesa: se sorteio(palavra 0x3b do goleiro) vence sorteio(285), sorteio(b x 0,7) +
/// b x 0,3; senao sorteio(b).
pub(crate) unsafe fn sorte_da_defesa(j: usize, b: i16) -> i32 {
    let r = sortear(*((j + 0x3b) as *const i16) as i32);
    if r > sortear(0x11d) {
        let x = b as f64;
        let t = sortear(ftol(x * f64_em(0x968e40)));
        ftol(t as f64 + x * f64_em(0x968e80))
    } else {
        sortear(b as i32)
    }
}

/// { 0x700420 } (match_pl.cpp): valor da defesa do goleiro (papel 1, 0x2b <= 1) contra o lance
/// para (a, b) com `codigo`. Base pela palavra 0x16a (com `a3`) ou 0x168, + 220; a distancia a bola
/// (tabela 12 x 9) entra no fim. `saida`: 1 fora do lado, 3 fora da area, 2 sem alcance, 4 nos
/// codigos 26/27; `rebote` quando o sorteio final perde. Devolve ftol(base / 6 + v x c + c').
pub(crate) unsafe extern "thiscall" fn defesa_do_goleiro(this: usize, a: i8, b: i8, a3: u8, codigo: i8, saida: *mut u8, rebote: *mut u8) -> u16 {
    *saida = 0;
    *rebote = 0;
    let w = *((this + if a3 != 0 { 0x16a } else { 0x168 }) as *const i16);
    let base = (w as u16).wrapping_add(0xdc) as i16;
    let motor = *((this + 0x19e) as *const usize);
    let dy = (byte(this, 0x103) as i32 - byte(motor, 0x8ea8) as i32).abs() as i8 as isize;
    let dx = (byte(this, 0x102) as i32 - byte(motor, 0x8ea7) as i32).abs() as i8 as isize;
    let dist = f32_em((0xa15e44 + 4 * (dy * 9 + dx)) as usize) as f64;
    if byte(this, 0x2b) > 1 || motor == 0 {
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    if tabela(motor, this) != 1 {
        return 0;
    }
    let longe = |e: i32| if dist > f64_em(0x96d700) { e + 10 - ftol(*((this + 0x3b) as *const i16) as f64 * f64_em(0x96d7c8)) } else { e };
    let perto = |e: i32| if !(dist >= f64_em(0x967890)) { e + 100 } else { e };
    let e: i32 = if codigo == 0x33 {
        let b0 = base as i32;
        let v = sortear(b0);
        sortear(b0 - v)
    } else {
        let (l, x, y) = (lado(this), byte(this, 0x102), byte(this, 0x103));
        let area = if l == 1 { 10..=11 } else { 0..=1 };
        if (l == 1 && y < b) || (l == 0 && y > b) {
            *saida = 1;
            0
        } else if !(2..=6).contains(&x) || !area.contains(&y) {
            *saida = 3;
            0
        } else {
            let alcanca: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_GOAL_SIDE_OF_CELL);
            if alcanca(this, a, b) == 0 {
                *saida = 2;
                sorte_da_defesa(this, base) - 200
            } else if y == 11 || y == 0 {
                match codigo {
                    26 => perto(sorte_da_defesa(this, base) + 75),
                    27 => perto(sorte_da_defesa(this, base) + 125),
                    _ => longe(sorte_da_defesa(this, base)),
                }
            } else {
                match codigo {
                    3 | 5 => sorte_da_defesa(this, base.wrapping_sub(150)),
                    26 => {
                        *saida = 4;
                        perto(sorte_da_defesa(this, base.wrapping_sub(150)))
                    }
                    27 => {
                        *saida = 4;
                        perto(sorte_da_defesa(this, base.wrapping_sub(200)))
                    }
                    _ => longe(sorte_da_defesa(this, base)),
                }
            }
        }
    };
    let motor = *((this + 0x19e) as *const usize);
    let r1 = sortear(ftol(f32_em(this + 0xd1) as f64 * 2.0 + *((this + 0xf) as *const u8) as f64 + f64_em(0x9688b0)));
    let r2 = sortear(ftol((f64_em(0x968950) - dist) * byte(motor, 0x8eaa) as f64 * f64_em(0x9688b0)));
    let v = if r1 > r2 {
        e as i16
    } else {
        *rebote = 1;
        sorte_da_defesa(this, e as i16) as i16
    };
    ftol((w as i32 / 6) as f64 + v as f64 * f64_em(0x96d698) + f64_em(0x968ea8)) as u16
}

// ---------- trigesima quarta leva: mover o jogador na grade ----------

/// Casa (x, y) da grade de ocupacao do motor: 90 bytes por casa, a partir de 0x215e; por lado,
/// lista de 11 ponteiros (44 bytes) e a quantidade em 0x58 + lado.
pub(crate) fn casa_da_grade(motor: usize, x: i8, y: i8) -> isize {
    motor as isize + 0x215e + 90 * (12 * x as isize + y as isize)
}

/// { 0x6f46b0 } (match_pl.cpp): move o jogador para (x, y): tira da lista do lado na casa antiga
/// (o ultimo da lista ocupa a vaga dele, 0x28) e poe no fim da lista da casa nova; atualiza a bola
/// se ele for o dono. Na mesma casa, nada. Estados impossiveis do original (lista vazia ao tirar,
/// mais de 11 ao por) mostram erro (e, no segundo, encerram); aqui, o primeiro so volta e o segundo
/// encerra como o original.
pub(crate) unsafe extern "thiscall" fn mover_para(this: usize, x: i8, y: i8) {
    if x == byte(this, 0x102) && y == byte(this, 0x103) {
        return;
    }
    let motor = *((this + 0x19e) as *const usize);
    let lista = |casa: isize, l: i8, i: isize| (casa + 4 * (11 * l as isize + i)) as *mut usize;
    let quantos = |casa: isize, l: i8| (casa + 0x58 + l as isize) as *mut i8;
    let vaga = (this + 0x28) as *mut i8;
    let nova = casa_da_grade(motor, x, y);
    if *vaga > -1 {
        let antiga = casa_da_grade(motor, byte(this, 0x102), byte(this, 0x103));
        let l = lado(this);
        let n = *quantos(antiga, l);
        if n == 0 {
            return;
        }
        *lista(antiga, l, *vaga as isize) = *lista(antiga, l, n as isize - 1);
        *quantos(antiga, l) = n.wrapping_sub(1);
        let v = *vaga;
        let movido = *lista(antiga, l, v as isize);
        if movido != 0 {
            *((movido + 0x28) as *mut i8) = v;
        }
        let l = lado(this);
        *vaga = *quantos(nova, l);
        *quantos(nova, l) = (*quantos(nova, l)).wrapping_add(1);
        *lista(nova, lado(this), *vaga as isize) = this;
    } else {
        let l = lado(this);
        *vaga = *quantos(nova, l);
        *quantos(nova, l) = (*quantos(nova, l)).wrapping_add(1);
        if *quantos(nova, lado(this)) > 11 {
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
            sair(-1);
        }
        *lista(nova, l, *vaga as isize) = this;
    }
    *((this + 0x102) as *mut i8) = x;
    *((this + 0x103) as *mut i8) = y;
    let motor = *((this + 0x19e) as *const usize);
    if this == *((motor + 0xf582) as *const usize) {
        *((motor + 0x8ea7) as *mut i8) = x;
        *((*((this + 0x19e) as *const usize) + 0x8ea8) as *mut i8) = byte(this, 0x103);
    }
}

// ---------- trigesima sexta leva: dominio do lance ----------

/// O jogador do lado esta na propria area (x 2..=6; lado 1: y 10..=11; senao y 0..=1)?
pub(crate) unsafe fn na_propria_area(j: usize) -> bool {
    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
    let area = if lado(j) == 1 { 10..=11 } else { 0..=1 };
    (2..=6).contains(&x) && area.contains(&y)
}

/// { 0x70d690 } (match_pl.cpp): valor do dominio do lance `codigo` (palavra do jogador por codigo,
/// menos o sorteio pela condicao fisica 0x4d, a dificuldade pela direcao da bola sem dono, a
/// palavra 0x19c do dono e, em alguns codigos, 20 x (0x8eaa - 1)^3). Fora da area, sem o codigo
/// 0x15/0x25, a distancia ao gol pode fazer perder na hora (-20). O goleiro na propria area troca
/// o codigo (41, ou 1 com 0x8ea9 == 4); 19/33/34 com valor baixo viram 45 em metade das vezes.
/// As saidas (`a`, `b`) dependem do codigo final.
pub(crate) unsafe extern "thiscall" fn dominio_do_lance(this: usize, codigo: *mut i8, a: *mut u8, b: *mut u8) -> u16 {
    let motor = *((this + 0x19e) as *const usize);
    let c8ea9 = byte(motor, 0x8ea9);
    *a = 0;
    *b = 0;
    let c0 = *codigo;
    if c0 != 0x25 && c0 != 0x15 && !na_propria_area(this) {
        let (x, y) = (byte(this, 0x102) as i32, byte(this, 0x103) as i32);
        let dy = (if lado(this) == 1 { y - 0xb } else { y }).abs() as i8 as isize;
        let dx = (x - 4).abs() as i8 as isize;
        let t = f32_em((0xa15e44 + 4 * (dy * 9 + dx)) as usize) as f64;
        let r1 = sortear(ftol(f32_em(this + 0x75) as f64 + f32_em(0x9689a8) as f64));
        if r1 < sortear(ftol(t + t)) {
            *b = 0;
            *a = 0;
            return 0xffec;
        }
    }
    let motor = *((this + 0x19e) as *const usize);
    let dono = *((motor + 0xf582) as *const usize);
    if dono == 0 {
        let c = *codigo;
        let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
        if c != 0x15 && c != 0x25 && especial(c) == 0 {
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
            sair(-1);
        }
    }
    let bp: u16 = if dono == this { *((dono + 0x19c) as *const u16) } else { 0 };
    let c = 10000 - ftol(f32_em(this + 0x4d) as f64) as i16 as i32;
    let r = sortear(c.wrapping_mul(c) / 50000) as u16;
    let mut dif: u16 = 0;
    if dono == 0 {
        let motor = *((this + 0x19e) as *const usize);
        let mut dd = (*((this + 0x19a) as *const i16) as i32 - *((motor + 0x8ed8) as *const i16) as i32).abs();
        if dd as i16 > 0xb4 {
            dd = 0x168 - dd;
        }
        let e = (0xb4 - dd) * 2;
        let f = (5 - byte(motor, 0x8ea9) as i32).max(1);
        dif = sortear(((e as i16 as i32) / (f as i16 as i32)) as i16 as i32) as u16;
    }
    let motor = *((this + 0x19e) as *const usize);
    let g = (byte(motor, 0x8eaa) as u8).wrapping_sub(1) as i8 as i16 as u16;
    let cubo = g.wrapping_mul(g).wrapping_mul(g).wrapping_mul(20);
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let w = |o: usize| *((this + o) as *const u16);
    let si: u16 = if tabela(motor, this) == 1 && na_propria_area(this) {
        let novo: i8 = if c8ea9 != 4 { 0x29 } else { 1 };
        *codigo = novo;
        w(0x11a + 2 * novo as usize).wrapping_sub(r).wrapping_sub(bp)
    } else {
        let menos = |v: u16| v.wrapping_sub(r).wrapping_sub(bp);
        let ou_45 = |v: u16| {
            if (v as i16) < 2 && sortear(2) != 0 {
                *codigo = 0x2d;
                menos(w(0x174))
            } else {
                v
            }
        };
        match *codigo {
            12 => w(0x132).wrapping_sub(cubo).wrapping_sub(r).wrapping_sub(dif).wrapping_sub(bp),
            21 => w(0x144).wrapping_sub(cubo).wrapping_sub(r).wrapping_sub(dif).wrapping_sub(bp),
            19 => ou_45(menos(w(0x140))),
            33 => ou_45(menos(w(0x15c))),
            34 => ou_45(menos(w(0x15e))),
            37 => w(0x164).wrapping_sub(bp),
            42 => menos(w(0x16e)),
            43 => menos(w(0x170)),
            44 => menos(w(0x172)),
            45 => menos(w(0x174)),
            46 => menos(w(0x176)),
            47 => menos(w(0x178)),
            49 => w(0x17c).wrapping_sub(bp),
            50 => w(0x17e).wrapping_sub(bp),
            _ => 0,
        }
    };
    let (sa, sb): (u8, u8) = match *codigo {
        1 | 41 => {
            let r = sortear(2) as u8;
            (1, r.wrapping_add(2))
        }
        12 => (0, 0),
        19 | 33 | 45 => (2, 2),
        21 => (1, 2),
        34 => (sortear(2) as u8 + 1, 2),
        36 | 37 | 43 => (0, 2),
        42 | 46 => (sortear(2) as u8, 2),
        44 => (5, 2),
        47 => (sortear(4) as u8 + 1, 2),
        49 => (sortear(4) as u8 + 1, 0),
        50 => (sortear(2) as u8 + 1, 2),
        _ => (0, 0),
    };
    *b = sb;
    *a = sa;
    si
}

// ---------- trigesima nona leva: comentarios (0x6d6f70) ----------

/// Chamada a 0x6d6f70 (enfileirar um evento de comentario): (this, 7 argumentos).
pub(crate) type ChamarComentario<'a> = &'a mut dyn FnMut(usize, [u32; 7]);

/// 0x6d6f70 pelo endereco do jogo.
pub(crate) unsafe fn comentar(this: usize, a: [u32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_EVENTS_ADD_EVENT);
    f(this, a[0], a[1], a[2], a[3], a[4], a[5], a[6])
}

/// { 0x7002f0 } (match_pl.cpp) com a chamada injetada: comentario com dois campos -1.
pub(crate) unsafe fn comentario_a_com(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, chamar: ChamarComentario) {
    chamar(this, [a1, a2, a3, a4, u32::MAX, u32::MAX, a5]);
}

/// { 0x7002f0 } (match_pl.cpp): enfileira um comentario (0x6d6f70) com dois campos -1.
pub(crate) unsafe extern "thiscall" fn comentario_a(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
    comentario_a_com(this, a1, a2, a3, a4, a5, &mut |t, a| comentar(t, a))
}

/// { 0x70a860 } (match_pl.cpp) com a chamada injetada: comentario com tres campos -1.
pub(crate) unsafe fn comentario_b_com(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, chamar: ChamarComentario) {
    chamar(this, [a1, a2, u32::MAX, a3, u32::MAX, u32::MAX, a4]);
}

/// { 0x70a860 } (match_pl.cpp): enfileira um comentario (0x6d6f70) com tres campos -1.
pub(crate) unsafe extern "thiscall" fn comentario_b(this: usize, a1: u32, a2: u32, a3: u32, a4: u32) {
    comentario_b_com(this, a1, a2, a3, a4, &mut |t, a| comentar(t, a))
}

// ---------- centesima vigesima sexta leva: a dividida (0x70ec80) ----------

/// A falta do desafiante `e` sobre `j` (tipo da 0x6cfc60, lance `lance`) punida pelo arbitro
/// (0x6cfcd0), como na dividida (0x70ec80) e no chute (0x6f6c00): narracao pelo lance, cartao, bola
/// parada 7, penalti (5) ou falta (6), expulsao, cobrador, as narracoes de sempre, o desgaste do time
/// e o cansaco; no fim `e` perde 25 de folego. `ramos`: penalti, expulsao, cobrador, vantagem.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn punir_desafiante_com(m: usize, e: usize, j: usize, mut tipo: u8, cartao: u8, grav: i8, arriscado: i8, vantagem: u8, lance: i8, ajustar_no_penalti: bool, ramos: &[&'static str; 4], chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let punir: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_DECIDE_FOUL_PUNISHMENT);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let ao_time: unsafe extern "thiscall" fn(usize, usize, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER_FROM_PLAYER);
    let ajustar: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_ADJUST_MORALE);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |p: usize| ub(p, 0x2e) | ub(p, 0x27) << 8;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let na_area = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let mut peso = 0f32;
    punir(m, e, j, &mut tipo, cartao as i8, grav, arriscado as u8, &mut peso, vantagem);
    narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(j));
    let cor: i32 = match tipo {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    let som: u32;
    if tipo == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if na_area(j, byte(j, 0x27) == 0) {
        if cor == -1 {
            match sortear(4) {
                0 => narrar(chamar, 0x20e9, u32::MAX, ub(j, 0x27), u32::MAX),
                1 => narrar(chamar, 0x20ec, u32::MAX, ub(j, 0x27), u32::MAX),
                _ => {}
            }
        }
        crate::comparacao::ramo(ramos[0]);
        som = 0x1f7a;
        *b(m, 0x8eb2) = 5;
        *((m + 0x478a) as *mut usize) = j;
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(500);
        // no chute (0x6f6c00) o penalti ainda mexe no +0x17 de quem fez
        if ajustar_no_penalti && sortear(byte(*((e + 0x69) as *const usize), 0x5a) as i32) == 0 {
            let r = sortear(10);
            if ajustar(e, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho(e, 0);
            }
        }
    } else {
        som = 0x1f79;
        *b(m, 0x8eb2) = 6;
    }
    *b(m, 0x8ea7) = *b(j, 0x102);
    *b(m, 0x8ea8) = *b(j, 0x103);
    *b(m, 0x8eae) = *b(j, 0x27);
    *((m + 0xf582) as *mut u32) = 0;
    if som == 0x1f7a {
        narrar(chamar, 0x1f7a, u32::MAX, ub(j, 0x27), u32::MAX);
    }
    let expulso = matches!(tipo, 3..=6);
    if cor > -1 {
        if (lance == 0x2c || lance == 0x2f) && vantagem != 0 {
            narrar(chamar, if lance == 0x2f { 0x21da } else { 0x21d9 }, ub(j, 0x2e), ub(j, 0x27), par(e));
            if expulso || som == 0x1f7a {
                *b(e, 0x16) = 1;
            }
        }
        if sortear(4) == 0 {
            let x = sortear(grav as i32);
            if x > sortear(0x14) {
                let t = *((m + 0x4792) as *const usize);
                let clube = *((t + if byte(j, 0x27) != 0 { 0x20 } else { 0x1c }) as *const usize);
                let k = *((clube + 0x53) as *const usize);
                if k != 0 && byte(k, 0x77) == 8 {
                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrar(chamar, 0x2109, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        }
        narrar(chamar, cor as u32, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        if expulso {
            crate::comparacao::ramo(ramos[1]);
            jogador_saiu_com(m, e, 0, chamar);
        }
    }
    if som != 0x1f7a {
        crate::comparacao::ramo(ramos[2]);
        let c = cobrador(m, byte(j, 0x27), byte(m, 0x8eb2), 0);
        if c == 0 {
            narrar(chamar, som, u32::MAX, ub(j, 0x27), u32::MAX);
        } else {
            if tipo == 7 {
                narrar(chamar, 0x21e2, ub(c, 0x2e), ub(j, 0x27), u32::MAX);
            } else {
                narrar(chamar, som, u32::MAX, ub(j, 0x27), u32::MAX);
            }
            // a cobranca longe do gol adversario (lado 1: linha < 4; lado 0: > 7)
            let longe = (byte(j, 0x27) == 1 && byte(j, 0x103) < 4) || (byte(j, 0x27) == 0 && byte(j, 0x103) > 7);
            if som == 0x1f79 && vantagem == 0 && longe {
                if tipo == 7 {
                    if !na_area(j, byte(j, 0x27) == 0) {
                        narrar(chamar, 0x21e1, ub(c, 0x2e), ub(j, 0x27), u32::MAX);
                    }
                } else if (2..=6).contains(&byte(j, 0x102)) {
                    narrar(chamar, 0x21e0, ub(c, 0x2e), ub(j, 0x27), u32::MAX);
                } else {
                    narrar(chamar, 0x21df, ub(c, 0x2e), ub(j, 0x27), u32::MAX);
                }
            }
        }
    }
    if especial(lance) == 0 {
        let extra = expulso || som == 0x1f7a;
        if vantagem != 0 {
            crate::comparacao::ramo(ramos[3]);
            narrar(chamar, 0x20dd, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
            if tipo == 4 || tipo == 5 {
                *b(e, 0x16) = 1;
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(j), u32::MAX);
            }
        } else if sortear(9) == 0 {
            narrar(chamar, 0x20df, u32::MAX, zero(j), u32::MAX);
            if sortear(3) == 0 {
                narrar(chamar, 0x20e0, u32::MAX, zero(j), u32::MAX);
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(j), u32::MAX);
            }
        }
        ao_time(m, e, peso);
        chamar(0x716e10, e, [0; 5]);
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// Narracao da falta pelo lance em disputa (tabelas 0x71026c e 0x710328 da 0x70ec80).
pub(crate) fn narracao_da_falta(lance: i8) -> u32 {
    match lance {
        1 => 0x1ff1,
        0xc => 0x1ff4,
        0x13 | 0x21 => 0x1fee,
        0x15 => 0x1ff0,
        0x22 => 0x1ff2,
        0x29 => 0x1fef,
        0x2a => 0x1feb,
        0x2b => 0x1fec,
        0x2c => 0x1fed,
        0x2d => 0x1fea,
        0x2e => 0x1ff3,
        _ => 0x1fe9,
    }
}

/// { 0x70ec80 } (ret 4, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x716c90 o lance de quem perdeu a bola, 0x6f0630 dominio, 0x6cd770 a bola
/// espirra; e as de dentro da lesao, da defesa e da saida de campo): a dividida contra `j`, dono da
/// bola. Antes de tudo sorteia pelo folego (+0x4d); so segue se `j` tem a bola (+0xf582). Com
/// `forte`, o lado adversario ganha 2,25 (0x6bb950) e o de `j` +10 de posse. O ataque e
/// sorteio(+0x19c - folego + +0x13a - sorteio(+0x8ea9 x 50)); o desafiante sai da 0x6c87e0 (lance
/// 0x10) com a nota, o lance e o risco, e a lesao na dividida (0x6cfe10) vem logo. Sem desafiante,
/// ou com o ataque acima da nota, `j` passa (ver o fim). Senao, com a nota abaixo do ataque e
/// sorteio(5), a disputa e limpa; senao o arbitro decide (0x6cfc60: falta, cartao, gravidade,
/// vantagem; 0x2168 na area com +0x8ebb). Falta com 1 em 30 sem risco so registra o lance
/// (0x6bb9a0); senao pune (0x6cfcd0), narra a falta pelo lance, escolhe o cartao (0x1ff6 a 0x1ff9 ou
/// 0x210a pela gravidade), marca bola parada 7, penalti (5, com 0x20e9/0x20ec sem cartao, -500 em
/// +0x35 do desafiante) ou falta (6); narra o cartao (0x21d9/0x21da no lance 0x2c/0x2f com vantagem,
/// 1 em 4 0x2109 e 0x20f5), a expulsao tira o desafiante (0x6cda20); sem penalti escolhe o cobrador
/// (0x6ceb90) e narra a cobranca (0x21df a 0x21e2 pela posicao); depois 0x20dd/0x20df/0x20e0/0x20d9,
/// o time do desafiante se desgasta (0x6bb8d0), ele cansa e perde 25 de folego. Disputa limpa: o
/// desafiante marca o lance (+0x198), conta (+4), perde 59 ou 70 de folego (0x6c38c0); com o ataque
/// acima da nota, `j` passa; senao, na propria area por sorteio(2), narra a falta e 0x20e8 a 0x20eb;
/// senao o lance (0x70d430 no lance 1, que pode encerrar; 0x1f61/0x1f62/0x1f64/0x2150/0x2164/0x1f5d).
/// O desafiante domina (0x6f0630) com +0x19c = nota - ataque - 200 (+-200); abaixo do limite do lance
/// (400, 250 ou 100) a bola espirra (0x6cd770) e, sem bola parada, ele fica no chao (+0x2b) ou, com
/// penalti marcado, `j` sofre (0x716c90); senao vira para o lance (0x6f4920), conta (+5) e, depois do
/// minuto 55/65, pode narrar 0x21cd/0x21ce pela media de acertos. Ganha folego (0xc8 ou 0x78, dobrado
/// fora das tabelas 2/4) e `j` soma o primeiro byte do desafio a +0x2b; 1 em 7 narra 0x1f41. Quando
/// `j` passa: fora da area, sorteio(+0x31) < -10 faz `j` crescer (+0x1a2 x 1,05, minimo
/// (25 - +0x5d) x 2, maximo 100, 0x211d); conta a posse (+0x8eab, +0x8d46), +0x19c ganha ataque -
/// nota (+-200), o desafiante soma o segundo byte; fora de bola parada e do lance 0x10 narra 0x2133
/// (com `forte`), 0x216d (com desafiante), 0x21de ou 0x1f40, e zera +0x8ea9.
pub(crate) unsafe fn dividida_com(j: usize, forte: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let aplicar_ao_lado: unsafe extern "thiscall" fn(usize, i8, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER);
    let desafio: unsafe extern "thiscall" fn(usize, usize, i8, i8, *mut u8, *mut u8, *mut i16, *mut i8, *mut i8, u8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OPPOSITION_CHALLENGE);
    let falta: unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_JUDGE_TACKLE);
    let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let apontar_: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_SET_FACING_DIRECTION);
    let pressao: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_PLAYERS_NEAR_CELL);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let m = *((j + 0x19e) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |p: usize| ub(p, 0x2e) | ub(p, 0x27) << 8;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    // na area de quem defende contra o lado de `p` (colunas 2 a 6, linhas 10-11 ou 0-1)
    let na_area = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let d = 10000 - ftol(f32_em(j + 0x4d) as f64) as i16 as i32;
    let folego = sortear(d.wrapping_mul(d) / 50000);
    let (mut a, mut b2, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let mut vantagem = 0u8;
    let mut penalti = false;
    if j != *((m + 0xf582) as *const usize) {
        return;
    }
    if forte != 0 {
        aplicar_ao_lado(m, (byte(j, 0x27) == 0) as i8, 2.25);
        let p = (m as isize + byte(j, 0x27) as isize + 0x8d46) as *mut u8;
        *p = (*p).wrapping_add(10);
    }
    let base = *w(j, 0x13a) as i32;
    let r = sortear(byte(m, 0x8ea9) as i32 * 50);
    let ataque = sortear(*w(j, 0x19c) as i32 - folego as i16 as i32 + base - r);
    let at = ataque as i16;
    let e = desafio(m, j, byte(j, 0x27), 0x10, &mut a, &mut b2, &mut nota, &mut lance, &mut arriscado, 0);
    lesao_na_dividida_com(m, j, 0x10, e, lance as i16, 0, chamar);
    let passa = 'disputa: {
        if e == 0 {
            crate::comparacao::ramo("dividida sem desafiante");
            break 'disputa true;
        }
        let mut tipo = 0u8;
        let (mut cartao, mut grav) = (0u8, 0i8);
        if !(nota < at && sortear(5) != 0) {
            tipo = falta(m, e, j, lance, -1, &mut cartao, &mut grav, &mut vantagem);
            if byte(m, 0x8ebb) != 0 && na_area(e, byte(j, 0x27) == 0)
                && (especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
                && sortear(3) != 0
            {
                narrar(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
            }
            if tipo != 0 {
                if sortear(0x1e) == 0 && arriscado == 0 {
                    crate::comparacao::ramo("dividida so registra");
                    registrar(m, lance as i16 as u16, e as u32, j as u32, 0, tipo, *b(j, 0x102), *b(j, 0x103), cartao, grav as u8);
                } else {
                    // a falta com punicao
                    crate::comparacao::ramo("dividida falta punida");
                    punir_desafiante_com(m, e, j, tipo, cartao, grav, arriscado, vantagem, lance, false, &["dividida penalti", "dividida expulsao", "dividida cobrador", "dividida vantagem"], chamar);
                    return;
                }
            }
        }
        if tipo == 0 && vantagem != 0 {
            penalti = na_area(j, byte(j, 0x27) == 0) || arriscado != 0;
        }
        crate::comparacao::ramo("dividida disputa limpa");
        // a disputa limpa
        *w(e, 0x198) = lance as i16;
        *b(e, 4) = (*b(e, 4)).wrapping_add(1);
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(if tabela(m, e) & 0x30 != 0 && tabela(m, e) & 0x880 != 0 { 0x3b } else { 0x46 });
        if at > nota {
            crate::comparacao::ramo("dividida passa na limpa");
            break 'disputa true;
        }
        let narrou = (especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
            && na_area(e, byte(e, 0x27) == 1)
            && sortear(2) == 0;
        if narrou {
            crate::comparacao::ramo("dividida narra falta na area");
            narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(j));
            let codigo = match sortear(0x32) {
                1 => 0x20ea,
                0 | 2 | 4 | 6 | 8 | 10 => 0x20eb,
                _ => 0x20e8,
            };
            narrar(chamar, codigo, u32::MAX, zero(e), u32::MAX);
        } else {
            let codigo = match lance {
                1 => {
                    crate::comparacao::ramo("dividida defesa");
                    let forca = nota.wrapping_sub(at);
                    let r = defender_com(e, forca, &mut |rotina, this, x| {
                        if rotina == 0x6d6f70 {
                            // a defesa narra com o quinto campo inteiro e o sexto -1
                            chamar(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                        } else {
                            chamar(rotina, this, x)
                        }
                    });
                    if r == 0 {
                        return;
                    }
                    0
                }
                0x13 | 0x21 | 0x2d => 0x1f62,
                0x15 | 0x2e => {
                    let r = sortear(6);
                    if ((r as f64 + f32_em(e + 0xad) as f64) + 3.0) < f32_em(j + 0xad) as f64 { 0x2164 } else { 0x1f5d }
                }
                0x22 | 0x2c => 0x2150,
                0x29 => 0x1f61,
                0x2a | 0x2b => 0x1f64,
                _ => 0,
            };
            if codigo != 0 {
                narrar(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), par(j));
            }
        }
        chamar(0x6f0630, e, [0; 5]);
        *w(e, 0x19c) = nota.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200);
        let mut dominou = false;
        let limite = match lance {
            0x13 => 400,
            0x21 | 0x2d => 250,
            _ => 100,
        };
        if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
            crate::comparacao::ramo("dividida bola espirra");
            chamar(0x6cd770, m, [ub(j, 0x102), ub(j, 0x103), 1 | zero(j) << 8, 0, 0]);
            if byte(m, 0x8eb2) == 0 {
                let mut k = 3;
                k -= (byte(e, 0x10d) as i32 > sortear(10)) as i32;
                k -= (byte(e, 0x10c) as i32 > sortear(10)) as i32;
                *b(e, 0x2b) = sortear(k) as u8;
                dominou = true;
            } else if penalti {
                crate::comparacao::ramo("dividida 716c90");
                chamar(0x716c90, j, [0; 5]);
            }
        } else {
            if (0..4).contains(&byte(m, 0x8ea9)) {
                *b(m, 0x8ea9) = 0;
            }
            let (x, y) = (byte(j, 0x102), byte(j, 0x103));
            if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
                apontar_(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
            } else {
                apontar_(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
            }
            let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
            let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
            let media = if byte(e, contas.0) != 0 { (ub(e, contas.1) as f64 / ub(e, contas.0) as f64) as f32 } else { 0.0 };
            if minuto > (sortear(0x14) + sorteio) as u8 as i8 {
                let gols = byte(m, if byte(e, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
                let tentativas = if lance == 0x15 { ub(e, 8) > 3 && byte(e, 9) != 0 } else { ub(e, 6) > 1 };
                if gols == 0 && media as f64 > 0.725 && tentativas && *w(e, 0x35) > 0x1a5e && byte(e, 0xfd) == 0 {
                    crate::comparacao::ramo("dividida media de acertos");
                    narrar(chamar, if lance == 0x15 { 0x21ce } else { 0x21cd }, ub(e, 0x2e), ub(e, 0x27), par(j));
                    *b(e, 0xfd) = 1;
                }
            }
        }
        *b(e, 5) = (*b(e, 5)).wrapping_add(1);
        let mm = *((e + 0x19e) as *const usize);
        let t1 = if mm != 0 { tabela(mm, e) } else { 0 };
        let longe = t1 & 4 == 0 && (if mm != 0 { tabela(mm, e) } else { 0 }) & 2 == 0;
        let ganho: i16 = if dominou { 0x78 } else { 0xc8 };
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if longe { ganho << 1 } else { ganho });
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(a);
        if dominou && sortear(7) == 0 {
            narrar(chamar, 0x1f41, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        }
        return;
    };
    let _ = passa;
    // `j` passa
    if !na_area(j, byte(j, 0x27) == 0) && sortear(*w(j, 0x31) as i32) < -10 {
        crate::comparacao::ramo("dividida cresce");
        let p = f32_em(j + 0x1a2) as f64 * 1.0499999523162842;
        *((j + 0x1a2) as *mut f32) = p as f32;
        let minimo = ((25 - byte(*((j + 0x69) as *const usize), 0x5d) as i32) * 2) as f32;
        let v = if p > minimo as f64 { p } else { minimo as f64 };
        *((j + 0x1a2) as *mut f32) = if v > 100.0 { 100.0 } else { v as f32 };
        narrar(chamar, 0x211d, u32::MAX, zero(j), u32::MAX);
    }
    let lado = byte(j, 0x27);
    *b(m, 0x8eab) = lado as u8;
    if lado != -1 {
        let p = (m as isize + lado as isize + 0x8d46) as *mut u8;
        *p = (*p).wrapping_add(1);
        let q = b(m, 0x8d46 + (lado == 0) as usize);
        if (*q as i8) > 0 {
            *q = (*q).wrapping_sub(10);
        }
    }
    *w(j, 0x19c) = (*w(j, 0x19c)).wrapping_add(at.wrapping_sub(nota)).clamp(-200, 200);
    if e != 0 {
        *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(b2);
    }
    if byte(m, 0x8ea9) == -1 || *w(j, 0x198) == 0x10 {
        return;
    }
    if forte != 0 {
        narrar(chamar, 0x2133, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
    } else if e != 0 {
        narrar(chamar, 0x216d, ub(j, 0x2e), ub(j, 0x27), ub(e, 0x2e) | zero(j) << 8);
    } else {
        let t = if m != 0 { tabela(m, j) } else { 0 };
        let perto = (lado == 1 && byte(j, 0x103) < 3) || (lado == 0 && byte(j, 0x103) > 8);
        if t & 1 == 0 && !perto && pressao(m, lado, byte(j, 0x102), byte(j, 0x103), 1) == 0 {
            narrar(chamar, 0x21de, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        } else {
            narrar(chamar, 0x1f40, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        }
    }
    *b(m, 0x8ea9) = 0;
}

/// Despacho real das chamadas injetadas da dividida (e das rotinas que ela leva junto).
pub(crate) unsafe fn despachar_dividida(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        // codigo | a7 << 16 (a7 com sinal), a5 | a6 << 8
        0x6d6f70 => comentar(this, [a[0] & 0xffff, a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, (a[0] >> 16) as u8 as i8 as u32]),
        0x716e10 | 0x6e1090 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
            f(this, a[0]);
        }
        0x716c90 | 0x6f0630 | 0x6e1aa0 | 0x7103e0 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
            f(this);
        }
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY);
            return f(this, a[0] as u8) as u32;
        }
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, 1, a[4] as usize);
        }
        0x62ed70 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
            return f(this, a[0] as usize) as u32;
        }
        0x6d0980 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_INJURE_PLAYER);
            f(this, a[0], a[1], a[2] as usize, a[3] as usize);
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
            return f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
        }
    }
    0
}

/// { 0x70ec80 } (ret 4, match_pl.cpp): ver `dividida_com`.
pub(crate) unsafe extern "thiscall" fn dividida(j: usize, forte: u32) {
    dividida_com(j, forte as u8, &mut |rotina, this, a| despachar_dividida(rotina, this, a))
}

// ---------- centesima vigesima oitava leva: a bola que chega ao jogador (0x6f5c30) ----------

/// A falta de `j` (tipo `t` da 0x6cfca0, lance `codigo`) punida pelo arbitro (0x6cfd10) no chute
/// (0x6f6c00), igual ao trecho da 0x6f5c30. A 0x6f5c30 guarda a sua copia em linha: chamar esta
/// funcao de la muda a carreira em 2002-01-12 sem mudar a logica (ver PENDENCIAS): 0x1fe8, cartao, bola parada 7, penalti (5, -500 em +0x35 e, por
/// sorteio(+0x5a da ficha), ajuste do +0x17 e desempenho 0x6eecb0) ou falta (6), expulsao, cobrador
/// (0x6ceb90) e as narracoes 0x21df a 0x21e2, e fora dos lances especiais 0x20df/0x20e0/0x20d9, o
/// desgaste do time (0x6bb8d0) e o cansaco. Devolve se foi penalti.
/// Sonda: enche 4 KB da pilha abaixo com 0xa5 (o lixo que uma rotina original chamada depois leria).
#[inline(never)]
pub(crate) fn sujar() {
    let mut a = [0xa5u8; 65536];
    std::hint::black_box(&mut a);
}

pub(crate) unsafe fn punir_falta_de_quem_disputa_com(j: usize, mut t: u8, cartao: u8, grav: u8, codigo: i16, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> bool {
    let punir: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_DECIDE_BOOKING);
    let ajustar: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_ADJUST_MORALE);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let ao_time: unsafe extern "thiscall" fn(usize, usize, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER_FROM_PLAYER);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let m = *((j + 0x19e) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, a3, a4, a5]);
    };
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let mut peso = 0f32;
    punir(m, j, &mut t, cartao as i8, grav as i8, 0, &mut peso);
    narrar(chamar, 0x1fe8, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
    let cor: i32 = match t {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        6 => 0x1ff5,
        _ => {
            if grav as i8 as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    let som: u32;
    if t == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if area_de(j, byte(j, 0x27) == 1) {
        if cor == -1 {
            match sortear(4) {
                0 => narrar(chamar, 0x20e9, u32::MAX, zero(j), u32::MAX),
                1 => narrar(chamar, 0x20ec, u32::MAX, zero(j), u32::MAX),
                _ => {}
            }
        }
        som = 0x1f7a;
        *b(m, 0x8eb2) = 5;
        *((m + 0x478a) as *mut usize) = 0;
        *w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(500);
        if sortear(byte(*((j + 0x69) as *const usize), 0x5a) as i32) == 0 {
            let r = sortear(10);
            if ajustar(j, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho(j, 0);
            }
        }
    } else {
        som = 0x1f79;
        *b(m, 0x8eb2) = 6;
    }
    *b(m, 0x8ea7) = *b(j, 0x102);
    *b(m, 0x8ea8) = *b(j, 0x103);
    *b(m, 0x8eae) = zero(j) as u8;
    *((m + 0xf582) as *mut u32) = 0;
    if som == 0x1f7a {
        narrar(chamar, 0x1f7a, u32::MAX, zero(j), u32::MAX);
    }
    let expulso = matches!(t, 3..=6);
    if cor > -1 {
        if sortear(4) == 0 {
            let a = sortear(grav as i8 as i32);
            if a > sortear(0x14) {
                let partida = *((m + 0x4792) as *const usize);
                let clube = *((partida + if byte(j, 0x27) == 0 { 0x20 } else { 0x1c }) as *const usize);
                let k = *((clube + 0x53) as *const usize);
                if k != 0 && byte(k, 0x77) == 8 {
                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrar(chamar, 0x2109, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        }
        narrar(chamar, cor as u32, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        if expulso {
            jogador_saiu_com(m, j, 0, chamar);
        }
    }
    if som != 0x1f7a {
        let c = cobrador(m, (byte(j, 0x27) == 0) as i8, byte(m, 0x8eb2), 0);
        if c == 0 {
            narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
        } else {
            if t == 7 {
                narrar(chamar, 0x21e2, ub(c, 0x2e), zero(j), u32::MAX);
            } else {
                narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
            }
            if som == 0x1f79 {
                // o cobrador longe do gol que ataca (lado 1: linha < 4; lado 0: > 7)
                let longe = (byte(c, 0x27) == 1 && byte(c, 0x103) < 4) || (byte(c, 0x27) == 0 && byte(c, 0x103) > 7);
                if t == 7 {
                    if longe && !area_de(c, byte(j, 0x27) == 1) {
                        narrar(chamar, 0x21e1, ub(c, 0x2e), zero(j), u32::MAX);
                    }
                } else if longe {
                    narrar(chamar, if (2..=6).contains(&byte(j, 0x102)) { 0x21e0 } else { 0x21df }, ub(c, 0x2e), zero(j), u32::MAX);
                }
            }
        }
    }
    if especial(codigo as i8) == 0 {
        if sortear(9) == 0 {
            narrar(chamar, 0x20df, u32::MAX, ub(j, 0x27), u32::MAX);
            if sortear(3) == 0 {
                narrar(chamar, 0x20e0, u32::MAX, ub(j, 0x27), u32::MAX);
            }
            if (expulso || som == 0x1f7a) && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, ub(j, 0x27), u32::MAX);
            }
        }
        ao_time(m, j, peso);
        chamar(0x716e10, j, [0; 5]);
    }
    som == 0x1f7a
}

/// { 0x6f5c30 } (ret 0x10, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x6cd770 a bola espirra, 0x6f0630 dominio; e as de dentro da saida de
/// campo): o jogador `j` disputa a bola que chega com `forca` (lance `tipo` 0..5, altura `p4`).
/// Devolve 1 se a jogada se resolveu nele (dominou, sofreu falta, a bola espirrou), 0 se nao.
/// Quem cobra (+0xf576) nao disputa; do lado sem a bola, com +0x10d x 2 < sorteio(6) e a linha
/// fora do limite (0x6cb930), tambem nao. O ataque e sorteio(2000 - +0xa5 x 40) para o goleiro na
/// propria area, senao pelo tipo (500, 600, 700, 1000, 1800; o 5 vale sorteio(500000) + 200 e vira
/// 4); com `p4` < 1,5 os tipos 4 e 5 nao disputam; com altura, ataque x (+0x8eaa + 1) / (p4 + 1). Com
/// a bola no proprio lado (+0x8eab) por sorteio(ataque + 100) == 0: 0x1f5e, -300 em +0x4782 e a bola
/// espirra. Senao o lance e o atributo: na propria area o goleiro (ou, sem area e por sorteio, a
/// cabecada 0x25 com +0x164) escolhe entre domínio (1, +0x11c), 0x15 (+0x144), 0x24 (+0x162) e 0x29
/// (+0x16c); a nota e sorteio(atributo) - forca / 2 - ataque: abaixo de -100 so ganha sorteio(2) em
/// +0x2b; abaixo de 150 nada. O arbitro olha a falta (0x6cfca0): 1 em 10 a registra (0x6bb9a0),
/// senao pune (0x6cfd10) com 0x1fe8, cartao, bola parada 7, penalti (5, -500 em +0x35 e, por
/// sorteio(+0x5a da ficha), ajuste do +0x17 e desempenho 0x6eecb0) ou falta (6), expulsao, cobrador
/// (0x6ceb90) e as narracoes 0x21df a 0x21e2, e fora dos lances especiais 0x20df/0x20e0/0x20d9, o
/// desgaste do time (0x6bb8d0) e o cansaco. Sem punicao: o limite e 1500 - p4^2 x 100 + +0x8eaa x 100
/// (minimo 160); com a nota nele, p4 >= 1 e acima de +0x8eaa + 1, o jogador domina (+0x2b, 0x1fe2 do
/// goleiro na area ou 0x1f5f, 0x6f0630, marca o lance, 0x21cd/0x21ce pela media de acertos); senao
/// perde (0x20d4 do goleiro na area, ou 0x213e/0x1f71) e a bola espirra (0x6cd770). Com `conta`, +1
/// em +0x12 e folego (+0x1c ou +10).
pub(crate) unsafe fn responder_com(j: usize, forca: i16, tipo_in: i8, p4: f32, conta: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let limite_: unsafe extern "stdcall" fn(u8, i8) -> u8 = core::mem::transmute(FN_IS_IN_ATTACKING_THIRD);
    let decidir: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_CHECK_FOUL);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let punir: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_DECIDE_BOOKING);
    let ajustar: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_ADJUST_MORALE);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let ao_time: unsafe extern "thiscall" fn(usize, usize, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER_FROM_PLAYER);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let m = *((j + 0x19e) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, a3, a4, a5]);
    };
    // na area que o lado de `p` defende (lado 1: linhas 10-11; os outros: 0-1), colunas 2 a 6
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let na_area = || area_de(j, byte(j, 0x27) == 1);
    if *((m + 0xf576) as *const usize) == j {
        return 0;
    }
    if byte(m, 0x8eae) != byte(j, 0x27) {
        let c = byte(j, 0x10d) as i32;
        if c * 2 < sortear(6) && limite_((byte(j, 0x27) == 0) as u8, byte(j, 0x103)) == 0 {
            return 0;
        }
    }
    let mut tipo = tipo_in;
    let mut ataque: i32 = if tabela(m, j) == 1 && na_area() {
        crate::comparacao::ramo("responder goleiro na area");
        sortear(ftol(2000.0 - f32_em(j + 0xa5) as f64 * 40.0))
    } else {
        match tipo {
            0 => sortear(500),
            1 => sortear(600),
            2 => sortear(700),
            3 => sortear(1000),
            4 => sortear(0x708),
            5 => {
                crate::comparacao::ramo("responder tipo 5");
                tipo = 4;
                sortear(500000) + 200
            }
            _ => {
                // ponytail: a original mostra a caixa "Error" (versao, arquivo, linha 0x17e2); tipo
                // fora de 0..5 nao acontece no jogo
                *(0xb67a34 as *mut u32) = 0;
                return 0;
            }
        }
    };
    if (p4 as f64) < 1.5 && tipo >= 4 {
        return 0;
    }
    let st = p4 as f64 + 1.0;
    if st > 0.0 {
        ataque = ftol(((byte(m, 0x8eaa) as i32 + 1) * ataque as i16 as i32) as f64 / st);
    }
    let espirra = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, dentro: u32, chutador: usize| {
        let r = sortear(byte(m, 0x8eaa) as i32);
        chamar(0x6cd770, m, [ub(j, 0x102), ub(j, 0x103), (r.wrapping_add(1) as u8 as u32) | ub(j, 0x27) << 8, dentro, chutador as u32]);
    };
    if byte(m, 0x8eab) == byte(j, 0x27) {
        if sortear(ataque as i16 as i32 + 100) != 0 {
            return 0;
        }
        crate::comparacao::ramo("responder bola do proprio lado");
        narrar(chamar, 0x1f5e, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        espirra(chamar, 0, j);
        return 1;
    }
    // o lance e o atributo que o disputa
    let em_area = na_area();
    let escolher = |tipo: i8| -> (i16, i16) {
        if tipo == 4 {
            if tabela(m, j) == 1 { (*w(j, 0x11c), 1) } else { (*w(j, 0x144), 0x15) }
        } else if tabela(m, j) != 1 {
            (*w(j, 0x162), 0x24)
        } else if tipo == 3 && sortear(4) != 0 {
            (*w(j, 0x11c), 1)
        } else {
            (*w(j, 0x16c), 0x29)
        }
    };
    let (valor, codigo) = if em_area && tabela(m, j) == 1 {
        escolher(tipo)
    } else {
        let ficha5c = byte(*((j + 0x69) as *const usize), 0x5c) as i32;
        if sortear(em_area as i32 * 500 + 100 + ficha5c * 200) != 0 { escolher(tipo) } else { (*w(j, 0x164), 0x25) }
    };
    let v = sortear(valor as i32);
    let nota = (v - forca as i32 / 2 - ataque) as i16;
    if nota < -100 {
        crate::comparacao::ramo("responder nota muito baixa");
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2) as u8);
        return 0;
    }
    if nota < 0x96 {
        return 0;
    }
    let (mut cartao, mut grav) = (0u8, 0u8);
    let mut t = decidir(m, j, codigo as u16, &mut cartao, &mut grav);
    if t != 0 {
        if sortear(10) == 0 {
            crate::comparacao::ramo("responder falta registrada");
            registrar(m, codigo as u16, j as u32, 0, 0, t, *b(j, 0x102), *b(j, 0x103), cartao, grav);
        } else {
            // a falta com punicao
            crate::comparacao::ramo("responder falta punida");
            let mut peso = 0f32;
            punir(m, j, &mut t, cartao as i8, grav as i8, 0, &mut peso);
            narrar(chamar, 0x1fe8, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
            let cor: i32 = match t {
                2 => 0x1ff9,
                3 => 0x1ff7,
                4 => 0x1ff8,
                6 => 0x1ff5,
                _ => {
                    if grav as i8 as i32 > sortear(7) + 2 { 0x210a } else { -1 }
                }
            };
            let som: u32;
            if t == 7 {
                som = 0x1f79;
                *b(m, 0x8eb2) = 7;
            } else if na_area() {
                if cor == -1 {
                    match sortear(4) {
                        0 => narrar(chamar, 0x20e9, u32::MAX, zero(j), u32::MAX),
                        1 => narrar(chamar, 0x20ec, u32::MAX, zero(j), u32::MAX),
                        _ => {}
                    }
                }
                crate::comparacao::ramo("responder penalti");
                som = 0x1f7a;
                *b(m, 0x8eb2) = 5;
                *((m + 0x478a) as *mut usize) = 0;
                *w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(500);
                if sortear(byte(*((j + 0x69) as *const usize), 0x5a) as i32) == 0 {
                    let r = sortear(10);
                    if ajustar(j, (r as u8).wrapping_neg() as i8) != 0 {
                        desempenho(j, 0);
                    }
                }
            } else {
                som = 0x1f79;
                *b(m, 0x8eb2) = 6;
            }
            *b(m, 0x8ea7) = *b(j, 0x102);
            *b(m, 0x8ea8) = *b(j, 0x103);
            *b(m, 0x8eae) = zero(j) as u8;
            *((m + 0xf582) as *mut u32) = 0;
            if som == 0x1f7a {
                narrar(chamar, 0x1f7a, u32::MAX, zero(j), u32::MAX);
            }
            let expulso = matches!(t, 3..=6);
            if cor > -1 {
                if sortear(4) == 0 {
                    let a = sortear(grav as i8 as i32);
                    if a > sortear(0x14) {
                        let partida = *((m + 0x4792) as *const usize);
                        let clube = *((partida + if byte(j, 0x27) == 0 { 0x20 } else { 0x1c }) as *const usize);
                        let k = *((clube + 0x53) as *const usize);
                        if k != 0 && byte(k, 0x77) == 8 {
                            narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                        }
                    }
                    narrar(chamar, 0x2109, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                }
                narrar(chamar, cor as u32, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                if expulso {
                    jogador_saiu_com(m, j, 0, chamar);
                }
            }
            if som != 0x1f7a {
                let c = cobrador(m, (byte(j, 0x27) == 0) as i8, byte(m, 0x8eb2), 0);
                if c == 0 {
                    narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
                } else {
                    if t == 7 {
                        narrar(chamar, 0x21e2, ub(c, 0x2e), zero(j), u32::MAX);
                    } else {
                        narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
                    }
                    if som == 0x1f79 {
                        // o cobrador longe do gol que ataca (lado 1: linha < 4; lado 0: > 7)
                        let longe = (byte(c, 0x27) == 1 && byte(c, 0x103) < 4) || (byte(c, 0x27) == 0 && byte(c, 0x103) > 7);
                        if t == 7 {
                            if longe && !area_de(c, byte(j, 0x27) == 1) {
                                narrar(chamar, 0x21e1, ub(c, 0x2e), zero(j), u32::MAX);
                            }
                        } else if longe {
                            narrar(chamar, if (2..=6).contains(&byte(j, 0x102)) { 0x21e0 } else { 0x21df }, ub(c, 0x2e), zero(j), u32::MAX);
                        }
                    }
                }
            }
            if especial(codigo as i8) == 0 {
                if sortear(9) == 0 {
                    narrar(chamar, 0x20df, u32::MAX, ub(j, 0x27), u32::MAX);
                    if sortear(3) == 0 {
                        narrar(chamar, 0x20e0, u32::MAX, ub(j, 0x27), u32::MAX);
                    }
                    if (expulso || som == 0x1f7a) && sortear(2) == 0 {
                        narrar(chamar, 0x20d9, u32::MAX, ub(j, 0x27), u32::MAX);
                    }
                }
                ao_time(m, j, peso);
                chamar(0x716e10, j, [0; 5]);
            }
            return 1;
        }
    }
    let p = p4 as f64;
    let mut lim = ftol((1500.0 - p * p * 100.0) + (byte(m, 0x8eaa) as i32 * 100) as f64) as i16;
    if lim < 0xa0 {
        lim = 0xa0;
    }
    if !(nota < lim) && !(p < 1.0) && ((byte(m, 0x8eaa) as i32 + 1) as f64) < p {
        // dominou
        crate::comparacao::ramo("responder dominou");
        if conta != 0 {
            *b(j, 0x12) = (*b(j, 0x12)).wrapping_add(1);
            *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1c);
        }
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(1);
        if tabela(m, j) == 1 && na_area() {
            narrar(chamar, 0x1fe2, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        } else if byte(m, 0x8ea9) < 4 {
            narrar(chamar, 0x1f5f, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        }
        chamar(0x6f0630, j, [0; 5]);
        *w(j, 0x198) = codigo;
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        let (contas, sorteio) = if codigo == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let media = if byte(j, contas.0) != 0 { (ub(j, contas.1) as f64 / ub(j, contas.0) as f64) as f32 } else { 0.0 };
        if minuto > (sortear(0x14) + sorteio) as u8 as i8 {
            let gols = byte(m, if byte(j, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
            let tentativas = if codigo == 0x15 { ub(j, 8) > 3 && byte(j, 9) != 0 } else { ub(j, 6) > 1 };
            if gols == 0 && media as f64 > 0.725 && tentativas && *w(j, 0x35) > 0x1a5e && byte(j, 0xfd) == 0 {
                crate::comparacao::ramo("responder media de acertos");
                narrar(chamar, if codigo == 0x15 { 0x21ce } else { 0x21cd }, ub(j, 0x2e), ub(j, 0x27), ub(j, 0x2e) | ub(j, 0x27) << 8);
                *b(j, 0xfd) = 1;
            }
        }
        return 1;
    }
    // perdeu: a bola espirra
    if conta != 0 {
        *b(j, 0x12) = (*b(j, 0x12)).wrapping_add(1);
        *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(10);
    }
    let (dentro, chutador) = if tabela(m, j) == 1 && na_area() {
        crate::comparacao::ramo("responder goleiro perdeu");
        narrar(chamar, 0x20d4, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        if sortear(10) != 0 {
            (1, 0)
        } else if sortear(200) != 0 {
            (0, 0)
        } else {
            (0, j)
        }
    } else {
        crate::comparacao::ramo("responder perdeu");
        narrar(chamar, if tipo <= 2 { 0x213e } else { 0x1f71 }, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        (0, j)
    };
    espirra(chamar, dentro, chutador);
    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2) as u8);
    *w(j, 0x198) = codigo;
    1
}

/// { 0x6f5c30 } (ret 0x10, match_pl.cpp): ver `responder_com`.
pub(crate) unsafe extern "thiscall" fn responder(j: usize, forca: u32, tipo: u32, p4: u32, conta: u32) -> u8 {
    responder_com(j, forca as i16, tipo as i8, f32::from_bits(p4), conta as u8, &mut |rotina, this, a| despachar_dividida(rotina, this, a))
}

// ---------- centesima trigesima sexta leva: o chute (0x6f6c00) ----------

/// { 0x6f6c00 } (ret 0xc, match_pl.cpp, chamada so pela 0x6b98c0) com as chamadas que mexem fora do
/// motor injetadas (0x6d6f70 narra, 0x716e10 cansa, 0x6f0630 dominio, 0x6cd770 a bola espirra,
/// 0x716c90, 0x7103e0 o lance seguinte; e as de dentro da lesao, da defesa e da saida de campo): o
/// jogador `j` chuta com `forca` mirando (x, y) (x = -1: a bola parada +0x8ea7/+0x8ea8).
/// A chance pela distancia ao gol (lado 0 pela tabela 0xa15e44, lado 1 pela 0x6f09b0); o folego
/// (sorteio((10000 - +0x4d)^2 / 50000)); o angulo da direcao (+0x19a) contra +0x8ed8 pesado pelo lance
/// (+0x8ea9); a forca perde 20 x (+0x8eaa - 1)^3 (e mais pelo byte 1 do motor). Com angulo ate 90 o
/// jogador vira (0x6f4920) para o gol (por +0x10c x +0x105, chance abaixo de 2 ou 0x6bd5b0) ou para o
/// alvo. O lance e o atributo: a cabecada 0x25 (+0x164), ou pelo +0x8ea9: 4+ fora da area de ataque
/// com forca e caminho livre (0x6c41b0) o 0xc ((+0x10c - 20) x 10 + +0x132, +0x8ea9 vira 2), senao
/// 0x2e (+0x176) ou 0x15 (+0x144) pelo +0x1a2 e +0xff contra +0x24; 2 e 3 o 0x3b (+0x190) perto do gol
/// (0x6f9f90, chance abaixo de 1,41 e sorteios), senao 0xc (+0x132); os outros 0xd (+0x134). A nota e
/// sorteio(atributo) - sorteio(folego) - sorteio(angulo) + sorteio(forca); 0x6f56c0 cansa. O arbitro
/// olha a falta do chutador (0x6cfca0): por sorteio(chance x 5 + 2) so registra (0x6bb9a0), senao pune
/// como a 0x6f5c30. O desafiante sai da 0x6c87e0 (lesao 0x6cfe10 logo); ele (nota >= a do chute ou 1
/// em 5) e o chutador (nota do chute maior ou 1 em 5) podem fazer falta (0x6cfc60: registrada 1 em 30
/// / 1 em 10 ou punida, a do desafiante como na dividida e com ajuste do +0x17 no penalti). Com a nota
/// do desafiante, ele fica com a bola: falta narrada na area, defesa do goleiro (0x70d430) ou a
/// narracao do lance, 0x6f0630, +0x19c, a bola espirra abaixo do limite do lance, contas e folego
/// (+0x35, 0x21cd/0x21ce, +0x17) e 1 em 7 0x1f41. Senao o chute segue: 0x6f0630, +0x4782/+0x4786
/// perdem a nota x 2/11; de cabeca (+0x8ea9 4) 0x2164/0x21cb/0x214f/0x1f5d, contas e 0x7103e0; senao
/// 0x216e (no 0xc) ou as narracoes do chute de longe (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c ganha a
/// nota - a do desafiante - 200 (+-200) e por sorteio(+0x19c) contra sorteio(200) 0x7103e0 (com
/// +0x1a7 e +100 de posse). Fora da area de ataque, sorteio(+0x31) < -10 faz o jogador crescer (0x211d).
pub(crate) unsafe fn chute_com(j: usize, forca_in: i16, x_in: i8, y_in: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    let caminho: unsafe extern "thiscall" fn(usize, usize, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_PATH_AHEAD_CLEAR);
    let apontar_: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_SET_FACING_DIRECTION);
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let livre: unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_CLEAR_AHEAD_OF_CELL);
    let desgastar: unsafe extern "thiscall" fn(usize, f32, i16) = core::mem::transmute(FN_MATCH_PLAYER_DRAIN_STAMINA);
    let decidir: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_CHECK_FOUL);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let desafio: unsafe extern "thiscall" fn(usize, usize, i8, i8, *mut u8, *mut u8, *mut i16, *mut i8, *mut i8, u8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OPPOSITION_CHALLENGE);
    let falta: unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_JUDGE_TACKLE);
    let punir: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_DECIDE_FOUL_PUNISHMENT);
    let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let ao_time: unsafe extern "thiscall" fn(usize, usize, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER_FROM_PLAYER);
    let ajustar: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_ADJUST_MORALE);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    let atras: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_BEHIND_OWN_BACK_LINE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let par = |p: usize| ub(p, 0x2e) | ub(p, 0x27) << 8;
    let mais = |p: *mut i16, v: i16| *p = (*p).wrapping_add(v);
    let um = |p: *mut u8| *p = (*p).wrapping_add(1);
    let m = *((j + 0x19e) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, a3, a4, a5]);
    };
    // nas colunas 2 a 6, linhas 10-11 (`lado1`) ou 0-1
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    // o desafiante longe do gol dele pela tabela e atras do extremo, ou o chutador sem como bater
    let conta_o_chute = |e: usize| {
        let dy = byte(e, 0x103) as i32 - if byte(e, 0x27) != 0 { 11 } else { 0 };
        !(tabela_12x9(dy, byte(e, 0x102) as i32 - 4) as f64 > 1.0 && atras(*((e + 0x19e) as *const usize), e) == 0) && pode_bater(j)
    };
    let (jx, jy) = (byte(j, 0x102), byte(j, 0x103));
    let chance = if byte(j, 0x27) == 0 { tabela_12x9(jy as i32 - 11, jx as i32 - 4) } else { dist_ponto(j, 4, 0) };
    let mut tipo = byte(m, 0x8ea9);
    let (x, y) = if x_in == -1 { (byte(m, 0x8ea7), byte(m, 0x8ea8)) } else { (x_in, y_in) };
    let d = 10000 - ftol(f32_em(j + 0x4d) as f64) as i16 as i32;
    let folego = sortear(d.wrapping_mul(d) / 50000);
    let mut ang = (*w(j, 0x19a) as i32 - *w(m, 0x8ed8) as i32).abs();
    if ang as i16 > 0xb4 {
        ang = 0x168 - ang;
    }
    let q = (((0xb4 - ang) * 2) as i16 as i32) / ((5 - tipo as i32).max(1) as i16 as i32);
    let angulo = if (ang as i16) < 0x5a { (11 - ang as i16 as i32 / 10) * q } else { q * 2 };
    let c = (*b(m, 0x8eaa)).wrapping_sub(1) as i8 as i16;
    let mut forca = forca_in.wrapping_sub(c.wrapping_mul(c).wrapping_mul(c).wrapping_mul(20));
    if c != 0 {
        match byte(m, 1) {
            2 => forca = forca.wrapping_sub(((sortear(3) + 1) as i16).wrapping_mul(c)),
            0x20 => forca = forca.wrapping_sub(((sortear(8) + 3) as i16).wrapping_mul(c)),
            _ => {}
        }
    }
    let dono = *((m + 0xf57a) as *const usize);
    // vira para o gol ou para o alvo
    if (ang as i16) <= 0x5a {
        let r = sortear(200);
        let ao_gol = byte(j, 0x10c) as i32 * byte(j, 0x105) as i32 > r || (chance as f64) < 2.0 || caminho(m, j, 0) != 0;
        if ao_gol {
            if byte(j, 0x27) == 0 {
                apontar_(j, jx, jy, 4, if jy < 9 { jy + 3 } else { 11 });
            } else {
                apontar_(j, jx, jy, 4, if jy > 3 { jy - 3 } else { 0 });
            }
        } else if byte(m, 0x8ea9) < 4 {
            apontar_(j, jx, jy, x, y);
        }
    }
    // o lance e o atributo
    let mut doze = false;
    let na_propria = area_de(j, byte(j, 0x27) == 1);
    let cabeca = !(na_propria && tabela(m, j) == 1)
        && sortear(na_propria as i32 * 500 + 100 + byte(*((j + 0x69) as *const usize), 0x5c) as i32 * 200) == 0;
    let (attr, lance): (i16, i8) = if cabeca {
        crate::comparacao::ramo("chute cabecada");
        (*w(j, 0x164), 0x25)
    } else if tipo >= 4 {
        if !area_de(j, byte(j, 0x27) == 0)
            && forca as i32 > (20 - byte(j, 0x10c) as i32) * 7
            && livre(m, byte(j, 0x27), jx, jy) != 0
        {
            crate::comparacao::ramo("chute 0xc de longe");
            doze = true;
            *b(m, 0x8ea9) = 2;
            tipo = byte(m, 0x8ea9);
            ((byte(j, 0x10c) as i16 - 20).wrapping_mul(10).wrapping_add(*w(j, 0x132)), 0xc)
        } else {
            let v = ftol(((f32_em(j + 0x1a2) as f64 * f64_em(0x967898) + 0.75) * byte(j, 0xff) as f64 + 5.0) * 0.5) as i8;
            let r1 = sortear(byte(j, 0x24) as i32 * 80 + na_propria as i32 * 1500 + 0x46);
            if sortear(v as i32) > r1 { (*w(j, 0x176), 0x2e) } else { (*w(j, 0x144), 0x15) }
        }
    } else if tipo != 3 && tipo != 2 {
        (*w(j, 0x134), 0xd)
    } else {
        let ok = pode_bater(j)
            && (chance as f64) < 1.41
            && {
                let r1 = sortear(*w(j, 0x132) as i32);
                sortear(*w(j, 0x190) as i32) > r1
            }
            && sortear(ftol(f32_em(j + 0x99) as f64)) >= sortear(5);
        if ok {
            crate::comparacao::ramo("chute 0x3b");
            (*w(j, 0x190), 0x3b)
        } else {
            (*w(j, 0x132), 0xc)
        }
    };
    let s1 = sortear(attr as i32);
    let s2 = sortear(folego as i16 as i32);
    let s3 = sortear(angulo as i16 as i32);
    let nota_chute = (s1 - s2 - s3 + sortear(forca as i32)) as i16;
    desgastar(j, -1.0, lance as i16);
    // a falta do chutador
    let (mut cartao, mut grav) = (0u8, 0u8);
    let t = decidir(m, j, lance as i16 as u16, &mut cartao, &mut grav);
    if t != 0 {
        if sortear(ftol(chance as f64 * 5.0 + 2.0)) == 0 {
            crate::comparacao::ramo("chute falta registrada");
            registrar(m, lance as i16 as u16, j as u32, 0, 0, t, *b(j, 0x102), *b(j, 0x103), cartao, grav);
        } else {
            crate::comparacao::ramo("chute falta punida");
            punir_falta_de_quem_disputa_com(j, t, cartao, grav, lance as i16, chamar);
            mais(w(j, 0x35), -0x19);
            return;
        }
    }
    // o desafiante
    *((m + 0xf582) as *mut usize) = j;
    let (mut a, mut b2, mut nota, mut ld, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = desafio(m, j, byte(j, 0x27), lance, &mut a, &mut b2, &mut nota, &mut ld, &mut arriscado, 0);
    lesao_na_dividida_com(m, j, lance as i16, e, ld as i16, 0, chamar);
    *((m + 0xf582) as *mut usize) = 0;
    let gr = &mut grav as *mut u8 as *mut i8;
    'disputa: {
        if e == 0 {
            crate::comparacao::ramo("chute sem desafiante");
            break 'disputa;
        }
        let mut vantagem = 0u8;
        let mut checar_chutador = true;
        // a falta do desafiante
        if nota >= nota_chute || sortear(5) == 0 {
            let td = falta(m, e, j, ld, -1, &mut cartao, gr, &mut vantagem);
            if byte(m, 0x8ebb) != 0 && area_de(e, byte(j, 0x27) == 0)
                && (especial(ld) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
                && sortear(3) != 0
            {
                narrar(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
            }
            if td != 0 {
                if sortear(0x1e) == 0 && arriscado == 0 {
                    crate::comparacao::ramo("chute desafiante registrado");
                    registrar(m, ld as i16 as u16, e as u32, j as u32, 0, td, *b(j, 0x102), *b(j, 0x103), cartao, grav);
                    checar_chutador = false;
                } else {
                    crate::comparacao::ramo("chute desafiante punido");
                    punir_desafiante_com(m, e, j, td, cartao, grav as i8, arriscado, vantagem, ld, true, &["chute penalti do desafiante", "chute expulsao do desafiante", "chute cobrador do desafiante", "chute vantagem do desafiante"], chamar);
                    return;
                }
            }
        }
        // a falta do chutador sobre o desafiante
        if checar_chutador && (nota_chute > nota || sortear(5) == 0) {
            let mut tj = falta(m, j, e, lance, -1, &mut cartao, gr, &mut vantagem);
            if byte(m, 0x8ebb) != 0 && area_de(j, byte(j, 0x27) == 1)
                && (especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
                && sortear(3) != 0
            {
                narrar(chamar, 0x2168, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
            }
            if tj != 0 {
                if sortear(10) == 0 {
                    crate::comparacao::ramo("chute chutador registrado");
                    registrar(m, lance as i16 as u16, j as u32, e as u32, 0, tj, *b(j, 0x102), *b(j, 0x103), cartao, grav);
                } else {
                    crate::comparacao::ramo("chute chutador punido");
                    let mut peso = 0f32;
                    punir(m, j, e, &mut tj, cartao as i8, grav as i8, 0, &mut peso, vantagem);
                    narrar(chamar, if lance == 0x38 { 0x1ff3 } else { narracao_da_falta(lance) }, ub(j, 0x2e), ub(j, 0x27), par(e));
                    let cor: i32 = match tj {
                        2 => 0x1ff9,
                        3 => 0x1ff7,
                        4 => 0x1ff8,
                        5 => 0x1ff6,
                        _ => {
                            if grav as i8 as i32 > sortear(7) + 2 { 0x210a } else { -1 }
                        }
                    };
                    let som: u32;
                    if tj == 7 {
                        som = 0x1f79;
                        *b(m, 0x8eb2) = 7;
                    } else if area_de(j, byte(j, 0x27) == 1) {
                        if cor == -1 {
                            match sortear(4) {
                                0 => narrar(chamar, 0x20e9, u32::MAX, zero(j), u32::MAX),
                                1 => narrar(chamar, 0x20ec, u32::MAX, zero(j), u32::MAX),
                                _ => {}
                            }
                        }
                        crate::comparacao::ramo("chute penalti do chutador");
                        som = 0x1f7a;
                        *b(m, 0x8eb2) = 5;
                        *((m + 0x478a) as *mut usize) = e;
                        mais(w(j, 0x35), -500);
                        if sortear(byte(*((j + 0x69) as *const usize), 0x5a) as i32) == 0 {
                            let r = sortear(10);
                            if ajustar(j, (r as u8).wrapping_neg() as i8) != 0 {
                                desempenho(j, 0);
                            }
                        }
                    } else {
                        som = 0x1f79;
                        *b(m, 0x8eb2) = 6;
                    }
                    *b(m, 0x8ea7) = *b(j, 0x102);
                    *b(m, 0x8ea8) = *b(j, 0x103);
                    *b(m, 0x8eae) = zero(j) as u8;
                    *((m + 0xf582) as *mut u32) = 0;
                    if som == 0x1f7a {
                        narrar(chamar, 0x1f7a, u32::MAX, zero(j), u32::MAX);
                    }
                    let expulso = matches!(tj, 3..=6);
                    if cor > -1 {
                        if sortear(4) == 0 {
                            let g = sortear(grav as i8 as i32);
                            if g > sortear(0x14) {
                                let partida = *((m + 0x4792) as *const usize);
                                let clube = *((partida + if byte(j, 0x27) == 0 { 0x20 } else { 0x1c }) as *const usize);
                                let k = *((clube + 0x53) as *const usize);
                                if k != 0 && byte(k, 0x77) == 8 {
                                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                                }
                            }
                            narrar(chamar, 0x2109, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                        }
                        narrar(chamar, cor as u32, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                        if expulso {
                            crate::comparacao::ramo("chute expulsao do chutador");
                            jogador_saiu_com(m, j, 0, chamar);
                        }
                    }
                    if som != 0x1f7a {
                        let cb = cobrador(m, zero(j) as i8, byte(m, 0x8eb2), 0);
                        if cb == 0 {
                            narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
                        } else {
                            if tj == 7 {
                                narrar(chamar, 0x21e2, ub(cb, 0x2e), zero(j), u32::MAX);
                            } else {
                                narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
                            }
                            if som == 0x1f79 && vantagem == 0 {
                                // o cobrador longe do gol que ataca (lado 1: linha < 4; lado 0: > 7)
                                let longe = (byte(cb, 0x27) == 1 && byte(cb, 0x103) < 4) || (byte(cb, 0x27) == 0 && byte(cb, 0x103) > 7);
                                if tj == 7 {
                                    if longe && !area_de(cb, byte(j, 0x27) == 1) {
                                        narrar(chamar, 0x21e1, ub(cb, 0x2e), zero(j), u32::MAX);
                                    }
                                } else if longe {
                                    narrar(chamar, if (2..=6).contains(&byte(j, 0x102)) { 0x21e0 } else { 0x21df }, ub(cb, 0x2e), zero(j), u32::MAX);
                                }
                            }
                        }
                    }
                    if especial(lance) == 0 {
                        let extra = expulso || som == 0x1f7a;
                        if vantagem != 0 {
                            crate::comparacao::ramo("chute vantagem do chutador");
                            narrar(chamar, 0x20dd, ub(e, 0x2e), zero(j), u32::MAX);
                            if matches!(tj, 4..=6) {
                                *b(e, 0x16) = 1;
                            }
                            if extra && sortear(3) != 0 {
                                narrar(chamar, 0x20d9, u32::MAX, ub(j, 0x27), u32::MAX);
                            }
                        } else if sortear(9) == 0 {
                            narrar(chamar, 0x20df, u32::MAX, ub(j, 0x27), u32::MAX);
                            if sortear(3) == 0 {
                                narrar(chamar, 0x20e0, u32::MAX, ub(j, 0x27), u32::MAX);
                            }
                            if extra && sortear(2) != 0 {
                                narrar(chamar, 0x20d9, u32::MAX, ub(j, 0x27), u32::MAX);
                            }
                        }
                        ao_time(m, j, peso);
                        chamar(0x716e10, j, [0; 5]);
                    }
                    mais(w(j, 0x35), -0x19);
                    return;
                }
            }
        }
        let penalti = vantagem != 0 && (area_de(j, byte(j, 0x27) == 0) || arriscado != 0);
        *w(e, 0x198) = ld as i16;
        if tipo != 4 {
            um(b(e, 4));
            mais(w(e, 0x35), if tabela(m, e) & 0x30 != 0 && tabela(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
        }
        if nota_chute > nota {
            break 'disputa;
        }
        // o desafiante fica com a bola
        crate::comparacao::ramo("chute desafiante ganha");
        let narrou = (especial(ld) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
            && area_de(e, byte(e, 0x27) == 1)
            && sortear(2) == 0;
        if narrou {
            crate::comparacao::ramo("chute desafiante narra falta na area");
            narrar(chamar, narracao_da_falta(ld), ub(e, 0x2e), ub(e, 0x27), par(j));
            if tipo < 4 {
                let codigo = match sortear(0x32) {
                    1 => 0x20ea,
                    0 | 2 | 5 | 6 | 9 => 0x20eb,
                    _ => 0x20e8,
                };
                narrar(chamar, codigo, u32::MAX, zero(e), u32::MAX);
            }
        } else {
            let codigo = match ld {
                1 => {
                    crate::comparacao::ramo("chute defesa do goleiro");
                    if defender_com(e, nota.wrapping_sub(nota_chute), chamar) == 0 {
                        return;
                    }
                    0
                }
                0x13 | 0x21 | 0x2d => 0x1f62,
                0x15 | 0x2e => {
                    let r = sortear(6);
                    if ((r as f64 + f32_em(e + 0xad) as f64) + 3.0) < f32_em(j + 0xad) as f64 { 0x2164 } else { 0x1f5d }
                }
                0x22 | 0x2c => 0x2150,
                0x29 => 0x1f61,
                0x2a | 0x2b => 0x1f64,
                _ => 0,
            };
            if codigo != 0 {
                narrar(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), par(j));
            }
        }
        chamar(0x6f0630, e, [0; 5]);
        *w(e, 0x19c) = nota.wrapping_sub(nota_chute).wrapping_sub(200).clamp(-200, 200);
        let mut dominou = false;
        let limite = match ld {
            0x13 => 400,
            0x21 | 0x2d => 250,
            _ => 100,
        };
        if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
            crate::comparacao::ramo("chute bola espirra");
            chamar(0x6cd770, m, [ub(j, 0x102), ub(j, 0x103), 1 | zero(j) << 8, 0, 0]);
            if byte(m, 0x8eb2) == 0 {
                let mut k = 3;
                k -= (byte(e, 0x10d) as i32 > sortear(10)) as i32;
                k -= (byte(e, 0x10c) as i32 > sortear(10)) as i32;
                *b(e, 0x2b) = sortear(k) as u8;
                dominou = true;
            } else if penalti {
                crate::comparacao::ramo("chute 716c90");
                chamar(0x716c90, j, [0; 5]);
            }
        } else {
            if (0..4).contains(&byte(m, 0x8ea9)) {
                *b(m, 0x8ea9) = 0;
            }
            let (x, y) = (byte(j, 0x102), byte(j, 0x103));
            if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
                apontar_(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
            } else {
                apontar_(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
            }
            let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
            let (contas, sorteio) = if ld == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
            let media = if byte(e, contas.0) != 0 { (ub(e, contas.1) as f64 / ub(e, contas.0) as f64) as f32 } else { 0.0 };
            if minuto > (sortear(0x14) + sorteio) as u8 as i8 {
                let me = *((e + 0x19e) as *const usize);
                let gols = byte(me, if byte(e, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
                let tentativas = if ld == 0x15 { ub(e, 8) > 3 && byte(e, 9) != 0 } else { ub(e, 6) > 1 };
                if gols == 0 && media as f64 > 0.725 && tentativas && *w(e, 0x35) > 0x1a5e && byte(e, 0xfd) == 0 {
                    crate::comparacao::ramo("chute media de acertos");
                    narrar(chamar, if ld == 0x15 { 0x21ce } else { 0x21cd }, ub(e, 0x2e), ub(e, 0x27), par(j));
                    *b(e, 0xfd) = 1;
                }
            }
        }
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(a);
        let ganho: i16 = if dominou { 0x78 } else { 0xc8 };
        mais(w(e, 0x35), ganho);
        if tipo != 4 {
            um(b(e, 5));
            let me = *((e + 0x19e) as *const usize);
            let t1 = if me != 0 { tabela(me, e) } else { 0 };
            let longe = t1 & 4 == 0 && (if me != 0 { tabela(me, e) } else { 0 }) & 2 == 0;
            mais(w(e, 0x35), if longe { ganho.wrapping_mul(2) } else { ganho });
            if conta_o_chute(e) {
                mais(w(e, 0x35), 300);
                um(b(e, 6));
                if sortear(byte(e, 0x17) as i32 * 30 + 0x46) == 0 {
                    let r = sortear(3);
                    if ajustar(e, r as i8) != 0 {
                        desempenho(e, 0);
                    }
                }
            }
        } else {
            crate::comparacao::ramo("chute desafiante ganha de cabeca");
            um(b(e, 7));
            um(b(e, 8));
            *w(e, 0x198) = 0x7b;
            mais(w(e, 0x35), 0x28);
            mais(w(j, 0x35), -10);
            um(b(j, 7));
            if conta_o_chute(e) {
                mais(w(e, 0x35), 300);
                um(b(e, 9));
                if sortear(byte(e, 0x17) as i32 * 30 + 0x46) == 0 {
                    let r = sortear(2);
                    if ajustar(e, r as i8) != 0 {
                        desempenho(e, 0);
                    }
                }
            }
        }
        if dominou && sortear(7) == 0 {
            narrar(chamar, 0x1f41, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        }
        return;
    }
    // o chute segue
    crate::comparacao::ramo("chute segue");
    chamar(0x6f0630, j, [0; 5]);
    mais(w(j, 0x35), 4);
    *w(j, 0x198) = lance as i16;
    let v = ftol(nota_chute as f64 * f64_em(0x96d7a0));
    let posse = |o: usize| *((m + o) as *mut i32) = (*((m + o) as *const i32)).wrapping_sub(v);
    posse(0x4782);
    posse(0x4786);
    if e != 0 {
        *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(b2);
    }
    let somar_19c = || *w(j, 0x19c) = (*w(j, 0x19c)).wrapping_add(nota_chute.wrapping_sub(nota).wrapping_sub(200)).clamp(-200, 200);
    if tipo == 4 {
        if e != 0 {
            crate::comparacao::ramo("chute de cabeca com desafiante");
            let (pj, pe) = (*((j + 0x6d) as *const usize), *((e + 0x6d) as *const usize));
            let me = *((e + 0x19e) as *const usize);
            let codigo = 'n: {
                if pj != 0 {
                    if pe != 0 {
                        let r = sortear(3);
                        if r + byte(pj, 0x2e) as i32 + 3 < byte(pe, 0x2e) as i32 {
                            break 'n 0x2164;
                        }
                    }
                    if pe != 0 && !(me != 0 && tabela(me, e) == 1) {
                        let r = sortear(3);
                        if byte(pj, 0x2e) as i32 > r + byte(pe, 0x2e) as i32 + 5 {
                            break 'n 0x21cb;
                        }
                    }
                }
                if me != 0 && tabela(me, e) == 1 && sortear(5) != 0 { 0x214f } else { 0x1f5d }
            };
            narrar(chamar, codigo, ub(j, 0x2e), ub(j, 0x27), par(e));
            *w(j, 0x198) = 0x7b;
            um(b(e, 7));
            let t1 = if me != 0 { tabela(me, e) } else { 0 };
            mais(w(e, 0x35), if t1 != 1 { -10 } else { -600 });
            mais(w(j, 0x35), 0x28);
            um(b(j, 7));
            um(b(j, 8));
            if conta_o_chute(e) {
                mais(w(j, 0x35), 300);
                um(b(j, 9));
                if sortear(byte(e, 0x17) as i32 * 30 + 0x46) == 0 {
                    let r = sortear(2);
                    if ajustar(e, r as i8) != 0 {
                        desempenho(e, 0);
                    }
                }
            }
        }
        somar_19c();
        chamar(0x7103e0, j, [0; 5]);
    } else {
        if doze {
            narrar(chamar, 0x216e, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        } else {
            let (jy, jx) = (byte(j, 0x103), byte(j, 0x102));
            if dono == j && tipo < 2 && ((jy != 0 && jy != 11) || (4 - jx as i32).abs() <= 1) {
                crate::comparacao::ramo("chute narra de longe");
                let lado = byte(j, 0x27);
                let codigo = if forca < -500 {
                    0x21cc
                } else if ((lado == 0 && jy < 3) || (lado == 1 && jy > 8)) && sortear(2) != 0 {
                    0x21db
                } else if ((lado == 1 && byte(j, 0x103) >= 6) || (lado == 0 && byte(j, 0x103) < 6)) && sortear(2) != 0 {
                    0x21dc
                } else {
                    0x1f47
                };
                narrar(chamar, codigo, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
            }
            somar_19c();
        }
        let r = sortear(*w(j, 0x19c) as i32);
        if r >= sortear(200) {
            if tipo < 4 {
                *b(m, 0x1a7) = 1;
                *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_add(100);
                *((m + 0x4786) as *mut i32) = (*((m + 0x4786) as *const i32)).wrapping_add(100);
            }
            chamar(0x7103e0, j, [0; 5]);
        }
    }
    // fora da area de ataque o jogador pode crescer
    if !area_de(j, byte(j, 0x27) == 0) && sortear(*w(j, 0x31) as i32) < -10 {
        crate::comparacao::ramo("chute cresce");
        let p = f32_em(j + 0x1a2) as f64 * 1.0499999523162842;
        *((j + 0x1a2) as *mut f32) = p as f32;
        let minimo = ((25 - byte(*((j + 0x69) as *const usize), 0x5d) as i32) * 2) as f32;
        let v = if p > minimo as f64 { p } else { minimo as f64 };
        *((j + 0x1a2) as *mut f32) = if v > 100.0 { 100.0 } else { v as f32 };
        narrar(chamar, 0x211d, u32::MAX, zero(j), u32::MAX);
    }
}

/// the chance by distance to goal (table 0xa15e44 on side 0, 0x6f09b0 on side 1), stamina, the direction angle (+0x19a
/// against +0x8ed8) weighted by the play +0x8ea9, power minus 20 × (+0x8eaa − 1)³ (and more by the engine's byte 1);
/// with an angle up to 90 the player turns to the goal or to the target (0x6f4920). The play and the attribute: header
/// 0x25 (+0x164); with +0x8ea9 4+ the long-range 0xc (power and clear path 0x6c41b0, +0x8ea9 becomes 2), 0x2e (+0x176)
/// or 0x15 (+0x144); with 2/3 the 0x3b (+0x190) near goal, else 0xc (+0x132); the others 0xd (+0x134). Score =
/// draw(attribute) − draw(stamina) − draw(angle) + draw(power). Three possible fouls: the shooter's before (0x6cfca0;
/// recorded by draw(chance × 5 + 2), else punished as in 0x6f5c30), the challenger's (0x6c87e0, injury 0x6cfe10;
/// punished as in the tackle, plus the +0x17 adjustment on a penalty) and the shooter's on him (narration by the play
/// table, penalty given to the challenger). A challenger with the higher score takes the ball (foul narrated in the
/// box, keeper save 0x70d430 or the play's narration; the ball spills below the limit; counts, stamina, +0x17; 1 in 7
/// 0x1f41). Otherwise the shot goes on: +0x4782/+0x4786 lose score × 2/11; headed 0x2164/0x21cb/0x214f/0x1f5d and
/// 0x7103e0; else 0x216e or the long-range narrations (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c gains the difference
/// (±200) and by draw 0x7103e0 (with +0x1a7 and +100 possession); outside the attacking box the player may grow
/// (0x211d). The tackle's punished foul became a shared function (`punir_desafiante_com`); the shooter's
/// (`punir_falta_de_quem_disputa_com`) repeats 0x6f5c30's, which keeps its inline copy (see COMO_CONTINUAR). Compared
/// through the tape
/// Original: `shot` (0x6f6c00, match_pl.cpp (12.7 KB, called only by 0x6b98c0 with power +0x8ed6 and target +0x1c8/+0x1c9), thiscall, ret 0xc).
///
/// [pt] { 0x6f6c00 } (ret 0xc, match_pl.cpp): ver `chute_com`.
pub(crate) unsafe extern "thiscall" fn chute(j: usize, forca: u32, x: u32, y: u32) {
    chute_com(j, forca as i16, x as i8, y as i8, &mut |rotina, this, a| despachar_dividida(rotina, this, a))
}

pub(crate) unsafe extern "thiscall" fn comparar_chute(j: usize, forca: u32, x: u32, y: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(301);
    if gravando_no_topo() {
        return o(j, forca, x, y);
    }
    let m = *((j + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, forca, x, y), |chamar| {
        chute_com(j, forca as i16, x as i8, y as i8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x7103e0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_716.store(gravar, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("chute(forca {}, alvo {} {}) {} chamadas: {}", forca as i16, x as i8, y as i8, k, detalhe_fita()));
}

// ---------- centesima quadragesima segunda leva: o desfecho do chute (0x70c040) ----------

/// { 0x70c040 } (ret 4, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x6cd770 bola parada, 0x716e10 e 0x6f0630; e as da falta 0x6bac40): o desfecho do chute de
/// `p` com a forca `f` (0 vale 1). A qualidade (sorteio(1000) - sorteio(2000)) x (+0x8eaa + 1) / f
/// contra a defesa do goleiro (0x6ce110, 0x700420, com rebote e saida) da o tipo: 1 no alvo, 2..11
/// para fora (so o placar 0x6ce310), 12/13 para fora com tiro de meta narrado (0x1fca/0x1fcb, bola
/// parada 2). No alvo, a parte do chute na soma com a defesa: acima de 71 e gol (disputa de penaltis
/// 0x6bba50, gol de ouro 0x6d4250, o goleiro sofre; narracoes pelo placar e o tempo, 0x6bb950 no
/// lado; o placar +0xf5bd do outro lado, as narracoes da torcida), acima de 61 defesa com escanteio,
/// acima de 51 defesa que sai pela linha (+0x8ea8, bola parada 3 e 0x1f75), senao defesa (0x6f0630);
/// nas defesas, o destaque do goleiro (0x717470) escolhe a narracao.
pub(crate) unsafe fn desfecho_do_chute_com(p: usize, f_in: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + 0x19e) as *const usize);
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let bm = |o: usize| (m + o) as *mut u8;
    let lado = byte(p, 0x27);
    let outro = (lado == 0) as u8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, x: u32, y: u8, a7: i16| narrar_chute(chamar, m, codigo, x, y, a7);
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let somar = |q: usize, d: i16| {
        let w = (q + 0x35) as *mut i16;
        *w = (*w).wrapping_add(d);
    };
    let mais_2b = |q: usize, d: u8| {
        let b = (q + 0x2b) as *mut u8;
        *b = (*b).wrapping_add(d);
    };
    let f = if f_in as f64 == f64_em(0x9689a0) { 1.0f32 } else { f_in };
    let fator = byte(m, 0x8eaa);
    let r1 = sortear(0x7d0);
    let r2 = sortear(0x3e8);
    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(fator as i32 + 1) as f64 / f as f64);
    let g = manter_posicao_um(m, lado);
    if g == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, f32)>(316)(p, f_in);
    }
    let mut b18 = 0u8;
    if byte(m, 0x8ea9) > 3 {
        b18 = 1;
    }
    let (mut b13, mut b16) = (0u8, 0u8);
    let defesa = if g != p {
        crate::comparacao::ramo("desfecho goleiro defende");
        defesa_do_goleiro(g, byte(p, 0x102), byte(p, 0x103), b18, -1, &mut b16, &mut b13)
    } else {
        // ponytail: com o proprio goleiro chutando, a original le b13/b16 sem valor (zerados no
        // original determinizado)
        0
    };
    let (mut b12, mut b14) = (0u8, 0u8);
    if b13 == 0 {
        if defesa as i16 > 0x177 {
            b12 = 1;
        } else if defesa as i16 > 0x104 {
            b14 = 1;
        }
    }
    let q = qualidade as i16;
    let tipo: u8 = if q > 0xf0 {
        1
    } else if q < 10 {
        if b18 != 0 { if sortear(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let d = 0xf0 - q as i32;
        if d > 0x32 {
            if b18 != 0 { if sortear(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if d > 0xa {
            8 + (b18 != 0) as u8
        } else if d > 6 {
            0xa + (b18 != 0) as u8
        } else {
            0xc + (b18 != 0) as u8
        }
    };
    let (mut b10, mut b11, mut b15, mut b23, mut b17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    placar_agregado(m, outro as i8, &mut b10, &mut b11, &mut b15, &mut b23, &mut b17, 1);
    if tipo != 1 {
        match tipo {
            0xc => narrar(chamar, 0x1fca, byte(p, 0x2e) as u8 as u32, outro, 0),
            0xd => narrar(chamar, 0x1fcb, byte(p, 0x2e) as u8 as u32, outro, 0),
            _ => {}
        }
        if !(0xc..=0xd).contains(&tipo) {
            crate::comparacao::ramo("desfecho para fora");
            return;
        }
        crate::comparacao::ramo("desfecho tiro de meta");
        if b13 != 0 && b16 == 0 && tabela1(g) {
            narrar(chamar, 0x20d5, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, 0);
        }
        narrar(chamar, 0x211b, byte(p, 0x2e) as u8 as u32, lado as u8, 0);
        chamar(0x6cd770, m, [byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32, 2 | (lado as u8 as u32) << 8, 1, 0]);
        if tabela1(g) {
            mais_2b(g, sortear(3) as u8);
        }
        *bm(0x1cd) = 1;
        return;
    }
    // no alvo
    let k = defesa as i16 as i32;
    let parte = if k + q as i32 == 0 { 0 } else { (q as i32 * 100) / (k + q as i32) };
    let parte = parte as i16;
    if parte <= 0x47 {
        if b14 != 0 && tabela1(g) {
            crate::comparacao::ramo("desfecho defesa media");
            somar(g, 0x15e);
            if sortear(byte(g, 0x17) as i32 * 20 + 0x32) == 0 {
                let r = sortear(4);
                if ajustar_17(g, r as i8) != 0 {
                    desempenho_na_partida(g, 0);
                }
            }
        } else if b12 != 0 && tabela1(g) {
            crate::comparacao::ramo("desfecho defesa dificil");
            somar(g, 0x258);
            if sortear((byte(g, 0x17) as i32 + 1) * 20) == 0 {
                let r = sortear(7);
                if ajustar_17(g, r as i8) != 0 {
                    desempenho_na_partida(g, 0);
                }
            }
        }
    }
    if parte > 0x47 {
        desfecho_gol(m, p, g, lado, outro, b10, b11, b12, b13, b14, b15, b16, b17, chamar);
        return;
    }
    let codigos = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
    let destaque = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, abertura: u32, comum: u32, quieta: u32| {
        let (g2e, g27) = (byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8);
        if b12 != 0 {
            narrar(chamar, abertura, g2e, g27, 0);
            if let Some(&c) = codigos.iter().find(|&&c| destaque_do_jogador(g, c) != 0) {
                crate::comparacao::ramo("desfecho destaque do goleiro");
                narrar(chamar, c, g2e, g27, 0);
            }
        } else {
            narrar(chamar, if b14 != 0 { comum } else { quieta }, g2e, g27, 0);
        }
    };
    if parte > 0x3d {
        crate::comparacao::ramo("desfecho escanteio");
        destaque(chamar, 0x2111, 0x210f, 0x1fe6);
        somar(g, 0x8c);
        mais_2b(g, (sortear(3) + 1) as u8);
        chamar(0x6cd770, m, [byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32, (lado as u8 as u32) << 8, 0, 0]);
        narrar(chamar, 0x211b, byte(p, 0x2e) as u8 as u32, lado as u8, 0);
        *bm(0x1cd) = 1;
    } else if parte > 0x33 {
        crate::comparacao::ramo("desfecho linha de fundo");
        destaque(chamar, 0x2111, 0x210f, 0x1fe4);
        if lado == 1 {
            *bm(0x8ea8) = 0xff;
        } else if lado == 0 {
            *bm(0x8ea8) = 0xc;
        }
        *bm(0x8eae) = outro;
        *((m + 0xf582) as *mut u32) = 0;
        narrar(chamar, 0x211b, byte(p, 0x2e) as u8 as u32, lado as u8, 0);
        *bm(0x8eb2) = 3;
        narrar(chamar, 0x1f75, u32::MAX, outro, 0);
    } else {
        crate::comparacao::ramo("desfecho defesa");
        let r = sortear(3);
        somar(g, 0xc8);
        mais_2b(g, r as u8);
        destaque(chamar, 0x2112, 0x2110, 0x1fe5);
        narrar(chamar, 0x211b, byte(p, 0x2e) as u8 as u32, lado as u8, 0);
        chamar(0x6f0630, g, [0; 5]);
    }
}

/// O gol do desfecho (parte do chute acima de 71): ver `desfecho_do_chute_com`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn desfecho_gol(m: usize, p: usize, g: usize, lado: i8, outro: u8, mut b10: i8, b11: i8, b12: u8, b13: u8, b14: u8, b15: u8, b16: u8, b17: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let _ = (b12, b14);
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let bm = |o: usize| (m + o) as *mut u8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, x: u32, y: u8, a7: i16| narrar_chute(chamar, m, codigo, x, y, a7);
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let somar = |q: usize, d: i16| {
        let w = (q + 0x35) as *mut i16;
        *w = (*w).wrapping_add(d);
    };
    let minuto = || *((m + 0x8ed0) as *const i16) as i32 / 11;
    let tarde = |d: i32| (0x55..0x5a).contains(&d) || d >= 0x73;
    let (p2e, p27) = (byte(p, 0x2e) as u8 as u32, lado as u8);
    crate::comparacao::ramo("desfecho gol");
    if lado_do_lance(m) as i8 == -1 {
        if momento_marcado(m) as u8 != 0 {
            crate::comparacao::ramo("desfecho gol de ouro");
            *bm(0x1a8) = 1;
        }
        if tabela1(g) {
            *((g + 0xf) as *mut u8) = byte(g, 0xf).wrapping_add(1) as u8;
            let ficha = u(g, 0x69);
            if sortear(byte(ficha, 0x58) as i32 + 1) != 0 && sortear(byte(ficha, 0x5a) as i32 + 1) != 0 {
                let s = byte(g, 0xf) as u8 as i32;
                let r = sortear(s * s) as u8;
                let pf = byte(p, 0xf) as u8;
                let v = if ((r as i8) as i32) < pf as i32 { pf } else { r };
                *((g + 0x100) as *mut u8) = (byte(g, 0x100) as u8).wrapping_add(v);
                desempenho_na_partida(g, 0);
            }
            somar(g, -500);
            if b13 != 0 {
                somar(g, -600);
            }
        }
    }
    if tabela1(g) && g != p && b13 == 0 && sortear(0x32) == 0 && b16 == 0 {
        narrar(chamar, 0x20d6, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, 0);
    }
    narrar(chamar, 0x1fff, p2e, outro, 0);
    let penaltis = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, y: u8| {
        crate::comparacao::ramo("desfecho gol nos penaltis");
        narrar(chamar, 0x20ed, u32::MAX, y, 0);
        let bl = lado_do_lance(m);
        aplicar_falta_com(m, chamar);
        let k = if bl as i8 == lado {
            narrar(chamar, 0x20df, u32::MAX, p27, 0);
            if sortear((b11 as i32) - (b10 as i32) + 3) > 0 {
                narrar(chamar, 0x20d9, u32::MAX, p27, 0);
            }
            3.0f32
        } else {
            if (b10 == b11 || (b10 as i32) - 1 == b11 as i32) && sortear(0x28) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, p27, 0);
            }
            f32::from_bits(0x3fb9999a)
        };
        aplicar_ao_time(m, p, k);
        chamar(0x716e10, p, [0; 5]);
    };
    if sortear(3) != 0 && lado_do_lance(m) as i8 > -1 {
        penaltis(chamar, outro);
        return;
    }
    let nos_penaltis = lado_do_lance(m) as i8 > -1;
    narrar(chamar, if nos_penaltis { 0x20fa } else { 0x1fcd }, p2e, p27, 0);
    if lado_do_lance(m) as i8 > -1 {
        penaltis(chamar, p27);
        return;
    }
    if tabela1(g) && g != p && b16 != 0 {
        narrar(chamar, 0x1fe1, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, b16 as i8 as i16);
    }
    b10 = b10.wrapping_add(1);
    let k_lado = |k: u32| aplicar_ao_lado(m, lado, f32::from_bits(k));
    if *bm(0x1a8) != 0 {
        narrar(chamar, 0x1fd2, u32::MAX, outro, 0);
    } else if b17 != 0 {
        torcida(chamar, m, lado, outro, b15, &narrar);
    } else {
        let dif = b10 as i32 - b11 as i32;
        let t = minuto();
        let fase2 = *((m + 0x8ed4) as *const i16) == 0x3de;
        if dif == 2 && ((t >= 0x46 && fase2) || t >= 0x6a) {
            narrar(chamar, 0x1fd0, u32::MAX, outro, 0);
            k_lado(0x3fc66666);
        } else if (dif == 1 || b15 != 0) && ((t >= 0x50 && fase2) || t >= 0x6e) {
            narrar(chamar, 0x1fd1, u32::MAX, outro, 0);
            k_lado(0x3fb9999a);
        } else if b10 == b11 && b15 == 0 && t >= 0x46 {
            crate::comparacao::ramo("desfecho empate tardio");
            let a = contar_ativos(m, outro as i8) as i8;
            let b = contar_ativos(m, lado) as i8;
            let bl = if a < b { byte(m, 4 + outro as usize) } else { -1 };
            let d = minuto() as i16 as i32;
            let (codigo, k) = if bl == -1 {
                if tarde(d) { (0x1fd8, 0x3fb9999a) } else { (0x1fd6, 0x3faccccd) }
            } else if tarde(d) {
                (0x1fd9, 0x3fd33333)
            } else {
                (0x1fd7, 0x3fb33333)
            };
            k_lado(k);
            narrar(chamar, codigo, u32::MAX, outro, bl as i16);
        } else if dif < 0 && dif > -3 && (t >= 0x50 || (t >= 0x46 && b10 == 1) || (t >= 0x3c && b10 == 1 && 1 - b11 as i32 == -2)) {
            narrar(chamar, if tarde(t as i16 as i32) { 0x1fdb } else { 0x1fda }, u32::MAX, outro, 0);
        } else {
            torcida(chamar, m, lado, outro, b15, &narrar);
        }
    }
    if tabela1(g) && g != p && b13 != 0 && b16 == 0 {
        narrar(chamar, 0x20d5, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, 0);
    }
    aplicar_ao_time(m, p, f32::from_bits(0x3fd9999a));
    let gols = bm(0xf5bd + outro as usize * 0x36);
    *gols = (*gols).wrapping_add(1);
    let r = sortear(5);
    let clube = || {
        let pt = u(m, 0x4792);
        u(if lado != 0 { pt + 0x1c } else { pt + 0x20 }, 0)
    };
    let tecnico_8 = || {
        let t = u(clube(), 0x53);
        t != 0 && byte(t, 0x77) == 8
    };
    let mut feito = false;
    if (b10 as i32 - b11 as i32) > r + 2 && sortear(7) == 0 && tecnico_8() {
        narrar(chamar, 0x2117, u32::MAX, outro, 0);
        feito = true;
    }
    if !feito && (b10 == b11 || b10 as i32 - 1 == b11 as i32 || b15 != 0) {
        let r1 = sortear(minuto());
        if r1 > sortear(0x78) && tecnico_8() {
            narrar(chamar, 0x2116, u32::MAX, outro, 0);
        }
    }
    *bm(0x8eae) = lado as u8;
    *((m + 0xf582) as *mut u32) = 0;
    *bm(0x8eb2) = 8;
    *bm(0x1ca) = 0;
}

/// O fim comum das narracoes do gol: a torcida (0x1fdc antes dos 80 com +0x15, fator 1,45) ou
/// so o fator 1,25 no lado.
pub(crate) unsafe fn torcida(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, lado: i8, outro: u8, b15: u8, narrar: &dyn Fn(&mut dyn FnMut(usize, usize, [u32; 5]) -> u32, u32, u32, u8, i16)) {
    let t = *((m + 0x8ed0) as *const i16) as i32 / 11;
    if b15 != 0 && t < 0x50 {
        narrar(chamar, 0x1fdc, u32::MAX, outro, 0);
        aplicar_ao_lado(m, lado, f32::from_bits(0x3fb9999a));
    } else {
        aplicar_ao_lado(m, lado, f32::from_bits(0x3fa00000));
    }
}

/// A narracao dos chutes: o ultimo argumento (a7) vai em 16 bits no alto do codigo (contagens de gols
/// em palavra; a fita confere o byte baixo).
pub(crate) unsafe fn narrar_chute(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, codigo: u32, x: u32, y: u8, a7: i16) {
    chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo | (a7 as u16 as u32) << 16, m as u32, x, y as u32, u32::MAX]);
}

pub(crate) unsafe fn despachar_chute(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    if rotina == 0x6d6f70 {
        comentar(this, [a[0] & 0xffff, a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, (a[0] >> 16) as u16 as i16 as i32 as u32]);
        return 0;
    }
    despachar_dividida(rotina, this, a)
}

/// the same structure against the keeper of the shooter's own side (the goal goes to the other side, commentary and
/// side factor mirrored). Determinized original. Compared through the tape
/// Original: `shot_outcome` (0x70c040, match_pl.cpp (5.1 KB), thiscall, ret 4).
///
/// [pt] { 0x70c040 }: ver `desfecho_do_chute_com`.
pub(crate) unsafe extern "thiscall" fn desfecho_do_chute(p: usize, f: f32) {
    desfecho_do_chute_com(p, f, &mut |r, t, a| despachar_chute(r, t, a))
}

pub(crate) unsafe extern "thiscall" fn comparar_desfecho_do_chute(p: usize, f: f32) {
    if gravando() {
        // unidade da fita de fora; por dentro, a propria comparacao (fita aninhada)
        chamar_gravando(0x70c040, p, [f.to_bits(), 0, 0, 0, 0], || {
            fita_do_desfecho(p, f);
            0
        });
        return;
    }
    fita_do_desfecho(p, f);
}

pub(crate) unsafe fn fita_do_desfecho(p: usize, f: f32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, f32)>(316);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || {
        zerar_pilha();
        o(p, f)
    }, |chamar| {
        desfecho_do_chute_com(p, f, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("desfecho_do_chute {} chamadas: {}", k, detalhe_fita()));
}

/// { 0x70a910 } (ret 4, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (como na
/// 0x70c040): o chute a gol de `p` com a forca `f` (0 vale 1), contra o goleiro do outro lado
/// (0x6ce110, 0x700420). Mesmo tipo da 0x70c040; fora da disputa de penaltis conta o chute (+0xa) e o
/// chute no alvo (+0xb); para fora narra 0x1fc6..0x1fcb pelo tipo (8..13) e, com 12/13, o tiro de
/// meta (0x211c). Gol: +0xc, atuacao +0x89 e as cinco contagens de gols (+0x3d..+0x45, palavras); as
/// narracoes do placar, o hat-trick (0x2011/0x1fd5) e as marcas da carreira (0x200a..0x2012 e o
/// destaque 0x21b6, com a contagem como ultimo argumento); o placar +0xf5bd do lado. As defesas como
/// na 0x70c040 (0x211c no lugar de 0x211b).
pub(crate) unsafe fn chute_a_gol_com(p: usize, f_in: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + 0x19e) as *const usize);
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let bm = |o: usize| (m + o) as *mut u8;
    let lado = byte(p, 0x27);
    let outro = (lado == 0) as u8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, x: u32, y: u8, a7: i16| narrar_chute(chamar, m, codigo, x, y, a7);
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let somar = |q: usize, d: i16| {
        let w = (q + 0x35) as *mut i16;
        *w = (*w).wrapping_add(d);
    };
    let mais = |q: usize, o: usize| {
        let b = (q + o) as *mut u8;
        *b = (*b).wrapping_add(1);
    };
    let mais_2b = |q: usize, d: u8| {
        let b = (q + 0x2b) as *mut u8;
        *b = (*b).wrapping_add(d);
    };
    let (p2e, p27) = (byte(p, 0x2e) as u8 as u32, lado as u8);
    let f = if f_in as f64 == f64_em(0x9689a0) { 1.0f32 } else { f_in };
    let fator = byte(m, 0x8eaa);
    let r1 = sortear(0x7d0);
    let r2 = sortear(0x3e8);
    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(fator as i32 + 1) as f64 / f as f64);
    let g = manter_posicao_um(m, outro as i8);
    if g == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, f32)>(317)(p, f_in);
    }
    let b18 = (byte(m, 0x8ea9) > 3) as u8;
    let (mut b13, mut b15) = (0u8, 0u8);
    let defesa = defesa_do_goleiro(g, byte(p, 0x102), byte(p, 0x103), b18, -1, &mut b15, &mut b13);
    let (mut b12, mut b14) = (0u8, 0u8);
    if b13 == 0 {
        if defesa as i16 > 0x177 {
            b12 = 1;
        } else if defesa as i16 > 0x104 {
            b14 = 1;
        }
    }
    let q = qualidade as i16;
    let tipo: u8 = if q > 0xf0 {
        1
    } else if q < 10 {
        if b18 != 0 { if sortear(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let d = 0xf0 - q as i32;
        if d > 0x32 {
            if b18 != 0 { if sortear(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if d > 0xa {
            8 + (b18 != 0) as u8
        } else if d > 6 {
            0xa + (b18 != 0) as u8
        } else {
            0xc + (b18 != 0) as u8
        }
    };
    if lado_do_lance(m) as i8 == -1 {
        mais(p, 0xa);
        if tipo == 1 {
            mais(p, 0xb);
        }
    }
    let (mut b10, mut b11, mut b16, mut b23, mut b17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    placar_agregado(m, lado, &mut b10, &mut b11, &mut b16, &mut b23, &mut b17, 1);
    if tipo != 1 {
        if (8..=0xd).contains(&tipo) {
            narrar(chamar, 0x1fc6 + (tipo as u32 - 8), p2e, p27, 0);
        }
        if !(0xc..=0xd).contains(&tipo) {
            crate::comparacao::ramo("chute a gol para fora");
            return;
        }
        crate::comparacao::ramo("chute a gol tiro de meta");
        if b13 != 0 && b15 == 0 && tabela1(g) {
            narrar(chamar, 0x20d5, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, 0);
        }
        narrar(chamar, 0x211c, p2e, p27, 0);
        chamar(0x6cd770, m, [byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32, 2 | (p27 as u32) << 8, 1, 0]);
        if tabela1(g) {
            mais_2b(g, sortear(3) as u8);
        }
        *bm(0x1cd) = 1;
        return;
    }
    let k = defesa as i16 as i32;
    let parte = if k + q as i32 == 0 { 0 } else { (q as i32 * 100) / (k + q as i32) } as i16;
    if parte <= 0x47 {
        if b14 != 0 && tabela1(g) {
            crate::comparacao::ramo("chute a gol defesa media");
            somar(g, 0x15e);
            if sortear(byte(g, 0x17) as i32 * 20 + 0x32) == 0 {
                let r = sortear(4);
                if ajustar_17(g, r as i8) != 0 {
                    desempenho_na_partida(g, 0);
                }
            }
        } else if b12 != 0 && tabela1(g) {
            crate::comparacao::ramo("chute a gol defesa dificil");
            somar(g, 0x258);
            if sortear((byte(g, 0x17) as i32 + 1) * 20) == 0 {
                let r = sortear(7);
                if ajustar_17(g, r as i8) != 0 {
                    desempenho_na_partida(g, 0);
                }
            }
        }
        let codigos = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
        let destaque = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, abertura: u32, comum: u32, quieta: u32| {
            let (g2e, g27) = (byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8);
            if b12 != 0 {
                narrar(chamar, abertura, g2e, g27, 0);
                if let Some(&c) = codigos.iter().find(|&&c| destaque_do_jogador(g, c) != 0) {
                    crate::comparacao::ramo("chute a gol destaque do goleiro");
                    narrar(chamar, c, g2e, g27, 0);
                }
            } else {
                narrar(chamar, if b14 != 0 { comum } else { quieta }, g2e, g27, 0);
            }
        };
        if parte > 0x3d {
            crate::comparacao::ramo("chute a gol escanteio");
            destaque(chamar, 0x2111, 0x210f, 0x1fe6);
            narrar(chamar, 0x211c, p2e, p27, 0);
            somar(g, 0x8c);
            mais_2b(g, (sortear(3) + 1) as u8);
            chamar(0x6cd770, m, [byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32, (outro as u32) << 8, 0, 0]);
            *bm(0x1cd) = 1;
        } else if parte > 0x33 {
            crate::comparacao::ramo("chute a gol linha de fundo");
            destaque(chamar, 0x2111, 0x210f, 0x1fe4);
            if lado == 1 {
                *bm(0x8ea8) = 0xff;
            } else if lado == 0 {
                *bm(0x8ea8) = 0xc;
            }
            *bm(0x8eae) = p27;
            *((m + 0xf582) as *mut u32) = 0;
            narrar(chamar, 0x211c, p2e, p27, 0);
            *bm(0x8eb2) = 3;
            narrar(chamar, 0x1f75, u32::MAX, p27, 0);
        } else {
            crate::comparacao::ramo("chute a gol defesa");
            let r = sortear(3);
            somar(g, 0xc8);
            mais_2b(g, r as u8);
            destaque(chamar, 0x2112, 0x2110, 0x1fe5);
            narrar(chamar, 0x211c, p2e, p27, 0);
            chamar(0x6f0630, g, [0; 5]);
        }
        return;
    }
    // o gol
    crate::comparacao::ramo("chute a gol gol");
    let w = |o: usize| (p + o) as *mut i16;
    if lado_do_lance(m) as i8 == -1 {
        if momento_marcado(m) as u8 != 0 {
            crate::comparacao::ramo("chute a gol gol de ouro");
            *bm(0x1a8) = 1;
        }
        mais(p, 0xc);
        somar(p, 0x89);
        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
            *w(o) = (*w(o)).wrapping_add(1);
        }
        if tabela1(g) {
            mais(g, 0xf);
            let ficha = u(g, 0x69);
            if sortear(byte(ficha, 0x58) as i32 + 1) != 0 && sortear(byte(ficha, 0x5a) as i32 + 1) != 0 {
                let s = byte(g, 0xf) as u8 as i32;
                let r = sortear(s * s) as u8;
                let pf = byte(p, 0xf) as u8;
                let v = if ((r as i8) as i32) < pf as i32 { pf } else { r };
                *((g + 0x100) as *mut u8) = (byte(g, 0x100) as u8).wrapping_add(v);
                desempenho_na_partida(g, 0);
            }
            somar(g, -500);
            if b13 != 0 {
                somar(g, -600);
            }
        }
    }
    if b13 == 0 && sortear(0x32) == 0 && b15 == 0 && tabela1(g) {
        narrar(chamar, 0x20d6, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, 0);
    }
    narrar(chamar, 0x1fff, p2e, p27, 0);
    // a disputa de penaltis: a falta (0x6bac40) e as narracoes do cobrador
    let penaltis = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        crate::comparacao::ramo("chute a gol nos penaltis");
        narrar(chamar, 0x20ed, u32::MAX, p27, 0);
        let bl = lado_do_lance(m);
        aplicar_falta_com(m, chamar);
        let k = if bl as i8 == lado {
            narrar(chamar, 0x20df, u32::MAX, p27, 0);
            if sortear((b11 as i32) - (b10 as i32) + 3) > 0 {
                narrar(chamar, 0x20d9, u32::MAX, p27, 0);
            }
            3.0f32
        } else {
            if (b10 == b11 || (b10 as i32) - 1 == b11 as i32) && sortear(0x28) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, p27, 0);
            }
            f32::from_bits(0x3fb9999a)
        };
        aplicar_ao_time(m, p, k);
        chamar(0x716e10, p, [0; 5]);
    };
    if sortear(3) != 0 && lado_do_lance(m) as i8 > -1 {
        penaltis(chamar);
        return;
    }
    narrar(chamar, if lado_do_lance(m) as i8 > -1 { 0x2115 } else { 0x2114 }, p2e, p27, 0);
    if sortear(3) != 0 && lado_do_lance(m) as i8 > -1 {
        penaltis(chamar);
        return;
    }
    if byte(p, 0xc) == 3 {
        crate::comparacao::ramo("chute a gol hat-trick");
        narrar(chamar, if byte(p, 0x30) != 0 { 0x2011 } else { 0x1fd5 }, p2e, p27, 0);
    }
    if lado_do_lance(m) as i8 > -1 {
        penaltis(chamar);
        return;
    }
    // as marcas: o ultimo argumento e a contagem
    let c = |o: usize| *w(o);
    let redondo = |v: i16, d: i16| v >= d && v % d == 0;
    let marca: Option<(u32, i16)> = if byte(p, 0x30) != 0 && byte(p, 0xc) == 1 {
        Some((if destaque_do_jogador(p, 0x21b6) != 0 { 0x21b6 } else { 0x2010 }, c(0x3d)))
    } else if c(0x3d) == 1 {
        Some((0x200f, 1))
    } else if c(0x3f) == 1 {
        Some((0x200b, 1))
    } else if c(0x43) == 1 {
        Some((0x2012, 1))
    } else if redondo(c(0x3f), 10) {
        Some((0x200b, c(0x3f)))
    } else if c(0x3d) == 0x32 || redondo(c(0x3d), 100) {
        Some((0x200a, c(0x3d)))
    } else if c(0x41) != c(0x3d) && (c(0x41) == 0x32 || redondo(c(0x41), 100)) {
        Some((0x200c, c(0x41)))
    } else if c(0x43) != c(0x41) && redondo(c(0x43), 100) {
        Some((0x200d, c(0x43)))
    } else if c(0x45) == 1 || redondo(c(0x45), 5) {
        Some((0x200e, c(0x45)))
    } else {
        None
    };
    if let Some((codigo, n)) = marca {
        crate::comparacao::ramo("chute a gol marca");
        narrar(chamar, codigo, p2e, p27, n);
    }
    if b15 != 0 && tabela1(g) {
        narrar(chamar, 0x1fe1, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, b15 as i8 as i16);
    }
    b10 = b10.wrapping_add(1);
    let minuto = || *((m + 0x8ed0) as *const i16) as i32 / 11;
    let tarde = |d: i32| (0x55..0x5a).contains(&d) || d >= 0x73;
    let k_outro = |k: u32| aplicar_ao_lado(m, outro as i8, f32::from_bits(k));
    let torcida = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if b16 != 0 && minuto() < 0x50 {
            narrar(chamar, 0x1fdc, u32::MAX, p27, 0);
            k_outro(0x3fb9999a);
        } else {
            k_outro(0x3fa00000);
        }
    };
    if *bm(0x1a8) != 0 {
        narrar(chamar, 0x1fd2, u32::MAX, p27, 0);
    } else if b17 != 0 {
        torcida(chamar);
    } else {
        let dif = b10 as i32 - b11 as i32;
        let t = minuto();
        let fase2 = *((m + 0x8ed4) as *const i16) == 0x3de;
        if dif == 2 && ((t >= 0x46 && fase2) || t >= 0x6a) {
            narrar(chamar, 0x1fd0, u32::MAX, p27, 0);
            k_outro(0x3fc66666);
        } else if (dif == 1 || b16 != 0) && ((t >= 0x50 && fase2) || t >= 0x6e) {
            narrar(chamar, 0x1fd1, u32::MAX, p27, 0);
            k_outro(0x3fb9999a);
        } else if b10 == b11 && b16 == 0 && t >= 0x46 {
            crate::comparacao::ramo("chute a gol empate tardio");
            let a = contar_ativos(m, outro as i8) as i8;
            let b = contar_ativos(m, lado) as i8;
            let bl = if b < a { byte(m, (4 + lado as isize) as usize) } else { -1 };
            let d = minuto() as i16 as i32;
            let (codigo, k) = if bl == -1 {
                if tarde(d) { (0x1fd8, 0x3fb9999a) } else { (0x1fd6, 0x3faccccd) }
            } else if tarde(d) {
                (0x1fd9, 0x3fd33333)
            } else {
                (0x1fd7, 0x3fb33333)
            };
            k_outro(k);
            narrar(chamar, codigo, u32::MAX, p27, bl as i16);
        } else if dif < 0 && dif > -3 && (t >= 0x50 || (t >= 0x46 && b10 == 1) || (t >= 0x3c && b10 == 1 && 1 - b11 as i32 == -2)) {
            narrar(chamar, if tarde(t as i16 as i32) { 0x1fdb } else { 0x1fda }, u32::MAX, p27, 0);
        } else {
            torcida(chamar);
        }
    }
    if b13 != 0 && b15 == 0 && tabela1(g) {
        narrar(chamar, 0x20d5, byte(g, 0x2e) as u8 as u32, byte(g, 0x27) as u8, 0);
    }
    aplicar_ao_time(m, p, f32::from_bits(0x3dcccccd));
    let gols = bm((0xf5bd + lado as isize * 0x36) as usize);
    *gols = (*gols).wrapping_add(1);
    let r = sortear(5);
    let tecnico_8 = || {
        let pt = u(m, 0x4792);
        let t = u(u(pt, if lado == 0 { 0x1c } else { 0x20 }), 0x53);
        t != 0 && byte(t, 0x77) == 8
    };
    let mut feito = false;
    if (b10 as i32 - b11 as i32) > r + 2 && sortear(7) == 0 && tecnico_8() {
        narrar(chamar, 0x2117, u32::MAX, p27, 0);
        feito = true;
    }
    if !feito && (b10 == b11 || b10 as i32 - 1 == b11 as i32 || b16 != 0) {
        let r1 = sortear(minuto());
        if r1 > sortear(0x78) && tecnico_8() {
            narrar(chamar, 0x2116, u32::MAX, p27, 0);
        }
    }
    let _ = b23;
    *bm(0x8eae) = outro;
    *((m + 0xf582) as *mut u32) = 0;
    *bm(0x8eb2) = 8;
    *bm(0x1ca) = 0;
}

/// `p`'s shot against the other side's keeper (0x6ce110, save 0x700420). Quality from strength and draws gives the
/// type: wide (0x1fc6..0x1fcb, goal kick), on target; on target, the shot's share against the save decides goal (player
/// counts, hat-trick, career milestones 0x200a..0x2012, score and time commentary, penalty shoot-out
/// 0x6bba50/0x6bac40), corner, byline or save (keeper highlight 0x717470). Compared through the tape (commentary, set
/// piece, 0x716e10, 0x6f0630 and the foul's calls as units)
/// Original: `shot_on_goal` (0x70a910, match_pl.cpp (5.9 KB), thiscall, ret 4).
///
/// [pt] { 0x70a910 }: ver `chute_a_gol_com`.
pub(crate) unsafe extern "thiscall" fn chute_a_gol(p: usize, f: f32) {
    chute_a_gol_com(p, f, &mut |r, t, a| despachar_chute(r, t, a))
}

pub(crate) unsafe extern "thiscall" fn comparar_chute_a_gol(p: usize, f: f32) {
    if gravando() {
        // unidade da fita de fora; por dentro, a propria comparacao (fita aninhada)
        chamar_gravando(0x70a910, p, [f.to_bits(), 0, 0, 0, 0], || {
            fita_do_chute_a_gol(p, f);
            0
        });
        return;
    }
    fita_do_chute_a_gol(p, f);
}

// ---------- centesima quadragesima terceira leva: o movimento do jogador (0x6f09f0) ----------

/// Unidades da fita do movimento: a 0x68d0d0 recebe o evento 0x82 montado na pilha (confere o conteudo).
pub(crate) static GRAVAR_MOVIMENTO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(crate) fn gravando_movimento() -> bool {
    GRAVAR_MOVIMENTO.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// A area atacada pelo lado (colunas 2 a 6; linhas 10-11 para o lado 0, 0-1 para o lado 1).
pub(crate) fn na_area_atacada(lado: i8, x: i8, y: i8) -> bool {
    (2..=6).contains(&x) && if lado == 0 { (10..=11).contains(&y) } else { (0..=1).contains(&y) }
}

/// Os jogadores marcados por `p` (+0x101 deles em +0x1ae) que ainda podem ser escolhidos (0x6bce20)
/// na casa dele saem do lugar (0x714d40) se +0x107 x +0x104 nao passar de sorteio(900) + 50.
pub(crate) unsafe fn soltar_marcados(m: usize, p: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mut i = 0i8;
    while i < byte(p, 0x101) {
        let q = *((p + 0x1ae + i as usize * 4) as *const usize);
        if pode_escolher(m, q, byte(p, 0x102), byte(p, 0x103)) as u8 != 0 {
            let a = byte(p, 0x107) as i32;
            let b = byte(p, 0x104) as i32;
            if a * b <= sortear(900) + 0x32 {
                chamar(0x714d40, q, [0; 5]);
            }
        }
        i = i.wrapping_add(1);
    }
}

/// O passo de uma casa em direcao ao alvo.
pub(crate) fn passo(de: i8, para: i8) -> i8 {
    if de < para {
        de + 1
    } else if de > para {
        de - 1
    } else {
        de
    }
}

/// { 0x6f09f0 } (ret 0x14, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x714d40 os marcados saem, 0x6f0630 dominio, 0x716e10/0x716c90, 0x6cd770 bola parada,
/// 0x6cd490 bola fora, 0x68d0d0 o evento; e as da lesao, da falta e da saida de campo): o jogador `p`
/// anda uma casa (duas, se for rapido) para (x, y) com a acao `codigo` (0xf, 0x11, 0x12...). Ja no
/// alvo (fora de 0xf e 0x100) so solta os marcados. Sem a bola: lesao na dividida (0x6cfe10) e, sem
/// estar virado para o alvo (0x6fa140), so vira (0x6f4920, +0x198 = 0x6f). Com a bola: o ataque pelo
/// folego, +0x19c e a acao, a narracao da aproximacao (0x2156..0x2162), o desafiante (0x6c87e0) com a
/// falta (0x6cfc60/0x6cfcd0, como na dividida, com o evento 0x82) ou a disputa limpa; vencendo o
/// desafiante, o lance dele (0x70d430 no 1) e o fim como na dividida; passando, a posse, +0x19c,
/// as contas do lance e as narracoes da acao (0x1f48/0x1f49; 0x2009, 0x20d1..0x20d3, 0x1f4a.. no 0x11
/// e 0x12); perdendo o controle (+0x1d ou sorteio), 0x1f42. Depois anda na grade (+0x215e, contas e
/// vagas por lado), o mais adiantado (+0xf596/+0xf59a), solta os marcados, desgasta (0x6f56c0) pela
/// distancia, pelo papel e pelas marcas da posicao (0x5b5270), e a bola: com ela acompanha
/// (+0x8ea7/+0x8ea8, fora 0x6cd490); sem ela, na bola livre a domina (0x1f46/0x2113/0x1f60). Por
/// fim vira para um terco do caminho ate a bola.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn mover_com(p: usize, x: i8, y: i8, codigo: i16, conduz_in: u8, modo: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + 0x19e) as *const usize);
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let lado = byte(p, 0x27);
    let outro = (lado == 0) as u8;
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let somar = |q: usize, d: i16| *w(q, 0x35) = (*w(q, 0x35)).wrapping_add(d);
    let fixar_19c = |q: usize, v: i16| *w(q, 0x19c) = v.clamp(-200, 200);
    let mut conduz = conduz_in;
    let (ox, oy) = (byte(p, 0x102), byte(p, 0x103));
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(1);
    let mut perdeu = false;
    if x == ox && y == oy && codigo != 0xf && codigo != 0x100 {
        soltar_marcados(m, p, chamar);
        return;
    }
    let com_bola = || p == u(m, 0xf582);
    if !com_bola() {
        lesao_na_dividida_com(m, p, codigo, 0, -1, 1, chamar);
    }
    if !com_bola() && virado_para(p, x, y) == 0 && byte(p, 0x10c) == 0 && byte(p, 0x10a) == 0 && byte(p, 0x105) == 0 {
        crate::comparacao::ramo("movimento so vira");
        apontar(p, byte(p, 0x102), byte(p, 0x103), x, y);
        soltar_marcados(m, p, chamar);
        *w(p, 0x198) = 0x6f;
        return;
    }
    let mut nx = passo(ox, x);
    let mut ny = passo(oy, y);
    if !com_bola() {
        let c = byte(p, 0x104) as i32;
        if c * c > sortear(1000) {
            nx = passo(nx, x);
            ny = passo(ny, y);
        }
    }
    'movimento: {
        if !com_bola() {
            break 'movimento;
        }
        crate::comparacao::ramo("movimento com a bola");
        // o ataque
        let d = 10000 - ftol(f32_em(p + 0x4d) as f64) as i16 as i32;
        let folego = sortear(d.wrapping_mul(d) / 50000) as i16;
        if conduz != 0 {
            let v = ftol(f32_em(p + 0x99) as f64 + f64_em(0x968ea8)) as i16;
            fixar_19c(p, (*w(p, 0x19c)).wrapping_add(v));
        }
        let base = *w(p, match codigo {
            0xf => 0x138,
            0x12 => 0x13e,
            _ => 0x13c,
        }) as i32;
        let mut at = sortear(base - folego as i32 + *w(p, 0x19c) as i32);
        match byte(m, 1) {
            1 | 2 => at += -1 - sortear(3),
            8 => at += -2 - sortear(5),
            0x20 => at += -3 - sortear(8),
            _ => {}
        }
        let c = byte(m, 0x8ea9) as i32;
        at -= sortear(c * c * c * 50);
        at += sortear(*w(p, 0x29) as i32 / 10 - 0x1e);
        let (mut a, mut b2, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
        let e = desafio_adversario(m, p, lado, codigo as i8, &mut a, &mut b2, &mut nota, &mut lance, &mut arriscado, 0);
        if e != 0 {
            lesao_na_dividida_com(m, p, codigo, e, lance as i16, 0, chamar);
        }
        // a narracao da aproximacao
        let acao = *w(p, 0x198);
        if !(0xf..=0x12).contains(&acao) && !tabela1(p) && sortear(0x14) == 0 {
            let valor = || {
                let t = if lado == 0 {
                    let i = ((byte(p, 0x103) as i32 - 0xb).abs() as i8 as i32) * 9 + (byte(p, 0x102) as i32 - 4).abs() as i8 as i32;
                    f32_em(0xa15e44 + i as usize * 4)
                } else {
                    distancia_ao_ponto(p, 4, 0)
                };
                ftol(t as f64 * f32_em(0x96d754) as f64 + f64_em(0x968e18)) as i8
            };
            let v = valor();
            let n = if (v as i32 / 5) * 5 < 1 { 1i8 } else { (((valor() as i32 / 5) as u8).wrapping_mul(5)) as i8 };
            let (px, py) = (byte(p, 0x102), byte(p, 0x103));
            let canto_proprio = (lado == 0 && px < 2) || (lado == 1 && px > 6);
            let canto_outro = (lado == 0 && px > 6) || (lado == 1 && px < 2);
            let fundo = (lado == 0 && py > 7) || (lado == 1 && py < 4);
            let meio = (lado == 0 && (4..=7).contains(&py)) || (lado == 1 && (4..=7).contains(&py));
            let codigo_n: Option<u32> = if fundo {
                if canto_proprio {
                    Some(0x2160)
                } else if canto_outro {
                    Some(0x2162)
                } else if na_area_atacada(lado, px, py) {
                    None
                } else {
                    Some(0x2161)
                }
            } else if !meio && (lado == 0 || lado == 1) {
                // atras (lado 0: < 4; lado 1: > 7)
                if canto_proprio {
                    Some(0x215a)
                } else if canto_outro {
                    Some(0x215c)
                } else {
                    Some(0x215b)
                }
            } else if (lado == 0 && px < 2) || (lado == 1 && px > 6) {
                Some(0x215d)
            } else if canto_outro {
                Some(0x215f)
            } else if (px as i32 - 4).abs() < 2 && (py == 5 || py == 6) && sortear(2) != 0 {
                Some(0x215e)
            } else {
                None
            };
            if let Some(cn) = codigo_n {
                crate::comparacao::ramo("movimento aproximacao narrada");
                narrar(chamar, cn, ub(p, 0x2e), ub(p, 0x27), u32::MAX, n);
            }
        }
        let at16 = at as i16;
        let mut penalti = false;
        let mut vantagem = 0u8;
        if e != 0 {
            if !tabela1(e) {
                narrar(chamar, 0x1f4d, ub(p, 0x2e), ub(p, 0x27), par(e), 0);
            }
            let mut tipo = 0u8;
            if !(nota < at16 && sortear(5) != 0) {
                let (mut cartao, mut grav) = (0u8, 0i8);
                tipo = falta_no_motor(m, e, p, lance, -1, &mut cartao, &mut grav, &mut vantagem);
                if byte(m, 0x8ebb) != 0
                    && na_area_atacada(lado, byte(e, 0x102), byte(e, 0x103))
                    && (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
                    && sortear(3) != 0
                {
                    narrar(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), u32::MAX, 0);
                }
                if tipo != 0 {
                    if sortear(0x1e) != 0 || arriscado != 0 {
                        crate::comparacao::ramo("movimento falta punida");
                        punir_no_movimento_com(m, e, p, tipo, cartao, grav, arriscado, vantagem, lance, chamar);
                        return;
                    }
                    crate::comparacao::ramo("movimento falta so registra");
                    registrar_lance(m, lance as i16 as u16, e as u32, p as u32, 0, tipo, *b(p, 0x102), *b(p, 0x103), cartao, grav as u8);
                }
            }
            if tipo == 0 && vantagem != 0 {
                penalti = na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) || arriscado != 0;
            }
            // a disputa limpa
            *w(e, 0x198) = lance as i16;
            *b(e, 4) = (*b(e, 4)).wrapping_add(1);
            let t = tabela_do_jogador(e);
            somar(e, if t & 0x30 != 0 && tabela_do_jogador(e) & 0x880 != 0 { -0x3b } else { -0x46 });
        }
        if byte(p, 0x1d) != 0 {
            perdeu = true;
        } else if modo != 4 && at16 < 1 {
            let r = sortear(ftol(f32_em(p + 0xe5) as f64 * f32_em(0x968990) as f64 + f32_em(0x96d750) as f64));
            if r == 0 && (e == 0 || nota < at16) {
                perdeu = true;
            }
        }
        if perdeu {
            crate::comparacao::ramo("movimento perde a bola");
            *((m + 0x4786) as *mut u32) = 0;
            *b(m, 0x8eae) = 0xff;
            *((m + 0xf582) as *mut u32) = 0;
            *b(m, 0x1a7) = 0;
            *w(p, 0x29) = 0;
            *w(p, 0x19c) = 0;
            let r = sortear(2);
            somar(p, -0x96);
            *b(p, 0x2b) = (r as u8).wrapping_add(1);
            narrar(chamar, 0x1f42, ub(p, 0x2e), ub(p, 0x27), u32::MAX, 0);
            if sortear((byte(u(p, 0x69), 0x5a) as i32 + 1) * 0x14) == 0 {
                let r = sortear(3);
                if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                    desempenho_na_partida(p, 0);
                }
            }
            break 'movimento;
        }
        if e != 0 && at16 <= nota {
            crate::comparacao::ramo("movimento desafiante vence");
            desafiante_vence_com(m, p, e, lance, nota, at16, vantagem, penalti, a, conduz, chamar);
            return;
        }
        // `p` passa
        crate::comparacao::ramo("movimento passa");
        if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) && sortear(*w(p, 0x31) as i32) < -10 {
            crate::comparacao::ramo("movimento cresce");
            let v = f32_em(p + 0x1a2) as f64 * f32_em(0x96d3b8) as f64;
            *((p + 0x1a2) as *mut f32) = v as f32;
            let minimo = ((25 - byte(u(p, 0x69), 0x5d) as i32) * 2) as f32;
            let v = if v > minimo as f64 { v } else { minimo as f64 };
            *((p + 0x1a2) as *mut f32) = v as f32;
            if v > f64_em(0x9688a8) {
                *((p + 0x1a2) as *mut f32) = 100.0;
            }
            narrar(chamar, 0x211d, u32::MAX, outro as u32, u32::MAX, 0);
        }
        *b(m, 0x8eab) = lado as u8;
        if lado != -1 {
            let q = (m as isize + lado as isize + 0x8d46) as *mut u8;
            *q = (*q).wrapping_add(1);
            let o2 = b(m, 0x8d46 + outro as usize);
            if (*o2 as i8) > 0 {
                *o2 = (*o2).wrapping_sub(10);
            }
        }
        let q = (m as isize + lado as isize + 0x8d46) as *mut u8;
        *q = (*q).wrapping_add(5);
        let limite_nota = (nota as i32 + 0x32).max(0xfa);
        let v = (at16 as i32 - limite_nota) as f64 + (f32_em(p + 0xe5) as f64 * f32_em(p + 0x95) as f64);
        let delta = if (f32_em(0x968970) as f64) < v { 0 } else { ftol(v) as i16 };
        fixar_19c(p, (*w(p, 0x19c)).wrapping_add(delta));
        let casa = (m as isize + outro as isize + 0x21b6 + ((byte(p, 0x103) as isize) + byte(p, 0x102) as isize * 0xc) * 0x5a) as *const u8;
        if *casa == 0 {
            fixar_19c(p, (*w(p, 0x19c)).wrapping_add(0x32));
        }
        let soma = |k: u32| {
            *((m + 0x4782) as *mut u32) = (*((m + 0x4782) as *const u32)).wrapping_add(k);
            *((m + 0x4786) as *mut u32) = (*((m + 0x4786) as *const u32)).wrapping_add(k);
        };
        if e == 0 {
            somar(p, 9);
            soma(0x24);
        } else if codigo == 0xf {
            *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(b2);
            somar(p, 0x4b);
            *b(p, 0x11) = (*b(p, 0x11)).wrapping_add(1);
            soma(300);
        } else {
            let k = if (b2 as i8) > 2 { b2 } else { 2 };
            *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(k);
            if tabela1(e) {
                *b(m, 0x1a4) = 1;
                soma(200);
            } else {
                soma(400);
            }
            somar(p, 100);
            *b(p, 0x11) = (*b(p, 0x11)).wrapping_add(1);
            if sortear((byte(p, 0x17) as i32 + 1) * 0x14) == 0 {
                let r = sortear(5);
                if ajustar_17(p, r as i8) != 0 {
                    desempenho_na_partida(p, 0);
                }
            }
        }
        *b(m, 0x8ea9) = 0;
        narrar_a_acao_com(m, p, e, codigo, nx, ny, conduz, modo, chamar);
        let _ = &mut conduz;
    }
    andar_com(m, p, ox, oy, nx, ny, perdeu, chamar);
}

/// As narracoes da acao de quem passa com a bola (0xf: 0x1f48/0x1f49; 0x11 e 0x12: pela area, pelo
/// meio, pelas pontas, pelo marcado e pelo goleiro).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn narrar_a_acao_com(m: usize, p: usize, e: usize, codigo: i16, x: i8, y: i8, conduz_in: u8, modo: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let lado = byte(p, 0x27);
    let outro = (lado == 0) as u8;
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, ub(p, 0x2e), lado as u8 as u32, a5]);
    };
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let (px, py) = (byte(p, 0x102), byte(p, 0x103));
    let mut conduz = conduz_in;
    let defesa_400 = || *((m + 0x9766 + outro as usize * 0x18e3) as *const u32) & 0x400 != 0;
    // a narracao com o desafiante (0x2173 pelo +0x71, 0x1f4a; com o goleiro 0x1f55/0x2158)
    let com_desafiante = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        let cod = if !tabela1(e) {
            let v = f32_em(p + 0x71);
            let forte = v > f32_em(0x96b020) || (v as f64 - f32_em(e + 0x71) as f64) > f32_em(0x968a78) as f64;
            if forte && sortear(ftol(f32_em(p + 0x95) as f64)) < sortear(0x1e) { 0x2173 } else { 0x1f4a }
        } else if na_area_atacada(lado, px, py) {
            0x1f55
        } else {
            0x2158
        };
        narrar(chamar, cod, par(e));
    };
    // a arvore sem desafiante, a partir do alvo
    let sem_desafiante = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if na_area_atacada(lado, x, y) && !na_area_atacada(lado, px, py) {
            narrar(chamar, 0x1f50, u32::MAX);
            return;
        }
        if fora_do_meio(x) != 0 && ((lado == 0 && y == 0xb && py < 0xb) || (lado == 1 && y == 0 && py > 0)) {
            narrar(chamar, 0x2156, u32::MAX);
            return;
        }
        if (lado == 0 && y == 6 && py < 6) || (lado == 1 && y == 5 && py > 5) {
            narrar(chamar, 0x1f51, u32::MAX);
            return;
        }
        if (lado == 0 && py < 6 && py > y) || (lado == 1 && py >= 6 && y > py) {
            narrar(chamar, 0x1f54, u32::MAX);
            return;
        }
        if ((lado == 1 && py < 3) || (lado == 0 && py > 8)) && (px < 2 || px > 6) && fora_do_meio(x) == 0 {
            narrar(chamar, 0x1f4e, u32::MAX);
            return;
        }
        if fora_do_meio(x) != 0 && ((lado == 0 && x < px && x < 2) || (lado == 1 && px < x && x > 6)) {
            narrar(chamar, 0x1f52, u32::MAX);
            return;
        }
        if fora_do_meio(x) != 0 && ((lado == 0 && px < x && x > 6) || (lado == 1 && x < px && x < 2)) {
            narrar(chamar, 0x1f53, u32::MAX);
            return;
        }
        let q = u(p, 0x1ae);
        if q != 0 && byte(q, 0x102) == px && byte(q, 0x103) == py {
            let c = byte(q, 0x10a) as i32;
            if sortear(3) < c && byte(q, 0x2b) < 2 {
                crate::comparacao::ramo("movimento narra o marcado");
                narrar(chamar, 0x2172, ub(q, 0x2e) | (outro as u32) << 8);
                return;
            }
        }
        let acao = *((p + 0x198) as *const i16);
        if acao == 0x12 || acao == 0x11 {
            narrar(chamar, 0x1f4c, u32::MAX);
        } else if na_area_atacada(lado, px, py) {
            narrar(chamar, 0x20d1, u32::MAX);
        } else if limite(outro, py) != 0 {
            narrar(chamar, 0x20d3, u32::MAX);
        } else {
            narrar(chamar, 0x20d2, u32::MAX);
        }
    };
    match codigo {
        0xf => {
            if e == 0 {
                narrar(chamar, 0x1f48, u32::MAX);
            } else {
                narrar(chamar, 0x1f49, par(e));
            }
        }
        0x11 => {
            if na_area_atacada(lado, px, py) || (lado == 1 && py > 5) || (lado == 0 && py < 6) {
                conduz = 0;
            }
            if modo == 4 {
                crate::comparacao::ramo("movimento 0x11 modo 4");
                narrar(chamar, if *((p + 0x198) as *const i16) != 0xf { 0x2009 } else { 0x20d2 }, u32::MAX);
            } else if e == 0 {
                if conduz == 0 || !(menor_distancia(m, outro as i8, px, py, 1) as f64 > f64_em(0x968940)) {
                    sem_desafiante(chamar);
                } else {
                    narrar(chamar, if defesa_400() { 0x20de } else { 0x1f45 }, u32::MAX);
                }
            } else {
                com_desafiante(chamar);
                if conduz != 0 && menor_distancia(m, outro as i8, px, py, 1) as f64 > f64_em(0x968940) {
                    narrar(chamar, if defesa_400() { 0x20de } else { 0x1f45 }, u32::MAX);
                }
            }
        }
        0x12 => {
            if na_area_atacada(lado, px, py) || modo == 4 {
                conduz = 0;
            }
            if e != 0 {
                com_desafiante(chamar);
                if conduz != 0
                    && ((lado == 1 && py < 3) || (lado == 0 && py > 8))
                    && menor_distancia(m, outro as i8, px, py, 1) as f64 > f64_em(0x968940)
                {
                    if defesa_400() && sortear(2) == 0 {
                        narrar(chamar, 0x20de, u32::MAX);
                    } else {
                        narrar(chamar, 0x1f45, u32::MAX);
                    }
                }
            } else {
                sem_desafiante(chamar);
            }
        }
        _ => {
            // a original mostra a caixa de erro ("unknown action") e zera 0xb67a34
            *(0xb67a34 as *mut u32) = 0;
        }
    }
}

/// O desafiante `e` vence a disputa (como na dividida, com outras narracoes e o desgaste de `p`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn desafiante_vence_com(m: usize, p: usize, e: usize, lance: i8, nota: i16, at: i16, vantagem: u8, penalti: bool, a: u8, conduz: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let zero = |q: usize| (byte(q, 0x27) == 0) as u32;
    let lado = byte(p, 0x27);
    let narrou = (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
        && na_area_atacada((byte(e, 0x27) == 0) as i8, byte(e, 0x102), byte(e, 0x103))
        && sortear(2) == 0;
    if narrou {
        crate::comparacao::ramo("movimento narra falta na area");
        narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
        let codigo = match sortear(0x32) {
            1 => 0x20ea,
            0 | 2 | 4 | 6 | 8 | 10 => 0x20eb,
            _ => 0x20e8,
        };
        narrar(chamar, codigo, u32::MAX, zero(e), u32::MAX);
    } else {
        let codigo = match lance {
            1 => {
                crate::comparacao::ramo("movimento defesa");
                if defender_com(e, nota.wrapping_sub(at), &mut |rotina, this, x| {
                    if rotina == 0x6d6f70 {
                        chamar(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                    } else {
                        chamar(rotina, this, x)
                    }
                }) == 0
                {
                    return;
                }
                0
            }
            0x13 | 0x21 | 0x2d => 0x1f62,
            0x22 | 0x2c => 0x2150,
            0x29 => 0x1f61,
            0x2a | 0x2b => 0x1f64,
            _ => 0,
        };
        if codigo != 0 {
            narrar(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), par(p));
        }
    }
    chamar(0x6f0630, e, [0; 5]);
    // `p` perde: mais na propria area
    if na_area_atacada((lado == 0) as i8, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("movimento perde na area");
        *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(0xfa);
        if sortear(byte(u(p, 0x69), 0x5a) as i32 * 0x1e + 0x14) == 0 {
            let r = sortear(3);
            if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(p, 0);
            }
        }
    } else if (lado == 0 && byte(p, 0x103) < 3) || (lado == 1 && byte(p, 0x103) > 8) {
        *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(100);
    } else {
        *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(0x14);
    }
    *w(e, 0x19c) = nota.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200);
    let mut dominou = false;
    let limite = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
        crate::comparacao::ramo("movimento bola espirra");
        chamar(0x6cd770, m, [ub(p, 0x102), ub(p, 0x103), 1 | zero(p) << 8, 0, 0]);
        if byte(m, 0x8eb2) == 0 {
            let mut k = 3;
            k -= (byte(e, 0x10d) as i32 > sortear(10)) as i32;
            k -= (byte(e, 0x10c) as i32 > sortear(10)) as i32;
            *b(e, 0x2b) = sortear(k) as u8;
            dominou = true;
        } else if penalti {
            chamar(0x716c90, p, [0; 5]);
        }
    } else {
        if (0..4).contains(&byte(m, 0x8ea9)) {
            *b(m, 0x8ea9) = 0;
        }
        let (x, y) = (byte(p, 0x102), byte(p, 0x103));
        if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
            apontar(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            apontar(e, x, y, 4, if y < 4 { 0 } else { y - 3 });
        }
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let media = if byte(e, contas.0) != 0 { (ub(e, contas.1) as f64 / ub(e, contas.0) as f64) as f32 } else { 0.0 };
        if minuto > (sortear(0x14) + sorteio) as u8 as i8 {
            let gols = byte(m, if byte(e, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
            let tentativas = if lance == 0x15 { ub(e, 8) > 3 && byte(e, 9) != 0 } else { ub(e, 6) > 1 };
            if gols == 0 && media as f64 > f64_em(0x96a6a0) && tentativas && *w(e, 0x35) > 0x1a5e && byte(e, 0xfd) == 0 {
                crate::comparacao::ramo("movimento media de acertos");
                narrar(chamar, if lance == 0x15 { 0x21ce } else { 0x21cd }, ub(e, 0x2e), ub(e, 0x27), par(p));
                *b(e, 0xfd) = 1;
            }
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let mm = u(e, 0x19e);
    let t = |_: ()| if mm != 0 { tabela_do_jogador(e) } else { 0 };
    let longe = t(()) & 4 == 0 && t(()) & 2 == 0;
    let mut ganho: i16 = if dominou { 0x78 } else { 0xc8 };
    if longe {
        ganho <<= 1;
    }
    let py = byte(p, 0x103);
    let na_linha = match lado {
        0 | 1 => py < 3 || py > 8,
        _ => false,
    };
    if !na_linha {
        ganho = ftol(ganho as f64 * f64_em(0x969520)) as i16;
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(ganho);
    let alvo = if byte(e, 0x27) != 0 { 0xb } else { 0 };
    let i = ((byte(e, 0x103) as i32 - alvo).abs() as i8 as i32) * 9 + (byte(e, 0x102) as i32 - 4).abs() as i8 as i32;
    let perto = !(f32_em(0xa15e44 + i as usize * 4) as f64 > f64_em(0x967890));
    if conduz != 0 || perto || atras_do_extremo(mm, e) != 0 {
        crate::comparacao::ramo("movimento desafiante bonus");
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(300);
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
        if sortear(byte(e, 0x17) as i32 * 0x1e + 0x32) == 0 {
            let r = sortear(5);
            if ajustar_17(e, r as i8) != 0 {
                desempenho_na_partida(e, 0);
            }
        }
    }
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(a);
    if dominou && sortear(7) == 0 {
        narrar(chamar, 0x1f41, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
    }
}

/// A falta de `e` sobre `p` punida no movimento (como `punir_desafiante_com`, com o evento 0x82 e a
/// marca da bola parada no fim).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn punir_no_movimento_com(m: usize, e: usize, p: usize, mut tipo: u8, cartao: u8, grav: i8, arriscado: i8, vantagem: u8, lance: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let zero = |q: usize| (byte(q, 0x27) == 0) as u32;
    let lado = byte(p, 0x27);
    let mut peso = 0f32;
    motor_punir_lance(m, e, p, &mut tipo, cartao as i8, grav, arriscado as u8, &mut peso, vantagem);
    narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
    let cor: i32 = match tipo {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    let som: u32;
    if tipo == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("movimento penalti");
        if cor as i16 == -1 {
            match sortear(4) {
                0 => narrar(chamar, 0x20e9, u32::MAX, ub(p, 0x27), u32::MAX),
                1 => narrar(chamar, 0x20ec, u32::MAX, ub(p, 0x27), u32::MAX),
                _ => {}
            }
        }
        som = 0x1f7a;
        *b(m, 0x8eb2) = 5;
        *((m + 0x478a) as *mut usize) = p;
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(500);
        if sortear(byte(u(e, 0x69), 0x5a) as i32 + 6) == 0 {
            let r = sortear(8);
            if ajustar_17(e, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(e, 0);
            }
        }
        narrar(chamar, 0x1f7a, u32::MAX, ub(p, 0x27), u32::MAX);
    } else {
        som = 0x1f79;
        *b(m, 0x8eb2) = 6;
    }
    let expulso = matches!(tipo, 3..=6);
    if cor as i16 > -1 {
        // o evento 0x82 (cartao 4/5 pelos motivos 5, 10, 11 e o historico do jogador)
        let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        let v = ftol(media(u(0xb74350, 0), u(e, 0x6d), 0xb, 2, 6.5)) as i8;
        let base = *(0xae2364 as *const i32);
        let lista = if m != 0 { u(m, 0x478e) } else { 0 };
        if lista != 0
            && (tipo == 5 || tipo == 4)
            && (cartao == 0xa || cartao == 5 || cartao == 0xb)
            && u(e, 0x69) != 0
            && *(u(e, 0x69) as *const i32) < base
            && (u(p, 0x69) == 0 || *(u(p, 0x69) as *const i32) < base)
            && u(e, 0xf9) != 0
            && humano(u(e, 0xf9), 0, 1, 0, 0) == 0
            && (byte(u(e, 0x69), 0x5b) as i32) < v as i32 + 10
        {
            let pessoa = u(e, 0x6d);
            let k = 0xfu8.wrapping_sub(v as u8);
            if byte(pessoa, 0x1c) > (sortear(5) as u8).wrapping_add(k) as i8
                && byte(pessoa, 0x25) > (sortear(5) as u8).wrapping_add(k) as i8
                && *((pessoa + 0xb) as *const i16) > (sortear(0xdac) + 3000) as i16
                && v as i32 > sortear(3) + 1
            {
                crate::comparacao::ramo("movimento evento 0x82");
                let mut ev = [0u8; 0x19];
                ev[0..4].copy_from_slice(&*(u(e, 0x69) as *const [u8; 4]));
                ev[4..8].copy_from_slice(&*(u(e, 0xf9) as *const [u8; 4]));
                ev[0xd] = lado as u8;
                ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
                ev[0x13..0x15].copy_from_slice(&0x82u16.to_le_bytes());
                ev[0x15..0x19].copy_from_slice(&*(u(p, 0x69) as *const [u8; 4]));
                pedir_vetores(&[(ev.as_ptr() as usize, 0x19)], &[]);
                chamar(0x68d0d0, lista, [0, 0x19, 0, 0, 0]);
            }
        }
        if (lance == 0x2c || lance == 0x2f) && vantagem != 0 {
            narrar(chamar, if lance == 0x2f { 0x21da } else { 0x21d9 }, ub(p, 0x2e), ub(p, 0x27), par(e));
            if expulso || som == 0x1f7a {
                *b(e, 0x16) = 1;
            }
        }
        if sortear(4) == 0 {
            let x = sortear(grav as i32);
            if x > sortear(0x14) {
                let t = u(m, 0x4792);
                let clube = u(t, if byte(e, 0x27) == 0 { 0x20 } else { 0x1c });
                let k = u(clube, 0x53);
                if k != 0 && byte(k, 0x77) == 8 {
                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrar(chamar, 0x2109, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        }
        narrar(chamar, cor as u32, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        if expulso {
            crate::comparacao::ramo("movimento expulsao");
            jogador_saiu_com(m, e, 0, chamar);
        }
    }
    if som != 0x1f7a {
        let c = escolher_cobrador(m, lado, byte(m, 0x8eb2), 0);
        if c == 0 {
            narrar(chamar, som, 0xff, ub(p, 0x27), u32::MAX);
        } else {
            if tipo == 7 {
                narrar(chamar, 0x21e2, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
            } else {
                narrar(chamar, som, 0xff, ub(p, 0x27), u32::MAX);
            }
            let longe = (lado == 1 && byte(p, 0x103) < 4) || (lado == 0 && byte(p, 0x103) > 7);
            if som == 0x1f79 && vantagem == 0 && longe {
                if tipo == 7 {
                    if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
                        narrar(chamar, 0x21e1, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                    }
                } else if (2..=6).contains(&byte(p, 0x102)) {
                    narrar(chamar, 0x21e0, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                } else {
                    narrar(chamar, 0x21df, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                }
            }
        }
    }
    if codigo_especial(lance) == 0 {
        let extra = expulso || som == 0x1f7a;
        if vantagem != 0 {
            narrar(chamar, 0x20dd, ub(p, 0x2e), ub(p, 0x27), u32::MAX);
            if tipo == 4 || tipo == 5 {
                *b(e, 0x16) = 1;
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        } else if sortear(9) == 0 {
            narrar(chamar, 0x20df, u32::MAX, zero(p), u32::MAX);
            if sortear(3) == 0 {
                narrar(chamar, 0x20e0, u32::MAX, zero(p), u32::MAX);
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        }
        aplicar_ao_time(m, e, peso);
        chamar(0x716e10, e, [0; 5]);
    }
    *b(m, 0x8ea7) = *b(p, 0x102);
    *b(m, 0x8ea8) = *b(p, 0x103);
    *b(m, 0x8eae) = *b(p, 0x27);
    *((m + 0xf582) as *mut u32) = 0;
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// O passo na grade e o que vem depois: vagas da casa (+0x215e, 0x5a bytes por casa, 11 vagas por
/// lado e a conta em +0x58), o mais adiantado, os marcados, o desgaste, a bola e a direcao.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn andar_com(m: usize, p: usize, ox: i8, oy: i8, nx: i8, ny: i8, perdeu: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, u32::MAX]);
    };
    let lado = byte(p, 0x27);
    let casa = |x: i8, y: i8| (m as isize + 0x215e + (y as isize + x as isize * 0xc) * 0x5a) as usize;
    let vaga = |c: usize, k: i8| (c as isize + (lado as isize * 0xb + k as isize) * 4) as *mut usize;
    let conta = |c: usize| (c as isize + lado as isize + 0x58) as *mut i8;
    if byte(p, 0x102) != nx || byte(p, 0x103) != ny {
        let nova = casa(nx, ny);
        if byte(p, 0x28) < 0 {
            *b(p, 0x28) = *conta(nova) as u8;
            *conta(nova) = (*conta(nova)).wrapping_add(1);
            *vaga(nova, byte(p, 0x28)) = p;
        } else {
            let velha = casa(byte(p, 0x102), byte(p, 0x103));
            *vaga(velha, byte(p, 0x28)) = *vaga(velha, (*conta(velha)).wrapping_sub(1));
            *conta(velha) = (*conta(velha)).wrapping_sub(1);
            let q = *vaga(velha, byte(p, 0x28));
            if q != 0 {
                *b(q, 0x28) = *b(p, 0x28);
            }
            *b(p, 0x28) = *conta(nova) as u8;
            *conta(nova) = (*conta(nova)).wrapping_add(1);
            *vaga(nova, byte(p, 0x28)) = p;
        }
    }
    *b(p, 0x102) = nx as u8;
    *b(p, 0x103) = ny as u8;
    if lado == 0 {
        let q = u(m, 0xf596);
        let qy = if q == 0 { -1 } else { byte(q, 0x103) };
        if ny > qy {
            *((m + 0xf596) as *mut usize) = p;
        }
    } else if lado == 1 {
        let q = u(m, 0xf59a);
        let qy = if q == 0 { 9 } else { byte(q, 0x103) };
        if ny < qy {
            *((m + 0xf59a) as *mut usize) = p;
        }
    }
    soltar_marcados(m, p, chamar);
    if byte(p, 0x19) >= 0 {
        let mut k: f32 = match byte(m, 1) {
            1 => 1.4,
            2 => 1.2,
            4 | 0x10 => 1.05,
            8 => 1.45,
            0x20 => 1.15,
            0x40 => 1.1,
            _ => 1.0,
        };
        match byte(m, 0x4756) {
            3 => k = (k as f64 * f32_em(0x96d3b8) as f64) as f32,
            4 => k = (k as f64 * f32_em(0x96a33c) as f64) as f32,
            _ => {}
        }
        let dy = (byte(p, 0x103) as i32 - oy as i32).abs() as i8 as i32;
        let dx = (byte(p, 0x102) as i32 - ox as i32).abs() as i8 as i32;
        let t = f32_em(0xa15e44 + (dy * 9 + dx) as usize * 4) as f64;
        let d5 = f32_em(p + 0xd5) as f64;
        let f = (f64_em(0x9688a0) - (d5 * f64_em(0x968968) + f64_em(0x968e08)) * f64_em(0x96a7a8)) + f64_em(0x968950);
        k = (t * f * k as f64) as f32;
        let papel_b = if m != 0 { tabela_b(m, p) } else { 0 };
        let papel = if m != 0 { tabela_do_jogador(p) } else { 0 };
        if papel_b != papel {
            let classe = |t: u16| -> i8 {
                if t & 1 != 0 {
                    0
                } else if t & 4 != 0 {
                    1
                } else if t & 8 != 0 {
                    2
                } else if t & 0x10 != 0 {
                    3
                } else if t & 0x20 != 0 {
                    4
                } else if t & 0x40 != 0 {
                    5
                } else {
                    3
                }
            };
            let a = classe(if m != 0 { tabela_do_jogador(p) } else { 0 });
            let c = classe(if m != 0 { tabela_b(m, p) } else { 0 });
            let kk = if (a as i32 - c as i32).abs() as u8 as i8 > 1 {
                0x968928
            } else if (if m != 0 { tabela_do_jogador(p) } else { 0 }) & 4 == 0 && (if m != 0 { tabela_b(m, p) } else { 0 }) & 4 == 0 {
                0x968978
            } else {
                0x968980
            };
            k = ((f64_em(kk) - d5 * f64_em(0x968ef0)) * k as f64) as f32;
        }
        let tatica = m + 0x91e2 + lado as usize * 0x18e3;
        let marcas = marcas_da_posicao(tatica, byte(p, 0x19));
        if marcas & 0x8000 != 0 || (marcas_da_posicao(tatica, byte(p, 0x19)) & 0x2000 != 0 && *((m + lado as usize * 0x18e3 + 0x9766) as *const u32) & 0x100 != 0) {
            k = ((f64_em(0x96d748) - d5 * f64_em(0x967878)) * k as f64) as f32;
        }
        desgastar(p, k, -1);
    }
    if p == u(m, 0xf582) || perdeu {
        *b(m, 0x8ea7) = *b(p, 0x102);
        *b(m, 0x8ea8) = *b(p, 0x103);
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        if !(bx >= 0 && by >= 0 && by < 0xc && bx < 9) {
            crate::comparacao::ramo("movimento bola fora");
            chamar(0x6cd490, m, [lado as u8 as u32, 0, 0, 0, 0]);
        }
    } else if byte(m, 0x8ea9) < 5
        && byte(m, 0x1c5) == 0
        && byte(m, 0x8eae) == -1
        && byte(m, 0x8ea7) == byte(p, 0x102)
        && byte(m, 0x8ea8) == byte(p, 0x103)
        && p != u(m, 0xf57a)
        && p != u(m, 0xf576)
    {
        let q = u(m, 0xf57a);
        if q != 0 {
            if byte(q, 0x27) != lado {
                crate::comparacao::ramo("movimento rouba a bola livre");
                narrar(chamar, 0x1f60, ub(p, 0x2e), lado as u8 as u32);
                chamar(0x6f0630, p, [0; 5]);
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x1c);
            }
        } else {
            crate::comparacao::ramo("movimento domina a bola livre");
            apontar(p, byte(p, 0x102), byte(p, 0x103), 4, if lado != 0 { 0 } else { 0xb });
            let (px, py) = (byte(p, 0x102), byte(p, 0x103));
            let gol = byte(m, 0x1cd) != 0 && (3..=5).contains(&px) && ((lado == 0 && py >= 10) || (lado == 1 && py <= 1));
            narrar(chamar, if gol { 0x2113 } else { 0x1f46 }, ub(p, 0x2e), lado as u8 as u32);
            *b(m, 0x8ea9) = 0;
            chamar(0x6f0630, p, [0; 5]);
            *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(4);
        }
    }
    // vira para um terco do caminho ate a bola
    if p == u(m, 0xf582) {
        return;
    }
    let bola_y = byte(m, 0x8ea8) as i32;
    let q = u(m, 0xf57a);
    let dono = byte(m, 0x8eae);
    let perto: bool = if lado == 0 {
        if dono == 1 {
            true
        } else if p == q {
            false
        } else if dono != 0 && (q == 0 || byte(q, 0x27) != 0) {
            byte(p, 0x103) < 6
        } else {
            false
        }
    } else if dono == 0 {
        false
    } else {
        !(p != q && dono != lado && (q == 0 || byte(q, 0x27) != lado) && lado == 1 && byte(p, 0x103) >= 6)
    };
    let dy = if perto { bola_y + ny as i32 } else { bola_y + ny as i32 + 0xb };
    let dx = byte(m, 0x8ea7) as i32 + 4 + nx as i32;
    apontar(p, byte(p, 0x102), byte(p, 0x103), (dx / 3) as i8, (dy / 3) as i8);
}

/// Despacho real das chamadas injetadas do movimento: as da dividida, o marcado que sai (0x714d40)
/// e o evento 0x82 (0x68d0d0, o bloco pedido por ultimo).
pub(crate) unsafe fn despachar_movimento(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE);
            f(this);
            0
        }
        0x68d0d0 => despachar_lesao(rotina, this, a),
        _ => despachar_dividida(rotina, this, a),
    }
}

/// the player moves one cell (two, if fast) towards (x, y) with the given action. Without the ball, the tackle injury
/// and, if not facing the target, only turns. With the ball: the attack from stamina and action, the approach
/// commentary, the challenger (foul with event 0x82, clean contest), possession and the action commentary; then the
/// grid, the furthest forward, the marked players released, wear from the position's marks and the ball (follows it or
/// takes the loose one). Compared through the tape (commentary, 0x714d40, control, set piece, ball out, the event and
/// the injury and foul calls as units)
/// Original: `player_move` (0x6f09f0, match_pl.cpp (9 KB), thiscall, ret 0x14).
///
/// [pt] { 0x6f09f0 }: ver `mover_com`.
pub(crate) unsafe extern "thiscall" fn mover_jogador(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    mover_com(p, a1 as u8 as i8, a2 as u8 as i8, a3 as u16 as i16, a4 as u8, a5 as u8, &mut |r, t, a| despachar_movimento(r, t, a));
    0
}

pub(crate) unsafe extern "thiscall" fn comparar_mover(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    if gravando() {
        // unidade da fita de fora; por dentro, a propria comparacao (fita aninhada)
        return chamar_gravando(0x6f09f0, p, [a1, a2, a3, a4, a5], || fita_do_movimento(p, a1, a2, a3, a4, a5));
    }
    fita_do_movimento(p, a1, a2, a3, a4, a5)
}

pub(crate) unsafe fn fita_do_movimento(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(318);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let mov = GRAVAR_MOVIMENTO.swap(true, Relaxed);
    let (r, _, igual, sequencia, k) = comparar_com_fita(reg, || o(p, a1, a2, a3, a4, a5), |chamar| {
        mover_com(p, a1 as u8 as i8, a2 as u8 as i8, a3 as u16 as i16, a4 as u8, a5 as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x6cd490 => [0xff, 0, 0, 0, 0],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        });
        0
    });
    GRAVAR_MOVIMENTO.store(mov, Relaxed);
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("mover(acao {:#x}) {} chamadas: {}", a3 as u16, k, detalhe_fita()));
    r
}

pub(crate) unsafe fn fita_do_chute_a_gol(p: usize, f: f32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, f32)>(317);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(p, f), |chamar| {
        chute_a_gol_com(p, f, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("chute_a_gol {} chamadas: {}", k, detalhe_fita()));
}

// ---------- centesima quadragesima quarta leva: o jogador com a bola (0x7109f0) ----------

/// Unidades da fita da conducao: a copia de 0x68d0d0 vai para a lista na pilha (endereco diferente
/// na copia Rust: grava sem o `this`) e a 0x702050 (o chute) entra como unidade.
pub(crate) static GRAVAR_CONDUCAO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Na metade de ataque do lado (lado 1: linhas abaixo de 6; lado 0: de 6 em diante; outro lado, sim).
pub(crate) fn na_metade_de_ataque(lado: i8, y: i8) -> bool {
    !((lado == 1 && y >= 6) || (lado == 0 && y < 6))
}

/// Na metade de defesa do lado (o contrario da de ataque, so para os lados 0 e 1).
pub(crate) fn na_metade_de_defesa(lado: i8, y: i8) -> bool {
    (lado == 1 && y >= 6) || (lado == 0 && y < 6)
}

/// Na propria area (a que o lado defende; lado 1: linhas 10-11, os outros 0-1).
pub(crate) fn na_area_defendida(lado: i8, x: i8, y: i8) -> bool {
    na_area_atacada((lado != 1) as i8, x, y)
}

/// { 0x7109f0 } (match_pl.cpp, ~13 KB) com as chamadas que mexem fora do motor injetadas (0x6fa250
/// o passe, 0x6fa490 o lancamento, 0x70ec80 a dividida, 0x6f09f0 o movimento, 0x702050 o chute,
/// 0x6d6f70 narra, 0x68d0d0 a copia na lista da pressao): a decisao do jogador `p` com a bola.
/// Bola parada (+0x8eb2): 2 e 3 (escanteios, 0x36/0x34), 4 (lateral, 0x5a), 6 e 7 (faltas, 0x37) passam
/// para o companheiro escolhido (0x6c6a30) ou para uma casa sorteada; 5 (penalti, 0x33) e 6 perto do
/// gol (0x35) chutam (0x700bd0 + 0x702050). Em jogo, soma as notas do chute (+0x10), do passe
/// (+0x14) e da conducao (+0x44) pela distancia ao gol, a pressao (0x6bcbd0), o placar, o goleiro,
/// as marcas da posicao e o relogio; pode lancar (0x6fa490), dividir (0x70ec80), chutar (0x702050),
/// passar (0x6fa250) ou conduzir (0x6f09f0) para a casa escolhida (0x6bce80). Modo 4: o goleiro na
/// disputa de penaltis. Devolve 1 so quando conduz.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn conduzir_com(p: usize, modo: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let fl = |o: usize| f32_em(p + o) as f64;
    let mut volta = 0u8;
    let m = u(p, 0x19e);
    let lado = byte(p, 0x27);
    let x0 = || byte(p, 0x102);
    let y0 = || byte(p, 0x103);
    let f: f32 = if lado == 0 { tabela_12x9(y0() as i32 - 0xb, x0() as i32 - 4) } else { distancia_ao_ponto(p, 4, 0) };
    let ff = f as f64;
    let (mut l14, mut l44) = (0i32, 0i32);
    let mut l10: i32;
    let mut l90 = 0f32;
    let mut l70 = 0i16;
    let mut l88: i8 = -1;
    let mut l64 = 0u16;
    let (mut l5f, mut l37) = (false, false);
    let mut la4 = 0i32;
    // a lista da pressao (0x68d020: contador e cabeca/cauda); so a copia do modo Rust a enche
    let mut lista = [0u8; 14];
    let la = lista.as_mut_ptr() as usize;
    let passar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, tipo: u32, r: usize, moveu: u8, x: i8, y: i8| {
        *w(p, 0x198) = tipo as i16;
        chamar(0x6fa250, p, [tipo, r as u32, moveu as u32, x as u8 as u32, y as u8 as u32]);
    };
    let chutar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, tipo: u32, v: i32, som: u16, a4: u32, com_lista: bool| {
        chamar(0x702050, p, [tipo, v as u32, som as u32, a4, if com_lista { la as u32 } else { 0 }]);
    };
    'fim: {
        let c = byte(m, 0x8eb2);
        let (mut tipo, mut moveu, mut x, mut y, mut melhor) = (0i8, 0u8, 0i8, 0i8, 0i32);
        let escolher = |pedido: i8, tipo: &mut i8, moveu: &mut u8, x: &mut i8, y: &mut i8, melhor: &mut i32| {
            escolher_companheiro(m, byte(m, 0x8ea7), byte(m, 0x8ea8), tipo, moveu, x, y, melhor, pedido)
        };
        match c {
            2 => {
                *b(m, 0x8eb2) = 0;
                let r = escolher(0x36, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 {
                    crate::comparacao::ramo("conducao escanteio sem companheiro");
                    let d: i8 = if lado != 1 { 1 } else { -1 };
                    y = byte(m, 0x8ea8).wrapping_add(d);
                    if y < 0 {
                        y = 0;
                    } else if y > 0xb {
                        y = 0xb;
                    }
                    x = if byte(m, 0x8ea7) == 0 { sortear(4) as i8 } else { 8 - sortear(4) as i8 };
                }
                passar(chamar, 0x36, r, moveu, x, y);
                break 'fim;
            }
            3 => {
                *b(m, 0x8eb2) = 0;
                let r = escolher(0x34, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 {
                    x = 5 - sortear(3) as i8;
                    if lado == 1 {
                        y = 0;
                    } else if lado == 0 {
                        y = 0xb;
                    }
                }
                passar(chamar, 0x34, r, moveu, x, y);
                break 'fim;
            }
            4 => {
                *b(m, 0x8eb2) = 0;
                let mut r = escolher(0x5a, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 || melhor < 2000 {
                    crate::comparacao::ramo("conducao lateral sorteada");
                    x = 7 - sortear(7) as i8;
                    y = if lado == 0 { sortear(3) as i8 + 6 } else { 5 - sortear(3) as i8 };
                    r = 0;
                }
                passar(chamar, 0x5a, r, moveu, x, y);
                break 'fim;
            }
            5 => {
                *b(m, 0x8eb2) = 0;
                crate::comparacao::ramo("conducao penalti");
                let mut t: i8 = 0x33;
                let v = valor_do_chute(p, &mut t, &mut l64) as i16 as i32;
                let a4 = if sortear(100) == 0 { 5 } else { 1 };
                chutar(chamar, 0x33, v, l64, a4, false);
                break 'fim;
            }
            6 | 7 => {
                *b(m, 0x8eb2) = 0;
                if c == 6 && (2..=6).contains(&x0()) && ff < f64_em(0x96d3e8) {
                    let bl = byte(p, 0x109) as i32;
                    let n = (sortear(ftol(fl(0xa1) + f32_em(0x96afb0) as f64)) + bl) / 3;
                    if (n as f64) > ff {
                        crate::comparacao::ramo("conducao falta direta");
                        *b(m, 0x8eb1) = 6;
                        let mut t: i8 = 0x35;
                        let v = valor_do_chute(p, &mut t, &mut l64) as i16 as i32;
                        let a4 = if sortear(100) == 0 { 6 } else { 1 };
                        chutar(chamar, 0x35, v, l64, a4, false);
                        break 'fim;
                    }
                }
                let mut r = escolher(0x37, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 || melhor < 2000 {
                    crate::comparacao::ramo("conducao falta na area");
                    x = 4;
                    y = if lado == 0 { byte(m, 0x8eaf).wrapping_sub(2) } else { byte(m, 0x8eb0).wrapping_add(2) };
                    r = 0;
                }
                passar(chamar, 0x37, r, moveu, x, y);
                break 'fim;
            }
            8 => *b(m, 0x8eb2) = 0,
            _ => {}
        }
        // em jogo: o alvo (casa de 0x6c2be0) e as notas
        let (l74, l36): (i8, i8);
        if modo == 4 {
            l74 = 4;
            l36 = if lado != 0 { 0 } else { 0xb };
        } else {
            let (mut sx, mut sy) = (0i8, 0i8);
            posicionar(m, p, &mut sx, &mut sy, 1);
            l74 = nota_ajustada(p, sx) as i8;
            l36 = sy;
            if (lado == 0 && y0() > l36) || (lado == 1 && y0() < l36) {
                l90 = tabela_12x9(y0() as i32 - l36 as i32, x0() as i32 - l74 as i32);
            }
        }
        let l53 = alem_da_linha(m, y0(), lado, 0);
        let mut l27: i8 = 0;
        let l68 = mais_perto_do_gol(m, p);
        let mut l60 = menor_distancia(m, (lado == 0) as i8, x0(), y0(), 1);
        let l2f = pressao_no_ponto(m, (lado == 0) as i8, x0(), y0(), 1);
        let mut l49: i8 = 0;
        let mut l4c = false;
        let g = manter_posicao_um(m, (lado == 0) as i8);
        let (mut casa, mut fora, mut cl, mut ida, mut f3) = (0i8, 0i8, 0u8, 0u8, 0u8);
        placar_agregado(m, lado, &mut casa, &mut fora, &mut cl, &mut ida, &mut f3, 0);
        let mut l30 = 0i32;
        let mut l58 = 0i32;
        let (mut l8c, mut l54, mut l94, mut l98) = (0i8, 0u8, 0i8, 0i8);
        if modo == 4 {
            *w(p, 0x29) = 0x15e;
        } else {
            if byte(m, 0x8ea9) == -1 {
                *w(p, 0x29) = 200;
            } else {
                l49 = pressao_com(m, p, &mut l70, la, &mut |l, q| {
                    chamar(0x68d0d0, l, [q as u32, 0x1be, 0, 0, 0]);
                }) as i8;
                if l49 == 0 {
                    if l60 as f64 > f64_em(0x968e38) {
                        l60 = 3.5;
                    }
                    *w(p, 0x29) = ftol(l60 as f64 * f32_em(0x96d7b0) as f64 + f32_em(0x96d844) as f64) as i16;
                } else {
                    if l70 < 0 {
                        l70 = 0;
                    }
                    let s = l70 as i32;
                    let k = (l49 as i32).wrapping_mul(s).wrapping_mul(s);
                    l58 = ftol(-(k as f64 * f64_em(0x969040)));
                    if l58 < -500 {
                        l58 = sortear((l58 + 500) / 3) - 500;
                        if l58 < -2000 {
                            l58 = -2000;
                        }
                    } else if l58 > 32000 {
                        l58 = 32000;
                    }
                    *w(p, 0x29) = ftol(l58 as f64 * f64_em(0x969040)) as i16;
                }
            }
            if na_metade_de_ataque(lado, y0()) {
                l27 = jogadores_no_trajeto(m, x0(), y0(), 4, if lado != 0 { 0 } else { 0xb }) as i8;
            }
            let soma = if na_area_atacada(lado, x0(), y0()) { 2 } else { 4 };
            let mut l28 = 0i32;
            l58 = escolher_companheiro(m, x0(), y0(), &mut l8c, &mut l54, &mut l94, &mut l98, &mut l28, -1) as i32;
            l30 = (l49 as i32 + l27 as i32 + soma).wrapping_mul(l28) / 5;
        }
        *w(p, 0x29) = (*w(p, 0x29)).wrapping_add(0x46);
        let flags = *((m + 0x9766 + (lado == 0) as usize * 0x18e3) as *const u32);
        if flags & 0x400 != 0 && byte(p, 0x106) as i32 > sortear(5) && !na_area_atacada(lado, x0(), y0()) && !na_area_defendida(lado, x0(), y0()) && na_metade_de_ataque(lado, y0()) {
            crate::comparacao::ramo("conducao instrucao de avancar");
            l14 = 4000 - l30;
            l44 = 2000 - l30 / 2;
        }
        // a nota do chute
        if byte(m, 0x8ea9) == 4 {
            if ff < f64_em(0x967890) || ((y0() == 0 || y0() == 0xb) && (x0() as i32 - 4).abs() == 1) {
                l10 = ftol(fl(0xa9) * f32_em(0x96d750) as f64 + f32_em(0x968dac) as f64 - (400 * l27 as i32) as f64);
            } else {
                l10 = -5000;
            }
        } else if ff <= f64_em(0x968940) {
            l10 = ftol(fl(0x99) * f32_em(0x968960) as f64 + f32_em(0x96d408) as f64 - (400 * l27 as i32) as f64 + *w(p, 0x29) as f64 * f64_em(0x968978));
        } else {
            let bl = byte(p, 0x109) as i32;
            let n = bl / 7 + sortear(3) + 3;
            if (n as f64) <= ff || (modo == 4 && bl <= sortear(0x32)) {
                l10 = -500;
            } else {
                crate::comparacao::ramo("conducao chute de longe");
                let v = valor_do_chute(p, &mut l88, &mut l64) as i16 as i32;
                let k = byte(p, 0x106) as i32 * 4;
                let r1 = sortear(ftol(ff * f32_em(0x96d750) as f64 - k as f64));
                let t2 = ftol(k as f64 - ff * f32_em(0x96d750) as f64);
                la4 = sortear(-(r1.wrapping_add(t2))).wrapping_add(v);
                l37 = true;
                if v > 0 {
                    let div = if byte(p, 0x109) < 1 { 1 } else { byte(p, 0x109) as i32 };
                    l10 = ftol((v * v) as f64 * f64_em(0x968a80) - ff * ff * f32_em(0x96d40c) as f64 / div as f64 - (700 * l27 as i32) as f64);
                } else {
                    l10 = ((byte(p, 0x10b) as i32 + byte(p, 0x106) as i32) * 4 + 2) * v;
                }
            }
        }
        // o placar
        let dif = fora as i32 - casa as i32;
        if byte(m, 0x1ab + lado as usize) == 5 {
            l10 = l10.wrapping_add(dif * 3000);
            l30 = l30.wrapping_mul(2);
            if !(m != 0 && tabela_a(m, p) == 1) && sortear((dif * 3 + byte(u(p, 0x69), 0x5b) as i32) * 2) < 1 {
                crate::comparacao::ramo("conducao mata-mata perde folego");
                *b(p, 0x100) = (*b(p, 0x100)).wrapping_sub(1);
            }
        } else if (fora as i32) < casa as i32 - 2 {
            l10 = l10.wrapping_add((dif + 2) * 3000);
            l30 = l30.wrapping_mul(2);
        }
        if byte(m, 0x8ea9) < 4 && ff > f32_em(0x968970) as f64 {
            if l68 == 2 {
                let pw = pow_do_jogo(ftol(ff) as f64, f64_em(0x968930));
                let q = fl(0x95);
                let r = sortear(4) + 1;
                let k = ftol((q * q + f64_em(0x967890)) * f64_em(0x968968)) as i16 as i32;
                l14 = l14.wrapping_add(ftol((r * k) as f64 * pw));
            }
            if tabela_12x9(y0() as i32 - byte(g, 0x103) as i32, x0() as i32 - byte(g, 0x102) as i32) as f64 == f64_em(0x9689a0)
                && byte(m, 0x21b6 + (y0() as isize + x0() as isize * 12) as usize * 0x5a + (lado == 0) as usize) == 1
            {
                crate::comparacao::ramo("conducao cara a cara");
                let r1 = sortear(ftol(fl(0x99) * f32_em(0x968a78) as f64));
                let r2 = sortear(ftol(fl(0x95) * f32_em(0x968a78) as f64));
                l14 = l14.wrapping_add(ftol((r2 - r1) as f64 * (ff * f32_em(0x969560) as f64 + f32_em(0x96d840) as f64)));
                l4c = true;
            }
        }
        // o fim de jogo: a dividida para ganhar tempo
        if l68 == 0 && l30 < 3000 && modo != 4 && (*w(m, 0x8ed0) as i32 / 11) > sortear(0x14) + 0x46 && !na_area_atacada(lado, x0(), y0()) && byte(p, 0x106) != 0 && byte(m, 0x8ea9) < 3 {
            let e = sortear(byte(u(p, 0x69), 0x5c) as i32);
            if e < sortear(10) && lado_sob_controle(m, lado) != 0 {
                let r = placar_agregado(m, lado, &mut casa, &mut fora, &mut cl, &mut ida, &mut f3, 1) as i8;
                if r > 0 && r < 3 {
                    if !(2..=6).contains(&x0()) && ((lado == 1 && y0() < 3) || (lado == 0 && y0() > 8)) {
                        crate::comparacao::ramo("conducao segura a bola");
                        chamar(0x70ec80, p, [1, 0, 0, 0, 0]);
                        *w(p, 0x198) = 0x10;
                        // ver o fim da 0x7139cb
                        break 'fim;
                    }
                    if na_metade_de_ataque(lado, y0()) && *w(p, 0x29) > -0x14 && l10 < 0 {
                        crate::comparacao::ramo("conducao segura o resultado");
                        l5f = true;
                        l14 += 4000;
                    }
                }
            }
        }
        if ff < f64_em(0x968930) && l10 > 0 {
            l44 = l44.wrapping_add(l10 / -5);
        }
        if m != 0 && tabela_a(m, p) == 1 {
            if byte(p, 0x10a) != 0 {
                l14 = l14.wrapping_add(ftol(fl(0x9d) * f32_em(0x96d210) as f64 - f32_em(0x968db0) as f64));
            }
            if na_metade_de_defesa(lado, y0()) {
                l10 = l10.wrapping_sub(25000);
            }
        }
        // o goleiro (ou quem guarda a posicao 1)
        if g != 0 && na_metade_de_ataque(lado, y0()) {
            let t = if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
            if t & 1 == 0 {
                l10 = l10.wrapping_add(5000);
            } else if byte(g, 0x2b) != 0 && byte(p, 0x10b) as i32 + 5 > sortear(ftol(ff * f32_em(0x96afb0) as f64)) {
                l10 = l10.wrapping_add(1500);
            }
            let (gx, gy) = (byte(g, 0x102), byte(g, 0x103));
            if !na_area_atacada(lado, gx, gy) {
                if byte(p, 0x10b) as i32 + 5 > sortear(ftol(ff)) {
                    l10 = l10.wrapping_sub(ftol(fl(0x95) * f32_em(0x968960) as f64 - f32_em(0x968dac) as f64));
                }
            } else if gx == 4 && gy == if byte(g, 0x27) == 1 { 0xb } else { 0 } {
                if alcanca_o_ponto(g, x0(), y0()) == 0 && byte(p, 0x10b) as i32 > sortear(ftol(ff * f32_em(0x968a78) as f64)) {
                    crate::comparacao::ramo("conducao goleiro no gol");
                    l10 = l10.wrapping_sub(ftol(fl(0x95) * f32_em(0x96d664) as f64 - f32_em(0x96d2dc) as f64));
                }
            } else {
                let e17 = byte(p, 0x17) as i32;
                let r = sortear(ftol(ff * ff * f32_em(0x969584) as f64));
                if byte(p, 0x10b) as i32 + e17 - 10 > r {
                    crate::comparacao::ramo("conducao goleiro adiantado");
                    l10 = l10.wrapping_sub(ftol(fl(0x95) * f32_em(0x96d664) as f64 - f32_em(0x96d3e4) as f64));
                }
                if x0() == gx && y0() == gy {
                    l14 = l14.wrapping_add(ftol((e17 as f64 + fl(0x95)) * f32_em(0x968960) as f64 - f32_em(0x96d408) as f64));
                }
            }
        }
        if byte(m, 0x8ea9) == 4 && ff > f64_em(0x967890) {
            l10 = l10.wrapping_add(ftol(ff * ff * f64_em(0x96d838)));
        } else if (4..=6).contains(&x0()) && ((lado == 1 && y0() == 0) || (lado == 0 && y0() == 0xb)) {
            l10 = l10.wrapping_add(5000 - (x0() as i32 - 4).abs() * 2000);
        }
        if l58 == 0 && l53 == 0 {
            l30 -= 2000;
        }
        // lancar
        if limite((lado == 0) as u8, y0()) != 0 && byte(m, 0x8ea9) != -1 && byte(p, 0x106) != 0 && l30 < 4000 - *w(m, 0x8ed0) as i32 && lado_sob_controle(m, lado) != 0 {
            crate::comparacao::ramo("conducao lancamento");
            chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 1, 0, 0, 0]);
            break 'fim;
        }
        let perto = (na_area_defendida(lado, x0(), y0()) && l60 as f64 <= f64_em(0x968940)) || (limite((lado == 0) as u8, y0()) != 0 && (l60 as f64) < f64_em(0x967890));
        if perto {
            let r1 = sortear(ftol(f32_em(0x96d408) as f64 - l60 as f64 * f32_em(0x96d2dc) as f64));
            if r1 > sortear(l30 + 2000) && byte(m, 0x8ea9) != -1 && modo != 4 {
                crate::comparacao::ramo("conducao alivio");
                chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                break 'fim;
            }
        }
        // a meta (o meio do gol atacado) e a boca do gol
        let ty: i8 = if lado == 0 { 0xb } else { 0 };
        let na_boca = || y0() == ty && (3..=5).contains(&x0());
        let consegue = || consegue_jogar(p, 4, ty, 0, 0.0, 0.0, core::ptr::null_mut()) != 0;
        if (diferenca_lado(m, lado as u8, y0() as u8) as i8) < 0 && (caminho_livre(m, p, 0) == 0 || l53 != 0 || tabela_a(m, p) == 1) {
            if na_area_atacada(lado, x0(), y0()) {
                l44 -= 1000;
            }
            if na_area_defendida(lado, x0(), y0()) {
                l44 -= 1000;
                l14 -= 1200;
            } else if na_metade_de_defesa(lado, y0()) && !na_boca() && !consegue() {
                l14 = l14.wrapping_add((-5 - l70 as i32) * 100);
            }
        } else {
            l44 -= 5000;
            l14 = l14.wrapping_add(ftol(ff * f32_em(0x96d3e4) as f64 - f32_em(0x96d3e4) as f64));
            if modo == 4 {
                l14 = l14.wrapping_add(ftol(ff * f32_em(0x96d404) as f64 - f32_em(0x96d3e4) as f64));
            }
            if na_metade_de_defesa(lado, y0()) {
                l10 -= 3000;
            }
            if na_area_atacada(lado, x0(), y0()) {
                l44 -= 1000;
                l10 += 1000;
            }
        }
        if tabela_a(m, p) == 1 || na_area_defendida(lado, x0(), y0()) {
            l44 = l44.wrapping_add(ftol(l60 as f64 * f32_em(0x96d1a8) as f64 - f32_em(0x96a588) as f64));
        } else {
            l14 = l14.wrapping_add(ftol((fl(0x95) - f32_em(0x96afb0) as f64 + l60 as f64 * f64_em(0x968eb0)) * f64_em(0x96afe8)));
        }
        if !na_boca() && !consegue() && na_metade_de_ataque(lado, y0()) {
            l14 = l14.wrapping_add(2000 - l30);
        }
        // a casa para onde conduzir (0x6bce80)
        let (mut sx, mut sy) = (l74, l36);
        let mut l43 = escolher_casa(m, p, &mut sx, &mut sy, (l68 != 0) as u32 + 2) as i8;
        let fundo = (lado == 1 && y0() < 3) || (lado == 0 && y0() > 8);
        let ta = |q: usize| if u(q, 0x19e) != 0 { tabela_a(u(q, 0x19e), q) } else { 0 };
        let tb = |q: usize| if u(q, 0x19e) != 0 { tabela_b(u(q, 0x19e), q) } else { 0 };
        if l43 != 0 {
            if modo == 4 {
                sx = sx.clamp(3, 5);
            }
            l44 -= 3000;
            if !(2..=6).contains(&x0()) && !fundo && !na_metade_de_defesa(lado, y0()) {
                l14 += 1250;
                l10 -= 500;
            } else if fundo && !na_area_atacada(lado, x0(), y0()) {
                l14 += 1000;
                l10 = l10.wrapping_add((byte(p, 0x109) as i32 - 20) * 100);
            }
        } else if (2..=6).contains(&x0()) || fundo {
            l14 -= 3000;
        } else {
            let k = (byte(p, 0x109) as f64 + fl(0x95)) * f32_em(0x968a78) as f64;
            if tabela_a(m, p) & 0x38 != 0 && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 && !((lado == 0 && y0() < 3) || (lado == 1 && y0() > 8)) {
                crate::comparacao::ramo("conducao ponta de lanca");
                l14 = l14.wrapping_add(ftol(k));
            } else if tabela_a(m, p) & 0x38 != 0 && tabela_a(m, p) & 0x880 != 0 {
                l14 = l14.wrapping_add(ftol(k - f32_em(0x96d210) as f64));
            } else {
                l14 -= 3000;
            }
            l10 -= 650;
        }
        // o chute de fora da area
        let na_area = na_area_atacada(lado, x0(), y0());
        if (2..=6).contains(&x0()) && !na_area && byte(m, 0x8ea9) != 4 {
            let t = tabela_a(m, p);
            let bl = byte(p, 0x109) as i32;
            let r = sortear(ftol((l27 as i32 + 1) as f64 * (ff + f32_em(0x968e70) as f64) * ff * ff * ff * f32_em(0x96d834) as f64));
            l10 = l10.wrapping_add(if t & 0x40 != 0 { bl * 500 } else { bl * 325 } - r);
        } else if na_area {
            l10 = l10.wrapping_add(ftol((byte(p, 0x109) as f64 + fl(0x99) - fl(0xe1)) * (f32_em(0x96afb0) as f64 - ff * f32_em(0x96d228) as f64)));
        } else if na_metade_de_ataque(lado, y0()) {
            l10 = l10.wrapping_sub(ftol(ff * f32_em(0x96d20c) as f64 - (byte(p, 0x109) as i32 * 75) as f64));
        }
        let marcas = |q: usize| marcas_da_posicao(m + 0x91e2 + byte(q, 0x27) as isize as usize * 0x18e3, byte(q, 0x19));
        'nota: {
            if sortear(ftol(fl(0xe1))) == 0 && !(tabela_a(m, p) & 0x40 == 0 && sortear(4) != 0) {
                crate::comparacao::ramo("conducao folego");
                if !(na_metade_de_defesa(lado, y0()) && (byte(p, 0x106) != 0 || byte(p, 0x10b) != 0)) {
                    let bl = na_area_defendida(lado, x0(), y0()) as i32;
                    let pw = pow_do_jogo(fl(0x95), f64_em(0x968930));
                    l14 = l14.wrapping_add(ftol(pw + (1000 - bl * 1000) as f64));
                }
                if ff >= f64_em(0x968950) && ff < f64_em(0x96afa0) && (2..=6).contains(&x0()) {
                    if byte(p, 0x109) != 0 {
                        let r = sortear(ftol(fl(0xb1)));
                        l10 = l10.wrapping_add(*w(p, 0x29) as i32 * 2 + r * 400 + 2500);
                    }
                } else if na_area {
                    l10 = l10.wrapping_add(ftol((f32_em(0x96afb0) as f64 - ff * f32_em(0x96d228) as f64) * (byte(p, 0x109) as f64 + fl(0x99))));
                }
                break 'nota;
            }
            if modo == 4 {
                break 'nota;
            }
            if marcas(p) & 0x2000000 != 0 && !na_area_defendida(lado, x0(), y0()) && l49 < 2 && ff >= f64_em(0x968950) {
                let segue = if l70 != 0 && na_metade_de_defesa(lado, y0()) { (sortear(byte(p, 0x106) as i32 * 2 + 20) as f64) < ff } else { true };
                if segue {
                    crate::comparacao::ramo("conducao armador");
                    let k = (byte(p, 0x11) as u8 as i32 * 30 + *w(p, 0x29) as i32 + 400) * 10 - l30;
                    l14 = l14.wrapping_add(ftol(k as f64 + byte(p, 0x109) as f64 * fl(0x95) * fl(0xe1)));
                    if !(2..=6).contains(&x0()) && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 && !fundo && marcas(p) & 0x4_0000_0000 != 0 {
                        l14 += 500;
                        l30 -= 250;
                    }
                }
            }
            if marcas(p) & 0x8000000 != 0 && na_metade_de_ataque(lado, y0()) && !na_boca() && !consegue() {
                l44 += 3500;
            }
            if marcas(p) & 0x20000000 != 0 && ff >= f64_em(0x968950) && ff < f64_em(0x96afa0) {
                let r = sortear(ftol(fl(0xb1)));
                l10 = l10.wrapping_add((*w(p, 0x29) as i32 + r * 200) * 2);
            }
        }
        // o lance de efeito e a pressao
        let c = byte(m, 0x8ea9);
        if c < 2 || (ff > f64_em(0x96d7a8) && c < 4) {
            let mut saida = 0u16;
            let r = classificar_lance(p, 0x16, x0(), y0(), 4, if lado == 1 { 0 } else { 0xb }, (ff + f64_em(0x967890)) as f32, &mut saida);
            if r != 2 && r != byte(p, 0x2c) as u8 && l68 == 0 {
                l10 = l10.wrapping_sub(sortear(byte(p, 0x2d) as i32 * 300));
            }
        }
        if ff < f64_em(0x9688a0) {
            let s = l70 as i32;
            l10 = l10.wrapping_sub(ftol(s.wrapping_mul(s).wrapping_mul(s) as f64 / (f64_em(0x9688b0) - ff)));
        }
        if byte(m, 0x8ea9) == 3 {
            l10 -= 2000;
        }
        let e = (l43 as i32 - l2f as i8 as i32) * 100;
        let t = ftol((f32_em(0x96d830) as f64 - fl(0x9d) * f32_em(0x96b020) as f64) * l90 as f64 * l90 as f64);
        l44 = l44.wrapping_add(e);
        l14 = l14.wrapping_add(e - t);
        let bl = byte(p, 0x109) as i32;
        if !(ff < f64_em(0x967890)) && bl * bl * bl * bl > sortear(ftol(ff * f32_em(0x96d1b0) as f64)) {
            if !na_area_defendida(lado, x0(), y0()) && ff >= f64_em(0x968950) && l49 < 2 && !(na_metade_de_defesa(lado, y0()) && (byte(p, 0x106) != 0 || byte(p, 0x10b) != 0)) {
                let pw = pow_do_jogo(fl(0x95), f64_em(0x968930));
                l14 = l14.wrapping_add(sortear(ftol(pw + (*w(p, 0x29) as i32 * 3) as f64 + (10000 - l30) as f64)));
            }
            if byte(m, 0x8ea9) < 4 && l27 < 2 && (ff < f64_em(0x9688a0) || (ff < f64_em(0x968988) && sortear(500) == 0)) {
                crate::comparacao::ramo("conducao chute colocado");
                let mut r = sortear(ftol(ff * f32_em(0x968a78) as f64)) as i8;
                if r < 10 {
                    r = 10;
                }
                l10 = l10.wrapping_add(sortear(400000 / r as i32));
            }
            l44 -= 10000;
        } else if !na_area_defendida(lado, x0(), y0()) {
            let e = sortear(ftol(byte(p, 0x106) as f64 * f64_em(0x968dd8)));
            if e > sortear(200) {
                // os adversarios na mesma casa
                let casa = m + 0x215e + (y0() as isize + x0() as isize * 12) as usize * 90;
                let outro = (lado == 0) as usize;
                let mut i = 0i8;
                while i < *((casa + outro + 0x58) as *const i8) {
                    let q = *((casa + outro * 44 + i as usize * 4) as *const usize);
                    if byte(q, 0x24) != 0 {
                        l14 = l14.wrapping_add(sortear(ftol((byte(p, 0x17) as i32 * 100) as f64 + fl(0x95) * f32_em(0x96d7c0) as f64)));
                    }
                    i += 1;
                }
            }
        }
        // o desconto da conducao (ebx do original)
        let k = ftol(pow_do_jogo(byte(m, 0x8ea9) as f64, f64_em(0x968930)) * f64_em(0x96d828));
        let mut nc = (-(*w(p, 0x29) as i32) - *w(p, 0x19c) as i32 * 10) * 10;
        nc = nc.wrapping_sub(ftol((f32_em(0x96d1b0) as f64 - fl(0x4d)) * f32_em(0x96d820) as f64)).wrapping_sub(k);
        if l58 != 0 && l30 > 0 && matches!(l8c as u8, 0x5e | 8 | 7 | 6) {
            nc = nc.wrapping_add(l30);
        } else if marcas(p) & 0x4_0000_0000 != 0 && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 {
            if y0() == 0 || y0() == 0xb {
                nc += 500;
            } else if (2..=6).contains(&x0()) {
                // nada
            } else if fundo {
                l14 += 2000;
            } else if ff < f64_em(0x9688a0) {
                l14 += 6000;
            }
        }
        if !l4c && l10 > 2000 && *w(p, 0x29) > -0x32 {
            nc = nc.wrapping_add(sortear(l10));
        }
        if byte(p, 0x106) == 0 {
            nc = sortear(nc);
        }
        if na_metade_de_defesa(lado, y0()) && nc < 0 {
            nc = 0;
        }
        l14 = l14.wrapping_sub(nc);
        // na linha de fundo, fora do meio: cruza (ou guarda a jogada ensaiada)
        if fora_do_meio(x0()) != 0 && !(m != 0 && byte(m, 0x8ea9) >= 3) && ((lado == 0 && y0() == 0xb) || (lado == 1 && y0() == 0)) {
            if l58 != 0 && matches!(l8c, 8 | 7 | 6) {
                l30 += 1500;
                if marcas(p) & 0x4_0000_0000 != 0 {
                    l30 += 1500;
                }
            } else if sortear(10) != 0 {
                crate::comparacao::ramo("conducao cruza da linha de fundo");
                chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                break 'fim;
            }
        }
        let k = (20 - byte(p, 0x106) as i32) * 50;
        l30 = l30.wrapping_add(sortear(k));
        l14 = l14.wrapping_add(sortear(k));
        let l44s = l44.wrapping_add(sortear(k));
        let mut nota = l10.wrapping_add(sortear(k));
        // a acao: 1 passa, 2 divide, 3 chuta, 4 conduz
        let mut acao = 3u8;
        if nota < l14 && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 && !(x0() == 4 && ((lado == 0 && y0() == 0xb) || (lado == 1 && y0() == 0))) {
            nota = l14;
            acao = 4;
        }
        if nota < l44s && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 && caminho_livre(m, p, 1) == 0 && !(*w(p, 0x198) == 0x10 && byte(m, 0x8d46 + lado as usize) >= 0xc) {
            nota = l44s;
            acao = 2;
        }
        'acao: {
            if *w(p, 0x198) != 0x10 && byte(m, 0x8ea9) == -1 {
                acao = 2;
                break 'acao;
            }
            if nota < l30 && modo != 4 {
                if l58 != 0 {
                    acao = 1;
                    break 'acao;
                }
                if (l53 != 0 && !fundo) || byte(m, 0x8ea9) == 4 {
                    crate::comparacao::ramo("conducao alivia sem companheiro");
                    chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                    break 'fim;
                }
                acao = if sortear(if *w(p, 0x198) != 0x10 { 2 } else { 10 }) != 0 { 4 } else { 2 };
            }
            if acao == 3 {
                if !na_boca() && !consegue() {
                    match byte(m, 0x8ea9) {
                        3 => {
                            if !(byte(p, 0x109) as i32 > sortear(0xf) && ff < f64_em(0x96d700)) {
                                acao = 2;
                                break 'acao;
                            }
                        }
                        4 => {
                            acao = 1;
                            break 'acao;
                        }
                        c => {
                            acao = if c == -1 { 1 } else { 4 };
                            break 'acao;
                        }
                    }
                }
                let c = byte(m, 0x8ea9);
                if (c == 4 && !(ff < f64_em(0x968950))) || c == -1 {
                    acao = 1;
                    break 'acao;
                }
                if !na_metade_de_defesa(lado, y0()) && ((2..=6).contains(&x0()) || byte(p, 0x109) as i32 >= sortear(0x78)) {
                    break 'acao;
                }
                if modo == 4 {
                    break 'acao;
                }
                if l58 != 0 {
                    acao = 1;
                    break 'acao;
                }
                if (m != 0 && byte(m, 0x8ea9) >= 3) || l60 as f64 <= f64_em(0x968940) {
                    crate::comparacao::ramo("conducao alivia de longe");
                    chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                    break 'fim;
                }
                if *w(p, 0x198) == 2 {
                    acao = 4;
                } else {
                    let bl = byte(p, 0x109) as i32;
                    let e = sortear(ftol(fl(0x95)) as i16 as i32) + bl;
                    acao = if e < sortear(0x32) { 2 } else { 4 };
                }
            } else if acao == 4 && (byte(m, 0x8ea9) == 4 || byte(m, 0x8ea9) == -1) {
                acao = 1;
            }
        }
        if *w(p, 0x198) == 0x3b {
            acao = 3;
        }
        if modo == 4 && (acao == 1 || acao == 2 || l43 == 0) {
            acao = 3;
        }
        let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
            chamar(0x6d6f70, u(m, 0x8f7a), [0x1f44, m as u32, ub(p, 0x2e), lado as u8 as u32, u32::MAX]);
            *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x19);
        };
        match acao {
            1 => {
                if l58 == 0 {
                    crate::comparacao::ramo("conducao lanca sem companheiro");
                    *w(p, 0x198) = 9;
                    chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                } else {
                    crate::comparacao::ramo("conducao passa");
                    *w(p, 0x198) = 0x6c;
                    chamar(0x6fa250, p, [l8c as u8 as u32, l58 as u32, l54 as u32, l94 as u8 as u32, l98 as u8 as u32]);
                }
            }
            2 => {
                crate::comparacao::ramo("conducao divide");
                chamar(0x70ec80, p, [0; 5]);
                *w(p, 0x198) = 0x10;
            }
            3 => {
                if modo == 4 {
                    crate::comparacao::ramo("conducao goleiro fica");
                    volta = 1;
                    break 'fim;
                }
                crate::comparacao::ramo("conducao chuta");
                if byte(m, 0x1a7) != 0 && *w(p, 0x198) != 0x3b && byte(m, 0x8ea9) == 0 {
                    narrar(chamar);
                }
                let v = if l37 {
                    let v = valor_do_chute(p, &mut l88, &mut l64) as i16 as i32;
                    v / 2 + la4 / 2
                } else {
                    valor_do_chute(p, &mut l88, &mut l64) as i16 as i32
                };
                let r = conferir_lance(m);
                chutar(chamar, l88 as u8 as u32, v, l64, r as u32, true);
            }
            _ => {
                // conduzir
                let (mut cx, mut cy) = (sx, sy);
                if l5f {
                    crate::comparacao::ramo("conducao segura no canto");
                    cx = if l74 > 4 || (l74 == 4 && sortear(2) == 0) { 8 } else { 0 };
                    cy = ty;
                    l43 = 1;
                }
                if !na_boca() && !consegue() && (l43 == 0 || modo == 4) {
                    crate::comparacao::ramo("conducao vira para o gol");
                    let d = if lado == 0 {
                        if y0() < 9 { y0() + 3 } else { 0xb }
                    } else if y0() > 3 {
                        y0() - 3
                    } else {
                        0
                    };
                    apontar(p, x0(), y0(), 4, d);
                    chamar(0x6f09f0, p, [x0() as u8 as u32, y0() as u8 as u32, 0xf, 0, 0]);
                    *w(p, 0x198) = 0xf;
                    break 'fim;
                }
                if l43 == 0 && modo != 4 && byte(p, 0x109) == 0 {
                    if sortear(10) == 0 || alcanca(p, 4, ty) != 0 {
                        crate::comparacao::ramo("conducao sem saida lanca");
                        *w(p, 0x198) = 9;
                        chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                    } else {
                        crate::comparacao::ramo("conducao sem saida divide");
                        chamar(0x70ec80, p, [0; 5]);
                        *w(p, 0x198) = 0x10;
                    }
                    break 'fim;
                }
                let nx = cx.clamp(0, 8);
                let ny = cy.clamp(0, 0xb);
                if (lado == 0 && nx >= x0()) || (lado == 1 && nx <= x0()) {
                    apontar(p, x0(), y0(), ((nx as i32 + 4) / 2) as i8, ((ny as i32 / 2) as i8).wrapping_add(if lado != 0 { 0 } else { 6 }));
                } else {
                    apontar(p, x0(), y0(), nx, ny);
                }
                if byte(m, 0x1a7) != 0 {
                    narrar(chamar);
                }
                let a = *w(p, 0x198);
                if a == 0x12 || a == 0x11 {
                    chamar(0x6f09f0, p, [nx as u8 as u32, ny as u8 as u32, 0x12, l68 as u32, modo as u32]);
                    *w(p, 0x198) = 0x12;
                } else {
                    let q = u(m, 0xf5a6);
                    if l68 != 0 && p == u(m, 0xf5aa) && q != 0 && *w(p, 0x29) > -0x64 && *w(q, 0x198) != 0x34 && *w(q, 0x198) != 0x37 {
                        crate::comparacao::ramo("conducao tabela devolvida");
                        *b(q, 3) = (*b(q, 3)).wrapping_add(1);
                        *w(q, 0x35) = (*w(q, 0x35)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0x113) as i16);
                        let t = (m + 0x4782) as *mut i32;
                        *t = (*t).wrapping_add(0x113);
                    }
                    chamar(0x6f09f0, p, [nx as u8 as u32, ny as u8 as u32, 0x11, l68 as u32, modo as u32]);
                    *w(p, 0x198) = 0x11;
                }
            }
        }
    }
    // o destrutor da lista (0x68d040): na copia Rust da comparacao ela fica vazia
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    destruir(la);
    volta
}

/// A lista da pressao na pilha da conducao entra na fita sem o endereco (o da original e o da
/// copia Rust diferem); o resto da copia (o bloco do jogador, 0x1be bytes) e conferido. Vale nas
/// duas fases da comparacao (na da copia Rust a fita ja foi tirada).
pub(crate) fn lista_gravada(l: usize, n: u32) -> usize {
    if GRAVAR_CONDUCAO.load(std::sync::atomic::Ordering::Relaxed) && n == 0x1be {
        0
    } else {
        l
    }
}

/// Modo Rust: as chamadas da conducao vao para o jogo (passe, lancamento, dividida e movimento pelos
/// proprios desvios; o chute com a lista: a cabeca e o endereco).
pub(crate) unsafe fn despachar_conducao(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x6fa250 | 0x6f09f0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(rotina);
            f(this, a[0], a[1], a[2], a[3], a[4])
        }
        0x6fa490 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
            f(this, a[0], a[1]);
            0
        }
        0x70ec80 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
            f(this, a[0]);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(rotina);
            let l = a[4] as usize;
            f(this, a[0], a[1], a[2], a[3], if l != 0 { *(l as *const u32) } else { 0 }, l as u32);
            0
        }
        0x68d0d0 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(rotina);
            f(this, a[0] as usize, a[1]);
            0
        }
        _ => despachar_dividida(rotina, this, a),
    }
}

/// the decision of the player on the ball. Set pieces: corners, throw-in and free kicks pass (0x6c6a30 picks the
/// team-mate, otherwise a drawn cell); penalty and free kick near goal shoot (0x700bd0 + 0x702050). In play, shot, pass
/// and dribble scores from the distance to goal, the pressure (0x6bcbd0, with list), the score, the keeper, the
/// position's marks and the clock; long ball (0x6fa490), dividida (0x70ec80), shot, pass (0x6fa250) or dribble
/// (0x6f09f0) to 0x6bce80's cell. Mode 4: the keeper in the shoot-out (returns 1 when he stays). Determinized original.
/// Compared through the tape
/// Original: `dribble` (0x7109f0, match_pl.cpp (13 KB), thiscall, ret 4).
///
/// [pt] { 0x7109f0 } (ret 4, match_pl.cpp): ver `conduzir_com`.
pub(crate) unsafe extern "thiscall" fn conduzir(p: usize, modo: u32) -> u32 {
    conduzir_com(p, modo as u8, &mut |r, t, a| despachar_conducao(r, t, a)) as u32
}

pub(crate) unsafe extern "thiscall" fn comparar_conduzir(p: usize, modo: u32) -> u32 {
    if gravando() {
        // unidade da fita de fora (penaltis, decisao do jogador); por dentro, a propria comparacao
        return chamar_gravando(0x7109f0, p, [modo, 0, 0, 0, 0], || fita_da_conducao(p, modo));
    }
    fita_da_conducao(p, modo)
}

pub(crate) unsafe fn fita_da_conducao(p: usize, modo: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(319);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let cond = GRAVAR_CONDUCAO.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(
        reg,
        || {
            zerar_pilha();
            o(p, modo) & 0xff
        },
        |chamar| {
            conduzir_com(p, modo as u8, &mut |rotina, this, args| match rotina {
                // passe, lancamento e dividida: as unidades deles entram na fita da conducao
                0x6fa250 => {
                    passar_com(this, args[0] as u8, args[1] as usize, args[3], args[4], &mut |r, t, a| {
                        let mascara = match r {
                            0x6fad40 => [0xff, 0xff, 0xff, u32::MAX, u32::MAX],
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x7103e0 => [0; 5],
                            _ => [u32::MAX; 5],
                        };
                        chamar(r, t, a, mascara)
                    });
                    0
                }
                0x6fa490 => {
                    lancar_com(this, args[0] as i8, args[1] as u8, &mut |r, t, a| {
                        let mascara = match r {
                            0x6fad40 => [0xff, 0xff, u32::MAX, u32::MAX, 0xff],
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x6cd490 => [0xff, 0, 0, 0, 0],
                            _ => [u32::MAX; 5],
                        };
                        // a porta da 0x6fad40 grava (x, y, tipo, receptor, coluna); a linha do alvo nao entra
                        let a = if r == 0x6fad40 { [a[0], a[1], a[2], 0, a[3]] } else { a };
                        chamar(r, t, a, mascara)
                    });
                    0
                }
                0x70ec80 => {
                    dividida_com(this, args[0] as u8, &mut |r, t, a| {
                        let mascara = match r {
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                            0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                            0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                            0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                            0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                            _ => [0xff, 0, 0, 0, 0],
                        };
                        chamar(r, t, a, mascara)
                    });
                    0
                }
                0x68d0d0 => chamar(rotina, lista_gravada(this, args[1]), args, [u32::MAX, u32::MAX, 0, 0, 0]),
                _ => {
                    let mascara = match rotina {
                        0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                        0x6f09f0 => [0xff, 0xff, 0xffff, 0xff, 0xff],
                        // a cabeca da lista da pressao: so a copia do modo Rust a enche
                        0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                        _ => [u32::MAX; 5],
                    };
                    chamar(rotina, this, args, mascara)
                }
            }) as u32
        },
    );
    GRAVAR_CONDUCAO.store(cond, Relaxed);
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("conduzir(modo {}) original={} rust={} {} chamadas: {}", modo as u8, r, rr, k, detalhe_fita()));
    r
}

// ---------- centesima quadragesima sexta leva: a execucao do passe (0x6fad40) ----------

/// A nota base do passe pelos marcadores da posicao (casos 6..=8 e 0x5e de 0x6fad40): na linha de
/// fundo, com as marcas 0x880 nas duas tabelas (e 0x38 + a marca 4 da posicao), +0x4b/+0x3e/+0x2d
/// conforme o clube humano (0x600a50); senao +0x19 no fundo ou o desconto de um terco do sorteio.
pub(crate) unsafe fn nota_do_passe_pela_posicao(p: usize, base: i16, l20: i32, l14: i32) -> i16 {
    let m = *((p + 0x19e) as *const usize);
    let ta = || if m != 0 { tabela_a(m, p) } else { 0 };
    let tb = || if m != 0 { tabela_b(m, p) } else { 0 };
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let clube = *((p + 0xf9) as *const usize);
    let dirigido = || clube != 0 && humano(clube, 0, 1, 0, 0) != 0;
    let y = byte(p, 0x103);
    let marcas = || marcas_da_posicao(m + 0x91e2 + byte(p, 0x27) as isize as usize * 0x18e3, byte(p, 0x19));
    let fundo = y == 0 || y == 0xb;
    let w = base.wrapping_sub(l20 as i16);
    if fundo && ta() & 0x880 != 0 && tb() & 0x880 != 0 && ta() & 0x38 != 0 && marcas() & 0x4_0000_0000 != 0 {
        return w.wrapping_add(if dirigido() { 0x4b } else { 0x3e });
    }
    if fundo && ta() & 0x880 != 0 && tb() & 0x880 != 0 {
        return w.wrapping_add(if dirigido() { 0x3e } else { 0x2d });
    }
    let lado = byte(p, 0x27);
    if fundo || (((lado == 1 && y < 3) || (lado == 0 && y > 8)) && ta() & 0x880 != 0) {
        return w.wrapping_add(0x19);
    }
    base.wrapping_sub((l14 as i16 as i32 / 3) as i16).wrapping_sub(l20 as i16)
}

/// O desafiante tirou a bola na execucao do passe (0x6fad40, de 0x6fc0e5 a 0x6fc988): a disputa
/// limpa conta para ele (+0x198 = lance, +4, -0x3b/-0x46 em +0x35), a narracao (na area dele, 1 em
/// 2: a da falta e 0x20e8/0x20ea/0x20eb; senao pelo lance, com a defesa do goleiro 0x70d430 no lance
/// 1), ele domina (0x6f0630, +0x19c = nota - 400 em -200..=200), a bola espirra (0x6cd770) abaixo do
/// limite do lance ou a media de acertos e narrada; os ganhos dele e o +0x2b do passador.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn passe_desarmado_com(m: usize, p: usize, e: usize, lance: i8, nota: i16, a: u8, vantagem: u8, penalti: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let zero = |q: usize| (byte(q, 0x27) == 0) as u32;
    // a disputa limpa
    *w(e, 0x198) = lance as i16;
    *b(e, 4) = (*b(e, 4)).wrapping_add(1);
    let t = tabela_a(m, e);
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if t & 0x30 != 0 && tabela_a(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
    let narrou = (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
        && na_area_atacada((byte(e, 0x27) != 1) as i8, byte(e, 0x102), byte(e, 0x103))
        && sortear(2) == 0;
    if narrou {
        crate::comparacao::ramo("passe desarme narrado na area");
        narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
        let codigo = match sortear(0x32) {
            1 => 0x20ea,
            0 | 2..=5 => 0x20eb,
            _ => 0x20e8,
        };
        narrar(chamar, codigo, u32::MAX, zero(e), u32::MAX);
    } else {
        let codigo = match lance {
            1 => {
                crate::comparacao::ramo("passe desarme do goleiro");
                if defender_com(e, (nota as i32 - 200) as i16, &mut |rotina, this, x| {
                    if rotina == 0x6d6f70 {
                        chamar(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                    } else {
                        chamar(rotina, this, x)
                    }
                }) == 0
                {
                    return;
                }
                0
            }
            0x13 | 0x21 | 0x2d => 0x1f62,
            0x22 | 0x2c => 0x2150,
            0x29 => 0x1f61,
            0x2a | 0x2b => 0x1f64,
            0x15 | 0x2e => {
                let r = sortear(6);
                if (r as f64 + f32_em(e + 0xad) as f64 + f32_em(0x969584) as f64) < f32_em(p + 0xad) as f64 { 0x2164 } else { 0x1f5d }
            }
            _ => 0,
        };
        if codigo != 0 {
            narrar(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), par(p));
        }
    }
    chamar(0x6f0630, e, [0; 5]);
    *w(e, 0x19c) = ((nota as i32 - 400) as i16).clamp(-200, 200);
    let mut dominou = false;
    let limite = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
        crate::comparacao::ramo("passe bola espirra");
        chamar(0x6cd770, m, [ub(p, 0x102), ub(p, 0x103), 1 | zero(p) << 8, 0, 0]);
        if byte(m, 0x8eb2) == 0 {
            let mut k = 3;
            k -= (byte(e, 0x10d) as i32 > sortear(10)) as i32;
            k -= (byte(e, 0x10c) as i32 > sortear(10)) as i32;
            *b(e, 0x2b) = sortear(k) as u8;
            dominou = true;
        } else if penalti {
            chamar(0x716c90, p, [0; 5]);
        }
    } else if byte(m, 0x8ea9) < 4 {
        if byte(m, 0x8ea9) > -1 {
            *b(m, 0x8ea9) = 0;
        }
        let (x, y) = (byte(p, 0x102), byte(p, 0x103));
        if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
            apontar(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            apontar(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
        }
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let media = if byte(e, contas.0) != 0 { (ub(e, contas.1) as f64 / ub(e, contas.0) as f64) as f32 } else { 0.0 };
        if minuto > (sortear(0x14) as u8).wrapping_add(sorteio) as i8 {
            let me = u(e, 0x19e);
            let gols = byte(me, if byte(e, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
            let tentativas = if lance == 0x15 { ub(e, 8) > 3 && byte(e, 9) != 0 } else { ub(e, 6) > 1 };
            if gols == 0 && media as f64 > f64_em(0x96a6a0) && tentativas && *w(e, 0x35) > 0x1a5e && byte(e, 0xfd) == 0 {
                crate::comparacao::ramo("passe media de acertos");
                narrar(chamar, if lance == 0x15 { 0x21ce } else { 0x21cd }, ub(e, 0x2e), ub(e, 0x27), par(p));
                *b(e, 0xfd) = 1;
            }
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let me = u(e, 0x19e);
    let t = || if me != 0 { tabela_a(me, e) } else { 0 };
    let ganho: i16 = if dominou { 0x78 } else { 0xc8 };
    let longe = t() & 4 == 0 && t() & 2 == 0;
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if longe { ganho << 1 } else { ganho });
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(a);
    let alvo = if byte(e, 0x27) != 0 { 0xb } else { 0 };
    let perto = !(tabela_12x9(byte(e, 0x103) as i32 - alvo, byte(e, 0x102) as i32 - 4) as f64 > f64_em(0x967890));
    if (perto || atras_do_extremo(me, e) != 0) && *w(p, 0x29) > 0 {
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
    }
    if dominou && sortear(7) == 0 {
        narrar(chamar, 0x1f41, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
    }
}

/// A falta do desafiante na execucao do passe (0x6fad40, de 0x6fb678 a 0x6fc08f): como
/// `punir_no_movimento_com`, mas a bola parada fica com o passador antes das narracoes, o penalti
/// sorteia pela pessoa (+0x5a, sem o +6) e 10, e o evento 0x82 nao depende do cartao.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn falta_no_passe_com(m: usize, e: usize, p: usize, mut tipo: u8, cartao: u8, grav: i8, arriscado: i8, vantagem: u8, lance: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let zero = |q: usize| (byte(q, 0x27) == 0) as u32;
    let lado = byte(p, 0x27);
    let mut peso = 0f32;
    motor_punir_lance(m, e, p, &mut tipo, cartao as i8, grav, arriscado as u8, &mut peso, vantagem);
    narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
    let cor: i32 = match tipo {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    // o evento 0x82 (cartao 4/5 pelos motivos 5, 10, 11 e o historico do jogador)
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let v = ftol(media(u(0xb74350, 0), u(e, 0x6d), 0xb, 2, 6.5)) as i8;
    let base = *(0xae2364 as *const i32);
    let lista = if m != 0 { u(m, 0x478e) } else { 0 };
    if lista != 0
        && (tipo == 5 || tipo == 4)
        && (cartao == 0xa || cartao == 5 || cartao == 0xb)
        && u(e, 0x69) != 0
        && *(u(e, 0x69) as *const i32) < base
        && (u(p, 0x69) == 0 || *(u(p, 0x69) as *const i32) < base)
        && u(e, 0xf9) != 0
        && humano(u(e, 0xf9), 0, 1, 0, 0) == 0
        && (byte(u(e, 0x69), 0x5b) as i32) < v as i32 + 10
    {
        let pessoa = u(e, 0x6d);
        let k = 0xfu8.wrapping_sub(v as u8);
        if byte(pessoa, 0x1c) > (sortear(5) as u8).wrapping_add(k) as i8
            && byte(pessoa, 0x25) > (sortear(5) as u8).wrapping_add(k) as i8
            && *((pessoa + 0xb) as *const i16) > (sortear(0xdac) + 3000) as i16
            && v as i32 > sortear(3) + 1
        {
            crate::comparacao::ramo("passe evento 0x82");
            let mut ev = [0u8; 0x19];
            ev[0..4].copy_from_slice(&*(u(e, 0x69) as *const [u8; 4]));
            ev[4..8].copy_from_slice(&*(u(e, 0xf9) as *const [u8; 4]));
            ev[0xd] = lado as u8;
            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
            ev[0x13..0x15].copy_from_slice(&0x82u16.to_le_bytes());
            ev[0x15..0x19].copy_from_slice(&*(u(p, 0x69) as *const [u8; 4]));
            pedir_vetores(&[(ev.as_ptr() as usize, 0x19)], &[]);
            chamar(0x68d0d0, lista, [0, 0x19, 0, 0, 0]);
        }
    }
    let som: u32;
    if tipo == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("passe penalti");
        if cor as i16 == -1 {
            match sortear(4) {
                0 => narrar(chamar, 0x20e9, u32::MAX, ub(p, 0x27), u32::MAX),
                1 => narrar(chamar, 0x20ec, u32::MAX, ub(p, 0x27), u32::MAX),
                _ => {}
            }
        }
        som = 0x1f7a;
        *b(m, 0x8eb2) = 5;
        *((m + 0x478a) as *mut usize) = p;
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(500);
        if sortear(byte(u(e, 0x69), 0x5a) as i32) == 0 {
            let r = sortear(10);
            if ajustar_17(e, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(e, 0);
            }
        }
    } else {
        som = 0x1f79;
        *b(m, 0x8eb2) = 6;
    }
    *b(m, 0x8ea7) = *b(p, 0x102);
    *b(m, 0x8ea8) = *b(p, 0x103);
    *b(m, 0x8eae) = *b(p, 0x27);
    *((m + 0xf582) as *mut u32) = 0;
    if som == 0x1f7a {
        narrar(chamar, 0x1f7a, u32::MAX, ub(p, 0x27), u32::MAX);
    }
    let expulso = matches!(tipo, 3..=6);
    if cor as i16 > -1 {
        if (lance == 0x2c || lance == 0x2f) && vantagem != 0 {
            narrar(chamar, if lance == 0x2f { 0x21da } else { 0x21d9 }, ub(p, 0x2e), ub(p, 0x27), par(e));
            if expulso || som == 0x1f7a {
                *b(e, 0x16) = 1;
            }
        }
        if sortear(4) == 0 {
            let x = sortear(grav as i32);
            if x > sortear(0x14) {
                let t = u(m, 0x4792);
                let clube = u(t, if byte(e, 0x27) == 0 { 0x20 } else { 0x1c });
                let k = u(clube, 0x53);
                if k != 0 && byte(k, 0x77) == 8 {
                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrar(chamar, 0x2109, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        }
        narrar(chamar, cor as u32, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        if expulso {
            crate::comparacao::ramo("passe expulsao");
            jogador_saiu_com(m, e, 0, chamar);
        }
    }
    if som != 0x1f7a {
        let c = escolher_cobrador(m, lado, byte(m, 0x8eb2), 0);
        if c == 0 {
            narrar(chamar, som, 0xff, ub(p, 0x27), u32::MAX);
        } else {
            if tipo == 7 {
                narrar(chamar, 0x21e2, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
            } else {
                narrar(chamar, som, 0xff, ub(p, 0x27), u32::MAX);
            }
            let longe = (lado == 1 && byte(p, 0x103) < 4) || (lado == 0 && byte(p, 0x103) > 7);
            if som == 0x1f79 && vantagem == 0 && longe {
                if tipo == 7 {
                    if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
                        narrar(chamar, 0x21e1, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                    }
                } else if !(2..=6).contains(&byte(p, 0x102)) {
                    narrar(chamar, 0x21df, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                } else {
                    narrar(chamar, 0x21e0, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                }
            }
        }
    }
    if codigo_especial(lance) == 0 {
        let extra = expulso || som == 0x1f7a;
        if vantagem != 0 {
            narrar(chamar, 0x20dd, ub(p, 0x2e), ub(p, 0x27), u32::MAX);
            if tipo == 4 || tipo == 5 {
                *b(e, 0x16) = 1;
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        } else if sortear(9) == 0 {
            narrar(chamar, 0x20df, u32::MAX, zero(p), u32::MAX);
            if sortear(3) == 0 {
                narrar(chamar, 0x20e0, u32::MAX, zero(p), u32::MAX);
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        }
        aplicar_ao_time(m, e, peso);
        chamar(0x716e10, e, [0; 5]);
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// Os oito codigos de narracao pela direcao do passe (`bl` de 0x6fad40: 0x10 pela faixa, 0x20/0x40
/// para a ponta, 2/4 de lado, 1 para frente, 8 para tras), na ordem 0x10, 0x20, 0x40, 2, 4, 1, 8.
pub(crate) fn pela_direcao(bl: u8, c: [u32; 7], padrao: u32) -> u32 {
    for (k, f) in [0x10u8, 0x20, 0x40, 2, 4, 1, 8].iter().enumerate() {
        if bl & f != 0 {
            return c[k];
        }
    }
    padrao
}

/// { 0x6fad40 } (ret 0x1c, match_pl.cpp, ~22 KB) com as chamadas que mexem fora do motor injetadas:
/// a execucao do passe de `p` (tipo 6..=0x5e) de (x, y) para (tx, ty), com `receptor` (ou 0) e a
/// saida (1 quando o passe nao chega ao receptor). A nota do passe pelo tipo e pela distancia; o
/// desafiante (0x6c87e0) com a falta (como na dividida) ou o desarme; sem ele, o folego, o passe
/// impreciso (o alvo sorteado pelo fator de +0x4756), o custo em +0x35, a narracao da aproximacao, a
/// narracao do tipo de passe (pela direcao, pela saida e por +0x1a7 do motor), a bola parada seguinte
/// (+0x8ea9, +0x8eaa) e o tracado (0x6ccb60): pela tabela, chute (0x70c040/0x70a910) ou a bola em
/// transito (+0x1c5..+0x1c9, +0x477e) e bola fora (0x6cd490). Devolve 1 se a bola segue em jogo.
#[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
pub(crate) unsafe fn executar_passe_com(p: usize, x: i8, y: i8, tipo_in: i8, saida: *mut u8, receptor: usize, tx_in: i8, ty_in: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let m = u(p, 0x19e);
    let lado = byte(p, 0x27);
    let (mut tipo, mut tx, mut ty) = (tipo_in, tx_in, ty_in);
    // narracao com sete argumentos (a5 | a6 << 8 e a7 << 16 no codigo)
    let narrar7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo | (a7 as u8 as u32) << 16, m as u32, a3, a4, (a5 & 0xff) | (a6 & 0xff) << 8]);
    };
    // o folego e a distancia
    let r0 = sortear({
        let v = 10000 - ftol(f32_em(p + 0x4d) as f64) as i16 as i32;
        v * v / 50000
    }) as i16 as i32;
    let mut d = distancia_entre(x, y, tx, ty);
    let k = if (f32_em(p + 0xc5) as f64) < f32_em(0x968e70) as f64 { f32_em(0x968e70) as f64 } else { f32_em(p + 0xc5) as f64 };
    let mut l14 = sortear(ftol(d as f64 * d as f64 * d as f64 * d as f64 * f32_em(0x96d228) as f64 / k));
    if receptor == 0 {
        l14 = sortear(l14 as i16 as i32);
    }
    if (d as f64) < f64_em(0x968950) && tipo != 0x3d && tipo != 0x5c {
        l14 = l14.wrapping_add(ftol(f32_em(0x96d210) as f64 - d as f64 * f32_em(0x96d7c0) as f64));
    }
    let cinco = |v: f64| ((ftol(v * f32_em(0x96d754) as f64 + f64_em(0x968e18)) as i8 as i32 / 5) * 5) as i8;
    let mut l12: i8 = 1;
    let c5 = cinco(d as f64);
    if c5 as i32 >= 1 {
        l12 = c5;
    }
    if tipo != 0x36 && tipo != 0x5c && tipo != 0x3d {
        *b(m, 0x1a5 + lado as isize as usize) = 1;
    }
    let mut saida_lance = 0u16;
    classificar_lance(p, tipo as i16, x, y, tx, ty, (d as f64 + f64_em(0x967890)) as f32, &mut saida_lance);
    let l20 = sortear(saida_lance as i16 as i32);
    // a nota do passe pelo tipo
    let mut l13 = false;
    let wp = |o: usize| *w(p, o);
    let mut bp: i16 = match tipo {
        6..=8 => nota_do_passe_pela_posicao(p, wp(0x11a + tipo as usize * 2), l20, l14),
        0x5e => nota_do_passe_pela_posicao(p, wp(0x12a), l20, l14),
        0x34 | 0x37 => {
            l13 = true;
            wp(0x182).wrapping_sub((l14 as i16 as i32 / 5) as i16).wrapping_add(300)
        }
        0x36 => {
            l13 = true;
            wp(0x186).wrapping_sub((l14 as i16).wrapping_add(l14 as i16))
        }
        0x5a => {
            l13 = true;
            wp(0x156).wrapping_sub(l14 as i16).wrapping_add(300)
        }
        0x5b | 0x5c => wp(0x194).wrapping_sub(l14 as i16),
        0x5d => wp(0x130).wrapping_sub(l20 as i16).wrapping_sub(l14 as i16),
        _ => wp(0x11a + (tipo as isize * 2) as usize).wrapping_sub(l20 as i16).wrapping_sub(l14 as i16),
    };
    if byte(m, 0x8ea9) == 5 && d as f64 > f64_em(0x96d700) {
        let (n, c) = match byte(m, 0x4756) {
            1 => (7, 0x9689d0),
            2 => (5, 0x96af30),
            3 => (3, 0x9689d8),
            4 => (3, 0x96a678),
            _ => (0, 0),
        };
        if n != 0 && sortear(n) == 0 {
            crate::comparacao::ramo("passe disputa de penaltis");
            bp = bp.wrapping_mul(ftol(bp as f64 * f64_em(c)) as i16);
        }
    }
    let r1 = sortear(r0 as i16 as i32);
    // a nota do ataque: so os 16 bits baixos sao lidos depois
    let l44 = wp(0x19c).wrapping_sub(r1 as i16).wrapping_add(sortear(bp as i32) as i16).wrapping_add(wp(0x29));
    // o desafiante
    let (mut a, mut bb, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = if byte(m, 0x8ea9) == 4 || l13 { 0 } else { desafio_adversario(m, p, lado, 0x6c, &mut a, &mut bb, &mut nota, &mut lance, &mut arriscado, 0) };
    lesao_na_dividida_com(m, p, 0x6c, e, lance as i16, 0, chamar);
    if e != 0 && nota as i32 >= wp(0x19c) as i32 + 200 {
        let (mut cartao, mut grav, mut vantagem) = (0u8, 0i8, 0u8);
        let tipo_falta = falta_no_motor(m, e, p, lance, 0x6c, &mut cartao, &mut grav, &mut vantagem);
        if byte(m, 0x8ebb) != 0
            && na_area_atacada(lado, byte(e, 0x102), byte(e, 0x103))
            && (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
            && sortear(3) != 0
        {
            narrar7(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), 0xff, 0xff, 0);
        }
        if tipo_falta != 0 {
            if sortear(10) == 0 && arriscado == 0 {
                crate::comparacao::ramo("passe falta so registra");
                registrar_lance(m, lance as i16 as u16, e as u32, p as u32, 0, tipo_falta, *b(p, 0x102), *b(p, 0x103), cartao, grav as u8);
            } else {
                crate::comparacao::ramo("passe falta punida");
                falta_no_passe_com(m, e, p, tipo_falta, cartao, grav, arriscado, vantagem, lance, chamar);
                return 0;
            }
        }
        // o registro do lance pula a conta do penalti (0x6fb667 vai direto a 0x6fc0e5)
        let penalti = tipo_falta == 0 && vantagem != 0 && (na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) || arriscado != 0);
        crate::comparacao::ramo("passe desarmado");
        passe_desarmado_com(m, p, e, lance, nota, a, vantagem, penalti, chamar);
        return 0;
    }
    // sem desafiante: o folego de quem passa
    if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) && sortear(*w(p, 0x31) as i32) < -10 {
        crate::comparacao::ramo("passe sem folego");
        let pessoa = u(p, 0x69);
        let mut v = f32_em(p + 0x1a2) as f64 * f32_em(0x96d3b8) as f64;
        *((p + 0x1a2) as *mut f32) = v as f32;
        let lim = ((25 - byte(pessoa, 0x5d) as i32) * 2) as f32;
        if !(v > lim as f64) {
            v = lim as f64;
        }
        *((p + 0x1a2) as *mut f32) = v as f32;
        if v > f64_em(0x9688a8) {
            *((p + 0x1a2) as *mut f32) = 100.0;
        }
        narrar7(chamar, 0x211d, u32::MAX, (lado == 0) as u32, 0xff, 0xff, 0);
        if sortear(byte(pessoa, 0x5a) as i32 * 20 + 10) == 0 {
            let r = sortear(3);
            if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(p, 0);
            }
        }
    }
    *((m + 0xf576) as *mut usize) = p;
    *w(m, 0x8d4a) = tipo as i16;
    *b(p, 1) = (*b(p, 1)).wrapping_add(1);
    if sortear(byte(u(p, 0x69), 0x5a) as i32 * 20 + 10) == 0 {
        let r = sortear(3);
        if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
            desempenho_na_partida(p, 0);
        }
    }
    let mut l24 = 0i32;
    if u(m, 0xf582) != 0 {
        l24 = byte(m, 0x1a7) as i32;
        *((m + 0x4786) as *mut usize) = 0;
        *b(m, 0x8eae) = 0xff;
        *((m + 0xf582) as *mut usize) = 0;
        *b(m, 0x1a7) = 0;
        *w(p, 0x29) = 0;
        *w(p, 0x19c) = 0;
    }
    *w(m, 0x8ed6) = l44.wrapping_sub(nota);
    *b(m, 0x8eb9) = x as u8;
    *b(m, 0x8ea6) = y as u8;
    // a precisao do passe
    let mut fator = (byte(m, 0x8ea9) as f64 * f64_em(0x968e80) + f64_em(0x967890)) as f32;
    let c = match byte(m, 0x4756) {
        1 => 0x96d7bc,
        2 => 0x96d3d4,
        3 => 0x96d7b8,
        4 => 0x96d7b4,
        _ => 0,
    };
    if c != 0 {
        fator = (fator as f64 * f32_em(c) as f64) as f32;
    }
    let r = sortear(ftol(fator as f64 * fator as f64 * f32_em(0x968a78) as f64));
    if (l44 as i32 - nota as i32) < r {
        crate::comparacao::ramo("passe impreciso");
        let e8 = ftol((fator as f64 * d as f64 + f64_em(0x967890)) * f64_em(0x968968)) as i8 as i32;
        let s1 = sortear(e8);
        let s2 = sortear(e8);
        let nx = (tx as u8).wrapping_sub(s1 as u8).wrapping_add(s2 as u8) as i8;
        let ny = if m != 0 && tabela_a(m, p) == 1 && sortear((wp(0x29) as i32 + 400) * 50) != 0 {
            ty
        } else {
            let s1 = sortear(e8);
            let s2 = sortear(e8);
            (ty as u8).wrapping_sub(s1 as u8).wrapping_add(s2 as u8) as i8
        };
        if nx != tx || ny != ty {
            if receptor != 0 {
                if m != 0 && tabela_a(m, p) == 1 {
                    *w(p, 0x35) = wp(0x35).wrapping_sub(0x32);
                } else if na_metade_de_defesa(lado, y) && na_metade_de_defesa(lado, ny) {
                    *w(p, 0x35) = wp(0x35).wrapping_sub(0x6e);
                }
                *w(p, 0x35) = wp(0x35).wrapping_sub(if matches!(tipo, 8 | 7 | 0x5e | 6) { 0xf } else { 0x64 });
            }
            ty = ny;
            tx = nx;
            d = distancia_entre(x, y, nx, ny);
        }
    }
    // o custo do passe
    if receptor != 0 {
        if *saida != 0 && limite(lado as u8, ty) != 0 {
            *w(p, 0x35) = wp(0x35).wrapping_sub(if m != 0 && tabela_a(m, p) == 1 { 0xd } else { 0x14 });
        } else {
            if m != 0 && tabela_a(m, p) == 1 {
                *w(p, 0x35) = wp(0x35).wrapping_sub(0x42);
            } else if na_metade_de_defesa(lado, y) && na_metade_de_defesa(lado, ty) {
                *w(p, 0x35) = wp(0x35).wrapping_sub(0x64);
            }
            *w(p, 0x35) = wp(0x35).wrapping_sub(if matches!(tipo, 8 | 7 | 6) { 0x14 } else { 0x48 });
        }
    } else {
        *w(p, 0x35) = wp(0x35).wrapping_sub(0x24);
    }
    // a narracao da aproximacao (para onde ele leva a jogada)
    let acao = wp(0x198);
    let setpiece = matches!(tipo, 6 | 7 | 8 | 0x5e);
    if !matches!(acao, 0x12 | 0x11 | 0x10 | 0xf) && tipo != 8 && !setpiece && !l13 {
        let r = sortear(0x14);
        if l12 as i32 >= 30 - r && byte(m, 0x8ea9) < 3 && !(m != 0 && tabela_a(m, p) == 1) && sortear(3) == 0 {
            let t = if lado == 0 { tabela_12x9(byte(p, 0x103) as i32 - 0xb, byte(p, 0x102) as i32 - 4) } else { distancia_ao_ponto(p, 4, 0) };
            let c5 = cinco(t as f64);
            let n5: i8 = if (c5 as i32) < 1 { 1 } else { c5 };
            let (px, py) = (byte(p, 0x102), byte(p, 0x103));
            let fundo = (lado == 0 && py > 7) || (lado == 1 && py < 4);
            let recuado = (lado == 0 && py < 4) || (lado == 1 && py > 7);
            let canto_proprio = (lado == 0 && px < 2) || (lado == 1 && px > 6);
            let canto_outro = (lado == 0 && px > 6) || (lado == 1 && px < 2);
            let codigo = if fundo {
                if canto_proprio {
                    Some(0x2160)
                } else if canto_outro {
                    Some(0x2162)
                } else if na_area_atacada(lado, px, py) {
                    None
                } else {
                    Some(0x2161)
                }
            } else if recuado {
                Some(if canto_proprio { 0x215a } else if canto_outro { 0x215c } else { 0x215b })
            } else if (lado == 0 && px < 2) || (lado == 1 && px > 6) {
                Some(0x215d)
            } else if canto_outro {
                Some(0x215f)
            } else if (px as i32 - 4).abs() < 2 && (py == 5 || py == 6) && sortear(2) != 0 {
                Some(0x215e)
            } else {
                None
            };
            if let Some(cn) = codigo {
                crate::comparacao::ramo("passe aproximacao narrada");
                narrar7(chamar, cn, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, n5);
            }
        }
    }
    // a direcao do passe e a narracao pelo tipo
    let mut l54: i32 = -1;
    let rp = |o: usize| if receptor != 0 { ub(receptor, o) } else { 0 };
    let (bl, al): (u8, u8);
    if setpiece {
        let mut l13b = 0u8;
        if (lado == 0 && x < 3) || (lado == 1 && x > 5) {
            l13b = 1;
        } else if (lado == 1 && x < 3) || (lado == 0 && x > 5) {
            l13b = 2;
        }
        let dif = ((x as i32 - 4).abs() as u8).wrapping_sub((x as i32 - tx as i32).abs() as u8) as i8;
        bl = l13b;
        al = if dif > 0 { l13b | 8 } else if dif < 0 { l13b | 4 } else { l13b };
    } else {
        let mut f = 0u8;
        if receptor != 0 && u(receptor, 0x19e) != 0 && tabela_a(u(receptor, 0x19e), receptor) == 1 {
            f = 0x80;
        }
        if (lado == 0 && ty > y) || (lado == 1 && ty < y) {
            f |= 1;
        } else if (lado == 0 && ty < y) || (lado == 1 && ty > y) {
            f |= 8;
        }
        let dy = (ty as i32 - y as i32).abs();
        if dy < 2 {
            let dx = (tx as i32 - x as i32).abs();
            if ((lado == 0 && tx < x) || (lado == 1 && tx > x)) && dy <= dx {
                f |= 2;
            } else if ((lado == 0 && tx > x) || (lado == 1 && tx < x)) && dy <= dx {
                f |= 4;
            }
        }
        if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) && faixa((lado == 0) as u8, tx, ty) != 0 {
            f |= 0x10;
        }
        let longo = (lado == 1 && byte(p, 0x103) < 3) || (lado == 0 && byte(p, 0x103) > 8) || (lado == 0 && ty as i32 > y as i32 + 3) || (lado == 1 && (ty as i32) < y as i32 - 3);
        if longo {
            if fora_do_meio(tx) != 0 && ((lado == 0 && tx < byte(p, 0x102) && tx < 2) || (lado == 1 && tx > byte(p, 0x102) && tx > 6)) {
                f |= 0x20;
            } else if fora_do_meio(tx) != 0 && ((lado == 0 && tx > byte(p, 0x102) && tx > 6) || (lado == 1 && tx < byte(p, 0x102) && tx < 2)) {
                f |= 0x40;
            }
        }
        if lado == 0 && tx == 4 && ty == 0xc && (d as f64) < f64_em(0x968950) {
            tipo = if tipo != 0x3d { 0x5d } else { 0x5c };
        } else if lado == 1 && tx == 4 && ty == -1 && (d as f64) < f64_em(0x968950) {
            tipo = if tipo != 0x3d { 0x5d } else { 0x5c };
        } else if byte(m, 0x8ea9) < 4 && tipo != 0x5d && tipo != 0x5c && tx == 4 && (ty == -1 || ty == 0xc) {
            l54 = 0x211f;
        }
        bl = f;
        al = l13 as u8;
    }
    let n = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, l12);
    let nr = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, rp(0x2e), rp(0x27), a7);
    let grupo = |saida_v: u8, l24: i32, c: [[u32; 7]; 4]| -> [u32; 7] {
        match (saida_v != 0, l24 != 0) {
            (false, false) => c[0],
            (false, true) => c[1],
            (true, false) => c[2],
            (true, true) => c[3],
        }
    };
    let de_lado = |al: u8, c: [u32; 3]| if al & 8 != 0 { c[0] } else if al & 4 != 0 { c[1] } else { c[2] };
    match tipo {
        9 => {
            if (ty >= 0xc && lado == 1) || (ty < 0 && lado == 0) {
                n(chamar, if tx != 4 && sortear(4) != 0 { 0x210e } else { 0x211a });
            } else {
                let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
                n(chamar, if f(p, lado as u8) as u8 == 0 { 0x2157 } else { 0x1f70 });
            }
        }
        0x3e => n(chamar, 0x2134),
        0x5b => {
            if (0..0xc).contains(&ty) {
                n(chamar, 0x1f6f);
            } else {
                n(chamar, if tx != 4 && sortear(4) != 0 { 0x210d } else { 0x2119 });
            }
        }
        0x5d => n(chamar, pela_direcao(bl, [0x20b2, 0x20b3, 0x20b4, 0x20af, 0x20b0, 0x20ae, 0x20b1], 0x1f72)),
        6 | 8 | 0x5e => {
            let codigo = if al & 1 != 0 {
                if l24 != 0 { de_lado(al, [0x20c3, 0x20c1, 0x20bf]) } else { de_lado(al, [0x20bd, 0x20bb, 0x20b9]) }
            } else if al & 2 != 0 {
                if l24 != 0 { de_lado(al, [0x20c4, 0x20c2, 0x20c0]) } else { de_lado(al, [0x20be, 0x20bc, 0x20ba]) }
            } else {
                0x1f73
            };
            narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, 0);
        }
        7 => {
            let codigo = if al & 1 != 0 {
                if l24 != 0 { de_lado(al, [0x20cf, 0x20cd, 0x20cb]) } else { de_lado(al, [0x20c9, 0x20c7, 0x20c5]) }
            } else if al & 2 != 0 {
                if l24 != 0 { de_lado(al, [0x20d0, 0x20ce, 0x20cc]) } else { de_lado(al, [0x20ca, 0x20c8, 0x20c6]) }
            } else {
                0x21e4
            };
            narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, 0);
        }
        0x5c => n(chamar, 0x1f71),
        0x3d => {
            let rr = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, rp(0x2e), rp(0x27), 0);
            if bl & 0x80 != 0 && bl & 1 == 0 {
                rr(chamar, 0x2148);
            } else if faixa((lado == 0) as u8, tx, ty) != 0 {
                rr(chamar, 0x21d5);
            } else if bl & 2 != 0 {
                rr(chamar, 0x20b6);
            } else if bl & 4 != 0 {
                rr(chamar, 0x20b7);
            } else if bl & 1 != 0 {
                rr(chamar, 0x20b5);
            } else {
                rr(chamar, if bl & 8 != 0 { 0x20b8 } else { 0x1f6d });
            }
        }
        0x34 => {
            if (d as f64) < f64_em(0x968930) {
                n(chamar, 0x2130);
            } else {
                n(chamar, if faixa((lado == 0) as u8, tx, ty) == 0 { 0x2131 } else { 0x1f80 });
            }
        }
        0x5a => n(chamar, 0x1f7e),
        0x37 => n(chamar, if fora_do_meio(tx) != 0 { 0x21bc } else { 0x1f7d }),
        0x36 => {
            if receptor != 0 {
                nr(chamar, 0x1f7b, l12);
            } else {
                n(chamar, 0x1f7c);
            }
        }
        0x1f => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214a, 0);
            } else {
                let c = if byte(m, 0x8ea9) == 0 {
                    grupo(*saida, l24, [
                        [0x2026, 0x2027, 0x2028, 0x2023, 0x2024, 0x2022, 0x2025],
                        [0x202d, 0x202e, 0x202f, 0x202a, 0x202b, 0x2029, 0x202c],
                        [0x2034, 0x2035, 0x2036, 0x2031, 0x2032, 0x2030, 0x2033],
                        [0x203b, 0x203c, 0x203d, 0x2038, 0x2039, 0x2037, 0x203a],
                    ])
                } else {
                    grupo(*saida, l24, [
                        [0x2042, 0x2043, 0x2044, 0x203f, 0x2040, 0x203e, 0x2041],
                        [0x2049, 0x204a, 0x204b, 0x2046, 0x2047, 0x2045, 0x2048],
                        [0x2050, 0x2051, 0x2052, 0x204d, 0x204e, 0x204c, 0x204f],
                        [0x2057, 0x2058, 0x2059, 0x2054, 0x2055, 0x2053, 0x2056],
                    ])
                };
                nr(chamar, pela_direcao(bl, c, 0x1f67), l12);
            }
        }
        0xb => {
            let area: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214a, 0);
            } else if area(receptor, (lado == 0) as u8) as u8 != 0 && area(p, (lado == 0) as u8) as u8 != 0 && (d as f64) < f64_em(0x968950) {
                nr(chamar, 0x2021, l12);
            } else {
                let c = grupo(*saida, l24, [
                    [0x205e, 0x205f, 0x2060, 0x205b, 0x205c, 0x205a, 0x205d],
                    [0x2065, 0x2066, 0x2067, 0x2062, 0x2063, 0x2061, 0x2064],
                    [0x206c, 0x206d, 0x206e, 0x2069, 0x206a, 0x2068, 0x206b],
                    [0x2073, 0x2074, 0x2075, 0x2070, 0x2071, 0x206f, 0x2072],
                ]);
                nr(chamar, pela_direcao(bl, c, 0x1f69), l12);
            }
        }
        0x1e => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214c, 0);
            } else {
                let c = grupo(*saida, l24, [
                    [0x2096, 0x2097, 0x2098, 0x2093, 0x2094, 0x2092, 0x2095],
                    [0x209d, 0x209e, 0x209f, 0x209a, 0x209b, 0x2099, 0x209c],
                    [0x20a4, 0x20a5, 0x20a6, 0x20a1, 0x20a2, 0x20a0, 0x20a3],
                    [0x20ab, 0x20ac, 0x20ad, 0x20a8, 0x20a9, 0x20a7, 0x20aa],
                ]);
                nr(chamar, pela_direcao(bl, c, 0x1f6a), l12);
            }
        }
        0x3c => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214c, 0);
            } else {
                let c = grupo(*saida, l24, [
                    [0x207a, 0x207b, 0x207c, 0x2077, 0x2078, 0x2076, 0x2079],
                    [0x2081, 0x2082, 0x2083, 0x207e, 0x207f, 0x207d, 0x2080],
                    [0x2088, 0x2089, 0x208a, 0x2085, 0x2086, 0x2084, 0x2087],
                    [0x208f, 0x2090, 0x2091, 0x208c, 0x208d, 0x208b, 0x208e],
                ]);
                nr(chamar, pela_direcao(bl, c, 0x1f6b), l12);
            }
        }
        _ => {
            // ponytail: o original para o jogo ("MATCH_PLAYER::pass() - unknown pass"); os chamadores so usam os 16 tipos acima
            linha(&format!("motor: executar_passe: tipo desconhecido {:#x}", tipo));
            std::process::abort();
        }
    }
    if l54 as i16 > -1 {
        n(chamar, l54 as u32);
    }
    // a bola parada seguinte (+0x8ea9) e o tempo dela (+0x8eaa)
    let (v, t): (i8, i8) = match tipo {
        0x34 | 0x37 => {
            let v = if (d as f64) < f64_em(0x968930) && faixa((lado == 0) as u8, tx, ty) == 0 {
                0
            } else if sortear(0x28) != 0 {
                4
            } else {
                sortear(3) as i8
            };
            (v, sortear(3) as i8 + 1)
        }
        0x1e | 8 => (4, sortear(3) as i8 + 1),
        0x3c | 0x1f | 0x20 => {
            let r = if sortear(3) != 0 { sortear(4) } else { 0 };
            (4 - r as i8, sortear(2) as i8 + 1)
        }
        0xb | 7 => ((sortear(7) == 0) as i8, sortear(2) as i8),
        9 | 0x3e | 0x5a => {
            let r = if sortear(0x23) != 0 { 0 } else { sortear(2) };
            (4 - r as i8, sortear(2) as i8 + 2)
        }
        _ => {
            let v = if sortear(3) != 0 { 2 - sortear(2) as i8 } else { 4 - sortear(4) as i8 };
            (v, sortear(2) as i8)
        }
    };
    *b(m, 0x8ea9) = v as u8;
    *b(m, 0x8eaa) = t as u8;
    let mut bl2 = byte(m, 0x8ea9);
    let cai = (*saida != 0 && bl2 > 0 && sortear(3) == 0) || (bl2 < 4 && wp(0x19c) as i32 > sortear(200));
    if cai {
        bl2 = bl2.wrapping_sub(sortear(2) as i8);
        *b(m, 0x8ea9) = bl2 as u8;
    }
    let cy = ty.clamp(0, 0xb);
    let cx = tx.clamp(0, 8);
    *w(m, 0x8ed8) = por_diferenca(m, x, y, cx, cy) as i16;
    // 0x7003b0: o lado do passe e a conta de posse
    *b(m, 0x8eab) = lado as u8;
    if lado != -1 {
        *b(m, 0x8d46 + lado as isize as usize) = (*b(m, 0x8d46 + lado as isize as usize)).wrapping_add(1);
        let o = (m + 0x8d46 + (lado == 0) as usize) as *mut i8;
        if *o > 0 {
            *o = (*o).wrapping_sub(10);
        }
    }
    if receptor != 0 && (tx != byte(receptor, 0x102) || ty != byte(receptor, 0x103)) {
        *saida = 1;
    }
    let forca = if l44 > -100 { l44 as i32 } else { -100 };
    let r = chamar(0x6ccb60, m, [x as u8 as u32 | (y as u8 as u32) << 8 | (tx as u8 as u32) << 16 | (ty as u8 as u32) << 24, tipo as u8 as u32, forca as u32, 1, 0]) as u8;
    let fora = |m: usize| !((0..9).contains(&byte(m, 0x8ea7)) && (0..0xc).contains(&byte(m, 0x8ea8)));
    if r != 0 {
        if byte(m, 0x8eb2) != 0 {
            return 0;
        }
        if fora(m) {
            crate::comparacao::ramo("passe tracado sai");
            chamar(0x6cd490, m, [(lado == 0) as u32, 0, 0, 0, 0]);
        }
        return 0;
    }
    if tx == 4 && (ty == 0xc || ty == -1) {
        crate::comparacao::ramo("passe vira chute");
        let contra = (ty == -1) == (lado == 0);
        chamar(if contra { 0x70c040 } else { 0x70a910 }, p, [d.to_bits(), 0, 0, 0, 0]);
        if byte(m, 0x8eb2) != 0 {
            return 0;
        }
    }
    let v = byte(m, 0x8eaa) as i32;
    let k = if (v as f64 + f64_em(0x968918)) < f64_em(0x967890) { f64_em(0x967890) } else { byte(m, 0x8eaa) as f64 + f64_em(0x968918) };
    *b(m, 0x1c5) = ftol((d as f64 + d as f64) / k + f64_em(0x968968)) as u8;
    *b(m, 0x1c6) = tx as u8;
    *b(m, 0x1c7) = ty as u8;
    *b(m, 0x1c8) = *b(p, 0x102);
    *b(m, 0x1c9) = *b(p, 0x103);
    *((m + 0x477e) as *mut usize) = receptor;
    *b(m, 0x8ea7) = tx as u8;
    *b(m, 0x8ea8) = ty as u8;
    if fora(m) {
        crate::comparacao::ramo("passe em transito sai");
        chamar(0x6cd490, m, [lado as u8 as u32, 0, 0, 0, 0]);
    }
    (byte(m, 0x8eb2) == 0) as u8
}

/// Unidades da fita do passe: o evento 0x82 vai para a lista montado na pilha (confere o conteudo).
pub(crate) static GRAVAR_PASSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(crate) fn gravando_passe() -> bool {
    GRAVAR_PASSE.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// Modo Rust: as chamadas da execucao do passe vao para o jogo (o tracado com as coordenadas num
/// dword, os chutes com a distancia em bits; o resto como na dividida e no movimento).
pub(crate) unsafe fn despachar_passe(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x6ccb60 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, i8, u32, u32) -> u8 = core::mem::transmute(rotina);
            f(this, a[0] as u8 as i8, (a[0] >> 8) as u8 as i8, (a[0] >> 16) as u8 as i8, (a[0] >> 24) as u8 as i8, a[1] as u8 as i8, a[2], a[3]) as u32
        }
        0x70c040 | 0x70a910 => {
            let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(rotina);
            f(this, f32::from_bits(a[0]));
            0
        }
        _ => despachar_movimento(rotina, this, a),
    }
}

/// `p`'s pass (type 6..=0x5e) from (x, y) to (tx, ty), with receiver and the output (1 when it does not reach him).
/// Score by type and distance; challenger with foul or dispossession; without him, stamina, inaccurate pass by
/// +0x4756's factor, cost in +0x35, approach and pass-type commentary, the next set piece (+0x8ea9/+0x8eaa) and the
/// trace (0x6ccb60): shot (0x70c040/0x70a910) or ball in transit (+0x1c5..+0x1c9, +0x477e). Compared through the tape,
/// with comparison inside the unit
/// Original: `execute_pass` (0x6fad40, match_pl.cpp (22 KB), thiscall, ret 0x1c).
///
/// [pt] { 0x6fad40 } (ret 0x1c, match_pl.cpp): ver `executar_passe_com`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn executar_passe(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    executar_passe_com(p, x as u8 as i8, y as u8 as i8, tipo as u8 as i8, saida, receptor, tx as u8 as i8, ty as u8 as i8, &mut |r, t, a| despachar_passe(r, t, a))
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_executar_passe(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    if gravando() {
        // unidade da fita de fora (passe, lancamento): a resposta com o byte da saida; por dentro, a propria comparacao
        let mut r = 0u8;
        chamar_gravando(0x6fad40, p, [x, y, tipo, receptor as u32, tx], || {
            r = fita_do_passe(p, x, y, tipo, saida, receptor, tx, ty);
            r as u32 | ((*saida as u32) << 8)
        });
        return r;
    }
    fita_do_passe(p, x, y, tipo, saida, receptor, tx, ty)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn fita_do_passe(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u8, usize, u32, u32) -> u8>(321);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4), (saida as usize, 1)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let passe = GRAVAR_PASSE.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(
        reg,
        || o(p, x, y, tipo, saida, receptor, tx, ty) as u32,
        |chamar| {
            executar_passe_com(p, x as u8 as i8, y as u8 as i8, tipo as u8 as i8, saida, receptor, tx as u8 as i8, ty as u8 as i8, &mut |rotina, this, args| {
                let mascara = match rotina {
                    0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                    0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                    0x6cd490 => [0xff, 0, 0, 0, 0],
                    0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                    0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                    0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                    0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                    0x6ccb60 => [u32::MAX, 0xff, u32::MAX, u32::MAX, 0],
                    0x70c040 | 0x70a910 => [u32::MAX, 0, 0, 0, 0],
                    0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                    _ => [0xff, 0, 0, 0, 0],
                };
                chamar(rotina, this, args, mascara)
            }) as u32
        },
    );
    GRAVAR_PASSE.store(passe, Relaxed);
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && (r as u8) == (rr as u8), || format!("executar_passe(tipo {:#x}) original={} rust={} {} chamadas: {}", tipo as u8, r as u8, rr as u8, k, detalhe_fita()));
    r as u8
}

// ---------- centesima trigesima leva: os eventos do pos-jogo (0x6b74a0) ----------

/// { 0x6b74a0 } (cdecl, match_day.cpp): depois da partida, percorre a lista de eventos em
/// [0xb67dc8] (cursor reposto no inicio; 0x68d1b0 da o proximo) e trata os dos dois clubes da
/// `partida` (+0x1c, +0x20) que nao estao fora (0x53b900). Evento: +0 pessoa, +4 clube, +8 marca,
/// +0x13 tipo, +0x15 outra pessoa. Tipo 0x82: 0x759610 com as duas pessoas. 0x13: com o contrato
/// (0x4cf5f0), sem marca tira o 0x40 (0x8676d0) e o 0x2000000 (0x8674e0); com marca, se o 0x40 ja
/// esta e +0x5b < 6, ou por sorteio(+0x5b) == 0, a noticia 0x72c1f0 e o 0x2000000 (0x867700); senao
/// poe o 0x40 (0x867560). 0x0a: poe o 0x4000000 e, com ficha (+0x61), se sorteio(+0x5d) <
/// sorteio(5), 0x735840 ou o contrato com mais que o 0x4000000, e +0xb da ficha > sorteio(2000) +
/// 4500, a noticia. 0x0b: 0x7daf50. 0x1e: por sorteio(+0x5a) < sorteio(10) o 0x40000; 0x733a60;
/// com 0x600a50 zero e o tecnico (+0xcf) de ficha +0x14 > 15, 0x547af0 e -50 na palavra +0xa da linha
/// do clube em [[0xb67a44]] (linhas de 0x49 bytes); senao -75 em +0xa e +0xc (entre 1 e 10000).
/// Os outros: por sorteio(+0x5a) < sorteio(15, 8 ou 6 pela idade +0x18 e +0x5b) o 0x40000, e a
/// noticia com o tipo.
pub(crate) unsafe fn eventos_do_pos_jogo_com(partida: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    let protegido: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MEDIA_HAS_STORY_FOR_PLAYER);
    // as que mexem no banco vao por `chamar` (no modo Rust a rotina do jogo; na comparacao, a fita)
    let ch = core::cell::RefCell::new(chamar);
    let duas = |n: usize, a: usize, b: usize, p: usize| (ch.borrow_mut())(0x759610, n, [a as u32, b as u32, p as u32, 0, 0]);
    let tirar_40 = |c: usize, f: u32| (ch.borrow_mut())(0x8676d0, c, [f, 0, 0, 0, 0]);
    let tirar = |c: usize, f: u32| (ch.borrow_mut())(0x8674e0, c, [f, 0, 0, 0, 0]);
    let por_40 = |c: usize, f: u32| (ch.borrow_mut())(0x867560, c, [f, 0, 0, 0, 0]);
    let por = |c: usize, f: u32, z: u32| (ch.borrow_mut())(0x867700, c, [f, z, 0, 0, 0]);
    let noticia = |n: usize, p: usize, c: usize, t: u32, pa: usize, z: u32| (ch.borrow_mut())(0x72c1f0, n, [p as u32, c as u32, t, pa as u32, z]);
    let tipo_0b = |p: usize, c: usize, pa: usize| (ch.borrow_mut())(0x7daf50, 0, [p as u32, c as u32, pa as u32, 0, 0]);
    let tipo_1e = |n: usize, ev: usize, pa: usize| (ch.borrow_mut())(0x733a60, n, [ev as u32, pa as u32, 0, 0, 0]);
    let reserva = |t: usize, p: usize, um: u32| (ch.borrow_mut())(0x547af0, 0, [t as u32, p as u32, um, 0, 0]);
    // as consultas que alcancam a 0x68d0d0 (aloca) tambem vao por `chamar`
    let fora = |c: usize| (ch.borrow_mut())(0x53b900, 0, [c as u32, 0, 0, 0, 0]) as i32;
    let contrato = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x4cf5f0, n, [p as u32, c as u32, 0, 0, 0]) as usize;
    let teste_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let lista = || *(0xb67dc8 as *const usize);
    let noticias = || *(0xb6b7c8 as *const usize);
    let ler = |p: usize| core::ptr::read_unaligned(p as *const i32);
    let pessoa = |id: i32| (*(0xae23bc as *const usize) as isize + id as isize * 0x6e) as usize;
    let c45 = |c: usize| core::ptr::read_unaligned((c + 0x45) as *const u32);
    // a palavra +o da linha do clube em [[0xb67a44]] menos `menos`, entre 1 e 10000
    let ajustar = |clube: usize, o: usize, menos: i16| {
        let linha = (**(0xb67a44 as *const *const usize) as isize + ler(clube) as isize * 0x49) as usize;
        let w = (linha + o) as *mut i16;
        let v = core::ptr::read_unaligned(w).wrapping_sub(menos);
        core::ptr::write_unaligned(w, v.clamp(1, 10000));
    };
    if lista() == 0 {
        return;
    }
    core::ptr::write_unaligned((lista() + 0xa) as *mut u32, core::ptr::read_unaligned((lista() + 2) as *const u32));
    let mut k = 0i32;
    while k < core::ptr::read_unaligned(lista() as *const i16) as i32 {
        let ev = proximo(lista());
        if ev == 0 {
            return;
        }
        k += 1;
        let clube = (*(0xae23b4 as *const usize) as isize + ler(ev + 4) as isize * 0x245) as usize;
        if (clube != *((partida + 0x1c) as *const usize) && clube != *((partida + 0x20) as *const usize)) || fora(clube) != 0 {
            continue;
        }
        let tipo = core::ptr::read_unaligned((ev + 0x13) as *const u16);
        match tipo {
            0x82 => {
                crate::comparacao::ramo("eventos 0x82");
                duas(noticias(), pessoa(ler(ev)), pessoa(ler(ev + 0x15)), partida);
            }
            0x0b => {
                crate::comparacao::ramo("eventos 0x0b");
                tipo_0b(pessoa(ler(ev)), clube, partida);
            }
            _ => {
                let p = pessoa(ler(ev));
                let c = contrato(*(0xae1050 as *const usize), p, clube);
                if c == 0 {
                    continue;
                }
                match tipo {
                    0x13 if ler(ev + 8) == 0 => {
                        crate::comparacao::ramo("eventos 0x13 sem marca");
                        if marcas(c) & 0x40 != 0 {
                            tirar_40(c, 0x40);
                        }
                        if c45(c) & 0x2000000 != 0 {
                            tirar(c, 0x2000000);
                        }
                    }
                    0x13 => {
                        let ja = marcas(c) & 0x40 != 0 && byte(p, 0x5b) < 6;
                        if !ja && sortear(byte(p, 0x5b) as i32) != 0 {
                            crate::comparacao::ramo("eventos 0x13 marca");
                            if marcas(c) & 0x40 == 0 {
                                por_40(c, 0x40);
                            }
                        } else {
                            crate::comparacao::ramo("eventos 0x13 noticia");
                            noticia(noticias(), p, clube, 0x13, partida, 0);
                            if c45(c) & 0x2000000 == 0 {
                                por(c, 0x2000000, 0);
                            }
                        }
                    }
                    0x0a => {
                        crate::comparacao::ramo("eventos 0x0a");
                        por(c, 0x4000000, 0);
                        if *((p + 0x61) as *const usize) == 0 {
                            continue;
                        }
                        let a = sortear(byte(p, 0x5d) as i32);
                        let b = sortear(5);
                        if a < b || protegido(noticias(), p) != 0 || c45(c) != 0x4000000 {
                            let r = sortear(2000) + 0x1194;
                            let ficha = *((p + 0x61) as *const usize);
                            if core::ptr::read_unaligned((ficha + 0xb) as *const i16) as i32 > r {
                                crate::comparacao::ramo("eventos 0x0a noticia");
                                noticia(noticias(), p, clube, 0xa, partida, 0);
                            }
                        }
                    }
                    0x1e => {
                        crate::comparacao::ramo("eventos 0x1e");
                        let a = sortear(byte(p, 0x5a) as i32);
                        if a < sortear(10) {
                            por(c, 0x40000, 0);
                        }
                        tipo_1e(noticias(), ev, partida);
                        let r = teste_600a50(clube, 1, 1, 0, 0);
                        let t = *((clube + 0xcf) as *const usize);
                        if r == 0 && t != 0 && *((t + 0x69) as *const usize) != 0 && byte(*((t + 0x69) as *const usize), 0x14) > 0xf {
                            crate::comparacao::ramo("eventos 0x1e tecnico");
                            reserva(t, pessoa(ler(ev)), 1);
                            if clube != 0 {
                                ajustar(clube, 0xa, 0x32);
                            }
                        } else if clube != 0 {
                            crate::comparacao::ramo("eventos 0x1e sem tecnico");
                            ajustar(clube, 0xa, 0x4b);
                            ajustar(clube, 0xc, 0x4b);
                        }
                    }
                    _ => {
                        crate::comparacao::ramo("eventos outros");
                        let idade = byte(p, 0x18);
                        let a = sortear(byte(p, 0x5a) as i32);
                        let b = if idade < 0x18 && byte(p, 0x5b) < 0xf {
                            sortear(0xf)
                        } else if idade < 0x20 {
                            sortear(8)
                        } else {
                            sortear(6)
                        };
                        if a < b {
                            por(c, 0x40000, 0);
                        }
                        noticia(noticias(), p, clube, tipo as u32, partida, 0);
                    }
                }
            }
        }
    }
}

/// { 0x70dfd0 } (ret 0x10, match_pl.cpp): ver `jogada_com`.
pub(crate) unsafe extern "thiscall" fn jogada(j: usize, forca: u32, altura: u32, estilo: u32, forte: u32) -> u8 {
    jogada_com(j, forca as i16, f32::from_bits(altura), estilo as i8, forte as u8, &mut |rotina, this, a| despachar_dividida(rotina, this, a))
}

// ---------- centesima segunda leva: o folego do jogador (0x6f5810) ----------

/// { 0x6f5810 } (ret 4, match_pl.cpp): recupera o folego (+0x4d) e o teto dele (+0x51), sem sortear.
/// Sem ficha (+0x6d) nada muda. O fator e 1, ou 1,4 - 0,0004 x lesao (0x62c3d0) acima de 1000.
/// Sem `teto`: em campo (0x6c38c0 = 1) soma 10 x (ritmo +0x34 da ficha + 15) x fator; senao, fora
/// do banco (+0x19 >= 0), soma 12 x ritmo + 325 (com +0x1f = -1) ou o mesmo 10 x (ritmo + 15); o
/// folego para em 10000. Com `teto`: +0x51 = folego + (12 x ritmo + 85) x fator, com a base vezes
/// 5,56 x +0x20 - 255,76 quando +0x20 (ou +0x21 se aquele for -1) fica entre 46 e 64; sem os dois,
/// no banco o teto e o folego. Fora de jogo (+0x1c zero) e com pessoa, se a lesao (0x62c590,
/// ate 6500) passar do teto, o teto vira 6500. Por fim o teto soma +0x59 e para em 10000.
pub(crate) unsafe extern "thiscall" fn recuperar_folego(j: usize, teto: u8) {
    let lesao: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_INJURY_GET_FATIGUE);
    let limite: unsafe extern "thiscall" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_INJURY_GET_CONDITION);
    let papel: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let f = |o: usize| (j + o) as *mut f32;
    let ficha = *((j + 0x6d) as *const usize);
    if ficha == 0 {
        return;
    }
    let lesoes = *((0xb63d28) as *const usize);
    let pessoa = *((j + 0x69) as *const usize);
    let mut fator = 1.0f64;
    let v = lesao(lesoes, *(pessoa as *const u32));
    if v > 1000 {
        fator = f64_em(0x968dc0) - v as f64 * f64_em(0x96a608);
    }
    let ritmo = byte(ficha, 0x34) as i32;
    if teto == 0 {
        let m = *((j + 0x19e) as *const usize);
        let base = if m != 0 && papel(m, j) == 1 {
            Some((ritmo + 15) * 10)
        } else if byte(j, 0x19) < 0 {
            None
        } else if byte(j, 0x1f) == -1 {
            Some(ritmo * 12 + 0x145)
        } else {
            Some((ritmo + 15) * 10)
        };
        if let Some(base) = base {
            let x = base as f64 * fator + *f(0x4d) as f64;
            *f(0x4d) = x as f32;
            if x > f64_em(0x968910) {
                *f(0x4d) = 10000.0;
            }
        }
    } else {
        let marca = if byte(j, 0x20) >= 0 { byte(j, 0x20) } else { byte(j, 0x21) };
        let k = ritmo * 12 + 0x55;
        if marca < 0 && byte(j, 0x19) < 0 {
            *f(0x51) = *f(0x4d);
        } else {
            let v = if marca > 0x2d && marca < 0x41 { (marca as f64 * f64_em(0x96d798) - f64_em(0x96d790)) * k as f64 } else { k as f64 };
            *f(0x51) = (v * fator + *f(0x4d) as f64) as f32;
            if byte(j, 0x1c) == 0 && pessoa != 0 {
                let l = limite(lesoes, pessoa, 0).min(0x1964);
                if l as f32 > *f(0x51) {
                    *f(0x51) = 6500.0;
                }
            }
        }
    }
    let x = *f(0x59) as f64 + *f(0x51) as f64;
    *f(0x51) = x as f32;
    if x > f64_em(0x968910) {
        *f(0x51) = 10000.0;
    }
}

// ---------- centesima primeira leva: a defesa do goleiro (0x70d430) ----------

/// { 0x70d430 } (ret 4, match_pl.cpp) com as chamadas que mexem no jogo injetadas (comentarios
/// 0x6d6f70, bola desviada 0x6cd770): o goleiro tenta defender o chute de `forca`. Acima de 1000, ou
/// se sorteio(ficha +0x21 + +0xa5) nao ficar abaixo de sorteio(20), nao defende (0x1fe2) e devolve 1.
/// Senao defende e devolve 0: com +0x106 acima de sorteio(8) espalma (0x1fe3, +0x2b soma sorteio(2),
/// a bola espirra 3 casas), senao rebate (0x216f, +0x2b soma sorteio(3), 1 casa). A bola fica em jogo
/// (dentro) com sorteio(10), senao pode sair; o goleiro so e o chutador da linha de fundo se
/// sorteio(200) der zero. Rebatendo, cansa (+0x35 - 250) e, com sorteio(30 x +0x5a da ficha +0x69 +
/// 20) zero, perde sorteio(3) em +0x17 (0x7172d0) e o desempenho e refeito (0x6eecb0).
pub(crate) unsafe fn defender_com(j: usize, forca: i16, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u32 {
    let ajustar_17: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_ADJUST_MORALE);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    let m = *((j + 0x19e) as *const usize);
    let dir = byte(j, 0x27);
    let narra = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| {
        c(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, byte(j, 0x2e) as u8 as u32, dir as u8 as u32, u32::MAX]);
    };
    let ficha = *((j + 0x6d) as *const usize);
    let pega = forca <= 1000 && {
        let n = ftol(byte(ficha, 0x21) as f64 + *((j + 0xa5) as *const f32) as f64);
        sortear(n) < sortear(0x14)
    };
    if !pega {
        narra(chamar, 0x1fe2);
        return 1;
    }
    let espalma = byte(j, 0x106) as i32 > sortear(8);
    narra(chamar, if espalma { 0x1fe3 } else { 0x216f });
    let b2 = (j + 0x2b) as *mut i8;
    *b2 = (*b2).wrapping_add(sortear(if espalma { 2 } else { 3 }) as i8);
    let (dentro, chutador) = if sortear(10) != 0 { (1u32, 0usize) } else { (0, if sortear(0xc8) != 0 { 0 } else { j }) };
    let espalhar = if espalma { 3 } else { 1 };
    chamar(0x6cd770, m, [byte(j, 0x102) as u8 as u32, byte(j, 0x103) as u8 as u32, espalhar | (dir as u8 as u32) << 8, dentro, chutador as u32]);
    if espalma {
        return 0;
    }
    let w35 = (j + 0x35) as *mut u16;
    *w35 = (*w35).wrapping_add(0xff06);
    let v = *((*((j + 0x69) as *const usize) + 0x5a) as *const i8) as i32;
    if sortear(v * 30 + 0x14) == 0 {
        let r = sortear(3) as i8;
        if ajustar_17(j, r.wrapping_neg()) != 0 {
            desempenho(j, 0);
        }
    }
    0
}

/// { 0x70d430 } (ret 4, match_pl.cpp): ver `defender_com`. A bola desviada vai com `cansa` 1.
pub(crate) unsafe extern "thiscall" fn defender(j: usize, forca: i16) -> u32 {
    defender_com(j, forca, &mut |rotina, this, a| match rotina {
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, 1, a[4] as usize);
            0
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

// ---------- nonagesima nona leva: o lancamento (0x6fa490) ----------

/// { 0x6fa490 } (ret 8, match_pl.cpp) com as chamadas que mexem no jogo injetadas (0x6fad40
/// executa, 0x6cd490 bola fora, comentario 0x6d6f70): o lancamento (chutao, cruzamento, bola longa).
/// A direcao do jogador (+0x19a, graus) marca os lados para onde a bola vai (f12, f13, f14, f15). Perto
/// do gol adversario (distancia < c) e fora do miolo, sem `modo` 4: cruzamento (0x5e) para a linha 0..1
/// ou 10..11 e coluna 3..5. No proprio campo: modo 4 = 0x5c (marca +- sorteio(2) e 1 pelo lado da
/// direcao), senao 0x5d (+- sorteio(4) e 2); no campo adversario: modo 4 = 0x5b (+- sorteio(2) e 2),
/// senao 0x3e (bola em jogo, +0x8ea9 = -1) ou 9 (+- sorteio(5) e 4/3). Fora o modo 4, a linha recua 2
/// para o proprio lado quando nao vai para la. Sem `refinar` (+0x10b ou sorteio(3)) e longe do proprio
/// gol, o alvo vai a meio caminho da ponta (coluna 1 ou 7, pela direcao) e da area (linha mais perto
/// a ate 5, ou 3 no modo 4). Executa (0x6fad40, tipo, alvo); impedimento como 0x6fa250; bola fora da
/// grade sem +0x8eb2: 0x6cd490.
pub(crate) unsafe fn lancar_com(j: usize, modo: i8, refinar: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let impedido_de: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_CHECK_OFFSIDE);
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let m = || *((j + 0x19e) as *const usize);
    let b = |p: usize, o: usize| (p + o) as *mut i8;
    let s = |n: i32| sortear(n) as i8;
    let dir = byte(j, 0x27);
    let ang = *((j + 0x19a) as *const i16);
    // os lados para onde a bola vai, pela direcao do jogador
    let (mut f12, mut f13, mut f14, mut f15) = (false, false, false, false);
    if !(0x32..=0x136).contains(&ang) {
        if dir == 1 {
            if ang <= 0x136 {
                f13 = true;
            } else {
                f12 = true;
            }
        } else {
            f14 = true;
        }
    }
    if ang > 0x28 && ang < 0x8c {
        f13 = true;
    }
    if ang > 0x82 && ang < 0xe6 {
        if dir == 0 {
            if ang > 0xb4 {
                f12 = true;
            } else {
                f13 = true;
            }
        } else {
            f15 = true;
        }
    }
    if !f12 && ang > 0xdc && ang < 0x140 {
        f12 = true;
    }
    let d = if dir == 0 {
        let lin = (byte(j, 0x103) as i32 - 11).abs() as i8 as isize;
        let col = (byte(j, 0x102) as i32 - 4).abs() as i8 as isize;
        f32_em((0xa15e44 + 4 * (lin * 9 + col)) as usize)
    } else {
        dist_ponto(j, 4, 0)
    };
    let (bx, by) = (|| byte(m(), 0x8ea7), || byte(m(), 0x8ea8));
    let tipo: u32;
    let (mut tx, mut ty): (i8, i8);
    // a coluna e a linha conforme os lados: (desvio do sorteio, passo)
    let mira = |base: i8, para_menos: bool, para_mais: bool, n: i32, passo: i8, n_meio: i32| -> i8 {
        if para_menos {
            base.wrapping_sub(s(n)).wrapping_sub(passo)
        } else if para_mais {
            s(n).wrapping_add(base).wrapping_add(passo)
        } else {
            let r = s(n_meio);
            base.wrapping_sub(r).wrapping_add(s(n_meio))
        }
    };
    let x = byte(j, 0x102);
    // fcomp + test ah,1: perto do gol quando a distancia e menor (NaN conta como menor)
    let refina;
    if !(d as f64 >= f64_em(0x9688a0)) && !(2..=6).contains(&x) && modo != 4 {
        tipo = 0x5e;
        ty = if dir == 0 { 11i8.wrapping_sub(s(2)) } else { s(2) };
        tx = s(3).wrapping_add(3);
        refina = modo != -1;
    } else {
        let y = byte(j, 0x103);
        // dir 1 ataca para a linha 0: y >= 6 e o campo dele (e o mesmo com dir 0 e y < 6)
        let campo_proprio = (dir == 1 && y >= 6) || (dir == 0 && y < 6);
        if !campo_proprio {
            // do meio para a frente
            if modo == 4 {
                tipo = 0x5c;
                ty = mira(by(), f15, f14, 2, 1, 2);
                tx = mira(bx(), f12, f13, 2, 1, 2);
                refina = true;
            } else {
                tipo = 0x5d;
                ty = mira(by(), f15, f14, 4, 2, 3);
                tx = mira(bx(), f12, f13, 4, 2, 3);
                if dir == 0 {
                    if !f14 {
                        ty = ty.wrapping_add(2);
                    }
                } else if !f15 {
                    ty = ty.wrapping_sub(2);
                }
                refina = modo != -1;
            }
        } else if modo == 4 {
            tipo = 0x5b;
            ty = mira(by(), f15, f14, 2, 2, 2);
            tx = mira(bx(), f12, f13, 2, 2, 2);
            refina = true;
        } else {
            tipo = if byte(m(), 0x8ea9) != -1 { 9 } else { 0x3e };
            ty = mira(by(), f15, f14, 5, 4, 3);
            tx = mira(bx(), f12, f13, 5, 3, 3);
            if dir == 0 {
                if !f14 {
                    ty = ty.wrapping_add(2);
                }
            } else if !f15 {
                ty = ty.wrapping_sub(2);
            }
            refina = modo != -1;
        }
    }
    if refina {
        let y = byte(j, 0x103);
        let segue = refinar != 0 || ((b(j, 0x10b).read() != 0 || s(3) != 0) && !(dir == 0 && y < 2) && !(dir == 1 && y > 9));
        if segue {
            let ponta: i8 = if f12 {
                1
            } else if !f13 && s(2) != 0 {
                1
            } else {
                7
            };
            let mut linha: i8;
            if dir == 0 {
                linha = 10;
                loop {
                    let e = linha as i32 - y as i32;
                    if e > 5 || (modo == 4 && e > 3) {
                        linha -= 1;
                    } else {
                        break;
                    }
                }
            } else {
                linha = 1;
                loop {
                    let e = y as i32 - linha as i32;
                    if e > 5 || (modo == 4 && e > 3) {
                        linha += 1;
                    } else {
                        break;
                    }
                }
            }
            tx = ((ponta as i32 + tx as i32) / 2) as i8;
            ty = ((linha as i32 + ty as i32) / 2) as i8;
        }
    }
    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(1);
    // (x, y, tipo, coluna e linha do alvo): o receptor e zero
    let r = chamar(0x6fad40, j, [x as u8 as u32, byte(j, 0x103) as u8 as u32, tipo, tx as u8 as u32, ty as u8 as u32]);
    if r as u8 == 0 {
        return;
    }
    if tipo as u16 != 0x36 {
        let q = impedido_de(m(), dir, byte(j, 0x103), ty, 0);
        if q != 0 {
            let lin = if byte(q, 0x27) == 0 { byte(q, 0x103) as i32 - 11 } else { byte(q, 0x103) as i32 };
            let col = (byte(q, 0x102) as i32 - 4).abs() as i8 as isize;
            let lin = lin.abs() as i8 as isize;
            let d = f32_em((0xa15e44 + 4 * (lin * 9 + col)) as usize) as f64;
            if sortear(ftol(d * d + d * d)) != 0 {
                let mm = m();
                chamar(0x6d6f70, *((mm + 0x8f7a) as *const usize), [0x1f78, mm as u32, byte(q, 0x2e) as u8 as u32, dir as u8 as u32, u32::MAX]);
                let cont = (m() as isize + 0xf5ca + 54 * dir as isize) as *mut u8;
                *cont = (*cont).wrapping_add(1);
                *b(q, 0x13) = (*b(q, 0x13)).wrapping_add(1);
                *b(j, 1) = (*b(j, 1)).wrapping_sub(1);
                *b(m(), 0x8ea7) = byte(q, 0x102);
                *b(m(), 0x8ea8) = byte(q, 0x103);
                *b(m(), 0x8eb2) = 7;
                *b(m(), 0x8eae) = (dir == 0) as i8;
                *((m() + 0xf582) as *mut usize) = 0;
                return;
            }
            registrar(m(), 0xffff, q as u32, 0, 0, 8, byte(q, 0x102) as u8, byte(q, 0x103) as u8, 0xff, 0xff);
        }
    }
    let mm = m();
    if byte(mm, 0x8eb2) != 0 {
        return;
    }
    if !(0..9).contains(&byte(mm, 0x8ea7)) || !(0..12).contains(&byte(mm, 0x8ea8)) {
        chamar(0x6cd490, mm, [byte(j, 0x27) as u8 as u32, 0, 0, 0, 0]);
    }
}

/// { 0x6fa490 } (ret 8, match_pl.cpp): ver `lancar_com`.
pub(crate) unsafe extern "thiscall" fn lancar(j: usize, modo: i8, refinar: u8) {
    lancar_com(j, modo, refinar, &mut |rotina, this, a| match rotina {
        0x6fad40 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u8, usize, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_PASS);
            let mut saida = 1u8;
            let r = f(this, a[0] as i8, a[1] as i8, a[2], &mut saida, 0, a[3], a[4]);
            r as u32 | ((saida as u32) << 8)
        }
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY);
            f(this, a[0] as u8) as u32
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

// ---------- nonagesima oitava leva: o passe e o impedimento (0x6fa250) ----------

/// { 0x6fa250 } (ret 0x14, match_pl.cpp) com as chamadas que mexem no jogo injetadas
/// (`chamar(rotina, this, argumentos)`; a 0x6fad40 devolve a resposta no byte baixo e a saida que
/// ela grava no argumento 3 no byte seguinte): o passe do jogador (+0x2b sobe). Executa (0x6fad40 com
/// a casa dele, `tipo`, o receptor e os argumentos 4 e 5); sem sucesso, nada. Fora do tipo 0x36,
/// procura o impedido (0x6cfd70 na linha 5o argumento): sorteio(2 x d1 x d2, a distancia dele ao gol
/// adversario) zero e impedimento: narra (0x1f78), conta (+0xf5ca + 54 x lado), o impedido marca +0x13,
/// o passador perde 1 em +1, a bola vai a casa dele com o outro lado (+0x8eb2 = 7) e ninguem a tem.
/// Senao registra o lance (0x6bb9a0, 8). Sem +0x8eb2 e com a saida do passe: o receptor e o batedor
/// (+0xf57a), o lado conta a posse (+0x8d46: o dele sobe, o outro cai 10) e o receptor decide ja
/// (0x7103e0) se sorteio(80) nao passar de +0x105 x +0x107.
pub(crate) unsafe fn passar_com(j: usize, tipo: u8, receptor: usize, a4: u32, a5: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let impedido_de: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_CHECK_OFFSIDE);
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let m = || *((j + 0x19e) as *const usize);
    let b = |p: usize, o: usize| (p + o) as *mut i8;
    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(1);
    let dir = byte(j, 0x27);
    let r = chamar(0x6fad40, j, [byte(j, 0x102) as u8 as u32, byte(j, 0x103) as u8 as u32, tipo as u32, receptor as u32, a4]);
    if r as u8 == 0 {
        return;
    }
    let completo = (r >> 8) as u8;
    if tipo != 0x36 {
        let q = impedido_de(m(), dir, byte(j, 0x103), a5 as i8, 0);
        if q != 0 {
            let gol = if byte(q, 0x27) == 0 { 11 } else { 0 };
            let d1 = dist_ponto(q, 4, gol);
            let d2 = dist_ponto(q, 4, gol);
            let n = ftol(d2 as f64 * d1 as f64 + d2 as f64 * d1 as f64);
            if sortear(n) != 0 {
                let mm = m();
                chamar(0x6d6f70, *((mm + 0x8f7a) as *const usize), [0x1f78, mm as u32, byte(q, 0x2e) as u8 as u32, dir as u8 as u32, u32::MAX]);
                let cont = (m() as isize + 0xf5ca + 54 * dir as isize) as *mut u8;
                *cont = (*cont).wrapping_add(1);
                *b(q, 0x13) = (*b(q, 0x13)).wrapping_add(1);
                *b(j, 1) = (*b(j, 1)).wrapping_sub(1);
                *b(m(), 0x8ea7) = byte(q, 0x102);
                *b(m(), 0x8ea8) = byte(q, 0x103);
                *b(m(), 0x8eb2) = 7;
                *b(m(), 0x8eae) = (dir == 0) as i8;
                *((m() + 0xf582) as *mut usize) = 0;
                return;
            }
            registrar(m(), 0xffff, q as u32, 0, 0, 8, byte(q, 0x102) as u8, byte(q, 0x103) as u8, 0xff, 0xff);
        }
    }
    let mm = m();
    if byte(mm, 0x8eb2) != 0 || completo == 0 {
        return;
    }
    *((mm + 0xf57a) as *mut usize) = receptor;
    let dir = byte(j, 0x27);
    *b(mm, 0x8eab) = dir;
    if dir != -1 {
        let meu = (mm as isize + 0x8d46 + dir as isize) as *mut i8;
        *meu = (*meu).wrapping_add(1);
        let outro = (mm as isize + 0x8d46 + (dir == 0) as isize) as *mut i8;
        if *outro > 0 {
            *outro = (*outro).wrapping_sub(10);
        }
    }
    if receptor != 0 && byte(receptor, 0x2b) == 0 {
        let r = sortear(0x50);
        if byte(receptor, 0x105) as i32 * byte(receptor, 0x107) as i32 > r {
            chamar(0x7103e0, receptor, [0; 5]);
        }
    }
}

/// { 0x6fa250 } (ret 0x14, match_pl.cpp): ver `passar_com`. O 3o argumento e a saida da 0x6fad40.
pub(crate) unsafe extern "thiscall" fn passar(j: usize, tipo: u8, receptor: usize, mut a3: u32, a4: u32, a5: u32) {
    passar_com(j, tipo, receptor, a4, a5, &mut |rotina, this, a| match rotina {
        0x6fad40 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32, usize, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_PASS);
            let r = f(this, a[0] as i8, a[1] as i8, a[2], &mut a3, a[3] as usize, a[4], a5);
            r as u32 | ((a3 as u8 as u32) << 8)
        }
        0x7103e0 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_TURN);
            f(this);
            0
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

// ---------- nonagesima setima leva: a decisao de cada jogador a cada instante (0x7103e0) ----------

/// { 0x7103e0 } (match_pl.cpp) com as chamadas que mexem no jogo injetadas (`chamar(rotina, this,
/// argumentos)`: 0x7109f0 conduzir, 0x713fc0 e 0x714d40 as decisoes sem a bola): a decisao do jogador.
/// Sem a bola e sem a marca 1 de `tabela_a`: "desliga" (0x67) se sorteio(10) + +0x4d x c nao passar do
/// sorteio(65). Com a bola do outro lado (+0x8eae = lado oposto), ou do lado dele fora da faixa ou com
/// a bola fora de jogo, ou do outro lado sem batedor do lado dele: sorteio(20) zero e sem +0x10d tambem
/// desliga, cansando (+0x35 - 15, +0x2b = 1, +0x4d + 5..9). Com o lado na situacao 5 (+0x1ab), longe do
/// gol pela tabela (ou 0x6f09b0 no lado 1) o sorteio(12) e +0x10d contra sorteio(10) desligam (sem o
/// -15). Com +0x2b, so os marcados. Antes de cada desfecho, os marcados por ele (+0x1ae, +0x101) que
/// podem ser escolhidos (0x6bce20) se mexem (0x714d40) se +0x107 x +0x104 nao passar de sorteio(900) +
/// 50. O dono da bola conduz (0x7109f0) e pode decidir de novo (sorteio(+0x7d) contra sorteio(5));
/// os outros vao a 0x713fc0 (bola com o lado dele, na faixa, em jogo) ou a 0x714d40.
pub(crate) unsafe fn decidir_jogador_com(j: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let pode: unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_CAN_MARKER_CHALLENGE);
    let faixa: unsafe extern "stdcall" fn(u8, i8, i8) -> u8 = core::mem::transmute(FN_IS_IN_OWN_PENALTY_AREA);
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    let w = |o: usize| (j + o) as *mut u16;
    let b = |o: usize| byte(j, o);
    let u = |a: usize| *(a as *const usize);
    // os marcados por ele se mexem (0x714d40) quando o sorteio deixa
    let marcados = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        let mut k = 0i8;
        while k < b(0x101) {
            let p = u(j + 0x1ae + 4 * k as usize);
            if pode(u(j + 0x19e), p, b(0x102), b(0x103)) as u8 != 0 {
                let prod = b(0x107) as i32 * b(0x104) as i32;
                if prod <= sortear(0x384) + 0x32 {
                    c(0x714d40, p, [0; 5]);
                }
            }
            k += 1;
        }
    };
    let desliga = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, cansa_35: bool| {
        marcados(c);
        if cansa_35 {
            *w(0x35) = (*w(0x35)).wrapping_add(0xfff1);
        }
        *w(0x198) = 0x67;
        if b(0x2b) == 0 {
            *((j + 0x2b) as *mut u8) = 1;
        }
        let r = sortear(5) + 5;
        let f = (j + 0x4d) as *mut f32;
        *f = (r as f64 + *f as f64) as f32;
    };
    loop {
        let m = u(j + 0x19e);
        let ta = || if m == 0 { 0 } else { tabela_a(m, j) };
        let dir = b(0x27);
        let e_dono = j == u(m + 0xf582);
        if !e_dono && ta() & 1 == 0 {
            let v4d = *((j + 0x4d) as *const f32);
            let r1 = sortear(0x41);
            let r2 = sortear(10);
            // fcomp + test ah,0x41: segue so se passar do sorteio (NaN desliga)
            if !(r2 as f64 + v4d as f64 * f64_em(0x96d818) > r1 as f64) {
                marcados(chamar);
                *w(0x198) = 0x67;
                return;
            }
        }
        if !e_dono && ta() & 1 == 0 {
            let ae = byte(m, 0x8eae);
            let pressa = if ae as i32 == (dir == 0) as i32 {
                true
            } else if dir == ae {
                false
            } else {
                let bat = u(m + 0xf57a);
                !(bat != 0 && byte(bat, 0x27) == dir) && (faixa((dir == 0) as u8, byte(m, 0x8ea7), byte(m, 0x8ea8)) == 0 || byte(m, 0x8ea9) == -1)
            };
            if pressa && sortear(0x14) == 0 && b(0x10d) == 0 {
                desliga(chamar, true);
                return;
            }
        }
        if !e_dono && *((m as isize + 0x1ab + dir as isize) as *const i8) == 5 && ta() & 1 == 0 {
            let d = if dir == 0 {
                let lin = (b(0x103) as i32 - 11).abs() as i8 as isize;
                let col = (b(0x102) as i32 - 4).abs() as i8 as isize;
                f32_em((0xa15e44 + 4 * (lin * 9 + col)) as usize)
            } else {
                dist_ponto(j, 4, 0)
            };
            let r = sortear(0xc);
            // fcomp + test ah,0x41: so longe (sorteio maior que a distancia, NaN nao)
            if r as f64 > d as f64 && (b(0x10d) as i32) < sortear(10) {
                desliga(chamar, false);
                return;
            }
        }
        if b(0x2b) != 0 && !e_dono {
            marcados(chamar);
            *w(0x198) = 0x67;
            return;
        }
        if !e_dono {
            let ae = byte(m, 0x8eae);
            let com_bola = if ae as i32 == (dir == 0) as i32 {
                false
            } else if dir == ae {
                true
            } else {
                let bat = u(m + 0xf57a);
                (bat != 0 && byte(bat, 0x27) == dir) || (faixa((dir == 0) as u8, byte(m, 0x8ea7), byte(m, 0x8ea8)) != 0 && byte(m, 0x8ea9) != -1)
            };
            if com_bola {
                chamar(0x713fc0, j, [1, 0, 0, 0, 0]);
            } else {
                chamar(0x714d40, j, [0; 5]);
            }
            return;
        }
        marcados(chamar);
        chamar(0x7109f0, j, [0; 5]);
        *((u(j + 0x19e) + 0x1a7) as *mut u8) = 0;
        let m = u(j + 0x19e);
        if j == u(m + 0xf582) || *w(0x198) == 0x6d || b(0x2b) != 0 || byte(m, 0x8eb2) != 0 || b(0x107) == 0 {
            return;
        }
        let r = sortear(ftol(*((j + 0x7d) as *const f32) as f64));
        if r <= sortear(5) {
            return;
        }
    }
}

/// off the ball, "switches off" (0x67) when draw(10) + +0x4d·c does not beat draw(65); with the ball out of the side's
/// reach, draw(20) zero and no +0x10d switches off tiring (+0x35 − 15, +0x2b = 1, +0x4d + 5..9); with the side in
/// situation 5 (+0x1ab) and far from goal, switches off without the −15; with +0x2b, idle. Before each outcome the
/// marked players (+0x1ae) move (0x714d40) by draw(900) against +0x107·+0x104. The ball holder dribbles (0x7109f0, new
/// tape port) and may decide again (draw(+0x7d) against draw(5)); the others go to 0x713fc0 (ball with the side, in the
/// band, in play) or 0x714d40
/// Original: `decide_player` (0x7103e0, match_pl.cpp, thiscall).
///
/// [pt] { 0x7103e0 } (match_pl.cpp): ver `decidir_jogador_com`.
pub(crate) unsafe extern "thiscall" fn decidir_jogador(j: usize) {
    decidir_jogador_com(j, &mut |rotina, this, _a| match rotina {
        0x713fc0 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_PLAYER_CHOOSE_ATTACKING_OFF_BALL_MOVE);
            f(this, 1);
            0
        }
        0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE);
            f(this);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DECIDE_BALL_ACTION);
            f(this, 0)
        }
    })
}

// ---------- nonagesima sexta leva: o movimento geral sem a bola (0x714d40) ----------

/// { 0x714d40 } (match_pl.cpp, ~4,7 KB) com as chamadas que mexem no jogo injetadas
/// (`chamar(rotina, this, argumentos)`: 0x6f09f0, 0x6f46b0, 0x6f5b80, 0x6f0630 e o comentario
/// 0x6d6f70): o movimento do jogador sem a bola fora da faixa de ataque (0x7109df; na faixa, com a
/// bola em jogo, e 0x713fc0). Acao em +0x198.
/// Bola parada: o batedor (+0xf57a) ou quem o lado aponta (0x6cfda0, sem +0x1c5) vai a marca (0x100).
/// Bola fora de jogo: formacao (0x6c2a40, 0x68). Infiltracao (0x7a): quem tem a ficha boa (+0x105 x
/// +0x106 x +0x10a contra sorteio(25)), com alguem com a bola e atras da bola, corre para alem da linha
/// (+0x8eb0 + 2 ou +0x8eaf - 2) se o goleiro guardado (+0xf59a/+0xf596) nao estiver a frente: logo na
/// linha (marca 0x400 do bloco) ou a uma linha dela por sorteio(100) contra +0x106 x +0x10b.
/// Sem `tabela_a` = 1: apoio ao dono da bola do outro lado (0x6c4180) e bola parada; papeis de
/// pressao (0x700 com 0x38) correm para a bola por sorteio (+0xdd e distancia). Marcando alguem
/// (+0x1aa, 0x105): segue a casa dele (como 0x713fc0) ou larga (posicao -5, -3..-1). Senao a pressao:
/// limiar 1,5 (ou ~2 por +0x10d) reduzido sem +0x106/+0x10a e pelo sorteio contra +0x1a2; perto da bola
/// pressiona (alvo = bola) pelas marcas 0x8000/0x2000/0x4000 ou sorteio da posse do outro lado,
/// senao a casa de 0x6c2be0. Goleiro (`tabela_a` = 1): na bola pega e narra (0x1f46, 0x103), perto
/// sai a bola (0x103), senao a casa (0x71). Os outros: no fundo com a bola e o goleiro adversario
/// fora (0x78), avancam uma linha (0x6f5b80, 0x6b), sob o lado da bola vao a ela (0x100; nela,
/// pegam e narram 0x1f46 ou 0x2113, ou 0x104), na area adversaria com um marcador sozinho ficam
/// (0x104); fora isso a casa (0x68, ou 0x100 pressionando; ja nela 0x76).
pub(crate) unsafe fn mover_sem_bola_com(j: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((j + 0x19e) as *const usize);
    let dir = byte(j, 0x27);
    let bloco = (m as isize + 0x91e2 + 6371 * dir as isize) as usize;
    let w = |o: usize| (j + o) as *mut u16;
    let u = |a: usize| *(a as *const usize);
    let b = |o: usize| byte(j, o);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let formacao: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_REFERENCE_TACTIC_CELL);
    let do_lado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_PLAYER_NEAREST_TO_BALL);
    let posicionar: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_TARGET_CELL);
    let nota: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_BIAS_TARGET_COLUMN);
    let dif_lado: unsafe extern "thiscall" fn(usize, u8, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OFFSIDE_MARGIN_FOR_ROW);
    let limite: unsafe extern "stdcall" fn(u8, i8) -> u8 = core::mem::transmute(FN_IS_IN_ATTACKING_THIRD);
    let dif_jogador: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OFFSIDE_MARGIN);
    let atras: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_BEHIND_OWN_BACK_LINE);
    let faixa: unsafe extern "stdcall" fn(u8, i8, i8) -> u8 = core::mem::transmute(FN_IS_IN_OWN_PENALTY_AREA);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let menor_dist: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> f32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_MIN_OUTFIELD_DISTANCE_TO_CELL);
    let dist_entre: unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32 = core::mem::transmute(FN_GET_CELL_DISTANCE);
    let apontar: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_SET_FACING_DIRECTION);
    let ta = || if m == 0 { 0 } else { tabela_a(m, j) };
    let rep = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, x: i8, y: i8, a: i32| {
        c(0x6f09f0, j, [x as u8 as u32, y as u8 as u32, a as u32, 0, 0]);
    };
    // distancia da tabela entre o jogador e (x, y)
    let dist_tabela = |x: i8, y: i8| {
        let lin = (byte(j, 0x103) as i32 - y as i32).abs() as i8 as isize;
        let col = (byte(j, 0x102) as i32 - x as i32).abs() as i8 as isize;
        f32_em((0xa15e44 + 4 * (lin * 9 + col)) as usize)
    };
    // pega a bola parada na casa dela: aponta, narra e pega (0x6f0630)
    let pegar_e_narrar = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| {
        c(0x6d6f70, u(m + 0x8f7a), [codigo, m as u32, b(0x2e) as u8 as u32, dir as u8 as u32, u32::MAX]);
        c(0x6f0630, j, [0; 5]);
    };
    let (ae, a9) = (byte(m, 0x8eae), byte(m, 0x8ea9));
    let a_marca = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        rep(c, byte(m, 0x8ea7), byte(m, 0x8ea8), 0x100);
        *w(0x198) = 0x100;
    };
    if ae == -1 {
        let batedor = u(m + 0xf57a);
        let vai = if batedor == j { sortear(0x14).wrapping_add(b(0x107) as i32) != 0 } else { batedor == 0 && do_lado(m, j) != 0 && byte(m, 0x1c5) == 0 };
        if vai {
            a_marca(chamar);
            return;
        }
    }
    let ousado = byte_pos(bloco, b(0x19)) & 4 != 0;
    *w(0x198) = 0xffff;
    let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
    let t = ta();
    if a9 == -1 && ae != -1 {
        let (mut x, mut y) = (0u8, 0u8);
        formacao(m, j, &mut x, &mut y);
        *w(0x198) = 0x68;
        chamar(0x6f46b0, j, [x as u32, y as u32, 0, 0, 0]);
        return;
    }
    // infiltracao para alem da linha de impedimento (0x7a)
    {
        let (x, y) = (b(0x102), b(0x103));
        let r = sortear(0x19);
        let ficha = b(0x105) as i32 * b(0x106) as i32 * b(0x10a) as i32;
        let base = ficha > r && u(m + 0xf582) != 0 && t != 1;
        let corre = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, linha: i8, alem: i8| {
            *w(0x198) = 0x7a;
            rep(c, x, linha.wrapping_add(alem), -1);
        };
        if dir == 0 {
            if base && by > y && y < 6 {
                let gk = u(m + 0xf59a);
                let gy = |gk: usize| if gk != 0 { byte(gk, 0x103) } else { 9 };
                let l = byte(m, 0x8eb0);
                if gy(gk) <= l {
                    let d = y as i32 - l as i32;
                    if *((bloco + 0x584) as *const u32) & 0x400 != 0 && d < 2 && d > -2 {
                        corre(chamar, l, 2);
                        return;
                    }
                    let r2 = sortear(100);
                    if b(0x106) as i32 * b(0x10b) as i32 > r2 && y as i32 - byte(m, 0x8eb0) as i32 == 1 && gy(u(m + 0xf59a)) as i32 + 1 <= byte(m, 0x8eb0) as i32 {
                        corre(chamar, byte(m, 0x8eb0), 2);
                        return;
                    }
                }
            }
        } else if base && by < y && y > 5 {
            let gk = u(m + 0xf596);
            let gy = |gk: usize| if gk != 0 { byte(gk, 0x103) } else { -1 };
            let l = byte(m, 0x8eaf);
            if gy(gk) >= l {
                let d = l as i32 - y as i32;
                if *((bloco + 0x584) as *const u32) & 0x400 != 0 && d < 2 && d > -2 {
                    corre(chamar, l, -2);
                    return;
                }
                let r2 = sortear(100);
                if b(0x106) as i32 * b(0x10b) as i32 > r2 && byte(m, 0x8eaf) as i32 - y as i32 == 1 && gy(u(m + 0xf596)) as i32 - 1 >= byte(m, 0x8eaf) as i32 {
                    corre(chamar, byte(m, 0x8eaf), -2);
                    return;
                }
            }
        }
    }
    let (x, y) = (b(0x102), b(0x103));
    if t != 1 {
        let dono = u(m + 0xf582);
        let apoio = ae == -1 || (dono != 0 && byte(dono, 0x27) != dir && dif_lado(m, (dir == 0) as u8, byte(dono, 0x103) as u8) as i8 >= 0);
        let batedor = u(m + 0xf57a);
        if apoio && !(batedor != 0 && byte(batedor, 0x27) == dir) {
            if do_lado(m, j) != 0 {
                a_marca(chamar);
                return;
            }
            if ta() & 0x700 != 0 && ta() & 0x38 != 0 {
                let d = dist_tabela(bx, by);
                let r1 = sortear(ftol(*((j + 0xdd) as *const f32) as f64));
                let v = r1 + b(0x10d) as i32;
                if v > sortear(ftol(d as f64 * f32_em(0x96d784) as f64)) {
                    rep(chamar, bx, by, 0x100);
                    *w(0x198) = 0x100;
                    return;
                }
            }
        }
    }
    // marcacao individual (+0x1aa)
    let marcado = u(j + 0x1aa);
    if marcado != 0 {
        let larga_a_vez = *((bloco + 0x585) as *const u8) & 4 != 0 && u(m + 0xf582) != 0 && {
            if dir == 0 && y < 6 {
                !((by as i32 - 1) < y as i32 || byte(m, 0x8eaf) <= byte(marcado, 0x103))
            } else if dir == 1 {
                y <= 5 || (by as i32 + 1 <= y as i32 && byte(m, 0x8eb0) < byte(marcado, 0x103))
            } else {
                true
            }
        };
        if !larga_a_vez {
            let pm = byte(marcado, 0x19);
            if pm == -5 || (pm > -4 && pm < 0) {
                *((j + 0x1aa) as *mut usize) = 0;
            } else {
                *w(0x198) = 0x105;
                if b(0x105) != 0 && u(m + 0xf57a) == marcado {
                    rep(chamar, bx, by, -1);
                    return;
                }
                let (mut tx, mut ty) = (byte(marcado, 0x102), byte(marcado, 0x103));
                if limite((dir == 0) as u8, ty) == 0 && !ousado {
                    let r = sortear(0x15);
                    // fcomp + test ah,0x41: so passo a frente se o sorteio for maior (NaN nao)
                    if r as f64 > *((marcado + 0xad) as *const f32) as f64 {
                        if dir == 0 && ty > 0 {
                            ty -= 1;
                        } else if dir == 1 && ty < 11 {
                            ty += 1;
                        }
                        if tx <= 2 {
                            tx += 1;
                        } else if tx >= 6 {
                            tx -= 1;
                        }
                    }
                }
                rep(chamar, tx, ty, -1);
                return;
            }
        }
    }
    // pressao: alvo na bola, ou a casa de 0x6c2be0
    let (mut px, mut py) = (0i8, 0i8);
    let mut pressiona = false;
    if ae != -1 && t & 1 == 0 {
        let limiar: f64 = if sortear(ftol(*((j + 0xe1) as *const f32) as f64 + f64_em(0x968968))) == 0 {
            f64_em(0x9689a0)
        } else {
            let mut f = f64::from_bits(0x3ff8_0000_0000_0000);
            if b(0x10d) as i32 > sortear(10) {
                f = f64::from_bits(0x3fff_eb85_1eb8_51ec);
            }
            if b(0x106) == 0 {
                f *= f64_em(0x968ee0);
            }
            if b(0x10a) == 0 {
                f *= f64_em(0x968ee0);
            }
            let a2 = *((j + 0x1a2) as *const f32);
            let r3 = sortear(0x96) + 0x1e;
            if !(r3 as f64 >= a2 as f64) {
                f * f64_em(0x968ee0)
            } else {
                f
            }
        };
        // fcomp st(1) + test ah,1: perto quando a distancia e menor (NaN conta como menor)
        if !(dist_tabela(bx, by) as f64 >= limiar) {
            let mk = marcas(bloco, b(0x19));
            let est = *((bloco + 0x584) as *const u32);
            pressiona = mk & 0x8000 != 0
                || (marcas(bloco, b(0x19)) & 0x2000 != 0 && est & 0x100 != 0)
                || est & 0x4000 != 0
                || (ta() & 2 == 0 && ta() & 4 == 0 && {
                    let r4 = sortear(*((m as isize + 0x8d46 + (dir == 0) as isize) as *const i8) as i32);
                    r4 > sortear(25) && atras(m, j) == 0
                });
        }
    }
    if pressiona {
        (px, py) = (bx, by);
    } else {
        posicionar(m, j, &mut px, &mut py, 1);
        px = nota(j, px) as i8;
    }
    let v_goleiro = if dir != 0 { 0 } else { 11 };
    if t == 1 {
        if bx == x && by == y {
            if ae == -1 && a9 < 5 && byte(m, 0x1c5) == 0 {
                apontar(j, x, y, 4, v_goleiro);
                pegar_e_narrar(chamar, 0x1f46);
                *w(0x35) = (*w(0x35)).wrapping_add(4);
                *w(0x198) = 0x103;
            }
            return;
        }
        // test ah,0x41 / je: segue quando a distancia nao passa do limite (NaN tambem segue)
        if do_lado(m, j) != 0 && !(dist_tabela(bx, by) as f64 > f64_em(0x968940)) {
            if faixa(dir as u8, bx, by) != 0 && !(ae != -1 && (b(0x106) != 0 || b(0x10a) != 0)) {
                *w(0x198) = 0x103;
                rep(chamar, bx, by, 0x100);
                return;
            }
            if ae == -1 && (menor_dist(m, (dir == 0) as i8, bx, by, 1) as f64 > f64_em(0x968e18) || (b(0x106) == 0 && b(0x10a) == 0)) {
                if (0..=8).contains(&bx) && (0..=11).contains(&by) {
                    *w(0x198) = 0x103;
                    rep(chamar, bx, by, 0x100);
                }
                return;
            }
        }
        *w(0x198) = 0x71;
        rep(chamar, px, py, -1);
        return;
    }
    let no_fundo = (2..=6).contains(&x) && if dir == 1 { (10..=11).contains(&y) } else { (0..=1).contains(&y) };
    if ae != -1 && no_fundo && faixa(dir as u8, bx, by) != 0 {
        let q = manter(m, dir);
        if q == 0 {
            // a original mostra a caixa de erro e encerra o jogo (0x5cc7d0, 0x5a8f60, exit(-1))
            let f1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
            let f2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
            f1();
            f2();
            sair(-1);
        }
        if byte(q, 0x2b) > 1 {
            rep(chamar, 4, if dir != 0 { 11 } else { 0 }, -1);
            *w(0x198) = 0x78;
            return;
        }
    }
    if dif_jogador(m, j) as i8 >= 0 && b(0x107) != 0 {
        chamar(0x6f5b80, j, [0, 0, 0, 0, 0]);
        *w(0x198) = 0x6b;
        return;
    }
    let vai_a_bola = do_lado(m, j) != 0
        && if ae == -1 {
            (dir == 1 && y >= 6) || (dir == 0 && y < 6) || (b(0x10d) != 0 && !(dist_entre(bx, by, px, py) as f64 >= f32_em(0x9689a8) as f64))
        } else {
            b(0x10d) != 0
        };
    if vai_a_bola {
        if !(0..=8).contains(&bx) || !(0..=11).contains(&by) {
            *w(0x198) = if pressiona { 0x100 } else { 0x68 };
            rep(chamar, px, py, 0x100);
            return;
        }
        if bx != x || by != y {
            *w(0x198) = 0x100;
            rep(chamar, bx, by, 0x100);
            return;
        }
        if ae == -1 && a9 < 5 && byte(m, 0x1c5) == 0 {
            *w(0x35) = (*w(0x35)).wrapping_add(4);
            *w(0x198) = 0x100;
            apontar(j, x, y, 4, v_goleiro);
            let perto_do_gol = byte(m, 0x1cd) != 0 && (3..=5).contains(&x) && ((dir == 0 && y >= 10) || (dir == 1 && y <= 1));
            pegar_e_narrar(chamar, if perto_do_gol { 0x2113 } else { 0x1f46 });
        } else {
            *w(0x198) = 0x104;
        }
        return;
    }
    if no_fundo {
        let casa = (m as isize + 0x21b6 + 90 * (12 * x as isize + y as isize)) as usize;
        let outro = *((casa as isize + (dir == 0) as isize) as *const u8);
        let meu = *((casa as isize + dir as isize) as *const u8);
        if outro != 0 && meu == 1 && b(0x10a) != 0 {
            *w(0x198) = 0x104;
            return;
        }
    }
    if px == x && py == y {
        *w(0x198) = if pressiona { 0x100 } else { 0x76 };
        return;
    }
    *w(0x198) = if pressiona { 0x100 } else { 0x68 };
    rep(chamar, px, py, 0x100);
}

/// set piece (spot, 0x100), formation (0x68), run beyond the offside line (0x7a, by record +0x105/+0x106/+0x10a/+0x10b
/// and the kept goalkeeper), support and pressing (0x100), man-marking (+0x1aa, 0x105, or drop), pressing threshold
/// (1.5 or ~2, reduced without +0x106/+0x10a and by the draw against +0x1a2). Goalkeeper: picks up and comments
/// (0x1f46, 0x103), comes out (0x103) or returns (0x71). The others: deep (0x78), advance (0x6b), go to the ball
/// (0x100; on it picks up and comments 0x1f46/0x2113, or 0x104), stay in the opponent's area (0x104), cell (0x68/0x76)
/// Original: `move_off_ball` (0x714d40, match_pl.cpp, ~4.7 KB (outside the attacking band; inside it, with the ball in play, it is 0x713fc0), thiscall).
///
/// [pt] { 0x714d40 } (match_pl.cpp): ver `mover_sem_bola_com`.
pub(crate) unsafe extern "thiscall" fn mover_sem_bola(j: usize) {
    mover_sem_bola_com(j, &mut |rotina, this, a| match rotina {
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(this, a[0] as i8, a[1] as i8);
            0
        }
        0x6f5b80 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW);
            f(this, 0)
        }
        0x6f0630 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_POSSESSION);
            f(this);
            0
        }
        0x6d6f70 => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO);
            f(this, a[0] as i8, a[1] as i8, a[2] as i32, 0, 0)
        }
    })
}

// ---------- nonagesima quinta leva: o jogador sem a bola decide para onde ir (0x713fc0) ----------

/// { 0x713fc0 } (ret 4, match_pl.cpp) com as chamadas que mexem no jogo injetadas (0x6f09f0
/// reposicionar, 0x6f46b0 mover, 0x6f5b80 avancar): o jogador sem a bola escolhe a acao (+0x198) e a
/// casa. Bola fora de jogo (+0x8ea9 e +0x8eae = -1 juntos nao): vai a casa da formacao (0x6c2a40, 0x68).
/// Bola parada: quem bate (+0xf57a) vai a marca (0x100, salvo sorteio(20) + 0x107 = 0); sem batedor,
/// quem o lado aponta (0x6cfda0, sem +0x1c5) tambem. No jogo: perto da area adversaria e fora do miolo
/// (ou longe da bola pela tabela, > 2), ou com a marca 0x200000 da posicao (sorteio pela ficha +0x1a
/// contra sorteio(5)), tenta subir para uma casa livre (0x6bce80, 0x6a, +0x35 sobe 7) se sorteio(20
/// com a marca 0x40 de `tabela_a`, senao 50) < (1 - 0x35 x c) x 0x101 x 0x107. Senao parte da casa de
/// 0x6c2be0 (x por 0x715f50): pontas (marca 34 da posicao) vao ao canto pela bola (0x80 ou 0x800 de
/// `tabela_a`); a linha vai ao fundo (metade do x e linha 0 ou 11) se a bola do lado (+6) ou a marca
/// 0x800000 sortearem contra 0x107, ou se a linha permitir (0x6cb700, 0x109); a linha anda pela
/// diferenca do lado (0x6c4180). Marcando alguem (+0x1aa, 0x105): segue a casa dele, um passo a frente
/// se o sorteio(21) passar do 0xad dele (fora da faixa, 0x6cb930, ou com a marca 4 da posicao, nao).
/// Senao avanca uma linha (0x6f5b80, 0x6b) pela diferenca do jogador (0x6c4130); ja na casa: fica (0x76)
/// ou procura casa livre (0x6a/0x69); fora dela, pelo sorteio de 0xe1 + c contra 0x109, procura casa
/// livre ou vai (0x68). A original mostra uma caixa de erro se o marcado estiver fora (-5/-1).
pub(crate) unsafe fn decidir_sem_bola_com(j: usize, chamar: &mut dyn FnMut(usize, [u32; 5]) -> u32) {
    let m = *((j + 0x19e) as *const usize);
    let dir = byte(j, 0x27);
    let bloco = (m as isize + 0x91e2 + 6371 * dir as isize) as usize;
    let w = |o: usize| (j + o) as *mut u16;
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
    let formacao: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_REFERENCE_TACTIC_CELL);
    let escolher: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_CHOOSE_MOVE_CELL);
    let do_lado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_PLAYER_NEAREST_TO_BALL);
    let posicionar: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_TARGET_CELL);
    let nota: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_BIAS_TARGET_COLUMN);
    let dif_lado: unsafe extern "thiscall" fn(usize, u8, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OFFSIDE_MARGIN_FOR_ROW);
    let alem: unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_PLAYERS_AHEAD_OF_ROW);
    let limite: unsafe extern "stdcall" fn(u8, i8) -> u8 = core::mem::transmute(FN_IS_IN_ATTACKING_THIRD);
    let dif_jogador: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OFFSIDE_MARGIN);
    let atras: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_BEHIND_OWN_BACK_LINE);
    let ta = || if m == 0 { 0 } else { tabela_a(m, j) };
    let reposicionar = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32, x: i8, y: i8, a: i32| {
        c(0x6f09f0, [x as u8 as u32, y as u8 as u32, a as u32, 0, 0]);
    };
    // procura uma casa livre (0x6bce80 com `flags`) e vai para ela com a acao `acao`
    let subir = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32, flags: u32, acao: u16, soma_35: bool| -> bool {
        let (mut x, mut y) = (0i8, 0i8);
        if escolher(m, j, &mut x, &mut y, flags) == 0 {
            return false;
        }
        if soma_35 {
            *w(0x35) = (*w(0x35)).wrapping_add(7);
        }
        *w(0x198) = acao;
        reposicionar(c, x, y, -1);
        true
    };
    let ousado = byte_pos(bloco, byte(j, 0x19)) & 4 != 0;
    *w(0x198) = 0xffff;
    let (a9, ae) = (byte(m, 0x8ea9), byte(m, 0x8eae));
    if a9 == -1 && ae != -1 {
        let (mut x, mut y) = (0u8, 0u8);
        formacao(m, j, &mut x, &mut y);
        *w(0x198) = 0x68;
        chamar(0x6f46b0, [x as u32, y as u32, 0, 0, 0]);
        return;
    }
    if ae == -1 {
        let batedor = *((m + 0xf57a) as *const usize);
        let vai_a_marca = if batedor == j {
            sortear(0x14).wrapping_add(byte(j, 0x107) as i32) != 0
        } else {
            batedor == 0 && do_lado(m, j) != 0 && byte(m, 0x1c5) == 0
        };
        if vai_a_marca {
            reposicionar(chamar, byte(m, 0x8ea7), byte(m, 0x8ea8), 0x100);
            *w(0x198) = 0x100;
            return;
        }
    }
    // tenta subir para uma casa livre: sorteio(20 ou 50) < (1 - 0x35 x c) x 0x101 x 0x107
    let arrancar = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32| -> bool {
        let t = byte(j, 0x107);
        let n = if ta() & 0x40 != 0 { 20 } else { 50 };
        let chance = ((f64_em(0x968e18) - *((j + 0x35) as *const i16) as f64 * f64_em(0x96a600)) * (byte(j, 0x101) as i32 * t as i32) as f64) as f32;
        let r = sortear(n);
        !(r as f64 >= chance as f64) && subir(c, 4, 0x6a, true)
    };
    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
    let no_ataque = (dir == 1 && y < 3) || (dir == 0 && y > 8);
    let mut decidido = false;
    let mut testar_c = true;
    if no_ataque {
        let longe = (2..=6).contains(&x) || {
            let lin = (y as i32 - byte(m, 0x8ea8) as i32).abs() as i8 as isize;
            let col = (x as i32 - byte(m, 0x8ea7) as i32).abs() as i8 as isize;
            // fcomp + test ah,0x41: so passa se a distancia for maior que 2 (NaN nao)
            f32_em((0xa15e44 + 4 * (lin * 9 + col)) as usize) as f64 > f64_em(0x968950)
        };
        if longe {
            decidido = arrancar(chamar);
            testar_c = false;
        }
    }
    if decidido {
        return;
    }
    if testar_c {
        if marcas(bloco, byte(j, 0x19)) & 0x200000 != 0 {
            let r1 = sortear(*((*((j + 0x6d) as *const usize) + 0x1a) as *const i8) as i32);
            if r1 >= sortear(5) && subir(chamar, 0xc, 0x6a, false) {
                return;
            }
        }
        if arrancar(chamar) {
            return;
        }
    }
    let (mut ax, mut ay) = (0i8, 0i8);
    posicionar(m, j, &mut ax, &mut ay, 1);
    ax = nota(j, ax) as i8;
    if (marcas(bloco, byte(j, 0x19)) >> 32) & 4 != 0 {
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        let t = ta();
        let canto = t & 0x78 != 0;
        if dir == 0 {
            if t & 0x80 != 0 && canto {
                if bx < 2 || (bx < 6 && !(10..=11).contains(&by)) {
                    (ax, ay) = (0, 11);
                }
            } else if t & 0x800 != 0 && canto && bx < 2 {
                (ax, ay) = (8, 11);
            }
        } else if t & 0x80 != 0 && canto {
            if bx < 2 {
                (ax, ay) = (8, 0);
            }
        } else if t & 0x800 != 0 && canto && (bx < 2 || (bx < 6 && !(0..=1).contains(&by))) {
            (ax, ay) = (0, 0);
        }
    }
    let n: i32 = {
        let t = ta();
        if t & 0x200 == 0 {
            0x10
        } else if t & 8 != 0 {
            0x12
        } else if t & 4 != 0 {
            0x14
        } else {
            0
        }
    };
    let bola_do_lado = *((m as isize + 6 + dir as isize) as *const i8) != 0;
    let sorteia = if *((bloco + 0x584) as *const u32) & 0x1000 != 0 { bola_do_lado } else { marcas(bloco, byte(j, 0x19)) & 0x800000 != 0 };
    let ao_fundo = (sorteia && byte(j, 0x107) as i32 > sortear(n)) || {
        let yj = byte(j, 0x103);
        let perto = (dir == 0 && byte(m, 0x8ea8) > yj && yj > 7) || (dir == 1 && byte(m, 0x8ea8) < yj && yj < 4);
        perto && byte(j, 0x109) != 0 && {
            let v = alem(m, yj, dir, 1) as i8 as i32;
            v < sortear(3) + 1
        }
    };
    if ao_fundo {
        ay = if dir != 0 { 0 } else { 11 };
        ax = ((ax as i32 + 4) / 2) as i8;
    }
    if byte(j, 0x107) != 0 {
        let d = dif_lado(m, dir as u8, ay as u8) as i8;
        if d >= 0 {
            ay = if dir == 0 { ay.wrapping_sub(d) } else { ay.wrapping_add(d) };
        }
    }
    let t = ta();
    let marcado = *((j + 0x1aa) as *const usize);
    if marcado != 0 {
        let pm = byte(marcado, 0x19);
        // posicao -5 ou -1: a original mostra a caixa de erro e segue para a diferenca do jogador
        if pm != -5 && pm != -1 {
            *w(0x198) = 0x105;
            let (mut tx, mut ty) = (byte(marcado, 0x102), byte(marcado, 0x103));
            if byte(j, 0x10a) == 0 {
                // sem 0x10a nao segue o marcado: cai na diferenca do jogador (com 0x198 = 0x105)
            } else {
                if limite((dir == 0) as u8, ty) == 0 && !ousado {
                    let r = sortear(0x15);
                    // fcomp + test ah,0x41: so passo a frente se o sorteio for maior (NaN nao)
                    if r as f64 > *((marcado + 0xad) as *const f32) as f64 {
                        if dir == 0 && ty > 0 {
                            ty -= 1;
                        } else if dir == 1 && ty < 11 {
                            ty += 1;
                        }
                        if tx <= 2 {
                            tx += 1;
                        } else if tx >= 6 {
                            tx -= 1;
                        }
                    }
                }
                reposicionar(chamar, tx, ty, -1);
                return;
            }
        }
    }
    let d = dif_jogador(m, j) as i8;
    if d >= 0 {
        let r = sortear(d as i32 * 5).wrapping_add(byte(j, 0x107) as i32);
        if r > sortear(3) {
            *w(0x198) = 0x6b;
            chamar(0x6f5b80, [0, 0, 0, 0, 0]);
            return;
        }
    }
    let livre = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32, subir: &dyn Fn(&mut dyn FnMut(usize, [u32; 5]) -> u32, u32, u16, bool) -> bool| -> bool {
        (byte(j, 0x107) != 0 && subir(c, 4, 0x6a, true)) || (byte(j, 0x101) != 0 && t & 6 == 0 && byte(j, 0x107) != 0 && subir(c, 4, 0x69, true))
    };
    if byte(j, 0x102) == ax && byte(j, 0x103) == ay {
        let (x, y) = (byte(j, 0x102), byte(j, 0x103));
        let fica = (t == 1 && byte(j, 0x10a) != 0) || byte(j, 0x10a) != 0 || atras(m, j) != 0;
        if fica || !livre(chamar, &subir) {
            *w(0x198) = 0x76;
            reposicionar(chamar, x, y, -1);
        }
        return;
    }
    let n = ftol(*((j + 0xe1) as *const f32) as f64 + f32_em(0x968a78) as f64);
    if byte(j, 0x109) as i32 > sortear(n) && livre(chamar, &subir) {
        return;
    }
    *w(0x198) = 0x68;
    reposicionar(chamar, ax, ay, -1);
}

/// { 0x713fc0 } (ret 4, match_pl.cpp): ver `decidir_sem_bola_com`. O argumento nao e lido.
pub(crate) unsafe extern "thiscall" fn decidir_sem_bola(j: usize, _a: u32) {
    decidir_sem_bola_com(j, &mut |rotina, a| match rotina {
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(j, a[0] as i8, a[1] as i8);
            0
        }
        0x6f5b80 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW);
            f(j, 0)
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO);
            f(j, a[0] as i8, a[1] as i8, a[2] as i32, 0, 0)
        }
    })
}

// ---------- nonagesima segunda leva: avancar uma linha (0x6f5b80), com fita generica ----------

/// { 0x6f5b80 } (ret 4, match_pl.cpp) com as chamadas injetadas: o jogador anda uma linha para o
/// lado dele (+0x27 = 1 sobe ate 11, senao desce ate 0). Com `direto`, vai (0x6f46b0) para a coluna
/// atual na linha da bola (+0x8eb0 + 1 ou +0x8eaf - 1 do motor); senao reposiciona (0x6f09f0) na
/// nova linha com -1, 0, 0. Devolve o `eax` da chamada (os chamadores ignoram).
pub(crate) unsafe fn avancar_linha_com(this: usize, direto: u8, chamar: &mut dyn FnMut(usize, [u32; 5]) -> u32) -> u32 {
    let x = byte(this, 0x102);
    let mut y = byte(this, 0x103);
    let motor = *((this + 0x19e) as *const usize);
    let sobe = byte(this, 0x27) == 1;
    if sobe {
        if y < 11 {
            y += 1;
        }
    } else if y > 0 {
        y -= 1;
    }
    if direto != 0 {
        let linha = if sobe { byte(motor, 0x8eb0).wrapping_add(1) } else { byte(motor, 0x8eaf).wrapping_sub(1) };
        return chamar(0x6f46b0, [x as u8 as u32, linha as u8 as u32, 0, 0, 0]);
    }
    chamar(0x6f09f0, [x as u8 as u32, y as u8 as u32, u32::MAX, 0, 0])
}

/// { 0x6f5b80 } (ret 4, match_pl.cpp): ver `avancar_linha_com`.
pub(crate) unsafe extern "thiscall" fn avancar_linha(this: usize, direto: u8) -> u32 {
    avancar_linha_com(this, direto, &mut |rotina, a| {
        if rotina == 0x6f46b0 {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(this, a[0] as i8, a[1] as i8)
        } else {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO);
            f(this, a[0] as i8, a[1] as i8, -1, 0, 0)
        }
    })
}

// ---------- octogesima oitava leva: lado da bola, momento do jogador, partida de interesse ----------

/// 0x6f44a0; 1 in 11: +0xff drops 1 above (+0x100/2 + record +0x25 + 10)/4, +0x4d adds +0x59 (which clears if 1 or
/// halves), above 50 in +1 subtracts (+1 − 50)^1.25/4 (up to 25); +0x4d within 1..10000 (prologue with a relative
/// `call`)
/// Original: `evolve_moment` (0x6f55c0, match_pl.cpp, thiscall).
///
/// [pt] { 0x6f55c0 } (match_pl.cpp): sorteia o momento (0x6f44a0); 1 em 11 vezes ainda: o byte +0xff
/// cai 1 se passar de (+0x100 / 2 + 10 + ficha +0x25) / 4; +0x4d soma +0x59 (que zera se era 1 e
/// senao cai a metade); acima de 50 em +1, tira (+1 - 50)^1,25 / 4 (no maximo 25); +0x4d fica em
/// 1..10000.
pub(crate) unsafe extern "thiscall" fn evoluir_momento(this: usize) {
    let momento: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_ROLL_CURRENT_RATINGS);
    momento(this);
    if sortear(11) != 0 {
        return;
    }
    let ff = (this + 0xff) as *mut i8;
    let h = (byte(this, 0x100) as i32 / 2 + byte(*((this + 0x6d) as *const usize), 0x25) as i32 + 10) / 4;
    if *ff as i32 > h {
        *ff = (*ff).wrapping_sub(1);
    }
    let f = |o: usize| (this + o) as *mut f32;
    *f(0x4d) = (*f(0x4d) as f64 + *f(0x59) as f64) as f32;
    if *(f(0x59) as *const u32) == 0x3f80_0000 {
        *(f(0x59) as *mut u32) = 0;
    } else {
        *f(0x59) = (*f(0x59) as f64 * f64_em(0x968968)) as f32;
    }
    let b1 = *((this + 1) as *const u8);
    if b1 > 50 {
        let v = pow_do_jogo((b1 as i32 - 50) as f64, f64_em(0x968928)) * f32_em(0x96d22c) as f64;
        // fcom + test ah,0x41: so passa de 25 quando maior (NaN fica)
        let v = if v > f64_em(0x968890) { f32_em(0x96af88) as f64 } else { v };
        *f(0x4d) = (*f(0x4d) as f64 - v) as f32;
    }
    if !(*f(0x4d) as f64 >= f64_em(0x967890)) {
        *f(0x4d) = 1.0;
    } else if *f(0x4d) as f64 > f64_em(0x968910) {
        *f(0x4d) = 10000.0;
    }
}

/// { 0x6b61e0 } (match_day.cpp): a partida interessa a algum dos 16 tecnicos humanos (as ultimas
/// pessoas, tipo +0x5f = 1)? Pela competicao (+0x14): as duas de 0x9cf668/0x9cf684 so pelas selecoes
/// dos clubes (+0x11c da nacao); as de 0x9cf794/0x9cf798 nunca; amistoso (0x5c6710 de 0xae2c74): o
/// clube do tecnico (+0x39) esta nela (0x48db80); a de 0x9cf6f8 so na rodada 150 (0x4b0870 de +0x30);
/// com nacao (+0x5d): clube do tecnico da mesma nacao e um dos dois clubes fora da divisao dela;
/// senao pelas selecoes dos tecnicos ou pela competicao continental do grupo (0xae23a4, 0xc6 bytes).
pub(crate) unsafe extern "C" fn partida_de_interesse(p: usize) -> u32 {
    let g = |a: usize| *(a as *const i32);
    let comp = || *((p + 0x14) as *const usize);
    let id = *(comp() as *const i32);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_INCLUDES_CLUB);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let rodada: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    let do_grupo: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    let objeto = || *((*(0xadadfc as *const usize)).wrapping_add((*(comp() as *const i32) as usize).wrapping_mul(4)) as *const usize);
    let tecnico = |s: i32| (*(0xae23bc as *const usize)).wrapping_add(((g(0xae2364) - s - 1) as usize).wrapping_mul(110));
    let u = |a: usize| *(a as *const usize);
    if id == g(KEY_CLUB_COMP_FRIENDLY) || id == g(KEY_CLUB_COMP_RESERVE) {
        let sel = |o: usize| e_selecao(u(p + o)) != 0 && *((u(u(p + o) + 0x53) + 0x11c) as *const u8) != 0;
        return (sel(0x1c) || sel(0x20)) as u32;
    }
    if id == g(KEY_CLUB_COMP_B_INTERNATIONAL) || id == g(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
        return 0;
    }
    if amistoso(u(0xae2c74), comp()) != 0 {
        for s in 0..16 {
            let t = tecnico(s);
            if byte(t, 0x5f) == 1 && u(t + 0x39) != 0 && na_competicao(objeto(), u(t + 0x39)) != 0 {
                return 1;
            }
        }
        return 0;
    }
    if id == g(KEY_CLUB_COMP_INTER_TOTO_CUP) && rodada(*((p + 0x30) as *const u16) as i32) as i16 != 150 {
        return 0;
    }
    if u(comp() + 0x5d) != 0 {
        if u(0xb74434) != 0 && do_grupo(comp()) != 0 {
            return 0;
        }
        for s in 0..16 {
            let t = tecnico(s);
            if byte(t, 0x5f) == 1 && u(t + 0x39) != 0 && u(u(t + 0x39) + 0x53) == u(comp() + 0x5d) {
                let c1c = u(p + 0x1c);
                let cm = comp();
                if u(c1c + 0x57) != cm && u(c1c + 0x60) != cm {
                    return 1;
                }
                let x = u(t + 0x39);
                return (u(x + 0x57) == cm || u(x + 0x60) == cm) as u32;
            }
        }
        return 0;
    }
    let mut edi = u(comp() + 0x59);
    if edi == 0 {
        let k = if id == g(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) || id == g(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS) {
            Some(g(KEY_CONTINENT_AFRICA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION) {
            Some(g(KEY_CONTINENT_ASIA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION) {
            Some(g(KEY_CONTINENT_NORTH_AMERICA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION) || id == g(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) || id == g(KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING) {
            Some(g(KEY_CONTINENT_EUROPE))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
            Some(g(KEY_CONTINENT_OCEANIA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT) || id == g(KEY_CLUB_COMP_COPA_AMERICA) {
            Some(g(KEY_CONTINENT_SOUTH_AMERICA))
        } else {
            None
        };
        if let Some(k) = k {
            edi = (*(0xae23a4 as *const usize)).wrapping_add((k as usize).wrapping_mul(0xc6));
        }
        if edi == 0 && id != g(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
            return 1;
        }
    }
    for s in 0..16 {
        let t = tecnico(s);
        if byte(t, 0x5f) != 1 {
            continue;
        }
        if na_competicao(objeto(), principal(u(t + 0x1a))) != 0 {
            return 1;
        }
        let x = u(t + 0x39);
        if x != 0 {
            let xn = u(x + 0x53);
            if u(xn + 0x71) == edi || u(u(t + 0x1a) + 0x71) == edi {
                return 1;
            }
            if na_competicao(objeto(), principal(xn)) != 0 {
                return 1;
            }
        }
    }
    0
}

// ---------- octogesima sexta leva: desempenho do jogador na partida (0x6eecb0, ~6,4 KB) ----------
/// { 0x6eecb0 } (ret 4, match_pl.cpp, ~6,4 KB): desempenho do jogador na partida. Atualiza os
/// atributos (0x6ec010), calcula a palavra +0x37 (posicao nas tabelas taticas A e B contra os
/// bytes de +0x10 a +0x1a da ficha; com `marcar`, conta no byte +0xaf do lado do motor), os bytes
/// +0x10e..+0x117 e as palavras derivadas de +0x11a a +0x196 (63 delas ajustadas por +0x37 numa
/// tabela de casos) e sorteia os atributos do momento (0x6f44a0).
pub(crate) unsafe extern "thiscall" fn desempenho_na_partida(this: usize, marcar: u8) {
    let q = |a: usize| f64_em(a);
    let f = |a: usize| f32_em(a) as f64;
    let rd = |o: usize| *((this + o) as *const f32) as f64;
    let b = |p: usize, o: usize| *((p + o) as *const i8) as i32;
    let motor = || *((this + 0x19e) as *const usize);
    let ficha = || *((this + 0x6d) as *const usize);
    let fb = |o: usize| b(ficha(), o);
    let ta = || if motor() != 0 { tabela_a(motor(), this) } else { 0 };
    let tb = || if motor() != 0 { tabela_b(motor(), this) } else { 0 };
    let p37 = (this + 0x37) as *mut i16;
    let soma37 = move |v: i32| *p37 = (*p37).wrapping_add(v as i16);
    let palavra = |o: usize, v: i32| *((this + o) as *mut i16) = v as i16;
    let atributos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_CALC_MATCH_ATTRIBUTES);
    atributos(this);
    let bl: i8 = if b(ficha(), 0xf) > 15 { 20 } else { ftol(b(ficha(), 0xf) as f64 * q(0x969040) + q(0x9688a0)) as i8 };
    *p37 = 0;
    // a posicao na tabela A contra os atributos de +0x10..+0x1a
    let maior_13_14 = || core::cmp::max(fb(0x14) - 5, fb(0x13));
    let x: Option<f64> = if ta() & 1 != 0 {
        soma37((bl as i16 as i32) * 2 - 40);
        None
    } else if ta() & 2 != 0 {
        Some((20 - fb(0x10)) as f64)
    } else if ta() & 4 != 0 {
        Some((20 - fb(0x11)) as f64)
    } else if ta() & 8 != 0 {
        if ta() & 0x800 == 0 && ta() & 0x80 == 0 {
            Some((20 - fb(0x12)) as f64)
        } else {
            let b_880 = || tb() & 0x800 != 0 || tb() & 0x80 != 0;
            if b_880() && (tb() & 8 != 0 || tb() & 0x10 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0) {
                Some((20 - fb(0x16)) as f64)
            } else if b_880() && (tb() & 4 != 0 || tb() & 2 != 0) {
                Some((20 - fb(0x11)) as f64)
            } else {
                Some((20 - fb(0x12)) as f64)
            }
        }
    } else if ta() & 0x10 != 0 {
        Some((20 - maior_13_14()) as f64)
    } else if ta() & 0x20 != 0 {
        if ta() & 0x800 != 0 || ta() & 0x80 != 0 {
            Some((20 - fb(0x14)) as f64)
        } else {
            let lado = byte(this, 0x27) as isize;
            let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(FN_FORMATION_GET_PLAYER_INSTRUCTIONS);
            let m = marcas((motor() as isize + 0x91e2 + 0x18e3 * lado) as usize, byte(this, 0x19));
            let v = if m as u32 & 0x200000 != 0 { fb(0x1a) * fb(0x14) } else { fb(0x15) * fb(0x14) };
            Some(q(0x968ea8) - v as f64 * q(0x967888) + q(0x968968))
        }
    } else if ta() & 0x40 != 0 {
        Some((20 - fb(0x15)) as f64)
    } else {
        None
    };
    if let Some(x) = x {
        soma37(ftol(x * q(0x96d740)) as i16 as i32);
    }
    if ta() & 0x80 != 0 {
        soma37(fb(0x18) - 20);
    } else if ta() & 0x100 != 0 {
        soma37(core::cmp::max(fb(0x19), fb(0x18) / 2) - 20);
    } else if ta() & 0x200 != 0 {
        soma37(fb(0x19) - 20);
    } else if ta() & 0x400 != 0 {
        soma37(core::cmp::max(fb(0x19), fb(0x17) / 2) - 20);
    } else if ta() & 0x800 != 0 {
        soma37(fb(0x17) - 20);
    }
    if ta() & 0x1000 != 0 {
        soma37(fb(0x1a) - 20);
    }
    // a tabela B, quando difere da A
    if tb() != ta() {
        let tres_quartos = |v: i32| soma37(ftol(v as f64 * q(0x969530)) as i16 as i32);
        let b_altas = || tb() & 0x100 != 0 || tb() & 0x200 != 0 || tb() & 0x400 != 0;
        let marca = || (motor() as isize + 0xaf + byte(this, 0x27) as isize) as *mut u8;
        if tb() & 2 != 0 {
            tres_quartos(20 - fb(0x10));
        } else if tb() & 4 != 0 {
            tres_quartos(20 - fb(0x11));
        } else if tb() & 8 != 0 {
            tres_quartos(20 - fb(0x12));
        } else if tb() & 0x10 != 0 {
            tres_quartos(20 - maior_13_14());
        } else if tb() & 0x20 != 0 {
            tres_quartos(20 - fb(0x14));
            if marcar != 0 && ta() & 0x40 == 0 && ta() & 0x20 == 0 && b_altas() {
                *marca() = (*marca()).wrapping_add(1);
            }
        } else if tb() & 0x40 != 0 {
            tres_quartos(20 - fb(0x15));
            if marcar != 0 && ta() & 0x40 == 0 && b_altas() {
                *marca() = (*marca()).wrapping_add(2);
            }
        }
    }
    let meio = |v: i32| soma37(v / 2 - 10);
    if tb() & 0x80 != 0 {
        meio(fb(0x18));
    } else if tb() & 0x100 != 0 {
        meio(core::cmp::max(fb(0x19), fb(0x18) / 2));
    } else if tb() & 0x200 != 0 {
        meio(fb(0x19));
    } else if tb() & 0x400 != 0 {
        meio(core::cmp::max(fb(0x19), fb(0x17) / 2));
    } else if tb() & 0x800 != 0 {
        meio(fb(0x17));
    }
    *p37 = ftol((q(0x96d730) - rd(0xed) * q(0x96d738)) * *p37 as f64) as i16;
    let w37 = move || *p37 as i32;

    // bytes +0x10e..+0x117 (no minimo 2)
    let min2 = |v: f64, dois: f64| if v > dois { v } else { dois };
    let byte_em = |o: usize, v: f64| *((this + o) as *mut u8) = ftol(v) as u8;
    let d3 = || (w37() / 3) as f64;
    byte_em(0x10e, min2(rd(0xc1) + f(0x96d228), f(0x9689a8)));
    byte_em(0x110, min2(d3() + rd(0xcd) + f(0x96d228), f(0x9689a8)));
    byte_em(0x111, min2(d3() + rd(0xb9) * q(0x9689d8) + q(0x9688a0), q(0x968950)));
    byte_em(0x112, min2(d3() + rd(0xb5) + f(0x96d228), f(0x9689a8)));
    byte_em(0x113, min2(pow_do_jogo(byte(this, 0x17) as f64 + rd(0x9d) - f(0x968a78), q(0x968950)) * q(0x967888), q(0x968950)));
    byte_em(0x114, min2(d3() + rd(0xf1) * q(0x9689b0) + q(0x9688a0), q(0x968950)));
    byte_em(0x115, min2(d3() + rd(0x7d) + f(0x968990), f(0x9689a8)));
    byte_em(0x116, min2(rd(0xf5) + f(0x9689a8), f(0x9689a8)));
    byte_em(0x117, min2(rd(0x79) + f(0x968990), f(0x9689a8)));
    byte_em(0x10f, min2(rd(0x91) + f(0x9689a8), f(0x9689a8)));

    // palavras derivadas: T = x*10 + 0,5; Q = (x^2 + 1)/2; D = 2x*10 + 0,5 (truncados)
    let t = |o: usize| ftol(rd(o) * q(0x9688b0) + q(0x968968)) as i16 as i32;
    let qd = |o: usize| ftol((rd(o) * rd(o) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let d = |o: usize| ftol((rd(o) + rd(o)) * q(0x9688b0) + q(0x968968)) as i16 as i32;
    let blq = ftol(((bl as i32 as f64) * (bl as i32 as f64) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let v = (blq + t(0xa5) + t(0xcd) + t(0xad) + t(0x91) + t(0x7d)) / 6;
    palavra(0x11a, v);
    palavra(0x11c, v);
    palavra(0x11e, t(0xa9));
    palavra(0x120, t(0xa9));
    palavra(0x122, (t(0x81) + t(0xa9)) / 2);
    palavra(0x124, (qd(0x79) + qd(0x9d) + qd(0xe5) + qd(0x81) + 100) / 5);
    palavra(0x18c, t(0xa9));
    palavra(0x190, (qd(0x85) + qd(0x79)) / 2);
    palavra(0x18e, (d(0x99) + 300) / 5);
    palavra(0x126, t(0x8d));
    palavra(0x128, t(0x8d));
    palavra(0x12a, t(0x8d));
    palavra(0x12c, (t(0xc5) + 200) / 2);
    palavra(0x196, (t(0xc5) + 200) / 2);
    palavra(0x12e, qd(0xc5));
    palavra(0x130, (d(0xc5) + 200) / 3);
    palavra(0x132, qd(0xe5));
    palavra(0x134, t(0xe5));
    palavra(0x136, (qd(0xe5) + t(0xe5)) / 2);
    palavra(0x138, (t(0xe5) + t(0x79) + t(0x81) + t(0xd9)) / 4);
    palavra(0x13a, (t(0xe5) + t(0x81) + t(0xd9)) / 3);
    palavra(0x13c, ftol((qd(0xe5) + d(0xe5) + t(0x71)) as f64 * q(0x96d728)) as i32);
    palavra(0x13e, ftol((d(0xc1) + d(0x95) + qd(0x95) + t(0xe5)) as f64 * q(0x96d720)) as i32);
    palavra(0x142, (t(0x79) + t(0x81)) / 2);
    palavra(0x146, t(0x99));
    palavra(0x148, t(0x99));
    // o quadrado de +0xe5 fica na pilha do x87 entre as contas seguintes
    let e5q = rd(0xe5) * rd(0xe5);
    let a = ftol((e5q + q(0x967890)) * q(0x968968)) as i16 as i32;
    let t99 = t(0x99);
    palavra(0x14a, (a + t99) / 2);
    palavra(0x14c, (t(0xe5) + t99) / 2);
    let bq = ftol(((e5q + e5q) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let d99 = d(0x99);
    let tc5 = t(0xc5);
    palavra(0x150, (bq + d99 + tc5) / 5);
    palavra(0x14e, (bq + d99 + tc5) / 5);
    palavra(0x152, (qd(0x9d) + d99 + bq + tc5) / 6);
    palavra(0x154, (t(0xd9) + t99) / 2);
    palavra(0x156, qd(0xc5));
    palavra(0x192, qd(0xc5));
    palavra(0x158, (a + tc5) / 2);
    palavra(0x15a, (a + tc5) / 2);
    palavra(0x194, t(0xa9));
    let ddq = rd(0xdd) * rd(0xdd);
    let c = ftol(((ddq + ddq) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let t81 = t(0x81);
    let t7d = t(0x7d);
    palavra(0x15c, (t81 + t7d + c) / 4);
    palavra(0x140, (qd(0xc1) + t81 + t7d + c) / 5);
    palavra(0x15e, ftol((ddq + q(0x967890)) * q(0x968968)) as i32);
    palavra(0x160, ftol(qd(0x85) as f64 * q(0x96d718)) as i32);
    let t79 = t(0x79);
    palavra(0x162, ftol((t7d + t79) as f64 * q(0x968e00)) as i32);
    palavra(0x166, (qd(0xd9) + t81) / 2);
    let bq2 = ftol(((bl as i32 * bl as i32) as f64 + q(0x967890)) * q(0x968968)) as i16 as i32;
    let ba5 = bq2 + t(0xa5);
    palavra(0x168, (ba5 + t79) / 3);
    palavra(0x16a, (ba5 + t79) / 3);
    palavra(0x16c, (t(0x85) + ba5) / 3);
    palavra(0x16e, 200);
    let dd9 = d(0xd9);
    palavra(0x178, 200);
    palavra(0x172, 400);
    palavra(0x174, 400);
    palavra(0x170, (dd9 + 400) / 2);
    palavra(0x176, (dd9 + 400) / 2);
    palavra(0x17a, 200);
    palavra(0x17c, 200);
    palavra(0x17e, 200);
    palavra(0x180, (t(0xc9) + 200) / 2);
    palavra(0x182, qd(0x89));
    palavra(0x184, qd(0xa1));
    palavra(0x144, qd(0xad));
    palavra(0x186, t(0xe9));
    palavra(0x188, qd(0xa1));
    palavra(0x164, 200);
    palavra(0x18a, (qd(0x81) + qd(0xd9)) / 2);

    // as 63 palavras de +0x11a ajustadas por +0x37 (tabela de casos do original)
    const CASOS: [u8; 63] = [
        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 3, 1, 1, 1, 4, 4, 4, 4, 1, 5, 6, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 7, 7, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 2, 5, 5, 5, 2, 1, 6, 3, 3, 1,
    ];
    for (k, &caso) in CASOS.iter().enumerate() {
        let p = (this + 0x11a + 2 * k) as *mut i16;
        let w = *p as f64;
        let w37f = w37();
        let v = match caso {
            0 => ((w37f + 25) * 10) as f64 + w * q(0x968e18),
            1 => ((w37f + 45) * 5) as f64 + w * q(0x968968),
            2 => ((w37f * 3) as f64 + f(0x96d710)) + w * q(0x96d708),
            3 => w * q(0x96d700) + w37f as f64 * q(0x968e18) + q(0x96d520),
            4 => w * q(0x968ec8) + w37f as f64 * q(0x968e18) + q(0x96d520),
            5 => w * q(0x96d640) + q(0x968ff8),
            6 => w * q(0x968e38),
            _ => ((w37f + 30) * 7) as f64 + w * q(0x96a6e8),
        };
        *p = ftol(v) as i16;
    }
    let momento: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_ROLL_CURRENT_RATINGS);
    momento(this);
}

// ---------- octogesima quinta leva: atributos do jogador na partida (0x6ec010, ~11 KB) ----------
/// recomputes word +0x3b and the 34 f32 fields from +0x71 to +0xf5 of the player: the designated player's rating
/// (0x6ebd70, slot 0x897590), age, days alive, contract (0x4cf5f0, +0x45 bits, +0x10/+0x14 history), morale, tactical
/// tables A/B (0x6c38c0/0x6c38f0), formation (0x6d5b20/0x6d5af0), managers and preferences
/// (0x547810/0x547840/0x547d30/0x547d60), powers (0x945ff0) and random draws. Only the player record changes
/// Original: `update_attributes` (0x6ec010, match_pl.cpp, thiscall).
///
/// [pt] { 0x6ec010 } (match_pl.cpp, ~11 KB): atributos do jogador na partida (campos f32 de +0x7d a
/// +0xf5 e a palavra +0x3b), pela nota do designado, idade, contrato, moral, cansaco e taticas.
pub(crate) unsafe extern "thiscall" fn atualizar_atributos(this: usize) {
    let q = |a: usize| f64_em(a);
    let f = |a: usize| f32_em(a) as f64;
    let rd = |o: usize| *((this + o) as *const f32) as f64;
    let wr = |o: usize, v: f64| *((this + o) as *mut f32) = v as f32;
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let b = |p: usize, o: usize| *((p + o) as *const i8) as i32;
    let pessoa = || *((this + 0x69) as *const usize);
    let ficha = || *((this + 0x6d) as *const usize);
    let motor = || *((this + 0x19e) as *const usize);
    let ta = || if motor() != 0 { tabela_a(motor(), this) } else { 0 };
    let tb = || if motor() != 0 { tabela_b(motor(), this) } else { 0 };
    let n_pessoas = || *(0xae2364 as *const i32);
    let clube_lado = || {
        let t = *((motor() + 0x4792) as *const usize);
        *((t + if byte(this, 0x27) != 0 { 0x20 } else { 0x1c }) as *const usize)
    };
    let nota: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_CALC_INFLUENCE_SCORE);
    let nota = |j: usize| nota(j) as i8 as i32;
    let vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
    let designado = |m: usize| {
        let t = *((m + 0x4792) as *const usize);
        let l = byte(this, 0x27);
        let (c, fl) = if l != 0 { (*((t + 0x20) as *const usize), (byte(t, 0x41) == 0) as u32) } else { (*((t + 0x1c) as *const usize), (byte(t, 0x40) == 0) as u32) };
        let k = vaga(*(0xde1d08 as *const usize), c, fl, 0) as i32;
        (m as isize + 0x4796 + 0x1be * (k + 20 * l as i32) as isize) as usize
    };
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let nivel: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_CONTRACT_RATING_MODIFIER);

    let (mut l28, mut l18, mut l24, mut l1c, mut l14) = (0f32, 0f32, 0f32, 0f32, 0f32);
    let l12 = byte(pessoa(), 0x18) as i32;

    // o jogador designado (vaga marcada da tatica do lado)
    let j = designado(motor());
    if j != 0 && *((j + 0x6d) as *const usize) != 0 {
        l18 = ((nota(j) as f64 - q(0x968e08)) * q(0x96d6f8)) as f32;
        let j2 = designado(motor());
        let fj = *((j + 0x6d) as *const usize);
        let x = (b(fj, 0x39) as f64 + b(fj, 0x39) as f64 + w(*((j2 + 0x6d) as *const usize), 5) as f64) * q(0x967880);
        let l34 = if x < q(0x9689a0) { 0.0 } else if !(x <= q(0x968ea8)) { 20.0 } else { x };
        l28 = (nota(j) as f64 * l34 * q(0x968ef0)) as f32;
        let mj = *((j + 0x19e) as *const usize);
        l24 = if mj != 0 && tabela_a(mj, j) == 1 {
            (nota(j) - 20) as f64 * q(0x968968)
        } else {
            ((nota(j) * b(*((j + 0x6d) as *const usize), 0x44)) as f64 * q(0x967888) - q(0x968e08)) * q(0x968968)
        } as f32;
        let idade = b(*((j + 0x69) as *const usize), 0x18);
        let m = |v: f32, c: usize| (v as f64 * f(c)) as f32;
        if idade < 18 {
            l18 = m(l18, 0x968f18);
            l24 = m(l24, 0x96d248);
            l28 = m(l28, 0x96d248);
        } else if idade < 21 {
            l18 = m(l18, 0x96d248);
            l24 = m(l24, 0x96d364);
            l28 = m(l28, 0x96d364);
        } else if idade < 24 {
            l18 = m(l18, 0x96a32c);
            l24 = m(l24, 0x96a32c);
            l28 = m(l28, 0x96a32c);
        } else if idade > 30 && idade < 36 {
            l18 = m(l18, 0x96d3b8);
        }
        let pj = *((j + 0x69) as *const usize);
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        let ler = |o: usize| core::ptr::read_unaligned((pj + o) as *const u32);
        if dias(0xae2c90, ler(0x3e), ler(0x42)) < (50 - b(pj, 0x59)) * 10 {
            l18 = m(l18, 0x96a32c);
        }
        if b(pj, 0x5b) < 10 {
            l18 = ((b(pj, 0x5b) as f64 * f(0x96bac8) + f(0x96a32c)) * l18 as f64) as f32;
        }
    }
    let l2c = if e_selecao(clube_lado()) == 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        contrato(*(0xae1050 as *const usize), pessoa(), clube_lado())
    } else {
        0
    };
    let lado = byte(this, 0x27) as i32;
    l18 = (((w(motor(), (0x1ce + 2 * lado) as usize) - 100) as f64 * q(0x96d260) + q(0x967890)) * l18 as f64) as f32;
    let p = pessoa();
    let maior = || core::cmp::max(byte(p, 0x5a), byte(p, 0x58)) as i32;
    let w33 = w(this, 0x33);
    let mut bl: i8 = if w33 > 10 { (w33 / 4) as i8 } else { (((10 - w33) * (maior() - 10)) / 10) as i8 };
    let w31 = w(this, 0x31);
    let dl = if w31 < 0 { ((maior() - 10) * w31) / -10 } else { core::cmp::min(((b(p, 0x5b) - 10) * w31) / 10, 10) };
    bl = bl.wrapping_add(dl as i8);
    bl = if ta() == 1 { bl.clamp(-5, 15) } else { bl.clamp(-10, 10) };
    let b1ad = byte(motor(), (0x1ad + lado) as usize);
    if b1ad != 0 {
        let t = *((motor() + 0x4792) as *const usize);
        let al = b1ad as i32;
        let base = |c: usize| {
            (al as f64 + q(0x9688b0)) * f(0x96d490) + q(c) - (b(p, 0x5b) - 10) as f64 * q(0x96a7a8)
                - (b(ficha(), 0x2c) - 10) as f64 * q(0x9689e0) - l18 as f64 * q(0x968968)
        };
        let x = if t != 0 && (byte(t, 0x3a) == 8 || byte(t, 0x3a) == 10) {
            base(0x968e38)
        } else {
            let mut s = f(0x96d6f4);
            if byte(motor(), 0) == 0 {
                let wt = w(*((t + 0x14) as *const usize), 0x69);
                if wt > 12 {
                    s = (wt - 10) as f64 * f(0x96d6f0) + f(0x96d6f4);
                }
            }
            base(0x968e18) / s
        };
        l1c = (((al / 2) as f64 + q(0x968e18) + x) * 2.0) as f32;
        if ta() == 1 {
            l1c = (l1c as f64 * f(0x96d6ec)) as f32;
        }
        if !(l1c as f64 >= q(0x967890)) {
            l1c = 1.0;
        }
    }
    let l30 = b(ficha(), 0x21) as f32;
    let tabela79 = |id: i32| *((*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79)).wrapping_add(0xa) as *const u8) as i32;
    let teto30 = |v: i32| if v as f64 > q(0x968dd8) { v as f64 } else { q(0x968dd8) };
    if p != 0 && *(p as *const i32) >= n_pessoas() {
        let v = w(ficha(), 5) as f64 + l18 as f64 * q(0x968f98) - l1c as f64;
        *((this + 0x3b) as *mut i16) = ftol(v) as i16;
        let h = (nivel(p, clube_lado()) as i8 as i32) / 2;
        let a = h as f64 + (l30 as f64 - l1c as f64);
        if (byte(this, 0x118) as f64) < a {
            l14 = (byte(this, 0x119) as f64 * teto30(w(this, 0x3b)) * f(0x96d6e8)) as f32;
        }
    } else {
        let id = *(p as *const i32);
        let v = w(ficha(), 5) as f64 + l18 as f64 * q(0x968f98) - l1c as f64 - tabela79(id) as f64 + (byte(this, 0x100) as i32 / 2) as f64;
        *((this + 0x3b) as *mut i16) = ftol(v) as i16;
        let h = (nivel(p, clube_lado()) as i8 as i32) / 2;
        let h17 = byte(this, 0x17) as i32 / 2 - 5;
        let a = (h as f64 + h17 as f64) + (l30 as f64 - l1c as f64) + bl as f64 + byte(this, 0x100) as f64;
        if (byte(this, 0x118) as f64) > a {
            l14 = (byte(this, 0x119) as f64 * teto30(w(this, 0x3b)) * f(0x96d6e8)) as f32;
        }
    }
    if e_selecao(*((this + 0xf9) as *const usize)) != 0 && ta() != 1 {
        l14 = (l14 as f64 + f(0x968990)) as f32;
    }
    *((this + 0x3b) as *mut i16) = ftol(((byte(this, 0x17) as i32 - 10) * byte(this, 0x118) as i32) as f64 * q(0x969020) + w(this, 0x3b) as f64) as i16;
    // bonus de juventude (+0x14 do quadro tatico, casa +0x69) em L14
    {
        let t = *((motor() + 0x4792) as *const usize);
        if t != 0 {
            let t14 = *((t + 0x14) as *const usize);
            if t14 != 0 && byte(t, 0x3a) != 6 {
                let ax = w(t14, 0x69);
                let pv = pessoa();
                if ax != 0 && !(pv != 0 && *(pv as *const i32) >= n_pessoas()) && (byte(pv, 0x18) as i32) < 24 {
                    let idade = byte(pv, 0x18) as i32;
                    let dx = ax.wrapping_mul(500) as i16 as i32;
                    let mut e = (byte(pv, 0x5a) as i32).wrapping_mul(150);
                    if byte(ficha(), 0xf) != 20 {
                        if idade > 21 {
                            e += 750;
                        } else if idade > 18 {
                            e += 250;
                        }
                    }
                    let ecx = e as i16 as i32;
                    let d = dx - ecx;
                    if d > 4500 && w(ficha(), 0xd) < d {
                        let edi = w(ficha(), 0xd) - dx + ecx;
                        let add = if ta() == 1 {
                            if edi <= 2500 { f(0x96b008) } else { f(0x968990) }
                        } else if edi > 2500 {
                            f(0x969078)
                        } else if edi > 1250 {
                            f(0x968990)
                        } else {
                            f(0x96b008)
                        };
                        l14 = (l14 as f64 + add) as f32;
                    }
                }
            }
        }
    }
    let p3b = (this + 0x3b) as *mut i16;
    if ta() == 1 {
        let v = ftol(w(ficha(), 5) as f64 * q(0x969040)) as i16;
        if *p3b < v {
            *p3b = v;
        }
        let pv = pessoa();
        if (pv == 0 || *(pv as *const i32) < n_pessoas()) && *p3b < 8 {
            *p3b = 8;
        }
    }
    let lim = if ta() == 1 { (w(ficha(), 7) as i16).wrapping_add(10) } else { w(ficha(), 7) as i16 };
    if *p3b > lim {
        *p3b = lim;
    }
    *p3b = ftol(*p3b as f64 * q(0x968968) + q(0x96d6e0)) as i16;
    let maior150 = *p3b > 150;
    let w3 = || *p3b as f64;
    let zera = |o: usize| *((this + o) as *mut u32) = 0;
    // fst + comparacao com o valor ainda nao arredondado
    let guarda = |o: usize, v: f64| {
        wr(o, v);
        // fcomp + test ah,1: NaN conta como menor (C0 ligado), como na original
        if !(v >= q(0x9689a0)) {
            zera(o);
        }
    };
    let pow = |o: usize, limiar: f64, e: f64| wr(o, pow_do_jogo(rd(o) - limiar, e) + limiar);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let jx = |l: i32| *((motor() as isize + 0x904d + 208 * l as isize) as *const usize);
    let do_banco = || {
        // o tecnico do lado (+0x904d) e a pessoa dele, quando o clube nao e humano
        if humano(clube_lado(), 0, 1, 0, 0) != 0 {
            return 0;
        }
        let x = jx(byte(this, 0x27) as i32);
        if x == 0 { 0 } else { *((x + 0x69) as *const usize) }
    };
    let t5b20: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_SIDE_HAS_NO_WIDE_DEFENDERS);
    let t5af0: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_SIDE_HAS_NO_WIDE_MIDFIELD_OR_ATTACK);
    let par_880 = || ta() & 0x880 != 0 && tb() & 0x880 != 0;
    let forma_b20 = || t5b20(motor(), (byte(this, 0x27) == 0) as u8) != 0 && par_880() && (ta() & 0x60 != 0 || tb() & 0x60 != 0);
    let forma_af0 = || t5af0(motor(), byte(this, 0x27) as u8) != 0 && (ta() & 0x78 != 0 || tb() & 0x78 != 0);
    let a_20_0c = || ta() & 0x20 != 0 && ta() & 0xc != 0;
    let fb = |o: usize| b(ficha(), o) as f64;
    let l12f = l12;

    // +0x7d
    let e1e = if l12f > 36 { fb(0x1e) * q(0x969040) } else { fb(0x1e) };
    let l4c = l14 as f64 * q(0x9689b0);
    wr(0x7d, ((e1e + e1e) + w3() - l4c) * q(0x967880));
    let pj = do_banco();
    if pj != 0 {
        let k = chance(pj, byte(pj, 0x21)) as i8 as i32 - 13;
        wr(0x7d, k as f64 * q(0x967880) + rd(0x7d));
    } else {
        wr(0x7d, rd(0x7d) + q(0x96d6d8));
    }
    if !(rd(0x7d) >= q(0x9689a0)) {
        zera(0x7d);
    }
    if a_20_0c() {
        wr(0x7d, rd(0x7d) * q(0x9689d0));
    }
    wr(0x7d, rd(0x7d) * q(0x968968) + q(0x9688a0));
    // +0x8d
    let l3c = l14 as f64 * q(0x968e48);
    wr(0x8d, (fb(0x23) + fb(0x23) + w3() - l3c) * q(0x967880));
    if forma_b20() {
        wr(0x8d, rd(0x8d) * q(0x968fa0));
    }
    if !(rd(0x8d) >= q(0x9689a0)) {
        zera(0x8d);
    }
    if rd(0x8d) > q(0x969508) && maior150 {
        pow(0x8d, q(0x969508), q(0x968928));
    }
    // +0x91, +0x95, +0x99
    guarda(0x91, (fb(0x24) + fb(0x24) + w3() - l14 as f64) * q(0x967880));
    let l54 = l14 as f64 * q(0x969510);
    wr(0x95, (fb(0x26) + fb(0x26) + w3() - l54) * q(0x967880));
    if a_20_0c() {
        wr(0x95, rd(0x95) * q(0x9689d0));
    }
    if !(rd(0x95) >= q(0x9689a0)) {
        zera(0x95);
    }
    let l44 = l14 as f64 * q(0x968f80);
    guarda(0x99, (fb(0x27) + fb(0x27) + w3() - l44) * q(0x967880));
    if par_880() {
        wr(0x99, rd(0x99) * q(0x96a720));
    }
    if a_20_0c() {
        wr(0x99, rd(0x99) * q(0x9689d0));
    }
    if rd(0x99) > q(0x969508) && maior150 && e_selecao(*((this + 0xf9) as *const usize)) == 0 {
        pow(0x99, q(0x969508), q(0x968928));
    }
    // +0xa5, +0xa9, +0xb1, +0xb5
    guarda(0xa5, (fb(0x2a) + fb(0x2a) + w3() - l14 as f64) * q(0x967880));
    guarda(0xa9, (fb(0x2b) + fb(0x2b) + w3() - l3c) * q(0x967880));
    if rd(0xa9) > q(0x969550) && maior150 {
        pow(0xa9, q(0x969550), q(0x968978));
    }
    guarda(0xb1, (fb(0x31) + fb(0x31) + w3() - l14 as f64) * q(0x967880));
    let l5c = l14 as f64 * q(0x968968);
    guarda(0xb5, (fb(0x32) + fb(0x32) + w3() - l5c) * q(0x967880));
    let t1c = (pow_do_jogo(rd(0xb5), q(0x96d6d0)) * q(0x967888) + q(0x967890)) as f32;
    wr(0xb5, t1c as f64);
    if t1c as f64 > q(0x969508) && maior150 {
        wr(0xb5, pow_do_jogo(t1c as f64 - q(0x969508), q(0x968978)) + q(0x969508));
    }
    // L18 pelo byte +0x33
    let b33 = fb(0x33);
    l18 = (if l12f > 36 {
        b33 * q(0x969040)
    } else if ta() & 0x40 != 0 {
        b33
    } else if ta() & 0x20 != 0 {
        b33 * q(0x9689d0)
    } else if ta() & 0x18 != 0 {
        if byte(ficha(), 0x12) == 20 { b33 * q(0x969520) } else { b33 * q(0x9689d8) }
    } else {
        b33 * q(0x9689b0)
    }) as f32;
    if par_880() {
        if ta() & 0x30 != 0 {
            l18 = (l18 as f64 * q(0x96d6c8)) as f32;
        } else if ta() & 8 != 0 || tb() & 8 != 0 {
            l18 = (l18 as f64 * q(0x968fa0)) as f32;
        }
    }
    if forma_b20() {
        l18 = (l18 as f64 * q(0x968fa0)) as f32;
    }
    if forma_af0() {
        l18 = (l18 as f64 * q(0x96af30)) as f32;
    }
    // +0xb9
    wr(0xb9, (l18 as f64 + l18 as f64 + w3() - l4c) * q(0x967880));
    let pj = do_banco();
    if pj != 0 {
        let m = (b(pj, 0xe) as f64 * f(0x96d6c4) + f(0x96a340)) as f32;
        let k = chance(pj, byte(pj, 0x21)) as i8 as i32 - 13;
        wr(0xb9, k as f64 * m as f64 + rd(0xb9));
    } else {
        wr(0xb9, rd(0xb9) + q(0x968fd0));
    }
    guarda(0xb9, rd(0xb9) * f(0x96d320) + f(0x96d6c0));
    // +0xbd, +0xc5, +0xc9
    guarda(0xbd, (fb(0x35) + fb(0x35) + w3() - l14 as f64) * q(0x967880));
    guarda(0xc5, (w3() + (fb(0x37) + fb(0x37)) - l54) * q(0x967880));
    if forma_b20() {
        wr(0xc5, rd(0xc5) * q(0x96d6b8));
    }
    if rd(0xc5) > q(0x969550) && maior150 {
        pow(0xc5, q(0x969550), q(0x968928));
    }
    guarda(0xc9, (fb(0x38) + fb(0x38) + w3() - l5c) * q(0x967880));
    // L1c pelo byte +0x39
    let b39 = fb(0x39);
    l1c = (if l12f > 36 { b39 * q(0x969040) } else { b39 }) as f32;
    if par_880() {
        if ta() & 4 != 0 {
            l1c = (l1c as f64 * q(0x96d6c8)) as f32;
        } else if ta() & 8 != 0 {
            l1c = (l1c as f64 * q(0x96d6b0)) as f32;
        } else if ta() & 0x10 != 0 || tb() & 0xc != 0 {
            // a original usa o L18 aqui
            l1c = (l18 as f64 * q(0x968fa0)) as f32;
        }
    }
    if ta() == 1 {
        if l12f < 21 || w(ficha(), 0xd) < 3750 {
            l1c = (l1c as f64 * q(0x96af30)) as f32;
        } else if l12f < 23 {
            l1c = (l1c as f64 * q(0x96a6d8)) as f32;
        }
    }
    if (ta() & 4 != 0 || tb() & 4 != 0) && tb() != ta() {
        let pos = |x: u16| -> i32 {
            if x & 1 != 0 {
                0
            } else if x & 4 != 0 {
                1
            } else if x & 8 != 0 {
                2
            } else if x & 0x10 != 0 {
                3
            } else if x & 0x20 != 0 {
                4
            } else if x & 0x40 != 0 {
                5
            } else {
                3
            }
        };
        let d = (pos(ta()) - pos(tb())).abs();
        l1c = (l1c as f64 * if d as i8 > 1 { q(0x96a678) } else { q(0x9689d0) }) as f32;
    }
    // +0xcd
    wr(0xcd, (l1c as f64 + l1c as f64 + w3() - l3c + l28 as f64) * q(0x967880));
    let pj = do_banco();
    if pj != 0 {
        l28 = (f(0x968f18) - b(pj, 0xe) as f64 * f(0x96d6c4)) as f32;
        let k = chance(pj, byte(pj, 0x21)) as i8 as i32 - 13;
        wr(0xcd, k as f64 * l28 as f64 + rd(0xcd));
    } else {
        wr(0xcd, rd(0xcd) + q(0x968fd0));
    }
    guarda(0xcd, pow_do_jogo(rd(0xcd), q(0x96d6d0)) * q(0x967888) + q(0x967890));
    if rd(0xcd) > q(0x969508) && maior150 && ta() != 1 {
        pow(0xcd, q(0x969508), q(0x968978));
    }
    // L28 pelo byte +0x3a; +0xd1, +0xdd, +0xe9
    let b3a = fb(0x3a);
    l28 = (if l12f > 39 { b3a * q(0x9689b0) } else if l12f > 37 { b3a * q(0x9689d8) } else { b3a }) as f32;
    guarda(0xd1, (l28 as f64 + l28 as f64 + w3() - l14 as f64) * q(0x967880));
    guarda(0xdd, (fb(0x3e) + fb(0x3e) + w3() - l44) * q(0x967880));
    guarda(0xe9, (fb(0x41) + fb(0x41) + w3()) * q(0x967880));
    // L28 pelo byte +0x43 (e a palavra +5); +0xf1
    let b43 = b(ficha(), 0x43);
    if b43 > 10 {
        let w5 = w(ficha(), 5);
        if w5 < 80 {
            l28 = ((b43 - 10) as f64 * q(0x969040) + q(0x9688b0)) as f32;
        } else if w5 < 125 {
            l28 = ((b43 - 10) as f64 * q(0x969520) + q(0x9688b0)) as f32;
        }
    } else {
        l28 = b43 as f32;
    }
    let x = if forma_af0() { l28 as f64 * q(0x96af30) } else { l28 as f64 };
    guarda(0xf1, (x + x + w3() - l44) * q(0x967880));
    if rd(0xf1) > q(0x96d4e8) && maior150 {
        pow(0xf1, q(0x96d4e8), q(0x968978));
    }
    // +0x71 e companhia (bytes da ficha), limitados pelo tipo do motor (+1: 1, 8, 32)
    let m1 = byte(motor(), 1) as i32;
    let s = byte(ficha(), 0x44).wrapping_add(byte(ficha(), 0x3d));
    let mut cl = byte(ficha(), 0x1b);
    if m1 == 1 || m1 == 8 || m1 == 32 {
        if s == 40 {
            if cl == 20 {
                cl = 19;
            }
        } else if s > 30 {
            cl = cl.min(18);
        } else if s > 25 {
            cl = cl.min(17);
        } else if s < 15 {
            cl = cl.min(16);
        } else {
            cl = cl.min(15);
        }
    }
    let v71 = cl as f64 * q(0x96a670) + q(0x96d6a8);
    wr(0x71, v71);
    if v71 > q(0x96d6a0) {
        wr(0x71, pow_do_jogo(v71 as f32 as f64 - q(0x96d6a0), q(0x968928)) + q(0x96d6a0));
    }
    let b1d = fb(0x1d);
    let v1d = if l12f > 39 { b1d * q(0x9689b0) } else if l12f > 37 { b1d * q(0x9689d8) } else { b1d };
    wr(0x79, v1d * q(0x968968) + q(0x96d6a8));
    wr(0x75, fb(0x1c) * q(0x968968));
    wr(0x81, fb(0x1f) * q(0x968968) + q(0x96d6a8));
    wr(0x85, fb(0x20) * q(0x968968) + q(0x96d6a8));
    wr(0x89, fb(0x22) * q(0x968968) + q(0x96d6a8));
    let b28 = fb(0x28);
    let v28 = if l12f > 36 { b28 * q(0x968968) } else if l12f > 34 { b28 * q(0x969040) } else { b28 };
    wr(0x9d, v28 - l14 as f64 * f(0x96afc4));
    wr(0xa1, fb(0x29) * q(0x96d698) + q(0x96d6a8));
    wr(0xad, fb(0x2e) - l14 as f64 * f(0x96d694));
    let mut al = byte(ficha(), 0x36);
    if m1 == 32 {
        if s == 40 {
            if al == 20 {
                al = 19;
            }
        } else if s > 35 {
            al = al.min(18);
        } else if s > 30 {
            al = al.min(17);
        } else if s < 20 {
            al = al.min(16);
        } else {
            al = al.min(14);
        }
    } else if m1 == 1 || m1 == 8 {
        if s == 40 {
            if al == 20 {
                al = 19;
            }
        } else if s > 30 {
            al = al.min(18);
        } else if s > 25 {
            al = al.min(17);
        } else if s < 15 {
            al = al.min(16);
        } else {
            al = al.min(15);
        }
    }
    let vc1 = al as f64 * q(0x96a670) + q(0x96d6a8);
    wr(0xc1, vc1);
    if vc1 > q(0x96d6a0) {
        wr(0xc1, pow_do_jogo(vc1 as f32 as f64 - f(0x96d690), q(0x968928)) + q(0x96d6a0));
    }
    wr(0xd9, fb(0x3d) * q(0x968968) + q(0x96d6a8));
    let soma_3c = if l12f > 38 {
        q(0x968ec8)
    } else if l12f > 35 {
        q(0x96d688)
    } else if l12f > 33 {
        q(0x96d680)
    } else if l12f < 20 {
        q(0x96d678)
    } else if l12f < 24 {
        q(0x96d670)
    } else if l12f < 30 {
        q(0x968eb0)
    } else {
        q(0x968988)
    };
    wr(0xd5, fb(0x3c) * q(0x96a5d0) + soma_3c);
    match m1 {
        1 => wr(0xd5, rd(0xd5) - q(0x968968)),
        8 => wr(0xd5, rd(0xd5) - q(0x969040)),
        32 => wr(0xd5, rd(0xd5) - q(0x9689e0)),
        _ => {}
    }
    let t = *((motor() + 0x4792) as *const usize);
    if l12f < 32 && t != 0 {
        let c1c = *((t + 0x1c) as *const usize);
        if c1c != 0 {
            let c57 = *((c1c + 0x57) as *const usize);
            if c57 != 0 && *((t + 0x14) as *const usize) == c57 {
                let ax = w(c57, 0x69);
                if ax < 5 {
                    wr(0xd5, rd(0xd5) + q(0x968928));
                } else if ax < 9 {
                    wr(0xd5, rd(0xd5) + q(0x967890));
                }
            }
        }
    }
    wr(0xe1, fb(0x3f));
    let mut a40 = byte(ficha(), 0x40);
    match m1 {
        1 => a40 = if a40 == 20 { 19 } else { a40.min(18) },
        8 => a40 = a40.min(17),
        32 => a40 = a40.min(15),
        _ => {}
    }
    wr(0xe5, a40 as f64);
    wr(0xed, fb(0x42));
    if ta() != 1 && t != 0 {
        let t14 = *((t + 0x14) as *const usize);
        if t14 != 0 && *(t14 as *const u32) == *(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as *const u32) {
            let pn = *((pessoa() + 0x1a) as *const usize);
            if pn != 0 {
                let id = *(pn as *const u32);
                let g = |a: usize| *(a as *const u32);
                // a original so poupa a nacao igual as tres globais ao mesmo tempo
                if !(id == g(KEY_NATION_UNITED_STATES) && id == g(KEY_NATION_CANADA) && id == g(KEY_NATION_MEXICO)) {
                    wr(0xf5, rd(0xf5) * f(0x96a32c));
                }
            }
            wr(0x9d, rd(0x9d) * f(0x96d66c));
            wr(0xf1, rd(0xf1) * f(0x96d66c));
            wr(0xb9, rd(0xb9) * f(0x96a32c));
        }
    }
    if ta() != 1 {
        let fi = ficha();
        if byte(fi, 0xf) == 20 && byte(fi, 0x11) < 10 && byte(fi, 0x12) < 10 && byte(fi, 0x13) < 15 && byte(fi, 0x14) < 10 && byte(fi, 0x15) < 10 {
            for (o, c) in [(0x9d, 0x96d424), (0xf1, 0x96d22c), (0x95, 0x96d22c), (0x91, 0x96d22c), (0x7d, 0x96d22c), (0xad, 0x96d364), (0xa9, 0x96d248), (0xcd, 0x968f18), (0xc5, 0x968f18), (0xdd, 0x96d248), (0xb9, 0x96d248)] {
                wr(o, rd(o) * f(c));
            }
        }
    }
    // +0xf5
    let b44 = fb(0x44);
    let v44 = if l12f < 36 { b44 } else { b44 * q(0x969040) };
    let h = (b(pessoa(), 0x5b) / 2 + 10) as f64;
    wr(0xf5, (byte(this, 0x100) as f64 + v44) - l14 as f64 / h + l24 as f64 * q(0x968968));
    let p = pessoa();
    if p != 0 && *(p as *const i32) >= n_pessoas() {
        return;
    }
    let mul = |o: usize, c: usize| wr(o, rd(o) * f(c));
    if tabela79(*(p as *const i32)) > 8 && b(p, 0x5b) < 15 {
        for (o, c) in [(0x7d, 0x96b014), (0xf5, 0x96b014), (0xf1, 0x96a32c), (0x9d, 0x96a32c), (0x95, 0x96a32c), (0xc1, 0x96b014)] {
            mul(o, c);
        }
    }
    if l2c != 0 {
        let c = l2c;
        let c45 = || core::ptr::read_unaligned((c + 0x45) as *const u32);
        if c45() & 0x40000 != 0 && b(p, 0x5a) + l12f < sortear(50) {
            for (o, k) in [(0xf5, 0x96b014), (0x9d, 0x96a32c), (0xf1, 0x96a32c), (0x95, 0x96b014), (0x91, 0x96b014), (0xc1, 0x96b014)] {
                mul(o, k);
            }
        }
        let pa61 = *((p + 0x61) as *const usize);
        if pa61 != 0 && byte(c, 0x44) < -25 && byte(pa61, 0x44) < 15 && b(p, 0x5b) < 10 && w(pa61, 0xb) > 3750 && w(pa61, 0xb) > w(clube_lado(), 0x80) + 250 {
            mul(0xf5, 0x96b014);
            mul(0x7d, 0x96a32c);
            if byte(c, 0x44) < -75 {
                for (o, k) in [(0xf1, 0x96a32c), (0x9d, 0x96a32c), (0x95, 0x96b014), (0x99, 0x96a32c), (0xc1, 0x96b014)] {
                    mul(o, k);
                }
            }
        }
        if byte(c, 0x45) as u8 & 0x17 != 0 {
            if b(p, 0x5b) > 15 && b(p, 0x57) > 15 {
                mul(0xf5, 0x96a33c);
            } else if b(p, 0x5b) < 5 {
                mul(0xf5, 0x96a32c);
            }
        }
        if c45() & 0x2088 != 0 {
            let w0b = w(*((p + 0x61) as *const usize), 0xb);
            if (w0b > 4250 && b(p, 0x5b) < 10) || b(p, 0x5b) < 5 {
                mul(0xf5, 0x96a32c);
            }
        }
        if c45() & 0x38da00 != 0 {
            if b(p, 0x5b) == 1 {
                mul(0xf5, 0x96d38c);
            } else if b(p, 0x5b) < 6 {
                mul(0xf5, 0x96b014);
            }
        }
        let fraco = || b(p, 0x5a) < 5 && b(p, 0x5b) < 6 && b(p, 0x5d) < 10 && w(*((p + 0x61) as *const usize), 0xd) > 5250;
        let forte = [(0xf5, 0x96d38c), (0x9d, 0x96a32c), (0xf1, 0x96a32c), (0x95, 0x96a32c), (0x91, 0x96d38c), (0xc1, 0x96a32c)];
        let sorte = [(0xf5, 0x96b014), (0x91, 0x96b014), (0x9d, 0x96a32c), (0xf1, 0x96a32c), (0x95, 0x96a32c), (0xc1, 0x96b014)];
        if c45() & 0x20000 != 0 {
            if fraco() {
                for (o, k) in forte {
                    mul(o, k);
                }
            } else if b(p, 0x5a) < sortear(20) {
                for (o, k) in sorte {
                    mul(o, k);
                }
            }
        }
        if byte(c, 0x45) as u8 & 0x40 != 0 {
            if fraco() {
                for (o, k) in forte {
                    mul(o, k);
                }
            } else if b(p, 0x5a) < sortear(25) {
                for (o, k) in sorte {
                    mul(o, k);
                }
            }
        }
    }
    // preferencias da pessoa (clubes do adversario e do proprio lado, tecnicos, designado)
    let t = *((motor() + 0x4792) as *const usize);
    if byte(t, 0x40) == 0 {
        return;
    }
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let nao_gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let menos = |o: usize, k: i32, c: usize| wr(o, rd(o) - k as f64 * f(c));
    let clube_outro = || {
        let t = *((motor() + 0x4792) as *const usize);
        *((t + if byte(this, 0x27) == 0 { 0x20 } else { 0x1c }) as *const usize)
    };
    let k25 = || 25 - b(pessoa(), 0x5b);
    let k50 = || 50 - b(pessoa(), 0x5b) - b(pessoa(), 0x59);
    if nao_gosta(pessoa(), clube_outro()) != 0 {
        wr(0xf5, rd(0xf5) + f(0x968990));
        wr(0x75, k25() as f64 * f(0x96d668) + rd(0x75));
    } else if gosta(pessoa(), clube_outro()) != 0 {
        menos(0xf5, k50(), 0x96afc4);
        menos(0x75, k50(), 0x96afc4);
    }
    let outro = (byte(this, 0x27) == 0) as i32;
    if nao_gosta_de(jx(outro), pessoa()) != 0 {
        wr(0xf5, rd(0xf5) + f(0x968990));
        wr(0x75, k25() as f64 * f(0x96d668) + rd(0x75));
    } else if gosta_de(jx(outro), pessoa()) != 0 {
        menos(0xf5, k50(), 0x96afc4);
        menos(0x75, k50(), 0x96afc4);
    }
    if gosta(pessoa(), clube_lado()) != 0 {
        wr(0xf5, rd(0xf5) + f(0x968990));
    } else if nao_gosta(pessoa(), clube_lado()) != 0 {
        menos(0xf5, k25(), 0x96d668);
    }
    let proprio = byte(this, 0x27) as i32;
    if nao_gosta_de(jx(proprio), pessoa()) != 0 {
        menos(0xf5, k25(), 0x96d668);
        menos(0xe1, k25(), 0x96d668);
    } else if gosta_de(jx(proprio), pessoa()) != 0 {
        wr(0xf5, rd(0xf5) + f(0x968990));
        wr(0xe1, rd(0xe1) + f(0x968990));
    }
    let pd = |m: usize| *((designado(m) + 0x69) as *const usize);
    if nao_gosta_de(pd(motor()), pessoa()) != 0 {
        menos(0xf5, k25(), 0x96d668);
        menos(0xe1, k25(), 0x96d668);
    } else if gosta_de(pd(motor()), pessoa()) != 0 {
        wr(0xf5, rd(0xf5) + f(0x968990));
        wr(0xe1, rd(0xe1) + f(0x968990));
    }
    // historico do contrato (+0x10 e +0x14 sobre +0xc jogos)
    if l2c != 0 {
        let c = l2c;
        let c0c = *((c + 0xc) as *const i32);
        for (o, tira) in [(0x10usize, true), (0x14, false)] {
            let e = *((c + o) as *const i32);
            if e > -1 && c0c > 0 {
                let r = e as f64 * f(0x96d664) / c0c as f64;
                if r > q(0x96afa8) {
                    let d = k25() as f64 * r * f(0x96d660);
                    wr(0xe1, if tira { rd(0xe1) - d } else { d + rd(0xe1) });
                }
            }
        }
    }
}

// ---------- septuagesima nona leva: nota do jogador pelo contrato, forma e idade ----------

/// n = contract level (0x53ce90); form = (word +0xd of the record − club word +0x80) × 0.002 + 1.5, clamped to −4..4; c
/// = contract byte +0x44 (0x4cf5f0) / 75 + 0.5, adjusted by type (+0x4f >> 4: −10, +2.5, +2, −3, −15, −20, −50); x =
/// byte +0x22 × 75 for a national side, otherwise days alive (0x54cdb0); y = 1 + (x − 1000) × 0.000125 above 1000.
/// Rating = byte +0x2f × y − (n² + 5) × 0.1 + form + c + 0.5, truncated
/// Original: `player_rating` (0x6ebd70, match_pl.cpp, thiscall).
///
/// [pt] { 0x6ebd70 } (match_pl.cpp): nota do jogador na partida, pelo clube do lado dele (motor +0x4792,
/// +0x1c/+0x20). n = nivel do contrato (0x53ce90, byte com sinal); forma = (palavra +0xd da ficha
/// +0x6d - palavra +0x80 do clube) x 0,002 + 1,5, limitada a -4..4; c = byte +0x44 do contrato
/// (0x4cf5f0) x (1/75) + 0,5, ajustado pelo tipo (+0x4f >> 4); x = byte +0x22 da pessoa x 75 na
/// selecao, senao os dias de vida (0x54cdb0); y = 1 + (x - 1000) x 0,000125 acima de 1000.
/// Resultado: byte +0x2f da ficha x y - (n^2 + 5) x 0,1 + forma + c + 0,5, truncado.
pub(crate) unsafe extern "thiscall" fn nota_do_jogador(this: usize) -> i32 {
    let clube = || {
        let t = *((*((this + 0x19e) as *const usize) + 0x4792) as *const usize);
        *((t + if byte(this, 0x27) == 0 { 0x1c } else { 0x20 }) as *const usize)
    };
    let pessoa = *((this + 0x69) as *const usize);
    let ficha = *((this + 0x6d) as *const usize);
    let nivel: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_CONTRACT_RATING_MODIFIER);
    let n = nivel(pessoa, clube()) as i8 as f64;
    let mut forma = (*((ficha + 0xd) as *const i16) as i32 - *((clube() + 0x80) as *const i16) as i32) as f64 * 0.002 + 1.5;
    if forma < -4.0 {
        forma = -4.0;
    } else if forma > 4.0 {
        forma = 4.0;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let r = contrato(*(0xae1050 as *const usize), pessoa, clube());
    let mut c = 0.0;
    if r != 0 {
        c = *((r + 0x44) as *const i8) as f64 * 0.013333333333333334 + 0.5;
        c = match byte(r, 0x4f) as u8 >> 4 {
            0 | 5 => c - 10.0,
            1 => c + 2.5,
            2 => c + 2.0,
            4 => c - 3.0,
            6 => c - 15.0,
            7 => c - 20.0,
            8 => c - 50.0,
            _ => c,
        };
    }
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let x = if e_selecao(clube()) != 0 {
        byte(pessoa, 0x22) as u8 as f64 * 75.0
    } else {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        let ler = |o: usize| core::ptr::read_unaligned((pessoa + o) as *const u32);
        dias(0xae2c90, ler(0x3e), ler(0x42)) as f64
    };
    let y = if x > 1000.0 { (x - 1000.0) * 0.000125 + 1.0 } else { 1.0 };
    let e = *((ficha + 0x2f) as *const i8) as f64;
    ftol(e * y - (n * n + 5.0) * 0.1 + forma + c + 0.5)
}

// ---------- quadragesima quinta leva: teste de condicao ----------

/// { 0x7164b0 } (match_pl.cpp): teste pela condicao fisica (0x1a2). v = condicao^2 (truncado);
/// limite 8 se `alvo` for o guardado em 0x1a6, senao 1. Codigo 1: sorteio(110 - condicao) < limite;
/// 2 a 5: sorteio(K / v) < limite, K = 400 mil, 2, 4 e 8 milhoes (mais condicao, mais chance).
/// Outros codigos: 0.
pub(crate) unsafe extern "thiscall" fn teste_de_condicao(this: usize, codigo: i8, alvo: usize) -> u8 {
    let limite: i32 = if alvo != 0 && alvo == *((this + 0x1a6) as *const usize) { 8 } else { 1 };
    let c = f32_em(this + 0x1a2) as f64;
    let v = ftol(pow_do_jogo(c, f64_em(0x968950))) as f64;
    let n = match codigo {
        1 => 0x6e - ftol(c) as i8 as i32,
        2 => ftol(f64_em(0x96d860) / v),
        3 => ftol(f64_em(0x96aff8) / v),
        4 => ftol(f64_em(0x96d858) / v),
        5 => ftol(f64_em(0x96d850) / v),
        _ => return 0,
    };
    (sortear(n) < limite) as u8
}

// ---------- quadragesima primeira leva: valor do chute (0x700bd0, 5,2 KB) ----------

/// pow da CRT do jogo (0x945ff0, `_CIpow`: base em st1, expoente em st0), para o resultado ser
/// identico bit a bit ao do original.
pub(crate) unsafe fn pow_do_jogo(base: f64, expoente: f64) -> f64 {
    let mut v = [base, expoente, 0.0];
    core::arch::asm!(
        "fld qword ptr [edi]",
        "fld qword ptr [edi + 8]",
        "call {f}",
        "fstp qword ptr [edi + 16]",
        // edi: a chamada preserva (eax/ecx/edx nao)
        in("edi") v.as_mut_ptr(),
        f = in(reg) 0x945ff0usize,
        clobber_abi("C"),
    );
    v[2]
}

/// Valor da tabela 12 x 9 (0xa15e44) para as distancias |dy| e |dx| (cada uma truncada a byte).
pub(crate) unsafe fn tabela_12x9(dy: i32, dx: i32) -> f32 {
    f32_em((0xa15e44 + 4 * ((dy.abs() as i8 as isize) * 9 + dx.abs() as i8 as isize)) as usize)
}

/// O jogador pode bater dali (0x6f9f90 para o centro do gol, salvo na pequena area central)?
pub(crate) unsafe fn pode_bater(j: usize) -> bool {
    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
    let fundo: i8 = if lado(j) == 0 { 0xb } else { 0 };
    if y == fundo && (3..=5).contains(&x) {
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_CELL_IN_VIEW);
    f(j, 4, fundo, 0, 0.0, 0.0, core::ptr::null_mut()) != 0
}

/// `tabela_a` do jogador por 0x6c38c0 (o motor do proprio jogador; sem motor, None).
pub(crate) unsafe fn papel_do(j: usize) -> Option<u16> {
    pelo_motor::<u16>(j, 0x6c38c0)
}

/// { 0x700bd0 } (match_pl.cpp, `MATCH_PLAYER::get_shot_score`): escolhe o tipo do chute (quando
/// `tipo` < 0) pela situacao e pelos atributos (palavras 0x146..0x154), calcula o valor (palavras
/// por tipo em 0x11a + 2 x tipo, condicao fisica, goleiro adversario, angulo, distancia ao gol com
/// pow 8 e 7) e grava em 0x39 a parte "de qualidade" e em `som` o comentario. Devolve o valor.
pub(crate) unsafe extern "thiscall" fn valor_do_chute(this: usize, tipo: *mut i8, som: *mut u16) -> u16 {
    let j = this;
    let w = |o: usize| *((j + o) as *const i16);
    let q39 = (j + 0x39) as *mut u16;
    let menos_mil = |tipo: *mut i8, som: *mut u16| {
        *tipo = 0x16;
        *som = 0x1f81;
        0xfc18u16
    };
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let gk = manter(*((j + 0x19e) as *const usize), (lado(j) == 0) as i8);
    let mut b13 = 0xfbu8.wrapping_sub(byte(j, 0x10b) as u8).wrapping_mul(2) as i8;
    let mut angulo_fechado = false;
    let gy = byte(gk, 0x103);
    if gy != 0xb && gy != 0 && papel_do(gk) == Some(1) {
        b13 = (byte(j, 0x10b) as u8).wrapping_add(5).wrapping_mul(2) as i8;
    }
    let dgk = tabela_12x9(byte(j, 0x103) as i32 - gy as i32, byte(j, 0x102) as i32 - byte(gk, 0x102) as i32);
    if !(dgk >= f32_em(0x968e70)) && papel_do(gk) == Some(1) {
        b13 = (b13 as u8).wrapping_sub(byte(j, 0x10b) as u8) as i8;
    }
    let (x, y) = (byte(j, 0x102) as i32, byte(j, 0x103) as i32);
    let dist = if lado(j) == 0 { tabela_12x9(y - 0xb, x - 4) } else { tabela_12x9(y, x - 4) } as f64;
    if *tipo == 0x33 {
        *som = 0x1f7f;
        return (sortear(w(0x180) as i32 * 5) + 0x96) as u16;
    }
    // escolha do tipo
    'escolha: {
        if *tipo == 0x35 {
            break 'escolha;
        }
        if w(0x198) == 0x3b {
            *tipo = 0x39;
            break 'escolha;
        }
        let motor = *((j + 0x19e) as *const usize);
        if pode_bater(j) && byte(motor, 0x8ea9) < 3 {
            let r = sortear(3);
            let l = lado(j);
            let alcance = byte(j, 0x10b) as i32 > r
                || (l == 0 && byte(j, 0x103) > byte(gk, 0x103))
                || (l == 1 && byte(j, 0x103) < byte(gk, 0x103));
            if alcance {
                let na_area = {
                    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
                    let area = if l == 0 { 10..=11 } else { 0..=1 };
                    (2..=6).contains(&x) && area.contains(&y)
                };
                let cobertura = if byte(gk, 0x2b) > 1 && na_area {
                    true
                } else if papel_do(gk) != Some(1) {
                    true
                } else {
                    let (gy, gx) = (byte(gk, 0x103) as i32, byte(gk, 0x102) as i32);
                    let t1 = if lado(gk) == 1 { tabela_12x9(gy - 0xb, gx - 4) } else { tabela_12x9(gy, gx - 4) };
                    // goleiro longe da propria meta (>= 2) e do atacante (>= 1): cobertura direta;
                    // senao, sorteio(0x29) contra sorteio(2000) e distancia ao gol < 1,5
                    let longe_da_meta = t1 as f64 >= f64_em(0x968950);
                    let direta = longe_da_meta && {
                        let t2 = tabela_12x9(gy - byte(j, 0x103) as i32, gx - byte(j, 0x102) as i32);
                        t2 as f64 >= f64_em(0x967890)
                    };
                    direta || {
                        let r1 = sortear(w(0x29) as i32);
                        r1 > sortear(0x7d0) && !(dist >= f64_em(0x968940))
                    }
                };
                if cobertura {
                    *tipo = 0x3a;
                    break 'escolha;
                }
            }
        }
        if *tipo > -1 {
            break 'escolha;
        }
        let maior = |v: &mut i16, r: i32, t: i8, tipo: *mut i8| {
            if r as i16 > *v {
                *v = r as i16;
                *tipo = t;
            }
        };
        match byte(motor, 0x8ea9) {
            0 => {
                if !pode_bater(j) {
                    return menos_mil(tipo, som);
                }
                let mut v = sortear(w(0x146) as i32) as i16;
                *tipo = 0x16;
                maior(&mut v, sortear(w(0x14e) as i32 + 20 * b13 as i32), 0x1a, tipo);
                if dist >= f64_em(0x968940) {
                    maior(&mut v, sortear(w(0x152) as i32), 0x1c, tipo);
                }
                let a = sortear(0x1f4);
                let b = sortear(0x96);
                maior(&mut v, sortear(w(0x154) as i32 - a + b), 0x1d, tipo);
                if sortear(w(0x148) as i32) as i16 > v {
                    *tipo = 0x17;
                }
            }
            1 => {
                if !pode_bater(j) {
                    return menos_mil(tipo, som);
                }
                let mut v = sortear(w(0x150) as i32 + 22 * b13 as i32) as i16;
                *tipo = 0x1b;
                let a = sortear(0x1f4);
                let b = sortear(0x96);
                maior(&mut v, sortear(w(0x154) as i32 - a + b), 0x1d, tipo);
                maior(&mut v, sortear(w(0x146) as i32), 0x16, tipo);
                if sortear(w(0x14c) as i32) as i16 > v {
                    *tipo = 0x19;
                }
            }
            2 | 3 => {
                if !pode_bater(j) {
                    if byte(*((j + 0x19e) as *const usize), 0x8ea9) != 3 {
                        return menos_mil(tipo, som);
                    }
                    *tipo = 5;
                    break 'escolha;
                }
                let v = sortear(w(0x150) as i32 + 22 * b13 as i32) as i16;
                *tipo = 0x1b;
                if sortear(w(0x14a) as i32) as i16 > v {
                    *tipo = 0x18;
                }
            }
            4 => {
                *tipo = if pode_bater(j) { 3 } else { 5 };
            }
            _ => return menos_mil(tipo, som),
        }
    }
    // valor
    let t = *tipo;
    let mut e: i32;
    let condicao = || {
        let q = ftol(f32_em(j + 0x4d) as f64) as i16 as i32;
        let c = 10000 - q;
        sortear(c.wrapping_mul(c) / 50000)
    };
    if t == -1 {
        // tipo desconhecido: o original mostra um erro antes
        e = sortear(0x32);
        *q39 = 0;
    } else if t == 0x3a {
        let r0 = condicao() as i16 as i32;
        let rr = r0 / 5;
        let r1 = sortear(rr);
        let base = w(0x29) as i32 / 2 - r1;
        let r2 = sortear(w(0x19c) as i32 / 5);
        *q39 = (base + r2 + 0xd2) as u16;
        let mut v = *((j + 0x11a + 2 * t as usize) as *const u16);
        v = v.wrapping_add((w(0x19c) as i32 / 5) as u16).wrapping_sub(rr as u16).wrapping_add(w(0x29) as u16);
        let a = sortear(10);
        let b = sortear(10);
        e = v as i32 + (a - b);
    } else {
        let r0 = condicao();
        if *tipo == 0x35 {
            *q39 = 0;
        } else {
            *q39 = ((w(0x29) as i32 / 2) as u16).wrapping_add(w(0x19c) as u16).wrapping_sub(r0 as u16);
        }
        let t = *tipo;
        let mut bp = *((j + 0x11a + 2 * t as isize as usize) as *const u16);
        bp = bp.wrapping_sub(sortear(r0 as i16 as i32) as u16);
        bp = bp.wrapping_add(sortear(w(0x19c) as i32) as u16);
        let motor = *((j + 0x19e) as *const usize);
        let c = byte(motor, 0x8ea9);
        let atributos = |bp: &mut u16| {
            for o in [0x79, 0x81, 0xe5] {
                let a = f32_em(j + o);
                if !(a >= f32_em(0x96b020)) {
                    let v = ftol(a as f64 * f32_em(0x96d7e0) as f64 - f32_em(0x96d1b0) as f64);
                    *bp = bp.wrapping_add(sortear(v) as u16);
                }
            }
        };
        if t == 5 && c == 4 {
            *q39 = (*q39).wrapping_add(0xfed4);
            atributos(&mut bp);
            bp = bp.wrapping_add((byte(j, 0x109) as i16 as u16).wrapping_mul(30)).wrapping_sub(400);
        } else if t == 5 && c == 3 {
            *q39 = (*q39).wrapping_add(0xff06);
            atributos(&mut bp);
            bp = bp.wrapping_add((byte(j, 0x109) as i16 as u16).wrapping_sub(15).wrapping_mul(20));
        } else if t == 0x39 {
            let v = (f64_em(0x96a648) - f32_em(j + 0x79) as f64 * f64_em(0x968eb0)) * (3 - c as i32) as f64;
            let r = sortear(ftol(v));
            bp = bp.wrapping_sub(r as u16);
            *q39 = (*q39).wrapping_sub(r as u16);
        } else if c == 3 {
            let r = sortear(0x190);
            bp = bp.wrapping_sub(r as u16);
            *q39 = (*q39).wrapping_sub(r as u16);
        }
        let r1 = sortear(w(0x29) as i32);
        e = r1 + sortear(bp as i16 as i32);
    }
    // angulo (fora das colunas centrais)
    let x = byte(j, 0x102);
    if !(3..=5).contains(&x) {
        let l = lado(j);
        let mut a = if l == 0 {
            (12 - byte(j, 0x103) as i32) as f64 / ((4 - x as i32) as f64).abs()
        } else if l == 1 {
            (byte(j, 0x103) as i32 + 1) as f64 / ((4 - x as i32) as f64).abs()
        } else {
            f32_em(0x968970) as f64
        };
        a *= a;
        if a < f64_em(0x968e80) {
            angulo_fechado = true;
        }
        a = f32_em(0x96d7d8) as f64 - a * f32_em(0x96d7dc) as f64;
        let minimo = f32_em(0x96af88) as f64;
        if minimo > a {
            a = minimo;
        }
        let mut k = ftol(a) as u16;
        if *tipo == 0x3a {
            k = (k as i16 as i32 / 3) as u16;
        }
        let e5 = f32_em(j + 0xe5) as f64;
        let f = if !(e5 >= 1.0) { 1.0 } else { e5 };
        let m = k as i16 as i32;
        e -= sortear(ftol(m as f64 * f64_em(0x9688a0) / f));
        *q39 = (*q39).wrapping_add((-(m / 2)) as u16);
    }
    // chute de longe (fora da area, distancia > 2,2): variacao pelo vento/gramado do motor
    let mut longe = false;
    if dist > f64_em(0x96d7d0) {
        let (x, y) = (byte(j, 0x102), byte(j, 0x103));
        let area = if lado(j) == 0 { 10..=11 } else { 0..=1 };
        if !((2..=6).contains(&x) && area.contains(&y)) {
            longe = true;
            let motor = *((j + 0x19e) as *const usize);
            if motor != 0 {
                match *tipo {
                    0x17 | 0x1a | 0x1b | 0x1d => match byte(motor, 0x4756) {
                        2 => {
                            let a = sortear(5);
                            e += sortear(2) - a;
                        }
                        3 => {
                            let a = sortear(10);
                            e += sortear(5) - a;
                        }
                        4 => {
                            let a = sortear(15);
                            e += sortear(10) - a;
                        }
                        _ => {}
                    },
                    0x16 => match byte(motor, 1) {
                        2 => e += sortear(4),
                        0x20 => {
                            let a = sortear(5);
                            e += a - sortear(5);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    if papel_do(gk) != Some(1) {
        e += sortear(0x1f4);
        *q39 = (*q39).wrapping_add(0xfa);
    }
    // distancia ao gol
    let t = *tipo;
    let x = byte(j, 0x102);
    let canto = (byte(j, 0x103) == 0 || byte(j, 0x103) == 0xb) && (x as i32 - 4).abs() == 1;
    if (t == 3 || t == 0x39) && dist >= f64_em(0x967890) && !canto {
        let mut v = ftol(pow_do_jogo(dist + f64_em(0x968950), f64_em(0x968f10)) * f64_em(0x968a80));
        if v > 0x3a98 {
            v = 0x3a98;
        }
        if (v as i16) < 0 {
            v = 0x3a98;
        }
        if *tipo == 0x3a {
            v = (v as i16 as i32) / 8;
        }
        let m = v as i16 as i32;
        e -= sortear(m);
        *q39 = (*q39).wrapping_add((-(m / 2)) as u16);
    } else if dist == 0.0 {
        e += 0xc8;
        *q39 = (*q39).wrapping_add(0xc8);
    } else if dist == 1.0 {
        e += 0x64;
        *q39 = (*q39).wrapping_add(0x64);
    } else if dist > f64_em(0x968940) {
        let mut c = ftol(pow_do_jogo(dist * f64_em(0x968e48) + f64_em(0x968940), f64_em(0x968988)));
        if c > 0x7530 {
            c = 0x7530;
        }
        let (k, mut f) = match *tipo {
            0x3a => (c / 20, (byte(j, 0x109) as f64 + f32_em(j + 0xb1) as f64) * f64_em(0x967878)),
            0x35 => (c / 15, f32_em(j + 0xa1) as f64 * f64_em(0x967888)),
            _ => (c / 2, (byte(j, 0x109) as f64 + f32_em(j + 0xb1) as f64) * f64_em(0x967878)),
        };
        f = f64_em(0x967890) - f;
        if f < f64_em(0x968918) {
            f = f64_em(0x968918);
        }
        *q39 = (*q39).wrapping_add((-(k / 2)) as u16);
        let mut r = ftol(k as f64 * f);
        if r > 0x7530 {
            r = 0x7530;
        }
        e -= sortear(r);
    }
    // comentario
    let fl = longe as u16;
    *som = match *tipo {
        3 => if angulo_fechado { 0x1fb8 } else { 0x1f92 + 0x18 * fl },
        5 => if angulo_fechado { 0x1fb7 } else { 0x1f91 + 0x18 * fl },
        0x16 => if angulo_fechado { 0x1fb0 } else { 0x1f86 + 0x1c * fl },
        0x17 => if angulo_fechado { 0x1fbb } else { 0x1f99 + 0x14 * fl },
        0x18 => if angulo_fechado { 0x1fb9 } else { 0x1f93 + 0x18 * fl },
        0x19 => if angulo_fechado { 0x1fba } else { 0x1f96 + 0x16 * fl },
        0x1a => if angulo_fechado { 0x1fb1 } else { 0x1f89 + 0x1a * fl },
        0x1b => if angulo_fechado { 0x1fb3 } else { 0x1f8b + 0x1a * fl },
        0x1c => if angulo_fechado { 0x1fb5 } else { 0x1f8d + 0x1a * fl },
        0x1d => if angulo_fechado { 0x1fb6 } else { 0x1f8e + 0x1a * fl },
        0x35 => {
            if angulo_fechado {
                0x1fbd
            } else if longe {
                0x1faf
            } else if sortear(3) != 0 {
                0x1fbe
            } else {
                0x21bd
            }
        }
        0x39 => if angulo_fechado { 0x1fbc } else { 0x1f9c + 0x12 * fl },
        0x3a => 0x1f9d,
        _ => 0x1f81,
    };
    e as u16
}

/// `tabela_a` do motor do jogador (0 sem motor), pelo endereco do jogo: os bits do papel tatico.
pub(crate) unsafe fn papel_word(motor: usize, j: usize) -> u16 {
    if motor == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    f(motor, j)
}

pub(crate) unsafe fn papel_por_endereco(endereco: usize, j: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(endereco);
    f(j)
}

