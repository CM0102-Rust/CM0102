// ---------- %%LEVA%%a leva: o estado da lesão do jogador, a data da decisão sobre as ofertas e os candidatos a técnico do ano (injury.cpp, contract_manager.cpp, staff_records.cpp) ----------

unsafe fn cpr10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpr10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpr10_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O estado pela lesão atual da pessoa (o índice em [this] + id × 31 + 0xe; −1 sem lesão): a diferença da data do jogo
/// para a da lesão (+0x20 do registro de 48 bytes em [this+4], 0x54cc30) em anos × 365 + dias; abaixo de +0x2a, 2
/// (lesionado); abaixo de +0x28, 1 (recuperando a forma); senão 0.
unsafe fn injury_status(this: usize, pessoa: usize) -> u8 {
    let id = cpr10_d(pessoa);
    let k = cpr10_d(cpr10_d(this).wrapping_add(id.wrapping_mul(31)) + 0xe) as i32;
    if k == -1 {
        return 0;
    }
    let l = cpr10_d(this + 4).wrapping_add((k as usize).wrapping_mul(48));
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), cpr10_d(l + 0x20) as u32, cpr10_d(l + 0x24) as u32);
    let dias = (cpr10_u(t.as_ptr() as usize + 2) as i32).wrapping_mul(365).wrapping_add(cpr10_w(t.as_ptr() as usize) as i32);
    if dias < cpr10_w(l + 0x2a) as i32 {
        return 2; // cpr10: lesionado
    }
    (dias < cpr10_w(l + 0x28) as i32) as u8 // cpr10: recuperando a forma
}

/// { 0x62c400 } (thiscall, ret 4, injury.cpp): `Injury::get_player_injury_status(pessoa)`: pessoa nula, a caixa
/// "Error" (linha 0x556) e 0; id a partir de [0xae2364], 0. Senão a forma do dia (0x62c680) e o estado
/// (injury_status). Quente (escalação automática): sem contador.
unsafe extern "thiscall" fn get_player_injury_status(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        show_error_at(0x9e7f28, 0x556);
        return 0;
    }
    if cpr10_d(pessoa) as i32 >= cpr10_d(0xae2364) as i32 {
        return 0;
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    atualizar(this, pessoa);
    injury_status(this, pessoa)
}

unsafe fn injury_date_add_days(d: usize, n: u32, t: &mut [u8; 8]) -> usize {
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    mais(d, t.as_mut_ptr(), n)
}

/// A data `d` = a data em `p` (0x417c10).
unsafe fn injury_assign_date(d: usize, p: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(d, t.as_mut_ptr(), p);
}

/// A data `d` = a data do jogo + `n` dias (0x54c5b0, 0x417c10).
unsafe fn set_today_plus_days(d: usize, n: u32) {
    let mut t = [0u8; 8];
    injury_assign_date(d, injury_date_add_days(0xae2c90, n, &mut t));
}

/// A data `d` passa para o dia seguinte quando a fatia (+0xc da entrada, com sinal) passa de 2 (menos 3).
unsafe fn advance_time_slice(e: usize, f: i8) {
    *((e + 0xc) as *mut i8) = f;
    if f > 2 {
        *((e + 0xc) as *mut i8) = f.wrapping_sub(3);
        let mut t = [0u8; 8];
        injury_assign_date(e + 4, injury_date_add_days(e + 4, 1, &mut t));
    }
}

/// A entrada de decisão de tipo 3 da pessoa no vetor de 14 bytes ([this+0xc], conta +0x14): da posição +0x18, nas
/// entradas até `limite` (ano sem sinal, dia com sinal), a primeira de tipo 3 da pessoa é anulada (id −1; sem
/// `alterar` só fica de fora) e a busca para; depois, a partir dessa posição (ou da conta − [this+0x1c], se maior),
/// a de tipo 3 da pessoa. Devolve o endereço dela ou 0.
unsafe fn find_type3_decision(this: usize, id: i32, limite: &[u8; 8], alterar: bool) -> usize {
    let conta = cpr10_d(this + 0x14) as i32;
    let base = cpr10_d(this + 0xc);
    let (la, ld) = (u16::from_le_bytes([limite[2], limite[3]]), i16::from_le_bytes([limite[0], limite[1]]));
    let mut i = cpr10_d(this + 0x18) as i32;
    let mut anulada = -1i32;
    while i < conta {
        let r = base.wrapping_add((i as usize).wrapping_mul(14));
        let ra = cpr10_u(r + 6);
        if la < ra || (la == ra && ld < cpr10_w(r + 4)) {
            break;
        }
        if *((r + 0xd) as *const u8) == 3 && cpr10_d(r) as i32 == id {
            if alterar {
                core::ptr::write_unaligned(r as *mut u32, u32::MAX);
            }
            anulada = i;
            break;
        }
        i += 1;
    }
    let conta = cpr10_d(this + 0x14) as i32;
    i = i.max(conta.wrapping_sub(cpr10_d(this + 0x1c) as i32));
    while i < conta {
        let r = cpr10_d(this + 0xc).wrapping_add((i as usize).wrapping_mul(14));
        if *((r + 0xd) as *const u8) == 3 && cpr10_d(r) as i32 == id && i != anulada {
            return r;
        }
        i += 1;
    }
    0
}

/// { 0x4cd230 } (thiscall, ret 8, contract_manager.cpp, SEH só da data local, de destrutor vazio):
/// `CONTRACT_MANAGER::schedule_offer_decision(id, vaga)`: a entrada de decisão de tipo 3 da pessoa (find_type3_decision, com
/// o limite de hoje + 14; sem ela, uma nova em hoje + 1 + sorteio(5), fatia 2, pela 0x4cd6d0). Com a vaga da oferta
/// (0x8c61e0): o prazo da resposta (0x8cc5b0: dias × 3 + fatias) contra o que falta (dias até a data × 3 + fatias,
/// 0x54cdb0), o menor, no mínimo 1, vira a data (hoje + ÷3) e a fatia (+ resto, passando de 2 vai ao dia seguinte);
/// oferta com +0x2f e menos de 4 dias: hoje + 4 + sorteio(2). Sem vaga (−1): chegada a data, ela vira hoje e a fatia
/// sobe (até a do dia [0xae2c88]); se o contrato da pessoa ([this] + 80 × índice do registro de 79 bytes) acaba antes
/// de hoje + 180, hoje + 4 + sorteio(4). No fim, a data não passa de hoje + 14, e +0x1c sobe.
unsafe extern "thiscall" fn schedule_offer_decision(this: usize, id: i32, vaga: i32) {
    let mut limite = [0u8; 8];
    injury_date_add_days(0xae2c90, 0xe, &mut limite);
    let mut e = find_type3_decision(this, id, &limite, true);
    let pessoa = cpr10_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if e == 0 {
        crate::comparacao::ramo("cpr_10 nova decisao");
        let mut d = [0u8; 8];
        injury_date_add_days(0xae2c90, (sortear(5) + 1) as u32, &mut d); // cpr10: a decisão em 1 a 5 dias
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, *const u8, u32, u32) = core::mem::transmute(0x4cd6d0usize);
        acrescentar(this, pessoa, d.as_ptr(), 2, 3);
        let n = cpr10_d(this + 0x14);
        e = cpr10_d(this + 0xc).wrapping_add(n.wrapping_sub(1).wrapping_mul(14));
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let data = e + 4;
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
        let of = vaga_da(cpr10_d(0xde1f54), vaga);
        // ponytail: a original passa as vagas dos dois argumentos como saída; começam com os bytes baixos deles
        let (mut a, mut b) = (id as u8, vaga as u8);
        let prazo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x8cc5b0usize);
        prazo(cpr10_d(0xde1f54), cpr10_d(of), &mut a, &mut b);
        let alvo = a.wrapping_mul(3).wrapping_add(b) as i8;
        let falta = dias(data, cpr10_d(0xae2c90) as u32, cpr10_d(0xae2c94) as u32);
        let mut f = (*((e + 0xc) as *const u8)).wrapping_sub(*(0xae2c88 as *const u8)) as i8;
        let mut dl = falta as u8;
        if f < 0 {
            f = f.wrapping_add(3);
            dl = dl.wrapping_sub(1);
        }
        let atual = dl.wrapping_mul(3).wrapping_add(f as u8) as i8;
        let x = if atual < alvo { atual.max(1) } else { alvo.max(1) } as i32;
        set_today_plus_days(data, (x / 3) as u32);
        advance_time_slice(e, ((x % 3) as u8).wrapping_add(*(0xae2c88 as *const u8)) as i8);
        if *((cpr10_d(of) + 0x2f) as *const u8) != 0 && dias(data, cpr10_d(0xae2c90) as u32, cpr10_d(0xae2c94) as u32) < 4 {
            set_today_plus_days(data, (sortear(2) + 4) as u32);
        }
    } else {
        let (td, ta) = (cpr10_w(0xae2c90), cpr10_u(0xae2c92));
        let (rd, ra) = (cpr10_w(data), cpr10_u(data + 2));
        if ta > ra || (ta == ra && td >= rd) {
            injury_assign_date(data, 0xae2c90);
            let f = *((e + 0xc) as *const i8);
            if f as i32 <= *(0xae2c88 as *const u8) as i32 {
                advance_time_slice(e, f.wrapping_add(1));
            }
        }
        let p = cpr10_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        if p != 0 && (cpr10_d(p) as i32) < cpr10_d(0xae2364) as i32 {
            let x = cpr10_d(cpr10_d(0xae2d0c).wrapping_add(cpr10_d(p).wrapping_mul(79))) as i32;
            let c = if x >= 0 { cpr10_d(this).wrapping_add((x as usize).wrapping_mul(0x50)) } else { 0 };
            if c != 0 {
                let mut t = [0u8; 8];
                let m = injury_date_add_days(0xae2c90, 0xb4, &mut t);
                let (ma, ca) = (cpr10_u(m + 2), cpr10_u(c + 0x2f));
                if ma > ca || (ma == ca && cpr10_w(m) > cpr10_w(c + 0x2d)) {
                    set_today_plus_days(data, (sortear(4) + 4) as u32);
                }
            }
        }
    }
    let (td, ta) = (cpr10_w(0xae2c90), cpr10_u(0xae2c92));
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut t = [0u8; 8];
    let m = menos(data, t.as_mut_ptr(), 0xe);
    let ma = cpr10_u(m + 2);
    if ta < ma || (ta == ma && td < cpr10_w(m)) {
        set_today_plus_days(data, 0xe);
    }
    core::ptr::write_unaligned((this + 0x1c) as *mut u32, cpr10_d(this + 0x1c).wrapping_add(1) as u32);
}

/// { 0x86fab0 } (thiscall, ret 8, staff_records.cpp, SEH sem objeto com destrutor):
/// `Staff_Records::get_manager_of_year_candidates(clubes, n)` (pela 0x768940): o qsort dos `n` clubes (0x870220) e, na
/// ordem, até três cujo técnico (+0xcf) chegou (+0x3e) antes de hoje − 28 semanas (0x54cab0; ano sem sinal, dia com
/// sinal) e tem registro (0x86f5b0 com 1) com jogos (+0x8e), copiados para o começo do próprio vetor. Devolve quantos.
unsafe extern "thiscall" fn get_manager_of_year_candidates(this: usize, clubes: usize, n: u32) -> u8 {
    let semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let mut lim = [0u8; 8];
    semanas(0xae2c90, lim.as_mut_ptr(), 0x1c);
    let n = n as u16 as i16 as i32;
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(clubes, n as u32, 4, 0x870220);
    let (ld, la) = (i16::from_le_bytes([lim[0], lim[1]]), u16::from_le_bytes([lim[2], lim[3]]));
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let mut k = 0u8;
    for i in 0..n.max(0) as usize {
        if k as i8 >= 3 {
            break;
        }
        let tec = cpr10_d(cpr10_d(clubes + 4 * i) + 0xcf);
        if tec == 0 {
            continue;
        }
        let ta = cpr10_u(tec + 0x40);
        if !(la > ta || (la == ta && ld > cpr10_w(tec + 0x3e))) {
            continue;
        }
        let r = registro(this, tec, 1);
        if r != 0 && cpr10_w(r + 0x8e) != 0 {
            core::ptr::write_unaligned((clubes + 4 * k as usize) as *mut u32, cpr10_d(clubes + 4 * i) as u32);
            k += 1;
        }
    }
    k
}

/// Comparador: a original (com a forma do dia) e o estado relido pela regra (injury_status, sem rodar a 0x62c680 de
/// novo: a original já a rodou).
unsafe extern "thiscall" fn comparar_get_player_injury_status(this: usize, pessoa: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%)(this, pessoa);
    if pessoa != 0 && (cpr10_d(pessoa) as i32) < cpr10_d(0xae2364) as i32 {
        let n = injury_status(this, pessoa);
        PLACAR.registrar(r == n, || format!("get_player_injury_status(pessoa {:x}) original={} rust={}", pessoa, r, n));
    }
    r
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const CPR10_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// Comparador: sem a entrada (a 0x4cd6d0 acrescenta e pode realocar) só a original; com ela, lado a lado pelo sorteio,
/// as datas estáticas, o vetor das decisões e +0x1c (a cópia sem contar).
unsafe extern "thiscall" fn comparar_schedule_offer_decision(this: usize, id: i32, vaga: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(%%I1%%);
    let mut limite = [0u8; 8];
    injury_date_add_days(0xae2c90, 0xe, &mut limite);
    if find_type3_decision(this, id, &limite, false) == 0 {
        return o(this, id, vaga);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&CPR10_ESTATICAS);
    reg.push((this + 0x1c, 4));
    reg.push((cpr10_d(this + 0xc), cpr10_d(this + 0x14).wrapping_mul(14)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id, vaga),
        || crate::aleatorio::sem_contar(|| schedule_offer_decision(this, id, vaga)));
    PLACAR.registrar(igual, || format!("schedule_offer_decision(id {}, vaga {}) {:x?} x {:x?}", id, vaga, mo, mr));
}

/// Comparador: lado a lado pelo vetor dos clubes e pela data estática da 0x54cab0, e a devolução (os registros dos
/// técnicos que a 0x86f5b0 cria na original já existem para a cópia).
unsafe extern "thiscall" fn comparar_get_manager_of_year_candidates(this: usize, clubes: usize, n: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I2%%);
    let k = (n as u16 as i16).max(0) as usize;
    let reg = [(clubes, 4 * k), (0xae24c8, 8), (0xae24aa, 1)];
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, n), || get_manager_of_year_candidates(this, clubes, n));
    PLACAR.registrar(igual && r == c, || format!("get_manager_of_year_candidates({}) original={} rust={} {:x?} x {:x?}", k, r, c, mo, mr));
    r
}
