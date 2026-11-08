// ---------- %%LEVA%%a leva: os construtores das copas entre continentes: a Copa dos Campeões da CONCACAF, a Copa Interamericana e a Copa Intercontinental (con_champ.cpp, inter_amer_cup.cpp, world_club_cup.cpp) ----------

unsafe fn cqw12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqw12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqw12_b(this: usize, d: usize, v: u8) {
    *((this + d) as *mut u8) = v;
}

/// Os bytes e campos de uma copa de mata-mata entre continentes (as três): +0xab = 0, +0x44 = −1, +0x2c = 0, +0x30 = −1.
unsafe fn cqw12_padrao(this: usize) {
    cqw12_b(this, 0xab, 0);
    cqw12_b(this, 0x44, 0xff);
    cqw12_pd(this + 0x2c, 0);
    cqw12_pd(this + 0x30, usize::MAX);
}

/// O fim comum: a tabela das rodadas da copa (a virtual +0x3c(−1, &+0x3c, &+0x3a, 0): push 0; push; push; push −1; mov
/// ecx -> thiscall, ret 0x10) em +0xa3; a árvore dos jogos (0x518790, thiscall, ret 0; negativa: a caixa da linha 0x4f
/// e volta); as estatísticas (0x944cf1(0x5ce) e o construtor Comp_Stats 0x49ee70(novo, this): push this; mov ecx ->
/// thiscall, ret 4) em +8 (nulas: a caixa da linha 0x56). Devolve se chegou ao fim com as estatísticas.
unsafe fn cqw12_fim(this: usize, arquivo: usize) -> bool {
    let arvore: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let tabela: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(cqw12_d(cqw12_d(this) + 0x3c));
    cqw12_pd(this + 0xa3, tabela(this, -1, this + 0x3c, this + 0x3a, 0));
    if arvore(this) < 0 {
        show_error_at(arquivo, 0x4f);
        return false;
    }
    let p = novo(0x5ce);
    let e = if p != 0 { estatisticas(p, this) } else { 0 };
    cqw12_pd(this + 8, e);
    if e == 0 {
        show_error_at(arquivo, 0x56);
        return false;
    }
    true
}

/// O começo comum: o construtor da Cup (0x518640: mov ecx -> thiscall, ret 0); o que estiver salvo (Cup::load
/// 0x51fc00(this, 1): push 1; mov ecx -> thiscall, ret 4) depois dos campos.
unsafe fn cqw12_copa(this: usize) {
    let copa: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    copa(this);
}

unsafe fn cqw12_carregar(this: usize) -> bool {
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    carregar(this, 1) != 0
}

/// { 0x4c0c40 } (thiscall, ret 8, con_champ.cpp; SEH só para exceções): `ConChamp::ConChamp(ano, comp)`: a Copa dos
/// Campeões da CONCACAF (vtable 0x968aa0) do ano do jogo: +4 o registro, o ano +0x40, +0x42 = 2, +0x50 = −1, +0xb1 = 0;
/// sem nada salvo, 8 times (+0x3e) numa tabela zerada de 6 bytes cada (calloc 0x945501(8, 6): push 6; push 8; add esp,
/// 8 -> cdecl; sem memória, a caixa da linha 0x41), o prêmio +0x28 = 2.000.000 e o fim comum.
unsafe extern "thiscall" fn cqw12_concacaf(this: usize, ano: u32, comp: usize) -> usize {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    cqw12_copa(this);
    crate::comparison::branch("cqw_12 copa da concacaf");
    cqw12_pd(this + 4, comp);
    cqw12_pd(this, 0x968aa0);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, ano as i16); // cqw12: a CONCACAF do ano do jogo
    cqw12_padrao(this);
    cqw12_b(this, 0x42, 2);
    cqw12_b(this, 0x50, 0xff);
    cqw12_b(this, 0xb1, 0);
    if cqw12_carregar(this) {
        return this;
    }
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 8);
    let t = calloc(8, 6);
    cqw12_pd(this + 0xa7, t);
    if t == 0 {
        show_error_at(0x9a7d30, 0x41);
        return this;
    }
    cqw12_pd(this + 0x28, 0x1e8480);
    cqw12_fim(this, 0x9a7d30);
    this
}

// ponytail: o construtor aloca a tabela, a árvore e as estatísticas: não roda duas vezes. Só a original; o plano: devolve
// o this, a vtable, o registro +4 e o ano +0x40.
unsafe fn cqw12_comparar(nome: &str, this: usize, ano: u32, comp: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let lido = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let ok = r == this && cqw12_d(this) == vtable && cqw12_d(this + 4) == comp && lido == ano as i16;
    PLACAR.record(ok, || format!("{}({:#x}): ano {} pedido {}, vtable {:#x}", nome, this, lido, ano as i16, cqw12_d(this)));
    r
}

unsafe extern "thiscall" fn comparar_cqw12_concacaf(this: usize, ano: u32, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    cqw12_comparar("cqw12_concacaf", this, ano, comp, 0x968aa0, || o(this, ano, comp))
}

/// O corpo das duas copas de dois clubes (0x632080 e 0x92b4b0): o ano +0x40, +4, a vtable, os campos padrão, +0x42 =
/// 2, +0x52 = `b52`, +0x49 = 7, +0x4a = `b4a`, +0x50 = −1, +0xb1 = 0; sem nada salvo, os dois clubes (`clubes`,
/// thiscall, ret 0; negativo: a caixa da linha 0x41 e volta), o fim comum e as posições previstas (0x5223a0, thiscall,
/// ret 0).
unsafe fn cqw12_dois_clubes(this: usize, ano: u32, comp: usize, vtable: usize, b52: u8, b4a: u8, clubes: usize, arquivo: usize) -> usize {
    let escolher: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(clubes);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5223a0usize);
    cqw12_copa(this);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, ano as i16);
    cqw12_pd(this + 4, comp);
    cqw12_pd(this, vtable);
    cqw12_padrao(this);
    cqw12_b(this, 0x42, 2);
    cqw12_b(this, 0x52, b52);
    cqw12_b(this, 0x49, 7);
    cqw12_b(this, 0x4a, b4a);
    cqw12_b(this, 0x50, 0xff);
    cqw12_b(this, 0xb1, 0);
    if cqw12_carregar(this) {
        return this;
    }
    if escolher(this) < 0 {
        show_error_at(arquivo, 0x41);
        return this;
    }
    if cqw12_fim(this, arquivo) {
        previstas(this);
    }
    this
}

/// { 0x92b4b0 } (thiscall, ret 8, world_club_cup.cpp; SEH só para exceções): `WorldClubCup::WorldClubCup(ano, comp)`: a
/// Copa Intercontinental (vtable 0x971250, +0x52 = 3, +0x4a = 3) do ano do jogo, com os dois clubes de 0x92b8b0.
unsafe extern "thiscall" fn cqw12_intercontinental(this: usize, ano: u32, comp: usize) -> usize {
    crate::comparison::branch("cqw_12 copa intercontinental");
    let ano = ano; // cqw12: a Intercontinental do ano do jogo
    cqw12_dois_clubes(this, ano, comp, 0x971250, 3, 3, 0x92b8b0, 0xad5848)
}

unsafe extern "thiscall" fn comparar_cqw12_intercontinental(this: usize, ano: u32, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    cqw12_comparar("cqw12_intercontinental", this, ano, comp, 0x971250, || o(this, ano, comp))
}
