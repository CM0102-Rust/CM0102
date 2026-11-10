use super::*;

/// Common start of the Scottish/Irish/Spanish loan rules: None (allowed) without contract or contract club, else the
/// owning club via a second contract lookup (0 if not found).
///
/// [pt] O começo comum das regras escocesa, irlandesa e espanhola: sem contrato do jogador (ou sem clube do contrato),
/// `None` (pode); senão o clube dono pelo contrato consultado de novo (0 se a segunda consulta não achar).
pub(crate) unsafe fn loan_owner_club(player: usize) -> Option<usize> {
    let c = game_get_contract_and_store(player);
    if c == 0 || england_rules_club_of_contract(Contract(c)) == 0 {
        return None;
    }
    let c2 = game_get_contract_and_store(player);
    Some(if c2 != 0 { england_rules_club_of_contract(Contract(c2)) } else { 0 })
}

/// Loan refusal "outside the window" (reason 9) with the rules nation's adjective (+0xa) in the message.
///
/// [pt] A recusa "fora da janela" (motivo 9), com o adjetivo da nação das regras (+0xa).
pub(crate) unsafe fn refuse_outside_window(this: usize, msg: usize, reason: *mut u16) -> u32 {
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nation; add esp, 0x10 -> cdecl
        let adjective: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_NATION_GET_NATIONALITY_ADJECTIVE);
        // 0x66f4e0: push ...; push format; push destination; add esp, 0x14 -> variadic cdecl
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let mut buf = [0u8; 100];
        adjective(read_usize(this + 0xa), buf.as_mut_ptr(), 100, 2);
        text(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

/// Loan refusal with reason m and text formato built from the record's byte +0x37 and name at +4.
///
/// [pt] Uma recusa com o texto `formato` montado com o byte +0x37 (duas vezes) e o nome em +4 do registro `r`.
pub(crate) unsafe fn refuse_loan(msg: usize, reason: *mut u16, formato: usize, r: usize, m: u16) -> u32 {
    if msg != 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let g = read_u8(r + 0x37) as i8 as i32;
        text(msg, formato, g, g, r + 4);
    }
    if !reason.is_null() {
        *reason = m;
    }
    0
}

/// refuses (0x1d) between two clubs of the same Scottish division
/// Original: `SCOTLAND_RULES::can_loan_player` (0x7f6650, scotland_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7f6650 } (thiscall, ret 0x10, scotland_rules.cpp): `SCOTLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`: com contrato, o clube que pede e o dono na mesma divisão escocesa ([0x9cf5dc]) não podem (0x1d).
pub(crate) unsafe extern "thiscall" fn scotland_rules_can_loan_player(_this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let club = Club(club);
    let owner = Club(match loan_owner_club(player) {
        None => return 1,
        Some(d) => d,
    });
    let comp = club.division().addr();
    if comp == owner.division().addr() && comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
        crate::comparison::branch("cqe_6 scotland same division");
        let r = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION).wrapping_mul(0x6b));
        return refuse_loan(msg, reason, 0x9c4590, r, 0x1d);
    }
    1
}

/// Returns the Scottish loan period: it starts next month; under 22 it ends in the season end exe::date_get_month(this year or
/// next), otherwise three months later.
///
/// `SCOTLAND_RULES::get_loan_period(a1, a2, start_month, start_year, end_month, end_year, player)`: it starts in the
/// following month; the end is the season end month of nation a1 (`contract_manager_season_end_month`) for those under 22 (+0x18), this year if
/// the start comes before it, otherwise the following year; for the others, three months after the start.
/// Original: `SCOTLAND_RULES::get_loan_period` (0x7f6530, scotland_rules.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x7f6530 } (thiscall, ret 0x1c, scotland_rules.cpp): `SCOTLAND_RULES::get_loan_period(a1, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`: começa no mês seguinte; o fim é o mês do fim de temporada da nação a1
/// (0x4d45f0) para quem tem menos de 22 anos (+0x18), neste ano se o começo vem antes dele, senão no ano seguinte;
/// para os outros, três meses depois do começo.
pub(crate) unsafe extern "thiscall" fn scotland_rules_get_loan_period(_this: usize, a1: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let player = Staff(player);
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    // 0x4d45f0: push a1; mov ecx, [0xae1050] -> thiscall, ret 4 (the month the season ends)
    let e = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), a1) as i8;
    *end_month = e;
    if ((player.age() as u8) as i8) < 0x16 {
        crate::comparison::branch("cqe_6 scotland until end of season");
        if *start_month < e {
            *end_year = *start_year;
        } else {
            *end_year = read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1);
        }
        return;
    }
    *end_month = (*start_month).wrapping_add(3);
    *end_year = *start_year;
    if *end_month > 0xb {
        *end_month = (*end_month).wrapping_sub(0xc);
        *end_year = (*end_year).wrapping_add(1);
    }
}

/// Constructs the Scottish transfer rules (rule 21): one window, from 30 May (moved to the Saturday [inference]) to 30
/// March (moved to the Thursday [inference]); dword +0x15 = 1.
///
/// `SCOTLAND_RULES::SCOTLAND_RULES`; SEH frame only for the `new`, no destructor on the normal path. The Scotland
/// transfer rules (rule 21): the base (`exe::new_transfer_rules`), the vtable 0x96f838, 1 window from `SCOTLAND_TRANSFER_WINDOWS` and
/// dword +0x15 = 1. Returns the object.
/// Original: `SCOTLAND_RULES::SCOTLAND_RULES` (0x7f6390, scotland_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x7f6390 } (thiscall, ret 0x14, scotland_rules.cpp, `SCOTLAND_RULES::SCOTLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Escócia (regra 21): a base (0x8e84a0), a vtable 0x96f838, 1 janela(s) de `SCOTLAND_TRANSFER_WINDOWS` e o dword +0x15 = 1. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_scotland_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_3 scotland");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x96f838, 1, &SCOTLAND_TRANSFER_WINDOWS, 0xa7b058, 0x50);
    write_u32(this + 0x15, 1);
    this
}

