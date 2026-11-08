// ---------- %%LEVA%%a leva: as ofertas abertas e as desfeitas: as ofertas pelos jogadores do clube, desfazer as ofertas pelo jogador e guardar o percentual da próxima venda (transfer_manager.cpp) ----------

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
unsafe extern "thiscall" fn get_open_offers_for_club_players(this: usize, clube: usize, lista: usize) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
                let c = contract_of(cpr4_d(0xae1050), cpr4_pessoa(cpr4_d(o + 0xc)));
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
        crate::comparison::branch("cpr_4 oferta desfeita");
    }
    if emprestado {
        crate::comparison::branch("cpr_4 jogador emprestado");
    }
    n
}

/// { 0x8c47d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::collapse_offers_for_player(pessoa)`: com ofertas pela pessoa (byte +1 do registro de 8 bytes em
/// [this+0x86e]), pelas ofertas por ela (0x8c5cf0 na lista local): a de transferência futura (0x13) perde a futura
/// (0x8cbf70); todas ficam no estado 0xf com +0x2e = 3; o vendedor é o clube do contrato do jogador da oferta (0x4cf5a0,
/// guardado em [0xae1080]); os lances do comprador (+8) pelo jogador e os do vendedor pelos da troca (lista +4) saem
/// (0x84aeb0 de [this+0x856]) e a oferta é agendada (0x8c6220 com 10 dias).
unsafe extern "thiscall" fn collapse_offers_for_player(this: usize, pessoa: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar_futura: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cbf70usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
            crate::comparison::branch("cpr_4 desfazer oferta");
            let o = cpr4_d(cpr4_d(this).wrapping_add(cpr4_d(e).wrapping_mul(4)));
            if cpr4_b(o + 0x2c) == 0x13 {
                tirar_futura(this, o);
            }
            *((o + 0x2c) as *mut u8) = 0xf; // desfeita
            *((o + 0x2e) as *mut u8) = 3;
            let jogador = cpr4_pessoa(cpr4_d(o + 0xc));
            let c = contract_of(cpr4_d(0xae1050), jogador);
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
unsafe extern "thiscall" fn add_next_sale_percentage(this: usize, registro: usize) {
    crate::comparison::branch("cpr_4 write_percent_pair da proxima venda");
    if !grow_record_vector(this, 0x880, 4, 50, 0xd, 0x2553, 0xa8eb50) {
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
unsafe extern "thiscall" fn comparar_get_open_offers_for_club_players(this: usize, clube: usize, lista: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    if lista != 0 {
        let n = get_open_offers_for_club_players(this, clube, 0);
        let r = o(this, clube, lista);
        PLACAR.record(r == n, || format!("get_open_offers_for_club_players({:x}, lista) original={} rust={}", clube, r, n));
        return r;
    }
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(this, clube, 0), || get_open_offers_for_club_players(this, clube, 0));
    PLACAR.record(igual && r == c, || format!("get_open_offers_for_club_players({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparador: agenda (aloca) e mexe nas listas de alvos: só a original. O plano antes: as ofertas pela pessoa (0x8c5cf0
/// numa lista própria, destruída depois); depois confere que todas ficaram no estado 0xf com +0x2e = 3.
unsafe extern "thiscall" fn comparar_collapse_offers_for_player(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
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
    PLACAR.record(ok, || format!("collapse_offers_for_player(pessoa {}): {} ofertas, estados {:x?}", cpr4_d(pessoa), alvo.len(), alvo.iter().map(|&of| (cpr4_b(of + 0x2c), cpr4_b(of + 0x2e))).collect::<Vec<_>>()));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
unsafe extern "thiscall" fn comparar_add_next_sale_percentage(this: usize, registro: usize) {
    let n = cpr4_d(this + 0x888);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, registro);
    if cpr4_d(this + 0x880) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpr4_d(this + 0x888);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(cpr4_d(this + 0x880), conta, 0xd, &alvo);
    PLACAR.record(ok, || format!("add_next_sale_percentage: conta {} -> {}, registro {:x?}", n, conta, alvo));
}
