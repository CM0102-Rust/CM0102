// ---------- %%LEVA%%a leva: os play-offs das Séries A e B: a fase seguinte e o mata-mata dos 4 primeiros (bra_nat_first.cpp, bra_nat_second.cpp) ----------

unsafe fn cqq7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqq7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqq7_clube(liga: usize, l: usize) -> usize {
    cqq7_d(cqq7_d(liga + 0xb1) + l * 0x3b)
}

/// O play-off de uma Série (o mesmo molde na A e na B, conferido instrução a instrução: só mudam o fonte e as linhas
/// das caixas): a lista dos 4 (malloc 0x944e46 de 0x10; sem memória: a caixa `linha_sem_lista` e volta) com os clubes
/// das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3); a virtual +0x3c (create_round_table: thiscall(this,
/// fase 0, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new 0x944cf1 de 0xb2;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 0, 1,
/// o extra, 0x14, 0, 0, 0 e nenhuma semente: a ordem não é por sementes) vai para a primeira vaga das fases ([+0xc]) e a
/// tabela de rodadas é liberada (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`, a lista liberada e volta. Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a lista liberada e +0x30 = 0. O quadro SEH da original só
/// cobre o construtor (sem destrutor).
unsafe fn cqq7_play_off(this: usize, linhas: &[usize; 4], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqq7_clube(this, l));
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqq7_d(cqq7_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqq7_d(this + 4), t, cqq7_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned(cqq7_d(this + 0xc) as *mut usize, fase);
    liberar(t);
    let fase = cqq7_d(cqq7_d(this + 0xc));
    if fase == 0 {
        aviso_em(arquivo, linha_sem_fase);
        liberar(lista);
        return;
    }
    crate::comparacao::ramo(ramo);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// A fase seguinte de uma Série (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o play-off
/// (`play_off`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqq7_fase_seguinte(this: usize, arquivo: usize, linha: u32, play_off: usize) {
    let fase = cqq7_d(this + 0x30) as i32;
    if fase >= (cqq7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this);
    }
}

/// { 0x42a950 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::advance_stage()`, a fase seguinte da Série A: no fim
/// dos pontos corridos, o play-off (0x42a9e0); a caixa da linha 0x264.
unsafe extern "thiscall" fn bra_nat_first_advance_stage(this: usize) {
    cqq7_fase_seguinte(this, 0x98aa30, 0x264, 0x42a9e0);
}

/// { 0x42a9e0 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_play_off_stage()`, o play-off da Série A:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x286 (sem lista) e 0x29d (sem fase).
unsafe extern "thiscall" fn bra_nat_first_create_play_off_stage(this: usize) {
    cqq7_play_off(this, &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A
}

/// { 0x42cdd0 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::advance_stage()`, a fase seguinte da Série B: no fim
/// dos pontos corridos, o play-off (0x42ce60); a caixa da linha 0x23d.
unsafe extern "thiscall" fn bra_nat_first_advance_stage_42cdd0(this: usize) {
    cqq7_fase_seguinte(this, 0x98ab64, 0x23d, 0x42ce60);
}

/// { 0x42ce60 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::create_play_off_stage()`, o play-off da Série B:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x25f (sem lista) e 0x276 (sem fase).
unsafe extern "thiscall" fn bra_nat_first_create_play_off_stage_42ce60(this: usize) {
    cqq7_play_off(this, &[0, 3, 1, 2], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // cqq7: 1º × 4º, 2º × 3º Série B
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o play-off, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o play-off grava 0, o mesmo valor).
unsafe fn cqq7_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqq7_d(this + 0x30) as i32;
    let ultima = antes >= (cqq7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqq7_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = 4, +4 = o +4 da liga e os 4
/// primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada pode
/// reordenar), e +0x30 = 0.
unsafe fn cqq7_comparar_play_off(nome: &str, this: usize, linhas: &[usize; 4], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqq7_clube(this, l)).collect();
    original();
    let fase = cqq7_d(cqq7_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqq7_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqq7_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqq7_w(fase + 0x3e) == 4 && cqq7_d(fase + 4) == cqq7_d(this + 4) && cqq7_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqq7_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_bra_nat_first_advance_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqq7_comparar_fase_seguinte("bra_nat_first_advance_stage", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_nat_first_create_play_off_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqq7_comparar_play_off("bra_nat_first_create_play_off_stage", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_bra_nat_first_advance_stage_42cdd0(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqq7_comparar_fase_seguinte("bra_nat_first_advance_stage_42cdd0", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_nat_first_create_play_off_stage_42ce60(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    cqq7_comparar_play_off("bra_nat_first_create_play_off_stage_42ce60", this, &[0, 3, 1, 2], || o(this))
}
