// ---------- %%LEVA%%a leva: o recorde do jogador mais velho e as permissões de trabalho: achar a do jogador e se ele passa no limite de estrangeiros ----------

unsafe fn cpj1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x450cd0 } (thiscall, club_records.cpp): `oldest_player_record_construct`: constrói o recorde do jogador mais
/// velho: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). O quadro SEH do original não
/// é preciso (a 0x54a110 não falha). Devolve `this`.
unsafe extern "thiscall" fn oldest_player_record_construct(this: usize) -> usize {
    init_default_date(this);
    init_default_date(this + 0x18); // a data do mais velho
    init_default_date(this + 0x20);
    this
}

/// { 0x8c7260 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_work_permit(pessoa)`: a permissão
/// de trabalho da pessoa: id fora dos [0xae2364] (com sinal), 0; sem a marca 0x20 no byte do id (8 bytes cada, em
/// [+0x86e]), 0; senão a busca binária (bsearch 0x946464, registros de 0xe bytes em [+0x93a], [+0x93e] deles,
/// comparação 0x8e55c0) pelo id. Pessoa nula: lê o endereço 0 como o original.
unsafe extern "thiscall" fn find_work_permit(this: usize, pessoa: *const i32) -> usize {
    let id = core::ptr::read_volatile(pessoa);
    if !pessoa.is_null() && id >= cpj1_d(0xae2364) as i32 {
        return 0;
    }
    if *((cpj1_d(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) as *const u8) & 0x20 == 0 {
        return 0; // sem a marca da permissão
    }
    let chave = id;
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(&chave, cpj1_d(this + 0x93a), cpj1_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// { 0x84b180 } (stdcall, ret 0xc, shortlist_manager.cpp): `passes_work_permit_check(jogador, _, comprador)`: o
/// jogador pode ir para o clube comprador? Sem nação (+0x1a): não. Nação do comprador (+0x53) sem a regra 0xe (+0x76),
/// a mesma nação, a 2ª nação (+0x1e) ou comunitário (0x53d740): sim. Sem seleção (+0x24) nem jogos por ela (+0x22):
/// não. Senão a permissão de trabalho (0x8c7950 sobre [0xde1f54], sem oferta, registro nem sorteio). O 2º argumento
/// não é usado.
unsafe extern "stdcall" fn passes_work_permit_check(jogador: usize, _atual: usize, comprador: usize) -> u8 {
    let nacao = cpj1_d(jogador + 0x1a);
    if nacao == 0 {
        return 0;
    }
    let destino = cpj1_d(comprador + 0x53);
    if *((destino + 0x76) as *const u8) != 0xe || nacao == destino || cpj1_d(jogador + 0x1e) == destino {
        return 1;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    if comunitaria(jogador, destino) != 0 {
        return 1; // comunitário
    }
    if cpj1_d(jogador + 0x24) == 0 && *((jogador + 0x22) as *const u8) == 0 {
        return 0;
    }
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u8 = core::mem::transmute(0x8c7950usize);
    permissao(cpj1_d(0xde1f54), jogador, destino, 0, 0, 0)
}

unsafe extern "thiscall" fn comparar_oldest_player_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || oldest_player_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("oldest_player_record_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: pelo valor (a busca só lê).
unsafe extern "thiscall" fn comparar_find_work_permit(this: usize, pessoa: *const i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> usize>(%%I1%%)(this, pessoa);
    let n = find_work_permit(this, pessoa);
    PLACAR.registrar(r == n, || format!("find_work_permit({:x}) original={:#x} rust={:#x}", pessoa as usize, r, n));
    r
}

/// Comparador: a 0x8c7950 (sem sorteio pedido) pode mexer no sorteio: lado a lado pelo estado dele, a cópia em
/// sem_contar.
unsafe extern "stdcall" fn comparar_passes_work_permit_check(jogador: usize, atual: usize, comprador: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, usize) -> u8>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(jogador, atual, comprador),
        || crate::aleatorio::sem_contar(|| passes_work_permit_check(jogador, atual, comprador)));
    PLACAR.registrar(igual && r == n, || format!("passes_work_permit_check({:x}, {:x}) original={} rust={} {:x?} x {:x?}", jogador, comprador, r, n, mo, mr));
    r
}
