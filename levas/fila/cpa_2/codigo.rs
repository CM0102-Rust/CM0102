// ---------- %%LEVA%%a leva: o fim de temporada da liga: os direitos de TV, as honras da temporada e o registro no histórico de competições (league.cpp, comp_util.cpp) ----------

unsafe fn cpa2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpa2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O pagamento da 0x683010 (`noticia` falso: sem a notícia da 0x48ea40, para o comparador).
unsafe fn pay_tv_money(liga: usize, valor: i32, filtro: usize, noticia: bool) {
    let financas = *(0xae2a38 as *const usize);
    if financas == 0 || cpa2_w(liga + 0x3e) <= 0 {
        return;
    }
    crate::comparacao::ramo("liga direitos de tv");
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let somar_tv: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5987c0usize);
    let mut i: i16 = 0;
    loop {
        let linha = cpa2_d(liga + 0xb1) + 0x3b * i as i32 as usize;
        let clube = cpa2_d(linha);
        if filtro == 0 || cpa2_d(clube + 0x5b) == filtro {
            somar_tv(get_club_finance(*(0xae2a38 as *const usize), clube), valor); // o dinheiro da TV
            if noticia {
                let avisar: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x48ea40usize);
                avisar(liga, cpa2_d(cpa2_d(liga + 0xb1) + 0x3b * i as i32 as usize), valor);
            }
        }
        i = i.wrapping_add(1);
        if i >= cpa2_w(liga + 0x3e) {
            break;
        }
    }
}

/// { 0x683010 } (thiscall, league.cpp): `League::pay_tv_rights(valor, filtro)`: com a tabela de finanças
/// [0xae2a38], a cada clube da tabela (+0xb1; com `filtro`, só os de +0x5b igual a ele) soma `valor` à receita de TV
/// (0x5a0590 → 0x5987c0) e manda a notícia (Comp::send_tv_rights_news 0x48ea40, só para clube de técnico humano).
/// Sem SEH, sem sorteio, sem locais.
unsafe extern "thiscall" fn pay_tv_rights(liga: usize, valor: i32, filtro: usize) {
    pay_tv_money(liga, valor, filtro, true)
}

/// Comparador: as finanças (0x167 bytes) de cada clube da tabela; a cópia roda sem a notícia (a da original fica, uma
/// vez só). Clube nulo na tabela (a caixa da 0x5a0590): só a original.
unsafe extern "thiscall" fn comparar_pay_tv_rights(liga: usize, valor: i32, filtro: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(%%I0%%);
    let financas = *(0xae2a38 as *const usize);
    let n = cpa2_w(liga + 0x3e).max(0) as usize;
    let clubes: Vec<usize> = (0..n).map(|i| cpa2_d(cpa2_d(liga + 0xb1) + 0x3b * i)).collect();
    if financas == 0 || clubes.contains(&0) {
        return o(liga, valor, filtro);
    }
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let reg: Vec<(usize, usize)> = clubes.iter().map(|&c| (get_club_finance(financas, c), 0x167)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, valor, filtro), || pay_tv_money(liga, valor, filtro, false));
    PLACAR.registrar(igual, || format!("pay_tv_rights({:x}, {}, {:x}) {:x?} x {:x?}", liga, valor, filtro, mo, mr));
}

/// { 0x684640 } (thiscall, league.cpp): `League::record_season_honours()`: campeão, vice e (com mais de 2 clubes)
/// terceiro da tabela (+0xb1). Se a divisão do campeão (+0x57) é esta competição (+4), zera nos clubes ([0xae23b4],
/// 0x245 bytes, [0xae235c] − [0xae2350]) o +0x5b que aponta para ela. Se a virtual +0x88 da liga da competição
/// ([0xadadfc][id]) pede, o destaque é a seleção (0x53b3d0) do país-sede (HostCountry::find_hosts 0x5fa730 com o ano
/// +0x40): com duas sedes, a de maior +0x8e (empate: a primeira). Grava no histórico (0x4afce0) e devolve o que ela
/// devolve. Sem SEH, sem sorteio; as duas locais das sedes só são lidas quando a 0x5fa730 as grava.
unsafe extern "thiscall" fn league_record_season_honours(liga: usize) -> usize {
    crate::comparacao::ramo("liga honras da temporada");
    let tab = cpa2_d(liga + 0xb1);
    let vice = cpa2_d(tab + 0x3b);
    let campeao = cpa2_d(tab);
    let terceiro = if cpa2_w(liga + 0x3e) > 2 { cpa2_d(tab + 0x76) } else { 0 };
    if cpa2_d(campeao + 0x57) == cpa2_d(liga + 4) {
        let quantos = || (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32));
        let mut k = 0i32;
        while k < quantos() {
            let p = (*(0xae23b4 as *const usize)).wrapping_add(0x245 * k as usize + 0x5b);
            if cpa2_d(p) == cpa2_d(liga + 4) {
                core::ptr::write_unaligned(p as *mut usize, 0); // a divisão anterior
            }
            k += 1;
        }
    }
    let comp = cpa2_d(liga + 4);
    let alvo = *((*(0xadadfc as *const usize)).wrapping_add(4 * *(comp as *const u32) as usize) as *const usize);
    let pede: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpa2_d(cpa2_d(alvo) + 0x88));
    let mut destaque = 0usize;
    if pede(alvo) != 0 {
        let sedes: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let (mut a, mut b) = (0i32, 0i32);
        let q = sedes(*(0xb5e134 as *const usize), *(cpa2_d(liga + 4) as *const u32), cpa2_w(liga + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let nacao = |k: i32| (*(0xae23a8 as *const usize)).wrapping_add(k.wrapping_mul(0x122) as usize);
        if q == 2 {
            destaque = if cpa2_w(nacao(a) + 0x8e) < cpa2_w(nacao(b) + 0x8e) { selecao(nacao(b)) } else { selecao(nacao(a)) };
        } else if q != 0 {
            destaque = selecao(nacao(a));
        }
    }
    let registrar: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4afce0usize);
    registrar(liga, campeao, vice, terceiro, destaque) // campeão, vice, terceiro
}

/// As regiões do histórico de competições: contagem, ponteiro e as linhas (26 bytes).
unsafe fn comp_history_regions() -> Vec<(usize, usize)> {
    let n = (*(0xae237c as *const i32)).max(0) as usize;
    vec![(0xae237c, 4), (0xae23d4, 4), (*(0xae23d4 as *const usize), 26 * n)]
}

/// Comparador: os clubes (o +0x5b) e o histórico de competições. Sem linha livre no histórico (a 0x4afce0 faria
/// realloc): só a original.
unsafe extern "thiscall" fn comparar_league_record_season_honours(liga: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    if free_comp_history_row() == 0 {
        return o(liga);
    }
    let mut reg = comp_history_regions();
    let quantos = (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32)).max(0) as usize;
    reg.push((*(0xae23b4 as *const usize), 0x245 * quantos));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga), || league_record_season_honours(liga));
    PLACAR.registrar(igual && r == n, || format!("league_record_season_honours({:x}) {:x}/{:x} {:x?} x {:x?}", liga, r, n, mo, mr));
    r
}

/// A linha livre do histórico de competições ([0xae23d4], [0xae237c] linhas de 26 bytes): a seguinte à última com
/// +4 (competição) preenchido; nenhuma preenchida, ou a última preenchida: 0 (falta crescer).
unsafe fn free_comp_history_row() -> usize {
    let n = *(0xae237c as *const i32);
    let t = *(0xae23d4 as *const usize);
    let mut i = n - 1;
    while i >= 0 {
        if cpa2_d(t.wrapping_add(26 * i as usize + 4)) != 0 {
            break;
        }
        i -= 1;
    }
    if i < 0 || i >= n - 1 {
        return 0;
    }
    t.wrapping_add(26 * (i + 1) as usize)
}

/// { 0x4afce0 } (cdecl, comp_util.cpp): `comp_util_add_history_record(liga, primeiro, segundo, terceiro, extra)`:
/// a linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60 e usa a primeira nova; sem
/// memória, 0) recebe a competição (+4 da liga), o ano [0xae2c92] (+8), e os quatro valores em +0xa, +0xe, +0x12,
/// +0x16. Devolve a linha. Sem SEH, sem sorteio, sem locais.
unsafe extern "C" fn comp_util_add_history_record(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let mut r = free_comp_history_row();
    if r == 0 {
        crate::comparacao::ramo("histórico de competições crescer");
        let crescer: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x527f60usize);
        if crescer(0x14) == 0 {
            return 0;
        }
        let n = *(0xae237c as *const i32);
        r = (*(0xae23d4 as *const usize)).wrapping_add(26usize.wrapping_mul(n.wrapping_sub(20) as usize));
    }
    core::ptr::write_unaligned((r + 4) as *mut usize, cpa2_d(liga + 4));
    core::ptr::write_unaligned((r + 8) as *mut u16, *(0xae2c92 as *const u16)); // o ano
    for (k, v) in [a, b, c, d].into_iter().enumerate() {
        core::ptr::write_unaligned((r + 0xa + 4 * k) as *mut u32, v);
    }
    r
}

unsafe extern "C" fn comparar_comp_util_add_history_record(liga: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> usize>(%%I2%%);
    if free_comp_history_row() == 0 {
        return o(liga, a, b, c, d); // cresce (realloc): só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&comp_history_regions(), || o(liga, a, b, c, d), || comp_util_add_history_record(liga, a, b, c, d));
    PLACAR.registrar(igual && r == n, || format!("comp_util_add_history_record({:x}, {:x}, {:x}, {:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", liga, a, b, c, d, r, n, mo, mr));
    r
}
