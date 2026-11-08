// ---------- %%LEVA%%a leva: a oferta de transferência posta no vetor das ofertas e o empréstimo aceito pelo clube (transfer_manager.cpp) ----------

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
unsafe fn contract_club_excluding_type5(pessoa: usize) -> usize {
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
unsafe fn find_offer_slot_and_rival(v: usize, lidas: i32, cap: i32, conta: i32, alvo: usize, comprador: usize, inicial: i32, primeira: bool) -> (i32, i32) {
    let (mut vaga, mut rival, mut contadas) = (inicial, -1i32, 0i32);
    let mut i = 0i32;
    while i < cap {
        let e = if i < lidas { cqf1_d(v + 4 * i as usize) } else { 0 };
        if e != 0 {
            contadas += 1;
            if cqf1_d(e + 0xc) == alvo {
                if cqf1_d(e + 8) == comprador {
                    crate::comparison::branch("cqf_1 mesma vaga");
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
unsafe fn show_out_of_memory_box(linha: u32, texto: usize) {
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
/// fim do jogo). A vaga (`find_offer_slot_and_rival`); nenhuma: a caixa "Error" (linha 0x511) e nada. A oferta antiga da vaga sai
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
unsafe extern "thiscall" fn add_transfer_offer(this: usize, oferta: usize, termos: u32) {
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
                show_out_of_memory_box(0x4d2, 0xa8e940);
                *(0xb67a34 as *mut u32) = 0;
                fechar1();
                fechar2();
                sair(-1);
            }
            core::ptr::write_bytes((p + 4 * cap as usize - 200) as *mut u32, 0, 0x32);
        }
    }
    let cap = cqf1_d(this + 4) as i32;
    let (vaga, rival) = find_offer_slot_and_rival(cqf1_d(this), cap, cap, cqf1_d(this + 8) as i32, alvo, comprador, inicial, primeira);
    if vaga == -1 {
        show_error_at(0xa8d514, 0x511);
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
        crate::comparison::branch("cqf_1 ofertas rivais ligadas");
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
    let vendedor = contract_club_excluding_type5(cqf1_pessoa(alvo));
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
        crate::comparison::branch("cqf_1 agenda pelo prazo");
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
/// notícia). Confere o plano feito antes: a vaga (`find_offer_slot_and_rival`, com o vetor que cresce quando está cheio) e o número
/// de série gravados na oferta de entrada e a série que sobe.
unsafe extern "thiscall" fn compare_add_transfer_offer(this: usize, oferta: usize, termos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let (alvo, comprador) = (cqf1_d(oferta + 0xc), cqf1_d(oferta + 8));
    let primeira = cqf1_b(cqf1_d(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    let (lidas, conta) = (cqf1_d(this + 4) as i32, cqf1_d(this + 8) as i32);
    let cap = if conta == lidas { lidas.wrapping_add(0x32) } else { lidas };
    let (vaga, _) = find_offer_slot_and_rival(cqf1_d(this), lidas, cap, conta, alvo, comprador, cqf1_d(this + 0x85e) as i32, primeira);
    let serie = cqf1_d(this + 0x85a);
    o(this, oferta, termos); // ponytail: só a original
    if vaga == -1 {
        return;
    }
    let (v, s, s2) = (cqf1_d(oferta + 0x20) as i32, cqf1_d(oferta + 0x24), cqf1_d(this + 0x85a));
    SCOREBOARD.record(v == vaga && s == serie && s2 == serie.wrapping_add(1), || format!(
        "add_transfer_offer(alvo {}, comprador {}): vaga original {} plano {}, série {} plano {}, próxima {} plano {}",
        alvo, comprador, v, vaga, s, serie, s2, serie.wrapping_add(1)));
}

/// O status da oferta deixa seguir (0x8c0660): 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3.
unsafe fn is_offer_settled(e: usize) -> bool {
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
/// agendada para 10 dias (0x8c6220). Se ela e todas as ofertas pelo jogador (+0) seguem (`is_offer_settled`), o bit 0 do
/// registro de 8 bytes do jogador ([this+0x86e]) apaga.
unsafe extern "thiscall" fn accept_loan_offer(this: usize, vaga: *mut usize, marcar: u32) {
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cc930usize);
    let noticia_aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d76d0usize);
    let noticia_recusa: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d8310usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    crate::comparison::branch("cqf_1 aceitar empréstimo");
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
    if !is_offer_settled(o) {
        return;
    }
    let lista = cqf1_d(o);
    if lista != 0 {
        cqf1_pd(lista + 0xa, cqf1_d(lista + 2));
        let mut n = proximo(cqf1_d(o));
        while n != 0 {
            if !is_offer_settled(cqf1_d(cqf1_d(this).wrapping_add(4 * cqf1_d(n)))) {
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
unsafe extern "thiscall" fn compare_accept_loan_offer(this: usize, vaga: *mut usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32)>(%%I1%%)(this, vaga, marcar) // ponytail: só a original
}
