// ---------- %%LEVA%%a leva: o olheiro da busca, a sequência atual do clube, construir o recordista e o técnico rival que comenta na mídia ----------

unsafe fn cph7_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x7ae950 } (thiscall, player_search.cpp): `PLAYER_SEARCH::get_scout`: o olheiro da busca: o índice em +0x180 (> −1)
/// no vetor de pessoas [0xae23bc] (110 bytes), se ainda confere com os nomes e a data guardados (0x53e990 com +0x174,
/// +0x178, +0x17c e a data em +0x184); senão 0.
unsafe extern "thiscall" fn get_scout(this: usize) -> usize {
    let i = cph7_d(this + 0x180) as i32;
    if i <= -1 {
        return 0;
    }
    let pessoa = (cph7_d(0xae23bc) as usize).wrapping_add((i as usize).wrapping_mul(110));
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    if confere(pessoa, cph7_d(this + 0x174), cph7_d(this + 0x178), cph7_d(this + 0x17c), this + 0x184) == 0 {
        return 0; // a vaga mudou de pessoa
    }
    pessoa
}

/// { 0x690c30 } (thiscall, ret 0xc, manager_manager.cpp): `MANAGER_MANAGER::get_current_run(clube, tipo, tamanho)`:
/// a sequência atual do clube na liga (+0x57; a competição pelo id em [0xadadfc], estatísticas em +8, a linha do clube
/// pela 0x4a4850): a primeira de mais de 1 jogo cujo último foi há menos de 10 dias (0x54cdb0 da data do jogo), na
/// ordem +0x19 (tipo 1, data +0x25), +0x2d (3, +0x39), +0x55 (4, +0x61) e +0x41 (2, +0x4d): o tipo e o tamanho (byte).
/// Nenhuma (ou sem liga, competição, estatísticas ou linha): tipo 5 e tamanho 0. O `this` não é usado.
unsafe extern "thiscall" fn get_current_run(_this: usize, clube: usize, tipo: *mut u8, tamanho: *mut u8) {
    let liga = cph7_d(clube + 0x57) as usize;
    let comp = if liga != 0 { cph7_d((cph7_d(0xadadfc) as usize).wrapping_add((cph7_d(liga) as usize).wrapping_mul(4))) as usize } else { 0 };
    let stats = if comp != 0 { cph7_d(comp + 8) as usize } else { 0 };
    if stats != 0 {
        let linha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let e = linha(stats, clube);
        if e != 0 {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            for (n, data, t) in [(0x19usize, 0x25usize, 1u8), (0x2d, 0x39, 3), (0x55, 0x61, 4), (0x41, 0x4d, 2)] {
                if cph7_d(e + n) as i32 > 1 && dias(0xae2c90, cph7_d(e + data), cph7_d(e + data + 4)) < 10 { // há menos de 10 dias
                    *tipo = t;
                    *tamanho = *((e + n) as *const u8);
                    return;
                }
            }
        }
    }
    *tipo = 5; // nenhuma
    *tamanho = 0;
}

/// { 0x450d40 } (thiscall, club_records.cpp): `record_holder_construct`: constrói o recordista: só a data em +0x10
/// (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn record_holder_construct(this: usize) -> usize {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(this + 0x10); // a data do recordista
    this
}

/// O técnico (+0xcf) de um clube rival passa nos sorteios (sorteio(+0x5c) = 0 e sorteio(+0x57) > 5)?
unsafe fn rival_manager_passes_draws(c: usize) -> bool {
    cph7_sortear(*((cph7_d(c + 0xcf) as usize + 0x5c) as *const i8) as i32) == 0
        && cph7_sortear(*((cph7_d(c + 0xcf) as usize + 0x57) as *const i8) as i32) > 5
}

/// { 0x72b940 } (thiscall, ret 4, media.cpp): `Media::pick_rival_manager_id(clube)`: com técnico (+0xcf) com contrato
/// (+0x69) e liga (+0x57; sem liga, só seleção, 0x53b900): o primeiro dos rejeitados do técnico (0x5480f0) com
/// registro de técnico e nota + 750 < nota (nunca: o sorteio(3) dali não acontece); depois cada rival (+0xb3, +0xb7,
/// +0xbb) com técnico que passa nos sorteios, da mesma liga e de reputação (+0x80) acima da do clube + 750, com
/// sorteio(3) ≠ 0; depois até 50 clubes sorteados (de [0xae235c] − 2 × [0xae2350]) que dão confronto na liga
/// (0x4b02e0, com o limite do sorteio como 3º argumento, resto da pilha do original) com técnico que passa nos
/// sorteios. Devolve o id do técnico, ou −1.
unsafe extern "thiscall" fn pick_rival_manager_id(_this: usize, clube: usize) -> u32 {
    if cph7_d(clube + 0xcf) == 0 {
        return u32::MAX;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if cph7_d(clube + 0x57) == 0 && selecao(clube) == 0 {
        return u32::MAX;
    }
    let tecnico = cph7_d(clube + 0xcf) as usize;
    if cph7_d(tecnico + 0x69) == 0 {
        return u32::MAX;
    }
    let mut rejeitados = [0usize; 20];
    let mut n = clube as u32; // a vaga do argumento: a 0x5480f0 zera o byte antes de contar
    let rejeitadas: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(0x5480f0usize);
    rejeitadas(tecnico, rejeitados.as_mut_ptr() as usize, &mut n as *mut u32 as usize);
    let mut k = 0i8;
    while k < n as u8 as i8 {
        let p = *rejeitados.as_ptr().add(k as u8 as usize); // como o original, sem conferir o tamanho (a 0x5480f0 dá até 3)
        if p != 0 && cph7_d(p + 0x69) != 0 {
            let nota = *((cph7_d(p + 0x69) as usize + 0xa) as *const i16) as i32;
            if nota.wrapping_add(0x2ee) < nota && cph7_sortear(3) != 0 {
                return cph7_d(p);
            }
        }
        k = k.wrapping_add(1);
    }
    for rival in [0xb3usize, 0xb7, 0xbb] {
        let r = cph7_d(clube + rival) as usize;
        if r == 0 || cph7_d(r + 0xcf) == 0 || !rival_manager_passes_draws(r) {
            continue;
        }
        if cph7_d(r + 0x57) != cph7_d(clube + 0x57) {
            continue;
        }
        let injury_word_0c = |c: usize| *((c + 0x80) as *const i16) as i32;
        if injury_word_0c(clube) + 0x2ee >= injury_word_0c(r) {
            continue; // o rival não é bem maior
        }
        if cph7_sortear(3) != 0 {
            return cph7_d(cph7_d(r + 0xcf) as usize);
        }
    }
    let confronto: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(0x4b02e0usize);
    let mut b = 0i8;
    while b < 0x32 {
        let limit = (cph7_d(0xae235c) as i32).wrapping_sub((cph7_d(0xae2350) as i32).wrapping_mul(2));
        let i = cph7_sortear(limit) as usize;
        let c = (cph7_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
        if confronto(clube, c, limit) != 0 && cph7_d(c + 0xcf) != 0 && rival_manager_passes_draws(c) {
            return cph7_d(cph7_d(c + 0xcf) as usize);
        }
        b += 1;
    }
    u32::MAX
}

// a 0x53e990 só lê: compara o valor
comparar_valor!(comparar_get_scout, get_scout, %%I0%%, () -> usize);

/// Comparador: a original grava nas saídas de verdade, a cópia em locais; a 0x4a4850 pode reordenar as estatísticas
/// (a original primeiro; a cópia acha a tabela já em ordem e a mesma linha).
unsafe extern "thiscall" fn comparar_get_current_run(this: usize, clube: usize, tipo: *mut u8, tamanho: *mut u8) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(%%I1%%)(this, clube, tipo, tamanho);
    let (mut t, mut n) = (0u8, 0u8);
    get_current_run(this, clube, &mut t, &mut n);
    PLACAR.registrar(*tipo == t && *tamanho == n, || format!("get_current_run({:x}) original={}/{} rust={}/{}", clube, *tipo, *tamanho, t, n));
}

unsafe extern "thiscall" fn comparar_record_holder_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x10, 8)], || o(this), || record_holder_construct(this));
    PLACAR.registrar(igual && r == n, || format!("record_holder_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio, a cópia em sem_contar (as escritas são só nas
/// locais da pilha de cada uma).
unsafe extern "thiscall" fn comparar_pick_rival_manager_id(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I3%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pick_rival_manager_id(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pick_rival_manager_id({:x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
