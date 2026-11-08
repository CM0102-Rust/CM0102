// ---------- %%LEVA%%a leva: as ofertas e o contrato do jogador: se ele está sob oferta, se o clube dele segue a lei Bosman e o veto da diretoria na oferta (transfer_manager.cpp) ----------

unsafe fn cpl2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O clube do id `i` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn club_record(i: usize) -> usize {
    cpl2_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como a original.
unsafe fn contract_of_person(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpl2_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O contrato vale (não nulo e a situação +0x35 & 0x3f ≠ 5).
unsafe fn is_contract_valid(c: usize) -> bool {
    c != 0 && *((c + 0x35) as *const u8) & 0x3f != 5
}

/// O técnico humano da vez ([0xb74c26] + 0x300 × (i16)[0xb77c7a]).
unsafe fn transfer_manager_current_human_manager() -> usize {
    let i = *(0xb77c7a as *const i16) as isize;
    cpl2_d((0xb74c26isize + i * 0x300) as usize)
}

/// O humano da vez manda no clube (0x600c90 com 0, 1).
unsafe fn current_human_controls_club(clube: usize) -> bool {
    let manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
    manda(clube, 0, 1) != 0
}

/// O corpo da 0x8cde70 sobre a lista local já construída.
unsafe fn is_under_offer_body(this: usize, pessoa: usize, visao: u32, lista: usize) -> u32 {
    if pessoa != 0 && cpl2_d(pessoa) as i32 >= cpl2_d(0xae2364) as i32 {
        return 0;
    }
    let marca = || (*((cpl2_d(this + 0x86e).wrapping_add(cpl2_d(pessoa).wrapping_mul(8))) as *const u8) & 1) as u32;
    if visao == 0 {
        return marca(); // a marca "sob oferta"
    }
    let m = transfer_manager_current_human_manager();
    if m == 0 {
        return marca();
    }
    let na_shortlist: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let marcado = na_shortlist(cpl2_d(0xb74448), pessoa, m);
    ofertas(this, pessoa, lista, 0);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    loop {
        let no = proximo(lista);
        if no == 0 {
            break;
        }
        let o = cpl2_d(cpl2_d(this).wrapping_add(cpl2_d(no).wrapping_mul(4)));
        if *((o + 0x2d) as *const u8) != 0 || current_human_controls_club(club_record(cpl2_d(o + 8))) {
            return 1;
        }
        let p = cpl2_d(0xae23bc).wrapping_add(cpl2_d(o + 0xc).wrapping_mul(0x6e));
        let c = contract_of_person(p);
        if is_contract_valid(c) && club_record(cpl2_d(c + 4)) != 0 {
            // o clube do jogador: a original pede o contrato de novo
            let c = contract_of_person(p);
            let clube = if is_contract_valid(c) { club_record(cpl2_d(c + 4)) } else { 0 };
            if current_human_controls_club(clube) {
                return 1;
            }
        }
        if marcado != 0 {
            return 1;
        }
    }
    let x = cpl2_d(transfer_manager_current_human_manager() + 0x39);
    if x != 0 {
        let contract_offer: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cc9e0usize);
        let o = contract_offer(cpl2_d(0xae1050), cpl2_d(pessoa), cpl2_d(x));
        if o != 0 && *((o + 0x35) as *const u8) & 0x3f == 5 {
            return 1;
        }
    }
    0
}

/// { 0x8cde70 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::is_player_under_offer(pessoa, visao)`: id
/// fora da tabela: 0. Sem `visao` (ou sem humano da vez), a marca "sob oferta" (bit 0 de [this+0x86e] + id × 8). Com
/// ela: 1 se alguma oferta em aberto pela pessoa (0x8c5cf0, numa lista local) é conhecida (+0x2d), é de clube do
/// humano da vez (0x600c90) ou o clube atual do jogador da oferta (+0xc; contrato 0x4cf5a0, que fica em [0xae1080]) é
/// dele, ou se a pessoa está na shortlist dele (0x7f7490; vale depois da 1ª oferta); por fim, 1 se o humano (+0x39)
/// tem oferta de contrato aceita (0x4cc9e0, +0x35 & 0x3f = 5) pela pessoa. Senão 0.
unsafe extern "thiscall" fn is_player_under_offer(this: usize, pessoa: usize, visao: u32) -> u32 {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    lista_nova(lista);
    let r = is_under_offer_body(this, pessoa, visao, lista);
    destruir(lista);
    r
}

/// { 0x8d0a70 } (stdcall, ret 4, transfer_manager.cpp): `player_club_uses_bosman(pessoa)`: 1 se o clube do contrato
/// da pessoa (0x4cf5a0, guardado em [0xae1080]) é de nação com transferência livre no fim do contrato (0x544670);
/// sem contrato que valha, sem clube ou sem nação (+0x53): 1. Pessoa nula: a original (caixa 0x3798 e 0). A data
/// local da original (0x54a110) só serve ao quadro SEH. Quente (~31 milhões de chamadas): sem contador.
unsafe extern "stdcall" fn player_club_uses_bosman(pessoa: usize) -> u32 {
    if pessoa == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I1%%)(pessoa);
    }
    let c = contract_of_person(pessoa);
    if !is_contract_valid(c) {
        return 1;
    }
    let clube = club_record(cpl2_d(c + 4));
    if clube == 0 || cpl2_d(clube + 0x53) == 0 {
        return 1;
    }
    let livre: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544670usize);
    livre(cpl2_d(clube + 0x53)) // a nação do clube
}

/// { 0x8cdb80 } (thiscall, ret 0x18, transfer_manager.cpp): `TRANSFER_MANAGER::check_board_veto(oferta, comprador,
/// vendedor, jogador, decide, motivo)`: o valor da oferta (0x8e7b50 com 0, 0, 1, 0, 1); se a oferta tem a parte de
/// troca (+4) vazia (nula ou palavra 0), a diretoria do clube que decide responde (0x696b10 em [0xb67a44] com o
/// jogador, o clube, o valor e "é o comprador" no byte baixo do ponteiro do comprador, como a original empurra). Bit
/// 0x2000: motivo 0x24 e o jogador (+0x61) perde 10% das reputações +0xb (atual), +9 (na nação) e +0xd (mundial); 0x800: motivo 0x17; 0x200: 0x18;
/// cada um devolve 1. Senão 0.
unsafe extern "thiscall" fn check_board_veto(_this: usize, oferta: usize, comprador: usize, _vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7b50usize);
    let v = valor(oferta, 0, 0, 1, 0, 1);
    // sem oferta, ou com a parte de troca (+4) preenchida: a diretoria não é consultada
    let resposta = if oferta == 0 || { let t = cpl2_d(oferta + 4); t != 0 && *(t as *const u16) != 0 } {
        0
    } else {
        let diretoria: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x696b10usize);
        let e_comprador = (comprador as u32 & 0xffff_ff00) | (decide == comprador) as u32;
        diretoria(cpl2_d(0xb67a44), jogador, decide, v, e_comprador)
    };
    let dar = |m: u16| {
        if !motivo.is_null() {
            *motivo = m;
        }
    };
    if resposta & 0x2000 != 0 {
        dar(0x24);
        crate::comparison::branch("cpl_2 board veto values");
        for k in [0xb, 9, 0xd] {
            let p = (cpl2_d(jogador + 0x61) + k) as *mut i16;
            core::ptr::write_unaligned(p, match_pl_ftol(core::ptr::read_unaligned(p) as f64 * read_f64_at(0x9689d8)) as i16); // 90%
        }
        return 1;
    }
    if resposta & 0x800 != 0 {
        dar(0x17);
        return 1;
    }
    if resposta & 0x200 != 0 {
        dar(0x18);
        return 1;
    }
    0
}

/// A região que a 0x8c5cf0 mexe (o cursor da lista própria da oferta da pessoa) e [0xae1080].
unsafe fn offer_cursor_and_lookup_regions(this: usize, pessoa: usize) -> [(usize, usize); 2] {
    let e = offer_for_player(this, cpl2_d(pessoa) as i32);
    let l = if e == 0 { 0 } else { cpl2_d(e) };
    [(0xae1080, 4), if l == 0 { (0xae2364, 4) } else { (l, 0xe) }]
}

// ponytail: só lê (a lista local nasce e morre nela); com a pessoa nula, só a original (lê o endereço 0)
unsafe extern "thiscall" fn compare_is_player_under_offer(this: usize, pessoa: usize, visao: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    if pessoa == 0 {
        return o(this, pessoa, visao);
    }
    let reg = offer_cursor_and_lookup_regions(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, pessoa, visao), || is_player_under_offer(this, pessoa, visao));
    SCOREBOARD.record(igual && r == n, || format!("is_player_under_offer({}, visao {}) original={} rust={} {:x?} x {:x?}", cpl2_d(pessoa), visao, r, n, mo, mr));
    r
}

unsafe extern "stdcall" fn compare_player_club_uses_bosman(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I1%%);
    if pessoa == 0 {
        return o(pessoa);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(pessoa), || player_club_uses_bosman(pessoa));
    SCOREBOARD.record(igual && r == n, || format!("player_club_uses_bosman({}) original={} rust={} {:x?} x {:x?}", cpl2_d(pessoa), r, n, mo, mr));
    r
}

/// Comparador: a diretoria (0x696b10) mexe na confiança e não pode rodar duas vezes: o plano antes (as três palavras
/// do jogador), a original roda e confere: devolução 0 ou 1; com motivo, 0x24 (as palavras a 90%), 0x17 ou 0x18 quando
/// 1 e intocado quando 0; as palavras intocadas fora do motivo 0x24 (sem motivo, intocadas ou as três a 90%).
unsafe extern "thiscall" fn compare_check_board_veto(this: usize, oferta: usize, comprador: usize, vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32>(%%I2%%);
    let dados = if jogador == 0 { 0 } else { cpl2_d(jogador + 0x61) };
    if dados == 0 {
        return o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    }
    let ler = || [0xb, 9, 0xd].map(|k| core::ptr::read_unaligned((dados + k) as *const i16));
    let antes = ler();
    let m0 = if motivo.is_null() { 0 } else { *motivo };
    let r = o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    let depois = ler();
    let a_90 = antes.map(|v| match_pl_ftol(v as f64 * read_f64_at(0x9689d8)) as i16);
    let m = if motivo.is_null() { 0 } else { *motivo };
    let ok = match (r, motivo.is_null()) {
        (0, false) => m == m0 && depois == antes,
        (0, true) => depois == antes,
        (1, false) => (m == 0x24 && depois == a_90) || ((m == 0x17 || m == 0x18) && depois == antes),
        (1, true) => depois == antes || depois == a_90,
        _ => false,
    };
    SCOREBOARD.record(ok, || format!("check_board_veto devolveu {}, motivo {:#x} (antes {:#x}), palavras {:?} -> {:?}", r, m, m0, antes, depois));
    r
}
