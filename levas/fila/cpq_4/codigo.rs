// ---------- %%LEVA%%a leva: os registros de carreira: a transferência, a eliminação e a não classificação (staff_records.cpp) ----------

unsafe fn cpq4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpq4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpq4_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpq4_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// O inteiro de 64 bits em `a` somado de `v` (estendido com sinal: cdq, add, adc).
unsafe fn staff_records_add_i64(a: usize, v: i32) {
    let x = core::ptr::read_unaligned(a as *const i64);
    core::ptr::write_unaligned(a as *mut i64, x.wrapping_add(v as i64));
}

unsafe fn put_i32(e: &mut [u8; 0x40], o: usize, v: i32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// A pessoa do id (0x6e bytes em [0xae23bc]).
unsafe fn cpq4_pessoa(id: usize) -> usize {
    cpq4_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A data do jogo copiada (0x417c10) em `a` (a cópia também vai para uma data local).
unsafe fn stamp_game_date(a: *mut u8) {
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(a, t.as_mut_ptr(), 0xae2c90);
}

/// Uma entrada de histórico na pilha com a data construída (0x54a110).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
unsafe fn history_entry() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    nova(e.as_mut_ptr());
    e
}

/// A entrada `e` no fim da lista da pessoa `id` no arquivo do histórico das pessoas (FILE_LLIST [this], 0x58e340; o
/// começo e o fim em [this+0xc] + id × 8). Sem `gravar` (a cópia pelo comparador), nada.
unsafe fn append_person_history_by_id(gravar: bool, this: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
    let p = cpq4_d(this + 0xc).wrapping_add(id.wrapping_mul(8));
    f(cpq4_d(this), p, p + 4, e.as_ptr());
}

/// O id do clube contratante da pessoa (+0x39), ou −1.
unsafe fn club_job_id(pessoa: usize) -> i32 {
    let c = cpq4_d(pessoa + 0x39);
    if c == 0 {
        -1
    } else {
        cpq4_i(c)
    }
}

unsafe fn staff_records_id_or_minus_one(p: usize) -> i32 {
    if p == 0 {
        -1
    } else {
        cpq4_i(p)
    }
}

/// O id da pessoa e os de +4, +8 e +0xc dela (+0x33 de cada) em `e` a partir de `o`.
unsafe fn put_person_keys(e: &mut [u8; 0x40], o: usize, p: usize) {
    put_i32(e, o, cpq4_i(p));
    for k in 0..3usize {
        put_i32(e, o + 4 + 4 * k, cpq4_i(cpq4_d(p + 4 + 4 * k) + 0x33));
    }
}

/// Os itens da lista da troca (LLIST em +4 da transferência, relida a cada passo; o cursor reposto antes), na ordem.
unsafe fn for_each_swap_item(t: usize, mut f: impl FnMut(usize)) {
    let l = cpq4_d(t + 4);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpq4_d(l + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(cpq4_d(t + 4));
    while e != 0 {
        f(e);
        e = proximo(cpq4_d(t + 4));
    }
}

/// O tipo do cargo é de jogador (0x53b620: 11, 12, 13 ou 15)?
unsafe fn staff_records_is_player_job(cargo: u8) -> bool {
    let f: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    f(cargo as u32) != 0 // ponytail: a original empurra o registro com o byte no al; a 0x53b620 lê só o byte
}

unsafe fn staff_records_manager_record(this: usize, tec: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    f(this, tec, 1)
}

/// Os lados da transferência: (pessoa, contrato da pessoa 0x4cf5a0, vendedor ou 0, comprador).
unsafe fn transfer_parties(t: usize) -> (usize, usize, usize, usize) {
    let pessoa = cpq4_pessoa(cpq4_d(t + 0xc));
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cpq4_d(0xae1050), pessoa);
    let vendedor = if c != 0 && cpq4_b(c + 0x35) & 0x3f != 5 {
        cpq4_d(0xae23b4).wrapping_add(cpq4_d(c + 4).wrapping_mul(0x245))
    } else {
        0
    };
    (pessoa, c, vendedor, cpq4_d(0xae23b4).wrapping_add(cpq4_d(t + 8).wrapping_mul(0x245)))
}

unsafe fn record_transfer(this: usize, t: usize, gravar: bool) {
    let (pessoa, c, vendedor, comprador) = transfer_parties(t);
    *(0xae1080 as *mut usize) = c;
    let mut e = history_entry();
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let valor = total(t, 1, 0, 0, 1);
    let com_troca = || (cpq4_d(t + 4) != 0) as i32;
    let tipo = cpq4_b(t + 0x11) as i8 as i32;
    if comprador != 0 && cpq4_d(comprador + 0xcf) != 0 {
        let tec = cpq4_d(comprador + 0xcf);
        let r = if staff_records_is_player_job(cpq4_b(t + 0x11)) { staff_records_manager_record(this, tec) } else { 0 };
        if r != 0 {
            crate::comparison::branch("cpq_4 compra");
            if valor > cpq4_i(r + 0x18) {
                crate::comparison::branch("cpq_4 maior compra");
                for k in 0..4usize {
                    cpq4_pi(r + 4 * k, if k == 0 { cpq4_i(pessoa) } else { cpq4_i(cpq4_d(pessoa + 4 * k) + 0x33) });
                }
                stamp_game_date((r + 0x10) as *mut u8);
                cpq4_pi(r + 0x18, valor);
                *((r + 0x1c) as *mut u8) = com_troca() as u8;
                cpq4_pi(r + 0x1d, staff_records_id_or_minus_one(vendedor));
                cpq4_pi(r + 0x21, cpq4_i(comprador));
            }
            cpq4_pw(r + 0xa0, cpq4_w(r + 0xa0).wrapping_add(1));
            staff_records_add_i64(r + 0xa4, valor);
            if cpq4_d(t + 4) != 0 {
                cpq4_pw(r + 0xa2, cpq4_w(r + 0xa2).wrapping_add(cpq4_w(cpq4_d(t + 4))));
                for_each_swap_item(t, |x| staff_records_add_i64(r + 0xac, cpq4_i(cpq4_pessoa(cpq4_d(x)) + 0x52)));
            }
        }
        // o histórico do técnico comprador: a compra e cada jogador dado na troca
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x64;
        put_i32(&mut e, 8, club_job_id(tec));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, valor);
        put_i32(&mut e, 0x21, com_troca());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(vendedor));
        put_i32(&mut e, 0x29, tipo);
        append_person_history_by_id(gravar, this, cpq4_d(tec), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x65;
            put_person_keys(&mut e, 0xd, cpq4_pessoa(cpq4_d(x)));
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, cpq4_i(vendedor));
            put_i32(&mut e, 0x29, tipo);
            append_person_history_by_id(gravar, this, cpq4_d(tec), &e);
        });
    }
    if vendedor != 0 && cpq4_d(vendedor + 0xcf) != 0 {
        let tec = cpq4_d(vendedor + 0xcf);
        let r = if staff_records_is_player_job(cpq4_b(pessoa + 0x3d)) { staff_records_manager_record(this, tec) } else { 0 };
        if r != 0 {
            crate::comparison::branch("cpq_4 venda");
            if valor > cpq4_i(r + 0x3d) {
                for k in 0..4usize {
                    cpq4_pi(r + 0x25 + 4 * k, if k == 0 { cpq4_i(pessoa) } else { cpq4_i(cpq4_d(pessoa + 4 * k) + 0x33) });
                }
                stamp_game_date((r + 0x35) as *mut u8);
                cpq4_pi(r + 0x3d, valor);
                *((r + 0x41) as *mut u8) = com_troca() as u8;
                cpq4_pi(r + 0x42, cpq4_i(vendedor));
                cpq4_pi(r + 0x46, staff_records_id_or_minus_one(comprador));
            }
            cpq4_pw(r + 0xa2, cpq4_w(r + 0xa2).wrapping_add(1));
            staff_records_add_i64(r + 0xac, valor);
            if cpq4_d(t + 4) != 0 {
                cpq4_pw(r + 0xa0, cpq4_w(r + 0xa0).wrapping_add(cpq4_w(cpq4_d(t + 4))));
                for_each_swap_item(t, |x| staff_records_add_i64(r + 0xa4, cpq4_i(cpq4_pessoa(cpq4_d(x)) + 0x52)));
            }
        }
        // o histórico do técnico vendedor: a venda e cada jogador recebido na troca
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x65;
        put_i32(&mut e, 8, club_job_id(tec));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, valor);
        put_i32(&mut e, 0x21, com_troca());
        put_i32(&mut e, 0x25, staff_records_id_or_minus_one(comprador));
        put_i32(&mut e, 0x29, tipo);
        append_person_history_by_id(gravar, this, cpq4_d(tec), &e);
        for_each_swap_item(t, |x| {
            e[0xc] = 0x64;
            put_person_keys(&mut e, 0xd, cpq4_pessoa(cpq4_d(x)));
            put_i32(&mut e, 0x1d, 0);
            put_i32(&mut e, 0x21, 1);
            put_i32(&mut e, 0x25, cpq4_i(comprador));
            put_i32(&mut e, 0x29, 0xb);
            append_person_history_by_id(gravar, this, cpq4_d(tec), &e);
        });
    }
    // o histórico do jogador transferido e o de cada jogador da troca
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x66;
    put_i32(&mut e, 8, club_job_id(pessoa));
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comprador));
    put_i32(&mut e, 0x11, valor);
    put_i32(&mut e, 0x19, tipo);
    put_i32(&mut e, 0x15, com_troca());
    if pessoa != 0 {
        append_person_history_by_id(gravar, this, cpq4_d(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
    for_each_swap_item(t, |x| {
        let q = cpq4_pessoa(cpq4_d(x));
        put_i32(&mut e, 8, cpq4_i(cpq4_d(q + 0x39)));
        put_i32(&mut e, 0x11, 0);
        put_i32(&mut e, 0xd, cpq4_i(vendedor));
        put_i32(&mut e, 0x15, 1);
        put_i32(&mut e, 0x19, cpq4_b(x + 6) as i8 as i32);
        if q != 0 {
            append_person_history_by_id(gravar, this, cpq4_d(q), &e);
        } else {
            show_error_at(0xa83b78, 0x1010);
        }
    });
}

/// { 0x86cd70 } (thiscall, ret 4, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_transfer_records(transf)`:
/// o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o vendedor (fora do estado 5); o valor total da oferta
/// (0x8e7a70 com 1, 0, 0, 1). O técnico do comprador, sendo de jogador o tipo da oferta (+0x11, 0x53b620): no registro
/// (0x86f5b0) a maior compra (+0..+0x24 se o valor passa +0x18), as compras (+0xa0), o gasto (+0xa4, 64 bits), os
/// jogadores dados (+0xa2) e o valor deles (+0x52 de cada, em +0xac). O técnico do vendedor, sendo de jogador o cargo
/// da pessoa (+0x3d): a maior venda (+0x25..+0x49 se passa +0x3d), as vendas (+0xa2), a receita (+0xac), os recebidos
/// (+0xa0) e o valor deles (+0xa4). Os históricos (tipos 0x64 compra, 0x65 venda, 0x66 transferência) do técnico do
/// comprador, do vendedor, da pessoa e dos jogadores da troca, no arquivo (0x58e340).
unsafe extern "thiscall" fn add_transfer_records(this: usize, t: usize) {
    record_transfer(this, t, true)
}

/// Comparador: lado a lado pela global [0xae1080], o estado do sorteador (o valor da oferta pode pedir a avaliação), a
/// lista da troca (o cursor) e os registros dos dois técnicos (criados antes só quando a original os pede); a cópia
/// sem os históricos em arquivo, que a original já gravou, e sem contar os sorteios.
unsafe extern "thiscall" fn compare_add_transfer_records(this: usize, t: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let (pessoa, _, vendedor, comprador) = transfer_parties(t);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((0xae1080, 4));
    if cpq4_d(t + 4) != 0 {
        reg.push((cpq4_d(t + 4), 0xe));
    }
    for (clube, cargo) in [(comprador, cpq4_b(t + 0x11)), (vendedor, cpq4_b(pessoa + 0x3d))] {
        if clube != 0 && cpq4_d(clube + 0xcf) != 0 && staff_records_is_player_job(cargo) {
            let r = staff_records_manager_record(this, cpq4_d(clube + 0xcf));
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, t),
        || crate::rng::without_counting(|| record_transfer(this, t, false)));
    SCOREBOARD.record(igual, || format!("add_transfer_records(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// A entrada da pessoa: o clube (+8), o tipo (+0xc), a competição ou −1 (+0xd) e a fase com sinal (+0x11).
unsafe fn stage_history_entry(clube: usize, tipo: u8, comp: usize, fase: u32) -> [u8; 0x40] {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    put_i32(&mut e, 8, cpq4_i(clube));
    e[0xc] = tipo;
    put_i32(&mut e, 0xd, staff_records_id_or_minus_one(comp));
    put_i32(&mut e, 0x11, fase as u16 as i16 as i32);
    e
}

/// Na competição de seleção guardada (0x76d940 de [0xb74318]), as convocações da seleção para ela acabam (0x774f90).
unsafe fn end_national_duty(clube: usize, comp: usize) {
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
    if especial(cpq4_d(0xb74318), comp) != 0 {
        crate::comparison::branch("cpq_4 fim das convocacoes");
        let encerrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x774f90usize);
        encerrar(cpq4_d(0xb74318), clube, comp);
    }
}

/// { 0x86c000 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_knocked_out_record(clube,
/// comp, fase, rodada, _)`: o histórico do técnico (+0xcf; tipo 4: clube, competição ou −1, a fase com sinal e a rodada
/// base 0x4b0870) e o fim das convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x73b).
unsafe extern "thiscall" fn add_knocked_out_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, _x: u32) {
    if clube == 0 {
        return show_error_at(0xa83b78, 0x73b);
    }
    crate::comparison::branch("cpq_4 eliminado");
    let mut e = stage_history_entry(clube, 4, comp, fase);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0870usize);
    put_i32(&mut e, 0x15, base(rodada) as u16 as i16 as i32);
    if cpq4_d(clube + 0xcf) != 0 {
        append_person_history_by_id(true, this, cpq4_d(cpq4_d(clube + 0xcf)), &e);
    }
    end_national_duty(clube, comp);
}

// ponytail: o histórico vai para o arquivo e as convocações acabam (não rodam duas vezes) e a cópia sem eles não mexe
// em nada: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn compare_add_knocked_out_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I1%%)(this, clube, comp, fase, rodada, x)
}

/// { 0x86c1d0 } (thiscall, ret 0x10, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_failed_to_qualify_record(
/// clube, comp, fase, _)`: o histórico do técnico (tipo 7: clube, competição ou −1, a fase com sinal) e o fim das
/// convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x768).
unsafe extern "thiscall" fn add_failed_to_qualify_record(this: usize, clube: usize, comp: usize, fase: u32, _x: u32) {
    if clube == 0 {
        return show_error_at(0xa83b78, 0x768);
    }
    crate::comparison::branch("cpq_4 nao classificado");
    let e = stage_history_entry(clube, 7, comp, fase);
    if cpq4_d(clube + 0xcf) != 0 {
        append_person_history_by_id(true, this, cpq4_d(cpq4_d(clube + 0xcf)), &e);
    }
    end_national_duty(clube, comp);
}

// ponytail: como a eliminação: só a original.
unsafe extern "thiscall" fn compare_add_failed_to_qualify_record(this: usize, clube: usize, comp: usize, fase: u32, x: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%)(this, clube, comp, fase, x)
}
