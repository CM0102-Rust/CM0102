// ---------- 268a leva: a resposta da IA ao lance e a negociação de empréstimo copiada da oferta (transfer_manager.cpp) ----------

unsafe fn cqe1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqe1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqe1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqe1_pessoa(id: usize) -> usize {
    cqe1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqe1_clube(id: usize) -> usize {
    cqe1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

unsafe fn cqe1_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// A oferta segue ativa (status 2, 3, 4, 0x10, 0x14, ou 0xf com +0x2e ≥ 3): a original pula essas na lista +0.
unsafe fn cqe1_ativa(o: usize) -> bool {
    let s = cqe1_b(o + 0x2c);
    matches!(s, 2 | 3 | 4 | 0x10 | 0x14) || (s == 0xf && cqe1_b(o + 0x2e) >= 3)
}

/// A caixa "Error" com "v%s %s.%s %d" (versão 0x90d130, nome e extensão do .cpp `arquivo` pela 0x944cff, `linha`) em
/// 0xae24d0 (0x944c9f) e a caixa 0x5e8290; [0xb67a34] = 0. As três chamadas cdecl, como na original.
unsafe fn cqe1_caixa(arquivo: usize, linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(arquivo, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    cqe1_pd(0xb67a34, 0);
}

/// { 0x8c8090 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor: a data local da vaga):
/// `TRANSFER_MANAGER::ai_respond_to_bid(vaga, ensaio)`, a resposta do clube vendedor do computador a um lance. O alvo é
/// o +0xc da oferta, o comprador o +8 e o vendedor o clube do contrato do alvo (0x4cf5a0, em [0xae1080]; fora o tipo 5).
/// - Alvo sem ficha de jogador (+0x61): status 0x10 e a oferta na fatia seguinte (0x8c6220: push 0; push 1; push oferta).
/// - Lance de empréstimo (+0x1f = 1): recusado sem olhar mais nada (0x8bfa30: push 0; push 0; push vaga), fora do ensaio.
/// - O vendedor deve aceitar (0x8cd240, stdcall(oferta, 0, 0)): aceito (0x8c0080: push 0; push vaga), fora do ensaio.
/// - Vendedor humano (0x600a50 com 1, 1) com o técnico (0x53ca90) que responde (0x5fd210 em [0xb63c98]): a regra 0 dele
///   (0x5fea50: push técnico; push 0) com troca, ou igual a 1: recusa (0x8bf680: push 0; push 0; push vaga).
/// - O piso (0x4d1600: push 1; push 1; push &motivo; push alvo; push comprador; push vendedor; mov ecx, [0xae1050]) e,
///   fora do ensaio, as outras ofertas da lista +0 que não estão ativas: o valor delas (0x8e7b50 com 0, 0, 1, 0, 1) acima
///   do piso vira o piso e zera o motivo. O total (0x8e7a70 com 1, 0, 0, 1), o pedido (0x8e7b50 com 0, 0, 1, 1, 1) e o
///   valor k (0x8e7b50 com 0, 1, 1, 1, 1). Cada jogador da troca (+4): a palavra da posição (0x5b7d20: o primeiro bit de
///   1, 2, 4, 0x10, 8; senão 0x20 ou 0x40) na vaga local, o valor dele para o vendedor (0x840d80 com −1, 0) até piso − k
///   e, fora do ensaio, se o vendedor não o aceita (0x843800 com 1) a troca fica recusada. O preço é o maior de piso e k.
/// - As opções (o dinheiro 0x8c8740, a troca 0x8c8860, a cláusula 0x8c8fd0): sem nenhuma, recusa com o motivo (0x8bf680
///   com 1). A de maior atração é a melhor; a primeira de valor (0x8e7b50 com 0, 0, 1, 1, 1) × 0,85 abaixo do pedido, sem
///   troca recusada: aceita (fora do ensaio). Sem melhor: a caixa (linha 0x1eee) e a recusa, sem soltar as opções.
/// - Senão: o valor da melhor (0x8e7b50 com 0, 0, 0, 0, 1) × 0,75 abaixo do pedido, tipo (+0x31 & 0xf) fora de 1 e 2 e
///   fora do ensaio: a contraproposta (o nibble baixo de +0x30 da oferta na melhor e 0x8be9d0: push 0; push melhor; push
///   oferta). Senão a recusa (0x8bf680 com 1), com o motivo 10 se o total ficou abaixo de 0,55 do valor do alvo (+0x52)
///   com 2 milhões de diferença, ou de 0,1 com 100 mil (fora do ensaio). As opções são destruídas (0x8e5710) e liberadas.
unsafe extern "thiscall" fn resposta_da_ia_ao_lance(this: usize, vaga: usize, ensaio: u32) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bfa30usize);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x8cd240usize);
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0080usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let regra: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bf680usize);
    let piso_de: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let total_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let valor_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(0x840d80usize);
    let aceita_troca: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32 = core::mem::transmute(0x843800usize);
    let dinheiro: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(0x8c8740usize);
    let troca: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(0x8c8860usize);
    let clausula: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(0x8c8fd0usize);
    let negociar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8be9d0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    // ponytail: a vaga local (esp+0x58) a original deixa com sobra de pilha fora da data (+0x1b), da palavra da posição,
    // do +0xb e dos bits 2..4 e 0..1 de +0x24; a 0x840d80 lê aqui só a posição, o +0x24 (tipo 4) e o +0xf (comparado com
    // o jogador da troca, que não é do vendedor): a cópia zera
    let mut vl = [0u8; 0x28];
    let pv = vl.as_mut_ptr() as usize;
    let mut melhor_atracao: i8 = -1;
    let mut troca_recusada: u32 = 0;
    data_padrao(pv + 0x1b);
    vl[0x24] = (vl[0x24] & 0xe4) | 4;
    cqe1_pd(pv + 0xb, 10000);
    let o = cqe1_d(vaga);
    let alvo = cqe1_pessoa(cqe1_d(o + 0xc));
    let comprador = cqe1_clube(cqe1_d(o + 8));
    let c = contrato_de(cqe1_d(0xae1050), alvo);
    cqe1_pd(0xae1080, c);
    let vendedor = if c == 0 || cqe1_b(c + 0x35) & 0x3f == 5 { 0 } else { cqe1_clube(cqe1_d(c + 4)) };
    if cqe1_d(alvo + 0x61) == 0 {
        cqe1_pb(o + 0x2c, 0x10);
        agendar(this, o, 1, 0);
        return;
    }
    let mut motivo: u32 = 0;
    let mut n: i8 = 0;
    if cqe1_b(o + 0x1f) == 1 {
        if ensaio == 0 {
            recusar_emprestimo(this, vaga, 0, 0);
        }
        return;
    }
    if deve_aceitar(cqe1_d(vaga), 0, 0) != 0 {
        if ensaio == 0 {
            aceitar(this, vaga, 0);
        }
        return;
    }
    if vendedor != 0 && humano(vendedor, 1, 1, 0, 0) != 0 && responde(cqe1_d(0xb63c98), tecnico(vendedor)) != 0 {
        if regra(cqe1_d(0xb63c98), 0, tecnico(vendedor)) != 0 && cqe1_d(o + 4) != 0 {
            recusar(this, vaga, 0, 0);
            return;
        }
        if regra(cqe1_d(0xb63c98), 0, tecnico(vendedor)) == 1 {
            recusar(this, vaga, 0, 0);
            return;
        }
    }
    let mut piso = piso_de(cqe1_d(0xae1050), vendedor, comprador, alvo, &mut motivo, 1, 1);
    let l = cqe1_d(o);
    if l != 0 && ensaio == 0 {
        cqe1_pd(l + 0xa, cqe1_d(l + 2));
        loop {
            let p = cqe1_proximo(cqe1_d(o));
            if p == 0 {
                break;
            }
            let k = cqe1_d(p);
            let outra = cqe1_d(cqe1_d(this).wrapping_add(k.wrapping_mul(4)));
            if k != cqe1_d(o + 0x20) && !cqe1_ativa(outra) {
                let v = valor(outra, 0, 0, 1, 0, 1);
                if v > piso {
                    piso = v;
                    motivo = 0;
                }
            }
        }
    }
    let total = total_de(o, 1, 0, 0, 1);
    let pedido = valor(o, 0, 0, 1, 1, 1);
    let k = valor(o, 0, 1, 1, 1, 1);
    let t = cqe1_d(o + 4);
    if t != 0 {
        cqe1_pd(t + 0xa, cqe1_d(t + 2));
        let teto = piso.wrapping_sub(k);
        let mut p = cqe1_proximo(cqe1_d(o + 4));
        while p != 0 {
            let j = cqe1_pessoa(cqe1_d(p));
            let m = mascara(cqe1_d(j + 0x61));
            let w: u16 = if m & 1 != 0 {
                1
            } else if m & 2 != 0 {
                2
            } else if m & 4 != 0 {
                4
            } else if m & 0x10 != 0 {
                0x10
            } else if m & 8 != 0 {
                8
            } else if m & 0x20 != 0 {
                0x20
            } else {
                0x40
            };
            core::ptr::write_unaligned((pv + 1) as *mut u16, w);
            let mut saida: i32 = 0;
            let mut v = valor_jogador(cqe1_d(cqe1_d(0xde1f54) + 0x856), vendedor, j, pv, &mut saida, -1, 0);
            if v > teto {
                v = teto;
            }
            if ensaio == 0 && aceita_troca(cqe1_d(this + 0x856), j, vendedor, v, 1) == 0 {
                troca_recusada = 1;
            }
            p = cqe1_proximo(cqe1_d(o + 4));
        }
    }
    let preco = if k > piso { // cqe1: o preço pelo valor k
        crate::comparacao::ramo("cqe_1 preço pelo k");
        k
    } else {
        piso
    };
    let mut opcoes = [0usize; 4];
    let mut atracao = [0i8; 4];
    if dinheiro(this, comprador, alvo, &mut opcoes[0], &mut atracao[0], preco) != 0 {
        n = 1;
    }
    if troca(this, comprador, alvo, o, &mut opcoes[n as usize], &mut atracao[n as usize], preco) != 0 {
        n += 1;
    }
    if clausula(this, comprador, alvo, &mut opcoes[n as usize], &mut atracao[n as usize], preco) != 0 {
        n += 1;
    }
    let mut aceito = false;
    let mut melhor: usize = 0;
    if n > 0 {
        let pedido_f = pedido as f64;
        for i in 0..n as usize {
            if atracao[i] > melhor_atracao {
                melhor = opcoes[i];
                melhor_atracao = atracao[i];
            }
            let v = valor(opcoes[i], 0, 0, 1, 1, 1);
            if v as f64 * f64_em(0x969520) < pedido_f && troca_recusada == 0 { // cqe1: a opção × 0,85 abaixo do pedido
                crate::comparacao::ramo("cqe_1 aceita");
                if ensaio == 0 {
                    aceitar(this, vaga, 0);
                }
                aceito = true;
                break;
            }
        }
    }
    if n == 0 {
        recusar(this, vaga, 1, motivo);
        return;
    }
    if melhor == 0 {
        cqe1_caixa(0xa8d514, 0x1eee);
        recusar(this, vaga, 1, motivo);
        return;
    }
    if !aceito {
        let pedido_f = pedido as f64;
        let v = valor(melhor, 0, 0, 0, 0, 1);
        let tipo = cqe1_d(o + 0x31) & 0xf;
        if v as f64 * f64_em(0x969040) < pedido_f && tipo != 1 && tipo != 2 && ensaio == 0 { // cqe1: a contraproposta
            crate::comparacao::ramo("cqe_1 contraproposta");
            cqe1_pb(melhor + 0x30, (cqe1_b(melhor + 0x30) & 0xf0) | (cqe1_b(o + 0x30) & 0xf));
            negociar(this, o, melhor, 0);
        } else {
            let w = cqe1_d(alvo + 0x52) as i32;
            let (tf, wf) = (total as f64, w as f64);
            if (tf < wf * f64_em(0x96a5d8) && w.wrapping_sub(total) > 2000000)
                || (tf < wf * f64_em(0x967880) && w.wrapping_sub(total) > 100000 && ensaio == 0)
            {
                motivo = 10;
            }
            recusar(this, vaga, 1, motivo);
        }
    }
    for i in 0..n as usize {
        let s = opcoes[i];
        if s != 0 {
            destruir(s);
            liberar(s);
        }
    }
}

/// Comparador da 0x8c8090: só a original (aceita, recusa ou contrapropõe, reagenda, mexe nas listas de observação e
/// aloca e solta as opções).
unsafe extern "thiscall" fn comparar_resposta_da_ia_ao_lance(this: usize, vaga: usize, ensaio: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1579)(this, vaga, ensaio) // ponytail: só a original
}

/// { 0x8bee50 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da oferta local): `TRANSFER_MANAGER::
/// add_loan_negotiation(oferta, a2, a3, a4, a5)`, só pela tela de transferências (0x8fb5e0). Um STAFF_CONTRACT local
/// (0x85e410) e uma oferta local (0x8e5680) com os campos da oferta (a data +0x12 pela 0x417c10; o nibble baixo de +0x30
/// e de +0x31; as listas +0 e +4 vazias); o contrato pelo atual (0x85e590: push a5; push a4; push a3; push a2; push
/// clube +8; push alvo +0xc; lea ecx, contrato) entra na lista em this+0x946 (criada se faltar: 0x944cf1 de 0xe bytes e
/// 0x68d020; 0x68d0d0 com 0x50 bytes) e a cópia vira negociação (0x8be9d0: push 0; push &cópia; push oferta). Devolve o
/// +0x20 da cópia (o índice); a cópia é destruída (0x8e5710).
unsafe extern "thiscall" fn negociacao_de_emprestimo(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let pelo_atual: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let negociar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8be9d0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let mut contrato = [0u8; 0x50];
    let pc = contrato.as_mut_ptr() as usize;
    // ponytail: o nibble alto de +0x30 e os bits 4..31 de +0x31 da cópia a original deixa com sobra de pilha; aqui zero
    let mut copia_oferta = [0u8; 0x44];
    let po = copia_oferta.as_mut_ptr() as usize;
    novo_contrato(pc);
    nova_oferta(po);
    let alvo = cqe1_pessoa(cqe1_d(o + 0xc));
    let clube = cqe1_clube(cqe1_d(o + 8));
    cqe1_pd(po + 0xc, cqe1_d(o + 0xc));
    cqe1_pb(po + 0x11, cqe1_b(o + 0x11));
    cqe1_pd(po + 8, cqe1_d(o + 8));
    cqe1_pb(po + 0x10, cqe1_b(o + 0x10));
    cqe1_pd(po, cqe1_d(o));
    cqe1_pd(po + 4, cqe1_d(o + 4));
    let mut tmp = [0u8; 8];
    copia(po + 0x12, tmp.as_mut_ptr() as usize, o + 0x12);
    cqe1_pd(po + 0x1b, cqe1_d(o + 0x1b));
    cqe1_pb(po + 0x1f, cqe1_b(o + 0x1f));
    cqe1_pb(po + 0x1a, cqe1_b(o + 0x1a));
    cqe1_pd(po + 0x24, cqe1_d(o + 0x24));
    cqe1_pb(po + 0x2d, cqe1_b(o + 0x2d));
    cqe1_pd(po + 0x28, cqe1_d(o + 0x28));
    cqe1_pb(po + 0x2e, cqe1_b(o + 0x2e));
    cqe1_pd(po + 0x20, cqe1_d(o + 0x20));
    cqe1_pb(po + 0x2c, cqe1_b(o + 0x2c));
    cqe1_pb(po + 0x2f, cqe1_b(o + 0x2f));
    cqe1_pb(po + 0x30, (cqe1_b(po + 0x30) & 0xf0) | (cqe1_b(o + 0x30) & 0xf));
    cqe1_pd(po + 0x31, (cqe1_d(po + 0x31) & !0xf) | (cqe1_d(o + 0x31) & 0xf));
    cqe1_pd(po + 0x35, cqe1_d(o + 0x35));
    cqe1_pb(po + 0x39, cqe1_b(o + 0x39));
    cqe1_pd(po + 0x3a, cqe1_d(o + 0x3a));
    cqe1_pb(po + 0x40, cqe1_b(o + 0x40));
    cqe1_pb(po + 0x3e, cqe1_b(o + 0x3e));
    cqe1_pb(po + 0x3f, cqe1_b(o + 0x3f));
    cqe1_pd(po + 4, 0);
    cqe1_pd(po, 0);
    pelo_atual(pc, alvo, clube, a2, a3, a4, a5);
    if cqe1_d(this + 0x946) == 0 {
        let m = alocar(0xe);
        let l = if m != 0 { lista_nova(m) } else { 0 };
        cqe1_pd(this + 0x946, l);
    }
    acrescentar(cqe1_d(this + 0x946), pc, 0x50);
    negociar(this, o, po, 0);
    let r = cqe1_d(po + 0x20) as u32;
    destruir(po);
    r
}

/// Comparador da 0x8bee50: só a original (a lista de contratos cresce, a negociação nova é alocada).
unsafe extern "thiscall" fn comparar_negociacao_de_emprestimo(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(1580)(this, o, a2, a3, a4, a5) // ponytail: só a original
}

// ---------- 268a leva: a opção de troca na resposta ao lance e o fim do empréstimo agendado (transfer_manager.cpp, contract_manager.cpp) ----------

unsafe fn cqe2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqe2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqe2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqe2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqe2_pessoa(id: usize) -> usize {
    cqe2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqe2_clube(id: usize) -> usize {
    cqe2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

unsafe fn cqe2_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080], e o
/// clube dele (sem contrato: 0; o tipo 5 conta).
unsafe fn cqe2_clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqe2_d(0xae1050), pessoa);
    cqe2_pd(0xae1080, c);
    if c == 0 { 0 } else { cqe2_clube(cqe2_d(c + 4)) }
}

/// As marcas da nota pelo alcance da reputação (0x5408c0, cdecl(pessoa, técnico do vendedor +0xcf)): 0..4 dão 0x40,
/// 0x50, 0x10, 0x30, 0x20 sobre `m` (as duas tabelas de salto da original, 0x8c8fa4 e 0x8c8fb8); fora disso a original
/// deixa o ebp como estava (`antes`).
unsafe fn cqe2_marcas(pessoa: usize, vendedor: usize, m: u32, antes: u32) -> u32 {
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x5408c0usize);
    match escopo(pessoa, cqe2_d(vendedor + 0xcf)) as i8 {
        0 => m | 0x40,
        1 => m | 0x50,
        2 => m | 0x10,
        3 => m | 0x30,
        4 => m | 0x20,
        _ => antes,
    }
}

/// A nota da pessoa para o vendedor (0x6e2f20, cdecl de 15 dwords: pessoa, +0x61, 0, a posição, as marcas, o vendedor,
/// 0, 0, −1 e seis zeros).
unsafe fn cqe2_nota(pessoa: usize, posicao: u16, marcas: u32, vendedor: usize) -> i32 {
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32 =
        core::mem::transmute(0x6e2f20usize);
    nota(pessoa, cqe2_d(pessoa + 0x61), 0, posicao as u32, marcas, vendedor, 0, 0, -1, 0, 0, 0, 0, 0, 0)
}

/// { 0x8c8860 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da LLIST local): `TRANSFER_MANAGER::
/// make_exchange_option(comprador, alvo, oferta, saida, atracao, valor)`, a opção de troca da resposta da IA ao lance
/// (0x8c8090). O vendedor é o clube do contrato do alvo (0x4cf5a0, em [0xae1080]); a LLIST local (0x68d020/0x68d040). O
/// contrato do alvo com o vendedor (0x4cf5f0). Sem a lista de observação do vendedor (o byte +4 do registro de 9 bytes em
/// [[this+0x856]+0xdc]): 0. Senão o elenco do comprador (0x541f90 com 0: 50 vagas), a oferta nova (0x944cf1 de 0x41
/// bytes e 0x8e5680) em *saida, preparada (0x8e78f0: push valor; push comprador; push alvo; mov ecx, oferta), atração 50,
/// +0x1b = valor e o teto = valor × 1,1 (_ftol). Para o alvo (vaga −1, fora do contrato de estatuto 7) e as 25 vagas da
/// lista do vendedor (o +0xf, fora dos jogadores do próprio vendedor), enquanto a troca não tem 3: a posição (0x5b7d20),
/// as marcas (0x4800, ou 0x4900 fora do estatuto 2 e 3; fora do 5, | 0x80 e o alcance) e a nota base. Depois o próprio e
/// os 50 do elenco com a mesma posição: contrato com o clube atual do candidato (0x4cf5f0) de tipo diferente de 5, no
/// comprador, sem ficha (0x4cf6d0), sem os bits 0x30 de +0xb do registro de 79 bytes, piso (0x4d1600 com 0, 1, 1) até o
/// teto; sem oferta (0x8c5cf0), troca (0x8c5e40 na lista local) nem negociação de troca do alvo (0x8c5fc0), ou só a troca
/// desta oferta (+0x20); o vendedor chega a ele (0x8ce990 com 0, 1); interesse (0x846920: push 0; push estatuto; push 0;
/// push 1; push vendedor; push jogador; mov ecx, [this+0x856]) de pelo menos 3,15; a nota dele / a base acima de 0,9: a
/// atração sobe 20 (nota maior) ou 5, entre 0 e 100; fora da troca (+4, criada se faltar), ele entra (0x68d0d0 com 7
/// bytes: id, 0, estatuto), o valor dele (0x840d80 com −1, 1) sai do teto e de +0x1b (no mínimo 0), e a vaga acaba.
/// Devolve 1.
unsafe extern "thiscall" fn opcao_de_troca(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let preparar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x8e78f0usize);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let piso_de: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x8c5fc0usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(0x846920usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let valor_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(0x840d80usize);
    let vendedor = cqe2_clube_do_contrato(alvo);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let pl = lista.as_mut_ptr() as usize;
    lista_nova(pl);
    let cv = contrato_com(cqe2_d(0xae1050), alvo, vendedor);
    let reg = cqe2_d(cqe2_d(this + 0x856) + 0xdc).wrapping_add(cqe2_d(vendedor).wrapping_mul(9));
    if cqe2_b(reg + 4) == 0 { // cqe2: o vendedor sem lista de observação ativa
        destruir_lista(pl);
        return 0;
    }
    let mut elenco = [0usize; 50];
    reunir(comprador, elenco.as_mut_ptr(), 0);
    let m = alocar(0x41);
    let nova = if m != 0 { nova_oferta(m) } else { 0 };
    *saida = nova;
    preparar(nova, alvo, comprador, valor);
    crate::comparacao::ramo("cqe_2 opção montada");
    *atracao = 0x32; // cqe2: a atração da opção
    cqe2_pd(nova + 0x1b, valor as u32 as usize); // cqe2: o preço da opção
    let mut teto = ftol(valor as f64 * f64_em(0x9689c0));
    // ponytail: a posição, a nota base, as marcas e o estatuto ficam de uma vaga para a outra (a original os deixa na
    // pilha sem valor até a primeira vaga com ficha +0x61; candidato sem ficha não acontece: começam em zero). As marcas
    // fora de 0..4 do alcance (que a 0x5408c0 não devolve) seriam o ebp anterior, de início o ponteiro da oferta.
    let mut posicao: u16 = 0;
    let mut base: i32 = 0;
    let mut marcas: u32 = 0;
    let mut estatuto: u8 = 0;
    let mut ebp: u32 = m as u32;
    let estatuto_da_vaga = |i: i8| (cqe2_b(cqe2_d(reg).wrapping_add((i as i32 * 0x25) as usize) + 0x24) >> 1) & 0xf;
    let mut i: i8 = -1;
    loop {
        let t = cqe2_d(nova + 4);
        if t != 0 && cqe2_w(t) == 3 {
            break;
        }
        'vaga: {
            let cand = if i == -1 {
                if cqe2_b(cv + 0x4f) & 0xf0 == 0x70 {
                    break 'vaga;
                }
                alvo
            } else {
                let id = cqe2_d(cqe2_d(reg).wrapping_add((i as i32 * 0x25) as usize) + 0xf) as i32;
                if id <= -1 { // cqe2: a vaga sem alvo
                    break 'vaga;
                }
                let p = cqe2_pessoa(id as usize);
                if cqe2_d(p + 0x39) == vendedor {
                    break 'vaga;
                }
                crate::comparacao::ramo("cqe_2 vaga com alvo");
                p
            };
            let cc = contrato_com(cqe2_d(0xae1050), cand, cqe2_clube_do_contrato(cand));
            let mut j: i8 = -1;
            loop {
                'jogador: {
                    let x = if j == -1 {
                        if cqe2_d(cand + 0x61) == 0 {
                            break 'jogador;
                        }
                        posicao = mascara(cqe2_d(cand + 0x61));
                        marcas = 0x4800;
                        estatuto = if i == -1 { cqe2_b(cc + 0x4f) >> 4 } else { estatuto_da_vaga(i) };
                        if estatuto != 2 && estatuto != 3 {
                            marcas = 0x4900;
                        }
                        if estatuto == 5 {
                            ebp = marcas;
                        } else {
                            marcas |= 0x80;
                            ebp = cqe2_marcas(cand, vendedor, marcas, ebp);
                        }
                        base = cqe2_nota(cand, posicao, ebp, vendedor);
                        cand
                    } else {
                        let x = elenco[j as usize];
                        if x == 0 || cqe2_d(x) as i32 >= cqe2_d(0xae2364) as i32 || cqe2_d(x + 0x61) == 0 {
                            break 'jogador;
                        }
                        if mascara(cqe2_d(x + 0x61)) != posicao {
                            break 'jogador;
                        }
                        x
                    };
                    let cx = contrato_com(cqe2_d(0xae1050), x, cqe2_clube_do_contrato(cand));
                    if cx == 0 || cqe2_d(x + 0x39) != comprador || ficha(cqe2_d(0xae1050), x) != 0 {
                        break 'jogador;
                    }
                    if cqe2_b(cqe2_d(0xae2d0c).wrapping_add(cqe2_d(x).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 || cqe2_b(cx + 0x35) & 0x3f == 5 {
                        break 'jogador;
                    }
                    if piso_de(cqe2_d(0xae1050), comprador, 0, x, 0, 1, 1) > teto {
                        break 'jogador;
                    }
                    esvaziar(pl);
                    if ofertas(this, x, 0, 0) != 0 || trocas(this, x, pl) != 0 || negociacoes(this, alvo, 0) != 0 {
                        if cqe2_w(pl) != 1 {
                            break 'jogador;
                        }
                        cqe2_pd(pl + 0xa, cqe2_d(pl + 2));
                        let p = cqe2_proximo(pl);
                        if cqe2_d(oferta + 0x20) != cqe2_d(p) {
                            break 'jogador;
                        }
                    }
                    if chegar(this, x, vendedor, 0, 1) == 0 {
                        break 'jogador;
                    }
                    // ponytail: no alvo a original empurra o estatuto com os bytes altos do ecx que a 0x8ce990 deixou (lixo);
                    // nas vagas, com os de 0x25 × vaga; a 0x846920 recebe um char: só o byte conta
                    let st = if i == -1 {
                        (cqe2_b(cc + 0x4f) >> 4) as u32
                    } else {
                        ((i as i32 * 0x25) as u32 & 0xffff_ff00) | estatuto_da_vaga(i) as u32
                    };
                    let r = interesse(cqe2_d(this + 0x856), x, vendedor, 1, 0, st, 0);
                    if !(r >= f64_em(0x968e58)) { // cqe2: o interesse abaixo de 3,15
                        break 'jogador;
                    }
                    crate::comparacao::ramo("cqe_2 interesse");
                    if estatuto == 5 {
                        ebp = marcas;
                    } else {
                        marcas |= 0x80;
                        ebp = cqe2_marcas(x, vendedor, marcas, ebp);
                    }
                    let nx = cqe2_nota(x, posicao, ebp, vendedor);
                    if nx == 0 || base == 0 {
                        break 'jogador;
                    }
                    if !(nx as f64 / base as f64 > f64_em(0x9689d8)) { // cqe2: a nota até 0,9 da base
                        break 'jogador;
                    }
                    crate::comparacao::ramo("cqe_2 nota da troca");
                    let a = if nx > base { (*atracao).wrapping_add(20) } else { (*atracao).wrapping_add(5) };
                    *atracao = if a < 0 { 0 } else if a > 100 { 100 } else { a };
                    if cqe2_d(nova + 4) == 0 {
                        let m = alocar(0xe);
                        let l = if m != 0 { lista_nova(m) } else { 0 };
                        cqe2_pd(nova + 4, l);
                    }
                    let t = cqe2_d(nova + 4);
                    cqe2_pd(t + 0xa, cqe2_d(t + 2));
                    loop {
                        let p = cqe2_proximo(cqe2_d(nova + 4));
                        if p == 0 {
                            break;
                        }
                        if cqe2_d(p) == cqe2_d(x) {
                            break 'jogador;
                        }
                    }
                    // ponytail: o 7º byte do item a original copia da pilha sem valor; aqui zero
                    let mut item = [0u8; 8];
                    cqe2_pd(item.as_mut_ptr() as usize, cqe2_d(x));
                    item[5] = estatuto;
                    acrescentar(cqe2_d(nova + 4), item.as_mut_ptr() as usize, 7);
                    let entrada = cqe2_d(reg).wrapping_add((i as i32 * 0x25) as usize);
                    let mut fora: i32 = 0;
                    let v = valor_jogador(cqe2_d(cqe2_d(0xde1f54) + 0x856), vendedor, x, entrada, &mut fora, -1, 1);
                    crate::comparacao::ramo("cqe_2 jogador na troca");
                    teto = teto.wrapping_sub(v);
                    let r = (cqe2_d(nova + 0x1b) as i32).wrapping_sub(v); // cqe2: o valor dele sai do preço
                    cqe2_pd(nova + 0x1b, if r < 0 { 0 } else { r as u32 as usize });
                    break 'vaga;
                }
                j += 1;
                if j >= 0x32 {
                    break;
                }
            }
        }
        i += 1;
        if i >= 0x19 {
            break;
        }
    }
    destruir_lista(pl);
    1
}

/// A foto da oferta da opção para o comparador: os 0x41 bytes sem o ponteiro +4, sem +0x1a, +0x20..+0x23 e +0x31..+0x34
/// e só o nibble baixo de +0x30 (o que a 0x8e5680 e a 0x8e78f0 não preenchem fica com o que havia na memória nova), e os
/// itens da troca (id, 0, estatuto: o 7º byte vem da pilha da original).
unsafe fn cqe2_foto(o: usize) -> Vec<u8> {
    let mut v: Vec<u8> = (0..0x41).filter(|&k| !(4..8).contains(&k) && k != 0x1a && !(0x20..0x24).contains(&k) && !(0x31..0x35).contains(&k))
        .map(|k| if k == 0x30 { cqe2_b(o + k) & 0xf } else { cqe2_b(o + k) }).collect();
    let l = cqe2_d(o + 4);
    if l != 0 {
        v.extend_from_slice(&cqe2_w(l).to_le_bytes());
        let mut no = cqe2_d(l + 2);
        while no != 0 {
            let item = cqe2_d(no);
            v.extend((0..6).map(|k| cqe2_b(item + k)));
            no = cqe2_d(no + 6);
        }
    }
    v
}

/// Comparador da 0x8c8860: lado a lado a partir do mesmo estado de [0xae1080], *saida, *atracao e do sorteador (a
/// 0x8e78f0 sorteia); a cópia roda sem contar sorteios. Confere a devolução, a atração, [0xae1080] e a foto da oferta
/// nova de cada lado; a da cópia é destruída (0x8e5710) e liberada (0x944c94) e fica tudo como a original deixou.
unsafe extern "thiscall" fn comparar_opcao_de_troca(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32>(1581);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let est = crate::aleatorio::REGIOES_DO_ESTADO;
    let regioes = [(0xae1080usize, 4usize), (saida as usize, 4), (atracao as usize, 1), est[0], est[1]];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let r = o(this, comprador, alvo, oferta, saida, atracao, valor);
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let (ao, fo) = (*atracao, if r != 0 && *saida != 0 { cqe2_foto(*saida) } else { Vec::new() });
    let e_o = cqe2_d(0xae1080);
    antes.escrever();
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| opcao_de_troca(this, comprador, alvo, oferta, saida, atracao, valor)));
    let (an, fn_) = (*atracao, if n != 0 && *saida != 0 { cqe2_foto(*saida) } else { Vec::new() });
    let e_n = cqe2_d(0xae1080);
    if n != 0 && *saida != 0 {
        let s = *saida;
        destruir(s);
        liberar(s);
    }
    depois.escrever();
    PLACAR.registrar(r == n && ao == an && e_o == e_n && fo == fn_, || format!(
        "opcao_de_troca(comprador {:#x}, alvo {:#x}, valor {}): original {} rust {}, atração {} x {}, [0xae1080] {:#x} x {:#x}, oferta {:x?} x {:x?}",
        comprador, alvo, valor, r, n, ao, an, e_o, e_n, fo, fn_));
    r
}

/// A caixa "Error" com "v%s %s.%s %d" (como em cqe1_caixa).
unsafe fn cqe2_caixa(arquivo: usize, linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(arquivo, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    cqe2_pd(0xb67a34, 0);
}

/// O contrato do registro de 79 bytes da pessoa em [0xae2d0c] pelo índice em `campo` (+0 o contrato, +4 o do
/// empréstimo) no vetor de 0x50 bytes em [`vetor`]: id fora do total ([0xae2364]) ou índice negativo, 0.
unsafe fn cqe2_contrato(pessoa: usize, campo: usize, vetor: usize) -> usize {
    let id = cqe2_d(pessoa) as i32;
    if id >= cqe2_d(0xae2364) as i32 {
        return 0;
    }
    let k = cqe2_d(cqe2_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)) + campo) as i32;
    if k < 0 { 0 } else { (k as usize).wrapping_mul(0x50).wrapping_add(cqe2_d(vetor)) }
}

/// { 0x4ccca0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::schedule_loan_end(pessoa, clube)`, só pela
/// tela de transferências (0x8fbcc0). Sem pessoa, id fora do total ou sem o contrato de empréstimo (+4 do registro de 79
/// bytes, no vetor de this): a caixa (linha 0xe4a). Senão o fim (+0x2d) vira amanhã (0x54c5b0 com 1; 0x417c10) e o tipo
/// (+0x35) vira 7 (os bits 3..5 zerados). O contrato atual (+0 do registro, no vetor de [0xae1050]) em [0xae1080]: do
/// `clube`, o clube da pessoa (+0x39) com técnico humano (0x7875e0, stdcall: o ecx = [0xb7431c] não é lido); de outro,
/// o clube do contrato atual (lido de novo duas vezes) com técnico humano: a volta (0x4db0b0: push clube; push pessoa;
/// mov ecx, this).
unsafe extern "thiscall" fn agendar_fim_do_emprestimo(this: usize, pessoa: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let tecnico_humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let voltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4db0b0usize);
    let c = if pessoa == 0 { 0 } else { cqe2_contrato(pessoa, 4, this) };
    if c == 0 {
        cqe2_caixa(0x9a8190, 0xe4a);
        return;
    }
    let mut amanha = [0u32; 2];
    let p = mais(0xae2c90, &mut amanha, 1);
    let mut tmp = [0u8; 8];
    copia(c + 0x2d, tmp.as_mut_ptr() as usize, p);
    cqe2_pb(c + 0x35, (cqe2_b(c + 0x35) & 0xc7) | 7);
    let a = cqe2_contrato(pessoa, 0, cqe2_d(0xae1050));
    cqe2_pd(0xae1080, a);
    let atual = if a == 0 { 0 } else { cqe2_clube(cqe2_d(a + 4)) };
    if clube == atual {
        let x = cqe2_d(pessoa + 0x39);
        if x == 0 || tecnico_humano(x) == 0 {
            return;
        }
    } else {
        let a = cqe2_contrato(pessoa, 0, cqe2_d(0xae1050));
        cqe2_pd(0xae1080, a);
        if a == 0 || cqe2_clube(cqe2_d(a + 4)) == 0 {
            return;
        }
        let a = cqe2_contrato(pessoa, 0, cqe2_d(0xae1050));
        cqe2_pd(0xae1080, a);
        let x = if a == 0 { 0 } else { cqe2_clube(cqe2_d(a + 4)) };
        if tecnico_humano(x) == 0 {
            return;
        }
    }
    voltar(this, pessoa, clube);
}

/// Comparador da 0x4ccca0: só a original (a volta do empréstimo entrega notícias e mexe no elenco).
unsafe extern "thiscall" fn comparar_agendar_fim_do_emprestimo(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1582)(this, pessoa, clube) // ponytail: só a original
}

// ---------- 268a leva: os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação (shortlist_manager.cpp) ----------

unsafe fn cqe3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqe3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqe3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe3_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqe3_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqe3_pessoa(id: usize) -> usize {
    cqe3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqe3_clube(id: usize) -> usize {
    cqe3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080].
unsafe fn cqe3_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqe3_d(0xae1050), pessoa);
    cqe3_pd(0xae1080, c);
    c
}

/// O clube do contrato da pessoa (o tipo 5 conta; sem contrato, 0), com o contrato em [0xae1080].
unsafe fn cqe3_clube_do_contrato(pessoa: usize) -> usize {
    let c = cqe3_contrato(pessoa);
    if c == 0 { 0 } else { cqe3_clube(cqe3_d(c + 4)) }
}

/// A pessoa tem a pessoa no registro de 79 bytes em [0xae2d0c]: o byte +0xb.
unsafe fn cqe3_registro_0b(pessoa: usize) -> u8 {
    cqe3_b(cqe3_d(0xae2d0c).wrapping_add(cqe3_d(pessoa).wrapping_mul(0x4f)) + 0xb)
}

/// { 0x84a020 } (thiscall, ret 4, shortlist_manager.cpp, quadro SEH: STAFF_CONTRACT e oferta locais, só a oferta com
/// destrutor): `Shortlist_Manager::make_club_bids(clube)`, os lances e pedidos de empréstimo da IA, pelo laço do dia
/// (0x5cccd0). Hoje + 7 (0x54c5b0) e um STAFF_CONTRACT local (0x85e410). Seleção (0x53b900), clube humano (0x600a50 com 0,
/// 1), ou humano de férias (com 1, 1) cujo técnico responde (0x5fd210) com a regra 4 (0x5fea50) igual a 1: nada. A lista
/// do clube (o registro de 9 bytes em [this+0xdc]): ativa (+4 = 1), com menos de 100 dias (+8) e finanças (0x5a0590).
/// A conta = vagas com lance de jogador (0x844540 com 0xb) + o elenco (0x53c590 com 0, 0). Cada uma das 40 vagas de
/// 0x25 bytes com alvo (+0xf) e sem oferta (+0x17 = −1), fora as de jogador com a conta em 50: o alvo fora do clube; nas
/// vagas de outro cargo, o cargo 8/9/10 com lugar livre nas listas +0x19f (5), +0x1b3 (7), +0x1cf (3) e o 6/7 sem
/// técnico (+0xd3); sem os bits 0x30 de +0xb do registro de 79 bytes (vaga de jogador), sem transferência futura
/// (0x8cca10), contrato acertado (0x8c5450) nem empréstimo de saída (+0x24 com +0x28 = 5).
/// - Vaga de compra (+0x24 bit 0 apagado): posição (+1) não negativa na de jogador; o clube chega a ele (0x8ce990, ou o
///   motivo 9); sem oferta do clube por ele (0x8cdc90) nem troca (0x8c5e40). O valor = +0xb (dobrado com o bit 0x20, pelo
///   menos 20000); com a liga (+0x57) ativa em [0xadadfc], × 3 ou × 2 pela reputação (+0x80 acima de 0x128e) e a conta
///   abaixo de 25/35, ou × 2 abaixo de 25. Vaga de jogador com substituto (+0x13): sem poder contratar (0x8ced40 com
///   hoje + 7, 1; fora o motivo 9) soma o +0xb (pelo menos 10000); a lesão do substituto (0x62ed70 em [0xb63d28], três
///   vezes) soma dias² × (reputação / 500); com ficha, (1,5 − o byte +0x45 / 30) × valor; sob oferta (0x8cde70) soma
///   max(+0xb × 1,5, 10000). Vaga de jogador com vendedor humano (ou alvo já no clube): o contrato desejado (0x85fc50) 0
///   pula; salário (+0xc) acima de 1,3 × o teto (0x5968d0) pula. Os sorteios (0x90cfc0): sorteio(valor) acima de
///   sorteio(2 × reputação + 1000) faz o lance: +0x24 bit 5 apagado, a oferta local (0x8e5680) preenchida (0x8e5d60);
///   sem oferta, a situação na lista (0x843e10 com 1); com ela, o lance (0x8be300) e, com termos, a oferta de contrato
///   (0x4cc490) e, vendedor da IA sem oferta de contrato (0x4cc9e0) e com contrato do alvo (0x4cf5f0), a negociação de
///   renovação (0x4cda70 com o fator pelo estatuto 1..5 e a entrada do vendedor 0x849ce0) e os termos liberados; +0x17 =
///   o índice da oferta e a conta sobe na de jogador. A oferta local é destruída (0x8e5710).
/// - Vaga de empréstimo (bit 0): fora da Inglaterra ([0x9cf300]) de +0x64 = 3; o empréstimo possível (0x8d0040); vendedor
///   humano sem o bit 2 de +0x4e do contrato pula; sem oferta (0x8cdc90) nem troca; sorteio(+0xb) acima de
///   sorteio(reputação / 10), ou de sorteio(2 × reputação) no segundo par: o pedido (0x842740) em +0x17; negativo, a
///   situação na lista; senão a conta sobe.
unsafe extern "thiscall" fn lances_do_clube(this: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let regra: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let vagas_com_lance: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x844540usize);
    let contar_elenco: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5450usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let tem_oferta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cdc90usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let pode_contratar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(0x8ced40usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let destruir_oferta: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, *mut usize) -> u32 = core::mem::transmute(0x8e5d60usize);
    let fazer_lance: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8be300usize);
    let oferta_de_contrato: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);
    let tem_oferta_de_contrato: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let renovar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cda70usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let pode_emprestar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x8d0040usize);
    let pedir_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x842740usize);
    let mut semana = [0u32; 2];
    mais(0xae2c90, &mut semana, 7);
    let pdata = semana.as_mut_ptr() as usize;
    let mut contrato0 = [0u8; 0x50];
    novo_contrato(contrato0.as_mut_ptr() as usize);
    if e_selecao(clube) != 0 || humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 && responde(cqe3_d(0xb63c98), tecnico(clube)) != 0
        && regra(cqe3_d(0xb63c98), 4, tecnico(clube)) == 1
    {
        return;
    }
    let reg = cqe3_d(this + 0xdc).wrapping_add(cqe3_d(clube).wrapping_mul(9));
    if cqe3_b(reg + 4) != 1 || cqe3_b(reg + 8) >= 100 || financas(cqe3_d(0xae2a38), clube) == 0 {
        return;
    }
    let mut conta = vagas_com_lance(this, clube, 0xb) as i8;
    conta = conta.wrapping_add(contar_elenco(clube, 0, 0) as i8);
    // ponytail: o motivo (palavra local) a original não inicializa; a 0x8ce990 e a 0x8ced40 o gravam antes da leitura
    let mut motivo: u16 = 0;
    let gm = cqe3_d(0xde1f54);
    for s in 0..0x28usize {
        let off = s * 0x25;
        let vaga = |k: usize| cqe3_d(reg).wrapping_add(off) + k;
        let jogador = cqe3_b(vaga(0)) == 0xb;
        if conta >= 0x32 && jogador {
            continue;
        }
        let id = cqe3_d(vaga(0xf)) as i32;
        if id <= -1 || cqe3_d(vaga(0x17)) as i32 != -1 {
            continue;
        }
        let p = cqe3_pessoa(id as usize);
        if cqe3_clube_do_contrato(p) == clube {
            continue;
        }
        if !jogador {
            let livre = |base: usize, n: i8| {
                let mut k: i8 = 0;
                while k < n {
                    if cqe3_d(clube + base + 4 * k as usize) == 0 {
                        break;
                    }
                    k += 1;
                }
                k != n
            };
            match cqe3_b(vaga(0)) {
                8 if !livre(0x19f, 5) => continue,
                9 if !livre(0x1b3, 7) => continue,
                10 if !livre(0x1cf, 3) => continue,
                6 | 7 if cqe3_d(clube + 0xd3) != 0 => continue,
                _ => {}
            }
        }
        if cqe3_registro_0b(p) & 0x30 != 0 && jogador {
            continue;
        }
        if futura(gm, p, 0) != 0 || acertado(gm, p, 0) != 0 {
            continue;
        }
        if cqe3_d(p + 0x24) != 0 && cqe3_b(p + 0x28) == 5 {
            continue;
        }
        if cqe3_b(vaga(0x24)) & 1 == 0 {
            // a vaga de compra
            if cqe3_w(vaga(1)) <= -1 && jogador {
                continue;
            }
            if chegar(gm, p, clube, &mut motivo, 0) == 0 && motivo != 9 {
                continue;
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            let mut v = cqe3_d(vaga(0xb)) as i32;
            if cqe3_b(vaga(0x24)) & 0x20 != 0 {
                v = v.wrapping_shl(1);
                if v <= 0x4e20 {
                    v = 0x4e20;
                }
            }
            let liga = cqe3_d(clube + 0x57);
            if liga != 0 {
                let k = cqe3_d(liga) as i32;
                if k > -1 && k < cqe3_d(0xae2378) as i32 && cqe3_d(cqe3_d(0xadadfc).wrapping_add(k as usize * 4)) != 0 {
                    if cqe3_w(clube + 0x80) > 0x128e {
                        if conta < 0x19 {
                            v = v.wrapping_mul(3);
                        } else if conta < 0x23 {
                            v = v.wrapping_mul(2);
                        }
                    } else if conta < 0x19 {
                        crate::comparacao::ramo("cqe_3 dobro pela liga");
                        v = v.wrapping_mul(2); // cqe3: o dobro pela liga ativa
                    }
                }
            }
            if cqe3_d(vaga(0x13)) as i32 > -1 && cqe3_b(vaga(0)) == 0xb {
                let atual = cqe3_clube_do_contrato(p);
                if pode_contratar(gm, clube, pdata, p, atual, 1, &mut motivo) == 0 && motivo != 9 {
                    let x = cqe3_d(vaga(0xb)) as i32;
                    v = v.wrapping_add(if x > 0x2710 { x } else { 0x2710 });
                }
                let sub = cqe3_pessoa(cqe3_d(vaga(0x13)));
                let les = cqe3_d(0xb63d28);
                if lesao(les, sub) != 0 {
                    let a = cqe3_w(lesao(les, sub) + 0x2c) as i32;
                    let b = cqe3_w(lesao(les, sub) + 0x2c) as i32;
                    v = v.wrapping_add(a.wrapping_mul(b).wrapping_mul(cqe3_w(clube + 0x80) as i32 / 500));
                }
                let f = cqe3_d(sub + 0x61);
                if f != 0 {
                    let x = cqe3_b(f + 0x45) as i8 as f64;
                    v = v.wrapping_add(ftol((f64_em(0x968940) - x * f64_em(0x969020)) * v as f64));
                }
                if sob_oferta(gm, sub, 0) != 0 {
                    let x = cqe3_d(vaga(0xb)) as i32 as f64 * f64_em(0x968940);
                    v = v.wrapping_add(ftol(if x > f64_em(0x968910) { x } else { f64_em(0x968910) }));
                }
            }
            let c = cqe3_contrato(p);
            if c != 0 && cqe3_clube(cqe3_d(c + 4)) != 0 && jogador {
                let vendedor = cqe3_clube_do_contrato(p);
                if humano(vendedor, 0, 1, 0, 0) != 0 || cqe3_d(p + 0x39) == clube {
                    let mut contrato = [0u8; 0x50];
                    let pc = contrato.as_mut_ptr() as usize;
                    novo_contrato(pc);
                    let e = vaga(0);
                    // ponytail: o estatuto vai com os bytes altos do ponteiro da vaga (como na original); o cargo, com os do
                    // edx que a 0x600a50 deixou (lixo): a 0x85fc50 recebe um char, só o byte conta
                    let st = (e as u32 & 0xffff_ff00) | ((cqe3_b(e + 0x24) >> 1) & 0xf) as u32;
                    let r = desejado(pc, p, clube, u32::MAX, st, cqe3_b(e) as u32, 0, 0);
                    if r as f64 == f64_em(0x9689a0) {
                        continue;
                    }
                    // ponytail: o cargo vai com os bytes altos do resultado e do fnstsw; a 0x5968d0 lê só o byte
                    let t = teto(financas(cqe3_d(0xae2a38), clube), p, ((cqe3_b(vaga(0x24)) >> 1) & 0xf) as u32);
                    if (t as f64 * f64_em(0x968de8)) < cqe3_d(pc + 0xc) as i32 as f64 {
                        continue;
                    }
                }
            }
            crate::comparacao::ramo("cqe_3 sorteio do lance");
            let r1 = sorteio((cqe3_w(clube + 0x80) as i32).wrapping_mul(2).wrapping_add(1000)); // cqe3: o sorteio do lance
            let r2 = sorteio(v);
            if r2 <= r1 {
                continue;
            }
            cqe3_pb(vaga(0x24), cqe3_b(vaga(0x24)) & 0xdf);
            let mut oferta = [0u8; 0x44];
            let po = oferta.as_mut_ptr() as usize;
            nova_oferta(po);
            let mut termos: usize = 0;
            if preencher(po, p, clube, vaga(0), cqe3_d(vaga(7)) as u32, 0, &mut termos) == 0 {
                situacao(this, p, clube, 1, (cqe3_b(vaga(0)) != 0xb) as u32);
            } else {
                fazer_lance(gm, po, 0);
                if termos != 0 {
                    // ponytail: o cargo vai com os bytes altos do ponteiro do vetor das vagas (como na original)
                    let cargo = (cqe3_d(reg) as u32 & 0xffff_ff00) | cqe3_b(vaga(0)) as u32;
                    oferta_de_contrato(cqe3_d(0xae1050), termos, cqe3_d(po + 0x20) as i32, cqe3_d(po + 0x24) as u32, cargo, 0, 1, 0);
                    let c = cqe3_contrato(p);
                    let vendedor = if c == 0 { 0 } else { cqe3_clube(cqe3_d(c + 4)) };
                    if vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0
                        && tem_oferta_de_contrato(cqe3_d(0xae1050), cqe3_d(p) as i32, cqe3_d(vendedor) as i32) == 0
                    {
                        let cv = contrato_com(cqe3_d(0xae1050), p, vendedor);
                        if cv != 0 {
                            let e = (cqe3_b(cv + 0x4f) >> 4) as i8;
                            let fator = match e {
                                1 | 2 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = cqe3_w(clube + 0x80);
                                    let div = if rep < 1 { 1 } else { rep as i32 };
                                    let f = (cqe3_registro_0b(p) & 0xf) as f64 * f64_em(0x967880) + rep as i32 as f64 / div as f64;
                                    Some(if f64_em(0x967890) <= f { if f64_em(0x968dc0) < f { 1.4 } else { f } } else { 1.0 })
                                }
                                3 | 4 | 5 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = cqe3_w(clube + 0x80) as i32;
                                    let div = if rep.wrapping_mul(2) >= 1 { rep.wrapping_mul(2) } else { 1 };
                                    let f = (cqe3_registro_0b(p) & 0xf) as f64 * f64_em(0x967880) + rep as f64 / div as f64 + f64_em(0x968968);
                                    Some(if f64_em(0x967890) <= f { if f64_em(0x968978) < f { 1.2 } else { f } } else { 1.0 })
                                }
                                _ => None,
                            };
                            if let Some(f) = fator {
                                let b = f.to_bits();
                                renovar(cqe3_d(0xae1050), p, 0, 0, u32::MAX, b as u32, (b >> 32) as u32, 2);
                            }
                        }
                    }
                    liberar(termos);
                }
                cqe3_pd(vaga(0x17), cqe3_d(po + 0x20));
                if jogador {
                    conta = conta.wrapping_add(1);
                }
            }
            destruir_oferta(po);
        } else {
            // a vaga de empréstimo
            let n = cqe3_d(clube + 0x53);
            if n != 0 && cqe3_d(n) == cqe3_d(0x9cf300) && cqe3_b(clube + 0x64) == 3 {
                continue;
            }
            if pode_emprestar(gm, clube, p, 0) == 0 {
                continue;
            }
            let c = cqe3_contrato(p);
            if c != 0 && cqe3_clube(cqe3_d(c + 4)) != 0 && humano(cqe3_clube_do_contrato(p), 0, 1, 0, 0) != 0 {
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                if cqe3_b(contrato(cqe3_d(0xae1050), p) + 0x4e) & 2 == 0 {
                    continue;
                }
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            crate::comparacao::ramo("cqe_3 sorteio do empréstimo");
            let r1 = sorteio(cqe3_w(clube + 0x80) as i32 / 10); // cqe3: o sorteio do empréstimo
            let r2 = sorteio(cqe3_d(vaga(0xb)) as i32);
            if r2 <= r1 {
                let r1 = sorteio((cqe3_w(clube + 0x80) as i32).wrapping_shl(1));
                let r2 = sorteio(cqe3_d(vaga(0xb)) as i32);
                if r2 <= r1 {
                    continue;
                }
            }
            let k = pedir_emprestimo(this, p, clube);
            cqe3_pd(vaga(0x17), k as u32 as usize);
            if cqe3_d(vaga(0x17)) as i32 > -1 {
                conta = conta.wrapping_add(1);
            } else {
                // ponytail: o setne vai com os bytes altos do ponteiro da vaga (como na original); a 0x843e10 lê o byte
                let e = vaga(0);
                situacao(this, p, clube, 1, (e as u32 & 0xffff_ff00) | (cqe3_b(e) != 0xb) as u32);
            }
        }
    }
}

/// Comparador da 0x84a020: só a original (sorteia, faz lances e pedidos de empréstimo, aloca ofertas e termos).
unsafe extern "thiscall" fn comparar_lances_do_clube(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1583)(this, clube) // ponytail: só a original
}

// ---------- 268a leva: a vontade de uma pessoa de ir para um clube (ou de ficar) e o construtor do contrato (shortlist_manager.cpp, staff_contracts.cpp) ----------

unsafe fn cqe4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqe4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe4_c(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqe4_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqe4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// Uma constante f64 do exe.
unsafe fn cqe4_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Uma constante f32 do exe.
unsafe fn cqe4_kf(a: usize) -> f32 {
    core::ptr::read_unaligned(a as *const f32)
}

/// A conta dos dias entre o ano e o dia de hoje ([0xae2c92], [0xae2c90]) e os de uma data (ano em `ano`, dia em
/// `dia`), como a original: (ano − ano de hoje) × 365 − dia de hoje + dia.
unsafe fn cqe4_dias_para(ano: usize, dia: usize) -> i32 {
    ((cqe4_w(ano) as i32 - cqe4_w(0xae2c92) as i32) * 365 - cqe4_h(0xae2c90) as i32) + cqe4_h(dia) as i32
}

/// Hoje menos 150 dias (0x54c8e0: push 0x96; push saída; mov ecx, 0xae2c90 -> thiscall, ret 8) já passou da data
/// guardada em +0x3e (dia) / +0x40 (ano) do técnico `t`: ano maior (sem sinal), ou o mesmo ano e o dia maior (com
/// sinal).
unsafe fn cqe4_depois_de(t: usize) -> bool {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(0x54c8e0usize);
    let mut d = [0u8; 8];
    menos(0xae2c90, &mut d, 0x96);
    let dia = i16::from_le_bytes([d[0], d[1]]);
    let ano = u16::from_le_bytes([d[2], d[3]]);
    let ano_t = cqe4_w(t + 0x40);
    ano > ano_t || (ano == ano_t && dia > cqe4_h(t + 0x3e))
}

/// A distância entre as cidades `a` e `b` (coordenadas f64 em +0x23 e +0x2b), × 55.
unsafe fn cqe4_distancia(a: usize, b: usize) -> f64 {
    let dx = cqe4_k(a + 0x2b) - cqe4_k(b + 0x2b);
    let dy = cqe4_k(a + 0x23) - cqe4_k(b + 0x23);
    (dy * dy + dx * dx).sqrt() * cqe4_k(0x96d1b8)
}

/// O máximo da original nas contas de reputação: `x` se ele passa `y`, senão `y`.
fn cqe4_max(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

/// A média das três reputações da ficha do jogador (+9, +0xb, +0xd), truncada.
unsafe fn cqe4_media3(f: usize) -> i32 {
    (cqe4_h(f + 0xd) as i32 + cqe4_h(f + 0xb) as i32 + cqe4_h(f + 9) as i32) / 3
}

/// A reputação do clube pelo estádio: sem estádio ou sem cidade (+0x69, +0x38), trunc(rep × 2/7); com a cidade de
/// importância 0 (+0x33), rep / 2; senão o maior entre rep × 2/7 e rep × importância / 15, truncado.
unsafe fn cqe4_pelo_estadio(clube: usize) -> i32 {
    let rep = cqe4_h(clube + 0x80) as i32;
    let e = cqe4_d(clube + 0x69);
    let cidade = if e != 0 { cqe4_d(e + 0x38) } else { 0 };
    if cidade == 0 {
        return ftol(rep as f64 * cqe4_k(0x96d5e0));
    }
    let imp = cqe4_c(cidade + 0x33);
    if imp == 0 {
        return rep / 2;
    }
    let q = (imp as i32).wrapping_mul(rep) / 15;
    let x = rep as f64 * cqe4_k(0x96d5e0);
    ftol(cqe4_max(x, q as f64))
}

/// A reputação do clube pela divisão (+0x57, o +0x69 dela): sem divisão, a própria (o resto do registrador vem do
/// ponteiro; só a palavra conta); com reputação de 7500 e público acima de 35000 (+0x73), o maior entre a reputação e
/// 500 × a da divisão; senão o maior entre rep/2 + 250 × a da divisão e 500 × a da divisão.
unsafe fn cqe4_pela_divisao(clube: usize) -> i16 {
    let rep = cqe4_h(clube + 0x80) as i32;
    let comp = cqe4_d(clube + 0x57);
    if comp == 0 {
        return rep as i16;
    }
    let cv = cqe4_h(comp + 0x69) as i32;
    let t = cv.wrapping_mul(500);
    if rep >= 0x1d4c && cqe4_d(clube + 0x73) as i32 > 0x88b8 {
        (if t > rep { t } else { rep }) as i16
    } else {
        let h = rep / 2 + cv.wrapping_mul(250);
        (if t > h { t } else { h }) as i16
    }
}

/// A reputação pela competição do clube (+0x1db; a da lista [0xae2d14] + id × 6 quando ela é negativa ou é a
/// [0x9cf6f8] com reputação de 5000): o registro de [0xae23d0] (0x6b bytes; 15000 para a [0x9cf6e8], senão 600 × +0x69
/// em 16 bits) e, com o clube na tabela dela (0x4a4850) em 1º a 16º sem a marca +4, (1,32 − 0,02 × posição) × isso.
/// Sem registro: o mínimo entre 0,8 × `pela_divisao` e 7000. Devolve (valor, registro).
unsafe fn cqe4_pela_competicao(clube: usize, pela_divisao: i16) -> (i32, usize) {
    let rep = cqe4_h(clube + 0x80);
    let mut idx = cqe4_d(clube + 0x1db) as i32;
    let mut tem = true;
    if idx <= -1 || (idx == cqe4_d(0x9cf6f8) as i32 && rep >= 0x1388) {
        idx = cqe4_d(cqe4_d(0xae2d14).wrapping_add(cqe4_d(clube).wrapping_mul(6))) as i32;
        if idx <= -1 {
            tem = false;
        }
    }
    let mut reg = 0usize;
    if tem {
        reg = cqe4_d(0xae23d0).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            let id = cqe4_d(reg);
            let s: i16 = if id == cqe4_d(0x9cf6e8) { 15000 } else { cqe4_h(reg + 0x69).wrapping_mul(600) };
            // 0x4a4850: push clube; mov ecx, [[0xadadfc] + 4 × id] + 8 -> thiscall, ret 4
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
            let r = achar(cqe4_d(cqe4_d(cqe4_d(0xadadfc).wrapping_add(id.wrapping_mul(4))) + 8), clube);
            if r != 0 && cqe4_b(r + 4) == 0 && (1..=0x10).contains(&cqe4_b(r + 0x72)) {
                let p = cqe4_b(r + 0x72) as f64;
                return (ftol((cqe4_k(0x96af38) - p * cqe4_k(0x968a98)) * s as f64), reg);
            }
            return (s as i32, reg);
        }
    }
    let mut x = pela_divisao as f64 * cqe4_k(0x9689b0);
    if !(x < cqe4_k(0x96d470)) {
        x = cqe4_k(0x96d470);
    }
    (ftol(x), reg)
}

/// A reputação vista pelo técnico ([0xb67a44] + id × 0x49: (+0xa + +8) × 0,25 + +6 × 0,5): abaixo de 5000, o maior
/// entre r / (5 − 0,0008 v) e r − 1000 × (5 − 0,0008 v); senão o menor entre r × (0,5 + 0,0001 v) e r + 1000 × isso.
unsafe fn cqe4_pelo_tecnico(clube: usize, r: i16) -> i32 {
    let t = cqe4_d(cqe4_d(0xb67a44)).wrapping_add(cqe4_d(clube).wrapping_mul(0x49));
    let v = ftol((cqe4_h(t + 0xa) as f64 + cqe4_h(t + 8) as f64) * cqe4_k(0x9689e0) + cqe4_h(t + 6) as f64 * cqe4_k(0x968968)) as i16;
    let dd = r as f64;
    if v < 0x1388 {
        let t = cqe4_k(0x9688a0) - v as f64 * cqe4_k(0x96a618);
        let q1 = dd / t;
        let m = dd - t * cqe4_k(0x968908);
        ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * cqe4_k(0x96a5c0) + cqe4_k(0x968968);
        let q1 = t * dd;
        let m = t * cqe4_k(0x968908) + dd;
        ftol(if q1 < m { q1 } else { m })
    }
}

/// O teto da reputação pelas finanças (0x5a0590 e 0x5986b0): faixa 1, o maior entre 1,25 r e r + 750; faixa 2, o maior
/// entre 1,5 r e r + 1500.
fn cqe4_pela_faixa(faixa: i8, r: i16) -> i16 {
    match faixa {
        1 => ftol(cqe4_max(r as f64 * 1.25, (r as i32 + 0x2ee) as f64)) as i16,
        2 => ftol(cqe4_max(r as f64 * 1.5, (r as i32 + 0x5dc) as f64)) as i16,
        _ => r,
    }
}

/// min(h / 2, h − 2000) sobre a palavra, a correção pelo +0x64 do clube (diferente de 1; e mais uma vez com 3).
fn cqe4_metade(h: i16) -> i16 {
    let a = h as i32 / 2;
    let e = h as i32 - 2000;
    (if a < e { a } else { e }) as i16
}

/// A fluência: max(F × 0,1, L / (12000 − 200 F)) + 0,2, com L = (s + 5000)² × 1/15000 e F a fluência da pessoa no
/// idioma da nação do clube (0x7c6830: push 1; push idioma; push clube; push pessoa; mov ecx, [0xb74350] -> thiscall,
/// ret 0x10; o idioma vai num registrador com lixo acima, a cópia passa só o byte). Devolve (fator, F).
unsafe fn cqe4_fluencia(p1: usize, clube: usize, s: i16) -> (f64, f64) {
    let q = (s as i32 + 0x1388) as f64;
    let l = q * cqe4_k(0x968d78) * q;
    let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x7c6830usize);
    let f = fluencia(cqe4_d(0xb74350), p1, clube, cqe4_b(cqe4_d(clube + 0x53) + 0x77) as u32, 1) as i8 as i32 as f64;
    let a = f * cqe4_k(0x967880);
    let b = l / (cqe4_k(0x96f938) - f * cqe4_k(0x96a648));
    (cqe4_max(a, b) + cqe4_k(0x968918), f)
}

/// { 0x846920 } (thiscall, ret 0x18, shortlist_manager.cpp): `Shortlist_Manager::staff_interest_in_joining_club(pessoa,
/// clube, salário, empréstimo, situação, motivos)`: a nota de vontade da pessoa de ir para o clube (ou de ficar, quando
/// o clube é o dela), x87 transcrito em f64 na ordem da original. Os motivos (bits) só são marcados com o ponteiro. A
/// nota parte de 2,75 (3,5 para o próprio clube; com contrato acabando, 1,1 − dias × 0,002; sem clube, a idade fora do
/// clube), e passa pelos fatores: reputação dos clubes no empréstimo, gostar ou não do clube e do técnico, rivais, as
/// nações e a distância, a moral (+0x44), a situação no elenco (+0x4f) e a pedida, a reputação comparada (a do clube
/// novo e a do atual, pela divisão, pela competição, pelo técnico e pelas finanças), a fluência no idioma, os jogos
/// de seleção e o tempo sem clube; e, pedido o salário (salário ≠ 0, sem empréstimo e nota de 1,75), o salário padrão
/// (0x864550 num STAFF_CONTRACT local) contra o teto do clube (0x5968d0). O resto das contas está nos comentários.
unsafe extern "thiscall" fn cqe4_interesse(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    use crate::comparacao::ramo;
    let k = |a: usize| cqe4_k(a);
    let zero = k(0x9689a0);
    // as chamadas por endereço, com as convenções conferidas nos pontos de chamada da original
    // 0x54a110: lea ecx, data -> thiscall, ret 0
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // 0x8cca10: push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    // 0x4cf5f0: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    // 0x54cdb0: push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    // 0x53d980: push 1; push nação; push pessoa; add esp, 0xc -> cdecl
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    // 0x544590 / 0x544520 / 0x5444a0: push clube; push pessoa; add esp, 8 -> cdecl
    let na_lista_emp: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let na_lista_transf: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let na_lista: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    // 0x547d30 / 0x547d60: push clube; push pessoa; add esp, 8 -> cdecl
    let nao_gosta_do_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let gosta_do_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    // 0x547810 / 0x547840: push pessoa; push técnico; add esp, 8 -> cdecl
    let gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    // 0x4b02e0: push atual; push clube; add esp, 8 -> cdecl
    let rival: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
    // 0x4ca470: push contrato; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    // 0x600a50: push 0; push 0; push 1 (ou 1); push 0 (ou 1); push clube; add esp, 0x14 -> cdecl
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    // 0x5407e0: push 1; push pessoa; push técnico; add esp, 0xc -> cdecl (só a palavra conta)
    let vista: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
    // 0x5a0590: push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0: mov ecx, finanças -> thiscall, ret 0
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    // 0x4cbb60: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    // 0x669bd0: push &data b; push &data a; push clube; add esp, 0xc -> cdecl
    let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);

    let mut data_a = [0u8; 8];
    let mut data_b = [0u8; 8];
    data_padrao(data_a.as_mut_ptr() as usize);
    data_padrao(data_b.as_mut_ptr() as usize);
    let p4b = p4 as u8 != 0;
    let p5b = p5 as u8 as i8;
    let mut l80 = 1.0f64; // [base+0x84]
    let mut l8c = 0usize; // [base+0x78]: o registro da competição do clube atual
    let mut l84 = 0usize; // [base+0x80]: o registro da competição do clube novo
    let mut ad: i8 = 0; // [base+0x57]
    let fl = move |b: u32| {
        if !p6.is_null() {
            *p6 |= b;
        }
    };
    if !p6.is_null() {
        *p6 = 0;
    }
    if cqe4_d(p1 + 0x61) == 0 || cqe4_d(p2 + 0x53) == 0 || cqe4_d(p1 + 0x1a) == 0 {
        return zero;
    }
    if futura(cqe4_d(0xde1f54), p1, 0) != 0 {
        return zero;
    }
    if cqe4_b(cqe4_d(0xae2d0c).wrapping_add(cqe4_d(p1).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return zero;
    }
    let a57 = cqe4_c(p1 + 0x57) as i32 as f64;
    let idade = cqe4_c(p1 + 0x18);
    let aa8: f32 = ((k(0x968dc0) - a57 * k(0x967878)) - cqe4_c(p1 + 0x5b) as i32 as f64 * k(0x967878)) as f32;
    let mut a88: f32 = (a57 * k(0x968a98) + k(0x9689b0)) as f32;
    let mut b4: f32 = (a57 * k(0x967888) + k(0x968968)) as f32;
    let c0 = contrato_de(cqe4_d(0xae1050), p1);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c0);
    let mut d4 = 0usize;
    if c0 != 0 {
        d4 = cqe4_d(0xae23b4).wrapping_add(cqe4_d(c0 + 4).wrapping_mul(0x245));
        if d4 != 0 {
            if cqe4_d(d4 + 0x53) == 0 {
                return k(0x968e38);
            }
            if cqe4_b(contrato_de(cqe4_d(0xae1050), p1) + 0x35) & 0x3f == 5 {
                d4 = 0;
            }
        }
    }
    let c = contrato_com(cqe4_d(0xae1050), p1, d4);
    let a0: u32 = if c != 0 { cqe4_d(c + 0x45) as u32 } else { 0 };
    let r61 = cqe4_d(p1 + 0x61);
    // o próprio clube com contrato: fica (3,5) quando o contrato não acaba logo e a reputação dele não passa a do clube
    if p2 != 0 && p2 == d4 && c != 0 {
        let dias = dias_ate(c + 0x2d, cqe4_d(0xae2c90) as u32, cqe4_d(0xae2c94) as u32);
        let r1 = cqe4_h(r61 + 0xb) as i32;
        let mut s = ftol((cqe4_h(p2 + 0x80) as i32 - r1) as f64 * k(0x968968)) as i16;
        if a0 & 0x10 != 0 {
            s = ftol(s as f64 * k(0x968968)) as i16;
        }
        let lim = cqe4_c(p1 + 0x59) as i32 * 10 + 0x465;
        let s: i32 = if s < 0 { 0 } else if s as i32 > lim { lim as i16 as i32 } else { s as i32 };
        if a0 & 0x161a8 == 0 {
            let id = cqe4_d(p1) as i32;
            if dias < id % 0x23 + 0x14
                && cqe4_b(cqe4_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)) + 0xb) & 0xf == 0
                && r1 + cqe4_c(p1 + 0x57) as i32 * 20 < (cqe4_c(p1 + 0x59) as i32 + 0xaa) * 25 + s
            {
                return k(0x968e38);
            }
        }
    }
    let mut f8: f64;
    if p2 == d4 {
        if c != 0 && a0 != 0 && a0 != 2 {
            return zero;
        }
        f8 = 3.5;
    } else {
        if c != 0 && d4 != 0 && cqe4_d(d4 + 0x53) != 0 && cqe4_d(p2 + 0x53) != 0
            && cqe4_b(cqe4_d(p2 + 0x53) + 0x77) == cqe4_b(cqe4_d(d4 + 0x53) + 0x77) && a0 & 0x20 != 0
        {
            return zero;
        }
        if idade < 0x10 && estrangeira(p1, cqe4_d(p2 + 0x53), 1) != 0 {
            return zero;
        }
        ramo("cqe_4 base de outro clube");
        f8 = 2.75; // cqe4: a base para outro clube
        if c != 0 && cqe4_b(c + 0x49) != 0 {
            let dd = cqe4_dias_para(c + 0x2f, c + 0x2d);
            let v = k(0x9689c0) - dd as f64 * k(0x969048);
            if !p6.is_null() && v < cqe4_c(p1 + 0x5a) as i32 as f64 * k(0x968a98) + k(0x968e48) {
                *p6 |= 1;
            }
            f8 = v * k(0x9690a8);
        } else if d4 == 0 {
            let dd =((cqe4_w(0xae2c92) as i32 - cqe4_w(p1 + 0x40) as i32) * 365 - cqe4_h(p1 + 0x3e) as i32) + cqe4_h(0xae2c90) as i32;
            let mut x = dd.wrapping_mul(dd) as f64 * k(0x96f9b8);
            if k(0x96d6a0) < x {
                x = k(0x96d6a0);
            }
            f8 = x + k(0x9690a8);
        }
    }
    // a reputação dos dois clubes num pedido de empréstimo
    if p4b && r61 != 0 && cqe4_h(r61 + 0xd) < 0x128e && p2 != 0 && d4 != 0 && p2 != d4 && a0 & 0x10 != 0 {
        let ax = cqe4_h(p2 + 0x80);
        let mesmo = cqe4_d(p2 + 0x53) == cqe4_d(d4 + 0x53);
        let rb = cqe4_h(r61 + 0xb);
        if ax > cqe4_h(d4 + 0x80) && (mesmo || ax > 0x1676) {
            if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x968940);
            } else {
                f8 *= k(0x968928);
            }
        } else if ax > 0xabe && (mesmo || ax > 0x1676) && ax > rb {
            if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x969588);
            } else {
                f8 *= k(0x968980);
            }
        } else if ax > 0x109a && mesmo {
            if rb > 0x186a && ax < 0x1482 {
                f8 *= k(0x968fa0);
            } else if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x968928);
            } else {
                f8 *= k(0x9689c0);
            }
        }
    }
    if nao_gosta_do_clube(p1, p2) != 0 {
        f8 *= k(0x968f78);
        fl(2);
    }
    if nao_gosta_do_clube(p1, d4) != 0 {
        let s = cqe4_b(p1 + 0x5a).wrapping_add(cqe4_b(p1 + 0x59)).wrapping_add(cqe4_b(p1 + 0x5b)) as i8;
        if s > 0x2d {
            f8 *= k(0x968928);
        } else if s > 0x19 {
            f8 *= k(0x968940);
        } else {
            f8 = f8 + f8;
        }
        fl(4);
    }
    if gosta_do_clube(p1, d4) != 0 && na_lista_transf(p1, d4) == 0 && cqe4_b(c + 0x49) == 0 {
        f8 *= k(0x968968);
        fl(8);
    }
    if gosta_do_clube(p1, p2) != 0 {
        fl(0x10);
        f8 *= k(0x968930);
    }
    if idade > 0x22 && d4 != p2 && cqe4_d(p2 + 0x53) != cqe4_d(p1 + 0x1a) {
        f8 *= k(0x969040);
    }
    let max1 = |n: i8| if n < 1 { 1 } else { n as i32 };
    if d4 != 0 && d4 != p2 {
        let (e1, e2) = (cqe4_d(0x9d0118), cqe4_d(0x9d011c));
        let i2 = cqe4_d(p2);
        let i4 = cqe4_d(d4);
        if (i2 == e1 || i2 == e2) && (i4 == e1 || i4 == e2) {
            f8 = (k(0x967878) / max1(cqe4_c(p1 + 0x59)) as f64) * f8;
            fl(0x20);
        } else {
            let n2 = cqe4_d(cqe4_d(p2 + 0x53));
            let (na, nb) = (cqe4_d(0x9cf2ac), cqe4_d(0x9cf51c));
            if n2 == nb && cqe4_d(cqe4_d(d4 + 0x53)) == na && cqe4_d(cqe4_d(p1 + 0x1a)) == na {
                f8 *= k(0x967888);
                fl(2);
            } else if n2 == na && cqe4_d(cqe4_d(d4 + 0x53)) == nb && cqe4_d(cqe4_d(p1 + 0x1a)) == nb {
                f8 *= k(0x967880);
                fl(2);
            } else {
                if p2 == cqe4_d(d4 + 0xb3) || p2 == cqe4_d(d4 + 0xb7) || p2 == cqe4_d(d4 + 0xbb) {
                    let m = if n2 == cqe4_d(0x9cf360) {
                        k(0x967890) - cqe4_c(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x968918) / max1(cqe4_c(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
                if rival(p2, d4) != 0 {
                    let m = if cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf360) {
                        k(0x967890) - cqe4_c(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x969040) / max1(cqe4_c(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
            }
        }
    }
    let t2 = cqe4_d(p2 + 0xcf);
    if gosta_de(t2, p1) != 0 {
        f8 = f8 + f8;
        fl(0x40);
    }
    if nao_gosta_de(t2, p1) != 0 {
        f8 *= k(0x968f78);
        fl(0x80);
    }
    if d4 != 0 {
        let t4 = cqe4_d(d4 + 0xcf);
        if nao_gosta_de(t4, p1) != 0 {
            f8 = f8 + f8;
            fl(0x100);
        }
        if gosta_de(t4, p1) != 0 {
            f8 *= k(0x968968);
            if !p6.is_null() && cqe4_b(c + 0x4e) & 8 == 0 {
                *p6 |= 0x200;
            }
        }
    }
    // a moral (+0x44), a situação no elenco (ea) e a pedida do contrato (c1 = +0x3d)
    let mut ea: i8;
    let c1: i8;
    if c == 0 {
        if p2 == d4 {
            c1 = 0;
            ea = p5b;
        } else {
            ea = 7;
            c1 = -100;
        }
    } else {
        let e44 = cqe4_c(c + 0x44) as i32;
        if p2 == d4 {
            f8 = (e44 as f64 * k(0x969028) + k(0x967890)) * f8;
            ea = p5b;
            if cqe4_d(p2 + 0xcf) != 0 && cqe4_depois_de(cqe4_d(p2 + 0xcf)) {
                ea = categoria(cqe4_d(0xae1050), p1, c) as i8;
            }
        } else {
            let v = k(0x967890) - e44 as f64 * k(0x969028);
            f8 = v * f8;
            if v > k(0x96afa8) {
                f8 = v * f8;
            }
            if a0 & 0x10 != 0 {
                a88 = (a88 as f64 * k(0x968940)) as f32;
            }
            if a0 & 0x100 != 0 {
                a88 = (a88 as f64 * k(0x968940)) as f32;
            }
            if a0 & 1 != 0 {
                b4 = (b4 as f64 * k(0x968940)) as f32;
            }
            if !p6.is_null() {
                if v > k(0x968940) && cqe4_b(c + 0x4e) & 8 == 0 {
                    *p6 |= 0x400;
                } else if v < k(0x968968) {
                    *p6 |= 0x800;
                }
            }
            if d4 != 0 && cqe4_d(d4 + 0xcf) != 0 && cqe4_depois_de(cqe4_d(d4 + 0xcf)) {
                ea = categoria(cqe4_d(0xae1050), p1, c) as i8;
            } else {
                ea = (cqe4_b(c + 0x4f) >> 4) as i8;
            }
            if cqe4_b(c + 0x4e) & 8 != 0 {
                ea = if ea == 1 { 3 } else if ea < 1 || ea > 3 { 7 } else { 4 };
            }
        }
        c1 = cqe4_c(c + 0x3d);
    }
    let pos20 = || cqe4_b(cqe4_d(p1 + 0x61) + 0xf) == 0x14;
    let jovem = idade < 0x20 && cqe4_c(p1 + 0x22) != 0;
    let mut sb: i16; // ebx: a reputação do clube atual (ou a da pessoa sem clube) na escala da comparação
    let mut e8: i32 = 0; // [base+0x1c]: a do clube novo
    let mut h68: i16 = 0;
    let mut h6c4: i32 = 0;
    let mut ae: i8 = 0;
    let mut den: f32 = 1.0; // [base+0x58]
    if p2 != d4 {
        if humano(p2, 0, 1, 0, 0) != 0 {
            let v = cqe4_c(cqe4_d(p2 + 0xcf) + 0x59) as i32 as f64 * k(0x967878) + k(0x968968);
            f8 = v * f8;
            if !p6.is_null() && v < k(0x96a698) {
                *p6 |= 0x20000000;
            }
        }
        match ea {
            1 => {
                f8 *= k(0x96a5d0);
                if !p6.is_null() && cqe4_d(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            2 => {
                f8 *= k(0x969040);
                if !p6.is_null() && cqe4_d(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            3 => f8 *= k(0x9689d8),
            4 => {
                f8 *= k(0x96d640);
                fl(0x2000);
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 = f8 + f8;
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 = f8 + f8;
                    }
                }
            }
            5 => {
                if !p4b {
                    f8 *= k(0x969040);
                }
                if jovem {
                    f8 = f8 + f8;
                }
            }
            6 => {
                f8 *= k(0x9689c0);
                if jovem {
                    f8 *= k(0x968938);
                }
            }
            7 => {
                f8 *= k(0x968988);
                fl(0x2000);
            }
            _ => {}
        }
        let cf10 = c != 0 && cqe4_b(c + 0x45) & 0x10 != 0;
        match p5b {
            1 => {
                if cf10 {
                    f8 = f8 + f8;
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9b0);
                }
                b4 *= cqe4_kf(0x96a33c);
            }
            2 => {
                if cf10 {
                    f8 *= k(0x96f9a8);
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9a0);
                }
                b4 *= cqe4_kf(0x96d3b8);
            }
            3 => {
                if !cf10 {
                    f8 *= k(0x96f998);
                }
                if pos20() {
                    f8 *= k(0x9689b0);
                }
            }
            4 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x96f990);
                    if cqe4_d(p1 + 0x69) != 0 {
                        f8 *= k(0x96f988);
                    } else {
                        fl(0x8000);
                    }
                } else {
                    f8 *= if pos20() { k(0x968f20) } else { k(0x968fb0) };
                }
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 *= k(0x968968);
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
                b4 *= cqe4_kf(0x96d38c);
            }
            5 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x969020);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { k(0x968918) } else { k(0x968e00) };
                }
                if jovem {
                    f8 *= k(0x968968);
                }
                b4 *= cqe4_kf(0x96f980);
            }
            6 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x968a98);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { k(0x967880) } else { k(0x968f20) };
                }
                if jovem {
                    f8 *= k(0x9689e0);
                }
                b4 *= cqe4_kf(0x968f18);
            }
            7 => f8 *= k(0x969048),
            _ => {}
        }
        // a reputação do clube novo, em várias escalas, contra a da pessoa
        let mut adv = cqe4_c(cqe4_d(p2 + 0x53) + 0x7e);
        if adv < 1 || adv > 3 {
            adv = 3;
        }
        ad = adv.wrapping_mul(adv);
        let h34 = cqe4_pelo_estadio(p2);
        h68 = cqe4_pela_divisao(p2);
        let (h6c, reg) = cqe4_pela_competicao(p2, h68);
        l84 = reg;
        let t = cqe4_d(p2 + 0xcf);
        let mut rv: i16 = 0;
        if t != 0 && cqe4_d(t + 0x69) != 0 {
            rv = vista(t, p1, 1);
            if cqe4_d(0xb67a44) != 0 {
                rv = cqe4_pelo_tecnico(p2, rv) as i16;
            }
        }
        let mut si: i16 = cqe4_h(p2 + 0x80);
        let fin = financas(cqe4_d(0xae2a38), p2);
        if fin != 0 && ad == 1 {
            si = cqe4_pela_faixa(faixa(fin) as i8, si);
        }
        let ax = cqe4_h(cqe4_d(this + 0xdc).wrapping_add(cqe4_d(p2).wrapping_mul(9)) + 5);
        let m = if si > ax { si } else { ax };
        let cx = ftol(si as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        let den0 = ((b4 as f64 + a88 as f64) + aa8 as f64) + cqe4_kf(0x9689a8) as f64;
        den = if den0 > zero { den0 as f32 } else { 0.5 };
        let s = (((h6c as i16 as f64 * b4 as f64 + h68 as f64 * a88 as f64) + h34 as i16 as f64 * aa8 as f64) + cx as f64) + rv as f64;
        let mut w = ftol(s / den as f64) as i16;
        let cl = cqe4_b(p2 + 0x64);
        if cl != 1 {
            if !(cf10 && cqe4_b(p1 + 0x22) == 0) {
                w = cqe4_metade(w);
            }
            if cl == 3 {
                w = cqe4_metade(w);
            }
        }
        e8 = ftol(w as f64 * k(0x968940) - k(0x9688e8));
        if cqe4_d(p2 + 0x53) != 0 {
            let (m, f) = cqe4_fluencia(p1, p2, e8 as i16);
            if idade < 0x12 && f < k(0x9688a0) && cqe4_d(cqe4_d(p1 + 0x1a) + 0x71) != cqe4_d(cqe4_d(p2 + 0x53) + 0x71) {
                return zero;
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x10000;
            }
        }
        // as nações, a distância e o que a pessoa quer (situação pedida)
        #[derive(PartialEq)]
        enum Segue {
            Padrao,
            Fatores,
        }
        let fav = cqe4_d(p1 + 0x39);
        let segue = 'n: {
            let cid2 = if cqe4_d(p2 + 0x69) != 0 { cqe4_d(cqe4_d(p2 + 0x69) + 0x38) } else { 0 };
            let cid4 = if d4 != 0 && cqe4_d(d4 + 0x69) != 0 { cqe4_d(cqe4_d(d4 + 0x69) + 0x38) } else { 0 };
            if fav != 0 && cqe4_d(fav + 0x53) == cqe4_d(p2 + 0x53) {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                let mut g = f8;
                if na_lista_emp(p1, 0) != 0 && idade < 0x18 && r61 != 0 && cqe4_h(r61 + 0xb) < 0x109a && cqe4_c(p1 + 0x5b) > 0xf
                    && cqe4_c(p1 + 0x5a) > 0xa && c != 0 && !matches!(ea, 1 | 2 | 3 | 4)
                {
                    g *= k(0x968e18);
                }
                if d4 != 0 && cqe4_d(p2 + 0x53) != 0 && cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf360) && idade < 0x18
                    && cqe4_d(d4 + 0x57) != 0 && cqe4_d(cqe4_d(d4 + 0x57)) == cqe4_d(0x9cf570)
                {
                    g *= k(0x968930);
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = cqe4_distancia(cid2, cid4);
                    if dd < k(0x968e30) {
                        f8 = g * k(0x96f978);
                        fl(0x20000);
                    } else if dd < k(0x96d6e0) {
                        let m = k(0x96afa0) - dd * k(0x968a90);
                        f8 = m * g;
                        if !p6.is_null() && m > k(0x969018) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 = g * k(0x968de8);
                    }
                } else {
                    f8 = g * k(0x968de8);
                }
                break 'n Segue::Fatores;
            }
            if cqe4_d(p2 + 0x53) != 0 && estrangeira(p1, cqe4_d(p2 + 0x53), 1) == 0 {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                if d4 == 0 {
                    break 'n Segue::Fatores;
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = cqe4_distancia(cid2, cid4);
                    if dd < k(0x968e30) {
                        f8 *= k(0x96f978);
                        fl(0x20000);
                    } else if dd < k(0x96d6e0) {
                        let m = k(0x96afa0) - dd * k(0x968a90);
                        f8 = m * f8;
                        if !p6.is_null() && m > k(0x969018) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 *= k(0x968de8);
                    }
                } else {
                    f8 *= k(0x968de8);
                }
                break 'n Segue::Fatores;
            }
            let t = cqe4_d(p2 + 0xcf);
            let mesmo_tecnico = t != 0 && cqe4_d(t + 0x1a) != 0 && estrangeira(p1, cqe4_d(t + 0x1a), 1) == 0;
            if cqe4_d(p2 + 0x53) != 0 && cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf314) {
                f8 *= k(0x969040);
            }
            if p4b {
                f8 *= if mesmo_tecnico { k(0x96a6a8) } else { k(0x968920) };
                fl(0x40000);
                break 'n Segue::Fatores;
            }
            let c56 = cqe4_c(p1 + 0x56) as i32;
            let c57 = cqe4_c(p1 + 0x57) as i32;
            if mesmo_tecnico {
                match p5b {
                    2 => f8 = ((c56 as f64 * k(0x968ef0) + c57 as f64 * k(0x968a98)) + k(0x9689b0)) * f8,
                    3 => f8 = ((c56 as f64 * k(0x968ef0) + c57 as f64 * k(0x968a98)) + k(0x968e40)) * f8,
                    4 => {
                        f8 *= k(0x968968);
                        fl(0x80000);
                    }
                    5 => {
                        f8 *= k(0x968e00);
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= k(0x968918);
                        fl(0x80000);
                    }
                    _ => {}
                }
            } else {
                match p5b {
                    2 => f8 = ((c56 + c57) as f64 * k(0x967898) + k(0x968e40)) * f8,
                    3 => f8 = ((c56 + c57) as f64 * k(0x967898) + k(0x968e48)) * f8,
                    4 => {
                        f8 *= k(0x9689e0);
                        fl(0x80000);
                    }
                    5 => {
                        f8 = ((c56 + c57) as f64 * k(0x968d60) + k(0x9689e0)) * f8;
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= k(0x967878);
                        fl(0x80000);
                    }
                    _ => {}
                }
            }
            Segue::Padrao
        };
        if segue == Segue::Padrao {
            if (c != 0 && cqe4_b(c + 0x49) != 0) || na_lista(p1, 0) != 0 {
                f8 *= k(0x968e18);
                if f8 < k(0x968950) && (cqe4_h(p2 + 0x80) > 0x1a5e || cqe4_h(p2 + 0x80) > cqe4_h(d4 + 0x80)) {
                    let t = f8 + k(0x967890);
                    f8 = if k(0x968950) < t { 2.0 } else { t };
                }
            }
        } else {
            if na_lista_emp(p1, 0) != 0 {
                f8 = (k(0x968940) - c1 as f64 * k(0x967898)) * f8;
            }
            let mut m = k(0x967890) - c1 as f64 * k(0x96d370);
            if m < k(0x967880) {
                m = k(0x967880);
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x40000;
            }
            let a = if pos20() {
                (0x21 - idade as i32) as f64
            } else {
                let a = (0x1c - idade as i32) as f64;
                if a < zero { a * k(0x968e00) } else { a }
            };
            let a = a.abs() * k(0x968f00) + k(0x9689e0);
            f8 = a * f8;
            if !p6.is_null() && a < k(0x968e48) {
                *p6 |= 0x40000;
            }
        }
        if d4 == 0 {
            f8 *= k(0x968f98);
        } else if na_lista_transf(p1, 0) == 0 && na_lista_emp(p1, 0) == 0 && (c == 0 || cqe4_b(c + 0x49) == 0) {
            let dd = ((cqe4_w(0xae2c92) as i32 - cqe4_w(p1 + 0x40) as i32) * 365 - cqe4_h(p1 + 0x3e) as i32) + cqe4_h(0xae2c90) as i32;
            let e: i32 = if p4b {
                (20 - c1 as i32 / 5).wrapping_mul(dd)
            } else {
                if cqe4_d(c + 0x45) == 0 && dd < (cqe4_c(p1 + 0x59) as i32 + 8) * 15 {
                    return zero;
                }
                let dv = cqe4_c(p1 + 0x59) as i32 + 10;
                dd.wrapping_mul(20) / if dv < 1 { 1 } else { dv }
            };
            if e < 0 {
                return zero;
            }
            if e > 0x190 {
                l80 = 400.0 * k(0x969028);
            } else {
                if e == 0 {
                    return zero;
                }
                l80 = e as f64 * k(0x969028);
            }
        }
    } else {
        // o próprio clube: a situação no elenco pela pedida, com o técnico há mais de 150 dias
        if humano(p2, 0, 1, 0, 0) != 0 && cqe4_depois_de(cqe4_d(p2 + 0xcf)) {
            let lim = 0xceu8.wrapping_sub(cqe4_b(p1 + 0x59)) as i8;
            if c1 < lim && ea != 5 && ea != 6 {
                ea = 7;
            } else if c1 < 0 {
                if ea < 4 {
                    ea = 4;
                }
            } else if c1 < 0x32 {
                if ea < 3 {
                    ea = 3;
                }
            } else if c1 < 0x4b {
                if ea < 2 {
                    ea = 2;
                }
            } else if ea < 1 {
                ea = 1;
            }
        }
        match ea {
            1 => f8 *= k(0x968978),
            3 => f8 *= k(0x9689d8),
            4 => {
                f8 *= k(0x969520);
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 *= k(0x968968);
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
            }
            5 => {
                f8 *= k(0x969040);
                if jovem {
                    f8 *= k(0x968968);
                }
            }
            6 => {
                f8 *= k(0x968e40);
                if jovem {
                    f8 *= k(0x9689e0);
                }
            }
            7 => f8 *= k(0x967898),
            _ => {}
        }
    }
    // a reputação do clube atual (ou da pessoa, sem clube) na mesma escala
    if d4 == 0 {
        let mut aev = cqe4_c(cqe4_d(p1 + 0x1a) + 0x7e);
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        sb = ftol(cqe4_media3(cqe4_d(p1 + 0x61)) as i16 as f64 * k(0x968940) - k(0x9688e8)) as i16;
    } else {
        let n4 = cqe4_d(d4 + 0x53);
        let mut aev = if n4 != 0 { cqe4_c(n4 + 0x7e) } else { 3 };
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        let h44 = cqe4_pelo_estadio(d4);
        h6c4 = cqe4_pela_divisao(d4) as i32;
        let (bx, reg) = cqe4_pela_competicao(d4, h6c4 as i16);
        l8c = reg;
        let t4 = cqe4_d(d4 + 0xcf);
        let si: i16 = if t4 == 0 || cqe4_d(t4 + 0x69) == 0 {
            ftol(h6c4 as i16 as f64 * k(0x969040)) as i16
        } else if cqe4_d(0xb67a44) == 0 {
            cqe4_h(cqe4_d(t4 + 0x69) + 0xa)
        } else {
            cqe4_pelo_tecnico(d4, cqe4_h(cqe4_d(t4 + 0x69) + 0xa)) as i16
        };
        let mut di: i16 = cqe4_h(d4 + 0x80);
        let fin = financas(cqe4_d(0xae2a38), d4);
        if fin != 0 && ae == 1 && c != 0 && cqe4_d(c + 0x45) == 0 {
            let m = faixa(fin);
            if m == 0xfe {
                fl(0x40000000);
            } else {
                di = cqe4_pela_faixa(m as i8, di);
            }
        }
        let ax = cqe4_h(cqe4_d(this + 0xdc).wrapping_add(cqe4_d(d4).wrapping_mul(9)) + 5);
        let m = if di > ax { di } else { ax };
        let axv = ftol(di as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        // ponytail: no próprio clube (p2 == d4) a original divide pelo float [base+0x58] que ninguém gravou (sobra de
        // pilha); o resultado não é usado depois (o caminho vai direto ao fim), e a cópia divide por 1.
        let s = (((bx as i16 as f64 * b4 as f64 + h6c4 as i16 as f64 * a88 as f64) + h44 as i16 as f64 * aa8 as f64) + axv as f64) + si as f64;
        let mut di = ftol(s / den as f64) as i16;
        if !p4b && na_lista(p1, 0) != 0 {
            di = ftol(di as f64 * k(0x9689d8)) as i16;
        }
        let cl = cqe4_b(d4 + 0x64);
        if cl != 1 {
            if !(c != 0 && cqe4_b(c + 0x45) & 0x10 != 0 && cqe4_b(p1 + 0x22) == 0) {
                di = cqe4_metade(di);
            }
            if cl == 3 {
                di = cqe4_metade(di);
            }
        }
        if lugar(cqe4_d(0xae1050), p1, d4) == 3 {
            di = ftol(di as f64 * k(0x968e40)) as i16;
        }
        sb = ftol(di as f64 * k(0x968940) - k(0x9688e8)) as i16;
        if n4 != 0 && p2 != d4 {
            let (m, _) = cqe4_fluencia(p1, d4, sb);
            f8 = f8 / m;
        }
    }
    if p2 != d4 {
        let comp2 = cqe4_d(p2 + 0x57);
        if comp2 == 0 || cqe4_d(comp2) == cqe4_d(0x9cf764) {
            f8 *= k(0x967880);
        } else if cqe4_d(p1 + 0x39) == p2 && r61 != 0 && cqe4_c(r61 + 0x45) > 0xa {
            f8 = cqe4_c(r61 + 0x45) as i32 as f64 * f8 * k(0x967880);
        }
        let mut a8 = (cqe4_h(p2 + 0x80) as i32 - cqe4_media3(r61)) as f64 * k(0x96a6b8) + k(0x967890);
        let e8s = e8 as i16 as i32;
        let sbs = sb as i32;
        let cf10 = c != 0 && cqe4_b(c + 0x45) & 0x10 != 0;
        let mut longo = false;
        if p4b && (cf10 || na_lista_emp(p1, 0) != 0) {
            let f = cqe4_d(p1 + 0x61);
            longo = (cqe4_h(f + 5) as i32 * 50) < cqe4_h(d4 + 0x80) as i32
                && cqe4_c(p1 + 0x22) == 0
                && !(idade >= 0x18 && idade <= 0x20)
                && cqe4_h(p2 + 0x80) > 0xdac
                && cqe4_h(f + 0xb) < 0x157c
                && cqe4_d(p2 + 0x53) != 0
                && cqe4_d(p2 + 0x57) != 0
                && cqe4_d(d4 + 0x57) != 0
                && !(cqe4_h(cqe4_d(p2 + 0x57) + 0x69) <= 6 && cqe4_h(cqe4_d(p2 + 0x57) + 0x69) as i32 <= cqe4_h(cqe4_d(d4 + 0x57) + 0x69) as i32 - 3)
                && cqe4_d(d4 + 0x53) != 0
                && !(cqe4_c(cqe4_d(p2 + 0x53) + 0x85) < 0xa && cqe4_c(cqe4_d(p2 + 0x53) + 0x85) < cqe4_c(cqe4_d(d4 + 0x53) + 0x85));
        }
        let dv: f64 = if longo {
            (e8s - sbs) as f64 * k(0x968968)
        } else if cf10 && e8s < sbs && sbs - e8s < 0x1388 {
            let d = (sbs - e8s) as f64;
            d * k(0x96f970) * d
        } else if e8s > sbs {
            if c != 0 && cqe4_b(c + 0x45) & 1 != 0 {
                (e8s - sbs) as f64 * k(0x968e18)
            } else {
                (cqe4_c(p1 + 0x57) as i32 as f64 * k(0x96f930) + k(0x969040)) * (e8s - sbs) as f64
            }
        } else if p5b > 3 {
            (e8s - sbs) as f64 * k(0x968940)
        } else if !(f8 > k(0x9688b0)) {
            (e8s - sbs) as f64
        } else {
            let v = if sbs > 0x1b58 && e8s < 0x7d0 && p4b {
                let z = cqe4_kf(0x9689a8) as f64 - (sbs - e8s) as f64 * cqe4_kf(0x96f968) as f64;
                let mn = if f8 < k(0x968ff8) { f8 } else { k(0x968ff8) };
                let zm = z * mn;
                if k(0x9688b0) > zm {
                    k(0x9688b0) * k(0x967898)
                } else {
                    zm * k(0x967898)
                }
            } else {
                f8 * k(0x967898)
            };
            let mut w = k(0x9689c0) - v;
            if w < k(0x967880) {
                w = k(0x967880);
            } else if !(w <= k(0x967890)) {
                w = k(0x967890);
            }
            w * (e8s - sbs) as f64
        };
        if dv > zero && l80 < k(0x967890) {
            let mut t = dv * k(0x968a98);
            if k(0x9688a8) < t {
                t = k(0x9688a8);
            }
            let r = ((k(0x9688a8) - t) * l80 + t) * k(0x967898);
            f8 = f8 * r;
            if !p6.is_null() && r < k(0x968e48) {
                *p6 |= 0x100000;
            }
        }
        ramo("cqe_4 fator da reputacao");
        let mut e0 = dv * k(0x96f960) + k(0x967890); // cqe4: o fator da reputação comparada
        if e0 < k(0x96a350) {
            e0 = f64::from_bits(0x3f50624dd2f1a9fc);
        }
        if c != 0 && cqe4_b(c + 0x46) & 1 != 0 {
            e0 *= k(0x968940);
        }
        let mut x = e0 * f8;
        let sem_pedida = |c: usize| c == 0 || cqe4_b(c + 0x49) == 0;
        if !p6.is_null() {
            if e0 < k(0x968e48) {
                let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && sem_pedida(c);
                *p6 |= if ok { 0x400000 } else { 0x800000 };
            } else if e0 > k(0x968940) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                && !(d4 != 0 && cqe4_h(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && sem_pedida(c)
            {
                *p6 |= 0x200000;
            }
        }
        if !p4b {
            x *= e0;
        }
        if c != 0 && cqe4_b(c + 0x45) & 1 != 0 {
            x *= e0;
            if !p6.is_null() {
                if e0 < k(0x9689b0) {
                    let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && cqe4_b(c + 0x49) == 0;
                    *p6 |= if ok { 0x400000 } else { 0x800000 };
                } else if e0 > k(0x968950) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                    && !(d4 != 0 && cqe4_h(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && cqe4_b(c + 0x49) == 0
                {
                    *p6 |= 0x200000;
                }
            }
        }
        if p4b {
            if cqe4_h(r61 + 0xb) > sb && sbs > e8s {
                a8 = ((e8s - sbs) as f64 * k(0x96a600) + k(0x967890)) * a8;
                if a8 < k(0x967880) {
                    a8 = f64::from_bits(0x3fb999999999999a);
                }
            }
            let r = ae as i32 as f64 / ad as i32 as f64;
            x = x * r;
            if !p6.is_null() && r < k(0x968e48) {
                *p6 |= 0x1000000;
            }
        } else if d4 != 0 {
            if ae == ad {
                let t = cqe4_d(cqe4_d(p1 + 0x1a) + 0x71);
                if t != 0 && cqe4_d(t) == cqe4_d(0x9cfa10) {
                    let u = cqe4_d(cqe4_d(p2 + 0x53) + 0x71);
                    if u != 0 && cqe4_d(u) != cqe4_d(0x9cfa10) && u != cqe4_d(cqe4_d(d4 + 0x53) + 0x71) {
                        x *= k(0x968e00);
                        fl(0x2000000);
                    }
                }
            } else {
                let v6c = h6c4 as i16;
                let wv = if v6c > 0x1f4 { v6c as i32 } else { 500 };
                let mut q = (ae as i32 as f64 * h68 as f64) / (wv as f64 * ad as i32 as f64);
                if !(k(0x967898) <= q) {
                    q = k(0x967898);
                } else if k(0x9688b0) < q {
                    q = k(0x9688b0);
                }
                x = x * q;
                if cqe4_d(cqe4_d(p2 + 0x53) + 0x71) != cqe4_d(cqe4_d(d4 + 0x53) + 0x71) && q < k(0x967890) {
                    x *= k(0x968918);
                    fl(0x2000000);
                }
            }
        }
        f8 = a8 * x;
        if !p6.is_null() && a8 < k(0x968e48) {
            *p6 |= 0x800000;
        }
    }
    // jovem com o fim do contrato antes do fim da temporada do clube atual: não sai
    if d4 != 0 && d4 != p2 {
        let s7 = cqe4_h(cqe4_d(p1 + 0x61) + 7);
        if s7 > 0x82 && s7 as i32 > cqe4_h(d4 + 0x80) as i32 / 60 && idade < 0x1a
            && na_temporada(d4, data_a.as_mut_ptr() as usize, data_b.as_mut_ptr() as usize) != 0 && c1 > 0 && c != 0
        {
            let ano = u16::from_le_bytes([data_b[2], data_b[3]]);
            let dia = i16::from_le_bytes([data_b[0], data_b[1]]);
            let ano_c = cqe4_w(c + 0x2f);
            let antes = ano < ano_c || (ano == ano_c && dia < cqe4_h(c + 0x2d));
            if antes && cqe4_d(c + 0x45) == 0 && cqe4_b(c + 0x49) == 0
                && !(humano(p2, 1, 1, 0, 0) != 0 && cqe4_c(p1 + 0x5a) <= 0xf)
                && dias_ate(data_b.as_mut_ptr() as usize, cqe4_d(0xae2c90) as u32, cqe4_d(0xae2c94) as u32) < 0x96
            {
                fl(0x80000000);
                return zero;
            }
        }
    }
    if p3 == 0 || p4b || f8 < k(0x968f98) {
        return f8;
    }
    // o salário: o padrão (num STAFF_CONTRACT local) contra o teto do clube
    ramo("cqe_4 salario");
    let mut contrato_local = [0u32; 0x14];
    // 0x85e410: lea ecx, [base+0xa0] -> thiscall, ret 0
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    construir(contrato_local.as_mut_ptr() as usize);
    let fin = financas(cqe4_d(0xae2a38), p2);
    if fin == 0 {
        return zero;
    }
    // 0x864550: push 0; push 0xb; push situação; push 0; push clube; push pessoa; lea ecx, contrato -> thiscall, ret 0x18
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x864550usize);
    let w0 = padrao(contrato_local.as_mut_ptr() as usize, p1, p2, 0, p5, 0xb, 0);
    // 0x5968d0: push situação; push pessoa; mov ecx, finanças -> thiscall, ret 8
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let a8w = teto(fin, p1, p5) as f64;
    let mut e8w = 0.0f64;
    let base = if !(k(0x967880) <= f8) { k(0x967880) } else { f8 };
    let mut q = k(0x968e38) / base; // cqe4: a pedida pelo interesse
    if q < k(0x9689d8) {
        q = f64::from_bits(0x3feccccccccccccd);
    }
    let mut wv = q * w0;
    if c != 0 {
        if a0 & 0x100 != 0 {
            e8w = 10.0;
        }
        if na_lista_transf(p1, d4) != 0 {
            e8w = e8w + k(0x9688b0);
        }
        if cqe4_c(c + 0x3d) < 0 {
            e8w = e8w - cqe4_c(c + 0x3d) as i32 as f64 * k(0x968918);
        }
        if a0 != 0 {
            e8w = e8w - cqe4_c(c + 0x44) as i32 as f64 * k(0x968918);
        }
        if !(zero <= e8w) {
            e8w = 0.0;
        } else if k(0x968ea8) < e8w {
            e8w = 20.0;
        }
    }
    let mut f = f8;
    if !(wv * k(0x96a5d0) <= a8w) {
        return zero;
    }
    if !(wv <= a8w) {
        let mut t = wv * k(0x968968);
        if !(a8w <= t) {
            t = a8w;
        }
        wv = (wv * (k(0x968ea8) - e8w) + t * e8w) * k(0x967888);
    }
    if !(wv <= a8w) && !(wv <= zero) {
        let r = a8w / wv;
        if r < k(0x968e40) {
            return k(0x967880);
        }
        f = f * r;
        wv = a8w;
    }
    if c == 0 {
        return f;
    }
    let mut g = cqe4_d(c + 0xc) as i32 as f64;
    if g < k(0x9688b0) {
        g = k(0x9688b0);
    }
    let mut h = if d4 == p2 { g / wv } else { wv / g };
    if !(k(0x967880) <= h) {
        h = k(0x967880);
    } else if k(0x9688b0) < h {
        return f * k(0x9688b0);
    } else if !(h < k(0x967890)) {
        return f * h;
    }
    f * ((((k(0x968ea8) - e8w) * h + e8w) * k(0x967888)) / q)
}

/// Comparador: lado a lado na nota (os bits do f64), nos motivos e em [0xae1080] (que a original e o salário padrão
/// 0x864550 gravam); o resto das chamadas só lê, e o STAFF_CONTRACT do salário é local.
unsafe extern "thiscall" fn comparar_cqe4_interesse(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64>(1584);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !p6.is_null() {
        regioes.push((p6 as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, p1, p2, p3, p4, p5, p6), || cqe4_interesse(this, p1, p2, p3, p4, p5, p6));
    PLACAR.registrar(igual && ro.to_bits() == rr.to_bits(), || format!("cqe4_interesse(pessoa {:#x}, clube {:#x}, {}, {}, {}) original={} {:x?} rust={} {:x?}", p1, p2, p3, p4 as u8, p5 as u8, ro, mo, rr, mr));
    ro
}

/// { 0x85e410 } (thiscall, ret 0, staff_contracts.cpp): `STAFF_CONTRACT::STAFF_CONTRACT()`: as duas datas (+0x25,
/// +0x2d) padrão (0x54a110), +4 e +0x4a = −1, +0x49, +0x3b, +0x4e, +0x40, +0x45 e +8 = 0. Devolve o próprio contrato.
unsafe extern "thiscall" fn cqe4_construir_contrato(this: usize) -> usize {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(this + 0x25);
    data_padrao(this + 0x2d);
    core::ptr::write_unaligned((this + 4) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((this + 0x4a) as *mut u32, u32::MAX);
    *((this + 0x49) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    *((this + 0x4e) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x40) as *mut u32, 0);
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0);
    core::ptr::write_unaligned((this + 8) as *mut u32, 0);
    this
}

/// Comparador: lado a lado nos 0x4f bytes do contrato e na devolução.
unsafe extern "thiscall" fn comparar_cqe4_construir_contrato(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1585);
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x4f)], || o(this), || cqe4_construir_contrato(this));
    PLACAR.registrar(igual && ro == rr, || format!("cqe4_construir_contrato({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}

// ---------- 268a leva: as regras de empréstimo da Inglaterra: pode emprestar, a conta da temporada e o dia (england_rules.cpp) ----------

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
unsafe fn cqe5_clube_do_contrato(c: usize) -> usize {
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

/// A entrada de 16 bytes do clube na tabela dos empréstimos da temporada ([[0xde1f54] + 0x91c] + id × 16, o byte +0
/// é a conta).
unsafe fn cqe5_conta(clube: usize) -> usize {
    cqe5_d(cqe5_d(0xde1f54) + 0x91c).wrapping_add(cqe5_d(clube).wrapping_mul(16))
}

/// { 0x579d60 } (thiscall, ret 0x10, england_rules.cpp): `ENGLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`, pelas regras da nação em `TRANSFER_MANAGER::can_loan_player` (0x8d0040). Sem contrato (ou sem clube do
/// contrato): pode. O clube que pede e o dono na mesma divisão inglesa ([0x9cf5bc]): motivo 0x1d. Cinco empréstimos na
/// temporada (a tabela [[0xde1f54] + 0x91c]): 0x1e. Mais de um jogador de outro clube no elenco (as 50 vagas em +0xd7,
/// contados só com o id do jogador abaixo de [0xae2364]): 0x26. Fora da janela (0x8e8e50): 9, com o adjetivo da nação.
unsafe extern "thiscall" fn cqe5_pode_emprestar(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x66f4e0: o texto da mensagem (push ...; push formato; push destino; add esp, 0x14) -> cdecl variádica
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let c = cqe5_contrato(jogador);
    if c == 0 || cqe5_clube_do_contrato(c) == 0 {
        return 1;
    }
    let c2 = cqe5_contrato(jogador);
    let dono = if c2 != 0 { cqe5_clube_do_contrato(c2) } else { 0 };
    let comp = cqe5_d(clube + 0x57);
    if comp == cqe5_d(dono + 0x57) && comp != 0 && cqe5_d(comp) == cqe5_d(0x9cf5bc) {
        crate::comparacao::ramo("cqe_5 mesma divisao");
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
    if cqe5_b(cqe5_conta(clube)) as i8 >= 5 {
        // cqe5: cinco na temporada
        crate::comparacao::ramo("cqe_5 cinco na temporada");
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
        let de = if cp != 0 { cqe5_clube_do_contrato(cp) } else { 0 };
        if de != clube {
            n = n.wrapping_add(1);
        }
    }
    if n > 1 { // cqe5: dois ao mesmo tempo
        crate::comparacao::ramo("cqe_5 dois ao mesmo tempo");
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
    crate::comparacao::ramo("cqe_5 fora da janela");
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
unsafe extern "thiscall" fn cqe5_registrar_emprestimo(_this: usize, oferta: usize) {
    let clube = cqe5_d(0xae23b4).wrapping_add(cqe5_d(oferta + 8).wrapping_mul(0x245));
    let nacao = cqe5_d(clube + 0x53);
    if nacao != 0 && cqe5_d(nacao) == cqe5_d(0x9cf2e4) {
        crate::comparacao::ramo("cqe_5 emprestimo contado");
        let p = cqe5_conta(clube);
        *(p as *mut u8) = cqe5_b(p).wrapping_add(1); // cqe5: a conta da temporada
    }
}

/// { 0x57a020 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::daily_update(data)`, pelo dia das regras: em
/// 20 de junho (só o dia do ano, 0x549f70 com 1998), a conta da temporada de cada clube ([0xae235c] em [0xae23b4]) da
/// nação destas regras (+0xa) volta a zero.
unsafe extern "thiscall" fn cqe5_dia_das_regras(this: usize, data: usize) {
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
            *(cqe5_conta(clube) as *mut u8) = 0;
        }
        i += 1;
    }
}

/// O texto da mensagem (até o zero, no máximo 0x200 bytes).
unsafe fn cqe5_texto(msg: usize) -> Vec<u8> {
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
unsafe fn cqe5_comparar_pode(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, nome: &str, this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || {
        let x = o(this, clube, jogador, msg, motivo);
        to = cqe5_texto(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, clube, jogador, msg, motivo);
        tr = cqe5_texto(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    PLACAR.registrar(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", nome, this, clube, jogador, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}

unsafe extern "thiscall" fn comparar_cqe5_pode_emprestar(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe5_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1586), cqe5_pode_emprestar, "cqe5_pode_emprestar", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado no byte da conta do clube da oferta.
unsafe extern "thiscall" fn comparar_cqe5_registrar_emprestimo(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1587);
    let clube = cqe5_d(0xae23b4).wrapping_add(cqe5_d(oferta + 8).wrapping_mul(0x245));
    let regioes = [(cqe5_conta(clube), 1usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || cqe5_registrar_emprestimo(this, oferta));
    PLACAR.registrar(igual, || format!("cqe5_registrar_emprestimo(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

/// Comparador: lado a lado na tabela das contas inteira ([0xae235c] clubes × 16 bytes) só no dia em que ela muda (20
/// de junho); nos outros dias as duas rodam sem gravar nada e só a original fica valendo.
unsafe extern "thiscall" fn comparar_cqe5_dia_das_regras(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1588);
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    let r = data_de(d.as_mut_ptr() as usize, 0x14, 5, 0x7ce, -1);
    if cqe5_h(data) != cqe5_h(r) {
        o(this, data);
        return;
    }
    let n = (cqe5_d(0xae235c) as i32).max(0) as usize;
    let regioes = [(cqe5_d(cqe5_d(0xde1f54) + 0x91c), n * 16)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, data), || cqe5_dia_das_regras(this, data));
    PLACAR.registrar(igual, || format!("cqe5_dia_das_regras(regras {:#x}) tabela diverge", this));
}

// ---------- 268a leva: o período do empréstimo na Inglaterra e as regras de empréstimo da Escócia, da Itália, da Irlanda e da Espanha (england_rules.cpp, scotland_rules.cpp, italy_rules.cpp, ireland_rules.cpp, spain_rules.cpp) ----------

unsafe fn cqe6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqe6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe6_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de um contrato: +4 é o índice em [0xae23b4] (registros de 0x245 bytes).
unsafe fn cqe6_clube_do_contrato(c: usize) -> usize {
    cqe6_d(0xae23b4).wrapping_add(cqe6_d(c + 4).wrapping_mul(0x245))
}

/// O contrato principal da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em
/// [0xae1080] como a original faz.
unsafe fn cqe6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqe6_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O começo comum das regras escocesa, irlandesa e espanhola: sem contrato do jogador (ou sem clube do contrato),
/// `None` (pode); senão o clube dono pelo contrato consultado de novo (0 se a segunda consulta não achar).
unsafe fn cqe6_dono(jogador: usize) -> Option<usize> {
    let c = cqe6_contrato(jogador);
    if c == 0 || cqe6_clube_do_contrato(c) == 0 {
        return None;
    }
    let c2 = cqe6_contrato(jogador);
    Some(if c2 != 0 { cqe6_clube_do_contrato(c2) } else { 0 })
}

/// A recusa "fora da janela" (motivo 9), com o adjetivo da nação das regras (+0xa).
unsafe fn cqe6_fora_da_janela(this: usize, msg: usize, motivo: *mut u16) -> u32 {
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nação; add esp, 0x10 -> cdecl
        let adjetivo: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x6181d0usize);
        // 0x66f4e0: push ...; push formato; push destino; add esp, 0x14 -> cdecl variádica
        let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
        let mut buf = [0u8; 100];
        adjetivo(cqe6_d(this + 0xa), buf.as_mut_ptr(), 100, 2);
        texto(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !motivo.is_null() {
        *motivo = 9;
    }
    0
}

/// Uma recusa com o texto `formato` montado com o byte +0x37 (duas vezes) e o nome em +4 do registro `r`.
unsafe fn cqe6_recusa(msg: usize, motivo: *mut u16, formato: usize, r: usize, m: u16) -> u32 {
    if msg != 0 {
        let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
        let g = cqe6_b(r + 0x37) as i8 as i32;
        texto(msg, formato, g, g, r + 4);
    }
    if !motivo.is_null() {
        *motivo = m;
    }
    0
}

/// { 0x579c70 } (thiscall, ret 0x1c, england_rules.cpp): `ENGLAND_RULES::get_loan_period(clube, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`, pelo período do empréstimo (0x8d0590). Depois de 19 de junho (só o dia do
/// ano, 0x549f70 com 1998) e com o jogador abaixo de 24 anos (+0x18): o período da regra base (0x8e8bd0). Senão começa
/// no mês seguinte e acaba três meses depois.
unsafe extern "thiscall" fn cqe6_periodo_inglaterra(this: usize, clube: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    // 0x549f70: push -1; push 0x7ce; push 5; push 0x13; lea ecx, local -> thiscall, ret 0x10
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u8; 8];
    data_de(d.as_mut_ptr() as usize, 0x13, 5, 0x7ce, -1);
    if cqe6_h(0xae2c90) > cqe6_h(d.as_ptr() as usize) && (cqe6_b(jogador + 0x18) as i8) < 0x18 { // cqe6: abaixo de 24 anos
        crate::comparacao::ramo("cqe_6 inglaterra periodo da base");
        // 0x8e8bd0: os mesmos 7 argumentos empurrados de novo; mov ecx, esi (this) -> thiscall, ret 0x1c
        let base: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize) = core::mem::transmute(0x8e8bd0usize);
        base(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador);
        return;
    }
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0 (o mês de hoje)
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    *ini_mes = (mes(0xae2c90) as i8).wrapping_add(1); // cqe6: o mês seguinte (Inglaterra)
    *ini_ano = cqe6_h(0xae2c92);
    if *ini_mes > 0xb {
        *ini_mes = (*ini_mes).wrapping_sub(0xc);
        *ini_ano = (*ini_ano).wrapping_add(1);
    }
    crate::comparacao::ramo("cqe_6 inglaterra tres meses");
    *fim_mes = (*ini_mes).wrapping_add(3); // cqe6: três meses
    *fim_ano = *ini_ano;
    if *fim_mes > 0xb {
        *fim_mes = (*fim_mes).wrapping_sub(0xc);
        *fim_ano = (*fim_ano).wrapping_add(1);
    }
}

/// { 0x7f6650 } (thiscall, ret 0x10, scotland_rules.cpp): `SCOTLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: com contrato, o clube que pede e o dono na mesma divisão escocesa ([0x9cf5dc]) não podem (0x1d).
unsafe extern "thiscall" fn cqe6_pode_emprestar_escocia(_this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    let comp = cqe6_d(clube + 0x57);
    if comp == cqe6_d(dono + 0x57) && comp != 0 && cqe6_d(comp) == cqe6_d(0x9cf5dc) {
        crate::comparacao::ramo("cqe_6 escocia mesma divisao");
        let r = cqe6_d(0xae23d0).wrapping_add(cqe6_d(0x9cf5dc).wrapping_mul(0x6b));
        return cqe6_recusa(msg, motivo, 0x9c4590, r, 0x1d);
    }
    1
}

/// { 0x7f6530 } (thiscall, ret 0x1c, scotland_rules.cpp): `SCOTLAND_RULES::get_loan_period(a1, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`: começa no mês seguinte; o fim é o mês do fim de temporada da nação a1
/// (0x4d45f0) para quem tem menos de 22 anos (+0x18), neste ano se o começo vem antes dele, senão no ano seguinte;
/// para os outros, três meses depois do começo.
unsafe extern "thiscall" fn cqe6_periodo_escocia(_this: usize, a1: usize, _a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    *ini_mes = (mes(0xae2c90) as i8).wrapping_add(1);
    *ini_ano = cqe6_h(0xae2c92);
    if *ini_mes > 0xb {
        *ini_mes = (*ini_mes).wrapping_sub(0xc);
        *ini_ano = (*ini_ano).wrapping_add(1);
    }
    // 0x4d45f0: push a1; mov ecx, [0xae1050] -> thiscall, ret 4 (o mês do fim da temporada)
    let mes_do_fim: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x4d45f0usize);
    let e = mes_do_fim(cqe6_d(0xae1050), a1) as i8;
    *fim_mes = e;
    if (cqe6_b(jogador + 0x18) as i8) < 0x16 {
        crate::comparacao::ramo("cqe_6 escocia ate o fim da temporada");
        if *ini_mes < e {
            *fim_ano = *ini_ano;
        } else {
            *fim_ano = cqe6_h(0xae2c92).wrapping_add(1);
        }
        return;
    }
    *fim_mes = (*ini_mes).wrapping_add(3);
    *fim_ano = *ini_ano;
    if *fim_mes > 0xb {
        *fim_mes = (*fim_mes).wrapping_sub(0xc);
        *fim_ano = (*fim_ano).wrapping_add(1);
    }
}

/// { 0x661770 } (thiscall, ret 0x10, italy_rules.cpp): `ITALY_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora das duas primeiras janelas (0x8e8c40 com [this+4] e [this+4] + 0xc): motivo 9. Um clube de uma das
/// cinco divisões ([0x9cf578] a [0x9cf588]) com quatro jogadores emprestados de outros clubes (ids abaixo de
/// [0xae2364], contrato de empréstimo 0x4cf6d0) não pode (0x25).
unsafe extern "thiscall" fn cqe6_pode_emprestar_italia(this: usize, clube: usize, _jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8c40: push janela; push 0xae2c90; ecx = this -> thiscall, ret 8
    let na_janela: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8e8c40usize);
    if na_janela(this, 0xae2c90, cqe6_d(this + 4)) == 0 && na_janela(this, 0xae2c90, cqe6_d(this + 4) + 0xc) == 0 {
        crate::comparacao::ramo("cqe_6 italia fora da janela");
        return cqe6_fora_da_janela(this, msg, motivo);
    }
    let comp = cqe6_d(clube + 0x57);
    if comp == 0 {
        return 1;
    }
    let id = cqe6_d(comp);
    if ![0x9cf578usize, 0x9cf57c, 0x9cf580, 0x9cf584, 0x9cf588].iter().any(|&g| cqe6_d(g) == id) {
        return 1;
    }
    // 0x4cf6d0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = cqe6_d(clube + 0xd7 + 4 * k);
        if p == 0 || cqe6_d(p) as i32 >= cqe6_d(0xae2364) as i32 || emprestimo(cqe6_d(0xae1050), p) == 0 {
            continue;
        }
        let cp = cqe6_contrato(p);
        let de = if cp != 0 { cqe6_clube_do_contrato(cp) } else { 0 };
        if de != clube {
            n = n.wrapping_add(1);
        }
    }
    if n >= 4 {
        crate::comparacao::ramo("cqe_6 italia quatro emprestados");
        if msg != 0 {
            let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
            let g = cqe6_b(clube + 0x52) as i8 as i32;
            texto(msg, 0x9ea9dc, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x25;
        }
        return 0;
    }
    1
}

/// { 0x63c3d0 } (thiscall, ret 0x10, ireland_rules.cpp): `IRELAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: cinco empréstimos na temporada (a conta de 16 bytes em [[0xde1f54] + 0x91c]): 0x1e; com contrato, o
/// clube que pede e o dono na mesma divisão (+0x57, sem conferir o nulo, como a original): 0x1d, com o nome da nação.
unsafe extern "thiscall" fn cqe6_pode_emprestar_irlanda(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let conta = cqe6_d(cqe6_d(0xde1f54) + 0x91c).wrapping_add(cqe6_d(clube).wrapping_mul(16));
    if cqe6_b(conta) as i8 >= 5 {
        crate::comparacao::ramo("cqe_6 irlanda cinco na temporada");
        if msg != 0 {
            let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
            let g = cqe6_b(clube + 0x52) as i8 as i32;
            texto(msg, 0x9c4534, g, g, clube + 0x38);
        }
        if !motivo.is_null() {
            *motivo = 0x1e;
        }
        return 0;
    }
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if cqe6_d(clube + 0x57) == cqe6_d(dono + 0x57) {
        crate::comparacao::ramo("cqe_6 irlanda mesma divisao");
        return cqe6_recusa(msg, motivo, 0x9ea168, cqe6_d(this + 0xa), 0x1d);
    }
    1
}

/// { 0x859630 } (thiscall, ret 0x10, spain_rules.cpp): `SPAIN_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: fora da janela (0x8e8e50): 9; com contrato, entre clubes da mesma divisão (+0x57) depois de mais de
/// [0x968990] jogos do jogador na liga (0x7c1e30 da ficha +0x61, tipo 2, 1, 6,5): 0x1f.
unsafe extern "thiscall" fn cqe6_pode_emprestar_espanha(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8
    let janela: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8e8e50usize);
    if janela(this, 0xae2c90, 0) == 0 {
        crate::comparacao::ramo("cqe_6 espanha fora da janela");
        return cqe6_fora_da_janela(this, msg, motivo);
    }
    let dono = match cqe6_dono(jogador) {
        None => return 1,
        Some(d) => d,
    };
    if cqe6_d(clube + 0x57) == cqe6_d(dono + 0x57) {
        // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall, ret 0x10 (devolve no st0)
        let jogos: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
        let v = jogos(cqe6_d(0xb74350), cqe6_d(jogador + 0x61), 2, 1, 6.5);
        if v > f32_em(0x968990) as f64 {
            crate::comparacao::ramo("cqe_6 espanha mesma divisao");
            return cqe6_recusa(msg, motivo, 0xa82b00, cqe6_d(this + 0xa), 0x1f);
        }
    }
    1
}

/// O texto da mensagem (até o zero, no máximo 0x200 bytes).
unsafe fn cqe6_texto(msg: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if msg == 0 {
        return v;
    }
    for k in 0..0x200usize {
        let b = cqe6_b(msg + k);
        v.push(b);
        if b == 0 {
            break;
        }
    }
    v
}

/// Comparador comum (pode emprestar): lado a lado na devolução, em [0xae1080] e no motivo; a mensagem, que só é
/// escrita, é conferida pelo texto (antes da cópia o primeiro byte vai a zero, e no fim volta o texto da original).
unsafe fn cqe6_comparar_pode(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, nome: &str, this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || {
        let x = o(this, clube, jogador, msg, motivo);
        to = cqe6_texto(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, clube, jogador, msg, motivo);
        tr = cqe6_texto(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    PLACAR.registrar(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", nome, this, clube, jogador, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}

/// Comparador: lado a lado nas quatro saídas e em [0xae1080] (a regra base pode consultar contratos).
unsafe extern "thiscall" fn comparar_cqe6_periodo_inglaterra(this: usize, clube: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(1589);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || cqe6_periodo_inglaterra(this, clube, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("cqe6_periodo_inglaterra(regras {:#x}, clube {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, clube, jogador, mo, mr));
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_escocia(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1590), cqe6_pode_emprestar_escocia, "cqe6_pode_emprestar_escocia", this, clube, jogador, msg, motivo)
}

/// Comparador: lado a lado nas quatro saídas e em [0xae1080].
unsafe extern "thiscall" fn comparar_cqe6_periodo_escocia(this: usize, a1: usize, a2: u32, ini_mes: *mut i8, ini_ano: *mut i16, fim_mes: *mut i8, fim_ano: *mut i16, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(1591);
    let regioes = [(ini_mes as usize, 1usize), (ini_ano as usize, 2), (fim_mes as usize, 1), (fim_ano as usize, 2), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador), || cqe6_periodo_escocia(this, a1, a2, ini_mes, ini_ano, fim_mes, fim_ano, jogador));
    PLACAR.registrar(igual, || format!("cqe6_periodo_escocia(regras {:#x}, jogador {:#x}) original={:x?} rust={:x?}", this, jogador, mo, mr));
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_italia(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1592), cqe6_pode_emprestar_italia, "cqe6_pode_emprestar_italia", this, clube, jogador, msg, motivo)
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_irlanda(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1593), cqe6_pode_emprestar_irlanda, "cqe6_pode_emprestar_irlanda", this, clube, jogador, msg, motivo)
}

unsafe extern "thiscall" fn comparar_cqe6_pode_emprestar_espanha(this: usize, clube: usize, jogador: usize, msg: usize, motivo: *mut u16) -> u32 {
    cqe6_comparar_pode(orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32>(1594), cqe6_pode_emprestar_espanha, "cqe6_pode_emprestar_espanha", this, clube, jogador, msg, motivo)
}
