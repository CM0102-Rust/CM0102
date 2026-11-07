// ---------- %%LEVA%%a leva: os construtores das copas asiáticas de clubes: a Liga dos Campeões, a Recopa e a Supercopa da Ásia (asia_club_champ.cpp, asia_cup_winner.cpp, asia_super_cup.cpp) ----------

unsafe fn cqw13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqw13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqw13_b(this: usize, d: usize, v: u8) {
    *((this + d) as *mut u8) = v;
}

/// As estatísticas (0x944cf1(0x5ce) e o construtor Comp_Stats 0x49ee70(novo, this): push this; mov ecx, novo ->
/// thiscall, ret 4) em +8; nulas: a caixa `linha` e false.
unsafe fn cqw13_estatisticas(this: usize, arquivo: usize, linha: u32) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let p = novo(0x5ce);
    let e = if p != 0 { estatisticas(p, this) } else { 0 };
    cqw13_pd(this + 8, e);
    if e == 0 {
        aviso_em(arquivo, linha);
        return false;
    }
    true
}

/// A tabela das rodadas da copa (a virtual +0x3c(−1, &+0x3c, &+0x3a, 0): push 0; push; push; push −1; mov ecx ->
/// thiscall, ret 0x10) em +0xa3.
unsafe fn cqw13_rodadas(this: usize) {
    let tabela: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(cqw13_d(cqw13_d(this) + 0x3c));
    cqw13_pd(this + 0xa3, tabela(this, -1, this + 0x3c, this + 0x3a, 0));
}

/// As rotinas e as linhas das caixas de uma das duas copas de 32 clubes.
struct Cqw13Copa {
    vtable: usize,
    fases: u32,
    arquivo: usize,
    classificados: usize,
    times: usize,
    fase: usize,
    previstas: usize,
    linhas: [u32; 4],
}

/// O corpo das duas copas asiáticas de 32 clubes (0x40aae0 e 0x40cbe0): o construtor da Cup (0x518640, thiscall, ret
/// 0); +4 o registro, a vtable, o ano +0x40, +0xab = 0, +0x44 = −1, +0x30 = −1, +0x2c = fases, +0x43 = 1, +0x42 = 2, +0x1c
/// = +0x20 = −1, +0xb2 = 0, +0x50 = 0x29, +0x49 = 5, +0x4a = 3; o que estiver salvo (Cup::load 0x51fc00(this, 1): push 1;
/// mov ecx -> thiscall, ret 4); sem nada: o vetor das fases +0xc (0x944e46(4 × fases); com fases ≤ 0, nulo), +0x51 =
/// 0xf, +0xb1 = 0, os 32 classificados (0: a caixa `linhas[0]`), a tabela dos times (0: a caixa `linhas[1]`), o prêmio
/// +0x28 = 4.000.000, a tabela das rodadas, as estatísticas (a caixa `linhas[2]`), a fase da zona oeste, a lista +0xb2
/// liberada (0x9452ca) e zerada, a árvore dos jogos (0x518790; 0: a caixa `linhas[3]`) e as posições previstas. Todas
/// as rotinas thiscall, ret 0. Devolve o this.
unsafe fn cqw13_copa_de_32(this: usize, ano: i16, comp: usize, c: &Cqw13Copa) -> usize {
    let copa: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let arvore: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let rotina = |a: usize| -> unsafe extern "thiscall" fn(usize) -> u32 { core::mem::transmute(a) };
    copa(this);
    cqw13_pd(this + 4, comp);
    cqw13_pd(this, c.vtable);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, ano);
    cqw13_b(this, 0xab, 0);
    cqw13_b(this, 0x44, 0xff);
    cqw13_pd(this + 0x30, usize::MAX);
    cqw13_pd(this + 0x2c, c.fases as usize);
    cqw13_b(this, 0x43, 1);
    cqw13_b(this, 0x42, 2);
    cqw13_pd(this + 0x1c, usize::MAX);
    cqw13_pd(this + 0x20, usize::MAX);
    cqw13_pd(this + 0xb2, 0);
    cqw13_b(this, 0x50, 0x29);
    cqw13_b(this, 0x49, 5);
    cqw13_b(this, 0x4a, 3);
    if carregar(this, 1) != 0 {
        return this;
    }
    let n = cqw13_d(this + 0x2c) as i32;
    cqw13_pd(this + 0xc, if n > 0 { alocar((n as u32).wrapping_mul(4)) } else { 0 });
    cqw13_b(this, 0x51, 0xf);
    cqw13_b(this, 0xb1, 0);
    if rotina(c.classificados)(this) == 0 {
        aviso_em(c.arquivo, c.linhas[0]);
        return this;
    }
    if rotina(c.times)(this) == 0 {
        aviso_em(c.arquivo, c.linhas[1]);
        return this;
    }
    cqw13_pd(this + 0x28, 0x3d0900);
    cqw13_rodadas(this);
    if !cqw13_estatisticas(this, c.arquivo, c.linhas[2]) {
        return this;
    }
    rotina(c.fase)(this);
    let l = cqw13_d(this + 0xb2);
    if l != 0 {
        soltar(l);
        cqw13_pd(this + 0xb2, 0);
    }
    if arvore(this) == 0 {
        aviso_em(c.arquivo, c.linhas[3]);
        return this;
    }
    rotina(c.previstas)(this);
    this
}

const CQW13_LIGA: Cqw13Copa = Cqw13Copa { vtable: 0x9674d4, fases: 4, arquivo: 0x987a40, classificados: 0x40b950, times: 0x40b620, fase: 0x40bec0, previstas: 0x40c720, linhas: [0x4d, 0x54, 0x63, 0x73] };
const CQW13_RECOPA: Cqw13Copa = Cqw13Copa { vtable: 0x967574, fases: 2, arquivo: 0x987b60, classificados: 0x40d830, times: 0x40d4d0, fase: 0x40ddf0, previstas: 0x40e3b0, linhas: [0x4e, 0x55, 0x64, 0x74] };

/// { 0x40aae0 } (thiscall, ret 8, asia_club_champ.cpp; SEH só para exceções): `AsiaClubChamp::AsiaClubChamp(ano,
/// comp)`: a Liga dos Campeões asiática do ano do jogo (vtable 0x9674d4, 4 fases; 0x40b950, 0x40b620, 0x40bec0,
/// 0x40c720; caixas nas linhas 0x4d, 0x54, 0x63 e 0x73).
unsafe extern "thiscall" fn cqw13_liga_asiatica(this: usize, ano: u32, comp: usize) -> usize {
    crate::comparacao::ramo("cqw_13 liga dos campeoes asiatica");
    let ano = ano as i16; // cqw13: a Liga dos Campeões asiática do ano do jogo
    cqw13_copa_de_32(this, ano, comp, &CQW13_LIGA)
}

// ponytail: o construtor aloca, sorteia e cria fases: não roda duas vezes. Só a original; o plano: devolve o this, a
// vtable, o registro +4 e o ano +0x40.
unsafe fn cqw13_comparar(nome: &str, this: usize, ano: u32, comp: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let lido = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let ok = r == this && cqw13_d(this) == vtable && cqw13_d(this + 4) == comp && lido == ano as i16;
    PLACAR.registrar(ok, || format!("{}({:#x}): ano {} pedido {}, vtable {:#x}", nome, this, lido, ano as i16, cqw13_d(this)));
    r
}

unsafe extern "thiscall" fn comparar_cqw13_liga_asiatica(this: usize, ano: u32, comp: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    cqw13_comparar("cqw13_liga_asiatica", this, ano, comp, 0x9674d4, || o(this, ano, comp))
}

/// { 0x40cbe0 } (thiscall, ret 8, asia_cup_winner.cpp; SEH só para exceções): `AsiaCupWinner::AsiaCupWinner(ano,
/// comp)`: a Recopa asiática do ano do jogo (vtable 0x967574, 2 fases; 0x40d830, 0x40d4d0, 0x40ddf0, 0x40e3b0; caixas
/// nas linhas 0x4e, 0x55, 0x64 e 0x74).
unsafe extern "thiscall" fn cqw13_recopa_asiatica(this: usize, ano: u32, comp: usize) -> usize {
    crate::comparacao::ramo("cqw_13 recopa asiatica");
    let ano = ano as i16; // cqw13: a Recopa asiática do ano do jogo
    cqw13_copa_de_32(this, ano, comp, &CQW13_RECOPA)
}

unsafe extern "thiscall" fn comparar_cqw13_recopa_asiatica(this: usize, ano: u32, comp: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    cqw13_comparar("cqw13_recopa_asiatica", this, ano, comp, 0x967574, || o(this, ano, comp))
}
