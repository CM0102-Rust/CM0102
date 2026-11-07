// ---------- 257a leva: as cláusulas do contrato: a cláusula de rescisão ativa, o período protegido da FIFA e o tipo de bônus permitido (staff_contracts.cpp) ----------



/// { 0x867b50 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::has_active_release_clause()`: 1 com a cláusula +0x1c
/// = 2, a +0x1d = 1 ou a +0x1f = 2; senão 0. Pelo dia dos contratos 0x4c7ac0, pelo valor do jogador 0x4d0c60, pelo preço
/// pedido 0x4d2070, pela resposta prevista do vendedor 0x84af00, pelo vendedor que tem de aceitar 0x8cd240 e pelo lance
/// 0x8e5d60. Chamada a cada contrato: sem contador.
unsafe extern "thiscall" fn clausula_de_rescisao_ativa(c: usize) -> u32 {
    if read_u8(c + 0x1c) == 2 || read_u8(c + 0x1d) == 1 || read_u8(c + 0x1f) == 2 {
        return 1; // cpu4: com a clausula ativa
    }
    0
}

unsafe extern "thiscall" fn comparar_clausula_de_rescisao_ativa(c: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1203)(c);
    let n = clausula_de_rescisao_ativa(c);
    PLACAR.registrar(r == n, || format!("clausula_de_rescisao_ativa({:x}) original={} rust={}", c, r, n));
    r
}

/// { 0x868210 } (thiscall, ret 0xc, staff_contracts.cpp, quadro SEH só da data local):
/// `STAFF_CONTRACT::check_fifa_protected_period(_, &no_limite, &anos)`: zera `no_limite` (se não nulo). Sem as regras
/// novas da FIFA ([0xae233c] ≠ 1), 1. Contrato (data +0x25) anterior à data das regras ([0xae2330]: ano sem sinal, dia
/// com sinal), 1. Senão o período protegido: 3 anos com a idade da pessoa ([0xae23bc] + 110 × [+0]) na data do contrato
/// (0x53ab90) abaixo de 28, senão 2, em `anos` (se não nulo); a duração de hoje até a data (0x54cc30, duas vezes, como a
/// original) com menos anos que o período, 1; com os anos exatos e 0 dias, `no_limite` = 1; 0. Pelo dia dos contratos
/// 0x4c7ac0, pelo lance 0x8e5d60, pela troca com compensação 0x84b6f0 e por telas. Sem contador.
unsafe extern "thiscall" fn periodo_protegido(c: usize, _x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    if !no_limite.is_null() {
        *no_limite = 0;
    }
    if read_u8(0xae233c) != 1 {
        return 1; // cpu4: sem as regras novas
    }
    let (rdia, rano) = (*(0xae2330 as *const i16), *(0xae2332 as *const u16));
    let d = c + 0x25;
    let (cdia, cano) = (core::ptr::read_unaligned(d as *const i16), core::ptr::read_unaligned((d + 2) as *const u16));
    if rano > cano || (rano == cano && rdia > cdia) {
        return 1;
    }
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let idade: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53ab90usize);
    let (d0, d1) = (read_usize(d) as u32, read_usize(d + 4) as u32);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(c).wrapping_mul(110));
    let limite: u8 = if (idade(pessoa, d) as u8 as i8) < 0x1c { 3 } else { 2 }; // cpu4: o periodo pela idade
    if !anos.is_null() {
        *anos = limite;
    }
    let mut l = [0u8; 8];
    let p = diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    core::ptr::copy_nonoverlapping(p as *const u8, l.as_mut_ptr(), 8);
    let a = u16::from_le_bytes([l[2], l[3]]) as i32;
    if a < limite as i32 {
        return 1; // cpu4: dentro do periodo
    }
    if !no_limite.is_null() && a == limite as i32 && u16::from_le_bytes([l[0], l[1]]) == 0 {
        *no_limite = 1;
    }
    0
}

unsafe extern "thiscall" fn comparar_periodo_protegido(c: usize, x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    // as saídas como estavam antes: a cópia parte delas (a original só grava `anos` no período)
    let (mut nl, mut na) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) -> u32>(1204)(c, x, no_limite, anos);
    let (ol, oa) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let n = periodo_protegido(
        c,
        x,
        if no_limite.is_null() { core::ptr::null_mut() } else { &mut nl },
        if anos.is_null() { core::ptr::null_mut() } else { &mut na },
    );
    let ok = r == n && (no_limite.is_null() || ol == nl) && (anos.is_null() || oa == na);
    PLACAR.registrar(ok, || format!("periodo_protegido({:x}) original={} {} {} rust={} {} {}", c, r, ol, oa, n, nl, na));
    r
}

/// { 0x867aa0 } (cdecl, staff_contracts.cpp): `staff_contract_is_bonus_type_allowed(pessoa, tipo)`: sem a ficha de
/// jogador (+0x61), 0; goleiro (+0xf da ficha = 0x14), 1 só para o tipo 2; senão 1 para os tipos que não são 2. Pela
/// tela do contrato.
unsafe extern "C" fn tipo_de_bonus_permitido(pessoa: usize, tipo: u32) -> u32 {
    let f = read_usize(pessoa + 0x61);
    if f == 0 {
        return 0;
    }
    if read_u8(f + 0xf) == 0x14 {
        return (tipo as u8 == 2) as u32;
    }
    (tipo as u8 != 2) as u32
}

unsafe extern "C" fn comparar_tipo_de_bonus_permitido(pessoa: usize, tipo: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u32>(1205)(pessoa, tipo);
    let n = tipo_de_bonus_permitido(pessoa, tipo);
    PLACAR.registrar(r == n, || format!("tipo_de_bonus_permitido({:x}, {}) original={} rust={}", pessoa, tipo as u8, r, n));
    r
}

// ---------- 248a leva: o contrato da comissão: montar a proposta pelo contrato atual, zerar a moral do contrato novo e a faixa do vencimento (staff_contracts.cpp) ----------


unsafe fn cpl5_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}



/// A parte comum da 0x866ee0 e da 0x85e590 (inline nesta): zera a felicidade no contrato (+0x44), o estado +0x4e e as queixas (+0x45),
/// limpa os vínculos de opinião da pessoa do contrato (0x549c90 com o registro [0xae23bc] + id × 0x6e), zera +0x40,
/// põe 4 em +0x4e, zera a felicidade (+0x3f) e +0x3e; com o clube e a nação dele (+0x53), o nível do idioma da nação
/// (+0x77) para a pessoa (0x7c6830 em [0xb74350], sem o contrato; no máximo 10) nos 4 bits baixos de +0x4f.
unsafe fn cpl5_zerar_moral(this: usize, pessoa: usize, clube: usize) {
    crate::comparacao::ramo("cpl_5 moral zerada");
    write_u8(this + 0x44, 0);
    write_u8(this + 0x4e, 0);
    cpl5_por(this + 0x45, 0);
    let limpar: unsafe extern "C" fn(usize) = core::mem::transmute(0x549c90usize);
    limpar(read_usize(0xae23bc).wrapping_add(read_usize(this).wrapping_mul(0x6e)));
    cpl5_por(this + 0x40, 0);
    write_u8(this + 0x4e, 4);
    write_u8(this + 0x3f, 0); // a felicidade
    write_u8(this + 0x3e, 0);
    if clube != 0 && read_usize(clube + 0x53) != 0 {
        let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x7c6830usize);
        let r = fluencia(read_usize(0xb74350), pessoa, clube, read_u8(read_usize(clube + 0x53) + 0x77) as u32, 0) as i8;
        let c = if r > 10 { 10 } else { r } as u8;
        let b = read_u8(this + 0x4f);
        write_u8(this + 0x4f, (b & 0xf0) | (c & 0xf));
    }
}

/// { 0x866ee0 } (thiscall, ret 8, staff_contracts.cpp): `STAFF_CONTRACT::reset_morale_state(pessoa, clube)`: o
/// `cpl5_zerar_moral`. Pela assinatura do contrato (CONTRACT_MANAGER::sign_contract 0x4cf720, nos termos que
/// depois são copiados para o contrato guardado).
unsafe extern "thiscall" fn zerar_moral_do_contrato(this: usize, pessoa: usize, clube: usize) {
    cpl5_zerar_moral(this, pessoa, clube);
}

/// { 0x85e590 } (thiscall, ret 0x18, staff_contracts.cpp): `STAFF_CONTRACT::init_from_current_contract(pessoa, clube,
/// fim, percentual, sem_acesso, multa)`: a proposta de transferência ou empréstimo pelo contrato atual da pessoa
/// (0x4cf5a0 em [0xae1050]; sem contrato, caixa 0xf5 e nada): id da pessoa +0, clube +4, 0 em +8, salário +0xc =
/// salário atual × percentual (com sinal) / 100, a data de hoje (0xae2c90) em +0x25, o fim em +0x2d, os bits
/// +0x35 (& 0xc6 | 6), o salário também em +0x36, 0 em +0x3a e +0x49, −1 em +0x4a, o `cpl5_zerar_moral`, 0 em
/// +0x3b (16 bits), +0x1c..=+0x20 e +0x21, −1 em +0x10/+0x14/+0x18 e as cláusulas: +0x1c sem acesso, +0x1d multa
/// mínima. As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn contrato_pelo_atual(this: usize, pessoa: usize, clube: usize, fim: usize, percentual: u32, sem_acesso: u32, multa: u32) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(read_usize(0xae1050), pessoa);
    if c == 0 {
        aviso_em(0xa8360c, 0xf5);
        return;
    }
    crate::comparacao::ramo("cpl_5 contrato pelo atual");
    cpl5_por(this, read_usize(pessoa));
    cpl5_por(this + 8, 0);
    cpl5_por(this + 4, read_usize(clube));
    let salario = (read_usize(c + 0xc) as i32).wrapping_mul(percentual as u8 as i8 as i32) / 100; // o percentual do salário
    cpl5_por(this + 0xc, salario as usize);
    core::ptr::write_unaligned((this + 0x25) as *mut [u8; 8], core::ptr::read_unaligned(0xae2c90 as *const [u8; 8])); // hoje
    core::ptr::write_unaligned((this + 0x2d) as *mut [u8; 8], core::ptr::read_unaligned(fim as *const [u8; 8])); // o fim do contrato
    let b = (read_u8(this + 0x35) & 0xc6) | 6;
    cpl5_por(this + 0x36, read_usize(this + 0xc));
    write_u8(this + 0x35, b);
    write_u8(this + 0x3a, 0);
    write_u8(this + 0x49, 0);
    cpl5_por(this + 0x4a, usize::MAX);
    cpl5_zerar_moral(this, pessoa, clube);
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    for k in 0x1c..=0x20 {
        write_u8(this + k, 0);
    }
    cpl5_por(this + 0x10, usize::MAX);
    cpl5_por(this + 0x14, usize::MAX);
    cpl5_por(this + 0x18, usize::MAX);
    cpl5_por(this + 0x21, 0);
    write_u8(this + 0x1c, sem_acesso as u8);
    write_u8(this + 0x1d, multa as u8);
}

/// { 0x867d30 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::get_expiry_band()`: a faixa 0..=14 do fim do contrato
/// (+0x2d) contra as datas de hoje (0xae2c90) e as 16 datas do CONTRACT_MANAGER (0xae1010..0xae1090, montadas na
/// criação): tipo (+0x35 & 0x3f) 1 ou fim já passado, 14; tipo diferente de 2, 0; depois de [0xae1090], 0; os
/// intervalos entre 0xae1040/0xae1060/0xae1010/0xae1058/0xae1030 dão os pares 1-2, 3-4, 5-6, 7-8, 9-10 (o ímpar no dia
/// exato do limite de cima: 0x4a7f20) e, antes de 0xae1030, 11 no dia, 13 no dia de 0xae1048, senão 12. "Depois"
/// pela 0x456740. Chamada a cada contrato no passe diário (0x4c7ac0): sem contador.
unsafe extern "thiscall" fn faixa_do_vencimento(this: usize) -> u8 {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let mesmo_dia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let fim = this + 0x2d;
    let m = |g: usize| depois(fim, read_usize(g) as u32, read_usize(g + 4) as u32) != 0;
    let q = |g: usize| mesmo_dia(fim, read_usize(g) as u32, read_usize(g + 4) as u32) != 0;
    if read_u8(this + 0x35) & 0x3f == 1 {
        return 14;
    }
    if !m(0xae2c90) {
        return 14;
    }
    if read_u8(this + 0x35) & 0x3f != 2 {
        return 0;
    }
    if m(0xae1090) {
        return 0;
    }
    if m(0xae1010) { // depois do limite do meio
        if m(0xae1060) {
            if m(0xae1040) {
                return if q(0xae1090) { 1 } else { 2 };
            }
            return if q(0xae1040) { 3 } else { 4 };
        }
        return if q(0xae1060) { 5 } else { 6 };
    }
    if m(0xae1030) {
        if m(0xae1058) {
            return if q(0xae1010) { 7 } else { 8 };
        }
        return if q(0xae1058) { 9 } else { 10 };
    }
    if q(0xae1030) {
        return 11;
    }
    if q(0xae1048) { 13 } else { 12 }
}

/// As regiões que as duas primeiras gravam: o contrato (0x50 bytes), o registro de 0x4f bytes da pessoa de id `id`
/// em [0xae2d0c] (os vínculos da 0x549c90, se o id está na tabela) e [0xb67a34] (a caixa).
unsafe fn cpl5_regioes(this: usize, id: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(this, 0x50), (0xb67a34, 4)];
    let id = read_usize(read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))) as i32;
    if id >= 0 && id < read_usize(0xae2364) as i32 {
        r.push((read_usize(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)), 0x4f));
    }
    r
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa (o idioma 0x7c6830 só lê).
unsafe extern "thiscall" fn comparar_zerar_moral_do_contrato(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(916);
    let regioes = cpl5_regioes(this, read_usize(this));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa, clube), || zerar_moral_do_contrato(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("zerar_moral_do_contrato(pessoa {}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa; sem contrato atual (a caixa), só a original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_contrato_pelo_atual(this: usize, pessoa: usize, clube: usize, fim: usize, percentual: u32, sem_acesso: u32, multa: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(917);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    if contrato_de(read_usize(0xae1050), pessoa) == 0 {
        return o(this, pessoa, clube, fim, percentual, sem_acesso, multa);
    }
    let regioes = cpl5_regioes(this, read_usize(pessoa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, fim, percentual, sem_acesso, multa),
        || contrato_pelo_atual(this, pessoa, clube, fim, percentual, sem_acesso, multa));
    PLACAR.registrar(igual, || {
        format!("contrato_pelo_atual(pessoa {}, percentual {}, {}, {}) {:x?} x {:x?}", read_usize(pessoa), percentual as u8 as i8, sem_acesso as u8, multa as u8, mo, mr)
    });
}

/// Comparador: só lê; compara a faixa.
unsafe extern "thiscall" fn comparar_faixa_do_vencimento(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(918)(this);
    let n = faixa_do_vencimento(this);
    PLACAR.registrar(r == n, || format!("faixa_do_vencimento(pessoa {}) original={} rust={}", read_usize(this), r, n));
    r
}

// ---------- 243a leva: o contrato de pessoal (staff_contracts.cpp): os dias até o fim, limpar todas as queixas e conferir a retirada do pedido de transferência ----------



/// A pessoa do contrato: o id em +0 no vetor de pessoas [0xae23bc] (110 bytes cada).
unsafe fn cph1_pessoa(contrato: usize) -> usize {
    (read_u32(0xae23bc) as usize).wrapping_add((read_u32(contrato) as usize).wrapping_mul(110))
}

/// { 0x85fa00 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::days_to_end_date`: os dias da data do jogo
/// (0xae2c90) até a data do fim do contrato (+0x2d), pela 0x54cdb0 (`dias_ate`, a data do jogo por valor). O jogo lê
/// só a palavra; devolve o eax inteiro da 0x54cdb0.
unsafe extern "thiscall" fn dias_ate_o_fim_do_contrato(this: usize) -> u32 {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x54cdb0usize);
    dias(this + 0x2d, read_u32(0xae2c90), read_u32(0xae2c94)) // a data do fim do contrato
}

/// { 0x867520 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::clear_all_grievances(conferir)`: zera as
/// queixas (+0x45), tira os vínculos de opinião da pessoa (0x549c90) e, com `conferir`, confere a retirada do pedido de
/// transferência (0x867240 com 0).
unsafe extern "thiscall" fn limpar_todas_as_queixas(this: usize, conferir: u32) {
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0); // as queixas
    let vinculos: unsafe extern "C" fn(usize) = core::mem::transmute(0x549c90usize);
    vinculos(cph1_pessoa(this));
    if conferir != 0 {
        let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
        retirada(this, 0);
    }
}

/// A decisão da 0x867240 (sem efeito além da ficha [0xae1080], gravada com o contrato da pessoa): o clube do contrato
/// (fora do tipo 5) existe, a pessoa é jogador (+0x61), o contrato é desse clube (+4 = id), pediu transferência (bit 3
/// de +0x4e), não está em +0x49, o critério da comissão (0x866f60) dá 0, o nibble alto de +0x4f não é 7 e não há
/// contrato acertado com outro clube (0x8c5450 em [0xde1f54]). Devolve a pessoa quando retira.
unsafe fn cph1_retira(this: usize) -> Option<usize> {
    let pessoa = cph1_pessoa(this);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(read_u32(0xae1050) as usize, pessoa);
    *(0xae1080 as *mut usize) = c; // a ficha do contrato
    let clube = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        let i = read_u32(c + 4) as usize;
        (read_u32(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
    } else {
        0
    };
    if clube == 0 || read_u32(pessoa + 0x61) == 0 || read_u32(this + 4) != read_u32(clube) {
        return None;
    }
    if read_u8(this + 0x4e) & 8 == 0 || read_u8(this + 0x49) != 0 {
        return None; // sem pedido de transferência
    }
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    if criterio(this) != 0 || read_u8(this + 0x4f) & 0xf0 == 0x70 {
        return None;
    }
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5450usize);
    if acertado(read_u32(0xde1f54) as usize, pessoa, 0) != 0 {
        return None;
    }
    Some(pessoa)
}

/// { 0x867240 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`:
/// quando a decisão (`cph1_retira`) manda, retira o pedido (0x4d9460 em [0xae1050] com o contrato e `ctx`), apaga o
/// bit 3 de +0x4e, a felicidade no contrato (+0x44) vira +0x5b + +0x59 + 5 da pessoa, zera +0x3e e sobe +0x3d a 15; e cada queixa
/// acesa (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 de +0x45, nessa ordem) é apagada, confere de novo (0x867240 com o
/// +0x45 de antes) e tira o vínculo de opinião (0x549d60 com o bit). Sem SEH, sem caixa, sem sorteio.
unsafe extern "thiscall" fn conferir_retirada_do_pedido(this: usize, ctx: u32) {
    let Some(pessoa) = cph1_retira(this) else { return };
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4d9460usize);
    retirar(read_u32(0xae1050) as usize, this, ctx);
    *((this + 0x4e) as *mut u8) &= 0xf7; // o pedido retirado
    *((this + 0x44) as *mut u8) = read_u8(pessoa + 0x5b).wrapping_add(read_u8(pessoa + 0x59)).wrapping_add(5);
    if read_u8(this + 0x3e) != 0 {
        *((this + 0x3e) as *mut u8) = 0;
    }
    if (read_u8(this + 0x3d) as i8) < 15 {
        *((this + 0x3d) as *mut u8) = 15; // a felicidade mínima
    }
    let conferir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
    let vinculo: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x549d60usize);
    for bit in [0x10u32, 1, 4, 8, 0x100, 0x80, 0x10000] {
        let q = read_u32(this + 0x45);
        if q & bit != 0 {
            core::ptr::write_unaligned((this + 0x45) as *mut u32, q & !bit); // a queixa apagada
            conferir(this, q);
            vinculo(cph1_pessoa(this), bit as i32);
        }
    }
}

// a 0x54cdb0 só lê: compara o valor
comparar_valor!(comparar_dias_ate_o_fim_do_contrato, dias_ate_o_fim_do_contrato, 826, () -> u32);

/// Comparador: sem `conferir`, lado a lado pelas queixas e pelo registro de opiniões da pessoa (79 bytes, o que a
/// 0x549c90 escreve); com `conferir`, a retirada (0x4d9460) mexe na transferência: só a original (a 0x867240 passa
/// pelo comparador dela).
unsafe extern "thiscall" fn comparar_limpar_todas_as_queixas(this: usize, conferir: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(827);
    if conferir != 0 {
        return o(this, conferir);
    }
    let mut reg = vec![(this + 0x45, 4)];
    if let Some(c) = registro_de_opinioes(cph1_pessoa(this)) {
        reg.push((c, 79));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, conferir), || limpar_todas_as_queixas(this, conferir));
    PLACAR.registrar(igual, || format!("limpar_todas_as_queixas({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparador: quando a decisão da cópia é não retirar, lado a lado pela ficha [0xae1080] e pelo contrato (0x50
/// bytes); quando retira, só a original (a 0x4d9460 mexe na transferência e a 0x867240 se chama de novo).
unsafe extern "thiscall" fn comparar_conferir_retirada_do_pedido(this: usize, ctx: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(828);
    let ficha = read_u32(0xae1080);
    let retira = cph1_retira(this).is_some();
    *(0xae1080 as *mut u32) = ficha;
    if retira {
        return o(this, ctx);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4), (this, 0x50)], || o(this, ctx), || conferir_retirada_do_pedido(this, ctx));
    PLACAR.registrar(igual, || format!("conferir_retirada_do_pedido({:x}, {}) {:x?} x {:x?}", this, ctx, mo, mr));
}

