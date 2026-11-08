// ---------- %%LEVA%%a leva: a lista de observação do técnico humano: o jogador na busca, as listas dos olheiros acertadas com os olheiros do clube e a vaga do jogador no registro do clube observado (player_search.cpp, scout_manager.cpp) ----------

unsafe fn cqa6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x7af600 } (thiscall, ret 4, player_search.cpp, quadro SEH só pelas duas datas locais, sem destrutor):
/// `PLAYER_SEARCH::contains_player(pessoa)`, da lista do olheiro do técnico humano (0x7f7490, que a avaliação dos
/// jogadores e os clubes interessados consultam a cada dia) e de telas. A chave local de 0x2d bytes começa com as duas
/// datas padrão (+0x10 e +0x18, 0x54a110); pessoa nula: a caixa da linha 0x33e e 0. Sem resultados (+0x1f6 = conta):
/// 0. A referência ao nome da pessoa na chave (0x7e4920: +0xc o id, o único campo que a comparação 0x7bce50 lê) e a
/// busca binária nos resultados (0x946464 com o vetor +0x1ee, a conta, 0x2d e 0x7bce50); achada e sem a marca de
/// removida (bit 0 de +0x20), 1; senão 0.
unsafe extern "thiscall" fn contains_player(this: usize, pessoa: usize) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut chave = [0u8; 0x30];
    let k = chave.as_mut_ptr() as usize;
    data_padrao(k + 0x10);
    data_padrao(k + 0x18);
    if pessoa == 0 {
        show_error_at(0xa701b0, 0x33e);
        return 0;
    }
    if cqa6_d(this + 0x1f6) == 0 {
        crate::comparacao::ramo("cqa_6 busca vazia");
        return 0; // cqa6: sem resultados, não está
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    referencia(pessoa, k);
    let bsearch: unsafe extern "C" fn(usize, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(k, cqa6_d(this + 0x1ee), cqa6_d(this + 0x1f6), 0x2d, 0x7bce50);
    if r == 0 {
        return 0;
    }
    if cqa6_b(r + 0x20) & 1 != 0 { // cqa6: a removida não conta
        crate::comparacao::ramo("cqa_6 removido");
        return 0;
    }
    crate::comparacao::ramo("cqa_6 achado");
    1 // cqa6: achado
}

/// Comparador da 0x7af600: pela devolução (só lê; as datas e a chave são locais). Pessoa nula: a caixa, só a original.
unsafe extern "thiscall" fn comparar_contains_player(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = o(this, pessoa);
    let n = contains_player(this, pessoa);
    PLACAR.registrar(r == n, || format!("contains_player(busca {:x}, pessoa {:x}) original={} rust={}", this, pessoa, r, n));
    r
}

/// A busca `i` (0..9) do técnico `m` (0x7f6cd0: thiscall(gerência, i, técnico), ret 8; lê só o byte do índice).
unsafe fn get_search(this: usize, i: u8, m: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    f(this, i as u32, m)
}

/// O olheiro da busca (0x7ae950: thiscall(busca)).
unsafe fn search_scout(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7ae950usize);
    f(l)
}

/// Põe o olheiro na busca (0x7ae8a0: thiscall(busca, olheiro), ret 4).
unsafe fn set_search_scout(l: usize, olheiro: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7ae8a0usize);
    f(l, olheiro);
}

/// { 0x7f7270 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::sync_scout_lists(técnico)`, quando a comissão do
/// clube humano muda (a saída do cargo 0x690890 e a da vaga dos olheiros, as transferências e contratações de pessoal).
/// Sem técnico, o humano da vez; sem ele, a caixa da linha 0x1ba e 0. Das 10 buscas do técnico, as listas de olheiro
/// (+0xa = 4) cujo olheiro (0x7ae950) não é mais olheiro (+0x3d = 9) do clube do técnico (+0x39) ficam sem olheiro
/// (0x7ae8a0 com 0; sem clube, todas; com clube, as sem olheiro ficam como estão). Com clube, cada um dos 7 olheiros do
/// clube (0x53cd60) que não tem lista vai para a primeira lista de olheiro sem olheiro. Depois a vaga do técnico (byte
/// +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x212 e 0): com a edição aberta ([this + vaga × 4 +
/// 0xc]), ela refaz os olheiros das listas (0x8043c0). Devolve 1.
unsafe extern "thiscall" fn sync_scout_lists(this: usize, tecnico: usize) -> u32 {
    let m = if tecnico != 0 { tecnico } else { scout_manager_current_human_manager() };
    if m == 0 {
        show_error_at(0xa7b094, 0x1ba);
        return 0;
    }
    let clube = cqa6_d(m + 0x39);
    for i in 0..10u8 {
        let l = get_search(this, i, m);
        if l == 0 || cqa6_b(l + 0xa) != 4 {
            continue;
        }
        let o = search_scout(l);
        if clube != 0 {
            if o == 0 || (cqa6_d(o + 0x39) == clube && cqa6_b(o + 0x3d) == 9) {
                continue;
            }
        }
        crate::comparacao::ramo("cqa_6 lista sem olheiro");
        set_search_scout(l, 0);
    }
    if clube != 0 {
        let olheiros: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x53cd60usize);
        let mut v = [0usize; 7];
        olheiros(clube, v.as_mut_ptr());
        for &s in v.iter() {
            if s == 0 {
                continue;
            }
            let mut livre: i8 = -1;
            let mut tem = false;
            for i in 0..10u8 {
                let l = get_search(this, i, m);
                if l == 0 || cqa6_b(l + 0xa) != 4 {
                    continue;
                }
                if search_scout(l) == s {
                    tem = true;
                    break;
                }
                if search_scout(l) == 0 && livre == -1 {
                    livre = i as i8;
                }
            }
            if !tem && livre > -1 {
                crate::comparacao::ramo("cqa_6 olheiro na lista livre");
                set_search_scout(get_search(this, livre as u8, m), s);
            }
        }
    }
    let vaga = cqa6_b(m).wrapping_sub(cqa6_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        show_error_at(0xa7b094, 0x212);
        return 0;
    }
    let s = cqa6_d(this + vaga as usize * 4 + 0xc);
    if s != 0 {
        let refazer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8043c0usize);
        refazer(s);
    }
    1
}

/// Comparador da 0x7f7270: lado a lado nas 10 buscas do técnico (0x2b0 bytes cada, achadas antes pela 0x7f6cd0, que
/// só lê) e na devolução. Sem técnico ou com a edição aberta (a 0x8043c0 mexe na edição): só a original.
unsafe extern "thiscall" fn comparar_sync_scout_lists(this: usize, tecnico: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    let m = if tecnico != 0 { tecnico } else { scout_manager_current_human_manager() };
    if m == 0 {
        return o(this, tecnico);
    }
    let vaga = cqa6_b(m).wrapping_sub(cqa6_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) || cqa6_d(this + vaga as usize * 4 + 0xc) != 0 {
        return o(this, tecnico);
    }
    let mut regioes = Vec::new();
    for i in 0..10u8 {
        let l = get_search(this, i, m);
        if l != 0 {
            regioes.push((l, 0x2b0usize));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico), || sync_scout_lists(this, tecnico));
    PLACAR.registrar(igual && r == n, || format!("sync_scout_lists(técnico {:x}) original={} rust={} {:x?} x {:x?}", m, r, n, mo, mr));
    r
}

/// { 0x7bd700 } (thiscall, ret 8, player_search.cpp): `player_search_find_or_claim_player_slot(pessoa, reservar)`, do
/// registro de 50 vagas (0x1a2 bytes a partir de +4: +0 o id, −1 livre) do clube observado, pela partida dele
/// (0x7bd310) e pelas telas (0x7f8610). A vaga com o id da pessoa; senão, com `reservar`, a primeira livre recebe o id
/// (pessoa nula: −1), zera a palavra +4 e os bytes +0x1a0 e +0x1a1 e é devolvida (nenhuma livre: a caixa da linha
/// 0x210d e 0); sem `reservar`, 0.
unsafe extern "thiscall" fn player_search_find_or_claim_player_slot(this: usize, pessoa: usize, reservar: u32) -> usize {
    if pessoa == 0 {
        // ponytail: a original lê o id da pessoa antes de conferir o ponteiro (cairia); fica com ela
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I2%%)(this, pessoa, reservar);
    }
    let id = cqa6_d(pessoa) as i32;
    let vaga = |i: i8| this + 4 + i as usize * 0x1a2;
    let mut livre: i8 = -1;
    for i in 0..0x32i8 {
        let v = cqa6_d(vaga(i)) as i32;
        if v == id {
            return vaga(i);
        }
        if reservar != 0 && livre == -1 && v == -1 {
            livre = i;
        }
    }
    if reservar == 0 {
        return 0;
    }
    if livre == -1 {
        show_error_at(0xa701b0, 0x210d);
        return 0;
    }
    let a = vaga(livre);
    core::ptr::write_unaligned(a as *mut i32, id);
    core::ptr::write_unaligned((a + 4) as *mut u16, 0);
    *((a + 0x1a0) as *mut u8) = 0;
    *((a + 0x1a1) as *mut u8) = 0;
    a
}

/// Comparador da 0x7bd700: sem `reservar`, pela devolução (só lê); com ele, lado a lado nas 50 vagas e na devolução.
unsafe extern "thiscall" fn comparar_player_search_find_or_claim_player_slot(this: usize, pessoa: usize, reservar: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I2%%);
    if pessoa == 0 {
        return o(this, pessoa, reservar);
    }
    if reservar == 0 {
        let r = o(this, pessoa, reservar);
        let n = player_search_find_or_claim_player_slot(this, pessoa, reservar);
        PLACAR.registrar(r == n, || format!("player_search_find_or_claim_player_slot(registro {:x}, pessoa {:x}) original={:x} rust={:x}", this, pessoa, r, n));
        return r;
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this + 4, 0x32 * 0x1a2)], || o(this, pessoa, reservar), || player_search_find_or_claim_player_slot(this, pessoa, reservar));
    PLACAR.registrar(igual && r == n, || format!("player_search_find_or_claim_player_slot(registro {:x}, pessoa {:x}, reservar) original={:x} rust={:x} iguais={}", this, pessoa, r, n, igual));
    r
}
