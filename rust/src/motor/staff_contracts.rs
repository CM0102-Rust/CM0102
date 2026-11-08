use super::*;

// ---------- 257a leva: as cláusulas do contrato: a cláusula de rescisão ativa, o período protegido da FIFA e o tipo de bônus permitido (staff_contracts.cpp) ----------



/// 1 when one of the three release clauses of the contract is active
/// Original: `STAFF_CONTRACT::has_active_release_clause` (0x867b50, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x867b50 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::has_active_release_clause()`: 1 com a cláusula +0x1c
/// = 2, a +0x1d = 1 ou a +0x1f = 2; senão 0. Pelo dia dos contratos 0x4c7ac0, pelo valor do jogador 0x4d0c60, pelo preço
/// pedido 0x4d2070, pela resposta prevista do vendedor 0x84af00, pelo vendedor que tem de aceitar 0x8cd240 e pelo lance
/// 0x8e5d60. Chamada a cada contrato: sem contador.
pub(crate) unsafe extern "thiscall" fn has_active_release_clause(c: usize) -> u32 {
    if read_u8(c + 0x1c) == 2 || read_u8(c + 0x1d) == 1 || read_u8(c + 0x1f) == 2 {
        return 1; // cpu4: com a clausula ativa
    }
    0
}

pub(crate) unsafe extern "thiscall" fn comparar_has_active_release_clause(c: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HAS_ACTIVE_RELEASE_CLAUSE)(c);
    let n = has_active_release_clause(c);
    PLACAR.registrar(r == n, || format!("has_active_release_clause({:x}) original={} rust={}", c, r, n));
    r
}

/// whether the contract is still in the protected period of the new FIFA rules: 3 years under age 28 at signing,
/// otherwise 2
/// Original: `STAFF_CONTRACT::check_fifa_protected_period` (0x868210, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x868210 } (thiscall, ret 0xc, staff_contracts.cpp, quadro SEH só da data local):
/// `STAFF_CONTRACT::check_fifa_protected_period(_, &no_limite, &anos)`: zera `no_limite` (se não nulo). Sem as regras
/// novas da FIFA ([0xae233c] ≠ 1), 1. Contrato (data +0x25) anterior à data das regras ([0xae2330]: ano sem sinal, dia
/// com sinal), 1. Senão o período protegido: 3 anos com a idade da pessoa ([0xae23bc] + 110 × [+0]) na data do contrato
/// (0x53ab90) abaixo de 28, senão 2, em `anos` (se não nulo); a duração de hoje até a data (0x54cc30, duas vezes, como a
/// original) com menos anos que o período, 1; com os anos exatos e 0 dias, `no_limite` = 1; 0. Pelo dia dos contratos
/// 0x4c7ac0, pelo lance 0x8e5d60, pela troca com compensação 0x84b6f0 e por telas. Sem contador.
pub(crate) unsafe extern "thiscall" fn check_fifa_protected_period(c: usize, _x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    if !no_limite.is_null() {
        *no_limite = 0;
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 1 {
        return 1; // cpu4: sem as regras novas
    }
    let (rdia, rano) = (*(G_TRANSFER_RULE_CHANGE_DATE as *const i16), *(G_TRANSFER_RULE_CHANGE_DATE_YEAR as *const u16));
    let d = c + 0x25;
    let (cdia, cano) = (core::ptr::read_unaligned(d as *const i16), core::ptr::read_unaligned((d + 2) as *const u16));
    if rano > cano || (rano == cano && rdia > cdia) {
        return 1;
    }
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_GET_DIFFERENCE);
    let idade: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_GET_STAFF_AGE);
    let (d0, d1) = (read_usize(d) as u32, read_usize(d + 4) as u32);
    let mut t = [0u8; 8];
    diferenca(0xae2c90, t.as_mut_ptr(), d0, d1);
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c).wrapping_mul(110));
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

pub(crate) unsafe extern "thiscall" fn comparar_check_fifa_protected_period(c: usize, x: usize, no_limite: *mut u32, anos: *mut u8) -> u32 {
    // as saídas como estavam antes: a cópia parte delas (a original só grava `anos` no período)
    let (mut nl, mut na) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) -> u32>(IDX_CHECK_FIFA_PROTECTED_PERIOD)(c, x, no_limite, anos);
    let (ol, oa) = (if no_limite.is_null() { 0 } else { *no_limite }, if anos.is_null() { 0 } else { *anos });
    let n = check_fifa_protected_period(
        c,
        x,
        if no_limite.is_null() { core::ptr::null_mut() } else { &mut nl },
        if anos.is_null() { core::ptr::null_mut() } else { &mut na },
    );
    let ok = r == n && (no_limite.is_null() || ol == nl) && (anos.is_null() || oa == na);
    PLACAR.registrar(ok, || format!("check_fifa_protected_period({:x}) original={} {} {} rust={} {} {}", c, r, ol, oa, n, nl, na));
    r
}

/// whether the bonus type applies to the player: type 2 only for goalkeepers, the others only for non-goalkeepers
/// Original: `staff_contract_is_bonus_type_allowed` (0x867aa0, staff_contracts.cpp, cdecl).
///
/// [pt] { 0x867aa0 } (cdecl, staff_contracts.cpp): `staff_contract_is_bonus_type_allowed(pessoa, tipo)`: sem a ficha de
/// jogador (+0x61), 0; goleiro (+0xf da ficha = 0x14), 1 só para o tipo 2; senão 1 para os tipos que não são 2. Pela
/// tela do contrato.
pub(crate) unsafe extern "C" fn staff_contract_is_bonus_type_allowed(pessoa: usize, tipo: u32) -> u32 {
    let f = read_usize(pessoa + staff::PLAYER);
    if f == 0 {
        return 0;
    }
    if read_u8(f + 0xf) == 0x14 {
        return (tipo as u8 == 2) as u32;
    }
    (tipo as u8 != 2) as u32
}

pub(crate) unsafe extern "C" fn comparar_staff_contract_is_bonus_type_allowed(pessoa: usize, tipo: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_STAFF_CONTRACT_IS_BONUS_TYPE_ALLOWED)(pessoa, tipo);
    let n = staff_contract_is_bonus_type_allowed(pessoa, tipo);
    PLACAR.registrar(r == n, || format!("staff_contract_is_bonus_type_allowed({:x}, {}) original={} rust={}", pessoa, tipo as u8, r, n));
    r
}

// ---------- 248a leva: o contrato da comissão: montar a proposta pelo contrato atual, zerar a moral do contrato novo e a faixa do vencimento (staff_contracts.cpp) ----------





/// Common part of 0x866ee0 and 0x85e590: clears the contract's happiness, grievances and morale state, the person's
/// opinion links, and sets the nation language level in +0x4f.
///
/// [pt] A parte comum da 0x866ee0 e da 0x85e590 (inline nesta): zera a felicidade no contrato (+0x44), o estado +0x4e e as queixas (+0x45),
/// limpa os vínculos de opinião da pessoa do contrato (0x549c90 com o registro [0xae23bc] + id × 0x6e), zera +0x40,
/// põe 4 em +0x4e, zera a felicidade (+0x3f) e +0x3e; com o clube e a nação dele (+0x53), o nível do idioma da nação
/// (+0x77) para a pessoa (0x7c6830 em [0xb74350], sem o contrato; no máximo 10) nos 4 bits baixos de +0x4f.
pub(crate) unsafe fn reset_contract_morale(this: usize, pessoa: usize, clube: usize) {
    crate::comparacao::ramo("cpl_5 moral zerada");
    write_u8(this + 0x44, 0);
    write_u8(this + 0x4e, 0);
    write_usize(this + 0x45, 0);
    let limpar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_CLEAR_OPINION_LINKS);
    limpar(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(this).wrapping_mul(0x6e)));
    write_usize(this + 0x40, 0);
    write_u8(this + 0x4e, 4);
    write_u8(this + 0x3f, 0); // a felicidade
    write_u8(this + 0x3e, 0);
    if clube != 0 && read_usize(clube + club::NATION) != 0 {
        let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_PLAYERSTATS_GET_LANGUAGE_FLUENCY);
        let r = fluencia(read_usize(G_PLAYER_STATS), pessoa, clube, read_u8(read_usize(clube + club::NATION) + 0x77) as u32, 0) as i8;
        let c = if r > 10 { 10 } else { r } as u8;
        let b = read_u8(this + 0x4f);
        write_u8(this + 0x4f, (b & 0xf0) | (c & 0xf));
    }
}

/// clears the contract morale, grievances and happiness, the person's opinion links (0x549c90), sets +0x4e to 4 and the
/// club nation language level (0x7c6830, up to 10) in +0x4f
/// Original: `STAFF_CONTRACT::reset_morale_state` (0x866ee0, staff_contracts.cpp, thiscall, ret 8).
///
/// [pt] { 0x866ee0 } (thiscall, ret 8, staff_contracts.cpp): `STAFF_CONTRACT::reset_morale_state(pessoa, clube)`: o
/// `reset_contract_morale`. Pela assinatura do contrato (CONTRACT_MANAGER::sign_contract 0x4cf720, nos termos que
/// depois são copiados para o contrato guardado).
pub(crate) unsafe extern "thiscall" fn reset_morale_state(this: usize, pessoa: usize, clube: usize) {
    reset_contract_morale(this, pessoa, clube);
}

/// the offer from the current contract: person, club, wage × percent / 100, today, the end date, flags, morale reset as
/// 0x866ee0 and the two clauses; without contract, error box
/// Original: `STAFF_CONTRACT::init_from_current_contract` (0x85e590, staff_contracts.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x85e590 } (thiscall, ret 0x18, staff_contracts.cpp): `STAFF_CONTRACT::init_from_current_contract(pessoa, clube,
/// fim, percentual, sem_acesso, multa)`: a proposta de transferência ou empréstimo pelo contrato atual da pessoa
/// (0x4cf5a0 em [0xae1050]; sem contrato, caixa 0xf5 e nada): id da pessoa +0, clube +4, 0 em +8, salário +0xc =
/// salário atual × percentual (com sinal) / 100, a data de hoje (0xae2c90) em +0x25, o fim em +0x2d, os bits
/// +0x35 (& 0xc6 | 6), o salário também em +0x36, 0 em +0x3a e +0x49, −1 em +0x4a, o `reset_contract_morale`, 0 em
/// +0x3b (16 bits), +0x1c..=+0x20 e +0x21, −1 em +0x10/+0x14/+0x18 e as cláusulas: +0x1c sem acesso, +0x1d multa
/// mínima. As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn init_from_current_contract(this: usize, pessoa: usize, clube: usize, fim: usize, write_percent_pair: u32, sem_acesso: u32, multa: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
    if c == 0 {
        show_error_at(0xa8360c, 0xf5);
        return;
    }
    crate::comparacao::ramo("cpl_5 contrato pelo atual");
    write_usize(this, read_usize(pessoa));
    write_usize(this + 8, 0);
    write_usize(this + 4, read_usize(clube));
    let salario = (read_usize(c + 0xc) as i32).wrapping_mul(write_percent_pair as u8 as i8 as i32) / 100; // o percentual do salário
    write_usize(this + 0xc, salario as usize);
    core::ptr::write_unaligned((this + 0x25) as *mut [u8; 8], core::ptr::read_unaligned(G_CURRENT_DATE as *const [u8; 8])); // hoje
    core::ptr::write_unaligned((this + 0x2d) as *mut [u8; 8], core::ptr::read_unaligned(fim as *const [u8; 8])); // o fim do contrato
    let b = (read_u8(this + 0x35) & 0xc6) | 6;
    write_usize(this + 0x36, read_usize(this + 0xc));
    write_u8(this + 0x35, b);
    write_u8(this + 0x3a, 0);
    write_u8(this + 0x49, 0);
    write_usize(this + 0x4a, usize::MAX);
    reset_contract_morale(this, pessoa, clube);
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    for k in 0x1c..=0x20 {
        write_u8(this + k, 0);
    }
    write_usize(this + 0x10, usize::MAX);
    write_usize(this + 0x14, usize::MAX);
    write_usize(this + 0x18, usize::MAX);
    write_usize(this + 0x21, 0);
    write_u8(this + 0x1c, sem_acesso as u8);
    write_u8(this + 0x1d, multa as u8);
}

/// the 0..=14 band of the contract end date against today and the CONTRACT_MANAGER limit dates (0xae1010..0xae1090)
/// Original: `STAFF_CONTRACT::get_expiry_band` (0x867d30, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x867d30 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::get_expiry_band()`: a faixa 0..=14 do fim do contrato
/// (+0x2d) contra as datas de hoje (0xae2c90) e as 16 datas do CONTRACT_MANAGER (0xae1010..0xae1090, montadas na
/// criação): tipo (+0x35 & 0x3f) 1 ou fim já passado, 14; tipo diferente de 2, 0; depois de [0xae1090], 0; os
/// intervalos entre 0xae1040/0xae1060/0xae1010/0xae1058/0xae1030 dão os pares 1-2, 3-4, 5-6, 7-8, 9-10 (o ímpar no dia
/// exato do limite de cima: 0x4a7f20) e, antes de 0xae1030, 11 no dia, 13 no dia de 0xae1048, senão 12. "Depois"
/// pela 0x456740. Chamada a cada contrato no passe diário (0x4c7ac0): sem contador.
pub(crate) unsafe extern "thiscall" fn get_expiry_band(this: usize) -> u8 {
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let mesmo_dia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3D_3D);
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

/// Memory regions the two morale-reset routines write: the contract (0x50 bytes), the person's 79-byte staff state
/// record and 0xb67a34.
///
/// [pt] As regiões que as duas primeiras gravam: o contrato (0x50 bytes), o registro de 0x4f bytes da pessoa de id `id`
/// em [0xae2d0c] (os vínculos da 0x549c90, se o id está na tabela) e [0xb67a34] (a caixa).
pub(crate) unsafe fn morale_reset_regions(this: usize, id: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(this, 0x50), (0xb67a34, 4)];
    let id = read_usize(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(0x6e))) as i32;
    if id >= 0 && id < read_usize(G_STAFF_COUNT) as i32 {
        r.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f)), 0x4f));
    }
    r
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa (o idioma 0x7c6830 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_reset_morale_state(this: usize, pessoa: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RESET_MORALE_STATE);
    let regioes = morale_reset_regions(this, read_usize(this));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, pessoa, clube), || reset_morale_state(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("reset_morale_state(pessoa {}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
}

/// Comparador: lado a lado no contrato e nos vínculos da pessoa; sem contrato atual (a caixa), só a original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_init_from_current_contract(this: usize, pessoa: usize, clube: usize, fim: usize, write_percent_pair: u32, sem_acesso: u32, multa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32)>(IDX_INIT_FROM_CURRENT_CONTRACT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    if contract_of(read_usize(G_CONTRACT_MANAGER), pessoa) == 0 {
        return o(this, pessoa, clube, fim, write_percent_pair, sem_acesso, multa);
    }
    let regioes = morale_reset_regions(this, read_usize(pessoa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, fim, write_percent_pair, sem_acesso, multa),
        || init_from_current_contract(this, pessoa, clube, fim, write_percent_pair, sem_acesso, multa));
    PLACAR.registrar(igual, || {
        format!("init_from_current_contract(pessoa {}, write_percent_pair {}, {}, {}) {:x?} x {:x?}", read_usize(pessoa), write_percent_pair as u8 as i8, sem_acesso as u8, multa as u8, mo, mr)
    });
}

/// Comparador: só lê; compara a faixa.
pub(crate) unsafe extern "thiscall" fn comparar_get_expiry_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_EXPIRY_BAND)(this);
    let n = get_expiry_band(this);
    PLACAR.registrar(r == n, || format!("get_expiry_band(pessoa {}) original={} rust={}", read_usize(this), r, n));
    r
}

// ---------- 243a leva: o contrato de pessoal (staff_contracts.cpp): os dias até o fim, limpar todas as queixas e conferir a retirada do pedido de transferência ----------



/// Person record (110 bytes in G_STAFF_TABLE) of the contract's id (+0).
///
/// [pt] A pessoa do contrato: o id em +0 no vetor de pessoas [0xae23bc] (110 bytes cada).
pub(crate) unsafe fn staff_of_contract(contrato: usize) -> usize {
    (read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contrato) as usize).wrapping_mul(110))
}

/// the days from the game date to the contract end date (+0x2d), through 0x54cdb0
/// Original: `STAFF_CONTRACT::days_to_end_date` (0x85fa00, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x85fa00 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::days_to_end_date`: os dias da data do jogo
/// (0xae2c90) até a data do fim do contrato (+0x2d), pela 0x54cdb0 (`dias_ate`, a data do jogo por valor). O jogo lê
/// só a palavra; devolve o eax inteiro da 0x54cdb0.
pub(crate) unsafe extern "thiscall" fn days_to_end_date(this: usize) -> u32 {
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    dias(this + 0x2d, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) // a data do fim do contrato
}

/// clears the grievances (+0x45), the opinion links (0x549c90) and, when asked, checks the withdrawal (0x867240)
/// Original: `STAFF_CONTRACT::clear_all_grievances` (0x867520, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x867520 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::clear_all_grievances(conferir)`: zera as
/// queixas (+0x45), tira os vínculos de opinião da pessoa (0x549c90) e, com `conferir`, confere a retirada do pedido de
/// transferência (0x867240 com 0).
pub(crate) unsafe extern "thiscall" fn clear_all_grievances(this: usize, conferir: u32) {
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0); // as queixas
    let vinculos: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_CLEAR_OPINION_LINKS);
    vinculos(staff_of_contract(this));
    if conferir != 0 {
        let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
        retirada(this, 0);
    }
}

/// Decision of 0x867240: returns the person when their transfer request should be withdrawn (player at the contract's
/// club, request bit set, no agreed move...), else None.
///
/// [pt] A decisão da 0x867240 (sem efeito além da ficha [0xae1080], gravada com o contrato da pessoa): o clube do contrato
/// (fora do tipo 5) existe, a pessoa é jogador (+0x61), o contrato é desse clube (+4 = id), pediu transferência (bit 3
/// de +0x4e), não está em +0x49, o critério da comissão (0x866f60) dá 0, o nibble alto de +0x4f não é 7 e não há
/// contrato acertado com outro clube (0x8c5450 em [0xde1f54]). Devolve a pessoa quando retira.
pub(crate) unsafe fn should_withdraw_transfer_request(this: usize) -> Option<usize> {
    let pessoa = staff_of_contract(this);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_u32(G_CONTRACT_MANAGER) as usize, pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c; // a ficha do contrato
    let clube = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        let i = read_u32(c + 4) as usize;
        (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
    } else {
        0
    };
    if clube == 0 || read_u32(pessoa + staff::PLAYER) == 0 || read_u32(this + 4) != read_u32(clube) {
        return None;
    }
    if read_u8(this + 0x4e) & 8 == 0 || read_u8(this + 0x49) != 0 {
        return None; // sem pedido de transferência
    }
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    if criterio(this) != 0 || read_u8(this + 0x4f) & 0xf0 == 0x70 {
        return None;
    }
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_AGREED_CONTRACT_CLUB);
    if acertado(read_u32(G_TRANSFER_MANAGER) as usize, pessoa, 0) != 0 {
        return None;
    }
    Some(pessoa)
}

/// a player of the contract club with a transfer request, no staff criterion and no agreed contract: withdraws the
/// request (0x4d9460), morale becomes +0x5b + +0x59 + 5, +0x3e = 0, +0x3d at least 15, and clears each grievance
/// checking again
/// Original: `STAFF_CONTRACT::check_transfer_request_withdrawal` (0x867240, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x867240 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`:
/// quando a decisão (`should_withdraw_transfer_request`) manda, retira o pedido (0x4d9460 em [0xae1050] com o contrato e `ctx`), apaga o
/// bit 3 de +0x4e, a felicidade no contrato (+0x44) vira +0x5b + +0x59 + 5 da pessoa, zera +0x3e e sobe +0x3d a 15; e cada queixa
/// acesa (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 de +0x45, nessa ordem) é apagada, confere de novo (0x867240 com o
/// +0x45 de antes) e tira o vínculo de opinião (0x549d60 com o bit). Sem SEH, sem caixa, sem sorteio.
pub(crate) unsafe extern "thiscall" fn check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let Some(pessoa) = should_withdraw_transfer_request(this) else { return };
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRANSFER_REQUEST_WITHDRAWN_NEWS);
    retirar(read_u32(G_CONTRACT_MANAGER) as usize, this, ctx);
    *((this + 0x4e) as *mut u8) &= 0xf7; // o pedido retirado
    *((this + 0x44) as *mut u8) = read_u8(pessoa + staff::PROFESSIONALISM).wrapping_add(read_u8(pessoa + staff::LOYALITY)).wrapping_add(5);
    if read_u8(this + 0x3e) != 0 {
        *((this + 0x3e) as *mut u8) = 0;
    }
    if (read_u8(this + 0x3d) as i8) < 15 {
        *((this + 0x3d) as *mut u8) = 15; // a felicidade mínima
    }
    let conferir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
    let contract_bond: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_STAFF_REMOVE_OPINION_LINK);
    for bit in [0x10u32, 1, 4, 8, 0x100, 0x80, 0x10000] {
        let q = read_u32(this + 0x45);
        if q & bit != 0 {
            core::ptr::write_unaligned((this + 0x45) as *mut u32, q & !bit); // a queixa apagada
            conferir(this, q);
            contract_bond(staff_of_contract(this), bit as i32);
        }
    }
}

// a 0x54cdb0 só lê: compara o valor
comparar_valor!(comparar_days_to_end_date, days_to_end_date, IDX_DAYS_TO_END_DATE, () -> u32);

/// Comparador: sem `conferir`, lado a lado pelas queixas e pelo registro de opiniões da pessoa (79 bytes, o que a
/// 0x549c90 escreve); com `conferir`, a retirada (0x4d9460) mexe na transferência: só a original (a 0x867240 passa
/// pelo comparador dela).
pub(crate) unsafe extern "thiscall" fn comparar_clear_all_grievances(this: usize, conferir: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CLEAR_ALL_GRIEVANCES);
    if conferir != 0 {
        return o(this, conferir);
    }
    let mut reg = vec![(this + 0x45, 4)];
    if let Some(c) = database_staff_state_record(staff_of_contract(this)) {
        reg.push((c, 79));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, conferir), || clear_all_grievances(this, conferir));
    PLACAR.registrar(igual, || format!("clear_all_grievances({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// Comparador: quando a decisão da cópia é não retirar, lado a lado pela ficha [0xae1080] e pelo contrato (0x50
/// bytes); quando retira, só a original (a 0x4d9460 mexe na transferência e a 0x867240 se chama de novo).
pub(crate) unsafe extern "thiscall" fn comparar_check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
    let ficha = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let retira = should_withdraw_transfer_request(this).is_some();
    *(G_CONTRACT_LOOKUP_RESULT as *mut u32) = ficha;
    if retira {
        return o(this, ctx);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4), (this, 0x50)], || o(this, ctx), || check_transfer_request_withdrawal(this, ctx));
    PLACAR.registrar(igual, || format!("check_transfer_request_withdrawal({:x}, {}) {:x?} x {:x?}", this, ctx, mo, mr));
}

/// sets the flags at +0x40 and, when the person now wants to leave and nothing prevents it, the transfer request
/// (0x4d8610)
/// Original: `STAFF_CONTRACT::set_flags` (0x867560, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x867560 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_flags(marcas)`, chamada por muitas rotinas
/// (no motor.rs pelo endereço; antes só a porta do pós-jogo a desviava, no modo comparação). Marcas acima de 0x1000: a
/// caixa "Error" e segue (aqui: a original inteira). Liga as marcas em +0x40; pessoa (+0 id) que não é das funções
/// 0xc, 0xd e 0xf, contrato com +0x44 positivo, sem +0x49, que quer sair (0x866f60), sem transferência futura
/// (0x8cca10 com 0), sem os bits 1 e 8 de +0x4e, fora da fila da transferência livre (0x8c1870) e com mais de 20 anos
/// (+0x18) ou reputação atual (+0xb do jogador) acima de 4250: o pedido de transferência (0x4d8610 em [0xae1050]).
/// Não lê local sem valor.
pub(crate) unsafe extern "thiscall" fn set_flags(c: usize, marcas: u32) {
    if marcas as i32 > 0x1000 {
        // ponytail: a caixa "Error" (antes de qualquer gravação): a original inteira
        return original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SET_FLAGS)(c, marcas);
    }
    write_u32(c + 0x40, read_usize(c + 0x40) as u32 | marcas);
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(c) as usize).wrapping_mul(0x6e));
    let funcao = read_u8(pessoa + staff::JOB_FOR_CLUB);
    if funcao == 0xc || funcao == 0xd || funcao == 0xf {
        return;
    }
    let moral = read_u8(c + 0x44) as i8;
    if moral == 0 || moral < 0 || read_u8(c + 0x49) != 0 {
        return;
    }
    let quer_sair: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    if quer_sair(c) == 0 {
        return;
    }
    let futura: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if futura(read_usize(G_TRANSFER_MANAGER), pessoa, 0) != 0 || read_u8(c + 0x4e) & 9 != 0 {
        return;
    }
    let na_fila: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_QUEUED_FOR_FREE_TRANSFER);
    if na_fila(read_usize(G_TRANSFER_MANAGER), pessoa) != 0 {
        return;
    }
    if read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8 <= 0x14 && read_i16(read_usize(pessoa + staff::PLAYER) + 0xb) <= 0x109a {
        return;
    }
    crate::comparacao::ramo("cpw_7 pedido de saida");
    if CPW7_ENSAIO {
        CPW7_DECISAO = 1;
        return;
    }
    let pedido: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_HANDLE_TRANSFER_REQUEST);
    pedido(read_usize(G_CONTRACT_MANAGER), c);
}

/// O comparador das marcas do contrato, em ensaio: a cópia sem o pedido de transferência e a original a partir do mesmo
/// estado (marcas e sorteio); sem pedido, tudo tem de bater; com pedido (que sorteia e dá notícia), só as marcas.
pub(crate) unsafe extern "thiscall" fn comparar_set_flags(c: usize, marcas: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SET_FLAGS);
    if marcas as i32 > 0x1000 {
        return o(c, marcas);
    }
    let mut regioes = vec![(c + 0x40, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW7_ENSAIO = true;
    CPW7_DECISAO = 0;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| set_flags(c, marcas)));
    CPW7_ENSAIO = false;
    let pediu = CPW7_DECISAO != 0;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(c, marcas);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if pediu { mo.0[0] == mr.0[0] } else { mo == mr };
    PLACAR.registrar(igual, || format!("set_flags({:#x}, {:#x}) pediu={} original={:x?} rust={:x?}", c, marcas, pediu, mo, mr));
}

/// 1 to 3: 100 × the value of the current contract / that of the desired one (with the deserved squad status), with an
/// allowance for recent signings; below 50: 1; below 80: 3; otherwise 2
/// Original: `get_contract_satisfaction` (0x866620, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x866620 } (thiscall, ret 4, staff_contracts.cpp, quadro SEH sem destrutor): `STAFF_CONTRACT::
/// get_contract_satisfaction(pessoa)`, pelo passe diário do pessoal (0x5ce2d0, com sorteio(50) zero) e pela tela do
/// contrato: 1 a 3. Sem pessoa, 1. A situação no elenco que ela merece (0x4ca470) vale se for menor que a do contrato
/// (+0x4f >> 4) e não for 0 nem 7; senão, a do contrato. O contrato desejado (0x85fc50 num contrato local, com o clube
/// do contrato, −1, a situação e 0xb) e a razão 100 × valor do contrato atual (0x866830, em f32) / valor do desejado
/// (valor zero: razão 0). Contratado há menos de 15 × adaptabilidade (+0x59) + 100 dias (de +0x25, teto 500): razão +
/// 50 + 7,5 × adaptabilidade − dias / 2. Razão abaixo de 50: 1; abaixo de 80: 3; senão 2.
pub(crate) unsafe extern "thiscall" fn get_contract_satisfaction(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 1;
    }
    let categoria: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    // o contrato local (0x50 bytes) como a original o deixa antes da 0x85fc50; o resto ela preenche
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    data_padrao(l + 0x25);
    data_padrao(l + 0x2d);
    core::ptr::write_unaligned((l + 4) as *mut i32, -1);
    core::ptr::write_unaligned((l + 0x4a) as *mut i32, -1);
    let merece = categoria(pessoa, this);
    let atual = read_u8(this + 0x4f) >> 4;
    // ponytail: a original empurra o dword inteiro da vaga onde guardou o byte (no primeiro caso, o resto é o ponteiro
    // da pessoa; no segundo, sobra de pilha); a 0x85fc50 só lê o byte.
    let situacao = if (merece as i8) < (atual as i8) && merece != 0 && merece != 7 {
        (pessoa as u32 & !0xff) | merece as u32
    } else {
        atual as u32
    };
    desejado(l, pessoa, friendly_club_by_id(read_usize(this + 4)), -1, situacao, 0xb, 0, 0);
    let v1 = calc_contract_value(this, pessoa, l) as f32;
    let v2 = calc_contract_value(l, pessoa, l) as f64;
    let mut razao = if v2 == read_f32_at(0x968970) as f64 {
        read_f32_at(0x968970) as f64
    } else {
        v1 as f64 * read_f32_at(0x968960) as f64 / v2
    };
    let anos = (read_i16(G_CURRENT_DATE_YEAR) as u16 as i32).wrapping_sub(read_i16(this + 0x27) as u16 as i32);
    let dias = anos.wrapping_mul(365).wrapping_sub(read_i16(this + 0x25) as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32).clamp(0, 500);
    let adapt = read_u8(pessoa + staff::LOYALITY) as i8 as i32;
    if dias < adapt.wrapping_mul(15).wrapping_add(100) {
        crate::comparacao::ramo("cpw_1 contratado ha pouco");
        razao = razao + read_f32_at(0x969560) as f64 + adapt as f64 * read_f64_at(0x968eb0) - (dias / 2) as f64; // cpw1: o desconto do recém-contratado
    }
    crate::comparacao::ramo("cpw_1 satisfacao avaliada");
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
pub(crate) unsafe extern "thiscall" fn comparar_get_contract_satisfaction(this: usize, pessoa: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_CONTRACT_SATISFACTION);
    let mut regioes = vec![(0xae1080usize, 4usize), (this, 0x50)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, pessoa),
        || crate::aleatorio::sem_contar(|| get_contract_satisfaction(this, pessoa)),
    );
    PLACAR.registrar(igual && r == n, || {
        format!("get_contract_satisfaction(pessoa {:#x}) original={} {:x?} rust={} {:x?}", if pessoa != 0 { read_usize(pessoa) } else { 0 }, r, mo, n, mr)
    });
    r
}

/// the wage (+0xc) = the default for the person at the club (0x862720) truncated, minimum 55
/// Original: `STAFF_CONTRACT::set_default_wage` (0x8626e0, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x8626e0 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_default_wage(pessoa)`: o salário (+0xc) é
/// o padrão da pessoa no clube dela ([+0x39]) pela 0x862720 (stdcall, 6 argumentos, float; o ecx vai junto, como na
/// original), truncado (0x945640), com mínimo 55. Devolve o truncado (antes do mínimo).
pub(crate) unsafe extern "thiscall" fn set_default_wage(this: usize, pessoa: usize) -> u32 {
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32, i32, i32) -> f64 = core::mem::transmute(FN_STAFF_CONTRACT_CALC_UNSET_DEFAULT_WAGE);
    let v = match_pl_ftol(padrao(this, pessoa, core::ptr::read_unaligned((pessoa + staff::CLUB_JOB) as *const usize), 1, -1, 0xb, 0));
    *((this + 0xc) as *mut i32) = if v < 0x37 { 0x37 } else { v }; // o mínimo de 55
    v as u32
}

/// Comparador: lado a lado pelo salário (+0xc). A 0x862720 grava a mesma ficha em [0xae1080] nas duas vezes.
pub(crate) unsafe extern "thiscall" fn comparar_set_default_wage(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_DEFAULT_WAGE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 4)], || o(this, pessoa), || set_default_wage(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("set_default_wage({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// +0x40 (a 4-byte routine; the hook covers the first alignment nop)
/// Original: `contract_flags` (0x8676f0, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x8676f0 } (staff_contracts.cpp): as bandeiras do contrato (+0x40).
pub(crate) unsafe extern "thiscall" fn contract_flags(this: usize) -> u32 {
    *((this + 0x40) as *const u32)
}

/// the empty contract: default dates at +0x25 and +0x2d, +4 and +0x4a = -1, +8, +0x3b, +0x40, +0x45, +0x49 and +0x4e =
/// 0
/// Original: `STAFF_CONTRACT::STAFF_CONTRACT` (0x85e410, staff_contracts.cpp, thiscall, ret 0).
///
/// [pt] { 0x85e410 } (thiscall, ret 0, staff_contracts.cpp): `STAFF_CONTRACT::STAFF_CONTRACT()`: as duas datas (+0x25,
/// +0x2d) padrão (0x54a110), +4 e +0x4a = −1, +0x49, +0x3b, +0x4e, +0x40, +0x45 e +8 = 0. Devolve o próprio contrato.
pub(crate) unsafe extern "thiscall" fn construct_staff_contract(this: usize) -> usize {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    data_padrao(this + 0x25);
    data_padrao(this + 0x2d);
    core::ptr::write_unaligned((this + 4) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((this + 0x4a) as *mut u32, u32::MAX);
    *((this + 0x49) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    *((this + 0x4e) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x40) as *mut u32, 0);
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0);
    core::ptr::write_unaligned((this + 8) as *mut u32, 0);
    this
}

/// Comparador: lado a lado nos 0x4f bytes do contrato e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_construct_staff_contract(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_STAFF_CONTRACT);
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x4f)], || o(this), || construct_staff_contract(this));
    PLACAR.registrar(igual && ro == rr, || format!("construct_staff_contract({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}
