use super::*;

/// Finds the person's contract with a club in the contract list (`this`, 80-byte records): the person has up to two
/// contract indices in the staff state table (79 bytes per person, +0 and +4) and the one whose club id (+4) matches
/// wins; if neither matches, retries with the linked B team when the club is the A team. Returns the contract record or
/// 0.
///
/// [pt] { 0x4cf5f0 } (ret 8): o contrato (registros de 80 bytes em [this]) da pessoa com o clube. A
/// pessoa tem ate dois indices (tabela 0xae2d0c, 79 bytes por pessoa, +0 e +4); vale o registro
/// cujo +4 e o id do clube. Sem nenhum, tenta o clube parceiro (0x540a50) quando ele e o time A.
pub(crate) unsafe extern "thiscall" fn contract_with_club(this: usize, pessoa: usize, clube: usize) -> usize {
    if pessoa == 0 || clube == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
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
    let id_clube = *(clube as *const u32);
    if id_de(r0) == id_clube {
        return r0;
    }
    let i1 = *((e + 4) as *const i32);
    let mut r1 = 0usize;
    if i1 >= 0 {
        r1 = base.wrapping_add((i1 as usize).wrapping_mul(80));
        if r1 != 0 && id_de(r1) == id_clube {
            return r1;
        }
    }
    let mut alvo = clube;
    let mut eh_a = pessoa as u32; // o original reaproveita a vaga do argumento
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let pc = parceiro(clube, &mut eh_a, 1);
    if pc != 0 && eh_a == 0 {
        alvo = pc;
    }
    let id_alvo = *(alvo as *const u32);
    if id_de(r0) == id_alvo {
        return r0;
    }
    if r1 != 0 && id_de(r1) == id_alvo {
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
pub(crate) unsafe extern "C" fn contract_level(pessoa: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(2 * *(G_NATION_COUNT as *const i32));
    if *(clube as *const i32) >= limit || *(pessoa as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return clube as u32 & !0xff;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let r = f(*(G_CONTRACT_MANAGER as *const usize), pessoa, clube);
    if r == 0 {
        return 0;
    }
    ((*((r + 0x4f) as *const u8) & 0xf) as u32).wrapping_sub(10)
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
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    for k in (0..0x1c).step_by(4) {
        write_u32_from_usize(d + k, read_u32_as_usize(o + k));
    }
    for k in 0x1c..0x21 {
        write_u8(d + k, read_u8(o + k));
    }
    write_u32_from_usize(d + 0x21, read_u32_as_usize(o + 0x21));
    let mut tmp = [0u8; 8];
    copiar_data(d + 0x25, tmp.as_mut_ptr(), o + 0x25);
    copiar_data(d + 0x2d, tmp.as_mut_ptr(), o + 0x2d);
    let s = read_u8(o + 0x35);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ read_u8(d + 0x35)) & 0x3f) ^ read_u8(d + 0x35);
    let al = ((((s << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl;
    write_u8(d + 0x35, ((al ^ s) & 0x7f) ^ s);
    write_u32_from_usize(d + 0x36, read_u32_as_usize(o + 0x36));
    write_u8(d + 0x3a, read_u8(o + 0x3a));
    write_u16(d + 0x3b, read_u16(o + 0x3b));
    for k in [0x3dusize, 0x3e, 0x3f] {
        write_u8(d + k, read_u8(o + k));
    }
    write_u32_from_usize(d + 0x40, read_u32_as_usize(o + 0x40));
    write_u8(d + 0x44, read_u8(o + 0x44));
    write_u32_from_usize(d + 0x45, read_u32_as_usize(o + 0x45));
    write_u8(d + 0x49, read_u8(o + 0x49));
    write_u32_from_usize(d + 0x4a, read_u32_as_usize(o + 0x4a));
    write_u8(d + 0x4e, read_u8(o + 0x4e));
    let s = read_u8(o + 0x4f);
    let a = ((read_u8(d + 0x4f) ^ s) & 0xf) ^ read_u8(d + 0x4f);
    write_u8(d + 0x4f, ((a ^ s) & 0xf) ^ s);
}

/// Sets the contract type byte (+0x3d) from the job nibble (+0x4f >> 4): 1→75, 2→50, 3→40, 4/5/6/8→0, others untouched.
///
/// [pt] O tipo do contrato (+0x3d) pelo cargo (nibble alto de +0x4f): 1 → 75, 2 → 50, 3 → 40, 4, 5, 6 e 8 → 0; os outros
/// não mexem (a tabela de saltos da original).
pub(crate) unsafe fn set_contract_type_from_job(c: usize) {
    match read_u8(c + 0x4f) >> 4 {
        1 => write_u8(c + 0x3d, 0x4b),
        2 => write_u8(c + 0x3d, 0x32),
        3 => write_u8(c + 0x3d, 0x28),
        4 | 5 | 6 | 8 => write_u8(c + 0x3d, 0),
        _ => {}
    }
}

/// puts the person in the contract club (or cancels without space), handles loans (full squad, job 11, length from
/// today), ensures 4 weeks of contract, stores the contract in the array (the old index, a free slot or the end,
/// growing by 100), the value at person +0x4e, the payroll and the club change
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
pub(crate) unsafe extern "thiscall" fn sign_contract(this: usize, contrato: usize, cargo: u32) -> u32 {
    let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
    let tem_vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    let noticia_sem_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_OFFER_CANCELLED_NO_SPACE_NEWS);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let olheiros: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
    let f8b2880: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRAINING_MANAGER_UPDATE_COACH_ASSIGNMENTS);
    let elenco: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let noticia_teste: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRIAL_CANCELLED_NEWS);
    let f90d860: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_REMOVE_VIRTUAL_PLAYERS);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, i32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let marcar_elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let zerar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_CONTRACT_RESET_MORALE_STATE);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    let c = contrato;
    let clube = transfer_manager_club_by_id(read_u32_as_usize(c + 4));
    let pessoa = read_u32_as_usize(G_STAFF_TABLE).wrapping_add(read_u32_as_usize(c).wrapping_mul(staff::SIZE));
    let emprestimo = || read_u8(c + 0x35) & 0x3f == 5;
    if read_u32_as_usize(pessoa + staff::CLUB_JOB) == 0 && !emprestimo() && incluir(clube, pessoa, cargo, G_CURRENT_DATE, 1, 0) == 0 {
        return 0;
    }
    if cargo as u8 != read_u8(pessoa + staff::JOB_FOR_CLUB) && !emprestimo() && tem_vaga(read_u32_as_usize(pessoa + staff::CLUB_JOB), cargo, pessoa) == 0 {
        noticia_sem_vaga(this, pessoa, read_u32_as_usize(pessoa + staff::CLUB_JOB));
        return 0;
    }
    if read_u32_as_usize(pessoa + staff::CLUB_JOB) != 0 && humano(read_u32_as_usize(pessoa + staff::CLUB_JOB), 1, 1, 0, 0) != 0 {
        olheiros(read_u32_as_usize(G_SCOUT_MANAGER), read_u32_as_usize(read_u32_as_usize(pessoa + staff::CLUB_JOB) + 0xcf));
        f8b2880(read_u32_as_usize(G_TRAINING_MANAGER), read_u32_as_usize(pessoa + staff::CLUB_JOB), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if emprestimo() {
        crate::comparacao::ramo("cqg_1 empréstimo");
        if (elenco(clube, 0, 0) as i32) + 1 > 0x32 {
            noticia_teste(this, pessoa, clube);
            return 0;
        }
        f90d860(read_u32_as_usize(G_VIRTUAL_STAFF), transfer_manager_club_by_id(read_u32_as_usize(c + 4)), 0);
        incluir(transfer_manager_club_by_id(read_u32_as_usize(c + 4)), pessoa, 0xb, G_CURRENT_DATE, 1, 0);
        let n = dias_ate(c + 0x2d, read_u32_as_usize(c + 0x25) as u32, read_u32_as_usize(c + 0x29) as u32);
        let fim = mais(G_CURRENT_DATE, tmp2.as_mut_ptr(), n); // cqg1: o fim pela duração
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), fim);
        write_u8(c + 0x4f, (read_u8(c + 0x4f) & 0xf) | 0x80);
        copiar_data(c + 0x25, tmp.as_mut_ptr(), G_CURRENT_DATE);
        marcar_elenco(pessoa, 1);
    }
    // o fim no mínimo 4 semanas depois do início
    let (ini_dia, ini_ano) = (read_i16(c + 0x25), read_u16(c + 0x27));
    let fim_ano = read_u16(c + 0x2f);
    if ini_ano > fim_ano || (ini_ano == fim_ano && !(ini_dia < read_i16(c + 0x2d))) {
        crate::comparacao::ramo("cqg_1 fim em 4 semanas");
        let p = mais_semanas(c + 0x25, tmp2.as_mut_ptr(), 4); // cqg1: as 4 semanas
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), p);
    }
    copiar_data(pessoa + staff::DATE_EXPIRES_CLUB, tmp.as_mut_ptr(), c + 0x2d);
    let idx = read_u32_as_usize(contract_manager_staff_state_record(read_u32_as_usize(c))) as i32;
    if idx > -1 {
        let off = (idx as usize).wrapping_mul(0x50);
        let r = read_u32_as_usize(this).wrapping_add(off);
        if read_u32_as_usize(r + 4) == read_u32_as_usize(c + 4) { // cqg1: o mesmo clube
            crate::comparacao::ramo("cqg_1 mesmo clube");
            let r = || read_u32_as_usize(this).wrapping_add(off);
            write_u8(c + 0x3a, read_u8(r() + 0x3a));
            for k in [0x3dusize, 0x3e, 0x3f, 0x44] {
                write_u8(c + k, read_u8(r() + k));
            }
            write_u32_from_usize(c + 0x45, read_u32_as_usize(r() + 0x45));
            write_u8(c + 0x4e, read_u8(r() + 0x4e));
            write_u8(c + 0x49, 0);
            write_u32_from_usize(c + 0x4a, u32::MAX as usize);
            let a = read_u8(c + 0x4f);
            write_u8(c + 0x4f, ((read_u8(r() + 0x4f) ^ a) & 0xf) ^ a);
            let a = read_u8(c + 0x35);
            write_u8(c + 0x35, ((((read_u8(r() + 0x35) << 1) as i8 >> 1) as u8 ^ a) & 0x40) ^ a);
            write_u16(c + 0x3b, read_u16(r() + 0x3b));
            let nacao = read_u32_as_usize(clube + club::NATION);
            if nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(KEY_NATION_PORTUGAL) && comunitaria(pessoa, 0) == 0 {
                let p = mais_dias(G_CURRENT_DATE, tmp2.as_mut_ptr(), 0x64);
                let (dia, ano) = (read_i16(p), read_u16(p + 2));
                let depois = ano > read_u16(pessoa + 0x40) || (ano == read_u16(pessoa + 0x40) && dia > read_i16(pessoa + staff::DATE_JOINED_CLUB));
                if depois {
                    let primeira = read_u32_as_usize(pessoa + staff::NATION);
                    if (primeira != 0 && read_u32_as_usize(primeira) == read_u32_as_usize(KEY_NATION_BRAZIL)) || sorteio(3) == 0 {
                        write_u32_from_usize(pessoa + staff::SECOND_NATION, read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_PORTUGAL).wrapping_mul(nation::SIZE)));
                    }
                }
            }
        } else {
            set_contract_type_from_job(c);
            write_u32_from_usize(c + 0x4a, u32::MAX as usize);
            write_u8(c + 0x49, 0);
            zerar(c, pessoa, transfer_manager_club_by_id(read_u32_as_usize(c + 4)));
            write_u16(c + 0x3b, 0);
        }
        copy_contract(read_u32_as_usize(this).wrapping_add(off), c);
    } else {
        let conta = read_u32_as_usize(this + 8) as i32;
        let mut i = 0i32;
        while i < conta {
            let r = read_u32_as_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if read_u32_as_usize(r) as i32 == -1 { // cqg1: a vaga livre
                crate::comparacao::ramo("cqg_1 vaga livre");
                set_contract_type_from_job(c);
                write_u8(c + 0x49, 0);
                write_u32_from_usize(c + 0x4a, u32::MAX as usize);
                zerar(c, pessoa, transfer_manager_club_by_id(read_u32_as_usize(c + 4)));
                copy_contract(r, c);
                write_u32_from_usize(contract_manager_staff_state_record(read_u32_as_usize(r)), i as u32 as usize);
                let f = financas(read_u32_as_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_u32_as_usize(c + 4)));
                if f != 0 {
                    folha(f);
                }
                return 1;
            }
            i += 1;
        }
        if conta >= read_u32_as_usize(this + 4) as i32 {
            let cap = (read_u32_as_usize(this + 4) as u32).wrapping_add(0x64);
            write_u32_from_usize(this + 4, cap as usize);
            let p = if read_u32_as_usize(this) == 0 { malloc(cap.wrapping_mul(0x50)) } else { realocar(read_u32_as_usize(this), cap.wrapping_mul(0x50)) };
            write_u32_from_usize(this, p);
            if p == 0 {
                show_error_at(0x9a8190, 0x1637);
            }
        }
        set_contract_type_from_job(c);
        write_u8(c + 0x49, 0);
        write_u32_from_usize(c + 0x4a, u32::MAX as usize);
        zerar(c, pessoa, transfer_manager_club_by_id(read_u32_as_usize(c + 4)));
        write_u16(c + 0x3b, 0);
        write_u32_from_usize(contract_manager_staff_state_record(read_u32_as_usize(c)), read_u32_as_usize(this + 8));
        let n = read_u32_as_usize(this + 8);
        let r = read_u32_as_usize(this).wrapping_add(n.wrapping_mul(0x50));
        write_u32_from_usize(this + 8, n.wrapping_add(1));
        copy_contract(r, c);
    }
    crate::comparacao::ramo("cqg_1 valor na pessoa");
    write_u32_from_usize(pessoa + staff::WAGE, read_u32_as_usize(c + 0xc)); // cqg1: o valor do contrato na pessoa
    if read_u8(c + 0x35) & 0x3f == 2 && read_u32_as_usize(pessoa + staff::PLAYER) != 0 {
        moral(pessoa, 0x14, 0);
    }
    let f = financas(read_u32_as_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_u32_as_usize(c + 4)));
    if f != 0 {
        folha(f);
    }
    if cargo as u8 == read_u8(pessoa + staff::JOB_FOR_CLUB) {
        return 1;
    }
    if tirar(read_u32_as_usize(pessoa + staff::CLUB_JOB), pessoa, 0, 1, 0) == 0 {
        show_error_at(0x9a8190, 0x1672);
        return 0;
    }
    if incluir(transfer_manager_club_by_id(read_u32_as_usize(c + 4)), pessoa, cargo, pessoa + staff::DATE_JOINED_CLUB, 1, 0) != 0 {
        return 1;
    }
    show_error_at(0x9a8190, 0x167a);
    0
}

/// Comparer for `sign_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4cf720: só a original (inclui e tira a pessoa dos clubes, realoca o vetor dos contratos, entrega
/// notícias, sorteia a segunda nacionalidade, recalcula a folha).
pub(crate) unsafe extern "thiscall" fn comparar_sign_contract(this: usize, contrato: usize, cargo: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SIGN_CONTRACT)(this, contrato, cargo) // ponytail: só a original
}








/// Multiplies the three contract bonuses (+0x10, +0x14, +0x18) that are not -1 by f, truncated.
///
/// [pt] Os três bônus do contrato (+0x10, +0x14, +0x18): os que não são −1 (> −1) × `f`, truncados.
pub(crate) unsafe fn scale_contract_bonuses(c: usize, f: f64) {
    for k in 0..3 {
        let a = c + 0x10 + 4 * k;
        let v = read_i32(a);
        if v > -1 {
            write_u32(a, match_pl_ftol(v as f64 * f) as u32);
        }
    }
}

/// the desired contract (0x85fc50) × factor, the minimum release fee, the cap (club or staff maximum wage), the end
/// next year, in 2 years or by age, the total by age that reduces the value or the cap, the cut-offs and the value
/// capped; returns the terms or makes the offer (0x4cc490)
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
pub(crate) unsafe extern "thiscall" fn contract_manager_offer_contract(this: usize, pessoa: usize, clube_arg: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let tipo_11_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    let estatuto_de: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_GET_SQUAD_STATUS_FOR_CLUB);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let valor_oferta: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let teto_max: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let teto_pessoal: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_STAFF_WAGE);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let limpar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_ALL_GRIEVANCES);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let propor: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);

    // o STAFF_CONTRACT local (0x50 bytes; a original o tem em [esp+0x38])
    let mut local = [0u32; 0x14];
    let c = local.as_mut_ptr() as usize;
    construir(c);
    let mut clube = clube_arg;
    if clube == 0 {
        let k = if pessoa == 0 {
            0
        } else if (read_u32_as_usize(pessoa) as i32) < read_i32(0xae2364) {
            let idx = read_i32(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(pessoa).wrapping_mul(0x4f)));
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
    if read_u8(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(pessoa).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return 0;
    }
    if tipo_11_15(read_u8(pessoa + staff::JOB_FOR_CLUB) as i8) == 0 && clube == read_u32_as_usize(pessoa + staff::CLUB_JOB) {
        let lista = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856) + 0xdc);
        if read_u8(lista.wrapping_add(read_u32_as_usize(clube).wrapping_mul(9)) + 4) == 2 || (read_u32_as_usize(clube + club::MANAGER) == 0 && dirigivel(clube) != 0) {
            return 0;
        }
    }
    if humano(clube, 0, 1, 0, 0) != 0 {
        return 0;
    }
    if futura(read_u32_as_usize(G_TRANSFER_MANAGER), pessoa, 0) != 0 {
        return 0;
    }
    write_u32(c + 4, read_u32_as_usize(clube) as u32);
    let mut oferta = 0usize;
    if vaga != -1 {
        let p = vaga_da(read_u32_as_usize(G_TRANSFER_MANAGER), vaga);
        if p != 0 {
            oferta = read_u32_as_usize(p);
        }
    }
    let mut cargo: u8 = 0;
    let mut est = estatuto_de(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), pessoa, clube, &mut cargo);
    if est == 3 && read_i16(read_u32_as_usize(pessoa + staff::PLAYER) + 0xb) > 0x1482 {
        est = 2;
    }
    // ponytail: o estatuto e o cargo vão em dwords com os bytes altos de sobra da pilha na original; a 0x85fc50 e a
    // 0x597db0 leem o byte
    let r = desejado(c, pessoa, clube, vaga, est as u32, cargo as u32, 0, 0);
    if r as f64 == read_f64_at(0x9689a0) { // 0x9689a0 = 0
        return 0;
    }
    write_u32(c + 8, match_pl_ftol(read_i32(c + 8) as f64 * fator) as u32);
    write_u32(c + 0xc, match_pl_ftol(read_i32(c + 0xc) as f64 * fator) as u32);
    if read_u8(c + 0x1d) != 0 {
        let mut piso: i32 = 0;
        if oferta != 0 {
            let v = valor_oferta(oferta, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                piso = match_pl_ftol((v - 0x4c4b40) as f64 * read_f64_at(0x968928) + read_f64_at(0x968e28)); // 0x968928 = 1.25, 0x968e28 = 8425000
            } else if v > 0xf4240 {
                piso = match_pl_ftol((v - 0xf4240) as f64 * read_f64_at(0x968940) + read_f64_at(0x968e20)); // 0x968940 = 1.5, 0x968e20 = 2425000
            } else if v > 0xc350 {
                piso = match_pl_ftol((v - 0xc350) as f64 * read_f64_at(0x968e18) + read_f64_at(0x9688f0)); // 0x968e18 = 2.5, 0x9688f0 = 100000
            } else if read_i32(c + 0x21) < 0x186a0 {
                piso = 0x186a0;
            }
        } else if read_i32(c + 0x21) < 0xc350 {
            piso = 0xc350;
        }
        if read_i32(c + 0x21) < piso {
            write_u32(c + 0x21, piso as u32);
        }
    } else if (read_u8(c + 0x1f) != 0 || read_u8(c + 0x1c) != 0) && read_i32(c + 0x21) < 0xc350 {
        write_u32(c + 0x21, 0xc350);
    }
    let mut teto: f64 = if cargo == 0xb {
        let nibble = (read_u8(c + 0x4f) >> 4) as u32;
        let f = financas(read_u32_as_usize(G_FINANCE_TABLE), clube);
        let m = teto_max(f, pessoa, nibble);
        (fator + read_f64_at(0x967890)) * read_f64_at(0x968968) * m as f64 // cqf3: o teto do estatuto 11; 0x967890 = 1, 0x968968 = 0.5
    } else {
        let f = financas(read_u32_as_usize(G_FINANCE_TABLE), clube);
        teto_pessoal(f, cargo as i8) as f64
    };
    let atual = contrato_com(this, pessoa, clube);
    let mut ano = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let ficha = read_u32_as_usize(pessoa + staff::PLAYER);
    if ficha != 0 {
        let dias = dias_ate(G_CURRENT_DATE, read_u32_as_usize(pessoa + staff::DATE_OF_BIRTH) as u32, read_u32_as_usize(pessoa + 0x14) as u32);
        let mut x = dias as f64 * read_f64_at(0x968e10); // 0x968e10 = 0.002739726027
        if read_u8(ficha + 0xf) == 0x14 {
            x -= read_f64_at(0x968950); // 0x968950 = 2
        } else {
            let s = read_u8(ficha + 0x3c) as i8 as i32 + read_u8(ficha + 0x34) as i8 as i32 + read_u8(pessoa + staff::PROFESSIONALISM) as i8 as i32;
            x -= (s as f64 * read_f64_at(0x968a80) - read_f64_at(0x968e08)) * read_f64_at(0x968e00); // 0x968a80 = 0.3333333333, 0x968e08 = 15, 0x968e00 = 0.4
        }
        let fim = |c: usize| core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32;
        if x > read_f64_at(0x968df8) || (atual != 0 && read_u8(atual + 0x4f) & 0xf0 == 0x60) { // 0x968df8 = 34
            if fim(c) > ano as u32 + 1 {
                crate::comparacao::ramo("cqf_3 fim no ano que vem");
                write_u16(c + 0x2f, ano.wrapping_add(1)); // cqf3: o fim no ano que vem
                write_u32(c + 0xc, match_pl_ftol(read_i32(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
            }
            scale_contract_bonuses(c, read_f64_at(0x968928)); // 0x968928 = 1.25
        } else if x > read_f64_at(0x968df0) { // 0x968df0 = 31
            if fim(c) > ano as u32 + 2 {
                write_u16(c + 0x2f, ano.wrapping_add(2));
                write_u32(c + 0xc, match_pl_ftol(read_i32(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
            }
            scale_contract_bonuses(c, read_f64_at(0x968de8)); // 0x968de8 = 1.3
        } else if x > read_f64_at(0x968de0) && read_i16(clube + club::REPUTATION) as f64 > read_i16(ficha + 0xd) as f64 * read_f64_at(0x968978) { // 0x968de0 = 29, 0x968978 = 1.2
            let k = match_pl_ftol(x + read_f64_at(0x968968)) as i8; // 0x968968 = 0.5
            if fim(c) as i32 > (ano as i32 - k as i32) + 0x22 {
                write_u16(c + 0x2f, ano.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                write_u32(c + 0xc, match_pl_ftol(read_i32(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
            }
            scale_contract_bonuses(c, read_f64_at(0x968940)); // 0x968940 = 1.5
        }
        if read_i32(c + 0xc) > 0x2710 && x > read_f64_at(0x968dd8) { // 0x968dd8 = 30
            let dias_fim = dias_ate(c + 0x2d, read_u32_as_usize(G_CURRENT_DATE) as u32, read_u32_as_usize(G_CURRENT_DATE_LEAP) as u32);
            let mut semanas = dias_fim / 7;
            let mut total = semanas.wrapping_mul(read_i32(c + 0xc));
            let mut a = game_pow(read_i16(ficha + 0xb) as f64, read_f64_at(0x968950)) * read_f64_at(0x968dd0); // 0x968950 = 2, 0x968dd0 = 0.0001333333333
            if !(a < read_f64_at(0x968dc8)) { // 0x968dc8 = 8000
                a = 8000.0;
            }
            let mut b = game_pow(x - read_f64_at(0x968dd8), read_f64_at(0x968950)); // 0x968dd8 = 30, 0x968950 = 2
            if !(b > read_f64_at(0x967890)) { // 0x967890 = 1
                b = 1.0;
            }
            let limit = match_pl_ftol(game_pow(a, read_f64_at(0x968950)) / b); // 0x968950 = 2
            let reduz = (limit as f64 * read_f64_at(0x968940)) < total as f64; // 0x968940 = 1.5
            let mut feito = false;
            if reduz {
                while semanas > 0x68 {
                    semanas -= 0x34;
                    write_u32(c + 0xc, match_pl_ftol(read_i32(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32); // 0x9689c0 = 1.1
                    total = semanas.wrapping_mul(read_i32(c + 0xc));
                    if limit >= total {
                        feito = true;
                        break;
                    }
                }
            }
            if !feito && total > limit {
                let q = limit / semanas;
                if (q as f64) < teto {
                    crate::comparacao::ramo("cqf_3 teto pelo total");
                    teto = q as f64; // cqf3: o teto pelo total
                }
            }
        }
        ano = read_i16(G_CURRENT_DATE_YEAR) as u16;
    }
    if atual != 0 {
        if core::ptr::read_unaligned((atual + 0x2f) as *const u16) >= core::ptr::read_unaligned((c + 0x2f) as *const u16) {
            return 0;
        }
        if read_u32_as_usize(G_INJURY) != 0 {
            let l = lesao(read_u32_as_usize(G_INJURY), pessoa);
            if l != 0 && read_i16(lesao(read_u32_as_usize(G_INJURY), pessoa) + 0x2c) > 0x46 {
                return 0;
            }
            ano = read_i16(G_CURRENT_DATE_YEAR) as u16;
        }
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32 > ano as u32 + 5 {
        write_u16(c + 0x2f, ano.wrapping_add(5));
    }
    let w = read_i32(c + 0xc) as f64;
    if w > teto * read_f64_at(0x968dc0) { // 0x968dc0 = 1.4
        return 0;
    }
    if w > teto {
        crate::comparacao::ramo("cqf_3 acima do teto");
        let den = if read_f64_at(0x967890) > teto { read_f64_at(0x967890) } else { teto }; // 0x967890 = 1
        let razao = w / den;
        write_u32(c + 8, match_pl_ftol(read_i32(c + 8) as f64 * razao) as u32);
        scale_contract_bonuses(c, razao);
        write_u32(c + 0xc, match_pl_ftol(teto) as u32); // cqf3: o valor no teto
    }
    limpar(c, 0);
    if saida != 0 {
        for k in (0..0x20).step_by(4) {
            write_u32(saida + k, read_u32_as_usize(c + k) as u32);
        }
        write_u8(saida + 0x20, read_u8(c + 0x20));
        write_u32(saida + 0x21, read_u32_as_usize(c + 0x21) as u32);
        let mut tmp = [0u8; 8];
        copiar_data(saida + 0x25, tmp.as_mut_ptr(), c + 0x25);
        copiar_data(saida + 0x2d, tmp.as_mut_ptr(), c + 0x2d);
        // o +0x35 pelos campos de bits da original: 6 bits, 1 bit e 1 bit
        let cl = read_u8(c + 0x35);
        let dl = read_u8(saida + 0x35);
        let al = ((((cl << 2) as i8 >> 2) as u8 ^ dl) & 0x3f) ^ dl;
        let d2 = (((((cl << 1) as i8 >> 1) as u8 ^ al) & 0xc0) ^ cl ^ al) & 0x7f;
        write_u32(saida + 0x36, read_u32_as_usize(c + 0x36) as u32);
        write_u8(saida + 0x35, d2 ^ cl);
        write_u8(saida + 0x3a, read_u8(c + 0x3a));
        write_u16(saida + 0x3b, read_i16(c + 0x3b) as u16);
        for k in [0x3dusize, 0x3e, 0x3f] {
            write_u8(saida + k, read_u8(c + k));
        }
        write_u32(saida + 0x40, read_u32_as_usize(c + 0x40) as u32);
        write_u8(saida + 0x44, read_u8(c + 0x44));
        write_u32(saida + 0x45, read_u32_as_usize(c + 0x45) as u32);
        write_u8(saida + 0x49, read_u8(c + 0x49));
        write_u32(saida + 0x4a, read_u32_as_usize(c + 0x4a) as u32);
        write_u8(saida + 0x4e, read_u8(c + 0x4e));
        write_u8(saida + 0x4f, read_u8(c + 0x4f));
        return 1;
    }
    let modo = modo as u8;
    let enviar = if modo == 2 {
        true
    } else if atual == 0 || modo != 1 {
        false
    } else {
        let mut d = [0u8; 8];
        let p = mais(G_CONTRACT_DATE_PLUS_365_DAYS, d.as_mut_ptr(), 0x96);
        let (dia, ano_d) = (read_i16(p), core::ptr::read_unaligned((p + 2) as *const u16));
        let ano_a = core::ptr::read_unaligned((atual + 0x2f) as *const u16);
        ano_d < ano_a || (ano_d == ano_a && dia < read_i16(atual + 0x2d))
    };
    if enviar {
        // ponytail: o cargo (+0x3d) vai num dword com os bytes altos de sobra do edx na original; a 0x4cc490 lê o byte
        propor(this, c, vaga, u32::MAX, read_u8(pessoa + staff::JOB_FOR_CLUB) as u32, 0, 1, 0);
    }
    1
}

/// Comparer for `contract_manager_offer_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4cda70: com `saida`, lado a lado (a saída, [0xae1080] e o estado do sorteador; a cópia sem contar os
/// sorteios); sem `saida` (pode propor o contrato, que aloca), só a original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_contract_manager_offer_contract(this: usize, pessoa: usize, clube: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, f64, u32) -> u32>(IDX_CONTRACT_MANAGER_OFFER_CONTRACT);
    if saida == 0 {
        return o(this, pessoa, clube, saida, vaga, fator, modo); // ponytail: só a original
    }
    let est = crate::aleatorio::REGIOES_DO_ESTADO;
    let regioes = [(saida, 0x50usize), (G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, saida, vaga, fator, modo),
        || crate::aleatorio::sem_contar(|| contract_manager_offer_contract(this, pessoa, clube, saida, vaga, fator, modo)));
    // ponytail: o STAFF_CONTRACT local da original não tem valor em +0x38..+0x39 (do dword +0x36), +0x44 e no nibble
    // baixo de +0x4f (nem a 0x85e410 nem a 0x85fc50 os gravam: sobra de pilha, copiada para a saída); a cópia deixa 0
    // (montada em memória zerada). A comparação ignora esses bits.
    let sem_sobra = |m: &crate::comparacao::Memoria| {
        let mut v = m.0[0].1.clone();
        v[0x38] = 0;
        v[0x39] = 0;
        v[0x44] = 0;
        v[0x4f] &= 0xf0;
        v
    };
    let igual = igual || (sem_sobra(&mo) == sem_sobra(&mr) && mo.0[1..] == mr.0[1..]);
    PLACAR.registrar(igual && r == n, || format!(
        "contract_manager_offer_contract(pessoa {:#x}, clube {:#x}, vaga {}, fator {}, modo {}): original {} rust {}, memória {:x?} x {:x?}",
        pessoa, clube, vaga, fator, modo, r, n, mo, mr));
    r
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
pub(crate) unsafe fn match_clubs_by_side(jogo: usize, lado: usize) -> (usize, usize) {
    if lado == 0 { (read_usize(jogo + 0x1c), read_usize(jogo + 0x20)) } else { (read_usize(jogo + 0x20), read_usize(jogo + 0x1c)) }
}

/// True when match byte +0x3a = 1 and the other club's reputation (+0x80) is below max(own rep × constant, own rep +
/// 1500): that side is skipped.
///
/// [pt] Com o byte +0x3a do jogo = 1, o lado fica de fora quando a reputação do outro clube (+0x80) fica abaixo do maior
/// entre a dele × [0x968928] e a dele + 1500 (x87 em dupla, como a original).
pub(crate) unsafe fn skip_side_by_reputation(jogo: usize, lado: usize) -> bool {
    if read_i8(jogo + 0x3a) != 1 {
        return false;
    }
    let (meu, outro) = match_clubs_by_side(jogo, lado);
    let r = read_i16(meu + 0x80) as i32;
    let x = r as f64 * read_f64_at(0x968928); // 0x968928 = 1.25
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > read_i16(outro + 0x80) as f64
}

/// Person (0x6e-byte record in G_STAFF_TABLE) of a match summary slot, or None when the original skips the slot (id -1
/// or out of range).
///
/// [pt] A pessoa da vaga do resumo (id em +0 e +4 diferente de −1; registro de 0x6e bytes em [0xae23bc]), ou None quando a
/// original pula a vaga. A pessoa nula (base 0) passa, como na original.
pub(crate) unsafe fn staff_of_summary_slot(vaga: usize) -> Option<usize> {
    let id = read_i32(vaga);
    if id == -1 || read_i32(vaga + 4) == -1 {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if p != 0 && read_i32(p) >= read_i32(0xae2364) {
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
pub(crate) unsafe fn match_bonuses_for_slot(jogo: usize, contrato: usize, vaga: usize, palavra: i16) -> [Option<i32>; 3] {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let comp = read_usize(jogo + 0x14);
    if comp != 0 && (amistoso(read_usize(G_FRIENDLY), comp) != 0 || read_usize(jogo) == read_usize(KEY_CLUB_COMP_RESERVE)) {
        return [None; 3];
    }
    let tipo = *((contrato + contract::CONTRACT_TYPE) as *const u8) & 0x3f;
    if tipo == 6 || tipo == 5 {
        return [None; 3];
    }
    let g = *((vaga + 0x21) as *const u8) as i32;
    let a = *((vaga + 0x27) as *const u8) as i32;
    let um = (g != 0 && read_i32(contrato + contract::GOAL_BONUS) > 0).then(|| g.wrapping_mul(read_i32(contrato + contract::GOAL_BONUS)));
    let dois = (a != 0 && read_i32(contrato + contract::ASSIST_BONUS) > 0).then(|| a.wrapping_mul(read_i32(contrato + contract::ASSIST_BONUS)));
    let tres = (palavra == 1 && read_i8(vaga + 0x22) != 0 && read_i32(contrato + contract::CLEAN_SHEET_BONUS) > 0).then(|| a.wrapping_mul(read_i32(contrato + contract::CLEAN_SHEET_BONUS)));
    [um, dois, tres]
}

/// after each match: contract bonuses, happiness by minutes played, morale by a draw and by the result, and the
/// development and happiness of those left out
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
pub(crate) unsafe extern "thiscall" fn update_after_match(this: usize, jogo: usize, resumo: usize) {
    if read_i8(jogo + 0x40) == 0 || read_i8(jogo + 0x41) == 0 {
        return;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let pagar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_BONUSES);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let marcar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    // a original põe ecx = this antes da 0x4cb0a0 (stdcall na cópia Rust): thiscall com 3 na pilha serve aos dois
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_RAISE_MATCH_MORALE);
    let baixar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_LOWER_MATCH_MORALE);
    let (c47, c48) = (read_i8(jogo + 0x47), read_i8(jogo + 0x48));
    let (a, b) = if c47 == -1 { (read_i8(jogo + 0x43), read_i8(jogo + 0x44)) } else { (c47, c48) };
    let venceu = [(a > b) as u32, (a < b) as u32];
    let mut lista = [[0usize; 50]; 2];
    for (lado, l) in lista.iter_mut().enumerate() {
        let c = match_clubs_by_side(jogo, lado).0;
        core::ptr::copy_nonoverlapping((c + 0xd7) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let clube = match_clubs_by_side(jogo, lado).0;
        if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if skip_side_by_reputation(jogo, lado) {
            continue;
        }
        let fin = financas(read_usize(G_FINANCE_TABLE), clube);
        let base = resumo + lado * 0x625;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = staff_of_summary_slot(vaga) else { continue };
            let contrato = if selecao(clube) != 0 {
                0
            } else {
                let c = com_clube(this, pessoa, clube);
                if c == 0 {
                    continue;
                }
                for v in match_bonuses_for_slot(jogo, c, vaga, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    pagar(fin, v);
                }
                c
            };
            let minutos: i8 = if k < 11 {
                let v = read_i8(vaga + 0x30);
                if v > -1 { v } else { 0x5a }
            } else {
                let v = read_i8(vaga + 0x32);
                if v <= -1 {
                    0
                } else {
                    let e = 0x5a - v as i32;
                    if e <= 0 { 0 } else { e as i8 }
                }
            };
            if k < 11 || read_i8(vaga + 0x32) != -1 {
                let q = ((minutos as i32 * 10) / 90) as i8;
                if selecao(clube) != 0 {
                    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 9;
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
            if selecao(clube) == 0 && (read_i32(jogo) < 0 || amistoso(read_usize(G_FRIENDLY), read_usize(jogo + 0x14)) == 0) {
                let testa = k >= 11 || !(minutos > 0x1e || read_i8(vaga + 0x1f) != 0 || read_i8(vaga + 0x20) < 0x46);
                let mal = testa && {
                    crate::comparacao::ramo("cqa_8 testa a moral");
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
                    baixar(this, pessoa, clube, contrato);
                } else {
                    *p.add(0x3e) = 0;
                    if *p.add(0x3f) != 100 {
                        *p.add(0x3f) = (*p.add(0x3f)).wrapping_add(1);
                    }
                    if read_i8(contrato + 0x3f) >= 5 && humano(clube, 0, 1, 0, 0) == 0 {
                        marcar(pessoa, 1);
                    }
                    subir(this, pessoa, clube, contrato);
                }
            }
            if venceu[lado] != 0 {
                crate::comparacao::ramo("cqa_8 moral do vencedor");
                let r = sorteio(0x15 - read_i8(pessoa + staff::AMBITION) as i32); // cqa8: o sorteio do vencedor
                if r < 4 && contrato != 0 {
                    add_clamped_100(contrato + 0x44, 3);
                }
            } else if venceu[1 - lado] != 0 {
                let r = sorteio(0x15 - read_i8(pessoa + staff::AMBITION) as i32);
                if r < 4 && contrato != 0 {
                    add_clamped_100(contrato + 0x44, -3);
                }
            }
            if let Some(x) = lista[lado].iter_mut().find(|x| **x == pessoa) {
                *x = 0;
            }
        }
    }
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let evoluir: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TOGGLE_SQUAD_PLACEMENT);
    for lado in 0..2usize {
        let (clube, outro) = match_clubs_by_side(jogo, lado);
        let limit: i8 = if parceiro(clube, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if skip_side_by_reputation(jogo, lado) {
            continue;
        }
        for i in 0..50usize {
            let pessoa = lista[lado][i];
            if pessoa == 0 || read_i32(pessoa) >= read_i32(0xae2364) || read_usize(pessoa + staff::PLAYER) == 0 || selecao(clube) != 0 {
                continue;
            }
            let contrato = com_clube(this, pessoa, clube);
            if contrato == 0 {
                continue;
            }
            if read_i32(jogo) != -2 && suspenso(read_usize(G_DISCIPLINE), pessoa, read_usize(jogo + 0x14), 1, 0) != 0 {
                continue;
            }
            if lesao(read_usize(G_INJURY), pessoa) != 0 || impedido(read_usize(G_NATIONAL_TEAMS), pessoa, G_CURRENT_DATE, clube, 0, 0) != 0 {
                continue;
            }
            // emprestado: o segundo contrato da pessoa (+4 do registro de 79 bytes)
            let idx = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 4);
            if idx >= 0 && (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)) != 0 {
                continue;
            }
            let m = read_usize(G_PLAYER_STATS);
            let x = core::ptr::read_unaligned((m + 0x18) as *const f64);
            let c = read_f64_at(0x9689a0); // 0x9689a0 = 0
            // fcomp + C3: igual ou desordenado vai para a constante
            let nota = if !(x < c || x > c) { read_f64_at(0x968998) } else { core::ptr::read_unaligned((m + 0x10) as *const f64) / x }; // 0x968998 = 6.8
            crate::comparacao::ramo("cqa_8 evolui quem ficou de fora");
            evoluir(0, pessoa, clube, outro, jogo, (nota - read_f64_at(0x967888)) as f32, 0, 0); // cqa8: a evolução de quem ficou de fora; 0x967888 = 0.05
            let p = contrato as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if read_i8(contrato + contract::UNKNOWN18_2) >= limit && humano(clube, 0, 1, 0, 0) == 0 && parceiro(clube, core::ptr::null_mut(), 1) == 0 {
                let l = lugar(this, pessoa, clube);
                if l == 2 || l == 4 {
                    trocar(this, pessoa, clube, 0);
                }
            }
            add_clamped_100(contrato + 0x3d, -8);
            baixar(this, pessoa, clube, contrato);
        }
    }
}

/// Comparer for `update_after_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a moral (0x4cb0a0, 0x4cb250: queixas, pedido de transferência), a evolução (0x6d26b0) e a troca de
/// elenco (0x4cbd20) sorteiam e alocam (não podem rodar duas vezes): só a original roda. Confere o que a rotina garante
/// antes de qualquer sorteio: os bônus que cada clube paga (+0x138 das finanças soma o total previsto pelas mesmas
/// regras da cópia, calculado antes). A cópia é provada pelos ramos e pelas mutações na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_update_after_match(this: usize, jogo: usize, resumo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_AFTER_MATCH);
    if read_i8(jogo + 0x40) == 0 || read_i8(jogo + 0x41) == 0 {
        return o(this, jogo, resumo);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let mut plano: [(usize, i32, i32); 2] = [(0, 0, 0); 2];
    let mut eh_a = 0u32;
    for (lado, pl) in plano.iter_mut().enumerate() {
        let clube = match_clubs_by_side(jogo, lado).0;
        if (parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0) || skip_side_by_reputation(jogo, lado) || selecao(clube) != 0 {
            continue;
        }
        let fin = financas(read_usize(G_FINANCE_TABLE), clube);
        let base = resumo + lado * 0x625;
        let mut total = 0i32;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = staff_of_summary_slot(vaga) else { continue };
            let c = com_clube(this, pessoa, clube);
            if c != 0 {
                for v in match_bonuses_for_slot(jogo, c, vaga, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    total = total.wrapping_add(v);
                }
            }
        }
        if fin != 0 {
            *pl = (fin, read_i32(fin + 0x138), total);
        }
    }
    o(this, jogo, resumo);
    for &(fin, antes, total) in &plano {
        if fin != 0 {
            let pago = read_i32(fin + 0x138).wrapping_sub(antes);
            PLACAR.registrar(pago == total, || format!("update_after_match(jogo {:x}) bônus de fin {:x}: original {} previsto {}", jogo, fin, pago, total));
        }
    }
}

/// the morale of the player record (+0x45), for the screens
/// Original: `contract_manager_get_player_morale` (0x4cc2d0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc2d0 } (stdcall, ret 4, contract_manager.cpp): `contract_manager_get_player_morale(pessoa)`: o byte +0x45 da
/// ficha (+0x61), pelas telas. O eax leva os bytes altos da pessoa, como na original.
pub(crate) unsafe extern "stdcall" fn contract_manager_get_player_morale(pessoa: usize) -> u32 {
    (pessoa as u32 & 0xffff_ff00) | *((read_usize(pessoa + staff::PLAYER) + 0x45) as *const u8) as u32
}

/// Comparer for `contract_manager_get_player_morale` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_get_player_morale(pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_CONTRACT_MANAGER_GET_PLAYER_MORALE)(pessoa);
    let n = contract_manager_get_player_morale(pessoa);
    PLACAR.registrar(r == n, || format!("contract_manager_get_player_morale({:x}) original={:#x} rust={:#x}", pessoa, r, n));
    r
}

/// Copies the zero-terminated text at a (up to 0x400 bytes) into a Vec, for the text comparers.
///
/// [pt] O texto terminado em zero em `a` (até 0x400 bytes), para os comparadores dos textos.
pub(crate) unsafe fn c_string_bytes(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// the morale word (from "Very Low" to "Superb") into a text, for the screens
/// Original: `contract_manager_write_morale_text` (0x4cc2e0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc2e0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_write_morale_text(pessoa, saída, _)`: a
/// palavra da moral (+0x45 da ficha: abaixo de 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", senão
/// "Superb") na saída (0x66f4e0 com 0xad9c64), pelas telas.
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_text(pessoa: usize, saida: usize, _x: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let m = read_i8(read_usize(pessoa + staff::PLAYER) + 0x45);
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
    formatar(saida, f, 0xad9c64usize);
}

/// Comparer for `contract_manager_write_morale_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original escreve a palavra; a cópia escreve de novo (o texto se sobrescreve) e as duas têm de bater;
/// fica o texto da original.
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_write_morale_text(pessoa: usize, saida: usize, x: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(IDX_CONTRACT_MANAGER_WRITE_MORALE_TEXT)(pessoa, saida, x);
    let r = c_string_bytes(saida);
    contract_manager_write_morale_text(pessoa, saida, x);
    let n = c_string_bytes(saida);
    core::ptr::copy_nonoverlapping(r.as_ptr(), saida as *mut u8, r.len());
    *((saida + r.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n, || format!("contract_manager_write_morale_text({:x}) original={:?} rust={:?}", pessoa, String::from_utf8_lossy(&r), String::from_utf8_lossy(&n)));
}





/// Main contract record (0x50 bytes in the contract manager) of a person via G_STAFF_STATE_TABLE, or 0 when none/out of
/// range.
///
/// [pt] O contrato principal da pessoa (índice +0 do registro de 79 bytes em [0xae2d0c], registros de 0x50 bytes em
/// [[0xae1050]]), ou 0: sem pessoa, fora das pessoas ([0xae2364]) ou sem índice.
pub(crate) unsafe fn main_contract_of_staff(pessoa: usize) -> usize {
    if pessoa == 0 || read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f))) as i32;
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

/// the player value by peak age and age, record ratings, ability as seen by the club manager, club nation, contract
/// days left and release clause
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
pub(crate) unsafe extern "thiscall" fn calc_player_value(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, _x: u32) -> i32 {
    let c1080 = main_contract_of_staff(pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c1080;
    let transfer_manager_contract_club = if c1080 == 0 { 0 } else { friendly_club_by_id(read_usize(c1080 + 4)) };
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = com_clube(this, pessoa, transfer_manager_contract_club);
    let ficha = read_usize(pessoa + staff::PLAYER);
    if ficha == 0 {
        if transfer_manager_contract_club == 0 {
            return 0;
        }
        // a original põe ecx = [0xde1f54] (a cópia Rust é stdcall): thiscall com 3 na pilha serve aos dois
        let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
        return compensacao(read_usize(G_TRANSFER_MANAGER), pessoa, transfer_manager_contract_club, 1);
    }
    let (base, auge): (f64, i8) = if read_i8(ficha + 0x15) >= 0xf {
        (13_000_000.0, 0x1c) // cqa9: a base de quem joga na frente
    } else if read_i8(ficha + 0x14) >= 0xf {
        (11_000_000.0, 0x1d)
    } else if read_i8(ficha + 0x13) >= 0xf {
        (10_000_000.0, 0x1d)
    } else if read_i8(ficha + 0x12) >= 0xf {
        (9_000_000.0, 0x1d)
    } else if read_i8(ficha + 0xf) == 0x14 {
        (6_000_000.0, 0x21)
    } else {
        (8_000_000.0, 0x1d)
    };
    let idade = read_i8(pessoa + staff::YEAR_OF_BIRTH);
    let dif = auge as i32 - idade as i32;
    let mut t = dif as f64;
    if t > read_f64_at(0x9688a0) { // 0x9688a0 = 5
        t = read_f64_at(0x968f10) - t * read_f32_at(0x968f18) as f64; // 0x968f10 = 8, 0x968f18 = 0.5
    }
    let a = match_pl_ftol(t).wrapping_abs();
    let mut f = t * a as f64 * read_f32_at(0x968f08) as f64 * read_f64_at(0x968f00) + read_f64_at(0x967890); // 0x968f08 = 0.3333333433, 0x968f00 = 0.08333333333, 0x967890 = 1
    if read_i8(ficha + 0xf) < 0xf {
        let nota = |o: usize, c: usize| (read_i8(ficha + o) as i32 - 0xf) as f64 * read_f64_at(c) + read_f64_at(0x967890); // 0x967890 = 1
        let fa = nota(0x28, 0x968ef8);
        let fb = read_i8(ficha + 0x27) as f64 * read_f64_at(0x968ef0) + read_f64_at(0x967890); // 0x968ef0 = 0.005, 0x967890 = 1
        let fc = read_i8(ficha + 0x43) as f64 * read_f64_at(0x968e60) + read_f64_at(0x967890); // 0x968e60 = 0.003333333333, 0x967890 = 1
        let fd = nota(0x36, 0x968ef8);
        let fe = nota(0x2f, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = read_i8(ficha + 0x2d) as i32;
    let fator = ((read_f64_at(0x968ee0) - (q * q) as f64 * read_f64_at(0x967888) * read_f64_at(0x968ee8)) * f) as f32; // 0x968ee0 = 1.33, 0x967888 = 0.05, 0x968ee8 = 0.034
    let mut r = read_i16(ficha + 9) as f64 * read_f64_at(0x968ed8) + read_f64_at(0x968ed0); // 0x968ed8 = 0.0003333333333, 0x968ed0 = 0.17
    if !(r >= read_f64_at(0x9689c0)) { // 0x9689c0 = 1.1
        r = read_f64_at(0x9689c0); // 0x9689c0 = 1.1
    }
    let x: f32 = if !(fator as f64 >= read_f64_at(0x967898)) { // 0x967898 = 0.01
        read_f64_at(0x967898) as f32 // 0x967898 = 0.01
    } else if !(r >= fator as f64) {
        r as f32
    } else {
        fator
    };
    let mut v: u32 = if clube == 0 {
        read_i16(ficha + 0xb) as i32 as u32
    } else {
        let vista: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
        let tecnico = read_usize(clube + club::MANAGER);
        let rep = vista(pessoa, tecnico, 0) as i16 as i32;
        let h = if habilidade as u16 == 0xffff {
            let hab: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
            hab(tecnico, pessoa, clube, 1, rep as i16) as u16
        } else {
            habilidade as u16
        };
        // só o ax de h × 50 conta (movsx depois); negativo vira 0
        let h50 = (h.wrapping_mul(50) as i16).max(0);
        let teto = if (rep as u32) < 0x1388 { rep as u32 } else { 0x1388 };
        let s = (h50 as i32).wrapping_add(rep) as u32;
        let y = match_pl_ftol((s as f64 + teto as f64 * x as f64) * read_f32_at(0x968f08) as f64) as u32; // 0x968f08 = 0.3333333433
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = read_i16(ficha + 0xd);
    if pot > 0x1388 && (idade as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && idade <= auge {
            let z = pot as f64 * read_f64_at(0x968ec8) + (v >> 1) as f64 - read_f64_at(0x968ec0); // 0x968ec8 = 2.25, 0x968ec0 = 12500
            let e = v as f64;
            v = match_pl_ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * read_f64_at(0x968eb8); // 0x968eb8 = 1e-08
    let mut idade_ef = idade;
    let exp = *((pessoa + staff::INT_APPS) as *const u8);
    if exp != 0 && read_usize(pessoa + staff::NATION) != 0 {
        let n = (read_i16(read_usize(pessoa + staff::NATION) + 0x8e) as i32).wrapping_mul(exp as i32) / 18;
        let n = n as i16 as i32;
        let m = if n < dif { n } else { dif };
        let soma = if m < 0 { 0 } else if n >= dif { dif } else { n };
        idade_ef = idade_ef.wrapping_add(soma as i8);
    }
    let mut i = auge.wrapping_sub(idade_ef).wrapping_add(10); // cqa9: o índice da tabela pela idade
    if i < 0 {
        i = 0;
    } else if i >= 0x14 {
        i = 0x13;
    }
    let tab = *((0x9a7f30 + i as usize) as *const i8) as i32;
    let mut w = tab as f64 * vv * read_f64_at(0x967898); // 0x967898 = 0.01
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
            let mut t = read_i8(nat + 0x7e) as f64;
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
            let n = if div != 0 { read_i16(div + 0x69) as i32 } else { read_i8(nat + 0x85) as i32 };
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
    let piso = k * read_f64_at(0x9688e8); // 0x9688e8 = 5000
    if !(w >= piso) {
        w = piso;
    }
    let nacao_e = |g: usize| -> bool {
        let nat = read_usize(clube + club::NATION);
        clube != 0 && nat != 0 && read_usize(nat) == read_usize(g)
    };
    // a nação [0x9cf434]: abaixo de [0x968e90] a curva com piso, senão × [0x9689d8]
    let curva = |w: f64, piso: f64| -> f64 {
        if !(w >= read_f64_at(0x968e90)) { // 0x968e90 = 6000000
            let v = (w * read_f64_at(0x968e88) * read_f64_at(0x968e48) + read_f64_at(0x968e80)) * w; // 0x968e88 = 1.666666667e-07, 0x968e48 = 0.6, 0x968e80 = 0.3
            if !(read_f64_at(0x9688e8) > v) { v } else { piso } // 0x9688e8 = 5000
        } else {
            w * read_f64_at(0x9689d8) // 0x9689d8 = 0.9
        }
    };
    if modo as u8 != 0 {
        if nacao_e(KEY_NATION_PORTUGAL) {
            w = curva(w, 5000.0);
        }
        return cap_value_at_release_fee(contrato, w);
    }
    let mut nacoes = true; // 0x4d1480 (as outras nações) ou 0x4d14ad (a [0x9cf434] antes)
    if contrato == 0 {
        w = read_f64_at(0x9689a0); // 0x9689a0 = 0
    } else {
        let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_CLUB_USES_BOSMAN);
        if bosman(read_usize(G_TRANSFER_MANAGER), pessoa) != 0 {
            let ano = |a: usize| core::ptr::read_unaligned(a as *const u16) as i32;
            let mut dias = (ano(contrato + 0x2f) - ano(0xae2c92)).wrapping_mul(365) + read_i16(contrato + contract::CONTRACT_EXPIRES) as i32 - read_i16(G_CURRENT_DATE) as i32;
            if dias < 1 {
                return 0;
            }
            let mut w2 = w;
            if clube == transfer_manager_contract_club && read_i8(contrato + 0x3d) > 0 {
                if *((contrato + contract::CONTRACT_TYPE) as *const u8) & 0x3f == 2 {
                    dias = dias + 7 * read_i8(contrato + 0x3d) as i32 + 0xc8;
                } else {
                    w2 = w * read_f64_at(0x968968); // 0x968968 = 0.5
                }
            }
            let x = dias as f64 * read_f64_at(0x968e78); // 0x968e78 = 0.001428571429
            if dias < 0x15e { // cqa9: o contrato curto
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
        nacoes = false;
    }
    if !nacoes {
        // 0x4d14ad: valor zero (ou desordenado) vai às outras nações; sem clube, ao fim
        if !(w < read_f64_at(0x9689a0) || w > read_f64_at(0x9689a0)) { // 0x9689a0 = 0
            nacoes = true;
        } else if clube == 0 {
            return cap_value_at_release_fee(contrato, w);
        } else if nacao_e(KEY_NATION_PORTUGAL) {
            w = curva(w, read_f64_at(0x9688e8)); // 0x9688e8 = 5000
            return cap_value_at_release_fee(contrato, w);
        } else {
            nacoes = true;
        }
    }
    if nacoes && clube != 0 {
        if nacao_e(KEY_NATION_FRANCE) {
            w *= read_f64_at(0x9689b0); // 0x9689b0 = 0.8
        } else if nacao_e(KEY_NATION_SOUTH_KOREA) {
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
    let (e, c) = (crate::aleatorio::REGIOES_DO_ESTADO, DATE_OP_STATICS);
    [e[0], e[1], (G_CONTRACT_LOOKUP_RESULT, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}

/// Comparer for `calc_player_value` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado (sem alocar) com o estado do sorteador (sem clube, sorteio(3)), o [0xae1080] e as estáticas
/// das datas da compensação; a cópia sem contar os sorteios. Quente (5,6 milhões de chamadas): sem contador.
pub(crate) unsafe extern "thiscall" fn comparar_calc_player_value(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, x: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32>(IDX_CALC_PLAYER_VALUE);
    let regioes = player_value_regions();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, clube, modo, habilidade, x),
        || crate::aleatorio::sem_contar(|| calc_player_value(this, pessoa, clube, modo, habilidade, x)));
    PLACAR.registrar(igual && r == n, || format!("calc_player_value(pessoa {:x}, clube {:x}, modo {}, hab {}) original={} rust={} {:x?} x {:x?}", pessoa, clube, modo as u8, habilidade as i16, r, n, mo, mr));
    r
}

/// the morale difference of two players as a word ("Same", "Marginal", "Better", "Superior"), for the comparison screen
/// Original: `contract_manager_write_morale_comparison_text` (0x4cc3a0, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4cc3a0 } (stdcall, ret 0x10, contract_manager.cpp): `contract_manager_write_morale_comparison_text(a, b, saída,
/// _)`: a diferença das morais (+0x45 das fichas, a − b, em byte) vira "Superior" (até −14 e desde 13), "Better" (−13 a
/// −8 e 7 a 12), "Marginal" (−7 a −4 e 3 a 6) ou "Same" (−3 a 2) na saída (0x66f4e0 com 0xad9c64); devolve a diferença
/// (o jogo lê o al), pela tela de comparação.
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_comparison_text(a: usize, b: usize, saida: usize, _x: u32) -> u8 {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let dif = read_i8(read_usize(a + 0x61) + 0x45).wrapping_sub(read_i8(read_usize(b + 0x61) + 0x45));
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
    formatar(saida, f, 0xad9c64usize);
    dif as u8
}


/// Comparer for `contract_manager_write_morale_comparison_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original escreve; a cópia escreve de novo (sobrescreve) e o texto e a diferença têm de bater; fica o
/// texto da original.
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_write_morale_comparison_text(a: usize, b: usize, saida: usize, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, usize, u32) -> u8>(IDX_CONTRACT_MANAGER_WRITE_MORALE_COMPARISON_TEXT)(a, b, saida, x);
    let tr = c_string_bytes(saida);
    let n = contract_manager_write_morale_comparison_text(a, b, saida, x);
    let tn = c_string_bytes(saida);
    core::ptr::copy_nonoverlapping(tr.as_ptr(), saida as *mut u8, tr.len());
    *((saida + tr.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n && tr == tn, || format!("contract_manager_write_morale_comparison_text original={} {:?} rust={} {:?}", r as i8, String::from_utf8_lossy(&tr), n as i8, String::from_utf8_lossy(&tn)));
    r
}

/// the squad flags the club screen shows, by the contract reserve bit when the player is on loan elsewhere
/// Original: `CONTRACT_MANAGER::get_squad_flags_for_club` (0x4cc130, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc130 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_flags_for_club(pessoa, clube)`,
/// pelas telas do clube: as marcas de elenco da pessoa (+0x5e sem os bits de seleção, & 0xf3), a não ser que ela esteja
/// emprestada (segundo contrato, +4 do registro de 79 bytes) a outro clube que não o `clube` nem o parceiro dele
/// (0x540a50): com parceiro, 1; senão 2 com o bit 0x40 de +0x35 do contrato principal, 1 sem. Sem clube, a caixa
/// "Error" (a original). O jogo lê o al.
pub(crate) unsafe extern "thiscall" fn get_squad_flags_for_club(this: usize, pessoa: usize, clube: usize) -> u8 {
    if clube == 0 || pessoa == 0 {
        // ponytail: sem clube a caixa de erro; sem pessoa a original lê [0x5e] e cai: as duas ficam com a original
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, pessoa, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let geral = || *((pessoa + staff::PLAYING_SQUAD) as *const u8) & 0xf3;
    if read_usize(pessoa) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return geral();
    }
    let reg = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f));
    let i = read_usize(reg + 4) as i32;
    if i < 0 {
        return geral();
    }
    let c = (i as usize).wrapping_mul(0x50).wrapping_add(read_usize(this));
    if c == 0 || read_usize(c + 4) == read_usize(clube) {
        return geral();
    }
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p != 0 && read_usize(c + 4) == read_usize(p) {
        return geral();
    }
    if parceiro(clube, core::ptr::null_mut(), 1) != 0 {
        return 1;
    }
    let i0 = read_usize(reg) as i32;
    if i0 < 0 {
        // a original lê [0x35] do contrato nulo e cai
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, pessoa, clube);
    }
    let c0 = (i0 as usize).wrapping_mul(0x50).wrapping_add(read_usize(this));
    (*((c0 + 0x35) as *const u8) & 0x40 != 0) as u8 + 1
}

/// Comparer for `get_squad_flags_for_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_flags_for_club(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB);
    if clube == 0 || pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let r = o(this, pessoa, clube);
    let n = get_squad_flags_for_club(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("get_squad_flags_for_club({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}






/// Calls STAFF_CONTRACT::calc_contract_value (contract, person, offer) → value.
///
/// [pt] O valor de um contrato para a pessoa (STAFF_CONTRACT::calc_contract_value 0x866830: thiscall(contrato, pessoa,
/// oferta), ret 8).
pub(crate) unsafe fn calc_contract_value(contrato: usize, pessoa: usize, oferta: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_CALC_CONTRACT_VALUE);
    f(contrato, pessoa, oferta)
}



/// Writes a person's id into news slot 2 and the +0x33 keys of +4/+8/+0xc into slots 3-5.
///
/// [pt] Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
pub(crate) unsafe fn set_news_staff_slots(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    gravar(n, 2, read_usize(p) as u32);
    gravar(n, 3, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, 4, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(n, 5, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// the 0xbc2 news that the player wants a new contract, to the club manager, unless he is transfer listed or in an
/// offer
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
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let clube = friendly_club_by_id(read_usize(contrato + 4));
    let p = staff_records_staff_by_id(read_usize(contrato));
    let tm = read_usize(G_TRANSFER_MANAGER);
    if in_list(tm, p) != 0 || ofertas(tm, p, 0, 0) != 0 || trocas(tm, p, 0) != 0 {
        return;
    }
    crate::comparacao::ramo("cpw_1 noticia do contrato novo");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc2, 0);
    gravar(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    gravar(n, 6, (read_u8(contrato + contract::SQUAD_STATUS) >> 4) as u32);
    gravar(n, 7, 0);
    gravar(n, 8, read_i16(contrato + contract::CONTRACT_EXPIRES) as i32 as u32);
    gravar(n, 9, read_i16(contrato + 0x2f) as u16 as u32);
    gravar(n, 10, read_usize(contrato + contract::WAGE) as u32);
    let ficha = read_usize(p + 0x61);
    let reputado = (read_u8(p + 0x59) as i8) < 0xa && (read_u8(p + 0x5b) as i8) < 5 && ficha != 0 && read_i16(ficha + 0xd) > 0x128e;
    gravar(n, 11, reputado as u32);
    entregar(read_usize(G_NEWS), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
/// Comparer for `post_wants_new_contract_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_wants_new_contract_news(this: usize, contrato: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_WANTS_NEW_CONTRACT_NEWS)(this, contrato)
}

/// the 0xbc0 news of a withdrawn transfer request, with the reason, to the club manager
/// Original: `post_transfer_request_withdrawn_news` (0x4d9460, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d9460 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_transfer_request_withdrawn_news(contrato, motivo)`. Sem contrato: a caixa "Error" (linha 0x2ba6). Senão a
/// notícia 0xbc0 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, o motivo e, na vaga 7, 1 se +0x3f do
/// contrato passar de 1.
pub(crate) unsafe extern "thiscall" fn post_transfer_request_withdrawn_news(this: usize, contrato: usize, motivo: u32) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2ba6);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpw_1 pedido retirado");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc0, 0);
    let clube = friendly_club_by_id(read_usize(contrato + 4));
    let p = staff_records_staff_by_id(read_usize(contrato));
    gravar(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    gravar(n, 6, motivo);
    gravar(n, 7, ((read_u8(contrato + 0x3f) as i8) > 1) as u32);
    entregar(read_usize(G_NEWS), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
/// Comparer for `post_transfer_request_withdrawn_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_request_withdrawn_news(this: usize, contrato: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_TRANSFER_REQUEST_WITHDRAWN_NEWS)(this, contrato, motivo)
}






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
pub(crate) unsafe fn staff_interest_in_club(lista: usize, pessoa: usize, clube: usize, salario: u32, modo: u32, situacao: u32) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    f(lista, pessoa, clube, salario, modo, situacao, 0)
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

/// the 0xbbd news of an expiring contract to the club manager and, if asked, to interested clubs of active nations
/// Original: `post_contract_expiry_news` (0x4d6790, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d6790 } (thiscall, ret 0x10, contract_manager.cpp, quadro SEH da notícia e da lista locais): `CONTRACT_MANAGER::
/// post_contract_expiry_news(contrato, período, bosman, avisar_interessados)`, pelo passe dos contratos. Sem contrato: a
/// caixa "Error" (linha 0x24f0). Senão a notícia 0xbbd ao técnico do clube (0x7861e0): o clube, a pessoa, o período (com
/// sinal), `bosman` e, na vaga 8, se a pessoa tem ficha (+0x61). Com `avisar_interessados` (anulado se a nação do clube,
/// +0x53, não estiver ativa, 0x544670): os clubes interessados no jogador (0x842180 numa LLIST local, com 1, 1, 0) de
/// outra nação ativa também a recebem.
pub(crate) unsafe extern "thiscall" fn post_contract_expiry_news(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x24f0);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbd, 0);
    let clube = friendly_club_by_id(read_usize(contrato + 4));
    let p = staff_records_staff_by_id(read_usize(contrato));
    gravar(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    gravar(n, 6, periodo as u8 as i8 as i32 as u32);
    gravar(n, 7, bosman);
    gravar(n, 8, (read_usize(p + 0x61) != 0) as u32);
    let nacao = read_usize(clube + club::NATION);
    let avisar = if nacao != 0 && !contract_manager_nation_allows_free_transfers(nacao) { 0 } else { avisar };
    entregar(read_usize(G_NEWS), n, clube);
    if avisar == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_3 avisar interessados");
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let mut lista = [0u8; 0xe];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    interessados(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut x = proximo(l);
    while x != 0 {
        let id = read_usize(x + 4);
        if id != read_usize(clube) {
            let nac = read_usize(friendly_club_by_id(id) + 0x53);
            if nac != 0 && contract_manager_nation_allows_free_transfers(nac) {
                entregar(read_usize(G_NEWS), n, friendly_club_by_id(read_usize(x + 4)));
            }
        }
        x = proximo(l);
    }
    destruir(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
/// Comparer for `post_contract_expiry_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_expiry_news(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_CONTRACT_EXPIRY_NEWS)(this, contrato, periodo, bosman, avisar)
}

/// the 0xbbe news of a Bosman departure, with the destination club or −1
/// Original: `post_bosman_departure_news` (0x4d6d70, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4d6d70 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_bosman_departure_news(contrato, destino)`. Sem contrato: a caixa "Error" (linha 0x2611). Senão a notícia 0xbbe ao
/// técnico do clube (0x7861e0): o clube, a pessoa e o clube de destino (−1 sem ele).
pub(crate) unsafe extern "thiscall" fn post_bosman_departure_news(this: usize, contrato: usize, destino: usize) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2611);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpw_3 saida pela lei bosman");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbe, 0);
    let clube = friendly_club_by_id(read_usize(contrato + 4));
    let p = staff_records_staff_by_id(read_usize(contrato));
    gravar(n, 0, read_usize(clube) as u32);
    set_news_staff_slots(n, p);
    gravar(n, 6, if destino != 0 { read_usize(destino) as u32 } else { 0xffff_ffff });
    entregar(read_usize(G_NEWS), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
/// Comparer for `post_bosman_departure_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_bosman_departure_news(this: usize, contrato: usize, destino: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_BOSMAN_DEPARTURE_NEWS)(this, contrato, destino)
}




/// Address of the person's contract offer counter (byte +8 of the 79-byte staff state record).
///
/// [pt] O contador de ofertas de contrato da pessoa (byte +8 do registro de 79 bytes em [0xae2d0c]).
pub(crate) unsafe fn contract_offer_counter(pessoa: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)).wrapping_add(8)
}

/// Assigns date origem to destino with the game's date operator (0x417c10).
///
/// [pt] A atribuição de data (0x417c10: thiscall(destino, temporária, origem)).
pub(crate) unsafe fn contract_manager_copy_date(destino: usize, origem: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut [u32; 2], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u32; 2];
    copia(destino, &mut t, origem);
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

/// updates the same club's offer to the person or adds a new one to the list (the offer count goes up); marks the terms
/// as offered on the transfer offer and, if asked, schedules the decision
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
pub(crate) unsafe extern "thiscall" fn add_contract_offer(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let _ = a5;
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut quinzena = [0u32; 2];
    mais(G_CURRENT_DATE, &mut quinzena, 0xe);
    // ponytail: os bytes +0x67..+0x6e do registro local a original não preenche (sobra de pilha); a cópia usa zero
    let mut registro = [0u8; 0x6f];
    let r = registro.as_mut_ptr() as usize;
    construir(r);
    data_padrao(r + 0x51);
    let id = read_usize(termos);
    let mut item = 0;
    if read_u8(contract_offer_counter(termos)) != 0 && read_usize(this + 0x34) != 0 {
        let clube = read_usize(termos + 4);
        let l = read_usize(this + 0x34);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let mut x = proximo(read_usize(this + 0x34));
        while x != 0 {
            if read_usize(x) == id && read_usize(x + 4) == clube {
                item = x;
                break;
            }
            x = proximo(read_usize(this + 0x34));
        }
    }
    let tipo6 = read_u8(termos + 0x35) & 0x3f == 6;
    if item != 0 {
        crate::comparacao::ramo("cpw_4 oferta atualizada");
        // ponytail: a original embute o operator= (0x4cc810) aqui; a cópia chama a versão Rust dele, as mesmas gravações
        contract_manager_operator(item, termos);
        *((item + 0x50) as *mut u8) = if acertado != 0 { 2 } else { 0 };
        contract_manager_copy_date(item + 0x51, G_CURRENT_DATE);
        write_u32(item + 0x59, vaga as u32);
        write_u32(item + 0x5d, valor);
        *((item + 0x61) as *mut u8) = marca as u8;
        *((item + 0x62) as *mut u8) = !tipo6 as u8;
    } else {
        let operador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_OPERATOR_3D);
        operador(r, termos);
        *((r + 0x50) as *mut u8) = 0;
        contract_manager_copy_date(r + 0x51, G_CURRENT_DATE);
        write_u32(r + 0x5d, valor);
        write_u32(r + 0x59, vaga as u32);
        *((r + 0x61) as *mut u8) = marca as u8;
        write_u32(r + 0x63, 0xffff_ffff);
        *((r + 0x62) as *mut u8) = !tipo6 as u8;
        if CONTRACT_OFFER_DRY_RUN {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(DRY_RUN_CONTRACT_OFFER_RECORD) as *mut u8, 0x6f);
        } else {
            if read_usize(this + 0x34) == 0 {
                let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
                let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
                let p = novo(0xe);
                let l = if p != 0 { lista_nova(p) } else { 0 };
                write_u32(this + 0x34, l as u32);
            }
            let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
            acrescentar(read_usize(this + 0x34), r, 0x6f);
        }
        crate::comparacao::ramo("cpw_4 oferta nova");
        let c = contract_offer_counter(termos) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: o contador de ofertas da pessoa
    }
    if CONTRACT_OFFER_DRY_RUN {
        return;
    }
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let v = vaga_da(tm, vaga);
        if read_usize(read_usize(v) + 0xc) == read_usize(termos) {
            crate::comparacao::ramo("cpw_4 termos ao jogador");
            let marcar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_TRANSFER_MANAGER_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER);
            marcar(read_usize(G_TRANSFER_MANAGER), vaga); // cpw4: os termos oferecidos ao jogador
        } else {
            let troca: unsafe extern "thiscall" fn(usize, i32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_NOTE_TERMS_OFFERED_TO_EXCHANGE_PLAYER);
            troca(read_usize(G_TRANSFER_MANAGER), vaga, read_usize(termos) as u32);
        }
    }
    if decidir != 0 {
        crate::comparacao::ramo("cpw_4 agendar decisao");
        let decisao: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_OFFER_DECISION);
        decisao(this, read_usize(termos) as u32, vaga); // cpw4: a decisão agendada
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
pub(crate) static mut DRY_RUN_CONTRACT_OFFER_RECORD: [u8; 0x6f] = [0; 0x6f];

/// Comparer for `add_contract_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador. Oferta já na lista (achada antes, sem mexer no cursor): a cópia em ensaio e a original a partir do mesmo
/// estado, comparando a oferta (0x63 bytes), o contador e a cabeça da lista (o cursor). Oferta nova (acrescentada, aloca):
/// a cópia em ensaio monta o registro; a original acrescenta, e o nó novo (o último do jogador pelo clube) tem de ter os
/// mesmos 0x63 primeiros bytes, com o contador + 1.
pub(crate) unsafe extern "thiscall" fn comparar_add_contract_offer(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32)>(IDX_ADD_CONTRACT_OFFER);
    let id = read_usize(termos);
    let clube = read_usize(termos + 4);
    let cont = contract_offer_counter(termos);
    let l = read_usize(this + 0x34);
    let item = if read_u8(cont) != 0 && l != 0 { find_contract_offer(l, id, clube) } else { 0 };
    let mut todas = vec![(cont, 1usize)];
    if item != 0 {
        todas.push((item, 0x63));
        todas.push((l, 0xe));
    }
    let n_cmp = todas.len();
    todas.extend(TRANSFER_MANAGER_DATE_STATICS);
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    CONTRACT_OFFER_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| add_contract_offer(this, termos, vaga, valor, marca, a5, decidir, acertado)));
    CONTRACT_OFFER_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let registro = DRY_RUN_CONTRACT_OFFER_RECORD;
    antes.escrever();
    o(this, termos, vaga, valor, marca, a5, decidir, acertado);
    let mo = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let igual = if item != 0 {
        mo == mr
    } else {
        // o nó novo: o último do jogador pelo clube na lista (criada pela original se faltava)
        let l = read_usize(this + 0x34);
        let mut novo = 0;
        if l != 0 {
            let mut no = read_usize(l + 2);
            let mut n = 0;
            while no != 0 && n < 100000 {
                let x = read_usize(no);
                if read_usize(x) == id && read_usize(x + 4) == clube {
                    novo = x;
                }
                no = read_usize(no + 6);
                n += 1;
            }
        }
        mo == mr && novo != 0 && core::slice::from_raw_parts(novo as *const u8, 0x63) == &registro[..0x63]
    };
    PLACAR.registrar(igual, || format!("add_contract_offer(pessoa {:#x}, clube {:#x}, vaga {}, atualizada {}) original={:x?} rust={:x?}", id, clube, vaga, item != 0, mo, mr));
}

/// removes the person's contract offer linked to the transfer slot from the list (clearing the agreed-contract flag)
/// and lowers the offer count
/// Original: `remove_contract_offer_for_transfer` (0x4ccb30, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4ccb30 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer_for_transfer(pessoa,
/// vaga)`, pelas ofertas retiradas e desfeitas (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): na lista das
/// ofertas de contrato (+0x34; cursor recomeçado) a da pessoa com a vaga (+0x59). Achada: com +0x63 marcado, a original
/// 0x7a81a0 de [0xb7434c] com +0x63, +0x67 e +0x6b; acertada (+0x50 = 2), desliga a marca de contrato acertado (0x8c5430);
/// tira da lista (0x68d1f0) e o contador de ofertas da pessoa (+8 do registro de 79 bytes) desce.
pub(crate) unsafe extern "thiscall" fn remove_contract_offer_for_transfer(this: usize, pessoa: usize, vaga: i32) {
    let l = read_usize(this + 0x34);
    if l == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut x = proximo(read_usize(this + 0x34));
    while x != 0 {
        if read_usize(x) == read_usize(pessoa) && read_usize(x + 0x59) as i32 == vaga {
            break;
        }
        x = proximo(read_usize(this + 0x34));
    }
    if x == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_4 oferta tirada pela vaga");
    if read_usize(x + 0x63) as i32 != -1 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_PLAYER_REGEN_ADD_SPARE_NON_PLAYER_RECORD);
        f(read_usize(G_PLAYER_REGEN), read_usize(x + 0x63) as u32, read_usize(x + 0x67) as u32, read_usize(x + 0x6b) as u32);
    }
    if read_u8(x + 0x50) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
        desmarcar(read_usize(G_TRANSFER_MANAGER), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    tirar(read_usize(this + 0x34), x);
    let c = contract_offer_counter(pessoa) as *mut u8;
    *c = (*c).wrapping_sub(1);
}

/// Comparer for `remove_contract_offer_for_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original tira o item da lista (libera): só ela, conferindo o plano feito antes: com a oferta da pessoa
/// pela vaga na lista, o contador desce 1; sem ela, fica.
pub(crate) unsafe extern "thiscall" fn comparar_remove_contract_offer_for_transfer(this: usize, pessoa: usize, vaga: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
    let l = read_usize(this + 0x34);
    let mut achou = false;
    if l != 0 {
        let mut no = read_usize(l + 2);
        let mut n = 0;
        while no != 0 && n < 100000 {
            let x = read_usize(no);
            if read_usize(x) == read_usize(pessoa) && read_usize(x + 0x59) as i32 == vaga {
                achou = true;
                break;
            }
            no = read_usize(no + 6);
            n += 1;
        }
    }
    let antes = read_u8(contract_offer_counter(pessoa));
    o(this, pessoa, vaga);
    let depois = read_u8(contract_offer_counter(pessoa));
    let esperado = if achou { antes.wrapping_sub(1) } else { antes };
    PLACAR.registrar(depois == esperado, || format!("remove_contract_offer_for_transfer(pessoa {:#x}, vaga {}) achou {} contador {} -> {}", read_usize(pessoa), vaga, achou, antes, depois));
}

/// the lower of the desired wages for squad status 1 and 2 at the club, at most 10 million
/// Original: `contract_manager_lowest_wage_demand` (0x4ce450, contract_manager.cpp, stdcall).
///
/// [pt] { 0x4ce450 } (stdcall, ret 8, contract_manager.cpp, quadro SEH sem destrutor): o menor salário que a pessoa pede ao
/// clube: num contrato local (0x85e410), o desejado (0x85fc50 com −1, a situação 1 e depois 2, 0xb), no máximo 10 milhões.
/// Pela tela do contrato.
pub(crate) unsafe extern "stdcall" fn contract_manager_lowest_wage_demand(pessoa: usize, clube: usize) -> i32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let mut local = [0u8; 0x50];
    let c = local.as_mut_ptr() as usize;
    construir(c);
    let mut menor = 0x989680;
    let a = desejado(c, pessoa, clube, -1, 1, 0xb, 0, 0);
    if a < menor {
        menor = a;
    }
    let b = desejado(c, pessoa, clube, -1, 2, 0xb, 0, 0);
    if b < menor { b } else { menor }
}

/// Comparer for `contract_manager_lowest_wage_demand` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia a partir do mesmo estado (o sorteio e o último contrato pedido [0xae1080]); a
/// devolução e a memória.
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_lowest_wage_demand(pessoa: usize, clube: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> i32>(IDX_CONTRACT_MANAGER_LOWEST_WAGE_DEMAND);
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(pessoa, clube), || crate::aleatorio::sem_contar(|| contract_manager_lowest_wage_demand(pessoa, clube)));
    PLACAR.registrar(igual && r == n, || format!("contract_manager_lowest_wage_demand(pessoa {:#x}) original={} {:x?} rust={} {:x?}", read_usize(pessoa), r, mo, n, mr));
    r
}



/// at a computer club: when the value (0x4d0c60) × 1.5 beats the asking price (0x4d1600), lists the player (+0x4e,
/// morale +12, 0x842c00, 0x4d71b0); otherwise a draw gives the 0x10000 grievance, morale drops 12 and the request
/// strength (+0x44) grows; at a human club, the news item (by the original)
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
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(contrato + 4).wrapping_mul(club::SIZE));
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(contrato).wrapping_mul(staff::SIZE));
    if read_usize(pessoa + staff::PLAYER) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if humano(read_usize(G_NEWS), clube) != 0 {
        // ponytail: o clube humano recebe a notícia montada na pilha (0x77bbe0 e a entrega estão desligadas): esse
        // caso fica com a original (que refaz a pergunta 0x7875e0, sem efeito).
        // o registro serve à prova do "Deixar infeliz" do CMRTE (31/10/2026; deixar_infeliz.py): chamada rara
        linha(&format!("pedido de transferencia no clube humano: pessoa {}", read_usize(contrato)));
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HANDLE_TRANSFER_REQUEST)(this, contrato);
    }
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    if read_u8(contrato + contract::SQUAD_STATUS) & 0xf0 != 0x10 {
        let preco: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
        let a = preco(this, clube, 0, pessoa, 0, 1, 1);
        let v = valor(this, pessoa, clube, 0, -1, 0);
        // o 1,5 é o double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: o valor passa do preço
            crate::comparacao::ramo("cps_1 listado pelo clube");
            let m = (contrato + contract::TRANSFER_STATUS) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            moral(pessoa, 0xc, 0);
            let listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
            listas(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), pessoa, 1);
            let listar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRANSFER_LISTED_NEWS);
            listar(this, pessoa, clube);
            return;
        }
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let r = sorteio(read_u8(contrato + 0x44) as i8 as i32 + (read_u8(pessoa + staff::LOYALITY) as i8 as i32 + 5) * 20);
    if r < sorteio(0x32) { // cps1: o sorteio da queixa
        crate::comparacao::ramo("cps_1 queixa do pedido");
        let queixa: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        queixa(contrato, 0x10000, 0);
    }
    crate::comparacao::ramo("cps_1 pedido no clube do computador");
    moral(pessoa, 0xffff_fff4, 0); // cps1: a moral cai 12
    // relidos depois das chamadas, como na original (o `imul` de 8 bits fica com o byte baixo)
    let al = read_u8(pessoa + staff::LOYALITY).wrapping_mul(5).wrapping_add(0x38);
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
pub(crate) unsafe extern "stdcall" fn is_contract_staff_job(cargo: u32) -> u32 {
    matches!(cargo as u8, 0 | 6..=0xd | 0xf) as u32
}

/// 0 when the job (+0x3d) is a board job (1 to 4) at a club (+0x39); otherwise 1
/// Original: `staff_is_not_board_member` (0x4cb960, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4cb960 } (stdcall, ret 4, contract_manager.cpp): `staff_is_not_board_member(pessoa)`: 0 se o cargo (+0x3d) é de
/// diretoria (1 a 4) num clube (+0x39); senão 1.
pub(crate) unsafe extern "stdcall" fn staff_is_not_board_member(pessoa: usize) -> u32 {
    let cargo = read_u8(pessoa + staff::JOB_FOR_CLUB);
    if (1..=4).contains(&cargo) && read_usize(pessoa + staff::CLUB_JOB) != 0 {
        return 0;
    }
    1
}

// ponytail: sorteia (0x90cfc0), põe na lista (0x842c00, 0x4d71b0), muda moral e queixas (que podem chamar o pedido de
// novo): só a original; a cópia leva os ramos.
/// Comparer for `handle_transfer_request` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_handle_transfer_request(this: usize, contrato: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HANDLE_TRANSFER_REQUEST)(this, contrato)
}

/// Comparer for `is_contract_staff_job` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "stdcall" fn comparar_is_contract_staff_job(cargo: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> u32>(IDX_IS_CONTRACT_STAFF_JOB)(cargo);
    let n = is_contract_staff_job(cargo);
    PLACAR.registrar(r == n, || format!("is_contract_staff_job({}) original={} rust={}", cargo as u8, r, n));
    r
}

/// Comparer for `staff_is_not_board_member` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "stdcall" fn comparar_staff_is_not_board_member(pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_STAFF_IS_NOT_BOARD_MEMBER)(pessoa);
    let n = staff_is_not_board_member(pessoa);
    PLACAR.registrar(r == n, || format!("staff_is_not_board_member(pessoa {}, cargo {}) original={} rust={}", read_usize(pessoa), read_u8(pessoa + staff::JOB_FOR_CLUB), r, n));
    r
}



/// Calls one of the game's date comparison functions f (>, <, ==) on date a with date b by value.
///
/// [pt] As comparações de data do jogo (thiscall na data `a`, a outra por valor): 0x456740 `>`, 0x456770 `<`, 0x4a7f20 `==`.
pub(crate) unsafe fn call_date_compare(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, read_u32(b), read_u32(b + 4))
}

/// the qsort order of the decision dates: by date and, on the same day, by the time slice (signed)
/// Original: `compare_decision_dates` (0x4dc910, contract_manager.cpp, cdecl).
///
/// [pt] { 0x4dc910 } (cdecl, contract_manager.cpp): `compare_decision_dates(a, b)`: a ordem do qsort das datas de decisão
/// (entradas de 14 bytes: +0 pessoa, +4 data, +0xc fatia do dia, +0xd tipo): a data de `a` antes (0x456770): −1;
/// depois (0x456740): 1; no mesmo dia, a fatia de `a` menos a de `b` (com sinal). Quente (o qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_decision_dates(a: usize, b: usize) -> i32 {
    if call_date_compare(0x456770, a + 4, b + 4) != 0 {
        return -1; // a antes de b
    }
    if call_date_compare(0x456740, a + 4, b + 4) != 0 {
        return 1;
    }
    read_u8(a + 0xc) as i8 as i32 - read_u8(b + 0xc) as i8 as i32
}

/// Comparer for `compare_decision_dates` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_decision_dates(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_DECISION_DATES);
    let r = o(a, b);
    let n = compare_decision_dates(a, b);
    PLACAR.registrar(r == n, || format!("compare_decision_dates original={} rust={}", r, n));
    r
}

/// consumes the decision dates due by the given date and slice: each one becomes −1 and, for type 3, the person decides
/// on the contract offers; in DESLIGADAS since 2026-10-30: the detour alone (without running the copy) made the career
/// in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the
/// stack residue a later original reads; the copy stays in the code and does not run
/// Original: `CONTRACT_MANAGER::process_due_decision_dates` (0x4cd9c0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd9c0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::process_due_decision_dates(data, fatia)`:
/// do índice +0x18 até a conta +0x14 do vetor das datas de decisão (+0xc, entradas de 14 bytes, ordenadas pela
/// 0x4dc910): para na primeira data depois de `data` (0x456740) ou, no mesmo dia (0x4a7f20), com a fatia da entrada
/// maior que `fatia`. Cada entrada vencida com pessoa (+0 ≠ −1) fica com −1 e, sendo do tipo 3 (+0xd), a pessoa decide
/// as ofertas de contrato (0x4ce4e0). O índice +0x18 sobe a cada entrada (relido depois da decisão).
pub(crate) unsafe extern "thiscall" fn process_due_decision_dates(this: usize, data: usize, fatia: u32) {
    let n = |c: usize| read_u32(c) as i32;
    if n(this + 0x18) >= n(this + 0x14) {
        return;
    }
    let resolver: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_RESOLVE_CONTRACT_OFFERS);
    loop {
        let e = (read_u32(this + 0xc) as usize).wrapping_add((n(this + 0x18) as usize).wrapping_mul(14));
        if call_date_compare(0x456740, e + 4, data) != 0 {
            return; // ainda não venceu
        }
        if call_date_compare(0x4a7f20, e + 4, data) != 0 && read_u8(e + 0xc) as i8 > fatia as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let pessoa = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            core::ptr::write_unaligned(e as *mut i32, -1);
            if read_u8(e + 0xd) == 3 {
                crate::comparacao::ramo("cpp_1 decidir ofertas");
                resolver(this, pessoa); // a pessoa decide as ofertas
            }
        }
        let i = n(this + 0x18) + 1;
        core::ptr::write_unaligned((this + 0x18) as *mut i32, i);
        if i >= n(this + 0x14) {
            return;
        }
    }
}

/// Comparer for `process_due_decision_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a decisão das ofertas (0x4ce4e0) mexe nos contratos, nas ofertas e no próprio vetor: o plano (as datas
/// lidas antes, sem rodar nada) diz se alguma entrada do tipo 3 com pessoa vence; com ela, só a original. Sem ela (o
/// caso de todo dia), lado a lado pelo índice +0x18 e pelas entradas que vencem (ficam com −1).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_decision_dates(this: usize, data: usize, fatia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_DUE_DECISION_DATES);
    let (i0, n) = (read_u32(this + 0x18) as i32, read_u32(this + 0x14) as i32);
    let mut i = i0;
    let mut decide = false;
    while i < n {
        let e = (read_u32(this + 0xc) as usize).wrapping_add((i as usize).wrapping_mul(14));
        if call_date_compare(0x456740, e + 4, data) != 0
            || (call_date_compare(0x4a7f20, e + 4, data) != 0 && read_u8(e + 0xc) as i8 > fatia as u8 as i8) {
            break;
        }
        decide |= read_u32(e) as i32 != -1 && read_u8(e + 0xd) == 3;
        i += 1;
    }
    if decide {
        return o(this, data, fatia);
    }
    let mut reg = vec![(this + 0x18, 4)];
    if i > i0 {
        reg.push(((read_u32(this + 0xc) as usize).wrapping_add((i0 as usize).wrapping_mul(14)), ((i - i0) as usize) * 14));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data, fatia), || process_due_decision_dates(this, data, fatia));
    PLACAR.registrar(igual, || format!("process_due_decision_dates(fatia {}) {:x?} x {:x?}", fatia as u8 as i8, mo, mr));
}

/// copies the club's open contract offers (outside states 5 and 6) into a new array of 111 bytes per offer and returns
/// how many
/// Original: `CONTRACT_MANAGER::get_current_contract_offers` (0x4ca300, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4ca300 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_current_contract_offers(clube, saida)`:
/// percorre a lista das ofertas (+0x34; cursor reposto, 0x68d1b0) e copia cada oferta do clube (+4 = id) fora dos
/// estados 5 e 6 (+0x35 & 0x3f) para um vetor novo de 111 bytes por oferta, que cresce de 20 em 20 (realloc 0x945667
/// com a capacidade em byte com sinal). Grava o vetor em `saida` e devolve quantas (byte). A original confere a lista
/// (+0x34), não o vetor, depois do realloc: só com a lista nula vai à caixa "cannot allocate memory" (linha 0x64c) e
/// devolve 0 com a saída nula. Quem a chama (0x598a06, o orçamento de transferências) libera o vetor.
pub(crate) unsafe extern "thiscall" fn get_current_contract_offers(this: usize, clube: usize, saida: usize) -> u8 {
    let mut n: i8 = 0;
    let mut cap: i8 = 0;
    let mut vetor: usize = 0;
    let lista = read_u32(this + 0x34) as usize;
    if lista != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32(lista + 2));
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let mut item = proximo(lista);
        while item != 0 {
            let estado = read_u8(item + 0x35) & 0x3f;
            if read_u32(item + 4) == read_u32(clube) && estado != 5 && estado != 6 {
                if cap == n {
                    cap = cap.wrapping_add(20);
                    vetor = realocar(vetor, (cap as i32 * 111) as u32);
                    if read_u32(this + 0x34) == 0 {
                        show_cannot_create_space_box(0x64c, 0x9a8220); // "cannot allocate memory"
                        core::ptr::write_unaligned(saida as *mut u32, 0);
                        return 0;
                    }
                }
                crate::comparacao::ramo("cpp_1 oferta do clube");
                core::ptr::copy(item as *const u8, vetor.wrapping_add((n as i32 * 111) as usize) as *mut u8, 111);
                n = n.wrapping_add(1);
            }
            item = proximo(read_u32(this + 0x34) as usize);
        }
    }
    core::ptr::write_unaligned(saida as *mut u32, vetor as u32);
    n as u8
}

/// Comparer for `get_current_contract_offers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca o vetor que quem chama libera: roda só ela e confere o plano feito antes (as ofertas
/// do clube lidas da lista, sem alocar): a conta e os 111 bytes de cada uma, na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_get_current_contract_offers(this: usize, clube: usize, saida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_CURRENT_CONTRACT_OFFERS);
    let lista = read_u32(this + 0x34) as usize;
    if lista == 0 || clube == 0 || saida == 0 {
        return o(this, clube, saida);
    }
    let mut plano: Vec<u8> = Vec::new();
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut item = proximo(lista);
    while item != 0 {
        let estado = read_u8(item + 0x35) & 0x3f;
        if read_u32(item + 4) == read_u32(clube) && estado != 5 && estado != 6 {
            plano.extend_from_slice(core::slice::from_raw_parts(item as *const u8, 111));
        }
        item = proximo(lista);
    }
    let r = o(this, clube, saida);
    let vetor = read_u32(saida) as usize;
    let k = plano.len() / 111;
    let ok = r as usize == k && (k == 0 || core::slice::from_raw_parts(vetor as *const u8, k * 111) == &plano[..]);
    PLACAR.registrar(ok, || format!("get_current_contract_offers(clube {:x}) original {} ofertas, plano {}", clube, r, k));
    r
}


/// Volatile byte read (so a missing main contract crashes like the original).
pub(crate) unsafe fn read_byte_volatile(a: usize) -> u8 {
    // ponytail: leitura volátil porque a original lê [0 + 0x35] quando o contrato principal falta (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}


/// Address of the person's 79-byte staff state record (+0 main contract index, +4 loan contract index; -1 none).
///
/// [pt] O registro de 79 bytes da pessoa em [0xae2d0c] (+0 o contrato principal, +4 o de empréstimo; −1 sem).
pub(crate) unsafe fn staff_state_entry(pessoa: usize) -> usize {
    (read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(0x4f))
}

/// Entry k (0x50 bytes) of the contract array starting at vetor.
///
/// [pt] A entrada `k` (0x50 bytes) do vetor de contratos que começa em `vetor`.
pub(crate) fn contract_at_index(vetor: u32, k: i32) -> usize {
    (vetor as usize).wrapping_add((k as usize).wrapping_mul(0x50))
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
pub(crate) unsafe fn main_contract_lookup(pessoa: usize) -> usize {
    let c = if read_u32(pessoa) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32(staff_state_entry(pessoa)) as i32;
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
pub(crate) unsafe fn show_contract_error_box(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    write_u32(0xb67a34, 0);
}

// As rotinas do banco que as três chamam (pelo endereço; as portadas passam pelo desvio).
/// Calls the club's linked A/B team lookup (0x540a50); sets whether the club is the A team.
pub(crate) unsafe fn contract_manager_linked_b_team(clube: usize, eh_a: *mut u32) -> usize {
    let f: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    f(clube, eh_a, 1)
}
/// Calls the exe's Staff set-squad-flags routine for the person.
pub(crate) unsafe fn set_squad_flags(pessoa: usize, elenco: u32) {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    f(pessoa, elenco);
}
/// Calls Tactics remove_player_from_selection on G_TACTICS for the club and person.
pub(crate) unsafe fn remove_from_tactics_selection(clube: usize, pessoa: usize, a: u32, b: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    f(read_u32(G_TACTICS) as usize, clube, pessoa, a, b);
}
/// Calls PlayerStats archive_player_stats on G_PLAYER_STATS for the person and club.
pub(crate) unsafe fn contract_manager_archive_player_stats(pessoa: usize, a4: u32, clube: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(FN_PLAYERSTATS_ARCHIVE_PLAYER_STATS);
    f(read_u32(G_PLAYER_STATS) as usize, pessoa, -1, 0, a4, clube, 0);
}
/// Calls Database remove_staff_from_club (club, person, date, flags).
pub(crate) unsafe fn contract_manager_remove_staff_from_club(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    f(clube, pessoa, data, desligar, como_jogador);
}
/// Calls Database add_staff_to_club (club, person, job, date, flags).
pub(crate) unsafe fn add_staff_to_club(clube: usize, pessoa: usize, cargo: u32, data: usize, reduzir: u32, gravar: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
    f(clube, pessoa, cargo, data, reduzir, gravar);
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
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    folha(financas(read_u32(G_FINANCE_TABLE) as usize, clube));
}
/// Assigns date origem to destino with the date operator (0x417c10).
pub(crate) unsafe fn contract_manager_assign_date(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut ret = [0u8; 8];
    copiar(destino, &mut ret, origem);
}

/// moves the person to the loan club (lists, tactics, squad as a player), marks the reserve bit in the main contract
/// and stores the loan contract in her slot (the existing one, a free one or at the end of the array); recalculates the
/// wage bills
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
    let construir: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    construir(&mut guardada);
    let pessoa = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contrato) as usize).wrapping_mul(staff::SIZE));
    let ficha = if pessoa == 0 { write_u32(G_CONTRACT_LOOKUP_RESULT, 0); 0 } else { main_contract_lookup(pessoa) };
    if ficha == 0 {
        return;
    }
    let dono = contract_manager_club_by_id(read_u32(ficha + 4));
    if dono == 0 {
        return;
    }
    contract_manager_assign_date(guardada.as_mut_ptr() as usize, pessoa + staff::DATE_JOINED_CLUB);
    let novo = contract_manager_club_by_id(read_u32(contrato + 4));
    contract_manager_archive_player_stats(pessoa, 1, read_u32(novo) as i32);
    let principal = if pessoa != 0 && (read_u32(pessoa) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(pessoa)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        show_contract_error_box(0x16a9);
        return;
    }
    crate::comparacao::ramo("cpp_2 assinar emprestimo");
    let m = principal + 0x35;
    let reserva = if club_has_reserve_squad(dono) != 0 {
        read_byte_volatile(pessoa + staff::PLAYING_SQUAD) & 1 == 0
    } else if contract_manager_linked_b_team(dono, core::ptr::null_mut()) != 0 {
        dono as u32 != read_u32(pessoa + staff::CLUB_JOB)
    } else {
        false
    };
    let cl = read_byte_volatile(m);
    *(m as *mut u8) = if reserva { cl | 0x40 } else { cl & 0xbf };
    let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_REMOVE_VIRTUAL_PLAYERS);
    avisar(read_u32(G_VIRTUAL_STAFF) as usize, novo, 0);
    remove_from_tactics_selection(read_u32(pessoa + staff::CLUB_JOB) as usize, pessoa, 0, 1);
    remove_from_tactics_selection(read_u32(pessoa + staff::CLUB_JOB) as usize, pessoa, 1, 1);
    add_staff_to_club(novo, pessoa, 0xb, G_CURRENT_DATE, 1, 0); // o jogador no clube novo
    contract_manager_assign_date(pessoa + staff::DATE_JOINED_CLUB, guardada.as_ptr() as usize);
    let emp = read_u32(staff_state_entry(pessoa) + 4) as i32;
    if emp > -1 {
        // ponytail: a original copia em linha, igual ao STAFF_CONTRACT::operator= (0x4cc810, portado)
        contract_manager_operator(contract_at_index(read_u32(this), emp), contrato);
    } else {
        let n = read_u32(this + 8) as i32;
        let vetor = read_u32(this);
        let mut i = 0i32;
        while i < n {
            if read_u32(contract_at_index(vetor, i)) as i32 == -1 {
                contract_manager_operator(contract_at_index(vetor, i), contrato);
                write_u32(staff_state_entry(pessoa) + 4, i as u32);
                return; // a original sai sem refazer as folhas
            }
            i += 1;
        }
        let cap = read_u32(this + 4) as i32;
        if n >= cap {
            let cap = cap + 100;
            write_u32(this + 4, cap as u32);
            let novo_vetor = if vetor != 0 {
                let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
                realocar(vetor as usize, (cap * 0x50) as u32)
            } else {
                let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
                alocar((cap * 0x50) as u32)
            };
            write_u32(this, novo_vetor as u32);
            if novo_vetor == 0 {
                show_contract_error_box(0x1708);
            }
        }
        let n = read_u32(this + 8);
        write_u32(staff_state_entry(pessoa) + 4, n);
        let destino = contract_at_index(read_u32(this), n as i32);
        write_u32(this + 8, n + 1);
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_OPERATOR_3D);
        copiar(destino, contrato);
    }
    update_club_wage_bill(dono);
    update_club_wage_bill(novo);
}

/// Comparer for `sign_loan_contract` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_sign_loan_contract(this: usize, contrato: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e realoca o vetor: só a original; a cópia prova no modo Rust
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SIGN_LOAN_CONTRACT)(this, contrato)
}

/// takes the person out of the loan club (lists, disputes, tactics, squad, shortlist), frees the loan contract,
/// recalculates the wage bills and returns her to the parent club with the squad mark
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
pub(crate) unsafe extern "thiscall" fn end_loan(this: usize, pessoa: usize) {
    let mut clube = read_u32(pessoa + staff::CLUB_JOB) as usize;
    if clube == 0 {
        show_contract_error_box(0x1729);
        return;
    }
    let mut eh_a = 0u32; // só lida quando a 0x540a50 a grava
    if contract_manager_linked_b_team(clube, &mut eh_a) != 0 && eh_a == 0 {
        clube = contract_manager_linked_b_team(clube, core::ptr::null_mut()); // o time A
    }
    let principal = if pessoa != 0 && (read_u32(pessoa) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(pessoa)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        show_contract_error_box(0x1735);
        return;
    }
    crate::comparacao::ramo("cpp_2 encerrar emprestimo");
    let ficha = contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), read_u32(staff_state_entry(pessoa)) as i32);
    write_u32(G_CONTRACT_LOOKUP_RESULT, ficha as u32);
    let dono = if ficha == 0 { 0 } else { contract_manager_club_by_id(read_u32(ficha + 4)) };
    contract_manager_archive_player_stats(pessoa, 0, read_u32(dono) as i32);
    let disputas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_RESTORE_HELD_BANS);
    disputas(read_u32(G_DISCIPLINE) as usize, pessoa);
    remove_from_tactics_selection(read_u32(pessoa + staff::CLUB_JOB) as usize, pessoa, 0, 1);
    remove_from_tactics_selection(read_u32(pessoa + staff::CLUB_JOB) as usize, pessoa, 1, 1);
    let emp = read_u32(staff_state_entry(pessoa) + 4) as i32;
    if emp != -1 {
        write_u32(contract_at_index(read_u32(this), emp), u32::MAX);
        write_u32(staff_state_entry(pessoa) + 4, u32::MAX);
    }
    contract_manager_remove_staff_from_club(read_u32(pessoa + staff::CLUB_JOB) as usize, pessoa, 0, 1, 0);
    update_club_wage_bill(clube);
    let f = main_contract_lookup(pessoa);
    update_club_wage_bill(if f == 0 { 0 } else { contract_manager_club_by_id(read_u32(f + 4)) });
    let lista = read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize;
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    if entrada(lista, clube, pessoa, 0, 0) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        tirar(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + 0x856) as usize, pessoa, clube, 4, 0);
    }
    let f = main_contract_lookup(pessoa);
    let volta = if f == 0 { 0 } else { contract_manager_club_by_id(read_u32(f + 4)) };
    write_u32(pessoa + staff::CLUB_JOB, volta as u32); // a pessoa volta ao dono
    if club_has_reserve_squad(volta) != 0 {
        set_squad_flags(pessoa, if read_byte_volatile(principal + 0x35) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let parceiro = contract_manager_linked_b_team(read_u32(pessoa + staff::CLUB_JOB) as usize, core::ptr::null_mut());
        set_squad_flags(pessoa, 1);
        if parceiro != 0 {
            for k in 0..0x32 {
                if read_u32(parceiro + 0xd7 + 4 * k) as usize == pessoa {
                    write_u32(pessoa + staff::CLUB_JOB, parceiro as u32);
                }
            }
        }
    }
    if read_byte_volatile(pessoa + staff::SQUAD_SELECTED_FOR) == 0x10 {
        *((pessoa + staff::SQUAD_SELECTED_FOR) as *mut u8) = 4;
    }
}

/// Comparer for `end_loan` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_end_loan(this: usize, pessoa: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e nas disputas (0x556570 libera itens): só a original
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_END_LOAN)(this, pessoa)
}

/// moves the person between the club's first team and reserves or between the club and its partner (A/B side), taking
/// the loan contract into account; 0 with no room at the destination
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
pub(crate) unsafe extern "thiscall" fn toggle_squad_placement(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    if clube == 0 {
        return 1;
    }
    if saida != 0 {
        write_u32(saida, 0);
    }
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let l = lugar(this, pessoa, clube);
    if l == 0 {
        return 1;
    }
    let na_faixa = |p: usize| p != 0 && (read_u32(p) as i32) < read_u32(G_STAFF_COUNT) as i32;
    // o contrato de empréstimo (no CONTRACT_MANAGER global) e o principal (no próprio vetor; 0 sem)
    let emprestimo = |p: usize| {
        let k = read_u32(staff_state_entry(p) + 4) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    let principal = |p: usize| {
        let k = read_u32(staff_state_entry(p)) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(this), k) }
    };
    if l == 1 || l == 2 {
        if na_faixa(pessoa) {
            let e = emprestimo(pessoa);
            if e != 0 && read_u32(e + 4) != read_u32(clube) {
                let m = principal(pessoa) + 0x35;
                let cl = read_byte_volatile(m);
                *(m as *mut u8) = if l == 1 { cl & 0xbf } else { cl | 0x40 };
                return 1;
            }
        }
        crate::comparacao::ramo("cpp_2 trocar no clube");
        set_squad_flags(pessoa, l as u32); // 1 principal, 2 reserva
        remove_from_tactics_selection(clube, pessoa, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (contract_manager_linked_b_team(clube, core::ptr::null_mut()), clube)
    } else {
        (clube, contract_manager_linked_b_team(clube, core::ptr::null_mut()))
    };
    if saida != 0 {
        write_u32(saida, destino as u32);
    }
    let vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    // ponytail: a original empurra o cargo num registrador com lixo nos bytes altos; a 0x53b9a0 lê só o byte
    if vaga(destino, read_byte_volatile(pessoa + staff::JOB_FOR_CLUB) as u32, pessoa) == 0 {
        return 0;
    }
    if na_faixa(pessoa) {
        let e = emprestimo(pessoa);
        if e != 0 && read_u32(e + 4) == read_u32(clube) {
            // emprestada a este clube: passa do outro ao destino com o cargo
            contract_manager_archive_player_stats(pessoa, 0, -1);
            contract_manager_remove_staff_from_club(origem, pessoa, 0, 1, 1);
            remove_from_tactics_selection(origem, pessoa, 1, 1);
            remove_from_tactics_selection(origem, pessoa, 0, 1);
            // ponytail: o cargo vai com os bytes altos do eax da 0x896c60; a 0x53bd10 lê só o byte
            add_staff_to_club(destino, pessoa, read_byte_volatile(pessoa + staff::JOB_FOR_CLUB) as u32, 0, 1, 1);
            return 1;
        }
        if na_faixa(pessoa) && e != 0 {
            let m = principal(pessoa) + 0x35;
            let cl = read_byte_volatile(m);
            *(m as *mut u8) = if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf };
            crate::comparacao::ramo("cpp_2 trocar emprestado");
            contract_manager_remove_staff_from_club(origem, pessoa, 0, 0, 1);
            add_staff_to_club(destino, pessoa, 0xb, 0, 0, 1);
            return 1;
        }
    }
    crate::comparacao::ramo("cpp_2 trocar de time");
    contract_manager_archive_player_stats(pessoa, 0, -1);
    contract_manager_remove_staff_from_club(origem, pessoa, 0, 1, 0);
    remove_from_tactics_selection(origem, pessoa, 1, 1);
    remove_from_tactics_selection(origem, pessoa, 0, 1);
    // ponytail: o cargo vai com os bytes altos do edx deixados pela 0x896c60; a 0x53bd10 lê só o byte
    add_staff_to_club(destino, pessoa, read_byte_volatile(pessoa + staff::JOB_FOR_CLUB) as u32, 0, 1, 0);
    1
}

/// Comparer for `toggle_squad_placement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_toggle_squad_placement(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    // ponytail: mexe no elenco, nas táticas e nas listas: só a original; a cópia prova no modo Rust
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_TOGGLE_SQUAD_PLACEMENT)(this, pessoa, clube, saida)
}



/// Volatile byte read (so a missing contract club crashes like the original).
pub(crate) unsafe fn read_u8_volatile(a: usize) -> u8 {
    // ponytail: volátil porque a original lê [0 + 0x53] quando falta o clube do contrato (cai como ela)
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



/// whether the person fits the job (6 to 15) by age, current job and the player and non-player records; null person: 0
/// Original: `staff_can_take_job` (0x4cb990, contract_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x4cb990 } (stdcall, ret 8, contract_manager.cpp): `staff_can_take_job(cargo, pessoa)`: pela tabela de salto
/// 0x4cbaf4 (cargo − 6, 0..9; fora: 0); 7, 12 e 14: 0; pessoa nula: 0 (o regen de não jogador 0x7ab9f2 pergunta sempre
/// com nula). 6: com 30 anos (+0x18) ou mais, não jogador (+0x69), acima de 36, ou com +0x57 abaixo de 10 e +0x2f do
/// jogador 12 ou mais: 1; senão, não jogador no cargo 8, 6 ou 5. 8: de 30 anos, não jogador ou acima de 36 ou (+0x2f ≥ 12
/// e +0x57 < 10) fora dos cargos 5 e 10: 1; senão como o 6. 9: não jogador no cargo 9. 10: não jogador no cargo 10 ou com
/// +0x28 = 10. 11: jogador (+0x61). 13 e 15: jogador e não jogador, ou acima de 34, ou de 30 anos com +0x2f ≥ 12.
pub(crate) unsafe extern "stdcall" fn staff_can_take_job(cargo: u32, pessoa: usize) -> u32 {
    let k = (cargo as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if pessoa == 0 {
        crate::comparacao::ramo("cpp_4 pessoa nula");
        return 0; // pessoa nula
    }
    let p = pessoa;
    let idade = read_u8_volatile(p + 0x18) as i8;
    let nao_jogador = read_u32(p + 0x69);
    let funcao = read_u8_volatile(p + 0x3d);
    let forca = || read_u8_volatile(read_u32(p + 0x61) as usize + 0x2f) as i8; // +0x2f do jogador
    // o fim comum (0x4cb9db): não jogador nos cargos 8, 6 ou 5
    let comum = || nao_jogador != 0 && (funcao == 8 || funcao == 6 || funcao == 5);
    let r = match k {
        0 => {
            if idade >= 0x1e {
                if nao_jogador != 0 || idade > 0x24 {
                    return 1;
                }
                if (read_u8_volatile(p + 0x57) as i8) < 0xa && forca() >= 0xc {
                    return 1;
                }
            }
            comum()
        }
        2 => {
            if idade < 0x1e {
                comum()
            } else if nao_jogador == 0 && idade <= 0x24 && (forca() < 0xc || read_u8_volatile(p + 0x57) as i8 >= 0xa) {
                comum()
            } else if funcao == 5 || funcao == 0xa {
                comum()
            } else {
                true
            }
        }
        3 => nao_jogador != 0 && funcao == 9,
        4 => nao_jogador != 0 && (funcao == 0xa || read_u8_volatile(p + 0x28) == 0xa),
        5 => read_u32(p + 0x61) != 0,
        _ => {
            // 13 e 15
            let jogador = read_u32(p + 0x61) as usize;
            jogador != 0 && (nao_jogador != 0 || idade > 0x22 || (idade >= 0x1e && read_u8_volatile(jogador + 0x2f) as i8 >= 0xc))
        }
    };
    r as u32
}

/// Comparer for `staff_can_take_job` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_staff_can_take_job(cargo: u32, pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(IDX_STAFF_CAN_TAKE_JOB)(cargo, pessoa);
    let n = staff_can_take_job(cargo, pessoa);
    PLACAR.registrar(r == n, || format!("staff_can_take_job(cargo {}, pessoa {:x}) original={} rust={}", cargo as u8 as i8, pessoa, r, n));
    r
}

/// Days (0..400) from the date (day, unsigned year) to today (G_CURRENT_DATE), 365-day years.
///
/// [pt] Os dias (0..400) desde a data (dia +0, ano +2 sem sinal) até hoje ([0xae2c90]), com o ano de 365 dias.
pub(crate) unsafe fn contract_manager_days_since_date(dia: usize, ano: usize) -> i32 {
    let hoje = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16) as i32;
    let a = core::ptr::read_unaligned(ano as *const u16) as i32;
    let d = (hoje - a) * 365 - read_i16(dia) as i32 + read_i16(G_CURRENT_DATE) as i32;
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
pub(crate) unsafe fn match_morale_drop(pessoa: usize, clube: usize, contrato: usize) -> Option<i8> {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return None;
    }
    if pessoa == read_u32(clube + club::MANAGER) as usize || pessoa == read_u32(clube + club::ASSISTANT_MANAGER) as usize || read_u8_volatile(pessoa + staff::JOB_FOR_CLUB) != 0xb {
        return None;
    }
    let a = contract_manager_days_since_date(pessoa + staff::DATE_JOINED_CLUB, pessoa + 0x40);
    let b = contract_manager_days_since_date(contrato + contract::CONTRACT_EXPIRES, contrato + 0x2f);
    let idade = read_u8_volatile(pessoa + staff::YEAR_OF_BIRTH) as i8;
    let jogador = read_u32(pessoa + staff::PLAYER) as usize;
    let dif = read_i16(jogador + 0xb) as i32 - read_i16(clube + club::REPUTATION) as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + idade as f64 * read_f64_at(0x968d90)) - read_f64_at(0x968db8); // 0x968d90 = 417, 0x968db8 = 31680
    let c57 = read_u8_volatile(pessoa + staff::AMBITION) as i8 as i32;
    let teto = read_f32_at(0x968da8) as f64; // 0x968da8 = 15000
    let limitar = |x: f64| if x >= teto { teto } else { x };
    let al = read_u8_volatile(pessoa + staff::INT_APPS);
    if b > 0x190 && idade > 0x17 && idade < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + read_f32_at(0x968db0) as f64); // nunca: b vai até 400; 0x968db0 = 5000
    } else if al != 0 && read_i16(read_u32(pessoa + staff::NATION) as usize + 0x8e) > 0xdac && idade < 0x20 {
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
pub(crate) fn lowered_morale(moral: i8, m: i8) -> i8 {
    let menos = m.wrapping_neg();
    let s = moral as i32 + menos as i32;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        moral.wrapping_add(menos)
    }
}

/// Club of the person's main contract (null if none); stores the contract in G_CONTRACT_LOOKUP_RESULT like the
/// original.
///
/// [pt] O contrato principal da pessoa no CONTRACT_MANAGER global e o clube dele, como a original os refaz (a ficha
/// [0xae1080] gravada); sem contrato o clube é nulo.
pub(crate) unsafe fn club_of_main_contract(pessoa: usize) -> usize {
    let c = if read_u32(pessoa) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(0x4f))) as i32;
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
pub(crate) unsafe fn transfer_window_open(pessoa: usize, clube: usize) -> bool {
    let c1 = club_of_main_contract(pessoa);
    let k = if read_u32(c1 + 0x53) != 0 {
        let c2 = club_of_main_contract(pessoa);
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
        indice(read_u32(c2 + 0x53) as usize) as u8 as i8
    } else {
        // ponytail: a original lê o byte baixo da vaga do argumento `clube` (o ponteiro), não um índice
        clube as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = G_LEAGUE_WORLD_CUP_START_DATE + (k as usize) * 0x48;
    let dia = read_i16(G_CURRENT_DATE);
    let ano = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    let ano_de = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let depois = (dia > read_i16(t) && ano == ano_de(t + 2)) || ano > ano_de(t + 2);
    let antes = (dia < read_i16(t + 0xf) && ano == ano_de(t + 0x11)) || ano < ano_de(t + 0x11);
    depois && antes
}

/// lowers the player's contract morale after a poor match (by rating, reputation, time at the club, age and contract
/// band) and, with very low morale, may add the wants-to-leave or lack-of-games grievance; injured, on loan or retiring
/// clears the grievances
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
pub(crate) unsafe extern "thiscall" fn contract_manager_lower_match_morale(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let Some(m) = match_morale_drop(pessoa, clube, contrato) else {
        return;
    };
    let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let aposentando: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let emprestado = || {
        if read_u32(pessoa) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
            return false;
        }
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(0x4f)) + 4) as i32;
        k >= 0 && (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let idade = read_u8_volatile(pessoa + staff::YEAR_OF_BIRTH) as i8;
    if lesao(read_u32(G_INJURY) as usize, pessoa) != 0 || emprestado() || aposentando(read_u32(G_PLAYER_REGEN) as usize, pessoa) != 0
        || idade < 0x12 || read_u8_volatile(contrato + contract::LEAVING_ON_BOSMAN) != 0 {
        if read_u32(contrato + 0x45) & 4 != 0 {
            apagar(contrato, 4);
        }
        if read_u8_volatile(contrato + 0x45) & 0x10 != 0 {
            apagar(contrato, 0x10);
        }
    } else if m > 5 && (read_u8_volatile(contrato + 0x44) as i8 as i32 - m as i32) < -30 - read_u8_volatile(pessoa + staff::LOYALITY) as i8 as i32 {
        let jogador = read_u32(pessoa + staff::PLAYER) as usize;
        if idade > 0x15 {
            let nota = read_i16(jogador + 0xb);
            if nota > read_i16(clube + club::REPUTATION) && nota > 0x8ca {
                if transfer_window_open(pessoa, clube) {
                    crate::comparacao::ramo("cpp_4 queixa na janela");
                    apagar(contrato, 4);
                    marcar(contrato, 4, 0);
                }
            } else {
                let mut feito = false;
                if lugar(this, pessoa, clube) == 1 && read_i16(jogador + 0xb) > 0x4e2 && idade > 0x17
                    && sortear(read_u8_volatile(pessoa + staff::LOYALITY) as i8 as i32 + 5) == 0 {
                    apagar(contrato, 4);
                    marcar(contrato, 0x10, 0);
                    feito = true;
                }
                if !feito {
                    let pot = read_i16(jogador + 0xd);
                    if (pot as i32) > read_i16(clube + club::REPUTATION) as i32 - 0x5dc && pot > 0x1f4
                        && sortear(read_u8_volatile(pessoa + staff::AMBITION) as i8 as i32) != 0 && (read_u8_volatile(contrato + 0x3d) as i8) < 0
                        && transfer_window_open(pessoa, clube) {
                        apagar(contrato, 4);
                        marcar(contrato, 4, 0);
                    }
                }
            }
        } else if lugar(this, pessoa, clube) == 1 && sortear(read_u8_volatile(pessoa + staff::AMBITION) as i8 as i32) > 5
            && read_i16(jogador + 0xb) > read_i16(clube + club::REPUTATION) && read_i16(jogador + 0xb) > 0x4e2 {
            apagar(contrato, 4);
            marcar(contrato, 0x10, 0);
        }
    }
    let p = (contrato + 0x44) as *mut i8;
    *p = lowered_morale(*p, m); // a moral cai
}

/// Comparer for `contract_manager_lower_match_morale` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as queixas (0x867700) sorteiam e podem abrir pedido de transferência: só a original; o plano (a queda
/// `m`, calculada antes, sem efeito) diz a moral +0x44 que ela deixa (as queixas não a gravam) ou que ela não mexe.
pub(crate) unsafe extern "thiscall" fn comparar_contract_manager_lower_match_morale(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_CONTRACT_MANAGER_LOWER_MATCH_MORALE);
    if contrato == 0 {
        return o(this, pessoa, clube, contrato);
    }
    let antes = read_u8_volatile(contrato + 0x44) as i8;
    let plano = match_morale_drop(pessoa, clube, contrato);
    // apagar uma queixa (0x8674e0) recalcula o contrato (0x867240), que pode regravar a moral: com queixa marcada antes
    // ou queda acima de 5 (os ramos que apagam sempre), só a original
    if read_u8_volatile(contrato + 0x45) & 0x14 != 0 || plano.is_some_and(|m| m > 5) {
        return o(this, pessoa, clube, contrato);
    }
    o(this, pessoa, clube, contrato);
    let depois = read_u8_volatile(contrato + 0x44) as i8;
    let esperada = plano.map_or(antes, |m| lowered_morale(antes, m));
    PLACAR.registrar(depois == esperada, || format!("contract_manager_lower_match_morale(queda {:?}) moral {} -> original {} plano {}", plano, antes, depois, esperada));
}




/// Address of the person's 79-byte staff state record (+0 main contract index, -1 none).
///
/// [pt] A vaga do contrato principal da pessoa: o registro de 79 bytes dela em [0xae2d0c] (+0 o índice do contrato, −1 sem).
pub(crate) unsafe fn main_contract_slot(pessoa: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f))
}

/// frees the person's main contract entry (−1), puts −1 in her slot of the 79-byte record and recalculates the wage
/// bill of the contract's club
/// Original: `CONTRACT_MANAGER::remove_contract` (0x4d0b50, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4d0b50 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract(pessoa)`: com o contrato
/// principal (índice no registro de 79 bytes da pessoa), libera a entrada dele (+0 = −1 no vetor de 0x50 bytes do
/// CONTRACT_MANAGER), põe −1 na vaga da pessoa e refaz a folha de salários (0x598370) das finanças (0x5a0590) do clube
/// do contrato (+4). Sem contrato: nada.
pub(crate) unsafe extern "thiscall" fn remove_contract(this: usize, pessoa: usize) {
    let indice = read_i32(main_contract_slot(pessoa));
    if indice == -1 {
        return;
    }
    let e = read_usize(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let id_clube = read_usize(e + 4);
    write_usize(e, usize::MAX);
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(id_clube.wrapping_mul(club::SIZE));
    crate::comparacao::ramo("cpo_2 tirar contrato");
    write_usize(main_contract_slot(pessoa), usize::MAX); // a pessoa fica sem contrato
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let f = get_club_finance(read_usize(G_FINANCE_TABLE), clube);
    if f != 0 {
        let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
        folha(f);
    }
}

/// resubmits the person's held offers as live offers (with the transfer of the same club, state 0xf becomes 8; without
/// a transfer, with −1), removes each one from the list and returns how many
/// Original: `CONTRACT_MANAGER::resubmit_held_offers` (0x4cf4a0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cf4a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::resubmit_held_offers(id)`: percorre a lista
/// das ofertas guardadas (+0x38) e, para cada uma da pessoa `id` (+0): com a transferência (+0x59, 0x8c61e0 em
/// [0xde1f54]) cujo clube (+0x24) é o da oferta (+0x5d), põe o estado 0xf em 8 e reenvia a oferta (0x4cc490 com a
/// transferência, o clube, o byte +0x61, 1, 1, 0); sem transferência (+0x59 = −1) reenvia com −1, −1. Tira a oferta da
/// lista (0x68d1f0) e conta. O byte +0x61 vai empurrado no registrador inteiro (os bits de cima do que estava nele:
/// o ponteiro lido ou −1). Devolve quantas.
pub(crate) unsafe extern "thiscall" fn resubmit_held_offers(this: usize, id: usize) -> u32 {
    let lista = read_usize(this + 0x38);
    if lista == 0 {
        return 0;
    }
    write_usize(lista + 0xa, read_usize(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let reenviar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
    let mut n = 0u32;
    let mut item = proximo(read_usize(this + 0x38));
    while item != 0 {
        if read_usize(item) == id {
            let t = read_usize(item + 0x59);
            let byte = *((item + 0x61) as *const u8) as u32;
            if t as i32 != -1 {
                let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
                let p = achar(read_usize(G_TRANSFER_MANAGER), t);
                if p != 0 {
                    let q = read_usize(p);
                    if q != 0 && read_usize(q + 0x24) == read_usize(item + 0x5d) {
                        if *((q + 0x2c) as *const u8) == 0xf {
                            *((q + 0x2c) as *mut u8) = 8;
                        }
                        crate::comparacao::ramo("cpo_2 reenviar oferta");
                        reenviar(this, item, t, read_usize(item + 0x5d), (q as u32 & 0xffff_ff00) | byte, 1, 1, 0); // com a transferência
                    }
                }
            } else {
                reenviar(this, item, usize::MAX, usize::MAX, 0xffff_ff00 | byte, 1, 1, 0);
            }
            remover(read_usize(this + 0x38), item);
            n += 1;
        }
        item = proximo(read_usize(this + 0x38));
    }
    n
}

/// the month (0..11) of the date ten days after the end of the season of the club's nation
/// Original: `contract_manager_season_end_month` (0x4d45f0, contract_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x4d45f0 } (stdcall, ret 4, contract_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `contract_manager_season_end_month(clube)`: a data do fim da temporada do clube (Data() 0x54a110 e 0x669bd0 com 0),
/// mais 10 dias (0x54c5b0), e o mês dela (0x54cfb0). Devolve o eax da 0x54cfb0 (o mês 0..11).
pub(crate) unsafe extern "stdcall" fn contract_manager_season_end_month(clube: usize) -> u32 {
    let mut d1 = [0u32; 2];
    let mut d2 = [0u32; 2];
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let fim: unsafe extern "C" fn(usize, u32, *mut u32) = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
    let somar: unsafe extern "thiscall" fn(*mut u32, *mut u32, i32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    construir(d1.as_mut_ptr());
    fim(clube, 0, d1.as_mut_ptr());
    let p = somar(d1.as_mut_ptr(), d2.as_mut_ptr(), 0xa); // dez dias depois
    mes(p)
}

/// Comparer for `remove_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela vaga da pessoa e pela entrada do contrato (a folha 0x598370 refeita duas vezes dá o
/// mesmo, pelo contrato já tirado).
pub(crate) unsafe extern "thiscall" fn comparar_remove_contract(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_CONTRACT);
    let vaga = main_contract_slot(pessoa);
    let indice = read_i32(vaga);
    if indice == -1 {
        return o(this, pessoa);
    }
    let e = read_usize(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let reg = [(vaga, 4), (e, 8)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || remove_contract(this, pessoa));
    PLACAR.registrar(igual, || format!("remove_contract(pessoa {:x}, contrato {}) {:x?} x {:x?}", read_usize(pessoa), indice, mo, mr));
}

/// Comparer for `resubmit_held_offers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original reenvia (0x4cc490, ofertas e transferências) e tira as ofertas da lista (libera): roda só ela
/// e confere o plano feito antes: devolve quantas ofertas da pessoa havia na lista.
pub(crate) unsafe extern "thiscall" fn comparar_resubmit_held_offers(this: usize, id: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RESUBMIT_HELD_OFFERS);
    let lista = read_usize(this + 0x38);
    if lista == 0 {
        return o(this, id);
    }
    // o plano: a original recomeça o cursor (+0xa) antes de percorrer
    write_usize(lista + 0xa, read_usize(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut esperado = 0u32;
    let mut item = proximo(lista);
    while item != 0 {
        if read_usize(item) == id {
            esperado += 1;
        }
        item = proximo(lista);
    }
    let r = o(this, id);
    PLACAR.registrar(r == esperado, || format!("resubmit_held_offers(id {:x}) devolveu {}, esperava {}", id, r, esperado));
    r
}

/// Comparer for `contract_manager_season_end_month` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (datas e a 0x669bd0 só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_season_end_month(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_CONTRACT_MANAGER_SEASON_END_MONTH)(clube);
    let n = contract_manager_season_end_month(clube);
    PLACAR.registrar(r == n, || format!("contract_manager_season_end_month({:x}) original={} rust={}", clube, r, n));
    r
}



/// Shows the CONTRACT_MANAGER "Memory Allocation Error / cannot create space" box for a line and text; the game
/// continues.
///
/// [pt] A caixa "cannot create space" do CONTRACT_MANAGER: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de
/// contract_manager.cpp 0x9a8190, `linha`, `texto`) em 0xae24d0; o título "Memory Allocation Error" (0x9a8108)
/// traduzido (0x66f4e0) em 0xae28bc, o corpo 0x9a7ff8 com o texto em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está).
pub(crate) unsafe fn show_cannot_create_space_box(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// the slot in the 0x50-byte contract vector: the first empty one (id −1) or, append only, a new one (grows by 200;
/// without memory, error box and −1)
/// Original: `CONTRACT_MANAGER::get_free_staff_contract_space` (0x4c79a0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c79a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::get_free_staff_contract_space(so_no_fim)`:
/// a vaga do vetor de contratos (registros de 0x50 bytes em [this], capacidade +4, conta +8). Sem `so_no_fim`, a
/// primeira vaga vazia (pessoa +0 = −1); senão (ou sem vaga vazia) a do fim: cheio, cresce de 200 em 200 (realloc
/// 0x945667, ou malloc 0x944e46 sem vetor; sem memória, caixa 0x175 e −1) e a conta sobe. Devolve o índice. Os dois
/// chamadores (a criação dos contratos 0x4c7560 e a 0x4c7950) passam 1.
pub(crate) unsafe extern "thiscall" fn get_free_staff_contract_space(this: usize, so_no_fim: u32) -> i32 {
    if so_no_fim == 0 {
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
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(p, cap.wrapping_mul(0x50) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(cap.wrapping_mul(0x50) as u32)
        };
        write_usize(this, q);
        if q == 0 {
            show_cannot_create_space_box(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = read_usize(this + 8).wrapping_add(1);
    write_usize(this + 8, n); // mais uma vaga
    (n as i32).wrapping_sub(1)
}

/// the appended slot for a new person (hired manager, regen): links the person to the slot in [0xae2d0c], stores the id
/// and generates the initial contract from the value (0x85e960)
/// Original: `CONTRACT_MANAGER::add_staff_contract` (0x4c7950, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c7950 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::add_staff_contract(pessoa, valor)`: a vaga
/// do fim (0x4c79a0 com 1); sem vaga, nada. Liga a pessoa à vaga (o índice +0 do registro de 0x4f bytes dela em
/// [0xae2d0c]), põe o id dela no contrato (+0) e gera o contrato inicial (0x85e960 com o valor). Pela contratação
/// de técnico (0x68f9c0, MANAGER_MANAGER::daily_update) e pelo regen que assina com um clube (0x7a6f10). Devolve o
/// eax da original (o da 0x85e960, ou o índice negativo); os chamadores não o leem.
pub(crate) unsafe extern "thiscall" fn add_staff_contract(this: usize, pessoa: usize, valor: u32) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_FREE_STAFF_CONTRACT_SPACE);
    let i = vaga(this, 1);
    if i < 0 {
        return i as u32;
    }
    crate::comparacao::ramo("cpl_4 contrato da pessoa");
    write_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)), i as usize); // a pessoa aponta a vaga
    let c = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
    write_usize(c, read_usize(pessoa));
    let gerar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GENERATE_INITIAL_CONTRACT);
    gerar(c, pessoa, valor) // o valor da pessoa
}

/// Reminder dates (type 2) a type-1 decision date gets before it, from the contract morale; returns up to two dates and
/// the count.
///
/// [pt] As datas dos lembretes (tipo 2) que a data de decisão de tipo 1 leva antes dela: com o registro de jogador
/// (+0x61), L = moral do contrato (+0x44) + 100 (8 bits; o contrato pela tabela [0xae2d0c], como a 0x4cf5a0
/// inline: sem contrato a original lê o endereço 0x44 e cai, como aqui); L ≤ 0x50 sem lembrete; L > 0x96 um a
/// L × 90 / 200 dias antes; e um a L / 4 dias antes (sem o registro de jogador, L = 100). As datas pela
/// Date::sub_days (0x54c8e0), na ordem da original.
pub(crate) unsafe fn decision_reminder_dates(this: usize, pessoa: usize, data: usize, tipo: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if tipo != 1 {
        return (v, k);
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut l: u8 = 100;
    if read_usize(pessoa + staff::PLAYER) != 0 {
        let id = read_usize(pessoa) as i32;
        let c = if pessoa == 0 || id >= read_usize(G_STAFF_COUNT) as i32 {
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
            let r = menos(data, t.as_mut_ptr(), (l as u32 * 90) / 200);
            v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
            k += 1;
        }
    }
    let mut t = [0u8; 8];
    let r = menos(data, t.as_mut_ptr(), (l as u32 * 50) / 200);
    v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
    k += 1;
    (v, k)
}

/// appends (person, date, time slice, type) to the 14-byte decision date vector (grows by 100); type 1 first adds type
/// 2 reminders from the contract morale
/// Original: `CONTRACT_MANAGER::add_decision_date` (0x4cd6d0, contract_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4cd6d0 } (thiscall, ret 0x10, contract_manager.cpp): `CONTRACT_MANAGER::add_decision_date(pessoa, data, fatia,
/// tipo)`: acrescenta ao vetor de 14 bytes [this+0xc] (capacidade +0x10, conta +0x14; cresce de 100 em 100 quando a
/// conta passa da capacidade − 4, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa 0x103e e nada) a entrada
/// (id da pessoa +0, data +4, fatia do dia +0xc, tipo +0xd). O tipo 1 leva antes os lembretes de tipo 2
/// (`decision_reminder_dates`), sem fatia (+0xc fica como está). Pelas propostas: a decisão da pessoa (0x4cd230, 0x4ce4e0).
/// As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
pub(crate) unsafe extern "thiscall" fn add_decision_date(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    crate::comparacao::ramo("cpl_4 data de decisao");
    if read_usize(this + 0x14) as i32 > (read_usize(this + 0x10) as i32).wrapping_sub(4) {
        let cap = read_usize(this + 0x10).wrapping_add(100);
        write_usize(this + 0x10, cap);
        let p = read_usize(this + 0xc);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(p, cap.wrapping_mul(14) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(cap.wrapping_mul(14) as u32)
        };
        write_usize(this + 0xc, q);
        if q == 0 {
            show_cannot_create_space_box(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = decision_reminder_dates(this, pessoa, data, tipo as u8);
    for d in &lembretes[..k] {
        let n = read_usize(this + 0x14);
        let e = read_usize(this + 0xc).wrapping_add(n.wrapping_mul(14));
        write_usize(e, read_usize(pessoa));
        core::ptr::write_unaligned((e + 4) as *mut [u8; 8], *d);
        *((e + 0xd) as *mut u8) = 2;
        write_usize(this + 0x14, n.wrapping_add(1));
    }
    let n = read_usize(this + 0x14);
    let e = read_usize(this + 0xc).wrapping_add(n.wrapping_mul(14));
    write_usize(e, read_usize(pessoa));
    *((e + 0xc) as *mut u8) = fatia as u8;
    core::ptr::write_unaligned((e + 4) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    *((e + 0xd) as *mut u8) = tipo as u8;
    write_usize(this + 0x14, n.wrapping_add(1)); // a data da decisão
}

// ponytail: as três realocam (e a 0x4c7950 gera o contrato pela 0x85e960); não rodam duas vezes. O comparador monta
// o plano antes, roda só a original e confere; o modo Rust e a carreira provam as cópias.

/// Comparer for `get_free_staff_contract_space` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a vaga esperada (a primeira vazia, ou a do fim com a conta + 1 e a capacidade + 200 se cheio).
pub(crate) unsafe extern "thiscall" fn comparar_get_free_staff_contract_space(this: usize, so_no_fim: u32) -> i32 {
    let (n, cap, p) = (read_usize(this + 8), read_usize(this + 4), read_usize(this));
    let mut livre = -1i32;
    if so_no_fim == 0 {
        for i in 0..(n as i32).max(0) {
            if read_usize(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                livre = i;
                break;
            }
        }
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i32>(IDX_GET_FREE_STAFF_CONTRACT_SPACE)(this, so_no_fim);
    if r == -1 && read_usize(this) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let (er, en, ecap) = if livre >= 0 { (livre, n, cap) } else { (n as i32, n.wrapping_add(1), if n == cap { cap.wrapping_add(200) } else { cap }) };
    PLACAR.registrar(r == er && read_usize(this + 8) == en && read_usize(this + 4) == ecap, || {
        format!("get_free_staff_contract_space({}) devolveu {} (esperava {}), conta {} -> {} (esperava {}), capacidade {} -> {} (esperava {})",
            so_no_fim, r, er, n, read_usize(this + 8), en, cap, read_usize(this + 4), ecap)
    });
    r
}

/// Comparer for `add_staff_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a vaga é a do fim (a conta de antes); confere a ligação da pessoa, o id no contrato e a conta.
pub(crate) unsafe extern "thiscall" fn comparar_add_staff_contract(this: usize, pessoa: usize, valor: u32) -> u32 {
    let n = read_usize(this + 8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ADD_STAFF_CONTRACT)(this, pessoa, valor);
    if read_usize(this) == 0 {
        return r; // sem memória
    }
    let id = read_usize(pessoa);
    let ligada = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f)));
    let no_contrato = read_usize(read_usize(this).wrapping_add(n.wrapping_mul(0x50)));
    PLACAR.registrar(ligada == n && no_contrato == id && read_usize(this + 8) == n.wrapping_add(1), || {
        format!("add_staff_contract(pessoa {}, valor {}) vaga {} (esperava {}), id no contrato {}, conta {} -> {}",
            id, valor, ligada, n, no_contrato, n, read_usize(this + 8))
    });
    r
}

/// Comparador: os lembretes e a entrada esperados antes da original; confere a conta e cada entrada nova.
pub(crate) unsafe extern "thiscall" fn comparar_add_decision_date(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    let n = read_usize(this + 0x14);
    let (lembretes, k) = decision_reminder_dates(this, pessoa, data, tipo as u8);
    let d = core::ptr::read_unaligned(data as *const [u8; 8]);
    let id = read_usize(pessoa);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_DECISION_DATE)(this, pessoa, data, fatia, tipo);
    if read_usize(this + 0xc) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let v = read_usize(this + 0xc);
    let e = |j: usize| v.wrapping_add(n.wrapping_add(j).wrapping_mul(14));
    let data_de = |j: usize| core::ptr::read_unaligned((e(j) + 4) as *const [u8; 8]);
    let mut certo = read_usize(this + 0x14) == n.wrapping_add(k + 1);
    for (j, l) in lembretes[..k].iter().enumerate() {
        certo &= read_usize(e(j)) == id && data_de(j) == *l && *((e(j) + 0xd) as *const u8) == 2;
    }
    certo &= read_usize(e(k)) == id && data_de(k) == d && *((e(k) + 0xc) as *const u8) == fatia as u8 && *((e(k) + 0xd) as *const u8) == tipo as u8;
    PLACAR.registrar(certo, || {
        format!("add_decision_date(pessoa {}, fatia {}, tipo {}) conta {} -> {} (esperava {}), lembretes {:?}, entradas {:?}",
            id, fatia as u8, tipo as u8, n, read_usize(this + 0x14), n + k + 1, &lembretes[..k],
            (0..=k).map(|j| core::ptr::read_unaligned(e(j) as *const [u8; 14])).collect::<Vec<_>>())
    });
}


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
    if c == 0 || read_usize(c + 4) != read_usize(clube) || *((c + 0x1f) as *const u8) != 1 {
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
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let p = parceiro(clube, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        return [0, 0];
    }
    [clube, p]
}

/// in the relegated club and its A/B partner, the 50 at +0xd7 with a pending relegation clause (+0x1f = 1) go to 2 and
/// the news is posted (0x8e3db0, type 3)
/// Original: `CONTRACT_MANAGER::trigger_relegation_clauses` (0x4cd030, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cd030 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::trigger_relegation_clauses(clube,
/// liga)`: no clube rebaixado e no parceiro (time A/B), cada um dos 50 da comissão e elenco (+0xd7) com a cláusula de
/// rebaixamento por acionar no contrato com o clube (+0x1f = 1) passa a 2 e sai a notícia (0x8e3db0 sobre
/// [0xde1f54] com o tipo 3).
pub(crate) unsafe extern "thiscall" fn trigger_relegation_clauses(this: usize, clube: usize, liga: usize) {
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_POST_RELEASE_CLAUSE_ACTIVATED_NEWS);
    for c in club_and_b_team(clube) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = read_usize(c + 0xd7 + k * 4);
            let contrato = contract_with_relegation_clause(this, c, p);
            if contrato != 0 {
                crate::comparacao::ramo("cpk_3 clausula de rebaixamento");
                *((contrato + contract::RELEGATION_RC) as *mut u8) = 2; // a cláusula acionada
                noticia(read_usize(G_TRANSFER_MANAGER), p, liga, 3, contrato);
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

/// the first offer of the person to the club (+0x34): returns the record reservation (0x7a81a0), unmarks the agreed one
/// (+0x50 = 2), leaves the list and the pending count drops by one
/// Original: `CONTRACT_MANAGER::remove_contract_offer` (0x4cca70, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cca70 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer(pessoa, clube)`: a
/// primeira oferta da lista (+0x34) da pessoa ao clube (ids): devolve a reserva de registro (+0x63 ≠ −1: 0x7a81a0
/// sobre [0xb7434c] com +0x67, +0x6b), tira a marca de contrato acertado se ela estava acertada (+0x50 = 2: 0x8c5430),
/// sai da lista (0x68d1f0) e as pendentes da pessoa (+8 do registro de 79 bytes em [0xae2d0c]) descem uma.
pub(crate) unsafe extern "thiscall" fn remove_contract_offer(this: usize, pessoa: *const u32, clube: *const u32) {
    let (c, p) = (*clube as usize, *pessoa as usize);
    let l = read_usize(this + 0x34);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = proximo(l);
    while e != 0 && !(read_usize(e) == p && read_usize(e + 4) == c) {
        e = proximo(l);
    }
    if e == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_3 tirar oferta de contrato");
    let reserva = read_usize(e + 0x63);
    if reserva as i32 != -1 {
        let devolver: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_REGEN_ADD_SPARE_NON_PLAYER_RECORD);
        devolver(read_usize(G_PLAYER_REGEN), reserva, read_usize(e + 0x67), read_usize(e + 0x6b));
    }
    if *((e + 0x50) as *const u8) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
        desmarcar(read_usize(G_TRANSFER_MANAGER), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    tirar(l, e);
    let n = (read_usize(G_STAFF_STATE_TABLE).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *mut u8;
    *n = (*n).wrapping_sub(1); // uma pendente a menos
}


/// Comparer for `trigger_relegation_clauses` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os contratos que a cópia acionaria (antes); a original aciona e dá as notícias (não roda duas vezes);
/// confere que todos ficaram com 2.
pub(crate) unsafe extern "thiscall" fn comparar_trigger_relegation_clauses(this: usize, clube: usize, liga: usize) {
    let mut alvos = Vec::new();
    for c in club_and_b_team(clube) {
        if c != 0 {
            alvos.extend((0..50).map(|k| contract_with_relegation_clause(this, c, read_usize(c + 0xd7 + k * 4))).filter(|&x| x != 0));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_TRIGGER_RELEGATION_CLAUSES)(this, clube, liga);
    let fora: Vec<usize> = alvos.iter().copied().filter(|&x| *((x + 0x1f) as *const u8) != 2).collect();
    PLACAR.registrar(fora.is_empty(), || format!("trigger_relegation_clauses({:x}) {} contratos, sem acionar {:x?}", clube, alvos.len(), fora));
}

/// Comparador: a lista esperada (sem a primeira oferta da pessoa ao clube) e as pendentes antes; a original tira (e
/// libera o item: nada dele é lido depois); confere os itens e o byte.
pub(crate) unsafe extern "thiscall" fn comparar_remove_contract_offer(this: usize, pessoa: *const u32, clube: *const u32) {
    let l = read_usize(this + 0x34);
    if l == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(IDX_REMOVE_CONTRACT_OFFER)(this, pessoa, clube);
    }
    let n = (read_usize(G_STAFF_STATE_TABLE).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *const u8;
    let alvo = first_offer_for_staff_club(l, *pessoa as usize, *clube as usize);
    let esperado: Vec<usize> = list_items_capped(l).into_iter().filter(|&e| Some(e) != alvo).collect();
    let pendentes = if alvo.is_some() { (*n).wrapping_sub(1) } else { *n };
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(IDX_REMOVE_CONTRACT_OFFER)(this, pessoa, clube);
    let depois = list_items_capped(l);
    PLACAR.registrar(depois == esperado && *n == pendentes, || format!("remove_contract_offer({}, {}) itens {} (esperava {}), pendentes {} (esperava {})", *pessoa, *clube, depois.len(), esperado.len(), *n, pendentes));
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

/// with pending offers (+8 of the 79-byte record), unmarks the agreed contract (0x8c5430) and removes the person offers
/// (+0x34) until the byte is zero
/// Original: `CONTRACT_MANAGER::remove_pending_offers_for_staff` (0x4ccbf0, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
pub(crate) unsafe extern "thiscall" fn remove_pending_offers_for_staff(this: usize, pessoa: *const u32) {
    let l = read_usize(this + 0x34);
    if l == 0 || *pending_offers_byte(*pessoa as usize) == 0 {
        return;
    }
    let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
    desmarcar(read_usize(G_TRANSFER_MANAGER), pessoa);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if read_usize(e) as u32 == *pessoa {
            list_remove(l, e);
            let p = pending_offers_byte(*pessoa as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // a última pendente
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
pub(crate) unsafe fn offer_to_schedule(l: usize, oferta: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == read_usize(oferta + 0xc) && read_usize(e + 4) == read_usize(oferta + 8) && *((e + 0x62) as *const u8) == 0)
}

/// 1 when the offered player's ability (+0xb of the player record) / 500 exceeds the offering club's reputation
/// (+0x80), else 0.
///
/// [pt] Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
pub(crate) unsafe fn ability_above_club_reputation(oferta: usize) -> u32 {
    let pessoa = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(staff::SIZE));
    let hab = *((read_usize(pessoa + staff::PLAYER) + 0xb) as *const i16) as i32 / 500;
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(oferta + 8).wrapping_mul(club::SIZE));
    (hab > *((clube + club::REPUTATION) as *const i16) as i32) as u32
}

/// marks (+0x62) the first offer of the person and club still undecided and schedules the decision (0x4cd230); returns
/// ability / 500 > club reputation
/// Original: `CONTRACT_MANAGER::schedule_decision_for_transfer` (0x4cc930, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
pub(crate) unsafe extern "thiscall" fn schedule_decision_for_transfer(this: usize, oferta: usize) -> u32 {
    let l = read_usize(this + 0x34);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = contract_manager_list_next(l);
    while e != 0 {
        if read_usize(e) == read_usize(oferta + 0xc) && read_usize(e + 4) == read_usize(oferta + 8) && *((e + 0x62) as *const u8) == 0 {
            *((e + 0x62) as *mut u8) = 1; // a oferta vale
            let agendar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_OFFER_DECISION);
            agendar(this, read_usize(oferta + 0xc) as u32, read_usize(oferta + 0x20) as u32);
            break;
        }
        e = contract_manager_list_next(l);
    }
    ability_above_club_reputation(oferta)
}


/// Comparer for `remove_pending_offers_for_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o mesmo plano pelo byte de pendentes; a original tira (e desmarca); confere os itens e o byte.
pub(crate) unsafe extern "thiscall" fn comparar_remove_pending_offers_for_staff(this: usize, pessoa: *const u32) {
    let l = read_usize(this + 0x34);
    if l == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(IDX_REMOVE_PENDING_OFFERS_FOR_STAFF)(this, pessoa);
    }
    let p = pending_offers_byte(*pessoa as usize);
    let antes = list_items_capped(l);
    let mut resta = *p;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta != 0 && read_usize(e) as u32 == *pessoa {
            resta = resta.wrapping_sub(1);
        } else {
            esperado.push(e);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(IDX_REMOVE_PENDING_OFFERS_FOR_STAFF)(this, pessoa);
    let depois = list_items_capped(l);
    PLACAR.registrar(depois == esperado && *p == resta, || format!("remove_pending_offers_for_staff({}) itens {} -> {} (esperava {}), pendentes {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *p, resta));
}

/// Comparer for `schedule_decision_for_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a oferta que a cópia marcaria antes; a original marca e agenda (a 0x4cd230 pode acrescentar na lista
/// das decisões); confere a marca e o valor.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_decision_for_transfer(this: usize, oferta: usize) -> u32 {
    let alvo = offer_to_schedule(read_usize(this + 0x34), oferta);
    let n = ability_above_club_reputation(oferta);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCHEDULE_DECISION_FOR_TRANSFER)(this, oferta);
    let marcada = alvo.map_or(true, |e| *((e + 0x62) as *const u8) == 1);
    PLACAR.registrar(marcada && r == n, || format!("schedule_decision_for_transfer({:x}) alvo {:x?} marcada={} original={} rust={}", oferta, alvo, marcada, r, n));
    r
}


/// removes from the held offers list (+0x38) every item of the person (0x68d1f0)
/// Original: `CONTRACT_MANAGER::remove_held_offers` (0x4cf560, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cf560 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_held_offers(pessoa)`: da lista das
/// ofertas guardadas (+0x38; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item cujo +0 é o id `pessoa`
/// (0x68d1b0 percorre). Sem lista: nada.
pub(crate) unsafe extern "thiscall" fn remove_held_offers(this: usize, pessoa: u32) {
    let l = read_usize(this + 0x38);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut e = proximo(l);
    while e != 0 {
        if read_usize(e) as u32 == pessoa { // a oferta da pessoa
            tirar(l, e);
        }
        e = proximo(l);
    }
}

/// Whether suspension type t applies at the loan club, by the type's competition (+0x17) or rule (+0x1b) against the
/// club's nation.
///
/// [pt] A suspensão do tipo `t` (vetor de tipos [this], 4 bytes cada) vale no clube do empréstimo? Pela competição do tipo
/// (+0x17, −1 = nenhuma) igual à nação do clube (+0x53, id em +0); sem competição, pelo +0x1b (−1: vale) igual ao
/// [+0x71] da nação. Clube sem nação: não vale.
pub(crate) unsafe fn suspension_valid_at_club(this: usize, t: i8, clube: usize) -> bool {
    let tipo = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nacao = read_usize(clube + club::NATION);
    let comp = read_usize(tipo + 0x17) as i32;
    if comp != -1 {
        return nacao != 0 && read_usize(nacao) as i32 == comp;
    }
    let x = read_usize(tipo + 0x1b) as i32;
    if x == -1 {
        return true;
    }
    if nacao == 0 {
        return false;
    }
    let m = read_usize(nacao + nation::CONTINENT);
    m != 0 && read_usize(m) as i32 == x // a do +0x71 da nação
}

/// Loan club and the 5-byte suspension slots of the player's discipline record that do not apply there (skipping the
/// 4th like the original).
///
/// [pt] O clube do empréstimo (+4 da ficha de contrato, clubes de 0x245 bytes em [0xae23b4]) e o registro de suspensões
/// do jogador (29 bytes em [this+0x14]); as vagas (5 bytes: tipo e o resto) que não valem lá, menos a 4ª (índice 3,
/// que o original pula).
pub(crate) unsafe fn suspensions_to_store(this: usize, ficha: usize, jogador: usize) -> (usize, Vec<u8>) {
    let clube = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(ficha + 4).wrapping_mul(club::SIZE));
    let reg = read_usize(this + 0x14).wrapping_add(read_usize(jogador).wrapping_mul(29));
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
pub(crate) fn job_sought_by_slot(cargo: u32) -> Option<u32> {
    match cargo as u8 as i8 {
        6 | 8 | 9 | 10 => Some(cargo),
        13 => Some((cargo & !0xff) | 6),
        15 => Some((cargo & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// First person in G_STAFF_TABLE without club, with the given job, a player record and the club's nation.
///
/// [pt] A primeira pessoa (de [0xae23bc], 0x6e bytes, [0xae2364] delas) sem clube (+0x39), no cargo (+0x3d), com registro
/// (+0x69) e da nação (+0x1a) do clube (+0x53).
pub(crate) unsafe fn first_unemployed_staff(clube: usize, cargo: u8) -> Option<usize> {
    let n = read_usize(G_STAFF_COUNT) as i32;
    let base = read_usize(G_STAFF_TABLE);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        read_usize(p + 0x39) == 0 && *((p + 0x3d) as *const u8) == cargo && read_usize(p + 0x69) != 0 && read_usize(p + 0x1a) == read_usize(clube + club::NATION)
    })
}


/// Comparer for `remove_held_offers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a 0x68d1f0 libera o item tirado, que não pode mais ser lido); a original tira; confere
/// que ficaram os itens de antes menos os da pessoa, na mesma ordem.
pub(crate) unsafe extern "thiscall" fn comparar_remove_held_offers(this: usize, pessoa: u32) {
    let l = read_usize(this + 0x38);
    let antes: Vec<usize> = if l != 0 { list_items_capped(l) } else { Vec::new() };
    let esperado: Vec<usize> = antes.iter().copied().filter(|&e| read_usize(e) as u32 != pessoa).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_REMOVE_HELD_OFFERS)(this, pessoa);
    let depois: Vec<usize> = if l != 0 { list_items_capped(l) } else { Vec::new() };
    PLACAR.registrar(depois == esperado, || format!("remove_held_offers({:x}) {} itens, esperava {}, ficaram {}", pessoa, antes.len(), esperado.len(), depois.len()));
}




/// The person's contract as 0x4cbb60 finds it inline (null, out of range or no index → 0).
///
/// [pt] O contrato da pessoa como a 0x4cbb60 acha por dentro (sem chamar a 0x4cf5a0): pessoa não nula, id abaixo de
/// [0xae2364], índice da tabela [0xae2d0c] (79 bytes por pessoa) não negativo; registro de 80 bytes em [objeto].
pub(crate) unsafe fn squad_contract_of_staff(objeto: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = read_u32(pessoa) as i32;
    if id >= read_u32(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    (read_u32(objeto) as usize).wrapping_add((i as usize).wrapping_mul(80))
}

/// 0 none, 1 reserves, 2 first team (bit 0 of +0x5e at his own club with reserves), 3 at the linked B side, 4 at the
/// main club with a B side (through the partner club 0x540a50 and bit 6 of the contract +0x35)
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
pub(crate) unsafe extern "thiscall" fn get_squad_placement(this: usize, pessoa: usize, clube: usize) -> u8 {
    let tipo = *((pessoa + staff::JOB_FOR_CLUB) as *const i8);
    let tipo_de_jogador: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    if tipo == 12 || tipo_de_jogador(tipo) == 0 {
        return 0;
    }
    let reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let proprio = read_u32(pessoa + staff::CLUB_JOB) as usize;
    if proprio != 0 && proprio == clube && tipo_de_jogador(*((pessoa + staff::JOB_FOR_CLUB) as *const i8)) != 0 && reservas(clube) != 0 {
        return if *((pessoa + staff::PLAYING_SQUAD) as *const u8) & 1 != 0 { 2 } else { 1 }; // principal ou reservas
    }
    let mut vaga = pessoa as u32; // a vaga do argumento: a 0x540a50 pode não gravar
    let mut b = 0usize;
    if proprio != 0 {
        b = parceiro(proprio, &mut vaga, 1);
        if b != 0 {
            if vaga != 0 {
                if clube == read_u32(pessoa + staff::CLUB_JOB) as usize {
                    return 4;
                }
            } else if clube == b {
                return 3;
            }
        }
    }
    let c = squad_contract_of_staff(read_u32(G_CONTRACT_MANAGER) as usize, pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c; // a ficha do contrato
    if c == 0 {
        return 0;
    }
    let i = read_u32(c + 4) as usize;
    let outro = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if outro == 0 || outro == read_u32(pessoa + staff::CLUB_JOB) as usize || outro == b {
        return 0;
    }
    let c2 = squad_contract_of_staff(this, pessoa);
    let marca = (*((c2 + 0x35) as *const u8) << 1) as i8 >> 7; // o bit 6, estendido
    if outro == clube && reservas(clube) != 0 {
        return if marca == 0 { 2 } else { 1 };
    }
    let p = parceiro(outro, &mut vaga, 1);
    if p == 0 || (p != clube && outro != clube) {
        return 0;
    }
    if marca == 0 { 4 } else { 3 }
}

/// If the squad placement (0x4cbb60) is one of de, toggles it (0x4cbd20); else 0.
///
/// [pt] A 0x4cbb60 pelo endereço (passa pelo desvio) e a troca 0x4cbd20 (com 0 na saída do novo clube).
pub(crate) unsafe fn toggle_placement_if_in(this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let l = lugar(this, pessoa, clube);
    if l != de[0] && l != de[1] {
        return 0;
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TOGGLE_SQUAD_PLACEMENT);
    trocar(this, pessoa, clube, 0)
}

/// in the reserves or at the B side (placement 1 or 3), swaps through 0x4cbd20; else 0
/// Original: `CONTRACT_MANAGER::move_to_first_team` (0x4cc0b0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cc0b0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_first_team(pessoa, clube)`: nos
/// reservas (1) ou no time B (3), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
pub(crate) unsafe extern "thiscall" fn move_to_first_team(this: usize, pessoa: usize, clube: usize) -> u32 {
    toggle_placement_if_in(this, pessoa, clube, [1, 3]) // reservas ou time B
}

/// in the first team or at the club with a B side (placement 2 or 4), swaps through 0x4cbd20; else 0
/// Original: `CONTRACT_MANAGER::move_to_reserves` (0x4cc0f0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4cc0f0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_reserves(pessoa, clube)`: no
/// principal (2) ou no clube com time B (4), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
pub(crate) unsafe extern "thiscall" fn move_to_reserves(this: usize, pessoa: usize, clube: usize) -> u32 {
    toggle_placement_if_in(this, pessoa, clube, [2, 4]) // principal ou clube com time B
}

/// Comparer for `get_squad_placement` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela ficha [0xae1080] (a vaga da saída é o argumento na pilha de cada uma).
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_placement(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_PLACEMENT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, pessoa, clube), || get_squad_placement(this, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("get_squad_placement({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparer of move_to_first_team / move_to_reserves: side by side outside the toggle placements, original only inside.
///
/// [pt] Comparador das duas trocas: o lugar (0x4cbb60, pelo desvio) decide antes; fora dos lugares da troca, lado a lado
/// pela ficha [0xae1080] (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
pub(crate) unsafe fn compare_placement_toggle(i: usize, nome: &str, this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let ficha = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let l = lugar(this, pessoa, clube);
    *(G_CONTRACT_LOOKUP_RESULT as *mut u32) = ficha;
    if l == de[0] || l == de[1] {
        return o(this, pessoa, clube);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, pessoa, clube), || toggle_placement_if_in(this, pessoa, clube, de));
    PLACAR.registrar(igual && r == n, || format!("{}({:x}, {:x}) lugar {} original={} rust={} {:x?} x {:x?}", nome, pessoa, clube, l, r, n, mo, mr));
    r
}

/// Comparer for `move_to_first_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_to_first_team(this: usize, pessoa: usize, clube: usize) -> u32 {
    compare_placement_toggle(830, "move_to_first_team", this, pessoa, clube, [1, 3])
}

/// Comparer for `move_to_reserves` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_to_reserves(this: usize, pessoa: usize, clube: usize) -> u32 {
    compare_placement_toggle(831, "move_to_reserves", this, pessoa, clube, [2, 4])
}




/// copies the contract (0x50 bytes) field by field, the dates through 0x417c10 and the bit fields +0x35 and +0x4f in
/// steps
/// Original: `STAFF_CONTRACT::operator=` (0x4cc810, contract_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x4cc810 } (thiscall, ret 4, contract_manager.cpp): `STAFF_CONTRACT::operator=`: copia o contrato (0x50 bytes) de
/// `origem`: quatro duplas, três duplas (+0x10), cinco bytes (+0x1c), +0x21, as datas de início (+0x25) e do fim
/// (+0x2d) pela 0x417c10, o byte de campos de bits +0x35 em três passos (6 bits, o 7º, o 8º: no fim é o de `origem`),
/// +0x36, +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e e o +0x4f em dois passos (nibble
/// baixo e depois o resto). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn contract_manager_operator(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, read_u32(origem + o));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = read_u8(origem + o);
    for o in [0, 4, 8, 0xc, 0x10, 0x14, 0x18] {
        d(o);
    }
    for o in 0x1c..0x21 {
        b(o); // os cinco bytes
    }
    d(0x21);
    copy_record_date(this + 0x25, origem + 0x25);
    copy_record_date(this + 0x2d, origem + 0x2d);
    let m = (this + 0x35) as *mut u8;
    let s = read_u8(origem + 0x35);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ *m) & 0x3f) ^ *m; // os 6 bits baixos
    *m = cl;
    let al = ((((read_u8(origem + 0x35) << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl; // o 7º bit
    *m = al;
    let s = read_u8(origem + 0x35);
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
    let m = (this + 0x4f) as *mut u8;
    let c = *m;
    let al = ((read_u8(origem + 0x4f) ^ c) & 0xf) ^ c; // o nibble baixo
    *m = al;
    let s = read_u8(origem + 0x4f);
    *m = ((al ^ s) & 0xf) ^ s;
    this
}

/// the morale gain from the rating against the club reputation (up to 10), morale between −100 and 100 and under the
/// grievance cap; clears grievances 4 and 0x10 (0x8674e0) and checks the withdrawal (0x867240)
/// Original: `contract_manager_raise_match_morale` (0x4cb0a0, contract_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x4cb0a0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_raise_match_morale(pessoa, clube,
/// contrato)`: com os três e o registro de jogador (+0x61): x = (nota +0xb − reputação do clube +0x80) + 417 × +0x18
/// − 11680; y = 1 + x × (6,6e-5 se x < 0, senão 6,6667e-5), entre 0,01 e 2; o ganho = ftol((+0x3f + 5) / y), no máximo
/// 10. O teto: 100 (50 com a queixa 2), −50 por queixa 1, 8, 0x20, 0x40, 0x80, 0x100 enquanto o teto não passa de −50
/// para baixo. A moral (+0x44) soma o ganho entre −100 e 100 e fica no teto. Sem +0x49: com ganho > 5 e moral + ganho
/// > 0, ou felicidade (+0x3d) ≥ 75 − ganho, apaga as queixas 4 e 0x10 (0x8674e0). Por fim confere a retirada do pedido
/// (0x867240 com 0). Constantes do .rdata; o x87 em dupla precisão, como as outras do contrato.
pub(crate) unsafe extern "stdcall" fn contract_manager_raise_match_morale(pessoa: usize, clube: usize, contrato: usize) {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return;
    }
    let jogador = read_u32(pessoa + staff::PLAYER) as usize;
    if jogador == 0 {
        return;
    }
    let a = core::ptr::read_unaligned((jogador + 0xb) as *const i16) as i32 - core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32;
    let x = (a as f64 + read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8 as f64 * read_f64_at(0x968d90)) - read_f64_at(0x968d88); // 0x968d90 = 417, 0x968d88 = 11680
    let mut y = (if x < read_f64_at(0x9689a0) { x * read_f64_at(0x968d80) } else { x * read_f64_at(0x968d78) }) + read_f64_at(0x967890); // 0x9689a0 = 0, 0x968d80 = 6.6e-05, 0x968d78 = 6.666666667e-05, 0x967890 = 1
    if y < read_f64_at(0x967898) { // 0x967898 = 0.01
        y = read_f64_at(0x967898); // 0x967898 = 0.01
    } else if y > read_f64_at(0x968950) { // 0x968950 = 2
        y = read_f64_at(0x968950); // 0x968950 = 2
    }
    let felicidade = (read_u8(contrato + 0x3f) as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut ganho = match_pl_ftol(felicidade / y) as u8 as i8;
    if ganho > 10 {
        ganho = 10;
    }
    let queixas = read_u32(contrato + 0x45);
    let mut teto: i8 = if queixas & 2 != 0 { 50 } else { 100 };
    if queixas & 1 != 0 {
        teto = teto.wrapping_sub(50);
    }
    for q in [8u32, 0x20, 0x40, 0x80, 0x100] {
        if teto < -50 {
            break;
        }
        if queixas & q != 0 {
            teto = teto.wrapping_sub(50); // menos 50 por queixa
        }
    }
    let moral = (contrato + 0x44) as *mut i8;
    let cl = *moral;
    let s = cl as i32 + ganho as i32;
    *moral = if s > 100 { 100 } else if s < -100 { -100 } else { cl.wrapping_add(ganho) };
    if *moral > teto {
        *moral = teto;
    }
    if read_u8(contrato + contract::LEAVING_ON_BOSMAN) == 0 {
        let apaga = if ganho > 5 && *moral as i32 + ganho as i32 > 0 {
            true
        } else {
            read_u8(contrato + 0x3d) as i8 as i32 >= 75 - ganho as i32
        };
        if apaga {
            let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
            if queixas & 4 != 0 {
                apagar(contrato, 4);
            }
            if read_u8(contrato + 0x45) & 0x10 != 0 {
                apagar(contrato, 0x10);
            }
        }
    }
    let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CHECK_TRANSFER_REQUEST_WITHDRAWAL);
    retirada(contrato, 0);
}

/// removes the person talks (and of the club and manager, when given) moving the last one into the gap; sorts again
/// Original: `CONTRACT_MANAGER::remove_talks_records` (0x4ca1c0, contract_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x4ca1c0 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::remove_talks_records(pessoa, tecnico,
/// clube)`: no vetor de conversas ([this+0x28], [this+0x2c] registros de 21 bytes), cada um da pessoa (+0 = id) e,
/// quando dados, do clube (+4) e do técnico (+8) recebe por cima o último (5 duplas e um byte), a contagem desce e o
/// mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x4dc970).
pub(crate) unsafe extern "thiscall" fn remove_talks_records(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let n = (this + 0x2c) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + 0x28) as usize;
            let r = base.wrapping_add(off);
            let casa = read_u32(r) == read_u32(pessoa)
                && (clube == 0 || read_u32(r + 4) == read_u32(clube)) // o clube
                && (tecnico == 0 || read_u32(r + 8) == read_u32(tecnico));
            if casa {
                let ultimo = base.wrapping_add((*n as usize).wrapping_mul(21)).wrapping_sub(21);
                core::ptr::copy(ultimo as *const u8, r as *mut u8, 21);
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
        qsort(read_u32(this + 0x28) as usize, *n, 0x15, 0x4dc970);
    }
}

/// Comparer for `contract_manager_operator` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contract_manager_operator(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONTRACT_MANAGER_OPERATOR);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x50)], || o(this, origem), || contract_manager_operator(this, origem));
    PLACAR.registrar(igual && r == n, || format!("contract_manager_operator {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: a 0x8674e0 e a 0x867240 mexem nas queixas, nos vínculos e na transferência (não podem rodar duas
/// vezes): só a original; a prova é a carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (153.222 chamadas); comparar pedia separar a moral das chamadas do fim.
/// Comparer for `contract_manager_raise_match_morale` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_raise_match_morale(pessoa: usize, clube: usize, contrato: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, usize)>(IDX_CONTRACT_MANAGER_RAISE_MATCH_MORALE)(pessoa, clube, contrato)
}

/// Comparer for `remove_talks_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_talks_records(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_REMOVE_TALKS_RECORDS);
    let n = (*((this + 0x2c) as *const i32)).max(0) as usize;
    let reg = [(this + 0x28, 8), (read_u32(this + 0x28) as usize, n * 21)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, tecnico, clube), || remove_talks_records(this, pessoa, tecnico, clube));
    PLACAR.registrar(igual, || format!("remove_talks_records({:x}, {:x}, {:x}) {:x?} x {:x?}", pessoa, tecnico, clube, mo, mr));
}

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
pub(crate) unsafe extern "thiscall" fn remove_loan_contract(gerente: usize, id: *const i32) {
    let e = loan_state_entry(id);
    let vaga = core::ptr::read_unaligned((e + 4) as *const i32);
    if vaga != -1 {
        *((*(gerente as *const usize) as isize + vaga as isize * 80) as *mut i32) = -1;
        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, -1);
    }
}

/// Shortlist entry of club *clube (9 bytes per club in [manager+0xdc]: +0 slots, +4 mark).
///
/// [pt] A lista de observados do clube `*clube` (9 bytes por clube em [gerente+0xdc]: +0 as vagas, +4 a marca).
pub(crate) unsafe fn club_shortlist(gerente: usize, clube: *const i32) -> usize {
    (*((gerente + 0xdc) as *const usize) as isize + *clube as isize * 9) as usize
}



/// Comparer for `remove_loan_contract` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_loan_contract(gerente: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_REMOVE_LOAN_CONTRACT);
    let e = loan_state_entry(id);
    let mut reg = vec![(e + 4, 4)];
    let vaga = core::ptr::read_unaligned((e + 4) as *const i32);
    if vaga != -1 {
        reg.push(((*(gerente as *const usize) as isize + vaga as isize * 80) as usize, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || remove_loan_contract(gerente, id));
    PLACAR.registrar(igual, || format!("remove_loan_contract({}) {:x?} x {:x?}", *id, mo, mr));
}



/// Walks the contract manager's offer list (+0x34) from the start until achou accepts an offer; 0 if none.
///
/// [pt] Percorre a lista do gerente de contratos (+0x34, cursor em +0xa voltando ao começo +2, 0x68d1b0)
/// até a oferta que `achou` aceita; 0 sem lista ou sem nenhuma.
pub(crate) unsafe fn find_manager_offer(gerente: usize, achou: impl Fn(usize) -> bool) -> usize {
    let lista = *((gerente + 0x34) as *const usize);
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    loop {
        let o = proximo(*((gerente + 0x34) as *const usize));
        if o == 0 || achou(o) {
            return o;
        }
    }
}

/// the person's contract offer for the club, in the contract manager's list
/// Original: `contract_offer` (0x4cc9e0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cc9e0 } (thiscall, ret 8, contract_manager.cpp): a oferta de contrato da pessoa `pessoa`
/// (+0) para o clube `clube` (+4), ou 0.
pub(crate) unsafe extern "thiscall" fn contract_offer(gerente: usize, pessoa: i32, clube: i32) -> usize {
    find_manager_offer(gerente, |o| *(o as *const i32) == pessoa && *((o + 4) as *const i32) == clube)
}

/// the person's agreed contract offer (state 2)
/// Original: `agreed_contract_offer` (0x4cca30, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cca30 } (thiscall, ret 4, contract_manager.cpp): a oferta acertada (+0x50 = 2) da pessoa, ou 0.
pub(crate) unsafe extern "thiscall" fn agreed_contract_offer(gerente: usize, pessoa: i32) -> usize {
    find_manager_offer(gerente, |o| *(o as *const i32) == pessoa && *((o + 0x50) as *const u8) == 2)
}

/// whether the transfer offer is closed (by its state and, in state 0xf, by its phase)
/// Original: `transfer_offer_closed` (0x8d0d90, transfer_offer.cpp, stdcall).
///
/// [pt] { 0x8d0d90 } (stdcall, transfer_offer.cpp): a oferta de transferência está encerrada? Estados
/// (+0x2c) 2, 3, 4, 0x10 e 0x14 sim; 0xf só com +0x2e a partir de 3; os demais não.
pub(crate) unsafe extern "system" fn transfer_offer_closed(o: usize) -> u32 {
    match *((o + 0x2c) as *const u8) {
        2 | 3 | 4 | 0x10 | 0x14 => 1,
        0xf => (*((o + 0x2e) as *const u8) >= 3) as u32,
        _ => 0,
    }
}

/// Memory regions the offer searches touch: the cursor of the manager's offer list.
///
/// [pt] As regiões que as buscas mexem: o cursor da lista do gerente.
pub(crate) unsafe fn offer_list_regions(gerente: usize) -> Vec<(usize, usize)> {
    let lista = *((gerente + 0x34) as *const usize);
    if lista == 0 { Vec::new() } else { vec![(lista, 0xe)] }
}

/// Comparer for `contract_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contract_offer(gerente: usize, pessoa: i32, clube: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> usize>(IDX_CONTRACT_OFFER);
    let reg = offer_list_regions(gerente);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, pessoa, clube), || contract_offer(gerente, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("contract_offer({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparer for `agreed_contract_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_agreed_contract_offer(gerente: usize, pessoa: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_AGREED_CONTRACT_OFFER);
    let reg = offer_list_regions(gerente);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, pessoa), || agreed_contract_offer(gerente, pessoa));
    PLACAR.registrar(igual && r == n, || format!("agreed_contract_offer({}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `transfer_offer_closed` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "system" fn comparar_transfer_offer_closed(o: usize) -> u32 {
    let r = original_fn::<unsafe extern "system" fn(usize) -> u32>(IDX_TRANSFER_OFFER_CLOSED)(o);
    let n = transfer_offer_closed(o);
    PLACAR.registrar(r == n, || format!("transfer_offer_closed(estado {:#x}, {:#x}) original={} rust={}", *((o + 0x2c) as *const u8), *((o + 0x2e) as *const u8), r, n));
    r
}

/// dword +0xc of the person's contract record
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
    let ficha = (*(t as *const usize)).wrapping_add(k as usize * 0x50);
    if ficha == 0 {
        return 0;
    }
    core::ptr::read_unaligned((ficha + 0xc) as *const u32)
}

/// finds or creates the person's type 3 decision entry and sets the day and time slice of the answer: by the offer
/// deadline, or without an offer by today and the contract end; at most 14 days ahead
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
pub(crate) unsafe extern "thiscall" fn schedule_offer_decision(this: usize, id: i32, vaga: i32) {
    let mut limit = [0u8; 8];
    injury_date_add_days(G_CURRENT_DATE, 0xe, &mut limit);
    let mut e = find_type3_decision(this, id, &limit, true);
    let pessoa = read_u32_as_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if e == 0 {
        crate::comparacao::ramo("cpr_10 nova decisao");
        let mut d = [0u8; 8];
        injury_date_add_days(G_CURRENT_DATE, (sortear(5) + 1) as u32, &mut d); // cpr10: a decisão em 1 a 5 dias
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, *const u8, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_DECISION_DATE);
        acrescentar(this, pessoa, d.as_ptr(), 2, 3);
        let n = read_u32_as_usize(this + 0x14);
        e = read_u32_as_usize(this + 0xc).wrapping_add(n.wrapping_sub(1).wrapping_mul(14));
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let data = e + 4;
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
        let of = vaga_da(read_u32_as_usize(G_TRANSFER_MANAGER), vaga);
        // ponytail: a original passa as vagas dos dois argumentos como saída; começam com os bytes baixos deles
        let (mut a, mut b) = (id as u8, vaga as u8);
        let prazo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_RESPONSE_DELAY);
        prazo(read_u32_as_usize(G_TRANSFER_MANAGER), read_u32_as_usize(of), &mut a, &mut b);
        let alvo = a.wrapping_mul(3).wrapping_add(b) as i8;
        let falta = dias(data, read_u32_as_usize(G_CURRENT_DATE) as u32, read_u32_as_usize(G_CURRENT_DATE_LEAP) as u32);
        let mut f = (*((e + 0xc) as *const u8)).wrapping_sub(*(G_CURRENT_DAY_SLICE as *const u8)) as i8;
        let mut dl = falta as u8;
        if f < 0 {
            f = f.wrapping_add(3);
            dl = dl.wrapping_sub(1);
        }
        let atual = dl.wrapping_mul(3).wrapping_add(f as u8) as i8;
        let x = if atual < alvo { atual.max(1) } else { alvo.max(1) } as i32;
        set_today_plus_days(data, (x / 3) as u32);
        advance_time_slice(e, ((x % 3) as u8).wrapping_add(*(G_CURRENT_DAY_SLICE as *const u8)) as i8);
        if *((read_u32_as_usize(of) + 0x2f) as *const u8) != 0 && dias(data, read_u32_as_usize(G_CURRENT_DATE) as u32, read_u32_as_usize(G_CURRENT_DATE_LEAP) as u32) < 4 {
            set_today_plus_days(data, (sortear(2) + 4) as u32);
        }
    } else {
        let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(0xae2c92));
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
                if ma > ca || (ma == ca && read_i16(m) > read_i16(c + 0x2d)) {
                    set_today_plus_days(data, (sortear(4) + 4) as u32);
                }
            }
        }
    }
    let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(0xae2c92));
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut t = [0u8; 8];
    let m = menos(data, t.as_mut_ptr(), 0xe);
    let ma = read_u16(m + 2);
    if ta < ma || (ta == ma && td < read_i16(m)) {
        set_today_plus_days(data, 0xe);
    }
    core::ptr::write_unaligned((this + 0x1c) as *mut u32, read_u32_as_usize(this + 0x1c).wrapping_add(1) as u32);
}

/// Comparer for `schedule_offer_decision` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem a entrada (a 0x4cd6d0 acrescenta e pode realocar) só a original; com ela, lado a lado pelo sorteio,
/// as datas estáticas, o vetor das decisões e +0x1c (a cópia sem contar).
pub(crate) unsafe extern "thiscall" fn comparar_schedule_offer_decision(this: usize, id: i32, vaga: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(IDX_SCHEDULE_OFFER_DECISION);
    let mut limit = [0u8; 8];
    injury_date_add_days(G_CURRENT_DATE, 0xe, &mut limit);
    if find_type3_decision(this, id, &limit, false) == 0 {
        return o(this, id, vaga);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&INJURY_DATE_STATICS);
    reg.push((this + 0x1c, 4));
    reg.push((read_u32_as_usize(this + 0xc), read_u32_as_usize(this + 0x14).wrapping_mul(14)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id, vaga),
        || crate::aleatorio::sem_contar(|| schedule_offer_decision(this, id, vaga)));
    PLACAR.registrar(igual, || format!("schedule_offer_decision(id {}, vaga {}) {:x?} x {:x?}", id, vaga, mo, mr));
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

/// the player's category 1..8 in the contract (status, happiness, age, ambition, reputation)
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
    let ficha = core::ptr::read_unaligned((p + 0x61) as *const usize);
    if ficha == 0 {
        return 7;
    }
    if c == 0 {
        return 4;
    }
    if *((c + 0x35) as *const u8) & 0x3f == 5 {
        return 8;
    }
    let k = *((c + 0x4f) as *const u8) >> 4;
    let v = *((c + 0x3d) as *const i8);
    let idade = *((p + 0x18) as *const i8);
    let ambicao = *((p + 0x57) as *const i8);
    let rep = |o: usize| core::ptr::read_unaligned((ficha + o) as *const i16);
    let baixo = |limit: i8| if v <= limit { 7 } else { 4 };
    match k {
        0 => {
            if idade > 0x15 {
                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { baixo(-30) }
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
            let nota = ambicao as i32 * 25 - *((p + 0x59) as *const i8) as i32 * 10 + rep(0xd) as i32;
            if v > 0x4b && nota > 0x186a && idade > 0x17 && idade < 0x22 {
                1
            } else if v > 0x19 {
                2
            } else {
                (v <= 0) as u8 + 3
            }
        }
        3 => if v > 0x37 && idade > 0x17 && idade < 0x22 { 2 } else { (v <= 0) as u8 + 3 },
        4 => {
            if v > 0x41 && idade > 0x17 {
                (idade < 0x22) as u8 + 2
            } else if v > -30 {
                4
            } else if idade >= 0x18 {
                7
            } else {
                5
            }
        }
        5 => {
            if idade > 0x18 {
                if idade > 0x22 { 4 } else if v > 0x4b { 2 } else if v > 0 { 3 } else { baixo(-30) }
            } else if ambicao > 5 && rep(0xb) > 0x4e2 {
                5
            } else {
                6
            }
        }
        6 => {
            if idade > 0x17 {
                if v > 0x4b { 1 } else if v > 0x1e { 2 } else if v > 0 { 3 } else { baixo(-50) }
            } else if rep(0xb) > 0x4e2 && ambicao > 0xf {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

/// Comparer for `contract_category` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_contract_category(p: usize, c: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_CONTRACT_CATEGORY)(p, c);
    let n = contract_category(p, c);
    PLACAR.registrar(r == n, || format!("contract_category({:#x}, {:#x}) original={} rust={}", p, c, r, n));
    r
}

/// news 0xbea of a player on the transfer list: the price (the value, or the asking price times a drawn factor,
/// rounded) and from 500 thousand up the up to 3 strongest interested clubs; posted to the interested human clubs
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
pub(crate) unsafe extern "thiscall" fn post_transfer_listed_news(this: usize, jogador: usize, clube: usize) {
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let lista_fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mascara: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let verba: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let destinatarios: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_PLAYER_NEWS_RECIPIENTS);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    if jogador == 0 || read_u32_as_usize(jogador + 0x61) == 0 || clube == 0 || read_u32_as_usize(clube + club::MANAGER) == 0 {
        return;
    }
    let c = contrato_com(this, jogador, clube);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    if c != 0 {
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        construir(n, 0xbea, 0);
        gravar(n, 0, read_u32_as_usize(clube) as u32);
        gravar(n, 1, read_u32_as_usize(jogador) as u32);
        gravar(n, 2, read_u32_as_usize(read_u32_as_usize(jogador + 4) + 0x33) as u32);
        gravar(n, 3, read_u32_as_usize(read_u32_as_usize(jogador + 8) + 0x33) as u32);
        gravar(n, 4, read_u32_as_usize(read_u32_as_usize(jogador + 0xc) + 0x33) as u32);
        let com = read_u32_as_usize(clube + club::MANAGER);
        gravar(n, 5, read_u32_as_usize(com) as u32);
        gravar(n, 6, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER) + 4) + 0x33) as u32);
        gravar(n, 7, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER) + 8) + 0x33) as u32);
        gravar(n, 8, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(clube + club::MANAGER) + 0xc) + 0x33) as u32);
        gravar(n, 9, read_u32_as_usize(c + 0x45) as u32);
        gravar(n, 10, (read_u8(c + 0x4f) >> 4) as u32);
        gravar(n, 11, mascara(read_u32_as_usize(jogador + 0x61)) as i32 as u32);
        crate::comparacao::ramo("cqf_2 notícia do listado");
        let s = sorteio(2); // cqf2: o sorteio do dado 12
        gravar(n, 12, s as u32);
        let fora_da_lista = (listado(jogador, clube) == 0) as u32;
        let preco = if humano(clube, 0, 1, 0, 0) != 0 && (fora_da_lista == 0 || read_u8(c + 0x4f) & 0xf0 == 0x10) {
            read_u32_as_usize(jogador + 0x52) as i32
        } else {
            let p = pedido(this, clube, 0, jogador, 0, 1, 1);
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
        gravar(n, 0xd, preco as u32);
        if preco >= 500_000 {
            crate::comparacao::ramo("cqf_2 interessados no listado");
            let mut clubes = [0usize; 30];
            let mut q: i8 = 0;
            interessados(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), jogador, l, 1, 1, 0);
            write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
            let mut no = proximo(l);
            while no != 0 && q < 0x1e {
                let id = read_u32_as_usize(no + 4);
                let pode = if humano(transfer_manager_club_by_id(id), 0, 1, 0, 0) == 0 {
                    true
                } else {
                    let f = financas(read_u32_as_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_u32_as_usize(no + 4)));
                    f != 0 && preco as f64 * read_f64_at(0x9689c0) <= verba(f, 0, 0, 0, 1, 0) as f64 // 0x9689c0 = 1.1
                };
                if pode && read_u32_as_usize(no + 4) as i32 > -1 {
                    clubes[q as usize] = transfer_manager_club_by_id(read_u32_as_usize(no + 4));
                    q += 1;
                }
                no = proximo(l);
            }
            let total = q as i32;
            let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
            let mut i = 0i32;
            while i < total - 1 {
                let mut j = i + 1;
                while j < total {
                    if rep(clubes[i as usize]) < rep(clubes[j as usize]) {
                        clubes.swap(i as usize, j as usize);
                    }
                    j += 1;
                }
                i += 1;
            }
            let m = if q >= 3 { 3 } else { total };
            gravar(n, 0xe, m as u32);
            let mut lugar: i8 = 0xf;
            let mut k: i8 = 0;
            let fim = loop {
                if k as i32 >= m {
                    break lugar;
                }
                if k == 1 && clubes[1] == clubes[0] {
                    gravar(n, lugar - 2, 1);
                    break lugar;
                }
                if k == 2 && (clubes[2] == clubes[0] || clubes[2] == clubes[1]) {
                    gravar(n, lugar - 3, 2);
                    break lugar;
                }
                gravar(n, lugar, read_u32_as_usize(clubes[k as usize]) as u32);
                lugar += 1;
                k += 1;
            };
            // ponytail: o lugar vai num dword com os bytes altos do preço na original; a 0x785790 lê o byte
            gravar(n, fim, fora_da_lista);
            let mut dest = clubes;
            let qd = if read_u32_as_usize(G_TRANSFER_MANAGER) != 0 && read_u32_as_usize(jogador + 0x61) != 0 {
                destinatarios(read_u32_as_usize(G_TRANSFER_MANAGER), jogador, 0x21a, dest.as_mut_ptr(), 0x1e) as i8
            } else {
                0
            };
            for &d in dest.iter().take(if qd > 0 { qd as usize } else { 0 }) {
                entregar(read_u32_as_usize(G_NEWS), n, d);
            }
        }
    }
    lista_fim(l);
}

/// Comparer for `post_transfer_listed_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4d71b0: só a original (sorteia, entrega a notícia, que aloca).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_listed_news(this: usize, jogador: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_TRANSFER_LISTED_NEWS)(this, jogador, clube) // ponytail: só a original
}

/// screens only: the person loan contract ends tomorrow (type 7) and, with a human manager at the relevant club, the
/// early return
/// Original: `CONTRACT_MANAGER::schedule_loan_end` (0x4ccca0, contract_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x4ccca0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::schedule_loan_end(pessoa, clube)`, só pela
/// tela de transferências (0x8fbcc0). Sem pessoa, id fora do total ou sem o contrato de empréstimo (+4 do registro de 79
/// bytes, no vetor de this): a caixa (linha 0xe4a). Senão o fim (+0x2d) vira amanhã (0x54c5b0 com 1; 0x417c10) e o tipo
/// (+0x35) vira 7 (os bits 3..5 zerados). O contrato atual (+0 do registro, no vetor de [0xae1050]) em [0xae1080]: do
/// `clube`, o clube da pessoa (+0x39) com técnico humano (0x7875e0, stdcall: o ecx = [0xb7431c] não é lido); de outro,
/// o clube do contrato atual (lido de novo duas vezes) com técnico humano: a volta (0x4db0b0: push clube; push pessoa;
/// mov ecx, this).
pub(crate) unsafe extern "thiscall" fn schedule_loan_end(this: usize, pessoa: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let voltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_LOAN_RECALL_NEWS);
    let c = if pessoa == 0 { 0 } else { staff_contract_by_field(pessoa, 4, this) };
    if c == 0 {
        show_error_box(0x9a8190, 0xe4a);
        return;
    }
    let mut amanha = [0u32; 2];
    let p = mais(G_CURRENT_DATE, &mut amanha, 1);
    let mut tmp = [0u8; 8];
    copia(c + 0x2d, tmp.as_mut_ptr() as usize, p);
    write_u8(c + 0x35, (read_u8(c + 0x35) & 0xc7) | 7);
    let a = staff_contract_by_field(pessoa, 0, read_u32_as_usize(G_CONTRACT_MANAGER));
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, a);
    let atual = if a == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(a + 4)) };
    if clube == atual {
        let x = read_u32_as_usize(pessoa + staff::CLUB_JOB);
        if x == 0 || human_manager_record_checked(x) == 0 {
            return;
        }
    } else {
        let a = staff_contract_by_field(pessoa, 0, read_u32_as_usize(G_CONTRACT_MANAGER));
        write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, a);
        if a == 0 || transfer_manager_club_by_id(read_u32_as_usize(a + 4)) == 0 {
            return;
        }
        let a = staff_contract_by_field(pessoa, 0, read_u32_as_usize(G_CONTRACT_MANAGER));
        write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, a);
        let x = if a == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(a + 4)) };
        if human_manager_record_checked(x) == 0 {
            return;
        }
    }
    voltar(this, pessoa, clube);
}

/// Comparer for `schedule_loan_end` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4ccca0: só a original (a volta do empréstimo entrega notícias e mexe no elenco).
pub(crate) unsafe extern "thiscall" fn comparar_schedule_loan_end(this: usize, pessoa: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SCHEDULE_LOAN_END)(this, pessoa, clube) // ponytail: só a original
}

/// the 0xbb9 news of a contract, trial or loan ending, to the club, the interested clubs or, for famous players, the
/// nation's managers or everyone
/// Original: `post_contract_end_news` (0x4da590, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4da590 } (thiscall, ret 0xc, contract_manager.cpp, quadro SEH da LLIST e da notícia locais): `CONTRACT_MANAGER::
/// post_contract_end_news(pessoa, tipo, amplo)`, pelo fim de contrato, de teste ou de empréstimo (0x4c7ac0). Sem
/// contrato ou sem o clube dele: nada. A notícia 0xbb9: o clube do contrato, a pessoa (o id e as três chaves), o tipo, o
/// clube do contrato no tipo 6 (senão −1) e `amplo`. Ampla, com ficha: reputação mundial (+0xd) acima de 7000, a todos
/// os técnicos (0x786780); nacional (+0xb) acima de 7000, aos técnicos da nação da pessoa, da do clube e da do clube
/// atual (+0x39) (0x7863f0) e aos clubes interessados (0x842180) de outras nações. Senão ao clube, ao clube atual no
/// tipo 6 e aos interessados (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_contract_end_news(this: usize, p: usize, tipo: u32, amplo: u32) {
    let c = contract_inline_lookup(p);
    if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        return;
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let do_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    crate::comparacao::ramo("cpx_4 noticia do fim do contrato");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbb9, 0);
    let c = contract_inline_lookup(p);
    // a original lê o id no endereço 0 sem contrato: não acontece, acabou de achá-lo
    let clube = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    gravar(nt, 0, read_usize(clube) as u32);
    gravar(nt, 2, read_usize(p) as u32);
    gravar(nt, 3, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(nt, 4, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(nt, 5, read_usize(read_usize(p + 0xc) + 0x33) as u32);
    let tipo = tipo as u8;
    gravar(nt, 6, tipo as i8 as u32);
    let c = contract_inline_lookup(p);
    let clube = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + 4)) };
    if tipo == 6 {
        // o clube do contrato pela tabela do próprio gerente (+0), sem guardar em [0xae1080]; sem contrato, a original
        // lê o endereço 4 (não acontece: acabou de achá-lo)
        let k = if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            -1
        } else {
            read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f))) as i32
        };
        let v = if k < 0 { read_usize(4) } else { read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(0x50)) + 4) };
        gravar(nt, 7, v as u32);
    } else {
        gravar(nt, 7, u32::MAX);
    }
    gravar(nt, 8, amplo);
    let ficha = read_usize(p + 0x61);
    let atual = read_usize(p + 0x39);
    let nacao = read_usize(p + 0x1a);
    let jornal = read_usize(G_NEWS);
    let mut padrao = true;
    if amplo != 0 && ficha != 0 {
        if read_i16(ficha + 0xd) > 0x1b58 {
            todos(jornal, nt);
            padrao = false;
        } else if read_i16(ficha + 0xb) > 0x1b58 {
            padrao = false;
            do_pais(jornal, nt, nacao);
            if nacao != read_usize(clube + club::NATION) {
                do_pais(jornal, nt, read_usize(clube + club::NATION));
            }
            if atual != 0 && atual != clube {
                let n = read_usize(atual + 0x53);
                if n != read_usize(clube + club::NATION) && n != nacao {
                    do_pais(jornal, nt, n);
                }
            }
            for id in interested_club_ids(p, l) {
                if id == read_usize(clube) || (tipo == 6 && id == read_usize(read_usize(p + 0x39))) {
                    continue;
                }
                let outro = friendly_club_by_id(id);
                let n = read_usize(outro + 0x53);
                if n != read_usize(p + 0x1a) && n != read_usize(read_usize(p + 0x39) + 0x53) && n != read_usize(clube + club::NATION) {
                    entregar(read_usize(G_NEWS), nt, outro);
                }
            }
        }
    }
    if padrao {
        entregar(jornal, nt, clube);
        if tipo == 6 {
            entregar(read_usize(G_NEWS), nt, read_usize(p + 0x39));
        }
        for id in interested_club_ids(p, l) {
            if id == read_usize(clube) || (tipo == 6 && id == read_usize(read_usize(p + 0x39))) {
                continue;
            }
            entregar(read_usize(G_NEWS), nt, friendly_club_by_id(id));
        }
    }
    soltar(l);
}

/// Comparer for `post_contract_end_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_end_news(this: usize, p: usize, tipo: u32, amplo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_POST_CONTRACT_END_NEWS)(this, p, tipo, amplo)
}

/// Comparer for `contract_with_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contract_with_club(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CONTRACT_WITH_CLUB);
    if is_recording_post_match() {
        return call_recording(0x4cf5f0, this, [pessoa as u32, clube as u32, 0, 0, 0], || o(this, pessoa, clube) as u32) as usize;
    }
    let r = o(this, pessoa, clube);
    let n = contract_with_club(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("contract_with_club({:#x}, {:?}, {:?}) original={:#x} rust={:#x}", this, pessoa, clube, r, n));
    r
}

/// Comparer for `contract_level` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_contract_level(pessoa: usize, clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_CONTRACT_LEVEL)(pessoa, clube);
    let v = contract_level(pessoa, clube);
    PLACAR.registrar(r == v, || format!("contract_level original={:#x} rust={:#x}", r, v));
    r
}
