use super::*;

/// Finds the person's contract with a club in the contract list (`this`, 80-byte records): the person has up to two
/// contract indices in the staff state table (79 bytes per person, +0 and +4) and the one whose club id (+4) matches
/// wins; if neither matches, retries with the linked B team when the club is the A team. Returns the contract record or
/// 0.
///
/// [pt] { 0x4cf5f0 } (ret 8): o contrato (registros de 80 bytes em [this]) da pessoa com o clube. A
/// pessoa tem ate dois indices (tabela 0xae2d0c, 79 bytes por pessoa, +0 e +4); vale o registro
/// cujo +4 e o id do clube. Sem nenhum, tenta o clube parceiro (0x540a50) quando ele e o time A.
pub(crate) unsafe extern "thiscall" fn contract_with_club(this: usize, person: usize, clube: usize) -> usize {
    if person == 0 || clube == 0 {
        return 0;
    }
    let id = *(person as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let e = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
    let i0 = *(e as *const i32);
    if i0 < 0 {
        return 0;
    }
    let base = *(this as *const usize);
    let r0 = base.wrapping_add((i0 as usize).wrapping_mul(80));
    if r0 == 0 {
        return 0;
    }
    let id_de = |q: usize| *((q + 4) as *const u32);
    let id_club = *(clube as *const u32);
    if id_de(r0) == id_club {
        return r0;
    }
    let i1 = *((e + 4) as *const i32);
    let mut r1 = 0usize;
    if i1 >= 0 {
        r1 = base.wrapping_add((i1 as usize).wrapping_mul(80));
        if r1 != 0 && id_de(r1) == id_club {
            return r1;
        }
    }
    let mut target = clube;
    let mut eh_a = person as u32; // the original reuses the argument’s slot
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let pc = partner(clube, &mut eh_a, 1);
    if pc != 0 && eh_a == 0 {
        target = pc;
    }
    let id_target = *(target as *const u32);
    if id_de(r0) == id_target {
        return r0;
    }
    if r1 != 0 && id_de(r1) == id_target {
        return r1;
    }
    0
}

/// Returns the low nibble of byte +0x4f of the contract between person `pessoa` and club `clube` (found by the contract
/// manager at [0xae1050]) minus 10 [inference: a contract type/level]. Null club or no contract: 0; national team (id
/// >= clubs - 2 x nations) or invalid person: the club pointer with its low byte cleared.
///
/// [pt] { 0x53ce90 }: (byte +0x4f do contrato da pessoa com o clube (0x4cf5f0 sobre [0xae1050]) & 0xf)
/// - 10, para clube que nao e selecao (id abaixo de clubes - 2 x nacoes) e pessoa valida. Senao o
/// byte baixo zerado (o resto do registro fica como estava: o clube, ou 0).
pub(crate) unsafe extern "C" fn contract_level(person: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(2 * *(G_NATION_COUNT as *const i32));
    if *(clube as *const i32) >= limit || *(person as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return clube as u32 & !0xff;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let r = f(*(G_CONTRACT_MANAGER as *const usize), person, clube);
    if r == 0 {
        return 0;
    }
    ((*((r + contract::SQUAD_STATUS) as *const u8) & 0xf) as u32).wrapping_sub(10)
}









/// Address of the 79-byte staff state record of person id in G_STAFF_STATE_TABLE (+0 = main contract index).
///
/// [pt] O registro de 79 bytes da pessoa `id` em [0xae2d0c] (+0 o índice do contrato no vetor).
pub(crate) unsafe fn contract_manager_staff_state_record(id: usize) -> usize {
    read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f))
}

/// Copies a 0x50-byte contract record field by field from o to d like the original (dates via the date assign operator,
/// bit fields merged).
///
/// [pt] A cópia do contrato de 0x50 bytes de `o` para `d`, campo a campo como a original (as datas +0x25 e +0x2d pela
/// 0x417c10; os campos de bits de +0x35 e o nibble de +0x4f, mesclados em passos, acabam iguais à origem).
pub(crate) unsafe fn copy_contract(d: usize, o: usize) {
    let copy_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    for k in (0..0x1c).step_by(4) {
        write_u32_from_usize(d + k, read_u32_as_usize(o + k));
    }
    for k in 0x1c..0x21 {
        write_u8(d + k, read_u8(o + k));
    }
    write_u32_from_usize(d + contract::RELEASE_FEE, read_u32_as_usize(o + contract::RELEASE_FEE));
    let mut tmp = [0u8; 8];
    copy_data(d + contract::DATE_STARTED, tmp.as_mut_ptr(), o + contract::DATE_STARTED);
    copy_data(d + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr(), o + contract::CONTRACT_EXPIRES);
    let s = read_u8(o + contract::CONTRACT_TYPE);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ read_u8(d + contract::CONTRACT_TYPE)) & 0x3f) ^ read_u8(d + contract::CONTRACT_TYPE);
    let al = ((((s << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl;
    write_u8(d + contract::CONTRACT_TYPE, ((al ^ s) & 0x7f) ^ s);
    write_u32_from_usize(d + contract::UNKNOWN18_1, read_u32_as_usize(o + contract::UNKNOWN18_1));
    write_u8(d + 0x3a, read_u8(o + 0x3a));
    write_u16(d + 0x3b, read_u16(o + 0x3b));
    for k in [0x3dusize, 0x3e, 0x3f] {
        write_u8(d + k, read_u8(o + k));
    }
    write_u32_from_usize(d + 0x40, read_u32_as_usize(o + 0x40));
    write_u8(d + 0x44, read_u8(o + 0x44));
    write_u32_from_usize(d + 0x45, read_u32_as_usize(o + 0x45));
    write_u8(d + contract::LEAVING_ON_BOSMAN, read_u8(o + contract::LEAVING_ON_BOSMAN));
    write_u32_from_usize(d + contract::TRANSFER_ARRANGED_FOR, read_u32_as_usize(o + contract::TRANSFER_ARRANGED_FOR));
    write_u8(d + contract::TRANSFER_STATUS, read_u8(o + contract::TRANSFER_STATUS));
    let s = read_u8(o + contract::SQUAD_STATUS);
    let a = ((read_u8(d + contract::SQUAD_STATUS) ^ s) & 0xf) ^ read_u8(d + contract::SQUAD_STATUS);
    write_u8(d + contract::SQUAD_STATUS, ((a ^ s) & 0xf) ^ s);
}

/// Sets the contract type byte (+0x3d) from the job nibble (+0x4f >> 4): 1→75, 2→50, 3→40, 4/5/6/8→0, others untouched.
///
/// [pt] O tipo do contrato (+0x3d) pelo cargo (nibble alto de +0x4f): 1 → 75, 2 → 50, 3 → 40, 4, 5, 6 e 8 → 0; os outros
/// não mexem (a tabela de saltos da original).
pub(crate) unsafe fn set_contract_type_from_job(c: usize) {
    match read_u8(c + contract::SQUAD_STATUS) >> 4 {
        1 => write_u8(c + 0x3d, 0x4b),
        2 => write_u8(c + 0x3d, 0x32),
        3 => write_u8(c + 0x3d, 0x28),
        4 | 5 | 6 | 8 => write_u8(c + 0x3d, 0),
        _ => {}
    }
}

/// `CONTRACT_MANAGER::sign_contract(contract, job)` (no SEH), called by the transfer completion (0x8c9720), by the
/// accepted offer (0x8c38b0) and by the staff screen: puts the person in the contract club, handles loans, and stores
/// the contract with its value, payroll and club change.
///
/// A person without a club (+0x39) outside a loan (+0x35 = 5): added to the contract club (0x53bd10 with today); on
/// failure: 0. A new job outside a loan with no room at the current club (0x53b9a0): the cancelled-offer news
/// (0x4d64e0) and 0. A human current club (0x600a50 with 1, 1): the scouts' lists (0x7f7270) and 0x8b2880. On a loan: a
/// squad above 50, the cancelled-trial news (0x4dade0) and 0; otherwise 0x90d860, added with job 11, the end by the
/// length (0x54cdb0 and 0x54c5b0 from today), job 8 in the high nibble of +0x4f, the start today and 0x540980. The end
/// is at least 4 weeks after the start (0x54c770); the end also goes to the person's +0x46.
///
/// The place in the array (+0 pointer, +4 capacity, +8 count): the index from the 79-byte record; with a contract at
/// the same club, the old terms of +0x3a..+0x4f stay (+0x49 0, +0x4a −1) and, in the nation of [0x9cf434] for a non-EU
/// player (0x53d740), 100 days (0x54cab0) after the person's date +0x3e, the second nationality (+0x1e) becomes that
/// nation's (with the 1st from [0x9cf260] or on a draw of 3); at another club, the type by the job, +0x49/+0x4a,
/// 0x866ee0 and +0x3b zeroed. Without a record: the first free slot (+0 = −1) gets the contract (type by the job,
/// 0x866ee0), the record, the payroll (0x598370) and 1; otherwise at the end (grows by 100, realloc 0x945667 or malloc
/// 0x944e46; without memory, box 0x1637).
///
/// Then: the value (+0xc) in the person's +0x4e, the morale (+0x14, 0x544710) in type 2 with a player sheet, the
/// payroll (0x598370), and with a new job the departure from the current club (0x53c190) and the arrival at the
/// contract club (0x53bd10); on failure: the box (0x1672, 0x167a) and 0.
/// Original: `CONTRACT_MANAGER::sign_contract` (0x4cf720, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cf720 } (thiscall, ret 8, contract_manager.cpp, sem SEH): `CONTRACT_MANAGER::sign_contract(contrato, cargo)`,
/// pela conclusão da transferência (0x8c9720), pela proposta aceita (0x8c38b0) e pela tela do pessoal. Pessoa sem clube
/// (+0x39) fora do empréstimo (+0x35 = 5): inclui no clube do contrato (0x53bd10 com hoje); falhou: 0. Cargo novo fora do
/// empréstimo sem vaga no clube atual (0x53b9a0): a notícia da proposta cancelada (0x4d64e0) e 0. Clube atual humano
/// (0x600a50 com 1, 1): as listas dos olheiros (0x7f7270) e 0x8b2880. No empréstimo: elenco acima de 50, a notícia do
/// teste cancelado (0x4dade0) e 0; senão 0x90d860, inclui com o cargo 11, o fim pela duração (0x54cdb0 e 0x54c5b0 a
/// partir de hoje), o cargo 8 no nibble alto de +0x4f, o início hoje e 0x540980. O fim no mínimo 4 semanas depois do
/// início (0x54c770); o fim também em +0x46 da pessoa. O lugar no vetor (+0 ponteiro, +4 capacidade, +8 conta): o
/// índice do registro de 79 bytes; com contrato no mesmo clube, os termos antigos de +0x3a..+0x4f ficam (+0x49 0, +0x4a
/// −1) e, na nação de [0x9cf434] para quem não é comunitário (0x53d740), daqui a 100 dias (0x54cab0) depois da data +0x3e
/// da pessoa, a segunda nacionalidade (+0x1e) vira a dessa nação (com a 1ª de [0x9cf260] ou num sorteio de 3); noutro
/// clube, o tipo pelo cargo, +0x49/+0x4a, 0x866ee0 e +0x3b zerado. Sem registro: a primeira vaga livre (+0 = −1) recebe
/// o contrato (tipo pelo cargo, 0x866ee0), o registro, a folha (0x598370) e 1; senão no fim (cresce 100, realloc
/// 0x945667 ou malloc 0x944e46; sem memória, a caixa 0x1637). Depois: o valor (+0xc) em +0x4e da pessoa, a moral (+0x14,
/// 0x544710) no tipo 2 com ficha, a folha (0x598370), e com cargo novo a saída do clube atual (0x53c190) e a entrada no
/// do contrato (0x53bd10); falhou: a caixa (0x1672, 0x167a) e 0.
pub(crate) unsafe extern "thiscall" fn sign_contract(this: usize, contrato: usize, job: u32) -> u32 {
    let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
    let has_slot: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    let news_without_slot: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_OFFER_CANCELLED_NO_SPACE_NEWS);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let olheiros: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
    let f8b2880: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS);
    let squad: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let noticia_teste: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRIAL_CANCELLED_NEWS);
    let f90d860: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_REMOVE_VIRTUAL_PLAYERS);
    let days_until: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let more: unsafe extern "thiscall" fn(usize, *mut u8, i32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let more_weeks: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let copy_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mark_squad: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let more_days: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let zerar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_CONTRACT_RESET_MORALE_STATE);
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    let morale: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let remove: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    let c = contrato;
    let clube = transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB));
    let person = read_u32_as_usize(G_STAFF_TABLE).wrapping_add(read_u32_as_usize(c).wrapping_mul(staff::SIZE));
    let loan = || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5;
    if read_u32_as_usize(person + staff::CLUB_JOB) == 0 && !loan() && incluir(clube, person, job, G_CURRENT_DATE, 1, 0) == 0 {
        return 0;
    }
    if job as u8 != read_u8(person + staff::JOB_FOR_CLUB) && !loan() && has_slot(read_u32_as_usize(person + staff::CLUB_JOB), job, person) == 0 {
        news_without_slot(this, person, read_u32_as_usize(person + staff::CLUB_JOB));
        return 0;
    }
    if read_u32_as_usize(person + staff::CLUB_JOB) != 0 && human(read_u32_as_usize(person + staff::CLUB_JOB), 1, 1, 0, 0) != 0 {
        olheiros(read_u32_as_usize(G_SCOUT_MANAGER), read_u32_as_usize(read_u32_as_usize(person + staff::CLUB_JOB) + 0xcf));
        f8b2880(read_u32_as_usize(G_TRAINING_MANAGER), read_u32_as_usize(person + staff::CLUB_JOB), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if loan() {
        crate::comparison::branch("cqg_1 empréstimo");
        if (squad(clube, 0, 0) as i32) + 1 > 0x32 {
            noticia_teste(this, person, clube);
            return 0;
        }
        f90d860(read_u32_as_usize(G_VIRTUAL_STAFF), transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)), 0);
        incluir(transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)), person, 0xb, G_CURRENT_DATE, 1, 0);
        let n = days_until(c + contract::CONTRACT_EXPIRES, read_u32_as_usize(c + contract::DATE_STARTED) as u32, read_u32_as_usize(c + 0x29) as u32);
        let end = more(G_CURRENT_DATE, tmp2.as_mut_ptr(), n); // cqg1: o fim pela duração
        copy_data(c + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr(), end);
        write_u8(c + contract::SQUAD_STATUS, (read_u8(c + contract::SQUAD_STATUS) & 0xf) | 0x80);
        copy_data(c + contract::DATE_STARTED, tmp.as_mut_ptr(), G_CURRENT_DATE);
        mark_squad(person, 1);
    }
    // the end at least 4 weeks after the start
    let (ini_dia, ini_ano) = (read_i16(c + contract::DATE_STARTED), read_u16(c + 0x27));
    let end_year = read_u16(c + 0x2f);
    if ini_ano > end_year || (ini_ano == end_year && !(ini_dia < read_i16(c + contract::CONTRACT_EXPIRES))) {
        crate::comparison::branch("cqg_1 fim em 4 semanas");
        let p = more_weeks(c + contract::DATE_STARTED, tmp2.as_mut_ptr(), 4); // cqg1: as 4 semanas
        copy_data(c + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr(), p);
    }
    copy_data(person + staff::DATE_EXPIRES_CLUB, tmp.as_mut_ptr(), c + contract::CONTRACT_EXPIRES);
    let idx = read_u32_as_usize(contract_manager_staff_state_record(read_u32_as_usize(c))) as i32;
    if idx > -1 {
        let off = (idx as usize).wrapping_mul(0x50);
        let r = read_u32_as_usize(this).wrapping_add(off);
        if read_u32_as_usize(r + 4) == read_u32_as_usize(c + contract::CLUB) { // cqg1: o mesmo clube
            crate::comparison::branch("cqg_1 mesmo clube");
            let r = || read_u32_as_usize(this).wrapping_add(off);
            write_u8(c + 0x3a, read_u8(r() + 0x3a));
            for k in [0x3dusize, 0x3e, 0x3f, 0x44] {
                write_u8(c + k, read_u8(r() + k));
            }
            write_u32_from_usize(c + 0x45, read_u32_as_usize(r() + 0x45));
            write_u8(c + contract::TRANSFER_STATUS, read_u8(r() + 0x4e));
            write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
            write_u32_from_usize(c + contract::TRANSFER_ARRANGED_FOR, u32::MAX as usize);
            let a = read_u8(c + contract::SQUAD_STATUS);
            write_u8(c + contract::SQUAD_STATUS, ((read_u8(r() + 0x4f) ^ a) & 0xf) ^ a);
            let a = read_u8(c + contract::CONTRACT_TYPE);
            write_u8(c + contract::CONTRACT_TYPE, ((((read_u8(r() + 0x35) << 1) as i8 >> 1) as u8 ^ a) & 0x40) ^ a);
            write_u16(c + 0x3b, read_u16(r() + 0x3b));
            let nacao = read_u32_as_usize(clube + club::NATION);
            if nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(KEY_NATION_PORTUGAL) && comunitaria(person, 0) == 0 {
                let p = more_days(G_CURRENT_DATE, tmp2.as_mut_ptr(), 0x64);
                let (dia, ano) = (read_i16(p), read_u16(p + 2));
                let after = ano > read_u16(person + 0x40) || (ano == read_u16(person + 0x40) && dia > read_i16(person + staff::DATE_JOINED_CLUB));
                if after {
                    let first = read_u32_as_usize(person + staff::NATION);
                    if (first != 0 && read_u32_as_usize(first) == read_u32_as_usize(KEY_NATION_BRAZIL)) || sorteio(3) == 0 {
                        write_u32_from_usize(person + staff::SECOND_NATION, read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_PORTUGAL).wrapping_mul(nation::SIZE)));
                    }
                }
            }
        } else {
            set_contract_type_from_job(c);
            write_u32_from_usize(c + contract::TRANSFER_ARRANGED_FOR, u32::MAX as usize);
            write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
            zerar(c, person, transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)));
            write_u16(c + 0x3b, 0);
        }
        copy_contract(read_u32_as_usize(this).wrapping_add(off), c);
    } else {
        let count = read_u32_as_usize(this + 8) as i32;
        let mut i = 0i32;
        while i < count {
            let r = read_u32_as_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if read_u32_as_usize(r) as i32 == -1 { // cqg1: a vaga livre
                crate::comparison::branch("cqg_1 vaga livre");
                set_contract_type_from_job(c);
                write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
                write_u32_from_usize(c + contract::TRANSFER_ARRANGED_FOR, u32::MAX as usize);
                zerar(c, person, transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)));
                copy_contract(r, c);
                write_u32_from_usize(contract_manager_staff_state_record(read_u32_as_usize(r)), i as u32 as usize);
                let f = finances(read_u32_as_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)));
                if f != 0 {
                    folha(f);
                }
                return 1;
            }
            i += 1;
        }
        if count >= read_u32_as_usize(this + 4) as i32 {
            let cap = (read_u32_as_usize(this + 4) as u32).wrapping_add(0x64);
            write_u32_from_usize(this + 4, cap as usize);
            let p = if read_u32_as_usize(this) == 0 { malloc(cap.wrapping_mul(0x50)) } else { realloc(read_u32_as_usize(this), cap.wrapping_mul(0x50)) };
            write_u32_from_usize(this, p);
            if p == 0 {
                show_error_at(0x9a8190, 0x1637);
            }
        }
        set_contract_type_from_job(c);
        write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
        write_u32_from_usize(c + contract::TRANSFER_ARRANGED_FOR, u32::MAX as usize);
        zerar(c, person, transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)));
        write_u16(c + 0x3b, 0);
        write_u32_from_usize(contract_manager_staff_state_record(read_u32_as_usize(c)), read_u32_as_usize(this + 8));
        let n = read_u32_as_usize(this + 8);
        let r = read_u32_as_usize(this).wrapping_add(n.wrapping_mul(0x50));
        write_u32_from_usize(this + 8, n.wrapping_add(1));
        copy_contract(r, c);
    }
    crate::comparison::branch("cqg_1 valor na pessoa");
    write_u32_from_usize(person + staff::WAGE, read_u32_as_usize(c + contract::WAGE)); // cqg1: o valor do contrato na pessoa
    if read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 2 && read_u32_as_usize(person + staff::PLAYER) != 0 {
        morale(person, 0x14, 0);
    }
    let f = finances(read_u32_as_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)));
    if f != 0 {
        folha(f);
    }
    if job as u8 == read_u8(person + staff::JOB_FOR_CLUB) {
        return 1;
    }
    if remove(read_u32_as_usize(person + staff::CLUB_JOB), person, 0, 1, 0) == 0 {
        show_error_at(0x9a8190, 0x1672);
        return 0;
    }
    if incluir(transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)), person, job, person + staff::DATE_JOINED_CLUB, 1, 0) != 0 {
        return 1;
    }
    show_error_at(0x9a8190, 0x167a);
    0
}









/// Multiplies the three contract bonuses (+0x10, +0x14, +0x18) that are not -1 by f, truncated.
///
/// [pt] Os três bônus do contrato (+0x10, +0x14, +0x18): os que não são −1 (> −1) × `f`, truncados.
pub(crate) unsafe fn scale_contract_bonuses(c: usize, f: f64) {
    for k in 0..3 {
        let a = c + contract::GOAL_BONUS + 4 * k;
        let v = read_i32(a);
        if v > -1 {
            write_u32(a, match_pl_ftol(v as f64 * f) as u32);
        }
    }
}

/// `contract_manager_offer_contract(person, club, out, slot, factor, mode)` (SEH frame for the local STAFF_CONTRACT
/// without a destructor; x87): the contract the club offers: the desired contract × factor, the minimum release fee,
/// the cap, the end date by age, the total by age and the value capped; returns the terms or makes the offer
/// (0x4cc490).
///
/// Without a club, the club of the person's contract (the 79-byte record in [0xae2d0c] → the 0x50-byte contract in
/// [[0xae1050]], kept in [0xae1080]); none: 0. A national side (0x53b900), a flagged regen (bits 0x30 of +0xb of the
/// record), the club itself outside types 11..15 (0x53b620 of +0x3d) with the club's list at 2 or without staff and
/// manageable (0x544820), a human club (0x600a50) or a future transfer (0x8cca10): 0. The desired contract (0x85fc50
/// into the local, with the status 0x849f00; a 3 with ability (+0xb of the sheet) above 5250 becomes 2); a return equal
/// to [0x9689a0]: 0. The wage (+8) and the value (+0xc) × factor. With a signing-on fee (+0x1d): the minimum release
/// fee (+0x21) by the value of the slot's offer (0x8e7b50: in bands of 50 thousand, 1 million and 5 million) or 50
/// thousand without an offer (100 thousand below the 1st band); without a signing-on fee, with +0x1f or +0x1c, 50
/// thousand.
///
/// The cap: the club's maximum wage (0x5968d0, with the job from the high nibble of +0x4f) × (factor + 1) × 0.5 for
/// status 11, otherwise the staff one (0x597db0). The current contract with the club (0x4cf5f0). A player (+0x61): the
/// age in days (0x54cdb0) × k minus the sheet's weight (type 0x14 fixed, otherwise +0x3c, +0x34 and the person's
/// +0x5b): above [0x968df8] (or a current contract with nibble 6) the end next year (value × [0x9689c0]; bonus ×
/// [0x968928]); above [0x968df0], in 2 years (bonus × [0x968de8]); above [0x968de0] with the club's reputation above
/// the sheet's (+0xd) × [0x968978], the year by age (bonus × [0x968940]). A value above 10 thousand and an age above
/// [0x968dd8]: the total (weeks to the end × value) against ability and age (pow 0x945ff0): reduces the value for each
/// 52 weeks fewer or lowers the cap to total / weeks.
///
/// A current contract that ends later: 0; an injury (0x62ed70) above 70 days with [0xb63d28]: 0. With [0xae233c] = 1,
/// at most 5 years. A value above the cap × [0x968dc0]: 0; above the cap: the wage and the bonuses × value / max(cap,
/// 1) and the value = cap. The grievances cleared (0x867520). With `out`: copies the contract and 1; otherwise, in mode
/// 2, or with a current contract in mode 1 that ends before today + 150 days, the offer (0x4cc490); 1.
/// Original: `contract_manager_offer_contract` (0x4cda70, contract_manager.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x4cda70 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH do STAFF_CONTRACT local sem destrutor; x87):
/// `contract_manager_offer_contract(pessoa, clube, saida, vaga, fator, modo)`: o contrato que o clube oferece. Sem
/// clube, o do contrato da pessoa (o registro de 79 bytes em [0xae2d0c] → o contrato de 0x50 bytes em [[0xae1050]],
/// guardado em [0xae1080]); nenhum: 0. Seleção (0x53b900), regen marcado (bits 0x30 do +0xb do registro), o próprio clube
/// fora dos tipos 11..15 (0x53b620 de +0x3d) com a lista do clube em 2 ou sem comissão e dirigível (0x544820), clube
/// humano (0x600a50) ou transferência futura (0x8cca10): 0. O contrato desejado (0x85fc50 no local, com o estatuto
/// 0x849f00, o 3 de habilidade (+0xb da ficha) acima de 5250 vira 2); a devolução igual a [0x9689a0]: 0. O salário (+8)
/// e o valor (+0xc) × fator. Com luvas (+0x1d): o mínimo da multa (+0x21) pelo valor da oferta da vaga (0x8e7b50: em
/// faixas de 50 mil, 1 milhão e 5 milhões) ou 50 mil sem oferta (100 mil abaixo da 1ª faixa); sem luvas, com +0x1f ou
/// +0x1c, 50 mil. O teto: o salário máximo do clube (0x5968d0, com o cargo do nibble alto de +0x4f) × (fator + 1) × 0,5
/// no estatuto 11, senão o do pessoal (0x597db0). O contrato atual com o clube (0x4cf5f0). Jogador (+0x61): a idade em
/// dias (0x54cdb0) × k menos o peso da ficha (o tipo 0x14 fixo, senão +0x3c, +0x34 e o +0x5b da pessoa): acima de
/// [0x968df8] (ou contrato atual com o nibble 6) o fim no ano que vem (valor × [0x9689c0]; bônus × [0x968928]); acima de
/// [0x968df0], daqui a 2 anos (bônus × [0x968de8]); acima de [0x968de0] com a reputação do clube acima da da ficha (+0xd)
/// × [0x968978], o ano pela idade (bônus × [0x968940]). Valor acima de 10 mil e idade acima de [0x968dd8]: o total
/// (semanas até o fim × valor) contra a habilidade e a idade (pow 0x945ff0): reduz o valor a cada 52 semanas a menos ou
/// baixa o teto para total / semanas. Contrato atual que acaba depois: 0; lesão (0x62ed70) acima de 70 dias com o
/// [0xb63d28]: 0. Com [0xae233c] = 1, no máximo 5 anos. Valor acima do teto × [0x968dc0]: 0; acima do teto: o salário e
/// os bônus × valor / max(teto, 1) e o valor = teto. As queixas limpas (0x867520). Com `saida`: copia o contrato e 1;
/// senão, no modo 2, ou com contrato atual no modo 1 que acaba antes de hoje + 150 dias, a proposta (0x4cc490); 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn contract_manager_offer_contract(this: usize, person: usize, club_arg: usize, out: usize, slot: i32, factor: f64, mode: u32) -> u32 {
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let kind_11_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let future: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let slot_of: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    let status_of: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_GET_SQUAD_STATUS_FOR_CLUB);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let value_offer: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let ceiling_max: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let teto_pessoal: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_STAFF_WAGE);
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let days_until: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let clear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_ALL_GRIEVANCES);
    let copy_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let more: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let propor: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);

    // the local STAFF_CONTRACT (0x50 bytes; the original has it at [esp+0x38])
    let mut local = [0u32; 0x14];
    let c = local.as_mut_ptr() as usize;
    construct(c);
    let mut clube = club_arg;
    if clube == 0 {
        let k = if person == 0 {
            0
        } else if (read_u32_as_usize(person) as i32) < read_i32(G_STAFF_COUNT) {
            let idx = read_i32(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(person).wrapping_mul(0x4f)));
            if idx < 0 { 0 } else { (idx as usize).wrapping_mul(0x50).wrapping_add(read_u32_as_usize(read_u32_as_usize(G_CONTRACT_MANAGER))) }
        } else {
            0
        };
        write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
        if k == 0 {
            return 0;
        }
        clube = read_u32_as_usize(G_CLUB_TABLE).wrapping_add(read_u32_as_usize(k + 4).wrapping_mul(club::SIZE));
        if clube == 0 {
            return 0;
        }
    }
    if is_national_team(clube) != 0 {
        return 0;
    }
    if read_u8(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(person).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return 0;
    }
    if kind_11_15(read_u8(person + staff::JOB_FOR_CLUB) as i8) == 0 && clube == read_u32_as_usize(person + staff::CLUB_JOB) {
        let list = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856) + 0xdc);
        if read_u8(list.wrapping_add(read_u32_as_usize(clube).wrapping_mul(9)) + 4) == 2 || (read_u32_as_usize(clube + club::MANAGER) == 0 && dirigivel(clube) != 0) {
            return 0;
        }
    }
    if human(clube, 0, 1, 0, 0) != 0 {
        return 0;
    }
    if future(read_u32_as_usize(G_TRANSFER_MANAGER), person, 0) != 0 {
        return 0;
    }
    write_u32(c + contract::CLUB, read_u32_as_usize(clube) as u32);
    let mut offer = 0usize;
    if slot != -1 {
        let p = slot_of(read_u32_as_usize(G_TRANSFER_MANAGER), slot);
        if p != 0 {
            offer = read_u32_as_usize(p);
        }
    }
    let mut job: u8 = 0;
    let mut est = status_of(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), person, clube, &mut job);
    if est == 3 && read_i16(read_u32_as_usize(person + staff::PLAYER) + 0xb) > 0x1482 {
        est = 2;
    }
    // simplification: the status and the job go in dwords with the high bytes of stack leftovers in the original; 0x85fc50 and
    // 0x597db0 leem o byte
    let r = desejado(c, person, clube, slot, est as u32, job as u32, 0, 0);
    if r as f64 == read_f64_at(0x9689a0) { // 0x9689a0 = 0
        return 0;
    }
    write_u32(c + contract::UNKNOWN, match_pl_ftol(read_i32(c + contract::UNKNOWN) as f64 * factor) as u32);
    write_u32(c + contract::WAGE, match_pl_ftol(read_i32(c + contract::WAGE) as f64 * factor) as u32);
    if read_u8(c + contract::MINIMUM_FEE_RC) != 0 {
        let mut floor: i32 = 0;
        if offer != 0 {
            let v = value_offer(offer, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                floor = match_pl_ftol((v - 0x4c4b40) as f64 * read_f64_at(0x968928) + read_f64_at(0x968e28)); // 0x968928 = 1.25, 0x968e28 = 8425000
            } else if v > 0xf4240 {
                floor = match_pl_ftol((v - 0xf4240) as f64 * read_f64_at(0x968940) + read_f64_at(0x968e20)); // 0x968940 = 1.5, 0x968e20 = 2425000
            } else if v > 0xc350 {
                floor = match_pl_ftol((v - 0xc350) as f64 * read_f64_at(0x968e18) + read_f64_at(0x9688f0)); // 0x968e18 = 2.5, 0x9688f0 = 100000
            } else if read_i32(c + contract::RELEASE_FEE) < 0x186a0 {
                floor = 0x186a0;
            }
        } else if read_i32(c + contract::RELEASE_FEE) < 0xc350 {
            floor = 0xc350;
        }
        if read_i32(c + contract::RELEASE_FEE) < floor {
            write_u32(c + contract::RELEASE_FEE, floor as u32);
        }
    } else if (read_u8(c + contract::RELEGATION_RC) != 0 || read_u8(c + contract::NON_PROMOTION_RC) != 0) && read_i32(c + contract::RELEASE_FEE) < 0xc350 {
        write_u32(c + contract::RELEASE_FEE, 0xc350);
    }
    let mut ceiling: f64 = if job == 0xb {
        let nibble = (read_u8(c + contract::SQUAD_STATUS) >> 4) as u32;
        let f = finances(read_u32_as_usize(G_FINANCE_TABLE), clube);
        let m = ceiling_max(f, person, nibble);
        (factor + read_f64_at(0x967890)) * read_f64_at(0x968968) * m as f64 // cqf3: o teto do estatuto 11; 0x967890 = 1, 0x968968 = 0.5
    } else {
        let f = finances(read_u32_as_usize(G_FINANCE_TABLE), clube);
        teto_pessoal(f, job as i8) as f64
    };
    let current = contract_with(this, person, clube);
    let mut year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let sheet = read_u32_as_usize(person + staff::PLAYER);
    if sheet != 0 {
        let days = days_until(G_CURRENT_DATE, read_u32_as_usize(person + staff::DATE_OF_BIRTH) as u32, read_u32_as_usize(person + 0x14) as u32);
        let mut x = days as f64 * read_f64_at(0x968e10); // 0x968e10 = 0.002739726027
        if read_u8(sheet + player_sheet::GOALKEEPER) == 0x14 {
            x -= read_f64_at(0x968950); // 0x968950 = 2
        } else {
            let s = read_u8(sheet + player_sheet::STAMINA) as i8 as i32 + read_u8(sheet + player_sheet::NATURAL_FITNESS) as i8 as i32 + read_u8(person + staff::PROFESSIONALISM) as i8 as i32;
            x -= (s as f64 * read_f64_at(0x968a80) - read_f64_at(0x968e08)) * read_f64_at(0x968e00); // 0x968a80 = 0.3333333333, 0x968e08 = 15, 0x968e00 = 0.4
        }
        let end = |c: usize| core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32;
        if x > read_f64_at(0x968df8) || (current != 0 && read_u8(current + contract::SQUAD_STATUS) & 0xf0 == 0x60) { // 0x968df8 = 34
            if end(c) > year as u32 + 1 {
                crate::comparison::branch("cqf_3 fim no ano que vem");
                write_u16(c + 0x2f, year.wrapping_add(1)); // cqf3: o fim no ano que vem
                write_u32(c + contract::WAGE, match_pl_ftol(read_i32(c + contract::WAGE) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
            }
            scale_contract_bonuses(c, read_f64_at(0x968928)); // 0x968928 = 1.25
        } else if x > read_f64_at(0x968df0) { // 0x968df0 = 31
            if end(c) > year as u32 + 2 {
                write_u16(c + 0x2f, year.wrapping_add(2));
                write_u32(c + contract::WAGE, match_pl_ftol(read_i32(c + contract::WAGE) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
            }
            scale_contract_bonuses(c, read_f64_at(0x968de8)); // 0x968de8 = 1.3
        } else if x > read_f64_at(0x968de0) && read_i16(clube + club::REPUTATION) as f64 > read_i16(sheet + player_sheet::WORLD_REPUTATION) as f64 * read_f64_at(0x968978) { // 0x968de0 = 29, 0x968978 = 1.2
            let k = match_pl_ftol(x + read_f64_at(0x968968)) as i8; // 0x968968 = 0.5
            if end(c) as i32 > (year as i32 - k as i32) + 0x22 {
                write_u16(c + 0x2f, year.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                write_u32(c + contract::WAGE, match_pl_ftol(read_i32(c + contract::WAGE) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
            }
            scale_contract_bonuses(c, read_f64_at(0x968940)); // 0x968940 = 1.5
        }
        if read_i32(c + contract::WAGE) > 0x2710 && x > read_f64_at(0x968dd8) { // 0x968dd8 = 30
            let days_end = days_until(c + contract::CONTRACT_EXPIRES, read_u32_as_usize(G_CURRENT_DATE) as u32, read_u32_as_usize(G_CURRENT_DATE_LEAP) as u32);
            let mut weeks = days_end / 7;
            let mut total = weeks.wrapping_mul(read_i32(c + contract::WAGE));
            let mut a = game_pow(read_i16(sheet + player_sheet::CURRENT_REPUTATION) as f64, read_f64_at(0x968950)) * read_f64_at(0x968dd0); // 0x968950 = 2, 0x968dd0 = 0.0001333333333
            if !(a < read_f64_at(0x968dc8)) { // 0x968dc8 = 8000
                a = 8000.0;
            }
            let mut b = game_pow(x - read_f64_at(0x968dd8), read_f64_at(0x968950)); // 0x968dd8 = 30, 0x968950 = 2
            if !(b > read_f64_at(0x967890)) { // 0x967890 = 1
                b = 1.0;
            }
            let limit = match_pl_ftol(game_pow(a, read_f64_at(0x968950)) / b); // 0x968950 = 2
            let reduz = (limit as f64 * read_f64_at(0x968940)) < total as f64; // 0x968940 = 1.5
            let mut done = false;
            if reduz {
                while weeks > 0x68 {
                    weeks -= 0x34;
                    write_u32(c + contract::WAGE, match_pl_ftol(read_i32(c + contract::WAGE) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
                    total = weeks.wrapping_mul(read_i32(c + contract::WAGE));
                    if limit >= total {
                        done = true;
                        break;
                    }
                }
            }
            if !done && total > limit {
                let q = limit / weeks;
                if (q as f64) < ceiling {
                    crate::comparison::branch("cqf_3 teto pelo total");
                    ceiling = q as f64; // cqf3: o teto pelo total
                }
            }
        }
        year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    }
    if current != 0 {
        if core::ptr::read_unaligned((current + 0x2f) as *const u16) >= core::ptr::read_unaligned((c + 0x2f) as *const u16) {
            return 0;
        }
        if read_u32_as_usize(G_INJURY) != 0 {
            let l = lesao(read_u32_as_usize(G_INJURY), person);
            if l != 0 && read_i16(lesao(read_u32_as_usize(G_INJURY), person) + 0x2c) > 0x46 {
                return 0;
            }
            year = read_i16(G_CURRENT_DATE_YEAR) as u16;
        }
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32 > year as u32 + 5 {
        write_u16(c + 0x2f, year.wrapping_add(5));
    }
    let w = read_i32(c + contract::WAGE) as f64;
    if w > ceiling * read_f64_at(0x968dc0) { // 0x968dc0 = 1.4
        return 0;
    }
    if w > ceiling {
        crate::comparison::branch("cqf_3 acima do teto");
        let den = if read_f64_at(0x967890) > ceiling { read_f64_at(0x967890) } else { ceiling }; // 0x967890 = 1
        let ratio = w / den;
        write_u32(c + contract::UNKNOWN, match_pl_ftol(read_i32(c + contract::UNKNOWN) as f64 * ratio) as u32);
        scale_contract_bonuses(c, ratio);
        write_u32(c + contract::WAGE, match_pl_ftol(ceiling) as u32); // cqf3: o valor no teto
    }
    clear(c, 0);
    if out != 0 {
        for k in (0..0x20).step_by(4) {
            write_u32(out + k, read_u32_as_usize(c + k) as u32);
        }
        write_u8(out + contract::MANAGER_JOB_RC, read_u8(c + contract::MANAGER_JOB_RC));
        write_u32(out + contract::RELEASE_FEE, read_u32_as_usize(c + contract::RELEASE_FEE) as u32);
        let mut tmp = [0u8; 8];
        copy_data(out + contract::DATE_STARTED, tmp.as_mut_ptr(), c + contract::DATE_STARTED);
        copy_data(out + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr(), c + contract::CONTRACT_EXPIRES);
        // +0x35 by the original’s bit fields: 6 bits, 1 bit and 1 bit
        let cl = read_u8(c + contract::CONTRACT_TYPE);
        let dl = read_u8(out + contract::CONTRACT_TYPE);
        let al = ((((cl << 2) as i8 >> 2) as u8 ^ dl) & 0x3f) ^ dl;
        let d2 = (((((cl << 1) as i8 >> 1) as u8 ^ al) & 0xc0) ^ cl ^ al) & 0x7f;
        write_u32(out + contract::UNKNOWN18_1, read_u32_as_usize(c + contract::UNKNOWN18_1) as u32);
        write_u8(out + contract::CONTRACT_TYPE, d2 ^ cl);
        write_u8(out + 0x3a, read_u8(c + 0x3a));
        write_u16(out + 0x3b, read_i16(c + 0x3b) as u16);
        for k in [0x3dusize, 0x3e, 0x3f] {
            write_u8(out + k, read_u8(c + k));
        }
        write_u32(out + 0x40, read_u32_as_usize(c + 0x40) as u32);
        write_u8(out + 0x44, read_u8(c + 0x44));
        write_u32(out + 0x45, read_u32_as_usize(c + 0x45) as u32);
        write_u8(out + contract::LEAVING_ON_BOSMAN, read_u8(c + contract::LEAVING_ON_BOSMAN));
        write_u32(out + contract::TRANSFER_ARRANGED_FOR, read_u32_as_usize(c + contract::TRANSFER_ARRANGED_FOR) as u32);
        write_u8(out + contract::TRANSFER_STATUS, read_u8(c + contract::TRANSFER_STATUS));
        write_u8(out + contract::SQUAD_STATUS, read_u8(c + contract::SQUAD_STATUS));
        return 1;
    }
    let mode = mode as u8;
    let enviar = if mode == 2 {
        true
    } else if current == 0 || mode != 1 {
        false
    } else {
        let mut d = [0u8; 8];
        let p = more(G_CONTRACT_DATE_PLUS_365_DAYS, d.as_mut_ptr(), 0x96);
        let (dia, ano_d) = (read_i16(p), core::ptr::read_unaligned((p + 2) as *const u16));
        let ano_a = core::ptr::read_unaligned((current + 0x2f) as *const u16);
        ano_d < ano_a || (ano_d == ano_a && dia < read_i16(current + contract::CONTRACT_EXPIRES))
    };
    if enviar {
        // simplification: the role (+0x3d) goes in a dword with high bytes left over from edx in the original; 0x4cc490 reads the byte
        propor(this, c, slot, u32::MAX, read_u8(person + staff::JOB_FOR_CLUB) as u32, 0, 1, 0);
    }
    1
}






/// Adds delta to a signed byte (contract happiness +0x3d or morale +0x44) clamped to -100..100 like the original.
///
/// [pt] Soma `delta` ao byte (felicidade +0x3d, moral +0x44 do contrato) entre −100 e 100, como a original: a soma em 32
/// bits decide o teto e o piso; dentro deles, a soma do byte.
pub(crate) unsafe fn add_clamped_100(a: usize, delta: i32) {
    let v = read_i8(a);
    let s = v as i32 + delta;
    *(a as *mut i8) = if s > 100 { 100 } else if s < -100 { -100 } else { v.wrapping_add(delta as i8) };
}

/// Returns (this side's club, other club) of a match from +0x1c home / +0x20 away.
///
/// [pt] Os dois clubes do jogo (+0x1c mandante, +0x20 visitante): o do lado e o outro.
pub(crate) unsafe fn match_clubs_by_side(jogo: usize, side: usize) -> (usize, usize) {
    if side == 0 { (read_usize(jogo + 0x1c), read_usize(jogo + 0x20)) } else { (read_usize(jogo + 0x20), read_usize(jogo + 0x1c)) }
}

/// True when match byte +0x3a = 1 and the other club's reputation (+0x80) is below max(own rep × constant, own rep +
/// 1500): that side is skipped.
///
/// [pt] Com o byte +0x3a do jogo = 1, o lado fica de fora quando a reputação do outro clube (+0x80) fica abaixo do maior
/// entre a dele × [0x968928] e a dele + 1500 (x87 em dupla, como a original).
pub(crate) unsafe fn skip_side_by_reputation(jogo: usize, side: usize) -> bool {
    if read_i8(jogo + 0x3a) != 1 {
        return false;
    }
    let (meu, outro) = match_clubs_by_side(jogo, side);
    let r = read_i16(meu + club::REPUTATION) as i32;
    let x = r as f64 * read_f64_at(0x968928); // 0x968928 = 1.25
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > read_i16(outro + club::REPUTATION) as f64
}

/// Person (0x6e-byte record in G_STAFF_TABLE) of a match summary slot, or None when the original skips the slot (id -1
/// or out of range).
///
/// [pt] A pessoa da vaga do resumo (id em +0 e +4 diferente de −1; registro de 0x6e bytes em [0xae23bc]), ou None quando a
/// original pula a vaga. A pessoa nula (base 0) passa, como na original.
pub(crate) unsafe fn staff_of_summary_slot(slot: usize) -> Option<usize> {
    let id = read_i32(slot);
    if id == -1 || read_i32(slot + 4) == -1 {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return None;
    }
    Some(p)
}

/// The three contract bonus payments for a summary slot (as Finance::pay_bonuses); none in friendlies, the reserve
/// competition or contract types 5/6.
///
/// [pt] Os três bônus do contrato pela vaga (Finance::pay_bonuses 0x599960): +0x21 × +0x10, +0x27 × +0x14 e, com a palavra
/// do resumo = 1 e +0x22, +0x27 × +0x18 (a original usa o +0x27 de novo); cada um só com o byte e o valor positivos.
/// Sem bônus no amistoso (0x5c6710), no jogo da competição [0x9cf684] e nos contratos de tipo 5 e 6.
pub(crate) unsafe fn match_bonuses_for_slot(jogo: usize, contrato: usize, slot: usize, word: i16) -> [Option<i32>; 3] {
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let comp = read_usize(jogo + 0x14);
    if comp != 0 && (friendly(read_usize(G_FRIENDLY), comp) != 0 || read_usize(jogo) == read_usize(KEY_CLUB_COMP_RESERVE)) {
        return [None; 3];
    }
    let kind = *((contrato + contract::CONTRACT_TYPE) as *const u8) & 0x3f;
    if kind == 6 || kind == 5 {
        return [None; 3];
    }
    let g = *((slot + 0x21) as *const u8) as i32;
    let a = *((slot + 0x27) as *const u8) as i32;
    let um = (g != 0 && read_i32(contrato + contract::GOAL_BONUS) > 0).then(|| g.wrapping_mul(read_i32(contrato + contract::GOAL_BONUS)));
    let two = (a != 0 && read_i32(contrato + contract::ASSIST_BONUS) > 0).then(|| a.wrapping_mul(read_i32(contrato + contract::ASSIST_BONUS)));
    let tres = (word == 1 && read_i8(slot + 0x22) != 0 && read_i32(contrato + contract::CLEAN_SHEET_BONUS) > 0).then(|| a.wrapping_mul(read_i32(contrato + contract::CLEAN_SHEET_BONUS)));
    [um, two, tres]
}

/// `CONTRACT_MANAGER::update_after_match(match, summary)`, called by the closing of the day's match (0x6b5f00): after
/// each match, contract bonuses, happiness by minutes played, morale by a draw and by the result, and the development
/// and happiness of those left out.
///
/// With the match's +0x40 and +0x41, the winner by the score (+0x47/+0x48; with +0x47 = −1, +0x43/+0x44) and a copy of
/// the 50 squad slots (+0xd7) of each club. First, per side (club B of the partner 0x540a50 and the side skipped by
/// reputation are left out), the 20 slots of the summary (+0x190 + side × 0x625, 0x49 bytes): outside national sides,
/// the contract with the club (0x4cf5f0) pays the bonuses (match_bonuses_for_slot); the minutes (+0x30 for starters, 90
/// without a value; 90 − +0x32 for substitutes) change the happiness (+0x3d, minutes × 10 / 90, held at 50 below 50)
/// or, for national sides, the +9 of the 79-byte record; outside national sides and friendlies, the substitutes and the
/// starters with little playing time draw (2 × draw(30)) against the minutes: the happiness drops 4, +0x3e counts and
/// the morale drops (0x4cb250); otherwise +0x3f counts (with 5 or more, outside the human club, squad mark 1 in
/// 0x540980) and the morale rises (0x4cb0a0). The winner (and the loser) draws (21 − +0x57) below 4 for +3 (−3) on
/// morale +0x44; the person leaves the squad copy.
///
/// Then, those left in the copy (with a player sheet, a contract, without suspension 0x55d270, injury 0x62c400,
/// national duty 0x7757d0 or loan) develop (0x6d26b0 with the average rating [0xb74350] − 1), +0x3e counts and, from 8
/// (14 with a partner), outside the human club and the partner, they leave the first team (0x4cbb60 2 or 4 → 0x4cbd20);
/// the happiness drops 8 and the morale drops.
/// Original: `CONTRACT_MANAGER::update_after_match` (0x4ca6d0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4ca6d0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::update_after_match(jogo, resumo)`, pelo
/// fechamento da partida do dia (0x6b5f00). Com +0x40 e +0x41 do jogo, o vencedor pelo placar (+0x47/+0x48; com +0x47 =
/// −1, +0x43/+0x44) e uma cópia dos 50 do elenco (+0xd7) de cada clube. Primeiro, por lado (clube B do parceiro
/// 0x540a50 e o lado pulado pela reputação ficam de fora), as 20 vagas do resumo (+0x190 + lado × 0x625, 0x49 bytes):
/// fora das seleções, o contrato com o clube (0x4cf5f0) paga os bônus (match_bonuses_for_slot); os minutos (+0x30 dos titulares,
/// 90 sem valor; 90 − +0x32 dos reservas) mudam a felicidade (+0x3d, minutos × 10 / 90, presa em 50 abaixo de 50) ou,
/// nas seleções, o +9 do registro de 79 bytes; fora das seleções e dos amistosos, os reservas e os titulares de pouco
/// jogo sorteiam (2 × sorteio(30)) contra os minutos: a felicidade cai 4, +0x3e conta e a moral cai (0x4cb250); senão
/// +0x3f conta (com 5 ou mais, fora do clube humano, a marca de elenco 1 em 0x540980) e a moral sobe (0x4cb0a0). O
/// vencedor (e o perdedor) sorteia(21 − +0x57) abaixo de 4 para +3 (−3) na moral +0x44; a pessoa sai da cópia do
/// elenco. Depois, quem ficou na cópia (com ficha, contrato, sem suspensão 0x55d270, lesão 0x62c400, seleção 0x7757d0
/// nem empréstimo) evolui (0x6d26b0 com a nota média [0xb74350] − 1), +0x3e conta e, a partir de 8 (14 com parceiro),
/// fora do humano e do parceiro, sai do principal (0x4cbb60 2 ou 4 → 0x4cbd20); a felicidade cai 8 e a moral cai.
pub(crate) unsafe extern "thiscall" fn update_after_match(this: usize, jogo: usize, summary: usize) {
    if read_i8(jogo + 0x40) == 0 || read_i8(jogo + 0x41) == 0 {
        return;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let pay: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_BONUSES);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let mark: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    // the original sets ecx = this before 0x4cb0a0 (stdcall in the Rust copy): thiscall with 3 on the stack serves both
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_RAISE_MATCH_MORALE);
    let baixar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_LOWER_MATCH_MORALE);
    let (c47, c48) = (read_i8(jogo + 0x47), read_i8(jogo + 0x48));
    let (a, b) = if c47 == -1 { (read_i8(jogo + 0x43), read_i8(jogo + 0x44)) } else { (c47, c48) };
    let venceu = [(a > b) as u32, (a < b) as u32];
    let mut list = [[0usize; 50]; 2];
    for (lado, l) in list.iter_mut().enumerate() {
        let c = match_clubs_by_side(jogo, lado).0;
        core::ptr::copy_nonoverlapping((c + 0xd7) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let clube = match_clubs_by_side(jogo, lado).0;
        if partner(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if skip_side_by_reputation(jogo, lado) {
            continue;
        }
        let fin = finances(read_usize(G_FINANCE_TABLE), clube);
        let base = summary + lado * 0x625;
        for k in 0..20usize {
            let slot = base + 0x190 + k * 0x49;
            let Some(person) = staff_of_summary_slot(slot) else { continue };
            let contrato = if national_team(clube) != 0 {
                0
            } else {
                let c = with_club(this, person, clube);
                if c == 0 {
                    continue;
                }
                for v in match_bonuses_for_slot(jogo, c, slot, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    pay(fin, v);
                }
                c
            };
            let minutos: i8 = if k < 11 {
                let v = read_i8(slot + 0x30);
                if v > -1 { v } else { 0x5a }
            } else {
                let v = read_i8(slot + 0x32);
                if v <= -1 {
                    0
                } else {
                    let e = 0x5a - v as i32;
                    if e <= 0 { 0 } else { e as i8 }
                }
            };
            if k < 11 || read_i8(slot + 0x32) != -1 {
                let q = ((minutos as i32 * 10) / 90) as i8;
                if national_team(clube) != 0 {
                    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 9;
                    *(r as *mut i8) = read_i8(r).wrapping_add(q);
                    if read_i8(r) > 100 {
                        *(r as *mut i8) = 100;
                    }
                } else {
                    let al = read_i8(contrato + 0x3d);
                    let s = al as i32 + q as i32;
                    *((contrato + 0x3d) as *mut i8) = if al > 0x32 {
                        if s > 100 { 100 } else if s < -100 { -100 } else { al.wrapping_add(q) }
                    } else if s > 0x32 {
                        0x32
                    } else {
                        let v = al.wrapping_add(q);
                        if v > 100 { 100 } else if v < -100 { -100 } else { v }
                    };
                }
            }
            if national_team(clube) == 0 && (read_i32(jogo) < 0 || friendly(read_usize(G_FRIENDLY), read_usize(jogo + 0x14)) == 0) {
                let testa = k >= 11 || !(minutos > 0x1e || read_i8(slot + 0x1f) != 0 || read_i8(slot + 0x20) < 0x46);
                let mal = testa && {
                    crate::comparison::branch("cqa_8 testa a moral");
                    let m = read_i8(contrato + 0x44) as i32;
                    let r1 = sorteio(0x1e); // cqa8: o primeiro sorteio da moral
                    let v = r1 as f64 + (read_f64_at(0x968d68) - m as f64 * read_f64_at(0x968d70)); // 0x968d68 = 45, 0x968d70 = 0.303030303
                    let r2 = sorteio(0x1e);
                    v - r2 as f64 > minutos as f64
                };
                let p = contrato as *mut u8;
                if mal {
                    *p.add(0x3f) = 0;
                    if *p.add(0x3e) != 100 {
                        *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
                    }
                    add_clamped_100(contrato + 0x3d, -4);
                    baixar(this, person, clube, contrato);
                } else {
                    *p.add(0x3e) = 0;
                    if *p.add(0x3f) != 100 {
                        *p.add(0x3f) = (*p.add(0x3f)).wrapping_add(1);
                    }
                    if read_i8(contrato + 0x3f) >= 5 && human(clube, 0, 1, 0, 0) == 0 {
                        mark(person, 1);
                    }
                    subir(this, person, clube, contrato);
                }
            }
            if venceu[lado] != 0 {
                crate::comparison::branch("cqa_8 moral do vencedor");
                let r = sorteio(0x15 - read_i8(person + staff::AMBITION) as i32); // cqa8: o sorteio do vencedor
                if r < 4 && contrato != 0 {
                    add_clamped_100(contrato + 0x44, 3);
                }
            } else if venceu[1 - lado] != 0 {
                let r = sorteio(0x15 - read_i8(person + staff::AMBITION) as i32);
                if r < 4 && contrato != 0 {
                    add_clamped_100(contrato + 0x44, -3);
                }
            }
            if let Some(x) = list[lado].iter_mut().find(|x| **x == person) {
                *x = 0;
            }
        }
    }
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let evoluir: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH);
    let place: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let swap: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TOGGLE_SQUAD_PLACEMENT);
    for lado in 0..2usize {
        let (clube, outro) = match_clubs_by_side(jogo, lado);
        let limit: i8 = if partner(clube, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if skip_side_by_reputation(jogo, lado) {
            continue;
        }
        for i in 0..50usize {
            let person = list[lado][i];
            if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_usize(person + staff::PLAYER) == 0 || national_team(clube) != 0 {
                continue;
            }
            let contrato = with_club(this, person, clube);
            if contrato == 0 {
                continue;
            }
            if read_i32(jogo) != -2 && suspenso(read_usize(G_DISCIPLINE), person, read_usize(jogo + 0x14), 1, 0) != 0 {
                continue;
            }
            if lesao(read_usize(G_INJURY), person) != 0 || impedido(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, clube, 0, 0) != 0 {
                continue;
            }
            // on loan: the person’s second contract (+4 of the 79-byte record)
            let idx = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 4);
            if idx >= 0 && (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)) != 0 {
                continue;
            }
            let m = read_usize(G_PLAYER_STATS);
            let x = core::ptr::read_unaligned((m + 0x18) as *const f64);
            let c = read_f64_at(0x9689a0); // 0x9689a0 = 0
            // fcomp + C3: equal or unordered goes to the constant
            let rating = if !(x < c || x > c) { read_f64_at(0x968998) } else { core::ptr::read_unaligned((m + 0x10) as *const f64) / x }; // 0x968998 = 6.8
            crate::comparison::branch("cqa_8 evolui quem ficou de fora");
            evoluir(0, person, clube, outro, jogo, (rating - read_f64_at(0x967888)) as f32, 0, 0); // cqa8: a evolução de quem ficou de fora; 0x967888 = 0.05
            let p = contrato as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if read_i8(contrato + contract::UNKNOWN18_2) >= limit && human(clube, 0, 1, 0, 0) == 0 && partner(clube, core::ptr::null_mut(), 1) == 0 {
                let l = place(this, person, clube);
                if l == 2 || l == 4 {
                    swap(this, person, clube, 0);
                }
            }
            add_clamped_100(contrato + 0x3d, -8);
            baixar(this, person, clube, contrato);
        }
    }
}


/// the morale of the player record (+0x45), for the screens
/// Original: `contract_manager_get_player_morale` (0x4cc2d0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc2d0 } (stdcall, ret 4, contract_manager.cpp): `contract_manager_get_player_morale(pessoa)`: o byte +0x45 da
/// ficha (+0x61), pelas telas. O eax leva os bytes altos da pessoa, como na original.
pub(crate) unsafe extern "stdcall" fn contract_manager_get_player_morale(person: usize) -> u32 {
    (person as u32 & 0xffff_ff00) | *((read_usize(person + staff::PLAYER) + 0x45) as *const u8) as u32
}


/// Copies the zero-terminated text at a (up to 0x400 bytes) into a Vec, for the text comparers.
///
/// [pt] O texto terminado em zero em `a` (até 0x400 bytes), para os comparadores dos textos.
pub(crate) unsafe fn c_string_bytes(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// `contract_manager_write_morale_text(person, out, _)` (stdcall, ret 0xc): writes the morale word into a text, for the
/// screens.
///
/// The morale (+0x45 of the sheet): below 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", otherwise
/// "Superb", written to the output (0x66f4e0 with 0xad9c64).
/// Original: `contract_manager_write_morale_text` (0x4cc2e0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc2e0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_write_morale_text(pessoa, saída, _)`: a
/// palavra da moral (+0x45 da ficha: abaixo de 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", senão
/// "Superb") na saída (0x66f4e0 com 0xad9c64), pelas telas.
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_text(person: usize, out: usize, _x: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let m = read_i8(read_usize(person + staff::PLAYER) + 0x45);
    let f = if m < 4 {
        0x9a8320usize
    } else if m < 8 {
        0x9a82fc
    } else if m < 0xc {
        0x9a82dc
    } else if m < 0xf {
        0x9a82b8
    } else if m < 0x12 {
        0x9a8290
    } else {
        0x9a826c
    };
    format(out, f, 0xad9c64usize);
}






/// Main contract record (0x50 bytes in the contract manager) of a person via G_STAFF_STATE_TABLE, or 0 when none/out of
/// range.
///
/// [pt] O contrato principal da pessoa (índice +0 do registro de 79 bytes em [0xae2d0c], registros de 0x50 bytes em
/// [[0xae1050]]), ou 0: sem pessoa, fora das pessoas ([0xae2364]) ou sem índice.
pub(crate) unsafe fn main_contract_of_staff(person: usize) -> usize {
    if person == 0 || read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f))) as i32;
    if i < 0 { 0 } else { (i as usize).wrapping_mul(0x50).wrapping_add(read_usize(read_usize(G_CONTRACT_MANAGER))) }
}

/// Caps value w at the contract release fee (+0x21) when the release clause is active, then truncates (_ftol).
///
/// [pt] O mínimo com a cláusula de rescisão (+0x21 do contrato, se 0x867b50 a diz ativa) e o truncamento (_ftol).
pub(crate) unsafe fn cap_value_at_release_fee(contrato: usize, w: f64) -> i32 {
    let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_HAS_ACTIVE_RELEASE_CLAUSE);
    if contrato != 0 && clausula(contrato) != 0 {
        let cl = core::ptr::read_unaligned((contrato + contract::RELEASE_FEE) as *const i32) as f64;
        return match_pl_ftol(if !(w >= cl) { w } else { cl });
    }
    match_pl_ftol(w)
}

/// `CONTRACT_MANAGER::calc_player_value(person, club, mode, ability, _)`: the player value by peak age and age, record
/// ratings, ability as seen by the club manager, club nation, contract days left and release clause.
///
/// The main contract goes to [0xae1080] and the contract with his club (0x4cf5f0, into the incoming `this`). Without a
/// player sheet (+0x61): the compensation (0x8d0780 with 1) by the contract club, or 0. The base and the peak age by
/// the sheet's positions (+0x15/+0x14/+0x13/+0x12 ≥ 15: 13, 11, 10, 9 million; goalkeeper +0xf = 20: 6 million, 33;
/// otherwise 8 million; peak 28 or 29). The age factor (peak − age, with the correction above [0x9688a0]) × the sheet's
/// ratings (+0x28, +0x27, +0x43, +0x36, +0x2f, outside goalkeepers) and +0x2d²; the reputation (+9) limits the factor.
/// The ability: without a club, the current one (+0xb); with a club, the one seen by the manager (0x6e86d0 if `ability`
/// = −1, with the seen reputation 0x5407e0) × 50 + the reputation + up to 5000 × factor, × [0x968f08], between 1 and
/// 12000; the potential (+0xd) above 5000 (and 8000) before the peak pulls it up.
///
/// Value = ability² × base × the table 0x9a7f30 (by age, with experience +0x22/+0x1a) × the club's nation (+0x53: +0x7e
/// and +0x85 or the division +0x69; without a club, draw(3) + 1), with a floor. `mode`: the nation [0x9cf434] and the
/// clause. Otherwise: the contract days (with the Bosman rule 0x8d0a70) and the nations [0x9cf434], [0x9cf300],
/// [0x9cf484]; at the end the active release clause (0x867b50) limits it.
/// Original: `CONTRACT_MANAGER::calc_player_value` (0x4d0c60, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0c60 } (thiscall, ret 0x14, contract_manager.cpp): `CONTRACT_MANAGER::calc_player_value(pessoa, clube, modo,
/// habilidade, _)`. O contrato principal vai para [0xae1080] e o contrato com o clube dele (0x4cf5f0, no this de
/// entrada). Sem ficha (+0x61): a compensação (0x8d0780 com 1) pelo clube do contrato, ou 0. A base e a idade do auge
/// pelas posições da ficha (+0x15/+0x14/+0x13/+0x12 ≥ 15: 13, 11, 10, 9 milhões; goleiro +0xf = 20: 6 milhões, 33; senão
/// 8 milhões; auge 28 ou 29). O fator da idade (auge − idade, com a correção acima de [0x9688a0]) × as notas da ficha
/// (+0x28, +0x27, +0x43, +0x36, +0x2f, fora dos goleiros) e +0x2d²; a reputação (+9) limita o fator. A habilidade: sem
/// clube, a atual (+0xb); com clube, a vista pelo técnico (0x6e86d0 se `habilidade` = −1, com a reputação vista 0x5407e0)
/// × 50 + a reputação + até 5000 × fator, × [0x968f08], entre 1 e 12000; o potencial (+0xd) acima de 5000 (e 8000) antes
/// do auge a puxa. Valor = habilidade² × base × a tabela 0x9a7f30 (pela idade, com a experiência +0x22/+0x1a) × a nação
/// do clube (+0x53: +0x7e e +0x85 ou a divisão +0x69; sem clube, sorteio(3) + 1), com piso. `modo`: a nação [0x9cf434]
/// e a cláusula. Senão: os dias de contrato (com a lei Bosman 0x8d0a70) e as nações [0x9cf434], [0x9cf300], [0x9cf484];
/// no fim a cláusula de rescisão ativa (0x867b50) limita.
pub(crate) unsafe extern "thiscall" fn calc_player_value(this: usize, person: usize, clube: usize, mode: u32, ability: u32, _x: u32) -> i32 {
    let c1080 = main_contract_of_staff(person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c1080;
    let transfer_manager_contract_club = if c1080 == 0 { 0 } else { friendly_club_by_id(read_usize(c1080 + 4)) };
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = with_club(this, person, transfer_manager_contract_club);
    let sheet = read_usize(person + staff::PLAYER);
    if sheet == 0 {
        if transfer_manager_contract_club == 0 {
            return 0;
        }
        // the original sets ecx = [0xde1f54] (the Rust copy is stdcall): thiscall with 3 on the stack works for both
        let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
        return compensacao(read_usize(G_TRANSFER_MANAGER), person, transfer_manager_contract_club, 1);
    }
    let (base, auge): (f64, i8) = if read_i8(sheet + player_sheet::ATTACKER) >= 0xf {
        (13_000_000.0, 0x1c) // cqa9: a base de quem joga na frente
    } else if read_i8(sheet + player_sheet::ATTACKING_MIDFIELDER) >= 0xf {
        (11_000_000.0, 0x1d)
    } else if read_i8(sheet + player_sheet::MIDFIELDER) >= 0xf {
        (10_000_000.0, 0x1d)
    } else if read_i8(sheet + player_sheet::DEFENSIVE_MIDFIELDER) >= 0xf {
        (9_000_000.0, 0x1d)
    } else if read_i8(sheet + player_sheet::GOALKEEPER) == 0x14 {
        (6_000_000.0, 0x21)
    } else {
        (8_000_000.0, 0x1d)
    };
    let age = read_i8(person + staff::YEAR_OF_BIRTH);
    let dif = auge as i32 - age as i32;
    let mut t = dif as f64;
    if t > read_f64_at(0x9688a0) { // 0x9688a0 = 5
        t = read_f64_at(0x968f10) - t * read_f32_at(0x968f18) as f64; // 0x968f10 = 8, 0x968f18 = 0.5
    }
    let a = match_pl_ftol(t).wrapping_abs();
    let mut f = t * a as f64 * read_f32_at(0x968f08) as f64 * read_f64_at(0x968f00) + read_f64_at(0x967890); // 0x968f08 = 0.3333333433, 0x968f00 = 0.08333333333, 0x967890 = 1
    if read_i8(sheet + player_sheet::GOALKEEPER) < 0xf {
        let rating = |o: usize, c: usize| (read_i8(sheet + o) as i32 - 0xf) as f64 * read_f64_at(c) + read_f64_at(0x967890); // 0x967890 = 1
        let fa = rating(0x28, 0x968ef8);
        let fb = read_i8(sheet + player_sheet::FINISHING) as f64 * read_f64_at(0x968ef0) + read_f64_at(0x967890); // 0x968ef0 = 0.005, 0x967890 = 1
        let fc = read_i8(sheet + player_sheet::VISION) as f64 * read_f64_at(0x968e60) + read_f64_at(0x967890); // 0x968e60 = 0.003333333333, 0x967890 = 1
        let fd = rating(0x36, 0x968ef8);
        let fe = rating(0x2f, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = read_i8(sheet + player_sheet::INJURY_PRONENESS) as i32;
    let factor = ((read_f64_at(0x968ee0) - (q * q) as f64 * read_f64_at(0x967888) * read_f64_at(0x968ee8)) * f) as f32; // 0x968ee0 = 1.33, 0x967888 = 0.05, 0x968ee8 = 0.034
    let mut r = read_i16(sheet + 9) as f64 * read_f64_at(0x968ed8) + read_f64_at(0x968ed0); // 0x968ed8 = 0.0003333333333, 0x968ed0 = 0.17
    if !(r >= read_f64_at(0x9689c0)) { // 0x9689c0 = 1.1
        r = read_f64_at(0x9689c0); // 0x9689c0 = 1.1
    }
    let x: f32 = if !(factor as f64 >= read_f64_at(0x967898)) { // 0x967898 = 0.01
        read_f64_at(0x967898) as f32 // 0x967898 = 0.01
    } else if !(r >= factor as f64) {
        r as f32
    } else {
        factor
    };
    let mut v: u32 = if clube == 0 {
        read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32 as u32
    } else {
        let vista: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
        let manager = read_usize(clube + club::MANAGER);
        let rep = vista(person, manager, 0) as i16 as i32;
        let h = if ability as u16 == 0xffff {
            let hab: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
            hab(manager, person, clube, 1, rep as i16) as u16
        } else {
            ability as u16
        };
        // only the ax of h × 50 counts (movsx afterwards); negative becomes 0
        let h50 = (h.wrapping_mul(50) as i16).max(0);
        let ceiling = if (rep as u32) < 0x1388 { rep as u32 } else { 0x1388 };
        let s = (h50 as i32).wrapping_add(rep) as u32;
        let y = match_pl_ftol((s as f64 + ceiling as f64 * x as f64) * read_f32_at(0x968f08) as f64) as u32; // 0x968f08 = 0.3333333433
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = read_i16(sheet + player_sheet::WORLD_REPUTATION);
    if pot > 0x1388 && (age as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && age <= auge {
            let z = pot as f64 * read_f64_at(0x968ec8) + (v >> 1) as f64 - read_f64_at(0x968ec0); // 0x968ec8 = 2.25, 0x968ec0 = 12500
            let e = v as f64;
            v = match_pl_ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * read_f64_at(0x968eb8); // 0x968eb8 = 1e-08
    let mut age_ef = age;
    let exp = *((person + staff::INT_APPS) as *const u8);
    if exp != 0 && read_usize(person + staff::NATION) != 0 {
        let n = (read_i16(read_usize(person + staff::NATION) + 0x8e) as i32).wrapping_mul(exp as i32) / 18;
        let n = n as i16 as i32;
        let m = if n < dif { n } else { dif };
        let sum = if m < 0 { 0 } else if n >= dif { dif } else { n };
        age_ef = age_ef.wrapping_add(sum as i8);
    }
    let mut i = auge.wrapping_sub(age_ef).wrapping_add(10); // cqa9: o índice da tabela pela idade
    if i < 0 {
        i = 0;
    } else if i >= 0x14 {
        i = 0x13;
    }
    let table = *((0x9a7f30 + i as usize) as *const i8) as i32;
    let mut w = table as f64 * vv * read_f64_at(0x967898); // 0x967898 = 0.01
    let c890 = read_f64_at(0x967890); // 0x967890 = 1
    let k = if clube == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        (sorteio(3) + 1) as f64
    } else {
        let div = read_usize(clube + club::DIVISION);
        let nat = read_usize(clube + club::NATION);
        if nat == 0 {
            c890
        } else {
            let mut t = read_i8(nat + nation::STATE_OF_DEVELOPMENT) as f64;
            if !(t >= c890) || t > read_f64_at(0x968930) { // 0x968930 = 3
                t = read_f64_at(0x968930); // 0x968930 = 3
            }
            if div != 0 {
                t = (t + c890) * read_f64_at(0x968968); // 0x968968 = 0.5
            }
            match read_i8(clube + club::PROFESSIONAL_STATUS) {
                2 => t = t + t,
                3 => t *= read_f64_at(0x968eb0), // 0x968eb0 = 7.5
                _ => {}
            }
            let n = if div != 0 { read_i16(div + competition::CLUB_COMP_REPUTATION) as i32 } else { read_i8(nat + nation::LEAGUE_STANDARD) as i32 };
            let e = n.wrapping_mul(n).wrapping_mul(25).wrapping_add(read_i16(clube + club::REPUTATION) as i32);
            e as f64 / (t * read_f64_at(0x968908)) // 0x968908 = 1000
        }
    };
    let dd = read_f64_at(0x968ea8) - k; // 0x968ea8 = 20
    let mut l = c890 - dd * dd * read_f64_at(0x968918) * read_f64_at(0x968a98); // 0x968918 = 0.2, 0x968a98 = 0.02
    if !(l >= read_f64_at(0x967880)) { // 0x967880 = 0.1
        l = read_f64_at(0x967880); // 0x967880 = 0.1
    } else if l > read_f64_at(0x9689c0) { // 0x9689c0 = 1.1
        l = read_f64_at(0x9689c0); // 0x9689c0 = 1.1
    }
    w = l * w;
    let k2 = k * k;
    if !(k2 * read_f64_at(0x968ea0) >= w) { // 0x968ea0 = 65000
        w = w * read_f64_at(0x9689e0) + k2 * read_f64_at(0x968e98); // 0x9689e0 = 0.25, 0x968e98 = 32500
    }
    let floor = k * read_f64_at(0x9688e8); // 0x9688e8 = 5000
    if !(w >= floor) {
        w = floor;
    }
    let nation_and = |g: usize| -> bool {
        let nat = read_usize(clube + club::NATION);
        clube != 0 && nat != 0 && read_usize(nat) == read_usize(g)
    };
    // the nation [0x9cf434]: below [0x968e90] the curve with a floor, otherwise × [0x9689d8]
    let curva = |w: f64, floor: f64| -> f64 {
        if !(w >= read_f64_at(0x968e90)) { // 0x968e90 = 6000000
            let v = (w * read_f64_at(0x968e88) * read_f64_at(0x968e48) + read_f64_at(0x968e80)) * w; // 0x968e88 = 1.666666667e-07, 0x968e48 = 0.6, 0x968e80 = 0.3
            if !(read_f64_at(0x9688e8) > v) { v } else { floor } // 0x9688e8 = 5000
        } else {
            w * read_f64_at(0x9689d8) // 0x9689d8 = 0.9
        }
    };
    if mode as u8 != 0 {
        if nation_and(KEY_NATION_PORTUGAL) {
            w = curva(w, 5000.0);
        }
        return cap_value_at_release_fee(contrato, w);
    }
    let mut nations = true; // 0x4d1480 (the other nations) or 0x4d14ad (the [0x9cf434] one before)
    if contrato == 0 {
        w = read_f64_at(0x9689a0); // 0x9689a0 = 0
    } else {
        let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_CLUB_USES_BOSMAN);
        if bosman(read_usize(G_TRANSFER_MANAGER), person) != 0 {
            let year = |a: usize| core::ptr::read_unaligned(a as *const u16) as i32;
            let mut days = (year(contrato + 0x2f) - year(0xae2c92)).wrapping_mul(365) + read_i16(contrato + contract::CONTRACT_EXPIRES) as i32 - read_i16(G_CURRENT_DATE) as i32;
            if days < 1 {
                return 0;
            }
            let mut w2 = w;
            if clube == transfer_manager_contract_club && read_i8(contrato + 0x3d) > 0 {
                if *((contrato + contract::CONTRACT_TYPE) as *const u8) & 0x3f == 2 {
                    days = days + 7 * read_i8(contrato + 0x3d) as i32 + 0xc8;
                } else {
                    w2 = w * read_f64_at(0x968968); // 0x968968 = 0.5
                }
            }
            let x = days as f64 * read_f64_at(0x968e78); // 0x968e78 = 0.001428571429
            if days < 0x15e { // cqa9: o contrato curto
                w = w2 * x;
            } else {
                let mut dv = read_f64_at(0x968e18) - x; // 0x968e18 = 2.5
                if !(dv >= c890) {
                    dv = read_f32_at(0x968e70) as f64; // 0x968e70 = 1
                }
                if dv > read_f64_at(0x968950) { // 0x968950 = 2
                    dv = read_f32_at(0x9689a8) as f64; // 0x9689a8 = 2
                }
                w = w2 / dv;
            }
        }
        nations = false;
    }
    if !nations {
        // 0x4d14ad: zero (or unordered) value goes to the other nations; with no club, to the end
        if !(w < read_f64_at(0x9689a0) || w > read_f64_at(0x9689a0)) { // 0x9689a0 = 0
            nations = true;
        } else if clube == 0 {
            return cap_value_at_release_fee(contrato, w);
        } else if nation_and(KEY_NATION_PORTUGAL) {
            w = curva(w, read_f64_at(0x9688e8)); // 0x9688e8 = 5000
            return cap_value_at_release_fee(contrato, w);
        } else {
            nations = true;
        }
    }
    if nations && clube != 0 {
        if nation_and(KEY_NATION_FRANCE) {
            w *= read_f64_at(0x9689b0); // 0x9689b0 = 0.8
        } else if nation_and(KEY_NATION_SOUTH_KOREA) {
            w *= read_f64_at(0x9689e0); // 0x9689e0 = 0.25
            let c = read_f64_at(0x968e68); // 0x968e68 = 125000
            if w > c {
                w = match_pl_ftol((w - c) * read_f64_at(0x968968) + c) as f64; // 0x968968 = 0.5
            }
            let c = read_f64_at(0x9688e0); // 0x9688e0 = 250000
            if w > c {
                w = match_pl_ftol((w - c) * read_f64_at(0x968968) + c) as f64; // 0x968968 = 0.5
            }
        }
    }
    cap_value_at_release_fee(contrato, w)
}

/// Memory regions the calc_player_value comparer watches: RNG state, date statics and the contract lookup global
/// 0xae1080.
///
/// [pt] As estáticas das datas (DATE_OP_STATICS) e o contrato guardado em [0xae1080]: o que a original e as chamadas dela
/// gravam, com o estado do sorteador (na pilha: a rotina é quente).
pub(crate) fn player_value_regions() -> [(usize, usize); 11] {
    let (e, c) = (crate::rng::STATE_REGIONS, DATE_OP_STATICS);
    [e[0], e[1], (G_CONTRACT_LOOKUP_RESULT, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}


/// `contract_manager_write_morale_comparison_text(a, b, out, _)` (stdcall, ret 0x10): writes the morale difference of
/// two players as a word, for the comparison screen.
///
/// The difference of the morales (+0x45 of the sheets, a − b, as a byte) becomes "Superior" (down to −14 and from 13),
/// "Better" (−13 to −8 and 7 to 12), "Marginal" (−7 to −4 and 3 to 6) or "Same" (−3 to 2) in the output (0x66f4e0 with
/// 0xad9c64); returns the difference (the game reads al).
/// Original: `contract_manager_write_morale_comparison_text` (0x4cc3a0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc3a0 } (stdcall, ret 0x10, contract_manager.cpp): `contract_manager_write_morale_comparison_text(a, b, saída,
/// _)`: a diferença das morais (+0x45 das fichas, a − b, em byte) vira "Superior" (até −14 e desde 13), "Better" (−13 a
/// −8 e 7 a 12), "Marginal" (−7 a −4 e 3 a 6) ou "Same" (−3 a 2) na saída (0x66f4e0 com 0xad9c64); devolve a diferença
/// (o jogo lê o al), pela tela de comparação.
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_comparison_text(a: usize, b: usize, out: usize, _x: u32) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let dif = read_i8(read_usize(a + staff::PLAYER) + 0x45).wrapping_sub(read_i8(read_usize(b + staff::PLAYER) + 0x45));
    let f = if dif < -13 {
        0x9a8348usize
    } else if dif < -7 {
        0x9a83bc
    } else if dif < -3 {
        0x9a8394
    } else if dif < 3 {
        0x9a8370
    } else if dif < 7 {
        0x9a8394
    } else if dif < 13 {
        0x9a83bc
    } else {
        0x9a8348
    };
    format(out, f, 0xad9c64usize);
    dif as u8
}



/// `CONTRACT_MANAGER::get_squad_flags_for_club(person, club)`, called by the club screens: returns the squad flags the
/// club screen shows.
///
/// The person's squad flags (+0x5e without the national side bits, & 0xf3), unless she is on loan (second contract, +4
/// of the 79-byte record) to a club other than `club` or its partner (0x540a50): with a partner, 1; otherwise 2 with
/// bit 0x40 of +0x35 of the main contract (the reserve bit), 1 without. Without a club, the "Error" box (the original).
/// The game reads al.
/// Original: `CONTRACT_MANAGER::get_squad_flags_for_club` (0x4cc130, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc130 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_flags_for_club(pessoa, clube)`,
/// pelas telas do clube: as marcas de elenco da pessoa (+0x5e sem os bits de seleção, & 0xf3), a não ser que ela esteja
/// emprestada (segundo contrato, +4 do registro de 79 bytes) a outro clube que não o `clube` nem o parceiro dele
/// (0x540a50): com parceiro, 1; senão 2 com o bit 0x40 de +0x35 do contrato principal, 1 sem. Sem clube, a caixa
/// "Error" (a original). O jogo lê o al.
pub(crate) unsafe extern "thiscall" fn get_squad_flags_for_club(this: usize, person: usize, clube: usize) -> u8 {
    if clube == 0 || person == 0 {
        // simplification: without a club the error box; without a person the original reads [0x5e] and crashes: both stay with the original
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, person, clube);
    }
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let geral = || *((person + staff::PLAYING_SQUAD) as *const u8) & 0xf3;
    if read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return geral();
    }
    let reg = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f));
    let i = read_usize(reg + 4) as i32;
    if i < 0 {
        return geral();
    }
    let c = (i as usize).wrapping_mul(0x50).wrapping_add(read_usize(this));
    if c == 0 || read_usize(c + 4) == read_usize(clube) {
        return geral();
    }
    let p = partner(clube, core::ptr::null_mut(), 1);
    if p != 0 && read_usize(c + 4) == read_usize(p) {
        return geral();
    }
    if partner(clube, core::ptr::null_mut(), 1) != 0 {
        return 1;
    }
    let i0 = read_usize(reg) as i32;
    if i0 < 0 {
        // the original reads [0x35] of the null contract and crashes
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, person, clube);
    }
    let c0 = (i0 as usize).wrapping_mul(0x50).wrapping_add(read_usize(this));
    (*((c0 + contract::CONTRACT_TYPE) as *const u8) & 0x40 != 0) as u8 + 1
}







/// Calls STAFF_CONTRACT::calc_contract_value (contract, person, offer) → value.
///
/// [pt] O valor de um contrato para a pessoa (STAFF_CONTRACT::calc_contract_value 0x866830: thiscall(contrato, pessoa,
/// oferta), ret 8).
pub(crate) unsafe fn calc_contract_value(contrato: usize, person: usize, offer: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_CALC_CONTRACT_VALUE);
    f(contrato, person, offer)
}



/// Writes a person's id into news slot 2 and the +0x33 keys of +4/+8/+0xc into slots 3-5.
///
/// [pt] Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
pub(crate) unsafe fn set_news_staff_slots(n: usize, p: usize) {
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    write(n, 2, read_usize(p) as u32);
    write(n, 3, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(n, 4, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(n, 5, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
}

/// `CONTRACT_MANAGER::post_wants_new_contract_news(contract)` (SEH frame for the local news item), called by the daily
/// staff pass (0x5ce2d0): posts news item 0xbc2, that the player wants a new contract, to the club manager, unless he
/// is transfer listed or in an offer.
///
/// Without a contract: the "Error" box (line 0x2d91). A person on the transfer list (0x8c1870), with offers (0x8c5cf0)
/// or offered in exchange (0x8c5e40): nothing. Otherwise news item 0xbc2 to the manager of the contract club
/// (0x7861e0): the club, the person, the squad status (+0x4f >> 4), 0, the contract end (+0x2d signed day, +0x2f year),
/// the wage (+0xc) and, in slot 11, 1 if the adaptability (+0x59) is below 10, +0x5b below 5 and the sheet's reputation
/// (+0xd) above 4750.
/// Original: `post_wants_new_contract_news` (0x4d9f80, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d9f80 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_wants_new_contract_news(contrato)`, pelo passe diário do pessoal (0x5ce2d0). Sem contrato: a caixa "Error"
/// (linha 0x2d91). Pessoa na lista de transferências (0x8c1870), com ofertas (0x8c5cf0) ou oferecida em troca (0x8c5e40):
/// nada. Senão a notícia 0xbc2 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, a situação no elenco (+0x4f
/// >> 4), 0, o fim do contrato (+0x2d dia com sinal, +0x2f ano), o salário (+0xc) e, na vaga 11, 1 se a adaptabilidade
/// (+0x59) for menor que 10, a +0x5b menor que 5 e a reputação da ficha (+0xd) passar de 4750.
pub(crate) unsafe extern "thiscall" fn post_wants_new_contract_news(this: usize, contrato: usize) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2d91);
    }
    let in_list: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_QUEUED_FOR_FREE_TRANSFER);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let clube = friendly_club_by_id(read_usize(contrato + contract::CLUB));
    let p = staff_records_staff_by_id(read_usize(contrato));
    let tm = read_usize(G_TRANSFER_MANAGER);
    if in_list(tm, p) != 0 || offers(tm, p, 0, 0) != 0 || swaps(tm, p, 0) != 0 {
        return;
    }
    crate::comparison::branch("cpw_1 noticia do contrato novo");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbc2, 0);
    write(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    write(n, 6, (read_u8(contrato + contract::SQUAD_STATUS) >> 4) as u32);
    write(n, 7, 0);
    write(n, 8, read_i16(contrato + contract::CONTRACT_EXPIRES) as i32 as u32);
    write(n, 9, read_i16(contrato + 0x2f) as u16 as u32);
    write(n, 10, read_usize(contrato + contract::WAGE) as u32);
    let sheet = read_usize(p + staff::PLAYER);
    let reputado = (read_u8(p + staff::LOYALITY) as i8) < 0xa && (read_u8(p + staff::PROFESSIONALISM) as i8) < 5 && sheet != 0 && read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x128e;
    write(n, 11, reputado as u32);
    deliver(read_usize(G_NEWS), n, clube);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// `CONTRACT_MANAGER::post_transfer_request_withdrawn_news(contract, reason)` (SEH frame for the local news item):
/// posts news item 0xbc0 of a withdrawn transfer request, with the reason, to the club manager.
///
/// Without a contract: the "Error" box (line 0x2ba6). Otherwise news item 0xbc0 to the manager of the contract club
/// (0x7861e0): the club, the person, the reason and, in slot 7, 1 if the contract's +0x3f is above 1.
/// Original: `post_transfer_request_withdrawn_news` (0x4d9460, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d9460 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_transfer_request_withdrawn_news(contrato, motivo)`. Sem contrato: a caixa "Error" (linha 0x2ba6). Senão a
/// notícia 0xbc0 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, o motivo e, na vaga 7, 1 se +0x3f do
/// contrato passar de 1.
pub(crate) unsafe extern "thiscall" fn post_transfer_request_withdrawn_news(this: usize, contrato: usize, reason: u32) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2ba6);
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparison::branch("cpw_1 pedido retirado");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbc0, 0);
    let clube = friendly_club_by_id(read_usize(contrato + contract::CLUB));
    let p = staff_records_staff_by_id(read_usize(contrato));
    write(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    write(n, 6, reason);
    write(n, 7, ((read_u8(contrato + 0x3f) as i8) > 1) as u32);
    deliver(read_usize(G_NEWS), n, clube);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.






/// Main contract of a person through CONTRACT_MANAGER::get_contract (0x4cf5a0).
///
/// [pt] O contrato principal da pessoa (0x4cf5a0).
pub(crate) unsafe fn main_contract_via_manager(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    contract_of(read_usize(G_CONTRACT_MANAGER), p)
}

/// Calls Shortlist_Manager::staff_interest_in_joining_club → float interest of the person in joining the club.
///
/// [pt] O interesse da pessoa em ir para o clube (Shortlist_Manager::staff_interest_in_joining_club 0x846920: thiscall(lista,
/// pessoa, clube, salário, modo, situação, motivos), ret 0x18, float no st0).
pub(crate) unsafe fn staff_interest_in_club(list: usize, person: usize, clube: usize, salary: u32, mode: u32, status: u32) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    f(list, person, clube, salary, mode, status, 0)
}

/// O ensaio do comparador: a cópia roda sem a oferta de empréstimo (0x8bed60) nem a atualização da lista (0x843e10),
/// que alocam e mexem nas listas; devolve −2 onde faria a oferta e −3 onde atualizaria a lista.
pub(crate) static mut LOAN_OFFER_DRY_RUN: bool = false;

/// True when 8-byte date a (day i16, year u16) is before date b, compared as the original does.
///
/// [pt] A data (dia i16, ano u16) de 8 bytes em `a`, como a original a compara (o ano sem sinal, o dia com sinal).
pub(crate) unsafe fn contract_manager_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (read_i16(a + 2) as u16, read_i16(b + 2) as u16);
    aa < ab || (aa == ab && read_i16(a) < read_i16(b))
}




/// Calls 0x544670 (cdecl): whether the nation is active / allows free transfers.
///
/// [pt] A nação está ativa (0x544670, cdecl)?
pub(crate) unsafe fn contract_manager_nation_allows_free_transfers(nacao: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_NATION_ALLOWS_FREE_TRANSFERS);
    f(nacao) != 0
}

/// `CONTRACT_MANAGER::post_contract_expiry_news(contract, period, bosman, notify_interested)` (SEH frame for the local
/// news item and list), called by the contracts pass: posts news item 0xbbd of an expiring contract to the club manager
/// and, if asked, to interested clubs of active nations.
///
/// Without a contract: the "Error" box (line 0x24f0). Otherwise news item 0xbbd to the club's manager (0x7861e0): the
/// club, the person, the period (signed), `bosman` and, in slot 8, whether the person has a player sheet (+0x61). With
/// `notify_interested` (cancelled if the club's nation, +0x53, is not active, 0x544670): the clubs interested in the
/// player (0x842180 into a local LLIST, with 1, 1, 0) from another active nation also get it.
/// Original: `post_contract_expiry_news` (0x4d6790, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d6790 } (thiscall, ret 0x10, contract_manager.cpp, quadro SEH da notícia e da lista locais): `CONTRACT_MANAGER::
/// post_contract_expiry_news(contrato, período, bosman, avisar_interessados)`, pelo passe dos contratos. Sem contrato: a
/// caixa "Error" (linha 0x24f0). Senão a notícia 0xbbd ao técnico do clube (0x7861e0): o clube, a pessoa, o período (com
/// sinal), `bosman` e, na vaga 8, se a pessoa tem ficha (+0x61). Com `avisar_interessados` (anulado se a nação do clube,
/// +0x53, não estiver ativa, 0x544670): os clubes interessados no jogador (0x842180 numa LLIST local, com 1, 1, 0) de
/// outra nação ativa também a recebem.
pub(crate) unsafe extern "thiscall" fn post_contract_expiry_news(this: usize, contrato: usize, period: u32, bosman: u32, warn: u32) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x24f0);
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbbd, 0);
    let clube = friendly_club_by_id(read_usize(contrato + 4));
    let p = staff_records_staff_by_id(read_usize(contrato));
    write(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    write(n, 6, period as u8 as i8 as i32 as u32);
    write(n, 7, bosman);
    write(n, 8, (read_usize(p + staff::PLAYER) != 0) as u32);
    let nacao = read_usize(clube + club::NATION);
    let warn = if nacao != 0 && !contract_manager_nation_allows_free_transfers(nacao) { 0 } else { warn };
    deliver(read_usize(G_NEWS), n, clube);
    if warn == 0 {
        return;
    }
    crate::comparison::branch("cpw_3 avisar interessados");
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let interested: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let mut list = [0u8; 0xe];
    let l = list.as_mut_ptr() as usize;
    list_new(l);
    interested(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut x = next(l);
    while x != 0 {
        let id = read_usize(x + 4);
        if id != read_usize(clube) {
            let nac = read_usize(friendly_club_by_id(id) + 0x53);
            if nac != 0 && contract_manager_nation_allows_free_transfers(nac) {
                deliver(read_usize(G_NEWS), n, friendly_club_by_id(read_usize(x + 4)));
            }
        }
        x = next(l);
    }
    destroy(l);
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// `CONTRACT_MANAGER::post_bosman_departure_news(contract, destination)` (SEH frame for the local news item): posts
/// news item 0xbbe of a Bosman departure.
///
/// Without a contract: the "Error" box (line 0x2611). Otherwise news item 0xbbe to the club's manager (0x7861e0): the
/// club, the person and the destination club (−1 without one).
/// Original: `post_bosman_departure_news` (0x4d6d70, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d6d70 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_bosman_departure_news(contrato, destino)`. Sem contrato: a caixa "Error" (linha 0x2611). Senão a notícia 0xbbe ao
/// técnico do clube (0x7861e0): o clube, a pessoa e o clube de destino (−1 sem ele).
pub(crate) unsafe extern "thiscall" fn post_bosman_departure_news(this: usize, contrato: usize, target: usize) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2611);
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparison::branch("cpw_3 saida pela lei bosman");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xbbe, 0);
    let clube = friendly_club_by_id(read_usize(contrato + 4));
    let p = staff_records_staff_by_id(read_usize(contrato));
    write(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    write(n, 6, if target != 0 { read_usize(target) as u32 } else { 0xffff_ffff });
    deliver(read_usize(G_NEWS), n, clube);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.




/// Address of the person's contract offer counter (byte +8 of the 79-byte staff state record).
///
/// [pt] O contador de ofertas de contrato da pessoa (byte +8 do registro de 79 bytes em [0xae2d0c]).
pub(crate) unsafe fn contract_offer_counter(person: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)).wrapping_add(8)
}

/// Assigns date origem to destino with the game's date operator (0x417c10).
///
/// [pt] A atribuição de data (0x417c10: thiscall(destino, temporária, origem)).
pub(crate) unsafe fn contract_manager_copy_date(target: usize, source: usize) {
    let copy: unsafe extern "thiscall" fn(usize, *mut [u32; 2], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u32; 2];
    copy(target, &mut t, source);
}

/// Finds the contract offer of person id from club in list l without moving the cursor; 0 if none.
///
/// [pt] A oferta de contrato da pessoa (id em +0) pelo clube (id em +4) na lista `l` (nós: +0 o item, +6 o próximo), sem
/// mexer no cursor; nenhuma: 0.
pub(crate) unsafe fn find_contract_offer(l: usize, id: usize, clube: usize) -> usize {
    let mut no = read_usize(l + 2);
    let mut n = 0;
    while no != 0 && n < 100000 {
        let x = read_usize(no);
        if read_usize(x) == id && read_usize(x + 4) == clube {
            return x;
        }
        no = read_usize(no + 6);
        n += 1;
    }
    0
}

/// O ensaio do comparador: a cópia não cria nem acrescenta à lista (0x944cf1, 0x68d020, 0x68d0d0) e não chama o que vem
/// depois (0x8c1f80, 0x8c2060, 0x4cd230), que alocam, sorteiam e mexem nas ofertas.
pub(crate) static mut CONTRACT_OFFER_DRY_RUN: bool = false;

/// `CONTRACT_MANAGER::add_contract_offer(terms, slot, value, flag, a5, decide, agreed)` (SEH frame without a
/// destructor), called by the loan offer (0x8bed60), the stored offers (0x4cf4a0), the lists (0x84a020), the
/// negotiation (0x4cda70) and the screen: updates the same club's offer to the person or adds a new one to the list;
/// marks the terms as offered on the transfer offer and, if asked, schedules the decision.
///
/// Computes today + 14 (0x54c5b0, discarded) and builds a local 0x6f-byte record (contract 0x85e410, date 0x54a110).
/// With counted offers (+8 of the person's 79-byte record in [0xae2d0c]) and a list (+0x34), the offer from the same
/// club is updated: the terms (the inlined operator= 0x4cc810), +0x50 = 2 if `agreed` (otherwise 0), +0x51 = today,
/// +0x59 the slot, +0x5d the value, +0x61 the flag, +0x62 = 1 outside a type 6 contract. Otherwise the local record
/// gets the same (+0x63 = −1, the rest left as the original leaves it) and goes into the list (created when missing;
/// 0x68d0d0), and the offer count goes up. With a slot, the offer (0x8c61e0) of the same player marks the terms as
/// offered (0x8c1f80), otherwise the exchange player (0x8c2060); with `decide`, it schedules the decision (0x4cd230).
/// Original: `add_contract_offer` (0x4cc490, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc490 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH sem destrutor): `CONTRACT_MANAGER::
/// add_contract_offer(termos, vaga, valor, marca, a5, decidir, acertado)`, pela oferta de empréstimo (0x8bed60), pelas
/// ofertas guardadas (0x4cf4a0), pelas listas (0x84a020), pela negociação (0x4cda70) e pela tela. Calcula hoje + 14 (0x54c5b0,
/// descartado) e monta um registro local de 0x6f bytes (contrato 0x85e410, data 0x54a110). Com ofertas contadas (+8 do
/// registro de 79 bytes da pessoa em [0xae2d0c]) e lista (+0x34), a oferta do mesmo clube é atualizada: os termos (o
/// operator= 0x4cc810 embutido), +0x50 = 2 se `acertado` (senão 0), +0x51 = hoje, +0x59 a vaga, +0x5d o valor, +0x61 a
/// marca, +0x62 = 1 fora do contrato de tipo 6. Senão o registro local recebe o mesmo (+0x63 = −1, o resto fica como a
/// original o deixa) e entra na lista (criada se faltar; 0x68d0d0), e o contador sobe. Com vaga, a oferta (0x8c61e0) do
/// mesmo jogador marca os termos oferecidos (0x8c1f80), senão o jogador da troca (0x8c2060); com `decidir`, agenda a
/// decisão (0x4cd230).
pub(crate) unsafe extern "thiscall" fn add_contract_offer(this: usize, terms: usize, slot: i32, value: u32, mark: u32, a5: u32, decidir: u32, acertado: u32) {
    let _ = a5;
    let more: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut quinzena = [0u32; 2];
    more(G_CURRENT_DATE, &mut quinzena, 0xe);
    // simplification: bytes +0x67..+0x6e of the local record the original does not fill (stack leftover); the copy uses zero
    let mut record = [0u8; 0x6f];
    let r = record.as_mut_ptr() as usize;
    construct(r);
    data_default(r + 0x51);
    let id = read_usize(terms);
    let mut item = 0;
    if read_u8(contract_offer_counter(terms)) != 0 && read_usize(this + 0x34) != 0 {
        let clube = read_usize(terms + 4);
        let l = read_usize(this + 0x34);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let mut x = next(read_usize(this + 0x34));
        while x != 0 {
            if read_usize(x) == id && read_usize(x + 4) == clube {
                item = x;
                break;
            }
            x = next(read_usize(this + 0x34));
        }
    }
    let tipo6 = read_u8(terms + 0x35) & 0x3f == 6;
    if item != 0 {
        crate::comparison::branch("cpw_4 oferta atualizada");
        // simplification: the original inlines operator= (0x4cc810) here; the copy calls its Rust version, the same writes
        contract_manager_operator(item, terms);
        *((item + 0x50) as *mut u8) = if acertado != 0 { 2 } else { 0 };
        contract_manager_copy_date(item + 0x51, G_CURRENT_DATE);
        write_u32(item + 0x59, slot as u32);
        write_u32(item + 0x5d, value);
        *((item + 0x61) as *mut u8) = mark as u8;
        *((item + 0x62) as *mut u8) = !tipo6 as u8;
    } else {
        let operador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_OPERATOR_3D);
        operador(r, terms);
        *((r + 0x50) as *mut u8) = 0;
        contract_manager_copy_date(r + 0x51, G_CURRENT_DATE);
        write_u32(r + 0x5d, value);
        write_u32(r + 0x59, slot as u32);
        *((r + 0x61) as *mut u8) = mark as u8;
        write_u32(r + 0x63, 0xffff_ffff);
        *((r + 0x62) as *mut u8) = !tipo6 as u8;
        if CONTRACT_OFFER_DRY_RUN {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(DRY_RUN_CONTRACT_OFFER_RECORD) as *mut u8, 0x6f);
        } else {
            if read_usize(this + 0x34) == 0 {
                let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
                let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
                let p = new(0xe);
                let l = if p != 0 { list_new(p) } else { 0 };
                write_u32(this + 0x34, l as u32);
            }
            let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
            append(read_usize(this + 0x34), r, 0x6f);
        }
        crate::comparison::branch("cpw_4 oferta nova");
        let c = contract_offer_counter(terms) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: o contador de ofertas da pessoa
    }
    if CONTRACT_OFFER_DRY_RUN {
        return;
    }
    if slot != -1 {
        let slot_of: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let v = slot_of(tm, slot);
        if read_usize(read_usize(v) + 0xc) == read_usize(terms) {
            crate::comparison::branch("cpw_4 termos ao jogador");
            let marcar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_TRANSFER_MANAGER_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER);
            marcar(read_usize(G_TRANSFER_MANAGER), slot); // cpw4: os termos oferecidos ao jogador
        } else {
            let swap: unsafe extern "thiscall" fn(usize, i32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_NOTE_TERMS_OFFERED_TO_EXCHANGE_PLAYER);
            swap(read_usize(G_TRANSFER_MANAGER), slot, read_usize(terms) as u32);
        }
    }
    if decidir != 0 {
        crate::comparison::branch("cpw_4 agendar decisao");
        let decisao: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_OFFER_DECISION);
        decisao(this, read_usize(terms) as u32, slot); // cpw4: a decisão agendada
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
pub(crate) static mut DRY_RUN_CONTRACT_OFFER_RECORD: [u8; 0x6f] = [0; 0x6f];


/// `CONTRACT_MANAGER::remove_contract_offer_for_transfer(person, slot)`, called by the withdrawn and undone offers
/// (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): removes the person's contract offer linked to the
/// transfer slot and lowers the offer count.
///
/// In the list of contract offers (+0x34; cursor restarted), the person's one with the slot (+0x59). When found: with
/// +0x63 flagged, the original 0x7a81a0 of [0xb7434c] with +0x63, +0x67 and +0x6b; if agreed (+0x50 = 2), it clears the
/// agreed-contract flag (0x8c5430); removes it from the list (0x68d1f0) and the person's offer count (+8 of the 79-byte
/// record) goes down.
/// Original: `remove_contract_offer_for_transfer` (0x4ccb30, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4ccb30 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer_for_transfer(pessoa,
/// vaga)`, pelas ofertas retiradas e desfeitas (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): na lista das
/// ofertas de contrato (+0x34; cursor recomeçado) a da pessoa com a vaga (+0x59). Achada: com +0x63 marcado, a original
/// 0x7a81a0 de [0xb7434c] com +0x63, +0x67 e +0x6b; acertada (+0x50 = 2), desliga a marca de contrato acertado (0x8c5430);
/// tira da lista (0x68d1f0) e o contador de ofertas da pessoa (+8 do registro de 79 bytes) desce.
pub(crate) unsafe extern "thiscall" fn remove_contract_offer_for_transfer(this: usize, person: usize, slot: i32) {
    let l = read_usize(this + 0x34);
    if l == 0 {
        return;
    }
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut x = next(read_usize(this + 0x34));
    while x != 0 {
        if read_usize(x) == read_usize(person) && read_usize(x + 0x59) as i32 == slot {
            break;
        }
        x = next(read_usize(this + 0x34));
    }
    if x == 0 {
        return;
    }
    crate::comparison::branch("cpw_4 oferta tirada pela vaga");
    if read_usize(x + 0x63) as i32 != -1 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_PLAYER_REGEN_ADD_SPARE_NON_PLAYER_RECORD);
        f(read_usize(G_PLAYER_REGEN), read_usize(x + 0x63) as u32, read_usize(x + 0x67) as u32, read_usize(x + 0x6b) as u32);
    }
    if read_u8(x + 0x50) == 2 {
        let unmark: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
        unmark(read_usize(G_TRANSFER_MANAGER), person);
    }
    let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    remove(read_usize(this + 0x34), x);
    let c = contract_offer_counter(person) as *mut u8;
    *c = (*c).wrapping_sub(1);
}


/// The lowest wage the person asks of the club (stdcall, ret 8; SEH frame without a destructor), for the contract
/// screen: the lower of the desired wages for squad status 1 and 2 at the club, at most 10 million.
///
/// In a local contract (0x85e410), the desired one (0x85fc50 with −1, status 1 and then 2, 0xb), at most 10 million.
/// Original: `contract_manager_lowest_wage_demand` (0x4ce450, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4ce450 } (stdcall, ret 8, contract_manager.cpp, quadro SEH sem destrutor): o menor salário que a pessoa pede ao
/// clube: num contrato local (0x85e410), o desejado (0x85fc50 com −1, a situação 1 e depois 2, 0xb), no máximo 10 milhões.
/// Pela tela do contrato.
pub(crate) unsafe extern "stdcall" fn contract_manager_lowest_wage_demand(person: usize, clube: usize) -> i32 {
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let mut local = [0u8; 0x50];
    let c = local.as_mut_ptr() as usize;
    construct(c);
    let mut smaller = 0x989680;
    let a = desejado(c, person, clube, -1, 1, 0xb, 0, 0);
    if a < smaller {
        smaller = a;
    }
    let b = desejado(c, person, clube, -1, 2, 0xb, 0, 0);
    if b < smaller { b } else { smaller }
}




/// `CONTRACT_MANAGER::handle_transfer_request(contract)` (SEH frame for the local news item): handles a player's
/// transfer request.
///
/// A null contract: the "Error" box 0x2999. A person without a player sheet (+0x61): nothing. A human club (0x7875e0):
/// the request news item (0xbbf), built on the stack (by the original). At a computer club, outside squad status 1
/// (+0x4f & 0xf0 = 0x10): if the value (0x4d0c60) × 1.5 beats the asking price (0x4d1600), the club lists him (+0x4e:
/// clears bit 2 and sets 8; morale +12 in 0x544710; the club lists 0x842c00; 0x4d71b0). Otherwise a draw by the request
/// strength (+0x44) and the temperament (+0x59) against another of 50 gives the 0x10000 grievance (0x867700); the
/// morale drops 12 and the request strength grows by temperament × 5 + 56, held at ±100.
/// Original: `CONTRACT_MANAGER::handle_transfer_request` (0x4d8610, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d8610 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// handle_transfer_request(contrato)`: contrato nulo: a caixa "Error" 0x2999. Pessoa sem ficha de jogador (+0x61): nada.
/// Clube humano (0x7875e0): a notícia do pedido (0xbbf), montada na pilha. No clube do computador, fora do status de
/// elenco 1 (+0x4f & 0xf0 = 0x10): se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), o clube o põe na lista
/// (+0x4e: tira o bit 2 e liga o 8; moral +12 em 0x544710; as listas dos clubes 0x842c00; 0x4d71b0). Senão um sorteio
/// pela força do pedido (+0x44) e pelo ânimo (+0x59) contra outro de 50 dá a queixa 0x10000 (0x867700); a moral cai 12
/// e a força do pedido sobe ânimo × 5 + 56, presa em ±100.
pub(crate) unsafe extern "thiscall" fn handle_transfer_request(this: usize, contrato: usize) {
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2999);
    }
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(contrato + contract::CLUB).wrapping_mul(club::SIZE));
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(contrato).wrapping_mul(staff::SIZE));
    if read_usize(person + staff::PLAYER) == 0 {
        return;
    }
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if human(read_usize(G_NEWS), clube) != 0 {
        // simplification: the human club receives the news item built on the stack (0x77bbe0 and the delivery are switched off): that
        // case stays with the original (which repeats the 0x7875e0 query, with no effect).
        // the record serves the proof of CMRTE’s ”Deixar infeliz” (make unhappy) (31/10/2026; deixar_infeliz.py): rare call
        linha(&format!("pedido de transferencia no clube humano: pessoa {}", read_usize(contrato)));
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HANDLE_TRANSFER_REQUEST)(this, contrato);
    }
    let morale: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    if read_u8(contrato + contract::SQUAD_STATUS) & 0xf0 != 0x10 {
        let price: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
        let a = price(this, clube, 0, person, 0, 1, 1);
        let v = value(this, person, clube, 0, -1, 0);
        // the 1.5 is the double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: o valor passa do preço
            crate::comparison::branch("cps_1 listado pelo clube");
            let m = (contrato + contract::TRANSFER_STATUS) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            morale(person, 0xc, 0);
            let lists: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
            lists(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), person, 1);
            let listar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRANSFER_LISTED_NEWS);
            listar(this, person, clube);
            return;
        }
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let r = sorteio(read_u8(contrato + 0x44) as i8 as i32 + (read_u8(person + staff::LOYALITY) as i8 as i32 + 5) * 20);
    if r < sorteio(0x32) { // cps1: o sorteio da queixa
        crate::comparison::branch("cps_1 queixa do pedido");
        let queixa: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        queixa(contrato, 0x10000, 0);
    }
    crate::comparison::branch("cps_1 pedido no clube do computador");
    morale(person, 0xffff_fff4, 0); // cps1: a moral cai 12
    // reread after the calls, as in the original (the 8-bit `imul` keeps the low byte)
    let al = read_u8(person + staff::LOYALITY).wrapping_mul(5).wrapping_add(0x38);
    let cl = read_u8(contrato + 0x44);
    let s = cl as i8 as i32 + al as i8 as i32;
    let m = (contrato + 0x44) as *mut u8;
    *m = if s > 100 {
        100
    } else if s < -100 {
        0x9c
    } else {
        cl.wrapping_add(al)
    };
}

/// 1 for jobs 0, 6 to 0xd and 0xf (those holding a contract manager contract)
/// Original: `is_contract_staff_job` (0x4cbb20, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4cbb20 } (stdcall, ret 4, contract_manager.cpp): `is_contract_staff_job(cargo)`: 1 para os cargos (byte) 0, 6 a
/// 0xd e 0xf, os que têm contrato pelo gerente de contratos; senão 0.
pub(crate) unsafe extern "stdcall" fn is_contract_staff_job(job: u32) -> u32 {
    matches!(job as u8, 0 | 6..=0xd | 0xf) as u32
}

/// 0 when the job (+0x3d) is a board job (1 to 4) at a club (+0x39); otherwise 1
/// Original: `staff_is_not_board_member` (0x4cb960, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4cb960 } (stdcall, ret 4, contract_manager.cpp): `staff_is_not_board_member(pessoa)`: 0 se o cargo (+0x3d) é de
/// diretoria (1 a 4) num clube (+0x39); senão 1.
pub(crate) unsafe extern "stdcall" fn staff_is_not_board_member(person: usize) -> u32 {
    let job = read_u8(person + staff::JOB_FOR_CLUB);
    if (1..=4).contains(&job) && read_usize(person + staff::CLUB_JOB) != 0 {
        return 0;
    }
    1
}

// simplification: draws (0x90cfc0), adds to the list (0x842c00, 0x4d71b0), changes morale and complaints (which may call the request for
// new): original only; the copy takes the branches.





/// Calls one of the game's date comparison functions f (>, <, ==) on date a with date b by value.
///
/// [pt] As comparações de data do jogo (thiscall na data `a`, a outra por valor): 0x456740 `>`, 0x456770 `<`, 0x4a7f20 `==`.
pub(crate) unsafe fn call_date_compare(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, read_u32(b), read_u32(b + 4))
}

/// `compare_decision_dates(a, b)` (cdecl): the qsort order of the decision dates (14-byte entries: +0 person, +4 date,
/// +0xc time slice of the day, +0xd type).
///
/// `a`'s date earlier (0x456770): −1; later (0x456740): 1; on the same day, `a`'s slice minus `b`'s (signed). Hot (the
/// qsort): no counter.
/// Original: `compare_decision_dates` (0x4dc910, contract_manager.cpp, cdecl).
///
/// [pt] { 0x4dc910 } (cdecl, contract_manager.cpp): `compare_decision_dates(a, b)`: a ordem do qsort das datas de decisão
/// (entradas de 14 bytes: +0 pessoa, +4 data, +0xc fatia do dia, +0xd tipo): a data de `a` antes (0x456770): −1;
/// depois (0x456740): 1; no mesmo dia, a fatia de `a` menos a de `b` (com sinal). Quente (o qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_decision_dates(a: usize, b: usize) -> i32 {
    if call_date_compare(0x456770, a + 4, b + 4) != 0 {
        return -1; // a before b
    }
    if call_date_compare(0x456740, a + 4, b + 4) != 0 {
        return 1;
    }
    read_u8(a + 0xc) as i8 as i32 - read_u8(b + 0xc) as i8 as i32
}


/// consumes the decision dates due by the given date and slice: each one becomes −1 and, for type 3, the person decides
/// on the contract offers; turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: the detour alone (without running the copy) made the career
/// in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the
/// stack residue a later original reads; the copy stays in the code and does not run
/// Original: `CONTRACT_MANAGER::process_due_decision_dates` (0x4cd9c0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd9c0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::process_due_decision_dates(data, fatia)`:
/// do índice +0x18 até a conta +0x14 do vetor das datas de decisão (+0xc, entradas de 14 bytes, ordenadas pela
/// 0x4dc910): para na primeira data depois de `data` (0x456740) ou, no mesmo dia (0x4a7f20), com a fatia da entrada
/// maior que `fatia`. Cada entrada vencida com pessoa (+0 ≠ −1) fica com −1 e, sendo do tipo 3 (+0xd), a pessoa decide
/// as ofertas de contrato (0x4ce4e0). O índice +0x18 sobe a cada entrada (relido depois da decisão).
pub(crate) unsafe extern "thiscall" fn process_due_decision_dates(this: usize, data: usize, slice: u32) {
    let n = |c: usize| read_u32(c) as i32;
    if n(this + contract_manager_fields::DECISION_NEXT_INDEX) >= n(this + contract_manager_fields::DECISION_COUNT) {
        return;
    }
    let resolver: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_RESOLVE_CONTRACT_OFFERS);
    loop {
        let e = (read_u32(this + contract_manager_fields::DECISION_DATES) as usize).wrapping_add((n(this + contract_manager_fields::DECISION_NEXT_INDEX) as usize).wrapping_mul(14));
        if call_date_compare(0x456740, e + 4, data) != 0 {
            return; // has not won yet
        }
        if call_date_compare(0x4a7f20, e + 4, data) != 0 && read_u8(e + 0xc) as i8 > slice as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let person = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            core::ptr::write_unaligned(e as *mut i32, -1);
            if read_u8(e + 0xd) == 3 {
                crate::comparison::branch("cpp_1 decidir ofertas");
                resolver(this, person); // the person decides the offers
            }
        }
        let i = n(this + contract_manager_fields::DECISION_NEXT_INDEX) + 1;
        core::ptr::write_unaligned((this + contract_manager_fields::DECISION_NEXT_INDEX) as *mut i32, i);
        if i >= n(this + contract_manager_fields::DECISION_COUNT) {
            return;
        }
    }
}


/// `CONTRACT_MANAGER::get_current_contract_offers(club, out)`: copies the club's open contract offers into a new array
/// of 111 bytes per offer and returns how many.
///
/// It walks the offers list (+0x34; cursor reset, 0x68d1b0) and copies each offer of the club (+4 = id) outside states
/// 5 and 6 (+0x35 & 0x3f) into a new vector of 111 bytes per offer, which grows by 20 at a time (realloc 0x945667 with
/// the capacity as a signed byte). Stores the vector in `out` and returns how many (byte). After the realloc the
/// original checks the list (+0x34), not the vector: only with a null list does it go to the "cannot allocate memory"
/// box (line 0x64c) and return 0 with a null output. The caller (0x598a06, the transfer budget) frees the vector.
/// Original: `CONTRACT_MANAGER::get_current_contract_offers` (0x4ca300, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4ca300 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_current_contract_offers(clube, saida)`:
/// percorre a lista das ofertas (+0x34; cursor reposto, 0x68d1b0) e copia cada oferta do clube (+4 = id) fora dos
/// estados 5 e 6 (+0x35 & 0x3f) para um vetor novo de 111 bytes por oferta, que cresce de 20 em 20 (realloc 0x945667
/// com a capacidade em byte com sinal). Grava o vetor em `saida` e devolve quantas (byte). A original confere a lista
/// (+0x34), não o vetor, depois do realloc: só com a lista nula vai à caixa "cannot allocate memory" (linha 0x64c) e
/// devolve 0 com a saída nula. Quem a chama (0x598a06, o orçamento de transferências) libera o vetor.
pub(crate) unsafe extern "thiscall" fn get_current_contract_offers(this: usize, clube: usize, out: usize) -> u8 {
    let mut n: i8 = 0;
    let mut cap: i8 = 0;
    let mut array: usize = 0;
    let list = read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize;
    if list != 0 {
        core::ptr::write_unaligned((list + 0xa) as *mut u32, read_u32(list + 2));
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let mut item = next(list);
        while item != 0 {
            let state = read_u8(item + 0x35) & 0x3f;
            if read_u32(item + 4) == read_u32(clube) && state != 5 && state != 6 {
                if cap == n {
                    cap = cap.wrapping_add(20);
                    array = realloc(array, (cap as i32 * 111) as u32);
                    if read_u32(this + contract_manager_fields::CONTRACT_OFFERS) == 0 {
                        show_cannot_create_space_box(0x64c, 0x9a8220); // "cannot allocate memory"
                        core::ptr::write_unaligned(out as *mut u32, 0);
                        return 0;
                    }
                }
                crate::comparison::branch("cpp_1 oferta do clube");
                core::ptr::copy(item as *const u8, array.wrapping_add((n as i32 * 111) as usize) as *mut u8, 111);
                n = n.wrapping_add(1);
            }
            item = next(read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize);
        }
    }
    core::ptr::write_unaligned(out as *mut u32, array as u32);
    n as u8
}



/// Volatile byte read (so a missing main contract crashes like the original).
pub(crate) unsafe fn read_byte_volatile(a: usize) -> u8 {
    // simplification: volatile read because the original reads [0 + 0x35] when the main contract is missing (crashes like it)
    core::ptr::read_volatile(a as *const u8)
}


/// Address of the person's 79-byte staff state record (+0 main contract index, +4 loan contract index; -1 none).
///
/// [pt] O registro de 79 bytes da pessoa em [0xae2d0c] (+0 o contrato principal, +4 o de empréstimo; −1 sem).
pub(crate) unsafe fn staff_state_entry(person: usize) -> usize {
    (read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person) as usize).wrapping_mul(0x4f))
}

/// Entry k (0x50 bytes) of the contract array starting at vetor.
///
/// [pt] A entrada `k` (0x50 bytes) do vetor de contratos que começa em `vetor`.
pub(crate) fn contract_at_index(array: u32, k: i32) -> usize {
    (array as usize).wrapping_add((k as usize).wrapping_mul(0x50))
}

/// Club record (0x245 bytes) of id in G_CLUB_TABLE.
///
/// [pt] O clube de id `id` (registro de 0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn contract_manager_club_by_id(id: u32) -> usize {
    (read_u32(G_CLUB_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(club::SIZE))
}

/// Main contract of the person in the global CONTRACT_MANAGER (0 if none/out of range), also stored in the lookup
/// global 0xae1080.
///
/// [pt] O contrato principal da pessoa no CONTRACT_MANAGER global ([[0xae1050]]), como as originais o refazem: pessoa fora
/// da faixa (id ≥ [0xae2364]) ou sem contrato principal: 0. Grava em [0xae1080] (a ficha global).
pub(crate) unsafe fn main_contract_lookup(person: usize) -> usize {
    let c = if read_u32(person) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32(staff_state_entry(person)) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    write_u32(G_CONTRACT_LOOKUP_RESULT, c as u32);
    c
}

/// Shows the contract_manager.cpp "Error" message box for a source line, as the original; the game continues.
///
/// [pt] A caixa "Error" do contract_manager.cpp na `linha`: "v%s %s.%s %d" em 0xae24d0 (0x944cff separa o caminho 0x9a8190,
/// 0x90d130 a versão; a original não desempilha os argumentos dela, que são o resto do formato), a caixa 0x5e8290 e
/// [0xb67a34] = 0. O jogo segue.
pub(crate) unsafe fn show_contract_error_box(row: u32) {
    let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    split(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), row);
    caixa(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

// The database routines all three call (by address; the ported ones go through the hook).
/// Calls the club's linked A/B team lookup (0x540a50); sets whether the club is the A team.
pub(crate) unsafe fn contract_manager_linked_b_team(clube: usize, eh_a: *mut u32) -> usize {
    let f: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    f(clube, eh_a, 1)
}
/// Calls the exe's Staff set-squad-flags routine for the person.
pub(crate) unsafe fn set_squad_flags(person: usize, squad: u32) {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    f(person, squad);
}
/// Calls Tactics remove_player_from_selection on G_TACTICS for the club and person.
pub(crate) unsafe fn remove_from_tactics_selection(clube: usize, person: usize, a: u32, b: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    f(read_u32(G_TACTICS) as usize, clube, person, a, b);
}
/// Calls PlayerStats archive_player_stats on G_PLAYER_STATS for the person and club.
pub(crate) unsafe fn contract_manager_archive_player_stats(person: usize, a4: u32, clube: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(FN_PLAYERSTATS_ARCHIVE_PLAYER_STATS);
    f(read_u32(G_PLAYER_STATS) as usize, person, -1, 0, a4, clube, 0);
}
/// Calls Database remove_staff_from_club (club, person, date, flags).
pub(crate) unsafe fn contract_manager_remove_staff_from_club(clube: usize, person: usize, data: usize, desligar: u32, como_jogador: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    f(clube, person, data, desligar, como_jogador);
}
/// Calls Database add_staff_to_club (club, person, job, date, flags).
pub(crate) unsafe fn add_staff_to_club(clube: usize, person: usize, job: u32, data: usize, reduzir: u32, write: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
    f(clube, person, job, data, reduzir, write);
}
/// Calls Database club_has_reserve_squad for the club.
pub(crate) unsafe fn club_has_reserve_squad(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    f(clube)
}
/// Recomputes the club's wage bill (Finance::update_wage_bill on its entry in G_FINANCE_TABLE).
///
/// [pt] Refaz a folha de salários (0x598370) das finanças do clube (0x5a0590 em [0xae2a38]; a original chama mesmo sem).
pub(crate) unsafe fn update_club_wage_bill(clube: usize) {
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    folha(finances(read_u32(G_FINANCE_TABLE) as usize, clube));
}
/// Assigns date origem to destino with the date operator (0x417c10).
pub(crate) unsafe fn contract_manager_assign_date(target: usize, source: usize) {
    let copy: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut ret = [0u8; 8];
    copy(target, &mut ret, source);
}

/// `CONTRACT_MANAGER::sign_loan_contract(contract)` (SEH frame with only a local date, no destructor): moves the person
/// to the loan club, marks the reserve bit in the main contract and stores the loan contract in her slot; recalculates
/// the wage bills.
///
/// The person (+0) of the new loan contract and her club by the main contract (the record [0xae1080]); without it,
/// nothing. Keeps the person's date +0x3e, takes her off the lists (0x7c5fe0 with the new club +4), checks the main
/// contract in the array itself (without it, the box of line 0x16a9 and nothing more) and sets in it bit 0x40 of +0x35
/// (in the reserves): with a reserve team (0x542740), by the squad mark +0x5e & 1; with a partner club (0x540a50), if
/// the current club (+0x39) is not the owner; otherwise clears it. Notifies the new club (0x90d860), takes the person
/// out of the current club's tactics (0x896c60 with 0 and 1), puts her into the new club as a player (0x53bd10, job 0xb
/// with today's date) and restores the date +0x3e.
///
/// The contract goes into the person's loan slot (+4 of the 79-byte record): the one she already has; otherwise the
/// first free one (−1) of the array, which takes the index (and then the original leaves without redoing the wage
/// bills); otherwise at the end (full: +100 slots, realloc 0x945667 or malloc 0x944e46; without memory, the box of line
/// 0x1708 and it goes on), through STAFF_CONTRACT::operator= 0x4cc810. Finally it redoes the wage bills of the owning
/// club and the new one.
/// Original: `CONTRACT_MANAGER::sign_loan_contract` (0x4d01e0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d01e0 } (thiscall, ret 4, contract_manager.cpp, quadro SEH só com uma data local, sem destrutor):
/// `CONTRACT_MANAGER::sign_loan_contract(contrato)`: a pessoa (+0) do contrato de empréstimo novo e o clube dela pelo
/// contrato principal (a ficha [0xae1080]); sem ele, nada. Guarda a data +0x3e da pessoa, tira-a das listas (0x7c5fe0
/// com o clube novo +4), confere o contrato principal no próprio vetor (sem ele, caixa da linha 0x16a9 e nada mais) e
/// marca nele o bit 0x40 de +0x35 (no reserva): com time reserva (0x542740), pela marca de elenco +0x5e & 1; com clube
/// parceiro (0x540a50), se o clube atual (+0x39) não é o dono; senão apaga. Avisa o clube novo (0x90d860), tira a pessoa
/// das táticas do clube atual (0x896c60 com 0 e 1), põe no clube novo como jogador (0x53bd10, cargo 0xb com a data de
/// hoje) e devolve a data +0x3e. O contrato vai para a vaga de empréstimo da pessoa (+4 do registro de 79 bytes): na
/// que ela já tem; senão na primeira livre (−1) do vetor, que fica com o índice (e aí a original sai sem refazer as
/// folhas); senão no fim (cheio: +100 vagas, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa da linha 0x1708
/// e segue), pelo STAFF_CONTRACT::operator= 0x4cc810. Por fim refaz as folhas de salários do clube dono e do novo.
pub(crate) unsafe extern "thiscall" fn sign_loan_contract(this: usize, contrato: usize) {
    let mut guardada = [0u8; 8];
    let construct: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    construct(&mut guardada);
    let person = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contrato) as usize).wrapping_mul(staff::SIZE));
    let sheet = if person == 0 { write_u32(G_CONTRACT_LOOKUP_RESULT, 0); 0 } else { main_contract_lookup(person) };
    if sheet == 0 {
        return;
    }
    let owner = contract_manager_club_by_id(read_u32(sheet + 4));
    if owner == 0 {
        return;
    }
    contract_manager_assign_date(guardada.as_mut_ptr() as usize, person + staff::DATE_JOINED_CLUB);
    let new = contract_manager_club_by_id(read_u32(contrato + 4));
    contract_manager_archive_player_stats(person, 1, read_u32(new) as i32);
    let principal = if person != 0 && (read_u32(person) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(person)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(person)) as i32)
    } else {
        0
    };
    if principal == 0 {
        show_contract_error_box(0x16a9);
        return;
    }
    crate::comparison::branch("cpp_2 assinar emprestimo");
    let m = principal + contract::CONTRACT_TYPE;
    let reserve = if club_has_reserve_squad(owner) != 0 {
        read_byte_volatile(person + staff::PLAYING_SQUAD) & 1 == 0
    } else if contract_manager_linked_b_team(owner, core::ptr::null_mut()) != 0 {
        owner as u32 != read_u32(person + staff::CLUB_JOB)
    } else {
        false
    };
    let cl = read_byte_volatile(m);
    *(m as *mut u8) = if reserve { cl | 0x40 } else { cl & 0xbf };
    let warn: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_REMOVE_VIRTUAL_PLAYERS);
    warn(read_u32(G_VIRTUAL_STAFF) as usize, new, 0);
    remove_from_tactics_selection(read_u32(person + staff::CLUB_JOB) as usize, person, 0, 1);
    remove_from_tactics_selection(read_u32(person + staff::CLUB_JOB) as usize, person, 1, 1);
    add_staff_to_club(new, person, 0xb, G_CURRENT_DATE, 1, 0); // the player at the new club
    contract_manager_assign_date(person + staff::DATE_JOINED_CLUB, guardada.as_ptr() as usize);
    let emp = read_u32(staff_state_entry(person) + 4) as i32;
    if emp > -1 {
        // simplification: the original copies inline, same as STAFF_CONTRACT::operator= (0x4cc810, ported)
        contract_manager_operator(contract_at_index(read_u32(this), emp), contrato);
    } else {
        let n = read_u32(this + 8) as i32;
        let array = read_u32(this);
        let mut i = 0i32;
        while i < n {
            if read_u32(contract_at_index(array, i)) as i32 == -1 {
                contract_manager_operator(contract_at_index(array, i), contrato);
                write_u32(staff_state_entry(person) + 4, i as u32);
                return; // the original exits without rebuilding the sheets
            }
            i += 1;
        }
        let cap = read_u32(this + 4) as i32;
        if n >= cap {
            let cap = cap + 100;
            write_u32(this + 4, cap as u32);
            let new_array = if array != 0 {
                let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
                realloc(array as usize, (cap * 0x50) as u32)
            } else {
                let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
                alloc((cap * 0x50) as u32)
            };
            write_u32(this, new_array as u32);
            if new_array == 0 {
                show_contract_error_box(0x1708);
            }
        }
        let n = read_u32(this + 8);
        write_u32(staff_state_entry(person) + 4, n);
        let target = contract_at_index(read_u32(this), n as i32);
        write_u32(this + 8, n + 1);
        let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_OPERATOR_3D);
        copy(target, contrato);
    }
    update_club_wage_bill(owner);
    update_club_wage_bill(new);
}


/// `CONTRACT_MANAGER::end_loan(person)`: takes the person out of the loan club, frees the loan contract, recalculates
/// the wage bills and returns her to the parent club with the squad mark.
///
/// The loan club (+0x39; null: the box of line 0x1729), or its A team when it is the B team (0x540a50). With the main
/// contract in the array itself (without it: the box of line 0x1735): the record [0xae1080] gives the owning club;
/// takes the person off the lists (0x7c5fe0 with the owner), the disputes (0x556570), the loan club's tactics (0x896c60
/// with 0 and 1); frees the loan contract (−1 in the entry and in slot +4); removes her from the loan club's squad
/// (0x53c190); redoes the wage bills of the loan club and the owner; removes her from the loan club's shortlist
/// (0x849ce0, 0x843e10 with 4). The person goes back to the owner (+0x39) and to the squad marks (0x540980): with a
/// reserve team (0x542740), 2 if the main contract has bit 0x40 of +0x35, otherwise 1; otherwise 1 and, with a partner
/// club, she stays at the partner if she is in its 50 slots. Status +0x6d 0x10 becomes 4.
/// Original: `CONTRACT_MANAGER::end_loan` (0x4d07e0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d07e0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::end_loan(pessoa)`: o clube do empréstimo
/// (+0x39; nulo: caixa da linha 0x1729), ou o time A dele quando é o B (0x540a50). Com o contrato principal no próprio
/// vetor (sem ele: caixa da linha 0x1735): a ficha [0xae1080] dá o clube dono; tira a pessoa das listas (0x7c5fe0 com o
/// dono), das disputas (0x556570), das táticas do clube do empréstimo (0x896c60 com 0 e 1); libera o contrato de
/// empréstimo (−1 na entrada e na vaga +4); tira do elenco do clube do empréstimo (0x53c190); refaz as folhas do clube
/// do empréstimo e do dono; tira da lista de observação do clube do empréstimo (0x849ce0, 0x843e10 com 4). A pessoa
/// volta ao dono (+0x39) e às marcas de elenco (0x540980): com time reserva (0x542740), 2 se o contrato principal tem o
/// bit 0x40 de +0x35, senão 1; senão 1 e, com clube parceiro, fica no parceiro se está nas 50 vagas dele. A situação
/// +0x6d 0x10 vira 4.
pub(crate) unsafe extern "thiscall" fn end_loan(this: usize, person: usize) {
    let mut clube = read_u32(person + staff::CLUB_JOB) as usize;
    if clube == 0 {
        show_contract_error_box(0x1729);
        return;
    }
    let mut eh_a = 0u32; // read only when 0x540a50 writes it
    if contract_manager_linked_b_team(clube, &mut eh_a) != 0 && eh_a == 0 {
        clube = contract_manager_linked_b_team(clube, core::ptr::null_mut()); // o time A
    }
    let principal = if person != 0 && (read_u32(person) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(person)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(person)) as i32)
    } else {
        0
    };
    if principal == 0 {
        show_contract_error_box(0x1735);
        return;
    }
    crate::comparison::branch("cpp_2 encerrar emprestimo");
    let sheet = contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), read_u32(staff_state_entry(person)) as i32);
    write_u32(G_CONTRACT_LOOKUP_RESULT, sheet as u32);
    let owner = if sheet == 0 { 0 } else { contract_manager_club_by_id(read_u32(sheet + 4)) };
    contract_manager_archive_player_stats(person, 0, read_u32(owner) as i32);
    let disputas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_RESTORE_HELD_BANS);
    disputas(read_u32(G_DISCIPLINE) as usize, person);
    remove_from_tactics_selection(read_u32(person + staff::CLUB_JOB) as usize, person, 0, 1);
    remove_from_tactics_selection(read_u32(person + staff::CLUB_JOB) as usize, person, 1, 1);
    let emp = read_u32(staff_state_entry(person) + 4) as i32;
    if emp != -1 {
        write_u32(contract_at_index(read_u32(this), emp), u32::MAX);
        write_u32(staff_state_entry(person) + 4, u32::MAX);
    }
    contract_manager_remove_staff_from_club(read_u32(person + staff::CLUB_JOB) as usize, person, 0, 1, 0);
    update_club_wage_bill(clube);
    let f = main_contract_lookup(person);
    update_club_wage_bill(if f == 0 { 0 } else { contract_manager_club_by_id(read_u32(f + 4)) });
    let list = read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize;
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    if entry(list, clube, person, 0, 0) != 0 {
        let remove: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        remove(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize, person, clube, 4, 0);
    }
    let f = main_contract_lookup(person);
    let volta = if f == 0 { 0 } else { contract_manager_club_by_id(read_u32(f + 4)) };
    write_u32(person + staff::CLUB_JOB, volta as u32); // a pessoa volta ao dono
    if club_has_reserve_squad(volta) != 0 {
        set_squad_flags(person, if read_byte_volatile(principal + contract::CONTRACT_TYPE) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let partner = contract_manager_linked_b_team(read_u32(person + staff::CLUB_JOB) as usize, core::ptr::null_mut());
        set_squad_flags(person, 1);
        if partner != 0 {
            for k in 0..0x32 {
                if read_u32(partner + club::SQUAD + 4 * k) as usize == person {
                    write_u32(person + staff::CLUB_JOB, partner as u32);
                }
            }
        }
    }
    if read_byte_volatile(person + staff::SQUAD_SELECTED_FOR) == 0x10 {
        *((person + staff::SQUAD_SELECTED_FOR) as *mut u8) = 4;
    }
}


/// `CONTRACT_MANAGER::toggle_squad_placement(person, club, out)`: moves the person between the club's first team and
/// reserves or between the club and its partner (A/B side), taking the loan contract into account; 0 with no room at
/// the destination.
///
/// By the place (0x4cbb60): 0 nothing. 1 and 2 (first team / reserves at the club): on loan to another club (loan
/// contract +4 with another club), it only clears (1) or sets (2) bit 0x40 of +0x35 of the main contract; otherwise
/// squad mark 1 or 2 (0x540980) and the club's tactics (0x896c60 with 1 or 0). 3 (B team) and the others (A team): the
/// destination club is the club (3) or the partner (0x540a50), stored in `out`; with no room for the job (+0x3d,
/// 0x53b9a0) it returns 0. On loan to this club: she leaves the other's lists, squad and tactics and enters the
/// destination with the job; with another club's loan contract: toggles bit 0x40 and moves to the destination as a
/// player (0xb); otherwise as the loaned one, without storing. Returns 1 (also for a null club). Without a main
/// contract the original reads [0x35] and crashes.
/// Original: `CONTRACT_MANAGER::toggle_squad_placement` (0x4cbd20, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4cbd20 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::toggle_squad_placement(pessoa, clube,
/// saida)`: pelo lugar (0x4cbb60): 0 nada. 1 e 2 (principal / reserva no clube): emprestada a outro clube (contrato de
/// empréstimo +4 com outro clube), só apaga (1) ou marca (2) o bit 0x40 de +0x35 do contrato principal; senão a marca de
/// elenco 1 ou 2 (0x540980) e as táticas do clube (0x896c60 com 1 ou 0). 3 (time B) e os outros (time A): o clube de
/// destino é o clube (3) ou o parceiro (0x540a50), gravado em `saida`; sem vaga para o cargo (+0x3d, 0x53b9a0) devolve
/// 0. Emprestada a este clube: sai das listas, do elenco e das táticas do outro e entra no destino com o cargo; com
/// contrato de empréstimo de outro clube: troca o bit 0x40 e passa ao destino como jogador (0xb); senão como o
/// emprestado, sem gravar. Devolve 1 (clube nulo também). Sem contrato principal a original lê [0x35] e cai.
pub(crate) unsafe extern "thiscall" fn toggle_squad_placement(this: usize, person: usize, clube: usize, out: usize) -> u32 {
    if clube == 0 {
        return 1;
    }
    if out != 0 {
        write_u32(out, 0);
    }
    let place: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let l = place(this, person, clube);
    if l == 0 {
        return 1;
    }
    let na_faixa = |p: usize| p != 0 && (read_u32(p) as i32) < read_u32(G_STAFF_COUNT) as i32;
    // the loan contract (in the global CONTRACT_MANAGER) and the main one (in the array itself; 0 if none)
    let loan = |p: usize| {
        let k = read_u32(staff_state_entry(p) + 4) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    let principal = |p: usize| {
        let k = read_u32(staff_state_entry(p)) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(this), k) }
    };
    if l == 1 || l == 2 {
        if na_faixa(person) {
            let e = loan(person);
            if e != 0 && read_u32(e + 4) != read_u32(clube) {
                let m = principal(person) + 0x35;
                let cl = read_byte_volatile(m);
                *(m as *mut u8) = if l == 1 { cl & 0xbf } else { cl | 0x40 };
                return 1;
            }
        }
        crate::comparison::branch("cpp_2 trocar no clube");
        set_squad_flags(person, l as u32); // 1 principal, 2 reserva
        remove_from_tactics_selection(clube, person, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (contract_manager_linked_b_team(clube, core::ptr::null_mut()), clube)
    } else {
        (clube, contract_manager_linked_b_team(clube, core::ptr::null_mut()))
    };
    if out != 0 {
        write_u32(out, destino as u32);
    }
    let slot: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    // simplification: the original pushes the job in a register with garbage in the high bytes; 0x53b9a0 reads only the byte
    if slot(destino, read_byte_volatile(person + staff::JOB_FOR_CLUB) as u32, person) == 0 {
        return 0;
    }
    if na_faixa(person) {
        let e = loan(person);
        if e != 0 && read_u32(e + 4) == read_u32(clube) {
            // loaned to this club: passes from the other one to the destination with the role
            contract_manager_archive_player_stats(person, 0, -1);
            contract_manager_remove_staff_from_club(origem, person, 0, 1, 1);
            remove_from_tactics_selection(origem, person, 1, 1);
            remove_from_tactics_selection(origem, person, 0, 1);
            // simplification: the job goes with the high bytes of eax from 0x896c60; 0x53bd10 reads only the byte
            add_staff_to_club(destino, person, read_byte_volatile(person + staff::JOB_FOR_CLUB) as u32, 0, 1, 1);
            return 1;
        }
        if na_faixa(person) && e != 0 {
            let m = principal(person) + 0x35;
            let cl = read_byte_volatile(m);
            *(m as *mut u8) = if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf };
            crate::comparison::branch("cpp_2 trocar emprestado");
            contract_manager_remove_staff_from_club(origem, person, 0, 0, 1);
            add_staff_to_club(destino, person, 0xb, 0, 0, 1);
            return 1;
        }
    }
    crate::comparison::branch("cpp_2 trocar de time");
    contract_manager_archive_player_stats(person, 0, -1);
    contract_manager_remove_staff_from_club(origem, person, 0, 1, 0);
    remove_from_tactics_selection(origem, person, 1, 1);
    remove_from_tactics_selection(origem, person, 0, 1);
    // simplification: the role goes with the high bytes of edx left by 0x896c60; 0x53bd10 reads only the byte
    add_staff_to_club(destino, person, read_byte_volatile(person + staff::JOB_FOR_CLUB) as u32, 0, 1, 0);
    1
}




/// Volatile byte read (so a missing contract club crashes like the original).
pub(crate) unsafe fn read_u8_volatile(a: usize) -> u8 {
    // simplification: volatile because the original reads [0 + 0x53] when the contract’s club is missing (crashes like it)
    core::ptr::read_volatile(a as *const u8)
}

/// Club reputation as seen by a manager: weighted by the nation's league standard (or × 0.65 without nation), truncated
/// and capped above 0x203a.
///
/// [pt] A reputação "vista" de um clube para o técnico: com nação (+0x53), o peso dela (+0x85) × reputação (+0x80) × 0,05
/// (0x967888); sem, a reputação × 0,65 (0x96a5d0); truncadas (16 bits) e, acima de 0x203a, 0x203a − (excesso × −1,1
/// truncado).
pub(crate) unsafe fn perceived_club_reputation(clube: usize) -> i16 {
    let r = read_i16(clube + club::REPUTATION) as i32;
    let nacao = read_u32(clube + club::NATION) as usize;
    let v = if nacao != 0 {
        match_pl_ftol((read_u8_volatile(nacao + nation::LEAGUE_STANDARD) as i8 as i32 * r) as f64 * read_f64_at(0x967888)) // 0x967888 = 0.05
    } else {
        match_pl_ftol(r as f64 * read_f64_at(0x96a5d0)) // 0x96a5d0 = 0.65
    } as i16;
    if v > 0x203a {
        (0x203a - match_pl_ftol((v as i32 - 0x203a) as f64 * read_f64_at(0x96d278))) as i16 // 0x96d278 = -1.1
    } else {
        v
    }
}



/// `staff_can_take_job(job, person)` (stdcall, ret 8): tells whether the person fits the job (6 to 15) by age, current
/// job and the player and non-player records; a null person: 0.
///
/// Through the jump table 0x4cbaf4 (job − 6, 0..9; outside: 0); 7, 12 and 14: 0; a null person: 0 (the non-player regen
/// 0x7ab9f2 always asks with null). 6: aged 30 (+0x18) or more, a non-player (+0x69), above 36, or with +0x57 below 10
/// and the player's +0x2f 12 or more: 1; otherwise, a non-player in job 8, 6 or 5. 8: aged 30, a non-player or above 36
/// or (+0x2f ≥ 12 and +0x57 < 10) outside jobs 5 and 10: 1; otherwise as for 6. 9: a non-player in job 9. 10: a
/// non-player in job 10 or with +0x28 = 10. 11: a player (+0x61). 13 and 15: both player and non-player, or above 34,
/// or aged 30 with +0x2f ≥ 12.
/// Original: `staff_can_take_job` (0x4cb990, contract_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x4cb990 } (stdcall, ret 8, contract_manager.cpp): `staff_can_take_job(cargo, pessoa)`: pela tabela de salto
/// 0x4cbaf4 (cargo − 6, 0..9; fora: 0); 7, 12 e 14: 0; pessoa nula: 0 (o regen de não jogador 0x7ab9f2 pergunta sempre
/// com nula). 6: com 30 anos (+0x18) ou mais, não jogador (+0x69), acima de 36, ou com +0x57 abaixo de 10 e +0x2f do
/// jogador 12 ou mais: 1; senão, não jogador no cargo 8, 6 ou 5. 8: de 30 anos, não jogador ou acima de 36 ou (+0x2f ≥ 12
/// e +0x57 < 10) fora dos cargos 5 e 10: 1; senão como o 6. 9: não jogador no cargo 9. 10: não jogador no cargo 10 ou com
/// +0x28 = 10. 11: jogador (+0x61). 13 e 15: jogador e não jogador, ou acima de 34, ou de 30 anos com +0x2f ≥ 12.
pub(crate) unsafe extern "stdcall" fn staff_can_take_job(job: u32, person: usize) -> u32 {
    let k = (job as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if person == 0 {
        crate::comparison::branch("cpp_4 pessoa nula");
        return 0; // pessoa nula
    }
    let p = person;
    let age = read_u8_volatile(p + staff::YEAR_OF_BIRTH) as i8;
    let not_player = read_u32(p + staff::NON_PLAYER);
    let funcao = read_u8_volatile(p + staff::JOB_FOR_CLUB);
    let strength = || read_u8_volatile(read_u32(p + staff::PLAYER) as usize + 0x2f) as i8; // +0x2f of the player
    // the common end (0x4cb9db): non-player in jobs 8, 6 or 5
    let common = || not_player != 0 && (funcao == 8 || funcao == 6 || funcao == 5);
    let r = match k {
        0 => {
            if age >= 0x1e {
                if not_player != 0 || age > 0x24 {
                    return 1;
                }
                if (read_u8_volatile(p + staff::AMBITION) as i8) < 0xa && strength() >= 0xc {
                    return 1;
                }
            }
            common()
        }
        2 => {
            if age < 0x1e {
                common()
            } else if not_player == 0 && age <= 0x24 && (strength() < 0xc || read_u8_volatile(p + staff::AMBITION) as i8 >= 0xa) {
                common()
            } else if funcao == 5 || funcao == 0xa {
                common()
            } else {
                true
            }
        }
        3 => not_player != 0 && funcao == 9,
        4 => not_player != 0 && (funcao == 0xa || read_u8_volatile(p + staff::JOB_FOR_NATION) == 0xa),
        5 => read_u32(p + staff::PLAYER) != 0,
        _ => {
            // 13 e 15
            let player = read_u32(p + staff::PLAYER) as usize;
            player != 0 && (not_player != 0 || age > 0x22 || (age >= 0x1e && read_u8_volatile(player + player_sheet::LEADERSHIP) as i8 >= 0xc))
        }
    };
    r as u32
}


/// Days (0..400) from the date (day, unsigned year) to today (G_CURRENT_DATE), 365-day years.
///
/// [pt] Os dias (0..400) desde a data (dia +0, ano +2 sem sinal) até hoje ([0xae2c90]), com o ano de 365 dias.
pub(crate) unsafe fn contract_manager_days_since_date(day: usize, year: usize) -> i32 {
    let today = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16) as i32;
    let a = core::ptr::read_unaligned(year as *const u16) as i32;
    let d = (today - a) * 365 - read_i16(day) as i32 + read_i16(G_CURRENT_DATE) as i32;
    d.clamp(0, 400)
}

/// Morale drop byte after a bad match (from rating, club reputation, days at club, age, contract band), or None when
/// the routine leaves things untouched.
///
/// [pt] A queda da moral (o byte `m` da original) pela partida ruim, ou None quando a rotina sai sem mexer (algum nulo, o
/// técnico ou o auxiliar do clube +0xcf/+0xd3, ou fora do cargo 0xb). x = (nota +0xb − reputação do clube) + 50 × dias
/// no clube (desde +0x3e, até 400) + 417 × idade − 31680; com os dias do contrato (desde +0x2d) acima de 400 (nunca: o
/// teto é 400) e idade 24..31: + 250 × +0x57 + 5000; senão, com +0x22 e a reputação da nação (+0x1a +0x8e) acima de 3500
/// e menos de 32 anos: + 250 × +0x57 + 25 × +0x22 + 2500; senão pela faixa do contrato (+0x4f >> 4): 1: + 250 × +0x57 +
/// 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 × +0x3e; nesses casos no máximo 15000. y = 1 − x × (6,667e-4 se x < 0, senão
/// 6e-5), entre 0,1 e 10; m = ftol((+0x3e + 4) / y + 0,5) (byte). Dupla precisão; as constantes do .rdata.
pub(crate) unsafe fn match_morale_drop(person: usize, clube: usize, contrato: usize) -> Option<i8> {
    if contrato == 0 || clube == 0 || person == 0 {
        return None;
    }
    if person == read_u32(clube + club::MANAGER) as usize || person == read_u32(clube + club::ASSISTANT_MANAGER) as usize || read_u8_volatile(person + staff::JOB_FOR_CLUB) != 0xb {
        return None;
    }
    let a = contract_manager_days_since_date(person + staff::DATE_JOINED_CLUB, person + 0x40);
    let b = contract_manager_days_since_date(contrato + contract::CONTRACT_EXPIRES, contrato + 0x2f);
    let age = read_u8_volatile(person + staff::YEAR_OF_BIRTH) as i8;
    let player = read_u32(person + staff::PLAYER) as usize;
    let dif = read_i16(player + player_sheet::CURRENT_REPUTATION) as i32 - read_i16(clube + club::REPUTATION) as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + age as f64 * read_f64_at(0x968d90)) - read_f64_at(0x968db8); // 0x968d90 = 417, 0x968db8 = 31680
    let c57 = read_u8_volatile(person + staff::AMBITION) as i8 as i32;
    let ceiling = read_f32_at(0x968da8) as f64; // 0x968da8 = 15000
    let limitar = |x: f64| if x >= ceiling { ceiling } else { x };
    let al = read_u8_volatile(person + staff::INT_APPS);
    if b > 0x190 && age > 0x17 && age < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + read_f32_at(0x968db0) as f64); // never: b goes up to 400; 0x968db0 = 5000
    } else if al != 0 && read_i16(read_u32(person + staff::NATION) as usize + 0x8e) > 0xdac && age < 0x20 {
        x = limitar(x + (c57 * 250 + al as i32 * 25) as f64 + read_f32_at(0x968dac) as f64); // 0x968dac = 2500
    } else {
        match (read_u8_volatile(contrato + contract::SQUAD_STATUS) >> 4) as i8 {
            1 => x = limitar(x + (c57 * 250) as f64 + read_f32_at(0x968db0) as f64), // 0x968db0 = 5000
            2 => x = limitar(x + (c57 * 125) as f64 + read_f32_at(0x968dac) as f64), // 0x968dac = 2500
            3 => x = limitar(x + (read_u8_volatile(contrato + contract::UNKNOWN18_2) as i8 as i32 * 1000) as f64),
            _ => {}
        }
    }
    let y = if !(x >= read_f64_at(0x9689a0)) { x * read_f64_at(0x968da0) } else { x * read_f64_at(0x968d98) }; // 0x9689a0 = 0, 0x968da0 = 0.0006666666667, 0x968d98 = 6e-05
    let mut y = read_f64_at(0x967890) - y; // 0x967890 = 1
    if !(y >= read_f64_at(0x967880)) { // 0x967880 = 0.1
        y = read_f64_at(0x967880); // 0x967880 = 0.1
    } else if y > read_f64_at(0x9688b0) { // 0x9688b0 = 10
        y = read_f64_at(0x9688b0); // 0x9688b0 = 10
    }
    let d = (read_u8_volatile(contrato + contract::UNKNOWN18_2) as i8 as i32 + 4) as f64;
    Some(match_pl_ftol(d / y + read_f64_at(0x968968)) as u8 as i8) // 0x968968 = 0.5
}

/// Morale minus m, clamped to -100..100 (byte sum when it fits).
///
/// [pt] A moral (+0x44) menos `m`, entre −100 e 100 (a soma em byte quando cabe).
pub(crate) fn lowered_morale(morale: i8, m: i8) -> i8 {
    let minus = m.wrapping_neg();
    let s = morale as i32 + minus as i32;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        morale.wrapping_add(minus)
    }
}

/// Club of the person's main contract (null if none); stores the contract in G_CONTRACT_LOOKUP_RESULT like the
/// original.
///
/// [pt] O contrato principal da pessoa no CONTRACT_MANAGER global e o clube dele, como a original os refaz (a ficha
/// [0xae1080] gravada); sem contrato o clube é nulo.
pub(crate) unsafe fn club_of_main_contract(person: usize) -> usize {
    let c = if read_u32(person) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person) as usize).wrapping_mul(0x4f))) as i32;
        if k < 0 { 0 } else { (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) }
    };
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut u32, c as u32);
    if c == 0 { 0 } else { (read_u32(G_CLUB_TABLE) as usize).wrapping_add((read_u32(c + 4) as usize).wrapping_mul(club::SIZE)) }
}

/// True when today is inside the transfer window (two dates in table 0xb63d8a by key nation index) for the club of the
/// person's contract.
///
/// [pt] A janela de transferências aberta para o clube do contrato da pessoa: o índice da nação-chave (0x669340 da nação do
/// clube; sem nação, o byte baixo do ponteiro do clube que a original lê da vaga do argumento) dá as duas datas da
/// tabela 0xb63d8a (72 bytes por nação): hoje depois da primeira e antes da segunda.
pub(crate) unsafe fn transfer_window_open(person: usize, clube: usize) -> bool {
    let c1 = club_of_main_contract(person);
    let k = if read_u32(c1 + club::NATION) != 0 {
        let c2 = club_of_main_contract(person);
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
        indice(read_u32(c2 + club::NATION) as usize) as u8 as i8
    } else {
        // simplification: the original reads the low byte of the slot of the `clube` argument (the pointer), not an index
        clube as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = G_LEAGUE_WORLD_CUP_START_DATE + (k as usize) * 0x48;
    let day = read_i16(G_CURRENT_DATE);
    let year = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    let year_of = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let after = (day > read_i16(t) && year == year_of(t + 2)) || year > year_of(t + 2);
    let before = (day < read_i16(t + 0xf) && year == year_of(t + 0x11)) || year < year_of(t + 0x11);
    after && before
}

/// `contract_manager_lower_match_morale(person, club, contract)`: lowers the player's contract morale after a poor
/// match (by rating, reputation, time at the club, age and contract band) and, with very low morale, may add the
/// wants-to-leave or lack-of-games grievance; injured, on loan or retiring clears the grievances.
///
/// The drop `m` (match_morale_drop). Injured (0x62c400), on loan (loan contract), retiring (0x7a7eb0), under 18 or with
/// +0x49: clears grievances 4 and 0x10 (0x8674e0). Otherwise, with m > 5 and morale − m below −30 − +0x59: above 21
/// years old, stronger (+0xb) than the club and above 2250, or with potential (+0xd) above reputation − 1500 and above
/// 500 with draw(+0x57) ≠ 0 and +0x3d negative: in the transfer window of the contract club, grievance 4 (0x8674e0 and
/// 0x867700 with 4); in the first team (0x4cbb60 = 1), rating above 1250, more than 23 years old and draw(+0x59 + 5) =
/// 0: grievance 0x10; up to 21 years old, in the first team with draw(+0x57) > 5, stronger than the club and above
/// 1250: grievance 0x10. Finally the happiness in the contract (+0x44) drops by m, kept between −100 and 100.
/// Original: `contract_manager_lower_match_morale` (0x4cb250, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4cb250 } (thiscall, ret 0xc, contract_manager.cpp): `contract_manager_lower_match_morale(pessoa, clube,
/// contrato)`: a queda `m` (match_morale_drop). Machucado (0x62c400), emprestado (contrato de empréstimo), aposentando
/// (0x7a7eb0), menor de 18 ou com +0x49: apaga as queixas 4 e 0x10 (0x8674e0). Senão, com m > 5 e a moral − m abaixo de
/// −30 − +0x59: acima de 21 anos, mais forte (+0xb) que o clube e acima de 2250, ou de potencial (+0xd) acima da
/// reputação − 1500 e de 500 com sorteio(+0x57) ≠ 0 e +0x3d negativo: na janela de transferências do clube do contrato,
/// a queixa 4 (0x8674e0 e 0x867700 com 4); no principal (0x4cbb60 = 1), nota acima de 1250, mais de 23 anos e
/// sorteio(+0x59 + 5) = 0: a queixa 0x10; até 21 anos, no principal com sorteio(+0x57) > 5, mais forte que o clube e
/// acima de 1250: a queixa 0x10. Por fim a felicidade no contrato (+0x44) cai m, entre −100 e 100.
pub(crate) unsafe extern "thiscall" fn contract_manager_lower_match_morale(this: usize, person: usize, clube: usize, contrato: usize) {
    let Some(m) = match_morale_drop(person, clube, contrato) else {
        return;
    };
    let erase: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
    let mark: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let aposentando: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let place: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let emprestado = || {
        if read_u32(person) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
            return false;
        }
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person) as usize).wrapping_mul(0x4f)) + 4) as i32;
        k >= 0 && (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let age = read_u8_volatile(person + staff::YEAR_OF_BIRTH) as i8;
    if lesao(read_u32(G_INJURY) as usize, person) != 0 || emprestado() || aposentando(read_u32(G_PLAYER_REGEN) as usize, person) != 0
        || age < 0x12 || read_u8_volatile(contrato + contract::LEAVING_ON_BOSMAN) != 0 {
        if read_u32(contrato + 0x45) & 4 != 0 {
            erase(contrato, 4);
        }
        if read_u8_volatile(contrato + 0x45) & 0x10 != 0 {
            erase(contrato, 0x10);
        }
    } else if m > 5 && (read_u8_volatile(contrato + 0x44) as i8 as i32 - m as i32) < -30 - read_u8_volatile(person + staff::LOYALITY) as i8 as i32 {
        let player = read_u32(person + staff::PLAYER) as usize;
        if age > 0x15 {
            let rating = read_i16(player + player_sheet::CURRENT_REPUTATION);
            if rating > read_i16(clube + club::REPUTATION) && rating > 0x8ca {
                if transfer_window_open(person, clube) {
                    crate::comparison::branch("cpp_4 queixa na janela");
                    erase(contrato, 4);
                    mark(contrato, 4, 0);
                }
            } else {
                let mut done = false;
                if place(this, person, clube) == 1 && read_i16(player + player_sheet::CURRENT_REPUTATION) > 0x4e2 && age > 0x17
                    && sortear(read_u8_volatile(person + staff::LOYALITY) as i8 as i32 + 5) == 0 {
                    erase(contrato, 4);
                    mark(contrato, 0x10, 0);
                    done = true;
                }
                if !done {
                    let pot = read_i16(player + player_sheet::WORLD_REPUTATION);
                    if (pot as i32) > read_i16(clube + club::REPUTATION) as i32 - 0x5dc && pot > 0x1f4
                        && sortear(read_u8_volatile(person + staff::AMBITION) as i8 as i32) != 0 && (read_u8_volatile(contrato + 0x3d) as i8) < 0
                        && transfer_window_open(person, clube) {
                        erase(contrato, 4);
                        mark(contrato, 4, 0);
                    }
                }
            }
        } else if place(this, person, clube) == 1 && sortear(read_u8_volatile(person + staff::AMBITION) as i8 as i32) > 5
            && read_i16(player + player_sheet::CURRENT_REPUTATION) > read_i16(clube + club::REPUTATION) && read_i16(player + player_sheet::CURRENT_REPUTATION) > 0x4e2 {
            erase(contrato, 4);
            mark(contrato, 0x10, 0);
        }
    }
    let p = (contrato + 0x44) as *mut i8;
    *p = lowered_morale(*p, m); // a moral cai
}





/// Address of the person's 79-byte staff state record (+0 main contract index, -1 none).
///
/// [pt] A vaga do contrato principal da pessoa: o registro de 79 bytes dela em [0xae2d0c] (+0 o índice do contrato, −1 sem).
pub(crate) unsafe fn main_contract_slot(person: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f))
}

/// `CONTRACT_MANAGER::remove_contract(person)`: frees the person's main contract entry, puts −1 in her slot of the
/// 79-byte record and recalculates the wage bill of the contract's club.
///
/// With the main contract (index in the person's 79-byte record), it frees its entry (+0 = −1 in the CONTRACT_MANAGER's
/// 0x50-byte vector), puts −1 in the person's slot and redoes the wage bill (0x598370) of the finances (0x5a0590) of
/// the contract club (+4). Without a contract: nothing.
/// Original: `CONTRACT_MANAGER::remove_contract` (0x4d0b50, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d0b50 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract(pessoa)`: com o contrato
/// principal (índice no registro de 79 bytes da pessoa), libera a entrada dele (+0 = −1 no vetor de 0x50 bytes do
/// CONTRACT_MANAGER), põe −1 na vaga da pessoa e refaz a folha de salários (0x598370) das finanças (0x5a0590) do clube
/// do contrato (+4). Sem contrato: nada.
pub(crate) unsafe extern "thiscall" fn remove_contract(this: usize, person: usize) {
    let indice = read_i32(main_contract_slot(person));
    if indice == -1 {
        return;
    }
    let e = read_usize(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let id_club = read_usize(e + 4);
    write_usize(e, usize::MAX);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(id_club.wrapping_mul(club::SIZE));
    crate::comparison::branch("cpo_2 tirar contrato");
    write_usize(main_contract_slot(person), usize::MAX); // a pessoa fica sem contrato
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let f = get_club_finance(read_usize(G_FINANCE_TABLE), clube);
    if f != 0 {
        let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
        folha(f);
    }
}

/// `CONTRACT_MANAGER::resubmit_held_offers(id)`: resubmits the person's held offers as live offers, removes each one
/// from the list and returns how many.
///
/// It walks the held offers list (+0x38) and, for each one of person `id` (+0): with the transfer (+0x59, 0x8c61e0 over
/// [0xde1f54]) whose club (+0x24) is the offer's (+0x5d), it sets state 0xf to 8 and resubmits the offer (0x4cc490 with
/// the transfer, the club, byte +0x61, 1, 1, 0); without a transfer (+0x59 = −1) it resubmits with −1, −1. It removes
/// the offer from the list (0x68d1f0) and counts it. Byte +0x61 is pushed in the whole register (the upper bits of
/// whatever was in it: the pointer read or −1). Returns how many.
/// Original: `CONTRACT_MANAGER::resubmit_held_offers` (0x4cf4a0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cf4a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::resubmit_held_offers(id)`: percorre a lista
/// das ofertas guardadas (+0x38) e, para cada uma da pessoa `id` (+0): com a transferência (+0x59, 0x8c61e0 em
/// [0xde1f54]) cujo clube (+0x24) é o da oferta (+0x5d), põe o estado 0xf em 8 e reenvia a oferta (0x4cc490 com a
/// transferência, o clube, o byte +0x61, 1, 1, 0); sem transferência (+0x59 = −1) reenvia com −1, −1. Tira a oferta da
/// lista (0x68d1f0) e conta. O byte +0x61 vai empurrado no registrador inteiro (os bits de cima do que estava nele:
/// o ponteiro lido ou −1). Devolve quantas.
pub(crate) unsafe extern "thiscall" fn resubmit_held_offers(this: usize, id: usize) -> u32 {
    let list = read_usize(this + contract_manager_fields::HELD_OFFERS);
    if list == 0 {
        return 0;
    }
    write_usize(list + 0xa, read_usize(list + 2));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let reenviar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
    let mut n = 0u32;
    let mut item = next(read_usize(this + contract_manager_fields::HELD_OFFERS));
    while item != 0 {
        if read_usize(item) == id {
            let t = read_usize(item + 0x59);
            let byte = *((item + 0x61) as *const u8) as u32;
            if t as i32 != -1 {
                let find: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
                let p = find(read_usize(G_TRANSFER_MANAGER), t);
                if p != 0 {
                    let q = read_usize(p);
                    if q != 0 && read_usize(q + 0x24) == read_usize(item + 0x5d) {
                        if *((q + 0x2c) as *const u8) == 0xf {
                            *((q + 0x2c) as *mut u8) = 8;
                        }
                        crate::comparison::branch("cpo_2 reenviar oferta");
                        reenviar(this, item, t, read_usize(item + 0x5d), (q as u32 & 0xffff_ff00) | byte, 1, 1, 0); // with the transfer
                    }
                }
            } else {
                reenviar(this, item, usize::MAX, usize::MAX, 0xffff_ff00 | byte, 1, 1, 0);
            }
            remover(read_usize(this + contract_manager_fields::HELD_OFFERS), item);
            n += 1;
        }
        item = next(read_usize(this + contract_manager_fields::HELD_OFFERS));
    }
    n
}

/// `contract_manager_season_end_month(club)` (stdcall, ret 4; SEH frame without a destructor on the normal path): the
/// month (0..11) of the date ten days after the end of the season of the club's nation.
///
/// The club's season end date (Date() 0x54a110 and 0x669bd0 with 0), plus 10 days (0x54c5b0), and its month (0x54cfb0).
/// Returns the eax of 0x54cfb0 (the month 0..11).
/// Original: `contract_manager_season_end_month` (0x4d45f0, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4d45f0 } (stdcall, ret 4, contract_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `contract_manager_season_end_month(clube)`: a data do fim da temporada do clube (Data() 0x54a110 e 0x669bd0 com 0),
/// mais 10 dias (0x54c5b0), e o mês dela (0x54cfb0). Devolve o eax da 0x54cfb0 (o mês 0..11).
pub(crate) unsafe extern "stdcall" fn contract_manager_season_end_month(clube: usize) -> u32 {
    let mut d1 = [0u32; 2];
    let mut d2 = [0u32; 2];
    let construct: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let end: unsafe extern "C" fn(usize, u32, *mut u32) = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
    let add: unsafe extern "thiscall" fn(*mut u32, *mut u32, i32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let month: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    construct(d1.as_mut_ptr());
    end(clube, 0, d1.as_mut_ptr());
    let p = add(d1.as_mut_ptr(), d2.as_mut_ptr(), 0xa); // dez dias depois
    month(p)
}






/// Shows the CONTRACT_MANAGER "Memory Allocation Error / cannot create space" box for a line and text; the game
/// continues.
///
/// [pt] A caixa "cannot create space" do CONTRACT_MANAGER: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de
/// contract_manager.cpp 0x9a8190, `linha`, `texto`) em 0xae24d0; o título "Memory Allocation Error" (0x9a8108)
/// traduzido (0x66f4e0) em 0xae28bc, o corpo 0x9a7ff8 com o texto em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está).
pub(crate) unsafe fn show_cannot_create_space_box(row: u32, text: usize) {
    let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    split(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// `CONTRACT_MANAGER::get_free_staff_contract_space(append_only)`: returns the slot in the contract vector (0x50-byte
/// records at [this], capacity +4, count +8).
///
/// Without `append_only`, the first empty slot (person +0 = −1); otherwise (or with no empty slot) the one at the end:
/// when full it grows by 200 at a time (realloc 0x945667, or malloc 0x944e46 without a vector; without memory, box
/// 0x175 and −1) and the count goes up. Returns the index. Both callers (the contract creation 0x4c7560 and 0x4c7950)
/// pass 1.
/// Original: `CONTRACT_MANAGER::get_free_staff_contract_space` (0x4c79a0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c79a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::get_free_staff_contract_space(so_no_fim)`:
/// a vaga do vetor de contratos (registros de 0x50 bytes em [this], capacidade +4, conta +8). Sem `so_no_fim`, a
/// primeira vaga vazia (pessoa +0 = −1); senão (ou sem vaga vazia) a do fim: cheio, cresce de 200 em 200 (realloc
/// 0x945667, ou malloc 0x944e46 sem vetor; sem memória, caixa 0x175 e −1) e a conta sobe. Devolve o índice. Os dois
/// chamadores (a criação dos contratos 0x4c7560 e a 0x4c7950) passam 1.
pub(crate) unsafe extern "thiscall" fn get_free_staff_contract_space(this: usize, so_in_end: u32) -> i32 {
    if so_in_end == 0 {
        let n = read_usize(this + 8) as i32;
        let p = read_usize(this);
        let mut i = 0i32;
        while i < n {
            if read_usize(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                return i;
            }
            i += 1;
        }
    }
    if read_usize(this + 8) == read_usize(this + 4) {
        let cap = read_usize(this + 4).wrapping_add(200);
        let p = read_usize(this);
        write_usize(this + 4, cap);
        let q = if p != 0 {
            let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realloc(p, cap.wrapping_mul(0x50) as u32)
        } else {
            let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alloc(cap.wrapping_mul(0x50) as u32)
        };
        write_usize(this, q);
        if q == 0 {
            show_cannot_create_space_box(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = read_usize(this + 8).wrapping_add(1);
    write_usize(this + 8, n); // one more slot
    (n as i32).wrapping_sub(1)
}

/// `CONTRACT_MANAGER::add_staff_contract(person, value)`: takes the appended slot for a new person (hired manager,
/// regen), links the person to it and generates the initial contract.
///
/// The slot at the end (0x4c79a0 with 1); without a slot, nothing. Links the person to the slot (index +0 of her
/// 0x4f-byte record in [0xae2d0c]), puts her id in the contract (+0) and generates the initial contract (0x85e960 with
/// the value). Called by the manager hiring (0x68f9c0, MANAGER_MANAGER::daily_update) and by the regen that signs with
/// a club (0x7a6f10). Returns the original's eax (that of 0x85e960, or the negative index); the callers do not read it.
/// Original: `CONTRACT_MANAGER::add_staff_contract` (0x4c7950, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c7950 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::add_staff_contract(pessoa, valor)`: a vaga
/// do fim (0x4c79a0 com 1); sem vaga, nada. Liga a pessoa à vaga (o índice +0 do registro de 0x4f bytes dela em
/// [0xae2d0c]), põe o id dela no contrato (+0) e gera o contrato inicial (0x85e960 com o valor). Pela contratação
/// de técnico (0x68f9c0, MANAGER_MANAGER::daily_update) e pelo regen que assina com um clube (0x7a6f10). Devolve o
/// eax da original (o da 0x85e960, ou o índice negativo); os chamadores não o leem.
pub(crate) unsafe extern "thiscall" fn add_staff_contract(this: usize, person: usize, value: u32) -> u32 {
    let slot: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_FREE_STAFF_CONTRACT_SPACE);
    let i = slot(this, 1);
    if i < 0 {
        return i as u32;
    }
    crate::comparison::branch("cpl_4 contrato da pessoa");
    write_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)), i as usize); // a pessoa aponta a vaga
    let c = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
    write_usize(c, read_usize(person));
    let gerar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GENERATE_INITIAL_CONTRACT);
    gerar(c, person, value) // the person’s value
}

/// Reminder dates (type 2) a type-1 decision date gets before it, from the contract morale; returns up to two dates and
/// the count.
///
/// [pt] As datas dos lembretes (tipo 2) que a data de decisão de tipo 1 leva antes dela: com o registro de jogador
/// (+0x61), L = moral do contrato (+0x44) + 100 (8 bits; o contrato pela tabela [0xae2d0c], como a 0x4cf5a0
/// inline: sem contrato a original lê o endereço 0x44 e cai, como aqui); L ≤ 0x50 sem lembrete; L > 0x96 um a
/// L × 90 / 200 dias antes; e um a L / 4 dias antes (sem o registro de jogador, L = 100). As datas pela
/// Date::sub_days (0x54c8e0), na ordem da original.
pub(crate) unsafe fn decision_reminder_dates(this: usize, person: usize, data: usize, kind: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if kind != 1 {
        return (v, k);
    }
    let minus: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut l: u8 = 100;
    if read_usize(person + staff::PLAYER) != 0 {
        let id = read_usize(person) as i32;
        let c = if person == 0 || id >= read_usize(G_STAFF_COUNT) as i32 {
            0
        } else {
            let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f))) as i32;
            if i < 0 { 0 } else { read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50)) }
        };
        l = (*((c + 0x44) as *const u8)).wrapping_add(100);
        if l <= 0x50 {
            return (v, k);
        }
        if l > 0x96 {
            let mut t = [0u8; 8];
            let r = minus(data, t.as_mut_ptr(), (l as u32 * 90) / 200);
            v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
            k += 1;
        }
    }
    let mut t = [0u8; 8];
    let r = minus(data, t.as_mut_ptr(), (l as u32 * 50) / 200);
    v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
    k += 1;
    (v, k)
}

/// `CONTRACT_MANAGER::add_decision_date(person, date, slice, type)`: appends an entry to the 14-byte decision date
/// vector; type 1 first adds type 2 reminders from the contract morale.
///
/// Appends to the 14-byte vector [this+0xc] (capacity +0x10, count +0x14; it grows by 100 at a time when the count
/// passes capacity − 4, realloc 0x945667 or malloc 0x944e46; without memory, box 0x103e and nothing) the entry (person
/// id +0, date +4, time slice of the day +0xc, type +0xd). Type 1 is preceded by the type 2 reminders
/// (`decision_reminder_dates`), without a slice (+0xc is left as it is). Called by the offers: the person's decision
/// (0x4cd230, 0x4ce4e0). The date copies (0x417c10, with no other effect) become 8-byte copies.
/// Original: `CONTRACT_MANAGER::add_decision_date` (0x4cd6d0, contract_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4cd6d0 } (thiscall, ret 0x10, contract_manager.cpp): `CONTRACT_MANAGER::add_decision_date(pessoa, data, fatia,
/// tipo)`: acrescenta ao vetor de 14 bytes [this+0xc] (capacidade +0x10, conta +0x14; cresce de 100 em 100 quando a
/// conta passa da capacidade − 4, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa 0x103e e nada) a entrada
/// (id da pessoa +0, data +4, fatia do dia +0xc, tipo +0xd). O tipo 1 leva antes os lembretes de tipo 2
/// (`decision_reminder_dates`), sem fatia (+0xc fica como está). Pelas propostas: a decisão da pessoa (0x4cd230, 0x4ce4e0).
/// As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
pub(crate) unsafe extern "thiscall" fn add_decision_date(this: usize, person: usize, data: usize, slice: u32, kind: u32) {
    crate::comparison::branch("cpl_4 data de decisao");
    if read_usize(this + contract_manager_fields::DECISION_COUNT) as i32 > (read_usize(this + contract_manager_fields::DECISION_CAPACITY) as i32).wrapping_sub(4) {
        let cap = read_usize(this + contract_manager_fields::DECISION_CAPACITY).wrapping_add(100);
        write_usize(this + contract_manager_fields::DECISION_CAPACITY, cap);
        let p = read_usize(this + contract_manager_fields::DECISION_DATES);
        let q = if p != 0 {
            let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realloc(p, cap.wrapping_mul(14) as u32)
        } else {
            let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alloc(cap.wrapping_mul(14) as u32)
        };
        write_usize(this + contract_manager_fields::DECISION_DATES, q);
        if q == 0 {
            show_cannot_create_space_box(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = decision_reminder_dates(this, person, data, kind as u8);
    for d in &lembretes[..k] {
        let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
        let e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_mul(14));
        write_usize(e, read_usize(person));
        core::ptr::write_unaligned((e + 4) as *mut [u8; 8], *d);
        *((e + 0xd) as *mut u8) = 2;
        write_usize(this + contract_manager_fields::DECISION_COUNT, n.wrapping_add(1));
    }
    let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
    let e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_mul(14));
    write_usize(e, read_usize(person));
    *((e + 0xc) as *mut u8) = slice as u8;
    core::ptr::write_unaligned((e + 4) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    *((e + 0xd) as *mut u8) = kind as u8;
    write_usize(this + contract_manager_fields::DECISION_COUNT, n.wrapping_add(1)); // the decision date
}

// simplification: all three reallocate (and 0x4c7950 generates the contract through 0x85e960); they do not run twice. The comparer builds
// the plan beforehand, runs only the original and checks; Rust mode and the career prove the copies.





/// The person's contract with the club whose relegation clause (+0x1f = 1) is still to trigger, else 0.
///
/// [pt] O contrato (registro de 80 bytes em [this], pelo índice +0 do registro de 79 bytes em [0xae2d0c]) da pessoa `p`
/// com o clube `clube` (+4 = id do clube) e a cláusula de rebaixamento ainda por acionar (+0x1f = 1); senão 0.
pub(crate) unsafe fn contract_with_relegation_clause(this: usize, clube: usize, p: usize) -> usize {
    if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    let c = (i as usize).wrapping_mul(80).wrapping_add(read_usize(this));
    if c == 0 || read_usize(c + contract::CLUB) != read_usize(clube) || *((c + contract::RELEGATION_RC) as *const u8) != 1 {
        return 0;
    }
    c
}

/// The club and its linked B team (a B club or null gives none: the A club handles both).
///
/// [pt] O clube e o parceiro (time A/B, 0x540a50 ativo): o clube B não aciona (o A cuida dos dois). Clube nulo ou B:
/// nenhum.
pub(crate) unsafe fn club_and_b_team(clube: usize) -> [usize; 2] {
    if clube == 0 {
        return [0, 0];
    }
    let mut eh_a = 0u32;
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let p = partner(clube, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        return [0, 0];
    }
    [clube, p]
}

/// `CONTRACT_MANAGER::trigger_relegation_clauses(club, league)`: triggers the relegation clauses at a relegated club.
///
/// At the relegated club and at its partner (A/B team), each of the 50 staff and squad members (+0xd7) with a
/// relegation clause still to be triggered in the contract with the club (+0x1f = 1) goes to 2 and the news is posted
/// (0x8e3db0 over [0xde1f54] with type 3).
/// Original: `CONTRACT_MANAGER::trigger_relegation_clauses` (0x4cd030, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd030 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::trigger_relegation_clauses(clube,
/// liga)`: no clube rebaixado e no parceiro (time A/B), cada um dos 50 da comissão e elenco (+0xd7) com a cláusula de
/// rebaixamento por acionar no contrato com o clube (+0x1f = 1) passa a 2 e sai a notícia (0x8e3db0 sobre
/// [0xde1f54] com o tipo 3).
pub(crate) unsafe extern "thiscall" fn trigger_relegation_clauses(this: usize, clube: usize, league: usize) {
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_POST_RELEASE_CLAUSE_ACTIVATED_NEWS);
    for c in club_and_b_team(clube) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = read_usize(c + club::SQUAD + k * 4);
            let contrato = contract_with_relegation_clause(this, c, p);
            if contrato != 0 {
                crate::comparison::branch("cpk_3 clausula de rebaixamento");
                *((contrato + contract::RELEGATION_RC) as *mut u8) = 2; // the triggered clause
                noticia(read_usize(G_TRANSFER_MANAGER), p, league, 3, contrato);
            }
        }
    }
}

/// First contract offer in list l (cursor untouched) from person p (+0) to club c (+4).
///
/// [pt] A primeira oferta de contrato da lista `l` (sem mexer no cursor) da pessoa `p` (+0) ao clube `c` (+4).
pub(crate) unsafe fn first_offer_for_staff_club(l: usize, p: usize, c: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == p && read_usize(e + 4) == c)
}

/// `CONTRACT_MANAGER::remove_contract_offer(person, club)`: removes the person's first offer to the club.
///
/// The first offer in the list (+0x34) of the person to the club (ids): returns the record reservation (+0x63 ≠ −1:
/// 0x7a81a0 over [0xb7434c] with +0x67, +0x6b), clears the agreed-contract flag if it was agreed (+0x50 = 2: 0x8c5430),
/// leaves the list (0x68d1f0) and the person's pending count (+8 of the 79-byte record in [0xae2d0c]) drops by one.
/// Original: `CONTRACT_MANAGER::remove_contract_offer` (0x4cca70, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cca70 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer(pessoa, clube)`: a
/// primeira oferta da lista (+0x34) da pessoa ao clube (ids): devolve a reserva de registro (+0x63 ≠ −1: 0x7a81a0
/// sobre [0xb7434c] com +0x67, +0x6b), tira a marca de contrato acertado se ela estava acertada (+0x50 = 2: 0x8c5430),
/// sai da lista (0x68d1f0) e as pendentes da pessoa (+8 do registro de 79 bytes em [0xae2d0c]) descem uma.
pub(crate) unsafe extern "thiscall" fn remove_contract_offer(this: usize, person: *const u32, clube: *const u32) {
    let (c, p) = (*clube as usize, *person as usize);
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = next(l);
    while e != 0 && !(read_usize(e) == p && read_usize(e + 4) == c) {
        e = next(l);
    }
    if e == 0 {
        return;
    }
    crate::comparison::branch("cpk_3 tirar oferta de contrato");
    let reserve = read_usize(e + 0x63);
    if reserve as i32 != -1 {
        let devolver: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_REGEN_ADD_SPARE_NON_PLAYER_RECORD);
        devolver(read_usize(G_PLAYER_REGEN), reserve, read_usize(e + 0x67), read_usize(e + 0x6b));
    }
    if *((e + 0x50) as *const u8) == 2 {
        let unmark: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
        unmark(read_usize(G_TRANSFER_MANAGER), person);
    }
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    remove(l, e);
    let n = (read_usize(G_STAFF_STATE_TABLE).wrapping_add((*person as usize).wrapping_mul(79)) + 8) as *mut u8;
    *n = (*n).wrapping_sub(1); // one fewer pending
}






/// Calls LList::get_next on list l.
pub(crate) unsafe fn contract_manager_list_next(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    f(l)
}

/// Calls LList::remove of item e from list l.
pub(crate) unsafe fn list_remove(l: usize, e: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    f(l, e);
}

/// Pointer to the stored suspensions counter (+0x1c) of the 29-byte discipline record of id in [this+0x14].
///
/// [pt] O contador de suspensões guardadas (+0x1c) do registro de 29 bytes do id `id` em [this+0x14].
pub(crate) unsafe fn stored_suspension_counter(this: usize, id: usize) -> *mut i8 {
    (read_usize(this + 0x14).wrapping_add(id.wrapping_mul(29)) + 0x1c) as *mut i8
}


/// Pointer to the pending offers byte (+8) of the person's 79-byte staff state record.
///
/// [pt] O byte de ofertas pendentes (+8) do registro de 79 bytes da pessoa em [0xae2d0c].
pub(crate) unsafe fn pending_offers_byte(id: usize) -> *mut u8 {
    (read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(79)) + 8) as *mut u8
}

/// `CONTRACT_MANAGER::remove_pending_offers_for_staff(person)`: removes all of the person's pending offers.
///
/// With the offers list (+0x34) and pending offers in the person's record (+8 of the 79-byte one in [0xae2d0c]): clears
/// the agreed-contract flag (0x8c5430 over [0xde1f54]) and, from the start of the list, removes each offer of the
/// person (0x68d1f0), decrementing the byte, until it reaches zero.
/// Original: `CONTRACT_MANAGER::remove_pending_offers_for_staff` (0x4ccbf0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
pub(crate) unsafe extern "thiscall" fn remove_pending_offers_for_staff(this: usize, person: *const u32) {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 || *pending_offers_byte(*person as usize) == 0 {
        return;
    }
    let unmark: unsafe extern "thiscall" fn(usize, *const u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
    unmark(read_usize(G_TRANSFER_MANAGER), person);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if read_usize(e) as u32 == *person {
            list_remove(l, e);
            let p = pending_offers_byte(*person as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // the last pending one
            }
        }
        e = contract_manager_list_next(l);
    }
}

/// First offer in list l (cursor untouched) matching the transfer offer's person and club and still without decision
/// (+0x62 = 0).
///
/// [pt] A primeira oferta da lista `l` (sem mexer no cursor) da pessoa (+0) e do clube (+4) da oferta de transferência e
/// ainda sem decisão (+0x62 = 0).
pub(crate) unsafe fn offer_to_schedule(l: usize, offer: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == read_usize(offer + offer::TARGET_STAFF_ID) && read_usize(e + 4) == read_usize(offer + 8) && *((e + 0x62) as *const u8) == 0)
}

/// 1 when the offered player's ability (+0xb of the player record) / 500 exceeds the offering club's reputation
/// (+0x80), else 0.
///
/// [pt] Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
pub(crate) unsafe fn ability_above_club_reputation(offer: usize) -> u32 {
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(staff::SIZE));
    let hab = *((read_usize(person + staff::PLAYER) + 0xb) as *const i16) as i32 / 500;
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 8).wrapping_mul(club::SIZE));
    (hab > *((clube + club::REPUTATION) as *const i16) as i32) as u32
}

/// `CONTRACT_MANAGER::schedule_decision_for_transfer(offer)`: marks the first still-undecided contract offer of the
/// person and club and schedules the decision.
///
/// The first contract offer in the list (+0x34) of the person (+0xc) and the club (+8) of the transfer offer still
/// without a decision (+0x62) is marked (1) and the person's decision is scheduled (0x4cd230 with slot +0x20). Returns
/// whether the person's ability / 500 exceeds the club's reputation.
/// Original: `CONTRACT_MANAGER::schedule_decision_for_transfer` (0x4cc930, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
pub(crate) unsafe extern "thiscall" fn schedule_decision_for_transfer(this: usize, offer: usize) -> u32 {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if read_usize(e) == read_usize(offer + offer::TARGET_STAFF_ID) && read_usize(e + 4) == read_usize(offer + 8) && *((e + 0x62) as *const u8) == 0 {
            *((e + 0x62) as *mut u8) = 1; // a oferta vale
            let schedule: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_OFFER_DECISION);
            schedule(this, read_usize(offer + offer::TARGET_STAFF_ID) as u32, read_usize(offer + offer::SLOT_INDEX) as u32);
            break;
        }
        e = contract_manager_list_next(l);
    }
    ability_above_club_reputation(offer)
}





/// `CONTRACT_MANAGER::remove_held_offers(person)`: removes from the held offers list (+0x38; cursor +0xa back to the
/// start +2) every item (0x68d1f0) whose +0 is the id `person` (walked with 0x68d1b0). Without a list: nothing.
/// Original: `CONTRACT_MANAGER::remove_held_offers` (0x4cf560, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cf560 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_held_offers(pessoa)`: da lista das
/// ofertas guardadas (+0x38; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item cujo +0 é o id `pessoa`
/// (0x68d1b0 percorre). Sem lista: nada.
pub(crate) unsafe extern "thiscall" fn remove_held_offers(this: usize, person: u32) {
    let l = read_usize(this + contract_manager_fields::HELD_OFFERS);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut e = next(l);
    while e != 0 {
        if read_usize(e) as u32 == person { // the person’s offer
            remove(l, e);
        }
        e = next(l);
    }
}

/// Whether suspension type t applies at the loan club, by the type's competition (+0x17) or rule (+0x1b) against the
/// club's nation.
///
/// [pt] A suspensão do tipo `t` (vetor de tipos [this], 4 bytes cada) vale no clube do empréstimo? Pela competição do tipo
/// (+0x17, −1 = nenhuma) igual à nação do clube (+0x53, id em +0); sem competição, pelo +0x1b (−1: vale) igual ao
/// [+0x71] da nação. Clube sem nação: não vale.
pub(crate) unsafe fn suspension_valid_at_club(this: usize, t: i8, clube: usize) -> bool {
    let kind = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nacao = read_usize(clube + club::NATION);
    let comp = read_usize(kind + 0x17) as i32;
    if comp != -1 {
        return nacao != 0 && read_usize(nacao) as i32 == comp;
    }
    let x = read_usize(kind + 0x1b) as i32;
    if x == -1 {
        return true;
    }
    if nacao == 0 {
        return false;
    }
    let m = read_usize(nacao + nation::CONTINENT);
    m != 0 && read_usize(m) as i32 == x // the nation’s +0x71
}

/// Loan club and the 5-byte suspension slots of the player's discipline record that do not apply there (skipping the
/// 4th like the original).
///
/// [pt] O clube do empréstimo (+4 da ficha de contrato, clubes de 0x245 bytes em [0xae23b4]) e o registro de suspensões
/// do jogador (29 bytes em [this+0x14]); as vagas (5 bytes: tipo e o resto) que não valem lá, menos a 4ª (índice 3,
/// que o original pula).
pub(crate) unsafe fn suspensions_to_store(this: usize, sheet: usize, player: usize) -> (usize, Vec<u8>) {
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(sheet + 4).wrapping_mul(club::SIZE));
    let reg = read_usize(this + 0x14).wrapping_add(read_usize(player).wrapping_mul(29));
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


/// Job a vacancy job code looks for: 6/8/9/10 itself, 13→6, 15→8, others none.
///
/// [pt] O cargo que a vaga `cargo` (byte com sinal) procura: 6, 8, 9 e 10 o mesmo; 13 procura 6 e 15 procura 8 (o byte do
/// argumento é trocado); os outros, nenhum.
pub(crate) fn job_sought_by_slot(job: u32) -> Option<u32> {
    match job as u8 as i8 {
        6 | 8 | 9 | 10 => Some(job),
        13 => Some((job & !0xff) | 6),
        15 => Some((job & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// First person in G_STAFF_TABLE without club, with the given job, a player record and the club's nation.
///
/// [pt] A primeira pessoa (de [0xae23bc], 0x6e bytes, [0xae2364] delas) sem clube (+0x39), no cargo (+0x3d), com registro
/// (+0x69) e da nação (+0x1a) do clube (+0x53).
pub(crate) unsafe fn first_unemployed_staff(clube: usize, job: u8) -> Option<usize> {
    let n = read_usize(G_STAFF_COUNT) as i32;
    let base = read_usize(G_STAFF_TABLE);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        read_usize(p + staff::CLUB_JOB) == 0 && *((p + staff::JOB_FOR_CLUB) as *const u8) == job && read_usize(p + staff::NON_PLAYER) != 0 && read_usize(p + staff::NATION) == read_usize(clube + club::NATION)
    })
}






/// The person's contract as 0x4cbb60 finds it inline (null, out of range or no index → 0).
///
/// [pt] O contrato da pessoa como a 0x4cbb60 acha por dentro (sem chamar a 0x4cf5a0): pessoa não nula, id abaixo de
/// [0xae2364], índice da tabela [0xae2d0c] (79 bytes por pessoa) não negativo; registro de 80 bytes em [objeto].
pub(crate) unsafe fn squad_contract_of_staff(objeto: usize, person: usize) -> usize {
    if person == 0 {
        return 0;
    }
    let id = read_u32(person) as i32;
    if id >= read_u32(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    (read_u32(objeto) as usize).wrapping_add((i as usize).wrapping_mul(80))
}

/// `CONTRACT_MANAGER::get_squad_placement(person, club)`: the person's place at `club`: 0 none, 1 reserves, 2 first
/// team, 3 at the linked B team, 4 at the main club with a B team.
///
/// Only for player jobs (+0x3d other than 12 and 0x53b620). At her own club (+0x39) when it can have reserves
/// (0x542740): 2 with bit 0 of +0x5e, otherwise 1. The partner club (0x540a50 with 1; the output byte is the person's
/// slot, which keeps her pointer if 0x540a50 does not write it): being A with `club` her own, 4; being B with `club`
/// the partner A, 3. Otherwise by the contract (0x4cbb60 finds it internally in [0xae1050], stored in [0xae1080]): the
/// contract club, which is neither her own nor the partner already seen; with bit 6 of the contract's +0x35 (found
/// again in [this]): at `club` with reserves, 1 with the bit and 2 without; or with the partner (0x540a50) being `club`
/// (or the contract's being `club`), 3 with the bit and 4 without. The game reads only al.
/// Original: `CONTRACT_MANAGER::get_squad_placement` (0x4cbb60, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cbb60 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_placement(pessoa, clube)`: o
/// lugar da pessoa no `clube`: 0 nenhum, 1 reservas, 2 principal, 3 no time B ligado, 4 no clube principal com time B.
/// Só para os cargos de jogador (+0x3d diferente de 12 e 0x53b620). No próprio clube (+0x39) que pode ter reservas
/// (0x542740): 2 com o bit 0 de +0x5e, senão 1. O clube parceiro (0x540a50 com 1; o byte de saída é a vaga da
/// pessoa, que fica com o ponteiro dela se a 0x540a50 não gravar): sendo A e `clube` o próprio, 4; sendo B e `clube` o
/// parceiro A, 3. Senão pelo contrato (0x4cbb60 acha por dentro em [0xae1050], gravado em [0xae1080]): o clube do
/// contrato, que não é o próprio nem o parceiro já visto; com o bit 6 de +0x35 do contrato (achado de novo em [this]):
/// no `clube` com reservas, 1 com o bit e 2 sem; ou com o parceiro (0x540a50) sendo o `clube` (ou o do contrato o
/// `clube`), 3 com o bit e 4 sem. O jogo lê só o al.
pub(crate) unsafe extern "thiscall" fn get_squad_placement(this: usize, person: usize, clube: usize) -> u8 {
    let kind = *((person + staff::JOB_FOR_CLUB) as *const i8);
    let kind_of_player: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    if kind == 12 || kind_of_player(kind) == 0 {
        return 0;
    }
    let reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let own = read_u32(person + staff::CLUB_JOB) as usize;
    if own != 0 && own == clube && kind_of_player(*((person + staff::JOB_FOR_CLUB) as *const i8)) != 0 && reservas(clube) != 0 {
        return if *((person + staff::PLAYING_SQUAD) as *const u8) & 1 != 0 { 2 } else { 1 }; // main or reserves
    }
    let mut slot = person as u32; // the argument’s slot: 0x540a50 may not write it
    let mut b = 0usize;
    if own != 0 {
        b = partner(own, &mut slot, 1);
        if b != 0 {
            if slot != 0 {
                if clube == read_u32(person + staff::CLUB_JOB) as usize {
                    return 4;
                }
            } else if clube == b {
                return 3;
            }
        }
    }
    let c = squad_contract_of_staff(read_u32(G_CONTRACT_MANAGER) as usize, person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c; // the contract record
    if c == 0 {
        return 0;
    }
    let i = read_u32(c + 4) as usize;
    let other = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if other == 0 || other == read_u32(person + staff::CLUB_JOB) as usize || other == b {
        return 0;
    }
    let c2 = squad_contract_of_staff(this, person);
    let mark = (*((c2 + contract::CONTRACT_TYPE) as *const u8) << 1) as i8 >> 7; // o bit 6, estendido
    if other == clube && reservas(clube) != 0 {
        return if mark == 0 { 2 } else { 1 };
    }
    let p = partner(other, &mut slot, 1);
    if p == 0 || (p != clube && other != clube) {
        return 0;
    }
    if mark == 0 { 4 } else { 3 }
}

/// If the squad placement (0x4cbb60) is one of de, toggles it (0x4cbd20); else 0.
///
/// [pt] A 0x4cbb60 pelo endereço (passa pelo desvio) e a troca 0x4cbd20 (com 0 na saída do novo clube).
pub(crate) unsafe fn toggle_placement_if_in(this: usize, person: usize, clube: usize, de: [u8; 2]) -> u32 {
    let place: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let l = place(this, person, clube);
    if l != de[0] && l != de[1] {
        return 0;
    }
    let swap: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TOGGLE_SQUAD_PLACEMENT);
    swap(this, person, clube, 0)
}

/// in the reserves or at the B side (placement 1 or 3), swaps through 0x4cbd20; else 0
/// Original: `CONTRACT_MANAGER::move_to_first_team` (0x4cc0b0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cc0b0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_first_team(pessoa, clube)`: nos
/// reservas (1) ou no time B (3), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
pub(crate) unsafe extern "thiscall" fn move_to_first_team(this: usize, person: usize, clube: usize) -> u32 {
    toggle_placement_if_in(this, person, clube, [1, 3]) // reserves or B team
}

/// in the first team or at the club with a B side (placement 2 or 4), swaps through 0x4cbd20; else 0
/// Original: `CONTRACT_MANAGER::move_to_reserves` (0x4cc0f0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cc0f0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_reserves(pessoa, clube)`: no
/// principal (2) ou no clube com time B (4), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
pub(crate) unsafe extern "thiscall" fn move_to_reserves(this: usize, person: usize, clube: usize) -> u32 {
    toggle_placement_if_in(this, person, clube, [2, 4]) // senior team or club with a B team
}


/// Comparer of move_to_first_team / move_to_reserves: side by side outside the toggle placements, original only inside.
///
/// [pt] Comparador das duas trocas: o lugar (0x4cbb60, pelo desvio) decide antes; fora dos lugares da troca, lado a lado
/// pela ficha [0xae1080] (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
pub(crate) unsafe fn compare_placement_toggle(i: usize, name: &str, this: usize, person: usize, clube: usize, de: [u8; 2]) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let place: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let sheet = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let l = place(this, person, clube);
    *(G_CONTRACT_LOOKUP_RESULT as *mut u32) = sheet;
    if l == de[0] || l == de[1] {
        return o(this, person, clube);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, person, clube), || toggle_placement_if_in(this, person, clube, de));
    PLACAR.record(igual && r == n, || format!("{}({:x}, {:x}) lugar {} original={} rust={} {:x?} x {:x?}", name, person, clube, l, r, n, mo, mr));
    r
}






/// `STAFF_CONTRACT::operator=`: copies the contract (0x50 bytes) from `source` field by field.
///
/// Four dwords, three dwords (+0x10), five bytes (+0x1c), +0x21, the start (+0x25) and end (+0x2d) dates through
/// 0x417c10, the bit-field byte +0x35 in three steps (6 bits, the 7th, the 8th: at the end it is `source`'s), +0x36,
/// +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e and +0x4f in two steps (low nibble and
/// then the rest). Returns `this`.
/// Original: `STAFF_CONTRACT::operator=` (0x4cc810, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cc810 } (thiscall, ret 4, contract_manager.cpp): `STAFF_CONTRACT::operator=`: copia o contrato (0x50 bytes) de
/// `origem`: quatro duplas, três duplas (+0x10), cinco bytes (+0x1c), +0x21, as datas de início (+0x25) e do fim
/// (+0x2d) pela 0x417c10, o byte de campos de bits +0x35 em três passos (6 bits, o 7º, o 8º: no fim é o de `origem`),
/// +0x36, +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e e o +0x4f em dois passos (nibble
/// baixo e depois o resto). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn contract_manager_operator(this: usize, source: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, read_u32(source + o));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((source + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = read_u8(source + o);
    for o in [0, 4, 8, 0xc, 0x10, 0x14, 0x18] {
        d(o);
    }
    for o in 0x1c..0x21 {
        b(o); // the five bytes
    }
    d(0x21);
    copy_record_date(this + staff_contract::DATE_STARTED, source + contract::DATE_STARTED);
    copy_record_date(this + staff_contract::CONTRACT_EXPIRES, source + contract::CONTRACT_EXPIRES);
    let m = (this + staff_contract::CONTRACT_TYPE) as *mut u8;
    let s = read_u8(source + contract::CONTRACT_TYPE);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ *m) & 0x3f) ^ *m; // the low 6 bits
    *m = cl;
    let al = ((((read_u8(source + contract::CONTRACT_TYPE) << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl; // o 7º bit
    *m = al;
    let s = read_u8(source + contract::CONTRACT_TYPE);
    *m = ((al ^ s) & 0x7f) ^ s; // o 8º bit
    d(0x36);
    b(0x3a);
    w(0x3b);
    b(0x3d);
    b(0x3e);
    b(0x3f);
    d(0x40);
    b(0x44);
    d(0x45);
    b(0x49);
    d(0x4a);
    b(0x4e);
    let m = (this + staff_contract::SQUAD_STATUS) as *mut u8;
    let c = *m;
    let al = ((read_u8(source + contract::SQUAD_STATUS) ^ c) & 0xf) ^ c; // o nibble baixo
    *m = al;
    let s = read_u8(source + contract::SQUAD_STATUS);
    *m = ((al ^ s) & 0xf) ^ s;
    this
}

/// `contract_manager_raise_match_morale(person, club, contract)` (stdcall, ret 0xc): raises the contract morale after a
/// match by the rating against the club reputation (up to 10), keeping morale between −100 and 100 and under the
/// grievance cap; clears grievances 4 and 0x10 and checks the request withdrawal.
///
/// With the three and the player record (+0x61): x = (rating +0xb − club reputation +0x80) + 417 × +0x18 − 11680; y = 1
/// + x × (6.6e-5 if x < 0, otherwise 6.6667e-5), between 0.01 and 2; the gain = ftol((+0x3f + 5) / y), at most 10. The
/// cap: 100 (50 with grievance 2), −50 per grievance 1, 8, 0x20, 0x40, 0x80, 0x100 while the cap does not go below −50.
/// The morale (+0x44) adds the gain within −100 and 100 and stays at or under the cap. Without +0x49: with gain > 5 and
/// morale + gain > 0, or happiness (+0x3d) ≥ 75 − gain, it clears grievances 4 and 0x10 (0x8674e0). Finally it checks
/// the withdrawal of the request (0x867240 with 0). Constants from .rdata; x87 in double precision, like the other
/// contract routines.
/// Original: `contract_manager_raise_match_morale` (0x4cb0a0, contract_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x4cb0a0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_raise_match_morale(pessoa, clube,
/// contrato)`: com os três e o registro de jogador (+0x61): x = (nota +0xb − reputação do clube +0x80) + 417 × +0x18
/// − 11680; y = 1 + x × (6,6e-5 se x < 0, senão 6,6667e-5), entre 0,01 e 2; o ganho = ftol((+0x3f + 5) / y), no máximo
/// 10. O teto: 100 (50 com a queixa 2), −50 por queixa 1, 8, 0x20, 0x40, 0x80, 0x100 enquanto o teto não passa de −50
/// para baixo. A moral (+0x44) soma o ganho entre −100 e 100 e fica no teto. Sem +0x49: com ganho > 5 e moral + ganho
/// > 0, ou felicidade (+0x3d) ≥ 75 − ganho, apaga as queixas 4 e 0x10 (0x8674e0). Por fim confere a retirada do pedido
/// (0x867240 com 0). Constantes do .rdata; o x87 em dupla precisão, como as outras do contrato.
pub(crate) unsafe extern "stdcall" fn contract_manager_raise_match_morale(person: usize, clube: usize, contrato: usize) {
    if contrato == 0 || clube == 0 || person == 0 {
        return;
    }
    let player = read_u32(person + staff::PLAYER) as usize;
    if player == 0 {
        return;
    }
    let a = core::ptr::read_unaligned((player + player_sheet::CURRENT_REPUTATION) as *const i16) as i32 - core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32;
    let x = (a as f64 + read_u8(person + staff::YEAR_OF_BIRTH) as i8 as f64 * read_f64_at(0x968d90)) - read_f64_at(0x968d88); // 0x968d90 = 417, 0x968d88 = 11680
    let mut y = (if x < read_f64_at(0x9689a0) { x * read_f64_at(0x968d80) } else { x * read_f64_at(0x968d78) }) + read_f64_at(0x967890); // 0x9689a0 = 0, 0x968d80 = 6.6e-05, 0x968d78 = 6.666666667e-05, 0x967890 = 1
    if y < read_f64_at(0x967898) { // 0x967898 = 0.01
        y = read_f64_at(0x967898); // 0x967898 = 0.01
    } else if y > read_f64_at(0x968950) { // 0x968950 = 2
        y = read_f64_at(0x968950); // 0x968950 = 2
    }
    let happiness = (read_u8(contrato + 0x3f) as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut gain = match_pl_ftol(happiness / y) as u8 as i8;
    if gain > 10 {
        gain = 10;
    }
    let queixas = read_u32(contrato + 0x45);
    let mut ceiling: i8 = if queixas & 2 != 0 { 50 } else { 100 };
    if queixas & 1 != 0 {
        ceiling = ceiling.wrapping_sub(50);
    }
    for q in [8u32, 0x20, 0x40, 0x80, 0x100] {
        if ceiling < -50 {
            break;
        }
        if queixas & q != 0 {
            ceiling = ceiling.wrapping_sub(50); // menos 50 por queixa
        }
    }
    let morale = (contrato + 0x44) as *mut i8;
    let cl = *morale;
    let s = cl as i32 + gain as i32;
    *morale = if s > 100 { 100 } else if s < -100 { -100 } else { cl.wrapping_add(gain) };
    if *morale > ceiling {
        *morale = ceiling;
    }
    if read_u8(contrato + contract::LEAVING_ON_BOSMAN) == 0 {
        let apaga = if gain > 5 && *morale as i32 + gain as i32 > 0 {
            true
        } else {
            read_u8(contrato + 0x3d) as i8 as i32 >= 75 - gain as i32
        };
        if apaga {
            let erase: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
            if queixas & 4 != 0 {
                erase(contrato, 4);
            }
            if read_u8(contrato + 0x45) & 0x10 != 0 {
                erase(contrato, 0x10);
            }
        }
    }
    let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
    retirada(contrato, 0);
}

/// `CONTRACT_MANAGER::remove_talks_records(person, manager, club)`: removes the person's talks (and of the club and
/// manager, when given), moving the last one into the gap, then sorts again.
///
/// In the talks vector ([this+0x28], [this+0x2c] records of 21 bytes), each one of the person (+0 = id) and, when
/// given, of the club (+4) and the manager (+8) is overwritten by the last one (5 dwords and a byte), the count goes
/// down and the same place is looked at again. If any are left, it sorts again (qsort 0x945333 with 0x4dc970).
/// Original: `CONTRACT_MANAGER::remove_talks_records` (0x4ca1c0, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4ca1c0 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::remove_talks_records(pessoa, tecnico,
/// clube)`: no vetor de conversas ([this+0x28], [this+0x2c] registros de 21 bytes), cada um da pessoa (+0 = id) e,
/// quando dados, do clube (+4) e do técnico (+8) recebe por cima o último (5 duplas e um byte), a contagem desce e o
/// mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x4dc970).
pub(crate) unsafe extern "thiscall" fn remove_talks_records(this: usize, person: usize, manager: usize, clube: usize) {
    let n = (this + contract_manager_fields::TALKS_COUNT) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize;
            let r = base.wrapping_add(off);
            let home = read_u32(r) == read_u32(person)
                && (clube == 0 || read_u32(r + 4) == read_u32(clube)) // o clube
                && (manager == 0 || read_u32(r + 8) == read_u32(manager));
            if home {
                let last = base.wrapping_add((*n as usize).wrapping_mul(21)).wrapping_sub(21);
                core::ptr::copy(last as *const u8, r as *mut u8, 21);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(21);
            }
            i += 1;
            off = off.wrapping_add(21);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        qsort(read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize, *n, 0x15, 0x4dc970);
    }
}


/// Comparador: a 0x8674e0 e a 0x867240 mexem nas queixas, nos vínculos e na transferência (não podem rodar duas
/// vezes): só a original; a prova é a carreira e o ramo da cópia no modo Rust.
// simplification: original only (153,222 calls); comparing would require separating morale from the end calls.


/// Address of person *id's 79-byte staff state record (+4 = loan contract index, -1 none).
///
/// [pt] A entrada de 79 bytes da pessoa `*id` na tabela [0xae2d0c] (em +4 a vaga do empréstimo, −1 sem).
pub(crate) unsafe fn loan_state_entry(id: *const i32) -> usize {
    (*(G_STAFF_STATE_TABLE as *const usize) as isize + *id as isize * 79) as usize
}

/// frees the person's loan contract slot, if any
/// Original: `CONTRACT_MANAGER::remove_loan_contract` (0x4d0bd0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0bd0 } (thiscall, ret 4, contract_manager.cpp): libera a vaga do contrato de empréstimo da
/// pessoa, se ela tiver uma: −1 no começo da vaga (80 bytes cada, em [gerente]) e na entrada da pessoa.
pub(crate) unsafe extern "thiscall" fn remove_loan_contract(manager: usize, id: *const i32) {
    let e = loan_state_entry(id);
    let slot = core::ptr::read_unaligned((e + 4) as *const i32);
    if slot != -1 {
        *((*(manager as *const usize) as isize + slot as isize * 80) as *mut i32) = -1;
        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, -1);
    }
}

/// Shortlist entry of club *clube (9 bytes per club in [manager+0xdc]: +0 slots, +4 mark).
///
/// [pt] A lista de observados do clube `*clube` (9 bytes por clube em [gerente+0xdc]: +0 as vagas, +4 a marca).
pub(crate) unsafe fn club_shortlist(manager: usize, clube: *const i32) -> usize {
    (*((manager + 0xdc) as *const usize) as isize + *clube as isize * 9) as usize
}






/// Walks the contract manager's offer list (+0x34) from the start until achou accepts an offer; 0 if none.
///
/// [pt] Percorre a lista do gerente de contratos (+0x34, cursor em +0xa voltando ao começo +2, 0x68d1b0)
/// até a oferta que `achou` aceita; 0 sem lista ou sem nenhuma.
pub(crate) unsafe fn find_manager_offer(manager: usize, found: impl Fn(usize) -> bool) -> usize {
    let list = *((manager + 0x34) as *const usize);
    if list == 0 {
        return 0;
    }
    core::ptr::write_unaligned((list + 0xa) as *mut u32, core::ptr::read_unaligned((list + 2) as *const u32));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    loop {
        let o = next(*((manager + 0x34) as *const usize));
        if o == 0 || found(o) {
            return o;
        }
    }
}

/// the person's contract offer for the club, in the contract manager's list
/// Original: `contract_offer` (0x4cc9e0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc9e0 } (thiscall, ret 8, contract_manager.cpp): a oferta de contrato da pessoa `pessoa`
/// (+0) para o clube `clube` (+4), ou 0.
pub(crate) unsafe extern "thiscall" fn contract_offer(manager: usize, person: i32, clube: i32) -> usize {
    find_manager_offer(manager, |o| *(o as *const i32) == person && *((o + 4) as *const i32) == clube)
}

/// the person's agreed contract offer (state 2)
/// Original: `agreed_contract_offer` (0x4cca30, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cca30 } (thiscall, ret 4, contract_manager.cpp): a oferta acertada (+0x50 = 2) da pessoa, ou 0.
pub(crate) unsafe extern "thiscall" fn agreed_contract_offer(manager: usize, person: i32) -> usize {
    find_manager_offer(manager, |o| *(o as *const i32) == person && *((o + 0x50) as *const u8) == 2)
}


/// Memory regions the offer searches touch: the cursor of the manager's offer list.
///
/// [pt] As regiões que as buscas mexem: o cursor da lista do gerente.
pub(crate) unsafe fn offer_list_regions(manager: usize) -> Vec<(usize, usize)> {
    let list = *((manager + 0x34) as *const usize);
    if list == 0 { Vec::new() } else { vec![(list, 0xe)] }
}




/// Returns dword +0xc of the contract record (0x50 bytes from [this]) of person `*p`, by the index at +0 of the 79-byte
/// record in [0xae2d0c].
///
/// 0 without a person, with an id outside [0xae2364], a negative index or a record at address 0.
/// Original: `contract_value` (0x4d0c20, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d0c20 } (thiscall, ret 4, contract_manager.cpp): o dword +0xc da ficha de contrato (0x50 bytes
/// a partir de [this]) da pessoa `*p`, pelo índice em +0 do registro de 79 bytes em [0xae2d0c]; 0 sem
/// pessoa, com id fora de [0xae2364], índice negativo ou ficha no endereço 0.
pub(crate) unsafe extern "thiscall" fn contract_value(t: usize, p: *const i32) -> u32 {
    if p.is_null() || *p >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((*p as usize).wrapping_mul(79)) as *const i32);
    if k < 0 {
        return 0;
    }
    let sheet = (*(t as *const usize)).wrapping_add(k as usize * 0x50);
    if sheet == 0 {
        return 0;
    }
    core::ptr::read_unaligned((sheet + contract::WAGE) as *const u32)
}

/// `CONTRACT_MANAGER::schedule_offer_decision(id, slot)` (SEH only for the local date, with an empty destructor): finds
/// or creates the person's type 3 decision entry and sets the day and time slice of the answer: by the offer deadline,
/// or without an offer by today and the contract end; at most 14 days ahead.
///
/// The person's type 3 decision entry (find_type3_decision, with the limit of today + 14; without one, a new one at
/// today + 1 + draw(5), slice 2, through 0x4cd6d0). With the offer's slot (0x8c61e0): the answer deadline (0x8cc5b0:
/// days × 3 + slices) against what is left (days until the date × 3 + slices, 0x54cdb0); the smaller, at least 1,
/// becomes the date (today + quotient of ÷3) and the slice (+ remainder; passing 2 it goes to the next day); an offer
/// with +0x2f and fewer than 4 days: today + 4 + draw(2). Without a slot (−1): once the date is reached, it becomes
/// today and the slice goes up (up to the day's [0xae2c88]); if the person's contract ([this] + 80 × index from the
/// 79-byte record) ends before today + 180, today + 4 + draw(4). At the end, the date is no later than today + 14, and
/// +0x1c goes up.
/// Original: `CONTRACT_MANAGER::schedule_offer_decision` (0x4cd230, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd230 } (thiscall, ret 8, contract_manager.cpp, SEH só da data local, de destrutor vazio):
/// `CONTRACT_MANAGER::schedule_offer_decision(id, vaga)`: a entrada de decisão de tipo 3 da pessoa (find_type3_decision, com
/// o limite de hoje + 14; sem ela, uma nova em hoje + 1 + sorteio(5), fatia 2, pela 0x4cd6d0). Com a vaga da oferta
/// (0x8c61e0): o prazo da resposta (0x8cc5b0: dias × 3 + fatias) contra o que falta (dias até a data × 3 + fatias,
/// 0x54cdb0), o menor, no mínimo 1, vira a data (hoje + ÷3) e a fatia (+ resto, passando de 2 vai ao dia seguinte);
/// oferta com +0x2f e menos de 4 dias: hoje + 4 + sorteio(2). Sem vaga (−1): chegada a data, ela vira hoje e a fatia
/// sobe (até a do dia [0xae2c88]); se o contrato da pessoa ([this] + 80 × índice do registro de 79 bytes) acaba antes
/// de hoje + 180, hoje + 4 + sorteio(4). No fim, a data não passa de hoje + 14, e +0x1c sobe.
pub(crate) unsafe extern "thiscall" fn schedule_offer_decision(this: usize, id: i32, slot: i32) {
    let mut limit = [0u8; 8];
    injury_date_add_days(G_CURRENT_DATE, 0xe, &mut limit);
    let mut e = find_type3_decision(this, id, &limit, true);
    let person = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if e == 0 {
        crate::comparison::branch("cpr_10 nova decisao");
        let mut d = [0u8; 8];
        injury_date_add_days(G_CURRENT_DATE, (sortear(5) + 1) as u32, &mut d); // cpr10: a decisão em 1 a 5 dias
        let append: unsafe extern "thiscall" fn(usize, usize, *const u8, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_DECISION_DATE);
        append(this, person, d.as_ptr(), 2, 3);
        let n = read_u32_as_usize(this + contract_manager_fields::DECISION_COUNT);
        e = read_u32_as_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_sub(1).wrapping_mul(14));
    }
    let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let data = e + 4;
    if slot != -1 {
        let slot_of: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
        let of = slot_of(read_u32_as_usize(G_TRANSFER_MANAGER), slot);
        // simplification: the original passes the slots of both arguments as output; they start with their low bytes
        let (mut a, mut b) = (id as u8, slot as u8);
        let prazo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_RESPONSE_DELAY);
        prazo(read_u32_as_usize(G_TRANSFER_MANAGER), read_u32_as_usize(of), &mut a, &mut b);
        let target = a.wrapping_mul(3).wrapping_add(b) as i8;
        let foul = days(data, read_u32_as_usize(G_CURRENT_DATE) as u32, read_u32_as_usize(G_CURRENT_DATE_LEAP) as u32);
        let mut f = (*((e + 0xc) as *const u8)).wrapping_sub(*(G_CURRENT_DAY_SLICE as *const u8)) as i8;
        let mut dl = foul as u8;
        if f < 0 {
            f = f.wrapping_add(3);
            dl = dl.wrapping_sub(1);
        }
        let current = dl.wrapping_mul(3).wrapping_add(f as u8) as i8;
        let x = if current < target { current.max(1) } else { target.max(1) } as i32;
        set_today_plus_days(data, (x / 3) as u32);
        advance_time_slice(e, ((x % 3) as u8).wrapping_add(*(G_CURRENT_DAY_SLICE as *const u8)) as i8);
        if *((read_u32_as_usize(of) + 0x2f) as *const u8) != 0 && days(data, read_u32_as_usize(G_CURRENT_DATE) as u32, read_u32_as_usize(G_CURRENT_DATE_LEAP) as u32) < 4 {
            set_today_plus_days(data, (sortear(2) + 4) as u32);
        }
    } else {
        let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
        let (rd, ra) = (read_i16(data), read_u16(data + 2));
        if ta > ra || (ta == ra && td >= rd) {
            injury_assign_date(data, G_CURRENT_DATE);
            let f = *((e + 0xc) as *const i8);
            if f as i32 <= *(G_CURRENT_DAY_SLICE as *const u8) as i32 {
                advance_time_slice(e, f.wrapping_add(1));
            }
        }
        let p = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        if p != 0 && (read_u32_as_usize(p) as i32) < read_u32_as_usize(G_STAFF_COUNT) as i32 {
            let x = read_u32_as_usize(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(p).wrapping_mul(79))) as i32;
            let c = if x >= 0 { read_u32_as_usize(this).wrapping_add((x as usize).wrapping_mul(0x50)) } else { 0 };
            if c != 0 {
                let mut t = [0u8; 8];
                let m = injury_date_add_days(G_CURRENT_DATE, 0xb4, &mut t);
                let (ma, ca) = (read_u16(m + 2), read_u16(c + 0x2f));
                if ma > ca || (ma == ca && read_i16(m) > read_i16(c + contract::CONTRACT_EXPIRES)) {
                    set_today_plus_days(data, (sortear(4) + 4) as u32);
                }
            }
        }
    }
    let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let minus: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut t = [0u8; 8];
    let m = minus(data, t.as_mut_ptr(), 0xe);
    let ma = read_u16(m + 2);
    if ta < ma || (ta == ma && td < read_i16(m)) {
        set_today_plus_days(data, 0xe);
    }
    core::ptr::write_unaligned((this + contract_manager_fields::DECISIONS_ADDED) as *mut u32, read_u32_as_usize(this + contract_manager_fields::DECISIONS_ADDED).wrapping_add(1) as u32);
}


/// the person's contract record (0x50 bytes)
/// Original: `contract_record` (0x4cf6d0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cf6d0 } (thiscall, ret 4, contract_manager.cpp): a ficha de contrato (0x50 bytes, a partir de
/// [this]) da pessoa `*p`, pelo índice em +4 do registro de 79 bytes em [0xae2d0c]; 0 sem pessoa, com
/// id fora de [0xae2364] ou índice negativo.
pub(crate) unsafe extern "thiscall" fn contract_record(t: usize, p: *const i32) -> usize {
    if p.is_null() {
        return 0;
    }
    let id = *p;
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79)).wrapping_add(4) as *const i32);
    if k < 0 {
        return 0;
    }
    (*(t as *const usize)).wrapping_add(k as usize * 0x50)
}

/// Returns the category 1..8 of player `p` in contract `c`.
///
/// Without a player sheet (+0x61), 7; without a contract, 4; a type 5 contract (+0x35 & 0x3f), 8; otherwise by the
/// squad status (+0x4f >> 4, 0..6; above, 7), the happiness (+0x3d, signed), the age (+0x18), the ambition (+0x57) and
/// the sheet's reputation (+0xb, +0xd).
/// Original: `contract_category` (0x4ca470, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4ca470 } (stdcall, ret 8, contract_manager.cpp): a categoria 1..8 do jogador `p` no contrato
/// `c`: sem ficha (+0x61), 7; sem contrato, 4; contrato de tipo 5 (+0x35 & 0x3f), 8; senão pela
/// situação (+0x4f >> 4, 0..6; acima, 7), a satisfação (+0x3d, com sinal), a idade (+0x18), a
/// ambição (+0x57) e a reputação da ficha (+0xb, +0xd).
pub(crate) unsafe extern "stdcall" fn contract_category(p: usize, c: usize) -> u8 {
    if p == 0 {
        return 7;
    }
    let sheet = core::ptr::read_unaligned((p + staff::PLAYER) as *const usize);
    if sheet == 0 {
        return 7;
    }
    if c == 0 {
        return 4;
    }
    if *((c + contract::CONTRACT_TYPE) as *const u8) & 0x3f == 5 {
        return 8;
    }
    let k = *((c + contract::SQUAD_STATUS) as *const u8) >> 4;
    let v = *((c + 0x3d) as *const i8);
    let age = *((p + staff::YEAR_OF_BIRTH) as *const i8);
    let ambicao = *((p + staff::AMBITION) as *const i8);
    let rep = |o: usize| core::ptr::read_unaligned((sheet + o) as *const i16);
    let low = |limit: i8| if v <= limit { 7 } else { 4 };
    match k {
        0 => {
            if age > 0x15 {
                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }
            } else if v > 0 {
                5
            } else if v > -30 || ambicao > 5 {
                6
            } else {
                7
            }
        }
        1 => if v > 0x2d { 1 } else { (v <= 0x19) as u8 + 2 },
        2 => {
            let rating = ambicao as i32 * 25 - *((p + staff::LOYALITY) as *const i8) as i32 * 10 + rep(0xd) as i32;
            if v > 0x4b && rating > 0x186a && age > 0x17 && age < 0x22 {
                1
            } else if v > 0x19 {
                2
            } else {
                (v <= 0) as u8 + 3
            }
        }
        3 => if v > 0x37 && age > 0x17 && age < 0x22 { 2 } else { (v <= 0) as u8 + 3 },
        4 => {
            if v > 0x41 && age > 0x17 {
                (age < 0x22) as u8 + 2
            } else if v > -30 {
                4
            } else if age >= 0x18 {
                7
            } else {
                5
            }
        }
        5 => {
            if age > 0x18 {
                if age > 0x22 { 4 } else if v > 0x4b { 2 } else if v > 0 { 3 } else { low(-30) }
            } else if ambicao > 5 && rep(0xb) > 0x4e2 {
                5
            } else {
                6
            }
        }
        6 => {
            if age > 0x17 {
                if v > 0x4b { 1 } else if v > 0x1e { 2 } else if v > 0 { 3 } else { low(-50) }
            } else if rep(0xb) > 0x4e2 && ambicao > 0xf {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}


/// `CONTRACT_MANAGER::post_transfer_listed_news(player, club)` (SEH frame for the local list), called by the transfer
/// request (0x4d8610), the player put up for sale (0x842320), the news (0x784100) and 0x4dc3a0: posts news item 0xbea
/// of a player on the transfer list, with the price and the strongest interested clubs, to the interested human clubs.
///
/// Without a player, a player sheet (+0x61), a club or the staff (+0xcf): nothing. Without a contract with the club
/// (0x4cf5f0): nothing. The news item 0xbea (local): 0 the club, 1 the player, 2..4 his +0x33 of +4, +8 and +0xc, 5 the
/// staff, 6..8 its +0x33, 9 the contract +0x45, 10 the high nibble of +0x4f, 11 the positions mask (0x5b7d20), 12 a
/// draw of 2 (0x90cfc0).
///
/// The price: the value (+0x52) with a human club, when the player is not on the club's list (0x5444a0) or the contract
/// has nibble 1; otherwise the asking price (0x4d1600) × (draw of 0x29 × [0x967898] + [0x9689b0]) truncated, rounded
/// down to 10 thousand (between 10 thousand and 200 thousand, exclusive), 100 thousand (below 1 million), 250 thousand
/// (below 5 million) or 1 million; 13 the price. From 500 thousand up: the interested clubs (0x842180), up to 30, the
/// computer ones and the human ones with a budget (0x598800) ≥ price × [0x9689c0]; sorted by reputation (+0x80); 14 =
/// up to 3; 15.. the ids, with 1 (in place 13) if the 2nd repeats the 1st and 2 (in 12) if the 3rd repeats one of them;
/// finally the "off the list" in the next place. The recipients (0x8d1da0 with 0x21a, up to 30, with [0xde1f54] and the
/// sheet) and the delivery (0x7861e0).
/// Original: `CONTRACT_MANAGER::post_transfer_listed_news` (0x4d71b0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4d71b0 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da lista local): `CONTRACT_MANAGER::
/// post_transfer_listed_news(jogador, clube)`, pelo pedido de transferência (0x4d8610), pelo jogador posto à venda
/// (0x842320), pela notícia (0x784100) e por 0x4dc3a0. Sem jogador, sem ficha (+0x61), sem clube ou sem a comissão
/// (+0xcf): nada. Sem contrato com o clube (0x4cf5f0): nada. A notícia 0xbea (local): 0 o clube, 1 o jogador, 2..4 os
/// +0x33 de +4, +8 e +0xc dele, 5 a comissão, 6..8 os +0x33 dela, 9 o contrato +0x45, 10 o nibble alto de +0x4f, 11 a
/// máscara de posições (0x5b7d20), 12 um sorteio de 2 (0x90cfc0). O preço: o valor (+0x52) com clube humano, quando o
/// jogador não está na lista do clube (0x5444a0) ou o contrato tem o nibble 1; senão o pedido (0x4d1600) × (sorteio de
/// 0x29 × [0x967898] + [0x9689b0]) truncado, arredondado para baixo em 10 mil (entre 10 mil e 200 mil, exclusive), 100 mil
/// (abaixo de 1 milhão), 250 mil (abaixo de 5 milhões) ou 1 milhão; 13 o preço. De 500 mil para cima: os interessados
/// (0x842180), até 30, os do computador e os humanos com verba (0x598800) ≥ preço × [0x9689c0]; ordenados pela reputação
/// (+0x80); 14 = até 3; 15.. os ids, com 1 (no lugar 13) se o 2º repete o 1º e 2 (no 12) se o 3º repete um deles; por
/// fim o "fora da lista" no lugar seguinte. Os destinatários (0x8d1da0 com 0x21a, até 30, com [0xde1f54] e ficha) e a
/// entrega (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_transfer_listed_news(this: usize, player: usize, clube: usize) {
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let list_end: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mask: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let request: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let interested: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let budget: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    if player == 0 || read_u32_as_usize(player + staff::PLAYER) == 0 || clube == 0 || read_u32_as_usize(clube + club::MANAGER) == 0 {
        return;
    }
    let c = contract_with(this, player, clube);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    list_new(l);
    if c != 0 {
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construct(n, 0xbea, 0);
        write(n, 0, read_u32_as_usize(clube) as u32);
        write(n, 1, read_u32_as_usize(player) as u32);
        write(n, 2, read_u32_as_usize(read_u32_as_usize(player + staff::FIRST_NAME) + 0x33) as u32);
        write(n, 3, read_u32_as_usize(read_u32_as_usize(player + staff::SECOND_NAME) + 0x33) as u32);
        write(n, 4, read_u32_as_usize(read_u32_as_usize(player + staff::COMMON_NAME) + 0x33) as u32);
        let com = read_u32_as_usize(clube + club::MANAGER);
        write(n, 5, read_u32_as_usize(com) as u32);
        write(n, 6, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER) + 4) + 0x33) as u32);
        write(n, 7, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER) + 8) + 0x33) as u32);
        write(n, 8, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER) + 0xc) + 0x33) as u32);
        write(n, 9, read_u32_as_usize(c + 0x45) as u32);
        write(n, 10, (read_u8(c + 0x4f) >> 4) as u32);
        write(n, 11, mask(read_u32_as_usize(player + staff::PLAYER)) as i32 as u32);
        crate::comparison::branch("cqf_2 notícia do listado");
        let s = sorteio(2); // cqf2: o sorteio do dado 12
        write(n, 12, s as u32);
        let away_of_list = (listado(player, clube) == 0) as u32;
        let price = if human(clube, 0, 1, 0, 0) != 0 && (away_of_list == 0 || read_u8(c + 0x4f) & 0xf0 == 0x10) {
            read_u32_as_usize(player + staff::VALUE) as i32
        } else {
            let p = request(this, clube, 0, player, 0, 1, 1);
            let r = sorteio(0x29);
            let v = match_pl_ftol((r as f64 * read_f64_at(0x967898) + read_f64_at(0x9689b0)) * p as f64); // cqf2: o preço da notícia; 0x967898 = 0.01, 0x9689b0 = 0.8
            if v > 0x2710 && v < 0x30d40 {
                v / 10000 * 10000
            } else if v < 1_000_000 {
                v / 100000 * 100000
            } else if v < 5_000_000 {
                v / 250000 * 250000
            } else {
                v / 1_000_000 * 1_000_000
            }
        };
        write(n, 0xd, price as u32);
        if price >= 500_000 {
            crate::comparison::branch("cqf_2 interessados no listado");
            let mut clubs = [0usize; 30];
            let mut q: i8 = 0;
            interested(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), player, l, 1, 1, 0);
            write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
            let mut no = next(l);
            while no != 0 && q < 0x1e {
                let id = read_u32_as_usize(no + 4);
                let can = if human(transfer_manager_club_by_id(id), 0, 1, 0, 0) == 0 {
                    true
                } else {
                    let f = finances(read_u32_as_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_u32_as_usize(no + 4)));
                    f != 0 && price as f64 * read_f64_at(0x9689c0) <= budget(f, 0, 0, 0, 1, 0) as f64 // 0x9689c0 = 1.1
                };
                if can && read_u32_as_usize(no + 4) as i32 > -1 {
                    clubs[q as usize] = transfer_manager_club_by_id(read_u32_as_usize(no + 4));
                    q += 1;
                }
                no = next(l);
            }
            let total = q as i32;
            let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
            let mut i = 0i32;
            while i < total - 1 {
                let mut j = i + 1;
                while j < total {
                    if rep(clubs[i as usize]) < rep(clubs[j as usize]) {
                        clubs.swap(i as usize, j as usize);
                    }
                    j += 1;
                }
                i += 1;
            }
            let m = if q >= 3 { 3 } else { total };
            write(n, 0xe, m as u32);
            let mut place: i8 = 0xf;
            let mut k: i8 = 0;
            let end = loop {
                if k as i32 >= m {
                    break place;
                }
                if k == 1 && clubs[1] == clubs[0] {
                    write(n, place - 2, 1);
                    break place;
                }
                if k == 2 && (clubs[2] == clubs[0] || clubs[2] == clubs[1]) {
                    write(n, place - 3, 2);
                    break place;
                }
                write(n, place, read_u32_as_usize(clubs[k as usize]) as u32);
                place += 1;
                k += 1;
            };
            // simplification: the place goes in a dword with the high bytes of the price in the original; 0x785790 reads the byte
            write(n, end, away_of_list);
            let mut dest = clubs;
            let qd = if read_u32_as_usize(G_TRANSFER_MANAGER) != 0 && read_u32_as_usize(player + staff::PLAYER) != 0 {
                recipients(read_u32_as_usize(G_TRANSFER_MANAGER), player, 0x21a, dest.as_mut_ptr(), 0x1e) as i8
            } else {
                0
            };
            for &d in dest.iter().take(if qd > 0 { qd as usize } else { 0 }) {
                deliver(read_u32_as_usize(G_NEWS), n, d);
            }
        }
    }
    list_end(l);
}


/// `CONTRACT_MANAGER::schedule_loan_end(person, club)`, only from the transfers screen (0x8fbcc0): the person's loan
/// contract ends tomorrow (type 7) and, with a human manager at the relevant club, the early return.
///
/// Without a person, with an id outside the total or without the loan contract (+4 of the 79-byte record, in this's
/// vector): the box (line 0xe4a). Otherwise the end (+0x2d) becomes tomorrow (0x54c5b0 with 1; 0x417c10) and the type
/// (+0x35) becomes 7 (bits 3..5 zeroed). The current contract (+0 of the record, in the vector of [0xae1050]) in
/// [0xae1080]: from `club`, the person's club (+0x39) with a human manager (0x7875e0, stdcall: ecx = [0xb7431c] is not
/// read); from another, the current contract's club (read again twice) with a human manager: the return (0x4db0b0: push
/// club; push person; mov ecx, this).
/// Original: `CONTRACT_MANAGER::schedule_loan_end` (0x4ccca0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4ccca0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::schedule_loan_end(pessoa, clube)`, só pela
/// tela de transferências (0x8fbcc0). Sem pessoa, id fora do total ou sem o contrato de empréstimo (+4 do registro de 79
/// bytes, no vetor de this): a caixa (linha 0xe4a). Senão o fim (+0x2d) vira amanhã (0x54c5b0 com 1; 0x417c10) e o tipo
/// (+0x35) vira 7 (os bits 3..5 zerados). O contrato atual (+0 do registro, no vetor de [0xae1050]) em [0xae1080]: do
/// `clube`, o clube da pessoa (+0x39) com técnico humano (0x7875e0, stdcall: o ecx = [0xb7431c] não é lido); de outro,
/// o clube do contrato atual (lido de novo duas vezes) com técnico humano: a volta (0x4db0b0: push clube; push pessoa;
/// mov ecx, this).
pub(crate) unsafe extern "thiscall" fn schedule_loan_end(this: usize, person: usize, clube: usize) {
    let more: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let voltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_LOAN_RECALL_NEWS);
    let c = if person == 0 { 0 } else { staff_contract_by_field(person, 4, this) };
    if c == 0 {
        show_error_box(0x9a8190, 0xe4a);
        return;
    }
    let mut amanha = [0u32; 2];
    let p = more(G_CURRENT_DATE, &mut amanha, 1);
    let mut tmp = [0u8; 8];
    copy(c + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr() as usize, p);
    write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc7) | 7);
    let a = staff_contract_by_field(person, 0, read_u32_as_usize(G_CONTRACT_MANAGER));
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, a);
    let current = if a == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(a + 4)) };
    if clube == current {
        let x = read_u32_as_usize(person + staff::CLUB_JOB);
        if x == 0 || human_manager_record_checked(x) == 0 {
            return;
        }
    } else {
        let a = staff_contract_by_field(person, 0, read_u32_as_usize(G_CONTRACT_MANAGER));
        write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, a);
        if a == 0 || transfer_manager_club_by_id(read_u32_as_usize(a + 4)) == 0 {
            return;
        }
        let a = staff_contract_by_field(person, 0, read_u32_as_usize(G_CONTRACT_MANAGER));
        write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, a);
        let x = if a == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(a + 4)) };
        if human_manager_record_checked(x) == 0 {
            return;
        }
    }
    voltar(this, person, clube);
}


/// `CONTRACT_MANAGER::post_contract_end_news(person, type, wide)` (SEH frame for the local LLIST and news item), called
/// by the end of a contract, trial or loan (0x4c7ac0): posts news item 0xbb9 to the club, the interested clubs or, for
/// famous players, the nation's managers or everyone.
///
/// Without a contract or without its club: nothing. The news item 0xbb9: the contract club, the person (the id and the
/// three keys), the type, the contract club for type 6 (otherwise −1) and `wide`. Wide, with a player sheet: world
/// reputation (+0xd) above 7000, to all managers (0x786780); national (+0xb) above 7000, to the managers of the
/// person's nation, of the club's and of the current club's (+0x39) (0x7863f0) and to the interested clubs (0x842180)
/// of other nations. Otherwise to the club, to the current club for type 6 and to the interested clubs (0x7861e0).
/// Original: `post_contract_end_news` (0x4da590, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4da590 } (thiscall, ret 0xc, contract_manager.cpp, quadro SEH da LLIST e da notícia locais): `CONTRACT_MANAGER::
/// post_contract_end_news(pessoa, tipo, amplo)`, pelo fim de contrato, de teste ou de empréstimo (0x4c7ac0). Sem
/// contrato ou sem o clube dele: nada. A notícia 0xbb9: o clube do contrato, a pessoa (o id e as três chaves), o tipo, o
/// clube do contrato no tipo 6 (senão −1) e `amplo`. Ampla, com ficha: reputação mundial (+0xd) acima de 7000, a todos
/// os técnicos (0x786780); nacional (+0xb) acima de 7000, aos técnicos da nação da pessoa, da do clube e da do clube
/// atual (+0x39) (0x7863f0) e aos clubes interessados (0x842180) de outras nações. Senão ao clube, ao clube atual no
/// tipo 6 e aos interessados (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_contract_end_news(this: usize, p: usize, kind: u32, amplo: u32) {
    let c = contract_inline_lookup(p);
    if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        return;
    }
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    let all: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    let mut list = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = list.as_mut_ptr() as usize;
    new(l);
    crate::comparison::branch("cpx_4 noticia do fim do contrato");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xbb9, 0);
    let c = contract_inline_lookup(p);
    // the original reads the id at address 0 without a contract: does not happen, it has just found it
    let clube = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    write(nt, 0, read_usize(clube) as u32);
    write(nt, 2, read_usize(p) as u32);
    write(nt, 3, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, 4, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, 5, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    let kind = kind as u8;
    write(nt, 6, kind as i8 as u32);
    let c = contract_inline_lookup(p);
    let clube = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    if kind == 6 {
        // the contract’s club from the manager’s own table (+0), without storing it in [0xae1080]; with no contract, the original
        // reads address 4 (does not happen: it has just found it)
        let k = if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            -1
        } else {
            read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f))) as i32
        };
        let v = if k < 0 { read_usize(4) } else { read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(0x50)) + 4) };
        write(nt, 7, v as u32);
    } else {
        write(nt, 7, u32::MAX);
    }
    write(nt, 8, amplo);
    let sheet = read_usize(p + staff::PLAYER);
    let current = read_usize(p + staff::CLUB_JOB);
    let nacao = read_usize(p + staff::NATION);
    let jornal = read_usize(G_NEWS);
    let mut default = true;
    if amplo != 0 && sheet != 0 {
        if read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x1b58 {
            all(jornal, nt);
            default = false;
        } else if read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0x1b58 {
            default = false;
            do_pais(jornal, nt, nacao);
            if nacao != read_usize(clube + club::NATION) {
                do_pais(jornal, nt, read_usize(clube + club::NATION));
            }
            if current != 0 && current != clube {
                let n = read_usize(current + club::NATION);
                if n != read_usize(clube + club::NATION) && n != nacao {
                    do_pais(jornal, nt, n);
                }
            }
            for id in interested_club_ids(p, l) {
                if id == read_usize(clube) || (kind == 6 && id == read_usize(read_usize(p + staff::CLUB_JOB))) {
                    continue;
                }
                let other = friendly_club_by_id(id);
                let n = read_usize(other + club::NATION);
                if n != read_usize(p + staff::NATION) && n != read_usize(read_usize(p + staff::CLUB_JOB) + 0x53) && n != read_usize(clube + club::NATION) {
                    deliver(read_usize(G_NEWS), nt, other);
                }
            }
        }
    }
    if default {
        deliver(jornal, nt, clube);
        if kind == 6 {
            deliver(read_usize(G_NEWS), nt, read_usize(p + staff::CLUB_JOB));
        }
        for id in interested_club_ids(p, l) {
            if id == read_usize(clube) || (kind == 6 && id == read_usize(read_usize(p + staff::CLUB_JOB))) {
                continue;
            }
            deliver(read_usize(G_NEWS), nt, friendly_club_by_id(id));
        }
    }
    soltar(l);
}



