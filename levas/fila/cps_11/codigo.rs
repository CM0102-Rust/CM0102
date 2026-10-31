// ---------- %%LEVA%%a leva: o calendário guardado: achar a cópia guardada de um jogo, guardar um jogo e tirar um jogo (fix_man.cpp) ----------

unsafe fn cps11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps11_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O bloco do ano do jogo `f` (+0x28, palavra) na corrente do calendário a partir de `fm` (o laço comum das 0x5ac7a0,
/// 0x5ab230 e 0x5abc30): o bloco seguinte (+0x112c) de outro ano é levado ao ano pedido (0x5aa2d0 set_year); sem
/// seguinte, um bloco novo (0x944cf1, 0x1184 bytes; 0x5a9cf0 com o ano, o bloco de antes e 0) entra na corrente. 0 se
/// faltou memória (a corrente fica com 0 em +0x112c, como na original).
unsafe fn cps11_bloco_do_ano(fm: usize, f: usize) -> usize {
    let mudar_ano: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x5a9cf0usize);
    let mut b = fm;
    loop {
        let ano = cps11_w(f + 0x28);
        if ano == cps11_w(b + 0x1130) {
            return b;
        }
        let prox = cps11_d(b + 0x112c);
        if prox != 0 {
            if cps11_w(prox + 0x1130) != ano {
                // ponytail: a original empurra o eax com o ano na palavra baixa e sobra de registrador em cima; a
                // 0x5aa2d0 compara a palavra (o caminho só roda na virada do ano do calendário)
                mudar_ano(prox, ano as u32);
            }
        } else {
            let p = novo(0x1184);
            // o ecx da original era o +0x112c nulo: o ano vai limpo
            let r = if p != 0 { construir(p, ano as u32, b, 0) } else { 0 };
            core::ptr::write_unaligned((b + 0x112c) as *mut u32, r as u32);
            if r == 0 {
                return 0;
            }
        }
        b = cps11_d(b + 0x112c);
    }
}

/// A lista do dia, fatia (+0x3f) e nação-chave do jogo `f` no bloco `b`: [[b + (dia +0x2a × 3 + fatia) × 4] + nação × 4],
/// a nação pela 0x48dd70 (competição +0x14, o jogo), com sinal.
unsafe fn cps11_lista_no_bloco(b: usize, f: usize) -> usize {
    let nacao: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x48dd70usize);
    let n = nacao(cps11_d(f + 0x14), f) as u8 as i8 as isize;
    let k = cps11_w(f + 0x2a) as usize * 3 + *((f + 0x3f) as *const u8) as usize;
    cps11_d((cps11_d(b + k * 4) as isize + n * 4) as usize)
}

/// O próximo item da lista (0x68d1b0: o cursor +0xa anda).
unsafe fn cps11_proximo(l: usize) -> usize {
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    prox(l)
}

/// { 0x5ac7a0 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::find_stored_fixture(jogo)`: a cópia guardada no
/// calendário cujos 0x4f bytes são iguais aos do jogo, na lista do dia, fatia e nação-chave dele (o bloco do ano pela
/// corrente; o cursor da lista volta ao começo e anda). Jogo nulo ou nenhum igual: 0; sem memória para o bloco do ano:
/// a caixa (linha 0x7cf) e 0.
unsafe extern "thiscall" fn jogo_guardado_igual(this: usize, f: usize) -> usize {
    if f == 0 {
        return 0;
    }
    let b = cps11_bloco_do_ano(this, f);
    if b == 0 {
        aviso_em(0x9cc208, 0x7cf);
        return 0;
    }
    let l = cps11_lista_no_bloco(b, f);
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cps11_d(l + 2) as u32);
    let alvo = core::slice::from_raw_parts(f as *const u8, 0x4f);
    let mut item = cps11_proximo(l);
    while item != 0 {
        if core::slice::from_raw_parts(item as *const u8, 0x4f) == alvo {
            crate::comparacao::ramo("cps_11 achou o jogo guardado");
            return item; // cps11: a cópia guardada
        }
        item = cps11_proximo(l);
    }
    0
}

/// { 0x5ab230 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::insert_fixture(jogo)`: acrescenta uma cópia dos 0x4f
/// bytes do jogo (0x68d0d0) à lista do dia, fatia e nação-chave dele no bloco do ano (pela corrente). Sem memória para o
/// bloco: a caixa (linha 0x3ff); a cópia que falha: a caixa (linha 0x3eb).
unsafe extern "thiscall" fn guardar_jogo_no_calendario(this: usize, f: usize) {
    let b = cps11_bloco_do_ano(this, f);
    if b == 0 {
        return aviso_em(0x9cc208, 0x3ff);
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let l = cps11_lista_no_bloco(b, f);
    if copiar(l, f, 0x4f) == 0 { // cps11: guardar
        return aviso_em(0x9cc208, 0x3eb);
    }
    crate::comparacao::ramo("cps_11 jogo guardado");
}

/// { 0x5abc30 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::remove_fixture(jogo)`: tira da lista do dia, fatia e
/// nação-chave do jogo (no bloco do ano, pela corrente) o primeiro item com os mesmos bytes (0x68d1f0). Devolve 1 se
/// tirou, 0 se não; sem memória para o bloco: a caixa (linha 0x5ef) e 0.
unsafe extern "thiscall" fn tirar_jogo_do_calendario(this: usize, f: usize) -> u32 {
    let b = cps11_bloco_do_ano(this, f);
    if b == 0 {
        aviso_em(0x9cc208, 0x5ef);
        return 0;
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let r = tirar(cps11_lista_no_bloco(b, f), f); // cps11: tirar
    if r != 0 {
        crate::comparacao::ramo("cps_11 jogo tirado");
    }
    (r != 0) as u32
}

/// A lista do jogo no calendário, achada antes da original (o laço da corrente faz o mesmo que ela faria: depois dele
/// a original acha o bloco do ano de primeira); 0 sem memória.
unsafe fn cps11_lista_antes(fm: usize, f: usize) -> usize {
    let b = cps11_bloco_do_ano(fm, f);
    if b == 0 { 0 } else { cps11_lista_no_bloco(b, f) }
}

/// Comparador: a original e a cópia só andam o cursor da lista (e a primeira pode criar o bloco do ano, que a segunda
/// acha pronto): pela devolução.
unsafe extern "thiscall" fn comparar_jogo_guardado_igual(this: usize, f: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, f);
    let n = jogo_guardado_igual(this, f);
    PLACAR.registrar(r == n, || format!("jogo_guardado_igual(jogo {:x}) original={:x} rust={:x}", f, r, n));
    r
}

/// Comparador: acrescenta à lista (não roda duas vezes): a lista é achada antes pela cópia, a original guarda e a conta
/// tem de subir 1 com a cauda nova igual aos 0x4f bytes do jogo.
unsafe extern "thiscall" fn comparar_guardar_jogo_no_calendario(this: usize, f: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let l = cps11_lista_antes(this, f);
    if l == 0 {
        return o(this, f); // ponytail: sem memória para o bloco, só a original (a caixa)
    }
    let conta = cps11_w(l);
    o(this, f);
    let cauda = cps11_d(l + 6);
    let igual = cps11_w(l) == conta.wrapping_add(1) && cauda != 0
        && core::slice::from_raw_parts(cps11_d(cauda) as *const u8, 0x4f) == core::slice::from_raw_parts(f as *const u8, 0x4f);
    PLACAR.registrar(igual, || format!("guardar_jogo_no_calendario(jogo {:x}) conta antes={} depois={}", f, conta, cps11_w(l)));
}

/// Comparador: tira da lista (não roda duas vezes): antes da original, a cópia acha a lista e vê se algum item tem os
/// bytes do jogo (o tamanho guardado em +4, como a 0x68d1f0); a original tira e devolve: a devolução e a conta.
unsafe extern "thiscall" fn comparar_tirar_jogo_do_calendario(this: usize, f: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let l = cps11_lista_antes(this, f);
    if l == 0 {
        return o(this, f); // ponytail: sem memória para o bloco, só a original (a caixa)
    }
    let conta = cps11_w(l);
    let mut no = cps11_d(l + 2);
    let mut tem = false;
    while no != 0 && !tem {
        let n = *((no + 4) as *const i16) as isize as usize;
        tem = core::slice::from_raw_parts(cps11_d(no) as *const u8, n) == core::slice::from_raw_parts(f as *const u8, n);
        no = cps11_d(no + 6);
    }
    let r = o(this, f);
    let esperado = tem as u32;
    let igual = r == esperado && cps11_w(l) == conta.wrapping_sub(esperado as u16);
    PLACAR.registrar(igual, || format!("tirar_jogo_do_calendario(jogo {:x}) original={} esperado={} conta antes={} depois={}",
        f, r, esperado, conta, cps11_w(l)));
    r
}
