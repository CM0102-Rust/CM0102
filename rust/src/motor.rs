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
    // 0x6f09b0: movsx eax, byte ptr [ecx + 0x103]
    Prologo { endereco: 0x6f09b0, bytes: &[0x0f, 0xbe, 0x81, 0x03, 0x01, 0x00, 0x00] },
    // 0x6fa1e0: mov al, byte ptr [ecx + 0x27]; push esi; test al, al
    Prologo { endereco: 0x6fa1e0, bytes: &[0x8a, 0x41, 0x27, 0x56, 0x84, 0xc0] },
    // 0x6cb780: push ebx; push ebp; push esi; push edi; mov edi, dword ptr [esp + 0x14]
    Prologo { endereco: 0x6cb780, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    // 0x6cbc00: sub esp, 0xc; movsx eax, byte ptr [esp + 0x18]
    Prologo { endereco: 0x6cbc00, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x44, 0x24, 0x18] },
    // 0x6f9f90: push ebx; push ebp; push esi; mov esi, ecx
    Prologo { endereco: 0x6f9f90, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] },
    // 0x6f4650: mov al, byte ptr [ecx + 0x27]; test al, al
    Prologo { endereco: 0x6f4650, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] },
    // 0x5b5270: movsx edx, byte ptr [esp + 4]
    Prologo { endereco: 0x5b5270, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    // 0x716270: push esi; mov esi, ecx; mov al, byte ptr [esi + 0x19]
    Prologo { endereco: 0x716270, bytes: &[0x56, 0x8b, 0xf1, 0x8a, 0x46, 0x19] },
    // 0x716310: push esi; mov esi, ecx; mov ecx, dword ptr [esi + 0x19e]
    Prologo { endereco: 0x716310, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x8e, 0x9e, 0x01, 0x00, 0x00] },
    // 0x6c2be0: sub esp, 8; push ebx; push ebp
    Prologo { endereco: 0x6c2be0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    // 0x6c3aa0: push ebx; push esi; push edi; mov edi, ecx
    Prologo { endereco: 0x6c3aa0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0xf9] },
    // 0x6f4aa0: sub esp, 0x10; push ebx; push esi
    Prologo { endereco: 0x6f4aa0, bytes: &[0x83, 0xec, 0x10, 0x53, 0x56] },
    // 0x6f44a0: push esi; mov esi, ecx; push edi; fld dword ptr [esi + 0x4d]
    Prologo { endereco: 0x6f44a0, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0xd9, 0x46, 0x4d] },
    // 0x6ce110: mov eax, dword ptr [ecx + 0xf59e]
    Prologo { endereco: 0x6ce110, bytes: &[0x8b, 0x81, 0x9e, 0xf5, 0x00, 0x00] },
    // 0x6bd5b0: push ebx; push esi; mov esi, dword ptr [esp + 0xc]
    Prologo { endereco: 0x6bd5b0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    // 0x6cbad0: push ebx; push ebp; push esi; push edi; mov edi, dword ptr [esp + 0x14]
    Prologo { endereco: 0x6cbad0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    // 0x6cb910: mov al, byte ptr [esp + 4]; cmp al, 2
    Prologo { endereco: 0x6cb910, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x02] },
    // 0x6cb700: push ecx; push ebx; mov bl, byte ptr [esp + 0x10]
    Prologo { endereco: 0x6cb700, bytes: &[0x51, 0x53, 0x8a, 0x5c, 0x24, 0x10] },
    // 0x6cc4e0: movsx eax, byte ptr [esp + 4]
    Prologo { endereco: 0x6cc4e0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    // decima segunda leva: acessores do estado da partida (match_pl.cpp, 0x700320..0x700410)
    Prologo { endereco: 0x700320, bytes: &[0x8a, 0x81, 0xa7, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700330, bytes: &[0x8a, 0x81, 0xa8, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700340, bytes: &[0x8a, 0x81, 0xa9, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700380, bytes: &[0x8a, 0x81, 0xaa, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700400, bytes: &[0x8a, 0x81, 0xb2, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700350, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xa9, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700360, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xa7, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700370, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xa8, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x700390, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xaa, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x7003a0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x700410, bytes: &[0x8b, 0x81, 0x7a, 0x8f, 0x00, 0x00] },
    Prologo { endereco: 0x7003b0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x80, 0xfa, 0xff] },
    Prologo { endereco: 0x714c60, bytes: &[0x8a, 0x91, 0x01, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6cd660, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] },
    // decima terceira leva
    Prologo { endereco: 0x6cbbb0, bytes: &[0x56, 0x0f, 0xbe, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x6cb650, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x6cd5f0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x6d4280, bytes: &[0x8b, 0x81, 0x92, 0x47, 0x00, 0x00] },
    Prologo { endereco: 0x6bd6b0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x6c3010, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x6c3a00, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c3950, bytes: &[0x51, 0x0f, 0xbe, 0x54, 0x24, 0x08] },
    // decima quarta leva
    Prologo { endereco: 0x6c06b0, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c07a0, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c0a70, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c2750, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] },
];
const N: usize = 66;

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

// ---------- sexta leva: distancias ----------

/// Celula da tabela de distancias (0xa15e44, 9 por linha). Diferencas com o mesmo calculo do
/// original: valor absoluto em 32 bits e depois o byte baixo com sinal.
unsafe fn distancia_tabela(dl: i32, dc: i32) -> f32 {
    let lin = dl.abs() as i8 as isize;
    let col = dc.abs() as i8 as isize;
    *((0xa15e44 + 4 * (lin * 9 + col)) as *const f32)
}

/// { 0x6f09b0 } (match_pl.cpp): distancia (tabela) do jogador ao ponto (a, b).
unsafe extern "thiscall" fn distancia_ao_ponto(this: usize, a: i8, b: i8) -> f32 {
    distancia_tabela(byte(this, 0x103) as i32 - b as i32, byte(this, 0x102) as i32 - a as i32)
}

/// { 0x6fa1e0 } (match_pl.cpp): distancia ao gol que o jogador ataca (coluna 4; linha 11 do lado 0, 0 do outro).
unsafe extern "thiscall" fn distancia_ao_gol(this: usize) -> f32 {
    let alvo = if lado(this) == 0 { 0xb } else { 0 };
    distancia_tabela(byte(this, 0x103) as i32 - alvo, byte(this, 0x102) as i32 - 4)
}

/// { 0x6cb780 } (match_eng.cpp): 0 se o jogador esta impedido de seguir (0x70dfa0, marca do
/// motor na casa dele) ou se algum adversario ativo esta a distancia do gol <= limite; 2 se algum
/// colega ativo esta estritamente mais perto; senao 1. Limite: distancia dele ao gol + 0,5, e mais
/// 1,0 fora da area (arredondado para float a cada passo, como o `fstp` do original). Os blocos
/// de 20 jogadores de cada lado ficam em motor + 0x4796 + 8920 x lado, de 0x1be em 0x1be.
unsafe extern "thiscall" fn mais_perto_do_gol(this: usize, j: usize) -> u8 {
    let f6f4650: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6f4650usize);
    let f70dfa0: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x70dfa0usize);
    let f6fa1e0: unsafe extern "thiscall" fn(usize) -> f32 = core::mem::transmute(0x6fa1e0usize);
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let f6f09b0: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(0x6f09b0usize);
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

// ---------- setima leva: linha de passe ----------

/// Jogadores numa casa (x, y) da grade do motor, pelo `procurar_no_lado` em modo 2 (so conta),
/// chamado pelo endereco do jogo. O 3o argumento e um float (bits), que o modo 2 nao usa.
unsafe fn contar_na_casa(motor: usize, x: i8, y: i8, valor: f32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8 = core::mem::transmute(0x6cd330usize);
    let grupo = (motor as isize + 2 * 45 * (y as isize + 12 * x as isize) + 0x215e) as usize;
    let lado = (byte(motor, 0x8eae) == 0) as i8;
    f(motor, lado, grupo, valor.to_bits(), u32::MAX, u32::MAX, u32::MAX, 2)
}

/// Distancia da tabela para (|dx|, |dy|) dentro dela (coluna 0..=8, linha 0..=11); fora, 1000.
unsafe fn distancia_ou_mil(dx: i32, dy: i32) -> f32 {
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
unsafe extern "thiscall" fn jogadores_na_linha(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let dx = (x2 as i32 - x1 as i32).abs() as i8;
    let dy = (y2 as i32 - y1 as i32).abs() as i8;
    let fora = |x: i8, y: i8| !(0..=8).contains(&x) || !(0..=11).contains(&y);
    let mut n = 0u8;
    if dx == 0 {
        let (inicio, fim) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        let mut y = inicio;
        while y < fim {
            if fora(x1, y) {
                return n;
            }
            n = n.wrapping_add(contar_na_casa(this, x1, y, (y as i32 - inicio as i32) as f32));
            y = y.wrapping_add(1);
        }
        return n;
    }
    if dy == 0 {
        let (inicio, fim) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let mut x = inicio;
        while x < fim {
            if fora(x, y1) {
                return n;
            }
            n = n.wrapping_add(contar_na_casa(this, x, y1, (x as i32 - inicio as i32) as f32));
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
            n = n.wrapping_add(contar_na_casa(this, x, y, distancia_ou_mil(x as i32 - xa as i32, y as i32 - ya as i32)));
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
        n = n.wrapping_add(contar_na_casa(this, x, y, distancia_ou_mil(x as i32 - xa as i32, y as i32 - ya as i32)));
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
unsafe extern "thiscall" fn consegue_jogar(this: usize, a: i8, b: i8, calcular: i8, forca: f32, ajuste: f32, saida: *mut u8) -> u8 {
    let alvo = *((TABELA_GLOBAL as isize + 2 * (a.wrapping_sub(byte(this, 0x102)) as isize * 24 + b.wrapping_sub(byte(this, 0x103)) as isize)) as *const i16);
    let dir = *((this + 0x19a) as *const i16);
    if dir == alvo {
        return 1;
    }
    let tolerancia: i16 = if calcular == 0 {
        0x4b
    } else {
        let f: unsafe extern "stdcall" fn(usize, i8, i8) -> u8 = core::mem::transmute(0x6cb9b0usize);
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
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8 = core::mem::transmute(0x6cbc00usize);
    let c = f(*((this + 0x19e) as *const usize), byte(this, 0x102), byte(this, 0x103), a, b);
    *saida = c;
    let c = c as i8 as i32;
    (byte(this, 0x10b) as i32 >= sortear(c * c)) as u8
}

/// { 0x6f4650 } (match_pl.cpp): na entrada da area (linha do gol, colunas 3..=5)? Senao,
/// 0x6f9f90 (pelo endereco) virado para o meio do gol, sem calculo nem saida (nao sorteia).
unsafe extern "thiscall" fn na_area(this: usize) -> u8 {
    let linha: i8 = if lado(this) == 0 { 0xb } else { 0 };
    if byte(this, 0x103) == linha && (3..=5).contains(&byte(this, 0x102)) {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8 = core::mem::transmute(0x6f9f90usize);
    f(this, 4, linha, 0, 0.0, 0.0, core::ptr::null_mut())
}

// ---------- oitava leva: posicionamento ----------

/// { 0x5b5270 }: marcas taticas (64 bits) de uma posicao: [this + 0x188b + 8 x posicao].
unsafe extern "thiscall" fn marcas_da_posicao(this: usize, posicao: i8) -> u64 {
    *((this as isize + 0x188b + 8 * posicao as isize) as *const u64)
}

/// Marcas da posicao do jogador no bloco do lado dele (0x91e2 + 6371 x lado), por 0x5b5270.
unsafe fn marcas_do_jogador(j: usize) -> u64 {
    let f: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(0x5b5270usize);
    let bloco = (*((j + 0x19e) as *const usize) as isize + 0x91e2 + 6371 * lado(j) as isize) as usize;
    f(bloco, byte(j, 0x19))
}

/// `tabela_a` do motor do jogador e 1 (pelo endereco do jogo); sem motor, nao.
unsafe fn tabela_e_um(j: usize) -> bool {
    let motor = *((j + 0x19e) as *const usize);
    let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    motor != 0 && f(motor, j) == 1
}

/// { 0x716270 } (match_pl.cpp): papel no ataque pelas marcas da posicao: 1 (marca 0x20000), 0 se
/// `tabela_a` = 1, 2 (marca 0x40000000) ou 0.
unsafe extern "thiscall" fn papel_no_ataque(j: usize) -> u8 {
    if marcas_do_jogador(j) & 0x20000 != 0 {
        return 1;
    }
    if tabela_e_um(j) {
        return 0;
    }
    if marcas_do_jogador(j) & 0x4000_0000 != 0 { 2 } else { 0 }
}

/// { 0x716310 } (match_pl.cpp): papel na defesa: 0 se `tabela_a` = 1; 3 (marca 0x80000000),
/// 4 (marca 0x80000) ou 0.
unsafe extern "thiscall" fn papel_na_defesa(j: usize) -> u8 {
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
unsafe extern "thiscall" fn posicionar(this: usize, j: usize, saida_x: *mut i8, saida_y: *mut i8, _a4: u32) {
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
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
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

/// { 0x6c3aa0 } (match_eng.cpp): as duas linhas defensivas (impedimento). Lado 1: maior y (0x103)
/// entre os ativos, partindo de 6, mais 1, em 0x8eaf. Lado 0: menor y, partindo de 5, menos 1, em
/// 0x8eb0. Ativo: posicao >= 0 e `tabela_do_jogador` (pelo endereco) diferente de 1. O original tem
/// os dois lacos de 20 jogadores desenrolados.
unsafe extern "thiscall" fn linhas_defensivas(this: usize) {
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let ativos = |lado: usize| {
        (0..20).map(move |k| (this + 0x4796 + 8920 * lado + k * 0x1be) as usize).filter(move |&p| byte(p, 0x19) >= 0 && f717410(p) != 1)
    };
    let maior = ativos(1).fold(6i8, |m, p| m.max(byte(p, 0x103)));
    *((this + 0x8eaf) as *mut i8) = maior.wrapping_add(1);
    let menor = ativos(0).fold(5i8, |m, p| m.min(byte(p, 0x103)));
    *((this + 0x8eb0) as *mut i8) = menor.wrapping_sub(1);
}

// ---------- decima leva: atualizacao do jogador a cada lance ----------

/// { 0x6f4aa0 } (match_pl.cpp): atualiza, a cada lance, o contador 0x2b (decresce ate 0), o float
/// 0x1a2 (x 0,85, com piso 2 x (25 - atributo 0x5d da pessoa) e teto 100) e a palavra 0x35
/// (disposicao): faixas pelos atributos e, pelo papel tatico (bits de `tabela_a`), sorteios de
/// 1/75 a 1/1000, razoes em ponto flutuante entre atributos e o estado do lado (0xf5bd/0xf5f3).
/// Transcrita bloco a bloco (enderecos do original nos comentarios); os finais compartilhados
/// viram as funcoes locais `ate`.
unsafe extern "thiscall" fn atualizar_jogador(this: usize) {
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
    let f6c38c0: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
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

// ---------- decima primeira leva ----------

/// { 0x6f44a0 } (match_pl.cpp): atributos do momento, sorteados no inicio do lance. Varios sao
/// "o melhor de dois sorteios". A ordem dos 15 sorteios e a do original.
unsafe extern "thiscall" fn sortear_momento(this: usize) {
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
unsafe fn tabela_na(motor: usize, p: usize, posicao: i8) -> u16 {
    let t = *((motor as isize + 0x8ebc + 4 * lado(p) as isize) as *const usize) as *const u16;
    *t.offset(posicao as isize)
}

/// { 0x6ce110 } (match_eng.cpp): mantem o jogador de posicao 1 de cada lado (0xf59e: bloco 0;
/// 0xf5a2: bloco 1). Perdido (posicao < 0) zera; sem ninguem, o primeiro com `tabela` 1 na posicao;
/// se ainda nao houver, promove o primeiro ativo, trocando com quem guardava a posicao 1 em
/// reserva (0x1a, com posicao -2 e 0x20 = -1). Devolve o do lado pedido.
unsafe extern "thiscall" fn manter_posicao_um(this: usize, lado_pedido: i8) -> usize {
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
unsafe extern "thiscall" fn caminho_livre(this: usize, j: usize, ignorar_casa: i8) -> u8 {
    let s = lado(j);
    let outro = (s == 0) as isize;
    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
    if ignorar_casa == 0 && *((this as isize + outro + 90 * (y as isize + 12 * x as isize) + 0x21b6) as *const u8) != 0 {
        return 0;
    }
    let f717410: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
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
unsafe extern "thiscall" fn espaco_a_frente(this: usize, j: usize) -> u8 {
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
unsafe extern "stdcall" fn fora_do_meio(v: i8) -> u8 {
    !(2..=6).contains(&v) as u8
}

/// { 0x6cb700 } (match_eng.cpp): quantos jogadores ativos do `lado` estao alem da linha `y` (lado
/// 1: y menor; lado 0: y maior; outros lados: nenhum), opcionalmente sem o guardado em 0xf582.
unsafe extern "thiscall" fn alem_da_linha(this: usize, y: i8, lado_b: i8, sem_guardado: i8) -> u8 {
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
unsafe extern "stdcall" fn distancia_entre(a: i8, b: i8, c: i8, d: i8) -> f32 {
    distancia_ou_mil(a as i32 - c as i32, b as i32 - d as i32)
}

// ---------- decima segunda leva: acessores do estado da partida e extremos ----------

macro_rules! leitor {
    ($nome:ident, $campo:expr) => {
        /// Leitor de um byte do estado da partida no motor (match_pl.cpp, 0x7003xx).
        unsafe extern "thiscall" fn $nome(this: usize) -> u8 {
            *((this + $campo) as *const u8)
        }
    };
}
macro_rules! gravador {
    ($nome:ident, $campo:expr) => {
        /// Gravador de um byte do estado da partida no motor (match_pl.cpp, 0x7003xx).
        unsafe extern "thiscall" fn $nome(this: usize, v: u8) {
            *((this + $campo) as *mut u8) = v;
        }
    };
}
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
unsafe extern "thiscall" fn gravar_8ed8(this: usize, v: u16) {
    *((this + 0x8ed8) as *mut u16) = v;
}

/// { 0x700410 }: le o ponteiro 0x8f7a.
unsafe extern "thiscall" fn ler_8f7a(this: usize) -> u32 {
    *((this + 0x8f7a) as *const u32)
}

/// { 0x7003b0 } (match_pl.cpp): registra o lado com a posse (0x8eab). Exceto -1, soma 1 ao
/// contador do lado (0x8d46 + lado) e tira 10 do contador do outro (0x8d46 + (lado == 0)), se positivo.
unsafe extern "thiscall" fn registrar_posse(this: usize, v: i8) {
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
unsafe extern "thiscall" fn na_lista(this: usize, alvo: usize) -> u8 {
    let n = *((this + 0x101) as *const i8);
    (0..n.max(0) as usize).any(|k| *((this + 0x1ae + 4 * k) as *const usize) == alvo) as u8
}

/// { 0x6cd660 } (match_eng.cpp): os jogadores extremos de cada lado entre os ativos (posicao >= 0 e,
/// em 0..=10, `tabela_a` diferente de 1): lado 0, menor y em 0xf58e e maior em 0xf596; lado 1,
/// maior y em 0xf592 e menor em 0xf59a. Sem candidato, o valor anterior fica.
unsafe extern "thiscall" fn guardar_extremos(this: usize) {
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

// ---------- decima terceira leva ----------

/// Bloco de 20 jogadores do lado no motor (0x4796 + 8920 x lado), de 0x1be em 0x1be.
fn jogador_do_bloco(motor: usize, lado_b: i8, k: usize) -> usize {
    (motor as isize + 0x4796 + 8920 * lado_b as isize) as usize + k * 0x1be
}

unsafe fn f717410(p: usize) -> u16 {
    let f: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    f(p)
}

unsafe fn f6f09b0(p: usize, a: i8, b: i8) -> f32 {
    let f: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(0x6f09b0usize);
    f(p, a, b)
}

/// { 0x6cbbb0 } (match_eng.cpp): quantos ativos (posicao >= 0) o lado tem; guarda em motor[4 + lado].
unsafe extern "thiscall" fn contar_ativos(this: usize, lado_b: i8) -> u8 {
    let n = (0..20).filter(|&k| byte(jogador_do_bloco(this, lado_b, k), 0x19) >= 0).count() as u8;
    *((this as isize + 4 + lado_b as isize) as *mut u8) = n;
    n
}

/// { 0x6cb650 } (match_eng.cpp): 1 para alguns codigos de lance (37, 42..=50, 96, 97), pelo switch
/// do original (tabelas lidas do exe: indices em 0x6cb680, destinos em 0x6cb678).
unsafe extern "stdcall" fn codigo_especial(codigo: i8) -> u8 {
    let i = codigo as i32 - 0x25;
    if !(0..=0x3c).contains(&i) {
        return 0;
    }
    let caso = *((0x6cb680 + i as usize) as *const u8) as usize;
    (*((0x6cb678 + 4 * caso) as *const u32) == 0x6cb66c) as u8
}

/// { 0x6cd5f0 } (match_eng.cpp): o jogador esta atras do extremo guardado do lado dele?
/// Lado 0: y <= y do menor (0xf58e; sem ele, 12). Outros: y >= y do maior (0xf592; sem ele, -1).
unsafe extern "thiscall" fn atras_do_extremo(this: usize, j: usize) -> u8 {
    let y = byte(j, 0x103);
    if lado(j) == 0 {
        let p = *((this + 0xf58e) as *const usize);
        (y <= if p != 0 { byte(p, 0x103) } else { 0xc }) as u8
    } else {
        let p = *((this + 0xf592) as *const usize);
        (y >= if p != 0 { byte(p, 0x103) } else { -1 }) as u8
    }
}

/// { 0x6d4280 } (match_eng.cpp): o ponteiro em 0x4792 aponta para -2?
unsafe extern "thiscall" fn sem_partida(this: usize) -> u8 {
    (**((this + 0x4792) as *const *const i32) == -2) as u8
}

/// { 0x6bd6b0 } (match_eng.cpp): pressao sobre a area do lado. Soma os adversarios nas linhas
/// perto do gol (grade de contagem, lado oposto), pesando mais as mais proximas (6..1), e marca
/// motor[6 + lado] se a soma (16 bits) passar de sorteio(15) + 30.
unsafe extern "thiscall" fn pressao_na_area(this: usize, lado_b: i8) {
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
unsafe extern "thiscall" fn mais_proximo_do_ponto(this: usize, j: usize, a: i8, b: i8) -> u8 {
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
unsafe extern "thiscall" fn pressao_no_ponto(this: usize, lado_b: i8, a: i8, b: i8, todos: i8) -> u8 {
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
unsafe extern "thiscall" fn menor_distancia(this: usize, lado_b: i8, a: i8, b: i8, todos: i8) -> f32 {
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
enum Parte {
    Pula,
    Fator(usize),
    Inteiro,
}

/// Soma a cada jogador ativo do lado uma parte de `delta` na palavra 0x35 (disposicao), pelo
/// primeiro bit do papel (`tabela`, ordem 1, 2, 4, 8, 0x10, 0x20, 0x40); sem bit, nada.
unsafe fn repartir(this: usize, lado_b: i8, delta: i16, partes: [Parte; 7]) {
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
        }
    }
}

/// { 0x6c06b0 } (match_eng.cpp): reparte a disposicao (papel 1 fica de fora).
unsafe extern "thiscall" fn repartir_a(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Pula, Fator(0x968918), Fator(0x968f20), Fator(0x96d328), Fator(0x969040), Fator(0x968978), Fator(0x968f88)]);
}

/// { 0x6c07a0 } (match_eng.cpp): reparte a disposicao (papel 4 recebe o delta inteiro).
unsafe extern "thiscall" fn repartir_b(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Fator(0x96d330), Fator(0x9689b8), Inteiro, Fator(0x969520), Fator(0x968e00), Fator(0x968f20), Fator(0x967888)]);
}

/// { 0x6c0a70 } (match_eng.cpp): reparte a disposicao (terceira tabela de fatores).
unsafe extern "thiscall" fn repartir_c(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Fator(0x96d330), Fator(0x968968), Fator(0x968968), Fator(0x968968), Fator(0x968968), Fator(0x969510), Fator(0x968918)]);
}

/// { 0x6c2750 } (match_eng.cpp): escolhe um jogador do lado (indice 0..19, -1 sem candidato) por
/// sorteio ponderado pelo papel: maior sorteio(25) x fator (0,1 / 0,1 / 0,2 / 0,5 / 0,75 / 1,05 ou
/// 1,0). O original guarda o fator no lugar do argumento na pilha: jogador sem bit de papel usa o
/// fator anterior, e o primeiro, os bits da palavra inteira empilhada pelo chamador (por isso u32).
unsafe extern "thiscall" fn escolher_por_papel(this: usize, argumento: u32) -> u8 {
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

/// `tabela_a` do motor do jogador (0 sem motor), pelo endereco do jogo: os bits do papel tatico.
unsafe fn papel_word(motor: usize, j: usize) -> u16 {
    if motor == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    f(motor, j)
}

unsafe fn papel_por_endereco(endereco: usize, j: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(endereco);
    f(j)
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
static GRAVACAO: std::sync::Mutex<Vec<(u32, usize, [u32; 4], u8)>> = std::sync::Mutex::new(Vec::new()); // (rotina, this, args, resposta)

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
        GRAVACAO.lock().unwrap_or_else(|e| e.into_inner()).push((0x6f5c30, p, [a, b, c, d], r));
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
                Some(&(0x6f5c30, gp, ga, gr)) if gp == p && ga == [w, x, y, z] => gr,
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

unsafe extern "thiscall" fn comparar_distancia_ao_ponto(this: usize, a: i8, b: i8) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8) -> f32>(21)(this, a, b);
    let n = distancia_ao_ponto(this, a, b);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("distancia_ao_ponto({}, {}) original={} rust={}", a, b, r, n));
    r
}
unsafe extern "thiscall" fn comparar_distancia_ao_gol(this: usize) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> f32>(22)(this);
    let n = distancia_ao_gol(this);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("distancia_ao_gol(lado {}) original={} rust={}", lado(this), r, n));
    r
}
unsafe extern "thiscall" fn comparar_mais_perto(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(23)(this, j);
    let n = mais_perto_do_gol(this, j);
    PLACAR.registrar(r == n, || format!("mais_perto_do_gol(lado {}) original={} rust={}", lado(j), r, n));
    r
}

unsafe extern "thiscall" fn comparar_na_linha(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8>(24)(this, x1, y1, x2, y2);
    let n = jogadores_na_linha(this, x1, y1, x2, y2);
    PLACAR.registrar(r == n, || format!("jogadores_na_linha({}, {}) -> ({}, {}) original={} rust={}", x1, y1, x2, y2, r, n));
    r
}
/// Sorteia e grava `saida`: mesmo estado do gerador para as duas, o jogo segue com o da original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_consegue(this: usize, a: i8, b: i8, calcular: i8, forca: f32, ajuste: f32, saida: *mut u8) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8>(25);
    let mut regioes: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !saida.is_null() {
        regioes.push((saida as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || f(this, a, b, calcular, forca, ajuste, saida),
        || crate::aleatorio::sem_contar(|| consegue_jogar(this, a, b, calcular, forca, ajuste, saida)),
    );
    PLACAR.registrar(igual && r == n, || format!("consegue_jogar({}, {}, {}, {}, {}) original={} rust={} {:x?} {:x?}", a, b, calcular, forca, ajuste, r, n, mo, mr));
    r
}
unsafe extern "thiscall" fn comparar_na_area(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(26)(this);
    let n = na_area(this);
    PLACAR.registrar(r == n, || format!("na_area(lado {}, x {}, y {}) original={} rust={}", lado(this), byte(this, 0x102), byte(this, 0x103), r, n));
    r
}

unsafe extern "thiscall" fn comparar_marcas(this: usize, posicao: i8) -> u64 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u64>(27)(this, posicao);
    let n = marcas_da_posicao(this, posicao);
    PLACAR.registrar(r == n, || format!("marcas_da_posicao({}) original={:#x} rust={:#x}", posicao, r, n));
    r
}
unsafe extern "thiscall" fn comparar_papel_ataque(j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(28)(j);
    let n = papel_no_ataque(j);
    PLACAR.registrar(r == n, || format!("papel_no_ataque(lado {}, pos {}) original={} rust={}", lado(j), byte(j, 0x19), r, n));
    r
}
unsafe extern "thiscall" fn comparar_papel_defesa(j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(29)(j);
    let n = papel_na_defesa(j);
    PLACAR.registrar(r == n, || format!("papel_na_defesa(lado {}, pos {}) original={} rust={}", lado(j), byte(j, 0x19), r, n));
    r
}
unsafe extern "thiscall" fn comparar_posicionar(this: usize, j: usize, sx: *mut i8, sy: *mut i8, a4: u32) {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32)>(30);
    let regioes = [(sx as usize, 1), (sy as usize, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this, j, sx, sy, a4), || posicionar(this, j, sx, sy, a4));
    PLACAR.registrar(igual, || format!("posicionar(lado {}, pos {}, bola {} {}) original={:x?} rust={:x?}", lado(j), byte(j, 0x19), byte(this, 0x8ea7), byte(this, 0x8ea8), mo, mr));
}

unsafe extern "thiscall" fn comparar_linhas(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(31);
    let regioes = [(this + 0x8eaf, 2)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this), || linhas_defensivas(this));
    PLACAR.registrar(igual, || format!("linhas_defensivas original={:x?} rust={:x?}", mo, mr));
}

/// Sorteia e grava (0x2b, 0x1a2, 0x35): mesmo estado do gerador, o jogo segue com o da original.
unsafe extern "thiscall" fn comparar_atualizar(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(32);
    let mut regioes = vec![(this + 0x2b, 1), (this + 0x1a2, 4), (this + 0x35, 2)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) =
        crate::comparacao::lado_a_lado(&regioes, || f(this), || crate::aleatorio::sem_contar(|| atualizar_jogador(this)));
    PLACAR.registrar(igual, || format!("atualizar_jogador(papel {:#x}) original={:x?} rust={:x?}", papel_word(*((this + 0x19e) as *const usize), this), mo, mr));
}

unsafe extern "thiscall" fn comparar_momento(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(33);
    let mut regioes = vec![(this + 0x104, 10)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this), || crate::aleatorio::sem_contar(|| sortear_momento(this)));
    PLACAR.registrar(igual, || format!("sortear_momento original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_posicao_um(this: usize, l: i8) -> usize {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8) -> usize>(34);
    let mut regioes = vec![(this + 0xf59e, 8)];
    for k in 0..40 {
        regioes.push((this + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be + 0x19, 2));
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || f(this, l), || manter_posicao_um(this, l));
    PLACAR.registrar(igual && r == n, || format!("manter_posicao_um({}) original={:#x} rust={:#x} memoria igual={}", l, r, n, igual));
    r
}
unsafe extern "thiscall" fn comparar_caminho(this: usize, j: usize, ig: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8) -> u8>(35)(this, j, ig);
    let n = caminho_livre(this, j, ig);
    PLACAR.registrar(r == n, || format!("caminho_livre(lado {}, {} {}, {}) original={} rust={}", lado(j), byte(j, 0x102), byte(j, 0x103), ig, r, n));
    r
}
unsafe extern "thiscall" fn comparar_espaco(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(36)(this, j);
    let n = espaco_a_frente(this, j);
    PLACAR.registrar(r == n, || format!("espaco_a_frente(lado {}, {} {}) original={} rust={}", lado(j), byte(j, 0x102), byte(j, 0x103), r, n));
    r
}
unsafe extern "stdcall" fn comparar_fora_do_meio(v: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i8) -> u8>(37)(v);
    let n = fora_do_meio(v);
    PLACAR.registrar(r == n, || format!("fora_do_meio({}) original={} rust={}", v, r, n));
    r
}
unsafe extern "thiscall" fn comparar_alem(this: usize, y: i8, l: i8, sg: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8>(38)(this, y, l, sg);
    let n = alem_da_linha(this, y, l, sg);
    PLACAR.registrar(r == n, || format!("alem_da_linha({}, {}, {}) original={} rust={}", y, l, sg, r, n));
    r
}
unsafe extern "stdcall" fn comparar_distancia_entre(a: i8, b: i8, c: i8, d: i8) -> f32 {
    let r = orig::<unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32>(39)(a, b, c, d);
    let n = distancia_entre(a, b, c, d);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("distancia_entre({} {} {} {}) original={} rust={}", a, b, c, d, r, n));
    r
}

macro_rules! comparar_leitor {
    ($nome:ident, $rust:ident, $i:expr) => {
        unsafe extern "thiscall" fn $nome(this: usize) -> u8 {
            let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>($i)(this);
            let n = $rust(this);
            PLACAR.registrar(r == n, || format!("{} original={} rust={}", stringify!($rust), r, n));
            r
        }
    };
}
macro_rules! comparar_gravador {
    ($nome:ident, $rust:ident, $i:expr, $campo:expr, $t:ty) => {
        unsafe extern "thiscall" fn $nome(this: usize, v: $t) {
            let f = orig::<unsafe extern "thiscall" fn(usize, $t)>($i);
            let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + $campo, core::mem::size_of::<$t>())], || f(this, v), || $rust(this, v));
            PLACAR.registrar(igual, || format!("{} original={:x?} rust={:x?}", stringify!($rust), mo, mr));
        }
    };
}
comparar_leitor!(comparar_ler_8ea7, ler_8ea7, 40);
comparar_leitor!(comparar_ler_8ea8, ler_8ea8, 41);
comparar_leitor!(comparar_ler_8ea9, ler_8ea9, 42);
comparar_leitor!(comparar_ler_8eaa, ler_8eaa, 43);
comparar_leitor!(comparar_ler_8eb2, ler_8eb2, 44);
comparar_gravador!(comparar_gravar_8ea9, gravar_8ea9, 45, 0x8ea9, u8);
comparar_gravador!(comparar_gravar_8ea7, gravar_8ea7, 46, 0x8ea7, u8);
comparar_gravador!(comparar_gravar_8ea8, gravar_8ea8, 47, 0x8ea8, u8);
comparar_gravador!(comparar_gravar_8eaa, gravar_8eaa, 48, 0x8eaa, u8);
comparar_gravador!(comparar_gravar_8ed8, gravar_8ed8, 49, 0x8ed8, u16);
unsafe extern "thiscall" fn comparar_ler_8f7a(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(50)(this);
    let n = ler_8f7a(this);
    PLACAR.registrar(r == n, || format!("ler_8f7a original={:#x} rust={:#x}", r, n));
    r
}
unsafe extern "thiscall" fn comparar_posse(this: usize, v: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8)>(51);
    let regioes = [(this + 0x8eab, 1), ((this as isize + 0x8d46 + v as isize) as usize, 1), (this + 0x8d46, 2)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this, v), || registrar_posse(this, v));
    PLACAR.registrar(igual, || format!("registrar_posse({}) original={:x?} rust={:x?}", v, mo, mr));
}
unsafe extern "thiscall" fn comparar_na_lista(this: usize, alvo: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(52)(this, alvo);
    let n = na_lista(this, alvo);
    PLACAR.registrar(r == n, || format!("na_lista(n {}) original={} rust={}", byte(this, 0x101), r, n));
    r
}
unsafe extern "thiscall" fn comparar_extremos(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(53);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0xf58e, 16)], || f(this), || guardar_extremos(this));
    PLACAR.registrar(igual, || format!("guardar_extremos original={:x?} rust={:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_contar_ativos(this: usize, l: i8) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8) -> u8>(54);
    let reg = [((this as isize + 4 + l as isize) as usize, 1)];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, l), || contar_ativos(this, l));
    PLACAR.registrar(igual && r == n, || format!("contar_ativos({}) original={} rust={}", l, r, n));
    r
}
unsafe extern "stdcall" fn comparar_codigo_especial(c: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i8) -> u8>(55)(c);
    let n = codigo_especial(c);
    PLACAR.registrar(r == n, || format!("codigo_especial({}) original={} rust={}", c, r, n));
    r
}
unsafe extern "thiscall" fn comparar_atras(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(56)(this, j);
    let n = atras_do_extremo(this, j);
    PLACAR.registrar(r == n, || format!("atras_do_extremo(lado {}, y {}) original={} rust={}", lado(j), byte(j, 0x103), r, n));
    r
}
unsafe extern "thiscall" fn comparar_sem_partida(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(57)(this);
    let n = sem_partida(this);
    PLACAR.registrar(r == n, || format!("sem_partida original={} rust={}", r, n));
    r
}
unsafe extern "thiscall" fn comparar_pressao_area(this: usize, l: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8)>(58);
    let mut reg = vec![((this as isize + 6 + l as isize) as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this, l), || crate::aleatorio::sem_contar(|| pressao_na_area(this, l)));
    PLACAR.registrar(igual, || format!("pressao_na_area({}) original={:x?} rust={:x?}", l, mo, mr));
}
unsafe extern "thiscall" fn comparar_mais_proximo(this: usize, j: usize, a: i8, b: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u8>(59)(this, j, a, b);
    let n = mais_proximo_do_ponto(this, j, a, b);
    PLACAR.registrar(r == n, || format!("mais_proximo_do_ponto({}, {}) original={} rust={}", a, b, r, n));
    r
}
unsafe extern "thiscall" fn comparar_pressao_ponto(this: usize, l: i8, a: i8, b: i8, t: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8>(60)(this, l, a, b, t);
    let n = pressao_no_ponto(this, l, a, b, t);
    PLACAR.registrar(r == n, || format!("pressao_no_ponto({}, {}, {}, {}) original={} rust={}", l, a, b, t, r, n));
    r
}
unsafe extern "thiscall" fn comparar_menor_distancia(this: usize, l: i8, a: i8, b: i8, t: i8) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> f32>(61)(this, l, a, b, t);
    let n = menor_distancia(this, l, a, b, t);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("menor_distancia({}, {}, {}, {}) original={} rust={}", l, a, b, t, r, n));
    r
}

macro_rules! comparar_repartir {
    ($nome:ident, $rust:ident, $i:expr) => {
        unsafe extern "thiscall" fn $nome(this: usize, l: i8, delta: i16) {
            let f = orig::<unsafe extern "thiscall" fn(usize, i8, i16)>($i);
            let reg: Vec<(usize, usize)> = (0..20).map(|k| (jogador_do_bloco(this, l, k) + 0x35, 2)).collect();
            let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this, l, delta), || $rust(this, l, delta));
            PLACAR.registrar(igual, || format!("{}({}, {}) original={:x?} rust={:x?}", stringify!($rust), l, delta, mo, mr));
        }
    };
}
comparar_repartir!(comparar_repartir_a, repartir_a, 62);
comparar_repartir!(comparar_repartir_b, repartir_b, 63);
comparar_repartir!(comparar_repartir_c, repartir_c, 64);
unsafe extern "thiscall" fn comparar_escolher(this: usize, arg: u32) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, u32) -> u8>(65);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, arg), || crate::aleatorio::sem_contar(|| escolher_por_papel(this, arg)));
    PLACAR.registrar(igual && r == n, || format!("escolher_por_papel({:#x}) original={} rust={}", arg, r as i8, n as i8));
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
        distancia_ao_ponto as *const () as usize,
        distancia_ao_gol as *const () as usize,
        mais_perto_do_gol as *const () as usize,
        jogadores_na_linha as *const () as usize,
        consegue_jogar as *const () as usize,
        na_area as *const () as usize,
        marcas_da_posicao as *const () as usize,
        papel_no_ataque as *const () as usize,
        papel_na_defesa as *const () as usize,
        posicionar as *const () as usize,
        linhas_defensivas as *const () as usize,
        atualizar_jogador as *const () as usize,
        sortear_momento as *const () as usize,
        manter_posicao_um as *const () as usize,
        caminho_livre as *const () as usize,
        espaco_a_frente as *const () as usize,
        fora_do_meio as *const () as usize,
        alem_da_linha as *const () as usize,
        distancia_entre as *const () as usize,
        ler_8ea7 as *const () as usize,
        ler_8ea8 as *const () as usize,
        ler_8ea9 as *const () as usize,
        ler_8eaa as *const () as usize,
        ler_8eb2 as *const () as usize,
        gravar_8ea9 as *const () as usize,
        gravar_8ea7 as *const () as usize,
        gravar_8ea8 as *const () as usize,
        gravar_8eaa as *const () as usize,
        gravar_8ed8 as *const () as usize,
        ler_8f7a as *const () as usize,
        registrar_posse as *const () as usize,
        na_lista as *const () as usize,
        guardar_extremos as *const () as usize,
        contar_ativos as *const () as usize,
        codigo_especial as *const () as usize,
        atras_do_extremo as *const () as usize,
        sem_partida as *const () as usize,
        pressao_na_area as *const () as usize,
        mais_proximo_do_ponto as *const () as usize,
        pressao_no_ponto as *const () as usize,
        menor_distancia as *const () as usize,
        repartir_a as *const () as usize,
        repartir_b as *const () as usize,
        repartir_c as *const () as usize,
        escolher_por_papel as *const () as usize,
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
        comparar_distancia_ao_ponto as *const () as usize,
        comparar_distancia_ao_gol as *const () as usize,
        comparar_mais_perto as *const () as usize,
        comparar_na_linha as *const () as usize,
        comparar_consegue as *const () as usize,
        comparar_na_area as *const () as usize,
        comparar_marcas as *const () as usize,
        comparar_papel_ataque as *const () as usize,
        comparar_papel_defesa as *const () as usize,
        comparar_posicionar as *const () as usize,
        comparar_linhas as *const () as usize,
        comparar_atualizar as *const () as usize,
        comparar_momento as *const () as usize,
        comparar_posicao_um as *const () as usize,
        comparar_caminho as *const () as usize,
        comparar_espaco as *const () as usize,
        comparar_fora_do_meio as *const () as usize,
        comparar_alem as *const () as usize,
        comparar_distancia_entre as *const () as usize,
        comparar_ler_8ea7 as *const () as usize,
        comparar_ler_8ea8 as *const () as usize,
        comparar_ler_8ea9 as *const () as usize,
        comparar_ler_8eaa as *const () as usize,
        comparar_ler_8eb2 as *const () as usize,
        comparar_gravar_8ea9 as *const () as usize,
        comparar_gravar_8ea7 as *const () as usize,
        comparar_gravar_8ea8 as *const () as usize,
        comparar_gravar_8eaa as *const () as usize,
        comparar_gravar_8ed8 as *const () as usize,
        comparar_ler_8f7a as *const () as usize,
        comparar_posse as *const () as usize,
        comparar_na_lista as *const () as usize,
        comparar_extremos as *const () as usize,
        comparar_contar_ativos as *const () as usize,
        comparar_codigo_especial as *const () as usize,
        comparar_atras as *const () as usize,
        comparar_sem_partida as *const () as usize,
        comparar_pressao_area as *const () as usize,
        comparar_mais_proximo as *const () as usize,
        comparar_pressao_ponto as *const () as usize,
        comparar_menor_distancia as *const () as usize,
        comparar_repartir_a as *const () as usize,
        comparar_repartir_b as *const () as usize,
        comparar_repartir_c as *const () as usize,
        comparar_escolher as *const () as usize,
    ];
    let nomes = [
        "motor_tabela_a", "motor_tabela_b", "motor_diferenca_lado", "motor_faixa", "motor_por_diferenca", "motor_limite",
        "motor_condicao_jogador", "motor_tabela_do_jogador", "motor_diferenca_jogador", "motor_byte_do_lado",
        "motor_limite_jogador", "motor_faixa_jogador", "motor_byte_da_posicao", "motor_campo_pela_posicao",
        "motor_pode_escolher", "motor_apontar", "motor_desgastar", "motor_virado_para", "motor_procurar_no_lado",
        "motor_alcanca", "motor_classificar_lance", "motor_distancia_ao_ponto", "motor_distancia_ao_gol",
        "motor_mais_perto_do_gol", "motor_jogadores_na_linha", "motor_consegue_jogar", "motor_na_area",
        "motor_marcas_da_posicao", "motor_papel_no_ataque", "motor_papel_na_defesa", "motor_posicionar",
        "motor_linhas_defensivas", "motor_atualizar_jogador",
        "motor_sortear_momento", "motor_manter_posicao_um", "motor_caminho_livre", "motor_espaco_a_frente",
        "motor_fora_do_meio", "motor_alem_da_linha", "motor_distancia_entre",
        "motor_ler_8ea7", "motor_ler_8ea8", "motor_ler_8ea9", "motor_ler_8eaa", "motor_ler_8eb2", "motor_gravar_8ea9",
        "motor_gravar_8ea7", "motor_gravar_8ea8", "motor_gravar_8eaa", "motor_gravar_8ed8", "motor_ler_8f7a",
        "motor_registrar_posse", "motor_na_lista", "motor_guardar_extremos",
        "motor_contar_ativos", "motor_codigo_especial", "motor_atras_do_extremo", "motor_sem_partida",
        "motor_pressao_na_area", "motor_mais_proximo_do_ponto", "motor_pressao_no_ponto", "motor_menor_distancia",
        "motor_repartir_a", "motor_repartir_b", "motor_repartir_c", "motor_escolher_por_papel",
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
    let mut motor = vec![0u8; 0x10000]; // campos ate 0xf5f5 (estado do lado)
    let mut tabelas = vec![vec![0u16; 11]; 4];
    let mut jogador = vec![0u8; 0x200];
    let outro = vec![0u8; 0x1a4];
    let mut pessoa = vec![0u8; 0x80]; // campos lidos ate 0x5d
    let mut formacoes = vec![vec![0u8; 4800]; 2];
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
            // sexta leva: distancias e os dois blocos de 20 jogadores no motor
            let e = s.proximo();
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let z = e.rotate_left(k as u32 * 3);
                *((p + 0x19) as *mut i8) = (z % 13) as i8 - 2;
                *((p + 0x27) as *mut i8) = (k / 20) as i8;
                *((p + 0x102) as *mut i8) = ((z >> 4) % 9) as i8;
                *((p + 0x103) as *mut i8) = ((z >> 8) % 12) as i8;
                *((p + 0x19e) as *mut usize) = m;
            }
            for k in 0..8usize {
                let casa = m + 0x21b6 + ((e >> (k * 7)) % 9700) as usize;
                *(casa as *mut u8) = ((e >> (k * 5)) % 3 == 0) as u8;
            }
            let jj = m + 0x4796 + 8920 * ((e >> 50) % 2) as usize + ((e >> 52) % 20) as usize * 0x1be;
            *((jj + 0x19) as *mut i8) = 3;
            comparar_distancia_ao_ponto(jj, ((e >> 40) % 9) as i8, ((e >> 44) % 12) as i8);
            comparar_distancia_ao_gol(jj);
            comparar_mais_perto(m, jj);
            // setima leva: grade com listas validas, linhas em todas as direcoes, passe com sorteio
            let w2 = s.proximo();
            for cel in 0..108usize {
                let g = m + 0x215e + 90 * cel;
                for l in 0..2usize {
                    *((g + 0x58 + l) as *mut i8) = ((w2.rotate_left((cel * 2 + l) as u32) >> 3) % 4) as i8;
                    for k in 0..11usize {
                        *((g + 44 * l + 4 * k) as *mut usize) = jogadores[(cel + k + l) % 22].as_ptr() as usize;
                    }
                }
            }
            let pt = |v: u64| (((v % 11) as i8) - 1, (((v >> 4) % 14) as i8) - 1);
            let (x1, y1) = pt(w2 >> 8);
            let (x2, y2) = match (w2 >> 16) % 4 {
                0 => (x1, ((w2 >> 20) % 14) as i8 - 1),
                1 => (((w2 >> 20) % 11) as i8 - 1, y1),
                _ => pt(w2 >> 24),
            };
            comparar_na_linha(m, x1, y1, x2, y2);
            let v2 = s.proximo();
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = (v2 % 2) as i8;
            *((j + 0x102) as *mut i8) = ((v2 >> 2) % 9) as i8;
            *((j + 0x103) as *mut i8) = ((v2 >> 6) % 12) as i8;
            *((j + 0x10b) as *mut i8) = ((v2 >> 10) % 21) as i8;
            *((j + 0x19a) as *mut i16) = ((v2 >> 15) % 360) as i16;
            let (a, b) = (((v2 >> 24) % 9) as i8, ((v2 >> 28) % 12) as i8);
            let mut saida = 0u8;
            let ptr = if (v2 >> 32) % 3 == 0 { core::ptr::null_mut() } else { &mut saida as *mut u8 };
            comparar_consegue(j, a, b, ((v2 >> 34) % 3 != 0) as i8, ((v2 >> 36) % 40) as f32 / 3.0, ((v2 >> 42) % 30) as f32 / 4.0, ptr);
            comparar_na_area(j);
            // oitava leva: posicionamento (formacoes com margem: o indice fica negativo com posicao < 0)
            let o2 = s.proximo();
            for (l, f) in formacoes.iter_mut().enumerate() {
                for (k, b) in f.iter_mut().enumerate() {
                    *b = (o2.rotate_left((k % 64) as u32) as u8) ^ (k as u8).wrapping_mul(37) ^ (l as u8);
                }
                *((m + 0xf56e + 4 * l) as *mut usize) = f.as_ptr() as usize + 1200;
            }
            for lado_m in 0..2usize {
                *((m + 0x9766 + 6371 * lado_m) as *mut u32) = (o2 >> (lado_m * 16)) as u32 & 0x1d000;
                for pos in 0..14usize {
                    let marcas = m + 0x91e2 + 6371 * lado_m + 0x188b + 8 * pos;
                    *(marcas as *mut u64) = o2.rotate_left((pos * 5 + lado_m * 3) as u32) & 0xc00a_0000_c00a_0000;
                }
            }
            *((m + 6) as *mut i8) = ((o2 >> 3) % 2) as i8;
            *((m + 7) as *mut i8) = ((o2 >> 4) % 2) as i8;
            *((m + 0x8ea7) as *mut i8) = ((o2 >> 5) % 12) as i8 - 2;
            *((m + 0x8ea8) as *mut i8) = ((o2 >> 9) % 15) as i8 - 2;
            *((m + 0x8ea9) as *mut i8) = [-1i8, 0, 3][((o2 >> 13) % 3) as usize];
            *((m + 0x8eae) as *mut i8) = ((o2 >> 15) % 3) as i8;
            *((m + 0x8eb1) as *mut i8) = ((o2 >> 17) % 3) as i8 - 1;
            *((m + 0xf57a) as *mut usize) = if (o2 >> 19) % 3 == 0 { 0 } else { jogadores[((o2 >> 21) % 22) as usize].as_ptr() as usize };
            let p2 = s.proximo();
            *((j + 0x27) as *mut i8) = (p2 % 2) as i8;
            *((j + 0x19) as *mut i8) = ((p2 >> 1) % 13) as i8 - 1;
            *((j + 0x102) as *mut i8) = ((p2 >> 5) % 12) as i8 - 2;
            *((j + 0x103) as *mut i8) = ((p2 >> 9) % 15) as i8 - 2;
            *((j + 0x19e) as *mut usize) = m;
            comparar_marcas(m + 0x91e2, ((p2 >> 14) % 14) as i8);
            comparar_papel_ataque(j);
            comparar_papel_defesa(j);
            let (mut sx, mut sy) = (0x55i8, 0x55i8);
            comparar_posicionar(m, j, &mut sx, &mut sy, 0);
            // nona leva: linhas defensivas sobre os dois blocos de 20 (ja preenchidos acima), y variado
            let y2 = s.proximo();
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                *((p + 0x103) as *mut i8) = (y2.rotate_left(k as u32 * 2) % 16) as i8 - 2;
                *((p + 0x19) as *mut i8) = (y2.rotate_left(k as u32 * 3 + 1) % 13) as i8 - 2;
            }
            comparar_linhas(m);
            // decima leva: atualizacao do jogador (todos os papeis, atributos e faixas variados)
            let z = s.proximo();
            let z2 = s.proximo();
            // cada campo: zero com chance 1/2 (combinacoes "tudo parado"), senao 1..=24; 1 e 2 ate 100
            for (k, o) in [1usize, 2, 3, 4, 5, 6, 7, 8, 9, 0xb, 0xc, 0xf, 0x10, 0x11].iter().enumerate() {
                let v = z.rotate_left(k as u32 * 5);
                let teto = if *o <= 2 { 100 } else { 24 };
                *((j + o) as *mut u8) = if v & 1 == 0 { 0 } else { (1 + (v >> 1) % teto) as u8 };
            }
            *((j + 0x1f) as *mut i8) = [-1i8, 10, 30, 31, 50][((z2 >> 2) % 5) as usize];
            *((j + 0x2b) as *mut i8) = ((z2 >> 5) % 5) as i8 - 2;
            *((j + 0x1a2) as *mut f32) = ((z2 >> 8) % 140) as f32 + 0.25;
            // palavra 0x35: metade das vezes exatamente num limiar do codigo (ou vizinho)
            const LIMIARES: [i16; 14] = [0x1cb5, 0x1cb6, 0x1b57, 0x1676, 0x1675, 0x1a5d, 0x186a, 0x1c52, 0x1c53, 0x1d4c, 0x1d4d, 0x1e46, 0x222e, 0x109a];
            *((j + 0x35) as *mut i16) = if (z2 >> 15) & 1 == 0 {
                LIMIARES[((z2 >> 16) % 14) as usize] + ((z2 >> 20) % 3) as i16 - 1
            } else {
                3500 + ((z2 >> 16) % 6500) as i16
            };
            *((pp + 0x5d) as *mut i8) = ((z2 >> 30) % 25) as i8;
            *((j + 0x69) as *mut usize) = pp;
            *((j + 0x27) as *mut i8) = ((z2 >> 35) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((z2 >> 36) % 11) as i8;
            *((j + 0x19e) as *mut usize) = if (z2 >> 40) % 9 == 0 { 0 } else { m };
            *((m + 0xf5bd) as *mut i8) = ((z2 >> 42) % 5) as i8 - 1;
            *((m + 0xf5f3) as *mut i8) = ((z2 >> 45) % 5) as i8 - 1;
            *((m + 0xf5bf) as *mut u8) = ((z2 >> 48) % 4) as u8;
            *((m + 0xf5bf + 54) as *mut u8) = ((z2 >> 50) % 4) as u8;
            *((m + 0x8ed0) as *mut i16) = ((z2 >> 52) % 900) as i16 - 100;
            comparar_atualizar(j);
            *((j + 0x19e) as *mut usize) = m;
            // decima primeira leva
            let q1 = s.proximo();
            let q2 = s.proximo();
            *((j + 0x4d) as *mut f32) = ((q1 >> 2) % 12000) as f32 / 1.5;
            for o in 0x10eusize..=0x117 {
                *((j + o) as *mut i8) = (q1.rotate_left(o as u32) % 30) as i8 - 3;
            }
            comparar_momento(j);
            // tabelas com varios 1 (posicao 1 e reserva), blocos com posicoes, reservas e 0x20
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    if (q2 >> (k % 50)) & 3 == 0 {
                        *v = 1;
                    }
                }
            }
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = q2.rotate_left(k as u32 * 3);
                *((p + 0x19) as *mut i8) = [(r % 13) as i8 - 2, -2, -1, 0][((r >> 8) % 4) as usize];
                *((p + 0x1a) as *mut i8) = (r >> 12) as i8 % 12;
                *((p + 0x20) as *mut i8) = if (r >> 20) & 1 == 0 { -1 } else { 0 };
                *((p + 0x102) as *mut i8) = ((r >> 24) % 10) as i8 - 1;
                *((p + 0x103) as *mut i8) = ((r >> 28) % 13) as i8 - 1;
            }
            let algum = |sh: u32| m + 0x4796 + 8920 * ((q2 >> sh) % 2) as usize + ((q2 >> (sh + 1)) % 20) as usize * 0x1be;
            *((m + 0xf59e) as *mut usize) = if (q2 >> 40) % 3 == 0 { 0 } else { algum(41) };
            *((m + 0xf5a2) as *mut usize) = if (q2 >> 47) % 3 == 0 { 0 } else { algum(48) };
            comparar_posicao_um(m, ((q2 >> 55) % 3) as i8);
            let jj = algum(57);
            comparar_caminho(m, jj, ((q2 >> 63) & 1) as i8);
            comparar_espaco(m, jj);
            comparar_fora_do_meio(((q1 >> 40) % 12) as i8 - 2);
            comparar_alem(m, ((q1 >> 44) % 14) as i8 - 1, ((q1 >> 48) % 3) as i8, ((q1 >> 50) & 1) as i8);
            // decima segunda leva
            let g2 = s.proximo();
            comparar_ler_8ea7(m);
            comparar_ler_8ea8(m);
            comparar_ler_8ea9(m);
            comparar_ler_8eaa(m);
            comparar_ler_8eb2(m);
            comparar_gravar_8ea9(m, g2 as u8);
            comparar_gravar_8ea7(m, (g2 >> 8) as u8);
            comparar_gravar_8ea8(m, (g2 >> 16) as u8);
            comparar_gravar_8eaa(m, (g2 >> 24) as u8);
            comparar_gravar_8ed8(m, (g2 >> 32) as u16);
            *((m + 0x8f7a) as *mut u32) = (g2 >> 3) as u32;
            *((m + 0x8f7e) as *mut u8) = (g2 >> 11) as u8;
            comparar_ler_8f7a(m);
            *((m + 0x8d46) as *mut i8) = ((g2 >> 40) % 30) as i8 - 5;
            *((m + 0x8d47) as *mut i8) = ((g2 >> 45) % 30) as i8 - 5;
            comparar_posse(m, [-1i8, 0, 1, 2][((g2 >> 50) % 4) as usize]);
            *((j + 0x101) as *mut i8) = ((g2 >> 52) % 8) as i8 - 1;
            for k in 0..8usize {
                *((j + 0x1ae + 4 * k) as *mut usize) = (g2 >> (k * 3)) as usize % 5;
            }
            comparar_na_lista(j, ((g2 >> 56) % 6) as usize);
            for k in 0..4usize {
                *((m + 0xf58e + 4 * k) as *mut usize) = jogadores[k].as_ptr() as usize;
            }
            comparar_extremos(m);
            // decima terceira leva (blocos ja preenchidos; alguns 0x2b zerados; grade com contagens)
            let h3 = s.proximo();
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                if (h3 >> (k % 60)) & 1 == 0 {
                    *((p + 0x2b) as *mut i8) = 0;
                }
            }
            for cel in 0..108usize {
                for l in 0..2usize {
                    // quase vazia (1 em 8), para a soma da pressao cair perto do limiar (30..44)
                    let v = h3.rotate_left((cel * 2 + l) as u32);
                    *((m + 0x21b6 + 90 * cel + l) as *mut i8) = if v % 8 == 0 { 1 + ((v >> 3) % 2) as i8 } else if v % 31 == 1 { -1 } else { 0 };
                }
            }
            comparar_contar_ativos(m, ((h3 >> 3) % 2) as i8);
            comparar_codigo_especial(((h3 >> 5) % 110) as i8 - 5);
            let jj3 = m + 0x4796 + 8920 * ((h3 >> 12) % 2) as usize + ((h3 >> 13) % 20) as usize * 0x1be;
            comparar_atras(m, jj3);
            let mut alvo = -2i32;
            *((m + 0x4792) as *mut usize) = if (h3 >> 18) & 1 == 0 { &mut alvo as *mut i32 as usize } else { m + 0x10 };
            comparar_sem_partida(m);
            comparar_pressao_area(m, ((h3 >> 19) % 2) as i8);
            let (a3, b3) = (((h3 >> 20) % 10) as i8 - 1, ((h3 >> 24) % 13) as i8 - 1);
            comparar_mais_proximo(m, jj3, a3, b3);
            comparar_mais_proximo(m, jj3, byte(jj3, 0x102), byte(jj3, 0x103));
            comparar_pressao_ponto(m, ((h3 >> 28) % 2) as i8, a3, b3, ((h3 >> 29) & 1) as i8);
            comparar_menor_distancia(m, ((h3 >> 30) % 2) as i8, a3, b3, ((h3 >> 31) & 1) as i8);
            // decima quarta leva (tabelas com bits de papel variados, inclusive sem nenhum e 0x700)
            let h4 = s.proximo();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    let r = h4.rotate_left(k as u32 * 7) as u16;
                    *v = [0u16, 1, 2, 4, 8, 0x10, 0x20, 0x40, 0x140, 0x240, 0x440, 0x46, r][(r % 13) as usize];
                }
            }
            let delta = ((h4 >> 20) % 4000) as i16 - 2000;
            comparar_repartir_a(m, ((h4 >> 32) % 2) as i8, delta);
            comparar_repartir_b(m, ((h4 >> 33) % 2) as i8, delta);
            comparar_repartir_c(m, ((h4 >> 34) % 2) as i8, delta);
            let alto = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h4 >> 36) % 5) as usize];
            comparar_escolher(m, (alto & !0xff) | ((h4 >> 40) % 2) as u32);
            comparar_distancia_entre(((q1 >> 51) % 12) as i8 - 1, ((q1 >> 55) % 14) as i8 - 1, ((q1 >> 59) % 11) as i8 - 1, ((q1 >> 33) % 14) as i8 - 1);
        }
    });
    SINTETICO.store(false, std::sync::atomic::Ordering::Relaxed);
}
