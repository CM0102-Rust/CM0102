// ---------- 259a leva: as notas do mês: as notas da partida, a nota do jogador no mês e o melhor do mês (month_ratings.cpp) ----------

unsafe fn cpw5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw5_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A pessoa `id` da tabela de pessoas ([0xae23bc], 0x6e bytes cada).
unsafe fn cpw5_pessoa(id: i32) -> usize {
    cpw5_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e))
}

/// { 0x767ee0 } (thiscall, ret 8, month_ratings.cpp): `MONTH_RATINGS::record_match_ratings(jogo, partida)`, pelo
/// 0x7683a0 a cada partida das ligas com prêmio do mês. Os dois lados da partida (+0x190 e +0x190 + 0x625), 20 vagas de
/// 0x49 bytes cada: fora a vaga marcada 0xfb em +0x1d e, da 12ª em diante, a reserva que não entrou (+0x32 = 0xff), a
/// pessoa (id em +0) abaixo da conta de pessoas [0xae2364] vai para as notas do mês (0x767fa0).
unsafe extern "thiscall" fn guardar_notas_da_partida(this: usize, jogo: usize, partida: usize) {
    let _ = jogo;
    let nota: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x767fa0usize);
    let mut lado = partida.wrapping_add(0x190);
    for l in 0..2 { // cpw5: os dois lados
        for k in 0..0x14usize {
            let e = lado + k * 0x49;
            if cpw5_b(e + 0x1d) == 0xfb {
                continue;
            }
            if k >= 0xb && cpw5_b(e + 0x32) == 0xff {
                continue;
            }
            let p = cpw5_pessoa(cpw5_i(e));
            if p != 0 && cpw5_i(p) >= cpw5_i(0xae2364) {
                continue;
            }
            if l == 1 {
                crate::comparacao::ramo("cpw_5 notas do visitante");
            }
            nota(this, e);
        }
        lado += 0x625;
    }
}

/// O comparador das notas da partida: lado a lado (cabeçalho e as vagas da tabela que podem mudar) quando cabem as 40
/// vagas sem crescer a tabela; senão só a original (a 0x767fa0 realoca e a original não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_guardar_notas_da_partida(this: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1284);
    let (tab, n, cap) = (cpw5_d(this), cpw5_i(this + 4), cpw5_i(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 40 {
        // ponytail: a tabela pode crescer (realloc): só a original; a 0x767fa0 compara cada nota
        return o(this, jogo, partida);
    }
    let regioes = [(this, 12), (tab, (n as usize + 40) * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, partida), || guardar_notas_da_partida(this, jogo, partida));
    PLACAR.registrar(igual, || format!("guardar_notas_da_partida(this {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// A nota da partida (+0x35 da vaga, sem sinal) soma na palavra +8 do registro de 12 bytes e o byte +0xa conta mais um
/// jogo.
unsafe fn cpw5_somar(r: usize, e: usize) {
    let s = (r + 8) as *mut u16;
    core::ptr::write_unaligned(s, core::ptr::read_unaligned(s).wrapping_add(cpw5_b(e + 0x35) as u16));
    *((r + 0xa) as *mut u8) = cpw5_b(r + 0xa).wrapping_add(1);
}

/// { 0x767fa0 } (thiscall, ret 4, month_ratings.cpp): `MONTH_RATINGS::add_player_rating(vaga)`. A tabela em [this]
/// (conta em +4, capacidade em +8) tem registros de 12 bytes: +0 pessoa, +4 liga do clube (ou −1), +8 soma das notas,
/// +0xa jogos, +0xb idade. Acha a pessoa da vaga (+0) ou acrescenta (cresce 300 com malloc 0x944e46 ou realloc
/// 0x945667 quando cheia; a liga pelo clube +0x39 da pessoa e o +0x57 dele; a idade pelo +0x18 da pessoa) e soma a nota.
unsafe extern "thiscall" fn nota_do_jogador_no_mes(this: usize, e: usize) {
    let id = cpw5_i(e);
    let n = cpw5_i(this + 4);
    let mut tab = cpw5_d(this);
    for k in 0..n.max(0) as usize {
        let r = tab + k * 12;
        if cpw5_i(r) == id {
            return cpw5_somar(r, e);
        }
    }
    let cap = cpw5_i(this + 8);
    if n == cap {
        let novo = cap.wrapping_add(300);
        cpw5_pd(this + 8, novo as u32);
        let bytes = (novo as u32).wrapping_mul(12);
        tab = if tab == 0 {
            let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            malloc(bytes)
        } else {
            let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realloc(tab, bytes)
        };
        if tab == 0 {
            // ponytail: sem memória (a caixa "Error" da original): volta a capacidade e deixa a original refazer
            cpw5_pd(this + 8, cap as u32);
            return orig::<unsafe extern "thiscall" fn(usize, usize)>(1285)(this, e);
        }
        cpw5_pd(this, tab as u32);
    }
    let p = cpw5_pessoa(id);
    let clube = cpw5_d(p + 0x39);
    let mut liga = -1i32;
    if clube != 0 {
        let l = cpw5_d(clube + 0x57);
        if l != 0 {
            liga = cpw5_i(l);
        }
    }
    let r = tab + (n as usize) * 12;
    cpw5_pd(r, id as u32);
    cpw5_pd(r + 4, liga as u32);
    core::ptr::write_unaligned((r + 8) as *mut u16, 0);
    *((r + 0xb) as *mut u8) = cpw5_b(p + 0x18);
    crate::comparacao::ramo("cpw_5 jogador novo no mes");
    *((r + 0xa) as *mut u8) = 0; // cpw5: começa sem jogos
    cpw5_pd(this + 4, n.wrapping_add(1) as u32);
    cpw5_somar(r, e);
}

/// O comparador da nota do mês: lado a lado (cabeçalho e os registros até o novo) quando a tabela não precisa crescer;
/// senão só a original (realloc).
unsafe extern "thiscall" fn comparar_nota_do_jogador_no_mes(this: usize, e: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1285);
    let (tab, n, cap) = (cpw5_d(this), cpw5_i(this + 4), cpw5_i(this + 8));
    let achou = tab != 0 && (0..n.max(0) as usize).any(|k| cpw5_i(tab + k * 12) == cpw5_i(e));
    if tab == 0 || n < 0 || (n >= cap && !achou) {
        // ponytail: a tabela cresce (malloc/realloc): só a original
        return o(this, e);
    }
    let vagas = if n < cap { n as usize + 1 } else { n as usize };
    let regioes = [(this, 12), (tab, vagas * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, e), || nota_do_jogador_no_mes(this, e));
    PLACAR.registrar(igual, || format!("nota_do_jogador_no_mes(pessoa {}) original={:x?} rust={:x?}", cpw5_i(e), mo, mr));
}

/// { 0x768130 } (thiscall, ret 8, month_ratings.cpp): `MONTH_RATINGS::find_best_player(liga, idade)`, pelo prêmio do
/// mês (0x767150). Ordena a tabela pela média (qsort 0x945333 com 0x768200: quem tem menos de 3 jogos vai para o fim) e
/// devolve o primeiro registro da liga (−1: qualquer) com pelo menos 3 jogos (depois 2, 1 e 0); com idade (não 0xff),
/// os acima dela só contam e o mais novo só vale depois de 3 contados; com liga, a liga atual do clube da pessoa (+0x39,
/// +0x57) também tem de ser a pedida. Nenhum: 0.
unsafe extern "thiscall" fn melhor_do_mes(this: usize, liga: i32, idade: u32) -> usize {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparacao::ramo("cpw_5 ordenou as notas");
    ordenar(cpw5_d(this), cpw5_d(this + 4) as u32, 0xc, 0x768200); // cpw5: ordena pela média
    let tab = cpw5_d(this);
    let n = cpw5_i(this + 4);
    let idade = idade as u8;
    let mut contados = 0i32;
    let mut minimo = 3i8;
    loop {
        for k in 0..n.max(0) as usize {
            let r = tab + k * 12;
            if liga != -1 && cpw5_i(r + 4) != liga {
                continue;
            }
            if (cpw5_b(r + 0xa) as i8) < minimo {
                continue;
            }
            if idade != 0xff {
                if cpw5_b(r + 0xb) as i8 > idade as i8 {
                    contados += 1;
                    continue;
                }
                if contados < 3 {
                    continue;
                }
            }
            if liga == -1 {
                return r;
            }
            let c = cpw5_d(cpw5_pessoa(cpw5_i(r)) + 0x39);
            if c != 0 {
                let l = cpw5_d(c + 0x57);
                if l != 0 && cpw5_i(l) == liga {
                    return r;
                }
            }
        }
        minimo -= 1;
        if minimo < 0 {
            return 0;
        }
    }
}

/// O comparador do melhor do mês: lado a lado pela tabela ordenada e pelo registro devolvido.
unsafe extern "thiscall" fn comparar_melhor_do_mes(this: usize, liga: i32, idade: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, u32) -> usize>(1286);
    let (tab, n) = (cpw5_d(this), cpw5_i(this + 4));
    let regioes: Vec<(usize, usize)> = if tab != 0 && n > 0 { vec![(tab, n as usize * 12)] } else { Vec::new() };
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, liga, idade), || melhor_do_mes(this, liga, idade));
    PLACAR.registrar(igual && r == m, || format!("melhor_do_mes(liga {}, idade {}) original={:#x} {:x?} rust={:#x} {:x?}", liga, idade as u8 as i8, r, mo, m, mr));
    r
}

