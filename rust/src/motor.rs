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
    // decima quinta leva
    Prologo { endereco: 0x6f4a30, bytes: &[0x66, 0x8b, 0x81, 0x9a, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6ce0f0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x33, 0xc0] },
    Prologo { endereco: 0x70df20, bytes: &[0x8a, 0x81, 0x02, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x7103c0, bytes: &[0x8b, 0x81, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6bba50, bytes: &[0x8a, 0x81, 0x5a, 0x47, 0x00, 0x00] },
    Prologo { endereco: 0x6c2fe0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x6c3920, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8a, 0x42, 0x1a] },
    Prologo { endereco: 0x6d4250, bytes: &[0x66, 0x8b, 0x81, 0xd4, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x6f4a40, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] },
    Prologo { endereco: 0x6f4a70, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] },
    Prologo { endereco: 0x70a890, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x70a8c0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x70df70, bytes: &[0x8a, 0x41, 0x27, 0x3c, 0x01] },
    Prologo { endereco: 0x6bba10, bytes: &[0x83, 0xc8, 0xff, 0x33, 0xd2] },
    Prologo { endereco: 0x6cf710, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    // decima sexta leva
    Prologo { endereco: 0x6c0890, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c0980, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c0b70, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6bb9a0, bytes: &[0x8b, 0x81, 0x61, 0x47, 0x00, 0x00] },
    Prologo { endereco: 0x714ca0, bytes: &[0x8a, 0x91, 0x01, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6cdd00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56, 0x57] },
    Prologo { endereco: 0x70d630, bytes: &[0x8a, 0x41, 0x27, 0x56, 0xf6, 0xd8] },
    // decima setima leva
    Prologo { endereco: 0x6bba80, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb] },
    Prologo { endereco: 0x6c41b0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x713d90, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] },
    // decima oitava leva
    Prologo { endereco: 0x6d0f80, bytes: &[0x83, 0xec, 0x20, 0x8b, 0x81, 0x92, 0x47, 0x00, 0x00] },
    Prologo { endereco: 0x6c2880, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] },
    Prologo { endereco: 0x6c2a40, bytes: &[0x8a, 0x81, 0xae, 0x8e, 0x00, 0x00] },
    // decima nona leva
    Prologo { endereco: 0x6ce9c0, bytes: &[0x83, 0xec, 0x10, 0x8a, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x6c2050, bytes: &[0x83, 0xec, 0x10, 0x0f, 0xbe, 0x54, 0x24, 0x14] },
    // vigesima leva
    Prologo { endereco: 0x716450, bytes: &[0xd9, 0x44, 0x24, 0x04, 0xd8, 0x89, 0xa2, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x7165b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x6bb8d0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x6bb950, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6c2310, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] },
    // vigesima primeira leva (rotinas sem arquivo no mapa, chamadas pelo motor)
    Prologo { endereco: 0x717400, bytes: &[0x66, 0x8b, 0x81, 0xbc, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x7160d0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x8a, 0x51, 0x2b] },
    Prologo { endereco: 0x7160e0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x41, 0x2b] },
    Prologo { endereco: 0x717430, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x717450, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x715f30, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x7163c0, bytes: &[0x8a, 0x41, 0x19, 0x50, 0x0f, 0xbe, 0x41, 0x27] },
    Prologo { endereco: 0x7163f0, bytes: &[0x0f, 0xbe, 0x41, 0x27, 0x56] },
    Prologo { endereco: 0x70def0, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] },
    Prologo { endereco: 0x717c70, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x8e, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6f0970, bytes: &[0x8b, 0x91, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6e1cd0, bytes: &[0x8b, 0x41, 0x23, 0x8b, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x6d5cd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x25, 0xff, 0x7f, 0x00, 0x00] },
    // vigesima segunda leva
    Prologo { endereco: 0x715f50, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x8e, 0x9e, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x7172d0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8a, 0x5c, 0x24, 0x10] },
    // vigesima terceira leva
    Prologo { endereco: 0x6f0630, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb] },
    // vigesima quarta leva
    Prologo { endereco: 0x6e9470, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x6bab90, bytes: &[0x8a, 0x81, 0x5a, 0x47, 0x00, 0x00] },
    Prologo { endereco: 0x6e9540, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    // vigesima quinta leva
    Prologo { endereco: 0x6e9630, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x6ea4e0, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    // vigesima sexta leva
    Prologo { endereco: 0x6cfca0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] },
    Prologo { endereco: 0x6cfcd0, bytes: &[0x8b, 0x44, 0x24, 0x20, 0x8b, 0x54, 0x24, 0x1c] },
    Prologo { endereco: 0x6cfd10, bytes: &[0x8b, 0x44, 0x24, 0x18, 0x8b, 0x54, 0x24, 0x14] },
    // vigesima setima leva
    Prologo { endereco: 0x6ea760, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6cfd70, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] },
    // vigesima oitava leva
    Prologo { endereco: 0x6eab60, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x6cfd40, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] },
    // vigesima nona leva
    Prologo { endereco: 0x6c23f0, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] },
    // trigesima leva
    Prologo { endereco: 0x6cf750, bytes: &[0x66, 0x8b, 0x81, 0xd4, 0x8e, 0x00, 0x00] },
    // trigesima primeira leva
    Prologo { endereco: 0x6ce460, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56, 0x57] },
    // trigesima segunda leva
    Prologo { endereco: 0x6c30e0, bytes: &[0x51, 0x0f, 0xbe, 0x54, 0x24, 0x08] },
    // trigesima terceira leva
    Prologo { endereco: 0x700420, bytes: &[0x8b, 0x44, 0x24, 0x18, 0x83, 0xec, 0x08] },
    // trigesima quarta leva
    Prologo { endereco: 0x6f46b0, bytes: &[0x8a, 0x81, 0x02, 0x01, 0x00, 0x00] },
    // trigesima quinta leva
    Prologo { endereco: 0x6bce80, bytes: &[0x83, 0xec, 0x2c, 0x55, 0x8b, 0x6c, 0x24, 0x34] },
    // trigesima sexta leva
    Prologo { endereco: 0x70d690, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    // trigesima setima leva
    Prologo { endereco: 0x6ccb60, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x14] },
    // trigesima oitava leva (match_day.cpp)
    Prologo { endereco: 0x6b59a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b7cf0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    // trigesima nona leva (comentarios: invólucros de 0x6d6f70)
    Prologo { endereco: 0x7002f0, bytes: &[0x8b, 0x44, 0x24, 0x14, 0x8b, 0x54, 0x24, 0x10] },
    Prologo { endereco: 0x70a860, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] },
    // quadragesima leva
    Prologo { endereco: 0x6cd490, bytes: &[0x56, 0x8b, 0xf1, 0x8a, 0x86, 0xb2, 0x8e, 0x00, 0x00] },
    // quadragesima primeira leva (prova de rotina grande)
    Prologo { endereco: 0x700bd0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    // quadragesima segunda leva (integradora grande)
    Prologo { endereco: 0x6c0c60, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    // quadragesima terceira leva
    Prologo { endereco: 0x6c51e0, bytes: &[0x83, 0xec, 0x24, 0x53, 0x55, 0x8b, 0xe9] },
    // quadragesima quarta leva
    Prologo { endereco: 0x6bbe40, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x82, 0xf5, 0x00, 0x00] },
    // quadragesima quinta leva
    Prologo { endereco: 0x7164b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] },
    // quadragesima sexta leva
    Prologo { endereco: 0x547810, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] },
    Prologo { endereco: 0x547840, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] },
    Prologo { endereco: 0x5b5910, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x33, 0xd2] },
    // quadragesima setima leva
    Prologo { endereco: 0x6d1770, bytes: &[0x81, 0xec, 0x34, 0x04, 0x00, 0x00] },
    // quadragesima oitava leva (folhas de outros modulos chamadas pelo motor)
    Prologo { endereco: 0x4b0850, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x3d, 0xe8, 0x03, 0x00, 0x00] },
    Prologo { endereco: 0x5c6710, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x04] },
    Prologo { endereco: 0x4cf5a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x53b900, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    // quadragesima nona leva (human_manager.cpp e formation.cpp)
    Prologo { endereco: 0x5fd210, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fd480, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b5a60, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04, 0x3d, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b73a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    // quinquagesima leva
    Prologo { endereco: 0x5409b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6ce310, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] },
    // quinquagesima primeira leva
    Prologo { endereco: 0x6d4330, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x8a, 0x5c, 0x24, 0x14] },
    // quinquagesima segunda leva
    Prologo { endereco: 0x6cdd50, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    // quinquagesima terceira leva
    Prologo { endereco: 0x6bbf60, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] },
    // quinquagesima quarta leva
    Prologo { endereco: 0x6d3de0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x0c] },
    // quinquagesima quinta leva
    Prologo { endereco: 0x6d3610, bytes: &[0x83, 0xec, 0x3c, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    // quinquagesima sexta leva
    Prologo { endereco: 0x6ceb90, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] },
    // quinquagesima setima leva
    Prologo { endereco: 0x6c4290, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6c4470, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6c50d0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    // quinquagesima oitava leva
    Prologo { endereco: 0x5b5290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d10a0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    // quinquagesima nona leva
    Prologo { endereco: 0x5b58e0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x7160f0, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x53b3d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53b4f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6bc030, bytes: &[0x83, 0xec, 0x54, 0x53, 0x55] },
    Prologo { endereco: 0x5fe040, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fd6b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x600a50, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d42a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x92, 0x47, 0x00, 0x00] },
    Prologo { endereco: 0x8982b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x898040, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6be520, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x6bd760, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x897590, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x897760, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x897dc0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x48db80, bytes: &[0x83, 0xec, 0x0c, 0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x76d940, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x6e9820, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x6c6a30, bytes: &[0x83, 0xec, 0x7c, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x6cfc60, bytes: &[0x8b, 0x44, 0x24, 0x1c, 0x8b, 0x54, 0x24, 0x18] },
    Prologo { endereco: 0x5fea50, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x547d30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x547d60, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x540a50, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4cf5f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x53ce90, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0] },
    Prologo { endereco: 0x544410, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5b09d0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] },
    Prologo { endereco: 0x5afe00, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] },
    Prologo { endereco: 0x849ce0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0xdc, 0x00, 0x00, 0x00] },
    // 4 bytes de rotina (mov + ret) e o primeiro nop do alinhamento: o ret sai antes do salto de volta
    Prologo { endereco: 0x8676f0, bytes: &[0x8b, 0x41, 0x40, 0xc3, 0x90] },
    Prologo { endereco: 0x68d1b0, bytes: &[0x8b, 0x51, 0x0a, 0x85, 0xd2] },
    Prologo { endereco: 0x62c3d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x64, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x4b0870, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x33, 0xd2] },
    Prologo { endereco: 0x7e6f20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x6d59b0, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] },
    Prologo { endereco: 0x6d5a10, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x4b0700, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xb0, 0xf6, 0x9c, 0x00] },
    Prologo { endereco: 0x4b0780, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x4b07b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] },
    Prologo { endereco: 0x4b07d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x4b0810, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x6ebd70, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x5afd00, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] },
    Prologo { endereco: 0x5afde0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x5b2e80, bytes: &[0x51, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5b2f10, bytes: &[0x66, 0x8b, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x5b5960, bytes: &[0x56, 0x8d, 0x81, 0xb4, 0x05, 0x00, 0x00] },
    Prologo { endereco: 0x5b59a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x5b2ae0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4bff70, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x66, 0x85, 0xc0] },
    Prologo { endereco: 0x53b610, bytes: &[0x8a, 0x4c, 0x24, 0x04, 0x33, 0xc0] },
    Prologo { endereco: 0x53b620, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x0b] },
    Prologo { endereco: 0x76d780, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x50] },
    Prologo { endereco: 0x76d7b0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x76d7e0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x53a5b0, bytes: &[0xa1, 0xb4, 0x59, 0x9b, 0x00] },
    Prologo { endereco: 0x76c0f0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x76e010, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x76e040, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x76e070, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x535920, bytes: &[0xa1, 0x84, 0x59, 0x9b, 0x00] },
    Prologo { endereco: 0x7751e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x7754c0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x775520, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x775790, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x776380, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x777590, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00] },
    Prologo { endereco: 0x6ec010, bytes: &[0x83, 0xec, 0x54, 0x53, 0x55] },
    Prologo { endereco: 0x6eecb0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x6bbb60, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    // estes dois terminam numa instrucao relativa, refeita no trampolim (RELATIVAS em desvios.rs)
    Prologo { endereco: 0x714d00, bytes: &[0x80, 0x79, 0x27, 0x01, 0x75, 0x15] },
    Prologo { endereco: 0x6f55c0, bytes: &[0x51, 0x56, 0x8b, 0xf1, 0xe8, 0xd7, 0xee, 0xff, 0xff] },
    Prologo { endereco: 0x6b61e0, bytes: &[0x53, 0x8b, 0x15, 0x68, 0xf6, 0x9c, 0x00] },
    Prologo { endereco: 0x6854a0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x6be120, bytes: &[0x83, 0xec, 0x10, 0x53, 0x56] },
    Prologo { endereco: 0x6cd3d0, bytes: &[0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x10] },
    Prologo { endereco: 0x6cc070, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x6cc540, bytes: &[0x83, 0xec, 0x14, 0x8a, 0x44, 0x24, 0x24] },
    Prologo { endereco: 0x6f5b80, bytes: &[0x51, 0x8a, 0x51, 0x27, 0x8a, 0x81, 0x03, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6c4ec0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6c4820, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x713fc0, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x714d40, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x7103e0, bytes: &[0x83, 0xec, 0x14, 0x53, 0x56] },
    Prologo { endereco: 0x6fa250, bytes: &[0x53, 0x55, 0x8b, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x6fa490, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x6cd770, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x8e, 0x82, 0xf5, 0x00, 0x00] },
    Prologo { endereco: 0x70d430, bytes: &[0x66, 0x81, 0x7c, 0x24, 0x04, 0xe8, 0x03] },
    Prologo { endereco: 0x6f5810, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x6bd930, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x6bcbd0, bytes: &[0x83, 0xec, 0x28, 0x53, 0x8b, 0x5c, 0x24, 0x34] },
    Prologo { endereco: 0x6c87e0, bytes: &[0x81, 0xec, 0x3c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d26b0, bytes: &[0x83, 0xec, 0x34, 0xb8, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6d47e0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x62ec00, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x62ece0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x6cfe10, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] },
    Prologo { endereco: 0x6e1cf0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6e1a40, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    Prologo { endereco: 0x6e1940, bytes: &[0x83, 0xec, 0x0c, 0x8a, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x6cda20, bytes: &[0x51, 0x8a, 0x44, 0x24, 0x0c] },
    Prologo { endereco: 0x6e1aa0, bytes: &[0x81, 0xec, 0x04, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6e1090, bytes: &[0x81, 0xec, 0xc4, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x6e2000, bytes: &[0x81, 0xec, 0x24, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x6df460, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6e0230, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6ddab0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
];
const N: usize = 282;

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

/// `_ftol` { 0x945640 }: trunca para inteiro (64 bits; o jogo usa a parte baixa). Fora da faixa
/// (infinito, NaN, |x| >= 2^63) o `fistp` do x87 grava o inteiro indefinido, de parte baixa 0.
fn ftol(x: f64) -> i32 {
    if x.is_finite() && x.abs() < 9.223372036854775808e18 {
        x as i64 as i32
    } else {
        0
    }
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
    percorrer_linha(x1, y1, x2, y2, false, 8, &|x, y, v| contar_na_casa(this, x, y, v))
}

/// { 0x6cc070 } (match_eng.cpp): como 0x6cbc00, contando pela 0x6cd3d0 (modo 2) e com as duas
/// pontas nas linhas retas (na horizontal, x ate 11).
unsafe extern "thiscall" fn jogadores_no_trajeto(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u8, u32, u32) -> u8 = core::mem::transmute(0x6cd3d0usize);
    let lado = (byte(this, 0x8eae) == 0) as i8;
    percorrer_linha(x1, y1, x2, y2, true, 11, &|x, y, v| {
        let grupo = (this as isize + 2 * 45 * (y as isize + 12 * x as isize) + 0x215e) as usize;
        f(this, lado, grupo, v.to_bits(), 0, 2, 6, 0)
    })
}

/// O percurso de 0x6cbc00 e 0x6cc070: `ate_o_fim` inclui a ultima casa nas linhas retas e a
/// horizontal aceita x ate `x_horizontal`; `contar` da os jogadores de cada casa.
#[allow(clippy::too_many_arguments)]
unsafe fn percorrer_linha(x1: i8, y1: i8, x2: i8, y2: i8, ate_o_fim: bool, x_horizontal: i8, contar: &dyn Fn(i8, i8, f32) -> u8) -> u8 {
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
    /// delta + delta (fadd st0, st0)
    Dobro,
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
            Parte::Dobro => *w = (*w).wrapping_add(ftol(delta as f64 + delta as f64) as i16),
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

// ---------- decima quinta leva: acessores pequenos ----------

/// { 0x6f4a30 } (match_pl.cpp): direcao do jogador (palavra 0x19a).
unsafe extern "thiscall" fn direcao(this: usize) -> u16 {
    *((this + 0x19a) as *const u16)
}

/// { 0x6ce0f0 } (match_eng.cpp): zera o guardado do lado (0xf59e para o lado 0, senao 0xf5a2).
unsafe extern "thiscall" fn zerar_guardado(this: usize, lado_b: u8) {
    *((this + if lado_b == 0 { 0xf59e } else { 0xf5a2 }) as *mut u32) = 0;
}

/// { 0x70df20 } (match_pl.cpp): 1 com o x do jogador (0x102) fora de 2..=6.
unsafe extern "thiscall" fn x_fora_do_meio(this: usize) -> u8 {
    !(2..=6).contains(&byte(this, 0x102)) as u8
}

/// { 0x7103c0 } (match_pl.cpp): o jogador e o guardado em 0xf582 do motor dele.
unsafe extern "thiscall" fn e_o_guardado(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    (this == *((motor + 0xf582) as *const usize)) as u8
}

/// { 0x6bba50 } (match_eng.cpp): lado 0 do jogador em 0x4761 (1 se lado 0, senao 0); 0xff sem
/// lance (0x475a zerado) ou sem jogador.
unsafe extern "thiscall" fn lado_do_lance(this: usize) -> u8 {
    let p = *((this + 0x4761) as *const usize);
    if byte(this, 0x475a) == 0 || p == 0 {
        return 0xff;
    }
    (byte(p, 0x27) == 0) as u8
}

/// { 0x6c2fe0 } (match_eng.cpp): endereco do jogador `k` do bloco do `lado`.
unsafe extern "thiscall" fn jogador_no_bloco(this: usize, lado_b: i8, k: i8) -> usize {
    (this as isize + 0x4796 + 0x1be * (lado_b as isize * 20 + k as isize)) as usize
}

/// { 0x6c3920 } (match_eng.cpp): como `tabela_a`, pela posicao em 0x1a.
unsafe extern "thiscall" fn tabela_a_1a(this: usize, jogador: usize) -> u16 {
    let posicao = byte(jogador, 0x1a);
    if !(0..=10).contains(&posicao) {
        return 0;
    }
    let tabela = *((this as isize + 0x8ebc + byte(jogador, 0x27) as isize * 4) as *const usize) as *const u16;
    *tabela.add(posicao as usize)
}

/// { 0x6d4250 } (match_eng.cpp): 1 com o momento (0x8ed4) fora de 0x1ef e 0x3de e o bit 0x80 do
/// byte 0x38 da partida (0x4792) ligado.
unsafe extern "thiscall" fn momento_marcado(this: usize) -> u32 {
    let m = *((this + 0x8ed4) as *const u16);
    let partida = *((this + 0x4792) as *const usize);
    (m != 0x1ef && m != 0x3de && byte(partida, 0x38) as u8 & 0x80 != 0) as u32
}

/// { 0x6f4a40 } (match_pl.cpp): estado do proprio lado (lado 0: 0xf5bd; outros: 0xf5f3).
unsafe extern "thiscall" fn estado_do_lado(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    byte(motor, if lado(this) != 0 { 0xf5f3 } else { 0xf5bd }) as u8
}

/// { 0x6f4a70 } (match_pl.cpp): estado do outro lado.
unsafe extern "thiscall" fn estado_do_adversario(this: usize) -> u8 {
    let motor = *((this + 0x19e) as *const usize);
    byte(motor, if lado(this) != 0 { 0xf5bd } else { 0xf5f3 }) as u8
}

/// { 0x70a890 } (match_pl.cpp): grava 0x19c limitado a -200..=200.
unsafe extern "thiscall" fn gravar_19c(this: usize, v: i16) {
    *((this + 0x19c) as *mut i16) = v.clamp(-200, 200);
}

/// { 0x70a8c0 } (match_pl.cpp, sem `this`): codigos 12, 13, 21, 37, 46 e 59.
unsafe extern "stdcall" fn codigo_de_bola_parada(c: i16) -> u8 {
    matches!(c, 12 | 13 | 21 | 37 | 46 | 59) as u8
}

/// { 0x70df70 } (match_pl.cpp): perto do gol adversario (lado 1: y < 4; lado 0: y > 7).
unsafe extern "thiscall" fn perto_do_gol(this: usize) -> u8 {
    let (l, y) = (lado(this), byte(this, 0x103));
    ((l == 1 && y < 4) || (l == 0 && y > 7)) as u8
}

/// { 0x6bba10 } (match_eng.cpp): limpa o lance (0x475a..=0x4769).
unsafe extern "thiscall" fn limpar_lance(this: usize) {
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
unsafe extern "thiscall" fn somar_disposicao(this: usize, lado_b: i8, delta: i16) {
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if byte(p, 0x19) >= 0 {
            let w = (p + 0x35) as *mut i16;
            *w = (*w).wrapping_add(delta);
        }
    }
}

// ---------- decima sexta leva ----------

/// { 0x6c0890 } (match_eng.cpp): reparte a disposicao (papel 1 de fora; quarta tabela de fatores).
unsafe extern "thiscall" fn repartir_d(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    let f = Fator(0x969040);
    repartir(this, lado_b, delta, [Pula, f, f, f, f, f, Fator(0x968940)]);
}

/// { 0x6c0980 } (match_eng.cpp): reparte a disposicao (papel 1 de fora; quinta tabela).
unsafe extern "thiscall" fn repartir_e(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Pula, Fator(0x967880), Fator(0x967880), Fator(0x96d340), Fator(0x96d338), Fator(0x9689d0), Fator(0x968940)]);
}

/// { 0x6c0b70 } (match_eng.cpp): reparte a disposicao (papel 1 recebe o dobro; sexta tabela).
unsafe extern "thiscall" fn repartir_f(this: usize, lado_b: i8, delta: i16) {
    use Parte::*;
    repartir(this, lado_b, delta, [Dobro, Fator(0x9689e0), Fator(0x968918), Fator(0x968f20), Fator(0x967888), Fator(0x968a98), Fator(0x968a98)]);
}

/// { 0x6bb9a0 } (match_eng.cpp): registra o lance (0x475a..=0x4769) se nao houver jogador nele.
unsafe extern "thiscall" fn registrar_lance(this: usize, codigo: u16, jogador: u32, outro: u32, b4769: u8, b475a: u8, b475b: u8, b475c: u8, b475d: u8, b475e: u8) {
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
unsafe extern "thiscall" fn remover_da_lista(this: usize, alvo: usize) -> u8 {
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
unsafe extern "thiscall" fn entrar_em_campo(this: usize, jogador: usize) -> u8 {
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

/// { 0x70d630 } (match_pl.cpp): perigo pela tabela de floats 12 x 9 em 0xa15e44 (distancia em y a
/// propria linha de fundo, em x ao centro): valor <= 1,0 (ou NaN) da 1; senao, `atras_do_extremo`.
unsafe extern "thiscall" fn em_perigo(this: usize) -> u8 {
    let fundo = if lado(this) != 0 { 11 } else { 0 };
    let dy = (byte(this, 0x103) as i32 - fundo).abs() as i8 as isize;
    let dx = (byte(this, 0x102) as i32 - 4).abs() as i8 as isize;
    let v = f32_em((0xa15e44 + 4 * (dy * 9 + dx)) as usize);
    if !(v as f64 > f64_em(0x967890)) {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cd5f0usize);
    (f(*((this + 0x19e) as *const usize), this) != 0) as u8
}

// ---------- decima setima leva ----------

/// { 0x6bba80 } (match_eng.cpp): confere o lance registrado. 1 sem lance; 2 se o jogador do lance
/// e do lado em 0x8eae, se o lance (codigo diferente de 8) esta na faixa 2..=6 x area do lado, ou
/// se sorteio(200) der 0; senao limpa o lance e da 1.
unsafe extern "thiscall" fn conferir_lance(this: usize) -> u8 {
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
unsafe extern "thiscall" fn caminho_livre_ate(this: usize, lado_b: i8, x: i8, y: i8) -> u8 {
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
unsafe extern "thiscall" fn alcanca_o_ponto(this: usize, x: i8, y: i8) -> u8 {
    let (jx, jy) = (byte(this, 0x102), byte(this, 0x103));
    if x == jx && y == jy {
        return 1;
    }
    let fundo: i8 = if lado(this) != 0 { 11 } else { 0 };
    let motor = *((this + 0x19e) as *const usize);
    let pd: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16 = core::mem::transmute(0x6cb6c0usize);
    let a = pd(motor, x, y, 4, fundo) as i16 as i32;
    let b = pd(motor, x, y, jx, jy) as i16 as i32;
    if (a - b).abs() >= byte(this, 0x10c) as i32 + 10 {
        return 0;
    }
    let dx = (jx as i32 - x as i32).abs() as i8 as isize;
    let dy = (jy as i32 - y as i32).abs() as i8 as isize;
    let t = f32_em((0xa15e44 + 4 * (dy * 9 + dx)) as usize);
    let de: unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32 = core::mem::transmute(0x6cc4e0usize);
    (de(x, y, 4, fundo) >= t) as u8
}

// ---------- decima oitava leva ----------

/// { 0x6d0f80 } (match_eng.cpp): o destaque entre os (byte 0x4d da partida & 0xf) + 11 primeiros
/// de cada lado (inativos com 0x20 = 0xff de fora): pontuacao (16 bits) = disposicao 0x35 +
/// 125 x (0x10 + 2 x 0x0c) + 50 x 0x03 + 35 x 0x06; troca so com pontuacao maior e
/// ftol(disposicao x fator) nao menor. Guarda o primeiro dword da ficha (0x69) em 0x477a.
unsafe extern "thiscall" fn escolher_destaque(this: usize) {
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
unsafe extern "thiscall" fn escolher_por_papel_b(this: usize, argumento: u32) -> u8 {
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
unsafe extern "thiscall" fn casa_da_formacao(this: usize, jogador: usize, x: *mut u8, y: *mut u8) {
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
unsafe fn percentual(this: usize, a: i32, total: i32, pa: isize, pb: isize) {
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
unsafe extern "thiscall" fn calcular_percentuais(this: usize, i: i8) {
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
unsafe extern "thiscall" fn escolher_por_papel_c(this: usize, argumento: u32, modo: u32) -> u8 {
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
unsafe extern "thiscall" fn aplicar_fator(this: usize, fator: f32) {
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
unsafe fn f716450(p: usize, fator: f32) {
    let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(0x716450usize);
    f(p, fator)
}

/// { 0x7165b0 } (match_pl.cpp): valor pela condicao e pelo byte 0xff; com `alvo` igual ao guardado
/// em 0x1a6, formula propria (0xff + 5).
unsafe extern "thiscall" fn valor_pela_condicao(this: usize, alvo: usize) -> i32 {
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
unsafe extern "thiscall" fn aplicar_ao_time(this: usize, jogador: usize, fator: f32) {
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
unsafe extern "thiscall" fn aplicar_ao_lado(this: usize, lado_b: i8, fator: f32) {
    for k in 0..20 {
        let p = jogador_do_bloco(this, lado_b, k);
        if *((p + 0x69) as *const usize) != 0 {
            f716450(p, fator);
        }
    }
}

/// { 0x6c2310 } (match_eng.cpp): escolha pelo valor da condicao: maior sorteio(valor + 5, ou + 25
/// no modo 2) x fator (0,2 com papel 1, senao 1,0).
unsafe extern "thiscall" fn escolher_pela_condicao(this: usize, lado_b: i8, modo: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x7165b0usize);
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

// ---------- vigesima primeira leva: rotinas sem arquivo no mapa ----------

/// { 0x717400 }: palavra 0x1bc do jogador.
unsafe extern "thiscall" fn ler_1bc(this: usize) -> u16 {
    *((this + 0x1bc) as *const u16)
}

/// { 0x7160d0 }: soma ao byte 0x2b do jogador.
unsafe extern "thiscall" fn somar_2b(this: usize, v: u8) {
    let b = (this + 0x2b) as *mut u8;
    *b = (*b).wrapping_add(v);
}

/// { 0x7160e0 }: grava o byte 0x2b do jogador.
unsafe extern "thiscall" fn gravar_2b(this: usize, v: u8) {
    *((this + 0x2b) as *mut u8) = v;
}

/// Chama uma rotina (motor, jogador) do motor do jogador pelo endereco do jogo; sem motor, None.
unsafe fn pelo_motor<R>(j: usize, rotina: usize) -> Option<R> {
    let motor = *((j + 0x19e) as *const usize);
    if motor == 0 {
        return None;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> R = core::mem::transmute(rotina);
    Some(f(motor, j))
}

/// { 0x717430 }: `tabela_a_1a` (0x6c3920) do motor do jogador, ou 0.
unsafe extern "thiscall" fn tabela_a_1a_do_jogador(this: usize) -> u16 {
    pelo_motor(this, 0x6c3920).unwrap_or(0)
}

/// { 0x717450 }: `tabela_b` (0x6c38f0) do motor do jogador, ou 0.
unsafe extern "thiscall" fn tabela_b_do_jogador(this: usize) -> u16 {
    pelo_motor(this, 0x6c38f0).unwrap_or(0)
}

/// { 0x715f30 }: `diferenca_jogador` (0x6c4130) do motor do jogador, ou 0xff.
unsafe extern "thiscall" fn diferenca_do_jogador(this: usize) -> u8 {
    pelo_motor(this, 0x6c4130).unwrap_or(0xff)
}

/// { 0x714d00 } (o prologo curto termina num jne relativo, refeito no trampolim: RELATIVAS em
/// desvios.rs): bola (y em 0x8ea8) no campo de ataque do lado (lado 1: y >= 6; outros: y < 6).
unsafe extern "thiscall" fn bola_no_ataque(this: usize) -> u8 {
    let y = byte(*((this + 0x19e) as *const usize), 0x8ea8);
    (if lado(this) == 1 { y >= 6 } else { y < 6 }) as u8
}

/// { 0x7163c0 }: marcas taticas (0x5b5270) da posicao do jogador. Devolve edx:eax inteiro.
unsafe extern "thiscall" fn marcas_do_jogador_rot(this: usize) -> u64 {
    marcas_do_jogador(this)
}

/// { 0x7163f0 }: com o bit 0x1000 do bloco do lado (0x9766), o byte 6 + lado do motor; senao, o
/// bit 0x800000 das marcas da posicao.
unsafe extern "thiscall" fn marca_800000(this: usize) -> u8 {
    let l = lado(this) as isize;
    let motor = *((this + 0x19e) as *const usize) as isize;
    let bloco = motor + 6371 * l;
    if *((bloco + 0x9766) as *const u32) & 0x1000 != 0 {
        return *((motor + 6 + l) as *const u8);
    }
    (marcas_do_jogador(this) as u32 & 0x800000 != 0) as u8
}

/// { 0x70def0 }: perto do proprio gol (lado 0: y < 3; lado 1: y > 8).
unsafe extern "thiscall" fn perto_do_proprio_gol(this: usize) -> u8 {
    let (l, y) = (lado(this), byte(this, 0x103));
    ((l == 0 && y < 3) || (l == 1 && y > 8)) as u8
}

/// { 0x717c70 }: papel 0x880 em `tabela_a` e em `tabela_b` do jogador (sem motor, 0).
unsafe extern "thiscall" fn papel_880(this: usize) -> u8 {
    let a: u16 = pelo_motor(this, 0x6c38c0).unwrap_or(0);
    if a & 0x880 == 0 {
        return 0;
    }
    let b: u16 = pelo_motor(this, 0x6c38f0).unwrap_or(0);
    (b & 0x880 != 0) as u8
}

/// { 0x6f0970 }: solta a bola: zera 0x4786, 0xf582 e 0x1a7 do motor (0x8eae = 0xff) e as palavras
/// 0x29 e 0x19c do jogador. O motor e relido a cada gravacao.
unsafe extern "thiscall" fn soltar_bola(this: usize) {
    let motor = || *((this + 0x19e) as *const usize);
    *((motor() + 0x4786) as *mut u32) = 0;
    *((motor() + 0x8eae) as *mut u8) = 0xff;
    *((motor() + 0xf582) as *mut u32) = 0;
    *((motor() + 0x1a7) as *mut u8) = 0;
    *((this + 0x29) as *mut u16) = 0;
    *((this + 0x19c) as *mut u16) = 0;
}

/// { 0x6e1cd0 }: o jogador (dos 20 a partir de [this + 0x23]) com a ficha dada, ou 0.
unsafe extern "thiscall" fn jogador_pela_ficha(this: usize, ficha: usize) -> usize {
    let base = *((this + 0x23) as *const usize);
    (0..20).map(|k| base + 0x1be * k).find(|&p| *((p + 0x69) as *const usize) == ficha).unwrap_or(0)
}

/// { 0x6d5cd0 } (sem `this`): byte da tabela de 18 bytes por codigo em 0xb68788 (codigos
/// 0x1f40..0x21e4, bit 15 ignorado); fora, 0.
unsafe extern "stdcall" fn byte_do_codigo(c: u32) -> i32 {
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
unsafe extern "thiscall" fn nota_ajustada(this: usize, base: i8) -> i32 {
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
unsafe extern "thiscall" fn ajustar_17(this: usize, delta: i8) -> u8 {
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

/// { 0x6f0630 } (match_pl.cpp): o jogador pega a bola. Solta a do dono anterior (0xf582, mesmas
/// gravacoes de `soltar_bola`), limpa o estado do lance, credita o passe (0xf576, do mesmo lado:
/// contador 0x2 e disposicao + 0x6c/0x60/0x96/0x78 conforme papel e zona), registra o novo dono,
/// a bola na casa dele, o lado com a posse (0x1a5 + lado) e a contagem de posse (0x8d46 + lado).
/// O motor e relido a cada acesso, como no original.
unsafe extern "thiscall" fn pegar_bola(this: usize) {
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
        let f: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
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

// ---------- vigesima quarta leva: lances de bola parada / arbitragem ----------

/// { 0x6e9470 }: inicia o objeto (motor, pessoa, valor): sem pessoa, 5000; senao o primeiro de
/// 7750, 7250, 6750, 6250, 5750 cuja palavra 0x1e da pessoa passa de sorteio(350, 300, ...), ou 5250.
unsafe extern "thiscall" fn iniciar_rigor(this: usize, motor: u32, pessoa: usize) {
    *(this as *mut u32) = motor;
    *((this + 4) as *mut u32) = pessoa as u32;
    let v = (this + 8) as *mut u16;
    if pessoa == 0 {
        *v = 0x1388;
        return;
    }
    for (n, valor) in [(0x15e, 0x1e46u16), (0x12c, 0x1c52), (0xfa, 0x1a5e), (0xc8, 0x186a), (0x96, 0x1676)] {
        let r = sortear(n);
        if *((pessoa + 0x1e) as *const i16) as i32 > r {
            *v = valor;
            return;
        }
    }
    *v = 0x1482;
}

/// { 0x6bab90 } (match_eng.cpp): o lance registrado continua valendo? 0 sem lance; com o dono da
/// bola (0xf582) do outro lado que o jogador do lance, a bola passando a linha do lance (0x475c)
/// encerra o lance (limpa e da 0); nos demais casos, 1.
unsafe extern "thiscall" fn lance_valendo(this: usize) -> u8 {
    let p = *((this + 0x4761) as *const usize);
    if byte(this, 0x475a) == 0 || p == 0 {
        return 0;
    }
    let l = byte(p, 0x27);
    let dono = *((this + 0xf582) as *const usize);
    if dono == 0 || byte(dono, 0x27) as i32 != (l == 0) as i32 {
        return 1;
    }
    let (y, linha) = (byte(this, 0x8ea8), byte(this, 0x475c));
    if (y > linha && l == 1) || (y < linha && l == 0) {
        limpar_lance(this);
        return 0;
    }
    1
}

/// { 0x6e9540 }: decide se o lance vira falta (ou similar) para o jogador: nao com lance valendo, nem
/// com papel 1 na area; a pessoa (0x4) pode anular (sorteio(0x28 + 15) == 0, -250 no valor 0x8);
/// na area, 3 em 4 anula; senao +175 e, pelo codigo (0x25: 1 em 20; outros: 1 em 155 com 0x8ea9 > 1),
/// grava 4 e sorteio(0x14 do jogador) + 1 nas saidas.
unsafe extern "thiscall" fn decidir_falta(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let motor = *(this as *const usize);
    let valendo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6bab90usize);
    if valendo(motor) != 0 {
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    let na_area: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
    if tabela(motor, j) & 1 != 0 && na_area(j, lado(j) as u8) as u8 != 0 {
        return 0;
    }
    let pessoa = *((this + 4) as *const usize);
    let v = (this + 8) as *mut u16;
    if pessoa != 0 && sortear(byte(pessoa, 0x28) as i32 + 0xf) == 0 {
        *v = (*v).wrapping_add(0xff06);
        return 0;
    }
    if na_area(j, lado(j) as u8) as u8 != 0 && sortear(4) != 0 {
        return 0;
    }
    *v = (*v).wrapping_add(0xaf);
    let marca = if codigo == 0x25 { sortear(0x14) != 0 } else { sortear(0x9b) == 0 && byte(motor, 0x8ea9) > 1 };
    if !marca {
        return 0;
    }
    let r = sortear(byte(j, 0x14) as u8 as i32);
    *valor = (r as u8).wrapping_add(1);
    *tipo = 4;
    1
}

// ---------- vigesima quinta leva: cartoes ----------

/// Cartao amarelo: o primeiro grava `x` em 0x25 (0x24 = 1, tipo 2, peso 2,0); o segundo, em 0x26
/// (0x24 + 1, tipo 3, peso 3,5).
unsafe fn amarelo(j: usize, tipo: *mut u8, x: i8, peso: *mut f32) {
    let n = (j + 0x24) as *mut u8;
    if *n != 0 {
        *n = (*n).wrapping_add(1);
        *tipo = 3;
        *peso = 3.5;
        *((j + 0x26) as *mut i8) = x;
    } else {
        *n = 1;
        *tipo = 2;
        *peso = 2.0;
        *((j + 0x25) as *mut i8) = x;
    }
}

/// Desconto na disposicao pelo cartao: ftol(c x c') ou, com 0x8ed0 / 11 acima de 45, ftol(c'' / (0x8ed0 / 11) x c').
unsafe fn desconto_do_cartao(motor: usize, j: usize) {
    let q = (*((motor + 0x8ed0) as *const i16) as i32 / 11) as i16;
    let base = if q > 0x2d { f32_em(0x96d64c) as f64 / q as f64 } else { f32_em(0x968e70) as f64 };
    let d = (j + 0x35) as *mut i16;
    *d = (*d).wrapping_add(ftol(base * f32_em(0x96d648) as f64) as i16);
}

/// Contador de faltas do lado no motor (0xf5c9 + 54 x lado).
unsafe fn contar_falta(motor: usize, j: usize) {
    let c = (motor as isize + 0xf5c9 + 54 * lado(j) as isize) as *mut u8;
    *c = (*c).wrapping_add(1);
}

unsafe fn na_area_do_jogador(j: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
    f(j, lado(j) as u8) as u8 != 0
}

/// { 0x6e9630 }: punicao da falta (tipo 1): vermelho direto (-1000, tipo 4), amarelo pela
/// severidade contra sorteios, ou so -25; com `flag` (ou papel 1), sorteio(3000): acima de 475
/// vermelho (-750, tipo 6, 9 no registro), acima de 50 (ou na area) amarelo com -375.
unsafe extern "thiscall" fn punir_falta(this: usize, j: usize, tipo: *mut u8, x: i8, severidade: i8, flag: u8, peso: *mut f32) {
    *peso = 1.5;
    let motor = *(this as *const usize);
    contar_falta(motor, j);
    *((j + 0x14) as *mut u8) = (*((j + 0x14) as *const u8)).wrapping_add(1);
    if *tipo != 1 || byte(motor, 0x8eb3) != 0 {
        return;
    }
    let d = (j + 0x35) as *mut i16;
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    if flag == 0 && tabela(motor, j) & 1 == 0 {
        if severidade as i32 > sortear(0xf) + 9 {
            *d = (*d).wrapping_add(-1000);
            *tipo = 4;
            *peso = 5.0;
        } else if severidade as i32 > sortear(5) + 3 {
            desconto_do_cartao(motor, j);
            amarelo(j, tipo, x, peso);
        } else {
            *d = (*d).wrapping_add(-25);
        }
        return;
    }
    let r = sortear(0xbb8) as i16;
    if r > 0x1db {
        *d = (*d).wrapping_add(-750);
        *tipo = 6;
        *peso = 5.0;
        *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
        return;
    }
    if r <= 0x32 && !na_area_do_jogador(j) {
        return;
    }
    *d = (*d).wrapping_add(-375);
    amarelo(j, tipo, x, peso);
}

/// { 0x6ea4e0 }: como `punir_falta`, para outro tipo de lance: inativo, peso 10; tipo 7, peso
/// 1,25; com `flag`, sorteio(3000) acima de 375 e vermelho (tipo 5), senao amarelo com desconto
/// (acima de 50, ou na area); sem `flag`, vermelho pela severidade (marca 0x16 com `expulsar`),
/// amarelo contra sorteio(6) ou -25. Conta a falta no jogador e 0x15 no outro.
unsafe extern "thiscall" fn punir_lance(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, severidade: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    *peso = 1.5;
    if byte(j, 0x19) < 0 {
        *peso = 10.0;
        return;
    }
    let motor = *(this as *const usize);
    let d = (j + 0x35) as *mut i16;
    match *tipo {
        7 => {
            *peso = 1.25;
            contar_falta(motor, j);
        }
        1 if byte(motor, 0x8eb3) == 0 => {
            contar_falta(motor, j);
            if flag != 0 {
                let r = sortear(0xbb8) as i16;
                if r > 0x177 {
                    *d = (*d).wrapping_add(-750);
                    *tipo = 5;
                    *peso = 5.0;
                    *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
                } else if r > 0x32 || na_area_do_jogador(j) {
                    desconto_do_cartao(motor, j);
                    amarelo(j, tipo, x, peso);
                }
            } else if severidade as i32 > sortear(0xf) + 7 {
                *d = (*d).wrapping_add(-1000);
                *tipo = 4;
                *peso = 5.0;
                *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut i8) = x;
                if expulsar != 0 {
                    *((j + 0x16) as *mut u8) = 1;
                }
            } else if severidade as i32 > sortear(6) {
                desconto_do_cartao(motor, j);
                amarelo(j, tipo, x, peso);
            } else {
                *d = (*d).wrapping_add(-25);
            }
        }
        _ => return,
    }
    *((j + 0x14) as *mut u8) = (*((j + 0x14) as *const u8)).wrapping_add(1);
    *((outro + 0x15) as *mut u8) = (*((outro + 0x15) as *const u8)).wrapping_add(1);
}

// ---------- vigesima sexta leva: arbitragem pelo motor (objeto em 0xf5b2) ----------

/// { 0x6cfca0 } (match_eng.cpp): `decidir_falta` do arbitro do motor (this + 0xf5b2).
unsafe extern "thiscall" fn motor_decidir_falta(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(0x6e9540usize);
    f(this + 0xf5b2, j, codigo, tipo, valor)
}

/// { 0x6cfcd0 } (match_eng.cpp): `punir_lance` do arbitro do motor.
unsafe extern "thiscall" fn motor_punir_lance(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8) = core::mem::transmute(0x6ea4e0usize);
    f(this + 0xf5b2, j, outro, tipo, x, sev, flag, peso, expulsar)
}

/// { 0x6cfd10 } (match_eng.cpp): `punir_falta` do arbitro do motor.
unsafe extern "thiscall" fn motor_punir_falta(this: usize, j: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(0x6e9630usize);
    f(this + 0xf5b2, j, tipo, x, sev, flag, peso)
}

// ---------- vigesima setima leva: impedimento (match_official.cpp) ----------

/// { 0x6ea760 } (match_official.cpp): procura um atacante do `lado` em impedimento no lance para a
/// casa de `y` (linha do passe em `linha`). Devolve o jogador (ajustando o valor do arbitro em 0x8)
/// ou 0. Sem lance valendo; `y` alem da linha e nao atras da ultima linha defensiva (0x8eaf para o
/// lado 0, 0x8eb0 para o 1); com `passador`, ele na area do outro lado e jogadores alem da linha.
/// A pessoa do arbitro pode deixar passar (sorteio(0x29 + 25) == 0, -75). Por jogador, conforme a
/// distancia a linha (2, 1, 0 casas ou alem), sorteios contra o byte 0x107.
unsafe extern "thiscall" fn procurar_impedimento(this: usize, lado_b: i8, linha: i8, y: i8, passador: usize) -> usize {
    let motor = *(this as *const usize);
    let valendo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6bab90usize);
    if valendo(motor) != 0 {
        return 0;
    }
    let (l0, l1) = (byte(motor, 0x8eaf), byte(motor, 0x8eb0));
    match lado_b {
        0 if !(y > linha && y >= l0) => return 0,
        1 if !(y < linha && y <= l1) => return 0,
        _ => {}
    }
    if passador != 0 {
        let alem: unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8 = core::mem::transmute(0x6cb700usize);
        if alem(motor, linha, lado_b, 0) == 0 {
            return 0;
        }
    }
    let v = (this + 8) as *mut i16;
    let pessoa = *((this + 4) as *const usize);
    if pessoa != 0 && sortear(byte(pessoa, 0x29) as i32 + 0x19) == 0 {
        *v = (*v).wrapping_add(-75);
        return 0;
    }
    let na_area: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
    for k in 0..20 {
        let p = jogador_do_bloco(motor, lado_b, k);
        if byte(p, 0x19) < 0 || p == passador {
            continue;
        }
        if passador != 0 {
            if na_area(passador, (lado_b == 0) as u8) as u8 == 0 {
                continue;
            }
            let dist = f6f09b0(p, 4, y);
            if ((sortear(4) + 1) as f64) < dist as f64 || dist.is_nan() {
                continue;
            }
        }
        let (py, a) = (byte(p, 0x103) as i32, byte(p, 0x107) as i32);
        let (linha_d, sinal, perto) = match lado_b {
            0 => (l0 as i32, 1, -25i16),
            1 => (l1 as i32, -1, -15i16),
            _ => continue,
        };
        if py == linha_d - 2 * sinal && sortear(6 * a + 500) == 0 {
            *v = (*v).wrapping_add(perto);
            return p;
        }
        if py == linha_d - sinal && sortear(ftol((a as f64) * (a as f64) * (a as f64) + f64_em(0x968890))) == 0 {
            *v = (*v).wrapping_add(-15);
            return p;
        }
        if py == linha_d && sortear(3) != 0 && a < sortear(0x12) {
            *v = (*v).wrapping_add(15);
            return p;
        }
        if (py - linha_d) * sinal > 0 && a < sortear(0x64) {
            return p;
        }
    }
    0
}

/// { 0x6cfd70 } (match_eng.cpp): `procurar_impedimento` do arbitro do motor (this + 0xf5b2).
unsafe extern "thiscall" fn motor_procurar_impedimento(this: usize, lado_b: i8, linha: i8, y: i8, passador: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(0x6ea760usize);
    f(this + 0xf5b2, lado_b, linha, y, passador)
}

// ---------- vigesima oitava leva: cartao pela gravidade ----------

/// Grava o codigo do cartao no registro do jogador: 0x25 com (0x23 + 0x24) == 1 (ou <= 1, `ate_um`),
/// 0x26 com outro total diferente de zero.
unsafe fn registrar_cartao(j: usize, codigo: u8, ate_um: bool) {
    let t = byte(j, 0x23) as i32 + byte(j, 0x24) as i32;
    if t == 1 || (ate_um && t <= 1) {
        *((j + 0x25) as *mut u8) = codigo;
    } else if t != 0 {
        *((j + 0x26) as *mut u8) = codigo;
    }
}

/// { 0x6eab60 } (match_official.cpp): cartao pela gravidade do lance (1..6): sorteio(0x14 do
/// jogador) ajustado pela pessoa do arbitro (0x25) e pela gravidade; segundo amarelo possivel com
/// 0x24; vermelho (-1000, tipo 4) contra sorteio(15) + 7; senao amarelo (desconto, tipo 2/3) contra
/// sorteio(3 x 0x24 + 5) + 2, ou so tipo 9. O codigo do registro fica no lugar do argumento `j`
/// (sem gravidade conhecida, o byte baixo do ponteiro).
unsafe extern "thiscall" fn cartao_pela_gravidade(this: usize, j: usize, tipo: *mut u8, gravidade: u8, flag: u8) {
    let mut nota = sortear(byte(j, 0x14) as u8 as i32) as u8;
    let pessoa = *((this + 4) as *const usize);
    if pessoa != 0 {
        let a = byte(pessoa, 0x25);
        nota = nota.wrapping_add(if a > 18 { 2 } else if a > 12 { 1 } else if a > 6 { 0 } else if a > 2 { 0xff } else { 0xfe });
    }
    let (soma, codigo) = match gravidade {
        1 | 6 => (4, 5),
        2 => (10, 8),
        3 => (7, 1),
        4 | 5 => (100, 12),
        _ => (0, j as u8),
    };
    nota = nota.wrapping_add(soma);
    if byte(j, 0x24) != 0 && (gravidade == 1 || gravidade == 6) && sortear(byte(pessoa, 0x24) as i32) != 0 {
        *tipo = 9;
        return;
    }
    let ativo = byte(j, 0x19) >= 0 && flag == 0;
    if gravidade != 1 && nota as i8 as i32 > sortear(0xf) + 7 {
        if ativo {
            let d = (j + 0x35) as *mut i16;
            *d = (*d).wrapping_add(-1000);
            *tipo = 4;
            let t = byte(j, 0x23) as i32 + byte(j, 0x24) as i32;
            *((j + if t == 0 { 0x25 } else { 0x26 }) as *mut u8) = codigo;
        } else if codigo == 12 || codigo == 8 {
            registrar_cartao(j, codigo, true);
        }
        return;
    }
    let r = sortear(3 * byte(j, 0x24) as i32 + 5);
    if nota as i8 as i32 > r + 2 {
        if !ativo {
            return;
        }
        desconto_do_cartao(*(this as *const usize), j);
        let n = (j + 0x24) as *mut u8;
        if *n != 0 {
            *n = (*n).wrapping_add(1);
            *tipo = 3;
        } else {
            *n = 1;
            *tipo = 2;
        }
        registrar_cartao(j, codigo, false);
    } else if ativo {
        *tipo = 9;
    }
}

/// { 0x6cfd40 } (match_eng.cpp): `cartao_pela_gravidade` do arbitro do motor (this + 0xf5b2).
unsafe extern "thiscall" fn motor_cartao_pela_gravidade(this: usize, j: usize, tipo: *mut u8, gravidade: u8, flag: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8) = core::mem::transmute(0x6eab60usize);
    f(this + 0xf5b2, j, tipo, gravidade, flag)
}

// ---------- vigesima nona leva ----------

/// { 0x6c23f0 } (match_eng.cpp): quinta escolha ponderada pelo papel (0,15 / 0,2 / 0,5 ou 0,3 /
/// 0,65 ou 0,8 / 1,05 / 0,95 ou 0,85), ajustada pela pessoa (palavra 5, bytes 0x37 e 0x43; sem
/// `modo`, a soma 0x40 + 0x36 + 0x26 + 0x1b; sorteio contra 0x21). Sorteio base: com `modo`,
/// 0xa1 - 0x99 - 0xad + c (sem fator); senao 4 em 5, 0xf1 + 0xc5 + 0x8d + c, ou 0xa9 + c. No fim,
/// 1 em 5 descarta a escolha (0xff). O fator fica no argumento e passa adiante.
unsafe extern "thiscall" fn escolher_por_papel_e(this: usize, argumento: u32, modo: u32) -> u8 {
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
const ZONAS_DE_POSSE: [([usize; 4], [usize; 4], [usize; 3]); 3] = [
    ([0x8f52, 0x8f5e, 0x8f62, 0x8e2d], [0x8f16, 0x8f22, 0x8f26, 0x8d88], [0x1f6, 0x1fc, 0x1fe]),
    ([0x8f2a, 0x8f36, 0x8f3a, 0x8dbf], [0x8f3e, 0x8f4a, 0x8f4e, 0x8df6], [0x200, 0x206, 0x208]),
    ([0x8f02, 0x8f0e, 0x8f12, 0x8d51], [0x8f66, 0x8f72, 0x8f76, 0x8e64], [0x20a, 0x210, 0x212]),
];

/// { 0x6cf750 } (match_eng.cpp): conta um instante de posse do `lado` na zona da bola, por periodo
/// (0x8ed4: 0x1ef, 0x3de ou outro). Com `janela`, mantem tambem os totais dos ultimos 55 instantes
/// (janela circular em 0x8e9b): tira o que sai, poe o que entra. Sem janela, lado -1 nao conta.
unsafe extern "thiscall" fn estatistica_de_posse(this: usize, lado_b: i8, janela: u8) {
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
unsafe fn resumir_lados(motor: usize) {
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
unsafe extern "thiscall" fn resumo_da_partida(this: usize, momento: i16) {
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
    let percentuais: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(0x6ce9c0usize);
    percentuais(this, per);
}

// ---------- trigesima segunda leva ----------

/// { 0x6c30e0 } (match_eng.cpp, laco desenrolado 20 vezes no original): o jogador ativo do `lado`
/// mais perto de (a, b) pela tabela (`f6f09b0`), sem `tabela` 1 e, sem `todos`, so com 0x2b <= 1.
/// Distancia zero devolve na hora; partindo de 5000, o menor estritamente. Sem candidato, 0.
unsafe extern "thiscall" fn mais_perto_no_lado(this: usize, lado_b: i8, a: i8, b: i8, todos: u8) -> usize {
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

// ---------- trigesima terceira leva: defesa do goleiro ----------

/// Sorteio da defesa: se sorteio(palavra 0x3b do goleiro) vence sorteio(285), sorteio(b x 0,7) +
/// b x 0,3; senao sorteio(b).
unsafe fn sorte_da_defesa(j: usize, b: i16) -> i32 {
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
unsafe extern "thiscall" fn defesa_do_goleiro(this: usize, a: i8, b: i8, a3: u8, codigo: i8, saida: *mut u8, rebote: *mut u8) -> u16 {
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
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
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
            let alcanca: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(0x713d90usize);
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
fn casa_da_grade(motor: usize, x: i8, y: i8) -> isize {
    motor as isize + 0x215e + 90 * (12 * x as isize + y as isize)
}

/// { 0x6f46b0 } (match_pl.cpp): move o jogador para (x, y): tira da lista do lado na casa antiga
/// (o ultimo da lista ocupa a vaga dele, 0x28) e poe no fim da lista da casa nova; atualiza a bola
/// se ele for o dono. Na mesma casa, nada. Estados impossiveis do original (lista vazia ao tirar,
/// mais de 11 ao por) mostram erro (e, no segundo, encerram); aqui, o primeiro so volta e o segundo
/// encerra como o original.
unsafe extern "thiscall" fn mover_para(this: usize, x: i8, y: i8) {
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
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
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

// ---------- trigesima quinta leva: escolher a casa para onde ir ----------

/// Pontua a casa (x, y) para o jogador em `escolher_casa`; None quando a casa nem entra.
#[allow(clippy::too_many_arguments)]
unsafe fn pontuar_casa(motor: usize, j: usize, x: i8, y: i8, v: f32, d: i8, flags: u8, alvo: (i8, i8)) -> Option<f64> {
    let (jx, jy) = (byte(j, 0x102), byte(j, 0x103));
    let flag2 = flags & 2 != 0;
    if x < 0 || x >= 9 || (jx == x && jy == y && flag2) {
        return None;
    }
    if flag2 {
        let virado: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(0x6fa140usize);
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
unsafe extern "thiscall" fn escolher_casa(this: usize, j: usize, sx: *mut i8, sy: *mut i8, flags: u32) -> u8 {
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

// ---------- trigesima sexta leva: dominio do lance ----------

/// O jogador do lado esta na propria area (x 2..=6; lado 1: y 10..=11; senao y 0..=1)?
unsafe fn na_propria_area(j: usize) -> bool {
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
unsafe extern "thiscall" fn dominio_do_lance(this: usize, codigo: *mut i8, a: *mut u8, b: *mut u8) -> u16 {
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
        let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(0x6cb650usize);
        if c != 0x15 && c != 0x25 && especial(c) == 0 {
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
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
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
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

// ---------- trigesima setima leva: tracado do lance (com chamadas internas gravadas) ----------

/// Chamada a `procurar_no_lado` (0x6cd330) feita pelo tracado: (lado, casa, distancia, a4, tipo, a6).
type ChamarProcurar<'a> = &'a mut dyn FnMut(i8, usize, u32, u32, u32, u32) -> u8;

/// { 0x6ccb60 } com a chamada a 0x6cd330 injetada: percorre as casas da linha de (x0, y0) a
/// (x1, y1) (eixo maior, erro acumulado como Bresenham; a casa final so entra se a linha for um
/// ponto) e, em cada uma, pergunta a `procurar_no_lado` por um jogador de um lado sorteado e
/// depois do outro, com a distancia desde a origem (tabela 12 x 9; fora, 1000). Primeiro que
/// achar, 1; sair do campo ou chegar ao fim, 0. O tipo (por `codigo`) vai junto.
#[allow(clippy::too_many_arguments)]
unsafe fn tracar_com(motor: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32, chamar: ChamarProcurar) -> u8 {
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
unsafe extern "thiscall" fn tracar_lance(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8 = core::mem::transmute(0x6cd330usize);
    tracar_com(this, x0, y0, x1, y1, codigo, a6, a7, &mut |l, c, d, a, t, b| f(this, l, c, d, a, t, b, 1))
}

// ---------- trigesima oitava leva: jogos do dia (match_day.cpp) ----------

/// { 0x6b59a0 } (match_day.cpp): o clube tem jogo na lista do dia ([this + 0x19], quantidade em
/// 0x21, 0x69 bytes por jogo; mandante em 0x20, visitante em 0x24)? Com `so_marcados`, so os jogos
/// com 0x5b preenchido. Clube nulo: o original mostra uma caixa de erro e devolve 0 (aqui, so 0).
unsafe extern "thiscall" fn tem_jogo(this: usize, clube: u32, so_marcados: u32) -> u32 {
    if clube == 0 {
        return 0;
    }
    let n = *((this + 0x21) as *const i32);
    let base = *((this + 0x19) as *const usize) + 0x20;
    for k in 0..n.max(0) as usize {
        let jogo = base + 0x69 * k;
        if so_marcados != 0 && *((jogo + 0x3b) as *const u32) == 0 {
            continue;
        }
        if *(jogo as *const u32) == clube || *((jogo + 4) as *const u32) == clube {
            return 1;
        }
    }
    0
}

/// { 0x6b7cf0 } (match_day.cpp, cdecl): o clube venceu o jogo? Decide pelos penaltis (0x49/0x4a),
/// senao pela prorrogacao (0x47/0x48), senao pelo tempo normal (0x43/0x44); placar -1 nao conta.
/// Jogo ou clube nulo, ou clube fora do jogo: o original mostra uma caixa de erro e devolve 0.
unsafe extern "C" fn venceu_o_jogo(jogo: usize, clube: u32) -> u8 {
    if jogo == 0 || clube == 0 {
        return 0;
    }
    let mandante = *((jogo + 0x1c) as *const u32) == clube;
    if !mandante && *((jogo + 0x20) as *const u32) != clube {
        return 0;
    }
    for (a, b) in [(0x49, 0x4a), (0x47, 0x48), (0x43, 0x44)] {
        let (pa, pb) = (byte(jogo, a), byte(jogo, b));
        if pa > -1 {
            return (if mandante { pa > pb } else { pa < pb }) as u8;
        }
    }
    0
}

// ---------- trigesima nona leva: comentarios (0x6d6f70) ----------

/// Chamada a 0x6d6f70 (enfileirar um evento de comentario): (this, 7 argumentos).
type ChamarComentario<'a> = &'a mut dyn FnMut(usize, [u32; 7]);

/// 0x6d6f70 pelo endereco do jogo.
unsafe fn comentar(this: usize, a: [u32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x6d6f70usize);
    f(this, a[0], a[1], a[2], a[3], a[4], a[5], a[6])
}

/// { 0x7002f0 } (match_pl.cpp) com a chamada injetada: comentario com dois campos -1.
unsafe fn comentario_a_com(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, chamar: ChamarComentario) {
    chamar(this, [a1, a2, a3, a4, u32::MAX, u32::MAX, a5]);
}

/// { 0x7002f0 } (match_pl.cpp): enfileira um comentario (0x6d6f70) com dois campos -1.
unsafe extern "thiscall" fn comentario_a(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
    comentario_a_com(this, a1, a2, a3, a4, a5, &mut |t, a| comentar(t, a))
}

/// { 0x70a860 } (match_pl.cpp) com a chamada injetada: comentario com tres campos -1.
unsafe fn comentario_b_com(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, chamar: ChamarComentario) {
    chamar(this, [a1, a2, u32::MAX, a3, u32::MAX, u32::MAX, a4]);
}

/// { 0x70a860 } (match_pl.cpp): enfileira um comentario (0x6d6f70) com tres campos -1.
unsafe extern "thiscall" fn comentario_b(this: usize, a1: u32, a2: u32, a3: u32, a4: u32) {
    comentario_b_com(this, a1, a2, a3, a4, &mut |t, a| comentar(t, a))
}

// ---------- quadragesima leva: bola fora do campo ----------

/// { 0x6cd490 } (match_eng.cpp) com o comentario injetado: com o lance ja parado (0x8eb2), 1.
/// Solta a bola do dono e, com a bola fora do campo, marca a reposicao (0x8eb2: 2 lateral, 3 ou
/// 4 conforme a linha de fundo e o lado que tocou por ultimo; 0x8eae: o lado que repoe), narra
/// (0x1f76/0x1f75/0x1f77) e devolve 1. Bola em campo, 0.
unsafe fn bola_fora_com(this: usize, lado_b: u8, chamar: ChamarComentario) -> u8 {
    if byte(this, 0x8eb2) != 0 {
        return 1;
    }
    if byte(this, 0x8eae) != -1 {
        *((this + 0x8eae) as *mut i8) = -1;
    }
    let dono = *((this + 0xf582) as *const usize);
    if dono != 0 {
        let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0970usize);
        soltar(dono);
    }
    let (x, y) = (byte(this, 0x8ea7), byte(this, 0x8ea8));
    let narrador = *((this + 0x8f7a) as *const usize);
    let (parada, quem, codigo): (u8, u8, u32) = if !(0..9).contains(&x) {
        let r = (lado_b == 0) as u8;
        (2, r, 0x1f76)
    } else if y < 0 {
        if lado_b == 1 { (4, 0, 0x1f77) } else { (3, 1, 0x1f75) }
    } else if y >= 0xc {
        if lado_b == 1 { (3, 0, 0x1f75) } else { (4, 1, 0x1f77) }
    } else {
        return 0;
    };
    *((this + 0x8eb2) as *mut u8) = parada;
    *((this + 0x8eae) as *mut u8) = quem;
    chamar(narrador, [codigo, this as u32, u32::MAX, quem as u32, u32::MAX, u32::MAX, 0]);
    1
}

// ---------- centesima decima setima leva: escalar (0x6e2000) ----------

/// { 0x6e2000 } (cdecl, match_man.cpp, ~3,9 KB, recursiva): distribui as `n` pessoas de `pessoas`
/// pelas `m` vagas (+ `x` extras) e grava em `saida` a pessoa de cada vaga. Cada par (pessoa, vaga)
/// vale a nota da 0x6e2f20 (codigo da vaga `codigos`, `marcas`, `bytes`, `curtos`, modo, clube,
/// comissao...). Com menos pessoas que vagas, cada pessoa livre procura a vaga onde rende mais do que
/// o ocupante (ate 10000 voltas) e no fim saem as vagas pares com +0xf da ficha 20 e as impares com
/// +0xf abaixo de 10. Com mais pessoas, cada vaga pega a melhor pessoa e os repetidos sao resolvidos
/// trocando pela pessoa de menor perda (ate 10000 voltas). As extras recebem, pela propria 0x6e2000
/// (modo | 0x200), as pessoas que sobraram. Por fim troca pares de vagas enquanto a soma melhorar (ate
/// 100 voltas). A memoria de trabalho e a do jogo (0x945501/0x9452ca), na mesma ordem. Devolve 1, ou 0
/// sem pessoas ou sem vagas.
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn escalar(
    pessoas: usize, a2: u32, codigos: usize, a4: u32, a5: u32, saida: usize, modo: u32, clube: usize, comissao: usize,
    a10: u32, _a11: u32, mut marcas: usize, mut bytes: usize, a14: u32, mut curtos: usize,
) -> u8 {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let nota_fn: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(0x6e2f20usize);
    let recursiva: Escalar = core::mem::transmute(0x6e2000usize);
    let tecnico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fed50usize);
    let n = a2 as u8 as i8 as i32;
    let m = a4 as u8 as i8 as i32;
    let x = a5 as u8 as i8 as i32;
    let tot = m + x;
    let pes = |k: i32| *((pessoas + 4 * k as usize) as *const usize);
    let sai = |i: i32| (saida + 4 * i as usize) as *mut usize;
    let (mut al12, mut al13, mut al15) = (false, false, false);
    if marcas == 0 {
        marcas = if tot != 0 { calloc(tot as u32, 8) } else { 0 };
        al12 = true;
    }
    if bytes == 0 {
        bytes = if tot != 0 { calloc(tot as u32, 1) } else { 0 };
        al13 = true;
    }
    if curtos == 0 {
        curtos = if tot != 0 { calloc(tot as u32, 2) } else { 0 };
        al15 = true;
    }
    let liberar_param = |free: unsafe extern "C" fn(usize)| {
        if al13 && bytes != 0 {
            free(bytes);
        }
        if al12 && marcas != 0 {
            free(marcas);
        }
        if al15 && curtos != 0 {
            free(curtos);
        }
    };
    if n < 1 {
        for i in 0..tot {
            *sai(i) = 0;
        }
        liberar_param(free);
        return 0;
    }
    let jog = calloc(n as u32, 4);
    let jg = |k: i32| *((jog + 4 * k as usize) as *const usize);
    if comissao != 0 {
        for k in 0..n {
            let mut j = *((comissao + 0x23) as *const usize);
            let mut achou = 0usize;
            for _ in 0..20 {
                if *((j + 0x69) as *const usize) == pes(k) {
                    achou = j;
                    break;
                }
                j += 0x1be;
            }
            *((jog + 4 * k as usize) as *mut usize) = achou;
        }
    }
    let mut lc = 0u32;
    if modo & 0x400000 != 0 && clube != 0 {
        let p = *((clube + 0xcf) as *const usize);
        let q = *(0xae2364 as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
            lc = tecnico(*(0xb63c98 as *const usize), p);
        }
    }
    let usado = calloc(n as u32, 1);
    let us = |k: i32| (usado + k as usize) as *mut i8;
    for i in 0..tot {
        *sai(i) = 0;
    }
    if m == 0 && x == 0 {
        free(usado);
        liberar_param(free);
        free(jog);
        return 0;
    }
    // a nota de `pessoa` (com o jogador `j`) na vaga `i`
    let nota = |pessoa: usize, j: usize, i: i32| -> i32 {
        let i = i as usize;
        nota_fn(
            pessoa, *((pessoa + 0x61) as *const usize), j, *((codigos + 2 * i) as *const u16) as u32, modo, clube, comissao, a10, u32::MAX,
            *((marcas + 8 * i) as *const u32), *((marcas + 8 * i + 4) as *const u32), *((bytes + i) as *const u8) as u32, a14,
            *((curtos + 2 * i) as *const u16) as u32, lc,
        )
    };
    let soltar = |alvo: usize| {
        for kk in 0..n {
            if pes(kk) == alvo {
                *us(kk) = 0;
            }
        }
    };
    if n < m {
        // menos pessoas que vagas: cada pessoa livre procura a melhor vaga
        let mut colocadas = 0i8;
        let mut voltas = 0;
        while (colocadas as i32) < n && voltas < 10000 {
            voltas += 1;
            for k in 0..n {
                if *us(k) != 0 {
                    continue;
                }
                let cand = pes(k);
                if cand == 0 || *((cand + 0x61) as *const usize) == 0 {
                    continue;
                }
                let (mut melhor, mut melhor_nota) = (-1i32, 0i32);
                for i in 0..m {
                    let r = nota(cand, jg(k), i);
                    if melhor == -1 || r > melhor_nota {
                        let ocupante = *sai(i);
                        if ocupante == 0 || nota(ocupante, jg(k), i) < r {
                            melhor = i;
                            melhor_nota = r;
                        }
                    }
                }
                if melhor > -1 {
                    let ocupante = *sai(melhor);
                    if ocupante != 0 {
                        for kk in 0..n {
                            if pes(kk) == *sai(melhor) {
                                *us(kk) = 0;
                                colocadas = colocadas.wrapping_sub(1);
                            }
                        }
                    }
                    *sai(melhor) = cand;
                    colocadas = colocadas.wrapping_add(1);
                    *us(k) = 1;
                }
            }
        }
        for i in 0..m {
            let ocupante = *sai(i);
            if ocupante == 0 {
                continue;
            }
            let f = byte(*((ocupante + 0x61) as *const usize), 0xf);
            let sair = if *((codigos + 2 * i as usize) as *const u8) & 1 != 0 { f < 10 } else { f == 20 };
            if sair {
                soltar(ocupante);
                *sai(i) = 0;
            }
        }
    } else {
        // mais pessoas: cada vaga pega a melhor; os repetidos sao resolvidos depois
        let mut conflitos = 0i8;
        let mut melhor_nota = 0i32;
        for i in 0..m {
            let (mut melhor, mut mk) = (0usize, 0i32);
            for k in 0..n {
                let cand = pes(k);
                if cand == 0 || *((cand + 0x61) as *const usize) == 0 {
                    continue;
                }
                let r = nota(cand, jg(k), i);
                if melhor == 0 || r > melhor_nota {
                    melhor = cand;
                    melhor_nota = r;
                    mk = k;
                }
            }
            if melhor != 0 {
                *us(mk) = (*us(mk)).wrapping_add(1);
                if *us(mk) > 1 {
                    conflitos = conflitos.wrapping_add(1);
                }
            }
            *sai(i) = melhor;
        }
        let mut menor_perda = 10000i32;
        let mut voltas = 0;
        let mut k2_atual = 0i32;
        while conflitos > 0 && voltas < 10000 {
            voltas += 1;
            let (mut vaga_t, mut k2_t) = (-1i32, -1i32);
            for k in 0..n {
                let mut troca = 0usize;
                let cand = pes(k);
                if m > 0 {
                    for j in 0..m {
                        if cand == 0 || *us(k) <= 1 {
                            break;
                        }
                        if cand != *sai(j) {
                            continue;
                        }
                        let r_atual = nota(cand, jg(k), j);
                        let (mut outra, mut outra_nota) = (0usize, 0i32);
                        for k2 in 0..n {
                            let o = pes(k2);
                            if o == 0 || *((o + 0x61) as *const usize) == 0 || o == cand {
                                continue;
                            }
                            let r = nota(o, jg(k2), j);
                            if (outra == 0 || r > outra_nota) && (r < r_atual || *us(k2) == 0) {
                                outra_nota = r;
                                outra = o;
                                k2_atual = k2;
                            }
                        }
                        if troca == 0 || r_atual - outra_nota < menor_perda {
                            vaga_t = j;
                            troca = outra;
                            k2_t = k2_atual;
                            menor_perda = (r_atual - outra_nota) as i16 as i32;
                        }
                    }
                    if troca != 0 {
                        *us(k2_t) = (*us(k2_t)).wrapping_add(1);
                        if *us(k2_t) > 1 {
                            conflitos = conflitos.wrapping_add(1);
                        }
                        *sai(vaga_t) = troca;
                        *us(k) = (*us(k)).wrapping_sub(1);
                        conflitos = conflitos.wrapping_sub(1);
                    }
                }
            }
        }
    }
    // as extras: as pessoas que sobraram, pela propria 0x6e2000
    if x != 0 {
        let mut resto = [0usize; 128];
        let mut c = 0usize;
        for k in 0..n {
            if *us(k) == 0 {
                resto[c] = pes(k);
                c += 1;
            }
        }
        recursiva(
            resto.as_ptr() as usize, c as u32, codigos + 2 * m as usize, x as u32, 0, saida + 4 * m as usize, modo | 0x200, clube, comissao, a10, 0,
            marcas, bytes, a14, curtos + 2 * m as usize,
        );
    }
    // trocas de pares enquanto a soma melhorar
    let jogador_de = |p: usize| -> usize {
        if comissao == 0 {
            return 0;
        }
        let mut j = *((comissao + 0x23) as *const usize);
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == p {
                return j;
            }
            j += 0x1be;
        }
        0
    };
    let mut voltas = 0;
    loop {
        let mut mudou = false;
        let mut i = 0;
        while i < m && !mudou {
            let a = *sai(i);
            if a != 0 && *((a + 0x61) as *const usize) != 0 {
                let ja = jogador_de(a);
                let a_i = nota(a, ja, i);
                let mut j = i + 1;
                while j < tot && !mudou {
                    let b = *sai(j);
                    if b != 0 && *((b + 0x61) as *const usize) != 0 {
                        let jb = jogador_de(b);
                        let b_i = nota(b, jb, i);
                        let trocar = if j < m {
                            let a_j = nota(a, ja, j);
                            let b_j = nota(b, jb, j);
                            b_j.wrapping_add(a_i) < a_j.wrapping_add(b_i)
                        } else {
                            a_i < b_i
                        };
                        if trocar {
                            mudou = true;
                            let t = *sai(j);
                            *sai(j) = *sai(i);
                            *sai(i) = t;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        voltas += 1;
        if !mudou || voltas >= 100 {
            break;
        }
    }
    free(usado);
    free(jog);
    liberar_param(free);
    1
}

// ---------- centesima decima oitava leva: a postura do time (0x6df460) ----------

/// { 0x6df460 } (thiscall, match_man.cpp): a postura que a comissao `c` (+0x12 motor, +0x18 lado,
/// +0x1f tecnico, +0x2f clube, +0x33 adversario, +0x37 partida) pede ao time: 1 ataque total, 2
/// ataque, 3 equilibrio, 4 defesa, 5 retranca. Parte da forca dos dois lados (com o motor rodando
/// (+0x8ea5), a soma das notas 0x6e86d0 dos que estao em campo * 50 / 11; senao a reputacao +0x80
/// dos clubes), da posse (+0x1fc/+0x206/+0x210), do tempo (+0x8ed0), dos expulsos (+0x1ad) e do
/// placar agregado (0x6ce310), e ajusta pela liga (posicoes na tabela, 0x4a4850), pelo tecnico e pelo
/// fim do jogo (+0x8ed4 = 0x3de). Guarda o fator em motor+0x476a[lado] (10 e -1 nos extremos); com o
/// motor rodando e o fator a menos de 0,5 do guardado, mantem a postura atual (+0x1ab).
unsafe extern "thiscall" fn postura(c: usize) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let i = |p: usize, o: usize| *((p + o) as *const i32);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let m = u(c, 0x12);
    let lado = byte(c, 0x18);
    let lu = lado as isize as usize;
    let (clube_a, clube_b) = (u(c, 0x2f), u(c, 0x33));
    let (rep_a, rep_b) = (w(clube_a, 0x80), w(clube_b, 0x80));
    let rodando = byte(m, 0x8ea5) != 0;
    // a forca de cada lado (so os 16 bits contam, como no original)
    let (mut proprio, mut outro) = (rep_a, rep_b);
    if rodando {
        let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
        let mut soma = [0i32; 2];
        for (l, s) in soma.iter_mut().enumerate() {
            for k in 0..20 {
                let j = m + 0x22d8 * l + 0x4796 + 0x1be * k;
                if byte(j, 0x19) >= 0 {
                    *s = s.wrapping_add(nota(u(c, 0x1b), u(j, 0x69), clube_a, 2, u32::MAX) as i32);
                }
            }
        }
        let (a, b) = if lado == 0 { (soma[0], soma[1]) } else { (soma[1], soma[0]) };
        proprio = a.wrapping_mul(50) / 11;
        outro = b.wrapping_mul(50) / 11;
    }
    let (mut x11, mut x12, mut x14, mut x16, mut x17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    let agregado: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> i8 = core::mem::transmute(0x6ce310usize);
    agregado(m, lado, &mut x11, &mut x12, &mut x16, &mut x17, &mut x14, 0);
    let proprio = proprio as i16 as i32;
    let outro = outro as i16 as i32;
    let f = (proprio - outro) as f64 / ((1.5 - proprio as f64 * 0.0001) * 6000.0) + 1.0;
    // a posse
    let (p1, p2, p3) = (w(m, 0x1fc), w(m, 0x206), w(m, 0x210));
    let tot = p1 + p2 + p3;
    let posse_b = if tot != 0 { (p3 * 100 / tot) as i8 } else { 0 };
    let posse_a = if tot != 0 { (p1 * 100 / tot) as i8 } else { 0 };
    let r = if lado == 0 && posse_a != 0 {
        Some(posse_b as f64 / posse_a as f64)
    } else if lado == 1 && posse_b != 0 {
        Some(posse_a as f64 / posse_b as f64)
    } else {
        None
    };
    let r = match r {
        Some(q) if q > 1.0 => q + q - 1.0,
        Some(q) => q * 1.29 - 0.29,
        None => f,
    };
    let r = if 0.05 > r { 0.05 } else if 6.0 < r { 6.0 } else { r };
    let mut t = ((w(m, 0x8ed0) << 2) / 99) as i8;
    if t > 20 {
        t = 20;
    }
    let t = t as f64;
    let k = byte(m, 0x1ad + lu);
    let g = f * (25.0 - t) + t * r;
    let mut h = g * 0.04 * (1.0 - k as f64 * 0.07);
    if !(h > 0.0) {
        h = 0.01;
    }
    if x14 != 0 {
        if lado == 1 && x11 != 0 {
            x11 = x11.wrapping_add(2);
        } else if lado == 0 {
            if x12 != 0 {
                x12 = x12.wrapping_add(2);
            } else {
                x11 = x11.wrapping_add(1);
            }
        }
    }
    // os expulsos: 0x6cbbb0 conta os ativos de cada lado
    let mut x13 = 0i8;
    if rodando {
        let contar: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x6cbbb0usize);
        let b = contar(m, (lado == 0) as u32);
        let a = contar(m, lado as u8 as u32);
        x13 = (a as i8).wrapping_sub(b as i8);
        if x13 != 0 {
            x11 = x11.wrapping_add(x13.wrapping_mul(-2));
        }
    }
    let tt = (w(m, 0x8ed0) / 11) as f64;
    if x14 == 0 && x17 == 0 && x11 as i32 + x12 as i32 == 0 {
        let (d, e) = (byte(m, 0x1ad + (lado == 0) as usize), byte(m, 0x1ad + lu));
        if d > e {
            h *= tt * 0.014925373134328358 + 1.25;
        } else if d < e {
            h *= 0.75;
        }
    }
    h *= 1.0 - (x11 as i32 - x12 as i32) as f64 * 0.1;
    if u(clube_a, 0x53) != u(clube_b, 0x53) {
        h *= 0.9;
    } else {
        let jogo = u(c, 0x37);
        let p14 = u(jogo, 0x14);
        if p14 != 0 {
            h += h;
            let comp = if p14 == u(clube_a, 0x57) { u(u(0xadadfc, 0), 4 * u(jogo, 0)) } else { 0 };
            if comp != 0 && u(comp, 8) != 0 {
                // a liga: quem esta acima na tabela e quanto falta
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                let e = busca(u(comp, 8), clube_a);
                if e != 0 {
                    let mut x18 = 0f32;
                    let mut feito = false;
                    if i(e, 0x55) > 1 && dias(0xae2c90, u(e, 0x61) as u32, u(e, 0x65) as u32) < 30 {
                        let (a, b) = (i(e, 0x55).wrapping_neg(), i(e, 0x2d).wrapping_neg().wrapping_shl(1));
                        x18 = (if a < b { a } else { b }) as f32;
                        feito = true;
                    }
                    if !feito && i(e, 0x41) > 1 && dias(0xae2c90, u(e, 0x4d) as u32, u(e, 0x51) as u32) < 10 {
                        let (a, b) = (i(e, 0x41), i(e, 0x19).wrapping_shl(1));
                        x18 = (if a > b { a } else { b }) as f32;
                    }
                    x18 = ((x18 as f64).abs() * x18 as f64) as f32;
                    if k == 0 && 0.0f32 > x18 {
                        x18 = 0.0;
                    }
                    let mut base = h;
                    let pe = *((e + 0x72) as *const u8);
                    if pe < 5 && (x11 as i32 - 3) < x12 as i32 {
                        let e2 = busca(u(u(u(0xadadfc, 0), 4 * u(jogo, 0)), 8), clube_b);
                        if e2 != 0 && *((e2 + 0x72) as *const u8) > pe {
                            let comp = u(u(0xadadfc, 0), 4 * u(jogo, 0));
                            let cx = w(comp, 0x3e);
                            let mut x30 = if cx > 0 { 22.0 / cx as f64 } else { 1.0 };
                            let ax = w(u(jogo, 0x14), 0x69);
                            if ax >= 10 {
                                let lim = ax * 500;
                                if rep_a > lim {
                                    x30 *= ((rep_a - lim) as f64 + (rep_a - 5000) as f64) * 0.0002 + 1.0;
                                }
                            }
                            let p = pow_do_jogo(x30, 1.5) * (*((e2 + 0x72) as *const u8) as i32 - pe as i32) as f64;
                            let p3 = p * p * p;
                            let q = p3 / ((k as i32 * k as i32) as f64 * 50.0 + 250.0) + 1.0;
                            let q = if 1.0 > q { 1.0 } else if 10.0 < q { 10.0 } else { q };
                            base = q * h;
                        }
                    }
                    let z = x18 as f64 * 0.013333333333333334 + 1.0;
                    let z = if 0.25 > z { 0.25 } else if 5.0 < z { 5.0 } else { z };
                    h = base * z;
                }
            }
        }
    }
    let soma = x11 as i32 + x12 as i32;
    if soma > 6 {
        h *= 0.825;
    } else if soma > 4 {
        h *= 0.875;
    }
    let kb = k;
    if kb != 0 || x13 < 0 {
        let pressao = (rep_b > 7500 && rep_b - rep_a > 2000) || (rep_b > 8500 && rep_b - rep_a > 750) || (rep_b > 8000 && x11 as i32 - 1 > x12 as i32);
        if pressao {
            let d = x12 as i32 - x11 as i32;
            if (d < 2 && w(m, 0x8ed4) == 0x1ef) || x12 <= x11 {
                h *= 0.3;
            }
        }
    }
    let d = x11 as i32 - x12 as i32;
    h *= if kb == 0 { 1.0 - d as f64 * 0.1 } else { 0.967 - (kb as i32 + 2 * d) as f64 * 0.067 };
    if x11 > x12 || (x11 == x12 && (x16 != 0 || lado == 1)) {
        let a = if 1.0 > tt { 1.0 } else { tt };
        h *= if 1.0 < 30.0 / a { 1.0 } else { 30.0 / a };
    } else if x11 < x12 || kb == 0 {
        let v = tt * 0.03333333333333333;
        h *= if 1.0 > v { 1.0 } else { v };
    }
    let d2 = x12 as i32 - x11 as i32;
    if d2 > 2 && d2 < 6 {
        h *= 1.01 - d2 as f64 * 0.2;
    }
    let ra_maior = w(clube_a, 0x80) > w(clube_b, 0x80);
    if ((soma < 5 && x13 > -1 && x11 == x12 && x14 == 0 && ra_maior) || x13 > 0) && !(h > 1.0) {
        h = 1.0;
    }
    if soma < 4 && x13 > -1 && x11 as i32 - 2 >= x12 as i32 && ra_maior && !(h > 1.15) {
        h = 1.15;
    }
    let tecnico = u(c, 0x1f);
    if tecnico != 0 && x13 > -1 {
        let e = byte(tecnico, 0xe) as i32;
        let mut f1 = (1.0 + (e - 10) as f64 * 0.06666666666666667) * h;
        let e2 = pow_do_jogo(e as f64, 2.0);
        if f1 < e2 * 0.005 {
            f1 = e2 * 0.0025 + f1 * 0.5;
        }
        let hm1 = h - 1.0;
        h = if f1 < hm1 { hm1 } else { f1 };
    }
    if (rep_b > 7500 || x13 < 0) && kb != 0 && d.abs() < 2 && (x11 >= x12 || tt < 75.0 || soma > 3) {
        let g = tt * 0.02857142857142857 + 0.2;
        if !(h < g) {
            h = g;
        }
    }
    if x17 != 0 && x14 == 0 && x16 == 0 && (x11 == x12 || x11 as i32 + 1 == x12 as i32) && kb == 0 {
        let g = (if tt < 45.0 { tt } else { 45.0 }) * 0.02857142857142857 + 0.2;
        if !(h > g) {
            h = g;
        }
    }
    if x12 > 1 && x11 > 1 {
        let cai = if x14 == 0 && x17 != 0 { (tt + 90.0) / soma as f64 } else { tt / soma as f64 };
        if cai < 18.0 {
            h *= 0.3;
        }
    }
    let fator = (m + 0x476a + 4 * lu) as *mut f32;
    let tempo = w(m, 0x8ed4);
    if tt > 85.0 - h * 10.0 && tempo == 0x3de && (x11 as i32 + 1 == x12 as i32 || (x17 != 0 && x14 == 0 && x16 == 0 && x11 == x12)) {
        *fator = 10.0;
        return 1;
    }
    if soma < 5 && tt > 82.0 - h * 10.0 && tempo == 0x3de && x11 == x12 && x16 == 0 && (h > 1.0 || byte(m, 0x1ab + lu) == 1) {
        *fator = 10.0;
        return 1;
    }
    if h < 1.4 && kb != 0 && d > 1 {
        h = 0.2;
    }
    let retranca = (d > 3 && h > 1.0)
        || (tt > 30.0 && x11 >= x12 && 10.0 / h < ((d + 1) * (d + 1)) as f64 && clube_a != u(clube_b, 0xb3) && clube_a != u(clube_b, 0xb7) && clube_a != u(clube_b, 0xbb));
    if retranca {
        *fator = -1.0;
        return 5;
    }
    if rodando && (h - *fator as f64).abs() < 0.5 {
        return byte(m, 0x1ab + lu) as u8;
    }
    *fator = h as f32;
    if !(h > 0.3) {
        4
    } else if h < 0.7 {
        3
    } else if h > 1.5 {
        1
    } else {
        2
    }
}

// ---------- centesima decima nona leva: a formacao (0x6e0230) ----------

/// { 0x6e0230 } (thiscall, ret 0x10, match_man.cpp): a comissao `c` escolhe a formacao do clube
/// (+0x2f) e carrega a tatica no motor. Com o motor em +0 sorteia a formacao; com `modo` != 2 so
/// recarrega; com tecnico humano a ser consultado (0x5fd210/0x5fea50) usa a tatica dele (0x5fbe20,
/// 0x5fbd00). Senao da nota (0x6e2f20) a cada setor (7, bits 1..0x40) e faixa (5, bits 0x80..0x800)
/// pelas duas melhores das `n` pessoas de `fichas` (o goleiro so na faixa do meio; com menos de 11, o
/// ultimo setor vale metade e as pontas dos setores 1 a 3 ficam com a menor), e julga cada formacao
/// do banco de taticas (0xde1d08): o tempo (+2, ate 3000) e a postura (0x6df460) pesam a nota da
/// formacao (+0x10/+0x18/+0x20, ate 3), o tecnico ajusta (+0x1c, +0xe, +0x1f; a favorita +0x43 vale
/// x1,1 ou ate x3,75, as outras x0,1) e cada vaga soma a nota do seu setor e faixa (0x5b2ae0) e as
/// marcas (0x5b5270/0x5b57c0); o equilibrio dos setores entra pela postura. Grava a melhor (0x8967d0),
/// copia a tatica (0x896660) para o motor (+0x91e2 do lado) e para +0x2b e acerta as instrucoes
/// (0x6ddab0). As notas das formacoes ficam numa lista da CRT do jogo (0x945501/0x9452ca). Devolve 0.
unsafe extern "thiscall" fn escolher_formacao(c: usize, fichas: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let postura: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6df460usize);
    let definir: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> u32 = core::mem::transmute(0x8967d0usize);
    let carregar: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(0x896660usize);
    let instrucoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6ddab0usize);
    let pp = postura(c) as i8;
    let clube = u(c, 0x2f);
    let mut escala = w(clube, 0x80) << 3;
    if escala <= 2000 {
        escala = 2000;
    }
    let m = u(c, 0x12);
    let humano = byte(c, 0x19) as i32;
    let tat = || u(0xde1d08, 0);
    let quantas = || (*((tat() + 0x5fd) as *const i32)).wrapping_add(*((tat() + 0x5f5) as *const i32));
    let melhor: i32;
    let tatica = 'tatica: {
        if byte(m, 0) != 0 {
            melhor = sortear(quantas()) as i16 as i32;
        } else if a4 as u8 != 2 {
            break 'tatica carregar(tat(), clube, humano);
        } else {
            let p = u(clube, 0xcf);
            let q = *(0xae2364 as *const i32);
            if humano == 0 && p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
                let gerente = u(0xb63c98, 0);
                let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
                if consultar(gerente, p) != 0 && opcao(gerente, 2, p) != 0 {
                    let preparar: unsafe extern "thiscall" fn(usize, u32, usize, usize) = core::mem::transmute(0x5fbe20usize);
                    let dele: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fbd00usize);
                    preparar(gerente, 0, clube, p);
                    break 'tatica dele(u(0xb63c98, 0), u(clube, 0xcf));
                }
            }
            melhor = julgar_formacoes(c, fichas, a2, a3, pp, escala, calloc, free);
        }
        definir(tat(), clube, melhor as i8 as i16 as u16 as u32, humano);
        carregar(tat(), clube, humano)
    };
    let lado = byte(c, 0x18) as isize;
    core::ptr::copy_nonoverlapping(tatica as *const u8, (m as isize + 0x91e2 + 0x18e3 * lado) as *mut u8, 0x18e3);
    // ponytail: sem tatica (ou sem +0x2b) o original para o jogo com erro (match_man.cpp 0xc1d); aqui so nao copia
    let copia = u(c, 0x2b);
    if tatica != 0 && copia != 0 {
        core::ptr::copy_nonoverlapping(tatica as *const u8, copia as *mut u8, 0x18e3);
        instrucoes(c);
    }
    0
}

/// O miolo da 0x6e0230: as notas por setor e faixa e o julgamento de cada formacao; devolve a melhor.
#[allow(clippy::too_many_arguments)]
unsafe fn julgar_formacoes(
    c: usize, fichas: usize, a2: u32, a3: u32, pp: i8, escala: i32, calloc: unsafe extern "C" fn(u32, u32) -> usize, free: unsafe extern "C" fn(usize),
) -> i32 {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16) as i32;
    let d = |p: usize, o: usize| *((p + o) as *const f64);
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> i32 =
        core::mem::transmute(0x6e2f20usize);
    let pegar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x896570usize);
    let posicao: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let marcas: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(0x5b5270usize);
    let marca_b: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x5b57c0usize);
    let clube = u(c, 0x2f);
    let n = a2 as u8 as i8 as i32;
    // as notas de cada setor (g) e faixa (s): a soma das duas melhores pessoas
    let mut tab = [0i32; 35];
    for g in 0..7 {
        for s in 0..5 {
            let faixa = if g == 0 {
                if s != 2 {
                    continue;
                }
                0
            } else {
                0x80u32 << s
            };
            let modo = faixa | 1 << g;
            let (mut a, mut b, mut quantos) = (-10000i32, -10000i32, 0);
            for k in 0..n {
                let p = u(fichas + 4 * k as usize, 0);
                let r = nota(p, u(p, 0x61), 0, modo, a3, clube, c, u(c, 0x1b) as u32, u32::MAX, 0, 0, 0, 0, 0, 0).max(-10000);
                match quantos {
                    0 => {
                        a = r;
                        quantos = 1;
                    }
                    1 => {
                        if r > a {
                            b = a;
                            a = r;
                        } else {
                            b = r;
                        }
                        quantos = 2;
                    }
                    _ => {
                        if r > a {
                            b = a;
                            a = r;
                        } else if r > b {
                            b = r;
                        }
                    }
                }
            }
            let soma = a.wrapping_add(b);
            tab[g * 5 + s] = if n < 11 && g == 6 { soma / 2 } else { soma };
        }
    }
    if n < 11 {
        for (x, y) in [(5, 9), (10, 14), (15, 19)] {
            let v = tab[x].min(tab[y]);
            tab[x] = v;
            tab[y] = v;
        }
    }
    let tat = || u(0xde1d08, 0);
    let quantas = || (*((tat() + 0x5fd) as *const i32)).wrapping_add(*((tat() + 0x5f5) as *const i32));
    let lista = calloc(quantas() as u32, 8);
    let (mut melhor, mut melhor_r) = (-1i32, 0f64);
    let tec = u(c, 0x1f);
    let mut idx = 0i32;
    while idx < quantas() {
        let f = u(tat(), 0) + 0x28 * idx as usize;
        let mut t = w(f, 2);
        if t as f64 > 3000.0 {
            t = 3000;
        }
        let v = match pp {
            1 => d(f, 0x10),
            3..=5 => d(f, 0x20),
            _ => d(f, 0x18),
        };
        let v = if v > 3.0 { 3.0 } else { v };
        let mut x = ((3000.0 - t as f64) * 5.0 + v * t as f64) * 0.005;
        let tatica = pegar(tat(), *(f as *const u16) as u32, 0);
        let flags = *((tatica + 0x584) as *const u32);
        if tec != 0 {
            if flags & 0x400 != 0 {
                x += (byte(tec, 0x1c) as i32 - 10) as f64;
            }
            if flags & 0x1000 != 0 {
                x += (10 - byte(tec, 0xe) as i32) as f64;
            }
            if flags & 0x100 != 0 {
                x += (byte(tec, 0x1f) as i32 - 10) as f64;
            }
            let fav = byte(tec, 0x43);
            let favorita = fav > 0 && fav < 10 && {
                let q = u(tat(), 8 + 4 * fav as usize);
                q != 0 && q == u(tatica, 0x187b)
            };
            x *= if !favorita {
                0.1
            } else if n >= 11 {
                let y = n as f64 * 0.2 - 0.2;
                if 3.75 < y { 3.75 } else { y }
            } else {
                1.1
            };
        }
        let (mut f18, mut f20, mut f28, mut f44, mut f54) = (0f32, 0f32, 0f32, 0f32, 0f32);
        let mut soma = 0i32;
        for vaga in 0..11u32 {
            let p1 = posicao(tatica, vaga, 0);
            let p2 = posicao(tatica, vaga, 1) as u8;
            if tec != 0 {
                let r = marcas(tatica, vaga) as u32;
                let fl = marca_b(tatica, vaga);
                if r & 0x8000 != 0 && flags & 0x100 == 0 {
                    x += byte(tec, 0x1f) as f64 * 0.25 - 2.5;
                }
                if r & 0x200000 != 0 {
                    x += byte(tec, 0x15) as f64 - 10.0;
                }
                if r & 0x800000 != 0 {
                    x += byte(tec, 0xe) as f64 * 0.5 - 5.0;
                }
                if fl & 4 != 0 {
                    x += byte(tec, 0x1a) as f64 * 1.5 - 15.0;
                }
            }
            let mais = |v: &mut f32, q: f64| *v = (*v as f64 + q) as f32;
            let (mut setor, mut faixa) = (0usize, -1i32);
            if p1 & 1 != 0 {
                faixa = 2;
            } else if p1 & 2 != 0 {
                mais(&mut f18, 1.0);
                setor = 1;
            } else if p1 & 4 != 0 {
                mais(&mut f18, 1.0);
                setor = 2;
            } else if p1 & 8 != 0 {
                mais(&mut f28, 1.0);
                setor = 3;
            } else if p1 & 0x10 != 0 {
                mais(&mut f54, 1.0);
                setor = 4;
            } else if p1 & 0x20 != 0 {
                mais(&mut f20, 1.0);
                setor = 5;
            } else {
                mais(&mut f44, 1.0);
                setor = 6;
            }
            if p2 & 2 != 0 {
                mais(&mut f18, 0.5);
                setor = 1;
            } else if p2 & 4 != 0 {
                mais(&mut f18, 0.5);
                setor = 2;
            } else if p2 & 8 != 0 {
                mais(&mut f28, 0.5);
                setor = 3;
            } else if p2 & 0x10 != 0 {
                mais(&mut f54, 0.5);
                setor = 4;
            } else if p2 & 0x20 != 0 {
                mais(&mut f20, 0.5);
                setor = 5;
            } else if p2 & 0x40 != 0 {
                mais(&mut f44, 0.5);
                setor = 6;
            }
            if faixa == -1 {
                faixa = if p1 & 0x80 != 0 {
                    0
                } else if p1 & 0x100 != 0 {
                    1
                } else if p1 & 0x200 != 0 {
                    2
                } else if p1 & 0x400 != 0 {
                    3
                } else {
                    4
                };
            }
            soma = soma.wrapping_add(tab[setor * 5 + faixa as usize]);
        }
        // o equilibrio dos setores
        let z = (f18 as f64 - f44 as f64) as f32 as f64;
        let q = |v: f32| pow_do_jogo(v as f64, 2.0);
        let s_ = (q(f44) + q(f44) + q(f20)) - (q(f18) + q(f18)) - q(f28);
        let e = 44.0 - ((32.0 - s_) - (z - f20 as f64 * 0.5).abs() * (z - f20 as f64)) * 0.8;
        let y = match pp {
            1 => (e * 0.015 + 1.0) * x,
            3 => (1.0 - e * 0.015) * x,
            4 => (1.0 - e * 0.025) * x,
            _ => x,
        };
        let r = y * (soma as f64 / escala as f64);
        *((lista + 8 * idx as usize) as *mut f64) = r;
        if melhor == -1 || r > melhor_r {
            melhor_r = r;
            melhor = idx;
        }
        idx += 1;
    }
    if lista != 0 {
        free(lista);
    }
    melhor
}

// ---------- centesima vigesima leva: as instrucoes individuais (0x6ddab0) ----------

/// { 0x6ddab0 } (thiscall, match_man.cpp, ~6,4 KB): a comissao `c` acerta as instrucoes da tatica
/// copiada em +0x2b (0x5b50d0 gerais, 0x5b2ba0 posicao, 0x5b5290 marcas de 64 bits, 0x5b5910 marcacao
/// individual no motor) e no banco de taticas (0x897ee0). Com tecnico humano a ser consultado nao
/// mexe. Em casa (ou com sorteio contra a diferenca de reputacao nas competicoes +0x3a = 1 ou 9) pede
/// 0x20; o tecnico (+0x13, +0x1f, +0x1c) escolhe o ritmo e a marcacao. Um fator (reputacoes dos dois
/// clubes / 17000, o tecnico +0xe e um resto pelos ids) e comparado com os atributos (0x5431e0) de cada
/// um dos 20 do lado em campo: avanco pelas pontas, marcacao individual do adversario de 0x6d4330
/// (com 0x7c1e30 para os meias), passes, chutes de longe, dribles, desarmes, impedimento, cobrancas
/// (acumulado dos que tem +0x2e alto), cruzamentos por tabela e o especialista de bola parada (0x897ee0
/// para quem esta em +6/+0xa/+0xe da comissao).
unsafe extern "thiscall" fn instrucoes(c: usize) {
    let u = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let attr: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x5431e0usize);
    let attr_tec: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x543f30usize);
    let attr_tec2: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let tabela_b: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x717450usize);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5b50d0usize);
    let pos: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x5b2ba0usize);
    let marca: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x5b5290usize);
    let marcas: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(0x5b5270usize);
    let marcar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5b5910usize);
    let banco: unsafe extern "thiscall" fn(usize, usize, u32, usize, i32) = core::mem::transmute(0x897ee0usize);
    let quem_marca: unsafe extern "thiscall" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x6d4330usize);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f32 = core::mem::transmute(0x7c1e30usize);
    let (ca, cb) = (u(c, 0x2f), u(c, 0x33));
    let mut acc = 0i8;
    let f40 = (w(ca, 0x80) as i32 as f64 + w(cb, 0x80) as i32 as f64) * 5.882352941176471e-05;
    let humano = byte(c, 0x19);
    if humano == 0 {
        let p = u(ca, 0xcf);
        let q = *(0xae2364 as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && byte(p, 0x5f) != 0 {
            let gerente = u(0xb63c98, 0);
            let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
            let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
            if consultar(gerente, p) != 0 && opcao(u(0xb63c98, 0), 2, u(u(c, 0x2f), 0xcf)) != 0 {
                return;
            }
        }
    }
    let t = || u(c, 0x2b);
    let em_casa = humano != 0 || {
        let comp = byte(u(c, 0x37), 0x3a);
        (comp == 1 || comp == 9) && {
            let r = sortear(2000).wrapping_add(sortear(-4000));
            (w(u(c, 0x2f), 0x80) as i32 - w(u(c, 0x33), 0x80) as i32) > r
        }
    };
    if em_casa {
        geral(t(), 0x20);
    }
    let tec = u(c, 0x1f);
    if tec == 0 {
        return;
    }
    let f40 = (1.0 - (byte(tec, 0xe) as f64 - 10.0) * 0.03333333333333333) * f40;
    let ritmo = byte(tec, 0x13);
    let r = if ritmo <= 8 {
        2
    } else if ritmo <= 14 {
        1
    } else if ritmo <= 18 {
        4
    } else if ritmo <= 20 {
        8
    } else {
        0
    };
    if r != 0 {
        geral(t(), r);
    }
    let m1f = byte(u(c, 0x1f), 0x1f);
    if m1f >= 15 {
        geral(t(), 0x100);
    } else if m1f <= 5 {
        geral(t(), 0x80);
    }
    if 19.0 - w(u(c, 0x2f), 0x80) as i32 as f64 * 0.0003 < byte(u(c, 0x1f), 0x1c) as f64 && *((t() + 0x187b) as *const u8) & 8 == 0 {
        geral(t(), 0x400);
    }
    let tat = || u(0xde1d08, 0);
    let hum = byte(c, 0x19) as i32;
    banco(tat(), u(c, 0x2f), 0, 0, hum);
    banco(tat(), u(c, 0x2f), 1, 0, byte(c, 0x19) as i32);
    let m = u(c, 0x12);
    let bloco = |k: usize| m + 0x22d8 * byte(c, 0x18) as isize as usize + 0x4796 + 0x1be * k;
    // o acumulado dos que sabem cobrar
    for k in 0..20 {
        let j = bloco(k);
        if byte(j, 0x19) >= 0 && tabela(j) & 0x40 != 0 && tabela(j) & 0x200 != 0 {
            let f = u(j, 0x6d);
            let v = byte(f, 0x2e);
            if v > 14 {
                acc = acc.wrapping_add(ftol(pow_do_jogo((v as i32 - 14) as f64, 1.5)) as i8);
                let v40 = byte(f, 0x40);
                if v40 < 8 {
                    acc = acc.wrapping_add(8 - v40);
                }
            }
        }
    }
    let tc = |o: usize| byte(u(c, 0x1f), o);
    for k in 0..20 {
        let j = bloco(k);
        if byte(j, 0x19) < 0 {
            continue;
        }
        let f = u(j, 0x6d);
        let pos_j = *((j + 0x19) as *const u8) as u32;
        let a = |k: u32| attr(u(j, 0x69), k) as i32;
        let tb = || tabela(j);
        let fb = |o: usize| byte(u(j, 0x6d), o);
        let div = 13 - (attr_tec2(u(c, 0x1f), 0xb) as i32) / 2;
        let resto = (*(u(c, 0x1f) as *const i32)).wrapping_add(*(f as *const i32)) % div;
        let mut f38 = (resto as f64 * 0.05 + 1.0) * f40 * 1.2;
        if f38 < 0.6 {
            f38 = 0.6;
        }
        // avanco pelas pontas
        if *((t() + 0x187b) as *const u32) & 0x1c00000 == 0 {
            let lat = |faixa: u16, ref_: usize| tb() & faixa != 0 && fb(0x14) as i32 >= fb(ref_) as i32 - 1 && fb(0x19) == 20;
            if tb() & 0x10 != 0 && tb() & 0x200 != 0 && fb(0x14) as i32 > fb(0x13) as i32 - 1 && fb(0x19) == 20 && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x220, 1);
            } else if tb() & 0x10 != 0 && tabela_b(j) & 0x20 == 0 && lat(0x100, 0x13) && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 1);
            } else if tb() & 0x10 != 0 && tabela_b(j) & 0x20 == 0 && lat(0x400, 0x13) && f38 * 16.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 1);
            } else if tb() & 0x40 != 0 && lat(0x200, 0x15) && f38 * 17.0 < a(7) as f64 {
                pos(t(), pos_j, 0x220, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x240, 1);
            } else if tb() & 0x40 != 0 && lat(0x100, 0x15) && f38 * 17.0 < a(7) as f64 {
                pos(t(), pos_j, 0x120, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x140, 1);
            } else if tb() & 0x40 != 0 && lat(0x400, 0x15) && f38 * 17.0 < a(7) as f64 {
                pos(t(), pos_j, 0x420, 0);
                pos(t(), byte(j, 0x19) as u8 as u32, 0x440, 1);
            }
        }
        let fl = |v: f32| *((j + 0x4d) as *const f32) as f64 > v as f64;
        if (tb() & 2 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0)
            && fl(8250.0)
            && fb(0x3c) > 12
            && f38 * 17.0 < a(7) as f64
            && fb(0x1a) > 15
            && f38 * 40.0 < (a(0x14) + (tc(0x15) as i32 - 10) * 3) as f64
        {
            marca(t(), pos_j, 0x200000, 0);
            if (*((j + 0x4d) as *const f32) as f64) < 8500.0 || fb(0x3c) < 18 || fb(0x44) > 16 {
                marca(t(), byte(j, 0x19) as u8 as u32, 0x4000, 0);
            }
        } else {
            let b = a(0x14) + a(0xd) + a(7) + (tc(0x15) as i32 + fb(0x1a) as i32 - 20) * 3;
            if f38 * 40.0 > b as f64 {
                marca(t(), pos_j, 0x100000, 0);
            }
        }
        // 0x5b5270 bit 1 e +0x584 bit 8: 2, senao 4
        let dois_ou_quatro = || {
            let v = if marcas(t(), byte(j, 0x19) as u8 as u32) & 1 != 0 && *((t() + 0x584) as *const u8) & 8 != 0 { 2 } else { 4 };
            marca(t(), byte(j, 0x19) as u8 as u32, v, 0);
        };
        if tb() & 0x20 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            dois_ou_quatro();
        }
        if (tb() & 8 != 0 || tb() & 0x10 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && f38 * 16.0 > a(7) as f64 {
            dois_ou_quatro();
            let r = quem_marca(m, *((j + 0x27) as *const u8) as u32, byte(j, 0x19) as u8 as u32);
            if r != -1 {
                let lado_o = (byte(j, 0x19) == 0) as usize;
                let o = m + 0x1be * (r as isize as usize).wrapping_add(20 * lado_o) + 0x4796;
                let blk = || (u(c, 0x12) as isize + 0x91e2 + 0x18e3 * byte(j, 0x27) as isize) as usize;
                let tbo = || tabela(o);
                let reps = || {
                    let ra = w(u(c, 0x2f), 0x80);
                    ra < w(u(c, 0x33), 0x80).wrapping_add(0xfa) || w(u(o, 0x6d), 0xb).wrapping_add(0xfa) > ra
                };
                let lados = || tbo() & 0x200 != 0 || tbo() & 0x100 != 0 || tbo() & 0x400 != 0;
                let resto5 = || *(u(o, 0x69) as *const i32) % 5;
                if u(o, 0x69) != 0 {
                    if (tbo() & 0x20 != 0 || tabela_b(o) & 0x20 != 0)
                        && lados()
                        && reps()
                        && a(0x1c) > 12
                        && a(0x11) > 10
                        && a(0x1f) > 10
                        && fb(0x12) == 20
                        && tb() & 0x10 != 0
                        && (tb() & 0x200 != 0 || tb() & 0x100 != 0 || tb() & 0x400 != 0)
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(10) as i32
                    {
                        if u(u(u(c, 0x12), 0x4792), 0x1c) != u(c, 0x2f) || a(0x1a) > 15 {
                            pos(t(), byte(j, 0x19) as u8 as u32, 0x208, 1);
                        }
                        marcar(blk(), byte(j, 0x19) as u8 as u32, u(o, 0x69));
                    } else if u(o, 0x69) != 0
                        && tb() & 8 != 0
                        && (tb() & 0x200 != 0 || tb() & 0x100 != 0 || tb() & 0x400 != 0)
                        && (tbo() & 0x20 != 0 || tabela_b(o) & 0x20 != 0)
                        && tbo() & 0x200 != 0
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 15
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(7) as i32
                    {
                        marcar(blk(), byte(j, 0x19) as u8 as u32, u(o, 0x69));
                    } else if u(o, 0x69) != 0
                        && (tbo() & 0x10 != 0 || tabela_b(o) & 0x10 != 0)
                        && lados()
                        && reps()
                        && a(0x1f) > 10
                        && a(0x1c) > 12
                        && a(0x13) > 10
                        && a(0x1a) > 10
                        && attr_tec(u(c, 0x1b), 0xb) as i32 > (resto5() as i8).wrapping_add(8) as i32
                    {
                        let fo = u(o, 0x6d);
                        let g = u(0xb74350, 0);
                        let m1 = media(g, fo, 6, 1, 6.5);
                        let m2 = media(u(0xb74350, 0), u(o, 0x6d), 6, 2, 6.5);
                        let m3 = media(u(0xb74350, 0), u(o, 0x6d), 6, 5, 6.5);
                        let (mut st0, mut r3) = (0.0f64, 0.0f32);
                        if m1 as f64 > 2.0 {
                            st0 = m2 as f64 / m1 as f64;
                            r3 = (m3 as f64 / m1 as f64) as f32;
                        }
                        if st0 > 0.35 || r3 as f64 > 0.4 {
                            if (tbo() & 0x20 != 0 || tabela_b(o) & 0x20 != 0) && (fb(0x12) == 20 || fb(0x13) == 20) && a(0x1a) > 14 {
                                pos(t(), byte(j, 0x19) as u8 as u32, 0x208, 1);
                            }
                            marcar(blk(), byte(j, 0x19) as u8 as u32, u(o, 0x69));
                        }
                    }
                }
            }
        }
        // os passes
        let q = f38 * 10.0;
        if (a(0x12) as f64) < q {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x10000000, 0);
        } else if f38 * 13.0 < a(0x12) as f64 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x20000000, 0);
        }
        // chutes de longe
        if (tb() & 8 != 0 || tb() & 0x10 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            let a2 = a(2);
            if (tc(0x1f) as i32 / 5) as f64 + q > a2 as f64 {
                let a3 = a(0x1f);
                let pj = byte(j, 0x19) as u8 as u32;
                if (tc(0x1f) as i32 / 5) as f64 + f38 * 5.0 > a3 as f64 {
                    let v = if marcas(t(), pj) & 0x200 != 0 && *((t() + 0x584) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                    marca(t(), pj, v, 0);
                    marca(t(), byte(j, 0x19) as u8 as u32, 0x8000, 0);
                } else {
                    let a3 = a(0x1f);
                    if (tc(0x1f) as i32 / 5) as f64 + q > a3 as f64 {
                        let v = if marcas(t(), pj) & 0x200 != 0 && *((t() + 0x584) as *const u8) & 0x20 != 0 { 0x400 } else { 0x1000 };
                        marca(t(), byte(j, 0x19) as u8 as u32, v, 0);
                    }
                }
            }
        }
        if byte(j, 0x24) != 0 && a(2) > tc(0x1f) as i32 / 4 + 8 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x800, 0);
        }
        // dribles
        if tb() & 8 != 0 || tb() & 0x10 != 0 || (tc(0xe) > 16 && tb() & 0x40 != 0) {
            let f28 = f38 * 14.0;
            let te4 = tc(0xe) as i32 / 4;
            let pj = || byte(j, 0x19) as u8 as u32;
            let fl8000 = || *((j + 0x4d) as *const f32) as f64;
            if f28 - (te4 as f64) < a(0x14) as f64 && f28 - ((tc(0xe) as i32 / 4) as f64) < a(0xc) as f64 && fl8000() > 8000.0 {
                marca(t(), pj(), 0x800000, 0);
            } else {
                let te4 = tc(0xe) as i32 / 4;
                let lim = q - te4 as f64;
                let alto = lim > a(0x14) as f64 && lim < a(0xc) as f64;
                if alto || fl8000() < 8000.0 {
                    marca(t(), pj(), 0x400000, 0);
                }
            }
        }
        // desarmes: os de meio e ataque (ou quem desarma bem) entram pela nota mais alta
        {
            let pj = || byte(j, 0x19) as u8 as u32;
            let entra = tb() & 0x10 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0 || f38 * 16.0 < a(0xb) as f64;
            let forte = entra && (f38 * 14.0 < a(0xb) as f64 || f38 * 34.0 - ((tc(0xe) as i32 / 4) as f64) < (a(0x15) + a(0xb)) as f64);
            if forte {
                marca(t(), pj(), 0x2000000, 0);
            } else if f38 * 25.0 - (tc(0xe) as i32 / 4) as f64 > (a(0x15) + a(0xb)) as f64 {
                marca(t(), pj(), 0x1000000, 0);
            }
        }
        if tb() & 0x10 != 0 && fb(0x1a) > 12 && fb(0x14) >= fb(0x13) {
            if tb() & 0x80 != 0 && fb(0x19) > fb(0x18) {
                pos(t(), byte(j, 0x19) as u8 as u32, 0x120, 1);
            } else if tb() & 0x800 != 0 && fb(0x19) > fb(0x17) {
                pos(t(), byte(j, 0x19) as u8 as u32, 0x420, 1);
            }
        }
        // impedimento
        let d = || a(0x1b) - a(0x15) - a(0x14);
        if tb() & 0x40 != 0 && tb() & 0x80 == 0 && tb() & 0x800 == 0 && d() > 0 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x8000000, 0);
        } else if d() < -5 {
            marca(t(), byte(j, 0x19) as u8 as u32, 0x4000000, 0);
        }
        // cobrancas
        let l28 = acc as f64;
        let pj = || byte(j, 0x19) as u8 as u32;
        let (k1, k2) = if tb() & 0x80 == 0 && tb() & 0x800 == 0 {
            (f38 * 16.0, q)
        } else if tb() & 8 == 0 && tb() & 0x10 == 0 && tb() & 0x20 == 0 {
            (f38 * 13.0, f38 * 7.0)
        } else {
            (q, f38 * 5.0)
        };
        if k1 - l28 < a(8) as f64 {
            marca(t(), pj(), 0, 4);
        } else if k2 - l28 > a(8) as f64 {
            marca(t(), pj(), 0, 8);
        }
        // cruzamentos pela tabela
        let tv = tb() as i16 as i32;
        let b = || a(0x16) + a(7);
        match tv {
            0x210 | 0x110 | 0xa0 | 0x120 | 0x420 | 0x220 | 0x410 | 0x820 => {
                if f38 * 23.0 < b() as f64 {
                    marca(t(), pj(), 0, 1);
                }
            }
            0x90 | 0x108 | 0x202 | 0x208 | 0x408 | 0x810 => {
                if f38 * 27.0 < b() as f64 {
                    marca(t(), pj(), 0, 1);
                }
            }
            _ => {
                if f38 * 31.0 < b() as f64 {
                    marca(t(), pj(), 0, 1);
                } else if f38 * 20.0 > b() as f64 {
                    marca(t(), pj(), 0, 2);
                }
            }
        }
        // bola parada
        if (tb() & 0x10 != 0 || tb() & 8 != 0 || tb() & 0x20 != 0) && tb() & 0x80 == 0 && tb() & 0x800 == 0 && tabela_b(j) & 0x40 == 0 {
            let ca = u(c, 0x2f);
            let s = a(0x16) + a(9) + a(7);
            if ((w(ca, 0x80) as i32 / 500) as f64 + f38 * 30.0) < s as f64 {
                let pj = u(j, 0x69);
                if u(c, 6) == pj || u(c, 0xa) == pj || u(c, 0xe) == pj {
                    banco(tat(), ca, 0, pj, byte(c, 0x19) as i32);
                }
            }
        }
    }
}

// ---------- centesima decima sexta leva: a substituicao (0x6e1090) ----------

/// Minuto do relogio (+0x8ed0 / 11), no maximo o fim do periodo (+0x8ed4: 45, 90, 105, 120).
unsafe fn minuto_do_periodo(m: usize) -> i8 {
    let min = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
    match *((m + 0x8ed4) as *const i16) {
        0x1ef => min.min(0x2d),
        0x3de => min.min(0x5a),
        0x483 => min.min(0x69),
        0x528 => min.min(0x78),
        _ => min,
    }
}

/// { 0x6e1090 } (ret 4, match_man.cpp) com as chamadas que mexem no jogo injetadas (0x6e2000 escala,
/// 0x6d6f70 narra, 0x897760 grava o capitao, 0x6e1aa0 refaz o time): a comissao `c` faz ate +0x17
/// trocas. Os elegiveis (em campo, ou -2 sem +0x1e) vao para uma lista ordenada pelo qsort da CRT
/// (0x945333 com 0x6e0df0, ou 0x6e0fe0 com 11 ou mais); os `m` primeiros dao as vagas (tabelas
/// 0x717430 para os -2, 0x717410/0x717450; posicao +0x19 ou +0x1a; a primeira e 0 com `goleiro`;
/// marcas 0x5b5270 e byte 0x5b57c0 da posicao); depois delas os reservas livres (+0x19 = -1) do 12o em diante, ate
/// 11 + (+0x4d da competicao & 0xf), entram como candidatos. O modo da escala sai do clube (0x53b900),
/// da forca contra o adversario (+0x80, com sorteios), da competicao (+0x3a, +0x1a = 3 e 0x48f150),
/// de +0x16 e do tecnico humano. Quem ganha vaga e estava no banco entra (0x6cdd00, -1 em +0x17,
/// minuto em +0x1f) com a posicao da vaga; quem fica sem vaga sai (0x6cdd50 com -1, +0x19 = -4, +0x1e
/// = 1, minuto em +0x20). Recompoe as posicoes (0x6ce0f0, 0x6ce110) e narra as trocas (0x1f59 mais de
/// 2, 0x1f5c duas, 0x1f5a uma; sem troca para), cada uma com os comentarios de cansaco (0x211d por
/// sorteio(+0x31) abaixo de -10) e de lesao (0x211e) do que sai e do que entra, e 0x20d7 ou 0x1f5b
/// (+0x22 negativo do que sai). Por fim o mais bem avaliado (0x6e1cf0) vira capitao e o time e refeito.
unsafe fn substituir_com(c: usize, goleiro: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let tabela_c: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717430usize);
    let tabela_b: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717450usize);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(0x5b5270usize);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x48f150usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let t9b23: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(0x5fea50usize);
    let entrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6cdd00usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(0x6cdd50usize);
    let recompor: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(0x6ce0f0usize);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(0x6ce110usize);
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(0x6e1cf0usize);
    let u = |o: usize| *((c + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let trocas = byte(c, 0x17);
    if trocas == 0 {
        return;
    }
    let mut lista = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        let e = byte(j, 0x19);
        if e >= 0 || (e == -2 && byte(j, 0x1e) == 0) {
            lista[n] = j;
            n += 1;
        }
    }
    qsort(lista.as_mut_ptr() as usize, n as u32, 4, if n < 11 { 0x6e0df0 } else { 0x6e0fe0 });
    let vagas = (trocas as i32).min(n as i32).max(0) as usize;
    let mut tab = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut pos = [0i8; 20];
    let mut marca = [0u64; 20];
    let mut bpos = [0u8; 20];
    for i in 0..vagas {
        let p = lista[i];
        if byte(p, 0x19) == -2 {
            tab[i] = tabela_c(p);
            tab_b[i] = 0;
        } else {
            tab[i] = tabela(p);
            tab_b[i] = tabela_b(p);
        }
        pos[i] = if i == 0 && goleiro != 0 {
            0
        } else if byte(p, 0x19) == -2 {
            byte(p, 0x1a)
        } else {
            byte(p, 0x19)
        };
        marca[i] = marcas(u(0x2b), pos[i]);
        bpos[i] = byte_pos(u(0x2b), pos[i]);
    }
    let comp = u(0x37);
    // os reservas entram logo depois das vagas (o resto da lista ordenada fica de fora)
    let mut n1 = vagas;
    let limite = (*((comp + 0x4d) as *const u8) & 0xf) as i32 + 0xb;
    if limite > 0xb {
        let mut p = u(0x23) + 0x132a;
        let mut k = 0xb;
        while k < limite {
            if p != 0 && byte(p, 0x19) == -1 {
                lista[n1] = p;
                n1 += 1;
            }
            k += 1;
            p += 0x1be;
        }
    }
    let mut fichas = [0usize; 20];
    for i in 0..n1 {
        fichas[i] = *((lista[i] + 0x69) as *const usize);
    }
    // o modo da escala
    let clube = u(0x2f);
    let forca = |o: usize| w(*((c + o) as *const usize), 0x80) as i32;
    let mut modo: u32 = if selecao(clube) != 0 { 0x1ffffd2 + 0x8a0cb } else { 0x8a0cb };
    if byte(c, 0x19) != 0 {
        modo = (modo | 0x20100) ^ 0x80000;
    } else {
        let mut feito = false;
        if matches!(byte(comp, 0x3a), 1 | 9) {
            let r = sortear(0x7d0).wrapping_add(sortear(-4000));
            if forca(0x2f) - forca(0x33) > r {
                modo |= 0x100;
                feito = true;
            }
        }
        if !feito {
            let r = sortear(0xbb8) + 0xbb8;
            let checa_selecao = if forca(0x2f) - forca(0x33) > r {
                modo |= 0x100;
                true
            } else if byte(c, 0x1a) != 3 {
                false
            } else {
                let id = *(comp as *const i32);
                id <= -1 || na_competicao(*((*(0xadadfc as *const usize) + id as usize * 4) as *const usize), clube) != 0
            };
            if checa_selecao {
                if selecao(clube) == 0 {
                    modo |= 0x10000;
                }
            } else {
                modo = (modo | 0x100) ^ 0x80000;
            }
        }
    }
    if byte(c, 0x16) != 0 {
        modo |= 0x800000;
    }
    if humano(clube, 1, 1, 0, 0) != 0 && byte(c, 0x19) == 0 {
        let p = *((clube + 0xcf) as *const usize);
        if p != 0 && t9a01(*(0xb63c98 as *const usize), p) != 0 && t9b23(*(0xb63c98 as *const usize), 3, p) != 0 {
            modo |= 0x400000;
        }
    }
    let mut lugar = [0usize; 20];
    let extra = *((u(0x2b) + 0x584) as *const u32);
    let args: [usize; 15] = [
        fichas.as_ptr() as usize, n1, tab.as_ptr() as usize, vagas, 0, lugar.as_mut_ptr() as usize, modo as usize, clube, c, u(0x1b), 0,
        marca.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    pedir_vetores(
        &[(args[0], 4 * n1), (args[2], 2 * vagas), (args[11], 8 * vagas), (args[12], vagas), (args[14], 2 * vagas)],
        &[(args[5], 4 * vagas)],
    );
    chamar(0x6e2000, 0, [args.as_ptr() as u32, n1 as u32, modo, clube as u32, c as u32]);
    let m = u(0x12);
    let mut entraram = [0usize; 20];
    let mut sairam = [0usize; 20];
    let (mut ne, mut ns) = (0usize, 0usize);
    for i in 0..n1 {
        let mut j = u(0x23);
        let mut achou = false;
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == fichas[i] {
                achou = true;
                break;
            }
            j += 0x1be;
        }
        if !achou || j == 0 {
            *(0xb67a34 as *mut u32) = 0;
            continue;
        }
        let vaga = (0..vagas).find(|&k| lugar[k] == *((j + 0x69) as *const usize));
        match vaga {
            Some(k) => {
                if byte(j, 0x19) == -1 {
                    *((c + 0x17) as *mut i8) = byte(c, 0x17).wrapping_sub(1);
                    entrar(m, j);
                    *((j + 0x1f) as *mut i8) = minuto_do_periodo(m);
                    entraram[ne] = j;
                    ne += 1;
                }
                let e = byte(j, 0x19);
                if e >= 0 || e == -1 {
                    *((j + 0x19) as *mut i8) = pos[k];
                }
            }
            None => {
                if byte(j, 0x19) == -1 {
                    continue;
                }
                tirar(m, j, -1);
                *((j + 0x19) as *mut i8) = -4;
                *((j + 0x1e) as *mut u8) = 1;
                *((j + 0x20) as *mut i8) = minuto_do_periodo(m);
                sairam[ns] = j;
                ns += 1;
            }
        }
    }
    recompor(m, byte(c, 0x18));
    manter(m, byte(c, 0x18));
    let narrador = *((m + 0x8f7a) as *const usize);
    let lado_c = byte(c, 0x18) as u8 as u32;
    let codigo = match ne {
        0 => return,
        1 => 0x1f5a,
        2 => 0x1f5c,
        _ => 0x1f59,
    };
    chamar(0x6d6f70, narrador, [codigo, m as u32, u32::MAX, lado_c, u32::MAX]);
    for i in 0..ne {
        let (sai, entra) = (sairam[i], entraram[i]);
        let comentarios = byte(m, 0x8ebb) != 0;
        // cansaco: primeiro o que sai, depois o que entra
        let cansado = if comentarios && sortear(w(sai, 0x31) as i32) < -10 {
            Some(sai)
        } else if comentarios && sortear(w(entra, 0x31) as i32) < -10 {
            Some(entra)
        } else {
            None
        };
        if let Some(p) = cansado {
            chamar(0x6d6f70, narrador, [0x211d, m as u32, u32::MAX, (byte(p, 0x27) == 0) as u32, u32::MAX]);
        }
        let machucado = if comentarios && {
            let r = sortear(w(sai, 0x35) as i32 / 30 + w(sai, 0x33) as i32 - 0xe9);
            r > sortear(0x1e)
        } {
            Some(sai)
        } else if comentarios && {
            let r = sortear(w(entra, 0x33) as i32);
            r > sortear(0x1e)
        } {
            Some(entra)
        } else {
            None
        };
        if let Some(p) = machucado {
            chamar(0x6d6f70, narrador, [0x211e, m as u32, u32::MAX, byte(p, 0x27) as u8 as u32, u32::MAX]);
        }
        let cod = if byte(sai, 0x22) > -1 { 0x20d7 } else { 0x1f5b };
        let a = [cod, m as u32, byte(entra, 0x2e) as u8 as u32, byte(entra, 0x27) as u8 as u32, byte(sai, 0x2e) as u8 as u32];
        PROXIMO_SEXTO.store(byte(sai, 0x27) as u8 as u32, std::sync::atomic::Ordering::Relaxed);
        chamar(0x6d6f70, narrador, a);
    }
    // o capitao
    let (mut melhor, mut nota) = (-1i8, 0i16);
    let mut j = u(0x23);
    for k in 0..20i8 {
        if byte(j, 0x19) >= 0 {
            let v = avaliar(0, j, clube);
            if melhor == -1 || v > nota {
                nota = v;
                melhor = k;
            }
        }
        j += 0x1be;
    }
    chamar(0x897760, *(0xde1d08 as *const usize), [clube as u32, melhor as u8 as u32, byte(c, 0x19) as i32 as u32, 0, 0]);
    chamar(0x6e1aa0, c, [0, 0, 0, 0, 0]);
}

/// O 6o argumento da proxima narracao (o lado de quem sai em 0x20d7/0x1f5b; -1 nas outras).
static PROXIMO_SEXTO: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);

/// { 0x6e1090 } (ret 4, match_man.cpp): ver `substituir_com`.
unsafe extern "thiscall" fn substituir(c: usize, goleiro: u32) {
    substituir_com(c, goleiro as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e2000 => {
                let f: Escalar = core::mem::transmute(0x6e2000usize);
                let v = *(a[0] as usize as *const [usize; 15]);
                return f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14]);
            }
            0x897760 => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(0x897760usize);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6e1aa0usize);
                f(this);
            }
            _ => {
                let sexto = PROXIMO_SEXTO.swap(u32::MAX, std::sync::atomic::Ordering::Relaxed);
                comentar(this, [a[0], a[1], a[2], a[3], a[4], sexto, 0]);
            }
        }
        0
    })
}

// ---------- centesima decima quinta leva: a comissao refaz o time (0x6e1aa0) ----------

/// Argumentos da 0x6e2000 (escalar: distribui as pessoas pelas posicoes, recursiva, aloca), cdecl.
type Escalar = unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u32;

/// { 0x6e1aa0 } (match_man.cpp) com as chamadas que mexem no jogo injetadas (0x6e0230 julga a
/// escalacao, 0x6e2000 escala, 0x6d6f70 narra): a comissao (`c`: +0x12 motor, +0x23 primeiro dos 20
/// jogadores, +0x2b tatica, +0x2f clube) refaz o time. Com exatamente 11 em campo e a escalacao
/// reprovada (0x6e0230 com 0x8c1, 2) narra 0x2007. Depois junta os de linha em campo (fora a tabela
/// 1): posicao +0x19, tabelas 0x717410/0x717450, marcas (0x5b5270) e byte (0x5b57c0) da posicao e a
/// ficha +0x69; escala (0x6e2000 com 0x8800, ou 0x408800 com tecnico humano a ser consultado:
/// 0x600a50, +0x19 da comissao, +0xcf do clube, 0x5fd210, 0x5fea50 com 3) e devolve a cada ficha
/// escalada a posicao do lugar que ela ganhou, refazendo o desempenho (0x6eecb0).
unsafe fn refazer_time_com(c: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let tabela_b: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717450usize);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(0x5b5270usize);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let t9b23: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(0x5fea50usize);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x6eecb0usize);
    let u = |o: usize| *((c + o) as *const usize);
    let mut fichas = [0usize; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        if byte(j, 0x19) >= 0 {
            fichas[n] = *((j + 0x69) as *const usize);
            n += 1;
        }
    }
    if n == 11 {
        pedir_vetores(&[(fichas.as_ptr() as usize, 44)], &[]);
        if chamar(0x6e0230, c, [fichas.as_ptr() as u32, 0xb, 0x8c1, 2, 0]) as u8 != 0 {
            let m = u(0x12);
            chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [0x2007, m as u32, u32::MAX, byte(c, 0x18) as u8 as u32, u32::MAX]);
        }
    }
    let mut pos = [0i8; 20];
    let mut tab = [0u16; 20];
    let mut tab_b = [0u16; 20];
    let mut marca = [0u64; 20];
    let mut bpos = [0u8; 20];
    let mut n = 0usize;
    for k in 0..20 {
        let j = u(0x23) + k * 0x1be;
        if byte(j, 0x19) < 0 || tabela(j) == 1 {
            continue;
        }
        pos[n] = byte(j, 0x19);
        tab[n] = tabela(j);
        tab_b[n] = tabela_b(j);
        marca[n] = marcas(u(0x2b), byte(j, 0x19));
        bpos[n] = byte_pos(u(0x2b), byte(j, 0x19));
        fichas[n] = *((j + 0x69) as *const usize);
        n += 1;
    }
    let clube = u(0x2f);
    let mut modo = 0x8800u32;
    if humano(clube, 1, 1, 0, 0) != 0 && byte(c, 0x19) == 0 {
        let p = *((clube + 0xcf) as *const usize);
        if p != 0 && t9a01(*(0xb63c98 as *const usize), p) != 0 && t9b23(*(0xb63c98 as *const usize), 3, p) != 0 {
            modo = 0x408800;
        }
    }
    let mut lugar = [0usize; 20];
    let extra = *((u(0x2b) + 0x584) as *const u32);
    let args: [usize; 15] = [
        fichas.as_ptr() as usize, n, tab.as_ptr() as usize, n, 0, lugar.as_mut_ptr() as usize, modo as usize, clube, c, u(0x1b), 0,
        marca.as_ptr() as usize, bpos.as_ptr() as usize, extra as usize, tab_b.as_ptr() as usize,
    ];
    pedir_vetores(
        &[(args[0], 4 * n), (args[2], 2 * n), (args[11], 8 * n), (args[12], n), (args[14], 2 * n)],
        &[(args[5], 4 * n)],
    );
    chamar(0x6e2000, 0, [args.as_ptr() as u32, n as u32, modo, clube as u32, c as u32]);
    for i in 0..n {
        let mut j = u(0x23);
        let mut achou = false;
        for _ in 0..20 {
            if *((j + 0x69) as *const usize) == fichas[i] {
                achou = true;
                break;
            }
            j += 0x1be;
        }
        if !achou {
            j = 0;
        }
        let f = *((j + 0x69) as *const usize);
        if let Some(k) = (0..n).find(|&k| lugar[k] == f) {
            *((j + 0x19) as *mut i8) = pos[k];
            desempenho(j, 0);
        }
    }
}

/// { 0x6e1aa0 } (match_man.cpp): ver `refazer_time_com`.
unsafe extern "thiscall" fn refazer_time(c: usize) {
    refazer_time_com(c, &mut |rotina, this, a| match rotina {
        0x6e0230 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(0x6e0230usize);
            f(this, a[0] as usize, a[1], a[2], a[3]) as u32
        }
        0x6e2000 => {
            let f: Escalar = core::mem::transmute(0x6e2000usize);
            let v = *(a[0] as usize as *const [usize; 15]);
            f(v[0], v[1] as u32, v[2], v[3] as u32, v[4] as u32, v[5], v[6] as u32, v[7], v[8], v[9] as u32, v[10] as u32, v[11], v[12], v[13] as u32, v[14])
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
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
unsafe fn jogador_saiu_com(m: usize, j: usize, ja_fora: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let tirar: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(0x6cdd50usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let humano_9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let humano_9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let vaga_marcada: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(0x897590usize);
    let melhor: unsafe extern "thiscall" fn(usize) -> i8 = core::mem::transmute(0x6e1a40usize);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(0x6ce110usize);
    let forca: unsafe extern "thiscall" fn(usize, i8) = core::mem::transmute(0x6bbb60usize);
    let marcacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6bbf60usize);
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
unsafe extern "thiscall" fn jogador_saiu(m: usize, j: usize, ja_fora: u32) {
    jogador_saiu_com(m, j, ja_fora as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6e1aa0usize);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x6e1090usize);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(0x897760usize);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

// Portas de 0x6e1aa0 e 0x6e1090 (a comissao refaz o time e faz a troca, match_man.cpp): unidades da
// fita ate serem portadas.


// ---------- centesima decima terceira leva: o mais bem avaliado da lista (0x6e1940) ----------

/// { 0x6e1940 } (ret 8, match_man.cpp): entre as `n` pessoas de `lista` (com ficha +0x61), a mais
/// bem avaliada pela comissao (0x6e1cf0 com o clube +0x2f); 0 com `n` <= 0 ou nenhuma. Com o objeto
/// +0x1f, se o valor dele (0x544410 com o byte +0x17) ficar abaixo do id da pessoa mod 20, a nota
/// cai para (1,1 - 0,01 x (endereco do objeto mod 20)) x nota, como no original. Empate fica com
/// a primeira.
unsafe extern "thiscall" fn melhor_da_lista(this: usize, lista: *const usize, n: i8) -> usize {
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(0x6e1cf0usize);
    let valor: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    let (mut melhor, mut nota) = (0usize, 0i16);
    for i in 0..n.max(0) as usize {
        let p = *lista.add(i);
        if p == 0 || *((p + 0x61) as *const usize) == 0 {
            continue;
        }
        let mut v = avaliar(p, 0, *((this + 0x2f) as *const usize)) as i32;
        let x = *((this + 0x1f) as *const usize);
        if x != 0 && (valor(x, byte(x, 0x17) as u8 as u32) as i32) < *(p as *const i32) % 20 {
            let m = (x as i32) % 20;
            v = ftol((f64_em(0x9689c0) - m as f64 * f64_em(0x967898)) * v as i16 as f64);
        }
        if melhor == 0 || v as i16 > nota {
            melhor = p;
            nota = v as i16;
        }
    }
    melhor
}

// ---------- centesima decima segunda leva: o mais bem avaliado do lado (0x6e1a40) ----------

/// { 0x6e1a40 } (match_man.cpp): o indice (0..20) do jogador ativo (+0x19 >= 0) mais bem avaliado
/// pela comissao (0x6e1cf0 com o clube +0x2f) entre os 20 do bloco que comeca em +0x23 (de 0x1be em
/// 0x1be); o primeiro ativo entra mesmo com nota negativa, depois so nota estritamente maior. -1 sem
/// nenhum ativo.
unsafe extern "thiscall" fn melhor_avaliado(this: usize) -> i8 {
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(0x6e1cf0usize);
    let clube = *((this + 0x2f) as *const usize);
    let mut p = *((this + 0x23) as *const usize);
    let (mut melhor, mut nota) = (-1i8, 0i16);
    for k in 0..20i8 {
        if byte(p, 0x19) >= 0 {
            let v = avaliar(0, p, clube);
            if melhor == -1 || v > nota {
                nota = v;
                melhor = k;
            }
        }
        p += 0x1be;
    }
    melhor
}

// ---------- centesima decima leva: a avaliacao do jogador pela comissao (0x6e1cf0) ----------

/// { 0x6e1cf0 } (cdecl, match_man.cpp): a nota (i16) que a comissao da a `pessoa` (ou a pessoa do
/// jogador `jogador` +0x69) para o `clube`; -10000 sem pessoa. Soma o encaixe (0x53ce90) em
/// (encaixe^2 + 5) x 0,1 negativo, a diferenca da nota +0xd da ficha para a reputacao do clube
/// (x 0,002 + 1,5, entre -4 e 4), o contrato (0x4cf5f0: +0x44 / 75 + 0,5, ajustado pelo tipo em +0x4f
/// >> 4: -10, +2,5, +2, -3, -15, -20, -50) e a ficha +0x2f vezes o fator da idade (dias de vida pela
/// data de hoje 0xae2c90 e a de nascimento +0x3e, 0x54cdb0; nas selecoes, +0x22 x 75): acima de 1000
/// dias, 1 + (dias - 1000) x 0,000125. Depois pela idade +0x18 (x0,5 abaixo de 18, x0,75 abaixo de
/// 21, x0,9 abaixo de 24, x1,05 acima de 25), x0,9 se os dias ficarem abaixo de (50 - +0x59) x 10 e
/// x(0,01 x +0x5b + 0,9) com +0x5b abaixo de 10.
unsafe extern "C" fn avaliar_jogador(pessoa: usize, jogador: usize, clube: usize) -> i16 {
    let encaixe: unsafe extern "C" fn(usize, usize) -> i8 = core::mem::transmute(0x53ce90usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let d8 = |a: usize| f64_em(a);
    let p = if jogador != 0 { *((jogador + 0x69) as *const usize) } else { pessoa };
    if p == 0 {
        return -10000;
    }
    let ficha = *((p + 0x61) as *const usize);
    let enc = encaixe(p, clube) as f64;
    let mut y = (*((ficha + 0xd) as *const i16) as i32 - *((clube + 0x80) as *const i16) as i32) as f64 * d8(0x969048) + d8(0x968940);
    if !(d8(0x96d220) <= y) {
        y = -4.0;
    } else if d8(0x968938) < y {
        y = 4.0;
    }
    let mut x = 0.0f64;
    let e = contrato(*(0xae1050 as *const usize), p, clube);
    if e != 0 {
        x = byte(e, 0x44) as f64 * d8(0x968ef8) + d8(0x968968);
        let ajuste = match byte(e, 0x4f) as u8 >> 4 {
            0 | 5 => -d8(0x9688b0),
            1 => d8(0x968e18),
            2 => d8(0x968950),
            4 => -d8(0x968930),
            6 => -d8(0x968e08),
            7 => -d8(0x968ea8),
            8 => -d8(0x968ff8),
            _ => 0.0,
        };
        if ajuste != 0.0 {
            x += ajuste;
        }
    }
    let a = if selecao(clube) != 0 {
        *((p + 0x22) as *const u8) as f64 * d8(0x96d520)
    } else {
        dias(0xae2c90, *((p + 0x3e) as *const u32), *((p + 0x42) as *const u32)) as f64
    };
    let f = if a > d8(0x968908) { (a - d8(0x968908)) * d8(0x96a6b8) + d8(0x967890) } else { 1.0 };
    let base = byte(ficha, 0x2f) as f64 * f - (enc * enc + d8(0x9688a0)) * d8(0x967880);
    let mut v = ftol(base + y + x + d8(0x968968)) as i16;
    let idade = byte(p, 0x18);
    let k = if idade < 0x12 {
        Some(d8(0x968968))
    } else if idade < 0x15 {
        Some(d8(0x969040))
    } else if idade < 0x18 {
        Some(d8(0x9689d8))
    } else if idade > 0x19 {
        Some(d8(0x9689b8))
    } else {
        None
    };
    if let Some(k) = k {
        v = ftol(v as f64 * k) as i16;
    }
    if a < ((0x32 - byte(p, 0x59) as i32) * 10) as f64 {
        v = ftol(v as f64 * d8(0x9689d8)) as i16;
    }
    let moral = byte(p, 0x5b);
    if moral < 10 {
        v = ftol((moral as f64 * d8(0x967898) + d8(0x9689d8)) * v as f64) as i16;
    }
    v
}

// ---------- centesima nona leva: a lesao na dividida (0x6cfe10) ----------

/// Codigo do lance -> multiplicador do risco de quem tem a bola (tabelas 0x6d08b0/0x6d0884).
const RISCO_POR_LANCE: [u8; 0x2f] = [0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 10, 10, 10, 10, 10, 10, 2, 10, 3, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 2, 4, 10, 10, 10, 10, 10, 10, 5, 6, 7, 8, 2, 2, 9];

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
unsafe fn lesao_na_dividida_com(m: usize, a: usize, tipo_a: i16, b: usize, tipo_b: i16, direto: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let gravidade: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x62ec00usize);
    let grau: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ece0usize);
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
unsafe extern "thiscall" fn lesao_na_dividida(m: usize, a: usize, tipo_a: u32, b: usize, tipo_b: u32, direto: u32) {
    lesao_na_dividida_com(m, a, tipo_a as u16 as i16, b, tipo_b as u16 as i16, direto as u8, &mut |rotina, this, x| match rotina {
        0x62ed70 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
            f(this, x[0] as usize) as u32
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x6d0980usize);
            f(this, x[0], x[1], x[2] as usize, x[3] as usize);
            0
        }
    })
}

// Portas de 0x62ed70 (lesao atual: atualiza a ficha pela 0x62c680) e 0x6d0980 (aplica a lesao,
// alocando): unidades da fita.
static mut PORTA_62ED70: usize = 0;
static mut PORTA_6D0980: usize = 0;
const PROLOGO_62ED70: Prologo = Prologo { endereco: 0x62ed70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] };
const PROLOGO_6D0980: Prologo = Prologo { endereco: 0x6d0980, bytes: &[0x51, 0x57, 0x8b, 0xf9, 0xc7, 0x44, 0x24, 0x04, 0x00, 0x00, 0x00, 0x00] };

unsafe extern "thiscall" fn porta_62ed70(l: usize, pessoa: usize) -> usize {
    let o: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(PORTA_62ED70);
    if !gravando() {
        return o(l, pessoa);
    }
    chamar_gravando(0x62ed70, l, [pessoa as u32, 0, 0, 0, 0], || o(l, pessoa) as u32) as usize
}
unsafe extern "thiscall" fn porta_6d0980(m: usize, tipo: u32, chance: u32, p: usize, o: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(PORTA_6D0980);
    if !gravando() {
        return f(m, tipo, chance, p, o);
    }
    chamar_gravando(0x6d0980, m, [tipo, chance, p as u32, o as u32, 0], || {
        f(m, tipo, chance, p, o);
        0
    });
}

// ---------- centesima oitava leva: as folhas das lesoes (0x62ec00, 0x62ece0) ----------

/// { 0x62ec00 } (ret 4, injury.cpp): o byte de gravidade do tipo de lesao `tipo` (tabela de 13 bytes
/// por tipo em 0x9e7ad9).
unsafe extern "thiscall" fn gravidade_da_lesao(_lesoes: usize, tipo: i8) -> u8 {
    *((0x9e7ad9 + tipo as isize as usize * 13) as *const u8)
}

/// { 0x62ece0 } (ret 4, injury.cpp): o grau da lesao da pessoa (registro de 31 bytes por id na
/// tabela das lesoes, +0x13, pelo menos 1), ou 0 sem pessoa, sem ficha (+0x61), id fora da tabela
/// (0xae2364) ou sem lesao (+0x12 negativo).
unsafe extern "thiscall" fn grau_da_lesao(lesoes: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) || *((pessoa + 0x61) as *const usize) == 0 {
        return 0;
    }
    let r = *(lesoes as *const usize) + (id as usize).wrapping_mul(31);
    if byte(r, 0x12) <= -1 {
        return 0;
    }
    let g = byte(r, 0x13);
    if g > 1 { g as u8 } else { 1 }
}

// ---------- centesima setima leva: o uniforme (0x6d47e0) ----------

/// { 0x6d47e0 } (cdecl, match_eng.cpp): qual dos tres uniformes do `clube` (pares de cores +0x83/+0x87,
/// +0x8b/+0x8f, +0x93/+0x97) usar contra o `outro`. Sem clube ou sem outro, registra o erro e devolve 0.
/// Para cada par, a 0x48ce70 (que devolve 0) decide se troca as cores (0x48ce90). Sem `qualquer`, o
/// par vale se a segunda cor nao choca (0x540490) com a do outro (+0x87 dele, ou a da tabela 0xae23ec
/// pelo indice 0x9cf17c); devolve 1, 2 ou 3 conforme o par, 1 se o par tiver cor zero, 2 se faltar
/// o 2o ou o 3o uniforme, 1 se nenhum servir.
unsafe extern "C" fn uniforme(clube: usize, outro: usize, qualquer: u32) -> u8 {
    let decide: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x48ce70usize);
    let trocar: unsafe extern "C" fn(*mut usize, *mut usize) -> u32 = core::mem::transmute(0x48ce90usize);
    let choca: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x540490usize);
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
        if c != 0 { c } else { *(0xae23ec as *const usize) + *(0x9cf17c as *const i32) as usize * 58 }
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
unsafe extern "C" fn evoluir_jogador(jm: usize, pessoa: usize, clube: usize, rep: usize, comp: usize, nota: f32, media_b: u32, forte: u32) {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let moral_jogo: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x7172d0usize);
    let moral_pessoa: unsafe extern "C" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x544710usize);
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
unsafe extern "thiscall" fn desafio_adversario(m: usize, atacante: usize, lado: i8, tipo: i8, a: *mut u8, b: *mut u8, nota: *mut i16, lance: *mut i8, arriscado: *mut i8, livre: u8) -> usize {
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7163c0usize);
    let faixa: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6f5b20usize);
    let mais_perto: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cb780usize);
    let x_fora: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x70df20usize);
    let perto_gol: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x70def0usize);
    let limite: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x70df40usize);
    let valor: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7165b0usize);
    let dominio: unsafe extern "thiscall" fn(usize, *mut i8, *mut u8, *mut u8) -> u16 = core::mem::transmute(0x70d690usize);
    let relacao: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
    let desgastar: unsafe extern "thiscall" fn(usize, f32, i16) = core::mem::transmute(0x6f56c0usize);
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
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
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
                let erro: unsafe extern "C" fn(usize, usize, i32) = core::mem::transmute(0x944c9fusize);
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
unsafe fn pressao_com(m: usize, j: usize, saida: *mut i16, lista: usize, copiar: &mut dyn FnMut(usize, usize)) -> u8 {
    let na_lista: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x714c60usize);
    let pela_posicao: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6f4600usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
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
unsafe extern "thiscall" fn pressao(m: usize, j: usize, saida: *mut i16, lista: usize) -> u8 {
    pressao_com(m, j, saida, lista, &mut |l, p| {
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
        f(l, p, 0x1be);
    })
}

// Porta de 0x68d0d0 (poe na lista a copia de um bloco, alocando): unidade da fita, para a copia
// Rust nao alocar de novo.
static mut PORTA_68D0D0: usize = 0;
const PROLOGO_68D0D0: Prologo = Prologo { endereco: 0x68d0d0, bytes: &[0x6a, 0xff, 0x68, 0x56, 0xcb, 0x95, 0x00] };

unsafe extern "thiscall" fn porta_68d0d0(l: usize, p: usize, n: u32) {
    let o: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(PORTA_68D0D0);
    if !gravando() {
        return o(l, p, n);
    }
    chamar_gravando(0x68d0d0, l, [p as u32, n, 0, 0, 0], || {
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
unsafe fn periodo_encerrado_com(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let folego: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x6f5810usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0970usize);
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
            let erro: unsafe extern "C" fn(usize, usize, i32) = core::mem::transmute(0x944c9fusize);
            erro(0xde1f64, 0xa158f8, per);
            0
        }
    }
}

/// { 0x6bd930 } (match_eng.cpp): ver `periodo_encerrado_com`.
unsafe extern "thiscall" fn periodo_encerrado(m: usize) -> u8 {
    periodo_encerrado_com(m, &mut |rotina, this, a| {
        match rotina {
            0x6be120 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x6be120usize);
                f(this, a[0]);
            }
            0x6ce460 => {
                let f: unsafe extern "thiscall" fn(usize, i16) = core::mem::transmute(0x6ce460usize);
                f(this, a[0] as i16);
            }
            _ => comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]),
        }
        0
    })
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
unsafe extern "thiscall" fn recuperar_folego(j: usize, teto: u8) {
    let lesao: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(0x62c3d0usize);
    let limite: unsafe extern "thiscall" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x62c590usize);
    let papel: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
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
unsafe fn defender_com(j: usize, forca: i16, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u32 {
    let ajustar_17: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x7172d0usize);
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x6eecb0usize);
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
unsafe extern "thiscall" fn defender(j: usize, forca: i16) -> u32 {
    defender_com(j, forca, &mut |rotina, this, a| match rotina {
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(0x6cd770usize);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, 1, a[4] as usize);
            0
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
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
unsafe fn desviar_bola_com(this: usize, x: i8, y: i8, espalhar: i8, mut lado: i8, dentro: u8, cansa: u8, chutador: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0970usize);
    let somar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x7160d0usize);
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
unsafe extern "thiscall" fn desviar_bola(this: usize, x: i8, y: i8, espalhar: i8, lado: i8, dentro: u8, cansa: u8, chutador: usize) {
    desviar_bola_com(this, x, y, espalhar, lado, dentro, cansa, chutador, &mut |rotina, alvo, a| match rotina {
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(0x6cd490usize);
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
static mut PORTA_70A910: usize = 0;
static mut PORTA_70C040: usize = 0;
const PROLOGO_70A910: Prologo = Prologo { endereco: 0x70a910, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] };
const PROLOGO_70C040: Prologo = Prologo { endereco: 0x70c040, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] };

unsafe extern "thiscall" fn porta_70a910(p: usize, f: f32) {
    let o: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(PORTA_70A910);
    chamar_gravando(0x70a910, p, [f.to_bits(), 0, 0, 0, 0], || {
        o(p, f);
        0
    });
}
unsafe extern "thiscall" fn porta_70c040(p: usize, f: f32) {
    let o: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(PORTA_70C040);
    chamar_gravando(0x70c040, p, [f.to_bits(), 0, 0, 0, 0], || {
        o(p, f);
        0
    });
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
unsafe fn lancar_com(j: usize, modo: i8, refinar: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let impedido_de: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(0x6cfd70usize);
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(0x6f09b0usize);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(0x6bb9a0usize);
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
unsafe extern "thiscall" fn lancar(j: usize, modo: i8, refinar: u8) {
    lancar_com(j, modo, refinar, &mut |rotina, this, a| match rotina {
        0x6fad40 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u8, usize, u32, u32) -> u8 = core::mem::transmute(0x6fad40usize);
            let mut saida = 1u8;
            let r = f(this, a[0] as i8, a[1] as i8, a[2], &mut saida, 0, a[3], a[4]);
            r as u32 | ((saida as u32) << 8)
        }
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(0x6cd490usize);
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
unsafe fn passar_com(j: usize, tipo: u8, receptor: usize, a4: u32, a5: u32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let impedido_de: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(0x6cfd70usize);
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(0x6f09b0usize);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(0x6bb9a0usize);
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
unsafe extern "thiscall" fn passar(j: usize, tipo: u8, receptor: usize, mut a3: u32, a4: u32, a5: u32) {
    passar_com(j, tipo, receptor, a4, a5, &mut |rotina, this, a| match rotina {
        0x6fad40 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32, usize, u32, u32) -> u8 = core::mem::transmute(0x6fad40usize);
            let r = f(this, a[0] as i8, a[1] as i8, a[2], &mut a3, a[3] as usize, a[4], a5);
            r as u32 | ((a3 as u8 as u32) << 8)
        }
        0x7103e0 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7103e0usize);
            f(this);
            0
        }
        _ => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

// Porta de 0x6fad40 (executa o passe, 13,8 KB): grava (x, y, tipo, receptor, 4o) e devolve a
// resposta com o byte que ela escreve na saida (5o argumento) no byte seguinte.
static mut PORTA_6FAD40: usize = 0;
const PROLOGO_6FAD40: Prologo = Prologo { endereco: 0x6fad40, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] };

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn porta_6fad40(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, a6: u32, a7: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u8, usize, u32, u32) -> u8 = core::mem::transmute(PORTA_6FAD40);
    let mut r = 0u8;
    chamar_gravando(0x6fad40, p, [x, y, tipo, receptor as u32, a6], || {
        r = f(p, x, y, tipo, saida, receptor, a6, a7);
        r as u32 | ((*saida as u32) << 8)
    });
    r
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
unsafe fn decidir_jogador_com(j: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let pode: unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u32 = core::mem::transmute(0x6bce20usize);
    let faixa: unsafe extern "stdcall" fn(u8, i8, i8) -> u8 = core::mem::transmute(0x6cb960usize);
    let dist_ponto: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(0x6f09b0usize);
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

/// { 0x7103e0 } (match_pl.cpp): ver `decidir_jogador_com`.
unsafe extern "thiscall" fn decidir_jogador(j: usize) {
    decidir_jogador_com(j, &mut |rotina, this, _a| match rotina {
        0x713fc0 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x713fc0usize);
            f(this, 1);
            0
        }
        0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x714d40usize);
            f(this);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7109f0usize);
            f(this, 0)
        }
    })
}

// Porta de 0x7109f0 (o jogador com a bola, 13 KB): grava na fita aberta, como as outras.
static mut PORTA_7109F0: usize = 0;
const PROLOGO_7109F0: Prologo = Prologo { endereco: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] };

unsafe extern "thiscall" fn porta_7109f0(p: usize, a1: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(PORTA_7109F0);
    chamar_gravando(0x7109f0, p, [a1, 0, 0, 0, 0], || f(p, a1))
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
unsafe fn mover_sem_bola_com(j: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((j + 0x19e) as *const usize);
    let dir = byte(j, 0x27);
    let bloco = (m as isize + 0x91e2 + 6371 * dir as isize) as usize;
    let w = |o: usize| (j + o) as *mut u16;
    let u = |a: usize| *(a as *const usize);
    let b = |o: usize| byte(j, o);
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(0x5b5270usize);
    let formacao: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x6c2a40usize);
    let do_lado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cfda0usize);
    let posicionar: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) = core::mem::transmute(0x6c2be0usize);
    let nota: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(0x715f50usize);
    let dif_lado: unsafe extern "thiscall" fn(usize, u8, u8) -> u8 = core::mem::transmute(0x6c4180usize);
    let limite: unsafe extern "stdcall" fn(u8, i8) -> u8 = core::mem::transmute(0x6cb930usize);
    let dif_jogador: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6c4130usize);
    let atras: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cd5f0usize);
    let faixa: unsafe extern "stdcall" fn(u8, i8, i8) -> u8 = core::mem::transmute(0x6cb960usize);
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(0x6ce110usize);
    let menor_dist: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> f32 = core::mem::transmute(0x6c3950usize);
    let dist_entre: unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32 = core::mem::transmute(0x6cc4e0usize);
    let apontar: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(0x6f4920usize);
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
            let f1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
            let f2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
            let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
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

/// { 0x714d40 } (match_pl.cpp): ver `mover_sem_bola_com`.
unsafe extern "thiscall" fn mover_sem_bola(j: usize) {
    mover_sem_bola_com(j, &mut |rotina, this, a| match rotina {
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(0x6f46b0usize);
            f(this, a[0] as i8, a[1] as i8);
            0
        }
        0x6f5b80 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b80usize);
            f(this, 0)
        }
        0x6f0630 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0630usize);
            f(this);
            0
        }
        0x6d6f70 => {
            comentar(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(0x6f09f0usize);
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
unsafe fn decidir_sem_bola_com(j: usize, chamar: &mut dyn FnMut(usize, [u32; 5]) -> u32) {
    let m = *((j + 0x19e) as *const usize);
    let dir = byte(j, 0x27);
    let bloco = (m as isize + 0x91e2 + 6371 * dir as isize) as usize;
    let w = |o: usize| (j + o) as *mut u16;
    let byte_pos: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(0x5b5270usize);
    let formacao: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x6c2a40usize);
    let escolher: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u8 = core::mem::transmute(0x6bce80usize);
    let do_lado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cfda0usize);
    let posicionar: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) = core::mem::transmute(0x6c2be0usize);
    let nota: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(0x715f50usize);
    let dif_lado: unsafe extern "thiscall" fn(usize, u8, u8) -> u8 = core::mem::transmute(0x6c4180usize);
    let alem: unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8 = core::mem::transmute(0x6cb700usize);
    let limite: unsafe extern "stdcall" fn(u8, i8) -> u8 = core::mem::transmute(0x6cb930usize);
    let dif_jogador: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6c4130usize);
    let atras: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cd5f0usize);
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
unsafe extern "thiscall" fn decidir_sem_bola(j: usize, _a: u32) {
    decidir_sem_bola_com(j, &mut |rotina, a| match rotina {
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(0x6f46b0usize);
            f(j, a[0] as i8, a[1] as i8);
            0
        }
        0x6f5b80 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b80usize);
            f(j, 0)
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(0x6f09f0usize);
            f(j, a[0] as i8, a[1] as i8, a[2] as i32, 0, 0)
        }
    })
}

// ---------- nonagesima quarta leva: arrumar para uma bola parada (0x6c4820) ----------

/// Depois de mover para a casa nova: o jogador sem a bola passou da linha defensiva do lado dele
/// (+0x8eb0 menos y no lado 1, y menos +0x8eaf no outro, nao negativo)? Quem tem a bola (+0xf582)
/// conta como passou. Trecho repetido de 0x6c4ec0 e 0x6c4820.
unsafe fn passou_da_linha(motor: usize, j: usize) -> bool {
    if j == *((motor + 0xf582) as *const usize) {
        return true;
    }
    let y = byte(j, 0x103);
    let d = if lado(j) == 1 { byte(motor, 0x8eb0).wrapping_sub(y) } else { y.wrapping_sub(byte(motor, 0x8eaf)) };
    d >= 0
}

/// { 0x6c4820 } (match_eng.cpp): arruma os times para a bola parada do lado +0x8eae (contador
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
unsafe extern "thiscall" fn arrumar_bola_parada(this: usize) {
    let b = |o: usize| (this + o) as *mut i8;
    let u = |o: usize| *((this + o) as *const usize);
    let lado_b = byte(this, 0x8eae);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0970usize);
    let linhas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6c3aa0usize);
    let avancar: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b80usize);
    let gravar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x7160e0usize);
    let pegar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0630usize);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(0x6ceb90usize);
    let apontar: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(0x6f4920usize);
    let papel_ataque: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x716270usize);
    let papel_defesa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x716310usize);
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
                    let f1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
                    let f2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
                    let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
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

/// { 0x6c4ec0 } (match_eng.cpp): arruma os times para a saida de bola do lado +0x8eae. Quem sai
/// jogando: +0xf59e (lado 0, centro 4, 0) ou +0xf5a2 (lado 1, centro 4, 11; outro lado nao mexe no
/// centro). Solta a bola (0x6f0970) de quem a tem; cada jogador em campo (+0x19 >= 0) vai para a casa
/// da formacao (tabela +0xf56e do lado da direcao +0x27, byte +0xa2 da posicao: x no nibble baixo, y no
/// alto), as linhas defensivas sao refeitas (0x6c3aa0) e, no lado que sai jogando, quem nao tem a bola
/// e passou da linha (+0x8eb0 / +0x8eaf) volta uma linha (0x6f5b80 direto); +0x2b zera (0x7160e0). No
/// fim, quem sai jogando vai ao centro (+0x8ea9 = 0) e pega a bola (0x6f0630), com +0x19c = 200 e
/// +0x19a = 180 se o centro nao for a linha 0. Sem ele: a original (caixa de erro).
unsafe extern "thiscall" fn arrumar_saida(this: usize) {
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
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0970usize);
    let linhas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6c3aa0usize);
    let avancar: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b80usize);
    let gravar_2b: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x7160e0usize);
    let pegar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0630usize);
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

// ---------- nonagesima segunda leva: avancar uma linha (0x6f5b80), com fita generica ----------

/// { 0x6f5b80 } (ret 4, match_pl.cpp) com as chamadas injetadas: o jogador anda uma linha para o
/// lado dele (+0x27 = 1 sobe ate 11, senao desce ate 0). Com `direto`, vai (0x6f46b0) para a coluna
/// atual na linha da bola (+0x8eb0 + 1 ou +0x8eaf - 1 do motor); senao reposiciona (0x6f09f0) na
/// nova linha com -1, 0, 0. Devolve o `eax` da chamada (os chamadores ignoram).
unsafe fn avancar_linha_com(this: usize, direto: u8, chamar: &mut dyn FnMut(usize, [u32; 5]) -> u32) -> u32 {
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
unsafe extern "thiscall" fn avancar_linha(this: usize, direto: u8) -> u32 {
    avancar_linha_com(this, direto, &mut |rotina, a| {
        if rotina == 0x6f46b0 {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) -> u32 = core::mem::transmute(0x6f46b0usize);
            f(this, a[0] as i8, a[1] as i8)
        } else {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(0x6f09f0usize);
            f(this, a[0] as i8, a[1] as i8, -1, 0, 0)
        }
    })
}

// ---------- nonagesima primeira leva: interceptacao no caminho do gol (0x6cc540) ----------

/// { 0x6cc540 } (ret 0x18, match_eng.cpp) com a procura (0x6cd3d0 em modo 1) injetada: alguem
/// intercepta no caminho de (x, y) ate o centro do gol (x 4; y 11 com `gol` = 1, senao 0)? As casas
/// saem ao estilo de Bresenham, da de saida ate a do gol; em cada uma sorteia (2) o lado que tenta
/// primeiro e procura nos dois (distancia da tabela desde a saida, ou 1000). Volta o primeiro codigo
/// diferente de zero; 0 ao sair da grade ou ao chegar. +0xf5ae do motor zera no inicio.
#[allow(clippy::too_many_arguments)]
unsafe fn interceptar_com(this: usize, x: i8, y: i8, a3: u32, gol: i8, a5: u32, a6: u32, procurar: &mut dyn FnMut(i8, usize, u32, u32, u32, u32) -> u8) -> u8 {
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
unsafe extern "thiscall" fn interceptar(this: usize, x: i8, y: i8, a3: u32, gol: i8, a5: u32, a6: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u8, u32, u32) -> u8 = core::mem::transmute(0x6cd3d0usize);
    interceptar_com(this, x, y, a3, gol, a5, a6, &mut |lado, grupo, v, a3, a5, a6| f(this, lado, grupo, v, a3, 1, a5, a6))
}

// ---------- nonagesima leva: quem tenta a jogada (0x6cd3d0), com gravacao da 0x70dfd0 ----------

/// { 0x6cd3d0 } (ret 0x1c, match_eng.cpp) com a jogada (0x70dfd0) injetada: percorre a lista do
/// lado no grupo (+0x58 jogadores) atras de quem pode agir (+0x2b ate 1 e +0x19 nao negativo). Com
/// modo 2 so conta. Senao cada um tenta a jogada (argumentos 4, 3, 6 e 7): 1 = conseguiu (+0x12
/// sobe), 2 = outra saida; nos dois casos o jogador fica em +0xf5ae do motor e o codigo volta.
#[allow(clippy::too_many_arguments)]
unsafe fn procurar_jogada_com(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, modo: u8, a6: u32, a7: u32, jogada: &mut dyn FnMut(usize, [u32; 4]) -> u8) -> u8 {
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
unsafe extern "thiscall" fn procurar_jogada(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, modo: u8, a6: u32, a7: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(0x70dfd0usize);
    procurar_jogada_com(this, lado, grupo, a3, a4, modo, a6, a7, &mut |p, a| f(p, a[0], a[1], a[2], a[3]))
}

// Fita de chamadas com efeitos (0x70dfd0 a jogada, 0x6f09f0 o reposicionamento, 0x6f46b0 o
// movimento). Na comparacao de uma rotina portada que as chama, cada chamada da original e gravada
// com a resposta e as regioes pedidas pelo comparador como ficaram depois; a copia Rust confere a
// sequencia e reescreve essas regioes no lugar de chamar de novo.
type Chamada = (usize, usize, [u32; 5], u32, crate::comparacao::Memoria, crate::comparacao::Memoria, (Vec<u8>, Vec<u8>)); // (rotina, this, argumentos, resposta, antes, depois, (bytes dos vetores de entrada, de saida))
#[allow(clippy::type_complexity)]
static FITA: std::sync::Mutex<Option<(Vec<(usize, usize)>, Vec<Chamada>)>> = std::sync::Mutex::new(None);

fn fita() -> std::sync::MutexGuard<'static, Option<(Vec<(usize, usize)>, Vec<Chamada>)>> {
    FITA.lock().unwrap_or_else(|e| e.into_inner())
}

/// Ha uma gravacao aberta (uma comparacao de fora ja grava estas chamadas)?
fn gravando() -> bool {
    fita().is_some()
}

/// Gravacao aberta e no nivel de cima dela (fora de qualquer unidade gravada em andamento): a
/// rotina e chamada direto pela que esta sendo comparada e entra na fita (ou so repassa). Dentro de
/// uma unidade, ela pode ser comparada por conta propria (a fita de fora fica guardada).
fn gravando_no_topo() -> bool {
    gravando() && DENTRO.load(std::sync::atomic::Ordering::Relaxed) == 0
}

/// Quantas chamadas gravaveis estao em andamento: so as de fora (profundidade 0) entram na fita;
/// as que uma delas faz por dentro (0x70dfd0 e 0x6f09f0 movem jogadores) ja estao na foto.
static DENTRO: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Roda uma chamada gravavel e guarda-a na fita aberta, com a foto das regioes depois dela.
unsafe fn chamar_gravando(rotina: usize, this: usize, a: [u32; 5], chamada: impl FnOnce() -> u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let nivel = DENTRO.fetch_add(1, Relaxed);
    // o estado antes da chamada: a replica Rust tem de chegar a ele (o que a rotina gravou antes de
    // chamar, como a acao em +0x198, seria apagado pela foto de depois)
    let antes = if nivel == 0 { fita().as_ref().map(|(reg, _)| crate::comparacao::Memoria::ler(reg)) } else { None };
    let r = chamada();
    DENTRO.fetch_sub(1, Relaxed);
    if let Some(antes) = antes {
        gravar_chamada(rotina, this, a, r, antes, (Vec::new(), Vec::new()));
    }
    r
}

unsafe fn gravar_chamada(rotina: usize, this: usize, a: [u32; 5], r: u32, antes: crate::comparacao::Memoria, vetores: (Vec<u8>, Vec<u8>)) {
    if let Some((reg, f)) = fita().as_mut() {
        let depois = crate::comparacao::Memoria::ler(reg);
        f.push((rotina, this, a, r, antes, depois, vetores));
    }
}

/// Os bytes dos vetores (endereco, tamanho), em sequencia.
unsafe fn ler_vetores(v: &[(usize, usize)]) -> Vec<u8> {
    let mut b = Vec::new();
    for &(a, n) in v {
        if a != 0 {
            b.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
        }
    }
    b
}

unsafe fn escrever_vetores(v: &[(usize, usize)], bytes: &[u8]) {
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
unsafe fn chamar_gravando_vetores(rotina: usize, this: usize, a: [u32; 5], entradas: &[(usize, usize)], saidas: &[(usize, usize)], chamada: impl FnOnce() -> u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let nivel = DENTRO.fetch_add(1, Relaxed);
    let antes = if nivel == 0 { fita().as_ref().map(|(reg, _)| crate::comparacao::Memoria::ler(reg)) } else { None };
    let ent = if antes.is_some() { ler_vetores(entradas) } else { Vec::new() };
    let r = chamada();
    DENTRO.fetch_sub(1, Relaxed);
    if let Some(antes) = antes {
        let sai = ler_vetores(saidas);
        gravar_chamada(rotina, this, a, r, antes, (ent, sai));
    }
    r
}

/// Os vetores da proxima unidade pedida pela copia Rust: (entradas, saidas).
#[allow(clippy::type_complexity)]
static VETORES_PEDIDOS: std::sync::Mutex<Option<(Vec<(usize, usize)>, Vec<(usize, usize)>)>> = std::sync::Mutex::new(None);

fn pedir_vetores(entradas: &[(usize, usize)], saidas: &[(usize, usize)]) {
    *VETORES_PEDIDOS.lock().unwrap_or_else(|e| e.into_inner()) = Some((entradas.to_vec(), saidas.to_vec()));
}

/// Roda a original gravando as chamadas e a copia Rust reproduzindo-as (`reproduzir`). Devolve as
/// duas respostas, se as regioes ficaram iguais e se a sequencia de chamadas bateu.
/// O que divergiu na ultima comparacao com fita (chamada gravada x pedida, regioes diferentes),
/// para a mensagem de divergencia. So e montado quando ha diferenca.
static DETALHE_FITA: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn detalhe_fita() -> String {
    DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

unsafe fn comparar_com_fita<R>(reg: Vec<(usize, usize)>, original: impl FnOnce() -> R, rust: impl FnOnce(&mut dyn FnMut(usize, usize, [u32; 5], [u32; 5]) -> u32) -> R) -> (R, R, bool, bool, usize) {
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
        DETALHE_FITA.lock().unwrap_or_else(|e| e.into_inner()).push_str(&format!(" | sorteios original {:?} rust {:?}", &ro[..ro.len().min(12)], &rr[..rr.len().min(12)]));
    }
    DENTRO.store(nivel, std::sync::atomic::Ordering::Relaxed);
    *fita() = externa;
    let k = gravadas.borrow().len();
    (r, n, igual, sequencia.get(), k)
}

// Porta de 0x70dfd0 (a jogada, 7 KB: narra, sorteia e mexe no motor e no jogador).
static mut PORTA_70DFD0: usize = 0;
const PROLOGO_70DFD0: Prologo = Prologo { endereco: 0x70dfd0, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] };

unsafe extern "thiscall" fn porta_70dfd0(p: usize, a1: u32, a2: u32, a3: u32, a4: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(PORTA_70DFD0);
    chamar_gravando(0x70dfd0, p, [a1, a2, a3, a4, 0], || f(p, a1, a2, a3, a4) as u32) as u8
}

// Porta de 0x6f09f0 (reposiciona o jogador, 13 KB, sorteia e mexe em tudo em volta).
static mut PORTA_6F09F0: usize = 0;
const PROLOGO_6F09F0: Prologo = Prologo { endereco: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] };

unsafe extern "thiscall" fn porta_6f09f0(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(PORTA_6F09F0);
    chamar_gravando(0x6f09f0, p, [a1, a2, a3, a4, a5], || f(p, a1, a2, a3, a4, a5))
}

/// 0x70dfd0 recebe os quatro argumentos inteiros.
const MASCARA_70DFD0: [u32; 5] = [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0];
/// 0x6f09f0 e 0x6f46b0 leem x e y como bytes (o resto do registro empilhado e lixo).
const MASCARA_XY: [u32; 5] = [0xff, 0xff, u32::MAX, u32::MAX, u32::MAX];

// ---------- octogesima nona leva: decisao pela classificacao do grupo, narracao da classificacao ----------

/// { 0x6854a0 } (ret 8, league.cpp): a fase (this) com a partida (`partida`, +0x42 = 0xff) ja decide
/// o clube? So nas competicoes que contam (0x4b0700), com o bit 1 de +0xd9 e a rodada +0xa9 >= 24:
/// a linha do clube (+0xb1, 0x3b bytes, +0x3e linhas) sem situacao +0x37 de 5, 2, 3, -2, -4 ou -5;
/// numa copia da tabela (0x687710) preenchida (0x685f60) e ordenada (0x6827d0), o clube tem de estar
/// na frente (+4 zero) e nenhum outro alcanca-lo com os jogos que faltam (+0xa7 - +0x11 vezes os
/// pontos por vitoria +0xc2, contra os pontos +0x19). A copia volta (0x687970).
unsafe extern "thiscall" fn grupo_decidido(this: usize, clube: usize, partida: usize) -> u32 {
    if *((partida + 0x42) as *const u8) != 0xff {
        return 0;
    }
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(*(*((this + 4) as *const usize) as *const i32)) == 0 {
        return 0;
    }
    if *((this + 0xd9) as *const u8) & 1 == 0 || *((this + 0xa9) as *const i16) < 24 {
        return 0;
    }
    let linhas = *((this + 0x3e) as *const i16);
    let tabela = *((this + 0xb1) as *const usize);
    let linha = |base: usize, k: i16| base.wrapping_add(k as usize * 0x3b);
    let Some(k) = (0..linhas).find(|&k| *(linha(tabela, k) as *const usize) == clube) else {
        return 0;
    };
    let r = linha(tabela, k);
    if r == 0 || matches!(*((r + 0x37) as *const i8), 5 | 2 | 3 | -2 | -4 | -5) {
        return 0;
    }
    let copia: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x687710usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32) = core::mem::transmute(0x685f60usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6827d0usize);
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let t = copia(this);
    if t == 0 {
        return 0;
    }
    preencher(this, partida, t, *((this + 0x3e) as *const u16) as i32, 0, 0, 0);
    ordenar(this, t);
    let n = *((this + 0x3e) as *const i16);
    // a original le +4 do endereco 0 se o clube nao estiver na copia (nunca acontece no jogo)
    let achada = (0..n).find(|&k| *(linha(t, k) as *const usize) == clube).map_or(0, |k| linha(t, k));
    if *((achada + 4) as *const u8) != 0 {
        soltar(this, t);
        return 0;
    }
    let rodadas = *((this + 0xa7) as *const i16);
    let pontos = *((this + 0xc2) as *const i8) as i16;
    for k in 1..*((this + 0x3e) as *const i16) {
        let o = linha(t, k);
        let jogos = *((o + 0x11) as *const i8) as i16;
        let alcance = rodadas.wrapping_sub(jogos).wrapping_mul(pontos).wrapping_add(*((o + 0x19) as *const i16));
        if alcance >= *((t + 0x19) as *const i16) && (*((t + 0x11) as *const i8) as i16 != rodadas || jogos != rodadas) {
            soltar(this, t);
            return 0;
        }
    }
    soltar(this, t);
    1
}

/// { 0x6be120 } (ret 4, match_eng.cpp) com o comentario injetado: narra a situacao da
/// classificacao do lado. Grupo decidido (0x6854a0) ou mata-mata ja resolvido (0x6ce310 na rodada
/// 0x82 ou 0x96): 0x2171/0x2170. Com +0x8ebb: 0x2120 se o lado esta na frente do agregado, senao
/// 0x2121 quando a media do +0x1b dos titulares e baixa e a forca (+0x80) do outro clube pesa. No fim,
/// para o lado 1 sem +0x1a8: 0x2001 se o jogo ainda nao estiver decidido.
unsafe fn narrar_classificacao_com(this: usize, lado: u32, chamar: ChamarComentario) {
    let t = || *((this + 0x4792) as *const usize);
    let w = |o: usize| *((t() + o) as *const i16);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let conta_t = || conta(*(t() as *const i32));
    let objeto = || *((*(0xadadfc as *const usize)).wrapping_add((*(t() as *const i32) as usize).wrapping_mul(4)) as *const usize);
    let decidido: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x6854a0usize);
    let agregado: unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> i8 = core::mem::transmute(0x6ce310usize);
    let narrador = || *((this + 0x8f7a) as *const usize);
    let fala = |c: &mut dyn FnMut(usize, [u32; 7]), cod: u32, l: u32| c(narrador(), [cod, this as u32, u32::MAX, l, u32::MAX, u32::MAX, 0]);
    // o placar agregado com a 5a saida (fase3): na original ela cai no proprio argumento
    let resolvido = |l: i8| -> bool {
        let (mut a, mut b, mut c, mut d, mut e) = (0i8, 0i8, 0u8, 0u8, 0u8);
        agregado(this, l, &mut a, &mut b, &mut c, &mut d, &mut e, 0) > 0 && e == 0
    };
    let bl = lado as u8 as i8;
    if w(0x32) == 0 && conta_t() == 0 && w(0x30) == 0x82 && resolvido(bl) {
        fala(chamar, 0x2171, lado);
        return;
    }
    if conta_t() != 0 {
        let clube = *((t() + if bl != 0 { 0x20 } else { 0x1c }) as *const usize);
        if decidido(objeto(), clube, t()) != 0 {
            fala(chamar, 0x2170, lado);
            return;
        }
    } else if w(0x30) == 0x96 && resolvido(bl) {
        fala(chamar, 0x2170, lado);
        return;
    }
    if byte(this, 0x8ebb) != 0 {
        let (mut casa, mut fora, mut classificado, mut x, mut y) = (0i8, 0i8, 0u8, 0u8, 0u8);
        agregado(this, bl, &mut casa, &mut fora, &mut classificado, &mut x, &mut y, 0);
        if casa > fora || (casa == fora && classificado != 0) {
            fala(chamar, 0x2120, lado);
        } else {
            let mut soma = 0.0f64;
            let mut n = 0i8;
            for k in 0..20usize {
                let j = jogador_do_bloco(this, bl, k);
                if k <= 10 || *((j + 0x1f) as *const u8) != 0xff {
                    soma += byte(j, 0x1b) as f64;
                    n = n.wrapping_add(1);
                }
            }
            // sem nenhum titular a media seria o que estivesse no argumento (nao acontece)
            let media = if n != 0 { (soma / n as f64) as f32 as f64 } else { 0.0 };
            let forca = |o: usize| *((*((t() + o) as *const usize) + 0x80) as *const i16) as f64;
            let x = media * f64_em(0x968968) - f64_em(0x968950);
            if bl == 0 {
                if media < f64_em(0x968988) && x * forca(0x20) < forca(0x1c) {
                    fala(chamar, 0x2121, 0);
                }
                return;
            }
            if media < f64_em(0x96d2e8) && x * forca(0x1c) < forca(0x20) * f64_em(0x9689b0) {
                fala(chamar, 0x2121, lado);
            }
        }
    }
    if bl != 1 || byte(this, 0x1a8) != 0 {
        return;
    }
    if w(0x32) == 0 && conta_t() == 0 && w(0x30) == 0x82 && resolvido(0) {
        return;
    }
    if conta_t() != 0 {
        if decidido(objeto(), *((t() + 0x1c) as *const usize), t()) != 0 {
            return;
        }
    } else if w(0x30) == 0x96 && resolvido(0) {
        return;
    }
    fala(chamar, 0x2001, 0);
}

/// { 0x6be120 } (ret 4, match_eng.cpp): ver `narrar_classificacao_com`.
unsafe extern "thiscall" fn narrar_classificacao(this: usize, lado: u32) {
    narrar_classificacao_com(this, lado, &mut |t, a| comentar(t, a))
}

// ---------- octogesima oitava leva: lado da bola, momento do jogador, partida de interesse ----------

/// { 0x6f55c0 } (match_pl.cpp): sorteia o momento (0x6f44a0); 1 em 11 vezes ainda: o byte +0xff
/// cai 1 se passar de (+0x100 / 2 + 10 + ficha +0x25) / 4; +0x4d soma +0x59 (que zera se era 1 e
/// senao cai a metade); acima de 50 em +1, tira (+1 - 50)^1,25 / 4 (no maximo 25); +0x4d fica em
/// 1..10000.
unsafe extern "thiscall" fn evoluir_momento(this: usize) {
    let momento: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f44a0usize);
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
unsafe extern "C" fn partida_de_interesse(p: usize) -> u32 {
    let g = |a: usize| *(a as *const i32);
    let comp = || *((p + 0x14) as *const usize);
    let id = *(comp() as *const i32);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x48db80usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let rodada: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x4b0870usize);
    let do_grupo: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7e6f20usize);
    let objeto = || *((*(0xadadfc as *const usize)).wrapping_add((*(comp() as *const i32) as usize).wrapping_mul(4)) as *const usize);
    let tecnico = |s: i32| (*(0xae23bc as *const usize)).wrapping_add(((g(0xae2364) - s - 1) as usize).wrapping_mul(110));
    let u = |a: usize| *(a as *const usize);
    if id == g(0x9cf668) || id == g(0x9cf684) {
        let sel = |o: usize| e_selecao(u(p + o)) != 0 && *((u(u(p + o) + 0x53) + 0x11c) as *const u8) != 0;
        return (sel(0x1c) || sel(0x20)) as u32;
    }
    if id == g(0x9cf794) || id == g(0x9cf798) {
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
    if id == g(0x9cf6f8) && rodada(*((p + 0x30) as *const u16) as i32) as i16 != 150 {
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
        let k = if id == g(0x9cf77c) || id == g(0x9cf7ac) {
            Some(g(0x9cfa08))
        } else if id == g(0x9cf774) {
            Some(g(0x9cfa0c))
        } else if id == g(0x9cf770) {
            Some(g(0x9cfa14))
        } else if id == g(0x9cf780) || id == g(0x9cf7a4) || id == g(0x9cf784) {
            Some(g(0x9cfa10))
        } else if id == g(0x9cf76c) {
            Some(g(0x9cfa18))
        } else if id == g(0x9cf778) || id == g(0x9cf7a8) {
            Some(g(0x9cfa1c))
        } else {
            None
        };
        if let Some(k) = k {
            edi = (*(0xae23a4 as *const usize)).wrapping_add((k as usize).wrapping_mul(0xc6));
        }
        if edi == 0 && id != g(0x9cf7a0) {
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

// ---------- octogesima setima leva: forca do lado (0x6bbb60) ----------

/// { 0x6bbb60 } (ret 4, match_eng.cpp): a forca do lado (palavra +0x1ce + 2 x lado do motor,
/// recomeca do zero, assim como o byte +0xaf do lado). Para cada um dos 20 ativos: -500 quando o bit
/// da casa (0x717410, 0x5b5a60) ja apareceu (a mascara soma, nao junta); com o contrato (0xae1050:
/// 0x4cf5a0, guardado em 0xae1080; 0x4cf5f0 com o clube dele) e tres sorteios favoraveis (20 contra
/// +0x2f da ficha, 7000 contra a palavra +0xb, 6 contra +0x5b da pessoa), a palavra +0xb acima de
/// +0x80 do clube - 750 e os bits 0x17a0b do contrato, perde 3x a nota (0x6ebd70) se for o designado
/// da tatica (0x897590), senao 2x ou 1x (sorteio pelo +0x5b). Quem nao e o designado soma a nota. No
/// fim, o desempenho de cada ativo (0x6eecb0 com marca).
unsafe extern "thiscall" fn forca_do_lado(this: usize, lado: i8) {
    let l = lado as isize;
    let forca = (this as isize + 0x1ce + 2 * l) as *mut i16;
    let soma = move |v: i32| *forca = (*forca).wrapping_add(v as i16);
    *forca = 0;
    *((this as isize + 0xaf + l) as *mut u8) = 0;
    let casa: unsafe extern "thiscall" fn(usize) -> u16 = core::mem::transmute(0x717410usize);
    let bit: unsafe extern "stdcall" fn(u32) -> u32 = core::mem::transmute(0x5b5a60usize);
    let contrato_principal: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let nota: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6ebd70usize);
    let nota = |j: usize| nota(j) as i8 as i32;
    let vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(0x897590usize);
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
    let desempenho: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x6eecb0usize);
    for k in 0..20usize {
        let j = jogador_do_bloco(this, lado, k);
        if byte(j, 0x19) >= 0 {
            desempenho(j, 1);
        }
    }
}

// ---------- octogesima sexta leva: desempenho do jogador na partida (0x6eecb0, ~6,4 KB) ----------
/// { 0x6eecb0 } (ret 4, match_pl.cpp, ~6,4 KB): desempenho do jogador na partida. Atualiza os
/// atributos (0x6ec010), calcula a palavra +0x37 (posicao nas tabelas taticas A e B contra os
/// bytes de +0x10 a +0x1a da ficha; com `marcar`, conta no byte +0xaf do lado do motor), os bytes
/// +0x10e..+0x117 e as palavras derivadas de +0x11a a +0x196 (63 delas ajustadas por +0x37 numa
/// tabela de casos) e sorteia os atributos do momento (0x6f44a0).
unsafe extern "thiscall" fn desempenho_na_partida(this: usize, marcar: u8) {
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
    let atributos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6ec010usize);
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
            let marcas: unsafe extern "thiscall" fn(usize, i8) -> u64 = core::mem::transmute(0x5b5270usize);
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
    let momento: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f44a0usize);
    momento(this);
}

// ---------- octogesima quinta leva: atributos do jogador na partida (0x6ec010, ~11 KB) ----------
/// { 0x6ec010 } (match_pl.cpp, ~11 KB): atributos do jogador na partida (campos f32 de +0x7d a
/// +0xf5 e a palavra +0x3b), pela nota do designado, idade, contrato, moral, cansaco e taticas.
unsafe extern "thiscall" fn atualizar_atributos(this: usize) {
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
    let nota: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6ebd70usize);
    let nota = |j: usize| nota(j) as i8 as i32;
    let vaga: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8 = core::mem::transmute(0x897590usize);
    let designado = |m: usize| {
        let t = *((m + 0x4792) as *const usize);
        let l = byte(this, 0x27);
        let (c, fl) = if l != 0 { (*((t + 0x20) as *const usize), (byte(t, 0x41) == 0) as u32) } else { (*((t + 0x1c) as *const usize), (byte(t, 0x40) == 0) as u32) };
        let k = vaga(*(0xde1d08 as *const usize), c, fl, 0) as i32;
        (m as isize + 0x4796 + 0x1be * (k + 20 * l as i32) as isize) as usize
    };
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nivel: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53ce90usize);

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
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        let ler = |o: usize| core::ptr::read_unaligned((pj + o) as *const u32);
        if dias(0xae2c90, ler(0x3e), ler(0x42)) < (50 - b(pj, 0x59)) * 10 {
            l18 = m(l18, 0x96a32c);
        }
        if b(pj, 0x5b) < 10 {
            l18 = ((b(pj, 0x5b) as f64 * f(0x96bac8) + f(0x96a32c)) * l18 as f64) as f32;
        }
    }
    let l2c = if e_selecao(clube_lado()) == 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
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
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let jx = |l: i32| *((motor() as isize + 0x904d + 208 * l as isize) as *const usize);
    let do_banco = || {
        // o tecnico do lado (+0x904d) e a pessoa dele, quando o clube nao e humano
        if humano(clube_lado(), 0, 1, 0, 0) != 0 {
            return 0;
        }
        let x = jx(byte(this, 0x27) as i32);
        if x == 0 { 0 } else { *((x + 0x69) as *const usize) }
    };
    let t5b20: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(0x6d5b20usize);
    let t5af0: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(0x6d5af0usize);
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
        if t14 != 0 && *(t14 as *const u32) == *(0x9cf590 as *const u32) {
            let pn = *((pessoa() + 0x1a) as *const usize);
            if pn != 0 {
                let id = *(pn as *const u32);
                let g = |a: usize| *(a as *const u32);
                // a original so poupa a nacao igual as tres globais ao mesmo tempo
                if !(id == g(0x9cf4f8) && id == g(0x9cf280) && id == g(0x9cf3d0)) {
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
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
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

// ---------- octogesima quarta leva: competicao da pessoa, bytes da selecao, selecoes especiais, data com marca ----------

/// { 0x7751e0 } (ret 4, national_teams.cpp): a competicao (0xae23d0, 107 bytes) do indice +4 do
/// registro da pessoa em [this+4] (110 bytes); sem pessoa ou indice negativo, 0.
unsafe extern "thiscall" fn competicao_da_pessoa(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let reg = (*((this + 4) as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110));
    let i = *((reg + 4) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(0xae23d0 as *const usize)).wrapping_add((i as usize).wrapping_mul(107))
}

/// O byte `o` do bloco da classe (0x76d780: 1 com `flag`, 2 para a segunda selecao, senao 0) da
/// nacao do clube (+0x53) nos registros de [this+4]. A original chama 0x5409b0 duas vezes.
unsafe fn byte_da_selecao(this: usize, clube: usize, flag: u32, o: usize) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let classe: usize = if segunda(clube) == 0 {
        if flag == 0 { 0 } else { 1 }
    } else if flag == 0 {
        2
    } else {
        1
    };
    segunda(clube);
    let nacao = *(*((clube + 0x53) as *const usize) as *const i32);
    *((*((this + 4) as *const usize)).wrapping_add((nacao as usize).wrapping_mul(110)).wrapping_add(classe * 20 + o) as *const u8)
}

/// { 0x7754c0 } (ret 8): o byte +0x42 do bloco da selecao do clube.
unsafe extern "thiscall" fn byte_42_da_selecao(this: usize, clube: usize, flag: u32) -> u8 {
    byte_da_selecao(this, clube, flag, 0x42)
}

/// { 0x775520 } (ret 8): o byte +0x43 do bloco da selecao do clube.
unsafe extern "thiscall" fn byte_43_da_selecao(this: usize, clube: usize, flag: u32) -> u8 {
    byte_da_selecao(this, clube, flag, 0x43)
}

/// { 0x775790 } (stdcall): selecao (0x53b900) ou um dos clubes de 0x9d00a4 e 0x9d00a8?
unsafe extern "stdcall" fn selecao_ou_especial(clube: usize) -> u32 {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if e_selecao(clube) != 0 {
        return 1;
    }
    let id = *(clube as *const u32);
    (id == *(0x9d00a4 as *const u32) || id == *(0x9d00a8 as *const u32)) as u32
}

/// { 0x776380 } (stdcall): a pessoa nao gosta (0x547d30) da selecao principal (0x53b3d0) da nacao +0x1a?
unsafe extern "stdcall" fn nao_gosta_da_selecao(pessoa: usize) -> u32 {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let s = principal(*((pessoa + 0x1a) as *const usize));
    (nao_gosta(pessoa, s) != 0) as u32
}

/// { 0x777590 } (ret 4): copia a data (0x417c10) e o byte +8 de `origem`; devolve `this`.
unsafe extern "thiscall" fn copiar_data_e_marca(this: usize, origem: usize) -> usize {
    let mut tmp = [0u8; 8];
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    copiar(this, tmp.as_mut_ptr() as usize, origem);
    *((this + 8) as *mut u8) = *((origem + 8) as *const u8);
    this
}

// ---------- octogesima terceira leva: mais campos por pessoa (national_teams.cpp), global 0x9b5984 ----------

/// O bloco `k` da pessoa nos registros de [this+4]: id x 110 + k x 20.
unsafe fn bloco_da_pessoa(this: usize, pessoa: usize, k: i8) -> usize {
    (*((this + 4) as *const usize))
        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
}

/// { 0x76c0f0 } (ret 8): liga (1) o byte +0x44 do bloco `k` da pessoa.
unsafe extern "thiscall" fn ligar_44_da_pessoa(this: usize, pessoa: usize, k: i8) {
    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *mut u8) = 1;
}

/// { 0x76e010 } (ret 8): o byte +0x44 do bloco `k` da pessoa, com sinal.
unsafe extern "thiscall" fn byte_44_da_pessoa(this: usize, pessoa: usize, k: i8) -> i32 {
    *((bloco_da_pessoa(this, pessoa, k) + 0x44) as *const i8) as i32
}

/// { 0x76e040 } (ret 8): o endereco do campo +0x3a do bloco `k` da pessoa.
unsafe extern "thiscall" fn campo_3a_da_pessoa(this: usize, pessoa: usize, k: i8) -> usize {
    bloco_da_pessoa(this, pessoa, k) + 0x3a
}

/// { 0x76e070 } (stdcall): sem selecao (+0x24), 0xff; segunda selecao (0x5409b0), 2; senao o bit 3
/// do byte +0x5e. Devolve so `al`.
unsafe extern "stdcall" fn classe_pela_selecao(pessoa: usize) -> u8 {
    let selecao = *((pessoa + 0x24) as *const usize);
    if selecao == 0 {
        return 0xff;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(selecao) != 0 {
        return 2;
    }
    (*((pessoa + 0x5e) as *const u8) >> 3) & 1
}

/// { 0x535920 } (Database.cpp): a global 0x9b5984.
unsafe extern "C" fn global_9b5984() -> u32 {
    *(0x9b5984 as *const u32)
}

// ---------- octogesima segunda leva: tipos de pessoa, selecao B, marcas por pessoa, registro livre ----------

/// { 0x53b610 } (Database.cpp): o tipo nao e 11?
unsafe extern "C" fn tipo_nao_11(tipo: i8) -> u32 {
    (tipo != 11) as u32
}

/// { 0x53b620 } (Database.cpp): o tipo e 11, 12, 13 ou 15?
unsafe extern "C" fn tipo_11_a_15(tipo: i8) -> u32 {
    matches!(tipo, 11 | 12 | 13 | 15) as u32
}

/// { 0x76d780 } (stdcall, national_teams.cpp): com `flag`, 1; sem ela, 2 para a segunda selecao
/// (0x5409b0) e 0 para as outras.
unsafe extern "stdcall" fn classe_da_selecao(clube: usize, flag: u32) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let e_segunda = segunda(clube) != 0;
    if flag != 0 {
        1
    } else if e_segunda {
        2
    } else {
        0
    }
}

/// O byte das marcas por pessoa: [this+4] + id x 110 + k x 20 + 0x45.
unsafe fn marca_da_pessoa(this: usize, pessoa: usize, k: i8) -> *mut u8 {
    (*((this + 4) as *const usize))
        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
        .wrapping_add(0x45) as *mut u8
}

/// { 0x76d7b0 } (ret 8, national_teams.cpp): a marca `k` da pessoa.
unsafe extern "thiscall" fn ler_marca_da_pessoa(this: usize, pessoa: usize, k: i8) -> u8 {
    *marca_da_pessoa(this, pessoa, k)
}

/// { 0x76d7e0 } (ret 8, national_teams.cpp): zera a marca `k` da pessoa.
unsafe extern "thiscall" fn zerar_marca_da_pessoa(this: usize, pessoa: usize, k: i8) {
    *marca_da_pessoa(this, pessoa, k) = 0;
}

/// { 0x53a5b0 } (Database.cpp): o registro livre (+0 zero) da tabela 0xae23f8 (60 bytes,
/// 0xae23a0 registros), guardado em 0x9b59b4 na primeira vez; sem nenhum, um sorteado.
unsafe extern "C" fn registro_livre() -> usize {
    let base = || *(0xae23f8 as *const usize);
    let mut i = *(0x9b59b4 as *const i32);
    if i == -1 {
        let n = *(0xae23a0 as *const i32);
        match (0..n.max(0)).find(|&k| *(base().wrapping_add(k as usize * 60) as *const u8) == 0) {
            Some(k) => {
                *(0x9b59b4 as *mut i32) = k;
                return base().wrapping_add(k as usize * 60);
            }
            None => i = sortear(n),
        }
    }
    base().wrapping_add((i as usize).wrapping_mul(60))
}

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

// ---------- septuagesima nona leva: nota do jogador pelo contrato, forma e idade ----------

/// { 0x6ebd70 } (match_pl.cpp): nota do jogador na partida, pelo clube do lado dele (motor +0x4792,
/// +0x1c/+0x20). n = nivel do contrato (0x53ce90, byte com sinal); forma = (palavra +0xd da ficha
/// +0x6d - palavra +0x80 do clube) x 0,002 + 1,5, limitada a -4..4; c = byte +0x44 do contrato
/// (0x4cf5f0) x (1/75) + 0,5, ajustado pelo tipo (+0x4f >> 4); x = byte +0x22 da pessoa x 75 na
/// selecao, senao os dias de vida (0x54cdb0); y = 1 + (x - 1000) x 0,000125 acima de 1000.
/// Resultado: byte +0x2f da ficha x y - (n^2 + 5) x 0,1 + forma + c + 0,5, truncado.
unsafe extern "thiscall" fn nota_do_jogador(this: usize) -> i32 {
    let clube = || {
        let t = *((*((this + 0x19e) as *const usize) + 0x4792) as *const usize);
        *((t + if byte(this, 0x27) == 0 { 0x1c } else { 0x20 }) as *const usize)
    };
    let pessoa = *((this + 0x69) as *const usize);
    let ficha = *((this + 0x6d) as *const usize);
    let nivel: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53ce90usize);
    let n = nivel(pessoa, clube()) as i8 as f64;
    let mut forma = (*((ficha + 0xd) as *const i16) as i32 - *((clube() + 0x80) as *const i16) as i32) as f64 * 0.002 + 1.5;
    if forma < -4.0 {
        forma = -4.0;
    } else if forma > 4.0 {
        forma = 4.0;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
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
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let x = if e_selecao(clube()) != 0 {
        byte(pessoa, 0x22) as u8 as f64 * 75.0
    } else {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        let ler = |o: usize| core::ptr::read_unaligned((pessoa + o) as *const u32);
        dias(0xae2c90, ler(0x3e), ler(0x42)) as f64
    };
    let y = if x > 1000.0 { (x - 1000.0) * 0.000125 + 1.0 } else { 1.0 };
    let e = *((ficha + 0x2f) as *const i8) as f64;
    ftol(e * y - (n * n + 5.0) * 0.1 + forma + c + 0.5)
}

// ---------- septuagesima oitava leva: competicao com rodada a disputar, comparacoes de ordenacao (comp_util.cpp) ----------

/// { 0x4b0700 } (comp_util.cpp): a competicao `id` conta? As tres de 0x9cf6b0, 0x9cf5d8 e 0x9cf548
/// sim; sem o +0x5d do registro (0xae23d0, 107 bytes) nao; senao so a do objeto em 0xadadfc com
/// fase 2 (+0x43), rodadas (+0x3e) e a primeira rodada (+0xb1) com o clube +4 em +0x57 ou +0x60.
unsafe extern "C" fn competicao_conta(id: i32) -> u32 {
    let g = |a: usize| *(a as *const i32);
    if id == g(0x9cf6b0) || id == g(0x9cf5d8) || id == g(0x9cf548) {
        return 1;
    }
    if *((*(0xae23d0 as *const usize)).wrapping_add((id as usize).wrapping_mul(107)).wrapping_add(0x5d) as *const u32) == 0 {
        return 0;
    }
    let c = *((*(0xadadfc as *const usize)).wrapping_add((id as usize).wrapping_mul(4)) as *const usize);
    if c == 0 || byte(c, 0x43) != 2 || *((c + 0x3e) as *const u16) == 0 {
        return 0;
    }
    let r = *((c + 0xb1) as *const usize);
    if r == 0 {
        return 0;
    }
    let primeira = *(r as *const usize);
    let alvo = *((c + 4) as *const i32);
    (*((primeira + 0x57) as *const i32) == alvo || *((primeira + 0x60) as *const i32) == alvo) as u32
}

/// { 0x4b0780 } (comparacao do qsort): byte +0x5f do segundo menos o do primeiro (nulo vale 0).
unsafe extern "C" fn ordem_por_5f(a: *const usize, b: *const usize) -> i32 {
    let v = |p: usize| if p == 0 { 0 } else { *((p + 0x5f) as *const i8) as i32 };
    v(*b) - v(*a)
}

/// { 0x4b07b0 } (comparacao do qsort): palavra +0x80 do segundo menos a do primeiro.
unsafe extern "C" fn ordem_por_80(a: *const usize, b: *const usize) -> i32 {
    *((*b + 0x80) as *const i16) as i32 - *((*a + 0x80) as *const i16) as i32
}

/// { 0x4b07d0 } (comparacao do qsort): como 0x4b07b0, com os nulos no fim (primeiro nulo: 1 se o
/// segundo nao for; segundo nulo: -1).
unsafe extern "C" fn ordem_por_80_desc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    *((y + 0x80) as *const i16) as i32 - *((x + 0x80) as *const i16) as i32
}

/// { 0x4b0810 } (comparacao do qsort): como 0x4b07d0, em ordem crescente.
unsafe extern "C" fn ordem_por_80_cresc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    *((x + 0x80) as *const i16) as i32 - *((y + 0x80) as *const i16) as i32
}

// ---------- septuagesima setima leva: proximo da lista, palavra +0xc, arredondamento, nacoes, marcadores ----------

/// { 0x68d1b0 }: o proximo item da lista (cursor em +0xa: devolve o +0 do no e avanca para o +6), ou 0.
unsafe extern "thiscall" fn proximo_da_lista(this: usize) -> u32 {
    let no = core::ptr::read_unaligned((this + 0xa) as *const usize);
    if no == 0 {
        return 0;
    }
    core::ptr::write_unaligned((this + 0xa) as *mut usize, core::ptr::read_unaligned((no + 6) as *const usize));
    core::ptr::read_unaligned(no as *const u32)
}

/// { 0x62c3d0 } (ret 4, injury.cpp): a palavra com sinal +0xc do registro de 31 bytes da pessoa `id`
/// em [this]; id a partir de 0xae2364: 0.
unsafe extern "thiscall" fn palavra_0c(this: usize, id: i32) -> i32 {
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    core::ptr::read_unaligned((*(this as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *const i16) as i32
}

/// { 0x4b0870 } (comp_util.cpp): `v` menos o resto por 10 da palavra baixa, quando ela (com sinal)
/// e menor que 1000.
unsafe extern "C" fn arredondar_abaixo_de_1000(v: i32) -> i32 {
    let w = v as i16;
    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })
}

/// { 0x7e6f20 } (stdcall): a nacao (id em +0) e uma das seis de 0x9cf794, 0x9cf684, 0x9cf798,
/// 0x9cf6b0, 0x9cf5d8 e 0x9cf548?
unsafe extern "stdcall" fn nacao_do_grupo(nacao: usize) -> u32 {
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const u32);
    [0x9cf794usize, 0x9cf684, 0x9cf798, 0x9cf6b0, 0x9cf5d8, 0x9cf548].iter().any(|&a| *(a as *const u32) == id) as u32
}

/// { 0x6d59b0 } (ret 4): quantos dos 20 jogadores do outro lado (ativos, +0x19 >= 0) marcam
/// `alvo` individualmente (0x7160f0).
unsafe extern "thiscall" fn quantos_marcam(this: usize, alvo: usize) -> u8 {
    let marcado: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7160f0usize);
    let outro = (lado(alvo) == 0) as i8;
    (0..20).filter(|&k| {
        let j = jogador_do_bloco(this, outro, k);
        *((j + 0x19) as *const i8) >= 0 && marcado(j) == alvo
    }).count() as u8
}

/// { 0x6d5a10 } (ret 4): o primeiro dos 20 jogadores do outro lado (ativos) que marca `alvo`, ou 0.
unsafe extern "thiscall" fn quem_marca(this: usize, alvo: usize) -> usize {
    let marcado: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7160f0usize);
    let outro = (lado(alvo) == 0) as i8;
    for k in 0..20 {
        let j = jogador_do_bloco(this, outro, k);
        if *((j + 0x19) as *const i8) >= 0 && marcado(j) == alvo {
            return j;
        }
    }
    0
}

// ---------- septuagesima sexta leva: lista de observacao, bandeiras do contrato ----------

/// { 0x849ce0 } (ret 0x10, shortlist_manager.cpp): a entrada da lista da pessoa (40 entradas de
/// 37 bytes no ponteiro em [this+0xdc] + id x 9) cujo +0xf (ou, com `tambem_13`, o +0x13) e o id
/// de `alvo`; com `nao_11` so serve a de tipo (+0) diferente de 11, sem ele so a de tipo 11. Nenhuma: 0.
unsafe extern "thiscall" fn entrada_da_lista(this: usize, pessoa: usize, alvo: usize, nao_11: u32, tambem_13: u32) -> usize {
    let lista = (*((this + 0xdc) as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9));
    let base = *(lista as *const usize);
    let id = *(alvo as *const u32);
    for k in 0..40usize {
        let e = base.wrapping_add(k * 37);
        let casa = *((e + 0xf) as *const u32) == id || (tambem_13 != 0 && *((e + 0x13) as *const u32) == id);
        if casa && (nao_11 != 0) == (*(e as *const u8) != 11) {
            return e;
        }
    }
    0
}

/// { 0x8676f0 } (staff_contracts.cpp): as bandeiras do contrato (+0x40).
unsafe extern "thiscall" fn bandeiras_do_contrato(this: usize) -> u32 {
    *((this + 0x40) as *const u32)
}

// ---------- septuagesima quinta leva: nevoa de guerra (o que o clube humano sabe dos jogadores) ----------

/// { 0x544410 }: x = (palavra +4 da pessoa + 2 x `nivel`) x 0,05 + 10; (x + 0,1 x^2) x (1/3) + 0,5,
/// limitado a 1..20 (NaN vira 1) e truncado.
unsafe extern "C" fn chance_de_conhecer(pessoa: usize, nivel: i8) -> i32 {
    let x = (*((pessoa + 4) as *const i16) as f64 + (nivel as f64 + nivel as f64)) * 0.05 + 10.0;
    let y = (x * x * 0.1 + x) * f64_em(0x968a80) + 0.5;
    ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y })
}

/// { 0x5b09d0 } (ret 8, fog_of_war.cpp): o que se sabe da pessoa (registros de 9 bytes em [this],
/// pelo id): grava a data de hoje (0xae2c90) em +1; sem observador (`fonte` +0x69) o nivel (+0)
/// vai a 3. Com ele e nivel abaixo de 3: c = 0x544410(observador, byte +0x17), sorteio(c) (byte);
/// c < 15 e sorteio 0: nivel + 1; senao nivel 0 e sorteio < 6: 2; senao 3.
unsafe extern "thiscall" fn conhecer_pessoa(this: usize, pessoa: usize, fonte: usize) {
    if pessoa == 0 || *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return;
    }
    let reg = (*(this as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9));
    let mut data = [0u8; 8];
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    copiar(reg + 1, data.as_mut_ptr() as usize, 0xae2c90);
    let obs = if fonte == 0 { 0 } else { *((fonte + 0x69) as *const usize) };
    let nivel = reg as *mut u8;
    if obs == 0 {
        *nivel = 3;
        return;
    }
    if *nivel == 3 {
        return;
    }
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let c = chance(obs, *((obs + 0x17) as *const i8)) as i8;
    let r = sortear(c as i32) as i8;
    *nivel = if c < 15 && r == 0 {
        (*nivel).wrapping_add(1)
    } else if *nivel == 0 && r < 6 {
        2
    } else {
        3
    };
}

/// { 0x5afe00 } (stdcall, fog_of_war.cpp): depois da partida, para cada lado (clube em +0x20 e
/// +0x1c) controlado por humano (0x600a50) com tecnico e quadro de conhecimento (0xae2a60, pelo
/// id do tecnico): as 20 vagas da escalacao (`dados` + lado x 0x625 + 0x190, 0x49 bytes cada) com
/// pessoa e +4 validos (reservas, da 11a em diante, so com +0x32 != 0xff) ficam conhecidas (0x5b09d0).
unsafe extern "stdcall" fn nevoa_apos_partida(partida: usize, dados: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5b09d0usize);
    let n = || *(0xae2364 as *const i32);
    for lado in 0..2usize {
        let clube = *((partida + if lado == 0 { 0x20 } else { 0x1c }) as *const usize);
        if humano(clube, 0, 1, 0, 0) == 0 {
            continue;
        }
        let q = *((clube + 0xcf) as *const usize);
        if q == 0 {
            continue;
        }
        let k = n().wrapping_sub(*(q as *const i32)) as usize;
        let obj = *((*(0xae2a60 as *const usize)).wrapping_add(k.wrapping_mul(4)).wrapping_sub(4) as *const usize);
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let e = dados + lado * 0x625 + 0x190 + vaga * 0x49;
            let i = *(e as *const i32);
            if i == -1 || *((e + 4) as *const i32) == -1 || (vaga > 10 && *((e + 0x32) as *const u8) == 0xff) {
                continue;
            }
            let p = (*(0xae23bc as *const usize)).wrapping_add((i as usize).wrapping_mul(0x6e));
            if p == 0 || *(p as *const i32) < n() {
                conhecer(obj, p, 0);
            }
        }
    }
}

// ---------- septuagesima quarta leva: contrato da pessoa com o clube ----------

/// { 0x4cf5f0 } (ret 8): o contrato (registros de 80 bytes em [this]) da pessoa com o clube. A
/// pessoa tem ate dois indices (tabela 0xae2d0c, 79 bytes por pessoa, +0 e +4); vale o registro
/// cujo +4 e o id do clube. Sem nenhum, tenta o clube parceiro (0x540a50) quando ele e o time A.
unsafe extern "thiscall" fn contrato_com_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    if pessoa == 0 || clube == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let e = (*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
    let i0 = *(e as *const i32);
    if i0 < 0 {
        return 0;
    }
    let base = *(this as *const usize);
    let r0 = base.wrapping_add((i0 as usize).wrapping_mul(80));
    if r0 == 0 {
        return 0;
    }
    let id_de = |q: usize| *((q + 4) as *const u32);
    let id_clube = *(clube as *const u32);
    if id_de(r0) == id_clube {
        return r0;
    }
    let i1 = *((e + 4) as *const i32);
    let mut r1 = 0usize;
    if i1 >= 0 {
        r1 = base.wrapping_add((i1 as usize).wrapping_mul(80));
        if r1 != 0 && id_de(r1) == id_clube {
            return r1;
        }
    }
    let mut alvo = clube;
    let mut eh_a = pessoa as u32; // o original reaproveita a vaga do argumento
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let pc = parceiro(clube, &mut eh_a, 1);
    if pc != 0 && eh_a == 0 {
        alvo = pc;
    }
    let id_alvo = *(alvo as *const u32);
    if id_de(r0) == id_alvo {
        return r0;
    }
    if r1 != 0 && id_de(r1) == id_alvo {
        return r1;
    }
    0
}

/// { 0x53ce90 }: (byte +0x4f do contrato da pessoa com o clube (0x4cf5f0 sobre [0xae1050]) & 0xf)
/// - 10, para clube que nao e selecao (id abaixo de clubes - 2 x nacoes) e pessoa valida. Senao o
/// byte baixo zerado (o resto do registro fica como estava: o clube, ou 0).
unsafe extern "C" fn nivel_do_contrato(pessoa: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let limite = (*(0xae235c as *const i32)).wrapping_sub(2 * *(0xae2350 as *const i32));
    if *(clube as *const i32) >= limite || *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return clube as u32 & !0xff;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let r = f(*(0xae1050 as *const usize), pessoa, clube);
    if r == 0 {
        return 0;
    }
    ((*((r + 0x4f) as *const u8) & 0xf) as u32).wrapping_sub(10)
}

// ---------- septuagesima terceira leva: clube parceiro (time B) ----------

/// Pares de globais com os ids dos clubes A e B (times B da Espanha e outros), na ordem dos testes
/// do original (0x540a50); extraidos da desmontagem.
const PARES_A_B: [(usize, usize); 82] = [
    (0x9d012c, 0x9d0130), (0x9d0120, 0x9d0134), (0x9d0138, 0x9d013c), (0x9d0140, 0x9d0144), (0x9d0148, 0x9d014c), (0x9d0150, 0x9d0154),
    (0x9d0158, 0x9d015c), (0x9d0160, 0x9d0164), (0x9d0168, 0x9d016c), (0x9d0170, 0x9d0174), (0x9d0178, 0x9d017c), (0x9d0180, 0x9d0184),
    (0x9d0188, 0x9d018c), (0x9d0190, 0x9d0194), (0x9d0198, 0x9d019c), (0x9d0128, 0x9d01a0), (0x9d0254, 0x9d0258), (0x9d02e4, 0x9d0328),
    (0x9d03ac, 0x9d03b0), (0x9d03b4, 0x9d03b8), (0x9d03bc, 0x9d03c0), (0x9d03c4, 0x9d03c8), (0x9d0124, 0x9d03d4), (0x9d03d8, 0x9d03dc),
    (0x9d03e0, 0x9d03e4), (0x9d03e8, 0x9d03ec), (0x9d03f0, 0x9d03f4), (0x9d03f8, 0x9d03fc), (0x9d0400, 0x9d0404), (0x9d0408, 0x9d040c),
    (0x9d0410, 0x9d0414), (0x9d0418, 0x9d041c), (0xb63cdc, 0xb63c9c), (0xb63cbc, 0xb63ca0), (0xb63ce8, 0xb63cac), (0xb63cd8, 0xb63cd0),
    (0xb63cb4, 0xb63cb0), (0xb63cd4, 0xb63ca8), (0xb63ce4, 0xb63cec), (0xb63cf0, 0xb63cf8), (0xb63cc0, 0xb63d00), (0xb63cc8, 0xb63ccc),
    (0xb63ce0, 0xb63cb8), (0xb63cc4, 0xb63cf4), (0xb63cfc, 0xb63ca4), (0x9d02a4, 0x9d02a8), (0x9d02ac, 0x9d02b0), (0x9d02b4, 0x9d02b8),
    (0x9d02bc, 0x9d02c0), (0x9d02c4, 0x9d02c8), (0x9d02cc, 0x9d02d0), (0x9d02d4, 0x9d02d8), (0x9d02dc, 0x9d02e0), (0x9d0440, 0x9d0444),
    (0x9d0448, 0x9d044c), (0x9d0450, 0x9d0454), (0x9d0458, 0x9d045c), (0x9d0460, 0x9d0464), (0x9d0468, 0x9d046c), (0x9d0470, 0x9d0474),
    (0x9d0478, 0x9d047c), (0x9d0480, 0x9d0484), (0x9d0488, 0x9d048c), (0x9d0490, 0x9d0494), (0x9d0498, 0x9d049c), (0x9d04a0, 0x9d04a4),
    (0x9d04a8, 0x9d04ac), (0x9d04b0, 0x9d04b4), (0x9d04b8, 0x9d04bc), (0x9d04c0, 0x9d04c4), (0x9d04c8, 0x9d04cc), (0x9d04d0, 0x9d04d4),
    (0x9d04d8, 0x9d04dc), (0x9d04e0, 0x9d04e4), (0x9d04e8, 0x9d04ec), (0x9d04f0, 0x9d04f4), (0x9d04f8, 0x9d04fc), (0x9d0500, 0x9d0504),
    (0x9d0508, 0x9d050c), (0x9d0510, 0x9d0514), (0x9d0518, 0x9d051c), (0x9d0520, 0x9d0524),
];
const PARES_B_A: [(usize, usize); 82] = [
    (0x9d0130, 0x9d012c), (0x9d0134, 0x9d0120), (0x9d013c, 0x9d0138), (0x9d0144, 0x9d0140), (0x9d014c, 0x9d0148), (0x9d0154, 0x9d0150),
    (0x9d015c, 0x9d0158), (0x9d0164, 0x9d0160), (0x9d016c, 0x9d0168), (0x9d0174, 0x9d0170), (0x9d017c, 0x9d0178), (0x9d0184, 0x9d0180),
    (0x9d018c, 0x9d0188), (0x9d0194, 0x9d0190), (0x9d019c, 0x9d0198), (0x9d01a0, 0x9d0128), (0x9d0258, 0x9d0254), (0x9d0328, 0x9d02e4),
    (0x9d03b0, 0x9d03ac), (0x9d03b8, 0x9d03b4), (0x9d03c0, 0x9d03bc), (0x9d03c8, 0x9d03c4), (0x9d03d4, 0x9d0124), (0x9d03dc, 0x9d03d8),
    (0x9d03e4, 0x9d03e0), (0x9d03ec, 0x9d03e8), (0x9d03f4, 0x9d03f0), (0x9d03fc, 0x9d03f8), (0x9d0404, 0x9d0400), (0x9d040c, 0x9d0408),
    (0x9d0414, 0x9d0410), (0x9d041c, 0x9d0418), (0xb63c9c, 0xb63cdc), (0xb63ca0, 0xb63cbc), (0xb63cac, 0xb63ce8), (0xb63cd0, 0xb63cd8),
    (0xb63cb0, 0xb63cb4), (0xb63ca8, 0xb63cd4), (0xb63cec, 0xb63ce4), (0xb63cf8, 0xb63cf0), (0xb63d00, 0xb63cc0), (0xb63ccc, 0xb63cc8),
    (0xb63cb8, 0xb63ce0), (0xb63cf4, 0xb63cc4), (0xb63ca4, 0xb63cfc), (0x9d02a8, 0x9d02a4), (0x9d02b0, 0x9d02ac), (0x9d02b8, 0x9d02b4),
    (0x9d02c0, 0x9d02bc), (0x9d02c8, 0x9d02c4), (0x9d02d0, 0x9d02cc), (0x9d02d8, 0x9d02d4), (0x9d02e0, 0x9d02dc), (0x9d0444, 0x9d0440),
    (0x9d044c, 0x9d0448), (0x9d0454, 0x9d0450), (0x9d045c, 0x9d0458), (0x9d0464, 0x9d0460), (0x9d046c, 0x9d0468), (0x9d0474, 0x9d0470),
    (0x9d047c, 0x9d0478), (0x9d0484, 0x9d0480), (0x9d048c, 0x9d0488), (0x9d0494, 0x9d0490), (0x9d049c, 0x9d0498), (0x9d04a4, 0x9d04a0),
    (0x9d04ac, 0x9d04a8), (0x9d04b4, 0x9d04b0), (0x9d04bc, 0x9d04b8), (0x9d04c4, 0x9d04c0), (0x9d04cc, 0x9d04c8), (0x9d04d4, 0x9d04d0),
    (0x9d04dc, 0x9d04d8), (0x9d04e4, 0x9d04e0), (0x9d04ec, 0x9d04e8), (0x9d04f4, 0x9d04f0), (0x9d04fc, 0x9d04f8), (0x9d0504, 0x9d0500),
    (0x9d050c, 0x9d0508), (0x9d0514, 0x9d0510), (0x9d051c, 0x9d0518), (0x9d0524, 0x9d0520),
];

/// { 0x540a50 } (Database.cpp): o clube parceiro (time A <-> time B). So para clubes de nacao com
/// id em 0x9cf488, 0x9cf314 ou 0x9cf434 (e, com `ativo`, com a marca +0x244). Clube A: `eh_a` = 1 e
/// o parceiro e o B do primeiro par que casa (aceito se o id dele for diferente de -1); clube B:
/// `eh_a` = 0 e o parceiro e o A (aceito se o id for >= 0). Parceiro recusado: zera +0x244 e 0.
/// Devolve o registro do parceiro (vetor 0xae23b4, 581 bytes cada). Clube nulo: a original.
unsafe extern "C" fn clube_parceiro(clube: usize, eh_a: *mut u32, ativo: u32) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut u32, u32) -> usize>(197)(clube, eh_a, ativo);
    }
    let nacao = *((clube + 0x53) as *const usize);
    if nacao == 0 || (ativo != 0 && byte(clube, 0x244) == 0) {
        return 0;
    }
    let g = |a: usize| *(a as *const u32);
    let n = *(nacao as *const u32);
    if n != g(0x9cf488) && n != g(0x9cf314) && n != g(0x9cf434) {
        return 0;
    }
    let id = *(clube as *const u32);
    let parceiro = if let Some(&(_, b)) = PARES_A_B.iter().find(|&&(a, _)| g(a) == id) {
        if !eh_a.is_null() {
            *eh_a = 1;
        }
        let v = g(b) as i32;
        (v != -1).then_some(v)
    } else if let Some(&(_, a)) = PARES_B_A.iter().find(|&&(b, _)| g(b) == id) {
        if !eh_a.is_null() {
            *eh_a = 0;
        }
        let v = g(a) as i32;
        (v > -1).then_some(v)
    } else {
        return 0;
    };
    match parceiro {
        Some(v) => (*(0xae23b4 as *const usize)).wrapping_add((v as usize).wrapping_mul(581)),
        None => {
            *((clube + 0x244) as *mut u8) = 0;
            0
        }
    }
}

// ---------- septuagesima segunda leva: falta pelo motor, tecnico humano (0x9b23), clubes preferidos ----------

/// { 0x6cfc60 } (match_eng.cpp): a decisao da falta (0x6e9820) com o arbitro do motor (+0xf5b2).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn falta_no_motor(
    m: usize, j: usize, outro: usize, tipo: i8, quem: i8, cartao: *mut u8, grav: *mut i8, vantagem: *mut u8,
) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8 = core::mem::transmute(0x6e9820usize);
    f(m + 0xf5b2, j, outro, tipo, quem, cartao, grav, vantagem)
}

/// { 0x5fea50 } (human_manager.cpp): byte +0x9b23 + `i` do registro do tecnico humano da pessoa (ou
/// do atual). Sem pessoa ou fora da faixa: a original (caixa de erro).
unsafe extern "thiscall" fn tecnico_9b23(this: usize, i: i8, pessoa: usize) -> u8 {
    match pessoa_e_registro(this, pessoa) {
        Some((_, r)) => *((r as isize + 0x9b23 + i as isize) as *const u8),
        None => orig::<unsafe extern "thiscall" fn(usize, i8, usize) -> u8>(194)(this, i, pessoa),
    }
}

/// { 0x547d60 }: `pessoa` gosta do clube (preferencias +0x04, +0x08, +0x0c)?
unsafe extern "C" fn gosta_do_clube(pessoa: usize, clube: usize) -> u32 {
    nas_preferencias(clube, pessoa, 4)
}

/// { 0x547d30 }: `pessoa` nao gosta do clube (preferencias +0x10, +0x14, +0x18)?
unsafe extern "C" fn nao_gosta_do_clube(pessoa: usize, clube: usize) -> u32 {
    nas_preferencias(clube, pessoa, 0x10)
}

// ---------- septuagesima primeira leva: escolha do companheiro para a jogada (0x6c6a30, 7 KB) ----------

/// Menor distancia (tabela) de um jogador ativo do bloco `lado` (fora os de `tabela_a` com o bit
/// 1) ao ponto (x, y), comecando de 5000; 0 se algum esta a distancia 0.
unsafe fn menor_distancia_livre(m: usize, lado: usize, x: i8, y: i8) -> f32 {
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
unsafe extern "thiscall" fn escolher_companheiro(
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
    let marcas: unsafe extern "thiscall" fn(usize) -> u64 = core::mem::transmute(0x7163c0usize);
    let consegue: unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8 = core::mem::transmute(0x6f9f90usize);
    let casa_livre: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u8 = core::mem::transmute(0x6bce80usize);
    let classificar: unsafe extern "thiscall" fn(usize, i16, i8, i8, i8, i8, f32, *mut u16) -> u8 = core::mem::transmute(0x713e90usize);
    let alcanca_f: unsafe extern "stdcall" fn(usize, i8, i8) -> u8 = core::mem::transmute(0x6cb9b0usize);
    let espaco: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x6cbad0usize);
    let caminho: unsafe extern "thiscall" fn(usize, usize, i8) -> u8 = core::mem::transmute(0x6bd5b0usize);
    let pela_ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x6e1cd0usize);
    let preferido: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize = core::mem::transmute(0x897dc0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let ao_gol: unsafe extern "thiscall" fn(usize) -> f32 = core::mem::transmute(0x6fa1e0usize);

    let mut res = 0usize;
    *((m + 0x8eb2) as *mut u8) = 0;
    let c = *((m + 0xf582) as *const usize);
    let mut fator_nacao = f32::from_bits(0x3e4ccccd);
    let nac = *((*((c + 0x69) as *const usize) + 0x1a) as *const usize);
    if nac != 0 {
        let id = *(nac as *const u32);
        let g = |a: usize| *(a as *const u32) == id;
        if g(0x9cf2e4) || g(0x9cf45c) || g(0x9cf510) || g(0x9cf3e4) || g(0x9cf2dc) || g(0x9cf408) {
            fator_nacao = 1.0;
        } else if g(0x9cf4a8) || g(0x9cf2c0) || g(0x9cf314) {
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
        let faixa_l: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(0x6f5b20usize);
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

// ---------- septuagesima leva: decisao do arbitro na falta (0x6e9820, 3 KB) ----------

/// { 0x6e9820 } (match_man.cpp): o arbitro decide a falta `tipo` do jogador `j` sobre `outro`.
/// Devolve 1 ou 7 (cartao dado; 0 = nada) e escreve o cartao (4, 5, 7, 10, 11), a gravidade e a
/// marca `vantagem`. `this`: +0 motor, +4 arbitro (ou 0), +8 paciencia (palavra). Fator de rigor
/// (dobra, 1,0 sem arbitro): pela severidade (+0x25), pelo modo do lado (+0x1ad do motor) e o
/// estilo (+0x27), pela vantagem e pela diferenca de placar (0xf5c9 + 54 x lado), limitado a
/// 0,1..10. Cada tipo de falta sorteia contra o fator; depois a gravidade sobe pela dureza do
/// jogador (+0x14) e cai pelo minuto (0x8ed0 / 11). Com 0x6cb650, aplica a gravidade em `outro`
/// (0x716450, 0x7164b0) e guarda `j` em +0x1a6 dele.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn decisao_da_falta(
    this: usize,
    j: usize,
    outro: usize,
    tipo: i8,
    quem: i8,
    cartao: *mut u8,
    grav: *mut i8,
    vantagem: *mut u8,
) -> u8 {
    *grav = 0;
    let mut res = 0u8;
    let m = *(this as *const usize);
    if m == 0 {
        return 0;
    }
    let parado: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6bab90usize);
    if parado(m) != 0 {
        return 0;
    }
    let paciencia = (this + 8) as *mut u16;
    let arb = *((this + 4) as *const usize);
    if arb != 0 && sortear(byte(arb, 0x28) as i32 + 0xf) == 0 {
        *paciencia = (*paciencia).wrapping_add(0xff06);
        return 0;
    }
    let minuto = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
    *paciencia = (*paciencia).wrapping_add(0xaf);
    let faixa_l: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x6f5b20usize);
    let dura = |t: i8| t == 0x2a || t == 0xc || t == 0x15;
    if arb != 0 && dura(tipo) && faixa_l(j, 0) == 0 && faixa_l(j, 1) == 0 && sortear(50 - byte(arb, 0x24) as i32) == 0 {
        *paciencia = (*paciencia).wrapping_add(0x19);
        return 0;
    }
    if (faixa_l(j, 0) != 0 || faixa_l(j, 1) != 0) && (dura(tipo) || tipo == 0x2e) && sortear(3) != 0 {
        return 0;
    }
    // vantagem: (20, ou 5 na faixa do proprio lado) x (+0x5c da ficha de `outro`)^1,5 / (1 ou 3)
    let a = if faixa_l(j, byte(j, 0x27)) != 0 { 5.0 } else { 20.0 };
    let b = byte(*((outro + 0x69) as *const usize), 0x5c) as f64;
    let div = if *((outro + 0x1a6) as *const usize) != j { 1.0 } else { 3.0 };
    *vantagem = (sortear(ftol(a * pow_do_jogo(b, f64_em(0x968940)) / div)) == 0) as u8;
    let palavra = |p: usize, d: i16| *((p + 0x31) as *mut i16) = (*((p + 0x31) as *const i16)).wrapping_add(d);
    if *vantagem != 0 {
        palavra(outro, -4);
    }
    let mut fator = 1.0f64;
    let mut rigor = 0i8;
    if arb != 0 {
        let sev = byte(arb, 0x25);
        fator = if sev > 10 { sev as f64 * f64_em(0x967880) } else { (sev as i32 - 10) as f64 * f64_em(0x967888) + f64_em(0x967890) };
        let lado_j = byte(j, 0x27);
        let modo = *((m as isize + lado_j as isize + 0x1ad) as *const i8) as f64;
        fator += modo / (byte(arb, 0x27) as f64 * f64_em(0x968968) + f64_em(0x968930));
        if byte(j, 0x24) != 0 {
            let x = sortear(21 - sev as i32);
            *grav = (*grav).wrapping_add((x / -5) as i8);
        }
        if *vantagem != 0 {
            let y = sortear(byte(arb, 0x28) as i32);
            fator *= f64_em(0x968940) - y as f64 * f64_em(0x967888);
        }
        rigor = match sev {
            20 => 2,
            18..=i8::MAX => 1,
            10..=19 => 0,
            5..=9 => -1,
            _ => (sev > 2) as i8 - 3,
        };
        let placar = *((m + 0xf5c9 + 54 * lado_j as usize) as *const u8) as i32;
        if placar > byte(arb, 0x27) as i32 + 5 {
            *paciencia = (*paciencia).wrapping_add(0xff6a);
            rigor = rigor.wrapping_add(1);
            let z = sortear(4);
            fator *= (f64_em(0x969000) - z as f64) * f64_em(0x967880) + f64_em(0x967890);
        }
    }
    let partida = *((m + 0x4792) as *const usize);
    if partida != 0 {
        let comp = *((partida + 0x14) as *const usize);
        let amistoso = *(0xae2c74 as *const usize);
        if comp != 0 && amistoso != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
            if f(amistoso, comp) != 0 {
                rigor = rigor.wrapping_sub(1);
            }
        }
    }
    if f64_em(0x967880) > fator {
        fator = 0.1;
    } else if f64_em(0x9688b0) < fator || fator.is_nan() {
        fator = 10.0;
    }
    let dureza = *((j + 0x14) as *const u8) as i32;
    let somar = |v: i32| *grav = (*grav).wrapping_add(v as i8);
    let pelo_minuto = |limite: i8| {
        if minuto < limite {
            *grav = sortear(*grav as i32) as i8;
        }
    };
    // ajuste final pela ficha de `outro` (+0x5c): 1 -> `um`; >= 6 -> +1 abaixo de 10; senao `senao`
    let pela_ficha = |um: i32, senao: i32| {
        if *vantagem != 0 && outro != 0 {
            let c = byte(*((outro + 0x69) as *const usize), 0x5c);
            if c == 1 {
                *grav = (*grav).wrapping_add(um as i8);
            } else if c >= 6 {
                if c < 10 {
                    *grav = (*grav).wrapping_add(1);
                }
            } else {
                *grav = (*grav).wrapping_add(senao as i8);
            }
        }
    };
    let teto = |v: f64, k: f64| if v > k { v } else { k };
    match tipo {
        0x2a => {
            palavra(j, -4);
            if sortear(ftol(fator * f64_em(0x969000))) != 0 {
                res = 1;
                somar(sortear(dureza) + 4);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        0x2b => {
            palavra(j, -3);
            if sortear(ftol(fator * f64_em(0x969558))) != 0 {
                res = 7;
                somar(sortear(dureza) + 3);
                *cartao = 4;
                pelo_minuto(15);
            }
        }
        0x2c => {
            let v = teto(fator * f64_em(0x9688b0), f64_em(0x968ff8));
            palavra(j, -12);
            if sortear(ftol(v)) != 0 {
                res = 1;
                somar(sortear(dureza + 20) + rigor as i32 + 9);
                *cartao = 10;
                pela_ficha(3, 2);
            }
        }
        0x2d => {
            palavra(j, -4);
            if sortear(ftol(fator * f64_em(0x968ea8))) != 0 {
                if faixa_l(j, byte(j, 0x27)) == 0 || sortear(0x12) == 0 {
                    res = 1;
                    somar(sortear(dureza) + rigor as i32 + 4);
                    *cartao = 7;
                }
                pelo_minuto(6);
            }
        }
        0x2e => {
            palavra(j, -1);
            let r = sortear(ftol(fator * f64_em(0x969000)));
            if r != 0 || (outro != 0 && tabela_do_jogador(outro) == 1 && sortear(100) != 0) {
                res = 1;
                somar(sortear(dureza) + rigor as i32 + 1);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        0x2f | 0x60 => {
            let (v, d) = if tipo == 0x2f {
                (teto(fator * f64_em(0x9688b0), f64_em(0x9688a8)), -9)
            } else {
                (fator * f64_em(0x968ea8), -25)
            };
            palavra(j, d);
            if sortear(ftol(v)) != 0 {
                res = 1;
                let y = sortear(if tipo == 0x2f { dureza } else { dureza + 20 });
                somar(y + rigor as i32 + 9);
                *cartao = 5;
                pela_ficha(5, 3);
            }
        }
        0x5f => {
            palavra(j, -12);
            if sortear(ftol(fator * f64_em(0x9688b0))) != 0 {
                res = 1;
                somar(sortear(dureza + 7) + rigor as i32 + 3);
                *cartao = 5;
            }
        }
        0x61 => {
            palavra(j, -25);
            if sortear(ftol(fator + fator)) != 0 {
                res = 1;
                somar(sortear(dureza) + rigor as i32 + 20);
                *cartao = 5;
            }
        }
        0x30 | 0x31 | 0x32 => {
            let (k, d, extra, mais) = match tipo {
                0x30 => (0x968878, -100, 0, 20),
                0x31 => (0x968908, -200, 10, 20),
                _ => (0x968908, -60, 0, 18),
            };
            palavra(j, d);
            if sortear(ftol(fator * f64_em(k))) != 0 {
                res = 1;
                somar(sortear(dureza + extra) + rigor as i32 + mais);
                *cartao = 11;
            }
        }
        0x13 | 0x21 | 0x22 => {
            let segue = sortear(ftol(f64_em(0x96d520) / fator)) == 0
                || (quem == 0x6c && sortear(if *vantagem != 0 { 2 } else { 6 }) == 0);
            if segue && (faixa_l(j, byte(j, 0x27)) == 0 || sortear(ftol(f64_em(0x969550) / fator)) == 0) {
                res = 1;
                let a = sortear(dureza);
                let b = sortear(5);
                somar(a + b + rigor as i32);
                *cartao = 7;
                pela_ficha(5, 3);
            }
        }
        0x23 | 0x0c | 0x24 => {
            if sortear(ftol(f64_em(0x968f50) / fator)) == 0 {
                res = if tipo == 0x0c { 7 } else { 1 };
                somar(sortear(dureza) + 1);
                *cartao = 4;
                if tipo == 0x24 {
                    pela_ficha(3, 2);
                }
            }
        }
        0x15 | 0x38 => {
            let r = sortear(ftol(f64_em(0x968f50) / fator));
            if r == 0 || (outro != 0 && tabela_do_jogador(outro) == 1 && sortear(5) == 0) {
                res = 1;
                somar(sortear(dureza) + 1);
                *cartao = 4;
                pelo_minuto(15);
            }
        }
        0x29 => {
            if sortear(ftol(f64_em(0x9688a8) / fator)) == 0 {
                res = 1;
                somar(sortear(dureza) + 2);
                *cartao = 7;
                pelo_minuto(10);
            }
        }
        0x01 => {
            if sortear(ftol(f64_em(0x968910) / fator)) == 0 {
                res = 1;
                somar(sortear(dureza) + 1);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        _ => {}
    }
    let aplica: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x6cb650usize);
    if aplica(m, tipo) != 0 {
        let multiplicar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(0x716450usize);
        let teste: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(0x7164b0usize);
        multiplicar(outro, (*grav as f64 * f64_em(0x968918) + f64_em(0x967890)) as f32);
        if teste(outro, 1, 0) != 0 {
            *((outro + 0xff) as *mut u8) = (*((outro + 0xff) as *const u8)).wrapping_add(1);
            if teste(outro, 1, 0) != 0 {
                *((outro + 0x1a6) as *mut usize) = j;
            }
        }
    }
    res
}

// ---------- sexagesima nona leva: clube na competicao ----------

/// { 0x48db80 } (comp.cpp): o clube esta na competicao? Antes, pela nacao: selecao (id a partir
/// de clubes - nacoes) de outro continente que o da competicao (+4 -> +0x59): nao. Clube da nacao
/// da competicao (+4 -> +0x5d) e da propria divisao (+0x57): sim; de nacao de outro continente: nao.
/// Depois percorre a competicao (-1) e as fases (+0xc, ate +0x30): a ultima de tipo 2 (+0x43, com
/// tabela de classificacao em +0xb1, linhas de 0x3b) ou, sem nenhuma ainda, a fase da vez (lista
/// de participantes em +0xa7, 6 bytes cada); quantos em +0x3e.
unsafe extern "thiscall" fn clube_na_competicao(this: usize, clube: usize) -> u8 {
    let base = *((this + 4) as *const usize);
    let d = |p: usize, o: usize| *((p + o) as *const usize);
    let limite = (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32));
    let nacao = d(clube, 0x53);
    if *(clube as *const i32) >= limite {
        let continente = d(base, 0x59);
        if continente != 0 && (nacao == 0 || d(nacao, 0x71) != continente) {
            return 0;
        }
    } else if nacao != 0 {
        if nacao == d(base, 0x5d) && d(clube, 0x57) != 0 && d(clube, 0x57) == base {
            return 1;
        }
        if d(nacao, 0x71) != d(base, 0x59) {
            return 0;
        }
    }
    let n = *((this + 0x30) as *const i32);
    let (mut grupo, mut outra) = (0usize, 0usize);
    let mut i = -1i32;
    while i <= n {
        let fase = if i == -1 { this } else { d(d(this, 0xc), 4 * i as usize) };
        if byte(fase, 0x43) == 2 {
            grupo = fase;
        } else {
            outra = fase;
        }
        let (f, tabela, passo) = if grupo != 0 { (grupo, 0xb1, 0x3b) } else { (outra, 0xa7, 6) };
        let quantos = *((f + 0x3e) as *const i16);
        let lista = d(f, tabela);
        let mut k = 0i16;
        while k < quantos {
            if d(lista, passo * k as usize) == clube {
                return 1;
            }
            k += 1;
        }
        i += 1;
    }
    0
}

/// { 0x76d940 } (ret 4): o clube e um dos nove guardados em 0x9cf78c, 0x9cf790, 0x9cf79c..0x9cf7ac,
/// 0x9cf8a0 e 0x9cf964 (pelo id)?
unsafe extern "stdcall" fn clube_especial(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let id = *(clube as *const u32);
    [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964]
        .iter()
        .any(|&a| *(a as *const u32) == id) as u32
}

// ---------- sexagesima oitava leva: vagas da tabela tatica ----------

/// Linha tatica do clube (0x8982b0) e o inicio dela na tabela em +0x601 (0x91 bytes por linha), ou
/// `None` na linha -1.
unsafe fn linha_na_tabela(this: usize, clube: usize, humano: u32) -> Option<usize> {
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha(clube, humano);
    (l != -1).then(|| (*((this + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)))
}

/// { 0x897590 } (tactics.cpp): a vaga (0..20) cuja pessoa (indice em +2 + 4 x vaga, no vetor de
/// 0xae23bc) tem o id guardado em +0x52 da linha; sem nenhuma: -1 com `menos_um`, senao 0. Linha
/// -1: 0. Clube nulo: a original (caixa de erro).
unsafe extern "thiscall" fn vaga_marcada(this: usize, clube: usize, humano: u32, menos_um: u32) -> i8 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(186)(this, clube, humano, menos_um);
    }
    let Some(linha) = linha_na_tabela(this, clube, humano) else { return 0 };
    let pessoas = *(0xae23bc as *const usize);
    for vaga in 0..20usize {
        let i = *((linha + 2 + 4 * vaga) as *const i32);
        if i != -1 && *(pessoas.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == *((linha + 0x52) as *const i32) {
            return vaga as i8;
        }
    }
    -((menos_um != 0) as i8)
}

/// { 0x897760 } (tactics.cpp): guarda em +0x52 da linha o indice da pessoa da vaga (0..20); 1 se
/// havia alguem (-1 = vazia). Linha -1: 0. Clube nulo ou vaga fora: a original (caixa de erro).
unsafe extern "thiscall" fn marcar_vaga(this: usize, clube: usize, vaga: i8, humano: u32) -> u32 {
    if clube == 0 || !(0..20).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(187)(this, clube, vaga, humano);
    }
    let Some(linha) = linha_na_tabela(this, clube, humano) else { return 0 };
    let i = *((linha + 2 + 4 * vaga as usize) as *const i32);
    if i == -1 {
        return 0;
    }
    *((linha + 0x52) as *mut i32) = i;
    1
}

/// { 0x897dc0 } (tactics.cpp): a pessoa (vetor de 0xae23bc, 110 bytes cada) do indice em +0x56 +
/// 4 x `qual` (0 ou 1) da linha; indice negativo ou linha -1: 0. Clube nulo ou `qual` fora: a
/// original (caixa de erro).
unsafe extern "thiscall" fn pessoa_da_linha(this: usize, clube: usize, qual: i8, humano: u32) -> usize {
    if clube == 0 || !(0..2).contains(&qual) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(188)(this, clube, qual, humano);
    }
    let Some(linha) = linha_na_tabela(this, clube, humano) else { return 0 };
    let i = *((linha + 0x56 + 4 * qual as usize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(0xae23bc as *const usize)).wrapping_add(110 * i as usize)
}

// ---------- sexagesima setima leva: ordem dos jogadores no lance ----------

/// { 0x6bd760 } (match_eng.cpp): a lista dos jogadores ativos (+0x19 >= 0) em +0x8f7e (contagem em
/// +0x8eb8): para cada vaga 0..20 sorteia qual lado entra primeiro (0x90cfc0(2)) e poe o daquele lado
/// e depois o do outro. Ordena com o `qsort` da CRT (0x945333) e a comparacao do proprio jogo
/// (0x6bd840, que le o motor em 0xb67a64).
unsafe extern "thiscall" fn ordem_dos_jogadores(m: usize) {
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
    let qsort: unsafe extern "C" fn(usize, i32, i32, usize) = core::mem::transmute(0x945333usize);
    qsort(m + 0x8f7e, *n as i32, 4, 0x6bd840);
}

// ---------- sexagesima sexta leva: ordem dos cobradores de penalti ----------

/// { 0x6be520 } (match_eng.cpp): monta a ordem dos 11 cobradores de penalti de cada lado em
/// +0x1af + 11 x lado (-1 antes), marcando os escolhidos nas 20 casas de cada lado em +0x8eda
/// (zeradas antes). Se o tecnico do lado (+0x904d + 208 x lado) e um dos 16 humanos, tem a marca
/// +0x5f, nao tem 0x9a01 (0x5fd210) e, com +0x904b, e de selecao (0x53b900) ou tem 0x9a05
/// (0x5fd480): a ordem vem da tabela tatica do clube (0x898040 sobre [0xde1d08], com `humano` =
/// byte +0x40/+0x41 da partida igual a 0). Senao, `escolher_cobrador` tipo 5; sem ninguem, zera as
/// marcas do lado e tenta a mesma vez de novo.
unsafe extern "thiscall" fn ordem_dos_penaltis(m: usize) {
    core::ptr::write_bytes((m + 0x8eda) as *mut u8, 0, 40);
    core::ptr::write_bytes((m + 0x1af) as *mut u8, 0xff, 22);
    let h = *(0xb63c98 as *const usize);
    let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let t9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let tatico: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8 = core::mem::transmute(0x898040usize);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(0x6ceb90usize);
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

// ---------- sexagesima quinta leva: tabela tatica do clube ----------

/// { 0x8982b0 } (tactics.cpp; o `this` nao e usado, ret 8): a linha do clube na tabela tatica.
/// Clube dirigido por humano (0x600a50 com 1, 1) e `humano` ligado: segunda selecao, -1; senao a
/// linha do tecnico, depois dos clubes (0xae235c): quantidade de pessoas (0xae2364) - id do tecnico
/// (+0xcf) - 1, ou + 15 numa selecao (0x53b900), fora da faixa: a original (caixa de erro). Nos
/// outros casos, o id do clube.
unsafe extern "stdcall" fn linha_tatica(clube: usize, humano: u32) -> i32 {
    let e_humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if e_humano(clube, 1, 1, 0, 0) != 0 && humano != 0 {
        let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
        if segunda(clube) != 0 {
            return -1;
        }
        let tecnico = *((clube + 0xcf) as *const usize);
        if tecnico == 0 {
            return orig::<unsafe extern "stdcall" fn(usize, u32) -> i32>(182)(clube, humano);
        }
        let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        let clubes = *(0xae235c as *const i32);
        let base = (*(0xae2364 as *const i32)).wrapping_sub(*(tecnico as *const i32));
        let i = base.wrapping_add(if e_selecao(clube) != 0 { 0xf } else { -1 }).wrapping_add(clubes);
        if i < 0 || i >= clubes.wrapping_add(0x20) {
            return orig::<unsafe extern "stdcall" fn(usize, u32) -> i32>(182)(clube, humano);
        }
        return i;
    }
    *(clube as *const i32)
}

/// { 0x898040 } (tactics.cpp): byte `pos` (0..=10) da linha tatica do clube (0x8982b0) na tabela
/// em +0x601 (0x91 bytes por linha, a partir de +0x86); linha -1: 0. Clube nulo ou posicao fora:
/// a original (caixa de erro).
unsafe extern "thiscall" fn byte_tatico(this: usize, clube: usize, pos: i8, humano: u32) -> u8 {
    if clube == 0 || !(0..=10).contains(&pos) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8>(183)(this, clube, pos, humano);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha(clube, humano);
    if l == -1 {
        return 0;
    }
    let tabela = *((this + 0x601) as *const usize);
    *((tabela.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x86 + pos as usize) as *const u8)
}

// ---------- sexagesima quarta leva: lado sob controle ----------

/// { 0x6d42a0 } (match_eng.cpp): o lado esta sob controle de quem decide por ele? Nao nas
/// partidas de tipo +0x3a 1 ou 9. Clube do lado dirigido por humano (0x600a50, `qualquer` = byte
/// +0x40 da partida): pela marca 0x4000 do bloco do lado (0x9766 + 6371 x lado). Senao: o modo do
/// lado em +0x1ab e 3 ou 4.
unsafe extern "thiscall" fn lado_sob_controle(m: usize, lado: i8) -> u8 {
    let partida = *((m + 0x4792) as *const usize);
    let tipo = byte(partida, 0x3a);
    if tipo == 1 || tipo == 9 {
        return 0;
    }
    let clube = *((partida + if lado != 0 { 0x20 } else { 0x1c }) as *const usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let l = lado as isize;
    if humano(clube, 0, byte(partida, 0x40) as u8, 0, 0) != 0 {
        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x4000 != 0) as u8
    } else {
        let modo = *((m as isize + l + 0x1ab) as *const i8);
        (modo == 3 || modo == 4) as u8
    }
}

// ---------- sexagesima terceira leva: clube controlado por humano ----------

/// { 0x600a50 } (human_manager.cpp): o clube e dirigido por um tecnico humano que decide por ele?
/// O tecnico (`pessoa`, ou o do clube em +0xcf) tem de ser o do clube e um dos 16 humanos. Sem
/// `sem_tatica`: ficam de fora os que tem 0x9ace (0x5fe040) ou +0x9a01 no registro. Com `forcar` ou
/// `qualquer`: sim. Senao, selecao: 0x5fd6b0; clube: 0x5fd480. Se nao, e o clube e do segundo
/// conjunto de selecoes (0x5409b0), repete com a selecao principal da nacao dele (0x53b3d0),
/// ja sem `qualquer`. Clube nulo: a original (caixa de erro, zera 0xb67a34, 0).
unsafe extern "C" fn clube_humano(clube: usize, sem_tatica: u32, qualquer: u8, pessoa: usize, forcar: u32) -> u32 {
    let h = *(0xb63c98 as *const usize);
    let (mut clube, mut qualquer, mut pessoa) = (clube, qualquer, pessoa);
    let t9ace: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fe040usize);
    let t9a06: unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8 = core::mem::transmute(0x5fd6b0usize);
    let t9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    loop {
        if clube == 0 {
            return orig::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(180)(0, sem_tatica, qualquer, pessoa, forcar);
        }
        if pessoa == 0 {
            pessoa = *((clube + 0xcf) as *const usize);
        }
        let tecnico = *((clube + 0xcf) as *const usize);
        if tecnico != 0 && pessoa == tecnico && *(pessoa as *const i32) >= (*(0xae2364 as *const i32)).wrapping_sub(16) {
            let tatica = sem_tatica == 0
                && (t9ace(h, pessoa) != 0 || matches!(tecnico_humano(h, pessoa), Ok(r) if *((r + 0x9a01) as *const u32) != 0));
            if !tatica {
                if forcar != 0 || qualquer != 0 {
                    return 1;
                }
                let sim = if e_selecao(clube) != 0 { t9a06(h, pessoa, core::ptr::null_mut()) as u32 } else { t9a05(h, pessoa) };
                if sim != 0 {
                    return 1;
                }
            }
        }
        if segunda(clube) == 0 {
            return 0;
        }
        qualquer = 0;
        clube = principal(*((clube + 0x53) as *const usize));
    }
}

// ---------- sexagesima segunda leva: tecnico humano (0x9ace, 0x9a06) ----------

/// A pessoa (ou, sem ela, o tecnico humano atual) e o registro dela; `None` nos casos de erro do
/// original (caixa de mensagem), que ficam com ele.
unsafe fn pessoa_e_registro(this: usize, pessoa: usize) -> Option<(usize, usize)> {
    let p = if pessoa != 0 { pessoa } else { *((0xb74c26 + 768 * *(0xb77c7a as *const i16) as isize) as *const usize) };
    if p == 0 {
        return None;
    }
    tecnico_humano(this, p).ok().map(|r| (p, r))
}

/// { 0x5fe040 } (human_manager.cpp): campo +0x9ace do registro do tecnico humano da pessoa.
/// Sem pessoa nenhuma ou fora da faixa: a original (caixa de erro, zera 0xb67a34, 0).
unsafe extern "thiscall" fn tecnico_9ace(this: usize, pessoa: usize) -> u32 {
    match pessoa_e_registro(this, pessoa) {
        Some((_, r)) => *((r + 0x9ace) as *const u32),
        None => orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(178)(this, pessoa),
    }
}

/// { 0x5fd6b0 } (human_manager.cpp): byte +0x9a06 do registro do tecnico humano e diferente de
/// zero? Se a pessoa tem clube (+0x24) de nacao com +0x88 abaixo de 400: falso, e `saida` = 1.
/// `saida` (se houver) comeca em 0. Erros como em `tecnico_9ace`.
unsafe extern "thiscall" fn tecnico_9a06(this: usize, pessoa: usize, saida: *mut u32) -> u8 {
    if !saida.is_null() {
        *saida = 0;
    }
    match pessoa_e_registro(this, pessoa) {
        Some((p, r)) => {
            let clube = *((p + 0x24) as *const usize);
            if clube != 0 && *((*((clube + 0x53) as *const usize) + 0x88) as *const i32) < 400 {
                if !saida.is_null() {
                    *saida = 1;
                }
                return 0;
            }
            (*((r + 0x9a06) as *const u8) != 0) as u8
        }
        None => orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8>(179)(this, pessoa, saida),
    }
}

// ---------- sexagesima primeira leva: quem marcar (0x6bc030, 3 KB) ----------

/// Distancia (mais o ajuste 0x968990) do ponto (x, y) ao gol que o lado do jogador `j` ataca
/// (coluna 4; linha 0 para o lado 0, 11 para o lado 1), arredondada para float.
unsafe fn distancia_do_gol(j: usize, x: i32, y: i32) -> f32 {
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
unsafe fn cerco(m: usize, s: usize, x: i8, y: i8) -> i8 {
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

static mut TABELA65: Vec<u8> = Vec::new();
static mut CLUBES74: Vec<u8> = Vec::new();

/// Bonus de fadiga do sorteio: 0x96d2d8 - distancia ao gol x 0x968a78.
unsafe fn bonus_do_gol(d: f32) -> i32 {
    ftol(f32_em(0x96d2d8) as f64 - d as f64 * f32_em(0x968a78) as f64)
}

/// Reputacao usada na nota do alvo: a de casa se ele e da nacao do jogador, a atual se os dois
/// clubes da partida sao da mesma nacao, senao a mundial; em centenas, menos 50.
unsafe fn reputacao_do_alvo(j: usize, p: usize, clubes: (usize, usize)) -> i32 {
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
unsafe extern "thiscall" fn quem_marcar(m: usize, j: usize) -> usize {
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

// ---------- sexagesima leva: clube da selecao ----------

/// Clube da selecao da nacao: as selecoes ficam no fim do vetor de clubes (0xae23b4, 581 bytes
/// cada), a principal `deslocamento` = 2 nacoes antes do fim e a B uma nacao antes. Sem nacao,
/// sem clubes ou sem 0xae23a8: o original mostra a caixa de erro (Database.cpp) e zera 0xb67a34;
/// esse caminho fica com ele.
unsafe fn clube_da_selecao(nacao: usize, nacoes_antes: i32, idx: usize) -> usize {
    let base = *(0xae23b4 as *const usize);
    if nacao == 0 || base == 0 || *(0xae23a8 as *const usize) == 0 {
        return orig::<unsafe extern "C" fn(usize) -> usize>(idx)(nacao);
    }
    let i = (*(nacao as *const i32))
        .wrapping_sub(nacoes_antes.wrapping_mul(*(0xae2350 as *const i32)))
        .wrapping_add(*(0xae235c as *const i32));
    base.wrapping_add((i as usize).wrapping_mul(581))
}

/// { 0x53b3d0 } (Database.cpp): o clube da selecao principal da nacao.
unsafe extern "C" fn selecao_principal(nacao: usize) -> usize {
    clube_da_selecao(nacao, 2, 175)
}

/// { 0x53b4f0 } (Database.cpp): o clube da selecao B da nacao.
unsafe extern "C" fn selecao_b(nacao: usize) -> usize {
    clube_da_selecao(nacao, 1, 176)
}

/// { 0x6cd490 } (match_eng.cpp): bola fora do campo (ver `bola_fora_com`).
unsafe extern "thiscall" fn bola_fora(this: usize, lado_b: u8) -> u8 {
    bola_fora_com(this, lado_b, &mut |t, a| comentar(t, a))
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

// ---------- quinquagesima setima leva: lateral, escanteio e penalti ----------

unsafe fn mover(j: usize, x: i8, y: i8) {
    let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(0x6f46b0usize);
    f(j, x, y);
}
unsafe fn marcar_2b(j: usize) {
    let f: unsafe extern "thiscall" fn(usize, u8) = core::mem::transmute(0x7160e0usize);
    f(j, 0);
}
unsafe fn pega_bola(j: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0630usize);
    f(j);
}
unsafe fn cobrador(this: usize, lado: i8, tipo: i8) -> usize {
    let f: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(0x6ceb90usize);
    f(this, lado, tipo, 0)
}

/// { 0x6c4290 } (match_eng.cpp): lateral do lado 0x8eae. Conta (0xf5c8 + 54 x lado), poe a bola
/// (0x8ea7, 0x8ea8) dentro do campo, escolhe o cobrador (tipo 2), arruma os 40 jogadores pela tabela
/// da formacao (0xf56e/0xf572 pelo lado do jogador; indice: do lado que cobra?, posicao, coluna e
/// linha da bola; byte = coluna | linha << 4) e poe o cobrador na bola, virado para dentro (90 ou 270).
/// Sem cobrador o original mostra uma caixa de erro (sem a caixa aqui) e marca o motor (+0 = 1).
unsafe extern "thiscall" fn lateral(this: usize) {
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

/// { 0x6c4470 } (match_eng.cpp): escanteio do lado 0x8eae. Conta (0xf5c7 + 54 x lado), marca o
/// lance (0x8eb1 = 3, 0x8eae = -1) e arruma os 40 jogadores: pelo papel no ataque (0x716270, do lado
/// que cobra) ou na defesa (0x716310), alguns vao para casas sorteadas na area (coluna 4 +- 1; linha
/// 5/6, ou 0..1/10..11 sorteada), os outros pela tabela da formacao (0xf56e lado 0, 0xf572 lado 1;
/// +0x3b ou +0x30). Fora do campo: erro fatal. Depois escolhe o cobrador (tipo 3) e o poe na
/// bandeira (coluna 0 ou 8 pelo sorteio contra 0x8ea7; linha 0 ou 11).
unsafe extern "thiscall" fn escanteio(this: usize) {
    let ataque: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x716270usize);
    let defesa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x716310usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
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

/// { 0x6c50d0 } (match_eng.cpp): penalti do lado 0x8eae. Escolhe o cobrador (tipo 5), da a bola a
/// ele na marca (coluna 4; linha 1 ou 10, tambem em 0x8ea7/0x8ea8) e tira os outros da area: os
/// goleiros guardados (0xf59e, 0xf5a2) vao para o gol (linha 0 / 11), quem esta nas linhas 0..1 vai
/// para a 2 e quem esta nas 10..11 para a 9, na mesma coluna.
unsafe extern "thiscall" fn penalti(this: usize) {
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
unsafe fn erro_fatal() -> ! {
    *(0xb67a34 as *mut u32) = 0;
    let a: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let b: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
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
unsafe extern "thiscall" fn escolher_cobrador(this: usize, lado: i8, tipo: i8, excluidos: usize) -> usize {
    let dist: unsafe extern "thiscall" fn(usize, i8, i8) -> f32 = core::mem::transmute(0x6f09b0usize);
    let tatica: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32) -> usize = core::mem::transmute(0x897990usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let f5fd210: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let f5fd480: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
    let f601070: unsafe extern "thiscall" fn(usize, i32, usize) -> usize = core::mem::transmute(0x601070usize);
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
unsafe extern "C" fn reputacao_do_jogador(reg: usize, meu: usize, outro: usize, partida: usize, valor: f32, alt: u8, bonus: u8) {
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
unsafe fn razao_global(alt: bool) -> f64 {
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
unsafe extern "C" fn nova_reputacao(this: usize, lado: i8, valor: f32, alt: u8, bonus: u8) {
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

/// { 0x6bbf60 } (match_eng.cpp): marcacao individual. Para cada lado e jogador ativo cuja posicao
/// tem as marcas 0x10 e 4 na tabela do lado (0x5b57c0), acha o marcador direto (0x6d4330) e grava
/// a ficha dele (0x69) na tabela por posicao (0x5b5910). Marcador -1 le o "jogador" anterior ao
/// bloco, como o original.
unsafe extern "thiscall" fn marcacao_individual(this: usize) {
    let marca: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x5b57c0usize);
    let marcador: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(0x6d4330usize);
    let marcar: unsafe extern "thiscall" fn(usize, i8, *const u32) -> u32 = core::mem::transmute(0x5b5910usize);
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
unsafe extern "thiscall" fn tirar_de_campo(this: usize, j: usize, modo: i8) {
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
        let na: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x714c60usize);
        if na(*marcado, j) != 0 {
            let tirar: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x714ca0usize);
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
        let t9a01: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
        if id >= npes - 16 && id < npes && byte(tec, 0x5f) != 0 && t9a01(gerente, tec) == 0 {
            if *((bloco_tec + 0x904b) as *const u8) == 0 {
                return;
            }
            let partida = *((this + 0x4792) as *const usize);
            let clube = *((partida + if lado_j == 0 { 0x1c } else { 0x20 }) as *const usize);
            let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
            if selecao(clube) != 0 {
                return;
            }
            let t9a05: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd480usize);
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
        let a: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
        let b: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
        let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
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
unsafe extern "thiscall" fn marcador_direto(this: usize, lado_b: i8, k: i8) -> u8 {
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
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
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

// ---------- quinquagesima leva: segunda selecao e placar agregado ----------

/// { 0x5409b0 } (Database.cpp): o clube e do segundo conjunto de selecoes (id a partir de total de
/// clubes 0xae235c menos nacoes 0xae2350)? Clube nulo: o original mostra uma caixa de erro e zera
/// 0xb67a34 (aqui, sem a caixa).
unsafe extern "C" fn e_segunda_selecao(clube: usize) -> u32 {
    if clube == 0 {
        // o caminho de erro do original (mensagem e caixa) tem efeitos que mudam a carreira: o jogo
        // real chama com clube nulo. Usa a propria original nesse caso.
        return orig::<unsafe extern "C" fn(usize) -> u32>(160)(0);
    }
    let limite = (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32));
    (*(clube as *const i32) >= limite) as u32
}

/// { 0x6ce310 } (match_eng.cpp): placar agregado de um mata-mata. `casa`/`fora` recebem os gols
/// desta partida (0xf5bd do lado 0, 0xf5f3 do lado 1; trocados com `lado` = 1), somados aos do jogo
/// de ida guardados no objeto da partida (+0x4b, +0x4c; 0xff = sem ida, e `ida` fica 1 com ida).
/// `fase3`: o algarismo das unidades de +0x30 e 3. Com o agregado empatado (mais 1 de bonus para o
/// lado de `bonus`), sem o bit 4 de +0x38 e (com o bit 0x4000) so depois do tempo 0x3de: decide pelos
/// gols fora (`classificado`). Devolve 1 classificado, senao casa - fora (byte).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn placar_agregado(this: usize, lado_b: i8, casa: *mut i8, fora: *mut i8, classificado: *mut u8, ida: *mut u8, fase3: *mut u8, bonus: u8) -> u8 {
    *casa = 0;
    *fora = 0;
    *classificado = 0;
    *ida = 0;
    *fase3 = 0;
    let (este, outro) = if lado_b == 0 { (casa, fora) } else { (fora, casa) };
    let (bonus_este, bonus_outro): (i8, i8) = match (lado_b == 0, bonus != 0) {
        (true, true) => (1, 0),
        (false, true) => (0, 1),
        _ => (0, 0),
    };
    *este = byte(this, 0xf5bd);
    *outro = byte(this, 0xf5f3);
    let partida = *((this + 0x4792) as *const usize);
    let unidades_: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    if unidades_(*((partida + 0x30) as *const i16) as i32) == 3 {
        *fase3 = 1;
    }
    let ida_casa = byte(partida, 0x4b);
    if ida_casa != -1 {
        *este = (*este).wrapping_add(ida_casa);
        *outro = (*outro).wrapping_add(byte(partida, 0x4c));
        *ida = 1;
        if *este as i32 + bonus_este as i32 == *outro as i32 + bonus_outro as i32 {
            let f = *((partida + 0x38) as *const u16);
            let tempo_ok = f & 0x4000 == 0 || *((this + 0x8ed4) as *const i16) > 0x3de;
            if f & 4 == 0 && tempo_ok {
                let gols_fora = *((this + 0xf5f3) as *const u8) as i32;
                if lado_b == 0 {
                    if gols_fora + (bonus_este as i32) < ida_casa as i32 {
                        *classificado = 1;
                    }
                } else if gols_fora + (bonus_outro as i32) > ida_casa as i32 {
                    *classificado = 1;
                }
            }
        }
    }
    if *classificado != 0 {
        1
    } else {
        (*casa as u8).wrapping_sub(*fora as u8)
    }
}

// ---------- quadragesima nona leva: tecnico humano e mascaras de formacao ----------

/// Registro do tecnico humano da pessoa (human_manager.cpp): os 16 tecnicos humanos sao as 16
/// ultimas pessoas (indice = id - quantidade de pessoas em 0xae2364 + 16), registros de 42.735
/// bytes a partir de +0 do objeto, quantidade em +4. Sem pessoa, usa a do tecnico atual (tabela
/// 0xb74c26, 768 bytes por tecnico, indice na palavra 0xb77c7a). `None`: sem pessoa nenhuma, ou
/// indice fora da faixa (o bool diz qual).
unsafe fn tecnico_humano(this: usize, pessoa: usize) -> Result<usize, bool> {
    let mut pessoa = pessoa;
    if pessoa == 0 {
        let atual = *(0xb77c7a as *const i16) as isize;
        pessoa = *((0xb74c26 + 768 * atual) as *const usize);
        if pessoa == 0 {
            return Err(true);
        }
    }
    let i = (*(pessoa as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(16);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(false);
    }
    Ok((*(this as *const usize)).wrapping_add((42735 * i) as usize))
}

/// { 0x5fd210 } (human_manager.cpp): campo +0x9a01 do registro do tecnico humano da pessoa (0 fora
/// da faixa). Sem pessoa nenhuma: o original mostra uma caixa de erro e zera 0xb67a34.
unsafe extern "thiscall" fn tecnico_9a01(this: usize, pessoa: usize) -> u32 {
    match tecnico_humano(this, pessoa) {
        Ok(r) => *((r + 0x9a01) as *const u32),
        Err(sem_pessoa) => {
            if sem_pessoa {
                *(0xb67a34 as *mut u32) = 0;
            }
            0
        }
    }
}

/// { 0x5fd480 } (human_manager.cpp): byte +0x9a05 do registro do tecnico humano da pessoa e diferente
/// de zero? Sem pessoa nenhuma ou fora da faixa: o original mostra uma caixa de erro e zera 0xb67a34.
unsafe extern "thiscall" fn tecnico_9a05(this: usize, pessoa: usize) -> u32 {
    match tecnico_humano(this, pessoa) {
        Ok(r) => (*((r + 0x9a05) as *const u8) != 0) as u32,
        Err(_) => {
            *(0xb67a34 as *mut u32) = 0;
            0
        }
    }
}

/// { 0x5b5a60 } (formation.cpp): bit da mascara de formacao pelo codigo da casa tatica (byte alto:
/// linha 1, 2, 4, 8; byte baixo: coluna). Tabela tirada do proprio jogo; 0x802 e 0x804 dao o mesmo
/// bit no original. Codigo desconhecido: 0.
unsafe extern "stdcall" fn bit_da_casa(codigo: i16) -> u32 {
    match codigo {
        0x1 => 0x1, 0x82 => 0x2, 0x84 => 0x40, 0x88 => 0x800, 0x90 => 0x10000, 0xa0 => 0x200000, 0xc0 => 0x4000000,
        0x102 => 0x4, 0x104 => 0x80, 0x108 => 0x1000, 0x110 => 0x20000, 0x120 => 0x400000, 0x140 => 0x8000000,
        0x202 => 0x8, 0x204 => 0x100, 0x208 => 0x2000, 0x210 => 0x40000, 0x220 => 0x800000, 0x240 => 0x10000000,
        0x402 => 0x10, 0x404 => 0x200, 0x408 => 0x4000, 0x410 => 0x80000, 0x420 => 0x1000000, 0x440 => 0x20000000,
        0x802 | 0x804 => 0x400, 0x808 => 0x8000, 0x810 => 0x100000, 0x820 => 0x2000000, 0x840 => 0x40000000,
        _ => 0,
    }
}

/// { 0x5b73a0 } (formation.cpp): separa a mascara em setor (o primeiro de 1, 2, 4 ... 0x40, em
/// `setor`) e lado (0x80, 0x100, 0x200 (tambem com o bit 1), 0x400, 0x800, nessa ordem, em `lado`).
/// Sem uma das partes: o original mostra uma caixa de erro, zera 0xb67a34 e devolve 0.
unsafe extern "C" fn separar_mascara(m: u32, setor: *mut u16, lado: *mut u16) -> u32 {
    match [1u16, 2, 4, 8, 0x10, 0x20, 0x40].iter().find(|&&b| m & b as u32 != 0) {
        Some(&b) => *setor = b,
        None => {
            *(0xb67a34 as *mut u32) = 0;
            return 0;
        }
    }
    let v = if m & 0x80 != 0 {
        0x80
    } else if m & 0x100 != 0 {
        0x100
    } else if m & 0x200 != 0 || m & 1 != 0 {
        0x200
    } else if m & 0x400 != 0 {
        0x400
    } else if m & 0x800 != 0 {
        0x800
    } else {
        *(0xb67a34 as *mut u32) = 0;
        return 0;
    };
    *lado = v;
    1
}

// ---------- quadragesima oitava leva: folhas de outros modulos ----------

/// { 0x4b0850 } (comp_util.cpp): algarismo das unidades de `n` (abaixo de 1000; senao 0).
unsafe extern "C" fn unidades(n: i32) -> u8 {
    if n < 1000 {
        (n % 10) as u8
    } else {
        0
    }
}

/// { 0x5c6710 } (friendly.cpp): a competicao e amistosa? E a guardada em +4 do objeto, ou o id dela
/// (+0) esta entre as 127 ultimas competicoes de clubes (quantidade em 0xae2378).
unsafe extern "thiscall" fn e_amistoso(this: usize, comp: usize) -> u32 {
    if comp == *((this + 4) as *const usize) {
        return 1;
    }
    (*(comp as *const i32) >= (*(0xae2378 as *const i32)).wrapping_sub(0x7f)) as u32
}

/// { 0x4cf5a0 } (contract_manager.cpp): contrato da pessoa (registros de 80 bytes a partir de +0
/// do objeto), pelo indice na tabela 0xae2d0c (79 bytes por pessoa; quantidade em 0xae2364).
/// Sem pessoa, id fora da tabela ou indice negativo: 0.
unsafe extern "thiscall" fn contrato_de(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let tabela = *(0xae2d0c as *const usize);
    let i = *((tabela as isize + 79 * id as isize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(this as *const usize)).wrapping_add((80 * i) as usize)
}

/// { 0x53b900 } (Database.cpp): o clube e uma selecao? Os ids das selecoes vem depois dos clubes
/// (total de clubes em 0xae235c menos 2 x nacoes em 0xae2350). Clube nulo: o original mostra uma
/// caixa de erro, zera 0xb67a34 e devolve 0 (aqui, sem a caixa).
unsafe extern "C" fn e_selecao(clube: usize) -> u32 {
    if clube == 0 {
        *(0xb67a34 as *mut u32) = 0;
        return 0;
    }
    let limite = (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2));
    (*(clube as *const i32) >= limite) as u32
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
unsafe extern "thiscall" fn posicoes_finas(this: usize, saida: *mut u8) {
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

// ---------- quadragesima sexta leva: preferencias da pessoa e marca por posicao ----------

/// Preferencias (`CM3_STAFF_PREFERENCES`, ponteiro em +0x65 da pessoa): `alvo` esta entre as tres
/// pessoas em `base`, +4, +8? (0 com qualquer ponteiro nulo.)
unsafe fn nas_preferencias(alvo: usize, pessoa: usize, base: usize) -> u32 {
    if alvo == 0 || pessoa == 0 {
        return 0;
    }
    let pref = *((pessoa + 0x65) as *const usize);
    if pref == 0 {
        return 0;
    }
    (0..3).any(|k| *((pref + base + 4 * k) as *const usize) == alvo) as u32
}

/// { 0x547810 }: `pessoa` gosta de `alvo` (preferencias +0x1c, +0x20, +0x24)?
unsafe extern "C" fn gosta_de(alvo: usize, pessoa: usize) -> u32 {
    nas_preferencias(alvo, pessoa, 0x1c)
}

/// { 0x547840 }: `pessoa` nao gosta de `alvo` (preferencias +0x28, +0x2c, +0x30)?
unsafe extern "C" fn nao_gosta_de(alvo: usize, pessoa: usize) -> u32 {
    nas_preferencias(alvo, pessoa, 0x28)
}

/// { 0x5b5910 }: grava na tabela por posicao (+0x5bf + 4 x posicao) o valor apontado por `ficha`
/// (-1 sem ficha) e zera +0x187f. Devolve 1.
unsafe extern "thiscall" fn marcar_posicao(this: usize, posicao: i8, ficha: *const u32) -> u32 {
    let destino = (this as isize + 0x5bf + 4 * posicao as isize) as *mut u32;
    *destino = if ficha.is_null() { u32::MAX } else { *ficha };
    *((this + 0x187f) as *mut u32) = 0;
    1
}

// ---------- quadragesima quinta leva: teste de condicao ----------

/// { 0x7164b0 } (match_pl.cpp): teste pela condicao fisica (0x1a2). v = condicao^2 (truncado);
/// limite 8 se `alvo` for o guardado em 0x1a6, senao 1. Codigo 1: sorteio(110 - condicao) < limite;
/// 2 a 5: sorteio(K / v) < limite, K = 400 mil, 2, 4 e 8 milhoes (mais condicao, mais chance).
/// Outros codigos: 0.
unsafe extern "thiscall" fn teste_de_condicao(this: usize, codigo: i8, alvo: usize) -> u8 {
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

// ---------- quadragesima quarta leva: bola rolando ----------

/// { 0x6bbe40 } (match_eng.cpp) com o comentario injetado: bola solta rolando. So sem dono
/// (0xf582), sem 0xf57a e com impulso (0x8eaa). Lance parado (0x8eb2): zera o impulso e 0x8ea9.
/// Senao, com impulso 1 a bola so anda se sorteio(1) >= sorteio(2); impulso <= 0 nao anda. Ao andar,
/// gasta um de impulso (e um de 0x8ea9, se houver) e da um passo pela direcao (0x8ed8, graus):
/// (220, 320) x - 1; (40, 140) x + 1; (130, 230) y - 1; abaixo de 50 ou acima de 310, y + 1.
/// Fora do campo (x fora de 0..9 ou y fora de 0..12): bola fora (0x6cd490) pelo lado de 0x8eab.
unsafe fn rolar_bola_com(this: usize, chamar: ChamarComentario) {
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

/// { 0x6bbe40 } (match_eng.cpp): bola rolando (ver `rolar_bola_com`).
unsafe extern "thiscall" fn rolar_bola(this: usize) {
    rolar_bola_com(this, &mut |t, a| comentar(t, a))
}

// ---------- quadragesima terceira leva: pontape inicial ----------

/// { 0x6c51e0 } (match_eng.cpp): pontape inicial. Quem sai jogando: 0x8eae (fora de 0..=1, o lado
/// 0); bola no centro (4, 5 ou 6). Todos os ativos vao para a casa da formacao (tabela 0xf56e + 0x9e
/// do lado 0 ou 0xf572 + 0xa6 do lado 1, 216 bytes por posicao; nibble baixo x, alto y, cada um no
/// seu campo) e olham para a frente. Do lado que sai, o mais perto da bola (posicao > 10 vira -1)
/// pega a bola e olha para a casa calculada com a ultima casa da formacao usada (mesmo ponto: o
/// byte baixo de 0x8ed8). Sem ninguem, marca o byte 0 do motor.
unsafe extern "thiscall" fn pontape_inicial(this: usize) {
    let mut sai = byte(this, 0x8eae);
    if !(0..=1).contains(&sai) {
        sai = 0;
    }
    *((this + 0x8eae) as *mut i8) = -1;
    *((this + 0x8ea7) as *mut i8) = 4;
    *((this + 0x8ea8) as *mut i8) = if sai == 0 { 5 } else { 6 };
    let mover: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(0x6f46b0usize);
    let apontar_para: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) = core::mem::transmute(0x6f4920usize);
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
    let pegar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6f0630usize);
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
unsafe fn degrau(v: f64, x: i8, regras: &[(char, i8, usize)], arredondar: bool) -> f64 {
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
unsafe fn sorte_do_dia(v: f64, pessoa: usize) -> f64 {
    let a = sortear(byte(pessoa, 0x21) as i32);
    if a < sortear(0x14) {
        (v * f32_em(0x96d248) as f64) as f32 as f64
    } else {
        v
    }
}

/// Palavra do motor (com sinal) vezes `k`, truncada de volta na palavra `destino`.
unsafe fn palavra_vezes(this: usize, origem: usize, k: f64, destino: usize) {
    let v = *((this + origem) as *const i16) as f64;
    *((this + destino) as *mut i16) = ftol(v * k) as i16;
}

/// { 0x6c0c60 } (match_eng.cpp): forca do time por lado em sete palavras (0x1da, 0x1de ... 0x1f2,
/// de 4 em 4, + 2 x lado), somando os 20 jogadores do bloco. Por jogador ativo: desgaste,
/// componentes (combinacoes dos floats do jogador, ajustadas pelos atributos da pessoa e por
/// sorteios) e, pelo papel tatico (bits 1..0x40), ajuste da palavra 0x35 e as parcelas.
/// Depois, ajustes pela competicao, pela reputacao dos clubes e pelo lado.
/// Jogador sem ficha (0x69): o original mostra uma caixa de erro e zera 0xb67a34 (aqui, so isso).
unsafe extern "thiscall" fn forca_do_time(this: usize, lado_b: i8) {
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
        let desgastar: unsafe extern "thiscall" fn(usize, f32, i16) = core::mem::transmute(0x6f56c0usize);
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
        let condicao: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x7165b0usize);
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
                let k = if id == g(0x9cf314) || id == g(0x9cf45c) || id == g(0x9cf244) {
                    0x96d390
                } else if id == g(0x9cf4a8) {
                    0x96b010
                } else if id == g(0x9cf408) || id == g(0x9cf2e4) {
                    0x96d3b8
                } else if id == g(0x9cf338) || id == g(0x9cf2c0) || id == g(0x9cf434) || id == g(0x9cf300) {
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
            let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x5c6710usize);
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

// ---------- quadragesima primeira leva: valor do chute (0x700bd0, 5,2 KB) ----------

/// pow da CRT do jogo (0x945ff0, `_CIpow`: base em st1, expoente em st0), para o resultado ser
/// identico bit a bit ao do original.
unsafe fn pow_do_jogo(base: f64, expoente: f64) -> f64 {
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
unsafe fn tabela_12x9(dy: i32, dx: i32) -> f32 {
    f32_em((0xa15e44 + 4 * ((dy.abs() as i8 as isize) * 9 + dx.abs() as i8 as isize)) as usize)
}

/// O jogador pode bater dali (0x6f9f90 para o centro do gol, salvo na pequena area central)?
unsafe fn pode_bater(j: usize) -> bool {
    let (x, y) = (byte(j, 0x102), byte(j, 0x103));
    let fundo: i8 = if lado(j) == 0 { 0xb } else { 0 };
    if y == fundo && (3..=5).contains(&x) {
        return true;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8 = core::mem::transmute(0x6f9f90usize);
    f(j, 4, fundo, 0, 0.0, 0.0, core::ptr::null_mut()) != 0
}

/// `tabela_a` do jogador por 0x6c38c0 (o motor do proprio jogador; sem motor, None).
unsafe fn papel_do(j: usize) -> Option<u16> {
    pelo_motor::<u16>(j, 0x6c38c0)
}

/// { 0x700bd0 } (match_pl.cpp, `MATCH_PLAYER::get_shot_score`): escolhe o tipo do chute (quando
/// `tipo` < 0) pela situacao e pelos atributos (palavras 0x146..0x154), calcula o valor (palavras
/// por tipo em 0x11a + 2 x tipo, condicao fisica, goleiro adversario, angulo, distancia ao gol com
/// pow 8 e 7) e grava em 0x39 a parte "de qualidade" e em `som` o comentario. Devolve o valor.
unsafe extern "thiscall" fn valor_do_chute(this: usize, tipo: *mut i8, som: *mut u16) -> u16 {
    let j = this;
    let w = |o: usize| *((j + o) as *const i16);
    let q39 = (j + 0x39) as *mut u16;
    let menos_mil = |tipo: *mut i8, som: *mut u16| {
        *tipo = 0x16;
        *som = 0x1f81;
        0xfc18u16
    };
    let manter: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(0x6ce110usize);
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

macro_rules! comparar_valor {
    ($nome:ident, $rust:ident, $i:expr, ($($a:ident: $t:ty),*) -> $r:ty) => {
        unsafe extern "thiscall" fn $nome(this: usize $(, $a: $t)*) -> $r {
            let r = orig::<unsafe extern "thiscall" fn(usize $(, $t)*) -> $r>($i)(this $(, $a)*);
            let n = $rust(this $(, $a)*);
            PLACAR.registrar(r == n, || format!("{}({:#x}{}) original={:#x} rust={:#x}", stringify!($rust), this, String::new() $(+ &format!(", {:?}", $a))*, r, n));
            r
        }
    };
}
comparar_valor!(comparar_direcao, direcao, 66, () -> u16);
comparar_valor!(comparar_x_fora, x_fora_do_meio, 68, () -> u8);
comparar_valor!(comparar_e_o_guardado, e_o_guardado, 69, () -> u8);
comparar_valor!(comparar_lado_do_lance, lado_do_lance, 70, () -> u8);
comparar_valor!(comparar_jogador_no_bloco, jogador_no_bloco, 71, (l: i8, k: i8) -> usize);
comparar_valor!(comparar_tabela_a_1a, tabela_a_1a, 72, (j: usize) -> u16);
comparar_valor!(comparar_momento_marcado, momento_marcado, 73, () -> u32);
comparar_valor!(comparar_estado_do_lado, estado_do_lado, 74, () -> u8);
comparar_valor!(comparar_estado_do_adversario, estado_do_adversario, 75, () -> u8);
comparar_valor!(comparar_perto_do_gol, perto_do_gol, 78, () -> u8);
unsafe extern "thiscall" fn comparar_zerar_guardado(this: usize, l: u8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, u8)>(67);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0xf59e, 8)], || f(this, l), || zerar_guardado(this, l));
    PLACAR.registrar(igual, || format!("zerar_guardado({}) original={:x?} rust={:x?}", l, mo, mr));
}
unsafe extern "thiscall" fn comparar_gravar_19c(this: usize, v: i16) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i16)>(76);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x19c, 2)], || f(this, v), || gravar_19c(this, v));
    PLACAR.registrar(igual, || format!("gravar_19c({}) original={:x?} rust={:x?}", v, mo, mr));
}
unsafe extern "stdcall" fn comparar_bola_parada(c: i16) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i16) -> u8>(77)(c);
    let n = codigo_de_bola_parada(c);
    PLACAR.registrar(r == n, || format!("codigo_de_bola_parada({}) original={} rust={}", c, r, n));
    r
}
unsafe extern "thiscall" fn comparar_limpar_lance(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(79);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x475a, 16)], || f(this), || limpar_lance(this));
    PLACAR.registrar(igual, || format!("limpar_lance original={:x?} rust={:x?}", mo, mr));
}
comparar_repartir!(comparar_somar_disposicao, somar_disposicao, 80);

comparar_repartir!(comparar_repartir_d, repartir_d, 81);
comparar_repartir!(comparar_repartir_e, repartir_e, 82);
comparar_repartir!(comparar_repartir_f, repartir_f, 83);
unsafe extern "thiscall" fn comparar_registrar_lance(this: usize, a: u16, b: u32, c: u32, d: u8, e: u8, f2: u8, g: u8, h: u8, i: u8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8)>(84);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x475a, 16)], || f(this, a, b, c, d, e, f2, g, h, i), || registrar_lance(this, a, b, c, d, e, f2, g, h, i));
    PLACAR.registrar(igual, || format!("registrar_lance original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_remover(this: usize, alvo: usize) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(85);
    let n = byte(this, 0x101).max(0) as usize;
    let reg = [(this + 0x101, 1), (this + 0x1ae, 4 * n)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this, alvo), || remover_da_lista(this, alvo));
    PLACAR.registrar(igual && r == x, || format!("remover_da_lista(n {}) original={} {:x?} rust={} {:x?}", n, r, mo, x, mr));
    r
}
unsafe extern "thiscall" fn comparar_entrar(this: usize, j: usize) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(86);
    let l = lado(j) as isize;
    let n = *((this as isize + 0x4592 + l) as *const i8) as isize;
    let reg = [(j + 0x28, 1), (j + 0x102, 2), ((this as isize + 0x4592 + l) as usize, 1), ((this as isize + 0x453a + 4 * (n + 11 * l)) as usize, 4)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this, j), || entrar_em_campo(this, j));
    PLACAR.registrar(igual && r == x, || format!("entrar_em_campo(lado {}, n {}) original={} {:x?} rust={} {:x?}", l, n, r, mo, x, mr));
    r
}
comparar_valor!(comparar_em_perigo, em_perigo, 87, () -> u8);

unsafe extern "thiscall" fn comparar_conferir_lance(this: usize) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize) -> u8>(88);
    let mut reg = vec![(this + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this), || crate::aleatorio::sem_contar(|| conferir_lance(this)));
    PLACAR.registrar(igual && r == n, || format!("conferir_lance original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}
comparar_valor!(comparar_caminho_livre, caminho_livre_ate, 89, (l: i8, x: i8, y: i8) -> u8);
comparar_valor!(comparar_alcanca_ponto, alcanca_o_ponto, 90, (x: i8, y: i8) -> u8);

unsafe extern "thiscall" fn comparar_destaque(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(91);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x477a, 4)], || f(this), || escolher_destaque(this));
    PLACAR.registrar(igual, || format!("escolher_destaque original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_escolher_b(this: usize, arg: u32) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, u32) -> u8>(92);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, arg), || crate::aleatorio::sem_contar(|| escolher_por_papel_b(this, arg)));
    PLACAR.registrar(igual && r == n, || format!("escolher_por_papel_b({:#x}) original={} rust={}", arg, r as i8, n as i8));
    r
}
unsafe extern "thiscall" fn comparar_casa_formacao(this: usize, j: usize, x: *mut u8, y: *mut u8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(93);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(x as usize, 1), (y as usize, 1)], || f(this, j, x, y), || casa_da_formacao(this, j, x, y));
    PLACAR.registrar(igual, || format!("casa_da_formacao(8eae {}, lado {}, pos {}) original={:x?} rust={:x?}", byte(this, 0x8eae), byte(j, 0x27), byte(j, 0x19), mo, mr));
}

unsafe extern "thiscall" fn comparar_percentuais(this: usize, i: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8)>(94);
    let mut reg = Vec::new();
    for s in [i as isize, 3, 4] {
        for o in [0xf5dd, 0xf609, 0xf5d8, 0xf60e, 0xf613, 0xf5d3, 0xf5ce, 0xf604] {
            reg.push(((this as isize + o + s) as usize, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this, i), || calcular_percentuais(this, i));
    PLACAR.registrar(igual, || format!("calcular_percentuais({}) original={:x?} rust={:x?}", i, mo, mr));
}
unsafe extern "thiscall" fn comparar_escolher_c(this: usize, arg: u32, modo: u32) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(95);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, arg, modo), || crate::aleatorio::sem_contar(|| escolher_por_papel_c(this, arg, modo)));
    PLACAR.registrar(igual && r == n, || format!("escolher_por_papel_c({:#x}, {}) original={} rust={}", arg, modo as u8, r as i8, n as i8));
    r
}

unsafe extern "thiscall" fn comparar_aplicar_fator(this: usize, f: f32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, f32)>(96);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x1a2, 4)], || o(this, f), || aplicar_fator(this, f));
    PLACAR.registrar(igual, || format!("aplicar_fator({}) original={:x?} rust={:x?}", f, mo, mr));
}
comparar_valor!(comparar_valor_condicao, valor_pela_condicao, 97, (alvo: usize) -> i32);
unsafe extern "thiscall" fn comparar_aplicar_time(this: usize, j: usize, f: f32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, f32)>(98);
    let reg: Vec<(usize, usize)> = (0..20).map(|k| (jogador_do_bloco(this, lado(j), k) + 0x1a2, 4)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, f), || aplicar_ao_time(this, j, f));
    PLACAR.registrar(igual, || format!("aplicar_ao_time({}) original={:x?} rust={:x?}", f, mo, mr));
}
unsafe extern "thiscall" fn comparar_aplicar_lado(this: usize, l: i8, f: f32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, f32)>(99);
    let reg: Vec<(usize, usize)> = (0..20).map(|k| (jogador_do_bloco(this, l, k) + 0x1a2, 4)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l, f), || aplicar_ao_lado(this, l, f));
    PLACAR.registrar(igual, || format!("aplicar_ao_lado({}, {}) original={:x?} rust={:x?}", l, f, mo, mr));
}
unsafe extern "thiscall" fn comparar_escolher_d(this: usize, l: i8, modo: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u8>(100);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, l, modo), || crate::aleatorio::sem_contar(|| escolher_pela_condicao(this, l, modo)));
    PLACAR.registrar(igual && r == n, || format!("escolher_pela_condicao({}, {}) original={} rust={}", l, modo as u8, r as i8, n as i8));
    r
}

comparar_valor!(comparar_ler_1bc, ler_1bc, 101, () -> u16);
unsafe extern "thiscall" fn comparar_somar_2b(this: usize, v: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8)>(102);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x2b, 1)], || o(this, v), || somar_2b(this, v));
    PLACAR.registrar(igual, || format!("somar_2b({}) original={:x?} rust={:x?}", v, mo, mr));
}
unsafe extern "thiscall" fn comparar_gravar_2b(this: usize, v: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8)>(103);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x2b, 1)], || o(this, v), || gravar_2b(this, v));
    PLACAR.registrar(igual, || format!("gravar_2b({}) original={:x?} rust={:x?}", v, mo, mr));
}
comparar_valor!(comparar_tabela_a_1a_j, tabela_a_1a_do_jogador, 104, () -> u16);
comparar_valor!(comparar_tabela_b_j, tabela_b_do_jogador, 105, () -> u16);
comparar_valor!(comparar_diferenca_j, diferenca_do_jogador, 106, () -> u8);
comparar_valor!(comparar_marcas_rot, marcas_do_jogador_rot, 107, () -> u64);
comparar_valor!(comparar_marca_800000, marca_800000, 108, () -> u8);
comparar_valor!(comparar_perto_proprio, perto_do_proprio_gol, 109, () -> u8);
comparar_valor!(comparar_papel_880, papel_880, 110, () -> u8);
unsafe extern "thiscall" fn comparar_soltar_bola(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(111);
    let m = *((this + 0x19e) as *const usize);
    let reg = [(m + 0x4786, 4), (m + 0x8eae, 1), (m + 0xf582, 4), (m + 0x1a7, 1), (this + 0x29, 2), (this + 0x19c, 2)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || soltar_bola(this));
    PLACAR.registrar(igual, || format!("soltar_bola original={:x?} rust={:x?}", mo, mr));
}
comparar_valor!(comparar_pela_ficha, jogador_pela_ficha, 112, (ficha: usize) -> usize);
unsafe extern "stdcall" fn comparar_byte_do_codigo(c: u32) -> i32 {
    let r = orig::<unsafe extern "stdcall" fn(u32) -> i32>(113)(c);
    let n = byte_do_codigo(c);
    PLACAR.registrar(r == n, || format!("byte_do_codigo({:#x}) original={} rust={}", c, r, n));
    r
}

unsafe extern "thiscall" fn comparar_nota_ajustada(this: usize, base: i8) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8) -> i32>(114);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, base), || crate::aleatorio::sem_contar(|| nota_ajustada(this, base)));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("nota_ajustada({}) original={} rust={}", base, r, n));
    r
}
unsafe extern "thiscall" fn comparar_ajustar_17(this: usize, delta: i8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8) -> u8>(115);
    let mut reg = vec![(this + 0x17, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, delta), || crate::aleatorio::sem_contar(|| ajustar_17(this, delta)));
    PLACAR.registrar(igual && r == n, || format!("ajustar_17({}) original={} {:x?} rust={} {:x?}", delta, r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn comparar_pegar_bola(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(116);
    if gravando_no_topo() {
        chamar_gravando(0x6f0630, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let m = *((this + 0x19e) as *const usize);
    let mut reg = vec![(this, 0x1be)];
    let motor = |mm: usize, reg: &mut Vec<(usize, usize)>| {
        for (a, n) in [(0x1a0usize, 0x30usize), (0x4780, 0x10), (0x8d40, 0x10), (0x8ea0, 0x40), (0xf570, 0x40)] {
            reg.push((mm + a, n));
        }
    };
    if m != 0 {
        motor(m, &mut reg);
        for campo in [0xf582usize, 0xf576] {
            let p = *((m + campo) as *const usize);
            if p != 0 && p != this {
                reg.push((p, 0x40));
                reg.push((p + 0x190, 0x10));
                let mp = *((p + 0x19e) as *const usize);
                if mp != 0 && mp != m {
                    motor(mp, &mut reg);
                }
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || pegar_bola(this));
    PLACAR.registrar(igual, || format!("pegar_bola(lado {}, {} {}) original={:x?} rust={:x?}", lado(this), byte(this, 0x102), byte(this, 0x103), mo, mr));
}

unsafe extern "thiscall" fn comparar_iniciar_rigor(this: usize, motor: u32, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(117);
    let mut reg = vec![(this, 10)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, motor, pessoa), || crate::aleatorio::sem_contar(|| iniciar_rigor(this, motor, pessoa)));
    PLACAR.registrar(igual, || format!("iniciar_rigor original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_lance_valendo(this: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u8>(118);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x475a, 16)], || o(this), || lance_valendo(this));
    PLACAR.registrar(igual && r == n, || format!("lance_valendo original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}
unsafe extern "thiscall" fn comparar_decidir_falta(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8>(119);
    let motor = *(this as *const usize);
    let mut reg = vec![(this + 8, 2), (tipo as usize, 1), (valor as usize, 1), (motor + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, codigo, tipo, valor), || crate::aleatorio::sem_contar(|| decidir_falta(this, j, codigo, tipo, valor)));
    PLACAR.registrar(igual && r == n, || format!("decidir_falta({:#x}) original={} {:x?} rust={} {:x?}", codigo, r, mo, n, mr));
    r
}

unsafe fn regioes_do_cartao(motor: usize, j: usize, tipo: *mut u8, peso: *mut f32) -> Vec<(usize, usize)> {
    let mut reg = vec![(j, 0x40), (tipo as usize, 1), (peso as usize, 4), ((motor as isize + 0xf5c9 + 54 * lado(j) as isize) as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    reg
}
unsafe extern "thiscall" fn comparar_punir_falta(this: usize, j: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32)>(120);
    let reg = regioes_do_cartao(*(this as *const usize), j, tipo, peso);
    let t0 = *tipo;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, tipo, x, sev, flag, peso), || crate::aleatorio::sem_contar(|| punir_falta(this, j, tipo, x, sev, flag, peso)));
    PLACAR.registrar(igual, || format!("punir_falta(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}
unsafe extern "thiscall" fn comparar_punir_lance(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8)>(121);
    let mut reg = regioes_do_cartao(*(this as *const usize), j, tipo, peso);
    reg.push((outro + 0x15, 1));
    let t0 = *tipo;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, outro, tipo, x, sev, flag, peso, expulsar), || crate::aleatorio::sem_contar(|| punir_lance(this, j, outro, tipo, x, sev, flag, peso, expulsar)));
    PLACAR.registrar(igual, || format!("punir_lance(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

// Os invólucros passam pela rotina interna (desviada): comparam-se as mesmas regioes dela.
unsafe extern "thiscall" fn comparar_motor_decidir(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8>(122);
    let mut reg = vec![(this + 0xf5b2 + 8, 2), (tipo as usize, 1), (valor as usize, 1), (this + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, codigo, tipo, valor), || crate::aleatorio::sem_contar(|| motor_decidir_falta(this, j, codigo, tipo, valor)));
    PLACAR.registrar(igual && r == n, || format!("motor_decidir_falta original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}
unsafe extern "thiscall" fn comparar_motor_punir_lance(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8)>(123);
    let mut reg = regioes_do_cartao(this, j, tipo, peso);
    reg.push((outro + 0x15, 1));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, outro, tipo, x, sev, flag, peso, expulsar), || crate::aleatorio::sem_contar(|| motor_punir_lance(this, j, outro, tipo, x, sev, flag, peso, expulsar)));
    PLACAR.registrar(igual, || format!("motor_punir_lance original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_motor_punir_falta(this: usize, j: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32)>(124);
    let reg = regioes_do_cartao(this, j, tipo, peso);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, tipo, x, sev, flag, peso), || crate::aleatorio::sem_contar(|| motor_punir_falta(this, j, tipo, x, sev, flag, peso)));
    PLACAR.registrar(igual, || format!("motor_punir_falta original={:x?} rust={:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_impedimento(this: usize, l: i8, linha: i8, y: i8, passador: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize>(125);
    let motor = *(this as *const usize);
    let mut reg = vec![(this + 8, 2), (motor + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l, linha, y, passador), || crate::aleatorio::sem_contar(|| procurar_impedimento(this, l, linha, y, passador)));
    PLACAR.registrar(igual && r == n, || format!("procurar_impedimento({}, {}, {}, {}) original={:#x} {:x?} rust={:#x} {:x?}", l, linha, y, passador != 0, r, mo, n, mr));
    r
}
unsafe extern "thiscall" fn comparar_motor_impedimento(this: usize, l: i8, linha: i8, y: i8, passador: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize>(126);
    let mut reg = vec![(this + 0xf5b2 + 8, 2), (this + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l, linha, y, passador), || crate::aleatorio::sem_contar(|| motor_procurar_impedimento(this, l, linha, y, passador)));
    PLACAR.registrar(igual && r == n, || format!("motor_procurar_impedimento original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn comparar_cartao_gravidade(this: usize, j: usize, tipo: *mut u8, g: u8, flag: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8)>(127);
    let mut reg = vec![(j, 0x40), (tipo as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, tipo, g, flag), || crate::aleatorio::sem_contar(|| cartao_pela_gravidade(this, j, tipo, g, flag)));
    PLACAR.registrar(igual, || format!("cartao_pela_gravidade(g {}, flag {}) original={:x?} rust={:x?}", g, flag, mo, mr));
}
unsafe extern "thiscall" fn comparar_motor_cartao(this: usize, j: usize, tipo: *mut u8, g: u8, flag: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8)>(128);
    let mut reg = vec![(j, 0x40), (tipo as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, tipo, g, flag), || crate::aleatorio::sem_contar(|| motor_cartao_pela_gravidade(this, j, tipo, g, flag)));
    PLACAR.registrar(igual, || format!("motor_cartao_pela_gravidade original={:x?} rust={:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_escolher_e(this: usize, arg: u32, modo: u32) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(129);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, arg, modo), || crate::aleatorio::sem_contar(|| escolher_por_papel_e(this, arg, modo)));
    PLACAR.registrar(igual && r == n, || format!("escolher_por_papel_e({:#x}, {}) original={} rust={}", arg, modo as u8, r as i8, n as i8));
    r
}

unsafe extern "thiscall" fn comparar_estatistica_posse(this: usize, l: i8, janela: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u8)>(130);
    let reg = [(this, 0x230), (this + 0x8d40, 0x260)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l, janela), || estatistica_de_posse(this, l, janela));
    PLACAR.registrar(igual, || format!("estatistica_de_posse({}, {}) original={:x?} rust={:x?}", l, janela, mo, mr));
}

unsafe extern "thiscall" fn comparar_resumo(this: usize, momento: i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i16)>(131);
    if gravando_no_topo() {
        chamar_gravando(0x6ce460, this, [momento as u16 as u32, 0, 0, 0, 0], || {
            o(this, momento);
            0
        });
        return;
    }
    let mut reg = vec![(this + 0x4796, 40 * 0x1be), (this + 0xf5b0, 0x90)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, momento), || crate::aleatorio::sem_contar(|| resumo_da_partida(this, momento)));
    PLACAR.registrar(igual, || format!("resumo_da_partida({}) original={:x?} rust={:x?}", momento, mo, mr));
}

comparar_valor!(comparar_mais_perto_no_lado, mais_perto_no_lado, 132, (l: i8, a: i8, b: i8, todos: u8) -> usize);

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_defesa(this: usize, a: i8, b: i8, a3: u8, codigo: i8, saida: *mut u8, rebote: *mut u8) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, u8, i8, *mut u8, *mut u8) -> u16>(133);
    let mut reg = vec![(saida as usize, 1), (rebote as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, a, b, a3, codigo, saida, rebote), || crate::aleatorio::sem_contar(|| defesa_do_goleiro(this, a, b, a3, codigo, saida, rebote)));
    PLACAR.registrar(igual && r == n, || format!("defesa_do_goleiro({}, {}, {}, {}) original={} {:x?} rust={} {:x?}", a, b, a3, codigo, r as i16, mo, n as i16, mr));
    r
}

unsafe extern "thiscall" fn comparar_mover_para(this: usize, x: i8, y: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8)>(134);
    if gravando_no_topo() {
        chamar_gravando(0x6f46b0, this, [x as u8 as u32, y as u8 as u32, 0, 0, 0], || {
            o(this, x, y);
            0
        });
        return;
    }
    let motor = *((this + 0x19e) as *const usize);
    let antiga = casa_da_grade(motor, byte(this, 0x102), byte(this, 0x103)) as usize;
    let nova = casa_da_grade(motor, x, y) as usize;
    let mut reg = vec![(this + 0x28, 1), (this + 0x102, 2), (antiga, 90), (nova, 90), (motor + 0x8ea7, 2)];
    let (l, v) = (lado(this), byte(this, 0x28));
    let n = byte(antiga, 0x58 + l as usize);
    if v >= 0 && n > 0 {
        let movido = *((antiga as isize + 4 * (11 * l as isize + n as isize - 1)) as *const usize);
        if movido != 0 {
            reg.push((movido + 0x28, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, x, y), || mover_para(this, x, y));
    PLACAR.registrar(igual, || format!("mover_para({}, {}) original={:x?} rust={:x?}", x, y, mo, mr));
}

unsafe extern "thiscall" fn comparar_escolher_casa(this: usize, j: usize, sx: *mut i8, sy: *mut i8, flags: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32) -> u8>(135);
    let mut reg = vec![(sx as usize, 1), (sy as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, sx, sy, flags), || crate::aleatorio::sem_contar(|| escolher_casa(this, j, sx, sy, flags)));
    PLACAR.registrar(igual && r == n, || format!("escolher_casa(flags {:#x}) original={} {:x?} rust={} {:x?}", flags, r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn comparar_dominio(this: usize, codigo: *mut i8, a: *mut u8, b: *mut u8) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut i8, *mut u8, *mut u8) -> u16>(136);
    let mut reg = vec![(codigo as usize, 1), (a as usize, 1), (b as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let c0 = *codigo;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, codigo, a, b), || crate::aleatorio::sem_contar(|| dominio_do_lance(this, codigo, a, b)));
    PLACAR.registrar(igual && r == n, || format!("dominio_do_lance({}) original={} {:x?} rust={} {:x?}", c0, r as i16, mo, n as i16, mr));
    r
}

// Gravacao das chamadas a 0x6cd330 feitas pela original de `tracar_lance`: argumentos, resposta e
// o estado do sorteio depois de cada uma. A copia Rust recebe as respostas gravadas (sem rodar a
// rotina, que tem efeitos nos jogadores) e o sorteio volta ao estado gravado a cada chamada.
static GRAVANDO_6CD330: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[allow(clippy::type_complexity)]
static FITA_6CD330: std::sync::Mutex<Vec<((i8, usize, u32, u32, u32, u32, i8), u8, crate::comparacao::Memoria)>> = std::sync::Mutex::new(Vec::new());

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_procurar_gravando(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8) -> u8 {
    let r = comparar_procurar(this, lado, grupo, a3, a4, a5, a6, modo);
    if GRAVANDO_6CD330.load(std::sync::atomic::Ordering::Relaxed) {
        let depois = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
        FITA_6CD330.lock().unwrap_or_else(|e| e.into_inner()).push(((lado, grupo, a3, a4, a5, a6, modo), r, depois));
    }
    r
}

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_tracar(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, i8, u32, u32) -> u8>(137);
    FITA_6CD330.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let fita = core::cell::RefCell::new(Vec::new());
    let sequencia = core::cell::Cell::new(true);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || {
            GRAVANDO_6CD330.store(true, Relaxed);
            let r = o(this, x0, y0, x1, y1, codigo, a6, a7);
            GRAVANDO_6CD330.store(false, Relaxed);
            *fita.borrow_mut() = core::mem::take(&mut *FITA_6CD330.lock().unwrap_or_else(|e| e.into_inner()));
            r
        },
        || {
            let gravadas = fita.borrow();
            let mut prox = gravadas.iter();
            let n = crate::aleatorio::sem_contar(|| {
                tracar_com(this, x0, y0, x1, y1, codigo, a6, a7, &mut |l, c, dd, a, t, b| match prox.next() {
                    // o tipo e a6/a7 so tem o byte baixo (o resto e lixo da pilha no original)
                    Some(((gl, gc, gd, ga, gt, gb, 1), gr, depois)) if *gl == l && *gc == c && *gd == dd && *ga as u16 == a as u16 && *gt as u8 == t as u8 && *gb as u8 == b as u8 => {
                        depois.escrever();
                        *gr
                    }
                    _ => {
                        sequencia.set(false);
                        0
                    }
                })
            });
            if prox.next().is_some() {
                sequencia.set(false);
            }
            n
        },
    );
    PLACAR.registrar(igual && r == n && sequencia.get(), || format!("tracar_lance({} {} -> {} {}, cod {}) original={} rust={} seq={} {:x?} {:x?}", x0, y0, x1, y1, codigo, r, n, sequencia.get(), mo, mr));
    r
}

comparar_valor!(comparar_tem_jogo, tem_jogo, 138, (clube: u32, so: u32) -> u32);
unsafe extern "C" fn comparar_venceu(jogo: usize, clube: u32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u8>(139)(jogo, clube);
    let n = venceu_o_jogo(jogo, clube);
    PLACAR.registrar(r == n, || format!("venceu_o_jogo original={} rust={}", r, n));
    r
}

// Porta de 0x6d6f70 (comentarios): na comparacao, grava as chamadas feitas pela original de uma
// rotina portada (com o estado do sorteio depois); a copia Rust confere a sequencia e nao roda o
// comentario de novo (evita eventos em dobro). Fora da gravacao, so passa adiante.
static mut PORTA_6D6F70: usize = 0;
static GRAVANDO_6D6F70: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static FITA_6D6F70: std::sync::Mutex<Vec<(usize, [u32; 7], crate::comparacao::Memoria)>> = std::sync::Mutex::new(Vec::new());
const PROLOGO_6D6F70: Prologo = Prologo { endereco: 0x6d6f70, bytes: &[0x8a, 0x44, 0x24, 0x10, 0x83, 0xec, 0x20] };

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn porta_6d6f70(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) {
    use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
    // 0x6d6f70 pode chamar a si mesma (um comentario puxa outro): so a chamada de fora e gravada
    static PROFUNDIDADE: AtomicU32 = AtomicU32::new(0);
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(PORTA_6D6F70);
    // dentro de uma gravacao da fita (os dois ultimos argumentos sao -1 e 0 nos chamadores portados)
    if gravando() {
        chamar_gravando(0x6d6f70, this, [a1, a2, a3, a4, a5], || {
            f(this, a1, a2, a3, a4, a5, a6, a7);
            0
        });
        return;
    }
    let nivel = PROFUNDIDADE.fetch_add(1, Relaxed);
    // no autoteste o narrador e falso: a chamada so e gravada
    if !SINTETICO.load(Relaxed) {
        f(this, a1, a2, a3, a4, a5, a6, a7);
    }
    PROFUNDIDADE.fetch_sub(1, Relaxed);
    if nivel == 0 && GRAVANDO_6D6F70.load(Relaxed) {
        let depois = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
        FITA_6D6F70.lock().unwrap_or_else(|e| e.into_inner()).push((this, [a1, a2, a3, a4, a5, a6, a7], depois));
    }
}

/// Roda a original gravando os comentarios e a copia Rust reproduzindo-os; devolve (igual, quantos).
unsafe fn comparar_com_comentarios(original: impl FnOnce(), rust: impl FnOnce(ChamarComentario)) -> (bool, usize) {
    comparar_com_comentarios_em(&[], original, rust)
}

/// 0x6d6f70 le o 1o argumento como palavra e o 3o ao 6o como bytes: o resto e lixo da pilha.
const MASCARA_6D6F70: [u32; 7] = [0xffff, u32::MAX, 0xff, 0xff, 0xff, 0xff, u32::MAX];

fn mesmos_argumentos(a: &[u32; 7], b: &[u32; 7]) -> bool {
    (0..7).all(|i| a[i] & MASCARA_6D6F70[i] == b[i] & MASCARA_6D6F70[i])
}

/// Como `comparar_com_comentarios`, comparando tambem as `extra` (alem do estado do sorteio).
unsafe fn comparar_com_comentarios_em(extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce(ChamarComentario)) -> (bool, usize) {
    use std::sync::atomic::Ordering::Relaxed;
    // dentro de outra comparacao que esta gravando (uma rotina portada chama outra que narra, ou uma
    // gravacao da fita esta aberta): so a original roda, e os comentarios dela entram na gravacao de fora
    if GRAVANDO_6D6F70.load(Relaxed) || gravando() {
        original();
        return (true, 0);
    }
    FITA_6D6F70.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let fita = core::cell::RefCell::new(Vec::new());
    let sequencia = core::cell::Cell::new(true);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(extra);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || {
            GRAVANDO_6D6F70.store(true, Relaxed);
            original();
            GRAVANDO_6D6F70.store(false, Relaxed);
            *fita.borrow_mut() = core::mem::take(&mut *FITA_6D6F70.lock().unwrap_or_else(|e| e.into_inner()));
        },
        || {
            let gravadas = fita.borrow();
            let mut prox = gravadas.iter();
            crate::aleatorio::sem_contar(|| {
                rust(&mut |t, a| match prox.next() {
                    Some((gt, ga, depois)) if *gt == t && mesmos_argumentos(ga, &a) => depois.escrever(),
                    _ => sequencia.set(false),
                })
            });
            if prox.next().is_some() {
                sequencia.set(false);
            }
        },
    );
    let n = fita.borrow().len();
    (igual && sequencia.get(), n)
}

unsafe extern "thiscall" fn comparar_comentario_a(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32)>(140);
    let (igual, n) = comparar_com_comentarios(|| o(this, a1, a2, a3, a4, a5), |c| comentario_a_com(this, a1, a2, a3, a4, a5, c));
    PLACAR.registrar(igual, || format!("comentario_a({:#x} {:#x} {:#x} {:#x} {:#x}) {} chamadas", a1, a2, a3, a4, a5, n));
}
unsafe extern "thiscall" fn comparar_comentario_b(this: usize, a1: u32, a2: u32, a3: u32, a4: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(141);
    let (igual, n) = comparar_com_comentarios(|| o(this, a1, a2, a3, a4), |c| comentario_b_com(this, a1, a2, a3, a4, c));
    PLACAR.registrar(igual, || format!("comentario_b({:#x} {:#x} {:#x} {:#x}) {} chamadas", a1, a2, a3, a4, n));
}

unsafe extern "thiscall" fn comparar_bola_fora(this: usize, l: u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8) -> u8>(142);
    // chamada de dentro de outra comparacao que grava os comentarios: so a original (ver porta)
    if GRAVANDO_6D6F70.load(std::sync::atomic::Ordering::Relaxed) {
        return o(this, l);
    }
    if gravando() {
        return chamar_gravando(0x6cd490, this, [l as u32, 0, 0, 0, 0], || o(this, l) as u32) as u8;
    }
    let mut extra = vec![(this + 0x8ea0, 0x20), (this + 0x4780, 0x10), (this + 0xf570, 0x20), (this + 0x1a0, 0x10)];
    let dono = *((this + 0xf582) as *const usize);
    if dono != 0 {
        extra.extend([(dono + 0x29, 2), (dono + 0x19c, 2)]);
    }
    let (r, n) = (core::cell::Cell::new(0u8), core::cell::Cell::new(0u8));
    let (igual, k) = comparar_com_comentarios_em(&extra, || r.set(o(this, l)), |c| n.set(bola_fora_com(this, l, c)));
    PLACAR.registrar(igual && r.get() == n.get(), || format!("bola_fora({}) original={} rust={} ({} comentarios)", l, r.get(), n.get(), k));
    r.get()
}

unsafe extern "thiscall" fn comparar_valor_chute(this: usize, tipo: *mut i8, som: *mut u16) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut i8, *mut u16) -> u16>(143);
    let mut reg = vec![(tipo as usize, 1), (som as usize, 2), (this + 0x39, 2)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let t0 = *tipo;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tipo, som), || crate::aleatorio::sem_contar(|| valor_do_chute(this, tipo, som)));
    PLACAR.registrar(igual && r == n, || format!("valor_do_chute(tipo {}) original={} {:x?} rust={} {:x?}", t0, r as i16, mo, n as i16, mr));
    r
}

unsafe extern "thiscall" fn comparar_forca_do_time(this: usize, l: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8)>(144);
    let mut reg = vec![(this + 0x1d8, 0x20), (0xb67a34, 4)];
    for k in 0..20 {
        let p = jogador_do_bloco(this, l, k);
        reg.extend([(p + 0x35, 2), (p + 0x4d, 4)]);
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l), || crate::aleatorio::sem_contar(|| forca_do_time(this, l)));
    PLACAR.registrar(igual, || format!("forca_do_time({}) original={:x?} rust={:x?}", l, mo, mr));
}

/// O pontape inicial mexe em todo o motor (jogadores, grade, bola): compara o objeto inteiro.
unsafe extern "thiscall" fn comparar_pontape_inicial(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(145);
    let mut reg = vec![(this, 0xf5f8)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| pontape_inicial(this)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("pontape_inicial() diferencas em {:x?}", dif)
    });
}

unsafe extern "thiscall" fn comparar_rolar_bola(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(146);
    let extra = [(this + 0x8ea0, 0x40), (this + 0x4780, 0x10), (this + 0xf570, 0x20), (this + 0x1a0, 0x10)];
    let (igual, k) = comparar_com_comentarios_em(&extra, || o(this), |c| rolar_bola_com(this, c));
    PLACAR.registrar(igual, || format!("rolar_bola ({} comentarios)", k));
}

unsafe extern "thiscall" fn comparar_teste_de_condicao(this: usize, codigo: i8, alvo: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, usize) -> u8>(147);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, codigo, alvo), || crate::aleatorio::sem_contar(|| teste_de_condicao(this, codigo, alvo)));
    PLACAR.registrar(igual && r == n, || format!("teste_de_condicao({}) condicao {} original={} rust={}", codigo, f32_em(this + 0x1a2), r, n));
    r
}

unsafe extern "C" fn comparar_gosta_de(alvo: usize, pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(148)(alvo, pessoa);
    let n = gosta_de(alvo, pessoa);
    PLACAR.registrar(r == n, || format!("gosta_de original={} rust={}", r, n));
    r
}
unsafe extern "C" fn comparar_nao_gosta_de(alvo: usize, pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(149)(alvo, pessoa);
    let n = nao_gosta_de(alvo, pessoa);
    PLACAR.registrar(r == n, || format!("nao_gosta_de original={} rust={}", r, n));
    r
}
unsafe extern "thiscall" fn comparar_marcar_posicao(this: usize, posicao: i8, ficha: *const u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, *const u32) -> u32>(150);
    let reg = [((this as isize + 0x5bf + 4 * posicao as isize) as usize, 4), (this + 0x187f, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, posicao, ficha), || marcar_posicao(this, posicao, ficha));
    PLACAR.registrar(igual && r == n, || format!("marcar_posicao({}) original={:x?} rust={:x?}", posicao, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_posicoes_finas(this: usize, saida: *mut u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u8)>(151);
    let mut reg = vec![(saida as usize, 2 + 6 * 22), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, saida), || crate::aleatorio::sem_contar(|| posicoes_finas(this, saida)));
    PLACAR.registrar(igual, || format!("posicoes_finas original={:x?} rust={:x?}", mo, mr));
}

unsafe extern "C" fn comparar_unidades(n: i32) -> u8 {
    let r = orig::<unsafe extern "C" fn(i32) -> u8>(152)(n);
    let v = unidades(n);
    PLACAR.registrar(r == v, || format!("unidades({}) original={} rust={}", n, r, v));
    r
}
comparar_valor!(comparar_e_amistoso, e_amistoso, 153, (comp: usize) -> u32);
comparar_valor!(comparar_contrato_de, contrato_de, 154, (pessoa: usize) -> usize);
unsafe extern "C" fn comparar_e_selecao(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(155)(clube);
    let v = e_selecao(clube);
    PLACAR.registrar(r == v, || format!("e_selecao original={} rust={}", r, v));
    r
}

comparar_valor!(comparar_tecnico_9a01, tecnico_9a01, 156, (pessoa: usize) -> u32);
comparar_valor!(comparar_tecnico_9a05, tecnico_9a05, 157, (pessoa: usize) -> u32);
unsafe extern "stdcall" fn comparar_bit_da_casa(codigo: i16) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(i16) -> u32>(158)(codigo);
    let v = bit_da_casa(codigo);
    PLACAR.registrar(r == v, || format!("bit_da_casa({:#x}) original={:#x} rust={:#x}", codigo, r, v));
    r
}
unsafe extern "C" fn comparar_separar_mascara(m: u32, setor: *mut u16, lado: *mut u16) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32>(159);
    let reg = [(setor as usize, 2), (lado as usize, 2)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m, setor, lado), || separar_mascara(m, setor, lado));
    PLACAR.registrar(igual && r == v, || format!("separar_mascara({:#x}) original={} {:x?} rust={} {:x?}", m, r, mo, v, mr));
    r
}

comparar_valor!(comparar_tecnico_9ace, tecnico_9ace, 178, (pessoa: usize) -> u32);
comparar_valor!(comparar_lado_sob_controle, lado_sob_controle, 181, (lado: i8) -> u8);
comparar_valor!(comparar_byte_tatico, byte_tatico, 183, (clube: usize, pos: i8, humano: u32) -> u8);
comparar_valor!(comparar_pessoa_da_linha, pessoa_da_linha, 188, (clube: usize, qual: i8, humano: u32) -> usize);
comparar_valor!(comparar_clube_na_competicao, clube_na_competicao, 189, (clube: usize) -> u8);
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_falta_no_motor(
    m: usize, j: usize, outro: usize, tipo: i8, quem: i8, cartao: *mut u8, grav: *mut i8, vantagem: *mut u8,
) -> u8 {
    // a decisao em si (0x6e9820) ja e comparada no proprio desvio; aqui so o repasse
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(193);
    let mut regioes = vec![(m + 0xf5b2 + 8, 2), (cartao as usize, 1), (grav as usize, 1), (vantagem as usize, 1)];
    for p in [j, outro] {
        if p != 0 {
            regioes.push((p, 0x1be));
        }
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, _, _) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(m, j, outro, tipo, quem, cartao, grav, vantagem),
        || crate::aleatorio::sem_contar(|| falta_no_motor(m, j, outro, tipo, quem, cartao, grav, vantagem)),
    );
    PLACAR.registrar(igual && r == v, || format!("falta_no_motor original={} rust={}", r, v));
    r
}
comparar_valor!(comparar_tecnico_9b23, tecnico_9b23, 194, (i: i8, pessoa: usize) -> u8);
comparar_valor!(comparar_contrato_com_clube, contrato_com_clube, 198, (pessoa: usize, clube: usize) -> usize);
unsafe extern "thiscall" fn comparar_entrada_da_lista(this: usize, pessoa: usize, alvo: usize, nao_11: u32, tambem_13: u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize>(203)(this, pessoa, alvo, nao_11, tambem_13);
    let v = entrada_da_lista(this, pessoa, alvo, nao_11, tambem_13);
    PLACAR.registrar(r == v, || format!("entrada_da_lista({}, {}) original={:#x} rust={:#x}", nao_11, tambem_13, r, v));
    r
}
comparar_valor!(comparar_bandeiras_do_contrato, bandeiras_do_contrato, 204, () -> u32);
unsafe extern "thiscall" fn comparar_proximo_da_lista(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(205);
    // o jogo chama bilhoes de vezes (carga de carreira com todas as ligas): o cursor de 4 bytes fica
    // numa variavel, sem as copias alocadas do `lado_a_lado` (que tomavam ~90% do tempo)
    let cursor = (this + 0xa) as *mut u32;
    let antes = cursor.read_unaligned();
    let r = o(this);
    let depois_o = cursor.read_unaligned();
    cursor.write_unaligned(antes);
    let v = proximo_da_lista(this);
    let depois_r = cursor.read_unaligned();
    cursor.write_unaligned(depois_o);
    PLACAR.registrar(depois_o == depois_r && r == v, || format!("proximo_da_lista original={:#x} cursor {:#x} rust={:#x} cursor {:#x}", r, depois_o, v, depois_r));
    r
}
comparar_valor!(comparar_palavra_0c, palavra_0c, 206, (id: i32) -> i32);
unsafe extern "C" fn comparar_arredondar(v: i32) -> i32 {
    let r = orig::<unsafe extern "C" fn(i32) -> i32>(207)(v);
    let n = arredondar_abaixo_de_1000(v);
    PLACAR.registrar(r == n, || format!("arredondar_abaixo_de_1000({:#x}) original={} rust={}", v, r, n));
    r
}
unsafe extern "stdcall" fn comparar_nacao_do_grupo(nacao: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(208)(nacao);
    let n = nacao_do_grupo(nacao);
    PLACAR.registrar(r == n, || format!("nacao_do_grupo({:#x}) original={} rust={}", nacao, r, n));
    r
}
comparar_valor!(comparar_quantos_marcam, quantos_marcam, 209, (alvo: usize) -> u8);
comparar_valor!(comparar_quem_marca, quem_marca, 210, (alvo: usize) -> usize);
unsafe extern "C" fn comparar_competicao_conta(id: i32) -> u32 {
    let r = orig::<unsafe extern "C" fn(i32) -> u32>(211)(id);
    let v = competicao_conta(id);
    PLACAR.registrar(r == v, || format!("competicao_conta({}) original={} rust={}", id, r, v));
    r
}
macro_rules! comparar_ordem {
    ($nome:ident, $rust:ident, $i:expr) => {
        unsafe extern "C" fn $nome(a: *const usize, b: *const usize) -> i32 {
            let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>($i)(a, b);
            let v = $rust(a, b);
            PLACAR.registrar(r == v, || format!("{} original={} rust={}", stringify!($rust), r, v));
            r
        }
    };
}
comparar_ordem!(comparar_ordem_por_5f, ordem_por_5f, 212);
comparar_ordem!(comparar_ordem_por_80, ordem_por_80, 213);
comparar_ordem!(comparar_ordem_por_80_desc, ordem_por_80_desc, 214);
comparar_ordem!(comparar_ordem_por_80_cresc, ordem_por_80_cresc, 215);
comparar_valor!(comparar_nota_do_jogador, nota_do_jogador, 216, () -> i32);
unsafe extern "thiscall" fn comparar_esquecer_pessoa(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(217);
    let mut reg: Vec<(usize, usize)> = Vec::new();
    if pessoa != 0 && *(pessoa as *const i32) < *(0xae2364 as *const i32) {
        for k in 0..16usize {
            let q = *((this + 4 * k) as *const usize);
            if q != 0 {
                reg.push(((*(q as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9)), 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || esquecer_pessoa(this, pessoa));
    PLACAR.registrar(igual, || format!("esquecer_pessoa original={:x?} rust={:x?}", mo, mr));
}
comparar_valor!(comparar_quadro_do_tecnico, quadro_do_tecnico, 218, (tecnico: usize) -> usize);
comparar_valor!(comparar_casa_do_setor_ocupada, casa_do_setor_ocupada, 219, (mascara: u32, jogadores: usize) -> u32);
comparar_valor!(comparar_vaga_da_casa, vaga_da_casa, 220, (codigo: u16) -> u32);
unsafe extern "thiscall" fn comparar_limpar_designados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(221);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x5b4, 11), (this + 0x5bf, 44)], || o(this), || limpar_designados(this));
    PLACAR.registrar(igual && r == v, || format!("limpar_designados original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}
comparar_valor!(comparar_casa_da_vaga, casa_da_vaga, 223, (vaga: i8, segunda: u32) -> u16);
unsafe extern "C" fn comparar_criterio_da_competicao(registro: usize, a: u32, b: u32, tipo: u8) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, u32, u32, u8) -> u8>(224)(registro, a, b, tipo);
    let v = criterio_da_competicao(registro, a, b, tipo);
    PLACAR.registrar(r == v, || format!("criterio_da_competicao({:#x}, {:#x}, {:#x}, {}) original={} rust={}", registro, a, b, tipo, r, v));
    r
}
unsafe extern "C" fn comparar_tipo_nao_11(tipo: i8) -> u32 {
    let r = orig::<unsafe extern "C" fn(i8) -> u32>(225)(tipo);
    let v = tipo_nao_11(tipo);
    PLACAR.registrar(r == v, || format!("tipo_nao_11({}) original={} rust={}", tipo, r, v));
    r
}
unsafe extern "C" fn comparar_tipo_11_a_15(tipo: i8) -> u32 {
    let r = orig::<unsafe extern "C" fn(i8) -> u32>(226)(tipo);
    let v = tipo_11_a_15(tipo);
    PLACAR.registrar(r == v, || format!("tipo_11_a_15({}) original={} rust={}", tipo, r, v));
    r
}
unsafe extern "stdcall" fn comparar_classe_da_selecao(clube: usize, flag: u32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32) -> u8>(227)(clube, flag);
    let v = classe_da_selecao(clube, flag);
    PLACAR.registrar(r == v, || format!("classe_da_selecao({:#x}, {}) original={} rust={}", clube, flag, r, v));
    r
}
comparar_valor!(comparar_ler_marca_da_pessoa, ler_marca_da_pessoa, 228, (pessoa: usize, k: i8) -> u8);
unsafe extern "thiscall" fn comparar_zerar_marca_da_pessoa(this: usize, pessoa: usize, k: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8)>(229);
    let reg = [(marca_da_pessoa(this, pessoa, k) as usize, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, k), || zerar_marca_da_pessoa(this, pessoa, k));
    PLACAR.registrar(igual, || format!("zerar_marca_da_pessoa original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "C" fn comparar_registro_livre() -> usize {
    let o = orig::<unsafe extern "C" fn() -> usize>(230);
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0x9b59b4, 4));
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| registro_livre()));
    PLACAR.registrar(igual && r == v, || format!("registro_livre original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}
unsafe extern "thiscall" fn comparar_ligar_44_da_pessoa(this: usize, pessoa: usize, k: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8)>(231);
    let reg = [(bloco_da_pessoa(this, pessoa, k) + 0x44, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, k), || ligar_44_da_pessoa(this, pessoa, k));
    PLACAR.registrar(igual, || format!("ligar_44_da_pessoa original={:x?} rust={:x?}", mo, mr));
}
comparar_valor!(comparar_byte_44_da_pessoa, byte_44_da_pessoa, 232, (pessoa: usize, k: i8) -> i32);
comparar_valor!(comparar_campo_3a_da_pessoa, campo_3a_da_pessoa, 233, (pessoa: usize, k: i8) -> usize);
unsafe extern "stdcall" fn comparar_classe_pela_selecao(pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u8>(234)(pessoa);
    let v = classe_pela_selecao(pessoa);
    PLACAR.registrar(r == v, || format!("classe_pela_selecao({:#x}) original={} rust={}", pessoa, r, v));
    r
}
unsafe extern "C" fn comparar_global_9b5984() -> u32 {
    let r = orig::<unsafe extern "C" fn() -> u32>(235)();
    let v = global_9b5984();
    PLACAR.registrar(r == v, || format!("global_9b5984 original={:#x} rust={:#x}", r, v));
    r
}
comparar_valor!(comparar_competicao_da_pessoa, competicao_da_pessoa, 236, (pessoa: usize) -> usize);
comparar_valor!(comparar_byte_42_da_selecao, byte_42_da_selecao, 237, (clube: usize, flag: u32) -> u8);
comparar_valor!(comparar_byte_43_da_selecao, byte_43_da_selecao, 238, (clube: usize, flag: u32) -> u8);
unsafe extern "stdcall" fn comparar_selecao_ou_especial(clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(239)(clube);
    let v = selecao_ou_especial(clube);
    PLACAR.registrar(r == v, || format!("selecao_ou_especial({:#x}) original={} rust={}", clube, r, v));
    r
}
unsafe extern "stdcall" fn comparar_nao_gosta_da_selecao(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(240)(pessoa);
    let v = nao_gosta_da_selecao(pessoa);
    PLACAR.registrar(r == v, || format!("nao_gosta_da_selecao({:#x}) original={} rust={}", pessoa, r, v));
    r
}
unsafe extern "thiscall" fn comparar_copiar_data_e_marca(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(241);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 9)], || o(this, origem), || copiar_data_e_marca(this, origem));
    PLACAR.registrar(igual && r == v, || format!("copiar_data_e_marca original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}
unsafe extern "thiscall" fn comparar_atualizar_atributos(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(242);
    // so o registro do jogador muda: a palavra +0x3b e os campos de +0x71 a +0xf8
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 0x3b, 2));
    reg.push((this + 0x71, 0x88));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| atualizar_atributos(this)));
    PLACAR.registrar(igual, || format!("atualizar_atributos original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_desempenho_na_partida(this: usize, marcar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8)>(243);
    // o registro inteiro do jogador (0x6ec010, a propria e 0x6f44a0 gravam nele), o byte do lado em
    // +0xaf do motor e o estado do sorteio
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0x1be));
    let motor = *((this + 0x19e) as *const usize);
    if motor != 0 {
        reg.push(((motor as isize + 0xaf + byte(this, 0x27) as isize) as usize, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, marcar), || crate::aleatorio::sem_contar(|| desempenho_na_partida(this, marcar)));
    PLACAR.registrar(igual, || format!("desempenho_na_partida({}) original={:x?} rust={:x?}", marcar, mo, mr));
}
unsafe extern "thiscall" fn comparar_forca_do_lado(this: usize, lado: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8)>(244);
    let l = lado as isize;
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push(((this as isize + 0x1ce + 2 * l) as usize, 2));
    reg.push(((this as isize + 0xaf + l) as usize, 1));
    reg.push((jogador_do_bloco(this, lado, 0), 20 * 0x1be));
    reg.push((0xae1080, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, lado), || crate::aleatorio::sem_contar(|| forca_do_lado(this, lado)));
    PLACAR.registrar(igual, || format!("forca_do_lado({}) original={:x?} rust={:x?}", lado, mo, mr));
}
comparar_valor!(comparar_bola_no_ataque, bola_no_ataque, 245, () -> u8);
unsafe extern "thiscall" fn comparar_evoluir_momento(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(246);
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0x1be));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| evoluir_momento(this)));
    PLACAR.registrar(igual, || format!("evoluir_momento original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "C" fn comparar_partida_de_interesse(p: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(247)(p);
    let v = partida_de_interesse(p);
    PLACAR.registrar(r == v, || format!("partida_de_interesse({:#x}) original={} rust={}", p, r, v));
    r
}
comparar_valor!(comparar_grupo_decidido, grupo_decidido, 248, (clube: usize, partida: usize) -> u32);
unsafe extern "thiscall" fn comparar_narrar_classificacao(this: usize, lado: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(249);
    if gravando_no_topo() {
        chamar_gravando(0x6be120, this, [lado, 0, 0, 0, 0], || {
            o(this, lado);
            0
        });
        return;
    }
    let (igual, n) = comparar_com_comentarios(|| o(this, lado), |c| narrar_classificacao_com(this, lado, c));
    PLACAR.registrar(igual, || format!("narrar_classificacao({}) {} chamadas", lado as u8, n));
}
unsafe extern "thiscall" fn comparar_interceptar(this: usize, x: i8, y: i8, a3: u32, gol: i8, a5: u32, a6: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, i8, u32, u32) -> u8>(252);
    if gravando_no_topo() {
        return o(this, x, y, a3, gol, a5, a6);
    }
    // o que as jogadas mudam e a copia Rust le: a grade inteira, os bytes de todos os jogadores, os
    // campos da bola no motor e o sorteio
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x215e, 9 * 12 * 90), (this + 0x8ea0, 0x20), (this + 0x4780, 0x10), (this + 0xf570, 0x44)]);
    for l in 0..2i8 {
        for k in 0..20 {
            let j = jogador_do_bloco(this, l, k);
            reg.extend([(j + 0x12, 1), (j + 0x19, 1), (j + 0x2b, 1), (j + 0x198, 2)]);
        }
    }
    let (r, n, igual, sequencia, k) = comparar_com_fita(reg, || o(this, x, y, a3, gol, a5, a6), |chamar| {
        let mut jogada = |p: usize, a: [u32; 4]| chamar(0x70dfd0, p, [a[0], a[1], a[2], a[3], 0], MASCARA_70DFD0) as u8;
        interceptar_com(this, x, y, a3, gol, a5, a6, &mut |lado, grupo, v, a3, a5, a6| procurar_jogada_com(this, lado, grupo, v, a3, 1, a5, a6, &mut jogada))
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("interceptar({}, {} gol {}) original={} rust={} ({} jogadas)", x, y, gol, r, n, k));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_escalar(
    a1: usize, a2: u32, a3: usize, a4: u32, a5: u32, a6: usize, a7: u32, a8: usize, a9: usize, a10: u32, a11: u32, a12: usize, a13: usize, a14: u32, a15: usize,
) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u8>(278);
    let chamar = || o(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15);
    let n1 = (a2 as u8 as i8).max(0) as usize;
    let n = ((a4 as u8 as i8 as i32 + a5 as u8 as i8 as i32).max(0)) as usize;
    if gravando_no_topo() {
        // dentro de uma comparacao com fita: unidade com vetores (a escala le e escreve na pilha de quem chama)
        return chamar_gravando_vetores(
            0x6e2000,
            0,
            [0, a2, a7, a8 as u32, a9 as u32],
            &[(a1, 4 * n1), (a3, 2 * n), (a12, 8 * n), (a13, n), (a15, 2 * n)],
            &[(a6, 4 * n)],
            || chamar() as u32,
        ) as u8;
    }
    // a saida e as datas estaticas que a nota (0x6e2f20) toca
    let reg = [(a6, 4 * n), (0xae2470, 8), (0xae2418, 1), (0xae24c0, 8), (0xae249e, 1)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, chamar, || escalar(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15));
    PLACAR.registrar(igual && r == rr, || format!("escalar(n {} vagas {}+{} modo {:#x}) original={} rust={} {:x?} x {:x?}", a2 as u8 as i8, a4 as u8 as i8, a5 as u8 as i8, a7, r, rr, mo, mr));
    r
}
unsafe extern "thiscall" fn comparar_postura(c: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u8>(279);
    let m = *((c + 0x12) as *const usize);
    // os ativos (motor+4/+5, 0x6cbbb0), o fator dos dois lados, as datas (0x54cdb0) e o ponteiro da
    // tabela que a busca na liga (0x4a4850) ordena
    let reg = [(m + 4, 2), (m + 0x476a, 8), (0xae2470, 8), (0xae2418, 1), (0xadae10, 4)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(c), || postura(c));
    PLACAR.registrar(igual && r == rr, || format!("postura(lado {}) original={} rust={} {:x?} x {:x?}", byte(c, 0x18), r, rr, mo, mr));
    r
}
unsafe extern "thiscall" fn comparar_escolher_formacao(c: usize, fichas: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8>(280);
    let n = (a2 as u8 as i8).max(0) as usize;
    if gravando_no_topo() {
        // dentro de uma comparacao com fita: unidade com vetores (as fichas estao na pilha de quem chama)
        return chamar_gravando_vetores(0x6e0230, c, [0, a2, a3, a4, 0], &[(fichas, 4 * n)], &[], || o(c, fichas, a2, a3, a4) as u32) as u8;
    }
    // o sorteio, o motor, a copia da tatica (+0x2b), as linhas do clube no banco de taticas (a formacao
    // gravada e o que as instrucoes mexem), as datas da nota (0x6e2f20) e o ponteiro da busca na liga
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, 0xf5f8), (*((c + 0x2b) as *const usize), 0x18e3), (0xb67a34, 4), (0xae2470, 8), (0xae2418, 1), (0xae24c0, 8), (0xae249e, 1), (0xadae10, 4)]);
    let taticas = *(0xde1d08 as *const usize);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let clube = *((c + 0x2f) as *const usize);
    for humano in 0..2u32 {
        let l = if clube == 0 { -1 } else { linha(clube, humano) };
        if l != -1 {
            reg.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(c, fichas, a2, a3, a4), || escolher_formacao(c, fichas, a2, a3, a4));
    PLACAR.registrar(igual && r == rr, || {
        // o primeiro byte diferente (as regioes sao grandes)
        let dif = mo.0.iter().zip(mr.0.iter()).find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        });
        format!("escolher_formacao(n {} modo {:#x} {}) original={} rust={} {}", a2 as u8 as i8, a3, a4 as u8, r, rr, dif.unwrap_or_default())
    });
    r
}
unsafe extern "thiscall" fn comparar_instrucoes(c: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(281);
    // o sorteio, o motor (a marcacao individual no bloco tatico), a tatica copiada (+0x2b) e as linhas
    // do clube no banco de taticas
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, 0xf5f8), (*((c + 0x2b) as *const usize), 0x18e3), (0xb67a34, 4)]);
    let taticas = *(0xde1d08 as *const usize);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let clube = *((c + 0x2f) as *const usize);
    for humano in 0..2u32 {
        let l = if clube == 0 { -1 } else { linha(clube, humano) };
        if l != -1 {
            reg.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(c), || instrucoes(c));
    PLACAR.registrar(igual, || {
        let dif = mo.0.iter().zip(mr.0.iter()).find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        });
        format!("instrucoes(lado {}) {}", byte(c, 0x18), dif.unwrap_or_default())
    });
}
unsafe extern "thiscall" fn comparar_substituir(c: usize, goleiro: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(277);
    if gravando_no_topo() {
        chamar_gravando(0x6e1090, c, [goleiro, 0, 0, 0, 0], || {
            o(c, goleiro);
            0
        });
        return;
    }
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, 0xf5f8), (0xb67a34, 4)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(c, goleiro), |chamar| {
        substituir_com(c, goleiro as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0, 0xff, u32::MAX, u32::MAX, u32::MAX],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("substituir(goleiro {}) {} chamadas: {}", goleiro as u8, k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_refazer_time(c: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(276);
    if gravando_no_topo() {
        chamar_gravando(0x6e1aa0, c, [0, 0, 0, 0, 0], || {
            o(c);
            0
        });
        return;
    }
    let m = *((c + 0x12) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, 0xf5f8), (0xb67a34, 4)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(c), |chamar| {
        refazer_time_com(c, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6e0230 => [0, 0xff, u32::MAX, u32::MAX, 0],
                _ => [0, 0xff, u32::MAX, u32::MAX, u32::MAX],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("refazer_time() {} chamadas: {}", k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_jogador_saiu(m: usize, j: usize, ja_fora: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(275);
    if gravando_no_topo() {
        return o(m, j, ja_fora);
    }
    // o motor inteiro (a grade, os jogadores, a comissao +0x9032), o sorteio e o que a forca do lado
    // e o erro tocam
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, 0xf5f8), (0xb67a34, 4), (0xae1080, 4)]);
    // o capitao guardado na linha da tatica (+0x52) dos dois clubes: a substituicao (0x6e1090) o troca
    // antes de a comparada conferir se quem saiu era o capitao
    let taticas = *(0xde1d08 as *const usize);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        for humano in 0..2u32 {
            let l = if clube == 0 { -1 } else { linha(clube, humano) };
            if l != -1 {
                reg.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)) + 0x52, 4));
            }
        }
    }
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, j, ja_fora), |chamar| {
        jogador_saiu_com(m, j, ja_fora as u8, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x897760 { [u32::MAX, 0xff, u32::MAX, 0, 0] } else { [0xff, 0, 0, 0, 0] })
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("jogador_saiu(ja_fora {}) {} chamadas: {}", ja_fora as u8, k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_melhor_da_lista(this: usize, lista: *const usize, n: i8) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const usize, i8) -> usize>(274);
    let reg = [crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1], (0xae2470, 8), (0xae2418, 1)];
    let (r, rr, igual, _, _) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, lista, n), || melhor_da_lista(this, lista, n));
    PLACAR.registrar(igual && r == rr, || format!("melhor_da_lista({}) original={:#x} rust={:#x}", n, r, rr));
    r
}
unsafe extern "thiscall" fn comparar_melhor_avaliado(this: usize) -> i8 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i8>(273);
    let reg = [crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1], (0xae2470, 8), (0xae2418, 1)];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || melhor_avaliado(this));
    PLACAR.registrar(igual && r == n, || format!("melhor_avaliado() original={} rust={}", r, n));
    r
}
unsafe extern "C" fn comparar_avaliar_jogador(pessoa: usize, jogador: usize, clube: usize) -> i16 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> i16>(272);
    // a 0x54cdb0 mexe na data estatica dela: a comparacao restaura
    let reg = [crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1], (0xae2470, 8), (0xae2418, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, jogador, clube), || avaliar_jogador(pessoa, jogador, clube));
    PLACAR.registrar(igual && r == n, || format!("avaliar_jogador() original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}
unsafe extern "thiscall" fn comparar_lesao_na_dividida(m: usize, a: usize, tipo_a: u32, b: usize, tipo_b: u32, direto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32)>(271);
    if gravando_no_topo() {
        return o(m, a, tipo_a, b, tipo_b, direto);
    }
    // o sorteio, os dois jogadores, os registros de lesao deles e o que a 0x6d0980 mexe no motor
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x8ea0, 0x20), (m + 0xf582, 4), (m + 0xf639, 2)]);
    let tabela = *(*(0xb63d28 as *const usize) as *const usize);
    for p in [a, b] {
        if p != 0 {
            reg.push((p, 0x1be));
            let pessoa = *((p + 0x69) as *const usize);
            if pessoa != 0 && *(pessoa as *const i32) >= 0 && *(pessoa as *const i32) < *(0xae2364 as *const i32) {
                reg.push((tabela + *(pessoa as *const i32) as usize * 31, 31));
            }
        }
    }
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, a, tipo_a, b, tipo_b, direto), |chamar| {
        lesao_na_dividida_com(m, a, tipo_a as u16 as i16, b, tipo_b as u16 as i16, direto as u8, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x62ed70 { [u32::MAX, 0, 0, 0, 0] } else { [0xff, 0xff, u32::MAX, u32::MAX, 0] })
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("lesao_na_dividida(lances {}, {} direto {}) {} chamadas: {}", tipo_a as i16, tipo_b as i16, direto as u8, k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_gravidade_da_lesao(lesoes: usize, tipo: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u8>(269)(lesoes, tipo);
    let n = gravidade_da_lesao(lesoes, tipo);
    PLACAR.registrar(r == n, || format!("gravidade_da_lesao({}) original={} rust={}", tipo, r, n));
    r
}
unsafe extern "thiscall" fn comparar_grau_da_lesao(lesoes: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(270)(lesoes, pessoa);
    let n = grau_da_lesao(lesoes, pessoa);
    PLACAR.registrar(r == n, || format!("grau_da_lesao() original={} rust={}", r, n));
    r
}
unsafe extern "C" fn comparar_uniforme(clube: usize, outro: usize, qualquer: u32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(268)(clube, outro, qualquer);
    let n = uniforme(clube, outro, qualquer);
    PLACAR.registrar(r == n, || format!("uniforme(qualquer {}) original={} rust={}", qualquer as u8, r, n));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn comparar_evoluir_jogador(jm: usize, pessoa: usize, clube: usize, rep: usize, comp: usize, nota: f32, media_b: u32, forte: u32) {
    type F = unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32);
    let o = orig::<F>(267);
    let st = *((pessoa + 0x61) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(st + 5, 0xa), (st + 0x45, 1)]);
    if jm != 0 {
        reg.extend([(jm + 0x17, 1), (jm + 0x47, 6)]);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jm, pessoa, clube, rep, comp, nota, media_b, forte), || {
        crate::aleatorio::sem_contar(|| evoluir_jogador(jm, pessoa, clube, rep, comp, nota, media_b, forte))
    });
    PLACAR.registrar(igual, || format!("evoluir_jogador(nota {} jogo {}) original={:x?} rust={:x?}", nota, jm != 0, mo, mr));
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_desafio_adversario(m: usize, atacante: usize, lado: i8, tipo: i8, a: *mut u8, b: *mut u8, nota: *mut i16, lance: *mut i8, arriscado: *mut i8, livre: u8) -> usize {
    type F = unsafe extern "thiscall" fn(usize, usize, i8, i8, *mut u8, *mut u8, *mut i16, *mut i8, *mut i8, u8) -> usize;
    let o = orig::<F>(266);
    // as saidas, o sorteio e o folego (+0x4d) dos 40 jogadores (quem vence cansa)
    let mut reg = [(0usize, 0usize); 48];
    reg[0] = crate::aleatorio::REGIOES_DO_ESTADO[0];
    reg[1] = crate::aleatorio::REGIOES_DO_ESTADO[1];
    reg[2] = (nota as usize, 2);
    reg[3] = (lance as usize, 1);
    reg[4] = (arriscado as usize, 1);
    reg[5] = (a as usize, if a.is_null() { 0 } else { 1 });
    reg[6] = (b as usize, if b.is_null() { 0 } else { 1 });
    for l in 0..2i8 {
        for k in 0..20 {
            reg[7 + l as usize * 20 + k] = (jogador_do_bloco(m, l, k) + 0x4d, 4);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg[..47], || o(m, atacante, lado, tipo, a, b, nota, lance, arriscado, livre), || {
        crate::aleatorio::sem_contar(|| desafio_adversario(m, atacante, lado, tipo, a, b, nota, lance, arriscado, livre))
    });
    PLACAR.registrar(igual && r == n, || format!("desafio_adversario(tipo {:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", tipo, r, n, mo, mr));
    r
}
unsafe extern "thiscall" fn comparar_pressao(m: usize, j: usize, saida: *mut i16, lista: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i16, usize) -> u8>(265);
    if lista == 0 {
        let reg = [(saida as usize, 2)];
        let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(m, j, saida, 0), || pressao(m, j, saida, 0));
        PLACAR.registrar(igual && r == n, || format!("pressao() original={} {:x?} rust={} {:x?}", r, mo, n, mr));
        return r;
    }
    if gravando_no_topo() {
        return o(m, j, saida, lista);
    }
    let (r, n, igual, sequencia, k) = comparar_com_fita(vec![(saida as usize, 2)], || o(m, j, saida, lista), |chamar| {
        pressao_com(m, j, saida, lista, &mut |l, p| {
            chamar(0x68d0d0, l, [p as u32, 0x1be, 0, 0, 0], [u32::MAX; 5]);
        })
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("pressao(lista) original={} rust={} {} copias: {}", r, n, k, detalhe_fita()));
    r
}
unsafe extern "thiscall" fn comparar_periodo_encerrado(m: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u8>(264);
    if gravando_no_topo() {
        return o(m);
    }
    let t = *((m + 0x4792) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0xd8), (t + 0x38, 0x18)]);
    let (r, n, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        periodo_encerrado_com(m, &mut |rotina, this, args| {
            chamar(rotina, this, args, match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6be120 => [u32::MAX, 0, 0, 0, 0],
                _ => [0xffff, 0, 0, 0, 0],
            })
        })
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("periodo_encerrado(periodo {:#x}) original={} rust={} {} chamadas: {}", *((m + 0x8ed4) as *const i16), r, n, k, detalhe_fita()));
    r
}
unsafe extern "thiscall" fn comparar_recuperar_folego(j: usize, teto: u8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, u8)>(263);
    let regioes = [(j + 0x4d, 8)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || f(j, teto), || recuperar_folego(j, teto));
    PLACAR.registrar(igual, || format!("recuperar_folego(teto {}) original={:x?} rust={:x?}", teto, mo, mr));
}
unsafe extern "thiscall" fn comparar_defender(j: usize, forca: i16) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i16) -> u32>(262);
    if gravando_no_topo() {
        return o(j, forca);
    }
    let m = *((j + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0xa0)]);
    let (r, n, igual, sequencia, k) = comparar_com_fita(reg, || o(j, forca), |chamar| {
        defender_com(j, forca, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x6d6f70 { [0xffff, u32::MAX, 0xff, 0xff, 0xff] } else { [0xff, 0xff, 0xffff, 0xff, u32::MAX] })
        })
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("defender(forca {}) original={} rust={} {} chamadas: {}", forca, r, n, k, detalhe_fita()));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_desviar_bola(this: usize, x: i8, y: i8, espalhar: i8, lado: i8, dentro: u8, cansa: u8, chutador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize)>(261);
    if gravando_no_topo() {
        chamar_gravando(0x6cd770, this, [x as u8 as u32, y as u8 as u32, espalhar as u8 as u32 | (lado as u8 as u32) << 8, dentro as u32, chutador as u32], || {
            o(this, x, y, espalhar, lado, dentro, cansa, chutador);
            0
        });
        return;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0xa0)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(this, x, y, espalhar, lado, dentro, cansa, chutador), |chamar| {
        desviar_bola_com(this, x, y, espalhar, lado, dentro, cansa, chutador, &mut |rotina, alvo, args| {
            chamar(rotina, alvo, args, if rotina == 0x6cd490 { [0xff, 0, 0, 0, 0] } else { [u32::MAX, 0, 0, 0, 0] })
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("desviar_bola({}, {} espalhar {} lado {}) {} chamadas: {}", x, y, espalhar, lado, k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_lancar(j: usize, modo: i8, refinar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u8)>(260);
    if gravando_no_topo() {
        return o(j, modo, refinar);
    }
    let m = *((j + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0xa0)]);
    const MASCARAS: [(usize, [u32; 5]); 3] = [
        (0x6fad40, [0xff, 0xff, u32::MAX, u32::MAX, 0xff]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x6cd490, [0xff, 0, 0, 0, 0]),
    ];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, modo, refinar), |chamar| {
        lancar_com(j, modo, refinar, &mut |rotina, this, args| {
            let mascara = MASCARAS.iter().find(|m| m.0 == rotina).map_or([u32::MAX; 5], |m| m.1);
            // a porta da 0x6fad40 grava (x, y, tipo, receptor, coluna); a linha do alvo nao entra
            let args = if rotina == 0x6fad40 { [args[0], args[1], args[2], 0, args[3]] } else { args };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("lancar(modo {} refinar {}) {} chamadas: {}", modo, refinar, k, detalhe_fita()));
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_passar(j: usize, tipo: u8, receptor: usize, a3: u32, a4: u32, a5: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8, usize, u32, u32, u32)>(259);
    if gravando_no_topo() {
        return o(j, tipo, receptor, a3, a4, a5);
    }
    let m = *((j + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0xa0)]);
    const MASCARAS: [(usize, [u32; 5]); 3] = [
        (0x6fad40, [0xff, 0xff, 0xff, u32::MAX, u32::MAX]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x7103e0, [0; 5]),
    ];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, tipo, receptor, a3, a4, a5), |chamar| {
        passar_com(j, tipo, receptor, a4, a5, &mut |rotina, this, args| {
            let mascara = MASCARAS.iter().find(|m| m.0 == rotina).map_or([u32::MAX; 5], |m| m.1);
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("passar(tipo {:#x}) {} chamadas: {}", tipo, k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_decidir_jogador(j: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(258);
    if gravando_no_topo() {
        chamar_gravando(0x7103e0, j, [0; 5], || {
            o(j);
            0
        });
        return;
    }
    let m = *((j + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    // o que a decisao grava e o que ela rele depois de conduzir (dono da bola, +0x8eb2, situacao do lado)
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0x48)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j), |chamar| {
        decidir_jogador_com(j, &mut |rotina, this, args| chamar(rotina, this, args, if rotina == 0x713fc0 { [u32::MAX, 0, 0, 0, 0] } else { [0; 5] }))
    });
    PLACAR.registrar(igual && sequencia, || format!("decidir_jogador acao {:#x} {} chamadas", *((j + 0x198) as *const u16), k));
}
unsafe extern "thiscall" fn comparar_mover_sem_bola(j: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(257);
    if gravando_no_topo() {
        chamar_gravando(0x714d40, j, [0; 5], || {
            o(j);
            0
        });
        return;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    // a copia Rust roda depois da original inteira: as chamadas gravadas (0x6f09f0 decide e move os
    // outros jogadores) mudam o que ela le antes delas; todos os jogadores, a grade e a bola entram
    let m = *((j + 0x19e) as *const usize);
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0x48)]);
    const MASCARAS: [(usize, [u32; 5]); 5] = [
        (0x6f09f0, MASCARA_XY),
        (0x6f46b0, MASCARA_XY),
        (0x6f5b80, [0xff, 0, 0, 0, 0]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x6f0630, [0; 5]),
    ];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j), |chamar| {
        mover_sem_bola_com(j, &mut |rotina, this, args| {
            let mascara = MASCARAS.iter().find(|m| m.0 == rotina).map_or(MASCARA_XY, |m| m.1);
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("mover_sem_bola acao {:#x} {} chamadas", *((j + 0x198) as *const u16), k));
}
unsafe extern "thiscall" fn comparar_decidir_sem_bola(j: usize, a: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(256);
    if gravando_no_topo() {
        chamar_gravando(0x713fc0, j, [a, 0, 0, 0, 0], || {
            o(j, a);
            0
        });
        return;
    }
    // a copia Rust roda depois da original inteira: o que ela (e o que chama sem gravar: 0x6c2be0,
    // 0x6bce80, 0x6c4130...) le antes de uma chamada gravada e que essa chamada muda tem de voltar ao
    // estado de antes. As chamadas gravadas mexem no jogador (casa, direcao), na grade e nas linhas.
    let m = *((j + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    // a 0x6f09f0 gravada move tambem os outros jogadores (decide por eles): todos entram
    reg.extend([(m + 0x1a0, 0x30), (m + 0x215e, 0x8f00 - 0x215e), (m + 0xf56e, 0x48)]);
    // 0x6f09f0 e 0x6f46b0 leem x e y como bytes; 0x6f5b80, so o `direto`
    const MASCARAS: [(usize, [u32; 5]); 3] = [(0x6f09f0, MASCARA_XY), (0x6f46b0, MASCARA_XY), (0x6f5b80, [0xff, 0, 0, 0, 0])];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, a), |chamar| {
        decidir_sem_bola_com(j, &mut |rotina, args| {
            let mascara = MASCARAS.iter().find(|m| m.0 == rotina).map_or(MASCARA_XY, |m| m.1);
            chamar(rotina, j, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia && k == 1, || format!("decidir_sem_bola acao {:#x} {} chamadas: {}", *((j + 0x198) as *const u16), k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_arrumar_bola_parada(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(255);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0x90)]);
    let lado_b = byte(this, 0x8eae);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| arrumar_bola_parada(this)));
    PLACAR.registrar(igual, || format!("arrumar_bola_parada(lado {})", lado_b));
}
unsafe extern "thiscall" fn comparar_arrumar_saida(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(254);
    // tudo o que a arrumacao (e o que ela chama) grava: o motor de 0x1a0, a grade, os 40 jogadores, a
    // bola e as posses (0x215e..0x8f00) e os ponteiros de 0xf56e
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0x48)]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| arrumar_saida(this)));
    PLACAR.registrar(igual, || format!("arrumar_saida(lado {})", byte(this, 0x8eae)));
}
unsafe extern "thiscall" fn comparar_avancar_linha(this: usize, direto: u8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u8) -> u32>(253);
    if gravando_no_topo() {
        return chamar_gravando(0x6f5b80, this, [direto as u32, 0, 0, 0, 0], || o(this, direto));
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 0x102, 2));
    let (r, _, igual, sequencia, k) = comparar_com_fita(reg, || o(this, direto), |chamar| {
        avancar_linha_com(this, direto, &mut |rotina, a| chamar(rotina, this, a, MASCARA_XY))
    });
    // a resposta e o `eax` da chamada (o movimento nao devolve nada): nao entra na comparacao
    PLACAR.registrar(igual && sequencia && k == 1, || format!("avancar_linha({}) {} chamadas", direto, k));
    r
}
comparar_valor!(comparar_jogadores_no_trajeto, jogadores_no_trajeto, 251, (x1: i8, y1: i8, x2: i8, y2: i8) -> u8);
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_procurar_jogada(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, modo: u8, a6: u32, a7: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u8, u32, u32) -> u8>(250);
    if gravando_no_topo() {
        return o(this, lado, grupo, a3, a4, modo, a6, a7);
    }
    // o que a jogada muda e a copia Rust le: a lista e a quantidade do lado, os bytes de cada
    // jogador, o jogador marcado no motor e o sorteio
    let base = (grupo as isize + lado as isize) as usize;
    let lista = (grupo as isize + lado as isize * 44) as usize;
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0xf5ae, 4), (base + 0x58, 1), (lista, 44)]);
    for k in 0..byte(base, 0x58).clamp(0, 11) as usize {
        let j = *((lista + 4 * k) as *const usize);
        reg.extend([(j + 0x12, 1), (j + 0x19, 1), (j + 0x2b, 1)]);
    }
    let (r, n, igual, sequencia, k) = comparar_com_fita(
        reg,
        || o(this, lado, grupo, a3, a4, modo, a6, a7),
        |chamar| procurar_jogada_com(this, lado, grupo, a3, a4, modo, a6, a7, &mut |p, a| chamar(0x70dfd0, p, [a[0], a[1], a[2], a[3], 0], MASCARA_70DFD0) as u8),
    );
    PLACAR.registrar(igual && sequencia && r == n, || format!("procurar_jogada({} modo {}) original={} rust={} ({} jogadas)", lado, modo, r, n, k));
    r
}
unsafe extern "thiscall" fn comparar_mascara_da_formacao(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(222);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x187b, 4)], || o(this), || mascara_da_formacao(this));
    PLACAR.registrar(igual && r == v, || format!("mascara_da_formacao original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}
unsafe extern "C" fn comparar_chance_de_conhecer(pessoa: usize, nivel: i8) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, i8) -> i32>(200)(pessoa, nivel);
    let v = chance_de_conhecer(pessoa, nivel);
    PLACAR.registrar(r == v, || format!("chance_de_conhecer({}, {}) original={} rust={}", *((pessoa + 4) as *const i16), nivel, r, v));
    r
}
unsafe extern "thiscall" fn comparar_conhecer_pessoa(this: usize, pessoa: usize, fonte: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(201);
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if pessoa != 0 && *(pessoa as *const i32) < *(0xae2364 as *const i32) {
        reg.push(((*(this as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9)), 9));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, fonte), || crate::aleatorio::sem_contar(|| conhecer_pessoa(this, pessoa, fonte)));
    PLACAR.registrar(igual, || format!("conhecer_pessoa original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "stdcall" fn comparar_nevoa_apos_partida(partida: usize, dados: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize)>(202);
    // os registros que a rotina pode mexer (mesmos testes, sem o 0x32 das reservas: sobra nao atrapalha)
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let n = *(0xae2364 as *const i32);
    let mut reg: Vec<(usize, usize)> = Vec::new();
    for lado in 0..2usize {
        let clube = *((partida + 0x20 - 4 * lado) as *const usize);
        if humano(clube, 0, 1, 0, 0) == 0 || *((clube + 0xcf) as *const usize) == 0 {
            continue;
        }
        let q = *((clube + 0xcf) as *const usize);
        let obj = *((*(0xae2a60 as *const usize)).wrapping_add((n.wrapping_sub(*(q as *const i32)) as usize).wrapping_mul(4)).wrapping_sub(4) as *const usize);
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let i = *((dados + lado * 0x625 + 0x190 + vaga * 0x49) as *const i32);
            let p = (*(0xae23bc as *const usize)).wrapping_add((i as usize).wrapping_mul(0x6e));
            if i != -1 && p != 0 && *(p as *const i32) < n {
                reg.push(((*(obj as *const usize)).wrapping_add((*(p as *const i32) as usize).wrapping_mul(9)), 9));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(partida, dados), || nevoa_apos_partida(partida, dados));
    PLACAR.registrar(igual, || format!("nevoa_apos_partida original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "C" fn comparar_nivel_do_contrato(pessoa: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(199)(pessoa, clube);
    let v = nivel_do_contrato(pessoa, clube);
    PLACAR.registrar(r == v, || format!("nivel_do_contrato original={:#x} rust={:#x}", r, v));
    r
}
unsafe extern "C" fn comparar_clube_parceiro(clube: usize, eh_a: *mut u32, ativo: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, *mut u32, u32) -> usize>(197);
    let mut reg: Vec<(usize, usize)> = vec![(clube + 0x244, 1)];
    if !eh_a.is_null() {
        reg.push((eh_a as usize, 4));
    }
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(clube, eh_a, ativo), || clube_parceiro(clube, eh_a, ativo));
    PLACAR.registrar(igual && r == v, || format!("clube_parceiro original={:x} {:x?} rust={:x} {:x?}", r, mo, v, mr));
    r
}
unsafe extern "C" fn comparar_gosta_do_clube(pessoa: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(196)(pessoa, clube);
    let n = gosta_do_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("gosta_do_clube original={} rust={}", r, n));
    r
}
unsafe extern "C" fn comparar_nao_gosta_do_clube(pessoa: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(195)(pessoa, clube);
    let n = nao_gosta_do_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("nao_gosta_do_clube original={} rust={}", r, n));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_escolher_companheiro(
    m: usize, bx: i8, by: i8, tipo: *mut i8, moveu: *mut u8, x: *mut i8, y: *mut i8, melhor: *mut i32, pedido: i8,
) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, *mut i8, *mut u8, *mut i8, *mut i8, *mut i32, i8) -> usize>(192);
    let mut regioes = vec![(tipo as usize, 1), (moveu as usize, 1), (x as usize, 1), (y as usize, 1), (melhor as usize, 4), (m + 0x8eb2, 1)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(m, bx, by, tipo, moveu, x, y, melhor, pedido),
        || crate::aleatorio::sem_contar(|| escolher_companheiro(m, bx, by, tipo, moveu, x, y, melhor, pedido)),
    );
    PLACAR.registrar(igual && r == v, || format!("escolher_companheiro(pedido {:#x}) original={:x} {:x?} rust={:x} {:x?}", pedido, r, mo, v, mr));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_decisao_da_falta(
    this: usize, j: usize, outro: usize, tipo: i8, quem: i8, cartao: *mut u8, grav: *mut i8, vantagem: *mut u8,
) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(191);
    let mut regioes = vec![(this + 8, 2), (cartao as usize, 1), (grav as usize, 1), (vantagem as usize, 1)];
    for p in [j, outro] {
        if p != 0 {
            regioes.push((p, 0x1be));
        }
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, j, outro, tipo, quem, cartao, grav, vantagem),
        || crate::aleatorio::sem_contar(|| decisao_da_falta(this, j, outro, tipo, quem, cartao, grav, vantagem)),
    );
    PLACAR.registrar(igual && r == v, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("decisao_da_falta(tipo {:#x}) original={} rust={} diferencas em {:x?}", tipo, r, v, dif)
    });
    r
}
unsafe extern "stdcall" fn comparar_clube_especial(clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(190)(clube);
    let v = clube_especial(clube);
    PLACAR.registrar(r == v, || format!("clube_especial({:#x}) original={} rust={}", clube, r, v));
    r
}
unsafe extern "thiscall" fn comparar_vaga_marcada(this: usize, clube: usize, humano: u32, menos_um: u32) -> i8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(186)(this, clube, humano, menos_um);
    let v = vaga_marcada(this, clube, humano, menos_um);
    PLACAR.registrar(r == v, || format!("vaga_marcada original={} rust={}", r, v));
    r
}
unsafe extern "thiscall" fn comparar_marcar_vaga(this: usize, clube: usize, vaga: i8, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(187);
    if gravando_no_topo() {
        return chamar_gravando(0x897760, this, [clube as u32, vaga as u8 as u32, humano, 0, 0], || o(this, clube, vaga, humano));
    }
    let tabela = *((this + 0x601) as *const usize);
    // a linha pode ser qualquer uma da tabela: compara as 0x52..0x56 de todas as linhas possiveis
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha(clube, humano);
    let reg: Vec<(usize, usize)> = if l == -1 || clube == 0 { vec![] } else { vec![(tabela.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x52, 4)] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, vaga, humano), || marcar_vaga(this, clube, vaga, humano));
    PLACAR.registrar(igual && r == v, || format!("marcar_vaga original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}
unsafe extern "thiscall" fn comparar_ordem_dos_jogadores(m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(185);
    let mut regioes = vec![(m + 0x8eb8, 1), (m + 0x8f7e, 4 * 40), (0xb67a64, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(m), || crate::aleatorio::sem_contar(|| ordem_dos_jogadores(m)));
    PLACAR.registrar(igual, || format!("ordem_dos_jogadores original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn comparar_ordem_dos_penaltis(m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(184);
    let mut regioes = vec![(m, 0xf5f8)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(m), || crate::aleatorio::sem_contar(|| ordem_dos_penaltis(m)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("ordem_dos_penaltis diferencas em {:x?}", dif)
    });
}
unsafe extern "stdcall" fn comparar_linha_tatica(clube: usize, humano: u32) -> i32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32) -> i32>(182)(clube, humano);
    let v = linha_tatica(clube, humano);
    PLACAR.registrar(r == v, || format!("linha_tatica({:#x}, {}) original={} rust={}", clube, humano, r, v));
    r
}
unsafe extern "C" fn comparar_clube_humano(clube: usize, a: u32, b: u8, pessoa: usize, c: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(180)(clube, a, b, pessoa, c);
    let v = clube_humano(clube, a, b, pessoa, c);
    PLACAR.registrar(r == v, || format!("clube_humano({:#x}, {}, {}, {:#x}, {}) original={} rust={}", clube, a, b, pessoa, c, r, v));
    r
}
unsafe extern "thiscall" fn comparar_tecnico_9a06(this: usize, pessoa: usize, saida: *mut u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8>(179);
    let reg: Vec<(usize, usize)> = if saida.is_null() { vec![] } else { vec![(saida as usize, 4)] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, saida), || tecnico_9a06(this, pessoa, saida));
    PLACAR.registrar(igual && r == v, || format!("tecnico_9a06 original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_quem_marcar(m: usize, j: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(177);
    let (r, v, igual, _, _) = crate::comparacao::lado_a_lado(
        &crate::aleatorio::REGIOES_DO_ESTADO,
        || o(m, j),
        || crate::aleatorio::sem_contar(|| quem_marcar(m, j)),
    );
    PLACAR.registrar(igual && r == v, || format!("quem_marcar original={:x} rust={:x}", r, v));
    r
}

unsafe extern "C" fn comparar_selecao_principal(nacao: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize) -> usize>(175)(nacao);
    let v = selecao_principal(nacao);
    PLACAR.registrar(r == v, || format!("selecao_principal original={:x} rust={:x}", r, v));
    r
}

unsafe extern "C" fn comparar_selecao_b(nacao: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize) -> usize>(176)(nacao);
    let v = selecao_b(nacao);
    PLACAR.registrar(r == v, || format!("selecao_b original={:x} rust={:x}", r, v));
    r
}

unsafe extern "C" fn comparar_e_segunda_selecao(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(160)(clube);
    let v = e_segunda_selecao(clube);
    PLACAR.registrar(r == v, || format!("e_segunda_selecao original={} rust={}", r, v));
    r
}
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_placar_agregado(this: usize, l: i8, casa: *mut i8, fora: *mut i8, cl: *mut u8, ida: *mut u8, f3: *mut u8, bonus: u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, *mut i8, *mut i8, *mut u8, *mut u8, *mut u8, u8) -> u8>(161);
    let reg = [(casa as usize, 1), (fora as usize, 1), (cl as usize, 1), (ida as usize, 1), (f3 as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, l, casa, fora, cl, ida, f3, bonus), || placar_agregado(this, l, casa, fora, cl, ida, f3, bonus));
    PLACAR.registrar(igual && r == n, || format!("placar_agregado({}, {}) original={} {:x?} rust={} {:x?}", l, bonus, r, mo, n, mr));
    r
}

comparar_valor!(comparar_marcador_direto, marcador_direto, 162, (l: i8, k: i8) -> u8);

unsafe extern "thiscall" fn comparar_tirar_de_campo(this: usize, j: usize, modo: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8)>(163);
    let reg = [(this, 0xf5f8), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, modo), || tirar_de_campo(this, j, modo));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("tirar_de_campo({}) diferencas em {:x?}", modo, dif)
    });
}

/// Diagnostico: rotinas que ficam no original (para isolar uma diferenca de carreira sem mexer no
/// ambiente do jogo, que desloca a heap). Vazio no uso normal.
const SEM_ROTINAS: &[&str] = &[];

unsafe extern "thiscall" fn comparar_marcacao_individual(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(164);
    let reg = [(this, 0xf5f8)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || marcacao_individual(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("marcacao_individual diferencas em {:x?}", dif)
    });
}

unsafe extern "C" fn comparar_nova_reputacao(this: usize, lado: i8, valor: f32, alt: u8, bonus: u8) {
    let o = orig::<unsafe extern "C" fn(usize, i8, f32, u8, u8)>(165);
    let reg = [(this, 0x200)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, lado, valor, alt, bonus), || nova_reputacao(this, lado, valor, alt, bonus));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        format!("nova_reputacao lado {} valor {} alt {} bonus {}: {:?} x {:?}", lado, valor, alt, bonus, &a[0x1d6..0x1da], &b[0x1d6..0x1da])
    });
}

unsafe extern "C" fn comparar_reputacao_do_jogador(reg: usize, meu: usize, outro: usize, partida: usize, valor: f32, alt: u8, bonus: u8) {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, usize, f32, u8, u8)>(166);
    let regioes = [(reg, 6)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(reg, meu, outro, partida, valor, alt, bonus), || reputacao_do_jogador(reg, meu, outro, partida, valor, alt, bonus));
    PLACAR.registrar(igual, || format!("reputacao_do_jogador valor {} alt {} bonus {}: {:?} x {:?}", valor, alt, bonus, mo.0[0].1, mr.0[0].1));
}

unsafe extern "thiscall" fn comparar_escolher_cobrador(this: usize, lado: i8, tipo: i8, excluidos: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize>(167);
    let mut regioes = vec![(this, 0xf5f8)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, lado, tipo, excluidos),
        || crate::aleatorio::sem_contar(|| escolher_cobrador(this, lado, tipo, excluidos)),
    );
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("escolher_cobrador(lado {}, tipo {}) original={:x} rust={:x}, diferencas em {:x?}", lado, tipo, r, n, dif)
    });
    r
}

/// Comparacao das bolas paradas da 57a leva: motor inteiro e sorteio.
unsafe fn comparar_reinicio(nome: &str, this: usize, o: unsafe extern "thiscall" fn(usize), r: unsafe extern "thiscall" fn(usize)) {
    let mut regioes = vec![(this, 0xf5f8), (0xb67a34, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || crate::aleatorio::sem_contar(|| r(this)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("{} diferencas em {:x?}", nome, dif)
    });
}
unsafe extern "thiscall" fn comparar_lateral(this: usize) {
    comparar_reinicio("lateral", this, orig::<unsafe extern "thiscall" fn(usize)>(168), lateral);
}
unsafe extern "thiscall" fn comparar_escanteio(this: usize) {
    comparar_reinicio("escanteio", this, orig::<unsafe extern "thiscall" fn(usize)>(169), escanteio);
}
unsafe extern "thiscall" fn comparar_penalti(this: usize) {
    comparar_reinicio("penalti", this, orig::<unsafe extern "thiscall" fn(usize)>(170), penalti);
}

unsafe extern "thiscall" fn comparar_gravar_casa(this: usize, posicao: i8, mascara: u64) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u64) -> u32>(171);
    let regioes = [(this + 0x187f, 0x64), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, posicao, mascara), || gravar_casa(this, posicao, mascara));
    PLACAR.registrar(igual && r == n, || format!("gravar_casa({}, {:x}) original={} rust={} {:x?} x {:x?}", posicao, mascara, r, n, mo, mr));
    r
}
unsafe extern "thiscall" fn comparar_marcar_craques(this: usize, lado: i8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8)>(172);
    let regioes = [(this, 0xf5f8), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, lado), || marcar_craques(this, lado));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("marcar_craques({}) diferencas em {:x?}", lado, dif)
    });
}

unsafe extern "thiscall" fn comparar_designado_da_posicao(this: usize, posicao: i8) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> usize>(173)(this, posicao);
    let n = designado_da_posicao(this, posicao);
    PLACAR.registrar(r == n, || format!("designado_da_posicao({}) original={:x} rust={:x}", posicao, r, n));
    r
}
unsafe extern "thiscall" fn comparar_marcado_designado(this: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> usize>(174)(this);
    let n = marcado_designado(this);
    PLACAR.registrar(r == n, || format!("marcado_designado original={:x} rust={:x}", r, n));
    r
}

pub unsafe fn instalar(comparando: bool) {
    if comparando {
        if let Some(t) = desvios::desviar("motor_porta_6f5c30 (gravacao)", &PROLOGO_6F5C30, porta_6f5c30 as *const () as usize) {
            PORTA_6F5C30 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_6d6f70 (gravacao)", &PROLOGO_6D6F70, porta_6d6f70 as *const () as usize) {
            PORTA_6D6F70 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_70dfd0 (gravacao)", &PROLOGO_70DFD0, porta_70dfd0 as *const () as usize) {
            PORTA_70DFD0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_6f09f0 (gravacao)", &PROLOGO_6F09F0, porta_6f09f0 as *const () as usize) {
            PORTA_6F09F0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_7109f0 (gravacao)", &PROLOGO_7109F0, porta_7109f0 as *const () as usize) {
            PORTA_7109F0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_6fad40 (gravacao)", &PROLOGO_6FAD40, porta_6fad40 as *const () as usize) {
            PORTA_6FAD40 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_70a910 (gravacao)", &PROLOGO_70A910, porta_70a910 as *const () as usize) {
            PORTA_70A910 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_70c040 (gravacao)", &PROLOGO_70C040, porta_70c040 as *const () as usize) {
            PORTA_70C040 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_68d0d0 (gravacao)", &PROLOGO_68D0D0, porta_68d0d0 as *const () as usize) {
            PORTA_68D0D0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_62ed70 (gravacao)", &PROLOGO_62ED70, porta_62ed70 as *const () as usize) {
            PORTA_62ED70 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_6d0980 (gravacao)", &PROLOGO_6D0980, porta_6d0980 as *const () as usize) {
            PORTA_6D0980 = t;
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
        direcao as *const () as usize,
        zerar_guardado as *const () as usize,
        x_fora_do_meio as *const () as usize,
        e_o_guardado as *const () as usize,
        lado_do_lance as *const () as usize,
        jogador_no_bloco as *const () as usize,
        tabela_a_1a as *const () as usize,
        momento_marcado as *const () as usize,
        estado_do_lado as *const () as usize,
        estado_do_adversario as *const () as usize,
        gravar_19c as *const () as usize,
        codigo_de_bola_parada as *const () as usize,
        perto_do_gol as *const () as usize,
        limpar_lance as *const () as usize,
        somar_disposicao as *const () as usize,
        repartir_d as *const () as usize,
        repartir_e as *const () as usize,
        repartir_f as *const () as usize,
        registrar_lance as *const () as usize,
        remover_da_lista as *const () as usize,
        entrar_em_campo as *const () as usize,
        em_perigo as *const () as usize,
        conferir_lance as *const () as usize,
        caminho_livre_ate as *const () as usize,
        alcanca_o_ponto as *const () as usize,
        escolher_destaque as *const () as usize,
        escolher_por_papel_b as *const () as usize,
        casa_da_formacao as *const () as usize,
        calcular_percentuais as *const () as usize,
        escolher_por_papel_c as *const () as usize,
        aplicar_fator as *const () as usize,
        valor_pela_condicao as *const () as usize,
        aplicar_ao_time as *const () as usize,
        aplicar_ao_lado as *const () as usize,
        escolher_pela_condicao as *const () as usize,
        ler_1bc as *const () as usize,
        somar_2b as *const () as usize,
        gravar_2b as *const () as usize,
        tabela_a_1a_do_jogador as *const () as usize,
        tabela_b_do_jogador as *const () as usize,
        diferenca_do_jogador as *const () as usize,
        marcas_do_jogador_rot as *const () as usize,
        marca_800000 as *const () as usize,
        perto_do_proprio_gol as *const () as usize,
        papel_880 as *const () as usize,
        soltar_bola as *const () as usize,
        jogador_pela_ficha as *const () as usize,
        byte_do_codigo as *const () as usize,
        nota_ajustada as *const () as usize,
        ajustar_17 as *const () as usize,
        pegar_bola as *const () as usize,
        iniciar_rigor as *const () as usize,
        lance_valendo as *const () as usize,
        decidir_falta as *const () as usize,
        punir_falta as *const () as usize,
        punir_lance as *const () as usize,
        motor_decidir_falta as *const () as usize,
        motor_punir_lance as *const () as usize,
        motor_punir_falta as *const () as usize,
        procurar_impedimento as *const () as usize,
        motor_procurar_impedimento as *const () as usize,
        cartao_pela_gravidade as *const () as usize,
        motor_cartao_pela_gravidade as *const () as usize,
        escolher_por_papel_e as *const () as usize,
        estatistica_de_posse as *const () as usize,
        resumo_da_partida as *const () as usize,
        mais_perto_no_lado as *const () as usize,
        defesa_do_goleiro as *const () as usize,
        mover_para as *const () as usize,
        escolher_casa as *const () as usize,
        dominio_do_lance as *const () as usize,
        tracar_lance as *const () as usize,
        tem_jogo as *const () as usize,
        venceu_o_jogo as *const () as usize,
        comentario_a as *const () as usize,
        comentario_b as *const () as usize,
        bola_fora as *const () as usize,
        valor_do_chute as *const () as usize,
        forca_do_time as *const () as usize,
        pontape_inicial as *const () as usize,
        rolar_bola as *const () as usize,
        teste_de_condicao as *const () as usize,
        gosta_de as *const () as usize,
        nao_gosta_de as *const () as usize,
        marcar_posicao as *const () as usize,
        posicoes_finas as *const () as usize,
        unidades as *const () as usize,
        e_amistoso as *const () as usize,
        contrato_de as *const () as usize,
        e_selecao as *const () as usize,
        tecnico_9a01 as *const () as usize,
        tecnico_9a05 as *const () as usize,
        bit_da_casa as *const () as usize,
        separar_mascara as *const () as usize,
        e_segunda_selecao as *const () as usize,
        placar_agregado as *const () as usize,
        marcador_direto as *const () as usize,
        tirar_de_campo as *const () as usize,
        marcacao_individual as *const () as usize,
        nova_reputacao as *const () as usize,
        reputacao_do_jogador as *const () as usize,
        escolher_cobrador as *const () as usize,
        lateral as *const () as usize,
        escanteio as *const () as usize,
        penalti as *const () as usize,
        gravar_casa as *const () as usize,
        marcar_craques as *const () as usize,
        designado_da_posicao as *const () as usize,
        marcado_designado as *const () as usize,
        selecao_principal as *const () as usize,
        selecao_b as *const () as usize,
        quem_marcar as *const () as usize,
        tecnico_9ace as *const () as usize,
        tecnico_9a06 as *const () as usize,
        clube_humano as *const () as usize,
        lado_sob_controle as *const () as usize,
        linha_tatica as *const () as usize,
        byte_tatico as *const () as usize,
        ordem_dos_penaltis as *const () as usize,
        ordem_dos_jogadores as *const () as usize,
        vaga_marcada as *const () as usize,
        marcar_vaga as *const () as usize,
        pessoa_da_linha as *const () as usize,
        clube_na_competicao as *const () as usize,
        clube_especial as *const () as usize,
        decisao_da_falta as *const () as usize,
        escolher_companheiro as *const () as usize,
        falta_no_motor as *const () as usize,
        tecnico_9b23 as *const () as usize,
        nao_gosta_do_clube as *const () as usize,
        gosta_do_clube as *const () as usize,
        clube_parceiro as *const () as usize,
        contrato_com_clube as *const () as usize,
        nivel_do_contrato as *const () as usize,
        chance_de_conhecer as *const () as usize,
        conhecer_pessoa as *const () as usize,
        nevoa_apos_partida as *const () as usize,
        entrada_da_lista as *const () as usize,
        bandeiras_do_contrato as *const () as usize,
        proximo_da_lista as *const () as usize,
        palavra_0c as *const () as usize,
        arredondar_abaixo_de_1000 as *const () as usize,
        nacao_do_grupo as *const () as usize,
        quantos_marcam as *const () as usize,
        quem_marca as *const () as usize,
        competicao_conta as *const () as usize,
        ordem_por_5f as *const () as usize,
        ordem_por_80 as *const () as usize,
        ordem_por_80_desc as *const () as usize,
        ordem_por_80_cresc as *const () as usize,
        nota_do_jogador as *const () as usize,
        esquecer_pessoa as *const () as usize,
        quadro_do_tecnico as *const () as usize,
        casa_do_setor_ocupada as *const () as usize,
        vaga_da_casa as *const () as usize,
        limpar_designados as *const () as usize,
        mascara_da_formacao as *const () as usize,
        casa_da_vaga as *const () as usize,
        criterio_da_competicao as *const () as usize,
        tipo_nao_11 as *const () as usize,
        tipo_11_a_15 as *const () as usize,
        classe_da_selecao as *const () as usize,
        ler_marca_da_pessoa as *const () as usize,
        zerar_marca_da_pessoa as *const () as usize,
        registro_livre as *const () as usize,
        ligar_44_da_pessoa as *const () as usize,
        byte_44_da_pessoa as *const () as usize,
        campo_3a_da_pessoa as *const () as usize,
        classe_pela_selecao as *const () as usize,
        global_9b5984 as *const () as usize,
        competicao_da_pessoa as *const () as usize,
        byte_42_da_selecao as *const () as usize,
        byte_43_da_selecao as *const () as usize,
        selecao_ou_especial as *const () as usize,
        nao_gosta_da_selecao as *const () as usize,
        copiar_data_e_marca as *const () as usize,
        atualizar_atributos as *const () as usize,
        desempenho_na_partida as *const () as usize,
        forca_do_lado as *const () as usize,
        bola_no_ataque as *const () as usize,
        evoluir_momento as *const () as usize,
        partida_de_interesse as *const () as usize,
        grupo_decidido as *const () as usize,
        narrar_classificacao as *const () as usize,
        procurar_jogada as *const () as usize,
        jogadores_no_trajeto as *const () as usize,
        interceptar as *const () as usize,
        avancar_linha as *const () as usize,
        arrumar_saida as *const () as usize,
        arrumar_bola_parada as *const () as usize,
        decidir_sem_bola as *const () as usize,
        mover_sem_bola as *const () as usize,
        decidir_jogador as *const () as usize,
        passar as *const () as usize,
        lancar as *const () as usize,
        desviar_bola as *const () as usize,
        defender as *const () as usize,
        recuperar_folego as *const () as usize,
        periodo_encerrado as *const () as usize,
        pressao as *const () as usize,
        desafio_adversario as *const () as usize,
        evoluir_jogador as *const () as usize,
        uniforme as *const () as usize,
        gravidade_da_lesao as *const () as usize,
        grau_da_lesao as *const () as usize,
        lesao_na_dividida as *const () as usize,
        avaliar_jogador as *const () as usize,
        melhor_avaliado as *const () as usize,
        melhor_da_lista as *const () as usize,
        jogador_saiu as *const () as usize,
        refazer_time as *const () as usize,
        substituir as *const () as usize,
        escalar as *const () as usize,
        postura as *const () as usize,
        escolher_formacao as *const () as usize,
        instrucoes as *const () as usize,
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
        comparar_procurar_gravando as *const () as usize,
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
        comparar_direcao as *const () as usize,
        comparar_zerar_guardado as *const () as usize,
        comparar_x_fora as *const () as usize,
        comparar_e_o_guardado as *const () as usize,
        comparar_lado_do_lance as *const () as usize,
        comparar_jogador_no_bloco as *const () as usize,
        comparar_tabela_a_1a as *const () as usize,
        comparar_momento_marcado as *const () as usize,
        comparar_estado_do_lado as *const () as usize,
        comparar_estado_do_adversario as *const () as usize,
        comparar_gravar_19c as *const () as usize,
        comparar_bola_parada as *const () as usize,
        comparar_perto_do_gol as *const () as usize,
        comparar_limpar_lance as *const () as usize,
        comparar_somar_disposicao as *const () as usize,
        comparar_repartir_d as *const () as usize,
        comparar_repartir_e as *const () as usize,
        comparar_repartir_f as *const () as usize,
        comparar_registrar_lance as *const () as usize,
        comparar_remover as *const () as usize,
        comparar_entrar as *const () as usize,
        comparar_em_perigo as *const () as usize,
        comparar_conferir_lance as *const () as usize,
        comparar_caminho_livre as *const () as usize,
        comparar_alcanca_ponto as *const () as usize,
        comparar_destaque as *const () as usize,
        comparar_escolher_b as *const () as usize,
        comparar_casa_formacao as *const () as usize,
        comparar_percentuais as *const () as usize,
        comparar_escolher_c as *const () as usize,
        comparar_aplicar_fator as *const () as usize,
        comparar_valor_condicao as *const () as usize,
        comparar_aplicar_time as *const () as usize,
        comparar_aplicar_lado as *const () as usize,
        comparar_escolher_d as *const () as usize,
        comparar_ler_1bc as *const () as usize,
        comparar_somar_2b as *const () as usize,
        comparar_gravar_2b as *const () as usize,
        comparar_tabela_a_1a_j as *const () as usize,
        comparar_tabela_b_j as *const () as usize,
        comparar_diferenca_j as *const () as usize,
        comparar_marcas_rot as *const () as usize,
        comparar_marca_800000 as *const () as usize,
        comparar_perto_proprio as *const () as usize,
        comparar_papel_880 as *const () as usize,
        comparar_soltar_bola as *const () as usize,
        comparar_pela_ficha as *const () as usize,
        comparar_byte_do_codigo as *const () as usize,
        comparar_nota_ajustada as *const () as usize,
        comparar_ajustar_17 as *const () as usize,
        comparar_pegar_bola as *const () as usize,
        comparar_iniciar_rigor as *const () as usize,
        comparar_lance_valendo as *const () as usize,
        comparar_decidir_falta as *const () as usize,
        comparar_punir_falta as *const () as usize,
        comparar_punir_lance as *const () as usize,
        comparar_motor_decidir as *const () as usize,
        comparar_motor_punir_lance as *const () as usize,
        comparar_motor_punir_falta as *const () as usize,
        comparar_impedimento as *const () as usize,
        comparar_motor_impedimento as *const () as usize,
        comparar_cartao_gravidade as *const () as usize,
        comparar_motor_cartao as *const () as usize,
        comparar_escolher_e as *const () as usize,
        comparar_estatistica_posse as *const () as usize,
        comparar_resumo as *const () as usize,
        comparar_mais_perto_no_lado as *const () as usize,
        comparar_defesa as *const () as usize,
        comparar_mover_para as *const () as usize,
        comparar_escolher_casa as *const () as usize,
        comparar_dominio as *const () as usize,
        comparar_tracar as *const () as usize,
        comparar_tem_jogo as *const () as usize,
        comparar_venceu as *const () as usize,
        comparar_comentario_a as *const () as usize,
        comparar_comentario_b as *const () as usize,
        comparar_bola_fora as *const () as usize,
        comparar_valor_chute as *const () as usize,
        comparar_forca_do_time as *const () as usize,
        comparar_pontape_inicial as *const () as usize,
        comparar_rolar_bola as *const () as usize,
        comparar_teste_de_condicao as *const () as usize,
        comparar_gosta_de as *const () as usize,
        comparar_nao_gosta_de as *const () as usize,
        comparar_marcar_posicao as *const () as usize,
        comparar_posicoes_finas as *const () as usize,
        comparar_unidades as *const () as usize,
        comparar_e_amistoso as *const () as usize,
        comparar_contrato_de as *const () as usize,
        comparar_e_selecao as *const () as usize,
        comparar_tecnico_9a01 as *const () as usize,
        comparar_tecnico_9a05 as *const () as usize,
        comparar_bit_da_casa as *const () as usize,
        comparar_separar_mascara as *const () as usize,
        comparar_e_segunda_selecao as *const () as usize,
        comparar_placar_agregado as *const () as usize,
        comparar_marcador_direto as *const () as usize,
        comparar_tirar_de_campo as *const () as usize,
        comparar_marcacao_individual as *const () as usize,
        comparar_nova_reputacao as *const () as usize,
        comparar_reputacao_do_jogador as *const () as usize,
        comparar_escolher_cobrador as *const () as usize,
        comparar_lateral as *const () as usize,
        comparar_escanteio as *const () as usize,
        comparar_penalti as *const () as usize,
        comparar_gravar_casa as *const () as usize,
        comparar_marcar_craques as *const () as usize,
        comparar_designado_da_posicao as *const () as usize,
        comparar_marcado_designado as *const () as usize,
        comparar_selecao_principal as *const () as usize,
        comparar_selecao_b as *const () as usize,
        comparar_quem_marcar as *const () as usize,
        comparar_tecnico_9ace as *const () as usize,
        comparar_tecnico_9a06 as *const () as usize,
        comparar_clube_humano as *const () as usize,
        comparar_lado_sob_controle as *const () as usize,
        comparar_linha_tatica as *const () as usize,
        comparar_byte_tatico as *const () as usize,
        comparar_ordem_dos_penaltis as *const () as usize,
        comparar_ordem_dos_jogadores as *const () as usize,
        comparar_vaga_marcada as *const () as usize,
        comparar_marcar_vaga as *const () as usize,
        comparar_pessoa_da_linha as *const () as usize,
        comparar_clube_na_competicao as *const () as usize,
        comparar_clube_especial as *const () as usize,
        comparar_decisao_da_falta as *const () as usize,
        comparar_escolher_companheiro as *const () as usize,
        comparar_falta_no_motor as *const () as usize,
        comparar_tecnico_9b23 as *const () as usize,
        comparar_nao_gosta_do_clube as *const () as usize,
        comparar_gosta_do_clube as *const () as usize,
        comparar_clube_parceiro as *const () as usize,
        comparar_contrato_com_clube as *const () as usize,
        comparar_nivel_do_contrato as *const () as usize,
        comparar_chance_de_conhecer as *const () as usize,
        comparar_conhecer_pessoa as *const () as usize,
        comparar_nevoa_apos_partida as *const () as usize,
        comparar_entrada_da_lista as *const () as usize,
        comparar_bandeiras_do_contrato as *const () as usize,
        comparar_proximo_da_lista as *const () as usize,
        comparar_palavra_0c as *const () as usize,
        comparar_arredondar as *const () as usize,
        comparar_nacao_do_grupo as *const () as usize,
        comparar_quantos_marcam as *const () as usize,
        comparar_quem_marca as *const () as usize,
        comparar_competicao_conta as *const () as usize,
        comparar_ordem_por_5f as *const () as usize,
        comparar_ordem_por_80 as *const () as usize,
        comparar_ordem_por_80_desc as *const () as usize,
        comparar_ordem_por_80_cresc as *const () as usize,
        comparar_nota_do_jogador as *const () as usize,
        comparar_esquecer_pessoa as *const () as usize,
        comparar_quadro_do_tecnico as *const () as usize,
        comparar_casa_do_setor_ocupada as *const () as usize,
        comparar_vaga_da_casa as *const () as usize,
        comparar_limpar_designados as *const () as usize,
        comparar_mascara_da_formacao as *const () as usize,
        comparar_casa_da_vaga as *const () as usize,
        comparar_criterio_da_competicao as *const () as usize,
        comparar_tipo_nao_11 as *const () as usize,
        comparar_tipo_11_a_15 as *const () as usize,
        comparar_classe_da_selecao as *const () as usize,
        comparar_ler_marca_da_pessoa as *const () as usize,
        comparar_zerar_marca_da_pessoa as *const () as usize,
        comparar_registro_livre as *const () as usize,
        comparar_ligar_44_da_pessoa as *const () as usize,
        comparar_byte_44_da_pessoa as *const () as usize,
        comparar_campo_3a_da_pessoa as *const () as usize,
        comparar_classe_pela_selecao as *const () as usize,
        comparar_global_9b5984 as *const () as usize,
        comparar_competicao_da_pessoa as *const () as usize,
        comparar_byte_42_da_selecao as *const () as usize,
        comparar_byte_43_da_selecao as *const () as usize,
        comparar_selecao_ou_especial as *const () as usize,
        comparar_nao_gosta_da_selecao as *const () as usize,
        comparar_copiar_data_e_marca as *const () as usize,
        comparar_atualizar_atributos as *const () as usize,
        comparar_desempenho_na_partida as *const () as usize,
        comparar_forca_do_lado as *const () as usize,
        comparar_bola_no_ataque as *const () as usize,
        comparar_evoluir_momento as *const () as usize,
        comparar_partida_de_interesse as *const () as usize,
        comparar_grupo_decidido as *const () as usize,
        comparar_narrar_classificacao as *const () as usize,
        comparar_procurar_jogada as *const () as usize,
        comparar_jogadores_no_trajeto as *const () as usize,
        comparar_interceptar as *const () as usize,
        comparar_avancar_linha as *const () as usize,
        comparar_arrumar_saida as *const () as usize,
        comparar_arrumar_bola_parada as *const () as usize,
        comparar_decidir_sem_bola as *const () as usize,
        comparar_mover_sem_bola as *const () as usize,
        comparar_decidir_jogador as *const () as usize,
        comparar_passar as *const () as usize,
        comparar_lancar as *const () as usize,
        comparar_desviar_bola as *const () as usize,
        comparar_defender as *const () as usize,
        comparar_recuperar_folego as *const () as usize,
        comparar_periodo_encerrado as *const () as usize,
        comparar_pressao as *const () as usize,
        comparar_desafio_adversario as *const () as usize,
        comparar_evoluir_jogador as *const () as usize,
        comparar_uniforme as *const () as usize,
        comparar_gravidade_da_lesao as *const () as usize,
        comparar_grau_da_lesao as *const () as usize,
        comparar_lesao_na_dividida as *const () as usize,
        comparar_avaliar_jogador as *const () as usize,
        comparar_melhor_avaliado as *const () as usize,
        comparar_melhor_da_lista as *const () as usize,
        comparar_jogador_saiu as *const () as usize,
        comparar_refazer_time as *const () as usize,
        comparar_substituir as *const () as usize,
        comparar_escalar as *const () as usize,
        comparar_postura as *const () as usize,
        comparar_escolher_formacao as *const () as usize,
        comparar_instrucoes as *const () as usize,
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
        "motor_direcao", "motor_zerar_guardado", "motor_x_fora_do_meio", "motor_e_o_guardado", "motor_lado_do_lance", "motor_jogador_no_bloco", "motor_tabela_a_1a", "motor_momento_marcado", "motor_estado_do_lado", "motor_estado_do_adversario", "motor_gravar_19c", "motor_codigo_de_bola_parada", "motor_perto_do_gol", "motor_limpar_lance", "motor_somar_disposicao",
        "motor_repartir_d", "motor_repartir_e", "motor_repartir_f", "motor_registrar_lance", "motor_remover_da_lista", "motor_entrar_em_campo", "motor_em_perigo",
        "motor_conferir_lance", "motor_caminho_livre_ate", "motor_alcanca_o_ponto",
        "motor_escolher_destaque", "motor_escolher_por_papel_b", "motor_casa_da_formacao",
        "motor_calcular_percentuais", "motor_escolher_por_papel_c",
        "motor_aplicar_fator", "motor_valor_pela_condicao", "motor_aplicar_ao_time", "motor_aplicar_ao_lado", "motor_escolher_pela_condicao",
        "motor_ler_1bc", "motor_somar_2b", "motor_gravar_2b", "motor_tabela_a_1a_do_jogador", "motor_tabela_b_do_jogador", "motor_diferenca_do_jogador", "motor_marcas_do_jogador_rot", "motor_marca_800000", "motor_perto_do_proprio_gol", "motor_papel_880", "motor_soltar_bola", "motor_jogador_pela_ficha", "motor_byte_do_codigo",
        "motor_nota_ajustada", "motor_ajustar_17",
        "motor_pegar_bola",
        "motor_iniciar_rigor", "motor_lance_valendo", "motor_decidir_falta",
        "motor_punir_falta", "motor_punir_lance",
        "motor_motor_decidir_falta", "motor_motor_punir_lance", "motor_motor_punir_falta",
        "motor_procurar_impedimento", "motor_motor_procurar_impedimento",
        "motor_cartao_pela_gravidade", "motor_motor_cartao_pela_gravidade",
        "motor_escolher_por_papel_e",
        "motor_estatistica_de_posse",
        "motor_resumo_da_partida",
        "motor_mais_perto_no_lado",
        "motor_defesa_do_goleiro",
        "motor_mover_para",
        "motor_escolher_casa",
        "motor_dominio_do_lance",
        "motor_tracar_lance",
        "motor_tem_jogo", "motor_venceu_o_jogo",
        "motor_comentario_a", "motor_comentario_b",
        "motor_bola_fora",
        "motor_valor_do_chute",
        "motor_forca_do_time",
        "motor_pontape_inicial",
        "motor_rolar_bola",
        "motor_teste_de_condicao",
        "motor_gosta_de", "motor_nao_gosta_de", "motor_marcar_posicao",
        "motor_posicoes_finas",
        "motor_unidades", "motor_e_amistoso", "motor_contrato_de", "motor_e_selecao",
        "motor_tecnico_9a01", "motor_tecnico_9a05", "motor_bit_da_casa", "motor_separar_mascara",
        "motor_e_segunda_selecao", "motor_placar_agregado",
        "motor_marcador_direto",
        "motor_tirar_de_campo",
        "motor_marcacao_individual",
        "motor_nova_reputacao",
        "motor_reputacao_do_jogador",
        "motor_escolher_cobrador",
        "motor_lateral",
        "motor_escanteio",
        "motor_penalti",
        "motor_gravar_casa",
        "motor_marcar_craques",
        "motor_designado_da_posicao",
        "motor_marcado_designado",
        "motor_selecao_principal", "motor_selecao_b",
        "motor_quem_marcar",
        "motor_tecnico_9ace", "motor_tecnico_9a06",
        "motor_clube_humano",
        "motor_lado_sob_controle",
        "motor_linha_tatica", "motor_byte_tatico",
        "motor_ordem_dos_penaltis",
        "motor_ordem_dos_jogadores",
        "motor_vaga_marcada", "motor_marcar_vaga", "motor_pessoa_da_linha",
        "motor_clube_na_competicao", "motor_clube_especial",
        "motor_decisao_da_falta",
        "motor_escolher_companheiro",
        "motor_falta_no_motor", "motor_tecnico_9b23", "motor_nao_gosta_do_clube", "motor_gosta_do_clube",
        "motor_clube_parceiro",
        "motor_contrato_com_clube", "motor_nivel_do_contrato",
        "motor_chance_de_conhecer", "motor_conhecer_pessoa", "motor_nevoa_apos_partida",
        "motor_entrada_da_lista", "motor_bandeiras_do_contrato",
        "motor_proximo_da_lista", "motor_palavra_0c", "motor_arredondar_abaixo_de_1000", "motor_nacao_do_grupo",
        "motor_quantos_marcam", "motor_quem_marca",
        "motor_competicao_conta", "motor_ordem_por_5f", "motor_ordem_por_80", "motor_ordem_por_80_desc", "motor_ordem_por_80_cresc",
        "motor_nota_do_jogador",
        "motor_esquecer_pessoa", "motor_quadro_do_tecnico", "motor_casa_do_setor_ocupada", "motor_vaga_da_casa",
        "motor_limpar_designados", "motor_mascara_da_formacao",
        "motor_casa_da_vaga", "motor_criterio_da_competicao",
        "motor_tipo_nao_11", "motor_tipo_11_a_15", "motor_classe_da_selecao", "motor_ler_marca_da_pessoa",
        "motor_zerar_marca_da_pessoa", "motor_registro_livre",
        "motor_ligar_44_da_pessoa", "motor_byte_44_da_pessoa", "motor_campo_3a_da_pessoa", "motor_classe_pela_selecao",
        "motor_global_9b5984",
        "motor_competicao_da_pessoa", "motor_byte_42_da_selecao", "motor_byte_43_da_selecao", "motor_selecao_ou_especial",
        "motor_nao_gosta_da_selecao", "motor_copiar_data_e_marca",
        "motor_atualizar_atributos",
        "motor_desempenho_na_partida",
        "motor_forca_do_lado",
        "motor_bola_no_ataque", "motor_evoluir_momento", "motor_partida_de_interesse",
        "motor_grupo_decidido", "motor_narrar_classificacao", "motor_procurar_jogada", "motor_jogadores_no_trajeto",
        "motor_interceptar", "motor_avancar_linha", "motor_arrumar_saida", "motor_arrumar_bola_parada",
        "motor_decidir_sem_bola", "motor_mover_sem_bola",
        "motor_decidir_jogador", "motor_passar", "motor_lancar",
        "motor_desviar_bola", "motor_defender", "motor_recuperar_folego", "motor_periodo_encerrado",
        "motor_pressao", "motor_desafio_adversario", "motor_evoluir_jogador", "motor_uniforme",
        "motor_gravidade_da_lesao", "motor_grau_da_lesao", "motor_lesao_na_dividida",
        "motor_avaliar_jogador", "motor_melhor_avaliado", "motor_melhor_da_lista",
        "motor_jogador_saiu", "motor_refazer_time", "motor_substituir",
        "motor_escalar", "motor_postura", "motor_escolher_formacao", "motor_instrucoes",
    ];
    for i in 0..N {
        if SEM_ROTINAS.contains(&nomes[i]) {
            linha(&format!("motor: {} fica no original (SEM_ROTINAS)", nomes[i]));
            continue;
        }
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
/// Diagnostico do autoteste: a grade do motor falso esta consistente (contagens 0..=11, cada
/// entrada um jogador do bloco do lado, na casa certa e com o indice certo em 0x28)?
unsafe fn grade_inconsistente(m: usize) -> Option<String> {
    for x in 0..9i8 {
        for y in 0..12i8 {
            let casa = casa_da_grade(m, x, y) as usize;
            for l in 0..2usize {
                let n = byte(casa, 0x58 + l);
                if !(0..=11).contains(&n) {
                    return Some(format!("casa ({},{}) lado {} contagem {}", x, y, l, n));
                }
                for i in 0..n as usize {
                    let p = *((casa + 4 * (11 * l + i)) as *const usize);
                    let base = m + 0x4796 + 8920 * l;
                    if p < base || p >= base + 20 * 0x1be || (p - base) % 0x1be != 0 {
                        return Some(format!("casa ({},{}) lado {} entrada {}/{} = {:#x}", x, y, l, i, n, p));
                    }
                    if byte(p, 0x102) != x || byte(p, 0x103) != y || byte(p, 0x28) != i as i8 {
                        return Some(format!("casa ({},{}) lado {} entrada {}: jogador em ({},{}) indice {}", x, y, l, i, byte(p, 0x102), byte(p, 0x103), byte(p, 0x28)));
                    }
                }
            }
        }
    }
    None
}

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
    let mut fichas = vec![0u32; 40];
    let mut pessoas = vec![vec![0u8; 0x80]; 4];
    let mut jogos = vec![0u8; 0x69 * 12 + 0x80];
    let mut partida42 = vec![0u8; 0x40];
    let mut clube42 = vec![vec![0u8; 0x90]; 2];
    let mut jogo42 = vec![0u8; 0x70];
    let mut comp42 = vec![0u8; 0x10];
    let mut prefs46 = vec![0u8; 0x40];
    let mut pessoa46 = vec![0u8; 0x80];
    let mut tabela55 = vec![0u8; 79 * 80 + 8];
    let mut tecnicos56 = vec![0u8; 42735 * 16 + 16];
    let mut partida50 = vec![0u8; 0x60];
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
            // decima quinta leva (acessores; extremos das faixas e lados fora de 0/1)
            let h5 = s.proximo();
            let h6 = s.proximo();
            *((j + 0x19a) as *mut u16) = h5 as u16;
            comparar_direcao(j);
            *((m + 0xf59e) as *mut usize) = if h6 & 1 == 0 { j } else { o };
            *((m + 0xf5a2) as *mut usize) = if h6 & 2 == 0 { j } else { o };
            comparar_zerar_guardado(m, [0u8, 1, 2, 255][((h5 >> 16) % 4) as usize]);
            *((j + 0x102) as *mut i8) = if (h5 >> 18) & 1 == 0 { ((h5 >> 19) % 11) as i8 - 1 } else { (h5 >> 19) as i8 };
            comparar_x_fora(j);
            *((j + 0x19e) as *mut usize) = m;
            *((m + 0xf582) as *mut usize) = if (h5 >> 27) & 1 == 0 { j } else { o };
            comparar_e_o_guardado(j);
            *((m + 0x475a) as *mut u8) = if (h5 >> 28) % 3 == 0 { 0 } else { (h5 >> 30) as u8 | 1 };
            *((m + 0x4761) as *mut usize) = if (h5 >> 38) % 4 == 0 { 0 } else { j };
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h5 >> 40) % 4) as usize];
            comparar_lado_do_lance(m);
            comparar_estado_do_lado(j);
            comparar_estado_do_adversario(j);
            *((j + 0x103) as *mut i8) = if (h5 >> 42) & 1 == 0 { ((h5 >> 43) % 14) as i8 - 1 } else { (h5 >> 43) as i8 };
            comparar_perto_do_gol(j);
            comparar_jogador_no_bloco(m, (h6 >> 3) as i8, (h6 >> 11) as i8);
            *((j + 0x27) as *mut i8) = ((h5 >> 51) % 2) as i8;
            *((j + 0x1a) as *mut i8) = if (h5 >> 52) & 1 == 0 { ((h5 >> 53) % 14) as i8 - 2 } else { (h5 >> 53) as i8 };
            comparar_tabela_a_1a(m, j);
            *((m + 0x8ed4) as *mut u16) = [0x1efu16, 0x3de, 0, h6 as u16][((h6 >> 20) % 4) as usize];
            *((j + 0x38) as *mut u8) = (h6 >> 22) as u8;
            *((m + 0x4792) as *mut usize) = j;
            comparar_momento_marcado(m);
            let v19c = [200i16, 201, -200, -201, i16::MIN, i16::MAX, 0, (h6 >> 30) as i16 % 400][((h6 >> 46) % 8) as usize];
            comparar_gravar_19c(j, v19c);
            comparar_bola_parada([12i16, 13, 21, 37, 46, 59, (h6 >> 49) as i16 % 70, (h6 >> 32) as i16][((h6 >> 61) % 8) as usize]);
            for o2 in 0..16 {
                *((m + 0x475a + o2) as *mut u8) = (h6 >> (o2 * 4)) as u8;
            }
            comparar_limpar_lance(m);
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                *((p + 0x19) as *mut i8) = (h6.rotate_left(k as u32) % 12) as i8 - 1;
            }
            comparar_somar_disposicao(m, ((h5 >> 60) % 2) as i8, (h6 >> 7) as i16);
            // decima sexta leva (tabelas de papel da 14a leva ainda valendo)
            let h7 = s.proximo();
            let h8 = s.proximo();
            let delta7 = ((h7 >> 20) % 4000) as i16 - 2000;
            comparar_repartir_d(m, (h7 % 2) as i8, delta7);
            comparar_repartir_e(m, ((h7 >> 1) % 2) as i8, delta7);
            comparar_repartir_f(m, ((h7 >> 2) % 2) as i8, delta7);
            // guarda pelo jogador (0x4761), nao pelo outro (0x4765): os dois variam independentes
            *((m + 0x4761) as *mut usize) = if (h7 >> 3) % 3 == 0 { j } else { 0 };
            *((m + 0x4765) as *mut usize) = if (h7 >> 4) % 2 == 0 { o } else { 0 };
            comparar_registrar_lance(m, (h8 >> 1) as u16, if (h8 >> 17) % 4 == 0 { 0 } else { j as u32 }, (h8 >> 19) as u32, (h8 >> 2) as u8, (h8 >> 10) as u8, (h8 >> 18) as u8, (h8 >> 26) as u8, (h8 >> 34) as u8, (h8 >> 42) as u8);
            *((j + 0x101) as *mut i8) = ((h7 >> 5) % 6) as i8 - 1;
            for k in 0..4usize {
                *((j + 0x1ae + 4 * k) as *mut usize) = (h7 >> (8 + k * 3)) as usize % 5;
            }
            comparar_remover(j, ((h7 >> 24) % 6) as usize);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h7 >> 28) % 4) as usize];
            *((m as isize + 0x4592 + byte(j, 0x27) as isize) as *mut i8) = ((h7 >> 30) % 14) as i8 - 2;
            comparar_entrar(m, j);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h7 >> 34) % 4) as usize];
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x102) as *mut i8) = ((h7 >> 36) % 11) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((h7 >> 40) % 14) as i8 - 1;
            comparar_em_perigo(j);
            // decima setima leva
            let h9 = s.proximo();
            let h10 = s.proximo();
            *((m + 0x475a) as *mut i8) = [0i8, 8, 1, 5][(h9 % 4) as usize];
            *((m + 0x4761) as *mut usize) = if (h9 >> 2) % 5 == 0 { 0 } else { j };
            *((j + 0x27) as *mut i8) = [0i8, 1, 2][((h9 >> 5) % 3) as usize];
            *((m + 0x8eae) as *mut i8) = [0i8, 1, 2][((h9 >> 7) % 3) as usize];
            *((m + 0x475b) as *mut i8) = ((h9 >> 9) % 10) as i8 - 1;
            *((m + 0x475c) as *mut i8) = ((h9 >> 13) % 14) as i8 - 1;
            comparar_conferir_lance(m);
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h10.rotate_left(k as u32 * 5);
                *((p + 0x19) as *mut i8) = (r % 12) as i8 - 1;
                *((p + 0x102) as *mut i8) = ((r >> 4) % 9) as i8;
                *((p + 0x103) as *mut i8) = ((r >> 8) % 12) as i8;
            }
            let (x9, y9) = (((h9 >> 20) % 11) as i8 - 1, ((h9 >> 24) % 15) as i8 - 2);
            comparar_caminho_livre(m, [0i8, 1, 2, -1][((h9 >> 28) % 4) as usize], x9, y9);
            let pp9 = m + 0x4796 + 8920 * ((h9 >> 30) % 2) as usize + ((h9 >> 31) % 20) as usize * 0x1be;
            comparar_caminho_livre(m, ((h9 >> 36) % 2) as i8, byte(pp9, 0x102), byte(pp9, 0x103));
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h9 >> 37) % 4) as usize];
            *((j + 0x102) as *mut i8) = ((h9 >> 39) % 9) as i8;
            *((j + 0x103) as *mut i8) = ((h9 >> 43) % 12) as i8;
            *((j + 0x10c) as *mut i8) = ((h9 >> 47) % 25) as i8 - 2;
            let (xa, ya) = if (h9 >> 52) % 6 == 0 { (byte(j, 0x102), byte(j, 0x103)) } else { (((h9 >> 55) % 9) as i8, ((h9 >> 59) % 12) as i8) };
            if (h9 >> 63) == 1 {
                // no limiar exato: 0x10c + 10 igual a diferenca das palavras
                let fundo9: i8 = if byte(j, 0x27) != 0 { 11 } else { 0 };
                let dif = (por_diferenca(m, xa, ya, 4, fundo9) as i16 as i32 - por_diferenca(m, xa, ya, byte(j, 0x102), byte(j, 0x103)) as i16 as i32).abs() - 10;
                if (-128..=127).contains(&dif) {
                    *((j + 0x10c) as *mut i8) = dif as i8;
                }
            }
            comparar_alcanca_ponto(j, xa, ya);
            // decima oitava leva (fichas e pessoas proprias; 0x69 e 0x6d sempre validos no bloco)
            let h11 = s.proximo();
            let h12 = s.proximo();
            for (k, f) in fichas.iter_mut().enumerate() {
                *f = (h11.rotate_left(k as u32) as u32) ^ k as u32;
            }
            for (k, pe) in pessoas.iter_mut().enumerate() {
                for o in [0x39usize, 0x3e, 0x44] {
                    pe[o] = (h12.rotate_left((k * 3 + o) as u32) % 23) as u8;
                }
            }
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h11.rotate_left(k as u32 * 3) ^ h12.rotate_right(k as u32);
                *((p + 0x19) as *mut i8) = (r % 13) as i8 - 2;
                *((p + 0x20) as *mut u8) = if (r >> 4) % 3 == 0 { 0xff } else { (r >> 6) as u8 };
                *((p + 0x35) as *mut i16) = if (r >> 9) % 4 == 0 { (r >> 12) as i16 } else { ((r >> 12) % 400) as i16 - 100 };
                for (o, sh) in [(0x03usize, 28), (0x06, 33), (0x0c, 38), (0x10, 43)] {
                    *((p + o) as *mut u8) = if (r >> 48) % 4 == 0 { (r >> sh) as u8 } else { ((r >> sh) % 21) as u8 };
                }
                *((p + 0x69) as *mut usize) = fichas.as_ptr().add(k) as usize;
                *((p + 0x6d) as *mut usize) = if (r >> 50) % 5 == 0 { 0 } else { pessoas[k % 4].as_ptr() as usize };
            }
            let partida11 = jogadores[0].as_mut_ptr() as usize;
            *((partida11 + 0x4d) as *mut u8) = (((h11 >> 40) % 10) as u8) | (((h11 >> 44) as u8) & 0xf0);
            *((m + 0x4792) as *mut usize) = partida11;
            comparar_destaque(m);
            let alto11 = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h11 >> 50) % 5) as usize];
            comparar_escolher_b(m, (alto11 & !0xff) | ((h11 >> 53) % 2) as u32);
            *((m + 0x8eae) as *mut i8) = [0i8, 1, 2, -1][((h12 >> 54) % 4) as usize];
            *((j + 0x27) as *mut i8) = ((h12 >> 56) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h12 >> 57) % 17) as i8 - 2;
            let (mut fx, mut fy) = (0u8, 0u8);
            comparar_casa_formacao(m, j, &mut fx, &mut fy);
            // decima nona leva: contagens (zeros, negativas e somas que transbordam) e atributos
            let h13 = s.proximo();
            let h14 = s.proximo();
            for k in 0..30usize {
                let r = h13.rotate_left(k as u32 * 2) ^ h14.rotate_right(k as u32);
                let v = match r % 6 {
                    0 => 0,
                    1 => -(((r >> 8) % 50) as i32),
                    2 => (r >> 8) as i32,
                    _ => ((r >> 8) % 300) as i32,
                };
                *((m + 0x8f02 + 4 * k) as *mut i32) = v;
            }
            comparar_percentuais(m, ((h13 >> 3) % 6) as i8 - 1);
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                for (n, o) in [0xc9usize, 0xa1, 0x9d, 0x95, 0xb1, 0x7d, 0x99, 0xd9, 0xad, 0xa9].iter().enumerate() {
                    let r = h14.rotate_left((k * 10 + n) as u32 % 64);
                    *((p + o) as *mut f32) = (r % 3000) as f32 / 100.0 - if r % 17 == 0 { 40.0 } else { 0.0 };
                }
            }
            for (k, pe) in pessoas.iter_mut().enumerate() {
                let r = h13.rotate_left(k as u32 * 9);
                let w = [0x8c, 0x8d, 0x3c, 0x3b, (r >> 8) % 200, (r >> 16) % 400][(r % 6) as usize] as i16;
                pe[5..7].copy_from_slice(&w.to_le_bytes());
                pe[0x21] = ((r >> 24) % 25) as u8;
            }
            let alto13 = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h13 >> 50) % 5) as usize];
            // vigesima leva: condicao 0x1a2, ficha 0x69 (pessoa com 0x5d) ou nenhuma, byte 0xff, 0x1a6
            let h15 = s.proximo();
            let h16 = s.proximo();
            for (k, pe) in pessoas.iter_mut().enumerate() {
                pe[0x5d] = (h15.rotate_left(k as u32 * 11) % 34) as u8;
            }
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h15.rotate_left(k as u32 * 3) ^ h16.rotate_right(k as u32 * 5);
                *((p + 0x1a2) as *mut f32) = [100.0f32, 0.0, 50.0, 99.5][(r % 4) as usize] + ((r >> 2) % 400) as f32 / 8.0 - 25.0;
                *((p + 0x69) as *mut usize) = if (r >> 12) % 5 == 0 { 0 } else { pessoas[k % 4].as_ptr() as usize };
                *((p + 0xff) as *mut i8) = ((r >> 15) % 40) as i8 - 8;
                *((p + 0x1a6) as *mut usize) = if (r >> 21) % 2 == 0 { j } else { 0 };
            }
            let pk = m + 0x4796 + 8920 * ((h16 >> 30) % 2) as usize + ((h16 >> 31) % 20) as usize * 0x1be;
            *((pk + 0x69) as *mut usize) = pessoas[0].as_ptr() as usize;
            let fat = [1.0f32, 0.0, 0.5, 1.5, 2.0, 0.95][((h15 >> 40) % 6) as usize] + ((h15 >> 43) % 9) as f32 / 64.0;
            comparar_aplicar_fator(pk, fat);
            comparar_valor_condicao(pk, [0usize, j, o][((h15 >> 47) % 3) as usize]);
            comparar_valor_condicao(pk, *((pk + 0x1a6) as *const usize));
            let jt = if (h16 >> 36) % 2 == 0 { pk } else { j };
            *((j + 0x27) as *mut i8) = ((h16 >> 37) % 2) as i8;
            comparar_aplicar_time(m, jt, fat);
            comparar_aplicar_lado(m, ((h16 >> 38) % 2) as i8, fat);
            comparar_escolher_d(m, ((h16 >> 39) % 2) as i8, [2u32, 0, 1, 0x102, (h16 >> 40) as u32][((h16 >> 44) % 5) as usize]);
            // vigesima primeira leva
            let h17 = s.proximo();
            let h18 = s.proximo();
            *((j + 0x1bc) as *mut u16) = h17 as u16;
            comparar_ler_1bc(j);
            *((j + 0x2b) as *mut u8) = (h17 >> 16) as u8;
            comparar_somar_2b(j, (h17 >> 24) as u8);
            comparar_gravar_2b(j, (h17 >> 32) as u8);
            *((j + 0x27) as *mut i8) = ((h17 >> 40) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h17 >> 41) % 15) as i8 - 2;
            *((j + 0x1a) as *mut i8) = ((h17 >> 45) % 15) as i8 - 2;
            *((j + 0x103) as *mut i8) = ((h17 >> 49) % 13) as i8 - 1;
            *((m + 0xf582) as *mut usize) = if (h17 >> 53) & 1 == 0 { j } else { o };
            *((j + 0x19e) as *mut usize) = if (h17 >> 54) % 4 == 0 { 0 } else { m };
            comparar_tabela_a_1a_j(j);
            comparar_tabela_b_j(j);
            comparar_diferenca_j(j);
            comparar_papel_880(j);
            *((j + 0x19e) as *mut usize) = m;
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [0x80u16, 0x800, 0x880, 0, (h18 >> k) as u16][((h18 >> (k * 3)) % 5) as usize];
                }
            }
            comparar_papel_880(j);
            *((j + 0x19) as *mut i8) = ((h17 >> 56) % 14) as i8 - 1;
            *((m + 0x9766 + 6371 * byte(j, 0x27) as usize) as *mut u32) = (h18 as u32) & 0x1801000;
            *((m + 6 + byte(j, 0x27) as usize) as *mut u8) = (h18 >> 32) as u8;
            comparar_marcas_rot(j);
            comparar_marca_800000(j);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h18 >> 40) % 4) as usize];
            *((m + 0x8ea8) as *mut i8) = ((h18 >> 42) % 14) as i8 - 1;
            comparar_perto_proprio(j);
            *((m + 0x1a7) as *mut u8) = (h18 >> 8) as u8 | 1;
            comparar_soltar_bola(j);
            let dono = jogadores[1].as_mut_ptr() as usize;
            *((dono + 0x23) as *mut usize) = m + 0x4796 + 8920 * ((h18 >> 46) % 2) as usize;
            let alvo21 = if (h18 >> 47) % 4 == 0 { 0x1234 } else { *((jogador_do_bloco(m, ((h18 >> 46) % 2) as i8, ((h18 >> 49) % 20) as usize) + 0x69) as *const usize) };
            comparar_pela_ficha(dono, alvo21);
            let cod = [0x1f40u32 - 1, 0x1f40, 0x1f40 + 0x2a4, 0x1f40 + 0x2a5, 0x1f40 + ((h18 >> 52) % 0x2a5) as u32, (h18 >> 20) as u32][((h18 >> 61) % 6) as usize];
            // a tabela (0xb68788) fica na area nao inicializada e esta zerada no inicio: valores temporarios
            let ic = ((cod & 0x7fff) as i32 - 0x1f40) as i16;
            let alvo_t = (0xb68788 + 18 * ic.clamp(0, 0x2a5) as usize) as *mut u8;
            let antes = *alvo_t;
            *alvo_t = (h17 >> 8) as u8 | 1;
            comparar_byte_do_codigo(cod | (((h18 >> 15) as u32 & 1) << 15) | ((h17 as u32) & 0xffff0000));
            *alvo_t = antes;
            // vigesima segunda leva
            let h19 = s.proximo();
            let h20 = s.proximo();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [0u16, 1, 0x80, 0x100, 0x200, 0x400, 0x800, 0x880, (h20 >> k) as u16][((h20 >> (k * 4)) % 9) as usize];
                }
            }
            *((j + 0x19e) as *mut usize) = if (h19 >> 60) % 6 == 0 { 0 } else { m };
            *((j + 0x27) as *mut i8) = ((h19 >> 2) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h19 >> 3) % 11) as i8;
            *((j + 0x10a) as *mut i8) = ((h19 >> 7) % 14) as i8 - 2;
            let pe22 = pessoas[0].as_mut_ptr();
            *((j + 0x6d) as *mut usize) = pe22 as usize;
            for (o, sh) in [(0x17usize, 11), (0x18, 16), (0x19, 21)] {
                *pe22.add(o) = ((h19 >> sh) % 45) as u8;
            }
            comparar_nota_ajustada(j, ((h19 >> 26) % 30) as i8 - 10);
            *((j + 0x19e) as *mut usize) = m;
            *((m + 0x477a) as *mut i32) = if (h19 >> 31) % 2 == 0 { -1 } else { (h19 >> 32) as i32 & 0xffff };
            *((j + 0x35) as *mut i16) = [7500i16, 7501, 7499, 9000, 3000, -5, (h20 >> 20) as i16][((h19 >> 34) % 7) as usize];
            *((j + 0x17) as *mut i8) = ((h19 >> 37) % 26) as i8 - 3;
            comparar_ajustar_17(j, if (h19 >> 42) % 4 == 0 { 0 } else { ((h19 >> 44) % 25) as i8 - 12 });
            // vigesima terceira leva: posse (dono anterior, passe do mesmo lado ou nao, zonas, papel 1)
            let h21 = s.proximo();
            let h22 = s.proximo();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = if (h22 >> k) & 1 == 0 { 1 } else { (h22 >> (k + 11)) as u16 & 0xfe };
                }
            }
            let o23 = outro.as_ptr() as usize;
            *((o23 + 0x19e) as *mut usize) = m;
            *((j + 0x19e) as *mut usize) = m;
            // lado so 0, 1 ou 2: com -1 a tabela de papeis do original le antes do motor e cai
            *((j + 0x27) as *mut i8) = [0i8, 1, 0, 1, 2][((h21 >> 1) % 5) as usize];
            *((j + 0x19) as *mut i8) = ((h21 >> 4) % 11) as i8;
            *((j + 0x102) as *mut i8) = ((h21 >> 8) % 10) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((h21 >> 12) % 14) as i8 - 1;
            *((j + 0x19a) as *mut u16) = (h21 >> 16) as u16;
            let pb = jogador_do_bloco(m, ((h21 >> 32) % 2) as i8, ((h21 >> 33) % 20) as usize);
            *((m + 0xf582) as *mut usize) = [0, o23, pb, j][((h21 >> 38) % 4) as usize];
            let ps = jogador_do_bloco(m, ((h21 >> 40) % 2) as i8, ((h21 >> 41) % 20) as usize);
            *((ps + 0x19e) as *mut usize) = m;
            *((pb + 0x19e) as *mut usize) = m;
            *((m + 0xf576) as *mut usize) = if (h21 >> 46) % 4 == 0 { 0 } else { ps };
            for (k, v) in [0x1a4usize, 0x1a5, 0x1a6, 0x1a7, 0x1cd, 0x8ea9, 0x8eab, 0x8eb1, 0x8eaa, 0x8d46, 0x8d47].iter().enumerate() {
                let r = (h22 >> (k * 5)) as u8;
                *((m + v) as *mut u8) = match r % 4 { 0 => 0, 1 => 1, 2 => 0xff, _ => r };
            }
            *((m + 0x4782) as *mut u32) = (h22 >> 13) as u32;
            comparar_pegar_bola(j);
            // vigesima quarta leva
            let h23 = s.proximo();
            let h24 = s.proximo();
            let arb = jogadores[2].as_mut_ptr() as usize;
            let pe24 = pessoas[1].as_mut_ptr();
            *(pe24.add(0x1e) as *mut i16) = [0i16, 151, 150, 349, 350, -3, ((h23 >> 8) % 420) as i16][(h23 % 7) as usize];
            *pe24.add(0x28) = [0u8, 0xf1, 0xf0, 2, (h23 >> 16) as u8 % 20][((h23 >> 24) % 5) as usize];
            comparar_iniciar_rigor(arb, m as u32, if (h23 >> 27) % 4 == 0 { 0 } else { pe24 as usize });
            *((arb + 4) as *mut usize) = if (h23 >> 29) % 4 == 0 { 0 } else { pe24 as usize };
            // lance: jogador do lance, dono da bola e a linha
            let jl = jogador_do_bloco(m, ((h23 >> 31) % 2) as i8, ((h23 >> 32) % 20) as usize);
            let dn = jogador_do_bloco(m, ((h23 >> 37) % 2) as i8, ((h23 >> 38) % 20) as usize);
            *((m + 0x475a) as *mut u8) = if (h23 >> 43) % 5 == 0 { 0 } else { 3 };
            *((m + 0x4761) as *mut usize) = if (h23 >> 45) % 6 == 0 { 0 } else { jl };
            *((m + 0xf582) as *mut usize) = if (h23 >> 47) % 6 == 0 { 0 } else { dn };
            *((m + 0x8ea8) as *mut i8) = ((h23 >> 49) % 13) as i8 - 1;
            *((m + 0x475c) as *mut i8) = ((h23 >> 53) % 13) as i8 - 1;
            comparar_lance_valendo(m);
            *((m + 0x475a) as *mut u8) = if (h24 >> 3) % 3 == 0 { 3 } else { 0 };
            *((m + 0x8ea9) as *mut i8) = ((h24 >> 5) % 5) as i8 - 1;
            let jf = if (h24 >> 8) % 2 == 0 { jl } else { dn };
            *((jf + 0x14) as *mut u8) = [(h24 >> 9) as u8 % 25, 200, 255][((h24 >> 60) % 3) as usize];
            *((jf + 0x102) as *mut i8) = ((h24 >> 17) % 10) as i8 - 1;
            *((jf + 0x103) as *mut i8) = [0i8, 1, 10, 11, ((h24 >> 21) % 12) as i8][((h24 >> 25) % 5) as usize];
            *((arb + 8) as *mut u16) = (h24 >> 28) as u16;
            let (mut t24, mut v24) = (0u8, 0u8);
            comparar_decidir_falta(arb, jf, if (h24 >> 44) % 2 == 0 { 0x25 } else { (h24 >> 45) as u16 % 64 }, &mut t24, &mut v24);
            // vigesima quinta leva: cartoes (tipo 1 na maioria; 7 e outros tambem)
            let h25 = s.proximo();
            let h26 = s.proximo();
            let jc = jogador_do_bloco(m, ((h25 >> 1) % 2) as i8, ((h25 >> 2) % 20) as usize);
            let oc = jogador_do_bloco(m, ((h25 >> 7) % 2) as i8, ((h25 >> 8) % 20) as usize);
            *((jc + 0x24) as *mut u8) = [0u8, 0, 1, 2][((h25 >> 13) % 4) as usize];
            *((jc + 0x102) as *mut i8) = ((h25 >> 15) % 10) as i8 - 1;
            *((jc + 0x103) as *mut i8) = [0i8, 1, 10, 11, ((h25 >> 19) % 12) as i8][((h25 >> 23) % 5) as usize];
            *((m + 0x8eb3) as *mut u8) = ((h25 >> 26) % 6 == 0) as u8;
            *((m + 0x8ed0) as *mut i16) = [0i16, 495, 496, 506, 1000, -600, (h26 >> 3) as i16][((h25 >> 29) % 7) as usize];
            let mut tc = [1u8, 1, 1, 7, 2, 0][((h25 >> 32) % 6) as usize];
            let mut pc = 0f32;
            let sev = ((h25 >> 35) % 30) as i8 - 5;
            let flag = ((h25 >> 40) % 3 == 0) as u8 | (((h25 >> 41) as u8) & 0xfe & if (h25 >> 49) % 5 == 0 { 0xff } else { 0 });
            comparar_punir_falta(arb, jc, &mut tc, ((h25 >> 42) % 90) as i8, sev, flag, &mut pc);
            let mut tc2 = [1u8, 1, 1, 7, 2, 0][((h26 >> 32) % 6) as usize];
            *((jc + 0x19) as *mut i8) = if (h26 >> 36) % 7 == 0 { -1 } else { ((h26 >> 37) % 11) as i8 };
            comparar_punir_lance(arb, jc, oc, &mut tc2, ((h26 >> 41) % 90) as i8, ((h26 >> 48) % 30) as i8 - 5, ((h26 >> 54) % 3 == 0) as u8, &mut pc, ((h26 >> 56) % 2) as u8);
            // vigesima sexta leva: o arbitro dentro do motor (0xf5b2: motor, pessoa, valor)
            let h27 = s.proximo();
            *((m + 0xf5b2) as *mut usize) = m;
            *((m + 0xf5b6) as *mut usize) = if h27 % 4 == 0 { 0 } else { pe24 as usize };
            *((m + 0xf5ba) as *mut u16) = (h27 >> 8) as u16;
            let (mut t26, mut v26) = (0u8, 0u8);
            comparar_motor_decidir(m, jf, if (h27 >> 24) % 2 == 0 { 0x25 } else { (h27 >> 25) as u16 % 64 }, &mut t26, &mut v26);
            let mut tc3 = [1u8, 1, 7, 2][((h27 >> 32) % 4) as usize];
            let mut pc3 = 0f32;
            comparar_motor_punir_falta(m, jc, &mut tc3, ((h27 >> 34) % 90) as i8, ((h27 >> 41) % 30) as i8 - 5, ((h27 >> 46) % 3 == 0) as u8, &mut pc3);
            let mut tc4 = [1u8, 1, 7, 2][((h27 >> 48) % 4) as usize];
            comparar_motor_punir_lance(m, jc, oc, &mut tc4, ((h27 >> 50) % 90) as i8, ((h27 >> 56) % 30) as i8 - 5, ((h27 >> 61) % 3 == 0) as u8, &mut pc3, ((h27 >> 63) & 1) as u8);
            // vigesima setima leva: impedimento (linhas 0x8eaf/0x8eb0, atacantes perto delas)
            let h28 = s.proximo();
            let h29 = s.proximo();
            let l27 = [0i8, 1, 0, 1, 2][(h28 % 5) as usize];
            let (lin0, lin1) = (((h28 >> 3) % 8) as i8 + 2, ((h28 >> 6) % 8) as i8 + 2);
            *((m + 0x8eaf) as *mut i8) = lin0;
            *((m + 0x8eb0) as *mut i8) = lin1;
            *((m + 0x475a) as *mut u8) = if (h28 >> 9) % 4 == 0 { 3 } else { 0 };
            pe24.add(0x29).write(((h28 >> 11) % 30) as u8);
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h29.rotate_left(k as u32 * 3);
                let base = if k < 20 { lin0 } else { lin1 };
                *((p + 0x103) as *mut i8) = base + (r % 6) as i8 - 2;
                *((p + 0x107) as *mut i8) = ((r >> 3) % 25) as i8 - 3;
            }
            let pas = if (h28 >> 16) % 3 == 0 { 0 } else { jogador_do_bloco(m, l27.max(0).min(1), ((h28 >> 18) % 20) as usize) };
            let linha27 = ((h28 >> 24) % 12) as i8;
            let y27 = if (h28 >> 28) % 2 == 0 { linha27 + if l27 == 0 { 1 } else { -1 } } else { ((h28 >> 29) % 12) as i8 };
            comparar_impedimento(m + 0xf5b2, l27, linha27, y27, pas);
            comparar_motor_impedimento(m, l27, linha27, ((h29 >> 50) % 12) as i8, pas);
            // vigesima oitava leva: cartao pela gravidade (pessoa do arbitro sempre presente aqui)
            let h30 = s.proximo();
            let h31 = s.proximo();
            pe24.add(0x25).write([0u8, 2, 3, 6, 7, 12, 13, 18, 19, 30][(h30 % 10) as usize]);
            pe24.add(0x24).write(((h30 >> 4) % 4) as u8);
            *((m + 0xf5b6) as *mut usize) = pe24 as usize;
            let jg = jogador_do_bloco(m, ((h30 >> 6) % 2) as i8, ((h30 >> 7) % 20) as usize);
            *((jg + 0x14) as *mut u8) = [((h30 >> 12) % 25) as u8, 200, 0][((h30 >> 17) % 3) as usize];
            *((jg + 0x19) as *mut i8) = if (h30 >> 19) % 6 == 0 { -1 } else { 3 };
            *((jg + 0x23) as *mut u8) = [0u8, 0, 1, 2][((h30 >> 22) % 4) as usize];
            *((jg + 0x24) as *mut u8) = [0u8, 0, 1, 2][((h30 >> 24) % 4) as usize];
            let mut tg = 0u8;
            let gv = [1u8, 2, 3, 4, 5, 6, 0, 7][((h30 >> 26) % 8) as usize];
            let fg = ((h30 >> 29) % 4 == 0) as u8;
            if (h30 >> 31) & 1 == 0 {
                comparar_cartao_gravidade(m + 0xf5b2, jg, &mut tg, gv, fg);
            } else {
                comparar_motor_cartao(m, jg, &mut tg, gv, fg);
            }
            let _ = h31;
            // vigesima nona leva: atributos da pessoa nos limiares (0x37, 0x43, soma de quatro)
            let h32 = s.proximo();
            for (k, pe) in pessoas.iter_mut().enumerate() {
                let r = h32.rotate_left(k as u32 * 13);
                pe[0x37] = [15u8, 16, 5, 6, 9, 10, (r >> 8) as u8 % 21][(r % 7) as usize];
                pe[0x43] = [15u8, 16, 5, 6, 9, 10, (r >> 16) as u8 % 21][((r >> 3) % 7) as usize];
                for (n, o) in [0x40usize, 0x36, 0x26, 0x1b].iter().enumerate() {
                    pe[*o] = ((r >> (24 + n * 6)) % 26) as u8;
                }
            }
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [2u16, 4, 8, 0x88, 0x808, 0x10, 0x90, 0x20, 0x40, 0x140, 0, (h32 >> k) as u16][((h32 >> (k * 4)) % 12) as usize];
                }
            }
            let alto32 = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h32 >> 50) % 5) as usize];
            comparar_escolher_e(m, (alto32 & !0xff) | ((h32 >> 53) % 2) as u32, if (h32 >> 54) % 3 == 0 { 0x100 } else { (h32 >> 56) as u32 & 0xff });
            // trigesima leva: janela de 55 instantes (indice perto do fim), zonas, periodos
            let h33 = s.proximo();
            *((m + 0x8ed4) as *mut u16) = [0x1efu16, 0x3de, 0x100, (h33 >> 4) as u16][(h33 % 4) as usize];
            *((m + 0x8ea8) as *mut i8) = [3i8, 4, 7, 8, -1, 11, ((h33 >> 20) % 14) as i8][((h33 >> 2) % 7) as usize];
            *((m + 0x8e9b) as *mut i8) = [0i8, 53, 54, ((h33 >> 24) % 55) as i8][((h33 >> 6) % 4) as usize];
            for k in 0..0x30usize {
                *((m + 0x1f0 + k) as *mut u8) = (h33.rotate_left(k as u32)) as u8;
            }
            for k in 0..55isize {
                for b in [0xa, 0x41, 0x78, 0x8d51, 0x8d88, 0x8dbf, 0x8df6, 0x8e2d, 0x8e64] {
                    *((m as isize + b + k) as *mut u8) = [0u8, 1, 0, 0xff][(h33.rotate_left((k as u32 + b as u32) % 64) % 4) as usize];
                }
            }
            comparar_estatistica_posse(m, [0i8, 1, 2, -1, -2][((h33 >> 30) % 5) as usize], [0u8, 1, 0x80][((h33 >> 32) % 3) as usize]);
            // trigesima primeira leva: resumo (notas nos limites 1 e 10, somas que transbordam)
            let h34 = s.proximo();
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h34.rotate_left(k as u32 * 7);
                *((p + 0x35) as *mut i16) = [-600i16, 499, 500, 1499, 1500, 10499, 10500, 12000, (r >> 8) as i16][(r % 9) as usize];
                for o in [1usize, 2, 4, 5, 7, 8, 9, 0xa, 0xb, 0xd, 0xe, 0xf, 0x12, 0x23, 0x24] {
                    *((p + o) as *mut u8) = (r >> (o % 50)) as u8;
                }
            }
            *((m + 0xf5bd) as *mut u8) = [0u8, 3, 200, (h34 >> 10) as u8][((h34 >> 2) % 4) as usize];
            *((m + 0xf5f3) as *mut u8) = [0u8, 2, 100, (h34 >> 18) as u8][((h34 >> 4) % 4) as usize];
            *((m + 0x8ed4) as *mut u16) = [0x1efu16, 0x3de, 7][((h34 >> 6) % 3) as usize];
            comparar_resumo(m, [0i16, 1, 0x3de, 2, -1][((h34 >> 8) % 5) as usize]);
            // trigesima segunda leva: o mais perto no lado (inclusive a propria casa: distancia zero)
            let h35 = s.proximo();
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h35.rotate_left(k as u32 * 3);
                *((p + 0x19) as *mut i8) = (r % 13) as i8 - 2;
                *((p + 0x2b) as *mut i8) = ((r >> 4) % 4) as i8 - 1;
                *((p + 0x102) as *mut i8) = ((r >> 8) % 9) as i8;
                *((p + 0x103) as *mut i8) = ((r >> 12) % 12) as i8;
            }
            comparar_mais_perto_no_lado(m, ((h35 >> 40) % 2) as i8, ((h35 >> 41) % 12) as i8, ((h35 >> 45) % 9) as i8, [0u8, 1, 2][((h35 >> 49) % 3) as usize]);
            // trigesima terceira leva: o goleiro (papel 1 quase sempre), na area ou fora, perto da bola
            let h36 = s.proximo();
            let h37 = s.proximo();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = if (h37 >> k) % 5 == 0 { 2 } else { 1 };
                }
            }
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = ((h36 >> 1) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h36 >> 2) % 11) as i8;
            *((j + 0x2b) as *mut i8) = [0i8, 1, 2, -1][((h36 >> 6) % 4) as usize];
            *((j + 0x102) as *mut i8) = [1i8, 2, 4, 6, 7][((h36 >> 8) % 5) as usize];
            let yg = if byte(j, 0x27) == 1 { [9i8, 10, 11, 5][((h36 >> 11) % 4) as usize] } else { [0i8, 1, 2, 5][((h36 >> 11) % 4) as usize] };
            *((j + 0x103) as *mut i8) = yg;
            *((j + 0x168) as *mut i16) = ((h36 >> 13) % 1200) as i16 - 100;
            *((j + 0x16a) as *mut i16) = ((h36 >> 24) % 1200) as i16 - 100;
            *((j + 0x3b) as *mut i16) = ((h36 >> 35) % 300) as i16;
            *((j + 0xd1) as *mut f32) = ((h36 >> 44) % 200) as f32 / 4.0;
            *((j + 0xf) as *mut u8) = (h36 >> 52) as u8;
            *((m + 0x8ea7) as *mut i8) = ((h37 >> 40) % 9) as i8;
            *((m + 0x8ea8) as *mut i8) = yg + ((h37 >> 44) % 5) as i8 - 2;
            *((m + 0x8eaa) as *mut i8) = ((h37 >> 48) % 30) as i8 - 3;
            let (mut sd, mut rb) = (0u8, 0u8);
            let cod = [3i8, 4, 5, 26, 27, 0x33, 1, -2, ((h37 >> 52) % 60) as i8][((h37 >> 56) % 9) as usize];
            let alvo_y = yg + ((h37 >> 20) % 5) as i8 - 2;
            comparar_defesa(j, ((h37 >> 24) % 9) as i8, alvo_y, ((h37 >> 28) % 2) as u8, cod, &mut sd, &mut rb);
            // trigesima quarta leva: grade consistente nas duas casas (listas com jogadores validos)
            let h38 = s.proximo();
            let lj = ((h38 >> 1) % 2) as i8;
            let jm = jogador_do_bloco(m, lj, ((h38 >> 2) % 20) as usize);
            let (ox, oy) = (((h38 >> 7) % 9) as i8, ((h38 >> 11) % 12) as i8);
            let (nx, ny) = if (h38 >> 15) % 6 == 0 { (ox, oy) } else { (((h38 >> 18) % 9) as i8, ((h38 >> 22) % 12) as i8) };
            *((jm + 0x102) as *mut i8) = ox;
            *((jm + 0x103) as *mut i8) = oy;
            let ca = casa_da_grade(m, ox, oy) as usize;
            let cn = casa_da_grade(m, nx, ny) as usize;
            let n_ant = 1 + ((h38 >> 26) % 11) as i8;
            *((ca + 0x58 + lj as usize) as *mut i8) = n_ant;
            for i in 0..11usize {
                let outro_j = jogador_do_bloco(m, lj, ((h38 >> (30 + i % 20)) % 20) as usize);
                *((ca as isize + 4 * (11 * lj as isize + i as isize)) as *mut usize) = if i % 5 == 4 { 0 } else { outro_j };
            }
            *((jm + 0x28) as *mut i8) = if (h38 >> 40) % 4 == 0 { -1 } else { ((h38 >> 42) % n_ant as u64) as i8 };
            if cn != ca {
                *((cn + 0x58 + lj as usize) as *mut i8) = ((h38 >> 46) % 11) as i8;
            } else if byte(jm, 0x28) < 0 {
                *((cn + 0x58 + lj as usize) as *mut i8) = ((h38 >> 46) % 11) as i8;
            }
            *((m + 0xf582) as *mut usize) = if (h38 >> 50) % 2 == 0 { jm } else { o };
            comparar_mover_para(jm, nx, ny);
            // trigesima quinta leva: escolha da casa (grade com contagens pequenas e listas validas)
            let h39 = s.proximo();
            let h40 = s.proximo();
            let lc = ((h39 >> 1) % 2) as i8;
            let jc5 = jogador_do_bloco(m, lc, ((h39 >> 2) % 20) as usize);
            *((jc5 + 0x102) as *mut i8) = [0i8, 1, 2, 3, 4, 5, 6, 7, 8][((h39 >> 7) % 9) as usize];
            *((jc5 + 0x103) as *mut i8) = ((h39 >> 11) % 12) as i8;
            *((jc5 + 0x19) as *mut i8) = ((h39 >> 15) % 11) as i8;
            for (o, sh) in [(0x105usize, 19), (0x106, 21), (0x107, 23), (0x2c, 25), (0x2d, 27)] {
                *((jc5 + o) as *mut i8) = ((h39 >> sh) % 4) as i8 - (o == 0x2d) as i8 * 0;
            }
            *((jc5 + 0x6d) as *mut usize) = pessoas[2].as_ptr() as usize;
            pessoas[2][0x26] = ((h39 >> 29) % 20) as u8;
            *((jc5 + 0xe1) as *mut f32) = ((h39 >> 34) % 40) as f32 / 3.0;
            *((jc5 + 0x19a) as *mut u16) = ((h39 >> 40) % 360) as u16;
            *((m + 0x8eaf) as *mut i8) = ((h40 >> 1) % 8) as i8 + 2;
            *((m + 0x8eb0) as *mut i8) = ((h40 >> 4) % 8) as i8 + 2;
            for i in 0..2usize {
                *((m + 6371 * i + 0x9766) as *mut u32) = (h40 >> (8 + i)) as u32 & 0x400;
            }
            for cel in 0..108usize {
                for l in 0..2usize {
                    let r = h40.rotate_left((cel * 2 + l) as u32 % 64);
                    let casa = m + 0x215e + 90 * cel;
                    let n = if r % 5 == 0 { (1 + (r >> 3) % 2) as i8 } else { 0 };
                    *((casa + 0x58 + l) as *mut i8) = n;
                    for i in 0..2usize {
                        *((casa + 4 * (11 * l + i)) as *mut usize) = jogador_do_bloco(m, l as i8, ((r >> (8 + i * 5)) % 20) as usize);
                    }
                }
            }
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [0x90u16, 0x810, 0xa0, 0x30, 0x10, 0x8b0, 1, 0, 0x880][((h40 >> (k * 5 % 60)) % 9) as usize];
                }
            }
            let (mut sxc, mut syc) = (((h39 >> 44) % 10) as i8 - 1, ((h39 >> 48) % 13) as i8 - 1);
            comparar_escolher_casa(m, jc5, &mut sxc, &mut syc, ((h39 >> 52) % 16) as u32 | if (h39 >> 56) % 4 == 0 { 0xff00 } else { 0 });
            // trigesima sexta leva: dominio (codigos da tabela, goleiro na area, bola com ou sem dono)
            let h41 = s.proximo();
            let h42 = s.proximo();
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = (h41 % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h41 >> 1) % 11) as i8;
            *((j + 0x102) as *mut i8) = ((h41 >> 5) % 9) as i8;
            *((j + 0x103) as *mut i8) = [0i8, 1, 10, 11, ((h41 >> 9) % 12) as i8][((h41 >> 13) % 5) as usize];
            for k in 0..0x40usize {
                *((j + 0x11a + 2 * k) as *mut u16) = (h42.rotate_left(k as u32) as u16) % 600;
            }
            *((j + 0x4d) as *mut f32) = ((h41 >> 16) % 10000) as f32 + 0.5;
            *((j + 0x75) as *mut f32) = ((h41 >> 30) % 50) as f32 / 2.0;
            *((j + 0x19a) as *mut i16) = ((h41 >> 36) % 360) as i16;
            *((j + 0x19c) as *mut i16) = ((h41 >> 45) % 100) as i16;
            *((m + 0x8ed8) as *mut i16) = ((h42 >> 3) % 360) as i16;
            *((m + 0x8ea9) as *mut i8) = [4i8, 3, 5, 6, -1, 9][((h42 >> 12) % 6) as usize];
            *((m + 0x8eaa) as *mut i8) = ((h42 >> 15) % 12) as i8 - 3;
            let dono36 = [j, o, 0][((h42 >> 20) % 3) as usize];
            *((m + 0xf582) as *mut usize) = dono36;
            *((o + 0x19c) as *mut i16) = ((h42 >> 40) % 90) as i16 + 1;
            let especiais = [37i8, 42, 43, 44, 45, 46, 47, 49, 50, 0x15, 0x25]; // 48 cairia no erro (caixa de mensagem) do original
            let todos = [1i8, 12, 19, 21, 33, 34, 36, 37, 41, 42, 43, 44, 45, 46, 47, 49, 50, 0x15, 0x25];
            let mut cod36 = if dono36 == 0 { especiais[((h42 >> 24) % especiais.len() as u64) as usize] } else { todos[((h42 >> 24) % todos.len() as u64) as usize] };
            let (mut a36, mut b36) = (7u8, 7u8);
            comparar_dominio(j, &mut cod36, &mut a36, &mut b36);
            // trigesima setima leva: tracado (linhas nos quatro sentidos, pontos, saidas do campo)
            let h43 = s.proximo();
            let (tx0, ty0) = (((h43 >> 1) % 11) as i8 - 1, ((h43 >> 5) % 14) as i8 - 1);
            let (tx1, ty1) = if (h43 >> 9) % 8 == 0 { (tx0, ty0) } else { (((h43 >> 12) % 11) as i8 - 1, ((h43 >> 16) % 14) as i8 - 1) };
            let cods = [6i8, 8, 9, 30, 31, 52, 54, 55, 60, 61, 62, 90, 91, 92, 94, 7, 0, 100];
            let cod37 = cods[((h43 >> 20) % cods.len() as u64) as usize];
            for cel in 0..108usize {
                for l in 0..2usize {
                    let r = h43.rotate_left((cel * 3 + l) as u32 % 64);
                    let casa = m + 0x215e + 90 * cel;
                    *((casa + 0x58 + l) as *mut i8) = if r % 4 == 0 { 1 } else { 0 };
                    *((casa + 44 * l) as *mut usize) = jogador_do_bloco(m, l as i8, ((r >> 5) % 20) as usize);
                }
            }
            comparar_tracar(m, tx0, ty0, tx1, ty1, cod37, (h43 >> 26) as u32 & 0x1ff | 0xabcd0000, (h43 >> 40) as u32 & 0x7);
            // trigesima oitava leva: lista de jogos do dia e placares (normal, prorrogacao, penaltis)
            let h44 = s.proximo();
            let clubes = [0x1000u32, 0x2000, 0x3000, 0x4000];
            for k in 0..10usize {
                let jg = jogos.as_mut_ptr() as usize + 0x69 * k + 0x20;
                let r = h44.rotate_left(k as u32 * 6);
                *(jg as *mut u32) = clubes[(r % 4) as usize];
                *((jg + 4) as *mut u32) = clubes[((r >> 2) % 4) as usize];
                *((jg + 0x3b) as *mut u32) = if (r >> 4) % 3 == 0 { 0 } else { 7 };
            }
            let dia = jogadores[4].as_mut_ptr() as usize;
            *((dia + 0x19) as *mut usize) = jogos.as_ptr() as usize;
            *((dia + 0x21) as *mut i32) = ((h44 >> 10) % 12) as i32 - 1;
            comparar_tem_jogo(dia, [0x1000u32, 0x3000, 0x5000][((h44 >> 14) % 3) as usize], ((h44 >> 16) % 2) as u32);
            let jogo44 = jogadores[3].as_mut_ptr() as usize;
            *((jogo44 + 0x1c) as *mut u32) = 0x1000;
            *((jogo44 + 0x20) as *mut u32) = 0x2000;
            for (k, o) in [0x43usize, 0x44, 0x47, 0x48, 0x49, 0x4a].iter().enumerate() {
                *((jogo44 + o) as *mut i8) = [-1i8, 0, 1, 2, 3][((h44 >> (20 + k * 3)) % 5) as usize];
            }
            comparar_venceu(jogo44, [0x1000u32, 0x2000][((h44 >> 40) % 2) as usize]);
            // quadragesima primeira leva: valor do chute (tipos dados e escolhidos, zonas, goleiro)
            let h45 = s.proximo();
            let h46 = s.proximo();
            let h47 = s.proximo();
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = (h45 % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h45 >> 1) % 11) as i8;
            *((j + 0x102) as *mut i8) = ((h45 >> 5) % 9) as i8;
            *((j + 0x103) as *mut i8) = [0i8, 1, 2, 9, 10, 11, ((h45 >> 9) % 12) as i8][((h45 >> 13) % 7) as usize];
            *((j + 0x10b) as *mut i8) = ((h45 >> 16) % 22) as i8;
            *((j + 0x109) as *mut i8) = ((h45 >> 21) % 26) as i8 - 5;
            // palavras por tipo de chute (0x11a + 2 x tipo, ate 0x3a); depois vem 0x19e (motor)
            for k in 0..0x3busize {
                *((j + 0x11a + 2 * k) as *mut i16) = (h46.rotate_left(k as u32 % 64) % 700) as i16;
            }
            *((j + 0x29) as *mut i16) = ((h45 >> 26) % 400) as i16;
            *((j + 0x19c) as *mut i16) = ((h45 >> 35) % 300) as i16 - 50;
            *((j + 0x198) as *mut i16) = if (h45 >> 44) % 9 == 0 { 0x3b } else { 7 };
            *((j + 0x4d) as *mut f32) = ((h47 >> 1) % 10001) as f32;
            for (o, sh) in [(0x79usize, 15), (0x81, 21), (0xe5, 27), (0xa1, 33), (0xb1, 39)] {
                *((j + o) as *mut f32) = ((h47 >> sh) % 120) as f32 / 4.0;
            }
            *((m + 0x8ea9) as *mut i8) = [0i8, 0, 1, 2, 3, 4, 5, -1][((h47 >> 45) % 8) as usize];
            *((m + 0x4756) as *mut i8) = ((h47 >> 48) % 6) as i8;
            *((m + 1) as *mut i8) = [2i8, 0x20, 0, 5][((h47 >> 51) % 4) as usize];
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = if (h46 >> k) % 3 == 0 { 1 } else { 2 };
                }
            }
            let mut tc41 = [-1i8, -1, -1, -1, 0x33, 0x35, 3, 5, 0x16, 0x39, 0x3a, 0x1b][((h47 >> 53) % 12) as usize];
            let mut som41 = 0u16;
            comparar_valor_chute(j, &mut tc41, &mut som41);
            // quadragesima segunda leva: forca do time (papeis, atributos nas bordas, competicao, reputacoes)
            let h48 = s.proximo();
            let h49 = s.proximo();
            let l42 = (h48 % 2) as i8;
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    let r = h49.rotate_left(k as u32 * 5);
                    *v = [1u16, 2, 4, 8, 0x10, 0x20, 0x40, 0, 0x80, (r >> 8) as u16][(r % 10) as usize];
                }
            }
            for (k, pe) in pessoas.iter_mut().enumerate() {
                for o in 0x1busize..0x46 {
                    let r = h48.rotate_left((k * 37 + o) as u32 % 64) ^ h49.rotate_right(o as u32);
                    pe[o] = match r % 9 {
                        0 => (r >> 8) as u8,
                        1 => [5u8, 6, 9, 10, 15, 16][((r >> 8) % 6) as usize],
                        _ => ((r >> 8) % 21) as u8,
                    };
                }
                pe[0x21] = (h49.rotate_left(k as u32 * 7) % 25) as u8;
                let r = h48.rotate_left(k as u32 * 13);
                pe[5..7].copy_from_slice(&(((r >> 3) % 300) as i16 - 20).to_le_bytes());
            }
            for k in 0..20usize {
                let p = jogador_do_bloco(m, l42, k);
                let r = h48.rotate_left(k as u32 * 3) ^ h49.rotate_right(k as u32 * 7);
                *((p + 0x19) as *mut i8) = if r % 7 == 0 { -1 } else { ((r >> 3) % 11) as i8 };
                *((p + 0x69) as *mut usize) = fichas.as_ptr().add(k) as usize;
                *((p + 0x6d) as *mut usize) = pessoas[(r >> 7) as usize % 4].as_ptr() as usize;
                *((p + 0x35) as *mut i16) = [7920i16, 7923, 5940, 5953, 5610, 5622, 6270, 6278, 6930, 6941, 7590, 7615, ((r >> 10) % 9000) as i16][((r >> 24) % 13) as usize];
                *((p + 0x4d) as *mut f32) = ((r >> 30) % 10001) as f32;
                for (n, o) in [0xddusize, 0xcd, 0xc1, 0x7d, 0xf1, 0xc5, 0x9d, 0xa9, 0x8d, 0x95, 0x71, 0xb9, 0x99, 0xd1, 0xa5, 0x75, 0xad].iter().enumerate() {
                    let z = h49.rotate_left((k * 17 + n) as u32 % 64) ^ r;
                    *((p + o) as *mut f32) = (z % 4000) as f32 / 100.0 - if z % 13 == 0 { 15.0 } else { 0.0 };
                }
            }
            let pa = partida42.as_mut_ptr() as usize;
            let (c42, f42, j42, x42) = (clube42[0].as_mut_ptr() as usize, clube42[1].as_mut_ptr() as usize, jogo42.as_mut_ptr() as usize, comp42.as_mut_ptr() as usize);
            *((pa + 0x14) as *mut usize) = if (h48 >> 40) % 6 == 0 { 0 } else { j42 };
            *((pa + 0x1c) as *mut usize) = if (h48 >> 43) % 8 == 0 { 0 } else { c42 };
            *((pa + 0x20) as *mut usize) = if (h48 >> 46) % 8 == 0 { 0 } else { f42 };
            *((c42 + 0x53) as *mut usize) = if (h48 >> 49) % 8 == 0 { 0 } else { x42 };
            *((j42 + 0x5d) as *mut usize) = if (h48 >> 52) % 4 == 0 { 0 } else { x42 };
            let ids = [0x9cf314usize, 0x9cf45c, 0x9cf244, 0x9cf4a8, 0x9cf408, 0x9cf2e4, 0x9cf338, 0x9cf2c0, 0x9cf434, 0x9cf300];
            // as competicoes do jogo podem estar vazias no inicio: valores distintos durante o teste
            let ids_antes: Vec<u32> = ids.iter().map(|&a| *(a as *const u32)).collect();
            for (i, &a) in ids.iter().enumerate() {
                *(a as *mut u32) = 0x7000 + i as u32;
            }
            *(x42 as *mut u32) = if (h48 >> 55) % 11 == 10 { 0x1234 } else { 0x7000 + ((h48 >> 55) % 11) as u32 };
            let reps = [1000i16, 1001, 1251, 751, 251, 0, -250, -249, -751, -750, 2500, 2499, 30000, -30000];
            *((c42 + 0x80) as *mut i16) = reps[((h49 >> 40) % 14) as usize];
            *((f42 + 0x80) as *mut i16) = if (h49 >> 44) % 3 == 0 { 0 } else { reps[((h49 >> 46) % 14) as usize] };
            *((m + 0x4792) as *mut usize) = pa;
            *((m + 0xf5bd) as *mut u8) = ((h49 >> 50) % 2) as u8;
            *((m + 0xf5f3) as *mut u8) = ((h49 >> 51) % 2) as u8;
            for o in 0..0x10usize {
                *((m + 0x1d8 + 2 * o) as *mut u16) = (h49 >> (o * 3)) as u16;
            }
            comparar_forca_do_time(m, l42);
            // quadragesima terceira leva: pontape inicial (grade consistente, formacoes com casas distintas)
            let h50 = s.proximo();
            let h51 = s.proximo();
            core::ptr::write_bytes((m + 0x215e) as *mut u8, 0, 90 * 108);
            for (l, f) in formacoes.iter_mut().enumerate() {
                let base = f.as_mut_ptr() as usize + 1200 + if l == 1 { 0xa6 } else { 0x9e };
                for pos in 0..13usize {
                    let c = (h50.rotate_left((pos * 5 + l * 3) as u32) % 108) as u8;
                    // casas distintas por posicao no proprio campo (lado 0: y 0..=5; lado 1: 6..=11) e, de
                    // vez em quando, fora dele (exercita o limite de y sem passar de 11 por casa)
                    // casa distinta por posicao (com o limite de y, no maximo 4 por casa): uma casa
                    // com mais de 11 jogadores corrompe as listas e o comparador de 0x6f46b0 le lixo
                    let _ = c;
                    let x = ((h50 >> 8) % 9) as u8 + pos as u8;
                    let x = x % 9;
                    let mut y = (pos as u8 / 9) * 3 + ((h50 >> 12) % 3) as u8;
                    if (h50 >> ((pos + 13 * l) % 64)) % 4 == 0 {
                        y += 6;
                    }
                    if l == 1 {
                        y = (y + 6) % 12;
                    }
                    *((base + 216 * pos) as *mut u8) = y << 4 | x;
                }
            }
            let desloc = (h50 % 13) as usize;
            for k in 0..40usize {
                let l = k / 20;
                let p = m + 0x4796 + 8920 * l + (k % 20) * 0x1be;
                let r = h51.rotate_left(k as u32 * 3) ^ h50.rotate_right(k as u32);
                *((p + 0x27) as *mut i8) = l as i8;
                *((p + 0x19) as *mut i8) = if r % 6 == 0 { -1 } else { ((k + desloc) % 13) as i8 };
                *((p + 0x19e) as *mut usize) = m; // 0 so cairia no original (a grade e lida pelo motor do jogador)
                *((p + 0x6d) as *mut usize) = pessoas[k % 4].as_ptr() as usize;
                *((p + 0x10c) as *mut i8) = ((r >> 8) % 21) as i8;
                *((p + 0x105) as *mut i8) = ((r >> 13) % 21) as i8;
                let (rx, ry) = (((r >> 18) % 9) as i8, ((r >> 22) % 12) as i8);
                *((p + 0x102) as *mut i8) = rx;
                *((p + 0x103) as *mut i8) = ry;
                let casa = casa_da_grade(m, rx, ry) as usize;
                let n = byte(casa, 0x58 + l);
                *((casa as isize + 4 * (11 * l as isize + n as isize)) as *mut usize) = p;
                *((casa + 0x58 + l) as *mut i8) = n + 1;
                *((p + 0x28) as *mut i8) = n;
            }
            // as vezes um lado sem ninguem nas posicoes 0..=10 (o pontape sem jogador: byte 0 do motor)
            if (h51 >> 58) % 4 == 0 {
                let lv = ((h51 >> 60) % 2) as usize;
                for k in 0..20usize {
                    *((m + 0x4796 + 8920 * lv + k * 0x1be + 0x19) as *mut i8) = [-1i8, 11, 12][(k + (h51 >> 62) as usize) % 3];
                }
            }
            let bloco = |x: u64| m + 0x4796 + 8920 * (x % 2) as usize + ((x >> 1) % 20) as usize * 0x1be;
            *((m + 0xf582) as *mut usize) = if (h51 >> 40) % 3 == 0 { 0 } else { bloco(h51 >> 42) };
            *((m + 0xf576) as *mut usize) = if (h51 >> 48) % 3 == 0 { 0 } else { bloco(h51 >> 50) };
            *((m + 0x8eae) as *mut i8) = [0i8, 1, -1, 2][((h51 >> 56) % 4) as usize];
            *((m + 0x8ed8) as *mut u16) = (h51 >> 20) as u16;
            *(m as *mut u8) = 0;
            static AVISOS47: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let avisar = || AVISOS47.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 6;
            if let Some(e) = grade_inconsistente(m) {
                if avisar() {
                    linha(&format!("motor: grade inconsistente antes do pontape: {}", e));
                }
            }
            comparar_pontape_inicial(m);
            let grade47 = grade_inconsistente(m);
            if let Some(e) = &grade47 {
                if avisar() {
                    linha(&format!("motor: grade inconsistente depois do pontape: {}", e));
                }
            }
            // quadragesima setima leva: posicoes finas (a grade do pontape continua consistente; posicoes 0..=10)
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let v = byte(p, 0x19);
                if !(0..=10).contains(&v) {
                    *((p + 0x19) as *mut i8) = (k % 11) as i8;
                }
                *(p as *mut u8) = (h51 >> (k % 50)) as u8;
            }
            let mut saida47 = [0xa5u8; 0x100];
            if grade47.is_none() {
                comparar_posicoes_finas(m, saida47.as_mut_ptr());
            }
            // quinquagesima segunda leva: tirar de campo (grade consistente do pontape, marcacoes, modos,
            // goleiro guardado, tecnico humano com as globais trocadas durante o teste)
            if grade47.is_none() {
                let h60 = s.proximo();
                let h61 = s.proximo();
                let lj = ((h60 >> 1) % 2) as usize;
                let jt = m + 0x4796 + 8920 * lj + ((h60 >> 2) % 20) as usize * 0x1be;
                for k in 0..40usize {
                    let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                    *((p + 0x1aa) as *mut usize) = 0;
                    *((p + 0x101) as *mut i8) = 0;
                }
                // jt marca alguem (que o tem na lista) e e marcado por ate 3
                let alvo = m + 0x4796 + 8920 * (1 - lj) + ((h60 >> 8) % 20) as usize * 0x1be;
                if (h60 >> 13) % 3 != 0 {
                    *((jt + 0x1aa) as *mut usize) = alvo;
                    *((alvo + 0x101) as *mut i8) = 1;
                    *((alvo + 0x1ae) as *mut usize) = if (h60 >> 15) % 4 == 0 { m } else { jt };
                }
                let nm = ((h60 >> 17) % 4) as i8;
                *((jt + 0x101) as *mut i8) = nm;
                for i in 0..nm as usize {
                    let q = m + 0x4796 + 8920 * (1 - lj) + ((h60 >> (20 + 3 * i)) % 20) as usize * 0x1be;
                    *((jt + 0x1ae + 4 * i) as *mut usize) = q;
                    *((q + 0x1aa) as *mut usize) = [jt, 0, alvo][((h60 >> (30 + 2 * i)) % 3) as usize];
                }
                *((m + 0xf59e) as *mut usize) = if (h60 >> 34) % 3 == 0 { jt } else { 0 };
                *((m + 0xf5a2) as *mut usize) = if (h60 >> 36) % 3 == 0 { jt } else { 0 };
                // tecnico humano
                let globais60 = [0xae2364usize, 0xb63c98];
                let antes60: Vec<u32> = globais60.iter().map(|&a| *(a as *const u32)).collect();
                *(0xae2364 as *mut i32) = 1000;
                let regs60 = tecnicos56.as_mut_ptr() as usize;
                for k in 0..16usize {
                    *((regs60 + 42735 * k + 0x9a01) as *mut u32) = if (h61 >> k) % 2 == 0 { 0 } else { 7 };
                    *((regs60 + 42735 * k + 0x9a05) as *mut u8) = ((h61 >> (k + 16)) % 2) as u8;
                }
                let gerente60 = [regs60, 16usize];
                *(0xb63c98 as *mut usize) = gerente60.as_ptr() as usize;
                let mut tec60 = [0u8; 0x60];
                *(tec60.as_mut_ptr() as *mut i32) = [1000 - 16, 999, 1000, 983, 5][((h61 >> 32) % 5) as usize];
                tec60[0x5f] = ((h61 >> 35) % 3) as u8;
                for l in 0..2usize {
                    *((m + 0x904d + 208 * l) as *mut usize) = if (h61 >> (37 + l)) % 3 == 0 { 0 } else { tec60.as_ptr() as usize };
                    *((m + 0x904b + 208 * l) as *mut u8) = ((h61 >> (40 + l)) % 2) as u8;
                }
                let globais_sel = [0xae235cusize, 0xae2350];
                let antes_sel: Vec<u32> = globais_sel.iter().map(|&a| *(a as *const u32)).collect();
                *(0xae235c as *mut i32) = 5000;
                *(0xae2350 as *mut i32) = 200;
                let clubes60 = [[4500i32 + ((h61 >> 42) % 2) as i32 * 200], [4500i32 + ((h61 >> 43) % 2) as i32 * 200]];
                let pa60 = partida50.as_mut_ptr() as usize;
                *((pa60 + 0x1c) as *mut usize) = clubes60[0].as_ptr() as usize;
                *((pa60 + 0x20) as *mut usize) = clubes60[1].as_ptr() as usize;
                *((m + 0x4792) as *mut usize) = pa60;
                comparar_tirar_de_campo(m, jt, [-1i8, -3, 0, 1, -2][((h61 >> 44) % 5) as usize]);
                // quinquagesima sexta leva: escolher o cobrador (tipos sem a tatica do clube: o objeto de
                // 0xde1d08 so existe com o banco carregado; escanteio e falta perto do gol ficam para o jogo real)
                {
                    // fichas contiguas, como o vetor de pessoas do jogo (0xae23bc, 110 bytes cada): a lista de
                    // penaltis do tecnico humano guarda indices nele
                    let mut fichas56 = vec![0u8; 110 * 20];
                    let pessoas_antes = *(0xae23bc as *const usize);
                    *(0xae23bc as *mut usize) = fichas56.as_mut_ptr() as usize;
                    let mut pessoas56 = vec![[0u8; 0x30]; 20];
                    let mut excl56 = [0u8; 40];
                    let r0 = s.proximo();
                    let lado56 = (r0 % 2) as i8;
                    // os blocos do lado voltam como estavam no fim (fichas e pessoas daqui sao locais;
                    // as coordenadas nao mudam, para nao desfazer a grade)
                    let bloco56 = jogador_do_bloco(m, lado56, 0);
                    let antes_bloco56 = core::slice::from_raw_parts(bloco56 as *const u8, 20 * 0x1be).to_vec();
                    let tab = *((m + 0x8ebc + 4 * lado56 as usize) as *const usize) as *mut u16;
                    for i in 0..11 {
                        let z = s.proximo();
                        *tab.add(i) = [1u16, 0x40, 0x41, 0, 2, z as u16][(z % 6) as usize];
                    }
                    for k in 0..20usize {
                        let q = jogador_do_bloco(m, lado56, k);
                        let z = s.proximo();
                        let z2 = s.proximo();
                        *((q + 0x19) as *mut i8) = [0i8, 3, 7, 10, 11, 12, -1, 5][(z % 8) as usize];
                        *((q + 0x27) as *mut i8) = lado56;
                        *((q + 0x69) as *mut usize) = fichas56.as_mut_ptr() as usize + 110 * k;
                        *((q + 0x6d) as *mut usize) = if (z >> 3) % 9 == 0 { 0 } else { pessoas56[k].as_mut_ptr() as usize };
                        for (n, o) in [0xddusize, 0xe9, 0x89, 0xad, 0xc9, 0xa1].iter().enumerate() {
                            let y = z2.rotate_left(n as u32 * 11) ^ z;
                            *((q + o) as *mut f32) = [0.0f32, 1.0, 10.5, 20.0, -3.0, ((y >> 8) % 4000) as f32 / 100.0][(y % 6) as usize];
                        }
                        *((q + 0x109) as *mut i8) = ((z >> 16) % 21) as i8 - 1;
                        fichas56[110 * k + 0x18] = [17u8, 19, 20, 23, 24, 30][((z >> 21) % 6) as usize];
                        fichas56[110 * k + 0x5a] = [0u8, 0, 1, 2, 5][((z >> 24) % 5) as usize];
                        pessoas56[k][0xf] = ((z >> 27) % 21) as u8;
                        *(pessoas56[k].as_mut_ptr().add(0xb) as *mut i16) = [0i16, 100, 10000, 20000, -50][((z >> 32) % 5) as usize];
                        pessoas56[k][0x29] = [0u8, 1, 10, 20, 0xf0][((z >> 36) % 5) as usize];
                        excl56[20 * lado56 as usize + k] = ((z >> 40) % 3 == 0) as u8;
                    }
                    // sempre um candidato valido (sem tatica e sem excluir: o original da erro fatal sem ninguem)
                    let q0 = jogador_do_bloco(m, lado56, (r0 >> 1) as usize % 20);
                    *((q0 + 0x19) as *mut i8) = 4;
                    *((q0 + 0x6d) as *mut usize) = pessoas56[0].as_mut_ptr() as usize;
                    let gq = jogador_do_bloco(m, lado56, (r0 >> 6) as usize % 20);
                    *((m + 0xf59e) as *mut usize) = if (r0 >> 11) % 3 == 0 { gq } else { 0 };
                    *((m + 0xf5a2) as *mut usize) = if (r0 >> 13) % 3 == 0 { gq } else { 0 };
                    if gq == q0 {
                        *((m + 0xf59e) as *mut usize) = 0;
                        *((m + 0xf5a2) as *mut usize) = 0;
                    }
                    let a7 = [-1i8, 0, 1, 2, 4, 6, 7, 8, 9][((r0 >> 15) % 9) as usize];
                    let a8 = [-1i8, 0, 1, 2, 5, 9, 10, 11, 12][((r0 >> 19) % 9) as usize];
                    *((m + 0x8ea7) as *mut i8) = a7;
                    *((m + 0x8ea8) as *mut i8) = a8;
                    let mut tipo = [1i8, 2, 4, 5, 5, 5, 6, 7, 8][((r0 >> 23) % 9) as usize];
                    let ofs = if lado56 != 0 { 0 } else { 11 };
                    let (cx, ax) = ((a7 as i32 - 4).abs(), (a8 as i32 - ofs).abs());
                    if (tipo == 6 || tipo == 7) && (2..=6).contains(&a7) && cx <= 8 && ax <= 11 && (distancia_tabela(ax, cx) as f64) < f64_em(0x96d428) {
                        tipo = 8; // perto do gol a falta consulta a tatica
                    }
                    let excl = if tipo == 5 && (r0 >> 27) % 3 == 0 { excl56.as_ptr() as usize } else { 0 };
                    // lista de cobradores de penalti do tecnico humano (0x601070: registro + 0x1e05 com 0x9a69 = 0):
                    // fichas dos jogadores em algumas das 50 vagas
                    let i56 = *(tec60.as_ptr() as *const i32) - 1000 + 16;
                    if (0..16).contains(&i56) {
                        let reg = regs60 + 42735 * i56 as usize;
                        if tipo == 5 && (r0 >> 30) % 2 == 0 {
                            // metade dos penaltis pelo caminho da lista (tecnico com a marca 0x5f, sem 0x9a01)
                            tec60[0x5f] = 1;
                            *((reg + 0x9a01) as *mut u32) = 0;
                            *((m + 0x904d + 208 * lado56 as usize) as *mut usize) = tec60.as_ptr() as usize;
                        }
                        for b in 0..50usize {
                            let z = s.proximo();
                            *((reg + 0x1e05 + 4 * b) as *mut i32) = if z % 4 == 0 { ((z >> 8) % 20) as i32 } else { -1 };
                        }
                    }
                    comparar_escolher_cobrador(m, lado56, tipo, excl);
                    core::ptr::copy_nonoverlapping(antes_bloco56.as_ptr(), bloco56 as *mut u8, antes_bloco56.len());
                    *(0xae23bc as *mut usize) = pessoas_antes;
                }
                for (i, &a) in globais60.iter().enumerate() {
                    *(a as *mut u32) = antes60[i];
                }
                for (i, &a) in globais_sel.iter().enumerate() {
                    *(a as *mut u32) = antes_sel[i];
                }
            }
            // quadragesima quarta leva: bola rolando (impulso, direcoes nas bordas, saidas do campo)
            let h52 = s.proximo();
            *((m + 0xf582) as *mut usize) = if h52 % 9 == 0 { j } else { 0 };
            *((m + 0xf57a) as *mut usize) = if (h52 >> 4) % 9 == 0 { o } else { 0 };
            *((m + 0x8eaa) as *mut i8) = [0i8, 1, 1, 1, 2, 3, 5, -1][((h52 >> 8) % 8) as usize];
            *((m + 0x8eb2) as *mut u8) = if (h52 >> 11) % 6 == 0 { [1u8, 2, 3][((h52 >> 14) % 3) as usize] } else { 0 };
            *((m + 0x8ea9) as *mut i8) = [0i8, 1, 2, 7, -1][((h52 >> 16) % 5) as usize];
            let dirs = [0i16, 359, 40, 41, 49, 50, 130, 131, 139, 140, 220, 221, 229, 230, 310, 311, 319, 320, -10, 400];
            *((m + 0x8ed8) as *mut i16) = if (h52 >> 20) % 4 == 0 { ((h52 >> 24) % 420) as i16 - 30 } else { dirs[((h52 >> 33) % 20) as usize] };
            *((m + 0x8ea7) as *mut i8) = [0i8, 8, 4, 1, 7, -1, 9][((h52 >> 38) % 7) as usize];
            *((m + 0x8ea8) as *mut i8) = [0i8, 11, 5, 1, 10, -1, 12][((h52 >> 41) % 7) as usize];
            *((m + 0x8eab) as *mut i8) = [0i8, 1, 2][((h52 >> 44) % 3) as usize];
            *((m + 0x8eae) as *mut i8) = [0i8, 1, -1][((h52 >> 46) % 3) as usize];
            comparar_rolar_bola(m);
            // sexagesima setima leva: ordem dos jogadores (guarda a lista, a contagem e 0xb67a64)
            {
                let mut antes67 = [0u8; 4 * 40];
                core::ptr::copy_nonoverlapping((m + 0x8f7e) as *const u8, antes67.as_mut_ptr(), antes67.len());
                let (n67, g67) = (*((m + 0x8eb8) as *const u8), *(0xb67a64 as *const u32));
                comparar_ordem_dos_jogadores(m);
                core::ptr::copy_nonoverlapping(antes67.as_ptr(), (m + 0x8f7e) as *mut u8, antes67.len());
                *((m + 0x8eb8) as *mut u8) = n67;
                *(0xb67a64 as *mut u32) = g67;
            }
            // quadragesima quinta leva: teste de condicao (condicao 0..100, abaixo de 1 divide por zero)
            let h53 = s.proximo();
            let cond = [0.0f32, 0.5, 0.999, 1.0, 1.5, 100.0, 99.99, 10.0, 31.62, ((h53 >> 8) % 100_001) as f32 / 1000.0][((h53 >> 4) % 10) as usize];
            *((j + 0x1a2) as *mut f32) = cond;
            *((j + 0x1a6) as *mut usize) = if (h53 >> 30) % 2 == 0 { o } else { 0 };
            let alvo53 = [0usize, o, j][((h53 >> 32) % 3) as usize];
            let cod53 = [1i8, 2, 3, 4, 5, 0, 6, -1, 1, 2, 5][((h53 >> 35) % 11) as usize];
            comparar_teste_de_condicao(j, cod53, alvo53);
            // quadragesima sexta leva: preferencias (nulos, cada uma das 3 vagas, ausente) e marca por posicao
            let h54 = s.proximo();
            let pf = prefs46.as_mut_ptr() as usize;
            let pe46 = pessoa46.as_mut_ptr() as usize;
            let cand = [j, o, 0, 0x1234usize]; // 0: vaga vazia (o alvo nulo nao pode casar com ela)
            for k in 0..6usize {
                *((pf + 0x1c + 4 * k) as *mut usize) = cand[((h54 >> (k * 2)) % 4) as usize];
            }
            *((pe46 + 0x65) as *mut usize) = if (h54 >> 12) % 6 == 0 { 0 } else { pf };
            let alvo54 = [0usize, j, o, 0, 0x1234][((h54 >> 15) % 5) as usize];
            let quem54 = if (h54 >> 18) % 7 == 0 { 0 } else { pe46 };
            comparar_gosta_de(alvo54, quem54);
            comparar_nao_gosta_de(alvo54, quem54);
            // septuagesima segunda leva: clubes preferidos (+0x04..+0x0c) e evitados (+0x10..+0x18)
            for k in 0..6usize {
                *((pf + 4 + 4 * k) as *mut usize) = cand[((h54 >> (k * 2 + 22)) % 4) as usize];
            }
            comparar_gosta_do_clube(quem54, alvo54);
            comparar_nao_gosta_do_clube(quem54, alvo54);
            let base54 = m + 0x91e2; // lado 0: +0x187f ainda dentro do motor falso
            *((base54 + 0x187f) as *mut u32) = (h54 >> 8) as u32 | 1;
            let v54 = (h54 >> 20) as u32;
            let ficha54: *const u32 = if (h54 >> 52) % 4 == 0 { core::ptr::null() } else { &v54 };
            comparar_marcar_posicao(base54, ((h54 >> 54) % 17) as i8 - 1, ficha54);
            // quadragesima oitava leva: folhas de outros modulos (globais do jogo trocadas durante o teste)
            let h55 = s.proximo();
            let globais = [0xae2378usize, 0xae2364, 0xae2d0c, 0xae235c, 0xae2350];
            let antes55: Vec<u32> = globais.iter().map(|&a| *(a as *const u32)).collect();
            comparar_unidades([0i32, 9, 10, 999, 1000, 1001, -1, -10, -999, i32::MIN, ((h55 >> 3) % 3000) as i32 - 1000][((h55 >> 60) % 11) as usize]);
            let ncomp = 200 + ((h55 >> 8) % 100) as i32;
            *(0xae2378 as *mut i32) = ncomp;
            let mut comp55 = [0i32; 2];
            comp55[0] = [ncomp - 0x7f, ncomp - 0x80, ncomp, 0, -5, ((h55 >> 16) % 400) as i32][((h55 >> 24) % 6) as usize];
            let obj55 = [0usize, if (h55 >> 27) % 4 == 0 { comp55.as_ptr() as usize } else { 0x4321 }];
            comparar_e_amistoso(obj55.as_ptr() as usize, comp55.as_ptr() as usize);
            let npes = 50 + ((h55 >> 30) % 20) as i32;
            *(0xae2364 as *mut i32) = npes;
            for k in 0..80usize {
                *((tabela55.as_mut_ptr() as usize + 79 * k) as *mut i32) = if (h55 >> (k % 40)) % 5 == 0 { -1 - (k as i32 % 3) } else { (k * 3) as i32 };
            }
            *(0xae2d0c as *mut usize) = tabela55.as_mut_ptr() as usize;
            let pessoa55 = [[0i32, 1, npes - 1, npes, npes + 3, 2, ((h55 >> 36) % 70) as i32][((h55 >> 40) % 7) as usize]];
            let contratos55 = [0x10000usize, 0];
            comparar_contrato_de(contratos55.as_ptr() as usize, if (h55 >> 43) % 6 == 0 { 0 } else { pessoa55.as_ptr() as usize });
            *(0xae235c as *mut i32) = 5000 + ((h55 >> 46) % 100) as i32;
            *(0xae2350 as *mut i32) = 200 + ((h55 >> 50) % 50) as i32;
            let limite55 = *(0xae235c as *const i32) - 2 * *(0xae2350 as *const i32);
            let clube55 = [[limite55, limite55 - 1, limite55 + 1, 0, 9999, -3][((h55 >> 54) % 6) as usize]];
            comparar_e_selecao(clube55.as_ptr() as usize);
            for (i, &a) in globais.iter().enumerate() {
                *(a as *mut u32) = antes55[i];
            }
            // quadragesima nona leva: tecnico humano (indices nas bordas, sem pessoa pelo tecnico atual),
            // codigos de casa (todos os validos e vizinhos) e mascaras de formacao
            let h56 = s.proximo();
            let globais56 = [0xae2364usize, 0xb77c7a];
            let antes56: Vec<u32> = globais56.iter().map(|&a| *(a as *const u32)).collect();
            let npes56 = 1000 + ((h56 >> 2) % 50) as i32;
            *(0xae2364 as *mut i32) = npes56;
            let atual56 = ((h56 >> 8) % 4) as i16;
            *(0xb77c7a as *mut i16) = atual56;
            let tab56 = 0xb74c26 + 768 * atual56 as usize;
            let antes_tab56 = *(tab56 as *const u32);
            let ntec = 1 + ((h56 >> 12) % 16) as i32;
            let regs56 = tecnicos56.as_mut_ptr() as usize;
            for k in 0..16usize {
                let r = regs56 + 42735 * k;
                *((r + 0x9a01) as *mut u32) = (h56 as u32).rotate_left(k as u32);
                *((r + 0x9a05) as *mut u8) = if (h56 >> (k + 20)) % 3 == 0 { 0 } else { (k as u8) | 0x40 };
            }
            let obj56 = [regs56, ntec as usize];
            let id56 = [[npes56 - 16, npes56 - 17, npes56 - 16 + ntec - 1, npes56 - 16 + ntec, npes56 - 1, npes56 - 20 + ((h56 >> 40) % 30) as i32][((h56 >> 44) % 6) as usize]];
            *(tab56 as *mut usize) = id56.as_ptr() as usize;
            let quem56 = if (h56 >> 47) % 4 == 0 { 0 } else { id56.as_ptr() as usize };
            comparar_tecnico_9a01(obj56.as_ptr() as usize, quem56);
            let dentro56 = id56[0] - npes56 + 16;
            if (0..ntec).contains(&dentro56) {
                // sexagesima segunda leva: 0x9ace e 0x9a06 (pessoa com clube e nacao de +0x88 nas bordas)
                let mut nacao62 = [0u8; 0x90];
                *(nacao62.as_mut_ptr().add(0x88) as *mut i32) = [399, 400, 0, -1, 401, 5000][((h56 >> 50) % 6) as usize];
                let mut clube62 = [0u8; 0x60];
                *(clube62.as_mut_ptr().add(0x53) as *mut usize) = nacao62.as_ptr() as usize;
                let mut pessoa62 = [0u32; 16];
                pessoa62[0] = id56[0] as u32;
                pessoa62[9] = if (h56 >> 53) % 3 == 0 { 0 } else { clube62.as_ptr() as u32 };
                for k in 0..16usize {
                    let r = regs56 + 42735 * k;
                    *((r + 0x9ace) as *mut u32) = (h56 as u32).rotate_right(k as u32);
                    *((r + 0x9a06) as *mut u8) = if (h56 >> (k + 24)) % 3 == 0 { 0 } else { 0x80 | k as u8 };
                }
                *(tab56 as *mut usize) = pessoa62.as_ptr() as usize;
                // o tecnico 0 com outra pessoa valida: pegar a vaga errada aparece
                let mut outra62 = [0u32; 16];
                outra62[0] = (npes56 - 16 + (dentro56 + 1) % ntec) as u32;
                let antes_vaga0 = *(0xb74c26 as *const u32);
                if atual56 != 0 {
                    *(0xb74c26 as *mut usize) = outra62.as_ptr() as usize;
                }
                let quem62 = if (h56 >> 47) % 4 == 0 { 0 } else { pessoa62.as_ptr() as usize };
                comparar_tecnico_9ace(obj56.as_ptr() as usize, quem62);
                for k in 0..16usize {
                    for b in 0..0x30usize {
                        *((regs56 + 42735 * k + 0x9b20 + b) as *mut u8) = (h56.rotate_left((k + b) as u32) as u8) ^ (b as u8);
                    }
                }
                comparar_tecnico_9b23(obj56.as_ptr() as usize, ((h56 >> 57) % 40) as i8 - 3, quem62);
                let mut saida62 = 7u32;
                let sp = if (h56 >> 55) % 2 == 0 { core::ptr::null_mut() } else { &mut saida62 as *mut u32 };
                comparar_tecnico_9a06(obj56.as_ptr() as usize, quem62, sp);
                // sexagesima terceira leva: clube controlado por humano (clube, selecao e segunda selecao
                // que volta para a principal; tecnico do clube, outro ou nenhum; com e sem tatica)
                let h63 = s.proximo();
                let globais63 = [0xb63c98usize, 0xae235c, 0xae2350, 0xae23b4, 0xae23a8];
                let antes63: Vec<u32> = globais63.iter().map(|&a| *(a as *const u32)).collect();
                *(0xb63c98 as *mut usize) = obj56.as_ptr() as usize;
                *(0xae235c as *mut i32) = 5000;
                *(0xae2350 as *mut i32) = 200;
                for k in 0..16usize {
                    let r = regs56 + 42735 * k;
                    if (h63 >> k) & 1 == 0 {
                        *((r + 0x9ace) as *mut u32) = 0;
                    }
                    if (h63 >> (k + 16)) & 1 == 0 {
                        *((r + 0x9a01) as *mut u32) = 0;
                    }
                }
                let tecnicos63 = [0usize, pessoa62.as_ptr() as usize, outra62.as_ptr() as usize];
                let mut clube_b63 = [0u8; 0x100];
                *(clube_b63.as_mut_ptr() as *mut i32) = 100;
                *(clube_b63.as_mut_ptr().add(0xcf) as *mut usize) = tecnicos63[((h63 >> 34) % 3) as usize];
                let nacao63 = [0i32; 4];
                *(0xae23b4 as *mut usize) = (clube_b63.as_ptr() as usize).wrapping_sub(581 * 4600);
                *(0xae23a8 as *mut u32) = 1;
                let mut clube_a63 = [0u8; 0x100];
                *(clube_a63.as_mut_ptr() as *mut i32) = [100i32, 4650, 4850][((h63 >> 36) % 3) as usize];
                *(clube_a63.as_mut_ptr().add(0xcf) as *mut usize) = tecnicos63[((h63 >> 38) % 3) as usize];
                *(clube_a63.as_mut_ptr().add(0x53) as *mut usize) = nacao63.as_ptr() as usize;
                comparar_clube_humano(
                    clube_a63.as_ptr() as usize,
                    ((h63 >> 40) & 1) as u32,
                    ((h63 >> 41) & 1) as u8,
                    tecnicos63[((h63 >> 42) % 3) as usize],
                    ((h63 >> 44) % 3 == 0) as u32,
                );
                // octogesima segunda leva: classe da selecao (o clube_a63 passa pela 0x5409b0 como no 63)
                comparar_classe_da_selecao(clube_a63.as_ptr() as usize, ((h63 >> 45) & 1) as u32);
                // octogesima terceira leva: classe pela selecao da pessoa (sem selecao ou com o clube_a63)
                let mut pessoa83 = [0u8; 0x60];
                *(pessoa83.as_mut_ptr().add(0x24) as *mut usize) = if (h63 >> 47) % 4 == 0 { 0 } else { clube_a63.as_ptr() as usize };
                pessoa83[0x5e] = (h63 >> 50) as u8;
                comparar_classe_pela_selecao(pessoa83.as_ptr() as usize);
                // octogesima quarta leva: bytes do bloco da selecao, selecoes especiais (0x9d00a4/a8 com os
                // ids do clube_a63; o 100 nao e selecao, so ele separa o 0x9d00a8) e a pessoa que nao gosta da selecao principal (o clube_b63)
                let mut blocos84 = vec![0u8; 110 * 2];
                for (k, b) in blocos84.iter_mut().enumerate() {
                    *b = (h63.rotate_left(k as u32 % 64) as u8) ^ (k as u8).wrapping_mul(7);
                }
                let obj84 = [0usize, blocos84.as_mut_ptr() as usize];
                comparar_byte_42_da_selecao(obj84.as_ptr() as usize, clube_a63.as_ptr() as usize, ((h63 >> 48) & 1) as u32);
                comparar_byte_43_da_selecao(obj84.as_ptr() as usize, clube_a63.as_ptr() as usize, ((h63 >> 49) & 1) as u32);
                let especiais84 = (*(0x9d00a4 as *const u32), *(0x9d00a8 as *const u32));
                *(0x9d00a4 as *mut u32) = [4650u32, 7, 9][((h63 >> 51) % 3) as usize];
                *(0x9d00a8 as *mut u32) = [100u32, 8][((h63 >> 53) % 2) as usize];
                comparar_selecao_ou_especial(clube_a63.as_ptr() as usize);
                *(0x9d00a4 as *mut u32) = especiais84.0;
                *(0x9d00a8 as *mut u32) = especiais84.1;
                let mut pref84 = [0usize; 8];
                pref84[4 + ((h63 >> 54) % 3) as usize] = if (h63 >> 56) & 1 == 0 { clube_b63.as_ptr() as usize } else { 1 };
                let mut pessoa84 = [0u8; 0x70];
                *(pessoa84.as_mut_ptr().add(0x1a) as *mut usize) = nacao63.as_ptr() as usize;
                *(pessoa84.as_mut_ptr().add(0x65) as *mut usize) = pref84.as_ptr() as usize;
                comparar_nao_gosta_da_selecao(pessoa84.as_ptr() as usize);
                // septuagesima quinta leva: nevoa de guerra (data de hoje, quadro de conhecimento,
                // pessoas sinteticas; os dois lados com o clube humano sintetico)
                {
                    let h75 = s.proximo();
                    let globais75 = [0xae2a60usize, 0xae23bc, 0xae2c90, 0xae2c94];
                    let antes75: Vec<u32> = globais75.iter().map(|&a| *(a as *const u32)).collect();
                    *(0xae2c90 as *mut u32) = (h75 >> 3) as u32;
                    *(0xae2c94 as *mut u32) = (h75 >> 29) as u32;
                    let mut conhec75 = vec![0u8; 9 * 1100];
                    for (k, b) in conhec75.iter_mut().enumerate() {
                        *b = if k % 9 == 0 { (h75.rotate_left(k as u32 % 61) % 4) as u8 } else { (k as u8).wrapping_mul(37) };
                    }
                    let obj75 = [conhec75.as_mut_ptr() as usize, 0];
                    let mut quadro75 = [0usize; 32];
                    for k in 0..32usize {
                        quadro75[k] = if (h75 >> (k % 40 + 5)) % 5 == 0 { 0 } else { obj75.as_ptr() as usize };
                    }
                    *(0xae2a60 as *mut usize) = quadro75.as_ptr() as usize;
                    let mut pessoas75 = vec![0u8; 0x6e * 64];
                    for k in 0..64usize {
                        let z = h75.rotate_left(k as u32) ^ (k as u64 * 0x9e37);
                        *(pessoas75.as_mut_ptr().add(0x6e * k) as *mut i32) = if z % 9 == 0 { npes56 + ((z >> 8) % 3) as i32 } else { (k * 13) as i32 % npes56 };
                    }
                    *(0xae23bc as *mut usize) = pessoas75.as_ptr() as usize;
                    let mut dados75 = vec![0u8; 0x1000];
                    for lado in 0..2usize {
                        for v in 0..20usize {
                            let z = h75.rotate_right((lado * 20 + v) as u32) ^ (v as u64 * 0x5bd1);
                            let e = dados75.as_mut_ptr().add(lado * 0x625 + 0x190 + v * 0x49);
                            *(e as *mut i32) = if z % 7 == 0 { -1 } else { ((z >> 4) % 64) as i32 };
                            *(e.add(4) as *mut i32) = if (z >> 12) % 7 == 0 { -1 } else { 5 };
                            *e.add(0x32) = if (z >> 16) % 3 == 0 { 0xff } else { 1 };
                        }
                    }
                    let mut partida75 = [0usize; 9];
                    partida75[7] = clube_a63.as_ptr() as usize;
                    // o lado 1 (+0x1c) com outro tecnico (ou nenhum): trocar os lados aparece
                    let mut clube_1_75 = clube_a63;
                    *(clube_1_75.as_mut_ptr().add(0xcf) as *mut usize) = tecnicos63[((h75 >> 1) % 3) as usize];
                    partida75[8] = clube_a63.as_ptr() as usize;
                    partida75[7] = clube_1_75.as_ptr() as usize;
                    comparar_nevoa_apos_partida(partida75.as_ptr() as usize, dados75.as_ptr() as usize);
                    // a pessoa direto, com e sem observador (o sorteio so entra com ele)
                    let mut obs75 = [0u8; 0x20];
                    *(obs75.as_mut_ptr().add(4) as *mut i16) = if (h75 >> 40) & 1 == 0 { (h75 >> 41) as i16 } else { ((h75 >> 41) % 1200) as i16 - 600 };
                    obs75[0x17] = (h75 >> 57) as u8;
                    let mut fonte75 = [0u8; 0x70];
                    *(fonte75.as_mut_ptr().add(0x69) as *mut usize) = if (h75 >> 58) % 4 == 0 { 0 } else { obs75.as_ptr() as usize };
                    let quem75 = [[((h75 >> 20) % npes56 as u64) as i32, npes56, npes56 - 1, 0][((h75 >> 60) % 4) as usize]];
                    comparar_conhecer_pessoa(obj75.as_ptr() as usize, if (h75 >> 62) & 1 == 1 && (h75 >> 61) & 1 == 1 { 0 } else { quem75.as_ptr() as usize }, if (h75 >> 59) % 5 == 0 { 0 } else { fonte75.as_ptr() as usize });
                    comparar_chance_de_conhecer(obs75.as_ptr() as usize, (h75 >> 33) as i8);
                    // septuagesima sexta leva: lista de observacao (ids repetidos em +0xf e +0x13, tipos
                    // 11 e outros) e bandeiras do contrato
                    let h76 = s.proximo();
                    let mut entradas76 = [0u8; 40 * 37];
                    for k in 0..40usize {
                        let z = h76.rotate_left(k as u32 * 3) ^ (k as u64 * 0x2545);
                        entradas76[37 * k] = if z % 3 == 0 { 11 } else { (z >> 2) as u8 % 14 };
                        *(entradas76.as_mut_ptr().add(37 * k + 0xf) as *mut u32) = 700 + ((z >> 8) % 6) as u32;
                        *(entradas76.as_mut_ptr().add(37 * k + 0x13) as *mut u32) = 700 + ((z >> 16) % 6) as u32;
                    }
                    let id76 = ((h76 >> 30) % 20) as usize;
                    let mut listas76 = [0u8; 9 * 20 + 4];
                    *(listas76.as_mut_ptr().add(9 * id76) as *mut usize) = entradas76.as_ptr() as usize;
                    let mut gerente76 = [0u8; 0xe0];
                    *(gerente76.as_mut_ptr().add(0xdc) as *mut usize) = listas76.as_ptr() as usize;
                    let pessoa76 = [id76 as u32];
                    let alvo76 = [700 + ((h76 >> 40) % 8) as u32];
                    comparar_entrada_da_lista(gerente76.as_ptr() as usize, pessoa76.as_ptr() as usize, alvo76.as_ptr() as usize, ((h76 >> 50) & 1) as u32, ((h76 >> 51) & 1) as u32);
                    let mut contrato76 = [0u32; 0x14];
                    for (k, c) in contrato76.iter_mut().enumerate() {
                        *c = (h76 as u32).rotate_left(k as u32) ^ k as u32;
                    }
                    comparar_bandeiras_do_contrato(contrato76.as_ptr() as usize);
                    // septuagesima setima leva: lista encadeada (nos de 10 bytes, desalinhados), palavra
                    // +0xc em registros de 31 bytes, arredondamento e nacoes do grupo
                    let h77 = s.proximo();
                    let mut nos77 = [0u8; 10 * 4 + 1];
                    let base77 = nos77.as_mut_ptr() as usize + 1;
                    for k in 0..4usize {
                        core::ptr::write_unaligned((base77 + 10 * k) as *mut u32, (h77 >> (8 * k)) as u32);
                        core::ptr::write_unaligned((base77 + 10 * k + 6) as *mut usize, if k == 3 || (h77 >> (k + 40)) % 4 == 0 { 0 } else { base77 + 10 * (k + 1) });
                    }
                    let mut cursor77 = [0u8; 0x10];
                    core::ptr::write_unaligned(cursor77.as_mut_ptr().add(0xa) as *mut usize, if (h77 >> 45) % 5 == 0 { 0 } else { base77 + 10 * ((h77 >> 46) % 4) as usize });
                    for _ in 0..3 {
                        comparar_proximo_da_lista(cursor77.as_mut_ptr() as usize);
                    }
                    let n77 = *(0xae2364 as *const i32);
                    let mut regs77 = vec![0u8; 31 * 8];
                    for (k, b) in regs77.iter_mut().enumerate() {
                        *b = (h77.rotate_left(k as u32 % 64) as u8) ^ (k as u8).wrapping_mul(29);
                    }
                    let id77 = [n77 - 1, n77, n77 + 1, n77 - 8 + ((h77 >> 50) % 8) as i32][((h77 >> 48) % 4) as usize];
                    let tab77 = [(regs77.as_ptr() as usize).wrapping_sub(31usize.wrapping_mul((n77 - 8) as usize))];
                    comparar_palavra_0c(tab77.as_ptr() as usize, id77);
                    let v77 = [(h77 >> 10) as i32, 990 + ((h77 >> 20) % 20) as i32, -(((h77 >> 30) % 30000) as i32), ((h77 >> 3) as i32 & !0xffff) | 999][((h77 >> 56) % 4) as usize];
                    comparar_arredondar(v77);
                    let globais77 = [0x9cf794usize, 0x9cf684, 0x9cf798, 0x9cf6b0, 0x9cf5d8, 0x9cf548];
                    let antes77: Vec<u32> = globais77.iter().map(|&a| *(a as *const u32)).collect();
                    for (k, &a) in globais77.iter().enumerate() {
                        *(a as *mut u32) = 60 + k as u32;
                    }
                    let nacao77 = [58 + ((h77 >> 58) % 10) as u32];
                    comparar_nacao_do_grupo(if (h77 >> 62) == 0 { 0 } else { nacao77.as_ptr() as usize });
                    for (k, &a) in globais77.iter().enumerate() {
                        *(a as *mut u32) = antes77[k];
                    }
                    // septuagesima oitava leva: competicao que conta (3 globais, tabela de 107 bytes,
                    // objetos com fase, rodadas e a primeira rodada) e as comparacoes de ordenacao
                    let h78 = s.proximo();
                    let globais78 = [0x9cf6b0usize, 0x9cf5d8, 0x9cf548, 0xae23d0, 0xadadfc];
                    let antes78: Vec<u32> = globais78.iter().map(|&a| *(a as *const u32)).collect();
                    for k in 0..3usize {
                        *(globais78[k] as *mut i32) = 10 + k as i32;
                    }
                    let mut tabela78 = [0u8; 107 * 16];
                    let mut rodada78 = [0u8; 0x70];
                    let primeira78 = [rodada78.as_mut_ptr() as usize];
                    let mut objs78 = [[0u8; 0xb8]; 16];
                    let mut ptrs78 = [0usize; 16];
                    for k in 0..16usize {
                        let z = h78.rotate_left(k as u32 * 4) ^ (k as u64 * 0x9e3);
                        *(tabela78.as_mut_ptr().add(107 * k + 0x5d) as *mut u32) = if z % 4 == 0 { 0 } else { z as u32 | 1 };
                        let o = &mut objs78[k];
                        o[0x43] = [2u8, 2, 1, 0][((z >> 3) % 4) as usize];
                        *(o.as_mut_ptr().add(0x3e) as *mut u16) = if (z >> 5) % 5 == 0 { 0 } else { 3 };
                        *(o.as_mut_ptr().add(0xb1) as *mut usize) = if (z >> 8) % 6 == 0 { 0 } else { primeira78.as_ptr() as usize };
                        *(o.as_mut_ptr().add(4) as *mut i32) = 500 + ((z >> 12) % 3) as i32;
                        ptrs78[k] = if (z >> 16) % 7 == 0 { 0 } else { o.as_ptr() as usize };
                    }
                    *(rodada78.as_mut_ptr().add(0x57) as *mut i32) = 500 + ((h78 >> 40) % 3) as i32;
                    *(rodada78.as_mut_ptr().add(0x60) as *mut i32) = 500 + ((h78 >> 42) % 3) as i32;
                    *(0xae23d0 as *mut usize) = tabela78.as_ptr() as usize;
                    *(0xadadfc as *mut usize) = ptrs78.as_ptr() as usize;
                    comparar_competicao_conta(((h78 >> 44) % 16) as i32);
                    for (k, &a) in globais78.iter().enumerate() {
                        *(a as *mut u32) = antes78[k];
                    }
                    let mut fichas78 = [[0u8; 0x84]; 2];
                    for (k, f) in fichas78.iter_mut().enumerate() {
                        f[0x5f] = (h78 >> (k * 8 + 1)) as u8;
                        *(f.as_mut_ptr().add(0x80) as *mut i16) = (h78 >> (k * 16 + 20)) as i16;
                    }
                    let pa78 = [if (h78 >> 60) % 4 == 0 { 0 } else { fichas78[0].as_ptr() as usize }];
                    let pb78 = [if (h78 >> 62) == 0 { 0 } else { fichas78[1].as_ptr() as usize }];
                    comparar_ordem_por_5f(pa78.as_ptr(), pb78.as_ptr());
                    comparar_ordem_por_80_desc(pa78.as_ptr(), pb78.as_ptr());
                    comparar_ordem_por_80_cresc(pa78.as_ptr(), pb78.as_ptr());
                    let qa78 = [fichas78[0].as_ptr() as usize];
                    let qb78 = [fichas78[1].as_ptr() as usize];
                    comparar_ordem_por_80(qa78.as_ptr(), qb78.as_ptr());
                    // octogesima leva: quadros da nevoa (16, alguns nulos ou inativos; nevoa ligada ou
                    // nao) e a formacao (casas das onze mascaras trocadas, ou casas sorteadas)
                    let h80 = s.proximo();
                    let antes_nevoa80 = *(0x9cc270 as *const u8);
                    *(0x9cc270 as *mut u8) = ((h80 >> 1) % 4 != 0) as u8;
                    let mut niveis80 = vec![0u8; 9 * 70];
                    for (k, b) in niveis80.iter_mut().enumerate() {
                        *b = 1 + (k % 3) as u8;
                    }
                    let base80 = (niveis80.as_mut_ptr() as usize).wrapping_sub(9usize.wrapping_mul((n77 - 64) as usize));
                    let mut quadros80 = [[0usize; 2]; 16];
                    let mut ptrs80 = [0usize; 16];
                    for k in 0..16usize {
                        quadros80[k] = [base80, ((h80 >> (k + 3)) % 3 != 0) as usize];
                        ptrs80[k] = if (h80 >> (k + 20)) % 4 == 0 { 0 } else { quadros80[k].as_ptr() as usize };
                    }
                    let pessoa80 = [n77 - 64 + ((h80 >> 40) % 66) as i32];
                    comparar_esquecer_pessoa(ptrs80.as_ptr() as usize, if (h80 >> 47) % 8 == 0 { 0 } else { pessoa80.as_ptr() as usize });
                    *(0x9cc270 as *mut u8) = antes_nevoa80;
                    let tecnico80 = [n77 - 1 - ((h80 >> 50) % 16) as i32];
                    comparar_quadro_do_tecnico(ptrs80.as_ptr() as usize, if (h80 >> 54) % 6 == 0 { 0 } else { tecnico80.as_ptr() as usize });
                    let codigos80: Vec<u16> = core::iter::once(1u16)
                        .chain([0x80u16, 0x100, 0x200, 0x400, 0x800].iter().flat_map(|&l| [2u16, 4, 8, 0x10, 0x20, 0x40].iter().map(move |&c| l | c)))
                        .collect();
                    let mapeadas80 = [0x101176c1u32, 0x101b26c1, 0x38115381, 0x3a242381, 0x280e07c1, 0x091a07c1, 0x2884ab81, 0x280aab81, 0x288c07c1, 0x281507c1, 0x288a8b81];
                    let mut casas80 = [0u16; 11];
                    if (h80 >> 56) % 2 == 0 {
                        let alvo = mapeadas80[((h80 >> 57) % 11) as usize];
                        let mut usados = 0u32;
                        let mut n = 0;
                        for &c in &codigos80 {
                            let b = bit_da_casa(c as i16);
                            if b & alvo != 0 && b & usados == 0 && n < 11 {
                                casas80[n] = c;
                                usados |= b;
                                n += 1;
                            }
                        }
                    } else {
                        for k in 0..11usize {
                            let z = h80.rotate_left(k as u32 * 5);
                            casas80[k] = if z % 9 == 0 { 0x1234 } else { codigos80[(z >> 4) as usize % codigos80.len()] };
                        }
                    }
                    let mut form80 = vec![0u8; 0x1880];
                    for k in 0..11usize {
                        core::ptr::write_unaligned(form80.as_mut_ptr().add(0x12d + 2 * k) as *mut u16, casas80[k]);
                        form80[0x5b4 + k] = (h80 >> (k * 3)) as u8 & 0x1e;
                        core::ptr::write_unaligned(form80.as_mut_ptr().add(0x5bf + 4 * k) as *mut u32, (h80 >> k) as u32);
                    }
                    let f80 = form80.as_mut_ptr() as usize;
                    comparar_mascara_da_formacao(f80);
                    let codigo80 = if (h80 >> 60) % 3 == 0 { 0x1234 } else { casas80[((h80 >> 33) % 11) as usize] };
                    comparar_vaga_da_casa(f80, codigo80);
                    let mut jogadores80 = vec![0u8; 20 * 0x1be];
                    for i in 0..20usize {
                        let z = h80.rotate_right(i as u32 * 3);
                        jogadores80[i * 0x1be + 0x19] = [0xfeu8, 0xff, (z % 11) as u8, (z % 13) as u8][((z >> 8) % 4) as usize];
                        jogadores80[i * 0x1be + 0x1a] = ((z >> 12) % 12) as u8;
                    }
                    // sempre com um bit de setor (1..0x40): sem ele a original mostra a caixa de erro do formation.cpp
                    let mascara80 = ((h80 >> 13) as u32 & 0xfff) | 0x200 | 1 << ((h80 >> 5) % 7);
                    comparar_casa_do_setor_ocupada(f80, mascara80, if (h80 >> 62) == 0 { 0 } else { jogadores80.as_ptr() as usize });
                    comparar_limpar_designados(f80);
                    // octogesima primeira leva: casa da vaga (so vagas 0..10: fora delas a original mostra a
                    // caixa de erro) e o criterio pelos bytes do registro (as palavras pedem a configuracao
                    // de 0xae3138, que ainda nao existe aqui; ficam com o jogo real)
                    let h81 = s.proximo();
                    for k in 0..0x30usize {
                        form80[0x12d + k] = (h81 >> (k % 56)) as u8 ^ (k as u8).wrapping_mul(13);
                    }
                    comparar_casa_da_vaga(f80, ((h81 >> 2) % 11) as i8, ((h81 >> 6) & 1) as u32);
                    let mut registro81 = [0u8; 0x60];
                    registro81[0x37] = (h81 >> 8) as u8;
                    registro81[0x52] = (h81 >> 16) as u8;
                    let a81 = ((h81 >> 24) as u32) & 0xffff_0000;
                    let b81 = ((h81 >> 40) as u32) & 0xffff_0000;
                    // octogesima segunda leva: tipos, marcas por pessoa e o registro livre (tabela sintetica,
                    // indice guardado ou nao; nunca 0 registros: a 0x90cfc0(0) nao e caso do jogo)
                    let h82 = s.proximo();
                    comparar_tipo_nao_11(((h82 >> 1) % 20) as i8 - 2);
                    comparar_tipo_11_a_15(((h82 >> 6) % 20) as i8 - 2);
                    let mut marcas82 = vec![0u8; 110 * 12 + 0x100];
                    for (k, b) in marcas82.iter_mut().enumerate() {
                        *b = (h82.rotate_left(k as u32 % 64) as u8) | 1;
                    }
                    let obj82 = [0usize, marcas82.as_mut_ptr() as usize + 0x40];
                    let pessoa82 = [((h82 >> 11) % 10) as i32];
                    let k82 = ((h82 >> 15) % 9) as i8 - 2;
                    comparar_ler_marca_da_pessoa(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, k82);
                    comparar_zerar_marca_da_pessoa(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, ((h82 >> 19) % 9) as i8 - 2);
                    // octogesima terceira leva: +0x44 (ligar e ler, com sinal) e o endereco do +0x3a
                    let h83 = s.proximo();
                    for (k, b) in marcas82.iter_mut().enumerate() {
                        *b = (h83.rotate_left(k as u32 % 64) as u8) ^ (k as u8);
                    }
                    let k83 = ((h83 >> 3) % 9) as i8 - 2;
                    comparar_byte_44_da_pessoa(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, k83);
                    comparar_ligar_44_da_pessoa(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, ((h83 >> 7) % 9) as i8 - 2);
                    comparar_campo_3a_da_pessoa(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, ((h83 >> 11) % 9) as i8 - 2);
                    let antes83 = *(0x9b5984 as *const u32);
                    *(0x9b5984 as *mut u32) = (h83 >> 20) as u32;
                    comparar_global_9b5984();
                    // octogesima quarta leva: competicao da pessoa (indice +4 negativo ou nao) e data com marca
                    let h84 = s.proximo();
                    let antes84 = *(0xae23d0 as *const usize);
                    *(0xae23d0 as *mut usize) = 0x0200_0000;
                    for k in 0..10usize {
                        *(marcas82.as_mut_ptr().add(0x40 + 110 * k + 4) as *mut i32) = if (h84 >> k) % 4 == 0 { -1 - k as i32 } else { (h84 >> (k + 10)) as i32 & 0xff };
                    }
                    comparar_competicao_da_pessoa(obj82.as_ptr() as usize, if (h84 >> 30) % 6 == 0 { 0 } else { pessoa82.as_ptr() as usize });
                    *(0xae23d0 as *mut usize) = antes84;
                    let origem84: [u8; 12] = core::array::from_fn(|k| (h84 >> (k * 5)) as u8);
                    let mut destino84 = [0u8; 12];
                    comparar_copiar_data_e_marca(destino84.as_mut_ptr() as usize, origem84.as_ptr() as usize);
                    *(0x9b5984 as *mut u32) = antes83;
                    let globais82 = [0x9b59b4usize, 0xae23a0, 0xae23f8];
                    let antes82: Vec<u32> = globais82.iter().map(|&a| *(a as *const u32)).collect();
                    let n82 = 1 + ((h82 >> 23) % 8) as i32;
                    let mut tabela82 = vec![1u8; 60 * 9];
                    if (h82 >> 27) % 3 != 0 {
                        tabela82[60 * ((h82 >> 29) % 9) as usize] = 0;
                    }
                    *(0xae23a0 as *mut i32) = n82;
                    *(0xae23f8 as *mut usize) = tabela82.as_ptr() as usize;
                    *(0x9b59b4 as *mut i32) = if (h82 >> 33) % 3 == 0 { ((h82 >> 35) % 9) as i32 } else { -1 };
                    comparar_registro_livre();
                    for (k, &a) in globais82.iter().enumerate() {
                        *(a as *mut u32) = antes82[k];
                    }
                    comparar_criterio_da_competicao(if (h81 >> 60) % 5 == 0 { 0 } else { registro81.as_ptr() as usize }, a81, b81, [1u8, 0, 2, 0x81][((h81 >> 61) % 4) as usize]);
                    for (k, &a) in globais75.iter().enumerate() {
                        *(a as *mut u32) = antes75[k];
                    }
                }
                // sexagesima quinta leva: linha e byte da tabela tatica (tabela unica, cobre ids e humanos)
                if TABELA65.is_empty() {
                    TABELA65 = vec![0u8; 0x91 * 5100];
                }
                for k in 0..0x91 * 40 {
                    TABELA65[0x91 * 4990 + k] = (h63 >> (k % 50)) as u8 ^ k as u8;
                }
                for &l in &[100usize, 4650, 4850] {
                    for k in 0..0x91 {
                        TABELA65[0x91 * l + k] = (h63 >> (k % 40)) as u8 ^ (l as u8);
                    }
                }
                let humano65 = ((h63 >> 54) & 1) as u32;
                comparar_linha_tatica(clube_a63.as_ptr() as usize, humano65);
                let mut obj65 = [0u8; 0x608];
                *(obj65.as_mut_ptr().add(0x601) as *mut usize) = TABELA65.as_ptr() as usize;
                comparar_byte_tatico(obj65.as_mut_ptr() as usize, clube_a63.as_ptr() as usize, ((h63 >> 56) % 11) as i8, humano65);
                // sexagesima oitava leva: vagas da linha tatica (indices em +2.., +0x52, +0x56) sobre
                // 30 pessoas sinteticas de 110 bytes; a tabela e restaurada (marcar_vaga escreve +0x52)
                {
                    let h68 = s.proximo();
                    let mut pessoas68 = [0u8; 110 * 30];
                    for k in 0..30usize {
                        *(pessoas68.as_mut_ptr().add(110 * k) as *mut i32) = 7000 + (k as i32 % 25);
                    }
                    let antes68 = *(0xae23bc as *const usize);
                    *(0xae23bc as *mut usize) = pessoas68.as_ptr() as usize;
                    let linhas68: Vec<usize> = [100usize, 4650, 4850].into_iter().chain(4999..5033).collect();
                    let copia68: Vec<[u8; 0x91]> = linhas68.iter().map(|&l| {
                        let mut c = [0u8; 0x91];
                        c.copy_from_slice(&TABELA65[0x91 * l..0x91 * l + 0x91]);
                        c
                    }).collect();
                    for (n, &l) in linhas68.iter().enumerate() {
                        let r = TABELA65.as_mut_ptr().add(0x91 * l) as usize;
                        for v in 0..20usize {
                            let z = h68.rotate_left((v + n) as u32);
                            *((r + 2 + 4 * v) as *mut i32) = if z % 5 == 0 { -1 } else { ((z >> 8) % 30) as i32 };
                        }
                        *((r + 0x52) as *mut i32) = 7000 + ((h68 >> (n + 3)) % 30) as i32;
                        *((r + 0x56) as *mut i32) = [-1i32, 0, 5, 29, -7][((h68 >> (n + 9)) % 5) as usize];
                        *((r + 0x5a) as *mut i32) = [-1i32, 3, 12][((h68 >> (n + 17)) % 3) as usize];
                    }
                    let o68 = obj65.as_mut_ptr() as usize;
                    let c68 = clube_a63.as_ptr() as usize;
                    comparar_vaga_marcada(o68, c68, humano65, ((h68 >> 40) & 1) as u32);
                    comparar_marcar_vaga(o68, c68, ((h68 >> 41) % 20) as i8, humano65);
                    comparar_pessoa_da_linha(o68, c68, ((h68 >> 46) % 2) as i8, humano65);
                    for (n, &l) in linhas68.iter().enumerate() {
                        TABELA65[0x91 * l..0x91 * l + 0x91].copy_from_slice(&copia68[n]);
                    }
                    *(0xae23bc as *mut usize) = antes68;
                }
                // sexagesima nona leva: clube na competicao e clube especial
                {
                    let h69 = s.proximo();
                    let mut cont69 = [[0u8; 0x80]; 2];
                    *(cont69[1].as_mut_ptr() as *mut i32) = 1;
                    let mut nac69 = [[0u8; 0x80]; 3];
                    for (k, n) in nac69.iter_mut().enumerate() {
                        *(n.as_mut_ptr().add(0x71) as *mut usize) = cont69[k % 2].as_ptr() as usize;
                    }
                    let mut base69 = [0u8; 0x80];
                    *(base69.as_mut_ptr().add(0x59) as *mut usize) = if (h69 >> 2) % 4 == 0 { 0 } else { cont69[0].as_ptr() as usize };
                    *(base69.as_mut_ptr().add(0x5d) as *mut usize) = nac69[(h69 % 3) as usize].as_ptr() as usize;
                    let mut clubes69 = [[0u8; 0x60]; 6];
                    for (k, c) in clubes69.iter_mut().enumerate() {
                        let z = h69.rotate_left(7 * k as u32);
                        *(c.as_mut_ptr() as *mut i32) = [100i32, 4799, 4800, 4900, 4650][(z % 5) as usize];
                        *(c.as_mut_ptr().add(0x53) as *mut usize) = if (z >> 3) % 5 == 0 { 0 } else { nac69[((z >> 5) % 3) as usize].as_ptr() as usize };
                        *(c.as_mut_ptr().add(0x57) as *mut usize) = [0usize, base69.as_ptr() as usize, 0x1234][((z >> 8) % 3) as usize];
                    }
                    let alvo69 = clubes69[((h69 >> 10) % 6) as usize].as_ptr() as usize;
                    // fases: participantes (6 bytes) e tabelas (0x3b) com alguns dos clubes
                    let mut listas69 = [[0u8; 6 * 4]; 4];
                    let mut tabelas69 = [[0u8; 0x3b * 4]; 4];
                    let mut fases69 = [[0u8; 0xc0]; 4];
                    let mut comp69 = [0u8; 0xc0];
                    for f in 0..4usize {
                        let z = h69.rotate_right(9 * f as u32);
                        for k in 0..4usize {
                            let c = clubes69[((z >> (3 * k)) % 6) as usize].as_ptr() as usize;
                            *(listas69[f].as_mut_ptr().add(6 * k) as *mut usize) = c;
                            *(tabelas69[f].as_mut_ptr().add(0x3b * k) as *mut usize) = c;
                        }
                        fases69[f][0x43] = [0u8, 1, 2, 2][((z >> 20) % 4) as usize];
                        *(fases69[f].as_mut_ptr().add(0x3e) as *mut i16) = [0i16, 1, 2, 4, -1][((z >> 22) % 5) as usize];
                        *(fases69[f].as_mut_ptr().add(0xa7) as *mut usize) = listas69[f].as_ptr() as usize;
                        *(fases69[f].as_mut_ptr().add(0xb1) as *mut usize) = tabelas69[f].as_ptr() as usize;
                    }
                    let ponteiros69: [usize; 4] = [0, 1, 2, 3].map(|f| fases69[f].as_ptr() as usize);
                    *(comp69.as_mut_ptr().add(4) as *mut usize) = base69.as_ptr() as usize;
                    *(comp69.as_mut_ptr().add(0xc) as *mut usize) = ponteiros69.as_ptr() as usize;
                    *(comp69.as_mut_ptr().add(0x30) as *mut i32) = [-2i32, -1, 0, 1, 3][((h69 >> 30) % 5) as usize];
                    comp69[0x43] = [0u8, 2][((h69 >> 33) % 2) as usize];
                    *(comp69.as_mut_ptr().add(0x3e) as *mut i16) = 3;
                    *(comp69.as_mut_ptr().add(0xa7) as *mut usize) = listas69[3].as_ptr() as usize;
                    *(comp69.as_mut_ptr().add(0xb1) as *mut usize) = tabelas69[3].as_ptr() as usize;
                    comparar_clube_na_competicao(comp69.as_mut_ptr() as usize, alvo69);
                    // clube especial: o id de um dos nove guardados, ou outro
                    // (antes de carregar a base os nove ids sao 0: valores distintos durante o teste)
                    let guardados = [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964];
                    let antes_g69: Vec<u32> = guardados.iter().map(|&a| *(a as *const u32)).collect();
                    for (k, &a) in guardados.iter().enumerate() {
                        *(a as *mut u32) = 3000 + 7 * k as u32;
                    }
                    let id69 = if (h69 >> 35) % 3 == 0 { 0x7fff_0000u32 } else { *(guardados[((h69 >> 37) % 9) as usize] as *const u32) };
                    let especial69 = [id69, 0];
                    comparar_clube_especial(if (h69 >> 41) % 7 == 0 { 0 } else { especial69.as_ptr() as usize });
                    // septuagesima terceira leva: clube parceiro (globais dos 82 pares e das 3 nacoes com
                    // valores distintos; alguns parceiros -1 e -5 para os dois testes de aceite)
                    {
                        let h73 = s.proximo();
                        let nac73 = [0x9cf488usize, 0x9cf314, 0x9cf434];
                        let mut todos73: Vec<usize> = nac73.to_vec();
                        for &(a, b) in PARES_A_B.iter().chain(PARES_B_A.iter()) {
                            todos73.push(a);
                            todos73.push(b);
                        }
                        let antes73: Vec<u32> = todos73.iter().map(|&a| *(a as *const u32)).collect();
                        for (k, &a) in nac73.iter().enumerate() {
                            *(a as *mut u32) = 900 + k as u32;
                        }
                        for (k, &(a, b)) in PARES_A_B.iter().enumerate() {
                            *(a as *mut i32) = if (h73 >> (k % 50 + 3)) % 11 == 0 { -5 } else { 1000 + k as i32 };
                            *(b as *mut i32) = match (h73 >> (k % 60)) % 9 { 0 => -1, 1 => -5, _ => 2000 + k as i32 };
                        }
                        let ids73: Vec<u32> = PARES_A_B.iter().map(|&(a, _)| *(a as *const u32))
                            .chain(PARES_A_B.iter().map(|&(_, b)| *(b as *const u32))).collect();
                        let mut nacao73 = [0u32; 4];
                        nacao73[0] = [900u32, 901, 902, 903][((h73 >> 2) % 4) as usize];
                        let mut clube73 = [0u8; 0x250];
                        *(clube73.as_mut_ptr() as *mut u32) = if (h73 >> 5) % 5 == 0 { 7777 } else { ids73[((h73 >> 8) as usize) % ids73.len()] };
                        *(clube73.as_mut_ptr().add(0x53) as *mut usize) = if (h73 >> 16) % 9 == 0 { 0 } else { nacao73.as_ptr() as usize };
                        clube73[0x244] = ((h73 >> 20) % 3 != 0) as u8;
                        let mut eh73 = 5u32;
                        let ponteiro73 = if (h73 >> 23) % 3 == 0 { core::ptr::null_mut() } else { &mut eh73 as *mut u32 };
                        let antes_base73 = *(0xae23b4 as *const u32);
                        *(0xae23b4 as *mut u32) = 0x0100_0000;
                        comparar_clube_parceiro(clube73.as_mut_ptr() as usize, ponteiro73, ((h73 >> 25) & 1) as u32);
                        // septuagesima quarta leva: contrato com o clube e nivel do contrato
                        if CLUBES74.is_empty() {
                            CLUBES74 = vec![0u8; 581 * 2200];
                            for k in 0..2200usize {
                                *(CLUBES74.as_mut_ptr().add(581 * k) as *mut i32) = k as i32 - 16;
                            }
                        }
                        // a partir da vaga 16: parceiros de indice negativo (-5) ainda caem dentro do vetor
                        *(0xae23b4 as *mut usize) = CLUBES74.as_ptr() as usize + 581 * 16;
                        let h74 = s.proximo();
                        let id_p74: i32 = [5i32, 40, 99999][((h74 >> 1) % 3) as usize];
                        let pessoa74 = [id_p74, 0, 0, 0];
                        let mut entrada74 = [0u8; 79];
                        *(entrada74.as_mut_ptr() as *mut i32) = [0i32, 1, 2, 3, -1][((h74 >> 3) % 5) as usize];
                        *(entrada74.as_mut_ptr().add(4) as *mut i32) = [0i32, 1, 2, 3, -1][((h74 >> 6) % 5) as usize];
                        let antes74 = (*(0xae2d0c as *const usize), *(0xae1050 as *const usize));
                        // limite das selecoes (clubes - 2 x nacoes) no meio dos ids dos pares
                        let limites74 = (*(0xae235c as *const i32), *(0xae2350 as *const i32));
                        *(0xae235c as *mut i32) = 2100;
                        *(0xae2350 as *mut i32) = 400 + ((h74 >> 20) % 700) as i32;
                        *(0xae2d0c as *mut usize) = (entrada74.as_ptr() as usize).wrapping_sub((id_p74.max(0) as usize).wrapping_mul(79));
                        let id_c74 = *(clube73.as_ptr() as *const u32);
                        let mut regs74 = [0u8; 80 * 4];
                        for k in 0..4usize {
                            let z = h74.rotate_left(9 * k as u32);
                            *(regs74.as_mut_ptr().add(80 * k + 4) as *mut u32) = match z % 4 { 0 => id_c74, 1 => 2000 + (z >> 8) as u32 % 82, 2 => 1000 + (z >> 8) as u32 % 82, _ => 4321 };
                            regs74[80 * k + 0x4f] = (z >> 16) as u8;
                            regs74[80 * k + 0x44] = (z >> 24) as u8;
                        }
                        let obj74 = [regs74.as_ptr() as usize, 0];
                        *(0xae1050 as *mut usize) = obj74.as_ptr() as usize;
                        comparar_contrato_com_clube(obj74.as_ptr() as usize, pessoa74.as_ptr() as usize, clube73.as_mut_ptr() as usize);
                        comparar_nivel_do_contrato(pessoa74.as_ptr() as usize, clube73.as_mut_ptr() as usize);
                        // septuagesima nona leva: nota do jogador (motor com o clube73 nos dois lados,
                        // pessoa com o mesmo id, ficha, data de hoje e de nascimento plausiveis)
                        let h79 = s.proximo();
                        let mut pessoa79 = [0u8; 0x50];
                        *(pessoa79.as_mut_ptr() as *mut i32) = id_p74;
                        pessoa79[0x22] = (h79 >> 3) as u8;
                        *(pessoa79.as_mut_ptr().add(0x3e) as *mut u16) = ((h79 >> 11) % 366) as u16;
                        *(pessoa79.as_mut_ptr().add(0x40) as *mut u16) = 1960 + ((h79 >> 20) % 45) as u16;
                        let mut ficha79 = [0u8; 0x30];
                        *(ficha79.as_mut_ptr().add(0xd) as *mut i16) = ((h79 >> 26) % 6000) as i16 - 1000;
                        ficha79[0x2f] = (h79 >> 40) as u8 % 30;
                        *(clube73.as_mut_ptr().add(0x80) as *mut i16) = ((h79 >> 48) % 4000) as i16;
                        // lado 0 (+0x1c) com uma copia de outra forma (+0x80): trocar os lados aparece
                        let mut clube79 = clube73;
                        *(clube79.as_mut_ptr().add(0x80) as *mut i16) = ((h79 >> 30) % 4000) as i16 - 2000;
                        let mut tab79 = [0usize; 9];
                        tab79[7] = clube79.as_ptr() as usize;
                        tab79[8] = clube73.as_ptr() as usize;
                        let mut motor79 = vec![0u8; 0x4800];
                        *(motor79.as_mut_ptr().add(0x4792) as *mut usize) = tab79.as_ptr() as usize;
                        let mut jog79 = [0u8; 0x1a4];
                        jog79[0x27] = ((h79 >> 60) & 1) as u8;
                        *(jog79.as_mut_ptr().add(0x19e) as *mut usize) = motor79.as_ptr() as usize;
                        *(jog79.as_mut_ptr().add(0x69) as *mut usize) = pessoa79.as_ptr() as usize;
                        *(jog79.as_mut_ptr().add(0x6d) as *mut usize) = ficha79.as_ptr() as usize;
                        let hoje79 = (*(0xae2c90 as *const u32), *(0xae2c94 as *const u32));
                        *(0xae2c90 as *mut u32) = (2001u32 << 16) | ((h79 >> 52) % 366) as u32;
                        *(0xae2c94 as *mut u32) = 0;
                        comparar_nota_do_jogador(jog79.as_ptr() as usize);
                        *(0xae2c90 as *mut u32) = hoje79.0;
                        *(0xae2c94 as *mut u32) = hoje79.1;
                        *(0xae2d0c as *mut usize) = antes74.0;
                        *(0xae1050 as *mut usize) = antes74.1;
                        *(0xae235c as *mut i32) = limites74.0;
                        *(0xae2350 as *mut i32) = limites74.1;
                        *(0xae23b4 as *mut u32) = antes_base73;
                        for (k, &a) in todos73.iter().enumerate().rev() {
                            *(a as *mut u32) = antes73[k];
                        }
                    }
                    for (k, &a) in guardados.iter().enumerate() {
                        *(a as *mut u32) = antes_g69[k];
                    }
                }
                // sexagesima quarta leva: lado sob controle (tipos 1 e 9, clube humano ou nao, modos 2..5)
                let mut partida64 = [0u8; 0x48];
                partida64[0x3a] = [0u8, 1, 9, 2][((h63 >> 46) % 4) as usize];
                partida64[0x40] = ((h63 >> 48) & 1) as u8;
                *(partida64.as_mut_ptr().add(0x1c) as *mut usize) = clube_a63.as_ptr() as usize;
                *(partida64.as_mut_ptr().add(0x20) as *mut usize) = clube_b63.as_ptr() as usize;
                let antes64 = (*((m + 0x4792) as *const usize), *((m + 0x1ab) as *const u16), *((m + 0x9766) as *const u32), *((m + 0x9766 + 6371) as *const u32));
                *((m + 0x4792) as *mut usize) = partida64.as_ptr() as usize;
                *((m + 0x1ab) as *mut u8) = 2 + ((h63 >> 49) % 4) as u8;
                *((m + 0x1ac) as *mut u8) = 2 + ((h63 >> 51) % 4) as u8;
                *((m + 0x9766) as *mut u32) = (h63 >> 20) as u32 & 0x4400;
                *((m + 0x9766 + 6371) as *mut u32) = (h63 >> 30) as u32 & 0x4400;
                comparar_lado_sob_controle(m, ((h63 >> 53) & 1) as i8);
                // sexagesima sexta leva: ordem dos penaltis pela tabela tatica (os dois lados com tecnico
                // humano com a marca 0x5f e sem 0x9a01; +0x904b so com selecao, senao o escolher_cobrador
                // dos blocos do autoteste poderia dar erro fatal)
                if TABELA65.len() == 0x91 * 5100 {
                    let regs66 = |p: &[u32; 16]| regs56 + 42735 * (p[0] as i32 - npes56 + 16) as usize;
                    let mut tec66 = [pessoa62, outra62];
                    for t in tec66.iter_mut() {
                        *(t.as_mut_ptr() as *mut u8).add(0x5f) = 1;
                    }
                    for t in tec66.iter() {
                        *((regs66(t) + 0x9a01) as *mut u32) = 0;
                    }
                    partida64[0x41] = ((h63 >> 57) & 1) as u8;
                    let antes66 = (*(0xde1d08 as *const usize), *((m + 0x904d) as *const usize), *((m + 0x911d) as *const usize),
                        *((m + 0x904b) as *const u8), *((m + 0x911b) as *const u8));
                    // a marca vai para +0x8eda + 20 x lado + byte da tabela (com sinal): ate 128 bytes em volta
                    let mut marcas66 = [0u8; 0x140];
                    let mut ordem66 = [0u8; 22];
                    core::ptr::copy_nonoverlapping((m + 0x8eda - 0x80) as *const u8, marcas66.as_mut_ptr(), 0x140);
                    core::ptr::copy_nonoverlapping((m + 0x1af) as *const u8, ordem66.as_mut_ptr(), 22);
                    let mut obj66 = [0u8; 0x608];
                    *(obj66.as_mut_ptr().add(0x601) as *mut usize) = TABELA65.as_ptr() as usize;
                    *(0xde1d08 as *mut usize) = obj66.as_ptr() as usize;
                    let ta = ((h63 >> 58) & 1) as usize;
                    *((m + 0x904d) as *mut usize) = tec66[ta].as_ptr() as usize;
                    *((m + 0x911d) as *mut usize) = tec66[1 - ta].as_ptr() as usize;
                    let sel_a = *(clube_a63.as_ptr() as *const i32) >= 4600;
                    *((m + 0x904b) as *mut u8) = (sel_a && (h63 >> 59) & 1 == 1) as u8;
                    *((m + 0x911b) as *mut u8) = 0;
                    comparar_ordem_dos_penaltis(m);
                    *(0xde1d08 as *mut usize) = antes66.0;
                    *((m + 0x904d) as *mut usize) = antes66.1;
                    *((m + 0x911d) as *mut usize) = antes66.2;
                    *((m + 0x904b) as *mut u8) = antes66.3;
                    *((m + 0x911b) as *mut u8) = antes66.4;
                    core::ptr::copy_nonoverlapping(marcas66.as_ptr(), (m + 0x8eda - 0x80) as *mut u8, 0x140);
                    core::ptr::copy_nonoverlapping(ordem66.as_ptr(), (m + 0x1af) as *mut u8, 22);
                }
                *((m + 0x4792) as *mut usize) = antes64.0;
                *((m + 0x1ab) as *mut u16) = antes64.1;
                *((m + 0x9766) as *mut u32) = antes64.2;
                *((m + 0x9766 + 6371) as *mut u32) = antes64.3;
                for (i, &a) in globais63.iter().enumerate() {
                    *(a as *mut u32) = antes63[i];
                }
                *(0xb74c26 as *mut u32) = antes_vaga0;
                *(tab56 as *mut usize) = id56.as_ptr() as usize;
                comparar_tecnico_9a05(obj56.as_ptr() as usize, quem56);
            }
            *(tab56 as *mut u32) = antes_tab56;
            for (i, &a) in globais56.iter().enumerate() {
                *(a as *mut u32) = antes56[i];
            }
            let codigos56 = [1i16, 0x82, 0x84, 0x88, 0x90, 0xa0, 0xc0, 0x102, 0x104, 0x108, 0x110, 0x120, 0x140, 0x202, 0x204, 0x208, 0x210, 0x220, 0x240,
                0x402, 0x404, 0x408, 0x410, 0x420, 0x440, 0x802, 0x804, 0x808, 0x810, 0x820, 0x840];
            let c56 = codigos56[((h56 >> 50) % 31) as usize];
            // quinquagesima leva: segunda selecao e placar agregado (empates, gols fora, sem ida, bits de +0x38)
            let h57 = s.proximo();
            let globais57 = [0xae235cusize, 0xae2350];
            let antes57: Vec<u32> = globais57.iter().map(|&a| *(a as *const u32)).collect();
            *(0xae235c as *mut i32) = 5000;
            *(0xae2350 as *mut i32) = 200;
            let clube57 = [[4800i32, 4799, 4801, 0, 4900][((h57 >> 2) % 5) as usize]];
            comparar_e_segunda_selecao(clube57.as_ptr() as usize);
            // sexagesima leva: clube da selecao (so aritmetica; base e 0xae23a8 falsos)
            let globais60 = [0xae23b4usize, 0xae23a8];
            let antes60: Vec<u32> = globais60.iter().map(|&a| *(a as *const u32)).collect();
            *(0xae23b4 as *mut u32) = 0x0100_0000 + ((h57 >> 53) as u32 & 0xfff) * 4;
            *(0xae23a8 as *mut u32) = 0x0200_0000;
            let nacao60 = [[0i32, 1, 199, 200, -3, 7000][((h57 >> 30) % 6) as usize]];
            comparar_selecao_principal(nacao60.as_ptr() as usize);
            comparar_selecao_b(nacao60.as_ptr() as usize);
            for (i, &a) in globais60.iter().enumerate() {
                *(a as *mut u32) = antes60[i];
            }
            for (i, &a) in globais57.iter().enumerate() {
                *(a as *mut u32) = antes57[i];
            }
            let pa57 = partida50.as_mut_ptr() as usize;
            *((pa57 + 0x30) as *mut i16) = [3i16, 13, 23, 4, 1003, 993, ((h57 >> 6) % 2000) as i16 - 500][((h57 >> 17) % 7) as usize];
            *((pa57 + 0x38) as *mut u16) = [0u16, 4, 0x4000, 0x4004, (h57 >> 20) as u16][((h57 >> 36) % 5) as usize];
            *((pa57 + 0x4b) as *mut i8) = [-1i8, 0, 1, 2, 3, 5][((h57 >> 39) % 6) as usize];
            *((pa57 + 0x4c) as *mut i8) = [0i8, 1, 2, 3, 4][((h57 >> 42) % 5) as usize];
            *((m + 0x4792) as *mut usize) = pa57;
            *((m + 0xf5bd) as *mut u8) = ((h57 >> 45) % 5) as u8;
            *((m + 0xf5f3) as *mut u8) = ((h57 >> 48) % 5) as u8;
            *((m + 0x8ed4) as *mut i16) = [0x3dei16, 0x3df, 0, 2000, -5][((h57 >> 51) % 5) as usize];
            let mut saidas57 = [0x77u8; 5];
            let sp = saidas57.as_mut_ptr();
            comparar_placar_agregado(m, ((h57 >> 54) % 2) as i8, sp as *mut i8, sp.add(1) as *mut i8, sp.add(2), sp.add(3), sp.add(4), [0u8, 1, 0x80][((h57 >> 56) % 3) as usize]);
            // quinquagesima primeira leva: marcador direto (papeis com setor e lado, goleiros, fora, distancias)
            let h58 = s.proximo();
            let h59 = s.proximo();
            for (ti, t) in tabelas.iter_mut().enumerate() {
                for (i, v) in t.iter_mut().enumerate() {
                    let _ = (h59, ti, i);
                    let r = s.proximo(); // um sorteio por entrada: rotacoes do mesmo numero saem correlacionadas
                    let setor = [1u16, 2, 4, 8, 0x10, 0x20, 0x40, 6, 0x48][(r % 9) as usize];
                    let lado_r = [0x80u16, 0x100, 0x200, 0x400, 0x800, 0x180, 0xa00][((r >> 4) % 7) as usize];
                    *v = setor | lado_r;
                }
            }
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = h58.rotate_left(k as u32 * 5);
                *((p + 0x19) as *mut i8) = if r % 5 == 0 { -1 } else { ((r >> 3) % 11) as i8 };
                *((p + 0x27) as *mut i8) = (k / 20) as i8;
                *((p + 0x19e) as *mut usize) = m;
            }
            for i in 0..10u32 {
                let k = ((h58 >> (i * 6)) % 40) as i8;
                comparar_marcador_direto(m, k / 20, k % 20);
            }
            // quinquagesima nona leva: marcado designado (marcas 4 e 0x10, indices no vetor de pessoas falso,
            // formacao adversaria fechada, 0x106, tabelas A/B do marcado)
            {
                let mut pessoas59 = vec![0u8; 110 * 24];
                let pessoas_antes = *(0xae23bc as *const usize);
                *(0xae23bc as *mut usize) = pessoas59.as_mut_ptr() as usize;
                let r0 = s.proximo();
                let l59 = (r0 % 2) as usize;
                let o59 = 1 - l59;
                let blocos = [jogador_do_bloco(m, 0, 0), jogador_do_bloco(m, 1, 0)];
                let antes59: Vec<Vec<u8>> = blocos.iter().map(|&b| core::slice::from_raw_parts(b as *const u8, 20 * 0x1be).to_vec()).collect();
                let form = m + 0x91e2 + 6371 * l59;
                let antes_form = core::slice::from_raw_parts(form as *const u8, 6371).to_vec();
                let form_o = m + 0x91e2 + 6371 * o59;
                let antes_fo = *((form_o + 0x187b) as *const u32);
                let antes_af = *((m + 0xaf + o59) as *const u8);
                for k in 0..20usize {
                    let z = s.proximo();
                    let q = jogador_do_bloco(m, o59 as i8, k);
                    *((q + 0x19) as *mut i8) = if z % 5 == 0 { -1 } else { (k % 11) as i8 };
                    *((q + 0x69) as *mut usize) = pessoas59.as_ptr() as usize + 110 * ((z >> 4) % 24) as usize;
                    *((q + 0x27) as *mut i8) = o59 as i8;
                    *((q + 0x19e) as *mut usize) = if (z >> 9) % 7 == 0 { 0 } else { m };
                }
                for pos in 0..11usize {
                    let z = s.proximo();
                    *((form + 0x5b4 + pos) as *mut u8) = [4u8, 0x14, 0x10, 0, 0x15, 5][(z % 6) as usize];
                    *((form + 0x5bf + 4 * pos) as *mut i32) = if z % 7 == 3 { -1 } else { ((z >> 4) % 24) as i32 };
                }
                *((form_o + 0x187b) as *mut u32) = if (r0 >> 2) % 3 == 0 { 0x1000_0000 } else { (r0 >> 8) as u32 & 0xefff_ffff };
                *((m + 0xaf + o59) as *mut u8) = [0u8, 1, 2, 5][((r0 >> 5) % 4) as usize];
                // as tabelas A/B sao do caso inteiro: voltam no fim (valores daqui travam secoes seguintes)
                let antes_tabelas: Vec<Vec<u16>> = tabelas.clone();
                for (ti, t) in tabelas.iter_mut().enumerate() {
                    for (i, v) in t.iter_mut().enumerate() {
                        let z = (r0 >> ((ti * 11 + i) % 50)) as u16;
                        *v = [0u16, 0x40, 0x41, 1, z][(z % 5) as usize];
                    }
                }
                for _ in 0..4 {
                    let z = s.proximo();
                    let j = jogador_do_bloco(m, l59 as i8, (z % 20) as usize);
                    *((j + 0x19e) as *mut usize) = m;
                    *((j + 0x27) as *mut i8) = l59 as i8;
                    *((j + 0x19) as *mut i8) = ((z >> 5) % 11) as i8;
                    *((j + 0x106) as *mut u8) = ((z >> 9) % 2) as u8;
                    let alvo77 = comparar_marcado_designado(j);
                    // septuagesima setima leva: quantos e quem marcam o alvo (ou o proprio jogador)
                    let alvo77 = if alvo77 != 0 && (z >> 20) % 4 != 0 { alvo77 } else { j };
                    // os do outro lado com motor e lado validos durante as duas chamadas (0x7160f0 le os dois)
                    let outro77 = (*((alvo77 + 0x27) as *const i8) == 0) as i8;
                    let guardados77: Vec<(usize, u8)> = (0..20).map(|k| {
                        let q = jogador_do_bloco(m, outro77, k);
                        let g = (*((q + 0x19e) as *const usize), *((q + 0x27) as *const u8));
                        *((q + 0x19e) as *mut usize) = m;
                        *((q + 0x27) as *mut i8) = outro77;
                        g
                    }).collect();
                    comparar_quantos_marcam(m, alvo77);
                    comparar_quem_marca(m, alvo77);
                    for (k, &(g, l)) in guardados77.iter().enumerate() {
                        let q = jogador_do_bloco(m, outro77, k);
                        *((q + 0x19e) as *mut usize) = g;
                        *((q + 0x27) as *mut u8) = l;
                    }
                    comparar_designado_da_posicao(form, [-1i8, 0, 5, 10][((z >> 12) % 4) as usize]);
                }
                for (i, &b) in blocos.iter().enumerate() {
                    core::ptr::copy_nonoverlapping(antes59[i].as_ptr(), b as *mut u8, antes59[i].len());
                }
                core::ptr::copy_nonoverlapping(antes_form.as_ptr(), form as *mut u8, antes_form.len());
                *((form_o + 0x187b) as *mut u32) = antes_fo;
                *((m + 0xaf + o59) as *mut u8) = antes_af;
                *(0xae23bc as *mut usize) = pessoas_antes;
                for (t, a) in tabelas.iter_mut().zip(antes_tabelas.iter()) {
                    t.copy_from_slice(a);
                }
            }
            // quinquagesima oitava leva: casas da formacao (todos os bits validos, posicoes 0..=10; o caso de
            // erro abre a caixa do jogo, fora do teste) e craques (atributos nas faixas, papeis)
            {
                let bits: [u64; 34] = [1, 2, 4, 8, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000,
                    0x10000, 0x20000, 0x40000, 0x80000, 0x10_0000, 0x20_0000, 0x40_0000, 0x80_0000, 0x100_0000, 0x200_0000,
                    0x400_0000, 0x800_0000, 0x1000_0000, 0x2000_0000, 0x4000_0000, 0x8000_0000, 1 << 32, 2 << 32];
                let mais: [u64; 2] = [4 << 32, 8 << 32];
                let lado58 = (s.proximo() % 2) as usize;
                let tabela58 = m + 0x91e2 + 6371 * lado58;
                for _ in 0..3 {
                    let z = s.proximo();
                    let b = if z % 18 == 0 { mais[((z >> 8) % 2) as usize] } else { bits[((z >> 8) % 34) as usize] };
                    *((tabela58 + 0x188b + 8 * ((z >> 16) % 11) as usize) as *mut u64) = s.proximo();
                    *((tabela58 + 0x187f) as *mut u32) = 0x5555;
                    comparar_gravar_casa(tabela58, ((z >> 16) % 11) as i8, b);
                }
                let bloco58 = jogador_do_bloco(m, lado58 as i8, 0);
                let antes58 = core::slice::from_raw_parts(bloco58 as *const u8, 20 * 0x1be).to_vec();
                for k in 0..20usize {
                    let q = bloco58 + k * 0x1be;
                    let z = s.proximo();
                    *((q + 0x19) as *mut i8) = if z % 6 == 0 { -1 } else { ((z >> 3) % 11) as i8 };
                    let pd = pessoas[k % 4].as_ptr() as usize;
                    *((q + 0x6d) as *mut usize) = pd;
                    *((q + 0x4d) as *mut f32) = [0.0f32, 5000.0, 7000.5, 8000.0, 8500.0, 9999.9, ((z >> 8) % 10000) as f32][((z >> 24) % 7) as usize];
                }
                for pd in pessoas.iter_mut() {
                    let z = s.proximo();
                    pd[0x43] = (z % 21) as u8;
                    pd[0x33] = ((z >> 8) % 21) as u8;
                    pd[0x27] = ((z >> 16) % 21) as u8;
                }
                comparar_marcar_craques(m, lado58 as i8);
                core::ptr::copy_nonoverlapping(antes58.as_ptr(), bloco58 as *mut u8, antes58.len());
            }
            // quinquagesima terceira leva: marcacao individual (marcas por posicao nos dois lados, fichas)
            for l in 0..2usize {
                for pos in 0..16usize {
                    let r = s.proximo();
                    *((m + 0x91e2 + 6371 * l + 0x5b4 + pos) as *mut u8) = [0x14u8, 0x10, 4, 0x15, 0, (r >> 8) as u8][(r % 6) as usize];
                }
            }
            for k in 0..40usize {
                let p = m + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be;
                let r = s.proximo();
                *((p + 0x69) as *mut usize) = if r % 5 == 0 { 0 } else { fichas.as_ptr().add(k) as usize };
            }
            for (k, f) in fichas.iter_mut().enumerate() {
                *f = 0x1000 + k as u32;
            }
            comparar_marcacao_individual(m);
            // quinquagesima quarta leva: nova reputacao (selecao, tipo 8 com os codigos, partida -2, razao zero)
            {
                let globais = [0xae235cusize, 0xae2350, 0xb74350];
                let antes: Vec<u32> = globais.iter().map(|&a| *(a as *const u32)).collect();
                let mut est = [0f64; 6];
                let mut pa = vec![0u8; 0x60];
                let mut cl = vec![vec![0u8; 0x90]; 2];
                let mut cp = vec![0u8; 0x70];
                *(0xae235c as *mut i32) = 5000;
                *(0xae2350 as *mut i32) = 200;
                *(0xb74350 as *mut usize) = est.as_mut_ptr() as usize;
                let antes_m = *((m + 0x4792) as *const usize);
                for _ in 0..6 {
                    let r: Vec<u64> = (0..12).map(|_| s.proximo()).collect();
                    for (i, e) in est.iter_mut().enumerate() {
                        *e = if r[i] % 5 == 0 { 0.0 } else { (r[i] >> 8) as f64 % 4000.0 / 100.0 };
                    }
                    let pp = pa.as_mut_ptr() as usize;
                    *(pp as *mut i32) = if r[6] % 6 == 0 { -2 } else { 1 };
                    *((pp + 0x14) as *mut usize) = cp.as_mut_ptr() as usize;
                    *((pp + 0x1c) as *mut usize) = cl[0].as_mut_ptr() as usize;
                    *((pp + 0x20) as *mut usize) = cl[1].as_mut_ptr() as usize;
                    *((pp + 0x40) as *mut u8) = (r[6] >> 4) as u8 % 2;
                    *((pp + 0x3a) as *mut u8) = [8u8, 8, 8, 0, 7][((r[6] >> 8) % 5) as usize];
                    *((pp + 0x32) as *mut u16) = [0x41eu16, 0x41f, 0x423, 0x426, 0x427, (r[7] >> 16) as u16][(r[7] % 6) as usize];
                    *((pp + 0x30) as *mut u16) = [0x77u16, 0x78, 0x7c, 0x7d, 0x81, 0x82, 0x87, 0x88, 0x95, 0x96, 0x9b, 0x9c][((r[7] >> 8) % 12) as usize];
                    *((cp.as_mut_ptr() as usize + 0x69) as *mut i16) = [0i16, 5, 10, 19, 20, 35, 30, -3, ((r[8] >> 8) % 21) as i16][(r[8] % 9) as usize];
                    for (i, c) in cl.iter_mut().enumerate() {
                        let z = r[9 + i];
                        *(c.as_mut_ptr() as *mut i32) = [4600i32, 4599, 4800, 0][(z % 4) as usize];
                        *((c.as_mut_ptr() as usize + 0x80) as *mut i16) = [0i16, 499, 500, 999, 1000, 4999, 5000, 9999, 10000, ((z >> 8) % 12000) as i16][((z >> 4) % 10) as usize];
                    }
                    // nivel 21..=28 com os codigos 0x78.. daria peso 0 (divisao por zero no original): fora do teste
                    *((m + 0x4792) as *mut usize) = pp;
                    *((m + 0x1d6) as *mut u32) = 0x55555555;
                    let valor = [0f32, 6.8, 1.0, 3.5, 10.0, 40.0][(r[11] % 6) as usize] + ((r[11] >> 8) % 1000) as f32 / 300.0 - 1.5;
                    comparar_nova_reputacao(m, ((r[11] >> 20) % 2) as i8, valor, ((r[11] >> 21) % 2) as u8, ((r[11] >> 22) % 2) as u8);
                    // quinquagesima quinta leva: reputacao do jogador (mesma partida; ficha, clube do jogador)
                    let mut reg = [0u8; 0x20];
                    let mut jog = [0u8; 0x70];
                    let mut ficha = [0u8; 0x10];
                    let rr = s.proximo();
                    let (rp, jp, fp) = (reg.as_mut_ptr() as usize, jog.as_mut_ptr() as usize, ficha.as_mut_ptr() as usize);
                    *((rp + 0x1b) as *mut usize) = if rr % 23 == 0 { 0 } else { jp };
                    *((jp + 0x69) as *mut usize) = if rr % 19 == 1 { 0 } else { fp };
                    for (i, o) in [8usize, 0xa, 0xc].iter().enumerate() {
                        let z = s.proximo();
                        *((fp + o) as *mut i16) = [0i16, 1, 999, 1000, 3000, 4999, 5000, 9999, 10000, ((z >> 8) % 12000) as i16][((z >> (i * 4)) % 10) as usize];
                    }
                    let alvo = cl[1].as_mut_ptr() as usize;
                    *((jp + 0x1a) as *mut usize) = [alvo, 0x1234, 0][((rr >> 8) % 3) as usize];
                    *((cl[0].as_mut_ptr() as usize + 0x53) as *mut usize) = [alvo, 0x4321, 0][((rr >> 10) % 3) as usize];
                    *((cl[1].as_mut_ptr() as usize + 0x53) as *mut usize) = [alvo, 0x4321, 0][((rr >> 12) % 3) as usize];
                    *((cp.as_mut_ptr() as usize + 0x5d) as *mut usize) = [alvo, 0x5678, 0][((rr >> 14) % 3) as usize];
                    let lado55 = ((rr >> 16) % 2) as usize;
                    comparar_reputacao_do_jogador(rp, cl[lado55].as_mut_ptr() as usize, cl[1 - lado55].as_mut_ptr() as usize, pp, valor, ((rr >> 17) % 2) as u8, ((rr >> 18) % 2) as u8);
                }
                *((m + 0x4792) as *mut usize) = antes_m;
                for (i, &a) in globais.iter().enumerate() {
                    *(a as *mut u32) = antes[i];
                }
            }
            comparar_bit_da_casa(match (h56 >> 56) % 4 { 0 => c56 + 1, 1 => c56 - 1, 2 => (h56 >> 30) as i16, _ => c56 });
            let setor56 = [1u32, 2, 4, 8, 0x10, 0x20, 0x40, 0x41, 0x7f][((h56 >> 33) % 9) as usize];
            let lado56 = [0x80u32, 0x100, 0x200, 0x400, 0x800, 0xf80, 0xc00, 0x0][((h56 >> 37) % 8) as usize];
            let m56 = setor56 | lado56 | (((h56 >> 18) as u32) & 0xfffff000);
            if m56 & 0x7f != 0 && (m56 & 0xf80 != 0 || m56 & 1 != 0) {
                let (mut st56, mut ld56) = (0x5555u16, 0xaaaau16);
                comparar_separar_mascara(m56, &mut st56, &mut ld56);
            }
            for (i, &a) in ids.iter().enumerate() {
                *(a as *mut u32) = ids_antes[i];
            }
            comparar_escolher_c(m, (alto13 & !0xff) | ((h13 >> 53) % 2) as u32, if (h13 >> 54) % 3 == 0 { 0x100 | ((h13 >> 56) as u32 & 0xfe00) } else { (h13 >> 57) as u32 & 0xff });
            comparar_distancia_entre(((q1 >> 51) % 12) as i8 - 1, ((q1 >> 55) % 14) as i8 - 1, ((q1 >> 59) % 11) as i8 - 1, ((q1 >> 33) % 14) as i8 - 1);
        }
    });
    SINTETICO.store(false, std::sync::atomic::Ordering::Relaxed);
}
