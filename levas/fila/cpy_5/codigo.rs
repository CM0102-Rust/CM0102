// ---------- %%LEVA%%a leva: os inscritos da Liga dos Campeões: a lista das quatro bandas, a tabela da qualificação e a fase seguinte (european_cup.cpp) ----------

unsafe fn cpy5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn european_cup_malloc(n: u32) -> usize {
    let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    f(n)
}

/// Uma entrada de 6 bytes da lista dos inscritos: o clube (+0), a rodada em que entra (+4) e +5 = 0 (nesta ordem,
/// como a original).
unsafe fn european_cup_write_entrant(p: usize, clube: usize, rodada: u8) {
    core::ptr::write_unaligned(p as *mut usize, clube);
    *((p + 5) as *mut u8) = 0;
    *((p + 4) as *mut u8) = rodada;
}

/// O laço da 0x584fa0 sobre os clubes ([0xae23b4], 0x245 bytes, [0xae235c] deles): quem tem a competição europeia
/// (+0x1db) da Liga dos Campeões ([0x9cf6e8]) entra pela banda (+0x1df) na lista `t`: 1 em +0 (até 16, rodada 7),
/// 2 em +0x60 (até 18, rodada 8), 3 em +0xcc (até 19, rodada 0xd), 4 em +0x13e (até 18, rodada 0). Devolve a soma
/// das quatro contas.
unsafe fn fill_champions_league_entrants(t: usize) -> i32 {
    let clubes = cpy5_i(0xae235c);
    let (mut n1, mut n2, mut n3, mut n4) = (0i32, 0i32, 0i32, 0i32);
    let (mut o1, mut o2, mut o3, mut o4) = (0usize, 0usize, 0usize, 0usize);
    if clubes <= 0 {
        return 0;
    }
    let cl = cpy5_i(0x9cf6e8);
    let mut c = cpy5_d(0xae23b4);
    let mut i: i32 = 0;
    loop {
        if cpy5_i(c + 0x1db) == cl {
            match cpy5_b(c + 0x1df) {
                1 => {
                    if o1 < 0x60 {
                        european_cup_write_entrant(t + o1, c, 7);
                        n1 += 1;
                        o1 += 6;
                    }
                }
                2 => {
                    if o2 < 0x6c {
                        crate::comparison::branch("cpy_5 entrada da banda 2");
                        european_cup_write_entrant(t + 0x60 + o2, c, 8); // cpy5: a banda 2 entra na rodada 8
                        n2 += 1;
                        o2 += 6;
                    }
                }
                3 => {
                    if o3 < 0x72 {
                        crate::comparison::branch("cpy_5 entrada da banda 3");
                        european_cup_write_entrant(t + 0xcc + o3, c, 0xd); // cpy5: a banda 3 entra na rodada 0xd
                        n3 += 1;
                        o3 += 6;
                    }
                }
                4 => {
                    if o4 < 0x6c {
                        crate::comparison::branch("cpy_5 entrada da banda 4");
                        european_cup_write_entrant(t + 0x13e + o4, c, 0); // cpy5: a banda 4 entra na rodada 0
                        n4 += 1;
                        o4 += 6;
                    }
                }
                _ => {}
            }
        }
        c = c.wrapping_add(0x245);
        i += 1;
        if i as i16 as i32 >= clubes {
            break;
        }
    }
    n1 + n2 + n3 + n4
}

/// { 0x584fa0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::collect_entrants()`, do construtor (0x583888, na
/// carreira nova) e do começo da temporada (0x584e5d). +0x36 e +0x38 = 71; libera a lista antiga (+0x14, free
/// 0x9452ca) e aloca 6 × 71 bytes (0x944e46; sem memória: a caixa da linha 0x44e e 0, com +0x14 nulo); preenche as
/// quatro bandas (fill_champions_league_entrants). Sem as 71 entradas: +0x36 = 0 e 0; senão 1.
unsafe extern "thiscall" fn european_cup_collect_entrants(this: usize) -> u32 {
    *((this + 0x36) as *mut i16) = 0x47;
    *((this + 0x38) as *mut i16) = 0x47;
    let velha = cpy5_d(this + 0x14);
    if velha != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        liberar(velha);
    }
    let t = european_cup_malloc((cpy5_w(this + 0x36) as i32).wrapping_mul(6) as u32);
    core::ptr::write_unaligned((this + 0x14) as *mut usize, t);
    if t == 0 {
        show_error_at(0x9c4900, 0x44e);
        return 0;
    }
    if fill_champions_league_entrants(t) != cpy5_w(this + 0x36) as i32 {
        *((this + 0x36) as *mut i16) = 0;
        return 0;
    }
    1
}

/// Comparador da 0x584fa0: a original aloca (só ela roda); o plano confere depois tudo o que ela gravou: a lista nova
/// copiada e repreenchida pela cópia (fill_champions_league_entrants só lê os clubes, que a original não muda) tem de sair igual,
/// +0x36, +0x38 e a devolução.
unsafe extern "thiscall" fn comparar_european_cup_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let t = cpy5_d(this + 0x14);
    if t == 0 {
        PLACAR.record(r == 0, || format!("european_cup_collect_entrants({:x}) sem lista original={}", this, r));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x47 * 6).to_vec();
    let total = fill_champions_league_entrants(v.as_mut_ptr() as usize);
    let n = if total == 0x47 { 1 } else { 0 };
    let igual = r == n
        && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x47 * 6)
        && cpy5_w(this + 0x36) == if n == 1 { 0x47 } else { 0 }
        && cpy5_w(this + 0x38) == 0x47;
    PLACAR.record(igual, || format!("european_cup_collect_entrants({:x}) original={} +0x36={} rust={} total={}", this, r, cpy5_w(this + 0x36), n, total));
    r
}

/// O laço da 0x584990 sobre as 55 linhas (+0x3e) da tabela `t`: a linha i leva o clube da entrada (+0x36) − i − 1 da
/// lista (+0x14; lista vazia: 0) e a rodada pela entrada (+0x36) − i, byte +4 da anterior (8 → 2, 0xd → 1, outra →
/// 0); +5 = 0. Com +0x36 = 0 a original lê antes da lista (índice negativo), como aqui.
unsafe fn fill_qualifying_table(this: usize, t: usize) {
    let mut i: i32 = 0;
    let mut k = 0usize;
    while i < cpy5_w(this + 0x3e) as i32 {
        let n = cpy5_w(this + 0x36) as i32;
        let lista = cpy5_d(this + 0x14);
        let clube = if n != 0 { cpy5_d(lista.wrapping_add((n - i - 1).wrapping_mul(6) as usize)) } else { 0 };
        core::ptr::write_unaligned((t + k) as *mut usize, clube);
        let lista = cpy5_d(this + 0x14);
        let codigo = cpy5_b(lista.wrapping_add(((n - i).wrapping_mul(6) - 2) as usize));
        *((t + k + 4) as *mut u8) = match codigo {
            8 => 2,
            0xd => 1,
            _ => 0,
        };
        *((t + k + 5) as *mut u8) = 0;
        i += 1;
        k += 6;
    }
}

/// { 0x584990 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::build_qualifying_entry_table()`, logo depois da
/// 0x584fa0 (0x58388f, 0x584e9a). +0x3e = 55; aloca 6 × 55 bytes em +0xa7 (sem memória: a caixa da linha 0x2f6 e −1)
/// e monta as linhas da qualificação de trás para a frente (fill_qualifying_table); devolve 1.
unsafe extern "thiscall" fn build_qualifying_entry_table(this: usize) -> i32 {
    *((this + 0x3e) as *mut i16) = 0x37;
    let t = european_cup_malloc(0x14a);
    core::ptr::write_unaligned((this + 0xa7) as *mut usize, t);
    if t == 0 {
        show_error_at(0x9c4900, 0x2f6);
        return -1;
    }
    fill_qualifying_table(this, t);
    1
}

/// Comparador da 0x584990: a original aloca (só ela roda); depois a tabela dela, copiada e remontada pela cópia,
/// tem de sair igual, com +0x3e = 55 e a devolução.
unsafe extern "thiscall" fn comparar_build_qualifying_entry_table(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%)(this);
    let t = cpy5_d(this + 0xa7);
    if t == 0 || cpy5_w(this + 0x3e) != 0x37 {
        PLACAR.record(r == -1 && t == 0, || format!("build_qualifying_entry_table({:x}) original={} tabela={:x} +0x3e={}", this, r, t, cpy5_w(this + 0x3e)));
        return r;
    }
    let mut v = core::slice::from_raw_parts(t as *const u8, 0x14a).to_vec();
    fill_qualifying_table(this, v.as_mut_ptr() as usize);
    let igual = r == 1 && v[..] == *core::slice::from_raw_parts(t as *const u8, 0x14a);
    PLACAR.record(igual, || format!("build_qualifying_entry_table({:x}) original={} tabela={:x?}", this, r, &v[..]));
    r
}

/// { 0x5851d0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::advance_stage()` (a virtual +0x28, do passe da
/// copa 0x51f904). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x495. Senão +0x30 + 1 e, na fase
/// 0, o sorteio dos grupos da fase 1 (0x585290, thiscall(this)); na 8, os da fase 2 (0x585ab0, thiscall(this, 0),
/// ret 4); na 12, as quartas (0x586330, thiscall(this)). Quem chama não lê a devolução.
unsafe extern "thiscall" fn european_cup_advance_stage(this: usize) {
    let f = cpy5_i(this + 0x30);
    if f >= cpy5_i(this + 0x2c).wrapping_sub(1) {
        return show_error_at(0x9c4900, 0x495);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    crate::comparison::branch("cpy_5 fase seguinte");
    match f {
        0 => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x585290usize);
            sortear(this);
        }
        8 => {
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x585ab0usize);
            sortear(this, 0);
        }
        0xc => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x586330usize);
            sortear(this);
        }
        _ => {}
    }
}

// ponytail: os sorteios sorteiam, alocam as fases e mandam notícias: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_european_cup_advance_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
