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
    Prologo { endereco: 0x6e0d00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6e0df0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x6e0fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x6dbb80, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] },
    Prologo { endereco: 0x6dbb10, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x2f] },
    Prologo { endereco: 0x6d0b60, bytes: &[0x83, 0xec, 0x28, 0x53, 0x55] },
    Prologo { endereco: 0x6d1210, bytes: &[0x83, 0xec, 0x10, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x6be710, bytes: &[0x83, 0xec, 0x18, 0x53, 0x56] },
    Prologo { endereco: 0x6bac40, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] },
    Prologo { endereco: 0x70ec80, bytes: &[0x83, 0xec, 0x28, 0x53, 0x55] },
    Prologo { endereco: 0x6c9cd0, bytes: &[0x83, 0xec, 0x4c, 0x8a, 0x44, 0x24, 0x50] },
    Prologo { endereco: 0x6f5c30, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x70dfd0, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b74a0, bytes: &[0xa1, 0xc8, 0x7d, 0xb6, 0x00] },
    Prologo { endereco: 0x6b6cc0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6b79a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b66a0, bytes: &[0xa1, 0x04, 0x57, 0xa1, 0x00] },
    Prologo { endereco: 0x6b5040, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9] },
    Prologo { endereco: 0x6c54a0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6f6c00, bytes: &[0x83, 0xec, 0x3c, 0x53, 0x55] },
    Prologo { endereco: 0x6d4a20, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x79ce10, bytes: &[0x8b, 0x11, 0x56, 0x85, 0xd2] },
    Prologo { endereco: 0x897990, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x896fe0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b6570, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x547790, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] },
    Prologo { endereco: 0x5477d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] },
    Prologo { endereco: 0x6d1cc0, bytes: &[0x83, 0xec, 0x44, 0x53, 0x55] },
    Prologo { endereco: 0x8983f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8984d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6beeb0, bytes: &[0x83, 0xec, 0x38, 0x53, 0x55] },
    Prologo { endereco: 0x6d0980, bytes: &[0x51, 0x57, 0x8b, 0xf9, 0xc7, 0x44, 0x24, 0x04, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x62be00, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x717470, bytes: &[0x6a, 0xff, 0x68, 0x90, 0xd4, 0x95, 0x00] },
    Prologo { endereco: 0x70c040, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x70a910, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] },
    Prologo { endereco: 0x6b98c0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6fad40, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d5950, bytes: &[0x51, 0x8d, 0x44, 0x24, 0x02] },
    Prologo { endereco: 0x6d5a70, bytes: &[0x53, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x702050, bytes: &[0x81, 0xec, 0x84, 0x06, 0x00, 0x00] },
    Prologo { endereco: 0x6b7e50, bytes: &[0x6a, 0xff, 0x68, 0x0c, 0xd3, 0x95, 0x00] },
    Prologo { endereco: 0x6b7f20, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6b97a0, bytes: &[0x6a, 0xff, 0x68, 0xda, 0xd3, 0x95, 0x00] },
    Prologo { endereco: 0x6db500, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b4190, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] },
    Prologo { endereco: 0x6b42a0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9] },
    Prologo { endereco: 0x6dbc50, bytes: &[0x6a, 0xff, 0x68, 0x52, 0xd4, 0x95, 0x00] },
    Prologo { endereco: 0x6b5a70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6b5ce0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6b5f00, bytes: &[0x81, 0xec, 0x58, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b4360, bytes: &[0x6a, 0xff, 0x68, 0xdc, 0xd1, 0x95, 0x00] },
    Prologo { endereco: 0x6b3c10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x714d30, bytes: &[0x8a, 0x41, 0x27, 0xc3, 0x90] },
    Prologo { endereco: 0x6d8c70, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    Prologo { endereco: 0x6d98b0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6d9c40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b55c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] },
    Prologo { endereco: 0x6d9e50, bytes: &[0x81, 0xec, 0x0c, 0x06, 0x00, 0x00] },
    Prologo { endereco: 0x6d8d10, bytes: &[0xb8, 0xd8, 0x1b, 0x00, 0x00] },
    Prologo { endereco: 0x6b5260, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6d7b00, bytes: &[0xb8, 0x18, 0x26, 0x00, 0x00] },
    Prologo { endereco: 0x9326e0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x933080, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x933170, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x5b7d20, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b7e60, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b8960, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
    Prologo { endereco: 0x7c2150, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c1950, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b8dc0, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] },
    Prologo { endereco: 0x844300, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x83bd00, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x866f60, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x896a90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8968f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8967d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b3c70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b50d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x452fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x68a030, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x0d, 0xe8, 0xf6, 0x9c, 0x00] },
    Prologo { endereco: 0x68a1c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5af180, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x897ee0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x76d430, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fdd30, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7f6cd0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x600c90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4ca470, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x6d58f0, bytes: &[0x8a, 0x81, 0xa8, 0x8e, 0x00, 0x00] },
    Prologo { endereco: 0x6d5930, bytes: &[0x8a, 0x81, 0x38, 0xf6, 0x00, 0x00] },
    Prologo { endereco: 0x6d5af0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] },
    Prologo { endereco: 0x6d5b20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] },
    Prologo { endereco: 0x6e9460, bytes: &[0x8b, 0xc1, 0xc7, 0x00, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6ebd60, bytes: &[0xc7, 0x81, 0x9e, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6d5d00, bytes: &[0x66, 0x8b, 0x41, 0x08, 0x66, 0x85, 0xc0] },
    Prologo { endereco: 0x6d5bf0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x83, 0xc8, 0xff] },
    Prologo { endereco: 0x6d5b50, bytes: &[0x53, 0x56, 0x57, 0xbe, 0x90, 0x87, 0xb6, 0x00] },
    Prologo { endereco: 0x6b6510, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x6bd840, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x6d5d80, bytes: &[0x51, 0x53, 0x56, 0x6a, 0x03] },
    Prologo { endereco: 0x6d60b0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x6d4b70, bytes: &[0x6a, 0xff, 0x68, 0x00, 0xd4, 0x95, 0x00] },
    Prologo { endereco: 0x6eada0, bytes: &[0x8b, 0xc1, 0x83, 0xca, 0xff] },
    Prologo { endereco: 0x6d78b0, bytes: &[0x51, 0x8b, 0xd1, 0x55, 0x56] },
    Prologo { endereco: 0x6db2d0, bytes: &[0x8a, 0x44, 0x24, 0x18, 0x81, 0xec, 0x58, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6e86d0, bytes: &[0x51, 0x8b, 0x15, 0x20, 0x71, 0xdd, 0x00] },
    Prologo { endereco: 0x6e8910, bytes: &[0x81, 0xec, 0x70, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x6e6340, bytes: &[0x83, 0xec, 0x50, 0x53, 0x55] },
    Prologo { endereco: 0x6e2f20, bytes: &[0x83, 0xec, 0x50, 0x53, 0x8b, 0x5c, 0x24, 0x58] },
    Prologo { endereco: 0x69fc60, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4cf6d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x62c2f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x896570, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c6930, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] },
    Prologo { endereco: 0x785790, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x62bd10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x8c1870, bytes: &[0x8b, 0x81, 0x2e, 0x09, 0x00, 0x00] },
    Prologo { endereco: 0x55cae0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x7a7eb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x0c, 0x2d, 0xae, 0x00] },
    Prologo { endereco: 0x684790, bytes: &[0x8a, 0x81, 0xbe, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6826d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8d, 0x14, 0xc5, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5d5a60, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] },
    Prologo { endereco: 0x4d0c20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x76d810, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x84, 0xc0] },
    Prologo { endereco: 0x4cc9e0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x4cca30, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x34] },
    Prologo { endereco: 0x8d0d90, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8a, 0x41, 0x2c] },
    Prologo { endereco: 0x725a70, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x71b120, bytes: &[0x56, 0x6a, 0x2d, 0xb9, 0x50, 0x44, 0xb7, 0x00] },
    Prologo { endereco: 0x727f00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x735780, bytes: &[0x56, 0x8b, 0x71, 0x08, 0x33, 0xd2] },
    Prologo { endereco: 0x735840, bytes: &[0x56, 0x8b, 0x71, 0x08, 0x33, 0xd2] },
    Prologo { endereco: 0x5221f0, bytes: &[0x8a, 0x51, 0x51, 0x53, 0x8a, 0x59, 0x3c] },
    Prologo { endereco: 0x71b1b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x71b300, bytes: &[0x83, 0xec, 0x08, 0x6a, 0x0e] },
    Prologo { endereco: 0x71b3d0, bytes: &[0xa1, 0x84, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x71b4a0, bytes: &[0x6a, 0x00, 0x68, 0x70, 0xc1, 0x71, 0x00] },
    Prologo { endereco: 0x71bdc0, bytes: &[0x51, 0xa1, 0x84, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x71c170, bytes: &[0x56, 0x6a, 0x2d, 0xb9, 0x50, 0x44, 0xb7, 0x00] },
    Prologo { endereco: 0x556210, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] },
    Prologo { endereco: 0x556240, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] },
    Prologo { endereco: 0x55cb20, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x8c5410, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] },
    Prologo { endereco: 0x8c5430, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] },
    Prologo { endereco: 0x8c61e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x04] },
    Prologo { endereco: 0x718aa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x725860, bytes: &[0x51, 0x53, 0x56, 0x57, 0x6a, 0x11] },
    Prologo { endereco: 0x722dc0, bytes: &[0x53, 0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00] },
    Prologo { endereco: 0x723d20, bytes: &[0x83, 0xec, 0x24, 0x53, 0x55] },
    Prologo { endereco: 0x7febb0, bytes: &[0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00] },
    Prologo { endereco: 0x839590, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x5d5ab0, bytes: &[0x8b, 0x81, 0x56, 0x08, 0x00, 0x00] },
    Prologo { endereco: 0x5d5ac0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x89, 0x1d, 0x65, 0x00, 0x00] },
    Prologo { endereco: 0x62c660, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x00] },
    Prologo { endereco: 0x5c2990, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x5cb9b0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x8a, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x86fc20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x57] },
    Prologo { endereco: 0x4d0bd0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x84aeb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x600e90, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x8c5450, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x8ce920, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x55cb60, bytes: &[0x53, 0x55, 0x8b, 0x51, 0x14] },
    Prologo { endereco: 0x7a7ed0, bytes: &[0x8b, 0x41, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x5ca030, bytes: &[0xa1, 0xfc, 0xad, 0xad, 0x00] },
    Prologo { endereco: 0x785830, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cb4f0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x8cb590, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x8cb630, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x7a82b0, bytes: &[0x8b, 0x51, 0x20, 0x53, 0x33, 0xc0] },
    Prologo { endereco: 0x7a8470, bytes: &[0x8b, 0x51, 0x40, 0x53, 0x33, 0xc0] },
    Prologo { endereco: 0x682f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c1a60, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7ae640, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7ae6e0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6834f0, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x6869a0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0] },
    Prologo { endereco: 0x8c1f80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fdc30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fed50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7ae570, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x717cc0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x718930, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x727e20, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c47b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x729500, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x729bd0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x71dd70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x71df00, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x728e90, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x725ca0, bytes: &[0x83, 0xec, 0x44, 0x53, 0x55] },
    Prologo { endereco: 0x725f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x727c30, bytes: &[0x53, 0x55, 0x56, 0x57, 0x6a, 0x00] },
    Prologo { endereco: 0x7217a0, bytes: &[0x83, 0xec, 0x34, 0x53, 0x56] },
    Prologo { endereco: 0x722ed0, bytes: &[0x81, 0xec, 0x0c, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x71e040, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] },
    Prologo { endereco: 0x71e750, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0xd5, 0x95, 0x00] },
    Prologo { endereco: 0x720320, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0xd5, 0x95, 0x00] },
    Prologo { endereco: 0x723350, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xd5, 0x95, 0x00] },
    Prologo { endereco: 0x71f500, bytes: &[0xa0, 0xb4, 0xb7, 0xb6, 0x00] },
    Prologo { endereco: 0x721000, bytes: &[0x6a, 0xff, 0x68, 0x9e, 0xd5, 0x95, 0x00] },
    Prologo { endereco: 0x717db0, bytes: &[0x81, 0xec, 0x9c, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x721cd0, bytes: &[0x81, 0xec, 0x18, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x724520, bytes: &[0x81, 0xec, 0x3c, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x727fb0, bytes: &[0x83, 0xec, 0x44, 0x53, 0x56] },
    Prologo { endereco: 0x729600, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7293f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x729820, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7260f0, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x72aa80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x72adf0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x403350, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x402ac0, bytes: &[0x66, 0x83, 0x7c, 0x24, 0x38, 0xff] },
    Prologo { endereco: 0x402e70, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x718e90, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x71a240, bytes: &[0xa1, 0x84, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x71b630, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x71d9e0, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0xd5, 0x95, 0x00] },
    Prologo { endereco: 0x71c270, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0xd5, 0x95, 0x00] },
    Prologo { endereco: 0x7280e0, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7357c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53, 0x55] },
    Prologo { endereco: 0x62d140, bytes: &[0x51, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x18] },
    Prologo { endereco: 0x785670, bytes: &[0x8b, 0x01, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x76c060, bytes: &[0x51, 0x53, 0x55, 0x32, 0xc0] },
    Prologo { endereco: 0x8972c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x72b810, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] },
    Prologo { endereco: 0x521d60, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] },
    Prologo { endereco: 0x519c90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c4680, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x8c6db0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x8c16b0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c5fc0, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x10] },
    Prologo { endereco: 0x775ba0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56, 0x57] },
    Prologo { endereco: 0x5af2f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x84b230, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x0c] },
    Prologo { endereco: 0x8c5cf0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x83bf40, bytes: &[0x56, 0x8b, 0x91, 0xdc, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4afeb0, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] },
    Prologo { endereco: 0x729ff0, bytes: &[0xa0, 0xc4, 0xb7, 0xb6, 0x00] },
    Prologo { endereco: 0x72b160, bytes: &[0x8b, 0x54, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x7c6bf0, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x8ced40, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x8c5e40, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] },
    Prologo { endereco: 0x786fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x786d30, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7875e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x521e60, bytes: &[0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x521eb0, bytes: &[0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x5c64e0, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x683210, bytes: &[0x51, 0x53, 0x8b, 0x1d, 0xfc, 0x79, 0xb6, 0x00] },
    Prologo { endereco: 0x6844b0, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x83, 0xec, 0x10] },
    Prologo { endereco: 0x68a4d0, bytes: &[0x0f, 0xbf, 0x81, 0xa9, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4b0010, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] },
    Prologo { endereco: 0x4b0890, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x521f10, bytes: &[0x66, 0x8b, 0x4c, 0x24, 0x0c] },
    Prologo { endereco: 0x5b7160, bytes: &[0x83, 0xec, 0x48, 0x53, 0x55] },
    Prologo { endereco: 0x7ad220, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] },
    Prologo { endereco: 0x5fe290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6b6970, bytes: &[0x6a, 0xff, 0x68, 0x8c, 0xd2, 0x95, 0x00] },
    Prologo { endereco: 0x729e30, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0xd6, 0x95, 0x00] },
    Prologo { endereco: 0x6847b0, bytes: &[0x8a, 0x91, 0xc1, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x68a160, bytes: &[0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x68a3e0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x10] },
    Prologo { endereco: 0x448490, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08] },
    Prologo { endereco: 0x448110, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00] },
    Prologo { endereco: 0x448170, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x54, 0x24, 0x00] },
    Prologo { endereco: 0x5fe420, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x600740, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a8780, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x729000, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7293c0, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x06] },
    Prologo { endereco: 0x933a00, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] },
    Prologo { endereco: 0x9331c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x933860, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x933610, bytes: &[0x81, 0xec, 0x20, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x933320, bytes: &[0x81, 0xec, 0x14, 0x03, 0x00, 0x00] },
    Prologo { endereco: 0x90cd50, bytes: &[0xb8, 0x1c, 0x68, 0x00, 0x00] },
    Prologo { endereco: 0x932c30, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x1c] },
    Prologo { endereco: 0x932e10, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x1c] },
    Prologo { endereco: 0x932b00, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x838e10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x839290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x838eb0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8395b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x839760, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x591f80, bytes: &[0x83, 0xec, 0x70, 0x8b, 0x44, 0x24, 0x74] },
    Prologo { endereco: 0x524d30, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x524d10, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x82ff40, bytes: &[0x53, 0xb9, 0x50, 0x44, 0xb7, 0x00] },
    Prologo { endereco: 0x827110, bytes: &[0x81, 0xec, 0x00, 0x08, 0x00, 0x00] },
    Prologo { endereco: 0x82e430, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x82a9c0, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5408c0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x5407e0, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x53d980, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x10] },
    Prologo { endereco: 0x53ab90, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x544670, bytes: &[0x80, 0x3d, 0x3c, 0x23, 0xae, 0x00, 0x01] },
    Prologo { endereco: 0x53d740, bytes: &[0x8b, 0x44, 0x24, 0x04, 0xb1, 0x02] },
    Prologo { endereco: 0x544520, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x543540, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5431e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x540780, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x53a610, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0xba, 0x3f, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5446d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xe4, 0xf2, 0x9c, 0x00] },
    Prologo { endereco: 0x53d7f0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] },
    Prologo { endereco: 0x5444a0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x544590, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x544600, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x544380, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x542740, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x539c60, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x53a1b0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    Prologo { endereco: 0x53dba0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53b640, bytes: &[0x66, 0xa1, 0xf0, 0xbd, 0xae, 0x00] },
    Prologo { endereco: 0x53ca90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x545880, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x79, 0x95, 0x00] },
    Prologo { endereco: 0x544710, bytes: &[0x8a, 0x4c, 0x24, 0x08, 0x84, 0xc9] },
    Prologo { endereco: 0x53a8b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x0c] },
    Prologo { endereco: 0x540980, bytes: &[0x8a, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x53c590, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x541f90, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x544820, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x53a980, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x53aa10, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x539490, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x540520, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xe4, 0xf1, 0x9c, 0x00] },
    Prologo { endereco: 0x540490, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x543fd0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x545a60, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x0c, 0xff] },
    Prologo { endereco: 0x543f30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5442f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x539b90, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x53b7c0, bytes: &[0x66, 0xa1, 0x84, 0x31, 0xae, 0x00] },
    Prologo { endereco: 0x53b9a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53bd10, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6146b0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x60ba80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xa4, 0x00, 0x9d, 0x00] },
    Prologo { endereco: 0x60d810, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x60efd0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x6115e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x54d5e0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54d740, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54d8a0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54da00, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54e780, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54fd60, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x550230, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x550380, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5504d0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5505c0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x550710, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x550860, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54db50, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54feb0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54e880, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54f030, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54e630, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54fc10, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54d6e0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54d840, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54d9a0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54daf0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54e5d0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54e720, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54efd0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54fbb0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54fd00, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54fe50, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x5501d0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x550320, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x550470, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x5506b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x550800, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x550950, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x54d580, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x54d5b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x527ee0, bytes: &[0xa1, 0xcc, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x527f20, bytes: &[0xa1, 0xe4, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x527f60, bytes: &[0xa1, 0x7c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x540020, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x5402f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x549c30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x538ec0, bytes: &[0x81, 0xec, 0x78, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53a0e0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x538bc0, bytes: &[0x81, 0xec, 0x74, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x54dc90, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x54e9a0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x53] },
    Prologo { endereco: 0x54f150, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x53] },
    Prologo { endereco: 0x54ffd0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x53] },
    Prologo { endereco: 0x5453f0, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x79, 0x95, 0x00] },
    Prologo { endereco: 0x545770, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x79, 0x95, 0x00] },
    Prologo { endereco: 0x541e60, bytes: &[0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x544970, bytes: &[0x83, 0xec, 0x10, 0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x539750, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x53c190, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53cd60, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x547770, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x549e50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x549ea0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x549c90, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x85, 0xd2] },
    Prologo { endereco: 0x549cf0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x549d60, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] },
    Prologo { endereco: 0x53cb80, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53cc30, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53c8d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5454c0, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0x79, 0x95, 0x00] },
    Prologo { endereco: 0x5455a0, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x79, 0x95, 0x00] },
    Prologo { endereco: 0x5456a0, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x79, 0x95, 0x00] },
    Prologo { endereco: 0x545990, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x7a, 0x95, 0x00] },
    Prologo { endereco: 0x535930, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x544f30, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x20] },
    Prologo { endereco: 0x527240, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5267c0, bytes: &[0x81, 0xec, 0x28, 0x06, 0x00, 0x00] },
    Prologo { endereco: 0x539ae0, bytes: &[0x8b, 0x0d, 0x7c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x5315a0, bytes: &[0x81, 0xec, 0x04, 0x06, 0x00, 0x00] },
    Prologo { endereco: 0x53db40, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] },
    Prologo { endereco: 0x549100, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x549200, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x5492c0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x5493c0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x549470, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x55] },
    Prologo { endereco: 0x549850, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x5494d0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x5495b0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x549690, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x549770, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x549940, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x549a30, bytes: &[0x81, 0xec, 0xcc, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x545e90, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x547c90, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x32, 0xc0] },
    Prologo { endereco: 0x547ce0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x32, 0xc0] },
    Prologo { endereco: 0x547f50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] },
    Prologo { endereco: 0x547bd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] },
    Prologo { endereco: 0x547c10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] },
    Prologo { endereco: 0x547c50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] },
    Prologo { endereco: 0x548090, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x547870, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x547940, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x547a10, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x547e70, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x547f90, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x5480f0, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x5481a0, bytes: &[0x51, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x548150, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x548da0, bytes: &[0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18] },
    Prologo { endereco: 0x5487b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x5487d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x53d900, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x1c, 0xf4, 0x9c, 0x00] },
    Prologo { endereco: 0x53d940, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xd0, 0xf4, 0x9c, 0x00] },
    Prologo { endereco: 0x53ac40, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x540340, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x540080, bytes: &[0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00] },
    Prologo { endereco: 0x540270, bytes: &[0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00] },
    Prologo { endereco: 0x53e990, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53fd40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53fe50, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53e140, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53eab0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5482f0, bytes: &[0x53, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x5461b0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5465a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x548810, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10] },
    Prologo { endereco: 0x546ab0, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x5470d0, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x53c9b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5448c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x544c70, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x543280, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x543440, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x543b10, bytes: &[0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53ff60, bytes: &[0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00] },
    Prologo { endereco: 0x53ffe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x53d7d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x53] },
    Prologo { endereco: 0x545dc0, bytes: &[0xb0, 0x01, 0xb9, 0x04, 0x21, 0xae, 0x00] },
    Prologo { endereco: 0x545e00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x549dd0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x33, 0xc0] },
    Prologo { endereco: 0x53e2e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53ad00, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x53cee0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x542100, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5428a0, bytes: &[0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x53ee50, bytes: &[0xb8, 0xa4, 0x11, 0x00, 0x00] },
    Prologo { endereco: 0x616f20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6177f0, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x0c] },
    Prologo { endereco: 0x601a40, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x603834, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x535a80, bytes: &[0x81, 0xec, 0x40, 0x0e, 0x00, 0x00] },
    Prologo { endereco: 0x6181d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x55] },
    Prologo { endereco: 0x6827d0, bytes: &[0x83, 0xec, 0x4c, 0x53, 0x55] },
    Prologo { endereco: 0x682b30, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x687970, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] },
    Prologo { endereco: 0x689ad0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x683010, bytes: &[0xa1, 0x38, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x684640, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x4afce0, bytes: &[0xa1, 0x7c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x4b02e0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x4c0720, bytes: &[0x81, 0xec, 0xf0, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x4afd80, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c88f0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x5ae6a0, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x8b, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x5ac920, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x5ae870, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x5cbdd0, bytes: &[0x66, 0x0f, 0xbe, 0x01, 0x8b, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x774f30, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x774f90, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x7774b0, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x776250, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8a, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x7762f0, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x76d880, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x775000, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x775070, bytes: &[0x51, 0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x775f90, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x28] },
    Prologo { endereco: 0x76abf0, bytes: &[0x83, 0xec, 0x28, 0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x778ed0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] },
    Prologo { endereco: 0x778ef0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x5c4cb0, bytes: &[0x83, 0xec, 0x1c, 0x89, 0x4c, 0x24, 0x00] },
    Prologo { endereco: 0x5c5040, bytes: &[0x83, 0xec, 0x20, 0x8b, 0x44, 0x24, 0x24] },
    Prologo { endereco: 0x685f60, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0xd1] },
    Prologo { endereco: 0x68aad0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] },
    Prologo { endereco: 0x683410, bytes: &[0xa1, 0xfc, 0x79, 0xb6, 0x00] },
    Prologo { endereco: 0x685cf0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x68ab70, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x68abd0, bytes: &[0x53, 0x56, 0x8b, 0xb1, 0xb1, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x448230, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x4484d0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x448500, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x448530, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x4485b0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x450c60, bytes: &[0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x11] },
    Prologo { endereco: 0x450d30, bytes: &[0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x12] },
    Prologo { endereco: 0x448550, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x451570, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x786010, bytes: &[0x83, 0xec, 0x08, 0xba, 0x32, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8cb940, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08] },
    Prologo { endereco: 0x8cbf40, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00] },
    Prologo { endereco: 0x7a8b70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08] },
    Prologo { endereco: 0x842080, bytes: &[0x8b, 0x01, 0x83, 0xec, 0x08] },
    Prologo { endereco: 0x7a6e60, bytes: &[0x51, 0xa1, 0x64, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x556c70, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] },
    Prologo { endereco: 0x8626e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x62c510, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x85fa00, bytes: &[0xa1, 0x94, 0x2c, 0xae, 0x00] },
    Prologo { endereco: 0x867520, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x06] },
    Prologo { endereco: 0x867240, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x06] },
    Prologo { endereco: 0x4cbb60, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] },
    Prologo { endereco: 0x4cc0b0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x4cc0f0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x4cc810, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x4cb0a0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x4ca1c0, bytes: &[0x51, 0x8b, 0xc1, 0x53, 0x33, 0xdb] },
    Prologo { endereco: 0x7549b0, bytes: &[0x51, 0x8b, 0xc1, 0x33, 0xd2] },
    Prologo { endereco: 0x7f8ee0, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9] },
    Prologo { endereco: 0x5fff30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x24] },
    Prologo { endereco: 0x7a8690, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x7a8050, bytes: &[0xa1, 0x54, 0x1f, 0xde, 0x00] },
    Prologo { endereco: 0x8c7470, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x8c7330, bytes: &[0x51, 0x55, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x7ae950, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x80, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x690c30, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x450d40, bytes: &[0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x10] },
    Prologo { endereco: 0x72b940, bytes: &[0x83, 0xec, 0x50, 0x53, 0x56] },
    Prologo { endereco: 0x4485d0, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x54, 0x95, 0x00] },
    Prologo { endereco: 0x448620, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0x54, 0x95, 0x00] },
    Prologo { endereco: 0x448670, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x54, 0x95, 0x00] },
    Prologo { endereco: 0x450ad0, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x56, 0x95, 0x00] },
    Prologo { endereco: 0x450b20, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x56, 0x95, 0x00] },
    Prologo { endereco: 0x450b70, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x56, 0x95, 0x00] },
    Prologo { endereco: 0x450bc0, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x56, 0x95, 0x00] },
    Prologo { endereco: 0x450c10, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x57, 0x95, 0x00] },
    Prologo { endereco: 0x450c70, bytes: &[0x6a, 0xff, 0x68, 0x43, 0x57, 0x95, 0x00] },
    Prologo { endereco: 0x8e05a0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x8d0530, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x0c] },
    Prologo { endereco: 0x5d5a20, bytes: &[0xdd, 0x44, 0x24, 0x04, 0xdd, 0x44, 0x24, 0x0c] },
    Prologo { endereco: 0x4cf560, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x38] },
    Prologo { endereco: 0x5563f0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cb6d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x786520, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7aefd0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x521e00, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8a, 0x86, 0xb1, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x450cd0, bytes: &[0x6a, 0xff, 0x68, 0x63, 0x57, 0x95, 0x00] },
    Prologo { endereco: 0x8c7260, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x10] },
    Prologo { endereco: 0x84b180, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x5567d0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x4ccbf0, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x34] },
    Prologo { endereco: 0x4cc930, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x34] },
    Prologo { endereco: 0x786780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7862d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7863f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4486c0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x62c590, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x8cf7e0, bytes: &[0x83, 0xec, 0x2c, 0x33, 0xc0] },
    Prologo { endereco: 0x5fe8d0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x33, 0xdb] },
    Prologo { endereco: 0x7f9690, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x10] },
    Prologo { endereco: 0x7ae8a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x83, 0xcb, 0xff] },
    Prologo { endereco: 0x6951e0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x55ca00, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x8420e0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10] },
    Prologo { endereco: 0x8cc930, bytes: &[0x51, 0x53, 0x8b, 0xd9, 0x8b, 0x4c, 0x24, 0x0c] },
    Prologo { endereco: 0x452710, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4528d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x452cb0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a81a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a8510, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a87d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4cd030, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x4cca70, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] },
    Prologo { endereco: 0x55d270, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x8d2fa0, bytes: &[0x8b, 0x15, 0x84, 0xf6, 0x9c, 0x00] },
    Prologo { endereco: 0x8cf5e0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x5c6740, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0xa0, 0x95, 0x00] },
    Prologo { endereco: 0x786640, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x786860, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x784eb0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x785a20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x74ae60, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7f9190, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c6830, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] },
    Prologo { endereco: 0x86fb80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b2ba0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cdc90, bytes: &[0x6a, 0xff, 0x68, 0x18, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8cdd80, bytes: &[0x6a, 0xff, 0x68, 0x38, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8cd150, bytes: &[0x6a, 0xff, 0x68, 0xd8, 0x3e, 0x96, 0x00] },
    Prologo { endereco: 0x8cde70, bytes: &[0x6a, 0xff, 0x68, 0x58, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8d0a70, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x40, 0x96, 0x00] },
    Prologo { endereco: 0x8cdb80, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x8cb810, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c7070, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cac40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c79a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c7950, bytes: &[0x56, 0x8b, 0xf1, 0x6a, 0x01] },
    Prologo { endereco: 0x4cd6d0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x866ee0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb] },
    Prologo { endereco: 0x85e590, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x867d30, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8a, 0x47, 0x35] },
    Prologo { endereco: 0x757a50, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x757c70, bytes: &[0x83, 0xec, 0x24, 0x53, 0x8b, 0xd9] },
    Prologo { endereco: 0x757da0, bytes: &[0x83, 0xec, 0x20, 0x55, 0x56] },
    Prologo { endereco: 0x754740, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x757ea0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x757ee0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x51c020, bytes: &[0x81, 0xec, 0xe8, 0x05, 0x00, 0x00] },
    Prologo { endereco: 0x51f2f0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x51ebc0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x687430, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x687a20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x68aa30, bytes: &[0x53, 0x33, 0xdb, 0x66, 0x39, 0x59, 0x3e] },
    Prologo { endereco: 0x6830b0, bytes: &[0x8b, 0x0d, 0xf0, 0x7e, 0xdd, 0x00] },
    Prologo { endereco: 0x4afb40, bytes: &[0x56, 0x57, 0x0f, 0xbf, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x4afbd0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x556e90, bytes: &[0x83, 0xec, 0x10, 0x0f, 0xbe, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x556d00, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x55e1d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x62e500, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] },
    Prologo { endereco: 0x631f90, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x62bfa0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x448ad0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] },
    Prologo { endereco: 0x448cb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x452a90, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c29b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5c45a0, bytes: &[0x6a, 0xff, 0x68, 0x00, 0xa0, 0x95, 0x00] },
    Prologo { endereco: 0x5c4b60, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x786090, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x18] },
    Prologo { endereco: 0x7861e0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x785d40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7874b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x787350, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7858c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x736720, bytes: &[0x83, 0xec, 0x10, 0x55, 0x8b, 0x6c, 0x24, 0x1c] },
    Prologo { endereco: 0x74af60, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] },
    Prologo { endereco: 0x786960, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7aace0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x7ad0e0, bytes: &[0x83, 0xec, 0x14, 0x89, 0x4c, 0x24, 0x10] },
    Prologo { endereco: 0x7ad590, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x69d060, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x69c540, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x8c6220, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x849f00, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x849d60, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x844660, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x7b7490, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x5c87b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5cc2d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x696570, bytes: &[0x83, 0xec, 0x08, 0x89, 0x4c, 0x24, 0x00] },
    Prologo { endereco: 0x6978f0, bytes: &[0x51, 0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x6ac470, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] },
    Prologo { endereco: 0x4d0b50, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x4cf4a0, bytes: &[0x55, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x4d45f0, bytes: &[0x6a, 0xff, 0x68, 0x40, 0x6e, 0x95, 0x00] },
    Prologo { endereco: 0x86dda0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x86d6e0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x86f950, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x69b650, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x690730, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x690890, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x77bbe0, bytes: &[0x6a, 0xff, 0x68, 0xfe, 0xe0, 0x95, 0x00] },
    Prologo { endereco: 0x738340, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7386f0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x735870, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] },
    Prologo { endereco: 0x787140, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x786b70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x732c80, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x743370, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x743a50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x839e60, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x83a780, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    Prologo { endereco: 0x842180, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x8423f0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84af00, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x8d0590, bytes: &[0x83, 0xec, 0x10, 0x32, 0xd2] },
    Prologo { endereco: 0x8cc5b0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8caee0, bytes: &[0x83, 0xec, 0x0c, 0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x8d0f60, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x556940, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x556270, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x556570, bytes: &[0x83, 0xec, 0x20, 0x53, 0x55] },
    Prologo { endereco: 0x7c6a30, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c6ca0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] },
    Prologo { endereco: 0x7c6650, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x775220, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7775c0, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x556830, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x4dc910, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x4cd9c0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x4ca300, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4d01e0, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x6e, 0x95, 0x00] },
    Prologo { endereco: 0x4d07e0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4cbd20, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x10] },
    Prologo { endereco: 0x694c80, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x69b350, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x86f5b0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x69f810, bytes: &[0x55, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x4cb990, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x4cb250, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x83c090, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x83a730, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x842b00, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x24, 0x96, 0x00] },
    Prologo { endereco: 0x842c00, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x842320, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x8cb340, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] },
    Prologo { endereco: 0x8c36b0, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x8c6ae0, bytes: &[0x6a, 0xff, 0x68, 0x18, 0x3d, 0x96, 0x00] },
    Prologo { endereco: 0x8c8740, bytes: &[0x6a, 0xff, 0x68, 0xce, 0x3d, 0x96, 0x00] },
    Prologo { endereco: 0x62e820, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x62e9e0, bytes: &[0x6a, 0xff, 0x68, 0x88, 0xb1, 0x95, 0x00] },
    Prologo { endereco: 0x62ce30, bytes: &[0x6a, 0xff, 0x68, 0x20, 0xb1, 0x95, 0x00] },
    Prologo { endereco: 0x62a920, bytes: &[0x6a, 0xff, 0x68, 0x90, 0xb0, 0x95, 0x00] },
    Prologo { endereco: 0x62cf70, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7acfc0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xf2, 0x95, 0x00] },
    Prologo { endereco: 0x452da0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x7c1350, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x448760, bytes: &[0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18] },
    Prologo { endereco: 0x5cc3e0, bytes: &[0x51, 0x8a, 0x81, 0x06, 0x35, 0x00, 0x00] },
    Prologo { endereco: 0x5cc5f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] },
    Prologo { endereco: 0x5cc610, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x776fc0, bytes: &[0x6a, 0xff, 0x68, 0x48, 0xdf, 0x95, 0x00] },
    Prologo { endereco: 0x778e90, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x7779b0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x83, 0xec, 0x08] },
    Prologo { endereco: 0x522190, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5221c0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x68a980, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x33, 0xf6] },
    Prologo { endereco: 0x69aba0, bytes: &[0x83, 0xec, 0x08, 0xa1, 0xcc, 0x6e, 0xdd, 0x00] },
    Prologo { endereco: 0x6ac560, bytes: &[0x6a, 0xff, 0x68, 0x88, 0xd0, 0x95, 0x00] },
    Prologo { endereco: 0x69b790, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x6a5610, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x69eb50, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x690b70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x09] },
    Prologo { endereco: 0x869480, bytes: &[0x6a, 0xff, 0x68, 0x96, 0x2b, 0x96, 0x00] },
    Prologo { endereco: 0x86a1c0, bytes: &[0x6a, 0xff, 0x68, 0x06, 0x2c, 0x96, 0x00] },
    Prologo { endereco: 0x86bdd0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x86cd70, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x2d, 0x96, 0x00] },
    Prologo { endereco: 0x86c000, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x86c1d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8cee70, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cf0e0, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8d0bb0, bytes: &[0x6a, 0xff, 0x68, 0x90, 0x40, 0x96, 0x00] },
    Prologo { endereco: 0x8d0780, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x40, 0x96, 0x00] },
    Prologo { endereco: 0x8ca8a0, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] },
    Prologo { endereco: 0x8cd240, bytes: &[0x6a, 0xff, 0x68, 0x00, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8c7e20, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c38b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c1e40, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x3b, 0x96, 0x00] },
    Prologo { endereco: 0x8c44e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x8bed60, bytes: &[0x6a, 0xff, 0x68, 0xc3, 0x3a, 0x96, 0x00] },
    Prologo { endereco: 0x8cbbe0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a7150, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x7ae2a0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x7ae240, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x7c3920, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c5d80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7ae350, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x51e220, bytes: &[0x81, 0xec, 0x28, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x522610, bytes: &[0x83, 0xec, 0x14, 0x56, 0x57] },
    Prologo { endereco: 0x4bf850, bytes: &[0x66, 0x8b, 0x4c, 0x24, 0x10] },
    Prologo { endereco: 0x4afa80, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x4afba0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x7f8d70, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x0f, 0x96, 0x00] },
    Prologo { endereco: 0x896e90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x897bb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cf6b0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x18] },
    Prologo { endereco: 0x8cf880, bytes: &[0x83, 0xec, 0x0c, 0x55, 0x8b, 0x6c, 0x24, 0x18] },
    Prologo { endereco: 0x8c7560, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8ce990, bytes: &[0x6a, 0xff, 0x68, 0x78, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8c2060, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c1780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8cca10, bytes: &[0x6a, 0xff, 0x68, 0x98, 0x3e, 0x96, 0x00] },
    Prologo { endereco: 0x8cbf70, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x8cb9a0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c6100, bytes: &[0x51, 0x55, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x8c47d0, bytes: &[0x6a, 0xff, 0x68, 0x88, 0x3c, 0x96, 0x00] },
    Prologo { endereco: 0x8caaf0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x68f130, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x6abef0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fe700, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7a79c0, bytes: &[0x83, 0xec, 0x0c, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x7a7310, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x7a58f0, bytes: &[0x6a, 0xff, 0x68, 0x88, 0xf1, 0x95, 0x00] },
    Prologo { endereco: 0x7a5790, bytes: &[0x6a, 0xff, 0x68, 0x58, 0xf1, 0x95, 0x00] },
    Prologo { endereco: 0x7aaeb0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x86d800, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0x2d, 0x96, 0x00] },
    Prologo { endereco: 0x62c400, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4cd230, bytes: &[0x6a, 0xff, 0x68, 0x50, 0x6d, 0x95, 0x00] },
    Prologo { endereco: 0x86fab0, bytes: &[0x6a, 0xff, 0x68, 0x08, 0x2e, 0x96, 0x00] },
    Prologo { endereco: 0x68ac40, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x68afb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] },
    Prologo { endereco: 0x68afd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5acbe0, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5abd90, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5ae6d0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c6b70, bytes: &[0x6a, 0xff, 0x68, 0x31, 0xa1, 0x95, 0x00] },
    Prologo { endereco: 0x522910, bytes: &[0x56, 0x66, 0x8b, 0x71, 0x3e] },
    Prologo { endereco: 0x5af5b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x4d8610, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4cbb20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x0b] },
    Prologo { endereco: 0x4cb960, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8a, 0x41, 0x3d] },
    Prologo { endereco: 0x8c6e60, bytes: &[0x6a, 0xff, 0x68, 0x68, 0x3d, 0x96, 0x00] },
    Prologo { endereco: 0x8c3e50, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x8d0ea0, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] },
    Prologo { endereco: 0x8c18c0, bytes: &[0x6a, 0xff, 0x68, 0xb0, 0x3b, 0x96, 0x00] },
    Prologo { endereco: 0x8cf470, bytes: &[0x6a, 0xff, 0x68, 0xd6, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8c72d0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x8c8fd0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c5560, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x8c54c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x5d6d10, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x5d7190, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x5ce0b0, bytes: &[0x83, 0xec, 0x1c, 0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x5d6aa0, bytes: &[0x51, 0x8b, 0x0d, 0x64, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x5ce060, bytes: &[0xa1, 0xf8, 0xad, 0xad, 0x00] },
    Prologo { endereco: 0x5cdde0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x85, 0xd2] },
    Prologo { endereco: 0x85b820, bytes: &[0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c] },
    Prologo { endereco: 0x85de20, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x85ddb0, bytes: &[0x83, 0xec, 0x14, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x7f9720, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x7f7490, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x86cc50, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x2d, 0x96, 0x00] },
    Prologo { endereco: 0x7c3d20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x86da20, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x2d, 0x96, 0x00] },
    Prologo { endereco: 0x632040, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x7c3bb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x86c730, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x870220, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x5ac7a0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5ab230, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5abc30, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5c2ae0, bytes: &[0x6a, 0xff, 0x68, 0x10, 0x9e, 0x95, 0x00] },
    Prologo { endereco: 0x5c6f00, bytes: &[0x6a, 0xff, 0x68, 0x5c, 0xa1, 0x95, 0x00] },
    Prologo { endereco: 0x5c4380, bytes: &[0x6a, 0xff, 0x68, 0xe4, 0x9f, 0x95, 0x00] },
    Prologo { endereco: 0x51f890, bytes: &[0x6a, 0xff, 0x68, 0x98, 0x77, 0x95, 0x00] },
    Prologo { endereco: 0x51c800, bytes: &[0x6a, 0xff, 0x68, 0xd9, 0x76, 0x95, 0x00] },
    Prologo { endereco: 0x5ab3c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x58d710, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x58d460, bytes: &[0x83, 0xec, 0x18, 0x53, 0x8b, 0x5c, 0x24, 0x20] },
    Prologo { endereco: 0x58e220, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x58dea0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x58e190, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x58dfc0, bytes: &[0x83, 0xec, 0x1c, 0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x48dd70, bytes: &[0x8b, 0x0d, 0x74, 0x2c, 0xae, 0x00] },
    Prologo { endereco: 0x669a10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x669ae0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x669470, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x58d610, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x58e240, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x922ee0, bytes: &[0x51, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x922c40, bytes: &[0x81, 0xec, 0xd0, 0x07, 0x00, 0x00] },
    Prologo { endereco: 0x922d20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0xe0, 0x0a, 0x00, 0x00] },
    Prologo { endereco: 0x5d5f20, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x5d6250, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x5d61d0, bytes: &[0x66, 0x0f, 0xbe, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x85da00, bytes: &[0x83, 0xec, 0x0c, 0xa1, 0xe4, 0xf2, 0x9c, 0x00] },
    Prologo { endereco: 0x85d8a0, bytes: &[0x83, 0xec, 0x10, 0x8b, 0x4c, 0x24, 0x14] },
    Prologo { endereco: 0x922810, bytes: &[0x81, 0xec, 0xf0, 0x05, 0x00, 0x00] },
    Prologo { endereco: 0x8b41d0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c] },
    Prologo { endereco: 0x8b4cd0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b4f60, bytes: &[0x51, 0x53, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x8b2be0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b4b20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b4db0, bytes: &[0x51, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x8b1730, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x7a6e30, bytes: &[0x8b, 0x41, 0x1c, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x62ed30, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x5c2460, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5c27a0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c2d40, bytes: &[0x6a, 0xff, 0x68, 0x4e, 0x9e, 0x95, 0x00] },
    Prologo { endereco: 0x5c4040, bytes: &[0x6a, 0xff, 0x68, 0x9e, 0x9f, 0x95, 0x00] },
    Prologo { endereco: 0x5c5270, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5c5290, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5223a0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c08f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4a2540, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x669340, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x32, 0xc0] },
    Prologo { endereco: 0x669360, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x669950, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x669bd0, bytes: &[0x6a, 0xff, 0x68, 0x58, 0xc6, 0x95, 0x00] },
    Prologo { endereco: 0x6695c0, bytes: &[0xa1, 0x08, 0x2d, 0xae, 0x00] },
    Prologo { endereco: 0x669600, bytes: &[0xa1, 0x08, 0x2d, 0xae, 0x00] },
    Prologo { endereco: 0x7c1e30, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c6d40, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x7c6d90, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x867b50, bytes: &[0x8a, 0x51, 0x1c, 0xb0, 0x02] },
    Prologo { endereco: 0x868210, bytes: &[0x6a, 0xff, 0x68, 0x08, 0x2b, 0x96, 0x00] },
    Prologo { endereco: 0x867aa0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x61] },
    Prologo { endereco: 0x923630, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x5a0590, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x85e380, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x923010, bytes: &[0x51, 0x53, 0x55, 0x8b, 0xe9] },
    Prologo { endereco: 0x9232c0, bytes: &[0x0f, 0xbe, 0x4c, 0x24, 0x04] },
    Prologo { endereco: 0x923970, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x53] },
    Prologo { endereco: 0x5986b0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x5d6220, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x85e350, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x0c] },
    Prologo { endereco: 0x8b3d40, bytes: &[0x81, 0xec, 0x0c, 0x0c, 0x00, 0x00] },
    Prologo { endereco: 0x8b4e50, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x8b3930, bytes: &[0x8b, 0x54, 0x24, 0x04, 0xa1, 0x64, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x8b3c50, bytes: &[0xa1, 0x6c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x7a8350, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a6e20, bytes: &[0x8b, 0x41, 0x20, 0x85, 0xc0] },
    Prologo { endereco: 0x7e8e10, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x7e7ed0, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x7e9050, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b3560, bytes: &[0x81, 0xec, 0x80, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b3460, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b57d0, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x686a70, bytes: &[0x81, 0xec, 0x68, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x521790, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5219b0, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7bca90, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10] },
    Prologo { endereco: 0x7bd830, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x7bd870, bytes: &[0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x8c2f50, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x8d76d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8e1780, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c28c0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6a4c40, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xce, 0x95, 0x00] },
    Prologo { endereco: 0x6a4a20, bytes: &[0x6a, 0xff, 0x68, 0x4b, 0xce, 0x95, 0x00] },
    Prologo { endereco: 0x8c4e10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x0c] },
    Prologo { endereco: 0x6a44a0, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0xce, 0x95, 0x00] },
    Prologo { endereco: 0x6a5d10, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0xce, 0x95, 0x00] },
    Prologo { endereco: 0x8be9d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x69fd50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8d8310, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x59b7e0, bytes: &[0x51, 0x53, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x599a20, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x59b630, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x59ed70, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x59dd40, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x59e5a0, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x597db0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x41, 0x08] },
    Prologo { endereco: 0x599960, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x38, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x598340, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x62c080, bytes: &[0x83, 0xec, 0x20, 0x53, 0x55] },
    Prologo { endereco: 0x62bf60, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x28] },
    Prologo { endereco: 0x631550, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x28, 0x3d, 0xb6, 0x00] },
    Prologo { endereco: 0x7a6f10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7a7f20, bytes: &[0x6a, 0xff, 0x68, 0xf0, 0xf1, 0x95, 0x00] },
    Prologo { endereco: 0x7aa340, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x62e700, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x72bbf0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x787630, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x775cd0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x76f7d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7701f0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x76db00, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0xdd, 0x95, 0x00] },
    Prologo { endereco: 0x76da80, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x5c57d0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x86c8c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5c52d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x80, 0x80, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5c74d0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x866620, bytes: &[0x6a, 0xff, 0x68, 0x90, 0x2a, 0x96, 0x00] },
    Prologo { endereco: 0x4d9f80, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4d9460, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c3310, bytes: &[0x6a, 0xff, 0x68, 0x10, 0x3c, 0x96, 0x00] },
    Prologo { endereco: 0x8c34e0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x8c91e0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] },
    Prologo { endereco: 0x842740, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4d6790, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4d6d70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4cc490, bytes: &[0x6a, 0xff, 0x68, 0x29, 0x6d, 0x95, 0x00] },
    Prologo { endereco: 0x4ccb30, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9] },
    Prologo { endereco: 0x4ce450, bytes: &[0x6a, 0xff, 0x68, 0x88, 0x6d, 0x95, 0x00] },
    Prologo { endereco: 0x767ee0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] },
    Prologo { endereco: 0x767fa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x768130, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x767150, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x09] },
    Prologo { endereco: 0x7671c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x412f00, bytes: &[0x53, 0x55, 0x8b, 0x4c, 0x24, 0x1c] },
    Prologo { endereco: 0x62c680, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0xb0, 0x95, 0x00] },
    Prologo { endereco: 0x690bd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x09] },
    Prologo { endereco: 0x69f7c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x62b7b0, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9] },
    Prologo { endereco: 0x62e010, bytes: &[0x6a, 0xff, 0x68, 0x68, 0xb1, 0x95, 0x00] },
    Prologo { endereco: 0x55d360, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x7a65d0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x757f30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x66, 0x81, 0xb8, 0x80, 0x00, 0x00, 0x00, 0x6a, 0x18] },
    Prologo { endereco: 0x757fa0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61] },
    Prologo { endereco: 0x685d30, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x687710, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x68a850, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x76b600, bytes: &[0x83, 0xec, 0x60, 0x56, 0x57] },
    Prologo { endereco: 0x775610, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x48d410, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8b, 0xc1] },
    Prologo { endereco: 0x7771c0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xdf, 0x95, 0x00] },
    Prologo { endereco: 0x48e190, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x48d200, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4190d0, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x55] },
    Prologo { endereco: 0x418ff0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x419070, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x41a360, bytes: &[0x6a, 0xff, 0x68, 0xe8, 0x3e, 0x95, 0x00] },
    Prologo { endereco: 0x419710, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] },
    Prologo { endereco: 0x867560, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8bf680, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] },
    Prologo { endereco: 0x8bfcb0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] },
    Prologo { endereco: 0x8d5170, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0x43, 0x96, 0x00] },
    Prologo { endereco: 0x8c1290, bytes: &[0x83, 0xec, 0x20, 0x53, 0x8b, 0x5c, 0x24, 0x28] },
    Prologo { endereco: 0x8de790, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8db8a0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c9390, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] },
    Prologo { endereco: 0x8dbef0, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x45, 0x96, 0x00] },
    Prologo { endereco: 0x8e05d0, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x46, 0x96, 0x00] },
    Prologo { endereco: 0x8bf400, bytes: &[0x6a, 0xff, 0x68, 0x28, 0x3b, 0x96, 0x00] },
    Prologo { endereco: 0x4da590, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8dda90, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x896660, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fbe20, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fbd00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x896c60, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8973e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x897250, bytes: &[0x8a, 0x81, 0xf0, 0x05, 0x00, 0x00] },
    Prologo { endereco: 0x6a0e70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6a1b00, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6a2e00, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x682e10, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x686940, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x3c, 0x05] },
    Prologo { endereco: 0x686670, bytes: &[0x6a, 0xff, 0x68, 0x8b, 0xca, 0x95, 0x00] },
    Prologo { endereco: 0x7c0d70, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x447430, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x448e30, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x76bab0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x76fd10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x519690, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x75, 0x95, 0x00] },
    Prologo { endereco: 0x844540, bytes: &[0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c] },
    Prologo { endereco: 0x844280, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x8442d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x843e10, bytes: &[0x6a, 0xff, 0x68, 0x18, 0x25, 0x96, 0x00] },
    Prologo { endereco: 0x8442b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x84b1f0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x83b6e0, bytes: &[0x83, 0xec, 0x2c, 0x53, 0x55] },
    Prologo { endereco: 0x84b4d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7fc150, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5cbb00, bytes: &[0x6a, 0xff, 0x68, 0xa0, 0xa3, 0x95, 0x00] },
    Prologo { endereco: 0x7f6e30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7f6f20, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x584fa0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x584990, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5851d0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x583b10, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x586c00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x48e360, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbf, 0x48, 0x30] },
    Prologo { endereco: 0x79c7b0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x79bf40, bytes: &[0x8b, 0x51, 0x04, 0x33, 0xc0] },
    Prologo { endereco: 0x79cee0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x79c5c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x79caa0, bytes: &[0x8b, 0x0d, 0x90, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x79c530, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] },
    Prologo { endereco: 0x86c390, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7e4a30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7e4b00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c39f0, bytes: &[0x81, 0xec, 0xc8, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x8cad90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8da5e0, bytes: &[0x6a, 0xff, 0x68, 0x96, 0x44, 0x96, 0x00] },
    Prologo { endereco: 0x696b10, bytes: &[0x83, 0xec, 0x14, 0x8a, 0x44, 0x24, 0x24] },
    Prologo { endereco: 0x6ac700, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6ac880, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c1af0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fd2f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8d7cd0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x573080, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x576a70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5783c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x576c50, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x573260, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x578660, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x684cd0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x572ff0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x5769e0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x584ae0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x586330, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x586660, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x896290, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x8976a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x897880, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7757d0, bytes: &[0x6a, 0xff, 0x68, 0xf0, 0xde, 0x95, 0x00] },
    Prologo { endereco: 0x76d9b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] },
    Prologo { endereco: 0x775580, bytes: &[0x53, 0x55, 0x8b, 0xe9, 0x32, 0xdb] },
    Prologo { endereco: 0x7e4750, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7e4920, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7e56d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5ae9d0, bytes: &[0x6a, 0xff, 0x68, 0x46, 0x98, 0x95, 0x00] },
    Prologo { endereco: 0x5abb10, bytes: &[0x6a, 0xff, 0x68, 0x40, 0x97, 0x95, 0x00] },
    Prologo { endereco: 0x5aac70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x41add0, bytes: &[0x6a, 0xff, 0x68, 0x36, 0x3f, 0x95, 0x00] },
    Prologo { endereco: 0x5a74b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5a7f20, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7dab90, bytes: &[0xb8, 0x94, 0x10, 0x00, 0x00] },
    Prologo { endereco: 0x7dad20, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] },
    Prologo { endereco: 0x758de0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x598370, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x59cf50, bytes: &[0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x59b4c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x599800, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x599890, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x5998e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x3c, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x5999a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x48, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x5987c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x58, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x599920, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x30, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x522240, bytes: &[0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c] },
    Prologo { endereco: 0x51e870, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x08] },
    Prologo { endereco: 0x521b30, bytes: &[0x66, 0x8b, 0x41, 0x3c, 0x53] },
    Prologo { endereco: 0x5c5f40, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xa0, 0x95, 0x00] },
    Prologo { endereco: 0x5c5570, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5ca3e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x51c410, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4bffc0, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0xc8, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x522360, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x57] },
    Prologo { endereco: 0x519890, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x519a90, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x519fc0, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] },
    Prologo { endereco: 0x8bfa30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x55] },
    Prologo { endereco: 0x8cd450, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8d0dd0, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] },
    Prologo { endereco: 0x7af600, bytes: &[0x6a, 0xff, 0x68, 0x86, 0xf3, 0x95, 0x00] },
    Prologo { endereco: 0x7f7270, bytes: &[0x81, 0xec, 0x30, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7bd700, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x840d80, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x7f8610, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x8d0f00, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] },
    Prologo { endereco: 0x4ca6d0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x4cc2d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61] },
    Prologo { endereco: 0x4cc2e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61] },
    Prologo { endereco: 0x4d0c60, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x4cc3a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x4cc130, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6aaee0, bytes: &[0x81, 0xec, 0xdc, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6ab670, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0xd0, 0x95, 0x00] },
    Prologo { endereco: 0x6ab2d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x64, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x868830, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] },
    Prologo { endereco: 0x86a070, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x2b, 0x96, 0x00] },
    Prologo { endereco: 0x86dc30, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x2d, 0x96, 0x00] },
    Prologo { endereco: 0x86b0b0, bytes: &[0x6a, 0xff, 0x68, 0x46, 0x2c, 0x96, 0x00] },
    Prologo { endereco: 0x86b710, bytes: &[0x6a, 0xff, 0x68, 0x66, 0x2c, 0x96, 0x00] },
    Prologo { endereco: 0x869aa0, bytes: &[0x6a, 0xff, 0x68, 0xc1, 0x2b, 0x96, 0x00] },
    Prologo { endereco: 0x758200, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] },
    Prologo { endereco: 0x75b6e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x75cfc0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x59a5d0, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x59af40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x599850, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x28, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x59e050, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] },
    Prologo { endereco: 0x59e940, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x5999e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x20, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x5ca4f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5cc510, bytes: &[0x51, 0x66, 0xa1, 0x92, 0x2c, 0xae, 0x00] },
    Prologo { endereco: 0x5c6830, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x5c4770, bytes: &[0x81, 0xec, 0x50, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c6e00, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56, 0x57] },
    Prologo { endereco: 0x5c68b0, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x86, 0xc8, 0x34, 0x00, 0x00] },
    Prologo { endereco: 0x62d250, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x62dca0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x418f60, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x8e4f20, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x48, 0x96, 0x00] },
    Prologo { endereco: 0x8cb0f0, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x8db3a0, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x44, 0x96, 0x00] },
    Prologo { endereco: 0x8c7950, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] },
    Prologo { endereco: 0x8c5800, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x8c6200, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x10] },
    Prologo { endereco: 0x755340, bytes: &[0x81, 0xec, 0xf4, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x758010, bytes: &[0x83, 0xec, 0x10, 0x53, 0x57] },
    Prologo { endereco: 0x735160, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x85, 0xc0] },
    Prologo { endereco: 0x557120, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x7c, 0x95, 0x00] },
    Prologo { endereco: 0x557000, bytes: &[0x53, 0x55, 0x8b, 0x5c, 0x24, 0x0c] },
    Prologo { endereco: 0x55d3c0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x68d1f0, bytes: &[0x8b, 0xc1, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x68d080, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x02] },
    Prologo { endereco: 0x68d2e0, bytes: &[0x8b, 0x41, 0x0a, 0x85, 0xc0] },
    Prologo { endereco: 0x51d2d0, bytes: &[0x81, 0xec, 0x2c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x51d9f0, bytes: &[0x81, 0xec, 0x30, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x518780, bytes: &[0xc6, 0x81, 0xb1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x48dee0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] },
    Prologo { endereco: 0x48df40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] },
    Prologo { endereco: 0x48dcd0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5ae380, bytes: &[0x6a, 0xff, 0x68, 0x16, 0x98, 0x95, 0x00] },
    Prologo { endereco: 0x5aee40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x5c, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x5aee60, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x5c, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x59b3b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x59b2a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x859cc0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x905510, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c6b70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5fb320, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x443f50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x79bb80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84c680, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5af040, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x5aee90, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] },
    Prologo { endereco: 0x5af4a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x59cdd0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x59cfa0, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x59fe10, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0x91, 0x95, 0x00] },
    Prologo { endereco: 0x5968d0, bytes: &[0x83, 0xec, 0x34, 0x53, 0x56] },
    Prologo { endereco: 0x59ffb0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0x91, 0x95, 0x00] },
    Prologo { endereco: 0x59fae0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5adba0, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x97, 0x95, 0x00] },
    Prologo { endereco: 0x5ad780, bytes: &[0x6a, 0xff, 0x68, 0xc9, 0x97, 0x95, 0x00] },
    Prologo { endereco: 0x5af390, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5ab510, bytes: &[0x6a, 0xff, 0x68, 0x07, 0x97, 0x95, 0x00] },
    Prologo { endereco: 0x5aa680, bytes: &[0x6a, 0xff, 0x68, 0x24, 0x96, 0x95, 0x00] },
    Prologo { endereco: 0x5aa2d0, bytes: &[0x6a, 0xff, 0x68, 0xea, 0x95, 0x95, 0x00] },
    Prologo { endereco: 0x51cb90, bytes: &[0x6a, 0xff, 0x68, 0x11, 0x77, 0x95, 0x00] },
    Prologo { endereco: 0x522950, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x77, 0x95, 0x00] },
    Prologo { endereco: 0x8c6bc0, bytes: &[0x6a, 0xff, 0x68, 0x48, 0x3d, 0x96, 0x00] },
    Prologo { endereco: 0x51ebe0, bytes: &[0x6a, 0xff, 0x68, 0x5a, 0x77, 0x95, 0x00] },
    Prologo { endereco: 0x8c41c0, bytes: &[0x6a, 0xff, 0x68, 0x68, 0x3c, 0x96, 0x00] },
    Prologo { endereco: 0x8c7690, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] },
    Prologo { endereco: 0x695240, bytes: &[0x83, 0xec, 0x2c, 0x8b, 0x44, 0x24, 0x3c] },
    Prologo { endereco: 0x6a5fa0, bytes: &[0x6a, 0xff, 0x68, 0x16, 0xcf, 0x95, 0x00] },
    Prologo { endereco: 0x6a5a60, bytes: &[0x6a, 0xff, 0x68, 0xcb, 0xce, 0x95, 0x00] },
    Prologo { endereco: 0x6a3770, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x69ba50, bytes: &[0x6a, 0xff, 0x68, 0xa0, 0xcc, 0x95, 0x00] },
    Prologo { endereco: 0x6a0540, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x754a30, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xd8, 0x95, 0x00] },
    Prologo { endereco: 0x587600, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x75c390, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5cbdf0, bytes: &[0x6a, 0xff, 0x68, 0xc6, 0xa3, 0x95, 0x00] },
    Prologo { endereco: 0x5ca090, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5ca290, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x76b100, bytes: &[0x6a, 0xff, 0x68, 0x08, 0xdd, 0x95, 0x00] },
    Prologo { endereco: 0x7f8420, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x7f8500, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] },
    Prologo { endereco: 0x76c120, bytes: &[0x6a, 0xff, 0x68, 0x7a, 0xdd, 0x95, 0x00] },
    Prologo { endereco: 0x7f8c00, bytes: &[0x6a, 0xff, 0x68, 0xd8, 0x0f, 0x96, 0x00] },
    Prologo { endereco: 0x7f6be0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7f6af0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8c5880, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x8c63a0, bytes: &[0x6a, 0xff, 0x68, 0x00, 0x3d, 0x96, 0x00] },
    Prologo { endereco: 0x8cfc90, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0x3f, 0x96, 0x00] },
    Prologo { endereco: 0x8c0080, bytes: &[0x6a, 0xff, 0x68, 0x48, 0x3b, 0x96, 0x00] },
    Prologo { endereco: 0x8d3090, bytes: &[0x6a, 0xff, 0x68, 0xc6, 0x42, 0x96, 0x00] },
    Prologo { endereco: 0x8d0040, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x40, 0x96, 0x00] },
    Prologo { endereco: 0x6847c0, bytes: &[0x6a, 0xff, 0x68, 0x21, 0xca, 0x95, 0x00] },
    Prologo { endereco: 0x6835c0, bytes: &[0x6a, 0xff, 0x68, 0x81, 0xc9, 0x95, 0x00] },
    Prologo { endereco: 0x689c80, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5aaf00, bytes: &[0x6a, 0xff, 0x68, 0x70, 0x96, 0x95, 0x00] },
    Prologo { endereco: 0x5abf10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5ac3b0, bytes: &[0x81, 0xec, 0x54, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x518790, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x48d2b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x48cec0, bytes: &[0x81, 0xec, 0x00, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x8b1750, bytes: &[0x81, 0xec, 0x34, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x8b39f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b4f50, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] },
    Prologo { endereco: 0x8b3640, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b37c0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b4770, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x08] },
    Prologo { endereco: 0x7a59e0, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0xf1, 0x95, 0x00] },
    Prologo { endereco: 0x7a92e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7a8ba0, bytes: &[0x6a, 0xff, 0x68, 0x0b, 0xf2, 0x95, 0x00] },
    Prologo { endereco: 0x4196a0, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] },
    Prologo { endereco: 0x4197c0, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] },
    Prologo { endereco: 0x41a630, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x84b6f0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x83b6c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x84b580, bytes: &[0x6a, 0xff, 0x68, 0xe8, 0x25, 0x96, 0x00] },
    Prologo { endereco: 0x868c50, bytes: &[0x6a, 0xff, 0x68, 0x76, 0x2b, 0x96, 0x00] },
    Prologo { endereco: 0x86a800, bytes: &[0x6a, 0xff, 0x68, 0x26, 0x2c, 0x96, 0x00] },
    Prologo { endereco: 0x86fc70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x64, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x8c8090, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x3d, 0x96, 0x00] },
    Prologo { endereco: 0x8bee50, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x3a, 0x96, 0x00] },
    Prologo { endereco: 0x8c8860, bytes: &[0x6a, 0xff, 0x68, 0x07, 0x3e, 0x96, 0x00] },
    Prologo { endereco: 0x4ccca0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84a020, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x846920, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x85e410, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x29, 0x96, 0x00] },
    Prologo { endereco: 0x579d60, bytes: &[0x83, 0xec, 0x6c, 0x55, 0x56] },
    Prologo { endereco: 0x579fd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x57a020, bytes: &[0x83, 0xec, 0x08, 0x56, 0x6a, 0xff] },
    Prologo { endereco: 0x579c70, bytes: &[0x6a, 0xff, 0x68, 0x08, 0x87, 0x95, 0x00] },
    Prologo { endereco: 0x7f6650, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] },
    Prologo { endereco: 0x7f6530, bytes: &[0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00] },
    Prologo { endereco: 0x661770, bytes: &[0x83, 0xec, 0x64, 0x55, 0x56] },
    Prologo { endereco: 0x63c3d0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x859630, bytes: &[0x83, 0xec, 0x64, 0x56, 0x57] },
    Prologo { endereco: 0x685640, bytes: &[0x6a, 0xff, 0x68, 0x72, 0xca, 0x95, 0x00] },
    Prologo { endereco: 0x684230, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x683ee0, bytes: &[0x6a, 0xff, 0x68, 0xbc, 0xc9, 0x95, 0x00] },
    Prologo { endereco: 0x8be300, bytes: &[0x6a, 0xff, 0x68, 0x6a, 0x3a, 0x96, 0x00] },
    Prologo { endereco: 0x8c0660, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x55] },
    Prologo { endereco: 0x8c2180, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4d71b0, bytes: &[0x6a, 0xff, 0x68, 0x16, 0x6f, 0x95, 0x00] },
    Prologo { endereco: 0x4cda70, bytes: &[0x6a, 0xff, 0x68, 0x68, 0x6d, 0x95, 0x00] },
    Prologo { endereco: 0x92e940, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92e7f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92f1a0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x0c] },
    Prologo { endereco: 0x92d0f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92d540, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x92f680, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92c290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92e740, bytes: &[0x66, 0x8b, 0x41, 0x36, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92f230, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8b2880, bytes: &[0x83, 0xec, 0x34, 0x53, 0x56] },
    Prologo { endereco: 0x8b2470, bytes: &[0x81, 0xec, 0xd0, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x8b2230, bytes: &[0x56, 0x57, 0x8b, 0xf1, 0xbf, 0xa7, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x7c3e40, bytes: &[0x81, 0xec, 0x48, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c6e30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7c6de0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x7a32e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x630ee0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x630c50, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7b5be0, bytes: &[0x6a, 0xff, 0x68, 0x68, 0xf5, 0x95, 0x00] },
    Prologo { endereco: 0x7bd310, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7706c0, bytes: &[0x6a, 0xff, 0x68, 0xa6, 0xde, 0x95, 0x00] },
    Prologo { endereco: 0x76e0b0, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0xdd, 0x95, 0x00] },
    Prologo { endereco: 0x76d560, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xdd, 0x95, 0x00] },
    Prologo { endereco: 0x7f8750, bytes: &[0x6a, 0xff, 0x68, 0xbe, 0x0f, 0x96, 0x00] },
    Prologo { endereco: 0x7f9890, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7f9a60, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x10, 0x96, 0x00] },
    Prologo { endereco: 0x4cf720, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7abac0, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x10] },
    Prologo { endereco: 0x56a090, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x5b5d10, bytes: &[0x81, 0xec, 0x08, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x43f2d0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8] },
    Prologo { endereco: 0x43f300, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8] },
    Prologo { endereco: 0x79bf60, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x79d410, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] },
    Prologo { endereco: 0x5c6570, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5cb9e0, bytes: &[0x6a, 0xff, 0x68, 0x80, 0xa3, 0x95, 0x00] },
    Prologo { endereco: 0x5c8260, bytes: &[0x6a, 0xff, 0x68, 0x77, 0xa2, 0x95, 0x00] },
    Prologo { endereco: 0x79d4e0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x79d480, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] },
    Prologo { endereco: 0x7c5fe0, bytes: &[0x81, 0xec, 0x3c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7a50f0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x14] },
    Prologo { endereco: 0x7a5090, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x0c] },
    Prologo { endereco: 0x7a3140, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x585290, bytes: &[0x6a, 0xff, 0x68, 0xde, 0x8a, 0x95, 0x00] },
    Prologo { endereco: 0x586f60, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x586900, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x585ab0, bytes: &[0x6a, 0xff, 0x68, 0xfe, 0x8a, 0x95, 0x00] },
    Prologo { endereco: 0x5871e0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] },
    Prologo { endereco: 0x4b01d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x3d, 0xe8, 0x03, 0x00, 0x00] },
    Prologo { endereco: 0x583c50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x68aa80, bytes: &[0xa1, 0x44, 0x7a, 0xb6, 0x00] },
    Prologo { endereco: 0x6831a0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] },
    Prologo { endereco: 0x48ce10, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x1c, 0xff] },
    Prologo { endereco: 0x48e1c0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x48cc70, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] },
    Prologo { endereco: 0x903b50, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x903cc0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x9050d0, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x904040, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x904210, bytes: &[0x81, 0xec, 0x64, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x904e30, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] },
    Prologo { endereco: 0x904580, bytes: &[0x81, 0xec, 0x48, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x903890, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x52, 0x96, 0x00] },
    Prologo { endereco: 0x9035a0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4c6430, bytes: &[0xa1, 0x3c, 0xf6, 0x9c, 0x00] },
    Prologo { endereco: 0x4c6730, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x4c6890, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] },
    Prologo { endereco: 0x4c6280, bytes: &[0xa1, 0x5c, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x4c61b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c6960, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] },
    Prologo { endereco: 0x5fa730, bytes: &[0x8b, 0x01, 0x53, 0x55, 0x56] },
    Prologo { endereco: 0x5fa7d0, bytes: &[0x8b, 0x01, 0x56, 0x66, 0x8b, 0x71, 0x04] },
    Prologo { endereco: 0x5fa850, bytes: &[0x6a, 0xff, 0x68, 0x78, 0xaf, 0x95, 0x00] },
    Prologo { endereco: 0x579b30, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x86, 0x95, 0x00] },
    Prologo { endereco: 0x90b940, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x55, 0x96, 0x00] },
    Prologo { endereco: 0x412c80, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x3b, 0x95, 0x00] },
    Prologo { endereco: 0x859470, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6613f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c15e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43f0e0, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x4e, 0x95, 0x00] },
    Prologo { endereco: 0x5dffe0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7f6390, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x0e, 0x96, 0x00] },
    Prologo { endereco: 0x633d40, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x633c10, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x53] },
    Prologo { endereco: 0x9030e0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04] },
    Prologo { endereco: 0x4c11a0, bytes: &[0x81, 0xec, 0xa0, 0x09, 0x00, 0x00] },
    Prologo { endereco: 0x4c1620, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c15f0, bytes: &[0x56, 0x8b, 0xf1, 0x6a, 0x00] },
    Prologo { endereco: 0x63f350, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04] },
    Prologo { endereco: 0x63d6e0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04] },
    Prologo { endereco: 0x426230, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03] },
    Prologo { endereco: 0x401ba0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x5e13b0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x91e840, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x4c3890, bytes: &[0x6a, 0xff, 0x68, 0x78, 0x6a, 0x95, 0x00] },
    Prologo { endereco: 0x4c1ed0, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x69, 0x95, 0x00] },
    Prologo { endereco: 0x7979b0, bytes: &[0x6a, 0xff, 0x68, 0xe8, 0xea, 0x95, 0x00] },
    Prologo { endereco: 0x9191e0, bytes: &[0x6a, 0xff, 0x68, 0x98, 0x59, 0x96, 0x00] },
    Prologo { endereco: 0x921040, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x912eb0, bytes: &[0x6a, 0xff, 0x68, 0x48, 0x57, 0x96, 0x00] },
    Prologo { endereco: 0x5e12d0, bytes: &[0x6a, 0xff, 0x68, 0x68, 0xa8, 0x95, 0x00] },
    Prologo { endereco: 0x401ab0, bytes: &[0x6a, 0xff, 0x68, 0xd8, 0x34, 0x95, 0x00] },
    Prologo { endereco: 0x91ecf0, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0x5a, 0x96, 0x00] },
    Prologo { endereco: 0x4c3f40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c4080, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x4c3a80, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x401e10, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x4019c0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x401b90, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x0c] },
    Prologo { endereco: 0x42a5a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x42cb70, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x42de50, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x430ff0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0] },
    Prologo { endereco: 0x4322d0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0] },
    Prologo { endereco: 0x43bc70, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x437fc0, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x439430, bytes: &[0x56, 0x8b, 0xf1, 0x33, 0xc0] },
    Prologo { endereco: 0x43a600, bytes: &[0x56, 0x8b, 0xf1, 0x33, 0xc0] },
    Prologo { endereco: 0x433640, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0] },
    Prologo { endereco: 0x663620, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x665140, bytes: &[0x56, 0x8b, 0xf1, 0xb2, 0x03] },
    Prologo { endereco: 0x411ce0, bytes: &[0x56, 0x8b, 0xf1, 0x33, 0xc0] },
    Prologo { endereco: 0x908660, bytes: &[0xb2, 0x02, 0xb8, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x432970, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x08] },
    Prologo { endereco: 0x408eb0, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x08] },
    Prologo { endereco: 0x42abc0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x42eb00, bytes: &[0x51, 0x8a, 0x44, 0x24, 0x10] },
    Prologo { endereco: 0x430210, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x4314e0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x434d90, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x411980, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x3c, 0xff] },
    Prologo { endereco: 0x4350d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x909dd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x42e100, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x908200, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] },
    Prologo { endereco: 0x9099d0, bytes: &[0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] },
    Prologo { endereco: 0x90a000, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x574b70, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x572d90, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x576780, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5780c0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03] },
    Prologo { endereco: 0x574a20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x576630, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x577f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x572bf0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x641640, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6468a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x64b770, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03] },
    Prologo { endereco: 0x64f920, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x641510, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x646780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x64b670, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x64f820, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bc410, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5bf0a0, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5c0700, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5bc2b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bef10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5c05d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84fdf0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x855cc0, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x853790, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x84fc70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x654180, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] },
    Prologo { endereco: 0x850880, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x5b9fa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5be230, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x64bd80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x64ff40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x56c570, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56f320, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56d640, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5ba3f0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bcc40, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84d220, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bf5d0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x858960, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x666020, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56ccc0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56b3c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56d780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5baa50, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bd160, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bf770, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x858aa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x666160, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x90a400, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56caf0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x570550, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x90a900, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0x54, 0x96, 0x00] },
    Prologo { endereco: 0x665640, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x661ff0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x90b170, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x665a00, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x90b530, bytes: &[0x81, 0xec, 0x98, 0x03, 0x00, 0x00] },
    Prologo { endereco: 0x6624f0, bytes: &[0x81, 0xec, 0xc8, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x85a760, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] },
    Prologo { endereco: 0x85add0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x85a850, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x85a470, bytes: &[0x81, 0xec, 0x08, 0x01, 0x00, 0x00] },
    Prologo { endereco: 0x859fe0, bytes: &[0x81, 0xec, 0x90, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x859dc0, bytes: &[0x55, 0x8b, 0xe9, 0x57, 0x33, 0xff] },
    Prologo { endereco: 0x859e20, bytes: &[0x81, 0xec, 0xd8, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x85aae0, bytes: &[0x81, 0xec, 0x9c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5f9690, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0xaf, 0x95, 0x00] },
    Prologo { endereco: 0x518500, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x75, 0x95, 0x00] },
    Prologo { endereco: 0x63c210, bytes: &[0x6a, 0xff, 0x68, 0xab, 0xb7, 0x95, 0x00] },
    Prologo { endereco: 0x7944f0, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0xe9, 0x95, 0x00] },
    Prologo { endereco: 0x7936d0, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xe7, 0x95, 0x00] },
    Prologo { endereco: 0x90f080, bytes: &[0x6a, 0xff, 0x68, 0x4b, 0x56, 0x96, 0x00] },
    Prologo { endereco: 0x9020a0, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x51, 0x96, 0x00] },
    Prologo { endereco: 0x7cd830, bytes: &[0x6a, 0xff, 0x68, 0xab, 0xf9, 0x95, 0x00] },
    Prologo { endereco: 0x5a9950, bytes: &[0x6a, 0xff, 0x68, 0x4b, 0x95, 0x95, 0x00] },
    Prologo { endereco: 0x40a620, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7d3df0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x66e630, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0xc8, 0x95, 0x00] },
    Prologo { endereco: 0x5ecf20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x7ec7e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x424c00, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x85a9b0, bytes: &[0x83, 0xec, 0x10, 0x8a, 0x44, 0x24, 0x14] },
    Prologo { endereco: 0x58dd50, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] },
    Prologo { endereco: 0x85dfc0, bytes: &[0x81, 0xec, 0x30, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42a480, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42ca70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42dc90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42fd10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x430ef0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4321d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x433540, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x434810, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x435d80, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x436d90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x437ec0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x439330, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43a500, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43bb70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43cc40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6634e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x665030, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x411b70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x430050, bytes: &[0x6a, 0xff, 0x68, 0x2e, 0x46, 0x95, 0x00] },
    Prologo { endereco: 0x4360c0, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x48, 0x95, 0x00] },
    Prologo { endereco: 0x4370d0, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x48, 0x95, 0x00] },
    Prologo { endereco: 0x439710, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x49, 0x95, 0x00] },
    Prologo { endereco: 0x43bf50, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x4a, 0x95, 0x00] },
    Prologo { endereco: 0x43cf80, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0x4b, 0x95, 0x00] },
    Prologo { endereco: 0x4325b0, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0x47, 0x95, 0x00] },
    Prologo { endereco: 0x4312d0, bytes: &[0x6a, 0xff, 0x68, 0x9e, 0x46, 0x95, 0x00] },
    Prologo { endereco: 0x433930, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x47, 0x95, 0x00] },
    Prologo { endereco: 0x642880, bytes: &[0x83, 0xec, 0x1c, 0x8b, 0x44, 0x24, 0x20] },
    Prologo { endereco: 0x647050, bytes: &[0x83, 0xec, 0x18, 0xb0, 0x02] },
    Prologo { endereco: 0x6471a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] },
    Prologo { endereco: 0x641dc0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x3c] },
    Prologo { endereco: 0x64b9e0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x64ba70, bytes: &[0x6a, 0xff, 0x68, 0xec, 0xba, 0x95, 0x00] },
    Prologo { endereco: 0x64fba0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x64fc30, bytes: &[0x6a, 0xff, 0x68, 0x8c, 0xbb, 0x95, 0x00] },
    Prologo { endereco: 0x578330, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x573380, bytes: &[0x6a, 0xff, 0x68, 0xc9, 0x83, 0x95, 0x00] },
    Prologo { endereco: 0x571680, bytes: &[0x6a, 0xff, 0x68, 0x59, 0x83, 0x95, 0x00] },
    Prologo { endereco: 0x5754a0, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x84, 0x95, 0x00] },
    Prologo { endereco: 0x576dd0, bytes: &[0x6a, 0xff, 0x68, 0x89, 0x84, 0x95, 0x00] },
    Prologo { endereco: 0x6406d0, bytes: &[0x6a, 0xff, 0x68, 0x59, 0xb9, 0x95, 0x00] },
    Prologo { endereco: 0x645810, bytes: &[0x6a, 0xff, 0x68, 0x09, 0xba, 0x95, 0x00] },
    Prologo { endereco: 0x64aa70, bytes: &[0x6a, 0xff, 0x68, 0x99, 0xba, 0x95, 0x00] },
    Prologo { endereco: 0x64ec20, bytes: &[0x6a, 0xff, 0x68, 0x39, 0xbb, 0x95, 0x00] },
    Prologo { endereco: 0x5be330, bytes: &[0x6a, 0xff, 0x68, 0x89, 0x9a, 0x95, 0x00] },
    Prologo { endereco: 0x5bfb40, bytes: &[0x6a, 0xff, 0x68, 0x29, 0x9b, 0x95, 0x00] },
    Prologo { endereco: 0x8539a0, bytes: &[0x6a, 0xff, 0x68, 0x59, 0x27, 0x96, 0x00] },
    Prologo { endereco: 0x84e130, bytes: &[0x6a, 0xff, 0x68, 0xa7, 0x26, 0x96, 0x00] },
    Prologo { endereco: 0x851020, bytes: &[0x6a, 0xff, 0x68, 0x0b, 0x27, 0x96, 0x00] },
    Prologo { endereco: 0x855b40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x8533d0, bytes: &[0x81, 0xec, 0x9c, 0x03, 0x00, 0x00] },
    Prologo { endereco: 0x5ba0a0, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5b91c0, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x98, 0x95, 0x00] },
    Prologo { endereco: 0x5bd600, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x9a, 0x95, 0x00] },
    Prologo { endereco: 0x5b9460, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x98, 0x95, 0x00] },
    Prologo { endereco: 0x5bd870, bytes: &[0x6a, 0xff, 0x68, 0x58, 0x9a, 0x95, 0x00] },
    Prologo { endereco: 0x642a70, bytes: &[0x6a, 0xff, 0x68, 0xde, 0xb9, 0x95, 0x00] },
    Prologo { endereco: 0x647230, bytes: &[0x6a, 0xff, 0x68, 0x6e, 0xba, 0x95, 0x00] },
    Prologo { endereco: 0x64be90, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0xbb, 0x95, 0x00] },
    Prologo { endereco: 0x650100, bytes: &[0x6a, 0xff, 0x68, 0xae, 0xbb, 0x95, 0x00] },
    Prologo { endereco: 0x5bb540, bytes: &[0xb8, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5bb550, bytes: &[0xb8, 0x01, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x85c260, bytes: &[0x6a, 0xff, 0x68, 0xd1, 0x29, 0x96, 0x00] },
    Prologo { endereco: 0x85c800, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] },
    Prologo { endereco: 0x85cba0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x5bfe30, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5be620, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bd920, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5b9510, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x64ad80, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x64ef30, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6409b0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x645af0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bb690, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x434ae0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x434b70, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x47, 0x95, 0x00] },
    Prologo { endereco: 0x434ed0, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02] },
    Prologo { endereco: 0x438230, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x4382c0, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0x49, 0x95, 0x00] },
    Prologo { endereco: 0x4384e0, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02] },
    Prologo { endereco: 0x43a870, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x43a900, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0x4a, 0x95, 0x00] },
    Prologo { endereco: 0x43ad20, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02] },
    Prologo { endereco: 0x63ec50, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63cf90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x65f920, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63c890, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63faf0, bytes: &[0x81, 0xec, 0x8c, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63dfe0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x65fa60, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x427700, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x427c20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x427ec0, bytes: &[0x56, 0x8b, 0xf1, 0x6a, 0x00] },
    Prologo { endereco: 0x4c31c0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c4930, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c5850, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5757a0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5770e0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x571980, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x573660, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84e450, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42fe10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42ffc0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x431090, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x431240, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x432370, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x432520, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x4336f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4338a0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x435e80, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x436030, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x436e90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x437040, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x4394d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x439680, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x43bd10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43bec0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x43cd40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43cef0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x853c90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x851260, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5bb160, bytes: &[0x6a, 0xff, 0x68, 0x81, 0x99, 0x95, 0x00] },
    Prologo { endereco: 0x42a950, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x42a9e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x42cdd0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x42ce60, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x42a640, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42ae90, bytes: &[0xa1, 0xfc, 0xad, 0xad, 0x00] },
    Prologo { endereco: 0x42cc10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6639c0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x663a50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x6636d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5f9970, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0xaf, 0x95, 0x00] },
    Prologo { endereco: 0x5faec0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5fb160, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x689c20, bytes: &[0x53, 0x8b, 0xd9, 0x57, 0x8b, 0x43, 0x1c] },
    Prologo { endereco: 0x48e3e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x48ea40, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x425c10, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c1960, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x57a3c0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x410910, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x40ae90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x40cf90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56ade0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5e0900, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4012e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x58b560, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x633300, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x92b770, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x632340, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c0f00, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x9025c0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6336c0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x633f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x57a500, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x410a50, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63c9d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x797230, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x40b620, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x40d4d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x43afa0, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x4a, 0x95, 0x00] },
    Prologo { endereco: 0x4351b0, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0x48, 0x95, 0x00] },
    Prologo { endereco: 0x438760, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x49, 0x95, 0x00] },
    Prologo { endereco: 0x663c20, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0xc1, 0x95, 0x00] },
    Prologo { endereco: 0x43a6a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x434910, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x438060, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x411f70, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x412010, bytes: &[0x6a, 0xff, 0x68, 0x1e, 0x3a, 0x95, 0x00] },
    Prologo { endereco: 0x4121c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x909400, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x9094c0, bytes: &[0x6a, 0xff, 0x68, 0x1e, 0x54, 0x96, 0x00] },
    Prologo { endereco: 0x9097d0, bytes: &[0x6a, 0xff, 0x68, 0x3e, 0x54, 0x96, 0x00] },
    Prologo { endereco: 0x427d10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x662950, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x665bc0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x6664e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x90b790, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x90a760, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x90a540, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c4410, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c37a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c3b10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4c5c90, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c4e60, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c51d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4c2c90, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c1de0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x4c2140, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x5e2120, bytes: &[0x6a, 0xff, 0x68, 0xe6, 0xa8, 0x95, 0x00] },
    Prologo { endereco: 0x5e11e0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5e16b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x908560, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x908fc0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x9091e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x908e00, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x909bb0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x909a40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42ee80, bytes: &[0x6a, 0xff, 0x68, 0xbe, 0x45, 0x95, 0x00] },
    Prologo { endereco: 0x42e430, bytes: &[0x6a, 0xff, 0x68, 0x4e, 0x45, 0x95, 0x00] },
    Prologo { endereco: 0x42e700, bytes: &[0x6a, 0xff, 0x68, 0x7c, 0x45, 0x95, 0x00] },
    Prologo { endereco: 0x42e980, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x42def0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42f0a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56b730, bytes: &[0x81, 0xec, 0x80, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x56bab0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x56bc50, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x56bce0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x56c010, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x522e00, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x78, 0x95, 0x00] },
    Prologo { endereco: 0x523300, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x86, 0xa7, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4c3d10, bytes: &[0x6a, 0xff, 0x68, 0xbe, 0x6a, 0x95, 0x00] },
    Prologo { endereco: 0x4c2330, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0x6a, 0x95, 0x00] },
    Prologo { endereco: 0x4c20b0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x4c2560, bytes: &[0x81, 0xec, 0x24, 0x04, 0x00, 0x00] },
    Prologo { endereco: 0x4c5140, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x5e1620, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x5e1cd0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5e2340, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x5e1530, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x42f2d0, bytes: &[0x6a, 0xff, 0x68, 0xe9, 0x45, 0x95, 0x00] },
    Prologo { endereco: 0x4304b0, bytes: &[0x6a, 0xff, 0x68, 0x59, 0x46, 0x95, 0x00] },
    Prologo { endereco: 0x4317c0, bytes: &[0x6a, 0xff, 0x68, 0xc9, 0x46, 0x95, 0x00] },
    Prologo { endereco: 0x432b50, bytes: &[0x6a, 0xff, 0x68, 0x39, 0x47, 0x95, 0x00] },
    Prologo { endereco: 0x433d50, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x47, 0x95, 0x00] },
    Prologo { endereco: 0x435340, bytes: &[0x6a, 0xff, 0x68, 0x39, 0x48, 0x95, 0x00] },
    Prologo { endereco: 0x4363f0, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x48, 0x95, 0x00] },
    Prologo { endereco: 0x437400, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x49, 0x95, 0x00] },
    Prologo { endereco: 0x4388f0, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x49, 0x95, 0x00] },
    Prologo { endereco: 0x439a40, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x4a, 0x95, 0x00] },
    Prologo { endereco: 0x43b130, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x4a, 0x95, 0x00] },
    Prologo { endereco: 0x43c280, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x4b, 0x95, 0x00] },
    Prologo { endereco: 0x5d6280, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] },
    Prologo { endereco: 0x797b40, bytes: &[0x6a, 0xff, 0x68, 0x16, 0xeb, 0x95, 0x00] },
    Prologo { endereco: 0x7978c0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x797e80, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x798220, bytes: &[0x81, 0xec, 0xfc, 0x03, 0x00, 0x00] },
    Prologo { endereco: 0x797df0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x798050, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x40c070, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x40c110, bytes: &[0x6a, 0xff, 0x68, 0xcc, 0x37, 0x95, 0x00] },
    Prologo { endereco: 0x40c3d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x4025a0, bytes: &[0x6a, 0xff, 0x68, 0x46, 0x35, 0x95, 0x00] },
    Prologo { endereco: 0x4022c0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x401ea0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x402090, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x40dfa0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x40e030, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x63f250, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63d5e0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x63f5b0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x63d950, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] },
    Prologo { endereco: 0x63f640, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0xb9, 0x95, 0x00] },
    Prologo { endereco: 0x63d9e0, bytes: &[0x6a, 0xff, 0x68, 0x6e, 0xb8, 0x95, 0x00] },
    Prologo { endereco: 0x63f870, bytes: &[0x6a, 0xff, 0x68, 0x2e, 0xb9, 0x95, 0x00] },
    Prologo { endereco: 0x63dc10, bytes: &[0x6a, 0xff, 0x68, 0x9c, 0xb8, 0x95, 0x00] },
    Prologo { endereco: 0x57e8f0, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0x88, 0x95, 0x00] },
    Prologo { endereco: 0x57d9d0, bytes: &[0x0f, 0xbe, 0x81, 0xf6, 0x00, 0x00, 0x00] },
    Prologo { endereco: 0x580b50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] },
    Prologo { endereco: 0x57e7e0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x902b90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x84d740, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
    Prologo { endereco: 0x584ad0, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x30] },
    Prologo { endereco: 0x5e93e0, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x28] },
    Prologo { endereco: 0x637750, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x1c] },
    Prologo { endereco: 0x63d770, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x3c] },
];
const N: usize = 2092;

/// Tamanho do objeto do motor (o jogo o aloca com new(0xf63b)): o bloco do lado 1 (+0xf5f2, 0x36
/// bytes) e os campos ate +0xf63a ficam alem de 0xf5f8.
const MOTOR: usize = 0xf63b;

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

// The rest of the engine, one file per source file of the game (stage 3 of the reorganisation, 07/11/2026).
// [pt] O resto do motor, um arquivo por arquivo de origem do jogo (etapa 3 da reorganização, 07/11/2026).
include!("motor/macros.rs");
include!("motor/mem.rs");
include!("motor/area.rs");
include!("motor/award/month_award.rs");
include!("motor/award/month_ratings.rs");
include!("motor/awol.rs");
include!("motor/cash.rs");
include!("motor/club_history.rs");
include!("motor/club_records.rs");
include!("motor/comp/comp.rs");
include!("motor/comp/comp_util.rs");
include!("motor/comp/conmebol_seeding.rs");
include!("motor/comp/cup.rs");
include!("motor/comp/cups/bra_champ_cup.rs");
include!("motor/comp/cups/bra_cup.rs");
include!("motor/comp/cups/cup_stage.rs");
include!("motor/comp/cups/eng_auto_cup.rs");
include!("motor/comp/cups/eng_cc_cup.rs");
include!("motor/comp/cups/fra_cup.rs");
include!("motor/comp/cups/fra_super.rs");
include!("motor/comp/cups/ita_cup.rs");
include!("motor/comp/cups/jap_j_cup.rs");
include!("motor/comp/cups/spa_cup.rs");
include!("motor/comp/cups/spa_super.rs");
include!("motor/comp/cups/usa_mls_all_stars.rs");
include!("motor/comp/eurocomp/asia_club_champ.rs");
include!("motor/comp/eurocomp/asia_cup_winner.rs");
include!("motor/comp/eurocomp/con_champ.rs");
include!("motor/comp/eurocomp/con_merc_cup.rs");
include!("motor/comp/eurocomp/conmebol_liber.rs");
include!("motor/comp/eurocomp/conmebol_merc.rs");
include!("motor/comp/eurocomp/eur_super_cup.rs");
include!("motor/comp/eurocomp/european_cup.rs");
include!("motor/comp/eurocomp/intertoto_cup.rs");
include!("motor/comp/eurocomp/oceania_club_champ.rs");
include!("motor/comp/eurocomp/uefa_cup.rs");
include!("motor/comp/fifa_rankings.rs");
include!("motor/comp/fix_man.rs");
include!("motor/comp/friendly.rs");
include!("motor/comp/host_country.rs");
include!("motor/comp/intercomp/african_nations.rs");
include!("motor/comp/intercomp/euro_champ_qual.rs");
include!("motor/comp/intercomp/goldcup.rs");
include!("motor/comp/league.rs");
include!("motor/comp/leagues/aus_nsl.rs");
include!("motor/comp/leagues/bra_nat_first.rs");
include!("motor/comp/leagues/bra_nat_third.rs");
include!("motor/comp/leagues/bra_reg_bahia.rs");
include!("motor/comp/leagues/bra_reg_central.rs");
include!("motor/comp/leagues/bra_reg_gaucho.rs");
include!("motor/comp/leagues/bra_reg_goias.rs");
include!("motor/comp/leagues/bra_reg_minas_gerais.rs");
include!("motor/comp/leagues/bra_reg_northeast.rs");
include!("motor/comp/leagues/bra_reg_parana.rs");
include!("motor/comp/leagues/bra_reg_pern.rs");
include!("motor/comp/leagues/bra_reg_rio.rs");
include!("motor/comp/leagues/eng_first.rs");
include!("motor/comp/leagues/eng_prm.rs");
include!("motor/comp/leagues/eng_second.rs");
include!("motor/comp/leagues/fin_prm.rs");
include!("motor/comp/leagues/fra_cfa.rs");
include!("motor/comp/leagues/fra_first.rs");
include!("motor/comp/leagues/fra_second.rs");
include!("motor/comp/leagues/fra_third.rs");
include!("motor/comp/leagues/ita_ser_a.rs");
include!("motor/comp/leagues/ita_ser_c1a.rs");
include!("motor/comp/leagues/jap_j1.rs");
include!("motor/comp/leagues/spa_first.rs");
include!("motor/comp/leagues/spa_lower.rs");
include!("motor/comp/leagues/spa_second.rs");
include!("motor/comp/leagues/usa_mls.rs");
include!("motor/comp/squad_manager.rs");
include!("motor/comp/stadium.rs");
include!("motor/comp/uefa_seeding.rs");
include!("motor/comp/worldcup/wc_europe_league.rs");
include!("motor/comp/worldcup/world_cup.rs");
include!("motor/contract_manager.rs");
include!("motor/database.rs");
include!("motor/db_files.rs");
include!("motor/discipline.rs");
include!("motor/fine.rs");
include!("motor/fog_of_war.rs");
include!("motor/formation.rs");
include!("motor/game.rs");
include!("motor/human_manager.rs");
include!("motor/index.rs");
include!("motor/injury.rs");
include!("motor/key_nation.rs");
include!("motor/main.rs");
include!("motor/manager_manager.rs");
include!("motor/match_day.rs");
include!("motor/match_eng.rs");
include!("motor/match_events.rs");
include!("motor/match_man.rs");
include!("motor/match_official.rs");
include!("motor/match_pl.rs");
include!("motor/match_screens.rs");
include!("motor/media.rs");
include!("motor/national_teams.rs");
include!("motor/news.rs");
include!("motor/officials_manager.rs");
include!("motor/player_regen.rs");
include!("motor/player_search.rs");
include!("motor/player_stats.rs");
include!("motor/printouts.rs");
include!("motor/record_utils.rs");
include!("motor/ruling_body/ruling_body.rs");
include!("motor/scout_manager.rs");
include!("motor/scrman.rs");
include!("motor/setup.rs");
include!("motor/shared.rs");
include!("motor/shortlist_manager.rs");
include!("motor/staff_contracts.rs");
include!("motor/staff_records.rs");
include!("motor/tactics.rs");
include!("motor/training_manager.rs");
include!("motor/transfer/argentina_rules.rs");
include!("motor/transfer/brazil_rules.rs");
include!("motor/transfer/england_rules.rs");
include!("motor/transfer/greece_rules.rs");
include!("motor/transfer/holland_rules.rs");
include!("motor/transfer/norway_rules.rs");
include!("motor/transfer/scotland_rules.rs");
include!("motor/transfer/spain_rules.rs");
include!("motor/transfer/turkey_rules.rs");
include!("motor/transfer_manager.rs");
include!("motor/weather.rs");
include!("motor/zipdir.rs");

// ---------- centesima sexagesima primeira leva: lista de observacao, codigo da posicao e criterio da comissao ----------

/// { 0x844300 } (stdcall, ret 8, shortlist_manager.cpp): o jogador `j` (ficha em [j+0x61]) serve
/// para a vaga `vaga` 0..15 da lista de observação: 0..14 pedem nota 15 nas posições da vaga
/// (setor e lado: +0xf..+0x16 e +0x17..+0x19); 15 pede ficha de contrato ([j+0x69]), id abaixo de
/// [0xae2364]−16 e, com treinador em [j+0x39] de função 11, 13 ou 15 em +0x3d, a 0x600a50 em zero.
/// Sem ficha (fora a vaga 15): 0. Vaga fora de 0..15: a original (caixa de erro). Devolve só `al`.
unsafe extern "stdcall" fn serve_para_a_vaga(j: usize, vaga: i8) -> u8 {
    let ficha = core::ptr::read_unaligned((j + 0x61) as *const usize);
    if ficha == 0 && vaga != 15 {
        return 0;
    }
    if !(0..=15).contains(&vaga) {
        return orig::<unsafe extern "stdcall" fn(usize, i8) -> u8>(355)(j, vaga);
    }
    let n = |o: usize| *((ficha + o) as *const i8) >= 15;
    // (setor, lado): 0 = so o setor
    let (setor, lado): (usize, usize) = match vaga {
        0 => (0xf, 0),
        1 => (0x10, 0x19),
        2 => (0x11, 0x18),
        3 => (0x11, 0x17),
        4 => (0x11, 0x19),
        5 => (0x16, 0x18),
        6 => (0x12, 0x19),
        7 => (0x16, 0x17),
        8 => (0x13, 0x18),
        9 => (0x13, 0x19),
        10 => (0x13, 0x17),
        11 => (0x14, 0x18),
        12 => (0x14, 0x19),
        13 => (0x14, 0x17),
        14 => (0x15, 0),
        _ => {
            let tecnico = core::ptr::read_unaligned((j + 0x39) as *const usize);
            if tecnico != 0 && matches!(*((j + 0x3d) as *const u8), 0xb | 0xf | 0xd) {
                let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
                if f(tecnico, 0, 1, 0, 0) != 0 {
                    return 0;
                }
            }
            let contrato = core::ptr::read_unaligned((j + 0x69) as *const usize);
            return (contrato != 0 && *(j as *const i32) < (*(0xae2364 as *const i32)).wrapping_sub(0x10)) as u8;
        }
    };
    (n(setor) && (lado == 0 || n(lado))) as u8
}

/// { 0x83bd00 } (stdcall, ret 4, shortlist_manager.cpp): o código da posição (0..13, 0x11..0x15)
/// da máscara `m` de um bit de setor com um de lado (0x80, 0x100, 0x200, 0x400, 0x800); fora das
/// combinações conhecidas, 0xe. Devolve só `al`.
unsafe extern "stdcall" fn codigo_da_posicao(m: i16) -> u8 {
    match m {
        1 => 0,
        2 => 1,
        4 => 0x11,
        8 => 0x13,
        0x10 => 0x12,
        0x20 => 0x15,
        0x40 => 0x14,
        0x84 => 2,
        0x88 => 5,
        0x90 => 8,
        0xa0 => 0xb,
        0x104 | 0x204 | 0x404 => 4,
        0x108 | 0x208 | 0x408 => 6,
        0x110 | 0x210 | 0x410 => 9,
        0x120 | 0x220 | 0x420 => 0xc,
        0x804 => 3,
        0x808 => 7,
        0x810 => 0xa,
        0x820 => 0xd,
        _ => 0xe,
    }
}

/// { 0x866f60 } (thiscall, staff_contracts.cpp): o critério da comissão técnica `t` (pessoa [t] em
/// [0xae23bc], 0x6e bytes; clube [t+4] em [0xae23b4], 0x245 bytes; funções em [t+0x45]): 0 se a
/// pessoa é o técnico ou o auxiliar do clube (+0xcf, +0xd3) ou não tem função; senão uma nota em 8
/// bits (id mod 7 + +0x59 + +0x5a + +0x5b, descontos pelas notas baixas e pela idade, +0x44 em
/// faixas, a reputação do clube, e menos o número de funções acima de uma) decide pelas funções e
/// pelas notas se devolve 1.
unsafe extern "thiscall" fn criterio_da_comissao(t: usize) -> u32 {
    let p = (*(0xae23bc as *const usize)).wrapping_add((*(t as *const i32) as usize).wrapping_mul(0x6e));
    let clube = (*(0xae23b4 as *const usize)).wrapping_add((*((t + 4) as *const i32) as usize).wrapping_mul(0x245));
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let tecnico = u(clube + 0xcf);
    if tecnico != 0 && p == tecnico {
        return 0;
    }
    let auxiliar = u(clube + 0xd3);
    if auxiliar != 0 && p == auxiliar {
        return 0;
    }
    let funcoes = u(t + 0x45) as u32;
    if funcoes == 0 {
        return 0;
    }
    let b = |o: usize| *((p + o) as *const i8);
    let (b59, b5a, b5b, b5d) = (b(0x59), b(0x5a), b(0x5b), b(0x5d));
    let b44 = *((t + 0x44) as *const i8);
    let mut nota = ((*(p as *const i32) % 7) as u8).wrapping_add(b59 as u8).wrapping_add(b5a as u8).wrapping_add(b5b as u8);
    if b5a == 1 {
        nota = nota.wrapping_sub(2);
    }
    if b59 == 1 {
        nota = nota.wrapping_sub(3);
    } else if b59 < 6 {
        nota = nota.wrapping_sub(1);
    }
    if b5b == 1 {
        nota = nota.wrapping_sub(3);
    } else if b5b < 6 {
        nota = nota.wrapping_sub(1);
    }
    if b5d == 1 {
        nota = nota.wrapping_sub(2);
    }
    if b44 > 0x4b {
        nota = nota.wrapping_add(10);
    } else if b44 > 0x32 {
        nota = nota.wrapping_add(5);
    } else if b44 > 0 {
        nota = nota.wrapping_add(2);
    }
    let ficha = u(p + 0x61);
    if ficha != 0 {
        let rep = core::ptr::read_unaligned((ficha + 0xb) as *const i16);
        if rep > 0x1c52 {
            nota = nota.wrapping_sub(5);
        } else if rep > 0x1676 {
            nota = nota.wrapping_sub(3);
        }
    }
    let tecnica = *((t + 0x40) as *const u32) & 8 != 0;
    let m1 = funcoes & 0x1000002;
    let mut k = 0u8;
    for bits in [0x20000u32, 0x40000, 0x1000002, 0x200600, 0x2840, 0x14] {
        if funcoes & bits != 0 {
            k += 1;
        }
    }
    if tecnica || funcoes & 0x4582208 != 0 {
        k += 1;
    }
    if k > 1 {
        nota = nota.wrapping_sub(3);
        if b5b < 6 {
            nota = nota.wrapping_sub(k);
        }
        if b5d < 6 {
            nota = nota.wrapping_sub(k);
        }
        if b59 == 1 {
            nota = nota.wrapping_add((k as i8).wrapping_mul(-2) as u8);
        } else if b59 < 6 {
            nota = nota.wrapping_sub(k);
        }
        if b5a < 6 {
            nota = nota.wrapping_sub(k);
        }
    }
    let nota = nota as i8;
    let sim = (b59 < 15 && *((t + 0x4e) as *const u8) & 1 != 0)
        || (m1 != 0 && b59 < 10 && nota < 25)
        || (funcoes & 0x20020 != 0 && nota < 25)
        || (funcoes & 0x200 != 0 && nota < 18)
        || (funcoes & 4 != 0 && b(0x57) > 15 && b5b < 10)
        || (funcoes & 0x2808 != 0 && b59 < 10 && b5a < 6 && nota < 30)
        || ((funcoes & 0x82200 != 0 || tecnica) && b5b < 10 && nota < 20)
        || funcoes & 0x111 != 0
        || (b44 as i32) < -0x46 - b59 as i32;
    sim as u32
}

unsafe extern "stdcall" fn comparar_serve_para_a_vaga(j: usize, vaga: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, i8) -> u8>(355)(j, vaga);
    let n = serve_para_a_vaga(j, vaga);
    PLACAR.registrar(r == n, || format!("serve_para_a_vaga({:#x}, {}) original={} rust={}", j, vaga, r, n));
    r
}

unsafe extern "stdcall" fn comparar_codigo_da_posicao(m: i16) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i16) -> u8>(356)(m);
    let n = codigo_da_posicao(m);
    PLACAR.registrar(r == n, || format!("codigo_da_posicao({:#x}) original={:#x} rust={:#x}", m, r, n));
    r
}

comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);

// ---------- centesima sexagesima segunda leva: as vagas da tabela tatica (tactics.cpp) ----------

/// A linha `linha` da tabela tática do objeto `t` ([t+0x601], 0x91 bytes por linha: palavra da
/// formação e 20 vagas de 4 bytes a partir de +2).
unsafe fn linha_da_tabela(t: usize, linha: i32) -> usize {
    core::ptr::read_unaligned((t + 0x601) as *const usize).wrapping_add((linha as usize).wrapping_mul(0x91))
}

/// { 0x896a90 } (thiscall, ret 0x10, tactics.cpp): grava `*pessoa` na vaga `vaga` 0..19 da linha do
/// `clube` (0x8982b0 com `humano`; sem linha, 0) e apaga, entre as 16 instruções pendentes
/// (+0x30, 0x5c bytes: clube, humano, ativa), as ativas desse clube e `humano`, descontando o
/// contador +0x5f0. Devolve 1. `clube` ou `pessoa` nulos, vaga fora: a original (caixa de erro).
unsafe extern "thiscall" fn gravar_vaga_tatica(t: usize, clube: usize, vaga: i8, pessoa: *const u32, humano: u32) -> u32 {
    if clube == 0 || !(0..=19).contains(&vaga) || pessoa.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(358)(t, clube, vaga, pessoa, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned((linha_da_tabela(t, l) + 2 + 4 * vaga as usize) as *mut u32, *pessoa);
    let pendentes = (t + 0x5f0) as *mut u8;
    if *pendentes != 0 {
        for k in 0..16 {
            let e = t + 0x30 + 0x5c * k;
            let ativa = (e + 8) as *mut u32;
            if *ativa != 0 && *(e as *const usize) == clube && *((e + 4) as *const u32) == humano {
                *ativa = 0;
                *pendentes = (*pendentes).wrapping_sub(1);
            }
        }
    }
    1
}

/// { 0x8968f0 } (thiscall, ret 0xc, tactics.cpp): a pessoa (em [0xae23bc], 0x6e bytes) da vaga `vaga`
/// 0..19 da linha do `clube`; 0 sem linha ou vaga vazia (negativa). Vaga com pessoa sem ficha
/// (+0x61), ou, numa seleção (0x53b900), pessoa sem nação (+0x24) ou de outra nação (+0x53): a vaga
/// vira −1 e devolve 0. `clube` nulo ou vaga fora: a original (caixa de erro).
unsafe extern "thiscall" fn pessoa_da_vaga_tatica(t: usize, clube: usize, vaga: i8, humano: u32) -> usize {
    if clube == 0 || !(0..=19).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(359)(t, clube, vaga, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    let casa = (linha_da_tabela(t, l) + 2 + 4 * vaga as usize) as *mut i32;
    let id = core::ptr::read_unaligned(casa);
    if id <= -1 {
        return 0;
    }
    let pessoas = *(0xae23bc as *const usize);
    let p = pessoas.wrapping_add((id as usize).wrapping_mul(0x6e));
    if core::ptr::read_unaligned((p + 0x61) as *const usize) == 0 {
        core::ptr::write_unaligned(casa, -1);
        return 0;
    }
    if selecao(clube) != 0 {
        let nacao = core::ptr::read_unaligned((p + 0x24) as *const usize);
        if nacao == 0 || core::ptr::read_unaligned((nacao + 0x53) as *const u32) != core::ptr::read_unaligned((clube + 0x53) as *const u32) {
            core::ptr::write_unaligned(casa, -1);
            return 0;
        }
    }
    p
}

/// { 0x8967d0 } (thiscall, ret 0xc, tactics.cpp): grava a formação `formacao` (palavra) na linha do
/// `clube`; 0 sem linha, senão 1. `clube` nulo ou formação fora de 0..[+0x5fd]+[+0x5f5]−1: a
/// original (caixa de erro).
unsafe extern "thiscall" fn gravar_formacao_tatica(t: usize, clube: usize, formacao: i16, humano: u32) -> u32 {
    let limite = core::ptr::read_unaligned((t + 0x5fd) as *const i32)
        .wrapping_add(core::ptr::read_unaligned((t + 0x5f5) as *const i32))
        .wrapping_sub(1);
    if clube == 0 || formacao < 0 || formacao as i32 > limite {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(360)(t, clube, formacao, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned(linha_da_tabela(t, l) as *mut i16, formacao);
    1
}

/// As regiões que as três podem tocar: a linha do clube na tabela (se houver) e as instruções
/// pendentes com o contador (+0x30..+0x5f1).
unsafe fn regioes_da_tabela(t: usize, clube: usize, humano: u32) -> Vec<(usize, usize)> {
    let mut r = vec![(t + 0x30, 0x5c1)];
    if clube != 0 {
        let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
        let l = linha_tatica(clube, humano);
        if l != -1 {
            r.push((linha_da_tabela(t, l), 0x91));
        }
    }
    r
}

unsafe extern "thiscall" fn comparar_gravar_vaga_tatica(t: usize, clube: usize, vaga: i8, pessoa: *const u32, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(358);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, vaga, pessoa, humano), || gravar_vaga_tatica(t, clube, vaga, pessoa, humano));
    PLACAR.registrar(igual && r == v, || format!("gravar_vaga_tatica({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, vaga, humano, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_pessoa_da_vaga_tatica(t: usize, clube: usize, vaga: i8, humano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(359);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, vaga, humano), || pessoa_da_vaga_tatica(t, clube, vaga, humano));
    PLACAR.registrar(igual && r == v, || format!("pessoa_da_vaga_tatica({:#x}, {:#x}, {}, {}) original={:#x} {:x?} rust={:#x} {:x?}", t, clube, vaga, humano, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_gravar_formacao_tatica(t: usize, clube: usize, formacao: i16, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(360);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, formacao, humano), || gravar_formacao_tatica(t, clube, formacao, humano));
    PLACAR.registrar(igual && r == v, || format!("gravar_formacao_tatica({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, formacao, humano, r, mo, v, mr));
    r
}
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
    if gravando_no_topo() {
        // unidade da fita da execucao do passe (a unica que o chama); por dentro, a propria comparacao
        let xy = x0 as u8 as u32 | (y0 as u8 as u32) << 8 | (x1 as u8 as u32) << 16 | (y1 as u8 as u32) << 24;
        return chamar_gravando(0x6ccb60, this, [xy, codigo as u8 as u32, a6, a7, 0], || comparar_tracar(this, x0, y0, x1, y1, codigo, a6, a7) as u32) as u8;
    }
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
    if gravando_pos() {
        return chamar_gravando(0x6b7cf0, 0, [jogo as u32, clube, 0, 0, 0], || orig::<unsafe extern "C" fn(usize, u32) -> u8>(139)(jogo, clube) as u32) as u8;
    }
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
/// Na comparacao da 0x70dfd0 a fita guarda o setimo argumento da narracao no alto do codigo.
static GRAVAR_A7: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
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
        let codigo = if GRAVAR_A7.load(Relaxed) { a1 & 0xffff | (a7 & 0xff) << 16 } else { a1 };
        chamar_gravando(0x6d6f70, this, [codigo, a2, a3, a4, a5], || {
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
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if gravando_reinicio() {
        let o = orig::<unsafe extern "thiscall" fn(usize)>(145);
        chamar_gravando(0x6c51e0, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let o = orig::<unsafe extern "thiscall" fn(usize)>(145);
    let mut reg = vec![(this, MOTOR)];
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
    if gravando_pos() {
        return chamar_gravando(0x53b900, 0, [clube as u32, 0, 0, 0, 0], || orig::<unsafe extern "C" fn(usize) -> u32>(155)(clube));
    }
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
unsafe extern "thiscall" fn comparar_contrato_com_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(198);
    if gravando_pos() {
        return chamar_gravando(0x4cf5f0, this, [pessoa as u32, clube as u32, 0, 0, 0], || o(this, pessoa, clube) as u32) as usize;
    }
    let r = o(this, pessoa, clube);
    let n = contrato_com_clube(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("contrato_com_clube({:#x}, {:?}, {:?}) original={:#x} rust={:#x}", this, pessoa, clube, r, n));
    r
}
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
    // Sonda (so na comparacao): a 0x6dbc50, no modo 2, nunca escreve o curto da 1a vaga extra e passa
    // o lixo da pilha a escala (retorno 0x6dd383). Conta quantas vezes ele nao e zero e se zera-lo muda
    // a escala (a original duas vezes: com o lixo e com 0).
    if crate::desvios::RETORNO == 0x6dd383 && a5 as u8 != 0 && a15 != 0 {
        use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
        static CHAMADAS: AtomicU32 = AtomicU32::new(0);
        static NAO_ZERO: AtomicU32 = AtomicU32::new(0);
        static MUDA: AtomicU32 = AtomicU32::new(0);
        let campo = (a15 + 2 * (a4 as u8 as usize)) as *mut u16;
        let lixo = *campo;
        let k = CHAMADAS.fetch_add(1, Relaxed) + 1;
        if lixo != 0 {
            NAO_ZERO.fetch_add(1, Relaxed);
            let (r1, r2, igual, _, _) = crate::comparacao::lado_a_lado(&[(a6, 4 * n)], chamar, || {
                *campo = 0;
                let r = chamar();
                *campo = lixo;
                r
            });
            if !igual || r1 != r2 {
                MUDA.fetch_add(1, Relaxed);
            }
        }
        if lixo != 0 || k % 500 == 0 {
            crate::registro::linha(&format!(
                "sonda_lixo: valor {:#06x}; {} escalas da 0x6dbc50, {} com lixo, {} mudam se zerado",
                lixo, k, NAO_ZERO.load(Relaxed), MUDA.load(Relaxed)
            ));
        }
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
    reg.extend([(m, MOTOR), (*((c + 0x2b) as *const usize), 0x18e3), (0xb67a34, 4), (0xae2470, 8), (0xae2418, 1), (0xae24c0, 8), (0xae249e, 1), (0xadae10, 4)]);
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
    reg.extend([(m, MOTOR), (*((c + 0x2b) as *const usize), 0x18e3), (0xb67a34, 4)]);
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
/// As linhas do clube da comissao no banco de taticas (0x8982b0 por tecnico humano ou nao).
unsafe fn linhas_do_clube(clube: usize) -> Vec<(usize, usize)> {
    let taticas = *(0xde1d08 as *const usize);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let mut v = Vec::new();
    for humano in 0..2u32 {
        let l = if clube == 0 || taticas == 0 { -1 } else { linha(clube, humano) };
        if l != -1 {
            v.push(((*((taticas + 0x601) as *const usize)).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    v
}
/// O primeiro byte diferente entre as memorias (as regioes do motor sao grandes).
fn primeira_diferenca(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> String {
    mo.0.iter()
        .zip(mr.0.iter())
        .find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        })
        .unwrap_or_default()
}
unsafe extern "thiscall" fn comparar_colocar_na_vaga(c: usize, pos: u32, pessoa: usize, extra: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8>(282);
    if gravando_no_topo() {
        // dentro dos titulares (ou da comissao): unidade da fita
        return chamar_gravando(0x6e0d00, c, [pos, pessoa as u32, extra, 0, 0], || o(c, pos, pessoa, extra) as u32) as u8;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(*((c + 0x12) as *const usize), MOTOR), (0xb67a34, 4)]);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(c, pos, pessoa, extra), |chamar| {
        colocar_na_vaga_com(c, pos, pessoa, extra, &mut |rotina, this, args| chamar(rotina, this, args, [u32::MAX, u32::MAX, 0xff, 0xff, 0xff]))
    });
    PLACAR.registrar(igual && sequencia && r == rr, || format!("colocar_na_vaga({}, {:#x}) original={} rust={} {} chamadas: {}", pos as u8 as i8, pessoa, r, rr, k, detalhe_fita()));
    r
}
unsafe extern "C" fn comparar_ordem_por_tabela(a: *const usize, b: *const usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(283)(a, b);
    let n = ordem_por_tabela(a, b);
    PLACAR.registrar(r == n, || format!("ordem_por_tabela({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}
unsafe extern "C" fn comparar_ordem_por_condicao(a: *const usize, b: *const usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(284)(a, b);
    let n = ordem_por_condicao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_por_condicao({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}
unsafe extern "thiscall" fn comparar_titulares(c: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u8>(285);
    if gravando_no_topo() {
        // dentro da comissao do inicio da partida: unidade da fita
        return chamar_gravando(0x6dbb80, c, [0, 0, 0, 0, 0], || o(c) as u32) as u8;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(*((c + 0x12) as *const usize), MOTOR), (0xb67a34, 4)]);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(c), |chamar| {
        titulares_com(c, &mut |rotina, this, args| chamar(rotina, this, args, [0xff, u32::MAX, 0xff, 0, 0]))
    });
    PLACAR.registrar(igual && sequencia && r == rr, || format!("titulares(lado {}) original={} rust={} {} chamadas: {}", byte(c, 0x18), r, rr, k, detalhe_fita()));
    r
}
unsafe extern "thiscall" fn comparar_guardar_tatica(c: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(286);
    let mut reg = linhas_do_clube(*((c + 0x2f) as *const usize));
    reg.push((0xb67a34, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(c), || guardar_tatica(c));
    PLACAR.registrar(igual, || format!("guardar_tatica(humano {}) {}", byte(c, 0x19), primeira_diferenca(&mo, &mr)));
}
unsafe extern "thiscall" fn comparar_aplicar_trocas(m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(287);
    if gravando_no_topo() {
        return o(m);
    }
    // o motor inteiro (jogadores e comissoes), o sorteio, o erro, 0xae1080 e a linha da tatica dos dois
    // clubes (o capitao em +0x52, que a substituicao troca)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        aplicar_trocas_com(m, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("aplicar_trocas() {} chamadas: {}", k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_punicoes(m: usize, primeiro: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(288);
    if gravando_no_topo() {
        return o(m, primeiro);
    }
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    GRAVAR_716.store(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, primeiro), |chamar| {
        punicoes_com(m, primeiro, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_716.store(false, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("punicoes() {} chamadas: {}", k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_penaltis(m: usize, modo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(289);
    if gravando_no_topo() {
        return o(m, modo);
    }
    // o motor inteiro, o sorteio, o erro, o narrador (+0x8f7a aponta para fora) e o placar da partida
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let jogo = *((m + 0x4792) as *const usize);
    reg.extend([(m, MOTOR), (0xb67a34, 4), (*((m + 0x8f7a) as *const usize), 1), (jogo + 0x49, 2)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(m, modo), |chamar| {
        penaltis_com(m, modo, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                0x6f46b0 => [0xff, 0xff, 0, 0, 0],
                0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("penaltis(estado {}) {} chamadas: {}", byte(m, 0x8eb3), k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_aplicar_falta(m: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u8>(290);
    if gravando_no_topo() {
        return o(m);
    }
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let tipo = byte(m, 0x475a);
    GRAVAR_716.store(true, std::sync::atomic::Ordering::Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(m), |chamar| {
        aplicar_falta_com(m, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_716.store(false, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("aplicar_falta(tipo {}) original={} rust={} {} chamadas: {}", tipo, r, rr, k, detalhe_fita()));
    r
}
unsafe extern "thiscall" fn comparar_dividida(j: usize, forte: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(291);
    if gravando_no_topo() {
        return o(j, forte);
    }
    let m = *((j + 0x19e) as *const usize);
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, forte), |chamar| {
        dividida_com(j, forte as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
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
    GRAVAR_716.store(gravar, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("dividida(forte {}) {} chamadas: {}", forte as u8, k, detalhe_fita()));
}
unsafe extern "thiscall" fn comparar_disputa(m: usize, x: u32, y: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(292);
    if gravando_no_topo() {
        return o(m, x, y);
    }
    VANTAGEM_SEM_RESPOSTA.store(false, std::sync::atomic::Ordering::Relaxed);
    // o motor inteiro, o sorteio, o erro, 0xae1080 e a linha da tatica dos dois clubes (a expulsao
    // pode trocar o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || o(m, x, y), |chamar| {
        disputa_com(m, x as i8, y as i8, &mut |rotina, this, args| {
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
    let certo = igual && sequencia && r == rr;
    if !certo && VANTAGEM_SEM_RESPOSTA.swap(false, std::sync::atomic::Ordering::Relaxed) {
        // a original leu lixo da pilha como vantagem (ver `disputa_com`): conta a parte
        let n = LIXO_DISPUTA.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if n <= 20 || n.is_power_of_two() {
            linha(&format!("motor: LIXO_DE_PILHA disputa({}, {}) vantagem sem resposta do juiz ({} ate agora): {}", x as i8, y as i8, n, detalhe_fita()));
        }
        PLACAR.registrar(true, String::new);
        return r;
    }
    PLACAR.registrar(certo, || format!("disputa({}, {}) original={:#x} rust={:#x} {} chamadas: {}", x as i8, y as i8, r, rr, k, detalhe_fita()));
    r
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
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
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
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
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
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
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
    if gravando_fechamento() {
        chamar_gravando(0x6d26b0, jm, [pessoa as u32, clube as u32, rep as u32, nota.to_bits(), media_b & 0xff | (forte & 0xff) << 8], || {
            o(jm, pessoa, clube, rep, comp, nota, media_b, forte);
            0
        });
        return;
    }
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
            chamar(0x68d0d0, lista_gravada(l, 0x1be), [p as u32, 0x1be, 0, 0, 0], [u32::MAX; 5]);
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
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if gravando_reinicio() {
        let o = orig::<unsafe extern "thiscall" fn(usize)>(255);
        chamar_gravando(0x6c4820, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let o = orig::<unsafe extern "thiscall" fn(usize)>(255);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x1a0, 0x30), (this + 0x215e, 0x8f00 - 0x215e), (this + 0xf56e, 0x90)]);
    let lado_b = byte(this, 0x8eae);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| arrumar_bola_parada(this)));
    PLACAR.registrar(igual, || format!("arrumar_bola_parada(lado {})", lado_b));
}
unsafe extern "thiscall" fn comparar_arrumar_saida(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if gravando_reinicio() {
        let o = orig::<unsafe extern "thiscall" fn(usize)>(254);
        chamar_gravando(0x6c4ec0, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
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
    let mut regioes = vec![(m, MOTOR)];
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
    if gravando_pos() {
        let o = orig::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(180);
        return chamar_gravando(0x600a50, 0, [clube as u32, a, b as u32, pessoa as u32, c], || o(clube, a, b, pessoa, c));
    }
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
    let reg = [(this, MOTOR), (0xb67a34, 4)];
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

/// Rotinas portadas cujo desvio fica desligado em TODOS os modos (a original roda na pilha do jogo,
/// sem passar pela pilha particular): o desvio, mesmo de passagem, muda o resíduo que ela deixa na
/// pilha do jogo e uma original lida depois decide com ele. Voltam quando a cópia reproduzir esse resíduo.
/// 0x7c6bf0 (212a, desligada em 22/10/2026): o bsearch dela deixa na pilha o ponteiro que vira a palavra
/// +0x38 da ficha do jogo simulado da 0x84be20 (regras de prorrogação e pênaltis da simulação rápida
/// 0x6beeb0). Zerar a 0x84be20 (como na 212a) dava empates sem desempate, caminho que o jogo de verdade
/// quase nunca percorre, e separava Rust e original na carreira de 8 ligas.
/// 0x524d30 (219a) ficou desligada até a zeragem da save_database 0x52dad0 (opção A, 23/10/2026).
/// 0x827110 (220a, desligada em 23/10/2026): a cópia roda na pilha particular e a create_game 0x828110 lê
/// resíduo diferente; o club.dat sai diferente (prova do save).
/// 0x541f90 (225a, desligada em 24/10/2026): a cópia confere com a desmontagem (vetor, eax = 1, nada
/// em +0x244 nem em global), mas a original pelo desvio de passagem (SEM_ROTINAS) também leva a carreira
/// de b2854287 a 82fe4ecf: a 0x540a50 que ela chama deixa resíduo na pilha particular em vez da do jogo.
/// 0x547770 (233a, desligada em 25/10/2026): club_is_in_season; com o desvio (mesmo de passagem, no modo original)
/// a carreira até 2002-06-01 vai de b2854287 a 79d4e53e (bissecção por desvio); o jogo real comparando não muda.
/// 0x689ad0 (241a, desligada em 26/10/2026): League::check_complete; com o desvio (mesmo só de passagem) a carreira até
/// 2002-06-01 vai de b2854287 a 41532e5b (bissecção por desvio e sonda de passagem); chamada 26 mil vezes.
/// 0x6827d0 e 0x4b02e0 (241a, desligadas em 27/10/2026; bissecções pela carreira e pelo jogo comparando):
/// ordenar_tabela_da_liga 0x6827d0: com o desvio (também no modo original, só de passagem) a carreira vai de b2854287
/// para 5effff2d; tipo_de_confronto_na_liga 0x4b02e0: com o desvio o jogo comparando vai de 963cea28 para 1d9532b3
/// (0 divergências; igual até 2001-10-01; inverter a ordem do comparador não resolveu). Nenhuma das duas lê variável
/// sem valor (a 0x4a4850 que a 0x4b02e0 chama monta a chave do bsearch e só compara o id): é o resíduo de pilha que
/// uma original lida depois usa. Com o desvio das duas ligado, a carreira volta a b2854287 por coincidência.
/// 0x62c510 (243a, desligada em 27/10/2026): Injury::get_match_condition; com o desvio (também no modo original, só
/// de passagem) a carreira até 2002-06-01 vai de b2854287 para ddda6c3f (bissecção por desvio, confirmada); o jogo
/// real comparando não muda. A cópia não lê variável sem valor: a 0x62c680 que ela chama (atualiza o físico do dia,
/// com sorteio) deixa o resíduo na pilha particular em vez da do jogo.
/// 0x7a8510 e 0x74ae60 (247a, desligadas em 29/10/2026): PLAYER_REGEN::add_retirement e Media::post_financial_trouble_news;
/// com os 21 desvios da 247a (também no modo original, só de passagem) a carreira até 2002-06-01 ia de be54a138 para
/// 2523505a, Rust = original, e o jogo comparando de 2f65fd6b para 0c722156 (0 divergências). Busca uma a uma pela
/// carreira original (prefixos de desvios ligados): só estas duas mudam a carreira; com elas desligadas volta a be54a138.
/// 0x51c800, 0x51f890, 0x5ce060, 0x5d6aa0, 0x5d7190, 0x5d6d10 (255a, desligadas em 31/10/2026): montar_rodada_da_copa,
/// atualizar_copa_no_dia, e quatro do dia do jogo (cps_5, cps_6); com os 39 desvios da 255a a carreira até 2002-06-01
/// ia de 7f4ecded para cba167d8 na original e 1e070297 no Rust (o jogo comparando: 0 divergências). Busca pela carreira
/// original (prefixos de desvios desligados, `copias\desligar255.txt`): só com as seis desligadas volta a 7f4ecded,
/// nos dois modos.
/// Desde 31/10/2026 (Lucas) nenhuma: a carreira idêntica à original deixou de barrar a leva (é só registrada); vale a
/// comparação rotina a rotina. As de antes (e o motivo de cada uma) estão acima e no histórico do git.
const DESLIGADAS: &[usize] = &[];

/// Rotinas portadas que, fora da comparacao, rodam a original (pelo desvio de passagem, como no modo
/// original): alguma rotina original chamada abaixo delas le lixo da pilha, e o lixo que a copia Rust
/// deixa e outro. Na comparacao a copia segue conferida chamada a chamada.
/// 0x6ccb60 `tracar_lance` (15/10/2026): com a pasta do jogo atual (o save e o hall da fama do Lucas)
/// a carreira Rust se separava da original em 2002-01-12; a busca binaria entre as 301 rotinas
/// apontou so ela, e uma sonda (a original e a copia tracando o mesmo lance, sem rodar nada) deu os
/// mesmos quadrados e argumentos: o que muda e o resto da pilha que fica para quem e chamado depois.
/// 0x6b5f00 `fechar_partida_do_dia` e 0x6b74a0 `eventos_do_pos_jogo` (31/10/2026): na carreira ampla (8 ligas, ate
/// 2003-01-31, DLL da 253a) o Rust se separava da original em 2002-299; a busca binaria entre as 1083 rotinas
/// (SEM_ROTINAS) so fecha com as duas juntas na original (388f944f, igual a original); uma so nao basta. A copia dos
/// eventos confere ramo a ramo com a desmontagem, e envenenar o quadro das 8 rotinas sem copia chamadas por ela nao
/// muda a carreira: quem le a sobra esta mais fundo ou depois dos eventos, dentro do fechamento (a varredura das
/// leitoras, `leitoras.py`, deve acha-la).
/// Desde 31/10/2026 (Lucas) nenhuma, pelo mesmo critério das DESLIGADAS.
const SO_NA_COMPARACAO: &[usize] = &[];


unsafe extern "thiscall" fn comparar_marcacao_individual(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(164);
    let reg = [(this, MOTOR)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || marcacao_individual(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|r| (0..a[r].1.len()).filter(move |&i| a[r].1[i] != b[r].1[i]).map(move |i| a[r].0 + i)).take(16).collect();
        format!("marcacao_individual diferencas em {:x?}", dif)
    });
}

unsafe extern "C" fn comparar_nova_reputacao(this: usize, lado: i8, valor: f32, alt: u8, bonus: u8) {
    let o = orig::<unsafe extern "C" fn(usize, i8, f32, u8, u8)>(165);
    if gravando_fechamento() {
        chamar_gravando(0x6d3de0, this, [lado as u8 as u32, valor.to_bits(), alt as u32 | (bonus as u32) << 8, 0, 0], || {
            o(this, lado, valor, alt, bonus);
            0
        });
        return;
    }
    let reg = [(this, 0x200)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, lado, valor, alt, bonus), || nova_reputacao(this, lado, valor, alt, bonus));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        format!("nova_reputacao lado {} valor {} alt {} bonus {}: {:?} x {:?}", lado, valor, alt, bonus, &a[0x1d6..0x1da], &b[0x1d6..0x1da])
    });
}

unsafe extern "C" fn comparar_reputacao_do_jogador(reg: usize, meu: usize, outro: usize, partida: usize, valor: f32, alt: u8, bonus: u8) {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, usize, f32, u8, u8)>(166);
    if gravando_fechamento() {
        chamar_gravando(0x6d3610, reg, [meu as u32, outro as u32, partida as u32, valor.to_bits(), alt as u32 | (bonus as u32) << 8], || {
            o(reg, meu, outro, partida, valor, alt, bonus);
            0
        });
        return;
    }
    let regioes = [(reg, 6)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(reg, meu, outro, partida, valor, alt, bonus), || reputacao_do_jogador(reg, meu, outro, partida, valor, alt, bonus));
    PLACAR.registrar(igual, || format!("reputacao_do_jogador valor {} alt {} bonus {}: {:?} x {:?}", valor, alt, bonus, mo.0[0].1, mr.0[0].1));
}

unsafe extern "thiscall" fn comparar_escolher_cobrador(this: usize, lado: i8, tipo: i8, excluidos: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize>(167);
    let mut regioes = vec![(this, MOTOR)];
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
    let mut regioes = vec![(this, MOTOR), (0xb67a34, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || crate::aleatorio::sem_contar(|| r(this)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("{} diferencas em {:x?}", nome, dif)
    });
}
unsafe extern "thiscall" fn comparar_lateral(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if gravando_reinicio() {
        let o = orig::<unsafe extern "thiscall" fn(usize)>(168);
        chamar_gravando(0x6c4290, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    comparar_reinicio("lateral", this, orig::<unsafe extern "thiscall" fn(usize)>(168), lateral);
}
unsafe extern "thiscall" fn comparar_escanteio(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if gravando_reinicio() {
        let o = orig::<unsafe extern "thiscall" fn(usize)>(169);
        chamar_gravando(0x6c4470, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    comparar_reinicio("escanteio", this, orig::<unsafe extern "thiscall" fn(usize)>(169), escanteio);
}
unsafe extern "thiscall" fn comparar_penalti(this: usize) {
    // dentro da comparacao do reinicio (0x6c54a0) a bola parada e uma unidade da fita
    if gravando_reinicio() {
        let o = orig::<unsafe extern "thiscall" fn(usize)>(170);
        chamar_gravando(0x6c50d0, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
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
    let regioes = [(this, MOTOR), (0xb67a34, 4)];
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
        for (k, pr) in PROLOGOS_POS.iter().enumerate() {
            if let Some(t) = desvios::desviar(&format!("motor_porta_{:x} (pos-jogo)", pr.endereco), pr, portas_pos()[k]) {
                PORTAS_POS[k] = t;
            }
        }
        if let Some(t) = desvios::desviar("motor_porta_6f5c30 (gravacao)", &PROLOGO_6F5C30, porta_6f5c30 as *const () as usize) {
            PORTA_6F5C30 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_6d6f70 (gravacao)", &PROLOGO_6D6F70, porta_6d6f70 as *const () as usize) {
            PORTA_6D6F70 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_6eaea0 (gravacao)", &PROLOGO_6EAEA0, porta_6eaea0 as *const () as usize) {
            PORTA_6EAEA0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_716630 (gravacao)", &PROLOGO_716630, porta_716630 as *const () as usize) {
            PORTA_716630 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_716e10 (gravacao)", &PROLOGO_716E10, porta_716e10 as *const () as usize) {
            PORTA_716E10 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_716c90 (gravacao)", &PROLOGO_716C90, porta_716c90 as *const () as usize) {
            PORTA_716C90 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_70dfd0 (gravacao)", &PROLOGO_70DFD0, porta_70dfd0 as *const () as usize) {
            PORTA_70DFD0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_68d0d0 (gravacao)", &PROLOGO_68D0D0, porta_68d0d0 as *const () as usize) {
            PORTA_68D0D0 = t;
        }
        if let Some(t) = desvios::desviar("motor_porta_62ed70 (gravacao)", &PROLOGO_62ED70, porta_62ed70 as *const () as usize) {
            PORTA_62ED70 = t;
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
        colocar_na_vaga as *const () as usize,
        ordem_por_tabela as *const () as usize,
        ordem_por_condicao as *const () as usize,
        titulares as *const () as usize,
        guardar_tatica as *const () as usize,
        aplicar_trocas as *const () as usize,
        punicoes as *const () as usize,
        penaltis as *const () as usize,
        aplicar_falta as *const () as usize,
        dividida as *const () as usize,
        disputa as *const () as usize,
        responder as *const () as usize,
        jogada as *const () as usize,
        eventos_do_pos_jogo as *const () as usize,
        jogadores_do_pos_jogo as *const () as usize,
        clube_do_pos_jogo as *const () as usize,
        partida_sem_humano as *const () as usize,
        interesse_do_dia as *const () as usize,
        reiniciar_lance as *const () as usize,
        chute as *const () as usize,
        nota_do_arbitro as *const () as usize,
        registrar_nota_do_arbitro as *const () as usize,
        batedor_guardado as *const () as usize,
        guardar_instrucoes as *const () as usize,
        numero_da_competicao as *const () as usize,
        favorito_de as *const () as usize,
        rejeitado_por as *const () as usize,
        fechamento as *const () as usize,
        ler_linha_tatica as *const () as usize,
        gravar_linha_tatica as *const () as usize,
        simulacao_rapida as *const () as usize,
        lesionar as *const () as usize,
        registrar_lesao as *const () as usize,
        destaque_do_jogador as *const () as usize,
        desfecho_do_chute as *const () as usize,
        chute_a_gol as *const () as usize,
        mover_jogador as *const () as usize,
        conduzir as *const () as usize,
        laco_do_motor as *const () as usize,
        executar_passe as *const () as usize,
        atras_no_agregado as *const () as usize,
        companheiro_livre as *const () as usize,
        chutar as *const () as usize,
        construir_motor as *const () as usize,
        preparar_partida as *const () as usize,
        destruir_motor as *const () as usize,
        escalar_lado as *const () as usize,
        encerrar_dia as *const () as usize,
        escolher_partida_do_dia as *const () as usize,
        escalar_automatico as *const () as usize,
        iniciar_proxima_partida as *const () as usize,
        andar_partida_do_dia as *const () as usize,
        fechar_partida_do_dia as *const () as usize,
        jogar_o_dia as *const () as usize,
        montar_o_dia as *const () as usize,
        lado_do_jogador as *const () as usize,
        sons_terminados as *const () as usize,
        gravar_eventos as *const () as usize,
        ler_eventos as *const () as usize,
        passo_em_rede as *const () as usize,
        ler_eventos_cfg as *const () as usize,
        texto_do_evento as *const () as usize,
        confirmar_partida_em_rede as *const () as usize,
        narrar_no_painel as *const () as usize,
        abrir_no_conteiner as *const () as usize,
        buscar_no_conteiner as *const () as usize,
        posicao_no_conteiner as *const () as usize,
        mascara_de_posicoes as *const () as usize,
        casa_no_campo as *const () as usize,
        vagas_sem_conflito as *const () as usize,
        estatistica_do_jogador as *const () as usize,
        media_das_notas as *const () as usize,
        zagueiros_sem_conflito as *const () as usize,
        serve_para_a_vaga as *const () as usize,
        codigo_da_posicao as *const () as usize,
        criterio_da_comissao as *const () as usize,
        gravar_vaga_tatica as *const () as usize,
        pessoa_da_vaga_tatica as *const () as usize,
        gravar_formacao_tatica as *const () as usize,
        casa_guardada as *const () as usize,
        ligar_marca_da_formacao as *const () as usize,
        reiniciar_recordes as *const () as usize,
        girar_rodada as *const () as usize,
        registrar_jogo_da_rodada as *const () as usize,
        marcar_ano_no_calendario as *const () as usize,
        vaga_extra_tatica as *const () as usize,
        idades_da_selecao as *const () as usize,
        gravar_dado_do_humano as *const () as usize,
        registro_do_observador as *const () as usize,
        humano_manda_no_clube as *const () as usize,
        categoria_do_contrato as *const () as usize,
        bola_na_area as *const () as usize,
        partida_abandonada as *const () as usize,
        vagas_do_lado_sem_conflito as *const () as usize,
        zagueiros_do_lado_sem_conflito as *const () as usize,
        construir_arbitro as *const () as usize,
        destruir_jogador_da_partida as *const () as usize,
        evento_na_frente as *const () as usize,
        construir_fila_da_narracao as *const () as usize,
        liberar_tabelas_de_eventos as *const () as usize,
        ordem_das_partidas_do_dia as *const () as usize,
        ordem_de_acao as *const () as usize,
        variante_do_evento as *const () as usize,
        combinar_com_o_anterior as *const () as usize,
        comentario_de_contexto as *const () as usize,
        construir_jogador_da_partida as *const () as usize,
        proximo_evento as *const () as usize,
        nome_do_jogador_na_narracao as *const () as usize,
        habilidade_vista as *const () as usize,
        cumprir_regra_do_elenco as *const () as usize,
        nota_fora_da_partida as *const () as usize,
        nota_para_a_vaga as *const () as usize,
        pessoa_na_lista as *const () as usize,
        ficha_de_contrato as *const () as usize,
        desgaste_diario as *const () as usize,
        registro_da_formacao as *const () as usize,
        peso_da_competicao as *const () as usize,
        gravar_dado_da_noticia as *const () as usize,
        recuperar_lesao as *const () as usize,
        pessoa_na_lista_de_transferencias as *const () as usize,
        punicao_da_tabela as *const () as usize,
        bits_do_registro as *const () as usize,
        competicao_marcada as *const () as usize,
        registro_da_liga as *const () as usize,
        media_do_jogo as *const () as usize,
        valor_do_contrato as *const () as usize,
        nivel_para_a_selecao as *const () as usize,
        oferta_de_contrato as *const () as usize,
        oferta_acertada as *const () as usize,
        oferta_fechada as *const () as usize,
        rotulo_do_controle as *const () as usize,
        fechar_tela_da_partida as *const () as usize,
        abrir_tela_de_espera as *const () as usize,
        noticia_com_resposta_2 as *const () as usize,
        tem_noticia as *const () as usize,
        importancia_da_partida as *const () as usize,
        abrir_ultimos_placares as *const () as usize,
        montar_ultimos_placares as *const () as usize,
        responder_ultimos_placares as *const () as usize,
        abrir_detalhes_da_partida as *const () as usize,
        responder_detalhes_da_partida as *const () as usize,
        fechar_detalhes_da_partida as *const () as usize,
        disciplina_temporada_nova as *const () as usize,
        disciplina_do_dia as *const () as usize,
        disciplina_gravar_vaga as *const () as usize,
        marcar_contrato_acertado as *const () as usize,
        desmarcar_contrato_acertado as *const () as usize,
        vaga_da_oferta as *const () as usize,
        abrir_tela_da_partida as *const () as usize,
        responder_ultimos_placares_da_partida as *const () as usize,
        responder_estatisticas_do_time as *const () as usize,
        painel_dos_ultimos_placares as *const () as usize,
        sem_tela_seguinte as *const () as usize,
        cor_do_titulo as *const () as usize,
        lista_de_observados as *const () as usize,
        entrada_do_treino as *const () as usize,
        gravar_carga_de_treino as *const () as usize,
        e_competicao_do_amistoso as *const () as usize,
        preencher_data_internacional as *const () as usize,
        limpar_registro_de_tecnico as *const () as usize,
        tirar_contrato_de_emprestimo as *const () as usize,
        desmarcar_lances_pelo_jogador as *const () as usize,
        tecnico_substituto as *const () as usize,
        clube_do_contrato_acertado as *const () as usize,
        registrar_jogo_pela_selecao as *const () as usize,
        punicao_guardada as *const () as usize,
        entrada_da_aposentadoria as *const () as usize,
        registrar_competicoes_do_amistoso as *const () as usize,
        vaga_da_noticia as *const () as usize,
        tirar_percentual_da_proxima_venda as *const () as usize,
        tirar_bonus_por_jogo as *const () as usize,
        tirar_bonus_pela_selecao as *const () as usize,
        tirar_reserva_de_tecnico as *const () as usize,
        tirar_jogador_do_banco as *const () as usize,
        premio_da_liga as *const () as usize,
        ultima_nota_do_jogador as *const () as usize,
        vaga_da_busca as *const () as usize,
        tipo_da_lista_da_busca as *const () as usize,
        ordem_do_confronto as *const () as usize,
        atualizar_confronto_direto as *const () as usize,
        marcar_termos_oferecidos as *const () as usize,
        hora_das_ferias as *const () as usize,
        escalacao_guardada as *const () as usize,
        tecnico_da_busca as *const () as usize,
        abrir_jogos_do_dia as *const () as usize,
        responder_jogos_do_dia as *const () as usize,
        vaga_do_mais_bem_avaliado as *const () as usize,
        estatistica_da_liga_do_jogador as *const () as usize,
        entrada_do_jogo_bate as *const () as usize,
        balde_do_registro as *const () as usize,
        atribuir_bloco_do_time as *const () as usize,
        atribuir_linha_do_jogador as *const () as usize,
        contar_placar_ao_vivo as *const () as usize,
        responder_continuar_da_partida as *const () as usize,
        abrir_escalacao_da_partida as *const () as usize,
        responder_escalacao_da_partida as *const () as usize,
        painel_das_estatisticas_do_time as *const () as usize,
        painel_das_notas_dos_jogadores as *const () as usize,
        painel_das_estatisticas_da_partida as *const () as usize,
        valores_das_estatisticas_da_partida as *const () as usize,
        valores_das_zonas_de_acao as *const () as usize,
        valores_das_notas_dos_jogadores as *const () as usize,
        painel_das_zonas_de_acao as *const () as usize,
        painel_do_relatorio_da_partida as *const () as usize,
        montar_jogos_do_dia as *const () as usize,
        preencher_estatisticas_do_time as *const () as usize,
        atualizar_ultimos_placares as *const () as usize,
        esperar_os_outros_jogadores as *const () as usize,
        ler_registro_da_partida as *const () as usize,
        registrar_partida as *const () as usize,
        gravar_registro_da_partida as *const () as usize,
        desenhar_conferencia_da_escalacao as *const () as usize,
        carregar_estatisticas_da_partida as *const () as usize,
        salvar_estatisticas_da_partida as *const () as usize,
        organizar_area as *const () as usize,
        iniciar_area as *const () as usize,
        desenhar_area as *const () as usize,
        montar_tela_da_partida as *const () as usize,
        responder_tela_da_partida as *const () as usize,
        montar_detalhes_da_partida as *const () as usize,
        atualizar_barra_da_posse as *const () as usize,
        painel_da_visao_geral_da_partida as *const () as usize,
        tabela_da_liga_ao_vivo as *const () as usize,
        historia_punivel_do_jogador as *const () as usize,
        escolher_tipo_de_lesao as *const () as usize,
        categoria_da_noticia as *const () as usize,
        apagar_repetidos_das_convocacoes as *const () as usize,
        vaga_do_escalado as *const () as usize,
        escolher_comentarista as *const () as usize,
        regras_da_rodada_no_jogo as *const () as usize,
        chave_do_jogo_na_copa as *const () as usize,
        pais_da_excursao as *const () as usize,
        fechar_propostas_da_oferta as *const () as usize,
        enfileirar_transferencia_livre as *const () as usize,
        negociacoes_de_troca_do_jogador as *const () as usize,
        tirar_jogador_da_selecao as *const () as usize,
        faixa_do_horario as *const () as usize,
        teto_da_observacao as *const () as usize,
        ofertas_pelo_jogador as *const () as usize,
        reputacao_alvo_do_clube as *const () as usize,
        gravar_jogo_na_noticia as *const () as usize,
        preencher_registro_da_partida as *const () as usize,
        ordem_dos_eventos as *const () as usize,
        tirar_idioma_do_jogador as *const () as usize,
        pode_contratar_jogador as *const () as usize,
        trocas_oferecendo_o_jogador as *const () as usize,
        noticias_nao_lidas as *const () as usize,
        gravar_marca_da_noticia as *const () as usize,
        clube_de_tecnico_humano as *const () as usize,
        data_do_sorteio_da_rodada as *const () as usize,
        data_do_jogo_da_rodada as *const () as usize,
        tirar_clube_dos_amistosos as *const () as usize,
        ordem_das_linhas_da_tabela as *const () as usize,
        montar_grade_de_turnos as *const () as usize,
        margem_do_ultimo_dia as *const () as usize,
        partida_das_vagas_da_noticia as *const () as usize,
        resultado_da_eliminatoria_na_noticia as *const () as usize,
        dia_da_rodada_da_copa as *const () as usize,
        mapa_de_casas_da_formacao as *const () as usize,
        nota_do_clube_para_o_jovem as *const () as usize,
        partida_assistida_pelo_humano as *const () as usize,
        largar_partida_assistida as *const () as usize,
        ficha_do_registro as *const () as usize,
        liga_tem_rebaixamento as *const () as usize,
        data_da_rodada_da_liga as *const () as usize,
        importancia_do_jogo_da_liga as *const () as usize,
        copiar_recordista as *const () as usize,
        copiar_registro_de_partida as *const () as usize,
        copiar_registro_de_sequencia as *const () as usize,
        humano_assiste_a_partida as *const () as usize,
        humano_deu_apelido as *const () as usize,
        copiar_aposentadoria_pendente as *const () as usize,
        montar_estatisticas_da_partida as *const () as usize,
        desmontar_estatisticas_da_partida as *const () as usize,
        casar_nome_com_padrao as *const () as usize,
        id_do_conteiner as *const () as usize,
        tamanho_dos_itens_do_padrao as *const () as usize,
        empacotar_no_conteiner as *const () as usize,
        desempacotar_do_conteiner as *const () as usize,
        limpar_pasta_temporaria as *const () as usize,
        ler_do_conteiner as *const () as usize,
        gravar_no_conteiner as *const () as usize,
        fechar_conteiner as *const () as usize,
        filtro_do_jogo_salvo as *const () as usize,
        filtro_do_inicio_rapido as *const () as usize,
        versao_do_jogo_salvo_valida as *const () as usize,
        gravar_regras_fifa as *const () as usize,
        carregar_regras_fifa as *const () as usize,
        montar_resumo_do_save as *const () as usize,
        gravar_banco_no_save as *const () as usize,
        carregar_banco_do_save as *const () as usize,
        liberar_dados_do_jogo as *const () as usize,
        criar_jogo_pela_configuracao as *const () as usize,
        gravar_jogo as *const () as usize,
        carregar_jogo as *const () as usize,
        escopo_da_reputacao as *const () as usize,
        reputacao_vista_por as *const () as usize,
        pessoa_e_estrangeira as *const () as usize,
        idade_da_pessoa as *const () as usize,
        nacao_com_transferencia_livre as *const () as usize,
        pessoa_e_comunitaria as *const () as usize,
        pessoa_listada_para_transferencia as *const () as usize,
        atributo_calculado_do_jogador as *const () as usize,
        atributo_da_pessoa as *const () as usize,
        ordem_do_historico_de_competicao as *const () as usize,
        vetor_de_atributos_do_jogador as *const () as usize,
        nacao_britanica_ou_irlanda as *const () as usize,
        clube_basco as *const () as usize,
        pessoa_listada_pelo_clube as *const () as usize,
        pessoa_listada_para_emprestimo as *const () as usize,
        pessoa_listada_a_pedido as *const () as usize,
        nota_relativa_do_atributo as *const () as usize,
        clube_pode_ter_reservas as *const () as usize,
        primeiro_nome_da_nacao as *const () as usize,
        segundo_nome_da_nacao as *const () as usize,
        formatar_nome_da_pessoa as *const () as usize,
        cores_do_uniforme_do_clube as *const () as usize,
        tecnico_do_clube_ou_selecao as *const () as usize,
        nome_ingles_do_clube as *const () as usize,
        ajustar_moral_da_pessoa as *const () as usize,
        vetor_de_atributos_do_nao_jogador as *const () as usize,
        marcar_elenco_da_pessoa as *const () as usize,
        contar_elenco_do_clube as *const () as usize,
        reunir_elenco_do_clube as *const () as usize,
        clube_pode_ser_dirigido as *const () as usize,
        vetor_para_o_nao_jogador as *const () as usize,
        vetor_para_o_jogador as *const () as usize,
        normalizar_atributos_do_nao_jogador as *const () as usize,
        familia_da_cor as *const () as usize,
        cores_se_confundem as *const () as usize,
        atributo_calculado_do_nao_jogador as *const () as usize,
        forma_do_adjetivo as *const () as usize,
        atributo_do_nao_jogador as *const () as usize,
        nota_relativa_com_habilidade_reduzida as *const () as usize,
        primeiro_nome_pelo_peso as *const () as usize,
        cores_da_competicao as *const () as usize,
        clube_tem_vaga_para_cargo as *const () as usize,
        incluir_pessoa_no_clube as *const () as usize,
        casar_chave_de_clube as *const () as usize,
        remapear_chave_de_clube as *const () as usize,
        casar_chave_de_nacao as *const () as usize,
        casar_chave_de_competicao_de_clube as *const () as usize,
        casar_chave_de_competicao_de_pessoa as *const () as usize,
        ler_continentes as *const () as usize,
        ler_nacoes as *const () as usize,
        ler_cidades as *const () as usize,
        ler_estadios as *const () as usize,
        ler_pessoas_157 as *const () as usize,
        ler_historico_de_pessoas as *const () as usize,
        ler_historicos_de_competicao as *const () as usize,
        ler_competicoes_de_pessoal as *const () as usize,
        ler_historico_de_competicoes_de_pessoal as *const () as usize,
        ler_arbitros as *const () as usize,
        ler_cores as *const () as usize,
        ler_nomes as *const () as usize,
        ler_clubes as *const () as usize,
        ler_competicoes as *const () as usize,
        ler_nao_jogadores as *const () as usize,
        ler_jogadores as *const () as usize,
        ler_pessoas_110 as *const () as usize,
        ler_preferencias as *const () as usize,
        gravar_continentes as *const () as usize,
        gravar_nacoes as *const () as usize,
        gravar_cidades as *const () as usize,
        gravar_estadios as *const () as usize,
        gravar_clubes as *const () as usize,
        gravar_pessoas_110 as *const () as usize,
        gravar_nao_jogadores as *const () as usize,
        gravar_jogadores as *const () as usize,
        gravar_preferencias as *const () as usize,
        gravar_historico_de_pessoas as *const () as usize,
        gravar_competicoes as *const () as usize,
        gravar_historicos_de_competicao as *const () as usize,
        gravar_competicoes_de_pessoal as *const () as usize,
        gravar_arbitros as *const () as usize,
        gravar_registros_58 as *const () as usize,
        gravar_nomes as *const () as usize,
        abrir_arquivo_do_banco as *const () as usize,
        criar_arquivo_do_banco as *const () as usize,
        liberar_historico_de_pessoas as *const () as usize,
        liberar_historico_de_premios as *const () as usize,
        crescer_historico_de_competicoes as *const () as usize,
        ordem_dos_clubes_pela_reputacao as *const () as usize,
        ordem_do_elenco_pela_habilidade as *const () as usize,
        ordem_do_elenco_pela_habilidade_atual as *const () as usize,
        normalizar_atributos_fisicos_do_jogador as *const () as usize,
        segundo_nome_pelo_peso as *const () as usize,
        normalizar_atributos_do_jogador as *const () as usize,
        ler_clubes_v1 as *const () as usize,
        ler_nao_jogadores_v1 as *const () as usize,
        ler_jogadores_v1 as *const () as usize,
        ler_competicoes_v1 as *const () as usize,
        nome_ingles_da_nacao as *const () as usize,
        nome_curto_ingles_do_clube as *const () as usize,
        cortar_elencos_de_a_e_b as *const () as usize,
        juntar_o_time_b_ao_a as *const () as usize,
        montar_indice_de_nomes as *const () as usize,
        tirar_pessoa_do_clube as *const () as usize,
        olheiros_do_clube as *const () as usize,
        clube_em_temporada as *const () as usize,
        clube_em_nacao_grande as *const () as usize,
        nacao_grande as *const () as usize,
        limpar_vinculos_de_opiniao as *const () as usize,
        juntar_vinculo_de_opiniao as *const () as usize,
        tirar_vinculo_de_opiniao as *const () as usize,
        auxiliar_do_clube as *const () as usize,
        treinadores_do_clube as *const () as usize,
        contar_treinadores_do_clube as *const () as usize,
        sigla_inglesa_da_nacao as *const () as usize,
        nome_ingles_da_competicao as *const () as usize,
        nome_ingles_da_competicao_de_pessoal as *const () as usize,
        nome_ingles_do_estadio as *const () as usize,
        arquivo_do_idioma_existe as *const () as usize,
        liberar_tabelas_em_ingles as *const () as usize,
        gerar_comissao_extra as *const () as usize,
        cortar_tabela_de_clubes as *const () as usize,
        podar_historico_de_competicoes as *const () as usize,
        resolver_indices_do_banco as *const () as usize,
        ordenar_clubes_pela_reputacao as *const () as usize,
        clube_tem_defensores as *const () as usize,
        clube_tem_meias as *const () as usize,
        clube_tem_atacantes as *const () as usize,
        clube_tem_esforcados as *const () as usize,
        clube_tem_goleiro as *const () as usize,
        clube_tem_meias_que_roubam as *const () as usize,
        faltam_atacantes_pelo_alto as *const () as usize,
        faltam_zagueiros_pelo_alto as *const () as usize,
        faltam_zagueiros_rapidos as *const () as usize,
        clube_tem_atacante_rapido as *const () as usize,
        clube_tem_meias_criativos as *const () as usize,
        faixa_de_forca_do_elenco as *const () as usize,
        reputacao_do_clube_para_pessoa as *const () as usize,
        contar_rejeitados_no_clube as *const () as usize,
        contar_favoritos_no_clube as *const () as usize,
        tirar_clube_favorito as *const () as usize,
        clube_tirar_pessoa_favorita as *const () as usize,
        tirar_pessoa_favorita as *const () as usize,
        tirar_pessoa_rejeitada as *const () as usize,
        pessoas_favoritas as *const () as usize,
        clube_acrescentar_pessoa_favorita as *const () as usize,
        clube_acrescentar_pessoa_rejeitada as *const () as usize,
        acrescentar_pessoa_favorita as *const () as usize,
        acrescentar_clube_rejeitado as *const () as usize,
        esmaecer_relacoes as *const () as usize,
        pessoas_rejeitadas as *const () as usize,
        mascara_da_posicao_natural as *const () as usize,
        contar_elenco_com_mascara as *const () as usize,
        escolher_companheiro_favorito as *const () as usize,
        clube_fora_da_argentina as *const () as usize,
        clube_fora_de_argentina_franca_croacia as *const () as usize,
        nacao_da_melanesia as *const () as usize,
        nacao_da_polinesia as *const () as usize,
        nome_do_cargo_de_treino as *const () as usize,
        anos_entre_datas_em_texto as *const () as usize,
        ordem_dos_clubes_pelo_nome_curto as *const () as usize,
        ordem_das_pessoas_pelo_nome_completo as *const () as usize,
        pessoa_confere_com_os_nomes as *const () as usize,
        ordem_dos_clubes_pela_latitude as *const () as usize,
        ordem_dos_clubes_pela_longitude as *const () as usize,
        nome_por_ids as *const () as usize,
        nome_oficial as *const () as usize,
        texto_da_mascara_da_posicao as *const () as usize,
        nome_da_posicao as *const () as usize,
        descricao_do_jogador as *const () as usize,
        melhor_candidato_a_favorito as *const () as usize,
        forcas_do_jogador as *const () as usize,
        fraquezas_do_jogador as *const () as usize,
        contar_fisioterapeutas as *const () as usize,
        clube_e_time_b as *const () as usize,
        pessoas_fora_do_elenco as *const () as usize,
        atributo_visivel as *const () as usize,
        atributo_para_quem_ve as *const () as usize,
        nome_do_atributo_de_tecnico as *const () as usize,
        ordem_das_pessoas_pelo_sobrenome as *const () as usize,
        ordem_dos_ponteiros_de_clube_pela_reputacao as *const () as usize,
        nacao_do_clube_na_cee as *const () as usize,
        regioes_pelo_nome as *const () as usize,
        ordem_das_regioes_pelo_nome as *const () as usize,
        pessoa_da_opiniao as *const () as usize,
        nome_pelos_textos as *const () as usize,
        nome_do_cargo as *const () as usize,
        nome_do_idioma as *const () as usize,
        nome_da_selecao as *const () as usize,
        nome_do_atributo_do_jogador as *const () as usize,
        texto_das_posicoes as *const () as usize,
        nome_da_regiao as *const () as usize,
        adjetivo_do_continente as *const () as usize,
        definir_chaves_fixas as *const () as usize,
        definir_chaves_fixas_1080 as *const () as usize,
        inicializar_jogadores as *const () as usize,
        adjetivo_da_nacao as *const () as usize,
        ordenar_tabela_da_liga as *const () as usize,
        mini_tabela_do_confronto as *const () as usize,
        liberar_tabela_da_liga as *const () as usize,
        liga_completa as *const () as usize,
        pagar_direitos_de_tv as *const () as usize,
        registrar_honras_da_temporada as *const () as usize,
        acrescentar_ao_historico as *const () as usize,
        tipo_de_confronto_na_liga as *const () as usize,
        forca_do_clube as *const () as usize,
        clube_colocado_mais_recente as *const () as usize,
        amistoso_do_clube as *const () as usize,
        lista_da_data as *const () as usize,
        proxima_partida as *const () as usize,
        estadio_em_uso as *const () as usize,
        data_do_amistoso_internacional as *const () as usize,
        encerrar_convocacoes_da_competicao as *const () as usize,
        encerrar_convocacoes_da_nacao as *const () as usize,
        podar_registros_de_convocacao as *const () as usize,
        por_no_elenco_da_selecao as *const () as usize,
        tirar_do_elenco_da_selecao as *const () as usize,
        dispensar_acima_da_idade_do_sub21 as *const () as usize,
        limpar_competicao_atual as *const () as usize,
        liberar_competicoes_do_continente as *const () as usize,
        convocar_substituto as *const () as usize,
        montar_reservas_das_selecoes as *const () as usize,
        ordem_das_reservas as *const () as usize,
        ordem_pelo_clube as *const () as usize,
        adversario_do_amistoso as *const () as usize,
        adversario_da_excursao as *const () as usize,
        aplicar_resultado_na_tabela as *const () as usize,
        pontos_do_resultado as *const () as usize,
        ordem_pela_posicao_do_confronto as *const () as usize,
        ordem_das_datas_da_liga as *const () as usize,
        linha_do_clube as *const () as usize,
        liga_tem_o_clube as *const () as usize,
        copiar_recorde_de_nota as *const () as usize,
        copiar_recorde_de_melhor_em_campo as *const () as usize,
        copiar_recorde_de_disciplina as *const () as usize,
        copiar_recorde_de_contagem as *const () as usize,
        copiar_recorde_do_torcedor as *const () as usize,
        construir_recorde_de_ocorrencias as *const () as usize,
        construir_recorde_de_contagem as *const () as usize,
        copiar_recorde_de_valor as *const () as usize,
        copiar_recorde_de_gols_na_partida as *const () as usize,
        copiar_noticia as *const () as usize,
        copiar_transferencia_passada as *const () as usize,
        copiar_transferencia_futura as *const () as usize,
        copiar_entrada_da_fila_de_regens as *const () as usize,
        esta_na_lista_de_observados as *const () as usize,
        escolher_doador_de_dados as *const () as usize,
        copiar_registro_de_suspensao as *const () as usize,
        salario_padrao_do_contrato as *const () as usize,
        condicao_de_jogo_do_jogador as *const () as usize,
        dias_ate_o_fim_do_contrato as *const () as usize,
        limpar_todas_as_queixas as *const () as usize,
        conferir_retirada_do_pedido as *const () as usize,
        lugar_no_elenco as *const () as usize,
        subir_para_o_principal as *const () as usize,
        descer_para_os_reservas as *const () as usize,
        copiar_contrato_de_pessoal as *const () as usize,
        subir_moral_pela_partida as *const () as usize,
        tirar_conversas_de_contrato as *const () as usize,
        tirar_interesses_de_transferencia as *const () as usize,
        tirar_pedido_de_observacao as *const () as usize,
        tirar_apelido_de_todos as *const () as usize,
        tirar_aposentadoria_pendente as *const () as usize,
        aposentadoria_do_desempregado as *const () as usize,
        oferta_precisa_de_permissao as *const () as usize,
        atualizar_permissoes_de_trabalho as *const () as usize,
        olheiro_da_busca as *const () as usize,
        sequencia_atual_do_clube as *const () as usize,
        construir_recordista as *const () as usize,
        tecnico_rival_para_comentar as *const () as usize,
        construir_recorde_de_valor_pago as *const () as usize,
        construir_recorde_de_valor_recebido as *const () as usize,
        construir_recorde_de_sequencia as *const () as usize,
        construir_recorde_de_vitorias_seguidas as *const () as usize,
        construir_recorde_de_derrotas_seguidas as *const () as usize,
        construir_recorde_de_invencibilidade as *const () as usize,
        construir_recorde_sem_vencer as *const () as usize,
        construir_recorde_de_gols_na_partida as *const () as usize,
        construir_recorde_do_mais_novo as *const () as usize,
        texto_do_fim_do_prazo as *const () as usize,
        pode_encerrar_emprestimo as *const () as usize,
        pow_do_game as *const () as usize,
        tirar_ofertas_guardadas as *const () as usize,
        guardar_suspensoes_do_emprestado as *const () as usize,
        preencher_vaga_da_comissao as *const () as usize,
        noticia_para_os_tecnicos_da_nacao as *const () as usize,
        missao_do_olheiro as *const () as usize,
        copa_completa as *const () as usize,
        construir_recorde_do_mais_velho as *const () as usize,
        permissao_de_trabalho_de as *const () as usize,
        passa_na_permissao as *const () as usize,
        soltar_suspensoes_guardadas as *const () as usize,
        tirar_ofertas_pendentes as *const () as usize,
        marcar_decisao_da_transferencia as *const () as usize,
        noticia_para_todos_os_tecnicos as *const () as usize,
        noticia_para_os_tecnicos_da_divisao as *const () as usize,
        noticia_para_os_tecnicos_do_pais as *const () as usize,
        oferta_bate_o_recorde as *const () as usize,
        condicao_do_jogador as *const () as usize,
        limite_de_escalacao as *const () as usize,
        limpar_partidas_assistidas as *const () as usize,
        alvos_dos_olheiros_do_clube as *const () as usize,
        trocar_olheiro_da_busca as *const () as usize,
        diretoria_depois_da_partida as *const () as usize,
        disciplina_da_partida as *const () as usize,
        entradas_do_clube_na_shortlist as *const () as usize,
        maior_oferta_pelo_jogador as *const () as usize,
        recordes_de_sempre as *const () as usize,
        recordes_da_temporada as *const () as usize,
        guardar_noticia_de_recorde as *const () as usize,
        acrescentar_reserva_de_tecnico as *const () as usize,
        acrescentar_aposentadoria as *const () as usize,
        guardar_vaga_para_regen as *const () as usize,
        acionar_clausulas_de_rebaixamento as *const () as usize,
        tirar_oferta_de_contrato as *const () as usize,
        esta_suspenso as *const () as usize,
        escopo_das_regras_da_partida as *const () as usize,
        codigo_de_inelegibilidade as *const () as usize,
        vaga_do_amistoso as *const () as usize,
        noticia_para_os_tecnicos_do_continente as *const () as usize,
        noticia_para_a_regiao_da_competicao as *const () as usize,
        noticia_importante_para_a_pessoa as *const () as usize,
        texto_do_dado_da_noticia as *const () as usize,
        noticia_de_apuro_financeiro as *const () as usize,
        olheiros_cobrem_a_nacao as *const () as usize,
        fluencia_no_idioma as *const () as usize,
        recomecar_historico_da_pessoa as *const () as usize,
        posicao_na_formacao as *const () as usize,
        clube_tem_oferta_pelo_jogador as *const () as usize,
        oferta_do_clube_pelo_jogador as *const () as usize,
        jogador_com_oferta_pendente as *const () as usize,
        jogador_sob_oferta as *const () as usize,
        clube_do_jogador_com_lei_bosman as *const () as usize,
        veto_da_diretoria_na_oferta as *const () as usize,
        guardar_transferencia_passada as *const () as usize,
        dar_permissao_de_trabalho as *const () as usize,
        guardar_premio_por_jogo_no_time as *const () as usize,
        vaga_livre_de_contrato as *const () as usize,
        acrescentar_contrato_da_pessoa as *const () as usize,
        acrescentar_data_de_decisao as *const () as usize,
        zerar_moral_do_contrato as *const () as usize,
        contrato_pelo_atual as *const () as usize,
        faixa_do_vencimento as *const () as usize,
        registrar_historia_da_midia as *const () as usize,
        expirar_historias_da_midia as *const () as usize,
        historia_ja_registrada as *const () as usize,
        acrescentar_interesse_de_transferencia as *const () as usize,
        ordem_das_chaves_de_ids as *const () as usize,
        ordem_das_historias as *const () as usize,
        decidir_jogo_da_copa as *const () as usize,
        decidir_confronto_da_copa as *const () as usize,
        ordem_dos_cabecas_de_chave as *const () as usize,
        iniciar_linha_da_tabela as *const () as usize,
        vetor_dos_clubes_da_liga as *const () as usize,
        gravar_posicoes_finais_da_liga as *const () as usize,
        mudar_clube_de_divisao as *const () as usize,
        embaralhar_ponteiros as *const () as usize,
        ordenar_e_embaralhar_os_melhores as *const () as usize,
        contar_jogo_da_suspensao as *const () as usize,
        soltar_suspensao as *const () as usize,
        apagar_historico_de_suspensoes as *const () as usize,
        sortear_lesao_recorrente as *const () as usize,
        recomecar_historico_de_lesoes as *const () as usize,
        reiniciar_fisico_do_jogador as *const () as usize,
        somar_totais_de_transferencia as *const () as usize,
        recordes_de_carreira_no_clube as *const () as usize,
        recorde_de_nota_da_temporada as *const () as usize,
        competicao_na_data_do_amistoso as *const () as usize,
        amistoso_perto_da_data as *const () as usize,
        acrescentar_amistoso_planejado as *const () as usize,
        noticia_pelo_alcance as *const () as usize,
        noticia_ao_tecnico_do_clube as *const () as usize,
        entregar_noticia_ao_tecnico as *const () as usize,
        responder_todas_as_noticias as *const () as usize,
        tecnico_tem_noticia_pendente as *const () as usize,
        gravar_texto_na_noticia as *const () as usize,
        nota_do_jogador_para_a_historia as *const () as usize,
        texto_da_crise_financeira as *const () as usize,
        noticias_da_caixa as *const () as usize,
        aposentar_do_elenco as *const () as usize,
        melhor_clube_para_o_jovem as *const () as usize,
        clube_da_nacao_para_o_jovem as *const () as usize,
        abrir_vagas_que_faltam as *const () as usize,
        melhor_nota_de_candidato_a_tecnico as *const () as usize,
        agendar_oferta as *const () as usize,
        estatuto_no_elenco_pelo_clube as *const () as usize,
        estimar_estatuto_no_elenco as *const () as usize,
        liberar_alvos_da_lista_do_clube as *const () as usize,
        busca_cobre_a_nacao as *const () as usize,
        tirar_amistoso_marcado as *const () as usize,
        iniciar_pares_proibidos_de_amistoso as *const () as usize,
        avaliar_o_negocio_da_transferencia as *const () as usize,
        confianca_da_diretoria_na_transferencia as *const () as usize,
        resolver_o_ultimato_da_diretoria as *const () as usize,
        tirar_o_contrato_principal as *const () as usize,
        reenviar_ofertas_guardadas as *const () as usize,
        mes_do_fim_da_temporada_do_clube as *const () as usize,
        pontos_dos_tecnicos_no_dia as *const () as usize,
        pontos_dos_tecnicos_no_fim_da_competicao as *const () as usize,
        tecnico_do_mes_na_liga as *const () as usize,
        saidas_pendentes_dos_tecnicos as *const () as usize,
        tirar_jogador_do_elenco_do_clube as *const () as usize,
        tirar_pessoa_do_cargo_no_clube as *const () as usize,
        construir_noticia as *const () as usize,
        ajustar_confianca_da_torcida as *const () as usize,
        ajustar_confianca_da_diretoria as *const () as usize,
        resposta_do_tecnico_da_maquina as *const () as usize,
        marcar_noticia_lida as *const () as usize,
        achar_noticia_pelo_id as *const () as usize,
        noticia_de_gratidao_do_jogador as *const () as usize,
        noticia_de_pedido_de_apoio as *const () as usize,
        noticia_de_reacao_a_critica as *const () as usize,
        montar_listas_de_posicao as *const () as usize,
        ordenar_listas_de_posicao as *const () as usize,
        clubes_interessados_no_jogador as *const () as usize,
        acrescentar_na_lista_de_observacao as *const () as usize,
        prever_resposta_do_vendedor as *const () as usize,
        periodo_do_emprestimo as *const () as usize,
        prazo_da_resposta_a_oferta as *const () as usize,
        pagar_prestacoes_das_transferencias as *const () as usize,
        tirar_oferta_de_transferencia as *const () as usize,
        acrescentar_suspensao as *const () as usize,
        tirar_suspensoes_de_fora_do_clube as *const () as usize,
        restaurar_suspensoes_guardadas as *const () as usize,
        acrescentar_idioma as *const () as usize,
        melhorar_idioma as *const () as usize,
        zerar_estatisticas_do_jogador as *const () as usize,
        definir_competicao_da_selecao as *const () as usize,
        marcar_clubes_com_jogo as *const () as usize,
        tirar_jogador_da_disciplina as *const () as usize,
        comparar_datas_de_decisao as *const () as usize,
        decidir_as_datas_vencidas as *const () as usize,
        ofertas_de_contrato_do_clube as *const () as usize,
        assinar_contrato_de_emprestimo as *const () as usize,
        encerrar_o_emprestimo as *const () as usize,
        trocar_lugar_no_elenco as *const () as usize,
        diretoria_do_tecnico_novo as *const () as usize,
        motivo_da_saida_do_tecnico as *const () as usize,
        registro_do_tecnico as *const () as usize,
        serve_para_o_cargo as *const () as usize,
        pode_assumir_o_cargo as *const () as usize,
        baixar_moral_pela_partida as *const () as usize,
        montar_vagas_da_formacao as *const () as usize,
        refazer_lista_do_clube as *const () as usize,
        suspender_clubes_que_miram_o_jogador as *const () as usize,
        por_jogador_nas_listas_dos_clubes as *const () as usize,
        por_jogador_a_venda as *const () as usize,
        pagar_premio_por_jogos as *const () as usize,
        clube_recusa_a_oferta as *const () as usize,
        soltar_jogadores_da_oferta as *const () as usize,
        montar_oferta_em_dinheiro as *const () as usize,
        acrescentar_surto_de_doenca as *const () as usize,
        espalhar_surtos_de_doenca as *const () as usize,
        jogador_fora_da_temporada as *const () as usize,
        forma_inicial_dos_elencos as *const () as usize,
        registros_iniciais_de_lesao as *const () as usize,
        listas_de_clubes_por_nacao as *const () as usize,
        jogador_do_ano_da_torcida as *const () as usize,
        arquivar_estatisticas_da_nacao as *const () as usize,
        recordes_de_valor_de_transferencia as *const () as usize,
        par_de_nacoes_sem_amistoso as *const () as usize,
        ordem_das_nacoes_pela_reputacao as *const () as usize,
        ordem_dos_amistosos_pela_data as *const () as usize,
        pode_marcar_jogo_do_clube as *const () as usize,
        ordem_das_convocacoes_pela_data as *const () as usize,
        acrescentar_nacao_classificada as *const () as usize,
        ordem_dos_cabecas_de_chave_decrescente as *const () as usize,
        ordem_dos_cabecas_de_chave_crescente as *const () as usize,
        teto_da_divisao_dos_clubes as *const () as usize,
        pensar_em_demitir_o_tecnico as *const () as usize,
        dar_o_ultimato_da_diretoria as *const () as usize,
        tirar_o_tecnico_do_cargo as *const () as usize,
        avaliar_a_selecao_na_competicao as *const () as usize,
        clube_em_apuro_financeiro as *const () as usize,
        faixa_da_confianca_da_diretoria as *const () as usize,
        registrar_o_acesso as *const () as usize,
        registrar_o_rebaixamento as *const () as usize,
        registrar_a_classificacao_na_fase as *const () as usize,
        registrar_a_transferencia as *const () as usize,
        registrar_a_eliminacao as *const () as usize,
        registrar_a_nao_classificacao as *const () as usize,
        data_de_conclusao_da_oferta as *const () as usize,
        primeiro_dia_permitido_da_oferta as *const () as usize,
        janela_fixa_de_transferencias as *const () as usize,
        compensacao_pela_pessoa as *const () as usize,
        pagar_transferencia_e_clausulas as *const () as usize,
        clube_vendedor_deve_aceitar as *const () as usize,
        permissao_de_trabalho_na_mudanca as *const () as usize,
        concluir_oferta_de_contrato_aceita as *const () as usize,
        oferecer_contrato_ao_jogador_da_oferta as *const () as usize,
        desfazer_oferta as *const () as usize,
        fazer_oferta_de_emprestimo as *const () as usize,
        concluir_transferencias_futuras as *const () as usize,
        nota_do_doador_de_dados as *const () as usize,
        ordem_das_aposentadorias as *const () as usize,
        ordem_da_fila_de_regens as *const () as usize,
        registro_da_categoria as *const () as usize,
        criar_estatisticas as *const () as usize,
        aposentadoria_marcada as *const () as usize,
        copa_sortear_times_da_rodada as *const () as usize,
        copa_separar_estadio_compartilhado as *const () as usize,
        escala_pela_rodada as *const () as usize,
        lado_do_jogo_para_o_clube as *const () as usize,
        ordem_dos_coeficientes_das_nacoes as *const () as usize,
        pedido_de_observacao as *const () as usize,
        limpar_vaga_da_escalacao as *const () as usize,
        marcar_batedor_de_bola_parada as *const () as usize,
        restricoes_da_partida as *const () as usize,
        limite_do_banco_do_dia as *const () as usize,
        precisa_de_permissao_de_trabalho as *const () as usize,
        clube_pode_chegar_ao_jogador as *const () as usize,
        termos_ao_jogador_da_troca as *const () as usize,
        fila_livre_do_clube as *const () as usize,
        transferencia_futura_do_jogador as *const () as usize,
        tirar_transferencia_futura as *const () as usize,
        acrescentar_transferencia_futura as *const () as usize,
        ofertas_abertas_pelos_jogadores_do_clube as *const () as usize,
        desfazer_ofertas_pelo_jogador as *const () as usize,
        guardar_percentual_da_proxima_venda as *const () as usize,
        iniciar_diretorias_dos_clubes as *const () as usize,
        apelacoes_de_suspensao as *const () as usize,
        pode_ver_a_partida as *const () as usize,
        aposentadoria_no_inicio as *const () as usize,
        aposentadoria_por_idade as *const () as usize,
        lista_inicial_de_aposentadorias as *const () as usize,
        novas_aposentadorias as *const () as usize,
        sair_dos_cargos as *const () as usize,
        registrar_a_liberacao as *const () as usize,
        estado_da_lesao as *const () as usize,
        marcar_decisao_da_oferta as *const () as usize,
        candidatos_a_tecnico_do_ano as *const () as usize,
        projetar_posicoes_da_liga as *const () as usize,
        ordem_da_projecao_pela_reputacao as *const () as usize,
        ordem_da_projecao_pelos_pontos as *const () as usize,
        achar_jogos_do_calendario as *const () as usize,
        jogos_da_competicao_no_dia as *const () as usize,
        competicao_tem_jogo_na_data as *const () as usize,
        clube_livre_na_data as *const () as usize,
        copa_tem_o_clube as *const () as usize,
        nacao_parceira_do_clube as *const () as usize,
        pedido_de_transferencia as *const () as usize,
        cargo_com_contrato as *const () as usize,
        fora_da_diretoria as *const () as usize,
        iniciar_permissoes_de_trabalho as *const () as usize,
        contagem_da_clausula as *const () as usize,
        premio_por_jogo_do_jogador as *const () as usize,
        fila_da_transferencia_livre as *const () as usize,
        proxima_janela_aberta as *const () as usize,
        emprestimo_negociado as *const () as usize,
        oferta_com_clausula as *const () as usize,
        motivo_da_recusa_vale as *const () as usize,
        motivo_pelo_status_no_elenco as *const () as usize,
        relacoes_do_elenco as *const () as usize,
        atrito_com_o_tecnico as *const () as usize,
        moral_pela_lideranca as *const () as usize,
        faltas_ao_treino as *const () as usize,
        competicoes_do_dia as *const () as usize,
        mudar_confianca_da_diretoria as *const () as usize,
        estadios_do_dia as *const () as usize,
        reservar_estadio as *const () as usize,
        estadio_livre_na_data as *const () as usize,
        jogo_com_observado as *const () as usize,
        na_lista_do_olheiro as *const () as usize,
        registrar_a_primeira_convocacao as *const () as usize,
        gols_na_carreira as *const () as usize,
        registrar_o_emprestimo as *const () as usize,
        ordem_da_volta_aos_treinos as *const () as usize,
        gols_pelo_clube as *const () as usize,
        registrar_a_selecao_do_ano as *const () as usize,
        ordem_pelos_pontos_do_tecnico as *const () as usize,
        jogo_guardado_igual as *const () as usize,
        guardar_jogo_no_calendario as *const () as usize,
        tirar_jogo_do_calendario as *const () as usize,
        criar_amistosos_planejados as *const () as usize,
        acrescentar_amistoso as *const () as usize,
        planejar_amistoso_em_data_livre as *const () as usize,
        atualizar_copa_no_dia as *const () as usize,
        montar_rodada_da_copa as *const () as usize,
        risco_de_doenca_na_viagem as *const () as usize,
        jogos_da_selecao_no_periodo as *const () as usize,
        atualizar_ranking_da_fifa as *const () as usize,
        ordem_dos_resultados_por_pontos as *const () as usize,
        resultado_no_historico as *const () as usize,
        faixa_da_selecao as *const () as usize,
        pontuacoes_do_ranking as *const () as usize,
        nacao_chave_do_jogo as *const () as usize,
        e_copa_do_mundo as *const () as usize,
        e_torneio_internacional as *const () as usize,
        indice_da_nacao_chave as *const () as usize,
        ordem_do_ranking as *const () as usize,
        ordem_das_pontuacoes as *const () as usize,
        tempo_das_cidades as *const () as usize,
        ler_regiao_do_clima as *const () as usize,
        ler_periodo_do_clima as *const () as usize,
        coeficientes_das_nacoes as *const () as usize,
        dia_das_aposentadorias as *const () as usize,
        soma_presa as *const () as usize,
        nota_do_estadio_neutro as *const () as usize,
        fator_da_distancia as *const () as usize,
        ler_o_clima as *const () as usize,
        fator_do_treinador as *const () as usize,
        comissao_do_humano as *const () as usize,
        carga_do_programa as *const () as usize,
        aplicar_o_progresso as *const () as usize,
        achar_o_treino_de_posicao as *const () as usize,
        aprende_depressa as *const () as usize,
        avancar_a_semana as *const () as usize,
        tirar_a_ficha_reservada as *const () as usize,
        lesao_que_volta as *const () as usize,
        atualizar_amistosos_do_dia as *const () as usize,
        planejar_pre_temporada_da_ia as *const () as usize,
        janelas_da_pre_temporada as *const () as usize,
        marcar_jogos_da_excursao as *const () as usize,
        ordem_pela_faixa as *const () as usize,
        ordem_pela_nacao_e_faixa as *const () as usize,
        posicoes_previstas_da_copa as *const () as usize,
        ordenar_clubes_pela_forca as *const () as usize,
        gravar_posicao_do_clube as *const () as usize,
        linha_da_nacao as *const () as usize,
        linha_da_nacao_do_clube as *const () as usize,
        ano_da_temporada_do_clube as *const () as usize,
        clube_na_temporada as *const () as usize,
        continente_principal as *const () as usize,
        nacao_principal as *const () as usize,
        estatistica_da_categoria as *const () as usize,
        ordem_dos_idiomas as *const () as usize,
        ordem_da_historia_por_clube as *const () as usize,
        clausula_de_rescisao_ativa as *const () as usize,
        periodo_protegido as *const () as usize,
        tipo_de_bonus_permitido as *const () as usize,
        adiar_pelo_clima as *const () as usize,
        financas_do_clube as *const () as usize,
        ordem_das_reservas_do_estadio as *const () as usize,
        clima_da_partida as *const () as usize,
        temperatura_do_jogo as *const () as usize,
        gramado_do_jogo as *const () as usize,
        faixa_de_riqueza as *const () as usize,
        ordem_por_habilidade as *const () as usize,
        ordem_pela_capacidade as *const () as usize,
        programas_padrao as *const () as usize,
        tipo_do_programa as *const () as usize,
        programa_do_humano as *const () as usize,
        fichas_de_treino_iniciais as *const () as usize,
        guardar_jogador_no_banco as *const () as usize,
        tem_reserva_de_tecnico as *const () as usize,
        suspensao_do_vermelho as *const () as usize,
        nova_temporada_do_orgao as *const () as usize,
        compactar_disciplina_do_orgao as *const () as usize,
        coordenada_da_casa as *const () as usize,
        posicao_repetida_na_formacao as *const () as usize,
        tipo_de_marcacao as *const () as usize,
        sortear_ordem_dos_clubes as *const () as usize,
        clubes_do_lado_do_jogo as *const () as usize,
        clubes_possiveis_do_jogo as *const () as usize,
        nota_alvo_do_clube as *const () as usize,
        jogo_do_historico as *const () as usize,
        gols_do_historico as *const () as usize,
        passar_a_oferta_aos_clubes as *const () as usize,
        noticia_da_oferta_aceita as *const () as usize,
        noticia_do_emprestimo_oferecido as *const () as usize,
        recusa_dos_termos as *const () as usize,
        noticia_da_confianca_da_diretoria as *const () as usize,
        noticia_das_expectativas_da_diretoria as *const () as usize,
        ofertas_em_espera_na_janela as *const () as usize,
        noticia_das_expectativas_da_federacao as *const () as usize,
        noticia_do_protesto_da_torcida as *const () as usize,
        guardar_a_negociacao as *const () as usize,
        noticia_do_boato_do_tecnico as *const () as usize,
        noticia_da_recusa_em_se_mudar as *const () as usize,
        receita_do_merchandising as *const () as usize,
        ingressos_da_temporada as *const () as usize,
        melhorar_o_centro_de_treino as *const () as usize,
        eventos_financeiros_da_semana as *const () as usize,
        diretoria_paga_a_divida as *const () as usize,
        injecao_de_dinheiro as *const () as usize,
        teto_salarial_do_pessoal as *const () as usize,
        pagar_os_bonus as *const () as usize,
        ajustar_o_teto_salarial as *const () as usize,
        efeitos_fisicos_da_partida as *const () as usize,
        batida_do_jogador as *const () as usize,
        jogador_de_ferias as *const () as usize,
        contratar_o_regen as *const () as usize,
        aposentar_pela_lesao as *const () as usize,
        noticia_da_aposentadoria_anunciada as *const () as usize,
        fim_da_lesao as *const () as usize,
        noticia_do_pedido_de_desculpas as *const () as usize,
        esvaziar_caixa_de_entrada as *const () as usize,
        tirar_convocado_machucado as *const () as usize,
        noticia_da_saida_do_convocado as *const () as usize,
        noticia_do_pedido_de_substituto as *const () as usize,
        pode_mudar_a_convocacao as *const () as usize,
        esta_na_convocacao as *const () as usize,
        marcar_adversario_do_amistoso as *const () as usize,
        registrar_premio_do_mes as *const () as usize,
        faixa_de_reputacao_do_amistoso as *const () as usize,
        folga_do_amistoso as *const () as usize,
        satisfacao_com_o_contrato as *const () as usize,
        noticia_quer_contrato_novo as *const () as usize,
        noticia_pedido_de_transferencia_retirado as *const () as usize,
        confirmar_oferta_pelo_clube as *const () as usize,
        tentar_concluir_oferta as *const () as usize,
        acertar_clausulas_anteriores as *const () as usize,
        tentar_emprestimo_pelo_jogador as *const () as usize,
        noticia_contrato_acabando as *const () as usize,
        noticia_saida_pela_lei_bosman as *const () as usize,
        acrescentar_oferta_de_contrato as *const () as usize,
        tirar_oferta_de_contrato_da_vaga as *const () as usize,
        menor_salario_pedido as *const () as usize,
        guardar_notas_da_partida as *const () as usize,
        nota_do_jogador_no_mes as *const () as usize,
        melhor_do_mes as *const () as usize,
        premiar_o_melhor_do_mes as *const () as usize,
        gravar_vencedor_do_mes as *const () as usize,
        reputacao_do_premiado as *const () as usize,
        fisico_do_dia as *const () as usize,
        faixa_da_confianca_na_liga as *const () as usize,
        gravar_notas_da_diretoria as *const () as usize,
        dia_das_lesoes as *const () as usize,
        noticias_de_volta_aos_treinos as *const () as usize,
        um_cartao_da_suspensao as *const () as usize,
        fila_de_regens as *const () as usize,
        veiculo_pelo_clube as *const () as usize,
        veiculo_pela_pessoa as *const () as usize,
        somar_resultado_na_liga as *const () as usize,
        copiar_tabela_da_liga as *const () as usize,
        posicoes_previstas_da_liga as *const () as usize,
        resultado_entre_selecoes as *const () as usize,
        nome_do_elenco_da_selecao as *const () as usize,
        rodada_com_jogos_conhecidos as *const () as usize,
        registrar_convocados_por_clube as *const () as usize,
        importancia_do_jogo_na_competicao as *const () as usize,
        fase_da_competicao as *const () as usize,
        faltas_do_dia as *const () as usize,
        teve_falta_ao_treino as *const () as usize,
        teve_incidente_de_forma as *const () as usize,
        marcar_falta_ao_treino as *const () as usize,
        fim_da_ausencia as *const () as usize,
        marcas_do_contrato as *const () as usize,
        recusar_oferta_de_transferencia as *const () as usize,
        retirar_oferta_de_transferencia as *const () as usize,
        noticia_oferta_recusada as *const () as usize,
        desassossegar_pelo_lance as *const () as usize,
        noticia_oferta_desfeita as *const () as usize,
        noticia_premio_por_jogos as *const () as usize,
        efetivar_transferencia as *const () as usize,
        noticia_transferencia_concluida as *const () as usize,
        noticia_transferencia_acertada as *const () as usize,
        emprestimo_da_configuracao as *const () as usize,
        noticia_fim_do_contrato as *const () as usize,
        noticia_pedido_de_adiamento as *const () as usize,
        tatica_do_clube as *const () as usize,
        escolher_vaga_de_tatica as *const () as usize,
        tatica_ativa_do_humano as *const () as usize,
        tirar_da_escalacao as *const () as usize,
        trocar_vagas_da_escalacao as *const () as usize,
        tem_copia_da_escalacao as *const () as usize,
        noticia_da_pressao as *const () as usize,
        noticia_da_saida_do_tecnico as *const () as usize,
        noticia_do_tecnico_novo as *const () as usize,
        pagar_premios_da_liga as *const () as usize,
        registros_pela_situacao_na_liga as *const () as usize,
        anunciar_situacao_na_liga as *const () as usize,
        registrar_partida_nas_estatisticas as *const () as usize,
        recordes_de_posicao_na_liga as *const () as usize,
        noticias_de_recordes as *const () as usize,
        convocar_selecoes_para_o_jogo as *const () as usize,
        noticia_do_convocado as *const () as usize,
        noticia_do_desfecho_na_copa as *const () as usize,
        contar_vagas_com_lance as *const () as usize,
        tirar_da_lista_de_transferencias as *const () as usize,
        tirar_da_lista_de_emprestimos as *const () as usize,
        situacao_na_lista_de_observacao as *const () as usize,
        por_na_lista_de_emprestimos as *const () as usize,
        teto_de_preco_pelo_estatuto as *const () as usize,
        distribuir_verba_de_transferencias as *const () as usize,
        estatuto_permitido as *const () as usize,
        nome_da_lista_do_olheiro as *const () as usize,
        amistosos_internacionais_do_dia as *const () as usize,
        lista_escolhida_do_tecnico as *const () as usize,
        escolher_lista_do_tecnico as *const () as usize,
        inscritos_da_liga_dos_campeoes as *const () as usize,
        tabela_da_qualificacao_da_liga_dos_campeoes as *const () as usize,
        proxima_fase_da_liga_dos_campeoes as *const () as usize,
        processar_jogo_da_liga_dos_campeoes as *const () as usize,
        progresso_do_clube_na_liga_dos_campeoes as *const () as usize,
        conta_para_conflito_de_datas as *const () as usize,
        escolher_arbitro_da_partida as *const () as usize,
        limpar_arbitros_ocupados as *const () as usize,
        renovar_arbitros as *const () as usize,
        arbitros_sorteados as *const () as usize,
        validar_arbitros as *const () as usize,
        copiar_ficha_de_arbitro as *const () as usize,
        registrar_premio_do_ano as *const () as usize,
        vitoria_maior as *const () as usize,
        derrota_maior as *const () as usize,
        jogos_das_permissoes_e_clausulas as *const () as usize,
        guardar_premio_por_jogos_na_selecao as *const () as usize,
        noticia_transferencia_finalizada as *const () as usize,
        veto_dos_fisios_e_da_diretoria as *const () as usize,
        noticia_do_ultimato as *const () as usize,
        texto_do_ultimato as *const () as usize,
        liberar_sem_custo as *const () as usize,
        ferias_do_humano as *const () as usize,
        noticia_conversa_de_contrato as *const () as usize,
        mata_mata_da_first as *const () as usize,
        mata_mata_da_second as *const () as usize,
        mata_mata_da_third as *const () as usize,
        registros_pela_situacao_inglesa as *const () as usize,
        situacao_na_first as *const () as usize,
        situacao_na_third as *const () as usize,
        posicoes_e_situacao_na_liga as *const () as usize,
        fase_seguinte_da_first as *const () as usize,
        fase_seguinte_da_second as *const () as usize,
        registros_da_liga_dos_campeoes as *const () as usize,
        sorteio_das_quartas as *const () as usize,
        clubes_da_liga_dos_campeoes as *const () as usize,
        registrar_resultado_da_tatica as *const () as usize,
        capitao_guardado as *const () as usize,
        escolher_capitao as *const () as usize,
        selecao_que_tem_o_jogador as *const () as usize,
        jogador_serve_a_selecao as *const () as usize,
        juntar_competicoes_das_convocacoes as *const () as usize,
        registro_da_partida_pelo_clube as *const () as usize,
        referencia_ao_nome as *const () as usize,
        codigo_do_resultado as *const () as usize,
        mover_jogos_vizinhos_do_clube as *const () as usize,
        remarcar_jogos_perto_da_data as *const () as usize,
        noticia_do_jogo_remarcado as *const () as usize,
        incidente_de_forma as *const () as usize,
        noticia_reacao_a_multa as *const () as usize,
        noticia_retorno_do_treinador as *const () as usize,
        sortear_comentarista as *const () as usize,
        nome_do_veiculo as *const () as usize,
        noticia_queixa_a_federacao as *const () as usize,
        folha_salarial_do_clube as *const () as usize,
        refazer_todas_as_folhas as *const () as usize,
        dias_ate_a_temporada as *const () as usize,
        receber_venda_de_jogador as *const () as usize,
        pagar_compra_de_jogador as *const () as usize,
        pagar_luvas as *const () as usize,
        receber_premio_em_dinheiro as *const () as usize,
        receber_direitos_de_tv as *const () as usize,
        pagar_salarios as *const () as usize,
        copa_premio_da_rodada as *const () as usize,
        copa_mandantes_da_rodada as *const () as usize,
        copa_progresso_do_time as *const () as usize,
        amistoso_responder_pedido as *const () as usize,
        amistoso_enviar_pedido as *const () as usize,
        amistoso_lista_das_nacoes as *const () as usize,
        copa_sortear_rodada as *const () as usize,
        nome_da_fase_e_rodada as *const () as usize,
        clube_da_divisao_mais_alta as *const () as usize,
        copa_registros_pelo_desfecho as *const () as usize,
        copa_honras_da_temporada as *const () as usize,
        copa_semifinalistas_para_a_uefa as *const () as usize,
        recusar_oferta_de_emprestimo as *const () as usize,
        vaga_no_elenco_para_a_oferta as *const () as usize,
        percentual_da_proxima_venda_achado as *const () as usize,
        jogador_na_busca as *const () as usize,
        sincronizar_listas_dos_olheiros as *const () as usize,
        vaga_do_jogador_observado as *const () as usize,
        valor_do_jogador_para_o_clube as *const () as usize,
        entrada_do_jogador_na_busca_do_clube as *const () as usize,
        premio_de_jogos_na_selecao_achado as *const () as usize,
        contratos_depois_do_jogo as *const () as usize,
        moral_da_ficha as *const () as usize,
        texto_da_moral as *const () as usize,
        valor_do_jogador as *const () as usize,
        diferenca_de_moral_em_texto as *const () as usize,
        marcas_de_elenco_para_a_tela as *const () as usize,
        jogadores_que_a_diretoria_cita as *const () as usize,
        veredito_da_nota_na_partida as *const () as usize,
        texto_da_diretoria_sobre_os_jogadores as *const () as usize,
        resultado_nos_registros_dos_tecnicos as *const () as usize,
        posicoes_no_fim_da_liga as *const () as usize,
        jogador_do_ano_da_torcida_no_registro as *const () as usize,
        vice_da_copa_nos_registros as *const () as usize,
        terceiro_da_copa_nos_registros as *const () as usize,
        troca_de_tecnico_nos_registros as *const () as usize,
        reacoes_dos_elencos_a_transferencia as *const () as usize,
        noticia_da_comemoracao_da_contratacao as *const () as usize,
        noticia_do_companheiro_desassossegado as *const () as usize,
        renda_da_partida as *const () as usize,
        renda_do_estadio as *const () as usize,
        pagar_multa_da_liga as *const () as usize,
        impulso_das_financas as *const () as usize,
        troca_do_presidente as *const () as usize,
        pagar_outros_custos as *const () as usize,
        tabela_das_datas_internacionais as *const () as usize,
        resolver_datas_internacionais as *const () as usize,
        torneio_do_clube_humano as *const () as usize,
        amistosos_dos_clubes as *const () as usize,
        acrescentar_torneio as *const () as usize,
        convite_aberto_do_clube_humano as *const () as usize,
        lesao_nova as *const () as usize,
        noticia_da_lesao as *const () as usize,
        ausente_sem_licenca as *const () as usize,
        parte_dos_clubes_anteriores as *const () as usize,
        percentual_da_proxima_venda_pago as *const () as usize,
        noticia_do_percentual_da_venda as *const () as usize,
        permissao_concedida as *const () as usize,
        estatuto_pelo_motivo as *const () as usize,
        vaga_da_negociacao as *const () as usize,
        reacao_do_elenco_a_historia as *const () as usize,
        aviso_ao_rival_de_posicao as *const () as usize,
        rotulo_da_resposta_da_midia as *const () as usize,
        cumprir_suspensoes_internacionais as *const () as usize,
        recurso_da_suspensao as *const () as usize,
        texto_da_vaga_da_suspensao as *const () as usize,
        tirar_da_lista as *const () as usize,
        esvaziar_lista as *const () as usize,
        item_do_cursor as *const () as usize,
        copa_sortear_pela_semente as *const () as usize,
        copa_sortear_separando_nacoes as *const () as usize,
        copa_comeco_da_temporada as *const () as usize,
        nacao_chave_do_clube as *const () as usize,
        nacao_chave_do_estadio as *const () as usize,
        entrada_da_nacao_chave_ativa as *const () as usize,
        conflito_de_jogos_no_mesmo_dia as *const () as usize,
        primeiro_ano_com_jogos as *const () as usize,
        ultimo_ano_com_jogos as *const () as usize,
        gravar_financas as *const () as usize,
        carregar_financas as *const () as usize,
        gravar_gerente_de_elencos as *const () as usize,
        gravar_sementes_da_uefa as *const () as usize,
        gravar_sementes_da_conmebol as *const () as usize,
        gravar_paises_anfitrioes as *const () as usize,
        gravar_historico_dos_clubes as *const () as usize,
        gravar_estatisticas_dos_arbitros as *const () as usize,
        gravar_estatisticas_simuladas as *const () as usize,
        gravar_datas_dos_clubes as *const () as usize,
        carregar_datas_dos_clubes as *const () as usize,
        gravar_conferencia_das_pernas as *const () as usize,
        fim_de_temporada_da_nacao as *const () as usize,
        fim_de_temporada_do_clube as *const () as usize,
        noticia_do_dividendo as *const () as usize,
        teto_salarial as *const () as usize,
        noticia_das_obras as *const () as usize,
        noticia_da_obra_recusada as *const () as usize,
        conflitos_do_dia as *const () as usize,
        conflitos_do_jogo as *const () as usize,
        carregar_conferencia_das_pernas as *const () as usize,
        nova_data_do_jogo as *const () as usize,
        jogo_no_calendario as *const () as usize,
        ano_do_calendario as *const () as usize,
        copa_criar_jogos_do_confronto as *const () as usize,
        copa_noticia_do_confronto_pendente as *const () as usize,
        janelas_avisos_do_dia as *const () as usize,
        copa_levar_jogos_as_datas as *const () as usize,
        tirar_jogador_da_troca as *const () as usize,
        permissoes_da_oferta as *const () as usize,
        confianca_depois_da_partida as *const () as usize,
        noticia_da_reacao_da_diretoria as *const () as usize,
        noticia_da_avaliacao_da_federacao as *const () as usize,
        expectativas_da_temporada as *const () as usize,
        vaga_de_tecnico as *const () as usize,
        noticia_da_oferta_de_emprego as *const () as usize,
        boatos_de_transferencia as *const () as usize,
        noticia_nao_vende as *const () as usize,
        noticia_do_titulo as *const () as usize,
        amistosos_entre_selecoes as *const () as usize,
        convite_de_amistoso_valido as *const () as usize,
        convite_de_torneio_valido as *const () as usize,
        dia_das_selecoes as *const () as usize,
        busca_atual_sem_clube as *const () as usize,
        clube_nas_buscas_da_sessao as *const () as usize,
        escolher_elenco_da_selecao as *const () as usize,
        olheiros_na_partida as *const () as usize,
        sessao_de_edicao as *const () as usize,
        escolher_sessao_de_edicao as *const () as usize,
        dia_das_transferencias as *const () as usize,
        processar_oferta_vencida as *const () as usize,
        restricoes_da_oferta as *const () as usize,
        aceitar_lance as *const () as usize,
        noticia_da_oferta_feita as *const () as usize,
        pode_emprestar as *const () as usize,
        liga_dia as *const () as usize,
        liga_criar_tabela as *const () as usize,
        liga_promover_e_rebaixar as *const () as usize,
        calendario_dia as *const () as usize,
        calendario_lista as *const () as usize,
        calendario_carregar_ano as *const () as usize,
        copa_arvore_dos_confrontos as *const () as usize,
        competicao_primeira_fase_do_grupo as *const () as usize,
        competicao_fase_atual as *const () as usize,
        atualizar_o_treino_do_jogador as *const () as usize,
        programa_de_treino_do_jogador as *const () as usize,
        nivel_da_categoria_do_programa as *const () as usize,
        tirar_ou_devolver_o_progresso as *const () as usize,
        mudanca_do_atributo_pelo_treino as *const () as usize,
        aptidao_para_a_posicao as *const () as usize,
        aposentadorias_do_dia as *const () as usize,
        noticia_da_aposentadoria as *const () as usize,
        noticia_do_jovem_promovido as *const () as usize,
        limpar_ausencia as *const () as usize,
        limpar_falta_ao_treino as *const () as usize,
        noticia_da_falta_ao_treino as *const () as usize,
        troca_com_compensacao as *const () as usize,
        ordem_das_vagas as *const () as usize,
        marcar_sob_oferta as *const () as usize,
        campeao_da_liga_nos_registros as *const () as usize,
        campeao_da_copa_nos_registros as *const () as usize,
        limpar_registro_do_humano as *const () as usize,
        resposta_da_ia_ao_lance as *const () as usize,
        negociacao_de_emprestimo as *const () as usize,
        opcao_de_troca as *const () as usize,
        agendar_fim_do_emprestimo as *const () as usize,
        lances_do_clube as *const () as usize,
        cqe4_interesse as *const () as usize,
        cqe4_construir_contrato as *const () as usize,
        cqe5_pode_emprestar as *const () as usize,
        cqe5_registrar_emprestimo as *const () as usize,
        cqe5_dia_das_regras as *const () as usize,
        cqe6_periodo_inglaterra as *const () as usize,
        cqe6_pode_emprestar_escocia as *const () as usize,
        cqe6_periodo_escocia as *const () as usize,
        cqe6_pode_emprestar_italia as *const () as usize,
        cqe6_pode_emprestar_irlanda as *const () as usize,
        cqe6_pode_emprestar_espanha as *const () as usize,
        liga_jogos_para_a_tv as *const () as usize,
        liga_noticia_dos_grupos as *const () as usize,
        liga_noticia_da_tabela as *const () as usize,
        por_oferta_de_transferencia as *const () as usize,
        aceitar_emprestimo as *const () as usize,
        termos_aceitos as *const () as usize,
        noticia_de_jogador_listado as *const () as usize,
        contrato_oferecido as *const () as usize,
        mundial_sortear_grupos as *const () as usize,
        mundial_sorteio_dos_grupos as *const () as usize,
        mundial_dia as *const () as usize,
        mundial_tabela_do_grupo_um as *const () as usize,
        mundial_fase_do_grupo as *const () as usize,
        mundial_posicoes_previstas as *const () as usize,
        mundial_rodadas_da_fase as *const () as usize,
        mundial_nacao_classificada as *const () as usize,
        mundial_cabecas_de_chave as *const () as usize,
        distribuir_os_treinadores as *const () as usize,
        trocar_os_programas_do_elenco as *const () as usize,
        nomes_padrao_dos_programas as *const () as usize,
        historico_da_carreira as *const () as usize,
        anos_desde_o_clube as *const () as usize,
        ordem_do_historico_por_pessoa as *const () as usize,
        lesao_pelo_nome as *const () as usize,
        noticia_da_volta_aos_treinos as *const () as usize,
        nome_da_categoria_da_lesao as *const () as usize,
        cqf10_olheiro_no_jogo as *const () as usize,
        cqf10_historico_do_clube_observado as *const () as usize,
        cqf11_relatorio_da_selecao as *const () as usize,
        cqf11_noticia_da_convocacao as *const () as usize,
        cqf11_adversarios_da_selecao as *const () as usize,
        cqf12_olheiros_do_dia as *const () as usize,
        cqf12_relatorios_dos_olheiros as *const () as usize,
        cqf12_noticia_do_relatorio as *const () as usize,
        assinar_contrato as *const () as usize,
        refazer_como_jovem as *const () as usize,
        tirar_da_disputa as *const () as usize,
        ler_formacao_do_disco as *const () as usize,
        trocar_bytes_das_palavras as *const () as usize,
        trocar_bytes_dos_dwords as *const () as usize,
        montar_os_arbitros as *const () as usize,
        ordem_das_vagas_de_competicao as *const () as usize,
        cqg10_amistoso_cancelado as *const () as usize,
        cqg10_jogo_da_selecao_cancelado as *const () as usize,
        cqg10_torneio_cancelado as *const () as usize,
        ordem_dos_arbitros_decrescente as *const () as usize,
        ordem_dos_arbitros_crescente as *const () as usize,
        arquivar_estatisticas as *const () as usize,
        pessoa_pelo_nome as *const () as usize,
        texto_entre_aspas as *const () as usize,
        tipo_da_linha as *const () as usize,
        campeoes_sortear_grupos as *const () as usize,
        campeoes_ordem_dos_potes as *const () as usize,
        campeoes_posicoes_previstas as *const () as usize,
        campeoes_sortear_segunda_fase as *const () as usize,
        campeoes_ordem_da_segunda_fase as *const () as usize,
        grupo_da_fase as *const () as usize,
        campeoes_rodadas_da_fase as *const () as usize,
        liga_expectativas_da_temporada as *const () as usize,
        liga_rebaixar_clube as *const () as usize,
        competicao_todas_completas as *const () as usize,
        competicao_lista_de_clubes as *const () as usize,
        competicao_progresso_do_clube as *const () as usize,
        uefa_rodar_coeficientes as *const () as usize,
        uefa_vagas_por_nacao as *const () as usize,
        uefa_enesimo_clube_da_nacao as *const () as usize,
        uefa_juntar_clubes_europeus as *const () as usize,
        uefa_vagas_liga_dos_campeoes as *const () as usize,
        uefa_vagas_intertoto as *const () as usize,
        uefa_vagas_copa_da_uefa as *const () as usize,
        uefa_voluntario_ingles as *const () as usize,
        uefa_virada_da_classificacao as *const () as usize,
        conmebol_vagas_libertadores as *const () as usize,
        conmebol_enesimo_clube_da_nacao as *const () as usize,
        conmebol_convidado_da_mercosul as *const () as usize,
        conmebol_juntar_clubes as *const () as usize,
        conmebol_virada_do_ano as *const () as usize,
        conmebol_campeao_dos_campeoes as *const () as usize,
        sede_da_edicao as *const () as usize,
        proxima_sede as *const () as usize,
        calendario_das_sedes as *const () as usize,
        regras_inglaterra as *const () as usize,
        regras_eua as *const () as usize,
        regras_australia as *const () as usize,
        janelas_espanha as *const () as usize,
        janelas_italia as *const () as usize,
        janelas_franca as *const () as usize,
        regras_brasil as *const () as usize,
        janelas_alemanha as *const () as usize,
        regras_escocia as *const () as usize,
        cqi10_intertoto_dia as *const () as usize,
        cqi10_intertoto_registros as *const () as usize,
        cqi10_uefa_registros as *const () as usize,
        cqi11_concacaf_classificados as *const () as usize,
        cqi11_concacaf_dia as *const () as usize,
        cqi11_concacaf_rodadas as *const () as usize,
        cqi12_copa_italia_regras as *const () as usize,
        cqi12_copa_c_regras as *const () as usize,
        cqi12_copa_dos_campeoes_regras as *const () as usize,
        cqj10_copa_africana_registros as *const () as usize,
        cqj10_copa_america_registros as *const () as usize,
        cqj10_eliminatoria_oceania_registros as *const () as usize,
        cqj11_libertadores_regras as *const () as usize,
        cqj11_merconorte_regras as *const () as usize,
        cqj11_oceania_clubes_regras as *const () as usize,
        cqj12_eliminatoria_europa_regras as *const () as usize,
        cqj12_eliminatoria_sul_americana_regras as *const () as usize,
        cqj12_eliminatoria_asia_regras as *const () as usize,
        cqk10_copa_america_regras as *const () as usize,
        cqk10_copa_africana_regras as *const () as usize,
        cqk10_eliminatoria_oceania_regras as *const () as usize,
        cqk11_libertadores_classificados as *const () as usize,
        cqk11_libertadores_registros as *const () as usize,
        cqk11_libertadores_fase_seguinte as *const () as usize,
        cqk12_africana_fase_seguinte as *const () as usize,
        cqk12_africana_grupo_a as *const () as usize,
        cqk12_africana_historico as *const () as usize,
        cqi7_regras_da_serie_a as *const () as usize,
        cqi7_regras_da_serie_b as *const () as usize,
        cqi7_regras_da_serie_c as *const () as usize,
        cqi8_regras_do_centro_oeste as *const () as usize,
        cqi8_regras_do_gaucho as *const () as usize,
        cqi8_regras_do_catarinense as *const () as usize,
        cqi9_regras_do_paranaense as *const () as usize,
        cqi9_regras_do_pernambucano as *const () as usize,
        cqi9_regras_do_carioca as *const () as usize,
        cqj7_regras_do_goiano as *const () as usize,
        cqj7_regras_da_j1 as *const () as usize,
        cqj7_regras_da_j2 as *const () as usize,
        cqj8_regras_da_nsl as *const () as usize,
        cqj8_regras_da_mls as *const () as usize,
        cqj8_historico_do_estadual as *const () as usize,
        cqj8_historico_do_primeiro_estagio as *const () as usize,
        cqj9_destino_na_serie_a as *const () as usize,
        cqj9_destino_na_serie_c as *const () as usize,
        cqj9_destino_no_baiano as *const () as usize,
        cqk7_destino_no_centro_oeste as *const () as usize,
        cqk7_destino_no_mineiro as *const () as usize,
        cqk7_destino_na_nsl as *const () as usize,
        cqk8_progresso_no_mineiro as *const () as usize,
        cqk8_progresso_na_mls as *const () as usize,
        cqk8_progresso_na_serie_c as *const () as usize,
        cqk9_destino_na_mls as *const () as usize,
        cqk9_resultado_na_mls as *const () as usize,
        cqk9_conferencia_das_estrelas as *const () as usize,
        regras_da_premier as *const () as usize,
        regras_da_first as *const () as usize,
        regras_da_second as *const () as usize,
        regras_da_third as *const () as usize,
        tabela_da_premier as *const () as usize,
        tabela_da_second as *const () as usize,
        tabela_da_third as *const () as usize,
        tabela_da_first as *const () as usize,
        regras_da_serie_a as *const () as usize,
        regras_da_serie_b as *const () as usize,
        regras_da_c1a as *const () as usize,
        regras_da_c1b as *const () as usize,
        tabela_da_serie_a as *const () as usize,
        tabela_da_serie_b as *const () as usize,
        tabela_da_c1a as *const () as usize,
        tabela_da_c1b as *const () as usize,
        regras_da_franca_1 as *const () as usize,
        regras_da_franca_2 as *const () as usize,
        regras_da_franca_3 as *const () as usize,
        tabela_da_franca_1 as *const () as usize,
        tabela_da_franca_2 as *const () as usize,
        tabela_da_franca_3 as *const () as usize,
        regras_da_primera as *const () as usize,
        regras_da_segunda as *const () as usize,
        regras_da_divisao_de_baixo as *const () as usize,
        tabela_da_primera as *const () as usize,
        registros_pela_situacao_na_c2a as *const () as usize,
        situacao_na_primera as *const () as usize,
        tabela_do_grupo_da_cfa as *const () as usize,
        tabela_da_divisao_de_baixo_francesa as *const () as usize,
        situacao_na_c1a as *const () as usize,
        situacao_na_c1b as *const () as usize,
        cql10_copa_da_liga_tabela as *const () as usize,
        cql10_copa_da_inglaterra_tabela as *const () as usize,
        cql10_supercopa_tabela as *const () as usize,
        cql11_copa_da_franca_tabela as *const () as usize,
        cql11_copa_da_liga_francesa_tabela as *const () as usize,
        cql11_copa_do_rei_tabela as *const () as usize,
        cql12_supercopa_da_franca_tabela as *const () as usize,
        cql12_supercopa_da_espanha_tabela as *const () as usize,
        cql12_supercopa_do_japao_tabela as *const () as usize,
        cqm10_copa_da_liga_inscritos as *const () as usize,
        cqm10_trofeu_inscritos as *const () as usize,
        cqm10_supercopa_clubes as *const () as usize,
        cqm11_copa_da_franca_inscritos as *const () as usize,
        cqm11_copa_da_liga_inscritos as *const () as usize,
        cqm11_supercopa_clubes as *const () as usize,
        cqm12_supercopa_da_espanha_clubes as *const () as usize,
        cqm12_supercopa_do_japao_clubes as *const () as usize,
        cqm12_jogo_das_estrelas_tabela as *const () as usize,
        cqn10_copa_da_liga_dia as *const () as usize,
        cqn10_copa_da_inglaterra_dia as *const () as usize,
        cqn10_jogo_das_estrelas_dia as *const () as usize,
        cqn11_copa_da_liga_japonesa_tabela as *const () as usize,
        cqn11_copa_do_imperador_tabela as *const () as usize,
        cqn11_copa_dos_eua_tabela as *const () as usize,
        cqn12_copa_da_liga_inscritos as *const () as usize,
        cqn12_copa_dos_eua_inscritos as *const () as usize,
        cqn12_copa_do_imperador_inscritos as *const () as usize,
        numero_de_camisa as *const () as usize,
        clube_usa_numeros as *const () as usize,
        gravar_numero_de_camisa as *const () as usize,
        dar_numero_de_camisa as *const () as usize,
        numerar_elenco as *const () as usize,
        numerar_no_comeco as *const () as usize,
        numerar_liga as *const () as usize,
        inscrever_na_europa as *const () as usize,
        regras_holanda as *const () as usize,
        regras_croacia as *const () as usize,
        regras_irlanda as *const () as usize,
        regras_noruega as *const () as usize,
        regras_irlanda_do_norte as *const () as usize,
        regras_pais_de_gales as *const () as usize,
        regras_turquia as *const () as usize,
        regras_polonia as *const () as usize,
        regras_finlandia as *const () as usize,
        janelas_argentina as *const () as usize,
        janelas_portugal as *const () as usize,
        regras_coreia as *const () as usize,
        janelas_grecia as *const () as usize,
        janelas_russia as *const () as usize,
        janelas_belgica as *const () as usize,
        inscricao_europeia as *const () as usize,
        ranking_fifa_inicial as *const () as usize,
        lista_de_estadios_neutros as *const () as usize,
        cql7_tabela_da_serie_a as *const () as usize,
        cql7_tabela_da_serie_b as *const () as usize,
        cql7_tabela_da_serie_c as *const () as usize,
        cql8_tabela_do_baiano as *const () as usize,
        cql8_tabela_do_centro_oeste as *const () as usize,
        cql8_tabela_do_gaucho as *const () as usize,
        cql9_tabela_do_goiano as *const () as usize,
        cql9_tabela_do_mineiro as *const () as usize,
        cql9_tabela_do_norte as *const () as usize,
        cqm7_tabela_do_nordeste as *const () as usize,
        cqm7_tabela_do_paranaense as *const () as usize,
        cqm7_tabela_do_pernambucano as *const () as usize,
        cqm8_tabela_do_carioca as *const () as usize,
        cqm8_tabela_do_catarinense as *const () as usize,
        cqm8_tabela_do_paulista as *const () as usize,
        cqm9_tabela_da_j1 as *const () as usize,
        cqm9_tabela_da_j2 as *const () as usize,
        cqm9_tabela_da_nsl as *const () as usize,
        cqn7_mata_mata_do_baiano as *const () as usize,
        cqn7_mata_mata_do_norte as *const () as usize,
        cqn7_mata_mata_do_nordeste as *const () as usize,
        cqn8_mata_mata_do_pernambucano as *const () as usize,
        cqn8_mata_mata_do_catarinense as *const () as usize,
        cqn8_mata_mata_do_paulista as *const () as usize,
        cqn9_mata_mata_do_gaucho as *const () as usize,
        cqn9_mata_mata_do_centro_oeste as *const () as usize,
        cqn9_mata_mata_do_goiano as *const () as usize,
        desempate_da_serie_a as *const () as usize,
        desempate_da_serie_b as *const () as usize,
        ordem_dos_empatados_na_italia as *const () as usize,
        resultado_do_desempate_da_serie_a as *const () as usize,
        fase_seguinte_da_c1a as *const () as usize,
        fases_do_fim_da_c1a as *const () as usize,
        fase_seguinte_da_c1b as *const () as usize,
        fases_do_fim_da_c1b as *const () as usize,
        fase_seguinte_da_third as *const () as usize,
        construir_premier as *const () as usize,
        construir_first as *const () as usize,
        construir_second as *const () as usize,
        construir_third as *const () as usize,
        construir_serie_a as *const () as usize,
        construir_serie_b as *const () as usize,
        construir_c1a as *const () as usize,
        construir_c1b as *const () as usize,
        construir_franca_2 as *const () as usize,
        construir_franca_3 as *const () as usize,
        construir_segunda as *const () as usize,
        construir_primera as *const () as usize,
        construir_divisao_de_baixo_espanhola as *const () as usize,
        tabela_da_segunda as *const () as usize,
        tabela_da_divisao_de_baixo_espanhola as *const () as usize,
        regras_da_cfa as *const () as usize,
        construir_cfa as *const () as usize,
        construir_divisao_de_baixo_francesa as *const () as usize,
        destruir_cfa as *const () as usize,
        destruir_divisao_de_baixo_francesa as *const () as usize,
        calendario_fixo_da_serie_a as *const () as usize,
        calendario_fixo_da_serie_b as *const () as usize,
        calendario_fixo_da_c1a as *const () as usize,
        calendario_fixo_da_c1b as *const () as usize,
        toco_um_com_dois as *const () as usize,
        toco_um_com_um as *const () as usize,
        estadio_da_partida as *const () as usize,
        estadio_neutro as *const () as usize,
        publico_do_jogo as *const () as usize,
        calendario_da_terceira_francesa as *const () as usize,
        calendario_da_segunda_francesa as *const () as usize,
        calendario_da_divisao_de_baixo_francesa as *const () as usize,
        calendario_da_cfa as *const () as usize,
        calendario_da_c1_a as *const () as usize,
        calendario_da_c1_b as *const () as usize,
        calendario_da_serie_a as *const () as usize,
        calendario_da_serie_b as *const () as usize,
        calendario_da_primeira_francesa as *const () as usize,
        cqo7_fase_seguinte_do_mineiro as *const () as usize,
        cqo7_final_do_mineiro as *const () as usize,
        cqo7_fim_da_temporada_do_mineiro as *const () as usize,
        cqo8_fase_seguinte_do_paranaense as *const () as usize,
        cqo8_final_do_paranaense as *const () as usize,
        cqo8_fim_da_temporada_do_paranaense as *const () as usize,
        cqo9_fase_seguinte_do_carioca as *const () as usize,
        cqo9_final_do_carioca as *const () as usize,
        cqo9_fim_da_temporada_do_carioca as *const () as usize,
        cqo10_copa_da_italia_tabela as *const () as usize,
        cqo10_copa_c_tabela as *const () as usize,
        cqo10_supercopa_tabela as *const () as usize,
        cqo10_supercopa_c1_tabela as *const () as usize,
        cqo11_copa_da_italia_inscritos as *const () as usize,
        cqo11_copa_c_inscritos as *const () as usize,
        cqo11_supercopa_clubes as *const () as usize,
        cqo12_copa_do_brasil_tabela as *const () as usize,
        cqo12_times_da_copa as *const () as usize,
        cqo12_regras_da_copa as *const () as usize,
        cqo13_libertadores_tabela as *const () as usize,
        cqo13_mercosul_tabela as *const () as usize,
        cqo13_mercosul_convidados as *const () as usize,
        calendario_da_segunda_inglesa as *const () as usize,
        calendario_da_terceira_inglesa as *const () as usize,
        calendario_da_primeira_inglesa as *const () as usize,
        calendario_da_premiership as *const () as usize,
        calendario_da_primera as *const () as usize,
        cqp7_virada_do_baiano as *const () as usize,
        cqp7_fase_seguinte_do_baiano as *const () as usize,
        cqp7_virada_do_centro_oeste as *const () as usize,
        cqp7_fase_seguinte_do_centro_oeste as *const () as usize,
        cqp7_virada_do_gaucho as *const () as usize,
        cqp7_fase_seguinte_do_gaucho as *const () as usize,
        cqp8_virada_do_goiano as *const () as usize,
        cqp8_fase_seguinte_do_goiano as *const () as usize,
        cqp8_virada_do_norte as *const () as usize,
        cqp8_fase_seguinte_do_norte as *const () as usize,
        cqp8_virada_do_nordeste as *const () as usize,
        cqp8_fase_seguinte_do_nordeste as *const () as usize,
        cqp9_virada_do_pernambucano as *const () as usize,
        cqp9_fase_seguinte_do_pernambucano as *const () as usize,
        cqp9_virada_do_catarinense as *const () as usize,
        cqp9_fase_seguinte_do_catarinense as *const () as usize,
        cqp9_virada_do_paulista as *const () as usize,
        cqp9_fase_seguinte_do_paulista as *const () as usize,
        calendario_da_segunda_espanhola as *const () as usize,
        calendario_da_divisao_de_baixo_espanhola as *const () as usize,
        construir_primeira_francesa as *const () as usize,
        cqq7_fase_seguinte_da_serie_a as *const () as usize,
        cqq7_play_off_da_serie_a as *const () as usize,
        cqq7_fase_seguinte_da_serie_b as *const () as usize,
        cqq7_play_off_da_serie_b as *const () as usize,
        cqq8_virada_da_serie_a as *const () as usize,
        cqq8_acesso_e_rebaixamento as *const () as usize,
        cqq8_virada_da_serie_b as *const () as usize,
        cqq9_fase_seguinte_da_j1 as *const () as usize,
        cqq9_final_da_j1 as *const () as usize,
        cqq9_virada_da_j1 as *const () as usize,
        tabela_das_sedes as *const () as usize,
        noticia_das_sedes as *const () as usize,
        carregar_sedes as *const () as usize,
        cadeia_de_acessos as *const () as usize,
        noticia_do_premio as *const () as usize,
        noticia_da_tv as *const () as usize,
        cqp10_copa_dos_campeoes_tabela as *const () as usize,
        cqp10_merconorte_tabela as *const () as usize,
        cqp10_supercopa_europeia_tabela as *const () as usize,
        cqp10_supercopa_asiatica_tabela as *const () as usize,
        cqp11_asia_clubes_tabela as *const () as usize,
        cqp11_asia_recopa_tabela as *const () as usize,
        cqp11_trofeu_tabela as *const () as usize,
        cqp12_copa_america_tabela as *const () as usize,
        cqp12_africana_tabela as *const () as usize,
        cqp12_confed_tabela as *const () as usize,
        cqp13_intertoto_tabela as *const () as usize,
        cqp13_intercontinental_tabela as *const () as usize,
        cqp13_interamericana_tabela as *const () as usize,
        cqp13_concacaf_tabela as *const () as usize,
        cqq10_uefa_tabela as *const () as usize,
        cqq10_intertoto_inscritos as *const () as usize,
        cqq10_intertoto_lista as *const () as usize,
        cqq11_europeia_clubes as *const () as usize,
        cqq11_asiatica_clubes as *const () as usize,
        cqq11_c1_vagas as *const () as usize,
        cqq12_oceania_tabela as *const () as usize,
        cqq12_asia_clubes_times as *const () as usize,
        cqq12_asia_recopa_times as *const () as usize,
        cqr1_turno_do_carioca as *const () as usize,
        cqr1_turno_do_mineiro as *const () as usize,
        cqr1_turno_do_paranaense as *const () as usize,
        cqr1_turno_da_j1 as *const () as usize,
        cqr2_virada_do_carioca as *const () as usize,
        cqr2_virada_do_mineiro as *const () as usize,
        cqr2_virada_do_paranaense as *const () as usize,
        cqr3_fase_seguinte_da_nsl as *const () as usize,
        cqr3_serie_final_da_nsl as *const () as usize,
        cqr3_grande_final_da_nsl as *const () as usize,
        cqr4_fase_seguinte_da_mls as *const () as usize,
        cqr4_play_off_da_mls as *const () as usize,
        cqr4_final_da_mls as *const () as usize,
        cqr5_virada_copa_do_brasil as *const () as usize,
        cqr5_virada_copa_do_imperador as *const () as usize,
        cqr5_virada_copa_da_liga_japonesa as *const () as usize,
        cqr6_virada_supercopa_japonesa as *const () as usize,
        cqr6_virada_copa_dos_eua as *const () as usize,
        cqr6_virada_jogo_das_estrelas as *const () as usize,
        cqr6_times_do_jogo_das_estrelas as *const () as usize,
        cqr10_libertadores_dia as *const () as usize,
        cqr10_libertadores_primeiro_grupo as *const () as usize,
        cqr10_libertadores_grupo as *const () as usize,
        cqr11_mercosul_dia as *const () as usize,
        cqr11_mercosul_primeiro_grupo as *const () as usize,
        cqr11_mercosul_grupo as *const () as usize,
        cqr12_merconorte_dia as *const () as usize,
        cqr12_merconorte_primeiro_grupo as *const () as usize,
        cqr12_merconorte_grupo as *const () as usize,
        cqr13_copa_america_dia as *const () as usize,
        cqr13_copa_america_primeiro_grupo as *const () as usize,
        cqr13_copa_america_grupo as *const () as usize,
        cqs1_tabela_da_mls as *const () as usize,
        cqs1_segunda_divisao_da_mls as *const () as usize,
        cqs1_terceira_divisao_da_mls as *const () as usize,
        cqs2_virada_da_mls as *const () as usize,
        cqs2_posicoes_previstas_da_mls as *const () as usize,
        cqs2_clubes_da_mls as *const () as usize,
        cqs3_grupo_da_serie_c as *const () as usize,
        cqs3_segunda_fase_da_serie_c as *const () as usize,
        cqs3_terceira_fase_da_serie_c as *const () as usize,
        cqs4_final_da_serie_c as *const () as usize,
        cqs4_virada_da_serie_c as *const () as usize,
        cqs4_posicoes_previstas_da_serie_c as *const () as usize,
        cqs5_trofeu_clubes as *const () as usize,
        cqs5_trofeu_fase_das_chaves as *const () as usize,
        cqs5_trofeu_fase_seguinte as *const () as usize,
        cqs6_trofeu_final as *const () as usize,
        cqs6_trofeu_classificacao as *const () as usize,
        cqs7_fase_de_copa as *const () as usize,
        cqs7_fim_da_fase_de_copa as *const () as usize,
        cqs10_libertadores_mata_mata as *const () as usize,
        cqs10_merconorte_semifinais as *const () as usize,
        cqs10_merconorte_fase_seguinte as *const () as usize,
        cqs11_merconorte_convidados as *const () as usize,
        cqs11_mercosul_fase_seguinte as *const () as usize,
        cqs11_copa_america_fase_seguinte as *const () as usize,
        cqs12_copa_america_selecoes as *const () as usize,
        cqs12_copa_america_previstas as *const () as usize,
        cqs12_copa_america_temporada_nova as *const () as usize,
        cqt1_construir_baiano as *const () as usize,
        cqt1_construir_centro_oeste as *const () as usize,
        cqt1_construir_gaucho as *const () as usize,
        cqt2_construir_goiano as *const () as usize,
        cqt2_construir_mineiro as *const () as usize,
        cqt2_construir_norte as *const () as usize,
        cqt3_construir_nordeste as *const () as usize,
        cqt3_construir_paranaense as *const () as usize,
        cqt3_construir_pernambucano as *const () as usize,
        cqt4_construir_carioca as *const () as usize,
        cqt4_construir_catarinense as *const () as usize,
        cqt4_construir_paulista as *const () as usize,
        cqt4_atualizar_media_de_publico as *const () as usize,
        cqt10_oceania_dia as *const () as usize,
        cqt10_oceania_grupo_a as *const () as usize,
        cqt10_oceania_grupo_b as *const () as usize,
        cqt11_oceania_clubes as *const () as usize,
        cqt11_oceania_fase_seguinte as *const () as usize,
        cqt11_oceania_semifinais as *const () as usize,
        cqt12_asia_fase_seguinte as *const () as usize,
        cqt12_asia_grupos as *const () as usize,
        cqt12_asia_semifinais as *const () as usize,
        cqs13_africana_dia as *const () as usize,
        cqs13_africana_sorteio as *const () as usize,
        cqs13_africana_grupo as *const () as usize,
        cqt13_africana_quartas as *const () as usize,
        cqt13_recopa_asiatica_fase_seguinte as *const () as usize,
        cqt13_recopa_asiatica_finais as *const () as usize,
        cqr7_copa_da_italia_grupo_um as *const () as usize,
        cqr7_copa_da_serie_c_grupo_um as *const () as usize,
        cqr7_copa_da_italia_fase_seguinte as *const () as usize,
        cqr7_copa_da_serie_c_fase_seguinte as *const () as usize,
        cqr8_copa_da_italia_grupo as *const () as usize,
        cqr8_copa_da_serie_c_grupo as *const () as usize,
        cqr9_copa_da_italia_mata_mata as *const () as usize,
        cqr9_copa_da_serie_c_mata_mata as *const () as usize,
        cqs8_eurocopa_regras as *const () as usize,
        cqs8_eurocopa_tamanho_do_grupo as *const () as usize,
        cqs8_eurocopa_situacao as *const () as usize,
        cqs8_eurocopa_grupo_um as *const () as usize,
        cqs9_uefa_inscritos as *const () as usize,
        cqt5_copa_do_rei_dia as *const () as usize,
        cqt5_historico_liga_dos_campeoes as *const () as usize,
        cqt5_historico_copa_da_grecia as *const () as usize,
        cqt5_historico_copa_da_liga_irlandesa as *const () as usize,
        cqt5_historico_copa_da_serie_c as *const () as usize,
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
        comparar_colocar_na_vaga as *const () as usize,
        comparar_ordem_por_tabela as *const () as usize,
        comparar_ordem_por_condicao as *const () as usize,
        comparar_titulares as *const () as usize,
        comparar_guardar_tatica as *const () as usize,
        comparar_aplicar_trocas as *const () as usize,
        comparar_punicoes as *const () as usize,
        comparar_penaltis as *const () as usize,
        comparar_aplicar_falta as *const () as usize,
        comparar_dividida as *const () as usize,
        comparar_disputa as *const () as usize,
        porta_6f5c30 as *const () as usize,
        porta_70dfd0 as *const () as usize,
        comparar_eventos_do_pos_jogo as *const () as usize,
        comparar_jogadores_do_pos_jogo as *const () as usize,
        comparar_clube_do_pos_jogo as *const () as usize,
        comparar_partida_sem_humano as *const () as usize,
        comparar_interesse_do_dia as *const () as usize,
        comparar_reiniciar_lance as *const () as usize,
        comparar_chute as *const () as usize,
        comparar_nota_do_arbitro as *const () as usize,
        comparar_registrar_nota_do_arbitro as *const () as usize,
        comparar_batedor_guardado as *const () as usize,
        comparar_guardar_instrucoes as *const () as usize,
        comparar_numero_da_competicao as *const () as usize,
        comparar_favorito_de as *const () as usize,
        comparar_rejeitado_por as *const () as usize,
        comparar_fechamento as *const () as usize,
        comparar_ler_linha_tatica as *const () as usize,
        comparar_gravar_linha_tatica as *const () as usize,
        comparar_simulacao_rapida as *const () as usize,
        comparar_lesionar as *const () as usize,
        comparar_registrar_lesao as *const () as usize,
        comparar_destaque_do_jogador as *const () as usize,
        comparar_desfecho_do_chute as *const () as usize,
        comparar_chute_a_gol as *const () as usize,
        comparar_mover as *const () as usize,
        comparar_conduzir as *const () as usize,
        comparar_laco_do_motor as *const () as usize,
        comparar_executar_passe as *const () as usize,
        comparar_atras_no_agregado as *const () as usize,
        comparar_companheiro_livre as *const () as usize,
        comparar_chutar as *const () as usize,
        comparar_construir_motor as *const () as usize,
        comparar_preparar_partida as *const () as usize,
        comparar_destruir_motor as *const () as usize,
        comparar_escalar_lado as *const () as usize,
        comparar_encerrar_dia as *const () as usize,
        comparar_escolher_partida_do_dia as *const () as usize,
        comparar_escalar_automatico as *const () as usize,
        comparar_iniciar_proxima_partida as *const () as usize,
        comparar_andar_partida_do_dia as *const () as usize,
        comparar_fechar_partida_do_dia as *const () as usize,
        comparar_jogar_o_dia as *const () as usize,
        comparar_montar_o_dia as *const () as usize,
        comparar_lado_do_jogador as *const () as usize,
        comparar_sons_terminados as *const () as usize,
        comparar_gravar_eventos as *const () as usize,
        comparar_ler_eventos as *const () as usize,
        comparar_passo_em_rede as *const () as usize,
        comparar_ler_eventos_cfg as *const () as usize,
        comparar_texto_do_evento as *const () as usize,
        comparar_confirmar_partida_em_rede as *const () as usize,
        comparar_narrar_no_painel as *const () as usize,
        comparar_abrir_no_conteiner as *const () as usize,
        comparar_buscar_no_conteiner as *const () as usize,
        comparar_posicao_no_conteiner as *const () as usize,
        comparar_mascara_de_posicoes as *const () as usize,
        comparar_casa_no_campo as *const () as usize,
        comparar_vagas_sem_conflito as *const () as usize,
        comparar_estatistica_do_jogador as *const () as usize,
        comparar_media_das_notas as *const () as usize,
        comparar_zagueiros_sem_conflito as *const () as usize,
        comparar_serve_para_a_vaga as *const () as usize,
        comparar_codigo_da_posicao as *const () as usize,
        comparar_criterio_da_comissao as *const () as usize,
        comparar_gravar_vaga_tatica as *const () as usize,
        comparar_pessoa_da_vaga_tatica as *const () as usize,
        comparar_gravar_formacao_tatica as *const () as usize,
        comparar_casa_guardada as *const () as usize,
        comparar_ligar_marca_da_formacao as *const () as usize,
        comparar_reiniciar_recordes as *const () as usize,
        comparar_girar_rodada as *const () as usize,
        comparar_registrar_jogo_da_rodada as *const () as usize,
        comparar_marcar_ano_no_calendario as *const () as usize,
        comparar_vaga_extra_tatica as *const () as usize,
        comparar_idades_da_selecao as *const () as usize,
        comparar_gravar_dado_do_humano as *const () as usize,
        comparar_registro_do_observador as *const () as usize,
        comparar_humano_manda_no_clube as *const () as usize,
        comparar_categoria_do_contrato as *const () as usize,
        comparar_bola_na_area as *const () as usize,
        comparar_partida_abandonada as *const () as usize,
        comparar_vagas_do_lado_sem_conflito as *const () as usize,
        comparar_zagueiros_do_lado_sem_conflito as *const () as usize,
        comparar_construir_arbitro as *const () as usize,
        comparar_destruir_jogador_da_partida as *const () as usize,
        comparar_evento_na_frente as *const () as usize,
        comparar_construir_fila_da_narracao as *const () as usize,
        comparar_liberar_tabelas_de_eventos as *const () as usize,
        comparar_ordem_das_partidas_do_dia as *const () as usize,
        comparar_ordem_de_acao as *const () as usize,
        comparar_variante_do_evento as *const () as usize,
        comparar_combinar_com_o_anterior as *const () as usize,
        comparar_comentario_de_contexto as *const () as usize,
        comparar_construir_jogador_da_partida as *const () as usize,
        comparar_proximo_evento as *const () as usize,
        comparar_nome_do_jogador_na_narracao as *const () as usize,
        comparar_habilidade_vista as *const () as usize,
        comparar_cumprir_regra_do_elenco as *const () as usize,
        comparar_nota_fora_da_partida as *const () as usize,
        comparar_nota_para_a_vaga as *const () as usize,
        comparar_pessoa_na_lista as *const () as usize,
        comparar_ficha_de_contrato as *const () as usize,
        comparar_desgaste_diario as *const () as usize,
        comparar_registro_da_formacao as *const () as usize,
        comparar_peso_da_competicao as *const () as usize,
        comparar_gravar_dado_da_noticia as *const () as usize,
        comparar_recuperar_lesao as *const () as usize,
        comparar_pessoa_na_lista_de_transferencias as *const () as usize,
        comparar_punicao_da_tabela as *const () as usize,
        comparar_bits_do_registro as *const () as usize,
        comparar_competicao_marcada as *const () as usize,
        comparar_registro_da_liga as *const () as usize,
        comparar_media_do_jogo as *const () as usize,
        comparar_valor_do_contrato as *const () as usize,
        comparar_nivel_para_a_selecao as *const () as usize,
        comparar_oferta_de_contrato as *const () as usize,
        comparar_oferta_acertada as *const () as usize,
        comparar_oferta_fechada as *const () as usize,
        comparar_rotulo_do_controle as *const () as usize,
        comparar_fechar_tela_da_partida as *const () as usize,
        comparar_abrir_tela_de_espera as *const () as usize,
        comparar_noticia_com_resposta_2 as *const () as usize,
        comparar_tem_noticia as *const () as usize,
        comparar_importancia_da_partida as *const () as usize,
        comparar_abrir_ultimos_placares as *const () as usize,
        comparar_montar_ultimos_placares as *const () as usize,
        comparar_responder_ultimos_placares as *const () as usize,
        comparar_abrir_detalhes_da_partida as *const () as usize,
        comparar_responder_detalhes_da_partida as *const () as usize,
        comparar_fechar_detalhes_da_partida as *const () as usize,
        comparar_disciplina_temporada_nova as *const () as usize,
        comparar_disciplina_do_dia as *const () as usize,
        comparar_disciplina_gravar_vaga as *const () as usize,
        comparar_marcar_contrato_acertado as *const () as usize,
        comparar_desmarcar_contrato_acertado as *const () as usize,
        comparar_vaga_da_oferta as *const () as usize,
        comparar_abrir_tela_da_partida as *const () as usize,
        comparar_responder_ultimos_placares_da_partida as *const () as usize,
        comparar_responder_estatisticas_do_time as *const () as usize,
        comparar_painel_dos_ultimos_placares as *const () as usize,
        comparar_sem_tela_seguinte as *const () as usize,
        comparar_cor_do_titulo as *const () as usize,
        comparar_lista_de_observados as *const () as usize,
        comparar_entrada_do_treino as *const () as usize,
        comparar_gravar_carga_de_treino as *const () as usize,
        comparar_e_competicao_do_amistoso as *const () as usize,
        comparar_preencher_data_internacional as *const () as usize,
        comparar_limpar_registro_de_tecnico as *const () as usize,
        comparar_tirar_contrato_de_emprestimo as *const () as usize,
        comparar_desmarcar_lances_pelo_jogador as *const () as usize,
        comparar_tecnico_substituto as *const () as usize,
        comparar_clube_do_contrato_acertado as *const () as usize,
        comparar_registrar_jogo_pela_selecao as *const () as usize,
        comparar_punicao_guardada as *const () as usize,
        comparar_entrada_da_aposentadoria as *const () as usize,
        comparar_registrar_competicoes_do_amistoso as *const () as usize,
        comparar_vaga_da_noticia as *const () as usize,
        comparar_tirar_percentual_da_proxima_venda as *const () as usize,
        comparar_tirar_bonus_por_jogo as *const () as usize,
        comparar_tirar_bonus_pela_selecao as *const () as usize,
        comparar_tirar_reserva_de_tecnico as *const () as usize,
        comparar_tirar_jogador_do_banco as *const () as usize,
        comparar_premio_da_liga as *const () as usize,
        comparar_ultima_nota_do_jogador as *const () as usize,
        comparar_vaga_da_busca as *const () as usize,
        comparar_tipo_da_lista_da_busca as *const () as usize,
        comparar_ordem_do_confronto as *const () as usize,
        comparar_atualizar_confronto_direto as *const () as usize,
        comparar_marcar_termos_oferecidos as *const () as usize,
        comparar_hora_das_ferias as *const () as usize,
        comparar_escalacao_guardada as *const () as usize,
        comparar_tecnico_da_busca as *const () as usize,
        comparar_abrir_jogos_do_dia as *const () as usize,
        comparar_responder_jogos_do_dia as *const () as usize,
        comparar_vaga_do_mais_bem_avaliado as *const () as usize,
        comparar_estatistica_da_liga_do_jogador as *const () as usize,
        comparar_entrada_do_jogo_bate as *const () as usize,
        comparar_balde_do_registro as *const () as usize,
        comparar_atribuir_bloco_do_time as *const () as usize,
        comparar_atribuir_linha_do_jogador as *const () as usize,
        comparar_contar_placar_ao_vivo as *const () as usize,
        comparar_responder_continuar_da_partida as *const () as usize,
        comparar_abrir_escalacao_da_partida as *const () as usize,
        comparar_responder_escalacao_da_partida as *const () as usize,
        comparar_painel_das_estatisticas_do_time as *const () as usize,
        comparar_painel_das_notas_dos_jogadores as *const () as usize,
        comparar_painel_das_estatisticas_da_partida as *const () as usize,
        comparar_valores_das_estatisticas_da_partida as *const () as usize,
        comparar_valores_das_zonas_de_acao as *const () as usize,
        comparar_valores_das_notas_dos_jogadores as *const () as usize,
        comparar_painel_das_zonas_de_acao as *const () as usize,
        comparar_painel_do_relatorio_da_partida as *const () as usize,
        comparar_montar_jogos_do_dia as *const () as usize,
        comparar_preencher_estatisticas_do_time as *const () as usize,
        comparar_atualizar_ultimos_placares as *const () as usize,
        comparar_esperar_os_outros_jogadores as *const () as usize,
        comparar_ler_registro_da_partida as *const () as usize,
        comparar_registrar_partida as *const () as usize,
        comparar_gravar_registro_da_partida as *const () as usize,
        comparar_desenhar_conferencia_da_escalacao as *const () as usize,
        comparar_carregar_estatisticas_da_partida as *const () as usize,
        comparar_salvar_estatisticas_da_partida as *const () as usize,
        comparar_organizar_area as *const () as usize,
        comparar_iniciar_area as *const () as usize,
        comparar_desenhar_area as *const () as usize,
        comparar_montar_tela_da_partida as *const () as usize,
        comparar_responder_tela_da_partida as *const () as usize,
        comparar_montar_detalhes_da_partida as *const () as usize,
        comparar_atualizar_barra_da_posse as *const () as usize,
        comparar_painel_da_visao_geral_da_partida as *const () as usize,
        comparar_tabela_da_liga_ao_vivo as *const () as usize,
        comparar_historia_punivel_do_jogador as *const () as usize,
        comparar_escolher_tipo_de_lesao as *const () as usize,
        comparar_categoria_da_noticia as *const () as usize,
        comparar_apagar_repetidos_das_convocacoes as *const () as usize,
        comparar_vaga_do_escalado as *const () as usize,
        comparar_escolher_comentarista as *const () as usize,
        comparar_regras_da_rodada_no_jogo as *const () as usize,
        comparar_chave_do_jogo_na_copa as *const () as usize,
        comparar_pais_da_excursao as *const () as usize,
        comparar_fechar_propostas_da_oferta as *const () as usize,
        comparar_enfileirar_transferencia_livre as *const () as usize,
        comparar_negociacoes_de_troca_do_jogador as *const () as usize,
        comparar_tirar_jogador_da_selecao as *const () as usize,
        comparar_faixa_do_horario as *const () as usize,
        comparar_teto_da_observacao as *const () as usize,
        comparar_ofertas_pelo_jogador as *const () as usize,
        comparar_reputacao_alvo_do_clube as *const () as usize,
        comparar_gravar_jogo_na_noticia as *const () as usize,
        comparar_preencher_registro_da_partida as *const () as usize,
        comparar_ordem_dos_eventos as *const () as usize,
        comparar_tirar_idioma_do_jogador as *const () as usize,
        comparar_pode_contratar_jogador as *const () as usize,
        comparar_trocas_oferecendo_o_jogador as *const () as usize,
        comparar_noticias_nao_lidas as *const () as usize,
        comparar_gravar_marca_da_noticia as *const () as usize,
        comparar_clube_de_tecnico_humano as *const () as usize,
        comparar_data_do_sorteio_da_rodada as *const () as usize,
        comparar_data_do_jogo_da_rodada as *const () as usize,
        comparar_tirar_clube_dos_amistosos as *const () as usize,
        comparar_ordem_das_linhas_da_tabela as *const () as usize,
        comparar_montar_grade_de_turnos as *const () as usize,
        comparar_margem_do_ultimo_dia as *const () as usize,
        comparar_partida_das_vagas_da_noticia as *const () as usize,
        comparar_resultado_da_eliminatoria_na_noticia as *const () as usize,
        comparar_dia_da_rodada_da_copa as *const () as usize,
        comparar_mapa_de_casas_da_formacao as *const () as usize,
        comparar_nota_do_clube_para_o_jovem as *const () as usize,
        comparar_partida_assistida_pelo_humano as *const () as usize,
        comparar_largar_partida_assistida as *const () as usize,
        comparar_ficha_do_registro as *const () as usize,
        comparar_liga_tem_rebaixamento as *const () as usize,
        comparar_data_da_rodada_da_liga as *const () as usize,
        comparar_importancia_do_jogo_da_liga as *const () as usize,
        comparar_copiar_recordista as *const () as usize,
        comparar_copiar_registro_de_partida as *const () as usize,
        comparar_copiar_registro_de_sequencia as *const () as usize,
        comparar_humano_assiste_a_partida as *const () as usize,
        comparar_humano_deu_apelido as *const () as usize,
        comparar_copiar_aposentadoria_pendente as *const () as usize,
        comparar_montar_estatisticas_da_partida as *const () as usize,
        comparar_desmontar_estatisticas_da_partida as *const () as usize,
        comparar_casar_nome_com_padrao as *const () as usize,
        comparar_id_do_conteiner as *const () as usize,
        comparar_tamanho_dos_itens_do_padrao as *const () as usize,
        comparar_empacotar_no_conteiner as *const () as usize,
        comparar_desempacotar_do_conteiner as *const () as usize,
        comparar_limpar_pasta_temporaria as *const () as usize,
        comparar_ler_do_conteiner as *const () as usize,
        comparar_gravar_no_conteiner as *const () as usize,
        comparar_fechar_conteiner as *const () as usize,
        comparar_filtro_do_jogo_salvo as *const () as usize,
        comparar_filtro_do_inicio_rapido as *const () as usize,
        comparar_versao_do_jogo_salvo_valida as *const () as usize,
        comparar_gravar_regras_fifa as *const () as usize,
        comparar_carregar_regras_fifa as *const () as usize,
        comparar_montar_resumo_do_save as *const () as usize,
        comparar_gravar_banco_no_save as *const () as usize,
        comparar_carregar_banco_do_save as *const () as usize,
        comparar_liberar_dados_do_jogo as *const () as usize,
        comparar_criar_jogo_pela_configuracao as *const () as usize,
        comparar_gravar_jogo as *const () as usize,
        comparar_carregar_jogo as *const () as usize,
        comparar_escopo_da_reputacao as *const () as usize,
        comparar_reputacao_vista_por as *const () as usize,
        comparar_pessoa_e_estrangeira as *const () as usize,
        comparar_idade_da_pessoa as *const () as usize,
        comparar_nacao_com_transferencia_livre as *const () as usize,
        comparar_pessoa_e_comunitaria as *const () as usize,
        comparar_pessoa_listada_para_transferencia as *const () as usize,
        comparar_atributo_calculado_do_jogador as *const () as usize,
        comparar_atributo_da_pessoa as *const () as usize,
        comparar_ordem_do_historico_de_competicao as *const () as usize,
        comparar_vetor_de_atributos_do_jogador as *const () as usize,
        comparar_nacao_britanica_ou_irlanda as *const () as usize,
        comparar_clube_basco as *const () as usize,
        comparar_pessoa_listada_pelo_clube as *const () as usize,
        comparar_pessoa_listada_para_emprestimo as *const () as usize,
        comparar_pessoa_listada_a_pedido as *const () as usize,
        comparar_nota_relativa_do_atributo as *const () as usize,
        comparar_clube_pode_ter_reservas as *const () as usize,
        comparar_primeiro_nome_da_nacao as *const () as usize,
        comparar_segundo_nome_da_nacao as *const () as usize,
        comparar_formatar_nome_da_pessoa as *const () as usize,
        comparar_cores_do_uniforme_do_clube as *const () as usize,
        comparar_tecnico_do_clube_ou_selecao as *const () as usize,
        comparar_nome_ingles_do_clube as *const () as usize,
        comparar_ajustar_moral_da_pessoa as *const () as usize,
        comparar_vetor_de_atributos_do_nao_jogador as *const () as usize,
        comparar_marcar_elenco_da_pessoa as *const () as usize,
        comparar_contar_elenco_do_clube as *const () as usize,
        comparar_reunir_elenco_do_clube as *const () as usize,
        comparar_clube_pode_ser_dirigido as *const () as usize,
        comparar_vetor_para_o_nao_jogador as *const () as usize,
        comparar_vetor_para_o_jogador as *const () as usize,
        comparar_normalizar_atributos_do_nao_jogador as *const () as usize,
        comparar_familia_da_cor as *const () as usize,
        comparar_cores_se_confundem as *const () as usize,
        comparar_atributo_calculado_do_nao_jogador as *const () as usize,
        comparar_forma_do_adjetivo as *const () as usize,
        comparar_atributo_do_nao_jogador as *const () as usize,
        comparar_nota_relativa_com_habilidade_reduzida as *const () as usize,
        comparar_primeiro_nome_pelo_peso as *const () as usize,
        comparar_cores_da_competicao as *const () as usize,
        comparar_clube_tem_vaga_para_cargo as *const () as usize,
        comparar_incluir_pessoa_no_clube as *const () as usize,
        comparar_casar_chave_de_clube as *const () as usize,
        comparar_remapear_chave_de_clube as *const () as usize,
        comparar_casar_chave_de_nacao as *const () as usize,
        comparar_casar_chave_de_competicao_de_clube as *const () as usize,
        comparar_casar_chave_de_competicao_de_pessoa as *const () as usize,
        comparar_ler_continentes as *const () as usize,
        comparar_ler_nacoes as *const () as usize,
        comparar_ler_cidades as *const () as usize,
        comparar_ler_estadios as *const () as usize,
        comparar_ler_pessoas_157 as *const () as usize,
        comparar_ler_historico_de_pessoas as *const () as usize,
        comparar_ler_historicos_de_competicao as *const () as usize,
        comparar_ler_competicoes_de_pessoal as *const () as usize,
        comparar_ler_historico_de_competicoes_de_pessoal as *const () as usize,
        comparar_ler_arbitros as *const () as usize,
        comparar_ler_cores as *const () as usize,
        comparar_ler_nomes as *const () as usize,
        comparar_ler_clubes as *const () as usize,
        comparar_ler_competicoes as *const () as usize,
        comparar_ler_nao_jogadores as *const () as usize,
        comparar_ler_jogadores as *const () as usize,
        comparar_ler_pessoas_110 as *const () as usize,
        comparar_ler_preferencias as *const () as usize,
        comparar_gravar_continentes as *const () as usize,
        comparar_gravar_nacoes as *const () as usize,
        comparar_gravar_cidades as *const () as usize,
        comparar_gravar_estadios as *const () as usize,
        comparar_gravar_clubes as *const () as usize,
        comparar_gravar_pessoas_110 as *const () as usize,
        comparar_gravar_nao_jogadores as *const () as usize,
        comparar_gravar_jogadores as *const () as usize,
        comparar_gravar_preferencias as *const () as usize,
        comparar_gravar_historico_de_pessoas as *const () as usize,
        comparar_gravar_competicoes as *const () as usize,
        comparar_gravar_historicos_de_competicao as *const () as usize,
        comparar_gravar_competicoes_de_pessoal as *const () as usize,
        comparar_gravar_arbitros as *const () as usize,
        comparar_gravar_registros_58 as *const () as usize,
        comparar_gravar_nomes as *const () as usize,
        comparar_abrir_arquivo_do_banco as *const () as usize,
        comparar_criar_arquivo_do_banco as *const () as usize,
        comparar_liberar_historico_de_pessoas as *const () as usize,
        comparar_liberar_historico_de_premios as *const () as usize,
        comparar_crescer_historico_de_competicoes as *const () as usize,
        comparar_ordem_dos_clubes_pela_reputacao as *const () as usize,
        comparar_ordem_do_elenco_pela_habilidade as *const () as usize,
        comparar_ordem_do_elenco_pela_habilidade_atual as *const () as usize,
        comparar_normalizar_atributos_fisicos_do_jogador as *const () as usize,
        comparar_segundo_nome_pelo_peso as *const () as usize,
        comparar_normalizar_atributos_do_jogador as *const () as usize,
        comparar_ler_clubes_v1 as *const () as usize,
        comparar_ler_nao_jogadores_v1 as *const () as usize,
        comparar_ler_jogadores_v1 as *const () as usize,
        comparar_ler_competicoes_v1 as *const () as usize,
        comparar_nome_ingles_da_nacao as *const () as usize,
        comparar_nome_curto_ingles_do_clube as *const () as usize,
        comparar_cortar_elencos_de_a_e_b as *const () as usize,
        comparar_juntar_o_time_b_ao_a as *const () as usize,
        comparar_montar_indice_de_nomes as *const () as usize,
        comparar_tirar_pessoa_do_clube as *const () as usize,
        comparar_olheiros_do_clube as *const () as usize,
        comparar_clube_em_temporada as *const () as usize,
        comparar_clube_em_nacao_grande as *const () as usize,
        comparar_nacao_grande as *const () as usize,
        comparar_limpar_vinculos_de_opiniao as *const () as usize,
        comparar_juntar_vinculo_de_opiniao as *const () as usize,
        comparar_tirar_vinculo_de_opiniao as *const () as usize,
        comparar_auxiliar_do_clube as *const () as usize,
        comparar_treinadores_do_clube as *const () as usize,
        comparar_contar_treinadores_do_clube as *const () as usize,
        comparar_sigla_inglesa_da_nacao as *const () as usize,
        comparar_nome_ingles_da_competicao as *const () as usize,
        comparar_nome_ingles_da_competicao_de_pessoal as *const () as usize,
        comparar_nome_ingles_do_estadio as *const () as usize,
        comparar_arquivo_do_idioma_existe as *const () as usize,
        comparar_liberar_tabelas_em_ingles as *const () as usize,
        comparar_gerar_comissao_extra as *const () as usize,
        comparar_cortar_tabela_de_clubes as *const () as usize,
        comparar_podar_historico_de_competicoes as *const () as usize,
        comparar_resolver_indices_do_banco as *const () as usize,
        comparar_ordenar_clubes_pela_reputacao as *const () as usize,
        comparar_clube_tem_defensores as *const () as usize,
        comparar_clube_tem_meias as *const () as usize,
        comparar_clube_tem_atacantes as *const () as usize,
        comparar_clube_tem_esforcados as *const () as usize,
        comparar_clube_tem_goleiro as *const () as usize,
        comparar_clube_tem_meias_que_roubam as *const () as usize,
        comparar_faltam_atacantes_pelo_alto as *const () as usize,
        comparar_faltam_zagueiros_pelo_alto as *const () as usize,
        comparar_faltam_zagueiros_rapidos as *const () as usize,
        comparar_clube_tem_atacante_rapido as *const () as usize,
        comparar_clube_tem_meias_criativos as *const () as usize,
        comparar_faixa_de_forca_do_elenco as *const () as usize,
        comparar_reputacao_do_clube_para_pessoa as *const () as usize,
        comparar_contar_rejeitados_no_clube as *const () as usize,
        comparar_contar_favoritos_no_clube as *const () as usize,
        comparar_tirar_clube_favorito as *const () as usize,
        comparar_clube_tirar_pessoa_favorita as *const () as usize,
        comparar_tirar_pessoa_favorita as *const () as usize,
        comparar_tirar_pessoa_rejeitada as *const () as usize,
        comparar_pessoas_favoritas as *const () as usize,
        comparar_clube_acrescentar_pessoa_favorita as *const () as usize,
        comparar_clube_acrescentar_pessoa_rejeitada as *const () as usize,
        comparar_acrescentar_pessoa_favorita as *const () as usize,
        comparar_acrescentar_clube_rejeitado as *const () as usize,
        comparar_esmaecer_relacoes as *const () as usize,
        comparar_pessoas_rejeitadas as *const () as usize,
        comparar_mascara_da_posicao_natural as *const () as usize,
        comparar_contar_elenco_com_mascara as *const () as usize,
        comparar_escolher_companheiro_favorito as *const () as usize,
        comparar_clube_fora_da_argentina as *const () as usize,
        comparar_clube_fora_de_argentina_franca_croacia as *const () as usize,
        comparar_nacao_da_melanesia as *const () as usize,
        comparar_nacao_da_polinesia as *const () as usize,
        comparar_nome_do_cargo_de_treino as *const () as usize,
        comparar_anos_entre_datas_em_texto as *const () as usize,
        comparar_ordem_dos_clubes_pelo_nome_curto as *const () as usize,
        comparar_ordem_das_pessoas_pelo_nome_completo as *const () as usize,
        comparar_pessoa_confere_com_os_nomes as *const () as usize,
        comparar_ordem_dos_clubes_pela_latitude as *const () as usize,
        comparar_ordem_dos_clubes_pela_longitude as *const () as usize,
        comparar_nome_por_ids as *const () as usize,
        comparar_nome_oficial as *const () as usize,
        comparar_texto_da_mascara_da_posicao as *const () as usize,
        comparar_nome_da_posicao as *const () as usize,
        comparar_descricao_do_jogador as *const () as usize,
        comparar_melhor_candidato_a_favorito as *const () as usize,
        comparar_forcas_do_jogador as *const () as usize,
        comparar_fraquezas_do_jogador as *const () as usize,
        comparar_contar_fisioterapeutas as *const () as usize,
        comparar_clube_e_time_b as *const () as usize,
        comparar_pessoas_fora_do_elenco as *const () as usize,
        comparar_atributo_visivel as *const () as usize,
        comparar_atributo_para_quem_ve as *const () as usize,
        comparar_nome_do_atributo_de_tecnico as *const () as usize,
        comparar_ordem_das_pessoas_pelo_sobrenome as *const () as usize,
        comparar_ordem_dos_ponteiros_de_clube_pela_reputacao as *const () as usize,
        comparar_nacao_do_clube_na_cee as *const () as usize,
        comparar_regioes_pelo_nome as *const () as usize,
        comparar_ordem_das_regioes_pelo_nome as *const () as usize,
        comparar_pessoa_da_opiniao as *const () as usize,
        comparar_nome_pelos_textos as *const () as usize,
        comparar_nome_do_cargo as *const () as usize,
        comparar_nome_do_idioma as *const () as usize,
        comparar_nome_da_selecao as *const () as usize,
        comparar_nome_do_atributo_do_jogador as *const () as usize,
        comparar_texto_das_posicoes as *const () as usize,
        comparar_nome_da_regiao as *const () as usize,
        comparar_adjetivo_do_continente as *const () as usize,
        comparar_definir_chaves_fixas as *const () as usize,
        comparar_definir_chaves_fixas_1080 as *const () as usize,
        comparar_inicializar_jogadores as *const () as usize,
        comparar_adjetivo_da_nacao as *const () as usize,
        comparar_ordenar_tabela_da_liga as *const () as usize,
        comparar_mini_tabela_do_confronto as *const () as usize,
        comparar_liberar_tabela_da_liga as *const () as usize,
        comparar_liga_completa as *const () as usize,
        comparar_pagar_direitos_de_tv as *const () as usize,
        comparar_registrar_honras_da_temporada as *const () as usize,
        comparar_acrescentar_ao_historico as *const () as usize,
        comparar_tipo_de_confronto_na_liga as *const () as usize,
        comparar_forca_do_clube as *const () as usize,
        comparar_clube_colocado_mais_recente as *const () as usize,
        comparar_amistoso_do_clube as *const () as usize,
        comparar_lista_da_data as *const () as usize,
        comparar_proxima_partida as *const () as usize,
        comparar_estadio_em_uso as *const () as usize,
        comparar_data_do_amistoso_internacional as *const () as usize,
        comparar_encerrar_convocacoes_da_competicao as *const () as usize,
        comparar_encerrar_convocacoes_da_nacao as *const () as usize,
        comparar_podar_registros_de_convocacao as *const () as usize,
        comparar_por_no_elenco_da_selecao as *const () as usize,
        comparar_tirar_do_elenco_da_selecao as *const () as usize,
        comparar_dispensar_acima_da_idade_do_sub21 as *const () as usize,
        comparar_limpar_competicao_atual as *const () as usize,
        comparar_liberar_competicoes_do_continente as *const () as usize,
        comparar_convocar_substituto as *const () as usize,
        comparar_montar_reservas_das_selecoes as *const () as usize,
        comparar_ordem_das_reservas as *const () as usize,
        comparar_ordem_pelo_clube as *const () as usize,
        comparar_adversario_do_amistoso as *const () as usize,
        comparar_adversario_da_excursao as *const () as usize,
        comparar_aplicar_resultado_na_tabela as *const () as usize,
        comparar_pontos_do_resultado as *const () as usize,
        comparar_ordem_pela_posicao_do_confronto as *const () as usize,
        comparar_ordem_das_datas_da_liga as *const () as usize,
        comparar_linha_do_clube as *const () as usize,
        comparar_liga_tem_o_clube as *const () as usize,
        comparar_copiar_recorde_de_nota as *const () as usize,
        comparar_copiar_recorde_de_melhor_em_campo as *const () as usize,
        comparar_copiar_recorde_de_disciplina as *const () as usize,
        comparar_copiar_recorde_de_contagem as *const () as usize,
        comparar_copiar_recorde_do_torcedor as *const () as usize,
        comparar_construir_recorde_de_ocorrencias as *const () as usize,
        comparar_construir_recorde_de_contagem as *const () as usize,
        comparar_copiar_recorde_de_valor as *const () as usize,
        comparar_copiar_recorde_de_gols_na_partida as *const () as usize,
        comparar_copiar_noticia as *const () as usize,
        comparar_copiar_transferencia_passada as *const () as usize,
        comparar_copiar_transferencia_futura as *const () as usize,
        comparar_copiar_entrada_da_fila_de_regens as *const () as usize,
        comparar_esta_na_lista_de_observados as *const () as usize,
        comparar_escolher_doador_de_dados as *const () as usize,
        comparar_copiar_registro_de_suspensao as *const () as usize,
        comparar_salario_padrao_do_contrato as *const () as usize,
        comparar_condicao_de_jogo_do_jogador as *const () as usize,
        comparar_dias_ate_o_fim_do_contrato as *const () as usize,
        comparar_limpar_todas_as_queixas as *const () as usize,
        comparar_conferir_retirada_do_pedido as *const () as usize,
        comparar_lugar_no_elenco as *const () as usize,
        comparar_subir_para_o_principal as *const () as usize,
        comparar_descer_para_os_reservas as *const () as usize,
        comparar_copiar_contrato_de_pessoal as *const () as usize,
        comparar_subir_moral_pela_partida as *const () as usize,
        comparar_tirar_conversas_de_contrato as *const () as usize,
        comparar_tirar_interesses_de_transferencia as *const () as usize,
        comparar_tirar_pedido_de_observacao as *const () as usize,
        comparar_tirar_apelido_de_todos as *const () as usize,
        comparar_tirar_aposentadoria_pendente as *const () as usize,
        comparar_aposentadoria_do_desempregado as *const () as usize,
        comparar_oferta_precisa_de_permissao as *const () as usize,
        comparar_atualizar_permissoes_de_trabalho as *const () as usize,
        comparar_olheiro_da_busca as *const () as usize,
        comparar_sequencia_atual_do_clube as *const () as usize,
        comparar_construir_recordista as *const () as usize,
        comparar_tecnico_rival_para_comentar as *const () as usize,
        comparar_construir_recorde_de_valor_pago as *const () as usize,
        comparar_construir_recorde_de_valor_recebido as *const () as usize,
        comparar_construir_recorde_de_sequencia as *const () as usize,
        comparar_construir_recorde_de_vitorias_seguidas as *const () as usize,
        comparar_construir_recorde_de_derrotas_seguidas as *const () as usize,
        comparar_construir_recorde_de_invencibilidade as *const () as usize,
        comparar_construir_recorde_sem_vencer as *const () as usize,
        comparar_construir_recorde_de_gols_na_partida as *const () as usize,
        comparar_construir_recorde_do_mais_novo as *const () as usize,
        comparar_texto_do_fim_do_prazo as *const () as usize,
        comparar_pode_encerrar_emprestimo as *const () as usize,
        comparar_pow_do_game as *const () as usize,
        comparar_tirar_ofertas_guardadas as *const () as usize,
        comparar_guardar_suspensoes_do_emprestado as *const () as usize,
        comparar_preencher_vaga_da_comissao as *const () as usize,
        comparar_noticia_para_os_tecnicos_da_nacao as *const () as usize,
        comparar_missao_do_olheiro as *const () as usize,
        comparar_copa_completa as *const () as usize,
        comparar_construir_recorde_do_mais_velho as *const () as usize,
        comparar_permissao_de_trabalho_de as *const () as usize,
        comparar_passa_na_permissao as *const () as usize,
        comparar_soltar_suspensoes_guardadas as *const () as usize,
        comparar_tirar_ofertas_pendentes as *const () as usize,
        comparar_marcar_decisao_da_transferencia as *const () as usize,
        comparar_noticia_para_todos_os_tecnicos as *const () as usize,
        comparar_noticia_para_os_tecnicos_da_divisao as *const () as usize,
        comparar_noticia_para_os_tecnicos_do_pais as *const () as usize,
        comparar_oferta_bate_o_recorde as *const () as usize,
        comparar_condicao_do_jogador as *const () as usize,
        comparar_limite_de_escalacao as *const () as usize,
        comparar_limpar_partidas_assistidas as *const () as usize,
        comparar_alvos_dos_olheiros_do_clube as *const () as usize,
        comparar_trocar_olheiro_da_busca as *const () as usize,
        comparar_diretoria_depois_da_partida as *const () as usize,
        comparar_disciplina_da_partida as *const () as usize,
        comparar_entradas_do_clube_na_shortlist as *const () as usize,
        comparar_maior_oferta_pelo_jogador as *const () as usize,
        comparar_recordes_de_sempre as *const () as usize,
        comparar_recordes_da_temporada as *const () as usize,
        comparar_guardar_noticia_de_recorde as *const () as usize,
        comparar_acrescentar_reserva_de_tecnico as *const () as usize,
        comparar_acrescentar_aposentadoria as *const () as usize,
        comparar_guardar_vaga_para_regen as *const () as usize,
        comparar_acionar_clausulas_de_rebaixamento as *const () as usize,
        comparar_tirar_oferta_de_contrato as *const () as usize,
        comparar_esta_suspenso as *const () as usize,
        comparar_escopo_das_regras_da_partida as *const () as usize,
        comparar_codigo_de_inelegibilidade as *const () as usize,
        comparar_vaga_do_amistoso as *const () as usize,
        comparar_noticia_para_os_tecnicos_do_continente as *const () as usize,
        comparar_noticia_para_a_regiao_da_competicao as *const () as usize,
        comparar_noticia_importante_para_a_pessoa as *const () as usize,
        comparar_texto_do_dado_da_noticia as *const () as usize,
        comparar_noticia_de_apuro_financeiro as *const () as usize,
        comparar_olheiros_cobrem_a_nacao as *const () as usize,
        comparar_fluencia_no_idioma as *const () as usize,
        comparar_recomecar_historico_da_pessoa as *const () as usize,
        comparar_posicao_na_formacao as *const () as usize,
        comparar_clube_tem_oferta_pelo_jogador as *const () as usize,
        comparar_oferta_do_clube_pelo_jogador as *const () as usize,
        comparar_jogador_com_oferta_pendente as *const () as usize,
        comparar_jogador_sob_oferta as *const () as usize,
        comparar_clube_do_jogador_com_lei_bosman as *const () as usize,
        comparar_veto_da_diretoria_na_oferta as *const () as usize,
        comparar_guardar_transferencia_passada as *const () as usize,
        comparar_dar_permissao_de_trabalho as *const () as usize,
        comparar_guardar_premio_por_jogo_no_time as *const () as usize,
        comparar_vaga_livre_de_contrato as *const () as usize,
        comparar_acrescentar_contrato_da_pessoa as *const () as usize,
        comparar_acrescentar_data_de_decisao as *const () as usize,
        comparar_zerar_moral_do_contrato as *const () as usize,
        comparar_contrato_pelo_atual as *const () as usize,
        comparar_faixa_do_vencimento as *const () as usize,
        comparar_registrar_historia_da_midia as *const () as usize,
        comparar_expirar_historias_da_midia as *const () as usize,
        comparar_historia_ja_registrada as *const () as usize,
        comparar_acrescentar_interesse_de_transferencia as *const () as usize,
        comparar_ordem_das_chaves_de_ids as *const () as usize,
        comparar_ordem_das_historias as *const () as usize,
        comparar_decidir_jogo_da_copa as *const () as usize,
        comparar_decidir_confronto_da_copa as *const () as usize,
        comparar_ordem_dos_cabecas_de_chave as *const () as usize,
        comparar_iniciar_linha_da_tabela as *const () as usize,
        comparar_vetor_dos_clubes_da_liga as *const () as usize,
        comparar_gravar_posicoes_finais_da_liga as *const () as usize,
        comparar_mudar_clube_de_divisao as *const () as usize,
        comparar_embaralhar_ponteiros as *const () as usize,
        comparar_ordenar_e_embaralhar_os_melhores as *const () as usize,
        comparar_contar_jogo_da_suspensao as *const () as usize,
        comparar_soltar_suspensao as *const () as usize,
        comparar_apagar_historico_de_suspensoes as *const () as usize,
        comparar_sortear_lesao_recorrente as *const () as usize,
        comparar_recomecar_historico_de_lesoes as *const () as usize,
        comparar_reiniciar_fisico_do_jogador as *const () as usize,
        comparar_somar_totais_de_transferencia as *const () as usize,
        comparar_recordes_de_carreira_no_clube as *const () as usize,
        comparar_recorde_de_nota_da_temporada as *const () as usize,
        comparar_competicao_na_data_do_amistoso as *const () as usize,
        comparar_amistoso_perto_da_data as *const () as usize,
        comparar_acrescentar_amistoso_planejado as *const () as usize,
        comparar_noticia_pelo_alcance as *const () as usize,
        comparar_noticia_ao_tecnico_do_clube as *const () as usize,
        comparar_entregar_noticia_ao_tecnico as *const () as usize,
        comparar_responder_todas_as_noticias as *const () as usize,
        comparar_tecnico_tem_noticia_pendente as *const () as usize,
        comparar_gravar_texto_na_noticia as *const () as usize,
        comparar_nota_do_jogador_para_a_historia as *const () as usize,
        comparar_texto_da_crise_financeira as *const () as usize,
        comparar_noticias_da_caixa as *const () as usize,
        comparar_aposentar_do_elenco as *const () as usize,
        comparar_melhor_clube_para_o_jovem as *const () as usize,
        comparar_clube_da_nacao_para_o_jovem as *const () as usize,
        comparar_abrir_vagas_que_faltam as *const () as usize,
        comparar_melhor_nota_de_candidato_a_tecnico as *const () as usize,
        comparar_agendar_oferta as *const () as usize,
        comparar_estatuto_no_elenco_pelo_clube as *const () as usize,
        comparar_estimar_estatuto_no_elenco as *const () as usize,
        comparar_liberar_alvos_da_lista_do_clube as *const () as usize,
        comparar_busca_cobre_a_nacao as *const () as usize,
        comparar_tirar_amistoso_marcado as *const () as usize,
        comparar_iniciar_pares_proibidos_de_amistoso as *const () as usize,
        comparar_avaliar_o_negocio_da_transferencia as *const () as usize,
        comparar_confianca_da_diretoria_na_transferencia as *const () as usize,
        comparar_resolver_o_ultimato_da_diretoria as *const () as usize,
        comparar_tirar_o_contrato_principal as *const () as usize,
        comparar_reenviar_ofertas_guardadas as *const () as usize,
        comparar_mes_do_fim_da_temporada_do_clube as *const () as usize,
        comparar_pontos_dos_tecnicos_no_dia as *const () as usize,
        comparar_pontos_dos_tecnicos_no_fim_da_competicao as *const () as usize,
        comparar_tecnico_do_mes_na_liga as *const () as usize,
        comparar_saidas_pendentes_dos_tecnicos as *const () as usize,
        comparar_tirar_jogador_do_elenco_do_clube as *const () as usize,
        comparar_tirar_pessoa_do_cargo_no_clube as *const () as usize,
        comparar_construir_noticia as *const () as usize,
        comparar_ajustar_confianca_da_torcida as *const () as usize,
        comparar_ajustar_confianca_da_diretoria as *const () as usize,
        comparar_resposta_do_tecnico_da_maquina as *const () as usize,
        comparar_marcar_noticia_lida as *const () as usize,
        comparar_achar_noticia_pelo_id as *const () as usize,
        comparar_noticia_de_gratidao_do_jogador as *const () as usize,
        comparar_noticia_de_pedido_de_apoio as *const () as usize,
        comparar_noticia_de_reacao_a_critica as *const () as usize,
        comparar_montar_listas_de_posicao as *const () as usize,
        comparar_ordenar_listas_de_posicao as *const () as usize,
        comparar_clubes_interessados_no_jogador as *const () as usize,
        comparar_acrescentar_na_lista_de_observacao as *const () as usize,
        comparar_prever_resposta_do_vendedor as *const () as usize,
        comparar_periodo_do_emprestimo as *const () as usize,
        comparar_prazo_da_resposta_a_oferta as *const () as usize,
        comparar_pagar_prestacoes_das_transferencias as *const () as usize,
        comparar_tirar_oferta_de_transferencia as *const () as usize,
        comparar_acrescentar_suspensao as *const () as usize,
        comparar_tirar_suspensoes_de_fora_do_clube as *const () as usize,
        comparar_restaurar_suspensoes_guardadas as *const () as usize,
        comparar_acrescentar_idioma as *const () as usize,
        comparar_melhorar_idioma as *const () as usize,
        comparar_zerar_estatisticas_do_jogador as *const () as usize,
        comparar_definir_competicao_da_selecao as *const () as usize,
        comparar_marcar_clubes_com_jogo as *const () as usize,
        comparar_tirar_jogador_da_disciplina as *const () as usize,
        comparar_comparar_datas_de_decisao as *const () as usize,
        comparar_decidir_as_datas_vencidas as *const () as usize,
        comparar_ofertas_de_contrato_do_clube as *const () as usize,
        comparar_assinar_contrato_de_emprestimo as *const () as usize,
        comparar_encerrar_o_emprestimo as *const () as usize,
        comparar_trocar_lugar_no_elenco as *const () as usize,
        comparar_diretoria_do_tecnico_novo as *const () as usize,
        comparar_motivo_da_saida_do_tecnico as *const () as usize,
        comparar_registro_do_tecnico as *const () as usize,
        comparar_serve_para_o_cargo as *const () as usize,
        comparar_pode_assumir_o_cargo as *const () as usize,
        comparar_baixar_moral_pela_partida as *const () as usize,
        comparar_montar_vagas_da_formacao as *const () as usize,
        comparar_refazer_lista_do_clube as *const () as usize,
        comparar_suspender_clubes_que_miram_o_jogador as *const () as usize,
        comparar_por_jogador_nas_listas_dos_clubes as *const () as usize,
        comparar_por_jogador_a_venda as *const () as usize,
        comparar_pagar_premio_por_jogos as *const () as usize,
        comparar_clube_recusa_a_oferta as *const () as usize,
        comparar_soltar_jogadores_da_oferta as *const () as usize,
        comparar_montar_oferta_em_dinheiro as *const () as usize,
        comparar_acrescentar_surto_de_doenca as *const () as usize,
        comparar_espalhar_surtos_de_doenca as *const () as usize,
        comparar_jogador_fora_da_temporada as *const () as usize,
        comparar_forma_inicial_dos_elencos as *const () as usize,
        comparar_registros_iniciais_de_lesao as *const () as usize,
        comparar_listas_de_clubes_por_nacao as *const () as usize,
        comparar_jogador_do_ano_da_torcida as *const () as usize,
        comparar_arquivar_estatisticas_da_nacao as *const () as usize,
        comparar_recordes_de_valor_de_transferencia as *const () as usize,
        comparar_par_de_nacoes_sem_amistoso as *const () as usize,
        comparar_ordem_das_nacoes_pela_reputacao as *const () as usize,
        comparar_ordem_dos_amistosos_pela_data as *const () as usize,
        comparar_pode_marcar_jogo_do_clube as *const () as usize,
        comparar_ordem_das_convocacoes_pela_data as *const () as usize,
        comparar_acrescentar_nacao_classificada as *const () as usize,
        comparar_ordem_dos_cabecas_de_chave_decrescente as *const () as usize,
        comparar_ordem_dos_cabecas_de_chave_crescente as *const () as usize,
        comparar_teto_da_divisao_dos_clubes as *const () as usize,
        comparar_pensar_em_demitir_o_tecnico as *const () as usize,
        comparar_dar_o_ultimato_da_diretoria as *const () as usize,
        comparar_tirar_o_tecnico_do_cargo as *const () as usize,
        comparar_avaliar_a_selecao_na_competicao as *const () as usize,
        comparar_clube_em_apuro_financeiro as *const () as usize,
        comparar_faixa_da_confianca_da_diretoria as *const () as usize,
        comparar_registrar_o_acesso as *const () as usize,
        comparar_registrar_o_rebaixamento as *const () as usize,
        comparar_registrar_a_classificacao_na_fase as *const () as usize,
        comparar_registrar_a_transferencia as *const () as usize,
        comparar_registrar_a_eliminacao as *const () as usize,
        comparar_registrar_a_nao_classificacao as *const () as usize,
        comparar_data_de_conclusao_da_oferta as *const () as usize,
        comparar_primeiro_dia_permitido_da_oferta as *const () as usize,
        comparar_janela_fixa_de_transferencias as *const () as usize,
        comparar_compensacao_pela_pessoa as *const () as usize,
        comparar_pagar_transferencia_e_clausulas as *const () as usize,
        comparar_clube_vendedor_deve_aceitar as *const () as usize,
        comparar_permissao_de_trabalho_na_mudanca as *const () as usize,
        comparar_concluir_oferta_de_contrato_aceita as *const () as usize,
        comparar_oferecer_contrato_ao_jogador_da_oferta as *const () as usize,
        comparar_desfazer_oferta as *const () as usize,
        comparar_fazer_oferta_de_emprestimo as *const () as usize,
        comparar_concluir_transferencias_futuras as *const () as usize,
        comparar_nota_do_doador_de_dados as *const () as usize,
        comparar_ordem_das_aposentadorias as *const () as usize,
        comparar_ordem_da_fila_de_regens as *const () as usize,
        comparar_registro_da_categoria as *const () as usize,
        comparar_criar_estatisticas as *const () as usize,
        comparar_aposentadoria_marcada as *const () as usize,
        comparar_copa_sortear_times_da_rodada as *const () as usize,
        comparar_copa_separar_estadio_compartilhado as *const () as usize,
        comparar_escala_pela_rodada as *const () as usize,
        comparar_lado_do_jogo_para_o_clube as *const () as usize,
        comparar_ordem_dos_coeficientes_das_nacoes as *const () as usize,
        comparar_pedido_de_observacao as *const () as usize,
        comparar_limpar_vaga_da_escalacao as *const () as usize,
        comparar_marcar_batedor_de_bola_parada as *const () as usize,
        comparar_restricoes_da_partida as *const () as usize,
        comparar_limite_do_banco_do_dia as *const () as usize,
        comparar_precisa_de_permissao_de_trabalho as *const () as usize,
        comparar_clube_pode_chegar_ao_jogador as *const () as usize,
        comparar_termos_ao_jogador_da_troca as *const () as usize,
        comparar_fila_livre_do_clube as *const () as usize,
        comparar_transferencia_futura_do_jogador as *const () as usize,
        comparar_tirar_transferencia_futura as *const () as usize,
        comparar_acrescentar_transferencia_futura as *const () as usize,
        comparar_ofertas_abertas_pelos_jogadores_do_clube as *const () as usize,
        comparar_desfazer_ofertas_pelo_jogador as *const () as usize,
        comparar_guardar_percentual_da_proxima_venda as *const () as usize,
        comparar_iniciar_diretorias_dos_clubes as *const () as usize,
        comparar_apelacoes_de_suspensao as *const () as usize,
        comparar_pode_ver_a_partida as *const () as usize,
        comparar_aposentadoria_no_inicio as *const () as usize,
        comparar_aposentadoria_por_idade as *const () as usize,
        comparar_lista_inicial_de_aposentadorias as *const () as usize,
        comparar_novas_aposentadorias as *const () as usize,
        comparar_sair_dos_cargos as *const () as usize,
        comparar_registrar_a_liberacao as *const () as usize,
        comparar_estado_da_lesao as *const () as usize,
        comparar_marcar_decisao_da_oferta as *const () as usize,
        comparar_candidatos_a_tecnico_do_ano as *const () as usize,
        comparar_projetar_posicoes_da_liga as *const () as usize,
        comparar_ordem_da_projecao_pela_reputacao as *const () as usize,
        comparar_ordem_da_projecao_pelos_pontos as *const () as usize,
        comparar_achar_jogos_do_calendario as *const () as usize,
        comparar_jogos_da_competicao_no_dia as *const () as usize,
        comparar_competicao_tem_jogo_na_data as *const () as usize,
        comparar_clube_livre_na_data as *const () as usize,
        comparar_copa_tem_o_clube as *const () as usize,
        comparar_nacao_parceira_do_clube as *const () as usize,
        comparar_pedido_de_transferencia as *const () as usize,
        comparar_cargo_com_contrato as *const () as usize,
        comparar_fora_da_diretoria as *const () as usize,
        comparar_iniciar_permissoes_de_trabalho as *const () as usize,
        comparar_contagem_da_clausula as *const () as usize,
        comparar_premio_por_jogo_do_jogador as *const () as usize,
        comparar_fila_da_transferencia_livre as *const () as usize,
        comparar_proxima_janela_aberta as *const () as usize,
        comparar_emprestimo_negociado as *const () as usize,
        comparar_oferta_com_clausula as *const () as usize,
        comparar_motivo_da_recusa_vale as *const () as usize,
        comparar_motivo_pelo_status_no_elenco as *const () as usize,
        comparar_relacoes_do_elenco as *const () as usize,
        comparar_atrito_com_o_tecnico as *const () as usize,
        comparar_moral_pela_lideranca as *const () as usize,
        comparar_faltas_ao_treino as *const () as usize,
        comparar_competicoes_do_dia as *const () as usize,
        comparar_mudar_confianca_da_diretoria as *const () as usize,
        comparar_estadios_do_dia as *const () as usize,
        comparar_reservar_estadio as *const () as usize,
        comparar_estadio_livre_na_data as *const () as usize,
        comparar_jogo_com_observado as *const () as usize,
        comparar_na_lista_do_olheiro as *const () as usize,
        comparar_registrar_a_primeira_convocacao as *const () as usize,
        comparar_gols_na_carreira as *const () as usize,
        comparar_registrar_o_emprestimo as *const () as usize,
        comparar_ordem_da_volta_aos_treinos as *const () as usize,
        comparar_gols_pelo_clube as *const () as usize,
        comparar_registrar_a_selecao_do_ano as *const () as usize,
        comparar_ordem_pelos_pontos_do_tecnico as *const () as usize,
        comparar_jogo_guardado_igual as *const () as usize,
        comparar_guardar_jogo_no_calendario as *const () as usize,
        comparar_tirar_jogo_do_calendario as *const () as usize,
        comparar_criar_amistosos_planejados as *const () as usize,
        comparar_acrescentar_amistoso as *const () as usize,
        comparar_planejar_amistoso_em_data_livre as *const () as usize,
        comparar_atualizar_copa_no_dia as *const () as usize,
        comparar_montar_rodada_da_copa as *const () as usize,
        comparar_risco_de_doenca_na_viagem as *const () as usize,
        comparar_jogos_da_selecao_no_periodo as *const () as usize,
        comparar_atualizar_ranking_da_fifa as *const () as usize,
        comparar_ordem_dos_resultados_por_pontos as *const () as usize,
        comparar_resultado_no_historico as *const () as usize,
        comparar_faixa_da_selecao as *const () as usize,
        comparar_pontuacoes_do_ranking as *const () as usize,
        comparar_nacao_chave_do_jogo as *const () as usize,
        comparar_e_copa_do_mundo as *const () as usize,
        comparar_e_torneio_internacional as *const () as usize,
        comparar_indice_da_nacao_chave as *const () as usize,
        comparar_ordem_do_ranking as *const () as usize,
        comparar_ordem_das_pontuacoes as *const () as usize,
        comparar_tempo_das_cidades as *const () as usize,
        comparar_ler_regiao_do_clima as *const () as usize,
        comparar_ler_periodo_do_clima as *const () as usize,
        comparar_coeficientes_das_nacoes as *const () as usize,
        comparar_dia_das_aposentadorias as *const () as usize,
        comparar_soma_presa as *const () as usize,
        comparar_nota_do_estadio_neutro as *const () as usize,
        comparar_fator_da_distancia as *const () as usize,
        comparar_ler_o_clima as *const () as usize,
        comparar_fator_do_treinador as *const () as usize,
        comparar_comissao_do_humano as *const () as usize,
        comparar_carga_do_programa as *const () as usize,
        comparar_aplicar_o_progresso as *const () as usize,
        comparar_achar_o_treino_de_posicao as *const () as usize,
        comparar_aprende_depressa as *const () as usize,
        comparar_avancar_a_semana as *const () as usize,
        comparar_tirar_a_ficha_reservada as *const () as usize,
        comparar_lesao_que_volta as *const () as usize,
        comparar_atualizar_amistosos_do_dia as *const () as usize,
        comparar_planejar_pre_temporada_da_ia as *const () as usize,
        comparar_janelas_da_pre_temporada as *const () as usize,
        comparar_marcar_jogos_da_excursao as *const () as usize,
        comparar_ordem_pela_faixa as *const () as usize,
        comparar_ordem_pela_nacao_e_faixa as *const () as usize,
        comparar_posicoes_previstas_da_copa as *const () as usize,
        comparar_ordenar_clubes_pela_forca as *const () as usize,
        comparar_gravar_posicao_do_clube as *const () as usize,
        comparar_linha_da_nacao as *const () as usize,
        comparar_linha_da_nacao_do_clube as *const () as usize,
        comparar_ano_da_temporada_do_clube as *const () as usize,
        comparar_clube_na_temporada as *const () as usize,
        comparar_continente_principal as *const () as usize,
        comparar_nacao_principal as *const () as usize,
        comparar_estatistica_da_categoria as *const () as usize,
        comparar_ordem_dos_idiomas as *const () as usize,
        comparar_ordem_da_historia_por_clube as *const () as usize,
        comparar_clausula_de_rescisao_ativa as *const () as usize,
        comparar_periodo_protegido as *const () as usize,
        comparar_tipo_de_bonus_permitido as *const () as usize,
        comparar_adiar_pelo_clima as *const () as usize,
        comparar_financas_do_clube as *const () as usize,
        comparar_ordem_das_reservas_do_estadio as *const () as usize,
        comparar_clima_da_partida as *const () as usize,
        comparar_temperatura_do_jogo as *const () as usize,
        comparar_gramado_do_jogo as *const () as usize,
        comparar_faixa_de_riqueza as *const () as usize,
        comparar_ordem_por_habilidade as *const () as usize,
        comparar_ordem_pela_capacidade as *const () as usize,
        comparar_programas_padrao as *const () as usize,
        comparar_tipo_do_programa as *const () as usize,
        comparar_programa_do_humano as *const () as usize,
        comparar_fichas_de_treino_iniciais as *const () as usize,
        comparar_guardar_jogador_no_banco as *const () as usize,
        comparar_tem_reserva_de_tecnico as *const () as usize,
        comparar_suspensao_do_vermelho as *const () as usize,
        comparar_nova_temporada_do_orgao as *const () as usize,
        comparar_compactar_disciplina_do_orgao as *const () as usize,
        comparar_coordenada_da_casa as *const () as usize,
        comparar_posicao_repetida_na_formacao as *const () as usize,
        comparar_tipo_de_marcacao as *const () as usize,
        comparar_sortear_ordem_dos_clubes as *const () as usize,
        comparar_clubes_do_lado_do_jogo as *const () as usize,
        comparar_clubes_possiveis_do_jogo as *const () as usize,
        comparar_nota_alvo_do_clube as *const () as usize,
        comparar_jogo_do_historico as *const () as usize,
        comparar_gols_do_historico as *const () as usize,
        comparar_passar_a_oferta_aos_clubes as *const () as usize,
        comparar_noticia_da_oferta_aceita as *const () as usize,
        comparar_noticia_do_emprestimo_oferecido as *const () as usize,
        comparar_recusa_dos_termos as *const () as usize,
        comparar_noticia_da_confianca_da_diretoria as *const () as usize,
        comparar_noticia_das_expectativas_da_diretoria as *const () as usize,
        comparar_ofertas_em_espera_na_janela as *const () as usize,
        comparar_noticia_das_expectativas_da_federacao as *const () as usize,
        comparar_noticia_do_protesto_da_torcida as *const () as usize,
        comparar_guardar_a_negociacao as *const () as usize,
        comparar_noticia_do_boato_do_tecnico as *const () as usize,
        comparar_noticia_da_recusa_em_se_mudar as *const () as usize,
        comparar_receita_do_merchandising as *const () as usize,
        comparar_ingressos_da_temporada as *const () as usize,
        comparar_melhorar_o_centro_de_treino as *const () as usize,
        comparar_eventos_financeiros_da_semana as *const () as usize,
        comparar_diretoria_paga_a_divida as *const () as usize,
        comparar_injecao_de_dinheiro as *const () as usize,
        comparar_teto_salarial_do_pessoal as *const () as usize,
        comparar_pagar_os_bonus as *const () as usize,
        comparar_ajustar_o_teto_salarial as *const () as usize,
        comparar_efeitos_fisicos_da_partida as *const () as usize,
        comparar_batida_do_jogador as *const () as usize,
        comparar_jogador_de_ferias as *const () as usize,
        comparar_contratar_o_regen as *const () as usize,
        comparar_aposentar_pela_lesao as *const () as usize,
        comparar_noticia_da_aposentadoria_anunciada as *const () as usize,
        comparar_fim_da_lesao as *const () as usize,
        comparar_noticia_do_pedido_de_desculpas as *const () as usize,
        comparar_esvaziar_caixa_de_entrada as *const () as usize,
        comparar_tirar_convocado_machucado as *const () as usize,
        comparar_noticia_da_saida_do_convocado as *const () as usize,
        comparar_noticia_do_pedido_de_substituto as *const () as usize,
        comparar_pode_mudar_a_convocacao as *const () as usize,
        comparar_esta_na_convocacao as *const () as usize,
        comparar_marcar_adversario_do_amistoso as *const () as usize,
        comparar_registrar_premio_do_mes as *const () as usize,
        comparar_faixa_de_reputacao_do_amistoso as *const () as usize,
        comparar_folga_do_amistoso as *const () as usize,
        comparar_satisfacao_com_o_contrato as *const () as usize,
        comparar_noticia_quer_contrato_novo as *const () as usize,
        comparar_noticia_pedido_de_transferencia_retirado as *const () as usize,
        comparar_confirmar_oferta_pelo_clube as *const () as usize,
        comparar_tentar_concluir_oferta as *const () as usize,
        comparar_acertar_clausulas_anteriores as *const () as usize,
        comparar_tentar_emprestimo_pelo_jogador as *const () as usize,
        comparar_noticia_contrato_acabando as *const () as usize,
        comparar_noticia_saida_pela_lei_bosman as *const () as usize,
        comparar_acrescentar_oferta_de_contrato as *const () as usize,
        comparar_tirar_oferta_de_contrato_da_vaga as *const () as usize,
        comparar_menor_salario_pedido as *const () as usize,
        comparar_guardar_notas_da_partida as *const () as usize,
        comparar_nota_do_jogador_no_mes as *const () as usize,
        comparar_melhor_do_mes as *const () as usize,
        comparar_premiar_o_melhor_do_mes as *const () as usize,
        comparar_gravar_vencedor_do_mes as *const () as usize,
        comparar_reputacao_do_premiado as *const () as usize,
        comparar_fisico_do_dia as *const () as usize,
        comparar_faixa_da_confianca_na_liga as *const () as usize,
        comparar_gravar_notas_da_diretoria as *const () as usize,
        comparar_dia_das_lesoes as *const () as usize,
        comparar_noticias_de_volta_aos_treinos as *const () as usize,
        comparar_um_cartao_da_suspensao as *const () as usize,
        comparar_fila_de_regens as *const () as usize,
        comparar_veiculo_pelo_clube as *const () as usize,
        comparar_veiculo_pela_pessoa as *const () as usize,
        comparar_somar_resultado_na_liga as *const () as usize,
        comparar_copiar_tabela_da_liga as *const () as usize,
        comparar_posicoes_previstas_da_liga as *const () as usize,
        comparar_resultado_entre_selecoes as *const () as usize,
        comparar_nome_do_elenco_da_selecao as *const () as usize,
        comparar_rodada_com_jogos_conhecidos as *const () as usize,
        comparar_registrar_convocados_por_clube as *const () as usize,
        comparar_importancia_do_jogo_na_competicao as *const () as usize,
        comparar_fase_da_competicao as *const () as usize,
        comparar_faltas_do_dia as *const () as usize,
        comparar_teve_falta_ao_treino as *const () as usize,
        comparar_teve_incidente_de_forma as *const () as usize,
        comparar_marcar_falta_ao_treino as *const () as usize,
        comparar_fim_da_ausencia as *const () as usize,
        comparar_marcas_do_contrato as *const () as usize,
        comparar_recusar_oferta_de_transferencia as *const () as usize,
        comparar_retirar_oferta_de_transferencia as *const () as usize,
        comparar_noticia_oferta_recusada as *const () as usize,
        comparar_desassossegar_pelo_lance as *const () as usize,
        comparar_noticia_oferta_desfeita as *const () as usize,
        comparar_noticia_premio_por_jogos as *const () as usize,
        comparar_efetivar_transferencia as *const () as usize,
        comparar_noticia_transferencia_concluida as *const () as usize,
        comparar_noticia_transferencia_acertada as *const () as usize,
        comparar_emprestimo_da_configuracao as *const () as usize,
        comparar_noticia_fim_do_contrato as *const () as usize,
        comparar_noticia_pedido_de_adiamento as *const () as usize,
        comparar_tatica_do_clube as *const () as usize,
        comparar_escolher_vaga_de_tatica as *const () as usize,
        comparar_tatica_ativa_do_humano as *const () as usize,
        comparar_tirar_da_escalacao as *const () as usize,
        comparar_trocar_vagas_da_escalacao as *const () as usize,
        comparar_tem_copia_da_escalacao as *const () as usize,
        comparar_noticia_da_pressao as *const () as usize,
        comparar_noticia_da_saida_do_tecnico as *const () as usize,
        comparar_noticia_do_tecnico_novo as *const () as usize,
        comparar_pagar_premios_da_liga as *const () as usize,
        comparar_registros_pela_situacao_na_liga as *const () as usize,
        comparar_anunciar_situacao_na_liga as *const () as usize,
        comparar_registrar_partida_nas_estatisticas as *const () as usize,
        comparar_recordes_de_posicao_na_liga as *const () as usize,
        comparar_noticias_de_recordes as *const () as usize,
        comparar_convocar_selecoes_para_o_jogo as *const () as usize,
        comparar_noticia_do_convocado as *const () as usize,
        comparar_noticia_do_desfecho_na_copa as *const () as usize,
        comparar_contar_vagas_com_lance as *const () as usize,
        comparar_tirar_da_lista_de_transferencias as *const () as usize,
        comparar_tirar_da_lista_de_emprestimos as *const () as usize,
        comparar_situacao_na_lista_de_observacao as *const () as usize,
        comparar_por_na_lista_de_emprestimos as *const () as usize,
        comparar_teto_de_preco_pelo_estatuto as *const () as usize,
        comparar_distribuir_verba_de_transferencias as *const () as usize,
        comparar_estatuto_permitido as *const () as usize,
        comparar_nome_da_lista_do_olheiro as *const () as usize,
        comparar_amistosos_internacionais_do_dia as *const () as usize,
        comparar_lista_escolhida_do_tecnico as *const () as usize,
        comparar_escolher_lista_do_tecnico as *const () as usize,
        comparar_inscritos_da_liga_dos_campeoes as *const () as usize,
        comparar_tabela_da_qualificacao_da_liga_dos_campeoes as *const () as usize,
        comparar_proxima_fase_da_liga_dos_campeoes as *const () as usize,
        comparar_processar_jogo_da_liga_dos_campeoes as *const () as usize,
        comparar_progresso_do_clube_na_liga_dos_campeoes as *const () as usize,
        comparar_conta_para_conflito_de_datas as *const () as usize,
        comparar_escolher_arbitro_da_partida as *const () as usize,
        comparar_limpar_arbitros_ocupados as *const () as usize,
        comparar_renovar_arbitros as *const () as usize,
        comparar_arbitros_sorteados as *const () as usize,
        comparar_validar_arbitros as *const () as usize,
        comparar_copiar_ficha_de_arbitro as *const () as usize,
        comparar_registrar_premio_do_ano as *const () as usize,
        comparar_vitoria_maior as *const () as usize,
        comparar_derrota_maior as *const () as usize,
        comparar_jogos_das_permissoes_e_clausulas as *const () as usize,
        comparar_guardar_premio_por_jogos_na_selecao as *const () as usize,
        comparar_noticia_transferencia_finalizada as *const () as usize,
        comparar_veto_dos_fisios_e_da_diretoria as *const () as usize,
        comparar_noticia_do_ultimato as *const () as usize,
        comparar_texto_do_ultimato as *const () as usize,
        comparar_liberar_sem_custo as *const () as usize,
        comparar_ferias_do_humano as *const () as usize,
        comparar_noticia_conversa_de_contrato as *const () as usize,
        comparar_mata_mata_da_first as *const () as usize,
        comparar_mata_mata_da_second as *const () as usize,
        comparar_mata_mata_da_third as *const () as usize,
        comparar_registros_pela_situacao_inglesa as *const () as usize,
        comparar_situacao_na_first as *const () as usize,
        comparar_situacao_na_third as *const () as usize,
        comparar_posicoes_e_situacao_na_liga as *const () as usize,
        comparar_fase_seguinte_da_first as *const () as usize,
        comparar_fase_seguinte_da_second as *const () as usize,
        comparar_registros_da_liga_dos_campeoes as *const () as usize,
        comparar_sorteio_das_quartas as *const () as usize,
        comparar_clubes_da_liga_dos_campeoes as *const () as usize,
        comparar_registrar_resultado_da_tatica as *const () as usize,
        comparar_capitao_guardado as *const () as usize,
        comparar_escolher_capitao as *const () as usize,
        comparar_selecao_que_tem_o_jogador as *const () as usize,
        comparar_jogador_serve_a_selecao as *const () as usize,
        comparar_juntar_competicoes_das_convocacoes as *const () as usize,
        comparar_registro_da_partida_pelo_clube as *const () as usize,
        comparar_referencia_ao_nome as *const () as usize,
        comparar_codigo_do_resultado as *const () as usize,
        comparar_mover_jogos_vizinhos_do_clube as *const () as usize,
        comparar_remarcar_jogos_perto_da_data as *const () as usize,
        comparar_noticia_do_jogo_remarcado as *const () as usize,
        comparar_incidente_de_forma as *const () as usize,
        comparar_noticia_reacao_a_multa as *const () as usize,
        comparar_noticia_retorno_do_treinador as *const () as usize,
        comparar_sortear_comentarista as *const () as usize,
        comparar_nome_do_veiculo as *const () as usize,
        comparar_noticia_queixa_a_federacao as *const () as usize,
        comparar_folha_salarial_do_clube as *const () as usize,
        comparar_refazer_todas_as_folhas as *const () as usize,
        comparar_dias_ate_a_temporada as *const () as usize,
        comparar_receber_venda_de_jogador as *const () as usize,
        comparar_pagar_compra_de_jogador as *const () as usize,
        comparar_pagar_luvas as *const () as usize,
        comparar_receber_premio_em_dinheiro as *const () as usize,
        comparar_receber_direitos_de_tv as *const () as usize,
        comparar_pagar_salarios as *const () as usize,
        comparar_copa_premio_da_rodada as *const () as usize,
        comparar_copa_mandantes_da_rodada as *const () as usize,
        comparar_copa_progresso_do_time as *const () as usize,
        comparar_amistoso_responder_pedido as *const () as usize,
        comparar_amistoso_enviar_pedido as *const () as usize,
        comparar_amistoso_lista_das_nacoes as *const () as usize,
        comparar_copa_sortear_rodada as *const () as usize,
        comparar_nome_da_fase_e_rodada as *const () as usize,
        comparar_clube_da_divisao_mais_alta as *const () as usize,
        comparar_copa_registros_pelo_desfecho as *const () as usize,
        comparar_copa_honras_da_temporada as *const () as usize,
        comparar_copa_semifinalistas_para_a_uefa as *const () as usize,
        comparar_recusar_oferta_de_emprestimo as *const () as usize,
        comparar_vaga_no_elenco_para_a_oferta as *const () as usize,
        comparar_percentual_da_proxima_venda_achado as *const () as usize,
        comparar_jogador_na_busca as *const () as usize,
        comparar_sincronizar_listas_dos_olheiros as *const () as usize,
        comparar_vaga_do_jogador_observado as *const () as usize,
        comparar_valor_do_jogador_para_o_clube as *const () as usize,
        comparar_entrada_do_jogador_na_busca_do_clube as *const () as usize,
        comparar_premio_de_jogos_na_selecao_achado as *const () as usize,
        comparar_contratos_depois_do_jogo as *const () as usize,
        comparar_moral_da_ficha as *const () as usize,
        comparar_texto_da_moral as *const () as usize,
        comparar_valor_do_jogador as *const () as usize,
        comparar_diferenca_de_moral_em_texto as *const () as usize,
        comparar_marcas_de_elenco_para_a_tela as *const () as usize,
        comparar_jogadores_que_a_diretoria_cita as *const () as usize,
        comparar_veredito_da_nota_na_partida as *const () as usize,
        comparar_texto_da_diretoria_sobre_os_jogadores as *const () as usize,
        comparar_resultado_nos_registros_dos_tecnicos as *const () as usize,
        comparar_posicoes_no_fim_da_liga as *const () as usize,
        comparar_jogador_do_ano_da_torcida_no_registro as *const () as usize,
        comparar_vice_da_copa_nos_registros as *const () as usize,
        comparar_terceiro_da_copa_nos_registros as *const () as usize,
        comparar_troca_de_tecnico_nos_registros as *const () as usize,
        comparar_reacoes_dos_elencos_a_transferencia as *const () as usize,
        comparar_noticia_da_comemoracao_da_contratacao as *const () as usize,
        comparar_noticia_do_companheiro_desassossegado as *const () as usize,
        comparar_renda_da_partida as *const () as usize,
        comparar_renda_do_estadio as *const () as usize,
        comparar_pagar_multa_da_liga as *const () as usize,
        comparar_impulso_das_financas as *const () as usize,
        comparar_troca_do_presidente as *const () as usize,
        comparar_pagar_outros_custos as *const () as usize,
        comparar_tabela_das_datas_internacionais as *const () as usize,
        comparar_resolver_datas_internacionais as *const () as usize,
        comparar_torneio_do_clube_humano as *const () as usize,
        comparar_amistosos_dos_clubes as *const () as usize,
        comparar_acrescentar_torneio as *const () as usize,
        comparar_convite_aberto_do_clube_humano as *const () as usize,
        comparar_lesao_nova as *const () as usize,
        comparar_noticia_da_lesao as *const () as usize,
        comparar_ausente_sem_licenca as *const () as usize,
        comparar_parte_dos_clubes_anteriores as *const () as usize,
        comparar_percentual_da_proxima_venda_pago as *const () as usize,
        comparar_noticia_do_percentual_da_venda as *const () as usize,
        comparar_permissao_concedida as *const () as usize,
        comparar_estatuto_pelo_motivo as *const () as usize,
        comparar_vaga_da_negociacao as *const () as usize,
        comparar_reacao_do_elenco_a_historia as *const () as usize,
        comparar_aviso_ao_rival_de_posicao as *const () as usize,
        comparar_rotulo_da_resposta_da_midia as *const () as usize,
        comparar_cumprir_suspensoes_internacionais as *const () as usize,
        comparar_recurso_da_suspensao as *const () as usize,
        comparar_texto_da_vaga_da_suspensao as *const () as usize,
        comparar_tirar_da_lista as *const () as usize,
        comparar_esvaziar_lista as *const () as usize,
        comparar_item_do_cursor as *const () as usize,
        comparar_copa_sortear_pela_semente as *const () as usize,
        comparar_copa_sortear_separando_nacoes as *const () as usize,
        comparar_copa_comeco_da_temporada as *const () as usize,
        comparar_nacao_chave_do_clube as *const () as usize,
        comparar_nacao_chave_do_estadio as *const () as usize,
        comparar_entrada_da_nacao_chave_ativa as *const () as usize,
        comparar_conflito_de_jogos_no_mesmo_dia as *const () as usize,
        comparar_primeiro_ano_com_jogos as *const () as usize,
        comparar_ultimo_ano_com_jogos as *const () as usize,
        comparar_gravar_financas as *const () as usize,
        comparar_carregar_financas as *const () as usize,
        comparar_gravar_gerente_de_elencos as *const () as usize,
        comparar_gravar_sementes_da_uefa as *const () as usize,
        comparar_gravar_sementes_da_conmebol as *const () as usize,
        comparar_gravar_paises_anfitrioes as *const () as usize,
        comparar_gravar_historico_dos_clubes as *const () as usize,
        comparar_gravar_estatisticas_dos_arbitros as *const () as usize,
        comparar_gravar_estatisticas_simuladas as *const () as usize,
        comparar_gravar_datas_dos_clubes as *const () as usize,
        comparar_carregar_datas_dos_clubes as *const () as usize,
        comparar_gravar_conferencia_das_pernas as *const () as usize,
        comparar_fim_de_temporada_da_nacao as *const () as usize,
        comparar_fim_de_temporada_do_clube as *const () as usize,
        comparar_noticia_do_dividendo as *const () as usize,
        comparar_teto_salarial as *const () as usize,
        comparar_noticia_das_obras as *const () as usize,
        comparar_noticia_da_obra_recusada as *const () as usize,
        comparar_conflitos_do_dia as *const () as usize,
        comparar_conflitos_do_jogo as *const () as usize,
        comparar_carregar_conferencia_das_pernas as *const () as usize,
        comparar_nova_data_do_jogo as *const () as usize,
        comparar_jogo_no_calendario as *const () as usize,
        comparar_ano_do_calendario as *const () as usize,
        comparar_copa_criar_jogos_do_confronto as *const () as usize,
        comparar_copa_noticia_do_confronto_pendente as *const () as usize,
        comparar_janelas_avisos_do_dia as *const () as usize,
        comparar_copa_levar_jogos_as_datas as *const () as usize,
        comparar_tirar_jogador_da_troca as *const () as usize,
        comparar_permissoes_da_oferta as *const () as usize,
        comparar_confianca_depois_da_partida as *const () as usize,
        comparar_noticia_da_reacao_da_diretoria as *const () as usize,
        comparar_noticia_da_avaliacao_da_federacao as *const () as usize,
        comparar_expectativas_da_temporada as *const () as usize,
        comparar_vaga_de_tecnico as *const () as usize,
        comparar_noticia_da_oferta_de_emprego as *const () as usize,
        comparar_boatos_de_transferencia as *const () as usize,
        comparar_noticia_nao_vende as *const () as usize,
        comparar_noticia_do_titulo as *const () as usize,
        comparar_amistosos_entre_selecoes as *const () as usize,
        comparar_convite_de_amistoso_valido as *const () as usize,
        comparar_convite_de_torneio_valido as *const () as usize,
        comparar_dia_das_selecoes as *const () as usize,
        comparar_busca_atual_sem_clube as *const () as usize,
        comparar_clube_nas_buscas_da_sessao as *const () as usize,
        comparar_escolher_elenco_da_selecao as *const () as usize,
        comparar_olheiros_na_partida as *const () as usize,
        comparar_sessao_de_edicao as *const () as usize,
        comparar_escolher_sessao_de_edicao as *const () as usize,
        comparar_dia_das_transferencias as *const () as usize,
        comparar_processar_oferta_vencida as *const () as usize,
        comparar_restricoes_da_oferta as *const () as usize,
        comparar_aceitar_lance as *const () as usize,
        comparar_noticia_da_oferta_feita as *const () as usize,
        comparar_pode_emprestar as *const () as usize,
        comparar_liga_dia as *const () as usize,
        comparar_liga_criar_tabela as *const () as usize,
        comparar_liga_promover_e_rebaixar as *const () as usize,
        comparar_calendario_dia as *const () as usize,
        comparar_calendario_lista as *const () as usize,
        comparar_calendario_carregar_ano as *const () as usize,
        comparar_copa_arvore_dos_confrontos as *const () as usize,
        comparar_competicao_primeira_fase_do_grupo as *const () as usize,
        comparar_competicao_fase_atual as *const () as usize,
        comparar_atualizar_o_treino_do_jogador as *const () as usize,
        comparar_programa_de_treino_do_jogador as *const () as usize,
        comparar_nivel_da_categoria_do_programa as *const () as usize,
        comparar_tirar_ou_devolver_o_progresso as *const () as usize,
        comparar_mudanca_do_atributo_pelo_treino as *const () as usize,
        comparar_aptidao_para_a_posicao as *const () as usize,
        comparar_aposentadorias_do_dia as *const () as usize,
        comparar_noticia_da_aposentadoria as *const () as usize,
        comparar_noticia_do_jovem_promovido as *const () as usize,
        comparar_limpar_ausencia as *const () as usize,
        comparar_limpar_falta_ao_treino as *const () as usize,
        comparar_noticia_da_falta_ao_treino as *const () as usize,
        comparar_troca_com_compensacao as *const () as usize,
        comparar_ordem_das_vagas as *const () as usize,
        comparar_marcar_sob_oferta as *const () as usize,
        comparar_campeao_da_liga_nos_registros as *const () as usize,
        comparar_campeao_da_copa_nos_registros as *const () as usize,
        comparar_limpar_registro_do_humano as *const () as usize,
        comparar_resposta_da_ia_ao_lance as *const () as usize,
        comparar_negociacao_de_emprestimo as *const () as usize,
        comparar_opcao_de_troca as *const () as usize,
        comparar_agendar_fim_do_emprestimo as *const () as usize,
        comparar_lances_do_clube as *const () as usize,
        comparar_cqe4_interesse as *const () as usize,
        comparar_cqe4_construir_contrato as *const () as usize,
        comparar_cqe5_pode_emprestar as *const () as usize,
        comparar_cqe5_registrar_emprestimo as *const () as usize,
        comparar_cqe5_dia_das_regras as *const () as usize,
        comparar_cqe6_periodo_inglaterra as *const () as usize,
        comparar_cqe6_pode_emprestar_escocia as *const () as usize,
        comparar_cqe6_periodo_escocia as *const () as usize,
        comparar_cqe6_pode_emprestar_italia as *const () as usize,
        comparar_cqe6_pode_emprestar_irlanda as *const () as usize,
        comparar_cqe6_pode_emprestar_espanha as *const () as usize,
        comparar_liga_jogos_para_a_tv as *const () as usize,
        comparar_liga_noticia_dos_grupos as *const () as usize,
        comparar_liga_noticia_da_tabela as *const () as usize,
        comparar_por_oferta_de_transferencia as *const () as usize,
        comparar_aceitar_emprestimo as *const () as usize,
        comparar_termos_aceitos as *const () as usize,
        comparar_noticia_de_jogador_listado as *const () as usize,
        comparar_contrato_oferecido as *const () as usize,
        comparar_mundial_sortear_grupos as *const () as usize,
        comparar_mundial_sorteio_dos_grupos as *const () as usize,
        comparar_mundial_dia as *const () as usize,
        comparar_mundial_tabela_do_grupo_um as *const () as usize,
        comparar_mundial_fase_do_grupo as *const () as usize,
        comparar_mundial_posicoes_previstas as *const () as usize,
        comparar_mundial_rodadas_da_fase as *const () as usize,
        comparar_mundial_nacao_classificada as *const () as usize,
        comparar_mundial_cabecas_de_chave as *const () as usize,
        comparar_distribuir_os_treinadores as *const () as usize,
        comparar_trocar_os_programas_do_elenco as *const () as usize,
        comparar_nomes_padrao_dos_programas as *const () as usize,
        comparar_historico_da_carreira as *const () as usize,
        comparar_anos_desde_o_clube as *const () as usize,
        comparar_ordem_do_historico_por_pessoa as *const () as usize,
        comparar_lesao_pelo_nome as *const () as usize,
        comparar_noticia_da_volta_aos_treinos as *const () as usize,
        comparar_nome_da_categoria_da_lesao as *const () as usize,
        comparar_cqf10_olheiro_no_jogo as *const () as usize,
        comparar_cqf10_historico_do_clube_observado as *const () as usize,
        comparar_cqf11_relatorio_da_selecao as *const () as usize,
        comparar_cqf11_noticia_da_convocacao as *const () as usize,
        comparar_cqf11_adversarios_da_selecao as *const () as usize,
        comparar_cqf12_olheiros_do_dia as *const () as usize,
        comparar_cqf12_relatorios_dos_olheiros as *const () as usize,
        comparar_cqf12_noticia_do_relatorio as *const () as usize,
        comparar_assinar_contrato as *const () as usize,
        comparar_refazer_como_jovem as *const () as usize,
        comparar_tirar_da_disputa as *const () as usize,
        comparar_ler_formacao_do_disco as *const () as usize,
        comparar_trocar_bytes_das_palavras as *const () as usize,
        comparar_trocar_bytes_dos_dwords as *const () as usize,
        comparar_montar_os_arbitros as *const () as usize,
        comparar_ordem_das_vagas_de_competicao as *const () as usize,
        comparar_cqg10_amistoso_cancelado as *const () as usize,
        comparar_cqg10_jogo_da_selecao_cancelado as *const () as usize,
        comparar_cqg10_torneio_cancelado as *const () as usize,
        comparar_ordem_dos_arbitros_decrescente as *const () as usize,
        comparar_ordem_dos_arbitros_crescente as *const () as usize,
        comparar_arquivar_estatisticas as *const () as usize,
        comparar_pessoa_pelo_nome as *const () as usize,
        comparar_texto_entre_aspas as *const () as usize,
        comparar_tipo_da_linha as *const () as usize,
        comparar_campeoes_sortear_grupos as *const () as usize,
        comparar_campeoes_ordem_dos_potes as *const () as usize,
        comparar_campeoes_posicoes_previstas as *const () as usize,
        comparar_campeoes_sortear_segunda_fase as *const () as usize,
        comparar_campeoes_ordem_da_segunda_fase as *const () as usize,
        comparar_grupo_da_fase as *const () as usize,
        comparar_campeoes_rodadas_da_fase as *const () as usize,
        comparar_liga_expectativas_da_temporada as *const () as usize,
        comparar_liga_rebaixar_clube as *const () as usize,
        comparar_competicao_todas_completas as *const () as usize,
        comparar_competicao_lista_de_clubes as *const () as usize,
        comparar_competicao_progresso_do_clube as *const () as usize,
        comparar_uefa_rodar_coeficientes as *const () as usize,
        comparar_uefa_vagas_por_nacao as *const () as usize,
        comparar_uefa_enesimo_clube_da_nacao as *const () as usize,
        comparar_uefa_juntar_clubes_europeus as *const () as usize,
        comparar_uefa_vagas_liga_dos_campeoes as *const () as usize,
        comparar_uefa_vagas_intertoto as *const () as usize,
        comparar_uefa_vagas_copa_da_uefa as *const () as usize,
        comparar_uefa_voluntario_ingles as *const () as usize,
        comparar_uefa_virada_da_classificacao as *const () as usize,
        comparar_conmebol_vagas_libertadores as *const () as usize,
        comparar_conmebol_enesimo_clube_da_nacao as *const () as usize,
        comparar_conmebol_convidado_da_mercosul as *const () as usize,
        comparar_conmebol_juntar_clubes as *const () as usize,
        comparar_conmebol_virada_do_ano as *const () as usize,
        comparar_conmebol_campeao_dos_campeoes as *const () as usize,
        comparar_sede_da_edicao as *const () as usize,
        comparar_proxima_sede as *const () as usize,
        comparar_calendario_das_sedes as *const () as usize,
        comparar_regras_inglaterra as *const () as usize,
        comparar_regras_eua as *const () as usize,
        comparar_regras_australia as *const () as usize,
        comparar_janelas_espanha as *const () as usize,
        comparar_janelas_italia as *const () as usize,
        comparar_janelas_franca as *const () as usize,
        comparar_regras_brasil as *const () as usize,
        comparar_janelas_alemanha as *const () as usize,
        comparar_regras_escocia as *const () as usize,
        comparar_cqi10_intertoto_dia as *const () as usize,
        comparar_cqi10_intertoto_registros as *const () as usize,
        comparar_cqi10_uefa_registros as *const () as usize,
        comparar_cqi11_concacaf_classificados as *const () as usize,
        comparar_cqi11_concacaf_dia as *const () as usize,
        comparar_cqi11_concacaf_rodadas as *const () as usize,
        comparar_cqi12_copa_italia_regras as *const () as usize,
        comparar_cqi12_copa_c_regras as *const () as usize,
        comparar_cqi12_copa_dos_campeoes_regras as *const () as usize,
        comparar_cqj10_copa_africana_registros as *const () as usize,
        comparar_cqj10_copa_america_registros as *const () as usize,
        comparar_cqj10_eliminatoria_oceania_registros as *const () as usize,
        comparar_cqj11_libertadores_regras as *const () as usize,
        comparar_cqj11_merconorte_regras as *const () as usize,
        comparar_cqj11_oceania_clubes_regras as *const () as usize,
        comparar_cqj12_eliminatoria_europa_regras as *const () as usize,
        comparar_cqj12_eliminatoria_sul_americana_regras as *const () as usize,
        comparar_cqj12_eliminatoria_asia_regras as *const () as usize,
        comparar_cqk10_copa_america_regras as *const () as usize,
        comparar_cqk10_copa_africana_regras as *const () as usize,
        comparar_cqk10_eliminatoria_oceania_regras as *const () as usize,
        comparar_cqk11_libertadores_classificados as *const () as usize,
        comparar_cqk11_libertadores_registros as *const () as usize,
        comparar_cqk11_libertadores_fase_seguinte as *const () as usize,
        comparar_cqk12_africana_fase_seguinte as *const () as usize,
        comparar_cqk12_africana_grupo_a as *const () as usize,
        comparar_cqk12_africana_historico as *const () as usize,
        comparar_cqi7_regras_da_serie_a as *const () as usize,
        comparar_cqi7_regras_da_serie_b as *const () as usize,
        comparar_cqi7_regras_da_serie_c as *const () as usize,
        comparar_cqi8_regras_do_centro_oeste as *const () as usize,
        comparar_cqi8_regras_do_gaucho as *const () as usize,
        comparar_cqi8_regras_do_catarinense as *const () as usize,
        comparar_cqi9_regras_do_paranaense as *const () as usize,
        comparar_cqi9_regras_do_pernambucano as *const () as usize,
        comparar_cqi9_regras_do_carioca as *const () as usize,
        comparar_cqj7_regras_do_goiano as *const () as usize,
        comparar_cqj7_regras_da_j1 as *const () as usize,
        comparar_cqj7_regras_da_j2 as *const () as usize,
        comparar_cqj8_regras_da_nsl as *const () as usize,
        comparar_cqj8_regras_da_mls as *const () as usize,
        comparar_cqj8_historico_do_estadual as *const () as usize,
        comparar_cqj8_historico_do_primeiro_estagio as *const () as usize,
        comparar_cqj9_destino_na_serie_a as *const () as usize,
        comparar_cqj9_destino_na_serie_c as *const () as usize,
        comparar_cqj9_destino_no_baiano as *const () as usize,
        comparar_cqk7_destino_no_centro_oeste as *const () as usize,
        comparar_cqk7_destino_no_mineiro as *const () as usize,
        comparar_cqk7_destino_na_nsl as *const () as usize,
        comparar_cqk8_progresso_no_mineiro as *const () as usize,
        comparar_cqk8_progresso_na_mls as *const () as usize,
        comparar_cqk8_progresso_na_serie_c as *const () as usize,
        comparar_cqk9_destino_na_mls as *const () as usize,
        comparar_cqk9_resultado_na_mls as *const () as usize,
        comparar_cqk9_conferencia_das_estrelas as *const () as usize,
        comparar_regras_da_premier as *const () as usize,
        comparar_regras_da_first as *const () as usize,
        comparar_regras_da_second as *const () as usize,
        comparar_regras_da_third as *const () as usize,
        comparar_tabela_da_premier as *const () as usize,
        comparar_tabela_da_second as *const () as usize,
        comparar_tabela_da_third as *const () as usize,
        comparar_tabela_da_first as *const () as usize,
        comparar_regras_da_serie_a as *const () as usize,
        comparar_regras_da_serie_b as *const () as usize,
        comparar_regras_da_c1a as *const () as usize,
        comparar_regras_da_c1b as *const () as usize,
        comparar_tabela_da_serie_a as *const () as usize,
        comparar_tabela_da_serie_b as *const () as usize,
        comparar_tabela_da_c1a as *const () as usize,
        comparar_tabela_da_c1b as *const () as usize,
        comparar_regras_da_franca_1 as *const () as usize,
        comparar_regras_da_franca_2 as *const () as usize,
        comparar_regras_da_franca_3 as *const () as usize,
        comparar_tabela_da_franca_1 as *const () as usize,
        comparar_tabela_da_franca_2 as *const () as usize,
        comparar_tabela_da_franca_3 as *const () as usize,
        comparar_regras_da_primera as *const () as usize,
        comparar_regras_da_segunda as *const () as usize,
        comparar_regras_da_divisao_de_baixo as *const () as usize,
        comparar_tabela_da_primera as *const () as usize,
        comparar_registros_pela_situacao_na_c2a as *const () as usize,
        comparar_situacao_na_primera as *const () as usize,
        comparar_tabela_do_grupo_da_cfa as *const () as usize,
        comparar_tabela_da_divisao_de_baixo_francesa as *const () as usize,
        comparar_situacao_na_c1a as *const () as usize,
        comparar_situacao_na_c1b as *const () as usize,
        comparar_cql10_copa_da_liga_tabela as *const () as usize,
        comparar_cql10_copa_da_inglaterra_tabela as *const () as usize,
        comparar_cql10_supercopa_tabela as *const () as usize,
        comparar_cql11_copa_da_franca_tabela as *const () as usize,
        comparar_cql11_copa_da_liga_francesa_tabela as *const () as usize,
        comparar_cql11_copa_do_rei_tabela as *const () as usize,
        comparar_cql12_supercopa_da_franca_tabela as *const () as usize,
        comparar_cql12_supercopa_da_espanha_tabela as *const () as usize,
        comparar_cql12_supercopa_do_japao_tabela as *const () as usize,
        comparar_cqm10_copa_da_liga_inscritos as *const () as usize,
        comparar_cqm10_trofeu_inscritos as *const () as usize,
        comparar_cqm10_supercopa_clubes as *const () as usize,
        comparar_cqm11_copa_da_franca_inscritos as *const () as usize,
        comparar_cqm11_copa_da_liga_inscritos as *const () as usize,
        comparar_cqm11_supercopa_clubes as *const () as usize,
        comparar_cqm12_supercopa_da_espanha_clubes as *const () as usize,
        comparar_cqm12_supercopa_do_japao_clubes as *const () as usize,
        comparar_cqm12_jogo_das_estrelas_tabela as *const () as usize,
        comparar_cqn10_copa_da_liga_dia as *const () as usize,
        comparar_cqn10_copa_da_inglaterra_dia as *const () as usize,
        comparar_cqn10_jogo_das_estrelas_dia as *const () as usize,
        comparar_cqn11_copa_da_liga_japonesa_tabela as *const () as usize,
        comparar_cqn11_copa_do_imperador_tabela as *const () as usize,
        comparar_cqn11_copa_dos_eua_tabela as *const () as usize,
        comparar_cqn12_copa_da_liga_inscritos as *const () as usize,
        comparar_cqn12_copa_dos_eua_inscritos as *const () as usize,
        comparar_cqn12_copa_do_imperador_inscritos as *const () as usize,
        comparar_numero_de_camisa as *const () as usize,
        comparar_clube_usa_numeros as *const () as usize,
        comparar_gravar_numero_de_camisa as *const () as usize,
        comparar_dar_numero_de_camisa as *const () as usize,
        comparar_numerar_elenco as *const () as usize,
        comparar_numerar_no_comeco as *const () as usize,
        comparar_numerar_liga as *const () as usize,
        comparar_inscrever_na_europa as *const () as usize,
        comparar_regras_holanda as *const () as usize,
        comparar_regras_croacia as *const () as usize,
        comparar_regras_irlanda as *const () as usize,
        comparar_regras_noruega as *const () as usize,
        comparar_regras_irlanda_do_norte as *const () as usize,
        comparar_regras_pais_de_gales as *const () as usize,
        comparar_regras_turquia as *const () as usize,
        comparar_regras_polonia as *const () as usize,
        comparar_regras_finlandia as *const () as usize,
        comparar_janelas_argentina as *const () as usize,
        comparar_janelas_portugal as *const () as usize,
        comparar_regras_coreia as *const () as usize,
        comparar_janelas_grecia as *const () as usize,
        comparar_janelas_russia as *const () as usize,
        comparar_janelas_belgica as *const () as usize,
        comparar_inscricao_europeia as *const () as usize,
        comparar_ranking_fifa_inicial as *const () as usize,
        comparar_lista_de_estadios_neutros as *const () as usize,
        comparar_cql7_tabela_da_serie_a as *const () as usize,
        comparar_cql7_tabela_da_serie_b as *const () as usize,
        comparar_cql7_tabela_da_serie_c as *const () as usize,
        comparar_cql8_tabela_do_baiano as *const () as usize,
        comparar_cql8_tabela_do_centro_oeste as *const () as usize,
        comparar_cql8_tabela_do_gaucho as *const () as usize,
        comparar_cql9_tabela_do_goiano as *const () as usize,
        comparar_cql9_tabela_do_mineiro as *const () as usize,
        comparar_cql9_tabela_do_norte as *const () as usize,
        comparar_cqm7_tabela_do_nordeste as *const () as usize,
        comparar_cqm7_tabela_do_paranaense as *const () as usize,
        comparar_cqm7_tabela_do_pernambucano as *const () as usize,
        comparar_cqm8_tabela_do_carioca as *const () as usize,
        comparar_cqm8_tabela_do_catarinense as *const () as usize,
        comparar_cqm8_tabela_do_paulista as *const () as usize,
        comparar_cqm9_tabela_da_j1 as *const () as usize,
        comparar_cqm9_tabela_da_j2 as *const () as usize,
        comparar_cqm9_tabela_da_nsl as *const () as usize,
        comparar_cqn7_mata_mata_do_baiano as *const () as usize,
        comparar_cqn7_mata_mata_do_norte as *const () as usize,
        comparar_cqn7_mata_mata_do_nordeste as *const () as usize,
        comparar_cqn8_mata_mata_do_pernambucano as *const () as usize,
        comparar_cqn8_mata_mata_do_catarinense as *const () as usize,
        comparar_cqn8_mata_mata_do_paulista as *const () as usize,
        comparar_cqn9_mata_mata_do_gaucho as *const () as usize,
        comparar_cqn9_mata_mata_do_centro_oeste as *const () as usize,
        comparar_cqn9_mata_mata_do_goiano as *const () as usize,
        comparar_desempate_da_serie_a as *const () as usize,
        comparar_desempate_da_serie_b as *const () as usize,
        comparar_ordem_dos_empatados_na_italia as *const () as usize,
        comparar_resultado_do_desempate_da_serie_a as *const () as usize,
        comparar_fase_seguinte_da_c1a as *const () as usize,
        comparar_fases_do_fim_da_c1a as *const () as usize,
        comparar_fase_seguinte_da_c1b as *const () as usize,
        comparar_fases_do_fim_da_c1b as *const () as usize,
        comparar_fase_seguinte_da_third as *const () as usize,
        comparar_construir_premier as *const () as usize,
        comparar_construir_first as *const () as usize,
        comparar_construir_second as *const () as usize,
        comparar_construir_third as *const () as usize,
        comparar_construir_serie_a as *const () as usize,
        comparar_construir_serie_b as *const () as usize,
        comparar_construir_c1a as *const () as usize,
        comparar_construir_c1b as *const () as usize,
        comparar_construir_franca_2 as *const () as usize,
        comparar_construir_franca_3 as *const () as usize,
        comparar_construir_segunda as *const () as usize,
        comparar_construir_primera as *const () as usize,
        comparar_construir_divisao_de_baixo_espanhola as *const () as usize,
        comparar_tabela_da_segunda as *const () as usize,
        comparar_tabela_da_divisao_de_baixo_espanhola as *const () as usize,
        comparar_regras_da_cfa as *const () as usize,
        comparar_construir_cfa as *const () as usize,
        comparar_construir_divisao_de_baixo_francesa as *const () as usize,
        comparar_destruir_cfa as *const () as usize,
        comparar_destruir_divisao_de_baixo_francesa as *const () as usize,
        comparar_calendario_fixo_da_serie_a as *const () as usize,
        comparar_calendario_fixo_da_serie_b as *const () as usize,
        comparar_calendario_fixo_da_c1a as *const () as usize,
        comparar_calendario_fixo_da_c1b as *const () as usize,
        comparar_toco_um_com_dois as *const () as usize,
        comparar_toco_um_com_um as *const () as usize,
        comparar_estadio_da_partida as *const () as usize,
        comparar_estadio_neutro as *const () as usize,
        comparar_publico_do_jogo as *const () as usize,
        comparar_calendario_da_terceira_francesa as *const () as usize,
        comparar_calendario_da_segunda_francesa as *const () as usize,
        comparar_calendario_da_divisao_de_baixo_francesa as *const () as usize,
        comparar_calendario_da_cfa as *const () as usize,
        comparar_calendario_da_c1_a as *const () as usize,
        comparar_calendario_da_c1_b as *const () as usize,
        comparar_calendario_da_serie_a as *const () as usize,
        comparar_calendario_da_serie_b as *const () as usize,
        comparar_calendario_da_primeira_francesa as *const () as usize,
        comparar_cqo7_fase_seguinte_do_mineiro as *const () as usize,
        comparar_cqo7_final_do_mineiro as *const () as usize,
        comparar_cqo7_fim_da_temporada_do_mineiro as *const () as usize,
        comparar_cqo8_fase_seguinte_do_paranaense as *const () as usize,
        comparar_cqo8_final_do_paranaense as *const () as usize,
        comparar_cqo8_fim_da_temporada_do_paranaense as *const () as usize,
        comparar_cqo9_fase_seguinte_do_carioca as *const () as usize,
        comparar_cqo9_final_do_carioca as *const () as usize,
        comparar_cqo9_fim_da_temporada_do_carioca as *const () as usize,
        comparar_cqo10_copa_da_italia_tabela as *const () as usize,
        comparar_cqo10_copa_c_tabela as *const () as usize,
        comparar_cqo10_supercopa_tabela as *const () as usize,
        comparar_cqo10_supercopa_c1_tabela as *const () as usize,
        comparar_cqo11_copa_da_italia_inscritos as *const () as usize,
        comparar_cqo11_copa_c_inscritos as *const () as usize,
        comparar_cqo11_supercopa_clubes as *const () as usize,
        comparar_cqo12_copa_do_brasil_tabela as *const () as usize,
        comparar_cqo12_times_da_copa as *const () as usize,
        comparar_cqo12_regras_da_copa as *const () as usize,
        comparar_cqo13_libertadores_tabela as *const () as usize,
        comparar_cqo13_mercosul_tabela as *const () as usize,
        comparar_cqo13_mercosul_convidados as *const () as usize,
        comparar_calendario_da_segunda_inglesa as *const () as usize,
        comparar_calendario_da_terceira_inglesa as *const () as usize,
        comparar_calendario_da_primeira_inglesa as *const () as usize,
        comparar_calendario_da_premiership as *const () as usize,
        comparar_calendario_da_primera as *const () as usize,
        comparar_cqp7_virada_do_baiano as *const () as usize,
        comparar_cqp7_fase_seguinte_do_baiano as *const () as usize,
        comparar_cqp7_virada_do_centro_oeste as *const () as usize,
        comparar_cqp7_fase_seguinte_do_centro_oeste as *const () as usize,
        comparar_cqp7_virada_do_gaucho as *const () as usize,
        comparar_cqp7_fase_seguinte_do_gaucho as *const () as usize,
        comparar_cqp8_virada_do_goiano as *const () as usize,
        comparar_cqp8_fase_seguinte_do_goiano as *const () as usize,
        comparar_cqp8_virada_do_norte as *const () as usize,
        comparar_cqp8_fase_seguinte_do_norte as *const () as usize,
        comparar_cqp8_virada_do_nordeste as *const () as usize,
        comparar_cqp8_fase_seguinte_do_nordeste as *const () as usize,
        comparar_cqp9_virada_do_pernambucano as *const () as usize,
        comparar_cqp9_fase_seguinte_do_pernambucano as *const () as usize,
        comparar_cqp9_virada_do_catarinense as *const () as usize,
        comparar_cqp9_fase_seguinte_do_catarinense as *const () as usize,
        comparar_cqp9_virada_do_paulista as *const () as usize,
        comparar_cqp9_fase_seguinte_do_paulista as *const () as usize,
        comparar_calendario_da_segunda_espanhola as *const () as usize,
        comparar_calendario_da_divisao_de_baixo_espanhola as *const () as usize,
        comparar_construir_primeira_francesa as *const () as usize,
        comparar_cqq7_fase_seguinte_da_serie_a as *const () as usize,
        comparar_cqq7_play_off_da_serie_a as *const () as usize,
        comparar_cqq7_fase_seguinte_da_serie_b as *const () as usize,
        comparar_cqq7_play_off_da_serie_b as *const () as usize,
        comparar_cqq8_virada_da_serie_a as *const () as usize,
        comparar_cqq8_acesso_e_rebaixamento as *const () as usize,
        comparar_cqq8_virada_da_serie_b as *const () as usize,
        comparar_cqq9_fase_seguinte_da_j1 as *const () as usize,
        comparar_cqq9_final_da_j1 as *const () as usize,
        comparar_cqq9_virada_da_j1 as *const () as usize,
        comparar_tabela_das_sedes as *const () as usize,
        comparar_noticia_das_sedes as *const () as usize,
        comparar_carregar_sedes as *const () as usize,
        comparar_cadeia_de_acessos as *const () as usize,
        comparar_noticia_do_premio as *const () as usize,
        comparar_noticia_da_tv as *const () as usize,
        comparar_cqp10_copa_dos_campeoes_tabela as *const () as usize,
        comparar_cqp10_merconorte_tabela as *const () as usize,
        comparar_cqp10_supercopa_europeia_tabela as *const () as usize,
        comparar_cqp10_supercopa_asiatica_tabela as *const () as usize,
        comparar_cqp11_asia_clubes_tabela as *const () as usize,
        comparar_cqp11_asia_recopa_tabela as *const () as usize,
        comparar_cqp11_trofeu_tabela as *const () as usize,
        comparar_cqp12_copa_america_tabela as *const () as usize,
        comparar_cqp12_africana_tabela as *const () as usize,
        comparar_cqp12_confed_tabela as *const () as usize,
        comparar_cqp13_intertoto_tabela as *const () as usize,
        comparar_cqp13_intercontinental_tabela as *const () as usize,
        comparar_cqp13_interamericana_tabela as *const () as usize,
        comparar_cqp13_concacaf_tabela as *const () as usize,
        comparar_cqq10_uefa_tabela as *const () as usize,
        comparar_cqq10_intertoto_inscritos as *const () as usize,
        comparar_cqq10_intertoto_lista as *const () as usize,
        comparar_cqq11_europeia_clubes as *const () as usize,
        comparar_cqq11_asiatica_clubes as *const () as usize,
        comparar_cqq11_c1_vagas as *const () as usize,
        comparar_cqq12_oceania_tabela as *const () as usize,
        comparar_cqq12_asia_clubes_times as *const () as usize,
        comparar_cqq12_asia_recopa_times as *const () as usize,
        comparar_cqr1_turno_do_carioca as *const () as usize,
        comparar_cqr1_turno_do_mineiro as *const () as usize,
        comparar_cqr1_turno_do_paranaense as *const () as usize,
        comparar_cqr1_turno_da_j1 as *const () as usize,
        comparar_cqr2_virada_do_carioca as *const () as usize,
        comparar_cqr2_virada_do_mineiro as *const () as usize,
        comparar_cqr2_virada_do_paranaense as *const () as usize,
        comparar_cqr3_fase_seguinte_da_nsl as *const () as usize,
        comparar_cqr3_serie_final_da_nsl as *const () as usize,
        comparar_cqr3_grande_final_da_nsl as *const () as usize,
        comparar_cqr4_fase_seguinte_da_mls as *const () as usize,
        comparar_cqr4_play_off_da_mls as *const () as usize,
        comparar_cqr4_final_da_mls as *const () as usize,
        comparar_cqr5_virada_copa_do_brasil as *const () as usize,
        comparar_cqr5_virada_copa_do_imperador as *const () as usize,
        comparar_cqr5_virada_copa_da_liga_japonesa as *const () as usize,
        comparar_cqr6_virada_supercopa_japonesa as *const () as usize,
        comparar_cqr6_virada_copa_dos_eua as *const () as usize,
        comparar_cqr6_virada_jogo_das_estrelas as *const () as usize,
        comparar_cqr6_times_do_jogo_das_estrelas as *const () as usize,
        comparar_cqr10_libertadores_dia as *const () as usize,
        comparar_cqr10_libertadores_primeiro_grupo as *const () as usize,
        comparar_cqr10_libertadores_grupo as *const () as usize,
        comparar_cqr11_mercosul_dia as *const () as usize,
        comparar_cqr11_mercosul_primeiro_grupo as *const () as usize,
        comparar_cqr11_mercosul_grupo as *const () as usize,
        comparar_cqr12_merconorte_dia as *const () as usize,
        comparar_cqr12_merconorte_primeiro_grupo as *const () as usize,
        comparar_cqr12_merconorte_grupo as *const () as usize,
        comparar_cqr13_copa_america_dia as *const () as usize,
        comparar_cqr13_copa_america_primeiro_grupo as *const () as usize,
        comparar_cqr13_copa_america_grupo as *const () as usize,
        comparar_cqs1_tabela_da_mls as *const () as usize,
        comparar_cqs1_segunda_divisao_da_mls as *const () as usize,
        comparar_cqs1_terceira_divisao_da_mls as *const () as usize,
        comparar_cqs2_virada_da_mls as *const () as usize,
        comparar_cqs2_posicoes_previstas_da_mls as *const () as usize,
        comparar_cqs2_clubes_da_mls as *const () as usize,
        comparar_cqs3_grupo_da_serie_c as *const () as usize,
        comparar_cqs3_segunda_fase_da_serie_c as *const () as usize,
        comparar_cqs3_terceira_fase_da_serie_c as *const () as usize,
        comparar_cqs4_final_da_serie_c as *const () as usize,
        comparar_cqs4_virada_da_serie_c as *const () as usize,
        comparar_cqs4_posicoes_previstas_da_serie_c as *const () as usize,
        comparar_cqs5_trofeu_clubes as *const () as usize,
        comparar_cqs5_trofeu_fase_das_chaves as *const () as usize,
        comparar_cqs5_trofeu_fase_seguinte as *const () as usize,
        comparar_cqs6_trofeu_final as *const () as usize,
        comparar_cqs6_trofeu_classificacao as *const () as usize,
        comparar_cqs7_fase_de_copa as *const () as usize,
        comparar_cqs7_fim_da_fase_de_copa as *const () as usize,
        comparar_cqs10_libertadores_mata_mata as *const () as usize,
        comparar_cqs10_merconorte_semifinais as *const () as usize,
        comparar_cqs10_merconorte_fase_seguinte as *const () as usize,
        comparar_cqs11_merconorte_convidados as *const () as usize,
        comparar_cqs11_mercosul_fase_seguinte as *const () as usize,
        comparar_cqs11_copa_america_fase_seguinte as *const () as usize,
        comparar_cqs12_copa_america_selecoes as *const () as usize,
        comparar_cqs12_copa_america_previstas as *const () as usize,
        comparar_cqs12_copa_america_temporada_nova as *const () as usize,
        comparar_cqt1_construir_baiano as *const () as usize,
        comparar_cqt1_construir_centro_oeste as *const () as usize,
        comparar_cqt1_construir_gaucho as *const () as usize,
        comparar_cqt2_construir_goiano as *const () as usize,
        comparar_cqt2_construir_mineiro as *const () as usize,
        comparar_cqt2_construir_norte as *const () as usize,
        comparar_cqt3_construir_nordeste as *const () as usize,
        comparar_cqt3_construir_paranaense as *const () as usize,
        comparar_cqt3_construir_pernambucano as *const () as usize,
        comparar_cqt4_construir_carioca as *const () as usize,
        comparar_cqt4_construir_catarinense as *const () as usize,
        comparar_cqt4_construir_paulista as *const () as usize,
        comparar_cqt4_atualizar_media_de_publico as *const () as usize,
        comparar_cqt10_oceania_dia as *const () as usize,
        comparar_cqt10_oceania_grupo_a as *const () as usize,
        comparar_cqt10_oceania_grupo_b as *const () as usize,
        comparar_cqt11_oceania_clubes as *const () as usize,
        comparar_cqt11_oceania_fase_seguinte as *const () as usize,
        comparar_cqt11_oceania_semifinais as *const () as usize,
        comparar_cqt12_asia_fase_seguinte as *const () as usize,
        comparar_cqt12_asia_grupos as *const () as usize,
        comparar_cqt12_asia_semifinais as *const () as usize,
        comparar_cqs13_africana_dia as *const () as usize,
        comparar_cqs13_africana_sorteio as *const () as usize,
        comparar_cqs13_africana_grupo as *const () as usize,
        comparar_cqt13_africana_quartas as *const () as usize,
        comparar_cqt13_recopa_asiatica_fase_seguinte as *const () as usize,
        comparar_cqt13_recopa_asiatica_finais as *const () as usize,
        comparar_cqr7_copa_da_italia_grupo_um as *const () as usize,
        comparar_cqr7_copa_da_serie_c_grupo_um as *const () as usize,
        comparar_cqr7_copa_da_italia_fase_seguinte as *const () as usize,
        comparar_cqr7_copa_da_serie_c_fase_seguinte as *const () as usize,
        comparar_cqr8_copa_da_italia_grupo as *const () as usize,
        comparar_cqr8_copa_da_serie_c_grupo as *const () as usize,
        comparar_cqr9_copa_da_italia_mata_mata as *const () as usize,
        comparar_cqr9_copa_da_serie_c_mata_mata as *const () as usize,
        comparar_cqs8_eurocopa_regras as *const () as usize,
        comparar_cqs8_eurocopa_tamanho_do_grupo as *const () as usize,
        comparar_cqs8_eurocopa_situacao as *const () as usize,
        comparar_cqs8_eurocopa_grupo_um as *const () as usize,
        comparar_cqs9_uefa_inscritos as *const () as usize,
        comparar_cqt5_copa_do_rei_dia as *const () as usize,
        comparar_cqt5_historico_liga_dos_campeoes as *const () as usize,
        comparar_cqt5_historico_copa_da_grecia as *const () as usize,
        comparar_cqt5_historico_copa_da_liga_irlandesa as *const () as usize,
        comparar_cqt5_historico_copa_da_serie_c as *const () as usize,
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
        "motor_colocar_na_vaga", "motor_ordem_por_tabela", "motor_ordem_por_condicao", "motor_titulares", "motor_guardar_tatica",
        "motor_aplicar_trocas", "motor_punicoes", "motor_penaltis", "motor_aplicar_falta", "motor_dividida", "motor_disputa", "motor_responder", "motor_jogada", "motor_eventos_do_pos_jogo", "motor_jogadores_do_pos_jogo", "motor_clube_do_pos_jogo", "motor_partida_sem_humano", "motor_interesse_do_dia", "motor_reiniciar_lance", "motor_chute", "motor_nota_do_arbitro", "motor_registrar_nota_do_arbitro", "motor_batedor_guardado", "motor_guardar_instrucoes", "motor_numero_da_competicao", "motor_favorito_de", "motor_rejeitado_por", "motor_fechamento", "motor_ler_linha_tatica", "motor_gravar_linha_tatica", "motor_simulacao_rapida", "motor_lesionar", "motor_registrar_lesao", "motor_destaque_do_jogador", "motor_desfecho_do_chute", "motor_chute_a_gol", "motor_mover", "motor_conduzir", "motor_laco", "motor_executar_passe", "motor_atras_no_agregado", "motor_companheiro_livre", "motor_chutar", "motor_construir", "motor_preparar_partida", "motor_destruir", "motor_escalar_lado", "motor_encerrar_dia", "motor_escolher_partida_do_dia", "motor_escalar_automatico", "motor_iniciar_proxima_partida", "motor_andar_partida_do_dia", "motor_fechar_partida_do_dia", "motor_jogar_o_dia", "motor_montar_o_dia", "motor_lado_do_jogador", "motor_sons_terminados", "motor_gravar_eventos", "motor_ler_eventos", "motor_passo_em_rede", "motor_ler_eventos_cfg", "motor_texto_do_evento", "motor_confirmar_partida_em_rede", "motor_narrar_no_painel", "motor_abrir_no_conteiner", "motor_buscar_no_conteiner", "motor_posicao_no_conteiner", "motor_mascara_de_posicoes", "motor_casa_no_campo", "motor_vagas_sem_conflito", "motor_estatistica_do_jogador", "motor_media_das_notas", "motor_zagueiros_sem_conflito", "motor_serve_para_a_vaga", "motor_codigo_da_posicao", "motor_criterio_da_comissao", "motor_gravar_vaga_tatica", "motor_pessoa_da_vaga_tatica", "motor_gravar_formacao_tatica", "motor_casa_guardada", "motor_ligar_marca_da_formacao", "motor_reiniciar_recordes", "motor_girar_rodada", "motor_registrar_jogo_da_rodada", "motor_marcar_ano_no_calendario", "motor_vaga_extra_tatica", "motor_idades_da_selecao", "motor_gravar_dado_do_humano", "motor_registro_do_observador", "motor_humano_manda_no_clube", "motor_categoria_do_contrato", "motor_bola_na_area", "motor_partida_abandonada", "motor_vagas_do_lado_sem_conflito", "motor_zagueiros_do_lado_sem_conflito", "motor_construir_arbitro", "motor_destruir_jogador_da_partida", "motor_evento_na_frente", "motor_construir_fila_da_narracao", "motor_liberar_tabelas_de_eventos", "motor_ordem_das_partidas_do_dia", "motor_ordem_de_acao", "motor_variante_do_evento", "motor_combinar_com_o_anterior", "motor_comentario_de_contexto", "motor_construir_jogador_da_partida", "motor_proximo_evento", "motor_nome_do_jogador_na_narracao", "motor_habilidade_vista", "motor_cumprir_regra_do_elenco", "motor_nota_fora_da_partida", "motor_nota_para_a_vaga", "motor_pessoa_na_lista", "motor_ficha_de_contrato", "motor_desgaste_diario", "motor_registro_da_formacao", "motor_peso_da_competicao", "motor_gravar_dado_da_noticia", "motor_recuperar_lesao", "motor_pessoa_na_lista_de_transferencias", "motor_punicao_da_tabela", "motor_bits_do_registro", "motor_competicao_marcada", "motor_registro_da_liga", "motor_media_do_jogo", "motor_valor_do_contrato", "motor_nivel_para_a_selecao", "motor_oferta_de_contrato", "motor_oferta_acertada", "motor_oferta_fechada", "motor_rotulo_do_controle", "motor_fechar_tela_da_partida", "motor_abrir_tela_de_espera", "motor_noticia_com_resposta_2", "motor_tem_noticia", "motor_importancia_da_partida", "motor_abrir_ultimos_placares", "motor_montar_ultimos_placares", "motor_responder_ultimos_placares", "motor_abrir_detalhes_da_partida", "motor_responder_detalhes_da_partida", "motor_fechar_detalhes_da_partida", "motor_disciplina_temporada_nova", "motor_disciplina_do_dia", "motor_disciplina_gravar_vaga", "motor_marcar_contrato_acertado", "motor_desmarcar_contrato_acertado", "motor_vaga_da_oferta", "motor_abrir_tela_da_partida", "motor_responder_ultimos_placares_da_partida", "motor_responder_estatisticas_do_time", "motor_painel_dos_ultimos_placares", "motor_sem_tela_seguinte", "motor_cor_do_titulo", "motor_lista_de_observados", "motor_entrada_do_treino", "motor_gravar_carga_de_treino", "motor_e_competicao_do_amistoso", "motor_preencher_data_internacional", "motor_limpar_registro_de_tecnico", "motor_tirar_contrato_de_emprestimo", "motor_desmarcar_lances_pelo_jogador", "motor_tecnico_substituto", "motor_clube_do_contrato_acertado", "motor_registrar_jogo_pela_selecao", "motor_punicao_guardada", "motor_entrada_da_aposentadoria", "motor_registrar_competicoes_do_amistoso", "motor_vaga_da_noticia", "motor_tirar_percentual_da_proxima_venda", "motor_tirar_bonus_por_jogo", "motor_tirar_bonus_pela_selecao", "motor_tirar_reserva_de_tecnico", "motor_tirar_jogador_do_banco", "motor_premio_da_liga", "motor_ultima_nota_do_jogador", "motor_vaga_da_busca", "motor_tipo_da_lista_da_busca", "motor_ordem_do_confronto", "motor_atualizar_confronto_direto", "motor_marcar_termos_oferecidos", "motor_hora_das_ferias", "motor_escalacao_guardada", "motor_tecnico_da_busca", "motor_abrir_jogos_do_dia", "motor_responder_jogos_do_dia", "motor_vaga_do_mais_bem_avaliado", "motor_estatistica_da_liga_do_jogador", "motor_entrada_do_jogo_bate", "motor_balde_do_registro", "motor_atribuir_bloco_do_time", "motor_atribuir_linha_do_jogador", "motor_contar_placar_ao_vivo", "motor_responder_continuar_da_partida", "motor_abrir_escalacao_da_partida", "motor_responder_escalacao_da_partida", "motor_painel_das_estatisticas_do_time", "motor_painel_das_notas_dos_jogadores", "motor_painel_das_estatisticas_da_partida", "motor_valores_das_estatisticas_da_partida", "motor_valores_das_zonas_de_acao", "motor_valores_das_notas_dos_jogadores", "motor_painel_das_zonas_de_acao", "motor_painel_do_relatorio_da_partida", "motor_montar_jogos_do_dia", "motor_preencher_estatisticas_do_time", "motor_atualizar_ultimos_placares", "motor_esperar_os_outros_jogadores", "motor_ler_registro_da_partida", "motor_registrar_partida", "motor_gravar_registro_da_partida", "motor_desenhar_conferencia_da_escalacao", "motor_carregar_estatisticas_da_partida", "motor_salvar_estatisticas_da_partida", "motor_organizar_area", "motor_iniciar_area", "motor_desenhar_area", "motor_montar_tela_da_partida", "motor_responder_tela_da_partida", "motor_montar_detalhes_da_partida", "motor_atualizar_barra_da_posse", "motor_painel_da_visao_geral_da_partida", "motor_tabela_da_liga_ao_vivo", "motor_historia_punivel_do_jogador", "motor_escolher_tipo_de_lesao", "motor_categoria_da_noticia", "motor_apagar_repetidos_das_convocacoes", "motor_vaga_do_escalado", "motor_escolher_comentarista", "motor_regras_da_rodada_no_jogo", "motor_chave_do_jogo_na_copa", "motor_pais_da_excursao", "motor_fechar_propostas_da_oferta", "motor_enfileirar_transferencia_livre", "motor_negociacoes_de_troca_do_jogador", "motor_tirar_jogador_da_selecao", "motor_faixa_do_horario", "motor_teto_da_observacao", "motor_ofertas_pelo_jogador", "motor_reputacao_alvo_do_clube", "motor_gravar_jogo_na_noticia", "motor_preencher_registro_da_partida", "motor_ordem_dos_eventos", "motor_tirar_idioma_do_jogador", "motor_pode_contratar_jogador", "motor_trocas_oferecendo_o_jogador", "motor_noticias_nao_lidas", "motor_gravar_marca_da_noticia", "motor_clube_de_tecnico_humano", "motor_data_do_sorteio_da_rodada", "motor_data_do_jogo_da_rodada", "motor_tirar_clube_dos_amistosos", "motor_ordem_das_linhas_da_tabela", "motor_montar_grade_de_turnos", "motor_margem_do_ultimo_dia", "motor_partida_das_vagas_da_noticia", "motor_resultado_da_eliminatoria_na_noticia", "motor_dia_da_rodada_da_copa", "motor_mapa_de_casas_da_formacao", "motor_nota_do_clube_para_o_jovem", "motor_partida_assistida_pelo_humano", "motor_largar_partida_assistida", "motor_ficha_do_registro", "motor_liga_tem_rebaixamento", "motor_data_da_rodada_da_liga", "motor_importancia_do_jogo_da_liga", "motor_copiar_recordista", "motor_copiar_registro_de_partida", "motor_copiar_registro_de_sequencia", "motor_humano_assiste_a_partida", "motor_humano_deu_apelido", "motor_copiar_aposentadoria_pendente", "motor_montar_estatisticas_da_partida", "motor_desmontar_estatisticas_da_partida", "motor_casar_nome_com_padrao", "motor_id_do_conteiner", "motor_tamanho_dos_itens_do_padrao", "motor_empacotar_no_conteiner", "motor_desempacotar_do_conteiner", "motor_limpar_pasta_temporaria", "motor_ler_do_conteiner", "motor_gravar_no_conteiner", "motor_fechar_conteiner", "motor_filtro_do_jogo_salvo", "motor_filtro_do_inicio_rapido", "motor_versao_do_jogo_salvo_valida", "motor_gravar_regras_fifa", "motor_carregar_regras_fifa", "motor_montar_resumo_do_save", "motor_gravar_banco_no_save", "motor_carregar_banco_do_save", "motor_liberar_dados_do_jogo", "motor_criar_jogo_pela_configuracao", "motor_gravar_jogo", "motor_carregar_jogo", "motor_escopo_da_reputacao", "motor_reputacao_vista_por", "motor_pessoa_e_estrangeira", "motor_idade_da_pessoa", "motor_nacao_com_transferencia_livre", "motor_pessoa_e_comunitaria", "motor_pessoa_listada_para_transferencia", "motor_atributo_calculado_do_jogador", "motor_atributo_da_pessoa", "motor_ordem_do_historico_de_competicao", "motor_vetor_de_atributos_do_jogador", "motor_nacao_britanica_ou_irlanda", "motor_clube_basco", "motor_pessoa_listada_pelo_clube", "motor_pessoa_listada_para_emprestimo", "motor_pessoa_listada_a_pedido", "motor_nota_relativa_do_atributo", "motor_clube_pode_ter_reservas", "motor_primeiro_nome_da_nacao", "motor_segundo_nome_da_nacao", "motor_formatar_nome_da_pessoa", "motor_cores_do_uniforme_do_clube", "motor_tecnico_do_clube_ou_selecao", "motor_nome_ingles_do_clube", "motor_ajustar_moral_da_pessoa", "motor_vetor_de_atributos_do_nao_jogador", "motor_marcar_elenco_da_pessoa", "motor_contar_elenco_do_clube", "motor_reunir_elenco_do_clube", "motor_clube_pode_ser_dirigido", "motor_vetor_para_o_nao_jogador", "motor_vetor_para_o_jogador", "motor_normalizar_atributos_do_nao_jogador", "motor_familia_da_cor", "motor_cores_se_confundem", "motor_atributo_calculado_do_nao_jogador", "motor_forma_do_adjetivo", "motor_atributo_do_nao_jogador", "motor_nota_relativa_com_habilidade_reduzida", "motor_primeiro_nome_pelo_peso", "motor_cores_da_competicao", "motor_clube_tem_vaga_para_cargo", "motor_incluir_pessoa_no_clube", "motor_casar_chave_de_clube", "motor_remapear_chave_de_clube", "motor_casar_chave_de_nacao", "motor_casar_chave_de_competicao_de_clube", "motor_casar_chave_de_competicao_de_pessoa", "motor_ler_continentes", "motor_ler_nacoes", "motor_ler_cidades", "motor_ler_estadios", "motor_ler_pessoas_157", "motor_ler_historico_de_pessoas", "motor_ler_historicos_de_competicao", "motor_ler_competicoes_de_pessoal", "motor_ler_historico_de_competicoes_de_pessoal", "motor_ler_arbitros", "motor_ler_cores", "motor_ler_nomes", "motor_ler_clubes", "motor_ler_competicoes", "motor_ler_nao_jogadores", "motor_ler_jogadores", "motor_ler_pessoas_110", "motor_ler_preferencias", "motor_gravar_continentes", "motor_gravar_nacoes", "motor_gravar_cidades", "motor_gravar_estadios", "motor_gravar_clubes", "motor_gravar_pessoas_110", "motor_gravar_nao_jogadores", "motor_gravar_jogadores", "motor_gravar_preferencias", "motor_gravar_historico_de_pessoas", "motor_gravar_competicoes", "motor_gravar_historicos_de_competicao", "motor_gravar_competicoes_de_pessoal", "motor_gravar_arbitros", "motor_gravar_registros_58", "motor_gravar_nomes", "motor_abrir_arquivo_do_banco", "motor_criar_arquivo_do_banco", "motor_liberar_historico_de_pessoas", "motor_liberar_historico_de_premios", "motor_crescer_historico_de_competicoes", "motor_ordem_dos_clubes_pela_reputacao", "motor_ordem_do_elenco_pela_habilidade", "motor_ordem_do_elenco_pela_habilidade_atual", "motor_normalizar_atributos_fisicos_do_jogador", "motor_segundo_nome_pelo_peso", "motor_normalizar_atributos_do_jogador", "motor_ler_clubes_v1", "motor_ler_nao_jogadores_v1", "motor_ler_jogadores_v1", "motor_ler_competicoes_v1", "motor_nome_ingles_da_nacao", "motor_nome_curto_ingles_do_clube", "motor_cortar_elencos_de_a_e_b", "motor_juntar_o_time_b_ao_a", "motor_montar_indice_de_nomes", "motor_tirar_pessoa_do_clube", "motor_olheiros_do_clube", "motor_clube_em_temporada", "motor_clube_em_nacao_grande", "motor_nacao_grande", "motor_limpar_vinculos_de_opiniao", "motor_juntar_vinculo_de_opiniao", "motor_tirar_vinculo_de_opiniao", "motor_auxiliar_do_clube", "motor_treinadores_do_clube", "motor_contar_treinadores_do_clube", "motor_sigla_inglesa_da_nacao", "motor_nome_ingles_da_competicao", "motor_nome_ingles_da_competicao_de_pessoal", "motor_nome_ingles_do_estadio", "motor_arquivo_do_idioma_existe", "motor_liberar_tabelas_em_ingles", "motor_gerar_comissao_extra", "motor_cortar_tabela_de_clubes", "motor_podar_historico_de_competicoes", "motor_resolver_indices_do_banco", "motor_ordenar_clubes_pela_reputacao", "motor_clube_tem_defensores", "motor_clube_tem_meias", "motor_clube_tem_atacantes", "motor_clube_tem_esforcados", "motor_clube_tem_goleiro", "motor_clube_tem_meias_que_roubam", "motor_faltam_atacantes_pelo_alto", "motor_faltam_zagueiros_pelo_alto", "motor_faltam_zagueiros_rapidos", "motor_clube_tem_atacante_rapido", "motor_clube_tem_meias_criativos", "motor_faixa_de_forca_do_elenco", "motor_reputacao_do_clube_para_pessoa", "motor_contar_rejeitados_no_clube", "motor_contar_favoritos_no_clube", "motor_tirar_clube_favorito", "motor_clube_tirar_pessoa_favorita", "motor_tirar_pessoa_favorita", "motor_tirar_pessoa_rejeitada", "motor_pessoas_favoritas", "motor_clube_acrescentar_pessoa_favorita", "motor_clube_acrescentar_pessoa_rejeitada", "motor_acrescentar_pessoa_favorita", "motor_acrescentar_clube_rejeitado", "motor_esmaecer_relacoes", "motor_pessoas_rejeitadas", "motor_mascara_da_posicao_natural", "motor_contar_elenco_com_mascara", "motor_escolher_companheiro_favorito", "motor_clube_fora_da_argentina", "motor_clube_fora_de_argentina_franca_croacia", "motor_nacao_da_melanesia", "motor_nacao_da_polinesia", "motor_nome_do_cargo_de_treino", "motor_anos_entre_datas_em_texto", "motor_ordem_dos_clubes_pelo_nome_curto", "motor_ordem_das_pessoas_pelo_nome_completo", "motor_pessoa_confere_com_os_nomes", "motor_ordem_dos_clubes_pela_latitude", "motor_ordem_dos_clubes_pela_longitude", "motor_nome_por_ids", "motor_nome_oficial", "motor_texto_da_mascara_da_posicao", "motor_nome_da_posicao", "motor_descricao_do_jogador", "motor_melhor_candidato_a_favorito", "motor_forcas_do_jogador", "motor_fraquezas_do_jogador", "motor_contar_fisioterapeutas", "motor_clube_e_time_b", "motor_pessoas_fora_do_elenco", "motor_atributo_visivel", "motor_atributo_para_quem_ve", "motor_nome_do_atributo_de_tecnico", "motor_ordem_das_pessoas_pelo_sobrenome", "motor_ordem_dos_ponteiros_de_clube_pela_reputacao", "motor_nacao_do_clube_na_cee", "motor_regioes_pelo_nome", "motor_ordem_das_regioes_pelo_nome", "motor_pessoa_da_opiniao", "motor_nome_pelos_textos", "motor_nome_do_cargo", "motor_nome_do_idioma", "motor_nome_da_selecao", "motor_nome_do_atributo_do_jogador", "motor_texto_das_posicoes", "motor_nome_da_regiao", "motor_adjetivo_do_continente", "motor_definir_chaves_fixas", "motor_definir_chaves_fixas_1080", "motor_inicializar_jogadores", "motor_adjetivo_da_nacao", "motor_ordenar_tabela_da_liga", "motor_mini_tabela_do_confronto", "motor_liberar_tabela_da_liga", "motor_liga_completa", "motor_pagar_direitos_de_tv", "motor_registrar_honras_da_temporada", "motor_acrescentar_ao_historico", "motor_tipo_de_confronto_na_liga", "motor_forca_do_clube", "motor_clube_colocado_mais_recente", "motor_amistoso_do_clube", "motor_lista_da_data", "motor_proxima_partida", "motor_estadio_em_uso", "motor_data_do_amistoso_internacional", "motor_encerrar_convocacoes_da_competicao", "motor_encerrar_convocacoes_da_nacao", "motor_podar_registros_de_convocacao", "motor_por_no_elenco_da_selecao", "motor_tirar_do_elenco_da_selecao", "motor_dispensar_acima_da_idade_do_sub21", "motor_limpar_competicao_atual", "motor_liberar_competicoes_do_continente", "motor_convocar_substituto", "motor_montar_reservas_das_selecoes", "motor_ordem_das_reservas", "motor_ordem_pelo_clube", "motor_adversario_do_amistoso", "motor_adversario_da_excursao", "motor_aplicar_resultado_na_tabela", "motor_pontos_do_resultado", "motor_ordem_pela_posicao_do_confronto", "motor_ordem_das_datas_da_liga", "motor_linha_do_clube", "motor_liga_tem_o_clube", "motor_copiar_recorde_de_nota", "motor_copiar_recorde_de_melhor_em_campo", "motor_copiar_recorde_de_disciplina", "motor_copiar_recorde_de_contagem", "motor_copiar_recorde_do_torcedor", "motor_construir_recorde_de_ocorrencias", "motor_construir_recorde_de_contagem", "motor_copiar_recorde_de_valor", "motor_copiar_recorde_de_gols_na_partida", "motor_copiar_noticia", "motor_copiar_transferencia_passada", "motor_copiar_transferencia_futura", "motor_copiar_entrada_da_fila_de_regens", "motor_esta_na_lista_de_observados", "motor_escolher_doador_de_dados", "motor_copiar_registro_de_suspensao", "motor_salario_padrao_do_contrato", "motor_condicao_de_jogo_do_jogador", "motor_dias_ate_o_fim_do_contrato", "motor_limpar_todas_as_queixas", "motor_conferir_retirada_do_pedido", "motor_lugar_no_elenco", "motor_subir_para_o_principal", "motor_descer_para_os_reservas", "motor_copiar_contrato_de_pessoal", "motor_subir_moral_pela_partida", "motor_tirar_conversas_de_contrato", "motor_tirar_interesses_de_transferencia", "motor_tirar_pedido_de_observacao", "motor_tirar_apelido_de_todos", "motor_tirar_aposentadoria_pendente", "motor_aposentadoria_do_desempregado", "motor_oferta_precisa_de_permissao", "motor_atualizar_permissoes_de_trabalho", "motor_olheiro_da_busca", "motor_sequencia_atual_do_clube", "motor_construir_recordista", "motor_tecnico_rival_para_comentar", "motor_construir_recorde_de_valor_pago", "motor_construir_recorde_de_valor_recebido", "motor_construir_recorde_de_sequencia", "motor_construir_recorde_de_vitorias_seguidas", "motor_construir_recorde_de_derrotas_seguidas", "motor_construir_recorde_de_invencibilidade", "motor_construir_recorde_sem_vencer", "motor_construir_recorde_de_gols_na_partida", "motor_construir_recorde_do_mais_novo", "motor_texto_do_fim_do_prazo", "motor_pode_encerrar_emprestimo", "motor_pow_do_game", "motor_tirar_ofertas_guardadas", "motor_guardar_suspensoes_do_emprestado", "motor_preencher_vaga_da_comissao", "motor_noticia_para_os_tecnicos_da_nacao", "motor_missao_do_olheiro", "motor_copa_completa", "motor_construir_recorde_do_mais_velho", "motor_permissao_de_trabalho_de", "motor_passa_na_permissao", "motor_soltar_suspensoes_guardadas", "motor_tirar_ofertas_pendentes", "motor_marcar_decisao_da_transferencia", "motor_noticia_para_todos_os_tecnicos", "motor_noticia_para_os_tecnicos_da_divisao", "motor_noticia_para_os_tecnicos_do_pais", "motor_oferta_bate_o_recorde", "motor_condicao_do_jogador", "motor_limite_de_escalacao", "motor_limpar_partidas_assistidas", "motor_alvos_dos_olheiros_do_clube", "motor_trocar_olheiro_da_busca", "motor_diretoria_depois_da_partida", "motor_disciplina_da_partida", "motor_entradas_do_clube_na_shortlist", "motor_maior_oferta_pelo_jogador", "motor_recordes_de_sempre", "motor_recordes_da_temporada", "motor_guardar_noticia_de_recorde", "motor_acrescentar_reserva_de_tecnico", "motor_acrescentar_aposentadoria", "motor_guardar_vaga_para_regen", "motor_acionar_clausulas_de_rebaixamento", "motor_tirar_oferta_de_contrato", "motor_esta_suspenso", "motor_escopo_das_regras_da_partida", "motor_codigo_de_inelegibilidade", "motor_vaga_do_amistoso", "motor_noticia_para_os_tecnicos_do_continente", "motor_noticia_para_a_regiao_da_competicao", "motor_noticia_importante_para_a_pessoa", "motor_texto_do_dado_da_noticia", "motor_noticia_de_apuro_financeiro", "motor_olheiros_cobrem_a_nacao", "motor_fluencia_no_idioma", "motor_recomecar_historico_da_pessoa", "motor_posicao_na_formacao", "motor_clube_tem_oferta_pelo_jogador", "motor_oferta_do_clube_pelo_jogador", "motor_jogador_com_oferta_pendente", "motor_jogador_sob_oferta", "motor_clube_do_jogador_com_lei_bosman", "motor_veto_da_diretoria_na_oferta", "motor_guardar_transferencia_passada", "motor_dar_permissao_de_trabalho", "motor_guardar_premio_por_jogo_no_time", "motor_vaga_livre_de_contrato", "motor_acrescentar_contrato_da_pessoa", "motor_acrescentar_data_de_decisao", "motor_zerar_moral_do_contrato", "motor_contrato_pelo_atual", "motor_faixa_do_vencimento", "motor_registrar_historia_da_midia", "motor_expirar_historias_da_midia", "motor_historia_ja_registrada", "motor_acrescentar_interesse_de_transferencia", "motor_ordem_das_chaves_de_ids", "motor_ordem_das_historias", "motor_decidir_jogo_da_copa", "motor_decidir_confronto_da_copa", "motor_ordem_dos_cabecas_de_chave", "motor_iniciar_linha_da_tabela", "motor_vetor_dos_clubes_da_liga", "motor_gravar_posicoes_finais_da_liga", "motor_mudar_clube_de_divisao", "motor_embaralhar_ponteiros", "motor_ordenar_e_embaralhar_os_melhores", "motor_contar_jogo_da_suspensao", "motor_soltar_suspensao", "motor_apagar_historico_de_suspensoes", "motor_sortear_lesao_recorrente", "motor_recomecar_historico_de_lesoes", "motor_reiniciar_fisico_do_jogador", "motor_somar_totais_de_transferencia", "motor_recordes_de_carreira_no_clube", "motor_recorde_de_nota_da_temporada", "motor_competicao_na_data_do_amistoso", "motor_amistoso_perto_da_data", "motor_acrescentar_amistoso_planejado", "motor_noticia_pelo_alcance", "motor_noticia_ao_tecnico_do_clube", "motor_entregar_noticia_ao_tecnico", "motor_responder_todas_as_noticias", "motor_tecnico_tem_noticia_pendente", "motor_gravar_texto_na_noticia", "motor_nota_do_jogador_para_a_historia", "motor_texto_da_crise_financeira", "motor_noticias_da_caixa", "motor_aposentar_do_elenco", "motor_melhor_clube_para_o_jovem", "motor_clube_da_nacao_para_o_jovem", "motor_abrir_vagas_que_faltam", "motor_melhor_nota_de_candidato_a_tecnico", "motor_agendar_oferta", "motor_estatuto_no_elenco_pelo_clube", "motor_estimar_estatuto_no_elenco", "motor_liberar_alvos_da_lista_do_clube", "motor_busca_cobre_a_nacao", "motor_tirar_amistoso_marcado", "motor_iniciar_pares_proibidos_de_amistoso", "motor_avaliar_o_negocio_da_transferencia", "motor_confianca_da_diretoria_na_transferencia", "motor_resolver_o_ultimato_da_diretoria", "motor_tirar_o_contrato_principal", "motor_reenviar_ofertas_guardadas", "motor_mes_do_fim_da_temporada_do_clube", "motor_pontos_dos_tecnicos_no_dia", "motor_pontos_dos_tecnicos_no_fim_da_competicao", "motor_tecnico_do_mes_na_liga", "motor_saidas_pendentes_dos_tecnicos", "motor_tirar_jogador_do_elenco_do_clube", "motor_tirar_pessoa_do_cargo_no_clube", "motor_construir_noticia", "motor_ajustar_confianca_da_torcida", "motor_ajustar_confianca_da_diretoria", "motor_resposta_do_tecnico_da_maquina", "motor_marcar_noticia_lida", "motor_achar_noticia_pelo_id", "motor_noticia_de_gratidao_do_jogador", "motor_noticia_de_pedido_de_apoio", "motor_noticia_de_reacao_a_critica", "motor_montar_listas_de_posicao", "motor_ordenar_listas_de_posicao", "motor_clubes_interessados_no_jogador", "motor_acrescentar_na_lista_de_observacao", "motor_prever_resposta_do_vendedor", "motor_periodo_do_emprestimo", "motor_prazo_da_resposta_a_oferta", "motor_pagar_prestacoes_das_transferencias", "motor_tirar_oferta_de_transferencia", "motor_acrescentar_suspensao", "motor_tirar_suspensoes_de_fora_do_clube", "motor_restaurar_suspensoes_guardadas", "motor_acrescentar_idioma", "motor_melhorar_idioma", "motor_zerar_estatisticas_do_jogador", "motor_definir_competicao_da_selecao", "motor_marcar_clubes_com_jogo", "motor_tirar_jogador_da_disciplina", "motor_comparar_datas_de_decisao", "motor_decidir_as_datas_vencidas", "motor_ofertas_de_contrato_do_clube", "motor_assinar_contrato_de_emprestimo", "motor_encerrar_o_emprestimo", "motor_trocar_lugar_no_elenco", "motor_diretoria_do_tecnico_novo", "motor_motivo_da_saida_do_tecnico", "motor_registro_do_tecnico", "motor_serve_para_o_cargo", "motor_pode_assumir_o_cargo", "motor_baixar_moral_pela_partida", "motor_montar_vagas_da_formacao", "motor_refazer_lista_do_clube", "motor_suspender_clubes_que_miram_o_jogador", "motor_por_jogador_nas_listas_dos_clubes", "motor_por_jogador_a_venda", "motor_pagar_premio_por_jogos", "motor_clube_recusa_a_oferta", "motor_soltar_jogadores_da_oferta", "motor_montar_oferta_em_dinheiro", "motor_acrescentar_surto_de_doenca", "motor_espalhar_surtos_de_doenca", "motor_jogador_fora_da_temporada", "motor_forma_inicial_dos_elencos", "motor_registros_iniciais_de_lesao", "motor_listas_de_clubes_por_nacao", "motor_jogador_do_ano_da_torcida", "motor_arquivar_estatisticas_da_nacao", "motor_recordes_de_valor_de_transferencia", "motor_par_de_nacoes_sem_amistoso", "motor_ordem_das_nacoes_pela_reputacao", "motor_ordem_dos_amistosos_pela_data", "motor_pode_marcar_jogo_do_clube", "motor_ordem_das_convocacoes_pela_data", "motor_acrescentar_nacao_classificada", "motor_ordem_dos_cabecas_de_chave_decrescente", "motor_ordem_dos_cabecas_de_chave_crescente", "motor_teto_da_divisao_dos_clubes", "motor_pensar_em_demitir_o_tecnico", "motor_dar_o_ultimato_da_diretoria", "motor_tirar_o_tecnico_do_cargo", "motor_avaliar_a_selecao_na_competicao", "motor_clube_em_apuro_financeiro", "motor_faixa_da_confianca_da_diretoria", "motor_registrar_o_acesso", "motor_registrar_o_rebaixamento", "motor_registrar_a_classificacao_na_fase", "motor_registrar_a_transferencia", "motor_registrar_a_eliminacao", "motor_registrar_a_nao_classificacao", "motor_data_de_conclusao_da_oferta", "motor_primeiro_dia_permitido_da_oferta", "motor_janela_fixa_de_transferencias", "motor_compensacao_pela_pessoa", "motor_pagar_transferencia_e_clausulas", "motor_clube_vendedor_deve_aceitar", "motor_permissao_de_trabalho_na_mudanca", "motor_concluir_oferta_de_contrato_aceita", "motor_oferecer_contrato_ao_jogador_da_oferta", "motor_desfazer_oferta", "motor_fazer_oferta_de_emprestimo", "motor_concluir_transferencias_futuras", "motor_nota_do_doador_de_dados", "motor_ordem_das_aposentadorias", "motor_ordem_da_fila_de_regens", "motor_registro_da_categoria", "motor_criar_estatisticas", "motor_aposentadoria_marcada", "motor_copa_sortear_times_da_rodada", "motor_copa_separar_estadio_compartilhado", "motor_escala_pela_rodada", "motor_lado_do_jogo_para_o_clube", "motor_ordem_dos_coeficientes_das_nacoes", "motor_pedido_de_observacao", "motor_limpar_vaga_da_escalacao", "motor_marcar_batedor_de_bola_parada", "motor_restricoes_da_partida", "motor_limite_do_banco_do_dia", "motor_precisa_de_permissao_de_trabalho", "motor_clube_pode_chegar_ao_jogador", "motor_termos_ao_jogador_da_troca", "motor_fila_livre_do_clube", "motor_transferencia_futura_do_jogador", "motor_tirar_transferencia_futura", "motor_acrescentar_transferencia_futura", "motor_ofertas_abertas_pelos_jogadores_do_clube", "motor_desfazer_ofertas_pelo_jogador", "motor_guardar_percentual_da_proxima_venda", "motor_iniciar_diretorias_dos_clubes", "motor_apelacoes_de_suspensao", "motor_pode_ver_a_partida", "motor_aposentadoria_no_inicio", "motor_aposentadoria_por_idade", "motor_lista_inicial_de_aposentadorias", "motor_novas_aposentadorias", "motor_sair_dos_cargos", "motor_registrar_a_liberacao", "motor_estado_da_lesao", "motor_marcar_decisao_da_oferta", "motor_candidatos_a_tecnico_do_ano", "motor_projetar_posicoes_da_liga", "motor_ordem_da_projecao_pela_reputacao", "motor_ordem_da_projecao_pelos_pontos", "motor_achar_jogos_do_calendario", "motor_jogos_da_competicao_no_dia", "motor_competicao_tem_jogo_na_data", "motor_clube_livre_na_data", "motor_copa_tem_o_clube", "motor_nacao_parceira_do_clube", "motor_pedido_de_transferencia", "motor_cargo_com_contrato", "motor_fora_da_diretoria", "motor_iniciar_permissoes_de_trabalho", "motor_contagem_da_clausula", "motor_premio_por_jogo_do_jogador", "motor_fila_da_transferencia_livre", "motor_proxima_janela_aberta", "motor_emprestimo_negociado", "motor_oferta_com_clausula", "motor_motivo_da_recusa_vale", "motor_motivo_pelo_status_no_elenco", "motor_relacoes_do_elenco", "motor_atrito_com_o_tecnico", "motor_moral_pela_lideranca", "motor_faltas_ao_treino", "motor_competicoes_do_dia", "motor_mudar_confianca_da_diretoria", "motor_estadios_do_dia", "motor_reservar_estadio", "motor_estadio_livre_na_data", "motor_jogo_com_observado", "motor_na_lista_do_olheiro", "motor_registrar_a_primeira_convocacao", "motor_gols_na_carreira", "motor_registrar_o_emprestimo", "motor_ordem_da_volta_aos_treinos", "motor_gols_pelo_clube", "motor_registrar_a_selecao_do_ano", "motor_ordem_pelos_pontos_do_tecnico", "motor_jogo_guardado_igual", "motor_guardar_jogo_no_calendario", "motor_tirar_jogo_do_calendario", "motor_criar_amistosos_planejados", "motor_acrescentar_amistoso", "motor_planejar_amistoso_em_data_livre", "motor_atualizar_copa_no_dia", "motor_montar_rodada_da_copa", "motor_risco_de_doenca_na_viagem", "motor_jogos_da_selecao_no_periodo", "motor_atualizar_ranking_da_fifa", "motor_ordem_dos_resultados_por_pontos", "motor_resultado_no_historico", "motor_faixa_da_selecao", "motor_pontuacoes_do_ranking", "motor_nacao_chave_do_jogo", "motor_e_copa_do_mundo", "motor_e_torneio_internacional", "motor_indice_da_nacao_chave", "motor_ordem_do_ranking", "motor_ordem_das_pontuacoes", "motor_tempo_das_cidades", "motor_ler_regiao_do_clima", "motor_ler_periodo_do_clima", "motor_coeficientes_das_nacoes", "motor_dia_das_aposentadorias", "motor_soma_presa", "motor_nota_do_estadio_neutro", "motor_fator_da_distancia", "motor_ler_o_clima", "motor_fator_do_treinador", "motor_comissao_do_humano", "motor_carga_do_programa", "motor_aplicar_o_progresso", "motor_achar_o_treino_de_posicao", "motor_aprende_depressa", "motor_avancar_a_semana", "motor_tirar_a_ficha_reservada", "motor_lesao_que_volta", "motor_atualizar_amistosos_do_dia", "motor_planejar_pre_temporada_da_ia", "motor_janelas_da_pre_temporada", "motor_marcar_jogos_da_excursao", "motor_ordem_pela_faixa", "motor_ordem_pela_nacao_e_faixa", "motor_posicoes_previstas_da_copa", "motor_ordenar_clubes_pela_forca", "motor_gravar_posicao_do_clube", "motor_linha_da_nacao", "motor_linha_da_nacao_do_clube", "motor_ano_da_temporada_do_clube", "motor_clube_na_temporada", "motor_continente_principal", "motor_nacao_principal", "motor_estatistica_da_categoria", "motor_ordem_dos_idiomas", "motor_ordem_da_historia_por_clube", "motor_clausula_de_rescisao_ativa", "motor_periodo_protegido", "motor_tipo_de_bonus_permitido", "motor_adiar_pelo_clima", "motor_financas_do_clube", "motor_ordem_das_reservas_do_estadio", "motor_clima_da_partida", "motor_temperatura_do_jogo", "motor_gramado_do_jogo", "motor_faixa_de_riqueza", "motor_ordem_por_habilidade", "motor_ordem_pela_capacidade", "motor_programas_padrao", "motor_tipo_do_programa", "motor_programa_do_humano", "motor_fichas_de_treino_iniciais", "motor_guardar_jogador_no_banco", "motor_tem_reserva_de_tecnico", "motor_suspensao_do_vermelho", "motor_nova_temporada_do_orgao", "motor_compactar_disciplina_do_orgao", "motor_coordenada_da_casa", "motor_posicao_repetida_na_formacao", "motor_tipo_de_marcacao", "motor_sortear_ordem_dos_clubes", "motor_clubes_do_lado_do_jogo", "motor_clubes_possiveis_do_jogo", "motor_nota_alvo_do_clube", "motor_jogo_do_historico", "motor_gols_do_historico", "motor_passar_a_oferta_aos_clubes", "motor_noticia_da_oferta_aceita", "motor_noticia_do_emprestimo_oferecido", "motor_recusa_dos_termos", "motor_noticia_da_confianca_da_diretoria", "motor_noticia_das_expectativas_da_diretoria", "motor_ofertas_em_espera_na_janela", "motor_noticia_das_expectativas_da_federacao", "motor_noticia_do_protesto_da_torcida", "motor_guardar_a_negociacao", "motor_noticia_do_boato_do_tecnico", "motor_noticia_da_recusa_em_se_mudar", "motor_receita_do_merchandising", "motor_ingressos_da_temporada", "motor_melhorar_o_centro_de_treino", "motor_eventos_financeiros_da_semana", "motor_diretoria_paga_a_divida", "motor_injecao_de_dinheiro", "motor_teto_salarial_do_pessoal", "motor_pagar_os_bonus", "motor_ajustar_o_teto_salarial", "motor_efeitos_fisicos_da_partida", "motor_batida_do_jogador", "motor_jogador_de_ferias", "motor_contratar_o_regen", "motor_aposentar_pela_lesao", "motor_noticia_da_aposentadoria_anunciada", "motor_fim_da_lesao", "motor_noticia_do_pedido_de_desculpas", "motor_esvaziar_caixa_de_entrada", "motor_tirar_convocado_machucado", "motor_noticia_da_saida_do_convocado", "motor_noticia_do_pedido_de_substituto", "motor_pode_mudar_a_convocacao", "motor_esta_na_convocacao", "motor_marcar_adversario_do_amistoso", "motor_registrar_premio_do_mes", "motor_faixa_de_reputacao_do_amistoso", "motor_folga_do_amistoso", "motor_satisfacao_com_o_contrato", "motor_noticia_quer_contrato_novo", "motor_noticia_pedido_de_transferencia_retirado", "motor_confirmar_oferta_pelo_clube", "motor_tentar_concluir_oferta", "motor_acertar_clausulas_anteriores", "motor_tentar_emprestimo_pelo_jogador", "motor_noticia_contrato_acabando", "motor_noticia_saida_pela_lei_bosman", "motor_acrescentar_oferta_de_contrato", "motor_tirar_oferta_de_contrato_da_vaga", "motor_menor_salario_pedido", "motor_guardar_notas_da_partida", "motor_nota_do_jogador_no_mes", "motor_melhor_do_mes", "motor_premiar_o_melhor_do_mes", "motor_gravar_vencedor_do_mes", "motor_reputacao_do_premiado", "motor_fisico_do_dia", "motor_faixa_da_confianca_na_liga", "motor_gravar_notas_da_diretoria", "motor_dia_das_lesoes", "motor_noticias_de_volta_aos_treinos", "motor_um_cartao_da_suspensao", "motor_fila_de_regens", "motor_veiculo_pelo_clube", "motor_veiculo_pela_pessoa", "motor_somar_resultado_na_liga", "motor_copiar_tabela_da_liga", "motor_posicoes_previstas_da_liga", "motor_resultado_entre_selecoes", "motor_nome_do_elenco_da_selecao", "motor_rodada_com_jogos_conhecidos", "motor_registrar_convocados_por_clube", "motor_importancia_do_jogo_na_competicao", "motor_fase_da_competicao", "motor_faltas_do_dia", "motor_teve_falta_ao_treino", "motor_teve_incidente_de_forma", "motor_marcar_falta_ao_treino", "motor_fim_da_ausencia", "motor_marcas_do_contrato", "motor_recusar_oferta_de_transferencia", "motor_retirar_oferta_de_transferencia", "motor_noticia_oferta_recusada", "motor_desassossegar_pelo_lance", "motor_noticia_oferta_desfeita", "motor_noticia_premio_por_jogos", "motor_efetivar_transferencia", "motor_noticia_transferencia_concluida", "motor_noticia_transferencia_acertada", "motor_emprestimo_da_configuracao", "motor_noticia_fim_do_contrato", "motor_noticia_pedido_de_adiamento", "motor_tatica_do_clube", "motor_escolher_vaga_de_tatica", "motor_tatica_ativa_do_humano", "motor_tirar_da_escalacao", "motor_trocar_vagas_da_escalacao", "motor_tem_copia_da_escalacao", "motor_noticia_da_pressao", "motor_noticia_da_saida_do_tecnico", "motor_noticia_do_tecnico_novo", "motor_pagar_premios_da_liga", "motor_registros_pela_situacao_na_liga", "motor_anunciar_situacao_na_liga", "motor_registrar_partida_nas_estatisticas", "motor_recordes_de_posicao_na_liga", "motor_noticias_de_recordes", "motor_convocar_selecoes_para_o_jogo", "motor_noticia_do_convocado", "motor_noticia_do_desfecho_na_copa", "motor_contar_vagas_com_lance", "motor_tirar_da_lista_de_transferencias", "motor_tirar_da_lista_de_emprestimos", "motor_situacao_na_lista_de_observacao", "motor_por_na_lista_de_emprestimos", "motor_teto_de_preco_pelo_estatuto", "motor_distribuir_verba_de_transferencias", "motor_estatuto_permitido", "motor_nome_da_lista_do_olheiro", "motor_amistosos_internacionais_do_dia", "motor_lista_escolhida_do_tecnico", "motor_escolher_lista_do_tecnico", "motor_inscritos_da_liga_dos_campeoes", "motor_tabela_da_qualificacao_da_liga_dos_campeoes", "motor_proxima_fase_da_liga_dos_campeoes", "motor_processar_jogo_da_liga_dos_campeoes", "motor_progresso_do_clube_na_liga_dos_campeoes", "motor_conta_para_conflito_de_datas", "motor_escolher_arbitro_da_partida", "motor_limpar_arbitros_ocupados", "motor_renovar_arbitros", "motor_arbitros_sorteados", "motor_validar_arbitros", "motor_copiar_ficha_de_arbitro", "motor_registrar_premio_do_ano", "motor_vitoria_maior", "motor_derrota_maior", "motor_jogos_das_permissoes_e_clausulas", "motor_guardar_premio_por_jogos_na_selecao", "motor_noticia_transferencia_finalizada", "motor_veto_dos_fisios_e_da_diretoria", "motor_noticia_do_ultimato", "motor_texto_do_ultimato", "motor_liberar_sem_custo", "motor_ferias_do_humano", "motor_noticia_conversa_de_contrato", "motor_mata_mata_da_first", "motor_mata_mata_da_second", "motor_mata_mata_da_third", "motor_registros_pela_situacao_inglesa", "motor_situacao_na_first", "motor_situacao_na_third", "motor_posicoes_e_situacao_na_liga", "motor_fase_seguinte_da_first", "motor_fase_seguinte_da_second", "motor_registros_da_liga_dos_campeoes", "motor_sorteio_das_quartas", "motor_clubes_da_liga_dos_campeoes", "motor_registrar_resultado_da_tatica", "motor_capitao_guardado", "motor_escolher_capitao", "motor_selecao_que_tem_o_jogador", "motor_jogador_serve_a_selecao", "motor_juntar_competicoes_das_convocacoes", "motor_registro_da_partida_pelo_clube", "motor_referencia_ao_nome", "motor_codigo_do_resultado", "motor_mover_jogos_vizinhos_do_clube", "motor_remarcar_jogos_perto_da_data", "motor_noticia_do_jogo_remarcado", "motor_incidente_de_forma", "motor_noticia_reacao_a_multa", "motor_noticia_retorno_do_treinador", "motor_sortear_comentarista", "motor_nome_do_veiculo", "motor_noticia_queixa_a_federacao", "motor_folha_salarial_do_clube", "motor_refazer_todas_as_folhas", "motor_dias_ate_a_temporada", "motor_receber_venda_de_jogador", "motor_pagar_compra_de_jogador", "motor_pagar_luvas", "motor_receber_premio_em_dinheiro", "motor_receber_direitos_de_tv", "motor_pagar_salarios", "motor_copa_premio_da_rodada", "motor_copa_mandantes_da_rodada", "motor_copa_progresso_do_time", "motor_amistoso_responder_pedido", "motor_amistoso_enviar_pedido", "motor_amistoso_lista_das_nacoes", "motor_copa_sortear_rodada", "motor_nome_da_fase_e_rodada", "motor_clube_da_divisao_mais_alta", "motor_copa_registros_pelo_desfecho", "motor_copa_honras_da_temporada", "motor_copa_semifinalistas_para_a_uefa", "motor_recusar_oferta_de_emprestimo", "motor_vaga_no_elenco_para_a_oferta", "motor_percentual_da_proxima_venda_achado", "motor_jogador_na_busca", "motor_sincronizar_listas_dos_olheiros", "motor_vaga_do_jogador_observado", "motor_valor_do_jogador_para_o_clube", "motor_entrada_do_jogador_na_busca_do_clube", "motor_premio_de_jogos_na_selecao_achado", "motor_contratos_depois_do_jogo", "motor_moral_da_ficha", "motor_texto_da_moral", "motor_valor_do_jogador", "motor_diferenca_de_moral_em_texto", "motor_marcas_de_elenco_para_a_tela", "motor_jogadores_que_a_diretoria_cita", "motor_veredito_da_nota_na_partida", "motor_texto_da_diretoria_sobre_os_jogadores", "motor_resultado_nos_registros_dos_tecnicos", "motor_posicoes_no_fim_da_liga", "motor_jogador_do_ano_da_torcida_no_registro", "motor_vice_da_copa_nos_registros", "motor_terceiro_da_copa_nos_registros", "motor_troca_de_tecnico_nos_registros", "motor_reacoes_dos_elencos_a_transferencia", "motor_noticia_da_comemoracao_da_contratacao", "motor_noticia_do_companheiro_desassossegado", "motor_renda_da_partida", "motor_renda_do_estadio", "motor_pagar_multa_da_liga", "motor_impulso_das_financas", "motor_troca_do_presidente", "motor_pagar_outros_custos", "motor_tabela_das_datas_internacionais", "motor_resolver_datas_internacionais", "motor_torneio_do_clube_humano", "motor_amistosos_dos_clubes", "motor_acrescentar_torneio", "motor_convite_aberto_do_clube_humano", "motor_lesao_nova", "motor_noticia_da_lesao", "motor_ausente_sem_licenca", "motor_parte_dos_clubes_anteriores", "motor_percentual_da_proxima_venda_pago", "motor_noticia_do_percentual_da_venda", "motor_permissao_concedida", "motor_estatuto_pelo_motivo", "motor_vaga_da_negociacao", "motor_reacao_do_elenco_a_historia", "motor_aviso_ao_rival_de_posicao", "motor_rotulo_da_resposta_da_midia", "motor_cumprir_suspensoes_internacionais", "motor_recurso_da_suspensao", "motor_texto_da_vaga_da_suspensao", "motor_tirar_da_lista", "motor_esvaziar_lista", "motor_item_do_cursor", "motor_copa_sortear_pela_semente", "motor_copa_sortear_separando_nacoes", "motor_copa_comeco_da_temporada", "motor_nacao_chave_do_clube", "motor_nacao_chave_do_estadio", "motor_entrada_da_nacao_chave_ativa", "motor_conflito_de_jogos_no_mesmo_dia", "motor_primeiro_ano_com_jogos", "motor_ultimo_ano_com_jogos", "motor_gravar_financas", "motor_carregar_financas", "motor_gravar_gerente_de_elencos", "motor_gravar_sementes_da_uefa", "motor_gravar_sementes_da_conmebol", "motor_gravar_paises_anfitrioes", "motor_gravar_historico_dos_clubes", "motor_gravar_estatisticas_dos_arbitros", "motor_gravar_estatisticas_simuladas", "motor_gravar_datas_dos_clubes", "motor_carregar_datas_dos_clubes", "motor_gravar_conferencia_das_pernas", "motor_fim_de_temporada_da_nacao", "motor_fim_de_temporada_do_clube", "motor_noticia_do_dividendo", "motor_teto_salarial", "motor_noticia_das_obras", "motor_noticia_da_obra_recusada", "motor_conflitos_do_dia", "motor_conflitos_do_jogo", "motor_carregar_conferencia_das_pernas", "motor_nova_data_do_jogo", "motor_jogo_no_calendario", "motor_ano_do_calendario", "motor_copa_criar_jogos_do_confronto", "motor_copa_noticia_do_confronto_pendente", "motor_janelas_avisos_do_dia", "motor_copa_levar_jogos_as_datas", "motor_tirar_jogador_da_troca", "motor_permissoes_da_oferta", "motor_confianca_depois_da_partida", "motor_noticia_da_reacao_da_diretoria", "motor_noticia_da_avaliacao_da_federacao", "motor_expectativas_da_temporada", "motor_vaga_de_tecnico", "motor_noticia_da_oferta_de_emprego", "motor_boatos_de_transferencia", "motor_noticia_nao_vende", "motor_noticia_do_titulo", "motor_amistosos_entre_selecoes", "motor_convite_de_amistoso_valido", "motor_convite_de_torneio_valido", "motor_dia_das_selecoes", "motor_busca_atual_sem_clube", "motor_clube_nas_buscas_da_sessao", "motor_escolher_elenco_da_selecao", "motor_olheiros_na_partida", "motor_sessao_de_edicao", "motor_escolher_sessao_de_edicao", "motor_dia_das_transferencias", "motor_processar_oferta_vencida", "motor_restricoes_da_oferta", "motor_aceitar_lance", "motor_noticia_da_oferta_feita", "motor_pode_emprestar", "motor_liga_dia", "motor_liga_criar_tabela", "motor_liga_promover_e_rebaixar", "motor_calendario_dia", "motor_calendario_lista", "motor_calendario_carregar_ano", "motor_copa_arvore_dos_confrontos", "motor_competicao_primeira_fase_do_grupo", "motor_competicao_fase_atual", "motor_atualizar_o_treino_do_jogador", "motor_programa_de_treino_do_jogador", "motor_nivel_da_categoria_do_programa", "motor_tirar_ou_devolver_o_progresso", "motor_mudanca_do_atributo_pelo_treino", "motor_aptidao_para_a_posicao", "motor_aposentadorias_do_dia", "motor_noticia_da_aposentadoria", "motor_noticia_do_jovem_promovido", "motor_limpar_ausencia", "motor_limpar_falta_ao_treino", "motor_noticia_da_falta_ao_treino", "motor_troca_com_compensacao", "motor_ordem_das_vagas", "motor_marcar_sob_oferta", "motor_campeao_da_liga_nos_registros", "motor_campeao_da_copa_nos_registros", "motor_limpar_registro_do_humano", "motor_resposta_da_ia_ao_lance", "motor_negociacao_de_emprestimo", "motor_opcao_de_troca", "motor_agendar_fim_do_emprestimo", "motor_lances_do_clube", "motor_cqe4_interesse", "motor_cqe4_construir_contrato", "motor_cqe5_pode_emprestar", "motor_cqe5_registrar_emprestimo", "motor_cqe5_dia_das_regras", "motor_cqe6_periodo_inglaterra", "motor_cqe6_pode_emprestar_escocia", "motor_cqe6_periodo_escocia", "motor_cqe6_pode_emprestar_italia", "motor_cqe6_pode_emprestar_irlanda", "motor_cqe6_pode_emprestar_espanha", "motor_liga_jogos_para_a_tv", "motor_liga_noticia_dos_grupos", "motor_liga_noticia_da_tabela", "motor_por_oferta_de_transferencia", "motor_aceitar_emprestimo", "motor_termos_aceitos", "motor_noticia_de_jogador_listado", "motor_contrato_oferecido", "motor_mundial_sortear_grupos", "motor_mundial_sorteio_dos_grupos", "motor_mundial_dia", "motor_mundial_tabela_do_grupo_um", "motor_mundial_fase_do_grupo", "motor_mundial_posicoes_previstas", "motor_mundial_rodadas_da_fase", "motor_mundial_nacao_classificada", "motor_mundial_cabecas_de_chave", "motor_distribuir_os_treinadores", "motor_trocar_os_programas_do_elenco", "motor_nomes_padrao_dos_programas", "motor_historico_da_carreira", "motor_anos_desde_o_clube", "motor_ordem_do_historico_por_pessoa", "motor_lesao_pelo_nome", "motor_noticia_da_volta_aos_treinos", "motor_nome_da_categoria_da_lesao", "motor_cqf10_olheiro_no_jogo", "motor_cqf10_historico_do_clube_observado", "motor_cqf11_relatorio_da_selecao", "motor_cqf11_noticia_da_convocacao", "motor_cqf11_adversarios_da_selecao", "motor_cqf12_olheiros_do_dia", "motor_cqf12_relatorios_dos_olheiros", "motor_cqf12_noticia_do_relatorio", "motor_assinar_contrato", "motor_refazer_como_jovem", "motor_tirar_da_disputa", "motor_ler_formacao_do_disco", "motor_trocar_bytes_das_palavras", "motor_trocar_bytes_dos_dwords", "motor_montar_os_arbitros", "motor_ordem_das_vagas_de_competicao", "motor_cqg10_amistoso_cancelado", "motor_cqg10_jogo_da_selecao_cancelado", "motor_cqg10_torneio_cancelado", "motor_ordem_dos_arbitros_decrescente", "motor_ordem_dos_arbitros_crescente", "motor_arquivar_estatisticas", "motor_pessoa_pelo_nome", "motor_texto_entre_aspas", "motor_tipo_da_linha", "motor_campeoes_sortear_grupos", "motor_campeoes_ordem_dos_potes", "motor_campeoes_posicoes_previstas", "motor_campeoes_sortear_segunda_fase", "motor_campeoes_ordem_da_segunda_fase", "motor_grupo_da_fase", "motor_campeoes_rodadas_da_fase", "motor_liga_expectativas_da_temporada", "motor_liga_rebaixar_clube", "motor_competicao_todas_completas", "motor_competicao_lista_de_clubes", "motor_competicao_progresso_do_clube", "motor_uefa_rodar_coeficientes", "motor_uefa_vagas_por_nacao", "motor_uefa_enesimo_clube_da_nacao", "motor_uefa_juntar_clubes_europeus", "motor_uefa_vagas_liga_dos_campeoes", "motor_uefa_vagas_intertoto", "motor_uefa_vagas_copa_da_uefa", "motor_uefa_voluntario_ingles", "motor_uefa_virada_da_classificacao", "motor_conmebol_vagas_libertadores", "motor_conmebol_enesimo_clube_da_nacao", "motor_conmebol_convidado_da_mercosul", "motor_conmebol_juntar_clubes", "motor_conmebol_virada_do_ano", "motor_conmebol_campeao_dos_campeoes", "motor_sede_da_edicao", "motor_proxima_sede", "motor_calendario_das_sedes", "motor_regras_inglaterra", "motor_regras_eua", "motor_regras_australia", "motor_janelas_espanha", "motor_janelas_italia", "motor_janelas_franca", "motor_regras_brasil", "motor_janelas_alemanha", "motor_regras_escocia", "motor_cqi10_intertoto_dia", "motor_cqi10_intertoto_registros", "motor_cqi10_uefa_registros", "motor_cqi11_concacaf_classificados", "motor_cqi11_concacaf_dia", "motor_cqi11_concacaf_rodadas", "motor_cqi12_copa_italia_regras", "motor_cqi12_copa_c_regras", "motor_cqi12_copa_dos_campeoes_regras", "motor_cqj10_copa_africana_registros", "motor_cqj10_copa_america_registros", "motor_cqj10_eliminatoria_oceania_registros", "motor_cqj11_libertadores_regras", "motor_cqj11_merconorte_regras", "motor_cqj11_oceania_clubes_regras", "motor_cqj12_eliminatoria_europa_regras", "motor_cqj12_eliminatoria_sul_americana_regras", "motor_cqj12_eliminatoria_asia_regras", "motor_cqk10_copa_america_regras", "motor_cqk10_copa_africana_regras", "motor_cqk10_eliminatoria_oceania_regras", "motor_cqk11_libertadores_classificados", "motor_cqk11_libertadores_registros", "motor_cqk11_libertadores_fase_seguinte", "motor_cqk12_africana_fase_seguinte", "motor_cqk12_africana_grupo_a", "motor_cqk12_africana_historico", "motor_cqi7_regras_da_serie_a", "motor_cqi7_regras_da_serie_b", "motor_cqi7_regras_da_serie_c", "motor_cqi8_regras_do_centro_oeste", "motor_cqi8_regras_do_gaucho", "motor_cqi8_regras_do_catarinense", "motor_cqi9_regras_do_paranaense", "motor_cqi9_regras_do_pernambucano", "motor_cqi9_regras_do_carioca", "motor_cqj7_regras_do_goiano", "motor_cqj7_regras_da_j1", "motor_cqj7_regras_da_j2", "motor_cqj8_regras_da_nsl", "motor_cqj8_regras_da_mls", "motor_cqj8_historico_do_estadual", "motor_cqj8_historico_do_primeiro_estagio", "motor_cqj9_destino_na_serie_a", "motor_cqj9_destino_na_serie_c", "motor_cqj9_destino_no_baiano", "motor_cqk7_destino_no_centro_oeste", "motor_cqk7_destino_no_mineiro", "motor_cqk7_destino_na_nsl", "motor_cqk8_progresso_no_mineiro", "motor_cqk8_progresso_na_mls", "motor_cqk8_progresso_na_serie_c", "motor_cqk9_destino_na_mls", "motor_cqk9_resultado_na_mls", "motor_cqk9_conferencia_das_estrelas", "motor_regras_da_premier", "motor_regras_da_first", "motor_regras_da_second", "motor_regras_da_third", "motor_tabela_da_premier", "motor_tabela_da_second", "motor_tabela_da_third", "motor_tabela_da_first", "motor_regras_da_serie_a", "motor_regras_da_serie_b", "motor_regras_da_c1a", "motor_regras_da_c1b", "motor_tabela_da_serie_a", "motor_tabela_da_serie_b", "motor_tabela_da_c1a", "motor_tabela_da_c1b", "motor_regras_da_franca_1", "motor_regras_da_franca_2", "motor_regras_da_franca_3", "motor_tabela_da_franca_1", "motor_tabela_da_franca_2", "motor_tabela_da_franca_3", "motor_regras_da_primera", "motor_regras_da_segunda", "motor_regras_da_divisao_de_baixo", "motor_tabela_da_primera", "motor_registros_pela_situacao_na_c2a", "motor_situacao_na_primera", "motor_tabela_do_grupo_da_cfa", "motor_tabela_da_divisao_de_baixo_francesa", "motor_situacao_na_c1a", "motor_situacao_na_c1b", "motor_cql10_copa_da_liga_tabela", "motor_cql10_copa_da_inglaterra_tabela", "motor_cql10_supercopa_tabela", "motor_cql11_copa_da_franca_tabela", "motor_cql11_copa_da_liga_francesa_tabela", "motor_cql11_copa_do_rei_tabela", "motor_cql12_supercopa_da_franca_tabela", "motor_cql12_supercopa_da_espanha_tabela", "motor_cql12_supercopa_do_japao_tabela", "motor_cqm10_copa_da_liga_inscritos", "motor_cqm10_trofeu_inscritos", "motor_cqm10_supercopa_clubes", "motor_cqm11_copa_da_franca_inscritos", "motor_cqm11_copa_da_liga_inscritos", "motor_cqm11_supercopa_clubes", "motor_cqm12_supercopa_da_espanha_clubes", "motor_cqm12_supercopa_do_japao_clubes", "motor_cqm12_jogo_das_estrelas_tabela", "motor_cqn10_copa_da_liga_dia", "motor_cqn10_copa_da_inglaterra_dia", "motor_cqn10_jogo_das_estrelas_dia", "motor_cqn11_copa_da_liga_japonesa_tabela", "motor_cqn11_copa_do_imperador_tabela", "motor_cqn11_copa_dos_eua_tabela", "motor_cqn12_copa_da_liga_inscritos", "motor_cqn12_copa_dos_eua_inscritos", "motor_cqn12_copa_do_imperador_inscritos", "motor_numero_de_camisa", "motor_clube_usa_numeros", "motor_gravar_numero_de_camisa", "motor_dar_numero_de_camisa", "motor_numerar_elenco", "motor_numerar_no_comeco", "motor_numerar_liga", "motor_inscrever_na_europa", "motor_regras_holanda", "motor_regras_croacia", "motor_regras_irlanda", "motor_regras_noruega", "motor_regras_irlanda_do_norte", "motor_regras_pais_de_gales", "motor_regras_turquia", "motor_regras_polonia", "motor_regras_finlandia", "motor_janelas_argentina", "motor_janelas_portugal", "motor_regras_coreia", "motor_janelas_grecia", "motor_janelas_russia", "motor_janelas_belgica", "motor_inscricao_europeia", "motor_ranking_fifa_inicial", "motor_lista_de_estadios_neutros", "motor_cql7_tabela_da_serie_a", "motor_cql7_tabela_da_serie_b", "motor_cql7_tabela_da_serie_c", "motor_cql8_tabela_do_baiano", "motor_cql8_tabela_do_centro_oeste", "motor_cql8_tabela_do_gaucho", "motor_cql9_tabela_do_goiano", "motor_cql9_tabela_do_mineiro", "motor_cql9_tabela_do_norte", "motor_cqm7_tabela_do_nordeste", "motor_cqm7_tabela_do_paranaense", "motor_cqm7_tabela_do_pernambucano", "motor_cqm8_tabela_do_carioca", "motor_cqm8_tabela_do_catarinense", "motor_cqm8_tabela_do_paulista", "motor_cqm9_tabela_da_j1", "motor_cqm9_tabela_da_j2", "motor_cqm9_tabela_da_nsl", "motor_cqn7_mata_mata_do_baiano", "motor_cqn7_mata_mata_do_norte", "motor_cqn7_mata_mata_do_nordeste", "motor_cqn8_mata_mata_do_pernambucano", "motor_cqn8_mata_mata_do_catarinense", "motor_cqn8_mata_mata_do_paulista", "motor_cqn9_mata_mata_do_gaucho", "motor_cqn9_mata_mata_do_centro_oeste", "motor_cqn9_mata_mata_do_goiano", "motor_desempate_da_serie_a", "motor_desempate_da_serie_b", "motor_ordem_dos_empatados_na_italia", "motor_resultado_do_desempate_da_serie_a", "motor_fase_seguinte_da_c1a", "motor_fases_do_fim_da_c1a", "motor_fase_seguinte_da_c1b", "motor_fases_do_fim_da_c1b", "motor_fase_seguinte_da_third", "motor_construir_premier", "motor_construir_first", "motor_construir_second", "motor_construir_third", "motor_construir_serie_a", "motor_construir_serie_b", "motor_construir_c1a", "motor_construir_c1b", "motor_construir_franca_2", "motor_construir_franca_3", "motor_construir_segunda", "motor_construir_primera", "motor_construir_divisao_de_baixo_espanhola", "motor_tabela_da_segunda", "motor_tabela_da_divisao_de_baixo_espanhola", "motor_regras_da_cfa", "motor_construir_cfa", "motor_construir_divisao_de_baixo_francesa", "motor_destruir_cfa", "motor_destruir_divisao_de_baixo_francesa", "motor_calendario_fixo_da_serie_a", "motor_calendario_fixo_da_serie_b", "motor_calendario_fixo_da_c1a", "motor_calendario_fixo_da_c1b", "motor_toco_um_com_dois", "motor_toco_um_com_um", "motor_estadio_da_partida", "motor_estadio_neutro", "motor_publico_do_jogo", "motor_calendario_da_terceira_francesa", "motor_calendario_da_segunda_francesa", "motor_calendario_da_divisao_de_baixo_francesa", "motor_calendario_da_cfa", "motor_calendario_da_c1_a", "motor_calendario_da_c1_b", "motor_calendario_da_serie_a", "motor_calendario_da_serie_b", "motor_calendario_da_primeira_francesa", "motor_cqo7_fase_seguinte_do_mineiro", "motor_cqo7_final_do_mineiro", "motor_cqo7_fim_da_temporada_do_mineiro", "motor_cqo8_fase_seguinte_do_paranaense", "motor_cqo8_final_do_paranaense", "motor_cqo8_fim_da_temporada_do_paranaense", "motor_cqo9_fase_seguinte_do_carioca", "motor_cqo9_final_do_carioca", "motor_cqo9_fim_da_temporada_do_carioca", "motor_cqo10_copa_da_italia_tabela", "motor_cqo10_copa_c_tabela", "motor_cqo10_supercopa_tabela", "motor_cqo10_supercopa_c1_tabela", "motor_cqo11_copa_da_italia_inscritos", "motor_cqo11_copa_c_inscritos", "motor_cqo11_supercopa_clubes", "motor_cqo12_copa_do_brasil_tabela", "motor_cqo12_times_da_copa", "motor_cqo12_regras_da_copa", "motor_cqo13_libertadores_tabela", "motor_cqo13_mercosul_tabela", "motor_cqo13_mercosul_convidados", "motor_calendario_da_segunda_inglesa", "motor_calendario_da_terceira_inglesa", "motor_calendario_da_primeira_inglesa", "motor_calendario_da_premiership", "motor_calendario_da_primera", "motor_cqp7_virada_do_baiano", "motor_cqp7_fase_seguinte_do_baiano", "motor_cqp7_virada_do_centro_oeste", "motor_cqp7_fase_seguinte_do_centro_oeste", "motor_cqp7_virada_do_gaucho", "motor_cqp7_fase_seguinte_do_gaucho", "motor_cqp8_virada_do_goiano", "motor_cqp8_fase_seguinte_do_goiano", "motor_cqp8_virada_do_norte", "motor_cqp8_fase_seguinte_do_norte", "motor_cqp8_virada_do_nordeste", "motor_cqp8_fase_seguinte_do_nordeste", "motor_cqp9_virada_do_pernambucano", "motor_cqp9_fase_seguinte_do_pernambucano", "motor_cqp9_virada_do_catarinense", "motor_cqp9_fase_seguinte_do_catarinense", "motor_cqp9_virada_do_paulista", "motor_cqp9_fase_seguinte_do_paulista", "motor_calendario_da_segunda_espanhola", "motor_calendario_da_divisao_de_baixo_espanhola", "motor_construir_primeira_francesa", "motor_cqq7_fase_seguinte_da_serie_a", "motor_cqq7_play_off_da_serie_a", "motor_cqq7_fase_seguinte_da_serie_b", "motor_cqq7_play_off_da_serie_b", "motor_cqq8_virada_da_serie_a", "motor_cqq8_acesso_e_rebaixamento", "motor_cqq8_virada_da_serie_b", "motor_cqq9_fase_seguinte_da_j1", "motor_cqq9_final_da_j1", "motor_cqq9_virada_da_j1", "motor_tabela_das_sedes", "motor_noticia_das_sedes", "motor_carregar_sedes", "motor_cadeia_de_acessos", "motor_noticia_do_premio", "motor_noticia_da_tv", "motor_cqp10_copa_dos_campeoes_tabela", "motor_cqp10_merconorte_tabela", "motor_cqp10_supercopa_europeia_tabela", "motor_cqp10_supercopa_asiatica_tabela", "motor_cqp11_asia_clubes_tabela", "motor_cqp11_asia_recopa_tabela", "motor_cqp11_trofeu_tabela", "motor_cqp12_copa_america_tabela", "motor_cqp12_africana_tabela", "motor_cqp12_confed_tabela", "motor_cqp13_intertoto_tabela", "motor_cqp13_intercontinental_tabela", "motor_cqp13_interamericana_tabela", "motor_cqp13_concacaf_tabela", "motor_cqq10_uefa_tabela", "motor_cqq10_intertoto_inscritos", "motor_cqq10_intertoto_lista", "motor_cqq11_europeia_clubes", "motor_cqq11_asiatica_clubes", "motor_cqq11_c1_vagas", "motor_cqq12_oceania_tabela", "motor_cqq12_asia_clubes_times", "motor_cqq12_asia_recopa_times", "motor_cqr1_turno_do_carioca", "motor_cqr1_turno_do_mineiro", "motor_cqr1_turno_do_paranaense", "motor_cqr1_turno_da_j1", "motor_cqr2_virada_do_carioca", "motor_cqr2_virada_do_mineiro", "motor_cqr2_virada_do_paranaense", "motor_cqr3_fase_seguinte_da_nsl", "motor_cqr3_serie_final_da_nsl", "motor_cqr3_grande_final_da_nsl", "motor_cqr4_fase_seguinte_da_mls", "motor_cqr4_play_off_da_mls", "motor_cqr4_final_da_mls", "motor_cqr5_virada_copa_do_brasil", "motor_cqr5_virada_copa_do_imperador", "motor_cqr5_virada_copa_da_liga_japonesa", "motor_cqr6_virada_supercopa_japonesa", "motor_cqr6_virada_copa_dos_eua", "motor_cqr6_virada_jogo_das_estrelas", "motor_cqr6_times_do_jogo_das_estrelas", "motor_cqr10_libertadores_dia", "motor_cqr10_libertadores_primeiro_grupo", "motor_cqr10_libertadores_grupo", "motor_cqr11_mercosul_dia", "motor_cqr11_mercosul_primeiro_grupo", "motor_cqr11_mercosul_grupo", "motor_cqr12_merconorte_dia", "motor_cqr12_merconorte_primeiro_grupo", "motor_cqr12_merconorte_grupo", "motor_cqr13_copa_america_dia", "motor_cqr13_copa_america_primeiro_grupo", "motor_cqr13_copa_america_grupo", "motor_cqs1_tabela_da_mls", "motor_cqs1_segunda_divisao_da_mls", "motor_cqs1_terceira_divisao_da_mls", "motor_cqs2_virada_da_mls", "motor_cqs2_posicoes_previstas_da_mls", "motor_cqs2_clubes_da_mls", "motor_cqs3_grupo_da_serie_c", "motor_cqs3_segunda_fase_da_serie_c", "motor_cqs3_terceira_fase_da_serie_c", "motor_cqs4_final_da_serie_c", "motor_cqs4_virada_da_serie_c", "motor_cqs4_posicoes_previstas_da_serie_c", "motor_cqs5_trofeu_clubes", "motor_cqs5_trofeu_fase_das_chaves", "motor_cqs5_trofeu_fase_seguinte", "motor_cqs6_trofeu_final", "motor_cqs6_trofeu_classificacao", "motor_cqs7_fase_de_copa", "motor_cqs7_fim_da_fase_de_copa", "motor_cqs10_libertadores_mata_mata", "motor_cqs10_merconorte_semifinais", "motor_cqs10_merconorte_fase_seguinte", "motor_cqs11_merconorte_convidados", "motor_cqs11_mercosul_fase_seguinte", "motor_cqs11_copa_america_fase_seguinte", "motor_cqs12_copa_america_selecoes", "motor_cqs12_copa_america_previstas", "motor_cqs12_copa_america_temporada_nova", "motor_cqt1_construir_baiano", "motor_cqt1_construir_centro_oeste", "motor_cqt1_construir_gaucho", "motor_cqt2_construir_goiano", "motor_cqt2_construir_mineiro", "motor_cqt2_construir_norte", "motor_cqt3_construir_nordeste", "motor_cqt3_construir_paranaense", "motor_cqt3_construir_pernambucano", "motor_cqt4_construir_carioca", "motor_cqt4_construir_catarinense", "motor_cqt4_construir_paulista", "motor_cqt4_atualizar_media_de_publico", "motor_cqt10_oceania_dia", "motor_cqt10_oceania_grupo_a", "motor_cqt10_oceania_grupo_b", "motor_cqt11_oceania_clubes", "motor_cqt11_oceania_fase_seguinte", "motor_cqt11_oceania_semifinais", "motor_cqt12_asia_fase_seguinte", "motor_cqt12_asia_grupos", "motor_cqt12_asia_semifinais", "motor_cqs13_africana_dia", "motor_cqs13_africana_sorteio", "motor_cqs13_africana_grupo", "motor_cqt13_africana_quartas", "motor_cqt13_recopa_asiatica_fase_seguinte", "motor_cqt13_recopa_asiatica_finais", "motor_cqr7_copa_da_italia_grupo_um", "motor_cqr7_copa_da_serie_c_grupo_um", "motor_cqr7_copa_da_italia_fase_seguinte", "motor_cqr7_copa_da_serie_c_fase_seguinte", "motor_cqr8_copa_da_italia_grupo", "motor_cqr8_copa_da_serie_c_grupo", "motor_cqr9_copa_da_italia_mata_mata", "motor_cqr9_copa_da_serie_c_mata_mata", "motor_cqs8_eurocopa_regras", "motor_cqs8_eurocopa_tamanho_do_grupo", "motor_cqs8_eurocopa_situacao", "motor_cqs8_eurocopa_grupo_um", "motor_cqs9_uefa_inscritos", "motor_cqt5_copa_do_rei_dia", "motor_cqt5_historico_liga_dos_campeoes", "motor_cqt5_historico_copa_da_grecia", "motor_cqt5_historico_copa_da_liga_irlandesa", "motor_cqt5_historico_copa_da_serie_c",
    ];
    for i in 0..N {
        if DESLIGADAS.contains(&PROLOGOS[i].endereco) {
            continue;
        }
        if SEM_ROTINAS.contains(&nomes[i]) {
            // a original pelo mesmo desvio de passagem do modo original (mesma pilha particular)
            linha(&format!("motor: {} fica no original (SEM_ROTINAS)", nomes[i]));
            match ZERADAS.iter().find(|z| z.0.endereco == PROLOGOS[i].endereco) {
                Some(z) => desvios::desviar_zerando(nomes[i], &PROLOGOS[i], z.1, desvios::PASSAGEM),
                None => desvios::desviar(nomes[i], &PROLOGOS[i], desvios::PASSAGEM),
            };
            continue;
        }
        // na comparacao a 0x6f5c30 e a 0x70dfd0 ja tem a porta (instalada antes), que tambem as compara
        if comparando && (PROLOGOS[i].endereco == 0x6f5c30 || PROLOGOS[i].endereco == 0x70dfd0 || PROLOGOS[i].endereco == 0x867560) {
            continue;
        }
        if !comparando && SO_NA_COMPARACAO.contains(&PROLOGOS[i].endereco) {
            linha(&format!("motor: {} roda a original fora da comparacao (SO_NA_COMPARACAO)", nomes[i]));
            desvios::desviar(nomes[i], &PROLOGOS[i], desvios::PASSAGEM);
            continue;
        }
        let destino = if comparando { comp[i] } else { rust[i] };
        // rotina do original determinizado: o desvio zera a pilha antes (ver ZERADAS)
        let feito = match ZERADAS.iter().find(|z| z.0.endereco == PROLOGOS[i].endereco) {
            Some(z) => desvios::desviar_zerando(nomes[i], &PROLOGOS[i], z.1, destino),
            None => desvios::desviar(nomes[i], &PROLOGOS[i], destino),
        };
        // recusado (rotina alterada por outro patch): orig(i) chama o proprio exe, que segue sem desvio
        ORIGINAIS[i] = feito.unwrap_or(PROLOGOS[i].endereco);
    }
    if comparando {
        linha("motor: modo comparacao (original x Rust)");
    } else if let Some(p) = std::env::var("CM_DIAG_PESSOA").ok().and_then(|v| v.parse::<usize>().ok()) {
        // diagnóstico (31/10/2026, prova do "Deixar infeliz" do CMRTE): a queixa nova e a marca nova da pessoa P,
        // com os campos do contrato antes e depois (o pedido 0x4d8610 do clube humano tem a própria linha)
        DIAG_PESSOA = p;
        DIAG_CLUBE = std::env::var("CM_DIAG_CLUBE").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX);
        DIAG_QUEIXA = desvios::desviar("diag queixa 0x867700", &Prologo { endereco: 0x867700, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] }, diag_queixa as *const () as usize).unwrap_or(0);
    }
}

static mut DIAG_PESSOA: usize = usize::MAX;
static mut DIAG_QUEIXA: usize = 0;
/// CM_DIAG_CLUBE=C: também todo contrato do clube C (+4), para achar quem do clube recebe queixa ou marca
static mut DIAG_CLUBE: usize = usize::MAX;
static mut DIAG_MARCA: usize = 0;

unsafe fn diag_contrato(c: usize) -> String {
    let b = |o: usize| *((c + o) as *const u8);
    let d = |o: usize| core::ptr::read_unaligned((c + o) as *const u32);
    format!("+0x44 {} +0x45 {:#x} +0x40 {:#x} +0x49 {} +0x4e {:#x} +0x4f {:#x}", b(0x44) as i8, d(0x45), d(0x40), b(0x49), b(0x4e), b(0x4f))
}

static mut DIAG_CHAMADAS: u32 = 0;

/// A cada potência de 2 de chamadas (de qualquer pessoa): a pessoa do contrato da vez e o contrato da pessoa P lido
/// direto na memória (0x4cf5a0 sobre [0xae1050], pessoa = [0xae23bc] + id × 110), para conferir a gravação do save.
unsafe fn diag_amostra(c: usize, qual: &str) {
    DIAG_CHAMADAS += 1;
    if !DIAG_CHAMADAS.is_power_of_two() {
        return;
    }
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let p = (*(0xae23bc as *const usize)).wrapping_add(DIAG_PESSOA.wrapping_mul(110));
    let k = achar(*(0xae1050 as *const usize), p);
    linha(&format!("diag {} chamada {}: contrato da vez +0 {} +4 {}; contrato da pessoa {}: {}", qual, DIAG_CHAMADAS,
        core::ptr::read_unaligned(c as *const i32), core::ptr::read_unaligned((c + 4) as *const i32), DIAG_PESSOA,
        if k == 0 { "nenhum".to_string() } else { format!("+0 {} +4 {} {}", core::ptr::read_unaligned(k as *const i32), core::ptr::read_unaligned((k + 4) as *const i32), diag_contrato(k)) }));
}

unsafe extern "thiscall" fn diag_queixa(c: usize, f: u32, z: u32) -> u32 {
    if c != 0 {
        diag_amostra(c, "queixa");
    }
    let alvo = c != 0 && (core::ptr::read_unaligned(c as *const u32) as usize == DIAG_PESSOA || core::ptr::read_unaligned((c + 4) as *const u32) as usize == DIAG_CLUBE);
    if alvo {
        linha(&format!("diag queixa pessoa {} queixa {:#x} z {} antes: {}", core::ptr::read_unaligned(c as *const u32), f, z, diag_contrato(c)));
    }
    let r = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(DIAG_QUEIXA)(c, f, z);
    if alvo {
        linha(&format!("diag queixa pessoa {} devolveu {} depois: {}", core::ptr::read_unaligned(c as *const u32), r, diag_contrato(c)));
    }
    // CM_DIAG_FORCAR=Q: uma queixa sintética Q no contrato da pessoa P, uma vez, logo depois da primeira queixa do jogo
    // (o mesmo evento nas variantes do save: só o estado do contrato decide se o pedido sai)
    static mut FORCADA: bool = false;
    if !FORCADA {
        if let Some(q) = std::env::var("CM_DIAG_FORCAR").ok().and_then(|v| u32::from_str_radix(v.trim_start_matches("0x"), 16).ok()) {
            FORCADA = true;
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let k = achar(*(0xae1050 as *const usize), (*(0xae23bc as *const usize)).wrapping_add(DIAG_PESSOA.wrapping_mul(110)));
            if k != 0 {
                linha(&format!("diag queixa forcada {:#x} pessoa {} antes: {}", q, DIAG_PESSOA, diag_contrato(k)));
                let r2 = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(DIAG_QUEIXA)(k, q, 0);
                linha(&format!("diag queixa forcada devolveu {} depois: {}", r2, diag_contrato(k)));
            }
        }
    }
    r
}

unsafe extern "thiscall" fn diag_marca(c: usize, f: u32) -> u32 {
    if c != 0 {
        diag_amostra(c, "marca");
    }
    let alvo = c != 0 && (core::ptr::read_unaligned(c as *const u32) as usize == DIAG_PESSOA || core::ptr::read_unaligned((c + 4) as *const u32) as usize == DIAG_CLUBE);
    if alvo {
        linha(&format!("diag marca pessoa {} marca {:#x} antes: {}", core::ptr::read_unaligned(c as *const u32), f, diag_contrato(c)));
    }
    let r = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(DIAG_MARCA)(c, f);
    if alvo {
        linha(&format!("diag marca pessoa {} devolveu {} depois: {}", core::ptr::read_unaligned(c as *const u32), r, diag_contrato(c)));
    }
    r
}

/// Original determinizado (15/10/2026, aprovado pelo Lucas): rotinas originais que leem variavel
/// local sem valor (a sobra da pilha muda com o modo) recebem, em todos os modos, um desvio que zera
/// a pilha abaixo da entrada antes de rodar. A carreira de referencia passa a ser a da original com
/// essas zeragens; as copias Rust usam zero. 0x6dbc50: o curto da 1a vaga extra na escalacao;
/// 0x6d1cc0: a media (double) de um lado sem avaliados; 0x6beeb0: bytes altos dos dwords passados as
/// escolhas da simulacao rapida; 0x70c040: o rebote e a saida da defesa quando o proprio goleiro
/// chuta.
/// Desde 31/10/2026 (Lucas) não há zeragem: o critério passou a ser "resultado que o jogo produz" (a comparação rotina a rotina,
/// sem fixar valor), e as 8 zeragens (0x6dbc50, 0x6d1cc0, 0x6beeb0, 0x70c040, 0x7109f0, 0x52dad0, 0x82e430, 0x5fb810) saíram.
const ZERADAS: [(Prologo, u32); 0] = [];

pub unsafe fn instalar_zeradas() {
    // as que tem copia Rust ja foram desviadas (zerando) junto com o motor
    for (p, n) in ZERADAS.iter().filter(|z| !PROLOGOS.iter().any(|q| q.endereco == z.0.endereco)) {
        desvios::desviar_zerando(&format!("zerada {:#x}", p.endereco), p, *n, desvios::PASSAGEM);
    }
}

/// Simulacao no modo original: as mesmas rotinas desviadas para a propria original (pilha particular).
pub unsafe fn instalar_passagem() {
    let (sonda, dentro, prologo) = crate::sonda_pilha::alvo();
    if sonda != 0 && prologo >= 5 && !PROLOGOS.iter().any(|p| p.endereco == sonda) {
        // rotina fora da tabela: o prólogo vem do próprio exe (o tamanho, da variável)
        let bytes: &'static [u8] = Box::leak(core::slice::from_raw_parts(sonda as *const u8, prologo).to_vec().into_boxed_slice());
        desvios::desviar_sonda("sonda de pilha", &Prologo { endereco: sonda, bytes }, dentro);
    }
    for (i, p) in PROLOGOS.iter().enumerate() {
        if p.endereco == sonda {
            desvios::desviar_sonda(&format!("motor {} (sonda de pilha)", i), p, dentro);
            continue;
        }
        if DESLIGADAS.contains(&p.endereco) {
            continue;
        }
        match ZERADAS.iter().find(|z| z.0.endereco == p.endereco) {
            Some(z) => desvios::desviar_zerando(&format!("motor {} (original isolada, zerada)", i), p, z.1, desvios::PASSAGEM),
            None => desvios::desviar(&format!("motor {} (original isolada)", i), p, desvios::PASSAGEM),
        };
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
