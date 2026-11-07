// ---------- 268a leva: o período do empréstimo na Inglaterra e as regras de empréstimo da Escócia, da Itália, da Irlanda e da Espanha (england_rules.cpp, scotland_rules.cpp, italy_rules.cpp, ireland_rules.cpp, spain_rules.cpp) ----------

unsafe fn cqe6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqe6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe6_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de um contrato: +4 é o índice em [0xae23b4] (registros de 0x245 bytes).
unsafe fn cqe6_clube_do_contrato(c: usize) -> usize {
    cqe6_d(0xae23b4).wrapping_add(cqe6_d(c + 4).wrapping_mul(0x245))
}

/// O contrato principal da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em
/// [0xae1080] como a original faz.
unsafe fn cqe6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqe6_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O começo comum das regras escocesa, irlandesa e espanhola: sem contrato do jogador (ou sem clube do contrato),
/// `None` (pode); senão o clube dono pelo contrato consultado de novo (0 se a segunda consulta não achar).
unsafe fn cqe6_dono(jogador: usize) -> Option<usize> {
    let c = cqe6_contrato(jogador);
    if c == 0 || cqe6_clube_do_contrato(c) == 0 {
        return None;
    }
    let c2 = cqe6_contrato(jogador);
    Some(if c2 != 0 { cqe6_clube_do_contrato(c2) } else { 0 })
}

/// A recusa "fora da janela" (motivo 9), com o adjetivo da nação das regras (+0xa).
unsafe fn cqe6_fora_da_janela(this: usize, msg: usize, motivo: *mut u16) -> u32 {
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nação; add esp, 0x10 -> cdecl
        let adjetivo: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x6181d0usize);
        // 0x66f4e0: push ...; push formato; push destino; add esp, 0x14 -> cdecl variádica
        let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
        let mut buf = [0u8; 100];
        adjetivo(cqe6_d(this + 0xa), buf.as_mut_ptr(), 100, 2);
        texto(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !motivo.is_null() {
        *motivo = 9;
    }
    0
}

/// Uma recusa com o texto `formato` montado com o byte +0x37 (duas vezes) e o nome em +4 do registro `r`.
unsafe fn cqe6_recusa(msg: usize, motivo: *mut u16, formato: usize, r: usize, m: u16) -> u32 {
    if msg != 0 {
        let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
        let g = cqe6_b(r + 0x37) as i8 as i32;
        texto(msg, formato, g, g, r + 4);
    }
    if !motivo.is_null() {
        *motivo = m;
    }
    0
}

/// { 0x579c70 } (thiscall, ret 0x1c, england_rules.cpp): `ENGLAND_RULES::get_loan_period(clube, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`, pelo período do empréstimo (0x8d0590). Depois de 19 de junho (só o dia do
/// ano, 0x549f70 com 1998) e com o jogador abaixo de 24 anos (+0x18): o período da regra base (0x8e8bd0). Senão começa
/// no mês seguinte e acaba três meses depois.
unsafe extern "thiscall" fn cqe6_periodo_inglaterra(this: usize, clube: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    // 0x549f70: push -1; push 0x7ce; push 5; push 0x13; lea ecx, local -> thiscall, ret 0x10
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    data_de(d.as_mut_ptr() as usize, 0x13, 5, 0x7ce, -1);
    if cqe6_h(0xae2c90) > cqe6_h(d.as_ptr() as usize) && (cqe6_b(jogador + 0x18) as i8) < 0x18 { // cqe6: abaixo de 24 anos
        crate::comparacao::ramo("cqe_6 inglaterra periodo da base");
        // 0x8e8bd0: os mesmos 7 argumentos empurrados de novo; mov ecx, esi (this) -> thiscall, ret 0x1c
        let base: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize) = core::mem::transmute(0x8e8bd0usize);
        base(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador);
        return;
    }
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0 (o mês de hoje)
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    *ini_mes = (mes(0xae2c90) as i8).wrapping_add(1); // cqe6: o mês seguinte (Inglaterra)
    *ini_ano = cqe6_h(0xae2c92);
    if *ini_mes > 0xb {
        *ini_mes = (*ini_mes).wrapping_sub(0xc);
        *ini_ano = (*ini_ano).wrapping_add(1);
    }
    crate::comparacao::ramo("cqe_6 inglaterra tres meses");
    *fim_mes = (*ini_mes).wrapping_add(3); // cqe6: três meses
    *fim_ano = *ini_ano;
    if *fim_mes > 0xb {
        *fim_mes = (*fim_mes).wrapping_sub(0xc);
        *fim_ano = (*fim_ano).wrapping_add(1);
    }
}

/// { 0x7f6650 } (thiscall, ret 0x10, scotland_rules.cpp): `SCOTLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: com contrato, o clube que pede e o dono na mesma divisão escocesa ([0x9cf5dc]) não podem (0x1d).
unsafe extern "thiscall" fn cqe6_pode_emprestar_escocia(_this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    let comp = cqe6_d(clube + 0x57);
    if comp == cqe6_d(dono + 0x57) && comp != 0 && cqe6_d(comp) == cqe6_d(0x9cf5dc) {
        crate::comparacao::ramo("cqe_6 escocia mesma divisao");
        let r = cqe6_d(0xae23d0).wrapping_add(cqe6_d(0x9cf5dc).wrapping_mul(0x6b));
        return cqe6_recusa(msg, motivo, 0x9c4590, r, 0x1d);
    }
    1
}

/// { 0x7f6530 } (thiscall, ret 0x1c, scotland_rules.cpp): `SCOTLAND_RULES::get_loan_period(a1, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`: começa no mês seguinte; o fim é o mês do fim de temporada da nação a1
/// (0x4d45f0) para quem tem menos de 22 anos (+0x18), neste ano se o começo vem antes dele, senão no ano seguinte;
/// para os outros, três meses depois do começo.
unsafe extern "thiscall" fn cqe6_periodo_escocia(_this: usize, a1: usize, _a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    *ini_mes = (mes(0xae2c90) as i8).wrapping_add(1);
    *ini_ano = cqe6_h(0xae2c92);
    if *ini_mes > 0xb {
        *ini_mes = (*ini_mes).wrapping_sub(0xc);
        *ini_ano = (*ini_ano).wrapping_add(1);
    }
    // 0x4d45f0: push a1; mov ecx, [0xae1050] -> thiscall, ret 4 (o mês do fim da temporada)
    let mes_do_fim: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x4d45f0usize);
    let e = mes_do_fim(cqe6_d(0xae1050), a1) as i8;
    *fim_mes = e;
    if (cqe6_b(jogador + 0x18) as i8) < 0x16 {
        crate::comparacao::ramo("cqe_6 escocia ate o fim da temporada");
        if *ini_mes < e {
            *fim_ano = *ini_ano;
        } else {
            *fim_ano = cqe6_h(0xae2c92).wrapping_add(1);
        }
        return;
    }
    *fim_mes = (*ini_mes).wrapping_add(3);
    *fim_ano = *ini_ano;
    if *fim_mes > 0xb {
        *fim_mes = (*fim_mes).wrapping_sub(0xc);
        *fim_ano = (*fim_ano).wrapping_add(1);
    }
}

/// { 0x661770 } (thiscall, ret 0x10, italy_rules.cpp): `ITALY_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora das duas primeiras janelas (0x8e8c40 com [this+4] e [this+4] + 0xc): motivo 9. Um clube de uma das
/// cinco divisões ([0x9cf578] a [0x9cf588]) com quatro jogadores emprestados de outros clubes (ids abaixo de
/// [0xae2364], contrato de empréstimo 0x4cf6d0) não pode (0x25).
unsafe extern "thiscall" fn cqe6_pode_emprestar_italia(this: usize, clube: usize, _jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8c40: push janela; push 0xae2c90; ecx = this -> thiscall, ret 8
    let na_janela: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8e8c40usize);
    if na_janela(this, 0xae2c90, cqe6_d(this + 4)) == 0 && na_janela(this, 0xae2c90, cqe6_d(this + 4) + 0xc) == 0 {
        crate::comparacao::ramo("cqe_6 italia fora da janela");
        return cqe6_fora_da_janela(this, msg, motivo);
    }
    let comp = cqe6_d(clube + 0x57);
    if comp == 0 {
        return 1;
    }
    let id = cqe6_d(comp);
    if ![0x9cf578usize, 0x9cf57c, 0x9cf580, 0x9cf584, 0x9cf588].iter().any(|&g| cqe6_d(g) == id) {
        return 1;
    }
    // 0x4cf6d0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = cqe6_d(clube + 0xd7 + 4 * k);
        if p == 0 || cqe6_d(p) as i32 >= cqe6_d(0xae2364) as i32 || emprestimo(cqe6_d(0xae1050), p) == 0 {
            continue;
        }
        let cp = cqe6_contrato(p);
        let de = if cp != 0 { cqe6_clube_do_contrato(cp) } else { 0 };
        if de != clube {
            n = n.wrapping_add(1);
        }
    }
    if n >= 4 {
        crate::comparacao::ramo("cqe_6 italia quatro emprestados");
        if msg != 0 {
            let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
            let g = cqe6_b(clube + 0x52) as i8 as i32;
            texto(msg, 0x9ea9dc, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x25;
        }
        return 0;
    }
    1
}

/// { 0x63c3d0 } (thiscall, ret 0x10, ireland_rules.cpp): `IRELAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: cinco empréstimos na temporada (a conta de 16 bytes em [[0xde1f54] + 0x91c]): 0x1e; com contrato, o
/// clube que pede e o dono na mesma divisão (+0x57, sem conferir o nulo, como a original): 0x1d, com o nome da nação.
unsafe extern "thiscall" fn cqe6_pode_emprestar_irlanda(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let conta = cqe6_d(cqe6_d(0xde1f54) + 0x91c).wrapping_add(cqe6_d(clube).wrapping_mul(16));
    if cqe6_b(conta) as i8 >= 5 {
        crate::comparacao::ramo("cqe_6 irlanda cinco na temporada");
        if msg != 0 {
            let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
            let g = cqe6_b(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c4534, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x1e;
        }
        return 0;
    }
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if cqe6_d(clube + 0x57) == cqe6_d(dono + 0x57) {
        crate::comparacao::ramo("cqe_6 irlanda mesma divisao");
        return cqe6_recusa(msg, motivo, 0x9ea168, cqe6_d(this + 0xa), 0x1d);
    }
    1
}

/// { 0x859630 } (thiscall, ret 0x10, spain_rules.cpp): `SPAIN_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora da janela (0x8e8e50): 9; com contrato, entre clubes da mesma divisão (+0x57) depois de mais de
/// [0x968990] jogos do jogador na liga (0x7c1e30 da ficha +0x61, tipo 2, 1, 6,5): 0x1f.
unsafe extern "thiscall" fn cqe6_pode_emprestar_espanha(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8
    let janela: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8e8e50usize);
    if janela(this, 0xae2c90, 0) == 0 {
        crate::comparacao::ramo("cqe_6 espanha fora da janela");
        return cqe6_fora_da_janela(this, msg, motivo);
    }
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if cqe6_d(clube + 0x57) == cqe6_d(dono + 0x57) {
        // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall, ret 0x10 (devolve no st0)
        let jogos: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
        let v = jogos(cqe6_d(0xb74350), cqe6_d(jogador + 0x61), 2, 1, 6.5);
        if v > f32_em(0x968990) as f64 {
            crate::comparacao::ramo("cqe_6 espanha mesma divisao");
            return cqe6_recusa(msg, motivo, 0xa82b00, cqe6_d(this + 0xa), 0x1f);
        }
    }
    1
}

/// O texto da mensagem (até o zero, no máximo 0x200 bytes).
unsafe fn cqe6_texto(msg: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if msg == 0 {
        return v;
    }
    for k in 0..0x200usize {
        let b = cqe6_b(msg + k);
        v.push(b);
        if b == 0 {
            break;
        }
    }
    v
}

/// Comparador comum (pode emprestar): lado a lado na devolução, em [0xae1080] e no motivo; a mensagem, que só é
/// escrita, é conferida pelo texto (antes da cópia o primeiro byte vai a zero, e no fim volta o texto da original).
unsafe fn cqe6_comparar_pode(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, nome: &str, this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || {
        let x = o(this, clube, jogador, msg, motivo);
        to = cqe6_texto(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, clube, jogador, msg, motivo);
        tr = cqe6_texto(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    PLACAR.registrar(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", nome, this, clube, jogador, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}

/// Comparador: lado a lado nas quatro saídas e em [0xae1080] (a regra base pode consultar contratos).
unsafe extern "thiscall" fn comparar_cqe6_periodo_inglaterra(this: usize, clube: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(1589);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || cqe6_periodo_inglaterra(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("cqe6_periodo_inglaterra(regras {:#x}, clube {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, clube, jogador, mo, mr));
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_escocia(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1590), cqe6_pode_emprestar_escocia, "cqe6_pode_emprestar_escocia", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado nas quatro saídas e em [0xae1080].
unsafe extern "thiscall" fn comparar_cqe6_periodo_escocia(this: usize, a1: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(1591);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || cqe6_periodo_escocia(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("cqe6_periodo_escocia(regras {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, jogador, mo, mr));
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_italia(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1592), cqe6_pode_emprestar_italia, "cqe6_pode_emprestar_italia", this, clube, jogador, msg, motivo)
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_irlanda(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1593), cqe6_pode_emprestar_irlanda, "cqe6_pode_emprestar_irlanda", this, clube, jogador, msg, motivo)
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_espanha(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1594), cqe6_pode_emprestar_espanha, "cqe6_pode_emprestar_espanha", this, clube, jogador, msg, motivo)
}

