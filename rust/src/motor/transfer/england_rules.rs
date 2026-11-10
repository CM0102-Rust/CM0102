use super::*;

/// Sets the transfer windows: `+8` = the number of windows, and the table allocated into `+4`.
///
/// The table (`t`) holds 6-byte dates: `+0` the rule, `+1` the window number, `+2` the weekday the date moves to (0xff
/// none) [inference], `+3` the day, `+4` the month (0 = January), `+5` 1 at the start of the window and 0 at the end;
/// two dates per window. Allocated via `0x944e46`; out of memory, the box of line `linha` of source file `arquivo`.
///
/// [pt] As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
pub(crate) unsafe fn set_transfer_windows(this: usize, n: u8, t: &[[u8; 6]], file: usize, row: u32) {
    *((this + 8) as *mut u8) = n;
    let j = exe::crt_malloc((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        show_error_at(file, row);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// Side-by-side comparer for transfer rules constructors: original on the object, copy on a scratch copy; fields and
/// windows must match.
///
/// [pt] Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
pub(crate) unsafe fn compare_transfer_rules_ctor(name: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let fields = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (read_u32(this + 4) as usize, read_u32(r + 4) as usize);
    let n = s[8] as usize * 12;
    let windows = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        exe::crt_free(jb);
    }
    SCOREBOARD.record(fields && windows, || format!("{}: campos {} janelas {}", name, fields, windows));
}

/// England's transfer window: from 2 June (moved to the Monday [inference]) to 26 March (moved to the Thursday
/// [inference]).
///
/// [pt] A janela da Inglaterra: de 2 de junho (andando para a segunda-feira [inferência]) a 26 de março (andando para a quinta-feira [inferência]).
pub(crate) const ENGLAND_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x07, 0x00, 0x01, 0x02, 0x05, 0x01], // cqk1: England opens in June
    [0x07, 0x00, 0x04, 0x1a, 0x02, 0x00],
];

/// the English rule (7): one transfer window, from 2 June (moved to the Monday [inference]) to 26 March (moved to the
/// Thursday [inference]); England's other rules (non-EU players, loans: 5 a season and 2 at once, same league) live in
/// the vtable 0x969f4c methods
/// Original: `ENGLAND_RULES::ENGLAND_RULES` (0x579b30, england_rules.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x579b30 } (thiscall, ret 0x14, england_rules.cpp, `ENGLAND_RULES::ENGLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Inglaterra (regra 7): a base (0x8e84a0), a vtable 0x969f4c, 1 janela(s) de `ENGLAND_TRANSFER_WINDOWS`. Devolve o objeto.
pub(crate) unsafe extern "thiscall" fn construct_england_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparison::branch("cqk_1 england");
    construct_nation_rules(this, [a1, a2, a3, a4, a5], 0x969f4c, 1, &ENGLAND_TRANSFER_WINDOWS, 0x9c4498, 0x4f);
    this
}

/// The USA's transfer window: from 7 January (moved to the Wednesday [inference]) to 15 August.
///
/// [pt] A janela dos EUA: de 7 de janeiro (andando para a quarta-feira [inferência]) a 15 de agosto.
pub(crate) const USA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x19, 0x00, 0x03, 0x07, 0x00, 0x01],
    [0x19, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqk1: the USA close in August
];

/// A janela da Austrália: de 11 de junho (andando para o sábado [inferência]) a 17 de abril.
pub(crate) const AUSTRALIA_TRANSFER_WINDOWS: [[u8; 6]; 2] = [
    [0x02, 0x00, 0x06, 0x0b, 0x05, 0x01], // cqk1: Australia opens in June
    [0x02, 0x00, 0xff, 0x11, 0x03, 0x00],
];

/// Club of a contract: club table + 0x245 x contract +4.
///
/// [pt] O clube de um contrato: +4 é o índice em [0xae23b4] (registros de 0x245 bytes), como as regras calculam.
pub(crate) unsafe fn england_rules_club_of_contract(c: Contract) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE))
}

/// The borrowing club's 16-byte entry in the season loans table ([[0xde1f54] + 0x91c] + id x 16; byte +0 is the count).
///
/// [pt] A entrada de 16 bytes do clube na tabela dos empréstimos da temporada ([[0xde1f54] + 0x91c] + id do clube que pega emprestado × 16, o byte +0
/// é a conta).
pub(crate) unsafe fn season_loans_entry(club: Club) -> usize {
    read_usize(read_usize(G_TRANSFER_MANAGER) + 0x91c).wrapping_add(read_usize(club.addr()).wrapping_mul(16))
}

/// Decides whether an English loan is allowed: refuses (0, with the message and the reason) between clubs of the same
/// English division (0x1d), with five loans this season (0x1e), with more than one player on loan from another club in
/// the squad (0x26) and outside the window (9); otherwise 1.
///
/// `ENGLAND_RULES::can_loan_player(club, player, message, reason)`, through the nation's rules in
/// `TRANSFER_MANAGER::can_loan_player` (`transfer_manager_can_loan_player`). Without a contract (or without the contract's club): allowed. The
/// requesting club and the owner in the same English division (`[0x9cf5bc]`): reason 0x1d. Five loans in the season
/// (the table `[[0xde1f54] + 0x91c]`): 0x1e. More than one player from another club in the squad (the 50 slots at
/// +0xd7, counted only with the player's id below `[0xae2364]`): 0x26. Outside the window (`exe::transfer_rules_find_open_window`): 9, with the
/// nation's adjective.
/// Original: `ENGLAND_RULES::can_loan_player` (0x579d60, england_rules.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x579d60 } (thiscall, ret 0x10, england_rules.cpp): `ENGLAND_RULES::can_loan_player(clube, jogador, mensagem,
/// motivo)`, pelas regras da nação em `TRANSFER_MANAGER::can_loan_player` (0x8d0040). Sem contrato (ou sem clube do
/// contrato): pode. O clube que pede e o dono na mesma divisão inglesa ([0x9cf5bc]): motivo 0x1d. Cinco empréstimos na
/// temporada (a tabela [[0xde1f54] + 0x91c]): 0x1e. Mais de um jogador de outro clube no elenco (as 50 vagas em +0xd7,
/// contados só com o id do jogador abaixo de [0xae2364]): 0x26. Fora da janela (0x8e8e50): 9, com o adjetivo da nação.
pub(crate) unsafe extern "thiscall" fn england_rules_can_loan_player(this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let club = Club(club);
    // 0x66f4e0: the message text (push ...; push format; push destination; add esp, 0x14) -> variadic cdecl
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = game_get_contract_and_store(player);
    if c == 0 || england_rules_club_of_contract(Contract(c)) == 0 {
        return 1;
    }
    let c2 = game_get_contract_and_store(player);
    let owner = Club(if c2 != 0 { england_rules_club_of_contract(Contract(c2)) } else { 0 });
    let comp = club.division().addr();
    if comp == owner.division().addr() && comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) {
        crate::comparison::branch("cqe_5 same division");
        if msg != 0 {
            let id = read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION);
            let r = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)));
            let g = r.gender_name() as i8 as i32;
            text(msg, 0x9c4590, g, g, r.addr() + competition::NAME);
        }
        if !reason.is_null() {
            *reason = 0x1d;
        }
        return 0;
    }
    if read_u8(season_loans_entry(club)) as i8 >= 5 {
        // cqe5: cinco na temporada
        crate::comparison::branch("cqe_5 five in the season");
        if msg != 0 {
            let g = club.short_gender_name() as i8 as i32;
            text(msg, 0x9c4534, g, g, club.addr() + club::SHORT_NAME);
        }
        if !reason.is_null() {
            *reason = 0x1e;
        }
        return 0;
    }
    // 0x4cf6d0: push person; mov ecx, [0xae1050] -> thiscall, ret 4 (the loan contract)
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = club.squad(k);
        if p == 0 {
            continue;
        }
        if player != 0 && read_usize(player) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0 {
            continue;
        }
        let cp = game_get_contract_and_store(p);
        let of = if cp != 0 { england_rules_club_of_contract(Contract(cp)) } else { 0 };
        if of != club.addr() {
            n = n.wrapping_add(1);
        }
    }
    if n > 1 { // cqe5: dois ao mesmo tempo
        crate::comparison::branch("cqe_5 two at the same time");
        if msg != 0 {
            let g = club.short_gender_name() as i8 as i32;
            text(msg, 0x9c44d4, g, g, club.addr() + club::SHORT_NAME);
        }
        if !reason.is_null() {
            *reason = 0x26;
        }
        return 0;
    }
    // 0x8e8e50: push 0; push 0xae2c90; mov ecx, edi (this) -> thiscall, ret 8 (a janela aberta hoje)
    if exe::transfer_rules_find_open_window(this, G_CURRENT_DATE, 0) != 0 {
        return 1;
    }
    crate::comparison::branch("cqe_5 outside the window");
    if msg != 0 {
        // 0x6181d0: push 2; push 100; push buffer; push nation; add esp, 0x10 -> cdecl (the nation’s adjective)
        let mut buf = [0u8; 100];
        exe::nation_get_nationality_adjective(read_usize(this + 0xa), (buf.as_mut_ptr()) as *mut u8 as usize, 100, 2);
        text(msg, 0x987d38, 100, 100, buf.as_ptr());
    }
    if !reason.is_null() {
        *reason = 9;
    }
    0
}

/// adds 1 to the season loan count of the borrowing club when it is English
/// Original: `ENGLAND_RULES::record_loan` (0x579fd0, england_rules.cpp, thiscall, ret 4).
///
/// [pt] { 0x579fd0 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::record_loan(oferta)`, quando um empréstimo se
/// conclui (0x8c9390): o clube da oferta (+8, índice em [0xae23b4]) da nação inglesa ([0x9cf2e4]) soma 1 na conta da
/// temporada.
pub(crate) unsafe extern "thiscall" fn record_loan(_this: usize, offer: usize) {
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE)));
    let nation = club.nation().addr();
    if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_ENGLAND) {
        crate::comparison::branch("cqe_5 loan counted");
        let p = season_loans_entry(club);
        *(p as *mut u8) = read_u8(p).wrapping_add(1); // cqe5: the season calculation
    }
}

/// On 20 June, resets the season loan count of every club of this rule set's nation.
///
/// `ENGLAND_RULES::daily_update(date)`, from the rules' daily pass: on 20 June (only the day of the year, `exe::date_date_549f70` with
/// 1998), the season count of each club (`[0xae235c]` in `[0xae23b4]`) of these rules' nation (+0xa) goes back to zero.
/// Original: `ENGLAND_RULES::daily_update` (0x57a020, england_rules.cpp, thiscall, ret 4).
///
/// [pt] { 0x57a020 } (thiscall, ret 4, england_rules.cpp): `ENGLAND_RULES::daily_update(data)`, pelo dia das regras: em
/// 20 de junho (só o dia do ano, 0x549f70 com 1998), a conta da temporada de cada clube ([0xae235c] em [0xae23b4]) da
/// nação destas regras (+0xa) volta a zero.
pub(crate) unsafe extern "thiscall" fn england_rules_daily_update(this: usize, data: usize) {
    let mut d = [0u8; 8];
    let r = exe::date_date_549f70((d.as_mut_ptr() as usize) as usize as *mut [u16; 4], (0x14) as i32 as u32, (5) as i32 as u32, 0x7ce, (-1) as i32 as u32);
    if read_i16(data) != read_i16(r) {
        return;
    }
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE)));
        if club.nation().addr() == read_usize(this + 0xa) {
            *(season_loans_entry(club) as *mut u8) = 0;
        }
        i += 1;
    }
}

/// Bytes of a message up to the zero (at most 0x200).
///
/// [pt] O texto da mensagem (até o zero, no máximo 0x200 bytes).
pub(crate) unsafe fn message_text(msg: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if msg == 0 {
        return v;
    }
    for k in 0..0x200usize {
        let b = read_u8(msg + k);
        v.push(b);
        if b == 0 {
            break;
        }
    }
    v
}

/// Shared comparer for loan rules (can loan): side by side on return, [0xae1080] and reason; message checked by text
/// and restored to the original's.
///
/// [pt] Comparador comum das regras de empréstimo (pode emprestar): lado a lado na devolução, em [0xae1080] e no motivo; a
/// mensagem, que só é escrita, é conferida pelo texto (antes da cópia o primeiro byte vai a zero, e no fim volta o
/// texto da original).
pub(crate) unsafe fn england_rules_compare_can_loan(o: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, r: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32, name: &str, this: usize, club: usize, player: usize, msg: usize, reason: *mut u16) -> u32 {
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if !reason.is_null() {
        regions.push((reason as usize, 2));
    }
    let mut to = Vec::new();
    let mut tr = Vec::new();
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&regions, || {
        let x = o(this, club, player, msg, reason);
        to = message_text(msg);
        x
    }, || {
        if msg != 0 {
            *(msg as *mut u8) = 0;
        }
        let x = r(this, club, player, msg, reason);
        tr = message_text(msg);
        x
    });
    if msg != 0 {
        core::ptr::copy_nonoverlapping(to.as_ptr(), msg as *mut u8, to.len());
    }
    SCOREBOARD.record(igual && ro == rr && to == tr, || format!("{}(regras {:#x}, clube {:#x}, jogador {:#x}) original={} {:x?} {:?} rust={} {:x?} {:?}", name, this, club, player, ro, mo, String::from_utf8_lossy(&to), rr, mr, String::from_utf8_lossy(&tr)));
    ro
}

/// Returns the loan period: after 19 June and with the player under 24, the base rule period; otherwise from next month
/// to three months later.
///
/// `ENGLAND_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, called from the
/// loan period (`transfer_manager_get_loan_period`). After 19 June (only the day of the year, `exe::date_date_549f70` with 1998) and with the player under 24
/// (+0x18): the base rule period (`exe::transfer_rules_get_loan_period`). Otherwise it starts the following month and ends three months later.
/// Original: `ENGLAND_RULES::get_loan_period` (0x579c70, england_rules.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x579c70 } (thiscall, ret 0x1c, england_rules.cpp): `ENGLAND_RULES::get_loan_period(clube, a2, início_mês,
/// início_ano, fim_mês, fim_ano, jogador)`, pelo período do empréstimo (0x8d0590). Depois de 19 de junho (só o dia do
/// ano, 0x549f70 com 1998) e com o jogador abaixo de 24 anos (+0x18): o período da regra base (0x8e8bd0). Senão começa
/// no mês seguinte e acaba três meses depois.
pub(crate) unsafe extern "thiscall" fn england_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let player = Staff(player);
    // 0x549f70: push -1; push 0x7ce; push 5; push 0x13; lea ecx, local -> thiscall, ret 0x10
    let mut d = [0u8; 8];
    exe::date_date_549f70((d.as_mut_ptr() as usize) as usize as *mut [u16; 4], (0x13) as i32 as u32, (5) as i32 as u32, 0x7ce, (-1) as i32 as u32);
    if read_i16(G_CURRENT_DATE) > read_i16(d.as_ptr() as usize) && ((player.age() as u8) as i8) < 0x18 { // cqe6: abaixo de 24 anos
        crate::comparison::branch("cqe_6 england youth period");
        // 0x8e8bd0: the same 7 arguments pushed again; mov ecx, esi (this) -> thiscall, ret 0x1c
        exe::transfer_rules_get_loan_period(this, club, a2, start_month, start_year, end_month, end_year, player);
        return;
    }
    // 0x54cfb0: mov ecx, 0xae2c90 -> thiscall, ret 0 (today’s month)
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1); // cqe6: o mês seguinte (Inglaterra)
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = (*start_month).wrapping_sub(0xc);
        *start_year = (*start_year).wrapping_add(1);
    }
    crate::comparison::branch("cqe_6 england three months");
    *end_month = (*start_month).wrapping_add(3); // cqe6: três meses
    *end_year = *start_year;
    if *end_month > 0xb {
        *end_month = (*end_month).wrapping_sub(0xc);
        *end_year = (*end_year).wrapping_add(1);
    }
}

