use super::*;

/// The three European cups of squad registration, in order: the Champions League, the UEFA Cup and the Cup Winners' Cup
/// [inference] (the keys of the competition ids).
///
/// [pt] As três copas europeias da inscrição, na ordem: a Liga dos Campeões, a Copa da UEFA e a Recopa [inferência] (as
/// chaves dos ids das competições).
pub(crate) const EUROPEAN_REGISTRATION_CUPS: [usize; 3] = [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP];

/// Gets the competition's club list via its virtual +0x58 (stage), calls f for each club and frees the list.
///
/// [pt] Os clubes da competição `comp` pelo método +0x58 dela (thiscall(comp, &lista, etapa), devolve a conta em ax); a
/// lista alocada é solta pela 0x9452ca depois de `cada` passar por todos.
pub(crate) unsafe fn for_each_comp_club(comp: usize, step: u32, mut cada: impl FnMut(usize)) {
    let metodo: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(read_u32(read_u32(comp) as usize + 0x58) as usize);
    let mut list = 0usize;
    let n = metodo(comp, &mut list, step) as i16 as i32;
    for i in 0..n.max(0) as usize {
        cada(read_u32(list + 4 * i) as usize);
    }
    if list != 0 {
        exe::crt_free(list);
    }
}

/// `SQUAD_MANAGER::process_euro_registration`: stage `stage` (byte) of the European cup registration. At the first
/// stage everyone's registration is cleared; at each stage each club in the Champions League, the UEFA Cup or the third
/// cup [inference: the Cup Winners' Cup] picks its registered players; human-managed clubs get the news and pick
/// themselves. In the career, stage 3 runs in 2002 (not in 2001).
///
/// At stage 1, the registration year (+0, word) becomes the game year and the registration mark (+0x6d) of every person
/// with a contract (0x6e-byte records in [0xae23bc]) is zeroed.
///
/// Then, in each of the three cups (`EUROPEAN_REGISTRATION_CUPS`; the competition through table [0xadadfc]), each club
/// of the list (method +0x58 with the stage) that is in it (+0x1db = the id): with a human manager (`news_club_has_human_manager`, ecx =
/// [0xb7431c]) gets the registration news (`exe::squad_manager_send_euro_registration_news`(this, club, id, stage), original); otherwise the registration is
/// done (`select_euro_registered_players`(club, stage, 0), with ecx = [0xdd7ef0]).
/// Original: `SQUAD_MANAGER::process_euro_registration` (0x85a9b0, squad_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x85a9b0 } (thiscall, ret 4, squad_manager.cpp, `SQUAD_MANAGER::process_euro_registration`): a etapa `etapa`
/// (byte) da inscrição nas copas europeias. Na etapa 1, o ano das inscrições (+0, word) fica o ano do jogo e a marca de
/// inscrição (+0x6d) de todas as pessoas com contrato (registros de 0x6e bytes em [0xae23bc]) é zerada. Depois, em cada
/// uma das três copas (`EUROPEAN_REGISTRATION_CUPS`; a competição pela tabela [0xadadfc]), cada clube da lista (o método +0x58 com a
/// etapa) que está nela (+0x1db = o id): com técnico humano (0x7875e0, ecx = [0xb7431c]) recebe a notícia da inscrição
/// (0x85b020(this, clube, id, etapa), original); senão a inscrição é feita (0x85aae0(clube, etapa, 0), com ecx =
/// [0xdd7ef0]).
pub(crate) unsafe extern "thiscall" fn process_euro_registration(this: usize, step: u32) {
    if step as u8 == 1 {
        write_u16(this, read_u16(G_CURRENT_DATE_YEAR));
        let mut i = 0i32;
        while i < read_u32(G_STAFF_COUNT) as i32 {
            let m = (read_u32(G_STAFF_TABLE) as usize + 0x6e * i as usize + 0x6d) as *mut u8;
            if *m != 0 {
                *m = 0;
            }
            i += 1;
        }
    }
    for &chave in EUROPEAN_REGISTRATION_CUPS.iter() {
        let id = read_u32(chave);
        let comp = read_u32(read_u32(G_COMP_OBJECT_TABLE) as usize + 4 * id as usize) as usize;
        if comp == 0 {
            continue;
        }
        for_each_comp_club(comp, step, |c| {
            if c == 0 || read_u32(c + club::CONTINENTAL_COMP_ID) != id {
                return;
            }
            if exe::news_club_has_human_manager(read_u32(G_NEWS) as usize, c) != 0 {
                exe::squad_manager_send_euro_registration_news(this, c, id, step);
            } else {
                crate::comparison::branch("cqn_3 registers the club");
                exe::squad_manager_select_euro_registered_players(read_u32(G_SQUAD_MANAGER) as usize, c, step, 0); // cqn3: enters the club
            }
        });
    }
}

/// True if contract `k` (non-null) counts at club `c`: the contract's club (+4, id) is `c` itself ([c] = id), or it is
/// the club returned by `exe::club_get_linked_b_team`(c, 0, 1) (cdecl with 3 arguments, verified: push 1; push 0; push club; add esp, 0xc)
/// [inference: the parent club of a B team].
///
/// The contract's id is re-read after the call, as in the original.
///
/// [pt] O contrato `k` (não nulo) vale no clube `c`: o clube do contrato (+4, id) é o próprio `c` ([c] = id), ou é o clube
/// que a 0x540a50(c, 0, 1) devolve (cdecl de 3, conferida: push 1; push 0; push clube; add esp, 0xc) [inferência: o
/// clube principal de um time B]. O id do contrato é relido depois da chamada, como na original.
pub(crate) unsafe fn contract_valid_for_club(k: usize, c: usize) -> bool {
    if read_u32(k + 4) == read_u32(c) {
        return true;
    }
    let p = exe::club_get_linked_b_team(c, (0) as *mut u32, 1);
    let id = read_u32(k + 4) as usize;
    (read_u32(G_CLUB_TABLE) as usize).wrapping_add(id.wrapping_mul(club::SIZE)) == p
}

/// `SQUAD_MANAGER::get_squad_number` (this is not read): the squad number (char, negative at clubs without a human
/// manager) of person `p` at club `c`. The number lives in the contract (+0x3a); the club's contract counts (or the
/// parent club's [inference: B team]) and, failing it, the loan contract.
///
/// Missing either, 0. A person outside the table of contracted people (id ≥ [0xae2364]): byte +4 of the player sheet
/// (+0x61). Otherwise the contract (`exe::contract_manager_get_contract`, thiscall on [0xae1050], push person), if the person is a player (`exe::database_is_player_job`
/// with +0x3d, cdecl of 1) and the contract is valid at the club: the contract's +0x3a; otherwise the loan contract
/// (`contract_record`, thiscall on [0xae1050], push person), if valid at the club: its +0x3a; otherwise 0.
/// Original: `SQUAD_MANAGER::get_squad_number` (0x85a760, squad_manager.cpp, thiscall, ret 8 (this não lido)).
///
/// [pt] { 0x85a760 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::get_squad_number`; o this não é lido): o número
/// de camisa (char, negativo nos clubes sem técnico humano) da pessoa `p` no clube `c`. Sem um dos dois, 0. Pessoa fora
/// da tabela das pessoas com contrato (id ≥ [0xae2364]): o byte +4 da ficha de jogador (+0x61). Senão o contrato
/// (0x4cf5a0, thiscall em [0xae1050], push pessoa), se a pessoa é jogador (0x53b620 com o +0x3d, cdecl de 1) e o
/// contrato vale no clube: o +0x3a do contrato; senão o contrato de empréstimo (0x4cf6d0, thiscall em [0xae1050], push
/// pessoa), se vale no clube: o +0x3a dele; senão 0.
pub(crate) unsafe extern "thiscall" fn get_squad_number(_this: usize, p: usize, c: usize) -> i8 {
    let p = Staff(p);
    if p.is_null() || c == 0 {
        return 0;
    }
    if read_u32(p.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        return read_i8((p.player().addr() as u32) as usize + 4);
    }
    let k = exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && exe::database_is_player_job((p.job_for_club() as i8)) != 0 && contract_valid_for_club(k, c) {
        crate::comparison::branch("cql_1 contract number");
        return read_i8(k + 0x3a); // cql1: the contract number
    }
    let k = exe::contract_manager_get_loan_contract(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        return read_i8(k + 0x3a);
    }
    0
}

/// `SQUAD_MANAGER::club_uses_squad_numbers` (this is not read): 1 if the club uses fixed squad numbers.
///
/// The competition of the club's division (the id in [[c+0x57]], the object in table [0xadadfc]) exists and has bit 2
/// of +0x52 and, with `other` (Comp*, e.g. a cup), the other competition (the one in the table by id [other]) also has
/// the bit. No division or no competition, 0; no club, the error box at line 0x4b0 and 0.
/// Original: `SQUAD_MANAGER::club_uses_squad_numbers` (0x85add0, squad_manager.cpp, thiscall, ret 8 (this não lido)).
///
/// [pt] { 0x85add0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::club_uses_squad_numbers`; o this não é lido):
/// 1 se o clube usa números de camisa: a competição da divisão do clube (o id em [[c+0x57]], o objeto na tabela
/// [0xadadfc]) existe e tem o bit 2 do +0x52 e, com `outra` (Comp*), a outra competição (a da tabela pelo id [outra])
/// também tem o bit. Sem divisão ou sem a competição, 0; sem clube, a caixa da linha 0x4b0 e 0.
pub(crate) unsafe extern "thiscall" fn club_uses_squad_numbers(_this: usize, c: usize, other: usize) -> u32 {
    let c = Club(c);
    if c.is_null() {
        show_error_at(0xa82b88, 0x4b0);
        return 0;
    }
    let div = (c.division().addr() as u32) as usize;
    if div == 0 {
        return 0;
    }
    let table = read_u32(G_COMP_OBJECT_TABLE) as usize;
    let comp = Competition(read_u32(table + read_u32(div) as usize * 4) as usize);
    if comp.is_null() {
        return 0;
    }
    if comp.short_gender_name() & 2 == 0 {
        return 0;
    }
    if other != 0 {
        let comp2 = Competition(read_u32(table + read_u32(other) as usize * 4) as usize);
        if comp2.short_gender_name() & 2 == 0 {
            return 0;
        }
    }
    crate::comparison::branch("cql_1 uses numbers");
    1 // cql1: usa números
}

/// `SQUAD_MANAGER::set_squad_number` (this is not read): writes the squad number `n` (char from −50 to 50; outside
/// that, the error box at line 0x332).
///
/// Written to +0x3a of person `p`'s contract (`exe::contract_manager_get_contract`) if it is valid at club `c`, otherwise to that of the loan
/// contract (`contract_record`) if it is valid at the club. No person or no club, nothing.
/// Original: `SQUAD_MANAGER::set_squad_number` (0x85a850, squad_manager.cpp, thiscall, ret 0xc (this não lido)).
///
/// [pt] { 0x85a850 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::set_squad_number`; o this não é lido): grava o
/// número `n` (char de −50 a 50; fora disso a caixa da linha 0x332) no +0x3a do contrato da pessoa `p` (0x4cf5a0) se ele
/// vale no clube `c`, senão no do contrato de empréstimo (0x4cf6d0) se ele vale no clube. Sem pessoa ou sem clube, nada.
pub(crate) unsafe extern "thiscall" fn set_squad_number(_this: usize, p: usize, c: usize, n: u32) {
    if p == 0 || c == 0 {
        return;
    }
    let n = n as u8 as i8;
    if !(-50..=50).contains(&n) {
        show_error_at(0xa82b88, 0x332);
        return;
    }
    let k = exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        crate::comparison::branch("cql_1 writes into contract");
        write_i8(k + 0x3a, n); // cql1: writes to the contract
        return;
    }
    let k = exe::contract_manager_get_loan_contract(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        write_i8(k + 0x3a, n);
    }
}

/// True if the person's id is below G_STAFF_COUNT (in the table of contracted people).
///
/// [pt] A pessoa `p` está na tabela das pessoas com contrato (id < [0xae2364], com sinal).
pub(crate) unsafe fn is_contracted_staff(p: usize) -> bool {
    (read_u32(p) as i32) < read_u32(G_STAFF_COUNT) as i32
}

/// The preferred squad numbers (index = number − 1) by the positions of player sheet `f` (position ratings from 1 to
/// 20, "good" = 15 or more).
///
/// Goalkeeper (+0xf): 1; defender or sweeper (+0x11, +0x10): 2 if he plays on the right (+0x17), 3 if on the left
/// (+0x18), otherwise 4, 5 or 6; defensive midfielder, midfielder or attacking midfielder (+0x12, +0x13, +0x14): 6, 7
/// or 8; forward (+0x15): 8, 9, 10 or 11. Ends with 0xff.
///
/// None of the positions good: `None` (the original then reads the 4-byte list with no value on the stack, see
/// `compare_allocate_squad_number`).
///
/// [pt] Os números preferidos (índice = número − 1) pela posição da ficha de jogador `f` (as notas de posição de 1 a 20,
/// "boa" = 15 ou mais): goleiro (+0xf) o 1; zagueiro ou líbero (+0x11, +0x10) o 2 se joga pela direita (+0x17), o 3 se
/// pela esquerda (+0x18), senão o 4, o 5 ou o 6; volante, meia ou meia-atacante (+0x12, +0x13, +0x14) o 6, o 7 ou o 8;
/// atacante (+0x15) o 8, o 9, o 10 ou o 11. Termina em 0xff. Nenhuma das posições boas: `None` (a original lê então a
/// lista de 4 bytes sem valor na pilha, ver `compare_allocate_squad_number`).
pub(crate) unsafe fn preferred_squad_numbers(f: usize) -> Option<[u8; 5]> {
    let b = |o: usize| read_i8(f + o) >= 15;
    if b(0xf) {
        return Some([0, 0xff, 0xff, 0xff, 0xff]);
    }
    if b(0x11) || b(0x10) {
        if b(0x17) {
            return Some([1, 0xff, 0xff, 0xff, 0xff]);
        }
        if b(0x18) {
            return Some([2, 0xff, 0xff, 0xff, 0xff]);
        }
        return Some([3, 4, 5, 0xff, 0xff]);
    }
    if b(0x13) || b(0x12) || b(0x14) {
        return Some([5, 6, 7, 0xff, 0xff]);
    }
    if b(0x15) {
        return Some([7, 8, 9, 10, 0xff]);
    }
    None
}

/// `SQUAD_MANAGER::allocate_squad_number`: gives a squad number to person `p` arriving at club `c`: the first free
/// number for his position; goalkeeper 1; defender 2 (right back), 3 (left) or 4 to 6; defensive, central and attacking
/// midfielder 6 to 8; striker 8 to 11 (good position = rating 15 or more).
///
/// Marks the numbers taken by the squad (`database_collect_club_squad`(c, list, 0), cdecl of 3; the list sorted by age with qsort `exe::crt_qsort`
/// and 0x540130; anyone outside the table of contracted people or without a player sheet does not count) with each
/// one's number (`get_squad_number`, absolute value). If `p` is in the table and has no number yet: the first free number among
/// the position's preferred ones (`preferred_squad_numbers`); otherwise the first free one from 12 to 50; and, if still
/// without a number, the highest free one from 11 down. The number goes positive for a club with a human manager
/// (`exe::human_manager_is_human_club`(c, 0, 1, 0, 0), cdecl of 5) and negative for the others (`set_squad_number`). No person, nothing.
/// Original: `SQUAD_MANAGER::allocate_squad_number` (0x85a470, squad_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x85a470 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::allocate_squad_number`): dá um número de camisa
/// à pessoa `p` que chega ao clube `c`. Marca os números ocupados pelo elenco (0x541f90(c, lista, 0), cdecl de 3; a
/// lista ordenada pela idade com o qsort 0x945333 e a 0x540130; quem está fora da tabela das pessoas com contrato ou sem
/// ficha de jogador não conta) com o número de cada um (0x85a760, em módulo). Se `p` está na tabela e ainda não tem
/// número: o primeiro número livre entre os preferidos da posição (`preferred_squad_numbers`); senão o primeiro livre de 12 a
/// 50; e, se ainda sem número, o maior livre de 11 para baixo. O número vai positivo para o clube de técnico humano
/// (0x600a50(c, 0, 1, 0, 0), cdecl de 5) e negativo para os outros (0x85a850). Sem a pessoa, nada.
pub(crate) unsafe extern "thiscall" fn allocate_squad_number(this: usize, c: usize, p: usize) {
    let p = Staff(p);
    if p.is_null() {
        return;
    }
    let mut used = [0u8; 50];
    let mut list = [0usize; 50];
    exe::database_collect_club_squad(c, list.as_mut_ptr(), 0);
    exe::crt_qsort(list.as_mut_ptr() as usize, 0x32, 4, 0x540130);
    for q in list.iter_mut() {
        let x = Staff(*q);
        if x.is_null() {
            continue;
        }
        if (x.player().addr() as u32) == 0 {
            *q = 0;
            continue;
        }
        if is_contracted_staff(x.addr()) {
            let n = (exe::squad_manager_get_squad_number(this, x, c) as i32).abs() as u8;
            if n != 0 {
                used[(n as i8 as isize - 1) as usize] = 1;
            }
        }
    }
    if !is_contracted_staff(p.addr()) || exe::squad_manager_get_squad_number(this, p, c) != 0 {
        return;
    }
    let pref = preferred_squad_numbers((p.player().addr() as u32) as usize).unwrap_or([0xff; 5]);
    // the number with the club’s sign: positive with a human manager, negative without
    let sign = |k: u8| -> u32 {
        if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 {
            k.wrapping_add(1) as u32
        } else {
            0xffu8.wrapping_sub(k) as u32
        }
    };
    let mut b = 0usize;
    while pref[b] != 0xff {
        let k = pref[b];
        if used[k as i8 as usize] == 0 {
            exe::squad_manager_set_squad_number(this, p, c, sign(k));
            used[k as usize] = 1;
            crate::comparison::branch("cql_2 position number");
            return; // cql2: the position number
        }
        b += 1;
        if b >= 5 {
            break;
        }
    }
    if let Some(k) = (11u8..50).find(|&k| used[k as usize] == 0) {
        exe::squad_manager_set_squad_number(this, p, c, sign(k));
        used[k as usize] = 1;
    }
    if exe::squad_manager_get_squad_number(this, p, c) != 0 {
        return;
    }
    if let Some(k) = (0u8..=10).rev().find(|&k| used[k as usize] == 0) {
        exe::squad_manager_set_squad_number(this, p, c, sign(k));
    }
}

/// Memory regions: byte +0x3a of the club and loan contracts of the given people, plus the error-box flag [0xb67a34].
///
/// [pt] O +0x3a dos contratos (o do clube e o de empréstimo) das pessoas `ps` e a marca da caixa [0xb67a34].
pub(crate) unsafe fn squad_number_regions(ps: &[usize]) -> Vec<(usize, usize)> {
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &p in ps {
        if p == 0 {
            continue;
        }
        for k in [exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, p), exe::contract_manager_get_loan_contract(read_u32(G_CONTRACT_MANAGER) as usize, p)] {
            if k != 0 && !reg.contains(&(k + 0x3a, 1)) {
                reg.push((k + 0x3a, 1));
            }
        }
    }
    reg
}

/// The slots of the reference line-up that `assign_club_squad_numbers` passes to 0x6e2000: the 11 starters and the 9 substitutes, each
/// with the positions that fit (masks: 1 goalkeeper, 4/0x10/0x40 the sectors, 0x80..0x800 the bands [inference]).
///
/// [pt] As vagas da escalação de referência da 0x859fe0 para a 0x6e2000: os 11 titulares e os 9 reservas, cada um com as
/// posições que servem (máscaras: 1 goleiro, 4/0x10/0x40 os setores, 0x80..0x800 as faixas [inferência]).
pub(crate) const SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS: [u16; 20] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // cql2: os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x4, // the reserves
];

/// the rule: the squad (with the linked B team) is ordered by the reference line-up (11 starters and 9 substitutes) and
/// the rest by age; whoever already has a number keeps it unless it is repeated (without `keep`, a starter's number, 1
/// to 11, only stays with the first 11 and a reserve's only with the others); whoever has none gets his order number if
/// free; from the 22nd on, without a contract with the club or with a youth/amateur contract [inference: types 6 and
/// 7], he gets no number; the first 40 still without one get one by the new-arrival rule; at the end the kept numbers
/// are rewritten (positive with a human manager, otherwise −1, −2... in number order)
/// Original: `SQUAD_MANAGER::assign_club_squad_numbers` (0x859fe0, squad_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x859fe0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::assign_club_squad_numbers`): numera o elenco do
/// clube `c`. Junta os jogadores do elenco (+0xd7, 50 vagas; só os da tabela das pessoas com contrato) e os do clube
/// ligado (0x540a50(c, 0, 1), cdecl de 3; até 50 no total); escala os 11 titulares e os 9 reservas de referência
/// (0x6e2000, cdecl de 15, com `SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS` e os modos 0x5840) e põe os outros depois, nas vagas livres; ordena os 30
/// últimos pela idade (qsort 0x945333 com a 0x540130) e junta a lista. Depois, na ordem: do 22º em diante, quem não tem
/// contrato com o clube (0x4cf5f0) ou tem contrato do tipo 6 ou 7 (+0x4f, nibble alto) [inferência: juvenil e amador]
/// fica sem número; quem tem número (0x85a760) o guarda se ele está livre e, sem `manter`, se é de titular (até 11)
/// entre os 11 primeiros ou de reserva (12 em diante) depois deles (senão fica sem número); com `manter`, guarda o número
/// com o sinal trocado; quem não tem número ganha o da sua ordem (1º o 1...) se livre. Os 40 primeiros sem número que
/// têm contrato com o clube (fora os tipos 6 e 7) ganham um pela 0x85a470. No fim, cada número guardado é regravado:
/// positivo (o número) com técnico humano (0x600a50(c, 0, 1, 0, 0)), senão −1, −2... na ordem dos números.
pub(crate) unsafe extern "thiscall" fn assign_club_squad_numbers(this: usize, c: usize, keep: u32) {
    let c = Club(c);
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let mut stored = [0usize; 50]; // by number − 1
    let mut order = [0usize; 50];
    let mut all = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let x = (c.squad(k) as u32) as usize;
        if x != 0 && is_contracted_staff(x) {
            all[n] = x;
            n += 1;
        }
    }
    let l = Club(exe::club_get_linked_b_team(c, (0) as *mut u32, 1));
    if !l.is_null() {
        for k in 0..50 {
            if n >= 50 {
                break;
            }
            let x = (l.squad(k) as u32) as usize;
            if x != 0 && is_contracted_staff(x) {
                all[n] = x;
                n += 1;
            }
        }
    }
    let slots = SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS;
    crate::comparison::branch("cql_2 picks the team");
    pick(all.as_ptr() as usize, n as u32, slots.as_ptr() as usize, 0xb, 9, order.as_mut_ptr() as usize, 0x5840, c.addr(), 0, 0, 0, 0, 0, 0, 0);
    let mut free = 0usize;
    for &x in all[..n].iter() {
        if order[..20].contains(&x) {
            continue;
        }
        while free < 50 && order[free] != 0 {
            free += 1;
        }
        if free < 50 {
            order[free] = x;
        }
    }
    exe::crt_qsort(order[20..].as_mut_ptr() as usize, 0x1e, 4, 0x540130);
    let mut a = 0usize;
    for b in 0..50 {
        let x = order[b];
        if x == 0 || b <= a {
            continue;
        }
        while a < b && order[a] != 0 {
            a += 1;
        }
        if a < b {
            order[a] = x;
            a += 1;
            order[b] = 0;
        }
    }
    // `vez` = −1, −2... (char), the order of each one; `k` = 1, 2... the same order as a positive number
    for i in 0..50usize {
        let team = (-1 - i as i32) as i8;
        let x = order[i];
        if x == 0 {
            continue;
        }
        if team < -21 {
            let ct = exe::contract_manager_get_contract_for_club(read_u32(G_CONTRACT_MANAGER) as usize, x, c);
            let t = if ct != 0 { read_u8(ct + contract::SQUAD_STATUS) & 0xf0 } else { 0 };
            if ct == 0 || t == 0x60 || t == 0x70 {
                exe::squad_manager_set_squad_number(this, x, c, 0);
                continue;
            }
        }
        let nb = (exe::squad_manager_get_squad_number(this, x, c) as i32).abs() as u8 as i8;
        if nb == 0 {
            if stored[i] == 0 {
                stored[i] = x;
                exe::squad_manager_set_squad_number(this, x, c, team as u8 as u32);
            }
            continue;
        }
        let slot = (nb as isize - 1) as usize;
        if stored[slot] != 0 {
            exe::squad_manager_set_squad_number(this, x, c, 0);
            continue;
        }
        if keep != 0 {
            exe::squad_manager_set_squad_number(this, x, c, nb.wrapping_neg() as u8 as u32);
        } else if (nb < 12 && team < -11) || (nb >= 12 && team > -12) {
            crate::comparison::branch("cql_2 loses the number");
            exe::squad_manager_set_squad_number(this, x, c, 0); // cql2: the out-of-place number
            continue;
        }
        if nb > 0 {
            stored[slot] = x;
        }
    }
    for b in 0..40usize {
        let x = order[b];
        if x == 0 {
            continue;
        }
        let ct = exe::contract_manager_get_contract_for_club(read_u32(G_CONTRACT_MANAGER) as usize, x, c);
        if ct == 0 {
            continue;
        }
        let t = read_u8(ct + contract::SQUAD_STATUS) & 0xf0;
        if t == 0x60 || t == 0x70 {
            continue;
        }
        if exe::squad_manager_get_squad_number(this, x, c) == 0 {
            exe::squad_manager_allocate_squad_number(this, c, x);
        }
    }
    if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 {
        for (k, &x) in stored.iter().enumerate() {
            if x != 0 {
                exe::squad_manager_set_squad_number(this, x, c, k as u32 + 1);
            }
        }
    } else {
        let mut v = -1i8;
        for &x in stored.iter() {
            if x != 0 {
                crate::comparison::branch("cql_2 rewrites negative");
                exe::squad_manager_set_squad_number(this, x, c, v as u8 as u32); // cql2: regrava negativo
            }
            v = v.wrapping_sub(1);
        }
    }
}

/// The leagues that the original `init_new_game` asked to number (recorded by the comparer of `assign_league_squad_numbers` while the comparer of
/// `init_new_game` runs the original), to check the copy's plan.
///
/// [pt] As ligas que a 0x859dc0 da original mandou numerar (gravadas pelo comparador da 0x859e20 enquanto o da 0x859dc0
/// roda a original), para conferir o plano da cópia.
pub(crate) static mut SQUAD_NUMBERS_PLAN: Option<Vec<usize>> = None;

/// Calls f for each competition in G_COMP_OBJECT_TABLE that is a league division and uses squad numbers (bit 2 of
/// +0x52), as `init_new_game` numbers them.
///
/// [pt] As ligas que a 0x859dc0 numera: as competições da tabela [0xadadfc] (conta [0xadadf8], relida a cada volta) que
/// são liga de divisão (0x4b0700 com o id [[+4]], cdecl de 1) e usam números (bit 2 do +0x52).
pub(crate) unsafe fn for_each_numbered_league(mut cada: impl FnMut(usize)) {
    let mut i = 0i32;
    while i < read_u32(G_COMP_OBJECT_COUNT) as i32 {
        let c = League(read_u32(read_u32(G_COMP_OBJECT_TABLE) as usize + 4 * i as usize) as usize);
        if !c.is_null() && exe::comp_util_is_league_division(read_u32((c.competition().addr() as u32) as usize) as i32) != 0 && c.squad_flags() & 2 != 0 {
            crate::comparison::branch("cql_3 numbers the league");
            cada(c.addr()); // cql3: numera a liga
        }
        i += 1;
    }
}

/// the rule: at the start of the game, each division league that uses squad numbers (bit 2 of its options) is numbered
/// from the database
/// Original: `SQUAD_MANAGER::init_new_game` (0x859dc0, squad_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x859dc0 } (thiscall, ret 0, squad_manager.cpp, `SQUAD_MANAGER::init_new_game`): zera o ano das inscrições (+0,
/// word) e numera (0x859e20(this, liga, 1, 0)) cada liga de divisão que usa números (`for_each_numbered_league`).
pub(crate) unsafe extern "thiscall" fn init_new_game(this: usize) {
    write_u16(this, 0);
    for_each_numbered_league(|c| exe::squad_manager_assign_league_squad_numbers(this, c, 1, 0));
}

/// `SQUAD_MANAGER::assign_league_squad_numbers`: numbers the clubs of league `league` (except B teams); a human-managed
/// club is not numbered by the game.
///
/// The list comes from the competition's method +0x58, thiscall(league, &list, 0), returning the count in ax; the list
/// is freed at the end by `exe::crt_free`. In 1998 `initial` always applies. Skips the B team (`exe::club_get_linked_b_team`(club, &parent, 1) with
/// the club linked and parent = 0). A club with a human manager (`news_club_has_human_manager`, with ecx = [0xb7431c]) without `automatic`:
/// the squad's negative numbers (`database_collect_club_squad`(club, list, 0)) become positive and the manager gets the squad numbers
/// request (`exe::squad_manager_send_squad_numbers_request_news`, original). Otherwise: with `initial`, each player of the squad (from the table of contracted
/// people and with a player sheet) first gets the database number (+4 of the player sheet, between 0 and 50); and the
/// squad is numbered (`assign_club_squad_numbers`(club, 1)).
/// Original: `SQUAD_MANAGER::assign_league_squad_numbers` (0x859e20, squad_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x859e20 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::assign_league_squad_numbers`): numera os clubes
/// da liga `liga` (a lista do método +0x58 da competição, thiscall(liga, &lista, 0), devolve a conta em ax; a lista é
/// solta no fim pela 0x9452ca). Em 1998 vale sempre `inicial`. Pula o time B (0x540a50(clube, &principal, 1) com o
/// clube ligado e principal = 0). Clube de técnico humano (0x7875e0, com ecx = [0xb7431c]) sem `automatico`: os números
/// negativos do elenco (0x541f90(clube, lista, 0)) viram positivos e o técnico recebe o pedido da lista de números
/// (0x85aec0, original). Senão: com `inicial`, cada jogador do elenco (da tabela das pessoas com contrato e com ficha)
/// recebe o número do banco (o +4 da ficha, entre 0 e 50); e o elenco é numerado (0x859fe0(clube, 1)).
pub(crate) unsafe extern "thiscall" fn assign_league_squad_numbers(this: usize, league: usize, mut initial: u32, automatico: u32) {
    if read_u16(G_CURRENT_DATE_YEAR) == 0x7ce {
        initial = 1;
    }
    let clubs: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(read_u32(read_u32(league) as usize + 0x58) as usize);
    let mut list = 0usize;
    let n = clubs(league, &mut list, 0) as i16 as i32;
    for i in 0..n.max(0) as usize {
        let c = read_u32(list + 4 * i) as usize;
        let mut main = 0u32;
        if exe::club_get_linked_b_team(c, &mut main, 1) != 0 && main == 0 {
            continue;
        }
        let mut sq = [0usize; 50];
        exe::database_collect_club_squad(c, sq.as_mut_ptr(), 0);
        if exe::news_club_has_human_manager(read_u32(G_NEWS) as usize, c) != 0 && automatico == 0 {
            for &p in sq.iter() {
                if p != 0 && exe::squad_manager_get_squad_number(this, p, c) < 0 {
                    exe::squad_manager_set_squad_number(this, p, c, (exe::squad_manager_get_squad_number(this, p, c) as u8).wrapping_neg() as u32);
                }
            }
            exe::squad_manager_send_squad_numbers_request_news(this, c);
            continue;
        }
        if initial != 0 {
            for &p in sq.iter() {
                if p == 0 || read_u32(p) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
                    continue;
                }
                let f = read_u32(p + staff::PLAYER) as usize;
                if f == 0 {
                    continue;
                }
                let b = read_i8(f + 4);
                let k = if b < 0 { 0 } else if b > 0x32 { 0x32 } else { b as i32 };
                crate::comparison::branch("cql_3 bench number");
                exe::squad_manager_set_squad_number(this, p, c, k as u32); // cql3: the bench number
            }
        }
        exe::squad_manager_assign_club_squad_numbers(this, c, 1);
    }
    if list != 0 {
        exe::crt_free(list);
    }
}

/// The slots of `select_euro_registered_players`'s reference lineup: the 11 starters and up to 14 substitutes (those of `assign_club_squad_numbers`,
/// `SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS`, up to the 19th; the 20th here is a goalkeeper's).
///
/// [pt] As vagas da escalação de referência da 0x85aae0: os 11 titulares e até 14 reservas (as da 0x859fe0, `SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS`,
/// até a 19ª; a 20ª aqui é de goleiro).
pub(crate) const EURO_REFERENCE_LINEUP_SLOTS: [u16; 25] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // the starters
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, // the reserves
];

/// `SQUAD_MANAGER::select_euro_registered_players` (this is not read): the registration of club `c` for the European
/// cups at stage `stage` (1 is the first; `mark` [inference: list A or B]), through the person's registration mark
/// (+0x6d: 1 or 2 registered, 4 and 8 other marks).
///
/// From the squad (`database_collect_club_squad`(c, list, 0)), the candidates are those in the table of contracted people, without a type-5
/// contract (+0x35 & 0x3f; the contract found stays in [0xae1080]), without a loan contract and without marks 8 and
/// 0x10 (+0x6d & 0x18); outside stage 1, mark 1 drops to 0 and 4 becomes 2, and the 2s are counted. The reference
/// line-up (0x6e2000 with `EURO_REFERENCE_LINEUP_SLOTS`, the 11 starters and 14 substitutes, or 9 at stage 1 without
/// `mark`) picks the best. At stage 3, up to 2 registered players (2) left out of the line-up lose the registration
/// (they go to 4 with `mark`, otherwise 0) and the count drops (to a minimum of 21). Then each selected player: at
/// stage 1, is registered (1 with `mark`, otherwise 2) and counted; at the others, anyone not yet 2 and without mark 8
/// is registered the same way while the count is below the line-up's places. Returns the count of registered players
/// (char).
/// Original: `SQUAD_MANAGER::select_euro_registered_players` (0x85aae0, squad_manager.cpp, thiscall, ret 0xc (this não lido)).
///
/// [pt] { 0x85aae0 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::select_euro_registered_players`; o this não é
/// lido): a inscrição do clube `c` nas copas europeias na etapa `etapa` (1 a primeira; `marca` [inferência: a lista A
/// ou B]), pela marca de inscrição da pessoa (+0x6d: 1 ou 2 inscrito, 4 e 8 outras marcas). Do elenco (0x541f90(c,
/// lista, 0)), os candidatos são os da tabela das pessoas com contrato, sem contrato do tipo 5 (+0x35 & 0x3f; o
/// contrato achado fica em [0xae1080]), sem contrato de empréstimo e sem as marcas 8 e 0x10 (+0x6d & 0x18); fora da
/// etapa 1, a marca 1 cai para 0 e a 4 vira 2, e os 2 são contados. A escalação de referência (0x6e2000 com
/// `EURO_REFERENCE_LINEUP_SLOTS`, os 11 titulares e 14 reservas, ou 9 na etapa 1 sem `marca`) escolhe os melhores. Na etapa 3, até 2
/// inscritos (2) que ficaram fora da escalação perdem a inscrição (vão a 4 com `marca`, senão 0) e a conta baixa (no
/// mínimo 21). Depois cada escalado: na etapa 1, é inscrito (1 com `marca`, senão 2) e contado; nas outras, quem ainda
/// não é 2 nem tem a marca 8 é inscrito do mesmo jeito enquanto a conta for menor que as vagas da escalação. Devolve a
/// conta dos inscritos (char).
pub(crate) unsafe extern "thiscall" fn select_euro_registered_players(_this: usize, c: usize, step: u32, mark: u32) -> u8 {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let step = step as u8;
    let mut sq = [0usize; 50];
    exe::database_collect_club_squad(c, sq.as_mut_ptr(), 0);
    let reservas: u8 = if step == 1 { if mark != 0 { 0xe } else { 9 } } else { 0xe };
    let mut cand = [0usize; 50];
    let mut n = 0u8;
    let mut count = 0i8;
    let mut tirados = 0i8;
    let m6d = |p: Staff| (p.addr() + staff::SQUAD_SELECTED_FOR) as *mut u8;
    for &p in sq.iter() {
        if p == 0 || read_u32(p) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
            continue;
        }
        let k = Contract(contract(read_u32(G_CONTRACT_MANAGER) as usize, p));
        write_u32(G_CONTRACT_LOOKUP_RESULT, k.addr() as u32);
        if !k.is_null() && k.contract_type() & 0x3f == 5 {
            continue;
        }
        if exe::contract_manager_get_loan_contract(read_u32(G_CONTRACT_MANAGER) as usize, p) != 0 || *m6d(Staff(p)) & 0x18 != 0 {
            continue;
        }
        cand[n as i8 as usize] = p;
        n = n.wrapping_add(1);
        let a = *m6d(Staff(p));
        if a == 0 || step == 1 {
            continue;
        }
        if a == 1 {
            *m6d(Staff(p)) = 0;
        } else if a == 4 {
            *m6d(Staff(p)) = 2;
        }
        if *m6d(Staff(p)) == 2 {
            count = count.wrapping_add(1);
        }
    }
    let slots = EURO_REFERENCE_LINEUP_SLOTS;
    let mut esc = [0usize; 50];
    pick(cand.as_ptr() as usize, n as u32, slots.as_ptr() as usize, 0xb, reservas as u32, esc.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
    if step == 3 {
        let mut d = 0i8;
        while d < n as i8 && tirados < 2 {
            let p = cand[d as usize];
            if p != 0 && *m6d(Staff(p)) == 2 && !esc[..0x19].contains(&p) {
                *m6d(Staff(p)) = if mark != 0 { 4 } else { 0 };
                tirados += 1;
                count = count.wrapping_sub(1);
                if count <= 0x15 {
                    count = 0x15;
                }
            }
            d += 1;
        }
    }
    let total = reservas as i8 as i32 + 0xb;
    let mut b = 0i32;
    while b < total {
        let p = esc[b as usize];
        if p != 0 {
            let v = if mark != 0 { 1 } else { 2 };
            if step == 1 {
                crate::comparison::branch("cql_3 registers in stage 1");
                *m6d(Staff(p)) = v; // cql3: inscreve
                count = count.wrapping_add(1);
            } else {
                let a = *m6d(Staff(p));
                if a != 2 && a & 8 == 0 && (count as i32) < total {
                    crate::comparison::branch("cql_3 registers later");
                    *m6d(Staff(p)) = v; // cql3: inscreve depois
                    count = count.wrapping_add(1);
                }
            }
        }
        b = (b as i8).wrapping_add(1) as i32;
    }
    count as u8
}

/// `SQUAD_MANAGER::save(save)`: writes the squad manager's u16 at +0 into the `squad_manager.dat` item, widened to 4
/// bytes (the original zeroes eax before the `mov ax`), and returns 1.
///
/// Error on opening or writing: the "Error" box (squad_manager.cpp 0x68, 0x6f) and 0, without closing the item. Does
/// not check for a null container (`exe::zopen` receives it as it comes).
/// Original: `SQUAD_MANAGER::save` (0x859cc0, squad_manager.cpp, thiscall).
///
/// [pt] { 0x859cc0 } (thiscall, ret 4, squad_manager.cpp): `SQUAD_MANAGER::save(save)`: grava no item `squad_manager.dat`
/// o u16 em +0 do gerente de elencos, estendido a 4 bytes (a original zera eax antes do `mov ax`), e devolve 1. Erro
/// ao abrir ou ao gravar: a caixa "Error" (squad_manager.cpp 0x68, 0x6f) e 0, sem fechar o item. Não confere o
/// contêiner nulo (o 0x9326e0 recebe como vier).
pub(crate) unsafe extern "thiscall" fn squad_manager_save(this: usize, save: usize) -> u32 {
    crate::comparison::branch("cpx_5 save squad manager");
    let mut v = read_u16(this) as u32; // cpx5: the squad manager value
    let mut z = exe::zopen(save, 0xa82bc0, 1, 0x16, 4);
    if z == 0 {
        show_error_at(0xa82b88, 0x68);
        return 0;
    }
    if exe::zwrite(&mut v as *mut u32 as usize, 4, 1, z) != 1 {
        show_error_at(0xa82b88, 0x6f);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

