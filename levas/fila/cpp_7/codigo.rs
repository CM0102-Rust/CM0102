// ---------- %%LEVA%%a leva: as ofertas de transferência: o clube recusa a oferta, soltar os jogadores da oferta e montar a oferta em dinheiro (transfer_manager.cpp) ----------

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
unsafe extern "thiscall" fn club_rejects_offer(this: usize, oferta: usize, clube: usize, motivo: u32) {
    let estado = cpp7_b(oferta + 0x2c);
    if estado == 0x11 || estado == 0x12 {
        return;
    }
    crate::comparison::branch("cpp_7 refusal");
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
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contract_of(cpp7_d(0xae1050), cpp7_pessoa(cpp7_d(oferta + 0xc)));
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
unsafe extern "thiscall" fn detach_offer_players(this: usize, oferta: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c41c0usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c47d0usize);
    let mut local = [0u32; 4]; // a LLIST (0xe bytes) da original
    lista_nova(local.as_mut_ptr() as usize);
    crate::comparison::branch("cpp_7 release players");
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
unsafe extern "thiscall" fn build_cash_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    // ponytail: a 0x5a0590 só procura; consultada antes de alocar para o caso de erro ir inteiro pela original
    if financas(cpp7_d(0xae2a38), clube) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(%%I2%%)(this, clube, jogador, saida, atracao, valor);
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x8e78f0usize);
    let p = novo(0x41);
    let oferta = if p != 0 { construir(p) } else { 0 };
    *saida = oferta;
    crate::comparison::branch("cpp_7 cash offer");
    iniciar(oferta, jogador, clube, valor); // o valor da oferta
    *atracao = 0x32;
    1
}

/// Comparador: sorteia, reagenda, rejeita ou retira (pode liberar a oferta) e dá notícia: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn compare_club_rejects_offer(this: usize, oferta: usize, clube: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%)(this, oferta, clube, motivo)
}

/// Comparador: tira jogadores de ofertas e desfaz ofertas (libera itens): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn compare_detach_offer_players(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, oferta)
}

/// Comparador: aloca a oferta e a 0x8e78f0 pode sortear: roda só a original e confere o resultado: devolve 1, a oferta
/// guardada em `saída` tem o valor (+0x1b) e a atração ficou em 50.
unsafe extern "thiscall" fn compare_build_cash_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(%%I2%%)(this, clube, jogador, saida, atracao, valor);
    let oferta = *saida;
    let ok = r == 1 && oferta != 0 && core::ptr::read_unaligned((oferta + 0x1b) as *const i32) == valor && *atracao == 0x32;
    SCOREBOARD.record(ok, || format!("build_cash_offer_option(jogador {:x}, valor {}) devolveu {}, oferta {:x}", cpp7_d(jogador), valor, r, oferta));
    r
}
