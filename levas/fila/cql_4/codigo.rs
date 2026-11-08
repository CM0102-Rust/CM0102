// ---------- %%LEVA%%a leva: o desempate da Série A e da Série B italianas no fim da temporada (o "spareggio": empate em pontos na fronteira de duas faixas), a ordem dos empatados pelo confronto direto e o resultado do jogo de desempate (ita_ser_a.cpp, ita_ser_b.cpp) ----------

unsafe fn cql4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cql4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cql4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cql4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A linha `i` da tabela [+0xb1] (0x3b bytes).
unsafe fn ita_ser_a_table_row(liga: usize, i: usize) -> usize {
    cql4_d(liga + 0xb1) + i * 0x3b
}

/// As faixas da Série A, por posição (0 = o primeiro): 1 o campeão; 2 as outras vagas da Liga dos Campeões (as `cl`
/// vagas da nação, UefaSeeding::get_nation_places 0x905c60: push &uefa; push &cl; push [[liga+4]+0x5d] (a nação da
/// competição); mov ecx, [0xde1f58] -> thiscall, ret 0xc, que grava 1 nas duas antes de procurar); 3 as `uefa` vagas da
/// Copa da UEFA, logo depois; 4 as duas seguintes [inferência: a Intertoto]; 5 as `+0xc1` últimas de 18 (o
/// rebaixamento); 0 o resto. A original monta as 18 faixas num registro local (16 + 2 bytes) e lê duas além com uma
/// tabela de mais de 18 clubes; a cópia usa 64 bytes com zero depois das 18 (a Série A tem 18).
unsafe fn serie_a_zones(liga: usize) -> [u8; 64] {
    let vagas: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(0x905c60usize);
    let mut z = [0u8; 64];
    z[0] = 1;
    let (mut cl, mut uefa) = (0i8, 0i8);
    vagas(cql4_d(0xde1f58), cql4_d(cql4_d(liga + 4) + 0x5d), &mut cl, &mut uefa);
    // ponytail: as contas da nação ficam de 1 a 16 no jogo; a cópia ignora as faixas que cairiam fora do registro.
    let cl = cl as i32;
    let uefa = uefa as i32;
    if cl > 1 {
        for k in 1..cl {
            if (k as usize) < 64 {
                z[k as usize] = 2;
            }
        }
    }
    if uefa > 0 {
        for k in 0..uefa {
            if ((cl + k) as usize) < 64 {
                z[(cl + k) as usize] = 3;
            }
        }
    }
    let i = uefa + cl;
    if i >= 0 && (i as usize) + 1 < 64 {
        z[i as usize] = 4; // cql4: as duas posições depois das vagas europeias
        z[i as usize + 1] = 4;
    }
    let rebaixados = *((liga + 0xc1) as *const i8) as i32;
    if rebaixados > 0 {
        for k in 0..rebaixados {
            if 17 - k >= 0 {
                z[(17 - k) as usize] = 5;
            }
        }
    }
    z
}

/// As faixas da Série B: 1 o campeão; 2 as 3 vagas seguintes de acesso; 3 as 4 últimas de 20 (o rebaixamento); 0 o
/// resto (registro local de 20 bytes na original).
fn serie_b_zones() -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    z[1] = 2;
    z[2] = 2;
    z[3] = 2; // cql4: as vagas de acesso da Série B
    for k in 16..20 {
        z[k] = 3;
    }
    z
}

/// Há duas linhas vizinhas com os mesmos pontos (+0x19) em faixas diferentes? (a conferência do comparador)
unsafe fn tie_on_zone_boundary(liga: usize, z: &[u8; 64]) -> bool {
    let n = cql4_w(liga + 0x3e) as i32 - 1;
    (0..n.max(0) as usize).any(|i| {
        let l = ita_ser_a_table_row(liga, i);
        cql4_w(l + 0x19) == cql4_w(l + 0x3b + 0x19) && z[i.min(63)] != z[(i + 1).min(63)]
    })
}

/// { 0x642880 } (thiscall, ret 8, ita_ser_a.cpp, slot +0x14): `ItaSerA::update_for_date(hoje, completo)`. Primeiro o
/// dia de liga (League::update_for_date 0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa. Depois, no fim da temporada (+0xdd), na fase principal (+0x2c = 0, sem fases criadas), com
/// `completo` e com +0xe6 [inferência: a tabela fechada]: zera +0x4c e procura, de cima para baixo, o primeiro par de
/// clubes vizinhos com os mesmos pontos em faixas diferentes (serie_a_zones). Achando: ordena os empatados
/// pelo confronto direto (0x6471a0: push pontos; mov ecx, this -> thiscall, ret 4), cria o vetor de 1 fase (+0x2c =
/// 1, +0xc = malloc 0x944e46 de 4: push 4; add esp, 4 -> cdecl) e chama a virtual +0x28 (a fase seguinte: o jogo de
/// desempate). Sem empate na fronteira: +0xdd = 0, +0xe6 = 0 e a virtual +0x94 (League::update_positions_and_status).
/// A regra: na Itália, empate em pontos entre clubes de faixas diferentes (título, Liga dos Campeões, Copa da UEFA,
/// Intertoto [inferência], rebaixamento) não se decide pelo saldo: vai a um jogo extra.
unsafe extern "thiscall" fn ita_ser_a_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let dia: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x6471a0usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let r = dia(this, hoje, completo);
    // ponytail: a original devolve o byte do dia de liga com os bits de cima do eax que sobram da última chamada; o
    // laço das competições só lê o byte.
    if cql4_d(this + 0xdd) == 0 || cql4_d(this + 0x2c) != 0 || completo == 0 || cql4_d(this + 0xe6) == 0 {
        return r;
    }
    let z = serie_a_zones(this);
    let n = cql4_w(this + 0x3e) as i32 - 1;
    cql4_pd(this + 0x4c, 0);
    crate::comparacao::ramo("cql_4 fim da serie a");
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let l = ita_ser_a_table_row(this, i as usize);
            let pontos = cql4_w(l + 0x19);
            if pontos == cql4_w(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série A na fronteira
                ordenar(this, pontos as u16 as u32);
                cql4_pd(this + 0x2c, 1);
                let v = alocar(4);
                cql4_pd(this + 0xc, v);
                let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x28));
                proxima(this);
                return r;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    cql4_pd(this + 0xdd, 0);
    cql4_pd(this + 0xe6, 0);
    let situacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x94));
    situacao(this);
    r
}

/// { 0x647050 } (thiscall, ret 8, ita_ser_b.cpp, slot +0x14): `ItaSerB::update_for_date(hoje, completo)`. Como a da
/// Série A, com as faixas fixas da Série B (o campeão, mais 3 de acesso, os 4 rebaixados de 20) e sem parar no
/// primeiro par: cada par vizinho empatado na fronteira é ordenado pelo confronto direto (0x6471a0: push edi, os
/// pontos na palavra de baixo e o resto do `completo` nos bits de cima, que a 0x6471a0 não lê; mov ecx -> thiscall,
/// ret 4); havendo algum, a fase do desempate (+0x2c = 1, +0xc = malloc de 4, virtual +0x28). Sem nenhum: +0xdd = 0,
/// +0xe6 = 0 e a virtual +0x94.
unsafe extern "thiscall" fn ita_ser_a_update_for_date_647050(this: usize, hoje: usize, completo: u32) -> u32 {
    let dia: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x6471a0usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let r = dia(this, hoje, completo);
    if cql4_d(this + 0xdd) == 0 || cql4_d(this + 0x2c) != 0 || completo == 0 || cql4_d(this + 0xe6) == 0 {
        return r;
    }
    let z = serie_b_zones();
    cql4_pd(this + 0x4c, 0);
    crate::comparacao::ramo("cql_4 fim da serie b");
    let mut achou = false;
    let mut i: i16 = 0;
    while (i as i32) < cql4_w(this + 0x3e) as i32 - 1 {
        let l = ita_ser_a_table_row(this, i as usize);
        let pontos = cql4_w(l + 0x19);
        if pontos == cql4_w(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série B na fronteira
            achou = true;
            ordenar(this, pontos as u16 as u32);
        }
        i = i.wrapping_add(1);
    }
    if achou {
        cql4_pd(this + 0x2c, 1);
        let v = alocar(4);
        cql4_pd(this + 0xc, v);
        let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x28));
        proxima(this);
        return r;
    }
    cql4_pd(this + 0xdd, 0);
    cql4_pd(this + 0xe6, 0);
    let situacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x94));
    situacao(this);
    r
}

/// O comparador das duas: o dia de liga e a fase do desempate não rodam duas vezes: só a original, conferindo o que
/// ela garante. Na fase principal e com `completo`: se a fase do desempate foi criada (+0x2c mudou), há na tabela um
/// par vizinho empatado em faixas diferentes; e a original nunca sai com o fim da temporada pendente (+0xdd e +0xe6
/// ligados e +0x2c = 0), porque ou cria a fase ou os zera.
unsafe fn compare_league_day(nome: &str, this: usize, completo: u32, faixas: impl Fn() -> [u8; 64], original: impl FnOnce() -> u32) -> u32 {
    let fases = cql4_d(this + 0x2c);
    let r = original();
    if completo != 0 && fases == 0 {
        if cql4_d(this + 0x2c) != 0 {
            let z = faixas();
            PLACAR.registrar(tie_on_zone_boundary(this, &z) && cql4_d(this + 0xc) != 0, || format!("{}({:#x}): fase do desempate sem empate na fronteira (+0xc = {:#x})", nome, this, cql4_d(this + 0xc)));
        } else {
            PLACAR.registrar(!(cql4_d(this + 0xdd) != 0 && cql4_d(this + 0xe6) != 0), || format!("{}({:#x}): o fim da temporada ficou pendente (+0xdd e +0xe6 ligados)", nome, this));
        }
    }
    r
}

unsafe extern "thiscall" fn comparar_ita_ser_a_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    compare_league_day("ita_ser_a_update_for_date", this, completo, || serie_a_zones(this), || o(this, hoje, completo))
}

unsafe extern "thiscall" fn comparar_ita_ser_a_update_for_date_647050(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    compare_league_day("ita_ser_a_update_for_date_647050", this, completo, serie_b_zones, || o(this, hoje, completo))
}

/// { 0x6471a0 } (thiscall, ret 4, ita_ser_b.cpp; chamada também pela da Série A): `ita_league_sort_clubs_level_on_points
/// (pontos)`. Monta o confronto direto em +4 de cada linha (League::build_h2h_table 0x682b30: push [+0xb1]; mov ecx,
/// this -> thiscall, ret 4), acha o bloco das linhas com esses pontos (a primeira e quantas; a tabela já está ordenada
/// pelos pontos), ordena o bloco pela comparação do confronto (qsort 0x945333 com 0x683410, linhas de 0x3b: push;
/// add esp, 0x10 -> cdecl; sem linha, o qsort recebe 0 e 0) e renumera a posição (+4) de todas as linhas.
unsafe extern "thiscall" fn ita_league_sort_clubs_level_on_points(this: usize, pontos: u32) {
    let confronto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x682b30usize);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    confronto(this, cql4_d(this + 0xb1));
    let n = cql4_w(this + 0x3e);
    let mut primeira = 0usize;
    let mut quantas: u32 = 0;
    if n > 0 {
        let mut l = cql4_d(this + 0xb1);
        for _ in 0..n as i32 {
            if cql4_w(l + 0x19) == pontos as u16 as i16 {
                if primeira == 0 {
                    primeira = l;
                }
                quantas = quantas.wrapping_add(1);
            }
            l += 0x3b;
        }
    }
    qsort(primeira, quantas as u16 as i16 as i32, 0x3b, 0x683410); // cql4: o bloco dos empatados
    let mut i: i16 = 0;
    while i < cql4_w(this + 0x3e) {
        *((ita_ser_a_table_row(this, i as usize) + 4) as *mut u8) = i as u8;
        i += 1;
    }
}

/// Lado a lado na tabela inteira (a 0x682b30 grava +4 de cada linha e solta a sua minitabela; o qsort só mexe no
/// bloco).
unsafe extern "thiscall" fn comparar_ita_league_sort_clubs_level_on_points(this: usize, pontos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%);
    let t = cql4_d(this + 0xb1);
    let n = cql4_w(this + 0x3e).max(0) as usize;
    if t == 0 || n == 0 {
        return o(this, pontos);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(t, n * 0x3b)], || o(this, pontos), || ita_league_sort_clubs_level_on_points(this, pontos));
    PLACAR.registrar(igual, || format!("ita_league_sort_clubs_level_on_points({:#x}, {}) original={:x?} rust={:x?}", this, pontos as u16 as i16, mo, mr));
}

/// { 0x641dc0 } (thiscall, ret 0x18, ita_ser_a.cpp, slot +0x44): `ItaSerA::update_staff_records_for_status(clube,
/// situação, fase, a4, jogo, a6)`. Na liga (fase 0xff): a da base (League::update_staff_records_for_status 0x686940:
/// push a6; push jogo; push a4; push -1; push situação; push clube; mov ecx, this -> thiscall, ret 0x18), cuja devolução
/// repassa. No jogo de desempate (outra fase), só com a situação 1 [inferência: o clube venceu]: +0xad = 0, ordena a
/// tabela (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall, ret 4) e acha a linha do clube: se o de cima é
/// um dos dois do jogo (+0x1c, +0x20), as duas linhas trocam de lugar; senão, se o de baixo é um deles, ficam. Nos dois
/// casos as duas linhas ganham a posição (+4) e o destino em aberto (+0x37 = 0xff). Depois, se nenhuma linha tem o
/// destino 1 (o play-off de cima), fecha a temporada: +0xe6 = 0, +0xdd = 0, a virtual +0x94, a virtual +0x10 (0, 0) e
/// +0xdd = 1. Devolve −1. A regra: quem vence o jogo de desempate fica à frente do outro.
unsafe extern "thiscall" fn ita_ser_a_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    if fase as u8 == 0xff {
        let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x686940usize);
        return base(this, clube, situacao, 0xffff_ffff, a4, jogo, a6);
    }
    if situacao as u8 != 1 {
        return 0xffff_ffff;
    }
    crate::comparacao::ramo("cql_4 resultado do desempate");
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    cql4_pd(this + 0xad, 0);
    ordenar(this, 0);
    let t = cql4_d(this + 0xb1);
    let (a, b) = (cql4_d(jogo + 0x1c), cql4_d(jogo + 0x20));
    let mut i: i16 = 0;
    while i < cql4_w(this + 0x3e) {
        let l = t + i as usize * 0x3b;
        if cql4_d(l) == clube {
            if i > 0 {
                let acima = cql4_d(l - 0x3b);
                if acima == a || acima == b {
                    // o clube estava abaixo do outro: trocam
                    let mut tmp = [0u8; 0x3b];
                    core::ptr::copy_nonoverlapping(l as *const u8, tmp.as_mut_ptr(), 0x3b);
                    core::ptr::copy_nonoverlapping((l - 0x3b) as *const u8, l as *mut u8, 0x3b);
                    let l2 = cql4_d(this + 0xb1) + i as usize * 0x3b;
                    core::ptr::copy_nonoverlapping(tmp.as_ptr(), (l2 - 0x3b) as *mut u8, 0x3b);
                    let l3 = cql4_d(this + 0xb1) + i as usize * 0x3b;
                    *((l3 + 4) as *mut u8) = i as u8;
                    *((l3 - 0x3b + 4) as *mut u8) = (i as u8).wrapping_sub(1);
                    *((l3 + 0x37) as *mut u8) = 0xff;
                    *((l3 - 0x3b + 0x37) as *mut u8) = 0xff;
                    break;
                }
            }
            if i < cql4_w(this + 0x3e) {
                // ponytail: a original lê o clube da linha seguinte mesmo na última (4 bytes além da tabela), como aqui.
                let abaixo = cql4_d(l + 0x3b);
                if abaixo == a || abaixo == b {
                    *((t + i as usize * 0x3b + 4) as *mut u8) = i as u8;
                    let t2 = cql4_d(this + 0xb1);
                    *((t2 + i as usize * 0x3b + 0x3f) as *mut u8) = (i as u8).wrapping_add(1);
                    *((cql4_d(this + 0xb1) + i as usize * 0x3b + 0x37) as *mut u8) = 0xff;
                    *((cql4_d(this + 0xb1) + i as usize * 0x3b + 0x72) as *mut u8) = 0xff;
                    break;
                }
            }
        }
        i += 1;
    }
    let n = cql4_w(this + 0x3e);
    let mut k: i16 = 0;
    while k < n {
        if cql4_b(ita_ser_a_table_row(this, k as usize) + 0x37) == 1 {
            return 0xffff_ffff;
        }
        k += 1;
    }
    cql4_pd(this + 0xe6, 0);
    cql4_pd(this + 0xdd, 0);
    let situacao_f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x94));
    situacao_f(this);
    let fim: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cql4_d(cql4_d(this) + 0x10));
    fim(this, 0, 0);
    cql4_pd(this + 0xdd, 1);
    0xffff_ffff
}

/// Na liga e no desempate com a situação 1 há chamadas que não rodam duas vezes (os registros da base, a ordenação, as
/// virtuais): só a original. Nas outras situações a cópia só devolve −1: compara a devolução.
unsafe extern "thiscall" fn comparar_ita_ser_a_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I3%%);
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    if fase as u8 != 0xff && situacao as u8 != 1 {
        PLACAR.registrar(r == 0xffff_ffff, || format!("ita_ser_a_update_staff_records_for_status: devolveu {:#x}", r));
    }
    r
}
