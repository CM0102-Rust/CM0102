// ---------- 243a leva: as cópias dos recordes de nota, de melhor em campo e de disciplina dos recordes do clube (club_records.cpp) ----------

/// O recordista (0x18 bytes) de `origem` em `destino`, pela 0x448490 (`record_holder_assign`, já em Rust).
unsafe fn cpf1_recordista(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    copiar(destino, origem);
}

/// { 0x448230 } (thiscall, ret 4, club_records.cpp): `rating_record_assign`: copia o recorde de nota (0x1f bytes) de
/// `origem`: a nota (+0, dupla), o recordista em +4, o byte +0x1c e a palavra +0x1d. Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_nota(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32)); // a nota
    cpf1_recordista(this + 4, origem + 4);
    *((this + 0x1c) as *mut u8) = *((origem + 0x1c) as *const u8); // o byte do recorde de nota
    core::ptr::write_unaligned((this + 0x1d) as *mut u16, core::ptr::read_unaligned((origem + 0x1d) as *const u16));
    this
}

/// { 0x4484d0 } (thiscall, ret 4, club_records.cpp): `man_of_match_record_assign`: copia o recorde de melhor em campo
/// (0x1b bytes) de `origem`: as vezes (+0, byte), o recordista em +1 e o ano da temporada (+0x19). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_melhor_em_campo(this: usize, origem: usize) -> usize {
    *(this as *mut u8) = *(origem as *const u8); // as vezes de melhor em campo
    cpf1_recordista(this + 1, origem + 1);
    core::ptr::write_unaligned((this + 0x19) as *mut u16, core::ptr::read_unaligned((origem + 0x19) as *const u16));
    this
}

/// { 0x448500 } (thiscall, ret 4, club_records.cpp): `discipline_record_assign`: copia o recorde de disciplina (0x1c
/// bytes) de `origem`: os amarelos (+0) e vermelhos (+1), o recordista em +2 e o ano da temporada (+0x1a). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_disciplina(this: usize, origem: usize) -> usize {
    *(this as *mut u8) = *(origem as *const u8);
    *((this + 1) as *mut u8) = *((origem + 1) as *const u8); // os vermelhos
    cpf1_recordista(this + 2, origem + 2);
    core::ptr::write_unaligned((this + 0x1a) as *mut u16, core::ptr::read_unaligned((origem + 0x1a) as *const u16));
    this
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_nota(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(808);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, origem), || copiar_recorde_de_nota(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_nota {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_melhor_em_campo(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(809);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1b)], || o(this, origem), || copiar_recorde_de_melhor_em_campo(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_melhor_em_campo {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_disciplina(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(810);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1c)], || o(this, origem), || copiar_recorde_de_disciplina(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_disciplina {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 243a leva: a cópia do recorde de contagem e do torcedor e a construção dos recordes de contagem (club_records.cpp) ----------

/// { 0x448530 } (thiscall, ret 4, club_records.cpp): `count_record_assign`: copia o recorde de contagem (0x1a bytes) de
/// `origem`: a contagem (+0, palavra) e o recordista em +2 (pela 0x448490, já em Rust). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_contagem(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u16, core::ptr::read_unaligned(origem as *const u16)); // a contagem
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    copiar(this + 2, origem + 2);
    this
}

/// { 0x4485b0 } (thiscall, ret 4, club_records.cpp): `fans_player_record_assign`: copia o recorde de jogador da
/// torcida (0x1a bytes) de `origem`: o recordista em +0 (pela 0x448490) e o ano da temporada (+0x18). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_do_torcedor(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    copiar(this, origem);
    core::ptr::write_unaligned((this + 0x18) as *mut u16, core::ptr::read_unaligned((origem + 0x18) as *const u16)); // o ano do torcedor
    this
}

/// A data do recordista em `a` pela 0x54a110 (o construtor da data: dia 40, ano 1972, +4 = 1).
unsafe fn cpf2_data_padrao(a: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(a);
}

/// { 0x450c60 } (thiscall, club_records.cpp): `tally_record_construct`: constrói o recorde de ocorrências (contagem em
/// byte + recordista): só a data do recordista em +0x11 (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_ocorrencias(this: usize) -> usize {
    cpf2_data_padrao(this + 0x11); // a data das ocorrências
    this
}

/// { 0x450d30 } (thiscall, club_records.cpp): `count_record_construct`: constrói o recorde de contagem (palavra +
/// recordista): só a data do recordista em +0x12 (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_contagem(this: usize) -> usize {
    cpf2_data_padrao(this + 0x12); // a data da contagem
    this
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_contagem(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(811);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || copiar_recorde_de_contagem(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_contagem {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_recorde_do_torcedor(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(812);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || copiar_recorde_do_torcedor(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_do_torcedor {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_ocorrencias(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(813);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x19)], || o(this), || construir_recorde_de_ocorrencias(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_ocorrencias {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_contagem(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(814);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this), || construir_recorde_de_contagem(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_contagem {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 243a leva: as cópias do recorde de valor de transferência e do recorde de gols numa partida (club_records.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cpf3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x448550 } (thiscall, ret 4, club_records.cpp): `transfer_fee_record_assign`: copia o recorde de valor de
/// transferência (0x29 bytes) de `origem`: cinco duplas (+0..+0x10: o valor, o jogador, o outro clube...), a data em
/// +0x14, a dupla +0x1c, a data em +0x20 e a marca de troca (+0x28). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_valor(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    for o in [0, 4, 8, 0xc, 0x10] {
        d(o); // o valor e os ids
    }
    cpf3_data(this + 0x14, origem + 0x14);
    d(0x1c);
    cpf3_data(this + 0x20, origem + 0x20);
    *((this + 0x28) as *mut u8) = *((origem + 0x28) as *const u8); // a marca de troca
    this
}

/// { 0x451570 } (thiscall, ret 4, club_records.cpp): `goals_in_match_record_assign`: copia o recorde de gols numa
/// partida (0x38 bytes) de `origem`: os gols (+0, byte), o recordista em +1 (0x448490) e o registro da partida em +0x19
/// (0x448110, ambos já em Rust). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_gols_na_partida(this: usize, origem: usize) -> usize {
    let recordista: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    let partida: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448110usize);
    *(this as *mut u8) = *(origem as *const u8); // os gols
    recordista(this + 1, origem + 1);
    partida(this + 0x19, origem + 0x19);
    this
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_valor(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(815);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x29)], || o(this, origem), || copiar_recorde_de_valor(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_valor {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_gols_na_partida(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(816);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x38)], || o(this, origem), || copiar_recorde_de_gols_na_partida(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_gols_na_partida {:x?} x {:x?}", mo, mr));
    r
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

// ---------- 243a leva: a cópia da entrada da fila de regens e a escolha do doador de dados (player_regen.cpp) e a busca na lista de observados (shortlist_manager.cpp) ----------

unsafe fn cpg2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x7a8b70 } (thiscall, ret 4, player_regen.cpp): `regen_queue_entry_assign`: copia a entrada da fila de regens
/// (0xc bytes) de `origem`: a pessoa (+0) e a data do regen em +4 (0x417c10, retorno numa local). Devolve `this`.
unsafe extern "thiscall" fn copiar_entrada_da_fila_de_regens(this: usize, origem: usize) -> usize {
    core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32)); // a pessoa do regen
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(this + 4, &mut local, origem + 4);
    this
}

/// { 0x842080 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::is_on_shortlist(jogador, clube)`: sem
/// vetor de pares ([this]) devolve 0; senão procura (bsearch 0x946464, [this+0x10] pares de 8 bytes, comparação
/// 0x757ea0) a chave (id do jogador, id do clube ou −1 sem clube). O al é 1 se achou; os 3 bytes altos do eax são os do
/// ponteiro achado (setne al sobre o retorno do bsearch), e a cópia os reproduz.
unsafe extern "thiscall" fn esta_na_lista_de_observados(this: usize, jogador: usize, clube: usize) -> u32 {
    let v = cpg2_d(this);
    if v == 0 {
        return 0;
    }
    let chave: [u32; 2] = [cpg2_d(jogador) as u32, if clube != 0 { cpg2_d(clube) as u32 } else { u32::MAX }]; // sem clube: −1
    let bsearch: unsafe extern "C" fn(*const [u32; 2], usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    let p = bsearch(&chave, v, cpg2_d(this + 0x10), 8, 0x757ea0);
    (p as u32 & !0xff) | (p != 0) as u32
}

/// { 0x7a6e60 } (thiscall, ret 4, player_regen.cpp): `player_regen_pick_staff_for_non_player_data(nacao)`: entre as
/// pessoas não jogadoras ([0xae23bc], 0x6e bytes, [0xae2364] − 16 delas, contagem relida a cada volta), a de maior
/// nota positiva da 0x7a7150 (sorteia; o ecx de entrada vai junto, como na original) da nação `nacao` ([+0x1a]+0 =
/// nacao; nota inicial −1000); sem nenhuma, a de maior nota positiva de qualquer nação (inicial −15000); senão 0.
unsafe extern "thiscall" fn escolher_doador_de_dados(this: usize, nacao: usize) -> usize {
    let nota: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x7a7150usize);
    let quantas = || (*(0xae2364 as *const i32)).wrapping_sub(0x10);
    let base = cpg2_d(0xae23bc);
    let (mut melhor, mut escolhida) = (-1000i32, 0usize);
    let mut i = 0;
    while i < quantas() {
        let p = base + 0x6e * i as usize;
        let n = cpg2_d(p + 0x1a);
        if n != 0 && cpg2_d(n) == nacao {
            let s = nota(this, p);
            if s > melhor && s > 0 { // a melhor da nação
                melhor = s;
                escolhida = p;
            }
        }
        i += 1;
    }
    if escolhida != 0 {
        return escolhida;
    }
    crate::comparacao::ramo("doador de qualquer nação");
    melhor = -15000;
    let base = cpg2_d(0xae23bc); // relida, como na original
    i = 0;
    while i < quantas() {
        let p = base + 0x6e * i as usize;
        let s = nota(this, p);
        if s > melhor && s > 0 {
            melhor = s;
            escolhida = p;
        }
        i += 1;
    }
    escolhida
}

unsafe extern "thiscall" fn comparar_copiar_entrada_da_fila_de_regens(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(820);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || copiar_entrada_da_fila_de_regens(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_entrada_da_fila_de_regens {:x?} x {:x?}", mo, mr));
    r
}

comparar_valor!(comparar_esta_na_lista_de_observados, esta_na_lista_de_observados, 821, (jogador: usize, clube: usize) -> u32);

/// Comparador: a 0x7a7150 sorteia (0x90cfc0) a cada pessoa; lado a lado pelo estado do sorteio, a cópia sem contar.
unsafe extern "thiscall" fn comparar_escolher_doador_de_dados(this: usize, nacao: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(822);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao),
        || crate::aleatorio::sem_contar(|| escolher_doador_de_dados(this, nacao)));
    PLACAR.registrar(igual && r == n, || format!("escolher_doador_de_dados({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

// ---------- 243a leva: a cópia do registro de suspensão (discipline.cpp), o salário padrão do contrato (staff_contracts.cpp) e a condição de jogo (injury.cpp) ----------

/// { 0x556c70 } (thiscall, ret 4, discipline.cpp): `BanRecord::operator=`: copia o registro de suspensão (0x14 bytes) de
/// `origem`: +0 e +1 (bytes), +2 (palavra), a data em +4 (0x417c10, retorno numa local), +0xc..+0xf (byte a byte),
/// +0x10 (byte), +0x11 (palavra) e os bits 0..2 de +0x13 (os outros bits ficam). Devolve `this`.
unsafe extern "thiscall" fn copiar_registro_de_suspensao(this: usize, origem: usize) -> usize {
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    b(0);
    b(1);
    w(2);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(this + 4, &mut local, origem + 4);
    for o in 0xc..0x10 {
        b(o);
    }
    b(0x10);
    w(0x11);
    let m = (this + 0x13) as *mut u8;
    *m = (*m & !7) | (*((origem + 0x13) as *const u8) & 7); // as três marcas
    this
}

/// { 0x8626e0 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_default_wage(pessoa)`: o salário (+0xc) é
/// o padrão da pessoa no clube dela ([+0x39]) pela 0x862720 (stdcall, 6 argumentos, float; o ecx vai junto, como na
/// original), truncado (0x945640), com mínimo 55. Devolve o truncado (antes do mínimo).
unsafe extern "thiscall" fn salario_padrao_do_contrato(this: usize, pessoa: usize) -> u32 {
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32, i32, i32) -> f64 = core::mem::transmute(0x862720usize);
    let v = ftol(padrao(this, pessoa, core::ptr::read_unaligned((pessoa + 0x39) as *const usize), 1, -1, 0xb, 0));
    *((this + 0xc) as *mut i32) = if v < 0x37 { 0x37 } else { v }; // o mínimo de 55
    v as u32
}

/// { 0x62c510 } (thiscall, ret 8, injury.cpp): `Injury::get_match_condition(jogador, em_porcento)`: sem jogador, 0; id
/// fora das pessoas ([0xae2364]): 10000 (100 em porcento); senão atualiza o físico do dia (0x62c680, uma vez por dia)
/// e devolve a condição (registro de 31 bytes em [this], +0xa), em porcento × 0,01f truncado. Só o ax é o retorno.
unsafe extern "thiscall" fn condicao_de_jogo_do_jogador(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    if jogador == 0 {
        return 0;
    }
    let id = *(jogador as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return if em_porcento != 0 { 100 } else { 10000 };
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    atualizar(this, jogador);
    let c = core::ptr::read_unaligned((*(this as *const usize)).wrapping_add((*(jogador as *const i32) as usize).wrapping_mul(31)).wrapping_add(0xa) as *const i16);
    if em_porcento != 0 {
        return ftol(c as f64 * *(0x96bac8 as *const f32) as f64) as u16; // em porcento
    }
    c as u16
}

unsafe extern "thiscall" fn comparar_copiar_registro_de_suspensao(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(823);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this, origem), || copiar_registro_de_suspensao(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_registro_de_suspensao {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: lado a lado pelo salário (+0xc). A 0x862720 grava a mesma ficha em [0xae1080] nas duas vezes.
unsafe extern "thiscall" fn comparar_salario_padrao_do_contrato(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(824);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 4)], || o(this, pessoa), || salario_padrao_do_contrato(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("salario_padrao_do_contrato({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// a original roda primeiro e atualiza o físico do dia (0x62c680, com sorteio); a cópia, depois, acha o dia já feito
comparar_valor!(comparar_condicao_de_jogo_do_jogador, condicao_de_jogo_do_jogador, 825, (jogador: usize, em_porcento: u32) -> u16);

// ---------- 243a leva: o contrato de pessoal (staff_contracts.cpp): os dias até o fim, limpar todas as queixas e conferir a retirada do pedido de transferência ----------

unsafe fn cph1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// A pessoa do contrato: o id em +0 no vetor de pessoas [0xae23bc] (110 bytes cada).
unsafe fn cph1_pessoa(contrato: usize) -> usize {
    (cph1_d(0xae23bc) as usize).wrapping_add((cph1_d(contrato) as usize).wrapping_mul(110))
}

/// { 0x85fa00 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::days_to_end_date`: os dias da data do jogo
/// (0xae2c90) até a data do fim do contrato (+0x2d), pela 0x54cdb0 (`dias_ate`, a data do jogo por valor). O jogo lê
/// só a palavra; devolve o eax inteiro da 0x54cdb0.
unsafe extern "thiscall" fn dias_ate_o_fim_do_contrato(this: usize) -> u32 {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x54cdb0usize);
    dias(this + 0x2d, cph1_d(0xae2c90), cph1_d(0xae2c94)) // a data do fim do contrato
}

/// { 0x867520 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::clear_all_grievances(conferir)`: zera as
/// queixas (+0x45), tira os vínculos de opinião da pessoa (0x549c90) e, com `conferir`, confere a retirada do pedido de
/// transferência (0x867240 com 0).
unsafe extern "thiscall" fn limpar_todas_as_queixas(this: usize, conferir: u32) {
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0); // as queixas
    let vinculos: unsafe extern "C" fn(usize) = core::mem::transmute(0x549c90usize);
    vinculos(cph1_pessoa(this));
    if conferir != 0 {
        let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
        retirada(this, 0);
    }
}

/// A decisão da 0x867240 (sem efeito além da ficha [0xae1080], gravada com o contrato da pessoa): o clube do contrato
/// (fora do tipo 5) existe, a pessoa é jogador (+0x61), o contrato é desse clube (+4 = id), pediu transferência (bit 3
/// de +0x4e), não está em +0x49, o critério da comissão (0x866f60) dá 0, o nibble alto de +0x4f não é 7 e não há
/// contrato acertado com outro clube (0x8c5450 em [0xde1f54]). Devolve a pessoa quando retira.
unsafe fn cph1_retira(this: usize) -> Option<usize> {
    let pessoa = cph1_pessoa(this);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cph1_d(0xae1050) as usize, pessoa);
    *(0xae1080 as *mut usize) = c; // a ficha do contrato
    let clube = if c != 0 && cph1_b(c + 0x35) & 0x3f != 5 {
        let i = cph1_d(c + 4) as usize;
        (cph1_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
    } else {
        0
    };
    if clube == 0 || cph1_d(pessoa + 0x61) == 0 || cph1_d(this + 4) != cph1_d(clube) {
        return None;
    }
    if cph1_b(this + 0x4e) & 8 == 0 || cph1_b(this + 0x49) != 0 {
        return None; // sem pedido de transferência
    }
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    if criterio(this) != 0 || cph1_b(this + 0x4f) & 0xf0 == 0x70 {
        return None;
    }
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5450usize);
    if acertado(cph1_d(0xde1f54) as usize, pessoa, 0) != 0 {
        return None;
    }
    Some(pessoa)
}

/// { 0x867240 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`:
/// quando a decisão (`cph1_retira`) manda, retira o pedido (0x4d9460 em [0xae1050] com o contrato e `ctx`), apaga o
/// bit 3 de +0x4e, a moral (+0x44) vira +0x5b + +0x59 + 5 da pessoa, zera +0x3e e sobe +0x3d a 15; e cada queixa
/// acesa (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 de +0x45, nessa ordem) é apagada, confere de novo (0x867240 com o
/// +0x45 de antes) e tira o vínculo de opinião (0x549d60 com o bit). Sem SEH, sem caixa, sem sorteio.
unsafe extern "thiscall" fn conferir_retirada_do_pedido(this: usize, ctx: u32) {
    let Some(pessoa) = cph1_retira(this) else { return };
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4d9460usize);
    retirar(cph1_d(0xae1050) as usize, this, ctx);
    *((this + 0x4e) as *mut u8) &= 0xf7; // o pedido retirado
    *((this + 0x44) as *mut u8) = cph1_b(pessoa + 0x5b).wrapping_add(cph1_b(pessoa + 0x59)).wrapping_add(5);
    if cph1_b(this + 0x3e) != 0 {
        *((this + 0x3e) as *mut u8) = 0;
    }
    if (cph1_b(this + 0x3d) as i8) < 15 {
        *((this + 0x3d) as *mut u8) = 15; // a felicidade mínima
    }
    let conferir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
    let vinculo: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x549d60usize);
    for bit in [0x10u32, 1, 4, 8, 0x100, 0x80, 0x10000] {
        let q = cph1_d(this + 0x45);
        if q & bit != 0 {
            core::ptr::write_unaligned((this + 0x45) as *mut u32, q & !bit); // a queixa apagada
            conferir(this, q);
            vinculo(cph1_pessoa(this), bit as i32);
        }
    }
}

// a 0x54cdb0 só lê: compara o valor
comparar_valor!(comparar_dias_ate_o_fim_do_contrato, dias_ate_o_fim_do_contrato, 826, () -> u32);

/// Comparador: sem `conferir`, lado a lado pelas queixas e pelo registro de opiniões da pessoa (79 bytes, o que a
/// 0x549c90 escreve); com `conferir`, a retirada (0x4d9460) mexe na transferência: só a original (a 0x867240 passa
/// pelo comparador dela).
unsafe extern "thiscall" fn comparar_limpar_todas_as_queixas(this: usize, conferir: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(827);
    if conferir != 0 {
        return o(this, conferir);
    }
    let mut reg = vec![(this + 0x45, 4)];
    if let Some(c) = registro_de_opinioes(cph1_pessoa(this)) {
        reg.push((c, 79));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, conferir), || limpar_todas_as_queixas(this, conferir));
    PLACAR.registrar(igual, || format!("limpar_todas_as_queixas({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparador: quando a decisão da cópia é não retirar, lado a lado pela ficha [0xae1080] e pelo contrato (0x50
/// bytes); quando retira, só a original (a 0x4d9460 mexe na transferência e a 0x867240 se chama de novo).
unsafe extern "thiscall" fn comparar_conferir_retirada_do_pedido(this: usize, ctx: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(828);
    let ficha = cph1_d(0xae1080);
    let retira = cph1_retira(this).is_some();
    *(0xae1080 as *mut u32) = ficha;
    if retira {
        return o(this, ctx);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4), (this, 0x50)], || o(this, ctx), || conferir_retirada_do_pedido(this, ctx));
    PLACAR.registrar(igual, || format!("conferir_retirada_do_pedido({:x}, {}) {:x?} x {:x?}", this, ctx, mo, mr));
}
