use super::*;

// ---------- 284a leva: os mata-matas da Copa Africana de Nações e da Recopa asiática: as quartas, a fase seguinte e as finais (african_nations.cpp, asia_cup_winner.cpp) ----------




/// O grupo de onde sai cada vaga das quartas da Copa Africana (−1 = o grupo A, a tabela da própria copa; 0..2 = as fases
/// dos grupos B a D) e a posição: as chaves são os pares da lista.
pub(crate) const CQT13_QUARTAS: [(i32, i32); 8] = [
    (-1, 0), (0, 1), // cqt13: 1º A × 2º B
    (1, 0), (2, 1), // cqt13: 1º C × 2º D
    (1, 1), (2, 0), // cqt13: 2º C × 1º D
    (0, 0), (-1, 1), // cqt13: 1º B × 2º A
];

/// O clube da posição `p` do grupo `g` (pela 0x6826d0 League::get_table_row: push p; mov ecx, grupo -> thiscall, ret 4).
pub(crate) unsafe fn cqt13_do_grupo(this: usize, g: i32, p: i32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let grupo = if g < 0 { this } else { read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4) };
    read_u32_as_usize(linha(grupo, p))
}

/// { 0x402090 } (thiscall, ret 0, african_nations.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AfricanNations::create_knockout_stage()`, as quartas da Copa Africana (da fase seguinte 0x401e10, ao chegar
/// a 3): a lista dos 8 (0x944e46(0x20): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x311 e volta) pelos
/// cruzamentos de CQT13_QUARTAS, na ordem das chamadas; a tabela das rodadas pela virtual +0x3c (push 0; push &id; push
/// &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 8, a lista, as rodadas, +4, a tabela, o ano +0x40, 3, 4, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][3]; a lista e a tabela liberadas (0x9452ca). Sem fase: a caixa da linha 0x32c e volta.
/// Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall, ret 4) e +0x30 = 3.
pub(crate) unsafe extern "thiscall" fn cqt13_africana_quartas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    let lista = alocar(0x20);
    if lista == 0 {
        return aviso_em(0x9870f8, 0x311);
    }
    crate::comparacao::ramo("cqt_13 quartas da copa africana");
    for (k, &(g, p)) in CQT13_QUARTAS.iter().enumerate() {
        write_u32_from_usize(lista + k * 4, cqt13_do_grupo(this, g, p));
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 3, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 8, lista, rodadas as u16 as u32, read_u32_as_usize(this + 4), tabela, read_i16(this + 0x40) as u16 as u32, 3, 4, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + 0xc, fase);
    soltar(lista);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if f == 0 {
        return aviso_em(0x9870f8, 0x32c);
    }
    montar(f, 0);
    write_u32_from_usize(this + 0x30, 3);
}

// ponytail: aloca a fase e monta a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][3] com 8 times,
// o +4 da copa e os 8 cruzados (lidos antes pelas tabelas, +0xb1 de cada grupo) na lista dela (+0xa7, 6 bytes; como
// conjunto) e +0x30 = 3.
pub(crate) unsafe extern "thiscall" fn comparar_cqt13_africana_quartas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQT13_AFRICANA_QUARTAS);
    let fases = read_u32_as_usize(this + 0xc);
    if fases == 0 || (0..3).any(|g| read_u32_as_usize(fases + g * 4) == 0) || read_u32_as_usize(this + 0xb1) == 0 {
        return o(this);
    }
    let tabela = |g: i32| if g < 0 { read_u32_as_usize(this + 0xb1) } else { read_u32_as_usize(read_u32_as_usize(fases + g as usize * 4) + 0xb1) };
    let mut plano: Vec<usize> = CQT13_QUARTAS.iter().map(|&(g, p)| read_u32_as_usize(tabela(g) + p as usize * 0x3b)).collect();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt13_africana_quartas({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 8 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4) && read_u32_as_usize(this + 0x30) == 3;
    PLACAR.registrar(igual, || format!("cqt13_africana_quartas({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// { 0x40dfa0 } (thiscall, ret 0, asia_cup_winner.cpp; a virtual +0x28): `AsiaCupWinner::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x328; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), as finais (0x40e030: mov ecx, this -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cqt13_recopa_asiatica_fase_seguinte(this: usize) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x987b60, 0x328);
    }
    let f = f + 1;
    write_u32_from_usize(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_13 finais da recopa asiatica");
        let finais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_ASIACUPWINNER_CREATE_FINALS_STAGE);
        finais(this);
    }
}

// ponytail: as finais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um (fora da última
// fase, que dá a caixa e não muda nada).
pub(crate) unsafe extern "thiscall" fn comparar_cqt13_recopa_asiatica_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQT13_RECOPA_ASIATICA_FASE_SEGUINTE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqt13_recopa_asiatica_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 4 finalistas da Recopa asiática: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da
/// copa, +0xa7) e depois os da segunda (a tabela +0xa7 da fase [+0xc][0]), até 4 ao todo, as duas buscas até o +0x3e da
/// copa (palavra com sinal); o 2º e o 3º trocam de lugar (os cruzamentos: as chaves são os pares da lista). As vagas
/// que sobrarem ficam 0 (a original as deixa com o que o malloc deu).
pub(crate) unsafe fn cqt13_finalistas(this: usize) -> [usize; 4] {
    let mut v = [0usize; 4];
    let mut n = 0i16;
    let times = read_i16(this + 0x3e);
    for tabela in [read_u32_as_usize(this + 0xa7), read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc)) + 0xa7)] {
        let mut i = 0i16;
        while i < times && n < 4 {
            let e = tabela + i as usize * 6;
            if *((e + 5) as *const u8) == 1 {
                v[n as usize] = read_u32_as_usize(e);
                n += 1;
            }
            i += 1;
        }
    }
    v.swap(1, 2); // cqt13: cruza o par do meio
    v
}

/// { 0x40e030 } (thiscall, ret 0, asia_cup_winner.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AsiaCupWinner::create_finals_stage()`: a lista dos 4 (0x944e46(0x10): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x34b e volta) de cqt13_finalistas; a tabela das rodadas pela virtual +0x3c (push 0; push
/// &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 1, 2, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][1]; a tabela liberada (0x9452ca). Sem fase: a caixa da linha 0x375 e volta (a lista não
/// é liberada, como na original). Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round: push 0; mov ecx, fase ->
/// thiscall, ret 4) e a lista liberada.
pub(crate) unsafe extern "thiscall" fn cqt13_recopa_asiatica_finais(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(0x987b60, 0x34b);
    }
    crate::comparacao::ramo("cqt_13 finalistas da recopa asiatica");
    // ponytail: com menos de 4 vivos a original deixa nas vagas o que o malloc deu; a cópia grava 0 (nesta carreira são 4)
    for (k, &c) in cqt13_finalistas(this).iter().enumerate() {
        write_u32_from_usize(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 4, lista, rodadas as u16 as u32, read_u32_as_usize(this + 4), tabela, read_i16(this + 0x40) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + 4, fase);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0x987b60, 0x375);
    }
    sortear(f, 0);
    soltar(lista);
}

// ponytail: aloca a fase e sorteia a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com 4
// times, o +4 da copa e os 4 finalistas (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto).
pub(crate) unsafe extern "thiscall" fn comparar_cqt13_recopa_asiatica_finais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQT13_RECOPA_ASIATICA_FINAIS);
    if read_u32_as_usize(this + 0xa7) == 0 || read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = cqt13_finalistas(this).to_vec();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt13_recopa_asiatica_finais({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("cqt13_recopa_asiatica_finais({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// rule: the Asian Cup Winners' Cup has, in each zone (stages −1 and 0, ids 0x480 and 0x481), three two-legged
/// qualifying rounds: 1st on 23/8, 2nd on 20/9, quarter-finals on 24 March of the following year (21 days between legs
/// in the 1st, 7 after); the final stage (stage 1) in May: semi-final on 24/5, third place and final on 26/5, single
/// matches; category 4 in the qualifiers and 6 after
/// Original: `AsiaCupWinner::create_round_table` (0x40cf90, asia_cup_winner.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x40cf90 } (thiscall, ret 0x10, asia_cup_winner.cpp; a virtual +0x3c): `AsiaCupWinner::create_round_table(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqp11_asia_recopa_fase`.
pub(crate) unsafe extern "thiscall" fn cqp11_asia_recopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp11_tabela(this, cqp11_asia_recopa_fase(fase), rodadas, extra, liga, 0x987b60)
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp11_asia_recopa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CQP11_ASIA_RECOPA_TABELA);
    cqp11_comparar("cqp11_asia_recopa_tabela", fase, cqp11_asia_recopa_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqp11_asia_recopa_tabela(this, fase, r, e, l))
}

/// rule: the Asian Cup Winners' Cup has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with
/// class and status 0
/// Original: `AsiaCupWinner::create_team_entries` (0x40d4d0, asia_cup_winner.cpp, thiscall, ret 0).
///
/// [pt] { 0x40d4d0 } (thiscall, ret 0, asia_cup_winner.cpp): `AsiaCupWinner::create_team_entries()`: os 16 times (a caixa
/// da linha 0x185 sem memória).
pub(crate) unsafe extern "thiscall" fn cqq12_asia_recopa_times(this: usize) -> i32 {
    cqq12_times(this, 0x987b60, 0x185, "cqq_12 times da recopa asiatica", |i| i) // cqq12: os times da Recopa asiática
}

pub(crate) unsafe extern "thiscall" fn comparar_cqq12_asia_recopa_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CQQ12_ASIA_RECOPA_TIMES);
    cqq12_comparar_times("cqq12_asia_recopa_times", this, || o(this), || cqq12_asia_recopa_times(this))
}
