// ---------- %%LEVA%%a leva: as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional (national_teams.cpp, tactics.cpp, media.cpp) ----------

/// As 50 vagas de convocação (dwords desalinhados em +0xd7) das duas seleções da nação: a principal
/// (0x53b3d0) e a B (0x53b4f0), nessa ordem.
unsafe fn national_squad_slots(nacao: usize) -> [usize; 2] {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    [principal(nacao).wrapping_add(0xd7), b(nacao).wrapping_add(0xd7)]
}

/// { 0x76c060 } (stdcall, ret 4, national_teams.cpp): apaga as convocações repetidas das duas
/// seleções da nação: em cada uma, todo id não nulo zera as vagas seguintes com o mesmo id.
unsafe extern "stdcall" fn national_teams_remove_duplicate_squad_entries(nacao: usize) {
    for vagas in national_squad_slots(nacao) {
        let vaga = |i: usize| (vagas + 4 * i) as *mut u32;
        for i in 0..50 {
            let v = core::ptr::read_unaligned(vaga(i));
            if v == 0 {
                continue;
            }
            for j in i + 1..50 {
                if core::ptr::read_unaligned(vaga(j)) == v {
                    core::ptr::write_unaligned(vaga(j), 0);
                }
            }
        }
    }
}

/// { 0x8972c0 } (thiscall, ret 0xc, tactics.cpp): a vaga (0..19) da escalação do clube em que está a
/// pessoa: linha tática do clube (0x8982b0 com `time`; -1 devolve 0) e as 20 vagas (dwords em +0x601 +
/// linha x 0x91 + 2) comparadas com o id; não está: 0xff. Clube ou pessoa nulos: a original (caixa
/// de erro).
unsafe extern "thiscall" fn find_selection_slot(tatica: usize, clube: usize, pessoa: *const i32, time: u32) -> u8 {
    if clube == 0 || pessoa.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(%%I1%%)(tatica, clube, pessoa, time);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, time);
    if k == -1 {
        return 0;
    }
    let id = *pessoa;
    let vagas = core::ptr::read_unaligned((tatica + 0x601) as *const usize).wrapping_add((k as usize).wrapping_mul(0x91) + 2);
    match (0..20).find(|&i| core::ptr::read_unaligned((vagas + 4 * i) as *const i32) == id) {
        Some(i) => i as u8,
        None => 0xff,
    }
}

/// { 0x72b810 } (thiscall, ret 8, media.cpp; `this` não é lido): sorteia o comentarista
/// ex-internacional para o `clube` (ou para falar de `assunto`). 150 rodadas; em cada uma sorteia
/// pessoas (0x90cfc0 com [0xae2364], 110 bytes em [0xae23bc]) até achar uma com idade (+0x18) ≥ 37,
/// jogos pela seleção (+0x22) ≥ 50, ficha (+0x69), sem clube (+0x39) se tiver contrato (+0x61) e que
/// não seja o técnico do clube (+0xcf); a 100ª tentativa encerra a rodada. O peso é um sorteio de
/// reputação da nação (+0x1a, palavra +0x8e) × jogos, dobrado se for da nação do assunto (sem
/// assunto: tem de ser da nação do clube, +0x53). De outra nação: a reputação tem de vencer 6500 +
/// sorteio de 100, a ficha +0xc ≥ 6000 e o assunto não pode desgostar dele (0x547840). Fica o de
/// maior peso (o primeiro aceito sempre entra). Sem clube e sem assunto: 0.
unsafe extern "thiscall" fn pick_former_international_pundit(_this: usize, clube: usize, assunto: usize) -> usize {
    if assunto == 0 && clube == 0 {
        return 0;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let injury_word_0c = |nacao: usize| core::ptr::read_unaligned((nacao + 0x8e) as *const i16) as i32;
    let (mut melhor, mut maior) = (0usize, 0i32);
    for _ in 0..150 {
        let mut tentativas = 0;
        let p = loop {
            let r = sorteio(*(0xae2364 as *const i32));
            tentativas += 1;
            if tentativas >= 100 {
                break 0;
            }
            let p = (*(0xae23bc as *const usize)).wrapping_add((r as usize).wrapping_mul(110));
            if *((p + 0x18) as *const i8) < 0x25 || *((p + 0x22) as *const u8) < 0x32 || u(p + 0x69) == 0 {
                continue;
            }
            if u(p + 0x61) != 0 && u(p + 0x39) != 0 {
                continue;
            }
            if u(clube + 0xcf) == p {
                continue;
            }
            break p;
        };
        if p == 0 {
            continue;
        }
        let nacao = u(p + 0x1a);
        let mut peso = sorteio(injury_word_0c(nacao).wrapping_mul(*((p + 0x22) as *const u8) as i32));
        let mesma_nacao = if assunto == 0 {
            if nacao != u(clube + 0x53) {
                continue;
            }
            true
        } else {
            nacao == u(assunto + 0x1a)
        };
        if mesma_nacao {
            peso = peso.wrapping_add(peso);
        } else {
            let corte = sorteio(100).wrapping_add(0x1964);
            if injury_word_0c(nacao) < corte
                || core::ptr::read_unaligned((u(p + 0x69) + 0xc) as *const i16) < 0x1770
                || nao_gosta(assunto, p) != 0
            {
                continue;
            }
        }
        if melhor == 0 || peso > maior {
            maior = peso;
            melhor = p;
        }
    }
    melhor
}

unsafe extern "stdcall" fn comparar_national_teams_remove_duplicate_squad_entries(nacao: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(%%I0%%);
    if nacao == 0 {
        return o(nacao);
    }
    let [a, b] = national_squad_slots(nacao);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 200), (b, 200)], || o(nacao), || national_teams_remove_duplicate_squad_entries(nacao));
    PLACAR.registrar(igual, || format!("national_teams_remove_duplicate_squad_entries {:x?} x {:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_find_selection_slot(tatica: usize, clube: usize, pessoa: *const i32, time: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(%%I1%%)(tatica, clube, pessoa, time);
    if clube == 0 || pessoa.is_null() {
        return r;
    }
    let n = find_selection_slot(tatica, clube, pessoa, time);
    PLACAR.registrar(r == n, || format!("find_selection_slot(time {}) original={} rust={}", time, r, n));
    r
}

unsafe extern "thiscall" fn comparar_pick_former_international_pundit(this: usize, clube: usize, assunto: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I2%%);
    if assunto == 0 && clube == 0 {
        return o(this, clube, assunto);
    }
    // o comentarista sai só do sorteio
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube, assunto),
        || crate::aleatorio::sem_contar(|| pick_former_international_pundit(this, clube, assunto)));
    PLACAR.registrar(igual && r == n, || format!("pick_former_international_pundit {:#x} x {:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}
