use super::*;

// ---------- 270a leva: a oferta de transferência posta no vetor das ofertas e o empréstimo aceito pelo clube (transfer_manager.cpp) ----------







/// Person -> club of their contract (get_contract, result stored in G_CONTRACT_LOOKUP_RESULT); 0 with no contract or a
/// type-5 contract (+0x35 & 0x3f).
///
/// [pt] O clube do contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; guardado em
/// [0xae1080]); sem contrato ou com o de tipo 5, 0.
pub(crate) unsafe fn contract_club_excluding_type5(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) }
}

/// Scans the offer pointer vector for the slot of a new offer (stored lowest free slot, same buyer+target slot, or
/// first empty) and the first rival offer by another buyer for the same target; returns (slot, rival).
///
/// [pt] A vaga da oferta no vetor de ponteiros `v` (capacidade `cap`; as vagas de `lidas` em diante contam como vazias: o
/// vetor que acabou de crescer): a vaga de `inicial` (a menor livre guardada em +0x85e), senão a primeira vazia; a
/// oferta do mesmo comprador pelo mesmo alvo dá a vaga dela; a primeira oferta de outro comprador pelo alvo é a rival.
/// Para na vaga vazia quando já contou as `conta` ofertas ou quando é a primeira oferta pelo alvo. Devolve (vaga, rival).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn find_offer_slot_and_rival(v: usize, lidas: i32, cap: i32, conta: i32, alvo: usize, comprador: usize, inicial: i32, primeira: bool) -> (i32, i32) {
    let (mut vaga, mut rival, mut contadas) = (inicial, -1i32, 0i32);
    let mut i = 0i32;
    while i < cap {
        let e = if i < lidas { read_u32_as_usize(v + 4 * i as usize) } else { 0 };
        if e != 0 {
            contadas += 1;
            if read_u32_as_usize(e + 0xc) == alvo {
                if read_u32_as_usize(e + 8) == comprador {
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

/// Shows transfer_manager.cpp's "Memory Allocation Error" box with version, file, line and text (sprintf into 0xae24d0,
/// translated title/body, GUI message box).
///
/// [pt] A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (0x90d130 com a linha e
/// o texto) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8) em 0xae26c4 e a caixa 0x5e8290.
pub(crate) unsafe fn show_out_of_memory_box(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// puts the offer in an array slot (the stored one, the same buyer's for the same target or the first empty one; grows
/// by 50 when full), copies the fields, gives the serial number, links rival offers for the player, schedules the
/// answer (28 days with a human, otherwise the reply deadline) and posts the news
/// Original: `TRANSFER_MANAGER::add_transfer_offer` (0x8be300, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8be300 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: só os `new` da
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
pub(crate) unsafe extern "thiscall" fn add_transfer_offer(this: usize, oferta: usize, termos: u32) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_OFFER);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let copiar_lista: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_COPY_FROM);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let prazo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_RESPONSE_DELAY);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let noticia_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_OFFER_NEWS);
    let noticia_oferta: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_MADE_NEWS);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let observar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCOUT_MANAGER_ADD_TO_SHORTLIST);

    let alvo = read_u32_as_usize(oferta + 0xc);
    let comprador = read_u32_as_usize(oferta + 8);
    let inicial = read_u32_as_usize(this + 0x85e) as i32;
    write_u32_from_usize(this + 0x85e, u32::MAX as usize);
    let primeira = read_u8(read_u32_as_usize(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    if read_u32_as_usize(oferta + 0x1b) == 0 {
        write_u8(oferta + 0x40, 0);
    }
    if read_u32_as_usize(this + 8) == read_u32_as_usize(this + 4) {
        let cap = (read_u32_as_usize(this + 4) as u32).wrapping_add(0x32);
        write_u32_from_usize(this + 4, cap as usize);
        if read_u32_as_usize(this) == 0 {
            write_u32_from_usize(this, calloc(cap, 4));
        } else {
            let p = realocar(read_u32_as_usize(this), cap.wrapping_mul(4));
            write_u32_from_usize(this, p);
            if p == 0 {
                show_out_of_memory_box(0x4d2, 0xa8e940);
                *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                fechar1();
                fechar2();
                sair(-1);
            }
            core::ptr::write_bytes((p + 4 * cap as usize - 200) as *mut u32, 0, 0x32);
        }
    }
    let cap = read_u32_as_usize(this + 4) as i32;
    let (vaga, rival) = find_offer_slot_and_rival(read_u32_as_usize(this), cap, cap, read_u32_as_usize(this + 8) as i32, alvo, comprador, inicial, primeira);
    if vaga == -1 {
        show_error_at(0xa8d514, 0x511);
        return;
    }
    let slot = |k: i32| read_u32_as_usize(this).wrapping_add(4 * k as usize);
    let velha = read_u32_as_usize(slot(vaga));
    if velha != 0 {
        tirar(this, velha);
    }
    let mem = novo(0x41);
    let nova = if mem != 0 { nova_oferta(mem) } else { 0 };
    write_u32_from_usize(slot(vaga), nova);
    write_u32_from_usize(this + 8, read_u32_as_usize(this + 8).wrapping_add(1));
    let o = read_u32_as_usize(slot(vaga));
    for k in [0usize, 4, 8, 0xc] {
        write_u32_from_usize(o + k, read_u32_as_usize(oferta + k));
    }
    write_u8(o + 0x10, read_u8(oferta + 0x10));
    write_u8(o + 0x11, read_u8(oferta + 0x11));
    let mut tmp = [0u8; 8];
    copiar_data(o + 0x12, tmp.as_mut_ptr(), oferta + 0x12);
    write_u8(o + 0x1a, read_u8(oferta + 0x1a));
    write_u32_from_usize(o + 0x1b, read_u32_as_usize(oferta + 0x1b));
    write_u8(o + 0x1f, read_u8(oferta + 0x1f));
    for k in [0x20usize, 0x24, 0x28] {
        write_u32_from_usize(o + k, read_u32_as_usize(oferta + k));
    }
    for k in [0x2cusize, 0x2d, 0x2e, 0x2f] {
        write_u8(o + k, read_u8(oferta + k));
    }
    write_u8(o + 0x30, (read_u8(oferta + 0x30) & 0xf) | (read_u8(o + 0x30) & 0xf0));
    write_u32_from_usize(o + 0x31, (read_u32_as_usize(oferta + 0x31) & 0xf) | (read_u32_as_usize(o + 0x31) & !0xf));
    write_u32_from_usize(o + 0x35, read_u32_as_usize(oferta + 0x35));
    write_u8(o + 0x39, read_u8(oferta + 0x39));
    write_u32_from_usize(o + 0x3a, read_u32_as_usize(oferta + 0x3a));
    write_u8(o + 0x3e, read_u8(oferta + 0x3e));
    write_u8(o + 0x3f, read_u8(oferta + 0x3f));
    write_u8(o + 0x40, read_u8(oferta + 0x40));
    write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 0x20, vaga as u32 as usize);
    write_u32_from_usize(oferta + 0x20, vaga as u32 as usize);
    write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 0x28, u32::MAX as usize);
    write_u8(read_u32_as_usize(slot(vaga)) + 0x2c, 0);
    write_u8(read_u32_as_usize(slot(vaga)) + 0x10, 0);
    let o = read_u32_as_usize(slot(vaga));
    if read_u8(o + 0x2d) == 2 {
        write_u8(o + 0x2d, 1);
    }
    write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 0x24, read_u32_as_usize(this + 0x85a));
    write_u32_from_usize(oferta + 0x24, read_u32_as_usize(read_u32_as_usize(slot(vaga)) + 0x24));
    write_u32_from_usize(this + 0x85a, read_u32_as_usize(this + 0x85a).wrapping_add(1));
    if read_u32_as_usize(oferta + 4) != 0 {
        let mem = novo(0xe);
        let l = if mem != 0 { lista_nova(mem) } else { 0 };
        write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 4, l);
        copiar_lista(read_u32_as_usize(read_u32_as_usize(slot(vaga)) + 4), read_u32_as_usize(oferta + 4));
    }
    if rival != -1 { // cqf1: a rival liga as ofertas
        // as listas copiam os 4 bytes do índice (a original passa o endereço das variáveis da pilha)
        let (vaga_l, rival_l) = (vaga, rival);
        crate::comparacao::ramo("cqf_1 ofertas rivais ligadas");
        let lista = read_u32_as_usize(read_u32_as_usize(slot(rival)));
        if lista != 0 {
            acrescentar(lista, &vaga_l, 4); // cqf1: a vaga na lista da rival
            write_u32_from_usize(read_u32_as_usize(slot(vaga)), read_u32_as_usize(read_u32_as_usize(slot(rival))));
        } else {
            let mem = novo(0xe);
            let l = if mem != 0 { lista_nova(mem) } else { 0 };
            acrescentar(l, &rival_l, 4);
            acrescentar(l, &vaga_l, 4);
            write_u32_from_usize(read_u32_as_usize(slot(rival)), l);
            write_u32_from_usize(read_u32_as_usize(slot(vaga)), l);
        }
    }
    let vendedor = contract_club_excluding_type5(transfer_manager_staff_by_id(alvo));
    // ponytail: os dois dwords que a original empurra para a 0x8c6220 têm só o byte baixo gravado (o resto é sobra da
    // pilha); a 0x8c6220 lê os bytes (dias as u8, fatias as u8)
    let (mut dias, mut fatias) = (0u8, 0u8);
    let agenda = if read_u8(oferta + 0x2f) != 0 {
        let h = humano(transfer_manager_club_by_id(comprador), 0, 1, 0, 0);
        write_u8(read_u32_as_usize(slot(vaga)) + 0x2c, 7);
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
        prazo(this, read_u32_as_usize(slot(vaga)), &mut dias, &mut fatias);
        dias != 0 || fatias != 0
    };
    if agenda {
        crate::comparacao::ramo("cqf_1 agenda pelo prazo");
        agendar(this, read_u32_as_usize(slot(vaga)), dias as u32, fatias as u32); // cqf1: o prazo da oferta
    }
    let o = read_u32_as_usize(slot(vaga));
    if read_u8(o + 0x1f) == 1 {
        noticia_emprestimo(this, o, termos);
    } else {
        noticia_oferta(this, o);
    }
    let reg = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(oferta + 0xc).wrapping_mul(8));
    write_u8(reg, read_u8(reg) | 1);
    let reg = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(oferta + 0xc).wrapping_mul(8));
    write_u8(reg + 1, read_u8(reg + 1).wrapping_add(1));
    let troca = read_u32_as_usize(oferta + 4);
    if troca != 0 {
        write_u32_from_usize(troca + 0xa, read_u32_as_usize(troca + 2));
        let mut n = proximo(read_u32_as_usize(oferta + 4));
        while n != 0 {
            let r = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(n).wrapping_mul(8)) + 2;
            write_u8(r, read_u8(r).wrapping_add(1));
            n = proximo(read_u32_as_usize(oferta + 4));
        }
    }
    if humano(transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)), 0, 1, 0, 0) != 0 {
        let p = transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc));
        if read_u32_as_usize(p + 0x61) != 0 {
            observar(read_u32_as_usize(G_SCOUT_MANAGER), p, 0);
        }
    }
}

/// Comparador da 0x8be300: só a original (aloca a oferta e as listas, tira a oferta antiga, agenda, entrega a
/// notícia). Confere o plano feito antes: a vaga (`find_offer_slot_and_rival`, com o vetor que cresce quando está cheio) e o número
/// de série gravados na oferta de entrada e a série que sobe.
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_offer(this: usize, oferta: usize, termos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ADD_TRANSFER_OFFER);
    let (alvo, comprador) = (read_u32_as_usize(oferta + 0xc), read_u32_as_usize(oferta + 8));
    let primeira = read_u8(read_u32_as_usize(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    let (lidas, conta) = (read_u32_as_usize(this + 4) as i32, read_u32_as_usize(this + 8) as i32);
    let cap = if conta == lidas { lidas.wrapping_add(0x32) } else { lidas };
    let (vaga, _) = find_offer_slot_and_rival(read_u32_as_usize(this), lidas, cap, conta, alvo, comprador, read_u32_as_usize(this + 0x85e) as i32, primeira);
    let serie = read_u32_as_usize(this + 0x85a);
    o(this, oferta, termos); // ponytail: só a original
    if vaga == -1 {
        return;
    }
    let (v, s, s2) = (read_u32_as_usize(oferta + 0x20) as i32, read_u32_as_usize(oferta + 0x24), read_u32_as_usize(this + 0x85a));
    // the serial goes up at least once: a nested offer made inside the call (news, scheduling) raises it again
    // (8-league audit of 07/11/2026, ENG SCO IRL ITA ESP, 2002-050: 4625 against 4624)
    // [pt] a série sobe pelo menos uma vez: uma oferta aninhada feita dentro da chamada (notícia, agenda) a sobe de novo
    PLACAR.registrar(v == vaga && s == serie && s2 > serie, || format!(
        "add_transfer_offer(alvo {}, comprador {}): vaga original {} plano {}, série {} plano {}, próxima {} plano {}",
        alvo, comprador, v, vaga, s, serie, s2, serie.wrapping_add(1)));
}

/// Offer status (+0x2c) is 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3 (the test that lets the under-offer mark be
/// cleared).
///
/// [pt] O status da oferta deixa seguir (0x8c0660): 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3.
pub(crate) unsafe fn is_offer_settled(e: usize) -> bool {
    match read_u8(e + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => read_u8(e + 0x2e) >= 3,
        _ => false,
    }
}

/// screens only: the club accepts the loan request; the contract decision, the news, and without agreement status 0xf,
/// the shortlists, the 10-day scheduling and bit 0 of the player record
/// Original: `TRANSFER_MANAGER::accept_loan_offer` (0x8c0660, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c0660 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::accept_loan_offer(vaga, marcar)`, só pela
/// tela de transferências (0x8ef1e0) e pela notícia (0x784100). Com `marcar`, +0x2d = 1. A decisão do contrato
/// (0x4cc930 em [0xae1050]), o status 8 e a notícia da oferta aceita (0x8d76d0); a decisão 0: nada mais. Senão a
/// notícia da recusa em se mudar (0x8d8310 com 0), o status 0xf com +0x2e = 3, o jogador sai das listas de observação
/// do comprador (0x84aeb0) e cada jogador da troca das do vendedor (o clube do contrato do jogador), e a oferta é
/// agendada para 10 dias (0x8c6220). Se ela e todas as ofertas pelo jogador (+0) seguem (`is_offer_settled`), o bit 0 do
/// registro de 8 bytes do jogador ([this+0x86e]) apaga.
pub(crate) unsafe extern "thiscall" fn accept_loan_offer(this: usize, vaga: *mut usize, marcar: u32) {
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_DECISION_FOR_TRANSFER);
    let noticia_aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ACCEPTED_NEWS);
    let noticia_recusa: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_PLAYER_REJECTS_MOVE_NEWS);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    crate::comparacao::ramo("cqf_1 aceitar empréstimo");
    if marcar != 0 {
        write_u8(*vaga + 0x2d, 1);
    }
    let d = decidir(read_u32_as_usize(G_CONTRACT_MANAGER), *vaga);
    write_u8(*vaga + 0x2c, 8);
    noticia_aceita(this, *vaga);
    if d == 0 {
        return;
    }
    noticia_recusa(this, *vaga, 0);
    write_u8(*vaga + 0x2c, 0xf);
    write_u8(*vaga + 0x2e, 3);
    let o = *vaga;
    let jogador = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), jogador);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let vendedor = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) } else { 0 };
    desmarcar(read_u32_as_usize(this + 0x856), transfer_manager_club_by_id(read_u32_as_usize(o + 8)), jogador);
    let troca = read_u32_as_usize(o + 4);
    if troca != 0 {
        write_u32_from_usize(troca + 0xa, read_u32_as_usize(troca + 2));
        let mut n = proximo(read_u32_as_usize(o + 4));
        while n != 0 {
            desmarcar(read_u32_as_usize(this + 0x856), vendedor, transfer_manager_staff_by_id(read_u32_as_usize(n)));
            n = proximo(read_u32_as_usize(o + 4));
        }
    }
    agendar(this, *vaga, 10, 0);
    let o = *vaga;
    if !is_offer_settled(o) {
        return;
    }
    let lista = read_u32_as_usize(o);
    if lista != 0 {
        write_u32_from_usize(lista + 0xa, read_u32_as_usize(lista + 2));
        let mut n = proximo(read_u32_as_usize(o));
        while n != 0 {
            if !is_offer_settled(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(4 * read_u32_as_usize(n)))) {
                return;
            }
            n = proximo(read_u32_as_usize(o));
        }
    }
    let r = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(o + 0xc).wrapping_mul(8));
    if read_u8(r) & 1 != 0 {
        write_u8(r, read_u8(r) & 0xfe);
    }
}

/// Comparador da 0x8c0660: só a original (entrega notícias, agenda a oferta, mexe nas listas de observação).
pub(crate) unsafe extern "thiscall" fn comparar_accept_loan_offer(this: usize, vaga: *mut usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32)>(IDX_ACCEPT_LOAN_OFFER)(this, vaga, marcar) // ponytail: só a original
}

// ---------- 270a leva: os termos aceitos pelo jogador e o empréstimo concluído na hora (transfer_manager.cpp) ----------









/// If the offer and every other offer for the same player (its list at +0) are settled, clears bit 0 of the player's
/// 8-byte flags record in [this+0x86e].
///
/// [pt] O bit 0 do registro de 8 bytes do jogador da oferta ([this+0x86e]) apaga quando ela e todas as ofertas pelo jogador
/// (a lista +0) seguem (`is_offer_settled`).
pub(crate) unsafe fn clear_under_offer_mark(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    if !is_offer_settled(oferta) {
        return;
    }
    let lista = read_u32_as_usize(oferta);
    if lista != 0 {
        write_u32_from_usize(lista + 0xa, read_u32_as_usize(lista + 2));
        let mut n = proximo(read_u32_as_usize(oferta));
        while n != 0 {
            if !is_offer_settled(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(4 * read_u32_as_usize(n)))) {
                return;
            }
            n = proximo(read_u32_as_usize(oferta));
        }
    }
    let r = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(oferta + 0xc).wrapping_mul(8));
    if read_u8(r) & 1 != 0 {
        write_u8(r, read_u8(r) & 0xfe);
    }
}

/// Finds the nation's transfer rules among the 28 objects at [this+0x8ac] (matching +0xa); None if none.
///
/// [pt] A regra de transferência da nação (os 28 objetos em [this+0x8ac], o +0xa de cada um comparado com `nacao`; a
/// original lê o +0xa sem conferir o ponteiro); None sem nenhuma.
pub(crate) unsafe fn nation_transfer_rules_opt(this: usize, nacao: usize) -> Option<usize> {
    for k in 0..0x1c {
        let p = this + 0x8ac + 4 * k;
        if read_u32_as_usize(read_u32_as_usize(p) + 0xa) == nacao {
            return Some(read_u32_as_usize(p));
        }
    }
    None
}

/// marks the agreement of the target (or the exchange player); outside loans, the reply news and the offer follow-up;
/// for loans, a squad above 50 or the nation rule refuse, otherwise completes at once (contract, history, numbers,
/// record, status 0x14 in 5 days)
/// Original: `TRANSFER_MANAGER::handle_player_accepted_terms` (0x8c2180, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c2180 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da data local sem destrutor):
/// `TRANSFER_MANAGER::handle_player_accepted_terms(pessoa, vaga, contrato, tipo, noticia)`, pela aceitação de um
/// contrato (0x4cf413). Vaga fora do vetor: a caixa "Error" (0xe0c, 0xe11) e nada; vaga vazia: nada. O +0x2e zera. O
/// alvo da oferta aceitou: +0x10 = 2 e +0x11 = tipo; A = o comprador, B = o clube do contrato do alvo (tipo 5 não
/// conta). Senão a pessoa é um jogador da troca: o nó dela na troca (+4) ganha 2 e o tipo (+4, +6); A = o clube do
/// contrato do alvo, B = o comprador (sem troca: a caixa 0xe4d; sem o nó, nada achado: A e B a original deixa sem
/// valor, a cópia 0). Fora do empréstimo (+0x1f ≠ 1): a notícia da resposta (0x8d8e90 com A, B, 2 e a oferta) e o
/// seguimento da oferta (0x8c2f50). No empréstimo: elenco de A (0x53c590) + 1 acima de 50, o motivo 4; senão a regra
/// da nação de A (+0x53) responde (vtable +0x20 com o comprador e o alvo) com o motivo. Com motivo: a recusa
/// (0x8e23d0), o status 0x10, o fim (0x8c6db0), o alvo sai das listas do comprador e os da troca das do clube do
/// contrato do alvo (0x84aeb0), 5 dias (0x8c6220) e a marca (`clear_under_offer_mark`). Sem motivo, o empréstimo na hora:
/// os termos (0x8e1f10), a regra da nação do comprador (vtable +8, ou +4 fora do empréstimo), o contrato assinado
/// (0x4d01e0), o +0x6d 2 da pessoa vira 0x10, o histórico (0x86da20), a lista de observação (0x842080/0x843e10 com 5),
/// 0x5563f0, os números (0x85a850, 0x540980, 0x85add0/0x85a470), 0x4196a0, o registro da transferência passada
/// (0x8cb810: a pessoa, os +0x33 de +4, +8 e +0xc dela, os ids de A e B, 0, 2 e hoje), o status 0x14 com 5 dias, a
/// marca, o fim (0x8c6db0) e a pessoa fora das listas (0x842b00 com 1).
pub(crate) unsafe extern "thiscall" fn handle_player_accepted_terms(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    let noticia_resposta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_RESPONSE_NEWS);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    let elenco: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_CANCELLED_NEWS);
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let termos: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_ACCEPTED_NEWS);
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_SIGN_LOAN_CONTRACT);
    let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LOAN_RECORDS);
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_IS_ON_SHORTLIST);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_HOLD_BANS_FOR_LOAN);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_SET_SQUAD_NUMBER);
    let marcar_elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_CLEAR_AWOL);
    let nova_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let registrar: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_PAST_TRANSFER);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
    if vaga < 0 {
        show_error_at(0xa8d514, 0xe0c);
        return;
    }
    if vaga >= read_u32_as_usize(this + 4) as i32 {
        show_error_at(0xa8d514, 0xe11);
        return;
    }
    let oferta = read_u32_as_usize(read_u32_as_usize(this).wrapping_add(4 * vaga as usize));
    if oferta == 0 {
        return;
    }
    write_u8(oferta + 0x2e, 0);
    // a = edi da original, b = [esp+0x1c]
    let (mut a, mut b) = (0usize, 0usize);
    if read_u32_as_usize(pessoa) == read_u32_as_usize(oferta + 0xc) {
        write_u8(oferta + 0x10, 2);
        write_u8(oferta + 0x11, tipo as u8);
        a = transfer_manager_club_by_id(read_u32_as_usize(oferta + 8));
        let c = contract_of_staff_cached(pessoa);
        b = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) };
    } else if read_u32_as_usize(oferta + 4) == 0 {
        show_error_at(0xa8d514, 0xe4d);
    } else {
        let troca = read_u32_as_usize(oferta + 4);
        write_u32_from_usize(troca + 0xa, read_u32_as_usize(troca + 2));
        let mut n = proximo(read_u32_as_usize(oferta + 4));
        while n != 0 {
            if read_u32_as_usize(n) == read_u32_as_usize(pessoa) {
                write_u8(n + 4, 2);
                write_u8(n + 6, tipo as u8);
                let c = contract_of_staff_cached(transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)));
                a = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) } else { 0 };
                b = transfer_manager_club_by_id(read_u32_as_usize(oferta + 8));
                break;
            }
            n = proximo(read_u32_as_usize(oferta + 4));
        }
    }
    if read_u8(oferta + 0x1f) != 1 {
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
    } else if let Some(r) = nation_transfer_rules_opt(this, read_u32_as_usize(a + 0x53)) {
        if r != 0 {
            let pode: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0x20));
            pode(r, transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)), transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)), 0, &mut motivo);
        }
    }
    if motivo as u16 != 0 {
        crate::comparacao::ramo("cqf_2 empréstimo recusado");
        recusar(this, oferta, motivo);
        write_u8(oferta + 0x2c, 0x10);
        fechar(this, oferta);
        let alvo = transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc));
        let c = contract_of_staff_cached(alvo);
        let dono = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) } else { 0 };
        desmarcar(read_u32_as_usize(this + 0x856), transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)), alvo);
        let troca = read_u32_as_usize(oferta + 4);
        if troca != 0 {
            write_u32_from_usize(troca + 0xa, read_u32_as_usize(troca + 2));
            let mut n = proximo(read_u32_as_usize(oferta + 4));
            while n != 0 {
                desmarcar(read_u32_as_usize(this + 0x856), dono, transfer_manager_staff_by_id(read_u32_as_usize(n)));
                n = proximo(read_u32_as_usize(oferta + 4));
            }
        }
        agendar(this, oferta, 5, 0);
        clear_under_offer_mark(this, oferta);
        return;
    }
    crate::comparacao::ramo("cqf_2 empréstimo na hora");
    termos(this, oferta, contrato);
    if let Some(r) = nation_transfer_rules_opt(this, read_u32_as_usize(transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)) + 0x53)) {
        if r != 0 {
            let k = if read_u8(oferta + 0x1f) == 0 { 4 } else { 8 };
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + k));
            f(r, oferta);
        }
    }
    assinar(read_u32_as_usize(G_CONTRACT_MANAGER), contrato);
    if read_u8(pessoa + staff::SQUAD_SELECTED_FOR) == 2 {
        write_u8(pessoa + staff::SQUAD_SELECTED_FOR, 0x10);
    }
    historico(read_u32_as_usize(G_STAFF_RECORDS), pessoa, b, a);
    if observado(read_u32_as_usize(this + 0x856), pessoa, a) != 0 {
        situacao(read_u32_as_usize(this + 0x856), pessoa, a, 5, 0);
    }
    guardar(read_u32_as_usize(G_DISCIPLINE), transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)));
    numero(read_u32_as_usize(G_SQUAD_MANAGER), transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)), a, 0);
    marcar_elenco(pessoa, 1);
    if humano(a, 0, 1, 0, 0) == 0 && usa_numeros(read_u32_as_usize(G_SQUAD_MANAGER), a, 0) != 0 {
        dar_numero(read_u32_as_usize(G_SQUAD_MANAGER), a, pessoa);
    }
    acabar(read_u32_as_usize(G_AWOL), pessoa);
    // o registro da transferência passada, 0x25 bytes: +0 a pessoa, +4/+8/+0xc os +0x33 dos ponteiros +4/+8/+0xc dela,
    // +0x10 e +0x14 os ids de A e B, +0x18 0, +0x1c 2, +0x1d hoje
    let mut reg = [0u8; 0x28];
    let r = reg.as_mut_ptr() as usize;
    nova_data(r + 0x1d);
    write_u32_from_usize(r + 0x10, read_u32_as_usize(a));
    write_u32_from_usize(r + 0x14, read_u32_as_usize(b));
    let mut tmp = [0u8; 8];
    copiar_data(r + 0x1d, tmp.as_mut_ptr(), 0xae2c90);
    write_u32_from_usize(r, read_u32_as_usize(pessoa));
    write_u32_from_usize(r + 4, read_u32_as_usize(read_u32_as_usize(pessoa + 4) + 0x33));
    write_u32_from_usize(r + 8, read_u32_as_usize(read_u32_as_usize(pessoa + 8) + 0x33));
    write_u32_from_usize(r + 0xc, read_u32_as_usize(read_u32_as_usize(pessoa + staff::COMMON_NAME) + 0x33));
    write_u8(r + 0x1c, 2);
    write_u32_from_usize(r + 0x18, 0);
    registrar(this, reg.as_mut_ptr());
    write_u8(oferta + 0x2c, 0x14);
    agendar(this, oferta, 5, 0); // cqf2: o empréstimo concluído em 5 dias
    clear_under_offer_mark(this, oferta);
    fechar(this, oferta);
    suspender(read_u32_as_usize(this + 0x856), pessoa, 1);
}

/// Comparador da 0x8c2180: só a original (entrega notícias, conclui ou recusa o empréstimo, assina contrato, registra a
/// transferência, mexe nas listas).
pub(crate) unsafe extern "thiscall" fn comparar_handle_player_accepted_terms(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, u32)>(IDX_HANDLE_PLAYER_ACCEPTED_TERMS)(this, pessoa, vaga, contrato, tipo, noticia) // ponytail: só a original
}


// ---------- 268a leva: a resposta da IA ao lance e a negociação de empréstimo copiada da oferta (transfer_manager.cpp) ----------









/// Shows the "Error" box "v%s %s.%s %d" for source file `arquivo` and line, then clears G_MOUSE_LEFT_BUTTON_DOWN.
///
/// [pt] A caixa "Error" com "v%s %s.%s %d" (versão 0x90d130, nome e extensão do .cpp `arquivo` pela 0x944cff, `linha`) em
/// 0xae24d0 (0x944c9f) e a caixa 0x5e8290; [0xb67a34] = 0. As três chamadas cdecl, como na original.
pub(crate) unsafe fn show_error_box(arquivo: usize, linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(arquivo, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    write_u32_from_usize(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// the computer seller answers a bid: loan bids are refused without looking at anything else; unless a rule makes it
/// accept, the floor fee and the other offers, the exchange value, the cash, exchange and clause options; an option
/// worth less than the bid / 0.85 accepts, the best one under the bid / 0.75 becomes a counter offer, otherwise refuses
/// Original: `TRANSFER_MANAGER::ai_respond_to_bid` (0x8c8090, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c8090 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor: a data local da vaga):
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
pub(crate) unsafe extern "thiscall" fn ai_respond_to_bid(this: usize, vaga: usize, ensaio: u32) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_LOAN_OFFER);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ACCEPT_TRANSFER_BID);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let regra: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let piso_de: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let total_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let valor_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_VALUE_PLAYER_FOR_CLUB);
    let aceita_troca: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_CLUB_WANTS_PLAYER);
    let dinheiro: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_BUILD_CASH_OFFER_OPTION);
    let troca: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_BUILD_EXCHANGE_OFFER_OPTION);
    let clausula: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_BUILD_CLAUSE_OFFER_OPTION);
    let negociar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_NEGOTIATION);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    // ponytail: a vaga local (esp+0x58) a original deixa com sobra de pilha fora da data (+0x1b), da palavra da posição,
    // do +0xb e dos bits 2..4 e 0..1 de +0x24; a 0x840d80 lê aqui só a posição, o +0x24 (tipo 4) e o +0xf (comparado com
    // o jogador da troca, que não é do vendedor): a cópia zera
    let mut vl = [0u8; 0x28];
    let pv = vl.as_mut_ptr() as usize;
    let mut melhor_atracao: i8 = -1;
    let mut troca_recusada: u32 = 0;
    data_padrao(pv + 0x1b);
    vl[0x24] = (vl[0x24] & 0xe4) | 4;
    write_u32_from_usize(pv + 0xb, 10000);
    let o = read_u32_as_usize(vaga);
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
    let comprador = transfer_manager_club_by_id(read_u32_as_usize(o + 8));
    let c = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), alvo);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let vendedor = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) };
    if read_u32_as_usize(alvo + 0x61) == 0 {
        write_u8(o + 0x2c, 0x10);
        agendar(this, o, 1, 0);
        return;
    }
    let mut motivo: u32 = 0;
    let mut n: i8 = 0;
    if read_u8(o + 0x1f) == 1 {
        if ensaio == 0 {
            recusar_emprestimo(this, vaga, 0, 0);
        }
        return;
    }
    if deve_aceitar(read_u32_as_usize(vaga), 0, 0) != 0 {
        if ensaio == 0 {
            aceitar(this, vaga, 0);
        }
        return;
    }
    if vendedor != 0 && humano(vendedor, 1, 1, 0, 0) != 0 && responde(read_u32_as_usize(G_HUMAN_MANAGER), tecnico(vendedor)) != 0 {
        if regra(read_u32_as_usize(G_HUMAN_MANAGER), 0, tecnico(vendedor)) != 0 && read_u32_as_usize(o + 4) != 0 {
            recusar(this, vaga, 0, 0);
            return;
        }
        if regra(read_u32_as_usize(G_HUMAN_MANAGER), 0, tecnico(vendedor)) == 1 {
            recusar(this, vaga, 0, 0);
            return;
        }
    }
    let mut piso = piso_de(read_u32_as_usize(G_CONTRACT_MANAGER), vendedor, comprador, alvo, &mut motivo, 1, 1);
    let l = read_u32_as_usize(o);
    if l != 0 && ensaio == 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        loop {
            let p = transfer_manager_llist_next(read_u32_as_usize(o));
            if p == 0 {
                break;
            }
            let k = read_u32_as_usize(p);
            let outra = read_u32_as_usize(read_u32_as_usize(this).wrapping_add(k.wrapping_mul(4)));
            if k != read_u32_as_usize(o + 0x20) && !is_offer_status_settled(outra) {
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
    let t = read_u32_as_usize(o + 4);
    if t != 0 {
        write_u32_from_usize(t + 0xa, read_u32_as_usize(t + 2));
        let teto = piso.wrapping_sub(k);
        let mut p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
        while p != 0 {
            let j = transfer_manager_staff_by_id(read_u32_as_usize(p));
            let m = mascara(read_u32_as_usize(j + 0x61));
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
            let mut v = valor_jogador(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), vendedor, j, pv, &mut saida, -1, 0);
            if v > teto {
                v = teto;
            }
            if ensaio == 0 && aceita_troca(read_u32_as_usize(this + 0x856), j, vendedor, v, 1) == 0 {
                troca_recusada = 1;
            }
            p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
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
            if v as f64 * read_f64_at(0x969520) < pedido_f && troca_recusada == 0 { // cqe1: a opção × 0,85 abaixo do pedido
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
        show_error_box(0xa8d514, 0x1eee);
        recusar(this, vaga, 1, motivo);
        return;
    }
    if !aceito {
        let pedido_f = pedido as f64;
        let v = valor(melhor, 0, 0, 0, 0, 1);
        let tipo = read_u32_as_usize(o + 0x31) & 0xf;
        if v as f64 * read_f64_at(0x969040) < pedido_f && tipo != 1 && tipo != 2 && ensaio == 0 { // cqe1: a contraproposta
            crate::comparacao::ramo("cqe_1 contraproposta");
            write_u8(melhor + 0x30, (read_u8(melhor + 0x30) & 0xf0) | (read_u8(o + 0x30) & 0xf));
            negociar(this, o, melhor, 0);
        } else {
            let w = read_u32_as_usize(alvo + 0x52) as i32;
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
pub(crate) unsafe extern "thiscall" fn comparar_ai_respond_to_bid(this: usize, vaga: usize, ensaio: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_AI_RESPOND_TO_BID)(this, vaga, ensaio) // ponytail: só a original
}

/// screens only: the offer copy without its lists, with the contract built from the new loan terms, becomes a
/// negotiation; returns the copy index
/// Original: `TRANSFER_MANAGER::add_loan_negotiation` (0x8bee50, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8bee50 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da oferta local): `TRANSFER_MANAGER::
/// add_loan_negotiation(oferta, a2, a3, a4, a5)`, só pela tela de transferências (0x8fb5e0). Um STAFF_CONTRACT local
/// (0x85e410) e uma oferta local (0x8e5680) com os campos da oferta (a data +0x12 pela 0x417c10; o nibble baixo de +0x30
/// e de +0x31; as listas +0 e +4 vazias); o contrato pelo atual (0x85e590: push a5; push a4; push a3; push a2; push
/// clube +8; push alvo +0xc; lea ecx, contrato) entra na lista em this+0x946 (criada se faltar: 0x944cf1 de 0xe bytes e
/// 0x68d020; 0x68d0d0 com 0x50 bytes) e a cópia vira negociação (0x8be9d0: push 0; push &cópia; push oferta). Devolve o
/// +0x20 da cópia (o índice); a cópia é destruída (0x8e5710).
pub(crate) unsafe extern "thiscall" fn add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let pelo_atual: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_INIT_FROM_CURRENT_CONTRACT);
    let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let negociar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_NEGOTIATION);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let mut contrato = [0u8; 0x50];
    let pc = contrato.as_mut_ptr() as usize;
    // ponytail: o nibble alto de +0x30 e os bits 4..31 de +0x31 da cópia a original deixa com sobra de pilha; aqui zero
    let mut copia_oferta = [0u8; 0x44];
    let po = copia_oferta.as_mut_ptr() as usize;
    novo_contrato(pc);
    nova_oferta(po);
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
    let clube = transfer_manager_club_by_id(read_u32_as_usize(o + 8));
    write_u32_from_usize(po + 0xc, read_u32_as_usize(o + 0xc));
    write_u8(po + 0x11, read_u8(o + 0x11));
    write_u32_from_usize(po + 8, read_u32_as_usize(o + 8));
    write_u8(po + 0x10, read_u8(o + 0x10));
    write_u32_from_usize(po, read_u32_as_usize(o));
    write_u32_from_usize(po + 4, read_u32_as_usize(o + 4));
    let mut tmp = [0u8; 8];
    copia(po + 0x12, tmp.as_mut_ptr() as usize, o + 0x12);
    write_u32_from_usize(po + 0x1b, read_u32_as_usize(o + 0x1b));
    write_u8(po + 0x1f, read_u8(o + 0x1f));
    write_u8(po + 0x1a, read_u8(o + 0x1a));
    write_u32_from_usize(po + 0x24, read_u32_as_usize(o + 0x24));
    write_u8(po + 0x2d, read_u8(o + 0x2d));
    write_u32_from_usize(po + 0x28, read_u32_as_usize(o + 0x28));
    write_u8(po + 0x2e, read_u8(o + 0x2e));
    write_u32_from_usize(po + 0x20, read_u32_as_usize(o + 0x20));
    write_u8(po + 0x2c, read_u8(o + 0x2c));
    write_u8(po + 0x2f, read_u8(o + 0x2f));
    write_u8(po + 0x30, (read_u8(po + 0x30) & 0xf0) | (read_u8(o + 0x30) & 0xf));
    write_u32_from_usize(po + 0x31, (read_u32_as_usize(po + 0x31) & !0xf) | (read_u32_as_usize(o + 0x31) & 0xf));
    write_u32_from_usize(po + 0x35, read_u32_as_usize(o + 0x35));
    write_u8(po + 0x39, read_u8(o + 0x39));
    write_u32_from_usize(po + 0x3a, read_u32_as_usize(o + 0x3a));
    write_u8(po + 0x40, read_u8(o + 0x40));
    write_u8(po + 0x3e, read_u8(o + 0x3e));
    write_u8(po + 0x3f, read_u8(o + 0x3f));
    write_u32_from_usize(po + 4, 0);
    write_u32_from_usize(po, 0);
    pelo_atual(pc, alvo, clube, a2, a3, a4, a5);
    if read_u32_as_usize(this + 0x946) == 0 {
        let m = alocar(0xe);
        let l = if m != 0 { lista_nova(m) } else { 0 };
        write_u32_from_usize(this + 0x946, l);
    }
    acrescentar(read_u32_as_usize(this + 0x946), pc, 0x50);
    negociar(this, o, po, 0);
    let r = read_u32_as_usize(po + 0x20) as u32;
    destruir(po);
    r
}

/// Comparador da 0x8bee50: só a original (a lista de contratos cresce, a negociação nova é alocada).
pub(crate) unsafe extern "thiscall" fn comparar_add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(IDX_ADD_LOAN_NEGOTIATION)(this, o, a2, a3, a4, a5) // ponytail: só a original
}

// ---------- 268a leva: a opção de troca na resposta ao lance e o fim do empréstimo agendado (transfer_manager.cpp, contract_manager.cpp) ----------









/// Person -> club of their contract (get_contract, stored in G_CONTRACT_LOOKUP_RESULT), type 5 included; 0 without
/// contract.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080], e o
/// clube dele (sem contrato: 0; o tipo 5 conta).
pub(crate) unsafe fn contract_club_any_type(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) }
}

/// ORs into `m` the flag (0x40/0x50/0x10/0x30/0x20) for the reputation scope of the person vs the seller's manager
/// (+0xcf); other scopes return `antes` unchanged.
///
/// [pt] As marcas da nota pelo alcance da reputação (0x5408c0, cdecl(pessoa, técnico do vendedor +0xcf)): 0..4 dão 0x40,
/// 0x50, 0x10, 0x30, 0x20 sobre `m` (as duas tabelas de salto da original, 0x8c8fa4 e 0x8c8fb8); fora disso a original
/// deixa o ebp como estava (`antes`).
pub(crate) unsafe fn reputation_scope_flags(pessoa: usize, vendedor: usize, m: u32, antes: u32) -> u32 {
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_DATABASE_GET_REPUTATION_SCOPE);
    match escopo(pessoa, read_u32_as_usize(vendedor + 0xcf)) as i8 {
        0 => m | 0x40,
        1 => m | 0x50,
        2 => m | 0x10,
        3 => m | 0x30,
        4 => m | 0x20,
        _ => antes,
    }
}

/// Rates the person for a position with the given flags for the seller club (MATCH_MAN score_player_for_role 0x6e2f20
/// with the original's fixed arguments).
///
/// [pt] A nota da pessoa para o vendedor (0x6e2f20, cdecl de 15 dwords: pessoa, +0x61, 0, a posição, as marcas, o vendedor,
/// 0, 0, −1 e seis zeros).
pub(crate) unsafe fn score_player_for_seller(pessoa: usize, posicao: u16, marcas: u32, vendedor: usize) -> i32 {
    let nota: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    nota(pessoa, read_u32_as_usize(pessoa + staff::PLAYER), 0, posicao as u32, marcas, vendedor, 0, 0, -1, 0, 0, 0, 0, 0, 0)
}

/// builds the exchange option: a new offer at the price with attraction 50; for the target and the 25 slots of the
/// seller shortlist, buyer players in the same position, free of offers and exchanges, with interest of at least 3.15
/// and rating above 0.9 of the candidate join the exchange (up to 3): attraction rises 20 or 5 and their value leaves
/// the price
/// Original: `TRANSFER_MANAGER::make_exchange_option` (0x8c8860, transfer_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x8c8860 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da LLIST local): `TRANSFER_MANAGER::
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
pub(crate) unsafe extern "thiscall" fn make_exchange_option(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let preparar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_TRANSFER_OFFER);
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let piso_de: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let valor_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_VALUE_PLAYER_FOR_CLUB);
    let vendedor = contract_club_any_type(alvo);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let pl = lista.as_mut_ptr() as usize;
    lista_nova(pl);
    let cv = contrato_com(read_u32_as_usize(G_CONTRACT_MANAGER), alvo, vendedor);
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0x856) + 0xdc).wrapping_add(read_u32_as_usize(vendedor).wrapping_mul(9));
    if read_u8(reg + 4) == 0 { // cqe2: o vendedor sem lista de observação ativa
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
    write_u32_from_usize(nova + 0x1b, valor as u32 as usize); // cqe2: o preço da opção
    let mut teto = match_pl_ftol(valor as f64 * read_f64_at(0x9689c0));
    // ponytail: a posição, a nota base, as marcas e o estatuto ficam de uma vaga para a outra (a original os deixa na
    // pilha sem valor até a primeira vaga com ficha +0x61; candidato sem ficha não acontece: começam em zero). As marcas
    // fora de 0..4 do alcance (que a 0x5408c0 não devolve) seriam o ebp anterior, de início o ponteiro da oferta.
    let mut posicao: u16 = 0;
    let mut base: i32 = 0;
    let mut marcas: u32 = 0;
    let mut estatuto: u8 = 0;
    let mut ebp: u32 = m as u32;
    let estatuto_da_vaga = |i: i8| (read_u8(read_u32_as_usize(reg).wrapping_add((i as i32 * 0x25) as usize) + 0x24) >> 1) & 0xf;
    let mut i: i8 = -1;
    loop {
        let t = read_u32_as_usize(nova + 4);
        if t != 0 && read_u16(t) == 3 {
            break;
        }
        'vaga: {
            let cand = if i == -1 {
                if read_u8(cv + 0x4f) & 0xf0 == 0x70 {
                    break 'vaga;
                }
                alvo
            } else {
                let id = read_u32_as_usize(read_u32_as_usize(reg).wrapping_add((i as i32 * 0x25) as usize) + 0xf) as i32;
                if id <= -1 { // cqe2: a vaga sem alvo
                    break 'vaga;
                }
                let p = transfer_manager_staff_by_id(id as usize);
                if read_u32_as_usize(p + 0x39) == vendedor {
                    break 'vaga;
                }
                crate::comparacao::ramo("cqe_2 vaga com alvo");
                p
            };
            let cc = contrato_com(read_u32_as_usize(G_CONTRACT_MANAGER), cand, contract_club_any_type(cand));
            let mut j: i8 = -1;
            loop {
                'jogador: {
                    let x = if j == -1 {
                        if read_u32_as_usize(cand + 0x61) == 0 {
                            break 'jogador;
                        }
                        posicao = mascara(read_u32_as_usize(cand + 0x61));
                        marcas = 0x4800;
                        estatuto = if i == -1 { read_u8(cc + 0x4f) >> 4 } else { estatuto_da_vaga(i) };
                        if estatuto != 2 && estatuto != 3 {
                            marcas = 0x4900;
                        }
                        if estatuto == 5 {
                            ebp = marcas;
                        } else {
                            marcas |= 0x80;
                            ebp = reputation_scope_flags(cand, vendedor, marcas, ebp);
                        }
                        base = score_player_for_seller(cand, posicao, ebp, vendedor);
                        cand
                    } else {
                        let x = elenco[j as usize];
                        if x == 0 || read_u32_as_usize(x) as i32 >= read_u32_as_usize(G_STAFF_COUNT) as i32 || read_u32_as_usize(x + 0x61) == 0 {
                            break 'jogador;
                        }
                        if mascara(read_u32_as_usize(x + 0x61)) != posicao {
                            break 'jogador;
                        }
                        x
                    };
                    let cx = contrato_com(read_u32_as_usize(G_CONTRACT_MANAGER), x, contract_club_any_type(cand));
                    if cx == 0 || read_u32_as_usize(x + 0x39) != comprador || ficha(read_u32_as_usize(G_CONTRACT_MANAGER), x) != 0 {
                        break 'jogador;
                    }
                    if read_u8(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(x).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 || read_u8(cx + 0x35) & 0x3f == 5 {
                        break 'jogador;
                    }
                    if piso_de(read_u32_as_usize(G_CONTRACT_MANAGER), comprador, 0, x, 0, 1, 1) > teto {
                        break 'jogador;
                    }
                    esvaziar(pl);
                    if ofertas(this, x, 0, 0) != 0 || trocas(this, x, pl) != 0 || negociacoes(this, alvo, 0) != 0 {
                        if read_u16(pl) != 1 {
                            break 'jogador;
                        }
                        write_u32_from_usize(pl + 0xa, read_u32_as_usize(pl + 2));
                        let p = transfer_manager_llist_next(pl);
                        if read_u32_as_usize(oferta + 0x20) != read_u32_as_usize(p) {
                            break 'jogador;
                        }
                    }
                    if chegar(this, x, vendedor, 0, 1) == 0 {
                        break 'jogador;
                    }
                    // ponytail: no alvo a original empurra o estatuto com os bytes altos do ecx que a 0x8ce990 deixou (lixo);
                    // nas vagas, com os de 0x25 × vaga; a 0x846920 recebe um char: só o byte conta
                    let st = if i == -1 {
                        (read_u8(cc + 0x4f) >> 4) as u32
                    } else {
                        ((i as i32 * 0x25) as u32 & 0xffff_ff00) | estatuto_da_vaga(i) as u32
                    };
                    let r = interesse(read_u32_as_usize(this + 0x856), x, vendedor, 1, 0, st, 0);
                    if !(r >= read_f64_at(0x968e58)) { // cqe2: o interesse abaixo de 3,15
                        break 'jogador;
                    }
                    crate::comparacao::ramo("cqe_2 interesse");
                    if estatuto == 5 {
                        ebp = marcas;
                    } else {
                        marcas |= 0x80;
                        ebp = reputation_scope_flags(x, vendedor, marcas, ebp);
                    }
                    let nx = score_player_for_seller(x, posicao, ebp, vendedor);
                    if nx == 0 || base == 0 {
                        break 'jogador;
                    }
                    if !(nx as f64 / base as f64 > read_f64_at(0x9689d8)) { // cqe2: a nota até 0,9 da base
                        break 'jogador;
                    }
                    crate::comparacao::ramo("cqe_2 nota da troca");
                    let a = if nx > base { (*atracao).wrapping_add(20) } else { (*atracao).wrapping_add(5) };
                    *atracao = if a < 0 { 0 } else if a > 100 { 100 } else { a };
                    if read_u32_as_usize(nova + 4) == 0 {
                        let m = alocar(0xe);
                        let l = if m != 0 { lista_nova(m) } else { 0 };
                        write_u32_from_usize(nova + 4, l);
                    }
                    let t = read_u32_as_usize(nova + 4);
                    write_u32_from_usize(t + 0xa, read_u32_as_usize(t + 2));
                    loop {
                        let p = transfer_manager_llist_next(read_u32_as_usize(nova + 4));
                        if p == 0 {
                            break;
                        }
                        if read_u32_as_usize(p) == read_u32_as_usize(x) {
                            break 'jogador;
                        }
                    }
                    // ponytail: o 7º byte do item a original copia da pilha sem valor; aqui zero
                    let mut item = [0u8; 8];
                    write_u32_from_usize(item.as_mut_ptr() as usize, read_u32_as_usize(x));
                    item[5] = estatuto;
                    acrescentar(read_u32_as_usize(nova + 4), item.as_mut_ptr() as usize, 7);
                    let entrada = read_u32_as_usize(reg).wrapping_add((i as i32 * 0x25) as usize);
                    let mut fora: i32 = 0;
                    let v = valor_jogador(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), vendedor, x, entrada, &mut fora, -1, 1);
                    crate::comparacao::ramo("cqe_2 jogador na troca");
                    teto = teto.wrapping_sub(v);
                    let r = (read_u32_as_usize(nova + 0x1b) as i32).wrapping_sub(v); // cqe2: o valor dele sai do preço
                    write_u32_from_usize(nova + 0x1b, if r < 0 { 0 } else { r as u32 as usize });
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

/// Byte snapshot of an option offer (0x41 bytes minus the fields the original leaves uninitialised) plus its exchange
/// list items, for the comparer.
///
/// [pt] A foto da oferta da opção para o comparador: os 0x41 bytes sem o ponteiro +4, sem +0x1a, +0x20..+0x23 e +0x31..+0x34
/// e só o nibble baixo de +0x30 (o que a 0x8e5680 e a 0x8e78f0 não preenchem fica com o que havia na memória nova), e os
/// itens da troca (id, 0, estatuto: o 7º byte vem da pilha da original).
pub(crate) unsafe fn option_offer_snapshot(o: usize) -> Vec<u8> {
    let mut v: Vec<u8> = (0..0x41).filter(|&k| !(4..8).contains(&k) && k != 0x1a && !(0x20..0x24).contains(&k) && !(0x31..0x35).contains(&k))
        .map(|k| if k == 0x30 { read_u8(o + k) & 0xf } else { read_u8(o + k) }).collect();
    let l = read_u32_as_usize(o + 4);
    if l != 0 {
        v.extend_from_slice(&read_u16(l).to_le_bytes());
        let mut no = read_u32_as_usize(l + 2);
        while no != 0 {
            let item = read_u32_as_usize(no);
            v.extend((0..6).map(|k| read_u8(item + k)));
            no = read_u32_as_usize(no + 6);
        }
    }
    v
}

/// Comparador da 0x8c8860: lado a lado a partir do mesmo estado de [0xae1080], *saida, *atracao e do sorteador (a
/// 0x8e78f0 sorteia); a cópia roda sem contar sorteios. Confere a devolução, a atração, [0xae1080] e a foto da oferta
/// nova de cada lado; a da cópia é destruída (0x8e5710) e liberada (0x944c94) e fica tudo como a original deixou.
pub(crate) unsafe extern "thiscall" fn comparar_make_exchange_option(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32>(IDX_MAKE_EXCHANGE_OPTION);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let est = crate::aleatorio::REGIOES_DO_ESTADO;
    let regioes = [(0xae1080usize, 4usize), (saida as usize, 4), (atracao as usize, 1), est[0], est[1]];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let r = o(this, comprador, alvo, oferta, saida, atracao, valor);
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let (ao, fo) = (*atracao, if r != 0 && *saida != 0 { option_offer_snapshot(*saida) } else { Vec::new() });
    let e_o = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    antes.escrever();
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| make_exchange_option(this, comprador, alvo, oferta, saida, atracao, valor)));
    let (an, fn_) = (*atracao, if n != 0 && *saida != 0 { option_offer_snapshot(*saida) } else { Vec::new() });
    let e_n = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    if n != 0 && *saida != 0 {
        let s = *saida;
        destruir(s);
        liberar(s);
    }
    depois.escrever();
    PLACAR.registrar(r == n && ao == an && e_o == e_n && fo == fn_, || format!(
        "make_exchange_option(comprador {:#x}, alvo {:#x}, valor {}): original {} rust {}, atração {} x {}, [0xae1080] {:#x} x {:#x}, oferta {:x?} x {:x?}",
        comprador, alvo, valor, r, n, ao, an, e_o, e_n, fo, fn_));
    r
}


/// Person -> contract record via the index at `campo` (+0 contract, +4 loan) of their 79-byte staff state record, into
/// the 0x50-byte vector at [vetor]; 0 if id out of range or index negative.
///
/// [pt] O contrato do registro de 79 bytes da pessoa em [0xae2d0c] pelo índice em `campo` (+0 o contrato, +4 o do
/// empréstimo) no vetor de 0x50 bytes em [`vetor`]: id fora do total ([0xae2364]) ou índice negativo, 0.
pub(crate) unsafe fn staff_contract_by_field(pessoa: usize, campo: usize, vetor: usize) -> usize {
    let id = read_u32_as_usize(pessoa) as i32;
    if id >= read_u32_as_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let k = read_u32_as_usize(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f)) + campo) as i32;
    if k < 0 { 0 } else { (k as usize).wrapping_mul(0x50).wrapping_add(read_u32_as_usize(vetor)) }
}



// ---------- 267a leva: o lance aceito pelo clube vendedor, a notícia da oferta feita e o empréstimo possível (transfer_manager.cpp) ----------








/// Person -> club of their contract (stored in G_CONTRACT_LOOKUP_RESULT); 0 without contract, or for a type-5 contract
/// when `sem_5`.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080], e o
/// clube dele; sem contrato, ou do tipo 5 (+0x35) quando `sem_5`: 0.
pub(crate) unsafe fn contract_club_optional_type5(pessoa: usize, sem_5: bool) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || (sem_5 && read_u8(c + 0x35) & 0x3f == 5) {
        0
    } else {
        transfer_manager_club_by_id(read_u32_as_usize(c + 4))
    }
}



/// Offer status (+0x2c) is 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3 (the test the original repeats).
///
/// [pt] A oferta segue ativa (status 2, 3, 4, 0x10, 0x14, ou 0xf com +0x2e ≥ 3), o teste que a original repete.
pub(crate) unsafe fn is_offer_status_settled(o: usize) -> bool {
    let s = read_u8(o + 0x2c);
    matches!(s, 2 | 3 | 4 | 0x10 | 0x14) || (s == 0xf && read_u8(o + 0x2e) >= 3)
}

/// If the offer's exchange list (+4) has count 0, destroys and frees it and zeroes +4.
///
/// [pt] A lista de troca (+4) vazia (palavra +0 zerada) é destruída (0x68d040: mov ecx, lista -> thiscall, ret 0), liberada
/// (0x944c94: push; add esp, 4 -> cdecl) e +4 zera.
pub(crate) unsafe fn free_empty_exchange_list(o: usize) {
    let l = read_u32_as_usize(o + 4);
    if read_i16(l) != 0 {
        return;
    }
    if l != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destruir(l);
        liberar(l);
    }
    write_u32_from_usize(o + 4, 0);
}

/// the seller accepts the bid: status 5; on loans, target interest below 0.75 gives the refusal to move; exchange
/// players wanting more than 1.5 times the seller wage cap leave, with their value added to the fee; the offer goes to
/// the next slice
/// Original: `TRANSFER_MANAGER::accept_transfer_bid` (0x8c0080, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c0080 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: o STAFF_CONTRACT
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
pub(crate) unsafe extern "thiscall" fn accept_transfer_bid(this: usize, vaga: usize, marcar: u32) {
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_MARK_PLAYER_UNDER_OFFER);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ACCEPTED_NEWS);
    let estatuto: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_GET_SQUAD_STATUS_FOR_CLUB);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_PLAYER_REJECTS_MOVE_NEWS);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_IS_CLOSED);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let noticia_troca: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_SWAP_PLAYER_REJECTS_NEWS);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let resposta_ia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let conversa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_TALKS_NEWS);
    let mut contrato = [0u8; 0x60];
    let pc = contrato.as_mut_ptr() as usize;
    novo_contrato(pc);
    let o0 = read_u32_as_usize(vaga);
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(o0 + 0xc));
    let comprador = transfer_manager_club_by_id(read_u32_as_usize(o0 + 8));
    let vendedor = contract_club_optional_type5(alvo, true);
    let o = read_u32_as_usize(vaga);
    write_u8(o + 0x2e, 0);
    if marcar != 0 {
        write_u8(o + 0x2d, 1);
        sob_oferta(read_u32_as_usize(this + 0x856), alvo, comprador, read_u32_as_usize(read_u32_as_usize(vaga)));
    }
    write_u8(o + 0x2e, 0);
    write_u8(o + 0x2c, 5);
    let mut humanos: u8 = 0;
    if human_manager_record_checked(transfer_manager_club_by_id(read_u32_as_usize(o + 8))) != 0 {
        humanos = 1;
    }
    if vendedor != 0 && human_manager_record_checked(vendedor) != 0 {
        humanos = humanos.wrapping_add(1);
    }
    aceita(this, o);
    if read_u8(o + 0x11) == 0xb {
        let e = if transfer_manager_is_human_club(comprador) { 1 } else { estatuto(read_u32_as_usize(this + 0x856), alvo, comprador, 0) };
        let dw = (marcar & 0xffff_ff00) | e as u32;
        let mut motivo: u32 = 0;
        let r = interesse(read_u32_as_usize(this + 0x856), alvo, comprador, 0, 0, dw, &mut motivo);
        crate::comparacao::ramo("cqd_2 empréstimo");
        if !(r >= 0.75) { // cqd2: o alvo não quer se mudar
            recusa_mudar(this, o, motivo);
            write_u8(o + 0x2c, 0xf);
            write_u8(o + 0x2e, 3);
            let a2 = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
            let atual = contract_club_optional_type5(a2, false);
            desmarcar(read_u32_as_usize(this + 0x856), transfer_manager_club_by_id(read_u32_as_usize(o + 8)), a2);
            let l = read_u32_as_usize(o + 4);
            if l != 0 {
                write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
                loop {
                    let p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
                    if p == 0 {
                        break;
                    }
                    desmarcar(read_u32_as_usize(this + 0x856), atual, transfer_manager_staff_by_id(read_u32_as_usize(p)));
                }
            }
            agendar(this, o, 0xa, 0);
            'fechar: {
                if !is_offer_status_settled(o) {
                    break 'fechar;
                }
                let l = read_u32_as_usize(o);
                if l != 0 {
                    write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
                    loop {
                        let p = transfer_manager_llist_next(read_u32_as_usize(o));
                        if p == 0 {
                            break;
                        }
                        if fechada(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(read_u32_as_usize(p).wrapping_mul(4)))) == 0 {
                            break 'fechar;
                        }
                    }
                }
                let m = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(o + 0xc).wrapping_mul(8));
                if read_u8(m) & 1 != 0 {
                    write_u8(m, read_u8(m) & 0xfe);
                }
            }
            let ne = (read_u8(o + 0x11) != 0xb) as u32;
            let x = entrada(read_u32_as_usize(this + 0x856), transfer_manager_club_by_id(read_u32_as_usize(o + 8)), transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc)), ne, 0);
            if x != 0 {
                // ponytail: a original empurra o setne com os bytes altos do ponteiro da entrada; a 0x843e10 só lê o byte
                let t = (x as u32 & 0xffff_ff00) | (read_u8(x) != 0xb) as u32;
                situacao(read_u32_as_usize(this + 0x856), transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc)), transfer_manager_club_by_id(read_u32_as_usize(o + 8)), 1, t);
            }
            return;
        }
    }
    if read_u32_as_usize(o + 4) != 0 {
        let l = read_u32_as_usize(o + 4);
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
        while p != 0 {
            let pessoa = transfer_manager_staff_by_id(read_u32_as_usize(p));
            if transfer_manager_is_human_club(vendedor) {
                write_u8(p + 5, 2);
            }
            let cargo = read_u8(p + 5);
            // ponytail: a original empurra o edx com o byte +5 e os bytes altos do que a 0x600a50 deixou nele; a 0x85fc50
            // só lê o byte (mov cl, [esp+0x284])
            let salario = desejado(pc, pessoa, vendedor, u32::MAX, cargo as u32, 0xb, 0, 0);
            let t = teto(financas(read_u32_as_usize(G_FINANCE_TABLE), vendedor), pessoa, cargo as u32);
            crate::comparacao::ramo("cqd_2 troca cara");
            if (t as f64 * 1.5) < salario as f64 { // cqd2: o jogador da troca caro demais
                let v = valor(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa, comprador, 0, u32::MAX, 0).wrapping_add(read_u32_as_usize(o + 0x1b) as i32);
                noticia_troca(this, o, pessoa, v);
                tirar(read_u32_as_usize(o + 4), p);
                let m = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(pessoa).wrapping_mul(8)) + 2;
                write_u8(m, read_u8(m).wrapping_sub(1));
                write_u32_from_usize(o + 0x1b, v as u32 as usize);
                if vendedor != 0 && !transfer_manager_is_human_club(vendedor) {
                    resposta_ia(this, vaga, 1);
                    if is_offer_status_settled(o) {
                        free_empty_exchange_list(o);
                        return;
                    }
                }
            }
            p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
        }
        free_empty_exchange_list(o);
    }
    if humanos != 0 {
        conversa(this, o);
    }
    crate::comparacao::ramo("cqd_2 lance aceito");
    agendar(this, o, 0, 1); // cqd2: a próxima fatia
    if read_u32_as_usize(o + 4) == 0 || transfer_manager_is_human_club(vendedor) {
        return;
    }
    let l = read_u32_as_usize(o + 4);
    write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
    loop {
        let p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
        if p == 0 {
            break;
        }
        let x = entrada(read_u32_as_usize(this + 0x856), vendedor, transfer_manager_staff_by_id(read_u32_as_usize(p)), 0, 0);
        if x != 0 {
            write_u32_from_usize(x + 0x17, read_u32_as_usize(o + 0x20));
        }
    }
}

/// Comparador da 0x8c0080: só a original (muda o status, reagenda, solta a troca, aloca notícias e chama a resposta da
/// IA).
pub(crate) unsafe extern "thiscall" fn comparar_accept_transfer_bid(this: usize, vaga: usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ACCEPT_TRANSFER_BID)(this, vaga, marcar) // ponytail: só a original
}

/// news 0xfa0 of an offer made (target, clubs, values, exchange, contract, window) to the interested human managers
/// Original: `TRANSFER_MANAGER::post_offer_made_news` (0x8d3090, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d3090 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a notícia e a data
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
pub(crate) unsafe extern "thiscall" fn post_offer_made_news(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let recorde: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_CLUB_RECORDS_OFFER_BREAKS_FEE_RECORD);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let janela: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
    let mut alvo = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
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
    g(0, read_u32_as_usize(alvo) as u32);
    g(1, read_u32_as_usize(read_u32_as_usize(alvo + 4) + 0x33) as u32);
    g(2, read_u32_as_usize(read_u32_as_usize(alvo + 8) + 0x33) as u32);
    g(3, read_u32_as_usize(read_u32_as_usize(alvo + 0xc) + 0x33) as u32);
    g(4, read_u32_as_usize(o + 8) as u32);
    g(5, if vendedor != 0 { read_u32_as_usize(vendedor) as u32 } else { u32::MAX });
    g(6, read_u32_as_usize(o + 0x20) as u32);
    g(7, read_u32_as_usize(o + 0x24) as u32);
    g(8, read_u32_as_usize(o + 0x1b) as u32);
    g(9, read_u8(o + 0x2c) as i8 as i32 as u32);
    g(0xa, total(o, 0, 1, 0, 1) as u32);
    g(0xb, read_u8(o + 0x2f) as i8 as i32 as u32);
    g(0xe, (read_u32_as_usize(o + 0x31) & 0xf) as u32);
    if read_u8(o + 0x2f) != 0 && vendedor != 0 && tipo(read_u8(alvo + 0x3d) as i8) != 0 {
        let c = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), alvo);
        if c != 0 {
            let (ha, ca) = (read_i16(G_CURRENT_DATE_YEAR) as u16, read_i16(c + 0x2f) as u16);
            if ha < ca || (ha == ca && read_i16(G_CURRENT_DATE) < read_i16(c + 0x2d)) {
                g(0xc, read_i16(c + 0x2d) as i32 as u32);
                g(0xd, ca as u32);
            } else {
                g(0xc, 0);
                g(0xd, 0);
            }
            g(0xf, (((read_u8(c + 0x35) << 2) as i8) >> 2) as i32 as u32);
        } else {
            g(0xc, 0);
            g(0xd, 0);
            g(0xf, 0);
        }
    }
    let mut k: u8 = 0x18;
    let mut conta: u8 = 0;
    let l = read_u32_as_usize(o + 4);
    if l != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        loop {
            let p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
            if p == 0 {
                break;
            }
            alvo = transfer_manager_staff_by_id(read_u32_as_usize(p));
            if (conta as i8) < 3 {
                g(k as i8, read_u32_as_usize(alvo) as u32);
                g(k.wrapping_add(1) as i8, read_u32_as_usize(read_u32_as_usize(alvo + 4) + 0x33) as u32);
                g(k.wrapping_add(2) as i8, read_u32_as_usize(read_u32_as_usize(alvo + 8) + 0x33) as u32);
                g(k.wrapping_add(3) as i8, read_u32_as_usize(read_u32_as_usize(alvo + 0xc) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
            conta = conta.wrapping_add(1);
        }
    }
    g(0x17, conta as i8 as i32 as u32);
    let mut marca: u8 = 0;
    let s = read_u32_as_usize(alvo + 0x24);
    if s != 0 && segunda(s) != 0 {
        marca = 1;
        g(0x11, read_u32_as_usize(read_u32_as_usize(alvo + 0x24)) as u32);
    } else {
        g(0x11, u32::MAX);
    }
    if recorde(read_u32_as_usize(G_CLUB_RECORDS), o, 1) != 0 {
        marca |= 2;
    }
    g(0x10, marca as i8 as i32 as u32);
    g(k as i8, read_u8(o + 0x11) as i8 as i32 as u32);
    for i in 0..(n as i8).max(0) as usize {
        entregar(read_u32_as_usize(G_NEWS), pn, clubes[i]);
    }
    if read_u8(o + 0x2f) == 0 {
        return;
    }
    if read_u8(o + 0x30) & 0xf != 5 {
        g(0x12, 3);
        return;
    }
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    janela(pd, 0);
    let hoje_igual = read_i16(G_CURRENT_DATE_YEAR) == read_i16(pd + 2) && read_i16(G_CURRENT_DATE) == read_i16(pd);
    g(0x12, if hoje_igual { 0 } else { 5 });
    g(0xc, read_i16(pd) as i32 as u32);
    g(0xd, read_i16(pd + 2) as u16 as u32);
}

/// Comparador da 0x8d3090: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_made_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_OFFER_MADE_NEWS)(this, o) // ponytail: só a notícia
}

/// Returns the virtual method at vtable +0x20 of a nation's transfer rules object (4 arguments).
///
/// [pt] O virtual +0x20 da regra de nação `r` (push 0; push mensagem; push jogador; push clube; mov ecx, regra: 4 dwords).
pub(crate) unsafe fn rules_vfunc_20(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 {
    core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0x20))
}

/// Returns the virtual method at vtable +0x24 of a nation's transfer rules object (5 arguments).
///
/// [pt] O virtual +0x24 da regra de nação `r` (push 1; push &data; push 0; push jogador; push clube; mov ecx, regra: 5 dwords).
pub(crate) unsafe fn rules_vfunc_24(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 {
    core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0x24))
}

/// whether the club can take the player on loan: current club, contract record, playing role, whether the club can
/// reach him, the rule of the club nation and of the current club month by month over the loan period; the reason text
/// in the message
/// Original: `TRANSFER_MANAGER::can_loan_player` (0x8d0040, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8d0040 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH sem destrutor: a data e os textos locais):
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
pub(crate) unsafe extern "thiscall" fn transfer_manager_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let nome: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let acertada: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_AGREED_CONTRACT_OFFER);
    let adjetivo: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATION_GET_NATIONALITY_ADJECTIVE);
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_SET);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let atual = contract_club_optional_type5(jogador, true);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    let dia = dia_do_mes(0xae2c90);
    if atual == 0 {
        return 0;
    }
    if ficha(read_u32_as_usize(G_CONTRACT_MANAGER), jogador) != 0 {
        return 0;
    }
    let mut buf_nome = [0u8; 0x130];
    let pn = buf_nome.as_mut_ptr() as usize;
    if read_u8(jogador + 0x3d) != 0xb {
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
                let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
                let id = read_u32_as_usize(jogador);
                let m = read_u32_as_usize(gm + 0x86e).wrapping_add(id.wrapping_mul(8));
                let mut c = atual;
                if read_u8(m) & 0x40 != 0 {
                    let a = acertada(read_u32_as_usize(G_CONTRACT_MANAGER), id);
                    if a != 0 {
                        c = transfer_manager_club_by_id(read_u32_as_usize(a + 4));
                    } else {
                        let m = read_u32_as_usize(gm + 0x86e).wrapping_add(read_u32_as_usize(jogador).wrapping_mul(8));
                        write_u8(m, read_u8(m) & 0xbf);
                    }
                }
                let x = read_u8(c + 0x52) as i8 as i32;
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
            0x19 => read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_PAYS_BASQUE).wrapping_mul(0x122)),
            0x28 => read_u32_as_usize(clube + club::NATION),
            0x1a => read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_SPAIN).wrapping_mul(0x122)),
            _ => return 0,
        };
        adjetivo(nacao, pa, 0x64, 0xd);
        let x = read_u8(clube + club::SHORT_GENDER_NAME) as i8 as i32;
        texto(0xde1f64, 0xa8f30c, x, x, 0x64, 0x64, clube + club::SHORT_NAME, pa);
        return 0;
    }
    for k in 0..0x1c {
        let r = read_u32_as_usize(this + 0x8ac + k * 4);
        if read_u32_as_usize(r + 0xa) == read_u32_as_usize(clube + club::NATION) {
            if r != 0 && rules_vfunc_20(r)(r, clube, jogador, msg, 0) == 0 {
                return 0;
            }
            break;
        }
    }
    let mut regra = 0;
    for k in 0..0x1c {
        let r = read_u32_as_usize(this + 0x8ac + k * 4);
        if read_u32_as_usize(r + 0xa) == read_u32_as_usize(atual + 0x53) {
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
        let (na, nd) = (read_i16(p + 2) as u16, read_i16(p));
        let (da, dd) = (read_i16(pd + 2) as u16, read_i16(pd));
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
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_TRANSFER_MANAGER_CAN_LOAN_PLAYER);
    let mut regioes = [(0xae1080usize, 4usize), (0xde1f64, 0x100), (0, 0), (0, 0)];
    let mut k = 2;
    if msg != 0 {
        regioes[k] = (msg, 0x100);
        k += 1;
    }
    let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
    if gm != 0 && read_u32_as_usize(gm + 0x86e) != 0 {
        regioes[k] = (read_u32_as_usize(gm + 0x86e).wrapping_add(read_u32_as_usize(jogador).wrapping_mul(8)), 1);
        k += 1;
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes[..k], || o(this, clube, jogador, msg),
        || crate::aleatorio::sem_contar(|| transfer_manager_can_loan_player(this, clube, jogador, msg)));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_can_loan_player(clube {:#x}, jogador {:#x}): original {} rust {}, memória {}",
        clube, jogador, r, n, if igual { "igual" } else { "diferente" }));
    r
}

// ---------- 266a leva: a copa e as transferências: os jogos da rodada levados às datas marcadas, o jogador que sai da troca e as permissões de trabalho da oferta (Cup.cpp, transfer_manager.cpp) ----------








/// Staff id -> person record (0x6e bytes each in G_STAFF_TABLE).
///
/// [pt] A pessoa (0x6e bytes em [0xae23bc]) e o clube (0x245 bytes em [0xae23b4]) pelo id.
pub(crate) unsafe fn transfer_manager_staff_by_id(id: usize) -> usize {
    read_u32_as_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(0x6e))
}

/// Club id -> club record (0x245 bytes each in G_CLUB_TABLE).
pub(crate) unsafe fn transfer_manager_club_by_id(id: usize) -> usize {
    read_u32_as_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(0x245))
}

/// Person -> main contract (CONTRACT_MANAGER::get_contract), also written to G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), gravado em [0xae1080].
pub(crate) unsafe fn transfer_manager_get_contract_stored(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// If the offer's list at +4 has count 0, destroys and frees it and zeroes +4 (reads the count before the null check,
/// like the original).
///
/// [pt] Lista vazia (palavra +0 = 0) da oferta (+4): destruída (0x68d040), liberada (0x944c94) e +4 = 0, como a original
/// (que lê a palavra antes de testar o ponteiro).
pub(crate) unsafe fn free_empty_offer_list(oferta: usize) {
    let l = read_u32_as_usize(oferta + 4);
    if read_i16(l) != 0 {
        return;
    }
    if l != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destruir(l);
        liberar(l);
    }
    write_u32_from_usize(oferta + 4, 0);
}

/// Cup + round index -> round entry (0x68 bytes each at [copa+0xa3]).
///
/// [pt] A entrada da rodada (0x68 bytes em [copa + 0xa3], o índice como o dword que a original multiplica).
pub(crate) unsafe fn cup_round_entry(copa: usize, rodada: u32) -> usize {
    read_u32_as_usize(copa + 0xa3).wrapping_add((rodada as usize).wrapping_mul(0x68))
}

/// Collects into `lista` the cup's fixtures on the round's date and slice (FIXTURE_MANAGER
/// collect_comp_fixtures_on_day); returns the count.
///
/// [pt] Os jogos da copa na data e fatia da rodada, numa lista (0x5abd90: push lista; push liga +0x44; push rodada; push
/// id; push fatia +6; push dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c).
pub(crate) unsafe fn collect_round_fixtures(copa: usize, data: usize, rodada: u32, lista: usize) -> u32 {
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_COLLECT_COMP_FIXTURES_ON_DAY);
    // ponytail: a original empurra o dia, a fatia e o tipo em registradores com sobra nos bytes altos; a 0x5abd90 só
    // lê as palavras e os bytes
    jogos(read_u32_as_usize(G_FIXTURE_MANAGER), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(cup_round_entry(copa, rodada) + 6) as u32,
        read_u32_as_usize(read_u32_as_usize(copa + 4)) as u32, rodada, read_u8(copa + 0x44) as u32, lista) & 0xffff
}

/// Fixture `f` is on the round entry's scheduled date: day, year (+cup year +0x40), slice and the league type (+0x44)
/// match.
///
/// [pt] O jogo `f` está na data marcada da rodada: dia (+0x2a, sem sinal) = +3 da rodada, ano (+0x28) = +5 + o da copa
/// (+0x40), fatia (+0x3f) = +6 e o tipo (+0x42) da liga.
pub(crate) unsafe fn fixture_on_round_date(copa: usize, e: usize, f: usize) -> bool {
    read_i16(f + 0x2a) as u16 as i32 == read_i16(e + 3) as i32
        && read_i16(f + 0x28) as u16 as i32 == read_u8(e + 5) as i8 as i32 + read_i16(copa + 0x40) as i32
        && read_u8(f + 0x3f) as i32 == read_u8(e + 6) as i8 as i32
        && read_u8(f + 0x42) == read_u8(copa + 0x44)
}



/// removes the player from the offer part exchange list (or from his other offers), his value goes to the price, the
/// news, the computer seller reply and the offer goes on
/// Original: `TRANSFER_MANAGER::remove_exchange_player` (0x8c41c0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8c41c0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a lista local):
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
pub(crate) unsafe extern "thiscall" fn remove_exchange_player(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_EXCHANGE_PLAYER_REMOVED_NEWS);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let menos_uma = |p: usize| {
        let a = read_u32_as_usize(this + 0x86e) + read_u32_as_usize(p) * 8 + 2;
        write_u8(a, read_u8(a).wrapping_sub(1));
    };
    let mut oferta = oferta;
    if so_esta != 0 {
        let c = transfer_manager_get_contract_stored(transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)));
        let dono = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) } else { 0 };
        let troca = read_u32_as_usize(oferta + 4);
        if troca == 0 {
            destruir(l);
            return;
        }
        write_u32_from_usize(troca + 0xa, read_u32_as_usize(troca + 2));
        let mut it = proximo(read_u32_as_usize(oferta + 4));
        while it != 0 && read_u32_as_usize(pessoa) != read_u32_as_usize(it) {
            it = proximo(read_u32_as_usize(oferta + 4));
        }
        if it != 0 {
            tirar(read_u32_as_usize(oferta + 4), it);
            menos_uma(pessoa);
            let v = read_u32_as_usize(pessoa + staff::VALUE).wrapping_add(read_u32_as_usize(oferta + 0x1b));
            noticia(this, oferta, pessoa, v, motivo);
            let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if dono != 0 && human_club(dono, 0, 1, 0, 0) == 0 {
                let resposta: unsafe extern "thiscall" fn(usize, *mut usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
                resposta(this, &mut oferta, 1);
                let s = read_u8(oferta + 0x2c);
                if matches!(s, 2 | 3 | 0x14 | 0x10 | 4) || (s == 0xf && read_u8(oferta + 0x2e) >= 3) {
                    free_empty_offer_list(oferta);
                    destruir(l);
                    return;
                }
            }
            crate::comparacao::ramo("cqc_7 sai da troca da oferta");
            write_u32_from_usize(oferta + 0x1b, v);
            seguir(this, oferta);
        }
        free_empty_offer_list(oferta);
    } else {
        if read_u8(read_u32_as_usize(this + 0x86e) + read_u32_as_usize(pessoa) * 8 + 2) != 0 {
            let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
            trocas(this, transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)), l);
            write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
            let mut achou = false;
            let mut it = proximo(l);
            while it != 0 {
                let o = read_u32_as_usize(read_u32_as_usize(this) + read_u32_as_usize(it) * 4);
                if o != oferta {
                    if read_u32_as_usize(o + 4) != 0 {
                        let t = read_u32_as_usize(o + 4);
                        write_u32_from_usize(t + 0xa, read_u32_as_usize(t + 2));
                        let mut x = proximo(read_u32_as_usize(o + 4));
                        while x != 0 {
                            if read_u32_as_usize(pessoa) == read_u32_as_usize(x) {
                                achou = true;
                                tirar(read_u32_as_usize(o + 4), x);
                                menos_uma(pessoa);
                                break;
                            }
                            x = proximo(read_u32_as_usize(o + 4));
                        }
                        free_empty_offer_list(o);
                    }
                    if achou {
                        crate::comparacao::ramo("cqc_7 sai da troca das outras");
                        let v = read_u32_as_usize(pessoa + staff::VALUE).wrapping_add(read_u32_as_usize(o + 0x1b));
                        noticia(this, o, pessoa, v, 0);
                        write_u32_from_usize(o + 0x1b, v);
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
pub(crate) unsafe extern "thiscall" fn comparar_remove_exchange_player(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_REMOVE_EXCHANGE_PLAYER)(this, pessoa, oferta, so_esta, motivo) // ponytail: o efeito segue por outras rotinas
}

/// Whether the person needs a work permit at the club, the contract offer found for them, and whether the permit is
/// granted; returns (needs, offer, granted).
///
/// [pt] A permissão de trabalho da pessoa no clube: precisa (0x8c7560: push 0; push clube; push pessoa; mov ecx, this ->
/// thiscall, ret 0xc), a proposta de contrato dela ao clube (0x4cc9e0: push id do clube; push id da pessoa; mov ecx,
/// [0xae1050] -> thiscall, ret 8) e a concessão (0x8c7950: push 0; push 0; push proposta; push nação do clube +0x53; push
/// pessoa; mov ecx, this -> thiscall, ret 0x14, sem sorteio). Devolve (precisa, proposta, concedida).
pub(crate) unsafe fn check_work_permit(this: usize, pessoa: usize, clube: usize) -> (bool, usize, bool) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if precisa(this, pessoa, clube, 0) == 0 {
        return (false, 0, false);
    }
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
    let r = proposta(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(pessoa) as u32, read_u32_as_usize(clube) as u32);
    if r == 0 {
        return (true, 0, false);
    }
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_WORK_PERMIT_GRANTED);
    (true, r, concedida(this, pessoa, read_u32_as_usize(clube + club::NATION), r, 0, 0) != 0)
}

/// the work permit of the target at the buyer (refused: the offer collapses, status 0x10, rescheduled in 10 days,
/// returns 0) and of each part exchange player at the seller (refused: he leaves the exchange and his value goes to the
/// price); the news and the offer goes on
/// Original: `TRANSFER_MANAGER::check_offer_work_permits` (0x8c7690, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c7690 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::check_offer_work_permits(oferta, quieto)`,
/// do processamento da oferta (status 0xa, 0x8c6880) e da conclusão (0x8cbc40). O dono do contrato do alvo (0x4cf5a0,
/// [0xae1080]; qualquer tipo). Alvo (+0xc) com permissão a pedir no comprador (+8) e proposta, sem concessão
/// (check_work_permit): a oferta desfeita (0x8de790 com 0x13), status 0x10, as propostas fechadas (0x8c6db0), as marcas de
/// lance saem (0x84aeb0 no gerente de listas [this+0x856]: o comprador e o alvo, depois o dono do contrato do alvo e cada
/// jogador da troca), reagendada em 10 dias (0x8c6220) e devolve 0. Concedida (ou sem proposta) e sem `quieto`: a
/// notícia da concessão (0x8e3130). Depois, cada jogador da troca (+4) no dono do contrato do alvo: recusado, a notícia
/// (0x8e35a0 com o valor +0x52 + o preço), sai da troca (0x68d1f0), o registro de 8 bytes dele perde uma oferta e o
/// preço (+0x1b) vira o valor; concedido sem `quieto`, a notícia. A lista vazia sai; sem `quieto`, a oferta segue
/// (0x8c2f50). Devolve 1.
pub(crate) unsafe extern "thiscall" fn check_offer_work_permits(this: usize, oferta: usize, quieto: u32) -> u32 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let concedida_noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_WORK_PERMIT_GRANTED_NEWS);
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc));
    let comprador = transfer_manager_club_by_id(read_u32_as_usize(oferta + 8));
    let c = transfer_manager_get_contract_stored(alvo);
    let dono = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) } else { 0 };
    let (precisa, proposta, concedida) = check_work_permit(this, alvo, comprador);
    if precisa {
        if proposta != 0 && !concedida {
            crate::comparacao::ramo("cqc_7 permissao recusada");
            let desfeita: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_COLLAPSED_NEWS);
            let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
            let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
            let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
            desfeita(this, oferta, comprador, 0x13);
            write_u8(oferta + 0x2c, 0x10); // cqc7: a oferta cai pela permissão
            fechar(this, oferta);
            let alvo = transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc));
            let c = transfer_manager_get_contract_stored(alvo);
            let dono = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) } else { 0 };
            desmarcar(read_u32_as_usize(this + 0x856), transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)), alvo);
            if read_u32_as_usize(oferta + 4) != 0 {
                let t = read_u32_as_usize(oferta + 4);
                write_u32_from_usize(t + 0xa, read_u32_as_usize(t + 2));
                let mut it = proximo(read_u32_as_usize(oferta + 4));
                while it != 0 {
                    desmarcar(read_u32_as_usize(this + 0x856), dono, transfer_manager_staff_by_id(read_u32_as_usize(it)));
                    it = proximo(read_u32_as_usize(oferta + 4));
                }
            }
            agendar(this, oferta, 0xa, 0);
            return 0;
        }
        if quieto == 0 {
            concedida_noticia(this, alvo, comprador, oferta);
        }
    }
    if read_u32_as_usize(oferta + 4) != 0 {
        let t = read_u32_as_usize(oferta + 4);
        write_u32_from_usize(t + 0xa, read_u32_as_usize(t + 2));
        let mut it = proximo(read_u32_as_usize(oferta + 4));
        while it != 0 {
            let x = transfer_manager_staff_by_id(read_u32_as_usize(it));
            let (precisa, proposta, concedida) = check_work_permit(this, x, dono);
            if precisa {
                if proposta != 0 && !concedida {
                    crate::comparacao::ramo("cqc_7 troca recusada pela permissao");
                    let v = read_u32_as_usize(x + 0x52).wrapping_add(read_u32_as_usize(oferta + 0x1b));
                    let recusada: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_EXCHANGE_PLAYER_PERMIT_REFUSED_NEWS);
                    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
                    recusada(this, oferta, x, v);
                    tirar(read_u32_as_usize(oferta + 4), it);
                    let a = read_u32_as_usize(this + 0x86e) + read_u32_as_usize(x) * 8 + 2;
                    write_u8(a, read_u8(a).wrapping_sub(1));
                    write_u32_from_usize(oferta + 0x1b, v);
                } else if quieto == 0 {
                    concedida_noticia(this, x, dono, oferta);
                }
            }
            it = proximo(read_u32_as_usize(oferta + 4));
        }
        free_empty_offer_list(oferta);
    }
    if quieto == 0 {
        let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
        seguir(this, oferta);
    }
    1
}

/// Comparador da 0x8c7690: só a original (as notícias alocam, a oferta é reagendada e segue pela 0x8c2f50). O plano,
/// feito antes pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo recusado devolve 0 com a
/// oferta no status 0x10; senão 1.
pub(crate) unsafe extern "thiscall" fn comparar_check_offer_work_permits(this: usize, oferta: usize, quieto: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CHECK_OFFER_WORK_PERMITS);
    let (precisa, proposta, concedida) = check_work_permit(this, transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)), transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)));
    let recusada = precisa && proposta != 0 && !concedida;
    let r = o(this, oferta, quieto);
    let ok = if recusada { r == 0 && read_u8(oferta + 0x2c) == 0x10 } else { r == 1 };
    PLACAR.registrar(ok, || format!("check_offer_work_permits(oferta {:x}) recusada prevista {} devolveu {}", oferta, recusada, r));
    r
}

// ---------- 266a leva: o dia das transferências: o dia da gerência, a oferta que vence hoje e as restrições da oferta (transfer_manager.cpp) ----------








/// Person -> club of their contract (stored in G_CONTRACT_LOOKUP_RESULT); 0 without contract, or for type 5 (+0x35)
/// when `sem_5`.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080] como
/// a original faz, e o clube dele (+4); sem contrato, ou com o tipo 5 em +0x35 quando `sem_5`: 0.
pub(crate) unsafe fn contract_club_filtered(pessoa: usize, sem_5: bool) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || (sem_5 && read_u8(c + 0x35) & 0x3f == 5) {
        0
    } else {
        transfer_manager_club_by_id(read_u32_as_usize(c + 4))
    }
}

/// Club is controlled by a human manager (HUMAN_MANAGER is_human_club with 0,1,0,0).
///
/// [pt] O clube é humano (0x600a50: push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
pub(crate) unsafe fn transfer_manager_is_human_club(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, 0, 1, 0, 0) != 0
}

/// Reschedules the offer `dias` days ahead (TRANSFER_MANAGER::schedule_offer, 0 slices).
///
/// [pt] Reagenda a oferta (0x8c6220: push 0; push dias; push oferta; mov ecx, this -> thiscall, ret 0xc).
pub(crate) unsafe fn schedule_offer_days(this: usize, o: usize, dias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    f(this, o, dias, 0);
}

/// Next node of a game LLIST (0x68d1b0), advancing its cursor.
///
/// [pt] O próximo da lista (0x68d1b0: mov ecx, lista -> thiscall, ret 0).
pub(crate) unsafe fn transfer_manager_llist_next(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LLIST_GET_NEXT);
    f(l) as usize
}

/// Clears the shortlist bid markers of the offer's buyer for the target, and of the target's contract club for each
/// player in the exchange list.
///
/// [pt] O bloco que a 0x8c63a0 repete: o clube do contrato do alvo (sem olhar o tipo 5), os lances do comprador (+8) pelo
/// alvo (+0xc) desmarcados na lista de observação (0x84aeb0: push jogador; push clube; mov ecx, [this+0x856] ->
/// thiscall, ret 8) e os do clube do contrato por cada jogador da troca (lista em +4, cursor no começo).
pub(crate) unsafe fn clear_offer_bid_markers(this: usize, o: usize) {
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
    let atual = contract_club_filtered(alvo, false);
    desmarcar(read_u32_as_usize(this + 0x856), transfer_manager_club_by_id(read_u32_as_usize(o + 8)), alvo);
    let l = read_u32_as_usize(o + 4);
    if l == 0 {
        return;
    }
    write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
    loop {
        let p = transfer_manager_llist_next(read_u32_as_usize(o + 4));
        if p == 0 {
            break;
        }
        desmarcar(read_u32_as_usize(this + 0x856), atual, transfer_manager_staff_by_id(read_u32_as_usize(p)));
    }
}

/// one slice of the transfer day: window notices, overdue offers rescheduled, the slice bucket processed; in slice 2
/// the base date moves to tomorrow, instalments at month end, nation rules; on day 100 the +0x1e of non-EU people at
/// the club for years
/// Original: `TRANSFER_MANAGER::daily_update` (0x8c5880, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c5880 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
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
pub(crate) unsafe extern "thiscall" fn transfer_manager_daily_update(this: usize, hoje: usize, fatia: u32) {
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let chamar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PROCESS_FREE_TRANSFER_QUEUE);
    let janelas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UPDATE_TRANSFER_WINDOW_NEWS);
    let processar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PROCESS_DUE_OFFER);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let prestacoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_TRANSFER_INSTALMENTS);
    let antigas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_OLD_PAST_TRANSFERS);
    let permissoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UPDATE_WORK_PERMITS);
    let futuras: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PROCESS_DUE_FUTURE_TRANSFERS);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let f = fatia as u8;
    let mut t20 = [0u8; 8];
    if read_i16(this + 0x84f) == 0 {
        copia(this + 0x84d, t20.as_mut_ptr() as usize, hoje);
        write_u8(this + 0x855, f);
    }
    if read_i16(hoje + 2) != read_i16(this + 0x84f) || read_i16(hoje) != read_i16(this + 0x84d) || read_u8(this + 0x855) != f {
        return;
    }
    if f == 0 {
        janelas(this, hoje);
    }
    if read_i16(hoje) as i32 % 7 == 0 {
        let mut i: i32 = 0;
        while i < read_u32_as_usize(this + 4) as i32 {
            let o = read_u32_as_usize(read_u32_as_usize(this).wrapping_add((i as usize).wrapping_mul(4)));
            if o != 0 {
                let (ha, oa) = (read_i16(hoje + 2) as u16, read_i16(o + 0x14) as u16);
                let vencida = ha > oa || (ha == oa && read_i16(hoje) > read_i16(o + 0x12));
                let s = read_u8(o + 0x2c);
                if vencida && s != 0x13 && s != 7 && s != 8 && s != 0xa {
                    schedule_offer_days(this, o, 1);
                }
            }
            i += 1;
        }
    }
    write_u8(this + 0x855, read_u8(this + 0x855).wrapping_add(1));
    chamar(this);
    let balde = |this: usize| {
        let k = (read_u8(this + 0x84c) as i8 as i32) * 3 + f as i8 as i32;
        this.wrapping_add((k as isize * 0xe) as usize).wrapping_add(0x18)
    };
    let l = balde(this);
    write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
    loop {
        let p = transfer_manager_llist_next(balde(this));
        if p == 0 {
            break;
        }
        let idx = read_u32_as_usize(p) as i32;
        if idx < read_u32_as_usize(this + 4) as i32 && idx >= 0 {
            let vaga = read_u32_as_usize(this).wrapping_add(idx as usize * 4);
            let o = read_u32_as_usize(vaga);
            if o != 0 && read_i16(hoje + 2) == read_i16(o + 0x14) && read_i16(hoje) == read_i16(o + 0x12)
                && read_u8(o + 0x1a) == f { // cqd1: a oferta desta fatia
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
        write_u8(this + 0x855, 0);
        let j = read_u8(this + 0x84c).wrapping_add(1);
        write_u8(this + 0x84c, j);
        if (j as i8) >= 0x32 {
            write_u8(this + 0x84c, 0);
        }
        let amanha = mais(hoje, t30.as_mut_ptr() as usize, 1);
        let m = mes(hoje);
        if m != mes(amanha) {
            crate::comparacao::ramo("cqd_1 prestações do mês");
            prestacoes(this); // cqd1: as prestações na virada do mês
        }
        if read_i16(hoje) == 0 {
            antigas(this);
            permissoes(this);
        }
        for k in 0..0x1c {
            let r = read_u32_as_usize(this + 0x8ac + k * 4);
            if r != 0 {
                let v: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r)));
                v(r, hoje);
            }
        }
        if read_i16(hoje) as i32 % 7 == 0 {
            permissoes(this);
        }
    } else if f == 0 {
        futuras(this);
    }
    if read_i16(hoje) != 100 || f != 2 {
        return;
    }
    // ponytail: a original empurra o ano em eax com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
    let ano = read_i16(hoje + 2) as u16;
    let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
    let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
    let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
    let mut s = read_u32_as_usize(G_STAFF_TABLE);
    let mut i: i32 = 0;
    while i < read_u32_as_usize(G_STAFF_COUNT) as i32 {
        if meets_residency_naturalisation(s, p2, p5, p10, comunitaria) {
            write_u32_from_usize(s + 0x1e, read_u32_as_usize(read_u32_as_usize(s + 0x39) + 0x53));
        }
        i += 1;
        s += 0x6e;
    }
}

/// Condition for day 100 of 0x8c5880: the person's date (+0x3e/+0x40) is older than the 2/5/10-year threshold of the
/// club's European non-member nation (Spain/Portugal 2, most 5, Italy/Finland/Russia 10) [inference: residency
/// naturalisation].
///
/// [pt] A condição do dia 100 da 0x8c5880 (também para o comparador): a data +0x3e (dia com sinal)/+0x40 (ano sem sinal) da
/// pessoa antes da de 2 anos, o clube na nação [0x9cfa10] sem ser comunitária, e o caso pelo id do clube.
pub(crate) unsafe fn meets_residency_naturalisation(s: usize, p2: usize, p5: usize, p10: usize, comunitaria: unsafe extern "C" fn(usize, usize) -> u32) -> bool {
    let antes = |d: usize| {
        let (da, sa) = (read_i16(d + 2) as u16, read_i16(s + 0x40) as u16);
        da > sa || (da == sa && read_i16(d) > read_i16(s + 0x3e))
    };
    if !antes(p2) {
        return false;
    }
    let j = read_u32_as_usize(s + 0x39);
    if j == 0 {
        return false;
    }
    let c = read_u32_as_usize(j + 0x53);
    if c == 0 {
        return false;
    }
    let n = read_u32_as_usize(c + 0x71);
    if n == 0 || read_u32_as_usize(n) != read_u32_as_usize(KEY_CONTINENT_EUROPE) || comunitaria(s, c) != 0 {
        return false;
    }
    let id = read_u32_as_usize(c);
    if id == read_u32_as_usize(KEY_NATION_SPAIN) || id == read_u32_as_usize(KEY_NATION_PORTUGAL) {
        return true;
    }
    if antes(p5) && id != read_u32_as_usize(KEY_NATION_ITALY) && id != read_u32_as_usize(KEY_NATION_FINLAND) && id != read_u32_as_usize(KEY_NATION_RUSSIA) {
        return true;
    }
    antes(p10)
}

/// Comparador da 0x8c5880: só a original (ela processa as ofertas, que mudam elencos, finanças e notícias). Confere o
/// que ela garante, contra o estado de antes: rodando o dia (hoje igual à base e a fatia igual a +0x855), na fatia 2 a
/// base vira amanhã, +0x855 volta a 0 e +0x84c sobe (de 49 a 0); nas outras +0x855 sobe 1; e no dia 100 da fatia 2
/// cada pessoa que cumpre a condição (o laço é o último passo, então o estado de depois é o dele) fica com +0x1e (a
/// segunda nacionalidade) = a nação do clube: a naturalização do dia 100.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_daily_update(this: usize, hoje: usize, fatia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRANSFER_MANAGER_DAILY_UPDATE);
    let f = fatia as u8;
    let sem_base = read_i16(this + 0x84f) == 0;
    let roda = if sem_base {
        true
    } else {
        read_i16(hoje + 2) == read_i16(this + 0x84f) && read_i16(hoje) == read_i16(this + 0x84d) && read_u8(this + 0x855) == f
    };
    let (f855, j84c) = (if sem_base { f } else { read_u8(this + 0x855) }, read_u8(this + 0x84c));
    let mut amanha = [0u8; 8];
    {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
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
        ok = read_u8(this + 0x855) == 0 && read_u8(this + 0x84c) == esperado && base[..] == amanha[..4];
    } else {
        ok = read_u8(this + 0x855) == f855.wrapping_add(1);
    }
    let mut erradas = 0u32;
    if f == 2 && read_i16(hoje) == 100 {
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
        let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
        let ano = read_i16(hoje + 2) as u16;
        let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
        let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
        let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
        let mut s = read_u32_as_usize(G_STAFF_TABLE);
        for _ in 0..(read_u32_as_usize(G_STAFF_COUNT) as i32).max(0) {
            if meets_residency_naturalisation(s, p2, p5, p10, comunitaria) && read_u32_as_usize(s + 0x1e) != read_u32_as_usize(read_u32_as_usize(s + 0x39) + 0x53) {
                erradas += 1;
            }
            s += 0x6e;
        }
        ok = ok && erradas == 0;
    }
    PLACAR.registrar(ok, || format!("transfer_manager_daily_update(dia {}, fatia {}): +0x855 {} -> {}, +0x84c {} -> {}, pessoas sem a nação do clube em +0x1e {}",
        read_i16(hoje), f, f855, read_u8(this + 0x855), j84c, read_u8(this + 0x84c), erradas));
}

/// dispatches the offer due today by its status: seller response, acceptance, player contract, permits, completion,
/// removal or a new round of the offer
/// Original: `TRANSFER_MANAGER::process_due_offer` (0x8c63a0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c63a0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH: as listas locais, construídas pela 0x68d020 e
/// destruídas pela 0x68d040 na mesma ordem): `TRANSFER_MANAGER::process_due_offer(vaga)`, a oferta que vence hoje, pelo
/// status +0x2c. O vendedor é o clube do contrato do alvo (+0xc; fora o tipo 5), o comprador o +8.
/// - 0: sem vendedor, a aceitação do lance (0x8c0080: push 0; push vaga -> thiscall, ret 8). Vendedor humano: status 4,
///   14 dias (0x8c6220) e os lances desmarcados (clear_offer_bid_markers). Senão, sem a marca +0x1f = 1, a resposta da IA
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
pub(crate) unsafe extern "thiscall" fn process_due_offer(this: usize, vaga: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut l1 = [0u8; 0x10];
    lista_nova(l1.as_mut_ptr() as usize);
    process_due_offer_body(this, vaga);
    destruir(l1.as_mut_ptr() as usize);
}

/// Body of TRANSFER_MANAGER::process_due_offer (0x8c63a0): advances the offer in slot `vaga` by its status (AI
/// response, rejection, contract, permits, completion, news, rescheduling).
pub(crate) unsafe fn process_due_offer_body(this: usize, vaga: usize) {
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ACCEPT_TRANSFER_BID);
    let respond: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let prever: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_PREDICT_SELLING_CLUB_RESPONSE);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_LOAN_OFFER);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_DECISION_FOR_TRANSFER);
    let aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ACCEPTED_NEWS);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_PLAYER_REJECTS_MOVE_NEWS);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_IS_CLOSED);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_OFFER);
    let ia_1a: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_SUBMIT_LOAN_CONTRACT_OFFER);
    let ia_1b: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REVIEW_AI_CLUB_BID);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_OFFER_CONTRACT_TO_OFFER_PLAYER);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_WORK_PERMITS);
    let concluir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_TRY_COMPLETE_OFFER);
    let pass_ball: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_feita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_MADE_NEWS);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let o = read_u32_as_usize(vaga);
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
    let vendedor = contract_club_filtered(alvo, true);
    let comprador = transfer_manager_club_by_id(read_u32_as_usize(o + 8));
    let s = read_u8(o + 0x2c);
    if s == 0 {
        if vendedor == 0 {
            aceitar(this, vaga, 0);
            return;
        }
        if transfer_manager_is_human_club(vendedor) {
            write_u8(o + 0x2c, 4);
            schedule_offer_days(this, o, 0xe);
            clear_offer_bid_markers(this, o);
            return;
        }
        if read_u8(read_u32_as_usize(vaga) + 0x1f) != 1 {
            crate::comparacao::ramo("cqd_1 resposta da IA");
            respond(this, vaga, 0); // cqd1: a resposta da IA ao lance
            return;
        }
        if prever(vendedor, alvo, 0, comprador, -1, 1) != 0 {
            recusar_emprestimo(this, vaga, 0, 0);
            return;
        }
        let r = decidir(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(vaga));
        write_u8(read_u32_as_usize(vaga) + 0x2c, 8);
        aceita(this, read_u32_as_usize(vaga));
        if r == 0 {
            return;
        }
        recusa_mudar(this, read_u32_as_usize(vaga), 0);
        write_u8(read_u32_as_usize(vaga) + 0x2c, 0xf);
        write_u8(read_u32_as_usize(vaga) + 0x2e, 3);
        clear_offer_bid_markers(this, read_u32_as_usize(vaga));
        schedule_offer_days(this, read_u32_as_usize(vaga), 0xa);
        let o3 = read_u32_as_usize(vaga);
        if fechada(o3) == 0 {
            return;
        }
        let l = read_u32_as_usize(o3);
        if l != 0 {
            write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
            loop {
                let p = transfer_manager_llist_next(read_u32_as_usize(o3));
                if p == 0 {
                    break;
                }
                if fechada(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(read_u32_as_usize(p).wrapping_mul(4)))) == 0 {
                    return;
                }
            }
        }
        let m = read_u32_as_usize(this + 0x86e).wrapping_add(read_u32_as_usize(o3 + 0xc).wrapping_mul(8));
        if read_u8(m) & 1 != 0 {
            write_u8(m, read_u8(m) & 0xfe);
        }
        return;
    }
    if matches!(s, 2 | 3 | 4 | 0xf | 0x10 | 0x14) {
        if s == 0xf && read_u8(o + 0x2e) < 3 {
            write_u8(o + 0x2e, 3);
            schedule_offer_days(this, o, 0xa);
        } else {
            tirar(this, o);
        }
        return;
    }
    if s == 1 {
        if transfer_manager_is_human_club(comprador) {
            write_u8(o + 0x2c, 4);
            clear_offer_bid_markers(this, o);
            schedule_offer_days(this, o, 0xa);
        } else if read_u8(read_u32_as_usize(vaga) + 0x1f) == 1 {
            ia_1a(this, vaga);
        } else {
            ia_1b(this, vaga, 0);
        }
        return;
    }
    if s == 5 || s == 6 {
        if transfer_manager_is_human_club(comprador) {
            write_u8(o + 0x2c, 9);
            schedule_offer_days(this, o, 0xa);
        } else {
            crate::comparacao::ramo("cqd_1 contrato ao jogador");
            contrato(this, comprador, transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc)), o); // cqd1: o contrato ao alvo
        }
        return;
    }
    if read_u32_as_usize(o + 4) != 0 && s == 7 {
        if transfer_manager_is_human_club(vendedor) {
            write_u8(o + 0x2c, 8);
            schedule_offer_days(this, o, 0xa);
            return;
        }
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_COPY_FROM);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let mut l2 = [0u8; 0x10];
        let l = l2.as_mut_ptr() as usize;
        lista_nova(l);
        copiar(l, read_u32_as_usize(o + 4));
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        loop {
            let p = transfer_manager_llist_next(l);
            if p == 0 {
                break;
            }
            contrato(this, vendedor, transfer_manager_staff_by_id(read_u32_as_usize(p)), o);
            let s2 = read_u8(o + 0x2c);
            if matches!(s2, 2 | 3 | 4 | 0x10 | 0x14) || (s2 == 0xf && read_u8(o + 0x2e) >= 3) {
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
    write_u8(o + 0x2c, 0);
    if read_u8(o + 0x10) != 0 {
        pass_ball(this, o);
        return;
    }
    let clube = contract_club_filtered(transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc)), false);
    if human_manager_record_checked(clube) != 0 {
        noticia_feita(this, o);
        schedule_offer_days(this, o, 0x1c);
        return;
    }
    let v = valor(o, 0, 0, 0, 0, 1);
    let r = prever(vendedor, transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc)), v as i64, comprador, -1, 0);
    if r == 0 {
        aceitar(this, vaga, 0);
    } else if (r as i8) > 0 && (r as i8) <= 2 {
        recusar(this, vaga, 0, 0);
    }
}

/// Comparador da 0x8c63a0: só a original (cada ramo muda o status, reagenda, aloca notícias, conclui ou tira a oferta).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_offer(this: usize, vaga: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PROCESS_DUE_OFFER)(this, vaga) // ponytail: só a original
}

/// checks completion date, board veto, budget (1.1 times, plus the shortlist offers), squad room and the nation rule; 0
/// allows, 1 or 2 block
/// Original: `TRANSFER_MANAGER::check_offer_restrictions` (0x8cfc90, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8cfc90 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH: a data e a lista locais, a lista construída pela
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
pub(crate) unsafe extern "thiscall" fn check_offer_restrictions(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, usize) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let veto: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_BOARD_VETO);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let orcamento: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let apply_attribute_step: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_CASH_STEP_UP);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_SQUAD_LIMITS);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    let comprador = transfer_manager_club_by_id(read_u32_as_usize(o + 8));
    let alvo = transfer_manager_staff_by_id(read_u32_as_usize(o + 0xc));
    let vendedor = contract_club_filtered(alvo, true);
    if read_u8(o + 0x30) & 0xf != 0 && read_u8(o + 0x2c) != 0x13 {
        let r = conclusao(this, o, pd);
        if !motivo.is_null() {
            *motivo = r;
        }
        if r != 0 {
            return 2;
        }
        let (ha, da) = (read_i16(G_CURRENT_DATE_YEAR) as u16, read_i16(pd + 2) as u16);
        if ha < da || (ha == da && read_i16(G_CURRENT_DATE) < read_i16(pd)) {
            return 0;
        }
    }
    if sem_elenco == 0 && read_u32_as_usize(alvo + 0x61) != 0 && tipo(read_u8(o + 0x11) as i8) != 0 {
        let mut vetar = false;
        if decide == vendedor && read_u8(o + 0x2f) == 0 && deve_aceitar(o, 0, 0) == 0 {
            vetar = true;
        } else if decide == comprador {
            vetar = true;
        }
        if vetar && veto(this, o, comprador, vendedor, alvo, decide, motivo) != 0 {
            return 2;
        }
    }
    let mut b = orcamento(financas(read_u32_as_usize(G_FINANCE_TABLE), comprador), alvo, 0, 0, 0, 0);
    if b < 0 {
        b = 0;
    }
    let mut dinheiro = b as u32;
    let v = apply_attribute_step(&mut dinheiro as *mut u32 as usize, 4) as i64;
    let t = total(o, 0, 1, 1, 0);
    if decide == comprador {
        crate::comparacao::ramo("cqd_1 orçamento");
        if t as f64 > v as f64 * 1.1 { // cqd1: acima do orçamento
            if !motivo.is_null() {
                *motivo = 3;
            }
            if transfer_manager_is_human_club(comprador) {
                return 1;
            }
            let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            let entradas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_CLUB_SHORTLIST_ENTRIES);
            let oferta_do_clube: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_CLUB_OFFER_FOR_PLAYER);
            let mut l = [0u8; 0x10];
            let pl = l.as_mut_ptr() as usize;
            lista_nova(pl);
            entradas(read_u32_as_usize(this + 0x856), comprador, pl);
            write_u32_from_usize(pl + 0xa, read_u32_as_usize(pl + 2));
            let mut soma = v;
            loop {
                let p = transfer_manager_llist_next(pl);
                if p == 0 {
                    break;
                }
                let x = oferta_do_clube(this, transfer_manager_staff_by_id(read_u32_as_usize(p)), comprador, 0);
                if x != 0 {
                    soma = soma.wrapping_add(read_u32_as_usize(x + 0x1b) as i32 as i64);
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
    if read_u32_as_usize(o + 4) == 0 && tipo(read_u8(o + 0x11) as i8) == 0 {
        return 0;
    }
    for k in 0..0x1c {
        let r = read_u32_as_usize(this + 0x8ac + k * 4);
        if read_u32_as_usize(r + 0xa) == read_u32_as_usize(decide + 0x53) {
            if r == 0 {
                return 0;
            }
            let regra: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32 =
                core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0xc));
            if regra(r, o, decide, 0xae2c90, motivo) != 0 {
                return 0;
            }
            if *motivo == 9 && read_u8(o + 0x30) & 0xf == 5 {
                return 0;
            }
            return 2;
        }
    }
    0
}

/// Comparador da 0x8cfc90: só a original (a vaga no elenco, 0x8cd450, pode liberar o jogador mais barato, e a lista de
/// observação aloca nós).
pub(crate) unsafe extern "thiscall" fn comparar_check_offer_restrictions(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u8>(IDX_CHECK_OFFER_RESTRICTIONS)(this, o, decide, motivo, sem_elenco) // ponytail: só a original
}

// ---------- 264a leva: a parte dos clubes anteriores na transferência, o percentual da próxima venda pago e a notícia dele (transfer_manager.cpp) ----------







/// Contract record -> club record (id at +4); 0 for no contract, type not checked.
///
/// [pt] O clube do contrato `c` (+4 = id; 0 sem contrato), sem olhar o tipo.
pub(crate) unsafe fn transfer_manager_club_of_contract(c: usize) -> usize {
    if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) }
}


/// Calls the finance routine `rotina` (sales 0x599800 or purchases 0x599890) on finances `fin` with the value
/// sign-extended to 64 bits.
///
/// [pt] O valor (i32 estendido para 64 bits, como o `cdq` da original) entra nas vendas (0x599800) ou sai pelas compras
/// (0x599890) das finanças `fin` (thiscall(fin, baixo, alto), ret 8).
pub(crate) unsafe fn post_finance_amount(rotina: usize, fin: usize, v: i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
    f(fin, v as u32, (v >> 31) as u32);
}

/// Constructs a local news item of the given type (NEWSITEM::NEWSITEM).
///
/// [pt] A notícia local: construir (0x77bbe0: thiscall(notícia, tipo, 0), ret 8), gravar um dado (0x785790:
/// thiscall(notícia, k, v), ret 8) e entregar ao técnico do clube (0x7861e0: thiscall([0xb7431c], notícia, clube),
/// ret 8).
pub(crate) unsafe fn transfer_manager_construct_news_item(n: usize, tipo: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    f(n, tipo, 0);
}


/// Sends a news item to the club's manager (G_NEWS send_to_club_manager).
pub(crate) unsafe fn send_news_to_club_manager(n: usize, clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    f(read_usize(G_NEWS), n, clube);
}

/// O ensaio dos comparadores: a cópia roda sem as notícias e sem os totais do clube (0x448ad0), que alocam ou entregam
/// e não podem rodar duas vezes; CQB6_ENTREGAS conta as entregas que a original faria (elas podem mexer em [0xae1080]
/// pela resposta automática do humano de férias).
pub(crate) static mut CQB6_ENSAIO: bool = false;
pub(crate) static mut CQB6_ENTREGAS: u32 = 0;

/// player over 22: (offer fee / 20) / sum of weights × weight of each of up to five former clubs goes into the buyer's
/// sales; news 0xfc3 goes to the former club when the cut exceeds 11 × reputation − 10000
/// Original: `TRANSFER_MANAGER::pay_former_club_fee_cuts` (0x8e4f20, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8e4f20 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::pay_former_club_fee_cuts(oferta)`, pela transferência efetivada com valor (+0x1b) de jogador sem
/// +0x69 (0x8c9390). Jogador (+0xc da oferta) com mais de 22 anos (+0x18): a notícia 0xfc3 (local; 0 = id, 1..3 = os
/// +0x33 de +4/+8/+0xc, 4 = o comprador +8, 7 = o clube do contrato ou −1); a soma (16 bits) dos pesos (byte +0x12)
/// das cinco vagas de clube anterior do registro de 79 bytes ([0xae2d0c], passo 5, clube em +0xe, −1 = vaga vazia;
/// soma 0 vira 1). Cada vaga com clube: ((valor / 20) / soma) × peso (peso abaixo de 1 vale 1) entra nas vendas das
/// finanças do COMPRADOR (0x5a0590 com +8, 0x599800; é o que a original faz); acima de 11 × reputação (+0x80) − 10000
/// do clube da vaga, a notícia com 5 = o valor e 6 = o id do clube vai ao técnico dele.
pub(crate) unsafe extern "thiscall" fn pay_former_club_fee_cuts(_this: usize, o: usize) {
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    if (read_u8(p + 0x18) as i8) < 0x17 { // cqb6: só acima de 22 anos
        return;
    }
    crate::comparacao::ramo("cqb_6 parte dos clubes anteriores");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    if !CQB6_ENSAIO {
        transfer_manager_construct_news_item(n, 0xfc3);
    }
    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f));
    let mut soma: i16 = 0;
    for k in 0..5 {
        if read_usize(r + 0xe + 5 * k) as i32 != -1 {
            soma = soma.wrapping_add(read_u8(r + 0x12 + 5 * k) as i8 as i16);
        }
    }
    let vinte = (read_usize(o + 0x1b) as i32) / 20;
    let divisor = if soma == 0 { 1 } else { soma as i32 };
    if !CQB6_ENSAIO {
        set_news_slot(n, 0, read_usize(p) as u32);
        set_news_slot(n, 1, read_usize(read_usize(p + 4) + 0x33) as u32);
        set_news_slot(n, 2, read_usize(read_usize(p + 8) + 0x33) as u32);
        set_news_slot(n, 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
        set_news_slot(n, 4, read_usize(o + 8) as u32);
    }
    let c = injury_main_contract_stored(p);
    let v7 = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 || transfer_manager_club_of_contract(c) == 0 {
        u32::MAX
    } else {
        let c2 = injury_main_contract_stored(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
        if c2 != 0 && read_u8(c2 + 0x35) & 0x3f != 5 { read_usize(transfer_manager_club_of_contract(c2)) as u32 } else { u32::MAX }
    };
    if !CQB6_ENSAIO {
        set_news_slot(n, 7, v7);
    }
    for k in 0..5 {
        let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 5 * k;
        let i = read_usize(r + 0xe);
        if i as i32 == -1 {
            continue;
        }
        let peso = read_u8(r + 0x12) as i8;
        let peso = if peso < 1 { 1 } else { peso as i32 };
        let clube = friendly_club_by_id(i);
        let valor = vinte.wrapping_div(divisor).wrapping_mul(peso); // cqb6: a parte da vaga
        let fin = european_cup_club_finance(friendly_club_by_id(read_usize(o + 8)));
        crate::comparacao::ramo("cqb_6 parte paga");
        post_finance_amount(0x599800, fin, valor); // cqb6: entra nas vendas do comprador
        if valor > (read_i16(clube + club::REPUTATION) as i32).wrapping_mul(11).wrapping_sub(10000) {
            CQB6_ENTREGAS += 1;
            if !CQB6_ENSAIO {
                set_news_slot(n, 5, valor as u32);
                set_news_slot(n, 6, read_usize(clube) as u32);
                send_news_to_club_manager(n, clube);
            }
        }
    }
}

/// Comparador da 0x8e4f20: a cópia em ensaio (sem a notícia) e a original a partir do mesmo estado, lado a lado nas
/// finanças do comprador (0x167 bytes) e em [0xae1080]; com entrega da notícia, só as finanças.
pub(crate) unsafe extern "thiscall" fn comparar_pay_former_club_fee_cuts(this: usize, o: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PAY_FORMER_CLUB_FEE_CUTS);
    let fin = european_cup_club_finance(friendly_club_by_id(read_usize(o + 8)));
    CQB6_ENTREGAS = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(fin, 0x167), (0xae1080, 4)], || original(this, o), || {
        CQB6_ENSAIO = true;
        crate::aleatorio::sem_contar(|| pay_former_club_fee_cuts(this, o));
        CQB6_ENSAIO = false;
    });
    let entregas = CQB6_ENTREGAS;
    let igual = igual || (entregas != 0 && mo.0[0] == mr.0[0]);
    PLACAR.registrar(igual, || format!("pay_former_club_fee_cuts(oferta {:#x}) entregas={} original={:x?} rust={:x?}", o, entregas, mo, mr));
}

/// binary-searches the person's clause with the contract club; the cut (percentage × offer fee plus exchange players,
/// or the person's value) from 1000 up: news, transfer totals, paid into the clause club and out of the contract club
/// Original: `TRANSFER_MANAGER::pay_next_sale_percentage` (0x8cb0f0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cb0f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::pay_next_sale_percentage(pessoa, oferta)`,
/// pela transferência efetivada com a marca 8 da pessoa (0x8c91e0). Com contrato (e clube do contrato), procura pela
/// busca binária (0x946464, entradas de 0xd bytes em [this+0x880], conta [this+0x888], comparação 0x8e54d0) a
/// cláusula com +1 = id do clube do contrato e +9 = id da pessoa. Achada: a base é o valor da oferta (+0x1b) mais o
/// valor (+0x52) de cada jogador da troca (+4, LLIST de ids, cursor recomeçado) quando a pessoa é a da oferta (+0xc);
/// senão o valor da pessoa para o clube do contrato (0x4d0c60 com 0, −1, 0). A parte = percentual (byte +0, com sinal)
/// × base / 100; de 1000 para cima: a notícia (0x8db3a0, com 1 se era a da oferta), os totais de transferência do
/// clube (0x448ad0 em [0xad9dac]: a parte, o clube do contrato, o clube da cláusula +5), a parte entra nas vendas do
/// clube da cláusula e sai pelas compras do clube do contrato. A cláusula fica (quem a tira é a 0x8cb4f0).
pub(crate) unsafe extern "thiscall" fn pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let c = injury_main_contract_stored(p);
    if c == 0 || transfer_manager_club_of_contract(c) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, read_usize(p));
    let c2 = injury_main_contract_stored(p);
    // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
    let id_clube = if c2 != 0 { read_usize(transfer_manager_club_of_contract(c2)) } else { usize::MAX };
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, id_clube);
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = busca(chave.as_ptr(), read_usize(this + 0x880), read_usize(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return 0;
    }
    let (base, da_oferta) = if read_usize(p) == read_usize(o + 0xc) {
        let mut v = read_usize(o + 0x1b) as i32;
        let l = read_usize(o + 4);
        if l != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
            core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
            let mut i = proximo(read_usize(o + 4));
            while i != 0 {
                v = v.wrapping_add(read_usize(staff_records_staff_by_id(read_usize(i)) + 0x52) as i32);
                i = proximo(read_usize(o + 4));
            }
        }
        (v, 1u32)
    } else {
        let clube = transfer_manager_club_of_contract(injury_main_contract_stored(p));
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
        (valor(read_usize(G_CONTRACT_MANAGER), p, clube, 0, -1, 0), 0)
    };
    let parte = (read_u8(e) as i8 as i32).wrapping_mul(base) / 100;
    if parte < 1000 {
        return 0;
    }
    crate::comparacao::ramo("cqb_6 write_percent_pair pago");
    if !CQB6_ENSAIO {
        let noticia: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_SALE_PERCENTAGE_NEWS);
        noticia(this, e, parte, o, da_oferta);
    }
    let da_clausula = friendly_club_by_id(read_usize(e + 5));
    let fin_recebe = european_cup_club_finance(da_clausula);
    let fin_paga = european_cup_club_finance(transfer_manager_club_of_contract(injury_main_contract_stored(p)));
    let do_contrato = transfer_manager_club_of_contract(injury_main_contract_stored(p));
    if !CQB6_ENSAIO {
        let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_ADD_TRANSFER_TOTALS);
        totais(read_usize(G_CLUB_RECORDS), parte, do_contrato, friendly_club_by_id(read_usize(e + 5)));
    }
    post_finance_amount(0x599800, fin_recebe, parte);
    post_finance_amount(0x599890, fin_paga, parte);
    1
}

/// For the comparer: finds the person's next-sale clause (bsearch in [this+0x880]) and returns the finances of the
/// clause club and of the contract club; None if no clause.
///
/// [pt] O que a 0x8cb0f0 vai mexer, para as regiões do comparador: as finanças do clube da cláusula e do clube do
/// contrato, se a cláusula existe (sem gravar nada além de [0xae1080], que a original regrava).
pub(crate) unsafe fn clause_finance_regions(this: usize, p: usize) -> Option<(usize, usize)> {
    let c = injury_main_contract_stored(p);
    if c == 0 || transfer_manager_club_of_contract(c) == 0 {
        return None;
    }
    let mut chave = [0u8; 0xd];
    core::ptr::write_unaligned(chave.as_mut_ptr().add(9) as *mut usize, read_usize(p));
    core::ptr::write_unaligned(chave.as_mut_ptr().add(1) as *mut usize, read_usize(transfer_manager_club_of_contract(c)));
    let busca: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = busca(chave.as_ptr(), read_usize(this + 0x880), read_usize(this + 0x888), 0xd, 0x8e54d0);
    if e == 0 {
        return None;
    }
    Some((european_cup_club_finance(friendly_club_by_id(read_usize(e + 5))), european_cup_club_finance(transfer_manager_club_of_contract(c))))
}

/// Comparador da 0x8cb0f0: a cópia em ensaio (sem a notícia e sem os totais do clube) e a original a partir do mesmo
/// estado, lado a lado nas finanças dos dois clubes, em [0xae1080] e no sorteio (a 0x4d0c60 pode sortear); pago (a
/// notícia e os totais da original podem regravar [0xae1080]), só as finanças e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PAY_NEXT_SALE_PERCENTAGE);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    if let Some((a, b)) = clause_finance_regions(this, p) {
        regioes.push((a, 0x167));
        regioes.push((b, 0x167));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || original(this, p, o), || {
        CQB6_ENSAIO = true;
        let n = crate::aleatorio::sem_contar(|| pay_next_sale_percentage(this, p, o));
        CQB6_ENSAIO = false;
        n
    });
    let igual = r == n && (igual || (r != 0 && mo.0[3..] == mr.0[3..]));
    PLACAR.registrar(igual, || format!("pay_next_sale_percentage(pessoa {:#x}, oferta {:#x}) original={} {:x?} rust={} {:x?}", p, o, r, mo, n, mr));
    r
}

/// news 0xfa7: the person, the percentage, the two clause clubs, the cut and the buyer (or the contract club), to the
/// managers of the clause club and the seller
/// Original: `TRANSFER_MANAGER::post_sale_percentage_news` (0x8db3a0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8db3a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::post_sale_percentage_news(cláusula, parte, oferta, da_oferta)`, pelo percentual pago (0x8cb0f0).
/// A notícia 0xfa7 (local): 0..3 a pessoa da cláusula (+9: id e os +0x33 de +4/+8/+0xc), 4 = o percentual (byte +0),
/// 5 = o clube da cláusula (+5), 6 = o clube vendedor (+1), 7 = a parte, 8 = `da_oferta`, 9 = o comprador (+8 da
/// oferta) com `da_oferta`; senão o id do clube do contrato (0x4cf5a0; sem contrato ou sem clube: acaba sem entregar).
/// Vai aos técnicos do clube da cláusula e do vendedor (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_sale_percentage_news(_this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    crate::comparacao::ramo("cqb_6 noticia do write_percent_pair");
    let p = staff_records_staff_by_id(read_usize(e + 9));
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    transfer_manager_construct_news_item(n, 0xfa7);
    set_news_slot(n, 0, read_usize(p) as u32);
    set_news_slot(n, 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    set_news_slot(n, 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    set_news_slot(n, 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    set_news_slot(n, 4, read_u8(e) as i8 as i32 as u32);
    set_news_slot(n, 5, read_usize(e + 5) as u32);
    set_news_slot(n, 6, read_usize(e + 1) as u32);
    set_news_slot(n, 7, parte as u32);
    set_news_slot(n, 8, da_oferta);
    let v9 = if da_oferta != 0 {
        read_usize(o + 8)
    } else {
        let c = injury_main_contract_stored(p);
        if c == 0 || transfer_manager_club_of_contract(c) == 0 {
            return;
        }
        let c2 = injury_main_contract_stored(p);
        // ponytail: a original lê [0] se o segundo contrato faltasse; inalcançável (o mesmo contrato acabou de passar)
        if c2 != 0 { read_usize(transfer_manager_club_of_contract(c2)) } else { usize::MAX }
    };
    set_news_slot(n, 9, v9 as u32);
    send_news_to_club_manager(n, friendly_club_by_id(read_usize(e + 5)));
    send_news_to_club_manager(n, friendly_club_by_id(read_usize(e + 1)));
}

/// Comparador da 0x8db3a0: só a original (a notícia é entregue e não pode sair duas vezes; a cópia leva o ramo no
/// modo Rust). ponytail: sem plano a conferir além do que a original entrega.
pub(crate) unsafe extern "thiscall" fn comparar_post_sale_percentage_news(this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(IDX_POST_SALE_PERCENTAGE_NEWS)(this, e, parte, o, da_oferta)
}

// ---------- 264a leva: a permissão de trabalho concedida, o estatuto pelo motivo da recusa e a vaga da negociação (transfer_manager.cpp) ----------






/// Date `a` is after date at `d` (Date operator> 0x456740, date passed by value).
///
/// [pt] A data `a` vem depois da data em `d` (0x456740: thiscall(a, d.dword0, d.dword1), ret 8; a data vai por valor).
pub(crate) unsafe fn transfer_manager_date_is_after(a: usize, d: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    f(a, read_usize(d) as u32, read_usize(d + 4) as u32) != 0
}

/// Person's name contains `agulha` (strstr) and the offer's club name equals "Falkirk" (0xa8eb48), the hard-coded
/// exception of the original.
///
/// [pt] O nome (texto C em [pessoa+4]) contém `agulha` (0x945a60, strstr: cdecl(texto, agulha)) e o clube da oferta (+4 =
/// id; nome em +4 do clube) é o de 0xa8eb48 ("Falkirk"; a comparação de bytes que a original faz em linha).
pub(crate) unsafe fn is_falkirk_name_exception(p: usize, agulha: usize, oferta: usize) -> bool {
    let nome = read_usize(p + 4);
    if nome == 0 {
        return false;
    }
    let strstr: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_STRSTR);
    if strstr(nome, agulha) == 0 || oferta == 0 {
        return false;
    }
    let mut a = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(oferta + 4).wrapping_mul(0x245)) + 4;
    let mut b = 0xa8eb48usize;
    loop {
        let (x, y) = (read_u8(a), read_u8(b));
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


/// decides the permit: by the permit record (matches and refusals), non-player by reputation, the 2001 exceptions,
/// potential above 8500, wage above 34999, youth above 190, under 5 caps no, over 20 with a strong nation yes, and the
/// 16-match national team history (over 11 or 70%), which advances with new matches
/// Original: `TRANSFER_MANAGER::is_work_permit_granted` (0x8c7950, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c7950 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::is_work_permit_granted(pessoa, nação,
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
pub(crate) unsafe extern "thiscall" fn is_work_permit_granted(this: usize, p: usize, _nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    let (emprestimo, contrato) = if c != 0 && read_u8(c + 0x35) & 0x3f != 2 { (0, c) } else { (c, 0) };
    let mut reg = registro;
    let tem_registro = if reg != 0 {
        true
    } else if (p == 0 || (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32) && read_u8(staff_flags_record(this, p)) & 0x20 != 0 {
        let chave = read_usize(p) as u32;
        let busca: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        reg = busca(&chave, read_usize(this + 0x93a), read_usize(this + 0x93e), 0xe, 0x8e55c0);
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
            let fora = (emprestimo != 0 && transfer_manager_date_is_after(oferta + 0x2d, emprestimo + 0x2d))
                || (contrato != 0 && read_usize(oferta + 4) != read_usize(contrato + 4));
            if fora && pela_conta {
                return 1;
            }
            if emprestimo != 0 && !transfer_manager_date_is_after(oferta + 0x2d, emprestimo + 0x2d) {
                return 1;
            }
        }
    }
    let np = read_usize(p + 0x69);
    if np != 0 && read_usize(p + 0x61) == 0 {
        return (read_i16(np + 0xc) > 0xa0) as u32;
    }
    if read_i16(G_CURRENT_DATE_YEAR) == 0x7d1 && (is_falkirk_name_exception(p, 0x9b5ac0, oferta) || read_i16(G_CURRENT_DATE_YEAR) == 0x7d1 && is_falkirk_name_exception(p, 0x9b5ab0, oferta)) {
        crate::comparacao::ramo("cqb_7 excecao de 2001");
        return 1;
    }
    let ficha = read_usize(p + 0x61);
    if sorteio != 0 {
        if ficha != 0 && read_i16(ficha + 0xd) as i32 > friendly_random(1000) + 8000 {
            return 1;
        }
    } else if ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o potencial");
        if read_i16(ficha + 0xd) > 0x2134 { // cqb7: o potencial acima de 8500
            return 1;
        }
    }
    if oferta != 0 && read_usize(oferta + 0xc) as i32 > 0x88b7 {
        return 1;
    }
    let idade = read_u8(p + 0x18) as i8;
    if idade < 0x18 && ficha != 0 {
        crate::comparacao::ramo("cqb_7 confere o jovem");
        if read_i16(ficha + 7) > 0xbe { // cqb7: o jovem acima de 190
            return 1;
        }
    }
    let jogos = read_u8(p + 0x22);
    if jogos < 5 {
        return 0;
    }
    if jogos > 0x14 {
        let nacao = read_usize(p + 0x1a);
        if nacao != 0 && read_i16(nacao + nation::REPUTATION) > 0xbb8 && idade < 0x20 {
            return 1;
        }
    }
    // o histórico pelos jogos novos da seleção
    let selecao = read_i16(read_usize(read_usize(G_NATIONAL_TEAMS) + 4).wrapping_add(read_usize(read_usize(p + 0x1a)).wrapping_mul(0x6e)) + 2);
    let novos = selecao.wrapping_sub(read_i16(staff_flags_record(this, p) + 5));
    if novos > 0 {
        for _ in 0..novos {
            let h = staff_flags_record(this, p) + 3;
            core::ptr::write_unaligned(h as *mut u16, core::ptr::read_unaligned(h as *const u16) << 1);
            *(h as *mut u8) &= 0xfe;
        }
    }
    core::ptr::write_unaligned((staff_flags_record(this, p) + 5) as *mut i16, selecao);
    let ano = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let janela: i16 = if (ano as i32) - 2 < 0x7d1 {
        0x10
    } else {
        let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
        let ate = mes(0xae2c90);
        let de = mes(0xae2c90);
        let periodo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16 = core::mem::transmute(FN_FIFA_RANKINGS_COUNT_RESULTS_IN_PERIOD);
        let n = periodo(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x1c), read_usize(p + 0x1a), de, (ano as u32).wrapping_sub(2), ate, ano as u32) as i16;
        if n > 0x10 { 0x10 } else { n }
    };
    let mut marcados: i16 = 0;
    let mut k: i16 = 0;
    while k < janela && k < 0x10 {
        let mascara = 1u16 << k; // ftol(pow(2.0, k)) (0x945ff0 e 0x945640): exato de 0 a 15
        if core::ptr::read_unaligned((staff_flags_record(this, p) + 3) as *const u16) & mascara != 0 {
            marcados += 1;
        }
        k += 1;
    }
    if sorteio != 0 {
        let a = friendly_random(3);
        let limit = 0xb - a + friendly_random(3);
        if marcados as i32 > limit {
            return 1;
        }
        if janela == 0 {
            return 0;
        }
        let b = friendly_random(10);
        let limit = 0x46 - b + friendly_random(10);
        return ((marcados as i32 * 100) / janela as i32 > limit) as u32;
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
pub(crate) unsafe extern "thiscall" fn comparar_is_work_permit_granted(this: usize, p: usize, nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_IS_WORK_PERMIT_GRANTED);
    let mut regioes = vec![crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1]];
    if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 {
        regioes.push((staff_flags_record(this, p), 8));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, p, nacao, oferta, registro, sorteio),
        || crate::aleatorio::sem_contar(|| is_work_permit_granted(this, p, nacao, oferta, registro, sorteio)));
    PLACAR.registrar(igual && r == n, || format!("is_work_permit_granted(pessoa {:#x}, oferta {:#x}, registro {:#x}, sorteio {}) original={} {:x?} rust={} {:x?}", p, oferta, registro, sorteio, r, mo, n, mr));
    r
}

/// the squad status for the refusal reason (0x200: 1, 0x400: 2, 0x2000: 5) or the one of the contract with the club
/// (+0x4f >> 4); no contract, 0
/// Original: `transfer_manager_get_squad_status_for_reason` (0x8c5800, transfer_manager.cpp, thiscall (this sem uso), ret 8).
///
/// [pt] { 0x8c5800 } (thiscall, ret 8, transfer_manager.cpp; o this não é usado): `transfer_manager_get_squad_status_for_reason
/// (pessoa, motivo)`, pela tela da oferta. O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) dá o clube; o
/// contrato com ele (0x4cf5f0); sem ele, 0. O motivo (16 bits com sinal) 0x200 dá 1, 0x400 dá 2, 0x2000 dá 5; os
/// outros, o estatuto do contrato (+0x4f >> 4). Devolve no al.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_squad_status_for_reason(_this: usize, p: usize, motivo: u32) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let cc = com_clube(read_usize(G_CONTRACT_MANAGER), p, injury_club_of_contract(c));
    if cc == 0 {
        return 0;
    }
    match motivo as i16 as i32 {
        0x200 => 1,
        0x400 => 2,
        0x2000 => 5,
        _ => (read_u8(cc + 0x4f) >> 4) as u32,
    }
}

/// Comparador da 0x8c5800: lado a lado em [0xae1080] e no al devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_squad_status_for_reason(this: usize, p: usize, motivo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_TRANSFER_MANAGER_GET_SQUAD_STATUS_FOR_REASON);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, p, motivo), || transfer_manager_get_squad_status_for_reason(this, p, motivo));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("transfer_manager_get_squad_status_for_reason(pessoa {:#x}, motivo {:#x}) original={} {:x?} rust={} {:x?}", p, motivo, r as u8, mo, n as u8, mr));
    r
}

/// the address of slot i of the negotiation array; out of range, the start of the array
/// Original: `TRANSFER_MANAGER::get_negotiation_slot` (0x8c6200, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c6200 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::get_negotiation_slot(i)`, pela resposta
/// automática às notícias (0x782f70) e pelas telas: o endereço da vaga `i` do vetor de negociações ([this+0xc], conta
/// [this+0x10], com sinal); fora da conta, o começo do vetor.
pub(crate) unsafe extern "thiscall" fn get_negotiation_slot(this: usize, i: i32) -> usize {
    if i < read_usize(this + 0x10) as i32 {
        read_usize(this + 0xc).wrapping_add((i as usize).wrapping_mul(4))
    } else {
        read_usize(this + 0xc)
    }
}

/// Comparador da 0x8c6200: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_negotiation_slot(this: usize, i: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_GET_NEGOTIATION_SLOT)(this, i);
    let n = get_negotiation_slot(this, i);
    PLACAR.registrar(r == n, || format!("get_negotiation_slot({}) original={:#x} rust={:#x}", i, r, n));
    r
}

// ---------- 263a leva: a vaga no elenco para a oferta (o limite de 50 jogadores e das vagas da comissão, com a liberação do mais barato), a oferta de empréstimo recusada pelo clube e a cláusula de percentual da próxima venda achada (transfer_manager.cpp) ----------



/// the club rejects the loan offer: state 2, the bids unmarked, the news, the offer comes back in 10 days to be
/// removed, the person's offer mark cleared with no open rival, the contract offers closed and the buyer's shortlist
/// status
/// Original: `TRANSFER_MANAGER::reject_loan_offer` (0x8bfa30, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8bfa30 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_loan_offer(vaga, marcar, motivo)`,
/// pelo vendedor do computador que recusa todo lance de empréstimo (0x8c8090, oferta com +0x1f = 1) e pela oferta
/// agendada (0x8c63a0). Com `marcar`, +0x2d = 1; estado 2 (recusada) já de cara; o contrato da pessoa vai para
/// [0xae1080] e dá o clube do contrato; os lances do comprador pelo jogador e os do clube do contrato pelos da troca
/// saem (0x84aeb0); a notícia (0x8d5170); a oferta volta em 10 dias (0x8c6220) para sair; sem rival em aberto, a marca
/// de oferta da pessoa sai (+0x86e); as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a 0x4ccb30 de cada jogador
/// da troca, cuja marca +4 zera), com +0x10 = 0 entre elas; por fim a situação na lista de observação do comprador
/// (0x849ce0; achada, 0x843e10 com evento 1 e tipo 0). É a recusa da transferência (0x8bf680) sem o desassossego, sem
/// a transferência futura, sem fechar as ofertas de contrato ligadas e sem a parte dos jogadores da troca no fim: os
/// ajudantes `cpx1_*` (com o ensaio CPX1_ENSAIO) servem igual.
pub(crate) unsafe extern "thiscall" fn reject_loan_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    if marcar != 0 {
        *((o + 0x2d) as *mut u8) = 1;
    }
    *((o + 0x2c) as *mut u8) = 2;
    crate::comparacao::ramo("cqa_5 emprestimo recusado");
    let vendedor = contract_club_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
    clear_offer_and_exchange_markers(this, o, vendedor);
    transfer_manager_post_offer_rejected_news(this, o, motivo);
    schedule_offer_unless_dry_run(this, o, 0xa);
    clear_offer_mark_if_closed(this, o);
    // vendedor 0: sem a parte dos jogadores da troca na lista do comprador, que esta rotina não tem
    withdraw_offer_contract_offers(this, o, 0, 0);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    if entrada(shortlist_manager_of(this), comprador, p, (read_u8(o + 0x11) != 0xb) as u32, 0) != 0 {
        update_shortlist_status(this, p, comprador, 1, 0);
    }
}

/// Comparador da 0x8bfa30: o da recusa e da retirada (`compare_reject_withdraw`: a cópia em ensaio e a original do mesmo estado;
/// iguais o estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa).
pub(crate) unsafe extern "thiscall" fn comparar_reject_loan_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_REJECT_LOAN_OFFER);
    compare_reject_withdraw("reject_loan_offer", this, vaga, || o(this, vaga, marcar, motivo), || reject_loan_offer(this, vaga, marcar, motivo))
}

/// O ensaio do comparador da 0x8cd450: a cópia roda sem liberar o jogador escolhido (0x86d800 e 0x8c1af0 mexem no
/// resto do jogo) e guarda quem seria em CQA5_LIBERADO.
pub(crate) static mut CQA5_ENSAIO: bool = false;
pub(crate) static mut CQA5_LIBERADO: usize = 0;

/// Job type byte is a player job (DATABASE is_player_job 0x53b620, types 11..15).
///
/// [pt] Os tipos 0x53b620 (11 a 15: jogador) e 0x53b610 (não 11), cdecl com o byte.
pub(crate) unsafe fn transfer_manager_is_player_job(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    f(t as i8) != 0
}

/// Job type byte is not type 11 (DATABASE 0x53b610).
pub(crate) unsafe fn is_not_plain_player_job(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_NOT_PLAIN_PLAYER_JOB);
    f(t as i8) != 0
}

/// Job type -> index of the club's staff slot counter (0 coaches 8/0xf, 1 scouts 9, 2 physios 0xa, 3 assistant 6/0xd);
/// None otherwise.
///
/// [pt] As quatro contas de vagas da comissão pelo tipo da pessoa: [0] treinadores (8 e 0xf, +0x19f), [1] olheiros (9,
/// +0x1b3), [2] fisioterapeutas (0xa, +0x1cf), [3] auxiliar (6 e 0xd, +0xd3).
pub(crate) unsafe fn staff_role_slot_group(t: u8) -> Option<usize> {
    match t {
        8 | 0xf => Some(0),
        9 => Some(1),
        0xa => Some(2),
        6 | 0xd => Some(3),
        _ => None,
    }
}

/// Person is non-null and its id is below G_STAFF_COUNT (the person still exists).
///
/// [pt] O id da pessoa no vetor (dword com sinal) abaixo de [0xae2364]: a pessoa ainda existe.
pub(crate) unsafe fn staff_id_in_range(p: usize) -> bool {
    p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32
}

/// Writes the failure reason (base + 1 if the club is not the offer's buyer) to `motivo` if given; returns 0.
///
/// [pt] A falha com o motivo: (clube ≠ comprador) + `base` na palavra do motivo, se houver; 0.
pub(crate) unsafe fn fail_with_reason(clube: usize, o: usize, motivo: *mut u16, base: u16) -> u32 {
    if !motivo.is_null() {
        *motivo = (read_usize(clube) != read_usize(o + 8)) as u16 + base;
    }
    0
}

/// Person's id is in the free-transfer queue LLIST (G_TRANSFER_MANAGER + 0x92c), rewinding the cursor first.
///
/// [pt] A pessoa está na fila da transferência livre ([0xde1f54] + 0x92c, LLIST de ids; o cursor +0xa volta a +2 e a busca
/// para no achado, como na original).
pub(crate) unsafe fn in_free_transfer_queue(p: usize) -> bool {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(G_TRANSFER_MANAGER) + 0x92c;
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if read_usize(p) == read_usize(e) {
            return true;
        }
        e = proximo(l);
    }
    false
}

/// checks that the offer keeps the club (with its partner) within 50 players and the coach (5), physio (3), scout (7)
/// and assistant slots; a computer buyer without exchange releases for free the cheapest player who can leave;
/// otherwise reason 4 or 0x22 (plus 1 for the seller) and 0
/// Original: `TRANSFER_MANAGER::check_squad_limits` (0x8cd450, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8cd450 } (thiscall, ret 0x10, transfer_manager.cpp; o mapa a dá como stdcall, mas os dois pontos de chamada na
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
pub(crate) unsafe extern "thiscall" fn check_squad_limits(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    if clube == 0 {
        show_error_at(0xa8d514, 0x2c3b);
        return 0;
    }
    let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let parceiro = parceiro_de(clube, core::ptr::null_mut(), 1);
    let mut n: i8 = 0;
    let mut com = [0i8; 4];
    if read_usize(clube) == read_usize(o + 8) {
        let p = staff_records_staff_by_id(read_usize(o + 0xc));
        let c = transfer_manager_get_contract_and_store(p);
        let mut proprio = false;
        if c != 0 && read_u8(c + 0x35) & 0x3f == 5 {
            proprio = contract_club_stored(p) == clube;
        }
        if !proprio && ficha(read_usize(G_CONTRACT_MANAGER), p) != 0 && read_usize(ficha(read_usize(G_CONTRACT_MANAGER), p) + 4) == read_usize(clube) {
            proprio = true;
        }
        if proprio {
            n = -1;
        }
        let t = read_u8(o + 0x11);
        if transfer_manager_is_player_job(t) {
            n = n.wrapping_add(1); // cqa5: o jogador da oferta conta
        }
        if is_not_plain_player_job(t) {
            if let Some(k) = staff_role_slot_group(t) {
                com[k] = 1;
            }
        }
        let l = read_usize(o + 4);
        if l != 0 {
            for e in llist_items_from_start(l) {
                let t = read_u8(staff_records_staff_by_id(read_usize(e)) + 0x3d);
                if transfer_manager_is_player_job(t) {
                    n = n.wrapping_sub(1);
                }
                if is_not_plain_player_job(t) {
                    if let Some(k) = staff_role_slot_group(t) {
                        com[k] = com[k].wrapping_sub(1);
                    }
                }
            }
        }
    } else {
        if read_usize(o + 4) == 0 {
            crate::comparacao::ramo("cqa_5 vendedor sem troca");
            return 1; // cqa5: o vendedor sem troca tem vaga
        }
        let t = read_u8(staff_records_staff_by_id(read_usize(o + 0xc)) + 0x3d);
        if transfer_manager_is_player_job(t) {
            n = -1;
        }
        if is_not_plain_player_job(t) {
            if let Some(k) = staff_role_slot_group(t) {
                com[k] = -1;
            }
        }
        for e in llist_items_from_start(read_usize(o + 4)) {
            let t = read_u8(e + 6);
            if transfer_manager_is_player_job(t) {
                n = n.wrapping_add(1);
            }
            if is_not_plain_player_job(t) {
                if let Some(k) = staff_role_slot_group(t) {
                    com[k] = com[k].wrapping_add(1);
                }
            }
        }
    }
    if n > 0 {
        let contar = |base: usize, n: &mut i8| {
            for k in 0..0x32 {
                if staff_id_in_range(read_usize(base + 0xd7 + 4 * k)) {
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
                let fila: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COUNT_CLUB_FREE_TRANSFER_QUEUE);
                n = n.wrapping_sub(fila(this, clube) as i8);
            }
            if n > 0x32 {
                let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
                if read_usize(clube) != read_usize(o + 8) || humano(clube, 1, 1, 0, 0) != 0 || read_usize(o + 4) != 0 {
                    crate::comparacao::ramo("cqa_5 elenco cheio");
                    return fail_with_reason(clube, o, motivo, 4);
                }
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
                let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
                let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
                let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
                let mut melhor = 1000000.0f64;
                let mut escolhido = 0usize;
                for k in 0..0x32 {
                    let p = read_usize(clube + club::SQUAD + 4 * k);
                    if !staff_id_in_range(p) || ficha(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                        continue;
                    }
                    // a primeira consulta do contrato não vai para [0xae1080]
                    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
                    if c == 0 {
                        continue;
                    }
                    let s = read_u8(c + 0x3d) as i8;
                    let v = (s as f64 + 100.0) * (read_usize(p + 0x52) as i32 as f64);
                    if !(v < melhor) || s > 0 {
                        continue;
                    }
                    let c = transfer_manager_get_contract_and_store(p);
                    if c != 0 && read_u8(c + 0x35) & 0x3f == 5 {
                        continue;
                    }
                    if ficha(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                        continue;
                    }
                    let g = read_usize(G_TRANSFER_MANAGER);
                    if ofertas(g, p, 0, 0) != 0 || trocas(g, p, 0) != 0 || futura(g, p, 0) != 0 {
                        continue;
                    }
                    if in_free_transfer_queue(p) {
                        continue;
                    }
                    if matches!(read_u8(p + 0x3d), 0xb | 0xf | 0xd) {
                        escolhido = p;
                        melhor = v;
                    }
                }
                if escolhido == 0 {
                    return fail_with_reason(clube, o, motivo, 4);
                }
                crate::comparacao::ramo("cqa_5 libera o mais barato");
                if CQA5_ENSAIO {
                    CQA5_LIBERADO = escolhido;
                } else {
                    let registrar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FREE_RELEASE_RECORDS);
                    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_FREE_TRANSFER_STAFF);
                    registrar(read_usize(G_STAFF_RECORDS), escolhido);
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
            if read_usize(clube + base + 4 * j) != 0 {
                m = m.wrapping_add(1);
            }
        }
        if parceiro != 0 {
            for j in 0..vagas {
                if staff_id_in_range(read_usize(parceiro + base + 4 * j)) {
                    m = m.wrapping_add(1);
                }
            }
        }
        if m > vagas as i8 {
            return fail_with_reason(clube, o, motivo, 0x22);
        }
    }
    if com[3] > 0 && (read_usize(clube + club::ASSISTANT_MANAGER) != 0 || (parceiro != 0 && read_usize(parceiro + 0xd3) != 0)) {
        return fail_with_reason(clube, o, motivo, 0x22);
    }
    1
}

/// Comparador da 0x8cd450: a cópia em ensaio (sem liberar) e depois a original, do mesmo estado ([0xae1080], o motivo,
/// os cursores da fila livre e da troca). Sem liberação: iguais a devolução e essas regiões. Com liberação (a cópia
/// guardou o escolhido): iguais a devolução e o escolhido ter saído do clube (o contrato dele, pela 0x4cf5a0 que só
/// lê, sem o clube); o resto a original refaz ao liberar. Clube nulo: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_check_squad_limits(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32>(IDX_CHECK_SQUAD_LIMITS);
    if clube == 0 {
        return f(this, clube, o, contar_fila, motivo);
    }
    let mut regioes = vec![(0xae1080usize, 4usize), (read_usize(G_TRANSFER_MANAGER) + 0x936, 4)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    if read_usize(o + 4) != 0 {
        regioes.push((read_usize(o + 4) + 0xa, 4));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQA5_ENSAIO = true;
    CQA5_LIBERADO = 0;
    let n = crate::aleatorio::pausar_rastro(|| check_squad_limits(this, clube, o, contar_fila, motivo));
    CQA5_ENSAIO = false;
    let liberado = CQA5_LIBERADO;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = f(this, clube, o, contar_fila, motivo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if liberado == 0 {
        mo.0 == mr.0
    } else {
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contrato(read_usize(G_CONTRACT_MANAGER), liberado);
        c == 0 || friendly_club_by_id(read_usize(c + 4)) != clube
    };
    PLACAR.registrar(igual && r == n, || {
        format!("check_squad_limits(clube {:x}, oferta {:x}, fila {}) original={} rust={} liberado={:x} {:x?} x {:x?}", clube, o, contar_fila, r, n, liberado, mo, mr)
    });
    r
}

/// with the person's flag 8, binary-searches the next-sale percentage clause of the person with the contract club; when
/// not found, clears the flag
/// Original: `TRANSFER_MANAGER::find_next_sale_percentage` (0x8d0dd0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0dd0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_sale_percentage(pessoa)`, das
/// telas da pessoa (0x884ea2): sem a marca 8 da pessoa ([this+0x86e] + id × 8), 0. O contrato dela (0x4cf5a0, em
/// [0xae1080]) e o clube; sem os dois, 0. A busca binária (0x946464: cdecl(chave, vetor [this+0x880], conta
/// [this+0x888], 0xd, 0x8e54d0)) pela chave com o id do clube (+1, de novo pelo contrato) e o da pessoa (+9), os dois
/// únicos campos que a comparação lê; achada, devolve o registro; senão a marca 8 sai e 0.
pub(crate) unsafe extern "thiscall" fn find_next_sale_percentage(this: usize, p: usize) -> usize {
    let m = (read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8))) as *mut u8;
    if *m & 8 == 0 {
        return 0;
    }
    if contract_club_stored(p) == 0 {
        return 0;
    }
    let mut chave = [0u8; 0x10];
    chave[9..13].copy_from_slice(&(read_usize(p) as u32).to_le_bytes());
    // ponytail: a original lê o id do clube sem conferir o contrato (acabou de achá-lo); sem ele, aqui 0
    let c = contract_club_stored(p);
    let id = if c != 0 { read_usize(c) as u32 } else { 0 };
    chave[1..5].copy_from_slice(&id.to_le_bytes());
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let r = bsearch(chave.as_ptr(), read_usize(this + 0x880), read_usize(this + 0x888), 0xd, 0x8e54d0);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(8);
    0
}

/// Comparador da 0x8d0dd0: lado a lado na marca da pessoa, em [0xae1080] e na devolução (só lê o resto).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_sale_percentage(this: usize, p: usize) -> usize {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_NEXT_SALE_PERCENTAGE);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1), (0xae1080, 4)], || f(this, p), || find_next_sale_percentage(this, p));
    PLACAR.registrar(igual && r == n, || format!("find_next_sale_percentage(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

// ---------- 261a leva: os jogos e as faltas das permissões de trabalho e os prêmios por jogos depois da partida, o prêmio por jogos na seleção guardado e a notícia da transferência finalizada (transfer_manager.cpp) ----------






/// Address of the person's 8-byte flags record in the transfer manager ([this+0x86e] + id x 8).
///
/// [pt] O byte das marcas da pessoa no registro de 8 bytes da gerência de transferências ([this+0x86e] + id × 8).
pub(crate) unsafe fn staff_flags_record(this: usize, p: usize) -> usize {
    read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8))
}

/// Person's work-permit record (0xe bytes, sorted at [this+0x93a], count +0x93e) by bsearch; 0 if id out of range or
/// flag 0x20 not set.
///
/// [pt] A permissão de trabalho da pessoa (0xe bytes, [this+0x93a], conta +0x93e), como a original procura em linha: id fora
/// dos [0xae2364] (com sinal) ou sem a marca 0x20, 0; senão a busca binária (bsearch 0x946464 com 0x8e55c0).
pub(crate) unsafe fn transfer_manager_find_work_permit(this: usize, p: usize) -> usize {
    if p != 0 && read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    if read_u8(staff_flags_record(this, p)) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let chave = [read_usize(p) as u32];
    bsearch(chave.as_ptr(), read_usize(this + 0x93a), read_usize(this + 0x93e), 0xe, 0x8e55c0)
}

/// Adds 1 to the u16 at offset `o` of a work-permit record (+4 games, +6 misses); nothing if the record is null.
///
/// [pt] Soma 1 à palavra `o` da permissão (+4 jogos, +6 faltas). A original soma sem conferir a busca (a marca garante a
/// permissão); sem ela, a original cairia: aqui nada.
pub(crate) unsafe fn increment_permit_counter(r: usize, o: usize) {
    if r != 0 {
        let a = (r + o) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
    }
}

/// O ensaio do comparador: a cópia roda sem pagar os prêmios por jogos (0x8cb340 dá notícia, aloca e paga), como se
/// nenhum pagasse (sem tirar o bônus); a contagem das permissões não depende deles.
pub(crate) static mut CPY11_ENSAIO: bool = false;

/// TRANSFER_MANAGER::pay_appearance_money(person, games, type); returns 0 without calling during the comparer's dry
/// run.
///
/// [pt] TRANSFER_MANAGER::pay_appearance_money (0x8cb340: thiscall(this, pessoa, jogos, tipo, 0), ret 0x10); no ensaio, 0.
pub(crate) unsafe fn pay_appearance_money_unless_dry_run(this: usize, p: usize, jogos: i32, tipo: u32) -> u32 {
    if CPY11_ENSAIO {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_APPEARANCE_MONEY);
    f(this, p, jogos, tipo, 0)
}

/// after the match, on both sides: pays the appearance money at 1, 10, 20... 50 games (club by contract, national team
/// by caps since the deal) and removes the paid bonus; in a counting match, adds an appearance to the work permit of
/// who played and an absence to those left out without injury, ban, call-up or loan
/// Original: `update_appearance_clauses_after_match` (0x8c39f0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c39f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::update_appearance_clauses_after_match(jogo,
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
pub(crate) unsafe extern "thiscall" fn update_appearance_clauses_after_match(this: usize, jogo: usize, partida: usize) {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    let tirar_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FIRST_TEAM_APPEARANCE_MONEY);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    // ponytail: o elenco local da original (esp+0x110) não é preenchido no lado da seleção e fica com o que a pilha tinha
    // (o do outro lado ou lixo); aqui começa zerado (vaga vazia, que o jogo também produz) e passa de um lado ao outro
    let mut elenco = [0usize; 50];
    let mut lado: i8 = 0;
    while lado < 2 {
        let clube = read_usize(jogo + if lado == 0 { 0x1c } else { 0x20 });
        let comp = read_usize(jogo + 0x14);
        let fora = comp == 0 || is_national_team(read_usize(jogo + 0x1c)) != 0 || amistoso(read_usize(G_FRIENDLY), comp) != 0;
        let contando = (!fora && read_u8(jogo + 0x40 + lado as usize) != 0) || read_usize(jogo) as i32 == -2;
        let mascara: u8 = if is_national_team(clube) != 0 {
            4
        } else {
            reunir(clube, elenco.as_mut_ptr(), 0);
            2
        };
        let mut seguidos = [0usize; 50];
        let mut n: i8 = 0;
        if contando {
            for &p in elenco.iter() {
                if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && read_u8(staff_flags_record(this, p)) & 0x20 != 0 {
                    seguidos[n as usize] = p;
                    n += 1;
                }
            }
        }
        let base = partida + lado as usize * 0x625;
        let mut k: i8 = 0;
        while k < 0x14 {
            let v = base + k as usize * 0x49;
            let id = read_usize(v + 0x190) as i32;
            if id != -1 && read_u8(v + 0x1ad) != 0xfb && (k < 0xb || read_u8(v + 0x1c2) != 0xff) {
                let p = staff_records_staff_by_id(id as usize);
                if read_usize(p + 0x61) == 0 {
                    break;
                }
                if (p == 0 || (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32) && !(mascara == 2 && ficha(read_usize(G_CONTRACT_MANAGER), p) != 0) {
                    let reg = staff_flags_record(this, p);
                    if read_u8(reg) & mascara != 0 {
                        let (jogos, tipo): (i16, u8) = if mascara == 4 {
                            ((read_u8(p + 0x22) as u16).wrapping_sub(read_u8(reg + 7) as u16) as i16, 5)
                        } else {
                            let mut j = 0i16;
                            if contrato(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                                j = read_i16(contrato(read_usize(G_CONTRACT_MANAGER), p) + 0x3b);
                            }
                            (j, 2)
                        };
                        // ponytail: a original empurra o dword do local do tipo, com os 3 bytes altos de lixo da pilha;
                        // a 0x8cb340 e a notícia só leem o byte: vai limpo
                        if (jogos as i32 % 10 == 0 && jogos <= 0x32) || jogos == 1 { // cpy11: os jogos do prêmio
                            if pay_appearance_money_unless_dry_run(this, p, jogos as i32, tipo as u32) != 0 {
                                crate::comparacao::ramo("cpy_11 premio pago");
                                if tipo == 5 {
                                    tirar_selecao(this, p);
                                } else {
                                    tirar_time(this, p);
                                }
                            }
                        }
                    }
                    if contando && read_u8(staff_flags_record(this, p)) & 0x20 != 0 {
                        let r = transfer_manager_find_work_permit(this, p);
                        crate::comparacao::ramo("cpy_11 jogo contado");
                        increment_permit_counter(r, 4); // cpy11: o jogo conta na permissão
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
                if p == 0 || lesao(read_usize(G_INJURY), p) != 0 {
                    continue;
                }
                let comp = read_usize(jogo + 0x14);
                if comp != 0 && suspenso(read_usize(G_DISCIPLINE), p, comp, 1, 0) != 0 {
                    continue;
                }
                if convocado(read_usize(G_NATIONAL_TEAMS), p, 0xae2c90, clube, 0, 0) != 0 {
                    continue;
                }
                if read_usize(jogo + 0x14) != 0 && inelegivel(this, p, jogo) != 0 {
                    continue;
                }
                if ficha(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
                    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
                    let dono = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
                    if dono == clube {
                        continue;
                    }
                }
                let r = transfer_manager_find_work_permit(this, p);
                crate::comparacao::ramo("cpy_11 falta contada");
                increment_permit_counter(r, 6); // cpy11: a falta conta na permissão
            }
        }
        lado += 1;
    }
}

/// Comparador: a cópia em ensaio (sem pagar os prêmios) e a original a partir do mesmo estado do vetor das permissões
/// de trabalho; os jogos (+4) e as faltas (+6) têm de bater. Seleção em casa: o elenco local da original começa com o
/// lixo da pilha, só a original roda. Os prêmios se provam pela própria 0x8cb340 (comparada) e pela carreira.
pub(crate) unsafe extern "thiscall" fn comparar_update_appearance_clauses_after_match(this: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_APPEARANCE_CLAUSES_AFTER_MATCH);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let v = read_usize(this + 0x93a);
    let n = read_usize(this + 0x93e) as i32;
    if is_national_team(read_usize(jogo + 0x1c)) != 0 || v == 0 || n <= 0 {
        return o(this, jogo, partida); // ponytail: só a original (lixo da pilha ou sem permissões)
    }
    let regioes = [(v, n as usize * 0xe)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPY11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| update_appearance_clauses_after_match(this, jogo, partida)));
    CPY11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, jogo, partida);
    // a original não realoca o vetor (os prêmios mexem em outros vetores)
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("update_appearance_clauses_after_match(jogo {:x}) permissões diferentes nos bytes {:x?}", jogo, dif)
    });
}

/// appends the international appearance money (13 bytes) to the array, growing by 50, and sorts it by person
/// Original: `add_international_team_appearance_money` (0x8cad90, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cad90 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_international_team_appearance_money(
/// registro)`: acrescenta o prêmio por jogos na seleção (13 bytes copiados de `registro`) ao vetor [this+0x898]
/// (capacidade +0x89c, conta +0x8a0); cheio, cresce de 50 em 50 (sem memória: caixa 0x25af e nada). Reordena pela pessoa
/// (qsort 0x945333 com 0x8b4e00). A gêmea do time principal é a 0x8cac40.
pub(crate) unsafe extern "thiscall" fn add_international_team_appearance_money(this: usize, registro: usize) {
    crate::comparacao::ramo("cpy_11 premio na selecao guardado");
    if !grow_record_vector(this, 0x898, 4, 50, 0xd, 0x25af, 0xa8ebe0) {
        return;
    }
    let n = read_usize(this + 0x8a0);
    core::ptr::copy_nonoverlapping(registro as *const u8, read_usize(this + 0x898).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    core::ptr::write_unaligned((this + 0x8a0) as *mut usize, n.wrapping_add(1));
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0x898), read_usize(this + 0x8a0) as u32, 0xd, 0x8b4e00);
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta e os 13
/// bytes), a original roda; confere a conta e que o registro está no vetor (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_international_team_appearance_money(this: usize, registro: usize) {
    let n = read_usize(this + 0x8a0);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_INTERNATIONAL_TEAM_APPEARANCE_MONEY)(this, registro);
    if read_usize(this + 0x898) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x8a0);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x898), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_international_team_appearance_money: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// the 0xfa5 news of the agreed transfer, with the completion date, to the human seller and buyer
/// Original: `post_transfer_finalised_news` (0x8da5e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8da5e0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da data e da notícia locais): `TRANSFER_MANAGER::
/// post_transfer_finalised_news(oferta)`, pela transferência acertada (0x8c2f50). O vendedor: o clube do contrato da
/// pessoa (0x4cf5a0, guardado em [0xae1080]) fora do tipo 5 (pedido de novo, sem guardar). A notícia 0xfa5: a pessoa (o
/// id e as três chaves), o comprador, o vendedor (ou −1), +0x20, +0x24, o valor +0x1b, o estado +0x2c, o total
/// (0x8e7a70 com 0, 1, 0, 1), +0x2f, +0x11; a data de conclusão (0x8cee70) depois de hoje vai nos campos 0xe (dia), 0xf
/// (ano) e 0xd (o nibble baixo de +0x30, com sinal), senão 0, 0, 0; a conta da troca (+4) no 0xb. Ao vendedor humano
/// (0x7875e0), fora do estado 0xc e sem troca pendente (+0x2f com nibble +0x30), e ao comprador humano, cada um no
/// campo 0x10.
pub(crate) unsafe extern "thiscall" fn post_transfer_finalised_news(this: usize, o: usize) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let mut vendedor = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let mut data = [0u8; 8];
    data_padrao(data.as_mut_ptr());
    if vendedor != 0 && read_u8(contrato(read_usize(G_CONTRACT_MANAGER), p) + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    crate::comparacao::ramo("cpy_11 noticia da transferencia finalizada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa5, 0);
    gravar(nt, 0, read_usize(p) as u32);
    gravar(nt, 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, read_usize(o + 8) as u32);
    gravar(nt, 5, if vendedor != 0 { read_usize(vendedor) as u32 } else { u32::MAX });
    gravar(nt, 6, read_usize(o + 0x20) as u32);
    gravar(nt, 7, read_usize(o + 0x24) as u32);
    gravar(nt, 8, read_usize(o + 0x1b) as u32);
    gravar(nt, 9, read_u8(o + 0x2c) as i8 as u32);
    gravar(nt, 0xa, total(o, 0, 1, 0, 1));
    gravar(nt, 0xc, read_u8(o + 0x2f) as i8 as u32);
    gravar(nt, 0x11, read_u8(o + 0x11) as i8 as u32);
    conclusao(this, o, data.as_mut_ptr());
    let (dia, ano) = (i16::from_le_bytes([data[0], data[1]]), u16::from_le_bytes([data[2], data[3]]));
    let (hoje_dia, hoje_ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    if hoje_ano < ano || (hoje_ano == ano && hoje_dia < dia) {
        gravar(nt, 0xe, dia as i32 as u32);
        gravar(nt, 0xf, ano as u32);
        gravar(nt, 0xd, (((read_u8(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    } else {
        gravar(nt, 0xe, 0);
        gravar(nt, 0xf, 0);
        gravar(nt, 0xd, 0);
    }
    let troca = read_usize(o + 4);
    gravar(nt, 0xb, if troca != 0 { read_u8(troca) as i8 as u32 } else { 0 });
    let jornal = read_usize(G_NEWS);
    if vendedor != 0 && humano(jornal, vendedor) != 0 && read_u8(o + 0x2c) != 0xc && (read_u8(o + 0x2f) == 0 || read_u8(o + 0x30) & 0xf == 0) {
        gravar(nt, 0x10, read_usize(vendedor) as u32);
        entregar(read_usize(G_NEWS), nt, vendedor);
    }
    if humano(read_usize(G_NEWS), comprador) != 0 {
        gravar(nt, 0x10, read_usize(comprador) as u32);
        entregar(read_usize(G_NEWS), nt, comprador);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_finalised_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_TRANSFER_FINALISED_NEWS)(this, o)
}

// ---------- 261a leva: a pessoa liberada sem custo, as férias do técnico humano e a notícia da permissão para discutir o contrato (transfer_manager.cpp, human_manager.cpp) ----------





/// Person -> main contract (get_contract), without writing G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
pub(crate) unsafe fn get_contract_unstored(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    f(read_usize(G_CONTRACT_MANAGER), p)
}

/// Copies today's date (0xae2c90) into `destino` (Date::operator=).
///
/// [pt] Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): a data de hoje ([0xae2c90]) no destino.
pub(crate) unsafe fn copy_today_to(destino: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), 0xae2c90);
}

/// records the free transfer and, unless record only, pays the club's compensation, removes the person from shortlists,
/// selections, squad, offers and contract, clauses and work permit
/// Original: `free_transfer_staff` (0x8c1af0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1af0 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::free_transfer_staff(pessoa, clube, motivo,
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
pub(crate) unsafe extern "thiscall" fn free_transfer_staff(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    if ficha(read_usize(G_CONTRACT_MANAGER), p) != 0 {
        return;
    }
    if !grow_record_vector(this, 0x862, 4, 100, 0x25, 0xce9, 0xa8e998) {
        return;
    }
    let n = read_usize(this + 0x86a);
    let r = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    core::ptr::write_unaligned((this + 0x86a) as *mut usize, n.wrapping_add(1));
    let por = |o: usize, v: u32| core::ptr::write_unaligned((r + o) as *mut u32, v);
    por(0x10, u32::MAX);
    por(0x14, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX });
    copy_today_to(r + 0x1d);
    por(0, read_usize(p) as u32);
    por(4, read_usize(read_usize(p + 4) + 0x33) as u32);
    por(8, read_usize(read_usize(p + 8) + 0x33) as u32);
    por(0xc, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    *((r + 0x1c) as *mut u8) = if so_registro != 0 { 5 } else { 3 };
    por(0x18, 0);
    if so_registro != 0 {
        return;
    }
    crate::comparacao::ramo("cpy_13 liberado");
    if read_usize(p + 0x61) != 0 {
        let historico: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_ARCHIVE_PLAYER_STATS);
        historico(read_usize(G_PLAYER_STATS), p, 0, 0, -1, motivo, 0);
    }
    if clube != 0 {
        let c = get_contract_unstored(p);
        let mut comp = compensacao;
        if comp == -1 {
            comp = if c != 0 && read_u8(c + 0x35) & 0x3f == 2 {
                let dias: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(FN_STAFF_CONTRACT_DAYS_TO_END_DATE);
                (dias(c) as i32).wrapping_mul(read_usize(c + 0xc) as i32) / 7
            } else {
                0
            };
        }
        let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
        let pagar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_WAGES);
        let fin = financas(read_usize(G_FINANCE_TABLE), clube);
        if comp != 0 {
            crate::comparacao::ramo("cpy_13 compensacao paga");
        }
        pagar(fin, comp); // cpy13: a compensação ao clube
        let listas: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        listas(read_usize(this + 0x856), p, clube, 4, 2); // cpy13: a situação nas listas de observação
        if read_usize(p + 0x39) != 0 {
            let escalacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
            let elenco: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
            crate::comparacao::ramo("cpy_13 sai do clube atual");
            escalacao(read_usize(G_TACTICS), read_usize(p + 0x39), p, 1, 1); // cpy13: as escalações
            escalacao(read_usize(G_TACTICS), read_usize(p + 0x39), p, 0, 1);
            elenco(read_usize(p + 0x39), p, 0, 1, 0);
        }
        copy_today_to(p + 0x3e);
    }
    let c = get_contract_unstored(p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c != 0 && friendly_club_by_id(read_usize(c + 4)) != 0 {
        let c = get_contract_unstored(p);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let dono = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
        let tirar_ofertas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
        tirar_ofertas(read_usize(G_CONTRACT_MANAGER), p, dono);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT);
    tirar_contrato(read_usize(G_CONTRACT_MANAGER), p);
    core::ptr::write_unaligned((p + 0x52) as *mut u32, 0);
    *((p + 0x6d) as *mut u8) = 0;
    let write_percent_pair: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_NEXT_SALE_PERCENTAGE);
    let bonus_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FIRST_TEAM_APPEARANCE_MONEY);
    let bonus_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UPDATE_WORK_PERMIT_ON_MOVE);
    let fim: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_CLEAR_AWOL);
    write_percent_pair(this, p);
    bonus_time(this, p);
    bonus_selecao(this, p);
    permissao(this, p, 0);
    fim(read_usize(G_AWOL), p);
}

// ponytail: paga, tira de listas, escalações, elenco e contrato (não pode rodar duas vezes): só a original, conferida
// pelo plano: emprestada, a conta das transferências passadas fica; senão sobe 1 com o registro novo (a pessoa, o
// clube, −1 e o tipo 5 ou 3) e, fora do só registro, sem contrato principal, com +0x52 e +0x6d zerados.
pub(crate) unsafe extern "thiscall" fn comparar_free_transfer_staff(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let emprestada = ficha(read_usize(G_CONTRACT_MANAGER), p) != 0;
    let n = read_usize(this + 0x86a);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32)>(IDX_FREE_TRANSFER_STAFF)(this, p, clube, motivo, compensacao, so_registro);
    if emprestada {
        PLACAR.registrar(read_usize(this + 0x86a) == n, || format!("free_transfer_staff(pessoa {}) emprestada: conta {} -> {}", read_usize(p), n, read_usize(this + 0x86a)));
        return;
    }
    if read_usize(this + 0x862) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let r = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let mut ok = read_usize(this + 0x86a) == n.wrapping_add(1)
        && read_usize(r) == read_usize(p)
        && read_usize(r + 0x10) as u32 == u32::MAX
        && read_usize(r + 0x14) as u32 == if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }
        && read_u8(r + 0x1c) == if so_registro != 0 { 5 } else { 3 };
    if so_registro == 0 {
        ok = ok && get_contract_unstored(p) == 0 && read_usize(p + 0x52) == 0 && read_u8(p + 0x6d) == 0;
    }
    PLACAR.registrar(ok, || format!("free_transfer_staff(pessoa {}, clube {:x}, só registro {}): conta {} -> {}, registro {:x?}", read_usize(p), clube, so_registro,
        n, read_usize(this + 0x86a), core::slice::from_raw_parts(r as *const u8, 0x1d)));
}


// ponytail: responde notícias e mexe no interesse do dia (não pode rodar duas vezes): só a original, conferida pelo
// plano: a marca de férias passa a `férias` e a conta muda de ±1 só quando a marca muda.

/// the 0xfac news of permission to discuss a contract to the human buyer and, per exchange player, to the human seller
/// Original: `post_contract_talks_news` (0x8d7cd0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d7cd0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH das notícias locais): `TRANSFER_MANAGER::
/// post_contract_talks_news(oferta)`, pela oferta aceita (0x8c0080). O vendedor: o clube do contrato da pessoa (0x4cf5a0,
/// guardado em [0xae1080]) fora do tipo 5. Comprador humano (0x7875e0): a notícia 0xfac com a pessoa (o id e as três
/// chaves), o vendedor (ou −1), o comprador, +0x20, +0x24, 0 e 0. Vendedor humano com troca (+4): uma 0xfac por jogador
/// da troca, com ele, o comprador, o vendedor, +0x20, +0x24, 0 e 1, ao vendedor.
pub(crate) unsafe extern "thiscall" fn post_contract_talks_news(_this: usize, o: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let c = get_contract_unstored(p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let vendedor = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let chaves = |nt: usize, q: usize| {
        gravar(nt, 0, read_usize(q) as u32);
        gravar(nt, 1, read_usize(read_usize(q + 4) + 0x33) as u32);
        gravar(nt, 2, read_usize(read_usize(q + 8) + 0x33) as u32);
        gravar(nt, 3, read_usize(read_usize(q + 0xc) + 0x33) as u32);
    };
    if humano(read_usize(G_NEWS), comprador) != 0 {
        crate::comparacao::ramo("cpy_13 noticia ao comprador");
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        construir(nt, 0xfac, 0);
        chaves(nt, p);
        gravar(nt, 4, if vendedor != 0 { read_usize(vendedor) as u32 } else { u32::MAX });
        gravar(nt, 5, read_usize(o + 8) as u32);
        gravar(nt, 7, read_usize(o + 0x20) as u32);
        gravar(nt, 8, read_usize(o + 0x24) as u32);
        gravar(nt, 9, 0);
        gravar(nt, 0xa, 0);
        entregar(read_usize(G_NEWS), nt, comprador);
    }
    if vendedor == 0 || humano(read_usize(G_NEWS), vendedor) == 0 || read_usize(o + 4) == 0 {
        return;
    }
    let troca = read_usize(o + 4);
    core::ptr::write_unaligned((troca + 0xa) as *mut usize, read_usize(troca + 2));
    let mut e = proximo(read_usize(o + 4));
    let mut local = [0u32; 0x38]; // a segunda notícia local
    let nt = local.as_mut_ptr() as usize;
    while e != 0 {
        let q = staff_records_staff_by_id(read_usize(e));
        crate::comparacao::ramo("cpy_13 noticia ao vendedor");
        construir(nt, 0xfac, 0);
        chaves(nt, q);
        gravar(nt, 4, read_usize(o + 8) as u32);
        gravar(nt, 5, read_usize(vendedor) as u32);
        gravar(nt, 7, read_usize(o + 0x20) as u32);
        gravar(nt, 8, read_usize(o + 0x24) as u32);
        gravar(nt, 9, 0);
        gravar(nt, 0xa, 1);
        entregar(read_usize(G_NEWS), nt, vendedor);
        e = proximo(read_usize(o + 4));
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_talks_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_CONTRACT_TALKS_NEWS)(this, o)
}

// ---------- 260a leva: a oferta de transferência recusada pelo clube vendedor e a retirada pelo comprador, com a notícia da oferta recusada (transfer_manager.cpp) ----------





/// Person -> main contract (get_contract), written to G_CONTRACT_LOOKUP_RESULT as the original does.
///
/// [pt] O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como
/// a original faz depois de cada chamada.
pub(crate) unsafe fn transfer_manager_get_contract_and_store(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Rewinds a LLIST's cursor (+0xa = +2) and collects all node pointers.
///
/// [pt] Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
pub(crate) unsafe fn llist_items_from_start(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
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
pub(crate) static mut CPX1_ENSAIO: bool = false;

/// As ofertas que o ensaio passou do estado 0xf para o 8 (`dry_run_restore_contract_offers`), devolvidas ao 0xf depois da cópia.
pub(crate) static mut CPX1_DESFAZER: Vec<usize> = Vec::new();

/// Dry run of 0x4cf4a0: for each stored contract offer of the person linked to a transfer offer with the same series, a
/// status-0xf offer goes back to 8 (recorded for undo).
///
/// [pt] No ensaio, o que a 0x4cf4a0 (as ofertas de contrato guardadas da pessoa voltam) muda no que a marca de oferta lê: em
/// cada oferta de contrato guardada da pessoa (lista [[0xae1050]+0x38], +0 o id) ligada a uma oferta de transferência
/// (+0x59 a vaga, ≠ −1; 0x8c61e0 da gerência [0xde1f54]) com a mesma série (+0x5d = +0x24 dela), a oferta no estado
/// 0xf volta ao 8 e deixa de contar como encerrada.
pub(crate) unsafe fn dry_run_restore_contract_offers(id: usize) {
    let vaga: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    for e in llist_items_keep_cursor(read_usize(read_usize(G_CONTRACT_MANAGER) + 0x38)) {
        if read_usize(e) != id || read_usize(e + 0x59) == usize::MAX {
            continue;
        }
        let v = vaga(read_usize(G_TRANSFER_MANAGER), read_usize(e + 0x59));
        if v == 0 || read_usize(v) == 0 {
            continue;
        }
        let r = read_usize(v);
        if read_usize(r + 0x24) == read_usize(e + 0x5d) && read_u8(r + 0x2c) == 0xf {
            *((r + 0x2c) as *mut u8) = 8;
            (*core::ptr::addr_of_mut!(CPX1_DESFAZER)).push(r);
        }
    }
}

/// Transfer manager -> its Shortlist_Manager (+0x856).
///
/// [pt] A gerência da lista de observação (Shortlist_Manager em +0x856 da gerência de transferências).
pub(crate) unsafe fn shortlist_manager_of(this: usize) -> usize {
    read_usize(this + 0x856)
}

/// Shortlist_Manager::clear_bid_markers_for_player(club, person); nothing during the dry run.
///
/// [pt] Shortlist_Manager::clear_bid_markers_for_player (0x84aeb0: thiscall(gerência, clube, pessoa), ret 8); no ensaio,
/// nada.
pub(crate) unsafe fn clear_bid_markers(this: usize, clube: usize, pessoa: usize) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    f(shortlist_manager_of(this), clube, pessoa);
}

/// Shortlist_Manager::update_staff_status(person, club, event, type); nothing during the dry run.
///
/// [pt] Shortlist_Manager::update_staff_status (0x843e10: thiscall(gerência, pessoa, clube, evento, tipo), ret 0x10); no
/// ensaio, nada.
pub(crate) unsafe fn update_shortlist_status(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    f(shortlist_manager_of(this), pessoa, clube, evento, tipo);
}

/// Calls thiscall f(transfer manager, offer) (0x8c1290, 0x8cbf70, 0x8c6db0); in the dry run only emulates 0x8cbf70's
/// clearing of the person's flag bit 0x10.
///
/// [pt] Uma chamada thiscall(gerência de transferências, oferta) com ret 4 (0x8c1290 desassossegar, 0x8cbf70 tirar a
/// transferência futura, 0x8c6db0 fechar as ofertas de contrato da oferta); no ensaio, nada, só o bit 0x10 da marca da
/// pessoa que a 0x8cbf70 apaga (o comparador lê o byte; as marcas dos jogadores da troca ele não lê nem devolve).
pub(crate) unsafe fn call_with_offer(f: usize, this: usize, o: usize) {
    if CPX1_ENSAIO {
        if f == 0x8cbf70 {
            *((read_usize(this + 0x86e).wrapping_add(read_usize(o + 0xc).wrapping_mul(8))) as *mut u8) &= 0xef;
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    f(this, o);
}

/// Offer status (+0x2c) is 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3 (closed for the under-offer mark).
///
/// [pt] O fim comum da recusa e da retirada, a partir do teste da oferta encerrada: sem oferta rival (+0) em aberto, a marca
/// de oferta em andamento (bit 0 do registro de 8 bytes da pessoa em +0x86e) sai.
pub(crate) unsafe fn offer_is_closed(o: usize) -> bool {
    match read_u8(o + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => read_u8(o + 0x2e) >= 3,
        _ => false,
    }
}

/// If the offer and all rival offers in its list (+0) are closed, clears bit 0 of the target's flags record (+0x86e).
pub(crate) unsafe fn clear_offer_mark_if_closed(this: usize, o: usize) {
    if !offer_is_closed(o) {
        return;
    }
    let rivais = read_usize(o);
    if rivais != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((rivais + 0xa) as *mut usize, read_usize(rivais + 2));
        let mut e = proximo(read_usize(o));
        while e != 0 {
            let r = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
            if !offer_is_closed(r) {
                return;
            }
            e = proximo(read_usize(o));
        }
    }
    let m = (read_usize(this + 0x86e).wrapping_add(read_usize(o + 0xc).wrapping_mul(8))) as *mut u8;
    if *m & 1 != 0 {
        *m &= 0xfe;
    }
}

/// Finds the buyer's shortlist entry for the target (0x849ce0) and, if present, updates its status (event 1).
///
/// [pt] O começo da lista de observação do comprador (0x849ce0) e as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a
/// 0x4ccb30 de cada jogador da troca, cuja marca +4 zera), com +0x10 zerado entre elas; e, com troca, vendedor e
/// vendedor do computador (0x600a50), a situação na lista do comprador de cada jogador da troca que estava nela
/// (0x842080, 0x843e10 evento 1).
pub(crate) unsafe fn update_buyer_shortlist_entry(this: usize, o: usize) {
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let e = entrada(shortlist_manager_of(this), friendly_club_by_id(read_usize(o + 8)), staff_records_staff_by_id(read_usize(o + 0xc)), (read_u8(o + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o registrador do ponteiro da entrada com o setne no byte baixo; a 0x843e10 só lê
        // o byte
        let tipo = (e as u32 & !0xff) | (read_u8(e) != 0xb) as u32;
        update_shortlist_status(this, staff_records_staff_by_id(read_usize(o + 0xc)), friendly_club_by_id(read_usize(o + 8)), 1, tipo);
    }
}

/// Removes the contract offers of the target and of each exchange player (clearing +0x10 and each item's +4), and
/// updates the buyer's shortlist status of exchange players when the seller is AI.
pub(crate) unsafe fn withdraw_offer_contract_offers(this: usize, o: usize, vendedor: usize, comprador: usize) {
    if !CPX1_ENSAIO {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
        tirar(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(o + 0xc)), friendly_club_by_id(read_usize(o + 8)));
    }
    *((o + 0x10) as *mut u8) = 0;
    let troca = read_usize(o + 4);
    if troca != 0 {
        let tirar_da_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
        for e in llist_items_from_start(troca) {
            if !CPX1_ENSAIO {
                tirar_da_vaga(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(e)), read_usize(o + 0x20));
            }
            *((e + 4) as *mut u8) = 0;
        }
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if read_usize(o + 4) != 0 && vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0 {
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_IS_ON_SHORTLIST);
        for e in llist_items_from_start(read_usize(o + 4)) {
            if in_list(shortlist_manager_of(this), staff_records_staff_by_id(read_usize(e)), comprador) != 0 {
                update_shortlist_status(this, staff_records_staff_by_id(read_usize(e)), comprador, 1, 0);
            }
        }
    }
}

/// Clears the buyer's bid markers for the target and the seller club's markers for each exchange player.
///
/// [pt] Desmarca os lances do comprador pelo jogador (0x84aeb0) e os do clube do contrato (`vendedor`, ou 0) por cada
/// jogador da troca.
pub(crate) unsafe fn clear_offer_and_exchange_markers(this: usize, o: usize, vendedor: usize) {
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    clear_bid_markers(this, friendly_club_by_id(read_usize(o + 8)), p); // cpx1: desmarcar os lances do comprador
    let troca = read_usize(o + 4);
    if troca != 0 {
        for e in llist_items_from_start(troca) {
            clear_bid_markers(this, vendedor, staff_records_staff_by_id(read_usize(e)));
        }
    }
}

/// Person -> club of their contract (or 0), with the contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato da pessoa (ou 0), com o contrato guardado em [0xae1080].
pub(crate) unsafe fn contract_club_stored(pessoa: usize) -> usize {
    let c = transfer_manager_get_contract_and_store(pessoa);
    if c == 0 {
        0
    } else {
        friendly_club_by_id(read_usize(c + 4))
    }
}

/// TRANSFER_MANAGER post_offer_rejected_news(offer, reason); nothing during the dry run.
///
/// [pt] A notícia da oferta recusada (0x8d5170: thiscall(gerência, oferta, motivo), ret 8); no ensaio, nada.
pub(crate) unsafe fn transfer_manager_post_offer_rejected_news(this: usize, o: usize, motivo: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_REJECTED_NEWS);
    f(this, o, motivo);
}

/// TRANSFER_MANAGER::schedule_offer(offer, days, 0); nothing during the dry run.
///
/// [pt] TRANSFER_MANAGER::schedule_offer (0x8c6220: thiscall(gerência, oferta, dias, fatias), ret 0xc); no ensaio, nada.
pub(crate) unsafe fn schedule_offer_unless_dry_run(this: usize, o: usize, dias: u32) {
    if CPX1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    f(this, o, dias, 0);
}

/// rejects the offer: the player may become unsettled, state 2, bids unmarked, contract offers closed, the news, back
/// in 25 days for removal and the person's offer flag
/// Original: `reject_transfer_offer` (0x8bf680, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8bf680 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_transfer_offer(vaga, marcar,
/// motivo)`, pelo clube vendedor que recusa (0x8c36b0 e a avaliação dos lances). Com `marcar`, +0x2d da oferta = 1; o
/// jogador pode se desassossegar (0x8c1290); transferência futura (estado 0x13): sai da lista (0x8cbf70). Estado 2
/// (recusada); os lances do comprador pelo jogador e os do clube do contrato pelos da troca saem (0x84aeb0); as ofertas
/// de contrato ligadas fecham (0x8c6db0); a notícia (0x8d5170); a oferta volta em 25 dias (0x8c6220) para sair. Sem
/// rival em aberto, a marca de oferta da pessoa sai (+0x86e). A lista de observação do comprador (0x849ce0, 0x843e10),
/// as ofertas de contrato (0x4cca70, 0x4ccb30) e +0x10 = 0; com troca e vendedor do computador, os jogadores da troca
/// na lista do comprador (0x842080, 0x843e10).
pub(crate) unsafe extern "thiscall" fn reject_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = contract_club_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
    let comprador = friendly_club_by_id(read_usize(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    crate::comparacao::ramo("cpx_1 recusa desassossega");
    call_with_offer(0x8c1290, this, o); // cpx1: o desassossego da recusa
    let atual = *vaga;
    if read_u8(atual + 0x2c) == 0x13 {
        call_with_offer(0x8cbf70, this, atual);
    }
    *((o + 0x2c) as *mut u8) = 2;
    let vendedor2 = contract_club_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
    crate::comparacao::ramo("cpx_1 recusa desmarca");
    clear_offer_and_exchange_markers(this, o, vendedor2);
    call_with_offer(0x8c6db0, this, o);
    transfer_manager_post_offer_rejected_news(this, o, motivo);
    schedule_offer_unless_dry_run(this, o, 0x19);
    clear_offer_mark_if_closed(this, o);
    update_buyer_shortlist_entry(this, o);
    withdraw_offer_contract_offers(this, o, vendedor, comprador);
}

/// withdraws the offer: state 3, held contract offers resubmitted, bids unmarked, back in 10 days; for reason 2, the
/// unsettling; the news and the contract offers
/// Original: `withdraw_transfer_offer` (0x8bfcb0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8bfcb0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::withdraw_transfer_offer(vaga, marcar,
/// motivo)`, pelo comprador que desiste (a revisão dos lances e as ofertas desfeitas). Com `marcar`, +0x2d = 1;
/// transferência futura: sai da lista (0x8cbf70); estado de 0xb para cima: as ofertas de contrato guardadas voltam
/// (0x4cf4a0). Estado 3 (retirada); as ofertas de contrato ligadas fecham (0x8c6db0); os lances saem (0x84aeb0); +0x10 =
/// 4; a oferta volta em 10 dias (0x8c6220). Sem rival em aberto, a marca de oferta da pessoa sai. Motivo 2: o jogador
/// pode se desassossegar (0x8c1290). A lista de observação do comprador, a notícia (0x8d5170), as ofertas de contrato e
/// +0x10 = 0; com troca e vendedor do computador, os jogadores da troca na lista do comprador.
pub(crate) unsafe extern "thiscall" fn withdraw_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = *vaga;
    let vendedor = contract_club_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
    let comprador = friendly_club_by_id(read_usize(o + 8));
    if marcar != 0 {
        *((*vaga + 0x2d) as *mut u8) = 1;
    }
    let atual = *vaga;
    if read_u8(atual + 0x2c) == 0x13 {
        call_with_offer(0x8cbf70, this, atual);
    }
    if read_u8(o + 0x2c) as i8 >= 0xb {
        if CPX1_ENSAIO {
            dry_run_restore_contract_offers(read_usize(o + 0xc));
        } else {
            let reenviar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_RESUBMIT_HELD_OFFERS);
            reenviar(read_usize(G_CONTRACT_MANAGER), read_usize(o + 0xc));
        }
    }
    *((o + 0x2c) as *mut u8) = 3;
    call_with_offer(0x8c6db0, this, o);
    let vendedor2 = contract_club_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
    clear_offer_and_exchange_markers(this, o, vendedor2);
    *((o + 0x10) as *mut u8) = 4;
    schedule_offer_unless_dry_run(this, o, 0xa);
    clear_offer_mark_if_closed(this, o);
    if motivo as u16 == 2 { // cpx1: só o motivo 2 desassossega
        call_with_offer(0x8c1290, this, o);
    } else {
        crate::comparacao::ramo("cpx_1 retirada sem desassossego");
    }
    update_buyer_shortlist_entry(this, o);
    transfer_manager_post_offer_rejected_news(read_usize(G_TRANSFER_MANAGER), o, motivo);
    withdraw_offer_contract_offers(this, o, vendedor, comprador);
}

/// All node pointers of a LLIST (empty if null), restoring its cursor afterwards.
///
/// [pt] As vagas de uma LLIST (ou nenhuma), sem mexer no cursor.
pub(crate) unsafe fn llist_items_keep_cursor(l: usize) -> Vec<usize> {
    if l == 0 {
        return Vec::new();
    }
    let cursor = read_usize(l + 0xa);
    let v = llist_items_from_start(l);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    v
}

/// Comparer for offer rejection/withdrawal: runs the Rust copy in dry run and the original from the same state,
/// comparing status, +0x10, exchange items' +4 and the under-offer mark.
///
/// [pt] O comparador da recusa e da retirada: a cópia em ensaio (sem as chamadas que mexem no resto do jogo) e a original a
/// partir do mesmo estado (a oferta, as duas listas dela, as vagas da troca, o registro da pessoa em +0x86e e
/// [0xae1080]); iguais o estado (+0x2c), +0x10, a marca +4 de cada jogador da troca e a marca de oferta da pessoa, que
/// a original grava sem depender das chamadas.
pub(crate) unsafe fn compare_reject_withdraw(nome: &str, this: usize, vaga: *mut usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let o = *vaga;
    let marca = read_usize(this + 0x86e).wrapping_add(read_usize(o + 0xc).wrapping_mul(8));
    let nos = llist_items_keep_cursor(read_usize(o + 4));
    let mut regioes = vec![(o, 0x30usize), (marca, 8), (0xae1080, 4)];
    for l in [read_usize(o), read_usize(o + 4)] {
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

pub(crate) unsafe extern "thiscall" fn comparar_reject_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_REJECT_TRANSFER_OFFER);
    compare_reject_withdraw("reject_transfer_offer", this, vaga, || o(this, vaga, marcar, motivo), || reject_transfer_offer(this, vaga, marcar, motivo))
}

pub(crate) unsafe extern "thiscall" fn comparar_withdraw_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_WITHDRAW_TRANSFER_OFFER);
    compare_reject_withdraw("withdraw_transfer_offer", this, vaga, || o(this, vaga, marcar, motivo), || withdraw_transfer_offer(this, vaga, marcar, motivo))
}

/// the 0xfa1 news of a rejected or withdrawn offer, with the reason, to the interested human clubs
/// Original: `post_offer_rejected_news` (0x8d5170, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d5170 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_rejected_news(oferta, motivo)`, pela recusa, pela retirada e pela recusa do empréstimo. Os clubes humanos
/// que recebem (0x8d1130, com 0x2d se a oferta estiver recusada, senão 0x2e; até 30); sem nenhum, nada. A notícia 0xfa1:
/// a pessoa (o id e as três chaves), o clube do contrato (sem contrato de tipo 5; nenhum no motivo 6 com +0x2f), o
/// comprador, a lista da troca (+4), o estado, o motivo, +0x1f, o valor total (0x8e7a70 com 0, 0, 0, 1) e a divisão do
/// clube do contrato (+0x57); a cada destinatário, na vaga 12, 1 se for o comprador ou o clube do contrato.
pub(crate) unsafe extern "thiscall" fn post_offer_rejected_news(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let tipo5 = |c: usize| read_u8(c + 0x35) & 0x3f == 5;
    let c = transfer_manager_get_contract_and_store(p);
    let do_contrato = if c == 0 || tipo5(c) { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (read_u8(o + 0x2c) != 2) as u32 + 0x2d, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    let c = transfer_manager_get_contract_and_store(p);
    let mut clube = if c == 0 || tipo5(c) { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let c = transfer_manager_get_contract_and_store(p);
    if c != 0 && tipo5(c) {
        clube = 0;
    }
    if motivo as i16 == 6 && read_u8(o + 0x2f) != 0 {
        clube = 0;
    }
    crate::comparacao::ramo("cpx_1 noticia da recusa");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa1, 0);
    gravar(nt, 0, read_usize(p) as u32);
    gravar(nt, 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX });
    gravar(nt, 5, read_usize(o + 8) as u32);
    gravar(nt, 6, read_usize(o + 4) as u32);
    gravar(nt, 7, read_u8(o + 0x2c) as i8 as u32);
    gravar(nt, 8, motivo as i16 as u32);
    gravar(nt, 9, read_u8(o + 0x1f) as i8 as u32);
    gravar(nt, 10, total(o, 0, 0, 0, 1));
    let divisao = if clube != 0 { read_usize(clube + club::DIVISION) } else { 0 };
    gravar(nt, 11, if divisao != 0 { read_usize(divisao) as u32 } else { u32::MAX });
    for &d in lista.iter().take(n.max(0) as usize) {
        gravar(nt, 12, (d == comprador || d == do_contrato) as u32);
        entregar(read_usize(G_NEWS), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_rejected_news(this: usize, o: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_OFFER_REJECTED_NEWS)(this, o, motivo)
}

// ---------- 260a leva: o jogador desassossegado pelo lance de um clube maior, a notícia da oferta desfeita e a do prêmio por jogos (transfer_manager.cpp) ----------








/// Person -> main contract, also written to G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O mesmo, guardado em [0xae1080] como a original faz.
pub(crate) unsafe fn get_contract_store_lookup(pessoa: usize) -> usize {
    let c = contract_of_staff(pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// O ensaio do comparador: a cópia para antes da queixa (0x867700), da fuga (0x419840) e das antipatias (0x547e70,
/// 0x547af0), que mexem no resto do jogo, e marca que chegou lá.
pub(crate) static mut CPX2_ENSAIO: bool = false;
pub(crate) static mut CPX2_CHEGOU: bool = false;

/// a draw against the buyer's reputation flags the player as unsettled; when flagged, a score from his attributes and
/// interest leads to the 0x80 complaint, going missing and grudges
/// Original: `unsettle_player_after_bid` (0x8c1290, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1290 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::unsettle_player_after_bid(oferta)`, pela
/// recusa (0x8bf680), pela retirada no motivo 2 (0x8bfcb0) e pelo clube que recusa (0x8c36b0). Sorteio(5750) + 4250
/// abaixo da reputação do comprador (+0x80): +0x2d da oferta = 1. Com +0x2d = 1, clube do contrato, comprador e ficha: o
/// interesse x (0x846920 pelo estatuto 0x849f00) e a nota n = (8 × ambição² − 1600) − reputação / 40 − min(x⁴, 300 ×
/// x) − 2 × lealdade²... (os atributos +0x5d, +0x57, +0x5b, +0x59 pela pow do jogo); listado (+0x4e & 8): − 2000 −
/// sorteio(6000). Com x de 5,5, n < −sorteio(700), (n < −4000 ou +0x5b + +0x59 < 15), reputação acima de 3500 e o
/// valor ponderado da oferta (0x8e7b50) acima do menor entre 1,5 × valor (0x4d0c60) e 0,8 × preço (0x4d1600): a queixa
/// 0x80 (0x867700); jogador de 5250, idade 19 a 31, n < −8000, com o critério (0x866f60), sem a marca +1 do registro
/// +0x86e e presente (0x418f60): some (0x419840) e, com +0x44 − sorteio(200) + sorteio(400) < −150, rejeita o clube
/// (0x547e70). x acima de 10, n < −5000 ou o menor abaixo de 0,75 × o ponderado: antipatia ao técnico (0x547af0).
pub(crate) unsafe extern "thiscall" fn unsettle_player_after_bid(this: usize, o: usize) {
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let c = get_contract_store_lookup(p);
    let do_contrato = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let r = sortear(0x1676);
    crate::comparacao::ramo("cpx_2 sorteio do desassossego");
    if (read_i16(comprador + 0x80) as i32) > r.wrapping_add(0x109a) { // cpx2: o sorteio do desassossego
        *((o + 0x2d) as *mut u8) = 1;
    }
    if read_u8(o + 0x2d) != 1 || do_contrato == 0 || comprador == 0 || read_usize(p + 0x61) == 0 {
        return;
    }
    let c2 = contract_of_staff(p);
    let listas = read_usize(this + 0x856);
    let estatuto: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_GET_SQUAD_STATUS_FOR_CLUB);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let e = estatuto(listas, p, comprador, 0);
    // ponytail: a original empurra o dword da vaga onde guardou o byte (o resto é sobra de pilha); a 0x846920 só lê o
    // byte
    let x = interesse(listas, p, comprador, 1, 0, e as u32, 0);
    let pw = game_pow(x, read_f64_at(0x968938));
    let a = x * read_f64_at(0x96a6b0);
    let v = if !(pw >= a) { pw } else { a };
    let mut n = match_pl_ftol(game_pow(read_i8_as_i32(p + 0x5d) as f64, read_f64_at(0x968950)) * read_f64_at(0x968f10) - read_f64_at(0x96a590)) as i16;
    let ficha = read_usize(p + 0x61);
    n = n.wrapping_sub((read_i16(ficha + 0xb) as i32 / 40) as i16);
    n = n.wrapping_sub(match_pl_ftol(v) as i16);
    n = n.wrapping_sub(match_pl_ftol(game_pow(read_i8_as_i32(p + 0x57) as f64, read_f64_at(0x968950))) as i16);
    n = n.wrapping_add(match_pl_ftol(game_pow(read_i8_as_i32(p + 0x5b) as f64, read_f64_at(0x968930))) as i16);
    n = n.wrapping_add(match_pl_ftol(game_pow(read_i8_as_i32(p + 0x59) as f64, read_f64_at(0x968930))) as i16);
    if c2 == 0 {
        return;
    }
    if read_u8(c2 + 0x4e) & 8 != 0 {
        crate::comparacao::ramo("cpx_2 listado");
        n = n.wrapping_add((-2000i32).wrapping_sub(sortear(0x1770)) as i16); // cpx2: o desconto do listado
    }
    crate::comparacao::ramo("cpx_2 interesse avaliado");
    if !(x >= read_f64_at(0x96d298)) { // cpx2: o interesse mínimo
        return;
    }
    if n as i32 >= -sortear(0x2bc) {
        return;
    }
    if n >= -4000 && read_i8_as_i32(p + 0x5b) + read_i8_as_i32(p + 0x59) >= 0xf {
        return;
    }
    if read_i16(read_usize(p + 0x61) + 0xb) <= 0xdac {
        return;
    }
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let a1 = match_pl_ftol(valor(read_usize(G_CONTRACT_MANAGER), p, do_contrato, 0, -1, 0) as f64 * read_f64_at(0x968940));
    let b1 = match_pl_ftol(preco(read_usize(G_CONTRACT_MANAGER), do_contrato, comprador, p, 0, 1, 1) as f64 * read_f64_at(0x9689b0));
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
    let queixa: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    queixa(c2, 0x80, 0);
    let idade = read_i8_as_i32(p + 0x18);
    if read_i16(read_usize(p + 0x61) + 0xb) > 0x1482 && idade > 0x12 && idade < 0x20 && n < -8000 {
        let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
        let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
        if criterio(c2) != 0
            && read_u8(read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8)) + 1) == 0
            && ausente(read_usize(G_AWOL), p, 0) == 0
        {
            let sumir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_AWOL_START_ABSENCE);
            sumir(read_usize(G_AWOL), p, comprador, 1);
            let forca = read_i8_as_i32(c2 + 0x44);
            let r1 = sortear(0xc8);
            let r2 = sortear(0x190);
            if forca.wrapping_sub(r1).wrapping_add(r2) < -150 {
                let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_CLUB);
                rejeitar(p, do_contrato, 1);
            }
        }
    }
    if x > read_f64_at(0x9688b0) || n < -5000 || (menor as f64) < pond as f64 * read_f64_at(0x969040) {
        let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
        desafeto(p, read_usize(do_contrato + 0xcf), 1);
    }
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado (a oferta, [0xae1080] e o sorteio). Sem chegar
/// à queixa (quase sempre), tudo igual; chegando, só a original (a queixa, a fuga e as antipatias não podem rodar duas
/// vezes e a original sorteia mais).
pub(crate) unsafe extern "thiscall" fn comparar_unsettle_player_after_bid(this: usize, o: usize) {
    let orig_f = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UNSETTLE_PLAYER_AFTER_BID);
    let mut regioes = vec![(o, 0x30usize), (0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPX2_ENSAIO = true;
    CPX2_CHEGOU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| unsettle_player_after_bid(this, o)));
    CPX2_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    orig_f(this, o);
    if CPX2_CHEGOU {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("unsettle_player_after_bid(oferta {:#x}) original={:x?} rust={:x?}", o, mo, mr));
}

/// the 0xfb0 news of a collapsed offer, with the club and the reason, to the interested human clubs
/// Original: `post_offer_collapsed_news` (0x8de790, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8de790 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_collapsed_news(oferta, clube, motivo)`, pela oferta de contrato aceita (0x8c38b0), pela oferta desfeita
/// (0x8c44e0), pelas permissões de trabalho (0x8c7690) e por 0x8c4950. Os clubes humanos que recebem (0x8d1130 com 0x2f,
/// até 30); sem nenhum, nada. A notícia 0xfb0: o clube do contrato (sem contrato de tipo 5), o comprador, a pessoa (o id
/// e as três chaves), o clube, o motivo e +0x1f; a cada destinatário, menos o próprio clube no motivo 0xb.
pub(crate) unsafe extern "thiscall" fn post_offer_collapsed_news(this: usize, o: usize, clube: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let c = get_contract_store_lookup(p);
    let mut do_contrato = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x2f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    if do_contrato != 0 && read_u8(contract_of_staff(p) + 0x35) & 0x3f == 5 {
        do_contrato = 0;
    }
    crate::comparacao::ramo("cpx_2 noticia da oferta desfeita");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb0, 0);
    gravar(nt, 0, if do_contrato != 0 { read_usize(do_contrato) as u32 } else { u32::MAX });
    gravar(nt, 1, read_usize(comprador) as u32);
    gravar(nt, 2, read_usize(p) as u32);
    gravar(nt, 3, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, 4, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, 5, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    gravar(nt, 6, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX });
    gravar(nt, 7, motivo as i16 as u32);
    gravar(nt, 8, read_u8(o + 0x1f) as i8 as u32);
    for &d in lista.iter().take(n.max(0) as usize) {
        if d != clube || motivo as u16 != 0xb {
            entregar(read_usize(G_NEWS), nt, d);
        }
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_collapsed_news(this: usize, o: usize, clube: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_POST_OFFER_COLLAPSED_NEWS)(this, o, clube, motivo)
}

/// the 0xfa6 news of appearance money paid, to the paying club and to the contract club
/// Original: `post_appearance_money_news` (0x8db8a0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8db8a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_appearance_money_news(cláusula, valor, tipo, calado)`, pelo prêmio por jogos (0x8cb340). Calado, sem contrato ou
/// sem o clube dele: nada. Senão a notícia 0xfa6: a pessoa da cláusula (+0: o id e as três chaves), +4, o valor, o
/// clube que paga (+9), o tipo e o clube do contrato; ao clube que paga e ao clube do contrato.
pub(crate) unsafe extern "thiscall" fn post_appearance_money_news(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    let _ = this;
    if calado != 0 {
        return;
    }
    let p = staff_records_staff_by_id(read_usize(clausula));
    let c = get_contract_store_lookup(p);
    if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpx_2 noticia do premio por jogos");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa6, 0);
    gravar(nt, 0, read_usize(p) as u32);
    gravar(nt, 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, read_usize(clausula + 4) as u32);
    gravar(nt, 5, valor);
    gravar(nt, 6, read_usize(clausula + 9) as u32);
    gravar(nt, 7, tipo as i8 as u32);
    let c = get_contract_store_lookup(p);
    // a original lê o id do clube do contrato mesmo sem contrato (o registro 0): nunca acontece aqui
    let clube = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    gravar(nt, 8, read_usize(clube) as u32);
    entregar(read_usize(G_NEWS), nt, friendly_club_by_id(read_usize(clausula + 9)));
    let c = get_contract_store_lookup(p);
    entregar(read_usize(G_NEWS), nt, if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) });
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_appearance_money_news(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_APPEARANCE_MONEY_NEWS)(this, clausula, valor, tipo, calado)
}

// ---------- 260a leva: a transferência efetivada, com a notícia da transferência concluída e a do acordo entre os clubes (transfer_manager.cpp) ----------







/// Person -> main contract, also written to G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O mesmo, guardado em [0xae1080] como a original faz.
pub(crate) unsafe fn get_contract_store_result(pessoa: usize) -> usize {
    let c = contract_of_staff(pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Person -> club of their main contract unless it is type 5 (or 0), contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato principal que não é de tipo 5 (ou 0), com o contrato guardado em [0xae1080].
pub(crate) unsafe fn contract_club_not_type5(pessoa: usize) -> usize {
    let c = get_contract_store_result(pessoa);
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        0
    } else {
        friendly_club_by_id(read_usize(c + 4))
    }
}



/// For a non-human club that uses squad numbers (SQUAD_MANAGER), allocates the numbers of new arrivals.
///
/// [pt] O clube que recebe: sem técnico humano (0x600a50) e com números de camisa (SQUAD_MANAGER 0x85add0 em [0xdd7ef0]),
/// os números de quem chegou (0x85a470).
pub(crate) unsafe fn allocate_arrival_squad_numbers(clube: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    if usa(read_usize(G_SQUAD_MANAGER), clube, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
        numerar(read_usize(G_SQUAD_MANAGER), clube, 0); // cpx3: os números de camisa de quem chegou
    }
}

/// carries out the transfer: board confidence, nation rules, clauses, payment, records, former clubs' cut, the player
/// and the exchange players moved, squad numbers, state 0x14 and back in 10 days
/// Original: `finalise_transfer` (0x8c9390, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c9390 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::finalise_transfer(oferta)`, pelo
/// fechamento da oferta (0x8c34e0) e pela transferência futura. A confiança das diretorias (0x6978f0); as regras da
/// nação do comprador (as 28 de +0x8ac, pela nação +0xa): método 1 (sem +0x1f) ou 2 com a oferta; as cláusulas
/// anteriores (0x8c91e0); com vendedor (contrato sem tipo 5), o pagamento (0x8ca8a0); sem +0x1f = 1, os registros do
/// técnico (0x86cd70); os recordes do clube (0x448760). Com troca, o primeiro da troca sai do elenco do clube dele
/// (+0xd7, 50 vagas). Com valor (+0x1b) e sem +0x69 na pessoa, a parte dos clubes anteriores (0x8e4f20). O jogador vai
/// ao comprador (0x8c9720) e os da troca ao vendedor, com os números de camisa; estado 0x14, a oferta volta em 10 dias
/// (0x8c6220); sem rival em aberto, a marca de oferta (+0x86e) sai; os lances (0x84aeb0) e as ofertas de contrato
/// ligadas (0x8c6db0).
pub(crate) unsafe extern "thiscall" fn finalise_transfer(this: usize, o: usize) {
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let vendedor = contract_club_not_type5(p);
    let confianca: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_APPLY_TRANSFER_CONFIDENCE_CHANGES);
    crate::comparacao::ramo("cpx_3 confianca das diretorias");
    confianca(read_usize(G_MANAGER_MANAGER), o); // cpx3: a confiança das diretorias
    let nacao = read_usize(friendly_club_by_id(read_usize(o + 8)) + 0x53);
    for k in 0..0x1c {
        let regras = read_usize(this + 0x8ac + k * 4);
        if read_usize(regras + 0xa) == nacao {
            if regras != 0 {
                let metodo = read_usize(read_usize(regras) + if read_u8(o + 0x1f) == 0 { 4 } else { 8 });
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
    if read_u8(o + 0x1f) != 1 {
        com_a_oferta(0x86cd70, read_usize(G_STAFF_RECORDS));
    }
    com_a_oferta(0x448760, read_usize(G_CLUB_RECORDS));
    let troca = read_usize(o + 4);
    if troca != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((troca + 0xa) as *mut usize, read_usize(troca + 2));
        let q = staff_records_staff_by_id(read_usize(proximo(read_usize(o + 4))));
        let clube = read_usize(q + 0x39);
        for j in 0..0x32usize {
            let vaga = clube + club::SQUAD + j * 4;
            if read_usize(vaga) == q {
                *(vaga as *mut usize) = 0;
                break;
            }
        }
    }
    if read_usize(o + 0x1b) != 0 && read_usize(p + 0x69) == 0 {
        crate::comparacao::ramo("cpx_3 parte dos clubes anteriores");
        com_a_oferta(0x8e4f20, this); // cpx3: a parte dos clubes anteriores
    }
    let mudar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_MOVE_PLAYER_TO_CLUB);
    mudar(this, p, comprador, o);
    allocate_arrival_squad_numbers(comprador);
    if read_usize(o + 4) != 0 {
        for e in llist_items_from_start(read_usize(o + 4)) {
            mudar(this, staff_records_staff_by_id(read_usize(e)), vendedor, o);
        }
        allocate_arrival_squad_numbers(vendedor);
    }
    *((o + 0x2c) as *mut u8) = 0x14;
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    agendar(this, o, 0xa, 0);
    'marca: {
        if !offer_is_closed(o) {
            break 'marca;
        }
        let rivais = read_usize(o);
        if rivais != 0 {
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
            core::ptr::write_unaligned((rivais + 0xa) as *mut usize, read_usize(rivais + 2));
            let mut e = proximo(read_usize(o));
            while e != 0 {
                if !offer_is_closed(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)))) {
                    break 'marca;
                }
                e = proximo(read_usize(o));
            }
        }
        let m = (read_usize(this + 0x86e).wrapping_add(read_usize(o + 0xc).wrapping_mul(8))) as *mut u8;
        if *m & 1 != 0 {
            *m &= 0xfe;
        }
    }
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let c = get_contract_store_result(p);
    let novo = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    desmarcar(read_usize(this + 0x856), friendly_club_by_id(read_usize(o + 8)), p);
    if read_usize(o + 4) != 0 {
        for e in llist_items_from_start(read_usize(o + 4)) {
            desmarcar(read_usize(this + 0x856), novo, staff_records_staff_by_id(read_usize(e)));
        }
    }
    com_a_oferta(0x8c6db0, this);
}

// ponytail: muda jogadores de clube, paga e agenda (não pode rodar duas vezes): só a original, com o plano do estado
// final (0x14, que nenhuma chamada depois dele muda); a cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_finalise_transfer(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FINALISE_TRANSFER)(this, o);
    let e = read_u8(o + 0x2c);
    PLACAR.registrar(e == 0x14, || format!("finalise_transfer(oferta {:#x}) estado={:#x} (plano 0x14)", o, e));
}

/// Writes the person's id and three keys (+0x33 of +4, +8, +0xc) into news slots k..k+3.
///
/// [pt] A notícia local com a pessoa (o id e as três chaves) nas vagas `k` a `k` + 3.
pub(crate) unsafe fn put_person_in_news(nt: usize, k: u8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    gravar(nt, k as i8, read_usize(p) as u32);
    gravar(nt, k.wrapping_add(1) as i8, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, k.wrapping_add(2) as i8, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, k.wrapping_add(3) as i8, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// Id of the person's non-type-5 contract club, or -1 (value for news slot 5).
///
/// [pt] A vaga 5 das duas notícias: o id do clube do contrato (sem tipo 5) ou −1 (a original relê o contrato e, sem ele, lê
/// o id no endereço 0, o que não acontece porque acabou de achá-lo).
pub(crate) unsafe fn contract_club_id_or_minus1(p: usize) -> u32 {
    if contract_club_not_type5(p) == 0 {
        return u32::MAX;
    }
    read_usize(contract_club_not_type5(p)) as u32
}

/// Writes the first three exchange players of the offer into the news from slot k (4 slots each); returns (count, next
/// slot).
///
/// [pt] Os da troca (+4) na notícia a partir da vaga `k` (os três primeiros, quatro vagas cada); devolve (quantos, a vaga
/// seguinte).
pub(crate) unsafe fn put_exchange_players_in_news(nt: usize, o: usize, mut k: u8) -> (i8, u8) {
    let mut n = 0i8;
    let troca = read_usize(o + 4);
    if troca != 0 {
        for e in llist_items_from_start(troca) {
            if n < 3 {
                put_person_in_news(nt, k, staff_records_staff_by_id(read_usize(e)));
                k = k.wrapping_add(4);
            }
            n = n.wrapping_add(1);
        }
    }
    (n, k)
}

/// Sends the news to all managers if the buyer has a human manager, otherwise to each of the first n recipients.
///
/// [pt] A entrega: com técnico humano no comprador (0x7875e0), a todos os técnicos (0x786780); senão a cada destinatário.
pub(crate) unsafe fn deliver_transfer_news(nt: usize, o: usize, lista: &[usize], n: i8) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if humano(read_usize(G_NEWS), friendly_club_by_id(read_usize(o + 8))) != 0 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        todos(read_usize(G_NEWS), nt);
        return;
    }
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(read_usize(G_NEWS), nt, d);
    }
}

/// the 0xfa8 news of a completed transfer, with the fee, the exchange players, a draw and the player's strong points
/// Original: `post_transfer_completed_news` (0x8dbef0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8dbef0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_completed_news(oferta)`, pelo fechamento da oferta (0x8c34e0). Os clubes humanos que recebem (0x8d1130
/// com 0x5f, até 30); sem nenhum, nada. A notícia 0xfa8: a pessoa, o comprador, o clube do contrato, o valor (+0x1b), 0,
/// o valor total (0x8e7a70 com 1, 1, 0, 1), +0x2f, +0x11, as cláusulas (0x8e7ef0); com +0x2f, contrato de clube e +0x30
/// & 0xf = 3, o fim do contrato se ainda vier (vagas 10 e 11). Os da troca (até 3, da vaga 15) e quantos (14); um
/// sorteio(3) na vaga seguinte; com ficha, 1, os pontos fortes (0x546ab0, sorteia) e as posições (0x5b7d20), senão 0.
pub(crate) unsafe extern "thiscall" fn post_transfer_completed_news(this: usize, o: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let clausulas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_CLAUSE_TOTAL);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0x5f, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia concluida");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa8, 0);
    put_person_in_news(nt, 0, p);
    gravar(nt, 4, read_usize(o + 8) as u32);
    gravar(nt, 5, contract_club_id_or_minus1(p));
    gravar(nt, 6, read_usize(o + 0x1b) as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, total(o, 1, 1, 0, 1));
    gravar(nt, 9, read_u8(o + 0x2f) as i8 as u32);
    gravar(nt, 0xc, read_u8(o + 0x11) as i8 as u32);
    gravar(nt, 0xd, clausulas(o));
    if read_u8(o + 0x2f) != 0 && contract_club_not_type5(p) != 0 && read_u8(o + 0x30) & 0xf == 3 {
        let c = contract_of_staff(p);
        let ano = read_i16(G_CURRENT_DATE_YEAR) as u16;
        let vem = c != 0
            && (ano < read_i16(c + 0x2f) as u16 || (ano == read_i16(c + 0x2f) as u16 && read_i16(G_CURRENT_DATE) < read_i16(c + 0x2d)));
        if vem {
            gravar(nt, 10, read_i16(c + 0x2d) as i32 as u32);
            gravar(nt, 11, read_i16(c + 0x2f) as u16 as u32);
        } else {
            gravar(nt, 10, 0);
            gravar(nt, 11, 0);
        }
    }
    let (m, k) = put_exchange_players_in_news(nt, o, 0xf);
    gravar(nt, 0xe, m as i32 as u32);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    gravar(nt, k as i8, sorteio(3) as u32);
    let ficha = read_usize(p + 0x61);
    if ficha != 0 {
        gravar(nt, k.wrapping_add(1) as i8, 1);
        let fortes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_GET_PLAYER_STRENGTH_FLAGS);
        let posicoes: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
        let clube = contract_club_not_type5(p);
        gravar(nt, k.wrapping_add(2) as i8, fortes(p, clube));
        gravar(nt, k.wrapping_add(3) as i8, posicoes(read_usize(p + 0x61)) as i32 as u32);
    } else {
        gravar(nt, k.wrapping_add(1) as i8, 0);
    }
    deliver_transfer_news(nt, o, &lista, n);
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_completed_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_TRANSFER_COMPLETED_NEWS)(this, o)
}

/// the 0xfb3 news of the clubs' agreement, with the fee, the exchange players and the completion date
/// Original: `post_transfer_agreed_news` (0x8e05d0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8e05d0 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_agreed_news(oferta, data)`, pela aprovação dos clubes (0x8c3310). Os clubes humanos que recebem
/// (0x8d1130 com 0xf, até 30); sem nenhum, nada. A notícia 0xfb3: a pessoa, o comprador, o clube do contrato, o valor
/// (+0x1b), +0x2f, o valor total (0x8e7a70 com 0, 1, 0, 1), o nibble baixo de +0x30 com sinal, a data da conclusão (dia
/// com sinal, ano), os da troca (até 3, da vaga 13) e quantos (12).
pub(crate) unsafe extern "thiscall" fn post_transfer_agreed_news(this: usize, o: usize, data: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, 0xf, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_3 noticia da transferencia acertada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfb3, 0);
    put_person_in_news(nt, 0, p);
    gravar(nt, 4, read_usize(o + 8) as u32);
    gravar(nt, 5, contract_club_id_or_minus1(p));
    gravar(nt, 6, read_usize(o + 0x1b) as u32);
    gravar(nt, 7, read_u8(o + 0x2f) as i8 as u32);
    gravar(nt, 8, total(o, 0, 1, 0, 1));
    gravar(nt, 9, (((read_u8(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    gravar(nt, 0xa, read_i16(data) as i32 as u32);
    gravar(nt, 0xb, read_i16(data + 2) as u16 as u32);
    let (m, _) = put_exchange_players_in_news(nt, o, 0xd);
    gravar(nt, 0xc, m as i32 as u32);
    deliver_transfer_news(nt, o, &lista, n);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_agreed_news(this: usize, o: usize, data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_TRANSFER_AGREED_NEWS)(this, o, data)
}

// ---------- 260a leva: o empréstimo do arquivo de configuração, a notícia do fim do contrato e a do pedido de adiamento da transferência (transfer_manager.cpp, contract_manager.cpp) ----------






/// Copies date `origem` into `destino` (Date::operator= 0x417c10).
///
/// [pt] Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): copia a data nos dois.
pub(crate) unsafe fn transfer_manager_assign_date(destino: usize, origem: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut resultado = [0u8; 8];
    f(destino, resultado.as_mut_ptr(), origem);
}

/// Date `a` (signed day +0, year +2) is before date `b`.
///
/// [pt] A data `a` (dia com sinal em +0, ano em +2) vem antes da `b`.
pub(crate) unsafe fn transfer_manager_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_i16(a + 2) as u16, read_i16(b + 2) as u16);
    ya < yb || (ya == yb && read_i16(a) < read_i16(b))
}

/// Checks the config loan conditions (linked B team, squad < 50, no current loan, start <= today <= end, has a
/// contract); returns Some(B team or 0) if allowed.
///
/// [pt] As condições do empréstimo da configuração, na ordem da original: o time B ligado ao clube (0x540a50; vale só se não
/// for o principal), o elenco do clube abaixo de 50 (0x53c590), a pessoa sem empréstimo (0x4cf6d0), o começo até hoje,
/// o fim de hoje em diante e um contrato (0x4cf5a0). Devolve o time B (ou 0) quando passam.
pub(crate) unsafe fn config_loan_allowed(p: usize, clube: usize, inicio: usize, fim: usize) -> Option<usize> {
    let ligado: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let mut principal = 0u32;
    let mut b = ligado(clube, &mut principal, 1);
    if b != 0 && principal != 0 {
        b = 0;
    }
    if contar(clube, 0, 0) as i8 >= 0x32 || emprestado(read_usize(G_CONTRACT_MANAGER), p) != 0 {
        return None;
    }
    if transfer_manager_date_before(0xae2c90, inicio) || transfer_manager_date_before(fim, 0xae2c90) {
        return None;
    }
    if contrato(read_usize(G_CONTRACT_MANAGER), p) == 0 {
        return None;
    }
    Some(b)
}

/// a loan from the configuration file: the checks, the contract extended to its end, the loan contract, the squad, the
/// squad numbers and the work permit
/// Original: `add_config_loan` (0x8bf400, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8bf400 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH do contrato local): `TRANSFER_MANAGER::
/// add_config_loan(pessoa, clube, começo, fim)`, pelos empréstimos do arquivo de configuração no começo do jogo. Pelas
/// condições (config_loan_allowed), o empréstimo vai ao time B ligado ou ao clube: com o fim depois da data +0x46 da pessoa, a
/// data e o fim do contrato (+0x2d) passam ao fim; o contrato local (0x85e410, 0x85e590 com o fim, 0, 1, 1; começo em
/// +0x25) é assinado (0x4d01e0); as suspensões ficam guardadas (0x5563f0); o elenco 1 (0x540980); +0x6d 2 vira 0x10; os
/// números de camisa (0x85add0, 0x85a470); no time B, a pessoa vai às reservas (0x4cc0f0). Precisando de permissão de
/// trabalho no clube (0x8c7560), o fim recebido anda 7 dias e a permissão é dada (0x8c7070 com a nação do clube e o ano
/// do começo).
pub(crate) unsafe extern "thiscall" fn add_config_loan(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let mut local = [0u8; 0x50]; // o STAFF_CONTRACT local (0x50 bytes)
    let l = local.as_mut_ptr() as usize;
    construir(l);
    let b = match config_loan_allowed(p, clube, inicio, fim) {
        Some(b) => b,
        None => return,
    };
    if transfer_manager_date_before(p + 0x46, fim) {
        crate::comparacao::ramo("cpx_4 contrato estendido");
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        transfer_manager_assign_date(p + 0x46, fim); // cpx4: a data da pessoa até o fim
        transfer_manager_assign_date(contrato(read_usize(G_CONTRACT_MANAGER), p) + 0x2d, fim); // cpx4: o fim do contrato até o fim do empréstimo
    }
    let destino = if b != 0 { b } else { clube };
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_INIT_FROM_CURRENT_CONTRACT);
    preencher(l, p, destino, fim, 0, 1, 1);
    transfer_manager_assign_date(l + 0x25, inicio);
    crate::comparacao::ramo("cpx_4 emprestimo da configuracao");
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_SIGN_LOAN_CONTRACT);
    assinar(read_usize(G_CONTRACT_MANAGER), l); // cpx4: o contrato de empréstimo
    let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_HOLD_BANS_FOR_LOAN);
    guardar(read_usize(G_DISCIPLINE), p);
    let elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    elenco(p, 1);
    if read_u8(p + 0x6d) == 2 {
        *((p + 0x6d) as *mut u8) = 0x10;
    }
    let usa: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    if usa(read_usize(G_SQUAD_MANAGER), destino, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
        numerar(read_usize(G_SQUAD_MANAGER), destino, 0);
    }
    if b != 0 && b != clube {
        let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_MOVE_TO_RESERVES);
        reservas(read_usize(G_CONTRACT_MANAGER), p, b);
    }
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if precisa(this, p, clube, 0) != 0 {
        crate::comparacao::ramo("cpx_4 permissao de trabalho");
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let mut t = [0u8; 8];
        let r = mais_dias(fim, t.as_mut_ptr(), 7);
        transfer_manager_assign_date(fim, r);
        // a original empurra cx sobre o ecx que a 0x417c10 zera: o ano limpo
        let permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_WORK_PERMIT);
        permissao(this, p, read_usize(clube + club::NATION), read_i16(inicio + 2) as u16 as u32, 1, 1); // cpx4: a permissão de trabalho
    }
}

// ponytail: assina o empréstimo, mexe no elenco e nas permissões (não pode rodar duas vezes): só a original, com o
// plano pelas mesmas condições (passando, a pessoa sai emprestada; senão o empréstimo dela fica como estava).
pub(crate) unsafe extern "thiscall" fn comparar_add_config_loan(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let tinha = emprestado(read_usize(G_CONTRACT_MANAGER), p);
    let passa = config_loan_allowed(p, clube, inicio, fim).is_some();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_ADD_CONFIG_LOAN)(this, p, clube, inicio, fim);
    let tem = emprestado(read_usize(G_CONTRACT_MANAGER), p);
    let igual = if passa { tem != 0 } else { tem == tinha };
    PLACAR.registrar(igual, || format!("add_config_loan(pessoa {:#x}) passa={} antes={:#x} depois={:#x}", read_usize(p), passa, tinha, tem));
}

/// Person -> main contract computed inline as 0x4da590 does (staff state index into the contract vector), stored in
/// G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato principal da pessoa como a 0x4da590 o acha (o mesmo da 0x4cf5a0, em linha): sem pessoa, id fora da tabela
/// ou índice negativo no registro de 79 bytes, nenhum; guardado em [0xae1080].
pub(crate) unsafe fn contract_inline_lookup(p: usize) -> usize {
    let c = if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f))) as i32;
        if k < 0 {
            0
        } else {
            read_usize(read_usize(G_CONTRACT_MANAGER)).wrapping_add((k as usize).wrapping_mul(0x50))
        }
    };
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Fills the local LLIST with clubs interested in the person (Shortlist_Manager 0x842180) and returns their ids.
///
/// [pt] Os clubes interessados na pessoa (Shortlist_Manager 0x842180 com 1, 1, 0) numa LLIST local já construída; devolve os
/// ids, do começo.
pub(crate) unsafe fn interested_club_ids(p: usize, l: usize) -> Vec<usize> {
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    interessados(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(read_usize(e + 4));
        e = proximo(l);
    }
    v
}


// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.

/// the 0xfab news of a club asking to delay a transfer by a week, to the interested human clubs
/// Original: `post_transfer_delay_request_news` (0x8dda90, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8dda90 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_delay_request_news(oferta, motivo)`, pelo clube que pede para adiar a transferência uma semana. Os
/// clubes humanos que recebem (0x8d1130 com 1, ou 2 fora do estado 0x11; até 30); sem nenhum, nada. A notícia 0xfab: o
/// clube do contrato (sem tipo 5) ou −1, o comprador, a pessoa (o id e as três chaves), o estado, 0, 0, a vaga +0x20 e o
/// motivo; a cada destinatário.
pub(crate) unsafe extern "thiscall" fn post_transfer_delay_request_news(this: usize, o: usize, motivo: u32) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let p = staff_records_staff_by_id(read_usize(o + 0xc));
    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let do_contrato = if c == 0 || read_u8(c + 0x35) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let mut lista = [0usize; 0x1e];
    let n = destinatarios(this, o, (read_u8(o + 0x2c) != 0x11) as u32 + 1, lista.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparacao::ramo("cpx_4 noticia do adiamento");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfab, 0);
    gravar(nt, 0, if do_contrato != 0 { read_usize(do_contrato) as u32 } else { u32::MAX });
    gravar(nt, 1, read_usize(comprador) as u32);
    gravar(nt, 2, read_usize(p) as u32);
    gravar(nt, 3, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, 4, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, 5, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    gravar(nt, 6, read_u8(o + 0x2c) as i8 as u32);
    gravar(nt, 7, 0);
    gravar(nt, 8, 0);
    gravar(nt, 9, read_usize(o + 0x20) as u32);
    gravar(nt, 10, motivo as i16 as u32);
    for &d in lista.iter().take(n.max(0) as usize) {
        entregar(read_usize(G_NEWS), nt, d);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_delay_request_news(this: usize, o: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_TRANSFER_DELAY_REQUEST_NEWS)(this, o, motivo)
}

// ---------- 259a leva: a aprovação final da oferta pelos clubes, o fechamento da oferta aprovada e o acerto das cláusulas das transferências anteriores (transfer_manager.cpp) ----------







/// TRANSFER_MANAGER::remove_exchange_player(person, offer, 0, 0): removes the person from other offers' exchange lists.
///
/// [pt] Tira a pessoa das listas de troca das outras ofertas (TRANSFER_MANAGER::remove_exchange_player 0x8c41c0 com a
/// oferta, 0, 0).
pub(crate) unsafe fn remove_from_other_exchanges(this: usize, pessoa: usize, oferta: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_EXCHANGE_PLAYER);
    f(this, pessoa, oferta, 0, 0);
}

/// TRANSFER_MANAGER::schedule_offer(offer, days, slices).
///
/// [pt] Agenda a oferta (0x8c6220: thiscall(this, oferta, dias, fatias)).
pub(crate) unsafe fn schedule_offer_with_slices(this: usize, oferta: usize, dias: u32, fatias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    f(this, oferta, dias, fatias);
}

/// Offer status after `clube` approves: buyer moves 0xb->0xd and 0xc->0xe, the other side 0xb->0xc and 0xd->0xe; others
/// unchanged.
///
/// [pt] O estado (+0x2c) da oferta depois da aprovação do `clube`: o comprador (+8) leva 0xb a 0xd e 0xc a 0xe; o outro
/// lado (ou sem clube) leva 0xb a 0xc e 0xd a 0xe. Os outros estados ficam.
pub(crate) unsafe fn state_after_approval(oferta: usize, clube: usize) -> u8 {
    let s = read_u8(oferta + 0x2c);
    if clube != 0 && read_usize(clube) == read_usize(oferta + 8) {
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

/// records one club's approval (states 0xb to 0xe); once both agree, schedules completion for today or, with the
/// deadline still ahead, books a future transfer
/// Original: `confirm_offer_by_club` (0x8c3310, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c3310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// confirm_offer_by_club(oferta, clube)`, pela oferta passada aos clubes (0x8c2f50), pelas notícias e pela tela: a
/// aprovação do clube muda o estado (`state_after_approval`); com os dois de acordo (0xe): sem prazo (+0x30 & 0xf), a
/// oferta é agendada para hoje (0x8c6220 com 0, 1); com prazo, a data de conclusão (0x8cee70) ainda por vir faz a
/// transferência futura: uma LLIST local que não usa (0x68d020/0x68d040), o jogador (+0xc) e os da troca (lista +4)
/// saem das outras ofertas (0x8c41c0, e 0x8c47d0 para os da troca), as ofertas guardadas pelo jogador saem (0x4cf560),
/// a notícia (0x8e05d0) e a transferência futura (0x8cb9a0) com a data; data já chegada: agendada para hoje.
pub(crate) unsafe extern "thiscall" fn confirm_offer_by_club(this: usize, oferta: usize, clube: usize) {
    let data_padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut data = [0u32; 2];
    data_padrao(&mut data);
    let s = state_after_approval(oferta, clube);
    *((oferta + 0x2c) as *mut u8) = s;
    if s != 0xe {
        return;
    }
    'futura: {
        if read_u8(oferta + 0x30) & 0xf == 0 {
            break 'futura;
        }
        let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
        conclusao(this, oferta, &mut data);
        let hoje = core::ptr::read_unaligned(G_CURRENT_DATE as *const [u16; 4]);
        let d = [data[0] as u16, (data[0] >> 16) as u16];
        let antes = hoje[1] < d[1] || (hoje[1] == d[1] && (hoje[0] as i16) < (d[0] as i16));
        if !antes { // cpw2: a data de conclusão ainda por vir
            break 'futura;
        }
        crate::comparacao::ramo("cpw_2 transferencia futura");
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFERS_FOR_PLAYER);
        let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_HELD_OFFERS);
        let noticia: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_AGREED_NEWS);
        let futura: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_FUTURE_TRANSFER);
        let mut lista = [0u8; 0xe];
        let l = lista.as_mut_ptr() as usize;
        lista_nova(l);
        remove_from_other_exchanges(this, staff_records_staff_by_id(read_usize(oferta + 0xc)), oferta);
        if read_usize(oferta + 4) != 0 {
            transfer_manager_llist_rewind(read_usize(oferta + 4));
            let mut x = list_next_item(read_usize(oferta + 4));
            while x != 0 {
                remove_from_other_exchanges(this, staff_records_staff_by_id(read_usize(x)), oferta);
                desfazer(this, staff_records_staff_by_id(read_usize(x)));
                x = list_next_item(read_usize(oferta + 4));
            }
        }
        destruir(l);
        guardadas(read_usize(G_CONTRACT_MANAGER), read_usize(oferta + 0xc) as u32);
        noticia(this, oferta, &mut data);
        futura(this, oferta, &mut data);
        return;
    }
    crate::comparacao::ramo("cpw_2 agendar conclusao");
    schedule_offer_with_slices(this, oferta, 0, 1); // cpw2: a conclusão agendada para hoje
}

/// Comparador: sem chegar ao acordo dos dois (estado final diferente de 0xe), só o estado muda: lado a lado sobre a
/// oferta. Com o acordo, a oferta é agendada ou vira transferência futura (mexe nas outras ofertas e aloca): só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_confirm_offer_by_club(this: usize, oferta: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_CONFIRM_OFFER_BY_CLUB);
    if state_after_approval(oferta, clube) == 0xe {
        return o(this, oferta, clube);
    }
    let regioes = [(oferta, 0x31usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta, clube), || confirm_offer_by_club(this, oferta, clube));
    PLACAR.registrar(igual, || format!("confirm_offer_by_club(oferta {:#x}, clube {:#x}) original={:x?} rust={:x?}", oferta, clube, mo, mr));
}

/// rechecks buyer and seller; a refusal collapses or rejects the offer; with both agreeing, completes the transfer or
/// defers it to the end of the contract
/// Original: `try_complete_offer` (0x8c34e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c34e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::try_complete_offer(oferta, no_prazo)`, pela
/// oferta agendada (0x8c63a0) e pelo fim da janela (0x8c4e10). O vendedor é o clube do contrato do jogador (+0xc;
/// 0x4cf5a0, guardado em [0xae1080]; tipo 5 não conta); o comprador (+8) reconfirma (0x8cfc90, que dá o motivo): fora do
/// prazo com motivo 9, não conta; comprador do computador com resposta 1: a recusa (0x8c36b0); resposta 1 ou 2: a oferta
/// se desfaz (0x8c44e0 com o motivo, 1). O mesmo com o vendedor. Os dois de acordo: o jogador e os da troca saem das
/// outras ofertas (0x8c6ae0), as ofertas guardadas pelo jogador saem (0x4cf560), a notícia (0x8dbef0); com confirmação
/// (+0x2f), vendedor e prazo do fim do contrato (+0x30 & 0xf = 3), a transferência futura no fim do contrato (0x8cb9a0
/// com +0x2d do contrato); senão a transferência se conclui (0x8c9390).
pub(crate) unsafe extern "thiscall" fn try_complete_offer(this: usize, oferta: usize, no_prazo: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let reconfirmar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_RESTRICTIONS);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let recusa: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLUB_REJECTS_OFFER);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFER);
    let p = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let vendedor = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    let comprador = friendly_club_by_id(read_usize(oferta + 8));
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
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_DETACH_OFFER_PLAYERS);
    let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_HELD_OFFERS);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_COMPLETED_NEWS);
    soltar(this, oferta);
    guardadas(read_usize(G_CONTRACT_MANAGER), read_usize(oferta + 0xc) as u32);
    noticia(this, oferta);
    if read_u8(oferta + 0x2f) != 0 && vendedor != 0 && read_u8(oferta + 0x30) & 0xf == 3 {
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
        if c != 0 {
            crate::comparacao::ramo("cpw_2 futura no fim do contrato");
            let futura: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_FUTURE_TRANSFER);
            return futura(this, oferta, c + 0x2d);
        }
    }
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_FINALISE_TRANSFER);
    crate::comparacao::ramo("cpw_2 concluir agora");
    concluir(this, oferta); // cpw2: a transferência se conclui
}

// ponytail: conclui transferências, desfaz ofertas e dá notícias (aloca, não roda duas vezes): só a original; a cópia
// leva os ramos e se prova pela carreira e pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_try_complete_offer(this: usize, oferta: usize, no_prazo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRY_COMPLETE_OFFER)(this, oferta, no_prazo)
}

/// By the person's flags (+0x86e): pays and removes the next-sale percentage (bit 8), first-team appearance money (bit
/// 2) and international appearance money (bit 4).
///
/// [pt] As cláusulas antigas de uma pessoa pelas marcas do registro de 8 bytes em [this+0x86e]: o percentual da próxima
/// venda (bit 8: 0x8cb0f0 paga, 0x8cb4f0 tira, este da pessoa `tirar`), o prêmio por jogo no time (bit 2: 0x8cb340 com
/// −1, 2 e tira 0x8cb590) e o pela seleção (bit 4: 0x8cb340 com −1, 5 e tira 0x8cb630).
pub(crate) unsafe fn settle_old_clauses(this: usize, pessoa: usize, tirar: usize, oferta: usize) {
    let marca = |p: usize| read_u8(read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8)));
    let pagar_venda: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_NEXT_SALE_PERCENTAGE);
    let tirar_venda: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_NEXT_SALE_PERCENTAGE);
    let pagar_jogos: unsafe extern "thiscall" fn(usize, usize, i32, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_APPEARANCE_MONEY);
    let tirar_jogos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FIRST_TEAM_APPEARANCE_MONEY);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    if marca(pessoa) & 8 != 0 {
        crate::comparacao::ramo("cpw_2 write_percent_pair da venda anterior");
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

/// pays and clears the old clauses (next sale percentage, club and international appearance money) of the player and
/// the exchange players
/// Original: `settle_previous_transfer_clauses` (0x8c91e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c91e0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::settle_previous_transfer_clauses(oferta)`,
/// pela transferência concluída (0x8c9390): as cláusulas antigas do jogador (+0xc) e das pessoas da troca (lista +4,
/// cursor recomeçado; a original tira o percentual da próxima venda do jogador da oferta, não o da pessoa da troca).
pub(crate) unsafe extern "thiscall" fn settle_previous_transfer_clauses(this: usize, oferta: usize) {
    let p = staff_records_staff_by_id(read_usize(oferta + 0xc));
    settle_old_clauses(this, p, p, oferta);
    let l = read_usize(oferta + 4);
    if l == 0 {
        return;
    }
    transfer_manager_llist_rewind(l);
    let mut x = list_next_item(read_usize(oferta + 4));
    while x != 0 {
        settle_old_clauses(this, staff_records_staff_by_id(read_usize(x)), p, oferta);
        x = list_next_item(read_usize(oferta + 4));
    }
}

/// Comparador: sem marca de cláusula (bits 2, 4 e 8) no jogador nem nas pessoas da troca, só o cursor da lista muda:
/// lado a lado sobre a lista e as marcas. Com alguma, paga (finanças, notícias que alocam): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_settle_previous_transfer_clauses(this: usize, oferta: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SETTLE_PREVIOUS_TRANSFER_CLAUSES);
    let marcas = read_usize(this + 0x86e);
    let tem = |id: usize| read_u8(marcas.wrapping_add(id.wrapping_mul(8))) & 0xe != 0;
    let l = read_usize(oferta + 4);
    let mut regioes = vec![(marcas.wrapping_add(read_usize(oferta + 0xc).wrapping_mul(8)), 8usize)];
    let mut alguma = tem(read_usize(oferta + 0xc));
    if l != 0 {
        regioes.push((l, 0xe));
        // o percurso sem mexer no cursor da lista: os nós (+0 o item, +6 o próximo) a partir da cabeça +2
        let mut no = read_usize(l + 2);
        let mut n = 0;
        while no != 0 && n < 64 {
            let item = read_usize(no);
            if item != 0 && tem(read_usize(item)) {
                alguma = true;
            }
            no = read_usize(no + 6);
            n += 1;
        }
        if n >= 64 {
            alguma = true;
        }
    }
    if alguma {
        return o(this, oferta);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || settle_previous_transfer_clauses(this, oferta));
    PLACAR.registrar(igual, || format!("settle_previous_transfer_clauses(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

// ---------- 258a leva: a oferta passada aos clubes, a notícia da oferta aceita e a do empréstimo oferecido (transfer_manager.cpp) ----------






/// Person -> main contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
pub(crate) unsafe fn transfer_manager_main_contract_stored(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Contract -> club (id +4); with `filtro`, 0 for a type-5 contract; 0 for no contract.
///
/// [pt] O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
pub(crate) unsafe fn club_of_contract_filtered(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || read_u8(c + 0x35) & 0x3f != 5) {
        friendly_club_by_id(read_usize(c + 4))
    } else {
        0
    }
}

/// Rewinds a LLIST cursor (+0xa = head +2).
///
/// [pt] Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
pub(crate) unsafe fn transfer_manager_llist_rewind(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
}


/// Writes the person's id and three keys (+0x33 of +4, +8, +0xc) into news slots k..k+3.
///
/// [pt] Os dados da pessoa numa notícia, a partir da vaga `k`: o id e as três chaves (+0x33 de +4, +8 e +0xc).
pub(crate) unsafe fn write_person_news_slots(n: usize, k: i8, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    gravar(n, k, read_usize(p) as u32);
    gravar(n, k + 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, k + 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(n, k + 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// once every player agrees, cancels the rival offers (rescheduled in 20 days), waits for the work permit (18 to 24
/// days) or passes the offer to the buyer's and the seller's decision
/// Original: `advance_offer_to_club_approval` (0x8c2f50, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c2f50 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::advance_offer_to_club_approval(oferta)`:
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
pub(crate) unsafe extern "thiscall" fn advance_offer_to_club_approval(this: usize, oferta: usize) {
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    // thiscall (ecx = this, o gerente de transferências; corrigido em 31/10/2026: antes stdcall, o this ia com o ecx da vez e às vezes 0)
    let pede_permissao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_OFFER_NEEDS_WORK_PERMIT);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_WORK_PERMIT_DELAY_NEWS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_da_transferencia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_FINALISED_NEWS);
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_RESTRICTIONS);
    let recusar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLUB_REJECTS_OFFER);
    let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
    let confirmar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CONFIRM_OFFER_BY_CLUB);
    let comprador = friendly_club_by_id(read_usize(oferta + 8));
    let jogador = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let mut vendedor = club_of_contract_filtered(transfer_manager_main_contract_stored(jogador), true);
    let c = transfer_manager_main_contract_stored(jogador);
    if c != 0 && read_u8(c + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    if read_u8(oferta + 0x2c) != 0xa {
        if read_u8(oferta + 0x10) != 2 {
            return;
        }
        let troca = read_usize(oferta + 4);
        if troca != 0 {
            transfer_manager_llist_rewind(troca);
            loop {
                let x = list_next_item(troca);
                if x == 0 {
                    break;
                }
                if read_u8(x + 4) != 2 {
                    return;
                }
            }
        }
        let rivais = read_usize(oferta);
        if rivais != 0 {
            transfer_manager_llist_rewind(rivais);
            let mut x = list_next_item(rivais);
            while x != 0 {
                let outra = read_usize(read_usize(this) + read_usize(x) * 4);
                if outra != 0 && read_usize(x) != read_usize(oferta + 0x20) {
                    crate::comparacao::ramo("cpv_1 ofertas rivais canceladas");
                    write_u8(outra + 0x2c, 0xf);
                    write_u8(read_usize(read_usize(this) + read_usize(x) * 4) + 0x2e, 3);
                    agendar(this, read_usize(read_usize(this) + read_usize(x) * 4), 0x14, 0); // cpv1: as rivais em 20 dias
                    let o = read_usize(read_usize(this) + read_usize(x) * 4);
                    let p = staff_records_staff_by_id(read_usize(o + 0xc));
                    let dono = club_of_contract_filtered(transfer_manager_main_contract_stored(p), false);
                    limpar_marcas(read_usize(this + 0x856), friendly_club_by_id(read_usize(o + 8)), p);
                    let l = read_usize(o + 4);
                    if l != 0 {
                        transfer_manager_llist_rewind(l);
                        loop {
                            let y = list_next_item(l);
                            if y == 0 {
                                break;
                            }
                            limpar_marcas(read_usize(this + 0x856), dono, staff_records_staff_by_id(read_usize(y)));
                        }
                    }
                }
                x = list_next_item(rivais);
            }
        }
        if read_u8(oferta + 0x30) & 0xf == 0 && pede_permissao(this, oferta) != 0 && read_u8(oferta + 0x2e) == 0 {
            noticia_da_espera(this, oferta);
            write_u8(oferta + 0x2c, 0xa);
            let r = sorteio(7);
            crate::comparacao::ramo("cpv_1 permissao de trabalho");
            // a original soma 0x12 só no byte baixo do eax do sorteio (0..6: em cima, zero)
            let dias = (r as u32 & !0xff) | (r as u8).wrapping_add(0x12) as u32; // cpv1: a espera da permissao
            agendar(this, oferta, dias, 0);
            return;
        }
    }
    write_u8(oferta + 0x2c, 0xb);
    if humano(read_usize(G_NEWS), comprador) != 0 || (vendedor != 0 && humano(read_usize(G_NEWS), vendedor) != 0) {
        noticia_da_transferencia(this, oferta);
    }
    let mut vaga = oferta;
    // ponytail: a original passa um short* para um dword local sem valor e empurra o dword inteiro; os chamados leem a
    // palavra. A cópia começa em zero.
    let mut motivo: u32 = 0;
    if human_club(comprador, 0, 1, 0, 0) == 0 {
        if restricoes(this, oferta, comprador, &mut motivo, 1) as u8 == 1 {
            if read_u8(oferta + 0x2e) < 3 {
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
    if read_u8(o + 0x2f) != 0 || vendedor == 0 {
        return confirmar(this, o, vendedor);
    }
    if human_club(vendedor, 0, 1, 0, 0) != 0 {
        return;
    }
    if restricoes(this, vaga, vendedor, &mut motivo, 1) as u8 == 1 {
        return recusar(this, vaga, vendedor, 0);
    }
    crate::comparacao::ramo("cpv_1 confirmada pelo vendedor");
    confirmar(this, vaga, vendedor); // cpv1: o vendedor confirma
}

/// the 0xfa3 news of an accepted offer to the human clubs involved
/// Original: `post_offer_accepted_news` (0x8d76d0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d76d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_offer_accepted_news(oferta)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os destinatários
/// (0x8d1130 com 0x2d, até 30), a notícia 0xfa3 (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 = +8 (comprador),
/// 6 = +4, 7 = +0x20, 8 = +0x24, 9 = +0x1f; entregue a cada destinatário (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_offer_accepted_news(this: usize, oferta: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpv_1 noticia da oferta aceita");
    let p = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let dono = club_of_contract_filtered(transfer_manager_main_contract_stored(p), true);
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2d, clubes.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfa3, 0);
    write_person_news_slots(n, 0, p);
    gravar(n, 4, if dono != 0 { read_usize(dono) as u32 } else { u32::MAX });
    gravar(n, 5, read_usize(oferta + 8) as u32);
    gravar(n, 6, read_usize(oferta + 4) as u32);
    gravar(n, 7, read_usize(oferta + 0x20) as u32);
    gravar(n, 8, read_usize(oferta + 0x24) as u32);
    gravar(n, 9, read_u8(oferta + 0x1f) as i8 as i32 as u32);
    for &c in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(read_usize(G_NEWS), n, c);
    }
}

/// the 0xfb5 news of a loan offer to the human clubs involved; without a contract, the error box
/// Original: `post_loan_offer_news` (0x8e1780, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8e1780 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_loan_offer_news(oferta, termos)`: os destinatários (0x8d1130 com 0x2e, até 30); sem nenhum, nada. A notícia
/// 0xfb5 (local): 0..3 o jogador (+0xc), 4 = +8, 5 = o id do clube do contrato (nenhum no tipo 5: a original lê o
/// endereço 0), 6 = +0x20, 7 = +0x24, 8 = termos +0xc, 9 = +0x2c, 10 = termos +0x2d (palavra com sinal), 11 = termos
/// +0x2f (palavra); sem contrato, a caixa "Error" (0x5e4c) e nada; senão 12 = contrato +0xc e a entrega a cada um.
pub(crate) unsafe extern "thiscall" fn post_loan_offer_news(this: usize, oferta: usize, termos: usize) {
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let p = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2e, clubes.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    crate::comparacao::ramo("cpv_1 noticia do emprestimo");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfb5, 0);
    write_person_news_slots(n, 0, p);
    gravar(n, 4, read_usize(oferta + 8) as u32);
    // ponytail: sem clube (contrato de tipo 5) a original lê o endereço 0 e cai; a cópia faz o mesmo.
    let clube = club_of_contract_filtered(transfer_manager_main_contract_stored(p), true);
    gravar(n, 5, read_usize(clube) as u32);
    gravar(n, 6, read_usize(oferta + 0x20) as u32);
    gravar(n, 7, read_usize(oferta + 0x24) as u32);
    gravar(n, 8, read_usize(termos + 0xc) as u32);
    gravar(n, 9, read_u8(oferta + 0x2c) as i8 as i32 as u32);
    gravar(n, 10, core::ptr::read_unaligned((termos + 0x2d) as *const i16) as i32 as u32);
    gravar(n, 11, core::ptr::read_unaligned((termos + 0x2f) as *const u16) as u32);
    let c = transfer_manager_main_contract_stored(p);
    if c == 0 {
        return show_error_at(0xa8d514, 0x5e4c);
    }
    gravar(n, 12, read_usize(c + 0xc) as u32);
    for &d in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(read_usize(G_NEWS), n, d);
    }
}

// ponytail: reagenda, cancela rivais, sorteia e decide pelos clubes (as chamadas mexem nas ofertas e alocam): não roda
// duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_advance_offer_to_club_approval(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADVANCE_OFFER_TO_CLUB_APPROVAL)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_accepted_news(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_OFFER_ACCEPTED_NEWS)(this, oferta)
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_loan_offer_news(this: usize, oferta: usize, termos: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_LOAN_OFFER_NEWS)(this, oferta, termos)
}

// ---------- 258a leva: a negociação guardada da oferta, o boato do técnico em outro clube e a recusa do jogador em se mudar (transfer_manager.cpp, manager_manager.cpp) ----------







/// stores the offer terms in a new negotiation (the array grows by 50; the next id) and schedules the offer in 2 days,
/// or in 14 with news for a human buyer
/// Original: `add_transfer_negotiation` (0x8be9d0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8be9d0 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH só dos `new`): `TRANSFER_MANAGER::
/// add_transfer_negotiation(oferta, termos, marcar)`: com `marcar`, +0x2d = 1 nas duas e o jogador marcado sob oferta
/// (0x84b580, com a lista +0). Sem vaga (+0x28 = −1): o vetor das negociações (+0xc, capacidade +0x10, conta +0x14)
/// cresce de 50 quando cheio (0x945667 zerando as novas; 0x945501 sem vetor), a primeira vaga vazia vai para +0x28 e a
/// oferta para o status 1 (sem vaga vazia: a caixa "Error" 0x60f e nada). A negociação antiga da vaga é destruída
/// (0x8e5710, 0x944c94), ou a conta sobe; uma nova (0x41 bytes, 0x944cf1 e 0x8e5680) recebe os termos campo a campo
/// (a data +0x12 por 0x417c10; de +0x30 e +0x31 só os 4 bits baixos), +0x20 da oferta e o id seguinte (+0x85a); a
/// lista +4 dos termos é copiada numa nova (0xe bytes, 0x68d020, 0x68d2f0). Comprador (termos +8) humano: a notícia da
/// negociação (0x8e4610 no empréstimo +0x1f = 1, senão 0x8d6bf0) e a oferta agendada em 14 dias; senão em 2 (0x8c6220).
pub(crate) unsafe extern "thiscall" fn add_transfer_negotiation(this: usize, oferta: usize, termos: usize, marcar: u32) {
    let mark_player_under_offer: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_MARK_PLAYER_UNDER_OFFER);
    let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let alocar_zerado: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let nova_lista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let copiar_lista: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_COPY_FROM);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_do_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_NEGOTIATION_NEWS);
    let noticia_da_negociacao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_NEGOTIATION_NEWS);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    if marcar != 0 {
        write_u8(oferta + 0x2d, 1);
        write_u8(termos + 0x2d, 1);
        mark_player_under_offer(read_usize(this + 0x856), staff_records_staff_by_id(read_usize(oferta + 0xc)), friendly_club_by_id(read_usize(oferta + 8)), read_usize(oferta));
    }
    let mut k = read_usize(oferta + 0x28);
    if k == usize::MAX {
        if read_usize(this + 0x14) == read_usize(this + 0x10) {
            let v = read_usize(this + 0xc);
            let cap = read_usize(this + 0x10).wrapping_add(0x32);
            write_usize(this + 0x10, cap);
            if v != 0 {
                let nv = realocar(v, cap * 4);
                write_usize(this + 0xc, nv);
                core::ptr::write_bytes((nv + read_usize(this + 0x10) * 4 - 0xc8) as *mut u32, 0, 0x32);
            } else {
                write_usize(this + 0xc, alocar_zerado(cap, 4));
            }
        }
        let cap = read_usize(this + 0x10) as i32;
        let v = read_usize(this + 0xc);
        match (0..cap.max(0) as usize).find(|&j| read_usize(v + 4 * j) == 0) {
            Some(j) => k = j,
            None => return show_error_at(0xa8d514, 0x60f),
        }
        crate::comparacao::ramo("cpv_4 vaga nova");
        write_usize(oferta + 0x28, k);
        write_u8(oferta + 0x2c, 1); // cpv4: a oferta em negociação
    }
    let antiga = read_usize(read_usize(this + 0xc) + k * 4);
    if antiga != 0 {
        destruir(antiga);
        liberar(antiga);
    } else {
        write_usize(this + 0x14, read_usize(this + 0x14).wrapping_add(1));
    }
    let mut nova = novo(0x41);
    nova = if nova != 0 { construir(nova) } else { 0 };
    write_usize(read_usize(this + 0xc) + k * 4, nova);
    let neg = read_usize(read_usize(this + 0xc) + k * 4);
    write_usize(neg, read_usize(termos));
    write_usize(neg + 4, read_usize(termos + 4));
    write_usize(neg + 8, read_usize(termos + 8));
    write_usize(neg + 0xc, read_usize(termos + 0xc));
    write_u8(neg + 0x10, read_u8(termos + 0x10));
    let b11 = read_u8(termos + 0x11);
    let mut temporaria = [0u32; 2];
    write_u8(neg + 0x11, b11);
    atribuir_data(neg + 0x12, temporaria.as_mut_ptr() as usize, termos + 0x12);
    write_u8(neg + 0x1a, read_u8(termos + 0x1a));
    write_usize(neg + 0x1b, read_usize(termos + 0x1b)); // cpv4: o preço dos termos
    write_u8(neg + 0x1f, read_u8(termos + 0x1f));
    write_usize(neg + 0x20, read_usize(termos + 0x20));
    write_usize(neg + 0x24, read_usize(termos + 0x24));
    write_usize(neg + 0x28, read_usize(termos + 0x28));
    for o in 0x2c..0x30 {
        write_u8(neg + o, read_u8(termos + o));
    }
    write_u8(neg + 0x30, (read_u8(neg + 0x30) & 0xf0) | (read_u8(termos + 0x30) & 0xf));
    write_usize(neg + 0x31, (read_usize(neg + 0x31) & !0xf) | (read_usize(termos + 0x31) & 0xf));
    write_usize(neg + 0x35, read_usize(termos + 0x35));
    write_u8(neg + 0x39, read_u8(termos + 0x39));
    write_usize(neg + 0x3a, read_usize(termos + 0x3a));
    write_u8(neg + 0x3e, read_u8(termos + 0x3e));
    write_u8(neg + 0x3f, read_u8(termos + 0x3f));
    write_u8(neg + 0x40, read_u8(termos + 0x40));
    let neg = || read_usize(read_usize(this + 0xc) + k * 4);
    write_usize(neg() + 0x20, read_usize(oferta + 0x20));
    write_usize(neg() + 0x24, read_usize(this + 0x85a));
    write_usize(this + 0x85a, read_usize(this + 0x85a).wrapping_add(1));
    if read_usize(termos + 4) != 0 {
        let mut l = novo(0xe);
        l = if l != 0 { nova_lista(l) } else { 0 };
        write_usize(neg() + 4, l);
        copiar_lista(read_usize(neg() + 4), read_usize(termos + 4));
    }
    if humano(read_usize(G_NEWS), friendly_club_by_id(read_usize(termos + 8))) != 0 {
        if read_u8(oferta + 0x1f) == 1 {
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


/// the 0xfae news of the player refusing the move, with the reason and both clubs' managers
/// Original: `post_player_rejects_move_news` (0x8d8310, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d8310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_player_rejects_move_news(oferta, motivo)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os
/// destinatários (0x8d1130 com 0x2f, até 30), a notícia 0xfae (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 =
/// +8, 6 = +0x1f, 7 = o motivo, 8 = o técnico do dono, 9 = o técnico do comprador (+8 > −1); a cada destinatário.
pub(crate) unsafe extern "thiscall" fn post_player_rejects_move_news(this: usize, oferta: usize, motivo: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpv_4 recusa em se mudar");
    let p = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let dono = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    let mut clubes = [0usize; 30];
    let q = destinatarios(this, oferta, 0x2f, clubes.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xfae, 0);
    gravar(n, 0, read_usize(p) as u32);
    gravar(n, 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(n, 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    gravar(n, 4, if dono != 0 { read_usize(dono) as u32 } else { u32::MAX });
    gravar(n, 5, read_usize(oferta + 8) as u32);
    gravar(n, 6, read_u8(oferta + 0x1f) as i8 as i32 as u32);
    gravar(n, 7, motivo);
    if dono != 0 {
        let t = read_usize(dono + 0xcf);
        if t != 0 {
            gravar(n, 8, read_usize(t) as u32);
        }
    }
    let comprador = read_usize(oferta + 8) as i32;
    if comprador > -1 {
        let t = read_usize(friendly_club_by_id(comprador as usize) + 0xcf);
        if t != 0 {
            gravar(n, 9, read_usize(t) as u32);
        }
    }
    for &d in clubes.iter().take(if q > 0 { q as usize } else { 0 }) {
        entregar(read_usize(G_NEWS), n, d);
    }
}

// ponytail: aloca e libera negociações e listas, agenda a oferta e manda notícias: não roda duas vezes. Só a original;
// a cópia leva os ramos e se prova pela carreira e pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_negotiation(this: usize, oferta: usize, termos: usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_ADD_TRANSFER_NEGOTIATION)(this, oferta, termos, marcar)
}

// ponytail: sorteia e a notícia vai às caixas (aloca): só a original; a cópia leva o ramo.

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_player_rejects_move_news(this: usize, oferta: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_PLAYER_REJECTS_MOVE_NEWS)(this, oferta, motivo)
}

// ---------- 255a leva: as permissões de trabalho do começo do jogo, a contagem da cláusula e o prêmio por jogo do jogador (transfer_manager.cpp) ----------




/// marks 0x3fff/0x1fff by international caps; for a player needing a permit (0x8c7560), by the earliest year in the
/// club nation (career 0x7c3e40): under 5 years, the permit (0x8c7070); otherwise the second nationality (+0x1e); sorts
/// the permits
/// Original: `TRANSFER_MANAGER::init_work_permits` (0x8c6e60, transfer_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x8c6e60 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH só da data local): `TRANSFER_MANAGER::
/// init_work_permits()`: pela criação do jogo (0x8bba30). Para cada pessoa: com mais de 13 jogos pela seleção (+0x22),
/// 0x3fff na palavra +3 do registro de 8 bytes ([this+0x86e] + i × 8); com mais de 5, 0x1fff. Jogador (+0x3d = 0xb) de
/// clube (+0x39) que precisa de permissão de trabalho (0x8c7560 sem ignorar a dada): o ano mais antigo na nação do
/// clube pela carreira (0x7c3e40, um grupo de 6 entradas de 0x2b bytes por temporada; para no primeiro clube de outra
/// nação), começando pelo ano de +0x40. Menos de 5 anos até o ano de hoje ([0xae2c92]): a permissão (0x8c7070, sem
/// ordenar; antes a original leva a data +0x3e de 52 em 52 semanas até a data [0xdd6ec8], sem usar o resultado); senão a
/// segunda nacionalidade (+0x1e) vira a nação do clube. No fim ordena as permissões (qsort 0x945333 com 0x8e55c0).
pub(crate) unsafe extern "thiscall" fn init_work_permits(this: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let carreira: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(FN_PLAYERSTATS_BUILD_CAREER_HISTORY);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_WORK_PERMIT);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let mut d = [0u8; 8];
    data_padrao(d.as_mut_ptr() as usize);
    let mut i = 0usize;
    let mut p = read_usize(G_STAFF_TABLE);
    while (i as i32) < read_usize(G_STAFF_COUNT) as i32 {
        let jogos = read_u8(p + 0x22);
        let marca = (read_usize(this + 0x86e).wrapping_add(i.wrapping_mul(8)) + 3) as *mut u16;
        if jogos > 0xd {
            core::ptr::write_unaligned(marca, 0x3fff);
        } else if jogos > 5 {
            core::ptr::write_unaligned(marca, 0x1fff);
        }
        let clube = read_usize(p + 0x39);
        if clube != 0 && read_u8(p + 0x3d) == 0xb && precisa(this, p, clube, 0) != 0 {
            let mut ano = read_u16(p + 0x40);
            let mut conta = 0i32;
            let t = carreira(read_usize(G_PLAYER_STATS), p, &mut conta);
            let (mut k, mut e) = (0i32, t + 2);
            while k < conta {
                let id = read_usize(e) as i32;
                if id >= 0 && id < read_usize(G_CLUB_COUNT) as i32 {
                    let c = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(0x245));
                    if read_usize(c + 0x53) != read_usize(read_usize(p + 0x39) + 0x53) {
                        break; // clube de outra nação
                    }
                    let a = read_u16(e - 2);
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
            if (read_u16(G_CURRENT_DATE_YEAR) as i32) - (ano as i32) < 5 { // cps2: menos de cinco anos na nação
                // a data +0x3e levada de 52 em 52 semanas até a data [0xdd6ec8] (a original não usa o resultado)
                let (mut t1, mut t2, mut t3) = ([0u8; 8], [0u8; 8], [0u8; 8]);
                let dp = d.as_mut_ptr() as usize;
                copiar(dp, t1.as_mut_ptr() as usize, p + 0x3e);
                loop {
                    let (ah, dh) = (read_u16(G_GAME_START_DATE_YEAR), read_u16(dp + 2));
                    if !(ah > dh || (ah == dh && (read_u16(G_GAME_START_DATE) as i16) > (read_u16(dp) as i16))) {
                        break;
                    }
                    let r = mais_semanas(dp, t2.as_mut_ptr() as usize, 0x34);
                    copiar(dp, t3.as_mut_ptr() as usize, r);
                }
                crate::comparacao::ramo("cps_2 permissao pela carreira");
                // ponytail: a original empurra o registrador do ano com os 16 bits altos de um ponteiro; a 0x8c7070
                // só grava a palavra baixa
                dar(this, p, read_usize(read_usize(p + 0x39) + 0x53), ano as u32, 1, 0);
            } else {
                crate::comparacao::ramo("cps_2 segunda nacionalidade");
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, read_usize(read_usize(p + 0x39) + 0x53)); // cps2: a segunda nacionalidade
            }
        }
        i += 1;
        p += 0x6e;
    }
    qsort(read_usize(this + 0x93a), read_usize(this + 0x93e) as u32, 0xe, 0x8e55c0);
}

/// type 4: international caps since the deal (+0x22 minus byte +7 of the person record); otherwise club appearances
/// (+0x3b of the contract)
/// Original: `TRANSFER_MANAGER::get_clause_count` (0x8c3e50, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c3e50 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_clause_count(pessoa, tipo)`: no tipo 4,
/// os jogos pela seleção (+0x22) menos os da época do acordo (byte +7 do registro de 8 bytes [this+0x86e] + id × 8);
/// senão os jogos pelo clube (+0x3b) do contrato (0x4cf5a0, pedido duas vezes como na original), ou 0 sem contrato.
pub(crate) unsafe extern "thiscall" fn get_clause_count(this: usize, pessoa: usize, tipo: u32) -> u16 {
    if tipo as u8 == 4 {
        let r = read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8));
        return (read_u8(pessoa + staff::INT_APPS) as u16).wrapping_sub(read_u8(r + 7) as u16);
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    if contract_of(read_usize(G_CONTRACT_MANAGER), pessoa) == 0 {
        return 0;
    }
    read_u16(contract_of(read_usize(G_CONTRACT_MANAGER), pessoa) + 0x3b)
}

/// with the person bit 2, searches the appearance money array +0x88c; when not found, clears the bit
/// Original: `TRANSFER_MANAGER::find_first_team_appearance_money` (0x8d0ea0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0ea0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_first_team_appearance_money(pessoa)`:
/// com o bit 2 do registro de 8 bytes ([this+0x86e] + id × 8), busca o prêmio por jogo da pessoa no vetor [this+0x88c]
/// (conta +0x894; busca binária 0x946464 com 0x8b4e00, registros de 13 bytes) e o devolve; sem achar, tira o bit (soma
/// 0xfe ao byte) e devolve 0.
pub(crate) unsafe extern "thiscall" fn find_first_team_appearance_money(this: usize, pessoa: usize) -> usize {
    let id = read_usize(pessoa);
    if read_u8(read_usize(this + 0x86e).wrapping_add(id.wrapping_mul(8))) & 2 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let chave = id as i32;
    let r = bsearch(&chave, read_usize(this + 0x88c), read_usize(this + 0x894), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8)) as *mut u8;
    *m = (*m).wrapping_add(0xfe);
    0
}

// ponytail: cresce o vetor das permissões (0x8c7070) e grava em todas as pessoas: só a original; a cópia leva os ramos
// (roda uma vez, na criação do jogo).
pub(crate) unsafe extern "thiscall" fn comparar_init_work_permits(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_WORK_PERMITS)(this)
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_clause_count(this: usize, pessoa: usize, tipo: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(IDX_GET_CLAUSE_COUNT)(this, pessoa, tipo);
    let n = get_clause_count(this, pessoa, tipo);
    PLACAR.registrar(r == n, || format!("get_clause_count(pessoa {}, tipo {}) original={} rust={}", read_usize(pessoa), tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelo byte do registro da pessoa, e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_first_team_appearance_money(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_FIRST_TEAM_APPEARANCE_MONEY);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, pessoa), || find_first_team_appearance_money(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("find_first_team_appearance_money(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(pessoa), r, n, mo, mr));
    r
}

// ---------- 255a leva: a fila da transferência livre, a próxima janela aberta e o empréstimo negociado (transfer_manager.cpp) ----------





/// for each queued player (+0x92c): at a human club the board may block it (0x696940); a draw adds the manager to the
/// disliked staff (0x547af0); the records (0x86d800), the cancelled offers (0x8df9b0) and the release with the
/// compensation (0x8c1af0); clears the queue
/// Original: `TRANSFER_MANAGER::process_free_transfer_queue` (0x8c18c0, transfer_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x8c18c0 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH das duas LLIST locais 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::process_free_transfer_queue()`: pelo dia a dia das transferências (0x8c5880). Percorre uma cópia
/// do cabeçalho da fila (+0x92c, 14 bytes, com o cursor no começo); para cada item (+0 id da pessoa, +4 motivo, +5
/// compensação) com contrato (0x4cf5a0, guardado em [0xae1080]) e clube: no clube humano (0x7875e0) sem motivo, a
/// diretoria pode barrar (0x696940; barrando, 0x6a0990 e o próximo); senão o clube entra nos rejeitados (0x547e70) e o
/// técnico (+0xcf) nos desafetos (0x547af0), e a diretoria registra (0x697430). Sem motivo, um sorteio pelo +0x5b contra
/// outro de 20 (ou gostar do clube, 0x547d60) põe o técnico nos desafetos. Depois os registros da liberação (0x86d800),
/// as ofertas canceladas (0x8df9b0) e a liberação (0x8c1af0 com motivo e compensação). No fim a cópia vira a lista
/// vazia, a fila é limpa (0x68d080) e as duas locais são destruídas.
pub(crate) unsafe extern "thiscall" fn process_free_transfer_queue(this: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let barra: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_CHECK_BOARD_BLOCKS_RELEASE);
    let barrado: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BOARD_RELEASE_REACTION_NEWS);
    let rejeitado: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_CLUB);
    let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let diretoria: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_APPLY_RELEASE_CONFIDENCE_PENALTY);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let registros: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FREE_RELEASE_RECORDS);
    let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CANCEL_OFFERS_FOR_RELEASED_PLAYER);
    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, i32, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_FREE_TRANSFER_STAFF);
    let (mut l1, mut l2) = ([0u8; 14], [0u8; 14]);
    let (a, b) = (l1.as_mut_ptr() as usize, l2.as_mut_ptr() as usize);
    nova(a);
    nova(b);
    let fila = this + 0x92c;
    // a cópia do cabeçalho: os 10 primeiros bytes e o cursor (+0xa) no começo (+2)
    core::ptr::copy_nonoverlapping(fila as *const u8, a as *mut u8, 0xa);
    write_usize(a + 0xa, read_usize(a + 2));
    let mut e = proximo(a);
    while e != 0 {
        let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e).wrapping_mul(0x6e));
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
        write_usize(0xae1080, c);
        let clube = if c != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(0x245)) } else { 0 };
        if clube != 0 {
            let mut seguir = true;
            if humano(read_usize(G_NEWS), clube) != 0 && read_u8(e + 4) == 0 {
                let r = barra(read_usize(G_MANAGER_MANAGER), p, clube);
                if r as u16 != 0 {
                    barrado(read_usize(G_MANAGER_MANAGER), p, clube, r);
                    seguir = false;
                } else {
                    if read_u8(e + 4) == 0 {
                        rejeitado(p, clube, 0);
                        if read_usize(clube + club::MANAGER) != 0 {
                            desafeto(p, read_usize(clube + club::MANAGER), 1);
                        }
                    }
                    diretoria(read_usize(G_MANAGER_MANAGER), p, clube);
                }
            }
            if seguir {
                if read_u8(e + 4) == 0 {
                    let r = sorteio(read_u8(p + 0x5b) as i8 as i32);
                    if r < sorteio(0x14) || gosta(p, clube) != 0 { // cps3: o sorteio do desafeto
                        crate::comparacao::ramo("cps_3 desafeto do tecnico");
                        desafeto(p, read_usize(clube + club::MANAGER), 1);
                    }
                }
                crate::comparacao::ramo("cps_3 liberado");
                registros(read_usize(G_STAFF_RECORDS), p); // cps3: os registros da liberação
                cancelar(this, p);
                let comp = read_usize(e + 5); // cps3: a compensação ao clube
                if comp != 0 {
                    crate::comparacao::ramo("cps_3 compensacao");
                }
                liberar(this, p, clube, read_u8(e + 4) as i8 as i32, comp, 0);
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

/// the first day, from today up to a year later, with the club nation window open (0x8e8e50; without a rule, today);
/// when none, the "Error" box
/// Original: `TRANSFER_MANAGER::find_next_window_open_date` (0x8cf470, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf470 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só das datas locais): `TRANSFER_MANAGER::
/// find_next_window_open_date(clube, saida)`: a regra da nação do clube (+0x53) entre as 28 de +0x8ac (+0xa); de hoje
/// ([0xae2c90]) até um ano depois (0x54c5b0 com 365), o primeiro dia com a janela aberta (0x8e8e50; sem regra, hoje) vai
/// para `saida`; sem achar, a caixa "Error" 0x32c6. A contratação desejada (0x85fc50) regrava a data com hoje logo depois.
pub(crate) unsafe extern "thiscall" fn find_next_window_open_date(this: usize, clube: usize, saida: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let aberta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_FIND_OPEN_WINDOW);
    let (mut d, mut lim, mut t1, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (dp, lp, p1, p2) = (d.as_mut_ptr() as usize, lim.as_mut_ptr() as usize, t1.as_mut_ptr() as usize, t2.as_mut_ptr() as usize);
    data_padrao(dp);
    mais(0xae2c90, lp, 0x16d);
    let nacao = read_usize(clube + club::NATION);
    let mut regra = 0usize;
    for k in 0..28 {
        let r = read_usize(this + 0x8ac + k * 4);
        if read_usize(r + 0xa) == nacao {
            regra = r;
            break;
        }
    }
    copiar(dp, p1, 0xae2c90);
    loop {
        let (la, da) = (read_u16(lp + 2), read_u16(dp + 2));
        if !(la > da || (la == da && (read_u16(lp) as i16) > (read_u16(dp) as i16))) {
            return show_error_at(0xa8d514, 0x32c6);
        }
        if regra == 0 || aberta(regra, dp, 0) != 0 {
            copiar(saida, p2, dp);
            return;
        }
        let r = mais(dp, p1, 1);
        copiar(dp, p2, r);
    }
}

/// the +0x946 list item with the person and the club, or 0
/// Original: `TRANSFER_MANAGER::find_negotiated_loan_contract` (0x8c72d0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c72d0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_negotiated_loan_contract(pessoa,
/// clube)`: na lista [this+0x946] (o cursor +0xa volta ao começo), o item com a pessoa (+0) e o clube (+4) pelos ids;
/// sem lista ou sem achar, 0.
pub(crate) unsafe extern "thiscall" fn find_negotiated_loan_contract(this: usize, pessoa: usize, clube: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(this + 0x946);
    if l == 0 {
        return 0;
    }
    write_usize(l + 0xa, read_usize(l + 2));
    let mut e = proximo(read_usize(this + 0x946));
    while e != 0 {
        if read_usize(pessoa) == read_usize(e) && read_usize(clube) == read_usize(e + 4) {
            return e;
        }
        e = proximo(read_usize(this + 0x946));
    }
    0
}

// ponytail: sorteia (0x90cfc0) e libera os jogadores (0x8c1af0, 0x8df9b0), limpando a fila: só a original; a cópia
// leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_process_free_transfer_queue(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_FREE_TRANSFER_QUEUE)(this)
}

/// Comparador: lado a lado pela data de saída (a data estática da 0x54c5b0 é rascunho). Sem janela num ano, a caixa
/// sairia duas vezes (não acontece com as regras do jogo).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_window_open_date(this: usize, clube: usize, saida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_FIND_NEXT_WINDOW_OPEN_DATE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 8)], || o(this, clube, saida), || find_next_window_open_date(this, clube, saida));
    PLACAR.registrar(igual, || format!("find_next_window_open_date(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparador: pelo valor (as duas voltas deixam o cursor da lista no mesmo lugar).
pub(crate) unsafe extern "thiscall" fn comparar_find_negotiated_loan_contract(this: usize, pessoa: usize, clube: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_FIND_NEGOTIATED_LOAN_CONTRACT)(this, pessoa, clube);
    let n = find_negotiated_loan_contract(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("find_negotiated_loan_contract(pessoa {}, clube {}) original={:x} rust={:x}", read_usize(pessoa), read_usize(clube), r, n));
    r
}

// ---------- 255a leva: a contraproposta com cláusula e os motivos da recusa do lance (transfer_manager.cpp) ----------





/// Offer value (+0x1b) x f, truncated with ftol.
///
/// [pt] O valor da oferta (+0x1b) × `f`, truncado (_ftol 0x945640; o x87 em precisão dupla).
pub(crate) unsafe fn offer_value_times(o: usize, f: f64) -> usize {
    match_pl_ftol(read_usize(o + 0x1b) as i32 as f64 * f) as usize
}

/// Person -> club of their contract (stored at 0xae1080), or 0.
///
/// [pt] O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]), ou 0.
pub(crate) unsafe fn current_contract_club(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
    write_usize(0xae1080, c);
    if c == 0 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
    }
}

/// with a much bigger bidding club and a high-potential player up to 30: a new offer (0x8e5680, 0x8e78f0) with a 20%
/// sell-on (appeal 80), international (70) or first-team (60) appearance money, and the fee × 0.8; none, frees the
/// offer and 0
/// Original: `TRANSFER_MANAGER::build_clause_offer_option` (0x8c8fd0, transfer_manager.cpp, thiscall (this sem uso), ret 0x14).
///
/// [pt] { 0x8c8fd0 } (thiscall com o `this` sem uso, ret 0x14, transfer_manager.cpp, quadro SEH do `new`):
/// `TRANSFER_MANAGER::build_clause_offer_option(clube, jogador, saida, apelo, valor)`: pela resposta do clube do computador
/// ao lance (0x8c8090). Com o clube do lance mais de 3 acima na reputação (+0x80) do clube atual (0x4cf5a0), ficha de
/// jogador (+0x61) com potencial (+7) ao menos 25 acima da habilidade (+5) e até 30 anos (+0x18): uma oferta nova
/// (0x944cf1 de 0x41 bytes, 0x8e5680) em `*saida`, iniciada com o valor (0x8e78f0), e apelo 50. Clube do lance abaixo de
/// 15 com a reputação da ficha (+0xb / 500) mais de 2 acima: 20% da próxima venda (+0x3f), o valor × 0,8 e apelo 80.
/// Senão, jogador sem jogos pela seleção (+0x22) de nação (+0x1a) acima de 5000 (+0x8e): o prêmio por jogo na seleção
/// (+0x3a, valor × 0,3, por 10 jogos +0x3e), o valor × 0,8 e apelo 70. Senão, reputação da ficha mais de 3 abaixo da do
/// clube: o prêmio por jogo no time (+0x35, valor × 0,3, por 20 jogos +0x39), o valor × 0,8 e apelo 60. Nenhum: a
/// oferta é destruída (0x8e5710, 0x944c94; `*saida` fica com o ponteiro, como na original) e 0.
pub(crate) unsafe extern "thiscall" fn build_clause_offer_option(_this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    let atual = current_contract_club(jogador);
    if (read_u16(clube + club::REPUTATION) as i16 as i32) - (read_u16(atual + 0x80) as i16 as i32) <= 3 {
        return 0;
    }
    let ficha = read_usize(jogador + 0x61);
    if ficha == 0 || (read_u16(ficha + 7) as i16 as i32) - (read_u16(ficha + 5) as i16 as i32) < 0x19 || read_u8(jogador + 0x18) as i8 > 0x1e {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_TRANSFER_OFFER);
    let m = novo(0x41);
    let o = if m != 0 { construir(m) } else { 0 };
    *saida = o;
    iniciar(o, jogador, clube, valor);
    *apelo = 0x32;
    let rep = read_u16(clube + club::REPUTATION) as i16;
    // a reputação da ficha (+0xb) / 500, com o sinal (a multiplicação mágica da original)
    let fama = || (read_u16(read_usize(jogador + 0x61) + 0xb) as i16 as i32) / 500;
    if rep < 0xf && fama() - (rep as i32) > 2 {
        crate::comparacao::ramo("cps_4 write_percent_pair da venda");
        let o = *saida;
        write_usize(o + 0x1b, offer_value_times(o, 0.8)); // cps4: o valor do percentual
        *((*saida + 0x3f) as *mut u8) = 0x14;
        *apelo = 0x50;
        return 1;
    }
    let nacao = read_usize(jogador + 0x1a);
    if nacao != 0 && read_u16(nacao + nation::REPUTATION) as i16 > 0x1388 && read_u8(jogador + 0x22) == 0 {
        crate::comparacao::ramo("cps_4 premio na selecao");
        let o = *saida;
        write_usize(o + 0x3a, offer_value_times(o, 0.3));
        *((*saida + 0x3e) as *mut u8) = 0xa;
        let o = *saida;
        write_usize(o + 0x1b, offer_value_times(o, 0.8)); // cps4: o valor do prêmio na seleção
        *apelo = 0x46;
        return 1;
    }
    if fama() - (rep as i32) < -3 {
        crate::comparacao::ramo("cps_4 premio no time");
        let o = *saida;
        write_usize(o + 0x35, offer_value_times(o, 0.3));
        *((*saida + 0x39) as *mut u8) = 0x14;
        let o = *saida;
        write_usize(o + 0x1b, offer_value_times(o, 0.8)); // cps4: o valor do prêmio no time
        *apelo = 0x3c;
        return 1;
    }
    let o = *saida;
    if o != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destruir(o);
        liberar(o);
    }
    0
}

/// 0 when the reason does not fit: hot prospect (0x2000) aged 24 or more, same division (0x1000), rival (0x800) without
/// a fixture or link; otherwise 1
/// Original: `transfer_manager_is_rejection_reason_valid` (0x8c5560, transfer_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x8c5560 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_is_rejection_reason_valid(motivo, oferta)`: 0
/// quando o motivo não cabe na oferta (pessoa +0xc, clube do lance +8): 0x2000 (promessa) com 24 anos ou mais (+0x18);
/// 0x1000 com o clube atual (0x4cf5a0) e o do lance em divisões diferentes (+0x57); 0x800 (rival) com os dois clubes,
/// sem os dois humanos (0x600a50), sem confronto na liga (0x4b02e0) e sem um ser parceiro (+0xb3/+0xb7/+0xbb) do outro.
/// Senão 1.
pub(crate) unsafe extern "stdcall" fn transfer_manager_is_rejection_reason_valid(motivo: u32, oferta: usize) -> u32 {
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(0x6e));
    let atual = current_contract_club(pessoa);
    let lance = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(oferta + 8).wrapping_mul(0x245));
    let r = match motivo as u16 {
        0x2000 => (read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8) < 0x18,
        0x1000 => atual == 0 || read_usize(atual + 0x57) == read_usize(lance + 0x57),
        0x800 => {
            if lance == 0 || atual == 0 {
                return 1;
            }
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
            if humano(atual, 0, 1, 0, 0) != 0 && humano(lance, 0, 1, 0, 0) != 0 {
                return 1;
            }
            confronto(lance, atual) != 0
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| read_usize(lance + k) == atual)
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| read_usize(atual + k) == lance)
        }
        _ => true,
    };
    r as u32
}

/// by the squad status of the current club contract: 1 → 0x200, 2 and 3 → 0x400, 5 → 0x2000; otherwise 0
/// Original: `transfer_manager_get_squad_status_reason` (0x8c54c0, transfer_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x8c54c0 } (stdcall, ret 4, transfer_manager.cpp): `transfer_manager_get_squad_status_reason(oferta)`: o motivo de
/// recusa pelo status no elenco (+0x4f >> 4) do contrato da pessoa (+0xc) com o clube atual (0x4cf5f0): 1 → 0x200, 2 e
/// 3 → 0x400, 5 → 0x2000; senão 0.
pub(crate) unsafe extern "stdcall" fn transfer_manager_get_squad_status_reason(oferta: usize) -> u16 {
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(0x6e));
    let atual = current_contract_club(pessoa);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = com_clube(read_usize(G_CONTRACT_MANAGER), pessoa, atual);
    if c == 0 {
        return 0;
    }
    match read_u8(c + 0x4f) >> 4 {
        1 => 0x200,
        2 | 3 => 0x400,
        5 => 0x2000,
        _ => 0,
    }
}

// ponytail: aloca e solta a oferta (0x944cf1, 0x944c94): só a original; a cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_build_clause_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, u32) -> u32>(IDX_BUILD_CLAUSE_OFFER_OPTION)(this, clube, jogador, saida, apelo, valor)
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_is_rejection_reason_valid(motivo: u32, oferta: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(IDX_TRANSFER_MANAGER_IS_REJECTION_REASON_VALID);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(motivo, oferta), || transfer_manager_is_rejection_reason_valid(motivo, oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_is_rejection_reason_valid({:x}, pessoa {}) original={} rust={} {:x?} x {:x?}", motivo as u16, read_usize(oferta + 0xc), r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_get_squad_status_reason(oferta: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(IDX_TRANSFER_MANAGER_GET_SQUAD_STATUS_REASON);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(oferta), || transfer_manager_get_squad_status_reason(oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_get_squad_status_reason(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(oferta + 0xc), r, n, mo, mr));
    r
}

// ---------- 254a leva: os limites da escalação: a tabela de restrições da partida, o limite do banco do dia e quem precisa de permissão de trabalho (transfer_manager.cpp) ----------



/// Fills the 42-byte restriction table with 0xff.
///
/// [pt] Os 42 bytes da tabela de restrições com 0xff (o `rep stosd` de 10 duplas e o `stosw` da original).
pub(crate) unsafe fn clear_restriction_table(t: *mut u8) {
    core::ptr::write_bytes(t, 0xff, 42);
}

/// fills the 42-byte table with 0xff and, by the fixture rules scope (0x8d2fa0), lets the nation or continent rule fill
/// it (virtual +0x1c); when asked, keeps only the first limit (+5/+3, +2/+0 or +0x16/+0x17)
/// Original: `TRANSFER_MANAGER::fill_squad_restrictions` (0x8cf6b0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8cf6b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::fill_squad_restrictions(partida, t,
/// so_a_principal)`: enche os 42 bytes de `t` com 0xff. Com partida: o alcance das regras (0x8d2fa0); 0xff para aí. Fora
/// do tipo 4, a primeira das 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo
/// (+0x12) enche a tabela (virtual +0x1c com a partida e `t`). Com `so_a_principal`, fica só o primeiro limite que
/// existe: +5 ou +3 (gravado em +5), +2 ou +0 (gravado em +2), ou +0x16 com +0x17; o resto volta a 0xff.
pub(crate) unsafe extern "thiscall" fn fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    clear_restriction_table(t);
    if partida == 0 {
        return;
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FIXTURE_RULES_SCOPE);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff {
        return;
    }
    if tipo != 4 {
        for k in 0..28 {
            let r = read_usize(this + 0x8ac + k * 4);
            let casa = if tipo == 2 {
                read_usize(r + 0xe) == continente && read_u8(r + 0x12) == 2
            } else {
                read_usize(r + 0xa) == nacao && read_u8(r + 0x12) == tipo
            };
            if casa {
                let encher: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(read_usize(read_usize(r) + 0x1c));
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
        crate::comparacao::ramo("cpr_1 limit de nao comunitarios");
        clear_restriction_table(t);
        *t.add(5) = v; // o limite de não comunitários
        return;
    }
    let v = if *t.add(2) != 0xff { *t.add(2) } else { *t };
    if v != 0xff {
        clear_restriction_table(t);
        *t.add(2) = v;
        return;
    }
    let (idade, n) = (*t.add(0x16), *t.add(0x17));
    clear_restriction_table(t);
    if idade != 0xff {
        *t.add(0x16) = idade;
        *t.add(0x17) = n;
    }
}

/// counts, over the matchday squad slots (given players or the tactics 0x8968f0), the non-EU, foreign or over-age
/// players of the fixture limit (0x8cf7e0); when over, 1 and the message
/// Original: `transfer_manager_check_matchday_squad_limit` (0x8cf880, transfer_manager.cpp, thiscall (this em ecx), ret 0x18).
///
/// [pt] { 0x8cf880 } (thiscall pelo `this` em ecx, que a original passa direto à 0x8cf7e0; ret 0x18, transfer_manager.cpp):
/// `transfer_manager_check_matchday_squad_limit(clube, partida, msg, jogadores, _, saida)`: o limite da partida
/// (0x8cf7e0 em 6 bytes locais: +0 tipo, +1 idade, +5 quantidade); sem tipo, 0. O lado humano (+0x1c/+0x40 ou
/// +0x20/+0x41) vai para a 0x8968f0. Pelas (+0x4d & 0xf) + 11 vagas, a pessoa da vaga (`jogadores[k]` ou a tática
/// [0xde1d08] 0x8968f0) conta se é estrangeira na nação do clube (0x53d980 com 1; tipo 1, também fora da UE, 0x53d740)
/// ou passa da idade (+0x18, tipo 3): +2 (titulares) ou +3 (as outras) e +4 (todas). A conta vai a `saida`; passando da
/// quantidade, 1 e, com `msg`, o aviso (0x66f4e0) pelo tipo, pela conta e pela quantidade.
pub(crate) unsafe extern "thiscall" fn transfer_manager_check_matchday_squad_limit(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, _nao_usado: u32, saida: usize) -> u32 {
    let limit: unsafe extern "thiscall" fn(usize, usize, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_SELECTION_LIMIT);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut l = [0u8; 6];
    limit(this, partida, l.as_mut_ptr());
    let tipo = l[0];
    if tipo == 0 {
        return 0;
    }
    let humano = ((clube == read_usize(partida + 0x1c) && read_u8(partida + 0x40) == 0) || (clube == read_usize(partida + 0x20) && read_u8(partida + 0x41) == 0)) as u32;
    if tipo == 1 {
        crate::comparacao::ramo("cpr_1 conta nao comunitarios");
    }
    let mut k: i8 = 0;
    while k < ((read_u8(partida + 0x4d) & 0xf).wrapping_add(0xb)) as i8 {
        let p = if jogadores == 0 {
            // ponytail: a original empurra a dupla da partida com o byte baixo trocado pela vaga (a 0x8968f0 lê o byte)
            da_tatica(read_usize(G_TACTICS), clube, (partida & !0xff) | k as u8 as usize, humano)
        } else {
            read_usize(jogadores + (k as isize * 4) as usize)
        };
        if p != 0 {
            let conta = match tipo {
                2 => estrangeira(p, read_usize(clube + club::NATION), 1) != 0,
                1 => estrangeira(p, read_usize(clube + club::NATION), 1) != 0 && comunitaria(p, read_usize(clube + club::NATION)) == 0,
                3 => read_u8(p + 0x18) as i8 > l[1] as i8,
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
        while read_u8(fim) != 0 {
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

/// 1 when the club nation uses work permits (+0x76 = 0xe), the player is not EU and holds no granted permit (type 2
/// contract, 0x20 flag, search in the +0x93a array)
/// Original: `TRANSFER_MANAGER::player_needs_work_permit` (0x8c7560, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c7560 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::player_needs_work_permit(jogador, clube,
/// ignorar_a_dada)`: 0 sem clube ou sem a nação dele (+0x53); jogador nulo: a caixa "Error" 0x1c23 e 0. Nação sem a marca
/// 0xe (+0x76) ou jogador da UE (0x53d740 com nação 0): 0. Com `ignorar_a_dada`: 1. Senão 0 só se o contrato (0x4cf5a0)
/// é do tipo 2 (+0x35 & 0x3f), o id cabe em [0xae2364], tem a marca 0x20 ([this+0x86e] + id × 8) e a permissão está no
/// vetor [this+0x93a] (conta +0x93e; busca binária 0x946464 com 0x8e55c0); senão 1.
pub(crate) unsafe extern "thiscall" fn player_needs_work_permit(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    if clube == 0 || read_usize(clube + club::NATION) == 0 {
        return 0;
    }
    if jogador == 0 {
        show_error_at(0xa8d514, 0x1c23);
        return 0;
    }
    if read_u8(read_usize(clube + club::NATION) + 0x76) != 0xe {
        return 0;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    if comunitaria(jogador, 0) != 0 {
        return 0;
    }
    if ignorar_a_dada != 0 {
        return 1;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), jogador);
    if c != 0 && read_u8(c + 0x35) & 0x3f == 2 {
        let id = read_usize(jogador) as i32;
        if id < read_usize(G_STAFF_COUNT) as i32 && read_u8(read_usize(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) & 0x20 != 0 {
            let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
            if bsearch(&id, read_usize(this + 0x93a), read_usize(this + 0x93e), 0xe, 0x8e55c0) != 0 {
                crate::comparacao::ramo("cpr_1 permissao achada");
                return 0; // tem a permissão
            }
        }
    }
    1
}

/// Comparador: lado a lado pelos 42 bytes da tabela (o alcance e a virtual das regras só escrevem nela).
pub(crate) unsafe extern "thiscall" fn comparar_fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32)>(IDX_FILL_SQUAD_RESTRICTIONS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(t as usize, 42)], || o(this, partida, t, so_a_principal), || fill_squad_restrictions(this, partida, t, so_a_principal));
    PLACAR.registrar(igual, || format!("fill_squad_restrictions({:x}, {}) {:x?} x {:x?}", partida, so_a_principal, mo, mr));
}

/// Comparador: lado a lado pela saída (6 bytes) e pelo aviso (0x100 bytes, com `msg`), e pela devolução. A 0x8968f0 pode
/// pôr −1 numa vaga sem pessoa válida: a original já deixou assim e a cópia acha a mesma vaga vazia.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_check_matchday_squad_limit(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, nao_usado: u32, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize) -> u32>(IDX_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
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
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes[..n], || o(this, clube, partida, msg, jogadores, nao_usado, saida), || transfer_manager_check_matchday_squad_limit(this, clube, partida, msg, jogadores, nao_usado, saida));
    PLACAR.registrar(igual && r == c, || format!("transfer_manager_check_matchday_squad_limit(clube {:x}, partida {:x}) original={} rust={} {:x?} x {:x?}", clube, partida, r, c, mo, mr));
    r
}

/// Comparador: pelo valor (só lê). Jogador nulo com clube de nação: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_player_needs_work_permit(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_PLAYER_NEEDS_WORK_PERMIT);
    if jogador == 0 {
        return o(this, jogador, clube, ignorar_a_dada);
    }
    let r = o(this, jogador, clube, ignorar_a_dada);
    let n = player_needs_work_permit(this, jogador, clube, ignorar_a_dada);
    PLACAR.registrar(r == n, || format!("player_needs_work_permit(jogador {}, clube {:x}, {}) original={} rust={}", read_usize(jogador), clube, ignorar_a_dada, r, n));
    r
}

// ---------- 254a leva: quem pode negociar: se o clube pode chegar ao jogador, os termos mandados ao jogador da troca e a fila da transferência livre do clube (transfer_manager.cpp) ----------




/// Rewinds a LLIST and collects all node pointers (cursor left at the end).
///
/// [pt] Os itens de uma LLIST, do começo (o cursor +0xa volta a +2 e fica no fim, como depois de uma volta da original).
pub(crate) unsafe fn llist_collect(l: usize) -> Vec<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    write_usize(l + 0xa, read_usize(l + 2));
    let mut v = Vec::new();
    let mut e = proximo(l);
    while e != 0 {
        v.push(e);
        e = proximo(l);
    }
    v
}

/// Body of TRANSFER_MANAGER::can_club_bid_for_player (0x8ce990) over a built local list: returns (reason, allowed).
///
/// [pt] O corpo da 0x8ce990 sobre a lista local `l` (já construída): devolve (motivo, pode).
pub(crate) unsafe fn can_club_bid_body(this: usize, jogador: usize, clube: usize, ignorar_abertas: u32, l: usize) -> (u16, u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let acertada: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_AGREED_CONTRACT_OFFER);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_BASQUE_CLUB);
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let nat = read_usize(jogador + 0x1a);
    if nat != 0 && read_usize(nat) == read_usize(KEY_NATION_SAUDI_ARABIA) {
        let cc = read_usize(jogador + 0x39);
        if (cc == 0 || read_usize(cc + 0x53) == nat) && clube != 0 && read_usize(clube + club::NATION) != nat {
            return (0x27, 0); // não pode sair da nação
        }
    }
    let marca = read_usize(this + 0x86e).wrapping_add(read_usize(jogador).wrapping_mul(8));
    if read_u8(marca) & 0x40 != 0 {
        if acertada(read_usize(G_CONTRACT_MANAGER), read_usize(jogador) as i32) != 0 {
            crate::comparacao::ramo("cpr_2 contrato acertado");
            return (0x21, 0); // contrato acertado
        }
        let m = read_usize(this + 0x86e).wrapping_add(read_usize(jogador).wrapping_mul(8)) as *mut u8;
        *m &= 0xbf;
    }
    if ignorar_abertas == 0 && (trocas(this, jogador, 0) != 0 || negociacoes(this, jogador, 0) != 0) {
        return (7, 0);
    }
    // a fila da transferência livre (+0x92c)
    let fila = this + 0x92c;
    write_usize(fila + 0xa, read_usize(fila + 2));
    let mut e = proximo(fila);
    while e != 0 {
        if read_usize(jogador) == read_usize(e) {
            return (1, 0);
        }
        e = proximo(fila);
    }
    if sob_oferta(this, jogador, 0) != 0 {
        ofertas(this, jogador, l, 0);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            let k = read_usize(e) as i32;
            let vaga = if k < read_usize(this + 4) as i32 { read_usize(this).wrapping_add((k as usize).wrapping_mul(4)) } else { read_usize(this) };
            let o = read_usize(vaga);
            let st = read_u8(o + 0x2c);
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
    if clube == 0 || jogador == 0 || read_usize(jogador + 0x61) == 0 {
        return (0, 1);
    }
    if basco(clube) != 0 {
        let nacao = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_PAYS_BASQUE).wrapping_mul(0x122));
        if estrangeira(jogador, nacao, 1) != 0 {
            return (0x19, 0);
        }
        return (0, 1);
    }
    let n = read_usize(clube + club::NATION);
    if n != 0 && read_usize(n) == read_usize(KEY_NATION_TURKEY) {
        let c57 = read_usize(clube + club::DIVISION);
        if c57 == 0 || read_usize(c57) != read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
            if estrangeira(jogador, n, 1) != 0 {
                return (0x28, 0);
            }
            return (0, 1);
        }
    }
    let id = read_usize(clube);
    if id == read_usize(G_KEY_CLUB_REAL_SOCIEDAD_C_F) || id == read_usize(G_KEY_CLUB_REAL_SOCIEDAD_C_F_B) {
        let n1 = read_usize(jogador + 0x1a);
        if n1 == 0 || read_usize(n1) == read_usize(KEY_NATION_SPAIN) {
            let n2 = read_usize(jogador + 0x1e);
            if n2 == 0 || read_usize(n2) != read_usize(KEY_NATION_PAYS_BASQUE) {
                return (0x1a, 0);
            }
        }
    }
    (0, 1)
}

/// cannot (0 and the reason): nation that keeps its players (0x27), agreed contract (0x21), open exchanges (7), free
/// queue (1), offer in negotiation (0x12, flag +0x2d) or future transfer (0x20), the rules of the Basque club (0x19),
/// of nation [0x9cf4dc] (0x28) and of clubs [0x9d0128]/[0x9d01a0] (0x1a); otherwise 1
/// Original: `TRANSFER_MANAGER::can_club_bid_for_player` (0x8ce990, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8ce990 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::can_club_bid_for_player(jogador, clube, motivo, ignorar_abertas)`: não pode (0 e o motivo) se o
/// jogador é da nação [0x9cf458] sem clube ou com clube dela e o clube é de fora (0x27); com a marca de contrato
/// acertado (0x40) e o acerto (0x4cca30) (0x21; sem acerto, a marca sai); sem `ignorar_abertas`, com trocas que o
/// oferecem (0x8c5e40) ou negociações de troca (0x8c5fc0) (7); na fila da transferência livre (+0x92c) (1); sob oferta
/// (0x8cde70), pelas ofertas por ele (0x8c5cf0): uma em negociação (estados 0xa a 0xe; marca +0x2d) (0x12) ou uma
/// transferência futura (0x13) (0x20). Pelo clube: basco (0x53d7f0) com jogador estrangeiro na nação [0x9cf424] (0x19);
/// da nação [0x9cf4dc] fora da [0x9cf8d8] (+0x57) com jogador estrangeiro (0x28); os clubes [0x9d0128]/[0x9d01a0] com
/// jogador sem a nação [0x9cf424] (0x1a). Senão pode (1, motivo 0).
pub(crate) unsafe extern "thiscall" fn can_club_bid_for_player(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let (m, r) = can_club_bid_body(this, jogador, clube, ignorar_abertas, l);
    if !motivo.is_null() {
        *motivo = m;
    }
    soltar(l);
    r
}

/// flags the exchange player sent terms; when none is flagged, the offer moves to state 8 (from 7) or 6
/// Original: `TRANSFER_MANAGER::note_terms_offered_to_exchange_player` (0x8c2060, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c2060 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::note_terms_offered_to_exchange_player(k,
/// pessoa)`: `k` negativo ou fora da conta (+4): a caixa "Error" (0xdc3 ou 0xdc8). Na oferta `k` (vazia: nada), pela
/// lista dos jogadores da troca (+4), marca (+4 do item) o da `pessoa`; nenhum item marcado: o estado (+0x2c) vira 8 (se
/// era 7) ou 6.
pub(crate) unsafe extern "thiscall" fn note_terms_offered_to_exchange_player(this: usize, k: i32, pessoa: u32) {
    if k < 0 {
        show_error_at(0xa8d514, 0xdc3);
        return;
    }
    if k >= read_usize(this + 4) as i32 {
        show_error_at(0xa8d514, 0xdc8);
        return;
    }
    let o = read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4)));
    if o == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(o + 4);
    write_usize(l + 0xa, read_usize(l + 2));
    let mut nenhum = true;
    let mut e = proximo(read_usize(o + 4));
    while e != 0 {
        if read_usize(e) as u32 == pessoa {
            *((e + 4) as *mut u8) = 1; // termos mandados
        }
        if read_u8(e + 4) != 0 {
            nenhum = false;
        }
        e = proximo(read_usize(o + 4));
    }
    if nenhum {
        *((o + 0x2c) as *mut u8) = if read_u8(o + 0x2c) == 7 { 8 } else { 6 };
    }
}

/// counts the free transfer queue people of the club or of its partner club (0x540a50)
/// Original: `TRANSFER_MANAGER::count_club_free_transfer_queue` (0x8c1780, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c1780 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::count_club_free_transfer_queue(clube)`:
/// clube nulo: a caixa "Error" 0xc04 e 0. Senão conta as pessoas da fila da transferência livre (+0x92c; o cursor volta
/// ao começo) com o clube do contrato (+0x39) igual ao clube ou ao parceiro dele (0x540a50 com 0 e 1), não nulo.
pub(crate) unsafe extern "thiscall" fn count_club_free_transfer_queue(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        show_error_at(0xa8d514, 0xc04);
        return 0;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let p = parceiro(clube, 0, 1);
    write_usize(this + 0x936, read_usize(this + 0x92e));
    let mut n = 0u8;
    let mut e = proximo(this + 0x92c);
    while e != 0 {
        let c = read_usize(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e).wrapping_mul(0x6e)) + 0x39);
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
pub(crate) unsafe extern "thiscall" fn comparar_can_club_bid_for_player(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32>(IDX_CAN_CLUB_BID_FOR_PLAYER);
    let marca = read_usize(this + 0x86e).wrapping_add(read_usize(jogador).wrapping_mul(8));
    let regioes = [(marca, 1), (this + 0x936, 4), (motivo as usize, 2)];
    let n = if motivo.is_null() { 2 } else { 3 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, clube, motivo, ignorar_abertas), || can_club_bid_for_player(this, jogador, clube, motivo, ignorar_abertas));
    PLACAR.registrar(igual && r == c, || format!("can_club_bid_for_player(jogador {}, clube {:x}, {}) original={} rust={} {:x?} x {:x?}", read_usize(jogador), clube, ignorar_abertas, r, c, mo, mr));
    r
}

/// Comparador: lado a lado pelo estado da oferta, pelas marcas (+4) dos itens da troca e pelo cursor da lista. Fora da
/// conta: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_note_terms_offered_to_exchange_player(this: usize, k: i32, pessoa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32)>(IDX_NOTE_TERMS_OFFERED_TO_EXCHANGE_PLAYER);
    if k < 0 || k >= read_usize(this + 4) as i32 || read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4))) == 0 {
        return o(this, k, pessoa);
    }
    let of = read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4)));
    let l = read_usize(of + 4);
    let mut regioes = vec![(of + 0x2c, 1), (l + 0xa, 4)];
    for e in llist_collect(l) {
        regioes.push((e + 4, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, k, pessoa), || note_terms_offered_to_exchange_player(this, k, pessoa));
    PLACAR.registrar(igual, || format!("note_terms_offered_to_exchange_player({}, pessoa {}) {:x?} x {:x?}", k, pessoa, mo, mr));
}

/// Comparador: lado a lado pelo cursor da fila e pela devolução (só lê). Clube nulo: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_count_club_free_transfer_queue(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_COUNT_CLUB_FREE_TRANSFER_QUEUE);
    if clube == 0 {
        return o(this, clube);
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x936, 4)], || o(this, clube), || count_club_free_transfer_queue(this, clube));
    PLACAR.registrar(igual && r == c, || format!("count_club_free_transfer_queue({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

// ---------- 254a leva: as transferências futuras: a do jogador, tirar a da oferta e acrescentar a da oferta acertada (transfer_manager.cpp) ----------





/// Pointer to the flags byte of person id in the 8-byte records at [this+0x86e].
///
/// [pt] O byte de marcas da pessoa de id `id` no registro de 8 bytes em [this+0x86e].
pub(crate) unsafe fn staff_flags_byte(this: usize, id: usize) -> *mut u8 {
    read_usize(this + 0x86e).wrapping_add(id.wrapping_mul(8)) as *mut u8
}

/// In a list of offer indices (rewound), index of the first offer with status 0x13 (future transfer), or None.
///
/// [pt] Na lista de índices de ofertas `l` (o cursor volta ao começo), o índice da primeira oferta de estado 0x13
/// (transferência futura), ou None.
pub(crate) unsafe fn find_future_offer_in_list(this: usize, l: usize) -> Option<usize> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    write_usize(l + 0xa, read_usize(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        if read_u8(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4))) + 0x2c) == 0x13 {
            return Some(read_usize(e));
        }
        e = proximo(l);
    }
    None
}

/// Body of TRANSFER_MANAGER::get_future_transfer (0x8cca10) over a built local list: finds the future-transfer offer,
/// writes its record to `saida`, clears flag 0x10 if none.
///
/// [pt] O corpo da 0x8cca10 sobre a lista local `l` (já construída).
pub(crate) unsafe fn get_future_transfer_body(this: usize, jogador: usize, saida: *mut usize, l: usize) -> u32 {
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    if *staff_flags_byte(this, read_usize(jogador)) & 0x10 == 0 {
        return 0;
    }
    let mut k = None;
    if ofertas(this, jogador, l, 0) != 0 {
        k = find_future_offer_in_list(this, l).filter(|&i| i != usize::MAX);
    }
    if k.is_none() {
        limpar(l);
        if trocas(this, jogador, l) != 0 {
            k = find_future_offer_in_list(this, l).filter(|&i| i != usize::MAX);
        }
    }
    let Some(k) = k else {
        // ponytail: a marca é relida pela original depois das chamadas; o vetor [this+0x86e] não muda entre elas
        *staff_flags_byte(this, read_usize(jogador)) &= 0xef; // sem transferência futura: a marca sai
        return 0;
    };
    crate::comparacao::ramo("cpr_3 futura achada");
    if !saida.is_null() {
        let n = read_usize(this + 0x928) as i32;
        for i in 0..n.max(0) as usize {
            let r = read_usize(this + 0x920).wrapping_add(i * 0xc);
            if read_usize(r + 8) == k {
                *saida = r;
                break;
            }
        }
    }
    1 // a transferência futura existe
}

/// with the 0x10 flag, searches the offers for the player (0x8c5cf0) and the exchanges offering him (0x8c5e40) for a
/// state 0x13 one; found, the +0x920 array entry and 1; otherwise clears the flag and 0
/// Original: `TRANSFER_MANAGER::get_future_transfer` (0x8cca10, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cca10 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::get_future_transfer(jogador, saida)`: sem a marca 0x10 ([this+0x86e] + id × 8), 0. Pelas ofertas
/// pelo jogador (0x8c5cf0) e, sem achar, pelas trocas que o oferecem (0x8c5e40, com a lista esvaziada por 0x68d080), a
/// primeira de estado 0x13 com índice ≠ −1; com `saida`, o registro de 12 bytes do vetor [this+0x920] (conta +0x928)
/// com a oferta em +8; 1. Nenhuma: tira a marca 0x10 e 0.
pub(crate) unsafe extern "thiscall" fn get_future_transfer(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = get_future_transfer_body(this, jogador, saida, l);
    soltar(l);
    r
}

/// clears the 0x10 flag of the player and the exchange players and removes the offer's entry from the +0x920 array (the
/// next ones move down, 0x8cbf40)
/// Original: `TRANSFER_MANAGER::remove_future_transfer` (0x8cbf70, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cbf70 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_future_transfer(oferta)`: tira a
/// marca 0x10 do jogador (+0xc) e dos jogadores da troca (lista +4, o cursor volta ao começo); no vetor [this+0x920]
/// (conta +0x928), o primeiro registro com a oferta (+0x20 da oferta, +8 do registro) sai: os seguintes descem um
/// (0x8cbf40) e a conta cai de 1.
pub(crate) unsafe extern "thiscall" fn remove_future_transfer(this: usize, oferta: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FUTURE_TRANSFER_ASSIGN);
    *staff_flags_byte(this, read_usize(oferta + 0xc)) &= 0xef;
    let l = read_usize(oferta + 4);
    if l != 0 {
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = proximo(read_usize(oferta + 4));
        while e != 0 {
            *staff_flags_byte(this, read_usize(e)) &= 0xef;
            e = proximo(read_usize(oferta + 4));
        }
    }
    let n = read_usize(this + 0x928) as i32;
    let alvo = read_usize(oferta + 0x20);
    for i in 0..n.max(0) {
        if read_usize(read_usize(this + 0x920).wrapping_add(i as usize * 0xc) + 8) == alvo {
            crate::comparacao::ramo("cpr_3 tirar futura");
            let mut j = i;
            while j < read_usize(this + 0x928) as i32 - 1 {
                let r = read_usize(this + 0x920).wrapping_add(j as usize * 0xc);
                copiar(r, r + 0xc);
                j += 1;
            }
            write_usize(this + 0x928, read_usize(this + 0x928).wrapping_sub(1));
            return;
        }
    }
}

/// adds (date, offer) to the +0x920 array and sorts it; the offer moves to state 0x13; suspends the shortlists for the
/// player and the exchange players (0x842b00) and sets their 0x10 flag; a flagged contract keeps the offer
/// Original: `TRANSFER_MANAGER::add_future_transfer` (0x8cb9a0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cb9a0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::add_future_transfer(oferta, data)`:
/// acrescenta ao vetor [this+0x920] (capacidade +0x924, conta +0x928; cheio, cresce de 10 em 10, sem memória: a caixa
/// 0x27c3 e nada) o registro de 12 bytes com a `data` (0x417c10) e a oferta (+0x20 em +8) e reordena (qsort 0x945333
/// com 0x778e90). A oferta fica com +0x2d = 1 e o estado 0x13. Suspende os clubes que miram o jogador (0x842b00 de
/// [this+0x856], com 0) e liga a marca 0x10 dele; com contrato (0x4cf5a0) marcado em +0x49, a oferta vai a +0x4a do
/// contrato. O mesmo (suspender e marcar) para cada jogador da troca (lista +4).
pub(crate) unsafe extern "thiscall" fn add_future_transfer(this: usize, oferta: usize, data: usize) {
    crate::comparacao::ramo("cpr_3 acrescentar futura");
    if !grow_record_vector(this, 0x920, 4, 10, 0xc, 0x27c3, 0xa8ec74) {
        return;
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut [u16; 4], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut temp = [0u16; 4];
    copia(read_usize(this + 0x920).wrapping_add(read_usize(this + 0x928).wrapping_mul(0xc)), &mut temp, data);
    write_usize(read_usize(this + 0x920).wrapping_add(read_usize(this + 0x928).wrapping_mul(0xc)) + 8, read_usize(oferta + 0x20));
    let n = read_usize(this + 0x928).wrapping_add(1);
    write_usize(this + 0x928, n);
    qsort(read_usize(this + 0x920), n as u32, 0xc, 0x778e90);
    *((oferta + 0x2d) as *mut u8) = 1;
    *((oferta + 0x2c) as *mut u8) = 0x13; // transferência futura
    suspender(read_usize(this + 0x856), staff_records_staff_by_id(read_usize(oferta + 0xc)), 0);
    *staff_flags_byte(this, read_usize(oferta + 0xc)) |= 0x10; // a marca da transferência futura
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)));
    if c != 0 && read_u8(c + 0x49) != 0 {
        write_usize(c + 0x4a, read_usize(oferta + 0x20));
    }
    let l = read_usize(oferta + 4);
    if l != 0 {
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = proximo(read_usize(oferta + 4));
        while e != 0 {
            suspender(read_usize(this + 0x856), staff_records_staff_by_id(read_usize(e)), 0);
            *staff_flags_byte(this, read_usize(e)) |= 0x10;
            e = proximo(read_usize(oferta + 4));
        }
    }
}

/// Comparador: lado a lado pela marca do jogador e por `saida`, e pela devolução (tirar a marca é o único outro efeito;
/// a cópia, depois da original, acha o mesmo).
pub(crate) unsafe extern "thiscall" fn comparar_get_future_transfer(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(IDX_GET_FUTURE_TRANSFER);
    let regioes = [(staff_flags_byte(this, read_usize(jogador)) as usize, 1), (saida as usize, 4)];
    let n = if saida.is_null() { 1 } else { 2 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, saida), || get_future_transfer(this, jogador, saida));
    PLACAR.registrar(igual && r == c, || format!("get_future_transfer(jogador {}) original={} rust={} {:x?} x {:x?}", read_usize(jogador), r, c, mo, mr));
    r
}

/// Comparador: lado a lado pela conta e pelo vetor das futuras, pelas marcas do jogador e dos da troca e pelo cursor
/// da lista da troca (não aloca nem libera).
pub(crate) unsafe extern "thiscall" fn comparar_remove_future_transfer(this: usize, oferta: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_FUTURE_TRANSFER);
    let mut regioes = vec![(this + 0x928, 4), (staff_flags_byte(this, read_usize(oferta + 0xc)) as usize, 1)];
    let n = read_usize(this + 0x928) as i32;
    if n > 0 {
        regioes.push((read_usize(this + 0x920), n as usize * 0xc));
    }
    let l = read_usize(oferta + 4);
    if l != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            regioes.push((staff_flags_byte(this, read_usize(e)) as usize, 1));
            e = proximo(l);
        }
        regioes.push((l + 0xa, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || remove_future_transfer(this, oferta));
    PLACAR.registrar(igual, || format!("remove_future_transfer(oferta {:x}) {:x?} x {:x?}", read_usize(oferta + 0x20), mo, mr));
}

/// Comparador: a original acrescenta (pode realocar), reordena e suspende as listas de alvos: só ela, conferida pelo
/// plano: a conta sobe de 1, o registro (data e oferta) está no vetor, o estado 0x13 com +0x2d = 1 e a marca 0x10 do
/// jogador.
pub(crate) unsafe extern "thiscall" fn comparar_add_future_transfer(this: usize, oferta: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_FUTURE_TRANSFER);
    let n = read_usize(this + 0x928);
    let mut alvo = [0u8; 0xc];
    alvo[..8].copy_from_slice(core::slice::from_raw_parts(data as *const u8, 8));
    alvo[8..].copy_from_slice(&(read_usize(oferta + 0x20) as u32).to_le_bytes());
    o(this, oferta, data);
    if read_usize(this + 0x920) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x928);
    let marca = *staff_flags_byte(this, read_usize(oferta + 0xc));
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x920), conta, 0xc, &alvo) && read_u8(oferta + 0x2c) == 0x13 && read_u8(oferta + 0x2d) == 1 && marca & 0x10 != 0;
    PLACAR.registrar(ok, || format!("add_future_transfer(oferta {:x}): conta {} -> {}, estado {:#x}, marca {:#x}", read_usize(oferta + 0x20), n, conta, read_u8(oferta + 0x2c), marca));
}

// ---------- 254a leva: as ofertas abertas e as desfeitas: as ofertas pelos jogadores do clube, desfazer as ofertas pelo jogador e guardar o percentual da próxima venda (transfer_manager.cpp) ----------






/// counts (and lists the indices of) the open offers for players contracted to the club, except states 2, 3, 4, 0x10,
/// 0x14, collapsed ones and loans
/// Original: `TRANSFER_MANAGER::get_open_offers_for_club_players` (0x8c6100, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c6100 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_open_offers_for_club_players(clube,
/// lista)`: pelas ofertas (vetor [this], conta +4), fora das vazias e das de estado 2, 3, 4, 0x10 e 0x14 e das de estado
/// 0xf com +0x2e ≥ 3: o contrato do jogador (+0xc; 0x4cf5a0, guardado em [0xae1080]) que não é de empréstimo (+0x35 &
/// 0x3f ≠ 5) é do `clube`? Conta e, com `lista`, acrescenta o índice (4 bytes, 0x68d0d0). Devolve a conta.
pub(crate) unsafe extern "thiscall" fn get_open_offers_for_club_players(this: usize, clube: usize, lista: usize) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let mut n = 0u32;
    let mut i: i32 = 0;
    let (mut desfeita, mut emprestado) = (false, false); // os contadores, uma vez por chamada
    while i < read_usize(this + 4) as i32 {
        let o = read_usize(read_usize(this).wrapping_add((i as usize).wrapping_mul(4)));
        if o != 0 {
            let conta = match read_u8(o + 0x2c) {
                2 | 3 | 4 | 0x10 | 0x14 => false,
                0xf => {
                    desfeita |= read_u8(o + 0x2e) >= 3;
                    read_u8(o + 0x2e) < 3 // desfeita (+0x2e = 3) não conta
                }
                _ => true,
            };
            if conta {
                let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(o + 0xc)));
                write_usize(0xae1080, c);
                emprestado |= c != 0 && read_u8(c + 0x35) & 0x3f == 5 && friendly_club_by_id(read_usize(c + 4)) == clube;
                let dono = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { friendly_club_by_id(read_usize(c + 4)) } else { 0 }; // o empréstimo não conta
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

/// each offer for the player loses its future transfer (0x8cbf70), is collapsed (0xf, +0x2e = 3), clears the buyer's
/// and seller's bid markers (0x84aeb0) and is scheduled (0x8c6220, 10 days)
/// Original: `TRANSFER_MANAGER::collapse_offers_for_player` (0x8c47d0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c47d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::collapse_offers_for_player(pessoa)`: com ofertas pela pessoa (byte +1 do registro de 8 bytes em
/// [this+0x86e]), pelas ofertas por ela (0x8c5cf0 na lista local): a de transferência futura (0x13) perde a futura
/// (0x8cbf70); todas ficam no estado 0xf com +0x2e = 3; o vendedor é o clube do contrato do jogador da oferta (0x4cf5a0,
/// guardado em [0xae1080]); os lances do comprador (+8) pelo jogador e os do vendedor pelos da troca (lista +4) saem
/// (0x84aeb0 de [this+0x856]) e a oferta é agendada (0x8c6220 com 10 dias).
pub(crate) unsafe extern "thiscall" fn collapse_offers_for_player(this: usize, pessoa: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar_futura: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FUTURE_TRANSFER);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    if read_u8(read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8)) + 1) != 0 {
        ofertas(this, pessoa, l, 0);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            crate::comparacao::ramo("cpr_4 desfazer oferta");
            let o = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
            if read_u8(o + 0x2c) == 0x13 {
                tirar_futura(this, o);
            }
            *((o + 0x2c) as *mut u8) = 0xf; // desfeita
            *((o + 0x2e) as *mut u8) = 3;
            let jogador = staff_records_staff_by_id(read_usize(o + 0xc));
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), jogador);
            write_usize(0xae1080, c);
            let vendedor = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
            desmarcar(read_usize(this + 0x856), friendly_club_by_id(read_usize(o + 8)), jogador);
            let t = read_usize(o + 4);
            if t != 0 {
                write_usize(t + 0xa, read_usize(t + 2));
                let mut x = proximo(read_usize(o + 4));
                while x != 0 {
                    desmarcar(read_usize(this + 0x856), vendedor, staff_records_staff_by_id(read_usize(x)));
                    x = proximo(read_usize(o + 4));
                }
            }
            agendar(this, o, 0xa, 0); // a oferta volta daqui a 10 dias
            e = proximo(l);
        }
    }
    soltar(l);
}

/// appends the 13-byte next sale percentage to the +0x880 array (growing by 50) and sorts it (0x8e54d0)
/// Original: `TRANSFER_MANAGER::add_next_sale_percentage` (0x8caaf0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8caaf0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_next_sale_percentage(registro)`:
/// acrescenta o percentual da próxima venda (13 bytes copiados de `registro`) ao vetor [this+0x880] (capacidade +0x884,
/// conta +0x888); cheio, cresce de 50 em 50 (sem memória: caixa 0x2553 e nada). Reordena (qsort 0x945333 com 0x8e54d0,
/// a ordem da busca 0x8cb0f0).
pub(crate) unsafe extern "thiscall" fn add_next_sale_percentage(this: usize, registro: usize) {
    crate::comparacao::ramo("cpr_4 write_percent_pair da proxima venda");
    if !grow_record_vector(this, 0x880, 4, 50, 0xd, 0x2553, 0xa8eb50) {
        return;
    }
    let n = read_usize(this + 0x888);
    core::ptr::copy_nonoverlapping(registro as *const u8, read_usize(this + 0x880).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    write_usize(this + 0x888, n.wrapping_add(1)); // mais um percentual
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0x880), read_usize(this + 0x888) as u32, 0xd, 0x8e54d0);
}

/// Comparador: sem `lista`, lado a lado por [0xae1080] e pela devolução; com `lista` (cresce a lista: não pode rodar
/// duas vezes), a cópia conta antes sem lista e a original roda: confere a conta.
pub(crate) unsafe extern "thiscall" fn comparar_get_open_offers_for_club_players(this: usize, clube: usize, lista: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_GET_OPEN_OFFERS_FOR_CLUB_PLAYERS);
    if lista != 0 {
        let n = get_open_offers_for_club_players(this, clube, 0);
        let r = o(this, clube, lista);
        PLACAR.registrar(r == n, || format!("get_open_offers_for_club_players({:x}, lista) original={} rust={}", clube, r, n));
        return r;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, clube, 0), || get_open_offers_for_club_players(this, clube, 0));
    PLACAR.registrar(igual && r == c, || format!("get_open_offers_for_club_players({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparador: agenda (aloca) e mexe nas listas de alvos: só a original. O plano antes: as ofertas pela pessoa (0x8c5cf0
/// numa lista própria, destruída depois); depois confere que todas ficaram no estado 0xf com +0x2e = 3.
pub(crate) unsafe extern "thiscall" fn comparar_collapse_offers_for_player(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_COLLAPSE_OFFERS_FOR_PLAYER);
    if read_u8(read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8)) + 1) == 0 {
        return o(this, pessoa);
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    ofertas(this, pessoa, l, 0);
    let mut alvo = Vec::new();
    write_usize(l + 0xa, read_usize(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        alvo.push(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4))));
        e = proximo(l);
    }
    soltar(l);
    o(this, pessoa);
    let ok = alvo.iter().all(|&of| read_u8(of + 0x2c) == 0xf && read_u8(of + 0x2e) == 3);
    PLACAR.registrar(ok, || format!("collapse_offers_for_player(pessoa {}): {} ofertas, estados {:x?}", read_usize(pessoa), alvo.len(), alvo.iter().map(|&of| (read_u8(of + 0x2c), read_u8(of + 0x2e))).collect::<Vec<_>>()));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_next_sale_percentage(this: usize, registro: usize) {
    let n = read_usize(this + 0x888);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_NEXT_SALE_PERCENTAGE)(this, registro);
    if read_usize(this + 0x880) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x888);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x880), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_next_sale_percentage: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

// ---------- 253a leva: as datas da transferência: a data de conclusão da oferta, o primeiro dia que as regras permitem e a janela fixa de transferências (transfer_manager.cpp) ----------



/// O dia de hoje: Data de 8 bytes em 0xae2c90 (+0 dia do ano i16, +2 ano u16, +4 bissexto).
pub(crate) const CPQ5_HOJE: usize = 0xae2c90;

/// As estáticas das operações de data (0x54c5b0, 0x54c8e0, 0x54c770, 0x54cdb0: valor e guarda), para os comparadores.
pub(crate) const CPQ5_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// Date `a` before `b` (unsigned year +2, then signed day +0), on 8-byte date values.
///
/// [pt] `a` antes de `b` na ordem dos originais: o ano (+2, 16 bits sem sinal) e, no mesmo ano, o dia (+0, 16 bits com sinal).
pub(crate) unsafe fn date_pair_before(a: *const [u32; 2], b: *const [u32; 2]) -> bool {
    let ler = |p: *const [u32; 2]| (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p as usize + 2) as *const u16));
    let ((da, aa), (db, ab)) = (ler(a), ler(b));
    aa < ab || (aa == ab && da < db)
}



/// Offer -> club of the target's (+0xc) contract unless type 5, contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato da pessoa da oferta (+0xc): 0x4cf5a0 com o gerente de contratos [0xae1050], guardado em
/// [0xae1080]; contrato de tipo 5 (+0x35 & 0x3f) não conta.
pub(crate) unsafe fn offer_seller_club(oferta: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        friendly_club_by_id(read_usize(c + 4))
    } else {
        0
    }
}

/// Calls TRANSFER_RULES::can_make_offer (vtable +0xc)(offer, club, date, reason): 1 allows, 0 with reason.
///
/// [pt] O método +0xc das regras (`TRANSFER_RULES::can_make_offer(oferta, clube, data, motivo)`): 1 permite; 0 e o motivo.
pub(crate) unsafe fn rules_can_make_offer(regra: usize, oferta: usize, clube: usize, data: *const [u32; 2], motivo: *mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const [u32; 2], *mut u32) -> u32 = core::mem::transmute(read_usize(read_usize(regra) + 0xc));
    f(regra, oferta, clube, data, motivo)
}

/// Reasons that do not hold the date back: 0x14, 0x15, 0x1f, 0x11.
///
/// [pt] Os motivos que não seguram a data (o dia vale assim mesmo): 0x14, 0x15, 0x1f e 0x11.
pub(crate) fn reason_ignores_date(m: u32) -> bool {
    matches!(m as u16, 0x14 | 0x15 | 0x1f | 0x11)
}

/// by the offer's timing type (+0x30): today, the first allowed day (0x8cf0e0), the seller's season end plus 7 days,
/// the day after the contract ends (never before today) or the start of the fixed window; returns the reason
/// Original: `transfer_manager_get_offer_completion_date` (0x8cee70, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cee70 } (thiscall, ret 8, transfer_manager.cpp): `transfer_manager_get_offer_completion_date(oferta, saida)`:
/// a data em que a oferta pode se concretizar, pelo tipo de prazo (+0x30 & 0xf): 0 hoje; 5 o início da janela fixa
/// (0x8d0bb0); 1 e 4 o primeiro dia que as regras permitem (0x8cf0e0, que dá o motivo); 2 o fim da temporada da nação
/// do clube vendedor mais 7 dias (0x669bd0; fora da temporada, hoje; sem vendedor, a caixa "Error" 0x31da); 3 o dia
/// seguinte ao fim do contrato (+0x2d), nunca antes de hoje (sem contrato, hoje). Outros tipos não gravam. Devolve o
/// motivo (só o ax: 0 fora dos tipos 1 e 4). O vendedor (0x4cf5a0, guardado em [0xae1080]) é calculado sempre.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_offer_completion_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut t, mut t2) = ([0u32; 2], [0u32; 2]);
    let vendedor = offer_seller_club(oferta);
    match read_u8(oferta + 0x30) & 0xf {
        0 => {
            crate::comparacao::ramo("cpq_5 conclusao imediata");
            copia(saida, &mut t, hoje); // tipo 0: hoje
        }
        5 => {
            let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
            janela(saida, 0);
        }
        1 | 4 => {
            let primeiro: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_NEXT_ALLOWED_TRANSFER_DATE);
            return primeiro(this, oferta, saida);
        }
        2 => {
            if vendedor == 0 {
                show_error_at(0xa8d514, 0x31da);
            } else {
                let temporada: unsafe extern "C" fn(usize, usize, *mut [u32; 2]) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
                if temporada(vendedor, 0, saida) != 0 {
                    let r = mais(saida, &mut t, 7);
                    copia(saida, &mut t2, r);
                } else {
                    copia(saida, &mut t2, hoje);
                }
            }
        }
        3 => {
            let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)));
            if c != 0 {
                let r = mais((c + 0x2d) as *const [u32; 2], &mut t2, 1);
                copia(saida, &mut t, r);
                if date_pair_before(saida, hoje) {
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

/// steps day by day, from today (or the contract end, in type 4) up to a year, until the rules of the buyer's and
/// seller's nations allow it (method +0xc); with no day, returns the first reason
/// Original: `TRANSFER_MANAGER::find_next_allowed_transfer_date` (0x8cf0e0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf0e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_allowed_transfer_date(oferta,
/// saida)`: grava hoje na saída e começa no maior entre hoje e, no tipo de prazo 4 (+0x30 & 0xf), o fim do contrato
/// (+0x2d). Tipo 5: o início da janela fixa (0x8d0bb0) e o método +0xc da regra da nação do comprador (+8) dá o motivo
/// (9 não conta). Pessoa sem ficha de jogador (+0x61): hoje. Senão anda dia a dia (até hoje + 365, sem chegar) pelas
/// regras da nação do comprador e, com o +4 da oferta e um vendedor (0x4cf5a0, guardado em [0xae1080]), da do vendedor:
/// o primeiro dia que as duas permitem (ou cujo motivo libera: 0x14, 0x15, 0x1f, 0x11) vai para a saída e devolve 0;
/// sem dia, devolve o primeiro motivo (9 não conta) e a saída fica hoje. Só o ax é devolvido.
pub(crate) unsafe extern "thiscall" fn find_next_allowed_transfer_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut dia, mut limit, mut inicio, mut t, mut t2) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    padrao(&mut dia);
    mais(hoje, &mut limit, 0x16d);
    padrao(&mut inicio);
    let mut motivo = 0u32;
    // ponytail: o motivo da regra é uma variável só, sem valor antes da primeira chamada (a original não a zera; a
    // regra o grava sempre que devolve 0)
    let mut m = 0u32;
    copia(saida, &mut t, hoje);
    let pessoa = staff_records_staff_by_id(read_usize(oferta + 0xc));
    if read_u8(oferta + 0x30) & 0xf == 4 {
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
        if c != 0 {
            copia(&mut inicio, &mut t, (c + 0x2d) as *const [u32; 2]);
            t = *hoje;
            if date_pair_before(&inicio, hoje) {
                copia(&mut inicio, &mut t, hoje);
            }
        } else {
            copia(&mut inicio, &mut t, hoje);
        }
    } else {
        copia(&mut inicio, &mut t, hoje);
    }
    let comprador = friendly_club_by_id(read_usize(oferta + 8));
    let vendedor = offer_seller_club(oferta);
    if read_u8(oferta + 0x30) & 0xf == 5 {
        let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
        janela(saida, 0);
        let regra = nation_transfer_rules_or_zero(this, read_usize(comprador + 0x53));
        if regra != 0 && rules_can_make_offer(regra, oferta, comprador, saida, &mut m) == 0 && m as u16 != 9 {
            motivo = m;
        }
        return motivo as u16;
    }
    if read_usize(pessoa + staff::PLAYER) == 0 {
        copia(saida, &mut t, hoje);
        return 0;
    }
    let regra_c = nation_transfer_rules_or_zero(this, read_usize(comprador + 0x53));
    let regra_v = if vendedor != 0 && read_usize(oferta + 4) != 0 { nation_transfer_rules_or_zero(this, read_usize(vendedor + 0x53)) } else { 0 };
    crate::comparacao::ramo("cpq_5 dia permitido");
    copia(&mut dia, &mut t, &inicio); // o primeiro dia testado é o início
    while date_pair_before(&dia, &limit) {
        let mut ve_vendedor = true;
        if regra_c != 0 && rules_can_make_offer(regra_c, oferta, comprador, &dia, &mut m) == 0 {
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            ve_vendedor = reason_ignores_date(motivo);
        }
        if ve_vendedor {
            if regra_v == 0 || rules_can_make_offer(regra_v, oferta, vendedor, &dia, &mut m) != 0 {
                copia(saida, &mut t2, &dia);
                return 0;
            }
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            if reason_ignores_date(motivo) {
                copia(saida, &mut t2, &dia);
                return 0;
            }
        }
        let r = mais(&dia, &mut t, 1);
        copia(&mut dia, &mut t2, r);
    }
    motivo as u16
}

/// the start (today, when inside) and the end of the current or next window: 1 to 31 July and 15 December to 15 January
/// Original: `transfer_manager_get_next_window_dates` (0x8d0bb0, transfer_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x8d0bb0 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_get_next_window_dates(inicio, fim)`: as
/// janelas fixas do ano de hoje são 1º a 31 de julho e 15 de dezembro a 15 de janeiro (0x549f70; comparadas só pelo
/// dia do ano, com sinal). Dentro de uma janela, o início é hoje e o fim o da janela (15 de janeiro do ano seguinte, se
/// for a de dezembro); antes de julho, 1º a 31 de julho; entre agosto e 14 de dezembro, 15 de dezembro a 15 de janeiro
/// do ano seguinte. O fim só com `fim` não nulo. Não usa o ecx (os chamadores passam o gerente).
pub(crate) unsafe extern "stdcall" fn transfer_manager_get_next_window_dates(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let dma: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> *mut [u32; 2] = core::mem::transmute(FN_DATE_DATE_549F70);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
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

/// Memory regions for the date comparers: the outputs (8 bytes each), G_CONTRACT_LOOKUP_RESULT and the dates' statics.
///
/// [pt] As regiões dos comparadores de datas: a saída, o contrato guardado [0xae1080] e as estáticas das datas.
pub(crate) fn date_comparer_regions(saidas: &[usize]) -> Vec<(usize, usize)> {
    let mut r: Vec<(usize, usize)> = saidas.iter().filter(|&&s| s != 0).map(|&s| (s, 8)).collect();
    r.push((0xae1080, 4));
    r.extend_from_slice(&CPQ5_ESTATICAS);
    r
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo (as regras, a temporada e a
/// janela só calculam).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_offer_completion_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(IDX_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    let reg = date_comparer_regions(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || transfer_manager_get_offer_completion_date(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_get_offer_completion_date(tipo {}) motivo {} x {}, memória igual {}", read_u8(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo.
pub(crate) unsafe extern "thiscall" fn comparar_find_next_allowed_transfer_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(IDX_FIND_NEXT_ALLOWED_TRANSFER_DATE);
    let reg = date_comparer_regions(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || find_next_allowed_transfer_date(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("find_next_allowed_transfer_date(tipo {}) motivo {} x {}, memória igual {}", read_u8(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pelo início e o fim (só datas, sem estáticas).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_get_next_window_dates(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut [u32; 2], *mut [u32; 2])>(IDX_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
    let reg = date_comparer_regions(&[inicio as usize, fim as usize]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(inicio, fim), || transfer_manager_get_next_window_dates(inicio, fim));
    PLACAR.registrar(igual, || format!("transfer_manager_get_next_window_dates(hoje {:x}, fim {}) diferente", *(CPQ5_HOJE as *const u32), !fim.is_null()));
}

// ---------- 253a leva: o dinheiro da transferência: a compensação pela pessoa, pagar a transferência e guardar as cláusulas, e se o clube vendedor tem de aceitar (transfer_manager.cpp) ----------





/// Person -> club of their contract unless type 5, contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato da `pessoa` (0x4cf5a0 com o gerente de contratos [0xae1050], guardado em [0xae1080]); contrato
/// de tipo 5 (+0x35 & 0x3f) não conta.
pub(crate) unsafe fn seller_club_of_person(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        friendly_club_by_id(read_usize(c + 4))
    } else {
        0
    }
}

/// Writes `v` as little-endian u32 at offset k of a 13-byte record buffer.
pub(crate) unsafe fn put_u32_le(r: &mut [u8; 13], k: usize, v: usize) {
    r[k..k + 4].copy_from_slice(&(v as u32).to_le_bytes());
}

/// As estáticas das operações de data (valor e guarda), para os comparadores.
pub(crate) const CPQ6_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// staff on a type 2 contract: what is left of the contract (1000 to 1000000); new rules: the average with the player
/// value; between clubs of nation [0x9cf4a8] or, under 24, between British nations: half the value (0x4d0c60)
/// Original: `transfer_manager_calc_compensation_fee` (0x8d0780, transfer_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x8d0780 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só da data local):
/// `transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas)`: sem pessoa ou comprador, a caixa
/// "Error" 0x372e e 0. Comissão técnica (sem ficha de jogador +0x61, com +0x69) de contrato tipo 2: o que falta do
/// contrato (dias até o fim +0x2d × o salário +0xc / 7) entre 1000 e 1000000. Regras novas ([0xae233c] = 1, sem o
/// argumento) com contrato tipo 2: a média entre o que falta e o valor do jogador no vendedor (0x4d0c60). Senão (contrato
/// que não é de tipo 5, as duas nações): entre clubes da nação [0x9cf4a8], metade do valor; fora disso, só abaixo de 24
/// anos na data (o fim do contrato, no tipo 2; senão hoje) e entre nações britânicas ou a Irlanda (0x5446d0), metade
/// do valor. Os outros casos, 0.
pub(crate) unsafe extern "stdcall" fn transfer_manager_calc_compensation_fee(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let idade: unsafe extern "C" fn(usize, *const [u32; 2]) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_AGE);
    let britanica: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_NATION_IS_UK_OR_IRELAND);
    let gerente = || read_usize(G_CONTRACT_MANAGER);
    // o que falta do contrato: os dias até o fim × o salário semanal, por 7 (divisão truncada, como a original)
    let resto = |c: usize| dias_ate(c + 0x2d, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32).wrapping_mul(read_usize(c + 0xc) as i32) / 7;
    let mut data = [0u32; 2];
    let mut t = [0u32; 2];
    padrao(&mut data);
    if pessoa == 0 || comprador == 0 {
        show_error_at(0xa8d514, 0x372e);
        return 0;
    }
    if read_usize(pessoa + staff::PLAYER) == 0 && read_usize(pessoa + staff::NON_PLAYER) != 0 {
        let c = contract_of(gerente(), pessoa);
        if c == 0 || read_u8(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        return resto(c).clamp(1000, 1_000_000);
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && sem_regras_novas == 0 {
        let c = contract_of(gerente(), pessoa);
        if c == 0 || read_u8(c + 0x35) & 0x3f != 2 {
            return 0;
        }
        let r = resto(c);
        let v = valor(gerente(), pessoa, friendly_club_by_id(read_usize(c + 4)), 1, 0xffff_ffff, 0);
        return v.wrapping_add(r) / 2;
    }
    let c = contract_of(gerente(), pessoa);
    if c == 0 {
        return 0;
    }
    let tipo = read_u8(c + 0x35) & 0x3f;
    if tipo == 5 {
        return 0;
    }
    let vendedor = friendly_club_by_id(read_usize(c + 4));
    if read_usize(comprador + 0x53) == 0 || read_usize(vendedor + 0x53) == 0 {
        return 0;
    }
    if tipo == 2 {
        copia(&mut data, &mut t, c + 0x2d);
    } else {
        copia(&mut data, &mut t, 0xae2c90);
    }
    let n = read_usize(KEY_NATION_SWEDEN);
    if read_usize(read_usize(vendedor + 0x53)) == n && read_usize(read_usize(comprador + 0x53)) == n {
        return valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2;
    }
    if idade(pessoa, &data) as u8 as i8 >= 0x18 {
        return 0;
    }
    if britanica(read_usize(comprador + 0x53)) == 0 || britanica(read_usize(vendedor + 0x53)) == 0 {
        return 0;
    }
    crate::comparacao::ramo("cpq_6 compensacao britanica");
    valor(gerente(), pessoa, vendedor, 1, 0xffff_ffff, 0) / 2 // menor de 24 anos entre clubes britânicos
}

/// adds the fee to the totals, the buyer pays and the seller receives the first instalment, stores the remaining
/// instalments (list +0x872) and the sell-on and appearance clauses, with the person's flags
/// Original: `TRANSFER_MANAGER::pay_fee_and_record_clauses` (0x8ca8a0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8ca8a0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::pay_fee_and_record_clauses(oferta)`: soma o
/// valor (+0x1b) aos totais das transferências (0x448ad0); o comprador (+8) paga e o vendedor (clube do contrato da
/// pessoa +0xc, 0x4cf5a0) recebe a primeira parcela (o valor / parcelas +0x40, ou tudo), em 64 bits (0x599890,
/// 0x599800). Com parcelas, guarda as que faltam (13 bytes: a parcela, os ids do comprador e do vendedor, parcelas − 1)
/// na lista +0x872 (0x68d0d0) e soma ao +0x1c das finanças a parcela × −5/21 (comprador) e × 5/21 (vendedor). Guarda
/// as cláusulas: a porcentagem da próxima venda (+0x3f, 0x8caaf0, marca 8), o prêmio por jogo no time (+0x35 com o byte
/// +0x39, 0x8cac40, marca 2) e o prêmio por jogo pela seleção (+0x3a com +0x3e, 0x8cad90, marca 4); as marcas no
/// registro de 8 bytes da pessoa em [this+0x86e].
pub(crate) unsafe extern "thiscall" fn pay_fee_and_record_clauses(this: usize, oferta: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_ADD_TRANSFER_TOTALS);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_PAY_PLAYERS_BOUGHT);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_ADD_PLAYERS_SOLD);
    let por_na_lista: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let proxima_venda: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_NEXT_SALE_PERCENTAGE);
    let premio_time: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_FIRST_TEAM_APPEARANCE_MONEY);
    let premio_selecao: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    let tabela = || read_usize(G_FINANCE_TABLE);
    let marcar = |bit: u8| {
        let m = (read_usize(this + 0x86e) + read_usize(oferta + 0xc) * 8) as *mut u8;
        *m |= bit;
    };
    let comprador = friendly_club_by_id(read_usize(oferta + 8));
    let vendedor = seller_club_of_person(staff_records_staff_by_id(read_usize(oferta + 0xc)));
    let fin_c = financas(tabela(), comprador);
    // ponytail: sem vendedor, a original leria as finanças de um lugar sem valor (e cairia adiante, em [vendedor]); a
    // cópia não paga ninguém
    let fin_v = if vendedor != 0 { financas(tabela(), vendedor) } else { 0 };
    let valor = read_usize(oferta + 0x1b) as i32;
    let n = read_u8(oferta + 0x40) as i8;
    let primeira = if n > 0 { (valor / n as i32) as i64 } else { valor as i64 };
    totais(read_usize(G_CLUB_RECORDS), valor, comprador, vendedor);
    crate::comparacao::ramo("cpq_6 pagar transferencia");
    pagar(fin_c, primeira);
    if fin_v != 0 {
        receber(fin_v, primeira); // o vendedor recebe
    }
    let mut r = [0u8; 13];
    let n = read_u8(oferta + 0x40) as i8;
    if n > 0 {
        let parcela = read_usize(oferta + 0x1b) as i32 / n as i32;
        put_u32_le(&mut r, 0, parcela as usize);
        put_u32_le(&mut r, 4, read_usize(comprador));
        put_u32_le(&mut r, 8, read_usize(vendedor));
        r[12] = (n as u8).wrapping_sub(1);
        let fc = financas(tabela(), comprador);
        let fv = financas(tabela(), vendedor);
        if fc != 0 {
            let s = (fc + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(match_pl_ftol(parcela as f64 * read_f64_at(0x970420)));
        }
        if fv != 0 {
            let s = (fv + 0x1c) as *mut i32;
            *s = (*s).wrapping_add(match_pl_ftol(parcela as f64 * read_f64_at(0x970418)));
        }
        por_na_lista(this + 0x872, r.as_ptr(), 0xd);
    }
    let p = read_u8(oferta + 0x3f);
    if (p as i8) > 0 {
        r[0] = p;
        put_u32_le(&mut r, 1, read_usize(comprador));
        put_u32_le(&mut r, 5, read_usize(vendedor));
        put_u32_le(&mut r, 9, read_usize(oferta + 0xc));
        proxima_venda(this, r.as_ptr());
        marcar(8);
    }
    let v = read_usize(oferta + 0x35) as i32;
    if v > 0 {
        put_u32_le(&mut r, 0, read_usize(oferta + 0xc));
        put_u32_le(&mut r, 4, v as usize);
        put_u32_le(&mut r, 9, read_usize(vendedor));
        r[8] = read_u8(oferta + 0x39);
        premio_time(this, r.as_ptr());
        marcar(2);
    }
    let v = read_usize(oferta + 0x3a) as i32;
    if v > 0 {
        put_u32_le(&mut r, 0, read_usize(oferta + 0xc));
        put_u32_le(&mut r, 4, v as usize);
        put_u32_le(&mut r, 9, read_usize(vendedor));
        r[8] = read_u8(oferta + 0x3e);
        premio_selecao(this, r.as_ptr());
        marcar(4);
    }
}

/// 1 when the fee covers an active release clause (reason 0x1b) or the seller in financial trouble gets far more than
/// the value (reason 0x1c); without a seller, 1
/// Original: `transfer_manager_selling_club_must_accept` (0x8cd240, transfer_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x8cd240 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só dos valores locais):
/// `transfer_manager_selling_club_must_accept(oferta, estrito, motivo)`: sem vendedor (clube do contrato da pessoa
/// +0xc que não é de tipo 5, 0x4cf5a0), 1. Com cláusula de rescisão ativa (0x867b50), sem troca (+4) nem parcelas
/// (+0x40) e o valor (+0x1b) não abaixo da multa (+0x21 do contrato; os dois por 0x43f460 com 4): motivo 0x1b e 1. Com
/// o vendedor em crise (+0x165 das finanças = 1), o preço pedido (0x4d1600) não abaixo de 20 × a reputação (+0x80;
/// pelo menos 20000), o valor ponderado da oferta (0x8e7b50) acima de 50000 e o dobro dele acima do preço: motivo 0x1c
/// e 1 (com `estrito`, só se o preço ficar abaixo de 1,3 × o valor ponderado). Senão 0.
pub(crate) unsafe extern "stdcall" fn transfer_manager_selling_club_must_accept(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let ativa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_HAS_ACTIVE_RELEASE_CLAUSE);
    let exibir: unsafe extern "thiscall" fn(*const u32, u32) -> i64 = core::mem::transmute(FN_CASH_GET_DISPLAY_AMOUNT);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let comprador = friendly_club_by_id(read_usize(oferta + 8));
    let pessoa = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let vendedor = seller_club_of_person(pessoa);
    if vendedor == 0 {
        return 1;
    }
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
    crate::comparacao::ramo("cpq_6 clube deve aceitar");
    if ativa(c) != 0 && read_usize(oferta + 4) == 0 && read_u8(oferta + 0x40) == 0 {
        let (multa, valor) = (read_usize(c + 0x21) as u32, read_usize(oferta + 0x1b) as u32);
        let m = exibir(&multa, 4);
        let v = exibir(&valor, 4);
        if v >= m {
            if !motivo.is_null() {
                *motivo = 0x1b;
            }
            return 1;
        }
    }
    if read_u8(financas(read_usize(G_FINANCE_TABLE), vendedor) + 0x165) != 1 {
        return 0;
    }
    let pedido = preco(read_usize(G_CONTRACT_MANAGER), vendedor, comprador, pessoa, 0, 1, 1);
    let w = ponderado(oferta, 0, 0, 1, 0, 1);
    let piso = (*((vendedor + 0x80) as *const i16) as i32 * 20).max(20000);
    if pedido < piso || w.wrapping_mul(2) <= pedido || w <= 50000 {
        return 0;
    }
    if estrito != 0 {
        let w2 = ponderado(oferta, 0, 0, 1, 0, 1);
        if !((pedido as f64) < w2 as f64 * read_f64_at(0x968de8)) {
            return 0;
        }
    }
    if !motivo.is_null() {
        *motivo = 0x1c;
    }
    1
}

/// Comparador: pelo valor (lado a lado pelas estáticas das datas; o valor do jogador 0x4d0c60 e a idade só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_calc_compensation_fee(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> i32>(IDX_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&CPQ6_ESTATICAS, || o(pessoa, comprador, sem_regras_novas), || transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_calc_compensation_fee(pessoa {:x}) {} x {}, estáticas iguais {}", if pessoa != 0 { read_usize(pessoa) } else { 0 }, ro, rr, igual));
    ro
}

/// Comparador: paga, acrescenta nas listas (aloca) e marca: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_pay_fee_and_record_clauses(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PAY_FEE_AND_RECORD_CLAUSES)(this, oferta)
}

/// Comparador: lado a lado pelo motivo e [0xae1080], e pelo valor devolvido (o preço e o valor ponderado só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_selling_club_must_accept(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, *mut u16) -> u32>(IDX_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let mut reg: Vec<(usize, usize)> = vec![(0xae1080, 4)];
    if !motivo.is_null() {
        reg.push((motivo as usize, 2));
    }
    reg.extend_from_slice(&CPQ6_ESTATICAS);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(oferta, estrito, motivo), || transfer_manager_selling_club_must_accept(oferta, estrito, motivo));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_selling_club_must_accept(estrito {}) {} x {}, memória igual {}", estrito, ro, rr, igual));
    ro
}

// ---------- 253a leva: os contratos da transferência: a permissão de trabalho na mudança de clube, concluir a oferta de contrato aceita e oferecer contrato ao jogador da oferta (transfer_manager.cpp) ----------





/// Person's work-permit record (14 bytes, sorted at [this+0x93a]) by bsearch; 0 if id out of range or flag 0x20 unset.
///
/// [pt] A permissão de trabalho da `pessoa` (14 bytes, ordenadas pelo id: [this+0x93a], conta +0x93e), como a original
/// procura: pessoa com id fora de [0xae2364] ou sem a marca (bit 0x20 do registro de 8 bytes em [this+0x86e]), 0; senão
/// a busca binária (0x946464 com 0x8e55c0).
pub(crate) unsafe fn find_work_permit_record(this: usize, pessoa: usize) -> usize {
    if pessoa != 0 && read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let id = read_usize(pessoa) as u32;
    if read_u8(read_usize(this + 0x86e) + id as usize * 8) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(&id, read_usize(this + 0x93a), read_usize(this + 0x93e), 0xe, 0x8e55c0)
}

/// when the new club needs a permit: updates the existing one (nation, year) or grants a new one (0x8c7070); otherwise:
/// removes any permit and re-sorts the array
/// Original: `TRANSFER_MANAGER::update_work_permit_on_move` (0x8c7e20, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c7e20 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só da data local):
/// `TRANSFER_MANAGER::update_work_permit_on_move(pessoa, clube)`: se o jogador precisa de permissão no clube novo
/// (0x8c7560 com 1) e já tem uma: troca a nação da permissão (+0xa) pela do clube (+0x53), zera +4 e +6 e, se mudou de
/// nação e as duas não são da marca 0xe (+0x76), grava o ano de hoje (+8); clube sem nação: a caixa "Error" 0x1dc2. Sem
/// permissão: a nova (0x8c7070 com o ano de hoje, ordenando), depois de hoje + 52 semanas (0x54c770, sem uso). Se não
/// precisa (a 0x8c7560 é chamada de novo) ou sem clube: tira a permissão que tiver (a marca 0x20, a última no lugar
/// dela, conta − 1 e reordena com 0x945333).
pub(crate) unsafe extern "thiscall" fn update_work_permit_on_move(this: usize, pessoa: usize, clube: usize) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if clube != 0 {
        if precisa(this, pessoa, clube, 1) != 0 {
            let r = find_work_permit_record(this, pessoa);
            if r != 0 {
                let nacao = read_usize(G_NATION_TABLE).wrapping_add(read_usize(r + 0xa).wrapping_mul(0x122));
                let nc = read_usize(clube + club::NATION);
                if nc == 0 {
                    show_error_at(0xa8d514, 0x1dc2);
                    return;
                }
                if nacao != nc && !(read_u8(nacao + nation::ACTUAL_REGION) == 0xe && read_u8(nc + 0x76) == 0xe) {
                    core::ptr::write_unaligned((r + 8) as *mut u16, *((0xae2c92) as *const u16));
                }
                core::ptr::write_unaligned((r + 4) as *mut u16, 0);
                core::ptr::write_unaligned((r + 6) as *mut u16, 0);
                core::ptr::write_unaligned((r + 0xa) as *mut u32, read_usize(nc) as u32);
                return;
            }
            let semanas: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
            let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_WORK_PERMIT);
            let mut t = [0u32; 2];
            semanas(0xae2c90, &mut t, 0x34);
            crate::comparacao::ramo("cpq_7 permissao nova");
            // ponytail: a original empurra o ano com lixo nos 16 bits altos; a 0x8c7070 só grava a palavra
            let ano = *((0xae2c92) as *const u16) as u32;
            dar(this, pessoa, read_usize(clube + club::NATION), ano, 0, 1); // a permissão nova
            return;
        }
        if precisa(this, pessoa, clube, 1) != 0 {
            return;
        }
    }
    let r = find_work_permit_record(this, pessoa);
    if r == 0 {
        return;
    }
    let marca = (read_usize(this + 0x86e) + read_usize(pessoa) * 8) as *mut u8;
    *marca &= 0xdf;
    let ultima = read_usize(this + 0x93a) + read_usize(this + 0x93e) * 0xe - 0xe;
    core::ptr::copy(ultima as *const u8, r as *mut u8, 0xe);
    let n = read_usize(this + 0x93e).wrapping_sub(1);
    core::ptr::write_unaligned((this + 0x93e) as *mut usize, n);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0x93a), n as u32, 0xe, 0x8e55c0);
}

/// without the work permit, the news of the collapsed offer or the refused permit and 0; otherwise signs (0x4cf720),
/// the club pays the signing-on fee and 1
/// Original: `TRANSFER_MANAGER::complete_accepted_contract_offer` (0x8c38b0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c38b0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::complete_accepted_contract_offer(oferta)`: oferta de contrato (+0 a pessoa, +4 o clube, +8 as
/// luvas, +0x61 o cargo). Se o jogador precisa de permissão no clube (0x8c7560 com 1) e ela não sai (0x8c7950): com
/// contrato de tipo 5 (0x4cf5a0, guardado em [0xae1080]), a notícia da oferta que caiu (0x8de790 com uma
/// TRANSFER_OFFER local: 0x8e5680, ids em +8 e +0xc, +0x1f = 0, motivo 0x13; 0x8e5710); senão a notícia da permissão
/// recusada (0x8e4290); devolve 0. Senão assina (0x4cf720) e, assinado, o clube paga as luvas (0x5a0590, 0x5998e0) e
/// devolve 1.
pub(crate) unsafe extern "thiscall" fn complete_accepted_contract_offer(this: usize, oferta: usize) -> u32 {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_WORK_PERMIT_GRANTED);
    let pessoa = staff_records_staff_by_id(read_usize(oferta));
    let clube = friendly_club_by_id(read_usize(oferta + 4));
    if precisa(this, pessoa, clube, 1) != 0 && concedida(this, pessoa, read_usize(clube + club::NATION), oferta, 0, 0) == 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        if c != 0 && read_u8(c + 0x35) & 0x3f == 5 {
            let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
            let caiu: unsafe extern "thiscall" fn(usize, *mut u8, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_COLLAPSED_NEWS);
            let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
            let mut o = [0u8; 0x44];
            construir(o.as_mut_ptr());
            o[8..12].copy_from_slice(&(read_usize(clube) as u32).to_le_bytes());
            o[0xc..0x10].copy_from_slice(&(read_usize(pessoa) as u32).to_le_bytes());
            o[0x1f] = 0;
            caiu(this, o.as_mut_ptr(), clube, 0x13);
            destruir(o.as_mut_ptr());
        } else {
            let recusada: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_PERMIT_REFUSED_NEWS);
            recusada(this, pessoa, oferta);
        }
        return 0;
    }
    let assinar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SIGN_CONTRACT);
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x4cf720 e a 0x53bd10 só usam o byte
    if assinar(read_usize(G_CONTRACT_MANAGER), oferta, read_u8(oferta + 0x61) as u32) == 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let luvas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FINANCE_PAY_SIGN_ON_FEE);
    let valor = read_usize(oferta + 8) as u32;
    let fin = financas(read_usize(G_FINANCE_TABLE), clube);
    crate::comparacao::ramo("cpq_7 luvas");
    luvas(fin, valor); // as luvas
    1
}

/// builds the club's contract (0x4cda70, factor 1) and adds the contract offer; without one, withdraws the offer (main
/// player) or drops the player from the exchange adding his value
/// Original: `TRANSFER_MANAGER::offer_contract_to_offer_player` (0x8c1e40, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c1e40 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH do contrato local):
/// `TRANSFER_MANAGER::offer_contract_to_offer_player(clube, pessoa, oferta)`: monta o contrato (STAFF_CONTRACT local,
/// 0x85e410) que o clube ofereceria (0x4cda70 com a vaga +0x20 da oferta, fator de salário 1 e modo 2). Montado:
/// acrescenta a oferta de contrato (0x4cc490 com a vaga +0x20, o valor +0x24, 0xb, 0, 1, 0). Sem contrato: sendo o
/// jogador principal (+0xc), atualiza a lista do clube (0x849ce0 e 0x843e10 com 1) e retira a oferta (0x8bfcb0, motivo
/// 0x16); sendo da troca, soma o valor dele (+0x52) ao da oferta (+0x1b) e o tira da troca (0x8c41c0 com 1 e 0x16).
/// Devolve o que a última chamada devolveu.
pub(crate) unsafe extern "thiscall" fn offer_contract_to_offer_player(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let oferecer: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, f64, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    let mut c = [0u8; 0x50];
    construir(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_7 contrato da oferta");
    if oferecer(read_usize(G_CONTRACT_MANAGER), pessoa, clube, c.as_mut_ptr(), read_usize(oferta + 0x20), 1.0f64, 2) != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
        return acrescentar(read_usize(G_CONTRACT_MANAGER), c.as_mut_ptr(), read_usize(oferta + 0x20), read_usize(oferta + 0x24), 0xb, 0, 1, 0);
    }
    if read_usize(pessoa) == read_usize(oferta + 0xc) {
        let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
        let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
        let e = entrada(read_usize(this + 0x856), clube, pessoa, (read_u8(oferta + 0x11) != 0xb) as u32, 0);
        if e != 0 {
            // ponytail: a original empurra o tipo (setne) com o resto do ponteiro da entrada acima; a 0x843e10 só lê o byte
            atualizar(read_usize(this + 0x856), pessoa, clube, 1, (read_u8(e) != 0xb) as u32);
        }
        let mut o = oferta;
        return retirar(this, &mut o, 0, 0x16);
    }
    let v = (oferta + 0x1b) as *mut i32;
    core::ptr::write_unaligned(v, core::ptr::read_unaligned(v).wrapping_add(read_usize(pessoa + staff::VALUE) as i32));
    let tirar_da_troca: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_EXCHANGE_PLAYER);
    tirar_da_troca(read_usize(G_TRANSFER_MANAGER), pessoa, oferta, 1, 0x16)
}

/// Comparador: grava as permissões (pode crescer o vetor e reordena): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_update_work_permit_on_move(this: usize, pessoa: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_WORK_PERMIT_ON_MOVE)(this, pessoa, clube)
}

/// Comparador: assina o contrato (aloca), paga e posta notícias: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_complete_accepted_contract_offer(this: usize, oferta: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_COMPLETE_ACCEPTED_CONTRACT_OFFER)(this, oferta)
}

/// Comparador: acrescenta ofertas de contrato (aloca), mexe nas listas e retira ofertas: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_offer_contract_to_offer_player(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_OFFER_CONTRACT_TO_OFFER_PLAYER)(this, clube, pessoa, oferta)
}

// ---------- 253a leva: as ofertas de transferência: desfazer a oferta, fazer a oferta de empréstimo e concluir as transferências futuras do dia (transfer_manager.cpp) ----------





/// Offer -> contract of its target person (+0xc), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa da oferta (+0xc; 0x4cf5a0 com o gerente [0xae1050]), guardado em [0xae1080].
pub(crate) unsafe fn offer_target_contract(oferta: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Offer -> club of the target's contract unless type 5.
///
/// [pt] O clube do contrato da pessoa da oferta; contrato de tipo 5 (+0x35 & 0x3f) não conta.
pub(crate) unsafe fn offer_target_seller_club(oferta: usize) -> usize {
    let c = offer_target_contract(oferta);
    if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        friendly_club_by_id(read_usize(c + 4))
    } else {
        0
    }
}

/// Rewinds a LLIST cursor (+0xa = +2, first node).
///
/// [pt] Recomeça o cursor da LLIST (+0xa = +2, o primeiro nó).
pub(crate) unsafe fn llist_restart(lista: usize) {
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
}

/// Offer status that lets the person's mark 1 clear: 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3.
///
/// [pt] O estado da oferta (+0x2c) que deixa a marca 1 da pessoa sair: 2, 3, 0x14, 0x10, 4 e 0xf com +0x2e ≥ 3.
pub(crate) unsafe fn is_offer_state_closed(oferta: usize) -> bool {
    match read_u8(oferta + 0x2c) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => read_u8(oferta + 0x2e) >= 3,
        _ => false,
    }
}

/// state 0x10, closes the proposals, releases the bids, posts the news, schedules the removal, clears the flags, the
/// contract offer and the exchange contracts
/// Original: `TRANSFER_MANAGER::collapse_offer` (0x8c44e0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8c44e0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::collapse_offer(oferta, clube, motivo,
/// tirar_futura)`: na oferta de estado 0x13 tira a marca 0x10 da pessoa (+0xc) e dos jogadores da troca (lista +4) e,
/// com `tirar_futura`, tira a transferência futura (0x8cbf70). Põe o estado 0x10, fecha as propostas (0x8c6db0), solta
/// os lances do comprador (+8) pela pessoa e do vendedor (clube do contrato, guardado em [0xae1080]) pelos da troca
/// (0x84aeb0), posta a notícia (0x8de790 com o clube e o motivo) e agenda a oferta (0x8c6220 com 0x14). Encerrada a
/// oferta e todas as da lista +0 (estados de `is_offer_state_closed`), tira a marca 1 da pessoa. Atualiza a lista do
/// comprador (0x849ce0 e 0x843e10 com 1), tira a oferta de contrato (0x4cca70), zera +0x10 e solta os contratos da troca
/// (0x4ccb30 com a vaga +0x20; +4 do item = 0).
pub(crate) unsafe extern "thiscall" fn collapse_offer(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let marca = |id: usize| (read_usize(this + 0x86e) + id * 8) as *mut u8;
    crate::comparacao::ramo("cpq_8 desfazer oferta");
    if read_u8(oferta + 0x2c) == 0x13 {
        *marca(read_usize(oferta + 0xc)) &= 0xef;
        if read_usize(oferta + 4) != 0 {
            llist_restart(read_usize(oferta + 4));
            loop {
                let it = proximo(read_usize(oferta + 4));
                if it == 0 {
                    break;
                }
                *marca(read_usize(it)) &= 0xef;
            }
        }
        if tirar_futura != 0 {
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FUTURE_TRANSFER);
            tirar(this, oferta);
        }
    }
    *((oferta + 0x2c) as *mut u8) = 0x10;
    let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    fechar(this, oferta);
    let pessoa = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let c = offer_target_contract(oferta);
    let vendedor = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    desmarcar(read_usize(this + 0x856), friendly_club_by_id(read_usize(oferta + 8)), pessoa);
    if read_usize(oferta + 4) != 0 {
        llist_restart(read_usize(oferta + 4));
        loop {
            let it = proximo(read_usize(oferta + 4));
            if it == 0 {
                break;
            }
            desmarcar(read_usize(this + 0x856), vendedor, staff_records_staff_by_id(read_usize(it)));
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_COLLAPSED_NEWS);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    noticia(this, oferta, clube, motivo);
    agendar(this, oferta, 0x14, 0);
    if is_offer_state_closed(oferta) {
        let mut todas = true;
        if read_usize(oferta) != 0 {
            llist_restart(read_usize(oferta));
            loop {
                let it = proximo(read_usize(oferta));
                if it == 0 {
                    break;
                }
                if !is_offer_state_closed(read_usize(read_usize(this) + read_usize(it) * 4)) {
                    todas = false;
                    break;
                }
            }
        }
        if todas {
            let m = marca(read_usize(oferta + 0xc));
            if *m & 1 != 0 {
                *m &= 0xfe;
            }
        }
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let e = entrada(read_usize(this + 0x856), friendly_club_by_id(read_usize(oferta + 8)), staff_records_staff_by_id(read_usize(oferta + 0xc)), (read_u8(oferta + 0x11) != 0xb) as u32, 0);
    if e != 0 {
        // ponytail: a original empurra o tipo (setne) com lixo nos bytes altos; a 0x843e10 só lê o byte
        atualizar(read_usize(this + 0x856), staff_records_staff_by_id(read_usize(oferta + 0xc)), friendly_club_by_id(read_usize(oferta + 8)), 1, (read_u8(e) != 0xb) as u32);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    tirar_contrato(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)), friendly_club_by_id(read_usize(oferta + 8))); // a oferta de contrato sai
    *((oferta + 0x10) as *mut u8) = 0;
    if read_usize(oferta + 4) != 0 {
        let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
        llist_restart(read_usize(oferta + 4));
        loop {
            let it = proximo(read_usize(oferta + 4));
            if it == 0 {
                break;
            }
            soltar(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(it)), read_usize(oferta + 0x20));
            *((it + 4) as *mut u8) = 0;
        }
    }
}

/// builds the loan offer and the contract from the current one (wage percentage), adds both and returns the offer slot;
/// in DESLIGADAS since 2026-10-30: with the detour the career in Rust mode left the reference be54a138 (bisection of
/// the 38 routines of batch 253); the original builds the offer and the contract in locals the constructors (0x8e5680,
/// 0x85e410) do not fully fill, and the stack residue of those bytes goes into the stored offer, where the copy puts
/// zero; the copy stays in the code and does not run
/// Original: `TRANSFER_MANAGER::make_loan_offer` (0x8bed60, transfer_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x8bed60 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::make_loan_offer(jogador, clube, fim, percentual, a5, a6)`: numa TRANSFER_OFFER local (0x8e5680)
/// monta a oferta de empréstimo (0x8e7810) e, num STAFF_CONTRACT local (0x85e410), o contrato pelo atual (0x85e590 com o
/// fim, a porcentagem do salário e as duas cláusulas); acrescenta a oferta (0x8be300 com o contrato) e a oferta de
/// contrato (0x4cc490 com a vaga +0x20 e o valor +0x24 da oferta, 0xb, 0, 1, 0). Destrói a oferta local (0x8e5710) e
/// devolve a vaga (+0x20).
pub(crate) unsafe extern "thiscall" fn make_loan_offer(this: usize, jogador: usize, clube: usize, fim: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let emprestimo: unsafe extern "thiscall" fn(*mut u8, usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_LOAN_OFFER);
    let construir_contrato: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let pelo_atual: unsafe extern "thiscall" fn(*mut u8, usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_INIT_FROM_CURRENT_CONTRACT);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_OFFER);
    let acrescentar_contrato: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
    let destruir: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let campo = |o: &[u8; 0x44], k: usize| u32::from_le_bytes([o[k], o[k + 1], o[k + 2], o[k + 3]]);
    let mut o = [0u8; 0x44];
    construir(o.as_mut_ptr());
    emprestimo(o.as_mut_ptr(), jogador, clube);
    let mut c = [0u8; 0x50];
    construir_contrato(c.as_mut_ptr());
    crate::comparacao::ramo("cpq_8 oferta de emprestimo");
    pelo_atual(c.as_mut_ptr(), jogador, clube, fim, write_percent_pair, a5, a6); // o salário pela porcentagem
    acrescentar(this, o.as_mut_ptr(), c.as_mut_ptr());
    acrescentar_contrato(read_usize(G_CONTRACT_MANAGER), c.as_mut_ptr(), campo(&o, 0x20), campo(&o, 0x24), 0xb, 0, 1, 0);
    let vaga = campo(&o, 0x20);
    destruir(o.as_mut_ptr());
    vaga
}

/// the future transfers whose date has come: collapses those the restrictions block, completes the others (0x8c9390)
/// and removes them from the array
/// Original: `TRANSFER_MANAGER::process_due_future_transfers` (0x8cbbe0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cbbe0 } (thiscall, sem argumentos, transfer_manager.cpp): `TRANSFER_MANAGER::process_due_future_transfers()`:
/// percorre as transferências futuras (12 bytes: +0 a data, +8 o índice da oferta; [this+0x920], conta +0x928) até a
/// primeira depois de hoje (0x456740). Oferta nula: a caixa "Error" 0x2846. Com as permissões (0x8c7690 com 1): se as
/// restrições (0x8cfc90) seguram pelo comprador ou, havendo vendedor, pelo vendedor (motivo 9 não conta), desfaz a
/// oferta (0x8c44e0 com o clube e o motivo); senão a notícia (0x8dbef0), conclui (0x8c9390), estado 0x14 e agenda
/// (0x8c6220 com 0xa). Depois tira as processadas do começo do vetor (0x8cbf40 uma a uma) e diminui a conta.
pub(crate) unsafe extern "thiscall" fn process_due_future_transfers(this: usize) {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_WORK_PERMITS);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_RESTRICTIONS);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFER);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_COMPLETED_NEWS);
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_FINALISE_TRANSFER);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    // o motivo das restrições: só a palavra baixa é gravada; os 16 bits altos ficam 0, como na original
    let mut motivo = 0u32;
    let mut feitas = 0i32;
    while feitas < read_usize(this + 0x928) as i32 {
        let e = read_usize(this + 0x920) + feitas as usize * 0xc;
        if depois(e, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 {
            break;
        }
        let oferta = read_usize(read_usize(this) + read_usize(e + 8) * 4);
        if oferta == 0 {
            show_error_at(0xa8d514, 0x2846);
        } else if permissoes(this, oferta, 1) != 0 {
            let comprador = friendly_club_by_id(read_usize(oferta + 8));
            let mut segura = false;
            if restricoes(this, oferta, comprador, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                desfazer(this, oferta, comprador, motivo, 0);
                segura = true;
            } else if offer_target_seller_club(oferta) != 0 {
                let v = offer_target_seller_club(oferta);
                if restricoes(this, oferta, v, &mut motivo, 0) as u8 != 0 && motivo as u16 != 9 {
                    let v = offer_target_seller_club(oferta);
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
        show_error_at(0xa8d514, 0x288a);
    }
    if feitas == 0 {
        return;
    }
    let mut k = feitas;
    if k < read_usize(this + 0x928) as i32 {
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FUTURE_TRANSFER_ASSIGN);
        while k < read_usize(this + 0x928) as i32 {
            let v = read_usize(this + 0x920);
            copiar(v + (k - feitas) as usize * 0xc, v + k as usize * 0xc);
            k += 1;
        }
    }
    let n = (this + 0x928) as *mut i32;
    core::ptr::write_unaligned(n, core::ptr::read_unaligned(n).wrapping_sub(feitas));
}

/// Comparador: muda o estado, as listas e as marcas, posta notícia e agenda: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_collapse_offer(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_COLLAPSE_OFFER)(this, oferta, clube, motivo, tirar_futura)
}

/// Comparador: acrescenta a oferta e a oferta de contrato (alocam): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_make_loan_offer(this: usize, jogador: usize, clube: usize, fim: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32>(IDX_MAKE_LOAN_OFFER)(this, jogador, clube, fim, write_percent_pair, a5, a6)
}

/// Comparador: conclui e desfaz transferências (elencos, dinheiro, notícias): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_process_due_future_transfers(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_DUE_FUTURE_TRANSFERS)(this)
}

// ---------- 252a leva: as ofertas de transferência: o clube recusa a oferta, soltar os jogadores da oferta e montar a oferta em dinheiro (transfer_manager.cpp) ----------





/// counts the offer's attempt; by the refusing side, a draw(2 × attempts) rejects or withdraws the offer, otherwise it
/// is rescheduled with the delay news
/// Original: `TRANSFER_MANAGER::club_rejects_offer` (0x8c36b0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c36b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::club_rejects_offer(oferta, clube,
/// motivo)`: fora dos estados 0x11/0x12 (+0x2c), conta a tentativa (+0x2e). Recusa do clube vendedor (id = +8): estado
/// 0x12; motivo 8 (16 bits) reagenda (0x8c6220 com 7, 0) e sai; senão, sem clube humano no contrato do jogador
/// (+0xc; contrato em [0xae1080], +0x35 & 0x3f ≠ 5, 0x600a50) e com mais de uma tentativa, um sorteio(2 × tentativas)
/// diferente de 0 rejeita a oferta (0x8bf680 com o endereço da oferta, 0, 6) e sai. Recusa do outro lado: estado
/// 0x11, desassossega o jogador (0x8c1290); com o clube vendedor do computador e o sorteio, retira a oferta (0x8bfcb0);
/// com motivo 8 o resto reagenda e sai (sem o teste de tentativas). No fim: reagenda e dá a notícia do adiamento
/// (0x8dda90 com o motivo inteiro, como a original o empurra).
pub(crate) unsafe extern "thiscall" fn club_rejects_offer(this: usize, oferta: usize, clube: usize, motivo: u32) {
    let estado = read_u8(oferta + 0x2c);
    if estado == 0x11 || estado == 0x12 {
        return;
    }
    crate::comparacao::ramo("cpp_7 recusa");
    *((oferta + 0x2e) as *mut u8) = read_u8(oferta + 0x2e).wrapping_add(1); // mais uma tentativa
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    // a original passa o endereço do próprio argumento
    let mut o = oferta;
    if read_usize(clube) == read_usize(oferta + 8) {
        *((oferta + 0x2c) as *mut u8) = 0x12;
        if motivo as u16 == 8 {
            agendar(this, oferta, 7, 0);
            return;
        }
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)));
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let mut no_fim = false;
        if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
            let cl = friendly_club_by_id(read_usize(c + 4));
            no_fim = cl != 0 && humano(cl, 0, 1, 0, 0) != 0;
        }
        if !no_fim {
            let n = read_u8(oferta + 0x2e);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                let rejeitar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
                rejeitar(this, &mut o, 0, 6);
                return;
            }
        }
    } else {
        *((oferta + 0x2c) as *mut u8) = 0x11;
        let desassossegar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UNSETTLE_PLAYER_AFTER_BID);
        desassossegar(this, oferta);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
        let cl = friendly_club_by_id(read_usize(oferta + 8));
        if motivo as u16 == 8 {
            if humano(cl, 0, 1, 0, 0) == 0 && sorteio(read_u8(oferta + 0x2e) as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
            agendar(this, oferta, 7, 0);
            return;
        }
        if humano(cl, 0, 1, 0, 0) == 0 {
            let n = read_u8(oferta + 0x2e);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_DELAY_REQUEST_NEWS);
    agendar(this, oferta, 7, 0);
    noticia(this, oferta, motivo);
}

/// takes the offer's player and each exchange player out of other offers and collapses the offers for each exchange
/// player
/// Original: `TRANSFER_MANAGER::detach_offer_players` (0x8c6ae0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c6ae0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// detach_offer_players(oferta)`: constrói uma LLIST local que não usa (0x68d020, destruída no fim por 0x68d040), tira
/// das outras ofertas o jogador da oferta (+0xc; 0x8c41c0 com a oferta, 0, 0) e, pela lista dos jogadores de troca
/// (+4, relida a cada chamada, cursor recomeçado), tira cada um (0x8c41c0) e desfaz as ofertas por ele (0x8c47d0). O
/// id do jogador de troca é relido do item depois da 0x8c41c0, como na original.
pub(crate) unsafe extern "thiscall" fn detach_offer_players(this: usize, oferta: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_EXCHANGE_PLAYER);
    let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFERS_FOR_PLAYER);
    let mut local = [0u32; 4]; // a LLIST (0xe bytes) da original
    lista_nova(local.as_mut_ptr() as usize);
    crate::comparacao::ramo("cpp_7 soltar jogadores");
    tirar(this, staff_records_staff_by_id(read_usize(oferta + 0xc)), oferta, 0, 0); // o jogador da oferta
    let trocas = read_usize(oferta + 4);
    if trocas != 0 {
        core::ptr::write_unaligned((trocas + 0xa) as *mut usize, read_usize(trocas + 2));
        let mut item = proximo(read_usize(oferta + 4));
        while item != 0 {
            tirar(this, staff_records_staff_by_id(read_usize(item)), oferta, 0, 0);
            desfazer(this, staff_records_staff_by_id(read_usize(item)));
            item = proximo(read_usize(oferta + 4));
        }
    }
    destruir(local.as_mut_ptr() as usize);
}

/// allocates and initialises a cash offer for the player at the given fee (0x8e78f0), stores it in the output, appeal
/// 50, and returns 1
/// Original: `TRANSFER_MANAGER::build_cash_offer_option` (0x8c8740, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c8740 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH só para a oferta nova):
/// `TRANSFER_MANAGER::build_cash_offer_option(clube, jogador, saída, atração, valor)`: aloca a oferta (0x944cf1(0x41) e
/// 0x8e5680), guarda em `saída`, inicia pelo jogador, o clube comprador e o valor (0x8e78f0, que pode sortear) e põe a
/// atração em 50. Clube sem finanças (0x5a0590 = 0): a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn build_cash_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    // ponytail: a 0x5a0590 só procura; consultada antes de alocar para o caso de erro ir inteiro pela original
    if financas(read_usize(G_FINANCE_TABLE), clube) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(IDX_BUILD_CASH_OFFER_OPTION)(this, clube, jogador, saida, atracao, valor);
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_TRANSFER_OFFER);
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
pub(crate) unsafe extern "thiscall" fn comparar_club_rejects_offer(this: usize, oferta: usize, clube: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_CLUB_REJECTS_OFFER)(this, oferta, clube, motivo)
}

/// Comparador: tira jogadores de ofertas e desfaz ofertas (libera itens): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_detach_offer_players(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DETACH_OFFER_PLAYERS)(this, oferta)
}

/// Comparador: aloca a oferta e a 0x8e78f0 pode sortear: roda só a original e confere o resultado: devolve 1, a oferta
/// guardada em `saída` tem o valor (+0x1b) e a atração ficou em 50.
pub(crate) unsafe extern "thiscall" fn comparar_build_cash_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(IDX_BUILD_CASH_OFFER_OPTION)(this, clube, jogador, saida, atracao, valor);
    let oferta = *saida;
    let ok = r == 1 && oferta != 0 && core::ptr::read_unaligned((oferta + 0x1b) as *const i32) == valor && *atracao == 0x32;
    PLACAR.registrar(ok, || format!("build_cash_offer_option(jogador {:x}, valor {}) devolveu {}, oferta {:x}", read_usize(jogador), valor, r, oferta));
    r
}

// ---------- 251a leva: as transferências: o prazo da resposta à oferta, pagar as prestações do mês e tirar a oferta (transfer_manager.cpp) ----------








/// Transfer rules of nation `nacao` among the 28 at this+0x8ac (matching +0xa), or 0.
///
/// [pt] A regra de transferência da nação `nacao` entre as 28 de `this + 0x8ac` (a de +0xa igual), ou 0.
pub(crate) unsafe fn nation_transfer_rules_or_zero(this: usize, nacao: usize) -> usize {
    for k in 0..0x1cusize {
        let r = read_usize(this + 0x8ac + k * 4);
        if read_usize(r + 0xa) == nacao {
            return r;
        }
    }
    0
}

/// from the time until the window closes (the rules of the buyer's and the player club's nations): 0 days, 1 or 2 days
/// with drawn slices, or a value from the player's interest (0x846920) or drawn from the status; days = value / 3,
/// slices = value % 3
/// Original: `TRANSFER_MANAGER::get_offer_response_delay` (0x8cc5b0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8cc5b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::get_offer_response_delay(oferta, dias,
/// fatias)`: o prazo é o de fechar a janela (0x8e8ed0) da nação do comprador (+8; sem regra: 365) e, com o +4 da oferta,
/// o menor com o da nação do clube do contrato do jogador (+0xc; contrato de tipo 5 não conta). Abaixo de 3: 0 dia e 1
/// fatia; abaixo de 6 ou de 11: 1 ou 2 dias e 1..3 fatias sorteadas. Senão o valor pelo estatuto do contrato com o clube
/// (+0x3d; sem ele −100): o jogador (0x53b620 do +0x11; sem ficha, a caixa "Error") de habilidade +7 da ficha acima de
/// 130 e da reputação do clube / 60, menos de 26 anos, em menos de 3 listas e estatuto positivo, com o interesse
/// (0x846920) acima de [0x968e58]: o prazo × [0x968e18] até [0x968dd8]; senão um sorteio até estatuto/10 + 12 (o maior
/// com 1..3). Dias = valor / 3, fatias = valor % 3.
pub(crate) unsafe extern "thiscall" fn get_offer_response_delay(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let janela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_GET_DAYS_UNTIL_WINDOW_CLOSES);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let comprador = friendly_club_by_id(read_usize(oferta + 8));
    let p = staff_records_staff_by_id(read_usize(oferta + 0xc));
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let dono = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    let k = com_clube(read_usize(G_CONTRACT_MANAGER), p, dono);
    let mut prazo: u16 = 0x16d;
    let r = nation_transfer_rules_or_zero(this, read_usize(comprador + 0x53));
    if r != 0 {
        prazo = janela(r) as u16;
    }
    if read_usize(oferta + 4) != 0 {
        let r = nation_transfer_rules_or_zero(this, read_usize(dono + 0x53));
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
    let estatuto: i8 = if k != 0 { read_u8(k + 0x3d) as i8 } else { -100 };
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x53b620 só lê o byte
    let e_jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    if e_jogador(read_u8(oferta + 0x11) as u32) != 0 && read_usize(p + 0x61) == 0 {
        show_error_at(0xa8d514, 0x29c3);
    }
    let mut valor: Option<u8> = None;
    if dono != 0 && e_jogador(read_u8(oferta + 0x11) as u32) != 0 {
        let hab = read_i16(read_usize(p + 0x61) + 7);
        if hab > 0x82
            && hab as i32 > read_i16(dono + 0x80) as i32 / 60
            && (read_u8(p + 0x18) as i8) < 0x1a
            && read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 0xb) & 0xf < 3
            && estatuto > 0
        {
            let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
            if interesse(read_usize(this + 0x856), p, comprador, 0, 0, 1, 0) > read_f64_at(0x968e58) {
                let x = prazo as i32 as f64 * read_f64_at(0x968e18);
                let teto = read_f64_at(0x968dd8);
                valor = Some(match_pl_ftol(if teto < x || x.is_nan() { teto } else { x }) as u8);
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

/// pays each instalment (payer's and receiver's finances), adds the fractions to the instalment figure (+0x1c) in the
/// last 8, removes the finished ones and zeroes the figure of clubs without an instalment in its direction
/// Original: `TRANSFER_MANAGER::pay_transfer_instalments` (0x8caee0, transfer_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x8caee0 } (thiscall, ret 0, transfer_manager.cpp): `TRANSFER_MANAGER::pay_transfer_instalments()`: no fim do mês,
/// cada prestação da lista (+0x872; 13 bytes: +0 o valor, +4 o clube que paga, +8 o que recebe, +0xc as que faltam) é
/// paga (0x599890 nas finanças do pagador, 0x599800 nas do recebedor, 0x5a0590) e conta uma a menos; faltando menos de
/// 4 ou de 8, a figura das prestações (+0x1c das finanças) recebe as frações ([0x968918]/[0x968f30] ou
/// [0x967880]/[0x969518]); acabada, sai da lista (0x68d1f0); senão os dois clubes ficam marcados (0x945501). Depois, em
/// cada clube com finanças, a figura zera: sem marca; positiva só pagando; negativa só recebendo.
pub(crate) unsafe extern "thiscall" fn pay_transfer_instalments(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_PAY_PLAYERS_BOUGHT);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_ADD_PLAYERS_SOLD);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let recebe = calloc(read_usize(G_CLUB_COUNT) as u32, 1);
    let paga = calloc(read_usize(G_CLUB_COUNT) as u32, 1);
    let lista = this + 0x872;
    write_u32(lista + 0xa, read_usize(this + 0x874) as u32);
    loop {
        let e = proximo(lista);
        if e == 0 {
            break;
        }
        let fr = financas(read_usize(G_FINANCE_TABLE), friendly_club_by_id(read_usize(e + 8)));
        let fp = financas(read_usize(G_FINANCE_TABLE), friendly_club_by_id(read_usize(e + 4)));
        pagar(fp, read_i32(e) as i64);
        receber(fr, read_i32(e) as i64);
        let n = (read_u8(e + 0xc) as i8).wrapping_sub(1);
        *((e + 0xc) as *mut i8) = n;
        let fracoes = if n < 4 { Some((0x968918, 0x968f30)) } else if n < 8 { Some((0x967880, 0x969518)) } else { None };
        if let Some((a, b)) = fracoes {
            write_u32(fp + 0x1c, read_i32(fp + 0x1c).wrapping_add(match_pl_ftol(read_i32(e) as f64 * read_f64_at(a))) as u32);
            write_u32(fr + 0x1c, read_i32(fr + 0x1c).wrapping_add(match_pl_ftol(read_i32(e) as f64 * read_f64_at(b))) as u32);
        }
        if read_u8(e + 0xc) == 0 {
            crate::comparacao::ramo("cpo_10 prestacao acabada");
            tirar(lista, e);
        } else {
            *((recebe + read_usize(friendly_club_by_id(read_usize(e + 8)))) as *mut u8) = 1;
            *((paga + read_usize(friendly_club_by_id(read_usize(e + 4)))) as *mut u8) = 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < read_i32(0xae235c) {
        let f = financas(read_usize(G_FINANCE_TABLE), read_usize(G_CLUB_TABLE).wrapping_add(i * 0x245));
        if f != 0 {
            let (r, p) = (read_u8(recebe + i) != 0, read_u8(paga + i) != 0);
            let v = read_i32(f + 0x1c);
            let zerar = match (r, p) {
                (false, false) => true,
                (false, true) => v > 0,
                (true, false) => v < 0,
                (true, true) => false,
            };
            if zerar {
                write_u32(f + 0x1c, 0);
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

/// destroys the linked offer, removes the pair from the loan list and the contract offers of the exchange and of the
/// player, lowers the player's (+0x86e) and the manager's offer counts, the lowest free slot follows and the offer is
/// destroyed
/// Original: `TRANSFER_MANAGER::remove_offer` (0x8d0f60, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0f60 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_offer(oferta)`: a oferta ligada
/// (+0x28 ≠ −1, em +0xc) é destruída (0x8e5710, 0x944c94), a vaga zera e a conta +0x14 baixa; com o +0x1f em 1, sai da
/// lista de empréstimos (+0x946) o par (clube +8, jogador +0xc); para cada jogador da troca (+4): a proposta de contrato
/// ligada à vaga +0x20 sai (0x4ccb30, se o jogador da oferta tem contrato) e o +2 do registro de 8 bytes dele (+0x86e)
/// baixa; sai a proposta de contrato do jogador com o comprador (0x4cca70), o +1 do registro do jogador baixa, a menor
/// vaga livre (+0x85e) acompanha, a vaga +0x20 do vetor (+0) zera, a oferta é destruída e a conta +8 baixa.
pub(crate) unsafe extern "thiscall" fn remove_offer(this: usize, oferta: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    crate::comparacao::ramo("cpo_10 tirar oferta");
    if read_i32(oferta + 0x28) != -1 {
        let x = read_usize(read_usize(this + 0xc).wrapping_add((read_i32(oferta + 0x28) as usize).wrapping_mul(4)));
        if x != 0 {
            destruir(x);
            apagar(x);
        }
        write_u32(read_usize(this + 0xc).wrapping_add((read_i32(oferta + 0x28) as usize).wrapping_mul(4)), 0);
        write_u32(this + 0x14, read_i32(this + 0x14).wrapping_sub(1) as u32);
    }
    if read_u8(oferta + 0x1f) == 1 && read_usize(this + 0x946) != 0 {
        let l = read_usize(this + 0x946);
        write_u32(l + 0xa, read_usize(l + 2) as u32);
        loop {
            let e = proximo(read_usize(this + 0x946));
            if e == 0 {
                break;
            }
            if read_usize(e + 4) == read_usize(oferta + 8) && read_usize(e) == read_usize(oferta + 0xc) {
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
                tirar(read_usize(this + 0x946), e);
                break;
            }
        }
    }
    if read_usize(oferta + 4) != 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
        let l = read_usize(oferta + 4);
        write_u32(l + 0xa, read_usize(l + 2) as u32);
        loop {
            let e = proximo(read_usize(oferta + 4));
            if e == 0 {
                break;
            }
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)));
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            if c != 0 && friendly_club_by_id(read_usize(c + 4)) != 0 {
                tirar_proposta(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(e)), read_i32(oferta + 0x20));
            }
            let q = (read_usize(this + 0x86e).wrapping_add(read_usize(e).wrapping_mul(8)) + 2) as *mut u8;
            *q = (*q).wrapping_sub(1);
        }
    }
    let tirar_do_clube: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    tirar_do_clube(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(oferta + 0xc)), friendly_club_by_id(read_usize(oferta + 8)));
    let q = (read_usize(this + 0x86e).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(8)) + 1) as *mut u8;
    *q = (*q).wrapping_sub(1); // as ofertas pela pessoa
    if read_i32(oferta + 0x20) < read_i32(this + 0x85e) {
        write_u32(this + 0x85e, read_usize(oferta + 0x20) as u32);
    }
    write_u32(read_usize(this).wrapping_add((read_i32(oferta + 0x20) as usize).wrapping_mul(4)), 0);
    if oferta != 0 {
        destruir(oferta);
        apagar(oferta);
    }
    write_u32(this + 8, read_i32(this + 8).wrapping_sub(1) as u32);
}


/// Comparador: lado a lado pelas duas saídas, o sorteio, [0xae1080] e [0xb67a34] (a caixa da falta de ficha).
pub(crate) unsafe extern "thiscall" fn comparar_get_offer_response_delay(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_GET_OFFER_RESPONSE_DELAY);
    let mut reg = vec![(dias as usize, 1usize), (fatias as usize, 1), (0xae1080, 4), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, oferta, dias, fatias),
        || crate::aleatorio::sem_contar(|| get_offer_response_delay(this, oferta, dias, fatias)),
    );
    PLACAR.registrar(igual, || format!("get_offer_response_delay({:#x}) {:x?} x {:x?}", oferta, mo, mr));
}

/// Comparador: só a original, conferindo o plano (feito antes): saem da lista as prestações que tinham 1 a pagar.
// ponytail: a original paga (finanças), tira itens da lista (libera) e aloca as marcas; não roda duas vezes. A cópia
// tem os ramos e a prova é pela carreira.
pub(crate) unsafe extern "thiscall" fn comparar_pay_transfer_instalments(this: usize) {
    let lista = this + 0x872;
    let antes = national_teams_list_items(lista);
    let acabam = antes.iter().filter(|&&e| read_u8(e + 0xc) == 1).count();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_TRANSFER_INSTALMENTS)(this);
    let depois = national_teams_list_items(lista).len();
    PLACAR.registrar(depois + acabam == antes.len(), || format!("pay_transfer_instalments: {} prestações, {} acabando, {} depois", antes.len(), acabam, depois));
}

/// Comparador: só a original, conferindo o plano (feito antes): a vaga zerada, as contas +8 e +0x14, a menor vaga livre
/// e o +1 do registro do jogador.
// ponytail: a original destrói e libera a oferta e mexe nas listas; não roda duas vezes.
pub(crate) unsafe extern "thiscall" fn comparar_remove_offer(this: usize, oferta: usize) {
    let vaga = read_i32(oferta + 0x20);
    let ligada = read_i32(oferta + 0x28) != -1;
    let (n8, n14, livre) = (read_i32(this + 8), read_i32(this + 0x14), read_i32(this + 0x85e));
    let q = read_usize(this + 0x86e).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(8)) + 1;
    let ofertas = read_u8(q);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_OFFER)(this, oferta);
    let ok = read_usize(read_usize(this).wrapping_add((vaga as usize).wrapping_mul(4))) == 0
        && read_i32(this + 8) == n8 - 1
        && read_i32(this + 0x14) == n14 - ligada as i32
        && read_i32(this + 0x85e) == livre.min(vaga)
        && read_u8(q) == ofertas.wrapping_sub(1);
    PLACAR.registrar(ok, || format!("remove_offer: vaga {} contas {}/{} -> {}/{} livre {} -> {} ofertas {} -> {}",
        vaga, n8, n14, read_i32(this + 8), read_i32(this + 0x14), livre, read_i32(this + 0x85e), ofertas, read_u8(q)));
}

// ---------- 248a leva: as ofertas pelo jogador: se o clube tem oferta por ele, a oferta do clube e se ele tem oferta pendente (transfer_manager.cpp) ----------


/// Person id has the "under offer" flag (bit 0 of [this+0x86e] + id x 8).
///
/// [pt] A marca "sob oferta" da pessoa (bit 0 do byte [this+0x86e] + id × 8).
pub(crate) unsafe fn is_marked_under_offer(this: usize, id: usize) -> bool {
    *((read_usize(this + 0x86e).wrapping_add(id.wrapping_mul(8))) as *const u8) & 1 != 0
}

/// Offer at index k in the pointer vector [this].
///
/// [pt] A oferta do índice `k` (vetor de ponteiros [this]).
pub(crate) unsafe fn offer_at(this: usize, k: usize) -> usize {
    read_usize(read_usize(this).wrapping_add(k.wrapping_mul(4)))
}

/// Builds a local LLIST with the offer indices for the person (get_offers_for_player), rewinds it, runs `f` on it and
/// destroys it.
///
/// [pt] A lista local de índices das ofertas pela pessoa (0x68d020, que a 0x8c5cf0 enche com `todas`), com o cursor
/// (+0xa) reposto no começo (+2) como a original; `f` percorre e a lista é destruída (0x68d040) no fim. Nas saídas
/// antes da 0x8c5cf0 a original constrói e destrói a lista vazia, sem efeito (não aloca): aqui não nasce.
pub(crate) unsafe fn with_offers_for_player<R>(this: usize, pessoa: usize, todas: u32, f: impl FnOnce(usize) -> R) -> R {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    lista_nova(lista);
    ofertas(this, pessoa, lista, todas);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let r = f(lista);
    destruir(lista);
    r
}

/// Next node of the LLIST (0x68d1b0), or 0.
///
/// [pt] O próximo nó da lista (0x68d1b0), ou 0.
pub(crate) unsafe fn llist_next_node(lista: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    proximo(lista)
}

/// Non-null person whose id is >= G_STAFF_COUNT.
///
/// [pt] A pessoa fora da tabela (id ≥ [0xae2364]); pessoa nula passa e é lida como na original.
pub(crate) unsafe fn staff_out_of_table(pessoa: usize) -> bool {
    pessoa != 0 && read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32
}

/// 1 when the person flagged as under offer (bit 0 of [+0x86e] + id × 8) has, among the open offers (0x8c5cf0), one
/// from the club (offer +8); else 0
/// Original: `TRANSFER_MANAGER::club_has_offer_for_player` (0x8cdc90, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cdc90 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::club_has_offer_for_player(pessoa,
/// clube)`: 1 se a pessoa marcada como sob oferta (bit 0 de [this+0x86e] + id × 8) tem, entre as ofertas em aberto
/// (0x8c5cf0 sem `todas`), uma do clube (+8 da oferta = id do clube); senão 0 (id fora da tabela também).
pub(crate) unsafe extern "thiscall" fn club_has_offer_for_player(this: usize, pessoa: usize, clube: usize) -> u32 {
    if staff_out_of_table(pessoa) || !is_marked_under_offer(this, read_usize(pessoa)) {
        return 0;
    }
    with_offers_for_player(this, pessoa, 0, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        if read_usize(offer_at(this, read_usize(no)) + 8) == read_usize(clube) { // a oferta é do clube
            crate::comparacao::ramo("cpl_1 clube tem oferta");
            return 1;
        }
    })
}

/// the club offer for the person (with `all`, closed ones too and without requiring the flag), or 0
/// Original: `TRANSFER_MANAGER::find_club_offer_for_player` (0x8cdd80, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8cdd80 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::find_club_offer_for_player(pessoa,
/// clube, todas)`: a oferta do clube pela pessoa (a primeira da lista da 0x8c5cf0, com `todas` também as encerradas),
/// ou 0. Sem `todas`, a pessoa tem de estar marcada como sob oferta; id fora da tabela: 0.
pub(crate) unsafe extern "thiscall" fn find_club_offer_for_player(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    if staff_out_of_table(pessoa) || (todas == 0 && !is_marked_under_offer(this, read_usize(pessoa))) {
        return 0;
    }
    with_offers_for_player(this, pessoa, todas, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        let o = offer_at(this, read_usize(no));
        if read_usize(o + 8) == read_usize(clube) { // a oferta do clube
            crate::comparacao::ramo("cpl_1 oferta do clube");
            return o;
        }
    })
}

/// 1 when the person is under offer (0x8cde70) and an open offer is in state (+0x2c) 0, 1, 5 or 6; else 0
/// Original: `TRANSFER_MANAGER::player_has_pending_offer` (0x8cd150, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cd150 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::player_has_pending_offer(pessoa)`: 1 se
/// a pessoa está sob oferta (0x8cde70 sem a visão do humano) e uma das ofertas em aberto (0x8c5cf0) está na situação
/// (+0x2c) 0, 1, 5 ou 6; senão 0. O índice além da conta (+4) lê a oferta 0, como a original.
pub(crate) unsafe extern "thiscall" fn player_has_pending_offer(this: usize, pessoa: usize) -> u32 {
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    // ponytail: a original constrói a lista antes de perguntar; vazia, a 0x68d020/0x68d040 não alocam (nada a imitar)
    if sob_oferta(this, pessoa, 0) == 0 {
        return 0;
    }
    with_offers_for_player(this, pessoa, 0, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        let k = read_usize(no) as i32;
        let o = if k < read_usize(this + 4) as i32 { offer_at(this, k as usize) } else { read_usize(read_usize(this)) };
        if matches!(*((o + 0x2c) as *const u8), 0 | 1 | 5 | 6) { // a situação pendente
            crate::comparacao::ramo("cpl_1 oferta pendente");
            return 1;
        }
    })
}

/// Region touched by 0x8c5cf0: the cursor bytes of the person's offer list (or a dummy region if none).
///
/// [pt] As regiões que a 0x8c5cf0 mexe: o cursor da lista própria da oferta da pessoa (+0xa..+0xe; a 0x8c5cf0 o repõe).
pub(crate) unsafe fn offer_cursor_region(this: usize, pessoa: usize) -> [(usize, usize); 1] {
    let e = offer_for_player(this, read_usize(pessoa) as i32);
    let l = if e == 0 { 0 } else { read_usize(e) };
    if l == 0 { [(0xae2364, 4)] } else { [(l, 0xe)] }
}

// ponytail: as três só leem (a lista local nasce e morre nelas); com a pessoa nula, só a original (lê o endereço 0)
pub(crate) unsafe extern "thiscall" fn comparar_club_has_offer_for_player(this: usize, pessoa: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CLUB_HAS_OFFER_FOR_PLAYER);
    if pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube), || club_has_offer_for_player(this, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("club_has_offer_for_player({}, clube {}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), read_usize(clube), r, n, mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_find_club_offer_for_player(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize>(IDX_FIND_CLUB_OFFER_FOR_PLAYER);
    if pessoa == 0 {
        return o(this, pessoa, clube, todas);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, todas), || find_club_offer_for_player(this, pessoa, clube, todas));
    PLACAR.registrar(igual && r == n, || format!("find_club_offer_for_player({}, clube {}, todas {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(pessoa), read_usize(clube), todas, r, n, mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_player_has_pending_offer(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PLAYER_HAS_PENDING_OFFER);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || player_has_pending_offer(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("player_has_pending_offer({}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), r, n, mo, mr));
    r
}

// ---------- 248a leva: as ofertas e o contrato do jogador: se ele está sob oferta, se o clube dele segue a lei Bosman e o veto da diretoria na oferta (transfer_manager.cpp) ----------


/// Club id -> club record (0x245 bytes in G_CLUB_TABLE).
///
/// [pt] O clube do id `i` (registros de 0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn club_record(i: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(i.wrapping_mul(0x245))
}

/// Person -> contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como a original.
pub(crate) unsafe fn contract_of_person(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Contract is non-null and its type (+0x35 & 0x3f) is not 5.
///
/// [pt] O contrato vale (não nulo e a situação +0x35 & 0x3f ≠ 5).
pub(crate) unsafe fn is_contract_valid(c: usize) -> bool {
    c != 0 && *((c + 0x35) as *const u8) & 0x3f != 5
}

/// The current human manager ([0xb74c26 + 0x300 x current screen slot]).
///
/// [pt] O técnico humano da vez ([0xb74c26] + 0x300 × (i16)[0xb77c7a]).
pub(crate) unsafe fn transfer_manager_current_human_manager() -> usize {
    let i = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    read_usize((0xb74c26isize + i * 0x300) as usize)
}

/// The current human manager controls the club (HUMAN_MANAGER current_controls_club with 0, 1).
///
/// [pt] O humano da vez manda no clube (0x600c90 com 0, 1).
pub(crate) unsafe fn current_human_controls_club(clube: usize) -> bool {
    let manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CURRENT_CONTROLS_CLUB);
    manda(clube, 0, 1) != 0
}

/// Body of TRANSFER_MANAGER::is_player_under_offer (0x8cde70) over a built local list: the under-offer flag, or as seen
/// by the current human (shortlist, offers involving clubs they control).
///
/// [pt] O corpo da 0x8cde70 sobre a lista local já construída.
pub(crate) unsafe fn is_under_offer_body(this: usize, pessoa: usize, visao: u32, lista: usize) -> u32 {
    if pessoa != 0 && read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let marca = || (*((read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8))) as *const u8) & 1) as u32;
    if visao == 0 {
        return marca(); // a marca "sob oferta"
    }
    let m = transfer_manager_current_human_manager();
    if m == 0 {
        return marca();
    }
    let na_shortlist: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let marcado = na_shortlist(read_usize(G_SCOUT_MANAGER), pessoa, m);
    ofertas(this, pessoa, lista, 0);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    loop {
        let no = proximo(lista);
        if no == 0 {
            break;
        }
        let o = read_usize(read_usize(this).wrapping_add(read_usize(no).wrapping_mul(4)));
        if *((o + 0x2d) as *const u8) != 0 || current_human_controls_club(club_record(read_usize(o + 8))) {
            return 1;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(o + 0xc).wrapping_mul(0x6e));
        let c = contract_of_person(p);
        if is_contract_valid(c) && club_record(read_usize(c + 4)) != 0 {
            // o clube do jogador: a original pede o contrato de novo
            let c = contract_of_person(p);
            let clube = if is_contract_valid(c) { club_record(read_usize(c + 4)) } else { 0 };
            if current_human_controls_club(clube) {
                return 1;
            }
        }
        if marcado != 0 {
            return 1;
        }
    }
    let x = read_usize(transfer_manager_current_human_manager() + 0x39);
    if x != 0 {
        let contract_offer: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
        let o = contract_offer(read_usize(G_CONTRACT_MANAGER), read_usize(pessoa), read_usize(x));
        if o != 0 && *((o + 0x35) as *const u8) & 0x3f == 5 {
            return 1;
        }
    }
    0
}

/// without the human view, the "under offer" flag (bit 0 of [+0x86e] + id × 8); with it, 1 when an offer is known
/// (+0x2d), comes from a club of the current human or is for a player of his club, when the person is on his shortlist
/// or when he has an accepted contract offer for the person
/// Original: `TRANSFER_MANAGER::is_player_under_offer` (0x8cde70, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cde70 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::is_player_under_offer(pessoa, visao)`: id
/// fora da tabela: 0. Sem `visao` (ou sem humano da vez), a marca "sob oferta" (bit 0 de [this+0x86e] + id × 8). Com
/// ela: 1 se alguma oferta em aberto pela pessoa (0x8c5cf0, numa lista local) é conhecida (+0x2d), é de clube do
/// humano da vez (0x600c90) ou o clube atual do jogador da oferta (+0xc; contrato 0x4cf5a0, que fica em [0xae1080]) é
/// dele, ou se a pessoa está na shortlist dele (0x7f7490; vale depois da 1ª oferta); por fim, 1 se o humano (+0x39)
/// tem oferta de contrato aceita (0x4cc9e0, +0x35 & 0x3f = 5) pela pessoa. Senão 0.
pub(crate) unsafe extern "thiscall" fn is_player_under_offer(this: usize, pessoa: usize, visao: u32) -> u32 {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    lista_nova(lista);
    let r = is_under_offer_body(this, pessoa, visao, lista);
    destruir(lista);
    r
}

/// 1 when the nation of the club of the person contract has free transfers at contract expiry (0x544670); without
/// contract, club or nation: 1
/// Original: `player_club_uses_bosman` (0x8d0a70, transfer_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x8d0a70 } (stdcall, ret 4, transfer_manager.cpp): `player_club_uses_bosman(pessoa)`: 1 se o clube do contrato
/// da pessoa (0x4cf5a0, guardado em [0xae1080]) é de nação com transferência livre no fim do contrato (0x544670);
/// sem contrato que valha, sem clube ou sem nação (+0x53): 1. Pessoa nula: a original (caixa 0x3798 e 0). A data
/// local da original (0x54a110) só serve ao quadro SEH. Quente (~31 milhões de chamadas): sem contador.
pub(crate) unsafe extern "stdcall" fn player_club_uses_bosman(pessoa: usize) -> u32 {
    if pessoa == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_PLAYER_CLUB_USES_BOSMAN)(pessoa);
    }
    let c = contract_of_person(pessoa);
    if !is_contract_valid(c) {
        return 1;
    }
    let clube = club_record(read_usize(c + 4));
    if clube == 0 || read_usize(clube + club::NATION) == 0 {
        return 1;
    }
    let livre: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_NATION_ALLOWS_FREE_TRANSFERS);
    livre(read_usize(clube + club::NATION)) // a nação do clube
}

/// with an empty exchange part, the deciding club board answers (0x696b10, by the 0x8e7b50 value); 0x2000: reason 0x24
/// and the player loses 10% of his reputations (+0xb current, +9 home, +0xd world); 0x800: 0x17; 0x200: 0x18; each
/// returns 1
/// Original: `TRANSFER_MANAGER::check_board_veto` (0x8cdb80, transfer_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x8cdb80 } (thiscall, ret 0x18, transfer_manager.cpp): `TRANSFER_MANAGER::check_board_veto(oferta, comprador,
/// vendedor, jogador, decide, motivo)`: o valor da oferta (0x8e7b50 com 0, 0, 1, 0, 1); se a oferta tem a parte de
/// troca (+4) vazia (nula ou palavra 0), a diretoria do clube que decide responde (0x696b10 em [0xb67a44] com o
/// jogador, o clube, o valor e "é o comprador" no byte baixo do ponteiro do comprador, como a original empurra). Bit
/// 0x2000: motivo 0x24 e o jogador (+0x61) perde 10% das reputações +0xb (atual), +9 (na nação) e +0xd (mundial); 0x800: motivo 0x17; 0x200: 0x18;
/// cada um devolve 1. Senão 0.
pub(crate) unsafe extern "thiscall" fn check_board_veto(_this: usize, oferta: usize, comprador: usize, _vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let v = valor(oferta, 0, 0, 1, 0, 1);
    // sem oferta, ou com a parte de troca (+4) preenchida: a diretoria não é consultada
    let resposta = if oferta == 0 || { let t = read_usize(oferta + 4); t != 0 && *(t as *const u16) != 0 } {
        0
    } else {
        let diretoria: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_CHECK_BOARD_TRANSFER_VETO);
        let e_comprador = (comprador as u32 & 0xffff_ff00) | (decide == comprador) as u32;
        diretoria(read_usize(G_MANAGER_MANAGER), jogador, decide, v, e_comprador)
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
            let p = (read_usize(jogador + 0x61) + k) as *mut i16;
            core::ptr::write_unaligned(p, match_pl_ftol(core::ptr::read_unaligned(p) as f64 * read_f64_at(0x9689d8)) as i16); // 90%
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

/// Regions touched by 0x8c5cf0 (the person's offer-list cursor) plus G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] A região que a 0x8c5cf0 mexe (o cursor da lista própria da oferta da pessoa) e [0xae1080].
pub(crate) unsafe fn offer_cursor_and_lookup_regions(this: usize, pessoa: usize) -> [(usize, usize); 2] {
    let e = offer_for_player(this, read_usize(pessoa) as i32);
    let l = if e == 0 { 0 } else { read_usize(e) };
    [(0xae1080, 4), if l == 0 { (0xae2364, 4) } else { (l, 0xe) }]
}

// ponytail: só lê (a lista local nasce e morre nela); com a pessoa nula, só a original (lê o endereço 0)
pub(crate) unsafe extern "thiscall" fn comparar_is_player_under_offer(this: usize, pessoa: usize, visao: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_IS_PLAYER_UNDER_OFFER);
    if pessoa == 0 {
        return o(this, pessoa, visao);
    }
    let reg = offer_cursor_and_lookup_regions(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, visao), || is_player_under_offer(this, pessoa, visao));
    PLACAR.registrar(igual && r == n, || format!("is_player_under_offer({}, visao {}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), visao, r, n, mo, mr));
    r
}

pub(crate) unsafe extern "stdcall" fn comparar_player_club_uses_bosman(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_PLAYER_CLUB_USES_BOSMAN);
    if pessoa == 0 {
        return o(pessoa);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa), || player_club_uses_bosman(pessoa));
    PLACAR.registrar(igual && r == n, || format!("player_club_uses_bosman({}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), r, n, mo, mr));
    r
}

/// Comparador: a diretoria (0x696b10) mexe na confiança e não pode rodar duas vezes: o plano antes (as três palavras
/// do jogador), a original roda e confere: devolução 0 ou 1; com motivo, 0x24 (as palavras a 90%), 0x17 ou 0x18 quando
/// 1 e intocado quando 0; as palavras intocadas fora do motivo 0x24 (sem motivo, intocadas ou as três a 90%).
pub(crate) unsafe extern "thiscall" fn comparar_check_board_veto(this: usize, oferta: usize, comprador: usize, vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32>(IDX_CHECK_BOARD_VETO);
    let dados = if jogador == 0 { 0 } else { read_usize(jogador + 0x61) };
    if dados == 0 {
        return o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    }
    let ler = || [0xb, 9, 0xd].map(|k| core::ptr::read_unaligned((dados + k) as *const i16));
    let antes = ler();
    let m0 = if motivo.is_null() { 0 } else { *motivo };
    let r = o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    let depois = ler();
    let a_90 = antes.map(|v| match_pl_ftol(v as f64 * read_f64_at(0x9689d8)) as i16);
    let m = if motivo.is_null() { 0 } else { *motivo };
    let ok = match (r, motivo.is_null()) {
        (0, false) => m == m0 && depois == antes,
        (0, true) => depois == antes,
        (1, false) => (m == 0x24 && depois == a_90) || ((m == 0x17 || m == 0x18) && depois == antes),
        (1, true) => depois == antes || depois == a_90,
        _ => false,
    };
    PLACAR.registrar(ok, || format!("check_board_veto devolveu {}, motivo {:#x} (antes {:#x}), palavras {:?} -> {:?}", r, m, m0, antes, depois));
    r
}

// ---------- 248a leva: os registros das transferências: guardar a transferência passada, dar a permissão de trabalho e guardar o prêmio por jogo no time (transfer_manager.cpp) ----------



/// Shows transfer_manager.cpp's "Memory Allocation Error" box (version, file, line, text); the game continues.
///
/// [pt] A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (versão 0x90d130, nome
/// e extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
pub(crate) unsafe fn show_transfer_memory_error(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Grows the vector at [this+v] by `passo` records of `tam` bytes when full (malloc/realloc), showing the memory box on
/// failure; returns whether a vector exists.
///
/// [pt] Cresce o vetor [this+v] (capacidade em +v+`c`, conta em +v+12−`c`: `c` = 4 nas passadas e nos prêmios, 8 nas
/// permissões) de `passo` registros de `tam` bytes quando cheio: malloc (0x944e46) sem vetor, senão realloc
/// (0x945667). Devolve se há vetor (sem memória: a caixa já saiu).
pub(crate) unsafe fn grow_record_vector(this: usize, v: usize, c: usize, step_towards: usize, tam: usize, linha: u32, texto: usize) -> bool {
    if read_usize(this + v + 12 - c) != read_usize(this + v + c) {
        return true;
    }
    let cap = read_usize(this + v + c).wrapping_add(step_towards);
    write_usize(this + v + c, cap);
    let p = if read_usize(this + v) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        alocar(cap.wrapping_mul(tam) as u32)
    } else {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        realocar(read_usize(this + v), cap.wrapping_mul(tam) as u32)
    };
    write_usize(this + v, p);
    if p == 0 {
        show_transfer_memory_error(linha, texto);
        return false;
    }
    true
}

/// appends the past transfer (0x25 bytes, 0x8cb940) to the +0x862 vector (capacity +0x866, count +0x86a), growing by
/// 100; returns the new record
/// Original: `TRANSFER_MANAGER::add_past_transfer` (0x8cb810, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cb810 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_past_transfer(registro)`: acrescenta
/// a transferência passada (0x25 bytes, cópia 0x8cb940) ao vetor [this+0x862] (capacidade +0x866, conta +0x86a, que
/// sobe antes da cópia); cheio, cresce de 100 em 100 (sem memória: caixa 0x2794 e nada). Devolve o registro novo.
pub(crate) unsafe extern "thiscall" fn add_past_transfer(this: usize, registro: usize) -> usize {
    crate::comparacao::ramo("cpl_3 transferencia passada");
    // ponytail: sem memória a original devolve o que a caixa deixou em eax; aqui 0 (o jogo não lê)
    if !grow_record_vector(this, 0x862, 4, 100, 0x25, 0x2794, 0xa8ec34) {
        return 0;
    }
    let n = read_usize(this + 0x86a);
    let novo = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    write_usize(this + 0x86a, n.wrapping_add(1)); // a conta das transferências passadas
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_PAST_TRANSFER_COPY_CONSTRUCT);
    copiar(novo, registro)
}

/// appends the permit (14 bytes: person, year +8, nation +0xa) to the +0x93a vector (growing by 30), sets the person
/// 0x20 flag at +0x86e and, when asked, sorts (0x8e55c0)
/// Original: `TRANSFER_MANAGER::add_work_permit` (0x8c7070, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c7070 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::add_work_permit(pessoa, nacao, ano, _,
/// ordenar)`: acrescenta a permissão de trabalho (14 bytes: +0 id da pessoa, +4 e +6 zerados, +8 ano, +0xa id da
/// nação) ao vetor [this+0x93a] (capacidade +0x942, conta +0x93e); cheio, cresce de 30 em 30 (sem memória: caixa
/// 0x1b1b e nada). Liga a marca da permissão (bit 0x20 de [this+0x86e] + id × 8) e, com `ordenar`, reordena (qsort
/// 0x945333 com 0x8e55c0, a ordem da busca 0x8c7560).
pub(crate) unsafe extern "thiscall" fn add_work_permit(this: usize, pessoa: usize, nacao: usize, ano: u32, _nao_usado: u32, ordenar: u32) {
    crate::comparacao::ramo("cpl_3 permissao de trabalho");
    if !grow_record_vector(this, 0x93a, 8, 30, 0xe, 0x1b1b, 0xa8eb04) {
        return;
    }
    let n = read_usize(this + 0x93e);
    let r = read_usize(this + 0x93a).wrapping_add(n.wrapping_mul(0xe));
    write_usize(r, read_usize(pessoa));
    write_usize(r + 0xa, read_usize(nacao));
    core::ptr::write_unaligned((r + 8) as *mut u16, ano as u16);
    core::ptr::write_unaligned((r + 6) as *mut u16, 0);
    core::ptr::write_unaligned((r + 4) as *mut u16, 0);
    write_usize(this + 0x93e, n.wrapping_add(1));
    let marca = (read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8))) as *mut u8;
    *marca |= 0x20; // a marca da permissão
    if ordenar != 0 {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        qsort(read_usize(this + 0x93a), read_usize(this + 0x93e) as u32, 0xe, 0x8e55c0);
    }
}

/// appends the appearance money (13 bytes) to the +0x88c vector (growing by 50) and sorts by person (0x8b4e00)
/// Original: `TRANSFER_MANAGER::add_first_team_appearance_money` (0x8cac40, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cac40 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_first_team_appearance_money(registro)`:
/// acrescenta o prêmio por jogo no time principal (13 bytes copiados de `registro`) ao vetor [this+0x88c] (capacidade
/// +0x890, conta +0x894); cheio, cresce de 50 em 50 (sem memória: caixa 0x2581 e nada). Reordena pela pessoa (qsort
/// 0x945333 com 0x8b4e00).
pub(crate) unsafe extern "thiscall" fn add_first_team_appearance_money(this: usize, registro: usize) {
    crate::comparacao::ramo("cpl_3 premio por jogo");
    if !grow_record_vector(this, 0x88c, 4, 50, 0xd, 0x2581, 0xa8eb94) {
        return;
    }
    let n = read_usize(this + 0x894);
    core::ptr::copy_nonoverlapping(registro as *const u8, read_usize(this + 0x88c).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    write_usize(this + 0x894, n.wrapping_add(1)); // mais um prêmio
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0x88c), read_usize(this + 0x894) as u32, 0xd, 0x8b4e00);
}

/// Some of the n records of `tam` bytes in vector v equals `alvo`.
///
/// [pt] Algum dos `n` registros de `tam` bytes do vetor `v` é igual a `alvo`.
pub(crate) unsafe fn vector_contains_record(v: usize, n: usize, tam: usize, alvo: &[u8]) -> bool {
    (0..n).any(|i| core::slice::from_raw_parts((v + i * tam) as *const u8, tam) == alvo)
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta, o registro),
/// a original roda, confere a conta, o registro novo (as duplas e o byte +0x1c; a data é da 0x8cb940) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_add_past_transfer(this: usize, registro: usize) -> usize {
    let n = read_usize(this + 0x86a);
    let fonte = core::slice::from_raw_parts(registro as *const u8, 0x1d).to_vec();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_ADD_PAST_TRANSFER)(this, registro);
    if read_usize(this + 0x862) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let novo = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let gravado = core::slice::from_raw_parts(novo as *const u8, 0x1d);
    PLACAR.registrar(r == novo && read_usize(this + 0x86a) == n.wrapping_add(1) && gravado == &fonte[..], || {
        format!("add_past_transfer: conta {} -> {}, devolveu {:x} (esperava {:x}), registro {:x?} x {:x?}", n, read_usize(this + 0x86a), r, novo, fonte, gravado)
    });
    r
}

/// Comparador: o plano antes (a conta e o registro esperado), a original roda; confere a conta, a marca 0x20 e que o
/// registro está no vetor (com `ordenar`, em qualquer lugar).
pub(crate) unsafe extern "thiscall" fn comparar_add_work_permit(this: usize, pessoa: usize, nacao: usize, ano: u32, nao_usado: u32, ordenar: u32) {
    let n = read_usize(this + 0x93e);
    let mut alvo = [0u8; 0xe];
    alvo[0..4].copy_from_slice(&(read_usize(pessoa) as u32).to_le_bytes());
    alvo[8..10].copy_from_slice(&(ano as u16).to_le_bytes());
    alvo[0xa..0xe].copy_from_slice(&(read_usize(nacao) as u32).to_le_bytes());
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_WORK_PERMIT)(this, pessoa, nacao, ano, nao_usado, ordenar);
    if read_usize(this + 0x93a) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x93e);
    let marca = *((read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8))) as *const u8);
    let ok = conta == n.wrapping_add(1) && marca & 0x20 != 0 && vector_contains_record(read_usize(this + 0x93a), conta, 0xe, &alvo);
    PLACAR.registrar(ok, || format!("add_work_permit(pessoa {}, nação {}, ano {}): conta {} -> {}, marca {:#x}", read_usize(pessoa), read_usize(nacao), ano as u16, n, conta, marca));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_first_team_appearance_money(this: usize, registro: usize) {
    let n = read_usize(this + 0x894);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_FIRST_TEAM_APPEARANCE_MONEY)(this, registro);
    if read_usize(this + 0x88c) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x894);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x88c), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_first_team_appearance_money: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

// ---------- 247a leva: as regras da partida e o amistoso: o alcance das regras de escalação da partida, o motivo de um jogador não poder jogar e a vaga do amistoso marcado ----------


/// reserves: the home nation and type 1; friendly: 0xff; otherwise the key nation of the competition (0x669470), the
/// continent (+0x59) and type 4 (0, 1), 2 (3 to 7) or 1
/// Original: `TRANSFER_MANAGER::get_fixture_rules_scope` (0x8d2fa0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8d2fa0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::get_fixture_rules_scope(partida, tipo,
/// nação, continente)`: competição da partida (+0x14) de reservas ([0x9cf684]): a nação do mandante (+0x1c, +0x53), o
/// continente dela (+0x71, ou 0) e tipo 1. Amistoso (0x5c6710 sobre [0xae2c74]): 0, 0 e 0xff. Senão, pela nação-chave
/// da competição (0x669470, com sinal): a nação da tabela 0xb63d60 (72 bytes cada), o continente da competição (+0x59)
/// e tipo 4 (chaves 0 e 1), 2 (3 a 7) ou 1.
pub(crate) unsafe extern "thiscall" fn get_fixture_rules_scope(_this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let comp = read_usize(partida + 0x14);
    if read_usize(comp) == read_usize(KEY_CLUB_COMP_RESERVE) {
        let n = read_usize(read_usize(partida + 0x1c) + 0x53);
        *nacao = n;
        *continente = if n != 0 { read_usize(n + 0x71) } else { 0 };
        *tipo = 1;
        return;
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    if amistoso(read_usize(G_FRIENDLY), comp) != 0 {
        *nacao = 0;
        *continente = 0;
        *tipo = 0xff;
        return;
    }
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_COMP);
    let k = chave(comp) as i8;
    *nacao = read_usize((0xb63d60isize + k as isize * 72) as usize);
    *continente = read_usize(read_usize(partida + 0x14) + 0x59);
    *tipo = match k {
        0 | 1 => 4,
        3..=7 => 2,
        _ => {
            crate::comparacao::ramo("cpk_4 escopo da nacao");
            1 // a nação da competição
        }
    };
}

/// 9 for an AWOL player (outside national teams); otherwise the first of the 28 rules (+0x8ac) of the continent or
/// nation with the same type answers (virtual +0x18); none: 0
/// Original: `TRANSFER_MANAGER::get_ineligibility_code` (0x8cf5e0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf5e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_ineligibility_code(jogador, partida)`:
/// mandante (+0x1c) que não é seleção (0x53b900) e jogador ausente sem licença (AWOL: 0x418f60 sobre [0xad9c6c], com 0,
/// que tira a marca de quem está sem clube): 9. Senão, pelo alcance das regras (0x8d2fa0; 0xff e 4: 0): a primeira das
/// 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo (+0x12) responde (virtual
/// +0x18 com jogador e partida). Nenhuma: 0.
pub(crate) unsafe extern "thiscall" fn get_ineligibility_code(this: usize, jogador: usize, partida: usize) -> u8 {
    if partida != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        if selecao(read_usize(partida + 0x1c)) == 0 {
            let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
            if ausente(read_usize(G_AWOL), jogador, 0) != 0 {
                return 9;
            }
        }
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FIXTURE_RULES_SCOPE);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff || tipo == 4 {
        return 0;
    }
    for k in 0..28 {
        let r = read_usize(this + 0x8ac + k * 4);
        let casa = if tipo == 2 {
            read_usize(r + 0xe) == continente && *((r + 0x12) as *const u8) == 2
        } else {
            read_usize(r + 0xa) == nacao && *((r + 0x12) as *const u8) == tipo // a regra da nação
        };
        if casa {
            crate::comparacao::ramo("cpk_4 regra da partida");
            let respond: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(read_usize(read_usize(r) + 0x18));
            return respond(r, jogador, partida);
        }
    }
    0
}


/// Comparador: lado a lado pelas três saídas (a 0x669470 e a 0x5c6710 só leem).
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_rules_scope(this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize)>(IDX_GET_FIXTURE_RULES_SCOPE);
    let regioes = [(tipo as usize, 1), (nacao as usize, 4), (continente as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, partida, tipo, nacao, continente), || get_fixture_rules_scope(this, partida, tipo, nacao, continente));
    PLACAR.registrar(igual, || format!("get_fixture_rules_scope({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparador: pelo valor. Jogador com a marca de ausente (0x418f60 com 1, que só lê): só a original (com 0 ela tira
/// a marca de quem está sem clube e a segunda chamada veria outro estado).
pub(crate) unsafe extern "thiscall" fn comparar_get_ineligibility_code(this: usize, jogador: usize, partida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_INELIGIBILITY_CODE);
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
    if ausente(read_usize(G_AWOL), jogador, 1) != 0 {
        return o(this, jogador, partida);
    }
    let r = o(this, jogador, partida);
    let n = get_ineligibility_code(this, jogador, partida);
    PLACAR.registrar(r == n, || format!("get_ineligibility_code({:x}, {:x}) original={} rust={}", jogador, partida, r, n));
    r
}


// ---------- 245a leva: as regras de transferência da nação: o texto da notícia do fim do prazo e o fim do empréstimo; o pow do game.cpp ----------


/// Transfer rules object i (pointers at this+0x8ac).
///
/// [pt] As regras de transferência (+0x8ac, 4 bytes cada) do índice `i` no gerenciador de transferências `this`.
pub(crate) unsafe fn transfer_rules_at(this: usize, i: usize) -> usize {
    read_usize(this + 0x8ac + i.wrapping_mul(4))
}

/// the deadline news text through the rules of the nation in news slot 0 (method +0x30)
/// Original: `TRANSFER_MANAGER::write_deadline_news_text` (0x8e05a0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8e05a0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::write_deadline_news_text(noticia,
/// completo, texto)`: o texto da notícia do fim do prazo de transferências fica com as regras da nação da vaga 0 da
/// notícia (0x785830): o método +0x30 delas, com os mesmos argumentos. Devolve o que ele devolve.
pub(crate) unsafe extern "thiscall" fn write_deadline_news_text(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let regras = transfer_rules_at(this, vaga(noticia, 0) as usize); // as regras da vaga 0
    let metodo: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(read_usize(read_usize(regras) + 0x30));
    metodo(regras, noticia, completo, texto)
}

/// Transfer rules of the club's nation if the player's owning club (+0x39) is in that nation; else None.
///
/// [pt] As regras (das 28 em +0x8ac) da nação do `clube` (+0x53), se o clube dono do `jogador` (+0x39) é da mesma nação;
/// senão nenhuma.
pub(crate) unsafe fn loan_transfer_rules(this: usize, clube: usize, jogador: usize) -> Option<usize> {
    let dono = read_usize(jogador + 0x39);
    if dono == 0 || read_usize(dono + 0x53) != read_usize(clube + club::NATION) {
        return None;
    }
    let nacao = read_usize(clube + club::NATION);
    (0..0x1c).map(|i| transfer_rules_at(this, i)).find(|&r| read_usize(r + 0xa) == nacao) // a nação das regras
}

/// with the player parent club in the same nation as the club, that nation rules answer (method +0x24); else 1
/// Original: `TRANSFER_MANAGER::can_end_loan` (0x8d0530, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8d0530 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::can_end_loan(clube, jogador, a, b,
/// c)`: com o clube dono do jogador (+0x39) na mesma nação (+0x53) do `clube`, quem responde são as regras dessa nação
/// (a primeira das 28 em +0x8ac com +0xa igual): o método +0x24 delas com `clube`, `jogador`, `a`, `b`, `c`. Senão 1.
pub(crate) unsafe extern "thiscall" fn can_end_loan(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    match loan_transfer_rules(this, clube, jogador) {
        Some(r) => {
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(r) + 0x24));
            metodo(r, clube, jogador, a, b, c)
        }
        None => 1,
    }
}


/// Comparador: o método das regras pode escrever o texto e sortear: só a original; a escolha das regras da cópia é a
/// mesma conta (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_write_deadline_news_text(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    crate::comparacao::ramo("write_deadline_news_text");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_WRITE_DEADLINE_NEWS_TEXT)(this, noticia, completo, texto)
}

/// Comparador: quando as regras da nação respondem, só a original (o método é delas); senão os dois devolvem 1.
pub(crate) unsafe extern "thiscall" fn comparar_can_end_loan(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(IDX_CAN_END_LOAN)(this, clube, jogador, a, b, c);
    if loan_transfer_rules(this, clube, jogador).is_some() {
        crate::comparacao::ramo("can_end_loan pelas regras");
        return r;
    }
    PLACAR.registrar(r == 1, || format!("can_end_loan({:x}, {:x}) original={} rust=1", clube, jogador, r));
    r
}


// ---------- 244a leva: as permissões de trabalho (transfer_manager.cpp): a oferta que precisa de permissão e a atualização anual ----------


/// Staff id -> person record (110 bytes each in G_STAFF_TABLE).
pub(crate) unsafe fn staff_record_by_id(i: u32) -> usize {
    (read_u32(G_STAFF_TABLE) as usize).wrapping_add((i as usize).wrapping_mul(110))
}

/// Club id -> club record (581 bytes each in G_CLUB_TABLE).
pub(crate) unsafe fn club_record_by_id(i: u32) -> usize {
    let i = i as usize;
    (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
}

/// Person needs a work permit to play at the club (0x8c7560 with 0).
///
/// [pt] A pessoa precisa de permissão para jogar no clube (0x8c7560, original, com 0 = sem ignorar as dadas)?
pub(crate) unsafe fn needs_work_permit(this: usize, pessoa: usize, clube: usize) -> bool {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    precisa(this, pessoa, clube, 0) != 0
}

/// 1 when the offer player needs a permit at the offer club (0x8c7560) or one of the exchange players at his contract
/// club; else 0
/// Original: `transfer_manager_offer_needs_work_permit` (0x8c7470, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c7470 } (thiscall, ret 4, transfer_manager.cpp): `transfer_manager_offer_needs_work_permit(oferta)`: 1 se o
/// jogador da oferta (+0xc) precisa de permissão no clube da oferta (+8), ou se algum dos jogadores de troca (a lista
/// em +4, do começo: o cursor +0xa volta ao +2 e anda pela 0x68d1b0) precisa no clube do contrato dele (0x4cf5a0 em
/// [0xae1050], gravado em [0xae1080]; sem contrato, clube nulo); senão 0.
pub(crate) unsafe extern "thiscall" fn transfer_manager_offer_needs_work_permit(this: usize, oferta: usize) -> u32 {
    if needs_work_permit(this, staff_record_by_id(read_u32(oferta + 0xc)), club_record_by_id(read_u32(oferta + 8))) {
        return 1; // o jogador da oferta
    }
    let lista = read_u32(oferta + 4) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32(lista + 2)); // o cursor no começo
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let mut item = proximo(lista);
    while item != 0 {
        let c = contract_of(read_u32(G_CONTRACT_MANAGER) as usize, staff_record_by_id(read_u32(item)));
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let clube = if c != 0 { club_record_by_id(read_u32(c + 4)) } else { 0 };
        if needs_work_permit(this, staff_record_by_id(read_u32(item)), clube) {
            return 1;
        }
        item = proximo(lista);
    }
    0
}

/// clears the flag of whoever lacks bit 0x20; after more than 4 years grants the second nationality and drops the
/// permit; drops the permit of non-players; sorts and trims the dropped ones from the end
/// Original: `TRANSFER_MANAGER::update_work_permits` (0x8c7330, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c7330 } (thiscall, transfer_manager.cpp): `TRANSFER_MANAGER::update_work_permits`: no vetor de permissões
/// ([this+0x93a], [this+0x93e] registros de 14 bytes: +0 pessoa, +8 ano, +0xa nação), a marca da pessoa (byte em
/// [this+0x86e] + 8 × id) sem o bit 0x20 é zerada inteira; com mais de 4 anos (ano do jogo − +8) a pessoa ganha a
/// segunda nacionalidade (+0x1e = nação +0xa, 0x122 bytes em [0xae23a8]), perde o bit 0x20 e a permissão sai (pessoa
/// −1); quem não é jogador (+0x61) também perde. Saindo alguma: reordena (qsort 0x945333 com 0x8e55c0) e desconta as
/// do fim com pessoa −1.
pub(crate) unsafe extern "thiscall" fn update_work_permits(this: usize) {
    let n = (this + 0x93e) as *mut i32;
    let mut saiu = false;
    let mut i = 0i32;
    let mut off = 0usize;
    while i < *n {
        let r = (read_u32(this + 0x93a) as usize).wrapping_add(off);
        let pessoa = staff_record_by_id(read_u32(r));
        let marca = |p: usize| ((read_u32(this + 0x86e) as usize).wrapping_add((read_u32(p) as usize).wrapping_mul(8))) as *mut u8;
        if *marca(pessoa) & 0x20 == 0 {
            *marca(pessoa) = 0; // a marca sem permissão
        }
        let r = (read_u32(this + 0x93a) as usize).wrapping_add(off);
        let anos = (*(G_CURRENT_DATE_YEAR as *const u16) as i32).wrapping_sub(*((r + 8) as *const u16) as i32);
        if anos > 4 {
            let nacao = read_u32(r + 0xa) as usize;
            core::ptr::write_unaligned((pessoa + staff::SECOND_NATION) as *mut usize, (read_u32(G_NATION_TABLE) as usize).wrapping_add(nacao.wrapping_mul(0x122)));
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((read_u32(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true;
        } else if read_u32(pessoa + staff::PLAYER) == 0 {
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((read_u32(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true; // quem não é jogador
        }
        i += 1;
        off = off.wrapping_add(0xe);
    }
    if !saiu {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32(this + 0x93a) as usize, *n, 0xe, 0x8e55c0);
    let mut k = *n - 1;
    while k >= 0 {
        let r = (read_u32(this + 0x93a) as usize).wrapping_add(k as usize * 0xe);
        if read_u32(r) != u32::MAX {
            break;
        }
        *n -= 1;
        k -= 1;
    }
}

/// Comparador: lado a lado pela ficha [0xae1080] e pelo cursor (+0xa) da lista de troca.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_offer_needs_work_permit(this: usize, oferta: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TRANSFER_MANAGER_OFFER_NEEDS_WORK_PERMIT);
    let lista = read_u32(oferta + 4) as usize;
    let mut reg = vec![(0xae1080, 4)];
    if lista != 0 {
        reg.push((lista + 0xa, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, oferta), || transfer_manager_offer_needs_work_permit(this, oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_offer_needs_work_permit({:x}) original={} rust={} {:x?} x {:x?}", oferta, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da pessoa e pela segunda
/// nacionalidade (+0x1e). Permissões com pessoa negativa não entram nas regiões (o original leria fora do vetor).
pub(crate) unsafe extern "thiscall" fn comparar_update_work_permits(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WORK_PERMITS);
    let n = (*((this + 0x93e) as *const i32)).max(0) as usize;
    let vetor = read_u32(this + 0x93a) as usize;
    let mut reg = vec![(this + 0x93a, 8), (vetor, n * 0xe)];
    for k in 0..n {
        let id = read_u32(vetor + k * 0xe);
        if (id as i32) < 0 {
            continue;
        }
        let pessoa = staff_record_by_id(id);
        reg.push(((read_u32(this + 0x86e) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(8)), 1));
        reg.push((pessoa + staff::SECOND_NATION, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || update_work_permits(this));
    PLACAR.registrar(igual, || format!("update_work_permits {:x?} x {:x?}", mo, mr));
}

// ---------- 243a leva: as cópias da notícia (news.cpp) e das transferências passada e futura (transfer_manager.cpp) ----------



/// copies the past transfer (0x25 bytes): seven dwords, the byte +0x1c and the date (+0x1d)
/// Original: `past_transfer_copy_construct` (0x8cb940, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cb940 } (thiscall, ret 4, transfer_manager.cpp): `past_transfer_copy_construct`: copia a transferência passada
/// (0x25 bytes) de `origem`: sete duplas (+0..+0x18), o byte +0x1c e a data em +0x1d. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn past_transfer_copy_construct(this: usize, origem: usize) -> usize {
    for k in 0..7 {
        core::ptr::write_unaligned((this + 4 * k) as *mut u32, core::ptr::read_unaligned((origem + 4 * k) as *const u32)); // as duplas da transferência
    }
    *((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8);
    copy_record_date(this + 0x1d, origem + 0x1d);
    this
}

/// copies the future transfer (0xc bytes): the date (+0) and the offer index (+8)
/// Original: `future_transfer_assign` (0x8cbf40, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cbf40 } (thiscall, ret 4, transfer_manager.cpp): `future_transfer_assign`: copia a transferência futura (0xc
/// bytes) de `origem`: a data em +0 e o índice da oferta (+8). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn future_transfer_assign(this: usize, origem: usize) -> usize {
    copy_record_date(this, origem);
    core::ptr::write_unaligned((this + 8) as *mut u32, core::ptr::read_unaligned((origem + 8) as *const u32)); // o índice da oferta
    this
}


pub(crate) unsafe extern "thiscall" fn comparar_past_transfer_copy_construct(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PAST_TRANSFER_COPY_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x25)], || o(this, origem), || past_transfer_copy_construct(this, origem));
    PLACAR.registrar(igual && r == n, || format!("past_transfer_copy_construct {:x?} x {:x?}", mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_future_transfer_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FUTURE_TRANSFER_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || future_transfer_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("future_transfer_assign {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 213a leva: o fim do empréstimo, a contratação pelas regras da nação e as trocas que oferecem o jogador (transfer_manager.cpp) ----------

/// Transfer rules of the nation among the 28 pointers at [manager+0x8ac] (matching +0xa), or 0.
///
/// [pt] As regras de transferência da nação: das 28 em [gerente+0x8ac] (ponteiros), a de nação (+0xa) igual;
/// 0 se nenhuma.
pub(crate) unsafe fn nation_transfer_rules(gerente: usize, nacao: usize) -> usize {
    (0..28)
        .map(|i| *((gerente + 0x8ac + 4 * i) as *const usize))
        .find(|&r| core::ptr::read_unaligned((r + 0xa) as *const usize) == nacao)
        .unwrap_or(0)
}

/// whether the club may sign the player under the nation rules, with the refusal reason and the human club's second try
/// on the foreign player limit
/// Original: `TRANSFER_MANAGER::can_sign_player` (0x8ced40, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8ced40 } (thiscall, ret 0x18, transfer_manager.cpp): se o `clube` pode contratar o `jogador`,
/// pelas regras da nação do clube (método +0x10 com clube, jogador, a4, a2 e o motivo); sem regras, 1. O
/// motivo (palavra) vai para `saida` (zerada antes). Recusado com `a5`, clube humano (0x600a50) e motivo
/// de estrangeiros (0x11, 0x14, 0x15, 0x1f): pergunta de novo com a nação do jogador (+0x1a) trocada pela
/// do clube (e devolvida depois); aceito assim, 1.
pub(crate) unsafe extern "thiscall" fn can_sign_player(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let regras = nation_transfer_rules(gerente, core::ptr::read_unaligned((clube + club::NATION) as *const usize));
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
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 0, 1, 0, 0) == 0 || !matches!(motivo as u16, 0x11 | 0x14 | 0x15 | 0x1f) {
        if !saida.is_null() {
            *saida = motivo as u16;
        }
        return 0;
    }
    let nacao = (jogador + 0x1a) as *mut u32;
    let guardada = core::ptr::read_unaligned(nacao);
    core::ptr::write_unaligned(nacao, core::ptr::read_unaligned((clube + club::NATION) as *const u32));
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

/// how many of the club's open offers include the player in the exchange, listing their indices when asked
/// Original: `TRANSFER_MANAGER::get_exchange_offers_for_player` (0x8c5e40, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5e40 } (thiscall, ret 8, transfer_manager.cpp): quantas ofertas abertas do clube do contrato do
/// `jogador` (0x4cf5a0) o oferecem na troca (lista +4 da oferta, nó com o id dele), anotando o índice
/// (+0x20) em `saida` (0x68d0d0) se houver. Ofertas no estado (+0x2c) 2, 3, 4, 0x10, 0x14 ou 0xf com
/// +0x2e ≥ 3 não contam. Para ao chegar ao número de trocas marcado ([+0x86e] + 8 × id + 2); marcado 0
/// ou id fora da faixa, 0.
pub(crate) unsafe extern "thiscall" fn get_exchange_offers_for_player(gerente: usize, jogador: usize, saida: usize) -> u32 {
    if jogador != 0 && *(jogador as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let limit = || *((*((gerente + 0x86e) as *const usize) as isize + *(jogador as *const i32) as isize * 8 + 2) as *const u8) as u32;
    if limit() == 0 {
        return 0; // sem trocas marcadas
    }
    if transfer_manager_contract_club(jogador) == 0 {
        return 0;
    }
    let clube = *(transfer_manager_contract_club(jogador) as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
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
        if conta == limit() {
            break;
        }
    }
    conta
}


pub(crate) unsafe extern "thiscall" fn comparar_can_sign_player(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, *mut u16) -> u32>(IDX_CAN_SIGN_PLAYER);
    let mut regioes = vec![];
    if !saida.is_null() {
        regioes.push((saida as usize, 2));
    }
    if jogador != 0 {
        regioes.push((jogador + 0x1a, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, clube, a2, jogador, a4, a5, saida),
        || can_sign_player(gerente, clube, a2, jogador, a4, a5, saida));
    PLACAR.registrar(igual && r == n, || format!("can_sign_player(a5 {}) original={} rust={} {:x?} x {:x?}", a5, r, n, mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_get_exchange_offers_for_player(gerente: usize, jogador: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    if saida != 0 {
        // ponytail: com lista de saída a original anexa (aloca); só a original
        return o(gerente, jogador, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if jogador != 0 && *(jogador as *const i32) < *(G_STAFF_COUNT as *const i32) {
        let guardado = *(G_CONTRACT_LOOKUP_RESULT as *const usize);
        let p = transfer_manager_contract_club(jogador);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = guardado;
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
        || get_exchange_offers_for_player(gerente, jogador, saida));
    PLACAR.registrar(igual && r == n, || format!("get_exchange_offers_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 211a leva: o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador (transfer_manager.cpp) ----------

/// Walks a game LLIST from head (+2) and returns each node's data (+0), next at +6.
///
/// [pt] O nó de uma lista do jogo (0x68d0d0/0x68d1b0): contagem u16 em +0, cabeça +2, cauda +6, cursor +0xa;
/// cada nó tem o dado em +0 e o próximo em +6.
pub(crate) unsafe fn llist_data_values(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = core::ptr::read_unaligned((l + 2) as *const usize);
    while no != 0 {
        v.push(*(no as *const usize));
        no = core::ptr::read_unaligned((no + 6) as *const usize);
    }
    v
}

/// Person -> club record of their contract (G_CLUB_TABLE + 581 x [contract+4]), contract stored in
/// G_CONTRACT_LOOKUP_RESULT; 0 without.
///
/// [pt] 0x4cf5a0 (o contrato do jogador `pessoa`, thiscall em [0xae1050]) gravado em [0xae1080]; devolve o
/// registro do clube do contrato ([0xae23b4] + 581 × [contrato+4]) ou 0 sem contrato.
pub(crate) unsafe fn transfer_manager_contract_club(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(*(G_CONTRACT_MANAGER as *const usize), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581))
}

/// closes (state 4) the contract offers linked to the transfer offer: the target player and the exchange players
/// Original: `transfer_manager_close_offer_contract_offers` (0x8c6db0, transfer_manager.cpp, stdcall).
///
/// [pt] { 0x8c6db0 } (stdcall, ret 4, transfer_manager.cpp): fecha (estado +0x50 = 4) as propostas de contrato
/// ligadas à oferta: a do jogador-alvo (0x4cc9e0 com o id +0xc e o clube +8) e a de cada jogador da troca
/// (lista +4, com o clube do contrato do alvo, 0x4cf5a0).
pub(crate) unsafe extern "stdcall" fn transfer_manager_close_offer_contract_offers(oferta: usize) {
    let id = *((oferta + 0xc) as *const usize);
    let pessoa = (*(G_STAFF_TABLE as *const usize)).wrapping_add(id.wrapping_mul(110));
    let clube = transfer_manager_contract_club(pessoa);
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let r = proposta(*(G_CONTRACT_MANAGER as *const usize), id as u32, *((oferta + 8) as *const u32));
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
        let t = proposta(*(G_CONTRACT_MANAGER as *const usize), *(e as *const u32), core::ptr::read_volatile(clube as *const u32));
        if t != 0 {
            *((t + 0x50) as *mut u8) = 4;
        }
        e = prox(l);
    }
}

/// Port of 0x8c16b0: appends a 9-byte record (id, reason, compensation) to the free-transfer queue (+0x92c) via the
/// injected `anexar`; false if the person is on loan.
///
/// [pt] { 0x8c16b0 } (thiscall, ret 0xc, transfer_manager.cpp): põe o jogador na fila da transferência livre
/// (lista +0x92c, registro de 9 bytes: id, motivo, compensação). Emprestado (0x4cf6d0): a original
/// (caixa de erro).
pub(crate) unsafe fn queue_free_transfer_with(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32, anexar: &mut dyn FnMut(usize, &[u8; 9])) -> bool {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    if emprestado(*(G_CONTRACT_MANAGER as *const usize), pessoa) != 0 {
        return false;
    }
    let mut r = [0u8; 9];
    r[..4].copy_from_slice(&(*(pessoa as *const u32)).to_le_bytes());
    r[4] = motivo;
    r[5..].copy_from_slice(&compensacao.to_le_bytes());
    anexar(gerente + 0x92c, &r);
    true
}

/// queues the player for a free transfer (id, reason and compensation), refusing players out on loan
/// Original: `TRANSFER_MANAGER::queue_free_transfer` (0x8c16b0, transfer_manager.cpp, thiscall).
///
/// [pt] põe o jogador na fila da transferência livre (id, motivo e compensação), recusando o emprestado
pub(crate) unsafe extern "thiscall" fn queue_free_transfer(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let anexar_9: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    if !queue_free_transfer_with(gerente, pessoa, motivo, compensacao, &mut |l, r| anexar_9(l, r.as_ptr() as usize, 9)) {
        original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(IDX_QUEUE_FREE_TRANSFER)(gerente, pessoa, motivo, compensacao);
    }
}

/// counts the open negotiations of the player's club that include him in a part exchange and, if asked, lists the offer
/// indices
/// Original: `TRANSFER_MANAGER::get_exchange_negotiations_for_player` (0x8c5fc0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5fc0 } (thiscall, ret 8, transfer_manager.cpp): conta as negociações ([+0xc], [+0x10] delas) do
/// clube do contrato do jogador (+8) cuja oferta ([gerente] + 4 × [+0x20]) segue aberta (estados 2, 3, 4,
/// 0x10 e 0x14 não; 0xf só com +0x2e < 3) e que têm o jogador na lista da troca (+4); com `saida`, põe
/// nela o índice da oferta (0x68d0d0, 4 bytes). Jogador fora do banco ou sem contrato: 0.
pub(crate) unsafe extern "thiscall" fn get_exchange_negotiations_for_player(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    if pessoa != 0 && *(pessoa as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    // como `transfer_manager_contract_club`, à parte para a mutação desta rotina não cair na 0x8c6db0
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(*(G_CONTRACT_MANAGER as *const usize), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    let p = (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581));
    if p == 0 {
        return 0;
    }
    let clube = *(p as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
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

pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_close_offer_contract_offers(oferta: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    // as propostas que podem fechar: as do id do alvo e dos ids da troca (o clube não filtra aqui)
    let mut ids = vec![*((oferta + 0xc) as *const u32)];
    let l = *((oferta + 4) as *const usize);
    if l != 0 {
        ids.extend(llist_data_values(l).iter().map(|&e| *(e as *const u32)));
    }
    let propostas = *((*(G_CONTRACT_MANAGER as *const usize) + 0x34) as *const usize);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if l != 0 {
        regioes.push((l + 0xa, 4));
    }
    if propostas != 0 {
        regioes.push((propostas + 0xa, 4));
        for r in llist_data_values(propostas) {
            if ids.contains(&*(r as *const u32)) {
                regioes.push((r + 0x50, 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(oferta), || transfer_manager_close_offer_contract_offers(oferta));
    PLACAR.registrar(igual, || format!("transfer_manager_close_offer_contract_offers(id {}) {:x?} x {:x?}", ids[0], mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_queue_free_transfer(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(IDX_QUEUE_FREE_TRANSFER);
    // ponytail: a fila aloca um nó (não roda duas vezes): a cópia só monta o registro (0x4cf6d0 é
    // consulta pura), a original põe na fila e confere-se o nó novo da cauda
    let mut registro = None;
    if !queue_free_transfer_with(gerente, pessoa, motivo, compensacao, &mut |_, r| registro = Some(*r)) {
        return o(gerente, pessoa, motivo, compensacao);
    }
    let l = gerente + 0x92c;
    let antes = core::ptr::read_unaligned(l as *const u16);
    o(gerente, pessoa, motivo, compensacao);
    let depois = core::ptr::read_unaligned(l as *const u16);
    let cauda = core::ptr::read_unaligned((l + 6) as *const usize);
    let novo = if cauda == 0 { None } else { Some(*(*(cauda as *const usize) as *const [u8; 9])) };
    PLACAR.registrar(depois == antes.wrapping_add(1) && novo == registro,
        || format!("queue_free_transfer {} -> {} original={:x?} rust={:x?}", antes, depois, novo, registro));
}

pub(crate) unsafe extern "thiscall" fn comparar_get_exchange_negotiations_for_player(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    if saida != 0 {
        // ponytail: com lista de saída (só a tela 0x47ec79) a original anexa (aloca); só a original
        return o(gerente, pessoa, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if pessoa != 0 && *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) {
        let guardado = *(G_CONTRACT_LOOKUP_RESULT as *const usize);
        let p = transfer_manager_contract_club(pessoa);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = guardado;
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
        || get_exchange_negotiations_for_player(gerente, pessoa, saida));
    PLACAR.registrar(igual && r == n, || format!("get_exchange_negotiations_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 211a leva: as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia (transfer_manager.cpp, shortlist_manager.cpp, comp_util.cpp) ----------

/// First offer in the manager's vector [this] (count [this+4]) whose target id (+0xc) is `id`, or 0.
///
/// [pt] A oferta do gerente de transferências para a pessoa `id`: a primeira das [this+4] em [this] com
/// +0xc = `id`, ou 0.
pub(crate) unsafe fn offer_for_player(this: usize, id: i32) -> usize {
    let ofertas = *(this as *const usize);
    let n = *((this + 4) as *const i32);
    (0..n.max(0) as usize)
        .map(|i| *((ofertas + 4 * i) as *const usize))
        .find(|&e| e != 0 && *((e + 0xc) as *const i32) == id)
        .unwrap_or(0)
}

/// counts the transfer offers for a player (skipping closed ones unless asked) and may list their indices
/// Original: `TRANSFER_MANAGER::get_offers_for_player` (0x8c5cf0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5cf0 } (thiscall, ret 0xc, transfer_manager.cpp): conta as ofertas pela pessoa e, com
/// `lista`, acrescenta nela (0x68d0d0, 4 bytes) o índice de cada uma. Id a partir de 0xae2364 ou sem
/// oferta: 0. A oferta achada sem lista própria (+0) conta 1 (acrescenta +0x20); com lista, percorre
/// os índices dela (cursor +0xa reiniciado de +2) e conta os que apontam para ofertas da mesma pessoa.
/// Sem `todas`, ficam de fora as encerradas (0x8d0d90, aqui embutida como na original).
pub(crate) unsafe extern "thiscall" fn get_offers_for_player(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    // a original lê o id antes de testar o ponteiro
    let id = core::ptr::read_volatile(pessoa);
    if !pessoa.is_null() && id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let e = offer_for_player(this, id);
    if e == 0 {
        return 0;
    }
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let proximo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = *(e as *const usize);
    if l == 0 {
        if todas == 0 && transfer_offer_closed(e) != 0 {
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
        if todas == 0 && transfer_offer_closed(o) != 0 {
            continue;
        }
        if lista != 0 {
            acrescentar(lista, no, 4);
        }
        conta += 1;
    }
    conta
}

/// 9-byte entry of the club (id +0) in the shopping list [this+0xdc].
///
/// [pt] O registro de 9 bytes do clube (id em +0) na lista de compras [this+0xdc].
pub(crate) unsafe fn shopping_list_entry(this: usize, clube: usize) -> usize {
    (*(clube as *const i32) as usize).wrapping_mul(9).wrapping_add(*((this + 0xdc) as *const usize))
}



pub(crate) unsafe extern "thiscall" fn comparar_get_offers_for_player(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, usize, u32) -> u32>(IDX_GET_OFFERS_FOR_PLAYER);
    // ponytail: com `lista` a original aloca nós (0x68d0d0) e não pode rodar duas vezes; só ela
    if lista != 0 || pessoa.is_null() {
        return o(this, pessoa, lista, todas);
    }
    // a original só mexe no cursor da lista da oferta achada
    let e = offer_for_player(this, *pessoa);
    let l = if e == 0 { 0 } else { *(e as *const usize) };
    let reg = if l == 0 { vec![] } else { vec![(l, 0xe)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, lista, todas), || get_offers_for_player(this, pessoa, lista, todas));
    PLACAR.registrar(igual && r == n, || format!("get_offers_for_player({}, todas {}) original={} rust={} {:x?} x {:x?}", *pessoa, todas, r, n, mo, mr));
    r
}



// ---------- 193a leva: as cláusulas do contrato no gerente de transferências: percentual da próxima venda, bônus por jogo e bônus pela seleção (transfer_manager.cpp) ----------

/// Clears bit `bit` of the player's flags (+0x86e) and removes their 13-byte record from the payment list at
/// [manager+lista], shifting the rest down and decrementing the count.
///
/// [pt] O comum das três: desliga o bit `bit` das marcas do jogador `*id` (+0x86e, 8 bytes por pessoa) e
/// tira o registro dele (13 bytes, o id em `chave`) da lista [gerente+`lista`] (quantos em
/// [gerente+`quantos`]), puxando os seguintes uma vaga para trás.
pub(crate) unsafe fn remove_from_payment_list(gerente: usize, id: *const i32, bit: u8, lista: usize, quantos: usize, chave: usize) {
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

/// removes the player's next sale percentage (flag and list record)
/// Original: `TRANSFER_MANAGER::remove_next_sale_percentage` (0x8cb4f0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cb4f0 } (thiscall, ret 4, transfer_manager.cpp): tira o percentual da próxima venda (bit 8;
/// lista +0x880, quantos +0x888, o id em +9).
pub(crate) unsafe extern "thiscall" fn remove_next_sale_percentage(gerente: usize, id: *const i32) {
    remove_from_payment_list(gerente, id, 8, 0x880, 0x888, 9);
}

/// removes the first-team appearance bonus (flag and list record)
/// Original: `TRANSFER_MANAGER::remove_first_team_appearance_money` (0x8cb590, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cb590 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo no time principal (bit 2;
/// lista +0x88c, quantos +0x894, o id em +0).
pub(crate) unsafe extern "thiscall" fn remove_first_team_appearance_money(gerente: usize, id: *const i32) {
    remove_from_payment_list(gerente, id, 2, 0x88c, 0x894, 0);
}

/// removes the international appearance bonus (flag and list record)
/// Original: `TRANSFER_MANAGER::remove_international_team_appearance_money` (0x8cb630, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cb630 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo pela seleção (bit 4;
/// lista +0x898, quantos +0x8a0, o id em +0).
pub(crate) unsafe extern "thiscall" fn remove_international_team_appearance_money(gerente: usize, id: *const i32) {
    remove_from_payment_list(gerente, id, 4, 0x898, 0x8a0, 0);
}

/// Comparer regions for the payment-list removals: the flags byte, the counter and the whole list.
///
/// [pt] As regiões das três: o byte das marcas, a lista inteira e o contador.
pub(crate) unsafe fn payment_list_regions(gerente: usize, id: *const i32, lista: usize, quantos: usize) -> Vec<(usize, usize)> {
    let marcas = (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as usize;
    let n = (*((gerente + quantos) as *const i32)).max(0) as usize;
    let mut r = vec![(marcas, 1), (gerente + quantos, 4)];
    if n > 0 {
        r.push((*((gerente + lista) as *const usize), n * 13));
    }
    r
}

pub(crate) unsafe fn comparar_lista_de_pagamentos(nome: &str, k: usize, rust: unsafe extern "thiscall" fn(usize, *const i32), gerente: usize, id: *const i32, lista: usize, quantos: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(k);
    let reg = payment_list_regions(gerente, id, lista, quantos);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || rust(gerente, id));
    PLACAR.registrar(igual, || format!("{}({}) {:x?} x {:x?}", nome, *id, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_remove_next_sale_percentage(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_next_sale_percentage", 451, remove_next_sale_percentage, gerente, id, 0x880, 0x888);
}

pub(crate) unsafe extern "thiscall" fn comparar_remove_first_team_appearance_money(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_first_team_appearance_money", 452, remove_first_team_appearance_money, gerente, id, 0x88c, 0x894);
}

pub(crate) unsafe extern "thiscall" fn comparar_remove_international_team_appearance_money(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_international_team_appearance_money", 453, remove_international_team_appearance_money, gerente, id, 0x898, 0x8a0);
}

// ---------- 191a leva: o clube do contrato acertado, o jogo pela seleção e a punição guardada (transfer_manager.cpp, discipline.cpp) ----------

/// the club of the person's agreed contract (clears the flag when the contract is gone)
/// Original: `TRANSFER_MANAGER::get_agreed_contract_club` (0x8c5450, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5450 } (thiscall, ret 8, transfer_manager.cpp): com a marca de contrato acertado (0x40) da
/// pessoa `*id`, procura o contrato (0x4cca30 em [0xae1050]): achou, grava em `clube` (se não nulo) o
/// clube do contrato ([0xae23b4], 581 bytes por clube, pelo id em +4) e devolve 1; não achou, desliga a
/// marca e devolve 0.
pub(crate) unsafe extern "thiscall" fn get_agreed_contract_club(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let marcas = |g: usize| (*((g + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marcas(gerente) & 0x40 == 0 {
        return 0;
    }
    let procurar: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_AGREED_CONTRACT_OFFER);
    let contrato = procurar(*(G_CONTRACT_MANAGER as *const usize), *id);
    if contrato == 0 {
        *marcas(gerente) &= 0xbf;
        return 0;
    }
    if !clube.is_null() {
        *clube = (*(G_CLUB_TABLE as *const usize) as isize + *((contrato + 4) as *const i32) as isize * 581) as usize;
    }
    1
}

/// Address of person *id's 8-byte record in the transfer manager (+0x86e).
///
/// [pt] O registro de 8 bytes da pessoa `*id` no gerente de transferências (+0x86e).
pub(crate) unsafe fn staff_flags_entry(gerente: usize, id: *const i32) -> usize {
    (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as usize
}

/// records the player's national team match in the recent-matches bits
/// Original: `TRANSFER_MANAGER::record_international_appearance` (0x8ce920, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8ce920 } (thiscall, ret 8, transfer_manager.cpp): registra que o jogador jogou o jogo `numero`
/// da seleção: os bits dos últimos jogos (+3, 16 bits) andam um por jogo desde o último registrado
/// (+5), o mais novo é ligado e o número fica em +5.
pub(crate) unsafe extern "thiscall" fn record_international_appearance(gerente: usize, id: *const i32, numero: i16) {
    let r = staff_flags_entry(gerente, id);
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


pub(crate) unsafe extern "thiscall" fn comparar_get_agreed_contract_club(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, *mut usize) -> u32>(IDX_GET_AGREED_CONTRACT_CLUB);
    let mut reg = vec![(staff_flags_entry(gerente, id), 1)];
    if !clube.is_null() {
        reg.push((clube as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, clube), || get_agreed_contract_club(gerente, id, clube));
    PLACAR.registrar(igual && ro == rr, || format!("get_agreed_contract_club({}) original={} rust={} {:x?} x {:x?}", *id, ro, rr, mo, mr));
    ro
}

pub(crate) unsafe extern "thiscall" fn comparar_record_international_appearance(gerente: usize, id: *const i32, numero: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, i16)>(IDX_RECORD_INTERNATIONAL_APPEARANCE);
    let reg = [(staff_flags_entry(gerente, id) + 3, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, numero), || record_international_appearance(gerente, id, numero));
    PLACAR.registrar(igual, || format!("record_international_appearance({}, {}) {:x?} x {:x?}", *id, numero, mo, mr));
}


// ---------- 185a leva: a marca de contrato acertado e a vaga da oferta (transfer_manager.cpp) ----------

/// Pointer to person *id's flags byte in the transfer manager (+0x86e, 8 bytes per person).
///
/// [pt] O byte das marcas da pessoa `*id` no gerente de transferências (+0x86e, 8 bytes por pessoa).
pub(crate) unsafe fn staff_flags_ptr(gerente: usize, id: *const i32) -> *mut u8 {
    (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8
}

/// sets the person's agreed-contract flag (0x40) in the transfer manager
/// Original: `set_agreed_contract_flag` (0x8c5410, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5410 } (thiscall, ret 4, transfer_manager.cpp): liga a marca de contrato acertado (0x40).
pub(crate) unsafe extern "thiscall" fn set_agreed_contract_flag(gerente: usize, id: *const i32) {
    *staff_flags_ptr(gerente, id) |= 0x40;
}

/// clears the person's agreed-contract flag
/// Original: `clear_agreed_contract_flag` (0x8c5430, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5430 } (thiscall, ret 4, transfer_manager.cpp): desliga a marca de contrato acertado.
pub(crate) unsafe extern "thiscall" fn clear_agreed_contract_flag(gerente: usize, id: *const i32) {
    *staff_flags_ptr(gerente, id) &= 0xbf;
}

/// the address of an offer slot (out of range, the table start)
/// Original: `offer_slot` (0x8c61e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c61e0 } (thiscall, ret 4, transfer_manager.cpp): o endereço da vaga `k` das ofertas (+0,
/// +4 quantas); fora da faixa, o próprio começo da tabela (o valor de +0, não o endereço).
pub(crate) unsafe extern "thiscall" fn offer_slot(tabela: usize, k: i32) -> usize {
    let base = *(tabela as *const usize);
    if k < *((tabela + 4) as *const i32) { (base as isize + 4 * k as isize) as usize } else { base }
}

pub(crate) unsafe extern "thiscall" fn comparar_set_agreed_contract_flag(gerente: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_SET_AGREED_CONTRACT_FLAG);
    let a = staff_flags_ptr(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || set_agreed_contract_flag(gerente, id));
    PLACAR.registrar(igual, || format!("set_agreed_contract_flag({}) {:x?} x {:x?}", *id, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_clear_agreed_contract_flag(gerente: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_CLEAR_AGREED_CONTRACT_FLAG);
    let a = staff_flags_ptr(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || clear_agreed_contract_flag(gerente, id));
    PLACAR.registrar(igual, || format!("clear_agreed_contract_flag({}) {:x?} x {:x?}", *id, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_offer_slot(tabela: usize, k: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_OFFER_SLOT)(tabela, k);
    let n = offer_slot(tabela, k);
    PLACAR.registrar(r == n, || format!("offer_slot({}) original={:#x} rust={:#x}", k, r, n));
    r
}

/// from the match restriction table (0x8cf6b0), the first one present: type at +0, count at +5 (and the age at +1 for
/// type 3)
/// Original: `get_selection_limit` (0x8cf7e0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf7e0 } (thiscall, ret 8, transfer_manager.cpp): `get_selection_limit(partida, saida)`: zera os 6 bytes da
/// saída, pede a tabela de 42 bytes das restrições da partida (0x8cf6b0 com 1; o `this` passa direto) e fica com a
/// primeira que existe (≠ 0xff): +5 ou +3 (tipo 1), +2 ou +0 (tipo 2), +0x16 com a quantidade em +0x17 (tipo 3, a
/// idade em +1). O tipo vai em +0 e a quantidade em +5.
pub(crate) unsafe extern "thiscall" fn get_selection_limit(this: usize, partida: usize, saida: *mut u8) {
    core::ptr::write_unaligned(saida as *mut u32, 0);
    core::ptr::write_unaligned(saida.add(4) as *mut u16, 0);
    let mut t = [0u8; 0x2c];
    let restricoes: unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_FILL_SQUAD_RESTRICTIONS);
    restricoes(this, partida, t.as_mut_ptr(), 1);
    let (tipo, n) = if t[5] != 0xff {
        (1, t[5])
    } else if t[3] != 0xff {
        (1, t[3])
    } else if t[2] != 0xff {
        (2, t[2]) // o 3º limite
    } else if t[0] != 0xff {
        (2, t[0])
    } else if t[0x16] != 0xff {
        *saida.add(1) = t[0x16];
        (3, t[0x17])
    } else {
        return;
    };
    *saida = tipo;
    *saida.add(5) = n;
}

/// Comparador: lado a lado pelos 6 bytes da saída (a 0x8cf6b0 só escreve na tabela local).
pub(crate) unsafe extern "thiscall" fn comparar_get_selection_limit(this: usize, partida: usize, saida: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8)>(IDX_GET_SELECTION_LIMIT);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 6)], || o(this, partida, saida), || get_selection_limit(this, partida, saida));
    PLACAR.registrar(igual, || format!("get_selection_limit({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// with the 0x20 flag of the id in [+0x86e], the binary search for the permit (0xe-byte entries in [+0x93a]); else 0
/// Original: `TRANSFER_MANAGER::find_work_permit` (0x8c7260, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c7260 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_work_permit(pessoa)`: a permissão
/// de trabalho da pessoa: id fora dos [0xae2364] (com sinal), 0; sem a marca 0x20 no byte do id (8 bytes cada, em
/// [+0x86e]), 0; senão a busca binária (bsearch 0x946464, registros de 0xe bytes em [+0x93a], [+0x93e] deles,
/// comparação 0x8e55c0) pelo id. Pessoa nula: lê o endereço 0 como o original.
pub(crate) unsafe extern "thiscall" fn find_work_permit(this: usize, pessoa: *const i32) -> usize {
    let id = core::ptr::read_volatile(pessoa);
    if !pessoa.is_null() && id >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    if *((read_usize(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) as *const u8) & 0x20 == 0 {
        return 0; // sem a marca da permissão
    }
    let chave = id;
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(&chave, read_usize(this + 0x93a), read_usize(this + 0x93e), 0xe, 0x8e55c0)
}

/// Comparador: pelo valor (a busca só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_work_permit(this: usize, pessoa: *const i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> usize>(IDX_FIND_WORK_PERMIT)(this, pessoa);
    let n = find_work_permit(this, pessoa);
    PLACAR.registrar(r == n, || format!("find_work_permit({:x}) original={:#x} rust={:#x}", pessoa as usize, r, n));
    r
}

/// the first unemployed person of the club nation in the wanted job (6, 8, 9, 10; 13 → 6, 15 → 8) joins the club
/// (0x53bd10)
/// Original: `fill_vacated_staff_job` (0x8cb6d0, transfer_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x8cb6d0 } (stdcall, ret 8, transfer_manager.cpp): `fill_vacated_staff_job(clube, cargo)`: a vaga da comissão
/// que ficou livre é preenchida pela primeira pessoa desempregada da nação do clube no cargo procurado (6, 8, 9 e 10;
/// 13 procura 6, 15 procura 8): 0x53bd10 com o cargo (o dword do argumento), a data de hoje [0xae2c90], gravar e sem
/// reduzir. Clube nulo: a original (caixa "Error").
pub(crate) unsafe extern "stdcall" fn fill_vacated_staff_job(clube: usize, cargo: u32) {
    if clube == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize, u32)>(IDX_FILL_VACATED_STAFF_JOB)(clube, cargo); // caixa: só a original
    }
    let Some(c) = job_sought_by_slot(cargo) else { return };
    if let Some(p) = first_unemployed_staff(clube, c as u8) {
        let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
        incluir(clube, p, c, 0xae2c90, 1, 0);
    }
}

/// Comparador: a escolha da cópia antes; sem ninguém, só a original. Com a pessoa, original e cópia lado a lado no
/// clube (0x245 bytes) e na pessoa (0x6e), que é tudo o que a 0x53bd10 grava: ela pode recusar (casa ocupada, faixa
/// dos humanos; com mais ligas acontece) ou gravar na seleção (+0x24), e a pessoa não fica em +0x39.
pub(crate) unsafe extern "stdcall" fn comparar_fill_vacated_staff_job(clube: usize, cargo: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32)>(IDX_FILL_VACATED_STAFF_JOB);
    let escolhida = if clube != 0 { job_sought_by_slot(cargo).and_then(|c| first_unemployed_staff(clube, c as u8)) } else { None };
    let Some(p) = escolhida else {
        crate::comparacao::ramo("fill_vacated_staff_job sem ninguem");
        return o(clube, cargo);
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(clube, 0x245), (p, 0x6e)], || o(clube, cargo),
        || fill_vacated_staff_job(clube, cargo));
    if read_usize(p + 0x39) != clube {
        crate::comparacao::ramo("fill_vacated_staff_job sem entrar no clube");
    }
    PLACAR.registrar(igual, || format!("fill_vacated_staff_job({:x}, {}) pessoa {:x} {:x?} x {:x?}", clube, cargo as u8, p, mo, mr));
}

/// is the person in list +0x92c?
/// Original: `person_in_transfer_list` (0x8c1870, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1870 } (thiscall, ret 4, transfer_manager.cpp): a pessoa `*p` está na lista +0x92c (recomeça
/// o iterador +0xa de +2 e anda pela 0x68d1b0, comparando o primeiro dword de cada item)?
pub(crate) unsafe extern "thiscall" fn person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let lista = t + 0x92c;
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((t + 0x92e) as *const u32));
    loop {
        let e = proximo(lista);
        if e == 0 {
            return 0;
        }
        if *p == *(e as *const i32) {
            return 1;
        }
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(IDX_PERSON_IN_TRANSFER_LIST);
    let reg = [(t + 0x92c + 0xa, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, p), || person_in_transfer_list(t, p));
    PLACAR.registrar(igual && r == v, || format!("person_in_transfer_list({:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, p, r, mo, v, mr));
    r
}

/// the player refuses the terms: the offer returns in 10 days, the bid markers go, his reputation rises if the buyer is
/// more famous; or he leaves the exchange and his value moves to the fee
/// Original: `handle_player_rejected_terms` (0x8c28c0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c28c0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::handle_player_rejected_terms(pessoa,
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
pub(crate) unsafe extern "thiscall" fn handle_player_rejected_terms(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let anotar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_RESPONSE_NEWS);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let resposta_do_vendedor: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let resposta_do_comprador: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REVIEW_AI_CLUB_BID);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    if indice < 0 {
        return show_error_at(0xa8d514, 0xf49);
    }
    if indice >= read_usize(this + 4) as i32 {
        return show_error_at(0xa8d514, 0xf4e);
    }
    let vaga = || read_usize(this).wrapping_add(indice as usize * 4);
    let o = read_usize(vaga());
    if o == 0 {
        return;
    }
    let preco = read_usize(o + 0x1b);
    if read_usize(pessoa) == read_usize(o + 0xc) {
        write_u8(o + 0x10, 4);
        let comprador = friendly_club_by_id(read_usize(o + 8));
        let vendedor = manager_manager_contract_club(transfer_manager_main_contract_stored(pessoa), true);
        write_u8(o + 0x2c, 0xf);
        if silencioso == 0 && human_club(comprador, 1, 1, 0, 0) != 0 && read_u8(o + 0x1f) != 1 {
            write_u8(o + 0x2e, read_u8(o + 0x2e).wrapping_add(1));
        } else {
            write_u8(o + 0x2e, 3);
        }
        let p = staff_records_staff_by_id(read_usize(o + 0xc));
        let dono = manager_manager_contract_club(transfer_manager_main_contract_stored(p), false);
        limpar_marcas(read_usize(this + 0x856), friendly_club_by_id(read_usize(o + 8)), p);
        let l = read_usize(o + 4);
        if l != 0 {
            transfer_manager_llist_rewind(l);
            loop {
                let y = list_next_item(l);
                if y == 0 {
                    break;
                }
                limpar_marcas(read_usize(this + 0x856), dono, staff_records_staff_by_id(read_usize(y)));
            }
        }
        if human_club(comprador, 0, 1, 0, 0) == 0 {
            crate::comparacao::ramo("cpv_2 recusa anotada na lista");
            // a original testa o byte com o eax zerado pela pergunta de antes (em cima, zero)
            anotar(read_usize(this + 0x856), pessoa, comprador, 1, (read_u8(o + 0x11) != 0xb) as u32); // cpv2: a recusa na lista
        }
        crate::comparacao::ramo("cpv_2 recusa do jogador");
        agendar(this, o, 0xa, 0); // cpv2: de volta em 10 dias
        if is_offer_resolved(o) {
            let rivais = read_usize(o);
            let mut todas = true;
            if rivais != 0 {
                transfer_manager_llist_rewind(rivais);
                loop {
                    let x = list_next_item(rivais);
                    if x == 0 {
                        break;
                    }
                    if !is_offer_resolved(read_usize(read_usize(this) + read_usize(x) * 4)) {
                        todas = false;
                        break;
                    }
                }
            }
            if todas {
                let m = read_usize(this + 0x86e).wrapping_add(read_usize(o + 0xc) * 8);
                if read_u8(m) & 1 != 0 {
                    write_u8(m, read_u8(m) & 0xfe);
                }
            }
        }
        let l = read_usize(o + 4);
        if l != 0 {
            transfer_manager_llist_rewind(l);
            loop {
                let y = list_next_item(l);
                if y == 0 {
                    break;
                }
                tirar_proposta(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(y)), vendedor);
            }
        }
        if silencioso == 0 {
            let ficha = read_usize(pessoa + staff::PLAYER);
            if comprador != 0 && vendedor != 0 && ficha != 0 {
                let rc = read_i16(comprador + 0x80);
                let rv = read_i16(vendedor + 0x80);
                if rc > rv {
                    let dif = rc.wrapping_sub(rv) as f64;
                    let k = if rc > 0x1e46 && read_i16(ficha + 0xb) < 0x128e {
                        core::ptr::read_unaligned(0x96a730 as *const f64)
                    } else if rc > 0x109a {
                        core::ptr::read_unaligned(0x967888 as *const f64)
                    } else {
                        core::ptr::read_unaligned(0x967878 as *const f64)
                    };
                    crate::comparacao::ramo("cpv_2 injury_word_0c do jogador");
                    let soma = match_pl_ftol(dif * k) as i16; // cpv2: a reputação sobe
                    let r = (ficha + 0xb) as *mut i16;
                    core::ptr::write_unaligned(r, core::ptr::read_unaligned(r).wrapping_add(soma));
                    let f2 = read_usize(pessoa + staff::PLAYER);
                    if read_i16(f2 + 0xb) > 0x2710 {
                        core::ptr::write_unaligned((f2 + 0xb) as *mut i16, 0x2710);
                    }
                }
            }
            noticia(this, pessoa, comprador, vendedor, 4, o, 0, 0, preco as u32);
        }
        return seguir(this, o);
    }
    let troca = read_usize(o + 4);
    if troca == 0 {
        show_error_at(0xa8d514, 0x101f);
        return seguir(this, o);
    }
    transfer_manager_llist_rewind(troca);
    let mut x = list_next_item(troca);
    while x != 0 && read_usize(x) != read_usize(pessoa) {
        x = list_next_item(troca);
    }
    if x == 0 {
        return seguir(this, o);
    }
    crate::comparacao::ramo("cpv_2 jogador saiu da troca");
    let comprador = friendly_club_by_id(read_usize(o + 8));
    let novo_preco = valor(read_usize(G_CONTRACT_MANAGER), pessoa, comprador, 0, -1, 0).wrapping_add(read_usize(o + 0x1b) as i32);
    let vendedor = manager_manager_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc))), false);
    let comprador = friendly_club_by_id(read_usize(o + 8));
    tirar(read_usize(o + 4), x);
    let l = read_usize(o + 4);
    if core::ptr::read_unaligned(l as *const u16) == 0 {
        if l != 0 {
            destruir(l);
            liberar(l);
        }
        core::ptr::write_unaligned((o + 4) as *mut usize, 0);
    }
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(pessoa) * 8 + 2);
    write_u8(m, read_u8(m).wrapping_sub(1));
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
    resposta_do_comprador(read_usize(G_TRANSFER_MANAGER), vaga(), 1);
    if is_offer_resolved(o) {
        return;
    }
    seguir(this, o);
}

pub(crate) unsafe extern "thiscall" fn comparar_handle_player_rejected_terms(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(IDX_HANDLE_PLAYER_REJECTED_TERMS)(this, pessoa, indice, silencioso)
}

/// when the window closes, completes the approved offers and puts the nation's offers on hold: news if a human is
/// involved, otherwise withdrawn or rejected; for the computer, status 3 and back the next day
/// Original: `hold_offers_at_window_close` (0x8c4e10, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c4e10 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::hold_offers_at_window_close(nação)`: as
/// regras da nação (das 28 em [this+0x8ac], a de +0xa igual à nação +0x53 da seleção 0x53b3d0; senão nenhuma). Para
/// cada oferta do vetor [this] (conta +4): status 0xe completa (0x8c34e0 no prazo). Comprador da nação: não resolvida,
/// jogador com ficha (+0x61), sem a marca +0x30 & 0xf, fora do status 0x13 e aceita pelas regras (virtual +0xc com o
/// comprador e hoje; resposta 0 = segura): em espera; a notícia da espera (0x8e1150) se o comprador ou o vendedor é
/// humano, senão retirada (0x8bfcb0); comprador do computador (0x600a50): status 3 e reagendada em 1 dia. Senão, com
/// jogadores na troca e vendedor da nação, pelos mesmos testes com o vendedor nas regras: em espera; a notícia, ou
/// recusada (0x8bf680); e o mesmo reagendamento.
pub(crate) unsafe extern "thiscall" fn hold_offers_at_window_close(this: usize, nacao: usize) {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let completar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_TRY_COMPLETE_OFFER);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ON_HOLD_NEWS);
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let da_nacao = read_usize(selecao(nacao) + 0x53);
    let mut regras = 0usize;
    for k in 0..0x1c {
        let r = read_usize(this + 0x8ac + 4 * k);
        if read_usize(r + 0xa) == da_nacao {
            regras = r;
            break;
        }
    }
    let aceita = |o: usize, clube: usize| -> bool {
        if regras == 0 {
            return true;
        }
        let segurar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(regras) + 0xc));
        segurar(regras, o, clube, 0xae2c90, 0) == 0
    };
    let mut i = 0usize;
    while (i as i32) < read_usize(this + 4) as i32 {
        let vaga = read_usize(this).wrapping_add(i * 4);
        let o = read_usize(vaga);
        i += 1;
        if o == 0 {
            continue;
        }
        let st = read_u8(o + 0x2c);
        if st == 0xe {
            completar(this, o, 1);
            continue;
        }
        let comprador = friendly_club_by_id(read_usize(o + 8));
        if read_usize(comprador + 0x53) == nacao
            && !is_offer_resolved(o)
            && read_usize(staff_records_staff_by_id(read_usize(o + 0xc)) + 0x61) != 0
            && read_u8(o + 0x30) & 0xf == 0
            && st != 0x13
            && aceita(o, comprador)
        {
            put_offer_on_hold(o);
            let humano_vende = || {
                let c = transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
                if c == 0 || read_u8(c + 0x35) & 0x3f == 5 || friendly_club_by_id(read_usize(c + 4)) == 0 {
                    return false;
                }
                let v = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc))), true);
                humano(read_usize(G_NEWS), v) != 0
            };
            if humano(read_usize(G_NEWS), friendly_club_by_id(read_usize(o + 8))) != 0 || humano_vende() {
                noticia_da_espera(this, o, nacao);
            } else {
                crate::comparacao::ramo("cpv_3 retirada na janela");
                // a vaga relida de [this] como na original
                retirar(this, read_usize(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o comprador retira
            }
            if human_club(friendly_club_by_id(read_usize(o + 8)), 0, 1, 0, 0) == 0 {
                crate::comparacao::ramo("cpv_3 reagendada na janela");
                write_u8(o + 0x2c, 3); // cpv3: status 3
                agendar(this, o, 1, 0); // cpv3: de volta amanhã
            }
            continue;
        }
        // os outros: oferta com troca cujo vendedor é da nação
        if read_usize(o + 4) == 0 {
            continue;
        }
        let c = transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
        if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
            continue;
        }
        let vendedor = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc))), false);
        if read_usize(vendedor + 0x53) != nacao || is_offer_resolved(o) || read_u8(o + 0x2c) == 0x13 {
            continue;
        }
        if regras != 0 {
            let dono = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc))), false);
            if !aceita(o, dono) {
                continue;
            }
        }
        put_offer_on_hold(o);
        let humano_vende = || {
            let c = transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc)));
            if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
                return false;
            }
            let v = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + 0xc))), false);
            humano(read_usize(G_NEWS), v) != 0
        };
        if humano(read_usize(G_NEWS), friendly_club_by_id(read_usize(o + 8))) != 0 || humano_vende() {
            noticia_da_espera(this, o, nacao);
        } else {
            crate::comparacao::ramo("cpv_3 recusada na janela");
            recusar(this, read_usize(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o vendedor recusa
        }
        if human_club(friendly_club_by_id(read_usize(o + 8)), 0, 1, 0, 0) == 0 {
            write_u8(o + 0x2c, 3);
            agendar(this, o, 1, 0);
        }
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_hold_offers_at_window_close(this: usize, nacao: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOLD_OFFERS_AT_WINDOW_CLOSE)(this, nacao)
}

/// gives the offer today's date + days and the day slice (0 to 2, every 3 slices one more day) and puts it in the
/// bucket (days since the base date + start, modulo 50, × 3 + slice) of the +0x18 lists
/// Original: `TRANSFER_MANAGER::schedule_offer` (0x8c6220, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c6220 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::schedule_offer(oferta, dias, fatias)`:
/// sem data base (ano +0x84f zerado), ela vira hoje (0x417c10 de [0xae2c90] em +0x84d) e a fatia +0x855 a de hoje
/// ([0xae2c88]); `dias` ≥ 50: a caixa "Error" (linha 0x18b9) e 10 dias. A oferta ganha a data (+0x12) e a fatia (+0x1a)
/// do plano (`vacancy_schedule_plan`) e o +0x20 dela (4 bytes) entra no balde (0x68d0d0, que aloca); devolve o que a 0x68d0d0
/// devolve. Sem SEH, sem sorteio.
pub(crate) unsafe extern "thiscall" fn schedule_offer(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    crate::comparacao::ramo("cpn_5 agendar oferta");
    if read_i16(this + 0x84f) == 0 {
        let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let mut tmp = [0u8; 8];
        copiar(this + 0x84d, tmp.as_mut_ptr(), 0xae2c90);
        *((this + 0x855) as *mut u8) = read_u8(G_CURRENT_DAY_SLICE);
    }
    if dias as u8 as i8 >= 0x32 {
        show_error_at(0xa8d514, 0x18b9);
    }
    // ponytail: as datas são feitas no plano sobre cópias (0x54c5b0, 0x54cdb0 e 0x417c10 só leem e copiam) e gravadas
    // de uma vez; a original grava a data e a fatia a cada volta da fatia, com o mesmo fim
    let (data, fatia, balde) = vacancy_schedule_plan(this, dias as u8, fatias as u8);
    core::ptr::copy_nonoverlapping(data.as_ptr(), (oferta + 0x12) as *mut u8, 8);
    *((oferta + 0x1a) as *mut i8) = fatia;
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    acrescentar(balde, oferta + 0x20, 4)
}

/// Comparador: a original roda (a 0x68d0d0 aloca o nó) e o resultado é conferido com o plano feito antes: a data
/// (+0x12) e a fatia (+0x1a) da oferta, a conta do balde mais 1 e a data base (+0x84d, 8 bytes) igual a hoje quando
/// não havia.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_offer(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    let sem_base = read_i16(this + 0x84f) == 0;
    let (data, fatia, balde) = vacancy_schedule_plan(this, dias as u8, fatias as u8);
    let conta = read_i16(balde);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_SCHEDULE_OFFER)(this, oferta, dias, fatias);
    let gravada = core::slice::from_raw_parts((oferta + 0x12) as *const u8, 8);
    let base_ok = !sem_base || core::slice::from_raw_parts((this + 0x84d) as *const u8, 8) == core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8);
    let ok = gravada == &data[..] && *((oferta + 0x1a) as *const i8) == fatia && read_i16(balde) == conta.wrapping_add(1) && base_ok;
    PLACAR.registrar(ok, || format!("schedule_offer(dias {}, fatias {}): data {:x?} (plano {:x?}), fatia {} (plano {}), balde {:x} conta {} -> {}, base {}",
        dias as u8 as i8, fatias as u8 as i8, gravada, data, *((oferta + 0x1a) as *const i8), fatia, balde, conta, read_i16(balde), base_ok));
    r
}

/// of the offers for the player from the club (or flagged) in state 2, the highest (0x8e7a70); 0 when none
/// Original: `TRANSFER_MANAGER::find_highest_offer_for_player` (0x8cc930, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cc930 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_highest_offer_for_player(jogador,
/// clube)`: com a marca de ofertas do jogador (byte +1 dos 8 de [+0x86e]), o primeiro registro do vetor [this] ([+4]
/// deles) do jogador (+0xc): sem lista (+0), ele mesmo se serve; com lista, das ofertas (índices no vetor) que servem,
/// a de maior valor (0x8e7a70 com 0, 0, 0, 1; a primeira no empate). Senão 0.
pub(crate) unsafe extern "thiscall" fn find_highest_offer_for_player(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let id = *jogador;
    if *((read_usize(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8)) + 1) as *const u8) == 0 {
        return 0;
    }
    let mut t = 0;
    let mut k = 0i32;
    while k < read_usize(this + 4) as i32 {
        let x = read_usize(read_usize(this).wrapping_add(k as usize * 4));
        if x != 0 && read_usize(x + 0xc) as u32 == id {
            t = x;
            break;
        }
        k += 1;
    }
    if t == 0 {
        return 0;
    }
    let l = read_usize(t);
    if l == 0 {
        return if is_offer_eligible(t, *clube) { t } else { 0 };
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let (mut melhor, mut maior) = (0usize, -1i32);
    let mut e = proximo(l);
    while e != 0 {
        let o = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
        if is_offer_eligible(o, *clube) {
            let v = valor(o, 0, 0, 0, 1);
            if v > maior { // a primeira no empate
                melhor = o;
                maior = v;
            }
        }
        e = proximo(l);
    }
    melhor
}

/// Comparador: pelo valor (a 0x8e7a70 só calcula; as duas voltam o cursor da lista ao começo).
pub(crate) unsafe extern "thiscall" fn comparar_find_highest_offer_for_player(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32) -> usize>(IDX_FIND_HIGHEST_OFFER_FOR_PLAYER)(this, jogador, clube);
    let n = find_highest_offer_for_player(this, jogador, clube);
    PLACAR.registrar(r == n, || format!("find_highest_offer_for_player({}, {}) original={:#x} rust={:#x}", *jogador, *clube, r, n));
    r
}

/// with the person's flag 4, binary-searches the international appearance fee clause; when not found, clears the flag
/// Original: `TRANSFER_MANAGER::find_international_appearance_money` (0x8d0f00, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0f00 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_international_appearance_money(pessoa)`,
/// das telas da pessoa (0x884ea2): sem a marca 4 da pessoa ([this+0x86e] + id × 8), 0. A busca binária (0x946464:
/// cdecl(chave, vetor [this+0x898], conta [this+0x8a0], 0xd, 0x8b4e00)) pela chave com o id da pessoa em +0, o único
/// campo que a comparação lê; achada, o registro; senão a marca 4 sai e 0.
pub(crate) unsafe extern "thiscall" fn find_international_appearance_money(this: usize, p: usize) -> usize {
    let m = (read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8))) as *mut u8;
    if *m & 4 == 0 {
        return 0;
    }
    let chave = [read_usize(p) as u32, 0, 0, 0];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let r = bsearch(chave.as_ptr(), read_usize(this + 0x898), read_usize(this + 0x8a0), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(4);
    0
}

/// Comparador da 0x8d0f00: lado a lado na marca da pessoa e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_international_appearance_money(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_INTERNATIONAL_APPEARANCE_MONEY);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, p), || find_international_appearance_money(this, p));
    PLACAR.registrar(igual && r == n, || format!("find_international_appearance_money(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

/// finds the appearance money record (club or national team) and, when the appearances match, posts the news, adds to
/// the totals and the former club receives the amount the current one pays; returns 1 when paid
/// Original: `TRANSFER_MANAGER::pay_appearance_money` (0x8cb340, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8cb340 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::pay_appearance_money(jogador, jogos,
/// tipo, arg)`: com contrato (guardado em [0xae1080]) e o registro do prêmio da pessoa (o `tipo` vai inteiro, como a
/// original o empurra) cujos jogos (+8, com sinal) são os `jogos` (> 0): dá a notícia (0x8db8a0), soma o valor (+4)
/// aos totais de transferência (0x448ad0 em [0xad9dac], com o clube atual e o antigo), o clube antigo (+9) recebe
/// (0x599800) e o atual paga (0x599890), em 64 bits com sinal. Devolve 1 se pagou.
pub(crate) unsafe extern "thiscall" fn pay_appearance_money(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let c = shortlist_manager_get_contract_and_store(jogador);
    if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        return 0;
    }
    let r = appearance_bonus_record(this, jogador, tipo);
    if r == 0 {
        return 0;
    }
    // a original: jogos diferentes só seguem com −1, que cai no teste seguinte (≤ 0)
    if read_u8(r + 8) as i8 as i32 != jogos || jogos <= 0 {
        return 0;
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_APPEARANCE_MONEY_NEWS);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let somar: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_ADD_TRANSFER_TOTALS);
    let receber: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_FINANCE_ADD_PLAYERS_SOLD);
    let pagar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_FINANCE_PAY_PLAYERS_BOUGHT);
    noticia(this, r, jogos, tipo, arg);
    let f_antigo = financas(read_usize(G_FINANCE_TABLE), friendly_club_by_id(read_usize(r + 9)));
    let c = shortlist_manager_get_contract_and_store(jogador);
    let f_atual = financas(read_usize(G_FINANCE_TABLE), if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 });
    let c = shortlist_manager_get_contract_and_store(jogador);
    let atual = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    let valor = core::ptr::read_unaligned((r + 4) as *const i32);
    somar(read_usize(G_CLUB_RECORDS), valor, atual, friendly_club_by_id(read_usize(r + 9)));
    crate::comparacao::ramo("cpp_6 premio por jogos");
    receber(f_antigo, valor as u32, (valor >> 31) as u32); // o clube antigo recebe
    pagar(f_atual, valor as u32, (valor >> 31) as u32);
    1
}

/// Comparador: dá notícia e mexe nas finanças: roda só a original e confere a devolução com o plano (contrato, clube,
/// registro do prêmio e jogos, sem efeitos).
pub(crate) unsafe extern "thiscall" fn comparar_pay_appearance_money(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32>(IDX_PAY_APPEARANCE_MONEY);
    let c = get_contract(jogador);
    let esperado = if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        0
    } else {
        let r = appearance_bonus_record(this, jogador, tipo);
        (r != 0 && read_u8(r + 8) as i8 as i32 == jogos && jogos > 0) as u32
    };
    let r = o(this, jogador, jogos, tipo, arg);
    PLACAR.registrar(r == esperado, || format!("pay_appearance_money(jogador {:x}, jogos {}, tipo {}) devolveu {}, esperava {}", read_usize(jogador), jogos, tipo as u8, r, esperado));
    r
}

/// the period from the club nation's rules and, with a type 2 contract, moves the end back month by month until it no
/// longer passes the contract end (the start follows); without a contract, expired, too far or before this year: the
/// default year 1972; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference
/// be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in
/// 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy
/// stays in the code and does not run
/// Original: `TRANSFER_MANAGER::get_loan_period` (0x8d0590, transfer_manager.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x8d0590 } (thiscall, ret 0x1c, transfer_manager.cpp): `TRANSFER_MANAGER::get_loan_period(clube, a2, jogador,
/// início_mês, início_ano, fim_mês, fim_ano)`: as regras da nação do clube (+0x53; das 28 em +0x8ac, a primeira sem
/// achar) dão o período (método +0x28). Sem contrato de tipo 2 (0x4cf5a0), com o contrato já vencido hoje ou o fim mais
/// de 5 anos à frente: o fim vira o ano da data padrão (1972). Senão, enquanto a data (dia do mês de hoje, fim_mês,
/// fim_ano; 0x549f70) passar do fim do contrato, recua um mês (o início acompanha se ficar depois do fim); fim antes do
/// ano de hoje: o ano padrão.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_loan_period(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let nacao = read_usize(clube + club::NATION);
    let mut regra = read_usize(this + 0x8ac);
    for k in 0..0x1cusize {
        let r = read_usize(this + 0x8ac + k * 4);
        if read_usize(r + 0xa) == nacao {
            if r != 0 {
                regra = r;
            }
            break;
        }
    }
    let periodo: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut u16, *mut i8, *mut u16, usize) = core::mem::transmute(read_usize(read_usize(regra) + 0x28));
    periodo(regra, clube, a2, ini_mes, ini_ano, mes, ano, jogador);
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let dia = dia_do_mes(0xae2c90);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), jogador);
    if c == 0 || read_u8(c + 0x35) & 0x3f != 2 || is_after_contract_end(read_i16(0xae2c90), read_i16(0xae2c92) as u16, c) {
        *ano = default_date_year();
        return;
    }
    if *ano as i32 > read_i16(0xae2c92) as u16 as i32 + 5 {
        *ano = default_date_year();
        return;
    }
    // ponytail: a original empurra o mês e o ano em registradores com lixo acima; a 0x549f70 lê o byte e a palavra
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    loop {
        let mut data = [0u8; 8];
        let d = data_de(data.as_mut_ptr() as usize, dia, *mes as i32, *ano as u32, -1);
        if !is_after_contract_end(read_i16(d), read_i16(d + 2) as u16, c) {
            return;
        }
        crate::comparacao::ramo("cpo_9 recua o mes");
        let m = (*mes).wrapping_sub(1);
        *mes = m;
        if m < 0 {
            *ano = (*ano).wrapping_sub(1);
            *mes = 11;
        }
        if *ini_ano > *ano || (*ini_ano == *ano && *ini_mes > *mes) {
            *ini_mes = *mes;
            *ini_ano = *ano;
        }
        if *ano < read_i16(0xae2c92) as u16 {
            *ano = default_date_year();
            return;
        }
    }
}

/// Comparador: lado a lado pelas quatro saídas, o sorteio e [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_loan_period(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut i8, *mut u16, *mut i8, *mut u16)>(IDX_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    let mut reg = vec![(ini_mes as usize, 1usize), (ini_ano as usize, 2), (mes as usize, 1), (ano as usize, 2), (0xae1080, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano),
        || crate::aleatorio::sem_contar(|| transfer_manager_get_loan_period(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano)),
    );
    PLACAR.registrar(igual, || format!("transfer_manager_get_loan_period({:#x}, {:#x}) {:x?} x {:x?}", clube, jogador, mo, mr));
}

/// checks each transfer window in the previous, current and next months: on the day, the opening or closing notice; the
/// day after the opening, the notice and the waiting offers of the nation; a week before the opening, the notice
/// Original: `TRANSFER_MANAGER::update_transfer_window_news` (0x8c6bc0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c6bc0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::update_transfer_window_news(hoje)`, do dia das transferências. Os meses anterior, atual e
/// seguinte (0x54cfb0: mov ecx, hoje -> thiscall, ret 0; o anterior de janeiro é dezembro do ano −1, o seguinte de
/// dezembro é janeiro do ano +1), hoje + 1 semana (0x54c770: push 1; push saída; mov ecx, hoje -> thiscall, ret 8) e
/// ontem (0x54c8e0: idem). Para cada janela do vetor [this+0x8a4] (conta +0x8a8, relida): no primeiro dos três meses
/// igual ao dela (+4), a data dela naquele ano (0x40a7d0: push ano; push saída; mov ecx, janela -> thiscall, ret 8;
/// copiada pela 0x417c10). Hoje: o virtual +0x2c das regras da nação dela ([this+0x8ac + 4·(+0)]: push o dword do ano
/// da data; push o código 1, ou 3 se a janela fecha (+5), com os bytes altos do mesmo dword; push janela -> thiscall,
/// ret 0xc); ontem e janela que abre: o virtual com 0 e as ofertas em espera da nação das regras (+0xa; 0x8c4e10: push
/// nação; mov ecx, esi -> thiscall, ret 4); daqui a uma semana e janela que abre: o virtual com 2.
pub(crate) unsafe extern "thiscall" fn update_transfer_window_news(this: usize, hoje: usize) {
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let data_da_janela: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_TRANSFER_WINDOW_GET_DATE);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_HOLD_OFFERS_AT_WINDOW_CLOSE);
    let mut anos = [0i16; 3];
    let mut d = [0u8; 8];
    let pd = d.as_mut_ptr() as usize;
    padrao(pd);
    let mut meses = [0u8; 3];
    meses[0] = mes(hoje).wrapping_sub(1);
    if (meses[0] as i8) < 0 {
        meses[0] = 0xb;
        anos[0] = -1;
    }
    meses[1] = mes(hoje);
    meses[2] = mes(hoje).wrapping_add(1);
    if (meses[2] as i8) > 0xb {
        meses[2] = 0;
        anos[2] = 1;
    }
    let (mut semana, mut ontem) = ([0u8; 8], [0u8; 8]);
    let (ps, po) = (semana.as_mut_ptr() as usize, ontem.as_mut_ptr() as usize);
    mais_semanas(hoje, ps, 1);
    menos(hoje, po, 1);
    let mut v = read_u32_as_usize(this + 0x8a4);
    let mut i: i32 = 0;
    while i < read_u32_as_usize(this + 0x8a8) as i32 {
        for j in 0..3 {
            let w = read_u32_as_usize(v);
            if read_u8(w + 4) != meses[j] {
                continue;
            }
            let ano = (anos[j] as u16).wrapping_add(read_i16(hoje + 2) as u16);
            let (mut t, mut c) = ([0u8; 8], [0u8; 8]);
            let r = data_da_janela(w, t.as_mut_ptr() as usize, ano as u32);
            copia(pd, c.as_mut_ptr() as usize, r);
            let (hd, ha) = (read_i16(hoje), read_i16(hoje + 2));
            let (dd, da, dw) = (read_i16(pd), read_i16(pd + 2), read_u32_as_usize(pd + 2) as u32);
            let regras = |w: usize| read_u32_as_usize(this + 0x8ac + (read_u8(w) as i8 as isize * 4) as usize);
            let aviso = |w: usize, codigo: u32| {
                let rg = regras(w);
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(rg) + 0x2c));
                f(rg, w, codigo, dw);
            };
            if ha == da && hd == dd {
                let codigo = if read_u8(w + 5) != 0 { 3 } else { 1 };
                aviso(w, (dw & 0xffff_ff00) | codigo); // cqc5: o aviso do dia da janela
                break;
            }
            if read_i16(po + 2) == da && read_i16(po) == dd && read_u8(w + 5) == 0 {
                aviso(w, 0);
                let nacao = read_u32_as_usize(regras(read_u32_as_usize(v)) + 0xa);
                if nacao != 0 {
                    crate::comparacao::ramo("cqc_5 janela aberta");
                    espera(this, nacao); // cqc5: as ofertas em espera da janela que abriu
                }
                break;
            }
            if read_i16(ps + 2) == da && read_i16(ps) == dd && read_u8(w + 5) == 0 {
                aviso(w, 2);
                break;
            }
        }
        i += 1;
        v += 4;
    }
}

/// Comparador da 0x8c6bc0: só a original (ela só grava nas locais; os avisos das regras alocam notícias e as ofertas
/// em espera mudam de status e são reagendadas).
pub(crate) unsafe extern "thiscall" fn comparar_update_transfer_window_news(this: usize, hoje: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_TRANSFER_WINDOW_NEWS)(this, hoje) // ponytail: nada próprio a conferir
}

/// marks that the transfer offer had contract terms offered to the player
/// Original: `TRANSFER_MANAGER::note_terms_offered_to_main_player` (0x8c1f80, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1f80 } (thiscall, ret 4, transfer_manager.cpp): marca que a oferta `k` (fora de empréstimo,
/// +0x1f ≠ 1) teve os termos oferecidos ao jogador: o estado +0x2c vira 8 se era 6, senão 7. Índice
/// fora da lista ([+4] ofertas): a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn note_terms_offered_to_main_player(gerente: usize, k: i32) {
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER)(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta != 0 && *((oferta + 0x1f) as *const u8) != 1 {
        let estado = (oferta + 0x2c) as *mut u8;
        *estado = (*estado == 6) as u8 + 7;
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_note_terms_offered_to_main_player(gerente: usize, k: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER);
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return o(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta == 0 {
        return o(gerente, k);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(oferta + 0x2c, 1)], || o(gerente, k), || note_terms_offered_to_main_player(gerente, k));
    PLACAR.registrar(igual, || format!("note_terms_offered_to_main_player({}) {:x?} x {:x?}", k, mo, mr));
}
