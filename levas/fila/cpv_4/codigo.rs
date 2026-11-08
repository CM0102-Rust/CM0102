// ---------- %%LEVA%%a leva: a negociação guardada da oferta, o boato do técnico em outro clube e a recusa do jogador em se mudar (transfer_manager.cpp, manager_manager.cpp) ----------

unsafe fn cpv4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpv4_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv4_clube(id: usize) -> usize {
    cpv4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv4_pessoa(id: usize) -> usize {
    cpv4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// { 0x8be9d0 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH só dos `new`): `TRANSFER_MANAGER::
/// add_transfer_negotiation(oferta, termos, marcar)`: com `marcar`, +0x2d = 1 nas duas e o jogador marcado sob oferta
/// (0x84b580, com a lista +0). Sem vaga (+0x28 = −1): o vetor das negociações (+0xc, capacidade +0x10, conta +0x14)
/// cresce de 50 quando cheio (0x945667 zerando as novas; 0x945501 sem vetor), a primeira vaga vazia vai para +0x28 e a
/// oferta para o status 1 (sem vaga vazia: a caixa "Error" 0x60f e nada). A negociação antiga da vaga é destruída
/// (0x8e5710, 0x944c94), ou a conta sobe; uma nova (0x41 bytes, 0x944cf1 e 0x8e5680) recebe os termos campo a campo
/// (a data +0x12 por 0x417c10; de +0x30 e +0x31 só os 4 bits baixos), +0x20 da oferta e o id seguinte (+0x85a); a
/// lista +4 dos termos é copiada numa nova (0xe bytes, 0x68d020, 0x68d2f0). Comprador (termos +8) humano: a notícia da
/// negociação (0x8e4610 no empréstimo +0x1f = 1, senão 0x8d6bf0) e a oferta agendada em 14 dias; senão em 2 (0x8c6220).
unsafe extern "thiscall" fn add_transfer_negotiation(this: usize, oferta: usize, termos: usize, marcar: u32) {
    let mark_player_under_offer: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x84b580usize);
    let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
    let alocar_zerado: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945501usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let nova_lista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let copiar_lista: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d2f0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_do_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e4610usize);
    let noticia_da_negociacao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8d6bf0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    if marcar != 0 {
        cpv4_pb(oferta + 0x2d, 1);
        cpv4_pb(termos + 0x2d, 1);
        mark_player_under_offer(cpv4_d(this + 0x856), cpv4_pessoa(cpv4_d(oferta + 0xc)), cpv4_clube(cpv4_d(oferta + 8)), cpv4_d(oferta));
    }
    let mut k = cpv4_d(oferta + 0x28);
    if k == usize::MAX {
        if cpv4_d(this + 0x14) == cpv4_d(this + 0x10) {
            let v = cpv4_d(this + 0xc);
            let cap = cpv4_d(this + 0x10).wrapping_add(0x32);
            cpv4_pd(this + 0x10, cap);
            if v != 0 {
                let nv = realocar(v, cap * 4);
                cpv4_pd(this + 0xc, nv);
                core::ptr::write_bytes((nv + cpv4_d(this + 0x10) * 4 - 0xc8) as *mut u32, 0, 0x32);
            } else {
                cpv4_pd(this + 0xc, alocar_zerado(cap, 4));
            }
        }
        let cap = cpv4_d(this + 0x10) as i32;
        let v = cpv4_d(this + 0xc);
        match (0..cap.max(0) as usize).find(|&j| cpv4_d(v + 4 * j) == 0) {
            Some(j) => k = j,
            None => return show_error_at(0xa8d514, 0x60f),
        }
        crate::comparison::branch("cpv_4 vaga nova");
        cpv4_pd(oferta + 0x28, k);
        cpv4_pb(oferta + 0x2c, 1); // cpv4: a oferta em negociação
    }
    let antiga = cpv4_d(cpv4_d(this + 0xc) + k * 4);
    if antiga != 0 {
        destruir(antiga);
        liberar(antiga);
    } else {
        cpv4_pd(this + 0x14, cpv4_d(this + 0x14).wrapping_add(1));
    }
    let mut nova = novo(0x41);
    nova = if nova != 0 { construir(nova) } else { 0 };
    cpv4_pd(cpv4_d(this + 0xc) + k * 4, nova);
    let neg = cpv4_d(cpv4_d(this + 0xc) + k * 4);
    cpv4_pd(neg, cpv4_d(termos));
    cpv4_pd(neg + 4, cpv4_d(termos + 4));
    cpv4_pd(neg + 8, cpv4_d(termos + 8));
    cpv4_pd(neg + 0xc, cpv4_d(termos + 0xc));
    cpv4_pb(neg + 0x10, cpv4_b(termos + 0x10));
    let b11 = cpv4_b(termos + 0x11);
    let mut temporaria = [0u32; 2];
    cpv4_pb(neg + 0x11, b11);
    atribuir_data(neg + 0x12, temporaria.as_mut_ptr() as usize, termos + 0x12);
    cpv4_pb(neg + 0x1a, cpv4_b(termos + 0x1a));
    cpv4_pd(neg + 0x1b, cpv4_d(termos + 0x1b)); // cpv4: o preço dos termos
    cpv4_pb(neg + 0x1f, cpv4_b(termos + 0x1f));
    cpv4_pd(neg + 0x20, cpv4_d(termos + 0x20));
    cpv4_pd(neg + 0x24, cpv4_d(termos + 0x24));
    cpv4_pd(neg + 0x28, cpv4_d(termos + 0x28));
    for o in 0x2c..0x30 {
        cpv4_pb(neg + o, cpv4_b(termos + o));
    }
    cpv4_pb(neg + 0x30, (cpv4_b(neg + 0x30) & 0xf0) | (cpv4_b(termos + 0x30) & 0xf));
    cpv4_pd(neg + 0x31, (cpv4_d(neg + 0x31) & !0xf) | (cpv4_d(termos + 0x31) & 0xf));
    cpv4_pd(neg + 0x35, cpv4_d(termos + 0x35));
    cpv4_pb(neg + 0x39, cpv4_b(termos + 0x39));
    cpv4_pd(neg + 0x3a, cpv4_d(termos + 0x3a));
    cpv4_pb(neg + 0x3e, cpv4_b(termos + 0x3e));
    cpv4_pb(neg + 0x3f, cpv4_b(termos + 0x3f));
    cpv4_pb(neg + 0x40, cpv4_b(termos + 0x40));
    let neg = || cpv4_d(cpv4_d(this + 0xc) + k * 4);
    cpv4_pd(neg() + 0x20, cpv4_d(oferta + 0x20));
    cpv4_pd(neg() + 0x24, cpv4_d(this + 0x85a));
    cpv4_pd(this + 0x85a, cpv4_d(this + 0x85a).wrapping_add(1));
    if cpv4_d(termos + 4) != 0 {
        let mut l = novo(0xe);
        l = if l != 0 { nova_lista(l) } else { 0 };
        cpv4_pd(neg() + 4, l);
        copiar_lista(cpv4_d(neg() + 4), cpv4_d(termos + 4));
    }
    if humano(cpv4_d(0xb7431c), cpv4_clube(cpv4_d(termos + 8))) != 0 {
        if cpv4_b(oferta + 0x1f) == 1 {
            noticia_do_emprestimo(this, neg(), oferta);
        } else {
            noticia_da_negociacao(this, neg(), oferta);
        }
        agendar(this, oferta, 0xe, 0);
    } else {
        crate::comparison::branch("cpv_4 negociacao agendada");
        agendar(this, oferta, 2, 0); // cpv4: a oferta em 2 dias
    }
}

/// { 0x69fd50 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_job_rumour_news(pessoa, clube)`: sem pessoa ou sem clube dela (+0x39), nada; um sorteio(4) = 0, nada; antes da
/// data das notícias ([0xdd6ec8]), nada. A notícia 0x1776 (local): 0..3 a pessoa, 4 = o cargo (+0x3d), 5 = o id do
/// clube dela, 6 = o id do clube do boato, 7 = sorteio(4); aos destinatários (0x8d1da0 do gerente de transferências
/// [0xde1f54], com 0x52, até 30; sem gerente, nenhum) pela entrega ao técnico (0x7861e0).
unsafe extern "thiscall" fn post_job_rumour_news(_this: usize, pessoa: usize, clube: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    if pessoa == 0 || cpv4_d(pessoa + 0x39) == 0 || sorteio(4) == 0 {
        return;
    }
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1da0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cpv_4 boato do tecnico");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1776, 0);
    gravar(n, 0, cpv4_d(pessoa) as u32);
    gravar(n, 1, cpv4_d(cpv4_d(pessoa + 4) + 0x33) as u32);
    gravar(n, 2, cpv4_d(cpv4_d(pessoa + 8) + 0x33) as u32);
    gravar(n, 3, cpv4_d(cpv4_d(pessoa + 0xc) + 0x33) as u32);
    gravar(n, 4, cpv4_b(pessoa + 0x3d) as i8 as i32 as u32);
    gravar(n, 5, cpv4_d(cpv4_d(pessoa + 0x39)) as u32);
    gravar(n, 6, cpv4_d(clube) as u32);
    let s = sorteio(4) as i16 as i32; // cpv4: a variante do boato
    gravar(n, 7, s as u32);
    let mut clubes = [0usize; 30];
    let gerente = cpv4_d(0xde1f54);
    let q = if gerente != 0 { destinatarios(gerente, pessoa, 0x52, clubes.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for &c in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv4_d(0xb7431c), n, c);
    }
}

/// { 0x8d8310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_player_rejects_move_news(oferta, motivo)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os
/// destinatários (0x8d1130 com 0x2f, até 30), a notícia 0xfae (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 =
/// +8, 6 = +0x1f, 7 = o motivo, 8 = o técnico do dono, 9 = o técnico do comprador (+8 > −1); a cada destinatário.
unsafe extern "thiscall" fn post_player_rejects_move_news(this: usize, oferta: usize, motivo: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cpv_4 recusa em se mudar");
    let p = cpv4_pessoa(cpv4_d(oferta + 0xc));
    let c = contract_of(cpv4_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 && cpv4_b(c + 0x35) & 0x3f != 5 { cpv4_clube(cpv4_d(c + 4)) } else { 0 };
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2f, clubes.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfae, 0);
    gravar(n, 0, cpv4_d(p) as u32);
    gravar(n, 1, cpv4_d(cpv4_d(p + 4) + 0x33) as u32);
    gravar(n, 2, cpv4_d(cpv4_d(p + 8) + 0x33) as u32);
    gravar(n, 3, cpv4_d(cpv4_d(p + 0xc) + 0x33) as u32);
    gravar(n, 4, if dono != 0 { cpv4_d(dono) as u32 } else { u32::MAX });
    gravar(n, 5, cpv4_d(oferta + 8) as u32);
    gravar(n, 6, cpv4_b(oferta + 0x1f) as i8 as i32 as u32);
    gravar(n, 7, motivo);
    if dono != 0 {
        let t = cpv4_d(dono + 0xcf);
        if t != 0 {
            gravar(n, 8, cpv4_d(t) as u32);
        }
    }
    let comprador = cpv4_d(oferta + 8) as i32;
    if comprador > -1 {
        let t = cpv4_d(cpv4_clube(comprador as usize) + 0xcf);
        if t != 0 {
            gravar(n, 9, cpv4_d(t) as u32);
        }
    }
    for &d in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv4_d(0xb7431c), n, d);
    }
}

// ponytail: aloca e libera negociações e listas, agenda a oferta e manda notícias: não roda duas vezes. Só a original;
// a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_add_transfer_negotiation(this: usize, oferta: usize, termos: usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%)(this, oferta, termos, marcar)
}

// ponytail: sorteia e a notícia vai às caixas (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_job_rumour_news(this: usize, pessoa: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, pessoa, clube)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_player_rejects_move_news(this: usize, oferta: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, oferta, motivo)
}
