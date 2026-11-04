// ---------- %%LEVA%%a leva: a tabela das quatro divisões inglesas: os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV no início da temporada (eng_prm.cpp, eng_second.cpp, eng_third.cpp, eng_first.cpp) ----------

unsafe fn cqi5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqi5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqi5_registro(g: usize) -> usize {
    cqi5_d(0xae23d0).wrapping_add((cqi5_d(g) as i32 as usize).wrapping_mul(0x6b))
}

/// O ano do jogo ([0xae2c92]) passou de 1999 (o dinheiro da TV maior).
unsafe fn cqi5_depois_de_1999() -> bool {
    core::ptr::read_unaligned(0xae2c92 as *const u16) > 0x7cf
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8): o valor a cada clube da tabela, ou só aos
/// que vieram da competição `filtro` (+0x5b do clube).
unsafe fn cqi5_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, valor, filtro);
}

/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Sem encher: 0. Cheia: o fundo de prêmios
/// +0x28 (League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4), +0xe1 = 1, o dinheiro da TV (`pagar`) e 1.
unsafe fn cqi5_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, cota: i32, pagar: impl FnOnce()) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqi5_d(0xae23b4);
    while (i as i32) < cqi5_d(0xae235c) as i32 {
        let comp = cqi5_d(clube + 0x57);
        if comp != 0 && cqi5_d(comp) == cqi5_d(g) {
            let linha = k; // cqi5: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqi5_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqi5_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo("cqi_5 tabela cheia");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar();
    1
}

/// { 0x574a20 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::create_table()`: 20 clubes ([0x9cf5bc]), cota 250.000 do
/// fundo de prêmios e a TV: 12.000.000 a cada clube depois de 1999 (5.000.000 antes). Fonte 0x9c43b0, linha 0x1ee.
unsafe extern "thiscall" fn tabela_da_premier(this: usize) -> u32 {
    cqi5_tabela(this, 0x14, 0x9c43b0, 0x1ee, 0x9cf5bc, 0x3d090, || {
        let valor = if cqi5_depois_de_1999() { 0xb71b00 } else { 0x4c4b40 }; // cqi5: a TV da Premier
        cqi5_pagar(this, valor, 0)
    })
}

/// { 0x576630 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_table()`: 24 clubes ([0x9cf5c4]), cota 20.000
/// e a TV: 2.500.000 depois de 1999 (300.000 antes). Fonte 0x9c43e8, linha 0x1f5.
unsafe extern "thiscall" fn tabela_da_second(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c43e8, 0x1f5, 0x9cf5c4, 0x4e20, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0x2625a0 } else { 0x493e0 }, 0))
}

/// { 0x577f70 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_table()`: 24 clubes ([0x9cf5c8]), cota 10.000
/// e a TV: 1.000.000 depois de 1999 (200.000 antes). Fonte 0x9c4424, linha 0x1fa.
unsafe extern "thiscall" fn tabela_da_third(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4424, 0x1fa, 0x9cf5c8, 0x2710, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0xf4240 } else { 0x30d40 }, 0))
}

/// { 0x572bf0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_table()`: 24 clubes ([0x9cf5c0]), cota 70.000
/// e a TV pela divisão de onde o clube veio (+0x5b): depois de 1999, 5.877.777 aos que caíram da Premier (o registro
/// de [0x9cf5bc]), 4.099.999 aos que já estavam na First (o próprio registro, +4) e 4.099.999 aos que subiram da
/// Second ([0x9cf5c4]); antes, 1.000.000 a todos. Fonte 0x9c4374, linha 0x22d.
unsafe extern "thiscall" fn tabela_da_first(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4374, 0x22d, 0x9cf5c0, 0x11170, || {
        if cqi5_depois_de_1999() {
            cqi5_pagar(this, 0x59b011, cqi5_registro(0x9cf5bc)); // cqi5: a TV de quem caiu da Premier
            cqi5_pagar(this, 0x3e8fa0, cqi5_d(this + 4));
            cqi5_pagar(this, 0x3e8fa0, cqi5_registro(0x9cf5c4));
        } else {
            cqi5_pagar(this, 0xf4240, 0);
        }
    })
}

/// O comparador das quatro: a original aloca a tabela e paga os clubes (não roda duas vezes): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu
/// (senão 0) e as linhas da tabela com os clubes do plano, na ordem; cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqi5_comparar(nome: &str, this: usize, n: i16, g: usize, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqi5_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqi5_d(0xae23b4) + (i as usize) * 0x245;
        let comp = cqi5_d(clube + 0x57);
        if comp != 0 && cqi5_d(comp) == cqi5_d(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqi5_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cqi5_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqi5_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_tabela_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqi5_comparar("tabela_da_premier", this, 0x14, 0x9cf5bc, 0x3d090, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_second(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqi5_comparar("tabela_da_second", this, 0x18, 0x9cf5c4, 0x4e20, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqi5_comparar("tabela_da_third", this, 0x18, 0x9cf5c8, 0x2710, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_first(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%);
    cqi5_comparar("tabela_da_first", this, 0x18, 0x9cf5c0, 0x11170, || o(this))
}
