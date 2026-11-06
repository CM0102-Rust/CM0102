// ---------- %%LEVA%%a leva: os mata-matas da Libertadores e da Merconorte: os classificados dos grupos e a fase seguinte (conmebol_liber.cpp, con_merc_cup.cpp) ----------

unsafe fn cqs10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os classificados por grupo (+0xbe, byte com sinal).
unsafe fn cqs10_vagas(this: usize) -> i32 {
    *((this + 0xbe) as *const i8) as i32
}

/// O clube da linha i da tabela do grupo 1 (a da própria copa: +0xb1, linhas de 0x3b bytes).
unsafe fn cqs10_do_grupo_um(this: usize, i: usize) -> usize {
    cqs10_d(cqs10_d(this + 0xb1) + i * 0x3b) // cqs10: a linha do grupo 1
}

/// O clube da posição j da fase f ([+0xc][f], um League_Stage): a linha pela 0x6826d0 (League::get_table_row: push j;
/// mov ecx, fase -> thiscall, ret 4) e o clube no +0 dela.
unsafe fn cqs10_da_fase(this: usize, f: usize, j: i32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let fase = cqs10_d(cqs10_d(this + 0xc) + f * 4);
    cqs10_d(linha(fase, j)) // cqs10: a posição j da fase
}

/// O mata-mata depois dos grupos (o mesmo molde na Libertadores e na Merconorte, conferido instrução a instrução: mudam
/// o tamanho da lista, as fases de grupo, a fase do mata-mata, o fonte e as linhas das caixas). A lista (0x944e46(4 ×
/// times): push; add esp, 4 -> cdecl; sem memória, a caixa `linha_sem_lista` e volta) com os +0xbe primeiros do grupo 1
/// (na ordem das linhas) e, de cada fase de grupo 0..`fases` − 1, os +0xbe primeiros (posições 0, 1...); a tabela das
/// rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push `mata`; mov ecx, this -> thiscall, ret 0x10); a
/// fase (0x944cf1(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, `times`,
/// a lista, as rodadas, +4, a tabela, o ano +0x40, `mata`, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra
/// as rodadas, o id e o ano com lixo nos 16 bits altos, que o construtor não lê) em [+0xc][`mata`]; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa `linha_sem_fase`. Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round:
/// push 0; mov ecx, fase -> thiscall, ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqs10_mata_mata(this: usize, times: u32, fases: usize, mata: u32, arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    let lista = alocar(times * 4);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs10_vagas(this) {
        cqs10_pd(lista + n * 4, cqs10_do_grupo_um(this, i as usize));
        n += 1;
        i += 1;
    }
    for f in 0..fases {
        let mut j = 0i32;
        while j < cqs10_vagas(this) {
            crate::comparacao::ramo(ramo);
            cqs10_pd(lista + n * 4, cqs10_da_fase(this, f, j)); // cqs10: os classificados de cada fase
            n += 1;
            j += 1;
        }
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqs10_d(cqs10_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, mata, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, times, lista, rodadas as u16 as u32, cqs10_d(this + 4), tabela, cqs10_w(this + 0x40) as u16 as u32, mata, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqs10_pd(cqs10_d(this + 0xc) + mata as usize * 4, fase);
    soltar(lista);
    soltar(tabela);
    let f = cqs10_d(cqs10_d(this + 0xc) + mata as usize * 4);
    if f == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    sortear(f, 0);
}

/// O plano do mata-mata (o comparador roda só a original: a lista, a tabela e a fase são alocadas e a rodada é
/// sorteada): os classificados lidos antes (o grupo 1 nas linhas, as fases pelas linhas da tabela +0xb1 delas, como a
/// 0x6826d0); depois a fase em [+0xc][`mata`] com +0x3e = `times`, o +4 da copa e os clubes da lista dela (+0xa7, 6
/// bytes) iguais aos do plano como conjunto (o sorteio pode reordenar).
unsafe fn cqs10_comparar_mata_mata(nome: &str, this: usize, times: usize, fases: usize, mata: usize, original: impl FnOnce()) {
    let q = cqs10_vagas(this).max(0) as usize;
    let mut plano: Vec<usize> = (0..q).map(|i| cqs10_d(cqs10_d(this + 0xb1) + i * 0x3b)).collect();
    for f in 0..fases {
        let fase = cqs10_d(cqs10_d(this + 0xc) + f * 4);
        for j in 0..q {
            plano.push(cqs10_d(cqs10_d(fase + 0xb1) + j * 0x3b));
        }
    }
    original();
    let fase = cqs10_d(cqs10_d(this + 0xc) + mata * 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqs10_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..times).map(|k| cqs10_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqs10_w(fase + 0x3e) as usize == times && cqs10_d(fase + 4) == cqs10_d(this + 4);
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqs10_w(fase + 0x3e)));
}

/// { 0x4c3d10 } (thiscall, ret 0, conmebol_liber.cpp): `ConmebolLiber::create_knockout_stage()`, o mata-mata da
/// Libertadores: os 16 (os 2 primeiros do grupo 1 e das 7 fases de grupo) na fase 7; caixas nas linhas 0x24c (sem
/// lista) e 0x26a (sem fase).
unsafe extern "thiscall" fn cqs10_libertadores_mata_mata(this: usize) {
    cqs10_mata_mata(this, 0x10, 7, 7, 0x9a7de4, 0x24c, 0x26a, "cqs_10 classificados da libertadores");
}

unsafe extern "thiscall" fn comparar_cqs10_libertadores_mata_mata(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqs10_comparar_mata_mata("cqs10_libertadores_mata_mata", this, 0x10, 7, 7, || o(this))
}

/// { 0x4c2330 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::create_semi_final_stage()`, as semifinais da
/// Merconorte: os 4 (o primeiro do grupo 1 e das 3 fases de grupo) na fase 3; caixas nas linhas 0x21d (sem lista) e
/// 0x23b (sem fase).
unsafe extern "thiscall" fn cqs10_merconorte_semifinais(this: usize) {
    cqs10_mata_mata(this, 4, 3, 3, 0x9a7d6c, 0x21d, 0x23b, "cqs_10 classificados da merconorte");
}

unsafe extern "thiscall" fn comparar_cqs10_merconorte_semifinais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqs10_comparar_mata_mata("cqs10_merconorte_semifinais", this, 4, 3, 3, || o(this))
}

/// { 0x4c20b0 } (thiscall, ret 0, con_merc_cup.cpp; a virtual +0x28): `ConMercCup::advance_stage()`, a fase seguinte da
/// Merconorte: na última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x1c7; senão +0x30 sobe e, chegando a 3
/// (depois dos grupos), as semifinais (0x4c2330: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqs10_merconorte_fase_seguinte(this: usize) {
    let f = cqs10_d(this + 0x30) as i32;
    if f >= (cqs10_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x9a7d6c, 0x1c7);
    }
    let f = f + 1;
    cqs10_pd(this + 0x30, f as usize);
    if f == 3 { // cqs10: as semifinais da Merconorte depois dos grupos
        crate::comparacao::ramo("cqs_10 semifinais da merconorte");
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c2330usize);
        semifinais(this);
    }
}

// ponytail: as semifinais (0x4c2330) alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn comparar_cqs10_merconorte_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let f = cqs10_d(this + 0x30) as i32;
    let ultima = f >= (cqs10_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqs10_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqs10_merconorte_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}
