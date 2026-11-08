// ---------- %%LEVA%%a leva: quem pode negociar: se o clube pode chegar ao jogador, os termos mandados ao jogador da troca e a fila da transferência livre do clube (transfer_manager.cpp) ----------

unsafe fn cpr2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// Os itens de uma LLIST, do começo (o cursor +0xa volta a +2 e fica no fim, como depois de uma volta da original).
unsafe fn llist_collect(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    cpr2_por(l + 0xa, cpr2_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O corpo da 0x8ce990 sobre a lista local `l` (já construída): devolve (motivo, pode).
unsafe fn can_club_bid_body(this: usize, jogador: usize, clube: usize, ignorar_abertas: u32, l: usize) -> (u16, u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let acertada: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x4cca30usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x8c5fc0usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let nat = cpr2_d(jogador + 0x1a);
    if nat != 0 && cpr2_d(nat) == cpr2_d(0x9cf458) {
        let cc = cpr2_d(jogador + 0x39);
        if (cc == 0 || cpr2_d(cc + 0x53) == nat) && clube != 0 && cpr2_d(clube + 0x53) != nat {
            return (0x27, 0); // não pode sair da nação
        }
    }
    let marca = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8));
    if cpr2_b(marca) & 0x40 != 0 {
        if acertada(cpr2_d(0xae1050), cpr2_d(jogador) as i32) != 0 {
            crate::comparison::branch("cpr_2 contrato acertado");
            return (0x21, 0); // contrato acertado
        }
        let m = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8)) as *mut u8;
        *m &= 0xbf;
    }
    if ignorar_abertas == 0 && (trocas(this, jogador, 0) != 0 || negociacoes(this, jogador, 0) != 0) {
        return (7, 0);
    }
    // a fila da transferência livre (+0x92c)
    let fila = this + 0x92c;
    cpr2_por(fila + 0xa, cpr2_d(fila + 2));
    let mut e = proximo(fila);
    while e != 0 {
        if cpr2_d(jogador) == cpr2_d(e) {
            return (1, 0);
        }
        e = proximo(fila);
    }
    if sob_oferta(this, jogador, 0) != 0 {
        ofertas(this, jogador, l, 0);
        cpr2_por(l + 0xa, cpr2_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            let k = cpr2_d(e) as i32;
            let vaga = if k < cpr2_d(this + 4) as i32 { cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)) } else { cpr2_d(this) };
            let o = cpr2_d(vaga);
            let st = cpr2_b(o + 0x2c);
            if (0xa..=0xe).contains(&st) {
                crate::comparison::branch("cpr_2 oferta em negociacao");
                *((o + 0x2d) as *mut u8) = 1;
                return (0x12, 0); // já em negociação
            }
            if st == 0x13 {
                crate::comparison::branch("cpr_2 transferencia futura");
                return (0x20, 0); // transferência futura acertada
            }
            e = proximo(l);
        }
    }
    if clube == 0 || jogador == 0 || cpr2_d(jogador + 0x61) == 0 {
        return (0, 1);
    }
    if basco(clube) != 0 {
        let nacao = cpr2_d(0xae23a8).wrapping_add(cpr2_d(0x9cf424).wrapping_mul(0x122));
        if estrangeira(jogador, nacao, 1) != 0 {
            return (0x19, 0);
        }
        return (0, 1);
    }
    let n = cpr2_d(clube + 0x53);
    if n != 0 && cpr2_d(n) == cpr2_d(0x9cf4dc) {
        let c57 = cpr2_d(clube + 0x57);
        if c57 == 0 || cpr2_d(c57) != cpr2_d(0x9cf8d8) {
            if estrangeira(jogador, n, 1) != 0 {
                return (0x28, 0);
            }
            return (0, 1);
        }
    }
    let id = cpr2_d(clube);
    if id == cpr2_d(0x9d0128) || id == cpr2_d(0x9d01a0) {
        let n1 = cpr2_d(jogador + 0x1a);
        if n1 == 0 || cpr2_d(n1) == cpr2_d(0x9cf488) {
            let n2 = cpr2_d(jogador + 0x1e);
            if n2 == 0 || cpr2_d(n2) != cpr2_d(0x9cf424) {
                return (0x1a, 0);
            }
        }
    }
    (0, 1)
}

/// { 0x8ce990 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::can_club_bid_for_player(jogador, clube, motivo, ignorar_abertas)`: não pode (0 e o motivo) se o
/// jogador é da nação [0x9cf458] sem clube ou com clube dela e o clube é de fora (0x27); com a marca de contrato
/// acertado (0x40) e o acerto (0x4cca30) (0x21; sem acerto, a marca sai); sem `ignorar_abertas`, com trocas que o
/// oferecem (0x8c5e40) ou negociações de troca (0x8c5fc0) (7); na fila da transferência livre (+0x92c) (1); sob oferta
/// (0x8cde70), pelas ofertas por ele (0x8c5cf0): uma em negociação (estados 0xa a 0xe; marca +0x2d) (0x12) ou uma
/// transferência futura (0x13) (0x20). Pelo clube: basco (0x53d7f0) com jogador estrangeiro na nação [0x9cf424] (0x19);
/// da nação [0x9cf4dc] fora da [0x9cf8d8] (+0x57) com jogador estrangeiro (0x28); os clubes [0x9d0128]/[0x9d01a0] com
/// jogador sem a nação [0x9cf424] (0x1a). Senão pode (1, motivo 0).
unsafe extern "thiscall" fn can_club_bid_for_player(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let (m, r) = can_club_bid_body(this, jogador, clube, ignorar_abertas, l);
    if !motivo.is_null() {
        *motivo = m;
    }
    soltar(l);
    r
}

/// { 0x8c2060 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::note_terms_offered_to_exchange_player(k,
/// pessoa)`: `k` negativo ou fora da conta (+4): a caixa "Error" (0xdc3 ou 0xdc8). Na oferta `k` (vazia: nada), pela
/// lista dos jogadores da troca (+4), marca (+4 do item) o da `pessoa`; nenhum item marcado: o estado (+0x2c) vira 8 (se
/// era 7) ou 6.
unsafe extern "thiscall" fn note_terms_offered_to_exchange_player(this: usize, k: i32, pessoa: u32) {
    if k < 0 {
        show_error_at(0xa8d514, 0xdc3);
        return;
    }
    if k >= cpr2_d(this + 4) as i32 {
        show_error_at(0xa8d514, 0xdc8);
        return;
    }
    let o = cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)));
    if o == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cpr2_d(o + 4);
    cpr2_por(l + 0xa, cpr2_d(l + 2));
    let mut nenhum = true;
    let mut e = proximo(cpr2_d(o + 4));
    while e != 0 {
        if cpr2_d(e) as u32 == pessoa {
            *((e + 4) as *mut u8) = 1; // termos mandados
        }
        if cpr2_b(e + 4) != 0 {
            nenhum = false;
        }
        e = proximo(cpr2_d(o + 4));
    }
    if nenhum {
        *((o + 0x2c) as *mut u8) = if cpr2_b(o + 0x2c) == 7 { 8 } else { 6 };
    }
}

/// { 0x8c1780 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::count_club_free_transfer_queue(clube)`:
/// clube nulo: a caixa "Error" 0xc04 e 0. Senão conta as pessoas da fila da transferência livre (+0x92c; o cursor volta
/// ao começo) com o clube do contrato (+0x39) igual ao clube ou ao parceiro dele (0x540a50 com 0 e 1), não nulo.
unsafe extern "thiscall" fn count_club_free_transfer_queue(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        show_error_at(0xa8d514, 0xc04);
        return 0;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let p = parceiro(clube, 0, 1);
    cpr2_por(this + 0x936, cpr2_d(this + 0x92e));
    let mut n = 0u8;
    let mut e = proximo(this + 0x92c);
    while e != 0 {
        let c = cpr2_d(cpr2_d(0xae23bc).wrapping_add(cpr2_d(e).wrapping_mul(0x6e)) + 0x39);
        if c == clube || (c == p && p != 0) {
            n = n.wrapping_add(1);
        }
        e = proximo(this + 0x92c);
    }
    n
}

/// Comparador: lado a lado pelo motivo, pela marca da pessoa (+0x86e) e pelo cursor da fila livre, e pela devolução.
/// As outras gravações (tirar a marca 0x40, marcar +0x2d da oferta em negociação) a cópia repete sobre o que a original
/// deixou e chega ao mesmo.
unsafe extern "thiscall" fn compare_can_club_bid_for_player(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32>(%%I0%%);
    let marca = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8));
    let regioes = [(marca, 1), (this + 0x936, 4), (motivo as usize, 2)];
    let n = if motivo.is_null() { 2 } else { 3 };
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&regioes[..n], || o(this, jogador, clube, motivo, ignorar_abertas), || can_club_bid_for_player(this, jogador, clube, motivo, ignorar_abertas));
    SCOREBOARD.record(igual && r == c, || format!("can_club_bid_for_player(jogador {}, clube {:x}, {}) original={} rust={} {:x?} x {:x?}", cpr2_d(jogador), clube, ignorar_abertas, r, c, mo, mr));
    r
}

/// Comparador: lado a lado pelo estado da oferta, pelas marcas (+4) dos itens da troca e pelo cursor da lista. Fora da
/// conta: a caixa, só a original.
unsafe extern "thiscall" fn compare_note_terms_offered_to_exchange_player(this: usize, k: i32, pessoa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32)>(%%I1%%);
    if k < 0 || k >= cpr2_d(this + 4) as i32 || cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4))) == 0 {
        return o(this, k, pessoa);
    }
    let of = cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)));
    let l = cpr2_d(of + 4);
    let mut regioes = vec![(of + 0x2c, 1), (l + 0xa, 4)];
    for e in llist_collect(l) {
        regioes.push((e + 4, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this, k, pessoa), || note_terms_offered_to_exchange_player(this, k, pessoa));
    SCOREBOARD.record(igual, || format!("note_terms_offered_to_exchange_player({}, pessoa {}) {:x?} x {:x?}", k, pessoa, mo, mr));
}

/// Comparador: lado a lado pelo cursor da fila e pela devolução (só lê). Clube nulo: a caixa, só a original.
unsafe extern "thiscall" fn compare_count_club_free_transfer_queue(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%);
    if clube == 0 {
        return o(this, clube);
    }
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + 0x936, 4)], || o(this, clube), || count_club_free_transfer_queue(this, clube));
    SCOREBOARD.record(igual && r == c, || format!("count_club_free_transfer_queue({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}
