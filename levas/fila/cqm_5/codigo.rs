// ---------- %%LEVA%%a leva: os construtores da segunda e da terceira divisões francesas e da Primera e da Segunda espanholas (as regras, a tabela, os jogos, as estatísticas, as posições previstas e a divisão de baixo espanhola na criação) (fra_second.cpp, fra_third.cpp, spa_second.cpp, spa_first.cpp) ----------

unsafe fn cqm5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O que muda de uma divisão para outra no construtor.
struct Cqm5Divisao {
    vtabela: usize,
    /// a regra de disciplina (+0x50): 0xc a francesa, 0x1a a espanhola
    disciplina: u8,
    regras: usize,
    tabela: usize,
    arquivo: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    linhas: [u32; 5],
    /// a Primera cria no fim a divisão de baixo espanhola (+0xee)
    baixo: bool,
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para os operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a disciplina
/// (+0x50); tenta carregar o estado (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4).
/// Carregou: pronto. Senão, a liga nova, só a liga: fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c = 0, +0xc = 0; as
/// regras, a tabela, os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 ->
/// thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats 0x49ee70:
/// push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0);
/// cada passo que falha mostra a caixa da sua linha e encerra. Na Primera, por fim: se a nação da competição
/// ([+4]+0x5d) não tem o bit 4 de +0x11c [inferência: a divisão de baixo jogável], a divisão de baixo espanhola
/// (operator new de 0xee; SpaLower::SpaLower 0x851020: push ano; mov ecx, novo -> thiscall, ret 4) vai para +0xee
/// (0 sem memória); com o bit, +0xee = 0. Devolve this.
unsafe fn cqm5_construir(this: usize, ano: u32, registro: usize, p: &Cqm5Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    let baixo: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x851020usize);
    base(this);
    cqm5_pd(this + 4, registro);
    cqm5_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = p.disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_5 divisao nova");
    *((this + 0x44) as *mut u8) = 0xff;
    cqm5_pd(this + 0x30, 0xffff_ffff);
    cqm5_pd(this + 0x2c, 0);
    cqm5_pd(this + 0xc, 0);
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cqm5_pd(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    if p.baixo {
        let nacao = cqm5_d(cqm5_d(this + 4) + 0x5d);
        if *((nacao + 0x11c) as *const u8) & 4 == 0 { // cqm5: a divisão de baixo espanhola
            crate::comparacao::ramo("cqm_5 divisao de baixo espanhola");
            let e = novo(0xee);
            // ponytail: a original empurra o ano na palavra de baixo do ecx, com o resto do ponteiro do registro em
            // cima; a 0x851020 só lê a palavra (mov ax, word ptr [esp + 0x21c]).
            let b = if e != 0 { baixo(e, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32) } else { 0 };
            cqm5_pd(this + 0xee, b);
        } else {
            cqm5_pd(this + 0xee, 0);
        }
    }
    this
}

const CQM5_FRANCA_2: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96ac2c,
    disciplina: 0xc, // cqm5: a regra de disciplina francesa
    regras: 0x5bf0a0,
    tabela: 0x5bef10,
    arquivo: 0x9ccc0c,
    linhas: [0x43, 0x4a, 0x51, 0x5a, 0x61],
    baixo: false,
};
const CQM5_FRANCA_3: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96ad80,
    disciplina: 0xc,
    regras: 0x5c0700,
    tabela: 0x5c05d0,
    arquivo: 0x9ccc80,
    linhas: [0x42, 0x49, 0x50, 0x59, 0x60],
    baixo: false,
};
const CQM5_SEGUNDA: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96fbd0,
    disciplina: 0x1a, // cqm5: a regra de disciplina espanhola
    regras: 0x855cc0,
    tabela: 0x855b40,
    arquivo: 0xa828bc,
    linhas: [0x46, 0x4d, 0x54, 0x5b, 0x62],
    baixo: false,
};
const CQM5_PRIMERA: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96fa68,
    disciplina: 0x1a,
    regras: 0x84fdf0,
    tabela: 0x84fc70,
    arquivo: 0xa82844,
    linhas: [0x47, 0x4e, 0x55, 0x5c, 0x63],
    baixo: true,
};

/// { 0x5be330 } (thiscall, ret 8, fra_second.cpp): `FraSecond::FraSecond(ano, registro)`: a segunda divisão francesa
/// (a Division 2), disciplina 0xc, só a liga.
unsafe extern "thiscall" fn construct_fra_second(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_FRANCA_2)
}

/// { 0x5bfb40 } (thiscall, ret 8, fra_third.cpp): `FraThird::FraThird(ano, registro)`: a terceira divisão francesa (o
/// National), disciplina 0xc, só a liga.
unsafe extern "thiscall" fn construct_fra_third(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_FRANCA_3)
}

/// { 0x8539a0 } (thiscall, ret 8, spa_second.cpp): `SpaSecond::SpaSecond(ano, registro)`: a Segunda espanhola,
/// disciplina 0x1a, só a liga.
unsafe extern "thiscall" fn construct_spa_second(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_SEGUNDA)
}

/// { 0x84e130 } (thiscall, ret 8, spa_first.cpp, 606 bytes): `SpaFirst::SpaFirst(ano, registro)`: a Primera,
/// disciplina 0x1a, só a liga, e a divisão de baixo espanhola em +0xee.
unsafe extern "thiscall" fn construct_spa_first(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_PRIMERA)
}

/// O comparador dos quatro: alocam: só a original, conferindo a devolução, a vtable, o ano, o registro e a disciplina;
/// na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c = 0 e +0xc = 0; na Primera, a divisão de baixo em +0xee
/// conforme o bit 4 da nação.
unsafe fn cqm5_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cqm5Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqm5_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqm5_d(this + 4) == registro && *((this + 0x50) as *const u8) == p.disciplina;
    if *((this + 0x44) as *const u8) == 0xff && cqm5_d(this + 8) != 0 {
        ok = ok && cqm5_d(this + 0x2c) == 0 && cqm5_d(this + 0xc) == 0;
        if p.baixo {
            let nacao = cqm5_d(cqm5_d(this + 4) + 0x5d);
            let tem = cqm5_d(this + 0xee) != 0;
            ok = ok && tem == (*((nacao + 0x11c) as *const u8) & 4 == 0);
        }
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +8 = {:#x}", nome, this, r, cqm5_d(this), cqm5_d(this + 0x2c), cqm5_d(this + 8)));
    r
}

unsafe extern "thiscall" fn comparar_construct_fra_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    cqm5_comparar("construct_fra_second", this, ano, registro, &CQM5_FRANCA_2, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_fra_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    cqm5_comparar("construct_fra_third", this, ano, registro, &CQM5_FRANCA_3, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_spa_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    cqm5_comparar("construct_spa_second", this, ano, registro, &CQM5_SEGUNDA, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_spa_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%);
    cqm5_comparar("construct_spa_first", this, ano, registro, &CQM5_PRIMERA, || o(this, ano, registro))
}
