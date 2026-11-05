// ---------- %%LEVA%%a leva: a tabela das ligas do Japão e da Austrália: os 16 clubes da J1 (o fundo de prêmios e a TV), os 12 da J2 (os que sobram saem da divisão) e os 13 da NSL (jap_j1.cpp, jap_j2.cpp, aus_nsl.cpp) ----------

unsafe fn cqm9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqm9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é da divisão do global `g`: a divisão do clube (+0x57) não é nula e o id do registro (+0) é o do global.
unsafe fn cqm9_da_divisao(clube: usize, g: usize) -> bool {
    let comp = cqm9_d(clube + 0x57);
    comp != 0 && cqm9_d(comp) == cqm9_d(g)
}

/// +0x3e = `n` e a tabela (malloc 0x944e46 de n × 0x3b, em +0xb1; sem memória: a caixa do fonte `arquivo` na linha
/// `linha`). Devolve a tabela (0 sem memória).
unsafe fn cqm9_alocar(this: usize, n: i16, arquivo: usize, linha: u32) -> usize {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
    }
    t
}

/// League::init_table_row (0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10). `girar` é 0 (o original: a linha
/// k); existe só para as mutações de cada liga. ponytail: a original empurra a linha no eax com bits de cima de sobra;
/// a 0x687430 só lê o byte.
unsafe fn cqm9_linha(this: usize, k: u8, n: i16, girar: u8, clube: usize) {
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    iniciar(this, (k as u32 + girar as u32) % n as u32, clube, 0, 0);
}

/// O laço do Japão: percorre todos os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles); cada clube da divisão do
/// global `g`, na ordem do banco, entra na linha seguinte enquanto houver linha; os que sobram perdem a divisão
/// (+0x57 = 0). Devolve 1 se a tabela encheu.
unsafe fn cqm9_japao(this: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm9_d(0xae23b4);
    while (i as i32) < cqm9_d(0xae235c) as i32 {
        if cqm9_da_divisao(clube, g) {
            if (k as i8 as i16) < cqm9_w(this + 0x3e) {
                cqm9_linha(this, k, cqm9_w(this + 0x3e), girar, clube);
                k = k.wrapping_add(1);
            } else {
                core::ptr::write_unaligned((clube + 0x57) as *mut usize, 0);
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqm9_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x6634e0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_table()`: a J1 tem 16 clubes (+0x57 = [0x9cf65c]; os que
/// sobram perdem a divisão); cheia, o fundo de prêmios com a cota 100.000 (League::calc_prize_fund 0x682f70: push
/// 0x186a0; mov ecx, edi -> thiscall, ret 4) em +0x28, +0xe1 = 1, a TV 3.000.000 a cada clube (League::pay_tv_rights
/// 0x683010: push 0; push 0x2dc6c0; mov ecx, edi -> thiscall, ret 8; manda a notícia da TV) e 1. Fonte 0x9eabc8, linha
/// 0x148.
unsafe extern "thiscall" fn cqm9_tabela_da_j1(this: usize) -> u32 {
    if cqm9_alocar(this, 0x10, 0x9eabc8, 0x148) == 0 {
        return 0;
    }
    if cqm9_japao(this, 0x9cf65c, 0, "cqm_9 j1 cheia") == 0 {
        return 0;
    }
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    let f = premio(this, 0x186a0);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar(this, 0x2dc6c0, 0);
    1
}

/// { 0x665030 } (thiscall, ret 0, jap_j2.cpp): `JapJ2::create_table()`: a J2 tem 12 clubes (+0x57 = [0x9cf768]; os que
/// sobram perdem a divisão), sem fundo de prêmios nem TV. Fonte 0x9eaedc, linha 0x13a. Devolve 1 se encheu.
unsafe extern "thiscall" fn cqm9_tabela_da_j2(this: usize) -> u32 {
    if cqm9_alocar(this, 0xc, 0x9eaedc, 0x13a) == 0 {
        return 0;
    }
    cqm9_japao(this, 0x9cf768, 0, "cqm_9 j2 cheia")
}

/// { 0x411b70 } (thiscall, ret 0, aus_nsl.cpp): `AusNsl::create_table()`: a NSL tem 13 clubes (+0x57 = [0x9cf89c]), na
/// ordem do banco, até encher (o contador num int, comparado no word); sem encher: a caixa da linha 0x252 e 0; cheia: 1,
/// sem fundo de prêmios nem TV. Fonte 0x987c20 (a caixa de memória na linha 0x241).
unsafe extern "thiscall" fn cqm9_tabela_da_nsl(this: usize) -> u32 {
    if cqm9_alocar(this, 0xd, 0x987c20, 0x241) == 0 {
        return 0;
    }
    let mut k: u32 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm9_d(0xae23b4);
    while (i as i32) < cqm9_d(0xae235c) as i32 {
        if (k as i16) >= cqm9_w(this + 0x3e) {
            break;
        }
        if cqm9_da_divisao(clube, 0x9cf89c) {
            cqm9_linha(this, k as u8, cqm9_w(this + 0x3e), 0, clube); // cqm9: a linha da NSL
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i16 != cqm9_w(this + 0x3e) {
        aviso_em(0x987c20, 0x252);
        return 0;
    }
    crate::comparacao::ramo("cqm_9 nsl cheia");
    1
}

/// O comparador dos três: a original aloca a tabela (e a J1 paga a TV com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (todos). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0), as
/// linhas da tabela (+0 o clube, 0x3b bytes) com os primeiros `n` do plano, na ordem; no Japão (`limpar`), os que
/// sobram do plano com a divisão zerada; na J1 cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqm9_comparar(nome: &str, this: usize, n: i16, g: usize, limpar: bool, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqm9_d(0xae235c) as i32 {
        let clube = cqm9_d(0xae23b4) + (i as usize) * 0x245;
        if cqm9_da_divisao(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqm9_d(this + 0xb1);
    let cabem = plano.len().min(n as usize);
    let esperado = (plano.len() >= n as usize) as u32;
    let mut igual = r == esperado && cqm9_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..cabem).map(|k| cqm9_d(t + k * 0x3b)).collect();
        igual = achou[..] == plano[..cabem];
        if limpar {
            igual = igual && plano[cabem..].iter().all(|&c| cqm9_d(c + 0x57) == 0);
        }
        if r == 1 && cota != 0 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cqm9_tabela_da_j1(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqm9_comparar("cqm9_tabela_da_j1", this, 0x10, 0x9cf65c, true, 0x186a0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm9_tabela_da_j2(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqm9_comparar("cqm9_tabela_da_j2", this, 0xc, 0x9cf768, true, 0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqm9_tabela_da_nsl(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqm9_comparar("cqm9_tabela_da_nsl", this, 0xd, 0x9cf89c, false, 0, || o(this))
}
