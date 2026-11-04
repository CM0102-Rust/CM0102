// ---------- %%LEVA%%a leva: a tabela das divisões italianas (Série A, B, C1/A e C1/B): os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqj4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqj4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
unsafe fn cqj4_encher(this: usize, n: i16, arquivo: usize, linha: u32, g: usize) -> Option<bool> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqj4_d(0xae23b4);
    while (i as i32) < cqj4_d(0xae235c) as i32 {
        let comp = cqj4_d(clube + 0x57);
        if comp != 0 && cqj4_d(comp) == cqj4_d(g) {
            let linha = k; // cqj4: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0); // cqj4: as linhas sem marca
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqj4_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    Some(k as i8 as i16 == cqj4_w(this + 0x3e))
}

/// O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (o fundo
/// é pago pela tabela no fim da liga, 0x682e10).
unsafe fn cqj4_fundo(this: usize, cota: i32) {
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
}

/// { 0x641510 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::create_table()`: 18 clubes ([0x9cf570]); cheia, o fundo
/// pela cota de 250.000 e a TV de 5.000.000 a cada clube (League::pay_tv_rights 0x683010: thiscall(liga, valor, 0),
/// ret 8); devolve 1 (0 sem encher). Fonte 0x9ea2a8, linha 0x253.
unsafe extern "thiscall" fn tabela_da_serie_a(this: usize) -> u32 {
    match cqj4_encher(this, 0x12, 0x9ea2a8, 0x253, 0x9cf570) {
        Some(true) => {}
        _ => return 0,
    }
    crate::comparacao::ramo("cqj_4 tabela da serie a");
    cqj4_fundo(this, 0x3d090); // cqj4: a cota da Série A
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, 0x4c4b40, 0); // cqj4: a TV da Série A
    1
}

/// { 0x646780 } (thiscall, ret 0, ita_ser_b.cpp): `ItaSerB::create_table()`: 20 clubes ([0x9cf574]); cheia, o fundo
/// pela cota de 70.000 (sem TV) e 1. Fonte 0x9ea384, linha 0x19d.
unsafe extern "thiscall" fn tabela_da_serie_b(this: usize) -> u32 {
    match cqj4_encher(this, 0x14, 0x9ea384, 0x19d, 0x9cf574) {
        Some(true) => {}
        _ => return 0,
    }
    cqj4_fundo(this, 0x11170);
    1
}

/// { 0x64b670 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::create_table()`: 18 clubes ([0x9cf578]), sem fundo
/// nem TV; devolve se encheu. Fonte 0x9ea474, linha 0x1bd.
unsafe extern "thiscall" fn tabela_da_c1a(this: usize) -> u32 {
    cqj4_encher(this, 0x12, 0x9ea474, 0x1bd, 0x9cf578).unwrap_or(false) as u32
}

/// { 0x64f820 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_table()`: 18 clubes ([0x9cf57c]); como a
/// C1/A. Fonte 0x9ea558, linha 0x1be.
unsafe extern "thiscall" fn tabela_da_c1b(this: usize) -> u32 {
    cqj4_encher(this, 0x12, 0x9ea558, 0x1be, 0x9cf57c).unwrap_or(false) as u32
}

/// O comparador das quatro: a original aloca a tabela (e a da A paga a TV, com notícia): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu,
/// as linhas com os clubes do plano na ordem e, com `cota`, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqj4_comparar(nome: &str, this: usize, n: i16, g: usize, cota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqj4_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqj4_d(0xae23b4) + (i as usize) * 0x245;
        let comp = cqj4_d(clube + 0x57);
        if comp != 0 && cqj4_d(comp) == cqj4_d(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqj4_d(this + 0xb1);
    let mut igual = r == (plano.len() as i16 == n) as u32 && cqj4_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqj4_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if let (1, Some(c)) = (r, cota) {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, c);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_tabela_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqj4_comparar("tabela_da_serie_a", this, 0x12, 0x9cf570, Some(0x3d090), || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqj4_comparar("tabela_da_serie_b", this, 0x14, 0x9cf574, Some(0x11170), || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_c1a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqj4_comparar("tabela_da_c1a", this, 0x12, 0x9cf578, None, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_c1b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%);
    cqj4_comparar("tabela_da_c1b", this, 0x12, 0x9cf57c, None, || o(this))
}
