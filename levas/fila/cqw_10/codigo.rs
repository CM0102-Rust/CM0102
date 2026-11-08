// ---------- %%LEVA%%a leva: os construtores dos torneios de seleções: a Copa Africana de Nações, a Copa América e a Copa das Nações da Oceania (african_nations.cpp, goldcup.cpp, oceania_nations.cpp) ----------

unsafe fn cqw10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqw10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O resto (com sinal, como o `and 0x80000001` com o acerto da original) de (ano − 1996) por 2.
fn cqw10_paridade(ano: i16) -> i32 {
    (ano as i32 - 0x7cc) % 2
}

/// O ano da edição: do `comeco`, sobe de um em um enquanto o resto de (ano − 1996) por 2 não for `resto`.
fn cqw10_edicao(comeco: i16, resto: i32) -> i16 {
    let mut ano = comeco;
    while cqw10_paridade(ano) != resto {
        ano = ano.wrapping_add(1);
    }
    ano
}

/// Um torneio de seleções montado sobre a League (o mesmo corpo nas três): o construtor da League (0x682200: mov ecx
/// -> thiscall, ret 0); +4 o registro da competição, a vtable, +0x42 = 4, +0x1c = +0x20 = −1, +0x50 = 0, o ano +0x40
/// (word) da edição, +0x51 = 0xf; o que estiver salvo (League::load 0x687b10(this, 1): push 1; mov ecx -> thiscall, ret
/// 4) e, sem nada salvo: +0xd9 = 40, +0x45 = 0, +0x44 = −1, +0x30 = −1, +0x2c = `fases` e o vetor das fases +0xc
/// (0x944e46(4 × fases): push; add esp, 4 -> cdecl, sem conferir), as regras (a rotina `regras`, thiscall, ret 0; 0: a
/// caixa `linhas.0` e volta) e as estatísticas (0x944cf1(0x5ce) e o construtor Comp_Stats 0x49ee70(novo, this): push
/// this; mov ecx, novo -> thiscall, ret 4) em +8 (nulas: a caixa `linhas.1`). Devolve o this.
unsafe fn cqw10_torneio(this: usize, ano: i16, comp: usize, vtable: usize, fases: u32, regras: usize, arquivo: usize, linhas: (u32, u32)) -> usize {
    let liga: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let montar_regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(regras);
    liga(this);
    cqw10_pd(this + 4, comp);
    cqw10_pd(this, vtable);
    *((this + 0x42) as *mut u8) = 4;
    cqw10_pd(this + 0x1c, usize::MAX);
    cqw10_pd(this + 0x20, usize::MAX);
    *((this + 0x50) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x40) as *mut i16, ano);
    *((this + 0x51) as *mut u8) = 0xf;
    if carregar(this, 1) != 0 {
        return this;
    }
    core::ptr::write_unaligned((this + 0xd9) as *mut i16, 0x28);
    cqw10_pd(this + 0x45, 0);
    *((this + 0x44) as *mut u8) = 0xff;
    cqw10_pd(this + 0x30, usize::MAX);
    cqw10_pd(this + 0x2c, fases as usize);
    cqw10_pd(this + 0xc, alocar(fases * 4));
    if montar_regras(this) == 0 {
        show_error_at(arquivo, linhas.0);
        return this;
    }
    let p = novo(0x5ce);
    let e = if p != 0 { estatisticas(p, this) } else { 0 };
    cqw10_pd(this + 8, e);
    if e == 0 {
        show_error_at(arquivo, linhas.1);
    }
    this
}

/// { 0x401000 } (thiscall, ret 8, african_nations.cpp; quadro SEH com estado só para exceções no construtor das
/// estatísticas): `AfricanNations::AfricanNations(ano, comp)`: a Copa Africana de Nações (vtable 0x967270, 4 fases,
/// regras 0x401ab0, caixas nas linhas 0x58 e 0x5f); a edição é a do ano seguinte ao do jogo, levada ao primeiro ano
/// par (de 1996 em diante, de dois em dois).
unsafe extern "thiscall" fn cqw10_africana(this: usize, ano: u32, comp: usize) -> usize {
    crate::comparison::branch("cqw_10 copa africana");
    let edicao = cqw10_edicao((ano as i16).wrapping_add(1), 0); // cqw10: a Copa Africana do ano seguinte, par
    cqw10_torneio(this, edicao, comp, 0x967270, 4, 0x401ab0, 0x9870f8, (0x58, 0x5f))
}

// ponytail: o construtor aloca as fases e as estatísticas: não roda duas vezes. Só a original; o plano: devolve o this,
// a vtable, o registro +4, o ano da edição e +0x51 = 0xf.
unsafe fn cqw10_comparar(nome: &str, this: usize, comp: usize, vtable: usize, ano: i16, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let lido = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let ok = r == this && cqw10_d(this) == vtable && cqw10_d(this + 4) == comp && lido == ano && *((this + 0x51) as *const u8) == 0xf;
    SCOREBOARD.record(ok, || format!("{}({:#x}): ano {} previsto {}, vtable {:#x}", nome, this, lido, ano, cqw10_d(this)));
    r
}

unsafe extern "thiscall" fn compare_cqw10_africana(this: usize, ano: u32, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    cqw10_comparar("cqw10_africana", this, comp, 0x967270, cqw10_edicao((ano as i16).wrapping_add(1), 0), || o(this, ano, comp))
}

/// { 0x5e0660 } (thiscall, ret 8, goldcup.cpp; SEH só para exceções): `GoldCup::GoldCup(ano, comp)`: a Copa América
/// (vtable 0x96b400, 3 fases, regras 0x5e12d0, caixas nas linhas 0x56 e 0x5d); a edição é a do próprio ano do jogo,
/// levada ao primeiro ano ímpar (1997, 1999, 2001...).
unsafe extern "thiscall" fn cqw10_copa_america(this: usize, ano: u32, comp: usize) -> usize {
    crate::comparison::branch("cqw_10 copa america");
    let edicao = cqw10_edicao(ano as i16, 1); // cqw10: a Copa América no ano ímpar
    cqw10_torneio(this, edicao, comp, 0x96b400, 3, 0x5e12d0, 0x9cd640, (0x56, 0x5d))
}

unsafe extern "thiscall" fn compare_cqw10_copa_america(this: usize, ano: u32, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    cqw10_comparar("cqw10_copa_america", this, comp, 0x96b400, cqw10_edicao(ano as i16, 1), || o(this, ano, comp))
}

/// { 0x7988b0 } (thiscall, ret 8, oceania_nations.cpp; SEH só para exceções): `OceaniaNations::OceaniaNations(ano,
/// comp)`: a Copa das Nações da Oceania (vtable 0x96e0cc, 2 fases, regras 0x799690, caixas nas linhas 0x57 e 0x5e); a
/// edição é a do ano do jogo, nunca antes de 2002, levada ao primeiro ano par.
unsafe extern "thiscall" fn cqw10_oceania_nacoes(this: usize, ano: u32, comp: usize) -> usize {
    crate::comparison::branch("cqw_10 copa da oceania");
    let edicao = cqw10_edicao((ano as i16).max(0x7d2), 0); // cqw10: a Copa da Oceania de 2002 em diante
    cqw10_torneio(this, edicao, comp, 0x96e0cc, 2, 0x799690, 0xa6a0ec, (0x57, 0x5e))
}

unsafe extern "thiscall" fn compare_cqw10_oceania_nacoes(this: usize, ano: u32, comp: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    cqw10_comparar("cqw10_oceania_nacoes", this, comp, 0x96e0cc, cqw10_edicao((ano as i16).max(0x7d2), 0), || o(this, ano, comp))
}
