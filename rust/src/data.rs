//! Datas do jogo (`si/code/Date.cpp`). Classe de 8 bytes:
//!   +0 i16 dia do ano, +2 u16 ano, +4 i32 bissexto (0/1).
//! Metodos thiscall (o objeto vem em ecx):
//!   { 0x54a110 } construtor padrao: dia 40 de 1972, bissexto;
//!   { 0x549ef0 } construtor (dia, ano): calcula o bissexto (regra gregoriana sobre o ano de 16 bits);
//!   { 0x54a3f0 } dia da semana (0..6), contado a partir de 1600.

use crate::comparacao::{Placar, Sorteio};
use crate::desvios::{self, Prologo};
use crate::registro::linha;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Data {
    pub dia: i16,
    pub ano: u16,
    pub bissexto: i32,
}

// 0x54a110: mov eax, ecx; mov word ptr [eax], 0x28
const PROLOGO_PADRAO: Prologo = Prologo { endereco: 0x54a110, bytes: &[0x8b, 0xc1, 0x66, 0xc7, 0x00, 0x28, 0x00] };
// 0x549ef0: mov ax, word ptr [esp + 4]
const PROLOGO_NOVA: Prologo = Prologo { endereco: 0x549ef0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] };
// 0x54a3f0: push esi; push edi; xor edi, edi; mov eax, 0x51eb851f
const PROLOGO_SEMANA: Prologo = Prologo { endereco: 0x54a3f0, bytes: &[0x56, 0x57, 0x33, 0xff, 0xb8, 0x1f, 0x85, 0xeb, 0x51] };

/// Regra do { 0x549ef0 }: impar ou nao divisivel por 4 -> nao; nao divisivel por 100 -> sim;
/// senao, so se divisivel por 400.
pub fn bissexto(ano: u16) -> i32 {
    let a = ano as i32;
    if a % 2 == 1 || a % 4 != 0 {
        0
    } else if a % 100 != 0 {
        1
    } else {
        (a % 400 == 0) as i32
    }
}

/// { 0x54a110 }
pub unsafe extern "thiscall" fn data_padrao(this: *mut Data) -> *mut Data {
    *this = Data { dia: 40, ano: 1972, bissexto: 1 };
    this
}

/// { 0x549ef0 } Os dois argumentos ocupam 4 bytes na pilha; so os 16 bits de baixo contam.
pub unsafe extern "thiscall" fn data_nova(this: *mut Data, dia: u32, ano: u32) -> *mut Data {
    let ano = ano as u16;
    *this = Data { dia: dia as u16 as i16, ano, bissexto: bissexto(ano) };
    this
}

/// { 0x54a3f0 } Dias desde 1600 (anos * 365 + bissextos) + dia do ano, menos 1 se o ano e
/// bissexto, modulo 7 com sinal; 1..6 vira 0..5, 0 vira 6 e negativo fica como esta.
pub unsafe extern "thiscall" fn dia_da_semana(this: *const Data) -> i32 {
    let d = *this;
    let y = d.ano as i32 - 1600;
    let mut dias = y / 400 + y * 365 - y / 100 + y / 4 + d.dia as i32;
    if d.bissexto != 0 {
        dias -= 1;
    }
    let r = dias % 7;
    if r > 0 {
        r - 1
    } else if r == 0 {
        6
    } else {
        r
    }
}

// ---------- comparacao com as originais ----------

static mut ORIG_PADRAO: usize = 0;
static mut ORIG_NOVA: usize = 0;
static mut ORIG_SEMANA: usize = 0;
static PLACAR: Placar = Placar::novo("data");

unsafe extern "thiscall" fn comparar_padrao(this: *mut Data) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data) -> *mut Data = core::mem::transmute(ORIG_PADRAO);
    let r = f(this);
    let mut nosso = Data::default();
    data_padrao(&mut nosso);
    PLACAR.registrar(r == this && *this == nosso, || format!("padrao original={:?} rust={:?}", *this, nosso));
    r
}

unsafe extern "thiscall" fn comparar_nova(this: *mut Data, dia: u32, ano: u32) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, u32, u32) -> *mut Data = core::mem::transmute(ORIG_NOVA);
    let r = f(this, dia, ano);
    let mut nosso = Data::default();
    data_nova(&mut nosso, dia, ano);
    PLACAR.registrar(r == this && *this == nosso, || format!("nova({:#x}, {:#x}) original={:?} rust={:?}", dia, ano, *this, nosso));
    r
}

unsafe extern "thiscall" fn comparar_semana(this: *const Data) -> i32 {
    let f: unsafe extern "thiscall" fn(*const Data) -> i32 = core::mem::transmute(ORIG_SEMANA);
    let r = f(this);
    let nosso = dia_da_semana(this);
    PLACAR.registrar(r == nosso, || format!("semana({:?}) original={} rust={}", *this, r, nosso));
    r
}

unsafe fn rodar_autoteste() {
    let mut s = Sorteio(0x1234_5678_9abc_def1);
    PLACAR.autoteste(|| {
        // Construtores: todos os anos de 16 bits, com lixo nos 16 bits de cima dos argumentos.
        for ano in 0..=0xffffu32 {
            let lixo = s.proximo() as u32;
            let mut d = Data::default();
            comparar_nova(&mut d, (lixo & 0xffff_0000) | (lixo >> 7 & 0xffff), (lixo << 16) | ano);
        }
        let mut d = Data { dia: -1, ano: 0, bissexto: 7 };
        comparar_padrao(&mut d);
        // Dia da semana: todos os anos x dias -5..370 x bissexto 0/1 e alguns valores estranhos.
        for ano in 0..=0xffffu16 {
            for dia in (-5i16..=370).step_by(if (1590..=2100).contains(&ano) { 1 } else { 23 }) {
                for b in [0, 1] {
                    comparar_semana(&Data { dia, ano, bissexto: b });
                }
            }
        }
        for _ in 0..200_000 {
            let x = s.proximo();
            comparar_semana(&Data { dia: x as i16, ano: (x >> 16) as u16, bissexto: (x >> 32) as i32 });
        }
    });
}

pub unsafe fn instalar(comparando: bool, autoteste: bool) {
    let comparando = comparando || autoteste;
    let alvos: [(&str, &Prologo, usize, usize, *mut usize); 3] = [
        ("data_padrao", &PROLOGO_PADRAO, data_padrao as *const () as usize, comparar_padrao as *const () as usize, &raw mut ORIG_PADRAO),
        ("data_nova", &PROLOGO_NOVA, data_nova as *const () as usize, comparar_nova as *const () as usize, &raw mut ORIG_NOVA),
        ("dia_da_semana", &PROLOGO_SEMANA, dia_da_semana as *const () as usize, comparar_semana as *const () as usize, &raw mut ORIG_SEMANA),
    ];
    let mut todos = true;
    for (nome, p, rust, comp, orig) in alvos {
        match desvios::desviar(nome, p, if comparando { comp } else { rust }) {
            Some(t) => *orig = t,
            None => todos = false,
        }
    }
    if comparando {
        linha("data: modo comparacao (original x Rust)");
    }
    if autoteste && todos {
        rodar_autoteste();
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn bissextos() {
        assert_eq!(bissexto(2000), 1);
        assert_eq!(bissexto(1900), 0);
        assert_eq!(bissexto(2001), 0);
        assert_eq!(bissexto(2004), 1);
        assert_eq!(bissexto(0), 1);
    }

    #[test]
    fn dia_da_semana_intervalo() {
        unsafe {
            // Depois de 1600 o resultado fica em 0..6 e anda um dia por vez.
            for ano in [1601u16, 1999, 2000, 2001] {
                let mut anterior = dia_da_semana(&Data { dia: 0, ano, bissexto: bissexto(ano) });
                for dia in 1..366 {
                    let r = dia_da_semana(&Data { dia, ano, bissexto: bissexto(ano) });
                    assert!((0..=6).contains(&r));
                    assert_eq!(r, (anterior + 1) % 7);
                    anterior = r;
                }
            }
            // Em 1600, dia 0, bissexto: a conta do jogo da -1 (resto negativo devolvido como esta).
            assert_eq!(dia_da_semana(&Data { dia: 0, ano: 1600, bissexto: 1 }), -1);
        }
    }
}
