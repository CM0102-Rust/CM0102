// ---------- 248a leva: as ofertas pelo jogador: se o clube tem oferta por ele, a oferta do clube e se ele tem oferta pendente (transfer_manager.cpp) ----------

unsafe fn cpl1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A marca "sob oferta" da pessoa (bit 0 do byte [this+0x86e] + id × 8).
unsafe fn cpl1_marcada(this: usize, id: usize) -> bool {
    *((cpl1_d(this + 0x86e).wrapping_add(id.wrapping_mul(8))) as *const u8) & 1 != 0
}

/// A oferta do índice `k` (vetor de ponteiros [this]).
unsafe fn cpl1_oferta(this: usize, k: usize) -> usize {
    cpl1_d(cpl1_d(this).wrapping_add(k.wrapping_mul(4)))
}

/// A lista local de índices das ofertas pela pessoa (0x68d020, que a 0x8c5cf0 enche com `todas`), com o cursor
/// (+0xa) reposto no começo (+2) como a original; `f` percorre e a lista é destruída (0x68d040) no fim. Nas saídas
/// antes da 0x8c5cf0 a original constrói e destrói a lista vazia, sem efeito (não aloca): aqui não nasce.
unsafe fn cpl1_com_ofertas<R>(this: usize, pessoa: usize, todas: u32, f: impl FnOnce(usize) -> R) -> R {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    lista_nova(lista);
    ofertas(this, pessoa, lista, todas);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let r = f(lista);
    destruir(lista);
    r
}

/// O próximo nó da lista (0x68d1b0), ou 0.
unsafe fn cpl1_proximo(lista: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(lista)
}

/// A pessoa fora da tabela (id ≥ [0xae2364]); pessoa nula passa e é lida como na original.
unsafe fn cpl1_fora(pessoa: usize) -> bool {
    pessoa != 0 && cpl1_d(pessoa) as i32 >= cpl1_d(0xae2364) as i32
}

/// { 0x8cdc90 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::club_has_offer_for_player(pessoa,
/// clube)`: 1 se a pessoa marcada como sob oferta (bit 0 de [this+0x86e] + id × 8) tem, entre as ofertas em aberto
/// (0x8c5cf0 sem `todas`), uma do clube (+8 da oferta = id do clube); senão 0 (id fora da tabela também).
unsafe extern "thiscall" fn clube_tem_oferta_pelo_jogador(this: usize, pessoa: usize, clube: usize) -> u32 {
    if cpl1_fora(pessoa) || !cpl1_marcada(this, cpl1_d(pessoa)) {
        return 0;
    }
    cpl1_com_ofertas(this, pessoa, 0, |lista| loop {
        let no = cpl1_proximo(lista);
        if no == 0 {
            return 0;
        }
        if cpl1_d(cpl1_oferta(this, cpl1_d(no)) + 8) == cpl1_d(clube) { // a oferta é do clube
            crate::comparacao::ramo("cpl_1 clube tem oferta");
            return 1;
        }
    })
}

/// { 0x8cdd80 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::find_club_offer_for_player(pessoa,
/// clube, todas)`: a oferta do clube pela pessoa (a primeira da lista da 0x8c5cf0, com `todas` também as encerradas),
/// ou 0. Sem `todas`, a pessoa tem de estar marcada como sob oferta; id fora da tabela: 0.
unsafe extern "thiscall" fn oferta_do_clube_pelo_jogador(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    if cpl1_fora(pessoa) || (todas == 0 && !cpl1_marcada(this, cpl1_d(pessoa))) {
        return 0;
    }
    cpl1_com_ofertas(this, pessoa, todas, |lista| loop {
        let no = cpl1_proximo(lista);
        if no == 0 {
            return 0;
        }
        let o = cpl1_oferta(this, cpl1_d(no));
        if cpl1_d(o + 8) == cpl1_d(clube) { // a oferta do clube
            crate::comparacao::ramo("cpl_1 oferta do clube");
            return o;
        }
    })
}

/// { 0x8cd150 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::player_has_pending_offer(pessoa)`: 1 se
/// a pessoa está sob oferta (0x8cde70 sem a visão do humano) e uma das ofertas em aberto (0x8c5cf0) está na situação
/// (+0x2c) 0, 1, 5 ou 6; senão 0. O índice além da conta (+4) lê a oferta 0, como a original.
unsafe extern "thiscall" fn jogador_com_oferta_pendente(this: usize, pessoa: usize) -> u32 {
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    // ponytail: a original constrói a lista antes de perguntar; vazia, a 0x68d020/0x68d040 não alocam (nada a imitar)
    if sob_oferta(this, pessoa, 0) == 0 {
        return 0;
    }
    cpl1_com_ofertas(this, pessoa, 0, |lista| loop {
        let no = cpl1_proximo(lista);
        if no == 0 {
            return 0;
        }
        let k = cpl1_d(no) as i32;
        let o = if k < cpl1_d(this + 4) as i32 { cpl1_oferta(this, k as usize) } else { cpl1_d(cpl1_d(this)) };
        if matches!(*((o + 0x2c) as *const u8), 0 | 1 | 5 | 6) { // a situação pendente
            crate::comparacao::ramo("cpl_1 oferta pendente");
            return 1;
        }
    })
}

/// As regiões que a 0x8c5cf0 mexe: o cursor da lista própria da oferta da pessoa (+0xa..+0xe; a 0x8c5cf0 o repõe).
unsafe fn cpl1_regioes(this: usize, pessoa: usize) -> [(usize, usize); 1] {
    let e = oferta_pelo_jogador(this, cpl1_d(pessoa) as i32);
    let l = if e == 0 { 0 } else { cpl1_d(e) };
    if l == 0 { [(0xae2364, 4)] } else { [(l, 0xe)] }
}

// ponytail: as três só leem (a lista local nasce e morre nelas); com a pessoa nula, só a original (lê o endereço 0)
unsafe extern "thiscall" fn comparar_clube_tem_oferta_pelo_jogador(this: usize, pessoa: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(904);
    if pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let reg = cpl1_regioes(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube), || clube_tem_oferta_pelo_jogador(this, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("clube_tem_oferta_pelo_jogador({}, clube {}) original={} rust={} {:x?} x {:x?}", cpl1_d(pessoa), cpl1_d(clube), r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_oferta_do_clube_pelo_jogador(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize>(905);
    if pessoa == 0 {
        return o(this, pessoa, clube, todas);
    }
    let reg = cpl1_regioes(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, todas), || oferta_do_clube_pelo_jogador(this, pessoa, clube, todas));
    PLACAR.registrar(igual && r == n, || format!("oferta_do_clube_pelo_jogador({}, clube {}, todas {}) original={:x} rust={:x} {:x?} x {:x?}", cpl1_d(pessoa), cpl1_d(clube), todas, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_jogador_com_oferta_pendente(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(906);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let reg = cpl1_regioes(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || jogador_com_oferta_pendente(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("jogador_com_oferta_pendente({}) original={} rust={} {:x?} x {:x?}", cpl1_d(pessoa), r, n, mo, mr));
    r
}

// ---------- 248a leva: as ofertas e o contrato do jogador: se ele está sob oferta, se o clube dele segue a lei Bosman e o veto da diretoria na oferta (transfer_manager.cpp) ----------

unsafe fn cpl2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O clube do id `i` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpl2_clube(i: usize) -> usize {
    cpl2_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como a original.
unsafe fn cpl2_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpl2_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O contrato vale (não nulo e a situação +0x35 & 0x3f ≠ 5).
unsafe fn cpl2_vale(c: usize) -> bool {
    c != 0 && *((c + 0x35) as *const u8) & 0x3f != 5
}

/// O técnico humano da vez ([0xb74c26] + 0x300 × (i16)[0xb77c7a]).
unsafe fn cpl2_humano_da_vez() -> usize {
    let i = *(0xb77c7a as *const i16) as isize;
    cpl2_d((0xb74c26isize + i * 0x300) as usize)
}

/// O humano da vez manda no clube (0x600c90 com 0, 1).
unsafe fn cpl2_manda(clube: usize) -> bool {
    let manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
    manda(clube, 0, 1) != 0
}

/// O corpo da 0x8cde70 sobre a lista local já construída.
unsafe fn cpl2_sob_oferta(this: usize, pessoa: usize, visao: u32, lista: usize) -> u32 {
    if pessoa != 0 && cpl2_d(pessoa) as i32 >= cpl2_d(0xae2364) as i32 {
        return 0;
    }
    let marca = || (*((cpl2_d(this + 0x86e).wrapping_add(cpl2_d(pessoa).wrapping_mul(8))) as *const u8) & 1) as u32;
    if visao == 0 {
        return marca(); // a marca "sob oferta"
    }
    let m = cpl2_humano_da_vez();
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
        if *((o + 0x2d) as *const u8) != 0 || cpl2_manda(cpl2_clube(cpl2_d(o + 8))) {
            return 1;
        }
        let p = cpl2_d(0xae23bc).wrapping_add(cpl2_d(o + 0xc).wrapping_mul(0x6e));
        let c = cpl2_contrato(p);
        if cpl2_vale(c) && cpl2_clube(cpl2_d(c + 4)) != 0 {
            // o clube do jogador: a original pede o contrato de novo
            let c = cpl2_contrato(p);
            let clube = if cpl2_vale(c) { cpl2_clube(cpl2_d(c + 4)) } else { 0 };
            if cpl2_manda(clube) {
                return 1;
            }
        }
        if marcado != 0 {
            return 1;
        }
    }
    let x = cpl2_d(cpl2_humano_da_vez() + 0x39);
    if x != 0 {
        let oferta_de_contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cc9e0usize);
        let o = oferta_de_contrato(cpl2_d(0xae1050), cpl2_d(pessoa), cpl2_d(x));
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
unsafe extern "thiscall" fn jogador_sob_oferta(this: usize, pessoa: usize, visao: u32) -> u32 {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    lista_nova(lista);
    let r = cpl2_sob_oferta(this, pessoa, visao, lista);
    destruir(lista);
    r
}

/// { 0x8d0a70 } (stdcall, ret 4, transfer_manager.cpp): `player_club_uses_bosman(pessoa)`: 1 se o clube do contrato
/// da pessoa (0x4cf5a0, guardado em [0xae1080]) é de nação com transferência livre no fim do contrato (0x544670);
/// sem contrato que valha, sem clube ou sem nação (+0x53): 1. Pessoa nula: a original (caixa 0x3798 e 0). A data
/// local da original (0x54a110) só serve ao quadro SEH. Quente (~31 milhões de chamadas): sem contador.
unsafe extern "stdcall" fn clube_do_jogador_com_lei_bosman(pessoa: usize) -> u32 {
    if pessoa == 0 {
        return orig::<unsafe extern "stdcall" fn(usize) -> u32>(908)(pessoa);
    }
    let c = cpl2_contrato(pessoa);
    if !cpl2_vale(c) {
        return 1;
    }
    let clube = cpl2_clube(cpl2_d(c + 4));
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
unsafe extern "thiscall" fn veto_da_diretoria_na_oferta(_this: usize, oferta: usize, comprador: usize, _vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
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
        crate::comparacao::ramo("cpl_2 veto da diretoria valores");
        for k in [0xb, 9, 0xd] {
            let p = (cpl2_d(jogador + 0x61) + k) as *mut i16;
            core::ptr::write_unaligned(p, ftol(core::ptr::read_unaligned(p) as f64 * f64_em(0x9689d8)) as i16); // 90%
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
unsafe fn cpl2_regioes(this: usize, pessoa: usize) -> [(usize, usize); 2] {
    let e = oferta_pelo_jogador(this, cpl2_d(pessoa) as i32);
    let l = if e == 0 { 0 } else { cpl2_d(e) };
    [(0xae1080, 4), if l == 0 { (0xae2364, 4) } else { (l, 0xe) }]
}

// ponytail: só lê (a lista local nasce e morre nela); com a pessoa nula, só a original (lê o endereço 0)
unsafe extern "thiscall" fn comparar_jogador_sob_oferta(this: usize, pessoa: usize, visao: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(907);
    if pessoa == 0 {
        return o(this, pessoa, visao);
    }
    let reg = cpl2_regioes(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, visao), || jogador_sob_oferta(this, pessoa, visao));
    PLACAR.registrar(igual && r == n, || format!("jogador_sob_oferta({}, visao {}) original={} rust={} {:x?} x {:x?}", cpl2_d(pessoa), visao, r, n, mo, mr));
    r
}

unsafe extern "stdcall" fn comparar_clube_do_jogador_com_lei_bosman(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(908);
    if pessoa == 0 {
        return o(pessoa);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa), || clube_do_jogador_com_lei_bosman(pessoa));
    PLACAR.registrar(igual && r == n, || format!("clube_do_jogador_com_lei_bosman({}) original={} rust={} {:x?} x {:x?}", cpl2_d(pessoa), r, n, mo, mr));
    r
}

/// Comparador: a diretoria (0x696b10) mexe na confiança e não pode rodar duas vezes: o plano antes (as três palavras
/// do jogador), a original roda e confere: devolução 0 ou 1; com motivo, 0x24 (as palavras a 90%), 0x17 ou 0x18 quando
/// 1 e intocado quando 0; as palavras intocadas fora do motivo 0x24 (sem motivo, intocadas ou as três a 90%).
unsafe extern "thiscall" fn comparar_veto_da_diretoria_na_oferta(this: usize, oferta: usize, comprador: usize, vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32>(909);
    let dados = if jogador == 0 { 0 } else { cpl2_d(jogador + 0x61) };
    if dados == 0 {
        return o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    }
    let ler = || [0xb, 9, 0xd].map(|k| core::ptr::read_unaligned((dados + k) as *const i16));
    let antes = ler();
    let m0 = if motivo.is_null() { 0 } else { *motivo };
    let r = o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    let depois = ler();
    let a_90 = antes.map(|v| ftol(v as f64 * f64_em(0x9689d8)) as i16);
    let m = if motivo.is_null() { 0 } else { *motivo };
    let ok = match (r, motivo.is_null()) {
        (0, false) => m == m0 && depois == antes,
        (0, true) => depois == antes,
        (1, false) => (m == 0x24 && depois == a_90) || ((m == 0x17 || m == 0x18) && depois == antes),
        (1, true) => depois == antes || depois == a_90,
        _ => false,
    };
    PLACAR.registrar(ok, || format!("veto_da_diretoria_na_oferta devolveu {}, motivo {:#x} (antes {:#x}), palavras {:?} -> {:?}", r, m, m0, antes, depois));
    r
}

// ---------- 248a leva: os registros das transferências: guardar a transferência passada, dar a permissão de trabalho e guardar o prêmio por jogo no time (transfer_manager.cpp) ----------

unsafe fn cpl3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (versão 0x90d130, nome
/// e extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
unsafe fn cpl3_sem_memoria(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Cresce o vetor [this+v] (capacidade em +v+`c`, conta em +v+12−`c`: `c` = 4 nas passadas e nos prêmios, 8 nas
/// permissões) de `passo` registros de `tam` bytes quando cheio: malloc (0x944e46) sem vetor, senão realloc
/// (0x945667). Devolve se há vetor (sem memória: a caixa já saiu).
unsafe fn cpl3_crescer(this: usize, v: usize, c: usize, passo: usize, tam: usize, linha: u32, texto: usize) -> bool {
    if cpl3_d(this + v + 12 - c) != cpl3_d(this + v + c) {
        return true;
    }
    let cap = cpl3_d(this + v + c).wrapping_add(passo);
    cpl3_por(this + v + c, cap);
    let p = if cpl3_d(this + v) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        alocar(cap.wrapping_mul(tam) as u32)
    } else {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        realocar(cpl3_d(this + v), cap.wrapping_mul(tam) as u32)
    };
    cpl3_por(this + v, p);
    if p == 0 {
        cpl3_sem_memoria(linha, texto);
        return false;
    }
    true
}

/// { 0x8cb810 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_past_transfer(registro)`: acrescenta
/// a transferência passada (0x25 bytes, cópia 0x8cb940) ao vetor [this+0x862] (capacidade +0x866, conta +0x86a, que
/// sobe antes da cópia); cheio, cresce de 100 em 100 (sem memória: caixa 0x2794 e nada). Devolve o registro novo.
unsafe extern "thiscall" fn guardar_transferencia_passada(this: usize, registro: usize) -> usize {
    crate::comparacao::ramo("cpl_3 transferencia passada");
    // ponytail: sem memória a original devolve o que a caixa deixou em eax; aqui 0 (o jogo não lê)
    if !cpl3_crescer(this, 0x862, 4, 100, 0x25, 0x2794, 0xa8ec34) {
        return 0;
    }
    let n = cpl3_d(this + 0x86a);
    let novo = cpl3_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    cpl3_por(this + 0x86a, n.wrapping_add(1)); // a conta das transferências passadas
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cb940usize);
    copiar(novo, registro)
}

/// { 0x8c7070 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::add_work_permit(pessoa, nacao, ano, _,
/// ordenar)`: acrescenta a permissão de trabalho (14 bytes: +0 id da pessoa, +4 e +6 zerados, +8 ano, +0xa id da
/// nação) ao vetor [this+0x93a] (capacidade +0x942, conta +0x93e); cheio, cresce de 30 em 30 (sem memória: caixa
/// 0x1b1b e nada). Liga a marca da permissão (bit 0x20 de [this+0x86e] + id × 8) e, com `ordenar`, reordena (qsort
/// 0x945333 com 0x8e55c0, a ordem da busca 0x8c7560).
unsafe extern "thiscall" fn dar_permissao_de_trabalho(this: usize, pessoa: usize, nacao: usize, ano: u32, _nao_usado: u32, ordenar: u32) {
    crate::comparacao::ramo("cpl_3 permissao de trabalho");
    if !cpl3_crescer(this, 0x93a, 8, 30, 0xe, 0x1b1b, 0xa8eb04) {
        return;
    }
    let n = cpl3_d(this + 0x93e);
    let r = cpl3_d(this + 0x93a).wrapping_add(n.wrapping_mul(0xe));
    cpl3_por(r, cpl3_d(pessoa));
    cpl3_por(r + 0xa, cpl3_d(nacao));
    core::ptr::write_unaligned((r + 8) as *mut u16, ano as u16);
    core::ptr::write_unaligned((r + 6) as *mut u16, 0);
    core::ptr::write_unaligned((r + 4) as *mut u16, 0);
    cpl3_por(this + 0x93e, n.wrapping_add(1));
    let marca = (cpl3_d(this + 0x86e).wrapping_add(cpl3_d(pessoa).wrapping_mul(8))) as *mut u8;
    *marca |= 0x20; // a marca da permissão
    if ordenar != 0 {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cpl3_d(this + 0x93a), cpl3_d(this + 0x93e) as u32, 0xe, 0x8e55c0);
    }
}

/// { 0x8cac40 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_first_team_appearance_money(registro)`:
/// acrescenta o prêmio por jogo no time principal (13 bytes copiados de `registro`) ao vetor [this+0x88c] (capacidade
/// +0x890, conta +0x894); cheio, cresce de 50 em 50 (sem memória: caixa 0x2581 e nada). Reordena pela pessoa (qsort
/// 0x945333 com 0x8b4e00).
unsafe extern "thiscall" fn guardar_premio_por_jogo_no_time(this: usize, registro: usize) {
    crate::comparacao::ramo("cpl_3 premio por jogo");
    if !cpl3_crescer(this, 0x88c, 4, 50, 0xd, 0x2581, 0xa8eb94) {
        return;
    }
    let n = cpl3_d(this + 0x894);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpl3_d(this + 0x88c).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    cpl3_por(this + 0x894, n.wrapping_add(1)); // mais um prêmio
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpl3_d(this + 0x88c), cpl3_d(this + 0x894) as u32, 0xd, 0x8b4e00);
}

/// Algum dos `n` registros de `tam` bytes do vetor `v` é igual a `alvo`.
unsafe fn cpl3_tem(v: usize, n: usize, tam: usize, alvo: &[u8]) -> bool {
    (0..n).any(|i| core::slice::from_raw_parts((v + i * tam) as *const u8, tam) == alvo)
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta, o registro),
/// a original roda, confere a conta, o registro novo (as duplas e o byte +0x1c; a data é da 0x8cb940) e a devolução.
unsafe extern "thiscall" fn comparar_guardar_transferencia_passada(this: usize, registro: usize) -> usize {
    let n = cpl3_d(this + 0x86a);
    let fonte = core::slice::from_raw_parts(registro as *const u8, 0x1d).to_vec();
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(910)(this, registro);
    if cpl3_d(this + 0x862) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let novo = cpl3_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let gravado = core::slice::from_raw_parts(novo as *const u8, 0x1d);
    PLACAR.registrar(r == novo && cpl3_d(this + 0x86a) == n.wrapping_add(1) && gravado == &fonte[..], || {
        format!("guardar_transferencia_passada: conta {} -> {}, devolveu {:x} (esperava {:x}), registro {:x?} x {:x?}", n, cpl3_d(this + 0x86a), r, novo, fonte, gravado)
    });
    r
}

/// Comparador: o plano antes (a conta e o registro esperado), a original roda; confere a conta, a marca 0x20 e que o
/// registro está no vetor (com `ordenar`, em qualquer lugar).
unsafe extern "thiscall" fn comparar_dar_permissao_de_trabalho(this: usize, pessoa: usize, nacao: usize, ano: u32, nao_usado: u32, ordenar: u32) {
    let n = cpl3_d(this + 0x93e);
    let mut alvo = [0u8; 0xe];
    alvo[0..4].copy_from_slice(&(cpl3_d(pessoa) as u32).to_le_bytes());
    alvo[8..10].copy_from_slice(&(ano as u16).to_le_bytes());
    alvo[0xa..0xe].copy_from_slice(&(cpl3_d(nacao) as u32).to_le_bytes());
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(911)(this, pessoa, nacao, ano, nao_usado, ordenar);
    if cpl3_d(this + 0x93a) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpl3_d(this + 0x93e);
    let marca = *((cpl3_d(this + 0x86e).wrapping_add(cpl3_d(pessoa).wrapping_mul(8))) as *const u8);
    let ok = conta == n.wrapping_add(1) && marca & 0x20 != 0 && cpl3_tem(cpl3_d(this + 0x93a), conta, 0xe, &alvo);
    PLACAR.registrar(ok, || format!("dar_permissao_de_trabalho(pessoa {}, nação {}, ano {}): conta {} -> {}, marca {:#x}", cpl3_d(pessoa), cpl3_d(nacao), ano as u16, n, conta, marca));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
unsafe extern "thiscall" fn comparar_guardar_premio_por_jogo_no_time(this: usize, registro: usize) {
    let n = cpl3_d(this + 0x894);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    orig::<unsafe extern "thiscall" fn(usize, usize)>(912)(this, registro);
    if cpl3_d(this + 0x88c) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpl3_d(this + 0x894);
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpl3_d(this + 0x88c), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("guardar_premio_por_jogo_no_time: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

// ---------- 248a leva: os contratos da comissão e as datas de decisão: achar a vaga de contrato, dar o contrato à pessoa nova e marcar a data da decisão (contract_manager.cpp) ----------

unsafe fn cpl4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A caixa "cannot create space" do CONTRACT_MANAGER: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de
/// contract_manager.cpp 0x9a8190, `linha`, `texto`) em 0xae24d0; o título "Memory Allocation Error" (0x9a8108)
/// traduzido (0x66f4e0) em 0xae28bc, o corpo 0x9a7ff8 com o texto em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está).
unsafe fn cpl4_sem_espaco(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// { 0x4c79a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::get_free_staff_contract_space(so_no_fim)`:
/// a vaga do vetor de contratos (registros de 0x50 bytes em [this], capacidade +4, conta +8). Sem `so_no_fim`, a
/// primeira vaga vazia (pessoa +0 = −1); senão (ou sem vaga vazia) a do fim: cheio, cresce de 200 em 200 (realloc
/// 0x945667, ou malloc 0x944e46 sem vetor; sem memória, caixa 0x175 e −1) e a conta sobe. Devolve o índice. Os dois
/// chamadores (a criação dos contratos 0x4c7560 e a 0x4c7950) passam 1.
unsafe extern "thiscall" fn vaga_livre_de_contrato(this: usize, so_no_fim: u32) -> i32 {
    if so_no_fim == 0 {
        let n = cpl4_d(this + 8) as i32;
        let p = cpl4_d(this);
        let mut i = 0i32;
        while i < n {
            if cpl4_d(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                return i;
            }
            i += 1;
        }
    }
    if cpl4_d(this + 8) == cpl4_d(this + 4) {
        let cap = cpl4_d(this + 4).wrapping_add(200);
        let p = cpl4_d(this);
        cpl4_por(this + 4, cap);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(p, cap.wrapping_mul(0x50) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(cap.wrapping_mul(0x50) as u32)
        };
        cpl4_por(this, q);
        if q == 0 {
            cpl4_sem_espaco(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = cpl4_d(this + 8).wrapping_add(1);
    cpl4_por(this + 8, n); // mais uma vaga
    (n as i32).wrapping_sub(1)
}

/// { 0x4c7950 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::add_staff_contract(pessoa, valor)`: a vaga
/// do fim (0x4c79a0 com 1); sem vaga, nada. Liga a pessoa à vaga (o índice +0 do registro de 0x4f bytes dela em
/// [0xae2d0c]), põe o id dela no contrato (+0) e gera o contrato inicial (0x85e960 com o valor). Pela contratação
/// de técnico (0x68f9c0, MANAGER_MANAGER::daily_update) e pelo regen que assina com um clube (0x7a6f10). Devolve o
/// eax da original (o da 0x85e960, ou o índice negativo); os chamadores não o leem.
unsafe extern "thiscall" fn acrescentar_contrato_da_pessoa(this: usize, pessoa: usize, valor: u32) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x4c79a0usize);
    let i = vaga(this, 1);
    if i < 0 {
        return i as u32;
    }
    crate::comparacao::ramo("cpl_4 contrato da pessoa");
    cpl4_por(cpl4_d(0xae2d0c).wrapping_add(cpl4_d(pessoa).wrapping_mul(0x4f)), i as usize); // a pessoa aponta a vaga
    let c = cpl4_d(this).wrapping_add((i as usize).wrapping_mul(0x50));
    cpl4_por(c, cpl4_d(pessoa));
    let gerar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85e960usize);
    gerar(c, pessoa, valor) // o valor da pessoa
}

/// As datas dos lembretes (tipo 2) que a data de decisão de tipo 1 leva antes dela: com o registro de jogador
/// (+0x61), L = moral do contrato (+0x44) + 100 (8 bits; o contrato pela tabela [0xae2d0c], como a 0x4cf5a0
/// inline: sem contrato a original lê o endereço 0x44 e cai, como aqui); L ≤ 0x50 sem lembrete; L > 0x96 um a
/// L × 90 / 200 dias antes; e um a L / 4 dias antes (sem o registro de jogador, L = 100). As datas pela
/// Date::sub_days (0x54c8e0), na ordem da original.
unsafe fn cpl4_lembretes(this: usize, pessoa: usize, data: usize, tipo: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if tipo != 1 {
        return (v, k);
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut l: u8 = 100;
    if cpl4_d(pessoa + 0x61) != 0 {
        let id = cpl4_d(pessoa) as i32;
        let c = if pessoa == 0 || id >= cpl4_d(0xae2364) as i32 {
            0
        } else {
            let i = cpl4_d(cpl4_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f))) as i32;
            if i < 0 { 0 } else { cpl4_d(this).wrapping_add((i as usize).wrapping_mul(0x50)) }
        };
        l = (*((c + 0x44) as *const u8)).wrapping_add(100);
        if l <= 0x50 {
            return (v, k);
        }
        if l > 0x96 {
            let mut t = [0u8; 8];
            let r = menos(data, t.as_mut_ptr(), (l as u32 * 90) / 200);
            v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
            k += 1;
        }
    }
    let mut t = [0u8; 8];
    let r = menos(data, t.as_mut_ptr(), (l as u32 * 50) / 200);
    v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
    k += 1;
    (v, k)
}

/// { 0x4cd6d0 } (thiscall, ret 0x10, contract_manager.cpp): `CONTRACT_MANAGER::add_decision_date(pessoa, data, fatia,
/// tipo)`: acrescenta ao vetor de 14 bytes [this+0xc] (capacidade +0x10, conta +0x14; cresce de 100 em 100 quando a
/// conta passa da capacidade − 4, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa 0x103e e nada) a entrada
/// (id da pessoa +0, data +4, fatia do dia +0xc, tipo +0xd). O tipo 1 leva antes os lembretes de tipo 2
/// (`cpl4_lembretes`), sem fatia (+0xc fica como está). Pelas propostas: a decisão da pessoa (0x4cd230, 0x4ce4e0).
/// As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
unsafe extern "thiscall" fn acrescentar_data_de_decisao(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    crate::comparacao::ramo("cpl_4 data de decisao");
    if cpl4_d(this + 0x14) as i32 > (cpl4_d(this + 0x10) as i32).wrapping_sub(4) {
        let cap = cpl4_d(this + 0x10).wrapping_add(100);
        cpl4_por(this + 0x10, cap);
        let p = cpl4_d(this + 0xc);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(p, cap.wrapping_mul(14) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(cap.wrapping_mul(14) as u32)
        };
        cpl4_por(this + 0xc, q);
        if q == 0 {
            cpl4_sem_espaco(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = cpl4_lembretes(this, pessoa, data, tipo as u8);
    for d in &lembretes[..k] {
        let n = cpl4_d(this + 0x14);
        let e = cpl4_d(this + 0xc).wrapping_add(n.wrapping_mul(14));
        cpl4_por(e, cpl4_d(pessoa));
        core::ptr::write_unaligned((e + 4) as *mut [u8; 8], *d);
        *((e + 0xd) as *mut u8) = 2;
        cpl4_por(this + 0x14, n.wrapping_add(1));
    }
    let n = cpl4_d(this + 0x14);
    let e = cpl4_d(this + 0xc).wrapping_add(n.wrapping_mul(14));
    cpl4_por(e, cpl4_d(pessoa));
    *((e + 0xc) as *mut u8) = fatia as u8;
    core::ptr::write_unaligned((e + 4) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    *((e + 0xd) as *mut u8) = tipo as u8;
    cpl4_por(this + 0x14, n.wrapping_add(1)); // a data da decisão
}

// ponytail: as três realocam (e a 0x4c7950 gera o contrato pela 0x85e960); não rodam duas vezes. O comparador monta
// o plano antes, roda só a original e confere; o modo Rust e a carreira provam as cópias.

/// Comparador: a vaga esperada (a primeira vazia, ou a do fim com a conta + 1 e a capacidade + 200 se cheio).
unsafe extern "thiscall" fn comparar_vaga_livre_de_contrato(this: usize, so_no_fim: u32) -> i32 {
    let (n, cap, p) = (cpl4_d(this + 8), cpl4_d(this + 4), cpl4_d(this));
    let mut livre = -1i32;
    if so_no_fim == 0 {
        for i in 0..(n as i32).max(0) {
            if cpl4_d(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                livre = i;
                break;
            }
        }
    }
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> i32>(913)(this, so_no_fim);
    if r == -1 && cpl4_d(this) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let (er, en, ecap) = if livre >= 0 { (livre, n, cap) } else { (n as i32, n.wrapping_add(1), if n == cap { cap.wrapping_add(200) } else { cap }) };
    PLACAR.registrar(r == er && cpl4_d(this + 8) == en && cpl4_d(this + 4) == ecap, || {
        format!("vaga_livre_de_contrato({}) devolveu {} (esperava {}), conta {} -> {} (esperava {}), capacidade {} -> {} (esperava {})",
            so_no_fim, r, er, n, cpl4_d(this + 8), en, cap, cpl4_d(this + 4), ecap)
    });
    r
}

/// Comparador: a vaga é a do fim (a conta de antes); confere a ligação da pessoa, o id no contrato e a conta.
unsafe extern "thiscall" fn comparar_acrescentar_contrato_da_pessoa(this: usize, pessoa: usize, valor: u32) -> u32 {
    let n = cpl4_d(this + 8);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(914)(this, pessoa, valor);
    if cpl4_d(this) == 0 {
        return r; // sem memória
    }
    let id = cpl4_d(pessoa);
    let ligada = cpl4_d(cpl4_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f)));
    let no_contrato = cpl4_d(cpl4_d(this).wrapping_add(n.wrapping_mul(0x50)));
    PLACAR.registrar(ligada == n && no_contrato == id && cpl4_d(this + 8) == n.wrapping_add(1), || {
        format!("acrescentar_contrato_da_pessoa(pessoa {}, valor {}) vaga {} (esperava {}), id no contrato {}, conta {} -> {}",
            id, valor, ligada, n, no_contrato, n, cpl4_d(this + 8))
    });
    r
}

/// Comparador: os lembretes e a entrada esperados antes da original; confere a conta e cada entrada nova.
unsafe extern "thiscall" fn comparar_acrescentar_data_de_decisao(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    let n = cpl4_d(this + 0x14);
    let (lembretes, k) = cpl4_lembretes(this, pessoa, data, tipo as u8);
    let d = core::ptr::read_unaligned(data as *const [u8; 8]);
    let id = cpl4_d(pessoa);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(915)(this, pessoa, data, fatia, tipo);
    if cpl4_d(this + 0xc) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let v = cpl4_d(this + 0xc);
    let e = |j: usize| v.wrapping_add(n.wrapping_add(j).wrapping_mul(14));
    let data_de = |j: usize| core::ptr::read_unaligned((e(j) + 4) as *const [u8; 8]);
    let mut certo = cpl4_d(this + 0x14) == n.wrapping_add(k + 1);
    for (j, l) in lembretes[..k].iter().enumerate() {
        certo &= cpl4_d(e(j)) == id && data_de(j) == *l && *((e(j) + 0xd) as *const u8) == 2;
    }
    certo &= cpl4_d(e(k)) == id && data_de(k) == d && *((e(k) + 0xc) as *const u8) == fatia as u8 && *((e(k) + 0xd) as *const u8) == tipo as u8;
    PLACAR.registrar(certo, || {
        format!("acrescentar_data_de_decisao(pessoa {}, fatia {}, tipo {}) conta {} -> {} (esperava {}), lembretes {:?}, entradas {:?}",
            id, fatia as u8, tipo as u8, n, cpl4_d(this + 0x14), n + k + 1, &lembretes[..k],
            (0..=k).map(|j| core::ptr::read_unaligned(e(j) as *const [u8; 14])).collect::<Vec<_>>())
    });
}

// ---------- 248a leva: o contrato da comissão: montar a proposta pelo contrato atual, zerar a moral do contrato novo e a faixa do vencimento (staff_contracts.cpp) ----------

unsafe fn cpl5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl5_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpl5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpl5_por_b(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// A parte comum da 0x866ee0 e da 0x85e590 (inline nesta): zera a moral (+0x44), o estado +0x4e e as queixas (+0x45),
/// limpa os vínculos de opinião da pessoa do contrato (0x549c90 com o registro [0xae23bc] + id × 0x6e), zera +0x40,
/// põe 4 em +0x4e, zera a felicidade (+0x3f) e +0x3e; com o clube e a nação dele (+0x53), o nível do idioma da nação
/// (+0x77) para a pessoa (0x7c6830 em [0xb74350], sem o contrato; no máximo 10) nos 4 bits baixos de +0x4f.
unsafe fn cpl5_zerar_moral(this: usize, pessoa: usize, clube: usize) {
    crate::comparacao::ramo("cpl_5 moral zerada");
    cpl5_por_b(this + 0x44, 0);
    cpl5_por_b(this + 0x4e, 0);
    cpl5_por(this + 0x45, 0);
    let limpar: unsafe extern "C" fn(usize) = core::mem::transmute(0x549c90usize);
    limpar(cpl5_d(0xae23bc).wrapping_add(cpl5_d(this).wrapping_mul(0x6e)));
    cpl5_por(this + 0x40, 0);
    cpl5_por_b(this + 0x4e, 4);
    cpl5_por_b(this + 0x3f, 0); // a felicidade
    cpl5_por_b(this + 0x3e, 0);
    if clube != 0 && cpl5_d(clube + 0x53) != 0 {
        let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x7c6830usize);
        let r = fluencia(cpl5_d(0xb74350), pessoa, clube, cpl5_b(cpl5_d(clube + 0x53) + 0x77) as u32, 0) as i8;
        let c = if r > 10 { 10 } else { r } as u8;
        let b = cpl5_b(this + 0x4f);
        cpl5_por_b(this + 0x4f, (b & 0xf0) | (c & 0xf));
    }
}

/// { 0x866ee0 } (thiscall, ret 8, staff_contracts.cpp): `STAFF_CONTRACT::reset_morale_state(pessoa, clube)`: o
/// `cpl5_zerar_moral`. Pela assinatura do contrato (CONTRACT_MANAGER::sign_contract 0x4cf720, nos termos que
/// depois são copiados para o contrato guardado).
unsafe extern "thiscall" fn zerar_moral_do_contrato(this: usize, pessoa: usize, clube: usize) {
    cpl5_zerar_moral(this, pessoa, clube);
}

/// { 0x85e590 } (thiscall, ret 0x18, staff_contracts.cpp): `STAFF_CONTRACT::init_from_current_contract(pessoa, clube,
/// fim, percentual, sem_acesso, multa)`: a proposta de transferência ou empréstimo pelo contrato atual da pessoa
/// (0x4cf5a0 em [0xae1050]; sem contrato, caixa 0xf5 e nada): id da pessoa +0, clube +4, 0 em +8, salário +0xc =
/// salário atual × percentual (com sinal) / 100, a data de hoje (0xae2c90) em +0x25, o fim em +0x2d, os bits
/// +0x35 (& 0xc6 | 6), o salário também em +0x36, 0 em +0x3a e +0x49, −1 em +0x4a, o `cpl5_zerar_moral`, 0 em
/// +0x3b (16 bits), +0x1c..=+0x20 e +0x21, −1 em +0x10/+0x14/+0x18 e as cláusulas: +0x1c sem acesso, +0x1d multa
/// mínima. As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn contrato_pelo_atual(this: usize, pessoa: usize, clube: usize, fim: usize, percentual: u32, sem_acesso: u32, multa: u32) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpl5_d(0xae1050), pessoa);
    if c == 0 {
        aviso_em(0xa8360c, 0xf5);
        return;
    }
    crate::comparacao::ramo("cpl_5 contrato pelo atual");
    cpl5_por(this, cpl5_d(pessoa));
    cpl5_por(this + 8, 0);
    cpl5_por(this + 4, cpl5_d(clube));
    let salario = (cpl5_d(c + 0xc) as i32).wrapping_mul(percentual as u8 as i8 as i32) / 100; // o percentual do salário
    cpl5_por(this + 0xc, salario as usize);
    core::ptr::write_unaligned((this + 0x25) as *mut [u8; 8], core::ptr::read_unaligned(0xae2c90 as *const [u8; 8])); // hoje
    core::ptr::write_unaligned((this + 0x2d) as *mut [u8; 8], core::ptr::read_unaligned(fim as *const [u8; 8])); // o fim do contrato
    let b = (cpl5_b(this + 0x35) & 0xc6) | 6;
    cpl5_por(this + 0x36, cpl5_d(this + 0xc));
    cpl5_por_b(this + 0x35, b);
    cpl5_por_b(this + 0x3a, 0);
    cpl5_por_b(this + 0x49, 0);
    cpl5_por(this + 0x4a, usize::MAX);
    cpl5_zerar_moral(this, pessoa, clube);
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    for k in 0x1c..=0x20 {
        cpl5_por_b(this + k, 0);
    }
    cpl5_por(this + 0x10, usize::MAX);
    cpl5_por(this + 0x14, usize::MAX);
    cpl5_por(this + 0x18, usize::MAX);
    cpl5_por(this + 0x21, 0);
    cpl5_por_b(this + 0x1c, sem_acesso as u8);
    cpl5_por_b(this + 0x1d, multa as u8);
}

/// { 0x867d30 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::get_expiry_band()`: a faixa 0..=14 do fim do contrato
/// (+0x2d) contra as datas de hoje (0xae2c90) e as 16 datas do CONTRACT_MANAGER (0xae1010..0xae1090, montadas na
/// criação): tipo (+0x35 & 0x3f) 1 ou fim já passado, 14; tipo diferente de 2, 0; depois de [0xae1090], 0; os
/// intervalos entre 0xae1040/0xae1060/0xae1010/0xae1058/0xae1030 dão os pares 1-2, 3-4, 5-6, 7-8, 9-10 (o ímpar no dia
/// exato do limite de cima: 0x4a7f20) e, antes de 0xae1030, 11 no dia, 13 no dia de 0xae1048, senão 12. "Depois"
/// pela 0x456740. Chamada a cada contrato no passe diário (0x4c7ac0): sem contador.
unsafe extern "thiscall" fn faixa_do_vencimento(this: usize) -> u8 {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let mesmo_dia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let fim = this + 0x2d;
    let m = |g: usize| depois(fim, cpl5_d(g) as u32, cpl5_d(g + 4) as u32) != 0;
    let q = |g: usize| mesmo_dia(fim, cpl5_d(g) as u32, cpl5_d(g + 4) as u32) != 0;
    if cpl5_b(this + 0x35) & 0x3f == 1 {
        return 14;
    }
    if !m(0xae2c90) {
        return 14;
    }
    if cpl5_b(this + 0x35) & 0x3f != 2 {
        return 0;
    }
    if m(0xae1090) {
        return 0;
    }
    if m(0xae1010) { // depois do limite do meio
        if m(0xae1060) {
            if m(0xae1040) {
                return if q(0xae1090) { 1 } else { 2 };
            }
            return if q(0xae1040) { 3 } else { 4 };
        }
        return if q(0xae1060) { 5 } else { 6 };
    }
    if m(0xae1030) {
        if m(0xae1058) {
            return if q(0xae1010) { 7 } else { 8 };
        }
        return if q(0xae1058) { 9 } else { 10 };
    }
    if q(0xae1030) {
        return 11;
    }
    if q(0xae1048) { 13 } else { 12 }
}

/// As regiões que as duas primeiras gravam: o contrato (0x50 bytes), o registro de 0x4f bytes da pessoa de id `id`
/// em [0xae2d0c] (os vínculos da 0x549c90, se o id está na tabela) e [0xb67a34] (a caixa).
unsafe fn cpl5_regioes(this: usize, id: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(this, 0x50), (0xb67a34, 4)];
    let id = cpl5_d(cpl5_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))) as i32;
    if id >= 0 && id < cpl5_d(0xae2364) as i32 {
        r.push((cpl5_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)), 0x4f));
    }
    r
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa (o idioma 0x7c6830 só lê).
unsafe extern "thiscall" fn comparar_zerar_moral_do_contrato(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(916);
    let regioes = cpl5_regioes(this, cpl5_d(this));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa, clube), || zerar_moral_do_contrato(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("zerar_moral_do_contrato(pessoa {}) {:x?} x {:x?}", cpl5_d(pessoa), mo, mr));
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa; sem contrato atual (a caixa), só a original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_contrato_pelo_atual(this: usize, pessoa: usize, clube: usize, fim: usize, percentual: u32, sem_acesso: u32, multa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(917);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    if contrato_de(cpl5_d(0xae1050), pessoa) == 0 {
        return o(this, pessoa, clube, fim, percentual, sem_acesso, multa);
    }
    let regioes = cpl5_regioes(this, cpl5_d(pessoa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, fim, percentual, sem_acesso, multa),
        || contrato_pelo_atual(this, pessoa, clube, fim, percentual, sem_acesso, multa));
    PLACAR.registrar(igual, || {
        format!("contrato_pelo_atual(pessoa {}, percentual {}, {}, {}) {:x?} x {:x?}", cpl5_d(pessoa), percentual as u8 as i8, sem_acesso as u8, multa as u8, mo, mr)
    });
}

/// Comparador: só lê; compara a faixa.
unsafe extern "thiscall" fn comparar_faixa_do_vencimento(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(918)(this);
    let n = faixa_do_vencimento(this);
    PLACAR.registrar(r == n, || format!("faixa_do_vencimento(pessoa {}) original={} rust={}", cpl5_d(this), r, n));
    r
}

// ---------- 248a leva: o registro de histórias da mídia: registrar uma história, expirar as antigas e se a história já foi contada ----------

unsafe fn cpl6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl6_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpl6_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A data `a` é anterior à `b`? Como a original: a palavra +2 sem sinal e, igual, a palavra +0 com sinal.
unsafe fn cpl6_antes(a: usize, b: usize) -> bool {
    let (x, y) = (cpl6_w(a + 2), cpl6_w(b + 2));
    if x != y {
        return x < y;
    }
    (cpl6_w(a) as i16) < (cpl6_w(b) as i16)
}

/// O id (+0) do sujeito, ou −1 sem ele.
unsafe fn cpl6_id(p: usize) -> usize {
    if p != 0 { cpl6_d(p) } else { usize::MAX }
}

/// Os 23 bytes que a 0x757a50 grava no registro (data de hoje [0xae2c90], ids, resposta, subtipo, tipo).
unsafe fn cpl6_registro(a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) -> [u8; 0x17] {
    let mut r = [0u8; 0x17];
    r[..8].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    r[8..0xc].copy_from_slice(&(cpl6_id(a) as u32).to_le_bytes());
    r[0xc..0x10].copy_from_slice(&(cpl6_id(b) as u32).to_le_bytes());
    r[0x10] = resposta as u8;
    r[0x11..0x13].copy_from_slice(&(subtipo as u8 as i8 as i16).to_le_bytes());
    r[0x13..].copy_from_slice(&tipo.to_le_bytes());
    r
}

unsafe fn cpl6_ordenar(base: usize, n: usize) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(base, n as u32, 0x17, 0x757ee0);
}

/// { 0x757a50 } (thiscall, ret 0x14, media.cpp): `Media::log_story(a, b, tipo, subtipo, resposta)`: acrescenta ao
/// registro de histórias ([this], capacidade +4, conta +8; registros de 23 bytes) a data de hoje (+0, 0x417c10 de
/// [0xae2c90]), o id de `a` (+8) e o de `b` (+0xc) (−1 sem eles), o `tipo` (+0x13), o `subtipo` (byte com sinal, em
/// palavra +0x11) e a `resposta` (+0x10); a conta sobe e o registro é reordenado (qsort 0x945333 com 0x757ee0). Sem
/// registro, aloca 20 (0x944e46; sem memória, caixa 0x6e11 e fim); cheio, cresce 20 (realloc 0x945667; caixa 0x6e1c).
unsafe extern "thiscall" fn registrar_historia_da_midia(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) {
    crate::comparacao::ramo("cpl_6 registrar historia");
    if cpl6_d(this) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let p = alocar(0x1cc);
        cpl6_por(this, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6e11);
            erro_fatal();
        }
        cpl6_por(this + 4, 0x14);
    }
    let cap = cpl6_d(this + 4);
    if cap == cpl6_d(this + 8) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpl6_d(this), cap.wrapping_add(0x14).wrapping_mul(0x17) as u32);
        cpl6_por(this, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6e1c);
            erro_fatal();
        }
        cpl6_por(this + 4, cpl6_d(this + 4).wrapping_add(0x14));
    }
    let item = || cpl6_d(this).wrapping_add(cpl6_d(this + 8).wrapping_mul(0x17));
    cpl6_por(item() + 8, cpl6_id(a));
    cpl6_por(item() + 0xc, cpl6_id(b));
    cph3_data(item(), 0xae2c90);
    cpl6_por(item() + 0x13, tipo as usize);
    core::ptr::write_unaligned((item() + 0x11) as *mut i16, subtipo as u8 as i8 as i16);
    *((item() + 0x10) as *mut u8) = resposta as u8;
    let n = cpl6_d(this + 8).wrapping_add(1);
    cpl6_por(this + 8, n); // mais uma história
    cpl6_ordenar(cpl6_d(this), n);
}

/// { 0x757c70 } (thiscall, ret 0, media.cpp): `Media::expire_story_log()`: cada história do registro anterior ao
/// limite (hoje − 100 dias; a do time, tipo 0xbdf, hoje − (40 + sorteio(40)), um sorteio 0x90cfc0 por história dessas
/// a cada passada; datas da 0x54c8e0 sobre [0xae2c90]) recebe por cima a última (23 bytes), a conta desce e o mesmo
/// lugar é visto de novo. Sobrando alguma, reordena (qsort com 0x757ee0). Sem registro, nada.
unsafe extern "thiscall" fn expirar_historias_da_midia(this: usize) {
    if cpl6_d(this) == 0 {
        return;
    }
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut i = 0i32;
    while i < cpl6_d(this + 8) as i32 {
        let r = cpl6_d(this).wrapping_add((i as usize).wrapping_mul(0x17));
        let dias = if cpl6_d(r + 0x13) == 0xbdf {
            crate::comparacao::ramo("cpl_6 expirar historia do time");
            let s = sorteio(0x28); // o sorteio da história do time
            (s + 0x28) as u32
        } else {
            100 // os dias das outras histórias
        };
        let mut local = [0u8; 8];
        let limite = datar(0xae2c90, &mut local, dias);
        if cpl6_antes(r, limite) {
            let n = cpl6_d(this + 8);
            let ultimo = cpl6_d(this).wrapping_add(n.wrapping_mul(0x17)).wrapping_sub(0x17);
            core::ptr::copy(ultimo as *const u8, r as *mut u8, 0x17);
            cpl6_por(this + 8, n.wrapping_sub(1));
            i -= 1; // o mesmo lugar de novo
        }
        i += 1;
    }
    if cpl6_d(this + 8) != 0 {
        cpl6_ordenar(cpl6_d(this), cpl6_d(this + 8));
    }
}

/// { 0x757da0 } (thiscall, ret 0x14, media.cpp): `Media::has_logged_story(a, b, tipo, subtipo, desde)`: a história do
/// time (tipo 0xbdf): procura em ordem a de tipo 0xbdf com o id de `b` (+0xc) e o `subtipo` (palavra +0x11), sem olhar
/// `a` nem a data: 1 se achou. As outras: busca binária (bsearch 0x946464 com 0x757ee0) pela chave {id de `a` ou −1,
/// id de `b`, tipo, subtipo} (a data padrão 0x54a110 na chave, que a comparação não lê); achou e sem `desde`, 1; com
/// `desde`, 1 se a história não é anterior a ela. Senão 0. `b` não pode ser nulo (a original lê sem conferir).
unsafe extern "thiscall" fn historia_ja_registrada(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, desde: usize) -> u32 {
    let mut chave = [0u8; 0x17];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(chave.as_mut_ptr() as usize);
    let s = subtipo as u16;
    if tipo == 0xbdf {
        crate::comparacao::ramo("cpl_6 historia do time ja contada");
        let n = cpl6_d(this + 8) as i32;
        for k in 0..n.max(0) as usize {
            let r = cpl6_d(this).wrapping_add(k * 0x17);
            if cpl6_d(r + 0x13) == 0xbdf && cpl6_d(r + 0xc) == cpl6_d(b) && cpl6_w(r + 0x11) == s {
                return 1; // a história do time já contada
            }
        }
        return 0;
    }
    chave[8..0xc].copy_from_slice(&(cpl6_id(a) as u32).to_le_bytes());
    chave[0x13..].copy_from_slice(&tipo.to_le_bytes());
    chave[0xc..0x10].copy_from_slice(&(cpl6_d(b) as u32).to_le_bytes());
    chave[0x11..0x13].copy_from_slice(&s.to_le_bytes());
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = bsearch(chave.as_ptr() as usize, cpl6_d(this), cpl6_d(this + 8) as u32, 0x17, 0x757ee0);
    if e == 0 {
        return 0;
    }
    if desde == 0 {
        return 1;
    }
    if cpl6_antes(e, desde) { 0 } else { 1 }
}

/// Comparador: a original pode alocar e realocar (não roda duas vezes): o registro esperado antes; confere a conta e
/// que o registro esperado está na lista (a ordem é a do qsort).
unsafe extern "thiscall" fn comparar_registrar_historia_da_midia(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, resposta: u32) {
    let n = cpl6_d(this + 8);
    let esperado = cpl6_registro(a, b, tipo, subtipo, resposta);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(919)(this, a, b, tipo, subtipo, resposta);
    let m = cpl6_d(this + 8);
    let achou = (0..m).any(|k| {
        core::slice::from_raw_parts(cpl6_d(this).wrapping_add(k * 0x17) as *const u8, 0x17) == &esperado[..]
    });
    PLACAR.registrar(m == n.wrapping_add(1) && achou, || {
        format!("registrar_historia_da_midia(tipo {:#x}, subtipo {}) conta {} -> {}, esperado {:x?} achou {}", tipo, subtipo as u8 as i8, n, m, esperado, achou)
    });
}

/// Comparador: lado a lado pelo registro (ponteiro, capacidade e conta; as histórias) e o estado do sorteio (a cópia
/// sem contar os sorteios).
unsafe extern "thiscall" fn comparar_expirar_historias_da_midia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(920);
    if cpl6_d(this) == 0 {
        return o(this);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0xc));
    reg.push((cpl6_d(this), (cpl6_d(this + 8) as i32).max(0) as usize * 0x17));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| expirar_historias_da_midia(this)));
    // mo.0[2] e mr.0[2]: ponteiro, capacidade e conta depois de cada lado
    PLACAR.registrar(igual, || format!("expirar_historias_da_midia {:x?} x {:x?}", mo.0[2].1, mr.0[2].1));
}

/// Comparador: pelo valor (a busca e a data padrão da chave só leem).
unsafe extern "thiscall" fn comparar_historia_ja_registrada(this: usize, a: usize, b: usize, tipo: u32, subtipo: u32, desde: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> u32>(921)(this, a, b, tipo, subtipo, desde);
    let n = historia_ja_registrada(this, a, b, tipo, subtipo, desde);
    PLACAR.registrar(r == n, || format!("historia_ja_registrada(tipo {:#x}, subtipo {}, desde {}) original={} rust={}", tipo, subtipo as u16 as i16, desde != 0, r, n));
    r
}

// ---------- 248a leva: os interesses de transferência da mídia: acrescentar um interesse e as comparações das chaves de ids e das histórias ----------

unsafe fn cpl7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl7_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A chave da 0x754740: {id do jogador, id do clube ou −1} (os 8 bytes que a 0x757ea0 lê).
unsafe fn cpl7_chave(jogador: usize, clube: usize) -> [u32; 2] {
    [cpl7_d(jogador) as u32, if clube != 0 { cpl7_d(clube) as u32 } else { u32::MAX }]
}

/// O interesse já está no vetor? (bsearch 0x946464 com 0x757ea0; sem vetor, não).
unsafe fn cpl7_ja_esta(this: usize, chave: &[u32; 2]) -> bool {
    let base = cpl7_d(this + 0xc);
    if base == 0 {
        return false;
    }
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(chave.as_ptr() as usize, base, cpl7_d(this + 0x14) as u32, 0x19, 0x757ea0) != 0
}

/// O valor que a 0x754740 grava: o `valor` limitado a 50.000.000.
fn cpl7_valor(valor: i32) -> i32 {
    if valor < 0x2faf080 { valor } else { 0x2faf080 }
}

/// { 0x754740 } (thiscall, ret 0x14, media.cpp): `Media::add_transfer_interest(jogador, clube, valor, avaliacao)`: com
/// o vetor de interesses ([this+0xc], capacidade +0x10, conta +0x14; registros de 25 bytes), procura (bsearch 0x946464
/// com 0x757ea0) a chave {id do jogador, id do clube ou −1}: já está, nada. Senão acrescenta: id do jogador (+0), id do
/// clube (+4, lido sem conferir), a data de hoje (+8, 0x417c10 de [0xae2c90]), 0 (+0x10, o boato ainda não saiu), o
/// `valor` até 50.000.000 (+0x11) e a `avaliacao` × 100 truncada (+0x15, _ftol 0x945640); a conta sobe e reordena (qsort
/// com 0x757ea0). Sem vetor, aloca 20 (0x944e46; sem memória, caixa 0x665d e fim); cheio, cresce 20 (realloc 0x945667;
/// caixa 0x6668). A data padrão (0x54a110) da chave na pilha, como a original.
unsafe extern "thiscall" fn acrescentar_interesse_de_transferencia(this: usize, jogador: usize, clube: usize, valor: i32, avaliacao: f64) {
    let mut data = [0u8; 8];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(data.as_mut_ptr() as usize);
    let chave = cpl7_chave(jogador, clube);
    if cpl7_ja_esta(this, &chave) {
        crate::comparacao::ramo("cpl_7 interesse repetido");
        return; // o interesse já está
    }
    crate::comparacao::ramo("cpl_7 interesse novo");
    if cpl7_d(this + 0xc) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let p = alocar(0x1f4);
        cpl7_por(this + 0xc, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x665d);
            erro_fatal();
        }
        cpl7_por(this + 0x10, 0x14);
    }
    let cap = cpl7_d(this + 0x10);
    if cap == cpl7_d(this + 0x14) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpl7_d(this + 0xc), cap.wrapping_add(0x14).wrapping_mul(0x19) as u32);
        cpl7_por(this + 0xc, p);
        if p == 0 {
            aviso_em(0xa17bec, 0x6668);
            erro_fatal();
        }
        cpl7_por(this + 0x10, cpl7_d(this + 0x10).wrapping_add(0x14));
    }
    let item = || cpl7_d(this + 0xc).wrapping_add(cpl7_d(this + 0x14).wrapping_mul(0x19));
    cpl7_por(item(), cpl7_d(jogador));
    cpl7_por(item() + 4, cpl7_d(clube));
    cph3_data(item() + 8, 0xae2c90);
    *((item() + 0x10) as *mut u8) = 0; // o boato ainda não saiu
    cpl7_por(item() + 0x11, cpl7_valor(valor) as usize);
    cpl7_por(item() + 0x15, ftol(avaliacao * 100.0) as usize); // [0x9688a8] = 100.0
    let n = cpl7_d(this + 0x14).wrapping_add(1);
    cpl7_por(this + 0x14, n); // mais um interesse
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpl7_d(this + 0xc), n as u32, 0x19, 0x757ea0);
}

/// { 0x757ea0 } (cdecl, media.cpp): `compare_id_pair_keys(a, b)` do qsort e do bsearch: nulo antes (`a` nulo −1, `b`
/// nulo 1); ids +0 diferentes, a diferença; iguais, 0 se um dos +4 é −1 (o coringa), senão a diferença dos +4.
unsafe extern "C" fn ordem_das_chaves_de_ids(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (x, y) = (cpl7_d(a) as i32, cpl7_d(b) as i32);
    if x != y {
        return x.wrapping_sub(y);
    }
    let (x, y) = (cpl7_d(a + 4) as i32, cpl7_d(b + 4) as i32);
    if x == -1 || y == -1 {
        return 0;
    }
    x.wrapping_sub(y)
}

/// { 0x757ee0 } (cdecl, media.cpp): `compare_story_log_records(a, b)` do qsort e do bsearch do registro de histórias:
/// nulo antes (`a` nulo −1, `b` nulo 1); pelo sujeito a (+8), o sujeito b (+0xc) e o tipo (+0x13), a diferença do
/// primeiro diferente; iguais, a diferença dos subtipos (palavras +0x11 com sinal).
unsafe extern "C" fn ordem_das_historias(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    for o in [8usize, 0xc, 0x13] {
        let (x, y) = (cpl7_d(a + o) as i32, cpl7_d(b + o) as i32);
        if x != y {
            return x.wrapping_sub(y);
        }
    }
    let s = |p: usize| core::ptr::read_unaligned((p + 0x11) as *const i16) as i32;
    s(a) - s(b)
}

/// Os 25 bytes que a 0x754740 acrescenta.
unsafe fn cpl7_interesse(jogador: usize, clube: usize, valor: i32, avaliacao: f64) -> [u8; 0x19] {
    let mut r = [0u8; 0x19];
    r[..4].copy_from_slice(&(cpl7_d(jogador) as u32).to_le_bytes());
    r[4..8].copy_from_slice(&(cpl7_d(clube) as u32).to_le_bytes());
    r[8..0x10].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    r[0x11..0x15].copy_from_slice(&cpl7_valor(valor).to_le_bytes());
    r[0x15..].copy_from_slice(&ftol(avaliacao * 100.0).to_le_bytes());
    r
}

/// Comparador: a original pode alocar e realocar (não roda duas vezes): o plano antes (já está? o registro novo);
/// confere a conta e que o registro novo está no vetor (a ordem é a do qsort).
unsafe extern "thiscall" fn comparar_acrescentar_interesse_de_transferencia(this: usize, jogador: usize, clube: usize, valor: i32, avaliacao: f64) {
    let n = cpl7_d(this + 0x14);
    let ja_esta = cpl7_ja_esta(this, &cpl7_chave(jogador, clube));
    let esperado = if ja_esta { None } else { Some(cpl7_interesse(jogador, clube, valor, avaliacao)) };
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, i32, f64)>(922)(this, jogador, clube, valor, avaliacao);
    let m = cpl7_d(this + 0x14);
    let ok = match &esperado {
        None => m == n,
        Some(e) => m == n.wrapping_add(1) && (0..m).any(|k| {
            core::slice::from_raw_parts(cpl7_d(this + 0xc).wrapping_add(k * 0x19) as *const u8, 0x19) == &e[..]
        }),
    };
    PLACAR.registrar(ok, || format!("acrescentar_interesse_de_transferencia(valor {}, avaliacao {}) conta {} -> {}, esperado {:x?}", valor, avaliacao, n, m, esperado));
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_das_chaves_de_ids(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(923)(a, b);
    let n = ordem_das_chaves_de_ids(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_chaves_de_ids({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_das_historias(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(924)(a, b);
    let n = ordem_das_historias(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_historias({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
