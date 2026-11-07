use super::*;

// ---------- quinta leva ----------

/// { 0x6cd330 } (match_eng.cpp): percorre os jogadores do lado `lado` no grupo `grupo` (quantidade
/// em grupo[0x58 + lado], ponteiros em grupo + lado x 44). Para cada um ativo (0x2b <= 1, posicao
/// >= 0): com `modo` 2 so conta; senao pergunta a 0x6f5c30 (pelo endereco do jogo) e, ao primeiro
/// sim, zera 0x1a5/0x1a6 deste objeto e devolve 1. Devolve a contagem (so cresce no modo 2).
pub(crate) unsafe extern "thiscall" fn procurar_no_lado(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_TRY_INTERCEPT_BALL);
    procurar_com(this, lado, grupo, a3, a4, a5, a6, modo, &mut |p, w, x, y, z| f(p, w, x, y, z))
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn procurar_com(
    this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8,
    f: &mut dyn FnMut(usize, u32, u32, u32, u32) -> u8,
) -> u8 {
    let lista = (grupo as isize + lado as isize * 44) as *const usize;
    let mut contagem = 0u8;
    let mut i = 0;
    while i < byte((grupo as isize + lado as isize) as usize, 0x58) as i32 {
        let p = *lista.add(i as usize);
        if byte(p, 0x2b) <= 1 && byte(p, 0x19) >= 0 {
            if modo == 2 {
                contagem = contagem.wrapping_add(1);
            } else if f(p, a4, a5, a3, a6) != 0 {
                *((this + 0x1a6) as *mut u8) = 0;
                *((this + 0x1a5) as *mut u8) = 0;
                return 1;
            }
        }
        i += 1;
    }
    contagem
}

/// { 0x6cb9b0 } (match_eng.cpp): o jogador (pelo lado dele) esta em posicao de alcancar (b, c)?
/// Faixas de coordenadas e, no fim, o valor da tabela de distancias (float, 12 x 9 em 0xa15e44;
/// fora dela, 1000) tem de passar de 1,5.
pub(crate) unsafe extern "stdcall" fn alcanca(j: usize, b: i8, c: i8) -> u8 {
    let s = lado(j);
    let faixa_c = if s == 0 { 10..=11 } else { 0..=1 };
    if !(2..=6).contains(&b) || !faixa_c.contains(&c) {
        return 0;
    }
    let x = byte(j, 0x102);
    if x == 4 {
        return 0;
    }
    let y = byte(j, 0x103);
    let meio = x < 6 && x > 2;
    if s == 0 {
        if (meio && c < y) || y < 9 {
            return 0;
        }
    } else if (meio && c > y) || y > 2 {
        return 0;
    }
    if b <= 2 || b >= 6 || (x as i32 - b as i32).abs() <= 1 {
        return 0;
    }
    let col = (b as i32 - x as i32).abs() as i8;
    let lin = (c as i32 - y as i32).abs() as i8;
    let v = if (0..=8).contains(&col) && (0..=11).contains(&lin) {
        f32_em(0xa15e44 + 4 * (lin as usize * 9 + col as usize))
    } else {
        f32_em(0x96d2dc)
    };
    (v as f64 > f64_em(0x968940)) as u8
}

/// Codigos de lance que 0x713e90 ignora (devolve 2).
pub(crate) const LANCES_IGNORADOS: [i16; 11] = [0x5b, 0x5c, 0x3d, 0x36, 0x34, 0x37, 0x5a, 0x33, 0x35, 0x39, 3];

/// { 0x713e90 } (match_pl.cpp): classifica um lance pelo angulo entre a direcao do jogador (0x19a)
/// e o alvo (0x6cb6c0 do motor, pelo endereco do jogo). Dentro da tolerancia
/// 90 - (18 x `forca` - float de 0xe5): devolve o byte 0x2c. Fora: 0 (um lado) ou 1 (outro), e
/// grava em `saida` (20 - atributo da pessoa) x 12 (0x30 ou 0x3b do registro).
pub(crate) unsafe extern "thiscall" fn classificar_lance(this: usize, codigo: i16, a: i8, b: i8, c: i8, d: i8, forca: f32, saida: *mut u16) -> u8 {
    *saida = 0;
    if LANCES_IGNORADOS.contains(&codigo) {
        return 2;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_DIRECTION_BETWEEN_CELLS);
    let dx = f(*((this + 0x19e) as *const usize), a, b, c, d).wrapping_sub(*((this + 0x19a) as *const u16)) as i16;
    let sinal: i16 = if dx >= 0 { 1 } else { -1 };
    let mut ang = sinal.wrapping_mul(dx);
    if ang > 0xb4 {
        ang = (0x168i32 - ang as i32) as i16;
    }
    let assinado = sinal.wrapping_mul(ang);
    let tol = ftol(f32_em(0x96d848) as f64 - (forca as f64 * f32_em(0x96d84c) as f64 - f32_em(this + 0xe5) as f64));
    let (lim, menos) = (tol as i16, (tol.wrapping_neg()) as i16);
    let pessoa = *((this + 0x6d) as *const usize);
    let (r, attr) = if assinado > lim {
        (0, byte(pessoa, 0x30))
    } else if assinado < menos {
        (1, byte(pessoa, 0x3b))
    } else {
        return byte(this, 0x2c) as u8;
    };
    *saida = ((0x14i16.wrapping_sub(attr as i16)).wrapping_mul(12)) as u16;
    r
}

// ---------- setima leva: linha de passe ----------

/// Jogadores numa casa (x, y) da grade do motor, pelo `procurar_no_lado` em modo 2 (so conta),
/// chamado pelo endereco do jogo. O 3o argumento e um float (bits), que o modo 2 nao usa.
pub(crate) unsafe fn contar_na_casa(motor: usize, x: i8, y: i8, valor: f32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_TRY_CELL_INTERCEPTION);
    let grupo = (motor as isize + 2 * 45 * (y as isize + 12 * x as isize) + 0x215e) as usize;
    let lado = (byte(motor, 0x8eae) == 0) as i8;
    f(motor, lado, grupo, valor.to_bits(), u32::MAX, u32::MAX, u32::MAX, 2)
}

/// Distancia da tabela para (|dx|, |dy|) dentro dela (coluna 0..=8, linha 0..=11); fora, 1000.
pub(crate) unsafe fn distancia_ou_mil(dx: i32, dy: i32) -> f32 {
    let (col, lin) = (dx.abs() as i8, dy.abs() as i8);
    if (0..=8).contains(&col) && (0..=11).contains(&lin) {
        f32_em(0xa15e44 + 4 * (lin as usize * 9 + col as usize))
    } else {
        f32::from_bits(0x447a0000)
    }
}

/// { 0x6cbc00 } (match_eng.cpp): quantos jogadores ha nas casas da linha de (x1, y1) a (x2, y2),
/// percorrida ao estilo de Bresenham. Grade de 9 x 12 casas; a contagem para na primeira casa fora.
/// Vertical/horizontal: do menor ao maior, sem a ultima casa. Diagonais: as duas pontas incluidas,
/// a partir da de menor x (rasa) ou menor y (ingreme). Contagem em byte (soma com estouro).
pub(crate) unsafe extern "thiscall" fn jogadores_na_linha(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    percorrer_linha(x1, y1, x2, y2, false, 8, &|x, y, v| contar_na_casa(this, x, y, v))
}

/// { 0x6cc070 } (match_eng.cpp): como 0x6cbc00, contando pela 0x6cd3d0 (modo 2) e com as duas
/// pontas nas linhas retas (na horizontal, x ate 11).
pub(crate) unsafe extern "thiscall" fn jogadores_no_trajeto(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u8, u32, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_TRY_CELL_SHOT_BLOCK);
    let lado = (byte(this, 0x8eae) == 0) as i8;
    percorrer_linha(x1, y1, x2, y2, true, 11, &|x, y, v| {
        let grupo = (this as isize + 2 * 45 * (y as isize + 12 * x as isize) + 0x215e) as usize;
        f(this, lado, grupo, v.to_bits(), 0, 2, 6, 0)
    })
}

/// O percurso de 0x6cbc00 e 0x6cc070: `ate_o_fim` inclui a ultima casa nas linhas retas e a
/// horizontal aceita x ate `x_horizontal`; `contar` da os jogadores de cada casa.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn percorrer_linha(x1: i8, y1: i8, x2: i8, y2: i8, ate_o_fim: bool, x_horizontal: i8, contar: &dyn Fn(i8, i8, f32) -> u8) -> u8 {
    let dx = (x2 as i32 - x1 as i32).abs() as i8;
    let dy = (y2 as i32 - y1 as i32).abs() as i8;
    let fora = |x: i8, y: i8| !(0..=8).contains(&x) || !(0..=11).contains(&y);
    let mut n = 0u8;
    if dx == 0 {
        let (inicio, fim) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        let mut y = inicio;
        while y < fim || (ate_o_fim && y == fim) {
            if fora(x1, y) {
                return n;
            }
            n = n.wrapping_add(contar(x1, y, (y as i32 - inicio as i32) as f32));
            y = y.wrapping_add(1);
        }
        return n;
    }
    if dy == 0 {
        let (inicio, fim) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let mut x = inicio;
        while x < fim || (ate_o_fim && x == fim) {
            if !(0..=x_horizontal).contains(&x) || !(0..=11).contains(&y1) {
                return n;
            }
            n = n.wrapping_add(contar(x, y1, (x as i32 - inicio as i32) as f32));
            x = x.wrapping_add(1);
        }
        return n;
    }
    // diagonais: os pontos trocam quando o eixo principal vem ao contrario
    let ingreme = dx < dy;
    let trocar = if ingreme { y1 > y2 } else { x1 > x2 };
    let (xa, ya, xb, yb) = if trocar { (x2, y2, x1, y1) } else { (x1, y1, x2, y2) };
    let mut erro: i8 = 0;
    if !ingreme {
        let passo: i8 = if yb as i32 - ya as i32 >= 0 { 1 } else { -1 };
        let mut y = ya;
        let mut x = xa;
        loop {
            if erro >= dx {
                erro = erro.wrapping_sub(dx);
                y = y.wrapping_add(passo);
            }
            if fora(x, y) {
                return n;
            }
            n = n.wrapping_add(contar(x, y, distancia_ou_mil(x as i32 - xa as i32, y as i32 - ya as i32)));
            erro = erro.wrapping_add(dy);
            x = x.wrapping_add(1);
            if x > xb {
                return n;
            }
        }
    }
    let passo: i8 = if xb as i32 - xa as i32 >= 0 { 1 } else { -1 };
    let mut x = xa;
    let mut y = ya;
    loop {
        if erro >= dy {
            erro = erro.wrapping_sub(dy);
            x = x.wrapping_add(passo);
        }
        if fora(x, y) {
            return n;
        }
        n = n.wrapping_add(contar(x, y, distancia_ou_mil(x as i32 - xa as i32, y as i32 - ya as i32)));
        erro = erro.wrapping_add(dx);
        y = y.wrapping_add(1);
        if y > yb {
            return n;
        }
    }
}

/// { 0x6f9f90 } (match_pl.cpp): o jogador consegue jogar a bola para (a, b) com a direcao atual?
/// 1 se ja esta virado exatamente para o alvo. Senao, tolerancia de angulo: 75 (sem `calcular`) ou
/// ftol(10 x 0x10b ... ) - ftol(...), limitada a 30..=135. Fora dela, 0. Com `saida`, grava quantos
/// jogadores ha na linha (0x6cbc00) e sorteia: 1 se 0x10b >= sorteio(c x c).
pub(crate) unsafe extern "thiscall" fn consegue_jogar(this: usize, a: i8, b: i8, calcular: i8, forca: f32, ajuste: f32, saida: *mut u8) -> u8 {
    let alvo = *((TABELA_GLOBAL as isize + 2 * (a.wrapping_sub(byte(this, 0x102)) as isize * 24 + b.wrapping_sub(byte(this, 0x103)) as isize)) as *const i16);
    let dir = *((this + 0x19a) as *const i16);
    if dir == alvo {
        return 1;
    }
    let tolerancia: i16 = if calcular == 0 {
        0x4b
    } else {
        let f: unsafe extern "stdcall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_CAN_RUN_INTO_CENTRAL_BOX);
        let v = byte(this, 0x10b) as i32;
        let (base, k) = if f(this, a, b) != 0 { (v * 4, 0x96d7b0) } else { (v * 5, 0x96d664) };
        let t1 = ftol((base as f64 + forca as f64 * f32_em(0x968a78) as f64) + f32_em(k) as f64);
        let d = distancia_tabela(byte(this, 0x103) as i32 - b as i32, byte(this, 0x102) as i32 - a as i32);
        let t2 = ftol((d as f64 - ajuste as f64) * f32_em(0x96afb0) as f64 - f32_em(0x969560) as f64);
        let t = t1.wrapping_sub(t2) as i16;
        t.clamp(0x1e, 0x87)
    };
    let mut ang = (dir as i32 - alvo as i32).abs();
    if ang as i16 > 0xb4 {
        ang = 0x168 - ang;
    }
    if ang as i16 >= tolerancia {
        return 0;
    }
    if saida.is_null() {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_INTERCEPTORS_ON_LINE);
    let c = f(*((this + 0x19e) as *const usize), byte(this, 0x102), byte(this, 0x103), a, b);
    *saida = c;
    let c = c as i8 as i32;
    (byte(this, 0x10b) as i32 >= sortear(c * c)) as u8
}

/// goal line and columns 3..5; otherwise 0x6f9f90 facing the middle of the goal, with no calculation and no output
/// (does not draw)
/// Original: `in_the_box` (0x6f4650, thiscall, `al`).
///
/// [pt] { 0x6f4650 } (match_pl.cpp): na entrada da area (linha do gol, colunas 3..=5)? Senao,
/// 0x6f9f90 (pelo endereco) virado para o meio do gol, sem calculo nem saida (nao sorteia).
pub(crate) unsafe extern "thiscall" fn na_area(this: usize) -> u8 {
    let linha: i8 = if lado(this) == 0 { 0xb } else { 0 };
    if byte(this, 0x103) == linha && (3..=5).contains(&byte(this, 0x102)) {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_CELL_IN_VIEW);
    f(this, 4, linha, 0, 0.0, 0.0, core::ptr::null_mut())
}

// ---------- decima primeira leva ----------

/// the moment's attributes (0x104 to 0x10d), several "best of two draws"; 15 draws in the original's order
/// Original: `draw_moment` (0x6f44a0, thiscall).
///
/// [pt] { 0x6f44a0 } (match_pl.cpp): atributos do momento, sorteados no inicio do lance. Varios sao
/// "o melhor de dois sorteios". A ordem dos 15 sorteios e a do original.
pub(crate) unsafe extern "thiscall" fn sortear_momento(this: usize) {
    let i = |o: usize| *((this + o) as *const i8) as i32;
    let grava = |o: usize, v: i32| *((this + o) as *mut u8) = v as u8;
    let melhor_de_dois = |n: i32| {
        let a = sortear(n);
        let b = sortear(n);
        if b > a { b } else { a }
    };
    let base = ftol(*((this + 0x4d) as *const f32) as f64 * f32_em(0x96d758) as f64) as i16 as i32;
    grava(0x104, melhor_de_dois(base + i(0x10e) - 0x19));
    grava(0x10a, melhor_de_dois(i(0x110)));
    grava(0x107, sortear(i(0x111)));
    grava(0x108, sortear(i(0x112)));
    grava(0x109, sortear(i(0x113)));
    grava(0x10b, melhor_de_dois(i(0x114)));
    grava(0x106, melhor_de_dois(i(0x10f)));
    grava(0x105, sortear(i(0x115)));
    grava(0x10d, sortear(i(0x116)));
    grava(0x10c, sortear(i(0x117)));
}

/// `tabela_a` direta: [motor + 0x8ebc + 4 x lado do jogador][posicao] (sem a checagem de faixa).
pub(crate) unsafe fn tabela_na(motor: usize, p: usize, posicao: i8) -> u16 {
    let t = *((motor as isize + 0x8ebc + 4 * lado(p) as isize) as *const usize) as *const u16;
    *t.offset(posicao as isize)
}

/// { 0x6ce110 } (match_eng.cpp): mantem o jogador de posicao 1 de cada lado (0xf59e: bloco 0;
/// 0xf5a2: bloco 1). Perdido (posicao < 0) zera; sem ninguem, o primeiro com `tabela` 1 na posicao;
/// se ainda nao houver, promove o primeiro ativo, trocando com quem guardava a posicao 1 em
/// reserva (0x1a, com posicao -2 e 0x20 = -1). Devolve o do lado pedido.
pub(crate) unsafe extern "thiscall" fn manter_posicao_um(this: usize, lado_pedido: i8) -> usize {
    for (campo, bloco) in [(0xf59eusize, 0x4796usize), (0xf5a2, 0x6a6e)] {
        let guardado = (this + campo) as *mut usize;
        if *guardado != 0 && byte(*guardado, 0x19) < 0 {
            *guardado = 0;
        }
        if *guardado != 0 {
            continue;
        }
        let mut reserva = 0usize;
        for k in 0..20 {
            let p = this + bloco + k * 0x1be;
            let pos = byte(p, 0x19);
            if *guardado == 0 && (0..=10).contains(&pos) && tabela_na(this, p, pos) == 1 {
                *guardado = p;
            }
            let r = byte(p, 0x1a);
            if (0..=10).contains(&r) && tabela_na(this, p, r) == 1 && byte(p, 0x19) == -2 && byte(p, 0x20) == -1 {
                reserva = p;
            }
        }
        if *guardado != 0 {
            continue;
        }
        let Some(p) = (0..20).map(|k| this + bloco + k * 0x1be).find(|&p| byte(p, 0x19) >= 0) else { continue };
        let mut nova = 0i8;
        if reserva != 0 {
            nova = byte(reserva, 0x1a);
            *((reserva + 0x1a) as *mut i8) = byte(p, 0x19);
        }
        *guardado = p;
        if tabela_na(this, p, nova) == 1 {
            *((p + 0x19) as *mut i8) = nova;
        }
    }
    *((this + if lado_pedido == 1 { 0xf5a2 } else { 0xf59e }) as *const usize)
}

/// { 0x6bd5b0 } (match_eng.cpp): caminho livre para avancar? 0 se a casa dele estiver marcada (sem
/// `ignorar_casa`) ou se algum adversario ativo a frente estiver na mesma coluna; senao 1.
pub(crate) unsafe extern "thiscall" fn caminho_livre(this: usize, j: usize, ignorar_casa: i8) -> u8 {
    let s = lado(j);
    let outro = (s == 0) as isize;
    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
    if ignorar_casa == 0 && *((this as isize + outro + 90 * (y as isize + 12 * x as isize) + 0x21b6) as *const u8) != 0 {
        return 0;
    }
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    for k in 0..20 {
        let p = (this as isize + 0x4796 + 8920 * outro) as usize + k * 0x1be;
        if byte(p, 0x19) < 0 || f717410(p) == 1 {
            continue;
        }
        let a_frente = if s == 1 { y > byte(p, 0x103) } else { s == 0 && y < byte(p, 0x103) };
        let px = byte(p, 0x102) as i32;
        if a_frente && x as i32 + 1 > px && (x as i32 - 1) < px {
            return 0;
        }
    }
    1
}

/// { 0x6cbad0 } (match_eng.cpp): ha espaco livre na casa a frente (diagonais primeiro)? Fora do
/// campo, 0. Casas marcadas na grade do lado oposto: x + 1 (se x < 8), x - 1 (se x > 0), x.
pub(crate) unsafe extern "thiscall" fn espaco_a_frente(this: usize, j: usize) -> u8 {
    let x = byte(j, 0x102);
    let outro = (lado(j) == 0) as isize;
    let y = if lado(j) == 1 {
        let y = byte(j, 0x103).wrapping_sub(1);
        if y < 0 {
            return 0;
        }
        y
    } else {
        let y = byte(j, 0x103).wrapping_add(1);
        if y > 0xb {
            return 0;
        }
        y
    };
    let casa = |base: isize| *((this as isize + outro + 90 * (y as isize + 12 * x as isize) + base) as *const u8);
    if x < 8 && casa(0x25ee) == 0 {
        return 1;
    }
    if x > 0 && casa(0x1d7e) == 0 {
        return 1;
    }
    (casa(0x21b6) == 0) as u8
}

/// { 0x6cb910 } (match_eng.cpp): 1 fora da faixa 2..=6.
pub(crate) unsafe extern "stdcall" fn fora_do_meio(v: i8) -> u8 {
    !(2..=6).contains(&v) as u8
}

/// { 0x6cb700 } (match_eng.cpp): quantos jogadores ativos do `lado` estao alem da linha `y` (lado
/// 1: y menor; lado 0: y maior; outros lados: nenhum), opcionalmente sem o guardado em 0xf582.
pub(crate) unsafe extern "thiscall" fn alem_da_linha(this: usize, y: i8, lado_b: i8, sem_guardado: i8) -> u8 {
    let mut n = 0u8;
    for k in 0..20 {
        let p = (this as isize + 0x4796 + 8920 * lado_b as isize) as usize + k * 0x1be;
        if byte(p, 0x19) < 0 || (sem_guardado != 0 && p == *((this + 0xf582) as *const usize)) {
            continue;
        }
        let py = byte(p, 0x103);
        if (lado_b == 1 && py < y) || (lado_b == 0 && py > y) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// { 0x6cc4e0 } (match_eng.cpp): distancia (tabela) entre (a, b) e (c, d); fora da tabela, 1000.
pub(crate) unsafe extern "stdcall" fn distancia_entre(a: i8, b: i8, c: i8, d: i8) -> f32 {
    distancia_ou_mil(a as i32 - c as i32, b as i32 - d as i32)
}

// ---------- decima terceira leva ----------

/// Bloco de 20 jogadores do lado no motor (0x4796 + 8920 x lado), de 0x1be em 0x1be.
pub(crate) fn jogador_do_bloco(motor: usize, lado_b: i8, k: usize) -> usize {
    (motor as isize + 0x4796 + 8920 * lado_b as isize) as usize + k * 0x1be
}

pub(crate) unsafe fn f717410(p: usize) -> u16 {
    let f: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    f(p)
}

pub(crate) unsafe fn f6f09b0(p: usize, a: i8, b: i8) -> f32 {
    let f: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    f(p, a, b)
}

/// { 0x6cbbb0 } (match_eng.cpp): quantos ativos (posicao >= 0) o lado tem; guarda em motor[4 + lado].
pub(crate) unsafe extern "thiscall" fn contar_ativos(this: usize, lado_b: i8) -> u8 {
    let n = (0..20).filter(|&k| byte(jogador_do_bloco(this, lado_b, k), 0x19) >= 0).count() as u8;
    *((this as isize + 4 + lado_b as isize) as *mut u8) = n;
    n
}

/// { 0x6cb650 } (match_eng.cpp): 1 para alguns codigos de lance (37, 42..=50, 96, 97), pelo switch
/// do original (tabelas lidas do exe: indices em 0x6cb680, destinos em 0x6cb678).
pub(crate) unsafe extern "stdcall" fn codigo_especial(codigo: i8) -> u8 {
    let i = codigo as i32 - 0x25;
    if !(0..=0x3c).contains(&i) {
        return 0;
    }
    let caso = *((0x6cb680 + i as usize) as *const u8) as usize;
    (*((0x6cb678 + 4 * caso) as *const u32) == 0x6cb66c) as u8
}

/// { 0x6cd5f0 } (match_eng.cpp): o jogador esta atras do extremo guardado do lado dele?
/// Lado 0: y <= y do menor (0xf58e; sem ele, 12). Outros: y >= y do maior (0xf592; sem ele, -1).
pub(crate) unsafe extern "thiscall" fn atras_do_extremo(this: usize, j: usize) -> u8 {
    let y = byte(j, 0x103);
    if lado(j) == 0 {
        let p = *((this + 0xf58e) as *const usize);
        (y <= if p != 0 { byte(p, 0x103) } else { 0xc }) as u8
    } else {
        let p = *((this + 0xf592) as *const usize);
        (y >= if p != 0 { byte(p, 0x103) } else { -1 }) as u8
    }
}

/// `*[0x4792] == −2`
/// Original: `no_match` (0x6d4280, thiscall, `al`).
///
/// [pt] { 0x6d4280 } (match_eng.cpp): o ponteiro em 0x4792 aponta para -2?
pub(crate) unsafe extern "thiscall" fn sem_partida(this: usize) -> u8 {
    (**((this + 0x4792) as *const *const i32) == -2) as u8
}

/// { 0x6bd6b0 } (match_eng.cpp): pressao sobre a area do lado. Soma os adversarios nas linhas
/// perto do gol (grade de contagem, lado oposto), pesando mais as mais proximas (6..1), e marca
/// motor[6 + lado] se a soma (16 bits) passar de sorteio(15) + 30.
pub(crate) unsafe extern "thiscall" fn pressao_na_area(this: usize, lado_b: i8) {
    let marca = (this as isize + 6 + lado_b as isize) as *mut u8;
    *marca = 0;
    let mut soma: i32 = 0;
    let celula = |base: usize, coluna: usize| *((base + coluna * 0x438) as *const i8) as i16 as i32;
    if lado_b == 0 {
        for linha in (0..=5i32).rev() {
            let base = this + 0x2379 - (5 - linha) as usize * 0x5a;
            for c in 0..9 {
                soma = soma.wrapping_add((celula(base, c) * (6 - linha)) as i16 as i32);
            }
        }
    } else {
        for linha in 6..12i32 {
            let base = this + 0x23d2 + (linha - 6) as usize * 0x5a;
            for c in 0..9 {
                soma = soma.wrapping_add((celula(base, c) * (linha - 5)) as i16 as i32);
            }
        }
    }
    let limite = sortear(0xf) + 0x1e;
    if soma as i16 as i32 > limite {
        *marca = 1;
    }
}

/// { 0x6c3010 } (match_eng.cpp): o jogador e o mais proximo de (a, b) entre os colegas disponiveis
/// (ativos, diferentes dele, 0x2b = 0)? 0 se algum estiver estritamente mais perto (ou na mesma
/// distancia, quando ele tem `tabela` 1). Na propria casa (distancia 0) e sem `tabela` 1: 1 direto.
pub(crate) unsafe extern "thiscall" fn mais_proximo_do_ponto(this: usize, j: usize, a: i8, b: i8) -> u8 {
    let d = f6f09b0(j, a, b);
    let zero = f64_em(0x9689a0);
    if (d as f64 == zero || (d as f64).is_nan()) && f717410(j) != 1 {
        return 1;
    }
    let s = lado(j);
    for k in 0..20 {
        let p = jogador_do_bloco(this, s, k);
        if byte(p, 0x19) < 0 || p == j || byte(p, 0x2b) != 0 {
            continue;
        }
        let dp = f6f09b0(p, a, b);
        if !(dp >= d) {
            return 0; // C0: menor ou sem ordem
        }
        if f717410(j) == 1 && (dp == d || dp.is_nan()) {
            return 0;
        }
    }
    1
}

/// { 0x6c3a00 } (match_eng.cpp): pressao no ponto (a, b): para cada jogador ativo do lado (sem
/// `tabela` 1; 0x2b = 0 salvo `todos`), +3 na propria casa (distancia 0), +1 a distancia <= 1,5.
pub(crate) unsafe extern "thiscall" fn pressao_no_ponto(this: usize, lado_b: i8, a: i8, b: i8, todos: i8) -> u8 {
    let mut n = 0u8;
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) < 0 || f717410(p) == 1 || (todos == 0 && byte(p, 0x2b) != 0) {
            continue;
        }
        let d = f6f09b0(p, a, b) as f64;
        if d == f64_em(0x9689a0) || d.is_nan() {
            n = n.wrapping_add(3);
        } else if !(d > f64_em(0x968940)) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// { 0x6c3950 } (match_eng.cpp): menor distancia ao ponto (a, b) entre os jogadores do lado (bit 0
/// de `tabela` zerado; 0x2b = 0 salvo `todos`), partindo de 5000; 0 se algum estiver na casa.
pub(crate) unsafe extern "thiscall" fn menor_distancia(this: usize, lado_b: i8, a: i8, b: i8, todos: i8) -> f32 {
    let mut menor = f32::from_bits(0x459c4000);
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) < 0 || f717410(p) & 1 != 0 || (todos == 0 && byte(p, 0x2b) != 0) {
            continue;
        }
        let d = f6f09b0(p, a, b);
        if d as f64 == f64_em(0x9689a0) || d.is_nan() {
            return f32_em(0x968970);
        }
        if d < menor {
            menor = d;
        }
    }
    menor
}

// ---------- decima quarta leva: disposicao repartida pelo papel ----------

/// O que cada bit do papel faz com o delta: nada (pula o jogador), soma ftol(delta x fator) ou soma o delta.
#[derive(Clone, Copy)]
pub(crate) enum Parte {
    Pula,
    Fator(usize),
    Inteiro,
    /// delta + delta (fadd st0, st0)
    Dobro,
}

/// Soma a cada jogador ativo do lado uma parte de `delta` na palavra 0x35 (disposicao), pelo
/// primeiro bit do papel (`tabela`, ordem 1, 2, 4, 8, 0x10, 0x20, 0x40); sem bit, nada.
pub(crate) unsafe fn repartir(this: usize, lado_b: i8, delta: i16, partes: [Parte; 7]) {
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let papel = f717410(p) as u8;
        let Some(i) = (0..7).find(|&i| papel & (1 << i) != 0) else { continue };
        let w = (p + 0x35) as *mut i16;
        match partes[i] {
            Parte::Pula => {}
            Parte::Fator(k) => *w = (*w).wrapping_add(ftol(delta as f64 * f64_em(k)) as i16),
            Parte::Inteiro => *w = (*w).wrapping_add(delta),
            Parte::Dobro => *w = (*w).wrapping_add(ftol(delta as f64 + delta as f64) as i16),
        }
    }
}

/// { 0x6c06b0 } (match_eng.cpp): reparte a disposicao (papel 1 fica de fora).
pub(crate) unsafe extern "thiscall" fn repartir_a(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Pula, Fator(0x968918), Fator(0x968f20), Fator(0x96d328), Fator(0x969040), Fator(0x968978), Fator(0x968f88)]);
}

/// { 0x6c07a0 } (match_eng.cpp): reparte a disposicao (papel 4 recebe o delta inteiro).
pub(crate) unsafe extern "thiscall" fn repartir_b(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Fator(0x96d330), Fator(0x9689b8), Inteiro, Fator(0x969520), Fator(0x968e00), Fator(0x968f20), Fator(0x967888)]);
}

/// { 0x6c0a70 } (match_eng.cpp): reparte a disposicao (terceira tabela de fatores).
pub(crate) unsafe extern "thiscall" fn repartir_c(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Fator(0x96d330), Fator(0x968968), Fator(0x968968), Fator(0x968968), Fator(0x968968), Fator(0x969510), Fator(0x968918)]);
}

/// { 0x6c2750 } (match_eng.cpp): escolhe um jogador do lado (indice 0..19, -1 sem candidato) por
/// sorteio ponderado pelo papel: maior sorteio(25) x fator (0,1 / 0,1 / 0,2 / 0,5 / 0,75 / 1,05 ou
/// 1,0). O original guarda o fator no lugar do argumento na pilha: jogador sem bit de papel usa o
/// fator anterior, e o primeiro, os bits da palavra inteira empilhada pelo chamador (por isso u32).
pub(crate) unsafe extern "thiscall" fn escolher_por_papel(this: usize, argumento: u32) -> u8 {
    let lado_b = argumento as i8;
    let mut fator = f32::from_bits(argumento);
    let mut melhor = 0.0f32;
    let mut escolhido: i8 = -1;
    for k in 0..20u8 {
        let p = jogador_do_bloco(this, lado_b, k as usize);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let papel = f717410(p);
        if papel & 1 != 0 {
            continue;
        }
        let bits: u32 = if papel & 2 != 0 || papel & 4 != 0 {
            0x3dcccccd
        } else if papel & 8 != 0 {
            0x3e4ccccd
        } else if papel & 0x10 != 0 {
            0x3f000000
        } else if papel & 0x20 != 0 {
            0x3f400000
        } else if papel & 0x40 != 0 {
            if papel & 0x700 != 0 { 0x3f866666 } else { 0x3f800000 }
        } else {
            fator.to_bits()
        };
        fator = f32::from_bits(bits);
        let v = sortear(0x19) as f64 * fator as f64;
        if escolhido == -1 || v > melhor as f64 {
            melhor = v as f32;
            escolhido = k as i8;
        }
    }
    escolhido as u8
}

// ---------- decima sexta leva ----------

/// { 0x6c0890 } (match_eng.cpp): reparte a disposicao (papel 1 de fora; quarta tabela de fatores).
pub(crate) unsafe extern "thiscall" fn repartir_d(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    let f = Fator(0x969040);
    repartir(this, lado_b, delta, [Pula, f, f, f, f, f, Fator(0x968940)]);
}

/// { 0x6c0980 } (match_eng.cpp): reparte a disposicao (papel 1 de fora; quinta tabela).
pub(crate) unsafe extern "thiscall" fn repartir_e(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Pula, Fator(0x967880), Fator(0x967880), Fator(0x96d340), Fator(0x96d338), Fator(0x9689d0), Fator(0x968940)]);
}

/// { 0x6c0b70 } (match_eng.cpp): reparte a disposicao (papel 1 recebe o dobro; sexta tabela).
pub(crate) unsafe extern "thiscall" fn repartir_f(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Dobro, Fator(0x9689e0), Fator(0x968918), Fator(0x968f20), Fator(0x967888), Fator(0x968a98), Fator(0x968a98)]);
}

/// { 0x6bb9a0 } (match_eng.cpp): registra o lance (0x475a..=0x4769) se nao houver jogador nele.
pub(crate) unsafe extern "thiscall" fn registrar_lance(this: usize, codigo: u16, jogador: u32, outro: u32, b4769: u8, b475a: u8, b475b: u8, b475c: u8, b475d: u8, b475e: u8) {
    if *((this + 0x4761) as *const u32) != 0 {
        return;
    }
    *((this + 0x475f) as *mut u16) = codigo;
    *((this + 0x4761) as *mut u32) = jogador;
    *((this + 0x4765) as *mut u32) = outro;
    for (o, v) in [(0x4769, b4769), (0x475a, b475a), (0x475b, b475b), (0x475c, b475c), (0x475d, b475d), (0x475e, b475e)] {
        *((this + o) as *mut u8) = v;
    }
}

/// { 0x714ca0 } (match_pl.cpp): tira o ponteiro da lista (0x1ae, quantidade em 0x101) pondo o
/// ultimo no lugar; devolve a nova quantidade (sem achar, a quantidade; lista vazia, 0).
pub(crate) unsafe extern "thiscall" fn remover_da_lista(this: usize, alvo: usize) -> u8 {
    let n = byte(this, 0x101);
    if n <= 0 {
        return 0;
    }
    let item = |k: i8| (this + 0x1ae + 4 * k as usize) as *mut usize;
    for k in 0..n {
        if *item(k) == alvo {
            *item(k) = *item(n - 1);
            *((this + 0x101) as *mut i8) = n - 1;
            return (n - 1) as u8;
        }
    }
    n as u8
}

/// { 0x6cdd00 } (match_eng.cpp): poe o jogador no meio do campo (8, 6) e na lista do lado
/// (0x453a, 11 por lado; quantidade em 0x4592 + lado), guardando a ordem em 0x28.
pub(crate) unsafe extern "thiscall" fn entrar_em_campo(this: usize, jogador: usize) -> u8 {
    let l = lado(jogador) as isize;
    *((jogador + 0x102) as *mut i8) = 8;
    *((jogador + 0x103) as *mut i8) = 6;
    let contagem = (this as isize + 0x4592 + l) as *mut i8;
    *((this as isize + 0x453a + 4 * (*contagem as isize + 11 * l)) as *mut usize) = jogador;
    // relidos depois de gravar: a vaga pode cair sobre a propria contagem (lado fora de 0/1)
    *((jogador + 0x28) as *mut i8) = *contagem;
    let n = (*contagem).wrapping_add(1);
    *contagem = n;
    n as u8
}

/// 12 × 9 float table (0xa15e44) ≤ 1.0, otherwise `atras_do_extremo`
/// Original: `in_danger` (0x70d630, thiscall, `al`).
///
/// [pt] { 0x70d630 } (match_pl.cpp): perigo pela tabela de floats 12 x 9 em 0xa15e44 (distancia em y a
/// propria linha de fundo, em x ao centro): valor <= 1,0 (ou NaN) da 1; senao, `atras_do_extremo`.
pub(crate) unsafe extern "thiscall" fn em_perigo(this: usize) -> u8 {
    let fundo = if lado(this) != 0 { 11 } else { 0 };
    let dy = (byte(this, 0x103) as i32 - fundo).abs() as i8 as isize;
    let dx = (byte(this, 0x102) as i32 - 4).abs() as i8 as isize;
    let v = f32_em((0xa15e44 + 4 * (dy * 9 + dx)) as usize);
    if !(v as f64 > f64_em(0x967890)) {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_BEHIND_OWN_BACK_LINE);
    (f(*((this + 0x19e) as *const usize), this) != 0) as u8
}

// ---------- decima setima leva ----------

/// 1/2 according to the move's side and area; draw(200) decides whether to clear
/// Original: `check_move` (0x6bba80, thiscall, `al`).
///
/// [pt] { 0x6bba80 } (match_eng.cpp): confere o lance registrado. 1 sem lance; 2 se o jogador do lance
/// e do lado em 0x8eae, se o lance (codigo diferente de 8) esta na faixa 2..=6 x area do lado, ou
/// se sorteio(200) der 0; senao limpa o lance e da 1.
pub(crate) unsafe extern "thiscall" fn conferir_lance(this: usize) -> u8 {
    let codigo = byte(this, 0x475a);
    let p = *((this + 0x4761) as *const usize);
    if codigo == 0 || p == 0 {
        return 1;
    }
    let lado_do_lance = byte(this, 0x8eae);
    if byte(p, 0x27) == lado_do_lance {
        return 2;
    }
    if codigo != 8 {
        let area = if lado_do_lance == 0 { 10..=11 } else { 0..=1 };
        if (2..=6).contains(&byte(this, 0x475b)) && area.contains(&byte(this, 0x475c)) {
            return 2;
        }
    }
    if sortear(200) == 0 {
        return 2;
    }
    limpar_lance(this);
    1
}

/// { 0x6c41b0 } (match_eng.cpp): 0 se (x, y) esta fora do campo util do lado (lado 0: y > 8;
/// outros: y < 3) ou se um adversario ativo esta na casa, ou (sem `tabela` 1) na mesma coluna
/// entre o ponto e o gol (lado 1: y maior que o dele; lado 0: menor); senao 1.
pub(crate) unsafe extern "thiscall" fn caminho_livre_ate(this: usize, lado_b: i8, x: i8, y: i8) -> u8 {
    if (lado_b != 0 && y < 3) || (lado_b == 0 && y > 8) {
        return 0;
    }
    for k in 0..20 {
        let p = jogador_do_bloco(this, (lado_b == 0) as i8, k);
        if byte(p, 0x19) < 0 {
            continue;
        }
        if x == byte(p, 0x102) && y == byte(p, 0x103) {
            return 0;
        }
        if f717410(p) == 1 {
            continue;
        }
        let alem = match lado_b {
            1 => y > byte(p, 0x103),
            0 => y < byte(p, 0x103),
            _ => false,
        };
        let (px, cx) = (byte(p, 0x102) as i32, x as i32);
        if alem && cx + 1 > px && cx - 1 < px {
            return 0;
        }
    }
    1
}

/// { 0x713d90 } (match_pl.cpp): o jogador alcanca (x, y)? Na propria casa, 1. Senao compara as
/// palavras de `por_diferenca` do ponto ao centro da linha de fundo e do ponto ao jogador (diferenca
/// abaixo de 0x10c + 10) e a distancia do ponto a linha de fundo com a tabela 12 x 9 (0xa15e44).
pub(crate) unsafe extern "thiscall" fn alcanca_o_ponto(this: usize, x: i8, y: i8) -> u8 {
    let (jx, jy) = (byte(this, 0x102), byte(this, 0x103));
    if x == jx && y == jy {
        return 1;
    }
    let fundo: i8 = if lado(this) != 0 { 11 } else { 0 };
    let motor = *((this + 0x19e) as *const usize);
    let pd: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_DIRECTION_BETWEEN_CELLS);
    let a = pd(motor, x, y, 4, fundo) as i16 as i32;
    let b = pd(motor, x, y, jx, jy) as i16 as i32;
    if (a - b).abs() >= byte(this, 0x10c) as i32 + 10 {
        return 0;
    }
    let dx = (jx as i32 - x as i32).abs() as i8 as isize;
    let dy = (jy as i32 - y as i32).abs() as i8 as isize;
    let t = f32_em((0xa15e44 + 4 * (dy * 9 + dx)) as usize);
    let de: unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32 = core::mem::transmute(FN_GET_CELL_DISTANCE);
    (de(x, y, 4, fundo) >= t) as u8
}

// ---------- decima oitava leva ----------

/// 16-bit score (disposition + 125 × (0x10 + 2 × 0x0c) + 50 × 0x03 + 35 × 0x06) and ftol(disposition × c); stores the
/// record's id at 0x477a
/// Original: `choose_standout` (0x6d0f80, among the (match byte 0x4d & 0xf) + 11 of each side, thiscall).
///
/// [pt] { 0x6d0f80 } (match_eng.cpp): o destaque entre os (byte 0x4d da partida & 0xf) + 11 primeiros
/// de cada lado (inativos com 0x20 = 0xff de fora): pontuacao (16 bits) = disposicao 0x35 +
/// 125 x (0x10 + 2 x 0x0c) + 50 x 0x03 + 35 x 0x06; troca so com pontuacao maior e
/// ftol(disposicao x fator) nao menor. Guarda o primeiro dword da ficha (0x69) em 0x477a.
pub(crate) unsafe extern "thiscall" fn escolher_destaque(this: usize) {
    let partida = *((this + 0x4792) as *const usize);
    let n = (byte(partida, 0x4d) & 0xf) + 0xb;
    let (mut melhor, mut pontos, mut valor) = (0usize, 0i16, 0i16);
    for l in 0..2usize {
        for k in 0..n.max(0) as usize {
            let p = this + 0x4796 + 0x22d8 * l + 0x1be * k;
            if byte(p, 0x19) < 0 && byte(p, 0x20) == -1 {
                continue;
            }
            let disp = *((p + 0x35) as *const i16);
            let v = ftol(disp as f64 * f64_em(0x96a350)) as i16;
            let u = |o: usize| byte(p, o) as u8 as i16;
            let pt = disp
                .wrapping_add(125i16.wrapping_mul(u(0x10).wrapping_add(2 * u(0x0c))))
                .wrapping_add(50i16.wrapping_mul(u(0x03)))
                .wrapping_add(35i16.wrapping_mul(u(0x06)));
            if melhor == 0 || (pt > pontos && v >= valor) {
                melhor = p;
                pontos = pt;
                valor = v;
            }
        }
    }
    if melhor != 0 {
        *((this + 0x477a) as *mut u32) = **((melhor + 0x69) as *const *const u32);
    }
}

/// { 0x6c2880 } (match_eng.cpp): como `escolher_por_papel`, com outros fatores (1,2 / 1,2 / 0,65 /
/// 0,5 / 0,35; papel 0x40: 0,25 com 0x700, senao 0,15) ajustados pelos atributos da pessoa (0x6d:
/// 0x44, 0x39 e 0x3e). O fator, inclusive os ajustes, fica no lugar do argumento e passa adiante.
pub(crate) unsafe extern "thiscall" fn escolher_por_papel_b(this: usize, argumento: u32) -> u8 {
    let lado_b = argumento as i8;
    let mut fator = f32::from_bits(argumento);
    let mut melhor = 0.0f32;
    let mut escolhido: u8 = 0xff;
    for k in 0..20u8 {
        let p = jogador_do_bloco(this, lado_b, k as usize);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let papel = f717410(p);
        if papel & 1 != 0 {
            continue;
        }
        let bits = if papel & 6 != 0 {
            Some(0x3f99999a)
        } else if papel & 8 != 0 {
            Some(0x3f266666)
        } else if papel & 0x10 != 0 {
            Some(0x3f000000)
        } else if papel & 0x20 != 0 {
            Some(0x3eb33333)
        } else if papel & 0x40 != 0 {
            Some(if papel & 0x700 != 0 { 0x3e800000 } else { 0x3e19999a })
        } else {
            None
        };
        if let Some(b) = bits {
            fator = f32::from_bits(b);
        }
        let pessoa = *((p + 0x6d) as *const usize);
        if pessoa != 0 {
            let a = byte(pessoa, 0x44);
            if a > 15 {
                fator *= f32_em(0x96a33c);
            } else if a > 10 {
                fator *= f32_em(0x96d3b8);
            } else if a < 5 {
                fator *= f32_em(0x96b014);
            }
            for o in [0x39, 0x3e] {
                let a = byte(pessoa, o);
                if a > 15 {
                    fator *= f32_em(0x96a33c);
                } else if a < 5 {
                    fator *= f32_em(0x96b014);
                }
            }
        }
        let v = sortear(0x19) as f64 * fator as f64;
        if escolhido == 0xff || v > melhor as f64 {
            melhor = v as f32;
            escolhido = k;
        }
    }
    escolhido
}

/// { 0x6c2a40 } (match_eng.cpp): casa (x, y) da formacao do jogador (tabela 0xf56e do lado, 216
/// bytes por posicao; nibble baixo e alto de um byte): campo 0x9f/0xa4 quando o lado em 0x8eae
/// (0 ou 1) e o do jogador; 0x30/0x3b quando e o outro; 0x35 com 0x8eae fora de 0/1. O byte e
/// relido para o segundo nibble, depois de gravar o primeiro.
pub(crate) unsafe extern "thiscall" fn casa_da_formacao(this: usize, jogador: usize, x: *mut u8, y: *mut u8) {
    let l = byte(this, 0x8eae);
    let jl = byte(jogador, 0x27);
    let campo = match l {
        0 if jl == 0 => 0x9f,
        0 => 0x30,
        1 if jl == 1 => 0xa4,
        1 => 0x3b,
        _ => 0x35,
    };
    let ler = || {
        let tabela = *((this as isize + 0xf56e + 4 * byte(jogador, 0x27) as isize) as *const isize);
        *((tabela + 216 * byte(jogador, 0x19) as isize + campo) as *const u8)
    };
    *x = ler() & 0xf;
    *y = ler() >> 4;
}

// ---------- decima nona leva ----------

/// Grava ftol(a / (a + b) x 100) e o complemento a 100 nos bytes `pa` e `pb`; 50 e 50 sem total positivo.
pub(crate) unsafe fn percentual(this: usize, a: i32, total: i32, pa: isize, pb: isize) {
    let (x, y) = if total > 0 {
        let v = ftol(a as f64 / total as f64 * f32_em(0x968960) as f64) as u8;
        (v, 100u8.wrapping_sub(v))
    } else {
        (50, 50)
    };
    *((this as isize + pa) as *mut u8) = x;
    *((this as isize + pb) as *mut u8) = y;
}

/// { 0x6ce9c0 } (match_eng.cpp): percentuais da partida (posse e afins) para os indices `i`, 3 e 4,
/// a partir das contagens em 0x8f02..0x8f66 (seis tabelas de dwords).
pub(crate) unsafe extern "thiscall" fn calcular_percentuais(this: usize, i: i8) {
    for s in [i as isize, 3, 4] {
        let c = |o: isize| *((this as isize + o + 4 * s) as *const i32);
        percentual(this, c(0x8f52), c(0x8f52).wrapping_add(c(0x8f16)), 0xf5dd + s, 0xf609 + s);
        percentual(this, c(0x8f2a), c(0x8f2a).wrapping_add(c(0x8f3e)), 0xf5d8 + s, 0xf60e + s);
        percentual(this, c(0x8f66), c(0x8f66).wrapping_add(c(0x8f02)), 0xf613 + s, 0xf5d3 + s);
        let a = c(0x8f02).wrapping_add(c(0x8f2a)).wrapping_add(c(0x8f52));
        let total = a.wrapping_add(c(0x8f66)).wrapping_add(c(0x8f3e)).wrapping_add(c(0x8f16));
        percentual(this, a, total, 0xf5ce + s, 0xf604 + s);
    }
}

/// { 0x6c2050 } (match_eng.cpp): terceira escolha ponderada pelo papel (0,4 / 0,45 / 0,5 / 0,75 /
/// 0,85; papel 0x40: 1,1 com 0x700, senao 1,0), ajustada pela pessoa (palavra 5 e sorteio contra o
/// byte 0x21). O sorteio base vem de atributos em float do jogador: com `modo`, 0xc9; senao, 1 em
/// 7, 0xa1 + c; 2 em 3 (dos restantes), uma soma ponderada de cinco; o resto, de tres. Nos dois
/// primeiros o fator nao entra. Acima de um limite, perde sorteio(5). O fator fica no argumento.
pub(crate) unsafe extern "thiscall" fn escolher_por_papel_c(this: usize, argumento: u32, modo: u32) -> u8 {
    let lado_b = argumento as i8;
    let mut fator = f32::from_bits(argumento);
    let mut melhor = 0.0f32;
    let mut escolhido: u8 = 0xff;
    for k in 0..20u8 {
        let p = jogador_do_bloco(this, lado_b, k as usize);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let papel = f717410(p);
        if papel & 1 != 0 {
            continue;
        }
        let bits = if papel & 2 != 0 {
            Some(0x3ecccccd)
        } else if papel & 4 != 0 {
            Some(0x3ee66666)
        } else if papel & 8 != 0 {
            Some(0x3f000000)
        } else if papel & 0x10 != 0 {
            Some(0x3f400000)
        } else if papel & 0x20 != 0 {
            Some(0x3f59999a)
        } else if papel & 0x40 != 0 {
            Some(if papel & 0x700 != 0 { 0x3f8ccccd } else { 0x3f800000 })
        } else {
            None
        };
        if let Some(b) = bits {
            fator = f32::from_bits(b);
        }
        let pessoa = *((p + 0x6d) as *const usize);
        if pessoa != 0 {
            let w = *((pessoa + 5) as *const i16);
            fator = if w > 0x8c {
                fator * f32_em(0x96d3d4)
            } else if w < 0x3c {
                fator * f32_em(0x96d3d0)
            } else {
                (w as f64 * f64_em(0x967898) * fator as f64) as f32
            };
            let r = sortear(byte(pessoa, 0x21) as i32);
            if r < sortear(0x14) {
                fator *= f32_em(0x96d3cc);
            }
        }
        let f = |o: usize| f32_em(p + o) as f64;
        let mut v = if modo as u8 != 0 {
            sortear(ftol(f(0xc9))) as f64
        } else if sortear(7) == 0 {
            sortear(ftol(f(0xa1) + f32_em(0x96b020) as f64)) as f64
        } else if sortear(3) != 0 {
            let x = f(0x9d) * f64_em(0x967880) + f(0x95) * f64_em(0x96a6a8) + f(0xb1) * f64_em(0x969510) + f(0x7d) * f64_em(0x969510) + f(0x99) + f64_em(0x969550);
            sortear(ftol(x)) as f64 * fator as f64
        } else {
            let x = f(0xd9) * f64_em(0x9689e0) + f(0xad) * f64_em(0x969040) + f(0xa9) + f32_em(0x968a78) as f64;
            sortear(ftol(x)) as f64 * fator as f64
        };
        if v > f32_em(0x96af88) as f64 {
            let v32 = v as f32;
            v = v32 as f64 - sortear(5) as f64;
        }
        if escolhido == 0xff || v > melhor as f64 {
            melhor = v as f32;
            escolhido = k;
        }
    }
    escolhido
}

// ---------- vigesima leva: condicao fisica (0x1a2) ----------

/// { 0x716450 } (match_pl.cpp): multiplica a condicao (0x1a2) pelo fator; o produto (em double) fica
/// no minimo em 2 x (25 - byte 0x5d da ficha 0x69) e, passando de um teto, vira 100.
pub(crate) unsafe extern "thiscall" fn aplicar_fator(this: usize, fator: f32) {
    let c = (this + 0x1a2) as *mut f32;
    let produto = fator as f64 * *c as f64;
    *c = produto as f32;
    let minimo = ((25 - byte(*((this + 0x69) as *const usize), 0x5d) as i32) * 2) as f32;
    let v = if produto > minimo as f64 { produto } else { minimo as f64 };
    *c = v as f32;
    if v > f64_em(0x9688a8) {
        *c = 100.0;
    }
}

/// { 0x716450 } pelo endereco do jogo (Rust ou original, conforme o modo).
pub(crate) unsafe fn f716450(p: usize, fator: f32) {
    let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(FN_MATCH_PLAYER_SCALE_ANGER);
    f(p, fator)
}

/// { 0x7165b0 } (match_pl.cpp): valor pela condicao e pelo byte 0xff; com `alvo` igual ao guardado
/// em 0x1a6, formula propria (0xff + 5).
pub(crate) unsafe extern "thiscall" fn valor_pela_condicao(this: usize, alvo: usize) -> i32 {
    let base = f32_em(this + 0x1a2) as f64 * f64_em(0x967898) + f64_em(0x969040);
    let b = byte(this, 0xff) as i32;
    if alvo != 0 && alvo == *((this + 0x1a6) as *const usize) {
        ftol(base * (b + 5) as f64 * f64_em(0x968e18))
    } else {
        ftol((base * b as f64 + f64_em(0x9688a0)) * f64_em(0x968968))
    }
}

/// { 0x6bb8d0 } (match_eng.cpp): aplica o fator ao jogador e um derivado aos colegas com ficha
/// (ativos: (f + c) x c'; inativos: f x c + c').
pub(crate) unsafe extern "thiscall" fn aplicar_ao_time(this: usize, jogador: usize, fator: f32) {
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado(jogador), k);
        if *((p + 0x69) as *const usize) == 0 {
            continue;
        }
        let f = if p == jogador {
            fator
        } else if byte(p, 0x19) >= 0 {
            ((fator as f64 + f64_em(0x967890)) * f64_em(0x968968)) as f32
        } else {
            (fator as f64 * f64_em(0x96a7a8) + f64_em(0x96d0e8)) as f32
        };
        f716450(p, f);
    }
}

/// { 0x6bb950 } (match_eng.cpp): aplica o fator a todos os jogadores com ficha do lado.
pub(crate) unsafe extern "thiscall" fn aplicar_ao_lado(this: usize, lado_b: i8, fator: f32) {
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if *((p + 0x69) as *const usize) != 0 {
            f716450(p, fator);
        }
    }
}

/// { 0x6c2310 } (match_eng.cpp): escolha pelo valor da condicao: maior sorteio(valor + 5, ou + 25
/// no modo 2) x fator (0,2 com papel 1, senao 1,0).
pub(crate) unsafe extern "thiscall" fn escolher_pela_condicao(this: usize, lado_b: i8, modo: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_GET_FOUL_CHANCE);
    let mut melhor = 0.0f32;
    let mut escolhido: u8 = 0xff;
    for k in 0..20u8 {
        let p = jogador_do_bloco(this, lado_b, k as usize);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let fator = if f717410(p) & 1 != 0 { f32::from_bits(0x3e4ccccd) } else { 1.0 };
        let v = f(p, 0) as i8 as i32 + if modo as u8 == 2 { 0x19 } else { 5 };
        let v = sortear(v) as f64 * fator as f64;
        if escolhido == 0xff || v > melhor as f64 {
            melhor = v as f32;
            escolhido = k;
        }
    }
    escolhido
}

// ---------- vigesima sexta leva: arbitragem pelo motor (objeto em 0xf5b2) ----------

/// { 0x6cfca0 } (match_eng.cpp): `decidir_falta` do arbitro do motor (this + 0xf5b2).
pub(crate) unsafe extern "thiscall" fn motor_decidir_falta(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_OFFICIAL_CHECK_FOUL);
    f(this + 0xf5b2, j, codigo, tipo, valor)
}

/// { 0x6cfcd0 } (match_eng.cpp): `punir_lance` do arbitro do motor.
pub(crate) unsafe extern "thiscall" fn motor_punir_lance(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_FOUL_PUNISHMENT);
    f(this + 0xf5b2, j, outro, tipo, x, sev, flag, peso, expulsar)
}

/// { 0x6cfd10 } (match_eng.cpp): `punir_falta` do arbitro do motor.
pub(crate) unsafe extern "thiscall" fn motor_punir_falta(this: usize, j: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_BOOKING);
    f(this + 0xf5b2, j, tipo, x, sev, flag, peso)
}

// ---------- vigesima nona leva ----------

/// { 0x6c23f0 } (match_eng.cpp): quinta escolha ponderada pelo papel (0,15 / 0,2 / 0,5 ou 0,3 /
/// 0,65 ou 0,8 / 1,05 / 0,95 ou 0,85), ajustada pela pessoa (palavra 5, bytes 0x37 e 0x43; sem
/// `modo`, a soma 0x40 + 0x36 + 0x26 + 0x1b; sorteio contra 0x21). Sorteio base: com `modo`,
/// 0xa1 - 0x99 - 0xad + c (sem fator); senao 4 em 5, 0xf1 + 0xc5 + 0x8d + c, ou 0xa9 + c. No fim,
/// 1 em 5 descarta a escolha (0xff). O fator fica no argumento e passa adiante.
pub(crate) unsafe extern "thiscall" fn escolher_por_papel_e(this: usize, argumento: u32, modo: u32) -> u8 {
    let lado_b = argumento as i8;
    let mut fator = f32::from_bits(argumento);
    let mut melhor = 0.0f32;
    let mut escolhido: u8 = 0xff;
    for k in 0..20u8 {
        let p = jogador_do_bloco(this, lado_b, k as usize);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let papel = f717410(p);
        if papel & 1 != 0 {
            continue;
        }
        let forte = papel & 0x880 != 0;
        let bits = if papel & 2 != 0 {
            Some(0x3e19999a)
        } else if papel & 4 != 0 {
            Some(0x3e4ccccd)
        } else if papel & 8 != 0 {
            Some(if forte { 0x3f000000 } else { 0x3e99999a })
        } else if papel & 0x10 != 0 {
            Some(if forte { 0x3f266666 } else { 0x3f4ccccd })
        } else if papel & 0x20 != 0 {
            Some(0x3f866666)
        } else if papel & 0x40 != 0 {
            Some(if papel & 0x700 != 0 { 0x3f733333 } else { 0x3f59999a })
        } else {
            None
        };
        if let Some(b) = bits {
            fator = f32::from_bits(b);
        }
        let pessoa = *((p + 0x6d) as *const usize);
        if pessoa != 0 {
            let w = *((pessoa + 5) as *const i16);
            fator = if w > 0x8c {
                fator * f32_em(0x96d3d4)
            } else if w < 0x3c {
                fator * f32_em(0x96d3d0)
            } else {
                (w as f64 * f64_em(0x967898) * fator as f64) as f32
            };
            for o in [0x37, 0x43] {
                let a = byte(pessoa, o);
                if a > 15 {
                    fator *= f32_em(0x96a33c);
                } else if a < 6 {
                    fator *= f32_em(0x96d38c);
                } else if a < 10 {
                    fator *= f32_em(0x96b014);
                }
            }
            if modo as u8 == 0 {
                let t = [0x40, 0x36, 0x26, 0x1b].iter().fold(0u8, |t, &o| t.wrapping_add(byte(pessoa, o) as u8)) as i8;
                if t > 0x4b {
                    fator *= f32_em(0x96b010);
                } else if t > 0x3c {
                    fator *= f32_em(0x96d3b8);
                } else if t < 0x19 {
                    fator = (fator as f64 * f64_em(0x969040)) as f32;
                } else if t < 0x28 {
                    fator *= f32_em(0x96b014);
                }
            }
            let r = sortear(byte(pessoa, 0x21) as i32);
            if r < sortear(0x14) {
                fator *= f32_em(0x96d248);
            }
        }
        let f = |o: usize| f32_em(p + o) as f64;
        let v = if modo as u8 != 0 {
            sortear(ftol(f(0xa1) - f(0x99) - f(0xad) + f32_em(0x969560) as f64)) as f64
        } else if sortear(5) != 0 {
            sortear(ftol(f(0xf1) + f(0xc5) + f(0x8d) + f32_em(0x968990) as f64)) as f64 * fator as f64
        } else {
            sortear(ftol(f(0xa9) + f32_em(0x968990) as f64)) as f64 * fator as f64
        };
        if escolhido == 0xff || v > melhor as f64 {
            melhor = v as f32;
            escolhido = k;
        }
    }
    if sortear(5) != 0 { escolhido } else { 0xff }
}

// ---------- trigesima leva: estatistica de posse por zona ----------

/// Por zona da bola (y <= 3, < 8, o resto): contadores do lado 0 e do lado 1 (por periodo, total,
/// total da janela, janela de bytes) e as palavras da zona (por periodo, total, total da janela).
pub(crate) const ZONAS_DE_POSSE: [([usize; 4], [usize; 4], [usize; 3]); 3] = [
    ([0x8f52, 0x8f5e, 0x8f62, 0x8e2d], [0x8f16, 0x8f22, 0x8f26, 0x8d88], [0x1f6, 0x1fc, 0x1fe]),
    ([0x8f2a, 0x8f36, 0x8f3a, 0x8dbf], [0x8f3e, 0x8f4a, 0x8f4e, 0x8df6], [0x200, 0x206, 0x208]),
    ([0x8f02, 0x8f0e, 0x8f12, 0x8d51], [0x8f66, 0x8f72, 0x8f76, 0x8e64], [0x20a, 0x210, 0x212]),
];

/// { 0x6cf750 } (match_eng.cpp): conta um instante de posse do `lado` na zona da bola, por periodo
/// (0x8ed4: 0x1ef, 0x3de ou outro). Com `janela`, mantem tambem os totais dos ultimos 55 instantes
/// (janela circular em 0x8e9b): tira o que sai, poe o que entra. Sem janela, lado -1 nao conta.
pub(crate) unsafe extern "thiscall" fn estatistica_de_posse(this: usize, lado_b: i8, janela: u8) {
    let w16 = |o: isize| (this as isize + o) as *mut u16;
    let w32 = |o: isize| (this as isize + o) as *mut i32;
    let b8 = |o: isize| (this as isize + o) as *mut i8;
    let mais16 = |o: isize| *w16(o) = (*w16(o)).wrapping_add(1);
    let mais32 = |o: isize| *w32(o) = (*w32(o)).wrapping_add(1);
    let per = match *w16(0x8ed4) {
        0x1ef => 0,
        0x3de => 1,
        _ => 2,
    };
    mais16(0x214 + 2 * per);
    mais16(0x21a);
    let y = byte(this, 0x8ea8);
    let z = if y <= 3 { 0 } else if y < 8 { 1 } else { 2 };
    let (l0, l1, pal) = ZONAS_DE_POSSE[z];
    let contadores = match lado_b {
        0 => Some(l0),
        1 => Some(l1),
        _ => None,
    };
    if janela == 0 {
        if lado_b == -1 {
            return;
        }
        if let Some(c) = contadores {
            mais32(c[0] as isize + 4 * per);
            mais32(c[1] as isize);
        }
        mais16(pal[0] as isize + 2 * per);
        mais16(pal[1] as isize);
        return;
    }
    let i = byte(this, 0x8e9b) as isize;
    for (o, b) in [(0x1fe, 0xa), (0x208, 0x41), (0x212, 0x78)] {
        *w16(o) = (*w16(o)).wrapping_sub(*b8(b + i) as i16 as u16);
    }
    let jan = [(0x8f12, 0x8d51), (0x8f26, 0x8d88), (0x8f3a, 0x8dbf), (0x8f4e, 0x8df6), (0x8f62, 0x8e2d), (0x8f76, 0x8e64)];
    for (o, b) in jan {
        *w32(o) = (*w32(o)).wrapping_sub(*b8(b + i) as i32);
    }
    for (_, b) in jan {
        *b8(b + i) = 0;
    }
    if let Some(c) = contadores {
        mais32(c[0] as isize + 4 * per);
        mais32(c[1] as isize);
        mais32(c[2] as isize);
        *b8(c[3] as isize + i) = 1;
    }
    mais16(pal[0] as isize + 2 * per);
    mais16(pal[1] as isize);
    mais16(pal[2] as isize);
    for (k, b) in [0xa, 0x41, 0x78].into_iter().enumerate() {
        *b8(b + i) = (k == z) as i8;
    }
    let n = (i as u8).wrapping_add(1) as i8;
    *b8(0x8e9b) = if n >= 0x37 { 0 } else { n };
}

// ---------- trigesima primeira leva: resumo dos lados ----------

/// Resumo de cada lado (0xf5c0 + 0x36 x lado): nota de cada um dos 20 jogadores do bloco
/// (0x1b = (disposicao + 500) / 1000, em 1..=10) e somas de atributos (bytes e palavras).
pub(crate) unsafe fn resumir_lados(motor: usize) {
    for lado_b in 0..2usize {
        let c = motor + 0xf5c0 + 0x36 * lado_b;
        let b8 = |o: isize| (c as isize + o) as *mut u8;
        let w16 = |o: usize| (c + o) as *mut u16;
        for o in [2, 0, 1, -2, -1] {
            *b8(o) = 0;
        }
        for o in (0x22..=0x30).step_by(2) {
            *w16(o) = 0;
        }
        *b8(0xb) = 0;
        *b8(0xc) = 0;
        for k in 0..20 {
            let p = motor + 0x4796 + 0x22d8 * lado_b + 0x1be * k;
            let nota = (p + 0x1b) as *mut i8;
            *nota = ((*((p + 0x35) as *const i16) as i32 + 500) / 1000) as i8;
            if *nota > 10 {
                *nota = 10;
            }
            if *nota < 1 {
                *nota = 1;
            }
            let u = |o: usize| *((p + o) as *const u8);
            for (dst, src) in [(2isize, 0xf), (0, 0xd), (1, 0xe), (-2, 0xa), (-1, 0xb)] {
                *b8(dst) = (*b8(dst)).wrapping_add(u(src));
            }
            for (dst, src) in [(0x22, 1), (0x24, 2), (0x26, 4), (0x28, 5), (0x2a, 7), (0x2c, 8), (0x2e, 9), (0x30, 0x12)] {
                *w16(dst) = (*w16(dst)).wrapping_add(u(src) as u16);
            }
            *b8(0xb) = (*b8(0xb)).wrapping_add(u(0x23));
            *b8(0xc) = (*b8(0xc)).wrapping_add(u(0x24));
        }
    }
}

/// { 0x6ce460 } (match_eng.cpp): resumo da partida no `momento`: 0 e 1 sorteiam o ritmo (0xf628 e
/// 0xf62c: 3..=9; 0xf630: sorteio(5) + estados dos lados, minimo 10, senao novo sorteio) e
/// resumem os lados; 0x3de so resume. Sempre recalcula os percentuais do periodo (0x8ed4).
pub(crate) unsafe extern "thiscall" fn resumo_da_partida(this: usize, momento: i16) {
    let per = match *((this + 0x8ed4) as *const u16) {
        0x1ef => 0i8,
        0x3de => 1,
        _ => 2,
    };
    match momento {
        0 | 1 => {
            let f = |o: usize| (this + o) as *mut f32;
            *f(0xf628) = ((sortear(7) as u8).wrapping_add(3) as i8) as f32;
            *f(0xf62c) = ((sortear(7) as u8).wrapping_add(3) as i8) as f32;
            let estados = |r: i32| r + *((this + 0xf5bd) as *const u8) as i32 + *((this + 0xf5f3) as *const u8) as i32;
            let t = estados(sortear(5));
            let t = if t < 10 { 10 } else { estados(sortear(5)) };
            *f(0xf630) = (t as u8 as i8) as f32;
            resumir_lados(this);
        }
        0x3de => resumir_lados(this),
        _ => {}
    }
    let percentuais: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_COMPUTE_POSSESSION_PERCENTAGES);
    percentuais(this, per);
}

// ---------- trigesima segunda leva ----------

/// { 0x6c30e0 } (match_eng.cpp, laco desenrolado 20 vezes no original): o jogador ativo do `lado`
/// mais perto de (a, b) pela tabela (`f6f09b0`), sem `tabela` 1 e, sem `todos`, so com 0x2b <= 1.
/// Distancia zero devolve na hora; partindo de 5000, o menor estritamente. Sem candidato, 0.
pub(crate) unsafe extern "thiscall" fn mais_perto_no_lado(this: usize, lado_b: i8, a: i8, b: i8, todos: u8) -> usize {
    let mut melhor = 5000.0f32;
    let mut escolhido = 0;
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) < 0 || f717410(p) == 1 || (todos == 0 && byte(p, 0x2b) > 1) {
            continue;
        }
        let d = f6f09b0(p, a, b);
        if d == 0.0 || d.is_nan() {
            return p;
        }
        if d < melhor {
            melhor = d;
            escolhido = p;
        }
    }
    escolhido
}

// ---------- trigesima quinta leva: escolher a casa para onde ir ----------

/// Pontua a casa (x, y) para o jogador em `escolher_casa`; None quando a casa nem entra.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn pontuar_casa(motor: usize, j: usize, x: i8, y: i8, v: f32, d: i8, flags: u8, alvo: (i8, i8)) -> Option<f64> {
    let (jx, jy) = (byte(j, 0x102), byte(j, 0x103));
    let flag2 = flags & 2 != 0;
    if x < 0 || x >= 9 || (jx == x && jy == y && flag2) {
        return None;
    }
    if flag2 {
        let virado: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_CELL_WITHIN_TURN_ARC);
        if virado(j, x, y) == 0 {
            return None;
        }
    }
    let t = tabela_por_posicao(motor, j, 0x8ebc);
    if t as u8 & 0x30 != 0 && t & 0x880 != 0 && (2..=6).contains(&x) && !(2..=6).contains(&jx) && flags & 1 == 0 {
        return None;
    }
    let casa = casa_da_grade(motor, x, y);
    let opp = (lado(j) == 0) as isize;
    let mut bl: i8 = 0;
    let mut base = v;
    if flag2 {
        bl = *((casa + 0x58 + opp) as *const i8);
        if bl != 0 {
            let pessoa = *((j + 0x6d) as *const usize);
            let r = sortear(byte(pessoa, 0x26) as i32);
            if r > sortear(3) {
                bl = (bl as i32 - 2).max(0) as i8;
            }
        }
        base = (v as f64 - (bl as f64 - f64_em(0x968968))) as f32;
        let l = lado(j);
        let ajusta = match byte(j, 0x2c) {
            1 => if l == 0 { x < jx || (x == jx && x < 4) } else { x > jx || (x == jx && x > 4) },
            0 => if l == 0 { x > jx || (x == jx && x > 4) } else { x < jx || (x == jx && x < 4) },
            _ => false,
        };
        if ajusta {
            base = (base as f64 - byte(j, 0x2d) as f64 * f64_em(0x968fd0)) as f32;
        }
    } else {
        let n = *((casa + 0x58 + opp) as *const i8);
        for i in 0..n.max(0) as isize {
            let p = *((casa + 4 * (11 * opp + i)) as *const usize);
            if byte(p, 0x19) >= 0 && byte(p, 0x2b) < 2 {
                bl = bl.wrapping_add(1);
            }
        }
    }
    let mut s = if bl != 0 && !flag2 {
        base as f64 - 2.0 * (bl as f64 * bl as f64)
    } else {
        let mut melhor = 5000.0f32;
        let mut zero = false;
        for k in 0..20 {
            let p = jogador_do_bloco(motor, opp as i8, k);
            if byte(p, 0x19) < 0 || f717410(p) & 1 != 0 {
                continue;
            }
            let dist = f6f09b0(p, x, y);
            if dist == 0.0 || dist.is_nan() {
                zero = true;
                break;
            }
            if dist < melhor {
                melhor = dist;
            }
        }
        let m = if zero { f32_em(0x968970) } else { melhor } as f64;
        m * m + base as f64
    };
    let (l, yj) = (lado(j), byte(j, 0x103));
    let mut volta = false;
    if (l == 0 && y < yj) || (l == 1 && y > yj) {
        if flags & 8 == 0 {
            s -= f64_em(0x96afa0);
            volta = true;
        }
    } else {
        s += if y == jy { f64_em(0x968968) } else { f64_em(0x968938) };
    }
    if d > 0 {
        s -= (d as i32 * 4) as f64;
    }
    if volta {
        return Some(s);
    }
    if (!flag2 || flags & 1 != 0) && !(3..=8).contains(&y) && x < 6 && x > 2 {
        s += f64_em(0x968e38);
    }
    if x == 4 || (x > 2 && jx < 4 && jx >= x) || (x < 6 && jx > 4 && jx <= x) {
        s += f64_em(0x968930);
    }
    let (a, b) = (jx as i32 - 4, x as i32 - 4);
    if !flag2 {
        if jx > 4 && a < b {
            s -= (3 * a) as f64;
        } else if jx < 4 && a > b {
            s -= (3 * (4 - jx as i32)) as f64;
        }
    } else {
        if !(2..=6).contains(&jx) && !(2..=6).contains(&x) && flags & 1 == 0 {
            s += f32_em(j + 0xe1) as f64 * f64_em(0x9689e0);
        }
        if jx >= 4 && a < b {
            s -= a as f64 * f64_em(0x968e18);
        } else if jx <= 4 && a > b {
            s -= (4 - jx as i32) as f64 * f64_em(0x968e18);
        }
        if alvo == (x, y) {
            s += f32_em(0x96b008) as f64;
        }
    }
    Some(s)
}

/// { 0x6bce80 } (match_eng.cpp): escolhe a casa vizinha (3 x 3 em volta do jogador) com a maior
/// pontuacao, partindo de 7,5 (empate fica com a ultima): adversarios na casa ou o mais perto
/// dela, recuo, linha de impedimento (flag 4), faixa central e o alvo anterior (`x`, `y` na entrada,
/// com flag 2). Grava a casa (ou -1) e devolve se achou. Sem flag 2, o jogador sem 0x107 nem 0x105
/// nao escolhe (devolve 0 sem gravar); com 0x105, um sorteio de -5..5 entra na base.
pub(crate) unsafe extern "thiscall" fn escolher_casa(this: usize, j: usize, sx: *mut i8, sy: *mut i8, flags: u32) -> u8 {
    let motor = this;
    let fl = flags as u8;
    let (jx, jy) = (byte(j, 0x102), byte(j, 0x103));
    let mut v = 0.0f32;
    if fl & 2 == 0 && byte(j, 0x107) == 0 {
        if byte(j, 0x105) == 0 {
            return 0;
        }
        let r = sortear(6);
        v = (r - sortear(6)) as f32;
    }
    let alvo = (*sx, *sy);
    let (mut bx, mut by) = (-1i8, -1i8);
    let mut melhor = 7.5f32;
    let (y0, y1) = (jy.wrapping_sub(1), jy.wrapping_add(1));
    let mut y = y0;
    while y0 <= y1 {
        'linha: {
            if !(0..12).contains(&y) {
                break 'linha;
            }
            let mut d: i8 = -10;
            if fl & 4 != 0 {
                let l = lado(j);
                d = if byte(j, 0x107) < 2 {
                    let base = if l == 1 { byte(motor, 0x8eb0).wrapping_sub(y) } else { y.wrapping_sub(byte(motor, 0x8eaf)) };
                    let r1 = sortear(3) as i8;
                    let r2 = sortear(3) as i8;
                    base.wrapping_sub(r1).wrapping_add(r2)
                } else if l == 1 {
                    byte(motor, 0x8eb0).wrapping_sub(y)
                } else {
                    y.wrapping_sub(byte(motor, 0x8eaf))
                };
                if byte(j, 0x106) != 0 {
                    let i = (lado(j) == 0) as usize;
                    if *((motor + 6371 * i + 0x9766) as *const u32) & 0x400 != 0 {
                        d = d.wrapping_add(1);
                    }
                }
                if d > 2 {
                    break 'linha;
                }
            }
            let (x0, x1) = (jx.wrapping_sub(1), jx.wrapping_add(1));
            let mut x = x0;
            while x0 <= x1 {
                if let Some(s) = pontuar_casa(motor, j, x, y, v, d, fl, alvo) {
                    if !(s < melhor as f64) && !s.is_nan() {
                        bx = x;
                        by = y;
                        melhor = s as f32;
                    }
                }
                x = x.wrapping_add(1);
                if x > x1 {
                    break;
                }
            }
        }
        y = y.wrapping_add(1);
        if y > y1 {
            break;
        }
    }
    *sy = by;
    *sx = bx;
    (bx > -1) as u8
}

// ---------- trigesima setima leva: tracado do lance (com chamadas internas gravadas) ----------

/// Chamada a `procurar_no_lado` (0x6cd330) feita pelo tracado: (lado, casa, distancia, a4, tipo, a6).
pub(crate) type ChamarProcurar<'a> = &'a mut dyn FnMut(i8, usize, u32, u32, u32, u32) -> u8;

/// { 0x6ccb60 } com a chamada a 0x6cd330 injetada: percorre as casas da linha de (x0, y0) a
/// (x1, y1) (eixo maior, erro acumulado como Bresenham; a casa final so entra se a linha for um
/// ponto) e, em cada uma, pergunta a `procurar_no_lado` por um jogador de um lado sorteado e
/// depois do outro, com a distancia desde a origem (tabela 12 x 9; fora, 1000). Primeiro que
/// achar, 1; sair do campo ou chegar ao fim, 0. O tipo (por `codigo`) vai junto.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn tracar_com(motor: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32, chamar: ChamarProcurar) -> u8 {
    let tipo: u8 = match codigo {
        9 | 62 | 90 => {
            let r = sortear(*(&a6 as *const u32 as *const i16) as i32 + 0xbb8);
            (r != 0) as u8 + 4
        }
        31 | 54 | 60 | 61 => sortear(4) as u8 + 1,
        6 | 8 | 30 | 52 | 55 | 91 | 92 | 94 => 4,
        _ => 0,
    };
    let ddx = (x1 as i32 - x0 as i32).abs() as i8;
    let ddy = (y1 as i32 - y0 as i32).abs() as i8;
    let sinal = |v: i32| if v >= 0 { 1i8 } else { -1 };
    // (eixo x e o maior?, passo do eixo maior, passo do menor, delta maior, delta menor)
    let (eixo_x, passo, passo_menor) = if ddx >= ddy {
        if x0 < x1 { (true, 1i8, sinal(y1 as i32 - y0 as i32)) } else { (true, -1, sinal(y1 as i32 - y0 as i32)) }
    } else if y0 < y1 {
        (false, 1, sinal(x1 as i32 - x0 as i32))
    } else {
        (false, -1, sinal(x1 as i32 - x0 as i32))
    };
    let (dmaior, dmenor) = if eixo_x { (ddx, ddy) } else { (ddy, ddx) };
    let (mut x, mut y) = (x0, y0);
    let mut erro: i8 = 0;
    let fora = |x: i8, y: i8| !(0..=8).contains(&x) || !(0..=11).contains(&y);
    loop {
        if fora(x, y) {
            return 0;
        }
        if erro >= dmaior {
            erro = erro.wrapping_sub(dmaior);
            if eixo_x { y = y.wrapping_add(passo_menor) } else { x = x.wrapping_add(passo_menor) }
        }
        let alvo = x == x1 && y == y1 && (ddx != 0 || ddy != 0);
        if !alvo {
            if fora(x, y) {
                return 0;
            }
            let casa = casa_da_grade(motor, x, y) as usize;
            let r = sortear(2) as i8;
            let ex = (x as i32 - x0 as i32).abs() as i8;
            let ey = (y as i32 - y0 as i32).abs() as i8;
            let dist = if (0..=8).contains(&ex) && (0..=11).contains(&ey) { *((0xa15e44 + 4 * (ey as usize * 9 + ex as usize)) as *const u32) } else { 0x447a0000 };
            if chamar(r, casa, dist, a6, tipo as u32, a7) != 0 || chamar((r == 0) as i8, casa, dist, a6, tipo as u32, a7) != 0 {
                return 1;
            }
        }
        erro = erro.wrapping_add(dmenor);
        if eixo_x {
            x = x.wrapping_add(passo);
            if (passo > 0 && x > x1) || (passo < 0 && x < x1) {
                return 0;
            }
        } else {
            y = y.wrapping_add(passo);
            if (passo > 0 && y > y1) || (passo < 0 && y < y1) {
                return 0;
            }
        }
    }
}

/// { 0x6ccb60 } (match_eng.cpp): tracado do lance de (x0, y0) a (x1, y1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn tracar_lance(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_TRY_CELL_INTERCEPTION);
    tracar_com(this, x0, y0, x1, y1, codigo, a6, a7, &mut |l, c, d, a, t, b| f(this, l, c, d, a, t, b, 1))
}

// ---------- centesima vigesima segunda leva: as trocas marcadas (0x6d0b60) ----------

/// { 0x6d0b60 } (match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70 narra,
/// 0x6e1090 e 0x6e1aa0 da comissao, 0x897760 grava o capitao): aplica as trocas marcadas (+0x1d) dos
/// dois lados. Quem esta em campo (ou -3) e marcado: com +0x1c sai (em campo: narra 0x1f56, 0x6cdd50
/// com -1 ou -2 conforme +0x9049 do lado, +0x1a = posicao, +0x19 = -2; com -3: narra 0x1f57 e 0x6cdd50
/// com 0) e grava o minuto em +0x22 (no maximo o fim do periodo); sem +0x1c narra 0x1f58 por sorteio(4)
/// ou com +0x55. Com +0x55 narra 0x20f6 (os dois e o lado de cada um, 0x714d30). Havendo marcados, fora
/// dos jogos com tecnico humano que nao delega (como na 0x6cda20): com +0x9049 a comissao substitui
/// (0x6e1090); senao refaz o time (0x6e1aa0), e se o capitao (0x897590) saiu de campo o mais bem
/// avaliado (0x6e1a40) vira capitao (0x897760), zera +0xf59e/+0xf5a2 e recompoe (0x6ce110). Guarda o
/// minuto da troca (+0x8ecc + 2 x lado), refaz a forca do lado (0x6bbb60) e a marcacao (0x6bbf60). No
/// fim zera +0x8eb5. Na fita, o 6o argumento da narracao vai no byte 1 do 5o (a 0x6d6f70 le os dois
/// como bytes; so o byte 0 e conferido).
pub(crate) unsafe fn aplicar_trocas_com(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tirar: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REMOVE_PLAYER_FROM_PITCH);
    let lado_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_SIDE);
    let humano_9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let humano_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let vaga_marcada: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
    let melhor: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(FN_MATCH_MAN_FIND_CAPTAIN_SLOT);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let forca: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_RECALC_SIDE_SCORE);
    let marcacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_ASSIGN_MARKING_TARGETS);
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    for lado in 0..2i8 {
        let lu = lado as usize;
        let mut marcados = 0u8;
        for k in 0..20 {
            let j = m + 0x4796 + 0x22d8 * lu + 0x1be * k;
            let e = byte(j, 0x19);
            if !(e >= 0 || e == -3) || byte(j, 0x1d) == 0 {
                continue;
            }
            marcados = marcados.wrapping_add(1);
            let nome = |p: usize| *((p + 0x2e) as *const u8) as u32;
            if byte(j, 0x1c) != 0 {
                if e >= 0 {
                    let a = nome(j);
                    chamar(0x6d6f70, u(m, 0x8f7a), [0x1f56, m as u32, a, lado_de(j) as u32, u32::MAX]);
                    tirar(m, j, if byte(m, 0x9049 + 0xd0 * lu) != 0 { -1 } else { -2 });
                    *((j + 0x1a) as *mut i8) = byte(j, 0x19);
                    *((j + 0x19) as *mut i8) = -2;
                } else {
                    let a = nome(j);
                    chamar(0x6d6f70, u(m, 0x8f7a), [0x1f57, m as u32, a, lado_de(j) as u32, u32::MAX]);
                    tirar(m, j, 0);
                }
                *((j + 0x22) as *mut i8) = minuto_do_periodo(m);
            } else if sortear(4) == 0 || u(j, 0x55) != 0 {
                let a = nome(j);
                chamar(0x6d6f70, u(m, 0x8f7a), [0x1f58, m as u32, a, lado_de(j) as u32, u32::MAX]);
            }
            let p55 = u(j, 0x55);
            if p55 != 0 {
                let (a5, a3) = (nome(p55), nome(j));
                let a6 = lado_de(p55) as u32;
                let a4 = lado_de(j) as u32;
                chamar(0x6d6f70, u(m, 0x8f7a), [0x20f6, m as u32, a3, a4, a5 | a6 << 8]);
            }
            *((j + 0x1d) as *mut u8) = 0;
        }
        if marcados == 0 {
            continue;
        }
        let base = m + 0xd0 * lu;
        let t = u(m, 0x4792);
        let clube_do = |l: i8| if l == 0 { u(t, 0x1c) } else { u(t, 0x20) };
        let p = u(base, 0x904d);
        let q = *(0xae2364 as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 && humano_9a01(u(0xb63c98, 0), p) == 0 {
            if byte(base, 0x904b) == 0 || selecao(clube_do(lado)) != 0 || humano_9a05(u(0xb63c98, 0), u(base, 0x904d)) != 0 {
                continue;
            }
        }
        let comissao = base + 0x9032;
        if byte(base, 0x9049) != 0 {
            chamar(0x6e1090, comissao, [0, 0, 0, 0, 0]);
        } else {
            chamar(0x6e1aa0, comissao, [0, 0, 0, 0, 0]);
            let principal = if lado != 0 { byte(t, 0x41) } else { byte(t, 0x40) } == 0;
            let v = vaga_marcada(u(0xde1d08, 0), clube_do(lado), principal as u32, 0);
            let jv = (m as isize + 0x4796 + 0x1be * (v as isize + 20 * lado as isize)) as usize;
            if byte(jv, 0x19) < 0 {
                let principal = if lado != 0 { byte(t, 0x41) } else { byte(t, 0x40) } == 0;
                let k = melhor(comissao);
                chamar(0x897760, u(0xde1d08, 0), [clube_do(lado) as u32, k as u8 as u32, principal as u32, 0, 0]);
            }
            *((m + if lado == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;
            manter(m, lado);
        }
        *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed0) as *const i16);
        forca(m, lado);
        marcacao(m);
    }
    *((m + 0x8eb5) as *mut u8) = 0;
}

/// applies both sides' marked substitutions (+0x1d). With +0x1c the player goes off (on the pitch: narrates 0x1f56,
/// 0x6cdd50 with −1 or −2 by +0x9049, +0x1a = position and +0x19 = −2; with −3: narrates 0x1f57 and 0x6cdd50 with 0)
/// and the minute goes to +0x22; without +0x1c it narrates 0x1f58 by draw(4) or with +0x55; with +0x55 it narrates
/// 0x20f6 with each one's side (0x714d30). With marked players and outside matches with a human manager who doesn't
/// delegate: with +0x9049 the staff substitute (0x6e1090); otherwise they redo the team (0x6e1aa0), give the armband
/// (0x897760, the best rated from 0x6e1a40) if the captain (0x897590) went off, clear +0xf59e/+0xf5a2 and recompose
/// (0x6ce110). Stores the minute (+0x8ecc), redoes the side's strength (0x6bbb60) and the marking (0x6bbf60); at the
/// end clears +0x8eb5. Compared through the tape (narration, staff and captain as units); the 6th argument of narration
/// 0x20f6 goes in byte 1 of the 5th (0x6d6f70 reads both as bytes)
/// Original: `apply_substitutions` (0x6d0b60, match_eng.cpp (~1 KB), thiscall).
///
/// [pt] { 0x6d0b60 } (match_eng.cpp): ver `aplicar_trocas_com`.
pub(crate) unsafe extern "thiscall" fn aplicar_trocas(m: usize) {
    aplicar_trocas_com(m, &mut |rotina, this, a| {
        match rotina {
            0x6d6f70 => comentar(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]),
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_MAN_MAKE_SUBSTITUTIONS);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

// ---------- centesima vigesima terceira leva: as punicoes pendentes (0x6d1210) ----------

/// { 0x6d1210 } (ret 4, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra; dentro da 0x6cda20 e no fim, 0x6e1aa0/0x6e1090 da comissao e 0x897760 do capitao): na 1a
/// chamada (+0x8eb4 = 0) so prepara `primeiro` (0x716630 com 0) e marca +0x8eb4 = 1 se houver
/// pendentes. Depois aplica cada um dos +0x1a3 lances pendentes (11 bytes a partir de +0xb1: jogador,
/// outro, severidade, codigo, x): a punicao do arbitro (0x6ea4e0, tipo 1) da o cartao; 2 narra
/// 0x1ff9, 3 e 4 (expulsao) narram 0x1ff7/0x1ff8 e marcam +0x1cb do lado (goleiro: tabela 1); sem
/// cartao, severidade acima de sorteio(7) + 2 com o jogador em campo narra 0x210a. Narrando, antes vem
/// 0x20f5 (sorteio(severidade) > sorteio(20)) e 0x2109. Fora dos codigos 0x25, 0x2a..0x32, 0x60 e 0x61:
/// por sorteio(9) narra 0x20df (e 0x20e0 por sorteio(3); tipos 3 a 6 marcam +0x16 e 0x20d9 por
/// sorteio(7)), passa o peso da punicao aos do lado (0x716450: o proprio, os em campo (p + 1) / 2, os
/// de fora p / 6 + 0,83) e 0x716e10 (1 na expulsao); expulsao tira o jogador (0x6cda20). No fim, para
/// cada lado marcado e fora dos jogos com tecnico humano que nao delega: com +0x9049 a comissao troca o
/// goleiro (0x6e1090 com 1), senao refaz o time (0x6e1aa0), da a braçadeira ao mais bem avaliado
/// (0x6e1a40/0x897760), zera +0xf59e/+0xf5a2 e recompoe (0x6ce110); guarda o minuto e refaz a forca e
/// a marcacao. Termina com +0x1a3 = 0 e +0x8eb4 = 2.
pub(crate) unsafe fn punicoes_com(m: usize, primeiro: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let punir: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_FOUL_PUNISHMENT);
    let desgastar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(FN_MATCH_PLAYER_SCALE_ANGER);
    let humano_9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let humano_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let melhor: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(FN_MATCH_MAN_FIND_CAPTAIN_SLOT);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let forca: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_RECALC_SIDE_SCORE);
    let marcacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_ASSIGN_MARKING_TARGETS);
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    if byte(m, 0x8eb4) == 0 {
        chamar(0x716630, primeiro, [0; 5]);
        if byte(m, 0x1a3) != 0 {
            *((m + 0x8eb4) as *mut u8) = 1;
        }
        return;
    }
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, u32::MAX]);
    };
    let mut k = 0i8;
    while k < byte(m, 0x1a3) {
        let e = (m as isize + 11 * k as isize) as usize;
        let j = u(e, 0xb1);
        let tab = tabela(j);
        let mut tipo = 1u8;
        let mut peso = 0f32;
        punir(m + 0xf5b2, j, u(e, 0xb5), &mut tipo, byte(e, 0xbb), byte(e, 0xb9), 0, &mut peso, 0);
        let marcar_lado = || *((m as isize + byte(j, 0x27) as isize + 0x1cb) as *mut u8) = (tab == 1) as u8;
        let codigo: i32 = match tipo as i8 {
            2 => 0x1ff9,
            3 => {
                marcar_lado();
                0x1ff7
            }
            4 => {
                marcar_lado();
                0x1ff8
            }
            _ => {
                let r = sortear(7) + 2;
                if byte(e, 0xb9) as i32 > r && byte(j, 0x19) >= 0 { 0x210a } else { -1 }
            }
        };
        if codigo != -1 {
            let a = sortear(byte(e, 0xb9) as i32);
            if a > sortear(20) {
                narrar(chamar, 0x20f5, 0, (byte(j, 0x27) == 0) as u32);
            }
            narrar(chamar, 0x2109, *((j + 0x2e) as *const u8) as u32, *((j + 0x27) as *const u8) as u32);
            narrar(chamar, codigo as u32, *((j + 0x2e) as *const u8) as u32, *((j + 0x27) as *const u8) as u32);
        }
        let v = (byte(e, 0xba) as i32 - 0x25) as u32;
        let pula = v <= 0x3c && matches!(v, 0 | 5..=13 | 0x3b | 0x3c);
        let expulso = tipo == 4 || tipo == 3;
        if !pula && codigo != -1 {
            if sortear(9) == 0 {
                narrar(chamar, 0x20df, u32::MAX, *((j + 0x27) as *const u8) as u32);
                if sortear(3) == 0 {
                    narrar(chamar, 0x20e0, u32::MAX, *((j + 0x27) as *const u8) as u32);
                }
                if matches!(tipo, 3..=6) {
                    *((j + 0x16) as *mut u8) = 1;
                    if sortear(7) == 0 {
                        narrar(chamar, 0x20d9, u32::MAX, *((j + 0x27) as *const u8) as u32);
                    }
                }
            }
            let bloco = m + 0x4796 + 0x22d8 * byte(j, 0x27) as isize as usize;
            for i in 0..20 {
                let p = bloco + 0x1be * i;
                if u(p, 0x69) == 0 {
                    continue;
                }
                let f = if p == j {
                    peso
                } else if byte(p, 0x19) < 0 {
                    (peso as f64 * 0.16666666666666666 + 0.83) as f32
                } else {
                    ((peso as f64 + 1.0) * 0.5) as f32
                };
                desgastar(p, f);
            }
            if expulso {
                chamar(0x716e10, j, [1, 0, 0, 0, 0]);
                jogador_saiu_com(m, j, 0, chamar);
            } else {
                chamar(0x716e10, j, [0; 5]);
            }
        } else if expulso {
            jogador_saiu_com(m, j, 0, chamar);
        }
        k = k.wrapping_add(1);
    }
    *((m + 0x1a3) as *mut u8) = 0;
    for lado in 0..2i8 {
        let lu = lado as usize;
        if byte(m, 0x1cb + lu) != 0 {
            let base = m + 0xd0 * lu;
            let t = u(m, 0x4792);
            let clube_do = |l: i8| if l == 0 { u(t, 0x1c) } else { u(t, 0x20) };
            let p = u(base, 0x904d);
            let q = *(0xae2364 as *const i32);
            let pula = p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 && humano_9a01(u(0xb63c98, 0), p) == 0
                && (byte(base, 0x904b) == 0 || selecao(clube_do(lado)) != 0 || humano_9a05(u(0xb63c98, 0), u(base, 0x904d)) != 0);
            if !pula {
                let comissao = base + 0x9032;
                if byte(base, 0x9049) != 0 {
                    chamar(0x6e1090, comissao, [1, 0, 0, 0, 0]);
                } else {
                    chamar(0x6e1aa0, comissao, [0, 0, 0, 0, 0]);
                    let principal = if lado != 0 { byte(t, 0x41) } else { byte(t, 0x40) } == 0;
                    let k = melhor(comissao);
                    chamar(0x897760, u(0xde1d08, 0), [clube_do(lado) as u32, k as u8 as u32, principal as u32, 0, 0]);
                    *((m + if lado == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;
                    manter(m, lado);
                }
                *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed0) as *const i16);
                forca(m, lado);
                marcacao(m);
            }
        }
        *((m + 0x1cb + lu) as *mut u8) = 0;
    }
    *((m + 0x8eb4) as *mut u8) = 2;
}

/// { 0x6d1210 } (ret 4, match_eng.cpp): ver `punicoes_com`.
// Portas de 0x716630 (prepara) e 0x716e10 (cansa): as duas narram por dentro. Unidades da fita so na
// comparacao das punicoes (a 0x716e10 tambem roda dentro de muitas outras rotinas).
pub(crate) static GRAVAR_716: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static mut PORTA_716630: usize = 0;
pub(crate) const PROLOGO_716630: Prologo = Prologo { endereco: 0x716630, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] };
pub(crate) static mut PORTA_716E10: usize = 0;
pub(crate) const PROLOGO_716E10: Prologo = Prologo { endereco: 0x716e10, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] };
pub(crate) static mut PORTA_716C90: usize = 0;
pub(crate) const PROLOGO_716C90: Prologo = Prologo { endereco: 0x716c90, bytes: &[0x51, 0x56, 0x8b, 0xf1, 0x57] };

pub(crate) unsafe fn porta_716(rotina: usize, alvo: usize, p: usize, a1: u32) {
    let o: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(alvo);
    if !(GRAVAR_716.load(std::sync::atomic::Ordering::Relaxed) && gravando()) {
        return o(p, a1);
    }
    chamar_gravando(rotina, p, [a1, 0, 0, 0, 0], || {
        o(p, a1);
        0
    });
}
pub(crate) unsafe extern "thiscall" fn porta_716630(p: usize, a1: u32) {
    porta_716(0x716630, PORTA_716630, p, a1)
}
pub(crate) unsafe extern "thiscall" fn porta_716e10(p: usize, a1: u32) {
    porta_716(0x716e10, PORTA_716E10, p, a1)
}
pub(crate) unsafe extern "thiscall" fn porta_716c90(p: usize) {
    let o: unsafe extern "thiscall" fn(usize) = core::mem::transmute(PORTA_716C90);
    if !(GRAVAR_716.load(std::sync::atomic::Ordering::Relaxed) && gravando()) {
        return o(p);
    }
    chamar_gravando(0x716c90, p, [0; 5], || {
        o(p);
        0
    });
}

pub(crate) unsafe extern "thiscall" fn punicoes(m: usize, primeiro: usize) {
    punicoes_com(m, primeiro, &mut |rotina, this, a| {
        match rotina {
            0x6d6f70 => comentar(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]),
            0x716630 | 0x716e10 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
                f(this, a[0]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_MAN_MAKE_SUBSTITUTIONS);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

// ---------- centesima vigesima quarta leva: a disputa de penaltis (0x6be710) ----------

/// { 0x6be710 } (ret 4, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x702050 cobra, 0x7109f0 move o goleiro): um passo da disputa de penaltis pelo estado
/// +0x8eb3. 1 e 4 preparam (zeram o lance, guardam e calam o narrador em +0x8ea4, cinco cobrancas
/// por lado em +0x8ea1/+0x8ea2, comeca o lado 1 ou o sorteado, 0x6be520) e vao para 2 e 5. 2 e 5: sem
/// cobrancas sobrando vem a morte subita (uma de cada, +0x8ea3); o batedor e o da vez na lista do lado
/// (+0x1af + 11 x lado, +0x8e9f conta), o goleiro e o do outro lado (0x6ce110); marca quem bateu
/// (+0x8eda), a forca sai de sorteio(+0x35 da ficha) e das cobrancas que faltam, e cresce (x2,25 com
/// narracao 0x2005 quando o lado precisa marcar, x0,75 com 0x2006 quando pode decidir). Com `modo` 3
/// o chute (0x700bd0 e 0x702050) sai direto; senao posiciona batedor, goleiro e os outros (0x6f46b0,
/// 0x7160e0), a direcao do chute pela forca, e o goleiro reage (0x714d40, ate 4 vezes 0x7109f0) antes
/// do chute. Passa a vez e, decidido o placar (+0xf5bc x +0xf5f2 com as cobrancas que faltam), vai
/// para 3 (modo 4) ou 6. 3 e 6 narram o resultado (0x2004), gravam o placar dos penaltis na partida
/// (+0x49/+0x4a) e devolvem o narrador.
pub(crate) unsafe fn penaltis_com(m: usize, modo: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let preparar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_CHOOSE_SHOOTOUT_TAKERS);
    let goleiro_de: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let parar: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_SET_ACTION_DELAY);
    let marcar: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_ADD_ACTION_DELAY);
    let chute: unsafe extern "thiscall" fn(usize, *mut i8, *mut u16) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_SHOT_SCORE);
    let b = |o: usize| (m + o) as *mut u8;
    let u = |o: usize| *((m + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| {
        chamar(0x6d6f70, u(0x8f7a), [codigo, m as u32, a3, a4, u32::MAX]);
    };
    match byte(m, 0x8eb3) {
        1 | 4 => {
            let narrador = u(0x8f7a) as *mut u8;
            *((m + 0x475f) as *mut u16) = 0xffff;
            *((m + 0x4761) as *mut u32) = 0;
            *((m + 0x4765) as *mut u32) = 0;
            *b(0x4769) = 0;
            *b(0x475a) = 0;
            for o in 0x475b..=0x475e {
                *b(o) = 0xff;
            }
            *b(0x8ea4) = *narrador;
            *narrador = 0;
            *b(0x8e9e) = if byte(m, 0x8eb3) == 1 { 1 } else { sortear(2) as u8 };
            *b(0x1c5) = 0;
            for o in [0x477e, 0xf5aa, 0xf5a6, 0xf576, 0xf57a, 0x91de] {
                *((m + o) as *mut u32) = 0;
            }
            *b(0x8eaa) = 0;
            *b(0x8ea9) = 0;
            *b(0x8ea6) = 0xff;
            *b(0x8eb9) = 0xff;
            *((m + 0x8ed6) as *mut u16) = 0;
            *b(0x8ea2) = 5;
            *b(0x8ea1) = 5;
            *b(0x8ea0) = 0;
            *b(0x8e9f) = 0;
            let proximo = if byte(m, 0x8eb3) == 1 { 2 } else { 5 };
            preparar(m);
            *b(0x8ea3) = 0;
            *b(0x8eb3) = proximo;
        }
        2 | 5 => {
            *b(0x8eb2) = 0;
            if byte(m, 0x8ea1) == 0 && byte(m, 0x8ea2) == 0 {
                *b(0x8ea2) = 1;
                *b(0x8ea1) = 1;
                *b(0x8ea3) = 1;
            }
            let lado = byte(m, 0x8e9e);
            let lu = lado as isize;
            let vez = byte(m, (0x8e9f + lu) as usize) as i32 % 11;
            let quem = byte(m, (0x1af + 11 * lu + vez as isize) as usize) as isize;
            let j = (m as isize + 0x4796 + 0x1be * (quem + 20 * lu)) as usize;
            let gol = goleiro_de(m, (lado == 0) as i8);
            *b((0x8e9f + lu) as usize) = byte(m, (0x8e9f + lu) as usize).wrapping_add(1) as u8;
            parar(j, 0);
            parar(gol, 0);
            *((m as isize + byte(j, 0) as isize + 20 * lu + 0x8eda) as *mut u8) = 1;
            chamar(0x6f0630, j, [0; 5]);
            *b(0x8ea9) = 0;
            let restam = byte(m, (0x8ea1 + lu) as usize) as i32;
            let r = sortear(byte(*((j + 0x6d) as *const usize), 0x35) as i32);
            let mut forca = (0xc8 - r.wrapping_mul(restam + 4)) << 1;
            let gols = |l: isize| *((m as isize + 0xf5bc + 0x36 * l) as *const u8) as i32;
            let outro = (lado == 0) as isize;
            let (meus, deles) = (gols(lu), gols(outro));
            let fator = if deles > meus + restam - 1 {
                if byte(m, 0x8ea3) == 0 {
                    narrar(chamar, 0x2005, *((j + 0x2e) as *const u8) as u32, lado as u8 as u32);
                }
                Some(2.25)
            } else if meus + 1 > byte(m, (0x8ea1 + outro) as usize) as i32 + deles {
                if byte(m, 0x8ea3) == 0 {
                    narrar(chamar, 0x2006, *((j + 0x2e) as *const u8) as u32, lado as u8 as u32);
                }
                Some(0.75)
            } else {
                None
            };
            if let Some(f) = fator {
                forca = forca.wrapping_add(sortear(ftol(forca as f64 * f)));
            }
            let (mut tipo, mut som) = (0x33i8, 0u16);
            if modo as u8 == 3 {
                let v = chute(j, &mut tipo, &mut som) as i16 as i32;
                let r = sortear(forca);
                chamar(0x702050, j, [tipo as u8 as u32, v.wrapping_sub(r) as u32, som as u32, 3, 0]);
            } else {
                let (dir, a_j, a_g, a_o) = if lado == 0 { (0u16, 7u32, (4u32, 0xbu32), (0u32, 0u32)) } else { (0xb4, 4, (4, 0), (8, 0xb)) };
                *((j + 0x19a) as *mut u16) = dir;
                let mut v = 4u8;
                if byte(j, 0x2d) > 3 {
                    let s2 = sortear(2) as u8;
                    v = if (byte(j, 0x2c) == 0) == (lado == 0) { 4u8.wrapping_sub(s2) } else { s2.wrapping_add(4) };
                }
                chamar(0x6f46b0, j, [v as u32, a_j, 0, 0, 0]);
                chamar(0x6f46b0, gol, [a_g.0, a_g.1, 0, 0, 0]);
                let base = m + 0x4796;
                for k in 0..20 {
                    for p in [base + 0x1be * k, base + 0x22d8 + 0x1be * k] {
                        if p != j && p != gol && byte(p, 0x19) >= 0 {
                            chamar(0x6f46b0, p, [a_o.0, a_o.1, 0, 0, 0]);
                            parar(p, 10);
                        }
                    }
                }
                let c = (0xc8 - sortear(forca)) as i16;
                *((j + 0x19c) as *mut i16) = c.clamp(-0xc8, 0xc8);
                let mut defendeu = 0u8;
                let mut vezes = 0i8;
                loop {
                    chamar(0x714d40, gol, [0; 5]);
                    let ativo = byte(m, 0x8eb2) == 0 && *b(0x8ea9) != 0xff;
                    if ativo {
                        defendeu = chamar(0x7109f0, j, [4, 0, 0, 0, 0]) as u8;
                    }
                    marcar(j, 0xff);
                    marcar(gol, 0xff);
                    if !(byte(m, 0x8eb2) == 0 && *b(0x8ea9) != 0xff) {
                        break;
                    }
                    let n = vezes;
                    vezes = vezes.wrapping_add(1);
                    if n >= 4 || defendeu != 0 || u(0xf582) != j {
                        break;
                    }
                }
                if u(0xf582) == j && byte(m, 0x8eb2) == 0 && *b(0x8ea9) != 0xff {
                    tipo = -1;
                    let v = chute(j, &mut tipo, &mut som) as i16 as i32;
                    chamar(0x702050, j, [tipo as u8 as u32, v as u32, som as u32, 4, 0]);
                }
            }
            *b((0x8ea1 + lu) as usize) = byte(m, (0x8ea1 + lu) as usize).wrapping_sub(1) as u8;
            *b(0x8e9e) = (lado == 0) as u8;
            let (g0, g1) = (*b(0xf5bc) as i32, *b(0xf5f2) as i32);
            if g0 > byte(m, 0x8ea2) as i32 + g1 || g1 > byte(m, 0x8ea1) as i32 + g0 {
                *b(0x8eb3) = if modo as u8 == 4 { 3 } else { 6 };
            }
        }
        3 | 6 => {
            narrar(chamar, 0x2004, u32::MAX, (*b(0xf5bc) <= *b(0xf5f2)) as u32);
            let jogo = u(0x4792);
            *((jogo + 0x49) as *mut u8) = *b(0xf5bc);
            *((jogo + 0x4a) as *mut u8) = *b(0xf5f2);
            *b(0x8eb3) = 0;
            *(u(0x8f7a) as *mut u8) = *b(0x8ea4);
        }
        _ => {}
    }
}

/// { 0x6be710 } (ret 4, match_eng.cpp): ver `penaltis_com`.
pub(crate) unsafe extern "thiscall" fn penaltis(m: usize, modo: u32) {
    penaltis_com(m, modo, &mut |rotina, this, a| match rotina {
        0x6d6f70 => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_SHOT);
            f(this, a[0], a[1], a[2], a[3], 0, 0);
            0
        }
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(this, a[0], a[1]);
            0
        }
        0x6f0630 | 0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
            f(this);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_DECIDE_BALL_ACTION);
            f(this, a[0]) as u32
        }
    })
}

// ---------- centesima vigesima quinta leva: a falta marcada (0x6bac40) ----------

/// { 0x6bac40 } (match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70 narra,
/// 0x716e10 cansa): aplica o lance pendente em +0x475a (tipo; +0x4761 quem fez, +0x4765 quem sofreu,
/// +0x475b/+0x475c onde, +0x475d/+0x475e/+0x4769 para a punicao, +0x475f o codigo). Sem tipo devolve
/// 0. Com bola parada em andamento (+0x8eb2) e o lance do lado que ja tinha a vantagem (+0x8eae)
/// atras da posicao marcada (+0x8ea8), descarta o lance e devolve 0; senao narra 0x20f0. Tipo 8:
/// narra 0x20ee, conta no placar do lado (+0xf5ca) e no jogador (+0x13) e marca a bola parada 7. Sem
/// quem sofreu, a punicao da falta (0x6e9630) e a narracao 0x20fb; com ele, a punicao do lance
/// (0x6ea4e0) e a narracao pelo codigo +0x475f (0x20fc a 0x2107). O tipo devolvido escolhe o cartao
/// (0x1ff9 amarelo, 0x1ff7/0x1ff8 vermelho, 0x1ff5 ou 0x1ff6 conforme o caso, 0x210a pela severidade
/// contra sorteio(7) + 2). Tipo 7 e bola parada 7; perto da area adversaria (+0x475b de 2 a 6 e
/// +0x475c na faixa do gol) e penalti (5, 0x1f7a, -500 em +0x35 de quem sofreu ou fez); senao falta
/// (6, 0x1f79). Com cartao: 1 em 4 narra 0x2109 (e 0x20f5 pela severidade), narra o cartao e a
/// expulsao tira o jogador (0x6cda20). Fora dos codigos de bola parada, 1 em 9 narra 0x20df (1 em 3
/// tambem 0x20e0) e, com expulsao ou penalti, marca +0x16 de quem fez (1 em 7 narra 0x20d9); o time
/// de quem fez se desgasta pelo peso da punicao (0x716450: o proprio pelo peso, os de fora p / 6 +
/// 0,83, os outros (p + 1) / 2) e quem fez cansa (0x716e10). Limpa o lance e devolve 1.
pub(crate) unsafe fn aplicar_falta_com(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let punir_falta: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_BOOKING);
    let punir: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_FOUL_PUNISHMENT);
    let desgastar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(FN_MATCH_PLAYER_SCALE_ANGER);
    let b = |o: usize| (m + o) as *mut u8;
    let u = |o: usize| *((m + o) as *const usize);
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let limpar = || {
        *((m + 0x475f) as *mut u16) = 0xffff;
        *((m + 0x4761) as *mut usize) = 0;
        *((m + 0x4765) as *mut usize) = 0;
        *b(0x4769) = 0;
        *b(0x475a) = 0;
        for o in 0x475b..=0x475e {
            *b(o) = 0xff;
        }
    };
    if *b(0x475a) == 0 {
        return 0;
    }
    if *b(0x8eb2) != 0 {
        let lado = byte(u(0x4761), 0x27);
        if byte(m, 0x8eae) as i32 == (lado == 0) as i32 {
            let (marcada, aqui) = (byte(m, 0x8ea8), byte(m, 0x475c));
            if (aqui < marcada && lado == 1) || (marcada < aqui && lado == 0) {
                limpar();
                return 0;
            }
        }
        narrar(chamar, 0x20f0, u32::MAX, lado as u8 as u32, u32::MAX);
    }
    let j = u(0x4761);
    if *b(0x475a) == 8 {
        narrar(chamar, 0x20ee, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        let j = u(0x4761);
        let placar = (m as isize + byte(j, 0x27) as isize * 0x36 + 0xf5ca) as *mut u8;
        *placar = (*placar).wrapping_add(1);
        *((j + 0x13) as *mut u8) = ub(j, 0x13).wrapping_add(1) as u8;
        *b(0x8ea7) = *b(0x475b);
        *b(0x8ea8) = *b(0x475c);
        *b(0x8eb2) = 7;
        *b(0x8eae) = zero(j) as u8;
        *((m + 0xf582) as *mut u32) = 0;
        limpar();
        return 1;
    }
    if j == 0 {
        limpar();
        return 1;
    }
    let sofreu = u(0x4765) != 0;
    let mut peso = 0f32;
    if !sofreu {
        punir_falta(m + 0xf5b2, j, b(0x475a), byte(m, 0x475d), byte(m, 0x475e), *b(0x4769), &mut peso);
        let j = u(0x4761);
        narrar(chamar, 0x20fb, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
    } else {
        punir(m + 0xf5b2, j, u(0x4765), b(0x475a), byte(m, 0x475d), byte(m, 0x475e), *b(0x4769), &mut peso, 0);
        let codigo = match *((m + 0x475f) as *const i16) {
            1 => 0x2104,
            0xc => 0x2107,
            0x13 | 0x21 => 0x2101,
            0x15 => 0x2103,
            0x22 => 0x2105,
            0x29 => 0x2102,
            0x2a => 0x20fe,
            0x2b => 0x20ff,
            0x2c => 0x2100,
            0x2d => 0x20fd,
            0x2e => 0x2106,
            _ => 0x20fc,
        };
        let (j, v) = (u(0x4761), u(0x4765));
        narrar(chamar, codigo, ub(j, 0x2e), ub(j, 0x27), ub(v, 0x2e) | ub(v, 0x27) << 8);
    }
    let tipo = *b(0x475a) as i8;
    let cartao: i32 = match tipo {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        6 if !sofreu => 0x1ff5,
        5 if sofreu => 0x1ff6,
        _ => {
            if byte(m, 0x475e) as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    let som: u32;
    if *b(0x475a) == 7 {
        som = 0x1f79;
        *b(0x8eb2) = 7;
    } else {
        let (x, y) = (byte(m, 0x475b), byte(m, 0x475c));
        let area = (2..=6).contains(&x) && if byte(u(0x4761), 0x27) == 1 { (10..=11).contains(&y) } else { (0..=1).contains(&y) };
        if area {
            *b(0x8eb2) = 5;
            let quem = if sofreu { u(0x4765) } else { u(0x4761) };
            *((m + 0x478a) as *mut usize) = if sofreu { quem } else { 0 };
            let d = (quem + 0x35) as *mut i16;
            *d = (*d).wrapping_sub(500);
            som = 0x1f7a;
        } else {
            som = 0x1f79;
            *b(0x8eb2) = 6;
        }
    }
    *b(0x8ea8) = *b(0x475c);
    *b(0x8ea7) = *b(0x475b);
    *((m + 0xf582) as *mut u32) = 0;
    *b(0x8eae) = zero(u(0x4761)) as u8;
    if som == 0x1f7a {
        let a4 = if sofreu { ub(u(0x4765), 0x27) } else { zero(u(0x4761)) };
        narrar(chamar, 0x1f7a, u32::MAX, a4, u32::MAX);
    }
    if cartao >= 0 {
        if sortear(4) == 0 {
            let a = sortear(byte(m, 0x475e) as i32);
            if a > sortear(0x14) {
                narrar(chamar, 0x20f5, 0, 0, u32::MAX);
            }
            let j = u(0x4761);
            narrar(chamar, 0x2109, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        }
        let j = u(0x4761);
        narrar(chamar, cartao as u32, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        if matches!(*b(0x475a), 3..=6) {
            jogador_saiu_com(m, u(0x4761), 0, chamar);
        }
    }
    if som != 0x1f7a {
        let a4 = if sofreu { ub(u(0x4765), 0x27) } else { zero(u(0x4761)) };
        narrar(chamar, som, u32::MAX, a4, u32::MAX);
    }
    // os codigos de bola parada (0x25, 0x2a a 0x32, 0x60 e 0x61) param aqui
    let k = (byte(m, 0x475f) as i32 - 0x25) as u32;
    if k <= 0x3c && matches!(k, 0 | 5..=13 | 0x3b | 0x3c) {
        limpar();
        return 1;
    }
    // sem quem sofreu, o lado cru de quem fez; com ele, "lado == 0" (0x20df pelo lado de quem sofreu)
    let lado_de = |p: usize| if sofreu { zero(p) } else { ub(p, 0x27) };
    if sortear(9) == 0 {
        narrar(chamar, 0x20df, u32::MAX, lado_de(if sofreu { u(0x4765) } else { u(0x4761) }), u32::MAX);
        if sortear(3) == 0 {
            narrar(chamar, 0x20e0, u32::MAX, lado_de(u(0x4761)), u32::MAX);
        }
        if matches!(*b(0x475a), 3..=6) || som == 0x1f7a {
            *((u(0x4761) + 0x16) as *mut u8) = 1;
            if sortear(7) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, lado_de(u(0x4761)), u32::MAX);
            }
        }
    }
    let j = u(0x4761);
    let bloco = m + 0x4796 + 0x22d8 * byte(j, 0x27) as isize as usize;
    for i in 0..20 {
        let p = bloco + 0x1be * i;
        if *((p + 0x69) as *const usize) == 0 {
            continue;
        }
        let f = if p == j {
            peso
        } else if byte(p, 0x19) < 0 {
            (peso as f64 * 0.16666666666666666 + 0.83) as f32
        } else {
            ((peso as f64 + 1.0) * 0.5) as f32
        };
        desgastar(p, f);
    }
    chamar(0x716e10, u(0x4761), [0; 5]);
    limpar();
    1
}

/// applies the pending incident in +0x475a (type; +0x4761 who committed it, +0x4765 who suffered it, +0x475b/+0x475c
/// where, +0x475f the code). With no type it returns 0; with a set piece under way and the incident behind the marked
/// position (+0x8ea8) of the side that had the advantage, it discards it and returns 0 (otherwise it narrates 0x20f0).
/// Type 8: narrates 0x20ee, counts in the side's score (+0xf5ca) and the player's (+0x13), set piece 7. With no victim:
/// foul punishment (0x6e9630) and 0x20fb; with one: incident punishment (0x6ea4e0) and narration by code (0x20fc to
/// 0x2107). The returned type picks the card (0x1ff5 to 0x1ff9, or 0x210a by severity); near the opponent's box it
/// becomes a penalty (+0x8eb2 = 5, 0x1f7a, −500 in +0x35), otherwise a free kick (6, 0x1f79). With a card, 1 in 4
/// narrates 0x2109 (and 0x20f5), narrates the card and a sending-off takes the player off (0x6cda20). Outside the
/// set-piece codes: narrations 0x20df/0x20e0/0x20d9 by draw, the offender's team wears down by the punishment weight
/// (0x716450) and the offender tires (0x716e10). Clears the incident and returns 1. Compared through the tape:
/// narration and tiring (0x716e10) are units
/// Original: `apply_foul` (0x6bac40, match_eng.cpp (~3.2 KB), thiscall, returns al).
///
/// [pt] { 0x6bac40 } (match_eng.cpp): ver `aplicar_falta_com`.
pub(crate) unsafe extern "thiscall" fn aplicar_falta(m: usize) -> u8 {
    aplicar_falta_com(m, &mut |rotina, this, a| {
        match rotina {
            0x6d6f70 => comentar(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]),
            0x716e10 | 0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
                f(this, a[0]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

// ---------- centesima vigesima setima leva: a disputa de bola (0x6c9cd0) ----------

/// { 0x6c9cd0 } (ret 8, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x6cd490 bola fora, 0x6f0630 dominio, 0x7103e0 o lance de quem ganhou; e
/// as de dentro da lesao, da defesa e da saida de campo): a bola solta na casa (x, y) disputada pelos
/// desafiantes dos dois lados (0x6c87e0, lance 0x15, a nota corrigida pelo angulo da direcao contra
/// +0x8ed8). Fora do campo e bola fora (0x6cd490). Sem desafiante, guarda a casa e devolve 0. O de
/// nota maior ganha; a lesao na dividida (0x6cfe10) vem logo. Com os dois, o arbitro olha a falta do
/// vencedor (0x6e9820, +4 de folego antes; 0x2168 na area com +0x8ebb) e depois a do perdedor; sem
/// falta, a 0x6e9540 (lance "valendo") pode marcar uma. A falta marcada vai para +0x475a (lance
/// pendente) por sorteio(distancia ao gol x 10 + 2) == 0 sem risco, ou e punida na hora (0x6ea4e0 /
/// 0x6e9630): narracao pelo lance (0x1fe8 a 0x1ff4), cartao, bola parada 7, penalti (5) ou falta (6),
/// expulsao (0x6cda20), as narracoes de sempre, o time de quem fez se desgasta (0x716450) e quem fez
/// cansa; os lances especiais saem antes do desgaste. Sem falta, a disputa limpa: no lance 1 a
/// defesa (0x70d430, que pode encerrar); com os dois, narra pela leitura (+0xad: 0x2164, 0x214f ou
/// 0x1f5d), conta tentativas (+7, +8, +9 com perigo) e mexe no folego. O vencedor marca o lance,
/// vira para a jogada (0x6f4920), domina (0x6f0630) com +0x19c = nota - nota do outro - 225
/// (+-200), o perdedor soma o segundo byte do desafio a +0x2b (0x7160d0) e o vencedor segue (0x7103e0).
/// Devolve o vencedor.
/// Marca posta na vantagem antes da 0x6e9820 (que so escreve 0 ou 1).
pub(crate) const SEM_RESPOSTA: u8 = 0xa5;
/// A disputa em andamento leu a vantagem sem resposta do juiz (lixo da pilha na original).
pub(crate) static VANTAGEM_SEM_RESPOSTA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Divergencias da disputa explicadas pelo lixo da pilha (contadas a parte, nao como divergencia).
pub(crate) static LIXO_DISPUTA: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub(crate) unsafe fn disputa_com(m: usize, x: i8, y: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> usize {
    let desafio: unsafe extern "thiscall" fn(usize, usize, i8, i8, *mut u8, *mut u8, *mut i16, *mut i8, *mut i8, u8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_OPPOSITION_CHALLENGE);
    let direcao_: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_FACING_ANGLE);
    let decisao: unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_OFFICIAL_JUDGE_TACKLE);
    let valendo: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_OFFICIAL_CHECK_FOUL);
    let punir_lance_: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_FOUL_PUNISHMENT);
    let punir_falta_: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_BOOKING);
    let ao_gol: unsafe extern "thiscall" fn(usize) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_OPPONENT_GOAL);
    let faixa: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
    let papel: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let perigo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_LAST_DEFENDER);
    let caminho: unsafe extern "thiscall" fn(usize, usize, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_PATH_AHEAD_CLEAR);
    let apontar_: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_SET_FACING_DIRECTION);
    let somar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_ADD_ACTION_DELAY);
    let desgastar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(FN_MATCH_PLAYER_SCALE_ANGER);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo, m as u32, a3, a4, a5]);
    };
    let especial = |l: i8| {
        let k = (l as i32 - 0x25) as u32;
        k <= 0x3c && matches!(k, 0 | 5..=13 | 0x3b | 0x3c)
    };
    if x < 0 || y < 0 || x >= 9 || y >= 12 {
        crate::comparacao::ramo("disputa fora do campo");
        chamar(0x6cd490, m, [ub(m, 0x8eab), 0, 0, 0, 0]);
        return 0;
    }
    // os desafiantes dos dois lados; a nota (palavra, ampliada sem sinal) corrigida pelo angulo
    let lado_do = |lado: i8, a: &mut u8, bb: &mut u8, lance: &mut i8, arr: &mut i8| -> (usize, i32) {
        let mut nota = 0i16;
        let e = desafio(m, 0, lado, 0x15, a, bb, &mut nota, lance, arr, 0);
        let mut v = nota as u16 as i32;
        if e != 0 {
            let mut d = (direcao_(e) as i16 as i32 - *w(m, 0x8ed8) as i32).abs();
            if d as i16 > 0xb4 {
                d = 0x168 - d;
            }
            v = v + d * 2 - 0x168;
        }
        (e, v)
    };
    let (mut a1, mut b1, mut lance1, mut arr1) = (0u8, 0u8, -1i8, 0i8);
    let (mut a0, mut b0, mut lance0, mut arr0) = (0u8, 0u8, -1i8, 0i8);
    let (e1, nota1) = lado_do(1, &mut a1, &mut b1, &mut lance1, &mut arr1);
    let (e0, nota0) = lado_do(0, &mut a0, &mut b0, &mut lance0, &mut arr0);
    let (wv, l, lance_w, lance_l, nota_w, nota_l, arr_w, arr_l, b_w) = if nota1 as i16 > nota0 as i16 {
        (e1, e0, lance1, lance0, nota1, nota0, arr1, arr0, b1)
    } else {
        (e0, e1, lance0, lance1, nota0, nota1, arr0, arr1, b0)
    };
    if wv == 0 {
        crate::comparacao::ramo("disputa sem desafiante");
        *b(m, 0x8ea7) = x as u8;
        *b(m, 0x8ea8) = y as u8;
        *((m + 0xf576) as *mut u32) = 0;
        return 0;
    }
    lesao_na_dividida_com(m, wv, lance_w as i16, l, lance_l as i16, 0, chamar);
    let arb = m + 0xf5b2;
    let (mut cartao, mut grav, mut vantagem) = (0u8, 0i8, 0u8);
    let mut tipo: u8;
    let mut peso = 0f32;
    // a falta marcada fica pendente (+0x475a) se ainda nao houver uma
    let pendente = |tipo: u8, lance: i8, quem: usize, outro: usize, cartao: u8, grav: i8| {
        crate::comparacao::ramo("disputa falta pendente");
        if *((m + 0x4761) as *const usize) == 0 {
            *w(m, 0x475f) = lance as i16;
            *b(m, 0x475a) = tipo;
            *b(m, 0x475b) = x as u8;
            *((m + 0x4761) as *mut usize) = quem;
            *((m + 0x4765) as *mut usize) = outro;
            *b(m, 0x4769) = 0;
            *b(m, 0x475c) = y as u8;
            *b(m, 0x475d) = cartao;
            *b(m, 0x475e) = grav as u8;
        }
    };
    let punir_agora = |d: usize| sortear(ftol(ao_gol(d) as f64 * 10.0 + 2.0)) != 0;
    // modo 1: falta do vencedor sobre o perdedor; 2: do perdedor sobre o vencedor; 3: so um desafiante
    let mut punido: Option<(u8, usize, usize)> = None;
    'faltas: {
        if l != 0 {
            *w(wv, 0x35) = (*w(wv, 0x35)).wrapping_add(4);
            // a 0x6e9820 pode sair antes de escrever a vantagem (falta na area, juiz sem paciencia): a
            // original le entao o que sobrou na pilha; a copia usa 0 (decisao do Lucas, 14/10/2026)
            vantagem = SEM_RESPOSTA;
            tipo = decisao(arb, wv, l, lance_w, -1, &mut cartao, &mut grav, &mut vantagem);
            if vantagem == SEM_RESPOSTA {
                crate::comparacao::ramo("disputa vantagem sem resposta");
                VANTAGEM_SEM_RESPOSTA.store(true, std::sync::atomic::Ordering::Relaxed);
                vantagem = 0;
            }
            if byte(m, 0x8ebb) != 0 && faixa(wv, byte(wv, 0x27) as u8) as u8 != 0
                && (especial(lance_w) || vantagem != 0 || sortear(*w(wv, 0x31) as i32) < -10)
                && sortear(3) != 0
            {
                narrar(chamar, 0x2168, ub(wv, 0x2e), ub(wv, 0x27), u32::MAX);
            }
            if tipo != 0 {
                crate::comparacao::ramo("disputa falta do vencedor");
                if punir_agora(wv) || arr_w != 0 {
                    punir_lance_(arb, wv, l, &mut tipo, cartao as i8, grav, arr_w as u8, &mut peso, 0);
                    if tipo != 0 {
                        punido = Some((1, wv, l));
                        break 'faltas;
                    }
                } else {
                    pendente(tipo, lance_w, wv, l, cartao, grav);
                }
            } else {
                tipo = valendo(arb, wv, lance_w as i16 as u16, &mut cartao, &mut grav as *mut i8 as *mut u8);
                if tipo != 0 {
                    if sortear(10) != 0 {
                        punir_falta_(arb, wv, &mut tipo, cartao as i8, grav, 0, &mut peso);
                        if tipo != 0 {
                            punido = Some((1, wv, l));
                            break 'faltas;
                        }
                    } else {
                        pendente(tipo, lance_w, wv, 0, cartao, grav);
                    }
                }
            }
            tipo = decisao(arb, l, wv, lance_l, -1, &mut cartao, &mut grav, &mut vantagem);
            if tipo != 0 {
                crate::comparacao::ramo("disputa falta do perdedor");
                if punir_agora(l) || arr_l != 0 {
                    punir_lance_(arb, l, wv, &mut tipo, cartao as i8, grav, arr_l as u8, &mut peso, 0);
                    if tipo != 0 {
                        punido = Some((2, l, wv));
                    }
                } else {
                    pendente(tipo, lance_l, l, wv, cartao, grav);
                }
            } else {
                tipo = valendo(arb, l, lance_l as i16 as u16, &mut cartao, &mut grav as *mut i8 as *mut u8);
                if tipo != 0 {
                    if sortear(10) != 0 {
                        punir_falta_(arb, l, &mut tipo, cartao as i8, grav, 0, &mut peso);
                        if tipo != 0 {
                            punido = Some((2, l, wv));
                        }
                    } else {
                        pendente(tipo, lance_l, l, 0, cartao, grav);
                    }
                }
            }
        } else {
            crate::comparacao::ramo("disputa so um desafiante");
            tipo = valendo(arb, wv, lance_w as i16 as u16, &mut cartao, &mut grav as *mut i8 as *mut u8);
            if tipo != 0 {
                if sortear(10) != 0 {
                    punir_falta_(arb, wv, &mut tipo, cartao as i8, grav, 0, &mut peso);
                    if tipo != 0 {
                        punido = Some((3, wv, 0));
                    }
                } else {
                    pendente(tipo, lance_w, wv, 0, cartao, grav);
                }
            }
        }
    }
    if let Some((modo, f, v)) = punido {
        crate::comparacao::ramo(["", "disputa punida modo 1", "disputa punida modo 2", "disputa punida modo 3"][modo as usize]);
        let lance = if modo == 2 { lance_l } else { lance_w };
        // a narracao da falta pelo lance de quem fez
        let codigo = if modo == 3 {
            0x1fe8
        } else {
            match lance {
                1 => 0x1ff1,
                0xc => 0x1ff4,
                0x15 => 0x1ff0,
                0x25 => 0x1fe8,
                0x2e => 0x1ff3,
                _ => 0x1fe9,
            }
        };
        if codigo == 0x1fe8 {
            narrar(chamar, codigo, ub(f, 0x2e), ub(f, 0x27), u32::MAX);
        } else {
            narrar(chamar, codigo, ub(f, 0x2e), ub(f, 0x27), ub(v, 0x2e) | ub(v, 0x27) << 8);
        }
        let cor: i32 = match tipo {
            2 => 0x1ff9,
            3 => 0x1ff7,
            4 => 0x1ff8,
            5 if modo != 3 => 0x1ff6,
            6 => 0x1ff5,
            _ => {
                if grav as i32 > sortear(7) + 2 { 0x210a } else { -1 }
            }
        };
        // o lado narrado: no modo 1 o de quem sofreu (cru); nos outros, "quem fez e do lado 0"
        let lado_narrado = if modo == 1 { ub(v, 0x27) } else { zero(f) };
        let som: u32;
        if tipo == 7 {
            som = 0x1f79;
            *b(m, 0x8eb2) = 7;
        } else if faixa(f, byte(f, 0x27) as u8) as u8 != 0 {
            if cor == -1 {
                match sortear(4) {
                    0 => narrar(chamar, 0x20e9, u32::MAX, lado_narrado, u32::MAX),
                    1 => narrar(chamar, 0x20ec, u32::MAX, lado_narrado, u32::MAX),
                    _ => {}
                }
            }
            crate::comparacao::ramo("disputa penalti");
            som = 0x1f7a;
            *b(m, 0x8eb2) = 5;
            *((m + 0x478a) as *mut usize) = v;
            *w(f, 0x35) = (*w(f, 0x35)).wrapping_sub(500);
        } else {
            som = 0x1f79;
            *b(m, 0x8eb2) = 6;
        }
        *b(m, 0x8ea7) = x as u8;
        *b(m, 0x8ea8) = y as u8;
        *b(m, 0x8eae) = zero(f) as u8;
        *((m + 0xf582) as *mut u32) = 0;
        if som == 0x1f7a {
            narrar(chamar, 0x1f7a, u32::MAX, lado_narrado, u32::MAX);
        }
        let expulso = matches!(tipo, 3..=6);
        if cor > -1 {
            if sortear(4) == 0 {
                let a = sortear(grav as i32);
                if a > sortear(0x14) {
                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                }
                narrar(chamar, 0x2109, ub(f, 0x2e), ub(f, 0x27), u32::MAX);
            }
            narrar(chamar, cor as u32, ub(f, 0x2e), ub(f, 0x27), u32::MAX);
            if expulso {
                crate::comparacao::ramo("disputa expulsao");
                jogador_saiu_com(m, f, 0, chamar);
            }
        }
        if som != 0x1f7a {
            narrar(chamar, som, u32::MAX, lado_narrado, u32::MAX);
        }
        if especial(lance) {
            crate::comparacao::ramo("disputa punida lance especial");
            return 0;
        }
        // no modo 1 as narracoes extras olham quem sofreu ("lado 0") e marcam +0x16 dele
        let (alvo, lado_extra) = if modo == 1 { (v, zero(v)) } else { (f, ub(f, 0x27)) };
        if sortear(9) == 0 {
            narrar(chamar, 0x20df, u32::MAX, lado_extra, u32::MAX);
            if sortear(3) == 0 {
                narrar(chamar, 0x20e0, u32::MAX, lado_extra, u32::MAX);
            }
            if expulso || som == 0x1f7a {
                *b(alvo, 0x16) = 1;
                if sortear(7) == 0 {
                    narrar(chamar, 0x20d9, u32::MAX, lado_extra, u32::MAX);
                }
            }
        }
        let bloco = m + 0x4796 + 0x22d8 * byte(f, 0x27) as isize as usize;
        for i in 0..20 {
            let p = bloco + 0x1be * i;
            if *((p + 0x69) as *const usize) == 0 {
                continue;
            }
            let fator = if p == f {
                peso
            } else if byte(p, 0x19) < 0 {
                (peso as f64 * 0.16666666666666666 + 0.83) as f32
            } else {
                ((peso as f64 + 1.0) * 0.5) as f32
            };
            desgastar(p, fator);
        }
        chamar(0x716e10, f, [0; 5]);
        return 0;
    }
    // a disputa limpa
    if lance_w == 1 {
        crate::comparacao::ramo("disputa defesa");
        let forca = if l == 0 { nota_w + 0x11 } else { nota_w - nota_l };
        let r = defender_com(wv, forca as i16, &mut |rotina, this, a| {
            if rotina == 0x6d6f70 {
                // a defesa narra com o quinto campo inteiro e o sexto -1
                chamar(rotina, this, [a[0], a[1], a[2], a[3], (a[4] & 0xff) | 0xff00])
            } else {
                chamar(rotina, this, a)
            }
        });
        if r == 0 {
            return 0;
        }
    } else if l != 0 {
        let r = sortear(6);
        let codigo = if ((r as f64 + f32_em(wv + 0xad) as f64) + 3.0) < f32_em(l + 0xad) as f64 {
            0x2164
        } else if papel(l) == 1 && sortear(5) != 0 {
            0x214f
        } else {
            0x1f5d
        };
        crate::comparacao::ramo(match codigo { 0x2164 => "disputa leitura 2164", 0x214f => "disputa leitura 214f", _ => "disputa leitura 1f5d" });
        narrar(chamar, codigo, ub(wv, 0x2e), ub(wv, 0x27), ub(l, 0x2e) | ub(l, 0x27) << 8);
        *b(wv, 7) = (*b(wv, 7)).wrapping_add(1);
        *b(wv, 8) = (*b(wv, 8)).wrapping_add(1);
        *w(wv, 0x35) = (*w(wv, 0x35)).wrapping_add(0x28);
        *b(l, 7) = (*b(l, 7)).wrapping_add(1);
        *w(l, 0x35) = (*w(l, 0x35)).wrapping_add(if papel(l) == 1 { -600 } else { -10 });
        if perigo(wv) != 0 {
            *b(wv, 9) = (*b(wv, 9)).wrapping_add(1);
            *w(wv, 0x35) = (*w(wv, 0x35)).wrapping_add(300);
        }
    }
    *w(wv, 0x198) = lance_w as i16;
    let k = sortear(200);
    let longe = byte(wv, 0x10c) as i32 * byte(wv, 0x105) as i32 > k
        || (ao_gol(wv) as f64) < 2.0
        || caminho(m, wv, 0) != 0;
    if !longe {
        crate::comparacao::ramo("disputa aponta para o lance marcado");
        apontar_(wv, x, y, byte(m, 0x1c8), byte(m, 0x1c9));
    } else if byte(wv, 0x27) == 0 {
        apontar_(wv, x, y, 4, if y < 9 { y + 3 } else { 11 });
    } else {
        apontar_(wv, x, y, 4, if y > 3 { y - 3 } else { 0 });
    }
    chamar(0x6f0630, wv, [0; 5]);
    *w(wv, 0x19c) = (nota_w - nota_l - 225) as i16;
    *w(wv, 0x19c) = (*w(wv, 0x19c)).clamp(-200, 200);
    if l != 0 {
        somar_2b(l, b_w);
    }
    chamar(0x7103e0, wv, [0; 5]);
    let _ = (a0, a1);
    wv
}

/// { 0x6c9cd0 } (ret 8, match_eng.cpp): ver `disputa_com`.
pub(crate) unsafe extern "thiscall" fn disputa(m: usize, x: u32, y: u32) -> usize {
    disputa_com(m, x as i8, y as i8, &mut |rotina, this, a| despachar_dividida(rotina, this, a))
}

// ---------- centesima trigesima quinta leva: o reinicio do lance (0x6c54a0) ----------

/// As bolas paradas que a 0x6c54a0 despacha por +0x8eb2 (2 lateral, 3 escanteio, 4 saida, 5
/// penalti, 6/7 falta, 8 pontape inicial). Na comparacao do reinicio elas sao unidades da fita
/// (`GRAVAR_REINICIO`): por dentro reposicionam jogadores (0x6f09f0), que a fita grava.
pub(crate) const BOLAS_PARADAS: [usize; 6] = [0x6c4290, 0x6c4470, 0x6c4ec0, 0x6c50d0, 0x6c4820, 0x6c51e0];
pub(crate) static GRAVAR_REINICIO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Dentro da comparacao do reinicio, com a fita aberta: as bolas paradas se gravam como unidade.
pub(crate) fn gravando_reinicio() -> bool {
    GRAVAR_REINICIO.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// { 0x6c54a0 } (thiscall, match_eng.cpp) com as chamadas que mexem fora injetadas (0x6d6f70 narra,
/// 0x6e1090 substitui, as bolas paradas): o reinicio do lance. Zera o estado do lance (+0x1c5,
/// +0x4782.., +0x8ea9/+0x8eaa, +0xf576.., o lance pendente em +0x475a..) e, sem +0x1a8:
/// - os 15 minutos desde a ultima mudanca de cada lado (+0x1d2/+0x1d4; com +0x8eb2 = 8, os dois e
///   -1000 em +0x476a/+0x476e);
/// - depois dos 60 minutos, 1 em 6, o primeiro jogador sem folego (+0x4d < 6750, +0x10d zerado) e
///   narrado (0x21cf);
/// - a postura de cada lado (0x6df460 em +0x9032 + 0xd0 x lado): mudou (ou no inicio) e guardada em
///   +0x1ab, com a anterior;
/// - postura 3/4 do lado com a bola (+0x8eae): sorteio(10) de acrescimo em +0x8ed0 e metade em
///   +0x8eb7; o outro lado, na postura 1, cansa (0x716450 com 1 + n/9);
/// - a substituicao da IA de cada lado (0x6e1090; sem humano, ou humano com +0x904b fora de selecao
///   e sem 0x5fd480): minuto > sorteio(50) + 10 e 55 de jogo desde a ultima, depois 0x6bbb60 e
///   0x6bbf60;
/// - perto do fim de cada tempo (+0x8ed4 = 495 ou 990) a narracao 0x21c1 com o acrescimo;
/// - a tatica de cada lado (sem humano) que mudou de postura, pelas marcas 0x5b50d0 no objeto em
///   +0x91e2 + 0x18e3 x lado, conforme a postura (1 a 5), o +0xe/+0x13 do tecnico (+0x9051), a
///   formacao (+0x905d: 3-4-3 e 3-5-2 Attacking ja sao de ataque) e +0x476a; as narracoes 0x2135 a
///   0x213d (0x6ce310 decide 0x213b/0x213d), 0x6d10a0 e a marca 0x400000 das casas (0x5b5290);
/// - com +0x8ebb (partida narrada), fora do escanteio e do penalti, e na falta so perto do gol
///   (tabela 0xa15e44 >= 5): os cantos da torcida pelo placar (0x6ce310), pelos clubes fixos
///   ([0x9d0230]..), pela nacao (+0x76: 4/12, 13, 14) ou o "Ruud Gullit" (0x84b8b0);
/// - e a bola parada por +0x8eb2.
pub(crate) unsafe fn reiniciar_lance_com(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let postura: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_MAN_CHOOSE_MENTALITY);
    let lado_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_SIDE);
    let cansar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(FN_MATCH_PLAYER_SCALE_ANGER);
    let lista_humanos: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let auto_humano: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let forca: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_RECALC_SIDE_SCORE);
    let marcacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_ASSIGN_MARKING_TARGETS);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FORMATION_SET_TEAM_INSTRUCTION);
    let gravar_casa_: unsafe extern "thiscall" fn(usize, i8, u64) -> u32 = core::mem::transmute(FN_FORMATION_SET_PLAYER_INSTRUCTION);
    let craques: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_INSTRUCTION_FOR_TOP_RATED_PLAYERS);
    let placar: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOAL_MARGIN);
    let nome_e: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_STAFF_NAME_EQUALS);
    let f69eb50: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_IS_CLUB_IN_FINANCIAL_TROUBLE);
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut i16;
    let d = |o: usize| (m + o) as *mut u32;
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let f32m = |o: usize| core::ptr::read_unaligned((m + o) as *const f32);
    let narr = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        chamar(0x6d6f70, g(m + 0x8f7a), [codigo | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let jogador = |lado: usize, k: usize| m + 0x4796 + lado * 0x22d8 + k * 0x1be;
    // o estado do lance
    *b(0x1c5) = 0;
    *d(0x4782) = 0;
    *d(0x4786) = 0;
    *d(0x477e) = 0;
    *b(0x8eaa) = 0;
    *b(0x8ea9) = 0;
    *d(0xf5aa) = 0;
    *d(0xf5a6) = 0;
    *d(0xf576) = 0;
    let mut mudou = [false; 2];
    let mut antiga = [0u8; 2];
    *d(0xf57a) = 0;
    *w(0x8ed6) = 0;
    *d(0x91de) = 0;
    *b(0x1cd) = 0;
    *d(0x4761) = 0;
    *d(0x4765) = 0;
    *b(0x4769) = 0;
    *b(0x475a) = 0;
    *b(0x8ea6) = 0xff;
    *b(0x8eb9) = 0xff;
    *w(0x475f) = -1;
    *b(0x475b) = 0xff;
    *b(0x475c) = 0xff;
    *b(0x475d) = 0xff;
    *b(0x475e) = 0xff;
    if *b(0x1a8) != 0 {
        return;
    }
    let quinze: [bool; 2] = if *b(0x8eb2) == 8 {
        *d(0x476a) = 0xc47a0000;
        *d(0x476e) = 0xc47a0000;
        [true, true]
    } else {
        let t = *w(0x8ed0) as i32;
        [t - *w(0x1d2) as i32 > 0xa5, t - *w(0x1d4) as i32 > 0xa5]
    };
    let minuto = (*w(0x8ed0) as i32 / 11) as i8;
    // o primeiro jogador sem folego
    if minuto > 0x3c && sortear(6) == 0 {
        crate::comparacao::ramo("reinicio sem folego");
        'achou: for lado in 0..2 {
            for k in 0..20 {
                let j = jogador(lado, k);
                if byte(j, 0x19) >= 0 && f32_em(j + 0x4d) < 6750.0 && byte(j, 0x10d) == 0 {
                    let l = lado_de(j) as u32;
                    narr(chamar, 0x21cf, *((j + 0x2e) as *const u8) as u32, l, u32::MAX, 0);
                    break 'achou;
                }
            }
        }
    }
    // a postura de cada lado
    for lado in 0..2usize {
        if *b(0x8eb2) == 5 || *b(0x1a8) != 0 {
            break;
        }
        let nova = postura(m + 0x9032 + lado * 0xd0);
        let velha = *b(0x1ab + lado);
        if nova != velha || *w(0x8ed0) <= 1 {
            mudou[lado] = true;
            antiga[lado] = velha;
            *b(0x1ab + lado) = nova;
        }
    }
    // o acrescimo (postura 3/4 de quem tem a bola) e o cansaco do outro lado
    if *b(0x8eae) != 0xff {
        let modo = *b((0x1ab + *(b(0x8eae) as *const i8) as isize) as usize);
        if modo == 3 || modo == 4 {
            crate::comparacao::ramo("reinicio acrescimo");
            let r = sortear(10) as i8 as i32;
            *w(0x8ed0) = (*w(0x8ed0)).wrapping_add(r as i16);
            let r2 = sortear(r / 2);
            *b(0x8eb7) = (*b(0x8eb7)).wrapping_add(r2 as u8);
            let outro = (*b(0x8eae) == 0) as usize;
            if *b(0x1ab + outro) == 1 {
                for k in 0..20 {
                    let j = jogador(outro, k);
                    if g(j + 0x69) != 0 {
                        cansar(j, (r as f64 * f64_em(0x96d3d8) + 1.0) as f32);
                    }
                }
            }
        }
    }
    // o humano de verdade no lado (ou a IA): +0x904d + 0xd0 x lado
    let humano_de_verdade = |lado: usize| -> bool {
        let p = g(m + 0x904d + lado * 0xd0);
        p != 0
            && ler(p) >= ler(0xae2364) - 0x10
            && ler(p) < ler(0xae2364)
            && *((p + 0x5f) as *const u8) != 0
            && lista_humanos(g(0xb63c98), p) == 0
    };
    let clube_do = |lado: usize| g(g(m + 0x4792) + if lado == 0 { 0x1c } else { 0x20 });
    // a substituicao da IA, lado 0 e lado 1
    for lado in 0..2usize {
        let ia = if humano_de_verdade(lado) {
            *b(0x904b + lado * 0xd0) != 0 && selecao(clube_do(lado)) == 0 && auto_humano(g(0xb63c98), g(m + 0x904d + lado * 0xd0)) == 0
        } else {
            true
        };
        let ultima = 0x8ecc + 2 * lado;
        if ia && (minuto as i32) > sortear(0x32) + 10 && *w(0x8ed0) as i32 - *w(ultima) as i32 > 0x37 {
            crate::comparacao::ramo("reinicio substituicao da IA");
            chamar(0x6e1090, m + 0x9032 + lado * 0xd0, [0; 5]);
            *w(ultima) = *w(0x8ed0);
            forca(m, lado as i8);
            marcacao(m);
        }
        if lado == 0 {
            // perto do fim de cada tempo, o acrescimo narrado
            let fim = match *w(0x8ed4) {
                0x1ef if minuto > 0x27 && minuto < 0x2e => Some(0x8d48),
                0x3de if minuto > 0x54 && minuto < 0x5b => Some(0x8d49),
                _ => None,
            };
            if let Some(f) = fim {
                if *b(f) == 0 {
                    crate::comparacao::ramo("reinicio acrescimo narrado");
                    let q = *(b(0x8eb7) as *const i8) as i32 / 11;
                    let v = if q <= 2 { 2 } else { q as i8 };
                    narr(chamar, 0x21c1, *b(0x8eae) as u32, 0, 0, v);
                    *b(f) = 1;
                }
            }
        }
    }
    // a tatica da IA pela postura
    for lado in 0..2usize {
        let blk = m + lado * 0xd0;
        if humano_de_verdade(lado) {
            if *((blk + 0x904b) as *const u8) == 0 || selecao(clube_do(lado)) != 0 || auto_humano(g(0xb63c98), g(blk + 0x904d)) != 0 {
                continue;
            }
        }
        if !mudou[lado] {
            continue;
        }
        let formacao = {
            let n = g(blk + 0x905d);
            let n = if n == 0 { 0xa15934 } else { n };
            std::ffi::CStr::from_ptr(n as *const core::ffi::c_char).to_bytes()
        };
        let ataque = formacao != b"3-4-3" && formacao != b"3-5-2 Attacking";
        let tobj = m + 0x91e2 + lado * 0x18e3;
        let tec = g(blk + 0x9051);
        let tb = |o: usize| *((tec + o) as *const i8);
        let lim = |base: i32| if *(b(0x1ad + lado) as *const i8) <= 0 { base } else { base + 3 };
        let la = lado as u32;
        let casas = || {
            for k in 0..20 {
                let c = byte(jogador(lado, k), 0x19);
                if c >= 0 {
                    gravar_casa_(tobj, c, 0x400000);
                }
            }
        };
        let quinze_l = quinze[lado];
        match *b(0x1ab + lado) {
            1 => {
                crate::comparacao::ramo("reinicio postura 1");
                let f = if (tec != 0 && f32m(0x476a + 4 * lado) < 2.5 && tb(0xe) <= 6) || !ataque { 0x20000 } else { 0x10000 };
                geral(tobj, f);
                geral(tobj, 0x800);
                geral(tobj, 0x2000);
                if *w(0x8ed0) <= 1 {
                    narr(chamar, 0x2137, u32::MAX, la, u32::MAX, 0);
                } else if quinze_l {
                    narr(chamar, 0x2139, u32::MAX, la, u32::MAX, 0);
                    *w(0x1d2 + 2 * lado) = *w(0x8ed0);
                }
                craques(m, lado as i8);
            }
            2 => {
                crate::comparacao::ramo("reinicio postura 2");
                if tec == 0 || (!(tb(0xe) as i32 > lim(0xf) && ataque) && (tb(0xe) > 5 || *b(0x1ad + lado) == 0 || f32m(0x476a + 4 * lado) as f64 > 1.0)) {
                    geral(tobj, 0x20000);
                    geral(tobj, 0x800);
                } else if tb(0xe) as i32 > lim(0xf) && ataque {
                    geral(tobj, 0x10000);
                    geral(tobj, 0x800);
                } else {
                    // NaN em +0x476a cai aqui, como o fcomp da original
                    geral(tobj, 0x8000);
                    geral(tobj, if (tb(0x13) as i32) < lim(4) { 0x1000 } else { 0x800 });
                }
                geral(tobj, 0x2000);
                if *w(0x8ed0) > 1 {
                    if matches!(antiga[lado], 3..=5) {
                        narr(chamar, 0x2138, u32::MAX, la, u32::MAX, 0);
                    } else if antiga[lado] == 1 && quinze_l {
                        narr(chamar, 0x213a, u32::MAX, la, u32::MAX, 0);
                        *w(0x1d2 + 2 * lado) = *w(0x8ed0);
                    }
                }
                casas();
            }
            modo @ (3 | 4) => {
                if modo == 3 {
                    crate::comparacao::ramo("reinicio postura 3");
                    if tec != 0 {
                        geral(tobj, if tb(0xe) > 0x10 { 0x20000 } else { 0x8000 });
                        geral(tobj, if (tb(0x13) as i32) < lim(10) { 0x1000 } else { 0x800 });
                    } else {
                        geral(tobj, 0x8000);
                        geral(tobj, 0x1000);
                    }
                    geral(tobj, 0x2000);
                } else {
                    crate::comparacao::ramo("reinicio postura 4");
                    geral(tobj, if tec != 0 && tb(0xe) > 0x12 { 0x20000 } else { 0x8000 });
                    geral(tobj, 0x800);
                    geral(tobj, 0x4000);
                }
                if *w(0x8ed0) <= 1 {
                    narr(chamar, if modo == 3 { 0x2136 } else { 0x2135 }, u32::MAX, la, u32::MAX, 0);
                } else if (antiga[lado] == 2 || antiga[lado] == 1) && quinze_l {
                    let (mut c1, mut c2, mut c3, mut c4, mut c5) = (0i8, 0i8, 0u8, 0u8, 0u8);
                    let r = placar(m, lado as i8, &mut c1, &mut c2, &mut c3, &mut c4, &mut c5, 0);
                    narr(chamar, if (r as i8) > 0 { 0x213b } else { 0x213d }, u32::MAX, la, u32::MAX, 0);
                    *w(0x1d2 + 2 * lado) = *w(0x8ed0);
                }
                casas();
            }
            5 => {
                crate::comparacao::ramo("reinicio postura 5");
                if tec != 0 {
                    let e = tb(0xe);
                    geral(tobj, if e > 0x10 && ataque { 0x10000 } else if e > 0xc { 0x20000 } else { 0x8000 });
                    geral(tobj, if (tb(0x13) as i32) < lim(10) { 0x1000 } else { 0x800 });
                } else {
                    geral(tobj, 0x20000);
                    geral(tobj, 0x1000);
                }
                geral(tobj, 0x2000);
                if quinze_l {
                    narr(chamar, 0x213c, u32::MAX, la, u32::MAX, 0);
                    *w(0x1d2 + 2 * lado) = *w(0x8ed0);
                }
                casas();
            }
            _ => {}
        }
    }
    // os cantos da torcida (partida narrada)
    'cantos: {
        let modo = *b(0x8eb2);
        if *b(0x8ebb) == 0 || modo == 3 || modo == 5 {
            break 'cantos;
        }
        if modo == 6 {
            // na falta, so perto do gol (chance da tabela 0xa15e44 >= 5)
            let chance = |x: i32, y: i32| -> f32 {
                let (x, y) = (x as i8, y as i8);
                if (0..=8).contains(&x) && (0..=11).contains(&y) { f32_em(0xa15e44 + (y as usize * 9 + x as usize) * 4) } else { 1000.0 }
            };
            let x = ((*(b(0x8ea7) as *const i8) as i32) - 4).abs();
            let ycol = *(b(0x8ea8) as *const i8) as i32;
            let lado = *b(0x8eae);
            if lado == 0 && (chance(x, (ycol - 0xb).abs()) as f64) < 5.0 {
                break 'cantos;
            }
            if lado == 1 && (chance(x, ycol.abs()) as f64) < 5.0 {
                break 'cantos;
            }
        }
        let (mut o1, mut o2, mut o3, mut o4, mut o5) = (0i8, 0i8, 0u8, 0u8, 0u8);
        placar(m, 0, &mut o1, &mut o2, &mut o3, &mut o4, &mut o5, 0);
        let (v1, v2, empate) = (o1 as i32, o2 as i32, o3);
        let canto = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, lado: u32| {
            crate::comparacao::ramo("reinicio canto da torcida");
            narr(chamar, codigo, u32::MAX, lado, u32::MAX, 0);
        };
        // os cantos de um clube (`lado_clube` 0 casa, 1 fora): o do outro lado com +0x77 = 8, os fixos
        // e os da nacao
        let cantos_de = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, lado_clube: usize| {
            let outro = clube_do(1 - lado_clube);
            if f69eb50(g(0xb67a44), outro) != 0 && *((g(outro + 0x53) + 0x77) as *const u8) == 8 {
                canto(chamar, 0x2129, (1 - lado_clube) as u32);
                return;
            }
            let clube = clube_do(lado_clube);
            for (fixo, codigo) in [(0x9d0230usize, 0x2127u32), (0x9d0234, 0x2128), (0x9d0238, 0x2163), (0x9d0268, 0x2166), (0x9d023c, 0x212b), (0x9d0240, 0x212c), (0x9d0244, 0x212d)] {
                if sortear(2) == 0 && ler(clube) == ler(fixo) {
                    canto(chamar, codigo, lado_clube as u32);
                    return;
                }
            }
            match *((g(clube + 0x53) + 0x76) as *const i8) {
                4 | 12 => {
                    let r = sortear(2);
                    canto(chamar, if r == 0 { 0x216b } else { 0x216a }, lado_clube as u32);
                }
                13 => {
                    let r = sortear(2);
                    canto(chamar, if r == 0 { 0x2167 } else { 0x216a }, lado_clube as u32);
                }
                14 => canto(chamar, 0x2118, lado_clube as u32),
                _ => {}
            }
        };
        // A: perdendo (ou empatado sem classificacao) por sorteio(10), o "Ruud Gullit"
        if (v1 < v2 || (v1 == v2 && empate == 0)) && sortear(10) == 0 && nome_e(g(m + 0x904d), 0x9b63a8) != 0 {
            canto(chamar, 0x212a, 1);
            break 'cantos;
        }
        // B/C: a torcida do clube de fora
        let r = sortear(v2 - v1 + 1);
        let fora = r != 0 && (v1 < v2 || (v1 == v2 && empate == 0));
        if fora || sortear(v1 - v2 + 5) == 0 {
            cantos_de(chamar, 1);
            break 'cantos;
        }
        // E/F: a torcida da casa
        let r = sortear(v1 - v2 + 1);
        let casa = r != 0 && (v1 > v2 || (v1 == v2 && empate != 0));
        if casa || sortear(v2 - v1 + 5) == 0 {
            cantos_de(chamar, 0);
        }
    }
    // a bola parada
    match *b(0x8eb2) {
        1 => {}
        2..=8 => {
            let k = match *b(0x8eb2) {
                2 => 0,
                3 => 1,
                4 => 2,
                5 => 3,
                6 | 7 => 4,
                _ => 5,
            };
            chamar(BOLAS_PARADAS[k], m, [0; 5]);
        }
        _ => {
            // ponytail: a original mostra a caixa "Error" (match_eng.cpp, linha 0x29ab) e sai do jogo
            *(0xb67a34 as *mut u32) = 0;
        }
    }
}

/// O despacho do reinicio no modo Rust: narracao e substituicao como na dividida; as bolas paradas
/// pelo endereco do jogo.
pub(crate) unsafe fn despachar_reinicio(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    if BOLAS_PARADAS.contains(&rotina) {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
        f(this);
        return 0;
    }
    despachar_dividida(rotina, this, a)
}

/// resets the play state (+0x1c5, +0x4782.., +0x8ea9/+0x8eaa, +0xf576.., the pending play +0x475a..) and, without
/// +0x1a8: the 15 minutes since each side's last change (+0x1d2/+0x1d4); after 60 minutes, 1 in 6, the first player out
/// of breath (+0x4d < 6750) narrated (0x21cf); each side's posture (0x6df460) stored in +0x1ab with the previous one;
/// with posture 3/4 for the side with the ball, draw(10) of stoppage time (+0x8ed0, half in +0x8eb7) and the other side
/// tiring in posture 1 (0x716450); the AI substitution (0x6e1090 on +0x9032 + 0xd0 × side; no human, or an automatic
/// human) with minute > draw(50) + 10 and 55 of play since the last, 0x6bbb60 and 0x6bbf60; the narrated stoppage time
/// (0x21c1) near the end of each half; the AI tactics by posture (0x5b50d0 marks on object +0x91e2 + 0x18e3 × side, by
/// the manager's +0xe/+0x13, the formation +0x905d and +0x476a; narrations 0x2135 to 0x213d, 0x6d10a0, the squares'
/// 0x400000 mark); with +0x8ebb the crowd chants (score 0x6ce310, fixed clubs [0x9d0230].., nation +0x76, "Ruud Gullit"
/// 0x84b8b0); and the set piece by +0x8eb2 (throw-in, corner, kick-off, penalty, free kick, kick-off). Ghidra's
/// pseudocode gets postures 2 and 3 wrong ("previous 3/4/5" and "previous 1/2" become "== true"): ported from the
/// disassembly. Compared through the tape (set pieces are units with `GRAVAR_REINICIO`, since they reposition players
/// through 0x6f09f0)
/// Original: `restart_play` (0x6c54a0, match_eng.cpp (3.2 KB, called by 0x6b98c0), thiscall).
///
/// [pt] { 0x6c54a0 } (thiscall, match_eng.cpp): ver `reiniciar_lance_com`.
pub(crate) unsafe extern "thiscall" fn reiniciar_lance(m: usize) {
    reiniciar_lance_com(m, &mut |r, t, a| despachar_reinicio(r, t, a))
}

pub(crate) unsafe extern "thiscall" fn comparar_reiniciar_lance(m: usize) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize)>(300);
    if gravando_no_topo() {
        return o(m);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_REINICIO.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        reiniciar_lance_com(m, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6e1090 => [u32::MAX, 0, 0, 0, 0],
                _ => [0, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_REINICIO.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("reiniciar_lance(modo {}) {} chamadas: {}", *((m + 0x8eb2) as *const u8), k, detalhe_fita()));
}

// ---------- centesima quadragesima quinta leva: o laco do motor (0x6b98c0) ----------

// Chamadas do laco pelo endereco: cada uma passa pelo proprio desvio (copia Rust ou comparador).
pub(crate) unsafe fn em0(a: usize, this: usize) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(a)(this)
}
pub(crate) unsafe fn em1(a: usize, this: usize, x: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(a)(this, x)
}
pub(crate) unsafe fn em2(a: usize, this: usize, x: u32, y: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(a)(this, x, y)
}
pub(crate) unsafe fn em3(a: usize, this: usize, x: u32, y: u32, z: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(a)(this, x, y, z)
}
pub(crate) unsafe fn em4(a: usize, this: usize, x: u32, y: u32, z: u32, w: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(a)(this, x, y, z, w)
}

/// Fim da partida pelo laco: o fechamento (0x6d1cc0) e, para cada clube que um humano dirige
/// (0x600a50) sem o registro +0x9a05 do tecnico (0x5fd480), as instrucoes guardadas voltam (0x896fe0).
pub(crate) unsafe fn encerrar_clubes(m: usize) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(FN_MATCH_ENG_UPDATE_POST_MATCH_REPUTATIONS);
    f(m);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    for (o, marca) in [(0x1c, 0x40), (0x20, 0x41)] {
        let t = u(m, 0x4792);
        if byte(t, marca) == 0 && humano(u(t, o), 1, 1, 0, 0) != 0 && em1(0x5fd480, u(0xb63c98, 0), u(u(t, o), 0xcf) as u32) == 0 {
            em4(0x896fe0, u(0xde1d08, 0), u(t, o) as u32, 1, 0, 0);
        }
    }
}

/// Os mais perto da bola de cada lado (0x6c30e0, so dentro da area quando a bola esta nela) e se o
/// guarda da posicao 1 e o mais proximo dela (0x6ce110 + 0x6c3010).
pub(crate) unsafe fn perto_da_bola(m: usize) {
    let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
    let a0 = (2..=6).contains(&bx) && (0..=1).contains(&by);
    *((m + 0xf586) as *mut usize) = em4(0x6c30e0, m, 0, bx as u8 as u32, by as u8 as u32, !a0 as u32) as usize;
    let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
    let a1 = (2..=6).contains(&bx) && (10..=11).contains(&by);
    *((m + 0xf58a) as *mut usize) = em4(0x6c30e0, m, 1, bx as u8 as u32, by as u8 as u32, !a1 as u32) as usize;
    for (lado, campo) in [(0u32, 0x8eac), (1, 0x8ead)] {
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        let g = em1(0x6ce110, m, lado);
        *((m + campo) as *mut u8) = em3(0x6c3010, m, g, bx as u8 as u32, by as u8 as u32) as u8;
    }
}

/// `n` match instants. Match already decided: quick simulation, highlight, closing, humans' instructions restored,
/// referee rating (2). Each instant: punishments, shoot-out, kick-off, drawn substitution, marked incident, restart and
/// taker; on the clock, abandonment (8), period (4) and full time (2 or 4); then possession, each player's time, lines,
/// nearest to the ball, marking and each active player's decision (0x7103e0), with the pending shot and the contest.
/// Validated by the career
/// Original: `engine_loop` (0x6b98c0, match_eng.cpp (4.8 KB), thiscall, ret 4).
///
/// [pt] { 0x6b98c0 } (ret 4, match_eng.cpp): o laco do motor, `n` instantes da partida. Partida ja
/// decidida (+0): simulacao rapida (0x6beeb0), destaque (0x6d0f80), fechamento, instrucoes dos
/// humanos de volta e nota do arbitro (0x6d4a20); devolve 2. Senao limpa as fichas e os ponteiros
/// de quem saiu e, a cada instante: punicoes pendentes (0x6d1210), disputa de penaltis (0x6be710;
/// no fim, 2), o pontape inicial narrado (0x212f/0x1f74), a substituicao sorteada, o lance marcado
/// (0x6bac40/0x6bba10), o reinicio (0x6c54a0) e o cobrador da bola parada (0x6ceb90). No relogio
/// (+0x8ed0): fim por abandono (8), o periodo (0x6bd930: 4 nos intervalos; fim da partida com o
/// placar agregado, 0x896290 e 2 ou 4). Bola rolando (0x6bbe40) ou parada somam acrescimos; senao
/// a posse (0x6cf750), o tempo de cada jogador (0x6f4aa0), as linhas (0x6c3aa0, 0x6cd660, 0x6bd760),
/// os mais perto da bola, a pressao nas areas (0x6bd6b0), as marcacoes (0x7160f0/0x6bc030) e a
/// decisao de cada jogador ativo (0x7103e0), com o chute pendente (0x6f6c00) e a disputa (0x6c9cd0).
/// Devolve 1 ao fim dos `n` instantes.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe extern "thiscall" fn laco_do_motor(m: usize, n_in: u32) -> u32 {
    let u = |o: usize| *((m + o) as *const usize);
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut i16;
    let narrar = |codigo: u32, a4: u32| {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_EVENTS_ADD_EVENT);
        f(u(0x8f7a), codigo, m as u32, u32::MAX, a4, u32::MAX, u32::MAX, 0);
    };
    let acrescimos = || {
        let r = sortear(10) as i8;
        *w(0x8ed0) = (*w(0x8ed0)).wrapping_add(r as i16);
        let v = sortear(ftol(r as f64 * f64_em(0x9689b0)));
        *b(0x8eb7) = (*b(0x8eb7)).wrapping_add(v as u8);
    };
    let mut n = n_in as u8 as i8;
    if byte(m, 0) != 0 {
        em0(0x6beeb0, m);
        em0(0x6d0f80, m);
        encerrar_clubes(m);
        em0(0x6d4a20, m);
        return 2;
    }
    for lado in 0..2 {
        for k in 0..20 {
            let f = m + 0x4796 + (lado * 20 + k) * 0x1be;
            if byte(f, 0x19) >= 0 && *((f + 0x19e) as *const u32) == 0 {
                *((f + 0x19) as *mut u8) = 0xfb;
            }
        }
    }
    for o in [0xf576, 0xf57a, 0xf57e, 0xf5ae, 0x91de, 0xf586, 0xf58a, 0xf5a6, 0xf5aa, 0xf58e, 0xf592] {
        let q = u(o);
        if q != 0 && *((q + 0x19e) as *const usize) == 0 {
            *((m + o) as *mut usize) = 0;
        }
    }
    'instante: while n != 0 {
        crate::extensao::instante(m);
        if byte(m, 0x8eb4) == 1 {
            em1(0x6d1210, m, 0);
            n = n.wrapping_sub(1);
            continue;
        }
        match byte(m, 0x8eb3) {
            1..=3 => {
                em1(0x6be710, m, 4);
                if byte(m, 0x8eb3) == 0 {
                    crate::comparacao::ramo("laco penaltis decididos");
                    encerrar_clubes(m);
                    em1(0x6ce460, m, *w(0x8ed4) as u16 as u32);
                    em0(0x6d4a20, m);
                    return 2;
                }
                n = n.wrapping_sub(1);
                continue;
            }
            4..=6 => {
                em1(0x6be710, m, 3);
                if byte(m, 0x8eb3) == 0 {
                    crate::comparacao::ramo("laco penaltis decididos sem resumo");
                    encerrar_clubes(m);
                    em0(0x6d4a20, m);
                    return 2;
                }
                n = n.wrapping_sub(1);
                continue;
            }
            _ => {}
        }
        if byte(m, 0x1ca) == 0 && byte(m, 0x8eb2) == 8 && byte(m, 0x1a8) == 0 {
            narrar(if *w(0x8ed0) > 1 { 0x1f74 } else { 0x212f }, byte(m, 0x8eae) as u8 as u32);
            *b(0x1ca) = 1;
            continue;
        }
        if byte(m, 0x8eb2) != 0 && byte(m, 0x8eb4) == 0 && byte(m, 0x1a8) == 0 {
            // a substituicao sorteada: um jogador em campo ate 20 tentativas (0x7164b0 libera os de fora)
            let mut tentativas = 0i8;
            let ficha = |a: i8, c: i8| m + 0x4796 + ((a as i32 * 20 + c as i32) * 0x1be) as usize;
            let (a, c) = loop {
                let a = sortear(2) as i8;
                let c = sortear(0x14) as i8;
                tentativas += 1;
                if byte(ficha(a, c), 0x19) >= 0 {
                    break (a, c);
                }
                if tentativas >= 0x14 {
                    break (a, c);
                }
                if em2(0x7164b0, ficha(a, c), 1, 0) as u8 != 0 {
                    break (a, c);
                }
            };
            if tentativas < 0x14 && byte(ficha(a, c), 0x19) >= 0 {
                em1(0x6d1210, m, ficha(a, c) as u32);
                if byte(m, 0x8eb4) != 0 {
                    continue;
                }
            }
        }
        if byte(m, 0x8eb4) == 2 {
            *b(0x8eb4) = 0;
        }
        // o lance marcado
        let q = u(0x4761);
        if byte(m, 0x475a) != 0 && q != 0 {
            let cl = byte(q, 0x27);
            if byte(m, 0x8eb2) != 0 {
                let d = u(0xf582);
                let (by, ly) = (byte(m, 0x8ea8), byte(m, 0x475c));
                if d != 0 && byte(d, 0x27) as i32 == (cl == 0) as i32 && ((by > ly && cl == 1) || (by < ly && cl == 0)) {
                    crate::comparacao::ramo("laco lance descartado");
                    em0(0x6bba10, m);
                } else if sortear(5) == 0 && em0(0x6bac40, m) as u8 != 0 {
                    continue;
                }
            } else if sortear(7) == 0 {
                let v = if byte(m, 0x475a) != 0 && u(0x4761) != 0 { (byte(u(0x4761), 0x27) == 0) as u32 } else { 0xff };
                narrar(0x20ef, v);
                if em0(0x6bac40, m) as u8 != 0 {
                    continue;
                }
            }
        }
        if byte(m, 0x1a8) == 0 {
            if byte(m, 0x8eb2) != 0 {
                em0(0x6c54a0, m);
            }
            let d = u(0xf582);
            if byte(m, 0x1a8) == 0 && byte(m, 0x8eb2) != 0 && (d == 0 || byte(d, 0x19) < 0) {
                crate::comparacao::ramo("laco cobrador da bola parada");
                let c = em3(0x6ceb90, m, byte(m, 0x8eae) as u8 as u32, byte(m, 0x8eb2) as u8 as u32, 0) as usize;
                *((m + 0xf582) as *mut usize) = c;
                em2(0x6f46b0, c, byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32);
                *((u(0xf582) + 0x19c) as *mut i16) = 200;
                em0(0x6f0630, u(0xf582));
                let d = if byte(m, 0x8eae) != 0 { 1 } else { 0xa };
                em4(0x6f4920, u(0xf582), byte(m, 0x8ea7) as u8 as u32, byte(m, 0x8ea8) as u8 as u32, 4, d);
            }
        }
        *w(0x8ed2) = (*w(0x8ed2)).wrapping_add(1);
        if *w(0x8ed2) >= 1 || byte(m, 0x1a8) != 0 {
            n = n.wrapping_sub(1);
            let limite = byte(m, 0xf638);
            if limite != -1 && (limite as i16).wrapping_mul(11) < *w(0x8ed0) {
                crate::comparacao::ramo("laco abandono");
                *b(0xf5bd) = 0xfd;
                *b(0xf5f3) = 0xfd;
                *((u(0x4792) + 0x43) as *mut u8) = 0xfd;
                *b(0x8eb2) = 0;
                narrar(0x217b, 0);
                return 8;
            }
            if byte(m, 0x8ebb) != 0 {
                em1(0x6ce460, m, 0x3de);
            }
            *w(0x8ed0) = (*w(0x8ed0)).wrapping_add(1);
            *w(0x8ed2) = 0;
            match em0(0x6bd930, m) as i8 {
                0 => {
                    for base in [0x4796, 0x6a6e] {
                        for k in 0..20 {
                            let f = m + base + k * 0x1be;
                            if byte(f, 0x19) >= 0 {
                                em0(0x6f55c0, f);
                            }
                        }
                    }
                }
                1..=3 => return 4,
                4 => {
                    // fim da partida
                    em0(0x6d0f80, m);
                    let t = u(0x4792);
                    let tb = |o: usize| (t + o) as *mut u8;
                    let ag = byte(t, 0x4b);
                    if ag != -1 {
                        if byte(t, 0x47) != -1 {
                            *tb(0x4b) = (*tb(0x47)).wrapping_add(ag as u8);
                            *tb(0x4c) = (*tb(0x4c)).wrapping_add(*tb(0x48));
                        } else {
                            *tb(0x4b) = (*tb(0x43)).wrapping_add(ag as u8);
                            *tb(0x4c) = (*tb(0x4c)).wrapping_add(*tb(0x44));
                        }
                    }
                    if *(t as *const i32) != -2 {
                        em1(0x896290, *(0xde1d08 as *const usize), t as u32);
                    }
                    let p = byte(m, 0x8eb3);
                    if p != 0 {
                        crate::comparacao::ramo("laco fim para os penaltis");
                        *w(0x8ed0) = (*w(0x8ed0) as i32 / 30 * 30) as i16;
                        narrar(if p == 1 { 0x2003 } else { 0x2002 }, 0);
                        return 4;
                    }
                    encerrar_clubes(m);
                    em0(0x6d4a20, m);
                    return 2;
                }
                r => {
                    // ponytail: o original para o jogo com "Error" (match_eng.cpp, linha 0x776); a 0x6bd930 so devolve 0..=4
                    linha(&format!("motor: laco_do_motor: periodo devolveu {}", r));
                    std::process::abort();
                }
            }
        }
        // bola rolando ou parada: so acrescimos
        if byte(m, 0x8eb2) == 0 && {
            em0(0x6bbe40, m);
            byte(m, 0x8eb2) != 0
        } {
            acrescimos();
            continue;
        }
        if u(0xf5aa) != u(0xf582) && u(0xf582) != 0 {
            *((m + 0xf5aa) as *mut usize) = 0;
            *((m + 0xf5a6) as *mut usize) = 0;
        }
        em2(0x6cf750, m, byte(m, 0x8eae) as u8 as u32, byte(m, 0x8ebb) as u8 as u32);
        for base in [0x4796, 0x6a6e] {
            for k in 0..20 {
                let f = m + base + k * 0x1be;
                if byte(f, 0x19) >= 0 {
                    em0(0x6f4aa0, f);
                }
            }
        }
        em0(0x6c3aa0, m);
        em0(0x6cd660, m);
        em0(0x6bd760, m);
        *((m + 0x91de) as *mut usize) = 0;
        perto_da_bola(m);
        *b(2) = 0xff;
        *b(3) = 0xff;
        let (mut vx, mut vy) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        if *((m + 0x9766) as *const u32) & 0x1000 != 0 {
            em1(0x6bd6b0, m, 0);
        }
        if *((m + 0xb049) as *const u32) & 0x1000 != 0 {
            em1(0x6bd6b0, m, 1);
        }
        let lista = |i: i32| *((m + 0x8f7e + i as usize * 4) as *const usize);
        let mut i = 0i8;
        while i < byte(m, 0x8eb8) {
            let q = lista(i as i32);
            *((q + 0x18) as *mut i8) = i;
            *((q + 0x101) as *mut u8) = 0;
            *((q + 0x1aa) as *mut usize) = 0;
            i += 1;
        }
        // as marcacoes
        let mut i = 0i8;
        while i < byte(m, 0x8eb8) {
            let q = lista(i as i32);
            i += 1;
            let lado = byte(q, 0x27);
            if em1(0x5b57c0, m + 0x91e2 + lado as isize as usize * 0x18e3, byte(q, 0x19) as u8 as u32) & 1 != 0 {
                continue;
            }
            let mut alvo = em0(0x7160f0, q) as usize;
            if alvo == 0 {
                if em0(0x717410, q) as u16 == 1 || q == u(0xf582) {
                    continue;
                }
                let tab = || em0(0x717410, q);
                let by = byte(m, 0x8ea8);
                let marcar = if *((m + 0x9766 + lado as isize as usize * 0x18e3) as *const u32) & 0x4000 != 0 {
                    true
                } else if byte(m, 0x8eae) == lado && if lado == 0 { by >= 3 } else { by <= 8 } {
                    false
                } else if (tab() & 4 != 0 || tab() & 2 != 0) && em0(0x714d00, q) as u8 != 0 {
                    true
                } else {
                    let fundo = if tab() & 8 == 0 && tab() & 0x10 == 0 {
                        false
                    } else if lado == 0 {
                        by < 3
                    } else {
                        by > 8
                    };
                    fundo || na_area_atacada((lado != 1) as i8, byte(m, 0x8ea7), by)
                };
                if !marcar {
                    continue;
                }
                alvo = em1(0x6bc030, m, q as u32) as usize;
                if alvo == 0 {
                    continue;
                }
            }
            *((q + 0x1aa) as *mut usize) = alvo;
            let k = byte(alvo, 0x101);
            if k < 3 {
                *((alvo + 0x1ae + k as usize * 4) as *mut usize) = q;
                *((alvo + 0x101) as *mut u8) = (k + 1) as u8;
            }
        }
        // a decisao de cada jogador ativo
        let mut cansou = false;
        let mut i = 0i32;
        while i < byte(m, 0x8eb8) as i32 {
            let q = lista(i);
            if *((q + 0x1aa) as *const usize) == 0 || byte(m, 0x8eae) == byte(q, 0x27) {
                if !cansou || byte(q, 0x104) as i32 > sortear(10) {
                    em0(0x7103e0, q);
                }
                let q = lista(i);
                let acao = *((q + 0x198) as *const i16);
                if acao == 0x11 || acao == 0x12 {
                    let v = f32_em(q + if acao == 0x11 { 0x71 } else { 0xc1 }) as f64;
                    if sortear(ftol(f32_em(0x96d2c0) as f64 - v * f32_em(0x968960) as f64)) == 0 {
                        cansou = true;
                    }
                }
                if byte(m, 0x8eb2) != 0 || byte(m, 0x1a8) != 0 {
                    acrescimos();
                    if byte(m, 0x8eb5) != 0 {
                        em0(0x6d0b60, m);
                    }
                    continue 'instante;
                }
                // o chute pendente e a disputa marcada
                let mut feito = false;
                if byte(m, 0x1c5) != 0 {
                    *b(0x1c5) = (*b(0x1c5)).wrapping_sub(1);
                    if byte(m, 0x1c5) == 0 && byte(m, 0x8ea9) == 4 && u(0x477e) == 0 {
                        crate::comparacao::ramo("laco disputa marcada");
                        em2(0x6c9cd0, m, byte(m, 0x1c6) as u8 as u32, byte(m, 0x1c7) as u8 as u32);
                        feito = true;
                    }
                }
                let c = u(0x477e);
                if !feito && byte(m, 0x1c5) == 0 && c != 0 {
                    let chutar = |c: usize| em3(0x6f6c00, c, *w(0x8ed6) as u16 as u32, byte(m, 0x1c8) as u8 as u32, byte(m, 0x1c9) as u8 as u32);
                    if byte(c, 0x102) == byte(m, 0x8ea7) && byte(c, 0x103) == byte(m, 0x8ea8) {
                        chutar(c);
                    } else if c == u(0xf57a) && byte(c, 0x2b) == 0 && byte(c, 0x105) != 0 && byte(c, 0x107) != 0 {
                        crate::comparacao::ramo("laco batedor se aproxima");
                        *b(0x1c5) = 1;
                        em1(0x713fc0, c, 1);
                        *b(0x1c5) = 0;
                        let c2 = u(0x477e);
                        if c2 == c && byte(c2, 0x102) == byte(m, 0x8ea7) && byte(c2, 0x103) == byte(m, 0x8ea8) {
                            chutar(c2);
                        }
                    }
                }
                if byte(m, 0x8eb2) != 0 || byte(m, 0x1a8) != 0 {
                    acrescimos();
                    continue 'instante;
                }
                if u(0x91de) != 0 {
                    continue 'instante;
                }
                let q = lista(i);
                let acao = *((q + 0x198) as *const i16);
                if (acao == 0x104 || acao == 0x76) && i < 0x16 {
                    crate::comparacao::ramo("laco entra na lista");
                    let k = byte(m, 0x8eb8);
                    *((m + 0x8f7e + k as isize as usize * 4) as *mut usize) = q;
                    *b(0x8eb8) = (k + 1) as u8;
                }
                em0(0x6c3aa0, m);
                if vx != byte(m, 0x8ea7) || vy != byte(m, 0x8ea8) {
                    perto_da_bola(m);
                    vx = byte(m, 0x8ea7);
                    vy = byte(m, 0x8ea8);
                }
            }
            i += 1;
        }
    }
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_laco_do_motor(m: usize, n: u32) -> u32 {
    // ponytail: orquestrador sem conta propria; a fita dele fotografaria o motor a cada decisao de
    // jogador. Validado pela carreira (Rust x original); as rotinas que ele chama seguem comparadas.
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(320)(m, n)
}

// ---------- centesima quadragesima setima leva: o chute (0x702050, 0x6d5950, 0x6d5a70) ----------

/// the side is losing on aggregate (0x6ce310 without bonus) or, in a first leg with no third phase and no qualified
/// side, level
/// Original: `behind_on_aggregate` (0x6d5950, match_eng.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d5950 } (ret 4, match_eng.cpp): o lado `lado` esta atras no agregado (0x6ce310 sem bonus): perde
/// no placar somado ou, no jogo de ida sem terceira fase nem classificado, empata.
pub(crate) unsafe extern "thiscall" fn atras_no_agregado(m: usize, lado: i8) -> u8 {
    let (mut casa, mut fora, mut classificado, mut ida, mut fase3) = (0i8, 0i8, 0u8, 0u8, 0u8);
    placar_agregado(m, lado, &mut casa, &mut fora, &mut classificado, &mut ida, &mut fase3, 0);
    if casa < fora {
        return 1;
    }
    (ida != 0 && fase3 == 0 && classificado == 0 && casa <= fora) as u8
}

/// the first active teammate in the deep attack (0x70df40), central (0x70df20) and unpressed (0x6bcbd0)
/// Original: `free_teammate` (0x6d5a70, match_eng.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d5a70 } (ret 4, match_eng.cpp): o primeiro companheiro de `p` (os 20 do lado dele, ativos) no
/// ataque fundo (0x70df40), no meio (0x70df20) e sem ninguem pressionando (0x6bcbd0 sem lista), ou 0.
pub(crate) unsafe extern "thiscall" fn companheiro_livre(m: usize, p: usize) -> usize {
    let l = lado(p);
    for k in 0..20 {
        let j = jogador_do_bloco(m, l, k);
        if byte(j, 0x19) < 0 || limite_jogador(j) == 0 || x_fora_do_meio(j) != 0 {
            continue;
        }
        let mut soma = 0i16;
        if pressao_com(m, j, &mut soma, 0, &mut |_, _| {}) == 0 {
            return j;
        }
    }
    0
}

/// Os locais do chute (0x702050) que o desfecho (gol, defesa, rebote) le: pelo deslocamento na pilha.
pub(crate) struct Chute {
    pub(crate) p: usize,
    pub(crate) m: usize,
    pub(crate) g: usize,
    pub(crate) lado: i8,
    pub(crate) tipo: u8,
    pub(crate) a1: u32,
    pub(crate) v: i32,
    pub(crate) v0: u32,
    pub(crate) som: u32,
    pub(crate) modo: u8,
    pub(crate) conta: i8,
    pub(crate) lista: usize,
    pub(crate) d: f32,
    pub(crate) l10: u8,
    pub(crate) l11: u8,
    pub(crate) l14: u8,
    pub(crate) l16: i8,
    pub(crate) l17: u8,
    pub(crate) l18: bool,
    pub(crate) l19: bool,
    pub(crate) l1a: bool,
    pub(crate) l1b: i8,
    pub(crate) l20: u8,
    pub(crate) l25: u8,
    pub(crate) l28: i8,
    pub(crate) l38_antes: i16,
    pub(crate) l40: i16,
    pub(crate) l50: i8,
    pub(crate) l54: i8,
    pub(crate) l58: i16,
    pub(crate) l5c: i32,
    pub(crate) l60: i32,
    pub(crate) l64: i32,
    pub(crate) l68: u32,
    pub(crate) e: usize,
    pub(crate) l8c: i16,
    pub(crate) l12: i8,
    pub(crate) l13: i8,
    pub(crate) l15: u8,
    pub(crate) l26: u8,
    pub(crate) l4e: u8,
    pub(crate) l4f: u8,
}

/// Narracao com sete argumentos pela fita (codigo | a7 << 16, a5 | a6 << 8).
pub(crate) fn narrar_finalizacao(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8) {
    unsafe {
        let narr = *((m + 0x8f7a) as *const usize);
        chamar(0x6d6f70, narr, [(codigo & 0xffff) | (a7 as u8 as u32) << 16, m as u32, a3, a4, (a5 & 0xff) | (a6 & 0xff) << 8]);
    }
}

/// { 0x702050 } (ret 0x18, match_pl.cpp, ~35 KB) com as chamadas que mexem fora do motor injetadas:
/// o chute de `p` (tipo a1, valor a2, narracao a3, modo a4: 1 jogo, 2 sem desafiante, 3 penaltis, 4
/// cobranca, 5/6 repetir; a5/a6 a lista dos companheiros da pressao).
#[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
pub(crate) unsafe fn chutar_com(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let m = u(p, 0x19e);
    let lado = byte(p, 0x27);
    let tipo = a1 as u8;
    let mut v = a2 as i32;
    let mut som = a3;
    let mut modo = a4 as u8;
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, a5, a6, a7);
    // 0x7002f0: dois campos -1; 0x70a860: tres
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let l60 = *dw(m, 0x4786);
    let l5c = *dw(m, 0x4782);
    let alvo_y = if lado == 0 { 0xb } else { 0 };
    let d = distancia_ao_ponto(p, 4, alvo_y);
    let (l58, l50, l54) = (*w(p, 0x39), byte(p, 0x102), byte(p, 0x103));
    let l8c = *w(p, 0x198);
    let mut l64: i32 = 0xf0;
    let v0 = a2;
    let mut l28: i8 = 6;
    let l25 = quantos_marcam(m, p);
    let minuto = (*w(m, 0x8ed0) as i32 / 11) as i16;
    let l16 = minuto as i8;
    *b(m, 0x8d4e) = lado as u8;
    *w(m, 0x8d4c) = minuto;
    let l18 = !((*dw(m, 0x4786) as f64) < f64_em(0x96d810));
    let l1b = ((ftol(d as f64 * f32_em(0x96d754) as f64 + f64_em(0x968e18)) as i8 as i32 / 5) * 5) as i8;
    if tipo == 0x39 || tipo == 3 {
        l28 = 5;
    }
    if tipo == 0x33 {
        l64 = 0x78;
    }
    let d2 = distancia_ao_ponto(p, 4, alvo_y);
    let mut l7c = 0u16;
    let mut l10 = classificar_lance(p, tipo as i8 as i16, l50, l54, 4, if lado == 1 { 0 } else { 0xb }, (d2 as f64 + f64_em(0x967890)) as f32, &mut l7c);
    *w(p, 0x39) = (*w(p, 0x39)).wrapping_add(-(l7c as i16 as i32 / 2) as i16);
    let r = sortear(l7c as i16 as i32);
    v = v.wrapping_sub(r);
    // quem deu o passe ganha a assistencia de chute
    let mut l68 = 0u32;
    let q = u(m, 0xf5a6);
    if p == u(m, 0xf5aa) && q != 0 && !matches!(*w(p, 0x198), 0x12 | 0x11) && (*w(p, 0x29) > -100 || v as i16 > 100) && modo == 1 {
        if !matches!(*w(q, 0x198), 0x34 | 0x37) {
            crate::comparacao::ramo("chute assistencia");
            *b(q, 3) = (*b(q, 3)).wrapping_add(1);
            *w(q, 0x35) = (*w(q, 0x35)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0xe1) as i16);
            *dw(m, 0x4782) = (*dw(m, 0x4782)).wrapping_add(0x113);
        }
        l68 = 1;
    }
    let g = manter_posicao_um(m, (lado == 0) as i8);
    if g == 0 {
        // ponytail: o original para o jogo (match_pl.cpp linha 0x2e0b); sem goleiro nao ha chute
        linha("motor: chutar: sem goleiro");
        std::process::abort();
    }
    let l1c: u8 = match tipo {
        0x16 | 0x39 => 0,
        0x17 | 0x1a | 0x1b => 1,
        0x18 | 0x19 | 0x1c => (sortear(3) != 0) as u8,
        0x3a => (sortear(4) == 0) as u8,
        _ => sortear(2) as u8,
    };
    let (mut l20, mut l14) = (0u8, 0u8);
    let mut l38 = defesa_do_goleiro(g, l50, l54, l1c, tipo as i8, &mut l20, &mut l14) as i16 as i32;
    let (mut l19, mut l1a) = (false, false);
    if l14 == 0 {
        if l38 as i16 > 0x177 {
            l1a = true;
        } else if l38 as i16 > 0x104 {
            l19 = true;
        }
    }
    // o desafiante
    let (mut l4c, mut l4d, mut l70, mut l30, mut l84) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = if byte(m, 0x8ea9) != 4 && modo != 4 && tipo != 0x33 && tipo != 0x35 {
        desafio_adversario(m, p, lado, 0x6d, &mut l4c, &mut l4d, &mut l70, &mut l30, &mut l84, l20)
    } else {
        0
    };
    lesao_na_dividida_com(m, p, 0x6d, e, l30 as i16, 0, chamar);
    if e != 0 {
        let mut penalti = false;
        let mut l44 = 0u8;
        'disputa: {
            if l70 < *w(p, 0x19c) {
                // sem falta: a disputa limpa
            } else {
                let (mut l88, mut l74) = (0u8, 0i8);
                let mut l2c = falta_no_motor(m, e, p, l30, -1, &mut l88, &mut l74, &mut l44);
                if byte(m, 0x8ebb) != 0
                    && na_area_atacada(lado, byte(e, 0x102), byte(e, 0x103))
                    && (codigo_especial(l30) != 0 || l44 != 0 || sortear(*w(e, 0x31) as i32) < -10)
                    && sortear(3) != 0
                {
                    n7(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), 0xff, 0xff, 0);
                }
                if l2c != 0 {
                    if sortear(0x32) == 0 && modo != 4 && l84 == 0 {
                        crate::comparacao::ramo("chute falta so registra");
                        registrar_lance(m, l30 as i16 as u16, e as u32, p as u32, 0, l2c, l50 as u8, l54 as u8, l88, l74 as u8);
                        modo = 2;
                        break 'disputa;
                    }
                    crate::comparacao::ramo("chute falta punida");
                    let mut peso = 0f32;
                    motor_punir_lance(m, e, p, &mut l2c, l88 as i8, l74, l84 as u8, &mut peso, l44);
                    n7(chamar, narracao_da_falta(l30), ub(e, 0x2e), ub(e, 0x27), ub(p, 0x2e), ub(p, 0x27), 0);
                    if modo == 4 {
                        // na cobranca, a falta vira tiro livre: o chute de novo (tipo 0x33)
                        crate::comparacao::ramo("chute falta na cobranca");
                        n7(chamar, 0x1f7a, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
                        let mut t = 0x33i8;
                        let mut s = som as u16;
                        let val = valor_do_chute(p, &mut t, &mut s);
                        let novo_som = (som & 0xffff_0000) | s as u32;
                        chamar(0x702050, p, [(a1 & 0xffff_ff00) | t as u8 as u32, val as i16 as i32 as u32, novo_som, 4, 0]);
                        return;
                    }
                    falta_no_chute_com(m, e, p, l2c, l74, l44, l30, peso, chamar);
                    return;
                }
            }
            penalti = l44 != 0 && (faixa_jogador(p, (lado == 0) as u8) as u8 != 0 || l84 != 0);
        }
        // a disputa limpa (0x702675)
        *w(e, 0x198) = l30 as i16;
        *b(e, 4) = (*b(e, 4)).wrapping_add(1);
        let t = tabela_a(m, e);
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if t & 0x30 != 0 && tabela_a(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
        if v as i16 > l70 {
            crate::comparacao::ramo("chute passa pelo desafiante");
            *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(l4d);
        } else {
            crate::comparacao::ramo("chute desarmado");
            chute_desarmado_com(m, p, e, l30, l70, v, l4c, l44, penalti, chamar);
            return;
        }
    } else if !matches!(tipo, 0x33 | 0x35 | 3 | 0x39) && modo != 4 {
        // sem desafiante: a narracao da aproximacao
        if faixa_jogador(p, (lado == 0) as u8) as u8 != 0 && a5 as u8 == 0 && codigo_de_bola_parada(*w(p, 0x198)) != 0 {
            na(chamar, 0x1f43, ub(p, 0x2e), lado as u8 as u32, 0);
        } else if sortear(10) == 0 && (0x14..=0x23).contains(&l1b) && faixa_jogador(p, (lado == 0) as u8) as u8 == 0 && byte(p, 0x102) < 6 && byte(p, 0x102) > 2 {
            na(chamar, 0x2161, ub(p, 0x2e), lado as u8 as u32, l1b);
        }
    }
    // 0x703634: a narracao do chute
    if l10 == byte(p, 0x2c) as u8 && sortear(10) != 0 {
        l10 = 0xff;
    }
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    if tipo == 3 {
        if a5 as u8 == 0 && e == 0 {
            na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 7);
        } else {
            som = pela_classe(som, l10);
            na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 0);
        }
    } else if tipo == 0x1b || tipo == 0x1a {
        if tg() != 1 {
            na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 0);
        } else if l20 == 4 && distancia_ao_ponto(p, byte(g, 0x102), byte(g, 0x103)) as f64 > f64_em(0x968940) {
            crate::comparacao::ramo("chute goleiro adiantado");
            n7(chamar, 0x1fe0, ub(p, 0x2e), lado as u8 as u32, ub(g, 0x2e), (lado == 0) as u32, 0);
            n7(chamar, som, ub(p, 0x2e), lado as u8 as u32, ub(g, 0x2e), ub(g, 0x27), 0);
            l20 = 0;
        } else {
            let novo = match som as u16 as i16 as i32 {
                0x1f89 => Some(0x1f8a),
                0x1f8b => Some(0x1f8c),
                0x1fa3 => Some(0x1fa4),
                0x1fa5 => Some(0x1fa6),
                0x1fb1 => Some(0x1fb2),
                0x1fb3 => Some(0x1fb4),
                _ => None,
            };
            if let Some(c) = novo {
                som = c;
            }
            l20 = 0;
            n7(chamar, som, ub(p, 0x2e), lado as u8 as u32, ub(g, 0x2e), ub(g, 0x27), 0);
        }
    } else {
        som = pela_classe(som, l10);
        na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 0);
    }
    // 0x703883: o impedimento
    *w(p, 0x198) = 0x6d;
    if tipo != 0x33 && tipo != 0x35 && modo == 1 {
        let o = motor_procurar_impedimento(m, lado, byte(p, 0x103), if lado == 0 { 0xb } else { 0 }, p);
        if o != 0 {
            if sortear(500) != 0 {
                crate::comparacao::ramo("chute impedimento registrado");
                registrar_lance(m, 0xffff, o as u32, 0, 0, 8, *b(o, 0x102), *b(o, 0x103), 0xff, 0xff);
                modo = 2;
            } else {
                crate::comparacao::ramo("chute impedimento");
                na(chamar, 0x1f78, ub(o, 0x2e), lado as u8 as u32, 0);
                let k = m + 0xf5ca + (lado as isize * 0x36) as usize;
                *b(k, 0) = (*b(k, 0)).wrapping_add(1);
                *b(o, 0x13) = (*b(o, 0x13)).wrapping_add(1);
                *b(m, 0x8ea7) = *b(o, 0x102);
                *b(m, 0x8ea8) = *b(o, 0x103);
                *b(m, 0x8eb2) = 7;
                *b(m, 0x8eae) = (lado == 0) as u8;
                *((m + 0xf582) as *mut usize) = 0;
                return;
            }
        }
    }
    *b(m, 0x8eae) = 0xff;
    *b(m, 0x1a7) = 0;
    let l40 = *w(p, 0x29);
    *w(p, 0x29) = 0;
    *w(p, 0x19c) = 0;
    let mut l11 = 0u8;
    if tipo != 0x33 && modo != 4 {
        let forca = if (v as i16) < 0 { 0 } else { v as i16 as i32 };
        l11 = chamar(0x6cc540, m, [l50 as u8 as u32 | (l54 as u8 as u32) << 8, forca as u32, (lado == 0) as u32, l28 as u8 as u32, (tipo == 0x35) as u32]) as u8;
        if byte(m, 0x8eb2) != 0 {
            return;
        }
    }
    if u(m, 0xf582) == p {
        *((m + 0xf582) as *mut usize) = 0;
    }
    if l11 == 2 {
        // o chute bateu em alguem no caminho
        crate::comparacao::ramo("chute bloqueado");
        let q = u(m, 0xf5ae);
        if l28 == 5 {
            na(chamar, 0x1fd4, ub(q, 0x2e), ub(q, 0x27), 5);
        } else {
            na(chamar, 0x1fd3, ub(q, 0x2e), ub(q, 0x27), l28);
        }
        let s1 = sortear(2000);
        let s2 = sortear(1000);
        v = v.wrapping_add(s2.wrapping_sub(s1));
        *w(p, 0x39) = 0;
    }
    // a classe do desfecho
    let tem = l1c != 0;
    l10 = if v as i16 > l64 as i16 {
        if sortear(v as i16 as i32) < 10 { 0xe + tem as u8 } else { 1 }
    } else if (v as i16) < 10 {
        if tem { if sortear(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let dif = l64 as i16 as i32 - v as i16 as i32;
        if dif > 0x3c {
            if tem { if sortear(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if dif > 0x19 {
            8 + tem as u8
        } else if dif > 0xe {
            0xa + tem as u8
        } else {
            0xc + tem as u8
        }
    };
    let no_alvo = matches!(l10, 1 | 0xe | 0xf);
    match modo {
        3 | 4 => {
            let k = m + 0x1a9 + lado as isize as usize;
            *b(k, 0) = (*b(k, 0)).wrapping_add(1);
        }
        1 => {
            if tipo == 0x33 {
                *b(p, 0xd) = (*b(p, 0xd)).wrapping_add(1);
            }
            let q = u(m, 0xf5ae);
            if tipo == 0x33 || q == 0 || (byte(q, 0x102) == 4 && (byte(q, 0x103) == 0 || byte(q, 0x103) == 0xb)) {
                *b(p, 0xa) = (*b(p, 0xa)).wrapping_add(1);
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(if tipo != 0x33 { -150 } else { -1000 });
                if no_alvo {
                    *b(p, 0xb) = (*b(p, 0xb)).wrapping_add(1);
                    let k: i32 = if tipo != 0x33 { 0xdc } else { 400 };
                    if l10 == 0xe || l10 == 0xf {
                        *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(ftol(k as f64 * f64_em(0x968968)) as i16);
                    } else {
                        *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(k as i16);
                    }
                }
            }
            if !no_alvo {
                let di = ((*w(p, 0x39)).wrapping_sub(v as i16).wrapping_add(0xf0)).clamp(0, 200);
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(di);
                if sortear(byte(u(p, 0x69), 0x5a) as i32) == 0 && ajustar_17(p, (-(di as i32 / 50)) as i8) != 0 {
                    desempenho_na_partida(p, 0);
                }
            }
        }
        _ => {}
    }
    if l11 == 1 {
        // a bola parou no caminho
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        if bx < 0 || by >= 0xc || (bx >= 9 && by < 0) {
            chamar(0x6cd490, m, [(lado == 0) as u32, 0, 0, 0, 0]);
        }
        *b(m, 0x1cd) = 1;
        return;
    }
    if lado == 1 {
        *b(m, 0x8ea7) = 4;
        *b(m, 0x8ea8) = 0;
    } else if lado == 0 {
        *b(m, 0x8ea7) = 4;
        *b(m, 0x8ea8) = 0xb;
    }
    l64 = l1b as i32;
    if l1b as i32 > sortear(0xf) + 0x19 {
        // chute de longe: todos decidem de novo com a bola parada 5
        crate::comparacao::ramo("chute de longe");
        let guardado = *b(m, 0x8ea9);
        *b(m, 0x8ea9) = 5;
        for l in 0..2i8 {
            for k in 0..20 {
                let j = jogador_do_bloco(m, l, k);
                if (0..=10).contains(&byte(j, 0x19)) && byte(j, 0x2b) == 0 && j != p {
                    chamar(0x7103e0, j, [0; 5]);
                }
            }
        }
        *b(m, 0x8ea9) = guardado;
    }
    if l20 != 0 {
        if modo == 1 && tg() == 1 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_sub(800);
            if sortear(byte(u(g, 0x69), 0x5a) as i32 * 20 + 10) == 0 {
                let r = sortear(3);
                ajustar_17(g, (r as u8).wrapping_neg() as i8);
            }
            desempenho_na_partida(g, 0);
        }
        if (lado == 0 && byte(p, 0x102) > byte(g, 0x102)) || (lado == 1 && byte(p, 0x102) < byte(g, 0x102)) {
            v = sortear(400).wrapping_add(v).wrapping_add(0xbe);
        } else {
            v = v.wrapping_add(0xbe);
        }
    }
    if u(m, 0xf5ae) == g {
        l38 = 0;
    }
    if tipo == 0x39 {
        *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(2);
    }
    let (mut l12, mut l13, mut l26, mut l4e, mut l4f) = (0i8, 0i8, 0u8, 0u8, 0u8);
    placar_agregado(m, lado, &mut l12, &mut l13, &mut l26, &mut l4e, &mut l4f, 1);
    let l38_antes = l38 as i16;
    l38 = (l38 as i16 as i32) - if l14 != 0 { 100 } else { 0 } - *w(p, 0x39) as i32 + v as i16 as i32 - 0x64;
    let fator = match tipo {
        5 => f32_em(0x96907c),
        0x17 | 0x1c => f32_em(0x96b004),
        0x18 => f32_em(0x96b008),
        0x19 => f32_em(0x96b00c),
        0x1a | 0x39 => f32_em(0x96d420),
        0x1b => f32_em(0x9689a8),
        0x35 => f32_em(0x969584),
        0x3a => f32_em(0x968f18),
        _ => f32_em(0x968e70),
    } as f64;
    let ganho = if l38 < 0 { l38 as f64 / fator } else { l38 as f64 * fator };
    *dw(m, 0x4786) = (*dw(m, 0x4786)).wrapping_add(ftol(ganho));
    let c = Chute {
        p, m, g, lado, tipo, a1, v, v0, som, modo, conta: a5 as u8 as i8, lista: a6, d, l10, l11, l14, l16,
        l17: 0, l18, l19, l1a, l1b, l20, l25, l28, l38_antes, l40, l50, l54, l58, l5c, l60, l64, l68,
        e, l8c, l12, l13, l15: 0, l26, l4e, l4f,
    };
    if no_alvo {
        chute_no_alvo_com(c, chamar);
        return;
    }
    chute_fora_com(&c, chamar);
}

/// A narracao do chute pela classe (0x7037ee): os codigos 0x1f86, 0x1f8e, 0x1f93, 0x1f96 e 0x1f99
/// passam ao seguinte (+1 com a classe diferente de zero); a classe 0xff (a do lance repetido) nao muda.
pub(crate) fn pela_classe(som: u32, classe: u8) -> u32 {
    if classe == 0xff {
        return som;
    }
    let k = (classe != 0) as u32;
    match som as u16 as i16 as i32 {
        0x1f86 => 0x1f87 + k,
        0x1f8e => 0x1f8f + k,
        0x1f93 => 0x1f94 + k,
        0x1f96 => 0x1f97 + k,
        0x1f99 => 0x1f9a + k,
        _ => som,
    }
}

/// A falta do desafiante no chute (0x702050, de 0x702934 a 0x702f1a): como a do passe, sem o evento
/// 0x82 e sem as narracoes do lance 0x2c/0x2f; o perto do gol vem de 0x70df70.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn falta_no_chute_com(m: usize, e: usize, p: usize, tipo: u8, grav: i8, vantagem: u8, lance: i8, peso: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let lado = byte(p, 0x27);
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, 0);
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, 0);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, 0xff, a4, 0xff, 0xff, 0);
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
        crate::comparacao::ramo("chute penalti");
        if cor as i16 == -1 {
            match sortear(4) {
                0 => n7(chamar, 0x20e9, 0xff, lado as u8 as u32),
                1 => n7(chamar, 0x20ec, 0xff, lado as u8 as u32),
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
        n7(chamar, 0x1f7a, 0xff, lado as u8 as u32);
    }
    let expulso = matches!(tipo, 3..=6);
    if cor as i16 > -1 {
        if sortear(4) == 0 {
            let x = sortear(grav as i32);
            if x > sortear(0x14) {
                let t = u(m, 0x4792);
                let clube = u(t, if byte(e, 0x27) == 0 { 0x20 } else { 0x1c });
                let k = u(clube, 0x53);
                if k != 0 && byte(k, 0x77) == 8 {
                    n7(chamar, 0x20f5, 0, 0);
                }
            }
            n7(chamar, 0x2109, ub(e, 0x2e), ub(e, 0x27));
        }
        n7(chamar, cor as u32, ub(e, 0x2e), ub(e, 0x27));
        if expulso {
            crate::comparacao::ramo("chute expulsao");
            jogador_saiu_com(m, e, 0, chamar);
        }
    }
    if som != 0x1f7a {
        let c = escolher_cobrador(m, lado, byte(m, 0x8eb2), 0);
        if c == 0 {
            n7(chamar, som, 0xff, lado as u8 as u32);
        } else {
            if tipo == 7 {
                n7(chamar, 0x21e2, ub(c, 0x2e), lado as u8 as u32);
            } else {
                n7(chamar, som, 0xff, lado as u8 as u32);
            }
            if som == 0x1f79 && vantagem == 0 {
                if tipo == 7 {
                    if perto_do_gol(p) != 0 && faixa_jogador(p, (lado == 0) as u8) as u8 == 0 {
                        n7(chamar, 0x21e1, ub(c, 0x2e), lado as u8 as u32);
                    }
                } else if fora_do_meio(byte(p, 0x102)) != 0 && perto_do_gol(p) != 0 {
                    na(chamar, 0x21df, ub(c, 0x2e), lado as u8 as u32);
                } else if perto_do_gol(p) != 0 {
                    na(chamar, 0x21e0, ub(c, 0x2e), lado as u8 as u32);
                }
            }
        }
    }
    if codigo_especial(lance) == 0 {
        let extra = expulso || som == 0x1f7a;
        let zero = (lado == 0) as u32;
        if vantagem != 0 {
            na(chamar, 0x20dd, ub(p, 0x2e), lado as u8 as u32);
            if tipo == 4 || tipo == 5 {
                *b(e, 0x16) = 1;
            }
            if extra && sortear(2) == 0 {
                nb(chamar, 0x20d9, zero);
            }
        } else if sortear(9) == 0 {
            nb(chamar, 0x20df, zero);
            if sortear(3) == 0 {
                nb(chamar, 0x20e0, zero);
            }
            if extra && sortear(2) == 0 {
                nb(chamar, 0x20d9, zero);
            }
        }
        aplicar_ao_time(m, e, peso);
        chamar(0x716e10, e, [0; 5]);
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// O desafiante tirou a bola no chute (0x702050, de 0x702f35 a 0x70353b): a narracao (na area dele,
/// 1 em 2, a da falta e 0x20e8/0x20ea/0x20eb; senao pelo lance, com a defesa 0x70d430 no lance 1),
/// ele domina (0x6f0630, 0x70a890 com nota - chute - 200), a bola espirra abaixo do limite do lance
/// (0x6cd770) ou ele aponta para o lado do campo; os ganhos e o +0x2b de quem chutou.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn chute_desarmado_com(m: usize, p: usize, e: usize, lance: i8, nota: i16, v: i32, a: u8, vantagem: u8, penalti: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let lado = byte(p, 0x27);
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, a5, a6, 0);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, 0xff, a4, 0xff, 0xff, 0);
    let narrou = (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
        && faixa_jogador(e, byte(e, 0x27) as u8) as u8 != 0
        && sortear(2) == 0;
    if narrou {
        crate::comparacao::ramo("chute desarme narrado na area");
        n7(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), ub(p, 0x2e), ub(p, 0x27));
        let zero = (byte(e, 0x27) == 0) as u32;
        match sortear(0x32) {
            0 => nb(chamar, 0x20eb, zero),
            1 => nb(chamar, 0x20ea, zero),
            _ => nb(chamar, 0x20e8, zero),
        }
    } else {
        let codigo = match lance {
            1 => {
                crate::comparacao::ramo("chute desarme do goleiro");
                if defender_com(e, (nota as i32).wrapping_sub(v) as i16, chamar) == 0 {
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
            n7(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), ub(p, 0x2e), ub(p, 0x27));
        }
    }
    chamar(0x6f0630, e, [0; 5]);
    gravar_19c(e, (nota as i32).wrapping_sub(v).wrapping_sub(200) as i16);
    let mut dominou = false;
    let limite: i16 = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
        crate::comparacao::ramo("chute bola espirra");
        chamar(0x6cd770, m, [ub(p, 0x102), ub(p, 0x103), 1 | ((lado == 0) as u32) << 8, 0, 0]);
        if byte(m, 0x8eb2) == 0 {
            let mut k = 3;
            let (c10d, c10c) = (byte(e, 0x10d), byte(e, 0x10c));
            k -= (c10d as i32 > sortear(10)) as i32;
            k -= (c10c as i32 > sortear(10)) as i32;
            *b(e, 0x2b) = sortear(k) as u8;
            dominou = true;
        } else if penalti {
            chamar(0x716c90, p, [0; 5]);
        }
    } else {
        if byte(m, 0x8ea9) < 4 && byte(m, 0x8ea9) > -1 {
            *b(m, 0x8ea9) = 0;
        }
        let (x, y) = (byte(p, 0x102), byte(p, 0x103));
        if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
            apontar(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            apontar(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let me = u(e, 0x19e);
    let t = || if me != 0 { tabela_a(me, e) } else { 0 };
    let ganho: i16 = if dominou { 0x78 } else { 0xc8 };
    let longe = t() & 4 == 0 && t() & 2 == 0;
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if longe { ganho << 1 } else { ganho });
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(a);
    if faixa_jogador(p, (lado == 0) as u8) as u8 != 0 {
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
    }
    if dominou && sortear(7) == 0 {
        narrar_finalizacao(chamar, m, 0x1f41, ub(e, 0x2e), ub(e, 0x27), 0xff, 0xff, 0);
    }
}

/// O nome (+0x69 -> +8) da ficha e "Shearer" (0xa16110): a comemoracao dele.
pub(crate) unsafe fn e_o_shearer(p: usize) -> bool {
    let ficha = *((p + 0x69) as *const usize);
    let nome = *((ficha + 8) as *const *const u8);
    std::ffi::CStr::from_ptr(nome as *const std::ffi::c_char).to_bytes() == b"Shearer"
}

/// Os destaques do jogador (0x717470), na ordem: o primeiro que valer e narrado com (q.2e, a4, 0).
pub(crate) unsafe fn destaques(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, q: usize, a4: u32, codigos: &[u32]) -> bool {
    for &c in codigos {
        if destaque_do_jogador(q, c) != 0 {
            narrar_finalizacao(chamar, m, c, byte(q, 0x2e) as u8 as u32, a4, 0xff, 0xff, 0);
            return true;
        }
    }
    false
}

pub(crate) const DESTAQUES_P: [u32; 4] = [0x219e, 0x21b1, 0x21b2, 0x21b7];
pub(crate) const DESTAQUES_R: [u32; 6] = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
pub(crate) const DESTAQUES_GOL: [u32; 7] = [0x219f, 0x21a0, 0x21a1, 0x21b0, 0x21b3, 0x21b4, 0x21b5];

/// O chute no alvo (0x702050 de 0x706cba ao fim): a defesa do goleiro ou o gol.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn chute_no_alvo_com(mut c: Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let ajustar = |q: usize, delta: i32| {
        if ajustar_17(q, delta as i8) != 0 {
            desempenho_na_partida(q, 0);
        }
    };
    // o gol pela razao chute / (defesa + chute)
    let l6c = (c.l38_antes as i32).wrapping_add(c.v as i16 as i32);
    let mut razao: f32;
    let gol = if l6c != 0 {
        razao = ((c.v as i16 as i32).wrapping_mul(100) as f64 / l6c as f64) as f32;
        if m != 0 && !(razao as f64 > f32_em(0x96d80c) as f64) && matches!(byte(m, 1), 2 | 8) {
            razao = (razao as f64 + f32_em(0x9689a8) as f64) as f32;
        }
        razao as f64 > f32_em(0x96d808) as f64
    } else {
        razao = 0.0;
        false
    };
    if !gol {
        if c.l19 && tg() == 1 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(0x15e);
            if sortear(byte(g, 0x17) as i32 * 20 + 0x32) == 0 {
                let r = sortear(4);
                ajustar(g, r);
            }
        } else if c.l1a && tg() == 1 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(0x258);
            if sortear((byte(g, 0x17) as i32 + 1) * 20) == 0 {
                let r = sortear(7);
                ajustar(g, r);
            }
        }
        if !(razao as f64 > f32_em(0x96d808) as f64) {
            defesa_no_chute_com(c, razao, chamar);
            return;
        }
    }
    // o gol (0x706dfd)
    crate::comparacao::ramo("chute gol");
    let modo = c.modo;
    if modo == 1 {
        if lado_do_lance(m) as i8 > -1 {
            limpar_lance(m);
        }
        let k = u(m, 0x478a);
        if c.tipo == 0x33 && k != 0 && k != p {
            *b(k, 0x10) = (*b(k, 0x10)).wrapping_add(1);
        }
        let base = m + (lado as isize * 0x36) as usize;
        match byte(m, 0x8eb1) {
            -1 | 7 => *b(base, 0xf5c4) = (*b(base, 0xf5c4)).wrapping_add(1),
            3 => *b(base, 0xf5c3) = (*b(base, 0xf5c3)).wrapping_add(1),
            6 => *b(base, 0xf5c5) = (*b(base, 0xf5c5)).wrapping_add(1),
            _ => {}
        }
        if momento_marcado(m) as u8 != 0 {
            *b(m, 0x1a8) = 1;
        }
        if c.tipo == 0x33 {
            *b(p, 0xe) = (*b(p, 0xe)).wrapping_add(1);
        }
    } else if modo == 3 || modo == 4 {
        gol_nos_penaltis_com(&c, chamar);
        gol_validado_com(&mut c, chamar);
        return;
    }
    if c.l11 == 0 {
        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
            *w(p, o) = (*w(p, o)).wrapping_add(1);
        }
        *b(p, 0xc) = (*b(p, 0xc)).wrapping_add(1);
        c.l17 = 1;
    }
    if modo == 1 {
        if c.l11 == 0 {
            if c.tipo == 0x33 {
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x2bc);
            } else {
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x28a);
                if sortear((byte(p, 0x17) as i32 + 1) * 20) == 0 {
                    let r = sortear(8);
                    ajustar(p, r);
                }
            }
            let piso = (ub(p, 0xc) as i32 + 5) * 1000;
            if (*w(p, 0x35) as i32) <= piso {
                *w(p, 0x35) = piso as i16;
            }
        } else {
            *b(p, 0x10) = (*b(p, 0x10)).wrapping_add(1);
            *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x113);
            if sortear(byte(p, 0x17) as i32 * 20 + 0x32) == 0 {
                let r = sortear(4);
                ajustar(p, r);
            }
            let piso = ftol((ub(p, 0x10) as f64 * f32_em(0x968f18) as f64 + f64_em(0x96afa0)) * f64_em(0x968908)) as i16;
            if *w(p, 0x35) <= piso {
                *w(p, 0x35) = piso;
            }
        }
        if tg() == 1 {
            // o goleiro sofreu o gol
            *b(g, 0xf) = (*b(g, 0xf)).wrapping_add(1);
            let ficha = u(g, 0x69);
            if sortear(byte(ficha, 0x58) as i32 + 1) != 0 && sortear(byte(ficha, 0x5a) as i32 + 1) != 0 {
                let r = sortear(ub(g, 0xf) as i32 * ub(g, 0xf) as i32) as i8;
                let soma = if (r as i32) < ub(p, 0xf) as i32 { *b(p, 0xf) } else { r as u8 };
                *b(g, 0x100) = (*b(g, 0x100)).wrapping_add(soma);
                desempenho_na_partida(g, 0);
            }
            if c.tipo != 0x33 {
                *w(g, 0x35) = (*w(g, 0x35)).wrapping_sub(500);
            }
            let di = *w(g, 0x35);
            if c.l14 != 0 {
                let mut v = (c.d as f64 + f64_em(0x9689e0)) * f64_em(0x96d800) * f64_em(0x968968);
                if !(v > f64_em(0x96d7f8)) {
                    v = f64_em(0x96d7f8);
                }
                *w(g, 0x35) = (ftol(v) as i16).wrapping_add(di);
            }
        }
        let q = u(m, 0xf5a6);
        if p == u(m, 0xf5aa) && q != 0 {
            *b(q, 0x10) = (*b(q, 0x10)).wrapping_add(1);
            *w(q, 0x35) = (*w(q, 0x35)).wrapping_add(0x113);
            if c.l68 == 0 && !matches!(*w(q, 0x198), 0x34 | 0x37) {
                *b(q, 3) = (*b(q, 3)).wrapping_add(1);
                *w(q, 0x35) = (*w(q, 0x35)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0xe1) as i16);
                *dw(m, 0x4782) = (*dw(m, 0x4782)).wrapping_add(0x113);
            }
        }
    }
    // 0x70717c: as narracoes do gol
    if c.tipo != 0x33 && c.tipo != 0x3a && c.l14 == 0 && sortear(0x32) == 0 && c.l20 == 0 && tg() == 1 {
        na(chamar, if c.tipo == 0x1a { 0x21d2 } else { 0x20d6 }, ub(g, 0x2e), ub(g, 0x27), 0);
    }
    na(chamar, 0x1fff, ub(p, 0x2e), lp, 0);
    if c.tipo == 0x33 {
        na(chamar, if modo == 1 { 0x1fcf } else { 0x20f8 }, ub(p, 0x2e), lp, 0);
    } else {
        *dw(m, 0x4782) = (*dw(m, 0x4782)).wrapping_sub(*dw(m, 0x4786));
        narrar_o_gol_com(&mut c, chamar);
    }
    gol_validado_com(&mut c, chamar);
}

/// As narracoes do gol pelo momento (0x70724d a 0x707cc6): a virada no placar (+0x4782, +0x4786), o
/// gol de quem esta atras no agregado, a comemoracao do Shearer, quem marcava, a assistencia e o
/// goleiro.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn narrar_o_gol_com(c: &mut Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, a5, a6, 0);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let ajustar = |q: usize, delta: i32| {
        if ajustar_17(q, delta as i8) != 0 {
            desempenho_na_partida(q, 0);
        }
    };
    let quem_bloqueou = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| {
        let q = u(m, 0xf5ae);
        na(chamar, codigo, ub(q, 0x2e), ub(q, 0x27), 0);
    };
    let atras = || atras_no_agregado(m, lado) != 0 && {
        let r = sortear(10);
        byte(u(p, 0x69), 0x58) > (r as u8).wrapping_add(0xe) as i8
    };
    let l4786 = *dw(m, 0x4786);
    let l4782 = || *dw(m, 0x4782) as f64;
    if lado_do_lance(m) as i8 > -1 || c.modo == 6 || c.modo == 5 {
        // 0x707c86
        if c.l11 == 0 {
            na(chamar, 0x20f9, ub(p, 0x2e), lp, 0);
        } else {
            quem_bloqueou(chamar, 0x20fa);
        }
        return;
    }
    if c.l18 && c.l11 == 0 {
        na(chamar, 0x201a, ub(p, 0x2e), lp, 0);
        if atras() {
            na(chamar, 0x21be, ub(p, 0x2e), lp, 0);
        }
        return;
    }
    if l4786 > 0x578 {
        let codigo = if l4782() > f64_em(0x96d810) {
            0x2013
        } else if l4782() > f64_em(0x96d7f0) {
            0x2015
        } else {
            0x201b
        };
        na(chamar, codigo, ub(p, 0x2e), lp, 0);
        let r = sortear(8);
        ajustar(p, r);
        if atras() {
            na(chamar, 0x21be, ub(p, 0x2e), lp, 0);
        }
        return;
    }
    if c.l18 && c.l11 != 0 {
        na(chamar, 0x201c, ub(p, 0x2e), lp, 0);
        quem_bloqueou(chamar, if c.modo == 1 { 0x1fcd } else { 0x20fa });
        let r = sortear(6);
        ajustar(p, r);
        return;
    }
    if l4786 > 0x384 {
        na(chamar, if l4782() > f64_em(0x96d7f0) { 0x2016 } else { 0x2018 }, ub(p, 0x2e), lp, 0);
        let r = sortear(6);
        ajustar(p, r);
        return;
    }
    if l4782() > f64_em(0x96d810) {
        na(chamar, 0x2019, ub(p, 0x2e), lp, 0);
        let r = sortear(4);
        ajustar(p, r);
        return;
    }
    if c.modo != 1 {
        // 0x707bcb
        if c.l11 != 0 {
            quem_bloqueou(chamar, 0x20fa);
            return;
        }
        na(chamar, 0x20f9, ub(p, 0x2e), lp, 0);
        if byte(m, 0x8ebb) != 0 && sortear(8) == 0 && e_o_shearer(p) {
            na(chamar, 0x216c, ub(p, 0x2e), lp, 0);
        }
        return;
    }
    if c.l11 != 0 {
        quem_bloqueou(chamar, 0x1fcd);
        return;
    }
    na(chamar, 0x1fcc, ub(p, 0x2e), lp, 0);
    if byte(m, 0x8ebb) != 0 && sortear(3) != 0 && e_o_shearer(p) {
        crate::comparacao::ramo("chute gol do Shearer");
        na(chamar, 0x216c, ub(p, 0x2e), lp, 0);
    }
    // quem marcava o artilheiro
    let marcadores = quantos_marcam(m, p);
    c.l16 = marcadores as i8;
    if c.tipo != 0x35 && marcadores != 0 {
        let mk = quem_marca(m, p);
        if mk == 0 {
            return;
        }
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        if c.conta == 0 {
            if c.e != 0 {
                return;
            }
            if marcadores == 1 {
                if *w(mk, 0x198) != 0x6e && *w(mk, 0x35) < 0x1a5e && *w(p, 0x35) > 0x1a5e && minuto > 0xf {
                    n7(chamar, 0x21c2, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
                }
            } else if byte(p, 0x1b) as i32 > 0x1c52 {
                n7(chamar, 0x21c3, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
            }
            return;
        }
        let (pe, me) = (u(p, 0x6d), u(mk, 0x6d));
        if c.e == 0 || pe == 0 || me == 0 || *w(p, 0x35) <= 0x1a5e || *w(mk, 0x35) >= 0x1c52 || mk != c.e {
            return;
        }
        if (c.tipo == 3 || c.tipo == 0x39) && byte(pe, 0x2e) > 10 && byte(pe, 0x2e) as i32 - byte(me, 0x2e) as i32 > 3 {
            n7(chamar, 0x21c4, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
        } else if matches!(c.l8c, 0x70 | 0x6a | 0x69 | 0x12) && byte(pe, 0x36) > 10 && byte(pe, 0x36) as i32 - byte(me, 0x36) as i32 > 3 {
            n7(chamar, 0x21c5, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
        } else if minuto > 0x3c && estado_do_adversario(p) == 0 && estado_do_lado(p) == 1 {
            let k = m + (lado as isize * 0x36) as usize;
            if ub(k, 0xf5fe) > 4 && ub(k, 0xf5d2) > 0x41 && ub(k, 0xf5d1) > 0x37 {
                n7(chamar, 0x21d7, ub(p, 0x2e), lp, ub(p, 0x2e), lp);
            }
        }
        return;
    }
    // 0x707840: o proprio artilheiro, a assistencia ou o goleiro
    let tp = if u(p, 0x19e) != 0 { tabela_a(u(p, 0x19e), p) } else { 0 };
    if tp & 0x30 != 0 && papel_880(p) == 0 && *w(p, 0x35) > 0x1a5e && ub(p, 0xb) > 1 {
        na(chamar, 0x21c7, ub(p, 0x2e), lp, 0);
        return;
    }
    let q = u(m, 0xf5a6);
    if p == u(m, 0xf5aa) && q != 0 {
        let tq = || if u(q, 0x19e) != 0 { tabela_a(u(q, 0x19e), q) } else { 0 };
        let nq = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| na(chamar, codigo, ub(q, 0x2e), ub(q, 0x27), 0);
        if papel_880(q) != 0 && tq() & 0x38 != 0 && matches!(*w(m, 0x8d4a), 0x5e | 6 | 7 | 8) {
            if *w(q, 0x35) <= 0x1a5e {
                return;
            }
            if ub(q, 3) > 1 && ub(q, 0x10) > 1 && ub(q, 1) > 0xa {
                nq(chamar, 0x21c8);
            } else if ub(q, 1) > 0xa && ub(q, 3) > 1 && byte(q, 0x10b) != 0 {
                nq(chamar, 0x21ca);
            }
            return;
        }
        if papel_880(q) == 0 && tq() & 0x30 != 0 && ub(q, 2) > 0xa && *w(q, 0x35) > 0x1a5e && ub(q, 3) > 1 && ub(q, 1) > 0xa && sortear(2) != 0 && byte(q, 0x10b) != 0 {
            nq(chamar, 0x21c9);
        }
        return;
    }
    if c.l14 != 0 {
        return;
    }
    let ng = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| na(chamar, codigo, ub(g, 0x2e), ub(g, 0x27), 0);
    if c.l20 == 0 && tg() == 1 {
        let r = (sortear(10) as u8).wrapping_add(5) as i8;
        if byte(u(g, 0x6d), 0x1c) as i32 + ub(g, 0xf) as i32 > r as i32 {
            let f = ub(g, 0xf) as i32;
            let r2 = (sortear(10) as u8).wrapping_add(5) as i8;
            if byte(u(g, 0x69), 0x57) as i32 + f > r2 as i32 && sortear(f) != 0 {
                let r3 = (sortear(10) as u8).wrapping_add(1) as i8;
                if byte(p, 0x100) as i32 + ub(g, 0xf) as i32 > r3 as i32 {
                    ng(chamar, 0x21d3);
                    return;
                }
            }
        }
    }
    if c.l20 != 0 || tg() != 1 {
        return;
    }
    if byte(u(g, 0x6d), 0x1c) >= 0xa || byte(u(g, 0x69), 0x57) as i32 >= ub(g, 0xf) as i32 + 0xa {
        return;
    }
    let f = ub(g, 0xf) as i32;
    let r = sortear(5);
    if byte(u(g, 0x69), 0x5a) as i32 >= r + f * 2 + 4 || f <= 1 || sortear(f) == 0 {
        return;
    }
    let r2 = (sortear(5) as u8).wrapping_add(1) as i8;
    if byte(p, 0x100) >= r2 {
        return;
    }
    ng(chamar, 0x21d4);
}

/// Repetir o lance (modos 5 e 6): o motor volta aos +0x4786/+0x4782 de antes, narra 0x20f7, o
/// chutador volta a casa e ao +0x39 de antes, domina e chuta de novo no modo 1 (0x704a57 e iguais).
pub(crate) unsafe fn repetir_chute_com(c: &Chute, aplicar: bool, desfazer: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m) = (c.p, c.m);
    *((m + 0x4786) as *mut i32) = c.l60;
    *((m + 0x4782) as *mut i32) = c.l5c;
    if aplicar {
        aplicar_ao_time(m, p, f32::from_bits(0x3fb9999a));
    }
    narrar_finalizacao(chamar, m, 0x20f7, byte(p, 0x2e) as u8 as u32, c.lado as u8 as u32, 0xff, 0xff, 0);
    *((p + 0x39) as *mut i16) = c.l58;
    if desfazer {
        desfazer_gol(p);
    }
    chamar(0x6f46b0, p, [c.l50 as u8 as u32, c.l54 as u8 as u32, 0, 0, 0]);
    chamar(0x6f0630, p, [0; 5]);
    chamar(0x702050, p, [c.a1, c.v0, c.som, 1, 0]);
}

/// Desfaz as contas do gol (+0x3d..+0x45 e +0xc) quando ele e anulado ou repetido.
pub(crate) unsafe fn desfazer_gol(p: usize) {
    for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
        let w = (p + o) as *mut i16;
        *w = (*w).wrapping_sub(1);
    }
    let b = (p + 0xc) as *mut u8;
    *b = (*b).wrapping_sub(1);
}

/// Os companheiros na lista da pressao (a5 = quantos, a6 a lista): com sorteio(n x n + k), 1 narra
/// 0x2154 com o primeiro, 2 (1 em 2) 0x2155 com os dois, senao 0x2132; depois os destaques deles.
/// Com `assistencia`, a de quem marcava (0x21c6) entra como na 0x704d4b.
pub(crate) unsafe fn narrar_companheiros_com(c: &Chute, k: i32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> bool {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let n = c.conta as i32;
    if sortear(n * n + k) == 0 {
        return false;
    }
    let zero = (lado == 0) as u32;
    let reiniciar = || core::ptr::write_unaligned((c.lista + 0xa) as *mut u32, core::ptr::read_unaligned((c.lista + 2) as *const u32));
    if n == 1 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2154, ub(p, 0x2e), lado as u8 as u32, ub(r1, 0x2e), zero, 0);
        destaques(chamar, m, r1, zero, &DESTAQUES_R);
    } else if n == 2 && sortear(2) != 0 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        let r2 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2155, ub(r1, 0x2e), zero, ub(r2, 0x2e), zero, 0);
        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) {
            destaques(chamar, m, r2, zero, &DESTAQUES_R);
        }
    } else {
        narrar_finalizacao(chamar, m, 0x2132, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, 0);
    }
    true
}

/// A assistencia contra quem marcava (0x21c6): o marcador (0x6d5a10) e o companheiro da lista, no
/// jogo (modo 1, nao 0x33/0x35), sem gols nem chutes demais do artilheiro, nos minutos e folegos.
pub(crate) unsafe fn assistencia_ao_marcador(c: &Chute, r: usize) -> bool {
    let (p, m) = (c.p, c.m);
    let w = |q: usize, o: usize| *((q + o) as *const i16);
    let mk = quem_marca(m, p);
    mk != 0
        && c.tipo != 0x33
        && c.tipo != 0x35
        && c.modo == 1
        && byte(p, 0xc) == 0
        && byte(p, 0x10) == 0
        && byte(p, 3) == 0
        && (byte(p, 0xa) as u8) < 5
        && (byte(p, 0xb) as u8) < 2
        && c.l16 > (sortear(0x14) as u8).wrapping_add(0x41) as i8
        && w(p, 0x35) < 0x1c52
        && w(mk, 0x35) > 0x1a5e
        && (byte(p, 0x100) < 1 || sortear(byte(p, 0x100) as i32) == 0)
        && r == mk
}

/// A defesa do goleiro (0x708e84 ao fim): pela razao, ele segura (ate 51), espalma para escanteio
/// (ate 61) ou rebate; as narracoes, os ganhos dele e a bola.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn defesa_no_chute_com(mut c: Chute, razao: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let ng = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar_finalizacao(chamar, m, codigo, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
    let modo = c.modo;
    let classe = if razao as f64 > f32_em(0x96d7e8) as f64 {
        1
    } else if razao as f64 > f32_em(0x96d7e4) as f64 {
        2
    } else {
        3
    };
    crate::comparacao::ramo(match classe {
        1 => "chute defesa rebatida",
        2 => "chute defesa escanteio",
        _ => "chute defesa segura",
    });
    if classe == 3 {
        *b(g, 0x2b) = (*b(g, 0x2b)).wrapping_add(sortear(3) as u8);
        *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(if c.tipo != 0x33 { 200 } else { 1300 });
    }
    // o comeco das narracoes: cobranca, penaltis ou tiro livre
    let mut a_2122 = false;
    if modo == 4 || modo == 3 {
        na(chamar, 0x2008, ub(p, 0x2e), lp, 0);
        if modo == 3 {
            a_2122 = true;
        } else {
            na(chamar, 0x2174, ub(p, 0x2e), lp, 0);
        }
    } else if c.tipo == 0x33 {
        if modo == 1 {
            na(chamar, 0x1fce, ub(p, 0x2e), lp, 0);
        }
        a_2122 = true;
    }
    if a_2122 {
        c.l15 = 1;
        ng(chamar, 0x2122);
    } else {
        let tipo_1a = c.tipo == 0x1a && (classe != 3 || c.l20 == 0);
        if c.l1a {
            ng(chamar, if classe == 3 { 0x2112 } else { 0x2111 });
            destaques(chamar, m, g, ub(g, 0x27), &DESTAQUES_R);
        } else if c.l19 {
            ng(chamar, if classe == 3 && tipo_1a { 0x21d0 } else if classe == 3 { 0x2110 } else { 0x210f });
        } else {
            match classe {
                1 => ng(chamar, 0x1fe6),
                2 => ng(chamar, if c.tipo == 0x1a { 0x21d0 } else { 0x1fe4 }),
                _ => {
                    ng(chamar, if tipo_1a { 0x21d0 } else { 0x1fe5 });
                    destaques(chamar, m, p, lp, &DESTAQUES_P);
                }
            }
        }
    }
    let alto = (*((m + 0x4782) as *const i32) as f64) > f64_em(0x96d810);
    if classe == 1 {
        // rebatida (0x70909d)
        if modo == 5 || modo == 6 {
            repetir_chute_com(&c, false, false, chamar);
            return;
        }
        if modo != 2 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(if c.tipo != 0x33 { 140 } else { 1000 });
        }
        if modo != 1 && modo != 2 {
            return;
        }
        *b(g, 0x2b) = (*b(g, 0x2b)).wrapping_add((sortear(3) + 1) as u8);
        chamar(0x6cd770, m, [ub(m, 0x8ea7), ub(m, 0x8ea8), ((lado == 0) as u32) << 8, ((c.tipo == 0x33) as u32) << 8, 0]);
        *b(m, 0x1cd) = 1;
        if c.l15 != 0 || byte(m, 0x8eb2) == 0 || modo == 4 {
            return;
        }
        if alto || (c.l18 && c.l11 == 0) {
            na(chamar, 0x2020, ub(p, 0x2e), lp, 0);
        } else if !c.l18 && c.l11 == 0 && c.l1b >= 0x19 && modo == 1 {
            na(chamar, 0x1f9e, ub(p, 0x2e), lp, c.l64 as i8);
        }
        return;
    }
    for codigo in [0x21a1, 0x21a2] {
        if destaque_do_jogador(p, codigo) != 0 {
            na(chamar, codigo, ub(p, 0x2e), lp, 0);
        }
    }
    if classe == 2 {
        // para escanteio (0x70950f)
        if c.l15 == 0 {
            if alto || (c.l18 && c.l11 == 0) {
                na(chamar, 0x2020, ub(p, 0x2e), lp, 0);
            }
            if !matches!(modo, 5 | 6 | 2) {
                *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(if c.tipo != 0x33 { 120 } else { 1000 });
            }
        }
        if modo == 4 || modo == 3 {
            return;
        }
        if modo == 5 || modo == 6 {
            repetir_chute_com(&c, false, false, chamar);
            return;
        }
        escanteio_do_chute(&c, chamar);
        return;
    }
    // segura (0x709afb)
    if c.l15 == 0 {
        if alto || (c.l18 && c.l11 == 0) {
            na(chamar, 0x2020, ub(p, 0x2e), lp, 0);
        } else {
            let limite = 0x12c + if c.l1a { 0x190 } else { 0 } + if c.l19 { 0xc8 } else { 0 };
            if *w(p, 0x39) as i32 > limite || c.tipo == 0x3a {
                na(chamar, 0x1fdd, ub(p, 0x2e), lp, byte(p, 0xc));
            }
            if c.l40 < -0x87 && c.tipo != 0x3a {
                narrar_companheiros_com(&c, 3, chamar);
            }
        }
    }
    if modo == 5 || modo == 6 {
        repetir_chute_com(&c, false, false, chamar);
        return;
    }
    chamar(0x6f0630, g, [0; 5]);
}

/// Escanteio para o adversario (0x70961e): a bola na linha de fundo do lado de quem chutou, a
/// reposicao 3 com a bola do lado dele e a narracao 0x1f75.
pub(crate) unsafe fn escanteio_do_chute(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let b = |o: usize| (m + o) as *mut u8;
    match lado {
        1 => *b(0x8ea8) = 0xff,
        0 => *b(0x8ea8) = 0xc,
        _ => {
            // ponytail: o original para o jogo (match_pl.cpp); o lado e sempre 0 ou 1
            linha("motor: chutar: lado invalido no escanteio");
            std::process::abort();
        }
    }
    *b(0x8eae) = *((p + 0x27) as *const u8);
    *((m + 0xf582) as *mut u32) = 0;
    *b(0x8eb2) = 3;
    narrar_finalizacao(chamar, m, 0x1f75, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
}

/// O gol nos penaltis e na cobranca (0x707db2): +1 na conta do lado (+0xf5bc), 0x2000 e, nos
/// penaltis, 0x212e.
pub(crate) unsafe fn gol_nos_penaltis_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let k = (m + 0xf5bc + (lado as isize * 0x36) as usize) as *mut u8;
    *k = (*k).wrapping_add(1);
    narrar_finalizacao(chamar, m, 0x2000, byte(p, 0x2e) as u8 as u32, lado as u8 as u32, 0xff, 0xff, 0);
    if c.modo == 3 {
        narrar_finalizacao(chamar, m, 0x212e, byte(p, 0x2e) as u8 as u32, lado as u8 as u32, 0xff, 0xff, 0);
    }
}

/// O gol anulado (0x20ed): o lance marcado antes (0x6bba50) e aplicado (0x6bac40). Contra o proprio
/// lado, 0x20df, talvez 0x20d9 e fator 3; senao, empatado ou um a frente, 1 em 40 0x20d9, fator
/// 1,45. Quem chutou leva o fator (0x6bb8d0) e cansa (0x716e10); as contas do gol sao desfeitas.
pub(crate) unsafe fn gol_anulado_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar_finalizacao(chamar, m, codigo, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
    crate::comparacao::ramo("chute gol anulado");
    nb(chamar, 0x20ed);
    let l = lado_do_lance(m) as i8;
    aplicar_falta_com(m, chamar);
    let fator: u32;
    if l == lado {
        nb(chamar, 0x20df);
        if sortear(c.l13 as i32 - c.l12 as i32 + 3) > 0 {
            nb(chamar, 0x20d9);
        }
        fator = 0x40400000;
    } else {
        if (c.l12 == c.l13 || c.l12 as i32 - 1 == c.l13 as i32) && sortear(0x28) == 0 {
            nb(chamar, 0x20d9);
        }
        fator = 0x3fb9999a;
    }
    aplicar_ao_time(m, p, f32::from_bits(fator));
    chamar(0x716e10, p, [0; 5]);
    if c.l17 != 0 {
        desfazer_gol(p);
    }
}

/// Depois das narracoes do gol (0x707cc7): o gol pode ser anulado (1 em 3 a cada passo, com lance
/// marcado), as narracoes de marca e recorde, o placar, a virada e o fim (+0x8eb2 = 8).
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn gol_validado_com(c: &mut Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| *((q + o) as *const i16);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let modo = c.modo;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, 0xff, lp, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let anular = |c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| -> bool {
        if sortear(3) != 0 && lado_do_lance(m) as i8 > -1 {
            gol_anulado_com(c, chamar);
            return true;
        }
        false
    };
    if modo != 3 && modo != 4 {
        match c.l10 {
            0xe => na(chamar, 0x2175, ub(p, 0x2e), lp, 0),
            0xf => na(chamar, 0x2176, ub(p, 0x2e), lp, 0),
            _ => {}
        }
    }
    if anular(c, chamar) {
        return;
    }
    let minuto = *((m + 0x8ed0) as *const i16) as i32 / 11;
    'final_: {
        if modo == 3 || modo == 4 {
            aplicar_ao_lado(m, (lado == 0) as i8, f32::from_bits(0x3fa00000));
            break 'final_;
        }
        if c.l11 == 0 && byte(p, 0xc) == 3 {
            crate::comparacao::ramo("chute tres gols");
            na(chamar, if byte(p, 0x30) != 0 { 0x2011 } else { 0x1fd5 }, ub(p, 0x2e), lp, 0);
        }
        if modo == 5 || modo == 6 {
            repetir_chute_com(c, true, c.l17 != 0, chamar);
            return;
        }
        if anular(c, chamar) {
            return;
        }
        if c.l11 == 0 && c.l1b >= 0x14 {
            let codigo = if c.l1b > 0x28 { 0x1fa1 } else if c.l1b > 0x19 { 0x1fa0 } else { 0x1f9f };
            na(chamar, codigo, ub(p, 0x2e), lp, c.l64 as i8);
        }
        if anular(c, chamar) {
            return;
        }
        if c.l11 == 0 {
            // as marcas do artilheiro
            if byte(p, 0x30) != 0 && byte(p, 0xc) == 1 {
                let codigo = if destaque_do_jogador(p, 0x21b6) != 0 { 0x21b6 } else { 0x2010 };
                na(chamar, codigo, ub(p, 0x2e), lp, w(p, 0x3d) as i8);
            } else {
                let (g3d, g3f, g41, g43, g45) = (w(p, 0x3d), w(p, 0x3f), w(p, 0x41), w(p, 0x43), w(p, 0x45));
                let redondo = |v: i16| v == 0x32 || (v >= 100 && v % 100 == 0);
                let marca = if g3d == 1 {
                    Some((0x200f, 1))
                } else if g3f == 1 {
                    Some((0x200b, 1))
                } else if g43 == 1 {
                    Some((0x2012, 1))
                } else if g3f >= 10 && g3f % 10 == 0 {
                    Some((0x200b, g3f))
                } else if redondo(g3d) {
                    Some((0x200a, g3d))
                } else if g41 != g3d && redondo(g41) {
                    Some((0x200c, g41))
                } else if g43 != g41 && g43 >= 100 && g43 % 100 == 0 {
                    Some((0x200d, g43))
                } else if g45 == 1 || (g45 >= 5 && g45 % 5 == 0) {
                    Some((0x200e, g45))
                } else {
                    None
                };
                if let Some((codigo, v)) = marca {
                    crate::comparacao::ramo("chute marca do artilheiro");
                    na(chamar, codigo, ub(p, 0x2e), lp, v as i8);
                }
            }
            destaques(chamar, m, p, lp, &DESTAQUES_GOL);
        }
        if anular(c, chamar) {
            return;
        }
        if byte(m, 0x1a4) == 0 && tg() == 1 && c.l20 != 0 {
            na(chamar, 0x1fe1, ub(g, 0x2e), ub(g, 0x27), c.l20 as i8);
        }
        if anular(c, chamar) {
            return;
        }
        // o gol vale (0x70847e): a virada e os momentos
        c.l12 = c.l12.wrapping_add(1);
        let tarde = (0x55..0x5a).contains(&(minuto as i16)) || minuto as i16 >= 0x73;
        let campeonato = w(m, 0x8ed4) == 0x3de;
        let dif = c.l12 as i32 - c.l13 as i32;
        let ao_lado = |fator: u32| aplicar_ao_lado(m, (lado == 0) as i8, f32::from_bits(fator));
        if byte(m, 0x1a8) != 0 {
            nb(chamar, 0x1fd2, 0);
            break 'final_;
        }
        if c.l4f == 0 {
            if dif == 2 && ((minuto >= 0x46 && campeonato) || minuto >= 0x6a) {
                nb(chamar, 0x1fd0, 0);
                ao_lado(0x3fc66666);
                break 'final_;
            }
            if (dif == 1 || c.l26 != 0) && ((minuto >= 0x50 && campeonato) || minuto >= 0x6e) {
                nb(chamar, 0x1fd1, 0);
                ao_lado(0x3fb9999a);
                break 'final_;
            }
            if c.l12 == c.l13 && c.l26 == 0 && minuto >= 0x46 && c.l4e == 0 {
                crate::comparacao::ramo("chute empate no fim");
                let a0 = contar_ativos(m, (lado == 0) as i8) as i8;
                let a1 = contar_ativos(m, lado) as i8;
                let bl: i8 = if a1 < a0 { byte(m, 4 + lado as isize as usize) } else { -1 };
                let (codigo, fator) = match (bl == -1, tarde) {
                    (true, true) => (0x1fd8, 0x3fb9999a),
                    (true, false) => (0x1fd6, 0x3faccccd),
                    (false, true) => (0x1fd9, 0x3fd33333),
                    (false, false) => (0x1fd7, 0x3fb33333),
                };
                ao_lado(fator);
                nb(chamar, codigo, bl);
                break 'final_;
            }
            if dif < 0 && dif > -3 && (minuto >= 0x50 || (minuto >= 0x46 && c.l12 == 1) || (minuto >= 0x3c && c.l12 == 1 && 1 - c.l13 as i32 == -2)) {
                nb(chamar, if tarde { 0x1fdb } else { 0x1fda }, 0);
                break 'final_;
            }
        }
        if c.l26 != 0 && minuto < 0x50 {
            nb(chamar, 0x1fdc, 0);
            ao_lado(0x3fb9999a);
        }
    }
    // 0x708844: o lance marcado ainda anula
    if lado_do_lance(m) as i8 > -1 {
        gol_anulado_com(c, chamar);
        return;
    }
    if c.l14 != 0 && c.l20 == 0 && tg() == 1 {
        na(chamar, 0x20d5, ub(g, 0x2e), ub(g, 0x27), 0);
    }
    if modo == 1 {
        crate::comparacao::ramo("chute gol valido");
        aplicar_ao_time(m, p, f32::from_bits(0x3ca3d70a));
        somar_disposicao(m, lado, 0x96);
        somar_disposicao(m, (lado == 0) as i8, -0x87);
        let k = m + 0xf5bd + (lado as isize * 0x36) as usize;
        *b(k, 0) = (*b(k, 0)).wrapping_add(1);
        // a narracao do placar (0x6d4b70 fica na original)
        let consulta: unsafe extern "thiscall" fn(usize, u32, u32, *mut u32) -> u32 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_CONTEXT_COMMENTARY_ALLOWED);
        let zero = (lado == 0) as u32;
        let mut l3c = 0u32;
        for (codigo, lado_c, com) in [
            (0x21a3, lp, false),
            (0x21a4, lp, false),
            (0x21a5, lp, false),
            (0x21a6, zero, false),
            (0x21a7, zero, false),
            (0x21a8, zero, false),
            (0x21a9, zero, false),
            (0x21aa, lp, false),
            (0x21ab, lp, true),
            (0x21ac, zero, true),
            (0x21ad, lp, true),
            (0x21ae, zero, true),
            (0x21af, zero, true),
        ] {
            if consulta(m, lado_c, codigo, if com { &mut l3c } else { core::ptr::null_mut() }) != 0 {
                narrar_finalizacao(chamar, m, codigo, 0xff, lado_c, 0xff, 0xff, if com { l3c as i8 } else { 0 });
                break;
            }
        }
        let clube_8 = || {
            let t = u(m, 0x4792);
            let clube = u(t, if lado != 0 { 0x20 } else { 0x1c });
            let k = u(clube, 0x53);
            k != 0 && byte(k, 0x77) == 8
        };
        let r = sortear(5);
        let mut feito = false;
        if c.l12 as i32 - c.l13 as i32 > r + 2 && sortear(2) == 0 && clube_8() {
            nb(chamar, 0x2117, 0);
            feito = true;
        }
        if !feito && (c.l12 == c.l13 || c.l12 as i32 - 1 == c.l13 as i32 || c.l26 != 0) {
            let x = sortear(minuto);
            if x > sortear(0x32) && clube_8() {
                nb(chamar, 0x2116, 0);
            }
        }
    }
    *b(m, 0x8eae) = (lado == 0) as u8;
    *((m + 0xf582) as *mut u32) = 0;
    *b(m, 0x8eb2) = 8;
    *b(m, 0x1ca) = 0;
}

/// O chute que nao foi no alvo (0x7040cb a 0x706cb7): bloqueado (escanteio), na trave (a bola
/// espirra) ou para fora (tiro de meta); as narracoes pela classe e pelo momento.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn chute_fora_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| *((q + o) as *const i16);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let modo = c.modo;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, ub(p, 0x2e), lp, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    if c.l11 == 2 {
        // o chute bateu em alguem e saiu (0x7040d6): escanteio
        crate::comparacao::ramo("chute bloqueado para escanteio");
        if c.l28 == 5 {
            na(chamar, 0x1f83, 5);
        } else {
            na(chamar, 0x1f82, c.l28);
        }
        if modo == 4 || modo == 3 {
            na(chamar, 0x2008, 0);
        } else if c.tipo == 0x33 && modo == 1 {
            na(chamar, 0x1fce, 0);
        }
        if modo == 5 || modo == 6 {
            match lado {
                1 => *b(m, 0x8ea8) = 0xff,
                0 => *b(m, 0x8ea8) = 0xc,
                _ => {}
            }
            repetir_chute_com(c, false, false, chamar);
            return;
        }
        escanteio_do_chute(c, chamar);
        return;
    }
    if modo == 4 || modo == 3 {
        na(chamar, 0x2008, 0);
        if modo == 4 {
            na(chamar, 0x2174, 0);
        }
    } else if c.tipo == 0x33 {
        na(chamar, 0x1fce, 0);
    }
    let goleiro = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if c.tipo != 0x33 && c.tipo != 0x3a && c.l14 == 0 && sortear(0x32) == 0 && c.l20 == 0 && tg() == 1 {
            narrar_finalizacao(chamar, m, if c.tipo == 0x1a { 0x21d2 } else { 0x20d6 }, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
        }
    };
    if c.tipo == 0x33 {
        match c.l10 {
            2 | 5 | 8 | 3 | 4 | 6 | 7 | 9 => {
                na(chamar, if matches!(c.l10, 2 | 5 | 8) { 0x2126 } else { 0x2125 }, c.l28);
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            0xa | 0xc => na(chamar, 0x2123, c.l28),
            0xb | 0xd => na(chamar, 0x2124, c.l28),
            _ => {}
        }
    } else {
        match c.l10 {
            2 => {
                na(chamar, 0x1fbf, 0);
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            3 => {
                if c.l28 == 5 {
                    na(chamar, 0x1fc1, 5);
                } else {
                    na(chamar, 0x1fc0, c.l28);
                }
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            4 => {
                na(chamar, 0x1fc2, 0);
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            5 => na(chamar, 0x1fc3, 0),
            6 => na(chamar, 0x1fc4, 0),
            7 => na(chamar, 0x1fc5, 0),
            8..=0xd => {
                goleiro(chamar);
                na(chamar, 0x1fc6 + (c.l10 as u32 - 8), 0);
            }
            _ => {}
        }
    }
    if modo != 1 && modo != 2 {
        if modo == 5 || modo == 6 {
            repetir_chute_com(c, false, false, chamar);
        }
        return;
    }
    match c.l10 {
        0xc | 0xd => {
            // na trave (0x704b27): a bola espirra
            crate::comparacao::ramo("chute na trave");
            if c.l18 {
                if c.l11 == 0 {
                    na(chamar, 0x2020, 0);
                }
            } else if c.l11 == 0 && c.l1b >= 0x19 && w(p, 0x39) < 0x32 {
                na(chamar, 0x1f9e, c.l64 as i8);
            }
            if c.l14 != 0 && c.l20 == 0 && tg() == 1 {
                narrar_finalizacao(chamar, m, 0x20d5, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
            }
            let l = if c.l11 != 0 { (lado == 0) as u32 } else { lp };
            chamar(0x6cd770, m, [ub(m, 0x8ea7), ub(m, 0x8ea8), 2 | l << 8, 1 | ((c.tipo == 0x33) as u32) << 8, 0]);
            if tg() == 1 {
                *b(g, 0x2b) = (*b(g, 0x2b)).wrapping_add(sortear(3) as u8);
            }
            *b(m, 0x1cd) = 1;
        }
        2..=4 => {
            crate::comparacao::ramo("chute fraco para fora");
            if c.tipo != 0x33 {
                fora_fraco_com(c, chamar);
            }
            for codigo in [0x21a1, 0x21a2] {
                if destaque_do_jogador(p, codigo) != 0 {
                    na(chamar, codigo, 0);
                }
            }
            tiro_de_meta(c, chamar);
        }
        _ => {
            crate::comparacao::ramo("chute para fora");
            if c.tipo != 0x33 {
                fora_com(c, chamar);
            }
            tiro_de_meta(c, chamar);
        }
    }
}

/// A bola saiu pela linha de fundo (0x705d15 e 0x706c5d): a linha do outro lado e a bola fora.
pub(crate) unsafe fn tiro_de_meta(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = c.m;
    match c.lado {
        1 => *((m + 0x8ea8) as *mut u8) = 0xff,
        0 => *((m + 0x8ea8) as *mut u8) = 0xc,
        _ => {}
    }
    chamar(0x6cd490, m, [c.lado as u8 as u32, 0, 0, 0, 0]);
}

/// As narracoes do chute fraco para fora (classes 2 a 4, 0x704c6d a 0x705cb2).
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn fora_fraco_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, lado) = (c.p, c.m, c.lado);
    let lp = lado as u8 as u32;
    let zero = (lado == 0) as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, ub(p, 0x2e), lp, 0xff, 0xff, a7);
    let n_21c6 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, r: usize| narrar_finalizacao(chamar, m, 0x21c6, ub(p, 0x2e), lp, ub(r, 0x2e), ub(r, 0x27), 0);
    let reiniciar = || core::ptr::write_unaligned((c.lista + 0xa) as *mut u32, core::ptr::read_unaligned((c.lista + 2) as *const u32));
    if c.l18 && c.l11 == 0 {
        let q = companheiro_livre(m, p);
        if q != 0 && byte(u(p, 0x6d), 0x3f) < 10 && {
            let r = sortear(6);
            byte(u(p, 0x69), 0x5b) < (r as u8).wrapping_add(10) as i8
        } && byte(u(q, 0x69), 0x5b) < 10
            && sortear(byte(u(q, 0x69), 0x5b) as i32) == 0
        {
            crate::comparacao::ramo("chute companheiro livre");
            narrar_finalizacao(chamar, m, 0x21dd, ub(p, 0x2e), lp, ub(q, 0x2e), ub(q, 0x27), 0);
        } else {
            na(chamar, 0x201e, 0);
        }
        return;
    }
    if (*((m + 0x4782) as *const i32) as f64) > f64_em(0x96d810) {
        if c.l40 >= -0x87 {
            if c.tipo == 0x3a && c.conta == 0 {
                na(chamar, 0x219a, 0);
            }
        } else if c.tipo != 0x3a {
            let n = c.conta as i32;
            if sortear(n * n + 4) != 0 {
                if n == 1 {
                    reiniciar();
                    let r1 = proximo_da_lista(c.lista) as usize;
                    if c.l25 != 0 {
                        if assistencia_ao_marcador(c, r1) {
                            n_21c6(chamar, r1);
                        }
                    } else {
                        narrar_finalizacao(chamar, m, 0x2154, ub(p, 0x2e), lp, ub(r1, 0x2e), zero, 0);
                    }
                    destaques(chamar, m, r1, zero, &DESTAQUES_R);
                } else if n == 2 {
                    if sortear(2) == 0 {
                        na(chamar, 0x2132, 0);
                    } else {
                        reiniciar();
                        let r1 = proximo_da_lista(c.lista) as usize;
                        let r2 = proximo_da_lista(c.lista) as usize;
                        if c.l25 != 0 {
                            if r2 != 0 && assistencia_ao_marcador(c, r2) {
                                n_21c6(chamar, r2);
                            }
                        } else {
                            narrar_finalizacao(chamar, m, 0x2155, ub(r1, 0x2e), zero, ub(r2, 0x2e), zero, 0);
                        }
                        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) {
                            destaques(chamar, m, r2, zero, &DESTAQUES_R);
                        }
                    }
                } else {
                    na(chamar, 0x2132, 0);
                }
            }
        }
        na(chamar, 0x201f, 0);
        return;
    }
    let x39 = *((p + 0x39) as *const i16);
    if x39 > 0x15e {
        if byte(p, 0xc) == 2 && (*((m + 0x8ed0) as *const i16) as i32 / 11) > 0x32 {
            na(chamar, 0x1fdf, 2);
        } else {
            na(chamar, 0x1fde, byte(p, 0xc));
            destaques(chamar, m, p, lp, &DESTAQUES_P);
        }
        return;
    }
    if x39 > 0xfa {
        na(chamar, 0x1fde, byte(p, 0xc));
        if !destaques(chamar, m, p, lp, &DESTAQUES_P) && c.l25 != 0 {
            reiniciar();
            let r1 = proximo_da_lista(c.lista) as usize;
            if assistencia_ao_marcador(c, r1) {
                n_21c6(chamar, r1);
            }
        }
        return;
    }
    if c.l40 >= -0x87 || c.tipo == 0x3a {
        return;
    }
    let n = c.conta as i32;
    if sortear(n * n + 4) == 0 {
        return;
    }
    if n == 1 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2154, ub(p, 0x2e), lp, ub(r1, 0x2e), zero, 0);
        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) && c.l25 != 0 && assistencia_ao_marcador(c, r1) {
            n_21c6(chamar, r1);
        }
    } else if n == 2 && sortear(2) != 0 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        let r2 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2155, ub(r1, 0x2e), zero, ub(r2, 0x2e), zero, 0);
        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) {
            destaques(chamar, m, r2, zero, &DESTAQUES_R);
        }
    } else {
        na(chamar, 0x2132, 0);
    }
}

/// As narracoes do chute para fora (classes 5 a 0xb, 0x705d52 a 0x706c5c).
pub(crate) unsafe fn fora_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, ub(p, 0x2e), lp, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let companheiros = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if c.l40 < -0x87 && c.tipo != 0x3a {
            narrar_companheiros_com(c, 4, chamar);
        }
    };
    if c.l18 && c.l11 == 0 {
        na(chamar, 0x201e, 0);
        companheiros(chamar);
        return;
    }
    if (*((m + 0x4782) as *const i32) as f64) > f64_em(0x96d810) {
        companheiros(chamar);
        na(chamar, 0x201f, 0);
        return;
    }
    let x39 = *((p + 0x39) as *const i16);
    let k3a = c.tipo == 0x3a;
    if (x39 > 0x15e && !k3a) || (x39 > 0xaf && k3a) {
        na(chamar, if byte(p, 0xc) != 2 { 0x1fdd } else { 0x1fdf }, byte(p, 0xc));
        destaques(chamar, m, p, lp, &DESTAQUES_P);
        return;
    }
    if x39 > 0xfa || k3a {
        na(chamar, 0x1fdd, byte(p, 0xc));
        destaques(chamar, m, p, lp, &DESTAQUES_P);
        return;
    }
    if c.l14 != 0 && c.l20 == 0 && tg() == 1 && !matches!(c.l10, 5 | 6 | 7) {
        narrar_finalizacao(chamar, m, 0x20d5, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
    }
    companheiros(chamar);
}

/// Modo Rust: as chamadas do chute vao para o jogo (a interceptacao com x | y << 8; a bola desviada
/// com dentro | sem cansar << 8; o chute de novo sem a lista).
pub(crate) unsafe fn despachar_finalizacao(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x6cc540 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, i8, u32, u32) -> u8 = core::mem::transmute(rotina);
            f(this, a[0] as u8 as i8, (a[0] >> 8) as u8 as i8, a[1], a[2] as u8 as i8, a[3], a[4]) as u32
        }
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(rotina);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, ((a[3] >> 8) as u8 == 0) as u8, a[4] as usize);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(rotina);
            f(this, a[0], a[1], a[2], a[3], 0, 0);
            0
        }
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
            f(this, a[0], a[1]);
            0
        }
        _ => despachar_movimento(rotina, this, a),
    }
}

/// `p`'s shot (type, value, commentary, mode 1 match / 2 no challenger / 3 shoot-out / 4 set-piece kick / 5-6 repeat,
/// the pressure teammates list). Challenger (0x6c87e0) with foul or dispossession; commentary; offside; interception
/// (0x6cc540); the outcome class; goal (can be disallowed), save, post or wide, with the commentary. Compared through
/// the tape, with comparison inside the unit
/// Original: `shoot` (0x702050, match_pl.cpp (35 KB), thiscall, ret 0x18).
///
/// [pt] { 0x702050 } (ret 0x18, match_pl.cpp): ver `chutar_com`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn chutar(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    chutar_com(p, a1, a2, a3, a4, a5, a6 as usize, &mut |r, t, a| despachar_finalizacao(r, t, a))
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_chutar(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    if gravando() {
        // unidade da fita de fora (conducao, penaltis, o proprio chute de novo); por dentro, a propria comparacao
        chamar_gravando(0x702050, p, [a1, a2, a3, a4, a6], || {
            fita_do_chute(p, a1, a2, a3, a4, a5, a6);
            0
        });
        return;
    }
    fita_do_chute(p, a1, a2, a3, a4, a5, a6)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn fita_do_chute(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32)>(324);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    if a6 != 0 {
        // a lista dos companheiros: o cursor (+0xa) anda
        reg.push((a6 as usize, 14));
    }
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(
        reg,
        || {
            o(p, a1, a2, a3, a4, a5, a6);
            0
        },
        |chamar| {
            chutar_com(p, a1, a2, a3, a4, a5, a6 as usize, &mut |rotina, this, args| match rotina {
                // a interceptacao roda por dentro (o comparador dela nao grava unidade): as jogadas sao as unidades
                0x6cc540 => {
                    let mut jogada = |q: usize, a: [u32; 4]| chamar(0x70dfd0, q, [a[0], a[1], a[2], a[3], 0], [u32::MAX, u32::MAX, 0xff, u32::MAX, 0]) as u8;
                    interceptar_com(this, args[0] as u8 as i8, (args[0] >> 8) as u8 as i8, args[1], args[2] as u8 as i8, args[3], args[4], &mut |lado, grupo, v, a3, a5, a6| {
                        procurar_jogada_com(this, lado, grupo, v, a3, 1, a5, a6, &mut jogada)
                    }) as u32
                }
                _ => {
                    let mascara = match rotina {
                        0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                        0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                        0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                        0x6f46b0 => [0xff, 0xff, 0, 0, 0],
                        0x6cd490 => [0xff, 0, 0, 0, 0],
                        0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                        0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                        0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                        0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                        0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 | 0x7103e0 => [0, 0, 0, 0, 0],
                        _ => [0xff, 0, 0, 0, 0],
                    };
                    chamar(rotina, this, args, mascara)
                }
            });
            0
        },
    );
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("chutar(tipo {:#x}, modo {}) {} chamadas: {}", a1 as u8, a4 as u8, k, detalhe_fita()));
}

// ---------- centesima quadragesima oitava leva: o motor nasce, prepara a partida e morre (0x6b7e50, 0x6b7f20, 0x6b97a0) ----------

/// the engine constructor (the player, tactics and tactical line vectors through the runtime vector constructor
/// 0x9463af; the referee strictness 0x6e9460; the empty list)
/// Original: `build_engine` (0x6b7e50, match_eng.cpp, thiscall).
///
/// [pt] { 0x6b7e50 } (match_eng.cpp): o construtor do motor. Os vetores de objetos pelo construtor de
/// vetores do runtime (0x9463af): os 40 jogadores (0x1be bytes, 0x6eada0), as duas taticas (0xd0,
/// 0x5961c0) e as duas copias das linhas taticas (0x18e3, 0x5b27c0); o rigor do arbitro (0x6e9460) e
/// a lista de eventos vazia (+0x478e).
pub(crate) unsafe extern "thiscall" fn construir_motor(m: usize) -> usize {
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let rigor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_OFFICIAL_MATCH_OFFICIAL);
    vetor(m + 0x4796, 0x1be, 0x28, 0x6eada0, 0x6ebd60);
    vetor(m + 0x9032, 0xd0, 2, 0x5961c0, 0x6dbb10);
    vetor(m + 0x91e2, 0x18e3, 2, 0x5b27c0, 0x5a53c0);
    vetor(m + 0xc3a8, 0x18e3, 2, 0x5b27c0, 0x5a53c0);
    rigor(m + 0xf5b2);
    *((m + 0x478e) as *mut usize) = 0;
    m
}

/// the destructor (its own commentator, the list, the strictness and the vectors)
/// Original: `destroy_engine` (0x6b97a0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6b97a0 } (match_eng.cpp): o destrutor do motor. O narrador proprio (+0x8e9d) e a lista de
/// eventos (+0x478e) sao destruidos e liberados; depois o rigor e os vetores, na ordem inversa.
pub(crate) unsafe extern "thiscall" fn destruir_motor(m: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let narr = *((m + 0x8f7a) as *const usize);
    if *((m + 0x8e9d) as *const u8) != 0 && narr != 0 {
        destruir(narr);
        liberar(narr);
        *((m + 0x8f7a) as *mut usize) = 0;
        *((m + 0x8e9d) as *mut u8) = 0;
    }
    let lista = *((m + 0x478e) as *const usize);
    if lista != 0 {
        destruir_lista(lista);
        liberar(lista);
        *((m + 0x478e) as *mut usize) = 0;
    }
    destruir(m + 0xf5b2);
    vetor(m + 0xc3a8, 0x18e3, 2, 0x5a53c0);
    vetor(m + 0x91e2, 0x18e3, 2, 0x5a53c0);
    vetor(m + 0x9032, 0xd0, 2, 0x6dbb10);
    vetor(m + 0x4796, 0x1be, 0x28, 0x6ebd60);
}

/// O angulo (graus, 0..=359) da casa (i, j) vista do centro, para a tabela global 0xb67c30 que a
/// bola desviada usa (+0x8ed8); a original calcula com fpatan, o Rust com atan.
pub(crate) unsafe fn angulo_da_casa(i: i32, j: i32) -> u16 {
    let graus = f64_em(0x96d2b8);
    let mut a: i32 = if i == 0 {
        if j > 0 {
            0
        } else if j < 0 {
            0xb4
        } else {
            return 0;
        }
    } else if i > 0 && j == 0 {
        0x5a
    } else if i < 0 && j == 0 {
        0x10e
    } else if i > 0 {
        if j > 0 {
            ftol((i as f64 / j as f64).abs().atan() * graus)
        } else {
            ftol((j as f64 / i as f64).abs().atan() * graus + f64_em(0x96d2b0))
        }
    } else if j < 0 {
        ftol((i as f64 / j as f64).abs().atan() * graus + f64_em(0x96d2a8))
    } else {
        ftol((j as f64 / i as f64).abs().atan() * graus + f64_em(0x96d2a0))
    };
    if a as i16 > 0x167 {
        a = 0;
    } else if (a as i16) < 0 {
        a += 0x168;
    }
    a as u16
}

/// Monta e poe na lista de eventos (+0x478e, 0x68d0d0) o evento de pre-jogo de 0x19 bytes: a ficha,
/// o clube, as bandeiras, o titular, o lado (ou a vaga), o codigo e -1 nos campos livres.
pub(crate) unsafe fn evento_de_prejogo(m: usize, ficha: usize, clube: usize, bandeiras: u32, titular: u8, lado: u8, codigo: u16) {
    let mut ev = [0u8; 0x19];
    ev[0..4].copy_from_slice(&*(ficha as *const [u8; 4]));
    ev[4..8].copy_from_slice(&*(clube as *const [u8; 4]));
    ev[8..0xc].copy_from_slice(&bandeiras.to_le_bytes());
    ev[0xc] = titular;
    ev[0xd] = lado;
    ev[0xe] = 0;
    ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
    ev[0x13..0x15].copy_from_slice(&codigo.to_le_bytes());
    ev[0x15..0x19].copy_from_slice(&u32::MAX.to_le_bytes());
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    copiar(*((m + 0x478e) as *const usize), ev.as_ptr() as usize, 0x19);
}

/// { 0x6b7f20 } (ret 0x10, match_eng.cpp, 6 KB): prepara a partida `jogo` no motor. Zera o estado,
/// o clima (+0x4756: 0x923010, 0x923970, 0x923630 na original), os uniformes (0x6d47e0), o arbitro
/// (0x6e9470) e as condicoes do gramado (0x85cba0, +0x91d2), os dois elencos (0x6db500, com a caixa de
/// erro abaixo de 11), o narrador (dado ou novo, 0x6d5bf0), a forca dos lados e a marcacao; a lista
/// de eventos de pre-jogo (0x68d020) recebe os jogadores insatisfeitos com o contrato (0x13), o
/// interesse de outro clube (0x1e), a forma (5, 4) e o fim de contrato (6). Sem `rapida`, sorteia
/// quem comeca (+0x8eb6/+0x8eae) e a bola parada 8. Devolve 0 se a partida acabou antes (+0x8eba).
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn preparar_partida_com(m: usize, jogo: usize, rapida: u8, narrador: usize, ao_vivo: u8) -> u8 {
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut u16;
    let dw = |o: usize| (m + o) as *mut u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let zerar = |de: usize, ate: usize| core::ptr::write_bytes((m + de) as *mut u8, 0, ate - de);
    *((m + 0x4792) as *mut usize) = jogo;
    *b(0x8eb3) = 0;
    *b(0x8eb4) = 0;
    zerar(0x21e, 0x21e + 8000);
    *b(0x8ea5) = 0;
    *b(0x1a3) = 0;
    *b(0x1cc) = 0;
    *b(0x1cb) = 0;
    *w(0x1d6) = *((u(jogo, 0x1c) + 0x80) as *const u16);
    *w(0x1d8) = *((u(jogo, 0x20) + 0x80) as *const u16);
    *b(0xaf) = 0;
    *b(0xb0) = 0;
    *dw(0xf596) = 0;
    *dw(0xf59a) = 0;
    *w(0x8d4c) = 0xffff;
    *b(0x8d4e) = 0;
    *w(0x8d4a) = 0xffff;
    let tempo = (jogo + 0x4d) as *mut u16;
    if *((jogo + 0x4d) as *const u8) & 0xf > 9 {
        *tempo = (*tempo & 0xfff9) | 9;
    }
    if (*((jogo + 0x4d) as *const u8) & 0xf0) as u32 > 0x90 {
        *tempo = (*tempo & 0xff9f) | 0x90;
    }
    let clima = *(0xde2f74 as *const usize);
    let preparar_clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_WEATHER_GET_MATCH_WEATHER);
    let ceu: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_GET_PITCH_CONDITION);
    let vento: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_CHECK_POSTPONEMENT);
    preparar_clima(clima, m + 0x4756, jogo);
    *b(1) = ceu(clima, m + 0x4756, jogo);
    let v = vento(clima, jogo);
    *tempo = ((((v & 3) as i8 as i16) << 12) as u16) | (*tempo & 0xcfff);
    if *tempo & 0x2000 == 0 {
        *tempo &= 0xcfff;
        *b(0xf638) = 0xff;
    } else {
        *b(0xf638) = if sortear(2) != 0 { (sortear(0x19) as u8).wrapping_add(0xf) } else { (sortear(0x1e) as u8).wrapping_add(0x32) };
        if matches!(*b(1), 0 | 2 | 4) {
            *tempo &= 0xcfff;
            *b(0xf638) = 0xff;
        }
    }
    *dw(0xf56e) = (m + 0x97cd) as u32;
    *dw(0xf572) = (m + 0xb9f8) as u32;
    *dw(0x8ebc) = (m + 0x930f) as u32;
    *dw(0x8ec0) = (m + 0xabf2) as u32;
    *dw(0x8ec4) = (m + 0x9330) as u32;
    *dw(0x8ec8) = (m + 0xac13) as u32;
    if *(0xa15854 as *const u8) != 0 {
        for i in -8..9i32 {
            for j in -11..12i32 {
                *((0xb67c30 + ((i * 24 + j) * 2) as isize as usize) as *mut u16) = angulo_da_casa(i, j);
            }
        }
        *(0xa15854 as *mut u8) = 0;
    }
    *w(0xf639) = 0;
    *b(0x8eba) = 0;
    *w(0x8ed0) = 0;
    *b(0x8eb7) = 0;
    *b(0x8eb5) = 0;
    *w(0x8ed4) = 0x1ef;
    *w(0x8ed2) = 0;
    *b(0x1a8) = 0;
    *b(0x1a9) = 0;
    *b(0x1aa) = 0;
    *w(0x8ecc) = 0;
    *w(0x8ece) = 0;
    *b(4) = 0xb;
    *b(5) = 0xb;
    *w(0x1d2) = 0x2710;
    *w(0x1d4) = 0x2710;
    *dw(0x477a) = u32::MAX;
    for o in [0x1a4, 0x1cd, 8, 9, 6, 7, 0x1a5, 0x1a6] {
        *b(o) = 0;
    }
    *dw(0x4786) = 0;
    *dw(0x4782) = 0;
    *b(0x1ab) = 0xff;
    *b(0x1ac) = 0xff;
    *dw(0x476a) = 0xc47a0000;
    *dw(0x476e) = 0xc47a0000;
    *b(0x1c5) = 0;
    *b(0x1c6) = 0;
    *b(0x1c7) = 0;
    *dw(0x477e) = 0;
    *b(0x8eb1) = 0;
    *b(0x1a7) = 0;
    *b(0x8d48) = 0;
    *b(0x8d49) = 0;
    *w(0x475f) = 0xffff;
    *dw(0x4761) = 0;
    *dw(0x4765) = 0;
    *b(0x4769) = 0;
    *b(0x475a) = 0;
    for o in 0x475b..=0x475e {
        *b(o) = 0xff;
    }
    *dw(0x478a) = 0;
    *b(2) = 0xff;
    *b(3) = 0xff;
    zerar(0x8f02, 0x8f7a);
    zerar(0x1f6, 0x21e);
    zerar(0xa, 0xaf);
    zerar(0x8d51, 0x8e9b);
    *b(0x8e9b) = 0;
    *b(0x8eab) = 0xff;
    *w(0x8ed8) = 0xb4;
    *b(0x8eaa) = 0;
    *dw(0xf582) = 0;
    *b(0x8ea9) = 0;
    for o in [0xf576, 0xf57a, 0xf57e, 0xf5ae, 0x91de, 0xf586, 0xf58a, 0xf5a6, 0xf5aa, 0xf58e, 0xf592] {
        *dw(o) = 0;
    }
    zerar(0x215e, 0x215e + 0x97e * 4);
    *dw(0xf59e) = 0;
    *dw(0xf5a2) = 0;
    for l in 0..2usize {
        let e = 0xf5bd + 0x36 * l;
        zerar(e - 1, e + 0x10);
        zerar(e + 0x11, e + 0x35);
        *b(0x8d46 + l) = 0;
    }
    *dw(0xf628) = 0x40c00000;
    *dw(0xf62c) = 0x40c00000;
    *dw(0xf630) = 0x40a00000;
    *b(0) = rapida;
    if *(0xa5a390 as *const u8) == 0 {
        *b(0) = 1;
        *b(0x8e9d) = 0;
    }
    *b(0x8ebb) = ao_vivo;
    let (casa, fora) = (u(jogo, 0x1c), u(jogo, 0x20));
    *b(0x8d4f) = uniforme(casa, fora, 1);
    *b(0x8d50) = uniforme(fora, casa, 0);
    let x2e = *((jogo + 0x2e) as *const i16);
    let arbitro = if *(0xb74340 as *const u32) != 0 && *(jogo as *const i32) != -2 && x2e > -1 {
        (*(0xae23e8 as *const u32)).wrapping_add((x2e as i32 * 0x2b) as u32) as usize
    } else {
        0
    };
    iniciar_rigor(m + 0xf5b2, m as u32, arbitro);
    let comp = u(jogo, 0x14);
    if *(jogo as *const i32) != -2 && *(comp as *const u32) != *(KEY_CLUB_COMP_RESERVE as *const u32) {
        let gramado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_CALCULATE_MATCH_ATTENDANCE);
        *dw(0xf634) = gramado(*(0xdd7ef4 as *const usize), jogo, m + 0x91d2);
    } else {
        *dw(0xf634) = 0;
        *dw(0x91d2) = 0;
        *dw(0x91d6) = 0;
        *dw(0x91da) = 0;
    }
    if *dw(0x91d2) == 0 {
        *dw(0x91d2) = 100;
        *dw(0x91d6) = 0x5a;
        *dw(0x91da) = 0xa;
    }
    if *(jogo as *const i32) == -2 {
        *w(0x1ad) = 0;
    } else {
        // o desgaste do gramado (+0x1ad) e o quanto ele pesa (+0x1ae)
        let (g2, g6, ga) = (*dw(0x91d2) as i32 as f64, *dw(0x91d6) as i32 as f64, *dw(0x91da) as i32 as f64);
        let x = g6 * f64_em(0x9688a8) / g2;
        let t = (f64_em(0x9688a8) - x) as f32;
        let a = (ga * f64_em(0x96a5c0) + f64_em(0x9688a0)) - x * f64_em(0x968a80);
        let a = if !(a > f64_em(0x9688b0)) && a < f64_em(0x9689a0) { f64_em(0x9689a0) } else if a > f64_em(0x9688b0) { f64_em(0x9688b0) } else { a };
        *b(0x1ad) = ftol(a) as u8;
        let estadio = u(casa, 0x69);
        let base = if estadio == u(jogo, 0x18) || estadio == u(fora, 0x69) { f32_em(0x968e70) } else { f32_em(0x968970) } as f64;
        let c = (g6 * f64_em(0x96a5c0) + f64_em(0x9688a0)) - t as f64 * f64_em(0x968a80);
        let cf = c as f32 as f64;
        let menor = if c > f64_em(0x9688b0) { f64_em(0x9688b0) } else { cf };
        let r = if base > menor { base } else if cf > f64_em(0x9688b0) { f64_em(0x9688b0) } else { cf };
        *b(0x1ae) = ftol(r) as u8;
    }
    // os elencos
    let escalar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize, usize, usize, *mut i8) -> i8 = core::mem::transmute(FN_MATCH_MAN_INIT);
    let (mut n_casa, mut n_fora) = (0i8, 0i8);
    let pouco = |n: i8, linha: u32, texto: usize| {
        let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(FN_DATE_DATE);
        let data_texto: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32, u32) = core::mem::transmute(FN_DATE_GET_SHORT_WEEKDAY_DATE_STRING);
        let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
        let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
        let mut data = [0u8; 8];
        let mut quando = [0u8; 200];
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        let j = *((m + 0x4792) as *const usize);
        data_nova(data.as_mut_ptr(), *((j + 0x2a) as *const u16) as u32, *((j + 0x28) as *const u16) as u32);
        data_texto(data.as_mut_ptr(), quando.as_mut_ptr(), 200, 0xff);
        let id = *(j as *const i32);
        let comp = if id == -2 { 0xa158dc } else if id == -1 { 0xa158ec } else { u(j, 0x14) + 0x38 };
        formatar(0xde1f64, texto, n as i32, u(j, 0x1c) + 0x38, u(j, 0x20) + 0x38, comp, quando.as_ptr());
        separar(0xa15884, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
        formatar(0xae24d0, 0x9870e8, s);
        caixa(0x9870e0, 0xae24d0, 0);
        *(0xb67a34 as *mut u32) = 0;
    };
    let nc = escalar(m + 0x9032, m, u(casa, 0xcf), casa + 0xd7, m + 0x4796, 0, casa, fora, jogo, &mut n_casa);
    if nc < 0xb {
        pouco(nc, 0x264, 0xa158b0);
    }
    let nf = escalar(m + 0x9102, m, u(fora, 0xcf), fora + 0xd7, m + 0x6a6e, 1, fora, casa, jogo, &mut n_fora);
    if nf < 0xb {
        pouco(nf, 0x28c, 0xa15858);
    }
    if (n_fora as i32 + n_casa as i32) < 5 && ao_vivo == 0 {
        *b(0) = 1;
    }
    core::ptr::copy_nonoverlapping((m + 0x91e2) as *const u8, (m + 0xc3a8) as *mut u8, 0xc71 * 4 + 2);
    *w(0x1ce) = 0x6e;
    *w(0x1d0) = 0x6e;
    *dw(0x8f7a) = 0;
    if *b(0x8eba) != 0 {
        *b(0) = 1;
        *b(0x8e9d) = 0;
        return 0;
    }
    // o narrador
    if narrador != 0 {
        *dw(0x8f7a) = narrador as u32;
        *b(0x8e9d) = 0;
    } else {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let narrador_novo: unsafe extern "thiscall" fn(usize, u8) -> usize = core::mem::transmute(FN_MATCH_EVENTS_MATCH_EVENTS);
        let p = novo(0x2db1);
        *dw(0x8f7a) = if p != 0 { narrador_novo(p, *b(0x8ebb)) as u32 } else { 0 };
        *b(0x8e9d) = 1;
        *(*dw(0x8f7a) as usize as *mut u8) = 1;
    }
    forca_do_lado(m, 0);
    forca_do_lado(m, 1);
    marcacao_individual(m);
    *b(0x8ea5) = 1;
    let vagas = *(0xde1d08 as *const usize);
    let titular = |l: usize, k: i8| m + (if l == 0 { 0x4796 } else { 0x6a6e }) + (k as isize * 0x1be) as usize;
    for (l, o_clube, o_marca, o_destino) in [(0usize, 0x1c, 0x40, 0xf5cd), (1, 0x20, 0x41, 0xf603)] {
        let k = vaga_marcada(vagas, u(jogo, o_clube), (*((jogo + o_marca) as *const u8) == 0) as u32, 0);
        *b(o_destino) = if titular(l, k) == 0 {
            0xff
        } else {
            let k = vaga_marcada(vagas, u(jogo, o_clube), (*((jogo + o_marca) as *const u8) == 0) as u32, 0);
            *(titular(l, k) as *const u8)
        };
    }
    // a lista de eventos de pre-jogo
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let p = novo(0xe);
    let lista = if p != 0 { lista_nova(p) } else { 0 };
    *dw(0x478e) = lista as u32;
    'eventos: {
        if lista == 0 || e_selecao(casa) != 0 {
            break 'eventos;
        }
        if *b(0) != 0 {
            return 1;
        }
        let comp = u(jogo, 0x14);
        if *((jogo + 0x40) as *const u8) == 0 || comp == 0 {
            break 'eventos;
        }
        let amistosos = *(0xae2c74 as *const usize);
        if amistosos != 0 && e_amistoso(amistosos, comp) != 0 {
            break 'eventos;
        }
        eventos_de_prejogo(m, jogo);
    }
    if *b(0) == 0 {
        if sortear(2) != 0 {
            *b(0x8eb6) = 1;
            *b(0x8eae) = 0;
        } else {
            *b(0x8eb6) = 0;
            *b(0x8eae) = 1;
        }
        *b(0x8eb2) = 8;
        *b(0x1ca) = 0;
    }
    1
}

/// Os eventos de pre-jogo de cada jogador em campo (0x6b8d9b a 0x6b9745): o contrato (0x4cf5f0) e as
/// marcas da posicao (0x5481a0 contra 0x717400/0x717450) dao a insatisfacao (0x13, com pontos pela
/// ficha); a reputacao dele contra a do clube, o interesse (0x1e); a forma pela media (0x7c1e30) e
/// pelo indice de [0xb74350], 5 ou 4; o fim de contrato perto (0x54cdb0), 6.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn eventos_de_prejogo(m: usize, jogo: usize) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let wq = |q: usize, o: usize| *((q + o) as *const i16);
    let contratos = *(0xae1050 as *const usize);
    let marcas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let vagas = *(0xde1d08 as *const usize);
    let indice = || {
        let t = *(0xb74350 as *const usize);
        let d = *((t + 0x18) as *const f64);
        if d == f64_em(0x9689a0) { f64_em(0x968998) } else { *((t + 0x10) as *const f64) / d }
    };
    for lado in 0..2i8 {
        let titular_da_vaga = |j: usize| {
            let (c, marca) = if lado == 0 { (u(jogo, 0x1c), *((jogo + 0x40) as *const u8)) } else { (u(jogo, 0x20), *((jogo + 0x41) as *const u8)) };
            let k = vaga_marcada(vagas, c, (marca == 0) as u32, 0);
            (j == m + 0x4796 + ((k as isize + lado as isize * 20) * 0x1be) as usize) as u8
        };
        for k in 0..20usize {
            let j = jogador_do_bloco(m, lado, k);
            if byte(j, 0x19) < 0 {
                continue;
            }
            let f = u(j, 0x69);
            let clube = if lado != 0 { u(jogo, 0x20) } else { u(jogo, 0x1c) };
            if e_selecao(clube) != 0 || u(clube, 0x53) == 0 {
                break;
            }
            if u(f, 0x61) == 0 {
                continue;
            }
            let c = contrato_com_clube(contratos, f, clube);
            if c == 0 {
                continue;
            }
            let a = marcas(f) as u16;
            let bb = ler_1bc(j);
            let d = tabela_b_do_jogador(j);
            let flags = bandeiras_do_contrato(c);
            let marcado = flags & 0x40 != 0 || *((c + 0x45) as *const u32) & 0x2000000 != 0;
            if marcado && a != 0 && ((d & a) == d || (bb & a) != bb) {
                crate::comparacao::ramo("prejogo insatisfeito");
                evento_de_prejogo(m, f, clube, 0, 0, k as u8, 0x13);
            } else if byte(f, 0x3d) != 0xc && a != 0 && (d & a) != d && (bb & a) != bb {
                let s: i16 = if a == 1 {
                    6
                } else if (d | 4) != 0 && byte(u(f, 0x61), 0x11) < 10 && byte(u(f, 0x61), 0x12) < 10 {
                    2
                } else if (d | 4) != 0 && (a | 4) != 0 {
                    -1
                } else if (d | 0x10) != 0 && (a | 0x10) != 0 {
                    0
                } else if (d | 0x40) != 0 && (a | 0x40) != 0 {
                    0
                } else {
                    1
                };
                let w = wq(u(f, 0x61), 0xb);
                let f5b = byte(f, 0x5b);
                let f57 = byte(f, 0x57);
                if w > 0xcb2
                    && (w > 0x1d4c || (w.wrapping_add((f57 as i16).wrapping_add(s.wrapping_mul(20)).wrapping_mul(25))) > wq(clube, 0x80))
                    && f5b < 0xf
                    && f57 > 5
                    && (byte(f, 0x5d) as i32 + f5b as i32) < 0x19
                {
                    let mut p: i16 = match s {
                        6 => 0x19,
                        2 => 5,
                        0 => -3,
                        _ => 0,
                    };
                    p += if f5b == 1 { 0x14 } else if f5b < 6 { 0xa } else { 5 };
                    if (a | 4) != 0 {
                        p -= 5;
                    }
                    if f57 == 1 {
                        p -= 3;
                    } else if f57 < 6 {
                        p -= 1;
                    }
                    if w > 0x186a {
                        p += if f57 > 0xf && f5b < 6 { 5 } else { 3 };
                    }
                    let f5a = byte(f, 0x5a);
                    if f5a > 0xa && byte(f, 0x18) < 0x18 && f57 > 0xf {
                        p += 5;
                    }
                    if f5a == 1 {
                        p -= 0xa;
                    } else if f5a < 6 {
                        p -= 5;
                    }
                    if *((c + 0x45) as *const u32) != 0 {
                        p += 5;
                    } else if bandeiras_do_contrato(c) != 0 {
                        p += 1;
                    } else {
                        p -= 3;
                    }
                    if bandeiras_do_contrato(c) & 0x40 != 0 {
                        p += 5;
                    }
                    if gosta_do_clube(f, clube) != 0 {
                        p -= 3;
                    }
                    if u(clube, 0xcf) != 0 && gosta_de(f, u(clube, 0xcf)) != 0 {
                        p -= 5;
                    }
                    if p > 5 && (p as i32) > sortear(0x1e) {
                        crate::comparacao::ramo("prejogo insatisfeito pela posicao");
                        evento_de_prejogo(m, f, clube, 0x40, 0, k as u8, 0x13);
                    }
                }
            }
            // o interesse de outro clube (0x6b917a)
            let pe = u(j, 0x6d);
            let r = sortear(7);
            if (byte(pe, 0x2f) as i32) > r + 6 && wq(pe, 0xb) > 0xea6 && byte(f, 0x5b) < 7 && (byte(f, 0x5b) as i32) < sortear(0xf) {
                let x = sortear(wq(u(j, 0x6d), 0xb) as i32);
                if sortear(wq(clube, 0x80) as i32) < x && *((c + 0x45) as *const u32) & 0x2f7a0b != 0 {
                    crate::comparacao::ramo("prejogo interesse");
                    evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 0x1e);
                }
            }
            // a forma
            let ficha_61 = u(f, 0x61);
            let mut forma = byte(f, 0x18);
            if ficha_61 != 0 {
                let x0f = byte(ficha_61, 0xf);
                if x0f == 0x14 {
                    forma = forma.wrapping_sub(2);
                } else if byte(ficha_61, 0x11) == 0x14 {
                    forma = forma.wrapping_sub(1);
                } else if byte(ficha_61, 0x15) == 0x14 || byte(ficha_61, 0x14) == 0x14 {
                    forma = forma.wrapping_add(1);
                }
                let x3c = byte(ficha_61, 0x3c);
                if x0f != 0x14 && x3c > 0xf {
                    forma = forma.wrapping_sub(2);
                } else if x3c > 0xa {
                    forma = forma.wrapping_sub(1);
                } else if x0f != 0x14 && x3c < 6 {
                    forma = forma.wrapping_add(1);
                }
            }
            let ficha_media = *(0xb74350 as *const usize);
            if forma < 0x17 {
                let m1 = media(ficha_media, ficha_61, 6, 1, 6.5) as f32;
                let m2 = media(ficha_media, u(f, 0x61), 6, 0x11, 6.5) as f32;
                let mut al = byte(f, 0x5b).wrapping_add(byte(f, 0x5a)).wrapping_add(byte(f, 0x18)).wrapping_sub(0x1e);
                if al < 4 {
                    al = 4;
                } else if forma < 0x12 && al > 0x19 {
                    al = 0x19;
                    let x3c = byte(u(f, 0x61), 0x3c);
                    if x3c < 0xa {
                        al = 0x19 - x3c;
                    }
                }
                let f61 = u(f, 0x61);
                if f61 != 0
                    && byte(f, 0x22) == 0
                    && wq(f61, 0xb) < 0x1b58
                    && wq(clube, 0x80) > 0xcb2
                    && (al as f64) < m1 as f64
                    && *((jogo + 0x40) as *const u8) != 0
                    && sortear(al as i32) == 0
                    && (m2 as f64) < indice()
                {
                    crate::comparacao::ramo("prejogo forma em baixa");
                    evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 5);
                }
            } else if forma > 0x20 {
                let m1 = media(ficha_media, ficha_61, 6, 1, 6.5) as f32;
                let m2 = media(ficha_media, u(f, 0x61), 6, 0x11, 6.5);
                if m1 as f64 > f32_em(0x9689a8) as f64 {
                    let mut z = m1 as f64 * f64_em(0x967880);
                    if !(z < f64_em(0x967890)) {
                        z = f64_em(0x967890);
                    }
                    if m2 < indice() - f64_em(0x967890) + z {
                        crate::comparacao::ramo("prejogo forma em alta");
                        evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 4);
                    }
                }
            }
            // o fim do contrato
            if sortear(byte(f, 0x5d) as i32) == 0 && byte(u(f, 0x61), 0x25) > 0xf {
                let d = dias(0xae2c90, *((f + 0x3e) as *const u32), *((f + 0x42) as *const u32));
                let r = sortear(200) + 0x32;
                if d > r && forma > 0x17 {
                    let g = byte(c, 0x3f);
                    if (g as i32) > sortear(4) + 2 {
                        crate::comparacao::ramo("prejogo fim de contrato");
                        evento_de_prejogo(m, f, clube, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 6);
                    }
                }
            }
        }
    }
}

/// clears the state, the weather (0x923010/0x923970/0x923630), the kits, the referee, the pitch (0x85cba0,
/// +0x1ad/+0x1ae), the squads (0x6db500), the commentator (given or new), the side strength, the marking, the angle
/// table and the pre-match event list (unhappiness 0x13, interest 0x1e, form 5/4, contract end 6); draws who kicks off
/// Original: `prepare_match` (0x6b7f20, match_eng.cpp (6 KB), thiscall, ret 0x10).
///
/// [pt] { 0x6b7f20 } (ret 0x10, match_eng.cpp): ver `preparar_partida_com`.
pub(crate) unsafe extern "thiscall" fn preparar_partida(m: usize, jogo: usize, rapida: u32, narrador: usize, ao_vivo: u32) -> u8 {
    preparar_partida_com(m, jogo, rapida as u8, narrador, ao_vivo as u8)
}

// ponytail: o nascimento, a preparacao e o fim do motor nao tem comparacao propria (alocam o
// narrador e a lista, que a fita teria de reproduzir); validados pela carreira (Rust x original),
// como o laco do motor. As rotinas que eles chamam seguem comparadas.
pub(crate) unsafe extern "thiscall" fn comparar_construir_motor(m: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(325)(m)
}
pub(crate) unsafe extern "thiscall" fn comparar_preparar_partida(m: usize, jogo: usize, rapida: u32, narrador: usize, ao_vivo: u32) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8>(326)(m, jogo, rapida, narrador, ao_vivo)
}
pub(crate) unsafe extern "thiscall" fn comparar_destruir_motor(m: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(327)(m)
}

// ---------- centesima decima quarta leva: o jogador que sai de campo (0x6cda20) ----------

/// { 0x6cda20 } (ret 8, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6e1aa0
/// e 0x6e1090 da comissao, 0x897760 grava o capitao): `j` sai de campo (lesao ou expulsao). Sem
/// `ja_fora`, soma +0x23, grava o minuto (+0x8ed0 / 11, no maximo o fim do periodo: 45, 90, 105, 120)
/// em +0x21 e tira da grade (0x6cdd50 com -3). Fica com +0x19 = -3. Nos jogos com tecnico humano
/// (+0x904d do lado, 0x5fd210/0x5fd480), so segue com +0x904b e fora de selecao. Com +0x1a3 para.
/// Goleiro (tabela 1) com +0x9049 do lado: a comissao troca o goleiro (0x6e1090 com 1). Senao a
/// comissao refaz o time (0x6e1aa0 e 0x6e1090 com 0); se ele era o capitao (0x897590), o mais bem
/// avaliado (0x6e1a40) vira capitao (0x897760); zera o jogador de posicao 1 do lado e o recompoe
/// (0x6ce110). Fora dos jogos com tecnico humano guarda o minuto da troca (+0x8ecc + 2 x lado). No
/// fim refaz a forca do lado (0x6bbb60) e a marcacao individual (0x6bbf60).
pub(crate) unsafe fn jogador_saiu_com(m: usize, j: usize, ja_fora: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tirar: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REMOVE_PLAYER_FROM_PITCH);
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let humano_9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let humano_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let vaga_marcada: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
    let melhor: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(FN_MATCH_MAN_FIND_CAPTAIN_SLOT);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let forca: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_RECALC_SIDE_SCORE);
    let marcacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_ASSIGN_MARKING_TARGETS);
    let b = |o: usize| (j + o) as *mut i8;
    if ja_fora == 0 {
        *b(0x23) = (*b(0x23)).wrapping_add(1);
        let min = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
        *b(0x21) = min;
        let teto = match *((m + 0x8ed4) as *const i16) {
            0x1ef => Some(0x2d),
            0x3de => Some(0x5a),
            0x483 => Some(0x69),
            0x528 => Some(0x78),
            _ => None,
        };
        if let Some(t) = teto {
            *b(0x21) = min.min(t);
        }
        tirar(m, j, -3);
    }
    let goleiro = tabela(j) == 1;
    *b(0x19) = -3;
    let lado = byte(j, 0x27);
    let base = (m as isize + lado as isize * 0xd0) as usize;
    let t = *((m + 0x4792) as *const usize);
    let clube_do = |l: i8| if l == 0 { *((t + 0x1c) as *const usize) } else { *((t + 0x20) as *const usize) };
    if byte(m, 0) == 0 {
        let p = *((base + 0x904d) as *const usize);
        let n = *(0xae2364 as *const i32);
        if p != 0 && *(p as *const i32) >= n - 0x10 && *(p as *const i32) < n && byte(p, 0x5f) != 0 && humano_9a01(*(0xb63c98 as *const usize), p) == 0 {
            if byte(base, 0x904b) == 0 || selecao(clube_do(lado)) != 0 || humano_9a05(*(0xb63c98 as *const usize), p) != 0 {
                return;
            }
        }
    }
    if byte(m, 0x1a3) != 0 {
        return;
    }
    let comissao = base + 0x9032;
    let mut guardar = byte(m, 0) == 0;
    if byte(base, 0x9049) != 0 && goleiro {
        chamar(0x6e1090, comissao, [1, 0, 0, 0, 0]);
    } else if byte(m, 0) != 0 {
        guardar = false;
    } else {
        chamar(0x6e1aa0, comissao, [0, 0, 0, 0, 0]);
        chamar(0x6e1090, comissao, [0, 0, 0, 0, 0]);
        let l = byte(j, 0x27);
        let principal = if l != 0 { byte(t, 0x41) } else { byte(t, 0x40) } == 0;
        let v = vaga_marcada(*(0xde1d08 as *const usize), clube_do(l), principal as u32, 0);
        if j == jogador_do_bloco(m, l, v as isize as usize) {
            let l = byte(j, 0x27);
            let principal = if l != 0 { byte(t, 0x41) } else { byte(t, 0x40) } == 0;
            let k = melhor(m + 0x9032 + l as isize as usize * 0xd0);
            chamar(0x897760, *(0xde1d08 as *const usize), [clube_do(l) as u32, k as u8 as u32, principal as u32, 0, 0]);
        }
        *((m + if byte(j, 0x27) == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;
        manter(m, byte(j, 0x27));
    }
    if guardar {
        let l = byte(j, 0x27) as isize;
        *((m as isize + l * 2 + 0x8ecc) as *mut i16) = *((m + 0x8ed0) as *const i16);
    }
    forca(m, byte(j, 0x27));
    marcacao(m);
}

/// { 0x6cda20 } (ret 8, match_eng.cpp): ver `jogador_saiu_com`.
pub(crate) unsafe extern "thiscall" fn jogador_saiu(m: usize, j: usize, ja_fora: u32) {
    jogador_saiu_com(m, j, ja_fora as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_MAN_MAKE_SUBSTITUTIONS);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

// Portas de 0x6e1aa0 e 0x6e1090 (a comissao refaz o time e faz a troca, match_man.cpp): unidades da
// fita ate serem portadas.


// ---------- centesima nona leva: a lesao na dividida (0x6cfe10) ----------

/// Codigo do lance -> multiplicador do risco de quem tem a bola (tabelas 0x6d08b0/0x6d0884).
pub(crate) const RISCO_POR_LANCE: [u8; 0x2f] = [0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 10, 10, 10, 10, 10, 10, 2, 10, 3, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 2, 4, 10, 10, 10, 10, 10, 10, 5, 6, 7, 8, 2, 2, 9];

/// { 0x6cfe10 } (ret 0x14, match_eng.cpp, ~2,6 KB) com as chamadas que mexem no jogo injetadas
/// (0x62ed70 lesao atual, que antes a atualiza; 0x6d0980 aplica a lesao): a dividida entre `a` e
/// `b` (lances `tipo_a`/`tipo_b`) pode machucar. Cada um tem uma chance (sorteio(sorteio(102)),
/// acima de 50 refeita por sorteio(sorteio(76)) em 2/3) e um risco 4 (6 ja machucado, com a
/// gravidade 0x62ec00 do tipo e a chance dobrada em 24/25), ajustado pelo folego (10000 /
/// max(1000, +0x4d/2 + 5000)), pela ficha (+0x2d, sempre a de `a` decidindo o ramo) e pelo grau
/// da lesao (0x62ece0). `a` sem folego (< 4000) se machuca na hora (-1). Com `direto`, so `a`
/// arrisca: sorteio(50000) abaixo de 10 x risco machuca pela gravidade, ou por sorteio(11).
/// Senao cada um arrisca: quem tem a bola soma o risco pelo lance do outro (+0xd9 do outro x
/// fator da tabela), o lance 5 divide por +0x79; machucado escolhe o tipo pela gravidade (com o
/// outro jogador, 1..=9; sem, 1..=5) ou por sorteio, e o lance 5 tem tipos proprios (6, 9, 10).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn lesao_na_dividida_com(m: usize, a: usize, tipo_a: i16, b: usize, tipo_b: i16, direto: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let gravidade: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_INJURY_GET_TYPE_CATEGORY);
    let grau: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_RECURRING_INJURY_LEVEL);
    let lesoes = *(0xb63d28 as *const usize);
    let f4 = |a: usize| f32_em(a) as f64;
    let d8 = |a: usize| f64_em(a);
    let fl = |p: usize, o: usize| *((p + o) as *const f32) as f64;
    let mut chance = [0i8; 2];
    let mut grav = [-1i8; 2];
    let mut f = [4.0f32; 2];
    for c in chance.iter_mut() {
        *c = sortear(sortear(0x66)) as i8;
        if *c > 0x32 && sortear(3) != 0 {
            *c = sortear(sortear(0x4c)) as i8;
        }
    }
    for (k, p) in [(0usize, a), (1, b)] {
        if p == 0 {
            continue;
        }
        let pessoa = *((p + 0x69) as *const usize);
        let e = chamar(0x62ed70, lesoes, [pessoa as u32, 0, 0, 0, 0]) as usize;
        if e != 0 {
            f[k] = 6.0;
            grav[k] = gravidade(lesoes, *(e as *const i8)) as i8;
            if sortear(0x19) != 0 {
                chance[k] = (chance[k] as u8).wrapping_shl(1) as i8;
            }
        }
        let mut x = fl(p, 0x4d) * f4(0x968f18) + f4(0x968db0);
        if !(x > f4(0x96d2dc)) {
            x = f4(0x96d2dc);
        }
        let q = f4(0x96d1b0) / x * f[k] as f64;
        // o ramo segue a ficha de `a` (como no original), o valor a do proprio jogador
        let ramo = byte(*((a + 0x6d) as *const usize), 0x2d);
        let val = byte(*((p + 0x6d) as *const usize), 0x2d) as i32;
        let v = if ramo > 10 { val as f64 * q * f4(0x96afc4) } else { ((val - 10) as f64 * f4(0x96afc4) + f4(0x968e70)) * q };
        f[k] = v as f32;
        if grau(lesoes, pessoa) != 0 {
            let mut t = grau(lesoes, pessoa) as i8 as f64 * d8(0x968968) + d8(0x968940);
            if t > d8(0x968930) {
                t = f4(0x969584);
            }
            f[k] = (t * f[k] as f64) as f32;
        }
    }
    let mut lesionar = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, tipo: u32, chance: u32, p: usize, o: usize| {
        c(0x6d0980, m, [tipo, chance, p as u32, o as u32, 0]);
    };
    if a != 0 && fl(a, 0x4d) < f4(0x96d40c) {
        lesionar(chamar, u32::MAX, 0, a, 0);
        return;
    }
    if direto != 0 && a != 0 {
        if sortear(50000) >= ftol(f[0] as f64 * f4(0x968a78)) {
            return;
        }
        let tipo = if (1..=5).contains(&grav[0]) {
            grav[0] as u32
        } else {
            match sortear(11) {
                0..=2 => 1,
                3..=5 => 3,
                6 | 7 => 4,
                8 | 9 => 2,
                _ => 5,
            }
        };
        let palavra = chance[0] as u8 as u32 | (chance[1] as u8 as u32) << 8 | (grav[0] as u8 as u32) << 16 | (grav[1] as u8 as u32) << 24;
        lesionar(chamar, tipo, palavra, a, 0);
        return;
    }
    let jog = [a, b];
    let cod = [tipo_a, tipo_b];
    for i in 0..2 {
        let p = jog[i];
        if p == 0 {
            continue;
        }
        let o = jog[1 - i];
        if p == *((m + 0xf582) as *const usize) && o != 0 {
            let c = cod[1 - i];
            let d9 = fl(o, 0xd9);
            let r = if (1..=0x2f).contains(&c) { RISCO_POR_LANCE[c as usize - 1] } else { 10 };
            match r {
                4 => {
                    f[i] = (d9 * d8(0x968fd0) * f[i] as f64) as f32;
                    chance[i] = ftol(chance[i] as f64 * d8(0x9689c0)) as i8;
                }
                8 | 9 => {
                    let k = if r == 8 { d8(0x96afd8) } else { d8(0x96d450) };
                    f[i] = (d9 * k * f[i] as f64) as f32;
                    chance[i] = (chance[i] as u8).wrapping_shl(1) as i8;
                }
                _ => {
                    let k = [0x96d440, 0x96d430, 0x967880, 0x96d438, 0, 0x968fd0, 0x968a98, 0x96d448, 0, 0, 0x968918][r as usize];
                    f[i] = (d9 * d8(k) * f[i] as f64) as f32;
                }
            }
        }
        if cod[i] == 5 {
            let r79 = fl(p, 0x79);
            let x = if r79 > d8(0x968950) { r79 } else { d8(0x968950) };
            f[i] = (f4(0x968a78) / x * f[i] as f64) as f32;
        }
        if sortear(50000) >= ftol(f[i] as f64 * f4(0x968a78)) {
            continue;
        }
        let ch = chance[i] as u8 as u32;
        let g = grav[i];
        if cod[i] == 5 {
            if matches!(g, 6 | 9 | 10) {
                lesionar(chamar, g as u32, ch, p, 0);
                return;
            }
            let t = match sortear(5) {
                0..=2 => 9,
                3 => 6,
                _ => 10,
            };
            lesionar(chamar, t, ch, p, 0);
        } else if byte(m, 0x8ea9) == 4 && o != 0 {
            lesionar(chamar, 0xb, ch, p, o);
        } else if o == 0 {
            if (1..=5).contains(&g) {
                lesionar(chamar, g as u32, ch, p, 0);
                return;
            }
            match sortear(11) {
                1..=3 => lesionar(chamar, 3, ch, p, 0),
                4 | 5 => lesionar(chamar, 4, ch, p, 0),
                6 | 7 => {
                    lesionar(chamar, 2, ch, p, 0);
                    lesionar(chamar, 5, ch, p, 0);
                }
                8 => lesionar(chamar, 5, ch, p, 0),
                _ => lesionar(chamar, 1, ch, p, 0),
            }
        } else {
            if (1..=9).contains(&g) {
                lesionar(chamar, g as u32, ch, p, o);
                return;
            }
            let t = sortear(9) + 1;
            lesionar(chamar, t as u32, ch, p, o);
        }
    }
}

/// { 0x6cfe10 } (ret 0x14, match_eng.cpp): ver `lesao_na_dividida_com`.
pub(crate) unsafe extern "thiscall" fn lesao_na_dividida(m: usize, a: usize, tipo_a: u32, b: usize, tipo_b: u32, direto: u32) {
    lesao_na_dividida_com(m, a, tipo_a as u16 as i16, b, tipo_b as u16 as i16, direto as u8, &mut |rotina, this, x| match rotina {
        0x62ed70 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
            f(this, x[0] as usize) as u32
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_INJURE_PLAYER);
            f(this, x[0], x[1], x[2] as usize, x[3] as usize);
            0
        }
    })
}

// Portas de 0x62ed70 (lesao atual: atualiza a ficha pela 0x62c680) e 0x6d0980 (aplica a lesao,
// alocando): unidades da fita.
pub(crate) static mut PORTA_62ED70: usize = 0;
pub(crate) const PROLOGO_62ED70: Prologo = Prologo { endereco: 0x62ed70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] };

pub(crate) unsafe extern "thiscall" fn porta_62ed70(l: usize, pessoa: usize) -> usize {
    let o: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(PORTA_62ED70);
    if !gravando() {
        return o(l, pessoa);
    }
    chamar_gravando(0x62ed70, l, [pessoa as u32, 0, 0, 0, 0], || o(l, pessoa) as u32) as usize
}

// ---------- centesima setima leva: o uniforme (0x6d47e0) ----------

/// { 0x6d47e0 } (cdecl, match_eng.cpp): qual dos tres uniformes do `clube` (pares de cores +0x83/+0x87,
/// +0x8b/+0x8f, +0x93/+0x97) usar contra o `outro`. Sem clube ou sem outro, registra o erro e devolve 0.
/// Para cada par, a 0x48ce70 (que devolve 0) decide se troca as cores (0x48ce90). Sem `qualquer`, o
/// par vale se a segunda cor nao choca (0x540490) com a do outro (+0x87 dele, ou a da tabela 0xae23ec
/// pelo indice 0x9cf17c); devolve 1, 2 ou 3 conforme o par, 1 se o par tiver cor zero, 2 se faltar
/// o 2o ou o 3o uniforme, 1 se nenhum servir.
pub(crate) unsafe extern "C" fn uniforme(clube: usize, outro: usize, qualquer: u32) -> u8 {
    let decide: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_SHARED_RETURN_ZERO);
    let trocar: unsafe extern "C" fn(*mut usize, *mut usize) -> u32 = core::mem::transmute(FN_SHARED_RETURN_ONE);
    let choca: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_DATABASE_COLOURS_CLASH);
    let u = |o: usize| *((clube + o) as *const usize);
    if clube == 0 || outro == 0 {
        *(0xb67a34 as *mut u32) = 0;
        return 0;
    }
    let qualquer = qualquer as u8 != 0;
    let cor_outro = if qualquer {
        0
    } else {
        let c = *((outro + 0x87) as *const usize);
        if c != 0 { c } else { *(0xae23ec as *const usize) + *(KEY_COLOUR_WHITE as *const i32) as usize * 58 }
    };
    let (mut x, mut y) = (0usize, 0usize);
    let mut k = 0u8;
    loop {
        match k {
            0 => (x, y) = (u(0x83), u(0x87)),
            1 | 2 => {
                (x, y) = if k == 1 { (u(0x8b), u(0x8f)) } else { (u(0x93), u(0x97)) };
                if x == 0 || y == 0 {
                    return 2;
                }
            }
            _ => return 1,
        }
        if decide(x, y) != 0 {
            trocar(&mut x, &mut y);
        }
        k += 1;
        if qualquer || (y != 0 && x != 0 && choca(y, cor_outro) == 0) {
            break;
        }
    }
    if x == 0 || y == 0 { 1 } else { k }
}

// ---------- centesima sexta leva: a evolucao do jogador depois do jogo (0x6d26b0) ----------

/// { 0x6d26b0 } (cdecl, match_eng.cpp, ~3,9 KB): depois do jogo, as tres notas da ficha da pessoa
/// (+0x61: +0x9, +0xb, +0xd, ate 10000) andam pela `nota` do jogo. O `nivel` vem da competicao
/// (`comp`: -2 da 5; com +0x40 a media da reputacao do clube/500 e do +0x69 do jogo; senao o +0x69,
/// 15 no minimo nas selecoes 0x76d940; ate 20). Cada nota converge para a nota + (nivel/2 + 5) x
/// desvio x 50, com o desvio da `nota` contra a media do campeonato (0xb74350; ao quadrado com o
/// sinal quando abaixo) e divisores pela idade da nota (/1000 + 16) menos o nivel; com `forte`, o
/// valor 250 x nivel x nota / media entra como piso. Pisos A/B/C sobem com reputacao (+0x80 do
/// `rep`) e nota em jogos grandes da selecao ou de copas (codigos +0x30/+0x32 da competicao). A
/// diferenca na nota +0xb da o ajuste do moral (0x7172d0 no jogador do jogo, ou 0x544710 na
/// pessoa) por sorteio, ate +-20. Depois as notas ficam presas a faixa do potencial (+0x5) e da
/// idade (250/300/375/500 x (2 - nota/10000)), e a +0xb puxa a reputacao do clube (0x4cf5f0,
/// idade +0x18 ate 19, potencia 3 da CRT). Com `jm` grava em +0x47/+0x49/+0x4b dele.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn evoluir_jogador(jm: usize, pessoa: usize, clube: usize, rep: usize, comp: usize, nota: f32, media_b: u32, forte: u32) {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let moral_jogo: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_ADJUST_MORALE);
    let moral_pessoa: unsafe extern "C" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let d8 = |a: usize| f64_em(a);
    let f4 = |a: usize| f32_em(a) as f64;
    let ficha = || *((pessoa + 0x61) as *const usize);
    let do_jogo = *((comp + 0x14) as *const usize);
    let reputacao = || w(rep, 0x80) as i32;
    // o nivel da competicao
    let mut nivel: i8 = if *(comp as *const i32) == -2 {
        5
    } else {
        let n = if byte(comp, 0x40) != 0 {
            Some(((reputacao() / 500 + w(do_jogo, 0x69) as i32) / 2) as i8)
        } else if *(0xb74318 as *const usize) != 0 && especial(*(0xb74318 as *const usize), do_jogo) != 0 {
            let v = w(do_jogo, 0x69);
            if v <= 0xf { None } else { Some(v as i8) }
        } else {
            Some(byte(do_jogo, 0x69))
        };
        match n {
            None => 15,
            Some(n) => n.min(20),
        }
    };
    let mut fator = nota;
    if jm != 0 && !((nota as f64) < d8(0x968f10)) {
        if byte(jm, 0xc) == 0 {
            fator = (nota as f64 * f4(0x96d48c)) as f32;
        }
        if byte(jm, 0x10) == 0 {
            fator = (fator as f64 * f4(0x96d488)) as f32;
        }
        if byte(jm, 0xd) != byte(jm, 0xe) {
            fator = (fator as f64 * f4(0x96d488)) as f32;
        }
        if byte(jm, 0x23) != 0 {
            fator = (fator as f64 * f4(0x96d48c)) as f32;
        }
    }
    // a "idade" de cada nota: /1000 + 16, com minimo
    let idade = |o: usize, min: i8| ((w(ficha(), o) as i32 / 1000) as i8).wrapping_add(0x10).max(min);
    let nivel0 = nivel as i16;
    let div_b = (idade(0xb, 0x16) as i16).wrapping_sub(nivel0);
    let mut grande = false;
    let ja_selecao = selecao(clube) != 0;
    let div_9 = if ja_selecao
        || (*(comp as *const i32) == -2 && *((clube + 0x53) as *const i32) != 0 && *((clube + 0x53) as *const i32) == *((pessoa + 0x1a) as *const i32))
        || (do_jogo != 0 && *((do_jogo + 0x5d) as *const i32) == *((pessoa + 0x1a) as *const i32))
        || *((clube + 0x53) as *const i32) == *((pessoa + 0x1a) as *const i32)
    {
        grande = true;
        (idade(9, 0x16) as i16).wrapping_sub(nivel0)
    } else {
        let v = (((0xca - nivel0 as i32 * 10) as i16 as i32 + (idade(9, 0x16) as i16).wrapping_sub(nivel0) as i32) / 2) as i16;
        v.max(4)
    };
    // pisos: (A, B, C) para +0xb, +0x9, +0xd
    let (mut pa, mut pb, mut pc) = (1i32, 1i32, 1i32);
    let piso = |p: f64, flag: bool| -> (i32, i32, i32) {
        let a = ftol(if p < d8(0x96d470) { p } else { d8(0x96d470) });
        let pk = p * d8(0x96a670);
        let c = ftol(if pk < d8(0x96d468) { pk } else { d8(0x96d468) });
        (a, if flag { a } else { c }, c)
    };
    let div_d: i16;
    if selecao(clube) != 0 {
        div_d = (idade(0xd, 0x15) as i16).wrapping_sub(nivel0);
        if byte(comp, 0x3a) == 0xa
            && *(0xb74318 as *const usize) != 0
            && especial(*(0xb74318 as *const usize), do_jogo) != 0
            && w(do_jogo, 0x69) >= 0x11
            && !((fator as f64) < d8(0x968988))
            && reputacao() > 0x157c
        {
            let c32 = w(comp, 0x32);
            let c30 = w(comp, 0x30);
            if (0x41f..=0x426).contains(&c32) || c30 == 0x8c {
                (pa, pb, pc) = piso((reputacao() - 0x1194) as f64 * (fator as f64 - d8(0x968988)), grande);
            } else {
                let lim = match c30 {
                    0x14 => Some((0x1194, 0x96af90, 0x9690b8)),
                    0x78 => Some((0xfa0, 0x96af90, 0x96d480)),
                    0x82 => Some((0xfa0, 0x96af90, 0x968dc8)),
                    0x96 => Some((0xfa0, 0x969000, 0x96d478)),
                    _ => None,
                };
                if let Some((menos, k, teto)) = lim {
                    let p = (reputacao() - menos) as f64 * (fator as f64 - d8(k));
                    let v = ftol(if p < d8(teto) { p } else { d8(teto) });
                    (pa, pb, pc) = (v, v, v);
                }
            }
        }
    } else if byte(comp, 0x3a) != 8 {
        div_d = (0xca - nivel0 as i32 * 10) as i16;
    } else {
        let c32 = w(comp, 0x32);
        let c30 = w(comp, 0x30);
        if (0x43d..=0x448).contains(&c32) {
            div_d = (idade(0xd, 0x15) as i16).wrapping_sub(nivel0).wrapping_add(1);
            if !((fator as f64) < d8(0x968f10)) && reputacao() > 0x157c {
                let p = (reputacao() - 0x157c) as f64 * (fator as f64 - d8(0x968f10));
                pa = ftol(p * d8(0x96afd8));
                let c = ftol(p * d8(0x968a80));
                pb = if grande { pa } else { c };
                pc = c;
            }
        } else if matches!(c30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b) {
            let id = idade(0xd, 0x15);
            nivel = byte(do_jogo, 0x69);
            div_d = (id as i16).wrapping_sub(nivel as i16);
            if !((fator as f64) < d8(0x968eb0)) && reputacao() > 0x157c {
                (pa, pb, pc) = piso((reputacao() - 0x1194) as f64 * (fator as f64 - d8(0x968988)), grande);
            }
        } else {
            div_d = (0xca - nivel0 as i32 * 10) as i16;
        }
    }
    // o desvio da nota contra a media do campeonato
    let media = || {
        let g = *(0xb74350 as *const usize);
        let (soma, n) = if media_b as u8 != 0 { (0x20, 0x28) } else { (0x10, 0x18) };
        if *((g + n) as *const f64) == d8(0x9689a0) { d8(0x968998) } else { *((g + soma) as *const f64) / *((g + n) as *const f64) }
    };
    let mut desvio = nota as f64 - media();
    if desvio < f4(0x968970) {
        desvio *= desvio.abs();
    }
    let st = ficha();
    let (v9, vb, vd) = (w(st, 9) as i32, w(st, 0xb) as i32, w(st, 0xd) as i32);
    let passo = (nivel as i32 as f64 * d8(0x968968) + d8(0x9688a0)) * desvio * d8(0x968ff8);
    let mut n9 = ftol(v9 as f64 + passo);
    let mut nb = ftol(vb as f64 + passo);
    let mut nd = ftol(vd as f64 + passo);
    if forte as u8 != 0 {
        let mut c = ftol((nivel as i32 * 250) as f64 * nota as f64 / media());
        if desvio < f4(0x968970) {
            c = ftol(c as i16 as f64 * nota as f64 / media());
        }
        let c16 = c as i16;
        if c16 > nb as i16 {
            nb = c;
        }
        if c16 > n9 as i16 {
            n9 = c;
        }
        if c16 > nd as i16 {
            nd = c;
        }
    }
    let novo_b = ((div_b as i32 - 1) * vb + nb as i16 as i32) / div_b as i32;
    let novo_9 = ((div_9 as i32 - 1) * v9 + n9 as i16 as i32) / div_9 as i32;
    let novo_d = ((div_d as i32 - 1) * vd + nd as i16 as i32) / div_d as i32;
    // o moral pela diferenca na nota +0xb
    let base = if vb as i16 > 1 { vb } else { 1 };
    let dif = novo_b as i16 as i32 - vb;
    let x = ftol(dif as f64 * d8(0x96a6b0) / base as f64 + (nota as f64 + d8(0x967890)) * d8(0x968968)) as i16;
    let mut m: i16 = if x > 0 {
        (sortear(3) + (x as i32).min(dif)) as i16
    } else if x == 0 {
        0
    } else {
        let v = x as f64 * d8(0x968940);
        let t = if v > dif as f64 { v } else { dif as f64 };
        ftol(t - sortear(3) as f64) as i16
    };
    if m < 0 {
        let r = sortear(byte(pessoa, 0x58) as i32) + 1;
        m = ftol(m as f32 as f64 / (r as f64 * d8(0x9689e0))) as i16;
    }
    let delta = sortear(m.clamp(-20, 20) as i32);
    if jm != 0 {
        moral_jogo(jm, delta as i8);
    } else {
        moral_pessoa(pessoa, delta as i8, 1);
    }
    // as faixas do potencial e da idade
    let st = ficha();
    let (cb, c9, cd) = (w(st, 0xb), w(st, 9), w(st, 0xd));
    let f_b = (d8(0x968950) - cb as f64 * d8(0x96a5c0)) as f32;
    let f_9 = (d8(0x968950) - c9 as f64 * d8(0x96a5c0)) as f32;
    let f_d = (d8(0x968950) - cd as f64 * d8(0x96a5c0)) as f32;
    let lim = (nivel as i16 as i32 * 300 + 4000) as i16;
    let pot = w(st, 5) as i32;
    let faixa = |novo: i32, atual: i16, lo: i32, hi: i32| -> i32 {
        let mx = lim.max(atual);
        let v = if (novo as i16) < mx { novo as i16 } else { mx } as i32;
        if lo > v { lo } else if hi >= v { v } else { hi }
    };
    let preso = |atual: i16, f: f32, baixo: f64, alto: f64, t: i32| -> i32 {
        let t = t as i16 as f32 as f64;
        let menor = atual as f32 as f64 - f as f64 * baixo;
        if menor > t {
            return ftol(menor);
        }
        let maior = alto * f as f64 + atual as f32 as f64;
        ftol(if maior < t { maior } else { t })
    };
    let lo = pot * 50 - 3750;
    let tb = faixa(novo_b, cb, lo, pot * 50 + 2500);
    let mut rb = preso(cb, f_b, f4(0x96d210), f4(0x96d20c), tb);
    let t9 = faixa(novo_9, c9, lo, pot * 50 + 2500);
    let r9 = preso(c9, f_9, f4(0x96d210), f4(0x96d208), t9);
    let td = faixa(novo_d, cd, pot * 50 - 10000, pot * 50 + 1250);
    let rd = preso(cd, f_d, f4(0x96d1a8), (nivel as i32 * 100) as f64, td);
    // a reputacao do clube puxa a +0xb
    if selecao(clube) == 0 && nivel > 10 {
        let e = contrato(*(0xae1050 as *const usize), pessoa, clube);
        if e != 0 {
            let mut id = byte(pessoa, 0x18) as i32 as f32;
            if f32_em(0x96d460) < id {
                id = 19.0;
            }
            let k = (pow_do_jogo(id as f64 * d8(0x96a578), d8(0x968930)) * d8(0x968968)) as f32 as f64;
            let r = w(clube, 0x80) as f64;
            let alvo = match byte(e, 0x4f) as u8 >> 4 {
                1 => ftol(r * k * d8(0x968978)),
                2 => ftol(r * k * d8(0x9689c0)),
                3 | 5 => ftol(r * k),
                4 => ftol(r * k * d8(0x9689b0)),
                _ => 0,
            };
            if (rb as i16) < alvo as i16 {
                rb = ftol(alvo as i16 as f64 * d8(0x967880) + rb as i16 as f64 * d8(0x9689d8));
            }
        }
    }
    let fim = |v: i32, p: i32| -> i16 {
        let v = (v as i16).min(10000);
        if v < p as i16 { p as i16 } else { v }
    };
    let (b, n, d) = (fim(rb, pa), fim(r9, pb), fim(rd, pc));
    if jm != 0 {
        *((jm + 0x4b) as *mut i16) = d;
        *((jm + 0x47) as *mut i16) = b;
        *((jm + 0x49) as *mut i16) = n;
    } else {
        let st = ficha();
        *((st + 0xb) as *mut i16) = b;
        *((st + 9) as *mut i16) = n;
        *((st + 0xd) as *mut i16) = d;
    }
}

// ---------- centesima quinta leva: o desafio dos adversarios (0x6c87e0) ----------

/// { 0x6c87e0 } (ret 0x24, match_eng.cpp, `CM3_MATCH_ENGINE::get_opposition_challenge`, ~5,3 KB):
/// quem, entre os adversarios ativos na casa da bola, desafia o lance `tipo` de `atacante` (ou do
/// `lado`, sem atacante). A distancia da bola ao gol (tabela 0xa15e44, 1000 fora dela) da a
/// dificuldade `dist`; cada adversario em pe (+0x2b < 2) tem um peso: 3,5 com +0x8ea9 = 4, senao
/// pelas marcas taticas (0x7163c0, ou o byte de estrategia do lado com 0x200) 0,5, 4 ou 2; vezes
/// 0,05 longe do gol, 2,25 se marca o atacante (+0x1a6), dobrado se o persegue (+0x1aa), o sorteio
/// pela leitura (+0x106) e o +0x19c do atacante, a forma (+0x24, +0x106, +0x75), o folego (+0x4d,
/// fora da tabela 1), 1/4 com a estrategia 5 do lado; no minimo 0,05. Cada `tipo` tem a sua
/// disputa: sorteio(k / peso) contra +0x10d + +0x10a - dist (variantes para 0x11/0x12, 0x15 com a
/// potencia 5 (pow da CRT) de quantos estao na casa, 0x6c com 10,5 - dist). Quem vence recebe um codigo e os
/// lances alternativos; o impedimento (0x6cb780) e a posicao decidem `*arriscado`, o valor
/// (0x7165b0, 0x5407e0, 0x6f5b20) escolhe entre os lances, e o dominio (0x70d690) da a nota. O de
/// maior nota fica em `*nota`/`*lance` e cansa (0x6f56c0 com -1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn desafio_adversario(m: usize, atacante: usize, lado: i8, tipo: i8, a: *mut u8, b: *mut u8, nota: *mut i16, lance: *mut i8, arriscado: *mut i8, livre: u8) -> usize {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_GET_INSTRUCTIONS);
    let faixa: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
    let mais_perto: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_CHECK_CLEAR_RUN_ON_GOAL);
    let x_fora: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_ON_WING);
    let perto_gol: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_OWN_THIRD);
    let limite: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_ATTACKING_THIRD);
    let valor: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_FOUL_CHANCE);
    let dominio: unsafe extern "thiscall" fn(usize, *mut i8, *mut u8, *mut u8) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_CHALLENGE_SCORE);
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let desgastar: unsafe extern "thiscall" fn(usize, f32, i16) = core::mem::transmute(FN_MATCH_PLAYER_DRAIN_STAMINA);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let f = |p: usize, o: usize| *((p + o) as *const f32);
    *arriscado = 0;
    let lado = if atacante != 0 { byte(atacante, 0x27) } else { lado };
    let flag = (lado == 0) as i32;
    if byte(m, 0x8ea9) == -1 && (atacante == *((m + 0xf59e) as *const usize) || atacante == *((m + 0xf5a2) as *const usize)) {
        return 0;
    }
    let bx = byte(m, 0x8ea7) as i32;
    let by = byte(m, 0x8ea8) as i32;
    let cx = ((bx - 4).abs()) as i8;
    let cy = ((if flag == 1 { by - 11 } else { by }).abs()) as i8;
    let base = if (0..=8).contains(&cx) && (0..=11).contains(&cy) { *((0xa15e44 + (cy as usize * 9 + cx as usize) * 4) as *const f32) } else { f32_em(0x96d2dc) };
    let mut dist = (base as f64 + f64_em(0x968968)) as f32;
    if dist as f64 > f64_em(0x967890) {
        let x = dist as f64 * f64_em(0x968968);
        dist = (x * x + x + f64_em(0x967890)) as f32;
    }
    let distf = dist as f64;
    let mut melhor = 0usize;
    let mut contado = false;
    let mut mais_perto_r = 0i8;
    let mut colegas_a_frente = 1i8;
    let casa = m + 0x215e + ((by + bx * 12) * 90) as usize;
    let n = || byte(casa + flag as usize, 0x58) as i32;
    let mut i = 0;
    while i < n() {
        let p = *((casa + (flag as usize * 11 + i as usize) * 4) as *const usize);
        i += 1;
        if p == 0 {
            // o original registra o erro e encerra o jogo
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
            sair(-1);
        }
        if byte(p, 0x19) < 0 || byte(p, 0x2b) >= 2 {
            continue;
        }
        // o peso fica no registro (precisao dupla) entre as contas; a copia em memoria e float
        let mut v: f64 = if byte(m, 0x8ea9) == 4 {
            f32_em(0x969080) as f64
        } else if marcas(p) & 0x200 != 0 {
            let e = *((m + 0x9766 + 6371 * flag as usize) as *const u32);
            if e & 0x20 != 0 {
                f32_em(0x968f18) as f64
            } else if e & 0x40 != 0 {
                f32_em(0x96d228) as f64
            } else {
                f32_em(0x9689a8) as f64
            }
        } else if marcas(p) & 0x800 != 0 {
            f32_em(0x968f18) as f64
        } else if marcas(p) & 0x1000 != 0 {
            f32_em(0x96d228) as f64
        } else {
            f32_em(0x9689a8) as f64
        };
        let mut peso = v as f32;
        if distf > f64_em(0x9688a0) {
            v *= f32_em(0x96d424) as f64;
            peso = v as f32;
        }
        if atacante != 0 {
            if atacante == *((p + 0x1a6) as *const usize) {
                v *= f32_em(0x96d420) as f64;
                peso = v as f32;
            }
            if *((p + 0x1aa) as *const usize) == atacante {
                v += v;
                peso = v as f32;
            }
            if byte(m, 0x8ea9) < 4 {
                let leitura = byte(p, 0x106) as i32;
                let r1 = sortear(0x1e - leitura);
                let r2 = sortear(0x1e - leitura);
                let k = (w(atacante, 0x19c) as i32).wrapping_add(r2.wrapping_sub(r1).wrapping_mul(6)) as i16;
                v = (k as f64 * f64_em(0x969028) + f64_em(0x967890)) * peso as f64;
            }
        }
        let forma = (byte(p, 0x24) as i32 * byte(p, 0x106) as i32) as f64;
        peso = (((f64_em(0x96a730) - forma * f64_em(0x969028)) * f(p, 0x75) as f64 + f64_em(0x968968)) * v) as f32;
        if tabela(p) != 1 {
            peso = (f(p, 0x4d) as f64 * f64_em(0x968eb8) * f(p, 0x4d) as f64 * peso as f64) as f32;
        }
        if byte(m + flag as usize, 0x1ab) == 5 {
            peso = (peso as f64 * f64_em(0x9689e0)) as f32;
        }
        if (peso as f64) < f64_em(0x967888) {
            peso = f32::from_bits(0x3d4ccccd);
        }
        let somado = (byte(p, 0x10d) as i32 + byte(p, 0x10a) as i32) as f64;
        // a disputa: sorteio(k / peso) abaixo do alvo
        let disputa = |k: i32| sortear(ftol(k as f64 / peso as f64));
        let atacante_t1 = || atacante != 0 && tabela(atacante) == 1;
        let bit100 = *((p + 0x198) as *const u16) & 0x100 != 0;
        let letras = || match sortear(4) {
            0 => (0x2a, 0x2c),
            1 => (0x2d, 0x2c),
            2 => (0x2b, 0x2c),
            _ => (0x2e, 0x2c),
        };
        // (codigo, lance, alternativo)
        let venceu: Option<(i8, i8, i8)> = match tipo {
            0xc..=0xe => {
                let k = if atacante_t1() { 3 } else if bit100 { 10 } else { 20 };
                let alvo = (somado - distf) as f32;
                if (disputa(k) as f32) < alvo {
                    let (l, a2) = letras();
                    Some((0x22, l, a2))
                } else {
                    None
                }
            }
            0xf | 0x10 | 0x67 => {
                let k = if atacante_t1() { 3 } else if bit100 { 5 } else { 10 };
                let alvo = (somado - distf) as f32;
                if (disputa(k) as f32) < alvo {
                    let (l, a2) = letras();
                    Some((0x21, l, a2))
                } else {
                    None
                }
            }
            0x11 | 0x12 => {
                let (perto, longe) = if tipo == 0x11 { (12, 8) } else { (8, 4) };
                // None: vence direto; Some(k): disputa
                let k = if tabela(p) == 1 {
                    if melhor != 0 || n() > 1 {
                        Some(0xc)
                    } else if flag == 0 {
                        if byte(p, 0x103) > 0 || byte(p, 0x102) != 4 { None } else { Some(2) }
                    } else if byte(p, 0x103) < 0xb || byte(p, 0x102) != 4 {
                        None
                    } else {
                        Some(2)
                    }
                } else if livre != 0 {
                    Some(2)
                } else if atacante_t1() {
                    Some(3)
                } else if bit100 {
                    Some(longe)
                } else {
                    Some(perto)
                };
                let vence = match k {
                    None => true,
                    Some(k) => {
                        let alvo = somado - distf * f64_em(0x968968);
                        ((disputa(k) + byte(atacante, 0x104) as i32) as f64) < alvo
                    }
                };
                if vence {
                    let (l, a2) = letras();
                    Some((0x13, l, a2))
                } else {
                    None
                }
            }
            0x15 | 0x25 | 0x2e | 0x38 => {
                let k: Option<i32> = if tabela(p) == 1 {
                    let quantos = n() as f64;
                    let mais = if byte(p, 0x102) == 4 && matches!(byte(p, 0x103), 0 | 0xb) { f64_em(0x9688b0) } else { f64_em(0x968930) };
                    let q5 = pow_do_jogo(quantos, f64_em(0x9688a0));
                    Some(ftol(q5 * byte(p, 0x106) as f64 + mais))
                } else {
                    let r = sortear(if distf < f64_em(0x968950) { 100 } else { 10 });
                    if r as f64 > distf {
                        None
                    } else if bit100 || atacante_t1() {
                        Some(1)
                    } else {
                        Some(3)
                    }
                };
                let vence = match k {
                    None => true,
                    Some(k) => {
                        let r = sortear(ftol(f(p, 0xad) as f64 + f(p, 0xad) as f64)) + byte(p, 0x10a) as i32;
                        r > sortear(k as i16 as i32)
                    }
                };
                if vence {
                    let a2 = if sortear(5) != 0 { -1 } else { 0x2f };
                    Some((0x15, 0x2e, a2))
                } else {
                    None
                }
            }
            0x3b => {
                let k = if atacante_t1() { 3 } else if bit100 { 20 } else { 60 };
                let alvo = (somado - distf) as f32;
                if (disputa(k) as f32) < alvo { Some((0xc, 0x32, 0x32)) } else { None }
            }
            0x6c => {
                let mut k = if atacante_t1() { 10 } else if bit100 { 500 } else { 1000 };
                let (x, y) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
                if (2..=6).contains(&x) && ((flag == 1 && (10..=11).contains(&y)) || (flag == 0 && (0..=1).contains(&y))) {
                    k += 0x514;
                }
                let alvo = (f64_em(0x96d418) - distf) * f(p, 0x75) as f64 + somado;
                if (disputa(k) as f64) < alvo && byte(m, 0x8ea9) < 4 {
                    let (l, a2) = letras();
                    Some((0x22, l, a2))
                } else {
                    None
                }
            }
            0x6d => {
                let k = if tabela(p) == 1 {
                    if faixa(p, flag as u32) as u8 == 0 {
                        None
                    } else if byte(m, 0x8eb3) != 0 {
                        Some(5)
                    } else if livre != 0 {
                        Some(6)
                    } else {
                        Some(30)
                    }
                } else if livre != 0 {
                    if byte(p, 0x105) != 0 { None } else { Some(2) }
                } else if bit100 {
                    Some(2)
                } else {
                    Some(4)
                };
                let vence = match k {
                    None => true,
                    Some(k) => {
                        let alvo = (somado - distf) as f32;
                        (disputa(k) as f32) < alvo
                    }
                };
                if !vence {
                    None
                } else if byte(m, 0x8ea9) < 2 {
                    let (l, a2) = letras();
                    Some((0x22, l, a2))
                } else {
                    Some((0xc, 0x2e, 0x2e))
                }
            }
            _ => {
                let erro: unsafe extern "C" fn(usize, usize, i32) = core::mem::transmute(FN_CRT_SPRINTF);
                erro(0xde1f64, 0xa15954, tipo as i32);
                *(0xb67a34 as *mut u32) = 0;
                None
            }
        };
        let Some((codigo, lance1, mut lance2)) = venceu else { continue };
        if atacante != 0 && !contado && tipo != 0x3b && tipo != 0x15 {
            mais_perto_r = mais_perto(m, atacante) as i8;
            let y = byte(atacante, 0x103);
            let l = byte(atacante, 0x27);
            let mut conta = 0i8;
            for k in 0..20 {
                let q = m + 0x4796 + l as usize * 0x22d8 + k * 0x1be;
                if byte(q, 0x19) >= 0 && ((l == 1 && byte(q, 0x103) < y) || (l == 0 && byte(q, 0x103) > y)) {
                    conta += 1;
                }
            }
            colegas_a_frente = conta;
            contado = true;
        }
        if mais_perto_r != 0 {
            *arriscado = 1;
        } else {
            let y = byte(p, 0x103);
            let ultimo = if byte(p, 0x27) == 0 {
                let u = *((m + 0xf58e) as *const usize);
                y <= if u != 0 { byte(u, 0x103) } else { 0xc }
            } else {
                let u = *((m + 0xf592) as *const usize);
                y >= if u != 0 { byte(u, 0x103) } else { -1 }
            };
            let segue = if ultimo && sortear(10) == 0 {
                true
            } else {
                let c = byte(p, 0x19);
                (0..=10).contains(&c) && *((*((m + 0x8ebc + byte(p, 0x27) as usize * 4) as *const usize) + c as usize * 2) as *const i16) == 1 && sortear(2) == 0
            };
            if segue && colegas_a_frente == 0 && (x_fora(p) == 0 || perto_gol(p) == 0) {
                *arriscado = 1;
            }
        }
        let mut forca = sortear(valor(p, atacante) as i8 as i32);
        if atacante != 0 && limite(atacante) == 0 && *arriscado == 0 && byte(p, 0x24) == 0 {
            let r1 = sortear(ftol(f(p, 0x75) as f64));
            if r1 > sortear(0xc) {
                let pula = w(atacante, 0x35) as i32 <= sortear(1000) + 0x1d4c && {
                    let rel = relacao(*((atacante + 0x69) as *const usize), *((p + 0x69) as *const usize), 0) as i32;
                    rel <= sortear(10000)
                };
                if !pula {
                    forca = forca.wrapping_add(sortear(10));
                    if byte(p, 0x106) != 0 {
                        lance2 = -1;
                    }
                }
            }
        }
        let leitura = byte(p, 0x106) as i32;
        let b5c = || byte(*((p + 0x69) as *const usize), 0x5c) as i32;
        let arr = *arriscado as i32;
        let escolhido: Option<i8> = 'e: {
            if tipo == 0x15 {
                let fx = faixa(p, flag as u32) as i8 as i32;
                if sortear(b5c() * 200 + fx * 1500 + 0xfa) == 0 {
                    break 'e Some(0x25);
                }
            }
            let fo = forca as i16 as i32;
            let fx = faixa(p, flag as u32) as i8 as i32;
            let lim = ((leitura + 5) * byte(p, 0x24) as i32 + fx * leitura * 14 + leitura * arr * 20) * 25 + 0x14;
            if fo > sortear(lim) {
                break 'e Some(lance1);
            }
            if lance2 > -1 {
                let fx = faixa(p, flag as u32) as i8 as i32;
                let lim = (fx * (leitura + 10) + 10) * 500 + (b5c() + 100) * leitura * arr * 80;
                if fo * fo > sortear(lim) {
                    break 'e Some(lance2);
                }
            }
            if arr != 0 && tabela(p) != 1 && !(distf < f64_em(0x968950)) && byte(m, 0x8ea9) != 4 && leitura != 0 {
                break 'e None;
            }
            Some(codigo)
        };
        let Some(mut escolha) = escolhido else { continue };
        let mut valor_lance = sortear(dominio(p, &mut escolha, a, b) as i16 as i32);
        if tipo == 0x11 && escolha != 0x29 {
            valor_lance = valor_lance.wrapping_add(sortear(ftol(byte(p, 0x104) as f64 * f(p, 0x71) as f64)));
        } else if tipo == 0x12 && escolha != 0x29 {
            let c = byte(p, 0x104) as i32;
            valor_lance = valor_lance.wrapping_add(c * c);
        }
        let v16 = valor_lance as i16;
        let aceita = v16 > 0 || faixa(p, byte(p, 0x27) as u8 as u32) as u8 != 0 || (leitura == 0 && byte(p, 0x10d) as i32 > sortear(100));
        if !aceita {
            continue;
        }
        if melhor != 0 && v16 <= *nota {
            continue;
        }
        melhor = p;
        *lance = escolha;
        *nota = v16;
    }
    if melhor != 0 {
        desgastar(melhor, -1.0, *lance as i16);
    }
    melhor
}

// ---------- centesima quarta leva: a pressao sobre o jogador (0x6bcbd0) ----------

/// { 0x6bcbd0 } (ret 0xc, match_eng.cpp) com a copia na lista injetada (0x68d0d0), sem sortear:
/// soma em `*saida` a pressao dos adversarios ativos na casa de `j` (a lista do lado de la na grade)
/// e devolve quantos pesam (2 por marcador). Quem marca `j` (0x714c60) vale (0x6f4600 + 0x10a -
/// +0x19c de `j` / 20 - +0x107 de `j`) x 1,7; os outros em pe (+0x2b >= 0), fora da tabela 1
/// (0x717410) e com +0x10a ou o bit 0x100 de +0x198, valem (0x10a - ...) x 1,7 / (+0x2b + 1). Com
/// `lista`, os dois que mais pesam (acima de 5) vao para ela como copia de 0x1be bytes.
pub(crate) unsafe fn pressao_com(m: usize, j: usize, saida: *mut i16, lista: usize, copiar: &mut dyn FnMut(usize, usize)) -> u8 {
    let na_lista: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_MARKED_BY);
    let pela_posicao: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_MARKING_RATING);
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    *saida = 0;
    let mut conta = 0u8;
    let (mut melhor, mut v_melhor) = (0usize, -1000i32);
    let (mut segundo, mut v_segundo) = (0usize, -1000i32);
    let casa = m + 0x215e + (byte(j, 0x102) as isize * 12 + byte(j, 0x103) as isize).wrapping_mul(90) as usize;
    let lado = (byte(j, 0x27) == 0) as usize;
    let n = byte(casa + lado, 0x58) as i32;
    let base = || byte(j, 0x107) as i32 + *((j + 0x19c) as *const i16) as i32 / 20;
    let k = f64_em(0x96d2e0);
    for i in 0..n.max(0) as usize {
        let p = *((casa + lado * 44 + i * 4) as *const usize);
        if byte(p, 0x19) < 0 {
            continue;
        }
        let mut peso = 0i32;
        if na_lista(j, p) != 0 {
            conta = conta.wrapping_add(2);
            let v = pela_posicao(p) as i8 as i32 + byte(p, 0x10a) as i32 - base();
            let r = ftol(v as f64 * k);
            *saida = (*saida).wrapping_add(r as i16);
            peso = r.wrapping_add(r);
        } else if byte(p, 0x2b) >= 0 && tabela(p) != 1 && (byte(p, 0x10a) != 0 || *((p + 0x198) as *const u16) & 0x100 != 0) {
            conta = conta.wrapping_add(1);
            let v = byte(p, 0x10a) as i32 - base();
            peso = ftol(v as f64 * k / (byte(p, 0x2b) as i32 + 1) as f64);
            *saida = (*saida).wrapping_add(peso as i16);
        }
        if lista != 0 && peso as i16 > 5 {
            if peso as i16 > v_melhor as i16 {
                if melhor != 0 {
                    segundo = melhor;
                    v_segundo = v_melhor;
                }
                melhor = p;
                v_melhor = peso;
            } else if peso as i16 > v_segundo as i16 {
                segundo = p;
                v_segundo = peso;
            }
        }
    }
    if lista != 0 {
        if melhor != 0 {
            copiar(lista, melhor);
        }
        if segundo != 0 {
            copiar(lista, segundo);
        }
    }
    conta
}

/// { 0x6bcbd0 } (ret 0xc, match_eng.cpp): ver `pressao_com`.
pub(crate) unsafe extern "thiscall" fn pressao(m: usize, j: usize, saida: *mut i16, lista: usize) -> u8 {
    pressao_com(m, j, saida, lista, &mut |l, p| {
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
        f(l, p, 0x1be);
    })
}

// Porta de 0x68d0d0 (poe na lista a copia de um bloco, alocando): unidade da fita, para a copia
// Rust nao alocar de novo.
pub(crate) static mut PORTA_68D0D0: usize = 0;
pub(crate) const PROLOGO_68D0D0: Prologo = Prologo { endereco: 0x68d0d0, bytes: &[0x6a, 0xff, 0x68, 0x56, 0xcb, 0x95, 0x00] };

pub(crate) unsafe extern "thiscall" fn porta_68d0d0(l: usize, p: usize, n: u32) {
    let o: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(PORTA_68D0D0);
    if !gravando() {
        return o(l, p, n);
    }
    if gravando_fechamento() || gravando_lesao() || gravando_movimento() || gravando_passe() {
        // no fechamento (e na lesao) o bloco vem da pilha (endereco diferente na copia Rust): confere o conteudo
        chamar_gravando_vetores(0x68d0d0, l, [0, n, 0, 0, 0], &[(p, n as usize)], &[], || {
            o(l, p, n);
            0
        });
        return;
    }
    chamar_gravando(0x68d0d0, lista_gravada(l, n), [p as u32, n, 0, 0, 0], || {
        o(l, p, n);
        0
    });
}

// ---------- centesima terceira leva: o fim do periodo (0x6bd930) ----------

/// { 0x6bd930 } (match_eng.cpp, `CM3_MATCH_ENGINE::period_completed`) com as chamadas que mexem no
/// jogo injetadas (comentarios 0x6d6f70, classificacao 0x6be120, resumo 0x6ce460): fecha o periodo
/// de +0x8ed4 quando o relogio (+0x8ed0) passa do fim dele mais os acrescimos (+0x8eb7). Com +0x8eb2
/// = 5 nada. Periodo 0 ou 1: devolve 4. 0x1ef (1o tempo): grava o placar do intervalo no jogo
/// (+0x4792: +0x45/+0x46), narra 0x20f1, recupera o folego e passa ao 2o tempo (devolve 1). 0x3de
/// (2o tempo): grava o placar (+0x43/+0x44); pela regra do jogo (+0x38: 2 jogo de volta, 4, 0x4000;
/// 1 e 0x20 decidem por penaltis 4 ou 1 em +0x8eb3) e o agregado (+0x4b, +0x4c), encerra (4, com a
/// classificacao narrada, teto do folego e resumo) ou vai a prorrogacao (narra 0x20f2, devolve 2).
/// 0x483 (prorrogacao, 1o tempo): com +0x1a8 encerra; senao narra 0x20f3 e passa ao 2o (3). 0x528:
/// grava +0x47/+0x48 e encerra, com penaltis quando o agregado da prorrogacao empata. Periodo
/// desconhecido: registra o erro (0x944c9f) e devolve 0, como quando o relogio nao passou.
pub(crate) unsafe fn periodo_encerrado_com(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let folego: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_RECOVER_CONDITION_AT_BREAK);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_RELEASE_POSSESSION);
    let b = |o: usize| (m + o) as *mut i8;
    let w = |o: usize| (m + o) as *mut i16;
    if byte(m, 0x8eb2) == 5 {
        return 0;
    }
    let per = *w(0x8ed4) as i32;
    let passou = |fim: i32| *w(0x8ed0) as i32 > byte(m, 0x8eb7) as i32 + fim;
    let t = *((m + 0x4792) as *const usize);
    let tb = |o: usize| (t + o) as *mut i8;
    let narrador = *((m + 0x8f7a) as *const usize);
    let fala = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, cod: u32| {
        c(0x6d6f70, narrador, [cod, m as u32, u32::MAX, 0, u32::MAX]);
    };
    let classificar = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        c(0x6be120, m, [0, 0, 0, 0, 0]);
        c(0x6be120, m, [1, 0, 0, 0, 0]);
    };
    let recuperar = |teto: u8| {
        for base in [0x4796, 0x6a6e] {
            for k in 0..20 {
                folego(m + base + k * 0x1be, teto);
            }
        }
    };
    let soltar_dono = || {
        let dono = *((m + 0xf582) as *const usize);
        if dono != 0 {
            soltar(dono);
        }
    };
    // a posse da saida: quem nao saiu no 1o tempo sai agora
    let trocar_saida = || {
        let a = byte(m, 0x8eb6);
        *b(0x8eb2) = 8;
        *b(0x1ca) = 0;
        *b(0x8eae) = a;
        *b(0x8eb6) = (a == 0) as i8;
    };
    match per {
        0x1ef => {
            if !passou(0x1ef) {
                return 0;
            }
            *tb(0x45) = byte(m, 0xf5bd);
            *tb(0x46) = byte(m, 0xf5f3);
            fala(chamar, 0x20f1);
            *w(0x8ed0) = 0x1ef;
            *b(0x8eb7) = 0;
            chamar(0x6ce460, m, [0x3de, 0, 0, 0, 0]);
            recuperar(0);
            *w(0x8ed4) = 0x3de;
            *b(0x8eb2) = 8;
            *b(0x1ca) = 0;
            soltar_dono();
            trocar_saida();
            1
        }
        0x3de => {
            if !passou(0x3de) {
                return 0;
            }
            *tb(0x43) = byte(m, 0xf5bd);
            *tb(0x44) = byte(m, 0xf5f3);
            let regra = *((t + 0x38) as *const u16);
            let cl = byte(t, 0x4b);
            let empate = || byte(t, 0x43) as i32 + cl as i32 == byte(t, 0x4c) as i32 + byte(t, 0x44) as i32;
            // encerra com a classificacao narrada antes (ou depois) de fechar o periodo
            let encerrar_antes = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
                *b(0x8eb2) = 8;
                *b(0x1ca) = 0;
                *w(0x8ed4) = 0;
                classificar(c);
                recuperar(1);
                c(0x6ce460, m, [0, 0, 0, 0, 0]);
                4
            };
            if regra & 2 == 0 {
                let decide = cl != -1 && regra & 4 == 0 && byte(t, 0x44) != cl && empate();
                if !decide && regra & 1 != 0 && empate() {
                    *b(0x8eb3) = 4;
                } else if !decide && regra & 0x20 != 0 && empate() {
                    *b(0x8eb3) = 1;
                } else {
                    classificar(chamar);
                }
                *b(0x8eb2) = 8;
                *b(0x1ca) = 0;
                recuperar(1);
                chamar(0x6ce460, m, [0, 0, 0, 0, 0]);
                *w(0x8ed4) = 0;
                return 4;
            }
            if cl == -1 {
                if byte(t, 0x43) != byte(t, 0x44) {
                    return encerrar_antes(chamar);
                }
                fala(chamar, 0x20f2);
                *w(0x8ed0) = 0x3de;
                *b(0x8eb7) = 0;
                *w(0x8ed4) = 0x483;
                soltar_dono();
                chamar(0x6ce460, m, [0x3de, 0, 0, 0, 0]);
                trocar_saida();
                return 2;
            }
            if !empate() {
                return encerrar_antes(chamar);
            }
            if regra & 4 == 0 && byte(t, 0x44) != cl && regra & 0x4000 == 0 {
                return encerrar_antes(chamar);
            }
            fala(chamar, 0x20f2);
            *w(0x8ed0) = 0x3de;
            *b(0x8eb7) = 0;
            *w(0x8ed4) = 0x483;
            chamar(0x6ce460, m, [0x3de, 0, 0, 0, 0]);
            soltar_dono();
            trocar_saida();
            2
        }
        0x483 => {
            if byte(m, 0x1a8) != 0 {
                *tb(0x47) = byte(m, 0xf5bd);
                *tb(0x48) = byte(m, 0xf5f3);
                *b(0x8eb2) = 8;
                *b(0x1ca) = 0;
                *w(0x8ed4) = 1;
                classificar(chamar);
                recuperar(1);
                chamar(0x6ce460, m, [1, 0, 0, 0, 0]);
                return 4;
            }
            if !passou(0x483) {
                return 0;
            }
            fala(chamar, 0x20f3);
            *w(0x8ed0) = 0x483;
            *b(0x8eb7) = 0;
            *w(0x8ed4) = 0x528;
            *b(0x8eb2) = 8;
            *b(0x1ca) = 0;
            soltar_dono();
            trocar_saida();
            3
        }
        0x528 => {
            if byte(m, 0x1a8) == 0 && !passou(0x528) {
                return 0;
            }
            *tb(0x47) = byte(m, 0xf5bd);
            *tb(0x48) = byte(m, 0xf5f3);
            let cl = byte(t, 0x4b);
            let regra = *((t + 0x38) as *const u16);
            let empate = || byte(t, 0x47) as i32 + cl as i32 == byte(t, 0x4c) as i32 + byte(t, 0x48) as i32;
            let penaltis = if cl == -1 {
                regra & 0x21 != 0
            } else if regra & 0x4000 != 0 {
                empate() && byte(t, 0x48) == cl && regra & 0x21 != 0
            } else {
                empate() && regra & 0x21 != 0
            };
            let iguais = byte(t, 0x47) as i32 - byte(t, 0x43) as i32 == byte(t, 0x48) as i32 - byte(t, 0x44) as i32;
            if regra & 1 != 0 && iguais && penaltis {
                *b(0x8eb3) = 4;
            } else if regra & 0x20 != 0 && iguais && penaltis {
                *b(0x8eb3) = 1;
            } else {
                classificar(chamar);
            }
            *b(0x8eb2) = 8;
            *b(0x1ca) = 0;
            *w(0x8ed4) = 1;
            recuperar(1);
            chamar(0x6ce460, m, [1, 0, 0, 0, 0]);
            4
        }
        0 | 1 => 4,
        _ => {
            let erro: unsafe extern "C" fn(usize, usize, i32) = core::mem::transmute(FN_CRT_SPRINTF);
            erro(0xde1f64, 0xa158f8, per);
            0
        }
    }
}

/// closes the period in +0x8ed4 when the clock (+0x8ed0) passes its end plus stoppage (+0x8eb7). 1st half: half-time
/// score into the fixture (+0x4792 +0x45/+0x46), narrates 0x20f1, recovers stamina and moves to the 2nd (1). 2nd half:
/// by the fixture rule (+0x38) and aggregate (+0x4b/+0x4c), ends (4: standings 0x6be120, stamina ceiling, summary
/// 0x6ce460; penalties in +0x8eb3) or goes to extra time (0x20f2, 2). Extra time: 0x20f3 and 2nd half (3), then ends.
/// Unknown period: logs the error (0x944c9f). Standings and summary become tape units when called directly by the
/// compared routine
/// Original: `period_completed` (0x6bd930, match_eng.cpp (`period_completed`), thiscall).
///
/// [pt] { 0x6bd930 } (match_eng.cpp): ver `periodo_encerrado_com`.
pub(crate) unsafe extern "thiscall" fn periodo_encerrado(m: usize) -> u8 {
    periodo_encerrado_com(m, &mut |rotina, this, a| {
        match rotina {
            0x6be120 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_POST_PERIOD_END_EVENTS);
                f(this, a[0]);
            }
            0x6ce460 => {
                let f: unsafe extern "thiscall" fn(usize, i16) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS);
                f(this, a[0] as i16);
            }
            _ => comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]),
        }
        0
    })
}

// ---------- centesima leva: a bola desviada (0x6cd770) ----------

/// { 0x6cd770 } (ret 0x1c, match_eng.cpp) com as chamadas que mexem no jogo injetadas (0x70a910,
/// 0x70c040 lances na linha de fundo, 0x6cd490 bola fora): a bola espirra de (x, y). Solta a bola do
/// dono, ninguem a tem (+0x8eae = -1); `lado` -1 vira sorteio(2) = 0. Sem `dentro`, fora do miolo e
/// por sorteio(4), sai pela lateral (coluna 9 ou -1) com a linha +- sorteio(espalhar + 1); senao cai
/// em x e y +- sorteio(espalhar + 1) (com `dentro`, ate a linha ficar em 0..=11). Altura (+0x8eaa)
/// 1 + sorteio(espalhar + 1), ou 0 sem espalhar; efeito (+0x8ed8) pela tabela 0xb67c30 do desvio (o
/// mesmo sem desvio); +0x8ea9 = sorteio(4); a bola fica na casa e o `lado` conta a posse (+0x8d46, o
/// outro perde 10). Com `cansa`, quem esta em campo sem +0x105 soma sorteio(2) em +0x2b (0x7160d0) se
/// sorteio(3) for zero. Na linha de fundo (coluna 4, linha -1 ou 12) com `chutador`, por sorteio(4):
/// 0x70c040 ou 0x70a910 pelo lado dele e a linha; depois, sem +0x8eb2, a bola fora (0x6cd490).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn desviar_bola_com(this: usize, x: i8, y: i8, espalhar: i8, mut lado: i8, dentro: u8, cansa: u8, chutador: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_RELEASE_POSSESSION);
    let somar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_ADD_ACTION_DELAY);
    let b = |o: usize| (this + o) as *mut i8;
    let dono = *((this + 0xf582) as *const usize);
    if dono != 0 {
        soltar(dono);
    }
    *b(0x8ea7) = x;
    *b(0x8ea8) = y;
    if byte(this, 0x8eae) != -1 {
        *b(0x8eae) = -1;
    }
    if lado == -1 {
        lado = (sortear(2) == 0) as i8;
    }
    let n = espalhar as i32 + 1;
    let (tx, mut ty): (i8, i8);
    let bx = byte(this, 0x8ea7);
    if dentro == 0 && !(2..=6).contains(&bx) && sortear(4) != 0 {
        tx = if bx >= 2 { 9 } else { -1 };
        let r = sortear(n) as i8;
        ty = byte(this, 0x8ea8).wrapping_sub(r).wrapping_add(sortear(n) as i8);
    } else {
        loop {
            let r = sortear(n) as i8;
            let x2 = byte(this, 0x8ea7).wrapping_sub(r).wrapping_add(sortear(n) as i8);
            let r = sortear(n) as i8;
            ty = byte(this, 0x8ea8).wrapping_sub(r).wrapping_add(sortear(n) as i8);
            if dentro == 0 || (0..=11).contains(&ty) {
                tx = x2;
                break;
            }
        }
    }
    *b(0x8eaa) = if espalhar == 0 { 0 } else { (sortear(n) + 1) as i8 };
    let dx = tx.wrapping_sub(byte(this, 0x8ea7));
    let dy = ty.wrapping_sub(byte(this, 0x8ea8));
    if dx != 0 || dy != 0 {
        let i = dy as isize + 24 * dx as isize;
        *((this + 0x8ed8) as *mut u16) = *((0xb67c30 + 2 * i) as *const u16);
    }
    *b(0x8ea9) = sortear(4) as i8;
    *b(0x8ea7) = tx;
    *b(0x8ea8) = ty;
    *b(0x8eab) = lado;
    if lado != -1 {
        let meu = (this as isize + 0x8d46 + lado as isize) as *mut i8;
        *meu = (*meu).wrapping_add(1);
        let outro = (this as isize + 0x8d46 + (lado == 0) as isize) as *mut i8;
        if *outro > 0 {
            *outro = (*outro).wrapping_sub(10);
        }
    }
    if cansa != 0 {
        for l in 0..2i8 {
            for k in 0..20 {
                let j = jogador_do_bloco(this, l, k);
                if byte(j, 0x19) >= 0 && byte(j, 0x105) == 0 && sortear(3) == 0 {
                    somar_2b(j, sortear(2) as u8);
                }
            }
        }
    }
    let fundo = chutador != 0 && byte(this, 0x8ea7) == 4 && matches!(byte(this, 0x8ea8), -1 | 12);
    if fundo && sortear(4) != 0 {
        let d = byte(chutador, 0x27);
        let rotina = if byte(this, 0x8ea8) == 12 { if d != 0 { 0x70c040 } else { 0x70a910 } } else if d == 0 { 0x70c040 } else { 0x70a910 };
        chamar(rotina, chutador, [0x3f80_0000, 0, 0, 0, 0]);
        if byte(this, 0x8eb2) != 0 {
            return;
        }
    }
    chamar(0x6cd490, this, [lado as u8 as u32, 0, 0, 0, 0]);
}

/// { 0x6cd770 } (ret 0x1c, match_eng.cpp): ver `desviar_bola_com`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn desviar_bola(this: usize, x: i8, y: i8, espalhar: i8, lado: i8, dentro: u8, cansa: u8, chutador: usize) {
    desviar_bola_com(this, x, y, espalhar, lado, dentro, cansa, chutador, &mut |rotina, alvo, a| match rotina {
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY);
            f(alvo, a[0] as u8) as u32
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(rotina);
            f(alvo, f32::from_bits(a[0]));
            0
        }
    })
}

// Portas de 0x70a910 e 0x70c040 (lances na linha de fundo, ~6 e 5 KB): unidades da fita.


// ---------- nonagesima quarta leva: arrumar para uma bola parada (0x6c4820) ----------

/// Depois de mover para a casa nova: o jogador sem a bola passou da linha defensiva do lado dele
/// (+0x8eb0 menos y no lado 1, y menos +0x8eaf no outro, nao negativo)? Quem tem a bola (+0xf582)
/// conta como passou. Trecho repetido de 0x6c4ec0 e 0x6c4820.
pub(crate) unsafe fn passou_da_linha(motor: usize, j: usize) -> bool {
    if j == *((motor + 0xf582) as *const usize) {
        return true;
    }
    let y = byte(j, 0x103);
    let d = if lado(j) == 1 { byte(motor, 0x8eb0).wrapping_sub(y) } else { y.wrapping_sub(byte(motor, 0x8eaf)) };
    d >= 0
}

/// counter +0xf5c6 + 54 × side; releases the ball. Ball in the side's own area: formation (as 0x6c4ec0) and whoever of
/// the other side lands on the spot moves one row. Otherwise: spot clamped to the grid, far (+0x8eb1 = 7) or near (−1),
/// base column 4 − draw + draw, base row 3/4 from the spot; near, everyone from the set-piece table (12 rows × 9
/// columns per position and kicking side); far, by role (0x716270 in attack: 1 deep, 2 middle; 0x716310 in defence: 4
/// deep, 3 middle). Finally the taker (0x6ceb90, type +0x8eb2) goes to the spot, takes the ball and aims (0x6f4920).
/// Cell off the grid: fatal error, as the original
/// Original: `line_up_set_piece` (0x6c4820, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4820 } (match_eng.cpp): arruma os times para a bola parada do lado +0x8eae (contador
/// +0xf5c6 + 54 x lado sobe; +0x8eae = -1; a bola e solta, 0x6f0970).
/// Bola na area do proprio lado (linha < 3 no lado 0, > 8 no lado 1): cada um vai a casa da formacao
/// (como 0x6c4ec0), e quem do outro lado cair na marca da bola sai uma linha; no lado que cobra,
/// quem passou da linha volta uma (0x6f5b80).
/// Senao (marca limitada a 0..=8 x 0..=11): longe do gol (+0x8eb1 = 7) ou perto (-1); coluna base 4 -
/// sorteio(2) + sorteio(2) e linha base a 3 (perto) ou 4 (longe) da marca para o lado do gol. Perto,
/// todos pela tabela de bola parada do lado (+0xf56e, 12 linhas por coluna, 9 colunas por posicao e
/// lado que cobra). Longe, pelo papel: no ataque (0x716270) 1 vai ao fundo e 2 ao meio, na defesa
/// (0x716310) 4 ao fundo e 3 ao meio, os outros pela tabela. Meio: x 4 +- sorteio, linha 6 (lado 1) ou 5.
/// Fundo: x da coluna base +- sorteio e linha 0..1 ou 10..11 junto ao gol atacado ou defendido. Quem
/// passou da linha volta uma (0x6f5b80). Casa fora da grade: a original encerra o jogo (erro fatal).
/// No fim o cobrador (0x6ceb90 com o tipo +0x8eb2) vai a marca, pega a bola (0x6f0630) e aponta
/// (0x6f4920, 4, 1 no lado 1 ou 10), com +0x19c = 200. Todos zeram +0x2b (0x7160e0).
pub(crate) unsafe extern "thiscall" fn arrumar_bola_parada(this: usize) {
    let b = |o: usize| (this + o) as *mut i8;
    let u = |o: usize| *((this + o) as *const usize);
    let lado_b = byte(this, 0x8eae);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_RELEASE_POSSESSION);
    let linhas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_UPDATE_OFFSIDE_LINES);
    let avancar: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW);
    let gravar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_SET_ACTION_DELAY);
    let pegar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_POSSESSION);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let apontar: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_SET_FACING_DIRECTION);
    let papel_ataque: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_ATTACKING_SET_PIECE_ROLE);
    let papel_defesa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_DEFENDING_SET_PIECE_ROLE);
    let contador = (this as isize + 0xf5c6 + 54 * lado_b as isize) as *mut u8;
    *contador = (*contador).wrapping_add(1);
    *b(0x8eae) = -1;
    if u(0xf582) != 0 {
        soltar(u(0xf582));
    }
    let na_area = (lado_b == 0 && byte(this, 0x8ea8) < 3) || (lado_b == 1 && byte(this, 0x8ea8) > 8);
    if na_area {
        for l in 0..2i8 {
            let sai = lado_b as i32 == l as i32;
            for k in 0..20 {
                let j = jogador_do_bloco(this, l, k);
                let pos = byte(j, 0x19);
                if pos < 0 {
                    continue;
                }
                let tabela = u(0xf56e + 4 * (byte(j, 0x27) == 1) as usize);
                let casa = *((tabela + pos as usize * 0xd8 + 0xa2) as *const u8);
                let x = (casa & 0xf) as i8;
                let mut y = (casa >> 4) as i8;
                if x == byte(this, 0x8ea7) && y == byte(this, 0x8ea8) && !sai {
                    y = if lado_b == 0 {
                        if y < 11 { y + 1 } else { y - 1 }
                    } else if y <= 0 {
                        y + 1
                    } else {
                        y - 1
                    };
                }
                mover(j, x, y);
                linhas(this);
                if passou_da_linha(this, j) && sai {
                    avancar(j, 1);
                }
                gravar_2b(j, 0);
            }
        }
    } else {
        if byte(this, 0x8ea8) >= 12 {
            *b(0x8ea8) = 11;
        } else if byte(this, 0x8ea8) < 0 {
            *b(0x8ea8) = 0;
        }
        if byte(this, 0x8ea7) >= 9 {
            *b(0x8ea7) = 8;
        } else if byte(this, 0x8ea7) < 0 {
            *b(0x8ea7) = 0;
        }
        let marca = byte(this, 0x8ea8);
        let longe = if lado_b == 0 { marca > 6 } else { marca < 5 };
        *b(0x8eb1) = if longe { 7 } else { -1 };
        let s1 = sortear(2) as i8;
        let coluna = 4i8.wrapping_sub(s1).wrapping_add(sortear(2) as i8);
        let passo = if longe { 4 } else { 3 };
        let bruta = if lado_b == 0 { marca.wrapping_add(passo) } else { marca.wrapping_sub(passo) };
        let linha = if bruta >= 12 { 11 } else if bruta < 0 { 0 } else { bruta };
        for l in 0..2i8 {
            let sai = lado_b as i32 == l as i32;
            for k in 0..20 {
                let j = jogador_do_bloco(this, l, k);
                let pos = byte(j, 0x19);
                if pos < 0 {
                    continue;
                }
                let da_tabela = || {
                    let t = u(0xf56e + 4 * l as usize);
                    let c = *((t as isize + 12 * ((sai as isize + 2 * pos as isize) * 9 + coluna as isize) + linha as isize) as *const u8);
                    ((c & 0xf) as i8, (c >> 4) as i8)
                };
                let (x, y) = if byte(this, 0x8eb1) <= 0 {
                    da_tabela()
                } else {
                    let (r, fundo, meio) = if sai { (papel_ataque(j), 1, 2) } else { (papel_defesa(j), 4, 3) };
                    if r == meio {
                        let s1 = sortear(2) as i8;
                        (4i8.wrapping_sub(s1).wrapping_add(sortear(2) as i8), if l == 1 { 6 } else { 5 })
                    } else if r == fundo {
                        let s1 = sortear(2) as i8;
                        let x = coluna.wrapping_sub(s1).wrapping_add(sortear(2) as i8);
                        let s3 = sortear(2) as i8;
                        // junto ao gol de cima (linha 11) quem defende o lado 1 ou ataca com o lado 0
                        (x, if (l == 1) != sai { 11 - s3 } else { s3 })
                    } else {
                        da_tabela()
                    }
                };
                if !(0..=8).contains(&x) || !(0..=11).contains(&y) {
                    // a original mostra a caixa de erro e encerra o jogo (0x5cc7d0, 0x5a8f60, exit(-1))
                    let f1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
                    let f2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
                    let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
                    f1();
                    f2();
                    sair(-1);
                }
                mover(j, x, y);
                linhas(this);
                if passou_da_linha(this, j) {
                    avancar(j, 1);
                }
                gravar_2b(j, 0);
            }
        }
    }
    let quem = cobrador(this, lado_b, byte(this, 0x8eb2), 0);
    *b(0x8ea9) = 0;
    mover(quem, byte(this, 0x8ea7), byte(this, 0x8ea8));
    pegar(quem);
    apontar(quem, byte(this, 0x8ea7), byte(this, 0x8ea8), 4, if lado_b != 0 { 1 } else { 10 });
    *((quem + 0x19c) as *mut u16) = 200;
}

// ---------- nonagesima terceira leva: arrumar para a saida de bola (0x6c4ec0) ----------

/// who kicks off (+0xf59e side 0, centre 4, 0; +0xf5a2 side 1, centre 4, 11); releases the ball (0x6f0970); every
/// player on the pitch goes to his formation cell (table +0xf56e of the direction, byte +0xa2 of the position: x in the
/// low nibble, y in the high one), the defensive lines are redone (0x6c3aa0) and, on the kicking-off side, whoever
/// crossed the line steps back one row (0x6f5b80); +0x2b is cleared; finally the kicker goes to the centre and takes
/// the ball (0x6f0630), +0x19c = 200, +0x19a = 180 off row 0. Without him: the original (error box). Compared with
/// `lado_a_lado` over the engine from 0x1a0, the grid, the 40 players and the 0xf56e pointers
/// Original: `line_up_kickoff` (0x6c4ec0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4ec0 } (match_eng.cpp): arruma os times para a saida de bola do lado +0x8eae. Quem sai
/// jogando: +0xf59e (lado 0, centro 4, 0) ou +0xf5a2 (lado 1, centro 4, 11; outro lado nao mexe no
/// centro). Solta a bola (0x6f0970) de quem a tem; cada jogador em campo (+0x19 >= 0) vai para a casa
/// da formacao (tabela +0xf56e do lado da direcao +0x27, byte +0xa2 da posicao: x no nibble baixo, y no
/// alto), as linhas defensivas sao refeitas (0x6c3aa0) e, no lado que sai jogando, quem nao tem a bola
/// e passou da linha (+0x8eb0 / +0x8eaf) volta uma linha (0x6f5b80 direto); +0x2b zera (0x7160e0). No
/// fim, quem sai jogando vai ao centro (+0x8ea9 = 0) e pega a bola (0x6f0630), com +0x19c = 200 e
/// +0x19a = 180 se o centro nao for a linha 0. Sem ele: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn arrumar_saida(this: usize) {
    let lado_bola = byte(this, 0x8eae);
    let u = |o: usize| *((this + o) as *const usize);
    let b = |o: usize| (this + o) as *mut i8;
    let quem = if lado_bola == 0 { u(0xf59e) } else { u(0xf5a2) };
    if quem == 0 {
        return orig::<unsafe extern "thiscall" fn(usize)>(254)(this);
    }
    if lado_bola == 0 {
        *b(0x8ea7) = 4;
        *b(0x8ea8) = 0;
    } else if lado_bola == 1 {
        *b(0x8ea7) = 4;
        *b(0x8ea8) = 11;
    }
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_RELEASE_POSSESSION);
    let linhas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_UPDATE_OFFSIDE_LINES);
    let avancar: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW);
    let gravar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_SET_ACTION_DELAY);
    let pegar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_POSSESSION);
    if u(0xf582) != 0 {
        soltar(u(0xf582));
    }
    for l in 0..2i8 {
        let sai_jogando = lado_bola as i32 == l as i32;
        for k in 0..20 {
            let j = jogador_do_bloco(this, l, k);
            let pos = byte(j, 0x19);
            if pos < 0 {
                continue;
            }
            let tabela = u(0xf56e + 4 * (byte(j, 0x27) == 1) as usize);
            let casa = *((tabela + pos as usize * 0xd8 + 0xa2) as *const u8);
            mover(j, (casa & 0xf) as i8, (casa >> 4) as i8);
            linhas(this);
            if passou_da_linha(this, j) && sai_jogando {
                avancar(j, 1);
            }
            gravar_2b(j, 0);
        }
    }
    *b(0x8ea9) = 0;
    mover(quem, byte(this, 0x8ea7), byte(this, 0x8ea8));
    pegar(quem);
    *((quem + 0x19c) as *mut u16) = 200;
    *((quem + 0x19a) as *mut u16) = if byte(this, 0x8ea8) != 0 { 0xb4 } else { 0 };
}

// ---------- nonagesima primeira leva: interceptacao no caminho do gol (0x6cc540) ----------

/// { 0x6cc540 } (ret 0x18, match_eng.cpp) com a procura (0x6cd3d0 em modo 1) injetada: alguem
/// intercepta no caminho de (x, y) ate o centro do gol (x 4; y 11 com `gol` = 1, senao 0)? As casas
/// saem ao estilo de Bresenham, da de saida ate a do gol; em cada uma sorteia (2) o lado que tenta
/// primeiro e procura nos dois (distancia da tabela desde a saida, ou 1000). Volta o primeiro codigo
/// diferente de zero; 0 ao sair da grade ou ao chegar. +0xf5ae do motor zera no inicio.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn interceptar_com(this: usize, x: i8, y: i8, a3: u32, gol: i8, a5: u32, a6: u32, procurar: &mut dyn FnMut(i8, usize, u32, u32, u32, u32) -> u8) -> u8 {
    *((this + 0xf5ae) as *mut usize) = 0;
    let alvo: i8 = if gol == 1 { 11 } else { 0 };
    let dx = (4 - x as i32).abs() as i8;
    let dy = (alvo as i32 - y as i32).abs() as i8;
    if dx == 0 && dy == 0 {
        return 0;
    }
    // Some(codigo) encerra (0 fora da grade); None segue para a proxima casa
    let mut casa = |cx: i8, cy: i8| -> Option<u8> {
        if !(0..=8).contains(&cx) || !(0..=11).contains(&cy) {
            return Some(0);
        }
        let grupo = (this as isize + 2 * 45 * (cy as isize + 12 * cx as isize) + 0x215e) as usize;
        let v = distancia_ou_mil(cx as i32 - x as i32, cy as i32 - y as i32).to_bits();
        let moeda = sortear(2) as i8;
        for lado in [moeda, (moeda == 0) as i8] {
            let r = procurar(lado, grupo, v, a3, a5, a6);
            if r != 0 {
                return Some(r);
            }
        }
        None
    };
    let (mut cx, mut cy, mut erro) = (x, y, 0i8);
    if dx >= dy {
        let passo: i8 = if alvo as i32 - y as i32 >= 0 { 1 } else { -1 };
        let sobe = x < 4;
        loop {
            if erro >= dx {
                erro = erro.wrapping_sub(dx);
                cy = cy.wrapping_add(passo);
            }
            if let Some(r) = casa(cx, cy) {
                return r;
            }
            erro = erro.wrapping_add(dy);
            cx = cx.wrapping_add(if sobe { 1 } else { -1 });
            if (sobe && cx > 4) || (!sobe && cx < 4) {
                return 0;
            }
        }
    }
    let passo: i8 = if 4 - x as i32 >= 0 { 1 } else { -1 };
    let sobe = y < alvo;
    loop {
        if erro >= dy {
            erro = erro.wrapping_sub(dy);
            cx = cx.wrapping_add(passo);
        }
        if let Some(r) = casa(cx, cy) {
            return r;
        }
        erro = erro.wrapping_add(dx);
        cy = cy.wrapping_add(if sobe { 1 } else { -1 });
        if (sobe && cy > alvo) || (!sobe && cy < alvo) {
            return 0;
        }
    }
}

/// { 0x6cc540 } (ret 0x18, match_eng.cpp): ver `interceptar_com`.
pub(crate) unsafe extern "thiscall" fn interceptar(this: usize, x: i8, y: i8, a3: u32, gol: i8, a5: u32, a6: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u8, u32, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_TRY_CELL_SHOT_BLOCK);
    interceptar_com(this, x, y, a3, gol, a5, a6, &mut |lado, grupo, v, a3, a5, a6| f(this, lado, grupo, v, a3, 1, a5, a6))
}

// ---------- nonagesima leva: quem tenta a jogada (0x6cd3d0), com gravacao da 0x70dfd0 ----------

/// { 0x6cd3d0 } (ret 0x1c, match_eng.cpp) com a jogada (0x70dfd0) injetada: percorre a lista do
/// lado no grupo (+0x58 jogadores) atras de quem pode agir (+0x2b ate 1 e +0x19 nao negativo). Com
/// modo 2 so conta. Senao cada um tenta a jogada (argumentos 4, 3, 6 e 7): 1 = conseguiu (+0x12
/// sobe), 2 = outra saida; nos dois casos o jogador fica em +0xf5ae do motor e o codigo volta.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn procurar_jogada_com(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, modo: u8, a6: u32, a7: u32, jogada: &mut dyn FnMut(usize, [u32; 4]) -> u8) -> u8 {
    let lista = (grupo as isize + lado as isize * 44) as *const usize;
    let quantos = || byte((grupo as isize + lado as isize) as usize, 0x58) as i32;
    let mut n = 0u8;
    let mut k = 0;
    while k < quantos() {
        let p = *lista.add(k as usize);
        if byte(p, 0x2b) <= 1 && byte(p, 0x19) >= 0 {
            if modo == 2 {
                n = n.wrapping_add(1);
            } else {
                match jogada(p, [a4, a3, a6, a7]) as i8 {
                    1 => {
                        *((p + 0x12) as *mut u8) = (*((p + 0x12) as *const u8)).wrapping_add(1);
                        *((this + 0xf5ae) as *mut usize) = p;
                        return 1;
                    }
                    2 => {
                        *((this + 0xf5ae) as *mut usize) = p;
                        return 2;
                    }
                    _ => {}
                }
            }
        }
        k += 1;
    }
    n
}

/// { 0x6cd3d0 } (ret 0x1c, match_eng.cpp): ver `procurar_jogada_com`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn procurar_jogada(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, modo: u8, a6: u32, a7: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_TRY_BLOCK_SHOT);
    procurar_jogada_com(this, lado, grupo, a3, a4, modo, a6, a7, &mut |p, a| f(p, a[0], a[1], a[2], a[3]))
}

// Fita de chamadas com efeitos (0x70dfd0 a jogada, 0x6f09f0 o reposicionamento, 0x6f46b0 o
// movimento). Na comparacao de uma rotina portada que as chama, cada chamada da original e gravada
// com a resposta e as regioes pedidas pelo comparador como ficaram depois; a copia Rust confere a
// sequencia e reescreve essas regioes no lugar de chamar de novo.
pub(crate) type Chamada = (usize, usize, [u32; 5], u32, crate::comparacao::Memoria, crate::comparacao::Memoria, (Vec<u8>, Vec<u8>)); // (rotina, this, argumentos, resposta, antes, depois, (bytes dos vetores de entrada, de saida))
#[allow(clippy::type_complexity)]
pub(crate) static FITA: std::sync::Mutex<Option<(Vec<(usize, usize)>, Vec<Chamada>)>> = std::sync::Mutex::new(None);

pub(crate) fn fita() -> std::sync::MutexGuard<'static, Option<(Vec<(usize, usize)>, Vec<Chamada>)>> {
    FITA.lock().unwrap_or_else(|e| e.into_inner())
}

/// Ha uma gravacao aberta (uma comparacao de fora ja grava estas chamadas)?
pub(crate) fn gravando() -> bool {
    fita().is_some()
}

/// Gravacao aberta e no nivel de cima dela (fora de qualquer unidade gravada em andamento): a
/// rotina e chamada direto pela que esta sendo comparada e entra na fita (ou so repassa). Dentro de
/// uma unidade, ela pode ser comparada por conta propria (a fita de fora fica guardada).
pub(crate) fn gravando_no_topo() -> bool {
    gravando() && DENTRO.load(std::sync::atomic::Ordering::Relaxed) == 0
}

/// Quantas chamadas gravaveis estao em andamento: so as de fora (profundidade 0) entram na fita;
/// as que uma delas faz por dentro (0x70dfd0 e 0x6f09f0 movem jogadores) ja estao na foto.
pub(crate) static DENTRO: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Roda uma chamada gravavel e guarda-a na fita aberta, com a foto das regioes depois dela.
pub(crate) unsafe fn chamar_gravando(rotina: usize, this: usize, a: [u32; 5], chamada: impl FnOnce() -> u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let nivel = DENTRO.fetch_add(1, Relaxed);
    // o estado antes da chamada: a replica Rust tem de chegar a ele (o que a rotina gravou antes de
    // chamar, como a acao em +0x198, seria apagado pela foto de depois)
    let antes = if nivel == 0 { fita().as_ref().map(|(reg, _)| crate::comparacao::Memoria::ler(reg)) } else { None };
    // os sorteios de dentro da unidade nao entram no rastro da original (ver `pausar_rastro`)
    let r = if antes.is_some() { crate::aleatorio::pausar_rastro(chamada) } else { chamada() };
    DENTRO.fetch_sub(1, Relaxed);
    if let Some(antes) = antes {
        gravar_chamada(rotina, this, a, r, antes, (Vec::new(), Vec::new()));
    }
    r
}

pub(crate) unsafe fn gravar_chamada(rotina: usize, this: usize, a: [u32; 5], r: u32, antes: crate::comparacao::Memoria, vetores: (Vec<u8>, Vec<u8>)) {
    if let Some((reg, f)) = fita().as_mut() {
        let depois = crate::comparacao::Memoria::ler(reg);
        f.push((rotina, this, a, r, antes, depois, vetores));
    }
}

/// Os bytes dos vetores (endereco, tamanho), em sequencia.
pub(crate) unsafe fn ler_vetores(v: &[(usize, usize)]) -> Vec<u8> {
    let mut b = Vec::new();
    for &(a, n) in v {
        if a != 0 {
            b.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
        }
    }
    b
}

pub(crate) unsafe fn escrever_vetores(v: &[(usize, usize)], bytes: &[u8]) {
    let mut k = 0;
    for &(a, n) in v {
        if a != 0 && k + n <= bytes.len() {
            core::ptr::copy_nonoverlapping(bytes.as_ptr().add(k), a as *mut u8, n);
            k += n;
        }
    }
}

/// Como `chamar_gravando`, para uma unidade que le e escreve vetores por ponteiro (na pilha de quem
/// chama, fora das regioes): grava os bytes das `entradas` antes e das `saidas` depois. A copia Rust
/// pede os seus (`pedir_vetores`) antes de chamar: as entradas sao conferidas e as saidas escritas.
pub(crate) unsafe fn chamar_gravando_vetores(rotina: usize, this: usize, a: [u32; 5], entradas: &[(usize, usize)], saidas: &[(usize, usize)], chamada: impl FnOnce() -> u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let nivel = DENTRO.fetch_add(1, Relaxed);
    let antes = if nivel == 0 { fita().as_ref().map(|(reg, _)| crate::comparacao::Memoria::ler(reg)) } else { None };
    let ent = if antes.is_some() { ler_vetores(entradas) } else { Vec::new() };
    let r = if antes.is_some() { crate::aleatorio::pausar_rastro(chamada) } else { chamada() };
    DENTRO.fetch_sub(1, Relaxed);
    if let Some(antes) = antes {
        let sai = ler_vetores(saidas);
        gravar_chamada(rotina, this, a, r, antes, (ent, sai));
    }
    r
}

/// Os vetores da proxima unidade pedida pela copia Rust: (entradas, saidas).
#[allow(clippy::type_complexity)]
pub(crate) static VETORES_PEDIDOS: std::sync::Mutex<Option<(Vec<(usize, usize)>, Vec<(usize, usize)>)>> = std::sync::Mutex::new(None);

pub(crate) fn pedir_vetores(entradas: &[(usize, usize)], saidas: &[(usize, usize)]) {
    *VETORES_PEDIDOS.lock().unwrap_or_else(|e| e.into_inner()) = Some((entradas.to_vec(), saidas.to_vec()));
}

/// Roda a original gravando as chamadas e a copia Rust reproduzindo-as (`reproduzir`). Devolve as
/// duas respostas, se as regioes ficaram iguais e se a sequencia de chamadas bateu.
/// O que divergiu na ultima comparacao com fita (chamada gravada x pedida, regioes diferentes),
/// para a mensagem de divergencia. So e montado quando ha diferenca.
pub(crate) static DETALHE_FITA: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

pub(crate) fn detalhe_fita() -> String {
    DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

pub(crate) unsafe fn comparar_com_fita<R>(reg: Vec<(usize, usize)>, original: impl FnOnce() -> R, rust: impl FnOnce(&mut dyn FnMut(usize, usize, [u32; 5], [u32; 5]) -> u32) -> R) -> (R, R, bool, bool, usize) {
    // a fita de fora (comparacao aninhada dentro de uma unidade gravada) volta no fim
    let externa = fita().take();
    *fita() = Some((reg.clone(), Vec::new()));
    // contexto proprio: a rotina comparada pode estar dentro de uma chamada gravavel em andamento (a
    // 0x6f5b80 roda dentro da 0x6f09f0 no jogo normal); as chamadas dela contam a partir de zero
    let nivel = DENTRO.swap(0, std::sync::atomic::Ordering::Relaxed);
    DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()).clear();
    *VETORES_PEDIDOS.lock().unwrap_or_else(|e| e.into_inner()) = None;
    let gravadas = core::cell::RefCell::new(Vec::new());
    let sequencia = core::cell::Cell::new(true);
    let rastros = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || {
            let (r, ro) = crate::aleatorio::rastrear(original);
            rastros.borrow_mut().0 = ro;
            *gravadas.borrow_mut() = fita().take().map_or(Vec::new(), |f| f.1);
            r
        },
        || {
            let g = gravadas.borrow();
            let mut prox = g.iter();
            // (rotina, this, argumentos, mascara dos argumentos que a rotina chamada le)
            let (n, rr) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(|| {
                rust(&mut |rotina, this, a, mascara| match prox.next() {
                    Some((gr, gt, ga, resposta, antes, depois, vetores)) if *gr == rotina && *gt == this && (0..5).all(|i| ga[i] & mascara[i] == a[i] & mascara[i]) => {
                        let agora = crate::comparacao::Memoria::ler(&reg);
                        if agora != *antes && sequencia.get() {
                            let difs: Vec<String> = antes.0.iter().zip(agora.0.iter()).filter(|(x, y)| x != y).map(|(x, y)| format!("{:#x}: {:x?} x {:x?}", x.0, x.1, y.1)).collect();
                            *DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()) = format!("estado antes de {:#x} difere: {}", rotina, difs.join("; ").chars().take(300).collect::<String>());
                            sequencia.set(false);
                        }
                        depois.escrever();
                        if !vetores.0.is_empty() || !vetores.1.is_empty() {
                            match VETORES_PEDIDOS.lock().unwrap_or_else(|e| e.into_inner()).take() {
                                Some((ent, sai)) => {
                                    if ler_vetores(&ent) != vetores.0 && sequencia.get() {
                                        *DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()) = format!("entradas de {:#x} diferem", rotina);
                                        sequencia.set(false);
                                    }
                                    escrever_vetores(&sai, &vetores.1);
                                }
                                None => {
                                    if sequencia.get() {
                                        *DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()) = format!("vetores de {:#x} nao pedidos", rotina);
                                    }
                                    sequencia.set(false);
                                }
                            }
                        }
                        *resposta
                    }
                    outra => {
                        if sequencia.get() {
                            *DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()) =
                                format!("gravada {:x?} pedida ({:#x}, {:#x}, {:x?})", outra.map(|c| (c.0, c.1, c.2, c.3)), rotina, this, a);
                        }
                        sequencia.set(false);
                        0
                    }
                })
            }));
            rastros.borrow_mut().1 = rr;
            if let Some(c) = prox.next() {
                if sequencia.get() {
                    *DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()) = format!("sobrou a gravada {:x?}", (c.0, c.1, c.2, c.3));
                }
                sequencia.set(false);
            }
            n
        },
    );
    // os sorteios (pedido, resultado) tambem tem de bater: `sortear(5)` no lugar de `sortear(6)` com o
    // mesmo resultado nao muda a memoria nem o estado do gerador, mas e outra rotina (131a leva)
    if sequencia.get() && rastros.borrow().0 != rastros.borrow().1 {
        *DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()) = "sorteios diferentes".to_string();
        sequencia.set(false);
    }
    if !igual {
        let difs: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a != b).map(|(a, b)| format!("{:#x}: {:x?} x {:x?}", a.0, a.1, b.1)).collect();
        let mut d = DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner());
        if sequencia.get() {
            d.clear();
        }
        d.push_str(&format!(" memoria {}", difs.join("; ").chars().take(300).collect::<String>()));
    }
    if !igual || !sequencia.get() {
        let (ro, rr) = &*rastros.borrow();
        // a partir de pouco antes do primeiro sorteio diferente
        let i = ro.iter().zip(rr.iter()).take_while(|(a, b)| a == b).count().saturating_sub(3);
        DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()).push_str(&format!(" | sorteios ({} x {}, desde o {}) original {:?} rust {:?}", ro.len(), rr.len(), i, &ro[i..ro.len().min(i + 12)], &rr[i..rr.len().min(i + 12)]));
    }
    DENTRO.store(nivel, std::sync::atomic::Ordering::Relaxed);
    *fita() = externa;
    let k = gravadas.borrow().len();
    (r, n, igual, sequencia.get(), k)
}

// Porta de 0x70dfd0 (a jogada, 7 KB: narra, sorteia e mexe no motor e no jogador).
pub(crate) static mut PORTA_70DFD0: usize = 0;
pub(crate) const PROLOGO_70DFD0: Prologo = Prologo { endereco: 0x70dfd0, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] };

pub(crate) unsafe extern "thiscall" fn porta_70dfd0(p: usize, a1: u32, a2: u32, a3: u32, a4: u32) -> u8 {
    let rodar = || {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(PORTA_70DFD0);
        f(p, a1, a2, a3, a4) as u32
    };
    // a comparacao da 0x70dfd0 (129a leva): o desvio dela e esta porta. As comparacoes de quem a
    // chama (0x6cd3d0) a gravam como unidade: a comparacao roda dentro da unidade (aninhada)
    let comparar = || comparar_jogada(p, a1, a2, a3, a4, &rodar) as u32;
    if gravando() {
        return chamar_gravando(0x70dfd0, p, [a1, a2, a3, a4, 0], comparar) as u8;
    }
    comparar() as u8
}

pub(crate) unsafe fn comparar_jogada(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, rodar: &dyn Fn() -> u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || rodar() as u8, |chamar| {
        jogada_com(p, a1 as i16, f32::from_bits(a2), a3 as i8, a4 as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("jogada(forca {}, estilo {}, forte {}) original={} rust={} {} chamadas: {}", a1 as i16, a3 as i8, a4 as u8, r, rr, k, detalhe_fita()));
    r
}

// Porta de 0x6f09f0 (reposiciona o jogador, 13 KB, sorteia e mexe em tudo em volta).


/// 0x70dfd0 recebe os quatro argumentos inteiros.
pub(crate) const MASCARA_70DFD0: [u32; 5] = [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0];
/// 0x6f09f0 e 0x6f46b0 leem x e y como bytes (o resto do registro empilhado e lixo).
pub(crate) const MASCARA_XY: [u32; 5] = [0xff, 0xff, u32::MAX, u32::MAX, u32::MAX];

// ---------- octogesima setima leva: forca do lado (0x6bbb60) ----------

/// { 0x6bbb60 } (ret 4, match_eng.cpp): a forca do lado (palavra +0x1ce + 2 x lado do motor,
/// recomeca do zero, assim como o byte +0xaf do lado). Para cada um dos 20 ativos: -500 quando o bit
/// da casa (0x717410, 0x5b5a60) ja apareceu (a mascara soma, nao junta); com o contrato (0xae1050:
/// 0x4cf5a0, guardado em 0xae1080; 0x4cf5f0 com o clube dele) e tres sorteios favoraveis (20 contra
/// +0x2f da ficha, 7000 contra a palavra +0xb, 6 contra +0x5b da pessoa), a palavra +0xb acima de
/// +0x80 do clube - 750 e os bits 0x17a0b do contrato, perde 3x a nota (0x6ebd70) se for o designado
/// da tatica (0x897590), senao 2x ou 1x (sorteio pelo +0x5b). Quem nao e o designado soma a nota. No
/// fim, o desempenho de cada ativo (0x6eecb0 com marca).
pub(crate) unsafe extern "thiscall" fn forca_do_lado(this: usize, lado: i8) {
    let l = lado as isize;
    let forca = (this as isize + 0x1ce + 2 * l) as *mut i16;
    let soma = move |v: i32| *forca = (*forca).wrapping_add(v as i16);
    *forca = 0;
    *((this as isize + 0xaf + l) as *mut u8) = 0;
    let casa: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(FN_MATCH_PLAYER_GET_POSITION);
    let bit: unsafe extern "stdcall" fn(u32) -> u32 = core::mem::transmute(FN_FORMATION_POSITION_TO_SHAPE_BIT);
    let contrato_principal: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let nota: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_CALC_INFLUENCE_SCORE);
    let nota = |j: usize| nota(j) as i8 as i32;
    let vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
    let designado = || {
        let t = *((this + 0x4792) as *const usize);
        let (c, fl) = if lado != 0 { (*((t + 0x20) as *const usize), (byte(t, 0x41) == 0) as u32) } else { (*((t + 0x1c) as *const usize), (byte(t, 0x40) == 0) as u32) };
        let k = vaga(*(0xde1d08 as *const usize), c, fl, 0) as i32;
        (this as isize + 0x4796 + 0x1be * (k + 20 * lado as i32) as isize) as usize
    };
    let mut mascara = 0u32;
    for k in 0..20usize {
        let j = jogador_do_bloco(this, lado, k);
        if byte(j, 0x19) < 0 {
            continue;
        }
        let bt = bit(casa(j) as u32);
        if mascara & bt != 0 {
            soma(-500);
        }
        mascara = mascara.wrapping_add(bt);
        let pessoa = *((j + 0x69) as *const usize);
        let principal = contrato_principal(*(0xae1050 as *const usize), pessoa);
        *(0xae1080 as *mut usize) = principal;
        let clube = if principal == 0 { 0 } else { (*(0xae23b4 as *const usize)).wrapping_add((*((principal + 4) as *const i32) as usize).wrapping_mul(581)) };
        let c = contrato(*(0xae1050 as *const usize), pessoa, clube);
        if c != 0 && clube != 0 {
            let ficha = *((j + 0x6d) as *const usize);
            if (byte(ficha, 0x2f) as i32) > sortear(20)
                && (*((ficha + 0xb) as *const i16) as i32) > sortear(7000)
                && (byte(pessoa, 0x5b) as i32) < sortear(6)
                && *((ficha + 0xb) as *const i16) > (*((clube + 0x80) as *const i16)).wrapping_sub(750)
                && core::ptr::read_unaligned((c + 0x45) as *const u32) & 0x17a0b != 0
            {
                if j == designado() {
                    soma(-3 * nota(j));
                } else if sortear(byte(pessoa, 0x5b) as i32) == 0 {
                    soma(-2 * nota(j));
                } else {
                    soma(-nota(j));
                }
            }
        }
        if j != designado() {
            soma(nota(j));
        }
    }
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_CALC_RATINGS);
    for k in 0..20usize {
        let j = jogador_do_bloco(this, lado, k);
        if byte(j, 0x19) >= 0 {
            desempenho(j, 1);
        }
    }
}

// ---------- septuagesima setima leva: proximo da lista, palavra +0xc, arredondamento, nacoes, marcadores ----------

/// returns the node's +0 and advances to its +6
/// Original: `next_list_item` (0x68d1b0, cursor at +0xa (unaligned), thiscall).
///
/// [pt] { 0x68d1b0 }: o proximo item da lista (cursor em +0xa: devolve o +0 do no e avanca para o +6), ou 0.
pub(crate) unsafe extern "thiscall" fn proximo_da_lista(this: usize) -> u32 {
    let no = core::ptr::read_unaligned((this + 0xa) as *const usize);
    if no == 0 {
        return 0;
    }
    core::ptr::write_unaligned((this + 0xa) as *mut usize, core::ptr::read_unaligned((no + 6) as *const usize));
    core::ptr::read_unaligned(no as *const u32)
}

/// { 0x62c3d0 } (ret 4, injury.cpp): a palavra com sinal +0xc do registro de 31 bytes da pessoa `id`
/// em [this]; id a partir de 0xae2364: 0.
pub(crate) unsafe extern "thiscall" fn palavra_0c(this: usize, id: i32) -> i32 {
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    core::ptr::read_unaligned((*(this as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *const i16) as i32
}

/// { 0x4b0870 } (comp_util.cpp): `v` menos o resto por 10 da palavra baixa, quando ela (com sinal)
/// e menor que 1000.
pub(crate) unsafe extern "C" fn arredondar_abaixo_de_1000(v: i32) -> i32 {
    let w = v as i16;
    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })
}

/// { 0x7e6f20 } (stdcall): a nacao (id em +0) e uma das seis de 0x9cf794, 0x9cf684, 0x9cf798,
/// 0x9cf6b0, 0x9cf5d8 e 0x9cf548?
pub(crate) unsafe extern "stdcall" fn nacao_do_grupo(nacao: usize) -> u32 {
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const u32);
    [KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any(|&a| *(a as *const u32) == id) as u32
}

/// { 0x6d59b0 } (ret 4): quantos dos 20 jogadores do outro lado (ativos, +0x19 >= 0) marcam
/// `alvo` individualmente (0x7160f0).
pub(crate) unsafe extern "thiscall" fn quantos_marcam(this: usize, alvo: usize) -> u8 {
    let marcado: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_MATCH_PLAYER_FIND_MAN_MARKING_TARGET);
    let outro = (lado(alvo) == 0) as i8;
    (0..20).filter(|&k| {
        let j = jogador_do_bloco(this, outro, k);
        *((j + 0x19) as *const i8) >= 0 && marcado(j) == alvo
    }).count() as u8
}

/// { 0x6d5a10 } (ret 4): o primeiro dos 20 jogadores do outro lado (ativos) que marca `alvo`, ou 0.
pub(crate) unsafe extern "thiscall" fn quem_marca(this: usize, alvo: usize) -> usize {
    let marcado: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_MATCH_PLAYER_FIND_MAN_MARKING_TARGET);
    let outro = (lado(alvo) == 0) as i8;
    for k in 0..20 {
        let j = jogador_do_bloco(this, outro, k);
        if *((j + 0x19) as *const i8) >= 0 && marcado(j) == alvo {
            return j;
        }
    }
    0
}

// ---------- septuagesima primeira leva: escolha do companheiro para a jogada (0x6c6a30, 7 KB) ----------

/// Menor distancia (tabela) de um jogador ativo do bloco `lado` (fora os de `tabela_a` com o bit
/// 1) ao ponto (x, y), comecando de 5000; 0 se algum esta a distancia 0.
pub(crate) unsafe fn menor_distancia_livre(m: usize, lado: usize, x: i8, y: i8) -> f32 {
    let mut menor = 5000.0f32;
    for k in 0..20 {
        let q = m + 0x4796 + 8920 * lado + 0x1be * k;
        if byte(q, 0x19) < 0 || tabela_do_jogador(q) & 1 != 0 {
            continue;
        }
        let d = distancia_ao_ponto(q, x, y);
        if d as f64 == f64_em(0x9689a0) || d.is_nan() {
            return 0.0;
        }
        if d < menor || d.is_nan() {
            menor = d;
        }
    }
    menor
}

/// { 0x6c6a30 } (match_eng.cpp): escolhe o companheiro e a jogada (passe, cruzamento, lancamento,
/// tabela...) para quem esta com a bola (0xf582), a partir da casa (bx, by). Para cada colega ativo
/// soma notas pelo espaco (adversario mais proximo dele e do ponto para onde ele pode se deslocar,
/// 0x6bce80), pelo papel na formacao, pela posicao em relacao a linha de impedimento (0x8eaf/0x8eb0),
/// pelo cerco, pelas marcas taticas (0x7163c0), pela preferencia do tecnico (0x897dc0) e por
/// gostos pessoais (0x547840/0x547810); depois escolhe o tipo de jogada pela distancia ate o alvo
/// (ou testa o tipo `pedido`) e fica com a maior nota acima de `*melhor`. Escreve o tipo, se o alvo
/// se desloca e a casa alvo; devolve o colega (ou 0).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn escolher_companheiro(
    m: usize,
    bx: i8,
    by: i8,
    tipo_out: *mut i8,
    moveu_out: *mut u8,
    x_out: *mut i8,
    y_out: *mut i8,
    melhor: *mut i32,
    pedido: i8,
) -> usize {
    type F1 = unsafe extern "thiscall" fn(usize) -> u8;
    let t = |a: usize| core::mem::transmute::<usize, F1>(a);
    let marcas: unsafe extern "thiscall" fn(usize) -> u64 = core::mem::transmute(FN_MATCH_PLAYER_GET_INSTRUCTIONS);
    let consegue: unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_CELL_IN_VIEW);
    let casa_livre: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_CHOOSE_MOVE_CELL);
    let classificar: unsafe extern "thiscall" fn(usize, i16, i8, i8, i8, i8, f32, *mut u16) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_CHOOSE_KICKING_FOOT);
    let alcanca_f: unsafe extern "stdcall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_CAN_RUN_INTO_CENTRAL_BOX);
    let espaco: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_HAS_FREE_CELL_AHEAD);
    let caminho: unsafe extern "thiscall" fn(usize, usize, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_PATH_AHEAD_CLEAR);
    let pela_ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MATCH_MAN_FIND_MATCH_PLAYER);
    let preferido: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_PLAYMAKER);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let ao_gol: unsafe extern "thiscall" fn(usize) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_OPPONENT_GOAL);

    let mut res = 0usize;
    *((m + 0x8eb2) as *mut u8) = 0;
    let c = *((m + 0xf582) as *const usize);
    let mut fator_nacao = f32::from_bits(0x3e4ccccd);
    let nac = *((*((c + 0x69) as *const usize) + 0x1a) as *const usize);
    if nac != 0 {
        let id = *(nac as *const u32);
        let g = |a: usize| *(a as *const u32) == id;
        if g(KEY_NATION_ENGLAND) || g(KEY_NATION_SCOTLAND) || g(KEY_NATION_WALES) || g(KEY_NATION_NORTHERN_IRELAND) || g(KEY_NATION_REPUBLIC_OF_IRELAND) || g(KEY_NATION_NORWAY) {
            fator_nacao = 1.0;
        } else if g(KEY_NATION_SWEDEN) || g(KEY_NATION_DENMARK) || g(KEY_NATION_GERMANY) {
            fator_nacao = f32::from_bits(0x3f19999a);
        }
    }
    let r1 = sortear(500);
    let base = (*((c + 0x29) as *const i16) as i32 - 1000) * 3 - r1;
    *melhor = base.wrapping_add(sortear(500));
    if t(0x70dfa0)(c) == 0 && mais_perto_do_gol(m, c) == 1 && t(0x6f4650)(c) != 0 {
        return 0;
    }
    let ls = byte(m, 0x8eae);
    let na_area = |x: i8, y: i8, s: i8| (2..=6).contains(&x) && if s == 0 { (10..=11).contains(&y) } else { (0..=1).contains(&y) };
    let area_atacada = na_area(bx, by, ls); // [0x17]
    let area_defendida = na_area(bx, by, if ls == 1 { 0 } else { 1 }); // [0x16]
    let adv = (ls == 0) as usize;
    let d_bola = menor_distancia_livre(m, adv, byte(c, 0x102), byte(c, 0x103));
    let papel_de = |q: usize| -> u16 {
        let pos = byte(q, 0x19);
        if (0..=10).contains(&pos) {
            *((*((m + 0x8ebc + 4 * lado(q) as usize) as *const usize) + 2 * pos as usize) as *const u16)
        } else {
            0
        }
    };
    let pf = |x: i32| ftol(x as f64);
    let _ = pf;
    for k in 0..20usize {
        let p = m + 0x4796 + 8920 * ls as usize + 0x1be * k;
        if byte(p, 0x19) < 0 || p == c {
            continue;
        }
        if papel_de(c) == 1 && ((ls == 0 && by > byte(p, 0x103)) || (ls == 1 && by < byte(p, 0x103))) {
            continue;
        }
        let (px, py) = (byte(p, 0x102), byte(p, 0x103));
        let mut d58 = menor_distancia_livre(m, adv, px, py);
        let mut s74 = 0u8;
        if pedido != 0x36 && pedido != 0x37 && consegue(c, px, py, 1, d_bola, d58, &mut s74) == 0 {
            continue;
        }
        let papel = papel_de(p);
        let mut v34 = ftol(f32_em(0x968db0) as f64 - d58 as f64 * d58 as f64 * f32_em(0x969580) as f64);
        if t(0x70df40)(p) != 0 {
            v34 = v34.wrapping_sub(1500);
        } else if t(0x70dfa0)(p) != 0 {
            let fundo = if ls == 0 { py < 3 } else { py > 8 };
            v34 = v34.wrapping_add(if fundo { 10000 } else { 3000 });
        }
        let mut v1c: i32 = if papel == 1 {
            0
        } else {
            ftol((f32_em(p + 0x4d) as f64 + *((p + 0x35) as *const i16) as f64 - f32_em(0x968da8) as f64) * f32_em(0x96d22c) as f64)
        };
        if t(0x70dfa0)(p) == 0 {
            let mp = mais_perto_do_gol(m, p);
            let passo = match mp {
                1 => 2000,
                2 => 4000,
                _ => 0,
            };
            if passo != 0 {
                v34 = v34.wrapping_sub(passo);
                v1c = v1c.wrapping_add(passo);
                if (marcas(c) >> 32) & 1 != 0 {
                    v1c = v1c.wrapping_add(passo);
                    v34 = v34.wrapping_sub(passo);
                }
            } else if (marcas(c) >> 32) & 1 != 0 {
                v1c = v1c.wrapping_add(500);
                v34 = v34.wrapping_sub(500);
            }
        }
        let partida = *((m + 0x4792) as *const usize);
        let (mut v44, mut v4c, mut v50) = (0i32, 0i32, 0i32);
        let comp = *((partida + 0x14) as *const usize);
        let mut edi: i32 = if comp != 0 { (*((comp + 0x69) as *const i16) as i32 - 12) * 25 } else { 0 };
        let mut v30 = edi.wrapping_mul(-4);
        let mut v38 = 0i32;
        let lp = lado(p);
        let (marca_p, clube_p) = if lp != 0 {
            (byte(partida, 0x41), *((partida + 0x20) as *const usize))
        } else {
            (byte(partida, 0x40), *((partida + 0x1c) as *const usize))
        };
        let tabela = *(0xde1d08 as *const usize);
        let ficha_p = *((p + 0x69) as *const usize);
        if ficha_p == preferido(tabela, clube_p, 0, (marca_p == 0) as u32) {
            v1c = v1c.wrapping_add(1000);
        } else if ficha_p == preferido(tabela, clube_p, 1, (marca_p == 0) as u32) {
            let pl0 = preferido(tabela, clube_p, 0, (marca_p == 0) as u32);
            let q = pela_ficha(m + 0x9032 + 208 * lado(c) as usize, pl0);
            if q == 0 || byte(q, 0x19) < 0 {
                v1c = v1c.wrapping_add(1000);
            }
        } else {
            let a = f32_em(p + 0xf1) as f64;
            let r = ftol(a * a * f64_em(0x96d410));
            edi = edi.wrapping_sub(r);
            v38 = r.wrapping_neg();
        }
        let faixa_l: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
        if faixa_l(p, (lp == 0) as u8) != 0 {
            let b = ftol(f32_em(p + 0xe5) as f64 * f32_em(p + 0xc1) as f64 * f64_em(0x96d410));
            let a = f32_em(p + 0x99) as f64;
            let d = ftol(a * a * f64_em(0x9688a0)).wrapping_sub(b);
            edi = edi.wrapping_add(d);
            v38 = v38.wrapping_add(d);
        }
        if t(0x70df40)(c) == 0 {
            let mc = marcas(c);
            let nivel = if mc & 1 != 0 {
                let f = *((m + 6371 * ls as usize + 0x9766) as *const u32);
                if f & 8 != 0 { 3 } else if f & 4 != 0 { 2 } else if f & 2 != 0 { 1 } else { 0 }
            } else if marcas(c) & 0x10 != 0 {
                3
            } else if marcas(c) & 8 != 0 {
                2
            } else if marcas(c) & 4 != 0 {
                1
            } else {
                0
            };
            match nivel {
                3 => {
                    edi = edi.wrapping_sub(1000);
                    v30 = v30.wrapping_add(1000);
                }
                2 => {
                    edi = edi.wrapping_sub(750);
                    v30 = v30.wrapping_add(500);
                }
                1 => {
                    edi = edi.wrapping_add(250);
                    v30 = v30.wrapping_sub(250);
                }
                _ => {}
            }
        }
        let mut v3c = edi;
        // deslocamento do alvo
        let (mut tx, mut ty, mut moveu) = (px, py, 0u8);
        if papel != 1 {
            let prod = byte(p, 0x105) as i32 * byte(p, 0x104) as i32 * byte(c, 0x10b) as i32 * 38;
            let lim = prod as f64 * f64_em(0x967880);
            let r = sortear(v34.wrapping_mul(6));
            let (mut x78, mut y77) = (0i8, 0i8);
            if !((r as f64) > lim) && casa_livre(m, p, &mut x78, &mut y77, 0) != 0 {
                x78 = x78.clamp(0, 8);
                y77 = y77.clamp(0, 11);
                if !(x78 == px && y77 == py) {
                    tx = x78;
                    ty = y77;
                    moveu = 1;
                    d58 = menor_distancia_livre(m, adv, tx, ty);
                    if tabela_do_jogador(p) & 0x40 != 0 {
                        v1c = v1c.wrapping_add(byte(p, 0x104) as i32 * byte(p, 0x105) as i32 * 50);
                    }
                }
            }
        }
        // impedimento: casas alem da linha (0x8eaf/0x8eb0)
        let mut k7c: i8 = if lp == 1 { byte(m, 0x8eb0).wrapping_sub(py) } else { py.wrapping_sub(byte(m, 0x8eaf)) }.wrapping_add(1);
        if k7c > -2
            && *((m + 6371 * (lp == 0) as usize + 0x9766) as *const u32) & 0x400 != 0
            && byte(c, 0x106) != 0
        {
            let b = byte(p, 0x107) as i32;
            if b < sortear(0x32) {
                k7c = k7c.wrapping_add(1);
            }
        }
        if pedido == 0x36 {
            k7c = -4;
        } else if k7c >= 0 {
            k7c = k7c.wrapping_mul(k7c).wrapping_add(1);
            let q = k7c as i32;
            let b = byte(p, 0x107) as i32;
            if b < sortear((q * 5 + 5) * 2) {
                v1c = v1c.wrapping_sub((byte(c, 0x10b) as i32 * 400 + 500).wrapping_mul(q).wrapping_mul(q));
            }
        }
        let alvo_area = na_area(tx, ty, ls); // [0x11]
        let alvo_area_def = na_area(tx, ty, if ls == 1 { 0 } else { 1 }); // [0x12]
        let (by_i, ty_i) = (by as i32, ty as i32);
        let d60 = by_i - ty_i;
        let d68 = distancia_ou_mil(bx as i32 - tx as i32, d60);
        // cerco do alvo
        let mut n = 0i8;
        for kk in 0..20 {
            let q = m + 0x4796 + 8920 * adv + 0x1be * kk;
            if byte(q, 0x19) < 0 || tabela_do_jogador(q) == 1 {
                continue;
            }
            let d = distancia_ao_ponto(q, tx, ty) as f64;
            if d == f64_em(0x9689a0) || d.is_nan() {
                n = n.wrapping_add(3);
            } else if !(d > f64_em(0x968940)) {
                n = n.wrapping_add(1);
            }
        }
        let n_i = n as i32;
        v1c = v1c.wrapping_add((5 - n_i) * 400);
        if alvo_area && n < 3 && k7c < 1 {
            v1c = v1c.wrapping_add(ftol(pow_do_jogo((3 - n_i) as f64, f64_em(0x968930)) * f64_em(0x9688a8)));
        }
        let mut s72 = 0u16;
        if byte(c, 0x2c) != classificar(c, 0x5d, bx, by, tx, ty, d68, &mut s72) as i8 {
            v1c = v1c.wrapping_sub(sortear(byte(c, 0x2d) as i32 * 400));
        }
        if pedido == 0x34 || pedido == 0x37 {
            let soma = |extra: f32| {
                f32_em(p + 0x9d) as f64 * f32_em(0x968960) as f64 + f32_em(p + 0x99) as f64 * f32_em(0x969560) as f64
                    + f32_em(p + 0x8d) as f64 * f32_em(0x969560) as f64
                    + extra as f64
            };
            let val = if alvo_area {
                Some(f32_em(0x96d40c) as f64 - ao_gol(p) as f64 * f32_em(0x96d2dc) as f64 + f32_em(p + 0xad) as f64 * f32_em(0x968960) as f64)
            } else if (d68 as f64) < f64_em(0x968930) {
                if n == 0 {
                    Some(soma(f32_em(0x96d408)))
                } else if (n_i as f64) < f64_em(0x968930) {
                    Some(soma(f32_em(0x96d404)))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(v) = val {
                v1c = v1c.wrapping_add(ftol(v));
            }
        }
        if !(d58 as f64 <= f64_em(0x968940) || d58.is_nan()) && !alvo_area {
            let e = byte(c, 0x106) as i32 + 5;
            v30 = v30.wrapping_add(ftol((-700 * e) as f64 / fator_nacao as f64));
            v44 = ftol((-500 * e) as f64 / fator_nacao as f64);
        } else if moveu == 0 {
            let x = (30 - byte(c, 0x10b) as i32) as f64 * (f32_em(0x968a78) as f64 - f32_em(p + 0xad) as f64) * fator_nacao as f64;
            v30 = v30.wrapping_add(ftol(x * f32_em(0x96d400) as f64));
            v44 = ftol(x * f32_em(0x96d3fc) as f64);
        }
        if papel == 1 {
            if t(0x70dfa0)(c) == 0 && sortear(100000) != 0 {
                continue;
            }
            let v = byte(c, 0x106) as i32;
            if t(0x70def0)(c) == 0 {
                v1c = v1c.wrapping_add(ftol((-1000 * v) as f64 * (d58 as f64 + f64_em(0x967880))));
            }
            v44 = v44.wrapping_add(-1000 - 2200 * v);
            v30 = v30.wrapping_add(-2000 - 5000 * v);
            v38 = v38.wrapping_add(-1500 * v);
        }
        if papel_de(c) == 1 {
            v1c = v1c.wrapping_sub(1000);
        }
        if area_defendida {
            if (d58 as f64) < f64_em(0x968950) && tabela_do_jogador(c) == 1 && t(0x70dfa0)(p) != 0 {
                v1c = v1c.wrapping_sub(2000);
            }
            if alvo_area_def {
                v1c = v1c.wrapping_add(ftol(byte(c, 0x106) as f64 * d58 as f64 * d58 as f64 * f32_em(0x96d3f8) as f64));
            }
        }
        if alvo_area_def {
            v1c = v1c.wrapping_sub(ftol((byte(c, 0x106) as i32 * 32 + 200) as f64 * d58 as f64));
        }
        let alcanca_r = alcanca_f(c, tx, ty);
        if alcanca_r != 0 {
            let v = byte(c, 0x106) as f64;
            let a = f32_em(p + 0xad) as f64 * f32_em(0x968960) as f64;
            let b = f32_em(c + 0x8d) as f64 * f32_em(0x968960) as f64;
            v4c = ftol((a + f32_em(0x96d1a8) as f64) * v * f32_em(0x96afc4) as f64 + b);
            v50 = ftol((f32_em(0x96d408) as f64 - a) * v * f32_em(0x96afc4) as f64 + b);
            if (marcas(c) >> 32) & 4 != 0 {
                let w = ftol(f32_em(c + 0xe1) as f64 * f32_em(0x968960) as f64 + f32_em(0x96d1a8) as f64);
                v4c = v4c.wrapping_add(w);
                v50 = v50.wrapping_add(w);
            }
            v30 = v30.wrapping_sub(2000);
            v38 = v38.wrapping_sub(2000);
            v3c = v3c.wrapping_sub(2000);
        }
        if marcas(c) & 0x40 != 0 && ((ls == 0 && px <= 2) || (ls == 1 && px >= 6)) {
            v1c = v1c.wrapping_add(1000);
        }
        if marcas(c) & 0x100 != 0 && ((ls == 0 && px >= 6) || (ls == 1 && px <= 2)) {
            v1c = v1c.wrapping_add(1000);
        }
        if marcas(c) & 0x80 != 0 && t(0x70df20)(p) == 0 && alcanca_r == 0 {
            v1c = v1c.wrapping_add(1000);
        }
        // proximidade da area (desvios do original em 0x6c7c95..0x6c7d5f)
        {
            let fundo = |y: i8| if ls == 1 { y < 3 } else if ls == 0 { y > 8 } else { false };
            let menos = |v1c: &mut i32, d58: &mut f32, q: i32| {
                *d58 = 0.0;
                *v1c = v1c.wrapping_sub(q);
            };
            if area_atacada && !alvo_area {
                menos(&mut v1c, &mut d58, 4000);
            } else if !area_atacada && alvo_area {
                d58 = (d58 as f64 * d58 as f64 + f64_em(0x967890)) as f32;
            } else {
                #[derive(Clone, Copy)]
                enum L {
                    Cf3,
                    Cce,
                    Cec,
                    Cfc,
                    D20,
                    D25,
                    D43,
                    D4a,
                    Fim,
                }
                let mut l = if bx <= 2 {
                    L::Cf3
                } else if area_atacada {
                    L::Cce
                } else if fundo(by) {
                    if bx < 6 { L::Cfc } else { L::Cce }
                } else {
                    L::Cf3
                };
                loop {
                    l = match l {
                        L::Cce => if fundo(ty) { L::Cec } else { L::Cf3 },
                        L::Cec => if tx < 2 { L::D4a } else { L::Cf3 },
                        L::Cf3 => {
                            if bx >= 6 {
                                L::Fim
                            } else if area_atacada {
                                L::D25
                            } else {
                                L::Cfc
                            }
                        }
                        L::Cfc => if fundo(by) { L::D20 } else { L::Fim },
                        L::D20 => if bx > 2 { L::Fim } else { L::D25 },
                        L::D25 => if fundo(ty) { L::D43 } else { L::Fim },
                        L::D43 => if tx <= 6 { L::Fim } else { L::D4a },
                        L::D4a => {
                            menos(&mut v1c, &mut d58, 2000);
                            L::Fim
                        }
                        L::Fim => break,
                    };
                }
            }
        }
        if ((ls == 0 && by >= ty) || (ls == 1 && by <= ty)) && t(0x70dfa0)(p) != 0 {
            v30 = v30.wrapping_sub(5000);
            v38 = v38.wrapping_sub(3000);
            v44 = v44.wrapping_sub(3000);
        }
        let zera = |v1c: &mut i32, d58: &mut f32, v: f64, k: f32| {
            *d58 = v as f32;
            if v < f64_em(0x9689a0) || v.is_nan() {
                *v1c = v1c.wrapping_add(ftol(*d58 as f64 * *d58 as f64 * k as f64));
                *d58 = 0.0;
            }
        };
        if ls == 0 && by > ty {
            let v = d58 as f64 - (d60 as f64 - f64_em(0x968968));
            zera(&mut v1c, &mut d58, v, f32_em(0x96d3f4));
        } else if ls == 1 && by < ty {
            let v = d58 as f64 - ((ty_i - by_i) as f64 - f64_em(0x968968));
            zera(&mut v1c, &mut d58, v, f32_em(0x96d3f4));
        } else {
            let mut cnt = 0i8;
            for kk in 0..20 {
                let q = m + 0x4796 + 8920 * ls as usize + 0x1be * kk;
                if byte(q, 0x19) >= 0 && ((ls == 1 && byte(q, 0x103) < ty) || (ls == 0 && byte(q, 0x103) > ty)) {
                    cnt = cnt.wrapping_add(1);
                }
            }
            if cnt == 0 {
                d58 = (d58 as f64 + d58 as f64 + f64_em(0x967890)) as f32;
            } else if !(2..=6).contains(&tx) && t(0x70dfa0)(p) == 0 && (marcas(p) >> 32) & 4 != 0 {
                d58 = (d58 as f64 * f64_em(0x968940) + f64_em(0x967890)) as f32;
            }
        }
        if k7c >= 0 {
            let v = d58 as f64 - k7c as f64;
            zera(&mut v1c, &mut d58, v, f32_em(0x96d3f0));
        }
        v1c = v1c.wrapping_add(ftol(d58 as f64 * d58 as f64 * f32_em(0x968960) as f64));
        let perto = (ls == 0 && by >= ty) || (ls == 1 && by <= ty);
        let mut c2 = true;
        if perto && (d58 as f64) < f64_em(0x96d3e8) && !(if ls == 1 { ty < 3 } else if ls == 0 { ty > 8 } else { false })
            && !(byte(c, 0x106) == 0 && byte(c, 0x10b) == 0)
        {
            c2 = false;
            let s: i16 = if t(0x70def0)(p) != 0 {
                ftol(f32_em(0x96d3e4) as f64 - d58 as f64 * f32_em(0x96d208) as f64) as i16
            } else {
                ftol(f32_em(0x96d2dc) as f64 - d58 as f64 * f32_em(0x96d210) as f64) as i16
            };
            if ls == 0 {
                v1c = v1c.wrapping_sub((d60 + 1).wrapping_mul(s as i32));
            } else if ls == 1 {
                v1c = v1c.wrapping_sub((ty_i - by_i + 1).wrapping_mul(s as i32));
            }
        }
        if c2 {
            if d58 < f32_em(0x9689a8) && t(0x70dfa0)(p) != 0 {
                v1c = v1c.wrapping_sub(1500);
            } else if papel != 1 {
                let v10 = byte(c, 0x10b) as i32 + 10;
                v1c = v1c.wrapping_add(ftol(v10 as f64 * d58 as f64 * d58 as f64));
                let fundo_t = if ls == 1 { ty < 3 } else if ls == 0 { ty > 8 } else { false };
                if fundo_t {
                    if espaco(m, p) != 0 {
                        v1c = v1c.wrapping_add(v10 * 20);
                    } else if caminho(m, p, 0) != 0 {
                        v1c = v1c.wrapping_add(v10 * 50);
                    }
                } else if t(0x70dfa0)(p) == 0 {
                    if espaco(m, p) != 0 {
                        v1c = v1c.wrapping_add(80 + byte(c, 0x10b) as i32 * 8);
                    } else if caminho(m, p, 0) != 0 {
                        v1c = v1c.wrapping_add(v10 * 20);
                    }
                }
                if (ls == 0 && ty > by) || (ls == 1 && ty < by) {
                    v1c = v1c.wrapping_add(ftol(v10 as f64 * d58 as f64 * d58 as f64));
                }
            }
        }
        let g = byte(p, 0x2b) as i32;
        if g > 1 {
            v1c = v1c.wrapping_add(-50 * (byte(c, 0x10b) as i32 + 4) * g * g);
        }
        let s74r = sortear(s74 as i8 as i32) as i8;
        let q = s74r as i32 * s74r as i32;
        v44 = v44.wrapping_sub(155 * q);
        let v3c2 = v3c.wrapping_sub(500 * q);
        v38 = v38.wrapping_sub(155 * q);
        let v30b = v30.wrapping_sub(25 * q);
        let mut base = v1c;
        if sortear(byte(*((c + 0x69) as *const usize), 0x5b) as i32 + 1) == 0 {
            let (fp, fc) = (*((p + 0x69) as *const usize), *((c + 0x69) as *const usize));
            if nao_gosta(fp, fc) != 0 {
                continue;
            }
            if gosta(fp, fc) != 0 {
                base = base.wrapping_add(1000);
            }
        }
        // tipo de jogada e nota final
        let mut tipo: i8 = -1;
        let mut nota: i32 = -100000;
        let lim = f32_em(0x968990) as f64 - (f32_em(0x96d3e0) as f64 - f32_em(c + 0xd9) as f64) * f64_em(0x96a7a8);
        let d = d68 as f64;
        let entre = |hi: f64, lo: f64| !(d > hi || d.is_nan()) && !(d < lo);
        if pedido < 0 {
            let pa = byte(m, 0x8ea9);
            if pa == 4 {
                if d > f32_em(0x9689a8) as f64 {
                    continue;
                }
                tipo = 0x3d;
                nota = base;
            } else {
                if !(lim < d) && !(d < f32_em(0x96d228) as f64) {
                    nota = v30b;
                    tipo = 0x1e;
                }
                if alcanca_r != 0 {
                    if entre(f32_em(0x968990) as f64, f32_em(0x968e70) as f64) && v4c > nota {
                        nota = v4c;
                        tipo = 8;
                    }
                    if entre(f32_em(0x968990) as f64, f32_em(0x968e70) as f64) && v50 > nota {
                        nota = v50;
                        tipo = 7;
                    }
                }
                if !(d > f32_em(0x9689a8) as f64 || d.is_nan()) && v3c2 > nota && pa < 3 {
                    nota = v3c2;
                    tipo = 0xb;
                }
                if entre(f32_em(0x96d228) as f64, f32_em(0x969584) as f64) && v38 > nota {
                    nota = v38;
                    tipo = 0x3c;
                }
                if entre(f32_em(0x969584) as f64, f64_em(0x968940)) && v44 > nota {
                    nota = v44;
                    tipo = 0x1f;
                }
                nota = nota.wrapping_add(base);
            }
        } else {
            tipo = pedido;
            let fora = -100000i32;
            nota = match pedido {
                6 | 8 => if entre(f32_em(0x968990) as f64, f32_em(0x968e70) as f64) { v4c.wrapping_add(base) } else { fora },
                7 => if entre(f32_em(0x968990) as f64, f32_em(0x968e70) as f64) { v50.wrapping_add(base) } else { fora },
                0xb => if d > f32_em(0x9689a8) as f64 || d.is_nan() { fora } else { base.wrapping_add(v3c2) },
                0x1e | 0x5a => if lim < d || d < f32_em(0x96d228) as f64 { fora } else { v30b.wrapping_add(base) },
                0x1f => if entre(f32_em(0x969584) as f64, f64_em(0x968940)) { v44.wrapping_add(base) } else { fora },
                0x34 => {
                    if entre(f64_em(0x96d298), f64_em(0x968930)) && tx > 1 && tx < 7 && !(3..=8).contains(&ty) {
                        v38.wrapping_add(base)
                    } else {
                        base.wrapping_add(v3c2)
                    }
                }
                0x36 => {
                    if (f32_em(c + 0xe9) as f64 * f64_em(0x967880) + f32_em(0x968e70) as f64) < d {
                        fora
                    } else {
                        base
                    }
                }
                0x37 => if lim < d || d < f32_em(0x96d228) as f64 { base.wrapping_add(v3c2) } else { v30b.wrapping_add(base) },
                0x3c => if entre(f32_em(0x96d228) as f64, f32_em(0x969584) as f64) { v38.wrapping_add(base) } else { fora },
                0x3d => if d > f32_em(0x9689a8) as f64 || d.is_nan() { fora } else { base },
                _ => base,
            };
        }
        if tipo >= 0 && nota > *melhor {
            *melhor = nota;
            *tipo_out = tipo;
            res = p;
            *moveu_out = moveu;
            *x_out = tx;
            *y_out = ty;
        }
    }
    res
}

// ---------- sexagesima setima leva: ordem dos jogadores no lance ----------

/// the list of active players (+0x19 ≥ 0) at +0x8f7e, count at +0x8eb8. For each slot 0..20 draws (0x90cfc0(2)) which
/// side goes first. Sorts with the CRT `qsort` (0x945333) and the game's own compare (0x6bd840, which reads the engine
/// at 0xb67a64)
/// Original: `player_order` (0x6bd760, match_eng.cpp, thiscall).
///
/// [pt] { 0x6bd760 } (match_eng.cpp): a lista dos jogadores ativos (+0x19 >= 0) em +0x8f7e (contagem em
/// +0x8eb8): para cada vaga 0..20 sorteia qual lado entra primeiro (0x90cfc0(2)) e poe o daquele lado
/// e depois o do outro. Ordena com o `qsort` da CRT (0x945333) e a comparacao do proprio jogo
/// (0x6bd840, que le o motor em 0xb67a64).
pub(crate) unsafe extern "thiscall" fn ordem_dos_jogadores(m: usize) {
    let n = (m + 0x8eb8) as *mut i8;
    *n = 0;
    let lista = (m + 0x8f7e) as *mut usize;
    for i in 0..20usize {
        let primeiro = sortear(2) as i8;
        for lado in [(primeiro as isize) as usize, (primeiro == 0) as usize] {
            let p = (m as isize + 0x4796 + 0x1be * (i as isize + 20 * lado as isize)) as usize;
            if byte(p, 0x19) >= 0 {
                *lista.offset(*n as isize) = p;
                *n = (*n).wrapping_add(1);
            }
        }
    }
    *(0xb67a64 as *mut usize) = m;
    let qsort: unsafe extern "C" fn(usize, i32, i32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(m + 0x8f7e, *n as i32, 4, 0x6bd840);
}

// ---------- sexagesima sexta leva: ordem dos cobradores de penalti ----------

/// the order of the 11 takers of each side at +0x1af + 11 × side (−1 first), marking those chosen in each side's 20
/// slots at +0x8eda (zeroed first). Side's manager (+0x904d + 208 × side) human, with +0x5f, without 0x9a01 and (with
/// +0x904b) national team or with 0x9a05: the order comes from the tactics table (0x898040 on [0xde1d08]). Otherwise,
/// `escolher_cobrador` type 5; with no one, clears the side's marks and repeats the turn
/// Original: `penalty_order` (0x6be520, match_eng.cpp, thiscall).
///
/// [pt] { 0x6be520 } (match_eng.cpp): monta a ordem dos 11 cobradores de penalti de cada lado em
/// +0x1af + 11 x lado (-1 antes), marcando os escolhidos nas 20 casas de cada lado em +0x8eda
/// (zeradas antes). Se o tecnico do lado (+0x904d + 208 x lado) e um dos 16 humanos, tem a marca
/// +0x5f, nao tem 0x9a01 (0x5fd210) e, com +0x904b, e de selecao (0x53b900) ou tem 0x9a05
/// (0x5fd480): a ordem vem da tabela tatica do clube (0x898040 sobre [0xde1d08], com `humano` =
/// byte +0x40/+0x41 da partida igual a 0). Senao, `escolher_cobrador` tipo 5; sem ninguem, zera as
/// marcas do lado e tenta a mesma vez de novo.
pub(crate) unsafe extern "thiscall" fn ordem_dos_penaltis(m: usize) {
    core::ptr::write_bytes((m + 0x8eda) as *mut u8, 0, 40);
    core::ptr::write_bytes((m + 0x1af) as *mut u8, 0xff, 22);
    let h = *(0xb63c98 as *const usize);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let t9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let tatico: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8 = core::mem::transmute(FN_TACTICS_GET_SHOOTOUT_TAKER_SLOT);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    for lado in 0..2i8 {
        let l = lado as usize;
        let mut vez = 0i8;
        while vez < 11 {
            let tecnico = *((m + 0x904d + 208 * l) as *const usize);
            let pessoas = *(0xae2364 as *const i32);
            let pela_tabela = tecnico != 0
                && (pessoas.wrapping_sub(16)..pessoas).contains(&*(tecnico as *const i32))
                && byte(tecnico, 0x5f) != 0
                && t9a01(h, tecnico) == 0
                && (byte(m + 208 * l, 0x904b) == 0 || {
                    let partida = *((m + 0x4792) as *const usize);
                    e_selecao(*((partida + if lado == 0 { 0x1c } else { 0x20 }) as *const usize)) != 0
                        || t9a05(h, *((m + 0x904d + 208 * l) as *const usize)) != 0
                });
            let escolhido = if pela_tabela {
                let partida = *((m + 0x4792) as *const usize);
                let (marca, clube) = if lado == 0 {
                    (byte(partida, 0x40), *((partida + 0x1c) as *const usize))
                } else {
                    (byte(partida, 0x41), *((partida + 0x20) as *const usize))
                };
                Some(tatico(*(0xde1d08 as *const usize), clube, vez, (marca == 0) as u32) as i8)
            } else {
                let c = cobrador(m, lado, 5, m + 0x8eda);
                if c == 0 {
                    vez = vez.wrapping_sub(1);
                    core::ptr::write_bytes((m + 0x8eda + 20 * l) as *mut u8, 0, 20);
                    None
                } else {
                    Some(*(c as *const i8))
                }
            };
            if let Some(e) = escolhido {
                *((m as isize + 0x1af + 11 * l as isize + vez as isize) as *mut i8) = e;
                *((m as isize + 0x8eda + 20 * l as isize + e as isize) as *mut u8) = 1;
            }
            vez = vez.wrapping_add(1);
        }
    }
}

// ---------- sexagesima quarta leva: lado sob controle ----------

/// { 0x6d42a0 } (match_eng.cpp): o lado esta sob controle de quem decide por ele? Nao nas
/// partidas de tipo +0x3a 1 ou 9. Clube do lado dirigido por humano (0x600a50, `qualquer` = byte
/// +0x40 da partida): pela marca 0x4000 do bloco do lado (0x9766 + 6371 x lado). Senao: o modo do
/// lado em +0x1ab e 3 ou 4.
pub(crate) unsafe extern "thiscall" fn lado_sob_controle(m: usize, lado: i8) -> u8 {
    let partida = *((m + 0x4792) as *const usize);
    let tipo = byte(partida, 0x3a);
    if tipo == 1 || tipo == 9 {
        return 0;
    }
    let clube = *((partida + if lado != 0 { 0x20 } else { 0x1c }) as *const usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let l = lado as isize;
    if humano(clube, 0, byte(partida, 0x40) as u8, 0, 0) != 0 {
        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x4000 != 0) as u8
    } else {
        let modo = *((m as isize + l + 0x1ab) as *const i8);
        (modo == 3 || modo == 4) as u8
    }
}

// ---------- sexagesima primeira leva: quem marcar (0x6bc030, 3 KB) ----------

/// Distancia (mais o ajuste 0x968990) do ponto (x, y) ao gol que o lado do jogador `j` ataca
/// (coluna 4; linha 0 para o lado 0, 11 para o lado 1), arredondada para float.
pub(crate) unsafe fn distancia_do_gol(j: usize, x: i32, y: i32) -> f32 {
    let dl = if lado(j) == 0 { y } else { y - 11 };
    let (c, l) = ((x - 4).wrapping_abs() as i8, dl.wrapping_abs() as i8);
    let d = if (0..=8).contains(&c) && (0..=11).contains(&l) {
        *((0xa15e44 + 4 * (l as isize * 9 + c as isize)) as *const f32)
    } else {
        f32_em(0x96d2dc)
    };
    (d as f64 + f32_em(0x968990) as f64) as f32
}

/// Quantos adversarios (bloco do lado `s`) cercam o ponto (x, y): 3 para cada um a distancia
/// igual a 0x9689a0, 1 para cada um ate 0x968940. Ficam de fora os inativos e os de `tabela_a` 1.
pub(crate) unsafe fn cerco(m: usize, s: usize, x: i8, y: i8) -> i8 {
    let mut n = 0i8;
    for i in 0..20 {
        let p = m + 0x4796 + 8920 * s + 0x1be * i;
        if byte(p, 0x19) < 0 || tabela_do_jogador(p) == 1 {
            continue;
        }
        let d = distancia_ao_ponto(p, x, y) as f64;
        if d == f64_em(0x9689a0) || d.is_nan() {
            n = n.wrapping_add(3);
        } else if !(d > f64_em(0x968940)) {
            n = n.wrapping_add(1);
        }
    }
    n
}

pub(crate) static mut TABELA65: Vec<u8> = Vec::new();
pub(crate) static mut CLUBES74: Vec<u8> = Vec::new();

/// Bonus de fadiga do sorteio: 0x96d2d8 - distancia ao gol x 0x968a78.
pub(crate) unsafe fn bonus_do_gol(d: f32) -> i32 {
    ftol(f32_em(0x96d2d8) as f64 - d as f64 * f32_em(0x968a78) as f64)
}

/// Reputacao usada na nota do alvo: a de casa se ele e da nacao do jogador, a atual se os dois
/// clubes da partida sao da mesma nacao, senao a mundial; em centenas, menos 50.
pub(crate) unsafe fn reputacao_do_alvo(j: usize, p: usize, clubes: (usize, usize)) -> i32 {
    let ficha = *((p + 0x6d) as *const usize);
    let nacao = |q: usize| *((*((q + 0x69) as *const usize) + 0x1a) as *const u32);
    let off = if nacao(j) == nacao(p) {
        9
    } else if clubes.0 != 0 && clubes.1 != 0 && *((clubes.0 + 0x53) as *const u32) == *((clubes.1 + 0x53) as *const u32) {
        0xb
    } else {
        0xd
    };
    (*((ficha + off) as *const i16) as i32) / 100 - 50
}

/// { 0x6bc030 } (match_eng.cpp): o adversario que o jogador `j` vai marcar, ou 0. Nao marca se a
/// casa dele tem a marca 1; o marcado designado (0x7160f0) ativo vem antes de tudo. Senao, pela
/// casa (x, y) onde ele deve estar (0x6c2be0), avalia primeiro quem esta com a bola (0xf582, se
/// perto) e depois os adversarios das casas das linhas y-1..y+1 e colunas x-1..x+1 (listas em
/// motor + 0x215e, 90 bytes por casa). Cada um recebe uma nota (sorteios, cerco de adversarios,
/// distancia ao gol, atributos 0x0a/0x0b/0x10/0x35/0x101, papel na formacao, reputacao) e fica o
/// de maior nota (comparada em 16 bits; o primeiro empate fica). O guardado em 0x1aa do jogador
/// pode ser escolhido na hora.
pub(crate) unsafe extern "thiscall" fn quem_marcar(m: usize, j: usize) -> usize {
    let lado_j = lado(j) as usize;
    if byte_da_posicao(m + 0x91e2 + 6371 * lado_j, byte(j, 0x19) as i8) & 1 != 0 {
        return 0;
    }
    let d = marcado_designado(j);
    if d != 0 && byte(d, 0x19) >= 0 {
        return d;
    }
    let s = (lado_j == 0) as usize; // o lado adversario
    let partida = *((m + 0x4792) as *const usize);
    let (c1c, c20) = (*((partida + 0x1c) as *const usize), *((partida + 0x20) as *const usize));
    let clubes = if s == 1 { (c1c, c20) } else { (c20, c1c) };
    let (mut x, mut y) = (0i8, 0i8);
    posicionar(m, j, &mut x, &mut y, 1);
    let bonus = byte(j, 0x10a);
    let fora_da_metade = |q: usize| -> bool {
        bonus != 0 && condicao_jogador(q) != 0 && if s == 0 { y >= 6 } else { y < 6 }
    };
    let mut melhor = 0usize;
    let mut nota_melhor = 0i16;
    let mut guardar = |q: usize, nota: i32| {
        if melhor == 0 || nota as i16 > nota_melhor {
            melhor = q;
            nota_melhor = nota as i16;
        }
    };

    // quem esta com a bola (a "linha" y-2 do laco do original)
    let c = *((m + 0xf582) as *const usize);
    let y0 = y.wrapping_sub(2);
    if y0 <= y.wrapping_add(1) && x.wrapping_sub(1) <= x.wrapping_add(1) && c != 0 {
        let (cx, cy) = (byte(c, 0x102), byte(c, 0x103));
        // o original mede do jogador com a bola ate a posicao de `j`
        if !(distancia_ao_ponto(c, byte(j, 0x102), byte(j, 0x103)) as f64 > f64_em(0x968950)) {
            let f1c = distancia_ou_mil(cx as i32 - x as i32, cy as i32 - y as i32);
            let n17 = cerco(m, s, cx, cy);
            let f18 = distancia_do_gol(j, cx as i32, cy as i32);
            let v16 = byte(c, 0x101);
            if v16 != 3 && !fora_da_metade(c) {
                let mut a = sortear(5 * (21 - bonus as i32));
                // o `fld [esp+0x1c]` do original roda com o argumento do sorteio ainda na pilha: e f18
                if (f18 as f64) < f64_em(0x9688a0) && byte(c, 0x109) != 0 {
                    a = a.wrapping_add(bonus_do_gol(f18));
                }
                if (sortear(30) as f64) < f32_em(c + 0xb1) as f64 && (sortear(30) as f64) < f32_em(j + 0xb1) as f64 {
                    a = a.wrapping_add(bonus_do_gol(f18));
                }
                let mut b = a.wrapping_sub(100 * v16 as i32);
                if mais_perto_do_gol(m, c) != 0 {
                    if c == *((j + 0x1aa) as *const usize) {
                        return c;
                    }
                    b = b.wrapping_add(0xaf);
                } else if c == *((j + 0x1aa) as *const usize) && sortear(v16 as i32 + 1) == 0 {
                    return c;
                }
                let mut e = (*((c + 0xb) as *const u8) as i32 + *((c + 0xa) as *const u8) as i32 + 0x14) * 10;
                e = e.wrapping_sub(ftol(f18 as f64 * f18 as f64 - f32_em(0x968960) as f64));
                let n = n17 as i32;
                e = e.wrapping_sub(n.wrapping_mul(n).wrapping_mul(n));
                e = e.wrapping_add(*((c + 0x35) as *const i16) as i32 / 200);
                e = e.wrapping_sub(ftol(*((c + 0x10) as *const u8) as f64 * f64_em(0x96d2d0)));
                e = e.wrapping_sub(ftol(f1c as f64 * f1c as f64 * f32_em(0x969560) as f64));
                b = b.wrapping_add(e);
                guardar(c, b.wrapping_add(reputacao_do_alvo(j, c, clubes)));
            }
        }
    }

    // as casas em volta
    let mut r = y.wrapping_sub(1);
    while r <= y.wrapping_add(1) {
        if !(0..=11).contains(&r) {
            r = r.wrapping_add(1);
            continue;
        }
        let mut col = x.wrapping_sub(1);
        while col <= x.wrapping_add(1) {
            if !(0..=8).contains(&col) {
                col = col.wrapping_add(1);
                continue;
            }
            let casa = m + 0x215e + 90 * (r as usize + 12 * col as usize);
            let quantos = |casa: usize| byte(casa, 0x58 + s);
            if quantos(casa) == 0 {
                col = col.wrapping_add(1);
                continue;
            }
            let f1c = distancia_ou_mil(col as i32 - x as i32, r as i32 - y as i32);
            let n14 = cerco(m, s, col, r);
            let f18 = distancia_do_gol(j, col as i32, r as i32);
            let mut k = 0i8;
            while k < quantos(casa) {
                let q = *((casa + 4 * (k as isize + 11 * s as isize) as usize) as *const usize);
                k = k.wrapping_add(1);
                let ponta = *((m + 0xf582) as *const usize);
                if q == ponta {
                    continue;
                }
                let v16 = byte(q, 0x101);
                if v16 == 3 || fora_da_metade(q) {
                    continue;
                }
                if q != ponta && q != *((m + 0xf57a) as *const usize) {
                    let marcas = *((m + 0x9766 + 6371 * lado_j) as *const u32);
                    let dif = diferenca_do_jogador(q) as i8;
                    if if marcas & 0x400 != 0 && bonus != 0 { dif >= 0 } else { dif > 0 } {
                        continue;
                    }
                }
                let mut a = sortear(5 * (21 - bonus as i32));
                if (sortear(30) as f64) < f32_em(q + 0xad) as f64 && (sortear(30) as f64) < f32_em(j + 0xad) as f64 {
                    a = a.wrapping_add(bonus_do_gol(f18));
                }
                a = a.wrapping_sub(100 * v16 as i32);
                let guardado = *((j + 0x1aa) as *const usize);
                if mais_perto_do_gol(m, q) != 0 {
                    if q == guardado {
                        return q;
                    }
                    a = a.wrapping_add(0xaf);
                } else if q == guardado && sortear(v16 as i32 + 1) == 0 {
                    if e_o_guardado(q) != 0 || q == *((m + 0xf57a) as *const usize) {
                        return q;
                    }
                    a = a.wrapping_add(0x14);
                }
                let mut e = (*((q + 0xb) as *const u8) as i32 + *((q + 0xa) as *const u8) as i32 + 0xa) * 10;
                e = e.wrapping_sub(ftol(f64_em(0x96d2a8) - *((q + 0x35) as *const i16) as f64 * f64_em(0x96d2c8)));
                e = e.wrapping_sub(ftol(f18 as f64 * f18 as f64 - f32_em(0x968960) as f64));
                e = e.wrapping_sub(ftol(*((q + 0x10) as *const u8) as f64 * f64_em(0x96d2d0)));
                let n = n14 as i32;
                e = e.wrapping_sub(n.wrapping_mul(n));
                e = e.wrapping_sub(ftol(f1c as f64 * f1c as f64 * f32_em(0x969560) as f64));
                let mut nota = a.wrapping_add(e);
                let pos = byte(q, 0x19);
                let papel = |base: usize| -> u16 {
                    if (0..=10).contains(&pos) {
                        let t = *((m + base + 4 * lado(q) as usize) as *const usize);
                        *((t + 2 * pos as usize) as *const u16)
                    } else {
                        0
                    }
                };
                let w = papel(0x8ebc);
                if w & 0x20 != 0 && w & 0x800 == 0 && w & 0x80 == 0 {
                    let ficha = *((q + 0x6d) as *const usize);
                    nota = nota.wrapping_add(10 * byte(ficha, 0x1a) as i32);
                } else {
                    let fixo = |v: u16| v == 0x140 || v == 0x240 || v == 0x440;
                    if (0..=10).contains(&pos) && !fixo(papel(0x8ebc)) && fixo(papel(0x8ec4)) {
                        nota = nota.wrapping_add(0x7d);
                    } else if marca_800000(q) != 0 {
                        nota = nota.wrapping_add(0x28);
                    }
                }
                guardar(q, nota.wrapping_add(reputacao_do_alvo(j, q, clubes)));
            }
            col = col.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
    melhor
}

// ---------- quinquagesima setima leva: lateral, escanteio e penalti ----------

pub(crate) unsafe fn mover(j: usize, x: i8, y: i8) {
    let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
    f(j, x, y);
}
pub(crate) unsafe fn marcar_2b(j: usize) {
    let f: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(FN_MATCH_PLAYER_SET_ACTION_DELAY);
    f(j, 0);
}
pub(crate) unsafe fn pega_bola(j: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_POSSESSION);
    f(j);
}
pub(crate) unsafe fn cobrador(this: usize, lado: i8, tipo: i8) -> usize {
    let f: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    f(this, lado, tipo, 0)
}

/// throw-in for side 0x8eae. Counts (0xf5c8 + 54 × side), puts the ball (0x8ea7, 0x8ea8) inside the pitch, picks the
/// taker (type 2), arranges the 40 players by the formation table (0xf56e/0xf572 by the player's side; index: of the
/// side taking?, position, ball column and row; byte = column + 16 × row) and puts the taker on the ball, facing
/// inwards (90 or 270). With no taker the original shows an error box (no box here) and marks the engine (+0 = 1)
/// Original: `throw_in` (0x6c4290, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4290 } (match_eng.cpp): lateral do lado 0x8eae. Conta (0xf5c8 + 54 x lado), poe a bola
/// (0x8ea7, 0x8ea8) dentro do campo, escolhe o cobrador (tipo 2), arruma os 40 jogadores pela tabela
/// da formacao (0xf56e/0xf572 pelo lado do jogador; indice: do lado que cobra?, posicao, coluna e
/// linha da bola; byte = coluna | linha << 4) e poe o cobrador na bola, virado para dentro (90 ou 270).
/// Sem cobrador o original mostra uma caixa de erro (sem a caixa aqui) e marca o motor (+0 = 1).
pub(crate) unsafe extern "thiscall" fn lateral(this: usize) {
    let t = byte(this, 0x8eae);
    let conta = (this as isize + 54 * t as isize + 0xf5c8) as *mut u8;
    *conta = (*conta).wrapping_add(1);
    let a7 = (this + 0x8ea7) as *mut i8;
    let a8 = (this + 0x8ea8) as *mut i8;
    *a7 = (*a7).max(0).min(8);
    *a8 = (*a8).max(0).min(11);
    let c = cobrador(this, t, 2);
    for l in 0..2i8 {
        let proprio = (t as i32 == l as i32) as isize;
        for k in 0..20 {
            let j = jogador_do_bloco(this, l, k);
            let pos = byte(j, 0x19);
            if pos < 0 {
                continue;
            }
            let tabela = *((this + 0xf56e + 4 * (byte(j, 0x27) == 1) as usize) as *const usize);
            let casa = *((tabela as isize + 12 * ((proprio + 2 * pos as isize) * 9 + *a7 as isize) + *a8 as isize) as *const u8);
            mover(j, (casa & 0xf) as i8, (casa >> 4) as i8);
            marcar_2b(j);
        }
    }
    if c == 0 {
        *(0xb67a34 as *mut u32) = 0;
        *(this as *mut u8) = 1;
        return;
    }
    mover(c, *a7, *a8);
    pega_bola(c);
    *((c + 0x19c) as *mut i16) = 200;
    *((c + 0x19a) as *mut i16) = if *a7 != 0 { 270 } else { 90 };
}

/// counts (0xf5c7 + 54 × side), marks the play (0x8eb1 = 3, 0x8eae = −1) and arranges the 40: by the attacking role
/// (0x716270, side taking) or defending role (0x716310), some go to drawn squares in the box (column 4 ± 1; row 5/6, or
/// drawn 0..1/10..11), the others by the formation table (+0x3b or +0x30). Off the pitch: fatal error. Taker (type 3)
/// at the flag (column 0 or 8 by a draw against 0x8ea7; row 0 or 11)
/// Original: `corner` (0x6c4470, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4470 } (match_eng.cpp): escanteio do lado 0x8eae. Conta (0xf5c7 + 54 x lado), marca o
/// lance (0x8eb1 = 3, 0x8eae = -1) e arruma os 40 jogadores: pelo papel no ataque (0x716270, do lado
/// que cobra) ou na defesa (0x716310), alguns vao para casas sorteadas na area (coluna 4 +- 1; linha
/// 5/6, ou 0..1/10..11 sorteada), os outros pela tabela da formacao (0xf56e lado 0, 0xf572 lado 1;
/// +0x3b ou +0x30). Fora do campo: erro fatal. Depois escolhe o cobrador (tipo 3) e o poe na
/// bandeira (coluna 0 ou 8 pelo sorteio contra 0x8ea7; linha 0 ou 11).
pub(crate) unsafe extern "thiscall" fn escanteio(this: usize) {
    let ataque: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_ATTACKING_SET_PIECE_ROLE);
    let defesa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_GET_DEFENDING_SET_PIECE_ROLE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let t = byte(this, 0x8eae);
    let conta = (this as isize + 54 * t as isize + 0xf5c7) as *mut u8;
    *conta = (*conta).wrapping_add(1);
    *((this + 0x8eb1) as *mut u8) = 3;
    *((this + 0x8eae) as *mut i8) = -1;
    for l in 0..2i8 {
        let proprio = t as i32 == l as i32;
        for k in 0..20 {
            let j = jogador_do_bloco(this, l, k);
            let pos = byte(j, 0x19);
            if pos < 0 {
                continue;
            }
            // casas da area: 'a' linha sorteada 0..1, 'b' 10..11, 'c' 6, 'd' 5
            let r = if proprio { ataque(j) } else { defesa(j) };
            let area = match (proprio, l == 1, r) {
                (true, true, 1) | (false, false, 4) => Some('a'),
                (true, false, 1) | (false, true, 4) => Some('b'),
                (true, true, 2) | (false, true, 3) => Some('c'),
                (true, false, 2) | (false, false, 3) => Some('d'),
                _ => None,
            };
            let (x, y) = match area {
                Some(z) => {
                    let mut x = 4i8.wrapping_sub(sorteio(2) as i8);
                    x = x.wrapping_add(sorteio(2) as i8);
                    let y = match z {
                        'a' => sorteio(2) as i8,
                        'b' => 11i8.wrapping_sub(sorteio(2) as i8),
                        'c' => 6,
                        _ => 5,
                    };
                    (x, y)
                }
                None => {
                    let tabela = *((this + if l == 0 { 0xf56e } else { 0xf572 }) as *const usize);
                    let desl = if proprio == (l == 0) { 0x3b } else { 0x30 };
                    let casa = *((tabela as isize + 4 * 27 * (proprio as isize + 2 * pos as isize) + desl) as *const u8);
                    ((casa & 0xf) as i8, (casa >> 4) as i8)
                }
            };
            if !(0..9).contains(&x) || !(0..12).contains(&y) {
                erro_fatal();
            }
            mover(j, x, y);
            marcar_2b(j);
        }
    }
    let c = cobrador(this, t, 3);
    let r = sorteio(2);
    let x = if byte(this, 0x8ea7) as i32 > 4 - r { 0 } else { 8 };
    mover(c, x, if t != 0 { 0 } else { 11 });
    pega_bola(c);
    *((c + 0x19c) as *mut i16) = 200;
    if t == 0 {
        *((c + 0x19a) as *mut i16) = if byte(this, 0x8ea7) != 0 { 90 } else { 270 };
    }
}

/// taker (type 5) with the ball on the spot (column 4; row 1 or 10, also in 0x8ea7/0x8ea8); the stored goalkeepers go
/// to goal, those on rows 0..1 go to row 2 and those on 10..11 to row 9
/// Original: `penalty` (0x6c50d0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c50d0 } (match_eng.cpp): penalti do lado 0x8eae. Escolhe o cobrador (tipo 5), da a bola a
/// ele na marca (coluna 4; linha 1 ou 10, tambem em 0x8ea7/0x8ea8) e tira os outros da area: os
/// goleiros guardados (0xf59e, 0xf5a2) vao para o gol (linha 0 / 11), quem esta nas linhas 0..1 vai
/// para a 2 e quem esta nas 10..11 para a 9, na mesma coluna.
pub(crate) unsafe extern "thiscall" fn penalti(this: usize) {
    let t = byte(this, 0x8eae);
    let c = cobrador(this, t, 5);
    pega_bola(c);
    let linha: i8 = if t != 0 { 1 } else { 10 };
    *((this + 0x8ea7) as *mut i8) = 4;
    *((this + 0x8ea8) as *mut i8) = linha;
    mover(c, 4, linha);
    for k in 0..20 {
        for l in 0..2i8 {
            let j = jogador_do_bloco(this, l, k);
            if byte(j, 0x19) >= 0 && j != c {
                let goleiro = *((this + if l == 0 { 0xf59e } else { 0xf5a2 }) as *const usize);
                let y = byte(j, 0x103);
                if j == goleiro {
                    mover(j, 4, if l == 0 { 0 } else { 11 });
                } else if y < 2 {
                    mover(j, byte(j, 0x102), 2);
                } else if y > 9 {
                    mover(j, byte(j, 0x102), 9);
                }
            }
            marcar_2b(j);
        }
    }
}

// ---------- quinquagesima sexta leva: escolher o cobrador ----------

/// Caixa de erro do original (match_eng.cpp, linha) sem a mensagem: zera 0xb67a34, 0x5cc7d0,
/// 0x5a8f60 e exit(-1).
pub(crate) unsafe fn erro_fatal() -> ! {
    *(0xb67a34 as *mut u32) = 0;
    let a: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let b: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    a();
    b();
    sair(-1)
}

/// { 0x6ceb90 } (match_eng.cpp): escolhe o cobrador do lado para a bola parada `tipo` (1..=8),
/// pela maior pontuacao entre os 20 do bloco (o primeiro empate fica). Fora: posicao < 0, ou
/// marcado em `excluidos` (tipo 5). Posicao > 10 ou sem pessoa (+0x6d): posicao vira -1. O goleiro
/// guardado (0xf59e/0xf5a2) nao cobra 1, 2, 3, 8, nem 6/7 longe da area adversaria. Pontuacoes:
/// 1 e 2: atributo (0xdd, 0xe9) - 2 x distancia a bola; 3 (escanteio): o cobrador designado na tatica
/// (0x897990 sobre 0xde1d08; 10000/9000, com sorteio no lado contrario) ou 0x89 - 0xad/2, x 0,75
/// (1,25 se negativo) com a marca 0x40 da tabela; 4: 100 com a tabela = 1; 5 (penalti): a lista do
/// tecnico humano (10000 - ordem, 0x601070) ou 0xc9 pela idade, sorteio de +0x5a e +0xb da pessoa;
/// 6/7 (falta): perto do gol, o designado ou bola parada da pessoa (+0x29) e 0x109; longe,
/// 0xa1 - distancias; 8: 100 - distancia. Sem ninguem: 0 com `excluidos`, senao erro fatal.
pub(crate) unsafe extern "thiscall" fn escolher_cobrador(this: usize, lado: i8, tipo: i8, excluidos: usize) -> usize {
    let dist: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(FN_MATCH_PLAYER_DISTANCE_TO_CELL);
    let tatica: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_SET_PIECE_TAKER);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let f5fd210: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let f5fd480: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
    let f601070: unsafe extern "thiscall" fn(usize, i32, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_SQUAD_NUMBER_PLAYER);
    let d32 = |p: usize, o: usize| *((p + o) as *const usize);
    let f = |p: usize, o: usize| f32_em(p + o) as f64;
    let partida = d32(this, 0x4792);
    let (mut melhor, mut melhor_v) = (0usize, -1i16);
    for k in 0..20isize {
        let i = lado as isize * 20 + k;
        let j = (this as isize + 0x4796 + 0x1be * i) as usize;
        let (a7, a8) = (byte(this, 0x8ea7), byte(this, 0x8ea8));
        if tipo == 5 && excluidos != 0 && *((excluidos as isize + i) as *const u8) != 0 {
            continue;
        }
        if byte(j, 0x19) < 0 {
            continue;
        }
        let guardado = (lado == 0 && j == d32(this, 0xf59e)) || (lado == 1 && j == d32(this, 0xf5a2));
        if guardado {
            match tipo {
                1 | 2 | 3 | 8 => continue,
                6 | 7 => {
                    let linha = if lado == 1 { (10..=11).contains(&a8) } else { (0..=1).contains(&a8) };
                    if !((2..=6).contains(&a7) && linha) {
                        continue;
                    }
                }
                _ => {}
            }
        }
        if byte(j, 0x19) > 10 || d32(j, 0x6d) == 0 {
            *((j + 0x19) as *mut i8) = -1;
            continue;
        }
        // cobrador designado na tatica do clube do jogador
        let designado = |a: i32, b: i32| {
            let fora = byte(j, 0x27) != 0;
            let reserva = (byte(partida, if fora { 0x41 } else { 0x40 }) == 0) as i32;
            let clube = d32(partida, if fora { 0x20 } else { 0x1c });
            tatica(*(0xde1d08 as *const usize), clube, a, b, reserva) == d32(j, 0x69)
        };
        // perto do gol: tabela de distancias da bola (|a7 - 4|, |a8 - linha do gol|)
        let tabela_bola = || {
            let cx = (a7 as i32 - 4).abs() as i8;
            let ax = (a8 as i32 - if lado != 0 { 0 } else { 11 }).abs() as i8;
            if (0..=8).contains(&cx) && (0..=11).contains(&ax) {
                distancia_tabela(ax as i32, cx as i32) as f64
            } else {
                f32_em(0x96d2dc) as f64
            }
        };
        let ataque = if lado == 0 { a7 < 4 } else { lado == 1 && a7 > 4 };
        let bola_parada = || {
            let pd = d32(j, 0x6d);
            let mut s = (byte(pd, 0x29) as i16).wrapping_mul(5).wrapping_add((byte(j, 0x109) as i16).wrapping_mul(2));
            if s <= 0 {
                return s;
            }
            let idade = byte(d32(j, 0x69), 0x18);
            if idade < 20 {
                s = ftol(s as f64 * f64_em(0x969520)) as i16;
            } else if idade < 24 {
                s = ftol(s as f64 * f64_em(0x9689d0)) as i16;
            }
            ftol((*((pd + 0xb) as *const i16) as f64 * f64_em(0x96d240) + f64_em(0x969040)) * s as f64) as i16
        };
        let longe = || {
            let g1 = dist(j, a7, a8);
            let g2 = dist(j, a7, a8);
            ftol(f(j, 0xa1) - g2 as f64 * g1 as f64) as i16
        };
        let v: i16 = match tipo {
            1 | 2 => ftol(f(j, if tipo == 1 { 0xdd } else { 0xe9 }) - 2.0 * dist(j, a7, a8) as f64) as i16,
            3 => {
                let pontos = || {
                    let s = ftol(f(j, 0x89) - f(j, 0xad) * f32_em(0x968f18) as f64) as i16;
                    if tabela_a(this, j) & 0x40 == 0 {
                        s
                    } else if s > 0 {
                        ftol(s as f64 * f64_em(0x969040)) as i16
                    } else {
                        ftol(s as f64 * f64_em(0x968928)) as i16
                    }
                };
                if ataque {
                    if designado(2, 0) { 10000 } else if designado(2, 1) { 9000 } else { pontos() }
                } else if designado(3, 0) {
                    (10000 - sorteio(1000)) as i16
                } else if designado(3, 1) {
                    (sorteio(500) + 9000) as i16
                } else {
                    pontos()
                }
            }
            4 => if tabela_a(this, j) == 1 { 100 } else { 0 },
            5 => {
                let bloco = (this as isize + 0xd0 * lado as isize) as usize;
                let tec = d32(bloco, 0x904d);
                let n = *(0xae2364 as *const i32);
                let lista = tec != 0
                    && (n - 16..n).contains(&*(tec as *const i32))
                    && byte(tec, 0x5f) != 0
                    && f5fd210(*(0xb63c98 as *const usize), tec) == 0
                    && (byte(bloco, 0x904b) == 0
                        || e_selecao(d32(partida, if lado == 0 { 0x1c } else { 0x20 })) != 0
                        || f5fd480(*(0xb63c98 as *const usize), d32(bloco, 0x904d)) != 0)
                    && excluidos == 0;
                if lista {
                    let mut v = ftol(f(j, 0xc9) - (byte(d32(j, 0x6d), 0xf) as i32 * 20 - 20) as f64) as i16;
                    for b in 0..50i32 {
                        if f601070(*(0xb63c98 as *const usize), b, d32(bloco, 0x904d)) == d32(j, 0x69) {
                            v = 10000 - b as i16;
                            break;
                        }
                    }
                    v
                } else if j == d32(this, 0xf59e) || j == d32(this, 0xf5a2) {
                    -10000
                } else {
                    let mut s = ftol(f(j, 0xc9)) as i16;
                    if s > 0 {
                        let ficha = d32(j, 0x69);
                        let idade = byte(ficha, 0x18);
                        if idade < 20 {
                            s = ftol(s as f64 * f64_em(0x969510)) as i16;
                        } else if idade < 24 {
                            s = ftol(s as f64 * f64_em(0x969040)) as i16;
                        }
                        let b5a = byte(ficha, 0x5a);
                        if b5a != 0 && sorteio(b5a as i32) == 0 {
                            s = ftol(s as f64 * f64_em(0x969510)) as i16;
                        }
                        s = ftol((*((d32(j, 0x6d) + 0xb) as *const i16) as f64 * f64_em(0x96d240) + f64_em(0x969040)) * s as f64) as i16;
                    }
                    s
                }
            }
            6 | 7 => {
                if (2..=6).contains(&a7) {
                    if tabela_bola() < f64_em(0x96d428) {
                        let a = if ataque { 0 } else { 1 };
                        if designado(a, 0) { 10000 } else if designado(a, 1) { 9000 } else { bola_parada() }
                    } else {
                        longe()
                    }
                } else if tabela_bola() < f64_em(0x96d310) {
                    ftol(f(j, 0xa1) * 2.0 - f(j, 0xad) * f32_em(0x968f18) as f64) as i16
                } else {
                    longe()
                }
            }
            8 => ftol(f32_em(0x968960) as f64 - dist(j, a7, a8) as f64) as i16,
            _ => erro_fatal(),
        };
        if melhor == 0 || v > melhor_v {
            melhor_v = v;
            melhor = j;
        }
    }
    if melhor == 0 && excluidos == 0 {
        erro_fatal();
    }
    melhor
}

// ---------- quinquagesima quinta leva: reputacao do jogador ----------

/// { 0x6d3610 } (match_eng.cpp): reputacoes do jogador depois da partida, na mesma conta da do
/// clube (0x6d3de0), para as tres da ficha (+0x69 do jogador): atual (+8), em casa (+0xa) e mundial
/// (+0xc). Grava em `reg` (casa, atual, mundial). Pesos: casa = (reputacao/1000 + 16, minimo 22) -
/// nivel; atual = o mesmo, se o clube e selecao ou o do jogador (+0x1a) e o do clube (+0x53) ou o da
/// competicao (+0x5d), senao a media disso com 202 - 10 x nivel (minimo 4); mundial como a do clube
/// (minimo 21). Faixas por partida: casa -250/+300, atual -250/+375, mundial -500/+100 x nivel (x
/// 2 - reputacao/10000). Tetos: 400 x nivel (/2 sem o caso especial), entre 5000 (mundial 3000) e 10000.
pub(crate) unsafe extern "C" fn reputacao_do_jogador(reg: usize, meu: usize, outro: usize, partida: usize, valor: f32, alt: u8, bonus: u8) {
    let p = *((reg + 0x1b) as *const usize);
    if p == 0 {
        return;
    }
    let st = *((p + 0x69) as *const usize);
    if st == 0 {
        return;
    }
    let w = |c: usize, o: usize| *((c + o) as *const i16) as i32;
    let comp = *((partida + 0x14) as *const usize);
    let mut nivel: i8 = if *(partida as *const i32) == -2 {
        5
    } else {
        let v = if byte(partida, 0x40) != 0 {
            ((w(outro, 0x80) / 500 + w(comp, 0x69)) / 2) as i8
        } else {
            byte(comp, 0x69)
        };
        v.min(20)
    };
    let teto = |rep: i32, minimo: i8| ((rep / 1000) as i8).wrapping_add(16).max(minimo) as i16;
    let (atual, casa, mundo) = (w(st, 8), w(st, 0xa), w(st, 0xc));
    let n_casa = teto(casa, 22).wrapping_sub(nivel as i16);
    let selecao = e_selecao(meu) != 0;
    let clube_p = *((p + 0x1a) as *const usize);
    let (mut f_atual, mut f_mundo) = (2i32, 2i32);
    // ponytail: o original tambem testa partida -2 com +0x53 nao nulo; o ultimo teste (+0x53 igual) ja cobre
    let n_atual: i16 = if selecao || (comp != 0 && *((comp + 0x5d) as *const usize) == clube_p) || *((meu + 0x53) as *const usize) == clube_p {
        f_atual = 1;
        teto(atual, 22).wrapping_sub(nivel as i16)
    } else {
        let x = ((0xca - 10 * nivel as i32) as i16 as i32 + teto(atual, 22).wrapping_sub(nivel as i16) as i32) / 2;
        if (x as i16) < 4 { 4 } else { x as i16 }
    };
    let w30 = *((partida + 0x30) as *const u16);
    let w32 = *((partida + 0x32) as *const u16);
    let n_mundo: i16 = if selecao {
        f_mundo = 1;
        teto(mundo, 21).wrapping_sub(nivel as i16)
    } else if byte(partida, 0x3a) == 8 {
        f_mundo = 1;
        if (0x41f..=0x426).contains(&w32) {
            teto(mundo, 21).wrapping_sub(nivel as i16).wrapping_add(1)
        } else if matches!(w30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b) {
            nivel = byte(comp, 0x69);
            teto(mundo, 21).wrapping_sub(nivel as i16)
        } else {
            (0xca - 10 * nivel as i32) as i16
        }
    } else {
        (0xca - 10 * nivel as i32) as i16
    };
    let nv = nivel as i32;
    let dif = (valor as f64 - razao_global(alt != 0)) * f32_em(0x96d490) as f64;
    let g = (nv as f64 * f64_em(0x968968) + f64_em(0x9688a0)) * dif * f64_em(0x968ff8);
    let mut novo_atual = ftol(atual as f64 + g) as i16;
    let mut novo_casa = ftol(casa as f64 + g) as i16;
    let mut novo_mundo = ftol(mundo as f64 + g) as i16;
    if bonus != 0 {
        let mut t = ftol((nv * 250) as f64 * valor as f64 / razao_global(alt != 0));
        if g < f32_em(0x968970) as f64 {
            t = ftol(t as i16 as f64 * valor as f64 / razao_global(alt != 0));
        }
        let t = t as i16;
        novo_casa = novo_casa.max(t);
        novo_atual = novo_atual.max(t);
        novo_mundo = novo_mundo.max(t);
    }
    let limite = (nv * 300 + 4000) as i16;
    let k = |r: i32| (f64_em(0x968950) - r as f64 * f64_em(0x96a5c0)) as f32;
    let media = |n: i16, r: i32, novo: i16| {
        let n = n as i32;
        (((n - 1).wrapping_mul(r).wrapping_add(novo as i32) / n) as i16).min(limite.max(r as i16))
    };
    let faixa = |r: i32, baixo: f64, alto: f64, m: i16| {
        let (rf, mf, k) = (r as f32 as f64, m as f32 as f64, k(r) as f64);
        let lo = rf - k * baixo;
        let v = if lo > mf {
            lo
        } else {
            let hi = k * alto + rf;
            if hi < mf { hi } else { mf }
        };
        ftol(v) as i16
    };
    let r_casa = faixa(casa, f32_em(0x96d210) as f64, f32_em(0x96d20c) as f64, media(n_casa, casa, novo_casa));
    let r_atual = faixa(atual, f32_em(0x96d210) as f64, f32_em(0x96d208) as f64, media(n_atual, atual, novo_atual));
    let r_mundo = faixa(mundo, f32_em(0x96d1a8) as f64, (nv * 100) as f64, media(n_mundo, mundo, novo_mundo));
    let c = nv.wrapping_mul(400);
    let cap = |x: i32, piso: i32, r: i32| (x.clamp(piso, 10000) as i16).max(r as i16);
    *(reg as *mut i16) = r_casa.min(cap(c, 5000, casa)).max(1);
    *((reg + 2) as *mut i16) = r_atual.min(cap(c / f_atual, 5000, atual)).max(1);
    *((reg + 4) as *mut i16) = r_mundo.min(cap(c / f_mundo, 3000, mundo)).max(1);
}

// ---------- quinquagesima quarta leva: nova reputacao ----------

/// Razao das estatisticas globais (0xb74350): +0x20/+0x28 ou +0x10/+0x18; divisor zero da 6,8.
pub(crate) unsafe fn razao_global(alt: bool) -> f64 {
    let g = *(0xb74350 as *const usize);
    let (a, b) = if alt { (0x20, 0x28) } else { (0x10, 0x18) };
    if f64_em(g + b) == f64_em(0x9689a0) {
        f64_em(0x968998)
    } else {
        f64_em(g + a) / f64_em(g + b)
    }
}

/// { 0x6d3de0 } (match_eng.cpp): reputacao do clube do lado depois da partida, gravada em
/// 0x1d6 + 2 x lado (1..=10000). `nivel` (ate 20) vem da competicao (+0x69), media com a reputacao
/// do adversario /500 quando a partida tem +0x40; 5 na partida -2. O peso `n` da media movel: 202 -
/// 10 x nivel, ou, para selecao (e certos codigos de competicao internacional, tipo 8), reputacao/1000
/// + 16 (no minimo 21) - nivel. `valor` contra a razao global da o ganho (negativo ao quadrado).
pub(crate) unsafe extern "C" fn nova_reputacao(this: usize, lado: i8, valor: f32, alt: u8, bonus: u8) {
    let partida = *((this + 0x4792) as *const usize);
    let (meu, outro) = if lado != 0 {
        (*((partida + 0x20) as *const usize), *((partida + 0x1c) as *const usize))
    } else {
        (*((partida + 0x1c) as *const usize), *((partida + 0x20) as *const usize))
    };
    let comp = *((partida + 0x14) as *const usize);
    let rep_de = |c: usize| *((c + 0x80) as *const i16) as i32;
    let mut nivel: i8 = if *(partida as *const i32) == -2 {
        5
    } else {
        let v = if byte(partida, 0x40) != 0 {
            ((rep_de(outro) / 500 + *((comp + 0x69) as *const i16) as i32) / 2) as i8
        } else {
            byte(comp, 0x69)
        };
        v.min(20)
    };
    let teto = || ((rep_de(meu) / 1000) as i8).wrapping_add(16).max(21) as i16;
    let w30 = *((partida + 0x30) as *const u16);
    let w32 = *((partida + 0x32) as *const u16);
    let n: i16 = if e_selecao(meu) != 0 {
        teto().wrapping_sub(nivel as i16)
    } else if byte(partida, 0x3a) == 8 && (0x41f..=0x426).contains(&w32) {
        teto().wrapping_sub(nivel as i16).wrapping_add(1)
    } else if byte(partida, 0x3a) == 8 && matches!(w30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b) {
        nivel = byte(comp, 0x69);
        teto().wrapping_sub(nivel as i16)
    } else {
        (0xca - 10 * nivel as i32) as i16
    };
    let mut f = (valor as f64 - razao_global(alt != 0)) as f32;
    let zero = f32_em(0x968970) as f64;
    if (f as f64) < zero {
        f = (f.abs() as f64 * f as f64) as f32;
    }
    let rep = rep_de(meu);
    let nv = nivel as i32;
    let mut novo = ftol((nv as f64 * f64_em(0x968968) + f64_em(0x9688a0)) * f as f64 * f64_em(0x968ff8) + rep as f64) as i16;
    if bonus != 0 {
        let mut t = ftol((nv * 250) as f64 * valor as f64 / razao_global(alt != 0));
        if (f as f64) < zero {
            t = ftol(t as i16 as f64 * valor as f64 / razao_global(alt != 0));
        }
        if t as i16 > novo {
            novo = t as i16;
        }
    }
    let k = f64_em(0x968950) - rep as f64 * f64_em(0x96a5c0);
    let nn = n as i32;
    let mut media = (((nn - 1).wrapping_mul(rep)).wrapping_add(novo as i32) / nn) as i16;
    let limite = (nv * 300 + 4000) as i16;
    let maximo = limite.max(rep as i16);
    if media >= maximo {
        media = maximo;
    }
    let delta = ((nv * 5) as f64 * k) as f32;
    let rf = rep as f32;
    let mf = media as f32;
    let baixo = rf as f64 - delta as f64;
    let v = if baixo > mf as f64 {
        baixo
    } else {
        let alto = delta as f64 + rf as f64;
        if alto < mf as f64 { alto } else { mf as f64 }
    };
    let r = (ftol(v) as i16).clamp(1, 10000);
    *((this as isize + 0x1d6 + 2 * lado as isize) as *mut i16) = r;
}

// ---------- quinquagesima terceira leva: marcacao individual ----------

/// for each side and active player whose position has marks 0x10 and 4 in the side's table (0x5b57c0), finds the direct
/// marker (0x6d4330) and stores his record (+0x69) in the per-position table (0x5b5910). Marker −1 reads the "player"
/// before the block, like the original
/// Original: `individual_marking` (0x6bbf60, match_eng.cpp, thiscall).
///
/// [pt] { 0x6bbf60 } (match_eng.cpp): marcacao individual. Para cada lado e jogador ativo cuja posicao
/// tem as marcas 0x10 e 4 na tabela do lado (0x5b57c0), acha o marcador direto (0x6d4330) e grava
/// a ficha dele (0x69) na tabela por posicao (0x5b5910). Marcador -1 le o "jogador" anterior ao
/// bloco, como o original.
pub(crate) unsafe extern "thiscall" fn marcacao_individual(this: usize) {
    let marca: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_FORMATION_GET_MARKING_TYPE);
    let marcador: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_FIND_MARKING_TARGET);
    let marcar: unsafe extern "thiscall" fn(usize, i8, *const u32) -> u32 = core::mem::transmute(FN_FORMATION_SET_MARKING_TARGET);
    for l in 0..2i8 {
        let tabela = this + 0x91e2 + 0x18e3 * l as usize;
        for k in 0..20i8 {
            let p = jogador_do_bloco(this, l, k as usize);
            let pos = byte(p, 0x19);
            if pos < 0 || marca(tabela, pos) & 0x10 == 0 || marca(tabela, byte(p, 0x19)) & 4 == 0 {
                continue;
            }
            let mk = marcador(this, l, k) as i8;
            let outro = (l == 0) as isize;
            let q = (this as isize + 0x4796 + 0x1be * (20 * outro + mk as isize)) as usize;
            marcar(tabela, byte(p, 0x19), *((q + 0x69) as *const usize) as *const u32);
        }
    }
}

// ---------- quinquagesima segunda leva: tirar de campo ----------

/// { 0x6cdd50 } (match_eng.cpp): tira o jogador de campo (expulsao ou substituicao). Sai da lista
/// da casa dele na grade (o ultimo ocupa a vaga), desfaz as marcacoes (o marcado em 0x1aa e a lista
/// 0x1ae dos que o marcam). Com `modo` = -1, so isso. Senao, se `modo` = -3 ou se ele e o guardado
/// em 0xf59e/0xf5a2, e nao e atacante (0x40/0x20): escolhe quem assume a posicao dele (0x19), pela
/// maior pontuacao (atacante 70 mil, meia ofensivo 50 mil, 0x10 15 mil, outros -5 mil; +5 mil pelos
/// lados 0x80/0x800); o primeiro candidato entra sem filtro, os seguintes nao podem ser goleiro nem
/// papel 2 ou 4. Goleiro tirado: o escolhido vira o guardado do lado. Time de tecnico humano (pessoa
/// em 0x904d + 208 x lado entre os 16 tecnicos humanos, com classificacao em 0x5f e 0x5fd210 = 0) so
/// escolhe sozinho com a marca 0x904b, sem ser selecao e sem 0x5fd480.
/// Sem candidato: o original mostra uma caixa de erro e encerra o jogo (aqui, o mesmo encerramento).
pub(crate) unsafe extern "thiscall" fn tirar_de_campo(this: usize, j: usize, modo: i8) {
    let posicao = byte(j, 0x19);
    if posicao < 0 {
        return;
    }
    let casa = casa_da_grade(this, byte(j, 0x102), byte(j, 0x103)) as usize;
    let l = byte(j, 0x27) as isize;
    let lista = |i: isize| (casa as isize + 4 * (11 * l + i)) as *mut usize;
    let n = (casa as isize + 0x58 + l) as *mut i8;
    *lista(byte(j, 0x28) as isize) = *lista(*n as isize - 1);
    *n = (*n).wrapping_sub(1);
    let idx = byte(j, 0x28);
    let movido = *lista(idx as isize);
    if movido != 0 {
        *((movido + 0x28) as *mut i8) = idx;
    }
    let marcado = (j + 0x1aa) as *mut usize;
    if *marcado != 0 {
        let na: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_MARKED_BY);
        if na(*marcado, j) != 0 {
            let tirar: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_REMOVE_MARKER);
            tirar(*marcado, j);
        }
        *marcado = 0;
    }
    if byte(j, 0x101) != 0 {
        let mut i: i8 = 0;
        while i < byte(j, 0x101) {
            let q = *((j + 0x1ae + 4 * i as usize) as *const usize);
            if *((q + 0x1aa) as *const usize) == j {
                *((q + 0x1aa) as *mut usize) = 0;
            }
            i += 1;
        }
        *((j + 0x101) as *mut i8) = 0;
    }
    if modo == -1 {
        return;
    }
    if modo != -3 && j != *((this + 0xf5a2) as *const usize) && j != *((this + 0xf59e) as *const usize) {
        return;
    }
    if f717410(j) & 0x40 != 0 || f717410(j) & 0x20 != 0 {
        return;
    }
    let lado_j = byte(j, 0x27);
    let bloco_tec = this as isize + 208 * lado_j as isize;
    let tec = *((bloco_tec + 0x904d) as *const usize);
    if tec != 0 {
        let npes = *(0xae2364 as *const i32);
        let id = *(tec as *const i32);
        let gerente = *(0xb63c98 as *const usize);
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
        if id >= npes - 16 && id < npes && byte(tec, 0x5f) != 0 && t9a01(gerente, tec) == 0 {
            if *((bloco_tec + 0x904b) as *const u8) == 0 {
                return;
            }
            let partida = *((this + 0x4792) as *const usize);
            let clube = *((partida + if lado_j == 0 { 0x1c } else { 0x20 }) as *const usize);
            let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
            if selecao(clube) != 0 {
                return;
            }
            let t9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM);
            if t9a05(gerente, *((bloco_tec + 0x904d) as *const usize)) != 0 {
                return;
            }
        }
    }
    let mut melhor: i32 = -10000;
    let mut escolhido = 0usize;
    for k in 0..20usize {
        let q = jogador_do_bloco(this, lado_j, k);
        if byte(q, 0x19) < 0 || q == j {
            continue;
        }
        if escolhido != 0 && (f717410(q) & 1 != 0 || f717410(q) & 4 != 0 || f717410(q) & 2 != 0) {
            continue;
        }
        let r = f717410(q);
        let mut v: i32 = if r & 0x40 != 0 {
            70000
        } else if r & 0x20 != 0 {
            50000
        } else if r & 0x10 != 0 {
            15000
        } else {
            -5000
        };
        if r & 0x80 != 0 || r & 0x800 != 0 {
            v += 5000;
        }
        if v > melhor || escolhido == 0 {
            melhor = v;
            escolhido = q;
        }
    }
    if escolhido == 0 {
        // o original mostra a caixa de erro e encerra (0x5cc7d0, 0x5a8f60, exit(-1))
        *(0xb67a34 as *mut u32) = 0;
        let a: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
        let b: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
        let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
        a();
        b();
        sair(-1);
    }
    *((escolhido + 0x19) as *mut i8) = posicao;
    if f717410(j) == 1 {
        let campo = if lado_j == 0 { 0xf59e } else { 0xf5a2 };
        *((this + campo) as *mut usize) = escolhido;
    }
}

// ---------- quinquagesima primeira leva: marcador direto ----------

/// { 0x6d4330 } (match_eng.cpp): o marcador direto do jogador `k` do lado (indice no bloco do outro
/// lado; -1 para goleiro ou jogador fora). Primeiro procura o papel espelhado: cada setor do jogador
/// exige o oposto (2 ou 4 -> 0x40, 8 -> 0x20, 0x10 -> 0x10, 0x20 -> 8, 0x40 -> 4 ou 2) e um lado
/// espelhado (0x80 <-> 0x800, 0x100 <-> 0x400, 0x200 <-> 0x200). Sem ninguem, o mais perto pela
/// tabela de distancias, com a formacao espelhada (linha e coluna pela mascara, 0x5b73a0).
pub(crate) unsafe extern "thiscall" fn marcador_direto(this: usize, lado_b: i8, k: i8) -> u8 {
    let p = (this as isize + 0x4796 + 0x1be * (20 * lado_b as isize + k as isize)) as usize;
    if byte(p, 0x19) < 0 || f717410(p) & 1 != 0 {
        return 0xff;
    }
    let outro = (lado_b == 0) as i8;
    let rp = f717410(p);
    let rpl = rp as u8;
    for k2 in 0..20usize {
        let q = jogador_do_bloco(this, outro, k2);
        if byte(q, 0x19) < 0 {
            continue;
        }
        let rq = f717410(q);
        let rql = rq as u8;
        if (rpl & 6 != 0 && rql & 0x40 == 0)
            || (rpl & 8 != 0 && rql & 0x20 == 0)
            || (rpl & 0x10 != 0 && rql & 0x10 == 0)
            || (rpl & 0x20 != 0 && rql & 8 == 0)
            || (rpl & 0x40 != 0 && rql & 6 == 0)
        {
            continue;
        }
        let pares = [(0x80u16, 0x800u16), (0x100, 0x400), (0x200, 0x200), (0x400, 0x100), (0x800, 0x80)];
        if pares.iter().any(|&(a, b)| rp & a != 0 && rq & b != 0) {
            return k2 as u8;
        }
    }
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(FN_FORMATION_SPLIT_POSITION_CODE);
    // ponytail: no original setor e lado comecam com lixo da pilha; so ficam sem valor com mascara invalida
    let (mut setor, mut lado_m) = (0u16, 0u16);
    separar(rp as u32, &mut setor, &mut lado_m);
    let linha_p: i8 = match setor {
        2 => 0,
        4 => 1,
        8 => 2,
        0x10 => 3,
        0x20 => 4,
        0x40 => 5,
        _ => lado_b,
    };
    let coluna_p: i8 = match lado_m {
        0x80 => 1,
        0x100 => 3,
        0x200 => 4,
        0x400 => 5,
        0x800 => 7,
        _ => 0,
    };
    let (mut linha_q, mut coluna_q) = (linha_p, k);
    let mut melhor = f32::from_bits(0x447a0000); // 1000
    let mut escolhido: i8 = -1;
    for k2 in 0..20usize {
        let q = jogador_do_bloco(this, outro, k2);
        if byte(q, 0x19) < 0 || f717410(q) & 1 != 0 {
            continue;
        }
        separar(f717410(q) as u32, &mut setor, &mut lado_m);
        linha_q = match setor {
            2 => 6,
            4 => 5,
            8 => 4,
            0x10 => 3,
            0x20 => 2,
            0x40 => 1,
            _ => linha_q,
        };
        coluna_q = match lado_m {
            0x100 => 5,
            0x80 => 7,
            0x200 => 4,
            0x800 => 1,
            0x400 => 3,
            _ => coluna_q,
        };
        let dc = (coluna_p as i32 - coluna_q as i32).abs() as i8;
        let dr = (linha_p as i32 - linha_q as i32).abs() as i8;
        let dist = if (0..=8).contains(&dc) && (0..=11).contains(&dr) {
            f32_em(0xa15e44 + 4 * (dr as usize * 9 + dc as usize))
        } else {
            f32_em(0x96d2dc)
        };
        if escolhido == -1 || dist < melhor {
            melhor = dist;
            escolhido = k2 as i8;
        }
    }
    escolhido as u8
}

// ---------- quadragesima setima leva: posicoes finas para desenhar a partida ----------

/// { 0x6d1770 } (match_eng.cpp): posicoes finas (grade 27 x 36) de cada jogador para desenhar a
/// partida. `saida`: bola (x, y) e, para cada lado e posicao tatica (6 bytes a partir de
/// saida + 2 + 6 x (posicao + 11 x lado)): x e y finos, byte 0 do jogador, papel (0x717410) e
/// a marca 1. Cada casa da grade vira 3 x 3 subcasas: quem tem a bola fica no centro; os outros
/// sorteiam uma subcasa (nunca o centro da casa de quem tem a bola), aceitando uma ja ocupada
/// so depois de 50 tentativas por ocupante. O lado que comeca em cada casa e sorteado.
/// Saida nula ou jogador sem posicao: o original mostra uma caixa de erro e zera 0xb67a34 (aqui,
/// so zera; nunca ocorre em jogo).
pub(crate) unsafe extern "thiscall" fn posicoes_finas(this: usize, saida: *mut u8) {
    if saida.is_null() {
        *(0xb67a34 as *mut u32) = 0;
    }
    *saida = byte(this, 0x8ea7) as u8;
    *saida.add(1) = byte(this, 0x8ea8) as u8;
    for i in 0..22 {
        *saida.add(7 + 6 * i) = 0;
    }
    for x in 0..9i8 {
        for y in 0..12i8 {
            let (bx, by) = (3 * x + 1, 3 * y + 1);
            let casa = casa_da_grade(this, x, y) as usize;
            let mut ocupada = [0i8; 9];
            let primeiro = sortear(2) as i8;
            for lado in [primeiro, (primeiro == 0) as i8] {
                let mut k: i8 = 0;
                while k < byte(casa, 0x58 + lado as usize) {
                    let p = *((casa as isize + 4 * (k as isize + 11 * lado as isize)) as *const usize);
                    let pos = byte(p, 0x19);
                    if pos < 0 {
                        *(0xb67a34 as *mut u32) = 0;
                        k += 1;
                        continue;
                    }
                    let dono = *((this + 0xf582) as *const usize);
                    let (fx, fy) = if p == dono {
                        ocupada[4] = 1;
                        (bx, by)
                    } else {
                        let mut tentativas: i32 = 0;
                        loop {
                            let r = sortear(3) as i8;
                            let c = sortear(3) as i8;
                            tentativas += 1;
                            let dono = *((this + 0xf582) as *const usize);
                            if dono != 0 && byte(dono, 0x102) == x && byte(dono, 0x103) == y && r == 1 && c == 1 {
                                continue;
                            }
                            let i = (3 * r + c) as usize;
                            if ocupada[i] as i32 > (tentativas as i16 as i32) / 50 {
                                continue;
                            }
                            ocupada[i] = 1;
                            break (r + bx - 1, c + by - 1);
                        }
                    };
                    let e = saida.offset(6 * (pos as isize + 11 * lado as isize));
                    *e.add(2) = fx as u8;
                    *e.add(3) = fy as u8;
                    *e.add(4) = *(p as *const u8);
                    (e.add(5) as *mut u16).write_unaligned(f717410(p));
                    *e.add(7) = 1;
                    k += 1;
                }
            }
        }
    }
}

// ---------- quadragesima quarta leva: bola rolando ----------

/// { 0x6bbe40 } (match_eng.cpp) com o comentario injetado: bola solta rolando. So sem dono
/// (0xf582), sem 0xf57a e com impulso (0x8eaa). Lance parado (0x8eb2): zera o impulso e 0x8ea9.
/// Senao, com impulso 1 a bola so anda se sorteio(1) >= sorteio(2); impulso <= 0 nao anda. Ao andar,
/// gasta um de impulso (e um de 0x8ea9, se houver) e da um passo pela direcao (0x8ed8, graus):
/// (220, 320) x - 1; (40, 140) x + 1; (130, 230) y - 1; abaixo de 50 ou acima de 310, y + 1.
/// Fora do campo (x fora de 0..9 ou y fora de 0..12): bola fora (0x6cd490) pelo lado de 0x8eab.
pub(crate) unsafe fn rolar_bola_com(this: usize, chamar: ChamarComentario) {
    let b = |o: usize| (this + o) as *mut i8;
    if *((this + 0xf582) as *const usize) != 0 || *((this + 0xf57a) as *const usize) != 0 || *b(0x8eaa) == 0 {
        return;
    }
    if *b(0x8eb2) != 0 {
        *b(0x8eaa) = 0;
        *b(0x8ea9) = 0;
        return;
    }
    let v = *b(0x8eaa);
    let anda = if v >= 2 {
        true
    } else if v <= 0 {
        false
    } else {
        let a = sortear(v as i32 * v as i32);
        a >= sortear(2)
    };
    if anda {
        *b(0x8eaa) = v.wrapping_sub(1);
        if *b(0x8ea9) != 0 {
            *b(0x8ea9) = (*b(0x8ea9)).wrapping_sub(1);
        }
        let dir = *((this + 0x8ed8) as *const i16);
        if dir > 0xdc && dir < 0x140 {
            *b(0x8ea7) = (*b(0x8ea7)).wrapping_sub(1);
        } else if dir > 0x28 && dir < 0x8c {
            *b(0x8ea7) = (*b(0x8ea7)).wrapping_add(1);
        } else if dir > 0x82 && dir < 0xe6 {
            *b(0x8ea8) = (*b(0x8ea8)).wrapping_sub(1);
        } else if dir < 0x32 || dir > 0x136 {
            *b(0x8ea8) = (*b(0x8ea8)).wrapping_add(1);
        }
    }
    let (x, y) = (*b(0x8ea7), *b(0x8ea8));
    if x < 0 || y < 0 || x >= 9 || y >= 12 {
        bola_fora_com(this, *b(0x8eab) as u8, chamar);
    }
}

/// loose ball (no owner at 0xf582, no 0xf57a) with momentum (0x8eaa). Play stopped: clears the momentum and 0x8ea9.
/// Momentum 1 only moves if draw(1) ≥ draw(2); momentum ≤ 0 does not move. When it moves, it spends one momentum (and
/// one of 0x8ea9) and takes a step along direction 0x8ed8: (220, 320) x − 1, (40, 140) x + 1, (130, 230) y − 1, below
/// 50 or above 310 y + 1. Out of the pitch: ball out (0x6cd490) for the side in 0x8eab
/// Original: `roll_ball` (0x6bbe40, match_eng.cpp, thiscall).
///
/// [pt] { 0x6bbe40 } (match_eng.cpp): bola rolando (ver `rolar_bola_com`).
pub(crate) unsafe extern "thiscall" fn rolar_bola(this: usize) {
    rolar_bola_com(this, &mut |t, a| comentar(t, a))
}

// ---------- quadragesima terceira leva: pontape inicial ----------

/// who kicks off (0x8eae; outside 0..=1, side 0), ball at the centre (4, 5 or 6); every active player goes to their
/// formation square (table 0xf56e + 0x9e for side 0 or 0xf572 + 0xa6 for side 1, 216 bytes per position; low nibble x,
/// high nibble y, clamped to their own half) through 0x6f46b0 and faces forward (0x6f4920). On the kicking side, the
/// player closest to the ball (a position > 10 becomes −1) takes the ball (0x6f0630) and faces the square computed from
/// the last formation square used (same point: the low byte of 0x8ed8, sign-extended). With nobody available, it sets
/// byte 0 of the engine
/// Original: `kick_off` (0x6c51e0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c51e0 } (match_eng.cpp): pontape inicial. Quem sai jogando: 0x8eae (fora de 0..=1, o lado
/// 0); bola no centro (4, 5 ou 6). Todos os ativos vao para a casa da formacao (tabela 0xf56e + 0x9e
/// do lado 0 ou 0xf572 + 0xa6 do lado 1, 216 bytes por posicao; nibble baixo x, alto y, cada um no
/// seu campo) e olham para a frente. Do lado que sai, o mais perto da bola (posicao > 10 vira -1)
/// pega a bola e olha para a casa calculada com a ultima casa da formacao usada (mesmo ponto: o
/// byte baixo de 0x8ed8). Sem ninguem, marca o byte 0 do motor.
pub(crate) unsafe extern "thiscall" fn pontape_inicial(this: usize) {
    let mut sai = byte(this, 0x8eae);
    if !(0..=1).contains(&sai) {
        sai = 0;
    }
    *((this + 0x8eae) as *mut i8) = -1;
    *((this + 0x8ea7) as *mut i8) = 4;
    *((this + 0x8ea8) as *mut i8) = if sai == 0 { 5 } else { 6 };
    let mover: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
    let apontar_para: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_SET_FACING_DIRECTION);
    // ponytail: no original x e y comecam com lixo da pilha; so sao lidos depois de algum ativo
    let (mut x, mut y) = (0i8, 0i8);
    for l in 0..2i8 {
        for k in 0..20 {
            let p = jogador_do_bloco(this, l, k);
            let pos = byte(p, 0x19) as isize;
            if pos < 0 {
                continue;
            }
            let (tabela, campo) = if byte(p, 0x27) == 1 { (0xf572, 0xa6) } else { (0xf56e, 0x9e) };
            let casa = *((*((this + tabela) as *const usize) as isize + 216 * pos + campo) as *const u8);
            x = (casa & 0xf) as i8;
            y = (casa >> 4) as i8;
            if byte(p, 0x27) == 1 {
                if y < 6 {
                    y = 6;
                }
            } else if y >= 6 {
                y = 5;
            }
            mover(p, x, y);
            if l == 0 {
                apontar_para(p, x, y, 4, if y < 9 { y + 3 } else { 11 });
            } else {
                apontar_para(p, x, y, 4, if y > 3 { y - 3 } else { 0 });
            }
        }
    }
    let mut melhor = f32::from_bits(0x461c4000); // 10000
    let mut escolhido = 0usize;
    for k in 0..20 {
        let p = jogador_do_bloco(this, sai, k);
        let pos = byte(p, 0x19);
        if pos < 0 {
            continue;
        }
        if pos > 10 {
            *((p + 0x19) as *mut i8) = -1;
            continue;
        }
        if *((p + 0x19e) as *const usize) == 0 {
            continue;
        }
        let d = f6f09b0(p, byte(this, 0x8ea7), byte(this, 0x8ea8));
        if d < melhor {
            melhor = d;
            escolhido = p;
        }
    }
    if escolhido == 0 {
        *(this as *mut u8) = 1;
        return;
    }
    let pegar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_POSSESSION);
    pegar(escolhido);
    let dx = 4i8.wrapping_sub(x);
    let dy = if sai == 0 { 11i8.wrapping_sub(y) } else { y.wrapping_neg() };
    let v = if dx == 0 && dy == 0 {
        *((this + 0x8ed8) as *const u16)
    } else {
        *((TABELA_GLOBAL as isize + 2 * (dx as isize * 24 + dy as isize)) as *const u16)
    };
    *((escolhido + 0x19a) as *mut i16) = v as u8 as i8 as i16;
}

// ---------- quadragesima segunda leva: forca do time (0x6c0c60, 5 KB) ----------

/// Degrau de multiplicadores pelo atributo `x` (byte com sinal): a primeira regra que vale
/// multiplica pela constante float `k` (0: nenhuma). Regras: ('>', limite), ('<', limite) ou
/// ('*', _) (sempre). `arredondar`: o original guarda em float a cada passo (fld/fmul/fstp).
pub(crate) unsafe fn degrau(v: f64, x: i8, regras: &[(char, i8, usize)], arredondar: bool) -> f64 {
    for &(op, lim, k) in regras {
        let vale = match op {
            '>' => x > lim,
            '<' => x < lim,
            _ => true,
        };
        if vale {
            if k == 0 {
                return v;
            }
            let n = v * f32_em(k) as f64;
            return if arredondar { n as f32 as f64 } else { n };
        }
    }
    v
}

/// sorteio(atributo 0x21 da pessoa) < sorteio(20): multiplica por 0x96d248.
pub(crate) unsafe fn sorte_do_dia(v: f64, pessoa: usize) -> f64 {
    let a = sortear(byte(pessoa, 0x21) as i32);
    if a < sortear(0x14) {
        (v * f32_em(0x96d248) as f64) as f32 as f64
    } else {
        v
    }
}

/// Palavra do motor (com sinal) vezes `k`, truncada de volta na palavra `destino`.
pub(crate) unsafe fn palavra_vezes(this: usize, origem: usize, k: f64, destino: usize) {
    let v = *((this + origem) as *const i16) as f64;
    *((this + destino) as *mut i16) = ftol(v * k) as i16;
}

/// { 0x6c0c60 } (match_eng.cpp): forca do time por lado em sete palavras (0x1da, 0x1de ... 0x1f2,
/// de 4 em 4, + 2 x lado), somando os 20 jogadores do bloco. Por jogador ativo: desgaste,
/// componentes (combinacoes dos floats do jogador, ajustadas pelos atributos da pessoa e por
/// sorteios) e, pelo papel tatico (bits 1..0x40), ajuste da palavra 0x35 e as parcelas.
/// Depois, ajustes pela competicao, pela reputacao dos clubes e pelo lado.
/// Jogador sem ficha (0x69): o original mostra uma caixa de erro e zera 0xb67a34 (aqui, so isso).
pub(crate) unsafe extern "thiscall" fn forca_do_time(this: usize, lado_b: i8) {
    let w = |o: usize| (this as isize + o as isize + 2 * lado_b as isize) as *mut i16;
    for o in (0x1da..=0x1f2).step_by(4) {
        *w(o) = 0;
    }
    let f = |p: usize, o: usize| f32_em(p + o) as f64;
    let (k898, k8a8) = (f64_em(0x967898), f64_em(0x9688a8));
    let (cinquenta, meio) = (f32_em(0x969560) as f64, f32_em(0x968f18) as f64);
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) < 0 {
            continue;
        }
        if *((p + 0x69) as *const usize) == 0 {
            *(0xb67a34 as *mut u32) = 0;
            return;
        }
        let pessoa = *((p + 0x6d) as *const usize);
        let desgastar: unsafe extern "thiscall" fn(usize, f32, i16) = core::mem::transmute(FN_MATCH_PLAYER_DRAIN_STAMINA);
        desgastar(p, (30 - byte(pessoa, 0x3c) as i32) as f32, -1);
        let papel = f717410(p) as u8;
        let palavra5 = |pe: usize| *((pe + 5) as *const i16) as f64;
        // S: chute e finalizacao
        let mut s = ((((((f(p, 0xdd) + f(p, 0xcd)) + f(p, 0xc1)) + f(p, 0x7d)) + cinquenta) * meio + f32_em(0x96d3c8) as f64)
            + f(p, 0x4d) * k898
            - k8a8) as f32 as f64;
        if pessoa != 0 {
            let t = (byte(pessoa, 0x2f) >> 2) as f64 + s;
            let kw = if papel & 4 != 0 || papel & 2 != 0 {
                0x967880
            } else if papel & 8 != 0 {
                0x96d3c0
            } else {
                0x967898
            };
            s = (palavra5(pessoa) * f64_em(kw) + t) as f32 as f64;
            let padrao = [('>', 15, 0x96d3b8), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)];
            let forte = [('>', 15, 0x96d3b0), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)];
            s = degrau(s, byte(pessoa, 0x32), &padrao, true);
            s = degrau(s, byte(pessoa, 0x3c), &[('>', 15, 0x96d3b8), ('<', 6, 0x96a32c), ('<', 10, 0x96b014)], true);
            s = degrau(s, byte(pessoa, 0x44), &forte, true);
            s = degrau(s, byte(pessoa, 0x3e), &padrao, true);
            s = degrau(s, byte(pessoa, 0x1e), &padrao, true);
            s = degrau(s, byte(pessoa, 0x2e), &forte, true);
            s = degrau(s, byte(pessoa, 0x2b), &forte, true);
            let soma = byte(pessoa, 0x36).wrapping_add(byte(pessoa, 0x1b));
            s = degrau(s, soma, &[('>', 0x23, 0x96d3b0), ('<', 10, 0x96a32c), ('<', 0x14, 0x96d3b4)], true);
            s = sorte_do_dia(s, pessoa);
        }
        // T2
        let mut t2 = ((((((f(p, 0xf1) * 2.0 + f(p, 0xc5) * 2.0) + f(p, 0x9d)) + f(p, 0xa9)) + f(p, 0x8d) * f64_em(0x968940)) + f64_em(0x968ff8))
            * f64_em(0x968968)
            + f(p, 0x4d) * k898
            + f64_em(0x96d3a8)
            - k8a8) as f32 as f64;
        if pessoa != 0 {
            let kw = if papel & 0x40 != 0 {
                0x967880
            } else if papel & 0x20 != 0 {
                0x96d3a0
            } else if papel & 0x10 != 0 {
                0x96d398
            } else if papel & 8 != 0 {
                0x96a740
            } else {
                0x967898
            };
            t2 = (palavra5(pessoa) * f64_em(kw) + t2) as f32 as f64;
            t2 = degrau(t2, byte(pessoa, 0x44), &[('>', 15, 0x96d3b8), ('<', 6, 0x96d394)], true);
            t2 = degrau(t2, byte(pessoa, 0x3c), &[('>', 15, 0x96d3b0), ('<', 6, 0x96b014)], true);
            t2 = degrau(t2, byte(pessoa, 0x35), &[('>', 15, 0x96a33c), ('<', 6, 0x96a32c)], true);
            let tres = [('>', 15, 0x96d390), ('<', 6, 0x96d38c), ('<', 10, 0x96d394)];
            t2 = degrau(t2, byte(pessoa, 0x37), &tres, true);
            t2 = degrau(t2, byte(pessoa, 0x43), &tres, true);
            let soma = byte(pessoa, 0x40).wrapping_add(byte(pessoa, 0x36)).wrapping_add(byte(pessoa, 0x26)).wrapping_add(byte(pessoa, 0x1b));
            t2 = degrau(t2, soma, &[('>', 0x4b, 0x96d388), ('>', 0x41, 0x96d3b8), ('>', 0x32, 0x96d3b0), ('>', 0x23, 0), ('>', 0x19, 0x96d3b4), ('*', 0, 0x96d394)], true);
            t2 = degrau(t2, byte(pessoa, 0x1e), &[('>', 15, 0x96a33c), ('<', 10, 0x96b014), ('<', 6, 0x96a32c)], true);
            t2 = degrau(t2, byte(pessoa, 0x33), &[('>', 15, 0x96d3b8), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)], true);
            t2 = degrau(t2, byte(pessoa, 0x3f), &[('>', 15, 0x96d3b0), ('<', 10, 0x96b014), ('<', 5, 0x96a32c)], true);
            t2 = sorte_do_dia(t2, pessoa);
        }
        // T3
        let k940 = f64_em(0x968940);
        let soma3 = (((f(p, 0x95) + f(p, 0x71)) + f(p, 0xb9) * k940) + f(p, 0x99) * 2.0) + f(p, 0x7d) * k940;
        let mut t3 = ((((soma3 + f(p, 0xa9)) + f(p, 0xc1)) + f64_em(0x968ff8)) * f64_em(0x968968)
            + f(p, 0x4d) * k898
            + f64_em(0x96d380)
            - k8a8) as f32 as f64;
        if pessoa != 0 {
            let kw = if papel & 0x40 != 0 {
                0x968918
            } else if papel & 0x20 != 0 {
                0x96d378
            } else if papel & 0x10 != 0 {
                0x96d1e0
            } else if papel & 8 != 0 {
                0x96d370
            } else {
                0x967898
            };
            // em st0, sem guardar em float entre os degraus
            let mut v = palavra5(pessoa) * f64_em(kw) + t3;
            let forte = [('>', 15, 0x96d3b0), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)];
            v = degrau(v, byte(pessoa, 0x44), &forte, false);
            v = degrau(v, byte(pessoa, 0x3c), &forte, false);
            v = degrau(v, byte(pessoa, 0x35), &[('>', 15, 0x96d3b8), ('>', 10, 0x96d3b0), ('<', 6, 0x96d394)], false);
            v = degrau(v, byte(pessoa, 0x31), &[('>', 15, 0x96d368), ('>', 10, 0x96d3b0), ('<', 6, 0x96d3b4)], false);
            let soma = byte(pessoa, 0x36).wrapping_add(byte(pessoa, 0x26)).wrapping_add(byte(pessoa, 0x1b)).wrapping_add(byte(pessoa, 0x40));
            v = degrau(v, soma, &[('>', 0x4b, 0x96d388), ('>', 0x41, 0x96d3b8), ('>', 0x32, 0x96d3b0), ('<', 10, 0x96d320), ('<', 0x19, 0x96d364), ('*', 0, 0x96b014)], false);
            t3 = v as f32 as f64;
            t3 = degrau(t3, byte(pessoa, 0x1e), &[('>', 15, 0x96d388), ('<', 6, 0x96a32c), ('<', 10, 0x96d360)], true);
            t3 = degrau(t3, byte(pessoa, 0x33), &[('>', 15, 0x96d388), ('>', 10, 0x96d3b0), ('<', 6, 0x96d394)], true);
            t3 = degrau(t3, byte(pessoa, 0x40), &[('>', 15, 0x96d3b8)], true);
            t3 = sorte_do_dia(t3, pessoa);
        }
        // C1, C4 (fica no x87, sem arredondar), C5, C6
        let c1 = ((((f(p, 0xd1) + f(p, 0xcd)) + f(p, 0xa5)) + cinquenta) * meio + cinquenta + f(p, 0x4d) * k898 - k8a8) as f32 as f64;
        let v4d = f(p, 0x4d);
        let condicao: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_GET_FOUL_CHANCE);
        let c = condicao(p, 0) as i8 as f64;
        let c4 = ((c + f(p, 0x75)) + cinquenta) * meio + cinquenta + v4d * k898 - k8a8;
        let a = f(p, 0xad) + f(p, 0xcd);
        let dd = v4d * k898;
        let c5 = (((a + f(p, 0xa9)) + cinquenta) * meio + cinquenta + dd - k8a8) as f32 as f64;
        let c6 = (((a + f(p, 0xa5)) + cinquenta) * meio + cinquenta + dd - k8a8) as f32 as f64;
        // papel: (desconto em 0x35, limiar, fatores de S, T2, T3, C1, C4, C5, C6; 0 = parcela ausente)
        let (desconto, limiar, ks): (i16, usize, [usize; 7]) = if papel & 1 != 0 {
            (3, 0x96d35c, [0x96d330, 0, 0, 1, 0x967880, 0, 0x96d330])
        } else if papel & 2 != 0 {
            (13, 0x96d358, [0x9689b8, 0x968918, 0x967880, 0x9689e0, 0x968968, 0x969040, 0x968968])
        } else if papel & 4 != 0 {
            (13, 0x96d358, [1, 0x968f20, 0x967880, 0x968918, 0x968968, 0x969040, 0x968968])
        } else if papel & 8 != 0 {
            (12, 0x96d354, [0x969520, 0x96d328, 0x96d340, 0x968f20, 0x968968, 0x969040, 0x968968])
        } else if papel & 0x10 != 0 {
            (8, 0x96d350, [0x968e00, 0x969040, 0x96d338, 0x967888, 0x968968, 0x969040, 0x968968])
        } else if papel & 0x20 != 0 {
            (11, 0x96d34c, [0x968f20, 0x968978, 0x9689d0, 0x968a98, 0x968968, 0x969040, 0x969510])
        } else if papel & 0x40 != 0 {
            (0x19, 0x96d348, [0x967888, 0x968f88, 0x968940, 0x968a98, 0x968968, 0x968940, 0x968918])
        } else {
            continue;
        };
        let p35 = (p + 0x35) as *mut i16;
        let v = (*p35).wrapping_sub(desconto);
        *p35 = if v as f64 <= f32_em(limiar) as f64 { v.wrapping_add(5) } else { v.wrapping_sub(5) };
        let parcelas = [s, t2, t3, c1, c4, c5, c6];
        for (i, &kk) in ks.iter().enumerate() {
            if kk == 0 {
                continue;
            }
            // 1: sem fator (S do papel 4); C1 do papel 1 entra dobrado
            let x = match (kk, i) {
                (1, 3) => parcelas[3] + parcelas[3],
                (1, _) => parcelas[i],
                _ => parcelas[i] * f64_em(kk),
            };
            *w(0x1da + 4 * i) = (*w(0x1da + 4 * i)).wrapping_add(ftol(x) as i16);
        }
    }
    let partida = *((this + 0x4792) as *const usize);
    // pela competicao (le a palavra do lado 0, grava na do lado)
    if partida != 0 {
        let clube = *((partida + 0x1c) as *const usize);
        let jogo = *((partida + 0x14) as *const usize);
        if clube != 0 && jogo != 0 {
            let comp = *((clube + 0x53) as *const usize);
            if *((jogo + 0x5d) as *const usize) == comp && comp != 0 {
                let id = *(comp as *const u32);
                let g = |a: usize| *(a as *const u32);
                let k = if id == g(KEY_NATION_GERMANY) || id == g(KEY_NATION_SCOTLAND) || id == g(KEY_NATION_BELGIUM) {
                    0x96d390
                } else if id == g(KEY_NATION_SWEDEN) {
                    0x96b010
                } else if id == g(KEY_NATION_NORWAY) || id == g(KEY_NATION_ENGLAND) {
                    0x96d3b8
                } else if id == g(KEY_NATION_HOLLAND) || id == g(KEY_NATION_DENMARK) || id == g(KEY_NATION_PORTUGAL) || id == g(KEY_NATION_FRANCE) {
                    0x96a33c
                } else {
                    0
                };
                if k != 0 {
                    *w(0x1de) = ftol(*((this + 0x1de) as *const i16) as f64 * f32_em(k) as f64) as i16;
                }
            }
        }
    }
    *w(0x1de) = ftol(*w(0x1de) as f64 * f32_em(0x96d3b8) as f64) as i16;
    *w(0x1e2) = ftol(*w(0x1e2) as f64 * f32_em(0x96d3b0) as f64) as i16;
    let jogo = *((partida + 0x14) as *const usize);
    let aplica = if jogo == 0 {
        true
    } else {
        let quem = *(0xae2c74 as *const usize);
        if quem == 0 {
            false
        } else {
            let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
            f(quem, jogo) != 0
        }
    };
    if aplica {
        *w(0x1de) = ftol(*((this + 0x1de) as *const i16) as f64 * f32_em(0x96a33c) as f64) as i16;
    }
    let (casa, fora) = (*((partida + 0x1c) as *const usize), *((partida + 0x20) as *const usize));
    if casa != 0 && fora != 0 && *((casa + 0x80) as *const i16) < 0x9c4 {
        *w(0x1de) = ftol(*w(0x1de) as f64 * f32_em(0x96d368) as f64) as i16;
        *w(0x1e2) = ftol(*w(0x1e2) as f64 * f32_em(0x96d390) as f64) as i16;
    }
    if partida == 0 || casa == 0 || fora == 0 {
        return;
    }
    let dif = (*((casa + 0x80) as *const i16)).wrapping_sub(*((fora + 0x80) as *const i16));
    let (kf88, kafa8) = (f64_em(0x968f88), f64_em(0x96afa8));
    if lado_b == 0 {
        if dif > 1000 {
            palavra_vezes(this, 0x1de, kf88, 0x1de);
            palavra_vezes(this, 0x1e2, kf88, 0x1e2);
            palavra_vezes(this, 0x1da, kf88, 0x1da);
        } else if dif > -250 {
            palavra_vezes(this, 0x1de, kf88, 0x1de);
            palavra_vezes(this, 0x1e2, kafa8, 0x1e2);
            palavra_vezes(this, 0x1da, kafa8, 0x1da);
        } else {
            palavra_vezes(this, 0x1da, kf88, 0x1da);
            palavra_vezes(this, 0x1de, f64_em(0x96a6d8), 0x1de);
        }
        let (k1e2, k1da) = if dif > 0x4e2 {
            (kf88, f64_em(0x9689b8))
        } else if dif > 0x2ee {
            (kf88, f64_em(0x969098))
        } else if dif > 0xfa {
            (kafa8, kf88)
        } else {
            return;
        };
        if byte(this, 0xf5bd) == 0 {
            palavra_vezes(this, 0x1e2, k1e2, 0x1e2);
        }
        palavra_vezes(this, 0x1da, k1da, 0x1da);
    } else if lado_b == 1 && dif < -750 && byte(this, 0xf5f3) == 0 {
        palavra_vezes(this, 0x1e4, kf88, 0x1e4);
        palavra_vezes(this, 0x1dc, kafa8, 0x1dc);
    }
}

// ---------- 168a leva: o fechamento do motor (2): os zagueiros do lado, o arbitro e o jogador da partida ----------

/// 0x5b8dc0 on the side's formation
/// Original: `side_centre_backs_without_conflict` (0x6d5b20, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d5b20 } (thiscall, ret 4, match_eng.cpp): os zagueiros do lado `lado` sem conflito (0x5b8dc0
/// na formação +0xaac5, ou +0x91e2 com lado 0).
pub(crate) unsafe extern "thiscall" fn zagueiros_do_lado_sem_conflito(m: usize, lado: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FORMATION_HAS_NO_WIDE_DEFENDERS);
    f(m + if lado != 0 { 0xaac5 } else { 0x91e2 })
}

/// the referee constructor (zeroes +0 and +4)
/// Original: `construct_referee` (0x6e9460, match_official.cpp, thiscall).
///
/// [pt] { 0x6e9460 } (thiscall, match_official.cpp): o construtor do árbitro: zera o motor (+0) e o oficial
/// (+4). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construir_arbitro(a: usize) -> usize {
    core::ptr::write_unaligned(a as *mut u32, 0);
    core::ptr::write_unaligned((a + 4) as *mut u32, 0);
    a
}

/// the match player destructor (zeroes +0x19e)
/// Original: `destroy_match_player` (0x6ebd60, match_pl.cpp, thiscall).
///
/// [pt] { 0x6ebd60 } (thiscall, match_pl.cpp): o destrutor do jogador da partida: zera o ponteiro do motor
/// (+0x19e).
pub(crate) unsafe extern "thiscall" fn destruir_jogador_da_partida(j: usize) {
    core::ptr::write_unaligned((j + 0x19e) as *mut u32, 0);
}

comparar_valor!(comparar_zagueiros_do_lado_sem_conflito, zagueiros_do_lado_sem_conflito, 376, (lado: i8) -> u8);

pub(crate) unsafe extern "thiscall" fn comparar_construir_arbitro(a: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(377);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(a, 8)], || o(a), || construir_arbitro(a));
    PLACAR.registrar(igual && r == v, || format!("construir_arbitro({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", a, r, mo, v, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_destruir_jogador_da_partida(j: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(378);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(j + 0x19e, 4)], || o(j), || destruir_jogador_da_partida(j));
    PLACAR.registrar(igual, || format!("destruir_jogador_da_partida({:#x}) {:x?} x {:x?}", j, mo, mr));
}

// ---------- 167a leva: o fechamento do motor (1): o gol da bola, a partida abandonada e as vagas do lado ----------

/// 1 with the ball in the top box, −1 in the bottom one, otherwise 0; returns only `al`
/// Original: `ball_goal_end` (0x6d58f0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d58f0 } (thiscall, match_eng.cpp): a bola numa área (casas centrais +0x8ea7 de 2 a 6): 1 na
/// área de cima (linha +0x8ea8 0 ou 1), −1 na de baixo (10 ou 11), senão 0. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn bola_na_area(m: usize) -> i8 {
    let linha = *((m + 0x8ea8) as *const i8);
    let coluna = *((m + 0x8ea7) as *const i8);
    if (2..=6).contains(&coluna) {
        if (0..=1).contains(&linha) {
            return 1;
        }
        if (10..=11).contains(&linha) {
            return -1;
        }
    }
    0
}

/// the abandonment minute × 11 below the clock (16-bit); returns only `al`
/// Original: `match_abandoned` (0x6d5930, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d5930 } (thiscall, match_eng.cpp): a partida foi abandonada? O minuto do abandono (+0xf638,
/// −1 sem abandono) × 11 abaixo do relógio (+0x8ed0), em 16 bits. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn partida_abandonada(m: usize) -> u8 {
    let minuto = *((m + 0xf638) as *const i8);
    if minuto == -1 {
        return 0;
    }
    ((minuto as i16).wrapping_mul(11) < core::ptr::read_unaligned((m + 0x8ed0) as *const i16)) as u8
}

/// 0x5b8960 on the side's formation
/// Original: `side_slots_without_conflict` (0x6d5af0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d5af0 } (thiscall, ret 4, match_eng.cpp): as vagas do lado `lado` sem conflito (0x5b8960 na
/// formação +0xaac5, ou +0x91e2 com lado 0).
pub(crate) unsafe extern "thiscall" fn vagas_do_lado_sem_conflito(m: usize, lado: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FORMATION_HAS_NO_WIDE_MIDFIELD_OR_ATTACK);
    f(m + if lado != 0 { 0xaac5 } else { 0x91e2 })
}

comparar_valor!(comparar_bola_na_area, bola_na_area, 373, () -> i8);
comparar_valor!(comparar_partida_abandonada, partida_abandonada, 374, () -> u8);
comparar_valor!(comparar_vagas_do_lado_sem_conflito, vagas_do_lado_sem_conflito, 375, (lado: i8) -> u8);

