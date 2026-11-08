// ---------- %%LEVA%%a leva: os mata-matas da Copa Africana de Nações e da Recopa asiática: as quartas, a fase seguinte e as finais (african_nations.cpp, asia_cup_winner.cpp) ----------

unsafe fn cqt13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O grupo de onde sai cada vaga das quartas da Copa Africana (−1 = o grupo A, a tabela da própria copa; 0..2 = as fases
/// dos grupos B a D) e a posição: as chaves são os pares da lista.
const CQT13_QUARTAS: [(i32, i32); 8] = [
    (-1, 0), (0, 1), // cqt13: 1º A × 2º B
    (1, 0), (2, 1), // cqt13: 1º C × 2º D
    (1, 1), (2, 0), // cqt13: 2º C × 1º D
    (0, 0), (-1, 1), // cqt13: 1º B × 2º A
];

/// O clube da posição `p` do grupo `g` (pela 0x6826d0 League::get_table_row: push p; mov ecx, grupo -> thiscall, ret 4).
unsafe fn group_table_club(this: usize, g: i32, p: i32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let grupo = if g < 0 { this } else { cqt13_d(cqt13_d(this + 0xc) + g as usize * 4) };
    cqt13_d(linha(grupo, p))
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
unsafe extern "thiscall" fn asia_cup_winner_create_knockout_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    let lista = alocar(0x20);
    if lista == 0 {
        return show_error_at(0x9870f8, 0x311);
    }
    crate::comparacao::ramo("cqt_13 quartas da copa africana");
    for (k, &(g, p)) in CQT13_QUARTAS.iter().enumerate() {
        cqt13_pd(lista + k * 4, group_table_club(this, g, p));
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt13_d(cqt13_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 3, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 8, lista, rodadas as u16 as u32, cqt13_d(this + 4), tabela, cqt13_w(this + 0x40) as u16 as u32, 3, 4, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt13_pd(cqt13_d(this + 0xc) + 0xc, fase);
    soltar(lista);
    soltar(tabela);
    let f = cqt13_d(cqt13_d(this + 0xc) + 0xc);
    if f == 0 {
        return show_error_at(0x9870f8, 0x32c);
    }
    montar(f, 0);
    cqt13_pd(this + 0x30, 3);
}

// ponytail: aloca a fase e monta a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][3] com 8 times,
// o +4 da copa e os 8 cruzados (lidos antes pelas tabelas, +0xb1 de cada grupo) na lista dela (+0xa7, 6 bytes; como
// conjunto) e +0x30 = 3.
unsafe extern "thiscall" fn comparar_asia_cup_winner_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let fases = cqt13_d(this + 0xc);
    if fases == 0 || (0..3).any(|g| cqt13_d(fases + g * 4) == 0) || cqt13_d(this + 0xb1) == 0 {
        return o(this);
    }
    let tabela = |g: i32| if g < 0 { cqt13_d(this + 0xb1) } else { cqt13_d(cqt13_d(fases + g as usize * 4) + 0xb1) };
    let mut plano: Vec<usize> = CQT13_QUARTAS.iter().map(|&(g, p)| cqt13_d(tabela(g) + p as usize * 0x3b)).collect();
    o(this);
    let fase = cqt13_d(cqt13_d(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("asia_cup_winner_create_knockout_stage({:#x}) sem fase", this));
        return;
    }
    let lista = cqt13_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| cqt13_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt13_w(fase + 0x3e) == 8 && cqt13_d(fase + 4) == cqt13_d(this + 4) && cqt13_d(this + 0x30) == 3;
    PLACAR.registrar(igual, || format!("asia_cup_winner_create_knockout_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// { 0x40dfa0 } (thiscall, ret 0, asia_cup_winner.cpp; a virtual +0x28): `AsiaCupWinner::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x328; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), as finais (0x40e030: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn asia_cup_winner_advance_stage(this: usize) {
    let f = cqt13_d(this + 0x30) as i32;
    if f >= (cqt13_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x987b60, 0x328);
    }
    let f = f + 1;
    cqt13_pd(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_13 finais da recopa asiatica");
        let finais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x40e030usize);
        finais(this);
    }
}

// ponytail: as finais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um (fora da última
// fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn comparar_asia_cup_winner_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let f = cqt13_d(this + 0x30) as i32;
    let ultima = f >= (cqt13_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt13_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("asia_cup_winner_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 4 finalistas da Recopa asiática: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da
/// copa, +0xa7) e depois os da segunda (a tabela +0xa7 da fase [+0xc][0]), até 4 ao todo, as duas buscas até o +0x3e da
/// copa (palavra com sinal); o 2º e o 3º trocam de lugar (os cruzamentos: as chaves são os pares da lista). As vagas
/// que sobrarem ficam 0 (a original as deixa com o que o malloc deu).
unsafe fn asian_cup_winners_finalists(this: usize) -> [usize; 4] {
    let mut v = [0usize; 4];
    let mut n = 0i16;
    let times = cqt13_w(this + 0x3e);
    for tabela in [cqt13_d(this + 0xa7), cqt13_d(cqt13_d(cqt13_d(this + 0xc)) + 0xa7)] {
        let mut i = 0i16;
        while i < times && n < 4 {
            let e = tabela + i as usize * 6;
            if *((e + 5) as *const u8) == 1 {
                v[n as usize] = cqt13_d(e);
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
/// memória, a caixa da linha 0x34b e volta) de asian_cup_winners_finalists; a tabela das rodadas pela virtual +0x3c (push 0; push
/// &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 1, 2, o
/// id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos, que o
/// construtor não lê) em [+0xc][1]; a tabela liberada (0x9452ca). Sem fase: a caixa da linha 0x375 e volta (a lista não
/// é liberada, como na original). Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round: push 0; mov ecx, fase ->
/// thiscall, ret 4) e a lista liberada.
unsafe extern "thiscall" fn create_finals_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    let lista = alocar(0x10);
    if lista == 0 {
        return show_error_at(0x987b60, 0x34b);
    }
    crate::comparacao::ramo("cqt_13 finalistas da recopa asiatica");
    // ponytail: com menos de 4 vivos a original deixa nas vagas o que o malloc deu; a cópia grava 0 (nesta carreira são 4)
    for (k, &c) in asian_cup_winners_finalists(this).iter().enumerate() {
        cqt13_pd(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt13_d(cqt13_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqt13_d(this + 4), tabela, cqt13_w(this + 0x40) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt13_pd(cqt13_d(this + 0xc) + 4, fase);
    soltar(tabela);
    let f = cqt13_d(cqt13_d(this + 0xc) + 4);
    if f == 0 {
        return show_error_at(0x987b60, 0x375);
    }
    sortear(f, 0);
    soltar(lista);
}

// ponytail: aloca a fase e sorteia a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com 4
// times, o +4 da copa e os 4 finalistas (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto).
unsafe extern "thiscall" fn comparar_create_finals_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    if cqt13_d(this + 0xa7) == 0 || cqt13_d(this + 0xc) == 0 || cqt13_d(cqt13_d(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = asian_cup_winners_finalists(this).to_vec();
    o(this);
    let fase = cqt13_d(cqt13_d(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("create_finals_stage({:#x}) sem fase", this));
        return;
    }
    let lista = cqt13_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqt13_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt13_w(fase + 0x3e) == 4 && cqt13_d(fase + 4) == cqt13_d(this + 4);
    PLACAR.registrar(igual, || format!("create_finals_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}
