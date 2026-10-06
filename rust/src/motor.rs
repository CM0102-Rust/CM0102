//! Motor de partidas (`match_eng.cpp`, `match_pl.cpp`): primeiras rotinas em Rust.
//!
//! Folhas escolhidas pela cobertura (`cobertura.py`, carreira com o motor ligado): acessores que
//! so leem tabelas do motor. Devolvem so parte de eax (`al` ou `ax`); os bits altos sao lixo que
//! nenhum chamador usa. Cada rotina reproduz exatamente a parte do retorno que o tipo dela define,
//! e a comparacao lado a lado confere essa parte; a impressao digital da carreira pega qualquer
//! chamador que dependesse de mais.
//!
//! O objeto do motor (`this`) tem, entre outros campos:
//!   0x8eaf, 0x8eb0  bytes por lado
//!   0x8ebc, 0x8ec4  ponteiros, um por lado, para tabelas de palavras indexadas por 0..=10
//!   0x8ed8          palavra
//! Tabela global de palavras 24 x N em 0xb67c30.

use crate::comparacao::{Placar, Sorteio};
use crate::desvios::{self, Prologo};
use crate::registro::linha;

const PROLOGOS: [Prologo; N] = [
    // 0x6c38c0: mov edx, dword ptr [esp + 4]; mov al, byte ptr [edx + 0x19]
    Prologo { endereco: 0x6c38c0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8a, 0x42, 0x19] },
    // 0x6c38f0: mov edx, dword ptr [esp + 4]; mov al, byte ptr [edx + 0x19]
    Prologo { endereco: 0x6c38f0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8a, 0x42, 0x19] },
    // 0x6c4180: cmp byte ptr [esp + 4], 1
    Prologo { endereco: 0x6c4180, bytes: &[0x80, 0x7c, 0x24, 0x04, 0x01] },
    // 0x6cb960: mov al, byte ptr [esp + 4]; cmp al, 1
    Prologo { endereco: 0x6cb960, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x01] },
    // 0x6cb6c0: mov al, byte ptr [esp + 0xc]; mov dl, byte ptr [esp + 4]
    Prologo { endereco: 0x6cb6c0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x8a, 0x54, 0x24, 0x04] },
    // 0x6cb930: mov al, byte ptr [esp + 4]; cmp al, 1
    Prologo { endereco: 0x6cb930, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x01] },
    // 0x70dfa0: mov al, byte ptr [ecx + 0x27]; mov dl, 6
    Prologo { endereco: 0x70dfa0, bytes: &[0x8a, 0x41, 0x27, 0xb2, 0x06] },
    // 0x717410: mov eax, ecx; mov ecx, dword ptr [eax + 0x19e]
    Prologo { endereco: 0x717410, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] },
    // 0x6c4130: push ebx; push esi; mov esi, ecx; mov ecx, dword ptr [esp + 0xc]
    Prologo { endereco: 0x6c4130, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x0c] },
    // 0x6cfda0: push esi; mov esi, dword ptr [esp + 8]
    Prologo { endereco: 0x6cfda0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    // 0x70df40: mov al, byte ptr [ecx + 0x27]; cmp al, 1
    Prologo { endereco: 0x70df40, bytes: &[0x8a, 0x41, 0x27, 0x3c, 0x01] },
    // 0x6f5b20: mov al, byte ptr [esp + 4]; cmp al, 1
    Prologo { endereco: 0x6f5b20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x01] },
    // 0x5b57c0: movsx eax, byte ptr [esp + 4]
    Prologo { endereco: 0x5b57c0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    // 0x6f4600: push esi; mov esi, ecx; mov al, byte ptr [esi + 0x19]
    Prologo { endereco: 0x6f4600, bytes: &[0x56, 0x8b, 0xf1, 0x8a, 0x46, 0x19] },
    // 0x6bce20: push esi; mov esi, ecx; mov ecx, dword ptr [esp + 8]
    Prologo { endereco: 0x6bce20, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08] },
    // 0x6f4920: mov dl, byte ptr [esp + 0x10]; push ebx
    Prologo { endereco: 0x6f4920, bytes: &[0x8a, 0x54, 0x24, 0x10, 0x53] },
    // 0x6f56c0: mov ax, word ptr [esp + 8]
    Prologo { endereco: 0x6f56c0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x08] },
    // 0x6fa140: mov al, byte ptr [esp + 4]; mov dl, byte ptr [ecx + 0x102]
    Prologo { endereco: 0x6fa140, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x8a, 0x91, 0x02, 0x01, 0x00, 0x00] },
    // 0x6cd330: sub esp, 8; push ebx; mov ebx, dword ptr [esp + 0x14]
    Prologo { endereco: 0x6cd330, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x14] },
    // 0x6cb9b0: push ebx; push esi; mov esi, dword ptr [esp + 0xc]
    Prologo { endereco: 0x6cb9b0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    // 0x713e90: push ebx; mov ebx, dword ptr [esp + 0x20]
    Prologo { endereco: 0x713e90, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x20] },
];
const N: usize = 21;

const TABELA_GLOBAL: usize = 0xb67c30;

unsafe fn byte(p: usize, off: usize) -> i8 {
    *((p + off) as *const i8)
}

/// Valor do jogador numa tabela de 11 palavras (uma por lado), pela posicao dele (0..=10).
unsafe fn tabela_por_posicao(this: usize, jogador: usize, campo: usize) -> u16 {
    let posicao = byte(jogador, 0x19);
    if !(0..=10).contains(&posicao) {
        return 0;
    }
    let lado = byte(jogador, 0x27) as isize;
    let tabela = *((this as isize + campo as isize + lado * 4) as *const usize) as *const u16;
    *tabela.add(posicao as usize)
}

/// { 0x6c38c0 } (match_eng.cpp)
unsafe extern "thiscall" fn tabela_a(this: usize, jogador: usize) -> u16 {
    tabela_por_posicao(this, jogador, 0x8ebc)
}

/// { 0x6c38f0 } (match_eng.cpp)
unsafe extern "thiscall" fn tabela_b(this: usize, jogador: usize) -> u16 {
    tabela_por_posicao(this, jogador, 0x8ec4)
}

/// { 0x6c4180 } (match_eng.cpp): diferenca entre um byte do lado e o valor dado.
unsafe extern "thiscall" fn diferenca_lado(this: usize, lado: u8, v: u8) -> u8 {
    if lado == 1 {
        (byte(this, 0x8eb0) as u8).wrapping_sub(v)
    } else {
        v.wrapping_sub(byte(this, 0x8eaf) as u8)
    }
}

/// { 0x6cb960 } (match_eng.cpp): faixa 2..=6 em `b` e, conforme o lado, 10..=11 ou 0..=1 em `c`.
unsafe extern "stdcall" fn faixa(lado: u8, b: i8, c: i8) -> u8 {
    let alvo = if lado == 1 { 10..=11 } else { 0..=1 };
    ((2..=6).contains(&b) && alvo.contains(&c)) as u8
}

/// { 0x6cb6c0 } (match_eng.cpp): palavra pela diferenca de duas coordenadas.
unsafe extern "thiscall" fn por_diferenca(this: usize, a: i8, b: i8, c: i8, d: i8) -> u16 {
    let (dl, dc) = (c.wrapping_sub(a), d.wrapping_sub(b));
    if dl == 0 && dc == 0 {
        return *((this + 0x8ed8) as *const u16);
    }
    *((TABELA_GLOBAL as isize + 2 * (dl as isize * 24 + dc as isize)) as *const u16)
}

/// { 0x6cb930 } (match_eng.cpp)
unsafe extern "stdcall" fn limite(lado: u8, v: i8) -> u8 {
    (if lado == 1 { v < 3 } else { lado == 0 && v > 8 }) as u8
}

/// { 0x70dfa0 } (match_pl.cpp)
unsafe extern "thiscall" fn condicao_jogador(this: usize) -> u8 {
    let (lado, v) = (byte(this, 0x27), byte(this, 0x103));
    (if lado == 1 && v >= 6 { true } else { lado == 0 && v < 6 }) as u8
}

/// { 0x714d30 } (match_pl.cpp, 4 bytes, nao desviavel): lado do jogador. Lido direto.
unsafe fn lado(jogador: usize) -> i8 {
    byte(jogador, 0x27)
}

/// { 0x717410 } (match_pl.cpp): `tabela_a` do motor do jogador (campo 0x19e), ou 0 sem motor.
/// Chama 0x6c38c0 pelo endereco do jogo (Rust ou original, conforme o modo).
unsafe extern "thiscall" fn tabela_do_jogador(jogador: usize) -> u16 {
    let motor = *((jogador + 0x19e) as *const usize);
    if motor == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    f(motor, jogador)
}

/// { 0x6c4130 } (match_eng.cpp): como `diferenca_lado`, com o valor e o lado tirados do jogador;
/// 0 se o jogador for o guardado em 0xf582.
unsafe extern "thiscall" fn diferenca_jogador(this: usize, jogador: usize) -> u8 {
    if jogador == *((this + 0xf582) as *const usize) {
        return 0;
    }
    let v = byte(jogador, 0x103) as u8;
    if lado(jogador) == 1 {
        (byte(this, 0x8eb0) as u8).wrapping_sub(v)
    } else {
        v.wrapping_sub(byte(this, 0x8eaf) as u8)
    }
}

/// { 0x6cfda0 } (match_eng.cpp)
unsafe extern "thiscall" fn byte_do_lado(this: usize, jogador: usize) -> u8 {
    let s = lado(jogador);
    let f: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    if f(jogador) == 1 {
        if s == 1 {
            return byte(this, 0x8ead) as u8;
        }
        if s == 0 {
            return byte(this, 0x8eac) as u8;
        }
    }
    let guardado = *((this + if s == 1 { 0xf58a } else { 0xf586 }) as *const usize);
    (jogador == guardado) as u8
}

/// { 0x70df40 } (match_pl.cpp): como `limite`, com o lado e o valor do proprio jogador.
unsafe extern "thiscall" fn limite_jogador(this: usize) -> u8 {
    let (s, v) = (byte(this, 0x27), byte(this, 0x103));
    (if s == 1 && v < 3 { true } else { s == 0 && v > 8 }) as u8
}

/// { 0x6f5b20 } (match_pl.cpp): como `faixa`, com os campos 0x102/0x103 do jogador. Devolve eax inteiro.
unsafe extern "thiscall" fn faixa_jogador(this: usize, lado: u8) -> u32 {
    let (b, c) = (byte(this, 0x102), byte(this, 0x103));
    let alvo = if lado == 1 { 10..=11 } else { 0..=1 };
    ((2..=6).contains(&b) && alvo.contains(&c)) as u32
}

/// { 0x5b57c0 }: byte `i` de uma tabela em +0x5b4 (por posicao; ver 0x6f4600).
unsafe extern "thiscall" fn byte_da_posicao(this: usize, i: i8) -> u8 {
    *((this as isize + 0x5b4 + i as isize) as *const u8)
}

/// { 0x6f4600 } (match_pl.cpp): pela marca da posicao do jogador (bit 2 do byte de 0x5b57c0 no
/// bloco do lado dele dentro do motor: 0x91e2 + 6371 x lado), devolve o campo 0x10a ou o 0x108.
unsafe extern "thiscall" fn campo_pela_posicao(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    let bloco = (motor as isize + 0x91e2 + 6371 * lado(this) as isize) as usize;
    let f: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    if f(bloco, byte(this, 0x19)) & 2 != 0 { byte(this, 0x10a) as u8 } else { byte(this, 0x108) as u8 }
}

/// { 0x6bce20 } (match_eng.cpp): 1 se o jogador pode ser escolhido (fora do ponto (b, c), campo
/// 0x2b abaixo de 2, nao e um dos guardados em 0xf582/0xf57a, 0x6f4600 nao zero e 0x8eb2 zerado).
/// Devolve eax inteiro.
unsafe extern "thiscall" fn pode_escolher(this: usize, j: usize, b: i8, c: i8) -> u32 {
    if j == 0 || (byte(j, 0x102) == b && byte(j, 0x103) == c) || byte(j, 0x2b) >= 2 {
        return 0;
    }
    if j == *((this + 0xf582) as *const usize) || j == *((this + 0xf57a) as *const usize) {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6f4600usize);
    (f(j) != 0 && byte(this, 0x8eb2) == 0) as u32
}

unsafe fn sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x6f4920 } (match_pl.cpp): direcao do jogador em graus, campo 0x19a (0..=359), de (a, b)
/// para (c, d). Mesmo ponto: sorteia 0..359 (salvo dois casos fixos: 0 e 180 graus). Depois
/// um desvio sorteado (+-30) quando 0x10c x 0x105 < sorteio(120), e a diferenca de dois
/// atributos do registro da pessoa (0x3b - 0x30, vezes 2).
unsafe extern "thiscall" fn apontar(this: usize, a: i8, b: i8, c: i8, d: i8) {
    let dir = (this + 0x19a) as *mut u16;
    let (dl, dc) = (c.wrapping_sub(a), d.wrapping_sub(b));
    *dir = if dl == 0 && dc == 0 {
        match (d, c) {
            (11, 4) => 0,
            (0, 4) => 180,
            _ => sortear(0x168) as u16,
        }
    } else {
        *((TABELA_GLOBAL as isize + 2 * (dl as isize * 24 + dc as isize)) as *const u16)
    };
    let (x, y) = (byte(this, 0x10c) as i32, byte(this, 0x105) as i32);
    if x * y < sortear(0x78) {
        let p = sortear(0x1e);
        let q = sortear(0x1e);
        *dir = dir.read().wrapping_add(p.wrapping_sub(q) as u16);
    }
    let pessoa = *((this + 0x6d) as *const usize);
    let ajuste = (byte(pessoa, 0x3b) as i16).wrapping_sub(byte(pessoa, 0x30) as i16).wrapping_mul(2);
    *dir = dir.read().wrapping_add(ajuste as u16);
    let v = *dir as i16;
    if v < 0 {
        *dir = v.wrapping_add(0x168) as u16;
    } else if v > 0x167 {
        *dir = v.wrapping_sub(0x168) as u16;
    }
}

// ---------- ponto flutuante ----------
//
// O x87 do jogo roda em precisao dupla (padrao do Windows): cada operacao arredonda para 53 bits,
// como f64 em SSE2. O que importa reproduzir e onde o original arredonda para float (32 bits):
// `fstp`+`fld` compara o valor ja arredondado; `fst` seguido de comparacao compara o de 53 bits.
// As constantes sao lidas do proprio .rdata do jogo.

unsafe fn f32_em(a: usize) -> f32 {
    *(a as *const f32)
}
unsafe fn f64_em(a: usize) -> f64 {
    *(a as *const f64)
}

/// { 0x6f56c0 } (match_pl.cpp): desgaste do jogador, float em 0x4d (nunca abaixo de zero).
/// codigo -1: `x` x (12, 10, 8 ou 6 pela faixa do valor atual: acima de 9500, 8750, 7750) / 11.
/// Outros codigos: (200 - 8,5 x float de 0xd5) x fator do codigo (switch do original).
unsafe extern "thiscall" fn desgastar(this: usize, x: f32, codigo: i16) {
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

/// `_ftol` { 0x945640 }: trunca para inteiro (64 bits; o jogo usa a parte baixa).
fn ftol(x: f64) -> i32 {
    x as i64 as i32
}

/// { 0x6fa140 } (match_pl.cpp): o jogador esta virado para (a, b)? Compara a direcao dele (0x19a)
/// com o angulo da tabela, com tolerancia de 0x10c x 2,25 + 45 graus.
unsafe extern "thiscall" fn virado_para(this: usize, a: i8, b: i8) -> u8 {
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

// ---------- quinta leva ----------

/// { 0x6cd330 } (match_eng.cpp): percorre os jogadores do lado `lado` no grupo `grupo` (quantidade
/// em grupo[0x58 + lado], ponteiros em grupo + lado x 44). Para cada um ativo (0x2b <= 1, posicao
/// >= 0): com `modo` 2 so conta; senao pergunta a 0x6f5c30 (pelo endereco do jogo) e, ao primeiro
/// sim, zera 0x1a5/0x1a6 deste objeto e devolve 1. Devolve a contagem (so cresce no modo 2).
unsafe extern "thiscall" fn procurar_no_lado(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(0x6f5c30usize);
    procurar_com(this, lado, grupo, a3, a4, a5, a6, modo, &mut |p, w, x, y, z| f(p, w, x, y, z))
}

#[allow(clippy::too_many_arguments)]
unsafe fn procurar_com(
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
unsafe extern "stdcall" fn alcanca(j: usize, b: i8, c: i8) -> u8 {
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
const LANCES_IGNORADOS: [i16; 11] = [0x5b, 0x5c, 0x3d, 0x36, 0x34, 0x37, 0x5a, 0x33, 0x35, 0x39, 3];

/// { 0x713e90 } (match_pl.cpp): classifica um lance pelo angulo entre a direcao do jogador (0x19a)
/// e o alvo (0x6cb6c0 do motor, pelo endereco do jogo). Dentro da tolerancia
/// 90 - (18 x `forca` - float de 0xe5): devolve o byte 0x2c. Fora: 0 (um lado) ou 1 (outro), e
/// grava em `saida` (20 - atributo da pessoa) x 12 (0x30 ou 0x3b do registro).
unsafe extern "thiscall" fn classificar_lance(this: usize, codigo: i16, a: i8, b: i8, c: i8, d: i8, forca: f32, saida: *mut u16) -> u8 {
    *saida = 0;
    if LANCES_IGNORADOS.contains(&codigo) {
        return 2;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16 = core::mem::transmute(0x6cb6c0usize);
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

// ---------- comparacao ----------

static mut ORIGINAIS: [usize; N] = [0; N];
static PLACAR: Placar = Placar::novo("motor");

unsafe fn orig<F: Copy>(i: usize) -> F {
    core::mem::transmute_copy(&ORIGINAIS[i])
}

unsafe extern "thiscall" fn comparar_tabela_a(this: usize, j: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u16>(0)(this, j);
    let n = tabela_a(this, j);
    PLACAR.registrar(r == n, || format!("tabela_a(pos {}, lado {}) original={} rust={}", byte(j, 0x19), byte(j, 0x27), r, n));
    r
}
unsafe extern "thiscall" fn comparar_tabela_b(this: usize, j: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u16>(1)(this, j);
    let n = tabela_b(this, j);
    PLACAR.registrar(r == n, || format!("tabela_b(pos {}, lado {}) original={} rust={}", byte(j, 0x19), byte(j, 0x27), r, n));
    r
}
unsafe extern "thiscall" fn comparar_diferenca(this: usize, l: u8, v: u8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u8, u8) -> u8>(2)(this, l, v);
    let n = diferenca_lado(this, l, v);
    PLACAR.registrar(r == n, || format!("diferenca_lado({}, {}) original={} rust={}", l, v, r, n));
    r
}
unsafe extern "stdcall" fn comparar_faixa(l: u8, b: i8, c: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(u8, i8, i8) -> u8>(3)(l, b, c);
    let n = faixa(l, b, c);
    PLACAR.registrar(r == n, || format!("faixa({}, {}, {}) original={} rust={}", l, b, c, r, n));
    r
}
unsafe extern "thiscall" fn comparar_por_diferenca(this: usize, a: i8, b: i8, c: i8, d: i8) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16>(4)(this, a, b, c, d);
    let n = por_diferenca(this, a, b, c, d);
    PLACAR.registrar(r == n, || format!("por_diferenca({}, {}, {}, {}) original={} rust={}", a, b, c, d, r, n));
    r
}
unsafe extern "stdcall" fn comparar_limite(l: u8, v: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(u8, i8) -> u8>(5)(l, v);
    let n = limite(l, v);
    PLACAR.registrar(r == n, || format!("limite({}, {}) original={} rust={}", l, v, r, n));
    r
}
unsafe extern "thiscall" fn comparar_condicao(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(6)(this);
    let n = condicao_jogador(this);
    PLACAR.registrar(r == n, || format!("condicao_jogador(lado {}, v {}) original={} rust={}", byte(this, 0x27), byte(this, 0x103), r, n));
    r
}

unsafe extern "thiscall" fn comparar_tabela_do_jogador(j: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u16>(7)(j);
    let n = tabela_do_jogador(j);
    PLACAR.registrar(r == n, || format!("tabela_do_jogador(motor {:#x}) original={} rust={}", *((j + 0x19e) as *const usize), r, n));
    r
}
unsafe extern "thiscall" fn comparar_diferenca_jogador(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(8)(this, j);
    let n = diferenca_jogador(this, j);
    PLACAR.registrar(r == n, || format!("diferenca_jogador(lado {}, v {}) original={} rust={}", lado(j), byte(j, 0x103), r, n));
    r
}
unsafe extern "thiscall" fn comparar_byte_do_lado(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(9)(this, j);
    let n = byte_do_lado(this, j);
    PLACAR.registrar(r == n, || format!("byte_do_lado(lado {}) original={} rust={}", lado(j), r, n));
    r
}
unsafe extern "thiscall" fn comparar_limite_jogador(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(10)(this);
    let n = limite_jogador(this);
    PLACAR.registrar(r == n, || format!("limite_jogador(lado {}, v {}) original={} rust={}", lado(this), byte(this, 0x103), r, n));
    r
}
unsafe extern "thiscall" fn comparar_faixa_jogador(this: usize, l: u8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u8) -> u32>(11)(this, l);
    let n = faixa_jogador(this, l);
    PLACAR.registrar(r == n, || format!("faixa_jogador({}, {}, {}) original={} rust={}", l, byte(this, 0x102), byte(this, 0x103), r, n));
    r
}

unsafe extern "thiscall" fn comparar_byte_da_posicao(this: usize, i: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u8>(12)(this, i);
    let n = byte_da_posicao(this, i);
    PLACAR.registrar(r == n, || format!("byte_da_posicao({}) original={} rust={}", i, r, n));
    r
}
unsafe extern "thiscall" fn comparar_campo_pela_posicao(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(13)(this);
    let n = campo_pela_posicao(this);
    PLACAR.registrar(r == n, || format!("campo_pela_posicao(lado {}, pos {}) original={} rust={}", lado(this), byte(this, 0x19), r, n));
    r
}
unsafe extern "thiscall" fn comparar_pode_escolher(this: usize, j: usize, b: i8, c: i8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u32>(14)(this, j, b, c);
    let n = pode_escolher(this, j, b, c);
    PLACAR.registrar(r == n, || format!("pode_escolher({}, {}) original={} rust={}", b, c, r, n));
    r
}
/// Sorteia: original e Rust partem do mesmo estado do gerador e o jogo segue com o da original.
unsafe extern "thiscall" fn comparar_apontar(this: usize, a: i8, b: i8, c: i8, d: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8)>(15);
    let mut regioes = vec![(this + 0x19a, 2)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || f(this, a, b, c, d),
        || crate::aleatorio::sem_contar(|| apontar(this, a, b, c, d)),
    );
    PLACAR.registrar(igual, || format!("apontar({}, {}, {}, {}) original={:x?} rust={:x?}", a, b, c, d, mo, mr));
}

unsafe extern "thiscall" fn comparar_desgastar(this: usize, x: f32, codigo: i16) {
    let f = orig::<unsafe extern "thiscall" fn(usize, f32, i16)>(16);
    let regioes = [(this + 0x4d, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this, x, codigo), || desgastar(this, x, codigo));
    PLACAR.registrar(igual, || format!("desgastar({}, {}, 0xd5={}) original={:x?} rust={:x?}", x, codigo, f32_em(this + 0xd5), mo, mr));
}
unsafe extern "thiscall" fn comparar_virado_para(this: usize, a: i8, b: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8) -> u8>(17)(this, a, b);
    let n = virado_para(this, a, b);
    PLACAR.registrar(r == n, || format!("virado_para({}, {}, dir {}, 0x10c {}) original={} rust={}", a, b, *((this + 0x19a) as *const i16), byte(this, 0x10c), r, n));
    r
}

// --- gravar e reproduzir: 0x6f5c30 (3,9 KB, sorteia e altera jogadores) chamada por 0x6cd330 ---
//
// Na comparacao a original roda e cada chamada dela a 0x6f5c30 e gravada (porta abaixo); a copia
// Rust recebe a gravacao no lugar da rotina, e a sequencia de chamadas tambem e comparada. No
// autoteste (jogadores sinteticos) a porta responde de forma sintetica, sem rodar a rotina real.

static mut PORTA_6F5C30: usize = 0;
static GRAVANDO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static SINTETICO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GRAVACAO: std::sync::Mutex<Vec<(usize, [u32; 4], u8)>> = std::sync::Mutex::new(Vec::new());

// 0x6f5c30: sub esp, 0x218
const PROLOGO_6F5C30: Prologo = Prologo { endereco: 0x6f5c30, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] };

unsafe extern "thiscall" fn porta_6f5c30(p: usize, a: u32, b: u32, c: u32, d: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let r = if SINTETICO.load(Relaxed) {
        ((p as u32 ^ a ^ b.wrapping_mul(3) ^ c.wrapping_mul(5) ^ d.wrapping_mul(7)) >> 4 & 1) as u8
    } else {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(PORTA_6F5C30);
        f(p, a, b, c, d)
    };
    if GRAVANDO.load(Relaxed) {
        GRAVACAO.lock().unwrap_or_else(|e| e.into_inner()).push((p, [a, b, c, d], r));
    }
    r
}

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_procurar(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let f = orig::<unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8>(18);
    GRAVACAO.lock().unwrap_or_else(|e| e.into_inner()).clear();
    GRAVANDO.store(true, Relaxed);
    let repeticao = core::cell::RefCell::new(Vec::new());
    let sequencia_igual = core::cell::Cell::new(true);
    // A copia Rust tem de ver o estado que a original viu: 0x6f5c30 altera o jogador que recebe
    // (0x2b, 0x19), entao esses bytes de cada jogador da lista e a quantidade entram nas regioes.
    let mut regioes = vec![(this + 0x1a5, 2), ((grupo as isize + lado as isize) as usize + 0x58, 1)];
    let lista = (grupo as isize + lado as isize * 44) as *const usize;
    for i in 0..byte((grupo as isize + lado as isize) as usize, 0x58).clamp(0, 11) as usize {
        let p = *lista.add(i);
        regioes.extend([(p + 0x19, 1), (p + 0x2b, 1)]);
    }
    let (r, n, _, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || {
            let r = f(this, lado, grupo, a3, a4, a5, a6, modo);
            GRAVANDO.store(false, Relaxed);
            *repeticao.borrow_mut() = GRAVACAO.lock().unwrap_or_else(|e| e.into_inner()).clone();
            r
        },
        || {
            let gravadas = repeticao.borrow();
            let mut prox = gravadas.iter();
            let n = procurar_com(this, lado, grupo, a3, a4, a5, a6, modo, &mut |p, w, x, y, z| match prox.next() {
                Some(&(gp, ga, gr)) if gp == p && ga == [w, x, y, z] => gr,
                _ => {
                    sequencia_igual.set(false);
                    0
                }
            });
            if prox.next().is_some() {
                sequencia_igual.set(false); // a original chamou mais vezes
            }
            n
        },
    );
    let (todas, sequencia_igual) = (repeticao.borrow().len(), sequencia_igual.get());
    // so o que a propria rotina escreve (0x1a5/0x1a6); os bytes dos jogadores mudam pela 0x6f5c30
    let igual = mo.0[0] == mr.0[0];
    // 0x6f5c30 altera os proximos jogadores da lista; a reproducao devolve so a resposta gravada,
    // sem esses efeitos, e a copia Rust pode querer perguntar por um jogador que a original pulou.
    // Com resultado e memoria iguais isso e inconclusivo, nao divergencia (a carreira inteira, em
    // que a rotina roda de verdade, e a prova). Resultado ou memoria diferentes continuam acusados.
    if !sequencia_igual && igual && r == n {
        static INCONCLUSIVAS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let k = INCONCLUSIVAS.fetch_add(1, Relaxed) + 1;
        if k.is_power_of_two() {
            linha(&format!("motor: procurar_no_lado {} comparacoes inconclusivas (efeito de 0x6f5c30 nos proximos jogadores)", k));
        }
        return r;
    }
    PLACAR.registrar(igual && r == n && sequencia_igual, || {
        format!("procurar_no_lado(lado {}, modo {}, {} chamadas) original={} rust={} seq={} {:x?} {:x?}", lado, modo, todas, r, n, sequencia_igual, mo, mr)
    });
    r
}
unsafe extern "stdcall" fn comparar_alcanca(j: usize, b: i8, c: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, i8, i8) -> u8>(19)(j, b, c);
    let n = alcanca(j, b, c);
    PLACAR.registrar(r == n, || format!("alcanca(lado {}, x {}, y {}, {}, {}) original={} rust={}", lado(j), byte(j, 0x102), byte(j, 0x103), b, c, r, n));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_classificar(this: usize, codigo: i16, a: i8, b: i8, c: i8, d: i8, forca: f32, saida: *mut u16) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, i16, i8, i8, i8, i8, f32, *mut u16) -> u8>(20);
    let (r, n, igual, mo, mr) =
        crate::comparacao::lado_a_lado(&[(saida as usize, 2)], || f(this, codigo, a, b, c, d, forca, saida), || classificar_lance(this, codigo, a, b, c, d, forca, saida));
    PLACAR.registrar(igual && r == n, || format!("classificar_lance({}, {} {} {} {}, {}) original={} rust={} {:x?} {:x?}", codigo, a, b, c, d, forca, r, n, mo, mr));
    r
}

pub unsafe fn instalar(comparando: bool) {
    if comparando {
        if let Some(t) = desvios::desviar("motor_porta_6f5c30 (gravacao)", &PROLOGO_6F5C30, porta_6f5c30 as *const () as usize) {
            PORTA_6F5C30 = t;
        }
    }
    let rust: [usize; N] = [
        tabela_a as *const () as usize,
        tabela_b as *const () as usize,
        diferenca_lado as *const () as usize,
        faixa as *const () as usize,
        por_diferenca as *const () as usize,
        limite as *const () as usize,
        condicao_jogador as *const () as usize,
        tabela_do_jogador as *const () as usize,
        diferenca_jogador as *const () as usize,
        byte_do_lado as *const () as usize,
        limite_jogador as *const () as usize,
        faixa_jogador as *const () as usize,
        byte_da_posicao as *const () as usize,
        campo_pela_posicao as *const () as usize,
        pode_escolher as *const () as usize,
        apontar as *const () as usize,
        desgastar as *const () as usize,
        virado_para as *const () as usize,
        procurar_no_lado as *const () as usize,
        alcanca as *const () as usize,
        classificar_lance as *const () as usize,
    ];
    let comp: [usize; N] = [
        comparar_tabela_a as *const () as usize,
        comparar_tabela_b as *const () as usize,
        comparar_diferenca as *const () as usize,
        comparar_faixa as *const () as usize,
        comparar_por_diferenca as *const () as usize,
        comparar_limite as *const () as usize,
        comparar_condicao as *const () as usize,
        comparar_tabela_do_jogador as *const () as usize,
        comparar_diferenca_jogador as *const () as usize,
        comparar_byte_do_lado as *const () as usize,
        comparar_limite_jogador as *const () as usize,
        comparar_faixa_jogador as *const () as usize,
        comparar_byte_da_posicao as *const () as usize,
        comparar_campo_pela_posicao as *const () as usize,
        comparar_pode_escolher as *const () as usize,
        comparar_apontar as *const () as usize,
        comparar_desgastar as *const () as usize,
        comparar_virado_para as *const () as usize,
        comparar_procurar as *const () as usize,
        comparar_alcanca as *const () as usize,
        comparar_classificar as *const () as usize,
    ];
    let nomes = [
        "motor_tabela_a", "motor_tabela_b", "motor_diferenca_lado", "motor_faixa", "motor_por_diferenca", "motor_limite",
        "motor_condicao_jogador", "motor_tabela_do_jogador", "motor_diferenca_jogador", "motor_byte_do_lado",
        "motor_limite_jogador", "motor_faixa_jogador", "motor_byte_da_posicao", "motor_campo_pela_posicao",
        "motor_pode_escolher", "motor_apontar", "motor_desgastar", "motor_virado_para", "motor_procurar_no_lado",
        "motor_alcanca", "motor_classificar_lance",
    ];
    for i in 0..N {
        if let Some(t) = desvios::desviar(nomes[i], &PROLOGOS[i], if comparando { comp[i] } else { rust[i] }) {
            ORIGINAIS[i] = t;
        }
    }
    if comparando {
        linha("motor: modo comparacao (original x Rust)");
    }
}

/// Simulacao no modo original: as mesmas rotinas desviadas para a propria original (pilha particular).
pub unsafe fn instalar_passagem() {
    for (i, p) in PROLOGOS.iter().enumerate() {
        desvios::desviar(&format!("motor {} (original isolada)", i), p, desvios::PASSAGEM);
    }
}

/// Entradas sinteticas: um objeto do motor e um jogador falsos, com tabelas proprias; posicoes,
/// lados e coordenadas variando, inclusive fora das faixas validas. Roda pelas funcoes de
/// comparacao, que chamam a original e a versao Rust com as mesmas entradas.
pub unsafe fn autoteste(casos: u64) {
    let mut s = Sorteio(0x4d4f544f52);
    let mut motor = vec![0u8; 0xf590];
    let mut tabelas = vec![vec![0u16; 11]; 4];
    let mut jogador = vec![0u8; 0x1a4];
    let outro = vec![0u8; 0x1a4];
    let mut pessoa = vec![0u8; 0x46];
    let mut grupo = vec![0u8; 0x60];
    let mut jogadores = vec![vec![0u8; 0x1a8]; 22];
    SINTETICO.store(true, std::sync::atomic::Ordering::Relaxed);
    PLACAR.autoteste(|| {
        for _ in 0..casos {
            for t in tabelas.iter_mut() {
                for v in t.iter_mut() {
                    *v = s.proximo() as u16;
                }
            }
            let m = motor.as_mut_ptr() as usize;
            for (lado, campo) in [(0usize, 0x8ebc), (1, 0x8ebc), (0, 0x8ec4), (1, 0x8ec4)] {
                let t = &tabelas[(campo - 0x8ebc) / 4 + lado];
                *((m + campo + lado * 4) as *mut usize) = t.as_ptr() as usize;
            }
            let x = s.proximo();
            *((m + 0x8eaf) as *mut u8) = x as u8;
            *((m + 0x8eb0) as *mut u8) = (x >> 8) as u8;
            *((m + 0x8ed8) as *mut u16) = (x >> 16) as u16;
            let j = jogador.as_mut_ptr() as usize;
            let y = s.proximo();
            *((j + 0x19) as *mut i8) = (y % 15) as i8 - 2; // -2..=12
            *((j + 0x27) as *mut i8) = ((y >> 8) % 2) as i8;
            *((j + 0x103) as *mut i8) = ((y >> 16) % 13) as i8 - 2;
            let lado = [0u8, 1, 2, 255][((y >> 24) % 4) as usize];
            comparar_tabela_a(m, j);
            comparar_tabela_b(m, j);
            comparar_diferenca(m, lado, (y >> 32) as u8);
            comparar_faixa(lado, ((y >> 40) % 10) as i8 - 1, ((y >> 44) % 14) as i8 - 1);
            comparar_limite(lado, ((y >> 48) % 14) as i8 - 2);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((y >> 52) % 4) as usize];
            comparar_condicao(j);
            // coordenadas com diferencas de -5..=5 (a tabela global e lida do proprio jogo)
            let z = s.proximo();
            let (a, b) = ((z % 20) as i8, ((z >> 8) % 20) as i8);
            let (c, d) = (a + ((z >> 16) % 11) as i8 - 5, b + ((z >> 24) % 11) as i8 - 5);
            comparar_por_diferenca(m, a, b, c, d);
            // rotinas que tiram lado e valores do proprio jogador
            let w = s.proximo();
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][(w % 4) as usize];
            *((j + 0x19) as *mut i8) = ((w >> 4) % 15) as i8 - 2;
            *((j + 0x102) as *mut i8) = ((w >> 8) % 10) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((w >> 12) % 15) as i8 - 2;
            let o = outro.as_ptr() as usize;
            for (campo, bit) in [(0xf582usize, 20), (0xf586, 22), (0xf58a, 24)] {
                *((m + campo) as *mut usize) = if (w >> bit) & 1 == 1 { j } else { o };
            }
            *((j + 0x19e) as *mut usize) = if (w >> 26) % 4 == 0 { 0 } else { m };
            *((m + 0x8eac) as *mut u8) = (w >> 32) as u8;
            *((m + 0x8ead) as *mut u8) = (w >> 40) as u8;
            comparar_diferenca_jogador(m, j);
            comparar_limite_jogador(j);
            comparar_faixa_jogador(j, lado);
            // as que passam por 0x6c38c0 so recebem lado 0 ou 1 (indice da tabela do lado)
            let lado_real = *((j + 0x27) as *const i8);
            *((j + 0x27) as *mut i8) = ((w >> 16) % 2) as i8;
            comparar_tabela_do_jogador(j);
            comparar_byte_do_lado(m, j);
            // terceira leva (com o lado 0 ou 1 ainda valendo: 0x6f4600 indexa o bloco do lado).
            // 0x6f4600 sempre tem motor (com 0x19e zerado a original le o endereco 0 e cai).
            *((j + 0x19e) as *mut usize) = m;
            let u = s.proximo();
            for k in 0..16 {
                *((m + 0x91e2 + 6371 * ((j + 0x27) as *const u8).read() as usize + 0x5b4 + k) as *mut u8) = (u >> (k * 4)) as u8;
            }
            *((j + 0x108) as *mut u8) = (u >> 3) as u8;
            *((j + 0x10a) as *mut u8) = (u >> 11) as u8;
            *((j + 0x2b) as *mut i8) = ((u >> 19) % 5) as i8 - 1;
            *((m + 0x8eb2) as *mut u8) = ((u >> 22) % 3 == 0) as u8;
            *((m + 0xf57a) as *mut usize) = if (u >> 24) % 5 == 0 { j } else { o };
            *((j + 0x19) as *mut i8) = ((u >> 27) % 11) as i8;
            comparar_byte_da_posicao(m + 0x91e2, ((u >> 31) % 16) as i8);
            comparar_campo_pela_posicao(j);
            comparar_pode_escolher(m, if (u >> 33) % 8 == 0 { 0 } else { j }, ((u >> 36) % 10) as i8 - 1, ((u >> 40) % 15) as i8 - 2);
            *((j + 0x27) as *mut i8) = lado_real;
            // direcao: coordenadas vizinhas (inclusive o mesmo ponto e os casos fixos), atributos
            let v = s.proximo();
            let pp = pessoa.as_mut_ptr() as usize;
            *((j + 0x6d) as *mut usize) = pp;
            *((pp + 0x30) as *mut i8) = ((v >> 4) % 21) as i8;
            *((pp + 0x3b) as *mut i8) = ((v >> 9) % 21) as i8;
            *((j + 0x10c) as *mut i8) = ((v >> 14) % 21) as i8;
            *((j + 0x105) as *mut i8) = ((v >> 19) % 21) as i8;
            *((j + 0x19a) as *mut u16) = (v >> 24) as u16;
            let (a, b) = (((v >> 40) % 12) as i8, ((v >> 44) % 12) as i8);
            let (c, d) = if v % 3 == 0 {
                (a, b) // mesmo ponto
            } else if v % 3 == 1 {
                ([4i8, 4, 3][((v >> 48) % 3) as usize], [0i8, 11, 5][((v >> 50) % 3) as usize])
            } else {
                (a + ((v >> 52) % 11) as i8 - 5, b + ((v >> 56) % 11) as i8 - 5)
            };
            let (a, b) = if v % 3 == 1 { (c, d) } else { (a, b) };
            comparar_apontar(j, a, b, c, d);
            // quarta leva (ponto flutuante): valores em volta das faixas, negativos e zero
            let q = s.proximo();
            let faixas = [9500.0f32, 8750.0, 7750.0, 0.0, 10000.0, 100.0];
            let base = faixas[(q % 6) as usize];
            *((j + 0x4d) as *mut f32) = base + ((q >> 8) % 2001) as f32 / 10.0 - 100.0;
            *((j + 0xd5) as *mut f32) = ((q >> 20) % 400) as f32 / 10.0 - 5.0;
            let x = ((q >> 30) % 5000) as f32 / 7.0;
            let codigo = [-1i16, 19, 21, 33, 36, 46, 20, 47, 18, 0, -2, 1000][((q >> 44) % 12) as usize];
            comparar_desgastar(j, x, codigo);
            *((j + 0x10c) as *mut i8) = ((q >> 50) % 25) as i8 - 2;
            *((j + 0x19a) as *mut i16) = ((q >> 54) % 400) as i16 - 20;
            comparar_virado_para(j, byte(j, 0x102).wrapping_add(((q >> 58) % 11) as i8 - 5), byte(j, 0x103).wrapping_add(((q >> 61) % 7) as i8 - 3));
            // quinta leva
            let g = s.proximo();
            let gp = grupo.as_mut_ptr() as usize;
            for (k, jog) in jogadores.iter_mut().enumerate() {
                let jp = jog.as_mut_ptr() as usize;
                *((jp + 0x2b) as *mut i8) = ((g >> (k % 32)) % 4) as i8 - 1;
                *((jp + 0x19) as *mut i8) = ((g >> ((k + 7) % 40)) % 13) as i8 - 2;
                *((gp + (k / 11) * 44 + (k % 11) * 4) as *mut usize) = jp;
            }
            *((gp + 0x58) as *mut i8) = (g % 13) as i8 - 1;
            *((gp + 0x59) as *mut i8) = ((g >> 5) % 13) as i8 - 1;
            let lado_g = ((g >> 10) % 2) as i8;
            let modo = [0i8, 1, 2, 2][((g >> 12) % 4) as usize];
            comparar_procurar(m, lado_g, gp, (g >> 14) as u32, (g >> 20) as u32, (g >> 26) as u32, (g >> 32) as u32, modo);
            let h = s.proximo();
            *((j + 0x27) as *mut i8) = [0i8, 1, 2][(h % 3) as usize];
            *((j + 0x102) as *mut i8) = ((h >> 2) % 9) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((h >> 6) % 14) as i8 - 1;
            comparar_alcanca(j, ((h >> 10) % 9) as i8 - 1, ((h >> 14) % 14) as i8 - 1);
            *((j + 0x27) as *mut i8) = lado_real;
            // lance: angulo pela tabela global (via motor falso), forca e atributos variados
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x19a) as *mut i16) = ((h >> 18) % 400) as i16 - 20;
            *((j + 0x2c) as *mut u8) = (h >> 27) as u8;
            *((j + 0xe5) as *mut f32) = ((h >> 30) % 300) as f32 / 7.0;
            let codigo = [0i16, 1, 3, 0x33, 0x5b, 0x40, 100, -1][((h >> 40) % 8) as usize];
            let (a, b) = (((h >> 43) % 12) as i8, ((h >> 47) % 12) as i8);
            let mut saida = 0xffffu16;
            comparar_classificar(j, codigo, a, b, a + ((h >> 51) % 11) as i8 - 5, b + ((h >> 55) % 11) as i8 - 5, ((h >> 59) % 32) as f32 / 3.0, &mut saida);
        }
    });
    SINTETICO.store(false, std::sync::atomic::Ordering::Relaxed);
}
