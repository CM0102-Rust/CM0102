// ---------- %%LEVA%%a leva: a virada do futebol brasileiro e da J2: o acesso e o rebaixamento entre os estaduais e as divisões de baixo, os 16 que entram na Série C e a virada da J2 (bra_nat_first.cpp, jap_j2.cpp) ----------

unsafe fn cqv3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// As chaves dos doze estaduais (a liga em [[0xadadfc] + chave × 4]) e as das divisões de baixo de cada um (o registro
/// em [0xae23d0] + chave × 0x6b), na ordem da original.
const CQV3_ESTADUAIS: [usize; 12] = [0x9cf9e4, 0x9cf9dc, 0x9cf9d4, 0x9cf9d0, 0x9cf998, 0x9cf9bc, 0x9cf9c0, 0x9cf9b0, 0x9cf9b4, 0x9cf994, 0x9cf9a4, 0x9cf9c8];
const CQV3_DE_BAIXO: [usize; 12] = [0x9cf99c, 0x9cf97c, 0x9cf988, 0x9cf98c, 0x9cf96c, 0x9cf974, 0x9cf968, 0x9cf970, 0x9cf984, 0x9cf978, 0x9cf980, 0x9cf990];

unsafe fn cqv3_liga(chave: usize) -> usize {
    cqv3_d(cqv3_d(0xadadfc) + 4 * cqv3_d(chave))
}

unsafe fn cqv3_registro(chave: usize) -> usize {
    cqv3_d(0xae23d0) + cqv3_d(chave) * 0x6b
}

/// O clube `i` do banco (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqv3_clube(i: usize) -> usize {
    cqv3_d(0xae23b4) + i * 0x245
}

/// O que um clube que sai de uma divisão sofre na original (os dois rebaixamentos): sem poder ser dirigido (0x544820,
/// cdecl, devolve al = 0) e com técnico humano decidindo por ele (0x600a50(clube, 1, 1, 0, 0), cdecl), o técnico sai
/// (MANAGER_MANAGER::remove_club_manager 0x69b790, thiscall([0xb67a44], clube, 1, 7, 0), ret 0x10); depois as cláusulas
/// de rebaixamento dos contratos (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, thiscall([0xae1050], clube,
/// registro da divisão que ele deixou), ret 8).
unsafe fn cqv3_saiu(clube: usize, registro: usize) {
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x69b790usize);
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cd030usize);
    if dirigivel(clube) == 0 && humano(clube, 1, 1, 0, 0) != 0 {
        tirar(cqv3_d(0xb67a44), clube, 1, 7, 0);
    }
    clausulas(cqv3_d(0xae1050), clube, registro);
}

/// { 0x42b130 } (cdecl, bra_nat_first.cpp, 1.001 bytes): `bra_nat_first_exchange_state_league_clubs()`, da virada do
/// futebol brasileiro. Os doze estaduais (CQV3_ESTADUAIS) e as doze divisões de baixo (CQV3_DE_BAIXO); quantos sobem de
/// cada divisão = as vagas de rebaixamento do estadual dela (+0xc1, byte com sinal).
/// 1) Uma lista de 200 clubes (malloc 0x944e46 de 0x320; sem memória: a caixa da linha 0x3d0 e volta) com os clubes do
///    banco cuja divisão regional (+0x60) é uma das doze de baixo, na ordem do banco, até 200.
/// 2) A lista ordenada pela reputação e os 75 primeiros embaralhados (comp_util_sort_and_shuffle_top_clubs 0x4afbd0,
///    cdecl(lista, n, 25): sorteia).
/// 3) Na ordem da lista, cada clube sobe para o estadual da sua divisão enquanto há vagas (+0x60 = o registro do
///    estadual, +4 da liga); se tem nação de clube (+0x57) e está sem técnico (+0xcf = 0), abre a vaga de técnico
///    (MANAGER_MANAGER::add_job_vacancy 0x69ba50, thiscall([0xb67a44], clube, 0), ret 8).
/// 4) Em cada estadual, os das últimas +0xc1 linhas da tabela (League::registro 0x6826d0) descem para a divisão de baixo
///    (+0x60 = o registro dela) e sofrem a saída (cqv3_saiu, com o registro do estadual).
/// 5) A lista é liberada (free 0x9452ca).
unsafe extern "C" fn cqv3_acesso_dos_estaduais() {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let vaga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x69ba50usize);
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let ligas: [usize; 12] = core::array::from_fn(|i| cqv3_liga(CQV3_ESTADUAIS[i]));
    let baixo: [usize; 12] = core::array::from_fn(|i| cqv3_registro(CQV3_DE_BAIXO[i]));
    let mut vagas: [i8; 12] = core::array::from_fn(|i| *((ligas[i] + 0xc1) as *const i8));
    let lista = alocar(0x320);
    if lista == 0 {
        return show_error_at(0x98aa30, 0x3d0);
    }
    let mut n = 0i16;
    let mut i = 0usize;
    while (i as i32) < cqv3_d(0xae235c) as i32 && n < 0xc8 {
        let c = cqv3_clube(i);
        let d = cqv3_d(c + 0x60);
        if d != 0 && baixo.contains(&d) {
            core::ptr::write_unaligned((lista + 4 * n as usize) as *mut usize, c);
            n += 1;
        }
        i += 1;
    }
    embaralhar(lista, n as u16 as u32, 0x19);
    for k in 0..n.max(0) as usize { // cqv3: a lista na ordem
        let c = cqv3_d(lista + 4 * k);
        let d = cqv3_d(c + 0x60);
        let e = match baixo.iter().position(|&b| b == d) {
            Some(e) => e,
            None => continue, // ponytail: todo clube da lista é de uma das doze
        };
        if vagas[e] > 0 { // cqv3: as vagas de acesso do estadual
            crate::comparison::branch("cqv_3 acesso ao estadual");
            core::ptr::write_unaligned((c + 0x60) as *mut usize, cqv3_d(ligas[e] + 4));
            if cqv3_d(c + 0x57) != 0 && cqv3_d(c + 0xcf) == 0 {
                vaga(cqv3_d(0xb67a44), c, 0);
            }
            vagas[e] -= 1;
        }
    }
    for e in 0..12 {
        let l = ligas[e];
        let total = core::ptr::read_unaligned((l + 0x3e) as *const i16) as i32;
        let mut s = total - *((l + 0xc1) as *const i8) as i32;
        while s < core::ptr::read_unaligned((l + 0x3e) as *const i16) as i32 {
            let c = cqv3_d(linha(l, s));
            core::ptr::write_unaligned((c + 0x60) as *mut usize, baixo[e]); // cqv3: o rebaixado vai à divisão de baixo
            crate::comparison::branch("cqv_3 rebaixado do estadual");
            cqv3_saiu(c, cqv3_d(l + 4));
            s += 1;
        }
    }
    liberar(lista);
}

/// O comparador do acesso: só a original (move clubes entre divisões, sorteia o embaralhamento e chama o técnico e os
/// contratos; não roda duas vezes), com o plano feito antes: os clubes de cada divisão de baixo na lista (até 200) e os
/// das últimas linhas de cada estadual. Depois: de cada divisão sobem exatamente min(vagas, os dela na lista) clubes
/// (+0x60 = o registro do estadual) e os outros dela ficam; os das últimas linhas estão na divisão de baixo.
unsafe extern "C" fn comparar_cqv3_acesso_dos_estaduais() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let ligas: [usize; 12] = core::array::from_fn(|i| cqv3_liga(CQV3_ESTADUAIS[i]));
    let baixo: [usize; 12] = core::array::from_fn(|i| cqv3_registro(CQV3_DE_BAIXO[i]));
    let mut deles: Vec<Vec<usize>> = vec![Vec::new(); 12];
    let mut n = 0;
    let mut i = 0usize;
    while (i as i32) < cqv3_d(0xae235c) as i32 && n < 0xc8 {
        let c = cqv3_clube(i);
        if let Some(e) = baixo.iter().position(|&b| b == cqv3_d(c + 0x60) && b != 0) {
            deles[e].push(c);
            n += 1;
        }
        i += 1;
    }
    let vagas: Vec<i32> = ligas.iter().map(|&l| *((l + 0xc1) as *const i8) as i32).collect();
    let registros: Vec<usize> = ligas.iter().map(|&l| cqv3_d(l + 4)).collect();
    let mut caem: Vec<(usize, usize)> = Vec::new();
    for e in 0..12 {
        let l = ligas[e];
        let t = core::ptr::read_unaligned((l + 0x3e) as *const i16) as i32;
        for s in (t - vagas[e]).max(0)..t {
            caem.push((cqv3_d(linha(l, s)), baixo[e]));
        }
    }
    o();
    let mut erros = Vec::new();
    for e in 0..12 {
        let subiram = deles[e].iter().filter(|&&c| cqv3_d(c + 0x60) == registros[e]).count();
        let ficaram = deles[e].iter().filter(|&&c| cqv3_d(c + 0x60) == baixo[e]).count();
        let esperado = (vagas[e].max(0) as usize).min(deles[e].len());
        if subiram != esperado || subiram + ficaram != deles[e].len() {
            erros.push(format!("divisão {}: subiram {} (esperado {}), ficaram {} de {}", e, subiram, esperado, ficaram, deles[e].len()));
        }
    }
    for &(c, b) in &caem {
        if cqv3_d(c + 0x60) != b {
            erros.push(format!("rebaixado {:#x} em {:#x} (esperado {:#x})", c, cqv3_d(c + 0x60), b));
        }
    }
    PLACAR.record(erros.is_empty(), || format!("cqv3_acesso_dos_estaduais: {}", erros.join("; ")));
}

/// A liga da Série C e o registro dela.
unsafe fn cqv3_serie_c() -> (usize, usize) {
    (cqv3_liga(0x9cf9ac), cqv3_registro(0x9cf9ac))
}

/// Os clubes brasileiros sem divisão nacional, na ordem do banco, até 160: clube (fora das seleções: os ids abaixo de
/// [0xae235c] − 2 × [0xae2350]) com nação (+0x53), sem divisão (+0x57 = 0) e com a nação do Brasil (o id da nação =
/// [0x9cf260]).
unsafe fn cqv3_sem_divisao() -> Vec<usize> {
    let mut v = Vec::new();
    let mut i = 0i32;
    while i < cqv3_d(0xae235c) as i32 - 2 * cqv3_d(0xae2350) as i32 && v.len() < 0xa0 {
        let c = cqv3_clube(i as usize);
        let nacao = cqv3_d(c + 0x53);
        if nacao != 0 && cqv3_d(c + 0x57) == 0 && cqv3_d(nacao) == cqv3_d(0x9cf260) {
            v.push(c);
        }
        i += 1;
    }
    v
}

/// { 0x42b520 } (cdecl, bra_nat_first.cpp, 576 bytes): `bra_nat_first_refresh_third_division_clubs()`, da virada do
/// futebol brasileiro: a Série C se renova.
/// 1) Uma lista de 160 (malloc 0x944e46 de 0x280; sem memória: a caixa da linha 0x43a e volta) com os clubes
///    brasileiros sem divisão nacional (cqv3_sem_divisao). Menos de 16: a lista é liberada e nada muda.
/// 2) A lista ordenada pela reputação e os 48 primeiros embaralhados (0x4afbd0, cdecl(lista, n, 16): sorteia); os 16
///    primeiros entram na Série C (+0x57 = o registro da Série C) e os que têm divisão regional (+0x60) e estão sem
///    técnico (+0xcf = 0) abrem a vaga de técnico (0x69ba50, thiscall([0xb67a44], clube, 0), ret 8).
/// 3) Na tabela da Série C e nas dos sete grupos (as fases 0 a 6), os clubes das linhas marcadas com +0x37 = 3 (os
///    eliminados na primeira fase [inferência]) saem da divisão nacional (+0x57 = 0) e sofrem a saída (cqv3_saiu, com
///    o registro da tabela de onde saem).
/// 4) A lista é liberada (free 0x9452ca).
unsafe extern "C" fn cqv3_renovar_serie_c() {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let vaga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x69ba50usize);
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let (t, reg) = cqv3_serie_c();
    let lista = alocar(0x280);
    if lista == 0 {
        return show_error_at(0x98aa30, 0x43a);
    }
    let v = cqv3_sem_divisao();
    for (k, &c) in v.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, c);
    }
    if v.len() < 0x10 {
        return liberar(lista);
    }
    embaralhar(lista, v.len() as u32, 0x10);
    for k in 0..0x10 { // cqv3: os 16 que entram na Série C
        let c = cqv3_d(lista + 4 * k);
        core::ptr::write_unaligned((c + 0x57) as *mut usize, reg);
        if cqv3_d(c + 0x60) != 0 && cqv3_d(c + 0xcf) == 0 {
            vaga(cqv3_d(0xb67a44), c, 0);
        }
    }
    crate::comparison::branch("cqv_3 serie c renovada");
    for g in -1i32..7 {
        let s = if g == -1 { t } else { cqv3_d(cqv3_d(t + 0xc) + 4 * g as usize) };
        let mut e = 0i32;
        while e < core::ptr::read_unaligned((s + 0x3e) as *const i16) as i32 {
            let r = linha(s, e);
            if *((r + 0x37) as *const u8) == 3 {
                let c = cqv3_d(r);
                core::ptr::write_unaligned((c + 0x57) as *mut usize, 0); // cqv3: o eliminado deixa a Série C
                cqv3_saiu(c, cqv3_d(s + 4));
            }
            e += 1;
        }
    }
    liberar(lista);
}

/// O comparador da renovação: só a original (sorteia o embaralhamento e chama o técnico e os contratos; não roda duas
/// vezes), com o plano feito antes: os clubes sem divisão (até 160) e os das linhas marcadas com 3 nas oito tabelas.
/// Depois, com 16 ou mais sem divisão: exatamente 16 deles estão na Série C e os outros sem divisão; os marcados, sem
/// divisão. Com menos de 16: nada mudou neles.
unsafe extern "C" fn comparar_cqv3_renovar_serie_c() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let (t, reg) = cqv3_serie_c();
    let v = cqv3_sem_divisao();
    let mut marcados = Vec::new();
    if v.len() >= 0x10 {
        for g in -1i32..7 {
            let s = if g == -1 { t } else { cqv3_d(cqv3_d(t + 0xc) + 4 * g as usize) };
            for e in 0..core::ptr::read_unaligned((s + 0x3e) as *const i16) as i32 {
                let r = linha(s, e);
                if *((r + 0x37) as *const u8) == 3 {
                    marcados.push(cqv3_d(r));
                }
            }
        }
    }
    o();
    let entraram = v.iter().filter(|&&c| cqv3_d(c + 0x57) == reg).count();
    let fora = v.iter().filter(|&&c| cqv3_d(c + 0x57) == 0).count();
    let esperado = if v.len() >= 0x10 { 0x10 } else { 0 };
    let saiu = marcados.iter().all(|&c| cqv3_d(c + 0x57) == 0);
    PLACAR.record(entraram == esperado && entraram + fora == v.len() && saiu,
                     || format!("cqv3_renovar_serie_c: {} sem divisão, entraram {} (esperado {}), ficaram {}, marcados {} todos fora: {}", v.len(), entraram, esperado, fora, marcados.len(), saiu));
}

/// { 0x6651f0 } (thiscall, ret 0, jap_j2.cpp, 397 bytes): `JapJ2::start_new_season()`, a virada da J2. +0x4c = 0 e a
/// tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o calendário (+0xba) liberado (free
/// 0x9452ca) e zerado; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4);
/// o ano (+0x40, word) + 1; então, cada um com a sua caixa (e devolvendo 0; a original devolve o eax da caixa, que
/// ninguém lê) se devolver 0: as regras (virtual +0x8c, thiscall, ret 0; linha 0x196), a tabela (JapJ2::create_table
/// 0x665030, thiscall, ret 0; 0x19d), os jogos (League::create_fixtures 0x6835c0; 0x1a3) e a ordem (League::sort_table
/// 0x6827d0, thiscall(this, 0), ret 4; 0x1aa); as posições previstas (virtual +0x5c), as expectativas das diretorias
/// (League::set_club_season_expectations 0x68aa80, thiscall, ret 0) e a renovação dos árbitros (OfficialsManager::
/// renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. A J2 não tem fases além da liga.
unsafe extern "thiscall" fn cqv3_virada_da_j2(this: usize) -> u32 {
    cqv3_virada_j2(this, 1) // cqv3: o ano seguinte da J2
}

unsafe fn cqv3_virada_j2(this: usize, anos: i16) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqv3_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqv3_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(anos));
    crate::comparison::branch("cqv_3 virada da j2");
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqv3_d(cqv3_d(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(0x9eaedc, 0x196);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x665030usize);
    if tabela(this) == 0 {
        show_error_at(0x9eaedc, 0x19d);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        show_error_at(0x9eaedc, 0x1a3);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        show_error_at(0x9eaedc, 0x1aa);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqv3_d(cqv3_d(this) + 0x5c));
    previstas(this);
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqv3_d(0xb74340), cqv3_d(this + 4)) // cqv3: a renovação dos árbitros da J2
}

/// O comparador da virada: só a original (libera a tabela e o calendário, aloca os novos, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1, o bloco de estatísticas igual e,
/// com as regras, a tabela e os jogos montados, a tabela nova (+0xb1) e +0x4c = 0.
unsafe extern "thiscall" fn comparar_cqv3_virada_da_j2(this: usize) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let bloco = cqv3_d(this + 8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let igual = depois == antes.wrapping_add(1) && cqv3_d(this + 8) == bloco && cqv3_d(this + 0xb1) != 0;
    PLACAR.record(igual, || format!("cqv3_virada_da_j2({:#x}) ano antes={} depois={} +8={:#x} +0xb1={:#x}", this, antes, depois, cqv3_d(this + 8), cqv3_d(this + 0xb1)));
    r
}
