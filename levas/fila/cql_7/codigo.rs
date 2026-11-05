// ---------- %%LEVA%%a leva: a tabela das três divisões nacionais do Brasil: os 28 clubes da Série A (e o dinheiro da TV), os 28 da Série B e o sorteio dos 56 da Série C (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------

unsafe fn cql7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cql7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube está na competição do global `g`: o ponteiro do clube em `campo` (+0x57 a divisão nacional) não é nulo e o
/// id do registro (+0) é o do global.
unsafe fn cql7_da_divisao(clube: usize, campo: usize, g: usize) -> bool {
    let comp = cql7_d(clube + campo);
    comp != 0 && cql7_d(comp) == cql7_d(g)
}

/// O corpo comum da Série A e da Série B (o mesmo molde dos estaduais): +0x3e = `n` clubes; a tabela (malloc 0x944e46
/// de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245
/// bytes, [0xae235c] deles) da competição do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada divisão.
unsafe fn cql7_tabela(this: usize, n: i16, arquivo: usize, linha: u32, campo: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
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
    let mut clube = cql7_d(0xae23b4);
    while (i as i32) < cql7_d(0xae235c) as i32 {
        if cql7_da_divisao(clube, campo, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cql7_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cql7_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x42a480 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_table()`: a Série A tem 28 clubes (os de
/// divisão nacional +0x57 = [0x9cf9ec]); cheia, a TV paga 150.000 a cada clube (League::pay_tv_rights 0x683010:
/// push 0; push 0x249f0; mov ecx, edi -> thiscall(liga, valor, filtro), ret 8; manda a notícia da TV) e devolve 1.
/// Fonte 0x98aa30, linha 0x1a1.
unsafe extern "thiscall" fn cql7_tabela_da_serie_a(this: usize) -> u32 {
    if cql7_tabela(this, 0x1c, 0x98aa30, 0x1a1, 0x57, 0x9cf9ec, 0, "cql_7 serie a cheia") == 0 {
        return 0;
    }
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, 0x249f0, 0);
    1
}

/// { 0x42ca70 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::create_table()`: a Série B tem 28 clubes (+0x57 =
/// [0x9cf9e0]), sem dinheiro da TV. Fonte 0x98ab64, linha 0x1aa.
unsafe extern "thiscall" fn cql7_tabela_da_serie_b(this: usize) -> u32 {
    cql7_tabela(this, 0x1c, 0x98ab64, 0x1aa, 0x57, 0x9cf9e0, 0, "cql_7 serie b cheia")
}

/// { 0x42dc90 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::create_table()`: +0x3e = 7; a lista dos
/// participantes (+0xee) refeita (a velha liberada, 0x9452ca push; add esp -> cdecl; a nova com 0xe0 bytes = 56
/// ponteiros; sem memória: a caixa da linha 0x18f e devolve 1, como a original); a tabela (+0xb1, 7 × 0x3b; sem
/// memória: a caixa da linha 0x196 e 0); a lista recebe, na ordem do banco, até 56 clubes cuja divisão (+0x57) é a
/// própria Série C (o registro em [this+4]); a lista é embaralhada (comp_util_shuffle_pointer_array 0x4afb40: push
/// conta (word); push lista; add esp, 8 -> cdecl; 3 × conta trocas sorteadas) e os 7 primeiros do sorteio entram nas
/// linhas 0..6 (0x687430, a linha num dword limpo). Fonte 0x98aba4. Devolve 1.
unsafe extern "thiscall" fn cql7_tabela_da_serie_c(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let embaralhar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4afb40usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 7);
    let velha = cql7_d(this + 0xee);
    if velha != 0 {
        soltar(velha);
    }
    let lista = alocar(0xe0);
    core::ptr::write_unaligned((this + 0xee) as *mut usize, lista);
    if lista == 0 {
        aviso_em(0x98aba4, 0x18f);
        return 1;
    }
    let t = alocar((cql7_w(this + 0x3e) as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(0x98aba4, 0x196);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut clube = cql7_d(0xae23b4);
    while i < cql7_d(0xae235c) as i32 {
        if (k as i8) >= 0x38 {
            break;
        }
        if cql7_d(clube + 0x57) == cql7_d(this + 4) {
            core::ptr::write_unaligned((cql7_d(this + 0xee) + (k as i8 as isize as usize) * 4) as *mut usize, clube);
            k = k.wrapping_add(1);
        }
        i += 1;
        clube = clube.wrapping_add(0x245);
    }
    crate::comparacao::ramo("cql_7 serie c sorteio");
    embaralhar(cql7_d(this + 0xee), k as i8 as i32); // cql7: o sorteio dos clubes da Série C
    for j in 0..7usize {
        iniciar(this, j as u32, cql7_d(cql7_d(this + 0xee) + j * 4), 0, 0);
    }
    1
}

/// Os clubes da competição do global `g` (ponteiro em `campo`), na ordem do banco, até `n`.
unsafe fn cql7_plano(n: usize, campo: usize, g: usize) -> Vec<usize> {
    let mut plano = Vec::new();
    let mut i: i32 = 0;
    while i < cql7_d(0xae235c) as i32 && plano.len() < n {
        let clube = cql7_d(0xae23b4) + (i as usize) * 0x245;
        if cql7_da_divisao(clube, campo, g) {
            plano.push(clube);
        }
        i += 1;
    }
    plano
}

/// O comparador da Série A e da Série B: a original aloca a tabela (e na A paga a TV com notícia): só ela. O plano,
/// feito antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano
/// encheu (senão 0) e as linhas da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn cql7_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let plano = cql7_plano(n as usize, 0x57, g);
    let r = original();
    let t = cql7_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cql7_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cql7_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_cql7_tabela_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cql7_comparar("cql7_tabela_da_serie_a", this, 0x1c, 0x9cf9ec, || o(this))
}

unsafe extern "thiscall" fn comparar_cql7_tabela_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cql7_comparar("cql7_tabela_da_serie_b", this, 0x1c, 0x9cf9e0, || o(this))
}

/// O comparador da Série C: a original libera e aloca e sorteia: só ela. O plano, feito antes: os clubes cuja divisão é
/// [this+4], na ordem do banco, até 56. Depois: a devolução 1, +0x3e = 7, a lista (+0xee) com os clubes do plano em
/// alguma ordem (o sorteio) e as linhas 0..6 da tabela com os 7 primeiros da lista.
unsafe extern "thiscall" fn comparar_cql7_tabela_da_serie_c(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let mut plano = Vec::new();
    let mut i: i32 = 0;
    while i < cql7_d(0xae235c) as i32 && plano.len() < 0x38 {
        let clube = cql7_d(0xae23b4) + (i as usize) * 0x245;
        if cql7_d(clube + 0x57) == cql7_d(this + 4) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = o(this);
    let (lista, t) = (cql7_d(this + 0xee), cql7_d(this + 0xb1));
    let mut igual = r == 1 && cql7_w(this + 0x3e) == 7 && lista != 0 && t != 0 && plano.len() >= 7;
    let mut sorteados: Vec<usize> = Vec::new();
    if igual {
        sorteados = (0..plano.len()).map(|k| cql7_d(lista + k * 4)).collect();
        let linhas_ok = (0..7usize).all(|j| cql7_d(t + j * 0x3b) == sorteados[j]);
        let (mut a, mut b) = (sorteados.clone(), plano.clone());
        a.sort_unstable();
        b.sort_unstable();
        igual = linhas_ok && a == b;
    }
    PLACAR.registrar(igual, || format!("cql7_tabela_da_serie_c({:#x}) original={} plano={:x?} lista={:x?}", this, r, plano, sorteados));
    r
}
