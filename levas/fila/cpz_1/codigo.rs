// ---------- %%LEVA%%a leva: os play-offs das divisões inglesas: o mata-mata da First, da Second e da Third (eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------

unsafe fn cpz1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube da linha `k` da tabela da liga (+0xb1, linhas de 0x3b bytes; o clube no +0).
unsafe fn eng_first_league_row_club(liga: usize, k: usize) -> usize {
    cpz1_d(cpz1_d(liga + 0xb1) + k * 0x3b)
}

/// O mata-mata das três divisões (o corpo de 0x573080, 0x576a70 e 0x5783c0, que só mudam nas linhas, no fonte e no
/// +0x3a da fase). A lista de `+0xbf` clubes (malloc 0x944e46 de 4 × o byte com sinal; sem memória: a caixa
/// `linha_sem_lista` e nada mais) leva os clubes das 4 `linhas` da tabela, nesta ordem (as chaves: 0 × 1 e 2 × 3).
/// A virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, a conta, a lista, as rodadas, +4, a tabela de rodadas, a palavra +0x40, 0, 1,
/// `a10` (ou o extra da virtual), 0x14, 0, 0, 0, 0) vai para a primeira vaga de fases ([+0xc]); a lista e a tabela de
/// rodadas são liberadas (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`; senão Cup::set_up_round (0x51c800,
/// thiscall(fase, 0), ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn create_playoff_stage(this: usize, linhas: [usize; 4], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, a10: Option<u32>, nome: &'static str) {
    let n = cpz1_b(this + 0xbf) as i8 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n << 2) as u32);
    if lista == 0 {
        return show_error_at(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, eng_first_league_row_club(this, l));
    }
    crate::comparacao::ramo(nome);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cpz1_d(cpz1_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra a conta, as rodadas e a palavra +0x40 em registradores inteiros com lixo nos 16 bits
    // altos; o construtor só lê as palavras ([esp+0x230], [esp+0x238], [esp+0x244] e [esp+0x250] com mov ax/dx).
    let fase = if p != 0 {
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(0x522e00usize);
        let a10 = a10.unwrap_or(extra as u16 as u32);
        construir(p, this, n as u16 as u32, lista, rodadas as u16 as u32, cpz1_d(this + 4), t, cpz1_w(this + 0x40) as u16 as u32, 0, 1, a10, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned(cpz1_d(this + 0xc) as *mut usize, fase);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(lista);
    liberar(t);
    let fase = cpz1_d(cpz1_d(this + 0xc));
    if fase == 0 {
        return show_error_at(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
}

/// { 0x573080 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_playoff_stage()`, da fase seguinte da First
/// (0x572ff0, quando a fase passa de −1 a 0). As chaves: o 6º contra o 3º e o 5º contra o 4º (linhas 5, 2, 4 e 3);
/// +0x3a da fase = 0xa0. Fonte 0x9c4374, linhas 0x2f0 (sem lista) e 0x306 (sem fase).
unsafe extern "thiscall" fn eng_first_create_playoff_stage(this: usize) {
    create_playoff_stage(this, [5, 2, 4, 3], 0x9c4374, 0x2f0, 0x306, Some(0xa0), "cpz_1 mata-mata da first"); // cpz1: 6º × 3º e 5º × 4º da First
}

/// { 0x576a70 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_play_off_stage()` (de 0x5769e0). As chaves da
/// First (linhas 5, 2, 4 e 3); +0x3a da fase = o extra da virtual +0x3c. Fonte 0x9c43e8, linhas 0x2b6 e 0x2cc.
unsafe extern "thiscall" fn eng_second_create_play_off_stage(this: usize) {
    create_playoff_stage(this, [5, 2, 4, 3], 0x9c43e8, 0x2b6, 0x2cc, None, "cpz_1 mata-mata da second"); // cpz1: 6º × 3º e 5º × 4º da Second
}

/// { 0x5783c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_play_off_stage()` (de 0x578330). As chaves: o 7º
/// contra o 4º e o 6º contra o 5º (linhas 6, 3, 5 e 4); +0x3a = o extra da virtual. Fonte 0x9c4424, linhas 0x2be e
/// 0x2d5.
unsafe extern "thiscall" fn eng_first_create_play_off_stage(this: usize) {
    create_playoff_stage(this, [6, 3, 5, 4], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // cpz1: 7º × 4º e 6º × 5º da Third
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das 4 linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = a conta
/// (+0xbf), +4 = o +4 da liga e os 4 primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como
/// conjunto: o sorteio da rodada 0x51c800 pode reordenar as chaves).
unsafe fn compare_playoff_stage(nome: &str, this: usize, linhas: [usize; 4], original: impl FnOnce()) {
    let mut plano: [usize; 4] = [0; 4];
    for (k, &l) in linhas.iter().enumerate() {
        plano[k] = eng_first_league_row_club(this, l);
    }
    let n = cpz1_b(this + 0xbf) as i8 as i16;
    original();
    let fase = cpz1_d(cpz1_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cpz1_d(fase + 0xa7);
    let mut achou: [usize; 4] = [0; 4];
    for (k, a) in achou.iter_mut().enumerate() {
        *a = cpz1_d(lista + 6 * k);
    }
    let (mut x, mut y) = (plano, achou);
    x.sort_unstable();
    y.sort_unstable();
    let igual = x == y && cpz1_w(fase + 0x3e) == n && cpz1_d(fase + 4) == cpz1_d(this + 4);
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={} n={}", nome, this, plano, fase, achou, cpz1_w(fase + 0x3e), n));
}

unsafe extern "thiscall" fn comparar_eng_first_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_playoff_stage("eng_first_create_playoff_stage", this, [5, 2, 4, 3], || o(this))
}

unsafe extern "thiscall" fn comparar_eng_second_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_playoff_stage("eng_second_create_play_off_stage", this, [5, 2, 4, 3], || o(this))
}

unsafe extern "thiscall" fn comparar_eng_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_playoff_stage("eng_first_create_play_off_stage", this, [6, 3, 5, 4], || o(this))
}
