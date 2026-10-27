// ---------- %%LEVA%%a leva: a cópia da entrada da fila de regens e a escolha do doador de dados (player_regen.cpp) e a busca na lista de observados (shortlist_manager.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || copiar_entrada_da_fila_de_regens(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_entrada_da_fila_de_regens {:x?} x {:x?}", mo, mr));
    r
}

comparar_valor!(comparar_esta_na_lista_de_observados, esta_na_lista_de_observados, %%I1%%, (jogador: usize, clube: usize) -> u32);

/// Comparador: a 0x7a7150 sorteia (0x90cfc0) a cada pessoa; lado a lado pelo estado do sorteio, a cópia sem contar.
unsafe extern "thiscall" fn comparar_escolher_doador_de_dados(this: usize, nacao: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao),
        || crate::aleatorio::sem_contar(|| escolher_doador_de_dados(this, nacao)));
    PLACAR.registrar(igual && r == n, || format!("escolher_doador_de_dados({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}
