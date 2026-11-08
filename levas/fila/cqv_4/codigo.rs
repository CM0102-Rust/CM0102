// ---------- %%LEVA%%a leva: as fases de liga: o construtor e o destrutor de League_Stage (league_stage.cpp) ----------

unsafe fn cqv4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqv4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqv4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Os clubes das +0x3e linhas da tabela (+0xb1, 0x3b bytes cada), ordenados (para comparar como conjunto).
unsafe fn cqv4_clubes_da_tabela(this: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..cqv4_w(this + 0x3e).max(0) as usize).map(|i| cqv4_d(cqv4_d(this + 0xb1) + 0x3b * i)).collect();
    v.sort_unstable();
    v
}

/// { 0x68b100 } (thiscall, ret 0x58, league_stage.cpp, 964 bytes; quadro SEH sem destrutor no caminho normal):
/// `League_Stage::League_Stage(...)`, uma fase de pontos corridos dentro de uma competição (os turnos dos estaduais e da
/// J1, os grupos da Série C, as conferências da MLS...). League::League (0x682200, thiscall, ret 0); a vtable 0x96d0f0;
/// os 22 argumentos nos campos, na ordem da original: +0x3e = os clubes (word), +0x3c = os turnos (word), +4 = o
/// registro, +0xa9 = as rodadas (word), +0xc2 / +0xc3 / +0xc4 = os pontos por vitória / por empate / a categoria
/// (bytes), +0xc5..+0xc8 = os 4 critérios de desempate (copiados), +0xbe..+0xc1 = as 4 vagas (copiadas), +0x40 = o ano
/// (word), +0x44 (byte), +0x3a (word), +0x51 (byte), +0x42 (byte), +0xab, +0xd9, +0xa7 (words), +0x28 (dword), +0xe1
/// (byte); +0x30 = +0x2c = +0x20 = +0x1c = −1, +0x43 = 2, +0x49 e +0x4a = os da competição de cima. Com a tabela de
/// rodadas dada: uma cópia dela (malloc 0x944e46 de 0x41 × rodadas; sem memória: a caixa da linha 0x60 e fim) em +0xba;
/// senão +0xba = 0. A tabela: malloc de 0x3b × clubes em +0xb1 (sem memória: a caixa 0x6f e fim); cada linha i
/// (League::init_table_row 0x687430, thiscall(this, i, clube ou 0, 0, 0), ret 0x10; a original empurra o dword com i no
/// byte de baixo e sobra da pilha em cima; a rotina só lê o byte). Com rodadas: os jogos (League::create_fixtures
/// 0x6835c0, thiscall, ret 0; 0: a caixa 0x7f e fim); sem elas: uma rodada só (malloc de 0x41 em +0xba; sem memória: a
/// caixa 0x8a e fim) com a data de hoje ([0xae2c90] na palavra +0, [0xae2c92] − o ano da fase na +2, 0 no byte +4) e
/// +0xa9 = 1. Por fim a ordem (League::sort_table 0x6827d0, thiscall(this, 0), ret 4). Devolve this.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn cqv4_construir_fase(this: usize, liga: usize, n: u32, clubes: usize, turnos: u32, registro: u32, rodadas: usize,
                                              nrod: u32, pv: u32, pe: u32, cat: u32, desempate: usize, vagas: usize, ano: u32, a13: u32,
                                              a14: u32, a15: u32, a16: u32, a17: u32, a18: u32, a19: u32, a20: u32, a21: u32) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    base(this);
    crate::comparacao::ramo("cqv_4 fase");
    let w = |o: usize, v: u32| core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
    let b = |o: usize, v: u32| *((this + o) as *mut u8) = v as u8;
    let x = |o: usize, v: u32| core::ptr::write_unaligned((this + o) as *mut u32, v);
    w(0x3e, n);
    w(0x3c, turnos);
    w(0xa9, nrod);
    b(0xc2, pv); // cqv4: os pontos por vitória da fase
    x(4, registro);
    b(0xc4, cat);
    b(0x51, a15);
    x(0, 0x96d0f0);
    b(0xc3, pe); // cqv4: os pontos por empate da fase
    for i in 0..4 {
        b(0xc5 + i, cqv4_b(desempate + i) as u32);
    }
    for i in 0..4 {
        b(0xbe + i, cqv4_b(vagas + i) as u32);
    }
    w(0x40, ano);
    x(0x30, 0xffff_ffff);
    x(0x2c, 0xffff_ffff);
    x(0x20, 0xffff_ffff);
    x(0x1c, 0xffff_ffff);
    b(0x44, a13);
    w(0x3a, a14);
    b(0x42, a16);
    w(0xa7, a19);
    b(0x43, 2);
    w(0xab, a17);
    w(0xd9, a18);
    b(0x49, cqv4_b(liga + 0x49) as u32);
    b(0x4a, cqv4_b(liga + 0x4a) as u32);
    x(0x28, a20);
    b(0xe1, a21);
    if rodadas != 0 {
        let t = (cqv4_w(this + 0xa9) as i32 * 0x41) as u32;
        let c = alocar(t);
        x(0xba, c as u32);
        if c == 0 {
            show_error_at(0x9f1630, 0x60);
            return this;
        }
        core::ptr::copy_nonoverlapping(rodadas as *const u8, c as *mut u8, t as usize);
    } else {
        x(0xba, 0);
    }
    let t = alocar((cqv4_w(this + 0x3e) as i32 * 0x3b) as u32);
    x(0xb1, t as u32);
    if t == 0 {
        show_error_at(0x9f1630, 0x6f);
        return this;
    }
    let total = cqv4_w(this + 0x3e) as i32;
    let mut i: i8 = 0;
    while (i as i32) < total {
        let c = if clubes != 0 { cqv4_d(clubes + 4 * i as usize) } else { 0 };
        linha(this, i as u8 as u32, c, 0, 0);
        i = i.wrapping_add(1);
    }
    if cqv4_d(this + 0xba) != 0 {
        if jogos(this) == 0 {
            show_error_at(0x9f1630, 0x7f);
            return this;
        }
    } else {
        let r = alocar(0x41);
        x(0xba, r as u32);
        if r == 0 {
            show_error_at(0x9f1630, 0x8a);
            return this;
        }
        crate::comparacao::ramo("cqv_4 fase de uma rodada");
        core::ptr::write_unaligned(r as *mut u16, core::ptr::read_unaligned(0xae2c90 as *const u16));
        core::ptr::write_unaligned((r + 2) as *mut i16, cqv4_w(0xae2c92).wrapping_sub(cqv4_w(this + 0x40)));
        *((r + 4) as *mut u8) = 0;
        w(0xa9, 1);
    }
    ordenar(this, 0);
    this
}

/// O comparador da fase: só a original (aloca a tabela e as rodadas e sorteia os jogos), conferindo o que ela grava e
/// nenhuma rotina de dentro reescreve: a devolução, a vtable, o registro, os clubes, os turnos, o ano, +0x44, +0x51, os
/// pontos, a categoria, os critérios e as vagas copiados, +0x43 = 2, +0x49/+0x4a da competição de cima, +0x28, +0xe1,
/// a tabela e as rodadas alocadas e os clubes da tabela (como conjunto: a ordem os reordena) iguais aos da lista dada.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_cqv4_construir_fase(this: usize, liga: usize, n: u32, clubes: usize, turnos: u32, registro: u32, rodadas: usize,
                                                       nrod: u32, pv: u32, pe: u32, cat: u32, desempate: usize, vagas: usize, ano: u32, a13: u32,
                                                       a14: u32, a15: u32, a16: u32, a17: u32, a18: u32, a19: u32, a20: u32, a21: u32) -> usize {
    let mut lista: Vec<usize> = if clubes != 0 { (0..(n as u16 as i16).max(0) as usize).map(|i| cqv4_d(clubes + 4 * i)).collect() } else { Vec::new() };
    lista.sort_unstable();
    let crit: [u8; 4] = core::array::from_fn(|i| cqv4_b(desempate + i));
    let vag: [u8; 4] = core::array::from_fn(|i| cqv4_b(vagas + i));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(%%I0%%)(
        this, liga, n, clubes, turnos, registro, rodadas, nrod, pv, pe, cat, desempate, vagas, ano, a13, a14, a15, a16, a17, a18, a19, a20, a21);
    let mut ok = r == this && cqv4_d(this) == 0x96d0f0 && cqv4_d(this + 4) == registro as usize && cqv4_w(this + 0x3e) == n as i16
        && cqv4_w(this + 0x3c) == turnos as i16 && cqv4_w(this + 0x40) == ano as i16 && cqv4_b(this + 0x44) == a13 as u8
        && cqv4_b(this + 0x51) == a15 as u8 && cqv4_b(this + 0xc2) == pv as u8 && cqv4_b(this + 0xc3) == pe as u8 && cqv4_b(this + 0xc4) == cat as u8
        && (0..4).all(|i| cqv4_b(this + 0xc5 + i) == crit[i] && cqv4_b(this + 0xbe + i) == vag[i]) && cqv4_b(this + 0x43) == 2
        && cqv4_b(this + 0x49) == cqv4_b(liga + 0x49) && cqv4_b(this + 0x4a) == cqv4_b(liga + 0x4a) && cqv4_d(this + 0x28) == a20 as usize
        && cqv4_b(this + 0xe1) == a21 as u8 && cqv4_d(this + 0xb1) != 0 && cqv4_d(this + 0xba) != 0;
    if ok && clubes != 0 {
        ok = cqv4_clubes_da_tabela(this) == lista;
    }
    PLACAR.registrar(ok, || format!("cqv4_construir_fase({:#x}): devolveu {:#x}, vtable {:#x}, +0x3e {}, +0xc2 {}, +0xb1 {:#x}, +0xba {:#x}", this, r, cqv4_d(this), cqv4_w(this + 0x3e), cqv4_b(this + 0xc2), cqv4_d(this + 0xb1), cqv4_d(this + 0xba)));
    r
}

/// { 0x68b6a0 } (thiscall, ret 0, league_stage.cpp, 123 bytes; quadro SEH sem destrutor no caminho normal):
/// `League_Stage::~League_Stage()`: a vtable 0x96d0f0, a tabela liberada (League::free_table 0x687970, thiscall(this,
/// 0), ret 4), as rodadas (+0xba) liberadas (free 0x9452ca, sem zerar), o vetor +0x18 liberado e zerado, e o destrutor
/// de League (0x682300, thiscall, ret 0).
unsafe extern "thiscall" fn cqv4_destruir_fase(this: usize) {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x682300usize);
    core::ptr::write_unaligned(this as *mut usize, 0x96d0f0);
    liberar_tabela(this, 0);
    let r = cqv4_d(this + 0xba);
    if r != 0 {
        liberar(r);
    }
    let v = cqv4_d(this + 0x18);
    if v != 0 {
        liberar(v);
        core::ptr::write_unaligned((this + 0x18) as *mut usize, 0);
    }
    destruir(this);
}

/// O comparador do destrutor: só a original (libera; não roda duas vezes), conferindo o vetor +0x18 zerado.
unsafe extern "thiscall" fn comparar_cqv4_destruir_fase(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    PLACAR.registrar(cqv4_d(this + 0x18) == 0, || format!("cqv4_destruir_fase({:#x}): +0x18 = {:#x}", this, cqv4_d(this + 0x18)));
}
