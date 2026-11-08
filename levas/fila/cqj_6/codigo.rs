// ---------- %%LEVA%%a leva: a tabela das três divisões francesas: os clubes (18 ou 20 na primeira, pelo ano), o fundo de prêmios e o dinheiro da TV, na segunda pela divisão de onde o clube veio (fra_first.cpp, fra_second.cpp, fra_third.cpp) ----------

unsafe fn cqj6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqj6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqj6_registro(g: usize) -> usize {
    cqj6_d(0xae23d0).wrapping_add((cqj6_d(g) as i32 as usize).wrapping_mul(0x6b))
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8).
unsafe fn cqj6_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, valor, filtro);
}

/// O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (pago pela
/// tabela no fim da liga, 0x682e10).
unsafe fn cqj6_fundo(this: usize, cota: i32) {
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
}

/// O corpo comum, com +0x3e já gravado: a tabela (malloc 0x944e46 de +0x3e × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
unsafe fn fra_first_fill_league_table(this: usize, arquivo: usize, linha: u32, g: usize) -> Option<bool> {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((cqj6_w(this + 0x3e) as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(arquivo, linha);
        return None;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqj6_d(0xae23b4);
    while (i as i32) < cqj6_d(0xae235c) as i32 {
        let comp = cqj6_d(clube + 0x57);
        if comp != 0 && cqj6_d(comp) == cqj6_d(g) {
            let linha = k; // cqj6: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqj6_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    Some(k as i8 as i16 == cqj6_w(this + 0x3e))
}

/// { 0x5bc2b0 } (thiscall, ret 0, fra_first.cpp): `FraFirst::create_table()`: 18 clubes na temporada 2001 (+0x40 =
/// 0x7d1) e 20 depois ([0x9cf5cc]), +0xa7 = −1; cheia, o fundo pela cota de 200.000 e a TV de 6.000.000 a cada clube;
/// 1 (0 sem encher). Fonte 0x9ccb5c, linha 0x1d3.
unsafe extern "thiscall" fn fra_first_create_table(this: usize) -> u32 {
    let n: i16 = if core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    core::ptr::write_unaligned((this + 0xa7) as *mut u16, 0xffff);
    if fra_first_fill_league_table(this, 0x9ccb5c, 0x1d3, 0x9cf5cc) != Some(true) {
        return 0;
    }
    crate::comparison::branch("cqj_6 tabela da primeira");
    cqj6_fundo(this, 0x30d40);
    cqj6_pagar(this, 0x5b8d80, 0); // cqj6: a TV da primeira
    1
}

/// { 0x5bef10 } (thiscall, ret 0, fra_second.cpp): `FraSecond::create_table()`: 20 clubes ([0x9cf5d0]); cheia, sem
/// fundo (+0x28 = 0, +0xe1 = 2: nada a pagar no fim) e a TV pela divisão de onde o clube veio (+0x5b): 2.000.000 aos
/// que caíram da primeira ([0x9cf5cc]), 1.500.000 aos que ficaram ([0x9cf5d0]) e 1.500.000 aos que subiram do
/// National ([0x9cf5d4]); 1. Fonte 0x9ccc0c, linha 0x171.
unsafe extern "thiscall" fn fra_second_create_table(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    if fra_first_fill_league_table(this, 0x9ccc0c, 0x171, 0x9cf5d0) != Some(true) {
        return 0;
    }
    crate::comparison::branch("cqj_6 tabela da segunda");
    core::ptr::write_unaligned((this + 0x28) as *mut i32, 0);
    *((this + 0xe1) as *mut u8) = 2;
    cqj6_pagar(this, 0x1e8480, cqj6_registro(0x9cf5cc)); // cqj6: a TV de quem caiu da primeira
    cqj6_pagar(this, 0x16e360, cqj6_registro(0x9cf5d0));
    cqj6_pagar(this, 0x16e360, cqj6_registro(0x9cf5d4));
    1
}

/// { 0x5c05d0 } (thiscall, ret 0, fra_third.cpp): `FraThird::create_table()`: 20 clubes ([0x9cf5d4]); cheia, o fundo
/// pela cota de 20.000 e a TV de 200.000 a cada clube; 1. Fonte 0x9ccc80, linha 0x15d.
unsafe extern "thiscall" fn fra_third_create_table(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    if fra_first_fill_league_table(this, 0x9ccc80, 0x15d, 0x9cf5d4) != Some(true) {
        return 0;
    }
    cqj6_fundo(this, 0x4e20);
    cqj6_pagar(this, 0x30d40, 0);
    1
}

/// O comparador das três: a original aloca a tabela e paga a TV (com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu, as linhas
/// com os clubes do plano na ordem e, cheia, +0xe1 e +0x28 como a rotina grava (`cota`: o fundo da cota e 1; None: 0
/// e 2).
unsafe fn fra_first_compare_league_table(nome: &str, this: usize, n: i16, g: usize, cota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqj6_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqj6_d(0xae23b4) + (i as usize) * 0x245;
        let comp = cqj6_d(clube + 0x57);
        if comp != 0 && cqj6_d(comp) == cqj6_d(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqj6_d(this + 0xb1);
    let mut igual = r == (plano.len() as i16 == n) as u32 && cqj6_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqj6_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let e1 = *((this + 0xe1) as *const u8);
            let f = core::ptr::read_unaligned((this + 0x28) as *const i32);
            igual = igual
                && match cota {
                    Some(c) => {
                        let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
                        e1 == 1 && f == premio(this, c)
                    }
                    None => e1 == 2 && f == 0,
                };
        }
    }
    SCOREBOARD.record(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_fra_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let n: i16 = if core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    fra_first_compare_league_table("fra_first_create_table", this, n, 0x9cf5cc, Some(0x30d40), || o(this))
}

unsafe extern "thiscall" fn comparar_fra_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    fra_first_compare_league_table("fra_second_create_table", this, 0x14, 0x9cf5d0, None, || o(this))
}

unsafe extern "thiscall" fn comparar_fra_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    fra_first_compare_league_table("fra_third_create_table", this, 0x14, 0x9cf5d4, Some(0x4e20), || o(this))
}
