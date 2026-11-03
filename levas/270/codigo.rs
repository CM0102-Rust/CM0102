// ---------- 270a leva: a oferta de transferência posta no vetor das ofertas e o empréstimo aceito pelo clube (transfer_manager.cpp) ----------

unsafe fn cqf1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqf1_pessoa(id: usize) -> usize {
    cqf1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf1_clube(id: usize) -> usize {
    cqf1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O clube do contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; guardado em
/// [0xae1080]); sem contrato ou com o de tipo 5, 0.
unsafe fn cqf1_clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqf1_d(0xae1050), pessoa);
    cqf1_pd(0xae1080, c);
    if c == 0 || cqf1_b(c + 0x35) & 0x3f == 5 { 0 } else { cqf1_clube(cqf1_d(c + 4)) }
}

/// A vaga da oferta no vetor de ponteiros `v` (capacidade `cap`; as vagas de `lidas` em diante contam como vazias: o
/// vetor que acabou de crescer): a vaga de `inicial` (a menor livre guardada em +0x85e), senão a primeira vazia; a
/// oferta do mesmo comprador pelo mesmo alvo dá a vaga dela; a primeira oferta de outro comprador pelo alvo é a rival.
/// Para na vaga vazia quando já contou as `conta` ofertas ou quando é a primeira oferta pelo alvo. Devolve (vaga, rival).
#[allow(clippy::too_many_arguments)]
unsafe fn cqf1_vagas(v: usize, lidas: i32, cap: i32, conta: i32, alvo: usize, comprador: usize, inicial: i32, primeira: bool) -> (i32, i32) {
    let (mut vaga, mut rival, mut contadas) = (inicial, -1i32, 0i32);
    let mut i = 0i32;
    while i < cap {
        let e = if i < lidas { cqf1_d(v + 4 * i as usize) } else { 0 };
        if e != 0 {
            contadas += 1;
            if cqf1_d(e + 0xc) == alvo {
                if cqf1_d(e + 8) == comprador {
                    crate::comparacao::ramo("cqf_1 mesma vaga");
                    vaga = i; // cqf1: a vaga da oferta do mesmo comprador
                } else if rival == -1 {
                    rival = i;
                }
            }
        } else {
            if vaga == -1 {
                vaga = i;
            }
            if contadas == conta || primeira {
                break;
            }
        }
        i += 1;
    }
    (vaga, rival)
}

/// A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (0x90d130 com a linha e
/// o texto) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8) em 0xae26c4 e a caixa 0x5e8290.
unsafe fn cqf1_sem_memoria(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// { 0x8be300 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: só os `new` da
/// oferta e das listas): `TRANSFER_MANAGER::add_transfer_offer(oferta, termos)`. A menor vaga livre guardada (+0x85e)
/// é lida e volta a −1; a oferta sem valor (+0x1b = 0) perde o +0x40. Vetor cheio (+8 = +4): cresce 50 (calloc
/// 0x945501 na primeira vez, senão realloc 0x945667 com as 50 novas zeradas; sem memória, a caixa (linha 0x4d2) e o
/// fim do jogo). A vaga (`cqf1_vagas`); nenhuma: a caixa "Error" (linha 0x511) e nada. A oferta antiga da vaga sai
/// (0x8d0f60); uma nova (0x944cf1 de 0x41 bytes, 0x8e5680) recebe a cópia dos campos (a data +0x12 pela 0x417c10, o
/// nibble baixo de +0x30 e de +0x31), o índice da vaga (+0x20, também na oferta de entrada), +0x28 = −1, +0x2c e +0x10
/// zerados, +0x2d 2 vira 1, e o número de série (+0x24 = [this+0x85a], que sobe; também na de entrada). Com troca
/// (+4), uma lista nova (0x68d020) com a cópia da troca (0x68d2f0). Com rival, a vaga entra na lista das ofertas pelo
/// jogador (+0, 0x68d0d0) da rival, ou numa nova com a rival e ela. O prazo: com +0x2f, o status 7 e 28 dias se o
/// comprador é humano (0x600a50 com 0, 1, 0, 0), senão nada; sem +0x2f, 28 dias se o vendedor (clube do contrato do
/// alvo) é humano, senão o prazo da resposta (0x8cc5b0), agendado (0x8c6220) se der dias ou fatias. A notícia: a de
/// empréstimo oferecido (0x8e1780 com os termos) ou a da oferta feita (0x8d3090). O registro de 8 bytes do alvo
/// ([this+0x86e]): bit 0 de +0 e +1 sobe; o +2 de cada jogador da troca sobe. Comprador humano e alvo com ficha de
/// jogador (+0x61): 0x7f75f0 ([0xb74448], alvo, 0).
unsafe extern "thiscall" fn por_oferta_de_transferencia(this: usize, oferta: usize, termos: u32) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d0f60usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let copiar_lista: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d2f0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let prazo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x8cc5b0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let noticia_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8e1780usize);
    let noticia_oferta: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d3090usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let observar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x7f75f0usize);

    let alvo = cqf1_d(oferta + 0xc);
    let comprador = cqf1_d(oferta + 8);
    let inicial = cqf1_d(this + 0x85e) as i32;
    cqf1_pd(this + 0x85e, u32::MAX as usize);
    let primeira = cqf1_b(cqf1_d(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    if cqf1_d(oferta + 0x1b) == 0 {
        cqf1_pb(oferta + 0x40, 0);
    }
    if cqf1_d(this + 8) == cqf1_d(this + 4) {
        let cap = (cqf1_d(this + 4) as u32).wrapping_add(0x32);
        cqf1_pd(this + 4, cap as usize);
        if cqf1_d(this) == 0 {
            cqf1_pd(this, calloc(cap, 4));
        } else {
            let p = realocar(cqf1_d(this), cap.wrapping_mul(4));
            cqf1_pd(this, p);
            if p == 0 {
                cqf1_sem_memoria(0x4d2, 0xa8e940);
                *(0xb67a34 as *mut u32) = 0;
                fechar1();
                fechar2();
                sair(-1);
            }
            core::ptr::write_bytes((p + 4 * cap as usize - 200) as *mut u32, 0, 0x32);
        }
    }
    let cap = cqf1_d(this + 4) as i32;
    let (vaga, rival) = cqf1_vagas(cqf1_d(this), cap, cap, cqf1_d(this + 8) as i32, alvo, comprador, inicial, primeira);
    if vaga == -1 {
        aviso_em(0xa8d514, 0x511);
        return;
    }
    let slot = |k: i32| cqf1_d(this).wrapping_add(4 * k as usize);
    let velha = cqf1_d(slot(vaga));
    if velha != 0 {
        tirar(this, velha);
    }
    let mem = novo(0x41);
    let nova = if mem != 0 { nova_oferta(mem) } else { 0 };
    cqf1_pd(slot(vaga), nova);
    cqf1_pd(this + 8, cqf1_d(this + 8).wrapping_add(1));
    let o = cqf1_d(slot(vaga));
    for k in [0usize, 4, 8, 0xc] {
        cqf1_pd(o + k, cqf1_d(oferta + k));
    }
    cqf1_pb(o + 0x10, cqf1_b(oferta + 0x10));
    cqf1_pb(o + 0x11, cqf1_b(oferta + 0x11));
    let mut tmp = [0u8; 8];
    copiar_data(o + 0x12, tmp.as_mut_ptr(), oferta + 0x12);
    cqf1_pb(o + 0x1a, cqf1_b(oferta + 0x1a));
    cqf1_pd(o + 0x1b, cqf1_d(oferta + 0x1b));
    cqf1_pb(o + 0x1f, cqf1_b(oferta + 0x1f));
    for k in [0x20usize, 0x24, 0x28] {
        cqf1_pd(o + k, cqf1_d(oferta + k));
    }
    for k in [0x2cusize, 0x2d, 0x2e, 0x2f] {
        cqf1_pb(o + k, cqf1_b(oferta + k));
    }
    cqf1_pb(o + 0x30, (cqf1_b(oferta + 0x30) & 0xf) | (cqf1_b(o + 0x30) & 0xf0));
    cqf1_pd(o + 0x31, (cqf1_d(oferta + 0x31) & 0xf) | (cqf1_d(o + 0x31) & !0xf));
    cqf1_pd(o + 0x35, cqf1_d(oferta + 0x35));
    cqf1_pb(o + 0x39, cqf1_b(oferta + 0x39));
    cqf1_pd(o + 0x3a, cqf1_d(oferta + 0x3a));
    cqf1_pb(o + 0x3e, cqf1_b(oferta + 0x3e));
    cqf1_pb(o + 0x3f, cqf1_b(oferta + 0x3f));
    cqf1_pb(o + 0x40, cqf1_b(oferta + 0x40));
    cqf1_pd(cqf1_d(slot(vaga)) + 0x20, vaga as u32 as usize);
    cqf1_pd(oferta + 0x20, vaga as u32 as usize);
    cqf1_pd(cqf1_d(slot(vaga)) + 0x28, u32::MAX as usize);
    cqf1_pb(cqf1_d(slot(vaga)) + 0x2c, 0);
    cqf1_pb(cqf1_d(slot(vaga)) + 0x10, 0);
    let o = cqf1_d(slot(vaga));
    if cqf1_b(o + 0x2d) == 2 {
        cqf1_pb(o + 0x2d, 1);
    }
    cqf1_pd(cqf1_d(slot(vaga)) + 0x24, cqf1_d(this + 0x85a));
    cqf1_pd(oferta + 0x24, cqf1_d(cqf1_d(slot(vaga)) + 0x24));
    cqf1_pd(this + 0x85a, cqf1_d(this + 0x85a).wrapping_add(1));
    if cqf1_d(oferta + 4) != 0 {
        let mem = novo(0xe);
        let l = if mem != 0 { lista_nova(mem) } else { 0 };
        cqf1_pd(cqf1_d(slot(vaga)) + 4, l);
        copiar_lista(cqf1_d(cqf1_d(slot(vaga)) + 4), cqf1_d(oferta + 4));
    }
    if rival != -1 { // cqf1: a rival liga as ofertas
        // as listas copiam os 4 bytes do índice (a original passa o endereço das variáveis da pilha)
        let (vaga_l, rival_l) = (vaga, rival);
        crate::comparacao::ramo("cqf_1 ofertas rivais ligadas");
        let lista = cqf1_d(cqf1_d(slot(rival)));
        if lista != 0 {
            acrescentar(lista, &vaga_l, 4); // cqf1: a vaga na lista da rival
            cqf1_pd(cqf1_d(slot(vaga)), cqf1_d(cqf1_d(slot(rival))));
        } else {
            let mem = novo(0xe);
            let l = if mem != 0 { lista_nova(mem) } else { 0 };
            acrescentar(l, &rival_l, 4);
            acrescentar(l, &vaga_l, 4);
            cqf1_pd(cqf1_d(slot(rival)), l);
            cqf1_pd(cqf1_d(slot(vaga)), l);
        }
    }
    let vendedor = cqf1_clube_do_contrato(cqf1_pessoa(alvo));
    // ponytail: os dois dwords que a original empurra para a 0x8c6220 têm só o byte baixo gravado (o resto é sobra da
    // pilha); a 0x8c6220 lê os bytes (dias as u8, fatias as u8)
    let (mut dias, mut fatias) = (0u8, 0u8);
    let agenda = if cqf1_b(oferta + 0x2f) != 0 {
        let h = humano(cqf1_clube(comprador), 0, 1, 0, 0);
        cqf1_pb(cqf1_d(slot(vaga)) + 0x2c, 7);
        if h != 0 {
            dias = 0x1c;
            true
        } else {
            false
        }
    } else if vendedor != 0 && humano(vendedor, 0, 1, 0, 0) != 0 {
        dias = 0x1c;
        true
    } else {
        prazo(this, cqf1_d(slot(vaga)), &mut dias, &mut fatias);
        dias != 0 || fatias != 0
    };
    if agenda {
        crate::comparacao::ramo("cqf_1 agenda pelo prazo");
        agendar(this, cqf1_d(slot(vaga)), dias as u32, fatias as u32); // cqf1: o prazo da oferta
    }
    let o = cqf1_d(slot(vaga));
    if cqf1_b(o + 0x1f) == 1 {
        noticia_emprestimo(this, o, termos);
    } else {
        noticia_oferta(this, o);
    }
    let reg = cqf1_d(this + 0x86e).wrapping_add(cqf1_d(oferta + 0xc).wrapping_mul(8));
    cqf1_pb(reg, cqf1_b(reg) | 1);
    let reg = cqf1_d(this + 0x86e).wrapping_add(cqf1_d(oferta + 0xc).wrapping_mul(8));
    cqf1_pb(reg + 1, cqf1_b(reg + 1).wrapping_add(1));
    let troca = cqf1_d(oferta + 4);
    if troca != 0 {
        cqf1_pd(troca + 0xa, cqf1_d(troca + 2));
        let mut n = proximo(cqf1_d(oferta + 4));
        while n != 0 {
            let r = cqf1_d(this + 0x86e).wrapping_add(cqf1_d(n).wrapping_mul(8)) + 2;
            cqf1_pb(r, cqf1_b(r).wrapping_add(1));
            n = proximo(cqf1_d(oferta + 4));
        }
    }
    if humano(cqf1_clube(cqf1_d(oferta + 8)), 0, 1, 0, 0) != 0 {
        let p = cqf1_pessoa(cqf1_d(oferta + 0xc));
        if cqf1_d(p + 0x61) != 0 {
            observar(cqf1_d(0xb74448), p, 0);
        }
    }
}

/// Comparador da 0x8be300: só a original (aloca a oferta e as listas, tira a oferta antiga, agenda, entrega a
/// notícia). Confere o plano feito antes: a vaga (`cqf1_vagas`, com o vetor que cresce quando está cheio) e o número
/// de série gravados na oferta de entrada e a série que sobe.
unsafe extern "thiscall" fn comparar_por_oferta_de_transferencia(this: usize, oferta: usize, termos: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1598);
    let (alvo, comprador) = (cqf1_d(oferta + 0xc), cqf1_d(oferta + 8));
    let primeira = cqf1_b(cqf1_d(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    let (lidas, conta) = (cqf1_d(this + 4) as i32, cqf1_d(this + 8) as i32);
    let cap = if conta == lidas { lidas.wrapping_add(0x32) } else { lidas };
    let (vaga, _) = cqf1_vagas(cqf1_d(this), lidas, cap, conta, alvo, comprador, cqf1_d(this + 0x85e) as i32, primeira);
    let serie = cqf1_d(this + 0x85a);
    o(this, oferta, termos); // ponytail: só a original
    if vaga == -1 {
        return;
    }
    let (v, s, s2) = (cqf1_d(oferta + 0x20) as i32, cqf1_d(oferta + 0x24), cqf1_d(this + 0x85a));
    PLACAR.registrar(v == vaga && s == serie && s2 == serie.wrapping_add(1), || format!(
        "por_oferta_de_transferencia(alvo {}, comprador {}): vaga original {} plano {}, série {} plano {}, próxima {} plano {}",
        alvo, comprador, v, vaga, s, serie, s2, serie.wrapping_add(1)));
}

/// O status da oferta deixa seguir (0x8c0660): 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3.
unsafe fn cqf1_segue(e: usize) -> bool {
    match cqf1_b(e + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cqf1_b(e + 0x2e) >= 3,
        _ => false,
    }
}

/// { 0x8c0660 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::accept_loan_offer(vaga, marcar)`, só pela
/// tela de transferências (0x8ef1e0) e pela notícia (0x784100). Com `marcar`, +0x2d = 1. A decisão do contrato
/// (0x4cc930 em [0xae1050]), o status 8 e a notícia da oferta aceita (0x8d76d0); a decisão 0: nada mais. Senão a
/// notícia da recusa em se mudar (0x8d8310 com 0), o status 0xf com +0x2e = 3, o jogador sai das listas de observação
/// do comprador (0x84aeb0) e cada jogador da troca das do vendedor (o clube do contrato do jogador), e a oferta é
/// agendada para 10 dias (0x8c6220). Se ela e todas as ofertas pelo jogador (+0) seguem (`cqf1_segue`), o bit 0 do
/// registro de 8 bytes do jogador ([this+0x86e]) apaga.
unsafe extern "thiscall" fn aceitar_emprestimo(this: usize, vaga: *mut usize, marcar: u32) {
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cc930usize);
    let noticia_aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d76d0usize);
    let noticia_recusa: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d8310usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    crate::comparacao::ramo("cqf_1 aceitar empréstimo");
    if marcar != 0 {
        cqf1_pb(*vaga + 0x2d, 1);
    }
    let d = decidir(cqf1_d(0xae1050), *vaga);
    cqf1_pb(*vaga + 0x2c, 8);
    noticia_aceita(this, *vaga);
    if d == 0 {
        return;
    }
    noticia_recusa(this, *vaga, 0);
    cqf1_pb(*vaga + 0x2c, 0xf);
    cqf1_pb(*vaga + 0x2e, 3);
    let o = *vaga;
    let jogador = cqf1_pessoa(cqf1_d(o + 0xc));
    let c = contrato(cqf1_d(0xae1050), jogador);
    cqf1_pd(0xae1080, c);
    let vendedor = if c != 0 { cqf1_clube(cqf1_d(c + 4)) } else { 0 };
    desmarcar(cqf1_d(this + 0x856), cqf1_clube(cqf1_d(o + 8)), jogador);
    let troca = cqf1_d(o + 4);
    if troca != 0 {
        cqf1_pd(troca + 0xa, cqf1_d(troca + 2));
        let mut n = proximo(cqf1_d(o + 4));
        while n != 0 {
            desmarcar(cqf1_d(this + 0x856), vendedor, cqf1_pessoa(cqf1_d(n)));
            n = proximo(cqf1_d(o + 4));
        }
    }
    agendar(this, *vaga, 10, 0);
    let o = *vaga;
    if !cqf1_segue(o) {
        return;
    }
    let lista = cqf1_d(o);
    if lista != 0 {
        cqf1_pd(lista + 0xa, cqf1_d(lista + 2));
        let mut n = proximo(cqf1_d(o));
        while n != 0 {
            if !cqf1_segue(cqf1_d(cqf1_d(this).wrapping_add(4 * cqf1_d(n)))) {
                return;
            }
            n = proximo(cqf1_d(o));
        }
    }
    let r = cqf1_d(this + 0x86e).wrapping_add(cqf1_d(o + 0xc).wrapping_mul(8));
    if cqf1_b(r) & 1 != 0 {
        cqf1_pb(r, cqf1_b(r) & 0xfe);
    }
}

/// Comparador da 0x8c0660: só a original (entrega notícias, agenda a oferta, mexe nas listas de observação).
unsafe extern "thiscall" fn comparar_aceitar_emprestimo(this: usize, vaga: *mut usize, marcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32)>(1599)(this, vaga, marcar) // ponytail: só a original
}

// ---------- 270a leva: os termos aceitos pelo jogador e o empréstimo concluído na hora (transfer_manager.cpp) ----------

unsafe fn cqf2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf2_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqf2_pessoa(id: usize) -> usize {
    cqf2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf2_clube(id: usize) -> usize {
    cqf2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080].
unsafe fn cqf2_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqf2_d(0xae1050), pessoa);
    cqf2_pd(0xae1080, c);
    c
}

/// O status da oferta deixa seguir: 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3.
unsafe fn cqf2_segue(e: usize) -> bool {
    match cqf2_b(e + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cqf2_b(e + 0x2e) >= 3,
        _ => false,
    }
}

/// O bit 0 do registro de 8 bytes do jogador da oferta ([this+0x86e]) apaga quando ela e todas as ofertas pelo jogador
/// (a lista +0) seguem (`cqf2_segue`).
unsafe fn cqf2_apagar_marca(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    if !cqf2_segue(oferta) {
        return;
    }
    let lista = cqf2_d(oferta);
    if lista != 0 {
        cqf2_pd(lista + 0xa, cqf2_d(lista + 2));
        let mut n = proximo(cqf2_d(oferta));
        while n != 0 {
            if !cqf2_segue(cqf2_d(cqf2_d(this).wrapping_add(4 * cqf2_d(n)))) {
                return;
            }
            n = proximo(cqf2_d(oferta));
        }
    }
    let r = cqf2_d(this + 0x86e).wrapping_add(cqf2_d(oferta + 0xc).wrapping_mul(8));
    if cqf2_b(r) & 1 != 0 {
        cqf2_pb(r, cqf2_b(r) & 0xfe);
    }
}

/// A regra de transferência da nação (os 28 objetos em [this+0x8ac], o +0xa de cada um comparado com `nacao`; a
/// original lê o +0xa sem conferir o ponteiro); None sem nenhuma.
unsafe fn cqf2_regra(this: usize, nacao: usize) -> Option<usize> {
    for k in 0..0x1c {
        let p = this + 0x8ac + 4 * k;
        if cqf2_d(cqf2_d(p) + 0xa) == nacao {
            return Some(cqf2_d(p));
        }
    }
    None
}

/// { 0x8c2180 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da data local sem destrutor):
/// `TRANSFER_MANAGER::handle_player_accepted_terms(pessoa, vaga, contrato, tipo, noticia)`, pela aceitação de um
/// contrato (0x4cf413). Vaga fora do vetor: a caixa "Error" (0xe0c, 0xe11) e nada; vaga vazia: nada. O +0x2e zera. O
/// alvo da oferta aceitou: +0x10 = 2 e +0x11 = tipo; A = o comprador, B = o clube do contrato do alvo (tipo 5 não
/// conta). Senão a pessoa é um jogador da troca: o nó dela na troca (+4) ganha 2 e o tipo (+4, +6); A = o clube do
/// contrato do alvo, B = o comprador (sem troca: a caixa 0xe4d; sem o nó, nada achado: A e B a original deixa sem
/// valor, a cópia 0). Fora do empréstimo (+0x1f ≠ 1): a notícia da resposta (0x8d8e90 com A, B, 2 e a oferta) e o
/// seguimento da oferta (0x8c2f50). No empréstimo: elenco de A (0x53c590) + 1 acima de 50, o motivo 4; senão a regra
/// da nação de A (+0x53) responde (vtable +0x20 com o comprador e o alvo) com o motivo. Com motivo: a recusa
/// (0x8e23d0), o status 0x10, o fim (0x8c6db0), o alvo sai das listas do comprador e os da troca das do clube do
/// contrato do alvo (0x84aeb0), 5 dias (0x8c6220) e a marca (`cqf2_apagar_marca`). Sem motivo, o empréstimo na hora:
/// os termos (0x8e1f10), a regra da nação do comprador (vtable +8, ou +4 fora do empréstimo), o contrato assinado
/// (0x4d01e0), o +0x6d 2 da pessoa vira 0x10, o histórico (0x86da20), a lista de observação (0x842080/0x843e10 com 5),
/// 0x5563f0, os números (0x85a850, 0x540980, 0x85add0/0x85a470), 0x4196a0, o registro da transferência passada
/// (0x8cb810: a pessoa, os +0x33 de +4, +8 e +0xc dela, os ids de A e B, 0, 2 e hoje), o status 0x14 com 5 dias, a
/// marca, o fim (0x8c6db0) e a pessoa fora das listas (0x842b00 com 1).
unsafe extern "thiscall" fn termos_aceitos(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    let noticia_resposta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(0x8d8e90usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let elenco: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8e23d0usize);
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let termos: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e1f10usize);
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d01e0usize);
    let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x86da20usize);
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5563f0usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let marcar_elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x540980usize);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4196a0usize);
    let nova_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let registrar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(0x8cb810usize);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
    if vaga < 0 {
        aviso_em(0xa8d514, 0xe0c);
        return;
    }
    if vaga >= cqf2_d(this + 4) as i32 {
        aviso_em(0xa8d514, 0xe11);
        return;
    }
    let oferta = cqf2_d(cqf2_d(this).wrapping_add(4 * vaga as usize));
    if oferta == 0 {
        return;
    }
    cqf2_pb(oferta + 0x2e, 0);
    // a = edi da original, b = [esp+0x1c]
    let (mut a, mut b) = (0usize, 0usize);
    if cqf2_d(pessoa) == cqf2_d(oferta + 0xc) {
        cqf2_pb(oferta + 0x10, 2);
        cqf2_pb(oferta + 0x11, tipo as u8);
        a = cqf2_clube(cqf2_d(oferta + 8));
        let c = cqf2_contrato(pessoa);
        b = if c == 0 || cqf2_b(c + 0x35) & 0x3f == 5 { 0 } else { cqf2_clube(cqf2_d(c + 4)) };
    } else if cqf2_d(oferta + 4) == 0 {
        aviso_em(0xa8d514, 0xe4d);
    } else {
        let troca = cqf2_d(oferta + 4);
        cqf2_pd(troca + 0xa, cqf2_d(troca + 2));
        let mut n = proximo(cqf2_d(oferta + 4));
        while n != 0 {
            if cqf2_d(n) == cqf2_d(pessoa) {
                cqf2_pb(n + 4, 2);
                cqf2_pb(n + 6, tipo as u8);
                let c = cqf2_contrato(cqf2_pessoa(cqf2_d(oferta + 0xc)));
                a = if c != 0 { cqf2_clube(cqf2_d(c + 4)) } else { 0 };
                b = cqf2_clube(cqf2_d(oferta + 8));
                break;
            }
            n = proximo(cqf2_d(oferta + 4));
        }
    }
    if cqf2_b(oferta + 0x1f) != 1 {
        // ponytail: a resposta (2) vai num dword com bytes altos de sobra na original; a 0x8d8e90 lê o byte
        noticia_resposta(this, pessoa, a, b, 2, oferta, 0, noticia, 0);
        crate::comparacao::ramo("cqf_2 seguir a oferta");
        seguir(this, oferta); // cqf2: a oferta segue
        return;
    }
    crate::comparacao::ramo("cqf_2 empréstimo");
    let mut motivo: u32 = 0;
    if (elenco(a, 0, 0) as i32) + 1 > 0x32 { // cqf2: o elenco cheio
        motivo = 4;
    } else if let Some(r) = cqf2_regra(this, cqf2_d(a + 0x53)) {
        if r != 0 {
            let pode: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) = core::mem::transmute(cqf2_d(cqf2_d(r) + 0x20));
            pode(r, cqf2_clube(cqf2_d(oferta + 8)), cqf2_pessoa(cqf2_d(oferta + 0xc)), 0, &mut motivo);
        }
    }
    if motivo as u16 != 0 {
        crate::comparacao::ramo("cqf_2 empréstimo recusado");
        recusar(this, oferta, motivo);
        cqf2_pb(oferta + 0x2c, 0x10);
        fechar(this, oferta);
        let alvo = cqf2_pessoa(cqf2_d(oferta + 0xc));
        let c = cqf2_contrato(alvo);
        let dono = if c != 0 { cqf2_clube(cqf2_d(c + 4)) } else { 0 };
        desmarcar(cqf2_d(this + 0x856), cqf2_clube(cqf2_d(oferta + 8)), alvo);
        let troca = cqf2_d(oferta + 4);
        if troca != 0 {
            cqf2_pd(troca + 0xa, cqf2_d(troca + 2));
            let mut n = proximo(cqf2_d(oferta + 4));
            while n != 0 {
                desmarcar(cqf2_d(this + 0x856), dono, cqf2_pessoa(cqf2_d(n)));
                n = proximo(cqf2_d(oferta + 4));
            }
        }
        agendar(this, oferta, 5, 0);
        cqf2_apagar_marca(this, oferta);
        return;
    }
    crate::comparacao::ramo("cqf_2 empréstimo na hora");
    termos(this, oferta, contrato);
    if let Some(r) = cqf2_regra(this, cqf2_d(cqf2_clube(cqf2_d(oferta + 8)) + 0x53)) {
        if r != 0 {
            let k = if cqf2_b(oferta + 0x1f) == 0 { 4 } else { 8 };
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqf2_d(cqf2_d(r) + k));
            f(r, oferta);
        }
    }
    assinar(cqf2_d(0xae1050), contrato);
    if cqf2_b(pessoa + 0x6d) == 2 {
        cqf2_pb(pessoa + 0x6d, 0x10);
    }
    historico(cqf2_d(0xdd7f00), pessoa, b, a);
    if observado(cqf2_d(this + 0x856), pessoa, a) != 0 {
        situacao(cqf2_d(this + 0x856), pessoa, a, 5, 0);
    }
    guardar(cqf2_d(0xae28f0), cqf2_pessoa(cqf2_d(oferta + 0xc)));
    numero(cqf2_d(0xdd7ef0), cqf2_pessoa(cqf2_d(oferta + 0xc)), a, 0);
    marcar_elenco(pessoa, 1);
    if humano(a, 0, 1, 0, 0) == 0 && usa_numeros(cqf2_d(0xdd7ef0), a, 0) != 0 {
        dar_numero(cqf2_d(0xdd7ef0), a, pessoa);
    }
    acabar(cqf2_d(0xad9c6c), pessoa);
    // o registro da transferência passada, 0x25 bytes: +0 a pessoa, +4/+8/+0xc os +0x33 dos ponteiros +4/+8/+0xc dela,
    // +0x10 e +0x14 os ids de A e B, +0x18 0, +0x1c 2, +0x1d hoje
    let mut reg = [0u8; 0x28];
    let r = reg.as_mut_ptr() as usize;
    nova_data(r + 0x1d);
    cqf2_pd(r + 0x10, cqf2_d(a));
    cqf2_pd(r + 0x14, cqf2_d(b));
    let mut tmp = [0u8; 8];
    copiar_data(r + 0x1d, tmp.as_mut_ptr(), 0xae2c90);
    cqf2_pd(r, cqf2_d(pessoa));
    cqf2_pd(r + 4, cqf2_d(cqf2_d(pessoa + 4) + 0x33));
    cqf2_pd(r + 8, cqf2_d(cqf2_d(pessoa + 8) + 0x33));
    cqf2_pd(r + 0xc, cqf2_d(cqf2_d(pessoa + 0xc) + 0x33));
    cqf2_pb(r + 0x1c, 2);
    cqf2_pd(r + 0x18, 0);
    registrar(this, reg.as_mut_ptr());
    cqf2_pb(oferta + 0x2c, 0x14);
    agendar(this, oferta, 5, 0); // cqf2: o empréstimo concluído em 5 dias
    cqf2_apagar_marca(this, oferta);
    fechar(this, oferta);
    suspender(cqf2_d(this + 0x856), pessoa, 1);
}

/// Comparador da 0x8c2180: só a original (entrega notícias, conclui ou recusa o empréstimo, assina contrato, registra a
/// transferência, mexe nas listas).
unsafe extern "thiscall" fn comparar_termos_aceitos(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, u32)>(1600)(this, pessoa, vaga, contrato, tipo, noticia) // ponytail: só a original
}

/// { 0x4d71b0 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da lista local): `CONTRACT_MANAGER::
/// post_transfer_listed_news(jogador, clube)`, pelo pedido de transferência (0x4d8610), pelo jogador posto à venda
/// (0x842320), pela notícia (0x784100) e por 0x4dc3a0. Sem jogador, sem ficha (+0x61), sem clube ou sem a comissão
/// (+0xcf): nada. Sem contrato com o clube (0x4cf5f0): nada. A notícia 0xbea (local): 0 o clube, 1 o jogador, 2..4 os
/// +0x33 de +4, +8 e +0xc dele, 5 a comissão, 6..8 os +0x33 dela, 9 o contrato +0x45, 10 o nibble alto de +0x4f, 11 a
/// máscara de posições (0x5b7d20), 12 um sorteio de 2 (0x90cfc0). O preço: o valor (+0x52) com clube humano, quando o
/// jogador não está na lista do clube (0x5444a0) ou o contrato tem o nibble 1; senão o pedido (0x4d1600) × (sorteio de
/// 0x29 × [0x967898] + [0x9689b0]) truncado, arredondado para baixo em 10 mil (entre 10 mil e 200 mil, exclusive), 100 mil
/// (abaixo de 1 milhão), 250 mil (abaixo de 5 milhões) ou 1 milhão; 13 o preço. De 500 mil para cima: os interessados
/// (0x842180), até 30, os do computador e os humanos com verba (0x598800) ≥ preço × [0x9689c0]; ordenados pela reputação
/// (+0x80); 14 = até 3; 15.. os ids, com 1 (no lugar 13) se o 2º repete o 1º e 2 (no 12) se o 3º repete um deles; por
/// fim o "fora da lista" no lugar seguinte. Os destinatários (0x8d1da0 com 0x21a, até 30, com [0xde1f54] e ficha) e a
/// entrega (0x7861e0).
unsafe extern "thiscall" fn noticia_de_jogador_listado(this: usize, jogador: usize, clube: usize) {
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let lista_fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mascara: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x5b7d20usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    if jogador == 0 || cqf2_d(jogador + 0x61) == 0 || clube == 0 || cqf2_d(clube + 0xcf) == 0 {
        return;
    }
    let c = contrato_com(this, jogador, clube);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    if c != 0 {
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construir(n, 0xbea, 0);
        gravar(n, 0, cqf2_d(clube) as u32);
        gravar(n, 1, cqf2_d(jogador) as u32);
        gravar(n, 2, cqf2_d(cqf2_d(jogador + 4) + 0x33) as u32);
        gravar(n, 3, cqf2_d(cqf2_d(jogador + 8) + 0x33) as u32);
        gravar(n, 4, cqf2_d(cqf2_d(jogador + 0xc) + 0x33) as u32);
        let com = cqf2_d(clube + 0xcf);
        gravar(n, 5, cqf2_d(com) as u32);
        gravar(n, 6, cqf2_d(cqf2_d(cqf2_d(clube + 0xcf) + 4) + 0x33) as u32);
        gravar(n, 7, cqf2_d(cqf2_d(cqf2_d(clube + 0xcf) + 8) + 0x33) as u32);
        gravar(n, 8, cqf2_d(cqf2_d(cqf2_d(clube + 0xcf) + 0xc) + 0x33) as u32);
        gravar(n, 9, cqf2_d(c + 0x45) as u32);
        gravar(n, 10, (cqf2_b(c + 0x4f) >> 4) as u32);
        gravar(n, 11, mascara(cqf2_d(jogador + 0x61)) as i32 as u32);
        crate::comparacao::ramo("cqf_2 notícia do listado");
        let s = sorteio(2); // cqf2: o sorteio do dado 12
        gravar(n, 12, s as u32);
        let fora_da_lista = (listado(jogador, clube) == 0) as u32;
        let preco = if humano(clube, 0, 1, 0, 0) != 0 && (fora_da_lista == 0 || cqf2_b(c + 0x4f) & 0xf0 == 0x10) {
            cqf2_d(jogador + 0x52) as i32
        } else {
            let p = pedido(this, clube, 0, jogador, 0, 1, 1);
            let r = sorteio(0x29);
            let v = ftol((r as f64 * f64_em(0x967898) + f64_em(0x9689b0)) * p as f64); // cqf2: o preço da notícia
            if v > 0x2710 && v < 0x30d40 {
                v / 10000 * 10000
            } else if v < 1_000_000 {
                v / 100000 * 100000
            } else if v < 5_000_000 {
                v / 250000 * 250000
            } else {
                v / 1_000_000 * 1_000_000
            }
        };
        gravar(n, 0xd, preco as u32);
        if preco >= 500_000 {
            crate::comparacao::ramo("cqf_2 interessados no listado");
            let mut clubes = [0usize; 30];
            let mut q: i8 = 0;
            interessados(cqf2_d(cqf2_d(0xde1f54) + 0x856), jogador, l, 1, 1, 0);
            cqf2_pd(l + 0xa, cqf2_d(l + 2));
            let mut no = proximo(l);
            while no != 0 && q < 0x1e {
                let id = cqf2_d(no + 4);
                let pode = if humano(cqf2_clube(id), 0, 1, 0, 0) == 0 {
                    true
                } else {
                    let f = financas(cqf2_d(0xae2a38), cqf2_clube(cqf2_d(no + 4)));
                    f != 0 && preco as f64 * f64_em(0x9689c0) <= verba(f, 0, 0, 0, 1, 0) as f64
                };
                if pode && cqf2_d(no + 4) as i32 > -1 {
                    clubes[q as usize] = cqf2_clube(cqf2_d(no + 4));
                    q += 1;
                }
                no = proximo(l);
            }
            let total = q as i32;
            let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
            let mut i = 0i32;
            while i < total - 1 {
                let mut j = i + 1;
                while j < total {
                    if rep(clubes[i as usize]) < rep(clubes[j as usize]) {
                        clubes.swap(i as usize, j as usize);
                    }
                    j += 1;
                }
                i += 1;
            }
            let m = if q >= 3 { 3 } else { total };
            gravar(n, 0xe, m as u32);
            let mut lugar: i8 = 0xf;
            let mut k: i8 = 0;
            let fim = loop {
                if k as i32 >= m {
                    break lugar;
                }
                if k == 1 && clubes[1] == clubes[0] {
                    gravar(n, lugar - 2, 1);
                    break lugar;
                }
                if k == 2 && (clubes[2] == clubes[0] || clubes[2] == clubes[1]) {
                    gravar(n, lugar - 3, 2);
                    break lugar;
                }
                gravar(n, lugar, cqf2_d(clubes[k as usize]) as u32);
                lugar += 1;
                k += 1;
            };
            // ponytail: o lugar vai num dword com os bytes altos do preço na original; a 0x785790 lê o byte
            gravar(n, fim, fora_da_lista);
            let mut dest = clubes;
            let qd = if cqf2_d(0xde1f54) != 0 && cqf2_d(jogador + 0x61) != 0 {
                destinatarios(cqf2_d(0xde1f54), jogador, 0x21a, dest.as_mut_ptr(), 0x1e) as i8
            } else {
                0
            };
            for &d in dest.iter().take(if qd > 0 { qd as usize } else { 0 }) {
                entregar(cqf2_d(0xb7431c), n, d);
            }
        }
    }
    lista_fim(l);
}

/// Comparador da 0x4d71b0: só a original (sorteia, entrega a notícia, que aloca).
unsafe extern "thiscall" fn comparar_noticia_de_jogador_listado(this: usize, jogador: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1601)(this, jogador, clube) // ponytail: só a original
}
// ---------- 270a leva: o contrato que o clube oferece a uma pessoa (contract_manager.cpp) ----------

unsafe fn cqf3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf3_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

unsafe fn cqf3_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqf3_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// Os três bônus do contrato (+0x10, +0x14, +0x18): os que não são −1 (> −1) × `f`, truncados.
unsafe fn cqf3_bonus(c: usize, f: f64) {
    for k in 0..3 {
        let a = c + 0x10 + 4 * k;
        let v = cqf3_i(a);
        if v > -1 {
            cqf3_pd(a, ftol(v as f64 * f) as u32);
        }
    }
}

/// { 0x4cda70 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH do STAFF_CONTRACT local sem destrutor; x87):
/// `contract_manager_offer_contract(pessoa, clube, saida, vaga, fator, modo)`: o contrato que o clube oferece. Sem
/// clube, o do contrato da pessoa (o registro de 79 bytes em [0xae2d0c] → o contrato de 0x50 bytes em [[0xae1050]],
/// guardado em [0xae1080]); nenhum: 0. Seleção (0x53b900), regen marcado (bits 0x30 do +0xb do registro), o próprio clube
/// fora dos tipos 11..15 (0x53b620 de +0x3d) com a lista do clube em 2 ou sem comissão e dirigível (0x544820), clube
/// humano (0x600a50) ou transferência futura (0x8cca10): 0. O contrato desejado (0x85fc50 no local, com o estatuto
/// 0x849f00, o 3 de habilidade (+0xb da ficha) acima de 5250 vira 2); a devolução igual a [0x9689a0]: 0. O salário (+8)
/// e o valor (+0xc) × fator. Com luvas (+0x1d): o mínimo da multa (+0x21) pelo valor da oferta da vaga (0x8e7b50: em
/// faixas de 50 mil, 1 milhão e 5 milhões) ou 50 mil sem oferta (100 mil abaixo da 1ª faixa); sem luvas, com +0x1f ou
/// +0x1c, 50 mil. O teto: o salário máximo do clube (0x5968d0, com o cargo do nibble alto de +0x4f) × (fator + 1) × 0,5
/// no estatuto 11, senão o do pessoal (0x597db0). O contrato atual com o clube (0x4cf5f0). Jogador (+0x61): a idade em
/// dias (0x54cdb0) × k menos o peso da ficha (o tipo 0x14 fixo, senão +0x3c, +0x34 e o +0x5b da pessoa): acima de
/// [0x968df8] (ou contrato atual com o nibble 6) o fim no ano que vem (valor × [0x9689c0]; bônus × [0x968928]); acima de
/// [0x968df0], daqui a 2 anos (bônus × [0x968de8]); acima de [0x968de0] com a reputação do clube acima da da ficha (+0xd)
/// × [0x968978], o ano pela idade (bônus × [0x968940]). Valor acima de 10 mil e idade acima de [0x968dd8]: o total
/// (semanas até o fim × valor) contra a habilidade e a idade (pow 0x945ff0): reduz o valor a cada 52 semanas a menos ou
/// baixa o teto para total / semanas. Contrato atual que acaba depois: 0; lesão (0x62ed70) acima de 70 dias com o
/// [0xb63d28]: 0. Com [0xae233c] = 1, no máximo 5 anos. Valor acima do teto × [0x968dc0]: 0; acima do teto: o salário e
/// os bônus × valor / max(teto, 1) e o valor = teto. As queixas limpas (0x867520). Com `saida`: copia o contrato e 1;
/// senão, no modo 2, ou com contrato atual no modo 1 que acaba antes de hoje + 150 dias, a proposta (0x4cc490); 1.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn contrato_oferecido(this: usize, pessoa: usize, clube_arg: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let tipo_11_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
    let estatuto_de: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8 = core::mem::transmute(0x849f00usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let valor_oferta: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let teto_max: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let teto_pessoal: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(0x597db0usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let limpar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867520usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let propor: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);

    // o STAFF_CONTRACT local (0x50 bytes; a original o tem em [esp+0x38])
    let mut local = [0u32; 0x14];
    let c = local.as_mut_ptr() as usize;
    construir(c);
    let mut clube = clube_arg;
    if clube == 0 {
        let k = if pessoa == 0 {
            0
        } else if (cqf3_d(pessoa) as i32) < cqf3_i(0xae2364) {
            let idx = cqf3_i(cqf3_d(0xae2d0c).wrapping_add(cqf3_d(pessoa).wrapping_mul(0x4f)));
            if idx < 0 { 0 } else { (idx as usize).wrapping_mul(0x50).wrapping_add(cqf3_d(cqf3_d(0xae1050))) }
        } else {
            0
        };
        cqf3_pd(0xae1080, k as u32);
        if k == 0 {
            return 0;
        }
        clube = cqf3_d(0xae23b4).wrapping_add(cqf3_d(k + 4).wrapping_mul(0x245));
        if clube == 0 {
            return 0;
        }
    }
    if e_selecao(clube) != 0 {
        return 0;
    }
    if cqf3_b(cqf3_d(0xae2d0c).wrapping_add(cqf3_d(pessoa).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return 0;
    }
    if tipo_11_15(cqf3_b(pessoa + 0x3d) as i8) == 0 && clube == cqf3_d(pessoa + 0x39) {
        let lista = cqf3_d(cqf3_d(cqf3_d(0xde1f54) + 0x856) + 0xdc);
        if cqf3_b(lista.wrapping_add(cqf3_d(clube).wrapping_mul(9)) + 4) == 2 || (cqf3_d(clube + 0xcf) == 0 && dirigivel(clube) != 0) {
            return 0;
        }
    }
    if humano(clube, 0, 1, 0, 0) != 0 {
        return 0;
    }
    if futura(cqf3_d(0xde1f54), pessoa, 0) != 0 {
        return 0;
    }
    cqf3_pd(c + 4, cqf3_d(clube) as u32);
    let mut oferta = 0usize;
    if vaga != -1 {
        let p = vaga_da(cqf3_d(0xde1f54), vaga);
        if p != 0 {
            oferta = cqf3_d(p);
        }
    }
    let mut cargo: u8 = 0;
    let mut est = estatuto_de(cqf3_d(cqf3_d(0xde1f54) + 0x856), pessoa, clube, &mut cargo);
    if est == 3 && cqf3_w(cqf3_d(pessoa + 0x61) + 0xb) > 0x1482 {
        est = 2;
    }
    // ponytail: o estatuto e o cargo vão em dwords com os bytes altos de sobra da pilha na original; a 0x85fc50 e a
    // 0x597db0 leem o byte
    let r = desejado(c, pessoa, clube, vaga, est as u32, cargo as u32, 0, 0);
    if r as f64 == f64_em(0x9689a0) {
        return 0;
    }
    cqf3_pd(c + 8, ftol(cqf3_i(c + 8) as f64 * fator) as u32);
    cqf3_pd(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * fator) as u32);
    if cqf3_b(c + 0x1d) != 0 {
        let mut piso: i32 = 0;
        if oferta != 0 {
            let v = valor_oferta(oferta, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                piso = ftol((v - 0x4c4b40) as f64 * f64_em(0x968928) + f64_em(0x968e28));
            } else if v > 0xf4240 {
                piso = ftol((v - 0xf4240) as f64 * f64_em(0x968940) + f64_em(0x968e20));
            } else if v > 0xc350 {
                piso = ftol((v - 0xc350) as f64 * f64_em(0x968e18) + f64_em(0x9688f0));
            } else if cqf3_i(c + 0x21) < 0x186a0 {
                piso = 0x186a0;
            }
        } else if cqf3_i(c + 0x21) < 0xc350 {
            piso = 0xc350;
        }
        if cqf3_i(c + 0x21) < piso {
            cqf3_pd(c + 0x21, piso as u32);
        }
    } else if (cqf3_b(c + 0x1f) != 0 || cqf3_b(c + 0x1c) != 0) && cqf3_i(c + 0x21) < 0xc350 {
        cqf3_pd(c + 0x21, 0xc350);
    }
    let mut teto: f64 = if cargo == 0xb {
        let nibble = (cqf3_b(c + 0x4f) >> 4) as u32;
        let f = financas(cqf3_d(0xae2a38), clube);
        let m = teto_max(f, pessoa, nibble);
        (fator + f64_em(0x967890)) * f64_em(0x968968) * m as f64 // cqf3: o teto do estatuto 11
    } else {
        let f = financas(cqf3_d(0xae2a38), clube);
        teto_pessoal(f, cargo as i8) as f64
    };
    let atual = contrato_com(this, pessoa, clube);
    let mut ano = cqf3_w(0xae2c92) as u16;
    let ficha = cqf3_d(pessoa + 0x61);
    if ficha != 0 {
        let dias = dias_ate(0xae2c90, cqf3_d(pessoa + 0x10) as u32, cqf3_d(pessoa + 0x14) as u32);
        let mut x = dias as f64 * f64_em(0x968e10);
        if cqf3_b(ficha + 0xf) == 0x14 {
            x -= f64_em(0x968950);
        } else {
            let s = cqf3_b(ficha + 0x3c) as i8 as i32 + cqf3_b(ficha + 0x34) as i8 as i32 + cqf3_b(pessoa + 0x5b) as i8 as i32;
            x -= (s as f64 * f64_em(0x968a80) - f64_em(0x968e08)) * f64_em(0x968e00);
        }
        let fim = |c: usize| core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32;
        if x > f64_em(0x968df8) || (atual != 0 && cqf3_b(atual + 0x4f) & 0xf0 == 0x60) {
            if fim(c) > ano as u32 + 1 {
                crate::comparacao::ramo("cqf_3 fim no ano que vem");
                cqf3_pw(c + 0x2f, ano.wrapping_add(1)); // cqf3: o fim no ano que vem
                cqf3_pd(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
            }
            cqf3_bonus(c, f64_em(0x968928));
        } else if x > f64_em(0x968df0) {
            if fim(c) > ano as u32 + 2 {
                cqf3_pw(c + 0x2f, ano.wrapping_add(2));
                cqf3_pd(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
            }
            cqf3_bonus(c, f64_em(0x968de8));
        } else if x > f64_em(0x968de0) && cqf3_w(clube + 0x80) as f64 > cqf3_w(ficha + 0xd) as f64 * f64_em(0x968978) {
            let k = ftol(x + f64_em(0x968968)) as i8;
            if fim(c) as i32 > (ano as i32 - k as i32) + 0x22 {
                cqf3_pw(c + 0x2f, ano.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                cqf3_pd(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
            }
            cqf3_bonus(c, f64_em(0x968940));
        }
        if cqf3_i(c + 0xc) > 0x2710 && x > f64_em(0x968dd8) {
            let dias_fim = dias_ate(c + 0x2d, cqf3_d(0xae2c90) as u32, cqf3_d(0xae2c94) as u32);
            let mut semanas = dias_fim / 7;
            let mut total = semanas.wrapping_mul(cqf3_i(c + 0xc));
            let mut a = pow_do_jogo(cqf3_w(ficha + 0xb) as f64, f64_em(0x968950)) * f64_em(0x968dd0);
            if !(a < f64_em(0x968dc8)) {
                a = 8000.0;
            }
            let mut b = pow_do_jogo(x - f64_em(0x968dd8), f64_em(0x968950));
            if !(b > f64_em(0x967890)) {
                b = 1.0;
            }
            let limite = ftol(pow_do_jogo(a, f64_em(0x968950)) / b);
            let reduz = (limite as f64 * f64_em(0x968940)) < total as f64;
            let mut feito = false;
            if reduz {
                while semanas > 0x68 {
                    semanas -= 0x34;
                    cqf3_pd(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
                    total = semanas.wrapping_mul(cqf3_i(c + 0xc));
                    if limite >= total {
                        feito = true;
                        break;
                    }
                }
            }
            if !feito && total > limite {
                let q = limite / semanas;
                if (q as f64) < teto {
                    crate::comparacao::ramo("cqf_3 teto pelo total");
                    teto = q as f64; // cqf3: o teto pelo total
                }
            }
        }
        ano = cqf3_w(0xae2c92) as u16;
    }
    if atual != 0 {
        if core::ptr::read_unaligned((atual + 0x2f) as *const u16) >= core::ptr::read_unaligned((c + 0x2f) as *const u16) {
            return 0;
        }
        if cqf3_d(0xb63d28) != 0 {
            let l = lesao(cqf3_d(0xb63d28), pessoa);
            if l != 0 && cqf3_w(lesao(cqf3_d(0xb63d28), pessoa) + 0x2c) > 0x46 {
                return 0;
            }
            ano = cqf3_w(0xae2c92) as u16;
        }
    }
    if cqf3_b(0xae233c) == 1 && core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32 > ano as u32 + 5 {
        cqf3_pw(c + 0x2f, ano.wrapping_add(5));
    }
    let w = cqf3_i(c + 0xc) as f64;
    if w > teto * f64_em(0x968dc0) {
        return 0;
    }
    if w > teto {
        crate::comparacao::ramo("cqf_3 acima do teto");
        let den = if f64_em(0x967890) > teto { f64_em(0x967890) } else { teto };
        let razao = w / den;
        cqf3_pd(c + 8, ftol(cqf3_i(c + 8) as f64 * razao) as u32);
        cqf3_bonus(c, razao);
        cqf3_pd(c + 0xc, ftol(teto) as u32); // cqf3: o valor no teto
    }
    limpar(c, 0);
    if saida != 0 {
        for k in (0..0x20).step_by(4) {
            cqf3_pd(saida + k, cqf3_d(c + k) as u32);
        }
        cqf3_pb(saida + 0x20, cqf3_b(c + 0x20));
        cqf3_pd(saida + 0x21, cqf3_d(c + 0x21) as u32);
        let mut tmp = [0u8; 8];
        copiar_data(saida + 0x25, tmp.as_mut_ptr(), c + 0x25);
        copiar_data(saida + 0x2d, tmp.as_mut_ptr(), c + 0x2d);
        // o +0x35 pelos campos de bits da original: 6 bits, 1 bit e 1 bit
        let cl = cqf3_b(c + 0x35);
        let dl = cqf3_b(saida + 0x35);
        let al = ((((cl << 2) as i8 >> 2) as u8 ^ dl) & 0x3f) ^ dl;
        let d2 = (((((cl << 1) as i8 >> 1) as u8 ^ al) & 0xc0) ^ cl ^ al) & 0x7f;
        cqf3_pd(saida + 0x36, cqf3_d(c + 0x36) as u32);
        cqf3_pb(saida + 0x35, d2 ^ cl);
        cqf3_pb(saida + 0x3a, cqf3_b(c + 0x3a));
        cqf3_pw(saida + 0x3b, cqf3_w(c + 0x3b) as u16);
        for k in [0x3dusize, 0x3e, 0x3f] {
            cqf3_pb(saida + k, cqf3_b(c + k));
        }
        cqf3_pd(saida + 0x40, cqf3_d(c + 0x40) as u32);
        cqf3_pb(saida + 0x44, cqf3_b(c + 0x44));
        cqf3_pd(saida + 0x45, cqf3_d(c + 0x45) as u32);
        cqf3_pb(saida + 0x49, cqf3_b(c + 0x49));
        cqf3_pd(saida + 0x4a, cqf3_d(c + 0x4a) as u32);
        cqf3_pb(saida + 0x4e, cqf3_b(c + 0x4e));
        cqf3_pb(saida + 0x4f, cqf3_b(c + 0x4f));
        return 1;
    }
    let modo = modo as u8;
    let enviar = if modo == 2 {
        true
    } else if atual == 0 || modo != 1 {
        false
    } else {
        let mut d = [0u8; 8];
        let p = mais(0xae1068, d.as_mut_ptr(), 0x96);
        let (dia, ano_d) = (cqf3_w(p), core::ptr::read_unaligned((p + 2) as *const u16));
        let ano_a = core::ptr::read_unaligned((atual + 0x2f) as *const u16);
        ano_d < ano_a || (ano_d == ano_a && dia < cqf3_w(atual + 0x2d))
    };
    if enviar {
        // ponytail: o cargo (+0x3d) vai num dword com os bytes altos de sobra do edx na original; a 0x4cc490 lê o byte
        propor(this, c, vaga, u32::MAX, cqf3_b(pessoa + 0x3d) as u32, 0, 1, 0);
    }
    1
}

/// Comparador da 0x4cda70: com `saida`, lado a lado (a saída, [0xae1080] e o estado do sorteador; a cópia sem contar os
/// sorteios); sem `saida` (pode propor o contrato, que aloca), só a original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_contrato_oferecido(this: usize, pessoa: usize, clube: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, f64, u32) -> u32>(1602);
    if saida == 0 {
        return o(this, pessoa, clube, saida, vaga, fator, modo); // ponytail: só a original
    }
    let est = crate::aleatorio::REGIOES_DO_ESTADO;
    let regioes = [(saida, 0x50usize), (0xae1080usize, 4usize), est[0], est[1]];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, saida, vaga, fator, modo),
        || crate::aleatorio::sem_contar(|| contrato_oferecido(this, pessoa, clube, saida, vaga, fator, modo)));
    // ponytail: o STAFF_CONTRACT local da original não tem valor em +0x38..+0x39 (do dword +0x36), +0x44 e no nibble
    // baixo de +0x4f (nem a 0x85e410 nem a 0x85fc50 os gravam: sobra de pilha, copiada para a saída); a cópia deixa 0
    // (montada em memória zerada). A comparação ignora esses bits.
    let sem_sobra = |m: &crate::comparacao::Memoria| {
        let mut v = m.0[0].1.clone();
        v[0x38] = 0;
        v[0x39] = 0;
        v[0x44] = 0;
        v[0x4f] &= 0xf0;
        v
    };
    let igual = igual || (sem_sobra(&mo) == sem_sobra(&mr) && mo.0[1..] == mr.0[1..]);
    PLACAR.registrar(igual && r == n, || format!(
        "contrato_oferecido(pessoa {:#x}, clube {:#x}, vaga {}, fator {}, modo {}): original {} rust {}, memória {:x?} x {:x?}",
        pessoa, clube, vaga, fator, modo, r, n, mo, mr));
    r
}

// ---------- 270a leva: a Copa do Mundo: o sorteio dos grupos (com o laço da queda de 27/12), o sorteio feito no dia e o dia da Copa (world_cup.cpp) ----------

unsafe fn cqf4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf4_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqf4_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// As chaves das 32 nações do sorteio real de 2002 (os globais g_key_nation_*, índice na tabela de nações [0xae23a8],
/// 0x122 bytes cada), na ordem em que a original as grava na lista: grupos A a H, quatro por grupo.
const CQF4_SORTEIO_2002: [usize; 32] = [
    0x9cf300, 0x9cf460, 0x9cf4f4, 0x9cf2c0, // A: França, Senegal, Uruguai, Dinamarca
    0x9cf488, 0x9cf474, 0x9cf420, 0x9cf480, // B: Espanha, Eslovênia, Paraguai, África do Sul
    0x9cf260, 0x9cf4dc, 0x9cf298, 0x9cf2a8, // C: Brasil, Turquia, China, Costa Rica
    0x9cf484, 0x9cf430, 0x9cf4f8, 0x9cf434, // D: Coreia do Sul, Polônia, Estados Unidos, Portugal
    0x9cf314, 0x9cf458, 0x9cf2dc, 0x9cf27c, // E: Alemanha, Arábia Saudita, Irlanda, Camarões
    0x9cf218, 0x9cf400, 0x9cf2e4, 0x9cf4a8, // F: Argentina, Nigéria, Inglaterra, Suécia
    0x9cf360, 0x9cf2d4, 0x9cf2ac, 0x9cf3d0, // G: Itália, Equador, Croácia, México
    0x9cf36c, 0x9cf244, 0x9cf448, 0x9cf4d8, // H: Japão, Bélgica, Rússia, Tunísia
];

/// { 0x92e940 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::draw_groups()`, do sorteio feito no dia
/// (0x92e7f0). A lista do sorteio: 32 entradas de 5 bytes (+0 o id do clube da seleção, +4 o grupo 1..8), zerada
/// (0x945501(0x20, 5): push 5; push 0x20; add esp, 8 -> cdecl; sem memória, a caixa da linha 0x4b5 e devolve 0).
/// 1) Os cabeças de chave (8 vagas de 5 bytes em +0xee: +0 o id, +4 o pote; 0xff = já tirada): a vaga 0 vai para o
///    grupo 1 e a vaga 1 (+0xf3) para o grupo 3; os grupos 2 e 4..8 tiram uma das vagas 2..7 (0x90cfc0(6) + 2),
///    sorteando de novo enquanto a vaga sorteada já saiu. ESTE É O LAÇO DA QUEDA DE 27/12/2001 (o jogo para sem erro,
///    contornado pelo Alan_v2 trocando o desvio em 0x92ea55): se menos de seis das vagas 2..7 estão de pé (numa carreira
///    começada antes de 2001, a seed_teams 0x92f230 não semeia a Copa de 2002 e as vagas ficam 0xff), o sorteio nunca
///    acha vaga e o laço não termina. A cópia repete o laço como a original, sem limite.
/// 2) Os outros 24 (as vagas de +0x116, na ordem) caem em grupos sorteados (0x90cfc0(8)), cada grupo com no máximo
///    (k / 8) + 1 times quando o k-ésimo entra (k de 8 a 31): sorteia de novo enquanto o grupo já tem o limite.
/// 3) Na Copa de 2002 (+0x40 = 2002), depois do sorteio, a lista vira o sorteio real (a ordem de `CQF4_SORTEIO_2002`:
///    o id do clube da seleção principal, 0x53b3d0 push nação; add esp -> cdecl), os grupos i/4 + 1, e a tabela das 32
///    nações da Copa ([+0x14], 6 bytes: +0 o clube, +4 a situação) recebe os clubes com a situação 6, a França (detentora)
///    2, a Coreia e o Japão (sedes) 1; +0x36 (quantas) = 32.
/// Devolve a lista.
unsafe extern "thiscall" fn mundial_sortear_grupos(this: usize) -> usize {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let mut conta = [0i8; 8];
    let lista = zerada(0x20, 5);
    if lista == 0 {
        aviso_em(0xad5888, 0x4b5);
        return 0;
    }
    let mut s = 0i32;
    while s < 8 {
        conta[s as usize] = conta[s as usize].wrapping_add(1);
        if s == 0 {
            cqf4_pd(lista, cqf4_d(this + 0xee));
            cqf4_pb(lista + 4, 1);
            cqf4_pb(this + 0xf2, 0xff);
        } else if s == 2 {
            cqf4_pd(lista + 0xa, cqf4_d(this + 0xf3));
            cqf4_pb(lista + 0xe, 3);
            cqf4_pb(this + 0xf7, 0xff);
        } else {
            // o laço da queda de 27/12/2001 (0x92ea55): sem vaga de pé entre as 2..7, não termina
            let mut k = sorteio(6).wrapping_add(2) as i16 as i32;
            while cqf4_b(this.wrapping_add((k * 5) as usize) + 0xf2) == 0xff {
                k = sorteio(6).wrapping_add(2) as i16 as i32;
            }
            let c = this.wrapping_add((k * 5) as usize);
            let e = lista + s as usize * 5;
            cqf4_pd(e, cqf4_d(c + 0xee));
            cqf4_pb(e + 4, (s as u8).wrapping_add(1));
            cqf4_pb(c + 0xf2, 0xff);
        }
        s += 1;
    }
    let mut src = this + 0x116;
    let mut dst = lista + 0x28;
    let mut k = 8i32;
    for _ in 0..0x18 {
        let lim = ((k / 8) as i8).wrapping_add(1); // cqf4: o limite de times por grupo
        let mut r = sorteio(8);
        while conta[r as i16 as usize] >= lim {
            r = sorteio(8);
        }
        crate::comparacao::ramo("cqf_4 vaga no grupo");
        cqf4_pd(dst, cqf4_d(src));
        src += 5;
        cqf4_pb(dst + 4, (r as u8).wrapping_add(1));
        dst += 5;
        let g = r as i16 as usize;
        conta[g] = conta[g].wrapping_add(1);
        k += 1;
    }
    if cqf4_w(this + 0x40) != 0x7d2 {
        return lista;
    }
    crate::comparacao::ramo("cqf_4 sorteio de 2002");
    for (i, &chave) in CQF4_SORTEIO_2002.iter().enumerate() { // cqf4: as nações do sorteio real
        let nacao = cqf4_d(0xae23a8).wrapping_add((cqf4_d(chave) as i32).wrapping_mul(0x122) as usize);
        cqf4_pd(lista + i * 5, cqf4_d(principal(nacao)));
    }
    for i in 0..32usize {
        let e = lista + i * 5;
        let clube = cqf4_d(0xae23b4).wrapping_add(cqf4_d(e).wrapping_mul(0x245));
        cqf4_pd(cqf4_d(this + 0x14) + i * 6, clube);
        cqf4_pb(cqf4_d(this + 0x14) + i * 6 + 4, 6);
        cqf4_pb(e + 4, (i / 4) as u8 + 1);
    }
    cqf4_pb(cqf4_d(this + 0x14) + 4, 2);
    cqf4_pb(cqf4_d(this + 0x14) + 0x4c, 1);
    cqf4_pb(cqf4_d(this + 0x14) + 0xac, 1);
    cqf4_pw(this + 0x36, 0x20);
    lista
}

/// Lado a lado: a original e a cópia (sem contar os sorteios) a partir do mesmo estado do sorteador, das vagas
/// (+0xee, 0xa0 bytes), de +0x36, de [0xb67a34] e, na Copa de 2002, da tabela das nações ([+0x14], 32 × 6); as duas
/// listas alocadas têm de ser iguais byte a byte (a da cópia é liberada, 0x9452ca).
unsafe extern "thiscall" fn comparar_mundial_sortear_grupos(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1603);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&[(this + 0xee, 0xa0), (this + 0x36, 2), (0xb67a34, 4)]);
    if cqf4_w(this + 0x40) == 0x7d2 && cqf4_d(this + 0x14) != 0 {
        reg.push((cqf4_d(this + 0x14), 0xc0));
    }
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| mundial_sortear_grupos(this)));
    let mesma = if a != 0 && b != 0 {
        core::slice::from_raw_parts(a as *const u8, 0xa0) == core::slice::from_raw_parts(b as *const u8, 0xa0)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && mesma, || format!("mundial_sortear_grupos: memória igual={} lista igual={}", igual, mesma));
    a
}

/// { 0x92e7f0 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::run_group_draw()`, do dia do sorteio (0x92f1a0).
/// A lista do sorteio (0x92e940, mov ecx, this -> thiscall); o grupo 1 na tabela da própria Copa (0x92d0f0 push lista;
/// mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x46d e fim); os jogos dele (0x6835c0 mov ecx -> thiscall, ret 0;
/// 0: a linha 0x473); a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a linha 0x47a). Com tudo
/// feito: +0x45 = 1, as fases dos grupos 2..8 (0x92d540 push lista; push g; mov ecx -> thiscall, ret 8, g de 0 a 6),
/// a lista liberada (0x9452ca push; add esp -> cdecl), as posições previstas (método +0x5c, 0x92f680) e os cartões
/// zerados no órgão da competição (0x7e9180 push 0; ecx = [[0xae28f0] + 4 × (char)+0x50] -> thiscall, ret 4).
/// Num erro a lista não é liberada (como a original).
unsafe extern "thiscall" fn mundial_sorteio_dos_grupos(this: usize) {
    let sortear: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x92e940usize);
    let grupo_um: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x92d0f0usize);
    let criar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(0x92d540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let cartoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e9180usize);
    let lista = sortear(this);
    if grupo_um(this, lista) == 0 {
        aviso_em(0xad5888, 0x46d);
        return;
    }
    if criar(this) == 0 {
        aviso_em(0xad5888, 0x473);
        return;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(0xad5888, 0x47a);
        return;
    }
    cqf4_pd(this + 0x45, 1);
    for g in 0..7 {
        fase(this, g, lista);
    }
    soltar(lista);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqf4_d(cqf4_d(this) + 0x5c));
    previstas(this);
    let orgao = cqf4_d(cqf4_d(cqf4_d(0xae28f0)).wrapping_add((cqf4_b(this + 0x50) as i8 as i32 * 4) as usize));
    cartoes(orgao, 0);
}

/// Só a original (o sorteio, os jogos, as notícias e as previsões não rodam duas vezes), conferindo o que ela garante:
/// terminada (+0x45 = 1), as sete fases dos grupos 2..8 existem e a fase atual (+0x30) é a última delas (6).
unsafe extern "thiscall" fn comparar_mundial_sorteio_dos_grupos(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1604)(this);
    if cqf4_d(this + 0x45) == 1 {
        let fases = cqf4_d(this + 0xc);
        let ok = fases != 0 && (0..7).all(|g| cqf4_d(fases + g * 4) != 0) && cqf4_d(this + 0x30) == 6;
        PLACAR.registrar(ok, || format!("mundial_sorteio_dos_grupos: fases dos grupos ou +0x30 ({}) fora do esperado", cqf4_d(this + 0x30) as i32));
    }
}

/// { 0x92f1a0 } (thiscall, ret 8, world_cup.cpp, método +0x14 da Copa do Mundo): `WorldCup::update_for_date(hoje,
/// completo)`, o dia da Copa pelo dia das competições. Na data das cabeças de chave (+0x18e o dia do ano, +0x190 o ano
/// relativo a +0x40: 15/12 do ano anterior, gravados pela 0x92d200) e com `completo`, semeia (0x92f230 mov ecx ->
/// thiscall, ret 0); na data do sorteio (+0xb6, +0xb8: 27/12 do ano anterior), com `completo` e sem tabela ainda (+0xb1
/// = 0), o sorteio (0x92e7f0 mov ecx -> thiscall, ret 0). Depois o dia de liga (0x6847c0 push completo; push hoje; mov
/// ecx -> thiscall, ret 8), cuja devolução (eax) ela repassa. A data: +0 o dia do ano, +2 o ano (sem sinal).
unsafe extern "thiscall" fn mundial_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let semear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x92f230usize);
    let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x92e7f0usize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let ano = core::ptr::read_unaligned((hoje + 2) as *const u16) as i32;
    let dia = core::ptr::read_unaligned(hoje as *const u16);
    if cqf4_w(this + 0x18e) as u16 == dia && cqf4_w(this + 0x190) as i32 + cqf4_w(this + 0x40) as i32 == ano && completo != 0 {
        semear(this);
    }
    let dia_do_sorteio = cqf4_w(this + 0xb6) as u16; // cqf4: o dia do sorteio
    if dia_do_sorteio == dia && cqf4_w(this + 0xb8) as i32 + cqf4_w(this + 0x40) as i32 == ano && completo != 0 && cqf4_d(this + 0xb1) == 0 {
        crate::comparacao::ramo("cqf_4 dia do sorteio");
        sortear(this);
    }
    liga(this, hoje, completo)
}

/// Só a original (a semeadura, o sorteio e o dia da liga não rodam duas vezes), conferindo o que ela garante: no dia do
/// sorteio (a condição da cópia), a Copa sai com a tabela do grupo 1 (+0xb1) e +0x45 = 1.
unsafe extern "thiscall" fn comparar_mundial_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1605);
    let ano = core::ptr::read_unaligned((hoje + 2) as *const u16) as i32;
    let dia = core::ptr::read_unaligned(hoje as *const u16);
    let sorteia = cqf4_w(this + 0xb6) as u16 == dia && cqf4_w(this + 0xb8) as i32 + cqf4_w(this + 0x40) as i32 == ano && completo != 0 && cqf4_d(this + 0xb1) == 0;
    let r = o(this, hoje, completo);
    if sorteia {
        PLACAR.registrar(cqf4_d(this + 0xb1) != 0 && cqf4_d(this + 0x45) == 1, || format!("mundial_dia: dia do sorteio sem a tabela (+0xb1 = {:#x}, +0x45 = {})", cqf4_d(this + 0xb1), cqf4_d(this + 0x45)));
    }
    r
}

// ---------- 270a leva: a Copa do Mundo: a tabela do grupo 1, as fases dos grupos 2 a 8 e as posições previstas (world_cup.cpp) ----------

unsafe fn cqf5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// O clube da entrada `i` da lista do sorteio (5 bytes: +0 o id): 0xae23b4 + id × 0x245.
unsafe fn cqf5_clube(lista: usize, i: usize) -> usize {
    cqf5_d(0xae23b4).wrapping_add(cqf5_d(lista + i * 5).wrapping_mul(0x245))
}

/// { 0x92d0f0 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::create_group_table(lista)`, do sorteio
/// (0x92e7f0). A própria Copa vira o grupo 1: +0x3e (times) = 4, a tabela [+0xb1] alocada com 0xec bytes (0x944e46 push;
/// add esp -> cdecl; sem memória, a caixa da linha 0x2b1 e devolve 0); as entradas da lista do grupo 1 (+4 = 1), na
/// ordem, entram nas linhas 0..3 (0x687430 push 0; push 0; push clube; push linha; mov ecx, this -> thiscall, ret 0x10;
/// a linha vai no cl e a original deixa lixo nos bytes altos do ecx, que a 0x687430 não lê: a cópia passa o byte
/// limpo), até 4 linhas ou o fim das 32 entradas; depois a notícia dos grupos (0x684230 mov ecx, this -> thiscall).
/// Devolve 1.
unsafe extern "thiscall" fn mundial_tabela_do_grupo_um(this: usize, lista: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha_da_tabela: unsafe extern "thiscall" fn(usize, u32, usize, u32, usize) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    cqf5_pw(this + 0x3e, 4);
    let t = alocar(0xec);
    cqf5_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xad5888, 0x2b1);
        return 0;
    }
    let mut linha = 0i8;
    let mut i = 0i8;
    while linha < 4 {
        let e = i as usize; // cqf5: a entrada da lista
        if cqf5_b(lista + e * 5 + 4) == 1 {
            crate::comparacao::ramo("cqf_5 time do grupo um");
            let clube = cqf5_clube(lista, e);
            linha_da_tabela(this, linha as u8 as u32, clube, 0, 0);
            linha += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    noticia(this);
    1
}

/// Os clubes do grupo `g` (+4 = g) da lista, na ordem, até 4 (o que a 0x92d0f0 e a 0x92d540 põem nas linhas).
unsafe fn cqf5_do_grupo(lista: usize, g: i32) -> Vec<usize> {
    (0..32usize).filter(|&i| cqf5_b(lista + i * 5 + 4) as i8 as i32 == g).map(|i| cqf5_clube(lista, i)).take(4).collect()
}

/// Só a original (aloca a tabela e manda a notícia dos grupos), conferindo o plano feito antes: devolve 1, +0x3e = 4 e
/// as linhas da tabela (0x3b bytes, +0 o clube) com os clubes do grupo 1 da lista, na ordem.
unsafe extern "thiscall" fn comparar_mundial_tabela_do_grupo_um(this: usize, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1606);
    let plano = cqf5_do_grupo(lista, 1);
    let r = o(this, lista);
    if r == 1 {
        let t = cqf5_d(this + 0xb1);
        let ok = cqf5_w(this + 0x3e) == 4 && plano.iter().enumerate().all(|(k, &c)| cqf5_d(t + k * 0x3b) == c);
        PLACAR.registrar(ok, || format!("mundial_tabela_do_grupo_um: as linhas não têm os clubes do grupo 1 ({:x?})", plano));
    }
    r
}

/// { 0x92d540 } (thiscall, ret 8, world_cup.cpp, quadro SEH só para o new: sem destrutor no caminho normal):
/// `WorldCup::create_group_stage(g, lista)`, do sorteio (0x92e7f0, g de 0 a 6: os grupos 2 a 8). Os times do grupo g+2
/// (até 4, na ordem da lista) num vetor de +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x366 e 0); a tabela
/// das rodadas pelo método +0x3c da Copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall, ret 0x10: a
/// 0x92c290 grava as duas saídas para as fases 0..6); a fase (0x944cf1(0xee) push; add esp -> cdecl; o construtor
/// League_Stage 0x68b100, thiscall, ret 0x58: 22 argumentos, conferidos na leitura dele: a Copa, os times (word),
/// o vetor, 1, +4, a tabela das rodadas, as rodadas (word: +0xa9 da fase, a conta das datas), +0xc2, +0xc3, +0xc4,
/// &+0xc5, &+0xbe, o ano +0x40 (word), g (byte), o id (word: +0x3a da fase), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x380 e 0. Com ela: +0x30 = g e a notícia do grupo
/// (0x684230 na fase). Devolve 1.
unsafe extern "thiscall" fn mundial_fase_do_grupo(this: usize, g: i32, lista: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(((cqf5_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0xad5888, 0x366);
        return 0;
    }
    let mut n = 0i16;
    let mut i = 0i16;
    while n < 4 {
        let e = lista + i as usize * 5;
        if cqf5_b(e + 4) as i8 as i32 == g + 2 {
            crate::comparacao::ramo("cqf_5 time do grupo");
            cqf5_pd(times + n as usize * 4, cqf5_clube(lista, i as usize)); // cqf5: o time na vaga do grupo
            n += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqf5_d(cqf5_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            cqf5_w(this + 0x3e) as u16 as u32,
            times,
            1,
            cqf5_d(this + 4),
            tabela,
            rodadas as u16 as u32, // cqf5: as rodadas (as datas, +0xa9 da fase)
            cqf5_b(this + 0xc2) as u32,
            cqf5_b(this + 0xc3) as u32,
            cqf5_b(this + 0xc4) as u32,
            this + 0xc5,
            this + 0xbe,
            cqf5_w(this + 0x40) as u16 as u32,
            g as u8 as u32,
            id as u16 as u32,
            cqf5_b(this + 0x51) as u32,
            4,
            0,
            cqf5_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqf5_pd(cqf5_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    if cqf5_d(cqf5_d(this + 0xc) + g as usize * 4) == 0 {
        aviso_em(0xad5888, 0x380);
        return 0;
    }
    cqf5_pd(this + 0x30, g as usize);
    noticia(cqf5_d(cqf5_d(this + 0xc) + g as usize * 4));
    1
}

/// Só a original (aloca a fase e manda a notícia), conferindo o plano: devolve 1 com a fase em [+0xc][g], +0x30 = g, a
/// fase com os times (+0x3e) da Copa, o ano dela e, na tabela da fase (+0xb1, linhas de 0x3b bytes), os clubes do
/// grupo g+2 da lista (em qualquer ordem: na comparação de 02/11 as linhas da fase não
/// saíram na ordem da lista [inferência: a fase ordena ou sorteia a tabela dela]).
unsafe extern "thiscall" fn comparar_mundial_fase_do_grupo(this: usize, g: i32, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(1607);
    let plano = cqf5_do_grupo(lista, g + 2);
    let r = o(this, g, lista);
    if r == 1 {
        let f = cqf5_d(cqf5_d(this + 0xc) + g as usize * 4);
        let t = cqf5_d(f + 0xb1);
        // as rodadas (3) na conta das datas +0xa9 e o id (0x420 + g) em +0x3a: a ordem dos argumentos do construtor
        let ok = f != 0 && cqf5_d(this + 0x30) as i32 == g && cqf5_w(f + 0x3e) == cqf5_w(this + 0x3e) && cqf5_w(f + 0x40) == cqf5_w(this + 0x40)
            && cqf5_w(f + 0xa9) == 3 && cqf5_w(f + 0x3a) as i32 == 0x420 + g
            && (t == 0 || {
                // as linhas da fase não seguem a ordem da lista: confere o conjunto dos clubes
                let mut a: Vec<usize> = (0..plano.len()).map(|k| cqf5_d(t + k * 0x3b)).collect();
                let mut b = plano.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
        PLACAR.registrar(ok, || format!("mundial_fase_do_grupo({}): a fase não bate com o plano ({:x?})", g, plano));
    }
    r
}

/// { 0x92f680 } (thiscall, ret 0, world_cup.cpp, método +0x5c da Copa do Mundo): `WorldCup::set_predicted_positions()`,
/// do sorteio (0x92e7f0). Sem estatística (+8): a caixa da linha 0x671 e volta. Menos de 32 nações (+0x36): a caixa da
/// linha 0x678 e +0x36 = 32. Os clubes da tabela das nações ([+0x14], 6 bytes) num vetor (0x944e46; sem memória, a
/// caixa da linha 0x680 e volta), ordenado pela reputação (qsort 0x945333 com 0x4b07d0: push; add esp, 0x10 -> cdecl),
/// e a posição prevista de cada um na estatística (0x4a2540 push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 4, quatro 5, oito 9, oito 17 e oito 25 (as faixas das fases finais); depois a competição atual
/// da nação de cada clube (+0x53) vira a Copa (0x775220 push [+4]; push nação; mov ecx, [0xb74318] -> thiscall, ret 8)
/// e o vetor é liberado (0x9452ca).
unsafe extern "thiscall" fn mundial_posicoes_previstas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let competicao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x775220usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqf5_d(this + 8) == 0 {
        aviso_em(0xad5888, 0x671);
        return;
    }
    if cqf5_w(this + 0x36) < 0x20 {
        aviso_em(0xad5888, 0x678);
        cqf5_pw(this + 0x36, 0x20);
    }
    let v = alocar(((cqf5_w(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        aviso_em(0xad5888, 0x680);
        return;
    }
    let mut i = 0i16;
    while i < cqf5_w(this + 0x36) {
        cqf5_pd(v + i as usize * 4, cqf5_d(cqf5_d(this + 0x14) + i as usize * 6));
        i += 1;
    }
    ordenar(v, cqf5_w(this + 0x36) as i32 as u32, 4, 0x4b07d0);
    const FAIXAS: [u32; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    for (k, &p) in FAIXAS.iter().enumerate() {
        posicao(cqf5_d(this + 8), cqf5_d(v + k * 4), p);
    }
    let mut i = 0i16;
    while i < cqf5_w(this + 0x36) {
        crate::comparacao::ramo("cqf_5 competição da nação");
        competicao(cqf5_d(0xb74318), cqf5_d(cqf5_d(v + i as usize * 4) + 0x53), cqf5_d(this + 4));
        i += 1; // cqf5: a próxima nação
    }
    soltar(v);
}

/// Só a original (acrescenta registros à estatística e avisa o gerenciador dos técnicos), conferindo o plano feito
/// antes: o vetor ordenado como ela ordena (a mesma qsort, numa cópia) e, depois, a posição prevista (+0x72 do registro
/// achado por 0x4a4850) de cada um dos 32 primeiros.
unsafe extern "thiscall" fn comparar_mundial_posicoes_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1608);
    if cqf5_d(this + 8) == 0 || cqf5_w(this + 0x36) < 0x20 || cqf5_d(this + 0x14) == 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let n = cqf5_w(this + 0x36) as usize;
    let mut v: Vec<usize> = (0..n).map(|i| cqf5_d(cqf5_d(this + 0x14) + i * 6)).collect();
    ordenar(v.as_mut_ptr(), n as u32, 4, 0x4b07d0);
    o(this);
    const FAIXAS: [u8; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    let errado = (0..32).find(|&k| {
        let e = if v[k] != 0 { achar(cqf5_d(this + 8), v[k]) } else { 0 };
        e == 0 || cqf5_b(e + 0x72) != FAIXAS[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("mundial_posicoes_previstas: a posição prevista da vaga {:?} não bate", errado));
}

// ---------- 270a leva: a Copa do Mundo: a tabela das rodadas de cada fase, a nação classificada e as cabeças de chave (world_cup.cpp) ----------

unsafe fn cqf6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf6_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqf6_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// Os jogos de cada fase de grupos na tabela das rodadas (0x68a1c0: rodada, jogo, a, b, c, valor), na ordem da
/// original, pela fase + 1 (a tabela de saltos 0x92ccd0: −1 é a própria Copa, o grupo 1; 0..6 os grupos 2..8).
const CQF6_JOGOS: [[(i32, i32, i32, i32, i32, u32); 6]; 8] = [
    [(0, 0, 2, 1, 3, 4), (0, 1, 2, 2, 3, 0x4000), (1, 0, 1, 1, 3, 0x1000), (1, 1, 1, 2, 3, 0x400), (2, 0, 1, 2, 3, 0x40), (2, 1, 1, 2, 3, 0x2000)],
    [(0, 0, 3, 1, 3, 0x1000), (0, 1, 3, 2, 3, 0x800), (1, 0, 2, 1, 3, 0x2000), (1, 1, 2, 2, 3, 0x4000), (2, 0, 1, 1, 3, 4), (2, 1, 1, 1, 3, 0x400)],
    [(0, 0, 4, 1, 3, 0x200), (0, 1, 4, 2, 3, 0x40), (1, 0, 3, 2, 3, 0x800), (1, 1, 3, 1, 3, 4), (2, 0, 2, 1, 3, 0x100), (2, 1, 2, 1, 3, 0x1000)],
    [(0, 0, 4, 1, 3, 0x4000), (0, 1, 5, 1, 3, 0x400), (1, 0, 5, 1, 3, 0x2000), (1, 1, 5, 2, 3, 0x80), (2, 0, 2, 2, 3, 0x200), (2, 1, 2, 2, 3, 0x800)],
    [(0, 0, 5, 1, 3, 0x100), (0, 1, 5, 2, 3, 4), (1, 0, 5, 1, 3, 0x1000), (1, 1, 5, 2, 3, 0x40), (2, 0, 3, 1, 3, 0x2000), (2, 1, 3, 1, 3, 0x80)],
    [(0, 0, 6, 1, 3, 0x2000), (0, 1, 0, 2, 3, 0x80), (1, 0, 6, 1, 3, 0x200), (1, 1, 6, 2, 3, 0x100), (2, 0, 3, 2, 3, 0x4000), (2, 1, 3, 2, 3, 0x400)],
    [(0, 0, 0, 1, 3, 0x40), (0, 1, 0, 2, 3, 0x100), (1, 0, 0, 1, 3, 0x4000), (1, 1, 0, 2, 3, 0x800), (2, 0, 4, 2, 3, 4), (2, 1, 4, 2, 3, 0x200)],
    [(0, 0, 6, 1, 3, 0x800), (0, 1, 6, 2, 3, 0x200), (1, 0, 5, 1, 3, 0x400), (1, 1, 6, 1, 3, 0x80), (2, 0, 4, 1, 3, 0x1000), (2, 1, 4, 1, 3, 0x100)],
];

/// { 0x92c290 } (thiscall, ret 0x10, world_cup.cpp, método +0x3c da Copa do Mundo): `WorldCup::create_schedule(fase,
/// &rodadas, &id, &é_liga)`, da criação das regras (0x92d200, fase −1) e das fases dos grupos (0x92d540, 0..6). Nas
/// fases de grupos (char < 7): é_liga (se pedido) = 1, rodadas = 3, id = 0x420 + fase; a tabela de 3 rodadas de 0x41
/// bytes (0x944e46(0xc3); sem memória, a caixa da linha 0xcc e devolve 0), as datas (0x68a160: 12, 19 e 24 de junho
/// do ano da Copa, levadas aos dias da semana 4, 4 e 2, código 1, valor 1), o jogo 2 de cada rodada vazio (−1) e os
/// dois jogos de cada rodada da fase (`CQF6_JOGOS`). O mata-mata (fase 7, com as rotinas da copa) e as fases acima:
/// a original. Devolve a tabela.
unsafe extern "thiscall" fn mundial_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f >= 7 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1609)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 3; // cqf6: três rodadas
    *id = (f as i16).wrapping_add(0x420);
    let t = alocar(0xc3);
    if t == 0 {
        aviso_em(0xad5888, 0xcc);
        return 0;
    }
    crate::comparacao::ramo("cqf_6 tabela das rodadas");
    // o ano vai com lixo nos bytes altos na original (o cx do ano sobre o ecx de antes); a 0x68a160 só usa os 16 bits
    // de baixo (a 0x549f70 lê o ano como word e a palavra +2 guarda ano − base): a cópia passa o ano limpo
    let ano = cqf6_w(this + 0x40) as u16 as u32;
    data(t, 0, 0xc, 5, 0, 4, 1, ano, 1); // cqf6: a primeira rodada
    jogo(t, 0, 2, -1, -1, -1, 0);
    data(t, 1, 0x13, 5, 0, 4, 1, ano, 1); // cqf6: a segunda rodada
    jogo(t, 1, 2, -1, -1, -1, 0);
    data(t, 2, 0x18, 5, 0, 2, 1, ano, 1); // cqf6: a terceira rodada
    jogo(t, 2, 2, -1, -1, -1, 0);
    let k = (f as i32 + 1) as u32;
    if k <= 7 {
        for &(r, j, a, b, c, v) in CQF6_JOGOS[k as usize].iter() {
            jogo(t, r, j, a, b, c, v);
        }
    }
    t
}

/// Os bytes que a tabela das rodadas tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4 e
/// +0x3d..+0x40; de cada um dos 3 jogos, +5..+0xb.
unsafe fn cqf6_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..3usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..3usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + j * 7 + 5) as *const u8, 7));
        }
    }
    v
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama e a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas alocadas têm de ser iguais (a da cópia é liberada).
/// O mata-mata: só a original.
unsafe extern "thiscall" fn comparar_mundial_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1609);
    if fase as u8 as i8 >= 7 {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || mundial_rodadas_da_fase(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqf6_gravados(a) == cqf6_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("mundial_rodadas_da_fase({}): saídas {} tabela {}", fase as u8 as i8, saidas, tabela));
    a
}

/// { 0x92e740 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::add_qualified_team(clube)`, das eliminatórias
/// (0x90f4f0, 0x911e10, 0x91538e, 0x9184c0, 0x91e200, 0x920500): com a lista das nações (+0x36) abaixo do máximo
/// (+0x38), a vaga +0x36 da tabela [+0x14] (6 bytes) recebe a situação 6 e o clube, e +0x36 sobe; cheia, a caixa da
/// linha 0x456.
unsafe extern "thiscall" fn mundial_nacao_classificada(this: usize, clube: usize) {
    let n = cqf6_w(this + 0x36);
    if n >= cqf6_w(this + 0x38) {
        aviso_em(0xad5888, 0x456);
        return;
    }
    crate::comparacao::ramo("cqf_6 nação classificada");
    let e = cqf6_d(this + 0x14).wrapping_add((n as i32 * 6) as usize);
    cqf6_pb(e + 4, 6);
    cqf6_pd(cqf6_d(this + 0x14).wrapping_add((cqf6_w(this + 0x36) as i32 * 6) as usize), clube);
    cqf6_pw(this + 0x36, (cqf6_w(this + 0x36) as u16).wrapping_add(1));
}

/// Lado a lado (+0x36 e a vaga da tabela); lista cheia (a caixa): só a original.
unsafe extern "thiscall" fn comparar_mundial_nacao_classificada(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1610);
    let n = cqf6_w(this + 0x36);
    if n >= cqf6_w(this + 0x38) || cqf6_d(this + 0x14) == 0 {
        return o(this, clube);
    }
    let reg = [(this + 0x36, 2usize), (cqf6_d(this + 0x14).wrapping_add((n as i32 * 6) as usize), 6usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || mundial_nacao_classificada(this, clube));
    PLACAR.registrar(igual, || format!("mundial_nacao_classificada: {:x?} x {:x?}", mo, mr));
}

/// { 0x92f230 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::seed_teams()`, do dia da Copa (0x92f1a0, na
/// data das cabeças de chave). Na Copa de 2002 (+0x40 = 2002) volta sem fazer nada: as vagas do sorteio dela vêm do
/// banco de dados, e é por isso que uma Copa de 2002 criada por outro caminho trava o sorteio (o laço da 0x92e940, cqf_4).
/// Nas outras: +0x4b = 0; com menos de 32 nações (+0x36), a caixa da linha 0x59e e completa a lista com a seleção
/// principal (0x53b3d0) de cada nação ([0xae2350] nações de 0x122 bytes) que ainda não esteja nela, até 32; ordena a
/// tabela [+0x14] (6 bytes) pela reputação (qsort 0x945333 com 0x4b07b0); as sedes (0x5fa730 em [0xb5e134]: thiscall,
/// ret 0x14, com o id da Copa [0x9cf79c], o ano como word, as duas saídas e 1; saída −1 = sem sede) e a detentora (o
/// campeão mais recente, 0x4afd80 da competição [0xae23d0] + id × 0x6b) vão para as vagas 1, 2 e 0 (trocando de lugar),
/// e as outras são ordenadas de novo a partir da primeira livre; as 8 primeiras ficam cabeças de chave (pote 0,
/// situação 3) e as outras 24 vão para o pote do continente (+0x53 nação, +0x71 continente: [0x9cfa10] pote 1,
/// [0x9cfa18]/[0x9cfa0c]/[0x9cfa1c] pote 2, [0x9cfa08]/[0x9cfa14] pote 3; sem nação ou continente, a caixa da linha
/// 0x61e) com o id do clube nas vagas +0xee (5 bytes); por fim as 24 da tabela (a partir da 9ª) e as vagas +0x116 são
/// ordenadas pelo pote (0x92f660).
unsafe extern "thiscall" fn mundial_cabecas_de_chave(this: usize) {
    if cqf6_w(this + 0x40) == 0x7d2 {
        crate::comparacao::ramo("cqf_6 sem cabeças de chave em 2002");
        return;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    cqf6_pb(this + 0x4b, 0);
    if cqf6_w(this + 0x36) < 0x20 {
        aviso_em(0xad5888, 0x59e);
        let mut k = 0i32;
        let mut desloc = 0usize;
        if cqf6_d(0xae2350) as i32 > 0 {
            while cqf6_w(this + 0x36) < 0x20 {
                let mut j = 0i16;
                let mut ja = false;
                while j < cqf6_w(this + 0x36) {
                    let c = principal(cqf6_d(0xae23a8).wrapping_add(desloc));
                    if cqf6_d(cqf6_d(this + 0x14) + j as usize * 6) == c {
                        ja = true;
                        break;
                    }
                    j += 1;
                }
                if !ja {
                    let c = principal(cqf6_d(0xae23a8).wrapping_add(desloc));
                    cqf6_pd(cqf6_d(this + 0x14) + cqf6_w(this + 0x36) as usize * 6, c);
                    cqf6_pw(this + 0x36, (cqf6_w(this + 0x36) as u16).wrapping_add(1));
                }
                k += 1;
                desloc += 0x122;
                if k >= cqf6_d(0xae2350) as i32 {
                    break;
                }
            }
        }
    }
    ordenar(cqf6_d(this + 0x14), cqf6_w(this + 0x36) as i32 as u32, 6, 0x4b07b0);
    // ponytail: as duas saídas começam em −1 (sem sede); a original as deixa com o resto da pilha quando a 0x5fa730 não
    // acha a Copa (ela só as grava achando)
    let (mut a, mut b) = (-1i32, -1i32);
    sedes(cqf6_d(0xb5e134), cqf6_d(0x9cf79c), cqf6_w(this + 0x40) as u16 as u32, &mut a, &mut b, 1);
    let sede1 = if a != -1 { principal(cqf6_d(0xae23a8).wrapping_add(a.wrapping_mul(0x122) as usize)) } else { 0 };
    let sede2 = if b != -1 { principal(cqf6_d(0xae23a8).wrapping_add(b.wrapping_mul(0x122) as usize)) } else { 0 };
    let detentor = campeao(cqf6_d(0xae23d0).wrapping_add((cqf6_d(0x9cf79c) as i32).wrapping_mul(0x6b) as usize), 0);
    let mut fixos = 0i8;
    for i in 0..32usize {
        for (vaga, alvo) in [(0usize, detentor), (1, sede1), (2, sede2)] {
            let t = cqf6_d(this + 0x14);
            let e = t + i * 6;
            if cqf6_d(e) == alvo {
                let (c, w) = (cqf6_d(e), cqf6_w(e + 4) as u16);
                let f = t + vaga * 6;
                cqf6_pd(e, cqf6_d(f));
                cqf6_pw(e + 4, cqf6_w(f + 4) as u16);
                let t = cqf6_d(this + 0x14) + vaga * 6;
                cqf6_pd(t, c);
                cqf6_pw(t + 4, w);
                if fixos < vaga as i8 + 1 {
                    fixos = vaga as i8 + 1;
                }
            }
        }
    }
    let mut n = fixos;
    if sede2 == sede1 || sede2 == detentor {
        n = 2;
    }
    if sede1 == detentor {
        n = 1;
    }
    ordenar(cqf6_d(this + 0x14) + n as i32 as usize * 6, (0x20 - n as i32) as u32, 6, 0x4b07b0);
    for i in 0..0x20usize {
        let clube = cqf6_d(cqf6_d(this + 0x14) + i * 6);
        let pote = this + 0xf2 + i * 5;
        cqf6_pd(pote - 4, cqf6_d(clube));
        if i < 8 {
            cqf6_pb(pote, 0);
            cqf6_pb(cqf6_d(this + 0x14) + i * 6 + 4, 3);
            continue;
        }
        let nacao = cqf6_d(clube + 0x53);
        let cont = if nacao != 0 { cqf6_d(nacao + 0x71) } else { 0 };
        if cont == 0 {
            aviso_em(0xad5888, 0x61e);
            continue;
        }
        let c = cqf6_d(cont);
        let (p, s) = if c == cqf6_d(0x9cfa10) {
            (1, 0xa)
        } else if c == cqf6_d(0x9cfa18) || c == cqf6_d(0x9cfa0c) || c == cqf6_d(0x9cfa1c) {
            (2, 0xb)
        } else if c == cqf6_d(0x9cfa08) || c == cqf6_d(0x9cfa14) {
            (3, 0xc)
        } else {
            continue;
        };
        cqf6_pb(pote, p);
        cqf6_pb(cqf6_d(this + 0x14) + i * 6 + 4, s);
    }
    ordenar(cqf6_d(this + 0x14) + 0x30, 0x18, 6, 0x92f660);
    ordenar(this + 0x116, 0x18, 5, 0x92f660);
}

/// Na Copa de 2002 (volta sem gravar nada) lado a lado em +0x4b; nas outras, só a original (caixas, e as saídas da
/// 0x5fa730 que a original pode deixar sem gravar).
unsafe extern "thiscall" fn comparar_mundial_cabecas_de_chave(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1611);
    if cqf6_w(this + 0x40) != 0x7d2 {
        return o(this);
    }
    let reg = [(this + 0x36, 0x16usize), (this + 0xee, 0xa0usize)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || mundial_cabecas_de_chave(this));
    PLACAR.registrar(igual, || "mundial_cabecas_de_chave: a Copa de 2002 mudou".to_string());
}

// ---------- 270a leva: a comissão de treino do técnico humano: as notas dos treinadores por categoria, os programas do elenco na troca de clube e os nomes padrão dos programas (training_manager.cpp) ----------

unsafe fn cqf7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf7_sb(a: usize) -> i8 {
    *(a as *const i8)
}

/// O registro da pessoa `id` (tabela [0xae23bc], 0x6e bytes).
unsafe fn cqf7_pessoa(id: i32) -> usize {
    cqf7_d(0xae23bc).wrapping_add((id as isize * 0x6e) as usize)
}

/// A vaga de 9 bytes da comissão (+0 id do treinador, +4..+8 as notas das cinco categorias) volta a vazia: −1, 0, 0.
unsafe fn cqf7_limpar(e: usize) {
    core::ptr::write_unaligned(e as *mut i32, -1);
    core::ptr::write_unaligned((e + 4) as *mut u32, 0);
    *((e + 8) as *mut u8) = 0;
}

/// Os divisores da nota pela quantidade de categorias do treinador (1 a 5), floats montados na pilha pela original:
/// 1,0; 1,2; 1,4; 1,6; 2,0.
const CQF7_DIVISORES: [u32; 5] = [0x3f80_0000, 0x3f99_999a, 0x3fb3_3333, 0x3fcc_cccd, 0x4000_0000];

/// A chance de conhecer (0x544410, cdecl: a ficha de não jogador e o byte do atributo; a original empurra o
/// registrador com lixo em cima: a chamada lê o byte).
unsafe fn cqf7_chance(f: usize, k: usize) -> i32 {
    let c: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    c(f, cqf7_sb(f + k)) as i8 as i32 // a original estende o al (movsx)
}

/// { 0x8b2880 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::update_coach_assignments(clube, técnico)`,
/// pelas contratações e saídas de treinadores (0x4ce4e0, 0x4cf720, 0x8c9720, a regeneração 0x7aaeb0) e pela troca de
/// clube (0x8b2470). Sem técnico e o clube sem humano (0x600a50 com 1, 1, 0, 0) ou clube de seleção (0x53b900): nada.
/// Técnico nulo: o do clube (+0xcf). A comissão do humano (0x8b4cd0, seis vagas de 9 bytes): (1) a vaga cujo
/// treinador não tem ficha de não jogador (+0x69) ou não é o auxiliar (0x53cb80) nem um dos 5 treinadores (0x53cc30)
/// fica vazia; (2) o auxiliar e os treinadores com ficha que não estão nas vagas entram na primeira vazia com as cinco
/// categorias (0x01010101 e 1); (3) cada categoria marcada recebe a nota: 0 (goleiros, ficha +0x10) (chance − ficha
/// +0x12 + 21) / 2; 1 (tática, +0x21 e +0x10) (chance + chance + +0x12) / 3; 2 (+0x10) (chance + +0xe + +0x12) / 3;
/// 3 (+0x10) (chance + +0x12) / 2; 4 (+0x11) a chance (divisões truncadas para zero; a chance 0x544410), o byte com
/// sinal dividido pelo divisor da quantidade de categorias marcadas (1,0 a 2,0, float), truncado e posto em 1..20.
unsafe extern "thiscall" fn distribuir_os_treinadores(this: usize, clube: usize, tecnico: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if tecnico == 0 && humano(clube, 1, 1, 0, 0) == 0 {
        return;
    }
    if selecao(clube) != 0 {
        return;
    }
    let tec = if tecnico == 0 { cqf7_d(clube + 0xcf) } else { tecnico };
    let comissao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b4cd0usize);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53cb80usize);
    let treinadores: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x53cc30usize);
    let t = comissao(this, tec);
    let aux = auxiliar(clube);
    let mut tr = [0usize; 5];
    treinadores(clube, tr.as_mut_ptr());
    let candidato = |k: i32| if k < 0 { aux } else { tr[k as usize] };
    for v in 0..6 {
        let e = t + 9 * v;
        let id = cqf7_i(e);
        if id == -1 {
            continue;
        }
        if cqf7_d(cqf7_pessoa(id) + 0x69) == 0 {
            cqf7_limpar(e);
            continue;
        }
        if !(-1..5).any(|k| {
            let p = candidato(k);
            p != 0 && cqf7_i(p) == id
        }) {
            cqf7_limpar(e);
        }
    }
    for k in -1..5 {
        let p = candidato(k);
        if p == 0 || cqf7_d(p + 0x69) == 0 {
            continue;
        }
        if (0..6).any(|v| cqf7_i(t + 9 * v) == cqf7_i(p)) {
            continue;
        }
        if let Some(v) = (0..6).find(|&v| cqf7_i(t + 9 * v) == -1) {
            let e = t + 9 * v;
            core::ptr::write_unaligned(e as *mut i32, cqf7_i(p));
            core::ptr::write_unaligned((e + 4) as *mut u32, 0x0101_0101);
            *((e + 8) as *mut u8) = 1;
        }
    }
    for v in 0..6 {
        let e = t + 9 * v;
        let id = cqf7_i(e);
        if id == -1 {
            continue;
        }
        let f = cqf7_d(cqf7_pessoa(id) + 0x69);
        if f == 0 {
            cqf7_limpar(e);
            continue;
        }
        let n = (0..5).filter(|&j| *((e + 4 + j) as *const u8) != 0).count();
        for j in 0..5 {
            if *((e + 4 + j) as *const u8) == 0 {
                continue;
            }
            let b = |k: usize| cqf7_sb(f + k) as i32;
            let x = match j {
                0 => (cqf7_chance(f, 0x10) - b(0x12) + 0x15) / 2, // cqf7: a nota de goleiros
                1 => {
                    let a = cqf7_chance(f, 0x21);
                    (a + cqf7_chance(f, 0x10) + b(0x12)) / 3 // cqf7: a nota de tática
                }
                2 => (cqf7_chance(f, 0x10) + b(0xe) + b(0x12)) / 3,
                3 => (cqf7_chance(f, 0x10) + b(0x12)) / 2,
                _ => cqf7_chance(f, 0x11),
            };
            // o byte de baixo, com sinal, em float dividido pelo float da quantidade (precisão dupla do x87)
            let q = ftol(x as i8 as f64 / f32::from_bits(CQF7_DIVISORES[n - 1]) as f64) as i8; // cqf7: o divisor da quantidade
            crate::comparacao::ramo("cqf_7 nota do treinador");
            *((e + 4 + j) as *mut i8) = q.clamp(1, 20);
        }
    }
}

/// Comparador: lado a lado na comissão do humano (54 bytes) e na marca +0x244 do clube (a 0x53cb80/0x53cc30 pela
/// 0x540a50 podem zerá-la). Sem humano ou clube de seleção: lado a lado só na marca. Técnico fora dos 16 humanos
/// (a 0x8b4cd0 mostra a caixa e a original escreveria no nulo): só a original.
unsafe extern "thiscall" fn comparar_distribuir_os_treinadores(this: usize, clube: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1612);
    if clube == 0 {
        return o(this, clube, tecnico);
    }
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let mut reg = vec![(clube + 0x244, 1)];
    if (tecnico != 0 || humano(clube, 1, 1, 0, 0) != 0) && selecao(clube) == 0 {
        let tec = if tecnico == 0 { cqf7_d(clube + 0xcf) } else { tecnico };
        if tec == 0 || !(0..0x10).contains(&cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1)) {
            return o(this, clube, tecnico);
        }
        reg.push((this + 0x652d + 54 * cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1) as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, tecnico), || distribuir_os_treinadores(this, clube, tecnico));
    PLACAR.registrar(igual, || format!("distribuir_os_treinadores(clube {:x}, tecnico {:x}) {:x?} x {:x?}", clube, tecnico, mo.0, mr.0));
}

/// O programa `vaga` (0..9; fora disso o 1) do técnico humano `tec`: this + (vaga + 7 + 10 × ([0xae2364] − id − 1)) ×
/// 0x9b; fora dos 16 humanos, a caixa "Error" (linha 0x68f) e 0 (a original lê então o byte +0x15 do nulo).
unsafe fn cqf7_programa(this: usize, tec: usize, vaga: i8) -> usize {
    let k = cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        aviso_em(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&vaga) { vaga as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// A entrada de 6 bytes do treino da ficha `f` (+0 a vaga do programa).
unsafe fn cqf7_entrada(this: usize, f: usize) -> usize {
    cqf7_d(this + 0x651d).wrapping_add((cqf7_i(f) as isize * 6) as usize)
}

/// { 0x8b2470 } (thiscall, ret 0xc, training_manager.cpp): `TRAINING_MANAGER::reassign_squad_schedules(técnico, antigo,
/// novo)`, quando o técnico humano troca de clube (0x690890 e 0x68f9c0/0x69ebc0, manager_manager.cpp). Elenco do clube
/// antigo (0x541f90, 50): cada pessoa válida (id abaixo de [0xae2364]) leva carga 50 (0x62c660) e a vaga 6 (goleiro de
/// 15 para cima, ficha +0xf) ou 1 na entrada do treino; sem ficha, a caixa (linha 0x34c). Com clube novo: dos dez
/// programas do técnico, o último de tipo 6 (+0x15) é o dos goleiros e o último de tipo 1 o geral; cada jogador do
/// elenco novo com ficha vai para o dos goleiros (goleiro de 15 para cima e havendo um), senão o geral (havendo), senão
/// a vaga 1, com a carga do programa (0x8b4f60) na ficha de forma; por fim a comissão (0x8b2880 com o clube novo e o
/// técnico).
unsafe extern "thiscall" fn trocar_os_programas_do_elenco(this: usize, tec: usize, antigo: usize, novo: usize) {
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let gravar_carga: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62c660usize);
    let (mut gol, mut geral) = (-1i8, -1i8);
    if antigo != 0 {
        let mut v = [0usize; 50];
        elenco(antigo, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || cqf7_i(p) >= cqf7_i(0xae2364) {
                continue;
            }
            gravar_carga(cqf7_d(0xb63d28), p, 0x32);
            let f = cqf7_d(p + 0x61);
            if f == 0 {
                aviso_em(0xa8ccd0, 0x34c);
                continue;
            }
            *(cqf7_entrada(this, f) as *mut u8) = if cqf7_sb(f + 0xf) >= 0xf { 6 } else { 1 };
        }
    }
    if novo == 0 {
        return;
    }
    for vaga in 0..10i8 {
        if *((cqf7_programa(this, tec, vaga) + 0x15) as *const u8) == 6 {
            gol = vaga;
        } else if *((cqf7_programa(this, tec, vaga) + 0x15) as *const u8) == 1 {
            geral = vaga;
        }
    }
    let mut v = [0usize; 50];
    elenco(novo, v.as_mut_ptr(), 0);
    let carga: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x8b4f60usize);
    for &p in &v {
        if p == 0 {
            continue;
        }
        let f = cqf7_d(p + 0x61);
        if f == 0 || cqf7_i(p) >= cqf7_i(0xae2364) {
            continue;
        }
        let e = cqf7_entrada(this, f) as *mut i8;
        *e = if cqf7_sb(f + 0xf) >= 0xf && gol != -1 {
            gol // cqf7: o programa dos goleiros
        } else if geral != -1 {
            geral
        } else {
            1
        };
        crate::comparacao::ramo("cqf_7 programa do elenco novo");
        let c = carga(cqf7_programa(this, tec, *e));
        gravar_carga(cqf7_d(0xb63d28), p, c as u32);
    }
    let comissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8b2880usize);
    comissao(this, novo, tec);
}

/// Comparador: lado a lado na entrada do treino e na carga de forma de cada jogador dos dois elencos, na comissão do
/// técnico (a 0x8b2880 do fim) e na marca +0x244 dos clubes (a 0x541f90 pela 0x540a50). Com caixa (jogador sem
/// ficha no elenco antigo ou técnico fora dos 16 humanos com clube novo): só a original.
unsafe extern "thiscall" fn comparar_trocar_os_programas_do_elenco(this: usize, tec: usize, antigo: usize, novo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1613);
    let k = if tec == 0 { -1 } else { cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1) };
    if novo != 0 && !(0..0x10).contains(&k) {
        return o(this, tec, antigo, novo);
    }
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let les = cqf7_d(cqf7_d(0xb63d28));
    let mut reg = Vec::new();
    for (clube, antigo_) in [(antigo, true), (novo, false)] {
        if clube == 0 {
            continue;
        }
        reg.push((clube + 0x244, 1));
        let mut v = [0usize; 50];
        elenco(clube, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || cqf7_i(p) >= cqf7_i(0xae2364) {
                continue;
            }
            let f = cqf7_d(p + 0x61);
            if f == 0 {
                if antigo_ {
                    return o(this, tec, antigo, novo);
                }
                continue;
            }
            reg.push((cqf7_entrada(this, f), 6));
            reg.push((les.wrapping_add((cqf7_i(p) as isize * 31 + 0x1e) as usize), 1));
        }
    }
    if novo != 0 {
        reg.push((this + 0x652d + 54 * k as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tec, antigo, novo), || trocar_os_programas_do_elenco(this, tec, antigo, novo));
    PLACAR.registrar(igual, || format!("trocar_os_programas_do_elenco(tecnico {:x}, {:x} -> {:x}) {:x?} x {:x?}", tec, antigo, novo, mo.0, mr.0));
}

/// { 0x8b2230 } (thiscall, ret 0, training_manager.cpp): os nomes padrão dos 167 programas de treino (0x8b50f0 em cada
/// um, de 0x9b em 0x9b bytes), pela criação dos programas padrão (0x8b3930).
unsafe extern "thiscall" fn nomes_padrao_dos_programas(this: usize) {
    let nome: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b50f0usize);
    for k in 0..0xa7 {
        nome(this + k * 0x9b);
    }
}

/// Comparador: lado a lado nos 167 programas (0x9b bytes cada).
unsafe extern "thiscall" fn comparar_nomes_padrao_dos_programas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1614);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(this, 0xa7 * 0x9b)], || o(this), || nomes_padrao_dos_programas(this));
    PLACAR.registrar(igual, || format!("nomes_padrao_dos_programas({:x})", this));
}

// ---------- 270a leva: o histórico da carreira do jogador (a tabela das temporadas por clube montada do arquivo de histórico) e os anos desde que jogou num clube (player_stats.cpp) ----------

unsafe fn cqf8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf8_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqf8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf8_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cqf8_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// O tamanho de uma entrada da tabela (uma categoria de uma temporada num clube): +0 o ano, +2 o id do clube (−1
/// nenhum), +6..+0x26 os 32 bytes da categoria (+6 jogos, +7 ..., +0x14 a palavra), +0x26 o valor da transferência
/// (−1 nenhum), +0x2a categoria × 8 e o tipo nos 3 bits de baixo. Seis entradas (categorias 1..6) por temporada.
const CQF8_E: usize = 0x2b;

/// A entrada montada na pilha pela original (0x2b bytes, zerada antes de cada uma).
struct Cqf8Reg([u8; CQF8_E]);

impl Cqf8Reg {
    fn nova(ano: u16, clube: u32, cat: u8, transf: u32) -> Cqf8Reg {
        let mut r = [0u8; CQF8_E];
        r[0..2].copy_from_slice(&ano.to_le_bytes());
        r[2..6].copy_from_slice(&clube.to_le_bytes());
        r[0x26..0x2a].copy_from_slice(&transf.to_le_bytes());
        r[0x2a] = cat << 3;
        Cqf8Reg(r)
    }
    unsafe fn gravar(&self, a: usize) {
        core::ptr::copy_nonoverlapping(self.0.as_ptr(), a as *mut u8, CQF8_E);
    }
}

/// A caixa "Error" de player_stats.cpp (0xa78310) na linha `linha`.
unsafe fn cqf8_erro(linha: u32) {
    aviso_em(0xa78310, linha);
}

/// A entrada `k` (contando de 1) a partir da conta `n`: buf + (n + k) × 0x2b − 0x2b.
fn cqf8_lugar(buf: usize, n: i32, k: i32) -> usize {
    buf.wrapping_add(((n + k) as isize * CQF8_E as isize - CQF8_E as isize) as usize)
}

/// { 0x7c3e40 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::build_career_history(pessoa, &conta)`: a tabela
/// (malloc 0x944e46 de 500 entradas, realloc 0x945667 de 500 em 500; quem chama libera com 0x9452ca) com seis entradas
/// por temporada e clube, e a conta em `*conta`. Pessoa nula (linha 0xb98) ou sem ficha (0xbaa): a caixa e 0; id a
/// partir de [0xae2364]: 0; sem memória: a caixa (0xbb2, 0xc00, 0xc26, 0xc41) e 0.
/// (1) A temporada atual: as seis categorias com o ano da temporada do clube (0x669950), o clube (+0x39; −1 sem ele), a
/// transferência −1 e os 32 bytes da categoria (0x7c3920) quando há. (2) O histórico do arquivo (FILE_LLIST::get_nodes
/// 0x58f120 do [this] pelas posições +0x18/+0x1c do registro de 0x27 bytes da ficha em [this+0x30]), do fim para o
/// começo (0x68d1d0): cada nó de outra temporada, clube ou tipo abre um grupo: a conta anda 6; anos vazios entre os dois
/// (até a conta passar de 500) viram grupos vazios (clube −1, transferência −1); o grupo do nó leva o ano, o clube, a
/// transferência e o tipo do nó; o nó vai para a entrada da sua categoria (+0x2a / 8) no grupo. A lista é destruída
/// (0x68d040 e 0x944c94). (3) A conta anda 6; cada entrada leva a transferência e o tipo da entrada seis adiante (as
/// do último grupo, −1 e 0). (4) Um grupo com um seguinte do mesmo ano e clube, sem transferência nos dois, tipo do
/// seguinte diferente de 1 e a palavra +0x14 diferente de −1 nos dois, soma os bytes +6..+0x13 e a palavra +0x14 no
/// seguinte e some (ano 0). (5) O grupo sem jogos (+6, +7 da primeira e da última entrada), com ano, sem transferência
/// e de tipo 0 some quando sem clube ou quando outro grupo tem o ano dele e o grupo seguinte é do mesmo clube; o de
/// jogos com a última entrada sem clube (+0xd9) também some. (6) Os grupos de ano 0 saem da tabela (os seguintes
/// descem) e a conta cai 6 por grupo.
unsafe extern "thiscall" fn historico_da_carreira(this: usize, p: usize, conta: *mut i32) -> usize {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
    let realloc: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
    let ano_do_clube: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x669950usize);
    let categoria: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    *conta = 0;
    if p == 0 {
        cqf8_erro(0xb98);
        return 0;
    }
    if cqf8_i(p) >= cqf8_i(0xae2364) {
        return 0;
    }
    let f = cqf8_d(p + 0x61);
    if f == 0 {
        cqf8_erro(0xbaa);
        return 0;
    }
    let mut buf = malloc(0x53fc);
    if buf == 0 {
        cqf8_erro(0xbb2);
        return 0;
    }
    let mut cap: i32 = 500;
    let clube = cqf8_d(p + 0x39);
    let mut ultimo = Cqf8Reg::nova(0, 0, 0, 0);
    for cat in 1..=6u8 {
        let ano = ano_do_clube(clube);
        let id = if clube == 0 { 0xffff_ffff } else { cqf8_d(clube) as u32 }; // cqf8: o clube da temporada atual
        crate::comparacao::ramo("cqf_8 temporada atual");
        let mut r = Cqf8Reg::nova(ano, id, cat, 0xffff_ffff);
        let c = categoria(this, f, cat as u32);
        if c != 0 {
            core::ptr::copy_nonoverlapping(c as *const u8, r.0.as_mut_ptr().add(6), 32);
        }
        r.gravar(cqf8_lugar(buf, *conta, cat as i32));
        ultimo = r;
    }
    // o ano, o clube e o tipo do grupo aberto (o tipo começa em 0xff: o primeiro nó sempre abre grupo)
    let mut ano_atual = u16::from_le_bytes([ultimo.0[0], ultimo.0[1]]);
    let mut clube_atual = u32::from_le_bytes([ultimo.0[2], ultimo.0[3], ultimo.0[4], ultimo.0[5]]);
    let mut tipo_atual: u8 = 0xff;
    let reg_ficha = cqf8_d(this + 0x30).wrapping_add((cqf8_i(f) as isize * 0x27) as usize);
    let nos: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x58f120usize);
    let lista = nos(cqf8_d(this), cqf8_d(reg_ficha + 0x18) as u32, cqf8_d(reg_ficha + 0x1c) as u32);
    if lista != 0 {
        let anterior: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1d0usize);
        cqf8_pd(lista + 0xa, cqf8_d(lista + 6) as u32);
        let mut no = anterior(lista);
        while no != 0 {
            let tipo = cqf8_b(no + 0x2a);
            if !(cqf8_w(no) == ano_atual && cqf8_d(no + 2) as u32 == clube_atual && tipo & 7 == tipo_atual) {
                *conta += 6;
                ano_atual = ano_atual.wrapping_sub(1);
                if ano_atual > cqf8_w(no) {
                    loop {
                        let mut tam = cap as usize * CQF8_E;
                        for cat in 1..=6u8 {
                            let r = Cqf8Reg::nova(ano_atual, 0xffff_ffff, cat, 0xffff_ffff);
                            if *conta + cat as i32 - 1 >= cap {
                                tam += 0x53fc;
                                cap += 500;
                                buf = realloc(buf, tam);
                                if buf == 0 {
                                    cqf8_erro(0xc00);
                                    return 0;
                                }
                            }
                            r.gravar(cqf8_lugar(buf, *conta, cat as i32));
                        }
                        *conta += 6;
                        if *conta > 500 {
                            break;
                        }
                        ano_atual = ano_atual.wrapping_sub(1);
                        if ano_atual <= cqf8_w(no) {
                            break;
                        }
                    }
                }
                let mut tam = cap as usize * CQF8_E;
                let ano_no = cqf8_w(no); // cqf8: o ano do grupo do nó
                let clube_no = cqf8_d(no + 2) as u32; // cqf8: o clube do grupo do nó
                for cat in 1..=6u8 {
                    let mut r = Cqf8Reg::nova(ano_no, clube_no, cat, cqf8_d(no + 0x26) as u32);
                    r.0[0x2a] = (cat << 3) | (tipo & 7);
                    crate::comparacao::ramo("cqf_8 temporada do historico");
                    if *conta + cat as i32 - 1 >= cap {
                        tam += 0x53fc;
                        cap += 500;
                        buf = realloc(buf, tam);
                        if buf == 0 {
                            cqf8_erro(0xc26);
                            return 0;
                        }
                    }
                    r.gravar(cqf8_lugar(buf, *conta, cat as i32));
                }
                ano_atual = cqf8_w(no);
                clube_atual = cqf8_d(no + 2) as u32;
                tipo_atual = tipo & 7;
            }
            if (tipo >> 3) as i32 - 1 + *conta >= cap {
                cap += 500;
                buf = realloc(buf, cap as usize * CQF8_E);
                if buf == 0 {
                    cqf8_erro(0xc41);
                    return 0;
                }
            }
            core::ptr::copy_nonoverlapping(no as *const u8, cqf8_lugar(buf, *conta, (tipo >> 3) as i32) as *mut u8, CQF8_E);
            no = anterior(lista);
        }
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(lista);
        apagar(lista);
    }
    *conta += 6;
    // (3) a transferência e o tipo descem do grupo seguinte
    let n = *conta;
    for i in 0..n {
        let e = buf + i as usize * CQF8_E;
        if i + 6 < n {
            cqf8_pd(e + 0x26, cqf8_d(e + 6 * CQF8_E + 0x26) as u32);
            *((e + 0x2a) as *mut u8) = (cqf8_b(e + 0x2a) & 0xf8) | (cqf8_b(e + 6 * CQF8_E + 0x2a) & 7);
        } else {
            cqf8_pd(e + 0x26, 0xffff_ffff);
            *((e + 0x2a) as *mut u8) = cqf8_b(e + 0x2a) & 0xf8;
        }
    }
    // (4) as entradas iguais do grupo seguinte somam e a de cima some
    let mut k = 6i32;
    while k - 6 < *conta {
        let ei = buf + (k - 6) as usize * CQF8_E;
        let mut j = k;
        while j < *conta {
            let ej = buf + j as usize * CQF8_E;
            if cqf8_w(ei) == cqf8_w(ej)
                && cqf8_b(ej + 0x2a) & 7 != 1
                && cqf8_i(ej + 0x26) == -1
                && cqf8_i(ei + 0x26) == -1
                && cqf8_d(ei + 2) == cqf8_d(ej + 2)
                && cqf8_w(ei + 0x14) != 0xffff
                && cqf8_w(ej + 0x14) != 0xffff
            {
                for o in 6..0x14 {
                    *((ej + o) as *mut u8) = cqf8_b(ej + o).wrapping_add(cqf8_b(ei + o)); // cqf8: soma da entrada repetida
                }
                cqf8_pw(ej + 0x14, cqf8_w(ej + 0x14).wrapping_add(cqf8_w(ei + 0x14)));
                if cqf8_i(ei + 0x26) != -1 || (cqf8_b(ei + 0x2a) & 7 == 1 && cqf8_i(ej + 0x26) == -1) {
                    cqf8_pd(ej + 0x26, cqf8_d(ei + 0x26) as u32);
                    *((ej + 0x2a) as *mut u8) = (cqf8_b(ej + 0x2a) & 0xf8) | (cqf8_b(ei + 0x2a) & 7);
                }
                cqf8_pw(ei, 0);
                break;
            }
            j += 6;
        }
        k += 1;
    }
    // (5) os grupos vazios somem (ano 0)
    let mut g = 0i32;
    while g < *conta {
        let e = buf + g as usize * CQF8_E;
        let zerar = if cqf8_b(e + 6) == 0
            && cqf8_b(e + 7) == 0
            && cqf8_b(e + 0xdd) == 0
            && cqf8_b(e + 0xde) == 0
            && cqf8_w(e) != 0
            && cqf8_i(e + 0x26) == -1
            && cqf8_b(e + 0x2a) & 7 == 0
        {
            let mut z = false;
            let mut j = 0i32;
            while j < *conta {
                if g != j {
                    let c = cqf8_i(e + 2);
                    if c == -1 {
                        z = true;
                        break;
                    }
                    let ej = buf + (j / 6) as usize * 0x102;
                    if cqf8_w(ej) == cqf8_w(e) && g + 6 < *conta && cqf8_i(e + 0x104) == c {
                        z = true;
                        break;
                    }
                }
                j += 6;
            }
            z
        } else {
            cqf8_i(e + 0xd9) == -1
        };
        if zerar {
            for t in 0..6 {
                cqf8_pw(e + t * CQF8_E, 0); // cqf8: o grupo some
            }
        }
        g += 6;
    }
    // (6) os grupos de ano 0 saem e os seguintes descem
    let mut i = 0i32;
    while i < *conta {
        let e = buf + (i / 6) as usize * 0x102;
        if cqf8_w(e) == 0 {
            let mut d = i + 6;
            let mut a = e + 0x102;
            while d < *conta {
                core::ptr::copy(a as *const u8, (a - 0x102) as *mut u8, 0x102);
                d += 6;
                a += 0x102;
            }
            *conta -= 6;
            i -= 6;
        }
        i += 6;
    }
    buf
}

/// Comparador: as duas montam a tabela (a original e a cópia, a partir do mesmo estado; o arquivo de histórico só é
/// lido) e comparam a conta e as entradas; a da cópia é liberada (0x9452ca) e o jogo segue com a da original. Os casos
/// de erro (caixa): só a original.
unsafe extern "thiscall" fn comparar_historico_da_carreira(this: usize, p: usize, conta: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize>(1615);
    if p == 0 || (cqf8_i(p) < cqf8_i(0xae2364) && cqf8_d(p + 0x61) == 0) {
        return o(this, p, conta);
    }
    let r = o(this, p, conta);
    let mut n = 0i32;
    let t = historico_da_carreira(this, p, &mut n);
    let igual = if r == 0 || t == 0 {
        r == t && *conta == n
    } else {
        *conta == n
            && core::slice::from_raw_parts(r as *const u8, n.max(0) as usize * CQF8_E)
                == core::slice::from_raw_parts(t as *const u8, n.max(0) as usize * CQF8_E)
    };
    PLACAR.registrar(igual, || {
        let k = (*conta).max(n).max(0) as usize;
        let dif = (0..k).find(|&i| {
            r == 0 || t == 0 || core::slice::from_raw_parts((r + i * CQF8_E) as *const u8, CQF8_E) != core::slice::from_raw_parts((t + i * CQF8_E) as *const u8, CQF8_E)
        });
        format!("historico_da_carreira(pessoa {:x}) conta original={} rust={}, primeira entrada diferente {:?}", p, *conta, n, dif)
    });
    if t != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        liberar(t);
    }
    r
}

/// { 0x7c6e30 } (stdcall, ret 8, player_stats.cpp): `player_stats_years_since_at_club(pessoa, clube)`: quantos anos
/// desde que a pessoa jogou pelo clube: 0 se está nele (+0x39), −1 se nunca; senão o ano de hoje (0xae2c92) menos o
/// maior ano dos grupos do histórico (0x7c3e40 em [0xb74350]; a última entrada de cada grupo, +0xd7 o ano, +0xd9 o
/// clube) com o clube. Pessoa nula (linha 0x133f), clube nulo (0x1347) ou sem ficha (0x135f): a caixa e −1; id a partir
/// de [0xae2364]: −1. Quem chama a põe com o ecx do PlayerStats, que ela não usa. Pela 0x5b03e0 (não roda na carreira
/// automática: passageira).
unsafe extern "stdcall" fn anos_desde_o_clube(p: usize, clube: usize) -> i16 {
    if p == 0 {
        cqf8_erro(0x133f);
        return -1;
    }
    if clube == 0 {
        cqf8_erro(0x1347);
        return -1;
    }
    if cqf8_i(p) >= cqf8_i(0xae2364) {
        return -1;
    }
    if cqf8_d(p + 0x39) == clube {
        return 0;
    }
    if cqf8_d(p + 0x61) == 0 {
        cqf8_erro(0x135f);
        return -1;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = montar(cqf8_d(0xb74350), p, &mut n);
    let mut maior = 0x7b3i32;
    if t != 0 {
        let mut i = 0i32;
        let mut e = t + 0xd7;
        while i < n {
            let c = cqf8_i(e + 2);
            if c >= 0 && c == cqf8_i(clube) && cqf8_w(e) as i32 > maior {
                maior = cqf8_w(e) as i32;
            }
            i += 6;
            e += 0x102;
        }
    }
    liberar(t);
    if maior == 0x7b3 {
        return -1;
    }
    cqf8_w(0xae2c92).wrapping_sub(maior as u16) as i16
}

/// Comparador: a devolução (16 bits); a tabela é montada e liberada pelas duas. Os casos de erro: só a original.
unsafe extern "stdcall" fn comparar_anos_desde_o_clube(p: usize, clube: usize) -> i16 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> i16>(1616);
    if p == 0 || clube == 0 || (cqf8_i(p) < cqf8_i(0xae2364) && cqf8_d(p + 0x39) != clube && cqf8_d(p + 0x61) == 0) {
        return o(p, clube);
    }
    let r = o(p, clube);
    let n = anos_desde_o_clube(p, clube);
    PLACAR.registrar(r == n, || format!("anos_desde_o_clube(pessoa {:x}, clube {:x}) original={} rust={}", p, clube, r, n));
    r
}

/// { 0x7c6de0 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_staff(a, b)`, a ordem do qsort dos
/// registros do staff_history.dat na criação das estatísticas (0x7bfd50, ponteiro em 0x7c05ad): `a` nulo 1, `b` nulo −1;
/// pela pessoa (+4, sem sinal), depois pelo ano (+8, palavra com sinal), depois a diferença do primeiro dword.
unsafe extern "C" fn ordem_do_historico_por_pessoa(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (pa, pb) = (cqf8_d(a + 4) as u32, cqf8_d(b + 4) as u32);
    if pa > pb {
        return 1;
    }
    if pa < pb {
        return -1;
    }
    let (aa, ab) = (cqf8_w(a + 8) as i16, cqf8_w(b + 8) as i16);
    if aa > ab {
        return 1;
    }
    if aa != ab {
        return -1;
    }
    cqf8_i(a).wrapping_sub(cqf8_i(b))
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_do_historico_por_pessoa(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1617)(a, b);
    let n = ordem_do_historico_por_pessoa(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_historico_por_pessoa({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 270a leva: a lesão pelo nome do player_setup.cfg; a notícia da volta aos treinos e o que ela acerta (o tratamento, as férias e a moral do jogador, a relação com o técnico, a fuga) e o nome da categoria da lesão (player_regen.cpp, injury.cpp) ----------

/// Os nomes das lesões do player_setup.cfg (a comparação, os caracteres comparados, o texto no .rdata e o tipo), na
/// ordem da original; o último, ARTHRITIS (8 caracteres, 0xa6b3a8), dá 0x4c, e nenhum, 0x4b.
const CQF9_LESOES: [(u32, usize, u8); 75] = [(4, 0xa6b840, 0), (3, 0xa6b83c, 1), (5, 0xa6b834, 2), (10, 0xa6b824, 3), (10, 0xa6b80c, 4), (10, 0xa6b7fc, 5), (10, 0xa6b7ec, 6), (10, 0xa6b7dc, 7), (10, 0xa6b7d0, 8), (10, 0xa6b7c0, 9), (10, 0xa6b7b4, 10), (15, 0xa6b7a0, 11), (15, 0xa6b788, 12), (10, 0xa6b778, 80), (10, 0xa6b768, 13), (10, 0xa6b758, 14), (10, 0xa6b74c, 15), (10, 0xa6b73c, 16), (10, 0xa6b730, 17), (10, 0xa6b71c, 18), (10, 0xa6b70c, 19), (10, 0xa6b6fc, 20), (10, 0xa6b6ec, 21), (10, 0xa6b6dc, 22), (10, 0xa6b6c8, 23), (10, 0xa6b6b4, 24), (10, 0xa6b6a0, 25), (10, 0xa6b690, 26), (10, 0xa6b678, 27), (10, 0xa6b664, 28), (10, 0xa6b654, 29), (10, 0xa6b644, 30), (10, 0xa6b634, 31), (10, 0xa6b628, 32), (10, 0xa6b618, 33), (10, 0xa6b608, 34), (10, 0xa6b5f8, 35), (10, 0xa6b5e4, 36), (5, 0xa6b5d8, 37), (10, 0xa6b5c8, 38), (10, 0xa6b5bc, 39), (10, 0xa6b5ac, 40), (10, 0xa6b5a0, 41), (14, 0xa6b590, 42), (10, 0xa6b584, 43), (10, 0xa6b574, 44), (10, 0xa6b564, 45), (10, 0xa6b554, 46), (5, 0xa6b548, 47), (10, 0xa6b538, 48), (10, 0xa6b52c, 49), (10, 0xa6b51c, 50), (10, 0xa6b50c, 52), (15, 0xa6b4fc, 53), (5, 0xa6b4f0, 54), (13, 0xa6b4e0, 55), (10, 0xa6b4d0, 56), (10, 0xa6b4c4, 57), (10, 0xa6b4b0, 58), (10, 0xa6b49c, 59), (10, 0xa6b48c, 60), (10, 0xa6b47c, 61), (10, 0xa6b46c, 62), (10, 0xa6b458, 63), (10, 0xa6b448, 64), (10, 0xa6b438, 65), (5, 0xa6b42c, 66), (10, 0xa6b420, 67), (5, 0xa6b414, 68), (5, 0xa6b408, 69), (13, 0xa6b3f8, 70), (10, 0xa6b3e8, 71), (19, 0xa6b3d4, 72), (10, 0xa6b3c0, 73), (10, 0xa6b3b4, 74)];

/// strncmp (0x946fa0, cdecl).
unsafe fn cqf9_strncmp(a: usize, b: usize, n: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x946fa0usize);
    f(a, b, n)
}

/// { 0x7a32e0 } (cdecl, player_regen.cpp): `player_setup_injury_from_name(nome)`: o tipo da lesão (0..0x4c) pelo nome
/// da linha de lesão do player_setup.cfg (strncmp com os 76 nomes, cada um pelo seu tamanho, na ordem; o primeiro que
/// bate); nenhum: 0x4b. Pela aplicação das linhas (0x7a3a10) na criação do jogo. A devolução vale pelo al (a original
/// deixa o resto do eax do strncmp).
unsafe extern "C" fn lesao_pelo_nome(nome: usize) -> u32 {
    for &(n, texto, tipo) in CQF9_LESOES.iter() {
        if cqf9_strncmp(nome, texto, n) == 0 {
            crate::comparacao::ramo("cqf_9 lesao pelo nome");
            return tipo as u32; // cqf9: o tipo da lesão pelo nome
        }
    }
    if cqf9_strncmp(nome, 0xa6b3a8, 8) == 0 { 0x4c } else { 0x4b }
}

/// Comparador: a devolução (o al; só lê).
unsafe extern "C" fn comparar_lesao_pelo_nome(nome: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(1618)(nome);
    let n = lesao_pelo_nome(nome);
    PLACAR.registrar(r as u8 == n as u8, || format!("lesao_pelo_nome({:x}) original={:x} rust={:x}", nome, r as u8, n as u8));
    r
}

unsafe fn cqf9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf9_sb(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqf9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O ensaio do comparador: a cópia roda sem a entrega da notícia (0x786090, que aloca) e para antes da fuga (0x419840,
/// que põe na lista e manda notícia), guardando a decisão.
static mut CQF9_ENSAIO: bool = false;
static mut CQF9_FUGIU: bool = false;

/// O registro de forma do jogador na tabela das lesões ([this] + 31 × id).
unsafe fn cqf9_forma(this: usize, p: usize) -> usize {
    cqf9_d(this).wrapping_add((*(p as *const i32) as isize * 31) as usize)
}

/// Gosta/não gosta do técnico (0x547810/0x547840) e tirar dos favoritos/rejeitados (0x547c10/0x547c50), cdecl.
unsafe fn cqf9_pref(rotina: usize, p: usize, tec: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(rotina);
    f(p, tec)
}

/// Acrescentar aos favoritos (0x547a10) ou aos rejeitados (0x547af0), cdecl(pessoa, outra, 1).
unsafe fn cqf9_acrescentar(rotina: usize, p: usize, tec: usize) {
    let f: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(rotina);
    f(p, tec, 1);
}

/// O técnico gostar mais: tira dos rejeitados e, com sorteio(+0x59) acima de 5, põe nos favoritos.
unsafe fn cqf9_gostar_mais(p: usize, clube: usize) {
    if cqf9_d(clube + 0xcf) != 0 && cqf9_pref(0x547840, p, cqf9_d(clube + 0xcf)) != 0 {
        cqf9_pref(0x547c50, p, cqf9_d(clube + 0xcf));
    }
    if cqf9_d(clube + 0xcf) != 0 && cqf9_sortear(cqf9_sb(p + 0x59) as i32) > 5 {
        cqf9_acrescentar(0x547a10, p, cqf9_d(clube + 0xcf));
    }
}

/// { 0x630ee0 } (thiscall, ret 0x14, injury.cpp, quadro SEH da notícia local): `Injury::post_recovery_news(jogador,
/// estágio, tipo, forçar, dias)`, pelas voltas aos treinos (0x62e010) e pelo fim da lesão (0x62e700). Sem jogador:
/// nada; sem forçar, só com clube de técnico humano (0x7875e0). A notícia 2001 (0x77bbe0) com o estágio, o tipo, o id,
/// os três nomes, o nascimento, o ano e os dias (0x785790, dados 0..8). Tipo de categoria 12 (a tabela de 13 bytes em
/// 0x9e7ad9): sorteio(32000) e: tipo 'Q' (férias): com contrato (0x4cf5a0), ficha e clube, cada marca de férias do
/// contrato (+0x45: 0x20000, 0x20, 0x40) cai (0x8674e0) com sorteio(+0x5b) > sorteio(25); sem nenhuma cair, sorteio(+0x5b)
/// 0 leva à chance de fuga (com o gerente das faltas [0xad9c6c] e +0x59 abaixo de 12, sorteio(+0x59) < sorteio(5): a
/// fuga 0x419840 com 4 e a notícia não sai); senão a felicidade do contrato (+0x44) sobe 15 (−100..100) e, com marca
/// caída, o técnico gosta mais (resultado 2); depois sorteio(+0x5b + +0x59, − 5 com valor +0xb da ficha acima de 7250) <
/// sorteio(20) e o sorteio(4) acerta +0x57, +0x5d, +0x44 da ficha e +0x59. Estágio 0, tipo 'M' (tratamento): 0 dá
/// resultado 2 (forma +0x12 = 0xff, +0x13 = 0); abaixo de 5, resultado 1 (+0x13 = 0); senão resultado 0 e, com +0x5d
/// abaixo de 5, o técnico pode perder a simpatia (0x547810/0x547c10, sorteio(+0x5d) 0: rejeitado 0x547af0); +0x59 (ou
/// +0x57) acima de 10: o técnico gosta mais. Estágio 1, tipos 'O' e 'N': o resultado pelo sorteio, a forma e a marca
/// +0xc do registro de 79 bytes ([0xae2d0c]) cai. Dados 9 (resultado) e 10; a entrega pelo alcance `forcar` (0x786090:
/// o clube, a nação do clube ou da pessoa e o continente dela).
unsafe extern "thiscall" fn noticia_da_volta_aos_treinos(this: usize, p: usize, estagio: u32, tipo: u32, forcar: u32, dias: u32) {
    if p == 0 {
        return;
    }
    if forcar as u8 == 0 {
        let clube = cqf9_d(p + 0x39);
        if clube == 0 {
            return;
        }
        let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
        if humano(clube) == 0 {
            return;
        }
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x7d1, 0);
    let est = estagio as u8 as i8;
    let tp = tipo as u8 as i8;
    dado(n, 0, est as i32 as u32);
    dado(n, 1, tp as i32 as u32);
    dado(n, 2, cqf9_d(p) as u32);
    dado(n, 3, cqf9_d(cqf9_d(p + 4) + 0x33) as u32);
    dado(n, 4, cqf9_d(cqf9_d(p + 8) + 0x33) as u32);
    dado(n, 5, cqf9_d(cqf9_d(p + 0xc) + 0x33) as u32);
    dado(n, 6, *((p + 0x10) as *const i16) as i32 as u32);
    dado(n, 7, *((p + 0x12) as *const u16) as u32);
    dado(n, 8, dias as u16 as i16 as i32 as u32);
    let clube = cqf9_d(p + 0x39);
    let mut res: i8 = -1;
    let mut f1: i8 = 0;
    if *((0x9e7ad9 + tp as isize * 13) as *const u8) == 0xc {
        let s = cqf9_sortear(32000) as i16;
        if tp == 0x51 {
            res = 0;
            let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contrato_de(cqf9_d(0xae1050), p);
            if c == 0 || cqf9_d(p + 0x61) == 0 || clube == 0 {
                f1 = 1;
            } else {
                let tirar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
                let mut caiu = false;
                for marca in [0x20000u32, 0x20] {
                    if cqf9_d(c + 0x45) as u32 & marca != 0 {
                        let a = cqf9_sortear(cqf9_sb(p + 0x5b) as i32);
                        let b = cqf9_sortear(0x19);
                        if a > b {
                            tirar(c, marca);
                            caiu = true;
                        }
                    }
                }
                let mut feliz = caiu;
                if cqf9_d(c + 0x45) as u32 & 0x40 != 0 {
                    let a = cqf9_sortear(cqf9_sb(p + 0x5b) as i32);
                    let b = cqf9_sortear(0x19);
                    if a > b {
                        tirar(c, 0x40);
                        caiu = true;
                        feliz = true;
                    }
                }
                if !feliz && cqf9_sortear(cqf9_sb(p + 0x5b) as i32) != 0 {
                    feliz = true;
                }
                let mut seguir = true;
                if feliz {
                    let v = cqf9_sb(c + 0x44) as i32 + 0xf; // cqf9: a felicidade sobe 15
                    crate::comparacao::ramo("cqf_9 felicidade pelas ferias");
                    *((c + 0x44) as *mut i8) = if v > 100 { 100 } else if v < -100 { -100 } else { v as i8 };
                }
                if feliz && caiu {
                    res = 2;
                    cqf9_gostar_mais(p, clube);
                } else {
                    if cqf9_d(0xad9c6c) != 0 && cqf9_sb(p + 0x59) < 0xc {
                        let a = cqf9_sortear(cqf9_sb(p + 0x59) as i32);
                        let b = cqf9_sortear(5);
                        if a < b {
                            if CQF9_ENSAIO {
                                CQF9_FUGIU = true;
                                return;
                            }
                            let fugir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x419840usize);
                            fugir(cqf9_d(0xad9c6c), p, clube, 4);
                            return;
                        }
                    }
                    res = 0;
                    seguir = true;
                }
                if seguir {
                    let mut x = (cqf9_sb(p + 0x5b) as i16).wrapping_add(cqf9_sb(p + 0x59) as i16);
                    f1 = 1;
                    if *((cqf9_d(p + 0x61) + 0xb) as *const i16) > 0x1c52 {
                        x = x.wrapping_sub(5);
                    }
                    let a = cqf9_sortear(x as i32);
                    let b = cqf9_sortear(0x14);
                    if a < b {
                        let r = cqf9_sortear(4);
                        let f = cqf9_d(p + 0x61);
                        let subir_57 = |p: usize| {
                            if cqf9_sb(p + 0x57) < 0x14 {
                                *((p + 0x57) as *mut i8) += 1;
                            }
                        };
                        let descer_5d_44 = |p: usize, f: usize| {
                            if cqf9_sb(p + 0x5d) > 1 {
                                *((p + 0x5d) as *mut i8) -= 1;
                            }
                            if cqf9_sb(f + 0x44) > 1 {
                                *((f + 0x44) as *mut i8) -= 1;
                            }
                        };
                        crate::comparacao::ramo("cqf_9 acerto pelas ferias");
                        if r == 0 {
                            subir_57(p); // cqf9: o sorteio 0 sobe +0x57
                            descer_5d_44(p, f);
                        } else if r == 1 {
                            subir_57(p);
                        } else {
                            descer_5d_44(p, f);
                        }
                        if r == 0 || r == 1 {
                            if cqf9_sb(p + 0x59) > 1 {
                                *((p + 0x59) as *mut i8) -= 1;
                            }
                        }
                    }
                }
            }
        } else if est == 0 {
            if tp != 0x4d {
                res = -1;
            } else {
                let forma = cqf9_forma(this, p);
                let mut v: Option<i8> = None;
                crate::comparacao::ramo("cqf_9 tratamento");
                if s == 0 { // cqf9: o tratamento dá certo
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                    v = Some(cqf9_sb(p + 0x59));
                } else if s < 5 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                    if cqf9_sb(p + 0x59) > 0xf {
                        v = Some(cqf9_sb(p + 0x57));
                    }
                } else {
                    res = 0;
                    if cqf9_sb(p + 0x5d) < 5 {
                        f1 = -1;
                        if clube != 0 {
                            if cqf9_d(clube + 0xcf) != 0 && cqf9_pref(0x547810, p, cqf9_d(clube + 0xcf)) != 0 {
                                cqf9_pref(0x547c10, p, cqf9_d(clube + 0xcf));
                            }
                            if cqf9_d(clube + 0xcf) != 0 && cqf9_sortear(cqf9_sb(p + 0x5d) as i32) == 0 {
                                cqf9_acrescentar(0x547af0, p, cqf9_d(clube + 0xcf));
                            }
                        }
                    }
                }
                if let Some(v) = v {
                    if v > 10 {
                        f1 = 1;
                        if clube != 0 {
                            cqf9_gostar_mais(p, clube);
                        }
                    }
                }
            }
        } else {
            let forma = cqf9_forma(this, p);
            crate::comparacao::ramo("cqf_9 volta da lesao");
            if tp == 0x4f {
                if s < 30000 { // cqf9: a volta 'O' completa
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                } else if cqf9_sortear(5) != 0 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                } else {
                    res = 3;
                    let a = forma + 0x13;
                    let r = cqf9_sortear(4);
                    *(a as *mut u8) = (*(a as *const u8)).wrapping_add((r as u8).wrapping_add(1));
                }
            } else if tp == 0x4e {
                if s < 0x733c { // cqf9: a volta 'N' completa
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                } else if s < 30000 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                } else {
                    res = 0;
                }
            }
            let e = cqf9_d(0xae2d0c).wrapping_add((*(p as *const i32) as isize * 0x4f + 0xc) as usize);
            *(e as *mut u8) &= 0xfe; // cqf9: a marca +0xc cai
        }
    }
    dado(n, 9, res as i32 as u32);
    dado(n, 10, f1 as i32 as u32);
    if CQF9_ENSAIO {
        return;
    }
    let nacao = if clube != 0 { cqf9_d(clube + 0x53) } else { cqf9_d(p + 0x1a) };
    let continente = if nacao != 0 { cqf9_d(nacao + 0x71) } else { 0 };
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    entregar(cqf9_d(0xb7431c), n, clube, 0, nacao, continente, forcar);
}

/// Comparador em ensaio (modelo CPV6_ENSAIO): a cópia roda primeiro sem a entrega da notícia e parando antes da fuga,
/// depois a original a partir do mesmo estado; compara o registro do jogador (0x6e), a ficha (0x46), o contrato (0x4f),
/// o registro de forma (31), o byte +0xc do registro de 79 bytes, o registro do técnico do clube e o estado do sorteio.
/// Com a fuga na cópia (a original põe na lista, sorteia e manda notícia): só a original.
unsafe extern "thiscall" fn comparar_noticia_da_volta_aos_treinos(this: usize, p: usize, estagio: u32, tipo: u32, forcar: u32, dias: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1619);
    if p == 0 {
        return o(this, p, estagio, tipo, forcar, dias);
    }
    let mut reg: Vec<(usize, usize)> = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.push((p, 0x6e));
    if cqf9_d(p + 0x61) != 0 {
        reg.push((cqf9_d(p + 0x61), 0x46));
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let antes_1080 = cqf9_d(0xae1080);
    let c = contrato_de(cqf9_d(0xae1050), p);
    *(0xae1080 as *mut usize) = antes_1080;
    if c != 0 {
        reg.push((c, 0x4f));
    }
    reg.push((0xae1080, 4));
    reg.push((cqf9_forma(this, p), 31));
    reg.push((cqf9_d(0xae2d0c).wrapping_add((*(p as *const i32) as isize * 0x4f) as usize), 0x4f));
    let clube = cqf9_d(p + 0x39);
    if clube != 0 && cqf9_d(clube + 0xcf) != 0 {
        reg.push((cqf9_d(clube + 0xcf), 0x6e));
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQF9_ENSAIO = true;
    CQF9_FUGIU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| noticia_da_volta_aos_treinos(this, p, estagio, tipo, forcar, dias)));
    CQF9_ENSAIO = false;
    let fugiu = CQF9_FUGIU;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, p, estagio, tipo, forcar, dias);
    if fugiu {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("noticia_da_volta_aos_treinos(pessoa {:x}, estagio {}, tipo {:x}) {:x?} x {:x?}", p, estagio as u8, tipo as u8, mo.0, mr.0));
}

/// { 0x630c50 } (stdcall, ret 0xc, injury.cpp): `injury_get_category_name(categoria, texto, tamanho)`: o nome traduzido
/// (0x66f4e0) da categoria da lesão 0..12 (interna, pé, canela, joelho, coxa, virilha, tronco, mão, braço, ombro,
/// pescoço, cabeça, recuperação); fora disso a caixa (linha 0x10e7). Devolve 2 com o idioma 3 (0x66f4d0), senão 0 (o
/// gênero do nome). Tamanho abaixo de 20: a caixa (linha 0x10d0) e o al zerado.
unsafe extern "stdcall" fn nome_da_categoria_da_lesao(categoria: u32, texto: usize, tamanho: i32) -> u32 {
    if tamanho < 0x14 {
        aviso_em(0x9e7f28, 0x10d0);
        return 0;
    }
    const NOMES: [usize; 13] = [0x9e89ac, 0x9e89a0, 0x9e8994, 0x9e8988, 0x9e8978, 0x9e8968, 0x9e8954, 0x9e8948, 0x9e893c, 0x9e892c, 0x9e8920, 0x9e8914, 0x9e8904];
    let k = categoria as u8 as i8 as i32 as u32;
    if k <= 0xc {
        let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
        traduzir(texto, NOMES[k as usize]);
    } else {
        aviso_em(0x9e7f28, 0x10e7);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    if idioma() == 3 { 2 } else { 0 }
}

/// Comparador: lado a lado no texto (o tamanho, até 0x100) e na devolução; o tamanho curto (a caixa): só a original.
unsafe extern "stdcall" fn comparar_nome_da_categoria_da_lesao(categoria: u32, texto: usize, tamanho: i32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(u32, usize, i32) -> u32>(1620);
    if tamanho < 0x14 || texto == 0 {
        return o(categoria, texto, tamanho);
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(texto, (tamanho as usize).min(0x100))], || o(categoria, texto, tamanho), || nome_da_categoria_da_lesao(categoria, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("nome_da_categoria_da_lesao({}) original={} rust={}", categoria as u8 as i8, r, n));
    r
}

// ---------- 270a leva: o olheiro na partida observada e o histórico do clube observado (player_search.cpp) ----------

unsafe fn cqf10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqf10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf10_pessoa(id: usize) -> usize {
    cqf10_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf10_clube(id: usize) -> usize {
    cqf10_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) ou 0.
unsafe fn cqf10_clube_do_contrato(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let c = contrato_de(cqf10_d(0xae1050), pessoa);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    if c != 0 { cqf10_clube(cqf10_d(c + 4)) } else { 0 }
}

/// { 0x7bd310 } (thiscall, ret 8, player_search.cpp): `player_search_record_scouted_club_match(jogo, resultado)`, pela
/// partida observada (0x7b5be0) quando a lista acompanha um clube (+0x190) que joga nela. `this` é o registro do clube
/// observado ([lista+0x170]: +0 o id do clube, 50 vagas de 0x1a2 bytes a partir de +4). Primeiro as vagas de quem saiu
/// do clube (id −1, o contador +4 e os bytes +0x1a0/+0x1a1 zerados); depois, do lado do clube no resultado (+0x190, 20
/// registros de 0x49, o lado de fora a 0x625), cada jogador que jogou (+4 ≠ −1; reservas além do 11º só com nota, +0x32)
/// ganha uma linha no histórico de 10 partidas da vaga dele (0x7bd700 com 1; sem vaga, a caixa da linha 0x2065): gols,
/// nota, data, adversário, placar, competição, o lado (0x4afa80) e as estatísticas; o tipo +0x2e (1 titular, 2 reserva,
/// 5/6 com a nota pelo +0x30) e a nota +0x15. Com o técnico do clube humano e as máscaras de atributos ligadas, o
/// técnico passa a conhecer o jogador (0x5b09d0, que sorteia).
unsafe extern "thiscall" fn cqf10_historico_do_clube_observado(this: usize, jogo: usize, resultado: usize) {
    let clube_obs = cqf10_clube(cqf10_d(this));
    for i in 0..0x32usize {
        let v = this + 4 + i * 0x1a2;
        let id = cqf10_i(v);
        if id == -1 {
            continue;
        }
        let p = cqf10_pessoa(id as usize);
        if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
            continue;
        }
        let c = cqf10_d(p + 0x39);
        if c != 0 && cqf10_d(c) == cqf10_d(this) {
            continue;
        }
        core::ptr::write_unaligned(v as *mut i32, -1);
        core::ptr::write_unaligned((v + 4) as *mut u16, 0);
        *((v + 0x1a0) as *mut u8) = 0;
        *((v + 0x1a1) as *mut u8) = 0;
    }
    // o lado do clube: 0 se ele é o +0xc do jogo (o adversário é o +0x10), senão 1 (adversário +0xc)
    let (lado, adversario): (i8, u32) = if cqf10_d(jogo + 0xc) == cqf10_d(this) {
        (0, cqf10_d(jogo + 0x10) as u32)
    } else {
        (1, cqf10_d(jogo + 0xc) as u32)
    };
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let quadro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5b09d0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7bd700usize);
    let lado_do_jogo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x4afa80usize);
    // 0x600a50: 5 pushes (clube, 0, 1, 0, 0); add esp, 0x14 -> cdecl; 0x5afde0: push técnico; mov ecx, [0xae2a60] ->
    // thiscall; 0x5b09d0: push 0; push jogador; mov ecx, eax -> thiscall, ret 8; 0x7bd700: push 1; push jogador; mov ecx,
    // this -> thiscall, ret 8; 0x4afa80: push jogo; push clube; add esp, 8 -> cdecl
    for j in 0..0x14i8 {
        let e = resultado + 0x190 + lado as usize * 0x625 + j as usize * 0x49;
        let id = cqf10_i(e);
        if id == -1 || cqf10_i(e + 4) == -1 {
            continue;
        }
        if j > 10 && cqf10_b(e + 0x32) == 0xff {
            continue;
        }
        let p = cqf10_pessoa(id as usize);
        if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
            continue;
        }
        if cqf10_d(clube_obs + 0xcf) != 0 && humano(clube_obs, 0, 1, 0, 0) != 0 && cqf10_b(0x9cc270) != 0 {
            let q = quadro(cqf10_d(0xae2a60), cqf10_d(clube_obs + 0xcf));
            if q != 0 {
                conhecer(q, p, 0);
            }
        }
        let r = vaga_de(this, p, 1);
        if r == 0 {
            aviso_em(0xa701b0, 0x2065);
            return;
        }
        let quantos = cqf10_b(r + 0x1a1);
        core::ptr::write_unaligned((r + 4) as *mut u16, (cqf10_w(r + 4) as u16).wrapping_add(1));
        let mut k = cqf10_b(r + 0x1a0);
        if quantos == 0xa {
            // o histórico cheio: a linha mais antiga (+0x1a0) é a da vez, e o começo anda
            let n = k.wrapping_add(1);
            *((r + 0x1a0) as *mut u8) = if n == 0xa { 0 } else { n };
        } else {
            k = k.wrapping_add(quantos);
            if (k as i8) >= 0xa {
                k = k.wrapping_add(0xf6);
            }
            *((r + 0x1a1) as *mut u8) = quantos.wrapping_add(1);
        }
        let l = r.wrapping_add((k as i8 as isize * 0x29) as usize);
        let por = |o: usize, v: u8| *((l + o) as *mut u8) = v;
        por(0x17, cqf10_b(e + 0x27));
        por(0x18, cqf10_b(e + 0x22));
        core::ptr::write_unaligned((l + 6) as *mut u16, cqf10_w(jogo + 0x2a) as u16);
        core::ptr::write_unaligned((l + 8) as *mut u16, cqf10_w(jogo + 0x28) as u16);
        por(0x16, cqf10_b(e + 0x21));
        por(0x1b, cqf10_b(e + 0x35));
        por(0x1a, cqf10_b(e + 0x2d));
        por(0x19, cqf10_b(e + 0x2c));
        let (m, n) = if lado == 0 { (0x43, 0x44) } else { (0x44, 0x43) };
        por(0xe, cqf10_b(jogo + m));
        por(0xf, cqf10_b(jogo + n));
        core::ptr::write_unaligned((l + 0x10) as *mut u32, cqf10_d(jogo + 8) as u32);
        por(0x14, lado_do_jogo(cqf10_clube(cqf10_d(this)), jogo) as u8);
        for t in 0..10 {
            por(0x1c + t, cqf10_b(e + 0x3e + t));
        }
        por(0x26, cqf10_b(e + 0x28));
        por(0x27, cqf10_b(e + 0x25));
        por(0x28, cqf10_b(e + 0x26));
        por(0x29, cqf10_b(e + 0x29));
        por(0x2a, cqf10_b(e + 0x2a));
        por(0x2b, cqf10_b(e + 0x2b));
        por(0x2c, cqf10_b(e + 0x23));
        por(0x2d, cqf10_b(e + 0x24));
        por(0x2e, 0);
        let adv = if cqf10_d(jogo) == cqf10_d(0x9cf684) { adversario.wrapping_neg() } else { adversario };
        core::ptr::write_unaligned((l + 0xa) as *mut u32, adv);
        if j < 0xb {
            por(0x2e, 1);
            let a = cqf10_b(e + 0x34);
            if (a as i8) > -1 {
                por(0x15, a);
            } else if (cqf10_b(e + 0x30) as i8) > -1 {
                por(0x2e, 5);
                por(0x15, cqf10_b(e + 0x30));
            } else {
                por(0x15, cqf10_b(resultado + 0x4d));
            }
        } else {
            por(0x2e, 2);
            let c = cqf10_b(e + 0x32);
            if c == 0xff {
                por(0x15, 0);
            } else {
                let a = cqf10_b(e + 0x34);
                if (a as i8) > -1 {
                    por(0x15, a.wrapping_sub(c));
                } else if (cqf10_b(e + 0x30) as i8) > -1 {
                    por(0x2e, 6);
                    por(0x15, cqf10_b(e + 0x30).wrapping_sub(c));
                } else {
                    por(0x15, cqf10_b(resultado + 0x4d).wrapping_sub(c));
                }
            }
        }
    }
}

/// Comparador da 0x7bd310: lado a lado nas 50 vagas (o registro inteiro). Com as máscaras ligadas e o técnico do clube
/// humano, a 0x5b09d0 grava o que se sabe do jogador e sorteia: aí só a original roda (ponytail: o caso de um humano
/// observando o próprio clube; a carreira automática não chega a esta rotina).
unsafe extern "thiscall" fn comparar_cqf10_historico_do_clube_observado(this: usize, jogo: usize, resultado: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1622);
    let clube_obs = cqf10_clube(cqf10_d(this));
    if cqf10_b(0x9cc270) != 0 && cqf10_d(clube_obs + 0xcf) != 0 {
        o(this, jogo, resultado);
        return;
    }
    let regioes = [(this, 4 + 0x32 * 0x1a2), (0xae24d0, 0x100), (0xb67a34, 4)];
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, resultado),
        || cqf10_historico_do_clube_observado(this, jogo, resultado));
    PLACAR.registrar(igual, || format!("cqf10_historico_do_clube_observado(clube {:#x}, jogo {:#x}) {:x?} x {:x?}", cqf10_d(this), jogo, mo, mr));
}

/// O ensaio da 0x7b5be0 (só no comparador): a cópia decide tudo até o relatório sem os efeitos (o histórico 0x7bd310 e o
/// relatório não rodam) e anota se chegaria ao relatório.
static mut CQF10_ENSAIO: bool = false;
static mut CQF10_RELATORIO: bool = false;

/// { 0x7b5be0 } (thiscall, ret 0xc, player_search.cpp, quadro SEH da lista local e das datas): `PLAYER_SEARCH::
/// scout_match(jogo, resultado, saida)`, pela partida observada (0x7f8c00). `this` é a lista de busca do olheiro: +0 o
/// dono, +0xa o tipo (4 = observar partidas), +0x170 o registro do clube observado, +0x174..+0x184 os nomes do olheiro,
/// +0x180 o id dele, +0x190 o clube acompanhado, +0x195 o jogador acompanhado, +0x1a5/+0x1ad o começo e o fim. Sem jogo
/// ou resultado: a caixa da linha 0xef1. Do tipo 4, com olheiro que confere (0x53e990) e resultado de partida jogada
/// (+0x4c zero): o clube acompanhado que joga recebe o histórico (0x7bd310); dentro das datas (antes do fim só com
/// jogador acompanhado), se o jogo é do jogador acompanhado, relata; senão a busca tem de cobrir a nação da competição
/// (0x7b7490) e o clube do olheiro tem de ter pedido (0x7f9690, 0x7f8d70 com 1) algum jogador de um dos dois clubes. O
/// relatório: o começo da lista vai a amanhã; dos dois lados (+0x1c0 / +0x1c0+0x625, 20 registros de 0x49), cada jogador
/// válido de outro clube (reservas além do 11º só com nota) é conhecido pelo dono (0x5b09d0, com as máscaras) e, pedido
/// (0x7f8d70 com 0), acompanhado ou com reputação vista abaixo de 4000 (0x5407e0) e nota de ao menos 15 (90 com o
/// resultado sem notas), vira um relatório (0x7b62a0, modo 0 acompanhado, 1 pedido, 2 com +5 ≥ 8) que entra na saída
/// (0x68d0d0, 0x67 bytes) se o byte +0x66 ficou ≥ 0. Devolve 1 com o relatório, senão 0.
unsafe extern "thiscall" fn cqf10_olheiro_no_jogo(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    // 0x68d020 / 0x68d040: lea ecx, local -> thiscall
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = cqf10_corpo(this, jogo, resultado, saida, l);
    soltar(l);
    r
}

unsafe fn cqf10_corpo(this: usize, jogo: usize, resultado: usize, saida: usize, l: usize) -> u32 {
    if jogo == 0 || resultado == 0 {
        aviso_em(0xa701b0, 0xef1);
        return 0;
    }
    let dono = cqf10_pessoa(cqf10_d(this));
    // a nação da competição do jogo (+0x18 → +0x38 → +0x1f) ou a do mandante (+0x1c → +0x53)
    let mut nacao = 0usize;
    let t = cqf10_d(jogo + 0x18);
    if t != 0 && cqf10_d(t + 0x38) != 0 && cqf10_d(cqf10_d(t + 0x38) + 0x1f) != 0 {
        nacao = cqf10_d(cqf10_d(t + 0x38) + 0x1f);
    }
    if nacao == 0 {
        nacao = cqf10_d(cqf10_d(jogo + 0x1c) + 0x53);
    }
    if cqf10_b(this + 0xa) != 4 {
        return 0;
    }
    let id_olheiro = cqf10_i(this + 0x180);
    if id_olheiro <= -1 {
        return 0;
    }
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
    // 0x53e990: 5 pushes; add esp, 0x14 -> cdecl
    if confere(cqf10_pessoa(id_olheiro as usize), cqf10_d(this + 0x174), cqf10_d(this + 0x178), cqf10_d(this + 0x17c), this + 0x184) == 0 {
        return 0;
    }
    let olheiro = cqf10_pessoa(cqf10_i(this + 0x180) as usize);
    if olheiro == 0 || cqf10_b(resultado + 0x4c) != 0 {
        return 0;
    }
    let acompanhado = cqf10_i(this + 0x190);
    if acompanhado > -1 && cqf10_d(this + 0x170) != 0 && (cqf10_i(jogo + 0xc) == acompanhado || cqf10_i(jogo + 0x10) == acompanhado) {
        if !CQF10_ENSAIO {
            crate::comparacao::ramo("cqf_10 historico do clube");
            let historico: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7bd310usize);
            // 0x7bd310: push resultado; push jogo; mov ecx, [this+0x170] -> thiscall, ret 8
            historico(cqf10_d(this + 0x170), jogo, resultado);
        }
    }
    // as datas: hoje antes do começo (+0x1a5: dia, +0x1a7: ano) sai; antes do fim (+0x1ad/+0x1af) só com jogador
    // acompanhado; ano sem sinal, dia com sinal, como a original
    let (dia, ano) = (cqf10_w(0xae2c90), cqf10_w(0xae2c92) as u16);
    let (dia_ini, ano_ini) = (cqf10_w(this + 0x1a5), cqf10_w(this + 0x1a7) as u16);
    if ano < ano_ini || (ano == ano_ini && dia < dia_ini) {
        return 0;
    }
    let (dia_fim, ano_fim) = (cqf10_w(this + 0x1ad), cqf10_w(this + 0x1af) as u16);
    let antes_do_fim = ano < ano_fim || (ano == ano_fim && dia < dia_fim);
    if antes_do_fim && cqf10_i(this + 0x195) == -1 {
        return 0;
    }
    let alvo = cqf10_i(this + 0x195);
    let do_alvo = alvo > -1 && cqf10_i(jogo) == alvo;
    if !do_alvo {
        let cobre: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7b7490usize);
        // 0x7b7490: push nação; mov ecx, this -> thiscall, ret 4
        if cobre(this, nacao) == 0 {
            return 0;
        }
        let alvos: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7f9690usize);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7f8d70usize);
        // 0x7f9690: push &lista; push clube do olheiro; mov ecx, [0xb74448] -> thiscall, ret 8; 0x68d1b0: lea ecx, lista
        // -> thiscall; 0x7f8d70: push 1; push clube; push jogador; mov ecx, [0xb74448] -> thiscall, ret 0xc
        alvos(cqf10_d(0xb74448), cqf10_d(olheiro + 0x39), l);
        let conta = cqf10_w(l);
        if conta == 0 {
            return 0;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cqf10_d(l + 2)); // o cursor volta à cabeça
        let mut livre = true; // ebp da original: nenhum pedido achado
        // ponytail: a original usa o mesmo registrador (edi) para contar os itens e as 20 vagas de cada lado; depois de
        // um item de um dos clubes o contador vira o número de vagas olhadas na última volta (o laço de fora acaba antes)
        let mut i: i32 = 0;
        if conta > 0 {
            loop {
                let item = proximo(l);
                if item != 0 {
                    let (m, v) = (cqf10_d(jogo + 0x1c), cqf10_d(jogo + 0x20));
                    let c39 = cqf10_d(item + 0x39);
                    let c24 = cqf10_d(item + 0x24);
                    if c39 == m || c39 == v || c24 == m || c24 == v {
                        let mut lado: i8 = 0;
                        while livre {
                            i = 0;
                            let mut k = 0usize;
                            while livre {
                                let id = cqf10_i(resultado + 0x190 + lado as usize * 0x625 + k);
                                if id != -1 && pedido(cqf10_d(0xb74448), cqf10_pessoa(id as usize), cqf10_d(olheiro + 0x39), 1) != 0 {
                                    livre = false;
                                }
                                k += 0x49;
                                i += 1;
                                if k >= 0x5b4 {
                                    break;
                                }
                            }
                            lado += 1;
                            if lado >= 2 {
                                break;
                            }
                        }
                    }
                }
                i += 1;
                if i >= cqf10_w(l) as i32 {
                    break;
                }
            }
        }
        if livre {
            return 0;
        }
    }
    crate::comparacao::ramo("cqf_10 relatorio");
    // o começo da lista vai a amanhã (0x54c5b0 com 1, 0x417c10 grava em +0x1a5); o byte baixo do dword da data copiada
    // vira o contador dos dois lados e vai inteiro como 7º argumento da 0x7b62a0
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    // 0x54c5b0: push 1; push &tmp; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x417c10: push data; push &copia; lea ecx,
    // [this+0x1a5] -> thiscall, ret 8
    let (mut tmp, mut copia) = ([0u32; 2], [0u32; 2]);
    let amanha = mais(0xae2c90, tmp.as_mut_ptr() as usize, 1); // cqf10: amanhã
    if CQF10_ENSAIO {
        CQF10_RELATORIO = true;
        return 1;
    }
    copiar(this + 0x1a5, copia.as_mut_ptr() as usize, amanha);
    let mut marca = copia[0] & !0xff;
    let quadro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5b09d0usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7f8d70usize);
    let reputacao: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
    let relatorio: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, usize, usize, u32) = core::mem::transmute(0x7b62a0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    // 0x5afde0: push dono; mov ecx, [0xae2a60] -> thiscall, ret 4; 0x5b09d0: push olheiro; push jogador; mov ecx, eax ->
    // thiscall, ret 8; 0x7f8d70: push 0; push clube; push jogador; mov ecx, [0xb74448] -> thiscall, ret 0xc; 0x5407e0:
    // push 0; push olheiro; push jogador; add esp, 0xc -> cdecl; 0x7b62a0: 7 pushes; mov ecx, this -> thiscall, ret 0x1c;
    // 0x68d0d0: push 0x67; push &registro; mov ecx, saida -> thiscall, ret 8
    // o registro local (0x67 bytes), reaproveitado entre os jogadores como na original, com as partes que o construtor da
    // original monta na entrada: a data em +0 (0x54a110) e os dois recordistas em +0x1f e +0x37 (0x450d40); o resto
    // começa zerado (na original, sobra da pilha que a 0x7b62a0 sobrescreve)
    let mut registro = [0u8; 0x68];
    let reg = registro.as_mut_ptr() as usize;
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let recordista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x450d40usize);
    // lea ecx, local -> thiscall nas duas
    data_padrao(reg);
    recordista(reg + 0x1f);
    recordista(reg + 0x37);
    let clube_olheiro = cqf10_d(olheiro + 0x39);
    for lado in 0..2usize {
        let mut e = resultado + 0x1c0 + lado * 0x625;
        for j in 0..0x14i32 {
            let atual = e;
            e += 0x49;
            if j > 10 && cqf10_b(atual + 2) == 0xff {
                continue;
            }
            let id = cqf10_i(atual - 0x30);
            if id == -1 {
                continue;
            }
            let p = cqf10_pessoa(id as usize);
            if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
                continue;
            }
            if cqf10_clube_do_contrato(p) == clube_olheiro || cqf10_d(p + 0x61) == 0 {
                continue;
            }
            if cqf10_b(0x9cc270) != 0 {
                let q = quadro(cqf10_d(0xae2a60), dono);
                if q != 0 {
                    conhecer(q, p, olheiro);
                }
            }
            let pediu = pedido(cqf10_d(0xb74448), p, clube_olheiro, 0);
            if pediu == 0 {
                let alvo = cqf10_i(this + 0x195);
                if !(alvo != -1 && cqf10_i(jogo) == alvo) && reputacao(p, olheiro, 0) >= 0xfa0 { // cqf10: reputação vista
                    continue;
                }
            }
            // a nota: titulares (até o 11º) pela nota +4, senão +0, senão a do resultado (+0x4d); reservas descontam a
            // entrada (+2); resultado sem notas (+0x4c): titular vale 90
            let nota_de = |x: usize| -> u8 {
                let a = cqf10_b(x + 4);
                if (a as i8) > -1 { a } else if (cqf10_b(x) as i8) > -1 { cqf10_b(x) } else { cqf10_b(resultado + 0x4d) }
            };
            let nota: u8 = if j < 0xb {
                if cqf10_b(resultado + 0x4c) != 0 { 0x5a } else { nota_de(atual) }
            } else {
                let s = cqf10_b(atual + 2);
                if s == 0xff {
                    continue;
                }
                nota_de(atual).wrapping_sub(s)
            };
            if (nota as i8) < 0xf { // cqf10: a nota mínima
                continue;
            }
            let alvo = cqf10_i(this + 0x195);
            let modo: u32 = if alvo > -1 && alvo == cqf10_i(jogo) {
                0
            } else if pediu != 0 {
                1
            } else {
                if cqf10_b(atual + 5) < 8 {
                    continue;
                }
                2
            };
            crate::comparacao::ramo("cqf_10 avaliar jogador");
            let n = ((cqf10_b(atual + 5) as i32 * 0x28 - 0x104).wrapping_mul(nota as i8 as i32)) / 2;
            relatorio(this, p, reg, n, modo, jogo, resultado, marca);
            if (cqf10_b(reg + 0x66) as i8) > -1 {
                acrescentar(saida, reg, 0x67);
            }
        }
        marca = (marca & !0xff) | ((marca as u8).wrapping_add(1)) as u32;
    }
    1
}

/// Comparador da 0x7b5be0: o relatório sorteia (0x7b62a0, 0x5b09d0) e põe registros na saída (aloca): só a original
/// roda. Antes, a cópia em ensaio decide até o relatório sem os efeitos (sem o histórico e sem o relatório, com o estado
/// do sorteador guardado e devolvido) e diz a devolução e se o começo da lista vai a amanhã; a original tem de bater.
unsafe extern "thiscall" fn comparar_cqf10_olheiro_no_jogo(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1621);
    if jogo == 0 || resultado == 0 {
        return o(this, jogo, resultado, saida);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = (cqf10_d(this + 0x1a5), cqf10_d(this + 0x1a9));
    CQF10_ENSAIO = true;
    CQF10_RELATORIO = false;
    let n = crate::aleatorio::sem_contar(|| cqf10_olheiro_no_jogo(this, jogo, resultado, saida));
    CQF10_ENSAIO = false;
    let relata = CQF10_RELATORIO;
    estado.escrever();
    let previsto = if relata {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u32; 2];
        let a = mais(0xae2c90, t.as_mut_ptr() as usize, 1);
        (cqf10_w(a + 2) as u16, cqf10_w(a))
    } else {
        (cqf10_w(this + 0x1a7) as u16, cqf10_w(this + 0x1a5))
    };
    let r = o(this, jogo, resultado, saida);
    let depois = (cqf10_w(this + 0x1a7) as u16, cqf10_w(this + 0x1a5));
    PLACAR.registrar(r == n && depois == previsto, || format!("cqf10_olheiro_no_jogo(lista {:#x}, jogo {:#x}) original={} ensaio={} começo antes {:x?} previsto {:x?} depois {:x?}",
        this, jogo, r, n, antes, previsto, depois));
    r
}

// ---------- 270a leva: as notícias das seleções aos clubes: o relatório do jogo, a convocação e os adversários (national_teams.cpp) ----------

unsafe fn cqf11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqf11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Os valores de 4 bytes dos itens de uma lista (nó: +0 o dado, +6 o próximo; cabeça em +2), sem mexer no cursor.
unsafe fn cqf11_itens(lista: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqf11_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqf11_d(cqf11_d(no)) as u32);
        no = cqf11_d(no + 6);
    }
    v
}

/// O ensaio do relatório (0x7706c0): os sorteios vêm da fita da original e a notícia não é entregue.
static mut CQF11_FITA: Option<(Vec<(i32, i32)>, usize, bool)> = None;

unsafe fn cqf11_sortear(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(CQF11_FITA);
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

/// O código de como o jogador foi no jogo (do registro dele no lado do clube, +0x190 + 0x49 × k): pelo tipo +0x21
/// (1 → 2, 2 → 8, 3 → 0x20, 4 a 8 → 0x40), +4 com nota (+0x34), +0x10 se é o melhor da partida (+0x3c = o id da ficha
/// +0x61), +1 se +0x23 − +0x24 é 1; 0 se ele não está no registro.
unsafe fn cqf11_codigo(resultado: usize, lado: usize, jogador: usize) -> u32 {
    let id = cqf11_d(jogador);
    for k in 0..0x14usize {
        let e = resultado + 0x190 + lado * 0x625 + k * 0x49;
        if cqf11_d(e) != id {
            continue;
        }
        let mut c: u32 = match cqf11_b(e + 0x21) {
            1 => 2,
            2 => 8,
            3 => 0x20,
            4..=8 => 0x40,
            _ => 0,
        };
        if cqf11_b(e + 0x34) != 0xff {
            c |= 4;
        }
        if cqf11_d(resultado + 0x3c) == cqf11_d(cqf11_d(jogador + 0x61)) {
            c |= 0x10;
        }
        if (cqf11_b(e + 0x23) as i32) - (cqf11_b(e + 0x24) as i32) == 1 {
            c |= 1;
        }
        return c;
    }
    0
}

/// { 0x7706c0 } (thiscall, ret 0x18, national_teams.cpp, quadro SEH da notícia local): `NationalTeams::
/// send_club_international_report_news(clube, jogo, resultado, destaque, elenco, conta)`, pelo resultado entre seleções
/// (0x76b600), de cada lado. O elenco (vetor de `conta` pessoas, o byte baixo) é ordenado pelo clube (0x945333 com
/// 0x778ef0); cada grupo de jogadores do mesmo clube vira uma notícia 0x24 ao técnico do clube (0x7861e0): o registro
/// da partida pelo clube (0x7e4750, uma vez só) e os dados dele (0x7e4de0), o destaque (id e chaves, se é do clube;
/// senão −1 nos quatro), quantos são, o clube, até 4 jogadores (o destaque primeiro, os outros pelo código de como
/// foram, do maior para o menor), 0x420e9, sorteio(2) e os códigos. Os jogadores do grupo saem do vetor (zerados).
unsafe extern "thiscall" fn cqf11_relatorio_da_selecao(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
    let _ = this;
    let q = conta as u8 as i8;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let registro_da_partida: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(0x7e4750usize);
    let dados_da_partida: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e4de0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    // 0x945333 (qsort): push 0x778ef0; push 4; push conta; push elenco; add esp, 0x10 -> cdecl; 0x77bbe0: push 0; push
    // 0x24; lea ecx, notícia -> thiscall; 0x785790: push valor; push índice; lea ecx, notícia -> thiscall, ret 8;
    // 0x7e4750: push &registro; push clube; push jogo; add esp, 0xc -> cdecl; 0x7e4de0: push &índice; push &notícia;
    // push &registro; add esp, 0xc -> cdecl; 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
    ordenar(elenco as usize, q as i32, 4, 0x778ef0);
    if q <= 0 {
        return;
    }
    let lado = (clube != cqf11_d(jogo + 0x1c)) as usize;
    let mut registro = [0u8; 0x20]; // a Data em +0 (0x54a110 na entrada), o resto a 0x7e4750 preenche
    let reg = registro.as_mut_ptr() as usize;
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(reg);
    let mut montado = false;
    let mut noticia = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = noticia.as_mut_ptr() as usize;
    let mut i: i8 = 1;
    loop {
        let slot = elenco.add(i as usize - 1);
        let primeiro = *slot;
        if primeiro != 0 && cqf11_d(primeiro + 0x39) != 0 {
            let mut codigos = [0u32; 4];
            let mut jogadores = [0usize; 4];
            codigos[0] = cqf11_codigo(resultado, lado, primeiro);
            jogadores[0] = primeiro;
            let mut quantos: i8 = 1;
            let mut j = i;
            while j < q {
                let pj = elenco.add(j as usize);
                let outro = *pj;
                if outro != 0 {
                    if cqf11_d(outro + 0x39) != cqf11_d(primeiro + 0x39) {
                        break;
                    }
                    if quantos < 4 {
                        let k = quantos as usize;
                        codigos[k] = cqf11_codigo(resultado, lado, outro);
                        jogadores[k] = outro;
                        if outro == destaque {
                            // o destaque vai para a frente (troca com o primeiro, código junto)
                            jogadores[k] = jogadores[0];
                            jogadores[0] = outro;
                            codigos.swap(0, k);
                        }
                    }
                    quantos += 1;
                    *pj = 0;
                }
                j += 1;
            }
            construir(nt, 0x24, 0);
            if !montado {
                montado = true;
                registro_da_partida(jogo, clube, reg);
            }
            let mut idx: u8 = 0;
            dados_da_partida(reg, nt, &mut idx as *mut u8);
            let mut pr = |v: u32| {
                gravar(nt, idx as u32, v);
                idx = idx.wrapping_add(1);
            };
            if cqf11_d(destaque + 0x39) == cqf11_d(primeiro + 0x39) {
                pr(cqf11_d(destaque) as u32);
                pr(cqf11_d(cqf11_d(destaque + 4) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(destaque + 8) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(destaque + 0xc) + 0x33) as u32);
            } else {
                for _ in 0..4 {
                    pr(u32::MAX);
                }
            }
            pr(quantos as i32 as u32);
            pr(cqf11_d(cqf11_d(primeiro + 0x39)) as u32);
            // os de depois do primeiro, pelo código, do maior para o menor (até a 3ª posição e 4 jogadores)
            let fim = quantos as i32 - 1;
            let mut a: i8 = 1;
            if fim > 1 {
                while a < 3 {
                    let mut b = a + 1;
                    while b < quantos && b < 4 {
                        if (codigos[a as usize] as i32) < (codigos[b as usize] as i32) { // cqf11: a ordem dos jogadores
                            codigos.swap(a as usize, b as usize);
                            jogadores.swap(a as usize, b as usize);
                        }
                        b += 1;
                    }
                    a += 1;
                    if a as i32 >= fim {
                        break;
                    }
                }
            }
            let n = quantos.min(4);
            for k in 0..n.max(0) as usize {
                let p = jogadores[k];
                pr(cqf11_d(p) as u32);
                pr(cqf11_d(cqf11_d(p + 4) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(p + 8) + 0x33) as u32);
                pr(cqf11_d(cqf11_d(p + 0xc) + 0x33) as u32);
            }
            pr(0x420e9);
            crate::comparacao::ramo("cqf_11 noticia do clube");
            let k_sorteio = idx;
            idx = idx.wrapping_add(1);
            let r = cqf11_sortear(2); // cqf11: o sorteio da notícia
            gravar(nt, k_sorteio as u32, r as u32);
            for k in 0..n.max(0) as usize {
                gravar(nt, idx as u32, codigos[k]);
                idx = idx.wrapping_add(1);
            }
            if (*core::ptr::addr_of!(CQF11_FITA)).is_none() {
                entregar(cqf11_d(0xb7431c), nt, cqf11_d(primeiro + 0x39));
            }
        }
        let anterior = i;
        i = i.wrapping_add(1);
        if anterior >= q {
            break;
        }
    }
}

/// Comparador da 0x7706c0: a original entrega as notícias (aloca nas caixas): só ela roda, com os sorteios rastreados e o
/// elenco guardado antes. Depois o elenco volta ao de antes, a cópia refaz em ensaio (sorteios da fita, sem entregar) e o
/// elenco tem de sair igual ao da original, com os sorteios do ensaio iguais aos da fita (mesmos n, todos usados).
unsafe extern "thiscall" fn comparar_cqf11_relatorio_da_selecao(this: usize, clube: usize, jogo: usize, resultado: usize, destaque: usize, elenco: *mut usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32)>(1623);
    let q = (conta as u8 as i8).max(0) as usize;
    let antes: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    let ((), fita) = crate::aleatorio::rastrear(|| o(this, clube, jogo, resultado, destaque, elenco, conta));
    let depois_o: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    let n = fita.len();
    for k in 0..q {
        *elenco.add(k) = antes[k];
    }
    *core::ptr::addr_of_mut!(CQF11_FITA) = Some((fita, 0, false));
    crate::aleatorio::sem_contar(|| cqf11_relatorio_da_selecao(this, clube, jogo, resultado, destaque, elenco, conta));
    let (k, falhou) = match &*core::ptr::addr_of!(CQF11_FITA) {
        Some((_, k, f)) => (*k, *f),
        None => (0, true),
    };
    *core::ptr::addr_of_mut!(CQF11_FITA) = None;
    let depois_r: Vec<usize> = (0..q).map(|k| *elenco.add(k)).collect();
    for k in 0..q {
        *elenco.add(k) = depois_o[k];
    }
    PLACAR.registrar(!falhou && k == n && depois_o == depois_r, || format!("cqf11_relatorio_da_selecao(clube {:#x}, jogo {:#x}, conta {}) fita de {} sorteios, ensaio usou {} (n diferente: {}); elenco original {:x?} copia {:x?}",
        clube, jogo, q, n, k, falhou, depois_o, depois_r));
}

/// { 0x76e0b0 } (thiscall, ret 0x1c, national_teams.cpp, quadro SEH da notícia local): `national_teams_post_call_up_news(
/// seleção, segunda, jogo, tipo, sem_adversarios, adversarios, conta)`, pela convocação (0x76c120) e pela tela. Das 50
/// vagas do elenco da seleção (+0xd7, ponteiros), cada jogador com a marca (+0x5e: 8 na segunda seleção, senão 4) e
/// clube de técnico humano (0x7875e0) entra no grupo do clube dele; quando o próximo da vaga não é do mesmo clube (ou
/// não tem a marca, ou acabou), sai a notícia 0x1d ao técnico do clube (0x7861e0): a seleção, o clube, `sem_adversarios`,
/// o tipo, quantos, `segunda`, quantos adversários e, com menos de 4, os ids deles (da lista dada ou montada uma vez pela
/// 0x76d560 com o jogo, sem lista e sem `sem_adversarios`), e até 3 jogadores (id e chaves). A lista montada aqui é
/// destruída e liberada no fim (0x68d040, 0x944c94).
unsafe extern "thiscall" fn cqf11_noticia_da_convocacao(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let montar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x76d560usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    // 0x7875e0: push clube; mov ecx, [0xb7431c] -> thiscall, ret 4; 0x76d560: push jogo; push seleção; mov ecx, this ->
    // thiscall, ret 8; 0x77bbe0 / 0x785790 / 0x68d1b0 / 0x68d040: lea ecx (ou mov ecx, lista) -> thiscall; 0x7861e0:
    // push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x944c94 (delete): push; add esp, 4 -> cdecl
    let mut lista = 0usize;
    let mut quantos_adv: i16 = 0;
    if adversarios != 0 {
        lista = adversarios;
        quantos_adv = conta as u8 as i8 as i16;
    }
    let marca: u8 = if segunda != 0 { 8 } else { 4 };
    let mut n: i8 = 0;
    let mut tres = [0usize; 3];
    let mut noticia = [0u32; 0x38];
    let nt = noticia.as_mut_ptr() as usize;
    for idx in 1..=0x32usize {
        let p = cqf11_d(selecao + 0xd7 + (idx - 1) * 4);
        if p == 0 || cqf11_b(p + 0x5e) & marca == 0 || cqf11_d(p + 0x39) == 0 || humano(cqf11_d(0xb7431c), cqf11_d(p + 0x39)) == 0 {
            continue;
        }
        if lista == 0 && sem_adversarios == 0 && jogo != 0 {
            lista = montar(this, selecao, jogo);
            quantos_adv = *(lista as *const i16);
        }
        if n < 3 {
            tres[n as usize] = p;
        }
        let clube = cqf11_d(p + 0x39);
        n = n.wrapping_add(1);
        if idx < 0x32 {
            let prox = cqf11_d(selecao + 0xd7 + idx * 4);
            if prox != 0 && cqf11_d(prox + 0x39) == clube && cqf11_b(prox + 0x5e) & marca != 0 {
                continue;
            }
        }
        crate::comparacao::ramo("cqf_11 noticia da convocacao");
        construir(nt, 0x1d, 0);
        gravar(nt, 0, cqf11_d(selecao) as u32);
        gravar(nt, 1, cqf11_d(clube) as u32);
        gravar(nt, 2, sem_adversarios);
        gravar(nt, 3, tipo);
        gravar(nt, 4, n as i32 as u32);
        gravar(nt, 5, segunda);
        gravar(nt, 6, quantos_adv as i32 as u32);
        if quantos_adv < 4 && lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqf11_d(lista + 2));
            let mut k: u8 = 7;
            let mut item = proximo(lista);
            while item != 0 {
                gravar(nt, k as u32, cqf11_d(cqf11_d(item)) as u32);
                k = k.wrapping_add(1);
                item = proximo(lista);
            }
        }
        if n <= 3 && n > 0 {
            let mut k: u8 = 0xa;
            for t in 0..n as usize {
                let j = tres[t];
                gravar(nt, k as u32, cqf11_d(j) as u32);
                gravar(nt, k.wrapping_add(1) as u32, cqf11_d(cqf11_d(j + 4) + 0x33) as u32);
                gravar(nt, k.wrapping_add(2) as u32, cqf11_d(cqf11_d(j + 8) + 0x33) as u32);
                gravar(nt, k.wrapping_add(3) as u32, cqf11_d(cqf11_d(j + 0xc) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
        }
        entregar(cqf11_d(0xb7431c), nt, clube);
        n = 0;
    }
    if lista != 0 && adversarios == 0 {
        soltar(lista);
        apagar(lista);
    }
}

/// Comparador da 0x76e0b0: ela só monta e entrega notícias (aloca nas caixas dos humanos) e monta e libera a lista dos
/// adversários: não pode rodar duas vezes e não grava nada do jogo além das caixas. Só a original roda (ponytail: o
/// efeito é a caixa de entrada do humano; a lista dos adversários é conferida pela 0x76d560).
unsafe extern "thiscall" fn comparar_cqf11_noticia_da_convocacao(this: usize, selecao: usize, segunda: u32, jogo: usize, tipo: u32, sem_adversarios: u32, adversarios: usize, conta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32)>(1624)(this, selecao, segunda, jogo, tipo, sem_adversarios, adversarios, conta);
}

/// O ensaio da 0x76d560: os valores que iriam para a lista nova, sem alocar.
static mut CQF11_PLANO: Option<Vec<u32>> = None;

/// { 0x76d560 } (thiscall, ret 8, national_teams.cpp, quadro SEH da lista local): `national_teams_collect_fixture_
/// opponents(seleção, jogo)`, pela notícia da convocação (0x76e0b0) e pelo pedido ao técnico (0x76f1a0). Uma lista
/// nova (0x944cf1 com 0xe, 0x68d020) com o adversário do jogo (o visitante se o mandante é da nação da seleção, senão o
/// mandante; na segunda seleção, a seleção principal da nação dele, 0x53b4f0) e os adversários dos outros jogos da
/// seleção de amanhã até 13 dias depois (0x5acbe0, com o byte +0x40 do jogo) que não são de fase final (as nove
/// competições de seleções). Devolve a lista.
unsafe extern "thiscall" fn cqf11_adversarios_da_selecao(this: usize, selecao: usize, jogo: usize) -> usize {
    let _ = this;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    // 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx (ou mov ecx, bloco) -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 ->
    // cdecl; 0x68d0d0: push 4; push &valor; mov ecx, lista -> thiscall, ret 8; 0x5409b0 / 0x53b4f0: push; add esp, 4 ->
    // cdecl; 0x549ef0: push +0x28; push +0x2a; lea ecx -> thiscall, ret 8; 0x54c5b0: push n; push &saída; mov ecx, data
    // -> thiscall, ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
    let ensaio = (*core::ptr::addr_of!(CQF11_PLANO)).is_some();
    let mut local = [0u8; 16];
    let l = local.as_mut_ptr() as usize;
    nova(l);
    let mut nl = 0usize;
    if !ensaio {
        let m = novo(0xe);
        if m != 0 {
            nl = nova(m);
        }
    }
    let mut por = |v: usize| {
        if ensaio {
            if let Some(p) = &mut *core::ptr::addr_of_mut!(CQF11_PLANO) {
                p.push(cqf11_d(v) as u32);
            }
        } else {
            acrescentar(nl, v, 4);
        }
    };
    // o 9º argumento da 0x5acbe0: o dword do argumento `jogo` na pilha (ou do 0x53b4f0 gravado em cima) com o byte baixo
    // trocado pelo +0x40 do jogo
    let mut celula: usize = jogo;
    let mandante = jogo + 0x1c;
    if cqf11_d(cqf11_d(mandante) + 0x53) == cqf11_d(selecao + 0x53) {
        if segunda(selecao) != 0 {
            celula = principal(cqf11_d(cqf11_d(jogo + 0x20) + 0x53));
            por(&celula as *const usize as usize);
        } else {
            por(jogo + 0x20);
        }
    } else if segunda(selecao) != 0 {
        celula = principal(cqf11_d(cqf11_d(mandante) + 0x53));
        por(&celula as *const usize as usize);
    } else {
        por(mandante);
    }
    celula = (celula & !0xff) | cqf11_b(jogo + 0x40) as usize;
    let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    let dia_ano = |p: usize| (cqf11_d(p) as u32, core::ptr::read_unaligned((p + 2) as *const u32));
    data_nova(p0, *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    mais(p0, p1, 1);
    mais(p1, p2, 0xd);
    let (a1, a2) = dia_ano(p1);
    let (a3, a4) = dia_ano(p2);
    achar(cqf11_d(0xae2a58), a1, a2, a3, a4, cqf11_d(selecao) as u32, l, -1, -2, celula as u32);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqf11_d(l + 2));
    let finais = [0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964];
    let mut item = proximo(l);
    while item != 0 {
        let c = cqf11_d(item + 0x14);
        if c == 0 || !finais.iter().any(|&k| cqf11_d(c) == cqf11_d(k)) {
            if cqf11_d(cqf11_d(item + 0x1c) + 0x53) == cqf11_d(selecao + 0x53) {
                por(item + 0x20);
            } else {
                por(item + 0x1c);
            }
        }
        item = proximo(l);
    }
    soltar(l);
    nl
}

/// Comparador da 0x76d560: a original aloca a lista nova (não pode rodar duas vezes): só ela roda; antes, a cópia em
/// ensaio diz os valores que vão na lista (sem alocar), e a lista da original tem de ter exatamente esses, na ordem.
unsafe extern "thiscall" fn comparar_cqf11_adversarios_da_selecao(this: usize, selecao: usize, jogo: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(1625);
    *core::ptr::addr_of_mut!(CQF11_PLANO) = Some(Vec::new());
    cqf11_adversarios_da_selecao(this, selecao, jogo);
    let plano = (*core::ptr::addr_of_mut!(CQF11_PLANO)).take().unwrap_or_default();
    let r = o(this, selecao, jogo);
    let agora = cqf11_itens(r);
    PLACAR.registrar(agora == plano, || format!("cqf11_adversarios_da_selecao(seleção {:#x}, jogo {:#x}) plano {:x?} original {:x?}", selecao, jogo, plano, agora));
    r
}

// ---------- 270a leva: os olheiros do dia, os relatórios deles e a notícia do relatório (scout_manager.cpp) ----------

unsafe fn cqf12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqf12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf12_pessoa(id: usize) -> usize {
    cqf12_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf12_clube(id: usize) -> usize {
    cqf12_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// `a` depois de `b` (datas: ano sem sinal em +2, dia com sinal em +0), como as comparações da original.
unsafe fn cqf12_depois(dia: i16, ano: u16, data: usize) -> bool {
    let (d, a) = (cqf12_w(data), cqf12_w(data + 2) as u16);
    ano > a || (ano == a && dia > d)
}

/// O ensaio da 0x7f8750: a cópia só poda os pedidos (num vetor dela) e para antes dos relatórios.
static mut CQF12_ENSAIO: bool = false;

/// A poda de um pedido (0x11 bytes: +0 o clube, +4 o jogador, +8 a data, +0x10 quantos relatórios): devolve
/// (tirar, processar). Sai o pedido de clube que não é humano, de jogador do próprio clube (o +0x39 ou o do contrato),
/// com mais de um ano, com 3 relatórios, ou (depois de 30 dias) de jogador machucado (0x62c400 = 2); entre 30 dias e um
/// ano o pedido é processado (e sai), salvo jogador sem nação ou de clube que joga nos 14 dias seguintes, que fica.
unsafe fn cqf12_julgar(pedido: usize, catorze: usize) -> (bool, bool, usize, usize, usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, u32) -> i16 = core::mem::transmute(0x5acbe0usize);
    // 0x600a50: 5 pushes (clube, 1, 1, 0, 0); add esp, 0x14 -> cdecl; 0x4cf5a0: push jogador; mov ecx, [0xae1050] ->
    // thiscall, ret 4; 0x54c5b0: push n; push &saída; lea ecx, [pedido+8] -> thiscall, ret 8; 0x62c400: push jogador;
    // mov ecx, [0xb63d28] -> thiscall, ret 4 (al); 0x4cf5f0: push clube; push jogador; mov ecx, [0xae1050] -> thiscall,
    // ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24 (ax)
    let jogador = cqf12_pessoa(cqf12_d(pedido + 4));
    let clube = cqf12_clube(cqf12_d(pedido));
    if humano(clube, 1, 1, 0, 0) == 0 {
        return (true, false, 0, 0, 0);
    }
    let tecnico = cqf12_d(clube + 0xcf);
    if cqf12_d(jogador + 0x39) == clube {
        return (true, false, 0, 0, 0);
    }
    let c = contrato_de(cqf12_d(0xae1050), jogador);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    let do_contrato = if c != 0 { cqf12_clube(cqf12_d(c + 4)) } else { 0 };
    if do_contrato == clube {
        return (true, false, 0, 0, 0);
    }
    let (dia, ano) = (cqf12_w(0xae2c90), cqf12_w(0xae2c92) as u16);
    let mut t = [0u32; 2];
    let ano_depois = mais(pedido + 8, t.as_mut_ptr() as usize, 0x16d); // cqf12: um ano
    if cqf12_depois(dia, ano, ano_depois) {
        return (true, false, 0, 0, 0);
    }
    if (cqf12_b(pedido + 0x10) as i8) >= 3 { // cqf12: três relatórios
        return (true, false, 0, 0, 0);
    }
    let mut t2 = [0u32; 2];
    let mes_depois = mais(pedido + 8, t2.as_mut_ptr() as usize, 0x1e); // cqf12: trinta dias
    if !cqf12_depois(dia, ano, mes_depois) {
        return (false, false, 0, 0, 0);
    }
    if lesao(cqf12_d(0xb63d28), jogador) == 2 {
        return (true, false, 0, 0, 0);
    }
    let seu = cqf12_d(jogador + 0x39);
    let mut nacao = if seu != 0 { cqf12_d(seu + 0x53) } else { 0 };
    if nacao == 0 {
        nacao = cqf12_d(jogador + 0x1a);
        if nacao == 0 {
            return (false, false, 0, 0, 0);
        }
    }
    let mut direto = false;
    if seu != 0 {
        let k = contrato_com(cqf12_d(0xae1050), jogador, seu);
        if k != 0 {
            let s = cqf12_b(k + 0x4f) & 0xf0;
            direto = s == 0x60 || s == 0x70 || s == 0;
        }
    }
    if !direto && seu != 0 {
        // o clube do jogador joga de hoje até 14 dias depois? (sem lista; a parte alta do 2º dword é sobra da pilha na
        // original e a 0x5acbe0 só lê a palavra baixa)
        let ultimo = cqf12_d(catorze) as u32;
        let ultimo_alto = core::ptr::read_unaligned((catorze + 2) as *const u32);
        let a1 = (ultimo_alto & 0xffff_0000) | dia as u16 as u32;
        let a2 = ano as u32;
        if achar(cqf12_d(0xae2a58), a1, a2, ultimo, ultimo_alto, cqf12_d(seu) as u32, 0, -1, -2, 1) != 0 {
            return (false, false, 0, 0, 0);
        }
    }
    (true, true, jogador, tecnico, nacao)
}

/// { 0x7f8750 } (thiscall, ret 0, scout_manager.cpp, quadro SEH da lista local e das datas): `SCOUT_MANAGER::
/// daily_update()`, pelo dia (só na fatia 2 do dia, [0xae2c88]; senão 0). Poda os pedidos de observação (+0x5c,
/// conta +0x64, 0x11 bytes; o tirado recebe o último), e o pedido processado vira até 3 relatórios dos olheiros do
/// técnico do clube (das 10 listas, 0x7f6cd0, a que cobre a nação do jogador, 0x7b7490): 0x7b62a0 com −1000, modo 1, e o
/// registro (0x67 bytes) entra na lista local se o byte +0x66 ficou ≥ 0. Depois os relatórios (0x7f9890), a ordem dos
/// pedidos (0x945333 com 0x7fc270) e uma busca (0x7b3e20) em cada uma das 10 listas dos técnicos humanos (as 16 últimas
/// pessoas com +0x5f = 1). Devolve 1.
unsafe extern "thiscall" fn cqf12_olheiros_do_dia(this: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // 0x68d020 / 0x68d040 / 0x54a110: lea ecx, local -> thiscall; 0x54c5b0: push 0xe; push &saída; mov ecx, 0xae2c90
    let mut d14 = [0u32; 2];
    let catorze = d14.as_mut_ptr() as usize;
    mais(0xae2c90, catorze, 0xe);
    // o registro do relatório (0x67 bytes) com as datas que a entrada da original constrói (+0, +0x2f, +0x47)
    let mut registro = [0u8; 0x68];
    let reg = registro.as_mut_ptr() as usize;
    data_padrao(reg);
    data_padrao(reg + 0x2f);
    data_padrao(reg + 0x47);
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    if cqf12_b(0xae2c88) != 2 {
        soltar(l);
        return 0;
    }
    let registro_do_olheiro: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let cobre: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7b7490usize);
    let relatorio: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, usize, usize, u32) = core::mem::transmute(0x7b62a0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let relatorios: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f9890usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let buscar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7b3e20usize);
    // 0x7f6cd0: push técnico; push k; mov ecx, this -> thiscall, ret 8; 0x7b7490: push nação; mov ecx, lista ->
    // thiscall, ret 4; 0x7b62a0: push 1; push 0; push 0; push 1; push −1000; push &registro; push jogador; mov ecx, lista
    // -> thiscall, ret 0x1c; 0x68d0d0: push 0x67; push &registro; lea ecx, local -> thiscall, ret 8; 0x7f9890: push
    // &local; mov ecx, this -> thiscall, ret 4; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x7b3e20: mov ecx,
    // lista -> thiscall, ret 0
    let mut i: i32 = 0;
    while i < cqf12_i(this + 0x64) {
        let pedido = cqf12_d(this + 0x5c) + i as usize * 0x11;
        let (tirar, processar, jogador, tecnico, nacao) = cqf12_julgar(pedido, catorze);
        if processar && !CQF12_ENSAIO {
            crate::comparacao::ramo("cqf_12 pedido processado");
            let mut feitos: i8 = 0;
            for k in 0..10u32 {
                if feitos >= 3 { // cqf12: até três relatórios
                    break;
                }
                let lo = registro_do_olheiro(this, k, tecnico);
                if cobre(lo, nacao) != 0 {
                    relatorio(lo, jogador, reg, -1000, 1, 0, 0, 1);
                    feitos += 1;
                    if (cqf12_b(reg + 0x66) as i8) > -1 {
                        crate::comparacao::ramo("cqf_12 relatorio do pedido");
                        acrescentar(l, reg, 0x67);
                    }
                }
            }
        }
        if tirar {
            let n = cqf12_i(this + 0x64);
            let base = cqf12_d(this + 0x5c);
            core::ptr::copy(base.wrapping_add((n as usize).wrapping_mul(0x11)).wrapping_sub(0x11) as *const u8, pedido as *mut u8, 0x11);
            core::ptr::write_unaligned((this + 0x64) as *mut i32, n - 1);
            i -= 1;
        }
        i += 1;
    }
    if CQF12_ENSAIO {
        soltar(l);
        return 1;
    }
    relatorios(this, l);
    ordenar(cqf12_d(this + 0x5c), cqf12_i(this + 0x64), 0x11, 0x7fc270);
    for k in 0..0x10i32 {
        let p = cqf12_pessoa((cqf12_i(0xae2364) - k - 1) as usize);
        if cqf12_b(p + 0x5f) == 1 {
            crate::comparacao::ramo("cqf_12 busca do humano");
            for j in 0..10u32 {
                buscar(registro_do_olheiro(this, j, p)); // cqf12: a busca de cada lista
            }
        }
    }
    soltar(l);
    1
}

/// Comparador da 0x7f8750: os relatórios sorteiam e alocam e as buscas mexem nas listas dos humanos: só a original
/// roda. Antes, a cópia em ensaio poda uma cópia dos pedidos (sem os relatórios) e os põe na ordem da original; os
/// pedidos que a original deixa (conta e os 0x11 bytes de cada) têm de ser esses.
unsafe extern "thiscall" fn comparar_cqf12_olheiros_do_dia(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1626);
    if cqf12_b(0xae2c88) != 2 {
        return o(this);
    }
    let n0 = cqf12_i(this + 0x64).max(0) as usize;
    let base = cqf12_d(this + 0x5c);
    let mut copia: Vec<u8> = (0..n0 * 0x11).map(|k| cqf12_b(base + k)).collect();
    copia.push(0);
    // o ensaio poda a cópia: troca o vetor e a conta do `this` pelos da cópia e devolve depois
    let (b0, c0) = (cqf12_d(this + 0x5c), cqf12_i(this + 0x64));
    core::ptr::write_unaligned((this + 0x5c) as *mut usize, copia.as_mut_ptr() as usize);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let a1080 = cqf12_d(0xae1080);
    CQF12_ENSAIO = true;
    crate::aleatorio::sem_contar(|| cqf12_olheiros_do_dia(this));
    CQF12_ENSAIO = false;
    estado.escrever();
    core::ptr::write_unaligned(0xae1080 as *mut usize, a1080);
    let n1 = cqf12_i(this + 0x64).max(0) as usize;
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(copia.as_mut_ptr() as usize, n1 as i32, 0x11, 0x7fc270);
    let plano: Vec<u8> = copia[..n1 * 0x11].to_vec();
    core::ptr::write_unaligned((this + 0x5c) as *mut usize, b0);
    core::ptr::write_unaligned((this + 0x64) as *mut i32, c0);
    let r = o(this);
    let n2 = cqf12_i(this + 0x64).max(0) as usize;
    let agora: Vec<u8> = (0..n2 * 0x11).map(|k| cqf12_b(cqf12_d(this + 0x5c) + k)).collect();
    PLACAR.registrar(r == 1 && agora == plano, || format!("cqf12_olheiros_do_dia: pedidos {} -> original {} ensaio {}; {:x?} x {:x?}", n0, n2, n1, agora, plano));
    r
}

/// O registro das notícias de relatório que a 0x7f9a60 recebe (jogador, clube, quantos), ligado só no comparador da
/// 0x7f9890 para conferir os grupos da original.
static mut CQF12_GRUPOS: Option<Vec<(u32, u32, u8)>> = None;

/// { 0x7f9890 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::send_scout_reports(lista)`, pelo dia dos olheiros
/// (0x7f8750) e pela busca (0x7f8c00). Com a lista não vazia (+0 conta): um vetor de conta × 0x67 (0x944e46; sem
/// memória, a caixa da linha 0x7dd) com cópias dos registros, ordenado (0x945333 com 0x7fc2a0); cada grupo seguido do
/// mesmo jogador (+0x2b) e do mesmo clube (+0x50) vira uma notícia de relatório (0x7f9a60 com o primeiro e quantos).
/// Libera o vetor (0x9452ca). A lista fica com o cursor no começo.
unsafe extern "thiscall" fn cqf12_relatorios_dos_olheiros(this: usize, lista: usize) {
    let conta = cqf12_w(lista);
    if conta == 0 {
        return;
    }
    let n = conta as i32;
    let alocar: unsafe extern "C" fn(i32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x7f9a60usize);
    // 0x944e46 (malloc): push; add esp, 4 -> cdecl; 0x9452ca (free): push; add esp, 4 -> cdecl; 0x68d1b0: mov ecx, lista
    // -> thiscall; 0x945333: 4 pushes; add esp, 0x10 -> cdecl; 0x7f9a60: push quantos; push registro; mov ecx, this ->
    // thiscall, ret 8
    let v = alocar(n.wrapping_mul(0x67));
    if v == 0 && n != 0 {
        aviso_em(0xa7b094, 0x7dd);
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqf12_d(lista + 2));
    let mut dst = v;
    let mut item = proximo(lista);
    while item != 0 {
        core::ptr::copy_nonoverlapping(item as *const u8, dst as *mut u8, 0x67);
        dst += 0x67;
        item = proximo(lista);
    }
    ordenar(v, n, 0x67, 0x7fc2a0);
    let mut i: i32 = 0;
    while i < n {
        let r = v + i as usize * 0x67;
        let clube_id = cqf12_d(cqf12_clube(cqf12_d(r + 0x50)));
        let jogador_id = cqf12_d(cqf12_pessoa(cqf12_d(r + 0x2b)));
        let inicio = i;
        let mut quantos: u8 = 1;
        let mut j = i + 1;
        while j < n {
            let s = v + j as usize * 0x67;
            if cqf12_d(s + 0x50) != clube_id || cqf12_d(s + 0x2b) != jogador_id {
                break;
            }
            quantos = quantos.wrapping_add(1);
            j += 1;
            i += 1;
        }
        let primeiro = v + inicio as usize * 0x67;
        if CQF12_ENSAIO {
            if let Some(g) = &mut *core::ptr::addr_of_mut!(CQF12_GRUPOS) {
                g.push((cqf12_d(primeiro + 0x2b) as u32, cqf12_d(primeiro + 0x50) as u32, quantos));
            }
        } else {
            noticia(this, primeiro, quantos as u32);
        }
        i += 1;
    }
    liberar(v);
}

/// Comparador da 0x7f9890: a original entrega notícias (aloca nas caixas): só ela roda, com o registro das chamadas à
/// 0x7f9a60 ligado; antes, a cópia em ensaio (sem a 0x7f9a60) diz os grupos (jogador, clube, quantos) em ordem, e as
/// chamadas da original têm de ser essas.
unsafe extern "thiscall" fn comparar_cqf12_relatorios_dos_olheiros(this: usize, lista: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1627);
    let cursor = cqf12_d(lista + 0xa);
    *core::ptr::addr_of_mut!(CQF12_GRUPOS) = Some(Vec::new());
    CQF12_ENSAIO = true;
    cqf12_relatorios_dos_olheiros(this, lista);
    CQF12_ENSAIO = false;
    let plano = (*core::ptr::addr_of_mut!(CQF12_GRUPOS)).replace(Vec::new()).unwrap_or_default();
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cursor);
    o(this, lista);
    let feito = (*core::ptr::addr_of_mut!(CQF12_GRUPOS)).take().unwrap_or_default();
    PLACAR.registrar(feito == plano, || format!("cqf12_relatorios_dos_olheiros(lista {:#x}) original {:x?} ensaio {:x?}", lista, feito, plano));
}

/// { 0x7f9a60 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da notícia local): `SCOUT_MANAGER::
/// send_scout_report_news(registro, quantos)`, pelos relatórios (0x7f9890). A notícia 0x138e (0x77bbe0): quantos (até
/// 3), +0x66, o olheiro (0x7e5160 com +0x1f), +0x59, +0x5a, o clube +0x50, os dados do jogo (0x7e4de0), +0x5e, +0x62,
/// +0x58, +0x54, de cada um dos registros (+0x37 + 0x67 × k) +0xc, +0, +4, +8 e +0x26 + 8 × +0x25, 0x420e9 e as forças
/// do jogador (+0x2b) para o clube (0x546ab0); vai ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn cqf12_noticia_do_relatorio(this: usize, r: usize, quantos: u32) {
    let _ = this;
    let n = (quantos as u8 as i8).min(3);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let olheiro: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e5160usize);
    let dados: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e4de0usize);
    let forcas: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    // 0x77bbe0: push 0; push 0x138e; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8; 0x7e5160
    // e 0x7e4de0: push &índice; push &notícia; push registro; add esp, 0xc -> cdecl; 0x546ab0: push clube; push
    // jogador; add esp, 8 -> cdecl; 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
    let mut noticia = [0u32; 0x38];
    let nt = noticia.as_mut_ptr() as usize;
    construir(nt, 0x138e, 0);
    gravar(nt, 0, n as i32 as u32);
    let mut idx: u8 = 1;
    let mut pr = |idx: &mut u8, v: u32| {
        gravar(nt, *idx as u32, v);
        *idx = idx.wrapping_add(1);
    };
    pr(&mut idx, cqf12_b(r + 0x66) as i8 as i32 as u32);
    olheiro(r + 0x1f, nt, &mut idx);
    pr(&mut idx, cqf12_b(r + 0x59) as i8 as i32 as u32);
    pr(&mut idx, cqf12_w(r + 0x5a) as i32 as u32);
    pr(&mut idx, cqf12_d(r + 0x50) as u32);
    dados(r, nt, &mut idx);
    pr(&mut idx, cqf12_d(r + 0x5e) as u32);
    pr(&mut idx, cqf12_d(r + 0x62) as u32);
    pr(&mut idx, cqf12_b(r + 0x58) as i8 as i32 as u32);
    pr(&mut idx, cqf12_d(r + 0x54) as u32);
    for k in 0..n.max(0) as usize {
        let p = r + 0x37 + k * 0x67;
        pr(&mut idx, cqf12_d(p + 0xc) as u32);
        pr(&mut idx, cqf12_d(p) as u32);
        pr(&mut idx, cqf12_d(p + 4) as u32);
        pr(&mut idx, cqf12_d(p + 8) as u32);
        pr(&mut idx, ((cqf12_b(p + 0x26) as i8 as i32) + (cqf12_b(p + 0x25) as i8 as i32) * 8) as u32);
    }
    pr(&mut idx, 0x420e9);
    let c = cqf12_i(r + 0x50);
    let clube = if c > -1 { cqf12_clube(c as usize) } else { 0 };
    let k = idx;
    idx = idx.wrapping_add(1);
    let _ = idx;
    gravar(nt, k as u32, forcas(cqf12_pessoa(cqf12_d(r + 0x2b)), clube));
    entregar(cqf12_d(0xb7431c), nt, cqf12_clube(cqf12_d(r + 0x50)));
}

/// Comparador da 0x7f9a60: a notícia é entregue (aloca na caixa do humano): só a original roda (ponytail: o efeito é a
/// caixa de entrada; os grupos que chegam aqui são conferidos pela 0x7f9890). Com o registro ligado, anota a chamada.
unsafe extern "thiscall" fn comparar_cqf12_noticia_do_relatorio(this: usize, r: usize, quantos: u32) {
    if let Some(g) = &mut *core::ptr::addr_of_mut!(CQF12_GRUPOS) {
        g.push((cqf12_d(r + 0x2b) as u32, cqf12_d(r + 0x50) as u32, quantos as u8));
    }
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1628)(this, r, quantos);
}
