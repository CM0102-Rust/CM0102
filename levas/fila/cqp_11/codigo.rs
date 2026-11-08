// ---------- %%LEVA%%a leva: o calendário das copas de clubes da Ásia e do Troféu da Liga inglesa: a Liga dos Campeões e a Recopa asiáticas e o LDV Vans Trophy (asia_club_champ.cpp, asia_cup_winner.cpp, eng_auto_cup.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum AsiaClubChampScheduleOp {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use AsiaClubChampScheduleOp::{B as Cqp11B, D as Cqp11D, E as Cqp11E, J as Cqp11J, L as Cqp11L, S as Cqp11S, W as Cqp11W, X as Cqp11X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações e o nome do contador. `None`: a fase devolve 0 sem gravar nada.
#[derive(Clone, Copy)]
struct ScheduleStage {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [AsiaClubChampScheduleOp],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn create_round_table(this: usize, f: Option<ScheduleStage>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
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
    crate::comparison::branch(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops {
        match *op {
            Cqp11D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp11L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqp11S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqp11J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp11E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp11B(o, v) => *((t + o) as *mut u8) = v,
            Cqp11W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqp11X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn written_round_table_bytes(ops: &[AsiaClubChampScheduleOp], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops {
        match *op {
            Cqp11D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp11L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp11S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp11J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp11E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp11B(o, _) => pega(o, 1),
            Cqp11W(o, _) => pega(o, 2),
            Cqp11X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn asia_club_champ_compare_round_table(nome: &str, fase: u32, f: Option<ScheduleStage>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            SCOREBOARD.record(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { written_round_table_bytes(f.ops, a) == written_round_table_bytes(f.ops, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    SCOREBOARD.record(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Liga dos Campeões asiática, eliminatórias de cada zona: 1ª rodada sorteada em 1/8 e jogada em 23/8, 2ª em 14/9 e 20/9, ida e volta.
const ASIAN_CHAMPIONS_LEAGUE_ZONE_KNOCKOUT_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0),
];
/// A Liga dos Campeões asiática, grupos finais: 3 rodadas (21, 23 e 25 de março do ano seguinte, valor 1) com os jogos de cada rodada.
const ASIAN_CLUB_CHAMPIONSHIP_GROUP_OPS: [AsiaClubChampScheduleOp; 12] = [
    Cqp11D(0, 0x15, 2, 1, 2, 1, 1), Cqp11L(0, 0, 2, 2, 3, 0x10), Cqp11L(0, 1, 2, 1, 3, 0x10),
    Cqp11L(0, 2, -1, -1, -1, 0), Cqp11D(1, 0x17, 2, 1, 4, 1, 1), Cqp11L(1, 0, 4, 2, 3, 0x10),
    Cqp11L(1, 1, 4, 1, 3, 0x10), Cqp11L(1, 2, -1, -1, -1, 0), Cqp11D(2, 0x19, 2, 1, 6, 1, 1),
    Cqp11L(2, 0, 6, 2, 3, 0x10), Cqp11L(2, 1, 6, 1, 3, 0x10), Cqp11L(2, 2, -1, -1, -1, 0),
];
/// A Liga dos Campeões asiática, fase final em maio do ano seguinte: semifinal (com os confrontos 0x521f10), terceiro lugar e final.
const ASIAN_CHAMPIONS_LEAGUE_FINALS: [AsiaClubChampScheduleOp; 57] = [
    Cqp11S(0, 0x1a, 2, 1, 0), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10),
    Cqp11E(0, 1, 3, 2, 3, 0x10), Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x383), Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2),
    Cqp11W(0xea, 1), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, eliminatórias: 1ª rodada em 23/8, 2ª em 20/9, quartas em 24 de março do ano seguinte, ida e volta.
const ASIAN_CUP_WINNERS_KNOCKOUT_OPS: [AsiaClubChampScheduleOp; 54] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x1c, 8, 0, 3), Cqp11J(2, 0x18, 2, 1, 2, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11W(0xdf, 0x83), Cqp11B(0xe7, 4), Cqp11W(0xe8, 4),
    Cqp11W(0xea, 2), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, fase final em maio do ano seguinte: semifinal, terceiro lugar e final (e a cauda comum +0x12c..+0x137).
const ASIAN_CUP_WINNERS_CUP_FINALS: [AsiaClubChampScheduleOp; 57] = [
    Cqp11S(0, 0xf, 3, 1, 6), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10), Cqp11E(0, 1, 3, 2, 3, 0x10),
    Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x383),
    Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0), Cqp11W(0x1e, 0),
    Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0), Cqp11X(0x64, 0),
    Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1), Cqp11W(0x73, 0),
    Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1), Cqp11W(0x84, 0),
    Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0),
    Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c), Cqp11W(0xd9, 2),
    Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0), Cqp11X(0x12c, 0),
    Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// O LDV Vans Trophy, cada chave: 5 rodadas de dezembro a março do ano seguinte, a última (a final da chave) em ida e volta.
const LDV_VANS_TROPHY_SECTION_OPS: [AsiaClubChampScheduleOp; 90] = [
    Cqp11S(0, 7, 0xa, 0, 5), Cqp11J(0, 0xa, 0xb, 0, 1, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 4), Cqp11W(0x18, 0x18), Cqp11W(0x1a, 0xc), Cqp11W(0x1c, 0x18), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xb, 0xb, 0, 2), Cqp11J(1, 0xe, 0, 1, 1, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 4), Cqp11W(0x80, 0x10), Cqp11W(0x82, 8),
    Cqp11W(0x84, 4), Cqp11W(0x86, 0x18), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0xf, 0, 1, 2), Cqp11J(2, 4, 1, 1, 1, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 4), Cqp11W(0xe8, 8),
    Cqp11W(0xea, 4), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0), Cqp11S(3, 5, 1, 1, 2), Cqp11J(3, 0x12, 1, 1, 1, 2, 0),
    Cqp11W(0x13f, 0x82), Cqp11W(0x141, 3), Cqp11W(0x143, 0), Cqp11W(0x145, 0x83), Cqp11W(0x147, 0), Cqp11B(0x14f, 4),
    Cqp11W(0x150, 4), Cqp11W(0x152, 2), Cqp11W(0x154, 0), Cqp11W(0x156, 0), Cqp11B(0x158, 0), Cqp11B(0x159, 1),
    Cqp11B(0x15a, 0), Cqp11X(0x194, 0), Cqp11X(0x198, 0), Cqp11X(0x19c, 0), Cqp11S(4, 0x13, 1, 1, 2),
    Cqp11J(4, 4, 2, 1, 1, 2, 0), Cqp11W(0x1a7, 0x96), Cqp11W(0x1a9, 4), Cqp11W(0x1ab, 0), Cqp11W(0x1ad, 0),
    Cqp11W(0x1af, 0x83), Cqp11B(0x1b7, 6), Cqp11W(0x1b8, 2), Cqp11W(0x1ba, 1), Cqp11W(0x1bc, 0), Cqp11W(0x1be, 0),
    Cqp11B(0x1c0, 0), Cqp11B(0x1c1, 2), Cqp11B(0x1c2, 7), Cqp11X(0x1fc, 0), Cqp11X(0x200, 0), Cqp11X(0x204, 0),
];
/// O LDV Vans Trophy, a final: sorteio em 12/3 e jogo em 20/4 do ano seguinte, mando 4 (estádio neutro [inferência]).
const LDV_VANS_TROPHY_FINAL: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 0xc, 2, 1, 3), Cqp11J(0, 0x14, 3, 1, 6, 1, 4), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 6), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0xffff_ffff),
    Cqp11X(0x64, 0),
];

/// A Liga dos Campeões asiática [inferência: o Campeonato Asiático de Clubes], as fases: −1 e 0 as eliminatórias de
/// cada zona (2 rodadas de ida e volta em agosto e setembro), 1 e 2 os grupos finais (3 rodadas em março), 3 a fase
/// final (semifinal, terceiro lugar e final em maio).
fn asian_club_champ_stage(fase: u32) -> Option<ScheduleStage> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(ScheduleStage { liga: 0, rodadas: 2, extra: 0x480 + (f != -1) as i16, extra_checa: false, tamanho: 0xd0, linha: 0xc5,
            ops: &ASIAN_CHAMPIONS_LEAGUE_ZONE_KNOCKOUT_OPS, nome: "cqp_11 eliminatorias da liga dos campeoes asiatica" }),
        1 | 2 => Some(ScheduleStage { liga: 1, rodadas: 3, extra: 0x482 + (f != 1) as i16, extra_checa: false, tamanho: 0xc3, linha: 0x103,
            ops: &ASIAN_CLUB_CHAMPIONSHIP_GROUP_OPS, nome: "cqp_11 grupos da liga dos campeoes asiatica" }),
        3 => Some(ScheduleStage { liga: 0, rodadas: 3, extra: 0, extra_checa: true, tamanho: 0x138, linha: 0x125,
            ops: &ASIAN_CHAMPIONS_LEAGUE_FINALS, nome: "cqp_11 final da liga dos campeoes asiatica" }),
        _ => None,
    }
}

/// { 0x40ae90 } (thiscall, ret 0x10, asia_club_champ.cpp; a virtual +0x3c): `AsiaClubChamp::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `asian_club_champ_stage`.
unsafe extern "thiscall" fn asia_club_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_round_table(this, asian_club_champ_stage(fase), rodadas, extra, liga, 0x987a40)
}

unsafe extern "thiscall" fn compare_asia_club_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    asia_club_champ_compare_round_table("asia_club_champ_create_round_table", fase, asian_club_champ_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| asia_club_champ_create_round_table(this, fase, r, e, l))
}

/// A Recopa asiática, as fases: −1 e 0 as eliminatórias de cada zona (3 rodadas de agosto a março), 1 a fase final
/// (semifinal, terceiro lugar e final em maio).
fn asian_cup_winners_stage(fase: u32) -> Option<ScheduleStage> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(ScheduleStage { liga: 0, rodadas: 3, extra: 0x480 + (f != -1) as i16, extra_checa: false, tamanho: 0x138, linha: 0xc6,
            ops: &ASIAN_CUP_WINNERS_KNOCKOUT_OPS, nome: "cqp_11 eliminatorias da recopa asiatica" }),
        1 => Some(ScheduleStage { liga: 0, rodadas: 3, extra: 0, extra_checa: true, tamanho: 0x138, linha: 0x112,
            ops: &ASIAN_CUP_WINNERS_CUP_FINALS, nome: "cqp_11 final da recopa asiatica" }),
        _ => None,
    }
}

/// { 0x40cf90 } (thiscall, ret 0x10, asia_cup_winner.cpp; a virtual +0x3c): `AsiaCupWinner::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `asian_cup_winners_stage`.
unsafe extern "thiscall" fn asia_cup_winner_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_round_table(this, asian_cup_winners_stage(fase), rodadas, extra, liga, 0x987b60)
}

unsafe extern "thiscall" fn compare_asia_cup_winner_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    asia_club_champ_compare_round_table("asia_cup_winner_create_round_table", fase, asian_cup_winners_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| asia_cup_winner_create_round_table(this, fase, r, e, l))
}

/// O LDV Vans Trophy (o troféu da liga inglesa de baixo), as fases: −1 e 0 as chaves do Norte e do Sul (5 rodadas em
/// jogo único de dezembro a fevereiro e a final da chave em ida e volta em março), 1 a final (abril, mando 4).
fn ldv_trophy_stage(fase: u32) -> Option<ScheduleStage> {
    let f = fase as u8 as i8;
    match f {
        -1 | 0 => Some(ScheduleStage { liga: 0, rodadas: 5, extra: 0x41d + (f != -1) as i16, extra_checa: false, tamanho: 0x208, linha: 0xca,
            ops: &LDV_VANS_TROPHY_SECTION_OPS, nome: "cqp_11 chaves do trofeu" }),
        1 => Some(ScheduleStage { liga: 0, rodadas: 1, extra: 0, extra_checa: false, tamanho: 0x68, linha: 0x13d,
            ops: &LDV_VANS_TROPHY_FINAL, nome: "cqp_11 final do trofeu" }),
        _ => None,
    }
}

/// { 0x56ade0 } (thiscall, ret 0x10, eng_auto_cup.cpp; a virtual +0x3c do LDV Vans Trophy): `EngAutoCup::
/// create_round_table(fase, &rodadas, &extra, &é_liga)`: a fase de `ldv_trophy_stage`.
unsafe extern "thiscall" fn eng_auto_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_round_table(this, ldv_trophy_stage(fase), rodadas, extra, liga, 0x9c4180)
}

unsafe extern "thiscall" fn compare_eng_auto_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    asia_club_champ_compare_round_table("eng_auto_cup_create_round_table", fase, ldv_trophy_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| eng_auto_cup_create_round_table(this, fase, r, e, l))
}
