// ---------- %%LEVA%%a leva: a satisfação do jogador com o contrato, a notícia de que ele quer contrato novo e a do pedido de transferência retirado (staff_contracts.cpp, contract_manager.cpp) ----------

unsafe fn cpw1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw1_clube(id: usize) -> usize {
    cpw1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw1_pessoa(id: usize) -> usize {
    cpw1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O valor de um contrato para a pessoa (STAFF_CONTRACT::calc_contract_value 0x866830: thiscall(contrato, pessoa,
/// oferta), ret 8).
unsafe fn calc_contract_value(contrato: usize, pessoa: usize, oferta: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x866830usize);
    f(contrato, pessoa, oferta)
}

/// { 0x866620 } (thiscall, ret 4, staff_contracts.cpp, quadro SEH sem destrutor): `STAFF_CONTRACT::
/// get_contract_satisfaction(pessoa)`, pelo passe diário do pessoal (0x5ce2d0, com sorteio(50) zero) e pela tela do
/// contrato: 1 a 3. Sem pessoa, 1. A situação no elenco que ela merece (0x4ca470) vale se for menor que a do contrato
/// (+0x4f >> 4) e não for 0 nem 7; senão, a do contrato. O contrato desejado (0x85fc50 num contrato local, com o clube
/// do contrato, −1, a situação e 0xb) e a razão 100 × valor do contrato atual (0x866830, em f32) / valor do desejado
/// (valor zero: razão 0). Contratado há menos de 15 × adaptabilidade (+0x59) + 100 dias (de +0x25, teto 500): razão +
/// 50 + 7,5 × adaptabilidade − dias / 2. Razão abaixo de 50: 1; abaixo de 80: 3; senão 2.
unsafe extern "thiscall" fn get_contract_satisfaction(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 1;
    }
    let categoria: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) = core::mem::transmute(0x85fc50usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // o contrato local (0x50 bytes) como a original o deixa antes da 0x85fc50; o resto ela preenche
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    data_padrao(l + 0x25);
    data_padrao(l + 0x2d);
    core::ptr::write_unaligned((l + 4) as *mut i32, -1);
    core::ptr::write_unaligned((l + 0x4a) as *mut i32, -1);
    let merece = categoria(pessoa, this);
    let atual = cpw1_b(this + 0x4f) >> 4;
    // ponytail: a original empurra o dword inteiro da vaga onde guardou o byte (no primeiro caso, o resto é o ponteiro
    // da pessoa; no segundo, sobra de pilha); a 0x85fc50 só lê o byte.
    let situacao = if (merece as i8) < (atual as i8) && merece != 0 && merece != 7 {
        (pessoa as u32 & !0xff) | merece as u32
    } else {
        atual as u32
    };
    desejado(l, pessoa, cpw1_clube(cpw1_d(this + 4)), -1, situacao, 0xb, 0, 0);
    let v1 = calc_contract_value(this, pessoa, l) as f32;
    let v2 = calc_contract_value(l, pessoa, l) as f64;
    let mut razao = if v2 == read_f32_at(0x968970) as f64 {
        read_f32_at(0x968970) as f64
    } else {
        v1 as f64 * read_f32_at(0x968960) as f64 / v2
    };
    let anos = (cpw1_w(0xae2c92) as u16 as i32).wrapping_sub(cpw1_w(this + 0x27) as u16 as i32);
    let dias = anos.wrapping_mul(365).wrapping_sub(cpw1_w(this + 0x25) as i32).wrapping_add(cpw1_w(0xae2c90) as i32).clamp(0, 500);
    let adapt = cpw1_b(pessoa + 0x59) as i8 as i32;
    if dias < adapt.wrapping_mul(15).wrapping_add(100) {
        crate::comparison::branch("cpw_1 recently signed");
        razao = razao + read_f32_at(0x969560) as f64 + adapt as f64 * read_f64_at(0x968eb0) - (dias / 2) as f64; // cpw1: o desconto do recém-contratado
    }
    crate::comparison::branch("cpw_1 satisfaction evaluated");
    if !(razao >= read_f32_at(0x969560) as f64) { // cpw1: o limite da insatisfação
        return 1;
    }
    if razao < read_f32_at(0x96ff70) as f64 { // cpw1: o limite da satisfação
        3
    } else {
        2
    }
}

/// Comparador: a original e a cópia a partir do mesmo estado (o sorteio, o último contrato pedido [0xae1080] que a
/// 0x85fc50 grava e o contrato); a devolução e a memória.
unsafe extern "thiscall" fn compare_get_contract_satisfaction(this: usize, pessoa: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I0%%);
    let mut regioes = vec![(0xae1080usize, 4usize), (this, 0x50)];
    regioes.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regioes,
        || o(this, pessoa),
        || crate::rng::without_counting(|| get_contract_satisfaction(this, pessoa)),
    );
    SCOREBOARD.record(igual && r == n, || {
        format!("get_contract_satisfaction(pessoa {:#x}) original={} {:x?} rust={} {:x?}", if pessoa != 0 { cpw1_d(pessoa) } else { 0 }, r, mo, n, mr)
    });
    r
}

/// Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
unsafe fn set_news_staff_slots(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, 2, cpw1_d(p) as u32);
    gravar(n, 3, cpw1_d(cpw1_d(p + 4) + 0x33) as u32);
    gravar(n, 4, cpw1_d(cpw1_d(p + 8) + 0x33) as u32);
    gravar(n, 5, cpw1_d(cpw1_d(p + 0xc) + 0x33) as u32);
}

/// { 0x4d9f80 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_wants_new_contract_news(contrato)`, pelo passe diário do pessoal (0x5ce2d0). Sem contrato: a caixa "Error"
/// (linha 0x2d91). Pessoa na lista de transferências (0x8c1870), com ofertas (0x8c5cf0) ou oferecida em troca (0x8c5e40):
/// nada. Senão a notícia 0xbc2 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, a situação no elenco (+0x4f
/// >> 4), 0, o fim do contrato (+0x2d dia com sinal, +0x2f ano), o salário (+0xc) e, na vaga 11, 1 se a adaptabilidade
/// (+0x59) for menor que 10, a +0x5b menor que 5 e a reputação da ficha (+0xd) passar de 4750.
unsafe extern "thiscall" fn post_wants_new_contract_news(this: usize, contrato: usize) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2d91);
    }
    let in_list: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8c1870usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let clube = cpw1_clube(cpw1_d(contrato + 4));
    let p = cpw1_pessoa(cpw1_d(contrato));
    let tm = cpw1_d(0xde1f54);
    if in_list(tm, p) != 0 || ofertas(tm, p, 0, 0) != 0 || trocas(tm, p, 0) != 0 {
        return;
    }
    crate::comparison::branch("cpw_1 new contract news");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc2, 0);
    gravar(n, 0, cpw1_d(clube) as u32);
    set_news_staff_slots(n, p);
    gravar(n, 6, (cpw1_b(contrato + 0x4f) >> 4) as u32);
    gravar(n, 7, 0);
    gravar(n, 8, cpw1_w(contrato + 0x2d) as i32 as u32);
    gravar(n, 9, cpw1_w(contrato + 0x2f) as u16 as u32);
    gravar(n, 10, cpw1_d(contrato + 0xc) as u32);
    let ficha = cpw1_d(p + 0x61);
    let reputado = (cpw1_b(p + 0x59) as i8) < 0xa && (cpw1_b(p + 0x5b) as i8) < 5 && ficha != 0 && cpw1_w(ficha + 0xd) > 0x128e;
    gravar(n, 11, reputado as u32);
    entregar(cpw1_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_wants_new_contract_news(this: usize, contrato: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, contrato)
}

/// { 0x4d9460 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_transfer_request_withdrawn_news(contrato, motivo)`. Sem contrato: a caixa "Error" (linha 0x2ba6). Senão a
/// notícia 0xbc0 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, o motivo e, na vaga 7, 1 se +0x3f do
/// contrato passar de 1.
unsafe extern "thiscall" fn post_transfer_request_withdrawn_news(this: usize, contrato: usize, motivo: u32) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2ba6);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cpw_1 request withdrawn");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc0, 0);
    let clube = cpw1_clube(cpw1_d(contrato + 4));
    let p = cpw1_pessoa(cpw1_d(contrato));
    gravar(n, 0, cpw1_d(clube) as u32);
    set_news_staff_slots(n, p);
    gravar(n, 6, motivo);
    gravar(n, 7, ((cpw1_b(contrato + 0x3f) as i8) > 1) as u32);
    entregar(cpw1_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_transfer_request_withdrawn_news(this: usize, contrato: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, contrato, motivo)
}
