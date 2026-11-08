// ---------- %%LEVA%%a leva: o empréstimo do arquivo de configuração, a notícia do fim do contrato e a do pedido de adiamento da transferência (transfer_manager.cpp, contract_manager.cpp) ----------

unsafe fn cpx4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx4_clube(id: usize) -> usize {
    cpx4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx4_pessoa(id: usize) -> usize {
    cpx4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): copia a data nos dois.
unsafe fn transfer_manager_assign_date(destino: usize, origem: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), origem);
}

/// A data `a` (dia com sinal em +0, ano em +2) vem antes da `b`.
unsafe fn transfer_manager_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (cpx4_w(a + 2) as u16, cpx4_w(b + 2) as u16);
    ya < yb || (ya == yb && cpx4_w(a) < cpx4_w(b))
}

/// As condições do empréstimo da configuração, na ordem da original: o time B ligado ao clube (0x540a50; vale só se não
/// for o principal), o elenco do clube abaixo de 50 (0x53c590), a pessoa sem empréstimo (0x4cf6d0), o começo até hoje,
/// o fim de hoje em diante e um contrato (0x4cf5a0). Devolve o time B (ou 0) quando passam.
unsafe fn config_loan_allowed(p: usize, clube: usize, inicio: usize, fim: usize) -> Option<usize> {
    let ligado: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mut principal = 0u32;
    let mut b = ligado(clube, &mut principal, 1);
    if b != 0 && principal != 0 {
        b = 0;
    }
    if contar(clube, 0, 0) as i8 >= 0x32 || emprestado(cpx4_d(0xae1050), p) != 0 {
        return None;
    }
    if transfer_manager_date_before(0xae2c90, inicio) || transfer_manager_date_before(fim, 0xae2c90) {
        return None;
    }
    if contrato(cpx4_d(0xae1050), p) == 0 {
        return None;
    }
    Some(b)
}

/// { 0x8bf400 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH do contrato local): `TRANSFER_MANAGER::
/// add_config_loan(pessoa, clube, começo, fim)`, pelos empréstimos do arquivo de configuração no começo do jogo. Pelas
/// condições (config_loan_allowed), o empréstimo vai ao time B ligado ou ao clube: com o fim depois da data +0x46 da pessoa, a
/// data e o fim do contrato (+0x2d) passam ao fim; o contrato local (0x85e410, 0x85e590 com o fim, 0, 1, 1; começo em
/// +0x25) é assinado (0x4d01e0); as suspensões ficam guardadas (0x5563f0); o elenco 1 (0x540980); +0x6d 2 vira 0x10; os
/// números de camisa (0x85add0, 0x85a470); no time B, a pessoa vai às reservas (0x4cc0f0). Precisando de permissão de
/// trabalho no clube (0x8c7560), o fim recebido anda 7 dias e a permissão é dada (0x8c7070 com a nação do clube e o ano
/// do começo).
unsafe extern "thiscall" fn add_config_loan(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let mut local = [0u8; 0x50]; // o STAFF_CONTRACT local (0x50 bytes)
    let l = local.as_mut_ptr() as usize;
    construir(l);
    let b = match config_loan_allowed(p, clube, inicio, fim) {
        Some(b) => b,
        None => return,
    };
    if transfer_manager_date_before(p + 0x46, fim) {
        crate::comparison::branch("cpx_4 contrato estendido");
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        transfer_manager_assign_date(p + 0x46, fim); // cpx4: a data da pessoa até o fim
        transfer_manager_assign_date(contrato(cpx4_d(0xae1050), p) + 0x2d, fim); // cpx4: o fim do contrato até o fim do empréstimo
    }
    let destino = if b != 0 { b } else { clube };
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    preencher(l, p, destino, fim, 0, 1, 1);
    transfer_manager_assign_date(l + 0x25, inicio);
    crate::comparison::branch("cpx_4 emprestimo da configuracao");
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d01e0usize);
    assinar(cpx4_d(0xae1050), l); // cpx4: o contrato de empréstimo
    let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5563f0usize);
    guardar(cpx4_d(0xae28f0), p);
    let elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x540980usize);
    elenco(p, 1);
    if cpx4_b(p + 0x6d) == 2 {
        *((p + 0x6d) as *mut u8) = 0x10;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    if usa(cpx4_d(0xdd7ef0), destino, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85a470usize);
        numerar(cpx4_d(0xdd7ef0), destino, 0);
    }
    if b != 0 && b != clube {
        let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cc0f0usize);
        reservas(cpx4_d(0xae1050), p, b);
    }
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if precisa(this, p, clube, 0) != 0 {
        crate::comparison::branch("cpx_4 permissao de trabalho");
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u8; 8];
        let r = mais_dias(fim, t.as_mut_ptr(), 7);
        transfer_manager_assign_date(fim, r);
        // a original empurra cx sobre o ecx que a 0x417c10 zera: o ano limpo
        let permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
        permissao(this, p, cpx4_d(clube + 0x53), cpx4_w(inicio + 2) as u16 as u32, 1, 1); // cpx4: a permissão de trabalho
    }
}

// ponytail: assina o empréstimo, mexe no elenco e nas permissões (não pode rodar duas vezes): só a original, com o
// plano pelas mesmas condições (passando, a pessoa sai emprestada; senão o empréstimo dela fica como estava).
unsafe extern "thiscall" fn comparar_add_config_loan(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let tinha = emprestado(cpx4_d(0xae1050), p);
    let passa = config_loan_allowed(p, clube, inicio, fim).is_some();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(%%I0%%)(this, p, clube, inicio, fim);
    let tem = emprestado(cpx4_d(0xae1050), p);
    let igual = if passa { tem != 0 } else { tem == tinha };
    PLACAR.record(igual, || format!("add_config_loan(pessoa {:#x}) passa={} antes={:#x} depois={:#x}", cpx4_d(p), passa, tinha, tem));
}

/// O contrato principal da pessoa como a 0x4da590 o acha (o mesmo da 0x4cf5a0, em linha): sem pessoa, id fora da tabela
/// ou índice negativo no registro de 79 bytes, nenhum; guardado em [0xae1080].
unsafe fn contract_inline_lookup(p: usize) -> usize {
    let c = if p == 0 || cpx4_d(p) as i32 >= cpx4_d(0xae2364) as i32 {
        0
    } else {
        let k = cpx4_d(cpx4_d(0xae2d0c).wrapping_add(cpx4_d(p).wrapping_mul(0x4f))) as i32;
        if k < 0 {
            0
        } else {
            cpx4_d(cpx4_d(0xae1050)).wrapping_add((k as usize).wrapping_mul(0x50))
        }
    };
    *(0xae1080 as *mut usize) = c;
    c
}

/// Os clubes interessados na pessoa (Shortlist_Manager 0x842180 com 1, 1, 0) numa LLIST local já construída; devolve os
/// ids, do começo.
unsafe fn interested_club_ids(p: usize, l: usize) -> Vec<usize> {
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    interessados(cpx4_d(cpx4_d(0xde1f54) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx4_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(cpx4_d(e + 4));
        e = proximo(l);
    }
    v
}

/// { 0x4da590 } (thiscall, ret 0xc, contract_manager.cpp, quadro SEH da LLIST e da notícia locais): `CONTRACT_MANAGER::
/// post_contract_end_news(pessoa, tipo, amplo)`, pelo fim de contrato, de teste ou de empréstimo (0x4c7ac0). Sem
/// contrato ou sem o clube dele: nada. A notícia 0xbb9: o clube do contrato, a pessoa (o id e as três chaves), o tipo, o
/// clube do contrato no tipo 6 (senão −1) e `amplo`. Ampla, com ficha: reputação mundial (+0xd) acima de 7000, a todos
/// os técnicos (0x786780); nacional (+0xb) acima de 7000, aos técnicos da nação da pessoa, da do clube e da do clube
/// atual (+0x39) (0x7863f0) e aos clubes interessados (0x842180) de outras nações. Senão ao clube, ao clube atual no
/// tipo 6 e aos interessados (0x7861e0).
unsafe extern "thiscall" fn post_contract_end_news(this: usize, p: usize, tipo: u32, amplo: u32) {
    let c = contract_inline_lookup(p);
    if c == 0 || cpx4_clube(cpx4_d(c + 4)) == 0 {
        return;
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    crate::comparison::branch("cpx_4 noticia do fim do contrato");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbb9, 0);
    let c = contract_inline_lookup(p);
    // a original lê o id no endereço 0 sem contrato: não acontece, acabou de achá-lo
    let clube = if c == 0 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    gravar(nt, 0, cpx4_d(clube) as u32);
    gravar(nt, 2, cpx4_d(p) as u32);
    gravar(nt, 3, cpx4_d(cpx4_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx4_d(cpx4_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx4_d(cpx4_d(p + 0xc) + 0x33) as u32);
    let tipo = tipo as u8;
    gravar(nt, 6, tipo as i8 as u32);
    let c = contract_inline_lookup(p);
    let clube = if c == 0 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    if tipo == 6 {
        // o clube do contrato pela tabela do próprio gerente (+0), sem guardar em [0xae1080]; sem contrato, a original
        // lê o endereço 4 (não acontece: acabou de achá-lo)
        let k = if p == 0 || cpx4_d(p) as i32 >= cpx4_d(0xae2364) as i32 {
            -1
        } else {
            cpx4_d(cpx4_d(0xae2d0c).wrapping_add(cpx4_d(p).wrapping_mul(0x4f))) as i32
        };
        let v = if k < 0 { cpx4_d(4) } else { cpx4_d(cpx4_d(this).wrapping_add((k as usize).wrapping_mul(0x50)) + 4) };
        gravar(nt, 7, v as u32);
    } else {
        gravar(nt, 7, u32::MAX);
    }
    gravar(nt, 8, amplo);
    let ficha = cpx4_d(p + 0x61);
    let atual = cpx4_d(p + 0x39);
    let nacao = cpx4_d(p + 0x1a);
    let jornal = cpx4_d(0xb7431c);
    let mut padrao = true;
    if amplo != 0 && ficha != 0 {
        if cpx4_w(ficha + 0xd) > 0x1b58 {
            todos(jornal, nt);
            padrao = false;
        } else if cpx4_w(ficha + 0xb) > 0x1b58 {
            padrao = false;
            do_pais(jornal, nt, nacao);
            if nacao != cpx4_d(clube + 0x53) {
                do_pais(jornal, nt, cpx4_d(clube + 0x53));
            }
            if atual != 0 && atual != clube {
                let n = cpx4_d(atual + 0x53);
                if n != cpx4_d(clube + 0x53) && n != nacao {
                    do_pais(jornal, nt, n);
                }
            }
            for id in interested_club_ids(p, l) {
                if id == cpx4_d(clube) || (tipo == 6 && id == cpx4_d(cpx4_d(p + 0x39))) {
                    continue;
                }
                let outro = cpx4_clube(id);
                let n = cpx4_d(outro + 0x53);
                if n != cpx4_d(p + 0x1a) && n != cpx4_d(cpx4_d(p + 0x39) + 0x53) && n != cpx4_d(clube + 0x53) {
                    entregar(cpx4_d(0xb7431c), nt, outro);
                }
            }
        }
    }
    if padrao {
        entregar(jornal, nt, clube);
        if tipo == 6 {
            entregar(cpx4_d(0xb7431c), nt, cpx4_d(p + 0x39));
        }
        for id in interested_club_ids(p, l) {
            if id == cpx4_d(clube) || (tipo == 6 && id == cpx4_d(cpx4_d(p + 0x39))) {
                continue;
            }
            entregar(cpx4_d(0xb7431c), nt, cpx4_clube(id));
        }
    }
    soltar(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_contract_end_news(this: usize, p: usize, tipo: u32, amplo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%)(this, p, tipo, amplo)
}

/// { 0x8dda90 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_delay_request_news(oferta, motivo)`, pelo clube que pede para adiar a transferência uma semana. Os
/// clubes humanos que recebem (0x8d1130 com 1, ou 2 fora do estado 0x11; até 30); sem nenhum, nada. A notícia 0xfab: o
/// clube do contrato (sem tipo 5) ou −1, o comprador, a pessoa (o id e as três chaves), o estado, 0, 0, a vaga +0x20 e o
/// motivo; a cada destinatário.
unsafe extern "thiscall" fn post_transfer_delay_request_news(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let p = cpx4_pessoa(cpx4_d(o + 0xc));
    let c = contrato(cpx4_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let do_contrato = if c == 0 || cpx4_b(c + 0x35) & 0x3f == 5 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    let comprador = cpx4_clube(cpx4_d(o + 8));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (cpx4_b(o + 0x2c) != 0x11) as u32 + 1, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparison::branch("cpx_4 noticia do adiamento");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfab, 0);
    gravar(nt, 0, if do_contrato != 0 { cpx4_d(do_contrato) as u32 } else { u32::MAX });
    gravar(nt, 1, cpx4_d(comprador) as u32);
    gravar(nt, 2, cpx4_d(p) as u32);
    gravar(nt, 3, cpx4_d(cpx4_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx4_d(cpx4_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx4_d(cpx4_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 6, cpx4_b(o + 0x2c) as i8 as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, 0);
    gravar(nt, 9, cpx4_d(o + 0x20) as u32);
    gravar(nt, 10, motivo as i16 as u32);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(cpx4_d(0xb7431c), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_transfer_delay_request_news(this: usize, o: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, o, motivo)
}
