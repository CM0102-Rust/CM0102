// ---------- %%LEVA%%a leva: os registros dos técnicos: o resultado de cada jogo, a posição no fim da liga e o jogador do ano da torcida (staff_records.cpp) ----------

unsafe fn cqa11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal em `a`.
unsafe fn cqa11_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cqa11_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cqa11_i(a).wrapping_add(v));
}

unsafe fn increment_i16(a: usize) {
    core::ptr::write_unaligned(a as *mut i16, cqa11_w(a).wrapping_add(1));
}

unsafe fn cqa11_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de pontos do técnico do clube (+0xcf, lido sem conferir o clube; 0x86f5b0 com criar), ou 0 sem técnico.
unsafe fn club_manager_record(this: usize, clube: usize) -> usize {
    let t = cqa11_d(clube + 0xcf);
    if t == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    f(this, t, 1)
}

/// Os pontos de um lado na partida `m`: ((a + b) × 5 + (c − b, em 8 bits)) × 2 + d, vezes 5 (bytes com sinal).
unsafe fn side_match_points(m: usize, a: usize, b: usize, c: usize, d: usize) -> i32 {
    let x = cqa11_c(m + a).wrapping_add(cqa11_c(m + b));
    let y = (*((m + c) as *const u8)).wrapping_sub(*((m + b) as *const u8)) as i8 as i32;
    let x = x.wrapping_mul(5).wrapping_add(y);
    cqa11_c(m + d).wrapping_add(x.wrapping_mul(2)).wrapping_mul(5)
}

/// A nota do jogo para o técnico: a importância (0x48e190 da competição do jogo, com `lado` e a saída em `saida`; só o
/// byte, com sinal) × (reputação do adversário / 500) × o peso da competição (+0x69 de [+0x14]) × (pontos + bônus) × 50,
/// dividido por 100000 (divisões com sinal truncadas, como as multiplicações mágicas da original; produtos em 32 bits).
unsafe fn match_rating_for_manager(p: usize, lado: u32, saida: &mut u32, adversario: usize, pontos: i32) -> i32 {
    let comp = cqa11_d(cqa11_d(0xadadfc).wrapping_add(4 * cqa11_d(p)));
    let importancia: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(0x48e190usize);
    let i = importancia(comp, p, lado, saida as *mut u32) as u8 as i8 as i32;
    let x = i
        .wrapping_mul(cqa11_w(adversario + 0x80) as i32 / 500)
        .wrapping_mul(cqa11_w(cqa11_d(p + 0x14) + 0x69) as i32)
        .wrapping_mul(pontos)
        .wrapping_mul(50);
    x / 100000
}

/// { 0x868830 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::record_match_result(jogo, partida)`, pelo
/// fechamento da partida do dia (0x6b5f00), a cada jogo. Jogo não disputado (+0x3a = 1): nada. Os registros dos dois
/// técnicos (+0xcf do mandante +0x1c e do visitante +0x20; 0x86f5b0 com criar): +1 jogo (+0x8e); os gols pró (+0x96) e
/// contra (+0x9a) pelo placar (+0x47/+0x48, ou +0x43/+0x44 com +0x47 = −1); vitória (+0x90, bônus 900), empate (+0x92,
/// 300 para os dois) ou derrota (+0x94). A nota de cada um (0x48e190 com 0 / 1, a reputação do adversário, os pontos da
/// partida em +0x11f.. / +0x744..) vai para +0xb6 e +0xca; na liga do clube (+0x57 ou +0x60 = a competição +0x14) também
/// para os pontos do mês (+0xce); na seleção (0x53b900) para +0xbe, senão +0xba. Defeito da original, reproduzido: no
/// visitante, +0xbe/+0xba recebem o [esp+0x10] da rotina, que é a nota do mandante (ou o bônus dele, sem registro).
unsafe extern "thiscall" fn record_match_result(this: usize, p: usize, m: usize) {
    if *((p + 0x3a) as *const u8) == 1 {
        return;
    }
    let (casa, fora) = (cqa11_d(p + 0x1c), cqa11_d(p + 0x20));
    let a = club_manager_record(this, casa);
    let v = club_manager_record(this, fora);
    if a != 0 {
        increment_i16(a + 0x8e);
    }
    if v != 0 {
        increment_i16(v + 0x8e);
    }
    let (ga, gv) = if *((p + 0x47) as *const u8) == 0xff {
        (cqa11_c(p + 0x43), cqa11_c(p + 0x44))
    } else {
        (cqa11_c(p + 0x47), cqa11_c(p + 0x48))
    };
    if a != 0 {
        cqa11_somar(a + 0x96, ga);
        cqa11_somar(a + 0x9a, gv);
    }
    if v != 0 {
        cqa11_somar(v + 0x96, gv);
        cqa11_somar(v + 0x9a, ga);
    }
    let (mut ba, mut bv) = (0i32, 0i32);
    if ga > gv {
        ba = 900; // cqa11: o bônus da vitória do mandante
        if a != 0 {
            crate::comparacao::ramo("cqa_11 vitoria do mandante");
            increment_i16(a + 0x90);
        }
        if v != 0 {
            increment_i16(v + 0x94);
        }
    } else if ga < gv {
        bv = 900;
        if v != 0 {
            increment_i16(v + 0x90);
        }
        if a != 0 {
            increment_i16(a + 0x94);
        }
    } else {
        ba = 300;
        bv = 300;
        if a != 0 {
            increment_i16(a + 0x92);
        }
        if v != 0 {
            increment_i16(v + 0x92);
        }
    }
    // a saída da 0x48e190 vai na vaga do argumento `jogo` da original, que começa com o próprio jogo
    let mut saida = p as u32;
    let mut guardado = ba; // o [esp+0x10] da original: o bônus do mandante, depois a nota dele
    let comp = cqa11_d(p + 0x14);
    if a != 0 {
        crate::comparacao::ramo("cqa_11 nota do mandante");
        let q = match_rating_for_manager(p, 0, &mut saida, fora, side_match_points(m, 0x11f, 0x128, 0x127, 0x129).wrapping_add(ba));
        cqa11_somar(a + 0xb6, q);
        cqa11_somar(a + 0xca, q);
        guardado = q;
        if comp == cqa11_d(casa + 0x57) || comp == cqa11_d(casa + 0x60) {
            crate::comparacao::ramo("cqa_11 pontos do mes");
            cqa11_somar(a + 0xce, q); // cqa11: os pontos do mês do mandante
        }
        if is_national_team(casa) != 0 {
            cqa11_somar(a + 0xbe, q);
        } else {
            cqa11_somar(a + 0xba, q);
        }
    }
    if v != 0 {
        let q = match_rating_for_manager(p, 1, &mut saida, casa, side_match_points(m, 0x744, 0x74d, 0x74c, 0x74e).wrapping_add(bv));
        cqa11_somar(v + 0xb6, q);
        cqa11_somar(v + 0xca, q);
        if comp == cqa11_d(fora + 0x57) || comp == cqa11_d(fora + 0x60) {
            cqa11_somar(v + 0xce, q);
        }
        if is_national_team(fora) != 0 {
            cqa11_somar(v + 0xbe, guardado);
        } else {
            cqa11_somar(v + 0xba, guardado);
        }
    }
}

/// Comparador lado a lado nos dois registros (0xd2 bytes), criados antes pela 0x86f5b0 como a original faria (a segunda
/// chamada acha o mesmo registro). Jogo não disputado: só a original (não cria nada).
unsafe extern "thiscall" fn comparar_record_match_result(this: usize, p: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if *((p + 0x3a) as *const u8) == 1 {
        return o(this, p, m);
    }
    let a = club_manager_record(this, cqa11_d(p + 0x1c));
    let v = club_manager_record(this, cqa11_d(p + 0x20));
    let mut reg = Vec::new();
    for r in [a, v] {
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, m), || record_match_result(this, p, m));
    PLACAR.registrar(igual, || format!("record_match_result(jogo {:#x}) original={:x?} rust={:x?}", p, mo, mr));
}

/// { 0x86a070 } (thiscall, ret 4, staff_records.cpp, SEH só da data local): `Staff_Records::add_league_finish_records(
/// liga)`, pela liga completa (0x689ad0), no fim da liga. Para cada linha da tabela (0x6826d0, até +0x3e), o histórico do
/// técnico do clube (+0xcf; tipo 0xa: o id do clube, a competição de +4, o ano +0x3a e a posição k + 1), no arquivo
/// (0x58e340).
unsafe extern "thiscall" fn add_league_finish_records(this: usize, liga: usize) {
    crate::comparacao::ramo("cqa_11 fim da liga");
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x6826d0usize);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0xa;
    put_i32(&mut e, 0x11, cqa11_w(liga + 0x3a) as i32);
    put_i32(&mut e, 0xd, cqa11_i(cqa11_d(liga + 4)));
    let mut k: i16 = 0;
    while k < cqa11_w(liga + 0x3e) {
        let clube = cqa11_d(linha(liga, k as i32));
        put_i32(&mut e, 0x15, k as i32 + 1);
        put_i32(&mut e, 8, cqa11_i(clube));
        let t = cqa11_d(clube + 0xcf);
        if t != 0 {
            append_person_history_by_id(true, this, cqa11_d(t), &e);
        }
        k += 1;
    }
}

// ponytail: só grava o histórico no arquivo (não roda duas vezes): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_add_league_finish_records(this: usize, liga: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, liga)
}

/// O ensaio do comparador da 0x86dc30: a cópia sem o histórico (arquivo).
static mut SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN: bool = false;

/// { 0x86dc30 } (thiscall, ret 0x10, staff_records.cpp, SEH só da data local): `Staff_Records::
/// add_supporters_player_of_year_record(pessoa, clube, a, b)`, pelo fim da temporada dos recordes (0x447880). O histórico
/// da pessoa (tipo 0x35: o id do clube, `a` em 16 bits sem sinal e `b`), no arquivo (0x58e340); depois, se sorteio(21 −
/// +0x59 da pessoa) < sorteio(10), a pessoa gosta do clube (0x547d90 com sorteio(2)) e o clube dela (0x547870 com
/// sorteio(2)). Pessoa nula: a original (a caixa e a leitura de +0x59 da pessoa nula).
unsafe extern "thiscall" fn add_supporters_player_of_year_record(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    if pessoa == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%)(this, pessoa, clube, a, b);
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x35;
    put_i32(&mut e, 8, cqa11_i(clube));
    put_i32(&mut e, 0xd, (a & 0xffff) as i32);
    put_i32(&mut e, 0x11, b as i32);
    append_person_history_by_id(!SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN, this, cqa11_d(pessoa), &e);
    let r1 = cqa11_sortear(0x15 - cqa11_c(pessoa + 0x59));
    if r1 < cqa11_sortear(10) {
        crate::comparacao::ramo("cqa_11 goal_crowd_reaction favorita");
        let gosta: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547d90usize);
        let favorita: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547870usize);
        gosta(pessoa, clube, cqa11_sortear(2) as u32);
        favorita(clube, pessoa, cqa11_sortear(2) as u32);
    }
}

/// Comparador em ensaio: a cópia sem o histórico e a original a partir do mesmo estado (sorteio, as pessoas favoritas e
/// rejeitadas do clube +0x9b e os clubes favoritos e rejeitados da pessoa, preferências +0x65: +4..+0x18). Pessoa nula:
/// só a original.
unsafe extern "thiscall" fn comparar_add_supporters_player_of_year_record(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%);
    if pessoa == 0 || clube == 0 {
        return o(this, pessoa, clube, a, b);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube + 0x9b, 0x18));
    let pref = cqa11_d(pessoa + 0x65);
    if pref != 0 {
        reg.push((pref + 4, 0x18));
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| add_supporters_player_of_year_record(this, pessoa, clube, a, b)));
    SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, pessoa, clube, a, b);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("add_supporters_player_of_year_record(pessoa {:#x}, clube {:#x}) original={:x?} rust={:x?}", pessoa, clube, mo, mr));
}
