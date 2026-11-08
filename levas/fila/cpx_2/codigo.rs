// ---------- %%LEVA%%a leva: o jogador desassossegado pelo lance de um clube maior, a notícia da oferta desfeita e a do prêmio por jogos (transfer_manager.cpp) ----------

unsafe fn cpx2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx2_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpx2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx2_clube(id: usize) -> usize {
    cpx2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx2_pessoa(id: usize) -> usize {
    cpx2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpx2_contrato_de(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx2_d(0xae1050), pessoa)
}

/// O mesmo, guardado em [0xae1080] como a original faz.
unsafe fn get_contract_store_lookup(pessoa: usize) -> usize {
    let c = cpx2_contrato_de(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O ensaio do comparador: a cópia para antes da queixa (0x867700), da fuga (0x419840) e das antipatias (0x547e70,
/// 0x547af0), que mexem no resto do jogo, e marca que chegou lá.
static mut UNSETTLE_PLAYER_DRY_RUN: bool = false;
static mut UNSETTLE_PLAYER_REACHED: bool = false;

/// { 0x8c1290 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::unsettle_player_after_bid(oferta)`, pela
/// recusa (0x8bf680), pela retirada no motivo 2 (0x8bfcb0) e pelo clube que recusa (0x8c36b0). Sorteio(5750) + 4250
/// abaixo da reputação do comprador (+0x80): +0x2d da oferta = 1. Com +0x2d = 1, clube do contrato, comprador e ficha: o
/// interesse x (0x846920 pelo estatuto 0x849f00) e a nota n = (8 × ambição² − 1600) − reputação / 40 − min(x⁴, 300 ×
/// x) − 2 × lealdade²... (os atributos +0x5d, +0x57, +0x5b, +0x59 pela pow do jogo); listado (+0x4e & 8): − 2000 −
/// sorteio(6000). Com x de 5,5, n < −sorteio(700), (n < −4000 ou +0x5b + +0x59 < 15), reputação acima de 3500 e o
/// valor ponderado da oferta (0x8e7b50) acima do menor entre 1,5 × valor (0x4d0c60) e 0,8 × preço (0x4d1600): a queixa
/// 0x80 (0x867700); jogador de 5250, idade 19 a 31, n < −8000, com o critério (0x866f60), sem a marca +1 do registro
/// +0x86e e presente (0x418f60): some (0x419840) e, com +0x44 − sorteio(200) + sorteio(400) < −150, rejeita o clube
/// (0x547e70). x acima de 10, n < −5000 ou o menor abaixo de 0,75 × o ponderado: antipatia ao técnico (0x547af0).
unsafe extern "thiscall" fn unsettle_player_after_bid(this: usize, o: usize) {
    let p = cpx2_pessoa(cpx2_d(o + 0xc));
    let c = get_contract_store_lookup(p);
    let do_contrato = if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    let comprador = cpx2_clube(cpx2_d(o + 8));
    let r = random(0x1676);
    crate::comparison::branch("cpx_2 unsettling draw");
    if (cpx2_w(comprador + 0x80) as i32) > r.wrapping_add(0x109a) { // cpx2: o sorteio do desassossego
        *((o + 0x2d) as *mut u8) = 1;
    }
    if cpx2_b(o + 0x2d) != 1 || do_contrato == 0 || comprador == 0 || cpx2_d(p + 0x61) == 0 {
        return;
    }
    let c2 = cpx2_contrato_de(p);
    let listas = cpx2_d(this + 0x856);
    let estatuto: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(0x849f00usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
    let e = estatuto(listas, p, comprador, 0);
    // ponytail: a original empurra o dword da vaga onde guardou o byte (o resto é sobra de pilha); a 0x846920 só lê o
    // byte
    let x = interesse(listas, p, comprador, 1, 0, e as u32, 0);
    let pw = game_pow(x, read_f64_at(0x968938));
    let a = x * read_f64_at(0x96a6b0);
    let v = if !(pw >= a) { pw } else { a };
    let mut n = match_pl_ftol(game_pow(cpx2_sb(p + 0x5d) as f64, read_f64_at(0x968950)) * read_f64_at(0x968f10) - read_f64_at(0x96a590)) as i16;
    let ficha = cpx2_d(p + 0x61);
    n = n.wrapping_sub((cpx2_w(ficha + 0xb) as i32 / 40) as i16);
    n = n.wrapping_sub(match_pl_ftol(v) as i16);
    n = n.wrapping_sub(match_pl_ftol(game_pow(cpx2_sb(p + 0x57) as f64, read_f64_at(0x968950))) as i16);
    n = n.wrapping_add(match_pl_ftol(game_pow(cpx2_sb(p + 0x5b) as f64, read_f64_at(0x968930))) as i16);
    n = n.wrapping_add(match_pl_ftol(game_pow(cpx2_sb(p + 0x59) as f64, read_f64_at(0x968930))) as i16);
    if c2 == 0 {
        return;
    }
    if cpx2_b(c2 + 0x4e) & 8 != 0 {
        crate::comparison::branch("cpx_2 listed");
        n = n.wrapping_add((-2000i32).wrapping_sub(random(0x1770)) as i16); // cpx2: o desconto do listado
    }
    crate::comparison::branch("cpx_2 interest evaluated");
    if !(x >= read_f64_at(0x96d298)) { // cpx2: o interesse mínimo
        return;
    }
    if n as i32 >= -random(0x2bc) {
        return;
    }
    if n >= -4000 && cpx2_sb(p + 0x5b) + cpx2_sb(p + 0x59) >= 0xf {
        return;
    }
    if cpx2_w(cpx2_d(p + 0x61) + 0xb) <= 0xdac {
        return;
    }
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let a1 = match_pl_ftol(valor(cpx2_d(0xae1050), p, do_contrato, 0, -1, 0) as f64 * read_f64_at(0x968940));
    let b1 = match_pl_ftol(preco(cpx2_d(0xae1050), do_contrato, comprador, p, 0, 1, 1) as f64 * read_f64_at(0x9689b0));
    let pond = ponderado(o, 0, 0, 0, 1, 1);
    let menor = if a1 < b1 { a1 } else { b1 };
    if pond < menor {
        return;
    }
    if UNSETTLE_PLAYER_DRY_RUN {
        UNSETTLE_PLAYER_REACHED = true;
        return;
    }
    crate::comparison::branch("cpx_2 unsettled");
    let queixa: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
    queixa(c2, 0x80, 0);
    let idade = cpx2_sb(p + 0x18);
    if cpx2_w(cpx2_d(p + 0x61) + 0xb) > 0x1482 && idade > 0x12 && idade < 0x20 && n < -8000 {
        let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
        let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
        if criterio(c2) != 0
            && cpx2_b(cpx2_d(this + 0x86e).wrapping_add(cpx2_d(p).wrapping_mul(8)) + 1) == 0
            && ausente(cpx2_d(0xad9c6c), p, 0) == 0
        {
            let sumir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x419840usize);
            sumir(cpx2_d(0xad9c6c), p, comprador, 1);
            let forca = cpx2_sb(c2 + 0x44);
            let r1 = random(0xc8);
            let r2 = random(0x190);
            if forca.wrapping_sub(r1).wrapping_add(r2) < -150 {
                let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
                rejeitar(p, do_contrato, 1);
            }
        }
    }
    if x > read_f64_at(0x9688b0) || n < -5000 || (menor as f64) < pond as f64 * read_f64_at(0x969040) {
        let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
        desafeto(p, cpx2_d(do_contrato + 0xcf), 1);
    }
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado (a oferta, [0xae1080] e o sorteio). Sem chegar
/// à queixa (quase sempre), tudo igual; chegando, só a original (a queixa, a fuga e as antipatias não podem rodar duas
/// vezes e a original sorteia mais).
unsafe extern "thiscall" fn compare_unsettle_player_after_bid(this: usize, o: usize) {
    let orig_f = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let mut regioes = vec![(o, 0x30usize), (0xae1080, 4)];
    regioes.extend(crate::rng::STATE_REGIONS);
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    UNSETTLE_PLAYER_DRY_RUN = true;
    UNSETTLE_PLAYER_REACHED = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| unsettle_player_after_bid(this, o)));
    UNSETTLE_PLAYER_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    orig_f(this, o);
    if UNSETTLE_PLAYER_REACHED {
        return;
    }
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    SCOREBOARD.record(mo == mr, || format!("unsettle_player_after_bid(oferta {:#x}) original={:x?} rust={:x?}", o, mo, mr));
}

/// { 0x8de790 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_collapsed_news(oferta, clube, motivo)`, pela oferta de contrato aceita (0x8c38b0), pela oferta desfeita
/// (0x8c44e0), pelas permissões de trabalho (0x8c7690) e por 0x8c4950. Os clubes humanos que recebem (0x8d1130 com 0x2f,
/// até 30); sem nenhum, nada. A notícia 0xfb0: o clube do contrato (sem contrato de tipo 5), o comprador, a pessoa (o id
/// e as três chaves), o clube, o motivo e +0x1f; a cada destinatário, menos o próprio clube no motivo 0xb.
unsafe extern "thiscall" fn post_offer_collapsed_news(this: usize, o: usize, clube: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpx2_pessoa(cpx2_d(o + 0xc));
    let c = get_contract_store_lookup(p);
    let mut do_contrato = if c == 0 || cpx2_b(c + 0x35) & 0x3f == 5 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    let comprador = cpx2_clube(cpx2_d(o + 8));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x2f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    if do_contrato != 0 && cpx2_b(cpx2_contrato_de(p) + 0x35) & 0x3f == 5 {
        do_contrato = 0;
    }
    crate::comparison::branch("cpx_2 offer undone news");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb0, 0);
    gravar(nt, 0, if do_contrato != 0 { cpx2_d(do_contrato) as u32 } else { u32::MAX });
    gravar(nt, 1, cpx2_d(comprador) as u32);
    gravar(nt, 2, cpx2_d(p) as u32);
    gravar(nt, 3, cpx2_d(cpx2_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx2_d(cpx2_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx2_d(cpx2_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 6, if clube != 0 { cpx2_d(clube) as u32 } else { u32::MAX });
    gravar(nt, 7, motivo as i16 as u32);
    gravar(nt, 8, cpx2_b(o + 0x1f) as i8 as u32);
    for &d in lista.iter().take(n.max(0) as usize) {
        if d != clube || motivo as u16 != 0xb {
            entregar(cpx2_d(0xb7431c), nt, d);
        }
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_offer_collapsed_news(this: usize, o: usize, clube: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I1%%)(this, o, clube, motivo)
}

/// { 0x8db8a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_appearance_money_news(cláusula, valor, tipo, calado)`, pelo prêmio por jogos (0x8cb340). Calado, sem contrato ou
/// sem o clube dele: nada. Senão a notícia 0xfa6: a pessoa da cláusula (+0: o id e as três chaves), +4, o valor, o
/// clube que paga (+9), o tipo e o clube do contrato; ao clube que paga e ao clube do contrato.
unsafe extern "thiscall" fn post_appearance_money_news(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    let _ = this;
    if calado != 0 {
        return;
    }
    let p = cpx2_pessoa(cpx2_d(clausula));
    let c = get_contract_store_lookup(p);
    if c == 0 || cpx2_clube(cpx2_d(c + 4)) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cpx_2 appearances bonus news");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa6, 0);
    gravar(nt, 0, cpx2_d(p) as u32);
    gravar(nt, 1, cpx2_d(cpx2_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpx2_d(cpx2_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpx2_d(cpx2_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpx2_d(clausula + 4) as u32);
    gravar(nt, 5, valor);
    gravar(nt, 6, cpx2_d(clausula + 9) as u32);
    gravar(nt, 7, tipo as i8 as u32);
    let c = get_contract_store_lookup(p);
    // a original lê o id do clube do contrato mesmo sem contrato (o registro 0): nunca acontece aqui
    let clube = if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    gravar(nt, 8, cpx2_d(clube) as u32);
    entregar(cpx2_d(0xb7431c), nt, cpx2_clube(cpx2_d(clausula + 9)));
    let c = get_contract_store_lookup(p);
    entregar(cpx2_d(0xb7431c), nt, if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) });
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_appearance_money_news(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I2%%)(this, clausula, valor, tipo, calado)
}
