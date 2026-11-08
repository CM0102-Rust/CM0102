// ---------- %%LEVA%%a leva: as regras de empréstimo da Inglaterra: pode emprestar, a conta da temporada e o dia (england_rules.cpp) ----------

unsafe fn cqe5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqe5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe5_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de um contrato: +4 é o índice em [0xae23b4] (registros de 0x245 bytes), como as regras calculam.
unsafe fn england_rules_club_of_contract(c: usize) -> usize {
    cqe5_d(0xae23b4).wrapping_add(cqe5_d(c + 4).wrapping_mul(0x245))
}

/// O contrato principal da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), que a original
/// guarda em [0xae1080] a cada consulta.
unsafe fn cqe5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqe5_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// A entrada de 16 bytes do clube na tabela dos empréstimos da temporada ([[0xde1f54] + 0x91c] + id do clube que pega emprestado × 16, o byte +0
/// é a conta).
unsafe fn season_loans_entry(clube: usize) -> usize {
    cqe5_d(cqe5_d(0xde1f54) + 0x91c).wrapping_add(cqe5_d(clube).wrapping_mul(16))
}

/// { 0x579d60 } (thiscall, ret 0x10, england_rules.cpp): `ENGLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`, pelas regras da nação em `TRANSFER_MANAGER::can_loan_player` (0x8d0040). Sem contrato (ou sem clube do
/// contrato): pode. O clube que pede e o dono na mesma divisão inglesa ([0x9cf5bc]): motivo 0x1d. Cinco empréstimos na
/// temporada (a tabela [[0xde1f54] + 0x91c]): 0x1e. Mais de um jogador de outro clube no elenco (as 50 vagas em +0xd7,
/// contados só com o id do jogador abaixo de [0xae2364]): 0x26. Fora da janela (0x8e8e50): 9, com o adjetivo da nação.
unsafe extern "thiscall" fn england_rules_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x66f4e0: o texto da mensagem (push ...; push formato; push destino; add esp, 0x14) -> cdecl variádica
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let c = cqe5_contrato(jogador);
    if c == 0 || england_rules_club_of_contract(c) == 0 {
        return 1;
    }
    let c2 = cqe5_contrato(jogador);
    let dono = if c2 != 0 { england_rules_club_of_contract(c2) } else { 0 };
    let comp = cqe5_d(clube + 0x57);
    if comp == cqe5_d(dono + 0x57) && comp != 0 && cqe5_d(comp) == cqe5_d(0x9cf5bc) {
        crate::comparison::branch("cqe_5 mesma divisao");
        if msg != 0 {
            let id = cqe5_d(0x9cf5bc);
            let r = cqe5_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b));
            let g = cqe5_b(r + 0x37) as i8 as i32;
            texto(msg, 0x9c4590, g, g, r + 4);
        }
        if !motivo.is_null() {
            *motivo = 0x1d;
        }
        return 0;
    }
    if cqe5_b(season_loans_entry(clube)) as i8 >= 5 {
        // cqe5: cinco na temporada
        crate::comparison::branch("cqe_5 cinco na temporada");
        if msg != 0 {
            let g = cqe5_b(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c4534, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x1e;
        }
        return 0;
    }
    // 0x4cf6d0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4 (o contrato de empréstimo)
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = cqe5_d(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        if jogador != 0 && cqe5_d(jogador) as i32 >= cqe5_d(0xae2364) as i32 {
            continue;
        }
        if emprestimo(cqe5_d(0xae1050), p) == 0 {
            continue;
        }
        let cp = cqe5_contrato(p);
        let de = if cp != 0 { england_rules_club_of_contract(cp) } else { 0 };
        if de != clube {
            n = n.wrapping_add(1);
        }
    }
    if n > 1 { // cqe5: dois ao mesmo tempo
        crate::comparison::branch("cqe_5 dois ao mesmo tempo");
        if msg != 0 {
            let g = cqe5_b(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c44d4, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x26;
        }
        return 0;
    }
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8 (a janela aberta hoje)
    let janela: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8e8e50usize);
    if janela(this, 0xae2c90, 0) != 0 {
        return 1;
    }
    crate::comparison::branch("cqe_5 fora da janela");
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nação; add esp, 0x10 -> cdecl (o adjetivo da nação)
        let adjetivo: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x6181d0usize);
        let mut buf = [0u8; 100];
        adjetivo(cqe5_d(this + 0xa), buf.as_mut_ptr(), 100, 2);
        texto(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !motivo.is_null() {
        *motivo = 9;
    }
    0
}

/// { 0x579fd0 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::record_loan(oferta)`, quando um empréstimo se
/// conclui (0x8c9390): o clube da oferta (+8, índice em [0xae23b4]) da nação inglesa ([0x9cf2e4]) soma 1 na conta da
/// temporada.
unsafe extern "thiscall" fn record_loan(_this: usize, oferta: usize) {
    let clube = cqe5_d(0xae23b4).wrapping_add(cqe5_d(oferta + 8).wrapping_mul(0x245));
    let nacao = cqe5_d(clube + 0x53);
    if nacao != 0 && cqe5_d(nacao) == cqe5_d(0x9cf2e4) {
        crate::comparison::branch("cqe_5 emprestimo contado");
        let p = season_loans_entry(clube);
        *(p as *mut u8) = cqe5_b(p).wrapping_add(1); // cqe5: a conta da temporada
    }
}

/// { 0x57a020 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::daily_update(data)`, pelo dia das regras: em
/// 20 de junho (só o dia do ano, 0x549f70 com 1998), a conta da temporada de cada clube ([0xae235c] em [0xae23b4]) da
/// nação destas regras (+0xa) volta a zero.
unsafe extern "thiscall" fn england_rules_daily_update(this: usize, data: usize) {
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if cqe5_h(data) != cqe5_h(r) {
        return;
    }
    let mut i: i32 = 0;
    while i < cqe5_d(0xae235c) as i32 {
        let clube = cqe5_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245));
        if cqe5_d(clube + 0x53) == cqe5_d(this + 0xa) {
            *(season_loans_entry(clube) as *mut u8) = 0;
        }
        i += 1;
    }
}

/// O texto da mensagem (até o zero, no máximo 0x200 bytes).
unsafe fn message_text(msg: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if msg == 0 {
        return v;
    }
    for k in 0..0x200usize {
        let b = cqe5_b(msg + k);
        v.push(b);
        if b == 0 {
            break;
        }
    }
    v
}

/// Comparador comum das regras de empréstimo (pode emprestar): lado a lado na devolução, em [0xae1080] e no motivo; a
/// mensagem, que só é escrita, é conferida pelo texto (antes da cópia o primeiro byte vai a zero, e no fim volta o
/// texto da original).
unsafe fn england_rules_compare_can_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, nome: &str, this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || {
        let x = o(this, clube, jogador, msg, motivo);
        to = message_text(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, clube, jogador, msg, motivo);
        tr = message_text(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    SCOREBOARD.record(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", nome, this, clube, jogador, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}

unsafe extern "thiscall" fn comparar_england_rules_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    england_rules_compare_can_loan(original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(%%I0%%), england_rules_can_loan_player, "england_rules_can_loan_player", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado no byte da conta do clube da oferta.
unsafe extern "thiscall" fn comparar_record_loan(this: usize, oferta: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let clube = cqe5_d(0xae23b4).wrapping_add(cqe5_d(oferta + 8).wrapping_mul(0x245));
    let regioes = [(season_loans_entry(clube), 1usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this, oferta), || record_loan(this, oferta));
    SCOREBOARD.record(igual, || format!("record_loan(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

/// Comparador: lado a lado na tabela das contas inteira ([0xae235c] clubes × 16 bytes) só no dia em que ela muda (20
/// de junho); nos outros dias as duas rodam sem gravar nada e só a original fica valendo.
unsafe extern "thiscall" fn comparar_england_rules_daily_update(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if cqe5_h(data) != cqe5_h(r) {
        o(this, data);
        return;
    }
    let n = (cqe5_d(0xae235c) as i32).max(0) as usize;
    let regioes = [(cqe5_d(cqe5_d(0xde1f54) + 0x91c), n * 16)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&regioes, || o(this, data), || england_rules_daily_update(this, data));
    SCOREBOARD.record(igual, || format!("england_rules_daily_update(regras {:#x}) tabela diverge", this));
}
