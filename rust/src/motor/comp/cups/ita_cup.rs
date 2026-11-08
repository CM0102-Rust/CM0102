use super::*;



/// Italian cup create_first_group_table: sets n teams (+0x3e), allocates the table (+0xb1), puts the first n entrants
/// (+0xee) in its rows clearing their slots, sends the group draw news; returns 1.
///
/// [pt] A tabela do grupo 1 de uma copa italiana (`create_first_group_table`; a copa é ela mesma a liga do grupo 1): +0x3e =
/// `n` times; a tabela (0x944e46 de n × 0x3b, cdecl) em +0xb1; sem memória, a caixa da linha `linha` e 0. Cada um dos n
/// primeiros inscritos (+0xee, vetor de ponteiros) vai à linha i da tabela (League::init_table_row 0x687430: push 0; push
/// 0; push clube; push i (a original empurra o dword do byte local, com lixo da pilha em cima; a 0x687430 só lê o byte);
/// mov ecx, copa -> thiscall, ret 0x10) e a vaga dele em +0xee é zerada (sai da lista dos que esperam a próxima fase); o
/// índice é um byte com sinal comparado à palavra +0x3e. Depois a notícia do sorteio do grupo (League::send_group_draw_news
/// 0x684230, mov ecx -> thiscall, ret 0). Devolve 1.
pub(crate) unsafe fn create_first_group_table(this: usize, n: i16, arquivo: usize, linha: u32, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(n as i32 as u32 * 0x3b);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(ramo);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut i: i8 = 0;
    while (i as i16) < read_i16(this + 0x3e) {
        let vaga = read_u32_as_usize(this + 0xee) + i as isize as usize * 4;
        iniciar(this, i as u8 as u32, read_u32_as_usize(vaga), 0, 0);
        core::ptr::write_unaligned((read_u32_as_usize(this + 0xee) + i as isize as usize * 4) as *mut u32, 0); // cqr7: a vaga do grupo 1 sai da lista
        i = i.wrapping_add(1);
    }
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    noticia(this);
    1
}

/// Runs only the original and checks the pre-made plan: returns 1, +0x3e = n, table rows hold the first n entrants in
/// order, their +0xee slots cleared.
///
/// [pt] Só a original (simplification: a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos; não
/// pode rodar duas vezes), conferindo o plano feito antes: devolve 1, +0x3e = n, a tabela em +0xb1 com os n primeiros
/// inscritos nas linhas (de 0x3b bytes), na ordem, e as n primeiras vagas de +0xee zeradas.
pub(crate) unsafe fn compare_first_group_table(nome: &str, this: usize, n: i16, original: impl FnOnce() -> u32) -> u32 {
    let plano: Vec<usize> = (0..n as usize).map(|k| read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4)).collect();
    let r = original();
    let t = read_u32_as_usize(this + 0xb1);
    let ok = r == 1 && read_i16(this + 0x3e) == n && t != 0
        && plano.iter().enumerate().all(|(k, &c)| read_u32_as_usize(t + k * 0x3b) == c && read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) == 0);
    PLACAR.registrar(ok, || format!("{}({:#x}) devolveu {}; plano {:x?}", nome, this, r, plano));
    r
}

/// ItaCup::create_first_group_table: creates group 1 of the Coppa Italia with 4 teams (via the shared
/// create_first_group_table, error box at line 0x19e of ita_cup.cpp); returns 1, or 0 when out of memory.
///
/// [pt] { 0x63f250 } (thiscall, ret 0, ita_cup.cpp; do construtor): `ItaCup::create_first_group_table()`: o grupo 1 da Copa
/// da Itália com 4 times (`create_first_group_table`; a caixa do ita_cup.cpp na linha 0x19e).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_first_group_table(this: usize) -> u32 {
    create_first_group_table(this, 4, 0x9ea23c, 0x19e, "cqr_7 grupo 1 da copa da italia")
}


/// ItaCCup::init_first_group(), called from the constructor: builds the table of group 1 of the Italian Serie C cup
/// with 5 teams (via create_first_group_table). Returns 1, or 0 when memory runs out.
///
/// [pt] { 0x63d5e0 } (thiscall, ret 0, ita_c_cup.cpp; do construtor): `ItaCCup::init_first_group()`: o grupo 1 da Copa da
/// Itália da Série C com 5 times (`create_first_group_table`; a caixa do ita_c_cup.cpp na linha 0x1a0).
pub(crate) unsafe extern "thiscall" fn init_first_group(this: usize) -> u32 {
    create_first_group_table(this, 5, 0x9ea204, 0x1a0, "cqr_7 grupo 1 da copa da serie c") // cqr7: 5 times no grupo 1 da Série C
}


/// Italian cup advance_stage: error box on the last stage; else increments the stage (+0x30) and builds the knockout
/// when the groups end.
///
/// [pt] A fase seguinte de uma copa italiana (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal), a caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a `ultimo_grupo` + 1 (o fim dos grupos), monta o
/// mata-mata (`mata_mata`, call direto com o ecx da copa -> thiscall, ret 0).
pub(crate) unsafe fn advance_cup_stage(this: usize, arquivo: usize, linha: u32, fim_dos_grupos: i32, mata_mata: usize) {
    let fase = read_u32_as_usize(this + 0x30) as i32;
    if fase >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == fim_dos_grupos {
        crate::comparacao::ramo("cqr_7 mata-mata das copas italianas");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this); // cqr7: monta o mata-mata
    }
}

/// Comparer of advance_stage: original only, checking the stage (+0x30) goes up by one except on the last stage.
///
/// [pt] O comparador da fase seguinte: só a original (simplification: o mata-mata aloca a fase e sorteia; não pode rodar duas vezes),
/// conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
pub(crate) unsafe fn ita_cup_compare_advance_stage(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = read_u32_as_usize(this + 0x30) as i32;
    let ultima = antes >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) fase {} -> {}", nome, this, antes, depois));
}

/// ItaCup::advance_stage: moves the Italian Cup to its next stage (shared `advance_cup_stage`, error box at ita_cup.cpp
/// line 0x23e); after the group stages (stage 7) it builds the knockout rounds (ItaCup::create_knockout_stage
/// 0x63f870).
///
/// [pt] { 0x63f5b0 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x28): `ItaCup::advance_stage()`: a fase seguinte
/// (`advance_cup_stage`; a caixa do ita_cup.cpp na linha 0x23e); depois da fase 6 (os 8 grupos: o grupo 1 e as fases
/// 0..6), o mata-mata da Copa da Itália (ItaCup::create_knockout_stage 0x63f870).
pub(crate) unsafe extern "thiscall" fn ita_cup_advance_stage(this: usize) {
    advance_cup_stage(this, 0x9ea23c, 0x23e, 7, 0x63f870) // cqr7: o mata-mata da Copa da Itália na fase 7
}


/// ItaCCup::advance_stage() (virtual +0x28) for the Italian Serie C cup: moves the cup to its next stage (error box at
/// the last stage) and, once the 14 group stages are over, builds the knockout stage (ItaCCup::create_knockout_stage,
/// 0x63dc10).
///
/// [pt] { 0x63d950 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x28): `ItaCCup::advance_stage()`: a fase seguinte
/// (`advance_cup_stage`; a caixa do ita_c_cup.cpp na linha 0x240); depois da fase 14 (os 16 grupos), o mata-mata da Copa
/// da Série C (ItaCCup::create_knockout_stage 0x63dc10).
pub(crate) unsafe extern "thiscall" fn ita_cup_advance_stage_63d950(this: usize) {
    advance_cup_stage(this, 0x9ea204, 0x240, 0xf, 0x63dc10)
}






/// Italian cup create_group_stage(g): builds the League_Stage of group g with the next entrants from +0xee, stores it
/// in [+0xc][g], sends the draw news, sets +0x30 = g.
///
/// [pt] A fase do grupo `g` de uma copa italiana (`create_group_stage(g)`, quadro SEH só para o new: sem destrutor no caminho
/// normal; o molde da Copa da Itália e da Série C, conferido instrução a instrução: só mudam o fonte e as linhas das
/// caixas). A tabela das rodadas pelo método +0x3c da copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall,
/// ret 0x10); um vetor de +0x3e ponteiros (0x944e46, cdecl; sem memória, a caixa da `linhas[0]` e 0) com os primeiros
/// +0x3e inscritos ainda na lista (+0xee, vetor de ponteiros: os não nulos, na ordem, cada um zerado ao sair); a fase
/// (0x944cf1(0xee), cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (word
/// +0x3e), o vetor, 1, +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40
/// (word), g, o id (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da `linhas[1]` e 0. Com ela: a notícia do sorteio do grupo (0x684230
/// na fase), +0x30 = g e 1.
pub(crate) unsafe fn ita_cup_create_group_stage_cqr8_fase_do_grupo(this: usize, g: u32, arquivo: usize, linhas: [u32; 2], ramo: &'static str) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let times = alocar(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        show_error_at(arquivo, linhas[0]);
        return 0;
    }
    if read_i16(this + 0x3e) > 0 {
        let mut k = 0i32;
        let mut o = 0usize;
        loop {
            let c = read_u32_as_usize(read_u32_as_usize(this + 0xee) + o);
            if c != 0 {
                crate::comparacao::ramo(ramo);
                write_u32_from_usize(times + k as usize * 4, c); // cqr8: o time na vaga k do grupo
                write_u32_from_usize(read_u32_as_usize(this + 0xee) + o, 0);
                k += 1;
            }
            o += 4;
            if k >= read_i16(this + 0x3e) as i32 {
                break;
            }
        }
    }
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            read_i16(this + 0x3e) as u16 as u32,
            times,
            1,
            read_u32_as_usize(this + 4),
            tabela,
            rodadas as u16 as u32,
            read_u8(this + 0xc2) as u32,
            read_u8(this + 0xc3) as u32,
            read_u8(this + 0xc4) as u32,
            this + 0xc5,
            this + 0xbe,
            read_i16(this + 0x40) as u16 as u32,
            g,
            id as u16 as u32,
            read_u8(this + 0x51) as u32,
            1,
            0,
            read_i16(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
    if f == 0 {
        show_error_at(arquivo, linhas[1]);
        return 0;
    }
    noticia(f);
    write_u32_from_usize(this + 0x30, g as usize);
    1
}

/// Runs only the original and checks the plan: stage stored in [+0xc][g], +0x30 = g, team count/year, entrants cleared
/// from +0xee and present in the stage table.
///
/// [pt] Só a original (aloca a fase, que aloca a tabela dela, e manda a notícia; não pode rodar duas vezes), conferindo o plano
/// feito antes: devolve 1 com a fase em [+0xc][g] e +0x30 = g; a fase com os times (+0x3e) e o ano (+0x40) da copa; os
/// primeiros +0x3e inscritos não nulos de +0xee zerados e, na tabela da fase (+0xb1, linhas de 0x3b bytes), esses clubes
/// (em qualquer ordem: a fase ordena a tabela dela [inferência, como na fase dos grupos da Copa do Mundo]).
pub(crate) unsafe fn compare_group_stage(nome: &str, this: usize, g: u32, original: impl FnOnce() -> u32) -> u32 {
    let n = read_i16(this + 0x3e).max(0) as usize;
    let mut plano = Vec::new();
    let mut vagas = Vec::new();
    let mut o = 0usize;
    while plano.len() < n && o < 0x400 {
        let c = read_u32_as_usize(read_u32_as_usize(this + 0xee) + o);
        if c != 0 {
            plano.push(c);
            vagas.push(o);
        }
        o += 4;
    }
    let r = original();
    let mut ok = r == 1;
    if ok {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = read_u32_as_usize(f + 0xb1);
        ok = f != 0 && read_u32_as_usize(this + 0x30) == g as usize && read_i16(f + 0x3e) == read_i16(this + 0x3e) && read_i16(f + 0x40) == read_i16(this + 0x40)
            && vagas.iter().all(|&o| read_u32_as_usize(read_u32_as_usize(this + 0xee) + o) == 0)
            && (t == 0 || {
                let mut a: Vec<usize> = (0..plano.len()).map(|k| read_u32_as_usize(t + k * 0x3b)).collect();
                let mut b = plano.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
    }
    PLACAR.registrar(ok, || format!("{}({:#x}, {}) devolveu {}; plano {:x?}", nome, this, g, r, plano));
    r
}

/// ItaCup::create_group_stage(g): creates the league stage of group g of the Italian Cup from the entrants still
/// waiting, stores it in the cup's stage list and sends the group draw news. Returns 1, or 0 (after the original error
/// box) when allocation fails.
///
/// [pt] { 0x63f640 } (thiscall, ret 4, ita_cup.cpp): `ItaCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// (`ita_cup_create_group_stage_cqr8_fase_do_grupo`; as caixas do ita_cup.cpp nas linhas 0x263 e 0x27c).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_group_stage(this: usize, g: u32) -> u32 {
    ita_cup_create_group_stage_cqr8_fase_do_grupo(this, g, 0x9ea23c, [0x263, 0x27c], "cqr_8 time do grupo da copa da italia")
}


/// ItaCCup::create_group_stage(g): builds the stage of group `g` of the Serie C Italian Cup through the shared
/// group-stage builder (error boxes at ita_c_cup.cpp lines 0x265 and 0x27e). Returns the builder's result.
///
/// [pt] { 0x63d9e0 } (thiscall, ret 4, ita_c_cup.cpp): `ItaCCup::create_group_stage(g)`: a fase do grupo g da Copa da Itália
/// da Série C (`ita_cup_create_group_stage_cqr8_fase_do_grupo`; as caixas do ita_c_cup.cpp nas linhas 0x265 e 0x27e).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_group_stage_63d9e0(this: usize, g: u32) -> u32 {
    ita_cup_create_group_stage_cqr8_fase_do_grupo(this, g, 0x9ea204, [0x265, 0x27e], "cqr_8 time do grupo da copa da serie c")
}





/// League table (+0xb1, 0x3b-byte rows) of group i: group 0 is the cup itself, group i ≥ 1 is stage [+0xc][i - 1].
///
/// [pt] A tabela (+0xb1, linhas de 0x3b bytes) do grupo `i` de uma copa italiana: o grupo 0 é a própria copa; o grupo i ≥ 1 é
/// a fase [+0xc][i − 1].
pub(crate) unsafe fn group_table(this: usize, i: usize) -> usize {
    if i == 0 {
        read_u32_as_usize(this + 0xb1)
    } else {
        read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc) + (i - 1) * 4) + 0xb1)
    }
}

/// Copies the entrants still waiting in +0xee into the knockout team array from slot k up to fim, clearing each.
///
/// [pt] Os que entram direto no mata-mata: a partir da vaga `k` do vetor `times`, até `fim`, os inscritos ainda na lista (+0xee,
/// até +0xf2 vagas: os não nulos, na ordem), cada um zerado ao sair.
pub(crate) unsafe fn take_direct_entrants(this: usize, times: usize, mut k: usize, fim: usize) {
    let mut j = 0i32;
    if (read_u32_as_usize(this + 0xf2) as i32) <= 0 {
        return;
    }
    loop {
        if k >= fim {
            break;
        }
        let c = read_u32_as_usize(read_u32_as_usize(this + 0xee) + j as usize * 4);
        if c != 0 {
            write_u32_from_usize(times + k * 4, c);
            write_u32_from_usize(read_u32_as_usize(this + 0xee) + j as usize * 4, 0);
            k += 1;
        }
        j += 1;
        if j >= read_u32_as_usize(this + 0xf2) as i32 {
            break;
        }
    }
}

/// Builds the Cup_Stage knockout with n teams in [+0xc][fase] (rounds from virtual +0x3c), frees the temporaries and
/// draws the first round.
///
/// [pt] O mata-mata (Cup_Stage) com os `n` times de `times`: as rodadas pelo método +0x3c da copa (push 0; push &extra; push
/// &rodadas; push fase; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2), cdecl; Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, n, o vetor, as rodadas (word), +4, a tabela das rodadas, o ano +0x40 (word), a fase, 1, o
/// extra (word), 0x14, 0, 0, 0 e `sementes`; a original empurra os words com lixo nos 16 bits altos, que o construtor não
/// lê) em [+0xc][fase]; o vetor e a tabela das rodadas liberados (0x9452ca, cdecl; nesta ordem). Sem fase, a caixa da
/// `linha`; com ela, o sorteio da primeira rodada (Cup::draw_round 0x51c410: push 0; mov ecx, fase -> thiscall, ret 4).
pub(crate) unsafe fn create_knockout_stage(this: usize, n: u32, times: usize, fase: u32, sementes: *const u8, arquivo: usize, linha: u32) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, fase, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, n, times, rodadas as u16 as u32, read_u32_as_usize(this + 4), tabela, read_i16(this + 0x40) as u16 as u32, fase, 1, extra as u16 as u32, 0x14, 0, 0, 0, sementes)
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + fase as usize * 4, f);
    soltar(times);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + fase as usize * 4);
    if f == 0 {
        return show_error_at(arquivo, linha);
    }
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    sortear(f, 0);
}

/// ItaCup::create_knockout_stage(): builds the 24-team knockout stage (stage 7) of the Italian Cup from the 8 group
/// winners (top row of each group table) followed by the 16 seeded clubs that enter directly, with seed 3 for the group
/// winners and 2/1 for the direct entrants [inference: the draw pot of each slot]. Out of memory shows the error box
/// and returns.
///
/// [pt] { 0x63f870 } (thiscall, ret 0, ita_cup.cpp; da fase seguinte, depois da fase 6; quadro SEH só para o new, sem
/// destrutor): `ItaCup::create_knockout_stage()`: o mata-mata da Copa da Itália com 24 times: um vetor de 24 ponteiros
/// (0x944e46 de 0x60; sem memória, a caixa da linha 0x29c) com os 8 campeões dos grupos (a primeira linha da tabela de
/// cada grupo, `group_table`) e, nas vagas 8 a 23, os 16 cabeças de chave que entram direto (`take_direct_entrants`); a fase 7
/// (`create_knockout_stage`) com as sementes {3 × 8, 2 × 8, 1 × 8} (os campeões dos grupos, depois os cabeças de chave
/// [inferência: o pote de cada vaga no sorteio]) e a caixa da linha 0x2c0.
pub(crate) unsafe extern "thiscall" fn ita_cup_create_knockout_stage(this: usize) {
    let mut sementes = [0u8; 24];
    for (k, s) in sementes.iter_mut().enumerate() {
        *s = 3 - (k / 8) as u8;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let times = alocar(0x60);
    if times == 0 {
        return show_error_at(0x9ea23c, 0x29c);
    }
    crate::comparacao::ramo("cqr_9 mata-mata da copa da italia");
    for i in 0..8usize {
        write_u32_from_usize(times + i * 4, read_u32_as_usize(group_table(this, i))); // cqr9: o campeão do grupo i
    }
    take_direct_entrants(this, times, 8, 0x18);
    create_knockout_stage(this, 0x18, times, 7, sementes.as_ptr(), 0x9ea23c, 0x2c0);
}

/// Comparer of the knockout creation: after the original, checks the stage in [+0xc][fase] and the planned entrants
/// cleared from +0xee.
///
/// [pt] O plano dos dois mata-matas (feito antes): os campeões dos `grupos` grupos e os primeiros inscritos não nulos de +0xee
/// que cabem; o comparador confere, depois da original, a fase em [+0xc][fase] e esses inscritos zerados em +0xee.
pub(crate) unsafe fn ita_cup_compare_knockout_stage(nome: &str, this: usize, fase: usize, cabem: usize, original: impl FnOnce()) {
    let mut vagas = Vec::new();
    let mut j = 0usize;
    while vagas.len() < cabem && (j as i32) < read_u32_as_usize(this + 0xf2) as i32 {
        if read_u32_as_usize(read_u32_as_usize(this + 0xee) + j * 4) != 0 {
            vagas.push(j);
        }
        j += 1;
    }
    original();
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + fase * 4);
    let ok = f != 0 && vagas.iter().all(|&j| read_u32_as_usize(read_u32_as_usize(this + 0xee) + j * 4) == 0);
    PLACAR.registrar(ok, || format!("{}({:#x}) fase {:#x}, vagas tiradas {:?}", nome, this, f, vagas));
}


/// ItaCCup::create_knockout_stage: builds the 32-team knockout of the Italian Serie C cup: ranks the 16 group
/// runners-up in a temporary league table, takes the 16 group winners plus the 6 best runners-up (marking each one's
/// group +0xbe = 2 and row +0x37 = 6), adds the 10 seeded direct entrants and creates knockout stage 15 unseeded.
///
/// [pt] { 0x63dc10 } (thiscall, ret 0, ita_c_cup.cpp; da fase seguinte, depois da fase 14; quadro SEH só para os dois new, sem
/// destrutor no caminho normal): `ItaCCup::create_knockout_stage()`: o mata-mata da Copa da Itália da Série C com 32
/// times. 1) A classificação dos segundos colocados: uma liga provisória (0x944cf1(0xee); League_Stage 0x68b100,
/// thiscall, ret 0x58: a copa, 16, 0, 0, +4, 0, 0, +0xc2, +0xc3, 0, &{1, 2, 3, 0} (os critérios de desempate), &+0xbe, o
/// ano +0x40 (word), −2, 0, 0, 1, 0, 2, −1, 0, 2) cuja tabela recebe, na linha i, a segunda linha (+0x3b) da tabela do
/// grupo i (os 0x3b bytes copiados) com +5 e +9 zerados, ordenada (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall,
/// ret 4). 2) Um vetor de 32 ponteiros (0x944e46 de 0x80; sem memória, a caixa da linha 0x2ba) com os 16 campeões dos
/// grupos (a primeira linha de cada tabela) e, nas vagas 16 a 21, os 6 melhores segundos (as 6 primeiras linhas da
/// classificação): cada um, achado na tabela do seu grupo (o primeiro grupo em que aparece), marca o grupo (+0xbe = 2) e a
/// linha dele (+0x37 = 6). 3) A liga provisória destruída (a virtual 0 com 1). 4) Nas vagas 22 a 31, os 10 cabeças de chave
/// que entram direto (`take_direct_entrants`). 5) A fase 15 (`create_knockout_stage`), sem sementes, e a caixa da linha 0x304.
pub(crate) unsafe extern "thiscall" fn ita_cup_create_knockout_stage_63dc10(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let desempate: [u8; 4] = [1, 2, 3, 0];
    let p = novo(0xee);
    let liga = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        construir(p, this, 0x10, 0, 0, read_u32_as_usize(this + 4), 0, 0, *((this + 0xc2) as *const u8) as u32, *((this + 0xc3) as *const u8) as u32, 0,
            desempate.as_ptr() as usize, this + 0xbe, read_i16(this + 0x40) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for i in 0..16usize {
        let de = group_table(this, i) + 0x3b;
        let para = read_u32_as_usize(liga + 0xb1) + i * 0x3b;
        core::ptr::copy_nonoverlapping(de as *const u8, para as *mut u8, 0x3b); // cqr9: o segundo do grupo i
        write_u32_from_usize(read_u32_as_usize(liga + 0xb1) + i * 0x3b + 9, 0);
        write_u32_from_usize(read_u32_as_usize(liga + 0xb1) + i * 0x3b + 5, 0);
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    ordenar(liga, 0); // cqr9: a classificação dos segundos
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let times = alocar(0x80);
    if times == 0 {
        return show_error_at(0x9ea204, 0x2ba);
    }
    crate::comparacao::ramo("cqr_9 mata-mata da copa da serie c");
    for i in 0..16usize {
        write_u32_from_usize(times + i * 4, read_u32_as_usize(group_table(this, i)));
    }
    let mut l = 0usize;
    while l < 0x162 {
        let c = read_u32_as_usize(read_u32_as_usize(liga + 0xb1) + l);
        write_u32_from_usize(times + 0x40 + (l / 0x3b) * 4, c); // cqr9: os 6 melhores segundos
        let mut g = 0usize;
        while g < 0x10 {
            let grupo = if g == 0 { this } else { read_u32_as_usize(read_u32_as_usize(this + 0xc) + (g - 1) * 4) };
            let n = read_i16(grupo + 0x3e) as i32;
            let mut r = 0i32;
            while r < n {
                if read_u32_as_usize(read_u32_as_usize(grupo + 0xb1) + r as usize * 0x3b) == read_u32_as_usize(read_u32_as_usize(liga + 0xb1) + l) {
                    *((grupo + 0xbe) as *mut u8) = 2;
                    *((read_u32_as_usize(grupo + 0xb1) + r as usize * 0x3b + 0x37) as *mut u8) = 6;
                    g = 0x10;
                    break;
                }
                r += 1;
            }
            g += 1;
        }
        l += 0x3b;
    }
    if liga != 0 {
        let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(liga)));
        destruir(liga, 1);
    }
    take_direct_entrants(this, times, 0x16, 0x20);
    create_knockout_stage(this, 0x20, times, 0xf, core::ptr::null(), 0x9ea204, 0x304);
}


/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
pub(crate) type ItaCupGroupRound = (u32, u32, u32, u32);

/// Italian cups create_schedule (virtual +0x3c): group round table (0x41 bytes/round) below limit, knockout round
/// table (0x68 bytes/round) at limit; returns the table or 0.
///
/// [pt] As duas fases das copas italianas (a virtual +0x3c, `create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret
/// 0x10; a fase lida como byte com sinal). Abaixo de `limite` (−1 inclusive), os grupos: é_liga (se pedido) = 1, rodadas
/// = o número de rodadas de grupo, extra = fase + `id` (16 bits); a tabela de 0x41 bytes por rodada (0x944e46; sem
/// memória, a caixa `linha_g` e devolve 0) com as datas de `grupos`. Na fase `limite`, o mata-mata: é_liga = 0, rodadas =
/// o tamanho de `mata`, extra = 0; a tabela de 0x68 bytes por rodada (sem memória, a caixa `linha_m`), as rodadas pela
/// `eng_cc_cup_write_cup_round` com o ano +0x40 da copa (com sinal). Acima de `limite`: devolve 0 sem gravar nada.
pub(crate) unsafe fn create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, limit: i8, id: i16, grupos: &[ItaCupGroupRound],
    mata: &[EngCcCupCupRound], arquivo: usize, linha_g: u32, linha_m: u32, nome_g: &'static str, nome: &'static str) -> usize {
    let f = fase as u8 as i8;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    if f < limit {
        let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
        let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
        if !liga.is_null() {
            *liga = 1;
        }
        *rodadas = grupos.len() as i16;
        *extra = (f as i16).wrapping_add(id);
        let t = alocar(grupos.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(arquivo, linha_g);
            return 0;
        }
        crate::comparacao::ramo(nome_g);
        // o ano vai com lixo nos bytes altos na original (o registrador de antes com a palavra +0x40 por cima); a
        // 0x68a160 só usa os 16 bits de baixo: a cópia passa o ano limpo
        let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
        for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
            data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, ano, 0);
            jogo(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limit {
        return 0;
    }
    if !liga.is_null() {
        *liga = 0;
    }
    *rodadas = mata.len() as i16;
    *extra = 0;
    let t = alocar(mata.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(arquivo, linha_m);
        return 0;
    }
    crate::comparacao::ramo(nome);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in mata.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, ano);
    }
    t
}

/// The bytes the original writes in a group schedule table of n 0x41-byte rounds (date/code, first match, value), for
/// comparison.
///
/// [pt] Os bytes que a original grava numa tabela de grupos de `n` rodadas de 0x41 bytes (o resto fica como veio da
/// alocação): de cada rodada, +0..+4 (a data e o código), o jogo 0 (+5..+0xb) e +0x3d..+0x40 (o valor).
pub(crate) unsafe fn written_group_schedule_bytes(t: usize, n: i16) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..n.max(0) as usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
    }
    v
}

/// Side-by-side comparer of create_schedule: outputs and written table bytes must match; frees the copy's table.
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas (de grupos ou de mata-mata, pelo é_liga
/// da cópia) têm de ser iguais; a tabela da cópia é liberada (0x9452ca).
pub(crate) unsafe fn ita_cup_compare_schedule(nome: &str, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize, limit: i8) -> usize {
    if fase as u8 as i8 > limit {
        // simplification: acima do mata-mata nenhuma das duas grava nada; as saídas podem nem existir
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (liga.is_null() || *liga == l2);
    let grupos = (fase as u8 as i8) < limit;
    let tabela = if a != 0 && b != 0 {
        if grupos { written_group_schedule_bytes(a, *rodadas) == written_group_schedule_bytes(b, r2) } else { eng_cc_cup_written_round_bytes(a, *rodadas) == eng_cc_cup_written_round_bytes(b, r2) }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa da Itália, grupos: três rodadas em agosto (12 e 19, sábado [inferência: dia da semana 6], e 29, terça [2]).
pub(crate) const ITALIAN_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 3] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x1d, 7, 2, 2)];

/// A Copa da Itália, mata-mata: cinco rodadas em ida e volta, de setembro a março do ano seguinte.
pub(crate) const ITALIAN_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x1e, 7, 0, 3), (0x13, 8, 0, 2, 2, 0), 0xa, (0, 0x400, 3), 4, (0x10, 8, 0x10, 0), (0, 2, 0x23), (0, 0xffff_ffff, 0)), // cqo10: a 1ª rodada do mata-mata da Copa da Itália
    cql10r((0x19, 9, 0, 3), (0xb, 0xa, 0, 6, 1, 0), 0x14, (0, 0x400, 3), 4, (0x10, 8, 8, 0x10), (0, 2, 0x11), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 0xa, 0, 3), (0xc, 0xb, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0x1c), (0, 0xffff_ffff, 0)),
    cql10r((0xa, 0, 1, 3), (0x17, 0, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((7, 1, 1, 3), (6, 2, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 0x39), (0, 0xffff_ffff, 0)),
];

/// rule: the Coppa Italia starts with three-round groups in August (12 and 19, Saturday [inference: weekday 6], and 29,
/// Tuesday), each group a league stage (is_league = 1, id 0x3e9 + stage); then the knockout (stage 7) over two legs:
/// 1st round drawn on 30/8 and played on 19/9 (16 teams from the groups and 16 new), 2nd draw 25/10, match 11/11 (8
/// new), quarter-finals 29/11 and 12/12, semi-final 10/1 and 23/1, final draw 7/2 and legs on 6/3 and 57 days later;
/// category 4 up to the 2nd round and 6 after; no per-round prize (−1)
/// Original: `ItaCup::create_schedule` (0x63ec50, ita_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63ec50 } (thiscall, ret 0x10, ita_cup.cpp; a virtual +0x3c da Copa da Itália): `ItaCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 7 (extra = fase + 0x3e9; a caixa da linha 0xc1), o mata-mata na
/// fase 7 (a caixa da linha 0xe1).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_schedule(this, fase, rodadas, extra, liga, 7, 0x3e9, &ITALIAN_CUP_GROUP_ROUNDS, &ITALIAN_CUP_KNOCKOUT_ROUNDS, 0x9ea23c, 0xc1, 0xe1,
        "cqo_10 grupos da copa da italia", "cqo_10 mata-mata da copa da italia")
}


/// A Copa da Itália da Série C, grupos: cinco rodadas em agosto (12, 19, 22, 26 e 29).
pub(crate) const SERIE_C_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 5] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x16, 7, 2, 2), (0x1a, 7, 6, 1), (0x1d, 7, 2, 2)]; // cqo10: os grupos da Copa da Série C

/// A Copa da Itália da Série C, mata-mata: cinco rodadas em ida e volta, de setembro a maio do ano seguinte (as
/// semifinais em abril).
pub(crate) const SERIE_C_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x1e, 7, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x14, (0, 0, 3), 4, (0x20, 0x10, 0x20, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 9, 0, 3), (1, 0xa, 0, 2, 2, 0), 0x1e, (0, 0, 3), 4, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0xf, 0xa, 0, 2, 2, 0), 0x78, (0, 0, 3), 6, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0xa, 0, 3), (0xb, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 3, 1, 3), (9, 4, 1, 2, 2, 0), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
];

/// rule: the Serie C Cup starts with five-round groups in August (12, 19, 22, 26 and 29; id 0x420 + stage); the
/// knockout (stage 0xf) over two legs: 1st round drawn on 30/8 and played on 11/10 (32 teams), 2nd 26/10 and 1/11,
/// quarter-finals 9/11 and 15/11, semi-final 30/11 and 11 April, final draw 26/4 and match on 9 May; 14 days between
/// legs, 7 in the 2nd round and the final; category 4 up to the 2nd round and 6 after
/// Original: `ItaCCup::create_schedule` (0x63cf90, ita_c_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63cf90 } (thiscall, ret 0x10, ita_c_cup.cpp; a virtual +0x3c da Copa da Série C): `ItaCCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: grupos abaixo da fase 0xf (extra = fase + 0x420; a caixa da linha 0xbc), o mata-mata na
/// fase 0xf (a caixa da linha 0xe3).
pub(crate) unsafe extern "thiscall" fn ita_cup_create_schedule_63cf90(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_schedule(this, fase, rodadas, extra, liga, 0xf, 0x420, &SERIE_C_CUP_GROUP_ROUNDS, &SERIE_C_CUP_KNOCKOUT_ROUNDS, 0x9ea204, 0xbc, 0xe3,
        "cqo_10 grupos da copa da serie c", "cqo_10 mata-mata da copa da serie c")
}


/// A Supercopa da Itália: um jogo único em 19 de agosto (sorteio em 5 de julho), mando neutro [inferência: o +0x13 da
/// data fica 0], sem prêmios.
pub(crate) const ITALIAN_SUPER_CUP: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (0x13, 7, 0, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqo10: a Supercopa da Itália
];

/// rule: the Italian Super Cup is a single match (2 teams) on 19 August, drawn on 5 July, category 6, tie-break 0x203,
/// no prizes
/// Original: `ItaSuper::create_round_table` (0x65f920, ita_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x65f920 } (thiscall, ret 0x10, ita_super.cpp; a virtual +0x3c da Supercopa da Itália): `ItaSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
pub(crate) unsafe extern "thiscall" fn ita_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &ITALIAN_SUPER_CUP, 0x9eac34, 0x99, "cqo_10 tabela da supercopa")
}


/// A Supercopa da Série C1: um jogo único em 21 de maio do ano seguinte (sorteio em 15 de maio), sem prêmios.
pub(crate) const SERIE_C1_SUPER_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((0xf, 4, 1, 0), (0x15, 4, 1, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)),
];

/// rule: the Serie C1 Super Cup is a single match on 21 May of the following year, drawn on 15 May, category 6, no
/// prizes
/// Original: `ItaC1Super::create_round_table` (0x63c890, ita_c1_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x63c890 } (thiscall, ret 0x10, ita_c1_super.cpp; a virtual +0x3c da Supercopa da Série C1): `ItaC1Super::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
pub(crate) unsafe extern "thiscall" fn ita_cup_create_round_table_63c890(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &SERIE_C1_SUPER_CUP_ROUNDS, 0x9ea1c8, 0x99, "cqo_10 tabela da supercopa c1")
}



/// Clubs of G_CLUB_TABLE the pass looks at: G_CLUB_COUNT - 2 × G_NATION_COUNT (without national teams [inference]).
///
/// [pt] Os clubes da tabela de clubes ([0xae23b4], 0x245 bytes) que a passada olha: [0xae235c] − 2 × [0xae2350] (sem as
/// seleções [inferência]).
pub(crate) unsafe fn ita_cup_club_count_without_nations() -> i32 {
    (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2))
}



/// True when the club's nation (+0x53) is Italy (KEY_NATION_ITALY).
///
/// [pt] O clube é da Itália (+0x53 a nação; [0x9cf360] o id da Itália).
pub(crate) unsafe fn is_italian_club(c: usize) -> bool {
    comp_id_at(c, 0x53) == Some(read_u32_as_usize(KEY_NATION_ITALY))
}

/// Club's previous-season league position (+0x5f, signed byte; 0 = champion [inference]).
///
/// [pt] A posição na temporada anterior (+0x5f, byte com sinal; 0 o campeão [inferência]).
pub(crate) unsafe fn last_season_position(c: usize) -> i8 {
    *((c + 0x5f) as *const i8)
}


/// Shuffles n pointers from v (CompUtil shuffle_pointer_array, cdecl).
///
/// [pt] Embaralha `n` ponteiros a partir de `v` (0x4afb40: push n; push v; add esp junto -> cdecl; já em Rust).
pub(crate) unsafe fn shuffle_pointers(v: usize, n: u32) {
    let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    f(v, n);
}

/// qsort of n pointers from v with comparison cmp (CRT qsort).
///
/// [pt] O qsort de `n` ponteiros a partir de `v` com a comparação `cmp` (0x945333: 4 pushes; add esp, 0x10 -> cdecl).
pub(crate) unsafe fn sort_pointers(v: usize, n: u32, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    ordenar(v, n, 4, cmp);
}

/// Allocates the entrant pointer list (+0xee) with max entries in +0xf2; on failure shows the error box for linha and
/// returns None.
///
/// [pt] A lista de ponteiros (+0xee) com o máximo `max` em +0xf2 (dword): 0x944e46 de 4 × max; sem memória, a caixa `linha`.
pub(crate) unsafe fn alloc_entrant_list(this: usize, max: u32, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, max);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(max * 4);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(arquivo, linha);
        return None;
    }
    Some(l)
}

/// Appends club c to the entrant list (+0xee); true when the count reaches the max (+0xf2).
///
/// [pt] Põe `c` no fim da lista (+0xee) e devolve verdadeiro quando a conta chega ao máximo +0xf2 (dword).
pub(crate) unsafe fn push_entrant(this: usize, n: &mut i32, c: usize) -> bool {
    *n += 1;
    core::ptr::write_unaligned((read_u32_as_usize(this + 0xee) + (*n as usize - 1) * 4) as *mut u32, c as u32);
    *n == read_u32_as_usize(this + 0xf2) as i32
}

/// ItaCup::select_entrants: the 48 Coppa Italia entrants (32 group teams drawn from 4 pots, then Serie B/A qualifiers);
/// returns the result pair.
///
/// [pt] { 0x63faf0 } (thiscall, ret 0, ita_cup.cpp): `ItaCup::select_entrants()`: os 48 inscritos da Copa da Itália (+0xf2 =
/// 0x30; a lista de 0xc0 bytes em +0xee; sem memória, a caixa da linha 0x2e1 e 0). Os 32 dos grupos num vetor local:
/// os clubes italianos da temporada (passada única) que não vieram da Série B ([0x9cf574]) entre os 3 primeiros nem da
/// Série A ([0x9cf570]) entre os 15 primeiros: os da Série B atual, os que vieram da Série B em 4º, 5º ou abaixo do 17º,
/// e os que vieram da C1 ([0x9cf578]/[0x9cf57c]) em 4º, 5º ou 6º; faltando, os da C1 fora da Série B pela posição (6º,
/// 7º...). O vetor vai ao qsort (0x640590) e é repartido em 4 potes de 8: 8 voltas, cada uma tira um clube sorteado de
/// cada pote (0x90cfc0(8), seguindo para o próximo do pote se já saiu). Depois os 3 primeiros da Série B anterior e os
/// da Série A anterior do 10º ao 15º (até completar 48) e, sem conferir antes, os 9 primeiros da Série A anterior.
/// Devolve 1.
pub(crate) unsafe fn select_coppa_italia_entrants(this: usize) -> (i32, i32) {
    let l = match alloc_entrant_list(this, 0x30, 0x9ea23c, 0x2e1) {
        Some(l) => l,
        None => return (0, 0),
    };
    let (serie_a, serie_b, c1a, c1b) = (read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut v = [0usize; 32];
    let mut k = 0usize;
    let n = ita_cup_club_count_without_nations();
    'grupos: {
        let mut i = 0;
        while i < n {
            let c = jap_j_cup_club_by_index(i);
            i += 1;
            if !is_italian_club(c) {
                continue;
            }
            let antes = comp_id_at(c, 0x5b);
            if antes == Some(serie_b) && last_season_position(c) < 3 || antes == Some(serie_a) && last_season_position(c) < 0xf {
                continue;
            }
            if comp_id_at(c, 0x57) == Some(serie_b) {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
                continue;
            }
            let p = last_season_position(c);
            let entra = match antes {
                None => false,
                Some(x) if x == serie_b => p == 3 || p == 4 || p > 0x10,
                Some(x) => (x == c1a || x == c1b) && (p == 3 || p == 4 || p == 5),
            };
            if entra {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
            }
        }
        let mut lugar = 5i8;
        while k < 32 {
            let mut i = 0;
            while i < n {
                let c = jap_j_cup_club_by_index(i);
                i += 1;
                if !is_italian_club(c) {
                    continue;
                }
                let agora = comp_id_at(c, 0x57);
                if agora.is_none() || agora == Some(serie_b) {
                    continue;
                }
                let antes = comp_id_at(c, 0x5b);
                if (antes == Some(c1a) || antes == Some(c1b)) && last_season_position(c) == lugar {
                    crate::comparacao::ramo("cqo_11 completar com a c1");
                    v[k] = c;
                    k += 1;
                    if k == 32 {
                        break;
                    }
                }
            }
            lugar = lugar.wrapping_add(1);
        }
    }
    sort_pointers(v.as_mut_ptr() as usize, k as u32, 0x640590);
    let mut m = 0i32;
    for _ in 0..8 {
        let mut pote = 0usize;
        while pote < 32 {
            crate::comparacao::ramo("cqo_11 potes da copa da italia");
            let mut r = jap_j_cup_random(8) as i8 as i32; // cqo11: o clube sorteado de cada pote
            while v[pote + r as usize] == 0 {
                r += 1;
                if r == 8 {
                    r = 0;
                }
            }
            m += 1;
            core::ptr::write_unaligned((l + (m as usize - 1) * 4) as *mut u32, v[pote + r as usize] as u32);
            v[pote + r as usize] = 0;
            pote += 8;
        }
    }
    'b: {
        let mut i = 0;
        while i < ita_cup_club_count_without_nations() {
            let c = jap_j_cup_club_by_index(i);
            i += 1;
            if !is_italian_club(c) {
                continue;
            }
            let antes = comp_id_at(c, 0x5b);
            let p = last_season_position(c);
            if antes == Some(serie_b) && p < 3 || antes == Some(serie_a) && p > 8 && p < 0xf { // cqo11: os que entram na 2ª rodada
                if push_entrant(this, &mut m, c) {
                    break 'b;
                }
            }
        }
    }
    let mut i = 0;
    while i < ita_cup_club_count_without_nations() {
        let c = jap_j_cup_club_by_index(i);
        i += 1;
        if is_italian_club(c) && comp_id_at(c, 0x5b) == Some(serie_a) && last_season_position(c) < 9 {
            if push_entrant(this, &mut m, c) {
                break;
            }
        }
    }
    (1, m)
}

/// rule: 48 Italian clubs enter the Coppa Italia. The 32 of the group stage: the season's Serie B clubs, those that
/// came from Serie B in 4th, 5th or from 18th down and those that came from C1 in 4th, 5th or 6th (positions counted
/// from 0 [inference]); if short, C1 clubs by position (6th, 7th...). The 32 are sorted (qsort 0x640590) and split into
/// 4 pots of 8, and each of the 8 rounds draws one club from each pot. In the 2nd round enter the top 3 of last
/// season's Serie B (the promoted) and the Serie A clubs from 10th to 15th; last, the top 9 of Serie A, up to 48 (the
/// rest are left out by table order)
/// Original: `ItaCup::select_entrants` (0x63faf0, ita_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Itália 48 clubes italianos. Os 32 da fase de grupos: os da Série B da temporada, os que
/// vieram da Série B em 4º, 5º ou do 18º para baixo e os que vieram da C1 em 4º, 5º ou 6º (posições contadas de 0
/// [inferência]); faltando, os da C1 pela posição (6º, 7º...). Os 32 são ordenados (qsort 0x640590) e repartidos em 4
/// potes de 8, e cada uma das 8 voltas sorteia um clube de cada pote. Na 2ª rodada entram os 3 primeiros da Série B
/// anterior (os que subiram) e os da Série A do 10º ao 15º; por último os 9 primeiros da Série A, até completar 48 (os
/// que passam disso ficam de fora pela ordem da tabela)
pub(crate) unsafe extern "thiscall" fn ita_cup_select_entrants(this: usize) -> i32 {
    select_coppa_italia_entrants(this).0
}

/// Side-by-side comparer for entrant selection routines that allocate +0xee and draw: compares +0xf2, 0xb67a34, RNG
/// state and written entrants; frees the copy's list.
///
/// [pt] Lado a lado para as rotinas que alocam a lista de ponteiros +0xee e sorteiam: o +0xf2, o [0xb67a34] e o estado do
/// sorteador a partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xee, a cópia faz
/// a sua, comparada nos inscritos que ela gravou e liberada.
pub(crate) unsafe fn compare_select_entrants(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(c);
        lr.set(read_u32_as_usize(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    let ver = |l: usize| if l == 0 { Vec::new() } else { core::slice::from_raw_parts(l as *const u32, k / 4).to_vec() };
    let (vo, vr) = (ver(lo), ver(lr));
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} {:x?} x {:x?} lista {} ({} bytes) {:x?} x {:x?}", nome, this, a, b, igual, mo, mr, lista, k, vo, vr));
    a
}


/// ItaCCup::select_entrants: the 90 Serie C cup entrants from C1/C2, reordered, shuffled and sorted by longitude into
/// groups.
///
/// [pt] { 0x63dfe0 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::select_entrants()`: os 90 inscritos da Copa da Série C (+0xf2
/// = 0x5a; a lista de 0x168 bytes em +0xee; sem memória, a caixa da linha 0x322 e 0): os clubes italianos da C1 e da C2
/// ([0x9cf578]..[0x9cf588]) pela ordem da tabela, até 90. Dos 80 primeiros, os que vieram da Série B ([0x9cf574]) e os
/// que vieram da C1 em 4º, 5º ou 6º vão, um a um, para as vagas 80, 81... (trocados com o que estava lá). Os 80 primeiros
/// são embaralhados (0x4afb40), ordenados pela longitude (qsort com 0x53fe50) e cada uma das 8 dezenas deles
/// embaralhada (os grupos por região [inferência]); as 10 últimas vagas ficam como estão. Devolve 1.
pub(crate) unsafe fn select_serie_c_cup_entrants(this: usize) -> (i32, i32) {
    let l = match alloc_entrant_list(this, 0x5a, 0x9ea204, 0x322) {
        Some(l) => l,
        None => return (0, 0),
    };
    let divisoes = [read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_B), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C2_C)];
    let mut m = 0i32;
    let mut i = 0;
    while i < ita_cup_club_count_without_nations() {
        let c = jap_j_cup_club_by_index(i);
        i += 1;
        if !is_italian_club(c) {
            continue;
        }
        match comp_id_at(c, 0x57) {
            Some(d) if divisoes.contains(&d) => {
                if push_entrant(this, &mut m, c) {
                    break;
                }
            }
            _ => {}
        }
    }
    let (serie_b, c1a, c1b) = (read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_B), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), read_u32_as_usize(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut j = 0x50i32;
    for idx in 0..0x50usize {
        let c = read_u32_as_usize(l + idx * 4);
        let antes = match comp_id_at(c, 0x5b) {
            Some(x) => x,
            None => continue,
        };
        let p = last_season_position(c);
        if antes == serie_b || (antes == c1a || antes == c1b) && (p == 3 || p == 4 || p == 5) {
            crate::comparacao::ramo("cqo_11 vagas do fim da copa da serie c");
            let e = l + (j as i16 as isize * 4) as usize;
            let t = read_u32_as_usize(e);
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            j += 1;
            core::ptr::write_unaligned((l + idx * 4) as *mut u32, t as u32);
            if j as i16 as i32 >= read_u32_as_usize(this + 0xf2) as i32 {
                break;
            }
        }
    }
    shuffle_pointers(l, 0x50);
    sort_pointers(l, 0x50, 0x53fe50);
    crate::comparacao::ramo("cqo_11 dezenas da copa da serie c");
    for k in 0..8usize {
        shuffle_pointers(l + k * 0x28, 0xa); // cqo11: as dezenas da Copa da Série C
    }
    (1, m)
}

/// rule: the Serie C Cup takes the 90 Italian clubs of C1 and C2 (the two C1 groups and the three C2 ones). Those that
/// came down from Serie B and those that came from C1 in 4th, 5th or 6th go to the last places (from the 81st); the
/// first 80 are shuffled, sorted by the city's longitude and each of their 8 tens shuffled (the last 10 places stay in
/// order): the groups are regional [inference]
/// Original: `ItaCCup::select_entrants` (0x63dfe0, ita_c_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Copa da Série C os 90 clubes italianos da C1 e da C2 (as duas chaves da C1 e as três da C2). Os que
/// vieram da Série B (rebaixados) e os que vieram da C1 em 4º, 5º ou 6º vão para as últimas vagas (a partir da 81ª); os
/// 80 primeiros são embaralhados, ordenados pela longitude da cidade e cada uma das suas 8 dezenas embaralhada (as 10
/// últimas vagas ficam na ordem): os grupos são regionais [inferência]
pub(crate) unsafe extern "thiscall" fn ita_cup_select_entrants_63dfe0(this: usize) -> i32 {
    select_serie_c_cup_entrants(this).0
}


/// ItaSuper::init_team_list: the two Supercoppa teams (Serie A champion and Coppa Italia winner, or the cup runner-up
/// if the same club).
///
/// [pt] { 0x65fa60 } (thiscall, ret 0, ita_super.cpp): `ItaSuper::init_team_list()`: a lista de dois (+0x3e = 2, 0xc bytes
/// em +0xa7; sem memória, a caixa da linha 0xc2 e −1). O campeão da Série A ([0x9cf570]; sem ele, a caixa 0xcb e 0) e o
/// da Copa da Itália ([0x9cf71c]; sem ele, 0xd4 e 0); se é o mesmo clube, o segundo vira o vice da Copa (sem ele, 0xe0
/// e 0). Classe e situação 0 nos dois; devolve 1.
pub(crate) unsafe fn init_supercoppa_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x9ea934;
    let l = match alloc_two_team_list(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_ITALIAN_SERIE_A, 0); // cqo11: o campeão da Série A
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_ITALIAN_CUP, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqo_11 dobradinha italiana");
        if !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_ITALIAN_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqo_11 clubes da supercopa da italia");
    finish_two_team_list(l)
}

/// rule: the Italian Super Cup is between the Serie A champion (the home side) and the Coppa Italia winner; if the same
/// club won both, the Cup runner-up plays
/// Original: `ItaSuper::init_team_list` (0x65fa60, ita_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da Itália é entre o campeão da Série A (o mandante) e o campeão da Copa da Itália; se o mesmo
/// clube ganhou as duas, entra o vice da Copa
pub(crate) unsafe extern "thiscall" fn ita_cup_init_team_list(this: usize) -> i32 {
    init_supercoppa_teams(this).0
}


/// As regras que a virtual +0x8c (set_up_rules) de uma copa com fase de grupos grava na liga do grupo, antes da tabela
/// das rodadas. Desempates: os quatro códigos de +0xc5..+0xc8 (docs/.md §2.7).
#[derive(Clone, Copy)]
pub(crate) struct GroupStageRules {
    pub(crate) vitoria: u8,     // +0xc2: pontos por vitória
    pub(crate) empate: u8,      // +0xc3: pontos por empate
    pub(crate) categoria: u8,   // +0xc4: a categoria de cada jogo (vai para o +0x3a da partida)
    pub(crate) desempate: [u8; 4], // +0xc5..+0xc8
    pub(crate) vagas: u8,       // +0xbe: vagas de cima (classificados por grupo)
    pub(crate) premio: Option<u32>, // +0x28: o fundo de prêmios (a Copa dos Campeões não grava)
    pub(crate) bits: Option<u16>,   // +0xd9: a palavra de bits de regra (só a Copa dos Campeões grava: 8)
}

pub(crate) const ITALIAN_CUP_GROUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0xc3500), bits: None };
pub(crate) const SERIE_C_CUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0x493e0), bits: None };
pub(crate) const CHAMPIONS_CUP_RULES: GroupStageRules = GroupStageRules { vitoria: 3, empate: 1, categoria: 0x17, desempate: [3, 1, 2, 4], vagas: 2, premio: None, bits: Some(8) };

/// Writes rules g into league this (no promotion/relegation, one leg, club type, no play-offs) and its round table from
/// virtual +0x3c; returns 1.
///
/// [pt] Grava as regras `g` na liga `this`: sobe/cai para nenhuma (+0x1c, +0x20 = −1), um turno (+0x3c = 1), o tipo 1
/// (+0x42, clubes), sem play-offs nem rebaixamento (+0xbf, +0xc0, +0xc1 = 0); depois a tabela das rodadas da virtual
/// +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
pub(crate) unsafe fn set_cup_league_rules(this: usize, g: &GroupStageRules, turnos: i16) -> u32 {
    let b = |o: usize, v: u8| *((this + o) as *mut u8) = v;
    b(0xc2, g.vitoria);
    b(0xc3, g.empate);
    b(0xc4, g.categoria);
    for (k, &v) in g.desempate.iter().enumerate() {
        b(0xc5 + k, v);
    }
    b(0xbe, g.vagas);
    b(0xbf, 0);
    b(0xc0, 0);
    b(0xc1, 0);
    b(0x42, 1);
    core::ptr::write_unaligned((this + 0x1c) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x20) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x3c) as *mut i16, turnos);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + 0x28) as *mut u32, p);
    }
    if let Some(w) = g.bits {
        core::ptr::write_unaligned((this + 0xd9) as *mut u16, w);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// Checks after the original that the fields set_cup_league_rules writes have the planned values.
///
/// [pt] O plano: os campos que `set_cup_league_rules` grava, lidos depois da original.
pub(crate) unsafe fn check_cup_league_rules(nome: &str, this: usize, g: &GroupStageRules, r: u32) {
    let b = |o: usize| *((this + o) as *const u8);
    let lido = (b(0xc2), b(0xc3), b(0xc4), [b(0xc5), b(0xc6), b(0xc7), b(0xc8)], b(0xbe), [b(0xbf), b(0xc0), b(0xc1)], b(0x42),
        core::ptr::read_unaligned((this + 0x1c) as *const i32), core::ptr::read_unaligned((this + 0x20) as *const i32),
        core::ptr::read_unaligned((this + 0x3c) as *const i16));
    let previsto = (g.vitoria, g.empate, g.categoria, g.desempate, g.vagas, [0u8; 3], 1u8, -1i32, -1i32, 1i16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + 0x28) as *const u32) == p);
    let bits = g.bits.map_or(true, |w| core::ptr::read_unaligned((this + 0xd9) as *const u16) == w);
    PLACAR.registrar(r == 1 && lido == previsto && premio && bits, || format!("{}({:#x}) devolveu {} regras {:?} previstas {:?} prêmio ok {} bits ok {}", nome, this, r, lido, previsto, premio, bits));
}

/// rule: the Italian Cup groups are single round robin, 3 points a win and 1 a draw; tiebreak by codes 4, 1 and 2
/// [inference: head-to-head, goal difference, goals for]; one qualifier per group (the winner); category 4 (national
/// cup) on each match; no promotion or relegation; prize fund of 800,000
/// Original: `ItaCup::set_up_rules` (0x63f350, ita_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x63f350 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x8c): `ItaCup::set_up_rules()`: os grupos da Copa da Itália
/// em turno único, 3 pontos a vitória e 1 o empate, a categoria 4 (copa nacional), o desempate (4, 1, 2), um
/// classificado por grupo e o fundo de prêmios de 800.000; depois a tabela das rodadas.
pub(crate) unsafe extern "thiscall" fn ita_cup_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa da italia");
    let mut g = ITALIAN_CUP_GROUP_RULES;
    g.vitoria = 3; // cqi12: a vitória nos grupos da Copa da Itália
    g.categoria = 4; // cqi12: a categoria dos jogos da Copa da Itália
    set_cup_league_rules(this, &g, 1)
}

// simplification: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.

/// rule: the Serie C Italian Cup groups follow the Italian Cup rules (single round robin, 3/1, tiebreak 4, 1, 2, one
/// qualifier per group); prize fund of 300,000
/// Original: `ItaCCup::set_up_rules` (0x63d6e0, ita_c_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x63d6e0 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x8c): `ItaCCup::set_up_rules()`: os grupos da Copa da
/// Itália da Série C, com as mesmas regras da Copa da Itália e o fundo de prêmios de 300.000.
pub(crate) unsafe extern "thiscall" fn ita_cup_set_up_rules_63d6e0(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa da serie c");
    let turnos = 1i16; // cqi12: o turno único dos grupos da Série C
    set_cup_league_rules(this, &SERIE_C_CUP_RULES, turnos)
}

// simplification: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.


// simplification: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.

