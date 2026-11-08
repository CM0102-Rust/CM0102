// ---------- %%LEVA%%a leva: as semifinais dos estaduais e regionais: Pernambucano, Catarinense e Paulista (bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqn8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqn8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqn8_clube(liga: usize, l: usize) -> usize {
    cqn8_d(cqn8_d(liga + 0xb1) + l * 0x3b)
}

/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqn8_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn8_clube(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqn8_d(cqn8_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, cqn8_d(this + 4), t, cqn8_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(cqn8_d(this + 0xc) as *mut usize, fase);
    let fase = cqn8_d(cqn8_d(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// { 0x439710 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98afd8, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // cqn8: 1º × 4º, 2º × 3º Pernambucano
}

/// { 0x43bf50 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b050, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage_43bf50(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // cqn8: 1º × 4º, 2º × 3º Catarinense
}

/// { 0x43cf80 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b090, linhas 0x20d (sem lista) e 0x221 (sem fase).
unsafe extern "thiscall" fn bra_reg_pern_create_semi_final_stage_43cf80(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // cqn8: 1º × 4º, 2º × 3º Paulista
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
unsafe fn cqn8_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn8_clube(this, l)).collect();
    original();
    let fase = cqn8_d(cqn8_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqn8_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| cqn8_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqn8_w(fase + 0x3e) as usize == linhas.len() && cqn8_d(fase + 4) == cqn8_d(this + 4) && cqn8_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqn8_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_bra_reg_pern_create_semi_final_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqn8_comparar("bra_reg_pern_create_semi_final_stage", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_pern_create_semi_final_stage_43bf50(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqn8_comparar("bra_reg_pern_create_semi_final_stage_43bf50", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_pern_create_semi_final_stage_43cf80(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqn8_comparar("bra_reg_pern_create_semi_final_stage_43cf80", this, &[0, 3, 1, 2], || o(this))
}
