// ---------- %%LEVA%%a leva: tirar as ofertas guardadas da pessoa, guardar as suspensões do emprestado e preencher a vaga da comissão ----------

unsafe fn cpi5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x4cf560 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_held_offers(pessoa)`: da lista das
/// ofertas guardadas (+0x38; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item cujo +0 é o id `pessoa`
/// (0x68d1b0 percorre). Sem lista: nada.
unsafe extern "thiscall" fn remove_held_offers(this: usize, pessoa: u32) {
    let l = cpi5_d(this + 0x38);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpi5_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut e = proximo(l);
    while e != 0 {
        if cpi5_d(e) as u32 == pessoa { // a oferta da pessoa
            tirar(l, e);
        }
        e = proximo(l);
    }
}

/// A suspensão do tipo `t` (vetor de tipos [this], 4 bytes cada) vale no clube do empréstimo? Pela competição do tipo
/// (+0x17, −1 = nenhuma) igual à nação do clube (+0x53, id em +0); sem competição, pelo +0x1b (−1: vale) igual ao
/// [+0x71] da nação. Clube sem nação: não vale.
unsafe fn suspension_valid_at_club(this: usize, t: i8, clube: usize) -> bool {
    let tipo = cpi5_d(cpi5_d(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nacao = cpi5_d(clube + 0x53);
    let comp = cpi5_d(tipo + 0x17) as i32;
    if comp != -1 {
        return nacao != 0 && cpi5_d(nacao) as i32 == comp;
    }
    let x = cpi5_d(tipo + 0x1b) as i32;
    if x == -1 {
        return true;
    }
    if nacao == 0 {
        return false;
    }
    let m = cpi5_d(nacao + 0x71);
    m != 0 && cpi5_d(m) as i32 == x // a do +0x71 da nação
}

/// O clube do empréstimo (+4 da ficha de contrato, clubes de 0x245 bytes em [0xae23b4]) e o registro de suspensões
/// do jogador (29 bytes em [this+0x14]); as vagas (5 bytes: tipo e o resto) que não valem lá, menos a 4ª (índice 3,
/// que o original pula).
unsafe fn suspensions_to_store(this: usize, ficha: usize, jogador: usize) -> (usize, Vec<u8>) {
    let clube = cpi5_d(0xae23b4).wrapping_add(cpi5_d(ficha + 4).wrapping_mul(0x245));
    let reg = cpi5_d(this + 0x14).wrapping_add(cpi5_d(jogador).wrapping_mul(29));
    let mut v = Vec::new();
    for k in 0..4u8 {
        let t = *((reg + 5 * k as usize) as *const i8);
        if k == 3 || t == -1 {
            continue;
        }
        if !suspension_valid_at_club(this, t, clube) {
            v.push(k);
        }
    }
    (reg, v)
}

/// { 0x5563f0 } (thiscall, ret 4, discipline.cpp): `Discipline::hold_bans_for_loan(jogador)`: com a ficha de contrato
/// do jogador (0x4cf6d0 sobre [0xae1050]), cada suspensão do registro dele (vagas 0 a 2) que não vale no clube do
/// empréstimo vai para a lista das guardadas (+0x1c, 0x68d0d0 com 10 bytes: o id, a vaga e os 5 bytes dela), a vaga
/// fica −1 e o contador +0x1c do registro sobe. Sem ficha: a original (caixa "Error").
unsafe extern "thiscall" fn hold_bans_for_loan(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(0x4cf6d0usize);
    let ficha = ficha_de(cpi5_d(0xae1050), jogador as *const i32);
    if ficha == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, jogador); // caixa: só a original
    }
    let (reg, vagas) = suspensions_to_store(this, ficha, jogador);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    for k in vagas {
        let vaga = reg + 5 * k as usize;
        let mut r = [0u8; 10];
        r[..4].copy_from_slice(&(cpi5_d(jogador) as u32).to_le_bytes());
        r[4] = k; // a vaga
        r[5..].copy_from_slice(core::slice::from_raw_parts(vaga as *const u8, 5));
        acrescentar(this + 0x1c, r.as_ptr(), 10);
        core::ptr::write_unaligned(vaga as *mut u32, u32::MAX);
        *((vaga + 4) as *mut u8) = 0xff;
        *((reg + 0x1c) as *mut u8) = (*((reg + 0x1c) as *const u8)).wrapping_add(1);
    }
}

/// O cargo que a vaga `cargo` (byte com sinal) procura: 6, 8, 9 e 10 o mesmo; 13 procura 6 e 15 procura 8 (o byte do
/// argumento é trocado); os outros, nenhum.
fn job_sought_by_slot(cargo: u32) -> Option<u32> {
    match cargo as u8 as i8 {
        6 | 8 | 9 | 10 => Some(cargo),
        13 => Some((cargo & !0xff) | 6),
        15 => Some((cargo & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// A primeira pessoa (de [0xae23bc], 0x6e bytes, [0xae2364] delas) sem clube (+0x39), no cargo (+0x3d), com registro
/// (+0x69) e da nação (+0x1a) do clube (+0x53).
unsafe fn first_unemployed_staff(clube: usize, cargo: u8) -> Option<usize> {
    let n = cpi5_d(0xae2364) as i32;
    let base = cpi5_d(0xae23bc);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        cpi5_d(p + 0x39) == 0 && *((p + 0x3d) as *const u8) == cargo && cpi5_d(p + 0x69) != 0 && cpi5_d(p + 0x1a) == cpi5_d(clube + 0x53)
    })
}

/// { 0x8cb6d0 } (stdcall, ret 8, transfer_manager.cpp): `fill_vacated_staff_job(clube, cargo)`: a vaga da comissão
/// que ficou livre é preenchida pela primeira pessoa desempregada da nação do clube no cargo procurado (6, 8, 9 e 10;
/// 13 procura 6, 15 procura 8): 0x53bd10 com o cargo (o dword do argumento), a data de hoje [0xae2c90], gravar e sem
/// reduzir. Clube nulo: a original (caixa "Error").
unsafe extern "stdcall" fn fill_vacated_staff_job(clube: usize, cargo: u32) {
    if clube == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize, u32)>(%%I2%%)(clube, cargo); // caixa: só a original
    }
    let Some(c) = job_sought_by_slot(cargo) else { return };
    if let Some(p) = first_unemployed_staff(clube, c as u8) {
        let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
        incluir(clube, p, c, 0xae2c90, 1, 0);
    }
}

/// Comparador: o plano antes (a 0x68d1f0 libera o item tirado, que não pode mais ser lido); a original tira; confere
/// que ficaram os itens de antes menos os da pessoa, na mesma ordem.
unsafe extern "thiscall" fn compare_remove_held_offers(this: usize, pessoa: u32) {
    let l = cpi5_d(this + 0x38);
    let antes: Vec<usize> = if l != 0 { list_items_capped(l) } else { Vec::new() };
    let esperado: Vec<usize> = antes.iter().copied().filter(|&e| cpi5_d(e) as u32 != pessoa).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, pessoa);
    let depois: Vec<usize> = if l != 0 { list_items_capped(l) } else { Vec::new() };
    SCOREBOARD.record(depois == esperado, || format!("remove_held_offers({:x}) {} itens, esperava {}, ficaram {}", pessoa, antes.len(), esperado.len(), depois.len()));
}

/// Comparador: o plano da cópia (as vagas que não valem no clube do empréstimo) antes; a original guarda; confere o
/// registro do jogador: as vagas do plano em −1, as outras iguais e o contador somado.
unsafe extern "thiscall" fn compare_hold_bans_for_loan(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(0x4cf6d0usize);
    let ficha = ficha_de(cpi5_d(0xae1050), jogador as *const i32);
    if ficha == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, jogador);
    }
    let (reg, vagas) = suspensions_to_store(this, ficha, jogador);
    let mut esperado = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    for &k in &vagas {
        esperado[5 * k as usize..5 * k as usize + 5].fill(0xff);
    }
    esperado[0x1c] = esperado[0x1c].wrapping_add(vagas.len() as u8);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, jogador);
    let depois = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    SCOREBOARD.record(depois == esperado, || format!("hold_bans_for_loan({:x}) vagas {:?} {:x?} x {:x?}", jogador, vagas, depois, esperado));
}

/// Comparador: a escolha da cópia antes; a original preenche; a pessoa escolhida tem de estar agora no clube (+0x39).
unsafe extern "stdcall" fn compare_fill_vacated_staff_job(clube: usize, cargo: u32) {
    let escolhida = if clube != 0 { job_sought_by_slot(cargo).and_then(|c| first_unemployed_staff(clube, c as u8)) } else { None };
    original_fn::<unsafe extern "stdcall" fn(usize, u32)>(%%I2%%)(clube, cargo);
    if let Some(p) = escolhida {
        SCOREBOARD.record(cpi5_d(p + 0x39) == clube, || format!("fill_vacated_staff_job({:x}, {}) a pessoa {:x} ficou em {:x}", clube, cargo as u8, p, cpi5_d(p + 0x39)));
    } else {
        crate::comparison::branch("fill_vacated_staff_job sem ninguem");
    }
}
