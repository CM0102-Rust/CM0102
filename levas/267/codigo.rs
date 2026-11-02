// ---------- 267a leva: o lance aceito pelo clube vendedor, a notícia da oferta feita e o empréstimo possível (transfer_manager.cpp) ----------

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
unsafe fn cqd2_clube_do_contrato(pessoa: usize, sem_5: bool) -> usize {
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
unsafe fn cqd2_ativa(o: usize) -> bool {
    let s = cqd2_b(o + 0x2c);
    matches!(s, 2 | 3 | 4 | 0x10 | 0x14) || (s == 0xf && cqd2_b(o + 0x2e) >= 3)
}

/// A lista de troca (+4) vazia (palavra +0 zerada) é destruída (0x68d040: mov ecx, lista -> thiscall, ret 0), liberada
/// (0x944c94: push; add esp, 4 -> cdecl) e +4 zera.
unsafe fn cqd2_soltar_troca_vazia(o: usize) {
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
unsafe extern "thiscall" fn aceitar_lance(this: usize, vaga: usize, marcar: u32) {
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x84b580usize);
    let tecnico_humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
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
    let vendedor = cqd2_clube_do_contrato(alvo, true);
    let o = cqd2_d(vaga);
    cqd2_pb(o + 0x2e, 0);
    if marcar != 0 {
        cqd2_pb(o + 0x2d, 1);
        sob_oferta(cqd2_d(this + 0x856), alvo, comprador, cqd2_d(cqd2_d(vaga)));
    }
    cqd2_pb(o + 0x2e, 0);
    cqd2_pb(o + 0x2c, 5);
    let mut humanos: u8 = 0;
    if tecnico_humano(cqd2_clube(cqd2_d(o + 8))) != 0 {
        humanos = 1;
    }
    if vendedor != 0 && tecnico_humano(vendedor) != 0 {
        humanos = humanos.wrapping_add(1);
    }
    aceita(this, o);
    if cqd2_b(o + 0x11) == 0xb {
        let e = if cqd2_humano(comprador) { 1 } else { estatuto(cqd2_d(this + 0x856), alvo, comprador, 0) };
        let dw = (marcar & 0xffff_ff00) | e as u32;
        let mut motivo: u32 = 0;
        let r = interesse(cqd2_d(this + 0x856), alvo, comprador, 0, 0, dw, &mut motivo);
        crate::comparacao::ramo("cqd_2 empréstimo");
        if !(r >= 0.75) { // cqd2: o alvo não quer se mudar
            recusa_mudar(this, o, motivo);
            cqd2_pb(o + 0x2c, 0xf);
            cqd2_pb(o + 0x2e, 3);
            let a2 = cqd2_pessoa(cqd2_d(o + 0xc));
            let atual = cqd2_clube_do_contrato(a2, false);
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
                if !cqd2_ativa(o) {
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
            crate::comparacao::ramo("cqd_2 troca cara");
            if (t as f64 * 1.5) < salario as f64 { // cqd2: o jogador da troca caro demais
                let v = valor(cqd2_d(0xae1050), pessoa, comprador, 0, u32::MAX, 0).wrapping_add(cqd2_d(o + 0x1b) as i32);
                noticia_troca(this, o, pessoa, v);
                tirar(cqd2_d(o + 4), p);
                let m = cqd2_d(this + 0x86e).wrapping_add(cqd2_d(pessoa).wrapping_mul(8)) + 2;
                cqd2_pb(m, cqd2_b(m).wrapping_sub(1));
                cqd2_pd(o + 0x1b, v as u32 as usize);
                if vendedor != 0 && !cqd2_humano(vendedor) {
                    resposta_ia(this, vaga, 1);
                    if cqd2_ativa(o) {
                        cqd2_soltar_troca_vazia(o);
                        return;
                    }
                }
            }
            p = cqd2_proximo(cqd2_d(o + 4));
        }
        cqd2_soltar_troca_vazia(o);
    }
    if humanos != 0 {
        conversa(this, o);
    }
    crate::comparacao::ramo("cqd_2 lance aceito");
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
unsafe extern "thiscall" fn comparar_aceitar_lance(this: usize, vaga: usize, marcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1549)(this, vaga, marcar) // ponytail: só a original
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
unsafe extern "thiscall" fn noticia_da_oferta_feita(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let recorde: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x4486c0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let janela: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(0x8d0bb0usize);
    let mut alvo = cqd2_pessoa(cqd2_d(o + 0xc));
    let vendedor = cqd2_clube_do_contrato(alvo, true);
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
        let c = contrato_de(cqd2_d(0xae1050), alvo);
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
unsafe extern "thiscall" fn comparar_noticia_da_oferta_feita(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1550)(this, o) // ponytail: só a notícia
}

/// O virtual +0x20 da regra de nação `r` (push 0; push mensagem; push jogador; push clube; mov ecx, regra: 4 dwords).
unsafe fn cqd2_regra_20(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 {
    core::mem::transmute(cqd2_d(cqd2_d(r) + 0x20))
}

/// O virtual +0x24 da regra de nação `r` (push 1; push &data; push 0; push jogador; push clube; mov ecx, regra: 5 dwords).
unsafe fn cqd2_regra_24(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 {
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
unsafe extern "thiscall" fn pode_emprestar(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
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
    let atual = cqd2_clube_do_contrato(jogador, true);
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
    crate::comparacao::ramo("cqd_2 clube chega");
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
            if r != 0 && cqd2_regra_20(r)(r, clube, jogador, msg, 0) == 0 {
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
    let c2 = cqd2_clube_do_contrato(jogador, false);
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
    let v24 = cqd2_regra_24(regra);
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
unsafe extern "thiscall" fn comparar_pode_emprestar(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1551);
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
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes[..k], || o(this, clube, jogador, msg),
        || crate::aleatorio::sem_contar(|| pode_emprestar(this, clube, jogador, msg)));
    PLACAR.registrar(igual && r == n, || format!("pode_emprestar(clube {:#x}, jogador {:#x}): original {} rust {}, memória {}",
        clube, jogador, r, n, if igual { "igual" } else { "diferente" }));
    r
}

// ---------- 267a leva: a liga: o dia da liga, a criação dos jogos da temporada e a promoção e o rebaixamento (league.cpp) ----------

unsafe fn cqd5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqd5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O método `k` (deslocamento na tabela virtual) do objeto `o`.
unsafe fn cqd5_metodo(o: usize, k: usize) -> usize {
    cqd5_d(cqd5_d(o) + k)
}

/// As duas datas (dia, ano) são a mesma (as palavras, como a original compara).
unsafe fn cqd5_mesma_data(a: usize, b: usize) -> bool {
    cqd5_w(a + 2) == cqd5_w(b + 2) && cqd5_w(a) == cqd5_w(b)
}

/// { 0x6847c0 } (thiscall, ret 8, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::update_for_date(data, completo)`, o dia de cada liga (pelas competições, 46522 chamadas até 2002-06-01).
/// Liga parada (+0xb5) ou sem clubes (+0x45 = 0): 0. Senão (devolve 1, no byte):
/// - +0xa7 = -1 com times (+0x3e) > 0: +0xa7 = (times − 1) · turnos (+0x3c), na palavra;
/// - na primeira fase (+0x44 = 0xff) com tabela (+0xb1, +0x11 = 0), completo e a data nova (≠ +0xd1): no dia seguinte ao
///   do dia da Copa da nação-chave (0x669470; [0xb63d75 + 72·k] + 1, 0 depois de 364) e no ano da liga (+0x40), a
///   notícia da tabela (0x683ee0: push 0; mov ecx, this -> thiscall, ret 4);
/// - na primeira fase, o dia de cada fase (+0xc, até +0x30; método 0x14 com a data e o completo);
/// - na data +0xc9 (sem +0xdd): +0xad = 0 e o método 0x94;
/// - na primeira fase com tabela e a data nova: o método 0x10 (0, 0); fim da temporada (+0xdd = 1) com todas as fases
///   acabadas (método 0x10 com (0, 1) ≠ 0): a fase seguinte (método 0x28) se +0x30 < +0x2c − 1, senão (uma vez, +0x4c)
///   o histórico (0x442e80: push liga; mov ecx, [0xad9da8] -> thiscall, ret 4), os recordes com 1 (0x447430: push 1;
///   push liga; mov ecx, [0xad9dac] -> thiscall, ret 8) se a competição conta (0x4b0700: push id; add esp, 4 -> cdecl),
///   a caixa da linha 0x74c se +0xe6, o método 0x30 e o fim do anexo +8 (0x4a19f0: mov ecx, [+8] -> thiscall, ret 0);
///   depois os recordes com 0 e o anexo (0x4a2450: mov ecx, [+8] -> thiscall, ret 0);
/// - com tabela e completo, sem +0xdd: os jogos para a TV quatro semanas adiante (0x54c770 + 0x685640: push &data; mov
///   ecx, this -> thiscall, ret 4);
/// - completo: na primeira fase com inscrições (+0x52 bit 2), a data nova, sem +0xdd e tabela em andamento: se a primeira
///   data da tabela da TV (+0xba; dia, ano − o da liga) cai daqui a 5 dias (Argentina [0x9cf218]), 20 (Itália
///   [0x9cf360]) ou 14 dias, o prazo das inscrições (0x859e20: push 0; push 0; push liga; mov ecx, [0xdd7ef0] ->
///   thiscall, ret 0xc); e +0xd1 fica com a data (0x417c10).
unsafe extern "thiscall" fn liga_dia(this: usize, data: usize, completo: u32) -> u32 {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x683ee0usize);
    let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x442e80usize);
    let conta: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x447430usize);
    let fim_anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4a19f0usize);
    let anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4a2450usize);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x685640usize);
    let prazo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x859e20usize);
    // a data pedida copiada no começo (a original guarda o dword do dia|ano e o do bissexto)
    let d0 = [cqd5_d(data) as u32, cqd5_d(data + 4) as u32];
    let pd0 = d0.as_ptr() as usize;
    let (mut t, mut x, mut w) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pt, px, pw) = (t.as_mut_ptr() as usize, x.as_mut_ptr() as usize, w.as_mut_ptr() as usize);
    if cqd5_b(this + 0xb5) != 0 || cqd5_d(this + 0x45) == 0 {
        return 0;
    }
    if cqd5_w(this + 0xa7) == -1 {
        let n = cqd5_w(this + 0x3e);
        if n > 0 {
            cqd5_pw(this + 0xa7, n.wrapping_sub(1).wrapping_mul(cqd5_w(this + 0x3c)) as u16);
        }
    }
    let tab = cqd5_d(this + 0xb1);
    if tab != 0 && cqd5_b(tab + 0x11) == 0 && cqd5_b(this + 0x44) == 0xff && completo != 0 && !cqd5_mesma_data(this + 0xd1, data) {
        let c = cqd5_d(this + 4);
        if cqd5_d(c + 0x5d) != 0 {
            let k = chave(c) as i8 as isize;
            let mut dia = (cqd5_w((0xb63d75 + k * 72) as usize) as u16).wrapping_add(1);
            if dia > 0x16c {
                dia = 0;
            }
            if dia as i32 == cqd5_w(0xae2c90) as i32 && cqd5_w(this + 0x40) as i32 == cqd5_w(0xae2c92) as u16 as i32 {
                noticia(this, 0);
            }
        }
    }
    if cqd5_b(this + 0x44) == 0xff {
        let mut i = 0i32;
        while i <= cqd5_d(this + 0x30) as i32 {
            let fase = cqd5_d(cqd5_d(this + 0xc) + i as usize * 4);
            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(cqd5_metodo(fase, 0x14));
            f(fase, data, completo);
            i += 1;
        }
    }
    if cqd5_w(this + 0xc9 + 2) == cqd5_w(0xae2c92) && cqd5_w(this + 0xc9) == cqd5_w(0xae2c90) && cqd5_d(this + 0xdd) == 0 {
        cqd5_pd(this + 0xad, 0);
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x94));
        f(this);
    }
    let mut seguir_tv = true;
    if cqd5_b(this + 0x44) == 0xff {
        if cqd5_d(this + 0xb1) == 0 {
            seguir_tv = false;
        } else if !cqd5_mesma_data(this + 0xd1, data) {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x10));
            f(this, 0, 0);
            if cqd5_d(this + 0xdd) == 1 {
                let mut acabadas = true;
                let mut i = 0i32;
                while i <= cqd5_d(this + 0x30) as i32 {
                    let fase = cqd5_d(cqd5_d(this + 0xc) + i as usize * 4);
                    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqd5_metodo(fase, 0x10));
                    if g(fase, 0, 1) as u8 == 0 {
                        acabadas = false;
                        break;
                    }
                    i += 1;
                }
                if acabadas {
                    if (cqd5_d(this + 0x30) as i32) < (cqd5_d(this + 0x2c) as i32).wrapping_sub(1) {
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x28));
                        h(this);
                    } else if cqd5_d(this + 0x4c) == 0 {
                        historico(cqd5_d(0xad9da8), this);
                        if conta(cqd5_d(cqd5_d(this + 4)) as u32) != 0 {
                            recordes(cqd5_d(0xad9dac), this, 1);
                        }
                        if cqd5_d(this + 0xe6) != 0 {
                            aviso_em(0x9f15f4, 0x74c);
                        }
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x30));
                        h(this);
                        if cqd5_d(this + 8) != 0 {
                            fim_anexo(cqd5_d(this + 8));
                        }
                        cqd5_pd(this + 0x4c, 1);
                    }
                }
            }
            if conta(cqd5_d(cqd5_d(this + 4)) as u32) != 0 {
                recordes(cqd5_d(0xad9dac), this, 0);
            }
            if cqd5_d(this + 8) != 0 {
                anexo(cqd5_d(this + 8));
            }
        }
    }
    if seguir_tv && cqd5_d(this + 0xb1) != 0 {
        if completo == 0 {
            return 1;
        }
        if cqd5_d(this + 0xdd) == 0 {
            semanas(pd0, px, 4); // cqd5: a TV quatro semanas adiante
            crate::comparacao::ramo("cqd_5 jogos para a tv");
            tv(this, px);
        }
    }
    if completo == 0 {
        return 1;
    }
    if cqd5_b(this + 0x52) & 2 != 0 && cqd5_b(this + 0x44) == 0xff && !cqd5_mesma_data(this + 0xd1, data)
        && cqd5_d(this + 0xdd) == 0 && cqd5_b(cqd5_d(this + 0xb1) + 0x11) == 0 {
        let e = cqd5_d(this + 0xba);
        // ponytail: a original empurra o dia e o ano em registradores com sobra nos bytes altos; a 0x549ef0 só lê as palavras
        nova(pt, cqd5_w(e) as u16 as u32, cqd5_w(e + 2).wrapping_add(cqd5_w(this + 0x40)) as u16 as u32);
        let c = cqd5_d(this + 4);
        let nacao = if c != 0 && cqd5_d(c + 0x5d) != 0 { cqd5_d(cqd5_d(c + 0x5d)) } else { 0 };
        let tem = c != 0 && cqd5_d(c + 0x5d) != 0;
        let dias = if tem && nacao == cqd5_d(0x9cf218) {
            5
        } else if tem && nacao == cqd5_d(0x9cf360) {
            0x14
        } else {
            0xe
        };
        let r = mais(pd0, pw, dias);
        if cqd5_mesma_data(pt, r) {
            prazo(cqd5_d(0xdd7ef0), this, 0, 0);
        }
    }
    copia(this + 0xd1, pw, data);
    1
}

/// O comparador do dia da liga: só a original (ponytail: chama os métodos de cada fase e da liga, as notícias, os
/// recordes e os jogos para a TV, que mexem no calendário: não roda duas vezes). Confere o que a rotina garante: 0 só
/// com a liga parada ou sem clubes e, completo, +0xd1 com a data.
unsafe extern "thiscall" fn comparar_liga_dia(this: usize, data: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1552);
    let parada = cqd5_b(this + 0xb5) != 0 || cqd5_d(this + 0x45) == 0;
    let r = o(this, data, completo);
    let ok = if parada { r as u8 == 0 } else { r as u8 == 1 && (completo == 0 || cqd5_mesma_data(this + 0xd1, data)) };
    PLACAR.registrar(ok, || format!("liga_dia(completo {}): parada {} devolveu {} +0xd1 {:x} data {:x}",
        completo, parada, r as u8, cqd5_d(this + 0xd1), cqd5_d(data)));
    r
}

/// Um jogo da tabela da liga no registro `r` (0x4f bytes) e no calendário: os clubes das entradas `ea` (mandante) e
/// `eb` (0x3b bytes na tabela +0xb1: +0 clube, +0x38 marca), as marcas +0x40/+0x41 (marca 0), os ids +0xc/+0x10 (-1 sem
/// clube), o estádio (0x85c260: push +0x24; push &jogo; push [+0x3d da data]; mov ecx, [0xdd7ef4] -> thiscall, ret
/// 0xc), a data (+0x28 ano + o da liga, +0x2a dia) e a fatia (+0x3f) da entrada `off` da tabela das datas (+0xba), os
/// bits 0..1 de +0x4e zerados e o bit 3 na última rodada (+0x34 = +0xa9 − 1; ou com `tipo4`, data do tipo 4), e o jogo
/// vai ao calendário (0x5aa680: push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 8).
unsafe fn cqd5_jogo(this: usize, r: usize, ea: usize, eb: usize, off: usize, tipo4: bool) {
    let estadio: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x85c260usize);
    let calendario: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let (ea, eb) = (ea, eb); // cqd5: o mandante e o visitante
    let (ca, cb) = (cqd5_d(ea), cqd5_d(eb));
    cqd5_pd(r + 0x1c, ca);
    cqd5_pd(r + 0x20, cb);
    cqd5_pb(r + 0x40, (cqd5_b(ea + 0x38) == 0) as u8);
    cqd5_pb(r + 0x41, (cqd5_b(eb + 0x38) == 0) as u8);
    cqd5_pd(r + 0xc, if ca != 0 { cqd5_d(ca) } else { u32::MAX as usize });
    cqd5_pd(r + 0x10, if cb != 0 { cqd5_d(cb) } else { u32::MAX as usize });
    estadio(cqd5_d(0xdd7ef4), cqd5_d(cqd5_d(this + 0xba) + off + 0x3d) as u32, r, cqd5_d(this + 0x24) as u32);
    let e = cqd5_d(this + 0xba) + off;
    cqd5_pw(r + 0x28, cqd5_w(e + 2).wrapping_add(cqd5_w(this + 0x40)) as u16);
    cqd5_pw(r + 0x2a, cqd5_w(e) as u16);
    cqd5_pb(r + 0x3f, cqd5_b(e + 4));
    cqd5_pw(r + 0x4d, cqd5_w(r + 0x4d) as u16 & 0xfcff);
    if cqd5_w(r + 0x34) as i32 == cqd5_w(this + 0xa9) as i32 - 1 || (tipo4 && cqd5_b(e + 0xb) == 4) {
        cqd5_pb(r + 0x4e, cqd5_b(r + 0x4e) | 8);
    } else {
        cqd5_pw(r + 0x4d, cqd5_w(r + 0x4d) as u16 & 0xf7ff);
    }
    crate::comparacao::ramo("cqd_5 jogo da tabela");
    calendario(cqd5_d(0xae2a58), r, 1);
}

/// { 0x6835c0 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::create_fixtures()`, da montagem das competições (44 chamadas até 2002-06-01). Com n = times (+0x3e) +
/// times % 2: +0xdd = 0; a grade de n linhas de n dwords (0x944cf1; sem memória, a caixa da linha 0x3fc ou 0x405 e 0),
/// montada pela 0x6844b0 (push n; push grade -> stdcall, ret 8: o mov ecx, this antes ela não lê) e, fora das
/// competições [0x9cf79c] e [0x9cf7a4], a ordem dos clubes sorteada (0x686a70: push n; mov ecx, this -> thiscall, ret
/// 4). O registro do jogo (0x4f bytes na pilha): +0 id da competição, +8 = -1, +0x14 a competição, +0x24/+0x26 a última
/// data da tabela (+0xba, entrada +0xa9 − 1), +0x2c ano (+0x40), +0x30 = 0, +0x32 (+0x3a), +0x36 (+0xdb), +0x38
/// (+0xab), +0x3a (+0xc4), +0x3b = 0xff, +0x3d/+0x3e = 0, +0x42 fase (+0x44), +0x43..+0x4c = 0xff, e o método 0x7c da
/// liga com ele. Com a lista própria (+0xea, entradas de 0x10 bytes: +8/+0xc os ids dos clubes) do ano da liga: em cada
/// rodada k (+0xa9) os times/2 jogos da lista (os índices pela tabela +0xb1), e a lista é liberada (0x9462bb +
/// 0x944c94). Senão o rodízio: para j = 1..turnos (+0x3c)·(n − 1), a rodada pela 0x68a030 (push +0xd9; push +0xa9;
/// push +0x3c; push +0x3e; push id; push &passo; push rodada; add esp, 0x1c -> cdecl) e a coluna j % (n − 1) (n − 1 no
/// zero): cada time i com adversário v ≠ 0 na grade apaga a vaga do adversário; o mando pela paridade de v > 0, do turno
/// ((j − 1)/(n − 1)) e do ano + fase; com a nação +0x24, o clube dessa nação (+0x69) fica como mandante. Cada jogo vai
/// ao calendário (cqd5_jogo). No fim, a grade é liberada e as datas do tipo 3 da tabela levam os jogos para a TV
/// (0x685640). Devolve 1.
unsafe extern "thiscall" fn liga_criar_tabela(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let montar: unsafe extern "stdcall" fn(usize, i32) -> u32 = core::mem::transmute(0x6844b0usize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x686a70usize);
    let girar: unsafe extern "C" fn(i32, *mut i8, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x68a030usize);
    let destruir_lista: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x685640usize);
    let mut rodada: i32 = -1;
    let times = cqd5_w(this + 0x3e) as i32;
    let n = times + times % 2;
    let mut passo: i8 = 0;
    let (mut x, mut y, mut z) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pz) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, z.as_mut_ptr() as usize);
    padrao(px);
    cqd5_pd(this + 0xdd, 0);
    let grade = novo((n * 4 + 4) as u32);
    if grade == 0 {
        aviso_em(0x9f15f4, 0x3fc);
        return 0;
    }
    let mut i = 1i32;
    while i <= n {
        let l = novo((n * 4) as u32);
        if l == 0 {
            aviso_em(0x9f15f4, 0x405);
            liberar(grade);
            return 0;
        }
        cqd5_pd(grade + i as usize * 4, l);
        i += 1;
    }
    montar(grade, n);
    let c = cqd5_d(this + 4);
    if c == 0 || (cqd5_d(c) != cqd5_d(0x9cf79c) && cqd5_d(c) != cqd5_d(0x9cf7a4)) {
        crate::comparacao::ramo("cqd_5 ordem sorteada");
        sortear(this, n as u32); // cqd5: a ordem dos clubes
    }
    // o registro do jogo; os bytes que a original não grava (+4..+7, +0x18, +0x2e, +0x39, +0x4d e os bits altos de
    // +0x4e: sobra da pilha dela, que o método 0x7c e a 0x85c260 podem preencher) ficam 0, um valor que o jogo produz
    let mut reg = [0u8; 0x60];
    let r = reg.as_mut_ptr() as usize;
    let c = cqd5_d(this + 4);
    cqd5_pd(r + 0x14, c);
    cqd5_pw(r + 0x32, cqd5_w(this + 0x3a) as u16);
    cqd5_pd(r, cqd5_d(c));
    cqd5_pw(r + 0x2c, cqd5_w(this + 0x40) as u16);
    cqd5_pw(r + 0x38, cqd5_w(this + 0xab) as u16);
    cqd5_pw(r + 0x4d, cqd5_w(r + 0x4d) as u16 & 0xfbff);
    cqd5_pb(r + 0x3a, cqd5_b(this + 0xc4));
    cqd5_pw(r + 0x36, cqd5_w(this + 0xdb) as u16);
    cqd5_pw(r + 0x30, 0);
    for k in 0x43..=0x4cusize {
        cqd5_pb(r + k, 0xff);
    }
    cqd5_pd(r + 8, u32::MAX as usize);
    cqd5_pb(r + 0x42, cqd5_b(this + 0x44));
    cqd5_pb(r + 0x3d, 0);
    cqd5_pb(r + 0x3e, 0);
    cqd5_pb(r + 0x3b, 0xff);
    let preparar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cqd5_metodo(this, 0x7c));
    preparar(this, r);
    let ult = cqd5_d(this + 0xba).wrapping_add((cqd5_w(this + 0xa9) as i32 * 0x41) as usize);
    // ponytail: a original empurra o dia e o ano com sobra nos bytes altos; a 0x549ef0 só lê as palavras
    let yy = nova(py, cqd5_w(ult - 0x41) as u16 as u32, cqd5_w(ult - 0x3f).wrapping_add(cqd5_w(this + 0x40)) as u16 as u32);
    copia(px, pz, yy);
    cqd5_pw(r + 0x26, cqd5_w(px) as u16);
    cqd5_pw(r + 0x24, cqd5_w(px + 2) as u16);
    let lista = cqd5_d(this + 0xea);
    if lista != 0 && cqd5_w(this + 0x40) as i32 == cqd5_w(lista + 2) as u16 as i32 {
        let mut k = 0i32;
        let mut off = 0usize;
        while k < cqd5_w(this + 0xa9) as i32 {
            let mut i = 0i32;
            loop {
                let times = cqd5_w(this + 0x3e) as i32;
                let metade = times / 2;
                if i >= metade {
                    break;
                }
                cqd5_pw(r + 0x34, k as u16);
                let e = cqd5_d(this + 0xea) + ((metade * k + i) as usize) * 0x10;
                let (mut ia, mut ib) = (-1i32, -1i32);
                if times > 0 {
                    let (a, b) = (cqd5_d(e + 8), cqd5_d(e + 0xc));
                    let tb = cqd5_d(this + 0xb1);
                    for p in 0..times {
                        let id = cqd5_d(cqd5_d(tb + p as usize * 0x3b));
                        if a == id {
                            ia = p;
                        }
                        if b == id {
                            ib = p;
                        }
                    }
                }
                let tb = cqd5_d(this + 0xb1);
                cqd5_jogo(this, r, tb.wrapping_add((ia as isize * 0x3b) as usize), tb.wrapping_add((ib as isize * 0x3b) as usize), off, true);
                i += 1;
            }
            k += 1;
            off += 0x41;
        }
        let lista = cqd5_d(this + 0xea);
        if lista != 0 {
            destruir_lista(lista, 0x10, cqd5_d(lista - 4) as u32, 0x5a53c0);
            liberar(lista - 4);
        }
        cqd5_pd(this + 0xea, 0);
    } else {
        let mut j = 1i32;
        while j <= cqd5_w(this + 0x3c) as i32 * (n - 1) {
            // ponytail: a original empurra as palavras +0x3e, +0x3c, +0xa9 e +0xd9 com sobra nos bytes altos; a 0x68a030
            // só lê as palavras e o byte
            rodada = girar(rodada, &mut passo, cqd5_d(cqd5_d(this + 4)) as u32, cqd5_w(this + 0x3e) as u16 as u32,
                cqd5_w(this + 0x3c) as u16 as u32, cqd5_w(this + 0xa9) as u16 as u32, cqd5_w(this + 0xd9) as u16 as u32);
            let mut col = j % (n - 1);
            if col == 0 {
                col = n - 1;
            }
            cqd5_pw(r + 0x34, rodada as u16);
            if cqd5_w(this + 0x3e) >= 1 {
                let mut i = 0i32;
                loop {
                    let v = cqd5_d(cqd5_d(grade + 4 + i as usize * 4) + col as usize * 4) as i32;
                    if v != 0 {
                        let turno = ((j - 1) / (n - 1)) % 2 != 0;
                        let paridade = (cqd5_w(this + 0x40) as i32 + cqd5_b(this + 0x44) as i8 as i32) % 2 != 0;
                        let idx = if v > 0 { v - 1 } else { -1 - v };
                        let linha = cqd5_d(grade + 4 + (idx as isize * 4) as usize);
                        let (a, b) = if (v > 0) ^ turno ^ paridade { (idx, i) } else { (i, idx) };
                        cqd5_pd(linha + col as usize * 4, 0);
                        let ult = cqd5_w(this + 0x3e) as i32 - 1;
                        if a >= 0 && a <= ult && b >= 0 && b <= ult {
                            let tb = cqd5_d(this + 0xb1);
                            let (ea, eb) = (tb + a as usize * 0x3b, tb + b as usize * 0x3b);
                            let sel = cqd5_d(this + 0x24) as u32;
                            let (m, f) = if sel != u32::MAX {
                                let nac = cqd5_d(cqd5_d(eb) + 0x69);
                                if nac != 0 && cqd5_d(nac) as u32 == sel { (eb, ea) } else { (ea, eb) }
                            } else {
                                (ea, eb)
                            };
                            cqd5_jogo(this, r, m, f, (rodada as isize * 0x41) as usize, false);
                        }
                    }
                    i += 1;
                    if i + 1 > cqd5_w(this + 0x3e) as i32 {
                        break;
                    }
                }
            }
            j += 1;
        }
    }
    let mut i = 0i32;
    while i < n {
        liberar(cqd5_d(grade + 4 + i as usize * 4));
        i += 1;
    }
    liberar(grade);
    let mut k = 0i32;
    while k < cqd5_w(this + 0xa9) as i32 {
        let e = cqd5_d(this + 0xba) + k as usize * 0x41;
        if cqd5_b(e + 0xb) == 3 {
            nova(py, cqd5_w(e) as u16 as u32, cqd5_w(e + 2).wrapping_add(cqd5_w(this + 0x40)) as u16 as u32);
            tv(this, py);
        }
        k += 1;
    }
    1
}

/// O comparador da criação dos jogos: só a original (ponytail: aloca a grade, sorteia a ordem dos clubes e põe os
/// jogos no calendário, que aloca: não roda duas vezes). Confere o que a rotina garante: com a lista própria do ano, ela
/// é liberada (+0xea = 0) quando devolve 1.
unsafe extern "thiscall" fn comparar_liga_criar_tabela(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1553);
    let lista = cqd5_d(this + 0xea);
    let propria = lista != 0 && cqd5_w(this + 0x40) as i32 == cqd5_w(lista + 2) as u16 as i32;
    let r = o(this);
    let ok = !(propria && r == 1) || cqd5_d(this + 0xea) == 0;
    PLACAR.registrar(ok, || format!("liga_criar_tabela: lista própria {} devolveu {} +0xea {:x}", propria, r, cqd5_d(this + 0xea)));
    r
}

/// { 0x689c80 } (ret 0x18, league.cpp): `league_promote_and_relegate(acima, abaixo, contratos, aos_pares, sobem,
/// descem)`, do fim das ligas (depois de 2002-06-01). Recebe o ecx de quem chama e o repassa à 0x6830b0, que não o lê:
/// thiscall aqui para o guardar. Com `contratos`, os clubes da liga de baixo que não sobem (+0x37 ≠ 0 e ≠ 5) acionam
/// as cláusulas de não promoção (0x4cced0: push liga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8). Aos pares:
/// cada rebaixado de cima (+0x37 = 3) troca com o primeiro de baixo que sobe (+0x37 = 0 ou 5): esse sobe (0x6830b0:
/// push 0; push competição de cima; push clube; ecx de quem chama -> thiscall, ret 0xc), o rebaixado guarda a
/// competição antiga (+0x5b) e fica com a de baixo (+0x57), aciona as cláusulas de rebaixamento (0x4cd030: push
/// antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8) e, numa competição sem registro ativo e com técnico
/// humano (0x600a50), perde o técnico (0x69b790: push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall,
/// ret 0x10); os dois ficam com +0x37 = 0xff. Senão, todos os rebaixados descem assim e depois todos os de baixo que
/// sobem sobem. Os contados que diferem dos esperados (0xff = qualquer) dão a caixa da linha 0x15dc (sobem) ou 0x15e2
/// (descem).
unsafe extern "thiscall" fn liga_promover_e_rebaixar(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cced0usize);
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6830b0usize);
    let rebaixamento: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let sem_tecnico: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let (mut desceram, mut subiram) = (0u8, 0u8);
    if contratos != 0 {
        let mut i = 0i32;
        while i < cqd5_w(abaixo + 0x3e) as i32 {
            let e = cqd5_d(abaixo + 0xb1) + i as usize * 0x3b;
            let s = cqd5_b(e + 0x37);
            if s != 5 && s != 0 {
                clausulas(cqd5_d(0xae1050), cqd5_d(e), cqd5_d(abaixo + 4));
            }
            i += 1;
        }
    }
    // o rebaixado desce para a competição de baixo e perde o técnico humano numa competição sem registro ativo
    let descer = |clube: usize| {
        let comp = cqd5_d(abaixo + 4);
        let antiga = cqd5_d(clube + 0x57);
        cqd5_pd(clube + 0x5b, antiga);
        cqd5_pd(clube + 0x57, comp);
        rebaixamento(cqd5_d(0xae1050), clube, antiga);
        let tab = cqd5_d(0xadadfc);
        if tab == 0 || comp == 0 || cqd5_d(tab + cqd5_d(comp) * 4) == 0 {
            if humano(clube, 1, 1, 0, 0) != 0 {
                sem_tecnico(cqd5_d(0xb67a44), clube, 1, 7, 0);
            }
        }
    };
    if aos_pares != 0 {
        let mut i = 0i32;
        while i < cqd5_w(acima + 0x3e) as i32 {
            let ue = cqd5_d(acima + 0xb1) + i as usize * 0x3b;
            if cqd5_b(ue + 0x37) == 3 {
                let n = cqd5_w(abaixo + 0x3e) as i32;
                let mut j = 0i32;
                while j < n {
                    let s = cqd5_b(cqd5_d(abaixo + 0xb1) + j as usize * 0x3b + 0x37);
                    if s == 0 || s == 5 {
                        break;
                    }
                    j += 1;
                }
                if j < n {
                    let le = cqd5_d(abaixo + 0xb1) + j as usize * 0x3b;
                    subir(ecx, cqd5_d(le), cqd5_d(acima + 4), 0);
                    descer(cqd5_d(ue));
                    desceram = desceram.wrapping_add(1);
                    subiram = subiram.wrapping_add(1);
                    cqd5_pb(le + 0x37, 0xff);
                    cqd5_pb(ue + 0x37, 0xff);
                }
            }
            i += 1;
        }
    } else {
        let mut i = 0i32;
        while i < cqd5_w(acima + 0x3e) as i32 {
            let ue = cqd5_d(acima + 0xb1) + i as usize * 0x3b;
            if cqd5_b(ue + 0x37) == 3 {
                descer(cqd5_d(ue));
                desceram = desceram.wrapping_add(1);
                cqd5_pb(ue + 0x37, 0xff);
            }
            i += 1;
        }
        let mut i = 0i32;
        while i < cqd5_w(abaixo + 0x3e) as i32 {
            let le = cqd5_d(abaixo + 0xb1) + i as usize * 0x3b;
            let s = cqd5_b(le + 0x37);
            if s == 0 || s == 5 {
                subir(ecx, cqd5_d(le), cqd5_d(acima + 4), 0);
                subiram = subiram.wrapping_add(1);
                cqd5_pb(le + 0x37, 0xff);
            }
            i += 1;
        }
    }
    if sobem as u8 != 0xff && subiram != sobem as u8 {
        aviso_em(0x9f15f4, 0x15dc);
    }
    if descem as u8 != 0xff && desceram != descem as u8 {
        aviso_em(0x9f15f4, 0x15e2);
    }
}

/// O comparador da promoção e do rebaixamento: só a original (ponytail: os contratos, o técnico demitido e as notícias
/// não rodam duas vezes); a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_liga_promover_e_rebaixar(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(1554)(ecx, acima, abaixo, contratos, aos_pares, sobem, descem)
}

// ---------- 267a leva: o calendário: o dia do calendário, a lista dos jogos do dia e a carga dos jogos do ano (fix_man.cpp) ----------

unsafe fn cqd6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O registro do clube `i` (0x245 bytes em [0xae23b4]; a original calcula base + i + 580·i).
unsafe fn cqd6_clube(i: usize) -> usize {
    cqd6_d(0xae23b4).wrapping_add(i).wrapping_add(i.wrapping_mul(0x244))
}

/// O clube tem técnico humano (0x600a50: push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl).
unsafe fn cqd6_humano(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 1, 1, 0, 0) != 0
}

/// { 0x5aaf00 } (thiscall, ret 0, fix_man.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `FIXTURE_MANAGER::daily_update()`, do laço do dia (0x5cc850). Pelo dia da semana de hoje (0x54a3f0 em 0xae2c90):
/// 1. dias 2 e 3: os jogos de reservas para daqui a 3 dias (0x54c5b0 + 0x7e6380: push &data; mov ecx, [0xb74434] ->
///    thiscall, ret 4) e, no dia 3, também para amanhã;
/// 2. a fila das notícias de jogo ([+0x1128], uma lista): cada notícia vai ao técnico do clube da vaga 0 se a vaga 0xe
///    pede e ao do clube da vaga 1 se a vaga 0xf pede (0x785830: push k; mov ecx, notícia -> thiscall, ret 4;
///    0x7861e0: push clube; push notícia; mov ecx, [0xb7431c] -> thiscall, ret 8); a lista é destruída (0x68d040 +
///    0x944c94) e [+0x1128] fica 0;
/// 3. dias 5 e 6, se ontem é do mesmo ano: nas listas de ontem (as 3 fatias, as nações-chave até [0xb64738]), de trás
///    para a frente, os amistosos ([0x9cf684], tipo +0x3a = 1) de dois clubes sem técnico humano saem da lista (0x68d1d0 +
///    0x68d280); o primeiro jogo que não é amistoso desse tipo encerra a lista;
/// 4. a lista das conferências das pernas ([0xae2a48]): cada entrada cuja copa (ou fase +4 dela) tem +0x43 = 1 rotula
///    os jogos seguintes da rodada (0x522c70: push +0xb; push +7; push rodada; ecx = a copa -> thiscall, ret 0xc); a
///    lista é esvaziada (0x68d080).
unsafe extern "thiscall" fn calendario_dia(this: usize) {
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let reservas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e6380usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anterior: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1d0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d280usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x785830usize);
    let avisar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let rotular: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x522c70usize);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    let dia = semana(0xae2c90) as i16;
    if dia == 2 || dia == 3 {
        mais(0xae2c90, pa, 3);
        reservas(cqd6_d(0xb74434), pa);
        if dia == 3 {
            mais(0xae2c90, pb, 1);
            crate::comparacao::ramo("cqd_6 reservas de amanha");
            reservas(cqd6_d(0xb74434), pb); // cqd6: as reservas de amanhã
        }
    }
    let l = cqd6_d(this + 0x1128);
    if l != 0 {
        cqd6_pd(l + 0xa, cqd6_d(l + 2));
        let mut n = proximo(l);
        while n != 0 {
            let ca = cqd6_clube(vaga(n, 0) as usize);
            let cb = cqd6_clube(vaga(n, 1) as usize);
            if vaga(n, 0xe) != 0 {
                avisar(cqd6_d(0xb7431c), n, ca);
            }
            if vaga(n, 0xf) != 0 {
                avisar(cqd6_d(0xb7431c), n, cb);
            }
            n = proximo(cqd6_d(this + 0x1128));
        }
        let l = cqd6_d(this + 0x1128);
        if l != 0 {
            destruir(l);
            liberar(l);
        }
        cqd6_pd(this + 0x1128, 0);
    }
    if dia == 5 || dia == 6 {
        let t = menos(0xae2c90, pa, 1);
        if cqd6_w(t + 2) == cqd6_w(0xae2c92) {
            let ontem = cqd6_w(menos(0xae2c90, pc, 1)) as u16 as usize;
            for fatia in 0..3usize {
                let listas = cqd6_d(this + (ontem * 3 + fatia) * 4);
                let mut k: i8 = 0;
                while k < cqd6_b(0xb64738) as i8 {
                    let lista = cqd6_d(listas + k as isize as usize * 4);
                    let mut no = cqd6_d(lista + 6);
                    cqd6_pd(lista + 0xa, no);
                    while no != 0 {
                        let f = cqd6_d(no);
                        if cqd6_d(f) != cqd6_d(0x9cf684) || cqd6_b(f + 0x3a) != 1 {
                            break;
                        }
                        if !cqd6_humano(cqd6_d(f + 0x1c)) && !cqd6_humano(cqd6_d(f + 0x20)) {
                            anterior(lista);
                            crate::comparacao::ramo("cqd_6 amistoso descartado");
                            tirar(lista, no); // cqd6: o amistoso de ontem sai da lista
                        } else {
                            anterior(lista);
                        }
                        no = cqd6_d(lista + 0xa);
                    }
                    k = k.wrapping_add(1);
                }
            }
        }
    }
    if cqd6_w(0xae2a48) != 0 {
        cqd6_pd(0xae2a52, cqd6_d(0xae2a4a));
        let mut e = proximo(0xae2a48);
        while e != 0 {
            let i = cqd6_d(e) as i32;
            let tab = cqd6_d(0xadadfc);
            if i >= 0 && tab != 0 {
                let mut cp = cqd6_d(tab + i as usize * 4);
                if cp != 0 {
                    let fase = cqd6_b(e + 4) as i8;
                    if fase as i32 <= cqd6_d(cp + 0x30) as i32 {
                        if fase != -1 {
                            cp = cqd6_d(cqd6_d(cp + 0xc) + fase as isize as usize * 4);
                        }
                        if cqd6_b(cp + 0x43) == 1 {
                            // ponytail: a original empurra eax com a palavra da rodada e os bytes altos do ponteiro da
                            // entrada; a 0x522c70 só lê a palavra
                            rotular(cp, cqd6_w(e + 5) as u16 as u32, cqd6_d(e + 7) as u32, cqd6_d(e + 0xb) as u32);
                        }
                    }
                }
            }
            e = proximo(0xae2a48);
        }
        esvaziar(0xae2a48);
    }
}

/// O comparador do dia do calendário: só a original (ponytail: as notícias vão aos técnicos, os nós dos amistosos e
/// das filas são liberados e os jogos de reservas entram no calendário: não roda duas vezes). Confere o que a rotina
/// garante: a fila das notícias some ([+0x1128] = 0 se havia) e a lista das conferências fica vazia.
unsafe extern "thiscall" fn comparar_calendario_dia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1555);
    let havia = cqd6_d(this + 0x1128);
    let antes = cqd6_w(0xae2a48);
    o(this);
    let fila = cqd6_d(this + 0x1128);
    let depois = cqd6_w(0xae2a48);
    PLACAR.registrar((havia == 0 || fila == 0) && (antes == 0 || depois == 0),
        || format!("calendario_dia: fila das notícias {:x} -> {:x}, conferências {} -> {}", havia, fila, antes, depois));
}

/// { 0x5abf10 } (thiscall, ret 0x10, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::get_fixture_list(ano, dia, fatia, tipo)`: a lista dos jogos do ano, dia (palavra), fatia (byte) e
/// nação-chave (byte com sinal) na corrente dos blocos dos anos. Pelo bloco do ano (+0x1130): se o outro (+0x112c) não
/// é do ano, ele é trocado para o ano (0x5aa2d0: push ano; ecx = o outro -> thiscall, ret 4); sem o outro, um bloco
/// novo (0x944cf1, 0x1184 bytes; 0x5a9cf0: push 0; push this; push ano; ecx = o novo -> thiscall, ret 0xc) entra em
/// +0x112c; sem memória, a caixa da linha 0x681 e 0. A lista: [[bloco + (dia·3 + fatia)·4] + tipo·4].
unsafe extern "thiscall" fn calendario_lista(this: usize, ano: u32, dia: u32, fatia: u32, tipo: u32) -> usize {
    let trocar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x5a9cf0usize);
    let mut b = this;
    while cqd6_w(b + 0x1130) != ano as i16 {
        let o = cqd6_d(b + 0x112c);
        if o == 0 {
            let m = novo(0x1184);
            let r = if m == 0 { 0 } else { construir(m, ano, b, 0) };
            cqd6_pd(b + 0x112c, r);
            if r == 0 {
                aviso_em(0x9cc208, 0x681);
                return 0;
            }
        } else if cqd6_w(o + 0x1130) != ano as i16 {
            trocar(o, ano);
        }
        b = cqd6_d(b + 0x112c);
    }
    let i = (dia & 0xffff) as usize * 3 + (fatia & 0xff) as usize;
    cqd6_d(cqd6_d(b + i * 4) + (tipo as u8 as i8) as isize as usize * 4)
}

/// O comparador da lista do dia: com o ano no bloco ou no outro bloco, as duas (só leem) e a mesma lista; senão só a
/// original (ponytail: troca o bloco do outro ano, que grava e lê o arquivo, ou aloca um bloco novo).
unsafe extern "thiscall" fn comparar_calendario_lista(this: usize, ano: u32, dia: u32, fatia: u32, tipo: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize>(1556);
    let outro = cqd6_d(this + 0x112c);
    let direto = cqd6_w(this + 0x1130) == ano as i16 || (outro != 0 && cqd6_w(outro + 0x1130) == ano as i16);
    if !direto {
        return o(this, ano, dia, fatia, tipo);
    }
    let r = o(this, ano, dia, fatia, tipo);
    let n = calendario_lista(this, ano, dia, fatia, tipo);
    PLACAR.registrar(r == n, || format!("calendario_lista(ano {} dia {} fatia {} tipo {}): original={:x} rust={:x}",
        ano as u16, dia as u16, fatia as u8, tipo as u8 as i8, r, n));
    r
}

/// { 0x5ac3b0 } (thiscall, ret 0, fix_man.cpp): `FIXTURE_MANAGER::load_fixtures()`: o nome `fixtures_<ano>.tmp` em
/// +0x1133 (0x944c9f) e, com ano (+0x1130, sem sinal) desde 1980 e o arquivo em CM3_TEMP (0x90bde0: push nome; push
/// "CM3_TEMP"; add esp, 8 -> cdecl; 0x945b00 fopen com "rb"), a conta (dword) e os jogos de 0x4f bytes (0x945bfe
/// fread; falhando, a caixa da linha 0x730 ou 0x73e e o arquivo fechado): os ids dos clubes (+0xc, +0x10 → +0x1c,
/// +0x20), da competição (+0 → +0x14, 0x6b bytes em [0xae23d0]) e do estádio (+4 → +0x18, 0x4e bytes em [0xae23b0], 0
/// fora de [0xae2358]) viram ponteiros (id -1: o jogo para, linhas 0x74b, 0x751, 0x758), e o jogo entra (0x68d0d0: push
/// 0x4f; push &jogo; ecx = a lista -> thiscall, ret 8; falhando, o jogo para, 0x785) na lista do dia (+0x2a), fatia
/// (+0x3f) e nação-chave do jogo (0x48dd70: push &jogo; push competição; add esp, 8 -> cdecl). No fim, fclose
/// (0x945a09).
unsafe extern "thiscall" fn calendario_carregar_ano(this: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let nacao: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x48dd70usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let nome = this + 0x1133;
    let ano = cqd6_w(this + 0x1130) as u16;
    formatar(nome, 0x9cc238, ano as u32);
    if ano < 0x7bc {
        return;
    }
    let f = abrir(caminho(0x987da0, nome), 0x98f928);
    if f == 0 {
        return;
    }
    let mut conta = 0i32;
    if ler(&mut conta as *mut i32 as usize, 4, 1, f) != 1 {
        aviso_em(0x9cc208, 0x730);
        fechar(f);
        return;
    }
    let mut jogo = [0u8; 0x4f];
    let j = jogo.as_mut_ptr() as usize;
    let mut i = 0i32;
    while i < conta {
        if ler(j, 0x4f, 1, f) != 1 {
            aviso_em(0x9cc208, 0x73e);
            fechar(f);
            return;
        }
        let a = cqd6_d(j + 0xc);
        if a as u32 == u32::MAX {
            erro_em(0x9cc208, 0x74b);
        }
        cqd6_pd(j + 0x1c, cqd6_clube(a));
        let b = cqd6_d(j + 0x10);
        if b as u32 == u32::MAX {
            erro_em(0x9cc208, 0x751);
        }
        cqd6_pd(j + 0x20, cqd6_clube(b));
        let c = cqd6_d(j);
        if c as u32 == u32::MAX {
            erro_em(0x9cc208, 0x758);
        }
        let comp = cqd6_d(0xae23d0).wrapping_add(c.wrapping_mul(0x6b));
        cqd6_pd(j + 0x14, comp);
        let e = cqd6_d(j + 4) as i32;
        let estadio = if e != -1 && e < cqd6_d(0xae2358) as i32 {
            cqd6_d(0xae23b0).wrapping_add((e as usize).wrapping_mul(0x4e))
        } else {
            0
        };
        cqd6_pd(j + 0x18, estadio);
        let k = nacao(comp, j) as i8;
        let dia = cqd6_w(j + 0x2a) as u16 as usize; // cqd6: o dia do jogo carregado
        let lista = cqd6_d(cqd6_d(this + (dia * 3 + cqd6_b(j + 0x3f) as usize) * 4) + k as isize as usize * 4);
        crate::comparacao::ramo("cqd_6 jogo carregado");
        if acrescentar(lista, j, 0x4f) == 0 {
            erro_em(0x9cc208, 0x785);
        }
        i += 1;
    }
    fechar(f);
}

/// O comparador da carga dos jogos do ano: só a original (ponytail: lê o arquivo e aloca os nós das listas; não roda
/// duas vezes). Confere o nome gravado em +0x1133.
unsafe extern "thiscall" fn comparar_calendario_carregar_ano(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1557)(this);
    let esperado = format!("fixtures_{}.tmp", cqd6_w(this + 0x1130) as u16);
    let s = core::slice::from_raw_parts((this + 0x1133) as *const u8, esperado.len() + 1);
    PLACAR.registrar(&s[..esperado.len()] == esperado.as_bytes() && s[esperado.len()] == 0,
        || format!("calendario_carregar_ano: nome {:?} (esperado {})", s, esperado));
}

// ---------- 267a leva: a copa e as fases: a árvore dos confrontos da copa, a primeira fase do grupo e a fase atual (Cup.cpp, comp.cpp) ----------

unsafe fn cqd7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd7_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqd7_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O confronto `p` (0x35 bytes) da rodada `r` da copa (vetores em [+0xad]).
unsafe fn cqd7_confronto(copa: usize, r: i32, p: i32) -> usize {
    cqd7_d(cqd7_d(copa + 0xad) + (r as isize * 4) as usize).wrapping_add((p as isize * 0x35) as usize)
}

/// A rodada `r` da copa (0x68 bytes na tabela [+0xa3]).
unsafe fn cqd7_rodada(copa: usize, r: i32) -> usize {
    cqd7_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// { 0x518790 } (thiscall, ret 0, Cup.cpp): `Cup::map_fixture_tree()`, da criação das copas (22 chamadas até
/// 2002-06-01). Os vetores dos confrontos de cada rodada (0x944e46: 4 por rodada e 0x35 por confronto; sem memória, a
/// caixa "Memory Allocation Error" da linha 0x8b ou 0x9e e o jogo para; rodada sem confrontos, a caixa da linha 0x95 e
/// -1); a marca +5 de cada time (+0xa7, 6 bytes) e +0xab/+0xac zeradas. Com clubes (+0x45): cada confronto vazio (os
/// times e as vagas −1, +8 = 0, +0x14..+0x31 = 0xff, +0x32..+0x34 = 0). Nas rodadas até a penúltima (a antepenúltima com
/// a última do tipo 0x8c): a ordem dos confrontos (embaralhada aos pares sorteados se a rodada seguinte tem menos da
/// metade); cada um ganha a vaga na rodada seguinte: pelo par (rodada +0xd bit 0x200: p/2) ou sorteada (com o bit 0x400,
/// a primeira vaga toda livre a partir dela), o lado pela moeda (0x90cfc0(2); ocupada, a vaga seguinte e outra moeda);
/// sem vaga livre, uma vaga sorteada duas rodadas adiante (sem ela, o jogo para: linhas 0x175 e 0x1a0). O confronto
/// guarda a vaga (+6), o lado (+8: 1 mandante) e a rodada de destino (+0x12); a vaga guarda o confronto (+0xa/+0xc) e a
/// rodada de origem (+0xe/+0x10). Depois os times de cada rodada (+0x1c, a partir de +0x1e) entram nos confrontos dela:
/// o primeiro pelo par ou sorteado (0x400: o primeiro todo livre), o lado pela moeda, e numa vaga ocupada o time da
/// nação +0x24 vira mandante (sem vaga, a caixa da linha 0x1c2 ou 0x222 e -1). Sem confrontos na primeira rodada, +0xb1
/// = 1. Por fim o número do primeiro confronto de cada rodada (+0x5b), de trás para a frente. Devolve 1.
unsafe extern "thiscall" fn copa_arvore_dos_confrontos(this: usize) -> i32 {
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let t = reservar((cqd7_w(this + 0x3c) as i32 as u32) << 2);
    cqd7_pd(this + 0xad, t);
    if t == 0 {
        cpq11_sem_memoria(0x8b, 0x9b5600);
    }
    let mut i = 0i32;
    while i < cqd7_w(this + 0x3c) as i32 {
        let n = cqd7_w(cqd7_rodada(this, i) + 0x1a);
        if n < 1 {
            aviso_em(0x9b55ac, 0x95);
            return -1;
        }
        let v = reservar((n as i32 * 0x35) as u32);
        cqd7_pd(cqd7_d(this + 0xad) + i as usize * 4, v);
        if cqd7_d(cqd7_d(this + 0xad) + i as usize * 4) == 0 {
            cpq11_sem_memoria(0x9e, 0x9b55d8);
        }
        i += 1;
    }
    if cqd7_d(this + 0xa7) != 0 {
        let mut k = 0i32;
        while k < cqd7_w(this + 0x3e) as i32 {
            cqd7_pb(cqd7_d(this + 0xa7) + k as usize * 6 + 5, 0);
            k += 1;
        }
    }
    cqd7_pb(this + 0xab, 0);
    cqd7_pb(this + 0xac, 0);
    if cqd7_d(this + 0x45) == 0 {
        return 1;
    }
    let mut i = 0i32;
    while i < cqd7_w(this + 0x3c) as i32 {
        let mut j = 0i32;
        while j < cqd7_w(cqd7_rodada(this, i) + 0x1a) as i32 {
            let c = cqd7_confronto(this, i, j);
            for k in [0usize, 2, 6, 0xa, 0xc, 0xe, 0x10, 0x12] {
                cqd7_pw(c + k, 0xffff);
            }
            cqd7_pw(c + 8, 0);
            for k in 0x14..=0x31usize {
                cqd7_pb(c + k, 0xff);
            }
            cqd7_pb(c + 0x32, 0);
            cqd7_pb(c + 0x33, 0);
            cqd7_pb(c + 0x34, 0);
            j += 1;
        }
        i += 1;
    }
    let nr = cqd7_w(this + 0x3c);
    let m = (if cqd7_w(cqd7_rodada(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut r = 0i32;
    while r < m - 1 {
        let n = cqd7_w(cqd7_rodada(this, r) + 0x1a);
        let ordem = if n != 0 { reservar(n as i32 as u32) } else { 0 };
        let mut k = 0i32;
        while k < cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32 {
            cqd7_pb(ordem + k as usize, k as u8);
            k += 1;
        }
        let nn = cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32;
        let prox = cqd7_w(cqd7_rodada(this, r) + 0x82) as i32;
        if prox < nn / 2 && nn > 0 {
            let (mut q, mut conta) = (0i32, nn);
            loop {
                let a = sorteio(conta) as usize;
                let b = sorteio(cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32) as usize;
                let x = cqd7_b(ordem + a);
                cqd7_pb(ordem + a, cqd7_b(ordem + b));
                cqd7_pb(ordem + b, x);
                q += 1;
                conta = cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32;
                if q >= conta {
                    break;
                }
            }
        }
        let mut k = 0i32;
        while k < cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32 {
            let rr = cqd7_rodada(this, r);
            let p = cqd7_b(ordem + k as usize) as i8 as i32;
            let mut s: i32;
            let mut inicio: i32;
            if cqd7_w(rr + 0x75) as u16 & 0x200 != 0 {
                s = p / 2; // cqd7: a vaga pelo par
                inicio = s;
            } else {
                s = sorteio(cqd7_w(rr + 0x82) as i32);
                inicio = s;
                if cqd7_w(rr + 0x75) as u16 & 0x400 != 0 {
                    loop {
                        let v = cqd7_confronto(this, r + 1, s as i16 as i32);
                        if cqd7_w(v + 0xa) == -1 && cqd7_w(v + 0xc) == -1 {
                            inicio = s;
                            break;
                        }
                        s += 1;
                        if s as i16 >= cqd7_w(rr + 0x82) {
                            s = 0;
                        }
                        if s as i16 == inicio as i16 {
                            break;
                        }
                    }
                }
            }
            let (mut s, inicio) = (s, inicio); // cqd7: a vaga inicial na rodada seguinte
            let c = cqd7_confronto(this, r, p);
            loop {
                let moeda = sorteio(2);
                let v = cqd7_confronto(this, r + 1, s as i16 as i32);
                let primeiro_a = moeda != 0; // cqd7: a moeda do lado na rodada seguinte
                let lado = if primeiro_a {
                    if cqd7_w(v + 0xa) == -1 { Some(true) } else if cqd7_w(v + 0xc) == -1 { Some(false) } else { None }
                } else if cqd7_w(v + 0xc) == -1 {
                    Some(false)
                } else if cqd7_w(v + 0xa) == -1 {
                    Some(true)
                } else {
                    None
                };
                if let Some(a) = lado {
                    crate::comparacao::ramo("cqd_7 vaga na rodada seguinte");
                    cqd7_pw(v + if a { 0xa } else { 0xc }, p as u16);
                    cqd7_pw(v + if a { 0xe } else { 0x10 }, r as u16);
                    cqd7_pw(c + 6, s as u16);
                    cqd7_pw(c + 8, a as u16);
                    cqd7_pw(c + 0x12, (r + 1) as u16);
                    break;
                }
                if (s as i16 as i32) < cqd7_w(cqd7_rodada(this, r) + 0x82) as i32 - 1 {
                    s += 1;
                } else {
                    s = 0;
                }
                if inicio as i16 == s as i16 {
                    // sem vaga livre: uma vaga sorteada duas rodadas adiante
                    if r >= m - 2 {
                        erro_em(0x9b55ac, 0x175);
                    }
                    let mut t = sorteio(cqd7_w(cqd7_rodada(this, r) + 0xea) as i32);
                    let ini = t;
                    loop {
                        let v = cqd7_confronto(this, r + 2, t as i16 as i32);
                        let a = if cqd7_w(v + 0xa) == -1 {
                            true
                        } else if cqd7_w(v + 0xc) == -1 {
                            false
                        } else {
                            if (t as i16 as i32) < cqd7_w(cqd7_rodada(this, r) + 0xea) as i32 - 1 {
                                t += 1;
                            } else {
                                t = 0;
                            }
                            if t as i16 == ini as i16 {
                                erro_em(0x9b55ac, 0x1a0);
                            }
                            continue;
                        };
                        cqd7_pw(v + if a { 0xa } else { 0xc }, p as u16);
                        cqd7_pw(v + if a { 0xe } else { 0x10 }, r as u16);
                        cqd7_pw(c + 6, t as u16);
                        cqd7_pw(c + 8, a as u16);
                        break;
                    }
                    cqd7_pw(c + 0x12, (r + 2) as u16);
                    break;
                }
            }
            k += 1;
        }
        if ordem != 0 {
            soltar(ordem);
        }
        r += 1;
    }
    if cqd7_w(cqd7_rodada(this, 0) + 0x1a) == 0 {
        cqd7_pb(this + 0xb1, 1);
        return 1;
    }
    // os times de cada rodada nos confrontos dela; `inicio` segue de um time para o outro, como na original
    let mut inicio: i32 = -1;
    let mut r = 0i32;
    while r < m {
        if cqd7_w(cqd7_rodada(this, r) + 0x1c) > 0 {
            let mut q = 0i32;
            loop {
                let rr = cqd7_rodada(this, r);
                let mut di: i32;
                if cqd7_w(rr + 0xd) as u16 & 0x200 != 0 {
                    di = 0;
                    if cqd7_w(rr + 0x1a) <= 0 {
                        aviso_em(0x9b55ac, 0x1c2);
                        return -1;
                    }
                } else {
                    di = sorteio(cqd7_w(rr + 0x1a) as i32);
                    inicio = di;
                    let rr = cqd7_rodada(this, r);
                    if cqd7_w(rr + 0xd) as u16 & 0x400 != 0 {
                        loop {
                            let c = cqd7_confronto(this, r, di as i16 as i32);
                            if cqd7_w(c) == -1 && cqd7_w(c + 2) == -1 {
                                inicio = di;
                                break;
                            }
                            di += 1;
                            if di as i16 >= cqd7_w(rr + 0x1a) {
                                di = 0;
                            }
                            if di as i16 == inicio as i16 {
                                break;
                            }
                        }
                    }
                }
                loop {
                    let moeda = sorteio(2);
                    let x = di as i16 as i32;
                    let c = cqd7_confronto(this, r, x);
                    let livre2 = || cqd7_w(c + 2) == -1 && cqd7_w(c + 0xc) == -1;
                    let primeiro_0 = moeda != 0; // cqd7: a moeda do lado na primeira rodada
                    let v0 = cqd7_w(c) as u16;
                    let livre0 = v0 == 0xffff && cqd7_w(c + 0xa) == -1;
                    let lado = if primeiro_0 {
                        if livre0 { Some(0usize) } else if livre2() { Some(2) } else { None }
                    } else if livre2() {
                        Some(2)
                    } else if livre0 {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(o) = lado {
                        crate::comparacao::ramo("cqd_7 time na primeira rodada");
                        cqd7_pw(c + o, (cqd7_w(cqd7_rodada(this, r) + 0x1e) as u16).wrapping_add(q as u16));
                        break;
                    }
                    // a vaga ocupada: o time da nação +0x24 no lado de fora vira mandante
                    if cqd7_d(this + 0x24) as u32 != u32::MAX && v0 != 0xffff {
                        let fora = cqd7_w(c + 2);
                        if fora != -1 {
                            let clube = cqd7_d(cqd7_d(this + 0xa7) + (fora as isize * 6) as usize);
                            if clube != 0 {
                                let nacao = cqd7_d(clube + 0x69);
                                if nacao != 0 && cqd7_d(nacao) == cqd7_d(this + 0x24) {
                                    cqd7_pw(c, fora as u16);
                                    cqd7_pw(c + 2, v0);
                                }
                            }
                        }
                    }
                    if x < cqd7_w(cqd7_rodada(this, r) + 0x1a) as i32 - 1 {
                        di += 1;
                    } else {
                        di = 0;
                    }
                    if inicio as i16 == di as i16 {
                        aviso_em(0x9b55ac, 0x222);
                        return -1;
                    }
                }
                q += 1;
                if q >= cqd7_w(cqd7_rodada(this, r) + 0x1c) as i32 {
                    break;
                }
            }
        }
        r += 1;
    }
    let nr = cqd7_w(this + 0x3c);
    if nr > 0 && cqd7_w(cqd7_rodada(this, nr as i32).wrapping_sub(0x61)) == 0x8c {
        let mut k = nr.wrapping_sub(3) as i32;
        let mut n: u8 = 5;
        while k >= 0 {
            cqd7_pb(cqd7_rodada(this, k) + 0x5b, n);
            n = n.wrapping_add(cqd7_b(cqd7_rodada(this, k) + 0x1a));
            k -= 1;
        }
    } else {
        let mut n = cqd7_b(cqd7_rodada(this, nr as i32).wrapping_sub(0x50)).wrapping_add(1);
        let mut k = nr.wrapping_sub(2) as i32;
        while k >= 0 {
            cqd7_pb(cqd7_rodada(this, k) + 0x5b, n);
            n = n.wrapping_add(cqd7_b(cqd7_rodada(this, k) + 0x1a));
            k -= 1;
        }
    }
    1
}

/// O comparador da árvore dos confrontos: só a original (ponytail: aloca os vetores dos confrontos e sorteia; não
/// roda duas vezes). Confere o que ela garante quando devolve 1 com clubes: cada confronto das rodadas mapeadas aponta
/// (+6 vaga, +8 lado, +0x12 rodada uma ou duas adiante) uma vaga que aponta de volta para ele (+0xa/+0xe mandante,
/// +0xc/+0x10 visitante).
unsafe extern "thiscall" fn comparar_copa_arvore_dos_confrontos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1558);
    let r = o(this);
    if r != 1 || cqd7_d(this + 0x45) == 0 {
        return r;
    }
    let nr = cqd7_w(this + 0x3c);
    let m = (if cqd7_w(cqd7_rodada(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut erro: Option<(i32, i32)> = None;
    'fora: for rd in 0..(m - 1).max(0) {
        for p in 0..cqd7_w(cqd7_rodada(this, rd) + 0x1a) as i32 {
            let c = cqd7_confronto(this, rd, p);
            let destino = cqd7_w(c + 0x12) as i32;
            let a = cqd7_w(c + 8) == 1;
            let ok = (destino == rd + 1 || destino == rd + 2) && {
                let v = cqd7_confronto(this, destino, cqd7_w(c + 6) as i32);
                cqd7_w(v + if a { 0xa } else { 0xc }) as i32 == p && cqd7_w(v + if a { 0xe } else { 0x10 }) as i32 == rd
            };
            if !ok {
                erro = Some((rd, p));
                break 'fora;
            }
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("copa_arvore_dos_confrontos: o confronto (rodada, índice) {:?} sem a vaga de volta", erro));
    r
}

/// A fase `k` da competição como a original a lê no meio das rotinas: −1 a própria, 0..+0x30 em [+0xc]; fora disso a
/// caixa da linha 0x36e de comp.cpp e 0.
unsafe fn cqd7_fase(comp: usize, k: i32) -> usize {
    if k < -1 || k > cqd7_d(comp + 0x30) as i32 {
        aviso_em(0x99b80c, 0x36e);
        return 0;
    }
    if k == -1 { comp } else { cqd7_d(cqd7_d(comp + 0xc) + k as usize * 4) }
}

/// { 0x48d2b0 } (thiscall, ret 4, comp.cpp): `Comp::get_first_stage_of_group(k)`, das telas das competições
/// (0x4947a0): da fase `k` para trás, enquanto as fases têm o mesmo grupo (0x4b01d0 do id +0x3a: push id; add esp, 4 ->
/// cdecl) e o índice é maior que −1, o índice da primeira delas (o próprio k sem grupo).
unsafe extern "thiscall" fn competicao_primeira_fase_do_grupo(this: usize, k: i32) -> i32 {
    let grupo: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x4b01d0usize);
    let mut e = k;
    let g = grupo(cqd7_w(cqd7_fase(this, e) + 0x3a) as i32);
    if g > -1 && e > -1 {
        loop {
            let f = cqd7_fase(this, e - 1);
            if grupo(cqd7_w(f + 0x3a) as i32) != g {
                break;
            }
            e -= 1;
            if e <= -1 {
                break;
            }
        }
    }
    e
}

/// O comparador da primeira fase do grupo: com o índice válido (−1..+0x30), as duas (só leem) e o mesmo índice; fora
/// dele só a original (a caixa).
unsafe extern "thiscall" fn comparar_competicao_primeira_fase_do_grupo(this: usize, k: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> i32>(1559);
    if k < -1 || k > cqd7_d(this + 0x30) as i32 {
        return o(this, k);
    }
    let r = o(this, k);
    let n = competicao_primeira_fase_do_grupo(this, k);
    PLACAR.registrar(r == n, || format!("competicao_primeira_fase_do_grupo({}): original={} rust={}", k, r, n));
    r
}

/// { 0x48cec0 } (thiscall, ret 0, comp.cpp): `Comp::get_current_stage()`, das telas das competições (0x48f8e0): a
/// primeira fase (−1 a própria, até +0x30) que não acabou (método 0x10 com (0, 1) = 0); se é uma copa (+0x43 = 1) que
/// não começou (+0xab = 0), volta ao começo do grupo dela pela fase anterior (0x4b01d0 e 0x48d200: push k; mov ecx,
/// this -> thiscall, ret 4). Todas acabadas: o começo do grupo da última. Fase inválida no fim: a caixa da linha 0x36e
/// e a própria competição; fase nula: a própria.
unsafe extern "thiscall" fn competicao_fase_atual(this: usize) -> usize {
    let grupo: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x4b01d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x48d200usize);
    let voltar = |s0: i32| -> i32 {
        let mut s = s0;
        let g = grupo(cqd7_w(cqd7_fase(this, s) + 0x3a) as i32);
        if g > -1 && s > -1 {
            loop {
                let f = fase(this, s - 1);
                if grupo(cqd7_w(f + 0x3a) as i32) != g {
                    break;
                }
                s -= 1;
                if s <= -1 {
                    break;
                }
            }
        }
        s
    };
    let fim = |s: i32| -> usize {
        if s < -1 || s > cqd7_d(this + 0x30) as i32 {
            aviso_em(0x99b80c, 0x36e);
            return this;
        }
        let f = if s == -1 { this } else { cqd7_d(cqd7_d(this + 0xc) + s as usize * 4) };
        if f != 0 { f } else { this }
    };
    let mut e: i32 = -1;
    if cqd7_d(this + 0x30) as i32 >= -1 {
        loop {
            let f = cqd7_fase(this, e);
            let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqd7_d(cqd7_d(f) + 0x10));
            if acabou(f, 0, 1) as u8 == 0 {
                if f == 0 {
                    break;
                }
                if cqd7_b(f + 0x43) == 1 && cqd7_b(f + 0xab) == 0 && e > -1 {
                    return fim(voltar(e - 1));
                }
                return f;
            }
            e += 1;
            if e > cqd7_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    fim(voltar(cqd7_d(this + 0x30) as i32))
}

/// O comparador da fase atual: só a original (ponytail: o método 0x10 das fases pode refazer a tabela; não roda duas
/// vezes); a cópia se prova no modo Rust pelas telas.
unsafe extern "thiscall" fn comparar_competicao_fase_atual(this: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(1560)(this)
}

// ---------- 267a leva: o treino do jogador posto em dia: o progresso das semanas, o programa do jogador e o nível da categoria (training_manager.cpp) ----------

unsafe fn cqd8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqd8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqd8_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqd8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
unsafe fn cqd8_entrada(this: usize, p: usize) -> usize {
    cqd8_d(this + 0x651d).wrapping_add((cqd8_i(cqd8_d(p + 0x61)) as isize * 6) as usize)
}

/// O contrato da pessoa (CONTRACT_MANAGER 0x4cf5a0, thiscall no gerente [0xae1050]), guardado em [0xae1080] como a
/// original faz a cada consulta.
unsafe fn cqd8_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqd8_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (+4 o id; registros de 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
unsafe fn cqd8_clube(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cqd8_d(0xae23b4).wrapping_add((cqd8_i(c + 4) as isize * 0x245) as usize)
    }
}

/// O técnico humano é um dos 16 últimos ids de pessoa ([0xae2364] − 16 para cima).
unsafe fn cqd8_humano(tec: usize) -> bool {
    cqd8_i(tec) >= cqd8_i(0xae2364).wrapping_sub(0x10)
}

/// O programa `vaga` (0..9; fora disso a 1) do técnico humano `tec`: this + (vaga + 7 + 10 × ([0xae2364] − id − 1)) ×
/// 0x9b. Fora dos 16 humanos, a caixa "Error" (linha 0x68f) e 0.
unsafe fn cqd8_programa_do_humano(this: usize, tec: usize, vaga: i8) -> usize {
    let k = cqd8_i(0xae2364).wrapping_sub(cqd8_i(tec)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        aviso_em(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&vaga) { vaga as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// O meio byte com sinal da categoria `k` na entrada do treino: 0 o baixo de +3, 1 o alto de +3, 2 o baixo de +4, 3 o
/// alto de +4, 4 o baixo de +5.
unsafe fn cqd8_meio_byte(e: usize, k: usize) -> i8 {
    let b = *((e + 3 + k / 2) as *const u8);
    if k % 2 == 0 {
        ((b << 4) as i8) >> 4
    } else {
        (b as i8) >> 4
    }
}

/// { 0x8b1750 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::update_player_training(pessoa, pular)`,
/// pela iniciação do jogador na partida (0x6eaea0, match_man.cpp, com 0: todo jogador de toda partida), pela lesão nova
/// (0x62d250) e pelas telas. Pessoa nula (linha 0x18a), sem a ficha (0x195) ou sem programa (0x8b39f0 nulo, 0x1a3): a
/// caixa "Error"; id a partir de [0xae2364]: nada. Jogador de clube de técnico humano sem a marca 0x20 em +5 da entrada
/// do treino: a marca, a vaga do programa (o de goleiro, tipo 6, para goleiro de 15 para cima; senão o geral, tipo 1;
/// senão 1) e a carga dele (0x8b4f60) na ficha de forma (0x62c660). Semanas = semana do treino (+0x688d) − a da entrada
/// (+1); nenhuma e sem `pular`: nada. Com a marca e o clube já sem técnico humano: tira a marca, carga 50, vaga 6 ou 1 e
/// os treinos de posição param (0x8b4450 com 0). Até 30 semanas: põe a entrada em dia; cada categoria (0..4) tem o
/// peso nível × 2 + 60 + (fator do treinador − 100) / 5 (0..200) e, por semana, sorteios do progresso: sobe 1 com peso
/// acima de 100, sorteio(210 − peso) < 20, valor abaixo de (+0x58) / 3 e sorteio(|valor| × 2 + 14) < 10; senão desce 1
/// com peso abaixo de 110, sorteio(peso + 60) < 20, valor acima de −7 e de (+0x5b) / 3 − 6 e o mesmo último sorteio
/// (goleiro de 19 para cima: só com sorteio(4), ou sorteio(2) na categoria 4, nulo). Os treinos de posição (0x8b4b20,
/// itens de 8 bytes: +4 a máscara, +6 a nota, +7 a taxa) andam por semana (ou uma vez, com `pular` na máscara): taxa
/// positiva, sorteio(taxa) < 5 dá nota + 2 (até 20) e o jogador pode reclamar (contrato do clube 0x4cf5f0 sem 0x8000
/// em +0x45: pela taxa acima de 50 ou pela aptidão 0x8b4770 abaixo de 12, com +0x5b abaixo de 15, sorteio(+0x5b) < 10
/// e sem lesão 0x62c400: a marca 0x8000 (0x867700), −5 em +0x44 (−100..100) e 0x5885c0); taxa negativa, sorteio(−taxa)
/// < 5 dá nota − 2 e, abaixo de 1, o item acaba (taxa 0, nota 1). Aplica o progresso (0x8b2be0) e, com itens acabados,
/// tira-os da lista (o último no lugar) e reordena (qsort 0x945333 com 0x8b4e00).
unsafe extern "thiscall" fn atualizar_o_treino_do_jogador(this: usize, p: usize, pular: u32) {
    if p == 0 {
        aviso_em(0xa8ccd0, 0x18a);
        return;
    }
    if cqd8_i(p) >= cqd8_i(0xae2364) {
        return;
    }
    if cqd8_d(p + 0x61) == 0 {
        aviso_em(0xa8ccd0, 0x195);
        return;
    }
    let mut d = [0i8; 8]; // o progresso das cinco categorias (a original zera um dword e um byte)
    let e = cqd8_entrada(this, p);
    let programa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b39f0usize);
    let prog = programa(this, p);
    if prog == 0 {
        aviso_em(0xa8ccd0, 0x1a3);
        return;
    }
    let gravar_carga: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62c660usize);
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let marca = (e + 5) as *mut u8;
    if *marca & 0x20 == 0 {
        let c = cqd8_contrato(p);
        if c != 0 && cqd8_clube(c) != 0 && cqd8_d(cqd8_clube(cqd8_contrato(p)) + 0xcf) != 0
            && cqd8_humano(cqd8_d(cqd8_clube(cqd8_contrato(p)) + 0xcf)) {
            let tec = cqd8_d(cqd8_clube(cqd8_contrato(p)) + 0xcf);
            *marca |= 0x20;
            let (mut goleiro, mut geral) = (-1i8, -1i8);
            for k in 0..10i8 {
                if *((cqd8_programa_do_humano(this, tec, k) + 0x15) as *const u8) == 6 {
                    goleiro = k;
                } else if *((cqd8_programa_do_humano(this, tec, k) + 0x15) as *const u8) == 1 {
                    geral = k;
                }
            }
            let vaga = if cqd8_b(cqd8_d(p + 0x61) + 0xf) >= 0xf && goleiro != -1 {
                goleiro
            } else if geral != -1 {
                geral
            } else {
                1
            };
            *(e as *mut i8) = vaga;
            let carga: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x8b4f60usize);
            let v = carga(cqd8_programa_do_humano(this, tec, vaga));
            crate::comparacao::ramo("cqd_8 programa do humano");
            gravar_carga(cqd8_d(0xb63d28), p, v as u32);
        }
    }
    let mut semanas = cqd8_w(this + 0x688d).wrapping_sub(cqd8_w(e + 1));
    let pular = pular as u16;
    if semanas == 0 && pular == 0 {
        return;
    }
    if *marca & 0x20 != 0 {
        let c = cqd8_contrato(p);
        let ainda = c != 0 && cqd8_clube(c) != 0 && cqd8_d(cqd8_clube(cqd8_contrato(p)) + 0xcf) != 0
            && cqd8_humano(cqd8_d(cqd8_clube(cqd8_contrato(p)) + 0xcf));
        if !ainda {
            crate::comparacao::ramo("cqd_8 saiu do clube humano");
            *marca &= 0xdf;
            gravar_carga(cqd8_d(0xb63d28), p, 0x32);
            let f = cqd8_d(p + 0x61);
            *(cqd8_entrada(this, p) as *mut u8) = if cqd8_b(f + 0xf) >= 0xf { 6 } else { 1 };
            let mut q = 0usize;
            let n = achar(this, p, &mut q);
            let parar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b4450usize);
            let mut k = 0i8;
            if n > 0 {
                loop {
                    parar(this, q.wrapping_add((k as isize * 8) as usize), 0);
                    k = k.wrapping_add(1);
                    if k as i32 >= n {
                        break;
                    }
                }
            }
        }
    }
    if semanas > 0x1e {
        semanas = 0x1e;
    }
    core::ptr::write_unaligned((e + 1) as *mut u16, cqd8_w(this + 0x688d));
    let mut q = 0usize;
    let npos = achar(this, p, &mut q);
    if *marca & 0x10 != 0 && npos == 0 {
        *marca &= 0xef;
    }
    let nivel: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x8b4f50usize);
    let fator: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8b41d0usize);
    for k in 0..5usize {
        // ponytail: a original empurra a categoria num dword cujos bytes de cima sobram da pilha; a 0x8b4f50 e a
        // 0x8b41d0 leem o byte
        let lv = nivel(prog, k as u32) as u8 as i8;
        let base = (lv as i16).wrapping_mul(2).wrapping_add(0x3c); // cqd8: o peso do nível
        let clube = cqd8_clube(cqd8_contrato(p));
        let fa = fator(this, clube, k as u32) as u16 as i16 as i32;
        let x = base.wrapping_add(((fa - 100) / 5) as i16);
        let peso: i16 = if x < 0 { 0 } else if x > 200 { 200 } else { x };
        let piso = ((cqd8_b(p + 0x5b) as i32 / 3) as i8).wrapping_sub(6);
        let teto = (cqd8_b(p + 0x58) as i32 / 3) as i8;
        let meio = cqd8_meio_byte(e, k);
        for _ in 0..semanas {
            let v = meio.wrapping_add(d[k]);
            if cqd8_b(cqd8_d(p + 0x61) + 0xf) > 0x12 {
                let r = if k == 4 { cqd8_sortear(2) } else { cqd8_sortear(4) };
                if r != 0 {
                    continue; // goleiro: a categoria quase não anda
                }
            }
            if peso > 0x64 {
                if cqd8_sortear(0xd2 - peso as i32) < 0x14 && v < teto { // cqd8: o sorteio da subida
                    if cqd8_sortear((v as i32).abs() * 2 + 0xe) < 10 {
                        crate::comparacao::ramo("cqd_8 progresso sobe");
                        d[k] = d[k].wrapping_add(1); // cqd8: o progresso sobe
                    }
                    continue;
                }
            }
            if peso >= 0x6e {
                continue;
            }
            if cqd8_sortear(peso as i32 + 0x3c) >= 0x14 || v <= -7 || v <= piso {
                continue;
            }
            if cqd8_sortear((v as i32).abs() * 2 + 0xe) < 10 {
                crate::comparacao::ramo("cqd_8 progresso desce");
                d[k] = d[k].wrapping_sub(1); // cqd8: o progresso desce
            }
        }
    }
    let mut voltas = semanas;
    let mut contrato_do_clube = 0usize;
    if pular != 0 || voltas != 0 {
        if pular != 0 {
            voltas = 1;
        }
        let ficha: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4cf5f0usize);
        contrato_do_clube = ficha(cqd8_d(0xae1050), p, cqd8_d(p + 0x39) as u32);
    }
    let mut acabou = false;
    if npos > 0 {
        let aptidao: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8b4770usize);
        let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
        let reclamar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
        let infeliz: unsafe extern "C" fn(usize) = core::mem::transmute(0x5885c0usize);
        let mut k = 0i8;
        let mut it = q + 7; // a taxa do item (+7); a nota em +6, a máscara em +4
        loop {
            for _ in 0..voltas {
                let taxa = cqd8_b(it);
                if (pular != 0 && cqd8_w(it - 3) == pular) || taxa > 0 {
                    if cqd8_sortear(cqd8_b(it) as i32) < 5 {
                        let n = cqd8_b(it - 1).wrapping_add(2);
                        *((it - 1) as *mut i8) = if n > 0x14 { 0x14 } else { n };
                    }
                    let c = contrato_do_clube;
                    if c != 0 && cqd8_d(c + 0x45) & 0x8000 == 0 {
                        // ponytail: a original empurra a máscara no ecx com os 16 bits de cima que sobram; a 0x8b4770
                        // lê a palavra
                        let apt = aptidao(this, p, cqd8_w(it - 3) as u32) as u8 as i8;
                        let t = cqd8_b(it);
                        let quer = (t > 0x32 && cqd8_sortear(100 - t as i32) < 10) || (apt < 0xc && cqd8_sortear(apt as i32) < 2);
                        let a = cqd8_b(p + 0x5b);
                        if quer && a < 0xf && cqd8_sortear(a as i32) < 10 && cqd8_d(0xb63d28) != 0
                            && estado(cqd8_d(0xb63d28), p) == 0 {
                            crate::comparacao::ramo("cqd_8 reclama do treino de posicao");
                            reclamar(c, 0x8000, 0);
                            let m = (c + 0x44) as *mut i8;
                            let v = *m as i32 - 5;
                            *m = if v > 100 { 100 } else if v < -100 { -100 } else { (*m).wrapping_sub(5) };
                            infeliz(p);
                            contrato_do_clube = 0;
                        }
                    }
                } else if taxa < 0 && cqd8_sortear((taxa as i32).abs()) < 5 {
                    let n = cqd8_b(it - 1).wrapping_sub(2);
                    *((it - 1) as *mut i8) = n;
                    if n < 1 {
                        *(it as *mut i8) = 0;
                        *((it - 1) as *mut i8) = 1;
                        acabou = true;
                    }
                }
            }
            k = k.wrapping_add(1);
            it += 8;
            if k as i32 >= npos {
                break;
            }
        }
    }
    let aplicar: unsafe extern "thiscall" fn(usize, usize, *const i8) = core::mem::transmute(0x8b2be0usize);
    aplicar(this, p, d.as_ptr());
    if acabou {
        crate::comparacao::ramo("cqd_8 treino de posicao acabou");
        let n0 = cqd8_i(this + 0x6525);
        let mut i = 0i32;
        if n0 > 0 {
            loop {
                let base = cqd8_d(this + 0x6521);
                let n = cqd8_i(this + 0x6525);
                let item = base.wrapping_add((i as isize * 8) as usize);
                if *((item + 7) as *const u8) == 0 {
                    let ultimo = base.wrapping_add((n as isize * 8 - 8) as usize);
                    core::ptr::write_unaligned(item as *mut u32, cqd8_d(ultimo) as u32);
                    core::ptr::write_unaligned((item + 4) as *mut u32, cqd8_d(ultimo + 4) as u32);
                    core::ptr::write_unaligned((this + 0x6525) as *mut i32, cqd8_i(this + 0x6525) - 1);
                }
                i += 1;
                if i >= cqd8_i(this + 0x6525) {
                    break;
                }
            }
        }
        if n0 != cqd8_i(this + 0x6525) {
            let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
            qsort(cqd8_d(this + 0x6521), cqd8_d(this + 0x6525) as u32, 8, 0x8b4e00);
        }
    }
}

/// Comparador: lado a lado no estado do sorteador, em [0xae1080], na entrada do treino (6 bytes), na ficha (+0xf..+0x41,
/// o que a 0x8b2be0 grava) e na carga da ficha de forma (31 bytes por jogador em [[0xb63d28]], +0x1e), com os sorteios
/// da cópia fora da contagem. Só a original: os casos de erro e o jogador com treino de posição (0x8b4b20 antes de
/// rodar): ali a 0x8b4450, a marca do contrato (0x867700) e a 0x5885c0 mexem fora dessas regiões.
unsafe extern "thiscall" fn comparar_atualizar_o_treino_do_jogador(this: usize, p: usize, pular: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1561);
    if p == 0 || cqd8_i(p) >= cqd8_i(0xae2364) || cqd8_d(p + 0x61) == 0 {
        return o(this, p, pular);
    }
    let programa: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b39f0usize);
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(0x8b4b20usize);
    let mut q = 0usize;
    if programa(this, p) == 0 || achar(this, p, &mut q) > 0 {
        return o(this, p, pular);
    }
    let f = cqd8_d(p + 0x61);
    let mut reg = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.extend_from_slice(&[(0xae1080, 4), (cqd8_entrada(this, p), 6), (f + 0xf, 0x33)]);
    let les = cqd8_d(0xb63d28);
    if les != 0 {
        reg.push((cqd8_d(les).wrapping_add((cqd8_i(p) as isize * 31 + 0x1e) as usize), 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, pular),
        || crate::aleatorio::sem_contar(|| atualizar_o_treino_do_jogador(this, p, pular)));
    PLACAR.registrar(igual, || format!("atualizar_o_treino_do_jogador(pessoa {:x}, pular {:x}) {:x?} x {:x?}", p, pular as u16, mo.0, mr.0));
}

/// { 0x8b39f0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_player_schedule(pessoa)`, pelo diário
/// das pessoas (game_daily_staff_update 0x5ce2d0), pela 0x5d6aa0 e pela update_player_training (0x8b1750). Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x6ad) e 0. No clube de técnico humano (contrato 0x4cf5a0, guardado em
/// [0xae1080]; técnico +0xcf entre os 16 últimos ids): lesionado (0x62c400 = 2) o programa 1 do humano, this + 0x4d8 +
/// 0x60e × ([0xae2364] − id − 1); senão o da vaga da entrada do treino (+0, fora de 0..9 a 1); fora dos 16, a caixa
/// (linha 0x68f) e 0. Senão: lesionado, o programa 0 (this); senão this + 0x9b × vaga (com sinal).
unsafe extern "thiscall" fn programa_de_treino_do_jogador(this: usize, p: usize) -> usize {
    if p == 0 || cqd8_d(p + 0x61) == 0 {
        aviso_em(0xa8ccd0, 0x6ad);
        return 0;
    }
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let c = cqd8_contrato(p);
    if c != 0 && cqd8_clube(c) != 0 {
        let tec = cqd8_d(cqd8_clube(cqd8_contrato(p)) + 0xcf);
        if tec != 0 && cqd8_humano(tec) {
            if estado(cqd8_d(0xb63d28), p) == 2 {
                let k = cqd8_i(0xae2364).wrapping_sub(cqd8_i(tec)).wrapping_sub(1);
                if !(0..0x10).contains(&k) {
                    aviso_em(0xa8ccd0, 0x68f);
                    return 0;
                }
                return this.wrapping_add(0x4d8 + 0x60e * k as usize);
            }
            return cqd8_programa_do_humano(this, tec, *(cqd8_entrada(this, p) as *const i8));
        }
    }
    if estado(cqd8_d(0xb63d28), p) == 2 {
        return this;
    }
    this.wrapping_add((*(cqd8_entrada(this, p) as *const i8) as isize * 0x9b) as usize)
}

/// Comparador: a devolução e [0xae1080] lado a lado; sem pessoa ou sem ficha (a caixa), só a original.
unsafe extern "thiscall" fn comparar_programa_de_treino_do_jogador(this: usize, p: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1562);
    if p == 0 || cqd8_d(p + 0x61) == 0 {
        return o(this, p);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, p), || programa_de_treino_do_jogador(this, p));
    PLACAR.registrar(igual && r == n, || format!("programa_de_treino_do_jogador(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo.0, mr.0));
    r
}

/// { 0x8b4f50 } (thiscall, ret 4, training_manager.cpp): `TRAINING_SCHEDULE::get_category_level(categoria)`: o nível da
/// categoria (+4 + categoria, com sinal) do programa, no al; os bytes de cima do eax ficam com a categoria estendida
/// (movsx), como na original. Pela update_player_training (0x8b1750, as cinco categorias de cada atualização).
unsafe extern "thiscall" fn nivel_da_categoria_do_programa(this: usize, categoria: u32) -> u32 {
    let k = categoria as u8 as i8 as i32;
    let v = *((this as isize + k as isize + 4) as *const u8);
    (k as u32 & 0xffff_ff00) | v as u32
}

/// Comparador: a devolução inteira (só lê).
unsafe extern "thiscall" fn comparar_nivel_da_categoria_do_programa(this: usize, categoria: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1563)(this, categoria);
    let n = nivel_da_categoria_do_programa(this, categoria);
    PLACAR.registrar(r == n, || format!("nivel_da_categoria_do_programa({:x}, {}) original={:x} rust={:x}", this, categoria as u8 as i8, r, n));
    r
}

// ---------- 267a leva: o progresso do treino guardado: tirar e devolver o progresso, o quanto o treino soma a um atributo e a aptidão para uma posição (training_manager.cpp) ----------

unsafe fn cqd9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd9_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// A entrada de 6 bytes do treino do jogador: [this+0x651d] + 6 × id da ficha (+0x61).
unsafe fn cqd9_entrada(this: usize, p: usize) -> usize {
    cqd9_d(this + 0x651d).wrapping_add((*(cqd9_d(p + 0x61) as *const i32) as isize * 6) as usize)
}

/// O meio byte com sinal da categoria `k` na entrada do treino: 0 o baixo de +3, 1 o alto de +3, 2 o baixo de +4, 3 o
/// alto de +4, 4 o baixo de +5.
unsafe fn cqd9_meio(e: usize, k: usize) -> i8 {
    let b = *((e + 3 + k / 2) as *const u8);
    if k % 2 == 0 {
        ((b << 4) as i8) >> 4
    } else {
        (b as i8) >> 4
    }
}

unsafe fn cqd9_aplicar(this: usize, p: usize, d: &[i8; 8]) {
    let f: unsafe extern "thiscall" fn(usize, usize, *const i8) = core::mem::transmute(0x8b2be0usize);
    f(this, p, d.as_ptr());
}

/// { 0x8b3640 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::remove_or_restore_training_progress(pessoa,
/// devolver)`, pelas normalizações dos atributos do jogador (0x538bc0 e 0x538ec0: no envelhecimento do diário das
/// pessoas 0x5ce2d0, no regen jovem 0x7abac0 e na carga), que tiram o progresso antes e o devolvem depois. Pessoa nula
/// ou sem a ficha: a caixa "Error" (linha 0x621). Tirar (`devolver` = 0): guarda os cinco meios bytes do progresso da
/// entrada do treino em 0xde1f3c..0xde1f40 (categoria 3, 2, 4, 0, 1) e aplica o progresso negado (0x8b2be0); devolver:
/// aplica o guardado.
unsafe extern "thiscall" fn tirar_ou_devolver_o_progresso(this: usize, p: usize, devolver: u32) {
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        aviso_em(0xa8ccd0, 0x621);
        return;
    }
    let e = cqd9_entrada(this, p);
    let mut d = [0i8; 8];
    let g = |a: usize| *(a as *const i8);
    if devolver != 0 {
        crate::comparacao::ramo("cqd_9 devolver");
        d[0] = g(0xde1f3f); // cqd9: devolve a categoria 0
        d[1] = g(0xde1f40);
        d[2] = g(0xde1f3d);
        d[3] = g(0xde1f3c);
        d[4] = g(0xde1f3e); // cqd9: devolve a categoria 4
    } else {
        crate::comparacao::ramo("cqd_9 tirar");
        *(0xde1f3f as *mut i8) = cqd9_meio(e, 0);
        *(0xde1f40 as *mut i8) = cqd9_meio(e, 1); // cqd9: guarda a categoria 1
        *(0xde1f3d as *mut i8) = cqd9_meio(e, 2);
        *(0xde1f3c as *mut i8) = cqd9_meio(e, 3);
        *(0xde1f3e as *mut i8) = cqd9_meio(e, 4);
        for k in 0..5 {
            d[k] = cqd9_meio(e, k).wrapping_neg();
        }
    }
    cqd9_aplicar(this, p, &d);
}

/// Comparador: lado a lado na ficha (+0xf..+0x41), na entrada do treino (6 bytes) e no guardado (0xde1f3c, 5 bytes);
/// sem pessoa ou sem ficha (a caixa), só a original.
unsafe extern "thiscall" fn comparar_tirar_ou_devolver_o_progresso(this: usize, p: usize, devolver: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1564);
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        return o(this, p, devolver);
    }
    let f = cqd9_d(p + 0x61);
    let reg = [(f + 0xf, 0x33), (cqd9_entrada(this, p), 6), (0xde1f3c, 5)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, p, devolver), || tirar_ou_devolver_o_progresso(this, p, devolver));
    PLACAR.registrar(igual, || format!("tirar_ou_devolver_o_progresso(pessoa {:x}, devolver {}) {:x?} x {:x?}", p, devolver, mo.0, mr.0));
}

/// { 0x8b37c0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_training_attribute_change(pessoa,
/// atributo)`, pela tela do treino (0x8b8184): o atributo (0x5431e0) menos o mesmo atributo sem o progresso do treino
/// (aplica o progresso negado, lê e reaplica o progresso), no al. Pessoa nula ou sem a ficha: a caixa "Error" (linha
/// 0x657) e 0.
unsafe extern "thiscall" fn mudanca_do_atributo_pelo_treino(this: usize, p: usize, atributo: u32) -> u8 {
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        aviso_em(0xa8ccd0, 0x657);
        return 0;
    }
    let e = cqd9_entrada(this, p);
    let ler: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x5431e0usize);
    let antes = ler(p, atributo);
    let mut d = [0i8; 8];
    for k in 0..5 {
        d[k] = cqd9_meio(e, k).wrapping_neg();
    }
    cqd9_aplicar(this, p, &d);
    let sem = ler(p, atributo);
    let mut v = [0i8; 8];
    for k in 0..5 {
        v[k] = d[k].wrapping_neg();
    }
    cqd9_aplicar(this, p, &v);
    antes.wrapping_sub(sem)
}

/// Comparador: lado a lado na ficha e na entrada do treino, com a devolução; sem pessoa ou sem ficha, só a original.
unsafe extern "thiscall" fn comparar_mudanca_do_atributo_pelo_treino(this: usize, p: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(1565);
    if p == 0 || cqd9_d(p + 0x61) == 0 {
        return o(this, p, atributo);
    }
    let f = cqd9_d(p + 0x61);
    let reg = [(f + 0xf, 0x33), (cqd9_entrada(this, p), 6)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, p, atributo), || mudanca_do_atributo_pelo_treino(this, p, atributo));
    PLACAR.registrar(igual && r == n, || format!("mudanca_do_atributo_pelo_treino(pessoa {:x}, atributo {}) original={} rust={} {:x?} x {:x?}", p, atributo as u8 as i8, r as i8, n as i8, mo.0, mr.0));
    r
}

/// As máscaras de posição que a 0x8b4770 aceita (as outras dão a caixa).
fn cqd9_mascara_valida(m: i32) -> bool {
    matches!(m, 1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x200 | 0x800)
}

/// { 0x8b4770 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_position_aptitude(pessoa, mascara)`
/// (this não usado), pela update_player_training (0x8b1750, a reclamação do treino de posição) e pela
/// set_position_training_rate (0x8b4450): a média da adaptabilidade (+0x56 da pessoa) com os atributos que a posição
/// pede (notas de 1 a 20 pela 0x544380 sobre a ficha +0x61): goleiro (1) +0x39, +0x2a e +0x35, por 4; defesa (2, 4)
/// +0x3e, +0x39 e +0x32, por 4; meio e frente (8, 0x10, 0x20) +0x37, +0x32, +0x40 e +0x3f, por 5; 0x40 +0x33, +0x27,
/// +0x40 e +0x36, por 5; 0x80 +0x30, por 2; 0x200 só a adaptabilidade (no al, os bytes de cima com a máscara); 0x800
/// +0x3b, por 2 (divisões truncadas). Outra máscara: a caixa "Error" (linha 0x945) e 0.
unsafe extern "thiscall" fn aptidao_para_a_posicao(this: usize, p: usize, mascara: u32) -> u32 {
    let _ = this;
    let m = mascara as u16 as i16 as i32;
    if !cqd9_mascara_valida(m) {
        aviso_em(0xa8ccd0, 0x945);
        return 0;
    }
    let f = cqd9_d(p + 0x61);
    let nota_de: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x544380usize);
    // ponytail: a original empurra o byte do atributo num registrador com os bytes de cima que sobram; a 0x544380 lê
    // o byte
    let nota = |o: usize| nota_de(f, *((f + o) as *const u8) as u32) as u8 as i8 as i32;
    let b = |o: usize| cqd9_b(f + o) as i32;
    let adapt = cqd9_b(p + 0x56) as i32;
    let r = match m {
        1 => {
            let x = nota(0x39);
            let y = nota(0x2a);
            (b(0x35) + x + y + adapt) / 4
        }
        2 | 4 => {
            let x = nota(0x3e);
            let y = nota(0x39);
            let z = nota(0x32);
            (x + y + z + adapt) / 4
        }
        8 | 0x10 | 0x20 => {
            let x = nota(0x37);
            let y = nota(0x32);
            (x + y + adapt + b(0x40) + b(0x3f)) / 5
        }
        0x40 => {
            let x = nota(0x33);
            let y = nota(0x27);
            (x + y + adapt + b(0x40) + b(0x36)) / 5
        }
        0x80 => (adapt + b(0x30)) / 2,
        0x200 => return (m as u32 & 0xffff_ff00) | *((p + 0x56) as *const u8) as u32,
        _ => (adapt + b(0x3b)) / 2,
    };
    r as u32
}

/// Comparador: a devolução inteira (só lê); máscara fora (a caixa), só a original.
unsafe extern "thiscall" fn comparar_aptidao_para_a_posicao(this: usize, p: usize, mascara: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1566);
    if !cqd9_mascara_valida(mascara as u16 as i16 as i32) {
        return o(this, p, mascara);
    }
    let r = o(this, p, mascara);
    let n = aptidao_para_a_posicao(this, p, mascara);
    PLACAR.registrar(r == n, || format!("aptidao_para_a_posicao(pessoa {:x}, mascara {:x}) original={:x} rust={:x}", p, mascara as u16, r, n));
    r
}

// ---------- 267a leva: as aposentadorias do dia: as anunciadas e as que vencem, a notícia da aposentadoria e a do jovem promovido (player_regen.cpp) ----------

unsafe fn cqd10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqd10_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqd10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O byte das marcas da aposentadoria da pessoa `id` (registros de 79 bytes em [0xae2d0c], +0xb).
unsafe fn cqd10_marca(id: i32) -> *mut u8 {
    cqd10_d(0xae2d0c).wrapping_add((id as isize * 0x4f + 0xb) as usize) as *mut u8
}

/// O contrato da pessoa (0x4cf5a0 no gerente [0xae1050]), sem guardar em [0xae1080] (a 0x7a59e0 não guarda).
unsafe fn cqd10_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cqd10_d(0xae1050), p)
}

/// PLAYER_REGEN::remove_retirement (0x7a8690): tira a aposentadoria pendente (as seguintes sobem um lugar).
unsafe fn cqd10_tirar(this: usize, item: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7a8690usize);
    f(this, item);
}

/// Uma rotina thiscall(gerente [g], pessoa).
unsafe fn cqd10_com(rotina: usize, g: usize, p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(rotina);
    f(cqd10_d(g), p);
}

/// O atributo do técnico que vira o jogador aposentado: min(max(P, X) × k, teto), truncado (as contas x87 da original
/// em f64; P e X inteiros).
fn cqd10_teto(pv: f64, x: f64, k: f64, teto: f64) -> u16 {
    let m = if pv > x { pv } else { x };
    let v = m * k;
    ftol(if v < teto { v } else { teto }) as u16
}

/// Os três atributos do técnico (+0xa, +8, +0xc do registro de não jogador `nj`) pelo jogador `p` (ficha `f`): a
/// reputação (+0x22) ao quadrado (pow 0x945640/0x945ff0) contra +0xb, +9 e +0xd da ficha (o último com o quadrado ×
/// 0,75), × `k` e até os tetos.
unsafe fn cqd10_atributos_do_tecnico(p: usize, f: usize, nj: usize, k: f64, tetos: [f64; 3]) {
    let r = *((p + 0x22) as *const u8) as f64;
    let q = pow_do_jogo(r, 2.0);
    core::ptr::write_unaligned((nj + 0xa) as *mut u16, cqd10_teto(q, cqd10_w(f + 0xb) as f64, k, tetos[0]));
    let q = pow_do_jogo(r, 2.0);
    core::ptr::write_unaligned((nj + 8) as *mut u16, cqd10_teto(q, cqd10_w(f + 9) as f64, k, tetos[1]));
    let q = pow_do_jogo(r, 2.0) * f64_em(0x969040);
    core::ptr::write_unaligned((nj + 0xc) as *mut u16, cqd10_teto(q, cqd10_w(f + 0xd) as f64, k, tetos[2]));
}

/// O jogador aposentado sem registro de não jogador (+0x69) pode virar técnico: sorteio(+0x5b) acima de sorteio(100);
/// sorteio(ftol(+0xb da ficha + reputação³)) acima de sorteio(50000), ou (+0x8e da nação +0x1a) × reputação acima de
/// sorteio(200000) + 200000; com mais de 2 fichas reservadas (this +0x20): a primeira (this +0x1c; registros de 0x44
/// bytes em [0xae23c0]) vira o +0x69 e sai da reserva (0x7a82b0); goleiro (+0xf da ficha 20): +0x10/+0x11 trocados
/// quando +0x10 é maior e, com sorteio(4) nulo, +0x11 = sorteio(50) + 50; a normalização (0x539490) e os atributos do
/// técnico (0x9/7500 com +0xb da ficha acima de 7500; senão, com sorteio(+0x57) ≠ 0, 0,75/6000/3750; senão
/// 0,5/4500/1).
unsafe fn cqd10_vira_tecnico(this: usize, p: usize) {
    let r = *((p + 0x22) as *const u8);
    let ra = cqd10_sortear(cqd10_b(p + 0x5b) as i32);
    if ra <= cqd10_sortear(100) {
        return;
    }
    let f = cqd10_d(p + 0x61);
    let t = ftol(cqd10_w(f + 0xb) as f64 + pow_do_jogo(r as f64, 3.0));
    let r1 = cqd10_sortear(t);
    let passa = r1 > cqd10_sortear(50000) || { // cqd10: o sorteio de virar técnico
        let r3 = cqd10_sortear(0x30d40).wrapping_add(0x30d40);
        let v = (cqd10_w(cqd10_d(p + 0x1a) + 0x8e) as i32).wrapping_mul(*((p + 0x22) as *const u8) as i32);
        v > r3
    };
    if !passa || cqd10_i(this + 0x20) <= 2 {
        return;
    }
    crate::comparacao::ramo("cqd_10 vira tecnico");
    let id = cqd10_i(cqd10_d(this + 0x1c));
    core::ptr::write_unaligned((p + 0x69) as *mut u32, cqd10_d(0xae23c0).wrapping_add((id as isize * 0x44) as usize) as u32);
    let reserva: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x7a82b0usize);
    reserva(this, cqd10_i(cqd10_d(this + 0x1c)));
    if cqd10_b(cqd10_d(p + 0x61) + 0xf) == 0x14 {
        let nj = cqd10_d(p + 0x69);
        let (a, b) = (cqd10_b(nj + 0x10), cqd10_b(nj + 0x11));
        if a > b {
            *((nj + 0x11) as *mut i8) = a;
            *((cqd10_d(p + 0x69) + 0x10) as *mut i8) = b;
            if cqd10_sortear(4) == 0 {
                let v = (cqd10_sortear(0x32) as u8).wrapping_add(0x32);
                *((cqd10_d(p + 0x69) + 0x11) as *mut u8) = v;
            }
        }
    }
    let normalizar: unsafe extern "C" fn(usize) = core::mem::transmute(0x539490usize);
    normalizar(p);
    let f = cqd10_d(p + 0x61);
    let nj = cqd10_d(p + 0x69);
    if cqd10_w(f + 0xb) > 0x1d4c {
        cqd10_atributos_do_tecnico(p, f, nj, f64_em(0x9689d8), [f64_em(0x9690b8), f64_em(0x9690b8), f64_em(0x96e298)]);
    } else if cqd10_sortear(cqd10_b(p + 0x57) as i32) != 0 {
        cqd10_atributos_do_tecnico(p, f, nj, f64_em(0x969040), [f64_em(0x96d300), f64_em(0x96d300), f64_em(0x96e290)]);
    } else {
        cqd10_atributos_do_tecnico(p, f, nj, f64_em(0x968968), [f64_em(0x96e288), f64_em(0x96e288), f64_em(0x967890)]);
    }
}

/// A aposentadoria que venceu (o item `item` da pessoa `p`).
unsafe fn cqd10_aposentar(this: usize, item: usize, p: usize, vaga: &mut [u32; 3]) {
    let id = cqd10_i(item);
    if cqd10_b(p + 0x18) < 0x1e && cqd10_b(item + 0x18) != 8 // cqd10: o jovem não se aposenta
        && (cqd10_d(p + 0x39) != 0 || *cqd10_marca(cqd10_i(p)) & 0xf != 0) {
        crate::comparacao::ramo("cqd_10 aposentadoria desfeita");
        cqd10_tirar(this, item);
        return;
    }
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    if ficha(cqd10_d(0xae1050), p) != 0 || oferta(cqd10_d(0xde1f54), p, 0) != 0 {
        *cqd10_marca(id) &= 0xcf;
        cqd10_tirar(this, item);
        return;
    }
    let antigo = cqd10_i(item + 4);
    let clube = cqd10_d(p + 0x39);
    let mudou = if antigo != -1 { clube != 0 && cqd10_i(clube) != antigo } else { clube != 0 };
    let mut acertar = true;
    if mudou {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        if dias(0xae2c90, cqd10_d(p + 0x3e) as u32, cqd10_d(p + 0x42) as u32) < 0xd2 && cqd10_b(p + 0x18) < 0x26 {
            acertar = false;
            let c = cqd10_contrato(p);
            let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
            if c != 0 && listado(p, 0) == 0 && cqd10_d(c + 0x45) == 0 {
                crate::comparacao::ramo("cqd_10 mudou de clube e fica");
                *cqd10_marca(id) &= 0xcf;
                cqd10_tirar(this, item);
                return;
            }
        }
    }
    if acertar && cqd10_d(p + 0x39) == 0 && cqd10_i(item + 4) != -1 {
        core::ptr::write_unaligned((item + 4) as *mut i32, -1);
    }
    // ponytail: a original empurra o motivo e a marca de anunciada em registradores com bytes de cima que sobram; a
    // 0x7a92e0 lê os bytes
    let noticia: unsafe extern "stdcall" fn(usize, u32, u32) = core::mem::transmute(0x7a92e0usize);
    noticia(p, *((item + 0x18) as *const u8) as u32, *((item + 0x19) as *const u8) as u32);
    let ofertas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ccbf0usize);
    ofertas(cqd10_d(0xae1050), p);
    if cqd10_d(p + 0x61) != 0 && cqd10_d(p + 0x69) != 0 {
        if cqd10_contrato(p) != 0 {
            let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x85f800usize);
            limpar(cqd10_contrato(p));
        }
    } else {
        crate::comparacao::ramo("cqd_10 tira o contrato");
        cqd10_com(0x4d0b50, 0xae1050, p); // cqd10: tira o contrato
    }
    cqd10_com(0x775ba0, 0xb74318, p);
    let sm = cqd10_d(cqd10_d(0xde1f54) + 0x856);
    if sm != 0 {
        let c = cqd10_d(p + 0x39);
        if c != 0 {
            let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
            situacao(sm, p, c, 4, 2);
        }
        let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
        suspender(sm, p, 0);
        let c = cqd10_d(p + 0x39);
        if c != 0 {
            *(cqd10_d(sm + 0xdc).wrapping_add((cqd10_i(c) as isize * 9 + 8) as usize) as *mut u8) = 100;
        }
    }
    let mut banco = false;
    if cqd10_d(p + 0x61) != 0 {
        let elenco: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x7aace0usize);
        crate::comparacao::ramo("cqd_10 sai do elenco");
        elenco(p); // cqd10: sai do elenco
        if cqd10_d(p + 0x39) == 0 {
            cqd10_com(0x4d0b50, 0xae1050, p);
        }
        cqd10_com(0x4196a0, 0xad9c6c, p);
        cqd10_com(0x4197c0, 0xad9c6c, p);
        cqd10_com(0x56a090, 0xae2a30, p);
        if cqd10_d(p + 0x61) != 0 {
            if cqd10_d(p + 0x69) != 0 {
                banco = true;
            } else {
                cqd10_vira_tecnico(this, p);
            }
        }
    }
    if !banco {
        banco = cqd10_d(p + 0x69) != 0 && cqd10_d(p + 0x61) != 0;
    }
    if banco {
        // o jogador com registro de não jogador: a ficha vai para o banco dos regens e ele segue como não jogador
        crate::comparacao::ramo("cqd_10 jogador para o banco");
        cqd10_com(0x7c6650, 0xb74350, p);
        cqd10_com(0x55e1d0, 0xae28f0, p);
        cqd10_com(0x631f90, 0xb63d28, p);
        let guardar: unsafe extern "thiscall" fn(usize, i32, u32, u32) = core::mem::transmute(0x7a8350usize);
        let pais = cqd10_d(p + 0x1a);
        if pais == 0 {
            let n = cqd10_sortear(cqd10_i(0xae2350));
            guardar(this, cqd10_i(p), n as u32, u32::MAX);
        } else if cqd10_d(p + 0x1e) == 0 {
            guardar(this, cqd10_i(p), cqd10_d(pais) as u32, u32::MAX);
        } else {
            guardar(this, cqd10_i(p), cqd10_d(pais) as u32, cqd10_d(cqd10_d(p + 0x1e)) as u32);
        }
        core::ptr::write_unaligned((p + 0x61) as *mut u32, 0);
        *cqd10_marca(cqd10_i(item)) &= 0xcf;
        *((p + 0x5f) as *mut u8) = 1;
        if cqd10_d(p + 0x39) != 0 {
            let c = cqd10_contrato(p);
            if c != 0 {
                let inicial: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85e960usize);
                inicial(c, p, 0);
            }
        }
        cqd10_tirar(this, item);
        return;
    }
    // a vaga para o regen: hoje (até 29 anos) ou de 1 a 65 dias depois (se passar do dia 365, sorteio(35) do ano
    // seguinte)
    vaga[0] = cqd10_i(item) as u32;
    let hoje = cqd10_d(0xae2c90) as u32;
    let (dia, ano) = if cqd10_b(p + 0x18) < 0x1e {
        (hoje as u16, cqd10_w(0xae2c92) as u16)
    } else {
        let v = (cqd10_sortear(0x41) as u32).wrapping_add(hoje).wrapping_add(1) as u16; // cqd10: o dia do regen
        crate::comparacao::ramo("cqd_10 dia do regen sorteado");
        if v as i16 <= 0x16d {
            (v, cqd10_w(0xae2c92) as u16)
        } else {
            (cqd10_sortear(0x23) as u16, (cqd10_w(0xae2c92) as u16).wrapping_add(1))
        }
    };
    vaga[1] = (vaga[1] & 0xffff_0000) | dia as u32;
    vaga[1] = (vaga[1] & 0xffff) | (ano as u32) << 16;
    let m = cqd10_marca(cqd10_i(item));
    *m = (*m & 0xef) | 0x20;
    let guardar_vaga: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(0x7a87d0usize);
    let nj = cqd10_d(p + 0x69);
    if nj != 0 {
        let limpar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86fc20usize);
        limpar(cqd10_d(0xdd7f00), nj);
        let c = cqd10_d(p + 0x39);
        let cargo = |p: usize| cqd10_b(p + 0x3d);
        let sair = if c == 0 || cargo(p) == 5 {
            true
        } else {
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if humano(c, 1, 1, 0, 0) != 0 && !matches!(cargo(p), 1 | 3 | 2 | 4) {
                true
            } else if cqd10_sortear(4) != 0 {
                false
            } else {
                !matches!(cargo(p), 1 | 3 | 2 | 4 | 0xa | 9)
            }
        };
        if sair {
            let cargos: unsafe extern "stdcall" fn(usize) = core::mem::transmute(0x7aaeb0usize);
            cargos(p);
            guardar_vaga(this, vaga.as_ptr());
        } else {
            crate::comparacao::ramo("cqd_10 refeito como nao jogador");
            let refazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7ab340usize);
            refazer(this, p);
        }
        cqd10_tirar(this, item);
        return;
    }
    guardar_vaga(this, vaga.as_ptr());
    cqd10_tirar(this, item);
}

/// { 0x7a59e0 } (thiscall, ret 0, player_regen.cpp; quadro SEH sem destrutor no caminho normal):
/// `PLAYER_REGEN::process_retirements()`, todo dia pelo laço do dia. A vaga local para o regen (id e a data padrão
/// 0x54a110, que deixa o bissexto 1 nela); ordena as aposentadorias pendentes ([this+4], [this+8] de 26 bytes: +0 a
/// pessoa, +4 o clube ou −1, +8/+0xa o dia e o ano do anúncio, +0x10/+0x12 os da aposentadoria, +0x18 o motivo, +0x19
/// anunciada) com 0x7ae2a0 e anda por elas (o ponteiro só avança no anúncio: a 0x7a8690 puxa as seguintes). Vencida
/// (a data passou): o jovem (até 29) com clube ou marca, fora do motivo 8, desiste; com ficha de contrato (0x4cf6d0)
/// ou sob oferta (0x8cde70), desiste (marcas & 0xcf); quem mudou de clube há menos de 210 dias, até 37 anos, com
/// contrato, fora da lista e sem +0x45, fica; senão a notícia (0x7a92e0), as ofertas (0x4ccbf0), o contrato, a seleção
/// (0x775ba0), as listas de observação, o elenco (0x7aace0), as faltas (0x4196a0, 0x4197c0) e a disputa (0x56a090); o
/// jogador pode virar técnico; com ficha e registro de não jogador, a ficha vai para o banco (0x7a8350) e o contrato
/// novo (0x85e960); senão a vaga para o regen (0x7a87d0) e, com registro de não jogador, sai dos cargos (0x7aaeb0) ou
/// é refeito como não jogador (0x7ab340). Não vencida: com a data do anúncio passada e sem anúncio, a notícia do
/// anúncio (0x7aa340, se as marcas & 0x30 não forem 0x10), a marca de anunciada e as marcas 0x10; senão para. No fim
/// ordena as vagas para o regen ([this+0x10], [this+0x14] de 12 bytes) com 0x7ae240.
unsafe extern "thiscall" fn aposentadorias_do_dia(this: usize) {
    let mut vaga = [0u32; 3]; // a pessoa e a data (dia, ano, bissexto)
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(vaga.as_mut_ptr() as usize + 4);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cqd10_d(this + 4), cqd10_d(this + 8) as u32, 0x1a, 0x7ae2a0);
    let n = cqd10_i(this + 8);
    let mut item = cqd10_d(this + 4);
    let mut i = 0i32;
    while i < n {
        let hoje = cqd10_d(0xae2c90) as u32;
        let (dia, ano) = (hoje as u16 as i16, (hoje >> 16) as u16);
        let (vdia, vano) = (cqd10_w(item + 0x10), cqd10_w(item + 0x12) as u16);
        if ano > vano || (ano == vano && dia > vdia) {
            let p = cqd10_d(0xae23bc).wrapping_add((cqd10_i(item) as isize * 0x6e) as usize);
            cqd10_aposentar(this, item, p, &mut vaga);
        } else {
            if *((item + 0x19) as *const u8) != 0 {
                break;
            }
            let (adia, aano) = (cqd10_w(item + 8), cqd10_w(item + 0xa) as u16);
            if !(ano > aano || (ano == aano && dia > adia)) {
                break;
            }
            crate::comparacao::ramo("cqd_10 aposentadoria anunciada");
            let id = cqd10_i(item);
            if *cqd10_marca(id) & 0x30 != 0x10 {
                let anunciar: unsafe extern "stdcall" fn(usize, u32) = core::mem::transmute(0x7aa340usize);
                // ponytail: a original empurra o motivo no edx com bytes de cima que sobram; a 0x7aa340 lê o byte
                anunciar(cqd10_d(0xae23bc).wrapping_add((id as isize * 0x6e) as usize), *((item + 0x18) as *const u8) as u32);
            }
            let id = cqd10_i(item);
            *((item + 0x19) as *mut u8) = 1;
            let m = cqd10_marca(id);
            *m = (*m & 0xdf) | 0x10;
            item += 0x1a;
        }
        i += 1;
    }
    qsort(cqd10_d(this + 0x10), cqd10_d(this + 0x14) as u32, 0xc, 0x7ae240);
}

// ponytail: chama as notícias (alocam e entregam), o regen como não jogador, os contratos novos e as listas: o caminho
// só se sabe rodando e não pode rodar duas vezes. Só a original; a cópia leva os ramos e a carreira a prova.
unsafe extern "thiscall" fn comparar_aposentadorias_do_dia(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1567)(this)
}

/// { 0x7a92e0 } (stdcall, ret 0xc, player_regen.cpp; a 0x7a59e0 põe o this no ecx, sem uso; quadro SEH sem destrutor
/// no caminho normal): `player_regen_post_retirement_news(pessoa, motivo, anunciada)`. Sem pessoa, jogador com cargo 8
/// ou 5, sem clube ou clube sem técnico humano (0x7875e0): nada. A notícia 0xbba (local): 0..3 a pessoa (id e os +0x33
/// de +4/+8/+0xc), 4 o motivo, 5 anunciada, 6 o clube (−1 sem), 7 a nação (+0x1a) com reputação (+0x22) ou −1, 8 o
/// cargo (0xf vira 8, 0xd vira 6), 9 a reputação, 0xa +0x28, 0xb ficha e registro de não jogador, 0xc a idade, 0xd o
/// registro de não jogador, 0xe a ficha. Os interessados (0x8d1da0 em [0xde1f54] com 0xa, até 30, só para jogador).
/// Nação de peso (+0x85 acima de 8) e estrela (+0xd da ficha acima de 8500, ou técnico de cargo 5 com +0xc acima de
/// 8750): a todos (0x786780); estrela menor (+0xd acima de 7500, ou cargo 5 com +0xa acima de 7250): aos técnicos da
/// nação e da do clube (0x7863f0); técnico de cargo 5 de clube com divisão: aos da divisão (0x7862d0); não jogador: ao
/// clube (0x7861e0); jogador: aos interessados. Motivo 8 com +0xb da ficha acima de 4250 e clube de contrato
/// (0x4cf5a0, guardado em [0xae1080]): o seguro (0x58a980 com +0x52).
unsafe extern "stdcall" fn noticia_da_aposentadoria(p: usize, motivo: u32, anunciada: u32) {
    if p == 0 {
        return;
    }
    if cqd10_d(p + 0x61) != 0 && matches!(cqd10_b(p + 0x3d), 8 | 5) {
        return;
    }
    let clube = cqd10_d(p + 0x39);
    if clube == 0 {
        return;
    }
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(clube) == 0 {
        return;
    }
    crate::comparacao::ramo("cqd_10 noticia da aposentadoria");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    construir(nw, 0xbba, 0);
    gravar(nw, 0, cqd10_d(p) as u32);
    for k in 1..4usize {
        gravar(nw, k as i8, cqd10_d(cqd10_d(p + 4 * k) + 0x33) as u32);
    }
    gravar(nw, 4, motivo as u8 as i8 as i32 as u32);
    gravar(nw, 5, anunciada as u8 as i8 as i32 as u32);
    let c = cqd10_d(p + 0x39);
    gravar(nw, 6, if c != 0 { cqd10_d(c) as u32 } else { u32::MAX });
    gravar(nw, 7, if *((p + 0x22) as *const u8) != 0 { cqd10_d(cqd10_d(p + 0x1a)) as u32 } else { u32::MAX });
    let cargo = match cqd10_b(p + 0x3d) {
        0xf => 8,
        0xd => 6,
        v => v,
    };
    gravar(nw, 8, cargo as i32 as u32);
    gravar(nw, 9, *((p + 0x22) as *const u8) as u32);
    gravar(nw, 0xa, cqd10_b(p + 0x28) as i32 as u32);
    gravar(nw, 0xb, (cqd10_d(p + 0x61) != 0 && cqd10_d(p + 0x69) != 0) as u32);
    gravar(nw, 0xc, cqd10_b(p + 0x18) as i32 as u32);
    gravar(nw, 0xd, (cqd10_d(p + 0x69) != 0) as u32);
    gravar(nw, 0xe, (cqd10_d(p + 0x61) != 0) as u32);
    let mut clubes = [0usize; 30];
    let tm = cqd10_d(0xde1f54);
    let n = if tm != 0 && cqd10_d(p + 0x61) != 0 {
        let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
        interessados(tm, p, 0xa, clubes.as_mut_ptr(), 0x1e) as i8
    } else {
        0
    };
    let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let da_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mn = cqd10_d(0xb7431c);
    let pais = cqd10_d(p + 0x1a);
    let f = cqd10_d(p + 0x61);
    let nj = cqd10_d(p + 0x69);
    let cargo5 = || cqd10_b(p + 0x3d) == 5;
    if pais != 0 && cqd10_b(pais + 0x85) > 8
        && ((f != 0 && cqd10_w(f + 0xd) > 0x2134) || (nj != 0 && cqd10_w(nj + 0xc) > 0x222e && cargo5())) {
        todos(mn, nw);
    } else if (f != 0 && cqd10_w(f + 0xd) > 0x1d4c) || (nj != 0 && cqd10_w(nj + 0xa) > 0x1c52 && cargo5()) {
        do_pais(mn, nw, pais);
        let c = cqd10_d(p + 0x39);
        if c != 0 && cqd10_d(c + 0x53) != cqd10_d(p + 0x1a) {
            do_pais(cqd10_d(0xb7431c), nw, cqd10_d(c + 0x53));
        }
    } else {
        let c = cqd10_d(p + 0x39);
        if c != 0 && cqd10_d(c + 0x57) != 0 && cargo5() {
            da_divisao(mn, nw, cqd10_d(c + 0x57));
        } else if f == 0 {
            ao_clube(mn, nw, c);
        } else if n > 0 {
            for k in 0..n as usize {
                ao_clube(cqd10_d(0xb7431c), nw, clubes[k]);
            }
        }
    }
    if motivo as u8 == 8 {
        let f = cqd10_d(p + 0x61);
        if f != 0 && cqd10_w(f + 0xb) > 0x109a {
            let c = cqd10_contrato(p);
            *(0xae1080 as *mut usize) = c;
            if c != 0 {
                let clube = cqd10_d(0xae23b4).wrapping_add((cqd10_i(c + 4) as isize * 0x245) as usize);
                if clube != 0 {
                    crate::comparacao::ramo("cqd_10 seguro da aposentadoria");
                    let seguro: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x58a980usize);
                    seguro(clube, p, cqd10_d(p + 0x52) as u32);
                }
            }
        }
    }
}

// ponytail: aloca e entrega a notícia (0x77bbe0, 0x786780...). Só a original; a cópia leva o ramo.
unsafe extern "stdcall" fn comparar_noticia_da_aposentadoria(p: usize, motivo: u32, anunciada: u32) {
    orig::<unsafe extern "stdcall" fn(usize, u32, u32)>(1568)(p, motivo, anunciada)
}

/// { 0x7a8ba0 } (stdcall, ret 8, player_regen.cpp; quadro SEH sem destrutor no caminho normal):
/// `player_regen_post_youth_promotion_news(jogador, clube)`, pelo regen jovem (0x7abac0). Sem jogador ou clube: nada.
/// A notícia 0xbce (local): 0..3 o jogador (id e os +0x33 de +4/+8/+0xc), 4 o id do clube, 5 a idade, 6 a força do
/// clube (+0x7f) acima de 15. O auxiliar (+0xd3) e os cinco treinadores (+0x19f) opinam até o primeiro que der
/// veredito: a reputação vista = sorteio(2500) + reputação do clube (+0x80) / 2 + força × 12,5 (truncada); as duas
/// habilidades vistas (0x6e86d0 com 2 e com 1) contra reputação / 50 + 75 (veredito 2) e reputação / 100 + 50 (1).
/// Com veredito: 7 auxiliar?, 8..0xb quem (id e nomes), 0xc e 0xd os vereditos; senão −1. Ao alcance do clube
/// (0x786090).
unsafe extern "stdcall" fn noticia_do_jovem_promovido(j: usize, c: usize) {
    if j == 0 || c == 0 {
        return;
    }
    crate::comparacao::ramo("cqd_10 noticia do jovem promovido");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let vista: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(0x6e86d0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    let (mut v1, mut v2) = (0i8, 0i8);
    let mut quem = 0usize;
    construir(nw, 0xbce, 0);
    gravar(nw, 0, cqd10_d(j) as u32);
    for k in 1..4usize {
        gravar(nw, k as i8, cqd10_d(cqd10_d(j + 4 * k) + 0x33) as u32);
    }
    gravar(nw, 4, cqd10_d(c) as u32);
    gravar(nw, 5, cqd10_b(j + 0x18) as i32 as u32);
    gravar(nw, 6, (cqd10_b(c + 0x7f) > 0xf) as u32);
    let mut atual = 0usize;
    let mut k = -1i8;
    loop {
        if atual != 0 {
            break;
        }
        let t = if k == -1 { cqd10_d(c + 0xd3) } else { cqd10_d(c + 0x19f + 4 * k as usize) };
        if t != 0 {
            let r = cqd10_sortear(0x9c4);
            let v = r.wrapping_add(cqd10_w(c + 0x80) as i32 / 2);
            let x = ftol(v as f64 + cqd10_b(c + 0x7f) as f64 * f64_em(0x96afc8));
            // ponytail: a original empurra o eax inteiro do _ftol e o da primeira vista; a 0x6e86d0 lê a palavra
            let a = vista(t, j, c, 2, x as i16);
            let b = vista(t, j, c, 1, a);
            let rep = cqd10_w(c + 0x80) as i32;
            let alto = rep / 50 + 0x4b;
            if a as i32 > alto {
                v1 = 2;
                quem = t;
            } else if a as i32 > rep / 100 + 0x32 {
                v1 = 1;
                quem = t;
            }
            if b as i32 > alto {
                v2 = 2;
                quem = t;
            } else if b as i32 > rep / 100 + 0x32 {
                v2 = 1;
                quem = t;
            }
            atual = quem;
        }
        k += 1;
        if k >= 5 {
            break;
        }
    }
    if atual != 0 {
        gravar(nw, 7, (atual == cqd10_d(c + 0xd3)) as u32);
        gravar(nw, 8, cqd10_d(atual) as u32);
        for k in 1..4usize {
            gravar(nw, 8 + k as i8, cqd10_d(cqd10_d(atual + 4 * k) + 0x33) as u32);
        }
        gravar(nw, 0xc, v1 as i32 as u32);
        gravar(nw, 0xd, v2 as i32 as u32);
    } else {
        for k in 7..14i8 {
            gravar(nw, k, u32::MAX);
        }
    }
    let alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    alcance(cqd10_d(0xb7431c), nw, c, 0, 0, 0, 0);
}

// ponytail: aloca e entrega a notícia e sorteia a opinião: só a original; a cópia leva o ramo.
unsafe extern "stdcall" fn comparar_noticia_do_jovem_promovido(j: usize, c: usize) {
    orig::<unsafe extern "stdcall" fn(usize, usize)>(1569)(j, c)
}

// ---------- 267a leva: o fim da falta ao treino e da ausência sem licença e a notícia da falta ao treino (awol.cpp) ----------

unsafe fn cqd11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os itens da lista (o ponteiro em +0 de cada nó, da cabeça +2 seguindo o +6), sem mexer no cursor.
unsafe fn cqd11_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqd11_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqd11_d(no));
        no = cqd11_d(no + 6);
    }
    v
}

/// O corpo comum de { 0x4196a0 } e { 0x4197c0 }: com a marca `bit` da pessoa ([this+0xc] + id), a marca troca (o
/// `or bit` da original nunca dá zero) e, com a lista ([this + `lista`]), o cursor volta à cabeça e, entre os primeiros
/// `conta` (+0, palavra com sinal) itens, o primeiro com o id da pessoa em +0 sai (0x68d1f0, que libera o nó). Uma lista
/// que acaba antes da conta encerra.
unsafe fn cqd11_tirar(this: usize, pessoa: usize, bit: u8, lista: usize, falta: bool) {
    if pessoa == 0 {
        return;
    }
    let marcas = cqd11_d(this + 0xc);
    if marcas == 0 {
        return;
    }
    let p = marcas.wrapping_add(cqd11_d(pessoa));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return;
    }
    if falta {
        crate::comparacao::ramo("cqd_11 falta tirada");
    }
    *(p as *mut u8) = m ^ bit;
    let l = cqd11_d(this + lista);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqd11_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    // 0x68d1b0: mov ecx, [edi + lista] -> thiscall; 0x68d1f0: push item; mov ecx, [edi + lista]; ret 4 -> thiscall
    let mut i: i32 = 0;
    if cqd11_w(l) as i32 <= 0 {
        return;
    }
    loop {
        let item = proximo(cqd11_d(this + lista));
        if item == 0 {
            return;
        }
        if cqd11_d(item) == cqd11_d(pessoa) {
            if falta {
                crate::comparacao::ramo("cqd_11 registro da falta tirado");
            }
            tirar(cqd11_d(this + lista), item);
            return;
        }
        i += 1;
        if i >= cqd11_w(cqd11_d(this + lista)) as i32 {
            return;
        }
    }
}

/// { 0x4196a0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_awol(pessoa)`, pelo fim da ausência sem contrato (0x418f60),
/// pelas aposentadorias (0x7a59e0) e pelas transferências (0x8c1af0, 0x8c2180, 0x8c9720). Pessoa e tabela de marcas
/// ([this+0xc]) não nulas e o bit 1 da marca: troca o bit e tira o registro da pessoa da lista das ausências ([this]).
unsafe extern "thiscall" fn limpar_ausencia(this: usize, pessoa: usize) {
    cqd11_tirar(this, pessoa, 1, 0, false);
}

/// { 0x4197c0 } (thiscall, ret 4, awol.cpp): `AWOL::clear_missed_training(pessoa)`, pela falta ao treino de quem ficou
/// sem contrato (0x418ff0, que os jogadores do dia consultam) e pelas aposentadorias (0x7a59e0). Pessoa e tabela de
/// marcas não nulas e o bit 2 da marca: troca o bit e tira o registro da pessoa da lista das faltas ([this+4]).
unsafe extern "thiscall" fn limpar_falta_ao_treino(this: usize, pessoa: usize) {
    cqd11_tirar(this, pessoa, 2, 4, true);
}

/// O plano das duas: a marca depois e os itens que ficam na lista (o primeiro com o id da pessoa entre os `conta`
/// primeiros sai). Sem nada a fazer: o estado de antes.
unsafe fn cqd11_plano(this: usize, pessoa: usize, bit: u8, lista: usize) -> (Option<(usize, u8)>, usize, Vec<usize>) {
    let l = cqd11_d(this + lista);
    let itens = cqd11_itens(l);
    if pessoa == 0 || cqd11_d(this + 0xc) == 0 {
        return (None, l, itens);
    }
    let p = cqd11_d(this + 0xc).wrapping_add(cqd11_d(pessoa));
    let m = *(p as *const u8);
    if m & bit == 0 {
        return (Some((p, m)), l, itens);
    }
    if l == 0 {
        return (Some((p, m ^ bit)), l, itens);
    }
    let conta = (cqd11_w(l) as i32).max(0) as usize;
    let mut fica = itens.clone();
    if let Some(k) = itens.iter().take(conta).position(|&x| cqd11_d(x) == cqd11_d(pessoa)) {
        fica.remove(k);
    }
    (Some((p, m ^ bit)), l, fica)
}

/// Comparador das duas: a original tira e libera o nó da lista (não pode rodar duas vezes): só ela roda, com o plano
/// montado antes (a marca e os itens que ficam, sem ler o item tirado depois).
unsafe fn cqd11_comparar(this: usize, pessoa: usize, bit: u8, lista: usize, o: unsafe extern "thiscall" fn(usize, usize), nome: &str) {
    let (marca, l, fica) = cqd11_plano(this, pessoa, bit, lista);
    o(this, pessoa);
    let depois = cqd11_itens(l);
    let marca_ok = marca.map_or(true, |(p, m)| *(p as *const u8) == m);
    PLACAR.registrar(marca_ok && depois == fica, || format!("{}(pessoa {:#x}) marca plano {:x?} agora {:x?}; lista plano {:x?} agora {:x?}",
        nome, pessoa, marca, marca.map(|(p, _)| *(p as *const u8)), fica, depois));
}

unsafe extern "thiscall" fn comparar_limpar_ausencia(this: usize, pessoa: usize) {
    cqd11_comparar(this, pessoa, 1, 0, orig::<unsafe extern "thiscall" fn(usize, usize)>(1570), "limpar_ausencia");
}

unsafe extern "thiscall" fn comparar_limpar_falta_ao_treino(this: usize, pessoa: usize) {
    cqd11_comparar(this, pessoa, 2, 4, orig::<unsafe extern "thiscall" fn(usize, usize)>(1571), "limpar_falta_ao_treino");
}

/// O ensaio da notícia: os sorteios vêm da fita da original (n e resposta) e, em vez de entregar, a cópia anota o
/// destino (1 o clube do jogador, 2 a nação do clube, 0 nenhum). A fita fica com o próximo e se algum n não bateu.
static mut CQD11_FITA: Option<(Vec<(i32, i32)>, usize, bool)> = None;
static mut CQD11_DESTINO: (u32, usize) = (0, 0);

unsafe fn cqd11_sortear(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(CQD11_FITA);
    match f {
        Some((v, k, falhou)) => match v.get(*k) {
            Some(&(m, r)) if m == n && !*falhou => {
                *k += 1;
                r
            }
            _ => {
                *falhou = true;
                0
            }
        },
        None => {
            let s: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
            s(n)
        }
    }
}

/// { 0x41a630 } (thiscall, ret 8, awol.cpp, SEH da notícia local, sem destrutor): `awol_post_missed_training_news(jogador,
/// clube)`, pela falta ao treino (0x41a360). Jogador com ficha (+0x61) e clube com técnico (+0xcf): a notícia 0xbc9
/// (0x77bbe0) com o jogador (id e as chaves +0x33 de +4, +8, +0xc), o id do clube, sorteio(3), o clube de humano que
/// decide (0x600a50 com 0, 1) e quem relata: o técnico, se o clube tem técnico e não é de humano (0x600a50 com 1, 1;
/// id e chaves); senão, com sorteio(2), um dos 5 treinadores (+0x19f, sorteio(5) até achar ou mil voltas), sem
/// sorteio o auxiliar (+0xd3); sem ninguém (ou id −1), −1 nos quatro. Clube de humano (0x7875e0): ao clube do jogador
/// (+0x39, 0x7861e0); senão, com nação (+0x53) e reputação do jogador (+0x61, +0xb) acima de 7250, às caixas da nação
/// (0x7863f0).
unsafe extern "thiscall" fn noticia_da_falta_ao_treino(this: usize, jogador: usize, clube: usize) {
    let _ = this;
    if jogador == 0 || cqd11_d(jogador + 0x61) == 0 || clube == 0 || cqd11_d(clube + 0xcf) == 0 { // cqd11: sem jogador ou técnico
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    // 0x77bbe0: push 0; push 0xbc9; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall; 0x600a50: 5
    // pushes; add esp, 0x14 -> cdecl (clube, 0, 1, 0, 0) e (clube, 1, 1, 0, 0)
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbc9, 0);
    gravar(nt, 0, cqd11_d(jogador) as u32);
    gravar(nt, 1, cqd11_d(cqd11_d(jogador + 4) + 0x33) as u32);
    gravar(nt, 2, cqd11_d(cqd11_d(jogador + 8) + 0x33) as u32);
    gravar(nt, 3, cqd11_d(cqd11_d(jogador + 0xc) + 0x33) as u32);
    gravar(nt, 4, cqd11_d(clube) as u32);
    crate::comparacao::ramo("cqd_11 noticia da falta");
    let r = cqd11_sortear(3); // cqd11: o sorteio da notícia
    gravar(nt, 5, r as u32);
    gravar(nt, 6, humano(clube, 0, 1, 0, 0));
    let tecnico = cqd11_d(clube + 0xcf);
    if tecnico != 0 && humano(clube, 1, 1, 0, 0) == 0 { // cqd11: o técnico da IA relata
        crate::comparacao::ramo("cqd_11 relator tecnico");
        let t = cqd11_d(clube + 0xcf);
        gravar(nt, 7, cqd11_d(t) as u32);
        gravar(nt, 8, cqd11_d(cqd11_d(t + 4) + 0x33) as u32);
        gravar(nt, 9, cqd11_d(cqd11_d(t + 8) + 0x33) as u32);
        gravar(nt, 0xa, cqd11_d(cqd11_d(t + 0xc) + 0x33) as u32);
    } else {
        let quem = if cqd11_sortear(2) != 0 {
            let mut k;
            let mut voltas = 0;
            loop {
                k = cqd11_sortear(5);
                voltas += 1;
                if voltas >= 1000 || cqd11_d(clube.wrapping_add(k as usize * 4) + 0x19f) != 0 {
                    break;
                }
            }
            cqd11_d(clube.wrapping_add(k as usize * 4) + 0x19f)
        } else {
            cqd11_d(clube + 0xd3)
        };
        let id = if quem != 0 { cqd11_d(quem) as i32 } else { -1 };
        if id == -1 {
            for k in 7..=0xa {
                gravar(nt, k, u32::MAX);
            }
        } else {
            let p = cqd11_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            gravar(nt, 7, id as u32);
            gravar(nt, 8, cqd11_d(cqd11_d(p + 4) + 0x33) as u32);
            gravar(nt, 9, cqd11_d(cqd11_d(p + 8) + 0x33) as u32);
            gravar(nt, 0xa, cqd11_d(cqd11_d(p + 0xc) + 0x33) as u32);
        }
    }
    let noticias = cqd11_d(0xb7431c);
    let de_humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    // 0x7875e0: mov ecx, [0xb7431c]; push clube (ret 4); 0x7861e0: push clube +0x39; push notícia; mov ecx, [0xb7431c];
    // 0x7863f0: push nação; push notícia; mov ecx, [0xb7431c] -> thiscall nas três
    let ensaio = (*core::ptr::addr_of!(CQD11_FITA)).is_some();
    if de_humano(noticias, clube) != 0 {
        if ensaio {
            CQD11_DESTINO = (1, cqd11_d(jogador + 0x39));
        } else {
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
            entregar(noticias, nt, cqd11_d(jogador + 0x39));
        }
        return;
    }
    let nacao = cqd11_d(clube + 0x53);
    if nacao == 0 || cqd11_w(cqd11_d(jogador + 0x61) + 0xb) <= 0x1c52 {
        return;
    }
    if ensaio {
        CQD11_DESTINO = (2, nacao);
    } else {
        let a_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
        a_nacao(noticias, nt, nacao);
    }
}

/// Comparador: a original monta a notícia e a entrega (aloca na caixa): só ela roda, com os sorteios rastreados; depois
/// a cópia em ensaio refaz o caminho com os sorteios da fita, sem entregar. Confere que os sorteios do ensaio são o
/// começo da fita da original (o resto é da entrega) com os mesmos n, e anota o destino (a original não o expõe).
unsafe extern "thiscall" fn comparar_noticia_da_falta_ao_treino(this: usize, jogador: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1572);
    let ((), fita) = crate::aleatorio::rastrear(|| o(this, jogador, clube));
    let n = fita.len();
    *core::ptr::addr_of_mut!(CQD11_FITA) = Some((fita, 0, false));
    CQD11_DESTINO = (0, 0);
    crate::aleatorio::sem_contar(|| noticia_da_falta_ao_treino(this, jogador, clube));
    let (k, falhou) = match &*core::ptr::addr_of!(CQD11_FITA) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(CQD11_FITA) = None;
    let destino = CQD11_DESTINO;
    PLACAR.registrar(!falhou && k <= n, || format!("noticia_da_falta_ao_treino(jogador {:#x}, clube {:#x}) fita de {} sorteios, o ensaio usou {} (n diferente: {}), destino {:x?}",
        jogador, clube, n, k, falhou, destino));
}

// ---------- 267a leva: a troca com compensação, a ordem das vagas da lista de compras e o jogador sob oferta (shortlist_manager.cpp) ----------

unsafe fn cqd12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Contador de ramo para a rotina quente (~27 milhões de chamadas até 2002-06-01): o `ramo` só a cada 4096 passagens
/// (a primeira inclusive), para não pesar na carreira.
fn cqd12_contar(n: &core::sync::atomic::AtomicU32, nome: &'static str) {
    // ponytail: rotina quente; o contador do ramo conta de 4096 em 4096
    if n.fetch_add(1, core::sync::atomic::Ordering::Relaxed) & 0xfff == 0 {
        crate::comparacao::ramo(nome);
    }
}

static CQD12_BOSMAN: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static CQD12_FAIXA: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static CQD12_LONGE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// { 0x84b6f0 } (thiscall, ret 0x10, shortlist_manager.cpp; o this não é lido): `shortlist_manager_is_compensation_deal(
/// vaga, contrato, clube, &taxa)`, pela vaga da lista de compras (0x842e10, 0x840000), pelo relatório do olheiro
/// (0x7b62a0), pelo lance (0x8e5d60) e pela tela. Zera a taxa; sem contrato, sem vaga ou com o byte +0x49 do contrato:
/// 1. A pessoa do contrato ([0xae23bc] + 110 × [+0]). Fora do período protegido da FIFA (0x868210 zero): a taxa de
/// compensação (0x8d0780 com 0) e 1 se ela é menor que o valor (+0x52) da pessoa. Senão: sem lei Bosman no clube do
/// jogador (0x8d0a70), 0; a taxa (0x8d0780 com 1) acima do valor, 0; pela faixa do vencimento (0x867d30): 0 → 0; 1-2,
/// 3-4, 5-6, 7-8 → 1 se o +3 da vaga × 0,5 / 0,65 / 0,8 / 0,9 passa do preço (+0xb); 9-14 → 1; acima, 0.
unsafe extern "thiscall" fn troca_com_compensacao(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let _ = this;
    *taxa = 0;
    if contrato == 0 || vaga == 0 || *((contrato + 0x49) as *const u8) != 0 {
        return 1;
    }
    let pessoa = |c: usize| cqd12_d(0xae23bc).wrapping_add(cqd12_d(c).wrapping_mul(0x6e));
    let protegido: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x868210usize);
    let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(0x8d0780usize);
    let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8d0a70usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    // 0x868210: push 0; push 0; push clube; mov ecx, contrato -> thiscall; 0x8d0780 e 0x8d0a70: mov ecx, [0xde1f54]
    // antes dos pushes (a cópia passa o ecx como a original; as duas só leem a pilha, ret 0xc e ret 4); 0x867d30: mov
    // ecx, contrato -> thiscall
    let gerente = cqd12_d(0xde1f54);
    if protegido(contrato, clube, 0, 0) == 0 {
        let t = compensacao(gerente, pessoa(contrato), clube, 0);
        *taxa = t;
        return (t < cqd12_i(pessoa(contrato) + 0x52)) as u32;
    }
    if bosman(gerente, pessoa(contrato)) == 0 { // cqd12: sem a lei Bosman
        return 0;
    }
    cqd12_contar(&CQD12_BOSMAN, "cqd_12 com lei Bosman");
    let t = compensacao(gerente, pessoa(contrato), clube, 1);
    *taxa = t;
    if t > cqd12_i(pessoa(contrato) + 0x52) {
        return 0;
    }
    let k = faixa(contrato) as i8 as i32;
    if k as u32 > 0xe {
        return 0;
    }
    let fator = match k {
        0 => return 0,
        1 | 2 => 0.5, // cqd12: a faixa 1-2
        3 | 4 => 0.65,
        5 | 6 => 0.8,
        7 | 8 => 0.9,
        _ => {
            cqd12_contar(&CQD12_LONGE, "cqd_12 faixa longe");
            return 1; // cqd12: as faixas 9-14
        }
    };
    cqd12_contar(&CQD12_FAIXA, "cqd_12 faixa pelo fator");
    // fild [vaga+0xb]; fild [vaga+3]; fmul fator; fcompp: 1 se o +3 × fator passa do +0xb (precisão dupla)
    (cqd12_i(vaga + 3) as f64 * fator > cqd12_i(vaga + 0xb) as f64) as u32
}

/// Comparador: lado a lado (rotina pura: só a taxa e a devolução), pelas cópias leves.
unsafe extern "thiscall" fn comparar_troca_com_compensacao(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32>(1573);
    let reg = [(taxa as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, vaga, contrato, clube, taxa),
        || troca_com_compensacao(this, vaga, contrato, clube, taxa));
    PLACAR.registrar(igual && r == n, || format!("troca_com_compensacao(vaga {:#x}, contrato {:#x}, clube {:#x}) original={} rust={} {:x?} x {:x?}",
        vaga, contrato, clube, r, n, mo, mr));
    r
}

/// { 0x83b6c0 } (cdecl, shortlist_manager.cpp): `shortlist_compare_slots_by_index(a, b)`, o critério do qsort das 40
/// vagas (0x25 bytes) da lista de compras na avaliação do elenco (0x83cb40): a diferença dos índices (+0x23, byte com
/// sinal); iguais, 1 (nunca empata).
unsafe extern "C" fn ordem_das_vagas(a: usize, b: usize) -> i32 {
    let d = (*((a + 0x23) as *const i8) as i32) - (*((b + 0x23) as *const i8) as i32);
    if d != 0 { d } else { 1 }
}

/// Comparador: a devolução (pura).
unsafe extern "C" fn comparar_ordem_das_vagas(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1574);
    let r = o(a, b);
    let n = ordem_das_vagas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_vagas({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Os clubes interessados no jogador, como a 0x84b580 os pede (0x842180 com modo 1 numa lista local; os itens de 8
/// bytes: id do jogador, id do clube).
unsafe fn cqd12_interessados(this: usize, jogador: usize) -> Vec<usize> {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    let mut v = Vec::new();
    let mut no = cqd12_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqd12_d(cqd12_d(no) + 4));
        no = cqd12_d(no + 6);
    }
    soltar(lista);
    v
}

/// { 0x84b580 } (thiscall, ret 0xc, shortlist_manager.cpp, quadro SEH da lista local 0x68d020/0x68d040):
/// `Shortlist_Manager::mark_player_under_offer(jogador, comprador, troca)`, pela negociação acrescentada (0x8be9d0) e
/// pelo lance aceito (0x8c0080). Os clubes interessados no jogador (0x842180 com modo 1, numa lista local), menos os de
/// humano que decide (0x600a50 com 0, 1): com a lista `troca` (cursor na cabeça), um clube de alguma oferta dela
/// (0x8c61e0 pelo id do item; o clube +8 da oferta) fica de fora; senão a vaga do jogador (0x849ce0, só de tipo 11 se o
/// byte +0x11 da última oferta achada é 11) ganha o bit 0x20 (+0x24); sem a lista, fora o comprador, a vaga de tipo 11
/// ou, sem ela, a de outro tipo. Clube sem vaga encerra.
unsafe extern "thiscall" fn marcar_sob_oferta(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let oferta: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c61e0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    // 0x68d020/0x68d040/0x68d1b0: lea ecx, lista -> thiscall; 0x842180: 5 pushes; mov ecx, ebx (ret 0x14); 0x600a50:
    // 5 pushes; add esp, 0x14 -> cdecl; 0x8c61e0: push id; mov ecx, [0xde1f54] (ret 4); 0x849ce0: 4 pushes; mov ecx,
    // ebx (ret 0x10)
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqd12_d(lista + 2));
    let mut ultima = jogador; // o ebp da original: o jogador, depois a última oferta achada (vale entre os clubes)
    let mut item = proximo(lista);
    'clubes: while item != 0 {
        let k = cqd12_d(item + 4);
        let clube = cqd12_d(0xae23b4).wrapping_add(k.wrapping_mul(0x245));
        if humano(clube, 0, 1, 0, 0) == 0 {
            let v = if troca != 0 {
                core::ptr::write_unaligned((troca + 0xa) as *mut usize, cqd12_d(troca + 2));
                let mut q = proximo(troca);
                while q != 0 {
                    ultima = oferta(cqd12_d(0xde1f54), cqd12_d(q));
                    if ultima != 0 && cqd12_d(cqd12_d(ultima) + 8) == k {
                        item = proximo(lista);
                        continue 'clubes;
                    }
                    q = proximo(troca);
                }
                let tipo = *((cqd12_d(ultima) + 0x11) as *const u8);
                vaga_de(this, clube, jogador, (tipo != 0xb) as u32, 0)
            } else {
                if clube == comprador {
                    item = proximo(lista);
                    continue 'clubes;
                }
                let v = vaga_de(this, clube, jogador, 0, 0);
                if v != 0 { v } else { vaga_de(this, clube, jogador, 1, 0) }
            };
            if v == 0 {
                break;
            }
            *((v + 0x24) as *mut u8) |= 0x20;
        }
        item = proximo(lista);
    }
    soltar(lista);
}

/// Comparador: lado a lado pelas listas de 40 vagas dos clubes interessados (a cópia e a original montam e soltam a
/// mesma lista local) e pelo cursor da lista `troca`.
unsafe extern "thiscall" fn comparar_marcar_sob_oferta(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1575);
    let mut reg: Vec<(usize, usize)> = cqd12_interessados(this, jogador).into_iter()
        .map(|k| cqd12_d(cqd12_d(this + 0xdc).wrapping_add(k.wrapping_mul(9))))
        .filter(|&t| t != 0)
        .map(|t| (t, 40 * 0x25))
        .collect();
    if troca != 0 {
        reg.push((troca + 0xa, 4));
    }
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, comprador, troca),
        || marcar_sob_oferta(this, jogador, comprador, troca));
    PLACAR.registrar(igual, || format!("marcar_sob_oferta(jogador {:#x}, comprador {:#x}, troca {:#x}) {:x?} x {:x?}",
        jogador, comprador, troca, mo, mr));
}

// ---------- 267a leva: os registros do campeão da liga e da copa e o registro do técnico humano zerado (staff_records.cpp) ----------

unsafe fn cqd13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd13_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqd13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd13_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// O ensaio dos comparadores: as cópias sem os históricos (arquivo), sem as notícias (0x75c390), sem acabar as
/// convocações (0x774f30) e sem a renovação (0x4cda70), anotando os contratos que a renovação mexeria.
static mut CQD13_ENSAIO: bool = false;
static mut CQD13_RENOVADOS: Vec<usize> = Vec::new();

/// A reputação do clube (+0x80) sobe até peso × 500, no máximo 750 de uma vez (entre 1 e 10000; soma em 16 bits).
unsafe fn cqd13_reputacao(clube: usize, peso: i16, campeao_da_liga: bool) {
    let alvo = (peso as i32).wrapping_mul(500);
    let rep = cqd13_w(clube + 0x80) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > 0x2ee { 0x2ee } else { falta }; // cqd13: o teto da subida
        let v = cqd13_w(clube + 0x80).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if campeao_da_liga {
            crate::comparacao::ramo("cqd_13 reputacao do campeao da liga");
        } else {
            crate::comparacao::ramo("cqd_13 reputacao do campeao da copa");
        }
        core::ptr::write_unaligned((clube + 0x80) as *mut i16, v); // cqd13: a reputação do campeão
    }
}

/// O moral do contrato (+0x44, byte com sinal) sobe `d`, preso em −100..100.
unsafe fn cqd13_moral(k: usize, d: i32) {
    let c = *((k + 0x44) as *const i8);
    let x = c as i32 + d;
    *((k + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { c.wrapping_add(d as i8) };
}

/// A renovação oferecida ao jogador do próprio clube (0x4cda70 com 1,0 e submeter): com o gerente de transferências
/// ([0xde1f54]), mais de 300 dias desde a data do contrato (+0x25; 0x54cdb0) e a vaga dele na lista do clube (0x849ce0
/// com 0, 0; +0xf o id). No ensaio, só anota o contrato.
unsafe fn cqd13_renovar(clube: usize, p: usize, k: usize) {
    if cqd13_d(0xde1f54) == 0 || cqa12_dias(0xae2c90, k + 0x25) <= 300 {
        return;
    }
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    // 0x849ce0: push 0; push 0; push p; push clube; mov ecx, [[0xde1f54]+0x856] -> thiscall (ret 0x10)
    let v = vaga_de(cqd13_d(cqd13_d(0xde1f54) + 0x856), clube, p, 0, 0);
    if v == 0 || cqd13_i(v + 0xf) != cqd13_i(p) {
        return;
    }
    if CQD13_ENSAIO {
        (*core::ptr::addr_of_mut!(CQD13_RENOVADOS)).push(k);
        return;
    }
    let oferecer: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, f64, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    // 0x4cda70: push 1; push 0x3ff00000; push 0; push −1; push 0; push 0; push p; mov ecx, [0xae1050] -> thiscall
    // (p, 0, 0, −1, 1.0 em dois dwords, 1)
    oferecer(cqd13_d(0xae1050), p, 0, 0, -1, 1.0, 1);
}

/// Os históricos da entrada `e` para o técnico (já feito por quem chama), o presidente (+0xbf) e os 20 da escalação
/// (+0x190 do lado, 0x49 bytes, −1 vazio; pessoas válidas e fora o técnico; nula: a caixa da linha 0x1010).
unsafe fn cqd13_escalacao(this: usize, clube: usize, escalacao: usize, e: &[u8; 0x40]) {
    for j in 0..0x14usize {
        let id = cqd13_i(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = cqd13_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && cqd13_i(p) >= cqd13_i(0xae2364) {
            continue;
        }
        if p == cqd13_d(clube + 0xcf) {
            continue;
        }
        if p == 0 {
            aviso_em(0xa83b78, 0x1010);
            continue;
        }
        cpq4_historico(!CQD13_ENSAIO, this, cqd13_d(p), e);
    }
}

/// O histórico do clube (tipo no +8, a competição no +9).
unsafe fn cqd13_historico_do_clube(this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[8] = tipo;
    cpq4_por(&mut e, 9, cqd13_i(comp));
    if !CQD13_ENSAIO {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
        let p = cqd13_d(this + 0x10).wrapping_add(cqd13_d(clube).wrapping_mul(8));
        f(cqd13_d(this + 4), p, p + 4, e.as_ptr());
    }
}

/// A notícia do elogio do título (0x75c390 em [0xb6b7c8]; fora do ensaio).
unsafe fn cqd13_elogio(clube: usize, comp: usize, marca: u32) {
    if CQD13_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x75c390usize);
    // 0x75c390: push marca; push comp; push clube; mov ecx, [0xb6b7c8] -> thiscall (ret 0xc)
    f(cqd13_d(0xb6b7c8), clube, comp, marca);
}

/// As notas k1/k2 do técnico pela fase: na competição de fase 1 ([0xadadfc] + 4 × id, +0x42 = 1) 200 na terceira e 200
/// na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1), senão 400.
unsafe fn cqd13_notas(t: usize, clube: usize, id_comp: usize) -> (i32, i32) {
    let c = cqd13_d(cqd13_d(0xadadfc).wrapping_add(4 * id_comp));
    if *((c + 0x42) as *const i8) != 1 {
        (400, 400)
    } else {
        let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        // 0x53d980: 3 pushes; add esp, 0xc -> cdecl
        (if estrangeiro(t, cqd13_d(clube + 0x53), 1) != 0 { 200 } else { 400 }, 200)
    }
}

/// { 0x868c50 } (thiscall, ret 8, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_league_champions_record(clube, comp)`, pelos campeões das ligas no fim delas (25 chamadores, um por liga; 4
/// chamadas até 2002-06-01). Clube ou competição nulos: a caixa (linha 0x21c); a competição sem objeto em [0xadadfc]
/// (+ 4 × id): a caixa (0x224). A reputação (até peso × 500, +750 no máximo); com +0x57 e o bloco +0x69 do clube: o
/// +0x7b sobe peso × 20 (abaixo de 150000 e de +0x73 + 500) e o +0x44 do bloco 25 × (peso + 20) (abaixo de 120000). O
/// limiar dos jogadores: ftol((+0x3e − 1) × +0x3c do objeto × 0,33), em 16 bits. Com técnico (+0xcf): as três notas
/// dele (+0xa peso × 400, +8 peso × k1, +0xc peso × k2 no registro +0x69, se houver) e o histórico (tipo 5: o clube e a
/// competição); o registro dele (0x86f5b0 com criar): +1 em +0x8c e peso × 5000 em +0xb6, +0xca e +0xba. O presidente
/// (+0xbf). Os 50 do elenco (+0xd7) com ficha, válidos, do clube (+0x39) e fora o técnico: a média 2/1 (0x7c1e30 com
/// 6,5) acima do limiar dá o histórico, moral +20 (0x544710) e +100 no contrato (0x4cf5f0) com a renovação; senão moral
/// +10 e +50 no contrato. O histórico do clube (tipo 5); com a mídia ([0xb6b7c8]) e peso acima de 3, o elogio (0x75c390
/// com 1) e, acima de 12, com 3.
unsafe extern "thiscall" fn campeao_da_liga_nos_registros(this: usize, clube: usize, comp: usize) {
    if clube == 0 || comp == 0 {
        return aviso_em(0xa83b78, 0x21c);
    }
    let obj = cqd13_d(cqd13_d(0xadadfc).wrapping_add(4 * cqd13_d(comp)));
    if obj == 0 {
        return aviso_em(0xa83b78, 0x224);
    }
    let peso = cqd13_w(comp + 0x69);
    cqd13_reputacao(clube, peso, true); // cqd13: a reputação do campeão da liga
    if cqd13_d(clube + 0x57) != 0 {
        let b = cqd13_d(clube + 0x69);
        if b != 0 {
            let x = cqd13_i(clube + 0x7b);
            if x < 150000 && x < cqd13_i(clube + 0x73).wrapping_add(500) {
                cqd13_pi(clube + 0x7b, x.wrapping_add((peso as i32).wrapping_mul(20)));
            }
            let y = cqd13_i(b + 0x44);
            if y < 120000 {
                cqd13_pi(b + 0x44, y.wrapping_add(((peso as i32).wrapping_add(20)).wrapping_mul(25)));
            }
        }
    }
    let n = ((cqd13_w(obj + 0x3e) as i32) - 1).wrapping_mul(cqd13_w(obj + 0x3c) as i32);
    let limiar = ftol(n as f64 * f64_em(0x968f78)) as i16; // o ftol fica no word (movsx word depois)
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 5;
    cpq4_por(&mut e, 8, cqd13_i(clube));
    cpq4_por(&mut e, 0xd, cqd13_i(comp));
    let t = cqd13_d(clube + 0xcf);
    if t != 0 {
        let np = cqd13_d(t + 0x69);
        if np != 0 {
            let (k1, k2) = cqd13_notas(t, clube, cqd13_d(comp));
            cqa12_subir(np + 0xa, peso, 400);
            cqa12_subir(np + 8, peso, k1);
            cqa12_subir(np + 0xc, peso, k2);
        }
        cpq4_historico(!CQD13_ENSAIO, this, cqd13_d(t), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        // 0x86f5b0: push 1; push técnico; mov ecx, edi (o this) -> thiscall (ret 8)
        let r = registro(this, cqd13_d(clube + 0xcf), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8c) as *mut i16, cqd13_w(r + 0x8c).wrapping_add(1));
            let v = (peso as i32).wrapping_mul(5000);
            cqd13_pi(r + 0xb6, cqd13_i(r + 0xb6).wrapping_add(v));
            cqd13_pi(r + 0xca, cqd13_i(r + 0xca).wrapping_add(v));
            cqd13_pi(r + 0xba, cqd13_i(r + 0xba).wrapping_add(v));
        }
    }
    let presidente = cqd13_d(clube + 0xbf);
    if presidente != 0 {
        cpq4_historico(!CQD13_ENSAIO, this, cqd13_d(presidente), &e);
    }
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall (ret 0x10, x87); 0x544710: 3
    // pushes; add esp, 0xc -> cdecl; 0x4cf5f0: push clube; push p; mov ecx, [0xae1050] -> thiscall (ret 8)
    for j in 0..50usize {
        let p = cqd13_d(clube + 0xd7 + 4 * j);
        if p == 0 || cqd13_d(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) || cqd13_d(p + 0x39) != clube
            || p == cqd13_d(clube + 0xcf)
        {
            continue;
        }
        let f = media(cqd13_d(0xb74350), cqd13_d(p + 0x61), 2, 1, 6.5);
        if f > limiar as f64 { // cqd13: acima do limiar
            crate::comparacao::ramo("cqd_13 jogador destacado do campeao");
            cpq4_historico(!CQD13_ENSAIO, this, cqd13_d(p), &e);
            moral(p, 0x14, 1); // cqd13: o moral do jogador destacado
            let k = contrato(cqd13_d(0xae1050), p, clube);
            if k == 0 {
                continue;
            }
            crate::comparacao::ramo("cqd_13 contrato do destacado");
            cqd13_moral(k, 100); // cqd13: o contrato do jogador destacado
            cqd13_renovar(clube, p, k);
        } else {
            moral(p, 0xa, 1);
            let k = contrato(cqd13_d(0xae1050), p, clube);
            if k != 0 {
                cqd13_moral(k, 50);
            }
        }
    }
    cqd13_historico_do_clube(this, clube, 5, comp);
    if cqd13_d(0xb6b7c8) != 0 && peso > 3 {
        cqd13_elogio(clube, comp, 1);
        if peso > 12 {
            cqd13_elogio(clube, comp, 3);
        }
    }
}

/// { 0x86a800 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::add_cup_winners_record(
/// clube, jogo, partida)`, pelas finais das copas (29 chamadores; 11 chamadas até 2002-06-01). Nulos: a caixa (linha
/// 0x51e). A reputação (como na liga); com técnico (+0xcf) que tem registro de não jogador (+0x69): as três notas do lado
/// do clube na partida (+0x188/+0x18a/+0x18c, lado 0x625 × visitante; peso × 400, × k1, × k2); o histórico do técnico
/// (tipo 0: o clube e a competição de +0x14 do jogo); o registro dele (0x86f5b0 com criar): +1 em +0x8a e ftol(peso^4 ×
/// 0,5) (pow da CRT 0x945ff0) em +0xb6, +0xca e, na seleção (0x53b900), +0xbe, senão +0xba. O presidente (+0xbf) e os
/// 20 da escalação. Os 50 do elenco enquanto a reputação não passar de peso × 400: com ficha, válidos e fora o técnico,
/// moral +15 (0x544710) e +100 no contrato; com a situação do contrato (+0x4f >> 4) 1-3 ou 5, a renovação. As
/// convocações da competição guardada (0x76d940) acabam (0x774f30); o histórico do clube (tipo 0); com a mídia e peso
/// acima de 10, o elogio (0x75c390 com 1) e, com peso acima de 12 ou reputação abaixo de 2500, com 3.
unsafe extern "thiscall" fn campeao_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return aviso_em(0xa83b78, 0x51e);
    }
    let comp = cqd13_d(jogo + 0x14);
    let peso = cqd13_w(comp + 0x69);
    cqd13_reputacao(clube, peso, false);
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0;
    cpq4_por(&mut e, 8, cqd13_i(clube));
    cpq4_por(&mut e, 0xd, cqd13_i(comp));
    let t = cqd13_d(clube + 0xcf);
    if t != 0 {
        if cqd13_d(t + 0x69) != 0 {
            let (k1, k2) = cqd13_notas(t, clube, cqd13_d(jogo));
            let lado = partida + 0x188 + 0x625 * (cqd13_d(jogo + 0x20) == clube) as usize;
            cqa12_subir(lado, peso, 400);
            cqa12_subir(lado + 2, peso, k1);
            cqa12_subir(lado + 4, peso, k2);
        }
        cpq4_historico(!CQD13_ENSAIO, this, cqd13_d(cqd13_d(clube + 0xcf)), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, cqd13_d(clube + 0xcf), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8a) as *mut i16, cqd13_w(r + 0x8a).wrapping_add(1));
            // fild peso; fld 4,0; _CIpow (base em st1); fmul 0,5; ftol
            let v = ftol(pow_do_jogo(cqd13_w(comp + 0x69) as f64, f64_em(0x968938)) * f64_em(0x968968)); // cqd13: o peso do título
            cqd13_pi(r + 0xb6, cqd13_i(r + 0xb6).wrapping_add(v));
            cqd13_pi(r + 0xca, cqd13_i(r + 0xca).wrapping_add(v));
            if e_selecao(clube) != 0 {
                cqd13_pi(r + 0xbe, cqd13_i(r + 0xbe).wrapping_add(v));
            } else {
                cqd13_pi(r + 0xba, cqd13_i(r + 0xba).wrapping_add(v));
            }
        }
    }
    let presidente = cqd13_d(clube + 0xbf);
    if presidente != 0 {
        cpq4_historico(!CQD13_ENSAIO, this, cqd13_d(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != cqd13_d(jogo + 0x1c)) as usize;
    cqd13_escalacao(this, clube, escalacao, &e);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    for j in 0..50usize {
        if (cqd13_w(comp + 0x69) as i32).wrapping_mul(400) < cqd13_w(clube + 0x80) as i32 {
            break;
        }
        let p = cqd13_d(clube + 0xd7 + 4 * j);
        if p == 0 || cqd13_d(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) || p == cqd13_d(clube + 0xcf) {
            continue;
        }
        crate::comparacao::ramo("cqd_13 elenco do campeao da copa");
        moral(p, 0xf, 1); // cqd13: o moral do jogador do campeão da copa
        let k = contrato(cqd13_d(0xae1050), p, clube);
        if k == 0 {
            continue;
        }
        crate::comparacao::ramo("cqd_13 moral do elenco campeao da copa");
        cqd13_moral(k, 100); // cqd13: o moral do contrato do campeão da copa
        let s = *((k + 0x4f) as *const u8) >> 4;
        if s == 0 || (s > 3 && s != 5) {
            continue;
        }
        cqd13_renovar(clube, p, k);
    }
    if !CQD13_ENSAIO {
        let guardada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
        // 0x76d940 e 0x774f30: push comp; mov ecx, [0xb74318] (a 0x76d940 não lê o ecx; ret 4 nas duas)
        if guardada(cqd13_d(0xb74318), cqd13_d(jogo + 0x14)) != 0 {
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x774f30usize);
            acabar(cqd13_d(0xb74318), cqd13_d(jogo + 0x14));
        }
    }
    cqd13_historico_do_clube(this, clube, 0, comp);
    if cqd13_d(0xb6b7c8) != 0 && peso > 10 {
        cqd13_elogio(clube, comp, 1);
        if cqd13_w(comp + 0x69) > 12 || cqd13_w(clube + 0x80) < 0x9c4 {
            cqd13_elogio(clube, comp, 3);
        }
    }
}

/// O comparador em ensaio dos dois campeões: o registro do técnico criado antes (0x86f5b0 com criar, como a original
/// faz), a cópia em ensaio (sem históricos, notícias, convocações e renovação; os sorteios do moral 0x544710 sem contar)
/// e a original a partir do mesmo estado. Compara: a reputação do clube, o registro do técnico (0xd2 bytes), as notas
/// (as três do lado na partida, ou +8..+0xd do registro de não jogador do técnico), o +0x7b do clube e o +0x44 do bloco
/// +0x69 (na liga), o moral (+0x45 da ficha) e o do contrato (+0x44) dos 50 do elenco e a data estática da 0x54cdb0. O
/// moral dos contratos que a renovação mexeria (a original renova) e o estado do sorteador (as notícias da original
/// sorteiam) não entram na conta; o sorteador volta ao de antes para a original.
unsafe fn cqd13_comparar(nome: &str, this: usize, clube: usize, notas: (usize, usize), liga: bool, original: impl FnOnce(), copia: impl FnOnce()) {
    let mut fixas: Vec<(usize, usize)> = vec![(clube + 0x80, 2), (0xae2470, 8), notas];
    let t = cqd13_d(clube + 0xcf);
    if t != 0 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, t, 1);
        if r != 0 {
            fixas.push((r, 0xd2));
        }
    }
    if liga && cqd13_d(clube + 0x57) != 0 && cqd13_d(clube + 0x69) != 0 {
        fixas.push((clube + 0x7b, 4));
        fixas.push((cqd13_d(clube + 0x69) + 0x44, 4));
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mut contratos = Vec::new();
    for j in 0..50usize {
        let p = cqd13_d(clube + 0xd7 + 4 * j);
        if p == 0 || cqd13_d(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) {
            continue;
        }
        fixas.push((cqd13_d(p + 0x61) + 0x45, 1));
        let k = contrato(cqd13_d(0xae1050), p, clube);
        if k != 0 {
            contratos.push(k);
        }
    }
    let mut todas = fixas.clone();
    todas.extend(contratos.iter().map(|&k| (k + 0x44, 1)));
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO.iter().copied());
    let antes = crate::comparacao::Memoria::ler(&todas);
    (*core::ptr::addr_of_mut!(CQD13_RENOVADOS)).clear();
    CQD13_ENSAIO = true;
    crate::aleatorio::sem_contar(copia);
    CQD13_ENSAIO = false;
    let renovados = core::mem::take(&mut *core::ptr::addr_of_mut!(CQD13_RENOVADOS));
    let mut conta = fixas;
    conta.extend(contratos.iter().filter(|k| !renovados.contains(k)).map(|&k| (k + 0x44, 1)));
    let mr = crate::comparacao::Memoria::ler(&conta);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&conta);
    PLACAR.registrar(mo == mr, || format!("{}(clube {:#x}) renovados {} original={:x?} rust={:x?}", nome, clube, renovados.len(), mo, mr));
}

unsafe extern "thiscall" fn comparar_campeao_da_liga_nos_registros(this: usize, clube: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1576);
    if clube == 0 || comp == 0 || cqd13_d(cqd13_d(0xadadfc).wrapping_add(4 * cqd13_d(comp))) == 0 {
        return o(this, clube, comp);
    }
    let t = cqd13_d(clube + 0xcf);
    let notas = if t != 0 && cqd13_d(t + 0x69) != 0 { (cqd13_d(t + 0x69) + 8, 6) } else { (clube + 0x80, 2) };
    cqd13_comparar("campeao_da_liga_nos_registros", this, clube, notas, true, || o(this, clube, comp),
        || campeao_da_liga_nos_registros(this, clube, comp));
}

unsafe extern "thiscall" fn comparar_campeao_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1577);
    if clube == 0 || jogo == 0 || partida == 0 {
        return o(this, clube, jogo, partida);
    }
    let notas = (partida + 0x188 + 0x625 * (cqd13_d(jogo + 0x20) == clube) as usize, 6);
    cqd13_comparar("campeao_da_copa_nos_registros", this, clube, notas, false, || o(this, clube, jogo, partida),
        || campeao_da_copa_nos_registros(this, clube, jogo, partida));
}

/// { 0x86fc70 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::clear_human_manager_record(pessoa)`, pela
/// aposentadoria do técnico humano (0x6b2b40) e pelo setup (0x82b9xx). Pessoa entre os 16 humanos (id de [0xae2364] − 16
/// até [0xae2364]): o registro dele no vetor +8 (vaga id − [0xae2364] + [0xae2368] + 16, 0xd2 bytes); senão, como a
/// original, a própria pessoa. Zera os 0xd2 bytes e põe −1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e e +0x72.
unsafe extern "thiscall" fn limpar_registro_do_humano(this: usize, pessoa: usize) {
    let id = cqd13_i(pessoa);
    let lim = cqd13_i(0xae2364);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(cqd13_i(0xae2368)).wrapping_add(0x10);
        cqd13_d(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    core::ptr::write_bytes(r as *mut u8, 0, 0xd2);
    for o in [0x86usize, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        cqd13_pi(r + o, -1);
    }
}

/// Comparador: lado a lado pelo registro (os 0xd2 bytes que a rotina zera).
unsafe extern "thiscall" fn comparar_limpar_registro_do_humano(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1578);
    let id = cqd13_i(pessoa);
    let lim = cqd13_i(0xae2364);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(cqd13_i(0xae2368)).wrapping_add(0x10);
        cqd13_d(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, pessoa),
        || limpar_registro_do_humano(this, pessoa));
    PLACAR.registrar(igual, || format!("limpar_registro_do_humano(pessoa {:#x}) {:x?} x {:x?}", pessoa, mo, mr));
}
