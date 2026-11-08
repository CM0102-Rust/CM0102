// ---------- %%LEVA%%a leva: a recusa dos termos pelo jogador, a notícia da confiança da diretoria e a das expectativas da diretoria (transfer_manager.cpp, manager_manager.cpp) ----------

unsafe fn cpv2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cpv2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv2_clube(id: usize) -> usize {
    cpv2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv2_pessoa(id: usize) -> usize {
    cpv2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpv2_contrato(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpv2_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
unsafe fn manager_manager_contract_club(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || cpv2_b(c + 0x35) & 0x3f != 5) {
        cpv2_clube(cpv2_d(c + 4))
    } else {
        0
    }
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
unsafe fn cpv2_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpv2_d(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
unsafe fn cpv2_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// A oferta já resolvida pelo status (+0x2c): 2, 3, 4, 0x10 ou 0x14, ou 0xf com +0x2e ≥ 3.
unsafe fn is_offer_resolved(o: usize) -> bool {
    match cpv2_b(o + 0x2c) {
        2 | 3 | 4 | 0x10 | 0x14 => true,
        0xf => cpv2_b(o + 0x2e) >= 3,
        _ => false,
    }
}

/// { 0x8c28c0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::handle_player_rejected_terms(pessoa,
/// índice, silencioso)`: índice fora do vetor [this] (conta +4): a caixa "Error" (0xf49, 0xf4e). Vaga vazia: nada.
/// Se a pessoa é o jogador da oferta (+0xc): termos recusados (+0x10 = 4), status 0xf; +0x2e sobe com comprador humano
/// (0x600a50 sem as férias) e +0x1f ≠ 1, senão 3; saem as marcas de lance (0x84aeb0) do comprador e do dono de cada
/// jogador da troca; o gerente de listas anota a recusa (0x843e10) se o comprador é do computador; reagendada em 10
/// dias (0x8c6220); resolvida com todas as rivais resolvidas, o bit 0 do registro de 8 bytes do jogador ([this+0x86e])
/// sai; saem as propostas de contrato (0x4cca70) dos jogadores da troca com o vendedor. Sem `silencioso`: com comprador
/// mais famoso que o vendedor (+0x80), a reputação do jogador (+0xb da ficha +0x61) sobe a diferença × 0,075 (acima de
/// 7750 com ficha abaixo de 4750), × 0,05 (acima de 4250) ou × 0,025, até 10000; a notícia da resposta (0x8d8e90).
/// Se não, a pessoa é um jogador da troca (+4; lista vazia: a caixa 0x101f): sai da lista (0x68d1f0; a lista vazia é
/// liberada), o valor dela (0x4d0c60) passa ao preço (+0x1b), o registro de 8 bytes dela perde uma oferta (+2), a
/// notícia (sem `silencioso`), e a reação do vendedor do computador (0x8c8090) e do comprador (0x8c0810), parando se a
/// oferta ficou resolvida. No fim, a oferta segue (0x8c2f50).
unsafe extern "thiscall" fn handle_player_rejected_terms(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let anotar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(0x8d8e90usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let resposta_do_vendedor: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c8090usize);
    let resposta_do_comprador: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0810usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    if indice < 0 {
        return show_error_at(0xa8d514, 0xf49);
    }
    if indice >= cpv2_d(this + 4) as i32 {
        return show_error_at(0xa8d514, 0xf4e);
    }
    let vaga = || cpv2_d(this).wrapping_add(indice as usize * 4);
    let o = cpv2_d(vaga());
    if o == 0 {
        return;
    }
    let preco = cpv2_d(o + 0x1b);
    if cpv2_d(pessoa) == cpv2_d(o + 0xc) {
        cpv2_pb(o + 0x10, 4);
        let comprador = cpv2_clube(cpv2_d(o + 8));
        let vendedor = manager_manager_contract_club(cpv2_contrato(pessoa), true);
        cpv2_pb(o + 0x2c, 0xf);
        if silencioso == 0 && human_club(comprador, 1, 1, 0, 0) != 0 && cpv2_b(o + 0x1f) != 1 {
            cpv2_pb(o + 0x2e, cpv2_b(o + 0x2e).wrapping_add(1));
        } else {
            cpv2_pb(o + 0x2e, 3);
        }
        let p = cpv2_pessoa(cpv2_d(o + 0xc));
        let dono = manager_manager_contract_club(cpv2_contrato(p), false);
        limpar_marcas(cpv2_d(this + 0x856), cpv2_clube(cpv2_d(o + 8)), p);
        let l = cpv2_d(o + 4);
        if l != 0 {
            cpv2_inicio(l);
            loop {
                let y = cpv2_proximo(l);
                if y == 0 {
                    break;
                }
                limpar_marcas(cpv2_d(this + 0x856), dono, cpv2_pessoa(cpv2_d(y)));
            }
        }
        if human_club(comprador, 0, 1, 0, 0) == 0 {
            crate::comparison::branch("cpv_2 recusa anotada na lista");
            // a original testa o byte com o eax zerado pela pergunta de antes (em cima, zero)
            anotar(cpv2_d(this + 0x856), pessoa, comprador, 1, (cpv2_b(o + 0x11) != 0xb) as u32); // cpv2: a recusa na lista
        }
        crate::comparison::branch("cpv_2 recusa do jogador");
        agendar(this, o, 0xa, 0); // cpv2: de volta em 10 dias
        if is_offer_resolved(o) {
            let rivais = cpv2_d(o);
            let mut todas = true;
            if rivais != 0 {
                cpv2_inicio(rivais);
                loop {
                    let x = cpv2_proximo(rivais);
                    if x == 0 {
                        break;
                    }
                    if !is_offer_resolved(cpv2_d(cpv2_d(this) + cpv2_d(x) * 4)) {
                        todas = false;
                        break;
                    }
                }
            }
            if todas {
                let m = cpv2_d(this + 0x86e).wrapping_add(cpv2_d(o + 0xc) * 8);
                if cpv2_b(m) & 1 != 0 {
                    cpv2_pb(m, cpv2_b(m) & 0xfe);
                }
            }
        }
        let l = cpv2_d(o + 4);
        if l != 0 {
            cpv2_inicio(l);
            loop {
                let y = cpv2_proximo(l);
                if y == 0 {
                    break;
                }
                tirar_proposta(cpv2_d(0xae1050), cpv2_pessoa(cpv2_d(y)), vendedor);
            }
        }
        if silencioso == 0 {
            let ficha = cpv2_d(pessoa + 0x61);
            if comprador != 0 && vendedor != 0 && ficha != 0 {
                let rc = cpv2_w(comprador + 0x80);
                let rv = cpv2_w(vendedor + 0x80);
                if rc > rv {
                    let dif = rc.wrapping_sub(rv) as f64;
                    let k = if rc > 0x1e46 && cpv2_w(ficha + 0xb) < 0x128e {
                        core::ptr::read_unaligned(0x96a730 as *const f64)
                    } else if rc > 0x109a {
                        core::ptr::read_unaligned(0x967888 as *const f64)
                    } else {
                        core::ptr::read_unaligned(0x967878 as *const f64)
                    };
                    crate::comparison::branch("cpv_2 injury_word_0c do jogador");
                    let soma = match_pl_ftol(dif * k) as i16; // cpv2: a reputação sobe
                    let r = (ficha + 0xb) as *mut i16;
                    core::ptr::write_unaligned(r, core::ptr::read_unaligned(r).wrapping_add(soma));
                    let f2 = cpv2_d(pessoa + 0x61);
                    if cpv2_w(f2 + 0xb) > 0x2710 {
                        core::ptr::write_unaligned((f2 + 0xb) as *mut i16, 0x2710);
                    }
                }
            }
            noticia(this, pessoa, comprador, vendedor, 4, o, 0, 0, preco as u32);
        }
        return seguir(this, o);
    }
    let troca = cpv2_d(o + 4);
    if troca == 0 {
        show_error_at(0xa8d514, 0x101f);
        return seguir(this, o);
    }
    cpv2_inicio(troca);
    let mut x = cpv2_proximo(troca);
    while x != 0 && cpv2_d(x) != cpv2_d(pessoa) {
        x = cpv2_proximo(troca);
    }
    if x == 0 {
        return seguir(this, o);
    }
    crate::comparison::branch("cpv_2 jogador saiu da troca");
    let comprador = cpv2_clube(cpv2_d(o + 8));
    let novo_preco = valor(cpv2_d(0xae1050), pessoa, comprador, 0, -1, 0).wrapping_add(cpv2_d(o + 0x1b) as i32);
    let vendedor = manager_manager_contract_club(cpv2_contrato(cpv2_pessoa(cpv2_d(o + 0xc))), false);
    let comprador = cpv2_clube(cpv2_d(o + 8));
    tirar(cpv2_d(o + 4), x);
    let l = cpv2_d(o + 4);
    if core::ptr::read_unaligned(l as *const u16) == 0 {
        if l != 0 {
            destruir(l);
            liberar(l);
        }
        core::ptr::write_unaligned((o + 4) as *mut usize, 0);
    }
    let m = cpv2_d(this + 0x86e).wrapping_add(cpv2_d(pessoa) * 8 + 2);
    cpv2_pb(m, cpv2_b(m).wrapping_sub(1));
    if silencioso == 0 {
        noticia(this, pessoa, vendedor, comprador, 4, o, 0, 0, novo_preco as u32);
    }
    core::ptr::write_unaligned((o + 0x1b) as *mut i32, novo_preco);
    if vendedor != 0 && human_club(vendedor, 0, 1, 0, 0) == 0 {
        resposta_do_vendedor(this, vaga(), 1);
        if is_offer_resolved(o) {
            return;
        }
    }
    if human_club(comprador, 0, 1, 0, 0) != 0 {
        return seguir(this, o);
    }
    resposta_do_comprador(cpv2_d(0xde1f54), vaga(), 1);
    if is_offer_resolved(o) {
        return;
    }
    seguir(this, o);
}

/// Antes da data das notícias ([0xdd6ec8]: dia +0 com sinal, ano +2) em relação a hoje ([0xae2c90])?
unsafe fn news_date_not_reached() -> bool {
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16))
}

/// { 0x6a4c40 } (thiscall, ret 4, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_board_confidence_update_news(clube)`: antes da data das notícias, nada. A notícia 0x1772 (local): 0 = o id do
/// clube, 1 = os dias do técnico (+0xcf) no cargo (desde a data +0x3e, 365 dias por ano) ou 100 sem técnico; pelo
/// alcance (0x786090) com o clube.
unsafe extern "thiscall" fn post_board_confidence_update_news(_this: usize, clube: usize) {
    if news_date_not_reached() {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1772, 0);
    gravar(n, 0, cpv2_d(clube) as u32);
    let tecnico = cpv2_d(clube + 0xcf);
    let dias = if tecnico != 0 {
        let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(core::ptr::read_unaligned((tecnico + 0x40) as *const u16) as i32);
        anos.wrapping_mul(365).wrapping_sub(cpv2_w(tecnico + 0x3e) as i32).wrapping_add(*(0xae2c90 as *const i16) as i32)
    } else {
        100
    };
    gravar(n, 1, dias as u32);
    por_alcance(cpv2_d(0xb7431c), n, clube, 0, 0, 0, 0);
}

/// { 0x6a4a20 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_board_expectations_news(clube, expectativas)`: antes da data das notícias, nada. A notícia 0x1771 (local): 0 =
/// o id do clube, 1 = as expectativas (palavra com sinal); ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn post_board_expectations_news(_this: usize, clube: usize, expectativas: u32) {
    if news_date_not_reached() {
        return;
    }
    crate::comparison::branch("cpv_2 noticia das expectativas");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1771, 0);
    gravar(n, 0, cpv2_d(clube) as u32);
    gravar(n, 1, expectativas as i16 as i32 as u32);
    entregar(cpv2_d(0xb7431c), n, clube);
}

// ponytail: reagenda, mexe nas listas e na reputação, pode liberar a lista da troca e chama a resposta dos clubes e a
// oferta seguinte (0x8c2f50): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.
unsafe extern "thiscall" fn compare_handle_player_rejected_terms(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(%%I0%%)(this, pessoa, indice, silencioso)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original. Quente (40 mil chamadas): sem contador.
unsafe extern "thiscall" fn compare_post_board_confidence_update_news(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, clube)
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_board_expectations_news(this: usize, clube: usize, expectativas: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, clube, expectativas)
}
