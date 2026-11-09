use super::*;

/// Returns 1 when one of the three release clauses of the contract is active.
///
/// `STAFF_CONTRACT::has_active_release_clause()`: 1 with clause +0x1c = 2, +0x1d = 1 or +0x1f = 2; otherwise 0. Called
/// from the contracts' day 0x4c7ac0, the player value `calc_player_value`, the asking price 0x4d2070, the seller's expected answer
/// `shortlist_manager_predict_selling_club_response`, the seller who has to accept `transfer_manager_selling_club_must_accept` and the bid `exe::transfer_offer_build_bid`. TapeCall on every contract: no counter.
/// Original: `STAFF_CONTRACT::has_active_release_clause` (0x867b50, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x867b50 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::has_active_release_clause()`: 1 com a cláusula +0x1c
/// = 2, a +0x1d = 1 ou a +0x1f = 2; senão 0. Pelo dia dos contratos 0x4c7ac0, pelo valor do jogador 0x4d0c60, pelo preço
/// pedido 0x4d2070, pela resposta prevista do vendedor 0x84af00, pelo vendedor que tem de aceitar 0x8cd240 e pelo lance
/// 0x8e5d60. TapeCall a cada contrato: sem contador.
pub(crate) unsafe extern "thiscall" fn has_active_release_clause(c: usize) -> u32 {
    let c = Contract(c);
    if c.non_promotion_rc() == 2 || c.minimum_fee_rc() == 1 || c.relegation_rc() == 2 {
        return 1; // cpu4: with the clause active
    }
    0
}

/// Checks whether the contract is still in the protected period of the new FIFA rules: 3 years under age 28 at signing,
/// otherwise 2.
///
/// SEH frame only for the local date. `STAFF_CONTRACT::check_fifa_protected_period(_, &at_limit, &years)`: clears
/// `at_limit` (if not null). Without the new FIFA rules (`[0xae233c]` ≠ 1), 1. A contract (date +0x25) before the
/// rules' date (`[0xae2330]`: unsigned year, signed day), 1. Otherwise the protected period: 3 years with the person's
/// age (`[0xae23bc]` + 110 × `[+0]`) at the contract date (`database_get_staff_age`) below 28, otherwise 2, in `years` (if not null);
/// the duration from today to the date (`exe::date_get_difference`, twice, as in the original) with fewer years than the period, 1; with
/// the exact years and 0 days, `at_limit` = 1; 0. Called from the contracts' day 0x4c7ac0, the bid `exe::transfer_offer_build_bid`, the swap
/// with compensation `shortlist_manager_is_compensation_deal` and screens. No counter.
/// Original: `STAFF_CONTRACT::check_fifa_protected_period` (0x868210, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x868210 } (thiscall, ret 0xc, staff_contracts.cpp, quadro SEH só da data local):
/// `STAFF_CONTRACT::check_fifa_protected_period(_, &no_limite, &anos)`: zera `no_limite` (se não nulo). Sem as regras
/// novas da FIFA ([0xae233c] ≠ 1), 1. Contrato (data +0x25) anterior à data das regras ([0xae2330]: ano sem sinal, dia
/// com sinal), 1. Senão o período protegido: 3 anos com a idade da pessoa ([0xae23bc] + 110 × [+0]) na data do contrato
/// (0x53ab90) abaixo de 28, senão 2, em `anos` (se não nulo); a duração de hoje até a data (0x54cc30, duas vezes, como a
/// original) com menos anos que o período, 1; com os anos exatos e 0 dias, `no_limite` = 1; 0. Pelo dia dos contratos
/// 0x4c7ac0, pelo lance 0x8e5d60, pela troca com compensação 0x84b6f0 e por telas. Sem contador.
pub(crate) unsafe extern "thiscall" fn check_fifa_protected_period(c: usize, _x: usize, no_limite: *mut u32, years: *mut u8) -> u32 {
    let c = Contract(c);
    if !no_limite.is_null() {
        *no_limite = 0;
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 1 {
        return 1; // cpu4: without the new rules
    }
    let (rdia, rano) = (*(G_TRANSFER_RULE_CHANGE_DATE as *const i16), *(G_TRANSFER_RULE_CHANGE_DATE_YEAR as *const u16));
    let d = c.addr() + contract::DATE_STARTED;
    let (cdia, cano) = (core::ptr::read_unaligned(d as *const i16), core::ptr::read_unaligned((d + 2) as *const u16));
    if rano > cano || (rano == cano && rdia > cdia) {
        return 1;
    }
    let (d0, d1) = (read_usize(d) as u32, read_usize(d + 4) as u32);
    let mut t = [0u8; 8];
    exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), d0, d1);
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c.addr()).wrapping_mul(110));
    let limit: u8 = if (exe::database_get_staff_age(person, d as *const u16)) < 0x1c { 3 } else { 2 }; // cpu4: the period by age
    if !years.is_null() {
        *years = limit;
    }
    let mut l = [0u8; 8];
    let p = exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), d0, d1);
    core::ptr::copy_nonoverlapping(p as *const u8, l.as_mut_ptr(), 8);
    let a = u16::from_le_bytes([l[2], l[3]]) as i32;
    if a < limit as i32 {
        return 1; // cpu4: dentro do periodo
    }
    if !no_limite.is_null() && a == limit as i32 && u16::from_le_bytes([l[0], l[1]]) == 0 {
        *no_limite = 1;
    }
    0
}

/// whether the bonus type applies to the player: type 2 only for goalkeepers, the others only for non-goalkeepers
/// Original: `staff_contract_is_bonus_type_allowed` (0x867aa0, staff_contracts.cpp, cdecl).
///
/// [pt] { 0x867aa0 } (cdecl, staff_contracts.cpp): `staff_contract_is_bonus_type_allowed(pessoa, tipo)`: sem a ficha de
/// jogador (+0x61), 0; goleiro (+0xf da ficha = 0x14), 1 só para o tipo 2; senão 1 para os tipos que não são 2. Pela
/// tela do contrato.
pub(crate) unsafe extern "C" fn staff_contract_is_bonus_type_allowed(person: usize, kind: u32) -> u32 {
    let person = Staff(person);
    let f = person.player();
    if f == 0 {
        return 0;
    }
    if read_u8(f + player_sheet::GOALKEEPER) == 0x14 {
        return (kind as u8 == 2) as u32;
    }
    (kind as u8 != 2) as u32
}

/// The common part of `reset_morale_state` and `init_from_current_contract` (inlined in the latter): clears the contract's happiness, grievances and
/// morale state and the person's opinion links, and sets the nation language level in +0x4f.
///
/// Zeroes the happiness in the contract (+0x44), state +0x4e and the complaints (+0x45); clears the opinion links of
/// the contract's person (`staff_clear_opinion_links` with record [0xae23bc] + id × 0x6e); zeroes +0x40, sets +0x4e to 4, zeroes the
/// happiness (+0x3f) and +0x3e; with the club and its nation (+0x53), the nation's language (+0x77) level for the
/// person (`get_language_fluency` on [0xb74350], without the contract; at most 10) goes in the low 4 bits of +0x4f.
///
/// [pt] A parte comum da 0x866ee0 e da 0x85e590 (inline nesta): zera a felicidade no contrato (+0x44), o estado +0x4e e as queixas (+0x45),
/// limpa os vínculos de opinião da pessoa do contrato (0x549c90 com o registro [0xae23bc] + id × 0x6e), zera +0x40,
/// põe 4 em +0x4e, zera a felicidade (+0x3f) e +0x3e; com o clube e a nação dele (+0x53), o nível do idioma da nação
/// (+0x77) para a pessoa (0x7c6830 em [0xb74350], sem o contrato; no máximo 10) nos 4 bits baixos de +0x4f.
pub(crate) unsafe fn reset_contract_morale(this: Contract, person: usize, club: Club) {
    crate::comparison::branch("cpl_5 morale zeroed");
    this.set_morale((0) as i8);
    this.set_transfer_status(0);
    this.set_grievances((0) as u32);
    exe::staff_clear_opinion_links(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(this.addr()).wrapping_mul(staff::SIZE)));
    this.set_flags((0) as u32);
    this.set_transfer_status(4);
    this.set_matches_left_out(0); // a felicidade
    this.set_unhappy_flag((0) as i8);
    if !club.is_null() && club.nation().addr() != 0 {
        let r = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), person, club, read_u8(club.nation().addr() + 0x77) as u32, 0) as i8;
        let c = if r > 10 { 10 } else { r } as u8;
        let b = this.squad_status();
        this.set_squad_status((b & 0xf0) | (c & 0xf));
    }
}

/// clears the contract morale, grievances and happiness, the person's opinion links (`staff_clear_opinion_links`), sets +0x4e to 4 and the
/// club nation language level (`get_language_fluency`, up to 10) in +0x4f
/// Original: `STAFF_CONTRACT::reset_morale_state` (0x866ee0, staff_contracts.cpp, thiscall, ret 8).
///
/// [pt] { 0x866ee0 } (thiscall, ret 8, staff_contracts.cpp): `STAFF_CONTRACT::reset_morale_state(pessoa, clube)`: o
/// `reset_contract_morale`. Pela assinatura do contrato (CONTRACT_MANAGER::sign_contract 0x4cf720, nos termos que
/// depois são copiados para o contrato guardado).
pub(crate) unsafe extern "thiscall" fn reset_morale_state(this: usize, person: usize, club: usize) {
    reset_contract_morale(Contract(this), person, Club(club));
}

/// Builds the offer from the current contract: person, club, wage × percent / 100, today, the end date, flags, morale
/// reset as in `reset_morale_state` and the two clauses; without a contract, error box.
///
/// `STAFF_CONTRACT::init_from_current_contract(person, club, end, percent, no_access, fee)`: the transfer or loan
/// proposal from the person's current contract (`exe::contract_manager_get_contract` on `[0xae1050]`; without a contract, error box 0xf5 and
/// nothing): person id +0, club +4, 0 at +8, wage +0xc = current wage × percent (signed) / 100, today's date (0xae2c90)
/// at +0x25, the end at +0x2d, the bits +0x35 (& 0xc6 | 6), the wage also at +0x36, 0 at +0x3a and +0x49, −1 at +0x4a,
/// the `reset_contract_morale`, 0 at +0x3b (16 bits), +0x1c..=+0x20 and +0x21, −1 at +0x10/+0x14/+0x18 and the clauses:
/// +0x1c no access, +0x1d minimum fee. The date copies (`exe::date_operator_3d`, with no other effect) become 8-byte copies.
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
pub(crate) unsafe extern "thiscall" fn init_from_current_contract(this: usize, person: usize, club: usize, end: usize, write_percent_pair: u32, without_promotion: u32, multa: u32) {
    let this = Contract(this);
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    if c.is_null() {
        show_error_at(0xa8360c, 0xf5);
        return;
    }
    crate::comparison::branch("cpl_5 contract by the current one");
    write_usize(this.addr(), read_usize(person));
    this.set_unknown((0) as u32);
    this.set_club(read_usize(club));
    let salary = ((c.wage() as usize) as i32).wrapping_mul(write_percent_pair as u8 as i8 as i32) / 100; // the salary percentage
    this.set_wage((salary as usize) as i32);
    core::ptr::write_unaligned((this.addr() + contract::DATE_STARTED) as *mut [u8; 8], core::ptr::read_unaligned(G_CURRENT_DATE as *const [u8; 8])); // hoje
    core::ptr::write_unaligned((this.addr() + contract::CONTRACT_EXPIRES) as *mut [u8; 8], core::ptr::read_unaligned(end as *const [u8; 8])); // the end of the contract
    let b = (this.contract_type() & 0xc6) | 6;
    this.set_previous_wage((this.wage() as usize));
    this.set_contract_type(b);
    this.set_squad_number(0);
    this.set_leaving_on_bosman(0);
    this.set_transfer_arranged_for(usize::MAX);
    reset_contract_morale(this, person, Club(club));
    this.set_club_appearances((0) as i16);
    for k in 0x1c..=0x20 {
        write_u8(this.addr() + k, 0);
    }
    this.set_goal_bonus((usize::MAX) as i32);
    this.set_assist_bonus((usize::MAX) as i32);
    this.set_clean_sheet_bonus((usize::MAX) as i32);
    this.set_release_fee(0);
    this.set_non_promotion_rc(without_promotion as u8);
    this.set_minimum_fee_rc(multa as u8);
}

/// Returns the 0..=14 band of the contract end date against today and the CONTRACT_MANAGER limit dates
/// (0xae1010..0xae1090).
///
/// `STAFF_CONTRACT::get_expiry_band()`: the 0..=14 band of the contract end (+0x2d) against today's date (0xae2c90) and
/// the 16 CONTRACT_MANAGER dates (0xae1010..0xae1090, built at creation): type (+0x35 & 0x3f) 1 or the end already
/// past, 14; type other than 2, 0; after `[0xae1090]`, 0; the intervals between
/// 0xae1040/0xae1060/0xae1010/0xae1058/0xae1030 give the pairs 1-2, 3-4, 5-6, 7-8, 9-10 (the odd one on the exact day
/// of the upper limit: `exe::date_operator_3d_3d`) and, before 0xae1030, 11 on the day, 13 on the day of 0xae1048, otherwise 12. "After"
/// via `exe::date_operator_3e`. TapeCall on every contract in the daily pass (0x4c7ac0): no counter.
/// Original: `STAFF_CONTRACT::get_expiry_band` (0x867d30, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x867d30 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::get_expiry_band()`: a faixa 0..=14 do fim do contrato
/// (+0x2d) contra as datas de hoje (0xae2c90) e as 16 datas do CONTRACT_MANAGER (0xae1010..0xae1090, montadas na
/// criação): tipo (+0x35 & 0x3f) 1 ou fim já passado, 14; tipo diferente de 2, 0; depois de [0xae1090], 0; os
/// intervalos entre 0xae1040/0xae1060/0xae1010/0xae1058/0xae1030 dão os pares 1-2, 3-4, 5-6, 7-8, 9-10 (o ímpar no dia
/// exato do limite de cima: 0x4a7f20) e, antes de 0xae1030, 11 no dia, 13 no dia de 0xae1048, senão 12. "Depois"
/// pela 0x456740. TapeCall a cada contrato no passe diário (0x4c7ac0): sem contador.
pub(crate) unsafe extern "thiscall" fn get_expiry_band(this: usize) -> u8 {
    let this = Contract(this);
    let end = this.addr() + contract::CONTRACT_EXPIRES;
    let m = |g: usize| exe::date_operator_3e(end, read_usize(g) as u32, read_usize(g + 4) as u32) != 0;
    let q = |g: usize| exe::date_operator_3d_3d(end, read_usize(g) as u32, read_usize(g + 4) as u32) != 0;
    if this.contract_type() & 0x3f == 1 {
        return 14;
    }
    if !m(G_CURRENT_DATE) {
        return 14;
    }
    if this.contract_type() & 0x3f != 2 {
        return 0;
    }
    if m(G_CONTRACT_DATE_PLUS_182_DAYS) {
        return 0;
    }
    if m(G_CONTRACT_DATE_PLUS_91_DAYS) { // past the midpoint limit
        if m(G_CONTRACT_DATE_PLUS_121_DAYS) {
            if m(G_CONTRACT_DATE_PLUS_152_DAYS) {
                return if q(G_CONTRACT_DATE_PLUS_182_DAYS) { 1 } else { 2 };
            }
            return if q(G_CONTRACT_DATE_PLUS_152_DAYS) { 3 } else { 4 };
        }
        return if q(G_CONTRACT_DATE_PLUS_121_DAYS) { 5 } else { 6 };
    }
    if m(G_CONTRACT_DATE_PLUS_30_DAYS) {
        if m(G_CONTRACT_DATE_PLUS_60_DAYS) {
            return if q(G_CONTRACT_DATE_PLUS_91_DAYS) { 7 } else { 8 };
        }
        return if q(G_CONTRACT_DATE_PLUS_60_DAYS) { 9 } else { 10 };
    }
    if q(G_CONTRACT_DATE_PLUS_30_DAYS) {
        return 11;
    }
    if q(G_CONTRACT_DATE_PLUS_7_DAYS) { 13 } else { 12 }
}

/// Memory regions the two morale-reset routines write: the contract (0x50 bytes), the person's 79-byte staff state
/// record and 0xb67a34.
///
/// [pt] As regiões que as duas primeiras gravam: o contrato (0x50 bytes), o registro de 0x4f bytes da pessoa de id `id`
/// em [0xae2d0c] (os vínculos da 0x549c90, se o id está na tabela) e [0xb67a34] (a caixa).
pub(crate) unsafe fn morale_reset_regions(this: usize, id: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(this, 0x50), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let id = read_usize(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE))) as i32;
    if id >= 0 && id < read_usize(G_STAFF_COUNT) as i32 {
        r.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f)), 0x4f));
    }
    r
}

/// Person record (110 bytes in G_STAFF_TABLE) of the contract's id (+0).
///
/// [pt] A pessoa do contrato: o id em +0 no vetor de pessoas [0xae23bc] (110 bytes cada).
pub(crate) unsafe fn staff_of_contract(contract: Contract) -> usize {
    (read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contract.addr()) as usize).wrapping_mul(110))
}

/// Returns the days from the game date to the contract end date (+0x2d), through `exe::date_days_since`.
///
/// `STAFF_CONTRACT::days_to_end_date`: the days from the game date (0xae2c90) to the contract end date (+0x2d), through
/// `exe::date_days_since` (`dias_ate`, the game date by value). The game reads only the word; it returns `exe::date_days_since`'s whole eax.
/// Original: `STAFF_CONTRACT::days_to_end_date` (0x85fa00, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x85fa00 } (thiscall, staff_contracts.cpp): `STAFF_CONTRACT::days_to_end_date`: os dias da data do jogo
/// (0xae2c90) até a data do fim do contrato (+0x2d), pela 0x54cdb0 (`dias_ate`, a data do jogo por valor). O jogo lê
/// só a palavra; devolve o eax inteiro da 0x54cdb0.
pub(crate) unsafe extern "thiscall" fn days_to_end_date(this: usize) -> u32 {
    let this = Contract(this);
    (exe::date_days_since(this.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) as u32) // the contract end date
}

/// Clears the grievances (+0x45), the opinion links (`staff_clear_opinion_links`) and, when asked, checks the withdrawal (`check_transfer_request_withdrawal`).
///
/// `STAFF_CONTRACT::clear_all_grievances(check)`: clears the grievances (+0x45), removes the person's opinion links
/// (`staff_clear_opinion_links`) and, with `check`, checks the withdrawal of the transfer request (`check_transfer_request_withdrawal` with 0).
/// Original: `STAFF_CONTRACT::clear_all_grievances` (0x867520, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x867520 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::clear_all_grievances(conferir)`: zera as
/// queixas (+0x45), tira os vínculos de opinião da pessoa (0x549c90) e, com `conferir`, confere a retirada do pedido de
/// transferência (0x867240 com 0).
pub(crate) unsafe extern "thiscall" fn clear_all_grievances(this: usize, check: u32) {
    let this = Contract(this);
    this.set_grievances(0); // the complaints
    exe::staff_clear_opinion_links(staff_of_contract(this));
    if check != 0 {
        exe::staff_contract_check_transfer_request_withdrawal(this, 0);
    }
}

/// The decision of `check_transfer_request_withdrawal` (no effect besides the sheet [0xae1080], written with the person's contract): returns the
/// person when their transfer request should be withdrawn, else None.
///
/// Conditions: the contract's club (outside type 5) exists, the person is a player (+0x61), the contract is with that
/// club (+4 = id), they requested a transfer (bit 3 of +0x4e), not in +0x49, the staff criterion (`staff_criterion`) gives 0,
/// the high nibble of +0x4f is not 7, and there is no agreed contract with another club (`get_agreed_contract_club` on [0xde1f54]).
///
/// [pt] A decisão da 0x867240 (sem efeito além da ficha [0xae1080], gravada com o contrato da pessoa): o clube do contrato
/// (fora do tipo 5) existe, a pessoa é jogador (+0x61), o contrato é desse clube (+4 = id), pediu transferência (bit 3
/// de +0x4e), não está em +0x49, o critério da comissão (0x866f60) dá 0, o nibble alto de +0x4f não é 7 e não há
/// contrato acertado com outro clube (0x8c5450 em [0xde1f54]). Devolve a pessoa quando retira.
pub(crate) unsafe fn should_withdraw_transfer_request(this: Contract) -> Option<usize> {
    let person = Staff(staff_of_contract(this));
    let c = Contract(exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, person));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c.addr(); // the contract record
    let club = if !c.is_null() && c.contract_type() & 0x3f != 5 {
        let i = (c.club() as u32) as usize;
        (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
    } else {
        0
    };
    if club == 0 || (person.player() as u32) == 0 || (this.club() as u32) != read_u32(club) {
        return None;
    }
    if this.transfer_status() & 8 == 0 || this.leaving_on_bosman() != 0 {
        return None; // no transfer request
    }
    if exe::staff_contract_wants_to_leave(this) != 0 || this.squad_status() & 0xf0 == 0x70 {
        return None;
    }
    if exe::transfer_manager_get_agreed_contract_club(read_u32(G_TRANSFER_MANAGER) as usize, person, 0) != 0 {
        return None;
    }
    Some(person.addr())
}

/// For a player of the contract's club with a transfer request, no staff criterion and no agreed contract: withdraws
/// the request (`post_transfer_request_withdrawn_news`), morale becomes +0x5b + +0x59 + 5, +0x3e = 0, +0x3d at least 15, and clears each grievance,
/// checking again.
///
/// `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`: when the decision (`should_withdraw_transfer_request`)
/// says so, withdraws the request (`post_transfer_request_withdrawn_news` on `[0xae1050]` with the contract and `ctx`), clears bit 3 of +0x4e, the
/// happiness in the contract (+0x44) becomes the person's +0x5b + +0x59 + 5, clears +0x3e and raises +0x3d to 15; and
/// each grievance that is set (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 of +0x45, in that order) is cleared, checks
/// again (`check_transfer_request_withdrawal` with the previous +0x45) and removes the opinion link (`staff_remove_opinion_link` with the bit). No SEH, no error box,
/// no random draw.
/// Original: `STAFF_CONTRACT::check_transfer_request_withdrawal` (0x867240, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x867240 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`:
/// quando a decisão (`should_withdraw_transfer_request`) manda, retira o pedido (0x4d9460 em [0xae1050] com o contrato e `ctx`), apaga o
/// bit 3 de +0x4e, a felicidade no contrato (+0x44) vira +0x5b + +0x59 + 5 da pessoa, zera +0x3e e sobe +0x3d a 15; e cada queixa
/// acesa (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 de +0x45, nessa ordem) é apagada, confere de novo (0x867240 com o
/// +0x45 de antes) e tira o vínculo de opinião (0x549d60 com o bit). Sem SEH, sem caixa, sem sorteio.
pub(crate) unsafe extern "thiscall" fn check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let this = Contract(this);
    let Some(pessoa) = should_withdraw_transfer_request(this) else { return };
    exe::contract_manager_post_transfer_request_withdrawn_news(read_u32(G_CONTRACT_MANAGER) as usize, this, ctx);
    this.set_transfer_status(this.transfer_status() & 0xf7); // o pedido retirado
    this.set_morale((read_u8(pessoa + staff::PROFESSIONALISM).wrapping_add(read_u8(pessoa + staff::LOYALTY)).wrapping_add(5)) as i8);
    if (this.unhappy_flag() as u8) != 0 {
        this.set_unhappy_flag((0) as i8);
    }
    if ((this.happiness() as u8) as i8) < 15 {
        this.set_happiness((15) as i8); // the minimum happiness
    }
    for bit in [0x10u32, 1, 4, 8, 0x100, 0x80, 0x10000] {
        let q = this.grievances();
        if q & bit != 0 {
            this.set_grievances(q & !bit); // a queixa apagada
            exe::staff_contract_check_transfer_request_withdrawal(this, q);
            exe::staff_remove_opinion_link(staff_of_contract(this), bit as i32);
        }
    }
}

// 0x54cdb0 only reads: compare the value
compare_value!(compare_days_to_end_date, days_to_end_date, IDX_DAYS_TO_END_DATE, () -> u32);

/// Sets the flags at +0x40 and, when the person now wants to leave and nothing prevents it, files the transfer request
/// (`handle_transfer_request`).
///
/// `STAFF_CONTRACT::set_flags(flags)`, called by many routines (in motor.rs through its address; before, only the
/// post-match gate detoured it, in comparison mode). Flags above 0x1000: the "Error" box and it carries on (here: the
/// whole original). Sets the flags in +0x40; a person (+0 id) who is not of functions 0xc, 0xd and 0xf, a contract with
/// +0x44 positive, without +0x49, who wants to leave (`staff_criterion`), without a future transfer (`get_future_transfer` with 0), without
/// bits 1 and 8 of +0x4e, outside the free transfer queue (`person_in_transfer_list`) and older than 20 (+0x18) or with current
/// reputation (the player's +0xb) above 4250: the transfer request (`handle_transfer_request` on `[0xae1050]`). Does not read
/// uninitialised locals.
/// Original: `STAFF_CONTRACT::set_flags` (0x867560, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x867560 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_flags(marcas)`, chamada por muitas rotinas
/// (no motor.rs pelo endereço; antes só a porta do pós-jogo a desviava, no modo comparação). Marcas acima de 0x1000: a
/// caixa "Error" e segue (aqui: a original inteira). Liga as marcas em +0x40; pessoa (+0 id) que não é das funções
/// 0xc, 0xd e 0xf, contrato com +0x44 positivo, sem +0x49, que quer sair (0x866f60), sem transferência futura
/// (0x8cca10 com 0), sem os bits 1 e 8 de +0x4e, fora da fila da transferência livre (0x8c1870) e com mais de 20 anos
/// (+0x18) ou reputação atual (+0xb do jogador) acima de 4250: o pedido de transferência (0x4d8610 em [0xae1050]).
/// Não lê local sem valor.
pub(crate) unsafe extern "thiscall" fn set_flags(c: usize, marks: u32) {
    let c = Contract(c);
    if marks as i32 > 0x1000 {
        // simplification: the ”Error” box (before any write): the whole original
        return original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SET_FLAGS)(c.addr(), marks);
    }
    c.set_flags((c.flags() as usize) as u32 | marks);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(c.addr()) as usize).wrapping_mul(staff::SIZE)));
    let function = person.job_for_club();
    if function == 0xc || function == 0xd || function == 0xf {
        return;
    }
    let morale = (c.morale() as u8) as i8;
    if morale == 0 || morale < 0 || c.leaving_on_bosman() != 0 {
        return;
    }
    if exe::staff_contract_wants_to_leave(c) == 0 {
        return;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 || c.transfer_status() & 9 != 0 {
        return;
    }
    if exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), person) != 0 {
        return;
    }
    if (person.age() as u8) as i8 <= 0x14 && read_i16(person.player() + 0xb) <= 0x109a {
        return;
    }
    crate::comparison::branch("cpw_7 transfer request");
    if MISS_TRAINING_DRY_RUN {
        MISS_TRAINING_DECISION = 1;
        return;
    }
    exe::contract_manager_handle_transfer_request(read_usize(G_CONTRACT_MANAGER), c);
}

/// Returns 1 to 3 from 100 × the value of the current contract / that of the desired one (with the deserved squad
/// status), with an allowance for recent signings; below 50: 1; below 80: 3; otherwise 2.
///
/// SEH frame with no destructor. `STAFF_CONTRACT::get_contract_satisfaction(person)`, called from the staff's daily
/// pass (0x5ce2d0, with random(50) zero) and from the contract screen: 1 to 3. No person, 1. The squad status they
/// deserve (`contract_category`) is used if it is lower than the contract's (+0x4f >> 4) and is neither 0 nor 7; otherwise, the
/// contract's. The desired contract (0x85fc50 into a local contract, with the contract's club, −1, the status and 0xb)
/// and the ratio 100 × value of the current contract (`exe::staff_contract_calc_contract_value`, as f32) / value of the desired one (zero value: ratio
/// 0). Signed less than 15 × adaptability (+0x59) + 100 days ago (from +0x25, ceiling 500): ratio + 50 + 7.5 ×
/// adaptability − days / 2. Ratio below 50: 1; below 80: 3; otherwise 2.
/// Original: `get_contract_satisfaction` (0x866620, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x866620 } (thiscall, ret 4, staff_contracts.cpp, quadro SEH sem destrutor): `STAFF_CONTRACT::
/// get_contract_satisfaction(pessoa)`, pelo passe diário do pessoal (0x5ce2d0, com sorteio(50) zero) e pela tela do
/// contrato: 1 a 3. Sem pessoa, 1. A situação no elenco que ela merece (0x4ca470) vale se for menor que a do contrato
/// (+0x4f >> 4) e não for 0 nem 7; senão, a do contrato. O contrato desejado (0x85fc50 num contrato local, com o clube
/// do contrato, −1, a situação e 0xb) e a razão 100 × valor do contrato atual (0x866830, em f32) / valor do desejado
/// (valor zero: razão 0). Contratado há menos de 15 × adaptabilidade (+0x59) + 100 dias (de +0x25, teto 500): razão +
/// 50 + 7,5 × adaptabilidade − dias / 2. Razão abaixo de 50: 1; abaixo de 80: 3; senão 2.
pub(crate) unsafe extern "thiscall" fn get_contract_satisfaction(this: usize, person: usize) -> u8 {
    let person = Staff(person);
    if person.is_null() {
        return 1;
    }
    let category: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    // the local contract (0x50 bytes) as the original leaves it before 0x85fc50; it fills in the rest
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    exe::date_date_54a110(l + contract::DATE_STARTED);
    exe::date_date_54a110(l + contract::CONTRACT_EXPIRES);
    core::ptr::write_unaligned((l + contract::CLUB) as *mut i32, -1);
    core::ptr::write_unaligned((l + contract::TRANSFER_ARRANGED_FOR) as *mut i32, -1);
    let merece = category(person.addr(), this);
    let current = read_u8(this + contract::SQUAD_STATUS) >> 4;
    // simplification: the original pushes the whole dword of the slot where it stored the byte (in the first case, the rest is the pointer
    // of the person; in the second, stack leftover); 0x85fc50 only reads the byte.
    let status = if (merece as i8) < (current as i8) && merece != 0 && merece != 7 {
        (person.addr() as u32 & !0xff) | merece as u32
    } else {
        current as u32
    };
    exe::staff_contract_set_to_desired_contract(l, person, friendly_club_by_id(read_usize(this + contract::CLUB)), -1, status, 0xb, 0, 0);
    let v1 = calc_contract_value(this, person, l) as f32;
    let v2 = calc_contract_value(l, person, l) as f64;
    let mut ratio = if v2 == read_f32(rdata::F32_0) as f64 {
        read_f32(rdata::F32_0) as f64
    } else {
        v1 as f64 * read_f32(rdata::F32_100) as f64 / v2
    };
    let years = (read_i16(G_CURRENT_DATE_YEAR) as u16 as i32).wrapping_sub(read_i16(this + 0x27) as u16 as i32);
    let days = years.wrapping_mul(365).wrapping_sub(read_i16(this + contract::DATE_STARTED) as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32).clamp(0, 500);
    let adapt = (person.loyalty() as u8) as i8 as i32;
    if days < adapt.wrapping_mul(15).wrapping_add(100) {
        crate::comparison::branch("cpw_1 recently signed");
        ratio = ratio + read_f32(rdata::F32_50) as f64 + adapt as f64 * read_f64(rdata::F64_7_5) - (days / 2) as f64; // cpw1: the discount of the newly signed;
    }
    crate::comparison::branch("cpw_1 satisfaction evaluated");
    if !(ratio >= read_f32(rdata::F32_50) as f64) { // cpw1: the dissatisfaction limit;
        return 1;
    }
    if ratio < read_f32(rdata::F32_80) as f64 { // cpw1: the satisfaction limit;
        3
    } else {
        2
    }
}

/// Sets the wage (+0xc) to the default for the person at the club (`exe::staff_contract_calc_unset_default_wage`), truncated, minimum 55.
///
/// `STAFF_CONTRACT::set_default_wage(person)`: the wage (+0xc) is the person's default at their club (`[+0x39]`) from
/// `exe::staff_contract_calc_unset_default_wage` (stdcall, 6 arguments, float; ecx goes along, as in the original), truncated (0x945640), with a minimum of
/// 55. Returns the truncated value (before the minimum).
/// Original: `STAFF_CONTRACT::set_default_wage` (0x8626e0, staff_contracts.cpp, thiscall, ret 4).
///
/// [pt] { 0x8626e0 } (thiscall, ret 4, staff_contracts.cpp): `STAFF_CONTRACT::set_default_wage(pessoa)`: o salário (+0xc) é
/// o padrão da pessoa no clube dela ([+0x39]) pela 0x862720 (stdcall, 6 argumentos, float; o ecx vai junto, como na
/// original), truncado (0x945640), com mínimo 55. Devolve o truncado (antes do mínimo).
pub(crate) unsafe extern "thiscall" fn set_default_wage(this: usize, person: usize) -> u32 {
    let this = Contract(this);
    let person = Staff(person);
    let v = ftol(exe::staff_contract_calc_unset_default_wage(this, person, person.club_job(), 1, -1, 0xb, 0));
    this.set_wage(if v < 0x37 { 0x37 } else { v }); // the minimum of 55
    v as u32
}

/// +0x40 (a 4-byte routine; the hook covers the first alignment nop)
/// Original: `contract_flags` (0x8676f0, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x8676f0 } (staff_contracts.cpp): as bandeiras do contrato (+0x40).
pub(crate) unsafe extern "thiscall" fn contract_flags(this: usize) -> u32 {
    let this = Contract(this);
    this.flags()
}

/// the empty contract: default dates at +0x25 and +0x2d, +4 and +0x4a = -1, +8, +0x3b, +0x40, +0x45, +0x49 and +0x4e =
/// 0
/// Original: `STAFF_CONTRACT::STAFF_CONTRACT` (0x85e410, staff_contracts.cpp, thiscall, ret 0).
///
/// [pt] { 0x85e410 } (thiscall, ret 0, staff_contracts.cpp): `STAFF_CONTRACT::STAFF_CONTRACT()`: as duas datas (+0x25,
/// +0x2d) padrão (0x54a110), +4 e +0x4a = −1, +0x49, +0x3b, +0x4e, +0x40, +0x45 e +8 = 0. Devolve o próprio contrato.
pub(crate) unsafe extern "thiscall" fn construct_staff_contract(this: usize) -> usize {
    let this = Contract(this);
    exe::date_date_54a110(this.addr() + contract::DATE_STARTED);
    exe::date_date_54a110(this.addr() + contract::CONTRACT_EXPIRES);
    this.set_club((u32::MAX) as usize);
    this.set_transfer_arranged_for((u32::MAX) as usize);
    this.set_leaving_on_bosman(0);
    this.set_club_appearances((0) as i16);
    this.set_transfer_status(0);
    this.set_flags(0);
    this.set_grievances(0);
    this.set_unknown(0);
    this.addr()
}

/// The staff criterion: an 8-bit score (id mod 7, ratings, age, reputation, number of roles) decides, by the roles,
/// whether it returns 1.
///
/// The criterion of staff member `t` (person `[t]` in `[0xae23bc]`, 0x6e bytes; club `[t+4]` in `[0xae23b4]`, 0x245
/// bytes; functions in `[t+0x45]`): 0 if the person is the club's manager or assistant (+0xcf, +0xd3) or has no
/// function; otherwise an 8-bit score (id mod 7 + +0x59 + +0x5a + +0x5b, deductions for low ratings and for age, +0x44
/// in bands, the club's reputation, minus the number of functions above one) decides, by the functions and the ratings,
/// whether it returns 1.
/// Original: `staff_criterion` (0x866f60, staff_contracts.cpp, thiscall).
///
/// [pt] { 0x866f60 } (thiscall, staff_contracts.cpp): o critério da comissão técnica `t` (pessoa [t] em
/// [0xae23bc], 0x6e bytes; clube [t+4] em [0xae23b4], 0x245 bytes; funções em [t+0x45]): 0 se a
/// pessoa é o técnico ou o auxiliar do clube (+0xcf, +0xd3) ou não tem função; senão uma nota em 8
/// bits (id mod 7 + +0x59 + +0x5a + +0x5b, descontos pelas notas baixas e pela idade, +0x44 em
/// faixas, a reputação do clube, e menos o número de funções acima de uma) decide pelas funções e
/// pelas notas se devolve 1.
pub(crate) unsafe extern "thiscall" fn staff_criterion(t: usize) -> u32 {
    let t = Contract(t);
    let p = Staff((*(G_STAFF_TABLE as *const usize)).wrapping_add((*(t.addr() as *const i32) as usize).wrapping_mul(staff::SIZE)));
    let club = Club((*(G_CLUB_TABLE as *const usize)).wrapping_add(((t.club() as i32) as usize).wrapping_mul(club::SIZE)));
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let manager = club.manager().addr();
    if manager != 0 && p.addr() == manager {
        return 0;
    }
    let auxiliar = club.assistant_manager();
    if auxiliar != 0 && p.addr() == auxiliar {
        return 0;
    }
    let funcoes = u(t.addr() + contract::GRIEVANCES) as u32;
    if funcoes == 0 {
        return 0;
    }
    let b = |o: usize| *((p.addr() + o) as *const i8);
    let (b59, b5a, b5b, b5d) = (b(staff::LOYALTY), b(staff::PRESSURE), b(staff::PROFESSIONALISM), b(staff::TEMPERAMENT));
    let b44 = t.morale();
    let mut rating = ((*(p.addr() as *const i32) % 7) as u8).wrapping_add(b59 as u8).wrapping_add(b5a as u8).wrapping_add(b5b as u8);
    if b5a == 1 {
        rating = rating.wrapping_sub(2);
    }
    if b59 == 1 {
        rating = rating.wrapping_sub(3);
    } else if b59 < 6 {
        rating = rating.wrapping_sub(1);
    }
    if b5b == 1 {
        rating = rating.wrapping_sub(3);
    } else if b5b < 6 {
        rating = rating.wrapping_sub(1);
    }
    if b5d == 1 {
        rating = rating.wrapping_sub(2);
    }
    if b44 > 0x4b {
        rating = rating.wrapping_add(10);
    } else if b44 > 0x32 {
        rating = rating.wrapping_add(5);
    } else if b44 > 0 {
        rating = rating.wrapping_add(2);
    }
    let sheet = p.player();
    if sheet != 0 {
        let rep = core::ptr::read_unaligned((sheet + player_sheet::CURRENT_REPUTATION) as *const i16);
        if rep > 0x1c52 {
            rating = rating.wrapping_sub(5);
        } else if rep > 0x1676 {
            rating = rating.wrapping_sub(3);
        }
    }
    let tecnica = t.flags() & 8 != 0;
    let m1 = funcoes & 0x1000002;
    let mut k = 0u8;
    for bits in [0x20000u32, 0x40000, 0x1000002, 0x200600, 0x2840, 0x14] {
        if funcoes & bits != 0 {
            k += 1;
        }
    }
    if tecnica || funcoes & 0x4582208 != 0 {
        k += 1;
    }
    if k > 1 {
        rating = rating.wrapping_sub(3);
        if b5b < 6 {
            rating = rating.wrapping_sub(k);
        }
        if b5d < 6 {
            rating = rating.wrapping_sub(k);
        }
        if b59 == 1 {
            rating = rating.wrapping_add((k as i8).wrapping_mul(-2) as u8);
        } else if b59 < 6 {
            rating = rating.wrapping_sub(k);
        }
        if b5a < 6 {
            rating = rating.wrapping_sub(k);
        }
    }
    let rating = rating as i8;
    let yes = (b59 < 15 && t.transfer_status() & 1 != 0)
        || (m1 != 0 && b59 < 10 && rating < 25)
        || (funcoes & 0x20020 != 0 && rating < 25)
        || (funcoes & 0x200 != 0 && rating < 18)
        || (funcoes & 4 != 0 && b(staff::AMBITION) > 15 && b5b < 10)
        || (funcoes & 0x2808 != 0 && b59 < 10 && b5a < 6 && rating < 30)
        || ((funcoes & 0x82200 != 0 || tecnica) && b5b < 10 && rating < 20)
        || funcoes & 0x111 != 0
        || (b44 as i32) < -0x46 - b59 as i32;
    yes as u32
}
