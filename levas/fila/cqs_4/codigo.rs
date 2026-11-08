// ---------- %%LEVA%%a leva: a Série C: a final, a virada da temporada e as posições previstas (bra_nat_third.cpp) ----------

unsafe fn cqs4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da primeira linha da tabela (+0xb1) da fase `k` da liga (k = −1: a própria liga).
unsafe fn stage_leader(this: usize, k: i32) -> usize {
    let t = if k == -1 { this } else { cqs4_d(cqs4_d(this + 0xc) + 4 * k as usize) };
    cqs4_d(cqs4_d(t + 0xb1))
}

/// { 0x42e980 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_final_stage()`, a
/// final da Série C, do fim da terceira fase: os campeões dos dois grupos da terceira fase (o primeiro da tabela da
/// fase 8 e o da fase 9, nessa ordem) numa lista local de 2; a tabela das rodadas pela virtual +0x3c
/// (create_round_table: thiscall(this, 10, &rodadas, &extra, 0), ret 0x10); a fase (operator new 0x944cf1(0xb2) e o
/// construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano, 10, 1, o
/// extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][10]. Sem fase: a caixa da linha 0x392 e volta (sem
/// liberar a tabela). Com ela: a tabela liberada (free 0x9452ca) e +0x30 = 10. [inferência] A final decide o campeão
/// da Série C, em ida e volta.
unsafe extern "thiscall" fn bra_nat_third_create_final_stage(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let mut lista = [0usize; 2];
    lista[0] = stage_leader(this, 8); // cqs4: o campeão do grupo A
    lista[1] = stage_leader(this, 9);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs4_d(cqs4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 10, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista.as_ptr(), rodadas as u16 as u32, cqs4_d(this + 4), t, cqs4_w(this + 0x40) as u16 as u32, 10, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs4_d(this + 0xc) + 0x28) as *mut usize, fase);
    if cqs4_d(cqs4_d(this + 0xc) + 0x28) == 0 {
        return show_error_at(0x98aba4, 0x392);
    }
    crate::comparison::branch("cqs_4 final");
    soltar(t);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 10);
}

/// { 0x42def0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::start_new_season()`, a virada da Série C (chamada
/// pela virada da Série A): +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o
/// calendário (+0xba) liberado (free 0x9452ca) e zerado; com estatísticas (+8), Comp_Stats::archive_season_and_reset
/// (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0,
/// o destrutor com 1); +0x30 = −1; o vetor das fases liberado; o ano (+0x40, word) sobe um; um vetor novo de +0x2c
/// fases (malloc de 4 × +0x2c, sem zerar; com +0x2c ≤ 0, nulo); então, cada um com a sua caixa (linhas 0x202, 0x209,
/// 0x20f e 0x216) se devolver 0: as regras (virtual +0x8c), a tabela e a lista dos 56 (0x42dc90, thiscall, ret 0), os
/// jogos (League::create_fixtures 0x6835c0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret
/// 4); por fim as expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0), a
/// renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), os sete
/// grupos da primeira fase (0x42ee80(g), thiscall, ret 4, g de 0 a 6), as posições previstas (virtual +0x5c) e a lista
/// dos 56 (+0xee) liberada (se houver) e zerada. Com caixa, a cópia devolve 0 (a original devolve o eax da caixa, que
/// a virada do Brasil não lê); no fim, o eax do free ou o +0xee (a cópia devolve 0: ninguém lê).
unsafe extern "thiscall" fn bra_nat_third_start_new_season(this: usize) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqs4_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqs4_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqs4_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqs4_d(cqs4_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqs4_d(cqs4_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqs4_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    liberar(cqs4_d(this + 0xc));
    let n = cqs4_d(this + 0x2c) as i32;
    let a = cqs4_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1));
    crate::comparison::branch("cqs_4 virada da serie c");
    let v = if n > 0 { alocar((n * 4) as u32) } else { 0 };
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs4_d(cqs4_d(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(0x98aba4, 0x202);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x42dc90usize);
    if tabela(this) == 0 {
        show_error_at(0x98aba4, 0x209);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        show_error_at(0x98aba4, 0x20f);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        show_error_at(0x98aba4, 0x216);
        return 0;
    }
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqs4_d(0xb74340), cqs4_d(this + 4)); // cqs4: a renovação dos árbitros da Série C
    let grupo: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x42ee80usize);
    for g in 0..7 {
        grupo(this, g);
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs4_d(cqs4_d(this) + 0x5c));
    previstas(this);
    let l = cqs4_d(this + 0xee);
    if l != 0 {
        liberar(l);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut usize, 0);
    0
}

/// As posições previstas da Série C, na ordem da lista dos 56 depois de ordenada: 1, 2, 3, 3, 5, 5, 7, 7, oito vezes 9
/// (as faixas do mata-mata) e, para os outros quarenta, a faixa de oito em oito: 16, 24, 32, 40 e 48.
const SERIE_C_PREDICTED_PLACES: [u8; 16] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 9, 9, 9, 9, 9, 9, 9]; // cqs4: as posições previstas do mata-mata

fn place_for_index(k: usize) -> u8 {
    if k < 16 { SERIE_C_PREDICTED_PLACES[k] } else { (16 + 8 * ((k - 16) / 8)) as u8 }
}

/// { 0x42f0a0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::set_predicted_positions()`, as posições previstas da
/// Série C (do construtor e da virada, depois dos grupos). Sem estatísticas (+8): a caixa da linha 0x4a2 e volta. A
/// lista dos 56 (+0xee) é ordenada no lugar pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no
/// fim) e cada clube, na ordem, ganha a posição prevista de `place_for_index` (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a original empurra a faixa dos quarenta num dword com o byte da faixa e lixo
/// da pilha em cima; a 0x4a2540 só lê o byte: a cópia passa o byte limpo).
unsafe extern "thiscall" fn bra_nat_third_set_predicted_positions(this: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqs4_d(this + 8) == 0 {
        return show_error_at(0x98aba4, 0x4a2);
    }
    ordenar(cqs4_d(this + 0xee), 0x38, 4, 0x4b07d0); // cqs4: a lista dos 56 pela reputação
    for k in 0..0x38usize {
        posicao(cqs4_d(this + 8), cqs4_d(cqs4_d(this + 0xee) + 4 * k), place_for_index(k) as u32);
    }
    crate::comparison::branch("cqs_4 posicoes previstas");
}

/// O comparador da final: só a original (aloca a tabela e a fase e sorteia a rodada), com o plano feito antes: os
/// campeões dos grupos da terceira fase. Depois, a fase da vaga 10 existe, com +0x3e = 2, os dois na lista dela (como
/// conjunto) e +0x30 = 10.
unsafe extern "thiscall" fn comparar_bra_nat_third_create_final_stage(this: usize) {
    let (a, b) = (stage_leader(this, 8), stage_leader(this, 9));
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let fase = cqs4_d(cqs4_d(this + 0xc) + 0x28);
    let igual = fase != 0 && {
        let l = cqs4_d(fase + 0xa7);
        let (x, y) = (cqs4_d(l), cqs4_d(l + 6));
        cqs4_w(fase + 0x3e) == 2 && cqs4_d(this + 0x30) == 10 && ((x, y) == (a, b) || (x, y) == (b, a))
    };
    PLACAR.record(igual, || format!("bra_nat_third_create_final_stage({:#x}) campeoes={:#x},{:#x} fase={:#x}", this, a, b, fase));
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia jogos, árbitros e
/// grupos; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1, os sete grupos nas vagas 0 a 6 e a
/// lista dos 56 zerada.
unsafe extern "thiscall" fn comparar_bra_nat_third_start_new_season(this: usize) -> u32 {
    let antes = cqs4_w(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    let depois = cqs4_w(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && cqs4_d(this + 0xee) == 0 && (0..7).all(|g| cqs4_d(cqs4_d(this + 0xc) + 4 * g) != 0);
    PLACAR.record(igual, || format!("bra_nat_third_start_new_season({:#x}) ano antes={} depois={} +0xee={:#x}", this, antes, depois, cqs4_d(this + 0xee)));
    r
}

/// O comparador das posições previstas: só a original (a 0x4a2540 acrescenta o registro de um clube que não tem; não
/// pode rodar duas vezes), com o plano feito antes: a lista dos 56 ordenada pela mesma qsort numa cópia; depois, a
/// lista da liga na mesma ordem e a posição (+0x72 do registro achado por 0x4a4850) de cada clube.
unsafe extern "thiscall" fn comparar_bra_nat_third_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    if cqs4_d(this + 8) == 0 || cqs4_d(this + 0xee) == 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v = [0usize; 0x38];
    for (k, c) in v.iter_mut().enumerate() {
        *c = cqs4_d(cqs4_d(this + 0xee) + 4 * k);
    }
    ordenar(v.as_mut_ptr(), 0x38, 4, 0x4b07d0);
    o(this);
    let errado = (0..0x38).find(|&k| {
        let e = if v[k] != 0 { achar(cqs4_d(this + 8), v[k]) } else { 0 };
        cqs4_d(cqs4_d(this + 0xee) + 4 * k) != v[k] || e == 0 || *((e + 0x72) as *const u8) != place_for_index(k)
    });
    PLACAR.record(errado.is_none(), || format!("bra_nat_third_set_predicted_positions({:#x}): a vaga {:?} não bate", this, errado));
}
