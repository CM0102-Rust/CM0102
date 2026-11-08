// ---------- %%LEVA%%a leva: a MLS: a virada da temporada, as posições previstas e a lista dos doze clubes (usa_mls.cpp) ----------

unsafe fn cqs2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`.
unsafe fn cqs2_linha(t: usize, i: usize) -> usize {
    cqs2_d(cqs2_d(t + 0xb1) + i * 0x3b)
}

/// { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::start_new_season()`, a virada da MLS: +0x4c = 0 e a tabela
/// liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o calendário (+0xba) liberado (free 0x9452ca) e
/// zerado; com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1); +0x30 = −1;
/// com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); o ano (+0x40, word)
/// sobe `ano` (o contador "cqs_2 mls rollover"); então, cada um com a sua caixa (linhas 0x4af, 0x4b6, 0x4bc e 0x4c3)
/// se devolver 0: as regras (virtual +0x8c), a tabela da primeira divisão (0x908560), os jogos da temporada
/// (UsaMls::create_season_fixtures 0x9086e0, que monta também as fases das outras duas divisões) e a ordem da tabela
/// (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as posições previstas (virtual +0x5c), as
/// expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com
/// caixa, a cópia devolve 0 (a original devolve o eax da caixa; a virada é chamada pelo laço das viradas, que não o lê).
/// O vetor das fases (+0xc) fica: as divisões e o play-off regravam as vagas.
unsafe fn mls_start_new_season(this: usize, ano: i16) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqs2_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    if (cqs2_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqs2_d(cqs2_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqs2_d(cqs2_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqs2_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let estatisticas = cqs2_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    let a = cqs2_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparison::branch("cqs_2 mls rollover");
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs2_d(cqs2_d(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(0xaa1fb4, 0x4af);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x908560usize);
    if tabela(this) == 0 {
        show_error_at(0xaa1fb4, 0x4b6);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x9086e0usize);
    if jogos(this) == 0 {
        show_error_at(0xaa1fb4, 0x4bc);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        show_error_at(0xaa1fb4, 0x4c3);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs2_d(cqs2_d(this) + 0x5c));
    previstas(this);
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqs2_d(0xb74340), cqs2_d(this + 4)) // cqs2: a renovação dos árbitros da MLS
}

/// { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): a virada da MLS (acima), um ano adiante.
unsafe extern "thiscall" fn usa_mls_start_new_season(this: usize) -> u32 {
    mls_start_new_season(this, 1) // cqs2: o ano seguinte da MLS
}

/// { 0x909bb0 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::set_predicted_positions()`, as posições previstas da MLS. Sem
/// estatísticas (+8): a caixa da linha 0x675 e volta. Um vetor de 3 × +0x3e clubes (malloc 0x944e46; sem memória: a
/// caixa da linha 0x67d e volta): os da tabela da liga, os da fase 0 e os da fase 1 (as três divisões, linha a linha),
/// cada bloco de +0x3e ordenado pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no fim). Os doze
/// primeiros do vetor ganham, em cada divisão, as posições 1, 4, 7 e 10 (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a ordem das doze chamadas é a do vetor); o vetor liberado (free 0x9452ca).
/// [inferência] As três divisões se misturam na previsão: o favorito de cada uma é previsto em 1º, o segundo em 4º...
unsafe extern "thiscall" fn usa_mls_set_predicted_positions(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4a2540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let n3 = cqs2_w(this + 0x3e).wrapping_mul(3);
    if cqs2_d(this + 8) == 0 {
        return show_error_at(0xaa1fb4, 0x675);
    }
    let v = alocar(((n3 as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0xaa1fb4, 0x67d);
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        let n = cqs2_w(this + 0x3e) as i32;
        let k = i as usize;
        core::ptr::write_unaligned((v + 4 * k) as *mut usize, cqs2_linha(this, k));
        core::ptr::write_unaligned((v + 4 * (k + n as usize)) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), k));
        core::ptr::write_unaligned((v + 4 * (k + 2 * n as usize)) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), k));
        i += 1;
    }
    let n = cqs2_w(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = cqs2_w(this + 0x3e) as i32;
    ordenar(v + 4 * n as usize, n as u32, 4, 0x4b07d0);
    let n = cqs2_w(this + 0x3e) as i32;
    ordenar(v + 8 * n as usize, n as u32, 4, 0x4b07d0);
    let lugares: [u32; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10]; // cqs2: as posições previstas das três divisões
    for (k, p) in lugares.iter().enumerate() {
        posicao(cqs2_d(this + 8), cqs2_d(v + 4 * k), *p);
    }
    crate::comparison::branch("cqs_2 expected positions");
    soltar(v);
}

/// { 0x909a40 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::get_club_array(&lista, &n)`, a lista dos clubes da MLS: *n = 12
/// (word); *lista = malloc 0x944e46 de 0x30 (sem memória: a caixa da linha 0x64c, *n = 0 e volta); os +0x3e clubes da
/// tabela da liga, depois os da fase 0 (a partir de +0x3e) e os da fase 1 (a partir de 2 × +0x3e), linha a linha.
unsafe extern "thiscall" fn usa_mls_get_club_array(this: usize, lista: *mut usize, n: *mut i16) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *n = 0xc;
    let v = alocar(0x30);
    *lista = v;
    if v == 0 {
        show_error_at(0xaa1fb4, 0x64c);
        *n = 0;
        return;
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        core::ptr::write_unaligned((*lista + 4 * i as usize) as *mut usize, cqs2_linha(this, i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        let k = cqs2_w(this + 0x3e) as i32 + i as i32;
        core::ptr::write_unaligned((*lista + 4 * k as usize) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        let k = 2 * cqs2_w(this + 0x3e) as i32 + i as i32;
        core::ptr::write_unaligned((*lista + 4 * k as usize) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), i as usize));
        i += 1;
    }
    crate::comparison::branch("cqs_2 mls clubs");
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1 e as três divisões montadas
/// (+0x3e = 4 e as vagas 0 e 1 das fases).
unsafe extern "thiscall" fn compare_usa_mls_start_new_season(this: usize) -> u32 {
    let antes = cqs2_w(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let depois = cqs2_w(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && cqs2_w(this + 0x3e) == 4 && cqs2_d(cqs2_d(this + 0xc)) != 0 && cqs2_d(cqs2_d(this + 0xc) + 4) != 0;
    SCOREBOARD.record(igual, || format!("usa_mls_start_new_season({:#x}) ano antes={} depois={} +0x3e={}", this, antes, depois, cqs2_w(this + 0x3e)));
    r
}

/// O comparador das posições previstas: só a original (ponytail: a 0x4a2540 acrescenta o registro de um clube que não
/// tem; não pode rodar duas vezes), com o plano feito antes: os doze clubes das três tabelas, cada bloco ordenado pela
/// mesma qsort numa cópia; depois, a posição (+0x72 do registro achado por 0x4a4850) de cada um.
unsafe extern "thiscall" fn compare_usa_mls_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let n = cqs2_w(this + 0x3e) as usize;
    if cqs2_d(this + 8) == 0 || n != 4 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v = [0usize; 12];
    for k in 0..4 {
        v[k] = cqs2_linha(this, k);
        v[4 + k] = cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), k);
        v[8 + k] = cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), k);
    }
    for b in 0..3 {
        ordenar(v[4 * b..].as_mut_ptr(), 4, 4, 0x4b07d0);
    }
    o(this);
    const LUGARES: [u8; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10];
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(cqs2_d(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) != LUGARES[k]
    });
    SCOREBOARD.record(errado.is_none(), || format!("usa_mls_set_predicted_positions({:#x}): a posição da vaga {:?} não bate ({:x?})", this, errado, v));
}

/// O comparador da lista: só a original (aloca a lista, que fica com quem chamou), com o plano: n = 12 e os doze clubes
/// das três tabelas na ordem.
unsafe extern "thiscall" fn compare_usa_mls_get_club_array(this: usize, lista: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I2%%)(this, lista, n);
    let m = cqs2_w(this + 0x3e) as usize;
    let igual = *n == 0xc && *lista != 0 && (0..m).all(|i| {
        cqs2_d(*lista + 4 * i) == cqs2_linha(this, i)
            && cqs2_d(*lista + 4 * (m + i)) == cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), i)
            && cqs2_d(*lista + 4 * (2 * m + i)) == cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), i)
    });
    SCOREBOARD.record(igual, || format!("usa_mls_get_club_array({:#x}) n={}", this, *n));
}
