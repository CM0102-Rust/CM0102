// ---------- %%LEVA%%a leva: as copas de clubes da Oceania e da Ásia: o calendário do Campeonato de Clubes da Oceania e os times da Liga dos Campeões e da Recopa asiáticas (oceania_club_champ.cpp, asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum Cqq12Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use Cqq12Op::{B as Cqq12B, D as Cqq12D, E as Cqq12E, J as Cqq12J, L as Cqq12L, S as Cqq12S, W as Cqq12W, X as Cqq12X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
struct Cqq12Fase {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [Cqq12Op],
    ops2: &'static [Cqq12Op],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn cqq12_tabela(this: usize, f: Option<Cqq12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
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
        aviso_em(arquivo, f.linha);
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
            Cqq12D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqq12L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqq12S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqq12J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqq12E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqq12B(o, v) => *((t + o) as *mut u8) = v,
            Cqq12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqq12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn cqq12_gravados(ops: &[Cqq12Op], ops2: &[Cqq12Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqq12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqq12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqq12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqq12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqq12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqq12B(o, _) => pega(o, 1),
            Cqq12W(o, _) => pega(o, 2),
            Cqq12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn cqq12_comparar(nome: &str, fase: u32, f: Option<Cqq12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
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
    let tabela = if a != 0 && b != 0 { cqq12_gravados(f.ops, f.ops2, a) == cqq12_gravados(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// O Campeonato de Clubes da Oceania, as três rodadas de cada grupo (10, 12 e 14 de janeiro do ano seguinte, valor 1) com o jogo 2 vazio.
const CQQ12_OCEANIA_GRUPOS: [Cqq12Op; 6] = [
    Cqq12D(0, 0xa, 0, 1, 4, 2, 1), Cqq12L(0, 2, -1, -1, -1, 0), Cqq12D(1, 0xc, 0, 1, 6, 1, 1),
    Cqq12L(1, 2, -1, -1, -1, 0), Cqq12D(2, 0xe, 0, 1, 1, 2, 1), Cqq12L(2, 2, -1, -1, -1, 0),
];
/// Os jogos do grupo da fase −1.
const CQQ12_OCEANIA_GRUPO_A: [Cqq12Op; 6] = [
    Cqq12L(0, 0, 4, 1, 3, 4), Cqq12L(0, 1, 4, 2, 3, 0x40), Cqq12L(1, 0, 6, 1, 3, 4), Cqq12L(1, 1, 6, 2, 3, 0x40),
    Cqq12L(2, 0, 1, 1, 3, 4), Cqq12L(2, 1, 1, 1, 3, 0x40),
];
/// Os jogos do grupo da fase 0.
const CQQ12_OCEANIA_GRUPO_B: [Cqq12Op; 6] = [
    Cqq12L(0, 0, 5, 1, 3, 0x40), Cqq12L(0, 1, 5, 2, 3, 4), Cqq12L(1, 0, 0, 1, 3, 0x40), Cqq12L(1, 1, 0, 2, 3, 4),
    Cqq12L(2, 0, 2, 1, 3, 4), Cqq12L(2, 1, 2, 1, 3, 0x40),
];
/// O mata-mata em jogo único: semifinal em 17/1, final e terceiro lugar em 19/1 do ano seguinte.
const CQQ12_OCEANIA_MATA: [Cqq12Op; 54] = [
    Cqq12S(0, 0x10, 0, 1, 3), Cqq12J(0, 0x11, 0, 1, 4, 2, 0x40), Cqq12W(0x7, 0x82), Cqq12W(0x9, 0), Cqq12W(0xb, 0),
    Cqq12W(0xd, 0x283), Cqq12B(0x17, 8), Cqq12W(0x18, 4), Cqq12W(0x1a, 2), Cqq12W(0x1c, 4), Cqq12W(0xf, 0),
    Cqq12W(0x1e, 0), Cqq12B(0x20, 0), Cqq12B(0x21, 1), Cqq12B(0x22, 0), Cqq12X(0x5c, 0), Cqq12X(0x60, 0),
    Cqq12X(0x64, 0), Cqq12S(1, 0x12, 0, 1, 5), Cqq12J(1, 0x13, 0, 1, 6, 1, 4), Cqq12W(0x6f, 0x96), Cqq12W(0x71, 1),
    Cqq12W(0x73, 0), Cqq12W(0x75, 0x83), Cqq12B(0x7f, 8), Cqq12W(0x80, 2), Cqq12W(0x82, 1), Cqq12W(0x84, 0),
    Cqq12W(0x77, 0), Cqq12W(0x86, 0), Cqq12B(0x88, 0), Cqq12B(0x89, 1), Cqq12B(0x8a, 0), Cqq12X(0xc4, 0),
    Cqq12X(0xc8, 0), Cqq12X(0xcc, 0), Cqq12S(2, 0x12, 0, 1, 5), Cqq12J(2, 0x13, 0, 1, 6, 1, 0x40), Cqq12W(0xd7, 0x8c),
    Cqq12W(0xd9, 2), Cqq12W(0xdb, 0), Cqq12W(0xdd, 0x83), Cqq12B(0xe7, 8), Cqq12W(0xe8, 2), Cqq12W(0xea, 1),
    Cqq12W(0xec, 0), Cqq12W(0xdf, 0), Cqq12W(0xee, 0), Cqq12B(0xf0, 0), Cqq12B(0xf1, 1), Cqq12B(0xf2, 0),
    Cqq12X(0x12c, 0), Cqq12X(0x130, 0), Cqq12X(0x134, 0),
];

/// A fase do Campeonato de Clubes da Oceania: abaixo de 1 os grupos (id 0x420 + fase; os jogos de cada grupo para −1
/// e 0), 1 o mata-mata, acima nada.
fn cqq12_oceania_fase(fase: u32) -> Option<Cqq12Fase> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [Cqq12Op] = match f {
            -1 => &CQQ12_OCEANIA_GRUPO_A,
            0 => &CQQ12_OCEANIA_GRUPO_B,
            _ => &[],
        };
        Some(Cqq12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x420), extra_checa: false, tamanho: 0xc3, linha: 0x93,
            ops: &CQQ12_OCEANIA_GRUPOS, ops2: jogos, nome: "cqq_12 grupos da oceania" })
    } else if f == 1 {
        Some(Cqq12Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: false, tamanho: 0x138, linha: 0xc5,
            ops: &CQQ12_OCEANIA_MATA, ops2: &[], nome: "cqq_12 mata-mata da oceania" })
    } else {
        None
    }
}

/// { 0x797230 } (thiscall, ret 0x10, oceania_club_champ.cpp; a virtual +0x3c): `OceaniaClubChamp::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqq12_oceania_fase`.
unsafe extern "thiscall" fn cqq12_oceania_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqq12_tabela(this, cqq12_oceania_fase(fase), rodadas, extra, liga, 0xa6a068)
}

unsafe extern "thiscall" fn comparar_cqq12_oceania_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cqq12_comparar("cqq12_oceania_tabela", fase, cqq12_oceania_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqq12_oceania_tabela(this, fase, r, e, l))
}

unsafe fn cqq12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// A tabela dos 16 times das copas asiáticas (AsiaClubChamp::build_team_table 0x40b620 e AsiaCupWinner::
/// create_team_entries 0x40d4d0, o mesmo corpo): +0x3e = 16 e a tabela (0x944e46 de 0x60 em +0xa7; sem memória, a caixa
/// `linha` de `arquivo` e −1); a vaga i recebe o clube i dos classificados (+0xb2, ponteiros; `ordem` dá o índice) com
/// classe e situação 0. Devolve 1.
unsafe fn cqq12_times(this: usize, arquivo: usize, linha: u32, nome: &'static str, ordem: impl Fn(i16) -> i16) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x10);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x60);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return -1;
    }
    crate::comparacao::ramo(nome);
    let mut i: i16 = 0;
    while i < core::ptr::read_unaligned((this + 0x3e) as *const i16) {
        let c = cqq12_d(cqq12_d(this + 0xb2) + ordem(i) as usize * 4);
        let e = cqq12_d(this + 0xa7) + i as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = 0;
        *((e + 5) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 16 vagas (com a devolução 1) e liberada.
unsafe fn cqq12_comparar_times(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqq12_d(this + 0xa7));
        r
    }, || {
        let r = c();
        lr.set(cqq12_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 0x60 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", nome, this, a, b, igual, lista));
    a
}

/// { 0x40b620 } (thiscall, ret 0, asia_club_champ.cpp): `AsiaClubChamp::build_team_table()`: os 16 times (a caixa da
/// linha 0x1cd sem memória).
unsafe extern "thiscall" fn cqq12_asia_clubes_times(this: usize) -> i32 {
    cqq12_times(this, 0x987a40, 0x1cd, "cqq_12 times da liga dos campeoes asiatica", |i| i) // cqq12: os times da Liga dos Campeões asiática
}

unsafe extern "thiscall" fn comparar_cqq12_asia_clubes_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    cqq12_comparar_times("cqq12_asia_clubes_times", this, || o(this), || cqq12_asia_clubes_times(this))
}

/// { 0x40d4d0 } (thiscall, ret 0, asia_cup_winner.cpp): `AsiaCupWinner::create_team_entries()`: os 16 times (a caixa
/// da linha 0x185 sem memória).
unsafe extern "thiscall" fn cqq12_asia_recopa_times(this: usize) -> i32 {
    cqq12_times(this, 0x987b60, 0x185, "cqq_12 times da recopa asiatica", |i| i) // cqq12: os times da Recopa asiática
}

unsafe extern "thiscall" fn comparar_cqq12_asia_recopa_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    cqq12_comparar_times("cqq12_asia_recopa_times", this, || o(this), || cqq12_asia_recopa_times(this))
}
