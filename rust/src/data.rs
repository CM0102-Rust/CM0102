//! Datas do jogo (`si/code/Date.cpp`). Classe de 8 bytes:
//!   +0 i16 dia do ano (0 = 1o de janeiro), +2 u16 ano, +4 i32 bissexto (0/1).
//! Metodos thiscall (o objeto vem em ecx); os que devolvem Data por valor recebem o endereco de
//! retorno como primeiro argumento e o devolvem em eax.
//!   { 0x54a110 } Data()                    dia 40 de 1972, bissexto
//!   { 0x549ef0 } Data(dia, ano)            calcula o bissexto (regra gregoriana, ano de 16 bits)
//!   { 0x549f70 } Data(dia_do_mes, mes, ano, dia_da_semana)  valida (erros Date.cpp 52 e 59),
//!                converte pelas tabelas de dias acumulados e, se pedido, ajusta ao dia da semana
//!   { 0x54a3f0 } dia_da_semana()           0..6, contado a partir de 1600
//!   { 0x54a2d0 } ajustar_semana(alvo, dest)  dest = esta data levada ao dia `alvo` mais proximo (+-3)
//!   { 0x417c10 } operator=(src) -> copia   (atribui e devolve uma copia)
//!   { 0x54c5b0 } operator+(n dias)         \
//!   { 0x54c8e0 } operator-(n dias)          > usam uma Data estatica local cada (inicializada na
//!   { 0x54c770 } mais_semanas(n)           /  primeira chamada, com destrutor vazio no atexit)

use crate::comparacao::{lado_a_lado, Placar, Sorteio};
use crate::desvios::{self, Prologo};
use crate::jogo;
use crate::registro::linha;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Data {
    pub dia: i16,
    pub ano: u16,
    pub bissexto: i32,
}

/// Caminho "E:\dev\CM3\cm3 00-01\si\code\Date.cpp" no exe (para as mensagens de erro).
const FONTE: usize = 0x9b8478;
/// Dias acumulados antes de cada mes (i16 x 12): ano normal e bissexto.
const TABELA_NORMAL: *const i16 = 0x9b8448 as *const i16;
const TABELA_BISSEXTO: *const i16 = 0x9b8460 as *const i16;

/// Data estatica local de um operador: valor, byte de guarda (bit 0 = ja inicializada) e destrutor.
struct Estatica {
    valor: usize,
    guarda: usize,
    destrutor: usize,
}

const EST_MAIS: Estatica = Estatica { valor: 0xae2420, guarda: 0xae2469, destrutor: 0x54c760 };
const EST_MENOS: Estatica = Estatica { valor: 0xae24c0, guarda: 0xae249e, destrutor: 0x54caa0 };
const EST_SEMANAS: Estatica = Estatica { valor: 0xae2410, guarda: 0xae2430, destrutor: 0x54c8d0 };

/// Memoria que os operadores tocam alem dos argumentos (para o modo comparacao).
const ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1)];

const PROLOGOS: [Prologo; 9] = [
    // 0x54a110: mov eax, ecx; mov word ptr [eax], 0x28
    Prologo { endereco: 0x54a110, bytes: &[0x8b, 0xc1, 0x66, 0xc7, 0x00, 0x28, 0x00] },
    // 0x549ef0: mov ax, word ptr [esp + 4]
    Prologo { endereco: 0x549ef0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] },
    // 0x54a3f0: push esi; push edi; xor edi, edi; mov eax, 0x51eb851f
    Prologo { endereco: 0x54a3f0, bytes: &[0x56, 0x57, 0x33, 0xff, 0xb8, 0x1f, 0x85, 0xeb, 0x51] },
    // 0x417c10: push ecx; mov eax, dword ptr [esp + 0xc]
    Prologo { endereco: 0x417c10, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] },
    // 0x54c5b0: sub esp, 0x10; mov al, byte ptr [0xae2469]
    Prologo { endereco: 0x54c5b0, bytes: &[0x83, 0xec, 0x10, 0xa0, 0x69, 0x24, 0xae, 0x00] },
    // 0x54c8e0: sub esp, 0x10; push esi; push edi
    Prologo { endereco: 0x54c8e0, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] },
    // 0x54c770: push ecx; mov al, byte ptr [0xae2430]
    Prologo { endereco: 0x54c770, bytes: &[0x51, 0xa0, 0x30, 0x24, 0xae, 0x00] },
    // 0x54a2d0: mov eax, dword ptr fs:[0]
    Prologo { endereco: 0x54a2d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] },
    // 0x549f70: sub esp, 0x200
    Prologo { endereco: 0x549f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] },
];

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

/// { 0x417c10 } this = *src (ano, dia, bissexto, nessa ordem); *ret = *this; devolve ret.
pub unsafe extern "thiscall" fn copia(this: *mut Data, ret: *mut Data, src: *const Data) -> *mut Data {
    (*this).ano = (*src).ano;
    (*this).dia = (*src).dia;
    (*this).bissexto = (*src).bissexto;
    *ret = *this;
    ret
}

/// Inicializacao da estatica local na primeira chamada: guarda |= 1, Data(), atexit(destrutor).
unsafe fn preparar(e: &Estatica) -> *mut Data {
    let g = e.guarda as *mut u8;
    if *g & 1 == 0 {
        *g |= 1;
        *(e.valor as *mut Data) = Data { dia: 40, ano: 1972, bissexto: 1 };
        jogo::atexit(e.destrutor);
    }
    e.valor as *mut Data
}

/// Avanca `dia` (que pode passar de 365) para os anos seguintes. Comparacoes em 16 bits com sinal.
fn normalizar_para_frente(mut dia: i16, mut ano: u16) -> (i16, u16) {
    loop {
        let limite = if bissexto(ano) == 1 { 365 } else { 364 };
        if dia <= limite {
            return (dia, ano);
        }
        dia = dia.wrapping_sub(limite + 1);
        ano = ano.wrapping_add(1);
    }
}

/// Volta `dia` (que pode ser negativo) para os anos anteriores, somando o tamanho do ano anterior.
fn normalizar_para_tras(mut dia: i16, mut ano: u16) -> (i16, u16) {
    loop {
        if dia >= 0 {
            return (dia, ano);
        }
        let anterior = ano.wrapping_sub(1);
        dia = dia.wrapping_add(if bissexto(anterior) == 1 { 366 } else { 365 });
        ano = anterior;
    }
}

unsafe fn gravar(est: *mut Data, ret: *mut Data, dia: i16, ano: u16) -> *mut Data {
    (*est).ano = ano;
    (*est).dia = dia;
    (*est).bissexto = bissexto(ano);
    *ret = *est;
    ret
}

/// { 0x54c5b0 } n (16 bits) negativo: estatica = *this e devolve estatica - (-n).
/// Esquisitice preservada: com n = -32768, -n continua -32768 e mais/menos se chamam ate estourar
/// a pilha (igual na original; o jogo nunca pede isso).
pub unsafe extern "thiscall" fn mais(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
    let est = preparar(&EST_MAIS);
    if (n as i16) < 0 {
        let (mut t1, mut t2, mut t3) = (Data::default(), Data::default(), Data::default());
        copia(est, &mut t1, this);
        let r = menos(est, &mut t2, (n as i32).wrapping_neg() as u32);
        copia(est, &mut t3, r);
        *ret = *est;
        return ret;
    }
    let (dia, ano) = normalizar_para_frente((*this).dia.wrapping_add(n as i16), (*this).ano);
    gravar(est, ret, dia, ano)
}

/// { 0x54c8e0 } n (16 bits) negativo: estatica = *this e devolve estatica + (-n).
pub unsafe extern "thiscall" fn menos(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
    let est = preparar(&EST_MENOS);
    if (n as i16) < 0 {
        let (mut t1, mut t2, mut t3) = (Data::default(), Data::default(), Data::default());
        copia(est, &mut t1, this);
        let r = mais(est, &mut t2, (n as i32).wrapping_neg() as u32);
        copia(est, &mut t3, r);
        *ret = *est;
        return ret;
    }
    let (dia, ano) = normalizar_para_tras((*this).dia.wrapping_sub(n as i16), (*this).ano);
    gravar(est, ret, dia, ano)
}

/// { 0x54c770 } n (8 bits) semanas.
pub unsafe extern "thiscall" fn mais_semanas(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
    let est = preparar(&EST_SEMANAS);
    let dias = ((n as u8) as u16).wrapping_mul(7) as i16;
    let (dia, ano) = normalizar_para_frente(dias.wrapping_add((*this).dia), (*this).ano);
    gravar(est, ret, dia, ano)
}

/// { 0x54a2d0 } *dest = *this e, se o dia da semana nao for `alvo`, move dest para o `alvo` mais
/// proximo (diferenca levada a -3..3). O valor devolvido nao e usado pelos chamadores.
pub unsafe extern "thiscall" fn ajustar_semana(this: *const Data, alvo: u32, dest: *mut Data) -> u32 {
    let semana = dia_da_semana(this);
    let mut t = Data::default();
    copia(dest, &mut t, this);
    if semana as i16 == alvo as i16 {
        return alvo;
    }
    let mut dif = semana.wrapping_sub(alvo as i32);
    if (dif as i16) < 0 {
        if (dif as i16) < -3 {
            dif += 7;
        }
    } else if (dif as i16) > 3 {
        dif -= 7;
    }
    let (mut t2, mut t3) = (Data::default(), Data::default());
    let r = menos(dest, &mut t2, dif as u32);
    copia(dest, &mut t3, r) as u32
}

/// Dia do mes e mes fora do intervalo: o jogo mostra o erro (Date.cpp 52 / 59) e corrige.
fn entrada_valida(dia: u32, mes: u32) -> bool {
    (1..=31).contains(&(dia as i16)) && (0..=11).contains(&(mes as u8 as i8))
}

/// { 0x549f70 } Data(dia_do_mes, mes 0..11, ano, dia_da_semana ou 0xffff).
pub unsafe extern "thiscall" fn data_dma(this: *mut Data, dia: u32, mes: u32, ano: u32, semana: u32) -> *mut Data {
    let mut di = dia as i16;
    if !(1..=31).contains(&di) {
        jogo::erro(FONTE, 0x34);
        di = 1;
    }
    let mut m = mes as u8 as i8;
    if !(0..=11).contains(&m) {
        jogo::erro(FONTE, 0x3b);
        m = 0;
    }
    let ano = ano as u16;
    (*this).ano = ano;
    if bissexto(ano) == 1 {
        (*this).bissexto = 1;
        (*this).dia = (*TABELA_BISSEXTO.add(m as usize)).wrapping_add(di);
    } else {
        (*this).bissexto = 0;
        (*this).dia = (*TABELA_NORMAL.add(m as usize)).wrapping_add(di);
    }
    if semana as u16 != 0xffff {
        ajustar_semana(this, semana, this);
    }
    this
}

// ---------- comparacao com as originais ----------

static mut ORIGINAIS: [usize; 9] = [0; 9];
static PLACAR: Placar = Placar::novo("data");

unsafe fn orig<F>(i: usize) -> F {
    core::mem::transmute_copy(&ORIGINAIS[i])
}

fn regioes(objetos: &[usize]) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = objetos.iter().map(|&a| (a, 8)).collect();
    v.extend(ESTATICAS);
    v
}

unsafe extern "thiscall" fn comparar_padrao(this: *mut Data) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data) -> *mut Data = orig(0);
    let (r, rr, igual, mo, mr) = lado_a_lado(&regioes(&[this as usize]), || f(this), || data_padrao(this));
    PLACAR.registrar(igual && r == rr, || format!("padrao original={:x?} rust={:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_nova(this: *mut Data, dia: u32, ano: u32) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, u32, u32) -> *mut Data = orig(1);
    let (r, rr, igual, mo, mr) = lado_a_lado(&regioes(&[this as usize]), || f(this, dia, ano), || data_nova(this, dia, ano));
    PLACAR.registrar(igual && r == rr, || format!("nova({:#x}, {:#x}) original={:x?} rust={:x?}", dia, ano, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_semana(this: *const Data) -> i32 {
    let f: unsafe extern "thiscall" fn(*const Data) -> i32 = orig(2);
    let r = f(this);
    let nosso = dia_da_semana(this);
    PLACAR.registrar(r == nosso, || format!("semana({:?}) original={} rust={}", *this, r, nosso));
    r
}

unsafe extern "thiscall" fn comparar_copia(this: *mut Data, ret: *mut Data, src: *const Data) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, *mut Data, *const Data) -> *mut Data = orig(3);
    let reg = regioes(&[this as usize, ret as usize, src as usize]);
    let (r, rr, igual, mo, mr) = lado_a_lado(&reg, || f(this, ret, src), || copia(this, ret, src));
    PLACAR.registrar(igual && r == rr, || format!("copia original={:x?} rust={:x?}", mo, mr));
    r
}

macro_rules! comparar_operador {
    ($nome:ident, $i:expr, $rust:ident, $texto:literal) => {
        unsafe extern "thiscall" fn $nome(this: *const Data, ret: *mut Data, n: u32) -> *mut Data {
            let f: unsafe extern "thiscall" fn(*const Data, *mut Data, u32) -> *mut Data = orig($i);
            let reg = regioes(&[this as usize, ret as usize]);
            let (r, rr, igual, mo, mr) = lado_a_lado(&reg, || f(this, ret, n), || $rust(this, ret, n));
            PLACAR.registrar(igual && r == rr, || format!("{}({:?}, {:#x}) original={:x?} rust={:x?}", $texto, *this, n, mo, mr));
            r
        }
    };
}

comparar_operador!(comparar_mais, 4, mais, "mais");
comparar_operador!(comparar_menos, 5, menos, "menos");
comparar_operador!(comparar_semanas, 6, mais_semanas, "mais_semanas");

unsafe extern "thiscall" fn comparar_ajustar(this: *const Data, alvo: u32, dest: *mut Data) -> u32 {
    let f: unsafe extern "thiscall" fn(*const Data, u32, *mut Data) -> u32 = orig(7);
    let reg = regioes(&[this as usize, dest as usize]);
    // O valor devolvido e o endereco de uma temporaria na pilha: nao e comparado.
    let (r, _, igual, mo, mr) = lado_a_lado(&reg, || f(this, alvo, dest), || ajustar_semana(this, alvo, dest));
    PLACAR.registrar(igual, || format!("ajustar_semana({:#x}) original={:x?} rust={:x?}", alvo, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_dma(this: *mut Data, dia: u32, mes: u32, ano: u32, semana: u32) -> *mut Data {
    let f: unsafe extern "thiscall" fn(*mut Data, u32, u32, u32, u32) -> *mut Data = orig(8);
    if !entrada_valida(dia, mes) {
        // Caminho de erro: mostraria a caixa duas vezes. So a original roda.
        return f(this, dia, mes, ano, semana);
    }
    let reg = regioes(&[this as usize]);
    let (r, rr, igual, mo, mr) = lado_a_lado(&reg, || f(this, dia, mes, ano, semana), || data_dma(this, dia, mes, ano, semana));
    PLACAR.registrar(igual && r == rr, || format!("dma({:#x}, {:#x}, {:#x}, {:#x}) original={:x?} rust={:x?}", dia, mes, ano, semana, mo, mr));
    r
}

/// Autoteste (roda depois de a CRT do jogo estar inicializada: os operadores chamam atexit).
pub unsafe fn autoteste() {
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
        // Operadores e cia.: datas plausiveis e estranhas, n pequenos, grandes, negativos e bordas.
        // Fora: n com 16 bits de baixo = 0x8000 (-32768). Negado continua -32768, e mais/menos
        // chamam um ao outro para sempre ate estourar a pilha; a original faz o mesmo.
        let bordas: [u32; 10] = [0, 1, 7, 365, 366, 0x7fff, 0x8001, 0xffff, 0x1_0005, 0xffff_fff9];
        for i in 0..400_000u32 {
            let x = s.proximo();
            let ano = if i % 4 == 0 { (x >> 48) as u16 } else { 1900 + ((x >> 48) % 250) as u16 };
            let dia = if i % 5 == 0 { (x >> 32) as i16 } else { ((x >> 32) % 400) as i16 - 20 };
            let a = Data { dia, ano, bissexto: if i % 7 == 0 { (x >> 20) as i32 } else { bissexto(ano) } };
            let n = if i % 3 == 0 { bordas[(x as usize >> 4) % bordas.len()] } else { (x >> 8) as u32 % 2000 };
            let n = if i % 2 == 0 { n } else { (n as i32).wrapping_neg() as u32 };
            if n as u16 == 0x8000 {
                continue;
            }
            let mut ret = Data::default();
            comparar_mais(&a, &mut ret, n);
            comparar_menos(&a, &mut ret, n);
            comparar_semanas(&a, &mut ret, n);
            let mut dest = Data { dia: 99, ano: 1, bissexto: 3 };
            comparar_ajustar(&a, (x >> 12) as u32 % 9, &mut dest);
            let mut alvo = a;
            let src = Data { dia: (x >> 16) as i16, ano: (x >> 24) as u16, bissexto: x as i32 };
            comparar_copia(&mut alvo, &mut ret, &src);
            comparar_copia(&mut alvo, &mut ret, &alvo as *const Data); // src == this
            let (dm, m) = (1 + (x >> 40) as u32 % 31, (x >> 45) as u32 % 12);
            let semana = if i % 2 == 0 { 0xffff } else { (x >> 50) as u32 % 7 };
            let mut c = Data::default();
            comparar_dma(&mut c, dm, m, ano as u32, semana);
        }
    });
}

pub unsafe fn instalar(comparando: bool) {
    let rust: [usize; 9] = [
        data_padrao as *const () as usize,
        data_nova as *const () as usize,
        dia_da_semana as *const () as usize,
        copia as *const () as usize,
        mais as *const () as usize,
        menos as *const () as usize,
        mais_semanas as *const () as usize,
        ajustar_semana as *const () as usize,
        data_dma as *const () as usize,
    ];
    let comp: [usize; 9] = [
        comparar_padrao as *const () as usize,
        comparar_nova as *const () as usize,
        comparar_semana as *const () as usize,
        comparar_copia as *const () as usize,
        comparar_mais as *const () as usize,
        comparar_menos as *const () as usize,
        comparar_semanas as *const () as usize,
        comparar_ajustar as *const () as usize,
        comparar_dma as *const () as usize,
    ];
    let nomes = ["data_padrao", "data_nova", "dia_da_semana", "data_copia", "data_mais", "data_menos", "data_mais_semanas", "data_ajustar_semana", "data_dma"];
    for i in 0..9 {
        if let Some(t) = desvios::desviar(nomes[i], &PROLOGOS[i], if comparando { comp[i] } else { rust[i] }) {
            ORIGINAIS[i] = t;
        }
    }
    if comparando {
        linha("data: modo comparacao (original x Rust)");
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

    #[test]
    fn normalizacao() {
        assert_eq!(normalizar_para_frente(365, 2001), (0, 2002));
        assert_eq!(normalizar_para_frente(365, 2000), (365, 2000)); // bissexto: 366 dias
        assert_eq!(normalizar_para_frente(366 + 365, 2000), (0, 2002));
        assert_eq!(normalizar_para_tras(-1, 2001), (365, 2000));
        assert_eq!(normalizar_para_tras(-1, 2002), (364, 2001));
    }
}
