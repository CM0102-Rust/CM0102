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

// ---------- 266a leva: a copa e as transferências: os jogos da rodada levados às datas marcadas, o jogador que sai da troca e as permissões de trabalho da oferta (Cup.cpp, transfer_manager.cpp) ----------

unsafe fn cqc7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc7_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqc7_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c] -> thiscall(notícias, clube),
/// ret 4).
unsafe fn cqc7_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc7_d(0xb7431c), clube) != 0
}

/// A pessoa (0x6e bytes em [0xae23bc]) e o clube (0x245 bytes em [0xae23b4]) pelo id.
unsafe fn cqc7_pessoa(id: usize) -> usize {
    cqc7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqc7_clube(id: usize) -> usize {
    cqc7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), gravado em [0xae1080].
unsafe fn cqc7_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqc7_d(0xae1050), pessoa);
    cqc7_pd(0xae1080, c);
    c
}

/// Lista vazia (palavra +0 = 0) da oferta (+4): destruída (0x68d040), liberada (0x944c94) e +4 = 0, como a original
/// (que lê a palavra antes de testar o ponteiro).
unsafe fn cqc7_soltar_lista_vazia(oferta: usize) {
    let l = cqc7_d(oferta + 4);
    if cqc7_w(l) != 0 {
        return;
    }
    if l != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(l);
        liberar(l);
    }
    cqc7_pd(oferta + 4, 0);
}

/// A entrada da rodada (0x68 bytes em [copa + 0xa3], o índice como o dword que a original multiplica).
unsafe fn cqc7_rodada(copa: usize, rodada: u32) -> usize {
    cqc7_d(copa + 0xa3).wrapping_add((rodada as usize).wrapping_mul(0x68))
}

/// Os jogos da copa na data e fatia da rodada, numa lista (0x5abd90: push lista; push liga +0x44; push rodada; push
/// id; push fatia +6; push dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c).
unsafe fn cqc7_jogos_da_rodada(copa: usize, data: usize, rodada: u32, lista: usize) -> u32 {
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    // ponytail: a original empurra o dia, a fatia e o tipo em registradores com sobra nos bytes altos; a 0x5abd90 só
    // lê as palavras e os bytes
    jogos(cqc7_d(0xae2a58), cqc7_w(data + 2) as u16 as u32, cqc7_w(data) as u16 as u32, cqc7_b(cqc7_rodada(copa, rodada) + 6) as u32,
        cqc7_d(cqc7_d(copa + 4)) as u32, rodada, cqc7_b(copa + 0x44) as u32, lista) & 0xffff
}

/// O jogo `f` está na data marcada da rodada: dia (+0x2a, sem sinal) = +3 da rodada, ano (+0x28) = +5 + o da copa
/// (+0x40), fatia (+0x3f) = +6 e o tipo (+0x42) da liga.
unsafe fn cqc7_na_data_da_rodada(copa: usize, e: usize, f: usize) -> bool {
    cqc7_w(f + 0x2a) as u16 as i32 == cqc7_w(e + 3) as i32
        && cqc7_w(f + 0x28) as u16 as i32 == cqc7_b(e + 5) as i8 as i32 + cqc7_w(copa + 0x40) as i32
        && cqc7_b(f + 0x3f) as i32 == cqc7_b(e + 6) as i8 as i32
        && cqc7_b(f + 0x42) == cqc7_b(copa + 0x44)
}

/// { 0x51ebe0 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: as notícias, a data e a lista
/// locais): `Cup::move_round_fixtures_to_set_dates(data, rodada)`, da montagem da rodada (0x51c800, em Rust). A notícia
/// 0x14 local é montada logo (0x77bbe0). Rodada (0x68 bytes em +0xa3) com o bit 0x100 em +0xd e a primeira vaga
/// marcada (+0x29 ≠ 0xff): os jogos da copa na data e fatia (cqc7_jogos_da_rodada). Com jogos, a notícia leva o id da
/// copa, +0x3a, o código da rodada (+7) e a rodada de duas mãos (+0x21 = 2). Cada jogo do tipo da liga (+0x42) ocupa
/// uma vaga (7 bytes a partir de +0x23: +0 regras do estádio, +4 dia da semana, +5 fatia, +6 tipo; até 8, até a
/// primeira 0xff): na data da rodada, vai ao dia da semana da vaga (0x549ef0 num bloco de 8 bytes de 0x944cf1, 0x54a2d0,
/// liberado depois), com a notícia 3 aos clubes fora do tipo 3, sai do calendário (0x5abc30), ganha a data, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta (0x5aa680); sem a copa ainda em [0xadadfc], para; senão a vaga fica
/// 0xff. Com ou sem a mudança, a notícia 0x14 vai aos clubes se um é de técnico humano. Sem vaga (fim das vagas ou
/// vaga vazia): só a notícia, com os dados do jogo.
unsafe extern "thiscall" fn copa_levar_jogos_as_datas(this: usize, data: usize, rodada: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut noticia = [0u32; 0x38];
    let n1 = noticia.as_mut_ptr() as usize;
    construir(n1, 0x14, 0);
    let id = cqc7_d(cqc7_d(this + 4));
    let e = cqc7_rodada(this, rodada);
    if cqc7_w(e + 0xd) as u16 & 0x100 == 0 || cqc7_b(e + 0x29) == 0xff {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    if cqc7_jogos_da_rodada(this, data, rodada, l) != 0 {
        gravar(n1, 0, cqc7_d(cqc7_d(this + 4)) as u32);
        gravar(n1, 1, cqc7_w(this + 0x3a) as i32 as u32);
        gravar(n1, 2, cqc7_w(cqc7_rodada(this, rodada) + 7) as i32 as u32);
        gravar(n1, 5, (cqc7_b(cqc7_rodada(this, rodada) + 0x21) == 2) as u32);
        // a notícia 0x14 do jogo (dados 3, 4, 6, 7, 8) aos dois clubes, se um deles é de técnico humano
        let noticia_do_jogo = |c1: usize, c2: usize, i1: usize, i2: usize, est: usize, dia: u16, ano: u16| {
            if cqc7_humano(c1) || cqc7_humano(c2) {
                if cqc7_humano(c1) {
                    gravar(n1, 3, i1 as u32);
                    gravar(n1, 4, i2 as u32);
                } else {
                    gravar(n1, 3, i2 as u32);
                    gravar(n1, 4, i1 as u32);
                }
                gravar(n1, 6, est as u32);
                gravar(n1, 7, dia as u32);
                gravar(n1, 8, ano as u32);
                entregar(cqc7_d(0xb7431c), n1, c1);
                entregar(cqc7_d(0xb7431c), n1, c2);
            }
        };
        cqc7_pd(l + 0xa, cqc7_d(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if cqc7_b(this + 0x44) == cqc7_b(f + 0x42) {
                let e = cqc7_rodada(this, rodada);
                if j < 0x38 && cqc7_b(e + j + 0x29) != 0xff {
                    let mut buf = [0u8; 0x50];
                    let r = buf.as_mut_ptr() as usize;
                    core::ptr::copy_nonoverlapping(f as *const u8, r as *mut u8, 0x4f);
                    if cqc7_na_data_da_rodada(this, e, f) {
                        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
                        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
                        let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
                        let p = alocar(8);
                        let dp = if p != 0 { nova(p, cqc7_w(f + 0x2a) as u16 as u32, cqc7_w(f + 0x28) as u16 as u32) } else { 0 };
                        let mut t = [0u8; 8];
                        let pt = t.as_mut_ptr() as usize;
                        padrao(pt);
                        let e = cqc7_rodada(this, rodada);
                        ajustar(dp, cqc7_b(e + j + 0x27) as i8 as i16 as u16 as u32, pt); // cqc7: o dia da semana marcado
                        if cqc7_b(cqc7_rodada(this, rodada) + j + 0x29) != 3 {
                            let mut noticia2 = [0u32; 0x38];
                            let n2 = noticia2.as_mut_ptr() as usize;
                            construir(n2, 3, 0);
                            gravar(n2, 0, cqc7_d(r + 0xc) as u32);
                            gravar(n2, 1, cqc7_d(r + 0x10) as u32);
                            gravar(n2, 2, cqc7_d(r) as u32);
                            gravar(n2, 3, cqc7_w(r + 0x32) as i32 as u32);
                            gravar(n2, 4, cqc7_w(r + 0x30) as i32 as u32);
                            gravar(n2, 5, cqc7_w(r + 0x2a) as u16 as u32);
                            gravar(n2, 6, cqc7_w(r + 0x28) as u16 as u32);
                            gravar(n2, 7, cqc7_b(r + 0x3f) as u32);
                            gravar(n2, 8, cqc7_w(pt) as i32 as u32);
                            gravar(n2, 9, cqc7_w(pt + 2) as u16 as u32);
                            gravar(n2, 10, cqc7_b(cqc7_rodada(this, rodada) + j + 0x28) as i8 as i32 as u32);
                            gravar(n2, 11, cqc7_b(cqc7_rodada(this, rodada) + j + 0x29) as i8 as i32 as u32);
                            entregar(cqc7_d(0xb7431c), n2, cqc7_d(r + 0x1c));
                            entregar(cqc7_d(0xb7431c), n2, cqc7_d(r + 0x20));
                        }
                        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
                        tirar(cqc7_d(0xae2a58), r);
                        crate::comparacao::ramo("cqc_7 jogo levado a data");
                        cqc7_pw(r + 0x2a, cqc7_w(pt) as u16); // cqc7: o dia marcado
                        cqc7_pw(r + 0x28, cqc7_w(pt + 2) as u16);
                        let v = cqc7_rodada(this, rodada) + j;
                        cqc7_pb(r + 0x3f, cqc7_b(v + 0x28)); // cqc7: a fatia marcada
                        cqc7_pb(r + 0x3b, cqc7_b(v + 0x29));
                        let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
                        estadio(cqc7_d(0xdd7ef4), cqc7_d(cqc7_rodada(this, rodada) + j + 0x23), r, cqc7_d(this + 0x24));
                        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
                        juntar(cqc7_d(0xae2a58), r, 1);
                        if dp != 0 {
                            let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
                            liberar(dp);
                        }
                        if cqc7_d(cqc7_d(0xadadfc) + id * 4) == 0 {
                            destruir(l);
                            return;
                        }
                        cqc7_pb(cqc7_rodada(this, rodada) + j + 0x29, 0xff);
                    }
                    noticia_do_jogo(cqc7_d(r + 0x1c), cqc7_d(r + 0x20), cqc7_d(r + 0xc), cqc7_d(r + 0x10), cqc7_d(r + 4),
                        cqc7_w(r + 0x2a) as u16, cqc7_w(r + 0x28) as u16);
                    j += 7;
                } else {
                    noticia_do_jogo(cqc7_d(f + 0x1c), cqc7_d(f + 0x20), cqc7_d(f + 0xc), cqc7_d(f + 0x10), cqc7_d(f + 4),
                        cqc7_w(f + 0x2a) as u16, cqc7_w(f + 0x28) as u16);
                }
            }
            f = proximo(l);
        }
    }
    destruir(l);
}

/// Comparador da 0x51ebe0: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, e as notícias
/// vão aos técnicos). O plano, feito antes: os jogos da copa na data e fatia (0x5abd90 numa lista provisória), na
/// ordem da lista; cada um do tipo da liga ocupa a vaga seguinte (de 7 em 7 bytes, até 0x38 e até a primeira 0xff), e
/// a vaga do que está na data da rodada fica 0xff; as outras vagas não mudam. Sem a copa em [0xadadfc] depois: nada a
/// conferir.
unsafe extern "thiscall" fn comparar_copa_levar_jogos_as_datas(this: usize, data: usize, rodada: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1524);
    let e = cqc7_rodada(this, rodada);
    if cqc7_w(e + 0xd) as u16 & 0x100 == 0 || cqc7_b(e + 0x29) == 0xff {
        return o(this, data, rodada);
    }
    let id = cqc7_d(cqc7_d(this + 4));
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let mut esperado = [0u8; 8];
    for k in 0..8 {
        esperado[k] = cqc7_b(e + k * 7 + 0x29);
    }
    if cqc7_jogos_da_rodada(this, data, rodada, l) != 0 {
        cqc7_pd(l + 0xa, cqc7_d(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if cqc7_b(this + 0x44) == cqc7_b(f + 0x42) && j < 0x38 && esperado[j / 7] != 0xff {
                if cqc7_na_data_da_rodada(this, e, f) {
                    esperado[j / 7] = 0xff;
                }
                j += 7;
            }
            f = proximo(l);
        }
    }
    destruir(l);
    o(this, data, rodada);
    if cqc7_d(cqc7_d(0xadadfc) + id * 4) == 0 {
        return; // ponytail: a copa saiu do jogo; nada a conferir
    }
    let e = cqc7_rodada(this, rodada);
    let depois: [u8; 8] = core::array::from_fn(|k| cqc7_b(e + k * 7 + 0x29));
    PLACAR.registrar(depois == esperado, || format!("copa_levar_jogos_as_datas(rodada {}) vagas {:02x?} (esperado {:02x?})",
        rodada, depois, esperado));
}

/// { 0x8c41c0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a lista local):
/// `TRANSFER_MANAGER::remove_exchange_player(pessoa, oferta, só_esta, motivo)`, da oferta de contrato ao jogador
/// (0x8c1e40), da confirmação pelo clube (0x8c3310), da soltura dos jogadores da oferta (0x8c6ae0) e da tela. Só esta:
/// sem a lista da troca (+4), nada; com a pessoa nela (+0 de cada item = id), sai (0x68d1f0), o registro de 8 bytes dela
/// ([this+0x86e] + id × 8, +2) perde uma oferta, a notícia (0x8de150 com o valor = +0x52 da pessoa + +0x1b da oferta e o
/// motivo) e, com o dono do contrato do alvo (0x4cf5a0, fora do tipo 5) do computador (0x600a50), a resposta dele
/// (0x8c8090 com &oferta e 1): oferta resolvida (status 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3) só solta a lista
/// vazia; senão o preço (+0x1b) vira o valor e a oferta segue (0x8c2f50). No fim, a lista vazia sai. Senão, com a pessoa
/// em alguma oferta (+2 do registro): as ofertas dela pelo clube (0x8c5e40 numa lista local), menos esta; em cada uma, sai
/// da lista da troca (a lista vazia sai) e, desde a primeira que a tinha, o preço vira +0x52 + o preço, a notícia (motivo
/// 0) e a oferta segue.
unsafe extern "thiscall" fn tirar_jogador_da_troca(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x8de150usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let menos_uma = |p: usize| {
        let a = cqc7_d(this + 0x86e) + cqc7_d(p) * 8 + 2;
        cqc7_pb(a, cqc7_b(a).wrapping_sub(1));
    };
    let mut oferta = oferta;
    if so_esta != 0 {
        let c = cqc7_contrato(cqc7_pessoa(cqc7_d(oferta + 0xc)));
        let dono = if c != 0 && cqc7_b(c + 0x35) & 0x3f != 5 { cqc7_clube(cqc7_d(c + 4)) } else { 0 };
        let troca = cqc7_d(oferta + 4);
        if troca == 0 {
            destruir(l);
            return;
        }
        cqc7_pd(troca + 0xa, cqc7_d(troca + 2));
        let mut it = proximo(cqc7_d(oferta + 4));
        while it != 0 && cqc7_d(pessoa) != cqc7_d(it) {
            it = proximo(cqc7_d(oferta + 4));
        }
        if it != 0 {
            tirar(cqc7_d(oferta + 4), it);
            menos_uma(pessoa);
            let v = cqc7_d(pessoa + 0x52).wrapping_add(cqc7_d(oferta + 0x1b));
            noticia(this, oferta, pessoa, v, motivo);
            let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if dono != 0 && clube_humano(dono, 0, 1, 0, 0) == 0 {
                let resposta: unsafe extern "thiscall" fn(usize, *mut usize, u32) = core::mem::transmute(0x8c8090usize);
                resposta(this, &mut oferta, 1);
                let s = cqc7_b(oferta + 0x2c);
                if matches!(s, 2 | 3 | 0x14 | 0x10 | 4) || (s == 0xf && cqc7_b(oferta + 0x2e) >= 3) {
                    cqc7_soltar_lista_vazia(oferta);
                    destruir(l);
                    return;
                }
            }
            crate::comparacao::ramo("cqc_7 sai da troca da oferta");
            cqc7_pd(oferta + 0x1b, v);
            seguir(this, oferta);
        }
        cqc7_soltar_lista_vazia(oferta);
    } else {
        if cqc7_b(cqc7_d(this + 0x86e) + cqc7_d(pessoa) * 8 + 2) != 0 {
            let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
            trocas(this, cqc7_pessoa(cqc7_d(oferta + 0xc)), l);
            cqc7_pd(l + 0xa, cqc7_d(l + 2));
            let mut achou = false;
            let mut it = proximo(l);
            while it != 0 {
                let o = cqc7_d(cqc7_d(this) + cqc7_d(it) * 4);
                if o != oferta {
                    if cqc7_d(o + 4) != 0 {
                        let t = cqc7_d(o + 4);
                        cqc7_pd(t + 0xa, cqc7_d(t + 2));
                        let mut x = proximo(cqc7_d(o + 4));
                        while x != 0 {
                            if cqc7_d(pessoa) == cqc7_d(x) {
                                achou = true;
                                tirar(cqc7_d(o + 4), x);
                                menos_uma(pessoa);
                                break;
                            }
                            x = proximo(cqc7_d(o + 4));
                        }
                        cqc7_soltar_lista_vazia(o);
                    }
                    if achou {
                        crate::comparacao::ramo("cqc_7 sai da troca das outras");
                        let v = cqc7_d(pessoa + 0x52).wrapping_add(cqc7_d(o + 0x1b));
                        noticia(this, o, pessoa, v, 0);
                        cqc7_pd(o + 0x1b, v);
                        seguir(this, o);
                    }
                }
                it = proximo(l);
            }
        }
    }
    destruir(l);
}

/// Comparador da 0x8c41c0: só a original (os nós da troca são liberados, a notícia vai aos técnicos e a oferta segue
/// pela 0x8c2f50, que pode fechar a transferência e tirar outros jogadores das trocas, ou ser respondida pelo vendedor
/// do computador, 0x8c8090, que pode mudar a oferta da vaga).
unsafe extern "thiscall" fn comparar_tirar_jogador_da_troca(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1525)(this, pessoa, oferta, so_esta, motivo) // ponytail: o efeito segue por outras rotinas
}

/// A permissão de trabalho da pessoa no clube: precisa (0x8c7560: push 0; push clube; push pessoa; mov ecx, this ->
/// thiscall, ret 0xc), a proposta de contrato dela ao clube (0x4cc9e0: push id do clube; push id da pessoa; mov ecx,
/// [0xae1050] -> thiscall, ret 8) e a concessão (0x8c7950: push 0; push 0; push proposta; push nação do clube +0x53; push
/// pessoa; mov ecx, this -> thiscall, ret 0x14, sem sorteio). Devolve (precisa, proposta, concedida).
unsafe fn cqc7_permissao(this: usize, pessoa: usize, clube: usize) -> (bool, usize, bool) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if precisa(this, pessoa, clube, 0) == 0 {
        return (false, 0, false);
    }
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let r = proposta(cqc7_d(0xae1050), cqc7_d(pessoa) as u32, cqc7_d(clube) as u32);
    if r == 0 {
        return (true, 0, false);
    }
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7950usize);
    (true, r, concedida(this, pessoa, cqc7_d(clube + 0x53), r, 0, 0) != 0)
}

/// { 0x8c7690 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::check_offer_work_permits(oferta, quieto)`,
/// do processamento da oferta (status 0xa, 0x8c6880) e da conclusão (0x8cbc40). O dono do contrato do alvo (0x4cf5a0,
/// [0xae1080]; qualquer tipo). Alvo (+0xc) com permissão a pedir no comprador (+8) e proposta, sem concessão
/// (cqc7_permissao): a oferta desfeita (0x8de790 com 0x13), status 0x10, as propostas fechadas (0x8c6db0), as marcas de
/// lance saem (0x84aeb0 no gerente de listas [this+0x856]: o comprador e o alvo, depois o dono do contrato do alvo e cada
/// jogador da troca), reagendada em 10 dias (0x8c6220) e devolve 0. Concedida (ou sem proposta) e sem `quieto`: a
/// notícia da concessão (0x8e3130). Depois, cada jogador da troca (+4) no dono do contrato do alvo: recusado, a notícia
/// (0x8e35a0 com o valor +0x52 + o preço), sai da troca (0x68d1f0), o registro de 8 bytes dele perde uma oferta e o
/// preço (+0x1b) vira o valor; concedido sem `quieto`, a notícia. A lista vazia sai; sem `quieto`, a oferta segue
/// (0x8c2f50). Devolve 1.
unsafe extern "thiscall" fn permissoes_da_oferta(this: usize, oferta: usize, quieto: u32) -> u32 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let concedida_noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8e3130usize);
    let alvo = cqc7_pessoa(cqc7_d(oferta + 0xc));
    let comprador = cqc7_clube(cqc7_d(oferta + 8));
    let c = cqc7_contrato(alvo);
    let dono = if c != 0 { cqc7_clube(cqc7_d(c + 4)) } else { 0 };
    let (precisa, proposta, concedida) = cqc7_permissao(this, alvo, comprador);
    if precisa {
        if proposta != 0 && !concedida {
            crate::comparacao::ramo("cqc_7 permissao recusada");
            let desfeita: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8de790usize);
            let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
            let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
            let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
            desfeita(this, oferta, comprador, 0x13);
            cqc7_pb(oferta + 0x2c, 0x10); // cqc7: a oferta cai pela permissão
            fechar(this, oferta);
            let alvo = cqc7_pessoa(cqc7_d(oferta + 0xc));
            let c = cqc7_contrato(alvo);
            let dono = if c != 0 { cqc7_clube(cqc7_d(c + 4)) } else { 0 };
            desmarcar(cqc7_d(this + 0x856), cqc7_clube(cqc7_d(oferta + 8)), alvo);
            if cqc7_d(oferta + 4) != 0 {
                let t = cqc7_d(oferta + 4);
                cqc7_pd(t + 0xa, cqc7_d(t + 2));
                let mut it = proximo(cqc7_d(oferta + 4));
                while it != 0 {
                    desmarcar(cqc7_d(this + 0x856), dono, cqc7_pessoa(cqc7_d(it)));
                    it = proximo(cqc7_d(oferta + 4));
                }
            }
            agendar(this, oferta, 0xa, 0);
            return 0;
        }
        if quieto == 0 {
            concedida_noticia(this, alvo, comprador, oferta);
        }
    }
    if cqc7_d(oferta + 4) != 0 {
        let t = cqc7_d(oferta + 4);
        cqc7_pd(t + 0xa, cqc7_d(t + 2));
        let mut it = proximo(cqc7_d(oferta + 4));
        while it != 0 {
            let x = cqc7_pessoa(cqc7_d(it));
            let (precisa, proposta, concedida) = cqc7_permissao(this, x, dono);
            if precisa {
                if proposta != 0 && !concedida {
                    crate::comparacao::ramo("cqc_7 troca recusada pela permissao");
                    let v = cqc7_d(x + 0x52).wrapping_add(cqc7_d(oferta + 0x1b));
                    let recusada: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8e35a0usize);
                    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                    recusada(this, oferta, x, v);
                    tirar(cqc7_d(oferta + 4), it);
                    let a = cqc7_d(this + 0x86e) + cqc7_d(x) * 8 + 2;
                    cqc7_pb(a, cqc7_b(a).wrapping_sub(1));
                    cqc7_pd(oferta + 0x1b, v);
                } else if quieto == 0 {
                    concedida_noticia(this, x, dono, oferta);
                }
            }
            it = proximo(cqc7_d(oferta + 4));
        }
        cqc7_soltar_lista_vazia(oferta);
    }
    if quieto == 0 {
        let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
        seguir(this, oferta);
    }
    1
}

/// Comparador da 0x8c7690: só a original (as notícias alocam, a oferta é reagendada e segue pela 0x8c2f50). O plano,
/// feito antes pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo recusado devolve 0 com a
/// oferta no status 0x10; senão 1.
unsafe extern "thiscall" fn comparar_permissoes_da_oferta(this: usize, oferta: usize, quieto: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1526);
    let (precisa, proposta, concedida) = cqc7_permissao(this, cqc7_pessoa(cqc7_d(oferta + 0xc)), cqc7_clube(cqc7_d(oferta + 8)));
    let recusada = precisa && proposta != 0 && !concedida;
    let r = o(this, oferta, quieto);
    let ok = if recusada { r == 0 && cqc7_b(oferta + 0x2c) == 0x10 } else { r == 1 };
    PLACAR.registrar(ok, || format!("permissoes_da_oferta(oferta {:x}) recusada prevista {} devolveu {}", oferta, recusada, r));
    r
}

// ---------- 266a leva: o dia das transferências: o dia da gerência, a oferta que vence hoje e as restrições da oferta (transfer_manager.cpp) ----------

unsafe fn cqd1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// A pessoa do id (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cqd1_pessoa(id: usize) -> usize {
    cqd1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do id (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqd1_clube(id: usize) -> usize {
    cqd1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080] como
/// a original faz, e o clube dele (+4); sem contrato, ou com o tipo 5 em +0x35 quando `sem_5`: 0.
unsafe fn cqd1_clube_do_contrato(pessoa: usize, sem_5: bool) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqd1_d(0xae1050), pessoa);
    cqd1_pd(0xae1080, c);
    if c == 0 || (sem_5 && cqd1_b(c + 0x35) & 0x3f == 5) {
        0
    } else {
        cqd1_clube(cqd1_d(c + 4))
    }
}

/// O clube é humano (0x600a50: push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
unsafe fn cqd1_humano(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0) != 0
}

/// Reagenda a oferta (0x8c6220: push 0; push dias; push oferta; mov ecx, this -> thiscall, ret 0xc).
unsafe fn cqd1_agendar(this: usize, o: usize, dias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, o, dias, 0);
}

/// O próximo da lista (0x68d1b0: mov ecx, lista -> thiscall, ret 0).
unsafe fn cqd1_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// O bloco que a 0x8c63a0 repete: o clube do contrato do alvo (sem olhar o tipo 5), os lances do comprador (+8) pelo
/// alvo (+0xc) desmarcados na lista de observação (0x84aeb0: push jogador; push clube; mov ecx, [this+0x856] ->
/// thiscall, ret 8) e os do clube do contrato por cada jogador da troca (lista em +4, cursor no começo).
unsafe fn cqd1_desmarcar(this: usize, o: usize) {
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let atual = cqd1_clube_do_contrato(alvo, false);
    desmarcar(cqd1_d(this + 0x856), cqd1_clube(cqd1_d(o + 8)), alvo);
    let l = cqd1_d(o + 4);
    if l == 0 {
        return;
    }
    cqd1_pd(l + 0xa, cqd1_d(l + 2));
    loop {
        let p = cqd1_proximo(cqd1_d(o + 4));
        if p == 0 {
            break;
        }
        desmarcar(cqd1_d(this + 0x856), atual, cqd1_pessoa(cqd1_d(p)));
    }
}

/// { 0x8c5880 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::daily_update(hoje, fatia)`. Sem data base (ano +0x84f zerado), ela vira hoje (0x417c10: push
/// hoje; push saída; lea ecx, [this+0x84d] -> thiscall, ret 8) e a fatia +0x855 a de hoje. Só com hoje igual à base e a
/// fatia igual a +0x855: na fatia 0 os avisos das janelas (0x8c6bc0: push hoje; mov ecx, this -> thiscall, ret 4); no
/// dia múltiplo de 7, cada oferta vencida (data +0x12/+0x14 antes de hoje; status +0x2c fora de 0x13, 7, 8 e 0xa) é
/// reagendada para amanhã (0x8c6220 com 1 dia); a fatia +0x855 sobe 1; a fila da transferência livre (0x8c18c0: mov
/// ecx, this -> thiscall, ret 0); o balde da vez (listas de 0xe bytes em this+0x18 + 0xe·(3·[+0x84c] + fatia), cursor no
/// começo): cada índice válido do vetor [this] (conta +4) com a oferta de hoje e desta fatia (+0x1a) é processado
/// (0x8c63a0: push vaga; mov ecx, this -> thiscall, ret 4); o balde é esvaziado (0x68d080). Na fatia 2: a base passa a
/// amanhã (0x54c5b0 push 1; push saída; mov ecx, base -> thiscall, ret 8 e 0x417c10), +0x855 volta a 0, +0x84c sobe (de
/// 49 volta a 0); mudando o mês de hoje para amanhã (0x54cfb0: mov ecx, data -> thiscall, ret 0), as prestações
/// (0x8caee0); no dia 0, as transferências antigas aos arquivos (0x8c3eb0) e as permissões de trabalho (0x8c7330); o
/// virtual +0 de cada uma das 28 regras de nação ([this+0x8ac], push hoje; mov ecx, regras); no dia múltiplo de 7, as
/// permissões de novo. Na fatia 0 (fora a 2), as transferências futuras (0x8cbbe0). No dia 100 da fatia 2: as datas 100
/// de 5, 2 e 10 anos atrás (0x549ef0: push ano; push 0x64; lea ecx, local -> thiscall, ret 8) e, para cada pessoa
/// (registros de 0x6e bytes em [0xae23bc], conta [0xae2364], relida) com a data +0x3e/+0x40 antes da de 2 anos, clube
/// ([[+0x39]+0x53]) numa nação ([+0x71]) de id [0x9cfa10] e sem ser comunitária lá (0x53d740: push clube; push pessoa;
/// add esp, 8 -> cdecl): +0x1e (a segunda nacionalidade) = a nação do clube (clube +0x53), se ela é [0x9cf488] ou [0x9cf434]; se a data é
/// antes da de 5 anos e ela não é [0x9cf360], [0x9cf2fc] ou [0x9cf448]; ou se a data é antes da de 10 anos.
unsafe extern "thiscall" fn dia_das_transferencias(this: usize, hoje: usize, fatia: u32) {
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let chamar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c18c0usize);
    let janelas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6bc0usize);
    let processar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c63a0usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let prestacoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8caee0usize);
    let antigas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c3eb0usize);
    let permissoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c7330usize);
    let futuras: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8cbbe0usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let f = fatia as u8;
    let mut t20 = [0u8; 8];
    if cqd1_w(this + 0x84f) == 0 {
        copia(this + 0x84d, t20.as_mut_ptr() as usize, hoje);
        cqd1_pb(this + 0x855, f);
    }
    if cqd1_w(hoje + 2) != cqd1_w(this + 0x84f) || cqd1_w(hoje) != cqd1_w(this + 0x84d) || cqd1_b(this + 0x855) != f {
        return;
    }
    if f == 0 {
        janelas(this, hoje);
    }
    if cqd1_w(hoje) as i32 % 7 == 0 {
        let mut i: i32 = 0;
        while i < cqd1_d(this + 4) as i32 {
            let o = cqd1_d(cqd1_d(this).wrapping_add((i as usize).wrapping_mul(4)));
            if o != 0 {
                let (ha, oa) = (cqd1_w(hoje + 2) as u16, cqd1_w(o + 0x14) as u16);
                let vencida = ha > oa || (ha == oa && cqd1_w(hoje) > cqd1_w(o + 0x12));
                let s = cqd1_b(o + 0x2c);
                if vencida && s != 0x13 && s != 7 && s != 8 && s != 0xa {
                    cqd1_agendar(this, o, 1);
                }
            }
            i += 1;
        }
    }
    cqd1_pb(this + 0x855, cqd1_b(this + 0x855).wrapping_add(1));
    chamar(this);
    let balde = |this: usize| {
        let k = (cqd1_b(this + 0x84c) as i8 as i32) * 3 + f as i8 as i32;
        this.wrapping_add((k as isize * 0xe) as usize).wrapping_add(0x18)
    };
    let l = balde(this);
    cqd1_pd(l + 0xa, cqd1_d(l + 2));
    loop {
        let p = cqd1_proximo(balde(this));
        if p == 0 {
            break;
        }
        let idx = cqd1_d(p) as i32;
        if idx < cqd1_d(this + 4) as i32 && idx >= 0 {
            let vaga = cqd1_d(this).wrapping_add(idx as usize * 4);
            let o = cqd1_d(vaga);
            if o != 0 && cqd1_w(hoje + 2) == cqd1_w(o + 0x14) && cqd1_w(hoje) == cqd1_w(o + 0x12)
                && cqd1_b(o + 0x1a) == f { // cqd1: a oferta desta fatia
                crate::comparacao::ramo("cqd_1 oferta do dia");
                processar(this, vaga);
            }
        }
    }
    esvaziar(balde(this));
    if f == 2 {
        let (mut t28, mut t30) = ([0u8; 8], [0u8; 8]);
        let r = mais(this + 0x84d, t28.as_mut_ptr() as usize, 1);
        copia(this + 0x84d, t30.as_mut_ptr() as usize, r);
        cqd1_pb(this + 0x855, 0);
        let j = cqd1_b(this + 0x84c).wrapping_add(1);
        cqd1_pb(this + 0x84c, j);
        if (j as i8) >= 0x32 {
            cqd1_pb(this + 0x84c, 0);
        }
        let amanha = mais(hoje, t30.as_mut_ptr() as usize, 1);
        let m = mes(hoje);
        if m != mes(amanha) {
            crate::comparacao::ramo("cqd_1 prestações do mês");
            prestacoes(this); // cqd1: as prestações na virada do mês
        }
        if cqd1_w(hoje) == 0 {
            antigas(this);
            permissoes(this);
        }
        for k in 0..0x1c {
            let r = cqd1_d(this + 0x8ac + k * 4);
            if r != 0 {
                let v: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqd1_d(cqd1_d(r)));
                v(r, hoje);
            }
        }
        if cqd1_w(hoje) as i32 % 7 == 0 {
            permissoes(this);
        }
    } else if f == 0 {
        futuras(this);
    }
    if cqd1_w(hoje) != 100 || f != 2 {
        return;
    }
    // ponytail: a original empurra o ano em eax com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
    let ano = cqd1_w(hoje + 2) as u16;
    let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
    let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
    let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
    let mut s = cqd1_d(0xae23bc);
    let mut i: i32 = 0;
    while i < cqd1_d(0xae2364) as i32 {
        if cqd1_naturaliza(s, p2, p5, p10, comunitaria) {
            cqd1_pd(s + 0x1e, cqd1_d(cqd1_d(s + 0x39) + 0x53));
        }
        i += 1;
        s += 0x6e;
    }
}

/// A condição do dia 100 da 0x8c5880 (também para o comparador): a data +0x3e (dia com sinal)/+0x40 (ano sem sinal) da
/// pessoa antes da de 2 anos, o clube na nação [0x9cfa10] sem ser comunitária, e o caso pelo id do clube.
unsafe fn cqd1_naturaliza(s: usize, p2: usize, p5: usize, p10: usize, comunitaria: unsafe extern "C" fn(usize, usize) -> u32) -> bool {
    let antes = |d: usize| {
        let (da, sa) = (cqd1_w(d + 2) as u16, cqd1_w(s + 0x40) as u16);
        da > sa || (da == sa && cqd1_w(d) > cqd1_w(s + 0x3e))
    };
    if !antes(p2) {
        return false;
    }
    let j = cqd1_d(s + 0x39);
    if j == 0 {
        return false;
    }
    let c = cqd1_d(j + 0x53);
    if c == 0 {
        return false;
    }
    let n = cqd1_d(c + 0x71);
    if n == 0 || cqd1_d(n) != cqd1_d(0x9cfa10) || comunitaria(s, c) != 0 {
        return false;
    }
    let id = cqd1_d(c);
    if id == cqd1_d(0x9cf488) || id == cqd1_d(0x9cf434) {
        return true;
    }
    if antes(p5) && id != cqd1_d(0x9cf360) && id != cqd1_d(0x9cf2fc) && id != cqd1_d(0x9cf448) {
        return true;
    }
    antes(p10)
}

/// Comparador da 0x8c5880: só a original (ela processa as ofertas, que mudam elencos, finanças e notícias). Confere o
/// que ela garante, contra o estado de antes: rodando o dia (hoje igual à base e a fatia igual a +0x855), na fatia 2 a
/// base vira amanhã, +0x855 volta a 0 e +0x84c sobe (de 49 a 0); nas outras +0x855 sobe 1; e no dia 100 da fatia 2
/// cada pessoa que cumpre a condição (o laço é o último passo, então o estado de depois é o dele) fica com +0x1e (a
/// segunda nacionalidade) = a nação do clube: a naturalização do dia 100.
unsafe extern "thiscall" fn comparar_dia_das_transferencias(this: usize, hoje: usize, fatia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1546);
    let f = fatia as u8;
    let sem_base = cqd1_w(this + 0x84f) == 0;
    let roda = if sem_base {
        true
    } else {
        cqd1_w(hoje + 2) == cqd1_w(this + 0x84f) && cqd1_w(hoje) == cqd1_w(this + 0x84d) && cqd1_b(this + 0x855) == f
    };
    let (f855, j84c) = (if sem_base { f } else { cqd1_b(this + 0x855) }, cqd1_b(this + 0x84c));
    let mut amanha = [0u8; 8];
    {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let r = mais(hoje, amanha.as_mut_ptr() as usize, 1);
        amanha = core::ptr::read_unaligned(r as *const [u8; 8]);
    }
    o(this, hoje, fatia);
    if !roda {
        return;
    }
    let mut ok;
    if f == 2 {
        let j = j84c.wrapping_add(1);
        let esperado = if (j as i8) >= 0x32 { 0 } else { j };
        let base = core::ptr::read_unaligned((this + 0x84d) as *const [u8; 4]);
        ok = cqd1_b(this + 0x855) == 0 && cqd1_b(this + 0x84c) == esperado && base[..] == amanha[..4];
    } else {
        ok = cqd1_b(this + 0x855) == f855.wrapping_add(1);
    }
    let mut erradas = 0u32;
    if f == 2 && cqd1_w(hoje) == 100 {
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
        let ano = cqd1_w(hoje + 2) as u16;
        let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
        let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
        let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
        let mut s = cqd1_d(0xae23bc);
        for _ in 0..(cqd1_d(0xae2364) as i32).max(0) {
            if cqd1_naturaliza(s, p2, p5, p10, comunitaria) && cqd1_d(s + 0x1e) != cqd1_d(cqd1_d(s + 0x39) + 0x53) {
                erradas += 1;
            }
            s += 0x6e;
        }
        ok = ok && erradas == 0;
    }
    PLACAR.registrar(ok, || format!("dia_das_transferencias(dia {}, fatia {}): +0x855 {} -> {}, +0x84c {} -> {}, pessoas sem a nação do clube em +0x1e {}",
        cqd1_w(hoje), f, f855, cqd1_b(this + 0x855), j84c, cqd1_b(this + 0x84c), erradas));
}

/// { 0x8c63a0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH: as listas locais, construídas pela 0x68d020 e
/// destruídas pela 0x68d040 na mesma ordem): `TRANSFER_MANAGER::process_due_offer(vaga)`, a oferta que vence hoje, pelo
/// status +0x2c. O vendedor é o clube do contrato do alvo (+0xc; fora o tipo 5), o comprador o +8.
/// - 0: sem vendedor, a aceitação do lance (0x8c0080: push 0; push vaga -> thiscall, ret 8). Vendedor humano: status 4,
///   14 dias (0x8c6220) e os lances desmarcados (cqd1_desmarcar). Senão, sem a marca +0x1f = 1, a resposta da IA
///   (0x8c8090: push 0; push vaga, ret 8); com ela, a previsão do vendedor (0x84af00, stdcall(vendedor, alvo, 0 i64,
///   comprador, −1 i64, 1): o ecx = [this+0x856] não é lido): recusa, a do empréstimo (0x8bfa30: push 0; push 0; push
///   vaga -> thiscall, ret 0xc); aceita, a decisão (0x4cc930: push oferta; mov ecx, [0xae1050] -> thiscall, ret 4),
///   status 8 e a notícia da aceitação (0x8d76d0: push oferta, ret 4); com a decisão, a notícia da recusa em se mudar
///   (0x8d8310: push 0; push oferta, ret 8), status 0xf, +0x2e = 3, os lances desmarcados, 10 dias, e se a oferta e as
///   da lista em +0 estão fechadas (0x8d0d90, stdcall(oferta)), o bit 0 do registro de 8 bytes do alvo em [this+0x86e]
///   apagado.
/// - 2, 3, 4, 0xf, 0x10, 0x14: com 0xf e +0x2e < 3, +0x2e = 3 e 10 dias; senão a oferta sai (0x8d0f60: push oferta,
///   ret 4).
/// - 1: comprador humano: status 4, os lances desmarcados e 10 dias; senão 0x8c0f40 (push vaga, ret 4) com +0x1f = 1 ou
///   0x8c0810 (push 0; push vaga, ret 8).
/// - 5, 6: comprador humano: status 9 e 10 dias; senão o contrato ao alvo (0x8c1e40: push oferta; push alvo; push
///   comprador, ret 0xc).
/// - 7 com troca (+4): vendedor humano: status 8 e 10 dias; senão, numa cópia da lista (0x68d2f0: push lista; lea ecx,
///   local, ret 4), o contrato a cada jogador da troca até a oferta ficar ativa.
/// - 0xa: as permissões (0x8c7690: push 0; push oferta, ret 8). 0xe: a conclusão (0x8c34e0: push 0; push oferta, ret 8).
/// - 0x11, 0x12: status 0; com +0x10, a oferta passa aos clubes (0x8c2f50: push oferta, ret 4); senão, com técnico
///   humano no clube do contrato (0x7875e0, stdcall(clube): o ecx = [0xb7431c] não é lido), a notícia da oferta feita
///   (0x8d3090: push oferta, ret 4) e 28 dias; senão o valor da oferta (0x8e7b50: mov ecx, oferta -> thiscall(oferta, 0,
///   0, 0, 0, 1), ret 0x14) e a previsão do vendedor (vendedor, alvo, valor, comprador, −1, 0): 0 aceita (0x8c0080),
///   1 ou 2 recusa (0x8bf680: push 0; push 0; push vaga, ret 0xc).
unsafe extern "thiscall" fn processar_oferta_vencida(this: usize, vaga: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut l1 = [0u8; 0x10];
    lista_nova(l1.as_mut_ptr() as usize);
    cqd1_processar(this, vaga);
    destruir(l1.as_mut_ptr() as usize);
}

unsafe fn cqd1_processar(this: usize, vaga: usize) {
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0080usize);
    let responder: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c8090usize);
    let prever: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(0x84af00usize);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bfa30usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bf680usize);
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cc930usize);
    let aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d76d0usize);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d8310usize);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x8d0d90usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d0f60usize);
    let ia_1a: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c0f40usize);
    let ia_1b: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0810usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x8c1e40usize);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8c7690usize);
    let concluir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c34e0usize);
    let passar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let tecnico_humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_feita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d3090usize);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let o = cqd1_d(vaga);
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let vendedor = cqd1_clube_do_contrato(alvo, true);
    let comprador = cqd1_clube(cqd1_d(o + 8));
    let s = cqd1_b(o + 0x2c);
    if s == 0 {
        if vendedor == 0 {
            aceitar(this, vaga, 0);
            return;
        }
        if cqd1_humano(vendedor) {
            cqd1_pb(o + 0x2c, 4);
            cqd1_agendar(this, o, 0xe);
            cqd1_desmarcar(this, o);
            return;
        }
        if cqd1_b(cqd1_d(vaga) + 0x1f) != 1 {
            crate::comparacao::ramo("cqd_1 resposta da IA");
            responder(this, vaga, 0); // cqd1: a resposta da IA ao lance
            return;
        }
        if prever(vendedor, alvo, 0, comprador, -1, 1) != 0 {
            recusar_emprestimo(this, vaga, 0, 0);
            return;
        }
        let r = decidir(cqd1_d(0xae1050), cqd1_d(vaga));
        cqd1_pb(cqd1_d(vaga) + 0x2c, 8);
        aceita(this, cqd1_d(vaga));
        if r == 0 {
            return;
        }
        recusa_mudar(this, cqd1_d(vaga), 0);
        cqd1_pb(cqd1_d(vaga) + 0x2c, 0xf);
        cqd1_pb(cqd1_d(vaga) + 0x2e, 3);
        cqd1_desmarcar(this, cqd1_d(vaga));
        cqd1_agendar(this, cqd1_d(vaga), 0xa);
        let o3 = cqd1_d(vaga);
        if fechada(o3) == 0 {
            return;
        }
        let l = cqd1_d(o3);
        if l != 0 {
            cqd1_pd(l + 0xa, cqd1_d(l + 2));
            loop {
                let p = cqd1_proximo(cqd1_d(o3));
                if p == 0 {
                    break;
                }
                if fechada(cqd1_d(cqd1_d(this).wrapping_add(cqd1_d(p).wrapping_mul(4)))) == 0 {
                    return;
                }
            }
        }
        let m = cqd1_d(this + 0x86e).wrapping_add(cqd1_d(o3 + 0xc).wrapping_mul(8));
        if cqd1_b(m) & 1 != 0 {
            cqd1_pb(m, cqd1_b(m) & 0xfe);
        }
        return;
    }
    if matches!(s, 2 | 3 | 4 | 0xf | 0x10 | 0x14) {
        if s == 0xf && cqd1_b(o + 0x2e) < 3 {
            cqd1_pb(o + 0x2e, 3);
            cqd1_agendar(this, o, 0xa);
        } else {
            tirar(this, o);
        }
        return;
    }
    if s == 1 {
        if cqd1_humano(comprador) {
            cqd1_pb(o + 0x2c, 4);
            cqd1_desmarcar(this, o);
            cqd1_agendar(this, o, 0xa);
        } else if cqd1_b(cqd1_d(vaga) + 0x1f) == 1 {
            ia_1a(this, vaga);
        } else {
            ia_1b(this, vaga, 0);
        }
        return;
    }
    if s == 5 || s == 6 {
        if cqd1_humano(comprador) {
            cqd1_pb(o + 0x2c, 9);
            cqd1_agendar(this, o, 0xa);
        } else {
            crate::comparacao::ramo("cqd_1 contrato ao jogador");
            contrato(this, comprador, cqd1_pessoa(cqd1_d(o + 0xc)), o); // cqd1: o contrato ao alvo
        }
        return;
    }
    if cqd1_d(o + 4) != 0 && s == 7 {
        if cqd1_humano(vendedor) {
            cqd1_pb(o + 0x2c, 8);
            cqd1_agendar(this, o, 0xa);
            return;
        }
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d2f0usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let mut l2 = [0u8; 0x10];
        let l = l2.as_mut_ptr() as usize;
        lista_nova(l);
        copiar(l, cqd1_d(o + 4));
        cqd1_pd(l + 0xa, cqd1_d(l + 2));
        loop {
            let p = cqd1_proximo(l);
            if p == 0 {
                break;
            }
            contrato(this, vendedor, cqd1_pessoa(cqd1_d(p)), o);
            let s2 = cqd1_b(o + 0x2c);
            if matches!(s2, 2 | 3 | 4 | 0x10 | 0x14) || (s2 == 0xf && cqd1_b(o + 0x2e) >= 3) {
                break;
            }
        }
        destruir(l);
        return;
    }
    if s == 0xa {
        permissoes(this, o, 0);
        return;
    }
    if s == 0xe {
        concluir(this, o, 0);
        return;
    }
    if s != 0x11 && s != 0x12 {
        return;
    }
    cqd1_pb(o + 0x2c, 0);
    if cqd1_b(o + 0x10) != 0 {
        passar(this, o);
        return;
    }
    let clube = cqd1_clube_do_contrato(cqd1_pessoa(cqd1_d(o + 0xc)), false);
    if tecnico_humano(clube) != 0 {
        noticia_feita(this, o);
        cqd1_agendar(this, o, 0x1c);
        return;
    }
    let v = valor(o, 0, 0, 0, 0, 1);
    let r = prever(vendedor, cqd1_pessoa(cqd1_d(o + 0xc)), v as i64, comprador, -1, 0);
    if r == 0 {
        aceitar(this, vaga, 0);
    } else if (r as i8) > 0 && (r as i8) <= 2 {
        recusar(this, vaga, 0, 0);
    }
}

/// Comparador da 0x8c63a0: só a original (cada ramo muda o status, reagenda, aloca notícias, conclui ou tira a oferta).
unsafe extern "thiscall" fn comparar_processar_oferta_vencida(this: usize, vaga: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1547)(this, vaga) // ponytail: só a original
}

/// { 0x8cfc90 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH: a data e a lista locais, a lista construída pela
/// 0x68d020 e destruída pela 0x68d040): `TRANSFER_MANAGER::check_offer_restrictions(oferta, decide, motivo, sem_elenco)`;
/// 0 libera, 1 ou 2 barram. O comprador é o +8, o alvo o +0xc e o vendedor o clube do contrato dele (fora o tipo 5).
/// - Com +0x30 & 0xf e status fora de 0x13: a data de conclusão (0x8cee70: push &data; push oferta, ret 8) vai ao
///   motivo; motivo diferente de 0 dá 2; hoje antes da data dá 0.
/// - Sem `sem_elenco`, alvo com jogador (+0x61) e tipo 11..15 em +0x11 (0x53b620, cdecl do byte): decide o vendedor com
///   +0x2f = 0 e o vendedor não obrigado a aceitar (0x8cd240, stdcall(oferta, 0, 0)), ou decide o comprador: o veto
///   (0x8cdb80: push motivo; push decide; push alvo; push vendedor; push comprador; push oferta, ret 0x18) dá 2.
/// - O orçamento do comprador (0x5a0590: push comprador; mov ecx, [0xae2a38], ret 4; 0x598800: push 0 ×4; push alvo;
///   mov ecx, finanças, ret 0x14, devolve i64; negativo vira 0) ao degrau seguinte (0x43f4e0: push 4; lea ecx, &dinheiro,
///   ret 4) e o valor da oferta (0x8e7a70: mov ecx, oferta -> thiscall(oferta, 0, 1, 1, 0), ret 0x10). Decide o
///   comprador: valor > orçamento × 1,1: motivo 3; comprador humano dá 1; senão o orçamento mais o +0x1b das ofertas do
///   comprador (0x8cdd80: push 0; push comprador; push pessoa, ret 0xc) pelos jogadores da lista de observação dele
///   (0x8420e0: push &lista; push comprador; mov ecx, [this+0x856], ret 8) passando do valor dá 1, senão 2. Dentro do
///   orçamento, sem vaga no elenco (0x8cd450: push motivo; push sem_elenco; push oferta; push clube, ret 0x10) dá 1.
/// - Decide o vendedor: sem vaga no elenco dele dá 1. Motivo 0. Com troca (+4) ou tipo 11..15: a regra da nação do
///   decide ([this+0x8ac], a de +0xa igual a [decide+0x53]): o virtual +0xc (push motivo; push 0xae2c90; push decide;
///   push oferta; mov ecx, regra) diferente de 0 dá 0; motivo 9 com +0x30 & 0xf = 5 dá 0; senão 2. Sem regra: 0.
unsafe extern "thiscall" fn restricoes_da_oferta(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, usize) -> u16 = core::mem::transmute(0x8cee70usize);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x8cd240usize);
    let veto: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32 = core::mem::transmute(0x8cdb80usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let orcamento: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let degrau: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x43f4e0usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(0x8cd450usize);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    let comprador = cqd1_clube(cqd1_d(o + 8));
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let vendedor = cqd1_clube_do_contrato(alvo, true);
    if cqd1_b(o + 0x30) & 0xf != 0 && cqd1_b(o + 0x2c) != 0x13 {
        let r = conclusao(this, o, pd);
        if !motivo.is_null() {
            *motivo = r;
        }
        if r != 0 {
            return 2;
        }
        let (ha, da) = (cqd1_w(0xae2c92) as u16, cqd1_w(pd + 2) as u16);
        if ha < da || (ha == da && cqd1_w(0xae2c90) < cqd1_w(pd)) {
            return 0;
        }
    }
    if sem_elenco == 0 && cqd1_d(alvo + 0x61) != 0 && tipo(cqd1_b(o + 0x11) as i8) != 0 {
        let mut vetar = false;
        if decide == vendedor && cqd1_b(o + 0x2f) == 0 && deve_aceitar(o, 0, 0) == 0 {
            vetar = true;
        } else if decide == comprador {
            vetar = true;
        }
        if vetar && veto(this, o, comprador, vendedor, alvo, decide, motivo) != 0 {
            return 2;
        }
    }
    let mut b = orcamento(financas(cqd1_d(0xae2a38), comprador), alvo, 0, 0, 0, 0);
    if b < 0 {
        b = 0;
    }
    let mut dinheiro = b as u32;
    let v = degrau(&mut dinheiro as *mut u32 as usize, 4) as i64;
    let t = total(o, 0, 1, 1, 0);
    if decide == comprador {
        crate::comparacao::ramo("cqd_1 orçamento");
        if t as f64 > v as f64 * 1.1 { // cqd1: acima do orçamento
            if !motivo.is_null() {
                *motivo = 3;
            }
            if cqd1_humano(comprador) {
                return 1;
            }
            let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let entradas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8420e0usize);
            let oferta_do_clube: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8cdd80usize);
            let mut l = [0u8; 0x10];
            let pl = l.as_mut_ptr() as usize;
            lista_nova(pl);
            entradas(cqd1_d(this + 0x856), comprador, pl);
            cqd1_pd(pl + 0xa, cqd1_d(pl + 2));
            let mut soma = v;
            loop {
                let p = cqd1_proximo(pl);
                if p == 0 {
                    break;
                }
                let x = oferta_do_clube(this, cqd1_pessoa(cqd1_d(p)), comprador, 0);
                if x != 0 {
                    soma = soma.wrapping_add(cqd1_d(x + 0x1b) as i32 as i64);
                }
            }
            destruir(pl);
            return if soma > t as i64 { 1 } else { 2 };
        }
        if vaga(this, comprador, o, sem_elenco, motivo) == 0 {
            return 1;
        }
    }
    if vendedor == decide && vaga(this, vendedor, o, sem_elenco, motivo) == 0 {
        return 1;
    }
    if !motivo.is_null() {
        *motivo = 0;
    }
    if cqd1_d(o + 4) == 0 && tipo(cqd1_b(o + 0x11) as i8) == 0 {
        return 0;
    }
    for k in 0..0x1c {
        let r = cqd1_d(this + 0x8ac + k * 4);
        if cqd1_d(r + 0xa) == cqd1_d(decide + 0x53) {
            if r == 0 {
                return 0;
            }
            let regra: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32 =
                core::mem::transmute(cqd1_d(cqd1_d(r) + 0xc));
            if regra(r, o, decide, 0xae2c90, motivo) != 0 {
                return 0;
            }
            if *motivo == 9 && cqd1_b(o + 0x30) & 0xf == 5 {
                return 0;
            }
            return 2;
        }
    }
    0
}

/// Comparador da 0x8cfc90: só a original (a vaga no elenco, 0x8cd450, pode liberar o jogador mais barato, e a lista de
/// observação aloca nós).
unsafe extern "thiscall" fn comparar_restricoes_da_oferta(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u8>(1548)(this, o, decide, motivo, sem_elenco) // ponytail: só a original
}

// ---------- 264a leva: a parte dos clubes anteriores na transferência, o percentual da próxima venda pago e a notícia dele (transfer_manager.cpp) ----------

unsafe fn cqb6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube pelo id (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cqb6_clube(id: usize) -> usize {
    cqb6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa pelo id (registro de 0x6e bytes em [0xae23bc]).
unsafe fn cqb6_pessoa(id: usize) -> usize {
    cqb6_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato da pessoa (0x4cf5a0: thiscall(gerência [0xae1050], pessoa), ret 4), guardado em [0xae1080] como a
/// original faz depois de cada chamada.
unsafe fn cqb6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqb6_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato `c` (+4 = id; 0 sem contrato), sem olhar o tipo.
unsafe fn cqb6_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { cqb6_clube(cqb6_d(c + 4)) }
}

/// As finanças do clube (0x5a0590: thiscall([0xae2a38], clube), ret 4).
unsafe fn cqb6_financas(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cqb6_d(0xae2a38), clube)
}

/// O valor (i32 estendido para 64 bits, como o `cdq` da original) entra nas vendas (0x599800) ou sai pelas compras
/// (0x599890) das finanças `fin` (thiscall(fin, baixo, alto), ret 8).
unsafe fn cqb6_dinheiro(rotina: usize, fin: usize, v: i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
    f(fin, v as u32, (v >> 31) as u32);
}

/// A notícia local: construir (0x77bbe0: thiscall(notícia, tipo, 0), ret 8), gravar um dado (0x785790:
/// thiscall(notícia, k, v), ret 8) e entregar ao técnico do clube (0x7861e0: thiscall([0xb7431c], notícia, clube),
/// ret 8).
unsafe fn cqb6_construir(n: usize, tipo: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    f(n, tipo, 0);
}

unsafe fn cqb6_gravar(n: usize, k: i8, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

unsafe fn cqb6_entregar(n: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    f(cqb6_d(0xb7431c), n, clube);
}

/// O ensaio dos comparadores: a cópia roda sem as notícias e sem os totais do clube (0x448ad0), que alocam ou entregam
/// e não podem rodar duas vezes; CQB6_ENTREGAS conta as entregas que a original faria (elas podem mexer em [0xae1080]
/// pela resposta automática do humano de férias).
static mut CQB6_ENSAIO: bool = false;
static mut CQB6_ENTREGAS: u32 = 0;

/// { 0x8e4f20 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::pay_former_club_fee_cuts(oferta)`, pela transferência efetivada com valor (+0x1b) de jogador sem
/// +0x69 (0x8c9390). Jogador (+0xc da oferta) com mais de 22 anos (+0x18): a notícia 0xfc3 (local; 0 = id, 1..3 = os
/// +0x33 de +4/+8/+0xc, 4 = o comprador +8, 7 = o clube do contrato ou −1); a soma (16 bits) dos pesos (byte +0x12)
/// das cinco vagas de clube anterior do registro de 79 bytes ([0xae2d0c], passo 5, clube em +0xe, −1 = vaga vazia;
/// soma 0 vira 1). Cada vaga com clube: ((valor / 20) / soma) × peso (peso abaixo de 1 vale 1) entra nas vendas das
/// finanças do COMPRADOR (0x5a0590 com +8, 0x599800; é o que a original faz); acima de 11 × reputação (+0x80) − 10000
/// do clube da vaga, a notícia com 5 = o valor e 6 = o id do clube vai ao técnico dele.
unsafe extern "thiscall" fn parte_dos_clubes_anteriores(_this: usize, o: usize) {
    let p = cqb6_pessoa(cqb6_d(o + 0xc));
    if (cqb6_b(p + 0x18) as i8) < 0x17 { // cqb6: só acima de 22 anos
        return;
    }
    crate::comparacao::ramo("cqb_6 parte dos clubes anteriores");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    if !CQB6_ENSAIO {
        cqb6_construir(n, 0xfc3);
    }
    let r = cqb6_d(0xae2d0c).wrapping_add(cqb6_d(p).wrapping_mul(0x4f));
    let mut soma: i16 = 0;
    for k in 0..5 {
        if cqb6_d(r + 0xe + 5 * k) as i32 != -1 {
            soma = soma.wrapping_add(cqb6_b(r + 0x12 + 5 * k) as i8 as i16);
        }
    }
    let vinte = (cqb6_d(o + 0x1b) as i32) / 20;
    let divisor = if soma == 0 { 1 } else { soma as i32 };
    if !CQB6_ENSAIO {
        cqb6_gravar(n, 0, cqb6_d(p) as u32);
        cqb6_gravar(n, 1, cqb6_d(cqb6_d(p + 4) + 0x33) as u32);
        cqb6_gravar(n, 2, cqb6_d(cqb6_d(p + 8) + 0x33) as u32);
        cqb6_gravar(n, 3, cqb6_d(cqb6_d(p + 0xc) + 0x33) as u32);
        cqb6_gravar(n, 4, cqb6_d(o + 8) as u32);
    }
    let c = cqb6_contrato(p);
    let v7 = if c == 0 || cqb6_b(c + 0x35) & 0x3f == 5 || cqb6_clube_do_contrato(c) == 0 {
        u32::MAX
    } else {
        let c2 = cqb6_contrato(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
        if c2 != 0 && cqb6_b(c2 + 0x35) & 0x3f != 5 { cqb6_d(cqb6_clube_do_contrato(c2)) as u32 } else { u32::MAX }
    };
    if !CQB6_ENSAIO {
        cqb6_gravar(n, 7, v7);
    }
    for k in 0..5 {
        let r = cqb6_d(0xae2d0c).wrapping_add(cqb6_d(p).wrapping_mul(0x4f)) + 5 * k;
        let i = cqb6_d(r + 0xe);
        if i as i32 == -1 {
            continue;
        }
        let peso = cqb6_b(r + 0x12) as i8;
        let peso = if peso < 1 { 1 } else { peso as i32 };
        let clube = cqb6_clube(i);
        let valor = vinte.wrapping_div(divisor).wrapping_mul(peso); // cqb6: a parte da vaga
        let fin = cqb6_financas(cqb6_clube(cqb6_d(o + 8)));
        crate::comparacao::ramo("cqb_6 parte paga");
        cqb6_dinheiro(0x599800, fin, valor); // cqb6: entra nas vendas do comprador
        if valor > (cqb6_w(clube + 0x80) as i32).wrapping_mul(11).wrapping_sub(10000) {
            CQB6_ENTREGAS += 1;
            if !CQB6_ENSAIO {
                cqb6_gravar(n, 5, valor as u32);
                cqb6_gravar(n, 6, cqb6_d(clube) as u32);
                cqb6_entregar(n, clube);
            }
        }
    }
}

/// Comparador da 0x8e4f20: a cópia em ensaio (sem a notícia) e a original a partir do mesmo estado, lado a lado nas
/// finanças do comprador (0x167 bytes) e em [0xae1080]; com entrega da notícia, só as finanças.
unsafe extern "thiscall" fn comparar_parte_dos_clubes_anteriores(this: usize, o: usize) {
    let original = orig::<unsafe extern "thiscall" fn(usize, usize)>(1473);
    let fin = cqb6_financas(cqb6_clube(cqb6_d(o + 8)));
    CQB6_ENTREGAS = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(fin, 0x167), (0xae1080, 4)], || original(this, o), || {
        CQB6_ENSAIO = true;
        crate::aleatorio::sem_contar(|| parte_dos_clubes_anteriores(this, o));
        CQB6_ENSAIO = false;
    });
    let entregas = CQB6_ENTREGAS;
    let igual = igual || (entregas != 0 && mo.0[0] == mr.0[0]);
    PLACAR.registrar(igual, || format!("parte_dos_clubes_anteriores(oferta {:#x}) entregas={} original={:x?} rust={:x?}", o, entregas, mo, mr));
}

/// { 0x8cb0f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::pay_next_sale_percentage(pessoa, oferta)`,
/// pela transferência efetivada com a marca 8 da pessoa (0x8c91e0). Com contrato (e clube do contrato), procura pela
/// busca binária (0x946464, entradas de 0xd bytes em [this+0x880], conta [this+0x888], comparação 0x8e54d0) a
/// cláusula com +1 = id do clube do contrato e +9 = id da pessoa. Achada: a base é o valor da oferta (+0x1b) mais o
/// valor (+0x52) de cada jogador da troca (+4, LLIST de ids, cursor recomeçado) quando a pessoa é a da oferta (+0xc);
/// senão o valor da pessoa para o clube do contrato (0x4d0c60 com 0, −1, 0). A parte = percentual (byte +0, com sinal)
/// × base / 100; de 1000 para cima: a notícia (0x8db3a0, com 1 se era a da oferta), os totais de transferência do
/// clube (0x448ad0 em [0xad9dac]: a parte, o clube do contrato, o clube da cláusula +5), a parte entra nas vendas do
/// clube da cláusula e sai pelas compras do clube do contrato. A cláusula fica (quem a tira é a 0x8cb4f0).
unsafe extern "thiscall" fn percentual_da_proxima_venda_pago(this: usize, p: usize, o: usize) -> u32 {
    let c = cqb6_contrato(p);
    if c == 0 || cqb6_clube_do_contrato(c) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, cqb6_d(p));
    let c2 = cqb6_contrato(p);
    // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
    let id_clube = if c2 != 0 { cqb6_d(cqb6_clube_do_contrato(c2)) } else { usize::MAX };
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, id_clube);
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), cqb6_d(this + 0x880), cqb6_d(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return 0;
    }
    let (base, da_oferta) = if cqb6_d(p) == cqb6_d(o + 0xc) {
        let mut v = cqb6_d(o + 0x1b) as i32;
        let l = cqb6_d(o + 4);
        if l != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            core::ptr::write_unaligned((l + 0xa) as *mut usize, cqb6_d(l + 2));
            let mut i = proximo(cqb6_d(o + 4));
            while i != 0 {
                v = v.wrapping_add(cqb6_d(cqb6_pessoa(cqb6_d(i)) + 0x52) as i32);
                i = proximo(cqb6_d(o + 4));
            }
        }
        (v, 1u32)
    } else {
        let clube = cqb6_clube_do_contrato(cqb6_contrato(p));
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
        (valor(cqb6_d(0xae1050), p, clube, 0, -1, 0), 0)
    };
    let parte = (cqb6_b(e) as i8 as i32).wrapping_mul(base) / 100;
    if parte < 1000 {
        return 0;
    }
    crate::comparacao::ramo("cqb_6 percentual pago");
    if !CQB6_ENSAIO {
        let noticia: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32) = core::mem::transmute(0x8db3a0usize);
        noticia(this, e, parte, o, da_oferta);
    }
    let da_clausula = cqb6_clube(cqb6_d(e + 5));
    let fin_recebe = cqb6_financas(da_clausula);
    let fin_paga = cqb6_financas(cqb6_clube_do_contrato(cqb6_contrato(p)));
    let do_contrato = cqb6_clube_do_contrato(cqb6_contrato(p));
    if !CQB6_ENSAIO {
        let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
        totais(cqb6_d(0xad9dac), parte, do_contrato, cqb6_clube(cqb6_d(e + 5)));
    }
    cqb6_dinheiro(0x599800, fin_recebe, parte);
    cqb6_dinheiro(0x599890, fin_paga, parte);
    1
}

/// O que a 0x8cb0f0 vai mexer, para as regiões do comparador: as finanças do clube da cláusula e do clube do
/// contrato, se a cláusula existe (sem gravar nada além de [0xae1080], que a original regrava).
unsafe fn cqb6_financas_da_clausula(this: usize, p: usize) -> Option<(usize, usize)> {
    let c = cqb6_contrato(p);
    if c == 0 || cqb6_clube_do_contrato(c) == 0 {
        return None;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, cqb6_d(p));
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, cqb6_d(cqb6_clube_do_contrato(c)));
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), cqb6_d(this + 0x880), cqb6_d(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return None;
    }
    Some((cqb6_financas(cqb6_clube(cqb6_d(e + 5))), cqb6_financas(cqb6_clube_do_contrato(c))))
}

/// Comparador da 0x8cb0f0: a cópia em ensaio (sem a notícia e sem os totais do clube) e a original a partir do mesmo
/// estado, lado a lado nas finanças dos dois clubes, em [0xae1080] e no sorteio (a 0x4d0c60 pode sortear); pago (a
/// notícia e os totais da original podem regravar [0xae1080]), só as finanças e a devolução.
unsafe extern "thiscall" fn comparar_percentual_da_proxima_venda_pago(this: usize, p: usize, o: usize) -> u32 {
    let original = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1474);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    if let Some((a, b)) = cqb6_financas_da_clausula(this, p) {
        regioes.push((a, 0x167));
        regioes.push((b, 0x167));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || original(this, p, o), || {
        CQB6_ENSAIO = true;
        let n = crate::aleatorio::sem_contar(|| percentual_da_proxima_venda_pago(this, p, o));
        CQB6_ENSAIO = false;
        n
    });
    let igual = r == n && (igual || (r != 0 && mo.0[3..] == mr.0[3..]));
    PLACAR.registrar(igual, || format!("percentual_da_proxima_venda_pago(pessoa {:#x}, oferta {:#x}) original={} {:x?} rust={} {:x?}", p, o, r, mo, n, mr));
    r
}

/// { 0x8db3a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::post_sale_percentage_news(cláusula, parte, oferta, da_oferta)`, pelo percentual pago (0x8cb0f0).
/// A notícia 0xfa7 (local): 0..3 a pessoa da cláusula (+9: id e os +0x33 de +4/+8/+0xc), 4 = o percentual (byte +0),
/// 5 = o clube da cláusula (+5), 6 = o clube vendedor (+1), 7 = a parte, 8 = `da_oferta`, 9 = o comprador (+8 da
/// oferta) com `da_oferta`; senão o id do clube do contrato (0x4cf5a0; sem contrato ou sem clube: acaba sem entregar).
/// Vai aos técnicos do clube da cláusula e do vendedor (0x7861e0).
unsafe extern "thiscall" fn noticia_do_percentual_da_venda(_this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    crate::comparacao::ramo("cqb_6 noticia do percentual");
    let p = cqb6_pessoa(cqb6_d(e + 9));
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    cqb6_construir(n, 0xfa7);
    cqb6_gravar(n, 0, cqb6_d(p) as u32);
    cqb6_gravar(n, 1, cqb6_d(cqb6_d(p + 4) + 0x33) as u32);
    cqb6_gravar(n, 2, cqb6_d(cqb6_d(p + 8) + 0x33) as u32);
    cqb6_gravar(n, 3, cqb6_d(cqb6_d(p + 0xc) + 0x33) as u32);
    cqb6_gravar(n, 4, cqb6_b(e) as i8 as i32 as u32);
    cqb6_gravar(n, 5, cqb6_d(e + 5) as u32);
    cqb6_gravar(n, 6, cqb6_d(e + 1) as u32);
    cqb6_gravar(n, 7, parte as u32);
    cqb6_gravar(n, 8, da_oferta);
    let v9 = if da_oferta != 0 {
        cqb6_d(o + 8)
    } else {
        let c = cqb6_contrato(p);
        if c == 0 || cqb6_clube_do_contrato(c) == 0 {
            return;
        }
        let c2 = cqb6_contrato(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
        if c2 != 0 { cqb6_d(cqb6_clube_do_contrato(c2)) } else { usize::MAX }
    };
    cqb6_gravar(n, 9, v9 as u32);
    cqb6_entregar(n, cqb6_clube(cqb6_d(e + 5)));
    cqb6_entregar(n, cqb6_clube(cqb6_d(e + 1)));
}

/// Comparador da 0x8db3a0: só a original (a notícia é entregue e não pode sair duas vezes; a cópia leva o ramo no
/// modo Rust). ponytail: sem plano a conferir além do que a original entrega.
unsafe extern "thiscall" fn comparar_noticia_do_percentual_da_venda(this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(1475)(this, e, parte, o, da_oferta)
}

// ---------- 264a leva: a permissão de trabalho concedida, o estatuto pelo motivo da recusa e a vaga da negociação (transfer_manager.cpp) ----------

unsafe fn cqb7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb7_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato), sem olhar o tipo.
unsafe fn cqb7_clube_do_contrato(c: usize) -> usize {
    if c == 0 { 0 } else { cqb7_d(0xae23b4).wrapping_add(cqb7_d(c + 4).wrapping_mul(0x245)) }
}

/// A data `a` vem depois da data em `d` (0x456740: thiscall(a, d.dword0, d.dword1), ret 8; a data vai por valor).
unsafe fn cqb7_depois(a: usize, d: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    f(a, cqb7_d(d) as u32, cqb7_d(d + 4) as u32) != 0
}

/// O nome (texto C em [pessoa+4]) contém `agulha` (0x945a60, strstr: cdecl(texto, agulha)) e o clube da oferta (+4 =
/// id; nome em +4 do clube) é o de 0xa8eb48 ("Falkirk"; a comparação de bytes que a original faz em linha).
unsafe fn cqb7_excecao(p: usize, agulha: usize, oferta: usize) -> bool {
    let nome = cqb7_d(p + 4);
    if nome == 0 {
        return false;
    }
    let strstr: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945a60usize);
    if strstr(nome, agulha) == 0 || oferta == 0 {
        return false;
    }
    let mut a = cqb7_d(0xae23b4).wrapping_add(cqb7_d(oferta + 4).wrapping_mul(0x245)) + 4;
    let mut b = 0xa8eb48usize;
    loop {
        let (x, y) = (cqb7_b(a), cqb7_b(b));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        a += 1;
        b += 1;
    }
}

/// As marcas da pessoa na gerência (8 bytes em [this+0x86e] + id × 8): bit 0x20 em +0 (tem registro de permissão),
/// +3 o histórico de 16 jogos da seleção (u16, bit 0 o mais novo) e +5 a conta dos jogos da seleção já vistos (u16).
unsafe fn cqb7_marcas(this: usize, p: usize) -> usize {
    cqb7_d(this + 0x86e).wrapping_add(cqb7_d(p).wrapping_mul(8))
}

/// { 0x8c7950 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::is_work_permit_granted(pessoa, nação,
/// oferta, registro, sorteio)`, pela permissão na lista de observação (0x84b180, sem oferta, registro nem sorteio),
/// pela oferta de contrato aceita (0x8c38b0, com a oferta) e pelas permissões da oferta (0x8c7690). Contrato da pessoa
/// (0x4cf5a0, sem gravar [0xae1080]): de tipo 2 é o "empréstimo", outro é o "contrato". Com o registro de permissão
/// (o dado, ou achado pela busca binária 0x946464 em [this+0x93a], conta [this+0x93e], 14 bytes, quando a marca 0x20
/// existe): sem oferta, sai se jogos (+4) > 3 × recusas (+6) ou recusas < 4; com oferta, a mesma regra se o empréstimo
/// acaba depois da oferta (+0x2d) ou o clube do contrato não é o da oferta (+4), e sai se o empréstimo não acaba depois.
/// Não jogador (+0x69 sem ficha +0x61): reputação (+0xc) acima de 160. Em 2001, os dois casos do Collin e do Devon no
/// Falkirk. Potencial (+0xd da ficha) acima de 8500 (com sorteio: 8000 + sorteio(1000)); salário da oferta (+0xc) acima
/// de 34999; menos de 24 anos com habilidade (+7) acima de 190: sai. Menos de 5 jogos pela seleção (+0x22): não sai;
/// mais de 20 com reputação da nação (+0x8e) acima de 3000 e menos de 32 anos: sai. O histórico anda pelos jogos novos
/// da seleção ([[0xb74318]+4] + id da nação × 0x6e + 2; cada um empurra um bit zero) e a conta fica. A janela: 16
/// jogos, ou os do período de dois anos (0x58d710, até 16) a partir de 2003; com mais de 11 jogos marcados (sorteio:
/// 11 − sorteio(3) + sorteio(3)) ou mais de 70% (70 − sorteio(10) + sorteio(10)): sai.
unsafe extern "thiscall" fn permissao_concedida(this: usize, p: usize, _nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cqb7_d(0xae1050), p);
    let (emprestimo, contrato) = if c != 0 && cqb7_b(c + 0x35) & 0x3f != 2 { (0, c) } else { (c, 0) };
    let mut reg = registro;
    let tem_registro = if reg != 0 {
        true
    } else if (p == 0 || (cqb7_d(p) as i32) < cqb7_d(0xae2364) as i32) && cqb7_b(cqb7_marcas(this, p)) & 0x20 != 0 {
        let chave = cqb7_d(p) as u32;
        let busca: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        reg = busca(&chave, cqb7_d(this + 0x93a), cqb7_d(this + 0x93e), 0xe, 0x8e55c0);
        reg != 0
    } else {
        false
    };
    if tem_registro {
        let jogos = core::ptr::read_unaligned((reg + 4) as *const u16) as u32;
        let recusas = core::ptr::read_unaligned((reg + 6) as *const u16) as u32;
        let pela_conta = jogos > recusas * 3 || recusas < 4;
        if oferta == 0 {
            if pela_conta {
                return 1;
            }
        } else {
            let fora = (emprestimo != 0 && cqb7_depois(oferta + 0x2d, emprestimo + 0x2d))
                || (contrato != 0 && cqb7_d(oferta + 4) != cqb7_d(contrato + 4));
            if fora && pela_conta {
                return 1;
            }
            if emprestimo != 0 && !cqb7_depois(oferta + 0x2d, emprestimo + 0x2d) {
                return 1;
            }
        }
    }
    let np = cqb7_d(p + 0x69);
    if np != 0 && cqb7_d(p + 0x61) == 0 {
        return (cqb7_w(np + 0xc) > 0xa0) as u32;
    }
    if cqb7_w(0xae2c92) == 0x7d1 && (cqb7_excecao(p, 0x9b5ac0, oferta) || cqb7_w(0xae2c92) == 0x7d1 && cqb7_excecao(p, 0x9b5ab0, oferta)) {
        crate::comparacao::ramo("cqb_7 excecao de 2001");
        return 1;
    }
    let ficha = cqb7_d(p + 0x61);
    if sorteio != 0 {
        if ficha != 0 && cqb7_w(ficha + 0xd) as i32 > cqb7_sorteio(1000) + 8000 {
            return 1;
        }
    } else if ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o potencial");
        if cqb7_w(ficha + 0xd) > 0x2134 { // cqb7: o potencial acima de 8500
            return 1;
        }
    }
    if oferta != 0 && cqb7_d(oferta + 0xc) as i32 > 0x88b7 {
        return 1;
    }
    let idade = cqb7_b(p + 0x18) as i8;
    if idade < 0x18 && ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o jovem");
        if cqb7_w(ficha + 7) > 0xbe { // cqb7: o jovem acima de 190
            return 1;
        }
    }
    let jogos = cqb7_b(p + 0x22);
    if jogos < 5 {
        return 0;
    }
    if jogos > 0x14 {
        let nacao = cqb7_d(p + 0x1a);
        if nacao != 0 && cqb7_w(nacao + 0x8e) > 0xbb8 && idade < 0x20 {
            return 1;
        }
    }
    // o histórico pelos jogos novos da seleção
    let selecao = cqb7_w(cqb7_d(cqb7_d(0xb74318) + 4).wrapping_add(cqb7_d(cqb7_d(p + 0x1a)).wrapping_mul(0x6e)) + 2);
    let novos = selecao.wrapping_sub(cqb7_w(cqb7_marcas(this, p) + 5));
    if novos > 0 {
        for _ in 0..novos {
            let h = cqb7_marcas(this, p) + 3;
            core::ptr::write_unaligned(h as *mut u16, core::ptr::read_unaligned(h as *const u16) << 1);
            *(h as *mut u8) &= 0xfe;
        }
    }
    core::ptr::write_unaligned((cqb7_marcas(this, p) + 5) as *mut i16, selecao);
    let ano = cqb7_w(0xae2c92) as u16;
    let janela: i16 = if (ano as i32) - 2 < 0x7d1 {
        0x10
    } else {
        let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
        let ate = mes(0xae2c90);
        let de = mes(0xae2c90);
        let periodo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16 = core::mem::transmute(0x58d710usize);
        let n = periodo(cqb7_d(cqb7_d(0xb74318) + 0x1c), cqb7_d(p + 0x1a), de, (ano as u32).wrapping_sub(2), ate, ano as u32) as i16;
        if n > 0x10 { 0x10 } else { n }
    };
    let mut marcados: i16 = 0;
    let mut k: i16 = 0;
    while k < janela && k < 0x10 {
        let mascara = 1u16 << k; // ftol(pow(2.0, k)) (0x945ff0 e 0x945640): exato de 0 a 15
        if core::ptr::read_unaligned((cqb7_marcas(this, p) + 3) as *const u16) & mascara != 0 {
            marcados += 1;
        }
        k += 1;
    }
    if sorteio != 0 {
        let a = cqb7_sorteio(3);
        let limite = 0xb - a + cqb7_sorteio(3);
        if marcados as i32 > limite {
            return 1;
        }
        if janela == 0 {
            return 0;
        }
        let b = cqb7_sorteio(10);
        let limite = 0x46 - b + cqb7_sorteio(10);
        return ((marcados as i32 * 100) / janela as i32 > limite) as u32;
    }
    crate::comparacao::ramo("cqb_7 jogos pela selecao");
    if marcados > 0xb { // cqb7: mais de 11 jogos marcados
        return 1;
    }
    if janela == 0 {
        return 0;
    }
    ((marcados as i32 * 100) / janela as i32 > 0x46) as u32
}

/// Comparador da 0x8c7950: lado a lado nas marcas da pessoa (8 bytes), no sorteio e na devolução (a cópia em
/// sem_contar).
unsafe extern "thiscall" fn comparar_permissao_concedida(this: usize, p: usize, nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(1476);
    let mut regioes = vec![crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1]];
    if p != 0 && (cqb7_d(p) as i32) < cqb7_d(0xae2364) as i32 {
        regioes.push((cqb7_marcas(this, p), 8));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, p, nacao, oferta, registro, sorteio),
        || crate::aleatorio::sem_contar(|| permissao_concedida(this, p, nacao, oferta, registro, sorteio)));
    PLACAR.registrar(igual && r == n, || format!("permissao_concedida(pessoa {:#x}, oferta {:#x}, registro {:#x}, sorteio {}) original={} {:x?} rust={} {:x?}", p, oferta, registro, sorteio, r, mo, n, mr));
    r
}

/// { 0x8c5800 } (thiscall, ret 8, transfer_manager.cpp; o this não é usado): `transfer_manager_get_squad_status_for_reason
/// (pessoa, motivo)`, pela tela da oferta. O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) dá o clube; o
/// contrato com ele (0x4cf5f0); sem ele, 0. O motivo (16 bits com sinal) 0x200 dá 1, 0x400 dá 2, 0x2000 dá 5; os
/// outros, o estatuto do contrato (+0x4f >> 4). Devolve no al.
unsafe extern "thiscall" fn estatuto_pelo_motivo(_this: usize, p: usize, motivo: u32) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cqb7_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let cc = com_clube(cqb7_d(0xae1050), p, cqb7_clube_do_contrato(c));
    if cc == 0 {
        return 0;
    }
    match motivo as i16 as i32 {
        0x200 => 1,
        0x400 => 2,
        0x2000 => 5,
        _ => (cqb7_b(cc + 0x4f) >> 4) as u32,
    }
}

/// Comparador da 0x8c5800: lado a lado em [0xae1080] e no al devolvido.
unsafe extern "thiscall" fn comparar_estatuto_pelo_motivo(this: usize, p: usize, motivo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1477);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, p, motivo), || estatuto_pelo_motivo(this, p, motivo));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("estatuto_pelo_motivo(pessoa {:#x}, motivo {:#x}) original={} {:x?} rust={} {:x?}", p, motivo, r as u8, mo, n as u8, mr));
    r
}

/// { 0x8c6200 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::get_negotiation_slot(i)`, pela resposta
/// automática às notícias (0x782f70) e pelas telas: o endereço da vaga `i` do vetor de negociações ([this+0xc], conta
/// [this+0x10], com sinal); fora da conta, o começo do vetor.
unsafe extern "thiscall" fn vaga_da_negociacao(this: usize, i: i32) -> usize {
    if i < cqb7_d(this + 0x10) as i32 {
        cqb7_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(4))
    } else {
        cqb7_d(this + 0xc)
    }
}

/// Comparador da 0x8c6200: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_vaga_da_negociacao(this: usize, i: i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(1478)(this, i);
    let n = vaga_da_negociacao(this, i);
    PLACAR.registrar(r == n, || format!("vaga_da_negociacao({}) original={:#x} rust={:#x}", i, r, n));
    r
}

// ---------- 263a leva: a vaga no elenco para a oferta (o limite de 50 jogadores e das vagas da comissão, com a liberação do mais barato), a oferta de empréstimo recusada pelo clube e a cláusula de percentual da próxima venda achada (transfer_manager.cpp) ----------

unsafe fn cqa5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x8bfa30 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_loan_offer(vaga, marcar, motivo)`,
/// pelo vendedor do computador que recusa todo lance de empréstimo (0x8c8090, oferta com +0x1f = 1) e pela oferta
/// agendada (0x8c63a0). Com `marcar`, +0x2d = 1; estado 2 (recusada) já de cara; o contrato da pessoa vai para
/// [0xae1080] e dá o clube do contrato; os lances do comprador pelo jogador e os do clube do contrato pelos da troca
/// saem (0x84aeb0); a notícia (0x8d5170); a oferta volta em 10 dias (0x8c6220) para sair; sem rival em aberto, a marca
/// de oferta da pessoa sai (+0x86e); as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a 0x4ccb30 de cada jogador
/// da troca, cuja marca +4 zera), com +0x10 = 0 entre elas; por fim a situação na lista de observação do comprador
/// (0x849ce0; achada, 0x843e10 com evento 1 e tipo 0). É a recusa da transferência (0x8bf680) sem o desassossego, sem
/// a transferência futura, sem fechar as ofertas de contrato ligadas e sem a parte dos jogadores da troca no fim: os
/// ajudantes `cpx1_*` (com o ensaio CPX1_ENSAIO) servem igual.
unsafe extern "thiscall" fn recusar_oferta_de_emprestimo(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    if marcar != 0 {
        *((o + 0x2d) as *mut u8) = 1;
    }
    *((o + 0x2c) as *mut u8) = 2;
    crate::comparacao::ramo("cqa_5 emprestimo recusado");
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    cpx1_desmarcar_lances(this, o, vendedor);
    cpx1_noticia(this, o, motivo);
    cpx1_agendar(this, o, 0xa);
    cpx1_tirar_marca(this, o);
    // vendedor 0: sem a parte dos jogadores da troca na lista do comprador, que esta rotina não tem
    cpx1_ofertas_de_contrato(this, o, 0, 0);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let comprador = cpx1_clube(cpx1_d(o + 8));
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    if entrada(cpx1_listas(this), comprador, p, (cpx1_b(o + 0x11) != 0xb) as u32, 0) != 0 {
        cpx1_situacao_na_lista(this, p, comprador, 1, 0);
    }
}

/// Comparador da 0x8bfa30: o da recusa e da retirada (`cpx1_comparar`: a cópia em ensaio e a original do mesmo estado;
/// iguais o estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa).
unsafe extern "thiscall" fn comparar_recusar_oferta_de_emprestimo(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(1431);
    cpx1_comparar("recusar_oferta_de_emprestimo", this, vaga, || o(this, vaga, marcar, motivo), || recusar_oferta_de_emprestimo(this, vaga, marcar, motivo))
}

/// O ensaio do comparador da 0x8cd450: a cópia roda sem liberar o jogador escolhido (0x86d800 e 0x8c1af0 mexem no
/// resto do jogo) e guarda quem seria em CQA5_LIBERADO.
static mut CQA5_ENSAIO: bool = false;
static mut CQA5_LIBERADO: usize = 0;

/// Os tipos 0x53b620 (11 a 15: jogador) e 0x53b610 (não 11), cdecl com o byte.
unsafe fn cqa5_jogador(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    f(t as i8) != 0
}

unsafe fn cqa5_nao_11(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b610usize);
    f(t as i8) != 0
}

/// As quatro contas de vagas da comissão pelo tipo da pessoa: [0] treinadores (8 e 0xf, +0x19f), [1] olheiros (9,
/// +0x1b3), [2] fisioterapeutas (0xa, +0x1cf), [3] auxiliar (6 e 0xd, +0xd3).
unsafe fn cqa5_comissao(t: u8) -> Option<usize> {
    match t {
        8 | 0xf => Some(0),
        9 => Some(1),
        0xa => Some(2),
        6 | 0xd => Some(3),
        _ => None,
    }
}

/// O id da pessoa no vetor (dword com sinal) abaixo de [0xae2364]: a pessoa ainda existe.
unsafe fn cqa5_viva(p: usize) -> bool {
    p != 0 && (cqa5_d(p) as i32) < cqa5_d(0xae2364) as i32
}

/// A falha com o motivo: (clube ≠ comprador) + `base` na palavra do motivo, se houver; 0.
unsafe fn cqa5_falha(clube: usize, o: usize, motivo: *mut u16, base: u16) -> u32 {
    if !motivo.is_null() {
        *motivo = (cqa5_d(clube) != cqa5_d(o + 8)) as u16 + base;
    }
    0
}

/// A pessoa está na fila da transferência livre ([0xde1f54] + 0x92c, LLIST de ids; o cursor +0xa volta a +2 e a busca
/// para no achado, como na original).
unsafe fn cqa5_na_fila_livre(p: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cqa5_d(0xde1f54) + 0x92c;
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqa5_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if cqa5_d(p) == cqa5_d(e) {
            return true;
        }
        e = proximo(l);
    }
    false
}

/// { 0x8cd450 } (thiscall, ret 0x10, transfer_manager.cpp; o mapa a dá como stdcall, mas os dois pontos de chamada na
/// 0x8cfc90 põem a gerência em ecx e ela o usa para a 0x8c1780 e a 0x8c1af0): `TRANSFER_MANAGER::check_squad_limits
/// (clube, oferta, contar_fila, motivo)`, das restrições da oferta (0x8cfc90), pelo comprador e pelo vendedor. Clube
/// nulo: a caixa da linha 0x2c3b e 0. O parceiro do clube (0x540a50 com 0 e 1). Pelo comprador (+8 da oferta = id do
/// clube): a pessoa da oferta já é dele (contrato de tipo 5, de novo pela 0x4cf5a0, com o clube; ou a ficha de
/// empréstimo 0x4cf6d0 com +4 = o clube) conta −1; jogador pelo tipo +0x11 da oferta conta +1; o tipo da comissão
/// pelo mesmo byte marca 1; cada jogador da troca (+4, LLIST de ids) que sai desconta (jogador −1, comissão −1 pelo
/// +0x3d da pessoa). Pelo vendedor: sem troca, 1; o jogador vendido conta −1 e o tipo da comissão −1 (pelo +0x3d); cada
/// nó da troca que chega soma pelo byte +6. Conta positiva: soma os 50 do elenco (+0xd7, ponteiros com id vivo) do clube
/// e do parceiro; acima de 50, menos a fila livre do clube (0x8c1780) se `contar_fila`; ainda acima de 50: fora do
/// comprador do computador sem troca, o motivo (clube ≠ comprador) + 4 e 0; senão o mais barato do elenco ((+0x3d do
/// contrato + 100) × valor +0x52, com +0x3d ≤ 0, sem ficha, sem contrato de tipo 5 em [0xae1080], sem ofertas 0x8c5cf0,
/// trocas 0x8c5e40 ou transferência futura 0x8cca10, fora da fila livre e de tipo 0xb, 0xf ou 0xd) sai de graça
/// (0x86d800 em [0xdd7f00], 0x8c1af0 com −1); nenhum, o motivo + 4 e 0. Depois as vagas da comissão marcadas: 5
/// treinadores (+0x19f), 3 fisioterapeutas (+0x1cf), 7 olheiros (+0x1b3) — os do clube contam por ponteiro, os do
/// parceiro só com id vivo — e o auxiliar (+0xd3 do clube ou do parceiro ocupado); passou do limite, o motivo + 0x22 e 0.
/// Devolve 1.
unsafe extern "thiscall" fn vaga_no_elenco_para_a_oferta(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    if clube == 0 {
        aviso_em(0xa8d514, 0x2c3b);
        return 0;
    }
    let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let parceiro = parceiro_de(clube, core::ptr::null_mut(), 1);
    let mut n: i8 = 0;
    let mut com = [0i8; 4];
    if cqa5_d(clube) == cqa5_d(o + 8) {
        let p = cpx1_pessoa(cqa5_d(o + 0xc));
        let c = cpx1_contrato(p);
        let mut proprio = false;
        if c != 0 && cqa5_b(c + 0x35) & 0x3f == 5 {
            proprio = cpx1_clube_do_contrato(p) == clube;
        }
        if !proprio && ficha(cqa5_d(0xae1050), p) != 0 && cqa5_d(ficha(cqa5_d(0xae1050), p) + 4) == cqa5_d(clube) {
            proprio = true;
        }
        if proprio {
            n = -1;
        }
        let t = cqa5_b(o + 0x11);
        if cqa5_jogador(t) {
            n = n.wrapping_add(1); // cqa5: o jogador da oferta conta
        }
        if cqa5_nao_11(t) {
            if let Some(k) = cqa5_comissao(t) {
                com[k] = 1;
            }
        }
        let l = cqa5_d(o + 4);
        if l != 0 {
            for e in cpx1_itens(l) {
                let t = cqa5_b(cpx1_pessoa(cqa5_d(e)) + 0x3d);
                if cqa5_jogador(t) {
                    n = n.wrapping_sub(1);
                }
                if cqa5_nao_11(t) {
                    if let Some(k) = cqa5_comissao(t) {
                        com[k] = com[k].wrapping_sub(1);
                    }
                }
            }
        }
    } else {
        if cqa5_d(o + 4) == 0 {
            crate::comparacao::ramo("cqa_5 vendedor sem troca");
            return 1; // cqa5: o vendedor sem troca tem vaga
        }
        let t = cqa5_b(cpx1_pessoa(cqa5_d(o + 0xc)) + 0x3d);
        if cqa5_jogador(t) {
            n = -1;
        }
        if cqa5_nao_11(t) {
            if let Some(k) = cqa5_comissao(t) {
                com[k] = -1;
            }
        }
        for e in cpx1_itens(cqa5_d(o + 4)) {
            let t = cqa5_b(e + 6);
            if cqa5_jogador(t) {
                n = n.wrapping_add(1);
            }
            if cqa5_nao_11(t) {
                if let Some(k) = cqa5_comissao(t) {
                    com[k] = com[k].wrapping_add(1);
                }
            }
        }
    }
    if n > 0 {
        let contar = |base: usize, n: &mut i8| {
            for k in 0..0x32 {
                if cqa5_viva(cqa5_d(base + 0xd7 + 4 * k)) {
                    *n = n.wrapping_add(1); // cqa5: cada um do elenco conta
                }
            }
        };
        crate::comparacao::ramo("cqa_5 conta o elenco");
        contar(clube, &mut n);
        if parceiro != 0 {
            contar(parceiro, &mut n);
        }
        if n > 0x32 { // cqa5: o limite de 50 no elenco
            if contar_fila != 0 {
                let fila: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x8c1780usize);
                n = n.wrapping_sub(fila(this, clube) as i8);
            }
            if n > 0x32 {
                let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
                if cqa5_d(clube) != cqa5_d(o + 8) || humano(clube, 1, 1, 0, 0) != 0 || cqa5_d(o + 4) != 0 {
                    crate::comparacao::ramo("cqa_5 elenco cheio");
                    return cqa5_falha(clube, o, motivo, 4);
                }
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
                let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
                let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
                let mut melhor = 1000000.0f64;
                let mut escolhido = 0usize;
                for k in 0..0x32 {
                    let p = cqa5_d(clube + 0xd7 + 4 * k);
                    if !cqa5_viva(p) || ficha(cqa5_d(0xae1050), p) != 0 {
                        continue;
                    }
                    // a primeira consulta do contrato não vai para [0xae1080]
                    let c = contrato(cqa5_d(0xae1050), p);
                    if c == 0 {
                        continue;
                    }
                    let s = cqa5_b(c + 0x3d) as i8;
                    let v = (s as f64 + 100.0) * (cqa5_d(p + 0x52) as i32 as f64);
                    if !(v < melhor) || s > 0 {
                        continue;
                    }
                    let c = cpx1_contrato(p);
                    if c != 0 && cqa5_b(c + 0x35) & 0x3f == 5 {
                        continue;
                    }
                    if ficha(cqa5_d(0xae1050), p) != 0 {
                        continue;
                    }
                    let g = cqa5_d(0xde1f54);
                    if ofertas(g, p, 0, 0) != 0 || trocas(g, p, 0) != 0 || futura(g, p, 0) != 0 {
                        continue;
                    }
                    if cqa5_na_fila_livre(p) {
                        continue;
                    }
                    if matches!(cqa5_b(p + 0x3d), 0xb | 0xf | 0xd) {
                        escolhido = p;
                        melhor = v;
                    }
                }
                if escolhido == 0 {
                    return cqa5_falha(clube, o, motivo, 4);
                }
                crate::comparacao::ramo("cqa_5 libera o mais barato");
                if CQA5_ENSAIO {
                    CQA5_LIBERADO = escolhido;
                } else {
                    let registrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86d800usize);
                    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) = core::mem::transmute(0x8c1af0usize);
                    registrar(cqa5_d(0xdd7f00), escolhido);
                    liberar(this, escolhido, clube, 0, -1, 0);
                }
            }
        }
    }
    // as vagas da comissão: (conta, deslocamento, vagas)
    for (k, base, vagas) in [(0usize, 0x19fusize, 5usize), (2, 0x1cf, 3), (1, 0x1b3, 7)] {
        let mut m = com[k];
        if m <= 0 {
            continue;
        }
        for j in 0..vagas {
            if cqa5_d(clube + base + 4 * j) != 0 {
                m = m.wrapping_add(1);
            }
        }
        if parceiro != 0 {
            for j in 0..vagas {
                if cqa5_viva(cqa5_d(parceiro + base + 4 * j)) {
                    m = m.wrapping_add(1);
                }
            }
        }
        if m > vagas as i8 {
            return cqa5_falha(clube, o, motivo, 0x22);
        }
    }
    if com[3] > 0 && (cqa5_d(clube + 0xd3) != 0 || (parceiro != 0 && cqa5_d(parceiro + 0xd3) != 0)) {
        return cqa5_falha(clube, o, motivo, 0x22);
    }
    1
}

/// Comparador da 0x8cd450: a cópia em ensaio (sem liberar) e depois a original, do mesmo estado ([0xae1080], o motivo,
/// os cursores da fila livre e da troca). Sem liberação: iguais a devolução e essas regiões. Com liberação (a cópia
/// guardou o escolhido): iguais a devolução e o escolhido ter saído do clube (o contrato dele, pela 0x4cf5a0 que só
/// lê, sem o clube); o resto a original refaz ao liberar. Clube nulo: a caixa, só a original.
unsafe extern "thiscall" fn comparar_vaga_no_elenco_para_a_oferta(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32>(1432);
    if clube == 0 {
        return f(this, clube, o, contar_fila, motivo);
    }
    let mut regioes = vec![(0xae1080usize, 4usize), (cqa5_d(0xde1f54) + 0x936, 4)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    if cqa5_d(o + 4) != 0 {
        regioes.push((cqa5_d(o + 4) + 0xa, 4));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQA5_ENSAIO = true;
    CQA5_LIBERADO = 0;
    let n = crate::aleatorio::pausar_rastro(|| vaga_no_elenco_para_a_oferta(this, clube, o, contar_fila, motivo));
    CQA5_ENSAIO = false;
    let liberado = CQA5_LIBERADO;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = f(this, clube, o, contar_fila, motivo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if liberado == 0 {
        mo.0 == mr.0
    } else {
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato(cqa5_d(0xae1050), liberado);
        c == 0 || cpx1_clube(cqa5_d(c + 4)) != clube
    };
    PLACAR.registrar(igual && r == n, || {
        format!("vaga_no_elenco_para_a_oferta(clube {:x}, oferta {:x}, fila {}) original={} rust={} liberado={:x} {:x?} x {:x?}", clube, o, contar_fila, r, n, liberado, mo, mr)
    });
    r
}

/// { 0x8d0dd0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_sale_percentage(pessoa)`, das
/// telas da pessoa (0x884ea2): sem a marca 8 da pessoa ([this+0x86e] + id × 8), 0. O contrato dela (0x4cf5a0, em
/// [0xae1080]) e o clube; sem os dois, 0. A busca binária (0x946464: cdecl(chave, vetor [this+0x880], conta
/// [this+0x888], 0xd, 0x8e54d0)) pela chave com o id do clube (+1, de novo pelo contrato) e o da pessoa (+9), os dois
/// únicos campos que a comparação lê; achada, devolve o registro; senão a marca 8 sai e 0.
unsafe extern "thiscall" fn percentual_da_proxima_venda_achado(this: usize, p: usize) -> usize {
    let m = (cqa5_d(this + 0x86e).wrapping_add(cqa5_d(p).wrapping_mul(8))) as *mut u8;
    if *m & 8 == 0 {
        return 0;
    }
    if cpx1_clube_do_contrato(p) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0x10];
    chave[9..13].copy_from_slice(&(cqa5_d(p) as u32).to_le_bytes());
    // ponytail: a original lê o id do clube sem conferir o contrato (acabou de achá-lo); sem ele, aqui 0
    let c = cpx1_clube_do_contrato(p);
    let id = if c != 0 { cqa5_d(c) as u32 } else { 0 };
    chave[1..5].copy_from_slice(&id.to_le_bytes());
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(chave.as_ptr(), cqa5_d(this + 0x880), cqa5_d(this + 0x888), 0xd, 0x8e54d0);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(8);
    0
}

/// Comparador da 0x8d0dd0: lado a lado na marca da pessoa, em [0xae1080] e na devolução (só lê o resto).
unsafe extern "thiscall" fn comparar_percentual_da_proxima_venda_achado(this: usize, p: usize) -> usize {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1433);
    let m = cqa5_d(this + 0x86e).wrapping_add(cqa5_d(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1), (0xae1080, 4)], || f(this, p), || percentual_da_proxima_venda_achado(this, p));
    PLACAR.registrar(igual && r == n, || format!("percentual_da_proxima_venda_achado(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

// ---------- 261a leva: os jogos e as faltas das permissões de trabalho e os prêmios por jogos depois da partida, o prêmio por jogos na seleção guardado e a notícia da transferência finalizada (transfer_manager.cpp) ----------

unsafe fn cpy11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy11_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy11_clube(id: usize) -> usize {
    cpy11_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpy11_pessoa(id: usize) -> usize {
    cpy11_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O byte das marcas da pessoa no registro de 8 bytes da gerência de transferências ([this+0x86e] + id × 8).
unsafe fn cpy11_marca(this: usize, p: usize) -> usize {
    cpy11_d(this + 0x86e).wrapping_add(cpy11_d(p).wrapping_mul(8))
}

/// A permissão de trabalho da pessoa (0xe bytes, [this+0x93a], conta +0x93e), como a original procura em linha: id fora
/// dos [0xae2364] (com sinal) ou sem a marca 0x20, 0; senão a busca binária (bsearch 0x946464 com 0x8e55c0).
unsafe fn cpy11_permissao(this: usize, p: usize) -> usize {
    if p != 0 && cpy11_d(p) as i32 >= cpy11_d(0xae2364) as i32 {
        return 0;
    }
    if cpy11_b(cpy11_marca(this, p)) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpy11_d(p) as u32];
    bsearch(chave.as_ptr(), cpy11_d(this + 0x93a), cpy11_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// Soma 1 à palavra `o` da permissão (+4 jogos, +6 faltas). A original soma sem conferir a busca (a marca garante a
/// permissão); sem ela, a original cairia: aqui nada.
unsafe fn cpy11_contar(r: usize, o: usize) {
    if r != 0 {
        let a = (r + o) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
    }
}

/// O ensaio do comparador: a cópia roda sem pagar os prêmios por jogos (0x8cb340 dá notícia, aloca e paga), como se
/// nenhum pagasse (sem tirar o bônus); a contagem das permissões não depende deles.
static mut CPY11_ENSAIO: bool = false;

/// TRANSFER_MANAGER::pay_appearance_money (0x8cb340: thiscall(this, pessoa, jogos, tipo, 0), ret 0x10); no ensaio, 0.
unsafe fn cpy11_pagar(this: usize, p: usize, jogos: i32, tipo: u32) -> u32 {
    if CPY11_ENSAIO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32 = core::mem::transmute(0x8cb340usize);
    f(this, p, jogos, tipo, 0)
}

/// { 0x8c39f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::update_appearance_clauses_after_match(jogo,
/// partida)`, pelo fim de cada partida (0x6b5f00 e as estatísticas simuladas 0x84be20). Para os dois lados (+0x1c casa,
/// +0x20 fora): o jogo conta para as permissões de trabalho com competição (+0x14) que não é de seleção (casa pela
/// 0x53b900) nem amistoso (0x5c6710) e o byte do lado (+0x40/+0x41) ligado, ou com +0 = −2. Clube que não é seleção: o
/// elenco (0x541f90, 50 vagas; seleção: o vetor fica como estava) e, contando, os jogadores dele com a marca 0x20 (o
/// registro de 8 bytes em +0x86e). Nas 20 vagas da partida (+0x190 o id, +0x1ad ≠ −5, reservas com +0x1c2 ≠ −1; sem
/// ficha de jogador +0x61, fim das vagas): fora do empréstimo (0x4cf6d0, só no clube) e com a marca da cláusula (2
/// clube, 4 seleção), os jogos (seleção: +0x22 menos o byte +7 do registro; clube: +0x3b do contrato 0x4cf5a0) múltiplos
/// de 10 até 50, ou 1, pagam o prêmio (0x8cb340 com o tipo 5 ou 2) e, pago, o bônus sai (0x8cb630 ou 0x8cb590). Contando,
/// o jogador marcado soma um jogo (+4 da permissão) e sai da lista; os que sobram, sem lesão (0x62c400), suspensão
/// (0x55d270), convocação (0x7757d0) nem inelegibilidade (0x8cf5e0), e não emprestados por outro clube (0x4cf6d0 com o
/// contrato guardado em [0xae1080]), somam uma falta (+6).
unsafe extern "thiscall" fn jogos_das_permissoes_e_clausulas(this: usize, jogo: usize, partida: usize) {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    let tirar_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x7757d0usize);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    // ponytail: o elenco local da original (esp+0x110) não é preenchido no lado da seleção e fica com o que a pilha tinha
    // (o do outro lado ou lixo); aqui começa zerado (vaga vazia, que o jogo também produz) e passa de um lado ao outro
    let mut elenco = [0usize; 50];
    let mut lado: i8 = 0;
    while lado < 2 {
        let clube = cpy11_d(jogo + if lado == 0 { 0x1c } else { 0x20 });
        let comp = cpy11_d(jogo + 0x14);
        let fora = comp == 0 || e_selecao(cpy11_d(jogo + 0x1c)) != 0 || amistoso(cpy11_d(0xae2c74), comp) != 0;
        let contando = (!fora && cpy11_b(jogo + 0x40 + lado as usize) != 0) || cpy11_d(jogo) as i32 == -2;
        let mascara: u8 = if e_selecao(clube) != 0 {
            4
        } else {
            reunir(clube, elenco.as_mut_ptr(), 0);
            2
        };
        let mut seguidos = [0usize; 50];
        let mut n: i8 = 0;
        if contando {
            for &p in elenco.iter() {
                if p != 0 && (cpy11_d(p) as i32) < cpy11_d(0xae2364) as i32 && cpy11_b(cpy11_marca(this, p)) & 0x20 != 0 {
                    seguidos[n as usize] = p;
                    n += 1;
                }
            }
        }
        let base = partida + lado as usize * 0x625;
        let mut k: i8 = 0;
        while k < 0x14 {
            let v = base + k as usize * 0x49;
            let id = cpy11_d(v + 0x190) as i32;
            if id != -1 && cpy11_b(v + 0x1ad) != 0xfb && (k < 0xb || cpy11_b(v + 0x1c2) != 0xff) {
                let p = cpy11_pessoa(id as usize);
                if cpy11_d(p + 0x61) == 0 {
                    break;
                }
                if (p == 0 || (cpy11_d(p) as i32) < cpy11_d(0xae2364) as i32) && !(mascara == 2 && ficha(cpy11_d(0xae1050), p) != 0) {
                    let reg = cpy11_marca(this, p);
                    if cpy11_b(reg) & mascara != 0 {
                        let (jogos, tipo): (i16, u8) = if mascara == 4 {
                            ((cpy11_b(p + 0x22) as u16).wrapping_sub(cpy11_b(reg + 7) as u16) as i16, 5)
                        } else {
                            let mut j = 0i16;
                            if contrato(cpy11_d(0xae1050), p) != 0 {
                                j = cpy11_w(contrato(cpy11_d(0xae1050), p) + 0x3b);
                            }
                            (j, 2)
                        };
                        // ponytail: a original empurra o dword do local do tipo, com os 3 bytes altos de lixo da pilha;
                        // a 0x8cb340 e a notícia só leem o byte: vai limpo
                        if (jogos as i32 % 10 == 0 && jogos <= 0x32) || jogos == 1 { // cpy11: os jogos do prêmio
                            if cpy11_pagar(this, p, jogos as i32, tipo as u32) != 0 {
                                crate::comparacao::ramo("cpy_11 premio pago");
                                if tipo == 5 {
                                    tirar_selecao(this, p);
                                } else {
                                    tirar_time(this, p);
                                }
                            }
                        }
                    }
                    if contando && cpy11_b(cpy11_marca(this, p)) & 0x20 != 0 {
                        let r = cpy11_permissao(this, p);
                        crate::comparacao::ramo("cpy_11 jogo contado");
                        cpy11_contar(r, 4); // cpy11: o jogo conta na permissão
                        let mut i: i8 = 0;
                        while i < n {
                            if seguidos[i as usize] == p {
                                seguidos[i as usize] = 0;
                                break;
                            }
                            i += 1;
                        }
                    }
                }
            }
            k += 1;
        }
        if contando && n > 0 {
            for i in 0..n as usize {
                let p = seguidos[i];
                if p == 0 || lesao(cpy11_d(0xb63d28), p) != 0 {
                    continue;
                }
                let comp = cpy11_d(jogo + 0x14);
                if comp != 0 && suspenso(cpy11_d(0xae28f0), p, comp, 1, 0) != 0 {
                    continue;
                }
                if convocado(cpy11_d(0xb74318), p, 0xae2c90, clube, 0, 0) != 0 {
                    continue;
                }
                if cpy11_d(jogo + 0x14) != 0 && inelegivel(this, p, jogo) != 0 {
                    continue;
                }
                if ficha(cpy11_d(0xae1050), p) != 0 {
                    let c = contrato(cpy11_d(0xae1050), p);
                    *(0xae1080 as *mut usize) = c;
                    let dono = if c == 0 { 0 } else { cpy11_clube(cpy11_d(c + 4)) };
                    if dono == clube {
                        continue;
                    }
                }
                let r = cpy11_permissao(this, p);
                crate::comparacao::ramo("cpy_11 falta contada");
                cpy11_contar(r, 6); // cpy11: a falta conta na permissão
            }
        }
        lado += 1;
    }
}

/// Comparador: a cópia em ensaio (sem pagar os prêmios) e a original a partir do mesmo estado do vetor das permissões
/// de trabalho; os jogos (+4) e as faltas (+6) têm de bater. Seleção em casa: o elenco local da original começa com o
/// lixo da pilha, só a original roda. Os prêmios se provam pela própria 0x8cb340 (comparada) e pela carreira.
unsafe extern "thiscall" fn comparar_jogos_das_permissoes_e_clausulas(this: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1371);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let v = cpy11_d(this + 0x93a);
    let n = cpy11_d(this + 0x93e) as i32;
    if e_selecao(cpy11_d(jogo + 0x1c)) != 0 || v == 0 || n <= 0 {
        return o(this, jogo, partida); // ponytail: só a original (lixo da pilha ou sem permissões)
    }
    let regioes = [(v, n as usize * 0xe)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPY11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| jogos_das_permissoes_e_clausulas(this, jogo, partida)));
    CPY11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, jogo, partida);
    // a original não realoca o vetor (os prêmios mexem em outros vetores)
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("jogos_das_permissoes_e_clausulas(jogo {:x}) permissões diferentes nos bytes {:x?}", jogo, dif)
    });
}

/// { 0x8cad90 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_international_team_appearance_money(
/// registro)`: acrescenta o prêmio por jogos na seleção (13 bytes copiados de `registro`) ao vetor [this+0x898]
/// (capacidade +0x89c, conta +0x8a0); cheio, cresce de 50 em 50 (sem memória: caixa 0x25af e nada). Reordena pela pessoa
/// (qsort 0x945333 com 0x8b4e00). A gêmea do time principal é a 0x8cac40.
unsafe extern "thiscall" fn guardar_premio_por_jogos_na_selecao(this: usize, registro: usize) {
    crate::comparacao::ramo("cpy_11 premio na selecao guardado");
    if !cpl3_crescer(this, 0x898, 4, 50, 0xd, 0x25af, 0xa8ebe0) {
        return;
    }
    let n = cpy11_d(this + 0x8a0);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpy11_d(this + 0x898).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    core::ptr::write_unaligned((this + 0x8a0) as *mut usize, n.wrapping_add(1));
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpy11_d(this + 0x898), cpy11_d(this + 0x8a0) as u32, 0xd, 0x8b4e00);
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta e os 13
/// bytes), a original roda; confere a conta e que o registro está no vetor (reordenado).
unsafe extern "thiscall" fn comparar_guardar_premio_por_jogos_na_selecao(this: usize, registro: usize) {
    let n = cpy11_d(this + 0x8a0);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1372)(this, registro);
    if cpy11_d(this + 0x898) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpy11_d(this + 0x8a0);
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpy11_d(this + 0x898), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("guardar_premio_por_jogos_na_selecao: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// { 0x8da5e0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da data e da notícia locais): `TRANSFER_MANAGER::
/// post_transfer_finalised_news(oferta)`, pela transferência acertada (0x8c2f50). O vendedor: o clube do contrato da
/// pessoa (0x4cf5a0, guardado em [0xae1080]) fora do tipo 5 (pedido de novo, sem guardar). A notícia 0xfa5: a pessoa (o
/// id e as três chaves), o comprador, o vendedor (ou −1), +0x20, +0x24, o valor +0x1b, o estado +0x2c, o total
/// (0x8e7a70 com 0, 1, 0, 1), +0x2f, +0x11; a data de conclusão (0x8cee70) depois de hoje vai nos campos 0xe (dia), 0xf
/// (ano) e 0xd (o nibble baixo de +0x30, com sinal), senão 0, 0, 0; a conta da troca (+4) no 0xb. Ao vendedor humano
/// (0x7875e0), fora do estado 0xc e sem troca pendente (+0x2f com nibble +0x30), e ao comprador humano, cada um no
/// campo 0x10.
unsafe extern "thiscall" fn noticia_transferencia_finalizada(this: usize, o: usize) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u16 = core::mem::transmute(0x8cee70usize);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpy11_pessoa(cpy11_d(o + 0xc));
    let c = contrato(cpy11_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let mut vendedor = if c == 0 || cpy11_b(c + 0x35) & 0x3f == 5 { 0 } else { cpy11_clube(cpy11_d(c + 4)) };
    let comprador = cpy11_clube(cpy11_d(o + 8));
    let mut data = [0u8; 8];
    data_padrao(data.as_mut_ptr());
    if vendedor != 0 && cpy11_b(contrato(cpy11_d(0xae1050), p) + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    crate::comparacao::ramo("cpy_11 noticia da transferencia finalizada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa5, 0);
    gravar(nt, 0, cpy11_d(p) as u32);
    gravar(nt, 1, cpy11_d(cpy11_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpy11_d(cpy11_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpy11_d(cpy11_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpy11_d(o + 8) as u32);
    gravar(nt, 5, if vendedor != 0 { cpy11_d(vendedor) as u32 } else { u32::MAX });
    gravar(nt, 6, cpy11_d(o + 0x20) as u32);
    gravar(nt, 7, cpy11_d(o + 0x24) as u32);
    gravar(nt, 8, cpy11_d(o + 0x1b) as u32);
    gravar(nt, 9, cpy11_b(o + 0x2c) as i8 as u32);
    gravar(nt, 0xa, total(o, 0, 1, 0, 1));
    gravar(nt, 0xc, cpy11_b(o + 0x2f) as i8 as u32);
    gravar(nt, 0x11, cpy11_b(o + 0x11) as i8 as u32);
    conclusao(this, o, data.as_mut_ptr());
    let (dia, ano) = (i16::from_le_bytes([data[0], data[1]]), u16::from_le_bytes([data[2], data[3]]));
    let (hoje_dia, hoje_ano) = (cpy11_w(0xae2c90), cpy11_w(0xae2c92) as u16);
    if hoje_ano < ano || (hoje_ano == ano && hoje_dia < dia) {
        gravar(nt, 0xe, dia as i32 as u32);
        gravar(nt, 0xf, ano as u32);
        gravar(nt, 0xd, (((cpy11_b(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    } else {
        gravar(nt, 0xe, 0);
        gravar(nt, 0xf, 0);
        gravar(nt, 0xd, 0);
    }
    let troca = cpy11_d(o + 4);
    gravar(nt, 0xb, if troca != 0 { cpy11_b(troca) as i8 as u32 } else { 0 });
    let jornal = cpy11_d(0xb7431c);
    if vendedor != 0 && humano(jornal, vendedor) != 0 && cpy11_b(o + 0x2c) != 0xc && (cpy11_b(o + 0x2f) == 0 || cpy11_b(o + 0x30) & 0xf == 0) {
        gravar(nt, 0x10, cpy11_d(vendedor) as u32);
        entregar(cpy11_d(0xb7431c), nt, vendedor);
    }
    if humano(cpy11_d(0xb7431c), comprador) != 0 {
        gravar(nt, 0x10, cpy11_d(comprador) as u32);
        entregar(cpy11_d(0xb7431c), nt, comprador);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_transferencia_finalizada(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1373)(this, o)
}

// ---------- 261a leva: a pessoa liberada sem custo, as férias do técnico humano e a notícia da permissão para discutir o contrato (transfer_manager.cpp, human_manager.cpp) ----------

unsafe fn cpy13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy13_clube(id: usize) -> usize {
    cpy13_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpy13_pessoa(id: usize) -> usize {
    cpy13_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpy13_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpy13_d(0xae1050), p)
}

/// Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): a data de hoje ([0xae2c90]) no destino.
unsafe fn cpy13_hoje_em(destino: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), 0xae2c90);
}

/// { 0x8c1af0 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::free_transfer_staff(pessoa, clube, motivo,
/// compensação, só_registro)`, pela fila das transferências livres (0x8c18c0), pelo fim dos contratos (0x4c7ac0) e pela
/// 0x8cd450. Pessoa emprestada (0x4cf6d0): nada. A transferência passada (0x25 bytes no vetor [this+0x862], capacidade
/// +0x866, conta +0x86a; cheio, cresce de 100 em 100; sem memória: a caixa 0xce9 e nada): +0x10 −1, +0x14 o clube ou −1,
/// a data de hoje em +0x1d, a pessoa (o id e as três chaves), +0x1c 5 (só registro) ou 3, +0x18 0. Só registro: acabou.
/// Senão: com ficha (+0x61), o histórico (0x7c5fe0 em [0xb74350] com o motivo); com clube, a compensação (−1: com
/// contrato do tipo 2, os dias até o fim (0x85fa00) × o salário (+0xc) / 7; senão 0) paga ao clube (0x599920), a situação
/// nas listas de observação (0x843e10, evento 4, tipo 2) e, no clube atual (+0x39), as duas escalações (0x896c60 em
/// [0xde1d08]) e o elenco (0x53c190), com a data +0x3e de hoje; as ofertas de contrato do clube do contrato (0x4cca70,
/// contrato guardado em [0xae1080]), o contrato principal (0x4d0b50), +0x52 e +0x6d zerados, as três cláusulas (0x8cb4f0,
/// 0x8cb590, 0x8cb630), a permissão de trabalho (0x8c7e20 sem clube) e 0x4196a0 em [0xad9c6c].
unsafe extern "thiscall" fn liberar_sem_custo(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if ficha(cpy13_d(0xae1050), p) != 0 {
        return;
    }
    if !cpl3_crescer(this, 0x862, 4, 100, 0x25, 0xce9, 0xa8e998) {
        return;
    }
    let n = cpy13_d(this + 0x86a);
    let r = cpy13_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    core::ptr::write_unaligned((this + 0x86a) as *mut usize, n.wrapping_add(1));
    let por = |o: usize, v: u32| core::ptr::write_unaligned((r + o) as *mut u32, v);
    por(0x10, u32::MAX);
    por(0x14, if clube != 0 { cpy13_d(clube) as u32 } else { u32::MAX });
    cpy13_hoje_em(r + 0x1d);
    por(0, cpy13_d(p) as u32);
    por(4, cpy13_d(cpy13_d(p + 4) + 0x33) as u32);
    por(8, cpy13_d(cpy13_d(p + 8) + 0x33) as u32);
    por(0xc, cpy13_d(cpy13_d(p + 0xc) + 0x33) as u32);
    *((r + 0x1c) as *mut u8) = if so_registro != 0 { 5 } else { 3 };
    por(0x18, 0);
    if so_registro != 0 {
        return;
    }
    crate::comparacao::ramo("cpy_13 liberado");
    if cpy13_d(p + 0x61) != 0 {
        let historico: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32) = core::mem::transmute(0x7c5fe0usize);
        historico(cpy13_d(0xb74350), p, 0, 0, -1, motivo, 0);
    }
    if clube != 0 {
        let c = cpy13_contrato(p);
        let mut comp = compensacao;
        if comp == -1 {
            comp = if c != 0 && cpy13_b(c + 0x35) & 0x3f == 2 {
                let dias: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(0x85fa00usize);
                (dias(c) as i32).wrapping_mul(cpy13_d(c + 0xc) as i32) / 7
            } else {
                0
            };
        }
        let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
        let pagar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x599920usize);
        let fin = financas(cpy13_d(0xae2a38), clube);
        if comp != 0 {
            crate::comparacao::ramo("cpy_13 compensacao paga");
        }
        pagar(fin, comp); // cpy13: a compensação ao clube
        let listas: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
        listas(cpy13_d(this + 0x856), p, clube, 4, 2); // cpy13: a situação nas listas de observação
        if cpy13_d(p + 0x39) != 0 {
            let escalacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
            let elenco: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
            crate::comparacao::ramo("cpy_13 sai do clube atual");
            escalacao(cpy13_d(0xde1d08), cpy13_d(p + 0x39), p, 1, 1); // cpy13: as escalações
            escalacao(cpy13_d(0xde1d08), cpy13_d(p + 0x39), p, 0, 1);
            elenco(cpy13_d(p + 0x39), p, 0, 1, 0);
        }
        cpy13_hoje_em(p + 0x3e);
    }
    let c = cpy13_contrato(p);
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpy13_clube(cpy13_d(c + 4)) != 0 {
        let c = cpy13_contrato(p);
        *(0xae1080 as *mut usize) = c;
        let dono = if c == 0 { 0 } else { cpy13_clube(cpy13_d(c + 4)) };
        let tirar_ofertas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
        tirar_ofertas(cpy13_d(0xae1050), p, dono);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d0b50usize);
    tirar_contrato(cpy13_d(0xae1050), p);
    core::ptr::write_unaligned((p + 0x52) as *mut u32, 0);
    *((p + 0x6d) as *mut u8) = 0;
    let percentual: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb4f0usize);
    let bonus_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let bonus_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8c7e20usize);
    let fim: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4196a0usize);
    percentual(this, p);
    bonus_time(this, p);
    bonus_selecao(this, p);
    permissao(this, p, 0);
    fim(cpy13_d(0xad9c6c), p);
}

// ponytail: paga, tira de listas, escalações, elenco e contrato (não pode rodar duas vezes): só a original, conferida
// pelo plano: emprestada, a conta das transferências passadas fica; senão sobe 1 com o registro novo (a pessoa, o
// clube, −1 e o tipo 5 ou 3) e, fora do só registro, sem contrato principal, com +0x52 e +0x6d zerados.
unsafe extern "thiscall" fn comparar_liberar_sem_custo(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let emprestada = ficha(cpy13_d(0xae1050), p) != 0;
    let n = cpy13_d(this + 0x86a);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32)>(1377)(this, p, clube, motivo, compensacao, so_registro);
    if emprestada {
        PLACAR.registrar(cpy13_d(this + 0x86a) == n, || format!("liberar_sem_custo(pessoa {}) emprestada: conta {} -> {}", cpy13_d(p), n, cpy13_d(this + 0x86a)));
        return;
    }
    if cpy13_d(this + 0x862) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let r = cpy13_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let mut ok = cpy13_d(this + 0x86a) == n.wrapping_add(1)
        && cpy13_d(r) == cpy13_d(p)
        && cpy13_d(r + 0x10) as u32 == u32::MAX
        && cpy13_d(r + 0x14) as u32 == if clube != 0 { cpy13_d(clube) as u32 } else { u32::MAX }
        && cpy13_b(r + 0x1c) == if so_registro != 0 { 5 } else { 3 };
    if so_registro == 0 {
        ok = ok && cpy13_contrato(p) == 0 && cpy13_d(p + 0x52) == 0 && cpy13_b(p + 0x6d) == 0;
    }
    PLACAR.registrar(ok, || format!("liberar_sem_custo(pessoa {}, clube {:x}, só registro {}): conta {} -> {}, registro {:x?}", cpy13_d(p), clube, so_registro,
        n, cpy13_d(this + 0x86a), core::slice::from_raw_parts(r as *const u8, 0x1d)));
}

/// { 0x5fd2f0 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::set_on_holiday(férias, técnico)`: sem técnico, o
/// humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); nenhum: a caixa da linha 0x41c. O índice do técnico (id − [0xae2364]
/// + 16) fora de 0..[this+4]: a caixa da linha 0x428. A marca de férias (+0x9a01 do registro de 0xa6ef bytes em [this])
/// passa a `férias`, com a conta dos humanos de férias (+0xc) subindo ao ligar e descendo ao desligar; ao ligar, as
/// notícias pendentes do técnico com o clube (+0x39) e com a seleção (+0x24) são respondidas (0x7874b0); com o dia em
/// [0xb67a50], o interesse do dia (0x6b5040).
unsafe extern "thiscall" fn ferias_do_humano(this: usize, ferias: u32, tecnico: usize) {
    let mut t = tecnico;
    if t == 0 {
        t = cpy13_d(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize));
        if t == 0 {
            aviso_em(0x9cf11c, 0x41c);
            return;
        }
    }
    let i = (cpy13_d(t) as i32).wrapping_sub(cpy13_d(0xae2364) as i32).wrapping_add(0x10);
    if i < 0 || i > (cpy13_d(this + 4) as i32).wrapping_sub(1) {
        aviso_em(0x9cf11c, 0x428);
        return;
    }
    let marca = cpy13_d(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let conta = (this + 0xc) as *mut i32;
    let antes = cpy13_d(marca);
    if ferias != 0 {
        if antes == 0 {
            *conta = (*conta).wrapping_add(1);
        }
    } else if antes != 0 {
        *conta = (*conta).wrapping_sub(1);
    }
    core::ptr::write_unaligned(marca as *mut u32, ferias);
    crate::comparacao::ramo("cpy_13 ferias do humano");
    if ferias != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7874b0usize);
        if cpy13_d(t + 0x39) != 0 {
            responder(cpy13_d(0xb7431c), t, cpy13_d(t + 0x39));
        }
        if cpy13_d(t + 0x24) != 0 {
            responder(cpy13_d(0xb7431c), t, cpy13_d(t + 0x24));
        }
    }
    let dia = cpy13_d(0xb67a50);
    if dia != 0 {
        let interesse: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6b5040usize);
        interesse(dia);
    }
}

// ponytail: responde notícias e mexe no interesse do dia (não pode rodar duas vezes): só a original, conferida pelo
// plano: a marca de férias passa a `férias` e a conta muda de ±1 só quando a marca muda.
unsafe extern "thiscall" fn comparar_ferias_do_humano(this: usize, ferias: u32, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(1378);
    let t = if tecnico != 0 { tecnico } else { cpy13_d(0xb74c26 + 0x300 * (*(0xb77c7a as *const i16) as isize as usize)) };
    let i = if t == 0 { -1 } else { (cpy13_d(t) as i32).wrapping_sub(cpy13_d(0xae2364) as i32).wrapping_add(0x10) };
    if i < 0 || i > (cpy13_d(this + 4) as i32).wrapping_sub(1) {
        return o(this, ferias, tecnico); // a caixa: só a original
    }
    let marca = cpy13_d(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let (antes, conta) = (cpy13_d(marca), cpy13_d(this + 0xc) as i32);
    let esperada = conta + if ferias != 0 && antes == 0 { 1 } else if ferias == 0 && antes != 0 { -1 } else { 0 };
    o(this, ferias, tecnico);
    let (depois, conta2) = (cpy13_d(marca) as u32, cpy13_d(this + 0xc) as i32);
    PLACAR.registrar(depois == ferias && conta2 == esperada, || format!("ferias_do_humano({}, técnico {:x}): marca {:x} -> {:x}, conta {} -> {} (esperava {})", ferias, t, antes, depois, conta, conta2, esperada));
}

/// { 0x8d7cd0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH das notícias locais): `TRANSFER_MANAGER::
/// post_contract_talks_news(oferta)`, pela oferta aceita (0x8c0080). O vendedor: o clube do contrato da pessoa (0x4cf5a0,
/// guardado em [0xae1080]) fora do tipo 5. Comprador humano (0x7875e0): a notícia 0xfac com a pessoa (o id e as três
/// chaves), o vendedor (ou −1), o comprador, +0x20, +0x24, 0 e 0. Vendedor humano com troca (+4): uma 0xfac por jogador
/// da troca, com ele, o comprador, o vendedor, +0x20, +0x24, 0 e 1, ao vendedor.
unsafe extern "thiscall" fn noticia_conversa_de_contrato(_this: usize, o: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let p = cpy13_pessoa(cpy13_d(o + 0xc));
    let c = cpy13_contrato(p);
    *(0xae1080 as *mut usize) = c;
    let vendedor = if c == 0 || cpy13_b(c + 0x35) & 0x3f == 5 { 0 } else { cpy13_clube(cpy13_d(c + 4)) };
    let comprador = cpy13_clube(cpy13_d(o + 8));
    let chaves = |nt: usize, q: usize| {
        gravar(nt, 0, cpy13_d(q) as u32);
        gravar(nt, 1, cpy13_d(cpy13_d(q + 4) + 0x33) as u32);
        gravar(nt, 2, cpy13_d(cpy13_d(q + 8) + 0x33) as u32);
        gravar(nt, 3, cpy13_d(cpy13_d(q + 0xc) + 0x33) as u32);
    };
    if humano(cpy13_d(0xb7431c), comprador) != 0 {
        crate::comparacao::ramo("cpy_13 noticia ao comprador");
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        construir(nt, 0xfac, 0);
        chaves(nt, p);
        gravar(nt, 4, if vendedor != 0 { cpy13_d(vendedor) as u32 } else { u32::MAX });
        gravar(nt, 5, cpy13_d(o + 8) as u32);
        gravar(nt, 7, cpy13_d(o + 0x20) as u32);
        gravar(nt, 8, cpy13_d(o + 0x24) as u32);
        gravar(nt, 9, 0);
        gravar(nt, 0xa, 0);
        entregar(cpy13_d(0xb7431c), nt, comprador);
    }
    if vendedor == 0 || humano(cpy13_d(0xb7431c), vendedor) == 0 || cpy13_d(o + 4) == 0 {
        return;
    }
    let troca = cpy13_d(o + 4);
    core::ptr::write_unaligned((troca + 0xa) as *mut usize, cpy13_d(troca + 2));
    let mut e = proximo(cpy13_d(o + 4));
    let mut local = [0u32; 0x38]; // a segunda notícia local
    let nt = local.as_mut_ptr() as usize;
    while e != 0 {
        let q = cpy13_pessoa(cpy13_d(e));
        crate::comparacao::ramo("cpy_13 noticia ao vendedor");
        construir(nt, 0xfac, 0);
        chaves(nt, q);
        gravar(nt, 4, cpy13_d(o + 8) as u32);
        gravar(nt, 5, cpy13_d(vendedor) as u32);
        gravar(nt, 7, cpy13_d(o + 0x20) as u32);
        gravar(nt, 8, cpy13_d(o + 0x24) as u32);
        gravar(nt, 9, 0);
        gravar(nt, 0xa, 1);
        entregar(cpy13_d(0xb7431c), nt, vendedor);
        e = proximo(cpy13_d(o + 4));
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_noticia_conversa_de_contrato(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1379)(this, o)
}

// ---------- 260a leva: a oferta de transferência recusada pelo clube vendedor e a retirada pelo comprador, com a notícia da oferta recusada (transfer_manager.cpp) ----------

unsafe fn cpx1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx1_clube(id: usize) -> usize {
    cpx1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx1_pessoa(id: usize) -> usize {
    cpx1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como
/// a original faz depois de cada chamada.
unsafe fn cpx1_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpx1_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
unsafe fn cpx1_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx1_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O ensaio do comparador: a cópia roda sem as chamadas que mexem no resto do jogo (o desassossego com sorteio, a
/// notícia, o agendamento, as listas de observação e as ofertas de contrato), que não podem rodar duas vezes.
static mut CPX1_ENSAIO: bool = false;

/// As ofertas que o ensaio passou do estado 0xf para o 8 (`cpx1_ensaio_reenviar`), devolvidas ao 0xf depois da cópia.
static mut CPX1_DESFAZER: Vec<usize> = Vec::new();

/// No ensaio, o que a 0x4cf4a0 (as ofertas de contrato guardadas da pessoa voltam) muda no que a marca de oferta lê: em
/// cada oferta de contrato guardada da pessoa (lista [[0xae1050]+0x38], +0 o id) ligada a uma oferta de transferência
/// (+0x59 a vaga, ≠ −1; 0x8c61e0 da gerência [0xde1f54]) com a mesma série (+0x5d = +0x24 dela), a oferta no estado
/// 0xf volta ao 8 e deixa de contar como encerrada.
unsafe fn cpx1_ensaio_reenviar(id: usize) {
    let vaga: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c61e0usize);
    for e in cpx1_nos(cpx1_d(cpx1_d(0xae1050) + 0x38)) {
        if cpx1_d(e) != id || cpx1_d(e + 0x59) == usize::MAX {
            continue;
        }
        let v = vaga(cpx1_d(0xde1f54), cpx1_d(e + 0x59));
        if v == 0 || cpx1_d(v) == 0 {
            continue;
        }
        let r = cpx1_d(v);
        if cpx1_d(r + 0x24) == cpx1_d(e + 0x5d) && cpx1_b(r + 0x2c) == 0xf {
            *((r + 0x2c) as *mut u8) = 8;
            (*core::ptr::addr_of_mut!(CPX1_DESFAZER)).push(r);
        }
    }
}

/// A gerência da lista de observação (Shortlist_Manager em +0x856 da gerência de transferências).
unsafe fn cpx1_listas(this: usize) -> usize {
    cpx1_d(this + 0x856)
}

/// Shortlist_Manager::clear_bid_markers_for_player (0x84aeb0: thiscall(gerência, clube, pessoa), ret 8); no ensaio,
/// nada.
unsafe fn cpx1_desmarcar(this: usize, clube: usize, pessoa: usize) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    f(cpx1_listas(this), clube, pessoa);
}

/// Shortlist_Manager::update_staff_status (0x843e10: thiscall(gerência, pessoa, clube, evento, tipo), ret 0x10); no
/// ensaio, nada.
unsafe fn cpx1_situacao_na_lista(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
    f(cpx1_listas(this), pessoa, clube, evento, tipo);
}

/// Uma chamada thiscall(gerência de transferências, oferta) com ret 4 (0x8c1290 desassossegar, 0x8cbf70 tirar a
/// transferência futura, 0x8c6db0 fechar as ofertas de contrato da oferta); no ensaio, nada, só o bit 0x10 da marca da
/// pessoa que a 0x8cbf70 apaga (o comparador lê o byte; as marcas dos jogadores da troca ele não lê nem devolve).
unsafe fn cpx1_com_a_oferta(f: usize, this: usize, o: usize) {
    if CPX1_ENSAIO {
        if f == 0x8cbf70 {
            *((cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8))) as *mut u8) &= 0xef;
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    f(this, o);
}

/// O fim comum da recusa e da retirada, a partir do teste da oferta encerrada: sem oferta rival (+0) em aberto, a marca
/// de oferta em andamento (bit 0 do registro de 8 bytes da pessoa em +0x86e) sai.
unsafe fn cpx1_encerrada(o: usize) -> bool {
    match cpx1_b(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpx1_b(o + 0x2e) >= 3,
        _ => false,
    }
}

unsafe fn cpx1_tirar_marca(this: usize, o: usize) {
    if !cpx1_encerrada(o) {
        return;
    }
    let rivais = cpx1_d(o);
    if rivais != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((rivais + 0xa) as *mut usize, cpx1_d(rivais + 2));
        let mut e = proximo(cpx1_d(o));
        while e != 0 {
            let r = cpx1_d(cpx1_d(this).wrapping_add(cpx1_d(e).wrapping_mul(4)));
            if !cpx1_encerrada(r) {
                return;
            }
            e = proximo(cpx1_d(o));
        }
    }
    let m = (cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8))) as *mut u8;
    if *m & 1 != 0 {
        *m &= 0xfe;
    }
}

/// O começo da lista de observação do comprador (0x849ce0) e as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a
/// 0x4ccb30 de cada jogador da troca, cuja marca +4 zera), com +0x10 zerado entre elas; e, com troca, vendedor e
/// vendedor do computador (0x600a50), a situação na lista do comprador de cada jogador da troca que estava nela
/// (0x842080, 0x843e10 evento 1).
unsafe fn cpx1_lista_do_comprador(this: usize, o: usize) {
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let e = entrada(cpx1_listas(this), cpx1_clube(cpx1_d(o + 8)), cpx1_pessoa(cpx1_d(o + 0xc)), (cpx1_b(o + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o registrador do ponteiro da entrada com o setne no byte baixo; a 0x843e10 só lê
        // o byte
        let tipo = (e as u32 & !0xff) | (cpx1_b(e) != 0xb) as u32;
        cpx1_situacao_na_lista(this, cpx1_pessoa(cpx1_d(o + 0xc)), cpx1_clube(cpx1_d(o + 8)), 1, tipo);
    }
}

unsafe fn cpx1_ofertas_de_contrato(this: usize, o: usize, vendedor: usize, comprador: usize) {
    if !CPX1_ENSAIO {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
        tirar(cpx1_d(0xae1050), cpx1_pessoa(cpx1_d(o + 0xc)), cpx1_clube(cpx1_d(o + 8)));
    }
    *((o + 0x10) as *mut u8) = 0;
    let troca = cpx1_d(o + 4);
    if troca != 0 {
        let tirar_da_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
        for e in cpx1_itens(troca) {
            if !CPX1_ENSAIO {
                tirar_da_vaga(cpx1_d(0xae1050), cpx1_pessoa(cpx1_d(e)), cpx1_d(o + 0x20));
            }
            *((e + 4) as *mut u8) = 0;
        }
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if cpx1_d(o + 4) != 0 && vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0 {
        let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
        for e in cpx1_itens(cpx1_d(o + 4)) {
            if na_lista(cpx1_listas(this), cpx1_pessoa(cpx1_d(e)), comprador) != 0 {
                cpx1_situacao_na_lista(this, cpx1_pessoa(cpx1_d(e)), comprador, 1, 0);
            }
        }
    }
}

/// Desmarca os lances do comprador pelo jogador (0x84aeb0) e os do clube do contrato (`vendedor`, ou 0) por cada
/// jogador da troca.
unsafe fn cpx1_desmarcar_lances(this: usize, o: usize, vendedor: usize) {
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    cpx1_desmarcar(this, cpx1_clube(cpx1_d(o + 8)), p); // cpx1: desmarcar os lances do comprador
    let troca = cpx1_d(o + 4);
    if troca != 0 {
        for e in cpx1_itens(troca) {
            cpx1_desmarcar(this, vendedor, cpx1_pessoa(cpx1_d(e)));
        }
    }
}

/// O clube do contrato da pessoa (ou 0), com o contrato guardado em [0xae1080].
unsafe fn cpx1_clube_do_contrato(pessoa: usize) -> usize {
    let c = cpx1_contrato(pessoa);
    if c == 0 {
        0
    } else {
        cpx1_clube(cpx1_d(c + 4))
    }
}

/// A notícia da oferta recusada (0x8d5170: thiscall(gerência, oferta, motivo), ret 8); no ensaio, nada.
unsafe fn cpx1_noticia(this: usize, o: usize, motivo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d5170usize);
    f(this, o, motivo);
}

/// TRANSFER_MANAGER::schedule_offer (0x8c6220: thiscall(gerência, oferta, dias, fatias), ret 0xc); no ensaio, nada.
unsafe fn cpx1_agendar(this: usize, o: usize, dias: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, o, dias, 0);
}

/// { 0x8bf680 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_transfer_offer(vaga, marcar,
/// motivo)`, pelo clube vendedor que recusa (0x8c36b0 e a avaliação dos lances). Com `marcar`, +0x2d da oferta = 1; o
/// jogador pode se desassossegar (0x8c1290); transferência futura (estado 0x13): sai da lista (0x8cbf70). Estado 2
/// (recusada); os lances do comprador pelo jogador e os do clube do contrato pelos da troca saem (0x84aeb0); as ofertas
/// de contrato ligadas fecham (0x8c6db0); a notícia (0x8d5170); a oferta volta em 25 dias (0x8c6220) para sair. Sem
/// rival em aberto, a marca de oferta da pessoa sai (+0x86e). A lista de observação do comprador (0x849ce0, 0x843e10),
/// as ofertas de contrato (0x4cca70, 0x4ccb30) e +0x10 = 0; com troca e vendedor do computador, os jogadores da troca
/// na lista do comprador (0x842080, 0x843e10).
unsafe extern "thiscall" fn recusar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    crate::comparacao::ramo("cpx_1 recusa desassossega");
    cpx1_com_a_oferta(0x8c1290, this, o); // cpx1: o desassossego da recusa
    let atual = *vaga;
    if cpx1_b(atual + 0x2c) == 0x13 {
        cpx1_com_a_oferta(0x8cbf70, this, atual);
    }
    *((o + 0x2c) as *mut u8) = 2;
    let vendedor2 = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    crate::comparacao::ramo("cpx_1 recusa desmarca");
    cpx1_desmarcar_lances(this, o, vendedor2);
    cpx1_com_a_oferta(0x8c6db0, this, o);
    cpx1_noticia(this, o, motivo);
    cpx1_agendar(this, o, 0x19);
    cpx1_tirar_marca(this, o);
    cpx1_lista_do_comprador(this, o);
    cpx1_ofertas_de_contrato(this, o, vendedor, comprador);
}

/// { 0x8bfcb0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::withdraw_transfer_offer(vaga, marcar,
/// motivo)`, pelo comprador que desiste (a revisão dos lances e as ofertas desfeitas). Com `marcar`, +0x2d = 1;
/// transferência futura: sai da lista (0x8cbf70); estado de 0xb para cima: as ofertas de contrato guardadas voltam
/// (0x4cf4a0). Estado 3 (retirada); as ofertas de contrato ligadas fecham (0x8c6db0); os lances saem (0x84aeb0); +0x10 =
/// 4; a oferta volta em 10 dias (0x8c6220). Sem rival em aberto, a marca de oferta da pessoa sai. Motivo 2: o jogador
/// pode se desassossegar (0x8c1290). A lista de observação do comprador, a notícia (0x8d5170), as ofertas de contrato e
/// +0x10 = 0; com troca e vendedor do computador, os jogadores da troca na lista do comprador.
unsafe extern "thiscall" fn retirar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    let atual = *vaga;
    if cpx1_b(atual + 0x2c) == 0x13 {
        cpx1_com_a_oferta(0x8cbf70, this, atual);
    }
    if cpx1_b(o + 0x2c) as i8 >= 0xb {
        if CPX1_ENSAIO {
            cpx1_ensaio_reenviar(cpx1_d(o + 0xc));
        } else {
            let reenviar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cf4a0usize);
            reenviar(cpx1_d(0xae1050), cpx1_d(o + 0xc));
        }
    }
    *((o + 0x2c) as *mut u8) = 3;
    cpx1_com_a_oferta(0x8c6db0, this, o);
    let vendedor2 = cpx1_clube_do_contrato(cpx1_pessoa(cpx1_d(o + 0xc)));
    cpx1_desmarcar_lances(this, o, vendedor2);
    *((o + 0x10) as *mut u8) = 4;
    cpx1_agendar(this, o, 0xa);
    cpx1_tirar_marca(this, o);
    if motivo as u16 == 2 { // cpx1: só o motivo 2 desassossega
        cpx1_com_a_oferta(0x8c1290, this, o);
    } else {
        crate::comparacao::ramo("cpx_1 retirada sem desassossego");
    }
    cpx1_lista_do_comprador(this, o);
    cpx1_noticia(cpx1_d(0xde1f54), o, motivo);
    cpx1_ofertas_de_contrato(this, o, vendedor, comprador);
}

/// As vagas de uma LLIST (ou nenhuma), sem mexer no cursor.
unsafe fn cpx1_nos(l: usize) -> Vec<usize> {
    if l == 0 {
        return Vec::new();
    }
    let cursor = cpx1_d(l + 0xa);
    let v = cpx1_itens(l);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    v
}

/// O comparador da recusa e da retirada: a cópia em ensaio (sem as chamadas que mexem no resto do jogo) e a original a
/// partir do mesmo estado (a oferta, as duas listas dela, as vagas da troca, o registro da pessoa em +0x86e e
/// [0xae1080]); iguais o estado (+0x2c), +0x10, a marca +4 de cada jogador da troca e a marca de oferta da pessoa, que
/// a original grava sem depender das chamadas.
unsafe fn cpx1_comparar(nome: &str, this: usize, vaga: *mut usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let o = *vaga;
    let marca = cpx1_d(this + 0x86e).wrapping_add(cpx1_d(o + 0xc).wrapping_mul(8));
    let nos = cpx1_nos(cpx1_d(o + 4));
    let mut regioes = vec![(o, 0x30usize), (marca, 8), (0xae1080, 4)];
    for l in [cpx1_d(o), cpx1_d(o + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    for &e in &nos {
        regioes.push((e, 5));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX1_ENSAIO = true;
    crate::aleatorio::pausar_rastro(copia);
    CPX1_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    for r in (*core::ptr::addr_of_mut!(CPX1_DESFAZER)).drain(..) {
        *((r + 0x2c) as *mut u8) = 0xf;
    }
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let (a, b) = (&mo.0, &mr.0);
    let mut igual = a[0].1[0x2c] == b[0].1[0x2c] && a[0].1[0x10] == b[0].1[0x10] && a[1].1[0] == b[1].1[0];
    for k in regioes.len() - nos.len()..regioes.len() {
        igual &= a[k].1[4] == b[k].1[4];
    }
    PLACAR.registrar(igual, || format!("{}(oferta {:#x}) original={:x?} rust={:x?}", nome, o, mo, mr));
}

unsafe extern "thiscall" fn comparar_recusar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(1314);
    cpx1_comparar("recusar_oferta_de_transferencia", this, vaga, || o(this, vaga, marcar, motivo), || recusar_oferta_de_transferencia(this, vaga, marcar, motivo))
}

unsafe extern "thiscall" fn comparar_retirar_oferta_de_transferencia(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(1315);
    cpx1_comparar("retirar_oferta_de_transferencia", this, vaga, || o(this, vaga, marcar, motivo), || retirar_oferta_de_transferencia(this, vaga, marcar, motivo))
}

/// { 0x8d5170 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_rejected_news(oferta, motivo)`, pela recusa, pela retirada e pela recusa do empréstimo. Os clubes humanos
/// que recebem (0x8d1130, com 0x2d se a oferta estiver recusada, senão 0x2e; até 30); sem nenhum, nada. A notícia 0xfa1:
/// a pessoa (o id e as três chaves), o clube do contrato (sem contrato de tipo 5; nenhum no motivo 6 com +0x2f), o
/// comprador, a lista da troca (+4), o estado, o motivo, +0x1f, o valor total (0x8e7a70 com 0, 0, 0, 1) e a divisão do
/// clube do contrato (+0x57); a cada destinatário, na vaga 12, 1 se for o comprador ou o clube do contrato.
unsafe extern "thiscall" fn noticia_oferta_recusada(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let p = cpx1_pessoa(cpx1_d(o + 0xc));
    let comprador = cpx1_clube(cpx1_d(o + 8));
    let tipo5 = |c: usize| cpx1_b(c + 0x35) & 0x3f == 5;
    let c = cpx1_contrato(p);
    let do_contrato = if c == 0 || tipo5(c) { 0 } else { cpx1_clube(cpx1_d(c + 4)) };
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (cpx1_b(o + 0x2c) != 2) as u32 + 0x2d, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    let c = cpx1_contrato(p);
    let mut clube = if c == 0 || tipo5(c) { 0 } else { cpx1_clube(cpx1_d(c + 4)) };
    let c = cpx1_contrato(p);
    if c != 0 && tipo5(c) {
        clube = 0;
    }
    if motivo as i16 == 6 && cpx1_b(o + 0x2f) != 0 {
        clube = 0;
    }
    crate::comparacao::ramo("cpx_1 noticia da recusa");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa1, 0);
    gravar(nt, 0, cpx1_d(p) as u32);
    gravar(nt, 1, cpx1_d(cpx1_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpx1_d(cpx1_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpx1_d(cpx1_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, if clube != 0 { cpx1_d(clube) as u32 } else { u32::MAX });
    gravar(nt, 5, cpx1_d(o + 8) as u32);
    gravar(nt, 6, cpx1_d(o + 4) as u32);
    gravar(nt, 7, cpx1_b(o + 0x2c) as i8 as u32);
    gravar(nt, 8, motivo as i16 as u32);
    gravar(nt, 9, cpx1_b(o + 0x1f) as i8 as u32);
    gravar(nt, 10, total(o, 0, 0, 0, 1));
    let divisao = if clube != 0 { cpx1_d(clube + 0x57) } else { 0 };
    gravar(nt, 11, if divisao != 0 { cpx1_d(divisao) as u32 } else { u32::MAX });
    for &d in lista.iter().take(n.max(0) as usize) {
        gravar(nt, 12, (d == comprador || d == do_contrato) as u32);
        entregar(cpx1_d(0xb7431c), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_oferta_recusada(this: usize, o: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1316)(this, o, motivo)
}

// ---------- 260a leva: o jogador desassossegado pelo lance de um clube maior, a notícia da oferta desfeita e a do prêmio por jogos (transfer_manager.cpp) ----------

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
unsafe fn cpx2_contrato(pessoa: usize) -> usize {
    let c = cpx2_contrato_de(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O ensaio do comparador: a cópia para antes da queixa (0x867700), da fuga (0x419840) e das antipatias (0x547e70,
/// 0x547af0), que mexem no resto do jogo, e marca que chegou lá.
static mut CPX2_ENSAIO: bool = false;
static mut CPX2_CHEGOU: bool = false;

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
unsafe extern "thiscall" fn desassossegar_pelo_lance(this: usize, o: usize) {
    let p = cpx2_pessoa(cpx2_d(o + 0xc));
    let c = cpx2_contrato(p);
    let do_contrato = if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    let comprador = cpx2_clube(cpx2_d(o + 8));
    let r = sortear(0x1676);
    crate::comparacao::ramo("cpx_2 sorteio do desassossego");
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
    let pw = pow_do_jogo(x, f64_em(0x968938));
    let a = x * f64_em(0x96a6b0);
    let v = if !(pw >= a) { pw } else { a };
    let mut n = ftol(pow_do_jogo(cpx2_sb(p + 0x5d) as f64, f64_em(0x968950)) * f64_em(0x968f10) - f64_em(0x96a590)) as i16;
    let ficha = cpx2_d(p + 0x61);
    n = n.wrapping_sub((cpx2_w(ficha + 0xb) as i32 / 40) as i16);
    n = n.wrapping_sub(ftol(v) as i16);
    n = n.wrapping_sub(ftol(pow_do_jogo(cpx2_sb(p + 0x57) as f64, f64_em(0x968950))) as i16);
    n = n.wrapping_add(ftol(pow_do_jogo(cpx2_sb(p + 0x5b) as f64, f64_em(0x968930))) as i16);
    n = n.wrapping_add(ftol(pow_do_jogo(cpx2_sb(p + 0x59) as f64, f64_em(0x968930))) as i16);
    if c2 == 0 {
        return;
    }
    if cpx2_b(c2 + 0x4e) & 8 != 0 {
        crate::comparacao::ramo("cpx_2 listado");
        n = n.wrapping_add((-2000i32).wrapping_sub(sortear(0x1770)) as i16); // cpx2: o desconto do listado
    }
    crate::comparacao::ramo("cpx_2 interesse avaliado");
    if !(x >= f64_em(0x96d298)) { // cpx2: o interesse mínimo
        return;
    }
    if n as i32 >= -sortear(0x2bc) {
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
    let a1 = ftol(valor(cpx2_d(0xae1050), p, do_contrato, 0, -1, 0) as f64 * f64_em(0x968940));
    let b1 = ftol(preco(cpx2_d(0xae1050), do_contrato, comprador, p, 0, 1, 1) as f64 * f64_em(0x9689b0));
    let pond = ponderado(o, 0, 0, 0, 1, 1);
    let menor = if a1 < b1 { a1 } else { b1 };
    if pond < menor {
        return;
    }
    if CPX2_ENSAIO {
        CPX2_CHEGOU = true;
        return;
    }
    crate::comparacao::ramo("cpx_2 desassossegado");
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
            let r1 = sortear(0xc8);
            let r2 = sortear(0x190);
            if forca.wrapping_sub(r1).wrapping_add(r2) < -150 {
                let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
                rejeitar(p, do_contrato, 1);
            }
        }
    }
    if x > f64_em(0x9688b0) || n < -5000 || (menor as f64) < pond as f64 * f64_em(0x969040) {
        let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
        desafeto(p, cpx2_d(do_contrato + 0xcf), 1);
    }
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado (a oferta, [0xae1080] e o sorteio). Sem chegar
/// à queixa (quase sempre), tudo igual; chegando, só a original (a queixa, a fuga e as antipatias não podem rodar duas
/// vezes e a original sorteia mais).
unsafe extern "thiscall" fn comparar_desassossegar_pelo_lance(this: usize, o: usize) {
    let orig_f = orig::<unsafe extern "thiscall" fn(usize, usize)>(1317);
    let mut regioes = vec![(o, 0x30usize), (0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX2_ENSAIO = true;
    CPX2_CHEGOU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| desassossegar_pelo_lance(this, o)));
    CPX2_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    orig_f(this, o);
    if CPX2_CHEGOU {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("desassossegar_pelo_lance(oferta {:#x}) original={:x?} rust={:x?}", o, mo, mr));
}

/// { 0x8de790 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_collapsed_news(oferta, clube, motivo)`, pela oferta de contrato aceita (0x8c38b0), pela oferta desfeita
/// (0x8c44e0), pelas permissões de trabalho (0x8c7690) e por 0x8c4950. Os clubes humanos que recebem (0x8d1130 com 0x2f,
/// até 30); sem nenhum, nada. A notícia 0xfb0: o clube do contrato (sem contrato de tipo 5), o comprador, a pessoa (o id
/// e as três chaves), o clube, o motivo e +0x1f; a cada destinatário, menos o próprio clube no motivo 0xb.
unsafe extern "thiscall" fn noticia_oferta_desfeita(this: usize, o: usize, clube: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpx2_pessoa(cpx2_d(o + 0xc));
    let c = cpx2_contrato(p);
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
    crate::comparacao::ramo("cpx_2 noticia da oferta desfeita");
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
unsafe extern "thiscall" fn comparar_noticia_oferta_desfeita(this: usize, o: usize, clube: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1318)(this, o, clube, motivo)
}

/// { 0x8db8a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_appearance_money_news(cláusula, valor, tipo, calado)`, pelo prêmio por jogos (0x8cb340). Calado, sem contrato ou
/// sem o clube dele: nada. Senão a notícia 0xfa6: a pessoa da cláusula (+0: o id e as três chaves), +4, o valor, o
/// clube que paga (+9), o tipo e o clube do contrato; ao clube que paga e ao clube do contrato.
unsafe extern "thiscall" fn noticia_premio_por_jogos(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    let _ = this;
    if calado != 0 {
        return;
    }
    let p = cpx2_pessoa(cpx2_d(clausula));
    let c = cpx2_contrato(p);
    if c == 0 || cpx2_clube(cpx2_d(c + 4)) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpx_2 noticia do premio por jogos");
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
    let c = cpx2_contrato(p);
    // a original lê o id do clube do contrato mesmo sem contrato (o registro 0): nunca acontece aqui
    let clube = if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) };
    gravar(nt, 8, cpx2_d(clube) as u32);
    entregar(cpx2_d(0xb7431c), nt, cpx2_clube(cpx2_d(clausula + 9)));
    let c = cpx2_contrato(p);
    entregar(cpx2_d(0xb7431c), nt, if c == 0 { 0 } else { cpx2_clube(cpx2_d(c + 4)) });
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_premio_por_jogos(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1319)(this, clausula, valor, tipo, calado)
}

// ---------- 260a leva: a transferência efetivada, com a notícia da transferência concluída e a do acordo entre os clubes (transfer_manager.cpp) ----------

unsafe fn cpx3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpx3_clube(id: usize) -> usize {
    cpx3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpx3_pessoa(id: usize) -> usize {
    cpx3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
unsafe fn cpx3_contrato_de(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpx3_d(0xae1050), pessoa)
}

/// O mesmo, guardado em [0xae1080] como a original faz.
unsafe fn cpx3_contrato(pessoa: usize) -> usize {
    let c = cpx3_contrato_de(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato principal que não é de tipo 5 (ou 0), com o contrato guardado em [0xae1080].
unsafe fn cpx3_clube_nao_5(pessoa: usize) -> usize {
    let c = cpx3_contrato(pessoa);
    if c == 0 || cpx3_b(c + 0x35) & 0x3f == 5 {
        0
    } else {
        cpx3_clube(cpx3_d(c + 4))
    }
}

/// Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
unsafe fn cpx3_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpx3_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O teste da oferta encerrada (estados 2, 3, 0x14, 0x10, 4; 0xf com +0x2e de 3).
unsafe fn cpx3_encerrada(o: usize) -> bool {
    match cpx3_b(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpx3_b(o + 0x2e) >= 3,
        _ => false,
    }
}

/// O clube que recebe: sem técnico humano (0x600a50) e com números de camisa (SQUAD_MANAGER 0x85add0 em [0xdd7ef0]),
/// os números de quem chegou (0x85a470).
unsafe fn cpx3_numeros(clube: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85add0usize);
    if usa(cpx3_d(0xdd7ef0), clube, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x85a470usize);
        numerar(cpx3_d(0xdd7ef0), clube, 0); // cpx3: os números de camisa de quem chegou
    }
}

/// { 0x8c9390 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::finalise_transfer(oferta)`, pelo
/// fechamento da oferta (0x8c34e0) e pela transferência futura. A confiança das diretorias (0x6978f0); as regras da
/// nação do comprador (as 28 de +0x8ac, pela nação +0xa): método 1 (sem +0x1f) ou 2 com a oferta; as cláusulas
/// anteriores (0x8c91e0); com vendedor (contrato sem tipo 5), o pagamento (0x8ca8a0); sem +0x1f = 1, os registros do
/// técnico (0x86cd70); os recordes do clube (0x448760). Com troca, o primeiro da troca sai do elenco do clube dele
/// (+0xd7, 50 vagas). Com valor (+0x1b) e sem +0x69 na pessoa, a parte dos clubes anteriores (0x8e4f20). O jogador vai
/// ao comprador (0x8c9720) e os da troca ao vendedor, com os números de camisa; estado 0x14, a oferta volta em 10 dias
/// (0x8c6220); sem rival em aberto, a marca de oferta (+0x86e) sai; os lances (0x84aeb0) e as ofertas de contrato
/// ligadas (0x8c6db0).
unsafe extern "thiscall" fn efetivar_transferencia(this: usize, o: usize) {
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let comprador = cpx3_clube(cpx3_d(o + 8));
    let vendedor = cpx3_clube_nao_5(p);
    let confianca: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6978f0usize);
    crate::comparacao::ramo("cpx_3 confianca das diretorias");
    confianca(cpx3_d(0xb67a44), o); // cpx3: a confiança das diretorias
    let nacao = cpx3_d(cpx3_clube(cpx3_d(o + 8)) + 0x53);
    for k in 0..0x1c {
        let regras = cpx3_d(this + 0x8ac + k * 4);
        if cpx3_d(regras + 0xa) == nacao {
            if regras != 0 {
                let metodo = cpx3_d(cpx3_d(regras) + if cpx3_b(o + 0x1f) == 0 { 4 } else { 8 });
                let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(metodo);
                f(regras, o);
            }
            break;
        }
    }
    let com_a_oferta = |f: usize, t: usize| {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
        f(t, o);
    };
    com_a_oferta(0x8c91e0, this);
    if vendedor != 0 {
        crate::comparacao::ramo("cpx_3 paga a transferencia");
        com_a_oferta(0x8ca8a0, this); // cpx3: o pagamento da transferência
    }
    if cpx3_b(o + 0x1f) != 1 {
        com_a_oferta(0x86cd70, cpx3_d(0xdd7f00));
    }
    com_a_oferta(0x448760, cpx3_d(0xad9dac));
    let troca = cpx3_d(o + 4);
    if troca != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((troca + 0xa) as *mut usize, cpx3_d(troca + 2));
        let q = cpx3_pessoa(cpx3_d(proximo(cpx3_d(o + 4))));
        let clube = cpx3_d(q + 0x39);
        for j in 0..0x32usize {
            let vaga = clube + 0xd7 + j * 4;
            if cpx3_d(vaga) == q {
                *(vaga as *mut usize) = 0;
                break;
            }
        }
    }
    if cpx3_d(o + 0x1b) != 0 && cpx3_d(p + 0x69) == 0 {
        crate::comparacao::ramo("cpx_3 parte dos clubes anteriores");
        com_a_oferta(0x8e4f20, this); // cpx3: a parte dos clubes anteriores
    }
    let mudar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8c9720usize);
    mudar(this, p, comprador, o);
    cpx3_numeros(comprador);
    if cpx3_d(o + 4) != 0 {
        for e in cpx3_itens(cpx3_d(o + 4)) {
            mudar(this, cpx3_pessoa(cpx3_d(e)), vendedor, o);
        }
        cpx3_numeros(vendedor);
    }
    *((o + 0x2c) as *mut u8) = 0x14;
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    agendar(this, o, 0xa, 0);
    'marca: {
        if !cpx3_encerrada(o) {
            break 'marca;
        }
        let rivais = cpx3_d(o);
        if rivais != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            core::ptr::write_unaligned((rivais + 0xa) as *mut usize, cpx3_d(rivais + 2));
            let mut e = proximo(cpx3_d(o));
            while e != 0 {
                if !cpx3_encerrada(cpx3_d(cpx3_d(this).wrapping_add(cpx3_d(e).wrapping_mul(4)))) {
                    break 'marca;
                }
                e = proximo(cpx3_d(o));
            }
        }
        let m = (cpx3_d(this + 0x86e).wrapping_add(cpx3_d(o + 0xc).wrapping_mul(8))) as *mut u8;
        if *m & 1 != 0 {
            *m &= 0xfe;
        }
    }
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let c = cpx3_contrato(p);
    let novo = if c == 0 { 0 } else { cpx3_clube(cpx3_d(c + 4)) };
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    desmarcar(cpx3_d(this + 0x856), cpx3_clube(cpx3_d(o + 8)), p);
    if cpx3_d(o + 4) != 0 {
        for e in cpx3_itens(cpx3_d(o + 4)) {
            desmarcar(cpx3_d(this + 0x856), novo, cpx3_pessoa(cpx3_d(e)));
        }
    }
    com_a_oferta(0x8c6db0, this);
}

// ponytail: muda jogadores de clube, paga e agenda (não pode rodar duas vezes): só a original, com o plano do estado
// final (0x14, que nenhuma chamada depois dele muda); a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_efetivar_transferencia(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1320)(this, o);
    let e = cpx3_b(o + 0x2c);
    PLACAR.registrar(e == 0x14, || format!("efetivar_transferencia(oferta {:#x}) estado={:#x} (plano 0x14)", o, e));
}

/// A notícia local com a pessoa (o id e as três chaves) nas vagas `k` a `k` + 3.
unsafe fn cpx3_pessoa_na_noticia(nt: usize, k: u8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(nt, k as i8, cpx3_d(p) as u32);
    gravar(nt, k.wrapping_add(1) as i8, cpx3_d(cpx3_d(p + 4) + 0x33) as u32);
    gravar(nt, k.wrapping_add(2) as i8, cpx3_d(cpx3_d(p + 8) + 0x33) as u32);
    gravar(nt, k.wrapping_add(3) as i8, cpx3_d(cpx3_d(p + 0xc) + 0x33) as u32);
}

/// A vaga 5 das duas notícias: o id do clube do contrato (sem tipo 5) ou −1 (a original relê o contrato e, sem ele, lê
/// o id no endereço 0, o que não acontece porque acabou de achá-lo).
unsafe fn cpx3_vaga_5(p: usize) -> u32 {
    if cpx3_clube_nao_5(p) == 0 {
        return u32::MAX;
    }
    cpx3_d(cpx3_clube_nao_5(p)) as u32
}

/// Os da troca (+4) na notícia a partir da vaga `k` (os três primeiros, quatro vagas cada); devolve (quantos, a vaga
/// seguinte).
unsafe fn cpx3_troca_na_noticia(nt: usize, o: usize, mut k: u8) -> (i8, u8) {
    let mut n = 0i8;
    let troca = cpx3_d(o + 4);
    if troca != 0 {
        for e in cpx3_itens(troca) {
            if n < 3 {
                cpx3_pessoa_na_noticia(nt, k, cpx3_pessoa(cpx3_d(e)));
                k = k.wrapping_add(4);
            }
            n = n.wrapping_add(1);
        }
    }
    (n, k)
}

/// A entrega: com técnico humano no comprador (0x7875e0), a todos os técnicos (0x786780); senão a cada destinatário.
unsafe fn cpx3_entregar(nt: usize, o: usize, lista: &[usize], n: i8) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(cpx3_d(0xb7431c), cpx3_clube(cpx3_d(o + 8))) != 0 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(cpx3_d(0xb7431c), nt);
        return;
    }
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(cpx3_d(0xb7431c), nt, d);
    }
}

/// { 0x8dbef0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_completed_news(oferta)`, pelo fechamento da oferta (0x8c34e0). Os clubes humanos que recebem (0x8d1130
/// com 0x5f, até 30); sem nenhum, nada. A notícia 0xfa8: a pessoa, o comprador, o clube do contrato, o valor (+0x1b), 0,
/// o valor total (0x8e7a70 com 1, 1, 0, 1), +0x2f, +0x11, as cláusulas (0x8e7ef0); com +0x2f, contrato de clube e +0x30
/// & 0xf = 3, o fim do contrato se ainda vier (vagas 10 e 11). Os da troca (até 3, da vaga 15) e quantos (14); um
/// sorteio(3) na vaga seguinte; com ficha, 1, os pontos fortes (0x546ab0, sorteia) e as posições (0x5b7d20), senão 0.
unsafe extern "thiscall" fn noticia_transferencia_concluida(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let clausulas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8e7ef0usize);
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x5f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia concluida");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa8, 0);
    cpx3_pessoa_na_noticia(nt, 0, p);
    gravar(nt, 4, cpx3_d(o + 8) as u32);
    gravar(nt, 5, cpx3_vaga_5(p));
    gravar(nt, 6, cpx3_d(o + 0x1b) as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, total(o, 1, 1, 0, 1));
    gravar(nt, 9, cpx3_b(o + 0x2f) as i8 as u32);
    gravar(nt, 0xc, cpx3_b(o + 0x11) as i8 as u32);
    gravar(nt, 0xd, clausulas(o));
    if cpx3_b(o + 0x2f) != 0 && cpx3_clube_nao_5(p) != 0 && cpx3_b(o + 0x30) & 0xf == 3 {
        let c = cpx3_contrato_de(p);
        let ano = cpx3_w(0xae2c92) as u16;
        let vem = c != 0
            && (ano < cpx3_w(c + 0x2f) as u16 || (ano == cpx3_w(c + 0x2f) as u16 && cpx3_w(0xae2c90) < cpx3_w(c + 0x2d)));
        if vem {
            gravar(nt, 10, cpx3_w(c + 0x2d) as i32 as u32);
            gravar(nt, 11, cpx3_w(c + 0x2f) as u16 as u32);
        } else {
            gravar(nt, 10, 0);
            gravar(nt, 11, 0);
        }
    }
    let (m, k) = cpx3_troca_na_noticia(nt, o, 0xf);
    gravar(nt, 0xe, m as i32 as u32);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    gravar(nt, k as i8, sorteio(3) as u32);
    let ficha = cpx3_d(p + 0x61);
    if ficha != 0 {
        gravar(nt, k.wrapping_add(1) as i8, 1);
        let fortes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x546ab0usize);
        let posicoes: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x5b7d20usize);
        let clube = cpx3_clube_nao_5(p);
        gravar(nt, k.wrapping_add(2) as i8, fortes(p, clube));
        gravar(nt, k.wrapping_add(3) as i8, posicoes(cpx3_d(p + 0x61)) as i32 as u32);
    } else {
        gravar(nt, k.wrapping_add(1) as i8, 0);
    }
    cpx3_entregar(nt, o, &lista, n);
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_transferencia_concluida(this: usize, o: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1321)(this, o)
}

/// { 0x8e05d0 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_agreed_news(oferta, data)`, pela aprovação dos clubes (0x8c3310). Os clubes humanos que recebem
/// (0x8d1130 com 0xf, até 30); sem nenhum, nada. A notícia 0xfb3: a pessoa, o comprador, o clube do contrato, o valor
/// (+0x1b), +0x2f, o valor total (0x8e7a70 com 0, 1, 0, 1), o nibble baixo de +0x30 com sinal, a data da conclusão (dia
/// com sinal, ano), os da troca (até 3, da vaga 13) e quantos (12).
unsafe extern "thiscall" fn noticia_transferencia_acertada(this: usize, o: usize, data: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let p = cpx3_pessoa(cpx3_d(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0xf, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia acertada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb3, 0);
    cpx3_pessoa_na_noticia(nt, 0, p);
    gravar(nt, 4, cpx3_d(o + 8) as u32);
    gravar(nt, 5, cpx3_vaga_5(p));
    gravar(nt, 6, cpx3_d(o + 0x1b) as u32);
    gravar(nt, 7, cpx3_b(o + 0x2f) as i8 as u32);
    gravar(nt, 8, total(o, 0, 1, 0, 1));
    gravar(nt, 9, (((cpx3_b(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    gravar(nt, 0xa, cpx3_w(data) as i32 as u32);
    gravar(nt, 0xb, cpx3_w(data + 2) as u16 as u32);
    let (m, _) = cpx3_troca_na_noticia(nt, o, 0xd);
    gravar(nt, 0xc, m as i32 as u32);
    cpx3_entregar(nt, o, &lista, n);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_transferencia_acertada(this: usize, o: usize, data: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1322)(this, o, data)
}

// ---------- 260a leva: o empréstimo do arquivo de configuração, a notícia do fim do contrato e a do pedido de adiamento da transferência (transfer_manager.cpp, contract_manager.cpp) ----------

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
unsafe fn cpx4_atribuir(destino: usize, origem: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), origem);
}

/// A data `a` (dia com sinal em +0, ano em +2) vem antes da `b`.
unsafe fn cpx4_antes(a: usize, b: usize) -> bool {
    let (ya, yb) = (cpx4_w(a + 2) as u16, cpx4_w(b + 2) as u16);
    ya < yb || (ya == yb && cpx4_w(a) < cpx4_w(b))
}

/// As condições do empréstimo da configuração, na ordem da original: o time B ligado ao clube (0x540a50; vale só se não
/// for o principal), o elenco do clube abaixo de 50 (0x53c590), a pessoa sem empréstimo (0x4cf6d0), o começo até hoje,
/// o fim de hoje em diante e um contrato (0x4cf5a0). Devolve o time B (ou 0) quando passam.
unsafe fn cpx4_pode(p: usize, clube: usize, inicio: usize, fim: usize) -> Option<usize> {
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
    if cpx4_antes(0xae2c90, inicio) || cpx4_antes(fim, 0xae2c90) {
        return None;
    }
    if contrato(cpx4_d(0xae1050), p) == 0 {
        return None;
    }
    Some(b)
}

/// { 0x8bf400 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH do contrato local): `TRANSFER_MANAGER::
/// add_config_loan(pessoa, clube, começo, fim)`, pelos empréstimos do arquivo de configuração no começo do jogo. Pelas
/// condições (cpx4_pode), o empréstimo vai ao time B ligado ou ao clube: com o fim depois da data +0x46 da pessoa, a
/// data e o fim do contrato (+0x2d) passam ao fim; o contrato local (0x85e410, 0x85e590 com o fim, 0, 1, 1; começo em
/// +0x25) é assinado (0x4d01e0); as suspensões ficam guardadas (0x5563f0); o elenco 1 (0x540980); +0x6d 2 vira 0x10; os
/// números de camisa (0x85add0, 0x85a470); no time B, a pessoa vai às reservas (0x4cc0f0). Precisando de permissão de
/// trabalho no clube (0x8c7560), o fim recebido anda 7 dias e a permissão é dada (0x8c7070 com a nação do clube e o ano
/// do começo).
unsafe extern "thiscall" fn emprestimo_da_configuracao(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let mut local = [0u8; 0x50]; // o STAFF_CONTRACT local (0x50 bytes)
    let l = local.as_mut_ptr() as usize;
    construir(l);
    let b = match cpx4_pode(p, clube, inicio, fim) {
        Some(b) => b,
        None => return,
    };
    if cpx4_antes(p + 0x46, fim) {
        crate::comparacao::ramo("cpx_4 contrato estendido");
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        cpx4_atribuir(p + 0x46, fim); // cpx4: a data da pessoa até o fim
        cpx4_atribuir(contrato(cpx4_d(0xae1050), p) + 0x2d, fim); // cpx4: o fim do contrato até o fim do empréstimo
    }
    let destino = if b != 0 { b } else { clube };
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    preencher(l, p, destino, fim, 0, 1, 1);
    cpx4_atribuir(l + 0x25, inicio);
    crate::comparacao::ramo("cpx_4 emprestimo da configuracao");
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
        crate::comparacao::ramo("cpx_4 permissao de trabalho");
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u8; 8];
        let r = mais_dias(fim, t.as_mut_ptr(), 7);
        cpx4_atribuir(fim, r);
        // a original empurra cx sobre o ecx que a 0x417c10 zera: o ano limpo
        let permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
        permissao(this, p, cpx4_d(clube + 0x53), cpx4_w(inicio + 2) as u16 as u32, 1, 1); // cpx4: a permissão de trabalho
    }
}

// ponytail: assina o empréstimo, mexe no elenco e nas permissões (não pode rodar duas vezes): só a original, com o
// plano pelas mesmas condições (passando, a pessoa sai emprestada; senão o empréstimo dela fica como estava).
unsafe extern "thiscall" fn comparar_emprestimo_da_configuracao(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let tinha = emprestado(cpx4_d(0xae1050), p);
    let passa = cpx4_pode(p, clube, inicio, fim).is_some();
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(1323)(this, p, clube, inicio, fim);
    let tem = emprestado(cpx4_d(0xae1050), p);
    let igual = if passa { tem != 0 } else { tem == tinha };
    PLACAR.registrar(igual, || format!("emprestimo_da_configuracao(pessoa {:#x}) passa={} antes={:#x} depois={:#x}", cpx4_d(p), passa, tinha, tem));
}

/// O contrato principal da pessoa como a 0x4da590 o acha (o mesmo da 0x4cf5a0, em linha): sem pessoa, id fora da tabela
/// ou índice negativo no registro de 79 bytes, nenhum; guardado em [0xae1080].
unsafe fn cpx4_contrato(p: usize) -> usize {
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
unsafe fn cpx4_interessados(p: usize, l: usize) -> Vec<usize> {
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
unsafe extern "thiscall" fn noticia_fim_do_contrato(this: usize, p: usize, tipo: u32, amplo: u32) {
    let c = cpx4_contrato(p);
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
    crate::comparacao::ramo("cpx_4 noticia do fim do contrato");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbb9, 0);
    let c = cpx4_contrato(p);
    // a original lê o id no endereço 0 sem contrato: não acontece, acabou de achá-lo
    let clube = if c == 0 { 0 } else { cpx4_clube(cpx4_d(c + 4)) };
    gravar(nt, 0, cpx4_d(clube) as u32);
    gravar(nt, 2, cpx4_d(p) as u32);
    gravar(nt, 3, cpx4_d(cpx4_d(p + 4) + 0x33) as u32);
    gravar(nt, 4, cpx4_d(cpx4_d(p + 8) + 0x33) as u32);
    gravar(nt, 5, cpx4_d(cpx4_d(p + 0xc) + 0x33) as u32);
    let tipo = tipo as u8;
    gravar(nt, 6, tipo as i8 as u32);
    let c = cpx4_contrato(p);
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
            for id in cpx4_interessados(p, l) {
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
        for id in cpx4_interessados(p, l) {
            if id == cpx4_d(clube) || (tipo == 6 && id == cpx4_d(cpx4_d(p + 0x39))) {
                continue;
            }
            entregar(cpx4_d(0xb7431c), nt, cpx4_clube(id));
        }
    }
    soltar(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_fim_do_contrato(this: usize, p: usize, tipo: u32, amplo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1324)(this, p, tipo, amplo)
}

/// { 0x8dda90 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_delay_request_news(oferta, motivo)`, pelo clube que pede para adiar a transferência uma semana. Os
/// clubes humanos que recebem (0x8d1130 com 1, ou 2 fora do estado 0x11; até 30); sem nenhum, nada. A notícia 0xfab: o
/// clube do contrato (sem tipo 5) ou −1, o comprador, a pessoa (o id e as três chaves), o estado, 0, 0, a vaga +0x20 e o
/// motivo; a cada destinatário.
unsafe extern "thiscall" fn noticia_pedido_de_adiamento(this: usize, o: usize, motivo: u32) {
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
    crate::comparacao::ramo("cpx_4 noticia do adiamento");
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
unsafe extern "thiscall" fn comparar_noticia_pedido_de_adiamento(this: usize, o: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1325)(this, o, motivo)
}

// ---------- 259a leva: a aprovação final da oferta pelos clubes, o fechamento da oferta aprovada e o acerto das cláusulas das transferências anteriores (transfer_manager.cpp) ----------

unsafe fn cpw2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw2_clube(id: usize) -> usize {
    cpw2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw2_pessoa(id: usize) -> usize {
    cpw2_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
unsafe fn cpw2_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw2_d(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
unsafe fn cpw2_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// Tira a pessoa das listas de troca das outras ofertas (TRANSFER_MANAGER::remove_exchange_player 0x8c41c0 com a
/// oferta, 0, 0).
unsafe fn cpw2_tirar_da_troca(this: usize, pessoa: usize, oferta: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c41c0usize);
    f(this, pessoa, oferta, 0, 0);
}

/// Agenda a oferta (0x8c6220: thiscall(this, oferta, dias, fatias)).
unsafe fn cpw2_agendar(this: usize, oferta: usize, dias: u32, fatias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, oferta, dias, fatias);
}

/// O estado (+0x2c) da oferta depois da aprovação do `clube`: o comprador (+8) leva 0xb a 0xd e 0xc a 0xe; o outro
/// lado (ou sem clube) leva 0xb a 0xc e 0xd a 0xe. Os outros estados ficam.
unsafe fn cpw2_estado_aprovado(oferta: usize, clube: usize) -> u8 {
    let s = cpw2_b(oferta + 0x2c);
    if clube != 0 && cpw2_d(clube) == cpw2_d(oferta + 8) {
        match s {
            0xb => 0xd,
            0xc => 0xe,
            _ => s,
        }
    } else {
        match s {
            0xb => 0xc,
            0xd => 0xe,
            _ => s,
        }
    }
}

/// { 0x8c3310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// confirm_offer_by_club(oferta, clube)`, pela oferta passada aos clubes (0x8c2f50), pelas notícias e pela tela: a
/// aprovação do clube muda o estado (`cpw2_estado_aprovado`); com os dois de acordo (0xe): sem prazo (+0x30 & 0xf), a
/// oferta é agendada para hoje (0x8c6220 com 0, 1); com prazo, a data de conclusão (0x8cee70) ainda por vir faz a
/// transferência futura: uma LLIST local que não usa (0x68d020/0x68d040), o jogador (+0xc) e os da troca (lista +4)
/// saem das outras ofertas (0x8c41c0, e 0x8c47d0 para os da troca), as ofertas guardadas pelo jogador saem (0x4cf560),
/// a notícia (0x8e05d0) e a transferência futura (0x8cb9a0) com a data; data já chegada: agendada para hoje.
unsafe extern "thiscall" fn confirmar_oferta_pelo_clube(this: usize, oferta: usize, clube: usize) {
    let data_padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let mut data = [0u32; 2];
    data_padrao(&mut data);
    let s = cpw2_estado_aprovado(oferta, clube);
    *((oferta + 0x2c) as *mut u8) = s;
    if s != 0xe {
        return;
    }
    'futura: {
        if cpw2_b(oferta + 0x30) & 0xf == 0 {
            break 'futura;
        }
        let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(0x8cee70usize);
        conclusao(this, oferta, &mut data);
        let hoje = core::ptr::read_unaligned(0xae2c90 as *const [u16; 4]);
        let d = [data[0] as u16, (data[0] >> 16) as u16];
        let antes = hoje[1] < d[1] || (hoje[1] == d[1] && (hoje[0] as i16) < (d[0] as i16));
        if !antes { // cpw2: a data de conclusão ainda por vir
            break 'futura;
        }
        crate::comparacao::ramo("cpw_2 transferencia futura");
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c47d0usize);
        let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x4cf560usize);
        let noticia: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(0x8e05d0usize);
        let futura: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(0x8cb9a0usize);
        let mut lista = [0u8; 0xe];
        let l = lista.as_mut_ptr() as usize;
        lista_nova(l);
        cpw2_tirar_da_troca(this, cpw2_pessoa(cpw2_d(oferta + 0xc)), oferta);
        if cpw2_d(oferta + 4) != 0 {
            cpw2_inicio(cpw2_d(oferta + 4));
            let mut x = cpw2_proximo(cpw2_d(oferta + 4));
            while x != 0 {
                cpw2_tirar_da_troca(this, cpw2_pessoa(cpw2_d(x)), oferta);
                desfazer(this, cpw2_pessoa(cpw2_d(x)));
                x = cpw2_proximo(cpw2_d(oferta + 4));
            }
        }
        destruir(l);
        guardadas(cpw2_d(0xae1050), cpw2_d(oferta + 0xc) as u32);
        noticia(this, oferta, &mut data);
        futura(this, oferta, &mut data);
        return;
    }
    crate::comparacao::ramo("cpw_2 agendar conclusao");
    cpw2_agendar(this, oferta, 0, 1); // cpw2: a conclusão agendada para hoje
}

/// Comparador: sem chegar ao acordo dos dois (estado final diferente de 0xe), só o estado muda: lado a lado sobre a
/// oferta. Com o acordo, a oferta é agendada ou vira transferência futura (mexe nas outras ofertas e aloca): só a
/// original.
unsafe extern "thiscall" fn comparar_confirmar_oferta_pelo_clube(this: usize, oferta: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1275);
    if cpw2_estado_aprovado(oferta, clube) == 0xe {
        return o(this, oferta, clube);
    }
    let regioes = [(oferta, 0x31usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta, clube), || confirmar_oferta_pelo_clube(this, oferta, clube));
    PLACAR.registrar(igual, || format!("confirmar_oferta_pelo_clube(oferta {:#x}, clube {:#x}) original={:x?} rust={:x?}", oferta, clube, mo, mr));
}

/// { 0x8c34e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::try_complete_offer(oferta, no_prazo)`, pela
/// oferta agendada (0x8c63a0) e pelo fim da janela (0x8c4e10). O vendedor é o clube do contrato do jogador (+0xc;
/// 0x4cf5a0, guardado em [0xae1080]; tipo 5 não conta); o comprador (+8) reconfirma (0x8cfc90, que dá o motivo): fora do
/// prazo com motivo 9, não conta; comprador do computador com resposta 1: a recusa (0x8c36b0); resposta 1 ou 2: a oferta
/// se desfaz (0x8c44e0 com o motivo, 1). O mesmo com o vendedor. Os dois de acordo: o jogador e os da troca saem das
/// outras ofertas (0x8c6ae0), as ofertas guardadas pelo jogador saem (0x4cf560), a notícia (0x8dbef0); com confirmação
/// (+0x2f), vendedor e prazo do fim do contrato (+0x30 & 0xf = 3), a transferência futura no fim do contrato (0x8cb9a0
/// com +0x2d do contrato); senão a transferência se conclui (0x8c9390).
unsafe extern "thiscall" fn tentar_concluir_oferta(this: usize, oferta: usize, no_prazo: u32) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let reconfirmar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u8 = core::mem::transmute(0x8cfc90usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let recusa: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8c36b0usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c44e0usize);
    let p = cpw2_pessoa(cpw2_d(oferta + 0xc));
    let c = contrato_de(cpw2_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let vendedor = if c != 0 && cpw2_b(c + 0x35) & 0x3f != 5 { cpw2_clube(cpw2_d(c + 4)) } else { 0 };
    let comprador = cpw2_clube(cpw2_d(oferta + 8));
    // ponytail: a 0x8cfc90 grava só a palavra baixa do motivo; a original empurra o dword (a palavra alta é sobra de
    // pilha). A cópia parte de zero, um valor que o jogo também produz.
    let mut motivo: u32 = 0;
    let r = reconfirmar(this, oferta, comprador, &mut motivo, 0);
    if no_prazo == 0 || motivo as u16 != 9 {
        if humano(comprador, 0, 1, 0, 0) == 0 && r == 1 {
            return recusa(this, oferta, comprador, 0);
        }
        if r == 2 || r == 1 {
            return desfazer(this, oferta, comprador, motivo, 1);
        }
    }
    if vendedor != 0 {
        let r = reconfirmar(this, oferta, vendedor, &mut motivo, 0);
        if humano(vendedor, 0, 1, 0, 0) == 0 && r == 1 {
            return recusa(this, oferta, vendedor, 0);
        }
        if r == 2 || r == 1 {
            return desfazer(this, oferta, vendedor, motivo, 1);
        }
    }
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6ae0usize);
    let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x4cf560usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8dbef0usize);
    soltar(this, oferta);
    guardadas(cpw2_d(0xae1050), cpw2_d(oferta + 0xc) as u32);
    noticia(this, oferta);
    if cpw2_b(oferta + 0x2f) != 0 && vendedor != 0 && cpw2_b(oferta + 0x30) & 0xf == 3 {
        let c = contrato_de(cpw2_d(0xae1050), p);
        if c != 0 {
            crate::comparacao::ramo("cpw_2 futura no fim do contrato");
            let futura: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8cb9a0usize);
            return futura(this, oferta, c + 0x2d);
        }
    }
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c9390usize);
    crate::comparacao::ramo("cpw_2 concluir agora");
    concluir(this, oferta); // cpw2: a transferência se conclui
}

// ponytail: conclui transferências, desfaz ofertas e dá notícias (aloca, não roda duas vezes): só a original; a cópia
// leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_tentar_concluir_oferta(this: usize, oferta: usize, no_prazo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1276)(this, oferta, no_prazo)
}

/// As cláusulas antigas de uma pessoa pelas marcas do registro de 8 bytes em [this+0x86e]: o percentual da próxima
/// venda (bit 8: 0x8cb0f0 paga, 0x8cb4f0 tira, este da pessoa `tirar`), o prêmio por jogo no time (bit 2: 0x8cb340 com
/// −1, 2 e tira 0x8cb590) e o pela seleção (bit 4: 0x8cb340 com −1, 5 e tira 0x8cb630).
unsafe fn cpw2_clausulas(this: usize, pessoa: usize, tirar: usize, oferta: usize) {
    let marca = |p: usize| cpw2_b(cpw2_d(this + 0x86e).wrapping_add(cpw2_d(p).wrapping_mul(8)));
    let pagar_venda: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8cb0f0usize);
    let tirar_venda: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb4f0usize);
    let pagar_jogos: unsafe extern "thiscall" fn(usize, usize, i32, u32, usize) -> u32 = core::mem::transmute(0x8cb340usize);
    let tirar_jogos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    if marca(pessoa) & 8 != 0 {
        crate::comparacao::ramo("cpw_2 percentual da venda anterior");
        pagar_venda(this, pessoa, oferta);
        tirar_venda(this, tirar);
    }
    if marca(pessoa) & 2 != 0 {
        crate::comparacao::ramo("cpw_2 premio por jogo anterior");
        pagar_jogos(this, pessoa, -1, 2, oferta);
        tirar_jogos(this, pessoa);
    }
    if marca(pessoa) & 4 != 0 {
        pagar_jogos(this, pessoa, -1, 5, oferta);
        tirar_selecao(this, pessoa);
    }
}

/// { 0x8c91e0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::settle_previous_transfer_clauses(oferta)`,
/// pela transferência concluída (0x8c9390): as cláusulas antigas do jogador (+0xc) e das pessoas da troca (lista +4,
/// cursor recomeçado; a original tira o percentual da próxima venda do jogador da oferta, não o da pessoa da troca).
unsafe extern "thiscall" fn acertar_clausulas_anteriores(this: usize, oferta: usize) {
    let p = cpw2_pessoa(cpw2_d(oferta + 0xc));
    cpw2_clausulas(this, p, p, oferta);
    let l = cpw2_d(oferta + 4);
    if l == 0 {
        return;
    }
    cpw2_inicio(l);
    let mut x = cpw2_proximo(cpw2_d(oferta + 4));
    while x != 0 {
        cpw2_clausulas(this, cpw2_pessoa(cpw2_d(x)), p, oferta);
        x = cpw2_proximo(cpw2_d(oferta + 4));
    }
}

/// Comparador: sem marca de cláusula (bits 2, 4 e 8) no jogador nem nas pessoas da troca, só o cursor da lista muda:
/// lado a lado sobre a lista e as marcas. Com alguma, paga (finanças, notícias que alocam): só a original.
unsafe extern "thiscall" fn comparar_acertar_clausulas_anteriores(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1277);
    let marcas = cpw2_d(this + 0x86e);
    let tem = |id: usize| cpw2_b(marcas.wrapping_add(id.wrapping_mul(8))) & 0xe != 0;
    let l = cpw2_d(oferta + 4);
    let mut regioes = vec![(marcas.wrapping_add(cpw2_d(oferta + 0xc).wrapping_mul(8)), 8usize)];
    let mut alguma = tem(cpw2_d(oferta + 0xc));
    if l != 0 {
        regioes.push((l, 0xe));
        // o percurso sem mexer no cursor da lista: os nós (+0 o item, +6 o próximo) a partir da cabeça +2
        let mut no = cpw2_d(l + 2);
        let mut n = 0;
        while no != 0 && n < 64 {
            let item = cpw2_d(no);
            if item != 0 && tem(cpw2_d(item)) {
                alguma = true;
            }
            no = cpw2_d(no + 6);
            n += 1;
        }
        if n >= 64 {
            alguma = true;
        }
    }
    if alguma {
        return o(this, oferta);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || acertar_clausulas_anteriores(this, oferta));
    PLACAR.registrar(igual, || format!("acertar_clausulas_anteriores(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

// ---------- 258a leva: a oferta passada aos clubes, a notícia da oferta aceita e a do empréstimo oferecido (transfer_manager.cpp) ----------

unsafe fn cpv1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpv1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpv1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpv1_clube(id: usize) -> usize {
    cpv1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpv1_pessoa(id: usize) -> usize {
    cpv1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpv1_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpv1_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
unsafe fn cpv1_clube_do_contrato(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || cpv1_b(c + 0x35) & 0x3f != 5) {
        cpv1_clube(cpv1_d(c + 4))
    } else {
        0
    }
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
unsafe fn cpv1_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpv1_d(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
unsafe fn cpv1_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(l)
}

/// Os dados da pessoa numa notícia, a partir da vaga `k`: o id e as três chaves (+0x33 de +4, +8 e +0xc).
unsafe fn cpv1_dados_da_pessoa(n: usize, k: i8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, k, cpv1_d(p) as u32);
    gravar(n, k + 1, cpv1_d(cpv1_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cpv1_d(cpv1_d(p + 8) + 0x33) as u32);
    gravar(n, k + 3, cpv1_d(cpv1_d(p + 0xc) + 0x33) as u32);
}

/// { 0x8c2f50 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::advance_offer_to_club_approval(oferta)`:
/// comprador = o clube +8; vendedor = o clube do contrato do jogador (+0xc), nenhum no contrato de tipo 5 (0x4cf5a0
/// duas vezes, [0xae1080]). Fora do status 0xa (+0x2c): só com +0x10 = 2 e todos os jogadores da troca (+4) com +4 = 2.
/// As outras ofertas pelo mesmo jogador (lista +0, índices no vetor [this]; menos a +0x20) caem: status 0xf, +0x2e = 3,
/// reagendadas em 0x14 dias (0x8c6220), e as marcas de lance (0x84aeb0 no gerente de listas [this+0x856]) do comprador
/// e do dono de cada jogador da troca saem. Sem a marca +0x30 & 0xf, com permissão de trabalho a pedir (0x8c7470) e
/// +0x2e = 0: a notícia da espera (0x8e2a00), status 0xa e reagendada em 18 + sorteio(7) dias. Senão status 0xb, a
/// notícia da transferência (0x8da5e0) se um dos clubes é humano, e a decisão do comprador do computador (0x600a50):
/// com restrição (0x8cfc90) recusa (0x8c36b0, +0x2e < 3) ou retira (0x8bfcb0, que pode trocar a oferta); senão
/// confirma (0x8c3310). Depois o vendedor: com +0x2f ou sem vendedor confirma direto; do computador, recusa com
/// restrição ou confirma.
unsafe extern "thiscall" fn passar_a_oferta_aos_clubes(this: usize, oferta: usize) {
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    // thiscall (ecx = this, o gerente de transferências; corrigido em 31/10/2026: antes stdcall, o this ia com o ecx da vez e às vezes 0)
    let pede_permissao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c7470usize);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8e2a00usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_da_transferencia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8da5e0usize);
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(0x8cfc90usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8c36b0usize);
    let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
    let confirmar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8c3310usize);
    let comprador = cpv1_clube(cpv1_d(oferta + 8));
    let jogador = cpv1_pessoa(cpv1_d(oferta + 0xc));
    let mut vendedor = cpv1_clube_do_contrato(cpv1_contrato(jogador), true);
    let c = cpv1_contrato(jogador);
    if c != 0 && cpv1_b(c + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    if cpv1_b(oferta + 0x2c) != 0xa {
        if cpv1_b(oferta + 0x10) != 2 {
            return;
        }
        let troca = cpv1_d(oferta + 4);
        if troca != 0 {
            cpv1_inicio(troca);
            loop {
                let x = cpv1_proximo(troca);
                if x == 0 {
                    break;
                }
                if cpv1_b(x + 4) != 2 {
                    return;
                }
            }
        }
        let rivais = cpv1_d(oferta);
        if rivais != 0 {
            cpv1_inicio(rivais);
            let mut x = cpv1_proximo(rivais);
            while x != 0 {
                let outra = cpv1_d(cpv1_d(this) + cpv1_d(x) * 4);
                if outra != 0 && cpv1_d(x) != cpv1_d(oferta + 0x20) {
                    crate::comparacao::ramo("cpv_1 ofertas rivais canceladas");
                    cpv1_pb(outra + 0x2c, 0xf);
                    cpv1_pb(cpv1_d(cpv1_d(this) + cpv1_d(x) * 4) + 0x2e, 3);
                    agendar(this, cpv1_d(cpv1_d(this) + cpv1_d(x) * 4), 0x14, 0); // cpv1: as rivais em 20 dias
                    let o = cpv1_d(cpv1_d(this) + cpv1_d(x) * 4);
                    let p = cpv1_pessoa(cpv1_d(o + 0xc));
                    let dono = cpv1_clube_do_contrato(cpv1_contrato(p), false);
                    limpar_marcas(cpv1_d(this + 0x856), cpv1_clube(cpv1_d(o + 8)), p);
                    let l = cpv1_d(o + 4);
                    if l != 0 {
                        cpv1_inicio(l);
                        loop {
                            let y = cpv1_proximo(l);
                            if y == 0 {
                                break;
                            }
                            limpar_marcas(cpv1_d(this + 0x856), dono, cpv1_pessoa(cpv1_d(y)));
                        }
                    }
                }
                x = cpv1_proximo(rivais);
            }
        }
        if cpv1_b(oferta + 0x30) & 0xf == 0 && pede_permissao(this, oferta) != 0 && cpv1_b(oferta + 0x2e) == 0 {
            noticia_da_espera(this, oferta);
            cpv1_pb(oferta + 0x2c, 0xa);
            let r = sorteio(7);
            crate::comparacao::ramo("cpv_1 permissao de trabalho");
            // a original soma 0x12 só no byte baixo do eax do sorteio (0..6: em cima, zero)
            let dias = (r as u32 & !0xff) | (r as u8).wrapping_add(0x12) as u32; // cpv1: a espera da permissao
            agendar(this, oferta, dias, 0);
            return;
        }
    }
    cpv1_pb(oferta + 0x2c, 0xb);
    if humano(cpv1_d(0xb7431c), comprador) != 0 || (vendedor != 0 && humano(cpv1_d(0xb7431c), vendedor) != 0) {
        noticia_da_transferencia(this, oferta);
    }
    let mut vaga = oferta;
    // ponytail: a original passa um short* para um dword local sem valor e empurra o dword inteiro; os chamados leem a
    // palavra. A cópia começa em zero.
    let mut motivo: u32 = 0;
    if clube_humano(comprador, 0, 1, 0, 0) == 0 {
        if restricoes(this, oferta, comprador, &mut motivo, 1) as u8 == 1 {
            if cpv1_b(oferta + 0x2e) < 3 {
                recusar(this, oferta, comprador, motivo);
            } else {
                retirar(this, &mut vaga, 0, motivo);
            }
        } else {
            confirmar(this, oferta, comprador);
        }
    }
    // relida da vaga, como a original ([esp+0x20], que a 0x8bfcb0 pode trocar)
    let o = vaga;
    if cpv1_b(o + 0x2f) != 0 || vendedor == 0 {
        return confirmar(this, o, vendedor);
    }
    if clube_humano(vendedor, 0, 1, 0, 0) != 0 {
        return;
    }
    if restricoes(this, vaga, vendedor, &mut motivo, 1) as u8 == 1 {
        return recusar(this, vaga, vendedor, 0);
    }
    crate::comparacao::ramo("cpv_1 confirmada pelo vendedor");
    confirmar(this, vaga, vendedor); // cpv1: o vendedor confirma
}

/// { 0x8d76d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_offer_accepted_news(oferta)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os destinatários
/// (0x8d1130 com 0x2d, até 30), a notícia 0xfa3 (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 = +8 (comprador),
/// 6 = +4, 7 = +0x20, 8 = +0x24, 9 = +0x1f; entregue a cada destinatário (0x7861e0).
unsafe extern "thiscall" fn noticia_da_oferta_aceita(this: usize, oferta: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpv_1 noticia da oferta aceita");
    let p = cpv1_pessoa(cpv1_d(oferta + 0xc));
    let dono = cpv1_clube_do_contrato(cpv1_contrato(p), true);
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2d, clubes.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfa3, 0);
    cpv1_dados_da_pessoa(n, 0, p);
    gravar(n, 4, if dono != 0 { cpv1_d(dono) as u32 } else { u32::MAX });
    gravar(n, 5, cpv1_d(oferta + 8) as u32);
    gravar(n, 6, cpv1_d(oferta + 4) as u32);
    gravar(n, 7, cpv1_d(oferta + 0x20) as u32);
    gravar(n, 8, cpv1_d(oferta + 0x24) as u32);
    gravar(n, 9, cpv1_b(oferta + 0x1f) as i8 as i32 as u32);
    for &c in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv1_d(0xb7431c), n, c);
    }
}

/// { 0x8e1780 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_loan_offer_news(oferta, termos)`: os destinatários (0x8d1130 com 0x2e, até 30); sem nenhum, nada. A notícia
/// 0xfb5 (local): 0..3 o jogador (+0xc), 4 = +8, 5 = o id do clube do contrato (nenhum no tipo 5: a original lê o
/// endereço 0), 6 = +0x20, 7 = +0x24, 8 = termos +0xc, 9 = +0x2c, 10 = termos +0x2d (palavra com sinal), 11 = termos
/// +0x2f (palavra); sem contrato, a caixa "Error" (0x5e4c) e nada; senão 12 = contrato +0xc e a entrega a cada um.
unsafe extern "thiscall" fn noticia_do_emprestimo_oferecido(this: usize, oferta: usize, termos: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpv1_pessoa(cpv1_d(oferta + 0xc));
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2e, clubes.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    crate::comparacao::ramo("cpv_1 noticia do emprestimo");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfb5, 0);
    cpv1_dados_da_pessoa(n, 0, p);
    gravar(n, 4, cpv1_d(oferta + 8) as u32);
    // ponytail: sem clube (contrato de tipo 5) a original lê o endereço 0 e cai; a cópia faz o mesmo.
    let clube = cpv1_clube_do_contrato(cpv1_contrato(p), true);
    gravar(n, 5, cpv1_d(clube) as u32);
    gravar(n, 6, cpv1_d(oferta + 0x20) as u32);
    gravar(n, 7, cpv1_d(oferta + 0x24) as u32);
    gravar(n, 8, cpv1_d(termos + 0xc) as u32);
    gravar(n, 9, cpv1_b(oferta + 0x2c) as i8 as i32 as u32);
    gravar(n, 10, core::ptr::read_unaligned((termos + 0x2d) as *const i16) as i32 as u32);
    gravar(n, 11, core::ptr::read_unaligned((termos + 0x2f) as *const u16) as u32);
    let c = cpv1_contrato(p);
    if c == 0 {
        return aviso_em(0xa8d514, 0x5e4c);
    }
    gravar(n, 12, cpv1_d(c + 0xc) as u32);
    for &d in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(cpv1_d(0xb7431c), n, d);
    }
}

// ponytail: reagenda, cancela rivais, sorteia e decide pelos clubes (as chamadas mexem nas ofertas e alocam): não roda
// duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_passar_a_oferta_aos_clubes(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1233)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_oferta_aceita(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1234)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_emprestimo_oferecido(this: usize, oferta: usize, termos: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1235)(this, oferta, termos)
}

// ---------- 258a leva: a negociação guardada da oferta, o boato do técnico em outro clube e a recusa do jogador em se mudar (transfer_manager.cpp, manager_manager.cpp) ----------

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
unsafe extern "thiscall" fn guardar_a_negociacao(this: usize, oferta: usize, termos: usize, marcar: u32) {
    let marcar_sob_oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x84b580usize);
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
        marcar_sob_oferta(cpv4_d(this + 0x856), cpv4_pessoa(cpv4_d(oferta + 0xc)), cpv4_clube(cpv4_d(oferta + 8)), cpv4_d(oferta));
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
            None => return aviso_em(0xa8d514, 0x60f),
        }
        crate::comparacao::ramo("cpv_4 vaga nova");
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
        crate::comparacao::ramo("cpv_4 negociacao agendada");
        agendar(this, oferta, 2, 0); // cpv4: a oferta em 2 dias
    }
}

/// { 0x69fd50 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_job_rumour_news(pessoa, clube)`: sem pessoa ou sem clube dela (+0x39), nada; um sorteio(4) = 0, nada; antes da
/// data das notícias ([0xdd6ec8]), nada. A notícia 0x1776 (local): 0..3 a pessoa, 4 = o cargo (+0x3d), 5 = o id do
/// clube dela, 6 = o id do clube do boato, 7 = sorteio(4); aos destinatários (0x8d1da0 do gerente de transferências
/// [0xde1f54], com 0x52, até 30; sem gerente, nenhum) pela entrega ao técnico (0x7861e0).
unsafe extern "thiscall" fn noticia_do_boato_do_tecnico(_this: usize, pessoa: usize, clube: usize) {
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
    crate::comparacao::ramo("cpv_4 boato do tecnico");
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
unsafe extern "thiscall" fn noticia_da_recusa_em_se_mudar(this: usize, oferta: usize, motivo: u32) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(0x8d1130usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparacao::ramo("cpv_4 recusa em se mudar");
    let p = cpv4_pessoa(cpv4_d(oferta + 0xc));
    let c = contrato_de(cpv4_d(0xae1050), p);
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
unsafe extern "thiscall" fn comparar_guardar_a_negociacao(this: usize, oferta: usize, termos: usize, marcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1242)(this, oferta, termos, marcar)
}

// ponytail: sorteia e a notícia vai às caixas (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_do_boato_do_tecnico(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1243)(this, pessoa, clube)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_da_recusa_em_se_mudar(this: usize, oferta: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1244)(this, oferta, motivo)
}

// ---------- 255a leva: as permissões de trabalho do começo do jogo, a contagem da cláusula e o prêmio por jogo do jogador (transfer_manager.cpp) ----------

unsafe fn cps2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x8c6e60 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH só da data local): `TRANSFER_MANAGER::
/// init_work_permits()`: pela criação do jogo (0x8bba30). Para cada pessoa: com mais de 13 jogos pela seleção (+0x22),
/// 0x3fff na palavra +3 do registro de 8 bytes ([this+0x86e] + i × 8); com mais de 5, 0x1fff. Jogador (+0x3d = 0xb) de
/// clube (+0x39) que precisa de permissão de trabalho (0x8c7560 sem ignorar a dada): o ano mais antigo na nação do
/// clube pela carreira (0x7c3e40, um grupo de 6 entradas de 0x2b bytes por temporada; para no primeiro clube de outra
/// nação), começando pelo ano de +0x40. Menos de 5 anos até o ano de hoje ([0xae2c92]): a permissão (0x8c7070, sem
/// ordenar; antes a original leva a data +0x3e de 52 em 52 semanas até a data [0xdd6ec8], sem usar o resultado); senão a
/// segunda nacionalidade (+0x1e) vira a nação do clube. No fim ordena as permissões (qsort 0x945333 com 0x8e55c0).
unsafe extern "thiscall" fn iniciar_permissoes_de_trabalho(this: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(0x54c770usize);
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let carreira: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let mut d = [0u8; 8];
    data_padrao(d.as_mut_ptr() as usize);
    let mut i = 0usize;
    let mut p = cps2_d(0xae23bc);
    while (i as i32) < cps2_d(0xae2364) as i32 {
        let jogos = cps2_b(p + 0x22);
        let marca = (cps2_d(this + 0x86e).wrapping_add(i.wrapping_mul(8)) + 3) as *mut u16;
        if jogos > 0xd {
            core::ptr::write_unaligned(marca, 0x3fff);
        } else if jogos > 5 {
            core::ptr::write_unaligned(marca, 0x1fff);
        }
        let clube = cps2_d(p + 0x39);
        if clube != 0 && cps2_b(p + 0x3d) == 0xb && precisa(this, p, clube, 0) != 0 {
            let mut ano = cps2_w(p + 0x40);
            let mut conta = 0i32;
            let t = carreira(cps2_d(0xb74350), p, &mut conta);
            let (mut k, mut e) = (0i32, t + 2);
            while k < conta {
                let id = cps2_d(e) as i32;
                if id >= 0 && id < cps2_d(0xae235c) as i32 {
                    let c = cps2_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245));
                    if cps2_d(c + 0x53) != cps2_d(cps2_d(p + 0x39) + 0x53) {
                        break; // clube de outra nação
                    }
                    let a = cps2_w(e - 2);
                    if a < ano { // cps2: o ano mais antigo
                        crate::comparacao::ramo("cps_2 ano mais antigo");
                        ano = a;
                    }
                }
                k += 6;
                e += 0x102;
            }
            if t != 0 {
                liberar(t);
            }
            if (cps2_w(0xae2c92) as i32) - (ano as i32) < 5 { // cps2: menos de cinco anos na nação
                // a data +0x3e levada de 52 em 52 semanas até a data [0xdd6ec8] (a original não usa o resultado)
                let (mut t1, mut t2, mut t3) = ([0u8; 8], [0u8; 8], [0u8; 8]);
                let dp = d.as_mut_ptr() as usize;
                copiar(dp, t1.as_mut_ptr() as usize, p + 0x3e);
                loop {
                    let (ah, dh) = (cps2_w(0xdd6eca), cps2_w(dp + 2));
                    if !(ah > dh || (ah == dh && (cps2_w(0xdd6ec8) as i16) > (cps2_w(dp) as i16))) {
                        break;
                    }
                    let r = mais_semanas(dp, t2.as_mut_ptr() as usize, 0x34);
                    copiar(dp, t3.as_mut_ptr() as usize, r);
                }
                crate::comparacao::ramo("cps_2 permissao pela carreira");
                // ponytail: a original empurra o registrador do ano com os 16 bits altos de um ponteiro; a 0x8c7070
                // só grava a palavra baixa
                dar(this, p, cps2_d(cps2_d(p + 0x39) + 0x53), ano as u32, 1, 0);
            } else {
                crate::comparacao::ramo("cps_2 segunda nacionalidade");
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, cps2_d(cps2_d(p + 0x39) + 0x53)); // cps2: a segunda nacionalidade
            }
        }
        i += 1;
        p += 0x6e;
    }
    qsort(cps2_d(this + 0x93a), cps2_d(this + 0x93e) as u32, 0xe, 0x8e55c0);
}

/// { 0x8c3e50 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_clause_count(pessoa, tipo)`: no tipo 4,
/// os jogos pela seleção (+0x22) menos os da época do acordo (byte +7 do registro de 8 bytes [this+0x86e] + id × 8);
/// senão os jogos pelo clube (+0x3b) do contrato (0x4cf5a0, pedido duas vezes como na original), ou 0 sem contrato.
unsafe extern "thiscall" fn contagem_da_clausula(this: usize, pessoa: usize, tipo: u32) -> u16 {
    if tipo as u8 == 4 {
        let r = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8));
        return (cps2_b(pessoa + 0x22) as u16).wrapping_sub(cps2_b(r + 7) as u16);
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    if contrato_de(cps2_d(0xae1050), pessoa) == 0 {
        return 0;
    }
    cps2_w(contrato_de(cps2_d(0xae1050), pessoa) + 0x3b)
}

/// { 0x8d0ea0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_first_team_appearance_money(pessoa)`:
/// com o bit 2 do registro de 8 bytes ([this+0x86e] + id × 8), busca o prêmio por jogo da pessoa no vetor [this+0x88c]
/// (conta +0x894; busca binária 0x946464 com 0x8b4e00, registros de 13 bytes) e o devolve; sem achar, tira o bit (soma
/// 0xfe ao byte) e devolve 0.
unsafe extern "thiscall" fn premio_por_jogo_do_jogador(this: usize, pessoa: usize) -> usize {
    let id = cps2_d(pessoa);
    if cps2_b(cps2_d(this + 0x86e).wrapping_add(id.wrapping_mul(8))) & 2 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = id as i32;
    let r = bsearch(&chave, cps2_d(this + 0x88c), cps2_d(this + 0x894), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    let m = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8)) as *mut u8;
    *m = (*m).wrapping_add(0xfe);
    0
}

// ponytail: cresce o vetor das permissões (0x8c7070) e grava em todas as pessoas: só a original; a cópia leva os ramos
// (roda uma vez, na criação do jogo).
unsafe extern "thiscall" fn comparar_iniciar_permissoes_de_trabalho(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1119)(this)
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_contagem_da_clausula(this: usize, pessoa: usize, tipo: u32) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(1120)(this, pessoa, tipo);
    let n = contagem_da_clausula(this, pessoa, tipo);
    PLACAR.registrar(r == n, || format!("contagem_da_clausula(pessoa {}, tipo {}) original={} rust={}", cps2_d(pessoa), tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelo byte do registro da pessoa, e pela devolução.
unsafe extern "thiscall" fn comparar_premio_por_jogo_do_jogador(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1121);
    let m = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, pessoa), || premio_por_jogo_do_jogador(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("premio_por_jogo_do_jogador(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", cps2_d(pessoa), r, n, mo, mr));
    r
}

// ---------- 255a leva: a fila da transferência livre, a próxima janela aberta e o empréstimo negociado (transfer_manager.cpp) ----------

unsafe fn cps3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// { 0x8c18c0 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH das duas LLIST locais 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::process_free_transfer_queue()`: pelo dia a dia das transferências (0x8c5880). Percorre uma cópia
/// do cabeçalho da fila (+0x92c, 14 bytes, com o cursor no começo); para cada item (+0 id da pessoa, +4 motivo, +5
/// compensação) com contrato (0x4cf5a0, guardado em [0xae1080]) e clube: no clube humano (0x7875e0) sem motivo, a
/// diretoria pode barrar (0x696940; barrando, 0x6a0990 e o próximo); senão o clube entra nos rejeitados (0x547e70) e o
/// técnico (+0xcf) nos desafetos (0x547af0), e a diretoria registra (0x697430). Sem motivo, um sorteio pelo +0x5b contra
/// outro de 20 (ou gostar do clube, 0x547d60) põe o técnico nos desafetos. Depois os registros da liberação (0x86d800),
/// as ofertas canceladas (0x8df9b0) e a liberação (0x8c1af0 com motivo e compensação). No fim a cópia vira a lista
/// vazia, a fila é limpa (0x68d080) e as duas locais são destruídas.
unsafe extern "thiscall" fn fila_da_transferencia_livre(this: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let barra: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x696940usize);
    let barrado: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6a0990usize);
    let rejeitado: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
    let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let diretoria: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x697430usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let registros: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86d800usize);
    let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8df9b0usize);
    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, i32, usize, u32) = core::mem::transmute(0x8c1af0usize);
    let (mut l1, mut l2) = ([0u8; 14], [0u8; 14]);
    let (a, b) = (l1.as_mut_ptr() as usize, l2.as_mut_ptr() as usize);
    nova(a);
    nova(b);
    let fila = this + 0x92c;
    // a cópia do cabeçalho: os 10 primeiros bytes e o cursor (+0xa) no começo (+2)
    core::ptr::copy_nonoverlapping(fila as *const u8, a as *mut u8, 0xa);
    cps3_por(a + 0xa, cps3_d(a + 2));
    let mut e = proximo(a);
    while e != 0 {
        let p = cps3_d(0xae23bc).wrapping_add(cps3_d(e).wrapping_mul(0x6e));
        let c = contrato_de(cps3_d(0xae1050), p);
        cps3_por(0xae1080, c);
        let clube = if c != 0 { cps3_d(0xae23b4).wrapping_add(cps3_d(c + 4).wrapping_mul(0x245)) } else { 0 };
        if clube != 0 {
            let mut seguir = true;
            if humano(cps3_d(0xb7431c), clube) != 0 && cps3_b(e + 4) == 0 {
                let r = barra(cps3_d(0xb67a44), p, clube);
                if r as u16 != 0 {
                    barrado(cps3_d(0xb67a44), p, clube, r);
                    seguir = false;
                } else {
                    if cps3_b(e + 4) == 0 {
                        rejeitado(p, clube, 0);
                        if cps3_d(clube + 0xcf) != 0 {
                            desafeto(p, cps3_d(clube + 0xcf), 1);
                        }
                    }
                    diretoria(cps3_d(0xb67a44), p, clube);
                }
            }
            if seguir {
                if cps3_b(e + 4) == 0 {
                    let r = sorteio(cps3_b(p + 0x5b) as i8 as i32);
                    if r < sorteio(0x14) || gosta(p, clube) != 0 { // cps3: o sorteio do desafeto
                        crate::comparacao::ramo("cps_3 desafeto do tecnico");
                        desafeto(p, cps3_d(clube + 0xcf), 1);
                    }
                }
                crate::comparacao::ramo("cps_3 liberado");
                registros(cps3_d(0xdd7f00), p); // cps3: os registros da liberação
                cancelar(this, p);
                let comp = cps3_d(e + 5); // cps3: a compensação ao clube
                if comp != 0 {
                    crate::comparacao::ramo("cps_3 compensacao");
                }
                liberar(this, p, clube, cps3_b(e + 4) as i8 as i32, comp, 0);
            }
        }
        e = proximo(a);
    }
    // a cópia vira a lista vazia (os 14 bytes), a fila é limpa e as duas locais saem
    core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 14);
    limpar(fila);
    soltar(b);
    soltar(a);
}

/// { 0x8cf470 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só das datas locais): `TRANSFER_MANAGER::
/// find_next_window_open_date(clube, saida)`: a regra da nação do clube (+0x53) entre as 28 de +0x8ac (+0xa); de hoje
/// ([0xae2c90]) até um ano depois (0x54c5b0 com 365), o primeiro dia com a janela aberta (0x8e8e50; sem regra, hoje) vai
/// para `saida`; sem achar, a caixa "Error" 0x32c6. A contratação desejada (0x85fc50) regrava a data com hoje logo depois.
unsafe extern "thiscall" fn proxima_janela_aberta(this: usize, clube: usize, saida: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let aberta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8e8e50usize);
    let (mut d, mut lim, mut t1, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (dp, lp, p1, p2) = (d.as_mut_ptr() as usize, lim.as_mut_ptr() as usize, t1.as_mut_ptr() as usize, t2.as_mut_ptr() as usize);
    data_padrao(dp);
    mais(0xae2c90, lp, 0x16d);
    let nacao = cps3_d(clube + 0x53);
    let mut regra = 0usize;
    for k in 0..28 {
        let r = cps3_d(this + 0x8ac + k * 4);
        if cps3_d(r + 0xa) == nacao {
            regra = r;
            break;
        }
    }
    copiar(dp, p1, 0xae2c90);
    loop {
        let (la, da) = (cps3_w(lp + 2), cps3_w(dp + 2));
        if !(la > da || (la == da && (cps3_w(lp) as i16) > (cps3_w(dp) as i16))) {
            return aviso_em(0xa8d514, 0x32c6);
        }
        if regra == 0 || aberta(regra, dp, 0) != 0 {
            copiar(saida, p2, dp);
            return;
        }
        let r = mais(dp, p1, 1);
        copiar(dp, p2, r);
    }
}

/// { 0x8c72d0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_negotiated_loan_contract(pessoa,
/// clube)`: na lista [this+0x946] (o cursor +0xa volta ao começo), o item com a pessoa (+0) e o clube (+4) pelos ids;
/// sem lista ou sem achar, 0.
unsafe extern "thiscall" fn emprestimo_negociado(this: usize, pessoa: usize, clube: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cps3_d(this + 0x946);
    if l == 0 {
        return 0;
    }
    cps3_por(l + 0xa, cps3_d(l + 2));
    let mut e = proximo(cps3_d(this + 0x946));
    while e != 0 {
        if cps3_d(pessoa) == cps3_d(e) && cps3_d(clube) == cps3_d(e + 4) {
            return e;
        }
        e = proximo(cps3_d(this + 0x946));
    }
    0
}

// ponytail: sorteia (0x90cfc0) e libera os jogadores (0x8c1af0, 0x8df9b0), limpando a fila: só a original; a cópia
// leva os ramos.
unsafe extern "thiscall" fn comparar_fila_da_transferencia_livre(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1122)(this)
}

/// Comparador: lado a lado pela data de saída (a data estática da 0x54c5b0 é rascunho). Sem janela num ano, a caixa
/// sairia duas vezes (não acontece com as regras do jogo).
unsafe extern "thiscall" fn comparar_proxima_janela_aberta(this: usize, clube: usize, saida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1123);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 8)], || o(this, clube, saida), || proxima_janela_aberta(this, clube, saida));
    PLACAR.registrar(igual, || format!("proxima_janela_aberta(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparador: pelo valor (as duas voltas deixam o cursor da lista no mesmo lugar).
unsafe extern "thiscall" fn comparar_emprestimo_negociado(this: usize, pessoa: usize, clube: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(1124)(this, pessoa, clube);
    let n = emprestimo_negociado(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("emprestimo_negociado(pessoa {}, clube {}) original={:x} rust={:x}", cps3_d(pessoa), cps3_d(clube), r, n));
    r
}

// ---------- 255a leva: a contraproposta com cláusula e os motivos da recusa do lance (transfer_manager.cpp) ----------

unsafe fn cps4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O valor da oferta (+0x1b) × `f`, truncado (_ftol 0x945640; o x87 em precisão dupla).
unsafe fn cps4_vezes(o: usize, f: f64) -> usize {
    ftol(cps4_d(o + 0x1b) as i32 as f64 * f) as usize
}

/// O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]), ou 0.
unsafe fn cps4_clube_atual(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cps4_d(0xae1050), pessoa);
    cps4_por(0xae1080, c);
    if c == 0 {
        0
    } else {
        cps4_d(0xae23b4).wrapping_add(cps4_d(c + 4).wrapping_mul(0x245))
    }
}

/// { 0x8c8fd0 } (thiscall com o `this` sem uso, ret 0x14, transfer_manager.cpp, quadro SEH do `new`):
/// `TRANSFER_MANAGER::build_clause_offer_option(clube, jogador, saida, apelo, valor)`: pela resposta do clube do computador
/// ao lance (0x8c8090). Com o clube do lance mais de 3 acima na reputação (+0x80) do clube atual (0x4cf5a0), ficha de
/// jogador (+0x61) com potencial (+7) ao menos 25 acima da habilidade (+5) e até 30 anos (+0x18): uma oferta nova
/// (0x944cf1 de 0x41 bytes, 0x8e5680) em `*saida`, iniciada com o valor (0x8e78f0), e apelo 50. Clube do lance abaixo de
/// 15 com a reputação da ficha (+0xb / 500) mais de 2 acima: 20% da próxima venda (+0x3f), o valor × 0,8 e apelo 80.
/// Senão, jogador sem jogos pela seleção (+0x22) de nação (+0x1a) acima de 5000 (+0x8e): o prêmio por jogo na seleção
/// (+0x3a, valor × 0,3, por 10 jogos +0x3e), o valor × 0,8 e apelo 70. Senão, reputação da ficha mais de 3 abaixo da do
/// clube: o prêmio por jogo no time (+0x35, valor × 0,3, por 20 jogos +0x39), o valor × 0,8 e apelo 60. Nenhum: a
/// oferta é destruída (0x8e5710, 0x944c94; `*saida` fica com o ponteiro, como na original) e 0.
unsafe extern "thiscall" fn oferta_com_clausula(_this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    let atual = cps4_clube_atual(jogador);
    if (cps4_w(clube + 0x80) as i16 as i32) - (cps4_w(atual + 0x80) as i16 as i32) <= 3 {
        return 0;
    }
    let ficha = cps4_d(jogador + 0x61);
    if ficha == 0 || (cps4_w(ficha + 7) as i16 as i32) - (cps4_w(ficha + 5) as i16 as i32) < 0x19 || cps4_b(jogador + 0x18) as i8 > 0x1e {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8e78f0usize);
    let m = novo(0x41);
    let o = if m != 0 { construir(m) } else { 0 };
    *saida = o;
    iniciar(o, jogador, clube, valor);
    *apelo = 0x32;
    let rep = cps4_w(clube + 0x80) as i16;
    // a reputação da ficha (+0xb) / 500, com o sinal (a multiplicação mágica da original)
    let fama = || (cps4_w(cps4_d(jogador + 0x61) + 0xb) as i16 as i32) / 500;
    if rep < 0xf && fama() - (rep as i32) > 2 {
        crate::comparacao::ramo("cps_4 percentual da venda");
        let o = *saida;
        cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do percentual
        *((*saida + 0x3f) as *mut u8) = 0x14;
        *apelo = 0x50;
        return 1;
    }
    let nacao = cps4_d(jogador + 0x1a);
    if nacao != 0 && cps4_w(nacao + 0x8e) as i16 > 0x1388 && cps4_b(jogador + 0x22) == 0 {
        crate::comparacao::ramo("cps_4 premio na selecao");
        let o = *saida;
        cps4_por(o + 0x3a, cps4_vezes(o, 0.3));
        *((*saida + 0x3e) as *mut u8) = 0xa;
        let o = *saida;
        cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do prêmio na seleção
        *apelo = 0x46;
        return 1;
    }
    if fama() - (rep as i32) < -3 {
        crate::comparacao::ramo("cps_4 premio no time");
        let o = *saida;
        cps4_por(o + 0x35, cps4_vezes(o, 0.3));
        *((*saida + 0x39) as *mut u8) = 0x14;
        let o = *saida;
        cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do prêmio no time
        *apelo = 0x3c;
        return 1;
    }
    let o = *saida;
    if o != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(o);
        liberar(o);
    }
    0
}

/// { 0x8c5560 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_is_rejection_reason_valid(motivo, oferta)`: 0
/// quando o motivo não cabe na oferta (pessoa +0xc, clube do lance +8): 0x2000 (promessa) com 24 anos ou mais (+0x18);
/// 0x1000 com o clube atual (0x4cf5a0) e o do lance em divisões diferentes (+0x57); 0x800 (rival) com os dois clubes,
/// sem os dois humanos (0x600a50), sem confronto na liga (0x4b02e0) e sem um ser parceiro (+0xb3/+0xb7/+0xbb) do outro.
/// Senão 1.
unsafe extern "stdcall" fn motivo_da_recusa_vale(motivo: u32, oferta: usize) -> u32 {
    let pessoa = cps4_d(0xae23bc).wrapping_add(cps4_d(oferta + 0xc).wrapping_mul(0x6e));
    let atual = cps4_clube_atual(pessoa);
    let lance = cps4_d(0xae23b4).wrapping_add(cps4_d(oferta + 8).wrapping_mul(0x245));
    let r = match motivo as u16 {
        0x2000 => (cps4_b(pessoa + 0x18) as i8) < 0x18,
        0x1000 => atual == 0 || cps4_d(atual + 0x57) == cps4_d(lance + 0x57),
        0x800 => {
            if lance == 0 || atual == 0 {
                return 1;
            }
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
            if humano(atual, 0, 1, 0, 0) != 0 && humano(lance, 0, 1, 0, 0) != 0 {
                return 1;
            }
            confronto(lance, atual) != 0
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| cps4_d(lance + k) == atual)
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| cps4_d(atual + k) == lance)
        }
        _ => true,
    };
    r as u32
}

/// { 0x8c54c0 } (stdcall, ret 4, transfer_manager.cpp): `transfer_manager_get_squad_status_reason(oferta)`: o motivo de
/// recusa pelo status no elenco (+0x4f >> 4) do contrato da pessoa (+0xc) com o clube atual (0x4cf5f0): 1 → 0x200, 2 e
/// 3 → 0x400, 5 → 0x2000; senão 0.
unsafe extern "stdcall" fn motivo_pelo_status_no_elenco(oferta: usize) -> u16 {
    let pessoa = cps4_d(0xae23bc).wrapping_add(cps4_d(oferta + 0xc).wrapping_mul(0x6e));
    let atual = cps4_clube_atual(pessoa);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = com_clube(cps4_d(0xae1050), pessoa, atual);
    if c == 0 {
        return 0;
    }
    match cps4_b(c + 0x4f) >> 4 {
        1 => 0x200,
        2 | 3 => 0x400,
        5 => 0x2000,
        _ => 0,
    }
}

// ponytail: aloca e solta a oferta (0x944cf1, 0x944c94): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_oferta_com_clausula(this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, u32) -> u32>(1125)(this, clube, jogador, saida, apelo, valor)
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
unsafe extern "stdcall" fn comparar_motivo_da_recusa_vale(motivo: u32, oferta: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(u32, usize) -> u32>(1126);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(motivo, oferta), || motivo_da_recusa_vale(motivo, oferta));
    PLACAR.registrar(igual && r == n, || format!("motivo_da_recusa_vale({:x}, pessoa {}) original={} rust={} {:x?} x {:x?}", motivo as u16, cps4_d(oferta + 0xc), r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
unsafe extern "stdcall" fn comparar_motivo_pelo_status_no_elenco(oferta: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u16>(1127);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(oferta), || motivo_pelo_status_no_elenco(oferta));
    PLACAR.registrar(igual && r == n, || format!("motivo_pelo_status_no_elenco(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", cps4_d(oferta + 0xc), r, n, mo, mr));
    r
}

// ---------- 254a leva: os limites da escalação: a tabela de restrições da partida, o limite do banco do dia e quem precisa de permissão de trabalho (transfer_manager.cpp) ----------

unsafe fn cpr1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Os 42 bytes da tabela de restrições com 0xff (o `rep stosd` de 10 duplas e o `stosw` da original).
unsafe fn cpr1_limpar(t: *mut u8) {
    core::ptr::write_bytes(t, 0xff, 42);
}

/// { 0x8cf6b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::fill_squad_restrictions(partida, t,
/// so_a_principal)`: enche os 42 bytes de `t` com 0xff. Com partida: o alcance das regras (0x8d2fa0); 0xff para aí. Fora
/// do tipo 4, a primeira das 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo
/// (+0x12) enche a tabela (virtual +0x1c com a partida e `t`). Com `so_a_principal`, fica só o primeiro limite que
/// existe: +5 ou +3 (gravado em +5), +2 ou +0 (gravado em +2), ou +0x16 com +0x17; o resto volta a 0xff.
unsafe extern "thiscall" fn restricoes_da_partida(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    cpr1_limpar(t);
    if partida == 0 {
        return;
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(0x8d2fa0usize);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff {
        return;
    }
    if tipo != 4 {
        for k in 0..28 {
            let r = cpr1_d(this + 0x8ac + k * 4);
            let casa = if tipo == 2 {
                cpr1_d(r + 0xe) == continente && cpr1_b(r + 0x12) == 2
            } else {
                cpr1_d(r + 0xa) == nacao && cpr1_b(r + 0x12) == tipo
            };
            if casa {
                let encher: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(cpr1_d(cpr1_d(r) + 0x1c));
                encher(r, partida, t);
                break;
            }
        }
    }
    if so_a_principal == 0 {
        return;
    }
    let v = if *t.add(5) != 0xff { *t.add(5) } else { *t.add(3) };
    if v != 0xff {
        crate::comparacao::ramo("cpr_1 limite de nao comunitarios");
        cpr1_limpar(t);
        *t.add(5) = v; // o limite de não comunitários
        return;
    }
    let v = if *t.add(2) != 0xff { *t.add(2) } else { *t };
    if v != 0xff {
        cpr1_limpar(t);
        *t.add(2) = v;
        return;
    }
    let (idade, n) = (*t.add(0x16), *t.add(0x17));
    cpr1_limpar(t);
    if idade != 0xff {
        *t.add(0x16) = idade;
        *t.add(0x17) = n;
    }
}

/// { 0x8cf880 } (thiscall pelo `this` em ecx, que a original passa direto à 0x8cf7e0; ret 0x18, transfer_manager.cpp):
/// `transfer_manager_check_matchday_squad_limit(clube, partida, msg, jogadores, _, saida)`: o limite da partida
/// (0x8cf7e0 em 6 bytes locais: +0 tipo, +1 idade, +5 quantidade); sem tipo, 0. O lado humano (+0x1c/+0x40 ou
/// +0x20/+0x41) vai para a 0x8968f0. Pelas (+0x4d & 0xf) + 11 vagas, a pessoa da vaga (`jogadores[k]` ou a tática
/// [0xde1d08] 0x8968f0) conta se é estrangeira na nação do clube (0x53d980 com 1; tipo 1, também fora da UE, 0x53d740)
/// ou passa da idade (+0x18, tipo 3): +2 (titulares) ou +3 (as outras) e +4 (todas). A conta vai a `saida`; passando da
/// quantidade, 1 e, com `msg`, o aviso (0x66f4e0) pelo tipo, pela conta e pela quantidade.
unsafe extern "thiscall" fn limite_do_banco_do_dia(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, _nao_usado: u32, saida: usize) -> u32 {
    let limite: unsafe extern "thiscall" fn(usize, usize, *mut u8) = core::mem::transmute(0x8cf7e0usize);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8968f0usize);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut l = [0u8; 6];
    limite(this, partida, l.as_mut_ptr());
    let tipo = l[0];
    if tipo == 0 {
        return 0;
    }
    let humano = ((clube == cpr1_d(partida + 0x1c) && cpr1_b(partida + 0x40) == 0) || (clube == cpr1_d(partida + 0x20) && cpr1_b(partida + 0x41) == 0)) as u32;
    if tipo == 1 {
        crate::comparacao::ramo("cpr_1 conta nao comunitarios");
    }
    let mut k: i8 = 0;
    while k < ((cpr1_b(partida + 0x4d) & 0xf).wrapping_add(0xb)) as i8 {
        let p = if jogadores == 0 {
            // ponytail: a original empurra a dupla da partida com o byte baixo trocado pela vaga (a 0x8968f0 lê o byte)
            da_tatica(cpr1_d(0xde1d08), clube, (partida & !0xff) | k as u8 as usize, humano)
        } else {
            cpr1_d(jogadores + (k as isize * 4) as usize)
        };
        if p != 0 {
            let conta = match tipo {
                2 => estrangeira(p, cpr1_d(clube + 0x53), 1) != 0,
                1 => estrangeira(p, cpr1_d(clube + 0x53), 1) != 0 && comunitaria(p, cpr1_d(clube + 0x53)) == 0,
                3 => cpr1_b(p + 0x18) as i8 > l[1] as i8,
                _ => false,
            };
            if conta {
                if k < 11 {
                    l[2] = l[2].wrapping_add(1);
                } else {
                    l[3] = l[3].wrapping_add(1);
                }
                l[4] = l[4].wrapping_add(1);
            }
        }
        k += 1;
    }
    if saida != 0 {
        core::ptr::copy_nonoverlapping(l.as_ptr(), saida as *mut u8, 6);
    }
    let (c, n) = (l[4] as i8, l[5] as i8);
    if !(1..=3).contains(&tipo) || c <= n {
        return 0;
    }
    if msg == 0 {
        return 1;
    }
    if tipo == 3 {
        if c > 1 {
            texto(msg, 0xa8ee08, c as i32, l[1] as i8 as i32);
        } else {
            texto(msg, 0xa8edbc, l[1] as i8 as i32);
        }
        let mut fim = msg; // o fim do texto (o `repne scasb` da original)
        while cpr1_b(fim) != 0 {
            fim += 1;
        }
        if n > 1 {
            texto(fim, 0xa8ed8c, n as i32);
        } else if n == 1 {
            texto(fim, 0xa8ed70);
        } else {
            texto(fim, 0xa8ed48);
        }
        return 1;
    }
    // os textos do tipo 1 (não comunitários) e do tipo 2 (estrangeiros)
    let f: [usize; 5] = if tipo == 1 { [0xa8f208, 0xa8f19c, 0xa8f134, 0xa8f0cc, 0xa8f07c] } else { [0xa8effc, 0xa8ef90, 0xa8ef28, 0xa8eebc, 0xa8ee68] };
    if c > 1 {
        if n > 1 {
            texto(msg, f[0], c as i32, n as i32);
        } else if n == 1 {
            texto(msg, f[1], c as i32);
        } else {
            texto(msg, f[2], c as i32);
        }
    } else if n == 1 {
        texto(msg, f[3], c as i32);
    } else if n == 0 {
        texto(msg, f[4]);
    }
    1
}

/// { 0x8c7560 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::player_needs_work_permit(jogador, clube,
/// ignorar_a_dada)`: 0 sem clube ou sem a nação dele (+0x53); jogador nulo: a caixa "Error" 0x1c23 e 0. Nação sem a marca
/// 0xe (+0x76) ou jogador da UE (0x53d740 com nação 0): 0. Com `ignorar_a_dada`: 1. Senão 0 só se o contrato (0x4cf5a0)
/// é do tipo 2 (+0x35 & 0x3f), o id cabe em [0xae2364], tem a marca 0x20 ([this+0x86e] + id × 8) e a permissão está no
/// vetor [this+0x93a] (conta +0x93e; busca binária 0x946464 com 0x8e55c0); senão 1.
unsafe extern "thiscall" fn precisa_de_permissao_de_trabalho(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    if clube == 0 || cpr1_d(clube + 0x53) == 0 {
        return 0;
    }
    if jogador == 0 {
        aviso_em(0xa8d514, 0x1c23);
        return 0;
    }
    if cpr1_b(cpr1_d(clube + 0x53) + 0x76) != 0xe {
        return 0;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    if comunitaria(jogador, 0) != 0 {
        return 0;
    }
    if ignorar_a_dada != 0 {
        return 1;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpr1_d(0xae1050), jogador);
    if c != 0 && cpr1_b(c + 0x35) & 0x3f == 2 {
        let id = cpr1_d(jogador) as i32;
        if id < cpr1_d(0xae2364) as i32 && cpr1_b(cpr1_d(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) & 0x20 != 0 {
            let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
            if bsearch(&id, cpr1_d(this + 0x93a), cpr1_d(this + 0x93e), 0xe, 0x8e55c0) != 0 {
                crate::comparacao::ramo("cpr_1 permissao achada");
                return 0; // tem a permissão
            }
        }
    }
    1
}

/// Comparador: lado a lado pelos 42 bytes da tabela (o alcance e a virtual das regras só escrevem nela).
unsafe extern "thiscall" fn comparar_restricoes_da_partida(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32)>(1083);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(t as usize, 42)], || o(this, partida, t, so_a_principal), || restricoes_da_partida(this, partida, t, so_a_principal));
    PLACAR.registrar(igual, || format!("restricoes_da_partida({:x}, {}) {:x?} x {:x?}", partida, so_a_principal, mo, mr));
}

/// Comparador: lado a lado pela saída (6 bytes) e pelo aviso (0x100 bytes, com `msg`), e pela devolução. A 0x8968f0 pode
/// pôr −1 numa vaga sem pessoa válida: a original já deixou assim e a cópia acha a mesma vaga vazia.
unsafe extern "thiscall" fn comparar_limite_do_banco_do_dia(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, nao_usado: u32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize) -> u32>(1084);
    let mut regioes = [(0usize, 0usize); 2];
    let mut n = 0;
    if saida != 0 {
        regioes[n] = (saida, 6);
        n += 1;
    }
    if msg != 0 {
        regioes[n] = (msg, 0x100);
        n += 1;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes[..n], || o(this, clube, partida, msg, jogadores, nao_usado, saida), || limite_do_banco_do_dia(this, clube, partida, msg, jogadores, nao_usado, saida));
    PLACAR.registrar(igual && r == c, || format!("limite_do_banco_do_dia(clube {:x}, partida {:x}) original={} rust={} {:x?} x {:x?}", clube, partida, r, c, mo, mr));
    r
}

/// Comparador: pelo valor (só lê). Jogador nulo com clube de nação: a caixa, só a original.
unsafe extern "thiscall" fn comparar_precisa_de_permissao_de_trabalho(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1085);
    if jogador == 0 {
        return o(this, jogador, clube, ignorar_a_dada);
    }
    let r = o(this, jogador, clube, ignorar_a_dada);
    let n = precisa_de_permissao_de_trabalho(this, jogador, clube, ignorar_a_dada);
    PLACAR.registrar(r == n, || format!("precisa_de_permissao_de_trabalho(jogador {}, clube {:x}, {}) original={} rust={}", cpr1_d(jogador), clube, ignorar_a_dada, r, n));
    r
}

// ---------- 254a leva: quem pode negociar: se o clube pode chegar ao jogador, os termos mandados ao jogador da troca e a fila da transferência livre do clube (transfer_manager.cpp) ----------

unsafe fn cpr2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// Os itens de uma LLIST, do começo (o cursor +0xa volta a +2 e fica no fim, como depois de uma volta da original).
unsafe fn cpr2_itens(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    cpr2_por(l + 0xa, cpr2_d(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// O corpo da 0x8ce990 sobre a lista local `l` (já construída): devolve (motivo, pode).
unsafe fn cpr2_pode(this: usize, jogador: usize, clube: usize, ignorar_abertas: u32, l: usize) -> (u16, u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let acertada: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x4cca30usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x8c5fc0usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let nat = cpr2_d(jogador + 0x1a);
    if nat != 0 && cpr2_d(nat) == cpr2_d(0x9cf458) {
        let cc = cpr2_d(jogador + 0x39);
        if (cc == 0 || cpr2_d(cc + 0x53) == nat) && clube != 0 && cpr2_d(clube + 0x53) != nat {
            return (0x27, 0); // não pode sair da nação
        }
    }
    let marca = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8));
    if cpr2_b(marca) & 0x40 != 0 {
        if acertada(cpr2_d(0xae1050), cpr2_d(jogador) as i32) != 0 {
            crate::comparacao::ramo("cpr_2 contrato acertado");
            return (0x21, 0); // contrato acertado
        }
        let m = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8)) as *mut u8;
        *m &= 0xbf;
    }
    if ignorar_abertas == 0 && (trocas(this, jogador, 0) != 0 || negociacoes(this, jogador, 0) != 0) {
        return (7, 0);
    }
    // a fila da transferência livre (+0x92c)
    let fila = this + 0x92c;
    cpr2_por(fila + 0xa, cpr2_d(fila + 2));
    let mut e = proximo(fila);
    while e != 0 {
        if cpr2_d(jogador) == cpr2_d(e) {
            return (1, 0);
        }
        e = proximo(fila);
    }
    if sob_oferta(this, jogador, 0) != 0 {
        ofertas(this, jogador, l, 0);
        cpr2_por(l + 0xa, cpr2_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            let k = cpr2_d(e) as i32;
            let vaga = if k < cpr2_d(this + 4) as i32 { cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)) } else { cpr2_d(this) };
            let o = cpr2_d(vaga);
            let st = cpr2_b(o + 0x2c);
            if (0xa..=0xe).contains(&st) {
                crate::comparacao::ramo("cpr_2 oferta em negociacao");
                *((o + 0x2d) as *mut u8) = 1;
                return (0x12, 0); // já em negociação
            }
            if st == 0x13 {
                crate::comparacao::ramo("cpr_2 transferencia futura");
                return (0x20, 0); // transferência futura acertada
            }
            e = proximo(l);
        }
    }
    if clube == 0 || jogador == 0 || cpr2_d(jogador + 0x61) == 0 {
        return (0, 1);
    }
    if basco(clube) != 0 {
        let nacao = cpr2_d(0xae23a8).wrapping_add(cpr2_d(0x9cf424).wrapping_mul(0x122));
        if estrangeira(jogador, nacao, 1) != 0 {
            return (0x19, 0);
        }
        return (0, 1);
    }
    let n = cpr2_d(clube + 0x53);
    if n != 0 && cpr2_d(n) == cpr2_d(0x9cf4dc) {
        let c57 = cpr2_d(clube + 0x57);
        if c57 == 0 || cpr2_d(c57) != cpr2_d(0x9cf8d8) {
            if estrangeira(jogador, n, 1) != 0 {
                return (0x28, 0);
            }
            return (0, 1);
        }
    }
    let id = cpr2_d(clube);
    if id == cpr2_d(0x9d0128) || id == cpr2_d(0x9d01a0) {
        let n1 = cpr2_d(jogador + 0x1a);
        if n1 == 0 || cpr2_d(n1) == cpr2_d(0x9cf488) {
            let n2 = cpr2_d(jogador + 0x1e);
            if n2 == 0 || cpr2_d(n2) != cpr2_d(0x9cf424) {
                return (0x1a, 0);
            }
        }
    }
    (0, 1)
}

/// { 0x8ce990 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::can_club_bid_for_player(jogador, clube, motivo, ignorar_abertas)`: não pode (0 e o motivo) se o
/// jogador é da nação [0x9cf458] sem clube ou com clube dela e o clube é de fora (0x27); com a marca de contrato
/// acertado (0x40) e o acerto (0x4cca30) (0x21; sem acerto, a marca sai); sem `ignorar_abertas`, com trocas que o
/// oferecem (0x8c5e40) ou negociações de troca (0x8c5fc0) (7); na fila da transferência livre (+0x92c) (1); sob oferta
/// (0x8cde70), pelas ofertas por ele (0x8c5cf0): uma em negociação (estados 0xa a 0xe; marca +0x2d) (0x12) ou uma
/// transferência futura (0x13) (0x20). Pelo clube: basco (0x53d7f0) com jogador estrangeiro na nação [0x9cf424] (0x19);
/// da nação [0x9cf4dc] fora da [0x9cf8d8] (+0x57) com jogador estrangeiro (0x28); os clubes [0x9d0128]/[0x9d01a0] com
/// jogador sem a nação [0x9cf424] (0x1a). Senão pode (1, motivo 0).
unsafe extern "thiscall" fn clube_pode_chegar_ao_jogador(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let (m, r) = cpr2_pode(this, jogador, clube, ignorar_abertas, l);
    if !motivo.is_null() {
        *motivo = m;
    }
    soltar(l);
    r
}

/// { 0x8c2060 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::note_terms_offered_to_exchange_player(k,
/// pessoa)`: `k` negativo ou fora da conta (+4): a caixa "Error" (0xdc3 ou 0xdc8). Na oferta `k` (vazia: nada), pela
/// lista dos jogadores da troca (+4), marca (+4 do item) o da `pessoa`; nenhum item marcado: o estado (+0x2c) vira 8 (se
/// era 7) ou 6.
unsafe extern "thiscall" fn termos_ao_jogador_da_troca(this: usize, k: i32, pessoa: u32) {
    if k < 0 {
        aviso_em(0xa8d514, 0xdc3);
        return;
    }
    if k >= cpr2_d(this + 4) as i32 {
        aviso_em(0xa8d514, 0xdc8);
        return;
    }
    let o = cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)));
    if o == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cpr2_d(o + 4);
    cpr2_por(l + 0xa, cpr2_d(l + 2));
    let mut nenhum = true;
    let mut e = proximo(cpr2_d(o + 4));
    while e != 0 {
        if cpr2_d(e) as u32 == pessoa {
            *((e + 4) as *mut u8) = 1; // termos mandados
        }
        if cpr2_b(e + 4) != 0 {
            nenhum = false;
        }
        e = proximo(cpr2_d(o + 4));
    }
    if nenhum {
        *((o + 0x2c) as *mut u8) = if cpr2_b(o + 0x2c) == 7 { 8 } else { 6 };
    }
}

/// { 0x8c1780 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::count_club_free_transfer_queue(clube)`:
/// clube nulo: a caixa "Error" 0xc04 e 0. Senão conta as pessoas da fila da transferência livre (+0x92c; o cursor volta
/// ao começo) com o clube do contrato (+0x39) igual ao clube ou ao parceiro dele (0x540a50 com 0 e 1), não nulo.
unsafe extern "thiscall" fn fila_livre_do_clube(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        aviso_em(0xa8d514, 0xc04);
        return 0;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let p = parceiro(clube, 0, 1);
    cpr2_por(this + 0x936, cpr2_d(this + 0x92e));
    let mut n = 0u8;
    let mut e = proximo(this + 0x92c);
    while e != 0 {
        let c = cpr2_d(cpr2_d(0xae23bc).wrapping_add(cpr2_d(e).wrapping_mul(0x6e)) + 0x39);
        if c == clube || (c == p && p != 0) {
            n = n.wrapping_add(1);
        }
        e = proximo(this + 0x92c);
    }
    n
}

/// Comparador: lado a lado pelo motivo, pela marca da pessoa (+0x86e) e pelo cursor da fila livre, e pela devolução.
/// As outras gravações (tirar a marca 0x40, marcar +0x2d da oferta em negociação) a cópia repete sobre o que a original
/// deixou e chega ao mesmo.
unsafe extern "thiscall" fn comparar_clube_pode_chegar_ao_jogador(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32>(1086);
    let marca = cpr2_d(this + 0x86e).wrapping_add(cpr2_d(jogador).wrapping_mul(8));
    let regioes = [(marca, 1), (this + 0x936, 4), (motivo as usize, 2)];
    let n = if motivo.is_null() { 2 } else { 3 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, clube, motivo, ignorar_abertas), || clube_pode_chegar_ao_jogador(this, jogador, clube, motivo, ignorar_abertas));
    PLACAR.registrar(igual && r == c, || format!("clube_pode_chegar_ao_jogador(jogador {}, clube {:x}, {}) original={} rust={} {:x?} x {:x?}", cpr2_d(jogador), clube, ignorar_abertas, r, c, mo, mr));
    r
}

/// Comparador: lado a lado pelo estado da oferta, pelas marcas (+4) dos itens da troca e pelo cursor da lista. Fora da
/// conta: a caixa, só a original.
unsafe extern "thiscall" fn comparar_termos_ao_jogador_da_troca(this: usize, k: i32, pessoa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, u32)>(1087);
    if k < 0 || k >= cpr2_d(this + 4) as i32 || cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4))) == 0 {
        return o(this, k, pessoa);
    }
    let of = cpr2_d(cpr2_d(this).wrapping_add((k as usize).wrapping_mul(4)));
    let l = cpr2_d(of + 4);
    let mut regioes = vec![(of + 0x2c, 1), (l + 0xa, 4)];
    for e in cpr2_itens(l) {
        regioes.push((e + 4, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, k, pessoa), || termos_ao_jogador_da_troca(this, k, pessoa));
    PLACAR.registrar(igual, || format!("termos_ao_jogador_da_troca({}, pessoa {}) {:x?} x {:x?}", k, pessoa, mo, mr));
}

/// Comparador: lado a lado pelo cursor da fila e pela devolução (só lê). Clube nulo: a caixa, só a original.
unsafe extern "thiscall" fn comparar_fila_livre_do_clube(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1088);
    if clube == 0 {
        return o(this, clube);
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x936, 4)], || o(this, clube), || fila_livre_do_clube(this, clube));
    PLACAR.registrar(igual && r == c, || format!("fila_livre_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

// ---------- 254a leva: as transferências futuras: a do jogador, tirar a da oferta e acrescentar a da oferta acertada (transfer_manager.cpp) ----------

unsafe fn cpr3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpr3_pessoa(id: usize) -> usize {
    cpr3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O byte de marcas da pessoa de id `id` no registro de 8 bytes em [this+0x86e].
unsafe fn cpr3_marca(this: usize, id: usize) -> *mut u8 {
    cpr3_d(this + 0x86e).wrapping_add(id.wrapping_mul(8)) as *mut u8
}

/// Na lista de índices de ofertas `l` (o cursor volta ao começo), o índice da primeira oferta de estado 0x13
/// (transferência futura), ou None.
unsafe fn cpr3_futura_na_lista(this: usize, l: usize) -> Option<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    cpr3_por(l + 0xa, cpr3_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if cpr3_b(cpr3_d(cpr3_d(this).wrapping_add(cpr3_d(e).wrapping_mul(4))) + 0x2c) == 0x13 {
            return Some(cpr3_d(e));
        }
        e = proximo(l);
    }
    None
}

/// O corpo da 0x8cca10 sobre a lista local `l` (já construída).
unsafe fn cpr3_futura(this: usize, jogador: usize, saida: *mut usize, l: usize) -> u32 {
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    if *cpr3_marca(this, cpr3_d(jogador)) & 0x10 == 0 {
        return 0;
    }
    let mut k = None;
    if ofertas(this, jogador, l, 0) != 0 {
        k = cpr3_futura_na_lista(this, l).filter(|&i| i != usize::MAX);
    }
    if k.is_none() {
        limpar(l);
        if trocas(this, jogador, l) != 0 {
            k = cpr3_futura_na_lista(this, l).filter(|&i| i != usize::MAX);
        }
    }
    let Some(k) = k else {
        // ponytail: a marca é relida pela original depois das chamadas; o vetor [this+0x86e] não muda entre elas
        *cpr3_marca(this, cpr3_d(jogador)) &= 0xef; // sem transferência futura: a marca sai
        return 0;
    };
    crate::comparacao::ramo("cpr_3 futura achada");
    if !saida.is_null() {
        let n = cpr3_d(this + 0x928) as i32;
        for i in 0..n.max(0) as usize {
            let r = cpr3_d(this + 0x920).wrapping_add(i * 0xc);
            if cpr3_d(r + 8) == k {
                *saida = r;
                break;
            }
        }
    }
    1 // a transferência futura existe
}

/// { 0x8cca10 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::get_future_transfer(jogador, saida)`: sem a marca 0x10 ([this+0x86e] + id × 8), 0. Pelas ofertas
/// pelo jogador (0x8c5cf0) e, sem achar, pelas trocas que o oferecem (0x8c5e40, com a lista esvaziada por 0x68d080), a
/// primeira de estado 0x13 com índice ≠ −1; com `saida`, o registro de 12 bytes do vetor [this+0x920] (conta +0x928)
/// com a oferta em +8; 1. Nenhuma: tira a marca 0x10 e 0.
unsafe extern "thiscall" fn transferencia_futura_do_jogador(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = cpr3_futura(this, jogador, saida, l);
    soltar(l);
    r
}

/// { 0x8cbf70 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_future_transfer(oferta)`: tira a
/// marca 0x10 do jogador (+0xc) e dos jogadores da troca (lista +4, o cursor volta ao começo); no vetor [this+0x920]
/// (conta +0x928), o primeiro registro com a oferta (+0x20 da oferta, +8 do registro) sai: os seguintes descem um
/// (0x8cbf40) e a conta cai de 1.
unsafe extern "thiscall" fn tirar_transferencia_futura(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cbf40usize);
    *cpr3_marca(this, cpr3_d(oferta + 0xc)) &= 0xef;
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(cpr3_d(oferta + 4));
        while e != 0 {
            *cpr3_marca(this, cpr3_d(e)) &= 0xef;
            e = proximo(cpr3_d(oferta + 4));
        }
    }
    let n = cpr3_d(this + 0x928) as i32;
    let alvo = cpr3_d(oferta + 0x20);
    for i in 0..n.max(0) {
        if cpr3_d(cpr3_d(this + 0x920).wrapping_add(i as usize * 0xc) + 8) == alvo {
            crate::comparacao::ramo("cpr_3 tirar futura");
            let mut j = i;
            while j < cpr3_d(this + 0x928) as i32 - 1 {
                let r = cpr3_d(this + 0x920).wrapping_add(j as usize * 0xc);
                copiar(r, r + 0xc);
                j += 1;
            }
            cpr3_por(this + 0x928, cpr3_d(this + 0x928).wrapping_sub(1));
            return;
        }
    }
}

/// { 0x8cb9a0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::add_future_transfer(oferta, data)`:
/// acrescenta ao vetor [this+0x920] (capacidade +0x924, conta +0x928; cheio, cresce de 10 em 10, sem memória: a caixa
/// 0x27c3 e nada) o registro de 12 bytes com a `data` (0x417c10) e a oferta (+0x20 em +8) e reordena (qsort 0x945333
/// com 0x778e90). A oferta fica com +0x2d = 1 e o estado 0x13. Suspende os clubes que miram o jogador (0x842b00 de
/// [this+0x856], com 0) e liga a marca 0x10 dele; com contrato (0x4cf5a0) marcado em +0x49, a oferta vai a +0x4a do
/// contrato. O mesmo (suspender e marcar) para cada jogador da troca (lista +4).
unsafe extern "thiscall" fn acrescentar_transferencia_futura(this: usize, oferta: usize, data: usize) {
    crate::comparacao::ramo("cpr_3 acrescentar futura");
    if !cpl3_crescer(this, 0x920, 4, 10, 0xc, 0x27c3, 0xa8ec74) {
        return;
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut [u16; 4], usize) -> usize = core::mem::transmute(0x417c10usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842b00usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut temp = [0u16; 4];
    copia(cpr3_d(this + 0x920).wrapping_add(cpr3_d(this + 0x928).wrapping_mul(0xc)), &mut temp, data);
    cpr3_por(cpr3_d(this + 0x920).wrapping_add(cpr3_d(this + 0x928).wrapping_mul(0xc)) + 8, cpr3_d(oferta + 0x20));
    let n = cpr3_d(this + 0x928).wrapping_add(1);
    cpr3_por(this + 0x928, n);
    qsort(cpr3_d(this + 0x920), n as u32, 0xc, 0x778e90);
    *((oferta + 0x2d) as *mut u8) = 1;
    *((oferta + 0x2c) as *mut u8) = 0x13; // transferência futura
    suspender(cpr3_d(this + 0x856), cpr3_pessoa(cpr3_d(oferta + 0xc)), 0);
    *cpr3_marca(this, cpr3_d(oferta + 0xc)) |= 0x10; // a marca da transferência futura
    let c = contrato_de(cpr3_d(0xae1050), cpr3_pessoa(cpr3_d(oferta + 0xc)));
    if c != 0 && cpr3_b(c + 0x49) != 0 {
        cpr3_por(c + 0x4a, cpr3_d(oferta + 0x20));
    }
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(cpr3_d(oferta + 4));
        while e != 0 {
            suspender(cpr3_d(this + 0x856), cpr3_pessoa(cpr3_d(e)), 0);
            *cpr3_marca(this, cpr3_d(e)) |= 0x10;
            e = proximo(cpr3_d(oferta + 4));
        }
    }
}

/// Comparador: lado a lado pela marca do jogador e por `saida`, e pela devolução (tirar a marca é o único outro efeito;
/// a cópia, depois da original, acha o mesmo).
unsafe extern "thiscall" fn comparar_transferencia_futura_do_jogador(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(1089);
    let regioes = [(cpr3_marca(this, cpr3_d(jogador)) as usize, 1), (saida as usize, 4)];
    let n = if saida.is_null() { 1 } else { 2 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, saida), || transferencia_futura_do_jogador(this, jogador, saida));
    PLACAR.registrar(igual && r == c, || format!("transferencia_futura_do_jogador(jogador {}) original={} rust={} {:x?} x {:x?}", cpr3_d(jogador), r, c, mo, mr));
    r
}

/// Comparador: lado a lado pela conta e pelo vetor das futuras, pelas marcas do jogador e dos da troca e pelo cursor
/// da lista da troca (não aloca nem libera).
unsafe extern "thiscall" fn comparar_tirar_transferencia_futura(this: usize, oferta: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1090);
    let mut regioes = vec![(this + 0x928, 4), (cpr3_marca(this, cpr3_d(oferta + 0xc)) as usize, 1)];
    let n = cpr3_d(this + 0x928) as i32;
    if n > 0 {
        regioes.push((cpr3_d(this + 0x920), n as usize * 0xc));
    }
    let l = cpr3_d(oferta + 4);
    if l != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        cpr3_por(l + 0xa, cpr3_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            regioes.push((cpr3_marca(this, cpr3_d(e)) as usize, 1));
            e = proximo(l);
        }
        regioes.push((l + 0xa, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || tirar_transferencia_futura(this, oferta));
    PLACAR.registrar(igual, || format!("tirar_transferencia_futura(oferta {:x}) {:x?} x {:x?}", cpr3_d(oferta + 0x20), mo, mr));
}

/// Comparador: a original acrescenta (pode realocar), reordena e suspende as listas de alvos: só ela, conferida pelo
/// plano: a conta sobe de 1, o registro (data e oferta) está no vetor, o estado 0x13 com +0x2d = 1 e a marca 0x10 do
/// jogador.
unsafe extern "thiscall" fn comparar_acrescentar_transferencia_futura(this: usize, oferta: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1091);
    let n = cpr3_d(this + 0x928);
    let mut alvo = [0u8; 0xc];
    alvo[..8].copy_from_slice(core::slice::from_raw_parts(data as *const u8, 8));
    alvo[8..].copy_from_slice(&(cpr3_d(oferta + 0x20) as u32).to_le_bytes());
    o(this, oferta, data);
    if cpr3_d(this + 0x920) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpr3_d(this + 0x928);
    let marca = *cpr3_marca(this, cpr3_d(oferta + 0xc));
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpr3_d(this + 0x920), conta, 0xc, &alvo) && cpr3_b(oferta + 0x2c) == 0x13 && cpr3_b(oferta + 0x2d) == 1 && marca & 0x10 != 0;
    PLACAR.registrar(ok, || format!("acrescentar_transferencia_futura(oferta {:x}): conta {} -> {}, estado {:#x}, marca {:#x}", cpr3_d(oferta + 0x20), n, conta, cpr3_b(oferta + 0x2c), marca));
}

// ---------- 254a leva: as ofertas abertas e as desfeitas: as ofertas pelos jogadores do clube, desfazer as ofertas pelo jogador e guardar o percentual da próxima venda (transfer_manager.cpp) ----------

unsafe fn cpr4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpr4_pessoa(id: usize) -> usize {
    cpr4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpr4_clube(id: usize) -> usize {
    cpr4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// { 0x8c6100 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_open_offers_for_club_players(clube,
/// lista)`: pelas ofertas (vetor [this], conta +4), fora das vazias e das de estado 2, 3, 4, 0x10 e 0x14 e das de estado
/// 0xf com +0x2e ≥ 3: o contrato do jogador (+0xc; 0x4cf5a0, guardado em [0xae1080]) que não é de empréstimo (+0x35 &
/// 0x3f ≠ 5) é do `clube`? Conta e, com `lista`, acrescenta o índice (4 bytes, 0x68d0d0). Devolve a conta.
unsafe extern "thiscall" fn ofertas_abertas_pelos_jogadores_do_clube(this: usize, clube: usize, lista: usize) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let mut n = 0u32;
    let mut i: i32 = 0;
    let (mut desfeita, mut emprestado) = (false, false); // os contadores, uma vez por chamada
    while i < cpr4_d(this + 4) as i32 {
        let o = cpr4_d(cpr4_d(this).wrapping_add((i as usize).wrapping_mul(4)));
        if o != 0 {
            let conta = match cpr4_b(o + 0x2c) {
                2 | 3 | 4 | 0x10 | 0x14 => false,
                0xf => {
                    desfeita |= cpr4_b(o + 0x2e) >= 3;
                    cpr4_b(o + 0x2e) < 3 // desfeita (+0x2e = 3) não conta
                }
                _ => true,
            };
            if conta {
                let c = contrato_de(cpr4_d(0xae1050), cpr4_pessoa(cpr4_d(o + 0xc)));
                cpr4_por(0xae1080, c);
                emprestado |= c != 0 && cpr4_b(c + 0x35) & 0x3f == 5 && cpr4_clube(cpr4_d(c + 4)) == clube;
                let dono = if c != 0 && cpr4_b(c + 0x35) & 0x3f != 5 { cpr4_clube(cpr4_d(c + 4)) } else { 0 }; // o empréstimo não conta
                if dono == clube {
                    if lista != 0 {
                        acrescentar(lista, &i, 4);
                    }
                    n += 1;
                }
            }
        }
        i += 1;
    }
    if desfeita {
        crate::comparacao::ramo("cpr_4 oferta desfeita");
    }
    if emprestado {
        crate::comparacao::ramo("cpr_4 jogador emprestado");
    }
    n
}

/// { 0x8c47d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::collapse_offers_for_player(pessoa)`: com ofertas pela pessoa (byte +1 do registro de 8 bytes em
/// [this+0x86e]), pelas ofertas por ela (0x8c5cf0 na lista local): a de transferência futura (0x13) perde a futura
/// (0x8cbf70); todas ficam no estado 0xf com +0x2e = 3; o vendedor é o clube do contrato do jogador da oferta (0x4cf5a0,
/// guardado em [0xae1080]); os lances do comprador (+8) pelo jogador e os do vendedor pelos da troca (lista +4) saem
/// (0x84aeb0 de [this+0x856]) e a oferta é agendada (0x8c6220 com 10 dias).
unsafe extern "thiscall" fn desfazer_ofertas_pelo_jogador(this: usize, pessoa: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar_futura: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cbf70usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    if cpr4_b(cpr4_d(this + 0x86e).wrapping_add(cpr4_d(pessoa).wrapping_mul(8)) + 1) != 0 {
        ofertas(this, pessoa, l, 0);
        cpr4_por(l + 0xa, cpr4_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            crate::comparacao::ramo("cpr_4 desfazer oferta");
            let o = cpr4_d(cpr4_d(this).wrapping_add(cpr4_d(e).wrapping_mul(4)));
            if cpr4_b(o + 0x2c) == 0x13 {
                tirar_futura(this, o);
            }
            *((o + 0x2c) as *mut u8) = 0xf; // desfeita
            *((o + 0x2e) as *mut u8) = 3;
            let jogador = cpr4_pessoa(cpr4_d(o + 0xc));
            let c = contrato_de(cpr4_d(0xae1050), jogador);
            cpr4_por(0xae1080, c);
            let vendedor = if c != 0 { cpr4_clube(cpr4_d(c + 4)) } else { 0 };
            desmarcar(cpr4_d(this + 0x856), cpr4_clube(cpr4_d(o + 8)), jogador);
            let t = cpr4_d(o + 4);
            if t != 0 {
                cpr4_por(t + 0xa, cpr4_d(t + 2));
                let mut x = proximo(cpr4_d(o + 4));
                while x != 0 {
                    desmarcar(cpr4_d(this + 0x856), vendedor, cpr4_pessoa(cpr4_d(x)));
                    x = proximo(cpr4_d(o + 4));
                }
            }
            agendar(this, o, 0xa, 0); // a oferta volta daqui a 10 dias
            e = proximo(l);
        }
    }
    soltar(l);
}

/// { 0x8caaf0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_next_sale_percentage(registro)`:
/// acrescenta o percentual da próxima venda (13 bytes copiados de `registro`) ao vetor [this+0x880] (capacidade +0x884,
/// conta +0x888); cheio, cresce de 50 em 50 (sem memória: caixa 0x2553 e nada). Reordena (qsort 0x945333 com 0x8e54d0,
/// a ordem da busca 0x8cb0f0).
unsafe extern "thiscall" fn guardar_percentual_da_proxima_venda(this: usize, registro: usize) {
    crate::comparacao::ramo("cpr_4 percentual da proxima venda");
    if !cpl3_crescer(this, 0x880, 4, 50, 0xd, 0x2553, 0xa8eb50) {
        return;
    }
    let n = cpr4_d(this + 0x888);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpr4_d(this + 0x880).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    cpr4_por(this + 0x888, n.wrapping_add(1)); // mais um percentual
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpr4_d(this + 0x880), cpr4_d(this + 0x888) as u32, 0xd, 0x8e54d0);
}

/// Comparador: sem `lista`, lado a lado por [0xae1080] e pela devolução; com `lista` (cresce a lista: não pode rodar
/// duas vezes), a cópia conta antes sem lista e a original roda: confere a conta.
unsafe extern "thiscall" fn comparar_ofertas_abertas_pelos_jogadores_do_clube(this: usize, clube: usize, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1092);
    if lista != 0 {
        let n = ofertas_abertas_pelos_jogadores_do_clube(this, clube, 0);
        let r = o(this, clube, lista);
        PLACAR.registrar(r == n, || format!("ofertas_abertas_pelos_jogadores_do_clube({:x}, lista) original={} rust={}", clube, r, n));
        return r;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, clube, 0), || ofertas_abertas_pelos_jogadores_do_clube(this, clube, 0));
    PLACAR.registrar(igual && r == c, || format!("ofertas_abertas_pelos_jogadores_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparador: agenda (aloca) e mexe nas listas de alvos: só a original. O plano antes: as ofertas pela pessoa (0x8c5cf0
/// numa lista própria, destruída depois); depois confere que todas ficaram no estado 0xf com +0x2e = 3.
unsafe extern "thiscall" fn comparar_desfazer_ofertas_pelo_jogador(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1093);
    if cpr4_b(cpr4_d(this + 0x86e).wrapping_add(cpr4_d(pessoa).wrapping_mul(8)) + 1) == 0 {
        return o(this, pessoa);
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    ofertas(this, pessoa, l, 0);
    let mut alvo = Vec::new();
    cpr4_por(l + 0xa, cpr4_d(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        alvo.push(cpr4_d(cpr4_d(this).wrapping_add(cpr4_d(e).wrapping_mul(4))));
        e = proximo(l);
    }
    soltar(l);
    o(this, pessoa);
    let ok = alvo.iter().all(|&of| cpr4_b(of + 0x2c) == 0xf && cpr4_b(of + 0x2e) == 3);
    PLACAR.registrar(ok, || format!("desfazer_ofertas_pelo_jogador(pessoa {}): {} ofertas, estados {:x?}", cpr4_d(pessoa), alvo.len(), alvo.iter().map(|&of| (cpr4_b(of + 0x2c), cpr4_b(of + 0x2e))).collect::<Vec<_>>()));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
unsafe extern "thiscall" fn comparar_guardar_percentual_da_proxima_venda(this: usize, registro: usize) {
    let n = cpr4_d(this + 0x888);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1094)(this, registro);
    if cpr4_d(this + 0x880) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpr4_d(this + 0x888);
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpr4_d(this + 0x880), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("guardar_percentual_da_proxima_venda: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

// ---------- 253a leva: as datas da transferência: a data de conclusão da oferta, o primeiro dia que as regras permitem e a janela fixa de transferências (transfer_manager.cpp) ----------

unsafe fn cpq5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O dia de hoje: Data de 8 bytes em 0xae2c90 (+0 dia do ano i16, +2 ano u16, +4 bissexto).
const CPQ5_HOJE: usize = 0xae2c90;

/// As estáticas das operações de data (0x54c5b0, 0x54c8e0, 0x54c770, 0x54cdb0: valor e guarda), para os comparadores.
const CPQ5_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// `a` antes de `b` na ordem dos originais: o ano (+2, 16 bits sem sinal) e, no mesmo ano, o dia (+0, 16 bits com sinal).
unsafe fn cpq5_antes(a: *const [u32; 2], b: *const [u32; 2]) -> bool {
    let ler = |p: *const [u32; 2]| (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p as usize + 2) as *const u16));
    let ((da, aa), (db, ab)) = (ler(a), ler(b));
    aa < ab || (aa == ab && da < db)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq5_clube(id: usize) -> usize {
    cpq5_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq5_pessoa(id: usize) -> usize {
    cpq5_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do contrato da pessoa da oferta (+0xc): 0x4cf5a0 com o gerente de contratos [0xae1050], guardado em
/// [0xae1080]; contrato de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn cpq5_vendedor(oferta: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpq5_d(0xae1050), cpq5_pessoa(cpq5_d(oferta + 0xc)));
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpq5_b(c + 0x35) & 0x3f != 5 {
        cpq5_clube(cpq5_d(c + 4))
    } else {
        0
    }
}

/// O método +0xc das regras (`TRANSFER_RULES::can_make_offer(oferta, clube, data, motivo)`): 1 permite; 0 e o motivo.
unsafe fn cpq5_permite(regra: usize, oferta: usize, clube: usize, data: *const [u32; 2], motivo: *mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const [u32; 2], *mut u32) -> u32 = core::mem::transmute(cpq5_d(cpq5_d(regra) + 0xc));
    f(regra, oferta, clube, data, motivo)
}

/// Os motivos que não seguram a data (o dia vale assim mesmo): 0x14, 0x15, 0x1f e 0x11.
fn cpq5_motivo_que_libera(m: u32) -> bool {
    matches!(m as u16, 0x14 | 0x15 | 0x1f | 0x11)
}

/// { 0x8cee70 } (thiscall, ret 8, transfer_manager.cpp): `transfer_manager_get_offer_completion_date(oferta, saida)`:
/// a data em que a oferta pode se concretizar, pelo tipo de prazo (+0x30 & 0xf): 0 hoje; 5 o início da janela fixa
/// (0x8d0bb0); 1 e 4 o primeiro dia que as regras permitem (0x8cf0e0, que dá o motivo); 2 o fim da temporada da nação
/// do clube vendedor mais 7 dias (0x669bd0; fora da temporada, hoje; sem vendedor, a caixa "Error" 0x31da); 3 o dia
/// seguinte ao fim do contrato (+0x2d), nunca antes de hoje (sem contrato, hoje). Outros tipos não gravam. Devolve o
/// motivo (só o ax: 0 fora dos tipos 1 e 4). O vendedor (0x4cf5a0, guardado em [0xae1080]) é calculado sempre.
unsafe extern "thiscall" fn data_de_conclusao_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut t, mut t2) = ([0u32; 2], [0u32; 2]);
    let vendedor = cpq5_vendedor(oferta);
    match cpq5_b(oferta + 0x30) & 0xf {
        0 => {
            crate::comparacao::ramo("cpq_5 conclusao imediata");
            copia(saida, &mut t, hoje); // tipo 0: hoje
        }
        5 => {
            let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(0x8d0bb0usize);
            janela(saida, 0);
        }
        1 | 4 => {
            let primeiro: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(0x8cf0e0usize);
            return primeiro(this, oferta, saida);
        }
        2 => {
            if vendedor == 0 {
                aviso_em(0xa8d514, 0x31da);
            } else {
                let temporada: unsafe extern "C" fn(usize, usize, *mut [u32; 2]) -> u32 = core::mem::transmute(0x669bd0usize);
                if temporada(vendedor, 0, saida) != 0 {
                    let r = mais(saida, &mut t, 7);
                    copia(saida, &mut t2, r);
                } else {
                    copia(saida, &mut t2, hoje);
                }
            }
        }
        3 => {
            let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contrato_de(cpq5_d(0xae1050), cpq5_pessoa(cpq5_d(oferta + 0xc)));
            if c != 0 {
                let r = mais((c + 0x2d) as *const [u32; 2], &mut t2, 1);
                copia(saida, &mut t, r);
                if cpq5_antes(saida, hoje) {
                    copia(saida, &mut t2, hoje);
                }
            } else {
                copia(saida, &mut t2, hoje);
            }
        }
        _ => {}
    }
    0
}

/// { 0x8cf0e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_allowed_transfer_date(oferta,
/// saida)`: grava hoje na saída e começa no maior entre hoje e, no tipo de prazo 4 (+0x30 & 0xf), o fim do contrato
/// (+0x2d). Tipo 5: o início da janela fixa (0x8d0bb0) e o método +0xc da regra da nação do comprador (+8) dá o motivo
/// (9 não conta). Pessoa sem ficha de jogador (+0x61): hoje. Senão anda dia a dia (até hoje + 365, sem chegar) pelas
/// regras da nação do comprador e, com o +4 da oferta e um vendedor (0x4cf5a0, guardado em [0xae1080]), da do vendedor:
/// o primeiro dia que as duas permitem (ou cujo motivo libera: 0x14, 0x15, 0x1f, 0x11) vai para a saída e devolve 0;
/// sem dia, devolve o primeiro motivo (9 não conta) e a saída fica hoje. Só o ax é devolvido.
unsafe extern "thiscall" fn primeiro_dia_permitido_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut dia, mut limite, mut inicio, mut t, mut t2) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    padrao(&mut dia);
    mais(hoje, &mut limite, 0x16d);
    padrao(&mut inicio);
    let mut motivo = 0u32;
    // ponytail: o motivo da regra é uma variável só, sem valor antes da primeira chamada (a original não a zera; a
    // regra o grava sempre que devolve 0)
    let mut m = 0u32;
    copia(saida, &mut t, hoje);
    let pessoa = cpq5_pessoa(cpq5_d(oferta + 0xc));
    if cpq5_b(oferta + 0x30) & 0xf == 4 {
        let c = contrato_de(cpq5_d(0xae1050), pessoa);
        if c != 0 {
            copia(&mut inicio, &mut t, (c + 0x2d) as *const [u32; 2]);
            t = *hoje;
            if cpq5_antes(&inicio, hoje) {
                copia(&mut inicio, &mut t, hoje);
            }
        } else {
            copia(&mut inicio, &mut t, hoje);
        }
    } else {
        copia(&mut inicio, &mut t, hoje);
    }
    let comprador = cpq5_clube(cpq5_d(oferta + 8));
    let vendedor = cpq5_vendedor(oferta);
    if cpq5_b(oferta + 0x30) & 0xf == 5 {
        let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(0x8d0bb0usize);
        janela(saida, 0);
        let regra = cpo10_regra(this, cpq5_d(comprador + 0x53));
        if regra != 0 && cpq5_permite(regra, oferta, comprador, saida, &mut m) == 0 && m as u16 != 9 {
            motivo = m;
        }
        return motivo as u16;
    }
    if cpq5_d(pessoa + 0x61) == 0 {
        copia(saida, &mut t, hoje);
        return 0;
    }
    let regra_c = cpo10_regra(this, cpq5_d(comprador + 0x53));
    let regra_v = if vendedor != 0 && cpq5_d(oferta + 4) != 0 { cpo10_regra(this, cpq5_d(vendedor + 0x53)) } else { 0 };
    crate::comparacao::ramo("cpq_5 dia permitido");
    copia(&mut dia, &mut t, &inicio); // o primeiro dia testado é o início
    while cpq5_antes(&dia, &limite) {
        let mut ve_vendedor = true;
        if regra_c != 0 && cpq5_permite(regra_c, oferta, comprador, &dia, &mut m) == 0 {
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            ve_vendedor = cpq5_motivo_que_libera(motivo);
        }
        if ve_vendedor {
            if regra_v == 0 || cpq5_permite(regra_v, oferta, vendedor, &dia, &mut m) != 0 {
                copia(saida, &mut t2, &dia);
                return 0;
            }
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            if cpq5_motivo_que_libera(motivo) {
                copia(saida, &mut t2, &dia);
                return 0;
            }
        }
        let r = mais(&dia, &mut t, 1);
        copia(&mut dia, &mut t2, r);
    }
    motivo as u16
}

/// { 0x8d0bb0 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_get_next_window_dates(inicio, fim)`: as
/// janelas fixas do ano de hoje são 1º a 31 de julho e 15 de dezembro a 15 de janeiro (0x549f70; comparadas só pelo
/// dia do ano, com sinal). Dentro de uma janela, o início é hoje e o fim o da janela (15 de janeiro do ano seguinte, se
/// for a de dezembro); antes de julho, 1º a 31 de julho; entre agosto e 14 de dezembro, 15 de dezembro a 15 de janeiro
/// do ano seguinte. O fim só com `fim` não nulo. Não usa o ecx (os chamadores passam o gerente).
unsafe extern "stdcall" fn janela_fixa_de_transferencias(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let dma: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> *mut [u32; 2] = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    // ponytail: a original empurra o ano num registrador com lixo acima (e o ano + 1 sobre a palavra do argumento); a
    // 0x549f70 só usa os 16 bits de baixo
    let ano = || *((CPQ5_HOJE + 2) as *const u16) as u32;
    let (mut a, mut b, mut c, mut d, mut e, mut t) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    dma(&mut a, 1, 6, ano(), 0xffff_ffff); // 1º de julho
    dma(&mut b, 0x1f, 6, ano(), 0xffff_ffff); // 31 de julho
    dma(&mut c, 0xf, 0xb, ano(), 0xffff_ffff); // 15 de dezembro
    dma(&mut d, 0xf, 0, ano(), 0xffff_ffff); // 15 de janeiro
    let dia = |x: &[u32; 2]| x[0] as u16 as i16;
    let h = *(CPQ5_HOJE as *const i16);
    if h >= dia(&a) && h <= dia(&b) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            copia(fim, &mut t, &b);
        }
    } else if h >= dia(&c) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            dma(&mut e, 0xf, 0, (ano() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copia(fim, &mut t, &e);
        }
    } else if h <= dia(&d) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            copia(fim, &mut t, &d);
        }
    } else if h < dia(&a) {
        crate::comparacao::ramo("cpq_5 janela de julho");
        copia(inicio, &mut t, &a);
        if !fim.is_null() {
            copia(fim, &mut t, &b);
        }
    } else {
        copia(inicio, &mut t, &c);
        if !fim.is_null() {
            dma(&mut e, 0xf, 0, (ano() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copia(fim, &mut t, &e);
        }
    }
}

/// As regiões dos comparadores de datas: a saída, o contrato guardado [0xae1080] e as estáticas das datas.
fn cpq5_regioes(saidas: &[usize]) -> Vec<(usize, usize)> {
    let mut r: Vec<(usize, usize)> = saidas.iter().filter(|&&s| s != 0).map(|&s| (s, 8)).collect();
    r.push((0xae1080, 4));
    r.extend_from_slice(&CPQ5_ESTATICAS);
    r
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo (as regras, a temporada e a
/// janela só calculam).
unsafe extern "thiscall" fn comparar_data_de_conclusao_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(1057);
    let reg = cpq5_regioes(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || data_de_conclusao_da_oferta(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("data_de_conclusao_da_oferta(tipo {}) motivo {} x {}, memória igual {}", cpq5_b(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo.
unsafe extern "thiscall" fn comparar_primeiro_dia_permitido_da_oferta(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(1058);
    let reg = cpq5_regioes(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || primeiro_dia_permitido_da_oferta(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("primeiro_dia_permitido_da_oferta(tipo {}) motivo {} x {}, memória igual {}", cpq5_b(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pelo início e o fim (só datas, sem estáticas).
unsafe extern "stdcall" fn comparar_janela_fixa_de_transferencias(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let o = orig::<unsafe extern "stdcall" fn(*mut [u32; 2], *mut [u32; 2])>(1059);
    let reg = cpq5_regioes(&[inicio as usize, fim as usize]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(inicio, fim), || janela_fixa_de_transferencias(inicio, fim));
    PLACAR.registrar(igual, || format!("janela_fixa_de_transferencias(hoje {:x}, fim {}) diferente", *(CPQ5_HOJE as *const u32), !fim.is_null()));
}

// ---------- 253a leva: o dinheiro da transferência: a compensação pela pessoa, pagar a transferência e guardar as cláusulas, e se o clube vendedor tem de aceitar (transfer_manager.cpp) ----------

unsafe fn cpq6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq6_clube(id: usize) -> usize {
    cpq6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq6_pessoa(id: usize) -> usize {
    cpq6_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do contrato da `pessoa` (0x4cf5a0 com o gerente de contratos [0xae1050], guardado em [0xae1080]); contrato
/// de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn cpq6_vendedor(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpq6_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpq6_b(c + 0x35) & 0x3f != 5 {
        cpq6_clube(cpq6_d(c + 4))
    } else {
        0
    }
}

unsafe fn cpq6_por_u32(r: &mut [u8; 13], k: usize, v: usize) {
    r[k..k + 4].copy_from_slice(&(v as u32).to_le_bytes());
}

/// As estáticas das operações de data (valor e guarda), para os comparadores.
const CPQ6_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// { 0x8d0780 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só da data local):
/// `transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas)`: sem pessoa ou comprador, a caixa
/// "Error" 0x372e e 0. Comissão técnica (sem ficha de jogador +0x61, com +0x69) de contrato tipo 2: o que falta do
/// contrato (dias até o fim +0x2d × o salário +0xc / 7) entre 1000 e 1000000. Regras novas ([0xae233c] = 1, sem o
/// argumento) com contrato tipo 2: a média entre o que falta e o valor do jogador no vendedor (0x4d0c60). Senão (contrato
/// que não é de tipo 5, as duas nações): entre clubes da nação [0x9cf4a8], metade do valor; fora disso, só abaixo de 24
/// anos na data (o fim do contrato, no tipo 2; senão hoje) e entre nações britânicas ou a Irlanda (0x5446d0), metade
/// do valor. Os outros casos, 0.
unsafe extern "stdcall" fn compensacao_pela_pessoa(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], usize) -> usize = core::mem::transmute(0x417c10usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let idade: unsafe extern "C" fn(usize, *const [u32; 2]) -> u16 = core::mem::transmute(0x53ab90usize);
    let britanica: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5446d0usize);
    let gerente = || cpq6_d(0xae1050);
    // o que falta do contrato: os dias até o fim × o salário semanal, por 7 (divisão truncada, como a original)
    let resto = |c: usize| dias_ate(c + 0x2d, cpq6_d(0xae2c90) as u32, cpq6_d(0xae2c94) as u32).wrapping_mul(cpq6_d(c + 0xc) as i32) / 7;
    let mut data = [0u32; 2];
    let mut t = [0u32; 2];
    padrao(&mut data);
    if pessoa == 0 || comprador == 0 {
        aviso_em(0xa8d514, 0x372e);
        return 0;
    }
    if cpq6_d(pessoa + 0x61) == 0 && cpq6_d(pessoa + 0x69) != 0 {
        let c = contrato_de(gerente(), pessoa);
        if c == 0 || cpq6_b(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        return resto(c).clamp(1000, 1_000_000);
    }
    if cpq6_b(0xae233c) == 1 && sem_regras_novas == 0 {
        let c = contrato_de(gerente(), pessoa);
        if c == 0 || cpq6_b(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        let r = resto(c);
        let v = valor(gerente(), pessoa, cpq6_clube(cpq6_d(c + 4)), 1, 0xffff_ffff, 0);
        return v.wrapping_add(r) / 2;
    }
    let c = contrato_de(gerente(), pessoa);
    if c == 0 {
        return 0;
    }
    let tipo = cpq6_b(c + 0x35) & 0x3f;
    if tipo == 5 {
        return 0;
    }
    let vendedor = cpq6_clube(cpq6_d(c + 4));
    if cpq6_d(comprador + 0x53) == 0 || cpq6_d(vendedor + 0x53) == 0 {
        return 0;
    }
    if tipo == 2 {
        copia(&mut data, &mut t, c + 0x2d);
    } else {
        copia(&mut data, &mut t, 0xae2c90);
    }
    let n = cpq6_d(0x9cf4a8);
    if cpq6_d(cpq6_d(vendedor + 0x53)) == n && cpq6_d(cpq6_d(comprador + 0x53)) == n {
        return valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2;
    }
    if idade(pessoa, &data) as u8 as i8 >= 0x18 {
        return 0;
    }
    if britanica(cpq6_d(comprador + 0x53)) == 0 || britanica(cpq6_d(vendedor + 0x53)) == 0 {
        return 0;
    }
    crate::comparacao::ramo("cpq_6 compensacao britanica");
    valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2 // menor de 24 anos entre clubes britânicos
}

/// { 0x8ca8a0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::pay_fee_and_record_clauses(oferta)`: soma o
/// valor (+0x1b) aos totais das transferências (0x448ad0); o comprador (+8) paga e o vendedor (clube do contrato da
/// pessoa +0xc, 0x4cf5a0) recebe a primeira parcela (o valor / parcelas +0x40, ou tudo), em 64 bits (0x599890,
/// 0x599800). Com parcelas, guarda as que faltam (13 bytes: a parcela, os ids do comprador e do vendedor, parcelas − 1)
/// na lista +0x872 (0x68d0d0) e soma ao +0x1c das finanças a parcela × −5/21 (comprador) e × 5/21 (vendedor). Guarda
/// as cláusulas: a porcentagem da próxima venda (+0x3f, 0x8caaf0, marca 8), o prêmio por jogo no time (+0x35 com o byte
/// +0x39, 0x8cac40, marca 2) e o prêmio por jogo pela seleção (+0x3a com +0x3e, 0x8cad90, marca 4); as marcas no
/// registro de 8 bytes da pessoa em [this+0x86e].
unsafe extern "thiscall" fn pagar_transferencia_e_clausulas(this: usize, oferta: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599890usize);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599800usize);
    let por_na_lista: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let proxima_venda: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8caaf0usize);
    let premio_time: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8cac40usize);
    let premio_selecao: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x8cad90usize);
    let tabela = || cpq6_d(0xae2a38);
    let marcar = |bit: u8| {
        let m = (cpq6_d(this + 0x86e) + cpq6_d(oferta + 0xc) * 8) as *mut u8;
        *m |= bit;
    };
    let comprador = cpq6_clube(cpq6_d(oferta + 8));
    let vendedor = cpq6_vendedor(cpq6_pessoa(cpq6_d(oferta + 0xc)));
    let fin_c = financas(tabela(), comprador);
    // ponytail: sem vendedor, a original leria as finanças de um lugar sem valor (e cairia adiante, em [vendedor]); a
    // cópia não paga ninguém
    let fin_v = if vendedor != 0 { financas(tabela(), vendedor) } else { 0 };
    let valor = cpq6_d(oferta + 0x1b) as i32;
    let n = cpq6_b(oferta + 0x40) as i8;
    let primeira = if n > 0 { (valor / n as i32) as i64 } else { valor as i64 };
    totais(cpq6_d(0xad9dac), valor, comprador, vendedor);
    crate::comparacao::ramo("cpq_6 pagar transferencia");
    pagar(fin_c, primeira);
    if fin_v != 0 {
        receber(fin_v, primeira); // o vendedor recebe
    }
    let mut r = [0u8; 13];
    let n = cpq6_b(oferta + 0x40) as i8;
    if n > 0 {
        let parcela = cpq6_d(oferta + 0x1b) as i32 / n as i32;
        cpq6_por_u32(&mut r, 0, parcela as usize);
        cpq6_por_u32(&mut r, 4, cpq6_d(comprador));
        cpq6_por_u32(&mut r, 8, cpq6_d(vendedor));
        r[12] = (n as u8).wrapping_sub(1);
        let fc = financas(tabela(), comprador);
        let fv = financas(tabela(), vendedor);
        if fc != 0 {
            let s = (fc + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(ftol(parcela as f64 * f64_em(0x970420)));
        }
        if fv != 0 {
            let s = (fv + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(ftol(parcela as f64 * f64_em(0x970418)));
        }
        por_na_lista(this + 0x872, r.as_ptr(), 0xd);
    }
    let p = cpq6_b(oferta + 0x3f);
    if (p as i8) > 0 {
        r[0] = p;
        cpq6_por_u32(&mut r, 1, cpq6_d(comprador));
        cpq6_por_u32(&mut r, 5, cpq6_d(vendedor));
        cpq6_por_u32(&mut r, 9, cpq6_d(oferta + 0xc));
        proxima_venda(this, r.as_ptr());
        marcar(8);
    }
    let v = cpq6_d(oferta + 0x35) as i32;
    if v > 0 {
        cpq6_por_u32(&mut r, 0, cpq6_d(oferta + 0xc));
        cpq6_por_u32(&mut r, 4, v as usize);
        cpq6_por_u32(&mut r, 9, cpq6_d(vendedor));
        r[8] = cpq6_b(oferta + 0x39);
        premio_time(this, r.as_ptr());
        marcar(2);
    }
    let v = cpq6_d(oferta + 0x3a) as i32;
    if v > 0 {
        cpq6_por_u32(&mut r, 0, cpq6_d(oferta + 0xc));
        cpq6_por_u32(&mut r, 4, v as usize);
        cpq6_por_u32(&mut r, 9, cpq6_d(vendedor));
        r[8] = cpq6_b(oferta + 0x3e);
        premio_selecao(this, r.as_ptr());
        marcar(4);
    }
}

/// { 0x8cd240 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só dos valores locais):
/// `transfer_manager_selling_club_must_accept(oferta, estrito, motivo)`: sem vendedor (clube do contrato da pessoa
/// +0xc que não é de tipo 5, 0x4cf5a0), 1. Com cláusula de rescisão ativa (0x867b50), sem troca (+4) nem parcelas
/// (+0x40) e o valor (+0x1b) não abaixo da multa (+0x21 do contrato; os dois por 0x43f460 com 4): motivo 0x1b e 1. Com
/// o vendedor em crise (+0x165 das finanças = 1), o preço pedido (0x4d1600) não abaixo de 20 × a reputação (+0x80;
/// pelo menos 20000), o valor ponderado da oferta (0x8e7b50) acima de 50000 e o dobro dele acima do preço: motivo 0x1c
/// e 1 (com `estrito`, só se o preço ficar abaixo de 1,3 × o valor ponderado). Senão 0.
unsafe extern "stdcall" fn clube_vendedor_deve_aceitar(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let ativa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
    let exibir: unsafe extern "thiscall" fn(*const u32, u32) -> i64 = core::mem::transmute(0x43f460usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let comprador = cpq6_clube(cpq6_d(oferta + 8));
    let pessoa = cpq6_pessoa(cpq6_d(oferta + 0xc));
    let vendedor = cpq6_vendedor(pessoa);
    if vendedor == 0 {
        return 1;
    }
    let c = contrato_de(cpq6_d(0xae1050), pessoa);
    crate::comparacao::ramo("cpq_6 clube deve aceitar");
    if ativa(c) != 0 && cpq6_d(oferta + 4) == 0 && cpq6_b(oferta + 0x40) == 0 {
        let (multa, valor) = (cpq6_d(c + 0x21) as u32, cpq6_d(oferta + 0x1b) as u32);
        let m = exibir(&multa, 4);
        let v = exibir(&valor, 4);
        if v >= m {
            if !motivo.is_null() {
                *motivo = 0x1b;
            }
            return 1;
        }
    }
    if cpq6_b(financas(cpq6_d(0xae2a38), vendedor) + 0x165) != 1 {
        return 0;
    }
    let pedido = preco(cpq6_d(0xae1050), vendedor, comprador, pessoa, 0, 1, 1);
    let w = ponderado(oferta, 0, 0, 1, 0, 1);
    let piso = (*((vendedor + 0x80) as *const i16) as i32 * 20).max(20000);
    if pedido < piso || w.wrapping_mul(2) <= pedido || w <= 50000 {
        return 0;
    }
    if estrito != 0 {
        let w2 = ponderado(oferta, 0, 0, 1, 0, 1);
        if !((pedido as f64) < w2 as f64 * f64_em(0x968de8)) {
            return 0;
        }
    }
    if !motivo.is_null() {
        *motivo = 0x1c;
    }
    1
}

/// Comparador: pelo valor (lado a lado pelas estáticas das datas; o valor do jogador 0x4d0c60 e a idade só calculam).
unsafe extern "stdcall" fn comparar_compensacao_pela_pessoa(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, u32) -> i32>(1060);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&CPQ6_ESTATICAS, || o(pessoa, comprador, sem_regras_novas), || compensacao_pela_pessoa(pessoa, comprador, sem_regras_novas));
    PLACAR.registrar(igual && ro == rr, || format!("compensacao_pela_pessoa(pessoa {:x}) {} x {}, estáticas iguais {}", if pessoa != 0 { cpq6_d(pessoa) } else { 0 }, ro, rr, igual));
    ro
}

/// Comparador: paga, acrescenta nas listas (aloca) e marca: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_pagar_transferencia_e_clausulas(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1061)(this, oferta)
}

/// Comparador: lado a lado pelo motivo e [0xae1080], e pelo valor devolvido (o preço e o valor ponderado só calculam).
unsafe extern "stdcall" fn comparar_clube_vendedor_deve_aceitar(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32, *mut u16) -> u32>(1062);
    let mut reg: Vec<(usize, usize)> = vec![(0xae1080, 4)];
    if !motivo.is_null() {
        reg.push((motivo as usize, 2));
    }
    reg.extend_from_slice(&CPQ6_ESTATICAS);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(oferta, estrito, motivo), || clube_vendedor_deve_aceitar(oferta, estrito, motivo));
    PLACAR.registrar(igual && ro == rr, || format!("clube_vendedor_deve_aceitar(estrito {}) {} x {}, memória igual {}", estrito, ro, rr, igual));
    ro
}

// ---------- 253a leva: os contratos da transferência: a permissão de trabalho na mudança de clube, concluir a oferta de contrato aceita e oferecer contrato ao jogador da oferta (transfer_manager.cpp) ----------

unsafe fn cpq7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq7_clube(id: usize) -> usize {
    cpq7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq7_pessoa(id: usize) -> usize {
    cpq7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A permissão de trabalho da `pessoa` (14 bytes, ordenadas pelo id: [this+0x93a], conta +0x93e), como a original
/// procura: pessoa com id fora de [0xae2364] ou sem a marca (bit 0x20 do registro de 8 bytes em [this+0x86e]), 0; senão
/// a busca binária (0x946464 com 0x8e55c0).
unsafe fn cpq7_permissao(this: usize, pessoa: usize) -> usize {
    if pessoa != 0 && cpq7_d(pessoa) as i32 >= cpq7_d(0xae2364) as i32 {
        return 0;
    }
    let id = cpq7_d(pessoa) as u32;
    if cpq7_b(cpq7_d(this + 0x86e) + id as usize * 8) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(&id, cpq7_d(this + 0x93a), cpq7_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// { 0x8c7e20 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só da data local):
/// `TRANSFER_MANAGER::update_work_permit_on_move(pessoa, clube)`: se o jogador precisa de permissão no clube novo
/// (0x8c7560 com 1) e já tem uma: troca a nação da permissão (+0xa) pela do clube (+0x53), zera +4 e +6 e, se mudou de
/// nação e as duas não são da marca 0xe (+0x76), grava o ano de hoje (+8); clube sem nação: a caixa "Error" 0x1dc2. Sem
/// permissão: a nova (0x8c7070 com o ano de hoje, ordenando), depois de hoje + 52 semanas (0x54c770, sem uso). Se não
/// precisa (a 0x8c7560 é chamada de novo) ou sem clube: tira a permissão que tiver (a marca 0x20, a última no lugar
/// dela, conta − 1 e reordena com 0x945333).
unsafe extern "thiscall" fn permissao_de_trabalho_na_mudanca(this: usize, pessoa: usize, clube: usize) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if clube != 0 {
        if precisa(this, pessoa, clube, 1) != 0 {
            let r = cpq7_permissao(this, pessoa);
            if r != 0 {
                let nacao = cpq7_d(0xae23a8).wrapping_add(cpq7_d(r + 0xa).wrapping_mul(0x122));
                let nc = cpq7_d(clube + 0x53);
                if nc == 0 {
                    aviso_em(0xa8d514, 0x1dc2);
                    return;
                }
                if nacao != nc && !(cpq7_b(nacao + 0x76) == 0xe && cpq7_b(nc + 0x76) == 0xe) {
                    core::ptr::write_unaligned((r + 8) as *mut u16, *((0xae2c92) as *const u16));
                }
                core::ptr::write_unaligned((r + 4) as *mut u16, 0);
                core::ptr::write_unaligned((r + 6) as *mut u16, 0);
                core::ptr::write_unaligned((r + 0xa) as *mut u32, cpq7_d(nc) as u32);
                return;
            }
            let semanas: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c770usize);
            let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
            let mut t = [0u32; 2];
            semanas(0xae2c90, &mut t, 0x34);
            crate::comparacao::ramo("cpq_7 permissao nova");
            // ponytail: a original empurra o ano com lixo nos 16 bits altos; a 0x8c7070 só grava a palavra
            let ano = *((0xae2c92) as *const u16) as u32;
            dar(this, pessoa, cpq7_d(clube + 0x53), ano, 0, 1); // a permissão nova
            return;
        }
        if precisa(this, pessoa, clube, 1) != 0 {
            return;
        }
    }
    let r = cpq7_permissao(this, pessoa);
    if r == 0 {
        return;
    }
    let marca = (cpq7_d(this + 0x86e) + cpq7_d(pessoa) * 8) as *mut u8;
    *marca &= 0xdf;
    let ultima = cpq7_d(this + 0x93a) + cpq7_d(this + 0x93e) * 0xe - 0xe;
    core::ptr::copy(ultima as *const u8, r as *mut u8, 0xe);
    let n = cpq7_d(this + 0x93e).wrapping_sub(1);
    core::ptr::write_unaligned((this + 0x93e) as *mut usize, n);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpq7_d(this + 0x93a), n as u32, 0xe, 0x8e55c0);
}

/// { 0x8c38b0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::complete_accepted_contract_offer(oferta)`: oferta de contrato (+0 a pessoa, +4 o clube, +8 as
/// luvas, +0x61 o cargo). Se o jogador precisa de permissão no clube (0x8c7560 com 1) e ela não sai (0x8c7950): com
/// contrato de tipo 5 (0x4cf5a0, guardado em [0xae1080]), a notícia da oferta que caiu (0x8de790 com uma
/// TRANSFER_OFFER local: 0x8e5680, ids em +8 e +0xc, +0x1f = 0, motivo 0x13; 0x8e5710); senão a notícia da permissão
/// recusada (0x8e4290); devolve 0. Senão assina (0x4cf720) e, assinado, o clube paga as luvas (0x5a0590, 0x5998e0) e
/// devolve 1.
unsafe extern "thiscall" fn concluir_oferta_de_contrato_aceita(this: usize, oferta: usize) -> u32 {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c7950usize);
    let pessoa = cpq7_pessoa(cpq7_d(oferta));
    let clube = cpq7_clube(cpq7_d(oferta + 4));
    if precisa(this, pessoa, clube, 1) != 0 && concedida(this, pessoa, cpq7_d(clube + 0x53), oferta, 0, 0) == 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato_de(cpq7_d(0xae1050), pessoa);
        *(0xae1080 as *mut usize) = c;
        if c != 0 && cpq7_b(c + 0x35) & 0x3f == 5 {
            let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x8e5680usize);
            let caiu: unsafe extern "thiscall" fn(usize, *mut u8, usize, u32) = core::mem::transmute(0x8de790usize);
            let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(0x8e5710usize);
            let mut o = [0u8; 0x44];
            construir(o.as_mut_ptr());
            o[8..12].copy_from_slice(&(cpq7_d(clube) as u32).to_le_bytes());
            o[0xc..0x10].copy_from_slice(&(cpq7_d(pessoa) as u32).to_le_bytes());
            o[0x1f] = 0;
            caiu(this, o.as_mut_ptr(), clube, 0x13);
            destruir(o.as_mut_ptr());
        } else {
            let recusada: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8e4290usize);
            recusada(this, pessoa, oferta);
        }
        return 0;
    }
    let assinar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x4cf720usize);
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x4cf720 e a 0x53bd10 só usam o byte
    if assinar(cpq7_d(0xae1050), oferta, cpq7_b(oferta + 0x61) as u32) == 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let luvas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5998e0usize);
    let valor = cpq7_d(oferta + 8) as u32;
    let fin = financas(cpq7_d(0xae2a38), clube);
    crate::comparacao::ramo("cpq_7 luvas");
    luvas(fin, valor); // as luvas
    1
}

/// { 0x8c1e40 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH do contrato local):
/// `TRANSFER_MANAGER::offer_contract_to_offer_player(clube, pessoa, oferta)`: monta o contrato (STAFF_CONTRACT local,
/// 0x85e410) que o clube ofereceria (0x4cda70 com a vaga +0x20 da oferta, fator de salário 1 e modo 2). Montado:
/// acrescenta a oferta de contrato (0x4cc490 com a vaga +0x20, o valor +0x24, 0xb, 0, 1, 0). Sem contrato: sendo o
/// jogador principal (+0xc), atualiza a lista do clube (0x849ce0 e 0x843e10 com 1) e retira a oferta (0x8bfcb0, motivo
/// 0x16); sendo da troca, soma o valor dele (+0x52) ao da oferta (+0x1b) e o tira da troca (0x8c41c0 com 1 e 0x16).
/// Devolve o que a última chamada devolveu.
unsafe extern "thiscall" fn oferecer_contrato_ao_jogador_da_oferta(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x85e410usize);
    let oferecer: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, f64, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    let mut c = [0u8; 0x50];
    construir(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_7 contrato da oferta");
    if oferecer(cpq7_d(0xae1050), pessoa, clube, c.as_mut_ptr(), cpq7_d(oferta + 0x20), 1.0f64, 2) != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cc490usize);
        return acrescentar(cpq7_d(0xae1050), c.as_mut_ptr(), cpq7_d(oferta + 0x20), cpq7_d(oferta + 0x24), 0xb, 0, 1, 0);
    }
    if cpq7_d(pessoa) == cpq7_d(oferta + 0xc) {
        let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
        let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
        let e = entrada(cpq7_d(this + 0x856), clube, pessoa, (cpq7_b(oferta + 0x11) != 0xb) as u32, 0);
        if e != 0 {
            // ponytail: a original empurra o tipo (setne) com o resto do ponteiro da entrada acima; a 0x843e10 só lê o byte
            atualizar(cpq7_d(this + 0x856), pessoa, clube, 1, (cpq7_b(e) != 0xb) as u32);
        }
        let mut o = oferta;
        return retirar(this, &mut o, 0, 0x16);
    }
    let v = (oferta + 0x1b) as *mut i32;
    core::ptr::write_unaligned(v, core::ptr::read_unaligned(v).wrapping_add(cpq7_d(pessoa + 0x52) as i32));
    let tirar_da_troca: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c41c0usize);
    tirar_da_troca(cpq7_d(0xde1f54), pessoa, oferta, 1, 0x16)
}

/// Comparador: grava as permissões (pode crescer o vetor e reordena): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_permissao_de_trabalho_na_mudanca(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1063)(this, pessoa, clube)
}

/// Comparador: assina o contrato (aloca), paga e posta notícias: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_concluir_oferta_de_contrato_aceita(this: usize, oferta: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1064)(this, oferta)
}

/// Comparador: acrescenta ofertas de contrato (aloca), mexe nas listas e retira ofertas: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_oferecer_contrato_ao_jogador_da_oferta(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1065)(this, clube, pessoa, oferta)
}

// ---------- 253a leva: as ofertas de transferência: desfazer a oferta, fazer a oferta de empréstimo e concluir as transferências futuras do dia (transfer_manager.cpp) ----------

unsafe fn cpq8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq8_clube(id: usize) -> usize {
    cpq8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq8_pessoa(id: usize) -> usize {
    cpq8_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato da pessoa da oferta (+0xc; 0x4cf5a0 com o gerente [0xae1050]), guardado em [0xae1080].
unsafe fn cpq8_contrato(oferta: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)));
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato da pessoa da oferta; contrato de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn cpq8_vendedor(oferta: usize) -> usize {
    let c = cpq8_contrato(oferta);
    if c != 0 && cpq8_b(c + 0x35) & 0x3f != 5 {
        cpq8_clube(cpq8_d(c + 4))
    } else {
        0
    }
}

/// Recomeça o cursor da LLIST (+0xa = +2, o primeiro nó).
unsafe fn cpq8_recomecar(lista: usize) {
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cpq8_d(lista + 2));
}

/// O estado da oferta (+0x2c) que deixa a marca 1 da pessoa sair: 2, 3, 0x14, 0x10, 4 e 0xf com +0x2e ≥ 3.
unsafe fn cpq8_estado_encerrado(oferta: usize) -> bool {
    match cpq8_b(oferta + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => cpq8_b(oferta + 0x2e) >= 3,
        _ => false,
    }
}

/// { 0x8c44e0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::collapse_offer(oferta, clube, motivo,
/// tirar_futura)`: na oferta de estado 0x13 tira a marca 0x10 da pessoa (+0xc) e dos jogadores da troca (lista +4) e,
/// com `tirar_futura`, tira a transferência futura (0x8cbf70). Põe o estado 0x10, fecha as propostas (0x8c6db0), solta
/// os lances do comprador (+8) pela pessoa e do vendedor (clube do contrato, guardado em [0xae1080]) pelos da troca
/// (0x84aeb0), posta a notícia (0x8de790 com o clube e o motivo) e agenda a oferta (0x8c6220 com 0x14). Encerrada a
/// oferta e todas as da lista +0 (estados de `cpq8_estado_encerrado`), tira a marca 1 da pessoa. Atualiza a lista do
/// comprador (0x849ce0 e 0x843e10 com 1), tira a oferta de contrato (0x4cca70), zera +0x10 e solta os contratos da troca
/// (0x4ccb30 com a vaga +0x20; +4 do item = 0).
unsafe extern "thiscall" fn desfazer_oferta(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let marca = |id: usize| (cpq8_d(this + 0x86e) + id * 8) as *mut u8;
    crate::comparacao::ramo("cpq_8 desfazer oferta");
    if cpq8_b(oferta + 0x2c) == 0x13 {
        *marca(cpq8_d(oferta + 0xc)) &= 0xef;
        if cpq8_d(oferta + 4) != 0 {
            cpq8_recomecar(cpq8_d(oferta + 4));
            loop {
                let it = proximo(cpq8_d(oferta + 4));
                if it == 0 {
                    break;
                }
                *marca(cpq8_d(it)) &= 0xef;
            }
        }
        if tirar_futura != 0 {
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cbf70usize);
            tirar(this, oferta);
        }
    }
    *((oferta + 0x2c) as *mut u8) = 0x10;
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
    fechar(this, oferta);
    let pessoa = cpq8_pessoa(cpq8_d(oferta + 0xc));
    let c = cpq8_contrato(oferta);
    let vendedor = if c != 0 { cpq8_clube(cpq8_d(c + 4)) } else { 0 };
    desmarcar(cpq8_d(this + 0x856), cpq8_clube(cpq8_d(oferta + 8)), pessoa);
    if cpq8_d(oferta + 4) != 0 {
        cpq8_recomecar(cpq8_d(oferta + 4));
        loop {
            let it = proximo(cpq8_d(oferta + 4));
            if it == 0 {
                break;
            }
            desmarcar(cpq8_d(this + 0x856), vendedor, cpq8_pessoa(cpq8_d(it)));
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8de790usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    noticia(this, oferta, clube, motivo);
    agendar(this, oferta, 0x14, 0);
    if cpq8_estado_encerrado(oferta) {
        let mut todas = true;
        if cpq8_d(oferta) != 0 {
            cpq8_recomecar(cpq8_d(oferta));
            loop {
                let it = proximo(cpq8_d(oferta));
                if it == 0 {
                    break;
                }
                if !cpq8_estado_encerrado(cpq8_d(cpq8_d(this) + cpq8_d(it) * 4)) {
                    todas = false;
                    break;
                }
            }
        }
        if todas {
            let m = marca(cpq8_d(oferta + 0xc));
            if *m & 1 != 0 {
                *m &= 0xfe;
            }
        }
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
    let e = entrada(cpq8_d(this + 0x856), cpq8_clube(cpq8_d(oferta + 8)), cpq8_pessoa(cpq8_d(oferta + 0xc)), (cpq8_b(oferta + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o tipo (setne) com lixo nos bytes altos; a 0x843e10 só lê o byte
        atualizar(cpq8_d(this + 0x856), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8)), 1, (cpq8_b(e) != 0xb) as u32);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    tirar_contrato(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(oferta + 0xc)), cpq8_clube(cpq8_d(oferta + 8))); // a oferta de contrato sai
    *((oferta + 0x10) as *mut u8) = 0;
    if cpq8_d(oferta + 4) != 0 {
        let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4ccb30usize);
        cpq8_recomecar(cpq8_d(oferta + 4));
        loop {
            let it = proximo(cpq8_d(oferta + 4));
            if it == 0 {
                break;
            }
            soltar(cpq8_d(0xae1050), cpq8_pessoa(cpq8_d(it)), cpq8_d(oferta + 0x20));
            *((it + 4) as *mut u8) = 0;
        }
    }
}

/// { 0x8bed60 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::make_loan_offer(jogador, clube, fim, percentual, a5, a6)`: numa TRANSFER_OFFER local (0x8e5680)
/// monta a oferta de empréstimo (0x8e7810) e, num STAFF_CONTRACT local (0x85e410), o contrato pelo atual (0x85e590 com o
/// fim, a porcentagem do salário e as duas cláusulas); acrescenta a oferta (0x8be300 com o contrato) e a oferta de
/// contrato (0x4cc490 com a vaga +0x20 e o valor +0x24 da oferta, 0xb, 0, 1, 0). Destrói a oferta local (0x8e5710) e
/// devolve a vaga (+0x20).
unsafe extern "thiscall" fn fazer_oferta_de_emprestimo(this: usize, jogador: usize, clube: usize, fim: usize, percentual: u32, a5: u32, a6: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x8e5680usize);
    let emprestimo: unsafe extern "thiscall" fn(*mut u8, usize, usize) = core::mem::transmute(0x8e7810usize);
    let construir_contrato: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x85e410usize);
    let pelo_atual: unsafe extern "thiscall" fn(*mut u8, usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x85e590usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(0x8be300usize);
    let acrescentar_contrato: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cc490usize);
    let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(0x8e5710usize);
    let campo = |o: &[u8; 0x44], k: usize| u32::from_le_bytes([o[k], o[k + 1], o[k + 2], o[k + 3]]);
    let mut o = [0u8; 0x44];
    construir(o.as_mut_ptr());
    emprestimo(o.as_mut_ptr(), jogador, clube);
    let mut c = [0u8; 0x50];
    construir_contrato(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_8 oferta de emprestimo");
    pelo_atual(c.as_mut_ptr(), jogador, clube, fim, percentual, a5, a6); // o salário pela porcentagem
    acrescentar(this, o.as_mut_ptr(), c.as_mut_ptr());
    acrescentar_contrato(cpq8_d(0xae1050), c.as_mut_ptr(), campo(&o, 0x20), campo(&o, 0x24), 0xb, 0, 1, 0);
    let vaga = campo(&o, 0x20);
    destruir(o.as_mut_ptr());
    vaga
}

/// { 0x8cbbe0 } (thiscall, sem argumentos, transfer_manager.cpp): `TRANSFER_MANAGER::process_due_future_transfers()`:
/// percorre as transferências futuras (12 bytes: +0 a data, +8 o índice da oferta; [this+0x920], conta +0x928) até a
/// primeira depois de hoje (0x456740). Oferta nula: a caixa "Error" 0x2846. Com as permissões (0x8c7690 com 1): se as
/// restrições (0x8cfc90) seguram pelo comprador ou, havendo vendedor, pelo vendedor (motivo 9 não conta), desfaz a
/// oferta (0x8c44e0 com o clube e o motivo); senão a notícia (0x8dbef0), conclui (0x8c9390), estado 0x14 e agenda
/// (0x8c6220 com 0xa). Depois tira as processadas do começo do vetor (0x8cbf40 uma a uma) e diminui a conta.
unsafe extern "thiscall" fn concluir_transferencias_futuras(this: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8c7690usize);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(0x8cfc90usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c44e0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8dbef0usize);
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c9390usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    // o motivo das restrições: só a palavra baixa é gravada; os 16 bits altos ficam 0, como na original
    let mut motivo = 0u32;
    let mut feitas = 0i32;
    while feitas < cpq8_d(this + 0x928) as i32 {
        let e = cpq8_d(this + 0x920) + feitas as usize * 0xc;
        if depois(e, cpq8_d(0xae2c90) as u32, cpq8_d(0xae2c94) as u32) != 0 {
            break;
        }
        let oferta = cpq8_d(cpq8_d(this) + cpq8_d(e + 8) * 4);
        if oferta == 0 {
            aviso_em(0xa8d514, 0x2846);
        } else if permissoes(this, oferta, 1) != 0 {
            let comprador = cpq8_clube(cpq8_d(oferta + 8));
            let mut segura = false;
            if restricoes(this, oferta, comprador, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                desfazer(this, oferta, comprador, motivo, 0);
                segura = true;
            } else if cpq8_vendedor(oferta) != 0 {
                let v = cpq8_vendedor(oferta);
                if restricoes(this, oferta, v, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                    let v = cpq8_vendedor(oferta);
                    desfazer(this, oferta, v, motivo, 0);
                    segura = true;
                }
            }
            if !segura {
                noticia(this, oferta);
                crate::comparacao::ramo("cpq_8 transferencia futura");
                concluir(this, oferta); // a transferência acontece
                *((oferta + 0x2c) as *mut u8) = 0x14;
                agendar(this, oferta, 0xa, 0);
            }
        }
        feitas += 1;
    }
    if feitas < 0 {
        aviso_em(0xa8d514, 0x288a);
    }
    if feitas == 0 {
        return;
    }
    let mut k = feitas;
    if k < cpq8_d(this + 0x928) as i32 {
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cbf40usize);
        while k < cpq8_d(this + 0x928) as i32 {
            let v = cpq8_d(this + 0x920);
            copiar(v + (k - feitas) as usize * 0xc, v + k as usize * 0xc);
            k += 1;
        }
    }
    let n = (this + 0x928) as *mut i32;
    core::ptr::write_unaligned(n, core::ptr::read_unaligned(n).wrapping_sub(feitas));
}

/// Comparador: muda o estado, as listas e as marcas, posta notícia e agenda: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_desfazer_oferta(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1066)(this, oferta, clube, motivo, tirar_futura)
}

/// Comparador: acrescenta a oferta e a oferta de contrato (alocam): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_fazer_oferta_de_emprestimo(this: usize, jogador: usize, clube: usize, fim: usize, percentual: u32, a5: u32, a6: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32>(1067)(this, jogador, clube, fim, percentual, a5, a6)
}

/// Comparador: conclui e desfaz transferências (elencos, dinheiro, notícias): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_concluir_transferencias_futuras(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1068)(this)
}

// ---------- 252a leva: as ofertas de transferência: o clube recusa a oferta, soltar os jogadores da oferta e montar a oferta em dinheiro (transfer_manager.cpp) ----------

unsafe fn cpp7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpp7_pessoa(id: usize) -> usize {
    cpp7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpp7_clube(id: usize) -> usize {
    cpp7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// { 0x8c36b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::club_rejects_offer(oferta, clube,
/// motivo)`: fora dos estados 0x11/0x12 (+0x2c), conta a tentativa (+0x2e). Recusa do clube vendedor (id = +8): estado
/// 0x12; motivo 8 (16 bits) reagenda (0x8c6220 com 7, 0) e sai; senão, sem clube humano no contrato do jogador
/// (+0xc; contrato em [0xae1080], +0x35 & 0x3f ≠ 5, 0x600a50) e com mais de uma tentativa, um sorteio(2 × tentativas)
/// diferente de 0 rejeita a oferta (0x8bf680 com o endereço da oferta, 0, 6) e sai. Recusa do outro lado: estado
/// 0x11, desassossega o jogador (0x8c1290); com o clube vendedor do computador e o sorteio, retira a oferta (0x8bfcb0);
/// com motivo 8 o resto reagenda e sai (sem o teste de tentativas). No fim: reagenda e dá a notícia do adiamento
/// (0x8dda90 com o motivo inteiro, como a original o empurra).
unsafe extern "thiscall" fn clube_recusa_a_oferta(this: usize, oferta: usize, clube: usize, motivo: u32) {
    let estado = cpp7_b(oferta + 0x2c);
    if estado == 0x11 || estado == 0x12 {
        return;
    }
    crate::comparacao::ramo("cpp_7 recusa");
    *((oferta + 0x2e) as *mut u8) = cpp7_b(oferta + 0x2e).wrapping_add(1); // mais uma tentativa
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // a original passa o endereço do próprio argumento
    let mut o = oferta;
    if cpp7_d(clube) == cpp7_d(oferta + 8) {
        *((oferta + 0x2c) as *mut u8) = 0x12;
        if motivo as u16 == 8 {
            agendar(this, oferta, 7, 0);
            return;
        }
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato_de(cpp7_d(0xae1050), cpp7_pessoa(cpp7_d(oferta + 0xc)));
        *(0xae1080 as *mut usize) = c;
        let mut no_fim = false;
        if c != 0 && cpp7_b(c + 0x35) & 0x3f != 5 {
            let cl = cpp7_clube(cpp7_d(c + 4));
            no_fim = cl != 0 && humano(cl, 0, 1, 0, 0) != 0;
        }
        if !no_fim {
            let n = cpp7_b(oferta + 0x2e);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                let rejeitar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bf680usize);
                rejeitar(this, &mut o, 0, 6);
                return;
            }
        }
    } else {
        *((oferta + 0x2c) as *mut u8) = 0x11;
        let desassossegar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c1290usize);
        desassossegar(this, oferta);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
        let cl = cpp7_clube(cpp7_d(oferta + 8));
        if motivo as u16 == 8 {
            if humano(cl, 0, 1, 0, 0) == 0 && sorteio(cpp7_b(oferta + 0x2e) as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
            agendar(this, oferta, 7, 0);
            return;
        }
        if humano(cl, 0, 1, 0, 0) == 0 {
            let n = cpp7_b(oferta + 0x2e);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8dda90usize);
    agendar(this, oferta, 7, 0);
    noticia(this, oferta, motivo);
}

/// { 0x8c6ae0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// detach_offer_players(oferta)`: constrói uma LLIST local que não usa (0x68d020, destruída no fim por 0x68d040), tira
/// das outras ofertas o jogador da oferta (+0xc; 0x8c41c0 com a oferta, 0, 0) e, pela lista dos jogadores de troca
/// (+4, relida a cada chamada, cursor recomeçado), tira cada um (0x8c41c0) e desfaz as ofertas por ele (0x8c47d0). O
/// id do jogador de troca é relido do item depois da 0x8c41c0, como na original.
unsafe extern "thiscall" fn soltar_jogadores_da_oferta(this: usize, oferta: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c41c0usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c47d0usize);
    let mut local = [0u32; 4]; // a LLIST (0xe bytes) da original
    lista_nova(local.as_mut_ptr() as usize);
    crate::comparacao::ramo("cpp_7 soltar jogadores");
    tirar(this, cpp7_pessoa(cpp7_d(oferta + 0xc)), oferta, 0, 0); // o jogador da oferta
    let trocas = cpp7_d(oferta + 4);
    if trocas != 0 {
        core::ptr::write_unaligned((trocas + 0xa) as *mut usize, cpp7_d(trocas + 2));
        let mut item = proximo(cpp7_d(oferta + 4));
        while item != 0 {
            tirar(this, cpp7_pessoa(cpp7_d(item)), oferta, 0, 0);
            desfazer(this, cpp7_pessoa(cpp7_d(item)));
            item = proximo(cpp7_d(oferta + 4));
        }
    }
    destruir(local.as_mut_ptr() as usize);
}

/// { 0x8c8740 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH só para a oferta nova):
/// `TRANSFER_MANAGER::build_cash_offer_option(clube, jogador, saída, atração, valor)`: aloca a oferta (0x944cf1(0x41) e
/// 0x8e5680), guarda em `saída`, inicia pelo jogador, o clube comprador e o valor (0x8e78f0, que pode sortear) e põe a
/// atração em 50. Clube sem finanças (0x5a0590 = 0): a original (caixa "Error").
unsafe extern "thiscall" fn montar_oferta_em_dinheiro(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    // ponytail: a 0x5a0590 só procura; consultada antes de alocar para o caso de erro ir inteiro pela original
    if financas(cpp7_d(0xae2a38), clube) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(1026)(this, clube, jogador, saida, atracao, valor);
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x8e78f0usize);
    let p = novo(0x41);
    let oferta = if p != 0 { construir(p) } else { 0 };
    *saida = oferta;
    crate::comparacao::ramo("cpp_7 oferta em dinheiro");
    iniciar(oferta, jogador, clube, valor); // o valor da oferta
    *atracao = 0x32;
    1
}

/// Comparador: sorteia, reagenda, rejeita ou retira (pode liberar a oferta) e dá notícia: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_clube_recusa_a_oferta(this: usize, oferta: usize, clube: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1024)(this, oferta, clube, motivo)
}

/// Comparador: tira jogadores de ofertas e desfaz ofertas (libera itens): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_soltar_jogadores_da_oferta(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1025)(this, oferta)
}

/// Comparador: aloca a oferta e a 0x8e78f0 pode sortear: roda só a original e confere o resultado: devolve 1, a oferta
/// guardada em `saída` tem o valor (+0x1b) e a atração ficou em 50.
unsafe extern "thiscall" fn comparar_montar_oferta_em_dinheiro(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(1026)(this, clube, jogador, saida, atracao, valor);
    let oferta = *saida;
    let ok = r == 1 && oferta != 0 && core::ptr::read_unaligned((oferta + 0x1b) as *const i32) == valor && *atracao == 0x32;
    PLACAR.registrar(ok, || format!("montar_oferta_em_dinheiro(jogador {:x}, valor {}) devolveu {}, oferta {:x}", cpp7_d(jogador), valor, r, oferta));
    r
}

// ---------- 251a leva: as transferências: o prazo da resposta à oferta, pagar as prestações do mês e tirar a oferta (transfer_manager.cpp) ----------

unsafe fn cpo10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo10_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo10_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpo10_clube(id: usize) -> usize {
    cpo10_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpo10_pessoa(id: usize) -> usize {
    cpo10_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A regra de transferência da nação `nacao` entre as 28 de `this + 0x8ac` (a de +0xa igual), ou 0.
unsafe fn cpo10_regra(this: usize, nacao: usize) -> usize {
    for k in 0..0x1cusize {
        let r = cpo10_d(this + 0x8ac + k * 4);
        if cpo10_d(r + 0xa) == nacao {
            return r;
        }
    }
    0
}

/// { 0x8cc5b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::get_offer_response_delay(oferta, dias,
/// fatias)`: o prazo é o de fechar a janela (0x8e8ed0) da nação do comprador (+8; sem regra: 365) e, com o +4 da oferta,
/// o menor com o da nação do clube do contrato do jogador (+0xc; contrato de tipo 5 não conta). Abaixo de 3: 0 dia e 1
/// fatia; abaixo de 6 ou de 11: 1 ou 2 dias e 1..3 fatias sorteadas. Senão o valor pelo estatuto do contrato com o clube
/// (+0x3d; sem ele −100): o jogador (0x53b620 do +0x11; sem ficha, a caixa "Error") de habilidade +7 da ficha acima de
/// 130 e da reputação do clube / 60, menos de 26 anos, em menos de 3 listas e estatuto positivo, com o interesse
/// (0x846920) acima de [0x968e58]: o prazo × [0x968e18] até [0x968dd8]; senão um sorteio até estatuto/10 + 12 (o maior
/// com 1..3). Dias = valor / 3, fatias = valor % 3.
unsafe extern "thiscall" fn prazo_da_resposta_a_oferta(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let janela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8e8ed0usize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let comprador = cpo10_clube(cpo10_d(oferta + 8));
    let p = cpo10_pessoa(cpo10_d(oferta + 0xc));
    let c = contrato_de(cpo10_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 && cpo10_b(c + 0x35) & 0x3f != 5 { cpo10_clube(cpo10_d(c + 4)) } else { 0 };
    let k = com_clube(cpo10_d(0xae1050), p, dono);
    let mut prazo: u16 = 0x16d;
    let r = cpo10_regra(this, cpo10_d(comprador + 0x53));
    if r != 0 {
        prazo = janela(r) as u16;
    }
    if cpo10_d(oferta + 4) != 0 {
        let r = cpo10_regra(this, cpo10_d(dono + 0x53));
        if r != 0 && prazo >= janela(r) as u16 {
            prazo = janela(r) as u16;
        }
    }
    if prazo < 3 {
        *dias = 0;
        *fatias = 1;
        return;
    }
    if prazo < 0xb {
        *dias = if prazo < 6 { 1 } else { 2 };
        *fatias = (sortear(3) as u8).wrapping_add(1);
        return;
    }
    let estatuto: i8 = if k != 0 { cpo10_b(k + 0x3d) as i8 } else { -100 };
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x53b620 só lê o byte
    let e_jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    if e_jogador(cpo10_b(oferta + 0x11) as u32) != 0 && cpo10_d(p + 0x61) == 0 {
        aviso_em(0xa8d514, 0x29c3);
    }
    let mut valor: Option<u8> = None;
    if dono != 0 && e_jogador(cpo10_b(oferta + 0x11) as u32) != 0 {
        let hab = cpo10_h(cpo10_d(p + 0x61) + 7);
        if hab > 0x82
            && hab as i32 > cpo10_h(dono + 0x80) as i32 / 60
            && (cpo10_b(p + 0x18) as i8) < 0x1a
            && cpo10_b(cpo10_d(0xae2d0c).wrapping_add(cpo10_d(p).wrapping_mul(0x4f)) + 0xb) & 0xf < 3
            && estatuto > 0
        {
            let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
            if interesse(cpo10_d(this + 0x856), p, comprador, 0, 0, 1, 0) > f64_em(0x968e58) {
                let x = prazo as i32 as f64 * f64_em(0x968e18);
                let teto = f64_em(0x968dd8);
                valor = Some(ftol(if teto < x || x.is_nan() { teto } else { x }) as u8);
            }
        }
    }
    let v = match valor {
        Some(v) => v,
        None => {
            crate::comparacao::ramo("cpo_10 prazo sorteado");
            let s = sortear(estatuto as i32 / 10 + 0xc) as i8;
            if sortear(3) + 1 > s as i32 {
                (sortear(3) as u8).wrapping_add(1)
            } else {
                s as u8
            }
        }
    } as i8 as i32;
    *dias = (v / 3) as u8;
    *fatias = (v % 3) as u8;
}

/// { 0x8caee0 } (thiscall, ret 0, transfer_manager.cpp): `TRANSFER_MANAGER::pay_transfer_instalments()`: no fim do mês,
/// cada prestação da lista (+0x872; 13 bytes: +0 o valor, +4 o clube que paga, +8 o que recebe, +0xc as que faltam) é
/// paga (0x599890 nas finanças do pagador, 0x599800 nas do recebedor, 0x5a0590) e conta uma a menos; faltando menos de
/// 4 ou de 8, a figura das prestações (+0x1c das finanças) recebe as frações ([0x968918]/[0x968f30] ou
/// [0x967880]/[0x969518]); acabada, sai da lista (0x68d1f0); senão os dois clubes ficam marcados (0x945501). Depois, em
/// cada clube com finanças, a figura zera: sem marca; positiva só pagando; negativa só recebendo.
unsafe extern "thiscall" fn pagar_prestacoes_das_transferencias(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599890usize);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599800usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let recebe = calloc(cpo10_d(0xae235c) as u32, 1);
    let paga = calloc(cpo10_d(0xae235c) as u32, 1);
    let lista = this + 0x872;
    cpo10_w(lista + 0xa, cpo10_d(this + 0x874) as u32);
    loop {
        let e = proximo(lista);
        if e == 0 {
            break;
        }
        let fr = financas(cpo10_d(0xae2a38), cpo10_clube(cpo10_d(e + 8)));
        let fp = financas(cpo10_d(0xae2a38), cpo10_clube(cpo10_d(e + 4)));
        pagar(fp, cpo10_i(e) as i64);
        receber(fr, cpo10_i(e) as i64);
        let n = (cpo10_b(e + 0xc) as i8).wrapping_sub(1);
        *((e + 0xc) as *mut i8) = n;
        let fracoes = if n < 4 { Some((0x968918, 0x968f30)) } else if n < 8 { Some((0x967880, 0x969518)) } else { None };
        if let Some((a, b)) = fracoes {
            cpo10_w(fp + 0x1c, cpo10_i(fp + 0x1c).wrapping_add(ftol(cpo10_i(e) as f64 * f64_em(a))) as u32);
            cpo10_w(fr + 0x1c, cpo10_i(fr + 0x1c).wrapping_add(ftol(cpo10_i(e) as f64 * f64_em(b))) as u32);
        }
        if cpo10_b(e + 0xc) == 0 {
            crate::comparacao::ramo("cpo_10 prestacao acabada");
            tirar(lista, e);
        } else {
            *((recebe + cpo10_d(cpo10_clube(cpo10_d(e + 8)))) as *mut u8) = 1;
            *((paga + cpo10_d(cpo10_clube(cpo10_d(e + 4)))) as *mut u8) = 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < cpo10_i(0xae235c) {
        let f = financas(cpo10_d(0xae2a38), cpo10_d(0xae23b4).wrapping_add(i * 0x245));
        if f != 0 {
            let (r, p) = (cpo10_b(recebe + i) != 0, cpo10_b(paga + i) != 0);
            let v = cpo10_i(f + 0x1c);
            let zerar = match (r, p) {
                (false, false) => true,
                (false, true) => v > 0,
                (true, false) => v < 0,
                (true, true) => false,
            };
            if zerar {
                cpo10_w(f + 0x1c, 0);
            }
        }
        i += 1;
    }
    if recebe != 0 {
        livrar(recebe);
    }
    if paga != 0 {
        livrar(paga);
    }
}

/// { 0x8d0f60 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_offer(oferta)`: a oferta ligada
/// (+0x28 ≠ −1, em +0xc) é destruída (0x8e5710, 0x944c94), a vaga zera e a conta +0x14 baixa; com o +0x1f em 1, sai da
/// lista de empréstimos (+0x946) o par (clube +8, jogador +0xc); para cada jogador da troca (+4): a proposta de contrato
/// ligada à vaga +0x20 sai (0x4ccb30, se o jogador da oferta tem contrato) e o +2 do registro de 8 bytes dele (+0x86e)
/// baixa; sai a proposta de contrato do jogador com o comprador (0x4cca70), o +1 do registro do jogador baixa, a menor
/// vaga livre (+0x85e) acompanha, a vaga +0x20 do vetor (+0) zera, a oferta é destruída e a conta +8 baixa.
unsafe extern "thiscall" fn tirar_oferta_de_transferencia(this: usize, oferta: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    crate::comparacao::ramo("cpo_10 tirar oferta");
    if cpo10_i(oferta + 0x28) != -1 {
        let x = cpo10_d(cpo10_d(this + 0xc).wrapping_add((cpo10_i(oferta + 0x28) as usize).wrapping_mul(4)));
        if x != 0 {
            destruir(x);
            apagar(x);
        }
        cpo10_w(cpo10_d(this + 0xc).wrapping_add((cpo10_i(oferta + 0x28) as usize).wrapping_mul(4)), 0);
        cpo10_w(this + 0x14, cpo10_i(this + 0x14).wrapping_sub(1) as u32);
    }
    if cpo10_b(oferta + 0x1f) == 1 && cpo10_d(this + 0x946) != 0 {
        let l = cpo10_d(this + 0x946);
        cpo10_w(l + 0xa, cpo10_d(l + 2) as u32);
        loop {
            let e = proximo(cpo10_d(this + 0x946));
            if e == 0 {
                break;
            }
            if cpo10_d(e + 4) == cpo10_d(oferta + 8) && cpo10_d(e) == cpo10_d(oferta + 0xc) {
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                tirar(cpo10_d(this + 0x946), e);
                break;
            }
        }
    }
    if cpo10_d(oferta + 4) != 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x4ccb30usize);
        let l = cpo10_d(oferta + 4);
        cpo10_w(l + 0xa, cpo10_d(l + 2) as u32);
        loop {
            let e = proximo(cpo10_d(oferta + 4));
            if e == 0 {
                break;
            }
            let c = contrato_de(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(oferta + 0xc)));
            *(0xae1080 as *mut usize) = c;
            if c != 0 && cpo10_clube(cpo10_d(c + 4)) != 0 {
                tirar_proposta(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(e)), cpo10_i(oferta + 0x20));
            }
            let q = (cpo10_d(this + 0x86e).wrapping_add(cpo10_d(e).wrapping_mul(8)) + 2) as *mut u8;
            *q = (*q).wrapping_sub(1);
        }
    }
    let tirar_do_clube: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    tirar_do_clube(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(oferta + 0xc)), cpo10_clube(cpo10_d(oferta + 8)));
    let q = (cpo10_d(this + 0x86e).wrapping_add(cpo10_d(oferta + 0xc).wrapping_mul(8)) + 1) as *mut u8;
    *q = (*q).wrapping_sub(1); // as ofertas pela pessoa
    if cpo10_i(oferta + 0x20) < cpo10_i(this + 0x85e) {
        cpo10_w(this + 0x85e, cpo10_d(oferta + 0x20) as u32);
    }
    cpo10_w(cpo10_d(this).wrapping_add((cpo10_i(oferta + 0x20) as usize).wrapping_mul(4)), 0);
    if oferta != 0 {
        destruir(oferta);
        apagar(oferta);
    }
    cpo10_w(this + 8, cpo10_i(this + 8).wrapping_sub(1) as u32);
}

/// Os dados dos nós de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
unsafe fn cpo10_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpo10_d(lista + 2);
    while no != 0 {
        v.push(cpo10_d(no));
        no = cpo10_d(no + 6);
    }
    v
}

/// Comparador: lado a lado pelas duas saídas, o sorteio, [0xae1080] e [0xb67a34] (a caixa da falta de ficha).
unsafe extern "thiscall" fn comparar_prazo_da_resposta_a_oferta(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(994);
    let mut reg = vec![(dias as usize, 1usize), (fatias as usize, 1), (0xae1080, 4), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, oferta, dias, fatias),
        || crate::aleatorio::sem_contar(|| prazo_da_resposta_a_oferta(this, oferta, dias, fatias)),
    );
    PLACAR.registrar(igual, || format!("prazo_da_resposta_a_oferta({:#x}) {:x?} x {:x?}", oferta, mo, mr));
}

/// Comparador: só a original, conferindo o plano (feito antes): saem da lista as prestações que tinham 1 a pagar.
// ponytail: a original paga (finanças), tira itens da lista (libera) e aloca as marcas; não roda duas vezes. A cópia
// tem os ramos e a prova é pela carreira.
unsafe extern "thiscall" fn comparar_pagar_prestacoes_das_transferencias(this: usize) {
    let lista = this + 0x872;
    let antes = cpo10_itens(lista);
    let acabam = antes.iter().filter(|&&e| cpo10_b(e + 0xc) == 1).count();
    orig::<unsafe extern "thiscall" fn(usize)>(995)(this);
    let depois = cpo10_itens(lista).len();
    PLACAR.registrar(depois + acabam == antes.len(), || format!("pagar_prestacoes_das_transferencias: {} prestações, {} acabando, {} depois", antes.len(), acabam, depois));
}

/// Comparador: só a original, conferindo o plano (feito antes): a vaga zerada, as contas +8 e +0x14, a menor vaga livre
/// e o +1 do registro do jogador.
// ponytail: a original destrói e libera a oferta e mexe nas listas; não roda duas vezes.
unsafe extern "thiscall" fn comparar_tirar_oferta_de_transferencia(this: usize, oferta: usize) {
    let vaga = cpo10_i(oferta + 0x20);
    let ligada = cpo10_i(oferta + 0x28) != -1;
    let (n8, n14, livre) = (cpo10_i(this + 8), cpo10_i(this + 0x14), cpo10_i(this + 0x85e));
    let q = cpo10_d(this + 0x86e).wrapping_add(cpo10_d(oferta + 0xc).wrapping_mul(8)) + 1;
    let ofertas = cpo10_b(q);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(996)(this, oferta);
    let ok = cpo10_d(cpo10_d(this).wrapping_add((vaga as usize).wrapping_mul(4))) == 0
        && cpo10_i(this + 8) == n8 - 1
        && cpo10_i(this + 0x14) == n14 - ligada as i32
        && cpo10_i(this + 0x85e) == livre.min(vaga)
        && cpo10_b(q) == ofertas.wrapping_sub(1);
    PLACAR.registrar(ok, || format!("tirar_oferta_de_transferencia: vaga {} contas {}/{} -> {}/{} livre {} -> {} ofertas {} -> {}",
        vaga, n8, n14, cpo10_i(this + 8), cpo10_i(this + 0x14), livre, cpo10_i(this + 0x85e), ofertas, cpo10_b(q)));
}

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

// ---------- 247a leva: as regras da partida e o amistoso: o alcance das regras de escalação da partida, o motivo de um jogador não poder jogar e a vaga do amistoso marcado ----------

unsafe fn cpk4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x8d2fa0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::get_fixture_rules_scope(partida, tipo,
/// nação, continente)`: competição da partida (+0x14) de reservas ([0x9cf684]): a nação do mandante (+0x1c, +0x53), o
/// continente dela (+0x71, ou 0) e tipo 1. Amistoso (0x5c6710 sobre [0xae2c74]): 0, 0 e 0xff. Senão, pela nação-chave
/// da competição (0x669470, com sinal): a nação da tabela 0xb63d60 (72 bytes cada), o continente da competição (+0x59)
/// e tipo 4 (chaves 0 e 1), 2 (3 a 7) ou 1.
unsafe extern "thiscall" fn escopo_das_regras_da_partida(_this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let comp = cpk4_d(partida + 0x14);
    if cpk4_d(comp) == cpk4_d(0x9cf684) {
        let n = cpk4_d(cpk4_d(partida + 0x1c) + 0x53);
        *nacao = n;
        *continente = if n != 0 { cpk4_d(n + 0x71) } else { 0 };
        *tipo = 1;
        return;
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    if amistoso(cpk4_d(0xae2c74), comp) != 0 {
        *nacao = 0;
        *continente = 0;
        *tipo = 0xff;
        return;
    }
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    let k = chave(comp) as i8;
    *nacao = cpk4_d((0xb63d60isize + k as isize * 72) as usize);
    *continente = cpk4_d(cpk4_d(partida + 0x14) + 0x59);
    *tipo = match k {
        0 | 1 => 4,
        3..=7 => 2,
        _ => {
            crate::comparacao::ramo("cpk_4 escopo da nacao");
            1 // a nação da competição
        }
    };
}

/// { 0x8cf5e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_ineligibility_code(jogador, partida)`:
/// mandante (+0x1c) que não é seleção (0x53b900) e jogador ausente sem licença (AWOL: 0x418f60 sobre [0xad9c6c], com 0,
/// que tira a marca de quem está sem clube): 9. Senão, pelo alcance das regras (0x8d2fa0; 0xff e 4: 0): a primeira das
/// 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo (+0x12) responde (virtual
/// +0x18 com jogador e partida). Nenhuma: 0.
unsafe extern "thiscall" fn codigo_de_inelegibilidade(this: usize, jogador: usize, partida: usize) -> u8 {
    if partida != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(cpk4_d(partida + 0x1c)) == 0 {
            let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
            if ausente(cpk4_d(0xad9c6c), jogador, 0) != 0 {
                return 9;
            }
        }
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(0x8d2fa0usize);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff || tipo == 4 {
        return 0;
    }
    for k in 0..28 {
        let r = cpk4_d(this + 0x8ac + k * 4);
        let casa = if tipo == 2 {
            cpk4_d(r + 0xe) == continente && *((r + 0x12) as *const u8) == 2
        } else {
            cpk4_d(r + 0xa) == nacao && *((r + 0x12) as *const u8) == tipo // a regra da nação
        };
        if casa {
            crate::comparacao::ramo("cpk_4 regra da partida");
            let responder: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(cpk4_d(cpk4_d(r) + 0x18));
            return responder(r, jogador, partida);
        }
    }
    0
}

/// { 0x5c6740 } (thiscall, ret 0x10, friendly.cpp): `Friendly::find_scheduled_friendly_slot(dia, ano, clube,
/// adversário)`: na lista dos amistosos (+0x34cc; o cursor volta ao começo), o primeiro registro do clube (+0) com uma
/// vaga (25 bytes a partir de +9, [+8] delas, com sinal) do dia (+0), ano (+2) e adversário (+0x10): a vaga. Senão 0.
unsafe extern "thiscall" fn vaga_do_amistoso(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    // ponytail: a Data local da original (0x549ef0) só guarda o dia e o ano em 16 bits (o bissexto não é lido)
    let (dia, ano) = (dia as u16, ano as u16);
    let l = cpk4_d(this + 0x34cc);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpk4_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(l);
    while e != 0 {
        if cpk4_d(e) == clube {
            for i in 0..(*((e + 8) as *const i8)).max(0) as usize {
                let v = e + 25 * i + 9;
                if *((v + 2) as *const u16) == ano && *(v as *const u16) == dia && cpk4_d(v + 0x10) == adversario {
                    crate::comparacao::ramo("cpk_4 vaga do amistoso");
                    return e + 25 * i + 9; // a vaga i
                }
            }
        }
        e = proximo(l);
    }
    0
}

/// Comparador: lado a lado pelas três saídas (a 0x669470 e a 0x5c6710 só leem).
unsafe extern "thiscall" fn comparar_escopo_das_regras_da_partida(this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize)>(892);
    let regioes = [(tipo as usize, 1), (nacao as usize, 4), (continente as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, partida, tipo, nacao, continente), || escopo_das_regras_da_partida(this, partida, tipo, nacao, continente));
    PLACAR.registrar(igual, || format!("escopo_das_regras_da_partida({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparador: pelo valor. Jogador com a marca de ausente (0x418f60 com 1, que só lê): só a original (com 0 ela tira
/// a marca de quem está sem clube e a segunda chamada veria outro estado).
unsafe extern "thiscall" fn comparar_codigo_de_inelegibilidade(this: usize, jogador: usize, partida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(893);
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    if ausente(cpk4_d(0xad9c6c), jogador, 1) != 0 {
        return o(this, jogador, partida);
    }
    let r = o(this, jogador, partida);
    let n = codigo_de_inelegibilidade(this, jogador, partida);
    PLACAR.registrar(r == n, || format!("codigo_de_inelegibilidade({:x}, {:x}) original={} rust={}", jogador, partida, r, n));
    r
}

/// Comparador: pelo valor (as duas voltam o cursor da lista ao começo e param no mesmo registro).
unsafe extern "thiscall" fn comparar_vaga_do_amistoso(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize>(894)(this, dia, ano, clube, adversario);
    let n = vaga_do_amistoso(this, dia, ano, clube, adversario);
    PLACAR.registrar(r == n, || format!("vaga_do_amistoso({}/{}, {:x}, {:x}) original={:#x} rust={:#x}", dia as u16, ano as u16, clube, adversario, r, n));
    r
}

// ---------- 245a leva: as regras de transferência da nação: o texto da notícia do fim do prazo e o fim do empréstimo; o pow do game.cpp ----------

unsafe fn cpi4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// As regras de transferência (+0x8ac, 4 bytes cada) do índice `i` no gerenciador de transferências `this`.
unsafe fn cpi4_regras(this: usize, i: usize) -> usize {
    cpi4_d(this + 0x8ac + i.wrapping_mul(4))
}

/// { 0x8e05a0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::write_deadline_news_text(noticia,
/// completo, texto)`: o texto da notícia do fim do prazo de transferências fica com as regras da nação da vaga 0 da
/// notícia (0x785830): o método +0x30 delas, com os mesmos argumentos. Devolve o que ele devolve.
unsafe extern "thiscall" fn texto_do_fim_do_prazo(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(0x785830usize);
    let regras = cpi4_regras(this, vaga(noticia, 0) as usize); // as regras da vaga 0
    let metodo: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(cpi4_d(cpi4_d(regras) + 0x30));
    metodo(regras, noticia, completo, texto)
}

/// As regras (das 28 em +0x8ac) da nação do `clube` (+0x53), se o clube dono do `jogador` (+0x39) é da mesma nação;
/// senão nenhuma.
unsafe fn cpi4_regras_do_emprestimo(this: usize, clube: usize, jogador: usize) -> Option<usize> {
    let dono = cpi4_d(jogador + 0x39);
    if dono == 0 || cpi4_d(dono + 0x53) != cpi4_d(clube + 0x53) {
        return None;
    }
    let nacao = cpi4_d(clube + 0x53);
    (0..0x1c).map(|i| cpi4_regras(this, i)).find(|&r| cpi4_d(r + 0xa) == nacao) // a nação das regras
}

/// { 0x8d0530 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::can_end_loan(clube, jogador, a, b,
/// c)`: com o clube dono do jogador (+0x39) na mesma nação (+0x53) do `clube`, quem responde são as regras dessa nação
/// (a primeira das 28 em +0x8ac com +0xa igual): o método +0x24 delas com `clube`, `jogador`, `a`, `b`, `c`. Senão 1.
unsafe extern "thiscall" fn pode_encerrar_emprestimo(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    match cpi4_regras_do_emprestimo(this, clube, jogador) {
        Some(r) => {
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(cpi4_d(cpi4_d(r) + 0x24));
            metodo(r, clube, jogador, a, b, c)
        }
        None => 1,
    }
}

/// { 0x5d5a20 } (cdecl, game.cpp): `pow_double(base, expoente)`: o pow da CRT do jogo (0x945ff0), resultado em st0.
unsafe extern "C" fn pow_do_game(base: f64, expoente: f64) -> f64 {
    pow_do_jogo(base, expoente)
}

/// Comparador: o método das regras pode escrever o texto e sortear: só a original; a escolha das regras da cópia é a
/// mesma conta (a prova é a carreira e o modo Rust).
unsafe extern "thiscall" fn comparar_texto_do_fim_do_prazo(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    crate::comparacao::ramo("texto_do_fim_do_prazo");
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(855)(this, noticia, completo, texto)
}

/// Comparador: quando as regras da nação respondem, só a original (o método é delas); senão os dois devolvem 1.
unsafe extern "thiscall" fn comparar_pode_encerrar_emprestimo(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(856)(this, clube, jogador, a, b, c);
    if cpi4_regras_do_emprestimo(this, clube, jogador).is_some() {
        crate::comparacao::ramo("pode_encerrar_emprestimo pelas regras");
        return r;
    }
    PLACAR.registrar(r == 1, || format!("pode_encerrar_emprestimo({:x}, {:x}) original={} rust=1", clube, jogador, r));
    r
}

unsafe extern "C" fn comparar_pow_do_game(base: f64, expoente: f64) -> f64 {
    let r = orig::<unsafe extern "C" fn(f64, f64) -> f64>(857)(base, expoente);
    let n = pow_do_game(base, expoente);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("pow_do_game({}, {}) original={} rust={}", base, expoente, r, n));
    r
}

// ---------- 244a leva: as permissões de trabalho (transfer_manager.cpp): a oferta que precisa de permissão e a atualização anual ----------

unsafe fn cph6_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph6_pessoa(i: u32) -> usize {
    (cph6_d(0xae23bc) as usize).wrapping_add((i as usize).wrapping_mul(110))
}

unsafe fn cph6_clube(i: u32) -> usize {
    let i = i as usize;
    (cph6_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
}

/// A pessoa precisa de permissão para jogar no clube (0x8c7560, original, com 0 = sem ignorar as dadas)?
unsafe fn cph6_precisa(this: usize, pessoa: usize, clube: usize) -> bool {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    precisa(this, pessoa, clube, 0) != 0
}

/// { 0x8c7470 } (thiscall, ret 4, transfer_manager.cpp): `transfer_manager_offer_needs_work_permit(oferta)`: 1 se o
/// jogador da oferta (+0xc) precisa de permissão no clube da oferta (+8), ou se algum dos jogadores de troca (a lista
/// em +4, do começo: o cursor +0xa volta ao +2 e anda pela 0x68d1b0) precisa no clube do contrato dele (0x4cf5a0 em
/// [0xae1050], gravado em [0xae1080]; sem contrato, clube nulo); senão 0.
unsafe extern "thiscall" fn oferta_precisa_de_permissao(this: usize, oferta: usize) -> u32 {
    if cph6_precisa(this, cph6_pessoa(cph6_d(oferta + 0xc)), cph6_clube(cph6_d(oferta + 8))) {
        return 1; // o jogador da oferta
    }
    let lista = cph6_d(oferta + 4) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cph6_d(lista + 2)); // o cursor no começo
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mut item = proximo(lista);
    while item != 0 {
        let c = contrato_de(cph6_d(0xae1050) as usize, cph6_pessoa(cph6_d(item)));
        *(0xae1080 as *mut usize) = c;
        let clube = if c != 0 { cph6_clube(cph6_d(c + 4)) } else { 0 };
        if cph6_precisa(this, cph6_pessoa(cph6_d(item)), clube) {
            return 1;
        }
        item = proximo(lista);
    }
    0
}

/// { 0x8c7330 } (thiscall, transfer_manager.cpp): `TRANSFER_MANAGER::update_work_permits`: no vetor de permissões
/// ([this+0x93a], [this+0x93e] registros de 14 bytes: +0 pessoa, +8 ano, +0xa nação), a marca da pessoa (byte em
/// [this+0x86e] + 8 × id) sem o bit 0x20 é zerada inteira; com mais de 4 anos (ano do jogo − +8) a pessoa ganha a
/// segunda nacionalidade (+0x1e = nação +0xa, 0x122 bytes em [0xae23a8]), perde o bit 0x20 e a permissão sai (pessoa
/// −1); quem não é jogador (+0x61) também perde. Saindo alguma: reordena (qsort 0x945333 com 0x8e55c0) e desconta as
/// do fim com pessoa −1.
unsafe extern "thiscall" fn atualizar_permissoes_de_trabalho(this: usize) {
    let n = (this + 0x93e) as *mut i32;
    let mut saiu = false;
    let mut i = 0i32;
    let mut off = 0usize;
    while i < *n {
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(off);
        let pessoa = cph6_pessoa(cph6_d(r));
        let marca = |p: usize| ((cph6_d(this + 0x86e) as usize).wrapping_add((cph6_d(p) as usize).wrapping_mul(8))) as *mut u8;
        if *marca(pessoa) & 0x20 == 0 {
            *marca(pessoa) = 0; // a marca sem permissão
        }
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(off);
        let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(*((r + 8) as *const u16) as i32);
        if anos > 4 {
            let nacao = cph6_d(r + 0xa) as usize;
            core::ptr::write_unaligned((pessoa + 0x1e) as *mut usize, (cph6_d(0xae23a8) as usize).wrapping_add(nacao.wrapping_mul(0x122)));
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((cph6_d(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true;
        } else if cph6_d(pessoa + 0x61) == 0 {
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((cph6_d(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true; // quem não é jogador
        }
        i += 1;
        off = off.wrapping_add(0xe);
    }
    if !saiu {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cph6_d(this + 0x93a) as usize, *n, 0xe, 0x8e55c0);
    let mut k = *n - 1;
    while k >= 0 {
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(k as usize * 0xe);
        if cph6_d(r) != u32::MAX {
            break;
        }
        *n -= 1;
        k -= 1;
    }
}

/// Comparador: lado a lado pela ficha [0xae1080] e pelo cursor (+0xa) da lista de troca.
unsafe extern "thiscall" fn comparar_oferta_precisa_de_permissao(this: usize, oferta: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(840);
    let lista = cph6_d(oferta + 4) as usize;
    let mut reg = vec![(0xae1080, 4)];
    if lista != 0 {
        reg.push((lista + 0xa, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, oferta), || oferta_precisa_de_permissao(this, oferta));
    PLACAR.registrar(igual && r == n, || format!("oferta_precisa_de_permissao({:x}) original={} rust={} {:x?} x {:x?}", oferta, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da pessoa e pela segunda
/// nacionalidade (+0x1e). Permissões com pessoa negativa não entram nas regiões (o original leria fora do vetor).
unsafe extern "thiscall" fn comparar_atualizar_permissoes_de_trabalho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(841);
    let n = (*((this + 0x93e) as *const i32)).max(0) as usize;
    let vetor = cph6_d(this + 0x93a) as usize;
    let mut reg = vec![(this + 0x93a, 8), (vetor, n * 0xe)];
    for k in 0..n {
        let id = cph6_d(vetor + k * 0xe);
        if (id as i32) < 0 {
            continue;
        }
        let pessoa = cph6_pessoa(id);
        reg.push(((cph6_d(this + 0x86e) as usize).wrapping_add((cph6_d(pessoa) as usize).wrapping_mul(8)), 1));
        reg.push((pessoa + 0x1e, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || atualizar_permissoes_de_trabalho(this));
    PLACAR.registrar(igual, || format!("atualizar_permissoes_de_trabalho {:x?} x {:x?}", mo, mr));
}

// ---------- 243a leva: as cópias da notícia (news.cpp) e das transferências passada e futura (transfer_manager.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cpg1_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x786010 } (thiscall, ret 4, news.cpp): `NewsItem::operator=`: copia a notícia (0xdf bytes) de `origem`: +0
/// (dupla), +4 (byte), as 50 fichas de +5 (duplas, na ordem), a data em +0xcd, +0xd5 (byte), +0xd6 e +0xda (duplas) e
/// +0xde (byte). Devolve `this`.
unsafe extern "thiscall" fn copiar_noticia(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    d(0);
    b(4);
    for k in 0..50 {
        d(5 + 4 * k); // as fichas da notícia
    }
    cpg1_data(this + 0xcd, origem + 0xcd);
    b(0xd5);
    d(0xd6);
    d(0xda);
    b(0xde);
    this
}

/// { 0x8cb940 } (thiscall, ret 4, transfer_manager.cpp): `past_transfer_copy_construct`: copia a transferência passada
/// (0x25 bytes) de `origem`: sete duplas (+0..+0x18), o byte +0x1c e a data em +0x1d. Devolve `this`.
unsafe extern "thiscall" fn copiar_transferencia_passada(this: usize, origem: usize) -> usize {
    for k in 0..7 {
        core::ptr::write_unaligned((this + 4 * k) as *mut u32, core::ptr::read_unaligned((origem + 4 * k) as *const u32)); // as duplas da transferência
    }
    *((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8);
    cpg1_data(this + 0x1d, origem + 0x1d);
    this
}

/// { 0x8cbf40 } (thiscall, ret 4, transfer_manager.cpp): `future_transfer_assign`: copia a transferência futura (0xc
/// bytes) de `origem`: a data em +0 e o índice da oferta (+8). Devolve `this`.
unsafe extern "thiscall" fn copiar_transferencia_futura(this: usize, origem: usize) -> usize {
    cpg1_data(this, origem);
    core::ptr::write_unaligned((this + 8) as *mut u32, core::ptr::read_unaligned((origem + 8) as *const u32)); // o índice da oferta
    this
}

unsafe extern "thiscall" fn comparar_copiar_noticia(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(817);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xdf)], || o(this, origem), || copiar_noticia(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_noticia {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_transferencia_passada(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(818);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x25)], || o(this, origem), || copiar_transferencia_passada(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_transferencia_passada {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_transferencia_futura(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(819);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || copiar_transferencia_futura(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_transferencia_futura {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 213a leva: o fim do empréstimo, a contratação pelas regras da nação e as trocas que oferecem o jogador (transfer_manager.cpp) ----------

/// As regras de transferência da nação: das 28 em [gerente+0x8ac] (ponteiros), a de nação (+0xa) igual;
/// 0 se nenhuma.
unsafe fn regras_de_transferencia_da_nacao(gerente: usize, nacao: usize) -> usize {
    (0..28)
        .map(|i| *((gerente + 0x8ac + 4 * i) as *const usize))
        .find(|&r| core::ptr::read_unaligned((r + 0xa) as *const usize) == nacao)
        .unwrap_or(0)
}

/// { 0x8ced40 } (thiscall, ret 0x18, transfer_manager.cpp): se o `clube` pode contratar o `jogador`,
/// pelas regras da nação do clube (método +0x10 com clube, jogador, a4, a2 e o motivo); sem regras, 1. O
/// motivo (palavra) vai para `saida` (zerada antes). Recusado com `a5`, clube humano (0x600a50) e motivo
/// de estrangeiros (0x11, 0x14, 0x15, 0x1f): pergunta de novo com a nação do jogador (+0x1a) trocada pela
/// do clube (e devolvida depois); aceito assim, 1.
unsafe extern "thiscall" fn pode_contratar_jogador(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let regras = regras_de_transferencia_da_nacao(gerente, core::ptr::read_unaligned((clube + 0x53) as *const usize));
    if !saida.is_null() {
        *saida = 0;
    }
    if regras == 0 {
        return 1;
    }
    let consultar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut u32) -> u32 =
        core::mem::transmute(*((*(regras as *const usize) + 0x10) as *const usize));
    // o motivo é a vaga do argumento `clube` na pilha da original: sem resposta das regras fica o clube
    let mut motivo = clube as u32;
    let r = consultar(regras, clube, jogador, a4, a2, &mut motivo);
    if !saida.is_null() {
        *saida = motivo as u16;
    }
    if r != 0 {
        return r;
    }
    if a5 == 0 {
        return 0; // recusado, sem segunda pergunta
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) == 0 || !matches!(motivo as u16, 0x11 | 0x14 | 0x15 | 0x1f) {
        if !saida.is_null() {
            *saida = motivo as u16;
        }
        return 0;
    }
    let nacao = (jogador + 0x1a) as *mut u32;
    let guardada = core::ptr::read_unaligned(nacao);
    core::ptr::write_unaligned(nacao, core::ptr::read_unaligned((clube + 0x53) as *const u32));
    let r = consultar(regras, clube, jogador, a4, a2, &mut motivo);
    core::ptr::write_unaligned(nacao, guardada);
    if r != 0 {
        return 1;
    }
    if !saida.is_null() {
        *saida = motivo as u16;
    }
    0
}

/// { 0x8c5e40 } (thiscall, ret 8, transfer_manager.cpp): quantas ofertas abertas do clube do contrato do
/// `jogador` (0x4cf5a0) o oferecem na troca (lista +4 da oferta, nó com o id dele), anotando o índice
/// (+0x20) em `saida` (0x68d0d0) se houver. Ofertas no estado (+0x2c) 2, 3, 4, 0x10, 0x14 ou 0xf com
/// +0x2e ≥ 3 não contam. Para ao chegar ao número de trocas marcado ([+0x86e] + 8 × id + 2); marcado 0
/// ou id fora da faixa, 0.
unsafe extern "thiscall" fn trocas_oferecendo_o_jogador(gerente: usize, jogador: usize, saida: usize) -> u32 {
    if jogador != 0 && *(jogador as *const i32) >= *(0xae2364 as *const i32) {
        return 0;
    }
    let limite = || *((*((gerente + 0x86e) as *const usize) as isize + *(jogador as *const i32) as isize * 8 + 2) as *const u8) as u32;
    if limite() == 0 {
        return 0; // sem trocas marcadas
    }
    if clube_do_contrato(jogador) == 0 {
        return 0;
    }
    let clube = *(clube_do_contrato(jogador) as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let mut conta = 0u32;
    let mut i = 0;
    while i < *((gerente + 4) as *const i32) {
        let vaga = (*(gerente as *const usize) + 4 * i as usize) as *const usize;
        i += 1;
        let oferta = *vaga;
        if oferta == 0 || *((oferta + 8) as *const u32) != clube {
            continue;
        }
        let estado = *((oferta + 0x2c) as *const u8);
        if matches!(estado, 2 | 3 | 4 | 0x10 | 0x14) || (estado == 0xf && *((oferta + 0x2e) as *const u8) >= 3) {
            continue;
        }
        let l = *((oferta + 4) as *const usize);
        if l != 0 {
            core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
            loop {
                let no = prox(*((*vaga + 4) as *const usize));
                if no == 0 {
                    break;
                }
                if *(no as *const u32) == *(jogador as *const u32) {
                    if saida != 0 {
                        anexar(saida, *vaga + 0x20, 4);
                    }
                    conta += 1; // trocas
                    break;
                }
            }
        }
        if conta == limite() {
            break;
        }
    }
    conta
}


unsafe extern "thiscall" fn comparar_pode_contratar_jogador(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, *mut u16) -> u32>(526);
    let mut regioes = vec![];
    if !saida.is_null() {
        regioes.push((saida as usize, 2));
    }
    if jogador != 0 {
        regioes.push((jogador + 0x1a, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, clube, a2, jogador, a4, a5, saida),
        || pode_contratar_jogador(gerente, clube, a2, jogador, a4, a5, saida));
    PLACAR.registrar(igual && r == n, || format!("pode_contratar_jogador(a5 {}) original={} rust={} {:x?} x {:x?}", a5, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_trocas_oferecendo_o_jogador(gerente: usize, jogador: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(527);
    if saida != 0 {
        // ponytail: com lista de saída a original anexa (aloca); só a original
        return o(gerente, jogador, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if jogador != 0 && *(jogador as *const i32) < *(0xae2364 as *const i32) {
        let guardado = *(0xae1080 as *const usize);
        let p = clube_do_contrato(jogador);
        *(0xae1080 as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das ofertas do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            for i in 0..(*((gerente + 4) as *const i32)).max(0) as usize {
                let oferta = *((*(gerente as *const usize) + 4 * i) as *const usize);
                if oferta != 0 && *((oferta + 8) as *const u32) == clube && *((oferta + 4) as *const usize) != 0 {
                    regioes.push((*((oferta + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(gerente, jogador, saida),
        || trocas_oferecendo_o_jogador(gerente, jogador, saida));
    PLACAR.registrar(igual && r == n, || format!("trocas_oferecendo_o_jogador original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 211a leva: o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador (transfer_manager.cpp) ----------

/// O nó de uma lista do jogo (0x68d0d0/0x68d1b0): contagem u16 em +0, cabeça +2, cauda +6, cursor +0xa;
/// cada nó tem o dado em +0 e o próximo em +6.
unsafe fn dados_da_lista(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = core::ptr::read_unaligned((l + 2) as *const usize);
    while no != 0 {
        v.push(*(no as *const usize));
        no = core::ptr::read_unaligned((no + 6) as *const usize);
    }
    v
}

/// 0x4cf5a0 (o contrato do jogador `pessoa`, thiscall em [0xae1050]) gravado em [0xae1080]; devolve o
/// registro do clube do contrato ([0xae23b4] + 581 × [contrato+4]) ou 0 sem contrato.
unsafe fn clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    (*(0xae23b4 as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581))
}

/// { 0x8c6db0 } (stdcall, ret 4, transfer_manager.cpp): fecha (estado +0x50 = 4) as propostas de contrato
/// ligadas à oferta: a do jogador-alvo (0x4cc9e0 com o id +0xc e o clube +8) e a de cada jogador da troca
/// (lista +4, com o clube do contrato do alvo, 0x4cf5a0).
unsafe extern "stdcall" fn fechar_propostas_da_oferta(oferta: usize) {
    let id = *((oferta + 0xc) as *const usize);
    let pessoa = (*(0xae23bc as *const usize)).wrapping_add(id.wrapping_mul(110));
    let clube = clube_do_contrato(pessoa);
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let r = proposta(*(0xae1050 as *const usize), id as u32, *((oferta + 8) as *const u32));
    if r != 0 {
        *((r + 0x50) as *mut u8) = 4;
    }
    let l = *((oferta + 4) as *const usize);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    let mut e = prox(l);
    while e != 0 {
        // sem contrato do alvo a original lê [0] (cai); a cópia também
        let t = proposta(*(0xae1050 as *const usize), *(e as *const u32), core::ptr::read_volatile(clube as *const u32));
        if t != 0 {
            *((t + 0x50) as *mut u8) = 4;
        }
        e = prox(l);
    }
}

/// { 0x8c16b0 } (thiscall, ret 0xc, transfer_manager.cpp): põe o jogador na fila da transferência livre
/// (lista +0x92c, registro de 9 bytes: id, motivo, compensação). Emprestado (0x4cf6d0): a original
/// (caixa de erro).
unsafe fn enfileirar_transferencia_livre_com(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32, anexar: &mut dyn FnMut(usize, &[u8; 9])) -> bool {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if emprestado(*(0xae1050 as *const usize), pessoa) != 0 {
        return false;
    }
    let mut r = [0u8; 9];
    r[..4].copy_from_slice(&(*(pessoa as *const u32)).to_le_bytes());
    r[4] = motivo;
    r[5..].copy_from_slice(&compensacao.to_le_bytes());
    anexar(gerente + 0x92c, &r);
    true
}

unsafe extern "thiscall" fn enfileirar_transferencia_livre(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let anexar_9: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    if !enfileirar_transferencia_livre_com(gerente, pessoa, motivo, compensacao, &mut |l, r| anexar_9(l, r.as_ptr() as usize, 9)) {
        orig::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(515)(gerente, pessoa, motivo, compensacao);
    }
}

/// { 0x8c5fc0 } (thiscall, ret 8, transfer_manager.cpp): conta as negociações ([+0xc], [+0x10] delas) do
/// clube do contrato do jogador (+8) cuja oferta ([gerente] + 4 × [+0x20]) segue aberta (estados 2, 3, 4,
/// 0x10 e 0x14 não; 0xf só com +0x2e < 3) e que têm o jogador na lista da troca (+4); com `saida`, põe
/// nela o índice da oferta (0x68d0d0, 4 bytes). Jogador fora do banco ou sem contrato: 0.
unsafe extern "thiscall" fn negociacoes_de_troca_do_jogador(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    if pessoa != 0 && *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return 0;
    }
    // como `clube_do_contrato`, à parte para a mutação desta rotina não cair na 0x8c6db0
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    let p = (*(0xae23b4 as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581));
    if p == 0 {
        return 0;
    }
    let clube = *(p as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let negociacoes = *((gerente + 0xc) as *const usize);
    let mut total = 0;
    let mut i = 0;
    while i < *((gerente + 0x10) as *const i32) {
        let neg = *((negociacoes + i as usize * 4) as *const usize);
        i += 1;
        if neg == 0 || *((neg + 8) as *const u32) != clube {
            continue;
        }
        let k = *((neg + 0x20) as *const usize);
        let oferta = *((*(gerente as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if oferta == 0 {
            continue;
        }
        match *((oferta + 0x2c) as *const u8) {
            2 | 3 | 4 | 0x10 | 0x14 => continue,
            0xf if *((oferta + 0x2e) as *const u8) >= 3 => continue,
            _ => {}
        }
        let l = *((neg + 4) as *const usize);
        if l == 0 {
            continue;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
        let mut e = prox(l);
        while e != 0 && *(e as *const u32) != *(pessoa as *const u32) {
            e = prox(l);
        }
        if e == 0 {
            continue;
        }
        if saida != 0 {
            anexar(saida, neg + 0x20, 4);
        }
        total += 1;
    }
    total
}

unsafe extern "stdcall" fn comparar_fechar_propostas_da_oferta(oferta: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(514);
    // as propostas que podem fechar: as do id do alvo e dos ids da troca (o clube não filtra aqui)
    let mut ids = vec![*((oferta + 0xc) as *const u32)];
    let l = *((oferta + 4) as *const usize);
    if l != 0 {
        ids.extend(dados_da_lista(l).iter().map(|&e| *(e as *const u32)));
    }
    let propostas = *((*(0xae1050 as *const usize) + 0x34) as *const usize);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if l != 0 {
        regioes.push((l + 0xa, 4));
    }
    if propostas != 0 {
        regioes.push((propostas + 0xa, 4));
        for r in dados_da_lista(propostas) {
            if ids.contains(&*(r as *const u32)) {
                regioes.push((r + 0x50, 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(oferta), || fechar_propostas_da_oferta(oferta));
    PLACAR.registrar(igual, || format!("fechar_propostas_da_oferta(id {}) {:x?} x {:x?}", ids[0], mo, mr));
}

unsafe extern "thiscall" fn comparar_enfileirar_transferencia_livre(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(515);
    // ponytail: a fila aloca um nó (não roda duas vezes): a cópia só monta o registro (0x4cf6d0 é
    // consulta pura), a original põe na fila e confere-se o nó novo da cauda
    let mut registro = None;
    if !enfileirar_transferencia_livre_com(gerente, pessoa, motivo, compensacao, &mut |_, r| registro = Some(*r)) {
        return o(gerente, pessoa, motivo, compensacao);
    }
    let l = gerente + 0x92c;
    let antes = core::ptr::read_unaligned(l as *const u16);
    o(gerente, pessoa, motivo, compensacao);
    let depois = core::ptr::read_unaligned(l as *const u16);
    let cauda = core::ptr::read_unaligned((l + 6) as *const usize);
    let novo = if cauda == 0 { None } else { Some(*(*(cauda as *const usize) as *const [u8; 9])) };
    PLACAR.registrar(depois == antes.wrapping_add(1) && novo == registro,
        || format!("enfileirar_transferencia_livre {} -> {} original={:x?} rust={:x?}", antes, depois, novo, registro));
}

unsafe extern "thiscall" fn comparar_negociacoes_de_troca_do_jogador(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(516);
    if saida != 0 {
        // ponytail: com lista de saída (só a tela 0x47ec79) a original anexa (aloca); só a original
        return o(gerente, pessoa, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if pessoa != 0 && *(pessoa as *const i32) < *(0xae2364 as *const i32) {
        let guardado = *(0xae1080 as *const usize);
        let p = clube_do_contrato(pessoa);
        *(0xae1080 as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das negociações do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            let negociacoes = *((gerente + 0xc) as *const usize);
            for i in 0..(*((gerente + 0x10) as *const i32)).max(0) as usize {
                let neg = *((negociacoes + i * 4) as *const usize);
                if neg != 0 && *((neg + 8) as *const u32) == clube && *((neg + 4) as *const usize) != 0 {
                    regioes.push((*((neg + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, pessoa, saida),
        || negociacoes_de_troca_do_jogador(gerente, pessoa, saida));
    PLACAR.registrar(igual && r == n, || format!("negociacoes_de_troca_do_jogador original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 211a leva: as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia (transfer_manager.cpp, shortlist_manager.cpp, comp_util.cpp) ----------

/// A oferta do gerente de transferências para a pessoa `id`: a primeira das [this+4] em [this] com
/// +0xc = `id`, ou 0.
unsafe fn oferta_pelo_jogador(this: usize, id: i32) -> usize {
    let ofertas = *(this as *const usize);
    let n = *((this + 4) as *const i32);
    (0..n.max(0) as usize)
        .map(|i| *((ofertas + 4 * i) as *const usize))
        .find(|&e| e != 0 && *((e + 0xc) as *const i32) == id)
        .unwrap_or(0)
}

/// { 0x8c5cf0 } (thiscall, ret 0xc, transfer_manager.cpp): conta as ofertas pela pessoa e, com
/// `lista`, acrescenta nela (0x68d0d0, 4 bytes) o índice de cada uma. Id a partir de 0xae2364 ou sem
/// oferta: 0. A oferta achada sem lista própria (+0) conta 1 (acrescenta +0x20); com lista, percorre
/// os índices dela (cursor +0xa reiniciado de +2) e conta os que apontam para ofertas da mesma pessoa.
/// Sem `todas`, ficam de fora as encerradas (0x8d0d90, aqui embutida como na original).
unsafe extern "thiscall" fn ofertas_pelo_jogador(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    // a original lê o id antes de testar o ponteiro
    let id = core::ptr::read_volatile(pessoa);
    if !pessoa.is_null() && id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let e = oferta_pelo_jogador(this, id);
    if e == 0 {
        return 0;
    }
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    let l = *(e as *const usize);
    if l == 0 {
        if todas == 0 && oferta_fechada(e) != 0 {
            return 0;
        }
        if lista != 0 {
            acrescentar(lista, e + 0x20, 4);
        }
        return 1;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    let mut conta = 0u32;
    loop {
        let no = proximo(*(e as *const usize)) as usize;
        if no == 0 {
            break;
        }
        let k = *(no as *const usize);
        let o = *((*(this as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if o == 0 || *((o + 0xc) as *const i32) != id {
            continue;
        }
        if todas == 0 && oferta_fechada(o) != 0 {
            continue;
        }
        if lista != 0 {
            acrescentar(lista, no, 4);
        }
        conta += 1;
    }
    conta
}

/// O registro de 9 bytes do clube (id em +0) na lista de compras [this+0xdc].
unsafe fn registro_da_lista_de_compras(this: usize, clube: usize) -> usize {
    (*(clube as *const i32) as usize).wrapping_mul(9).wrapping_add(*((this + 0xdc) as *const usize))
}

/// { 0x83bf40 } (thiscall, ret 4, shortlist_manager.cpp): a reputação que o clube procura nas
/// contratações, gravada na palavra +5 do registro dele: +0x73 / 10 + 4500; se não for de humano
/// (0x600a50 com 1, 1), o técnico (+0xcf) ajusta por (+0x57 − ficha +0x1d) × 0,0125; o presidente
/// (+0xbf, com ficha) por (+0x57 − ficha +0x1d) / 60 e por (ficha +0x20 − 10) × 0,025 (cada um
/// multiplica 1 + fator, truncando). Nunca abaixo de +0x80 nem acima de 10000. Devolve o valor.
unsafe extern "thiscall" fn reputacao_alvo_do_clube(this: usize, clube: usize) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let r = registro_da_lista_de_compras(this, clube);
    let alvo = (r + 5) as *mut i16;
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let w = || core::ptr::read_unaligned(alvo) as i32;
    let por = |dif: i32, k: usize, v: i32| ftol((dif as f64 * f64_em(k) + f64_em(0x967890)) * v as f64);
    core::ptr::write_unaligned(alvo, (core::ptr::read_unaligned((clube + 0x73) as *const i32) / 10 + 0x1194) as i16);
    if humano(clube, 1, 1, 0, 0) == 0 {
        let tecnico = u(clube + 0xcf);
        if tecnico != 0 {
            let dif = byte(tecnico, 0x57) as i32 - byte(u(tecnico + 0x69), 0x1d) as i32;
            core::ptr::write_unaligned(alvo, por(dif, 0x96a740, w()) as i16);
        }
    }
    let p = u(clube + 0xbf);
    if p != 0 && u(p + 0x69) != 0 {
        let dif = byte(p, 0x57) as i32 - byte(u(p + 0x69), 0x1d) as i32;
        let v = por(dif, 0x968958, w()) as i16;
        core::ptr::write_unaligned(alvo, v);
        let ficha = u(u(clube + 0xbf) + 0x69);
        core::ptr::write_unaligned(alvo, por(byte(ficha, 0x20) as i32 - 10, 0x967878, v as i32) as i16);
    }
    let piso = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    let v = core::ptr::read_unaligned(alvo);
    let v = if piso > v { piso } else if v > 0x2710 { 0x2710 } else { v };
    core::ptr::write_unaligned(alvo, v);
    v as i32
}

/// { 0x4afeb0 } (cdecl, comp_util.cpp; a convenção do mapa dá o 2º argumento como sem uso, mas é a
/// notícia): grava o jogo em 18 dados seguidos da notícia (0x785790), a partir do dado [cursor], e
/// avança o cursor 18. Ordem: +0xc, +0x10, +0x3d, +0x3e, +4, +0, +0x42, +0x32, +0x30, +0x43, +0x44,
/// +0x4b, +0x4c, +0x47, +0x48, +0x2a, +0x28, +8 (bytes e palavras +0x30/+0x32 com sinal; +0x28 e
/// +0x2a sem). Devolve o que a última gravação devolveu. A original guarda o índice do 18º dado no
/// byte baixo do próprio argumento `cursor` (só a parte baixa chega à 0x785790).
unsafe extern "C" fn gravar_jogo_na_noticia(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let d = |o: usize| core::ptr::read_unaligned((jogo + o) as *const u32);
    let b = |o: usize| byte(jogo, o) as i32 as u32;
    let s = |o: usize| core::ptr::read_unaligned((jogo + o) as *const i16) as i32 as u32;
    let z = |o: usize| core::ptr::read_unaligned((jogo + o) as *const u16) as u32;
    let valores = [
        d(0xc), d(0x10), b(0x3d), b(0x3e), d(4), d(0), b(0x42), s(0x32), s(0x30),
        b(0x43), b(0x44), b(0x4b), b(0x4c), b(0x47), b(0x48), z(0x2a), z(0x28), d(8),
    ];
    let mut k = *cursor;
    let mut r = 0;
    for v in valores {
        r = gravar(noticia, k as i8, v);
        k = k.wrapping_add(1);
    }
    *cursor = k;
    r
}

unsafe extern "thiscall" fn comparar_ofertas_pelo_jogador(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, usize, u32) -> u32>(520);
    // ponytail: com `lista` a original aloca nós (0x68d0d0) e não pode rodar duas vezes; só ela
    if lista != 0 || pessoa.is_null() {
        return o(this, pessoa, lista, todas);
    }
    // a original só mexe no cursor da lista da oferta achada
    let e = oferta_pelo_jogador(this, *pessoa);
    let l = if e == 0 { 0 } else { *(e as *const usize) };
    let reg = if l == 0 { vec![] } else { vec![(l, 0xe)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, lista, todas), || ofertas_pelo_jogador(this, pessoa, lista, todas));
    PLACAR.registrar(igual && r == n, || format!("ofertas_pelo_jogador({}, todas {}) original={} rust={} {:x?} x {:x?}", *pessoa, todas, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_reputacao_alvo_do_clube(this: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(521);
    let reg = [(registro_da_lista_de_compras(this, clube) + 5, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || reputacao_alvo_do_clube(this, clube));
    PLACAR.registrar(igual && r == n, || format!("reputacao_alvo_do_clube({}) original={} rust={} {:x?} x {:x?}", *(clube as *const i32), r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_gravar_jogo_na_noticia(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, *mut u8) -> u32>(522);
    let k = *cursor as i8;
    // índice fora de 0..49 em algum dos 18: a 0x785790 abre a caixa de erro; só a original
    if !(0..0x32 - 17).contains(&k) {
        return o(jogo, noticia, cursor);
    }
    let reg = [(noticia + 5, 200), (cursor as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jogo, noticia, cursor), || gravar_jogo_na_noticia(jogo, noticia, cursor));
    PLACAR.registrar(igual && r == n, || format!("gravar_jogo_na_noticia(cursor {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

// ---------- 193a leva: as cláusulas do contrato no gerente de transferências: percentual da próxima venda, bônus por jogo e bônus pela seleção (transfer_manager.cpp) ----------

/// O comum das três: desliga o bit `bit` das marcas do jogador `*id` (+0x86e, 8 bytes por pessoa) e
/// tira o registro dele (13 bytes, o id em `chave`) da lista [gerente+`lista`] (quantos em
/// [gerente+`quantos`]), puxando os seguintes uma vaga para trás.
unsafe fn tirar_da_lista_de_pagamentos(gerente: usize, id: *const i32, bit: u8, lista: usize, quantos: usize, chave: usize) {
    let marcas = (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marcas & bit != 0 {
        *marcas -= bit;
    }
    let n = *((gerente + quantos) as *const i32);
    let base = *((gerente + lista) as *const usize);
    let Some(i) = (0..n.max(0) as usize).find(|&i| core::ptr::read_unaligned((base + i * 13 + chave) as *const i32) == *id) else {
        return;
    };
    let mut j = i + 1;
    while (j as i32) < *((gerente + quantos) as *const i32) {
        let origem = *((gerente + lista) as *const usize) + j * 13;
        core::ptr::copy(origem as *const u8, (base + (j - 1) * 13) as *mut u8, 13);
        j += 1;
    }
    *((gerente + quantos) as *mut i32) -= 1;
}

/// { 0x8cb4f0 } (thiscall, ret 4, transfer_manager.cpp): tira o percentual da próxima venda (bit 8;
/// lista +0x880, quantos +0x888, o id em +9).
unsafe extern "thiscall" fn tirar_percentual_da_proxima_venda(gerente: usize, id: *const i32) {
    tirar_da_lista_de_pagamentos(gerente, id, 8, 0x880, 0x888, 9);
}

/// { 0x8cb590 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo no time principal (bit 2;
/// lista +0x88c, quantos +0x894, o id em +0).
unsafe extern "thiscall" fn tirar_bonus_por_jogo(gerente: usize, id: *const i32) {
    tirar_da_lista_de_pagamentos(gerente, id, 2, 0x88c, 0x894, 0);
}

/// { 0x8cb630 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo pela seleção (bit 4;
/// lista +0x898, quantos +0x8a0, o id em +0).
unsafe extern "thiscall" fn tirar_bonus_pela_selecao(gerente: usize, id: *const i32) {
    tirar_da_lista_de_pagamentos(gerente, id, 4, 0x898, 0x8a0, 0);
}

/// As regiões das três: o byte das marcas, a lista inteira e o contador.
unsafe fn regioes_da_lista_de_pagamentos(gerente: usize, id: *const i32, lista: usize, quantos: usize) -> Vec<(usize, usize)> {
    let marcas = (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as usize;
    let n = (*((gerente + quantos) as *const i32)).max(0) as usize;
    let mut r = vec![(marcas, 1), (gerente + quantos, 4)];
    if n > 0 {
        r.push((*((gerente + lista) as *const usize), n * 13));
    }
    r
}

unsafe fn comparar_lista_de_pagamentos(nome: &str, k: usize, rust: unsafe extern "thiscall" fn(usize, *const i32), gerente: usize, id: *const i32, lista: usize, quantos: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(k);
    let reg = regioes_da_lista_de_pagamentos(gerente, id, lista, quantos);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || rust(gerente, id));
    PLACAR.registrar(igual, || format!("{}({}) {:x?} x {:x?}", nome, *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_tirar_percentual_da_proxima_venda(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("tirar_percentual_da_proxima_venda", 451, tirar_percentual_da_proxima_venda, gerente, id, 0x880, 0x888);
}

unsafe extern "thiscall" fn comparar_tirar_bonus_por_jogo(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("tirar_bonus_por_jogo", 452, tirar_bonus_por_jogo, gerente, id, 0x88c, 0x894);
}

unsafe extern "thiscall" fn comparar_tirar_bonus_pela_selecao(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("tirar_bonus_pela_selecao", 453, tirar_bonus_pela_selecao, gerente, id, 0x898, 0x8a0);
}

// ---------- 191a leva: o clube do contrato acertado, o jogo pela seleção e a punição guardada (transfer_manager.cpp, discipline.cpp) ----------

/// { 0x8c5450 } (thiscall, ret 8, transfer_manager.cpp): com a marca de contrato acertado (0x40) da
/// pessoa `*id`, procura o contrato (0x4cca30 em [0xae1050]): achou, grava em `clube` (se não nulo) o
/// clube do contrato ([0xae23b4], 581 bytes por clube, pelo id em +4) e devolve 1; não achou, desliga a
/// marca e devolve 0.
unsafe extern "thiscall" fn clube_do_contrato_acertado(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let marcas = |g: usize| (*((g + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marcas(gerente) & 0x40 == 0 {
        return 0;
    }
    let procurar: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x4cca30usize);
    let contrato = procurar(*(0xae1050 as *const usize), *id);
    if contrato == 0 {
        *marcas(gerente) &= 0xbf;
        return 0;
    }
    if !clube.is_null() {
        *clube = (*(0xae23b4 as *const usize) as isize + *((contrato + 4) as *const i32) as isize * 581) as usize;
    }
    1
}

/// O registro de 8 bytes da pessoa `*id` no gerente de transferências (+0x86e).
unsafe fn registro_da_pessoa(gerente: usize, id: *const i32) -> usize {
    (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as usize
}

/// { 0x8ce920 } (thiscall, ret 8, transfer_manager.cpp): registra que o jogador jogou o jogo `numero`
/// da seleção: os bits dos últimos jogos (+3, 16 bits) andam um por jogo desde o último registrado
/// (+5), o mais novo é ligado e o número fica em +5.
unsafe extern "thiscall" fn registrar_jogo_pela_selecao(gerente: usize, id: *const i32, numero: i16) {
    let r = registro_da_pessoa(gerente, id);
    let bits = (r + 3) as *mut u16;
    let ultimo = (r + 5) as *mut i16;
    let d = numero.wrapping_sub(core::ptr::read_unaligned(ultimo));
    for _ in 0..d.max(0) {
        core::ptr::write_unaligned(bits, core::ptr::read_unaligned(bits) << 1);
        *(bits as *mut u8) &= 0xfe;
    }
    *(bits as *mut u8) |= 1;
    core::ptr::write_unaligned(ultimo, numero);
}

/// { 0x55cb60 } (thiscall, ret 8, discipline.cpp): a punição guardada do jogador `id` na vaga `vaga`:
/// percorre a lista das guardadas (+0x1c, do começo, 0x68d1b0) enquanto o jogador tem alguma
/// (byte +0x1c do registro de 29 bytes em [+0x14]) e devolve a de mesmo jogador e vaga; 0 se nenhuma.
unsafe extern "thiscall" fn punicao_guardada(disciplina: usize, id: i32, vaga: i8) -> usize {
    let lista = disciplina + 0x1c;
    let registro = (*((disciplina + 0x14) as *const usize) as isize + id as isize * 29) as usize;
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let proxima: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut p = proxima(lista);
    while p != 0 {
        if *((registro + 0x1c) as *const i8) <= 0 {
            return 0;
        }
        if *(p as *const i32) == id && *((p + 4) as *const i8) == vaga {
            return p;
        }
        p = proxima(lista);
    }
    0
}

unsafe extern "thiscall" fn comparar_clube_do_contrato_acertado(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, *mut usize) -> u32>(445);
    let mut reg = vec![(registro_da_pessoa(gerente, id), 1)];
    if !clube.is_null() {
        reg.push((clube as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, clube), || clube_do_contrato_acertado(gerente, id, clube));
    PLACAR.registrar(igual && ro == rr, || format!("clube_do_contrato_acertado({}) original={} rust={} {:x?} x {:x?}", *id, ro, rr, mo, mr));
    ro
}

unsafe extern "thiscall" fn comparar_registrar_jogo_pela_selecao(gerente: usize, id: *const i32, numero: i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, i16)>(446);
    let reg = [(registro_da_pessoa(gerente, id) + 3, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, numero), || registrar_jogo_pela_selecao(gerente, id, numero));
    PLACAR.registrar(igual, || format!("registrar_jogo_pela_selecao({}, {}) {:x?} x {:x?}", *id, numero, mo, mr));
}

unsafe extern "thiscall" fn comparar_punicao_guardada(disciplina: usize, id: i32, vaga: i8) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i8) -> usize>(447);
    // a lista guarda o cursor: as duas partem do mesmo e têm de deixá-lo igual
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(disciplina + 0x1c, 0x12)], || o(disciplina, id, vaga), || punicao_guardada(disciplina, id, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("punicao_guardada({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", id, vaga, ro, rr, mo, mr));
    ro
}

// ---------- 185a leva: a marca de contrato acertado e a vaga da oferta (transfer_manager.cpp) ----------

/// O byte das marcas da pessoa `*id` no gerente de transferências (+0x86e, 8 bytes por pessoa).
unsafe fn marcas_da_pessoa(gerente: usize, id: *const i32) -> *mut u8 {
    (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8
}

/// { 0x8c5410 } (thiscall, ret 4, transfer_manager.cpp): liga a marca de contrato acertado (0x40).
unsafe extern "thiscall" fn marcar_contrato_acertado(gerente: usize, id: *const i32) {
    *marcas_da_pessoa(gerente, id) |= 0x40;
}

/// { 0x8c5430 } (thiscall, ret 4, transfer_manager.cpp): desliga a marca de contrato acertado.
unsafe extern "thiscall" fn desmarcar_contrato_acertado(gerente: usize, id: *const i32) {
    *marcas_da_pessoa(gerente, id) &= 0xbf;
}

/// { 0x8c61e0 } (thiscall, ret 4, transfer_manager.cpp): o endereço da vaga `k` das ofertas (+0,
/// +4 quantas); fora da faixa, o próprio começo da tabela (o valor de +0, não o endereço).
unsafe extern "thiscall" fn vaga_da_oferta(tabela: usize, k: i32) -> usize {
    let base = *(tabela as *const usize);
    if k < *((tabela + 4) as *const i32) { (base as isize + 4 * k as isize) as usize } else { base }
}

unsafe extern "thiscall" fn comparar_marcar_contrato_acertado(gerente: usize, id: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(427);
    let a = marcas_da_pessoa(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || marcar_contrato_acertado(gerente, id));
    PLACAR.registrar(igual, || format!("marcar_contrato_acertado({}) {:x?} x {:x?}", *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_desmarcar_contrato_acertado(gerente: usize, id: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(428);
    let a = marcas_da_pessoa(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || desmarcar_contrato_acertado(gerente, id));
    PLACAR.registrar(igual, || format!("desmarcar_contrato_acertado({}) {:x?} x {:x?}", *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_vaga_da_oferta(tabela: usize, k: i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(429)(tabela, k);
    let n = vaga_da_oferta(tabela, k);
    PLACAR.registrar(r == n, || format!("vaga_da_oferta({}) original={:#x} rust={:#x}", k, r, n));
    r
}

