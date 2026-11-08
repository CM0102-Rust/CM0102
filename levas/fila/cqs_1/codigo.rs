// ---------- %%LEVA%%a leva: as três divisões da MLS: a tabela da primeira e as fases da segunda e da terceira (usa_mls.cpp) ----------

unsafe fn cqs1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O código de divisão do clube `c` na MLS (usa_mls_get_club_division 0x909ec0: push c; call; add esp, 4 -> cdecl; a
/// palavra baixa do eax: 0x413, 0x47b ou 0x414), se ele é da competição da liga (+0x57 do clube = +4 da liga).
unsafe fn is_in_mls_division(this: usize, c: usize, codigo: i16) -> bool {
    let divisao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x909ec0usize);
    cqs1_d(c + 0x57) == cqs1_d(this + 4) && divisao(c) as u16 as i16 == codigo
}

/// { 0x908560 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::create_table()`, a tabela da MLS (a primeira divisão, código
/// 0x413): +0x3e = 4; a tabela (malloc 0x944e46 de 0xec, 4 linhas de 0x3b; sem memória: a caixa da linha 0x2fe e 0)
/// em +0xb1; na ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes, contador i16), enquanto faltam linhas, cada
/// clube da competição com o código 0x413 vai à linha seguinte (League::init_table_row 0x687430: push 0; push 0; push
/// clube; push eax com a linha no byte baixo e o lixo do código em cima, que ela não lê; mov ecx -> thiscall, ret
/// 0x10). Devolve 1 se encheu as 4 linhas.
unsafe extern "thiscall" fn usa_mls_create_table(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = alocar(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(0xaa1fb4, 0x2fe);
        return 0;
    }
    let mut n: i16 = 0;
    let base = cqs1_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < cqs1_d(0xae235c) as i32 && n < cqs1_w(this + 0x3e) {
        let c = base + i as u16 as usize * 0x245;
        if is_in_mls_division(this, c, 0x413) {
            iniciar(this, n as u8 as u32, c, 0, 0); // cqs1: a linha seguinte da primeira divisão
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    crate::comparison::branch("cqs_1 tabela da mls");
    (n == cqs1_w(this + 0x3e)) as u32
}

/// A fase de uma das outras divisões da MLS (`create_second_division_stage` e `create_third_division_stage`; o mesmo
/// molde, conferido instrução a instrução: só mudam o código da divisão, a vaga k, o fonte das caixas e as linhas).
/// 1) Um vetor de +0x3e clubes (malloc 0x944e46 de 4 × +0x3e; sem memória: a caixa da linha `linhas[0]` e volta) com os
///    clubes da competição com o `codigo`, na ordem do banco (contador i32), enquanto faltam vagas.
/// 2) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, −3, &rodadas, &extra, 0), ret 0x10).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, +0x3c (word, os turnos), +4, a tabela das rodadas, as rodadas
///    (word), os pontos +0xc2 (`pv`) e +0xc3, a categoria +0xc4, &+0xc5, &+0xbe (as vagas da liga), o ano +0x40 (word),
///    k, o extra (word), +0x51, 1, +0xab (word), +0xd9 (word), +0xa7 (word), 0, 2; a cópia passa os valores limpos).
///    Sem memória: 0.
/// 4) [+0xc][k] = a fase; o vetor e a tabela das rodadas liberados (free 0x9452ca, nessa ordem). Sem fase: a caixa da
///    linha `linhas[1]`. Com ela: +0x30 = `atual` (a original grava k) e +0x3a da fase = o código da divisão.
unsafe fn create_division_stage(this: usize, codigo: i16, k: u32, atual: u32, pv: usize, linhas: [u32; 2], ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let v = alocar(((cqs1_w(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, linhas[0]);
    }
    let mut n: i16 = 0;
    let base = cqs1_d(0xae23b4);
    let mut i = 0i32;
    while i < cqs1_d(0xae235c) as i32 && n < cqs1_w(this + 0x3e) {
        let c = base + i as usize * 0x245;
        if is_in_mls_division(this, c, codigo) {
            core::ptr::write_unaligned((v + 4 * n as usize) as *mut usize, c);
            n += 1;
        }
        i += 1;
    }
    let tabela: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs1_d(cqs1_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, -3, &mut rodadas, &mut extra, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, cqs1_w(this + 0x3e) as u16 as u32, v, cqs1_w(this + 0x3c) as u16 as u32, cqs1_d(this + 4), t, rodadas as u16 as u32,
                  cqs1_b(this + pv) as u32, cqs1_b(this + 0xc3) as u32, cqs1_b(this + 0xc4) as u32, this + 0xc5, this + 0xbe,
                  cqs1_w(this + 0x40) as u16 as u32, k, extra as u16 as u32, cqs1_b(this + 0x51) as u32, 1, cqs1_w(this + 0xab) as u16 as u32,
                  cqs1_w(this + 0xd9) as u16 as u32, cqs1_w(this + 0xa7) as u16 as u32, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs1_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    soltar(v);
    soltar(t);
    let fase = cqs1_d(cqs1_d(this + 0xc) + 4 * k as usize);
    if fase == 0 {
        return show_error_at(0xaa1fb4, linhas[1]);
    }
    core::ptr::write_unaligned((this + 0x30) as *mut u32, atual);
    crate::comparison::branch(ramo);
    core::ptr::write_unaligned((fase + 0x3a) as *mut i16, codigo);
}

/// { 0x908fc0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_second_division_stage()`, a
/// segunda divisão da MLS (código 0x47b) na fase 0; caixas nas linhas 0x4e6 e 0x506.
unsafe extern "thiscall" fn create_second_division_stage(this: usize) {
    create_division_stage(this, 0x47b, 0, 0, 0xc2, [0x4e6, 0x506], "cqs_1 segunda divisao") // cqs1: os pontos por vitória na segunda
}

/// { 0x9091e0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_third_division_stage()`, a
/// terceira divisão da MLS (código 0x414) na fase 1; caixas nas linhas 0x524 e 0x544.
unsafe extern "thiscall" fn create_third_division_stage(this: usize) {
    create_division_stage(this, 0x414, 1, 1, 0xc2, [0x524, 0x544], "cqs_1 terceira divisao") // cqs1: a fase atual depois da terceira
}

/// O comparador da tabela: só a original (ponytail: aloca a tabela e as listas das linhas), com o plano feito antes:
/// os clubes da competição com o código 0x413, na ordem do banco, até 4. Depois, a devolução, +0x3e = 4 e as linhas.
unsafe extern "thiscall" fn compare_usa_mls_create_table(this: usize) -> u32 {
    let mut plano = [0usize; 4];
    let mut n = 0usize;
    let base = cqs1_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < cqs1_d(0xae235c) as i32 && n < 4 {
        let c = base + i as u16 as usize * 0x245;
        if is_in_mls_division(this, c, 0x413) {
            plano[n] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let t = cqs1_d(this + 0xb1);
    let igual = r == (n == 4) as u32 && cqs1_w(this + 0x3e) == 4 && t != 0 && (0..n).all(|k| cqs1_d(t + k * 0x3b) == plano[k]);
    SCOREBOARD.record(igual, || format!("usa_mls_create_table({:#x}) r={} n={} plano={:x?}", this, r, n, plano));
    r
}

/// O comparador de uma divisão: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes:
/// os clubes do código, na ordem do banco. Depois, a fase na vaga k com +0x3e da liga, os clubes na tabela dela (como
/// conjunto), +0x3a = o código e +0x30 = k.
unsafe fn compare_division_stage(nome: &str, this: usize, codigo: i16, k: usize, original: impl FnOnce()) {
    let mut plano = [0usize; 8];
    let mut n = 0usize;
    let base = cqs1_d(0xae23b4);
    let mut i = 0i32;
    while i < cqs1_d(0xae235c) as i32 && (n as i16) < cqs1_w(this + 0x3e) && n < 8 {
        let c = base + i as usize * 0x245;
        if is_in_mls_division(this, c, codigo) {
            plano[n] = c;
            n += 1;
        }
        i += 1;
    }
    original();
    let fase = cqs1_d(cqs1_d(this + 0xc) + 4 * k);
    let igual = fase != 0 && cqs1_w(fase + 0x3a) == codigo && cqs1_d(this + 0x30) == k && cqs1_w(fase + 0x3e) == cqs1_w(this + 0x3e) && {
        let mut nela = [0usize; 8];
        for (j, c) in nela.iter_mut().enumerate().take(n) {
            *c = cqs1_d(cqs1_d(fase + 0xb1) + j * 0x3b);
        }
        let (mut a, mut b) = (plano, nela);
        a[..n].sort_unstable();
        b[..n].sort_unstable();
        a[..n] == b[..n]
    };
    SCOREBOARD.record(igual, || format!("{}({:#x}) fase={:#x} n={} plano={:x?}", nome, this, fase, n, plano));
}

unsafe extern "thiscall" fn compare_create_second_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_division_stage("create_second_division_stage", this, 0x47b, 0, || o(this))
}

unsafe extern "thiscall" fn compare_create_third_division_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_division_stage("create_third_division_stage", this, 0x414, 1, || o(this))
}
