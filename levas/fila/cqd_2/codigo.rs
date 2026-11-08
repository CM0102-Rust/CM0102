// ---------- %%LEVA%%a leva: o lance aceito pelo clube vendedor, a notícia da oferta feita e o empréstimo possível (transfer_manager.cpp) ----------

unsafe fn cqd2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqd2_pessoa(id: usize) -> usize {
    cqd2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqd2_clube(id: usize) -> usize {
    cqd2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080], e o
/// clube dele; sem contrato, ou do tipo 5 (+0x35) quando `sem_5`: 0.
unsafe fn contract_club_optional_type5(pessoa: usize, sem_5: bool) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqd2_d(0xae1050), pessoa);
    cqd2_pd(0xae1080, c);
    if c == 0 || (sem_5 && cqd2_b(c + 0x35) & 0x3f == 5) {
        0
    } else {
        cqd2_clube(cqd2_d(c + 4))
    }
}

unsafe fn cqd2_humano(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0) != 0
}

unsafe fn cqd2_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// A oferta segue ativa (status 2, 3, 4, 0x10, 0x14, ou 0xf com +0x2e ≥ 3), o teste que a original repete.
unsafe fn is_offer_status_settled(o: usize) -> bool {
    let s = cqd2_b(o + 0x2c);
    matches!(s, 2 | 3 | 4 | 0x10 | 0x14) || (s == 0xf && cqd2_b(o + 0x2e) >= 3)
}

/// A lista de troca (+4) vazia (palavra +0 zerada) é destruída (0x68d040: mov ecx, lista -> thiscall, ret 0), liberada
/// (0x944c94: push; add esp, 4 -> cdecl) e +4 zera.
unsafe fn free_empty_exchange_list(o: usize) {
    let l = cqd2_d(o + 4);
    if cqd2_w(l) != 0 {
        return;
    }
    if l != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(l);
        liberar(l);
    }
    cqd2_pd(o + 4, 0);
}

/// { 0x8c0080 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: o STAFF_CONTRACT
/// local da 0x85e410): `TRANSFER_MANAGER::accept_transfer_bid(vaga, marcar)`, o clube vendedor aceita o lance. O alvo é
/// o +0xc, o comprador o +8 e o vendedor o clube do contrato do alvo (fora o tipo 5). +0x2e = 0; com `marcar`, +0x2d = 1
/// e o jogador sob oferta na lista de observação (0x84b580: push +0 da oferta; push comprador; push alvo; mov ecx,
/// [this+0x856] -> thiscall, ret 0xc). +0x2e = 0, status 5; a conta dos técnicos humanos (0x7875e0, stdcall: o ecx =
/// [0xb7431c] não é lido) entre comprador e vendedor; a notícia da aceitação (0x8d76d0).
/// - Empréstimo (+0x11 = 0xb): o estatuto que o comprador daria (1 se humano; senão 0x849f00: push 0; push comprador;
///   push alvo, ret 0xc) vai no byte baixo do dword de `marcar`, e o interesse do alvo (0x846920: push &motivo; push
///   esse dword; push 0; push 0; push comprador; push alvo; mov ecx, [this+0x856] -> thiscall, ret 0x18, devolve ST0)
///   abaixo de 0,75 (ou NaN): a recusa em se mudar (0x8d8310: push motivo; push oferta, ret 8), status 0xf, +0x2e = 3, os
///   lances desmarcados (0x84aeb0), 10 dias (0x8c6220), com a oferta ativa e fechada com as da lista +0 (0x8d0d90,
///   stdcall) o bit 0 do registro de 8 bytes do alvo em [this+0x86e] apagado, e a situação na lista de observação
///   (0x849ce0: push 0; push tipo ≠ 0xb; push alvo; push comprador, ret 0x10; achada, 0x843e10: push o dword com o byte
///   baixo = cargo ≠ 0xb e os altos do ponteiro da entrada; push 1; push comprador; push alvo, ret 0x10). Fim.
/// - Senão, cada jogador da troca (+4): vendedor humano, +5 = 2; o contrato desejado dele (0x85fc50: push 0; push 0;
///   push 0xb; push +5 (só o byte é lido); push −1; push vendedor; push pessoa; lea ecx, contrato ->
///   thiscall, ret 0x1c) e o teto salarial do vendedor (0x5a0590: push vendedor; mov ecx, [0xae2a38], ret 4; 0x5968d0:
///   push +5; push pessoa; mov ecx, finanças, ret 8): teto × 1,5 abaixo do salário, ele sai da troca: o valor dele
///   (0x4d0c60: push 0; push −1; push 0; push comprador; push pessoa; mov ecx, [0xae1050], ret 0x14) soma ao +0x1b, a
///   notícia (0x8d8910: push valor; push pessoa; push oferta, ret 0xc), a saída da lista (0x68d1f0: push nó; mov ecx,
///   lista, ret 4), o byte +2 do registro dele em [this+0x86e] cai 1; com vendedor da IA, a resposta da IA (0x8c8090:
///   push 1; push vaga, ret 8) e, ativa a oferta, a troca vazia é solta e fim. Ao fim da lista, a troca vazia é solta.
/// - Técnico humano: a notícia da conversa de contrato (0x8d7cd0); a oferta reagendada para a fatia seguinte (0x8c6220:
///   push 1; push 0); com troca e vendedor da IA, a entrada de cada jogador da troca na lista do vendedor (0x849ce0 com
///   0, 0) recebe o índice +0x20 da oferta em +0x17.
unsafe extern "thiscall" fn accept_transfer_bid(this: usize, vaga: usize, marcar: u32) {
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x84b580usize);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d76d0usize);
    let estatuto: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(0x849f00usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64 = core::mem::transmute(0x846920usize);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d8310usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x8d0d90usize);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let noticia_troca: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x8d8910usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let resposta_ia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c8090usize);
    let conversa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d7cd0usize);
    let mut contrato = [0u8; 0x60];
    let pc = contrato.as_mut_ptr() as usize;
    novo_contrato(pc);
    let o0 = cqd2_d(vaga);
    let alvo = cqd2_pessoa(cqd2_d(o0 + 0xc));
    let comprador = cqd2_clube(cqd2_d(o0 + 8));
    let vendedor = contract_club_optional_type5(alvo, true);
    let o = cqd2_d(vaga);
    cqd2_pb(o + 0x2e, 0);
    if marcar != 0 {
        cqd2_pb(o + 0x2d, 1);
        sob_oferta(cqd2_d(this + 0x856), alvo, comprador, cqd2_d(cqd2_d(vaga)));
    }
    cqd2_pb(o + 0x2e, 0);
    cqd2_pb(o + 0x2c, 5);
    let mut humanos: u8 = 0;
    if human_manager_record_checked(cqd2_clube(cqd2_d(o + 8))) != 0 {
        humanos = 1;
    }
    if vendedor != 0 && human_manager_record_checked(vendedor) != 0 {
        humanos = humanos.wrapping_add(1);
    }
    aceita(this, o);
    if cqd2_b(o + 0x11) == 0xb {
        let e = if cqd2_humano(comprador) { 1 } else { estatuto(cqd2_d(this + 0x856), alvo, comprador, 0) };
        let dw = (marcar & 0xffff_ff00) | e as u32;
        let mut motivo: u32 = 0;
        let r = interesse(cqd2_d(this + 0x856), alvo, comprador, 0, 0, dw, &mut motivo);
        crate::comparison::branch("cqd_2 loan");
        if !(r >= 0.75) { // cqd2: o alvo não quer se mudar
            recusa_mudar(this, o, motivo);
            cqd2_pb(o + 0x2c, 0xf);
            cqd2_pb(o + 0x2e, 3);
            let a2 = cqd2_pessoa(cqd2_d(o + 0xc));
            let atual = contract_club_optional_type5(a2, false);
            desmarcar(cqd2_d(this + 0x856), cqd2_clube(cqd2_d(o + 8)), a2);
            let l = cqd2_d(o + 4);
            if l != 0 {
                cqd2_pd(l + 0xa, cqd2_d(l + 2));
                loop {
                    let p = cqd2_proximo(cqd2_d(o + 4));
                    if p == 0 {
                        break;
                    }
                    desmarcar(cqd2_d(this + 0x856), atual, cqd2_pessoa(cqd2_d(p)));
                }
            }
            agendar(this, o, 0xa, 0);
            'fechar: {
                if !is_offer_status_settled(o) {
                    break 'fechar;
                }
                let l = cqd2_d(o);
                if l != 0 {
                    cqd2_pd(l + 0xa, cqd2_d(l + 2));
                    loop {
                        let p = cqd2_proximo(cqd2_d(o));
                        if p == 0 {
                            break;
                        }
                        if fechada(cqd2_d(cqd2_d(this).wrapping_add(cqd2_d(p).wrapping_mul(4)))) == 0 {
                            break 'fechar;
                        }
                    }
                }
                let m = cqd2_d(this + 0x86e).wrapping_add(cqd2_d(o + 0xc).wrapping_mul(8));
                if cqd2_b(m) & 1 != 0 {
                    cqd2_pb(m, cqd2_b(m) & 0xfe);
                }
            }
            let ne = (cqd2_b(o + 0x11) != 0xb) as u32;
            let x = entrada(cqd2_d(this + 0x856), cqd2_clube(cqd2_d(o + 8)), cqd2_pessoa(cqd2_d(o + 0xc)), ne, 0);
            if x != 0 {
                // ponytail: a original empurra o setne com os bytes altos do ponteiro da entrada; a 0x843e10 só lê o byte
                let t = (x as u32 & 0xffff_ff00) | (cqd2_b(x) != 0xb) as u32;
                situacao(cqd2_d(this + 0x856), cqd2_pessoa(cqd2_d(o + 0xc)), cqd2_clube(cqd2_d(o + 8)), 1, t);
            }
            return;
        }
    }
    if cqd2_d(o + 4) != 0 {
        let l = cqd2_d(o + 4);
        cqd2_pd(l + 0xa, cqd2_d(l + 2));
        let mut p = cqd2_proximo(cqd2_d(o + 4));
        while p != 0 {
            let pessoa = cqd2_pessoa(cqd2_d(p));
            if cqd2_humano(vendedor) {
                cqd2_pb(p + 5, 2);
            }
            let cargo = cqd2_b(p + 5);
            // ponytail: a original empurra o edx com o byte +5 e os bytes altos do que a 0x600a50 deixou nele; a 0x85fc50
            // só lê o byte (mov cl, [esp+0x284])
            let salario = desejado(pc, pessoa, vendedor, u32::MAX, cargo as u32, 0xb, 0, 0);
            let t = teto(financas(cqd2_d(0xae2a38), vendedor), pessoa, cargo as u32);
            crate::comparison::branch("cqd_2 expensive swap");
            if (t as f64 * 1.5) < salario as f64 { // cqd2: o jogador da troca caro demais
                let v = valor(cqd2_d(0xae1050), pessoa, comprador, 0, u32::MAX, 0).wrapping_add(cqd2_d(o + 0x1b) as i32);
                noticia_troca(this, o, pessoa, v);
                tirar(cqd2_d(o + 4), p);
                let m = cqd2_d(this + 0x86e).wrapping_add(cqd2_d(pessoa).wrapping_mul(8)) + 2;
                cqd2_pb(m, cqd2_b(m).wrapping_sub(1));
                cqd2_pd(o + 0x1b, v as u32 as usize);
                if vendedor != 0 && !cqd2_humano(vendedor) {
                    resposta_ia(this, vaga, 1);
                    if is_offer_status_settled(o) {
                        free_empty_exchange_list(o);
                        return;
                    }
                }
            }
            p = cqd2_proximo(cqd2_d(o + 4));
        }
        free_empty_exchange_list(o);
    }
    if humanos != 0 {
        conversa(this, o);
    }
    crate::comparison::branch("cqd_2 bid accepted");
    agendar(this, o, 0, 1); // cqd2: a próxima fatia
    if cqd2_d(o + 4) == 0 || cqd2_humano(vendedor) {
        return;
    }
    let l = cqd2_d(o + 4);
    cqd2_pd(l + 0xa, cqd2_d(l + 2));
    loop {
        let p = cqd2_proximo(cqd2_d(o + 4));
        if p == 0 {
            break;
        }
        let x = entrada(cqd2_d(this + 0x856), vendedor, cqd2_pessoa(cqd2_d(p)), 0, 0);
        if x != 0 {
            cqd2_pd(x + 0x17, cqd2_d(o + 0x20));
        }
    }
}

/// Comparador da 0x8c0080: só a original (muda o status, reagenda, solta a troca, aloca notícias e chama a resposta da
/// IA).
unsafe extern "thiscall" fn compare_accept_transfer_bid(this: usize, vaga: usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, vaga, marcar) // ponytail: só a original
}

/// { 0x8d3090 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a notícia e a data
/// locais): `TRANSFER_MANAGER::post_offer_made_news(oferta)`. O vendedor é o clube do contrato do alvo (fora o tipo 5).
/// Os destinatários (0x8d1130: push 0x1e; push &vetor; push 0x3e; push oferta; mov ecx, this -> thiscall, ret 0x10);
/// nenhum: nada. A notícia 0xfa0 (0x77bbe0: push 0; push 0xfa0; lea ecx, notícia, ret 8; os dados pela 0x785790: push
/// v; push k; lea ecx, notícia, ret 8): 0 o id do alvo, 1..3 o +0x33 de +4/+8/+0xc dele, 4 o comprador (+8), 5 o id do
/// vendedor (−1 sem), 6 +0x20, 7 +0x24, 8 +0x1b, 9 o status, 0xa o valor (0x8e7a70: push 1; push 0; push 1; push 0; mov
/// ecx, oferta -> thiscall(oferta, 0, 1, 0, 1), ret 0x10), 0xb +0x2f, 0xe +0x31 & 0xf. Com +0x2f, vendedor e cargo 11..15
/// do alvo (+0x3d; 0x53b620, cdecl): pelo contrato do alvo (0x4cf5a0, sem guardar em [0xae1080]) a data dele (+0x2d/
/// +0x2f) se ainda não chegou (senão 0, 0) em 0xc/0xd e o tipo (+0x35, 6 bits com sinal) em 0xf; sem contrato, 0, 0, 0.
/// Cada jogador da troca (+4): os 3 primeiros dão o id e os 3 +0x33 a partir de 0x18; 0x17 a conta. A pessoa da vez (o
/// último da troca, ou o alvo) com seleção (+0x24) segunda seleção (0x5409b0, cdecl): 0x11 o id dela e a marca 1; senão
/// −1. O recorde (0x4486c0: push 1; push oferta; mov ecx, [0xad9dac] -> thiscall, ret 8) soma a marca 2: 0x10 a marca; o
/// próximo índice livre recebe o +0x11. A notícia a cada destinatário (0x7861e0: push clube; push &notícia; mov ecx,
/// [0xb7431c] -> thiscall, ret 8). Com +0x2f: +0x30 & 0xf = 5, a janela fixa (0x8d0bb0, stdcall(&data, 0): o ecx não é
/// lido) dá 0x12 = 0 se hoje é a data, senão 5, e a data em 0xc/0xd; senão 0x12 = 3.
unsafe extern "thiscall" fn post_offer_made_news(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let recorde: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x4486c0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let janela: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(0x8d0bb0usize);
    let mut alvo = cqd2_pessoa(cqd2_d(o + 0xc));
    let vendedor = contract_club_optional_type5(alvo, true);
    let mut clubes = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x3e, clubes.as_mut_ptr() as usize, 0x1e);
    if n == 0 {
        return;
    }
    let mut noticia = [0u32; 0x38];
    let pn = noticia.as_mut_ptr() as usize;
    construir(pn, 0xfa0, 0);
    let g = |k: i8, v: u32| { gravar(pn, k, v); };
    g(0, cqd2_d(alvo) as u32);
    g(1, cqd2_d(cqd2_d(alvo + 4) + 0x33) as u32);
    g(2, cqd2_d(cqd2_d(alvo + 8) + 0x33) as u32);
    g(3, cqd2_d(cqd2_d(alvo + 0xc) + 0x33) as u32);
    g(4, cqd2_d(o + 8) as u32);
    g(5, if vendedor != 0 { cqd2_d(vendedor) as u32 } else { u32::MAX });
    g(6, cqd2_d(o + 0x20) as u32);
    g(7, cqd2_d(o + 0x24) as u32);
    g(8, cqd2_d(o + 0x1b) as u32);
    g(9, cqd2_b(o + 0x2c) as i8 as i32 as u32);
    g(0xa, total(o, 0, 1, 0, 1) as u32);
    g(0xb, cqd2_b(o + 0x2f) as i8 as i32 as u32);
    g(0xe, (cqd2_d(o + 0x31) & 0xf) as u32);
    if cqd2_b(o + 0x2f) != 0 && vendedor != 0 && tipo(cqd2_b(alvo + 0x3d) as i8) != 0 {
        let c = contract_of(cqd2_d(0xae1050), alvo);
        if c != 0 {
            let (ha, ca) = (cqd2_w(0xae2c92) as u16, cqd2_w(c + 0x2f) as u16);
            if ha < ca || (ha == ca && cqd2_w(0xae2c90) < cqd2_w(c + 0x2d)) {
                g(0xc, cqd2_w(c + 0x2d) as i32 as u32);
                g(0xd, ca as u32);
            } else {
                g(0xc, 0);
                g(0xd, 0);
            }
            g(0xf, (((cqd2_b(c + 0x35) << 2) as i8) >> 2) as i32 as u32);
        } else {
            g(0xc, 0);
            g(0xd, 0);
            g(0xf, 0);
        }
    }
    let mut k: u8 = 0x18;
    let mut conta: u8 = 0;
    let l = cqd2_d(o + 4);
    if l != 0 {
        cqd2_pd(l + 0xa, cqd2_d(l + 2));
        loop {
            let p = cqd2_proximo(cqd2_d(o + 4));
            if p == 0 {
                break;
            }
            alvo = cqd2_pessoa(cqd2_d(p));
            if (conta as i8) < 3 {
                g(k as i8, cqd2_d(alvo) as u32);
                g(k.wrapping_add(1) as i8, cqd2_d(cqd2_d(alvo + 4) + 0x33) as u32);
                g(k.wrapping_add(2) as i8, cqd2_d(cqd2_d(alvo + 8) + 0x33) as u32);
                g(k.wrapping_add(3) as i8, cqd2_d(cqd2_d(alvo + 0xc) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
            conta = conta.wrapping_add(1);
        }
    }
    g(0x17, conta as i8 as i32 as u32);
    let mut marca: u8 = 0;
    let s = cqd2_d(alvo + 0x24);
    if s != 0 && segunda(s) != 0 {
        marca = 1;
        g(0x11, cqd2_d(cqd2_d(alvo + 0x24)) as u32);
    } else {
        g(0x11, u32::MAX);
    }
    if recorde(cqd2_d(0xad9dac), o, 1) != 0 {
        marca |= 2;
    }
    g(0x10, marca as i8 as i32 as u32);
    g(k as i8, cqd2_b(o + 0x11) as i8 as i32 as u32);
    for i in 0..(n as i8).max(0) as usize {
        entregar(cqd2_d(0xb7431c), pn, clubes[i]);
    }
    if cqd2_b(o + 0x2f) == 0 {
        return;
    }
    if cqd2_b(o + 0x30) & 0xf != 5 {
        g(0x12, 3);
        return;
    }
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    janela(pd, 0);
    let hoje_igual = cqd2_w(0xae2c92) == cqd2_w(pd + 2) && cqd2_w(0xae2c90) == cqd2_w(pd);
    g(0x12, if hoje_igual { 0 } else { 5 });
    g(0xc, cqd2_w(pd) as i32 as u32);
    g(0xd, cqd2_w(pd + 2) as u16 as u32);
}

/// Comparador da 0x8d3090: só a original (a notícia vai aos técnicos e não tem outro efeito).
unsafe extern "thiscall" fn compare_post_offer_made_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, o) // ponytail: só a notícia
}

/// O virtual +0x20 da regra de nação `r` (push 0; push mensagem; push jogador; push clube; mov ecx, regra: 4 dwords).
unsafe fn rules_vfunc_20(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 {
    core::mem::transmute(cqd2_d(cqd2_d(r) + 0x20))
}

/// O virtual +0x24 da regra de nação `r` (push 1; push &data; push 0; push jogador; push clube; mov ecx, regra: 5 dwords).
unsafe fn rules_vfunc_24(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 {
    core::mem::transmute(cqd2_d(cqd2_d(r) + 0x24))
}

/// { 0x8d0040 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH sem destrutor: a data e os textos locais):
/// `TRANSFER_MANAGER::can_loan_player(clube, jogador, mensagem)`, 1 pode e 0 não pode. O clube atual é o do contrato
/// do jogador (0x4cf5a0, em [0xae1080]; fora o tipo 5); a data local padrão (0x54a110) e o dia do mês de hoje (0x54cf70:
/// mov ecx, 0xae2c90 -> thiscall, ret 0). Sem clube atual, ou com ficha de contrato (0x4cf6d0: push jogador; mov ecx,
/// [0xae1050], ret 4): 0. Cargo (+0x3d) diferente de 0xb: 0, com o texto do nome (0x53dba0: push 1; push 0x12c; push
/// texto; push jogador; add esp, 0x10) em [0xde1f64] (0x66f4e0, cdecl) se há mensagem. Se o clube não pode chegar ao
/// jogador (0x8ce990: push 0; push &motivo; push clube; push jogador; mov ecx, this, ret 0x10): 0 e, com mensagem, o
/// texto pelo motivo (0x21 o clube da oferta acertada, 0x4cca30, ou o atual, apagando o bit 0x40 do registro de 8
/// bytes em [[0xde1f54]+0x86e] sem oferta; 0x27, 7 e 1 na mensagem; 0x12/0x13; 0x19, 0x28 e 0x1a com o adjetivo da
/// nação, 0x6181d0: push 0xd; push 0x64; push texto; push nação; add esp, 0x10). A regra da nação do clube (das 28 em
/// this+0x8ac, a de +0xa igual a [clube+0x53]): o virtual +0x20 (push 0; push mensagem; push jogador; push clube; mov
/// ecx, regra) igual a 0 dá 0. A regra da nação do clube atual: sem ela, 1. O período do empréstimo (0x8d0590: push
/// &fim_ano; push &fim_mês; push &início_ano; push &início_mês; push jogador; push clube do contrato; push clube; mov
/// ecx, this -> thiscall, ret 0x1c): início igual ao fim dá 1. Senão a data (dia do mês, início; 0x54a130: push −1;
/// push ano; push mês; push dia; lea ecx, data -> thiscall, ret 0x10) e o virtual +0x24 (push 1; push &data; push 0;
/// push jogador; push clube atual; mov ecx, regra) diferente de 0 dá 1; avança um mês (0x549f70: push −1; push fim_ano;
/// push fim_mês; push dia; lea ecx, local -> thiscall, ret 0x10) enquanto a data do fim passar da data, de novo a 0x54a130
/// e o virtual (diferente de 0 dá 1); passou: 0, com "This loan deal cannot go ahead" na mensagem.
unsafe extern "thiscall" fn transfer_manager_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cf70usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let nome: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let acertada: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cca30usize);
    let adjetivo: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x6181d0usize);
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize) = core::mem::transmute(0x8d0590usize);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x54a130usize);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let atual = contract_club_optional_type5(jogador, true);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    let dia = dia_do_mes(0xae2c90);
    if atual == 0 {
        return 0;
    }
    if ficha(cqd2_d(0xae1050), jogador) != 0 {
        return 0;
    }
    let mut buf_nome = [0u8; 0x130];
    let pn = buf_nome.as_mut_ptr() as usize;
    if cqd2_b(jogador + 0x3d) != 0xb {
        if msg != 0 {
            nome(jogador, pn, 0x12c, 1);
            texto(0xde1f64, 0xa8f4dc, pn);
        }
        return 0;
    }
    let mut motivo: u16 = 0;
    crate::comparison::branch("cqd_2 club arrives");
    if chegar(this, jogador, clube, &mut motivo, 0) == 0 { // cqd2: o clube não chega ao jogador
        if msg == 0 {
            return 0;
        }
        nome(jogador, pn, 0x12c, 1);
        let mut adj = [0u8; 0x68];
        let pa = adj.as_mut_ptr() as usize;
        let nacao = match motivo {
            0x21 => {
                let gm = cqd2_d(0xde1f54);
                let id = cqd2_d(jogador);
                let m = cqd2_d(gm + 0x86e).wrapping_add(id.wrapping_mul(8));
                let mut c = atual;
                if cqd2_b(m) & 0x40 != 0 {
                    let a = acertada(cqd2_d(0xae1050), id);
                    if a != 0 {
                        c = cqd2_clube(cqd2_d(a + 4));
                    } else {
                        let m = cqd2_d(gm + 0x86e).wrapping_add(cqd2_d(jogador).wrapping_mul(8));
                        cqd2_pb(m, cqd2_b(m) & 0xbf);
                    }
                }
                let x = cqd2_b(c + 0x52) as i8 as i32;
                texto(0xde1f64, 0xa8f468, x, x, pn, c + 0x38);
                return 0;
            }
            0x27 => {
                texto(msg, 0xa8f420);
                return 0;
            }
            7 => {
                texto(msg, 0xa8f3c4, pn);
                return 0;
            }
            1 => {
                texto(msg, 0xa8f378, pn);
                return 0;
            }
            0x12 | 0x13 => {
                texto(0xde1f64, 0xa8f2bc, pn);
                return 0;
            }
            0x19 => cqd2_d(0xae23a8).wrapping_add(cqd2_d(0x9cf424).wrapping_mul(0x122)),
            0x28 => cqd2_d(clube + 0x53),
            0x1a => cqd2_d(0xae23a8).wrapping_add(cqd2_d(0x9cf488).wrapping_mul(0x122)),
            _ => return 0,
        };
        adjetivo(nacao, pa, 0x64, 0xd);
        let x = cqd2_b(clube + 0x52) as i8 as i32;
        texto(0xde1f64, 0xa8f30c, x, x, 0x64, 0x64, clube + 0x38, pa);
        return 0;
    }
    for k in 0..0x1c {
        let r = cqd2_d(this + 0x8ac + k * 4);
        if cqd2_d(r + 0xa) == cqd2_d(clube + 0x53) {
            if r != 0 && rules_vfunc_20(r)(r, clube, jogador, msg, 0) == 0 {
                return 0;
            }
            break;
        }
    }
    let mut regra = 0;
    for k in 0..0x1c {
        let r = cqd2_d(this + 0x8ac + k * 4);
        if cqd2_d(r + 0xa) == cqd2_d(atual + 0x53) {
            regra = r;
            break;
        }
    }
    if regra == 0 {
        return 1;
    }
    let c2 = contract_club_optional_type5(jogador, false);
    // ponytail: a original lê os quatro como dwords com sobra de pilha nos bytes que a 0x8d0590 não grava (o mês é um
    // byte, o ano uma palavra); a 0x54a130 e a 0x549f70 só leem o byte do mês e a palavra do ano (and ecx, 0xffff)
    let (mut im, mut ia, mut fm, mut fa) = (0u32, 0u32, 0u32, 0u32);
    periodo(this, clube, c2, jogador, &mut im as *mut u32 as usize, &mut ia as *mut u32 as usize,
        &mut fm as *mut u32 as usize, &mut fa as *mut u32 as usize);
    if im as u8 == fm as u8 && ia as u16 == fa as u16 {
        return 1;
    }
    let mut mes = im as u8;
    let mut ano = ia;
    marcar(pd, dia, mes as u32, ano, u32::MAX);
    let v24 = rules_vfunc_24(regra);
    if v24(regra, atual, jogador, 0, pd, 1) != 0 {
        return 1;
    }
    loop {
        mes = mes.wrapping_add(1);
        if (mes as i8) > 0xb {
            mes = 0;
            ano = ano.wrapping_add(1);
        }
        let mut t = [0u8; 8];
        let p = dma(t.as_mut_ptr() as usize, dia, fm, fa, u32::MAX);
        let (na, nd) = (cqd2_w(p + 2) as u16, cqd2_w(p));
        let (da, dd) = (cqd2_w(pd + 2) as u16, cqd2_w(pd));
        if !(na > da || (na == da && nd > dd)) {
            if msg != 0 {
                texto(msg, 0xa8f288);
            }
            return 0;
        }
        marcar(pd, dia, mes as u32, ano, u32::MAX);
        if v24(regra, atual, jogador, 0, pd, 1) != 0 {
            return 1;
        }
    }
}

/// Comparador da 0x8d0040: lado a lado, com [0xae1080], o texto em [0xde1f64] e a mensagem (0x100 bytes), e o byte do
/// registro de 8 bytes do jogador em [[0xde1f54]+0x86e] (o único que ela pode apagar); a devolução também. As regras,
/// o período e as datas não sorteiam nem gravam fora das locais.
unsafe extern "thiscall" fn compare_transfer_manager_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I2%%);
    let mut regioes = [(0xae1080usize, 4usize), (0xde1f64, 0x100), (0, 0), (0, 0)];
    let mut k = 2;
    if msg != 0 {
        regioes[k] = (msg, 0x100);
        k += 1;
    }
    let gm = cqd2_d(0xde1f54);
    if gm != 0 && cqd2_d(gm + 0x86e) != 0 {
        regioes[k] = (cqd2_d(gm + 0x86e).wrapping_add(cqd2_d(jogador).wrapping_mul(8)), 1);
        k += 1;
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&regioes[..k], || o(this, clube, jogador, msg),
        || crate::rng::without_counting(|| transfer_manager_can_loan_player(this, clube, jogador, msg)));
    SCOREBOARD.record(igual && r == n, || format!("transfer_manager_can_loan_player(clube {:#x}, jogador {:#x}): original {} rust {}, memória {}",
        clube, jogador, r, n, if igual { "igual" } else { "diferente" }));
    r
}
