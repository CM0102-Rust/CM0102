// ---------- %%LEVA%%a leva: a virada do futebol brasileiro: a Série A, o acesso e o rebaixamento em cadeia e a Série B (bra_nat_first.cpp, bra_nat_second.cpp) ----------

unsafe fn cqq8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A competição de id [`global`] (a tabela das competições [0xadadfc]).
unsafe fn comp_by_global(global: usize) -> usize {
    cqq8_d(cqq8_d(0xadadfc) + 4 * cqq8_d(global))
}

/// A virtual `deslocamento` de `c`, thiscall sem argumentos.
unsafe fn bra_nat_first_call_virtual_no_args(c: usize, deslocamento: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqq8_d(cqq8_d(c) + deslocamento));
    f(c)
}

/// O corpo comum da virada de uma Série (o mesmo da virada dos estaduais, cqp_7..9): a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada (free 0x9452ca) e zerada; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0,
/// as fases 0..=+0x30 que existem destruídas (a virtual 0 com 1); o ano (+0x40, word) sobe `ano` e +0x30 = −1; então,
/// cada um com a sua caixa se devolver 0: as regras (virtual +0x8c), a tabela (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c). Devolve falso depois de uma caixa.
unsafe fn rebuild_season(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16) -> bool {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar_tabela(this, 0);
    let l = cqq8_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqq8_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqq8_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqq8_d(cqq8_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqq8_d(cqq8_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqq8_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    if bra_nat_first_call_virtual_no_args(this, 0x8c) == 0 {
        show_error_at(arquivo, linhas[0]);
        return false;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        show_error_at(arquivo, linhas[1]);
        return false;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        show_error_at(arquivo, linhas[2]);
        return false;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        show_error_at(arquivo, linhas[3]);
        return false;
    }
    bra_nat_first_call_virtual_no_args(this, 0x5c);
    true
}

/// O fim comum: as expectativas da diretoria de cada clube (League::set_club_season_expectations 0x68aa80, thiscall,
/// ret 0) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4),
/// cujo eax a virada devolve.
unsafe fn set_expectations_and_renew_officials(this: usize) -> u32 {
    let expectativas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqq8_d(0xb74340), cqq8_d(this + 4)) // cqq8: os árbitros
}

/// As viradas das outras catorze competições do Brasil, na ordem da original: a Série B, a Série C e os doze estaduais
/// (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste, Paranaense, Pernambucano, Carioca, Catarinense e
/// Paulista: os globais de id 0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998, 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4,
/// 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4).
const CQQ8_VIRADAS: [usize; 14] = [0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998, 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4, 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4];

/// { 0x42a640 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::start_new_season()`, a virada do futebol brasileiro
/// (a Série A comanda): +0x4c = 0; o acesso e o rebaixamento em cadeia (a virtual +0xb0, 0x42ae90, thiscall(this, 1),
/// ret 4); os inscritos da Copa do Brasil (0x42b760), a entrada de 16 clubes na Série C (0x42b520) e as trocas dos
/// estaduais com as divisões de baixo (0x42b130), as três com mov ecx, esi e sem push (thiscall, ret 0; o mapa as diz
/// cdecl(void): o ecx sobra, sem efeito); o nível da divisão dos clubes da Série A e da Série B limitado
/// (League::limit_club_division_tier 0x68a980, thiscall(liga, 1, 3, −3, 1), ret 0x10); a Série A refeita (o corpo comum,
/// a tabela 0x42a480, caixas nas linhas 0x223, 0x22a, 0x230 e 0x237); as viradas das outras catorze (a virtual +8);
/// as expectativas e os árbitros.
unsafe extern "thiscall" fn bra_nat_first_start_new_season(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(cqq8_d(cqq8_d(this) + 0xb0));
    cadeia(this, 1);
    for f in [0x42b760usize, 0x42b520, 0x42b130] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this);
    }
    let nivel: unsafe extern "thiscall" fn(usize, u32, u32, i32, u32) = core::mem::transmute(0x68a980usize);
    nivel(this, 1, 3, -3, 1);
    nivel(comp_by_global(0x9cf9e0), 1, 3, -3, 1);
    if !rebuild_season(this, 0x42a480, 0x98aa30, [0x223, 0x22a, 0x230, 0x237], 1) {
        return 0;
    }
    for (k, g) in CQQ8_VIRADAS.iter().enumerate() {
        if k == 13 {
            crate::comparacao::ramo("cqq_8 virada do paulista");
        }
        bra_nat_first_call_virtual_no_args(comp_by_global(*g), 8); // cqq8: a virada de cada uma
    }
    set_expectations_and_renew_officials(this)
}

/// { 0x42ae90 } (thiscall, ret 4, bra_nat_first.cpp): `BraNatFirst::process_promotion_relegation_chain(int)` (slot 45,
/// +0xb0; o argumento não é lido): as posições finais (a virtual +0xa4, record_final_positions, thiscall, ret 0) da
/// Série A, da Série B, da Série C e das sete fases da Série C ([C+0xc][0..7]); depois o acesso e o rebaixamento
/// (league_promote_and_relegate 0x689c80, thiscall(this, de cima, de baixo, 1, 1, −1, −1), ret 0x18: a original põe
/// o ecx da Série A e a rotina o guarda) entre A e B, entre B e C e entre a B e cada uma das sete fases da C. Devolve o
/// eax da última.
unsafe extern "thiscall" fn bra_nat_first_process_promotion_relegation_chain(this: usize, _pares: u32) -> u32 {
    let (a, b, c) = (0x9cf9ec, 0x9cf9e0, 0x9cf9ac);
    bra_nat_first_call_virtual_no_args(comp_by_global(a), 0xa4);
    bra_nat_first_call_virtual_no_args(comp_by_global(b), 0xa4);
    bra_nat_first_call_virtual_no_args(comp_by_global(c), 0xa4);
    for k in 0..7 {
        bra_nat_first_call_virtual_no_args(cqq8_d(cqq8_d(comp_by_global(c) + 0xc) + 4 * k), 0xa4);
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, i32) -> u32 = core::mem::transmute(0x689c80usize);
    crate::comparacao::ramo("cqq_8 acesso e rebaixamento");
    trocar(this, comp_by_global(a), comp_by_global(b), 1, 1, -1, -1); // cqq8: A e B
    let mut r = trocar(this, comp_by_global(b), comp_by_global(c), 1, 1, -1, -1); // cqq8: B e C
    for k in 0..7 {
        r = trocar(this, comp_by_global(b), cqq8_d(cqq8_d(comp_by_global(c) + 0xc) + 4 * k), 1, 1, -1, -1);
    }
    r
}

/// { 0x42cc10 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::start_new_season()`, a virada da Série B (da
/// virada da Série A): +0x4c = 0, o corpo comum (a tabela 0x42ca70, caixas nas linhas 0x20c, 0x213, 0x219 e 0x220), as
/// expectativas e os árbitros.
unsafe extern "thiscall" fn bra_nat_first_start_new_season_42cc10(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    if !rebuild_season(this, 0x42ca70, 0x98ab64, [0x20c, 0x213, 0x219, 0x220], 1) {
        return 0;
    }
    crate::comparacao::ramo("cqq_8 virada da serie b");
    set_expectations_and_renew_officials(this)
}

/// Comparador das viradas: só a original (ponytail: libera e aloca tabelas e fases, sorteia jogos e árbitros e roda as
/// viradas das outras competições; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um.
unsafe fn cqq8_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

unsafe extern "thiscall" fn comparar_bra_nat_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqq8_comparar_virada("bra_nat_first_start_new_season", this, || o(this))
}

/// Comparador do acesso e rebaixamento: só a original (ponytail: move clubes entre as listas das ligas, que realocam).
/// A cópia roda no modo Rust (contador "cqq_8 acesso e rebaixamento") e é provada pelas mutações e pelas carreiras.
unsafe extern "thiscall" fn comparar_bra_nat_first_process_promotion_relegation_chain(this: usize, pares: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%)(this, pares)
}

unsafe extern "thiscall" fn comparar_bra_nat_first_start_new_season_42cc10(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqq8_comparar_virada("bra_nat_first_start_new_season_42cc10", this, || o(this))
}
