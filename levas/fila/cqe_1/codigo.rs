// ---------- %%LEVA%%a leva: a resposta da IA ao lance e a negociação de empréstimo copiada da oferta (transfer_manager.cpp) ----------

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
unsafe fn show_error_box(arquivo: usize, linha: u32) {
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
unsafe extern "thiscall" fn ai_respond_to_bid(this: usize, vaga: usize, ensaio: u32) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
    let c = contract_of(cqe1_d(0xae1050), alvo);
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
        crate::comparison::branch("cqe_1 price by k");
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
            if v as f64 * read_f64_at(0x969520) < pedido_f && troca_recusada == 0 { // cqe1: a opção × 0,85 abaixo do pedido
                crate::comparison::branch("cqe_1 accepts");
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
        show_error_box(0xa8d514, 0x1eee);
        recusar(this, vaga, 1, motivo);
        return;
    }
    if !aceito {
        let pedido_f = pedido as f64;
        let v = valor(melhor, 0, 0, 0, 0, 1);
        let tipo = cqe1_d(o + 0x31) & 0xf;
        if v as f64 * read_f64_at(0x969040) < pedido_f && tipo != 1 && tipo != 2 && ensaio == 0 { // cqe1: a contraproposta
            crate::comparison::branch("cqe_1 counter-offer");
            cqe1_pb(melhor + 0x30, (cqe1_b(melhor + 0x30) & 0xf0) | (cqe1_b(o + 0x30) & 0xf));
            negociar(this, o, melhor, 0);
        } else {
            let w = cqe1_d(alvo + 0x52) as i32;
            let (tf, wf) = (total as f64, w as f64);
            if (tf < wf * read_f64_at(0x96a5d8) && w.wrapping_sub(total) > 2000000)
                || (tf < wf * read_f64_at(0x967880) && w.wrapping_sub(total) > 100000 && ensaio == 0)
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
unsafe extern "thiscall" fn compare_ai_respond_to_bid(this: usize, vaga: usize, ensaio: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, vaga, ensaio) // ponytail: só a original
}

/// { 0x8bee50 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da oferta local): `TRANSFER_MANAGER::
/// add_loan_negotiation(oferta, a2, a3, a4, a5)`, só pela tela de transferências (0x8fb5e0). Um STAFF_CONTRACT local
/// (0x85e410) e uma oferta local (0x8e5680) com os campos da oferta (a data +0x12 pela 0x417c10; o nibble baixo de +0x30
/// e de +0x31; as listas +0 e +4 vazias); o contrato pelo atual (0x85e590: push a5; push a4; push a3; push a2; push
/// clube +8; push alvo +0xc; lea ecx, contrato) entra na lista em this+0x946 (criada se faltar: 0x944cf1 de 0xe bytes e
/// 0x68d020; 0x68d0d0 com 0x50 bytes) e a cópia vira negociação (0x8be9d0: push 0; push &cópia; push oferta). Devolve o
/// +0x20 da cópia (o índice); a cópia é destruída (0x8e5710).
unsafe extern "thiscall" fn add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
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
unsafe extern "thiscall" fn compare_add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(%%I1%%)(this, o, a2, a3, a4, a5) // ponytail: só a original
}
