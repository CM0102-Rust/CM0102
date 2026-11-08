// ---------- %%LEVA%%a leva: o ranking da FIFA no jogo: o resultado no histórico da seleção, a faixa de força da seleção e as pontuações do ranking (fifa_rankings.cpp) ----------

unsafe fn cpt2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt2_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpt2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A entrada da nação `nacao` (id em [nacao]) na tabela do ranking [this] (0x2c bytes cada).
unsafe fn ranking_entry(this: usize, nacao: usize) -> usize {
    cpt2_d(this).wrapping_add((cpt2_d(nacao) as u32).wrapping_mul(0x2c) as usize)
}

/// O mês e o ano do jogo `partida`: a data (0x549ef0) do dia do ano (+0x2a) e do ano (+0x28), o mês dela (0x54cfb0).
unsafe fn match_month_year(partida: usize) -> (u8, u16) {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let mut d = [0u8; 8];
    let p = data_nova(d.as_mut_ptr(), cpt2_w(partida + 0x2a) as u32, cpt2_w(partida + 0x28) as u32);
    (mes_de(p), cpt2_w(partida + 0x28))
}

/// { 0x58dea0 } (thiscall, ret 0xc, fifa_rankings.cpp, SEH só da data local, de destrutor vazio):
/// `FIFA_RANKINGS::append_result(nação, pontos, partida)` (pelo resultado internacional 0x58cf70, para cada seleção):
/// com a conta (+0xa) igual à capacidade (+8), a capacidade cresce 5 e o histórico (+4) é realocado (0x945667) ou
/// alocado (0x944e46) com capacidade × 5 bytes. A entrada nova, no lugar da conta antiga: os pontos (16 bits), o mês do
/// jogo (+2) e o ano (+3).
unsafe extern "thiscall" fn append_result(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = ranking_entry(this, nacao);
    if cpt2_w(e + 0xa) == cpt2_w(e + 8) {
        *((e + 8) as *mut u16) = cpt2_w(e + 8).wrapping_add(5);
        let h = cpt2_d(e + 4);
        let n = cpt2_h(e + 8) as i32 * 5;
        let novo = if h != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(h, n as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(n as u32)
        };
        *((e + 4) as *mut usize) = novo;
    }
    let c = cpt2_h(e + 0xa);
    *((e + 0xa) as *mut i16) = c.wrapping_add(1); // cpt2: mais um resultado
    let p = cpt2_d(e + 4).wrapping_add(c as isize as usize * 5);
    *(p as *mut u16) = pontos as u16;
    let (mes, ano) = match_month_year(partida);
    *((p + 2) as *mut u8) = mes; // cpt2: o mes do jogo
    core::ptr::write_unaligned((p + 3) as *mut u16, ano); // cpt2: o ano do jogo
    crate::comparison::branch("cpt_2 resultado");
}

// ponytail: a original pode realocar o histórico (não roda duas vezes): o plano antes (conta, capacidade, pontos, mês e
// ano), a original roda e a entrada nova é conferida; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_append_result(this: usize, nacao: usize, pontos: u32, partida: usize) {
    let e = ranking_entry(this, nacao);
    let (conta, cap) = (cpt2_h(e + 0xa), cpt2_h(e + 8));
    let (mes, ano) = match_month_year(partida);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%)(this, nacao, pontos, partida);
    let p = cpt2_d(e + 4).wrapping_add(conta as isize as usize * 5);
    let cap_n = if conta == cap { cap.wrapping_add(5) } else { cap };
    let lido = (cpt2_h(e + 0xa), cpt2_h(e + 8), cpt2_w(p), *((p + 2) as *const u8), cpt2_w(p + 3));
    let plano = (conta.wrapping_add(1), cap_n, pontos as u16, mes, ano);
    PLACAR.record(lido == plano, || format!("append_result(nação {}, pontos {}) original={:?} plano={:?}", cpt2_d(nacao), pontos as u16, lido, plano));
}

/// { 0x58e190 } (thiscall, ret 4, fifa_rankings.cpp): `FIFA_RANKINGS::get_nation_strength_tier(nação)` (duas vezes por
/// resultado internacional 0x58cf70): reputação (+0x8e) abaixo de 3000 e posição (+0x10 da entrada) abaixo de 50: 3;
/// reputação abaixo de 6000 e posição abaixo de 15: 2; senão pela posição: abaixo de 21, 1; de 61, 2; de 101, 3; 4.
unsafe extern "thiscall" fn get_nation_strength_tier(this: usize, nacao: usize) -> u8 {
    let rep = cpt2_h(nacao + 0x8e);
    let pos = || cpt2_h(ranking_entry(this, nacao) + 0x10);
    if rep < 0xbb8 && pos() < 0x32 {
        return 3;
    }
    if rep < 0x1770 && pos() < 0xf {
        return 2;
    }
    let p = pos();
    if p < 0x15 {
        1
    } else if p < 0x3d {
        2
    } else if p >= 0x65 {
        4
    } else {
        3
    }
}

unsafe extern "thiscall" fn comparar_get_nation_strength_tier(this: usize, nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, nacao);
    let n = get_nation_strength_tier(this, nacao);
    PLACAR.record(r == n, || format!("get_nation_strength_tier(nação {}) original={} rust={}", cpt2_d(nacao), r, n));
    r
}

/// { 0x58dfc0 } (thiscall, fifa_rankings.cpp): `FIFA_RANKINGS::recalculate_scores()` (pela atualização do dia 18
/// 0x58d460): em cada nação, a soma dos pontos dos oito primeiros resultados (o histórico vem ordenado) e a de todos;
/// com mais de oito, o total do ano (+0x14) é (todos × 8 / conta + oito) / 2, senão os oito. Se a data de [0xdd6ec8]
/// passa da de hoje menos 365 dias (0x54c8e0), com menos de nove resultados o total é (+0x18 + oito) / (conta + 8) × 8
/// (sem resultados, o +0x18). A pontuação (+0xc, f32): (+0x14 + 5/6 de +0x18 + 4/6 de +0x1c + 3/6 de +0x20 + 2/6 de
/// +0x24 + 1/6 de +0x28, cada parte truncada) × [0x96a350].
unsafe extern "thiscall" fn recalculate_scores(this: usize) {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let fator = core::ptr::read_unaligned(0x96a350 as *const f64);
    let mut i = 0i32;
    if cpt2_d(0xae2350) as i32 <= 0 {
        return;
    }
    loop {
        let e = cpt2_d(this) + i as usize * 0x2c;
        let conta = cpt2_h(e + 0xa);
        let ponto = |k: i8| cpt2_h(cpt2_d(e + 4).wrapping_add(k as isize as usize * 5)) as i32;
        let mut oito = 0i32;
        let mut k = 0i8;
        if conta > 0 {
            loop {
                if k >= 8 {
                    break;
                }
                oito = oito.wrapping_add(ponto(k));
                k = k.wrapping_add(1);
                if k as i16 >= conta {
                    break;
                }
            }
        }
        let mut todos = oito;
        while (k as i16) < conta {
            todos = todos.wrapping_add(ponto(k));
            k = k.wrapping_add(1);
        }
        let ano = if conta > 8 { (todos.wrapping_mul(8) / conta as i32 + oito) / 2 } else { oito };
        *((e + 0x14) as *mut i32) = ano;
        let base = core::ptr::read_unaligned(0xdd6ec8 as *const [u8; 8]);
        let mut d = [0u8; 8];
        let r = datar(0xae2c90, &mut d, 0x16d);
        let (by, ry) = (u16::from_le_bytes([base[2], base[3]]), cpt2_w(r + 2));
        let depois = by > ry || (by == ry && i16::from_le_bytes([base[0], base[1]]) > cpt2_h(r));
        if depois && cpt2_h(e + 0xa) < 9 {
            let c = cpt2_h(e + 0xa);
            let v18 = *((e + 0x18) as *const i32);
            *((e + 0x14) as *mut i32) = if c != 0 { (v18.wrapping_add(oito) / (c as i32 + 8)).wrapping_shl(3) } else { v18 };
        }
        let t = |o: usize| *((e + o) as *const i32);
        let soma = (t(0x24).wrapping_mul(2) / 6)
            .wrapping_add(t(0x20).wrapping_mul(3) / 6)
            .wrapping_add(t(0x1c).wrapping_mul(4) / 6)
            .wrapping_add(t(0x18).wrapping_mul(5) / 6)
            .wrapping_add(t(0x28) / 6)
            .wrapping_add(t(0x14));
        *((e + 0xc) as *mut f32) = (soma as f64 * fator) as f32;
        i += 1;
        if i >= cpt2_d(0xae2350) as i32 {
            break;
        }
    }
}

unsafe extern "thiscall" fn comparar_recalculate_scores(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let reg = [(cpt2_d(this), cpt2_d(0xae2350) * 0x2c)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || recalculate_scores(this));
    PLACAR.record(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("recalculate_scores diferenças nos deslocamentos {:x?}", dif)
    });
}
