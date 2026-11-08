// ---------- %%LEVA%%a leva: o jogo da Liga dos Campeões: o prêmio e o coeficiente do jogo, o progresso do clube e o conflito de datas da competição (european_cup.cpp, comp.cpp) ----------

unsafe fn cpy6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de finanças do clube (0x5a0590: thiscall(tabela [0xae2a38], clube), ret 4).
unsafe fn european_cup_club_finance(clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    f(cpy6_d(0xae2a38), clube)
}

/// Soma no saldo, em +0xa8 e em +0x148 (0x5999a0: thiscall(finanças, valor), ret 4).
unsafe fn european_cup_add_prize_money(fin: usize, valor: i32) {
    let f: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x5999a0usize);
    f(fin, valor)
}

/// Os pontos do coeficiente da UEFA para a nação (0x9058b0: thiscall(tabela [0xde1f58], nação, pontos), ret 8; soma
/// o byte dos pontos na palavra +0xf + 18 × índice (+0x1df da seleção 0x53b3d0 da nação)).
unsafe fn add_uefa_coefficient(nacao: usize, pontos: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x9058b0usize);
    f(cpy6_d(0xde1f58), nacao, pontos);
}

/// O ensaio do comparador da 0x583b10: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca) e sem a base da copa
/// (0x51a150, que processa o jogo inteiro e não pode rodar duas vezes).
static mut EUROPEAN_CUP_FIXTURE_DRY_RUN: bool = false;

/// A notícia do prêmio (Comp 0x48e3e0: thiscall(this, clube, valor, 0, −1, 0, jogo, −2), ret 0x1c); no ensaio, nada.
unsafe fn european_cup_send_prize_money_news(this: usize, clube: usize, valor: u32, jogo: usize) {
    if EUROPEAN_CUP_FIXTURE_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, usize, i32) = core::mem::transmute(0x48e3e0usize);
    f(this, clube, valor, 0, -1, 0, jogo, -2);
}

/// { 0x583b10 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::process_fixture(jogo, arg)` (a virtual +4 da Liga dos
/// Campeões, a cada jogo dela). O placar: +0x47/+0x48 (com +0x47 = −1, +0x43/+0x44), com sinal. O vencedor ganha 2
/// pontos no coeficiente da sua nação (+0x53; 0x9058b0), o empate 1 para cada. Na fase de grupos (+0x42 de 0 a 11):
/// 200 mil ao vencedor ou 100 mil a cada um no empate (0x5a0590 + 0x5999a0), com a notícia do prêmio (0x48e3e0;
/// no empate, as duas somas antes das duas notícias). Depois a base da copa (0x51a150 com o jogo e `arg`).
unsafe extern "thiscall" fn process_fixture(this: usize, jogo: usize, arg: u32) {
    let (ga, gb) = if cpy6_b(jogo + 0x47) != 0xff {
        (cpy6_b(jogo + 0x47) as i8, cpy6_b(jogo + 0x48) as i8)
    } else {
        (cpy6_b(jogo + 0x43) as i8, cpy6_b(jogo + 0x44) as i8)
    };
    let (a, b) = (cpy6_d(jogo + 0x1c), cpy6_d(jogo + 0x20));
    let (vencedor, perdedor) = if ga > gb {
        add_uefa_coefficient(cpy6_d(a + 0x53), 2);
        (cpy6_d(jogo + 0x1c), cpy6_d(jogo + 0x20))
    } else if ga < gb {
        add_uefa_coefficient(cpy6_d(b + 0x53), 2);
        (cpy6_d(jogo + 0x20), cpy6_d(jogo + 0x1c))
    } else {
        add_uefa_coefficient(cpy6_d(a + 0x53), 1);
        add_uefa_coefficient(cpy6_d(cpy6_d(jogo + 0x20) + 0x53), 1);
        (0, 0)
    };
    let _ = perdedor;
    let fase = cpy6_b(jogo + 0x42) as i8;
    if fase > -1 && fase < 0xc {
        if vencedor != 0 {
            crate::comparison::branch("cpy_6 premio da vitoria");
            european_cup_add_prize_money(european_cup_club_finance(vencedor), 0x30d40); // cpy6: o prêmio da vitória
            european_cup_send_prize_money_news(this, vencedor, 0x30d40, jogo);
        } else {
            crate::comparison::branch("cpy_6 premio do empate");
            european_cup_add_prize_money(european_cup_club_finance(cpy6_d(jogo + 0x1c)), 0x186a0); // cpy6: o prêmio do empate do mandante
            european_cup_add_prize_money(european_cup_club_finance(cpy6_d(jogo + 0x20)), 0x186a0);
            european_cup_send_prize_money_news(this, cpy6_d(jogo + 0x1c), 0x186a0, jogo);
            european_cup_send_prize_money_news(this, cpy6_d(jogo + 0x20), 0x186a0, jogo);
        }
    }
    if EUROPEAN_CUP_FIXTURE_DRY_RUN {
        return;
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x51a150usize);
    base(this, jogo, arg);
}

/// A palavra do coeficiente da nação do clube: [0xde1f58] + 18 × (+0x1df da seleção 0x53b3d0 da nação +0x53) + 0xf,
/// como a 0x9058b0 calcula; 0 sem tabela, sem nação ou sem seleção.
unsafe fn nation_coefficient_word(clube: usize) -> usize {
    let tabela = cpy6_d(0xde1f58);
    let nacao = cpy6_d(clube + 0x53);
    if tabela == 0 || nacao == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let s = selecao(nacao);
    if s == 0 {
        return 0;
    }
    tabela.wrapping_add((cpy6_b(s + 0x1df) as i8 as isize * 18) as usize + 0xf)
}

/// Comparador da 0x583b10, em ensaio: a cópia sem a notícia e sem a base, e a original inteira, a partir do mesmo
/// estado (as palavras do coeficiente das duas nações e os registros de finanças dos dois clubes, restaurados depois
/// do ensaio). Confere as palavras do coeficiente, que a base não toca, salvo pelo sorteio das fases (0x585290,
/// 0x585ab0: o coeficiente dos classificados), que só roda quando a fase (+0x30) anda: aí, sem comparação. As
/// finanças não entram (a base paga o prêmio da rodada da copa nos mesmos campos, 0x51a2fe); o prêmio segue o ramo do
/// placar que os pontos do coeficiente já conferem. Sem clube, nação, seleção ou finanças: só a original.
unsafe extern "thiscall" fn comparar_process_fixture(this: usize, jogo: usize, arg: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let (a, b) = if jogo != 0 { (cpy6_d(jogo + 0x1c), cpy6_d(jogo + 0x20)) } else { (0, 0) };
    if a == 0 || b == 0 {
        return o(this, jogo, arg); // ponytail: sem clube, só a original
    }
    let (ca, cb) = (nation_coefficient_word(a), nation_coefficient_word(b));
    let (fa, fb) = (european_cup_club_finance(a), european_cup_club_finance(b));
    if ca == 0 || cb == 0 || fa == 0 || fb == 0 {
        return o(this, jogo, arg); // ponytail: sem nação, seleção ou finanças, só a original (cai do mesmo jeito)
    }
    let conferir = [(ca, 2), (cb, 2)];
    let antes = crate::comparison::MemorySnapshot::read(&[(ca, 2), (cb, 2), (fa, 0x167), (fb, 0x167)]);
    let fase = cpy6_i(this + 0x30);
    EUROPEAN_CUP_FIXTURE_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| process_fixture(this, jogo, arg)));
    EUROPEAN_CUP_FIXTURE_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&conferir);
    antes.write_back();
    o(this, jogo, arg);
    if cpy6_i(this + 0x30) != fase {
        return; // ponytail: a fase andou na base e o sorteio somou coeficiente: sem comparação
    }
    let mo = crate::comparison::MemorySnapshot::read(&conferir);
    PLACAR.record(mo == mr, || format!("process_fixture(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// { 0x586c00 } (thiscall, ret 0x14, european_cup.cpp): `EuropeanCup::set_club_progress(clube, fase, p1, p2, p3)` (a
/// virtual +0x68, da base da copa 0x521b30): converte os três códigos de progresso pela fase e grava no registro do
/// clube na estatística da competição (0x4a4850 de +8, sem conferir o nulo, como a original) +0x73 ← p1, +0x74 ← p2,
/// +0x75 ← p3. Fase −1 (a qualificação): 1 → 0x19 (p2: 1), 17 → 0x21, 33 → 0x31, 47 → 0x3f; fase abaixo de 8 (com
/// sinal): 1 e 2 → 1 (p3: 0xd), 3 → 0x11, 4 → 0x19; abaixo de 12: 1 e 2 → 1 (p3: 7), 3 → 9, 4 → 0xd; os outros
/// códigos e as fases de 12 em diante passam como estão.
unsafe extern "thiscall" fn european_cup_set_club_progress(this: usize, clube: usize, fase: u32, p1: u32, p2: u32, p3: u32) {
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = buscar(cpy6_d(this + 8), clube);
    let (fase, p1, p2, p3) = (fase as u8 as i8, p1 as u8, p2 as u8, p3 as u8);
    let (c73, c74, c75) = if fase == -1 {
        let m = |p: u8, um: u8| match p {
            1 => um,
            17 => 0x21,
            33 => 0x31,
            47 => 0x3f,
            _ => p,
        };
        (m(p1, 0x19), m(p2, 1), m(p3, 0x19))
    } else if fase < 8 {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 0x11,
            4 => 0x19,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 0xd))
    } else if fase < 0xc {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 9,
            4 => 0xd,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 7))
    } else {
        (p1, p2, p3)
    };
    *((e + 0x74) as *mut u8) = c74;
    *((e + 0x73) as *mut u8) = c73;
    *((e + 0x75) as *mut u8) = c75;
}

/// Comparador da 0x586c00: lado a lado nos três bytes do registro do clube (+0x73..+0x75); sem registro (a original
/// cai), só a original.
unsafe extern "thiscall" fn comparar_european_cup_set_club_progress(this: usize, clube: usize, fase: u32, p1: u32, p2: u32, p3: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = buscar(cpy6_d(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, p1, p2, p3); // ponytail: sem registro, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(
        &[(e + 0x73, 3)],
        || o(this, clube, fase, p1, p2, p3),
        || european_cup_set_club_progress(this, clube, fase, p1, p2, p3),
    );
    PLACAR.record(igual, || format!("european_cup_set_club_progress(fase {} {} {} {}) original={:x?} rust={:x?}", fase as u8 as i8, p1 as u8, p2 as u8, p3 as u8, mo, mr));
}

/// { 0x48e360 } (thiscall, ret 8, comp.cpp): `Comp::counts_for_fixture_clash(jogo, outro)` (a virtual +0x60 das
/// competições, do conflito de datas 0x5adba0): 1 se o algarismo das unidades (0x4b0850) da rodada (+0x30, com sinal)
/// é 0 (rodada de 1000 em diante também dá 0), senão 0. `outro` não é lido.
unsafe extern "thiscall" fn counts_for_fixture_clash(_this: usize, jogo: usize, _outro: usize) -> u32 {
    let units_digit: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    (units_digit(cpy6_w(jogo + 0x30) as i32) == 0) as u32
}

unsafe extern "thiscall" fn comparar_counts_for_fixture_clash(this: usize, jogo: usize, outro: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, jogo, outro);
    let n = counts_for_fixture_clash(this, jogo, outro);
    PLACAR.record(r == n, || format!("counts_for_fixture_clash(rodada {}) original={} rust={}", cpy6_w(jogo + 0x30), r, n));
    r
}
