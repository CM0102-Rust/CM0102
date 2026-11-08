// ---------- %%LEVA%%a leva: a Copa da Itália e a Copa da Itália da Série C: a tabela do grupo 1 (o próprio objeto da copa) e a fase seguinte (o mata-mata depois do último grupo) (ita_cup.cpp, ita_c_cup.cpp) ----------

unsafe fn cqr7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A tabela do grupo 1 de uma copa italiana (`create_first_group_table`; a copa é ela mesma a liga do grupo 1): +0x3e =
/// `n` times; a tabela (0x944e46 de n × 0x3b, cdecl) em +0xb1; sem memória, a caixa da linha `linha` e 0. Cada um dos n
/// primeiros inscritos (+0xee, vetor de ponteiros) vai à linha i da tabela (League::init_table_row 0x687430: push 0; push
/// 0; push clube; push i (a original empurra o dword do byte local, com lixo da pilha em cima; a 0x687430 só lê o byte);
/// mov ecx, copa -> thiscall, ret 0x10) e a vaga dele em +0xee é zerada (sai da lista dos que esperam a próxima fase); o
/// índice é um byte com sinal comparado à palavra +0x3e. Depois a notícia do sorteio do grupo (League::send_group_draw_news
/// 0x684230, mov ecx -> thiscall, ret 0). Devolve 1.
unsafe fn create_first_group_table(this: usize, n: i16, arquivo: usize, linha: u32, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(n as i32 as u32 * 0x3b);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(arquivo, linha);
        return 0;
    }
    crate::comparacao::ramo(ramo);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut i: i8 = 0;
    while (i as i16) < cqr7_w(this + 0x3e) {
        let vaga = cqr7_d(this + 0xee) + i as isize as usize * 4;
        iniciar(this, i as u8 as u32, cqr7_d(vaga), 0, 0);
        core::ptr::write_unaligned((cqr7_d(this + 0xee) + i as isize as usize * 4) as *mut u32, 0); // cqr7: a vaga do grupo 1 sai da lista
        i = i.wrapping_add(1);
    }
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    noticia(this);
    1
}

/// Só a original (ponytail: a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos; não
/// pode rodar duas vezes), conferindo o plano feito antes: devolve 1, +0x3e = n, a tabela em +0xb1 com os n primeiros
/// inscritos nas linhas (de 0x3b bytes), na ordem, e as n primeiras vagas de +0xee zeradas.
unsafe fn compare_first_group_table(nome: &str, this: usize, n: i16, original: impl FnOnce() -> u32) -> u32 {
    let plano: Vec<usize> = (0..n as usize).map(|k| cqr7_d(cqr7_d(this + 0xee) + k * 4)).collect();
    let r = original();
    let t = cqr7_d(this + 0xb1);
    let ok = r == 1 && cqr7_w(this + 0x3e) == n && t != 0
        && plano.iter().enumerate().all(|(k, &c)| cqr7_d(t + k * 0x3b) == c && cqr7_d(cqr7_d(this + 0xee) + k * 4) == 0);
    PLACAR.registrar(ok, || format!("{}({:#x}) devolveu {}; plano {:x?}", nome, this, r, plano));
    r
}

/// { 0x63f250 } (thiscall, ret 0, ita_cup.cpp; do construtor): `ItaCup::create_first_group_table()`: o grupo 1 da Copa
/// da Itália com 4 times (`create_first_group_table`; a caixa do ita_cup.cpp na linha 0x19e).
unsafe extern "thiscall" fn ita_cup_create_first_group_table(this: usize) -> u32 {
    create_first_group_table(this, 4, 0x9ea23c, 0x19e, "cqr_7 grupo 1 da copa da italia")
}

unsafe extern "thiscall" fn comparar_ita_cup_create_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_first_group_table("ita_cup_create_first_group_table", this, 4, || o(this))
}

/// { 0x63d5e0 } (thiscall, ret 0, ita_c_cup.cpp; do construtor): `ItaCCup::init_first_group()`: o grupo 1 da Copa da
/// Itália da Série C com 5 times (`create_first_group_table`; a caixa do ita_c_cup.cpp na linha 0x1a0).
unsafe extern "thiscall" fn init_first_group(this: usize) -> u32 {
    create_first_group_table(this, 5, 0x9ea204, 0x1a0, "cqr_7 grupo 1 da copa da serie c") // cqr7: 5 times no grupo 1 da Série C
}

unsafe extern "thiscall" fn comparar_init_first_group(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_first_group_table("init_first_group", this, 5, || o(this))
}

/// A fase seguinte de uma copa italiana (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal), a caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a `ultimo_grupo` + 1 (o fim dos grupos), monta o
/// mata-mata (`mata_mata`, call direto com o ecx da copa -> thiscall, ret 0).
unsafe fn advance_cup_stage(this: usize, arquivo: usize, linha: u32, fim_dos_grupos: i32, mata_mata: usize) {
    let fase = cqr7_d(this + 0x30) as i32;
    if fase >= (cqr7_d(this + 0x2c) as i32).wrapping_sub(1) {
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

/// O comparador da fase seguinte: só a original (ponytail: o mata-mata aloca a fase e sorteia; não pode rodar duas vezes),
/// conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
unsafe fn ita_cup_compare_advance_stage(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqr7_d(this + 0x30) as i32;
    let ultima = antes >= (cqr7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqr7_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) fase {} -> {}", nome, this, antes, depois));
}

/// { 0x63f5b0 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x28): `ItaCup::advance_stage()`: a fase seguinte
/// (`advance_cup_stage`; a caixa do ita_cup.cpp na linha 0x23e); depois da fase 6 (os 8 grupos: o grupo 1 e as fases
/// 0..6), o mata-mata da Copa da Itália (ItaCup::create_knockout_stage 0x63f870).
unsafe extern "thiscall" fn ita_cup_advance_stage(this: usize) {
    advance_cup_stage(this, 0x9ea23c, 0x23e, 7, 0x63f870) // cqr7: o mata-mata da Copa da Itália na fase 7
}

unsafe extern "thiscall" fn comparar_ita_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    ita_cup_compare_advance_stage("ita_cup_advance_stage", this, || o(this))
}

/// { 0x63d950 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x28): `ItaCCup::advance_stage()`: a fase seguinte
/// (`advance_cup_stage`; a caixa do ita_c_cup.cpp na linha 0x240); depois da fase 14 (os 16 grupos), o mata-mata da Copa
/// da Série C (ItaCCup::create_knockout_stage 0x63dc10).
unsafe extern "thiscall" fn ita_cup_advance_stage_63d950(this: usize) {
    advance_cup_stage(this, 0x9ea204, 0x240, 0xf, 0x63dc10)
}

unsafe extern "thiscall" fn comparar_ita_cup_advance_stage_63d950(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    ita_cup_compare_advance_stage("ita_cup_advance_stage_63d950", this, || o(this))
}
