// ---------- %%LEVA%%a leva: o calendário dos torneios de seleções: a Copa América, a Copa Africana de Nações e a Copa das Confederações (goldcup.cpp, african_nations.cpp, fifa_confed.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum GoldcupScheduleOp {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use GoldcupScheduleOp::{B as Cqp12B, D as Cqp12D, E as Cqp12E, J as Cqp12J, L as Cqp12L, S as Cqp12S, W as Cqp12W, X as Cqp12X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
struct GoldcupStagePlan {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [GoldcupScheduleOp],
    ops2: &'static [GoldcupScheduleOp],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn build_phase_table(this: usize, f: Option<GoldcupStagePlan>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(f.tamanho);
    if t == 0 {
        show_error_at(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqp12D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp12L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqp12S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqp12J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp12E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp12B(o, v) => *((t + o) as *mut u8) = v,
            Cqp12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqp12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn phase_table_written_bytes(ops: &[GoldcupScheduleOp], ops2: &[GoldcupScheduleOp], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqp12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp12B(o, _) => pega(o, 1),
            Cqp12W(o, _) => pega(o, 2),
            Cqp12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn compare_phase_table(nome: &str, fase: u32, f: Option<GoldcupStagePlan>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { phase_table_written_bytes(f.ops, f.ops2, a) == phase_table_written_bytes(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa América, as três rodadas de cada grupo (6, 10 e 13 de julho, código 1, valor 1) com o jogo 2 vazio.
const COPA_AMERICA_GROUP_DATE_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 6, 6, 0, 3, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xa, 6, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xd, 6, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa América, os jogos do grupo da fase −1.
const COPA_AMERICA_GROUP_GAMES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 2, 2, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 0x100), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x80),
];
/// A Copa América, os jogos do grupo da fase 0.
const COPA_AMERICA_GROUP_B_MATCHES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 4, 1, 3, 0x40), Cqp12L(0, 1, 4, 2, 3, 0x100), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x80),
    Cqp12L(2, 0, 3, 1, 3, 0x40), Cqp12L(2, 1, 3, 2, 3, 0x100),
];
/// A Copa América, os jogos do grupo da fase 1.
const COPA_AMERICA_GROUP_C_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 1, 1, 3, 0x100), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x80),
];
/// A Copa América, o mata-mata em jogo único: quartas em 16/7 (com os confrontos fixos), semifinal em 19/7, final em 23/7 e terceiro lugar em 22/7.
const COPA_AMERICA_KNOCKOUT_OPS: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0xf, 6, 0, 5), Cqp12J(0, 0x10, 6, 0, 6, 1, 1), Cqp12E(0, 0, 6, 2, 3, 4), Cqp12E(0, 1, 6, 1, 3, 0x40),
    Cqp12E(0, 2, 0, 1, 3, 0x100), Cqp12E(0, 3, 0, 2, 3, 0x80), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x103), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x12, 6, 0, 1), Cqp12J(1, 0x13, 6, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 0x40), Cqp12E(1, 1, 2, 2, 3, 4), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x103), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x14, 6, 0, 3), Cqp12J(2, 0x17, 6, 0, 6, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x14, 6, 0, 3),
    Cqp12J(3, 0x16, 6, 0, 5, 1, 0x80), Cqp12W(0x150, 2), Cqp12W(0x141, 3), Cqp12W(0x145, 3), Cqp12W(0x13f, 0x8c),
    Cqp12W(0x143, 0), Cqp12B(0x14f, 0xa), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa Africana, as três rodadas de cada grupo (8, 12 e 16 de fevereiro) com o jogo 2 vazio.
const AFRICAN_NATIONS_GROUP_ROUNDS: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 8, 1, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xc, 1, 0, 3, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x10, 1, 0, 0, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa Africana, os jogos do grupo da fase −1.
const AFRICAN_NATIONS_GROUP_A_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 6, 1, 3, 0x40), Cqp12L(1, 0, 2, 1, 3, 4), Cqp12L(1, 1, 2, 2, 3, 0x40),
    Cqp12L(2, 0, 6, 1, 3, 4), Cqp12L(2, 1, 6, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 0.
const AFRICAN_NATIONS_GROUP_GAMES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x40), Cqp12L(1, 0, 3, 1, 3, 4), Cqp12L(1, 1, 3, 2, 3, 0x40),
    Cqp12L(2, 0, 0, 1, 3, 4), Cqp12L(2, 1, 0, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 1.
const AFRICAN_NATIONS_GROUP_C_MATCHES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 0x80), Cqp12L(0, 1, 6, 2, 3, 0x80), Cqp12L(1, 0, 2, 2, 3, 0x80),
    Cqp12L(1, 1, 3, 2, 3, 0x80), Cqp12L(2, 0, 0, 2, 3, 0x80), Cqp12L(2, 1, 0, 2, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 2.
const AFRICAN_NATIONS_GROUP_D_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 0, 2, 3, 0x80), Cqp12L(0, 1, 1, 2, 3, 0x80), Cqp12L(1, 0, 4, 1, 3, 0x80),
    Cqp12L(1, 1, 4, 2, 3, 0x80), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// A Copa Africana, o mata-mata em jogo único: quartas em 21/2, semifinal em 25/2, final em 28/2 e terceiro lugar em 27/2.
const AFRICAN_NATIONS_KNOCKOUT_OPS: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0x12, 1, 0, 2), Cqp12J(0, 0x15, 1, 0, 5, 1, 1), Cqp12E(0, 0, 4, 2, 3, 0x80), Cqp12E(0, 1, 5, 1, 3, 4),
    Cqp12E(0, 2, 5, 2, 3, 0x40), Cqp12E(0, 3, 6, 1, 3, 0x40), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x17, 1, 0, 0), Cqp12J(1, 0x19, 1, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 4), Cqp12E(1, 1, 2, 2, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x303), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x1a, 1, 0, 3), Cqp12J(2, 0x1c, 1, 0, 5, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x203), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x1a, 1, 0, 3),
    Cqp12J(3, 0x1b, 1, 0, 4, 2, 0x40), Cqp12W(0x13f, 0x8c), Cqp12W(0x141, 3), Cqp12W(0x143, 0), Cqp12W(0x145, 0x203),
    Cqp12B(0x14f, 0xa), Cqp12W(0x150, 2), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa das Confederações, as três rodadas de cada grupo (29 de julho, 1º e 4 de agosto) com o jogo 2 vazio.
const CONFED_CUP_GROUP_ROUNDS: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0x1d, 6, 0, 5, 2, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 1, 7, 0, 1, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 4, 7, 0, 4, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa das Confederações, os jogos do grupo da fase −1.
const CONFED_CUP_GROUP_A_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 4), Cqp12L(0, 1, 6, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];
/// A Copa das Confederações, os jogos do grupo da fase 0.
const CONFED_CUP_GROUP_GAMES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x40), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(1, 1, 2, 2, 3, 4),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];
/// A Copa das Confederações, o mata-mata em jogo único: semifinal em 6/8, final em 9/8 e terceiro lugar em 8/8.
const CONFED_CUP_KNOCKOUT: [GoldcupScheduleOp; 57] = [
    Cqp12S(0, 5, 7, 0, 5), Cqp12J(0, 6, 7, 0, 6, 1, 1), Cqp12E(0, 0, 6, 1, 3, 0x40), Cqp12E(0, 1, 6, 1, 3, 4),
    Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x383),
    Cqp12B(0x17, 0xa), Cqp12W(0x18, 4), Cqp12W(0x1a, 2), Cqp12W(0x1c, 4), Cqp12W(0xf, 0), Cqp12W(0x1e, 0),
    Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
    Cqp12S(1, 7, 7, 0, 0), Cqp12J(1, 9, 7, 0, 2, 1, 4), Cqp12W(0x6f, 0x96), Cqp12W(0x71, 1), Cqp12W(0x73, 0),
    Cqp12W(0x75, 0x83), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 2), Cqp12W(0x82, 1), Cqp12W(0x84, 0), Cqp12W(0x77, 0),
    Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0), Cqp12X(0xc4, 0), Cqp12X(0xc8, 0),
    Cqp12X(0xcc, 0), Cqp12S(2, 7, 7, 0, 0), Cqp12J(2, 8, 7, 0, 1, 1, 0x40), Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 2),
    Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x83), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2), Cqp12W(0xea, 1), Cqp12W(0xec, 0),
    Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1), Cqp12B(0xf2, 0), Cqp12X(0x12c, 0),
    Cqp12X(0x130, 0), Cqp12X(0x134, 0),
];

/// A fase da Copa América pelo byte com sinal: abaixo de 2 os grupos (id 0x420 + fase; os jogos de cada grupo para −1,
/// 0 e 1), 2 o mata-mata, acima nada.
fn copa_america_phase(fase: u32) -> Option<GoldcupStagePlan> {
    let f = fase as u8 as i8;
    if f < 2 {
        let jogos: &'static [GoldcupScheduleOp] = match f {
            -1 => &COPA_AMERICA_GROUP_GAMES,
            0 => &COPA_AMERICA_GROUP_B_MATCHES,
            1 => &COPA_AMERICA_GROUP_C_OPS,
            _ => &[],
        };
        Some(GoldcupStagePlan { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x420), extra_checa: false, tamanho: 0xc3, linha: 0x9c,
            ops: &COPA_AMERICA_GROUP_DATE_OPS, ops2: jogos, nome: "cqp_12 grupos da copa america" })
    } else if f == 2 {
        Some(GoldcupStagePlan { liga: 0, rodadas: 4, extra: 0, extra_checa: false, tamanho: 0x1a0, linha: 0xd6,
            ops: &COPA_AMERICA_KNOCKOUT_OPS, ops2: &[], nome: "cqp_12 mata-mata da copa america" })
    } else {
        None
    }
}

/// { 0x5e0900 } (thiscall, ret 0x10, goldcup.cpp; a virtual +0x3c da Copa América): `GoldCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `copa_america_phase`.
unsafe extern "thiscall" fn goldcup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    build_phase_table(this, copa_america_phase(fase), rodadas, extra, liga, 0x9cd640)
}

unsafe extern "thiscall" fn comparar_goldcup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_phase_table("goldcup_create_schedule", fase, copa_america_phase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| goldcup_create_schedule(this, fase, r, e, l))
}

/// A fase da Copa Africana: abaixo de 3 os grupos (id 0x3e9 + fase; os jogos de cada grupo para −1, 0, 1 e 2), 3 o
/// mata-mata, acima nada.
fn african_nations_phase(fase: u32) -> Option<GoldcupStagePlan> {
    let f = fase as u8 as i8;
    if f < 3 {
        let jogos: &'static [GoldcupScheduleOp] = match f {
            -1 => &AFRICAN_NATIONS_GROUP_A_OPS,
            0 => &AFRICAN_NATIONS_GROUP_GAMES,
            1 => &AFRICAN_NATIONS_GROUP_C_MATCHES,
            2 => &AFRICAN_NATIONS_GROUP_D_OPS,
            _ => &[],
        };
        Some(GoldcupStagePlan { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x3e9), extra_checa: false, tamanho: 0xc3, linha: 0x9e,
            ops: &AFRICAN_NATIONS_GROUP_ROUNDS, ops2: jogos, nome: "cqp_12 grupos da copa africana" })
    } else if f == 3 {
        Some(GoldcupStagePlan { liga: 0, rodadas: 4, extra: 0, extra_checa: false, tamanho: 0x1a0, linha: 0xe0,
            ops: &AFRICAN_NATIONS_KNOCKOUT_OPS, ops2: &[], nome: "cqp_12 mata-mata da copa africana" })
    } else {
        None
    }
}

/// { 0x4012e0 } (thiscall, ret 0x10, african_nations.cpp; a virtual +0x3c da Copa Africana): `AfricanNations::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `african_nations_phase`.
unsafe extern "thiscall" fn african_nations_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    build_phase_table(this, african_nations_phase(fase), rodadas, extra, liga, 0x9870f8)
}

unsafe extern "thiscall" fn comparar_african_nations_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_phase_table("african_nations_create_schedule", fase, african_nations_phase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| african_nations_create_schedule(this, fase, r, e, l))
}

/// A fase da Copa das Confederações: abaixo de 1 os grupos (id 0x3e9 + fase; os jogos de cada grupo para −1 e 0), 1 o
/// mata-mata, acima nada.
fn confed_cup_phase(fase: u32) -> Option<GoldcupStagePlan> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [GoldcupScheduleOp] = match f {
            -1 => &CONFED_CUP_GROUP_A_OPS,
            0 => &CONFED_CUP_GROUP_GAMES,
            _ => &[],
        };
        Some(GoldcupStagePlan { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x3e9), extra_checa: false, tamanho: 0xc3, linha: 0x95,
            ops: &CONFED_CUP_GROUP_ROUNDS, ops2: jogos, nome: "cqp_12 grupos da copa das confederacoes" })
    } else if f == 1 {
        Some(GoldcupStagePlan { liga: 0, rodadas: 3, extra: 0, extra_checa: false, tamanho: 0x138, linha: 0xc7,
            ops: &CONFED_CUP_KNOCKOUT, ops2: &[], nome: "cqp_12 mata-mata da copa das confederacoes" })
    } else {
        None
    }
}

/// { 0x58b560 } (thiscall, ret 0x10, fifa_confed.cpp; a virtual +0x3c da Copa das Confederações): `FifaConfed::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `confed_cup_phase`.
unsafe extern "thiscall" fn goldcup_create_schedule_58b560(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    build_phase_table(this, confed_cup_phase(fase), rodadas, extra, liga, 0x9c7474)
}

unsafe extern "thiscall" fn comparar_goldcup_create_schedule_58b560(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_phase_table("goldcup_create_schedule_58b560", fase, confed_cup_phase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| goldcup_create_schedule_58b560(this, fase, r, e, l))
}
