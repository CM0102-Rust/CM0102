use super::*;


/// As três copas europeias da inscrição, na ordem: a Liga dos Campeões, a Copa da UEFA e a Recopa [inferência] (as
/// chaves dos ids das competições).
pub(crate) const EUROPEAN_REGISTRATION_CUPS: [usize; 3] = [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP];

/// Gets the competition's club list via its virtual +0x58 (stage), calls f for each club and frees the list.
///
/// [pt] Os clubes da competição `comp` pelo método +0x58 dela (thiscall(comp, &lista, etapa), devolve a conta em ax); a
/// lista alocada é solta pela 0x9452ca depois de `cada` passar por todos.
pub(crate) unsafe fn for_each_comp_club(comp: usize, etapa: u32, mut cada: impl FnMut(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let metodo: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(read_u32(read_u32(comp) as usize + 0x58) as usize);
    let mut list = 0usize;
    let n = metodo(comp, &mut list, etapa) as i16 as i32;
    for i in 0..n.max(0) as usize {
        cada(read_u32(list + 4 * i) as usize);
    }
    if list != 0 {
        soltar(list);
    }
}

/// the rule: at the first stage everyone's registration is cleared; at each stage each club in the Champions League,
/// the UEFA Cup or the third cup [inference: the Cup Winners' Cup] picks its registered players (human-managed clubs
/// get the news and pick themselves); in the career, stage 3 runs in 2002 (not in 2001)
/// Original: `SQUAD_MANAGER::process_euro_registration` (0x85a9b0, squad_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x85a9b0 } (thiscall, ret 4, squad_manager.cpp, `SQUAD_MANAGER::process_euro_registration`): a etapa `etapa`
/// (byte) da inscrição nas copas europeias. Na etapa 1, o ano das inscrições (+0, word) fica o ano do jogo e a marca de
/// inscrição (+0x6d) de todas as pessoas com contrato (registros de 0x6e bytes em [0xae23bc]) é zerada. Depois, em cada
/// uma das três copas (`EUROPEAN_REGISTRATION_CUPS`; a competição pela tabela [0xadadfc]), cada clube da lista (o método +0x58 com a
/// etapa) que está nela (+0x1db = o id): com técnico humano (0x7875e0, ecx = [0xb7431c]) recebe a notícia da inscrição
/// (0x85b020(this, clube, id, etapa), original); senão a inscrição é feita (0x85aae0(clube, etapa, 0), com ecx =
/// [0xdd7ef0]).
pub(crate) unsafe extern "thiscall" fn process_euro_registration(this: usize, etapa: u32) {
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_SQUAD_MANAGER_SEND_EURO_REGISTRATION_NEWS);
    let inscrever: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SQUAD_MANAGER_SELECT_EURO_REGISTERED_PLAYERS);
    if etapa as u8 == 1 {
        *(this as *mut u16) = *(G_CURRENT_DATE_YEAR as *const u16);
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
        for_each_comp_club(comp, etapa, |c| {
            if c == 0 || read_u32(c + 0x1db) != id {
                return;
            }
            if human(read_u32(G_NEWS) as usize, c) != 0 {
                noticia(this, c, id, etapa);
            } else {
                crate::comparacao::ramo("cqn_3 inscreve o clube");
                inscrever(read_u32(G_SQUAD_MANAGER) as usize, c, etapa, 0); // cqn3: inscreve o clube
            }
        });
    }
}







/// True if contract k's club (+4) is club c itself or the club returned by 0x540a50(c, 0, 1) (the parent of a B team)
/// [inference].
///
/// [pt] O contrato `k` (não nulo) vale no clube `c`: o clube do contrato (+4, id) é o próprio `c` ([c] = id), ou é o clube
/// que a 0x540a50(c, 0, 1) devolve (cdecl de 3, conferida: push 1; push 0; push clube; add esp, 0xc) [inferência: o
/// clube principal de um time B]. O id do contrato é relido depois da chamada, como na original.
pub(crate) unsafe fn contract_valid_for_club(k: usize, c: usize) -> bool {
    let partner: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    if read_u32(k + 4) == read_u32(c) {
        return true;
    }
    let p = partner(c, 0, 1);
    let id = read_u32(k + 4) as usize;
    (read_u32(G_CLUB_TABLE) as usize).wrapping_add(id.wrapping_mul(club::SIZE)) == p
}

/// the rule: the squad number lives in the contract (+0x3a), positive at human-managed clubs and negative elsewhere;
/// the club's contract counts (or the parent club's [inference: B team]) and, failing it, the loan contract; a person
/// outside the contracted-people table has the number in the player record
/// Original: `SQUAD_MANAGER::get_squad_number` (0x85a760, squad_manager.cpp, thiscall, ret 8 (this não lido)).
///
/// [pt] { 0x85a760 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::get_squad_number`; o this não é lido): o número
/// de camisa (char, negativo nos clubes sem técnico humano) da pessoa `p` no clube `c`. Sem um dos dois, 0. Pessoa fora
/// da tabela das pessoas com contrato (id ≥ [0xae2364]): o byte +4 da ficha de jogador (+0x61). Senão o contrato
/// (0x4cf5a0, thiscall em [0xae1050], push pessoa), se a pessoa é jogador (0x53b620 com o +0x3d, cdecl de 1) e o
/// contrato vale no clube: o +0x3a do contrato; senão o contrato de empréstimo (0x4cf6d0, thiscall em [0xae1050], push
/// pessoa), se vale no clube: o +0x3a dele; senão 0.
pub(crate) unsafe extern "thiscall" fn get_squad_number(_this: usize, p: usize, c: usize) -> i8 {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let loan: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let player: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    if p == 0 || c == 0 {
        return 0;
    }
    if read_u32(p) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        return *((read_u32(p + 0x61) as usize + 4) as *const i8);
    }
    let k = contrato(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && player(*((p + 0x3d) as *const i8)) != 0 && contract_valid_for_club(k, c) {
        crate::comparacao::ramo("cql_1 numero do contrato");
        return *((k + 0x3a) as *const i8); // cql1: o número do contrato
    }
    let k = loan(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        return *((k + 0x3a) as *const i8);
    }
    0
}


/// the rule: a club uses fixed squad numbers when its division's league has bit 2 of its options (+0x52); with a second
/// competition (e.g. a cup), it needs the bit too
/// Original: `SQUAD_MANAGER::club_uses_squad_numbers` (0x85add0, squad_manager.cpp, thiscall, ret 8 (this não lido)).
///
/// [pt] { 0x85add0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::club_uses_squad_numbers`; o this não é lido):
/// 1 se o clube usa números de camisa: a competição da divisão do clube (o id em [[c+0x57]], o objeto na tabela
/// [0xadadfc]) existe e tem o bit 2 do +0x52 e, com `outra` (Comp*), a outra competição (a da tabela pelo id [outra])
/// também tem o bit. Sem divisão ou sem a competição, 0; sem clube, a caixa da linha 0x4b0 e 0.
pub(crate) unsafe extern "thiscall" fn club_uses_squad_numbers(_this: usize, c: usize, other: usize) -> u32 {
    if c == 0 {
        show_error_at(0xa82b88, 0x4b0);
        return 0;
    }
    let div = read_u32(c + 0x57) as usize;
    if div == 0 {
        return 0;
    }
    let table = read_u32(G_COMP_OBJECT_TABLE) as usize;
    let comp = read_u32(table + read_u32(div) as usize * 4) as usize;
    if comp == 0 {
        return 0;
    }
    if *((comp + competition::SHORT_GENDER_NAME) as *const u8) & 2 == 0 {
        return 0;
    }
    if other != 0 {
        let comp2 = read_u32(table + read_u32(other) as usize * 4) as usize;
        if *((comp2 + competition::SHORT_GENDER_NAME) as *const u8) & 2 == 0 {
            return 0;
        }
    }
    crate::comparacao::ramo("cql_1 usa numeros");
    1 // cql1: usa números
}


/// the rule: numbers go from −50 to 50 (otherwise the error box); written to the contract valid at the club, or to the
/// loan contract
/// Original: `SQUAD_MANAGER::set_squad_number` (0x85a850, squad_manager.cpp, thiscall, ret 0xc (this não lido)).
///
/// [pt] { 0x85a850 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::set_squad_number`; o this não é lido): grava o
/// número `n` (char de −50 a 50; fora disso a caixa da linha 0x332) no +0x3a do contrato da pessoa `p` (0x4cf5a0) se ele
/// vale no clube `c`, senão no do contrato de empréstimo (0x4cf6d0) se ele vale no clube. Sem pessoa ou sem clube, nada.
pub(crate) unsafe extern "thiscall" fn set_squad_number(_this: usize, p: usize, c: usize, n: u32) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let loan: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    if p == 0 || c == 0 {
        return;
    }
    let n = n as u8 as i8;
    if !(-50..=50).contains(&n) {
        show_error_at(0xa82b88, 0x332);
        return;
    }
    let k = contrato(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        crate::comparacao::ramo("cql_1 grava no contrato");
        *((k + 0x3a) as *mut i8) = n; // cql1: grava no contrato
        return;
    }
    let k = loan(read_u32(G_CONTRACT_MANAGER) as usize, p);
    if k != 0 && contract_valid_for_club(k, c) {
        *((k + 0x3a) as *mut i8) = n;
    }
}



/// True if the person's id is below G_STAFF_COUNT (in the table of contracted people).
///
/// [pt] A pessoa `p` está na tabela das pessoas com contrato (id < [0xae2364], com sinal).
pub(crate) unsafe fn is_contracted_staff(p: usize) -> bool {
    (read_u32(p) as i32) < read_u32(G_STAFF_COUNT) as i32
}

/// Preferred squad numbers (index = number − 1, ending 0xff) by the player record's good positions (rating ≥ 15): GK 1,
/// defenders 2-6, midfield 6-8, attack 8-11; None if no good position.
///
/// [pt] Os números preferidos (índice = número − 1) pela posição da ficha de jogador `f` (as notas de posição de 1 a 20,
/// "boa" = 15 ou mais): goleiro (+0xf) o 1; zagueiro ou líbero (+0x11, +0x10) o 2 se joga pela direita (+0x17), o 3 se
/// pela esquerda (+0x18), senão o 4, o 5 ou o 6; volante, meia ou meia-atacante (+0x12, +0x13, +0x14) o 6, o 7 ou o 8;
/// atacante (+0x15) o 8, o 9, o 10 ou o 11. Termina em 0xff. Nenhuma das posições boas: `None` (a original lê então a
/// lista de 4 bytes sem valor na pilha, ver `comparar_allocate_squad_number`).
pub(crate) unsafe fn preferred_squad_numbers(f: usize) -> Option<[u8; 5]> {
    let b = |o: usize| *((f + o) as *const i8) >= 15;
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

/// the rule: a new arrival gets the first free number for his position: goalkeeper 1; defender 2 (right back), 3 (left)
/// or 4 to 6; defensive, central and attacking midfielder 6 to 8; striker 8 to 11 (good position = rating 15 or more);
/// otherwise the first free from 12 to 50, and lastly the highest free from 11 down; positive with a human manager,
/// negative without
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
    if p == 0 {
        return;
    }
    let squad: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let number: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(FN_SQUAD_MANAGER_GET_SQUAD_NUMBER);
    let write: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_SET_SQUAD_NUMBER);
    let mut used = [0u8; 50];
    let mut list = [0usize; 50];
    squad(c, list.as_mut_ptr(), 0);
    sort(list.as_mut_ptr() as usize, 0x32, 4, 0x540130);
    for q in list.iter_mut() {
        let x = *q;
        if x == 0 {
            continue;
        }
        if read_u32(x + 0x61) == 0 {
            *q = 0;
            continue;
        }
        if is_contracted_staff(x) {
            let n = (number(this, x, c) as i32).abs() as u8;
            if n != 0 {
                used[(n as i8 as isize - 1) as usize] = 1;
            }
        }
    }
    if !is_contracted_staff(p) || number(this, p, c) != 0 {
        return;
    }
    let pref = preferred_squad_numbers(read_u32(p + 0x61) as usize).unwrap_or([0xff; 5]);
    // o número com o sinal do clube: positivo com técnico humano, negativo sem
    let sinal = |k: u8| -> u32 {
        if human(c, 0, 1, 0, 0) != 0 {
            k.wrapping_add(1) as u32
        } else {
            0xffu8.wrapping_sub(k) as u32
        }
    };
    let mut b = 0usize;
    while pref[b] != 0xff {
        let k = pref[b];
        if used[k as i8 as usize] == 0 {
            write(this, p, c, sinal(k));
            used[k as usize] = 1;
            crate::comparacao::ramo("cql_2 numero da posicao");
            return; // cql2: o número da posição
        }
        b += 1;
        if b >= 5 {
            break;
        }
    }
    if let Some(k) = (11u8..50).find(|&k| used[k as usize] == 0) {
        write(this, p, c, sinal(k));
        used[k as usize] = 1;
    }
    if number(this, p, c) != 0 {
        return;
    }
    if let Some(k) = (0u8..=10).rev().find(|&k| used[k as usize] == 0) {
        write(this, p, c, sinal(k));
    }
}


/// Memory regions: byte +0x3a of the club and loan contracts of the given people, plus the error-box flag [0xb67a34].
///
/// [pt] O +0x3a dos contratos (o do clube e o de empréstimo) das pessoas `ps` e a marca da caixa [0xb67a34].
pub(crate) unsafe fn squad_number_regions(ps: &[usize]) -> Vec<(usize, usize)> {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let loan: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &p in ps {
        if p == 0 {
            continue;
        }
        for k in [contrato(read_u32(G_CONTRACT_MANAGER) as usize, p), loan(read_u32(G_CONTRACT_MANAGER) as usize, p)] {
            if k != 0 && !reg.contains(&(k + 0x3a, 1)) {
                reg.push((k + 0x3a, 1));
            }
        }
    }
    reg
}

/// As vagas da escalação de referência da 0x859fe0 para a 0x6e2000: os 11 titulares e os 9 reservas, cada um com as
/// posições que servem (máscaras: 1 goleiro, 4/0x10/0x40 os setores, 0x80..0x800 as faixas [inferência]).
pub(crate) const SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS: [u16; 20] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // cql2: os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x4, // os reservas
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
    let partner: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let of_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let number: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(FN_SQUAD_MANAGER_GET_SQUAD_NUMBER);
    let write: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_SET_SQUAD_NUMBER);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
    let mut stored = [0usize; 50]; // por número − 1
    let mut order = [0usize; 50];
    let mut all = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let x = read_u32(c + 0xd7 + 4 * k) as usize;
        if x != 0 && is_contracted_staff(x) {
            all[n] = x;
            n += 1;
        }
    }
    let l = partner(c, 0, 1);
    if l != 0 {
        for k in 0..50 {
            if n >= 50 {
                break;
            }
            let x = read_u32(l + 0xd7 + 4 * k) as usize;
            if x != 0 && is_contracted_staff(x) {
                all[n] = x;
                n += 1;
            }
        }
    }
    let slots = SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS;
    crate::comparacao::ramo("cql_2 escala");
    pick(all.as_ptr() as usize, n as u32, slots.as_ptr() as usize, 0xb, 9, order.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
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
    sort(order[20..].as_mut_ptr() as usize, 0x1e, 4, 0x540130);
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
    // `vez` = −1, −2... (char), a ordem de cada um; `k` = 1, 2... a mesma ordem em positivo
    for i in 0..50usize {
        let vez = (-1 - i as i32) as i8;
        let x = order[i];
        if x == 0 {
            continue;
        }
        if vez < -21 {
            let ct = of_club(read_u32(G_CONTRACT_MANAGER) as usize, x, c);
            let t = if ct != 0 { *((ct + 0x4f) as *const u8) & 0xf0 } else { 0 };
            if ct == 0 || t == 0x60 || t == 0x70 {
                write(this, x, c, 0);
                continue;
            }
        }
        let nb = (number(this, x, c) as i32).abs() as u8 as i8;
        if nb == 0 {
            if stored[i] == 0 {
                stored[i] = x;
                write(this, x, c, vez as u8 as u32);
            }
            continue;
        }
        let slot = (nb as isize - 1) as usize;
        if stored[slot] != 0 {
            write(this, x, c, 0);
            continue;
        }
        if keep != 0 {
            write(this, x, c, nb.wrapping_neg() as u8 as u32);
        } else if (nb < 12 && vez < -11) || (nb >= 12 && vez > -12) {
            crate::comparacao::ramo("cql_2 perde o numero");
            write(this, x, c, 0); // cql2: o número fora do lugar
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
        let ct = of_club(read_u32(G_CONTRACT_MANAGER) as usize, x, c);
        if ct == 0 {
            continue;
        }
        let t = *((ct + 0x4f) as *const u8) & 0xf0;
        if t == 0x60 || t == 0x70 {
            continue;
        }
        if number(this, x, c) == 0 {
            dar(this, c, x);
        }
    }
    if human(c, 0, 1, 0, 0) != 0 {
        for (k, &x) in stored.iter().enumerate() {
            if x != 0 {
                write(this, x, c, k as u32 + 1);
            }
        }
    } else {
        let mut v = -1i8;
        for &x in stored.iter() {
            if x != 0 {
                crate::comparacao::ramo("cql_2 regrava negativo");
                write(this, x, c, v as u8 as u32); // cql2: regrava negativo
            }
            v = v.wrapping_sub(1);
        }
    }
}



/// As ligas que a 0x859dc0 da original mandou numerar (gravadas pelo comparador da 0x859e20 enquanto o da 0x859dc0
/// roda a original), para conferir o plano da cópia.
pub(crate) static mut SQUAD_NUMBERS_PLAN: Option<Vec<usize>> = None;

/// Calls f for each competition in G_COMP_OBJECT_TABLE that is a league division and uses squad numbers (bit 2 of
/// +0x52), as 0x859dc0 numbers them.
///
/// [pt] As ligas que a 0x859dc0 numera: as competições da tabela [0xadadfc] (conta [0xadadf8], relida a cada volta) que
/// são liga de divisão (0x4b0700 com o id [[+4]], cdecl de 1) e usam números (bit 2 do +0x52).
pub(crate) unsafe fn for_each_numbered_league(mut cada: impl FnMut(usize)) {
    let division: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(FN_COMP_UTIL_IS_LEAGUE_DIVISION);
    let mut i = 0i32;
    while i < read_u32(G_COMP_OBJECT_COUNT) as i32 {
        let c = read_u32(read_u32(G_COMP_OBJECT_TABLE) as usize + 4 * i as usize) as usize;
        if c != 0 && division(read_u32(read_u32(c + 4) as usize) as i32) != 0 && *((c + 0x52) as *const u8) & 2 != 0 {
            crate::comparacao::ramo("cql_3 numera a liga");
            cada(c); // cql3: numera a liga
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
    let numerar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ASSIGN_LEAGUE_SQUAD_NUMBERS);
    *(this as *mut u16) = 0;
    for_each_numbered_league(|c| numerar(this, c, 1, 0));
}


/// the rule: each club of the league (except B teams) is numbered; at the start each player first gets the database
/// number (0 to 50); a human-managed club is not numbered by the game: its numbers become positive and the manager gets
/// the squad numbers request
/// Original: `SQUAD_MANAGER::assign_league_squad_numbers` (0x859e20, squad_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x859e20 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::assign_league_squad_numbers`): numera os clubes
/// da liga `liga` (a lista do método +0x58 da competição, thiscall(liga, &lista, 0), devolve a conta em ax; a lista é
/// solta no fim pela 0x9452ca). Em 1998 vale sempre `inicial`. Pula o time B (0x540a50(clube, &principal, 1) com o
/// clube ligado e principal = 0). Clube de técnico humano (0x7875e0, com ecx = [0xb7431c]) sem `automatico`: os números
/// negativos do elenco (0x541f90(clube, lista, 0)) viram positivos e o técnico recebe o pedido da lista de números
/// (0x85aec0, original). Senão: com `inicial`, cada jogador do elenco (da tabela das pessoas com contrato e com ficha)
/// recebe o número do banco (o +4 da ficha, entre 0 e 50); e o elenco é numerado (0x859fe0(clube, 1)).
pub(crate) unsafe extern "thiscall" fn assign_league_squad_numbers(this: usize, league: usize, mut inicial: u32, automatico: u32) {
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let squad: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let request: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SQUAD_MANAGER_SEND_SQUAD_NUMBERS_REQUEST_NEWS);
    let number: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(FN_SQUAD_MANAGER_GET_SQUAD_NUMBER);
    let write: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_SET_SQUAD_NUMBER);
    let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ASSIGN_CLUB_SQUAD_NUMBERS);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if *(G_CURRENT_DATE_YEAR as *const u16) == 0x7ce {
        inicial = 1;
    }
    let clubs: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(read_u32(read_u32(league) as usize + 0x58) as usize);
    let mut list = 0usize;
    let n = clubs(league, &mut list, 0) as i16 as i32;
    for i in 0..n.max(0) as usize {
        let c = read_u32(list + 4 * i) as usize;
        let mut principal = 0u32;
        if partner(c, &mut principal, 1) != 0 && principal == 0 {
            continue;
        }
        let mut sq = [0usize; 50];
        squad(c, sq.as_mut_ptr(), 0);
        if human(read_u32(G_NEWS) as usize, c) != 0 && automatico == 0 {
            for &p in sq.iter() {
                if p != 0 && number(this, p, c) < 0 {
                    write(this, p, c, (number(this, p, c) as u8).wrapping_neg() as u32);
                }
            }
            request(this, c);
            continue;
        }
        if inicial != 0 {
            for &p in sq.iter() {
                if p == 0 || read_u32(p) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
                    continue;
                }
                let f = read_u32(p + 0x61) as usize;
                if f == 0 {
                    continue;
                }
                let b = *((f + 4) as *const i8);
                let k = if b < 0 { 0 } else if b > 0x32 { 0x32 } else { b as i32 };
                crate::comparacao::ramo("cql_3 numero do banco");
                write(this, p, c, k as u32); // cql3: o número do banco
            }
        }
        numerar(this, c, 1);
    }
    if list != 0 {
        soltar(list);
    }
}


/// As vagas da escalação de referência da 0x85aae0: os 11 titulares e até 14 reservas (as da 0x859fe0, `SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS`,
/// até a 19ª; a 20ª aqui é de goleiro).
pub(crate) const EURO_REFERENCE_LINEUP_SLOTS: [u16; 25] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, // os reservas
];

/// the European cup registration rule: the best of the reference line-up are registered (11 starters and 14
/// substitutes, or 9 at the first stage without the mark [inference: list B]); loaned players, type-5 contracts and
/// those already marked 8 or 0x10 cannot be registered; at stage 3 up to 2 registered players outside the line-up drop
/// out (minimum of 21 registered); after the first stage only those who fit the places get in
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
pub(crate) unsafe extern "thiscall" fn select_euro_registered_players(_this: usize, c: usize, etapa: u32, mark: u32) -> u8 {
    let squad: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let loan: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    let etapa = etapa as u8;
    let mut sq = [0usize; 50];
    squad(c, sq.as_mut_ptr(), 0);
    let reservas: u8 = if etapa == 1 { if mark != 0 { 0xe } else { 9 } } else { 0xe };
    let mut cand = [0usize; 50];
    let mut n = 0u8;
    let mut count = 0i8;
    let mut tirados = 0i8;
    let m6d = |p: usize| (p + 0x6d) as *mut u8;
    for &p in sq.iter() {
        if p == 0 || read_u32(p) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
            continue;
        }
        let k = contrato(read_u32(G_CONTRACT_MANAGER) as usize, p);
        *(G_CONTRACT_LOOKUP_RESULT as *mut u32) = k as u32;
        if k != 0 && *((k + 0x35) as *const u8) & 0x3f == 5 {
            continue;
        }
        if loan(read_u32(G_CONTRACT_MANAGER) as usize, p) != 0 || *m6d(p) & 0x18 != 0 {
            continue;
        }
        cand[n as i8 as usize] = p;
        n = n.wrapping_add(1);
        let a = *m6d(p);
        if a == 0 || etapa == 1 {
            continue;
        }
        if a == 1 {
            *m6d(p) = 0;
        } else if a == 4 {
            *m6d(p) = 2;
        }
        if *m6d(p) == 2 {
            count = count.wrapping_add(1);
        }
    }
    let slots = EURO_REFERENCE_LINEUP_SLOTS;
    let mut esc = [0usize; 50];
    pick(cand.as_ptr() as usize, n as u32, slots.as_ptr() as usize, 0xb, reservas as u32, esc.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
    if etapa == 3 {
        let mut d = 0i8;
        while d < n as i8 && tirados < 2 {
            let p = cand[d as usize];
            if p != 0 && *m6d(p) == 2 && !esc[..0x19].contains(&p) {
                *m6d(p) = if mark != 0 { 4 } else { 0 };
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
            if etapa == 1 {
                crate::comparacao::ramo("cql_3 inscreve na etapa 1");
                *m6d(p) = v; // cql3: inscreve
                count = count.wrapping_add(1);
            } else {
                let a = *m6d(p);
                if a != 2 && a & 8 == 0 && (count as i32) < total {
                    crate::comparacao::ramo("cql_3 inscreve depois");
                    *m6d(p) = v; // cql3: inscreve depois
                    count = count.wrapping_add(1);
                }
            }
        }
        b = (b as i8).wrapping_add(1) as i32;
    }
    count as u8
}


/// writes the u16 at +0, widened to 4 bytes, into the squad_manager.dat item and returns 1; on error the "Error" box
/// (squad_manager.cpp 0x68, 0x6f) and 0
/// Original: `SQUAD_MANAGER::save` (0x859cc0, squad_manager.cpp, thiscall).
///
/// [pt] { 0x859cc0 } (thiscall, ret 4, squad_manager.cpp): `SQUAD_MANAGER::save(save)`: grava no item `squad_manager.dat`
/// o u16 em +0 do gerente de elencos, estendido a 4 bytes (a original zera eax antes do `mov ax`), e devolve 1. Erro
/// ao abrir ou ao gravar: a caixa "Error" (squad_manager.cpp 0x68, 0x6f) e 0, sem fechar o item. Não confere o
/// contêiner nulo (o 0x9326e0 recebe como vier).
pub(crate) unsafe extern "thiscall" fn squad_manager_save(this: usize, save: usize) -> u32 {
    let open_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let write_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    let close_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    crate::comparacao::ramo("cpx_5 gravar gerente de elencos");
    let mut v = *(this as *const u16) as u32; // cpx5: o valor do gerente de elencos
    let mut z = open_item(save, 0xa82bc0, 1, 0x16, 4);
    if z == 0 {
        show_error_at(0xa82b88, 0x68);
        return 0;
    }
    if write_item(&mut v as *mut u32 as usize, 4, 1, z) != 1 {
        show_error_at(0xa82b88, 0x6f);
        return 0;
    }
    close_item(&mut z);
    1
}

