// ---------- septuagesima quarta leva: contrato da pessoa com o clube ----------

/// { 0x4cf5f0 } (ret 8): o contrato (registros de 80 bytes em [this]) da pessoa com o clube. A
/// pessoa tem ate dois indices (tabela 0xae2d0c, 79 bytes por pessoa, +0 e +4); vale o registro
/// cujo +4 e o id do clube. Sem nenhum, tenta o clube parceiro (0x540a50) quando ele e o time A.
unsafe extern "thiscall" fn contrato_com_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    if pessoa == 0 || clube == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let e = (*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
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

/// { 0x53ce90 }: (byte +0x4f do contrato da pessoa com o clube (0x4cf5f0 sobre [0xae1050]) & 0xf)
/// - 10, para clube que nao e selecao (id abaixo de clubes - 2 x nacoes) e pessoa valida. Senao o
/// byte baixo zerado (o resto do registro fica como estava: o clube, ou 0).
unsafe extern "C" fn nivel_do_contrato(pessoa: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let limite = (*(0xae235c as *const i32)).wrapping_sub(2 * *(0xae2350 as *const i32));
    if *(clube as *const i32) >= limite || *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return clube as u32 & !0xff;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let r = f(*(0xae1050 as *const usize), pessoa, clube);
    if r == 0 {
        return 0;
    }
    ((*((r + 0x4f) as *const u8) & 0xf) as u32).wrapping_sub(10)
}

// ---------- 271a leva: o contrato assinado e posto no vetor dos contratos (contract_manager.cpp) ----------



unsafe fn cqg1_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}





unsafe fn cqg1_clube(id: usize) -> usize {
    read_u32_as_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O registro de 79 bytes da pessoa `id` em [0xae2d0c] (+0 o índice do contrato no vetor).
unsafe fn cqg1_registro(id: usize) -> usize {
    read_u32_as_usize(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f))
}

/// A cópia do contrato de 0x50 bytes de `o` para `d`, campo a campo como a original (as datas +0x25 e +0x2d pela
/// 0x417c10; os campos de bits de +0x35 e o nibble de +0x4f, mesclados em passos, acabam iguais à origem).
unsafe fn cqg1_copiar(d: usize, o: usize) {
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
    write_u16(d + 0x3b, cqg1_u(o + 0x3b));
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

/// O tipo do contrato (+0x3d) pelo cargo (nibble alto de +0x4f): 1 → 75, 2 → 50, 3 → 40, 4, 5, 6 e 8 → 0; os outros
/// não mexem (a tabela de saltos da original).
unsafe fn cqg1_tipo_pelo_cargo(c: usize) {
    match read_u8(c + 0x4f) >> 4 {
        1 => write_u8(c + 0x3d, 0x4b),
        2 => write_u8(c + 0x3d, 0x32),
        3 => write_u8(c + 0x3d, 0x28),
        4 | 5 | 6 | 8 => write_u8(c + 0x3d, 0),
        _ => {}
    }
}

/// { 0x4cf720 } (thiscall, ret 8, contract_manager.cpp, sem SEH): `CONTRACT_MANAGER::sign_contract(contrato, cargo)`,
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
unsafe extern "thiscall" fn assinar_contrato(this: usize, contrato: usize, cargo: u32) -> u32 {
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
    let clube = cqg1_clube(read_u32_as_usize(c + 4));
    let pessoa = read_u32_as_usize(0xae23bc).wrapping_add(read_u32_as_usize(c).wrapping_mul(0x6e));
    let emprestimo = || read_u8(c + 0x35) & 0x3f == 5;
    if read_u32_as_usize(pessoa + 0x39) == 0 && !emprestimo() && incluir(clube, pessoa, cargo, 0xae2c90, 1, 0) == 0 {
        return 0;
    }
    if cargo as u8 != read_u8(pessoa + 0x3d) && !emprestimo() && tem_vaga(read_u32_as_usize(pessoa + 0x39), cargo, pessoa) == 0 {
        noticia_sem_vaga(this, pessoa, read_u32_as_usize(pessoa + 0x39));
        return 0;
    }
    if read_u32_as_usize(pessoa + 0x39) != 0 && humano(read_u32_as_usize(pessoa + 0x39), 1, 1, 0, 0) != 0 {
        olheiros(read_u32_as_usize(0xb74448), read_u32_as_usize(read_u32_as_usize(pessoa + 0x39) + 0xcf));
        f8b2880(read_u32_as_usize(0xde1f44), read_u32_as_usize(pessoa + 0x39), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if emprestimo() {
        crate::comparacao::ramo("cqg_1 empréstimo");
        if (elenco(clube, 0, 0) as i32) + 1 > 0x32 {
            noticia_teste(this, pessoa, clube);
            return 0;
        }
        f90d860(read_u32_as_usize(0xde2f70), cqg1_clube(read_u32_as_usize(c + 4)), 0);
        incluir(cqg1_clube(read_u32_as_usize(c + 4)), pessoa, 0xb, 0xae2c90, 1, 0);
        let n = dias_ate(c + 0x2d, read_u32_as_usize(c + 0x25) as u32, read_u32_as_usize(c + 0x29) as u32);
        let fim = mais(0xae2c90, tmp2.as_mut_ptr(), n); // cqg1: o fim pela duração
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), fim);
        write_u8(c + 0x4f, (read_u8(c + 0x4f) & 0xf) | 0x80);
        copiar_data(c + 0x25, tmp.as_mut_ptr(), 0xae2c90);
        marcar_elenco(pessoa, 1);
    }
    // o fim no mínimo 4 semanas depois do início
    let (ini_dia, ini_ano) = (read_i16(c + 0x25), cqg1_u(c + 0x27));
    let fim_ano = cqg1_u(c + 0x2f);
    if ini_ano > fim_ano || (ini_ano == fim_ano && !(ini_dia < read_i16(c + 0x2d))) {
        crate::comparacao::ramo("cqg_1 fim em 4 semanas");
        let p = mais_semanas(c + 0x25, tmp2.as_mut_ptr(), 4); // cqg1: as 4 semanas
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), p);
    }
    copiar_data(pessoa + 0x46, tmp.as_mut_ptr(), c + 0x2d);
    let idx = read_u32_as_usize(cqg1_registro(read_u32_as_usize(c))) as i32;
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
            write_u16(c + 0x3b, cqg1_u(r() + 0x3b));
            let nacao = read_u32_as_usize(clube + 0x53);
            if nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(KEY_NATION_PORTUGAL) && comunitaria(pessoa, 0) == 0 {
                let p = mais_dias(0xae2c90, tmp2.as_mut_ptr(), 0x64);
                let (dia, ano) = (read_i16(p), cqg1_u(p + 2));
                let depois = ano > cqg1_u(pessoa + 0x40) || (ano == cqg1_u(pessoa + 0x40) && dia > read_i16(pessoa + 0x3e));
                if depois {
                    let primeira = read_u32_as_usize(pessoa + 0x1a);
                    if (primeira != 0 && read_u32_as_usize(primeira) == read_u32_as_usize(KEY_NATION_BRAZIL)) || sorteio(3) == 0 {
                        write_u32_from_usize(pessoa + 0x1e, read_u32_as_usize(0xae23a8).wrapping_add(read_u32_as_usize(KEY_NATION_PORTUGAL).wrapping_mul(0x122)));
                    }
                }
            }
        } else {
            cqg1_tipo_pelo_cargo(c);
            write_u32_from_usize(c + 0x4a, u32::MAX as usize);
            write_u8(c + 0x49, 0);
            zerar(c, pessoa, cqg1_clube(read_u32_as_usize(c + 4)));
            write_u16(c + 0x3b, 0);
        }
        cqg1_copiar(read_u32_as_usize(this).wrapping_add(off), c);
    } else {
        let conta = read_u32_as_usize(this + 8) as i32;
        let mut i = 0i32;
        while i < conta {
            let r = read_u32_as_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if read_u32_as_usize(r) as i32 == -1 { // cqg1: a vaga livre
                crate::comparacao::ramo("cqg_1 vaga livre");
                cqg1_tipo_pelo_cargo(c);
                write_u8(c + 0x49, 0);
                write_u32_from_usize(c + 0x4a, u32::MAX as usize);
                zerar(c, pessoa, cqg1_clube(read_u32_as_usize(c + 4)));
                cqg1_copiar(r, c);
                write_u32_from_usize(cqg1_registro(read_u32_as_usize(r)), i as u32 as usize);
                let f = financas(read_u32_as_usize(0xae2a38), cqg1_clube(read_u32_as_usize(c + 4)));
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
                aviso_em(0x9a8190, 0x1637);
            }
        }
        cqg1_tipo_pelo_cargo(c);
        write_u8(c + 0x49, 0);
        write_u32_from_usize(c + 0x4a, u32::MAX as usize);
        zerar(c, pessoa, cqg1_clube(read_u32_as_usize(c + 4)));
        write_u16(c + 0x3b, 0);
        write_u32_from_usize(cqg1_registro(read_u32_as_usize(c)), read_u32_as_usize(this + 8));
        let n = read_u32_as_usize(this + 8);
        let r = read_u32_as_usize(this).wrapping_add(n.wrapping_mul(0x50));
        write_u32_from_usize(this + 8, n.wrapping_add(1));
        cqg1_copiar(r, c);
    }
    crate::comparacao::ramo("cqg_1 valor na pessoa");
    write_u32_from_usize(pessoa + 0x4e, read_u32_as_usize(c + 0xc)); // cqg1: o valor do contrato na pessoa
    if read_u8(c + 0x35) & 0x3f == 2 && read_u32_as_usize(pessoa + 0x61) != 0 {
        moral(pessoa, 0x14, 0);
    }
    let f = financas(read_u32_as_usize(0xae2a38), cqg1_clube(read_u32_as_usize(c + 4)));
    if f != 0 {
        folha(f);
    }
    if cargo as u8 == read_u8(pessoa + 0x3d) {
        return 1;
    }
    if tirar(read_u32_as_usize(pessoa + 0x39), pessoa, 0, 1, 0) == 0 {
        aviso_em(0x9a8190, 0x1672);
        return 0;
    }
    if incluir(cqg1_clube(read_u32_as_usize(c + 4)), pessoa, cargo, pessoa + 0x3e, 1, 0) != 0 {
        return 1;
    }
    aviso_em(0x9a8190, 0x167a);
    0
}

/// Comparador da 0x4cf720: só a original (inclui e tira a pessoa dos clubes, realoca o vetor dos contratos, entrega
/// notícias, sorteia a segunda nacionalidade, recalcula a folha).
unsafe extern "thiscall" fn comparar_assinar_contrato(this: usize, contrato: usize, cargo: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1629)(this, contrato, cargo) // ponytail: só a original
}

// ---------- 270a leva: o contrato que o clube oferece a uma pessoa (contract_manager.cpp) ----------


unsafe fn cqf3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}






/// Os três bônus do contrato (+0x10, +0x14, +0x18): os que não são −1 (> −1) × `f`, truncados.
unsafe fn cqf3_bonus(c: usize, f: f64) {
    for k in 0..3 {
        let a = c + 0x10 + 4 * k;
        let v = cqf3_i(a);
        if v > -1 {
            write_u32(a, ftol(v as f64 * f) as u32);
        }
    }
}

/// { 0x4cda70 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH do STAFF_CONTRACT local sem destrutor; x87):
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
unsafe extern "thiscall" fn contrato_oferecido(this: usize, pessoa: usize, clube_arg: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
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
        } else if (read_u32_as_usize(pessoa) as i32) < cqf3_i(0xae2364) {
            let idx = cqf3_i(read_u32_as_usize(0xae2d0c).wrapping_add(read_u32_as_usize(pessoa).wrapping_mul(0x4f)));
            if idx < 0 { 0 } else { (idx as usize).wrapping_mul(0x50).wrapping_add(read_u32_as_usize(read_u32_as_usize(0xae1050))) }
        } else {
            0
        };
        write_u32(0xae1080, k as u32);
        if k == 0 {
            return 0;
        }
        clube = read_u32_as_usize(0xae23b4).wrapping_add(read_u32_as_usize(k + 4).wrapping_mul(0x245));
        if clube == 0 {
            return 0;
        }
    }
    if e_selecao(clube) != 0 {
        return 0;
    }
    if read_u8(read_u32_as_usize(0xae2d0c).wrapping_add(read_u32_as_usize(pessoa).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return 0;
    }
    if tipo_11_15(read_u8(pessoa + 0x3d) as i8) == 0 && clube == read_u32_as_usize(pessoa + 0x39) {
        let lista = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(0xde1f54) + 0x856) + 0xdc);
        if read_u8(lista.wrapping_add(read_u32_as_usize(clube).wrapping_mul(9)) + 4) == 2 || (read_u32_as_usize(clube + 0xcf) == 0 && dirigivel(clube) != 0) {
            return 0;
        }
    }
    if humano(clube, 0, 1, 0, 0) != 0 {
        return 0;
    }
    if futura(read_u32_as_usize(0xde1f54), pessoa, 0) != 0 {
        return 0;
    }
    write_u32(c + 4, read_u32_as_usize(clube) as u32);
    let mut oferta = 0usize;
    if vaga != -1 {
        let p = vaga_da(read_u32_as_usize(0xde1f54), vaga);
        if p != 0 {
            oferta = read_u32_as_usize(p);
        }
    }
    let mut cargo: u8 = 0;
    let mut est = estatuto_de(read_u32_as_usize(read_u32_as_usize(0xde1f54) + 0x856), pessoa, clube, &mut cargo);
    if est == 3 && read_i16(read_u32_as_usize(pessoa + 0x61) + 0xb) > 0x1482 {
        est = 2;
    }
    // ponytail: o estatuto e o cargo vão em dwords com os bytes altos de sobra da pilha na original; a 0x85fc50 e a
    // 0x597db0 leem o byte
    let r = desejado(c, pessoa, clube, vaga, est as u32, cargo as u32, 0, 0);
    if r as f64 == f64_em(0x9689a0) {
        return 0;
    }
    write_u32(c + 8, ftol(cqf3_i(c + 8) as f64 * fator) as u32);
    write_u32(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * fator) as u32);
    if read_u8(c + 0x1d) != 0 {
        let mut piso: i32 = 0;
        if oferta != 0 {
            let v = valor_oferta(oferta, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                piso = ftol((v - 0x4c4b40) as f64 * f64_em(0x968928) + f64_em(0x968e28));
            } else if v > 0xf4240 {
                piso = ftol((v - 0xf4240) as f64 * f64_em(0x968940) + f64_em(0x968e20));
            } else if v > 0xc350 {
                piso = ftol((v - 0xc350) as f64 * f64_em(0x968e18) + f64_em(0x9688f0));
            } else if cqf3_i(c + 0x21) < 0x186a0 {
                piso = 0x186a0;
            }
        } else if cqf3_i(c + 0x21) < 0xc350 {
            piso = 0xc350;
        }
        if cqf3_i(c + 0x21) < piso {
            write_u32(c + 0x21, piso as u32);
        }
    } else if (read_u8(c + 0x1f) != 0 || read_u8(c + 0x1c) != 0) && cqf3_i(c + 0x21) < 0xc350 {
        write_u32(c + 0x21, 0xc350);
    }
    let mut teto: f64 = if cargo == 0xb {
        let nibble = (read_u8(c + 0x4f) >> 4) as u32;
        let f = financas(read_u32_as_usize(0xae2a38), clube);
        let m = teto_max(f, pessoa, nibble);
        (fator + f64_em(0x967890)) * f64_em(0x968968) * m as f64 // cqf3: o teto do estatuto 11
    } else {
        let f = financas(read_u32_as_usize(0xae2a38), clube);
        teto_pessoal(f, cargo as i8) as f64
    };
    let atual = contrato_com(this, pessoa, clube);
    let mut ano = read_i16(0xae2c92) as u16;
    let ficha = read_u32_as_usize(pessoa + 0x61);
    if ficha != 0 {
        let dias = dias_ate(0xae2c90, read_u32_as_usize(pessoa + 0x10) as u32, read_u32_as_usize(pessoa + 0x14) as u32);
        let mut x = dias as f64 * f64_em(0x968e10);
        if read_u8(ficha + 0xf) == 0x14 {
            x -= f64_em(0x968950);
        } else {
            let s = read_u8(ficha + 0x3c) as i8 as i32 + read_u8(ficha + 0x34) as i8 as i32 + read_u8(pessoa + 0x5b) as i8 as i32;
            x -= (s as f64 * f64_em(0x968a80) - f64_em(0x968e08)) * f64_em(0x968e00);
        }
        let fim = |c: usize| core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32;
        if x > f64_em(0x968df8) || (atual != 0 && read_u8(atual + 0x4f) & 0xf0 == 0x60) {
            if fim(c) > ano as u32 + 1 {
                crate::comparacao::ramo("cqf_3 fim no ano que vem");
                write_u16(c + 0x2f, ano.wrapping_add(1)); // cqf3: o fim no ano que vem
                write_u32(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
            }
            cqf3_bonus(c, f64_em(0x968928));
        } else if x > f64_em(0x968df0) {
            if fim(c) > ano as u32 + 2 {
                write_u16(c + 0x2f, ano.wrapping_add(2));
                write_u32(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
            }
            cqf3_bonus(c, f64_em(0x968de8));
        } else if x > f64_em(0x968de0) && read_i16(clube + 0x80) as f64 > read_i16(ficha + 0xd) as f64 * f64_em(0x968978) {
            let k = ftol(x + f64_em(0x968968)) as i8;
            if fim(c) as i32 > (ano as i32 - k as i32) + 0x22 {
                write_u16(c + 0x2f, ano.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                write_u32(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
            }
            cqf3_bonus(c, f64_em(0x968940));
        }
        if cqf3_i(c + 0xc) > 0x2710 && x > f64_em(0x968dd8) {
            let dias_fim = dias_ate(c + 0x2d, read_u32_as_usize(0xae2c90) as u32, read_u32_as_usize(0xae2c94) as u32);
            let mut semanas = dias_fim / 7;
            let mut total = semanas.wrapping_mul(cqf3_i(c + 0xc));
            let mut a = pow_do_jogo(read_i16(ficha + 0xb) as f64, f64_em(0x968950)) * f64_em(0x968dd0);
            if !(a < f64_em(0x968dc8)) {
                a = 8000.0;
            }
            let mut b = pow_do_jogo(x - f64_em(0x968dd8), f64_em(0x968950));
            if !(b > f64_em(0x967890)) {
                b = 1.0;
            }
            let limite = ftol(pow_do_jogo(a, f64_em(0x968950)) / b);
            let reduz = (limite as f64 * f64_em(0x968940)) < total as f64;
            let mut feito = false;
            if reduz {
                while semanas > 0x68 {
                    semanas -= 0x34;
                    write_u32(c + 0xc, ftol(cqf3_i(c + 0xc) as f64 * f64_em(0x9689c0)) as u32);
                    total = semanas.wrapping_mul(cqf3_i(c + 0xc));
                    if limite >= total {
                        feito = true;
                        break;
                    }
                }
            }
            if !feito && total > limite {
                let q = limite / semanas;
                if (q as f64) < teto {
                    crate::comparacao::ramo("cqf_3 teto pelo total");
                    teto = q as f64; // cqf3: o teto pelo total
                }
            }
        }
        ano = read_i16(0xae2c92) as u16;
    }
    if atual != 0 {
        if core::ptr::read_unaligned((atual + 0x2f) as *const u16) >= core::ptr::read_unaligned((c + 0x2f) as *const u16) {
            return 0;
        }
        if read_u32_as_usize(0xb63d28) != 0 {
            let l = lesao(read_u32_as_usize(0xb63d28), pessoa);
            if l != 0 && read_i16(lesao(read_u32_as_usize(0xb63d28), pessoa) + 0x2c) > 0x46 {
                return 0;
            }
            ano = read_i16(0xae2c92) as u16;
        }
    }
    if read_u8(0xae233c) == 1 && core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32 > ano as u32 + 5 {
        write_u16(c + 0x2f, ano.wrapping_add(5));
    }
    let w = cqf3_i(c + 0xc) as f64;
    if w > teto * f64_em(0x968dc0) {
        return 0;
    }
    if w > teto {
        crate::comparacao::ramo("cqf_3 acima do teto");
        let den = if f64_em(0x967890) > teto { f64_em(0x967890) } else { teto };
        let razao = w / den;
        write_u32(c + 8, ftol(cqf3_i(c + 8) as f64 * razao) as u32);
        cqf3_bonus(c, razao);
        write_u32(c + 0xc, ftol(teto) as u32); // cqf3: o valor no teto
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
        let p = mais(0xae1068, d.as_mut_ptr(), 0x96);
        let (dia, ano_d) = (read_i16(p), core::ptr::read_unaligned((p + 2) as *const u16));
        let ano_a = core::ptr::read_unaligned((atual + 0x2f) as *const u16);
        ano_d < ano_a || (ano_d == ano_a && dia < read_i16(atual + 0x2d))
    };
    if enviar {
        // ponytail: o cargo (+0x3d) vai num dword com os bytes altos de sobra do edx na original; a 0x4cc490 lê o byte
        propor(this, c, vaga, u32::MAX, read_u8(pessoa + 0x3d) as u32, 0, 1, 0);
    }
    1
}

/// Comparador da 0x4cda70: com `saida`, lado a lado (a saída, [0xae1080] e o estado do sorteador; a cópia sem contar os
/// sorteios); sem `saida` (pode propor o contrato, que aloca), só a original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_contrato_oferecido(this: usize, pessoa: usize, clube: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, f64, u32) -> u32>(1602);
    if saida == 0 {
        return o(this, pessoa, clube, saida, vaga, fator, modo); // ponytail: só a original
    }
    let est = crate::aleatorio::REGIOES_DO_ESTADO;
    let regioes = [(saida, 0x50usize), (0xae1080usize, 4usize), est[0], est[1]];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || o(this, pessoa, clube, saida, vaga, fator, modo),
        || crate::aleatorio::sem_contar(|| contrato_oferecido(this, pessoa, clube, saida, vaga, fator, modo)));
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
        "contrato_oferecido(pessoa {:#x}, clube {:#x}, vaga {}, fator {}, modo {}): original {} rust {}, memória {:x?} x {:x?}",
        pessoa, clube, vaga, fator, modo, r, n, mo, mr));
    r
}

// ---------- 263a leva: os contratos depois do jogo (os bônus, a felicidade, a moral e a evolução de quem ficou de fora), a moral da ficha e o texto dela (contract_manager.cpp) ----------


unsafe fn cqa8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



/// Soma `delta` ao byte (felicidade +0x3d, moral +0x44 do contrato) entre −100 e 100, como a original: a soma em 32
/// bits decide o teto e o piso; dentro deles, a soma do byte.
unsafe fn cqa8_somar(a: usize, delta: i32) {
    let v = read_i8(a);
    let s = v as i32 + delta;
    *(a as *mut i8) = if s > 100 { 100 } else if s < -100 { -100 } else { v.wrapping_add(delta as i8) };
}

/// Os dois clubes do jogo (+0x1c mandante, +0x20 visitante): o do lado e o outro.
unsafe fn cqa8_clubes(jogo: usize, lado: usize) -> (usize, usize) {
    if lado == 0 { (read_usize(jogo + 0x1c), read_usize(jogo + 0x20)) } else { (read_usize(jogo + 0x20), read_usize(jogo + 0x1c)) }
}

/// Com o byte +0x3a do jogo = 1, o lado fica de fora quando a reputação do outro clube (+0x80) fica abaixo do maior
/// entre a dele × [0x968928] e a dele + 1500 (x87 em dupla, como a original).
unsafe fn cqa8_pula_lado(jogo: usize, lado: usize) -> bool {
    if read_i8(jogo + 0x3a) != 1 {
        return false;
    }
    let (meu, outro) = cqa8_clubes(jogo, lado);
    let r = read_i16(meu + 0x80) as i32;
    let x = r as f64 * f64_em(0x968928);
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > read_i16(outro + 0x80) as f64
}

/// A pessoa da vaga do resumo (id em +0 e +4 diferente de −1; registro de 0x6e bytes em [0xae23bc]), ou None quando a
/// original pula a vaga. A pessoa nula (base 0) passa, como na original.
unsafe fn cqa8_pessoa_da_vaga(vaga: usize) -> Option<usize> {
    let id = cqa8_i(vaga);
    if id == -1 || cqa8_i(vaga + 4) == -1 {
        return None;
    }
    let p = read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if p != 0 && cqa8_i(p) >= cqa8_i(0xae2364) {
        return None;
    }
    Some(p)
}

/// Os três bônus do contrato pela vaga (Finance::pay_bonuses 0x599960): +0x21 × +0x10, +0x27 × +0x14 e, com a palavra
/// do resumo = 1 e +0x22, +0x27 × +0x18 (a original usa o +0x27 de novo); cada um só com o byte e o valor positivos.
/// Sem bônus no amistoso (0x5c6710), no jogo da competição [0x9cf684] e nos contratos de tipo 5 e 6.
unsafe fn cqa8_bonus(jogo: usize, contrato: usize, vaga: usize, palavra: i16) -> [Option<i32>; 3] {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let comp = read_usize(jogo + 0x14);
    if comp != 0 && (amistoso(read_usize(0xae2c74), comp) != 0 || read_usize(jogo) == read_usize(KEY_CLUB_COMP_RESERVE)) {
        return [None; 3];
    }
    let tipo = *((contrato + 0x35) as *const u8) & 0x3f;
    if tipo == 6 || tipo == 5 {
        return [None; 3];
    }
    let g = *((vaga + 0x21) as *const u8) as i32;
    let a = *((vaga + 0x27) as *const u8) as i32;
    let um = (g != 0 && cqa8_i(contrato + 0x10) > 0).then(|| g.wrapping_mul(cqa8_i(contrato + 0x10)));
    let dois = (a != 0 && cqa8_i(contrato + 0x14) > 0).then(|| a.wrapping_mul(cqa8_i(contrato + 0x14)));
    let tres = (palavra == 1 && read_i8(vaga + 0x22) != 0 && cqa8_i(contrato + 0x18) > 0).then(|| a.wrapping_mul(cqa8_i(contrato + 0x18)));
    [um, dois, tres]
}

/// { 0x4ca6d0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::update_after_match(jogo, resumo)`, pelo
/// fechamento da partida do dia (0x6b5f00). Com +0x40 e +0x41 do jogo, o vencedor pelo placar (+0x47/+0x48; com +0x47 =
/// −1, +0x43/+0x44) e uma cópia dos 50 do elenco (+0xd7) de cada clube. Primeiro, por lado (clube B do parceiro
/// 0x540a50 e o lado pulado pela reputação ficam de fora), as 20 vagas do resumo (+0x190 + lado × 0x625, 0x49 bytes):
/// fora das seleções, o contrato com o clube (0x4cf5f0) paga os bônus (cqa8_bonus); os minutos (+0x30 dos titulares,
/// 90 sem valor; 90 − +0x32 dos reservas) mudam a felicidade (+0x3d, minutos × 10 / 90, presa em 50 abaixo de 50) ou,
/// nas seleções, o +9 do registro de 79 bytes; fora das seleções e dos amistosos, os reservas e os titulares de pouco
/// jogo sorteiam (2 × sorteio(30)) contra os minutos: a felicidade cai 4, +0x3e conta e a moral cai (0x4cb250); senão
/// +0x3f conta (com 5 ou mais, fora do clube humano, a marca de elenco 1 em 0x540980) e a moral sobe (0x4cb0a0). O
/// vencedor (e o perdedor) sorteia(21 − +0x57) abaixo de 4 para +3 (−3) na moral +0x44; a pessoa sai da cópia do
/// elenco. Depois, quem ficou na cópia (com ficha, contrato, sem suspensão 0x55d270, lesão 0x62c400, seleção 0x7757d0
/// nem empréstimo) evolui (0x6d26b0 com a nota média [0xb74350] − 1), +0x3e conta e, a partir de 8 (14 com parceiro),
/// fora do humano e do parceiro, sai do principal (0x4cbb60 2 ou 4 → 0x4cbd20); a felicidade cai 8 e a moral cai.
unsafe extern "thiscall" fn contratos_depois_do_jogo(this: usize, jogo: usize, resumo: usize) {
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
        let c = cqa8_clubes(jogo, lado).0;
        core::ptr::copy_nonoverlapping((c + 0xd7) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let clube = cqa8_clubes(jogo, lado).0;
        if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if cqa8_pula_lado(jogo, lado) {
            continue;
        }
        let fin = financas(read_usize(0xae2a38), clube);
        let base = resumo + lado * 0x625;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = cqa8_pessoa_da_vaga(vaga) else { continue };
            let contrato = if selecao(clube) != 0 {
                0
            } else {
                let c = com_clube(this, pessoa, clube);
                if c == 0 {
                    continue;
                }
                for v in cqa8_bonus(jogo, c, vaga, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
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
                    let r = read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 9;
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
            if selecao(clube) == 0 && (cqa8_i(jogo) < 0 || amistoso(read_usize(0xae2c74), read_usize(jogo + 0x14)) == 0) {
                let testa = k >= 11 || !(minutos > 0x1e || read_i8(vaga + 0x1f) != 0 || read_i8(vaga + 0x20) < 0x46);
                let mal = testa && {
                    crate::comparacao::ramo("cqa_8 testa a moral");
                    let m = read_i8(contrato + 0x44) as i32;
                    let r1 = sorteio(0x1e); // cqa8: o primeiro sorteio da moral
                    let v = r1 as f64 + (f64_em(0x968d68) - m as f64 * f64_em(0x968d70));
                    let r2 = sorteio(0x1e);
                    v - r2 as f64 > minutos as f64
                };
                let p = contrato as *mut u8;
                if mal {
                    *p.add(0x3f) = 0;
                    if *p.add(0x3e) != 100 {
                        *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
                    }
                    cqa8_somar(contrato + 0x3d, -4);
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
                let r = sorteio(0x15 - read_i8(pessoa + 0x57) as i32); // cqa8: o sorteio do vencedor
                if r < 4 && contrato != 0 {
                    cqa8_somar(contrato + 0x44, 3);
                }
            } else if venceu[1 - lado] != 0 {
                let r = sorteio(0x15 - read_i8(pessoa + 0x57) as i32);
                if r < 4 && contrato != 0 {
                    cqa8_somar(contrato + 0x44, -3);
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
        let (clube, outro) = cqa8_clubes(jogo, lado);
        let limite: i8 = if parceiro(clube, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if cqa8_pula_lado(jogo, lado) {
            continue;
        }
        for i in 0..50usize {
            let pessoa = lista[lado][i];
            if pessoa == 0 || cqa8_i(pessoa) >= cqa8_i(0xae2364) || read_usize(pessoa + 0x61) == 0 || selecao(clube) != 0 {
                continue;
            }
            let contrato = com_clube(this, pessoa, clube);
            if contrato == 0 {
                continue;
            }
            if cqa8_i(jogo) != -2 && suspenso(read_usize(0xae28f0), pessoa, read_usize(jogo + 0x14), 1, 0) != 0 {
                continue;
            }
            if lesao(read_usize(0xb63d28), pessoa) != 0 || impedido(read_usize(0xb74318), pessoa, 0xae2c90, clube, 0, 0) != 0 {
                continue;
            }
            // emprestado: o segundo contrato da pessoa (+4 do registro de 79 bytes)
            let idx = cqa8_i(read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 4);
            if idx >= 0 && (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)) != 0 {
                continue;
            }
            let m = read_usize(0xb74350);
            let x = core::ptr::read_unaligned((m + 0x18) as *const f64);
            let c = f64_em(0x9689a0);
            // fcomp + C3: igual ou desordenado vai para a constante
            let nota = if !(x < c || x > c) { f64_em(0x968998) } else { core::ptr::read_unaligned((m + 0x10) as *const f64) / x };
            crate::comparacao::ramo("cqa_8 evolui quem ficou de fora");
            evoluir(0, pessoa, clube, outro, jogo, (nota - f64_em(0x967888)) as f32, 0, 0); // cqa8: a evolução de quem ficou de fora
            let p = contrato as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if read_i8(contrato + 0x3e) >= limite && humano(clube, 0, 1, 0, 0) == 0 && parceiro(clube, core::ptr::null_mut(), 1) == 0 {
                let l = lugar(this, pessoa, clube);
                if l == 2 || l == 4 {
                    trocar(this, pessoa, clube, 0);
                }
            }
            cqa8_somar(contrato + 0x3d, -8);
            baixar(this, pessoa, clube, contrato);
        }
    }
}

/// Comparador: a moral (0x4cb0a0, 0x4cb250: queixas, pedido de transferência), a evolução (0x6d26b0) e a troca de
/// elenco (0x4cbd20) sorteiam e alocam (não podem rodar duas vezes): só a original roda. Confere o que a rotina garante
/// antes de qualquer sorteio: os bônus que cada clube paga (+0x138 das finanças soma o total previsto pelas mesmas
/// regras da cópia, calculado antes). A cópia é provada pelos ramos e pelas mutações na carreira.
unsafe extern "thiscall" fn comparar_contratos_depois_do_jogo(this: usize, jogo: usize, resumo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1440);
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
        let clube = cqa8_clubes(jogo, lado).0;
        if (parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0) || cqa8_pula_lado(jogo, lado) || selecao(clube) != 0 {
            continue;
        }
        let fin = financas(read_usize(0xae2a38), clube);
        let base = resumo + lado * 0x625;
        let mut total = 0i32;
        for k in 0..20usize {
            let vaga = base + 0x190 + k * 0x49;
            let Some(pessoa) = cqa8_pessoa_da_vaga(vaga) else { continue };
            let c = com_clube(this, pessoa, clube);
            if c != 0 {
                for v in cqa8_bonus(jogo, c, vaga, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    total = total.wrapping_add(v);
                }
            }
        }
        if fin != 0 {
            *pl = (fin, cqa8_i(fin + 0x138), total);
        }
    }
    o(this, jogo, resumo);
    for &(fin, antes, total) in &plano {
        if fin != 0 {
            let pago = cqa8_i(fin + 0x138).wrapping_sub(antes);
            PLACAR.registrar(pago == total, || format!("contratos_depois_do_jogo(jogo {:x}) bônus de fin {:x}: original {} previsto {}", jogo, fin, pago, total));
        }
    }
}

/// { 0x4cc2d0 } (stdcall, ret 4, contract_manager.cpp): `contract_manager_get_player_morale(pessoa)`: o byte +0x45 da
/// ficha (+0x61), pelas telas. O eax leva os bytes altos da pessoa, como na original.
unsafe extern "stdcall" fn moral_da_ficha(pessoa: usize) -> u32 {
    (pessoa as u32 & 0xffff_ff00) | *((read_usize(pessoa + 0x61) + 0x45) as *const u8) as u32
}

unsafe extern "stdcall" fn comparar_moral_da_ficha(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1441)(pessoa);
    let n = moral_da_ficha(pessoa);
    PLACAR.registrar(r == n, || format!("moral_da_ficha({:x}) original={:#x} rust={:#x}", pessoa, r, n));
    r
}

/// O texto terminado em zero em `a` (até 0x400 bytes), para os comparadores dos textos.
unsafe fn cqa8_texto(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// { 0x4cc2e0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_write_morale_text(pessoa, saída, _)`: a
/// palavra da moral (+0x45 da ficha: abaixo de 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very Good", senão
/// "Superb") na saída (0x66f4e0 com 0xad9c64), pelas telas.
unsafe extern "stdcall" fn texto_da_moral(pessoa: usize, saida: usize, _x: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let m = read_i8(read_usize(pessoa + 0x61) + 0x45);
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

/// Comparador: a original escreve a palavra; a cópia escreve de novo (o texto se sobrescreve) e as duas têm de bater;
/// fica o texto da original.
unsafe extern "stdcall" fn comparar_texto_da_moral(pessoa: usize, saida: usize, x: u32) {
    orig::<unsafe extern "stdcall" fn(usize, usize, u32)>(1442)(pessoa, saida, x);
    let r = cqa8_texto(saida);
    texto_da_moral(pessoa, saida, x);
    let n = cqa8_texto(saida);
    core::ptr::copy_nonoverlapping(r.as_ptr(), saida as *mut u8, r.len());
    *((saida + r.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n, || format!("texto_da_moral({:x}) original={:?} rust={:?}", pessoa, String::from_utf8_lossy(&r), String::from_utf8_lossy(&n)));
}

// ---------- 263a leva: o valor do jogador, a comparação da moral de dois jogadores e as marcas de elenco da tela do clube (contract_manager.cpp) ----------




/// O clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cqa9_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato principal da pessoa (índice +0 do registro de 79 bytes em [0xae2d0c], registros de 0x50 bytes em
/// [[0xae1050]]), ou 0: sem pessoa, fora das pessoas ([0xae2364]) ou sem índice.
unsafe fn cqa9_contrato_principal(pessoa: usize) -> usize {
    if pessoa == 0 || read_usize(pessoa) as i32 >= read_usize(0xae2364) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f))) as i32;
    if i < 0 { 0 } else { (i as usize).wrapping_mul(0x50).wrapping_add(read_usize(read_usize(0xae1050))) }
}

/// O mínimo com a cláusula de rescisão (+0x21 do contrato, se 0x867b50 a diz ativa) e o truncamento (_ftol).
unsafe fn cqa9_fim(contrato: usize, w: f64) -> i32 {
    let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_HAS_ACTIVE_RELEASE_CLAUSE);
    if contrato != 0 && clausula(contrato) != 0 {
        let cl = core::ptr::read_unaligned((contrato + 0x21) as *const i32) as f64;
        return ftol(if !(w >= cl) { w } else { cl });
    }
    ftol(w)
}

/// { 0x4d0c60 } (thiscall, ret 0x14, contract_manager.cpp): `CONTRACT_MANAGER::calc_player_value(pessoa, clube, modo,
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
unsafe extern "thiscall" fn valor_do_jogador(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, _x: u32) -> i32 {
    let c1080 = cqa9_contrato_principal(pessoa);
    *(0xae1080 as *mut usize) = c1080;
    let clube_do_contrato = if c1080 == 0 { 0 } else { cqa9_clube(read_usize(c1080 + 4)) };
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let contrato = com_clube(this, pessoa, clube_do_contrato);
    let ficha = read_usize(pessoa + 0x61);
    if ficha == 0 {
        if clube_do_contrato == 0 {
            return 0;
        }
        // a original põe ecx = [0xde1f54] (a cópia Rust é stdcall): thiscall com 3 na pilha serve aos dois
        let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
        return compensacao(read_usize(0xde1f54), pessoa, clube_do_contrato, 1);
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
    let idade = read_i8(pessoa + 0x18);
    let dif = auge as i32 - idade as i32;
    let mut t = dif as f64;
    if t > f64_em(0x9688a0) {
        t = f64_em(0x968f10) - t * f32_em(0x968f18) as f64;
    }
    let a = ftol(t).wrapping_abs();
    let mut f = t * a as f64 * f32_em(0x968f08) as f64 * f64_em(0x968f00) + f64_em(0x967890);
    if read_i8(ficha + 0xf) < 0xf {
        let nota = |o: usize, c: usize| (read_i8(ficha + o) as i32 - 0xf) as f64 * f64_em(c) + f64_em(0x967890);
        let fa = nota(0x28, 0x968ef8);
        let fb = read_i8(ficha + 0x27) as f64 * f64_em(0x968ef0) + f64_em(0x967890);
        let fc = read_i8(ficha + 0x43) as f64 * f64_em(0x968e60) + f64_em(0x967890);
        let fd = nota(0x36, 0x968ef8);
        let fe = nota(0x2f, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = read_i8(ficha + 0x2d) as i32;
    let fator = ((f64_em(0x968ee0) - (q * q) as f64 * f64_em(0x967888) * f64_em(0x968ee8)) * f) as f32;
    let mut r = read_i16(ficha + 9) as f64 * f64_em(0x968ed8) + f64_em(0x968ed0);
    if !(r >= f64_em(0x9689c0)) {
        r = f64_em(0x9689c0);
    }
    let x: f32 = if !(fator as f64 >= f64_em(0x967898)) {
        f64_em(0x967898) as f32
    } else if !(r >= fator as f64) {
        r as f32
    } else {
        fator
    };
    let mut v: u32 = if clube == 0 {
        read_i16(ficha + 0xb) as i32 as u32
    } else {
        let vista: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
        let tecnico = read_usize(clube + 0xcf);
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
        let y = ftol((s as f64 + teto as f64 * x as f64) * f32_em(0x968f08) as f64) as u32;
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = read_i16(ficha + 0xd);
    if pot > 0x1388 && (idade as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && idade <= auge {
            let z = pot as f64 * f64_em(0x968ec8) + (v >> 1) as f64 - f64_em(0x968ec0);
            let e = v as f64;
            v = ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * f64_em(0x968eb8);
    let mut idade_ef = idade;
    let exp = *((pessoa + 0x22) as *const u8);
    if exp != 0 && read_usize(pessoa + 0x1a) != 0 {
        let n = (read_i16(read_usize(pessoa + 0x1a) + 0x8e) as i32).wrapping_mul(exp as i32) / 18;
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
    let mut w = tab as f64 * vv * f64_em(0x967898);
    let c890 = f64_em(0x967890);
    let k = if clube == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        (sorteio(3) + 1) as f64
    } else {
        let div = read_usize(clube + 0x57);
        let nat = read_usize(clube + 0x53);
        if nat == 0 {
            c890
        } else {
            let mut t = read_i8(nat + 0x7e) as f64;
            if !(t >= c890) || t > f64_em(0x968930) {
                t = f64_em(0x968930);
            }
            if div != 0 {
                t = (t + c890) * f64_em(0x968968);
            }
            match read_i8(clube + 0x64) {
                2 => t = t + t,
                3 => t *= f64_em(0x968eb0),
                _ => {}
            }
            let n = if div != 0 { read_i16(div + 0x69) as i32 } else { read_i8(nat + 0x85) as i32 };
            let e = n.wrapping_mul(n).wrapping_mul(25).wrapping_add(read_i16(clube + 0x80) as i32);
            e as f64 / (t * f64_em(0x968908))
        }
    };
    let dd = f64_em(0x968ea8) - k;
    let mut l = c890 - dd * dd * f64_em(0x968918) * f64_em(0x968a98);
    if !(l >= f64_em(0x967880)) {
        l = f64_em(0x967880);
    } else if l > f64_em(0x9689c0) {
        l = f64_em(0x9689c0);
    }
    w = l * w;
    let k2 = k * k;
    if !(k2 * f64_em(0x968ea0) >= w) {
        w = w * f64_em(0x9689e0) + k2 * f64_em(0x968e98);
    }
    let piso = k * f64_em(0x9688e8);
    if !(w >= piso) {
        w = piso;
    }
    let nacao_e = |g: usize| -> bool {
        let nat = read_usize(clube + 0x53);
        clube != 0 && nat != 0 && read_usize(nat) == read_usize(g)
    };
    // a nação [0x9cf434]: abaixo de [0x968e90] a curva com piso, senão × [0x9689d8]
    let curva = |w: f64, piso: f64| -> f64 {
        if !(w >= f64_em(0x968e90)) {
            let v = (w * f64_em(0x968e88) * f64_em(0x968e48) + f64_em(0x968e80)) * w;
            if !(f64_em(0x9688e8) > v) { v } else { piso }
        } else {
            w * f64_em(0x9689d8)
        }
    };
    if modo as u8 != 0 {
        if nacao_e(KEY_NATION_PORTUGAL) {
            w = curva(w, 5000.0);
        }
        return cqa9_fim(contrato, w);
    }
    let mut nacoes = true; // 0x4d1480 (as outras nações) ou 0x4d14ad (a [0x9cf434] antes)
    if contrato == 0 {
        w = f64_em(0x9689a0);
    } else {
        let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_CLUB_USES_BOSMAN);
        if bosman(read_usize(0xde1f54), pessoa) != 0 {
            let ano = |a: usize| core::ptr::read_unaligned(a as *const u16) as i32;
            let mut dias = (ano(contrato + 0x2f) - ano(0xae2c92)).wrapping_mul(365) + read_i16(contrato + 0x2d) as i32 - read_i16(0xae2c90) as i32;
            if dias < 1 {
                return 0;
            }
            let mut w2 = w;
            if clube == clube_do_contrato && read_i8(contrato + 0x3d) > 0 {
                if *((contrato + 0x35) as *const u8) & 0x3f == 2 {
                    dias = dias + 7 * read_i8(contrato + 0x3d) as i32 + 0xc8;
                } else {
                    w2 = w * f64_em(0x968968);
                }
            }
            let x = dias as f64 * f64_em(0x968e78);
            if dias < 0x15e { // cqa9: o contrato curto
                w = w2 * x;
            } else {
                let mut dv = f64_em(0x968e18) - x;
                if !(dv >= c890) {
                    dv = f32_em(0x968e70) as f64;
                }
                if dv > f64_em(0x968950) {
                    dv = f32_em(0x9689a8) as f64;
                }
                w = w2 / dv;
            }
        }
        nacoes = false;
    }
    if !nacoes {
        // 0x4d14ad: valor zero (ou desordenado) vai às outras nações; sem clube, ao fim
        if !(w < f64_em(0x9689a0) || w > f64_em(0x9689a0)) {
            nacoes = true;
        } else if clube == 0 {
            return cqa9_fim(contrato, w);
        } else if nacao_e(KEY_NATION_PORTUGAL) {
            w = curva(w, f64_em(0x9688e8));
            return cqa9_fim(contrato, w);
        } else {
            nacoes = true;
        }
    }
    if nacoes && clube != 0 {
        if nacao_e(KEY_NATION_FRANCE) {
            w *= f64_em(0x9689b0);
        } else if nacao_e(KEY_NATION_SOUTH_KOREA) {
            w *= f64_em(0x9689e0);
            let c = f64_em(0x968e68);
            if w > c {
                w = ftol((w - c) * f64_em(0x968968) + c) as f64;
            }
            let c = f64_em(0x9688e0);
            if w > c {
                w = ftol((w - c) * f64_em(0x968968) + c) as f64;
            }
        }
    }
    cqa9_fim(contrato, w)
}

/// As estáticas das datas (CPQ6_ESTATICAS) e o contrato guardado em [0xae1080]: o que a original e as chamadas dela
/// gravam, com o estado do sorteador (na pilha: a rotina é quente).
fn cqa9_regioes() -> [(usize, usize); 11] {
    let (e, c) = (crate::aleatorio::REGIOES_DO_ESTADO, CPQ6_ESTATICAS);
    [e[0], e[1], (0xae1080, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}

/// Comparador: lado a lado (sem alocar) com o estado do sorteador (sem clube, sorteio(3)), o [0xae1080] e as estáticas
/// das datas da compensação; a cópia sem contar os sorteios. Quente (5,6 milhões de chamadas): sem contador.
unsafe extern "thiscall" fn comparar_valor_do_jogador(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, x: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32>(1443);
    let regioes = cqa9_regioes();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, clube, modo, habilidade, x),
        || crate::aleatorio::sem_contar(|| valor_do_jogador(this, pessoa, clube, modo, habilidade, x)));
    PLACAR.registrar(igual && r == n, || format!("valor_do_jogador(pessoa {:x}, clube {:x}, modo {}, hab {}) original={} rust={} {:x?} x {:x?}", pessoa, clube, modo as u8, habilidade as i16, r, n, mo, mr));
    r
}

/// { 0x4cc3a0 } (stdcall, ret 0x10, contract_manager.cpp): `contract_manager_write_morale_comparison_text(a, b, saída,
/// _)`: a diferença das morais (+0x45 das fichas, a − b, em byte) vira "Superior" (até −14 e desde 13), "Better" (−13 a
/// −8 e 7 a 12), "Marginal" (−7 a −4 e 3 a 6) ou "Same" (−3 a 2) na saída (0x66f4e0 com 0xad9c64); devolve a diferença
/// (o jogo lê o al), pela tela de comparação.
unsafe extern "stdcall" fn diferenca_de_moral_em_texto(a: usize, b: usize, saida: usize, _x: u32) -> u8 {
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

/// O texto terminado em zero em `a` (até 0x400 bytes).
unsafe fn cqa9_texto(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// Comparador: a original escreve; a cópia escreve de novo (sobrescreve) e o texto e a diferença têm de bater; fica o
/// texto da original.
unsafe extern "stdcall" fn comparar_diferenca_de_moral_em_texto(a: usize, b: usize, saida: usize, x: u32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, usize, u32) -> u8>(1444)(a, b, saida, x);
    let tr = cqa9_texto(saida);
    let n = diferenca_de_moral_em_texto(a, b, saida, x);
    let tn = cqa9_texto(saida);
    core::ptr::copy_nonoverlapping(tr.as_ptr(), saida as *mut u8, tr.len());
    *((saida + tr.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n && tr == tn, || format!("diferenca_de_moral_em_texto original={} {:?} rust={} {:?}", r as i8, String::from_utf8_lossy(&tr), n as i8, String::from_utf8_lossy(&tn)));
    r
}

/// { 0x4cc130 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_flags_for_club(pessoa, clube)`,
/// pelas telas do clube: as marcas de elenco da pessoa (+0x5e sem os bits de seleção, & 0xf3), a não ser que ela esteja
/// emprestada (segundo contrato, +4 do registro de 79 bytes) a outro clube que não o `clube` nem o parceiro dele
/// (0x540a50): com parceiro, 1; senão 2 com o bit 0x40 de +0x35 do contrato principal, 1 sem. Sem clube, a caixa
/// "Error" (a original). O jogo lê o al.
unsafe extern "thiscall" fn marcas_de_elenco_para_a_tela(this: usize, pessoa: usize, clube: usize) -> u8 {
    if clube == 0 || pessoa == 0 {
        // ponytail: sem clube a caixa de erro; sem pessoa a original lê [0x5e] e cai: as duas ficam com a original
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1445)(this, pessoa, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let geral = || *((pessoa + 0x5e) as *const u8) & 0xf3;
    if read_usize(pessoa) as i32 >= read_usize(0xae2364) as i32 {
        return geral();
    }
    let reg = read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f));
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
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1445)(this, pessoa, clube);
    }
    let c0 = (i0 as usize).wrapping_mul(0x50).wrapping_add(read_usize(this));
    (*((c0 + 0x35) as *const u8) & 0x40 != 0) as u8 + 1
}

unsafe extern "thiscall" fn comparar_marcas_de_elenco_para_a_tela(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1445);
    if clube == 0 || pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let r = o(this, pessoa, clube);
    let n = marcas_de_elenco_para_a_tela(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("marcas_de_elenco_para_a_tela({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

// ---------- 259a leva: a satisfação do jogador com o contrato, a notícia de que ele quer contrato novo e a do pedido de transferência retirado (staff_contracts.cpp, contract_manager.cpp) ----------




/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw1_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw1_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O valor de um contrato para a pessoa (STAFF_CONTRACT::calc_contract_value 0x866830: thiscall(contrato, pessoa,
/// oferta), ret 8).
unsafe fn cpw1_valor(contrato: usize, pessoa: usize, oferta: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_CALC_CONTRACT_VALUE);
    f(contrato, pessoa, oferta)
}

/// { 0x866620 } (thiscall, ret 4, staff_contracts.cpp, quadro SEH sem destrutor): `STAFF_CONTRACT::
/// get_contract_satisfaction(pessoa)`, pelo passe diário do pessoal (0x5ce2d0, com sorteio(50) zero) e pela tela do
/// contrato: 1 a 3. Sem pessoa, 1. A situação no elenco que ela merece (0x4ca470) vale se for menor que a do contrato
/// (+0x4f >> 4) e não for 0 nem 7; senão, a do contrato. O contrato desejado (0x85fc50 num contrato local, com o clube
/// do contrato, −1, a situação e 0xb) e a razão 100 × valor do contrato atual (0x866830, em f32) / valor do desejado
/// (valor zero: razão 0). Contratado há menos de 15 × adaptabilidade (+0x59) + 100 dias (de +0x25, teto 500): razão +
/// 50 + 7,5 × adaptabilidade − dias / 2. Razão abaixo de 50: 1; abaixo de 80: 3; senão 2.
unsafe extern "thiscall" fn satisfacao_com_o_contrato(this: usize, pessoa: usize) -> u8 {
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
    desejado(l, pessoa, cpw1_clube(read_usize(this + 4)), -1, situacao, 0xb, 0, 0);
    let v1 = cpw1_valor(this, pessoa, l) as f32;
    let v2 = cpw1_valor(l, pessoa, l) as f64;
    let mut razao = if v2 == f32_em(0x968970) as f64 {
        f32_em(0x968970) as f64
    } else {
        v1 as f64 * f32_em(0x968960) as f64 / v2
    };
    let anos = (read_i16(0xae2c92) as u16 as i32).wrapping_sub(read_i16(this + 0x27) as u16 as i32);
    let dias = anos.wrapping_mul(365).wrapping_sub(read_i16(this + 0x25) as i32).wrapping_add(read_i16(0xae2c90) as i32).clamp(0, 500);
    let adapt = read_u8(pessoa + 0x59) as i8 as i32;
    if dias < adapt.wrapping_mul(15).wrapping_add(100) {
        crate::comparacao::ramo("cpw_1 contratado ha pouco");
        razao = razao + f32_em(0x969560) as f64 + adapt as f64 * f64_em(0x968eb0) - (dias / 2) as f64; // cpw1: o desconto do recém-contratado
    }
    crate::comparacao::ramo("cpw_1 satisfacao avaliada");
    if !(razao >= f32_em(0x969560) as f64) { // cpw1: o limite da insatisfação
        return 1;
    }
    if razao < f32_em(0x96ff70) as f64 { // cpw1: o limite da satisfação
        3
    } else {
        2
    }
}

/// Comparador: a original e a cópia a partir do mesmo estado (o sorteio, o último contrato pedido [0xae1080] que a
/// 0x85fc50 grava e o contrato); a devolução e a memória.
unsafe extern "thiscall" fn comparar_satisfacao_com_o_contrato(this: usize, pessoa: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1272);
    let mut regioes = vec![(0xae1080usize, 4usize), (this, 0x50)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, pessoa),
        || crate::aleatorio::sem_contar(|| satisfacao_com_o_contrato(this, pessoa)),
    );
    PLACAR.registrar(igual && r == n, || {
        format!("satisfacao_com_o_contrato(pessoa {:#x}) original={} {:x?} rust={} {:x?}", if pessoa != 0 { read_usize(pessoa) } else { 0 }, r, mo, n, mr)
    });
    r
}

/// Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
unsafe fn cpw1_dados_da_pessoa(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    gravar(n, 2, read_usize(p) as u32);
    gravar(n, 3, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, 4, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(n, 5, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// { 0x4d9f80 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_wants_new_contract_news(contrato)`, pelo passe diário do pessoal (0x5ce2d0). Sem contrato: a caixa "Error"
/// (linha 0x2d91). Pessoa na lista de transferências (0x8c1870), com ofertas (0x8c5cf0) ou oferecida em troca (0x8c5e40):
/// nada. Senão a notícia 0xbc2 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, a situação no elenco (+0x4f
/// >> 4), 0, o fim do contrato (+0x2d dia com sinal, +0x2f ano), o salário (+0xc) e, na vaga 11, 1 se a adaptabilidade
/// (+0x59) for menor que 10, a +0x5b menor que 5 e a reputação da ficha (+0xd) passar de 4750.
unsafe extern "thiscall" fn noticia_quer_contrato_novo(this: usize, contrato: usize) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2d91);
    }
    let na_lista: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_QUEUED_FOR_FREE_TRANSFER);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let clube = cpw1_clube(read_usize(contrato + 4));
    let p = cpw1_pessoa(read_usize(contrato));
    let tm = read_usize(0xde1f54);
    if na_lista(tm, p) != 0 || ofertas(tm, p, 0, 0) != 0 || trocas(tm, p, 0) != 0 {
        return;
    }
    crate::comparacao::ramo("cpw_1 noticia do contrato novo");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc2, 0);
    gravar(n, 0, read_usize(clube) as u32);
    cpw1_dados_da_pessoa(n, p);
    gravar(n, 6, (read_u8(contrato + 0x4f) >> 4) as u32);
    gravar(n, 7, 0);
    gravar(n, 8, read_i16(contrato + 0x2d) as i32 as u32);
    gravar(n, 9, read_i16(contrato + 0x2f) as u16 as u32);
    gravar(n, 10, read_usize(contrato + 0xc) as u32);
    let ficha = read_usize(p + 0x61);
    let reputado = (read_u8(p + 0x59) as i8) < 0xa && (read_u8(p + 0x5b) as i8) < 5 && ficha != 0 && read_i16(ficha + 0xd) > 0x128e;
    gravar(n, 11, reputado as u32);
    entregar(read_usize(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_quer_contrato_novo(this: usize, contrato: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1273)(this, contrato)
}

/// { 0x4d9460 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_transfer_request_withdrawn_news(contrato, motivo)`. Sem contrato: a caixa "Error" (linha 0x2ba6). Senão a
/// notícia 0xbc0 ao técnico do clube do contrato (0x7861e0): o clube, a pessoa, o motivo e, na vaga 7, 1 se +0x3f do
/// contrato passar de 1.
unsafe extern "thiscall" fn noticia_pedido_de_transferencia_retirado(this: usize, contrato: usize, motivo: u32) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2ba6);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpw_1 pedido retirado");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbc0, 0);
    let clube = cpw1_clube(read_usize(contrato + 4));
    let p = cpw1_pessoa(read_usize(contrato));
    gravar(n, 0, read_usize(clube) as u32);
    cpw1_dados_da_pessoa(n, p);
    gravar(n, 6, motivo);
    gravar(n, 7, ((read_u8(contrato + 0x3f) as i8) > 1) as u32);
    entregar(read_usize(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_pedido_de_transferencia_retirado(this: usize, contrato: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1274)(this, contrato, motivo)
}

// ---------- 259a leva: a oferta de empréstimo pelo jogador da lista, a notícia do contrato acabando e a da saída pela lei Bosman (shortlist_manager.cpp, contract_manager.cpp) ----------




/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw3_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw3_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0).
unsafe fn cpw3_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    contrato_de(read_usize(0xae1050), p)
}

/// O interesse da pessoa em ir para o clube (Shortlist_Manager::staff_interest_in_joining_club 0x846920: thiscall(lista,
/// pessoa, clube, salário, modo, situação, motivos), ret 0x18, float no st0).
unsafe fn cpw3_interesse(lista: usize, pessoa: usize, clube: usize, salario: u32, modo: u32, situacao: u32) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    f(lista, pessoa, clube, salario, modo, situacao, 0)
}

/// O ensaio do comparador: a cópia roda sem a oferta de empréstimo (0x8bed60) nem a atualização da lista (0x843e10),
/// que alocam e mexem nas listas; devolve −2 onde faria a oferta e −3 onde atualizaria a lista.
static mut CPW3_ENSAIO: bool = false;

/// A data (dia i16, ano u16) de 8 bytes em `a`, como a original a compara (o ano sem sinal, o dia com sinal).
unsafe fn cpw3_antes(a: usize, b: usize) -> bool {
    let (aa, ab) = (read_i16(a + 2) as u16, read_i16(b + 2) as u16);
    aa < ab || (aa == ab && read_i16(a) < read_i16(b))
}

/// { 0x842740 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH sem destrutor): `Shortlist_Manager::
/// try_loan_offer_for_player(jogador, clube)`, pela avaliação das listas dos clubes (0x84a020): a vaga da oferta de
/// empréstimo, ou −1. Sem dono (contrato 0x4cf5a0, guardado em [0xae1080]): a vaga da lista (0x849ce0) volta ao estado 1
/// (0x843e10) e −1. O interesse (0x846920, com a situação da vaga) abaixo de 3,5: −1. O período do empréstimo (0x8d0590):
/// os meses até o fim e até o início (no mínimo 1); início depois do fim: −1; clube da nação 0x9cf408 fora de março a
/// setembro: −1. O interesse sem salário de 2,75 (fora das nações 0x9cf488, 0x9cf434, 0x9cf360) conta do início; senão do
/// fim. A data de volta (dia de hoje, no máximo 28; 0x549f70); sem poder encerrar o empréstimo (0x8d0530), depois do fim
/// do contrato, antes de 4 semanas do começo da janela da nação-chave (0x669360, [0xb63d8a]) ou a menos de 3 semanas de
/// hoje: −1. A porcentagem do salário (100 com reputação 2500 acima do dono, 0 com 1500 abaixo, senão 50) e a oferta
/// (0x8bed60).
unsafe extern "thiscall" fn tentar_emprestimo_pelo_jogador(this: usize, jogador: usize, clube: usize) -> i32 {
    let entrada_da_lista: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let entrada = entrada_da_lista(this, clube, jogador, 0, 0);
    let c = cpw3_contrato(jogador);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 { cpw3_clube(read_usize(c + 4)) } else { 0 };
    if dono == 0 {
        if entrada != 0 {
            if CPW3_ENSAIO {
                return -3;
            }
            let estado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
            estado(this, jogador, clube, 1, 0);
        }
        return -1;
    }
    // ponytail: a original empurra a situação da vaga num registrador cujo resto é o endereço do clube dono (base +
    // id); a cópia reproduz o dword.
    let situacao = (read_usize(0xae23b4).wrapping_add(read_usize(c + 4)) as u32 & !0xff) | ((read_u8(entrada + 0x24) >> 1) & 0xf) as u32;
    let f1 = cpw3_interesse(read_usize(read_usize(0xde1f54) + 0x856), jogador, clube, 0, 1, situacao);
    crate::comparacao::ramo("cpw_3 interesse no emprestimo");
    if !(f1 >= f64_em(0x968e38)) { // cpw3: o interesse mínimo
        return -1;
    }
    let mes_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    let m = mes_de(0xae2c90) as u8;
    let c = cpw3_contrato(jogador);
    *(0xae1080 as *mut usize) = c;
    let a2 = if c != 0 { cpw3_clube(read_usize(c + 4)) } else { 0 };
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i8, *mut u16, *mut i8, *mut u16) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    // ponytail: o mês do fim vai para a vaga do argumento do clube (o byte baixo), o resto fica; os outros são locais
    let (mut ini_mes, mut ini_ano, mut mes, mut ano) = (0i8, 0u16, clube as u8 as i8, 0u16);
    periodo(read_usize(0xde1f54), clube, a2, jogador, &mut ini_mes, &mut ini_ano, &mut mes, &mut ano);
    let hoje_ano = read_i16(0xae2c92) as u16;
    let m16 = m as i8 as i16;
    let ate_fim = (ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(mes as i16);
    let ate_ini = (ini_ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(ini_mes as i16);
    let inicio = if ate_ini >= 1 { ate_ini } else { 1 };
    if inicio > ate_fim {
        return -1;
    }
    let nacao = read_usize(clube + 0x53);
    if nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_NORWAY) && (m as i8 >= 0xa || m as i8 <= 2) {
        return -1;
    }
    let f2 = cpw3_interesse(this, jogador, clube, 0, 0, 1);
    let mut meses = ate_fim as u8;
    crate::comparacao::ramo("cpw_3 conta do inicio");
    if f2 >= f64_em(0x9690a8) { // cpw3: o interesse para contar do início
        let n = read_usize(read_usize(clube + 0x53));
        if n != read_usize(KEY_NATION_SPAIN) && n != read_usize(KEY_NATION_PORTUGAL) && n != read_usize(KEY_NATION_ITALY) {
            meses = inicio as u8;
        }
    }
    let mut mes_volta = meses.wrapping_add(m);
    // ponytail: o ano vai na vaga do argumento do jogador (a palavra baixa; o resto é o endereço do jogador), e a
    // porcentagem depois no byte baixo dela; a cópia reproduz o dword.
    let mut vaga = (jogador as u32 & 0xffff_0000) | hoje_ano as u32;
    if mes_volta as i8 > 0xb {
        let q = (mes_volta as i8 as i32 as u32) / 12;
        mes_volta = mes_volta.wrapping_add((q as u8).wrapping_mul(0xf4));
        vaga = vaga.wrapping_add(q);
    }
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let dia = dia_do_mes(0xae2c90) as i16;
    // a original relê o dia (segunda chamada) quando ele não passa de 28
    let dia = if dia > 0x1c { 0x1c } else { dia_do_mes(0xae2c90) as i16 as i32 };
    let data_dma: unsafe extern "thiscall" fn(*mut [u16; 4], i32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut volta = [0u16; 4];
    data_dma(&mut volta, dia, mes_volta as u32, vaga, 0xffff_ffff);
    let v = volta.as_ptr() as usize;
    let encerrar: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_END_LOAN);
    if encerrar(read_usize(0xde1f54), clube, jogador, 0, v, 1) == 0 {
        return -1;
    }
    let c = cpw3_contrato(jogador);
    if c == 0 || cpw3_antes(c + 0x2d, v) {
        return -1;
    }
    let chave: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let k = chave(clube) as u8 as i8 as i32;
    let janela = (0xb63d8a + k * 72) as usize;
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let mut t = [0u16; 4];
    let r = menos_semanas(v, &mut t, 4);
    if cpw3_antes(r, janela) {
        return -1;
    }
    crate::comparacao::ramo("cpw_3 tres semanas");
    let r = mais_semanas(0xae2c90, &mut t, 3); // cpw3: as 3 semanas a partir de hoje
    if cpw3_antes(v, r) {
        return -1;
    }
    let dif = read_i16(clube + 0x80) as i32 - read_i16(dono + 0x80) as i32;
    let pct = if dif > 0x9c4 { 100 } else if dif <= -0x5dc { 0 } else { 50 };
    let vaga = (vaga & !0xff) | pct;
    if CPW3_ENSAIO {
        return -2;
    }
    let oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_MAKE_LOAN_OFFER);
    oferta(read_usize(0xde1f54), jogador, clube, v, vaga, 1, 1) as i32
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado ([0xae1080], o sorteio e as estáticas das
/// operações de data). A cópia recusou (−1): a original também, com a mesma memória. A cópia atualizaria a lista (−3):
/// a original devolve −1. A cópia faria a oferta (−2): a original não devolve −1.
unsafe extern "thiscall" fn comparar_tentar_emprestimo_pelo_jogador(this: usize, jogador: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(1278);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend(CPQ5_ESTATICAS);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW3_ENSAIO = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| tentar_emprestimo_pelo_jogador(this, jogador, clube)));
    CPW3_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = o(this, jogador, clube);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = match n {
        -1 => r == -1 && mo == mr,
        -3 => r == -1,
        _ => r != -1,
    };
    PLACAR.registrar(igual, || format!("tentar_emprestimo_pelo_jogador(jogador {:#x}, clube {:#x}) original={} {:x?} rust={} {:x?}", read_usize(jogador), read_usize(clube), r, mo, n, mr));
    r
}

/// Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
unsafe fn cpw3_dados_da_pessoa(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    gravar(n, 2, read_usize(p) as u32);
    gravar(n, 3, read_usize(read_usize(p + 4) + 0x33) as u32);
    gravar(n, 4, read_usize(read_usize(p + 8) + 0x33) as u32);
    gravar(n, 5, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// A nação está ativa (0x544670, cdecl)?
unsafe fn cpw3_nacao_ativa(nacao: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_NATION_ALLOWS_FREE_TRANSFERS);
    f(nacao) != 0
}

/// { 0x4d6790 } (thiscall, ret 0x10, contract_manager.cpp, quadro SEH da notícia e da lista locais): `CONTRACT_MANAGER::
/// post_contract_expiry_news(contrato, período, bosman, avisar_interessados)`, pelo passe dos contratos. Sem contrato: a
/// caixa "Error" (linha 0x24f0). Senão a notícia 0xbbd ao técnico do clube (0x7861e0): o clube, a pessoa, o período (com
/// sinal), `bosman` e, na vaga 8, se a pessoa tem ficha (+0x61). Com `avisar_interessados` (anulado se a nação do clube,
/// +0x53, não estiver ativa, 0x544670): os clubes interessados no jogador (0x842180 numa LLIST local, com 1, 1, 0) de
/// outra nação ativa também a recebem.
unsafe extern "thiscall" fn noticia_contrato_acabando(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x24f0);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbd, 0);
    let clube = cpw3_clube(read_usize(contrato + 4));
    let p = cpw3_pessoa(read_usize(contrato));
    gravar(n, 0, read_usize(clube) as u32);
    cpw3_dados_da_pessoa(n, p);
    gravar(n, 6, periodo as u8 as i8 as i32 as u32);
    gravar(n, 7, bosman);
    gravar(n, 8, (read_usize(p + 0x61) != 0) as u32);
    let nacao = read_usize(clube + 0x53);
    let avisar = if nacao != 0 && !cpw3_nacao_ativa(nacao) { 0 } else { avisar };
    entregar(read_usize(0xb7431c), n, clube);
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
    interessados(read_usize(read_usize(0xde1f54) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut x = proximo(l);
    while x != 0 {
        let id = read_usize(x + 4);
        if id != read_usize(clube) {
            let nac = read_usize(cpw3_clube(id) + 0x53);
            if nac != 0 && cpw3_nacao_ativa(nac) {
                entregar(read_usize(0xb7431c), n, cpw3_clube(read_usize(x + 4)));
            }
        }
        x = proximo(l);
    }
    destruir(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_contrato_acabando(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1279)(this, contrato, periodo, bosman, avisar)
}

/// { 0x4d6d70 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_bosman_departure_news(contrato, destino)`. Sem contrato: a caixa "Error" (linha 0x2611). Senão a notícia 0xbbe ao
/// técnico do clube (0x7861e0): o clube, a pessoa e o clube de destino (−1 sem ele).
unsafe extern "thiscall" fn noticia_saida_pela_lei_bosman(this: usize, contrato: usize, destino: usize) {
    let _ = this;
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2611);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cpw_3 saida pela lei bosman");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbe, 0);
    let clube = cpw3_clube(read_usize(contrato + 4));
    let p = cpw3_pessoa(read_usize(contrato));
    gravar(n, 0, read_usize(clube) as u32);
    cpw3_dados_da_pessoa(n, p);
    gravar(n, 6, if destino != 0 { read_usize(destino) as u32 } else { 0xffff_ffff });
    entregar(read_usize(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_noticia_saida_pela_lei_bosman(this: usize, contrato: usize, destino: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1280)(this, contrato, destino)
}

// ---------- 259a leva: a oferta de contrato acrescentada ou atualizada, a oferta de contrato tirada pela vaga e o menor salário pedido (contract_manager.cpp) ----------




/// O contador de ofertas de contrato da pessoa (byte +8 do registro de 79 bytes em [0xae2d0c]).
unsafe fn cpw4_contador(pessoa: usize) -> usize {
    read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)).wrapping_add(8)
}

/// A atribuição de data (0x417c10: thiscall(destino, temporária, origem)).
unsafe fn cpw4_copiar_data(destino: usize, origem: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut [u32; 2], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u32; 2];
    copia(destino, &mut t, origem);
}

/// A oferta de contrato da pessoa (id em +0) pelo clube (id em +4) na lista `l` (nós: +0 o item, +6 o próximo), sem
/// mexer no cursor; nenhuma: 0.
unsafe fn cpw4_achar(l: usize, id: usize, clube: usize) -> usize {
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
static mut CPW4_ENSAIO: bool = false;

/// { 0x4cc490 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH sem destrutor): `CONTRACT_MANAGER::
/// add_contract_offer(termos, vaga, valor, marca, a5, decidir, acertado)`, pela oferta de empréstimo (0x8bed60), pelas
/// ofertas guardadas (0x4cf4a0), pelas listas (0x84a020), pela negociação (0x4cda70) e pela tela. Calcula hoje + 14 (0x54c5b0,
/// descartado) e monta um registro local de 0x6f bytes (contrato 0x85e410, data 0x54a110). Com ofertas contadas (+8 do
/// registro de 79 bytes da pessoa em [0xae2d0c]) e lista (+0x34), a oferta do mesmo clube é atualizada: os termos (o
/// operator= 0x4cc810 embutido), +0x50 = 2 se `acertado` (senão 0), +0x51 = hoje, +0x59 a vaga, +0x5d o valor, +0x61 a
/// marca, +0x62 = 1 fora do contrato de tipo 6. Senão o registro local recebe o mesmo (+0x63 = −1, o resto fica como a
/// original o deixa) e entra na lista (criada se faltar; 0x68d0d0), e o contador sobe. Com vaga, a oferta (0x8c61e0) do
/// mesmo jogador marca os termos oferecidos (0x8c1f80), senão o jogador da troca (0x8c2060); com `decidir`, agenda a
/// decisão (0x4cd230).
unsafe extern "thiscall" fn acrescentar_oferta_de_contrato(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let _ = a5;
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut quinzena = [0u32; 2];
    mais(0xae2c90, &mut quinzena, 0xe);
    // ponytail: os bytes +0x67..+0x6e do registro local a original não preenche (sobra de pilha); a cópia usa zero
    let mut registro = [0u8; 0x6f];
    let r = registro.as_mut_ptr() as usize;
    construir(r);
    data_padrao(r + 0x51);
    let id = read_usize(termos);
    let mut item = 0;
    if read_u8(cpw4_contador(termos)) != 0 && read_usize(this + 0x34) != 0 {
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
        copiar_contrato_de_pessoal(item, termos);
        *((item + 0x50) as *mut u8) = if acertado != 0 { 2 } else { 0 };
        cpw4_copiar_data(item + 0x51, 0xae2c90);
        write_u32(item + 0x59, vaga as u32);
        write_u32(item + 0x5d, valor);
        *((item + 0x61) as *mut u8) = marca as u8;
        *((item + 0x62) as *mut u8) = !tipo6 as u8;
    } else {
        let operador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_OPERATOR_3D);
        operador(r, termos);
        *((r + 0x50) as *mut u8) = 0;
        cpw4_copiar_data(r + 0x51, 0xae2c90);
        write_u32(r + 0x5d, valor);
        write_u32(r + 0x59, vaga as u32);
        *((r + 0x61) as *mut u8) = marca as u8;
        write_u32(r + 0x63, 0xffff_ffff);
        *((r + 0x62) as *mut u8) = !tipo6 as u8;
        if CPW4_ENSAIO {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(CPW4_REGISTRO) as *mut u8, 0x6f);
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
        let c = cpw4_contador(termos) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: o contador de ofertas da pessoa
    }
    if CPW4_ENSAIO {
        return;
    }
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
        let tm = read_usize(0xde1f54);
        let v = vaga_da(tm, vaga);
        if read_usize(read_usize(v) + 0xc) == read_usize(termos) {
            crate::comparacao::ramo("cpw_4 termos ao jogador");
            let marcar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_TRANSFER_MANAGER_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER);
            marcar(read_usize(0xde1f54), vaga); // cpw4: os termos oferecidos ao jogador
        } else {
            let troca: unsafe extern "thiscall" fn(usize, i32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_NOTE_TERMS_OFFERED_TO_EXCHANGE_PLAYER);
            troca(read_usize(0xde1f54), vaga, read_usize(termos) as u32);
        }
    }
    if decidir != 0 {
        crate::comparacao::ramo("cpw_4 agendar decisao");
        let decisao: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_OFFER_DECISION);
        decisao(this, read_usize(termos) as u32, vaga); // cpw4: a decisão agendada
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
static mut CPW4_REGISTRO: [u8; 0x6f] = [0; 0x6f];

/// Comparador. Oferta já na lista (achada antes, sem mexer no cursor): a cópia em ensaio e a original a partir do mesmo
/// estado, comparando a oferta (0x63 bytes), o contador e a cabeça da lista (o cursor). Oferta nova (acrescentada, aloca):
/// a cópia em ensaio monta o registro; a original acrescenta, e o nó novo (o último do jogador pelo clube) tem de ter os
/// mesmos 0x63 primeiros bytes, com o contador + 1.
unsafe extern "thiscall" fn comparar_acrescentar_oferta_de_contrato(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32)>(1281);
    let id = read_usize(termos);
    let clube = read_usize(termos + 4);
    let cont = cpw4_contador(termos);
    let l = read_usize(this + 0x34);
    let item = if read_u8(cont) != 0 && l != 0 { cpw4_achar(l, id, clube) } else { 0 };
    let mut todas = vec![(cont, 1usize)];
    if item != 0 {
        todas.push((item, 0x63));
        todas.push((l, 0xe));
    }
    let n_cmp = todas.len();
    todas.extend(CPQ5_ESTATICAS);
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    CPW4_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| acrescentar_oferta_de_contrato(this, termos, vaga, valor, marca, a5, decidir, acertado)));
    CPW4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let registro = CPW4_REGISTRO;
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
    PLACAR.registrar(igual, || format!("acrescentar_oferta_de_contrato(pessoa {:#x}, clube {:#x}, vaga {}, atualizada {}) original={:x?} rust={:x?}", id, clube, vaga, item != 0, mo, mr));
}

/// { 0x4ccb30 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer_for_transfer(pessoa,
/// vaga)`, pelas ofertas retiradas e desfeitas (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): na lista das
/// ofertas de contrato (+0x34; cursor recomeçado) a da pessoa com a vaga (+0x59). Achada: com +0x63 marcado, a original
/// 0x7a81a0 de [0xb7434c] com +0x63, +0x67 e +0x6b; acertada (+0x50 = 2), desliga a marca de contrato acertado (0x8c5430);
/// tira da lista (0x68d1f0) e o contador de ofertas da pessoa (+8 do registro de 79 bytes) desce.
unsafe extern "thiscall" fn tirar_oferta_de_contrato_da_vaga(this: usize, pessoa: usize, vaga: i32) {
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
        f(read_usize(0xb7434c), read_usize(x + 0x63) as u32, read_usize(x + 0x67) as u32, read_usize(x + 0x6b) as u32);
    }
    if read_u8(x + 0x50) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
        desmarcar(read_usize(0xde1f54), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    tirar(read_usize(this + 0x34), x);
    let c = cpw4_contador(pessoa) as *mut u8;
    *c = (*c).wrapping_sub(1);
}

/// Comparador: a original tira o item da lista (libera): só ela, conferindo o plano feito antes: com a oferta da pessoa
/// pela vaga na lista, o contador desce 1; sem ela, fica.
unsafe extern "thiscall" fn comparar_tirar_oferta_de_contrato_da_vaga(this: usize, pessoa: usize, vaga: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(1282);
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
    let antes = read_u8(cpw4_contador(pessoa));
    o(this, pessoa, vaga);
    let depois = read_u8(cpw4_contador(pessoa));
    let esperado = if achou { antes.wrapping_sub(1) } else { antes };
    PLACAR.registrar(depois == esperado, || format!("tirar_oferta_de_contrato_da_vaga(pessoa {:#x}, vaga {}) achou {} contador {} -> {}", read_usize(pessoa), vaga, achou, antes, depois));
}

/// { 0x4ce450 } (stdcall, ret 8, contract_manager.cpp, quadro SEH sem destrutor): o menor salário que a pessoa pede ao
/// clube: num contrato local (0x85e410), o desejado (0x85fc50 com −1, a situação 1 e depois 2, 0xb), no máximo 10 milhões.
/// Pela tela do contrato.
unsafe extern "stdcall" fn menor_salario_pedido(pessoa: usize, clube: usize) -> i32 {
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

/// Comparador: a original e a cópia a partir do mesmo estado (o sorteio e o último contrato pedido [0xae1080]); a
/// devolução e a memória.
unsafe extern "stdcall" fn comparar_menor_salario_pedido(pessoa: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> i32>(1283);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(pessoa, clube), || crate::aleatorio::sem_contar(|| menor_salario_pedido(pessoa, clube)));
    PLACAR.registrar(igual && r == n, || format!("menor_salario_pedido(pessoa {:#x}) original={} {:x?} rust={} {:x?}", read_usize(pessoa), r, mo, n, mr));
    r
}

// ---------- 255a leva: o pedido de transferência do jogador, os cargos com contrato e quem não é da diretoria (contract_manager.cpp) ----------



/// { 0x4d8610 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// handle_transfer_request(contrato)`: contrato nulo: a caixa "Error" 0x2999. Pessoa sem ficha de jogador (+0x61): nada.
/// Clube humano (0x7875e0): a notícia do pedido (0xbbf), montada na pilha. No clube do computador, fora do status de
/// elenco 1 (+0x4f & 0xf0 = 0x10): se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), o clube o põe na lista
/// (+0x4e: tira o bit 2 e liga o 8; moral +12 em 0x544710; as listas dos clubes 0x842c00; 0x4d71b0). Senão um sorteio
/// pela força do pedido (+0x44) e pelo ânimo (+0x59) contra outro de 50 dá a queixa 0x10000 (0x867700); a moral cai 12
/// e a força do pedido sobe ânimo × 5 + 56, presa em ±100.
unsafe extern "thiscall" fn pedido_de_transferencia(this: usize, contrato: usize) {
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2999);
    }
    let clube = read_usize(0xae23b4).wrapping_add(read_usize(contrato + 4).wrapping_mul(0x245));
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(contrato).wrapping_mul(0x6e));
    if read_usize(pessoa + 0x61) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if humano(read_usize(0xb7431c), clube) != 0 {
        // ponytail: o clube humano recebe a notícia montada na pilha (0x77bbe0 e a entrega estão desligadas): esse
        // caso fica com a original (que refaz a pergunta 0x7875e0, sem efeito).
        // o registro serve à prova do "Deixar infeliz" do CMRTE (31/10/2026; deixar_infeliz.py): chamada rara
        linha(&format!("pedido de transferencia no clube humano: pessoa {}", read_usize(contrato)));
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(1116)(this, contrato);
    }
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    if read_u8(contrato + 0x4f) & 0xf0 != 0x10 {
        let preco: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
        let a = preco(this, clube, 0, pessoa, 0, 1, 1);
        let v = valor(this, pessoa, clube, 0, -1, 0);
        // o 1,5 é o double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: o valor passa do preço
            crate::comparacao::ramo("cps_1 listado pelo clube");
            let m = (contrato + 0x4e) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            moral(pessoa, 0xc, 0);
            let listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
            listas(read_usize(read_usize(0xde1f54) + 0x856), pessoa, 1);
            let listar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRANSFER_LISTED_NEWS);
            listar(this, pessoa, clube);
            return;
        }
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let r = sorteio(read_u8(contrato + 0x44) as i8 as i32 + (read_u8(pessoa + 0x59) as i8 as i32 + 5) * 20);
    if r < sorteio(0x32) { // cps1: o sorteio da queixa
        crate::comparacao::ramo("cps_1 queixa do pedido");
        let queixa: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        queixa(contrato, 0x10000, 0);
    }
    crate::comparacao::ramo("cps_1 pedido no clube do computador");
    moral(pessoa, 0xffff_fff4, 0); // cps1: a moral cai 12
    // relidos depois das chamadas, como na original (o `imul` de 8 bits fica com o byte baixo)
    let al = read_u8(pessoa + 0x59).wrapping_mul(5).wrapping_add(0x38);
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

/// { 0x4cbb20 } (stdcall, ret 4, contract_manager.cpp): `is_contract_staff_job(cargo)`: 1 para os cargos (byte) 0, 6 a
/// 0xd e 0xf, os que têm contrato pelo gerente de contratos; senão 0.
unsafe extern "stdcall" fn cargo_com_contrato(cargo: u32) -> u32 {
    matches!(cargo as u8, 0 | 6..=0xd | 0xf) as u32
}

/// { 0x4cb960 } (stdcall, ret 4, contract_manager.cpp): `staff_is_not_board_member(pessoa)`: 0 se o cargo (+0x3d) é de
/// diretoria (1 a 4) num clube (+0x39); senão 1.
unsafe extern "stdcall" fn fora_da_diretoria(pessoa: usize) -> u32 {
    let cargo = read_u8(pessoa + 0x3d);
    if (1..=4).contains(&cargo) && read_usize(pessoa + 0x39) != 0 {
        return 0;
    }
    1
}

// ponytail: sorteia (0x90cfc0), põe na lista (0x842c00, 0x4d71b0), muda moral e queixas (que podem chamar o pedido de
// novo): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_pedido_de_transferencia(this: usize, contrato: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1116)(this, contrato)
}

/// Comparador: pelo valor.
unsafe extern "stdcall" fn comparar_cargo_com_contrato(cargo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(u32) -> u32>(1117)(cargo);
    let n = cargo_com_contrato(cargo);
    PLACAR.registrar(r == n, || format!("cargo_com_contrato({}) original={} rust={}", cargo as u8, r, n));
    r
}

/// Comparador: pelo valor.
unsafe extern "stdcall" fn comparar_fora_da_diretoria(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1118)(pessoa);
    let n = fora_da_diretoria(pessoa);
    PLACAR.registrar(r == n, || format!("fora_da_diretoria(pessoa {}, cargo {}) original={} rust={}", read_usize(pessoa), read_u8(pessoa + 0x3d), r, n));
    r
}

// ---------- 252a leva: as datas de decisão e as ofertas do clube: a ordem das datas, decidir as datas vencidas e as ofertas de contrato do clube (contract_manager.cpp) ----------



/// As comparações de data do jogo (thiscall na data `a`, a outra por valor): 0x456740 `>`, 0x456770 `<`, 0x4a7f20 `==`.
unsafe fn cpp1_data(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, read_u32(b), read_u32(b + 4))
}

/// { 0x4dc910 } (cdecl, contract_manager.cpp): `compare_decision_dates(a, b)`: a ordem do qsort das datas de decisão
/// (entradas de 14 bytes: +0 pessoa, +4 data, +0xc fatia do dia, +0xd tipo): a data de `a` antes (0x456770): −1;
/// depois (0x456740): 1; no mesmo dia, a fatia de `a` menos a de `b` (com sinal). Quente (o qsort): sem contador.
unsafe extern "C" fn comparar_datas_de_decisao(a: usize, b: usize) -> i32 {
    if cpp1_data(0x456770, a + 4, b + 4) != 0 {
        return -1; // a antes de b
    }
    if cpp1_data(0x456740, a + 4, b + 4) != 0 {
        return 1;
    }
    read_u8(a + 0xc) as i8 as i32 - read_u8(b + 0xc) as i8 as i32
}

unsafe extern "C" fn comparar_comparar_datas_de_decisao(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1006);
    let r = o(a, b);
    let n = comparar_datas_de_decisao(a, b);
    PLACAR.registrar(r == n, || format!("comparar_datas_de_decisao original={} rust={}", r, n));
    r
}

/// { 0x4cd9c0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::process_due_decision_dates(data, fatia)`:
/// do índice +0x18 até a conta +0x14 do vetor das datas de decisão (+0xc, entradas de 14 bytes, ordenadas pela
/// 0x4dc910): para na primeira data depois de `data` (0x456740) ou, no mesmo dia (0x4a7f20), com a fatia da entrada
/// maior que `fatia`. Cada entrada vencida com pessoa (+0 ≠ −1) fica com −1 e, sendo do tipo 3 (+0xd), a pessoa decide
/// as ofertas de contrato (0x4ce4e0). O índice +0x18 sobe a cada entrada (relido depois da decisão).
unsafe extern "thiscall" fn decidir_as_datas_vencidas(this: usize, data: usize, fatia: u32) {
    let n = |c: usize| read_u32(c) as i32;
    if n(this + 0x18) >= n(this + 0x14) {
        return;
    }
    let resolver: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_RESOLVE_CONTRACT_OFFERS);
    loop {
        let e = (read_u32(this + 0xc) as usize).wrapping_add((n(this + 0x18) as usize).wrapping_mul(14));
        if cpp1_data(0x456740, e + 4, data) != 0 {
            return; // ainda não venceu
        }
        if cpp1_data(0x4a7f20, e + 4, data) != 0 && read_u8(e + 0xc) as i8 > fatia as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let pessoa = (read_u32(0xae23bc) as usize).wrapping_add((id as usize).wrapping_mul(0x6e));
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

/// Comparador: a decisão das ofertas (0x4ce4e0) mexe nos contratos, nas ofertas e no próprio vetor: o plano (as datas
/// lidas antes, sem rodar nada) diz se alguma entrada do tipo 3 com pessoa vence; com ela, só a original. Sem ela (o
/// caso de todo dia), lado a lado pelo índice +0x18 e pelas entradas que vencem (ficam com −1).
unsafe extern "thiscall" fn comparar_decidir_as_datas_vencidas(this: usize, data: usize, fatia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1007);
    let (i0, n) = (read_u32(this + 0x18) as i32, read_u32(this + 0x14) as i32);
    let mut i = i0;
    let mut decide = false;
    while i < n {
        let e = (read_u32(this + 0xc) as usize).wrapping_add((i as usize).wrapping_mul(14));
        if cpp1_data(0x456740, e + 4, data) != 0
            || (cpp1_data(0x4a7f20, e + 4, data) != 0 && read_u8(e + 0xc) as i8 > fatia as u8 as i8) {
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
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data, fatia), || decidir_as_datas_vencidas(this, data, fatia));
    PLACAR.registrar(igual, || format!("decidir_as_datas_vencidas(fatia {}) {:x?} x {:x?}", fatia as u8 as i8, mo, mr));
}

/// { 0x4ca300 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_current_contract_offers(clube, saida)`:
/// percorre a lista das ofertas (+0x34; cursor reposto, 0x68d1b0) e copia cada oferta do clube (+4 = id) fora dos
/// estados 5 e 6 (+0x35 & 0x3f) para um vetor novo de 111 bytes por oferta, que cresce de 20 em 20 (realloc 0x945667
/// com a capacidade em byte com sinal). Grava o vetor em `saida` e devolve quantas (byte). A original confere a lista
/// (+0x34), não o vetor, depois do realloc: só com a lista nula vai à caixa "cannot allocate memory" (linha 0x64c) e
/// devolve 0 com a saída nula. Quem a chama (0x598a06, o orçamento de transferências) libera o vetor.
unsafe extern "thiscall" fn ofertas_de_contrato_do_clube(this: usize, clube: usize, saida: usize) -> u8 {
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
                        cpl4_sem_espaco(0x64c, 0x9a8220); // "cannot allocate memory"
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

/// Comparador: a original aloca o vetor que quem chama libera: roda só ela e confere o plano feito antes (as ofertas
/// do clube lidas da lista, sem alocar): a conta e os 111 bytes de cada uma, na ordem.
unsafe extern "thiscall" fn comparar_ofertas_de_contrato_do_clube(this: usize, clube: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1008);
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
    PLACAR.registrar(ok, || format!("ofertas_de_contrato_do_clube(clube {:x}) original {} ofertas, plano {}", clube, r, k));
    r
}

// ---------- 252a leva: os empréstimos e o lugar no elenco: assinar o contrato de empréstimo, encerrar o empréstimo e trocar o lugar no elenco (contract_manager.cpp) ----------


unsafe fn cpp2_b(a: usize) -> u8 {
    // ponytail: leitura volátil porque a original lê [0 + 0x35] quando o contrato principal falta (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}

unsafe fn cpp2_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de 79 bytes da pessoa em [0xae2d0c] (+0 o contrato principal, +4 o de empréstimo; −1 sem).
unsafe fn cpp2_vaga(pessoa: usize) -> usize {
    (read_u32(0xae2d0c) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(0x4f))
}

/// A entrada `k` (0x50 bytes) do vetor de contratos que começa em `vetor`.
fn cpp2_contrato(vetor: u32, k: i32) -> usize {
    (vetor as usize).wrapping_add((k as usize).wrapping_mul(0x50))
}

/// O clube de id `id` (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cpp2_clube(id: u32) -> usize {
    (read_u32(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O contrato principal da pessoa no CONTRACT_MANAGER global ([[0xae1050]]), como as originais o refazem: pessoa fora
/// da faixa (id ≥ [0xae2364]) ou sem contrato principal: 0. Grava em [0xae1080] (a ficha global).
unsafe fn cpp2_ficha(pessoa: usize) -> usize {
    let c = if read_u32(pessoa) as i32 >= read_u32(0xae2364) as i32 {
        0
    } else {
        let k = read_u32(cpp2_vaga(pessoa)) as i32;
        if k < 0 { 0 } else { cpp2_contrato(read_u32(read_u32(0xae1050) as usize), k) }
    };
    cpp2_por(0xae1080, c as u32);
    c
}

/// A caixa "Error" do contract_manager.cpp na `linha`: "v%s %s.%s %d" em 0xae24d0 (0x944cff separa o caminho 0x9a8190,
/// 0x90d130 a versão; a original não desempilha os argumentos dela, que são o resto do formato), a caixa 0x5e8290 e
/// [0xb67a34] = 0. O jogo segue.
unsafe fn cpp2_erro(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    cpp2_por(0xb67a34, 0);
}

// As rotinas do banco que as três chamam (pelo endereço; as portadas passam pelo desvio).
unsafe fn cpp2_parceiro(clube: usize, eh_a: *mut u32) -> usize {
    let f: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    f(clube, eh_a, 1)
}
unsafe fn cpp2_marcar(pessoa: usize, elenco: u32) {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    f(pessoa, elenco);
}
unsafe fn cpp2_tatica(clube: usize, pessoa: usize, a: u32, b: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    f(read_u32(0xde1d08) as usize, clube, pessoa, a, b);
}
unsafe fn cpp2_listas(pessoa: usize, a4: u32, clube: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(FN_PLAYERSTATS_ARCHIVE_PLAYER_STATS);
    f(read_u32(0xb74350) as usize, pessoa, -1, 0, a4, clube, 0);
}
unsafe fn cpp2_tirar(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    f(clube, pessoa, data, desligar, como_jogador);
}
unsafe fn cpp2_por_no_clube(clube: usize, pessoa: usize, cargo: u32, data: usize, reduzir: u32, gravar: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
    f(clube, pessoa, cargo, data, reduzir, gravar);
}
unsafe fn cpp2_reservas(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    f(clube)
}
/// Refaz a folha de salários (0x598370) das finanças do clube (0x5a0590 em [0xae2a38]; a original chama mesmo sem).
unsafe fn cpp2_folha(clube: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
    folha(financas(read_u32(0xae2a38) as usize, clube));
}
unsafe fn cpp2_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut ret = [0u8; 8];
    copiar(destino, &mut ret, origem);
}

/// { 0x4d01e0 } (thiscall, ret 4, contract_manager.cpp, quadro SEH só com uma data local, sem destrutor):
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
unsafe extern "thiscall" fn assinar_contrato_de_emprestimo(this: usize, contrato: usize) {
    let mut guardada = [0u8; 8];
    let construir: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    construir(&mut guardada);
    let pessoa = (read_u32(0xae23bc) as usize).wrapping_add((read_u32(contrato) as usize).wrapping_mul(0x6e));
    let ficha = if pessoa == 0 { cpp2_por(0xae1080, 0); 0 } else { cpp2_ficha(pessoa) };
    if ficha == 0 {
        return;
    }
    let dono = cpp2_clube(read_u32(ficha + 4));
    if dono == 0 {
        return;
    }
    cpp2_data(guardada.as_mut_ptr() as usize, pessoa + 0x3e);
    let novo = cpp2_clube(read_u32(contrato + 4));
    cpp2_listas(pessoa, 1, read_u32(novo) as i32);
    let principal = if pessoa != 0 && (read_u32(pessoa) as i32) < read_u32(0xae2364) as i32 && (read_u32(cpp2_vaga(pessoa)) as i32) >= 0 {
        cpp2_contrato(read_u32(this), read_u32(cpp2_vaga(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        cpp2_erro(0x16a9);
        return;
    }
    crate::comparacao::ramo("cpp_2 assinar emprestimo");
    let m = principal + 0x35;
    let reserva = if cpp2_reservas(dono) != 0 {
        cpp2_b(pessoa + 0x5e) & 1 == 0
    } else if cpp2_parceiro(dono, core::ptr::null_mut()) != 0 {
        dono as u32 != read_u32(pessoa + 0x39)
    } else {
        false
    };
    let cl = cpp2_b(m);
    *(m as *mut u8) = if reserva { cl | 0x40 } else { cl & 0xbf };
    let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_VIRTUAL_STAFF_REMOVE_VIRTUAL_PLAYERS);
    avisar(read_u32(0xde2f70) as usize, novo, 0);
    cpp2_tatica(read_u32(pessoa + 0x39) as usize, pessoa, 0, 1);
    cpp2_tatica(read_u32(pessoa + 0x39) as usize, pessoa, 1, 1);
    cpp2_por_no_clube(novo, pessoa, 0xb, 0xae2c90, 1, 0); // o jogador no clube novo
    cpp2_data(pessoa + 0x3e, guardada.as_ptr() as usize);
    let emp = read_u32(cpp2_vaga(pessoa) + 4) as i32;
    if emp > -1 {
        // ponytail: a original copia em linha, igual ao STAFF_CONTRACT::operator= (0x4cc810, portado)
        copiar_contrato_de_pessoal(cpp2_contrato(read_u32(this), emp), contrato);
    } else {
        let n = read_u32(this + 8) as i32;
        let vetor = read_u32(this);
        let mut i = 0i32;
        while i < n {
            if read_u32(cpp2_contrato(vetor, i)) as i32 == -1 {
                copiar_contrato_de_pessoal(cpp2_contrato(vetor, i), contrato);
                cpp2_por(cpp2_vaga(pessoa) + 4, i as u32);
                return; // a original sai sem refazer as folhas
            }
            i += 1;
        }
        let cap = read_u32(this + 4) as i32;
        if n >= cap {
            let cap = cap + 100;
            cpp2_por(this + 4, cap as u32);
            let novo_vetor = if vetor != 0 {
                let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
                realocar(vetor as usize, (cap * 0x50) as u32)
            } else {
                let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
                alocar((cap * 0x50) as u32)
            };
            cpp2_por(this, novo_vetor as u32);
            if novo_vetor == 0 {
                cpp2_erro(0x1708);
            }
        }
        let n = read_u32(this + 8);
        cpp2_por(cpp2_vaga(pessoa) + 4, n);
        let destino = cpp2_contrato(read_u32(this), n as i32);
        cpp2_por(this + 8, n + 1);
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_OPERATOR_3D);
        copiar(destino, contrato);
    }
    cpp2_folha(dono);
    cpp2_folha(novo);
}

unsafe extern "thiscall" fn comparar_assinar_contrato_de_emprestimo(this: usize, contrato: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e realoca o vetor: só a original; a cópia prova no modo Rust
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1009)(this, contrato)
}

/// { 0x4d07e0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::end_loan(pessoa)`: o clube do empréstimo
/// (+0x39; nulo: caixa da linha 0x1729), ou o time A dele quando é o B (0x540a50). Com o contrato principal no próprio
/// vetor (sem ele: caixa da linha 0x1735): a ficha [0xae1080] dá o clube dono; tira a pessoa das listas (0x7c5fe0 com o
/// dono), das disputas (0x556570), das táticas do clube do empréstimo (0x896c60 com 0 e 1); libera o contrato de
/// empréstimo (−1 na entrada e na vaga +4); tira do elenco do clube do empréstimo (0x53c190); refaz as folhas do clube
/// do empréstimo e do dono; tira da lista de observação do clube do empréstimo (0x849ce0, 0x843e10 com 4). A pessoa
/// volta ao dono (+0x39) e às marcas de elenco (0x540980): com time reserva (0x542740), 2 se o contrato principal tem o
/// bit 0x40 de +0x35, senão 1; senão 1 e, com clube parceiro, fica no parceiro se está nas 50 vagas dele. A situação
/// +0x6d 0x10 vira 4.
unsafe extern "thiscall" fn encerrar_o_emprestimo(this: usize, pessoa: usize) {
    let mut clube = read_u32(pessoa + 0x39) as usize;
    if clube == 0 {
        cpp2_erro(0x1729);
        return;
    }
    let mut eh_a = 0u32; // só lida quando a 0x540a50 a grava
    if cpp2_parceiro(clube, &mut eh_a) != 0 && eh_a == 0 {
        clube = cpp2_parceiro(clube, core::ptr::null_mut()); // o time A
    }
    let principal = if pessoa != 0 && (read_u32(pessoa) as i32) < read_u32(0xae2364) as i32 && (read_u32(cpp2_vaga(pessoa)) as i32) >= 0 {
        cpp2_contrato(read_u32(this), read_u32(cpp2_vaga(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        cpp2_erro(0x1735);
        return;
    }
    crate::comparacao::ramo("cpp_2 encerrar emprestimo");
    let ficha = cpp2_contrato(read_u32(read_u32(0xae1050) as usize), read_u32(cpp2_vaga(pessoa)) as i32);
    cpp2_por(0xae1080, ficha as u32);
    let dono = if ficha == 0 { 0 } else { cpp2_clube(read_u32(ficha + 4)) };
    cpp2_listas(pessoa, 0, read_u32(dono) as i32);
    let disputas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_RESTORE_HELD_BANS);
    disputas(read_u32(0xae28f0) as usize, pessoa);
    cpp2_tatica(read_u32(pessoa + 0x39) as usize, pessoa, 0, 1);
    cpp2_tatica(read_u32(pessoa + 0x39) as usize, pessoa, 1, 1);
    let emp = read_u32(cpp2_vaga(pessoa) + 4) as i32;
    if emp != -1 {
        cpp2_por(cpp2_contrato(read_u32(this), emp), u32::MAX);
        cpp2_por(cpp2_vaga(pessoa) + 4, u32::MAX);
    }
    cpp2_tirar(read_u32(pessoa + 0x39) as usize, pessoa, 0, 1, 0);
    cpp2_folha(clube);
    let f = cpp2_ficha(pessoa);
    cpp2_folha(if f == 0 { 0 } else { cpp2_clube(read_u32(f + 4)) });
    let lista = read_u32(read_u32(0xde1f54) as usize + 0x856) as usize;
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    if entrada(lista, clube, pessoa, 0, 0) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        tirar(read_u32(read_u32(0xde1f54) as usize + 0x856) as usize, pessoa, clube, 4, 0);
    }
    let f = cpp2_ficha(pessoa);
    let volta = if f == 0 { 0 } else { cpp2_clube(read_u32(f + 4)) };
    cpp2_por(pessoa + 0x39, volta as u32); // a pessoa volta ao dono
    if cpp2_reservas(volta) != 0 {
        cpp2_marcar(pessoa, if cpp2_b(principal + 0x35) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let parceiro = cpp2_parceiro(read_u32(pessoa + 0x39) as usize, core::ptr::null_mut());
        cpp2_marcar(pessoa, 1);
        if parceiro != 0 {
            for k in 0..0x32 {
                if read_u32(parceiro + 0xd7 + 4 * k) as usize == pessoa {
                    cpp2_por(pessoa + 0x39, parceiro as u32);
                }
            }
        }
    }
    if cpp2_b(pessoa + 0x6d) == 0x10 {
        *((pessoa + 0x6d) as *mut u8) = 4;
    }
}

unsafe extern "thiscall" fn comparar_encerrar_o_emprestimo(this: usize, pessoa: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e nas disputas (0x556570 libera itens): só a original
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1010)(this, pessoa)
}

/// { 0x4cbd20 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::toggle_squad_placement(pessoa, clube,
/// saida)`: pelo lugar (0x4cbb60): 0 nada. 1 e 2 (principal / reserva no clube): emprestada a outro clube (contrato de
/// empréstimo +4 com outro clube), só apaga (1) ou marca (2) o bit 0x40 de +0x35 do contrato principal; senão a marca de
/// elenco 1 ou 2 (0x540980) e as táticas do clube (0x896c60 com 1 ou 0). 3 (time B) e os outros (time A): o clube de
/// destino é o clube (3) ou o parceiro (0x540a50), gravado em `saida`; sem vaga para o cargo (+0x3d, 0x53b9a0) devolve
/// 0. Emprestada a este clube: sai das listas, do elenco e das táticas do outro e entra no destino com o cargo; com
/// contrato de empréstimo de outro clube: troca o bit 0x40 e passa ao destino como jogador (0xb); senão como o
/// emprestado, sem gravar. Devolve 1 (clube nulo também). Sem contrato principal a original lê [0x35] e cai.
unsafe extern "thiscall" fn trocar_lugar_no_elenco(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    if clube == 0 {
        return 1;
    }
    if saida != 0 {
        cpp2_por(saida, 0);
    }
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let l = lugar(this, pessoa, clube);
    if l == 0 {
        return 1;
    }
    let na_faixa = |p: usize| p != 0 && (read_u32(p) as i32) < read_u32(0xae2364) as i32;
    // o contrato de empréstimo (no CONTRACT_MANAGER global) e o principal (no próprio vetor; 0 sem)
    let emprestimo = |p: usize| {
        let k = read_u32(cpp2_vaga(p) + 4) as i32;
        if k < 0 { 0 } else { cpp2_contrato(read_u32(read_u32(0xae1050) as usize), k) }
    };
    let principal = |p: usize| {
        let k = read_u32(cpp2_vaga(p)) as i32;
        if k < 0 { 0 } else { cpp2_contrato(read_u32(this), k) }
    };
    if l == 1 || l == 2 {
        if na_faixa(pessoa) {
            let e = emprestimo(pessoa);
            if e != 0 && read_u32(e + 4) != read_u32(clube) {
                let m = principal(pessoa) + 0x35;
                let cl = cpp2_b(m);
                *(m as *mut u8) = if l == 1 { cl & 0xbf } else { cl | 0x40 };
                return 1;
            }
        }
        crate::comparacao::ramo("cpp_2 trocar no clube");
        cpp2_marcar(pessoa, l as u32); // 1 principal, 2 reserva
        cpp2_tatica(clube, pessoa, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (cpp2_parceiro(clube, core::ptr::null_mut()), clube)
    } else {
        (clube, cpp2_parceiro(clube, core::ptr::null_mut()))
    };
    if saida != 0 {
        cpp2_por(saida, destino as u32);
    }
    let vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_ROOM_FOR_JOB);
    // ponytail: a original empurra o cargo num registrador com lixo nos bytes altos; a 0x53b9a0 lê só o byte
    if vaga(destino, cpp2_b(pessoa + 0x3d) as u32, pessoa) == 0 {
        return 0;
    }
    if na_faixa(pessoa) {
        let e = emprestimo(pessoa);
        if e != 0 && read_u32(e + 4) == read_u32(clube) {
            // emprestada a este clube: passa do outro ao destino com o cargo
            cpp2_listas(pessoa, 0, -1);
            cpp2_tirar(origem, pessoa, 0, 1, 1);
            cpp2_tatica(origem, pessoa, 1, 1);
            cpp2_tatica(origem, pessoa, 0, 1);
            // ponytail: o cargo vai com os bytes altos do eax da 0x896c60; a 0x53bd10 lê só o byte
            cpp2_por_no_clube(destino, pessoa, cpp2_b(pessoa + 0x3d) as u32, 0, 1, 1);
            return 1;
        }
        if na_faixa(pessoa) && e != 0 {
            let m = principal(pessoa) + 0x35;
            let cl = cpp2_b(m);
            *(m as *mut u8) = if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf };
            crate::comparacao::ramo("cpp_2 trocar emprestado");
            cpp2_tirar(origem, pessoa, 0, 0, 1);
            cpp2_por_no_clube(destino, pessoa, 0xb, 0, 0, 1);
            return 1;
        }
    }
    crate::comparacao::ramo("cpp_2 trocar de time");
    cpp2_listas(pessoa, 0, -1);
    cpp2_tirar(origem, pessoa, 0, 1, 0);
    cpp2_tatica(origem, pessoa, 1, 1);
    cpp2_tatica(origem, pessoa, 0, 1);
    // ponytail: o cargo vai com os bytes altos do edx deixados pela 0x896c60; a 0x53bd10 lê só o byte
    cpp2_por_no_clube(destino, pessoa, cpp2_b(pessoa + 0x3d) as u32, 0, 1, 0);
    1
}

unsafe extern "thiscall" fn comparar_trocar_lugar_no_elenco(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    // ponytail: mexe no elenco, nas táticas e nas listas: só a original; a cópia prova no modo Rust
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1011)(this, pessoa, clube, saida)
}

// ---------- 252a leva: quem serve para o cargo e a moral depois da partida ruim: o candidato a técnico, o cargo que a pessoa pode assumir e baixar a moral pela partida (manager_manager.cpp, contract_manager.cpp) ----------



unsafe fn cpp4_b(a: usize) -> u8 {
    // ponytail: volátil porque a original lê [0 + 0x53] quando falta o clube do contrato (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}

/// A reputação "vista" de um clube para o técnico: com nação (+0x53), o peso dela (+0x85) × reputação (+0x80) × 0,05
/// (0x967888); sem, a reputação × 0,65 (0x96a5d0); truncadas (16 bits) e, acima de 0x203a, 0x203a − (excesso × −1,1
/// truncado).
unsafe fn cpp4_reputacao_vista(clube: usize) -> i16 {
    let r = read_i16(clube + 0x80) as i32;
    let nacao = read_u32(clube + 0x53) as usize;
    let v = if nacao != 0 {
        ftol((cpp4_b(nacao + 0x85) as i8 as i32 * r) as f64 * f64_em(0x967888))
    } else {
        ftol(r as f64 * f64_em(0x96a5d0))
    } as i16;
    if v > 0x203a {
        (0x203a - ftol((v as i32 - 0x203a) as f64 * f64_em(0x96d278))) as i16
    } else {
        v
    }
}

/// { 0x69f810 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::is_staff_suitable_for_job(pessoa, clube,
/// nota_minima)`: sem ficha de não jogador (+0x69) ou sem clube: 0. Os vetos (0): reputação de outra nação (+0xc) acima
/// de 4250 de outra nação com peso (+0x85) abaixo de 10; doméstica (+8) acima de 6250, técnico (+0x3d = 5) de outra
/// nação e clube abaixo de 5750; seleção que já é uma das três dele (+0x65 +0x10..+0x18); jogador (0xb) de nota
/// (+0x61 +0xb) acima de 4750 num clube abaixo de 6750, com contrato (0x4cf5a0) de faixa 1..3 (+0x4f >> 4), menos de
/// 180 dias, +0x3f acima de 1, +0x3e zero, menor de 35 com +0x3d positivo, ou de outra nação em clube abaixo de 7250
/// que não é o dele; técnico de clube numa divisão melhor (+0x57 +0x69), ou mais forte, ou recente (dias desde +0x3e
/// abaixo de (+0x59 + 21) × 10) num clube acima de 5250; técnico de seleção (+0x24, +0x28 = 5); clube abaixo de 2250
/// (com reputação +0xa acima de 4750 ou sem ficha); +0x22 acima de 75 num clube abaixo de 3250; não jogador com a
/// aposentadoria pendente (0x7ae350). Os sim (1): clube abaixo de 2500 da nação dele, doméstica abaixo de 1500, sem
/// clube nem seleção; seleção da nação dele com reputação abaixo de 6750, sem clube, ou com o clube dele mais fraco
/// (abaixo de 7750 e 750 abaixo da seleção) há tempo; clube acima de 7750 com a reputação vista acima da do clube dele
/// + 25 × +0x59 + 250. Senão, o interesse dele no cargo (0x844780, float) não abaixo de `nota_minima`.
unsafe extern "thiscall" fn serve_para_o_cargo(_this: usize, pessoa: usize, clube: usize, nota_minima: f64) -> u8 {
    let ficha = read_u32(pessoa + 0x69) as usize;
    if ficha == 0 || clube == 0 {
        return 0;
    }
    let rep = |c: usize| read_i16(c + 0x80);
    if read_i16(ficha + 0xc) > 0x109a {
        let (n1, n2) = (read_u32(pessoa + 0x1a) as usize, read_u32(clube + 0x53) as usize);
        if n1 != 0 && n2 != 0 && n1 != n2 && (cpp4_b(n2 + 0x85) as i8) < 0xa {
            return 0;
        }
    }
    if read_i16(ficha + 8) > 0x186a && read_u32(pessoa + 0x1a) != read_u32(clube + 0x53) && cpp4_b(pessoa + 0x3d) == 5 && rep(clube) < 0x1676 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        let s = read_u32(pessoa + 0x65) as usize;
        if s != 0 && (read_u32(s + 0x10) as usize == clube || read_u32(s + 0x14) as usize == clube || read_u32(s + 0x18) as usize == clube) {
            return 0;
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let tempo = |p: usize| dias(0xae2c90, read_u32(p + 0x3e), read_u32(p + 0x42));
    let limite = |p: usize| (cpp4_b(p + 0x59) as i8 as i32 + 0x15) * 10;
    if cpp4_b(pessoa + 0x3d) == 0xb {
        let jogador = read_u32(pessoa + 0x61) as usize;
        if jogador != 0 && read_i16(jogador + 0xb) > 0x128e && rep(clube) < 0x1a5e {
            let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
            let c = contrato_de(read_u32(0xae1050) as usize, pessoa);
            if c != 0 {
                let faixa = cpp4_b(c + 0x4f) >> 4;
                if faixa != 0 && faixa <= 3 {
                    return 0;
                }
                if dias(0xae2c90, read_u32(c + 0x25), read_u32(c + 0x29)) < 0xb4 {
                    return 0;
                }
                if cpp4_b(c + 0x3f) as i8 > 1 || cpp4_b(c + 0x3e) == 0 {
                    return 0;
                }
                if (cpp4_b(pessoa + 0x18) as i8) < 0x23 && cpp4_b(c + 0x3d) as i8 > 0 {
                    return 0;
                }
                if read_u32(pessoa + 0x1a) != read_u32(clube + 0x53) && rep(clube) < 0x1c52 && read_u32(pessoa + 0x39) as usize != clube {
                    return 0;
                }
            }
        }
    }
    if cpp4_b(pessoa + 0x3d) == 5 {
        let liga = read_u32(clube + 0x57) as usize;
        let atual = read_u32(pessoa + 0x39) as usize;
        if liga != 0 && atual != 0 && read_u32(atual + 0x57) != 0 {
            let liga2 = read_u32(atual + 0x57) as usize;
            if read_i16(liga + 0x69) < read_i16(liga2 + 0x69) {
                return 0; // o clube dele está numa divisão melhor
            }
            if liga == liga2 && (rep(clube) as i32 - 0x4e2) < rep(atual) as i32 {
                return 0;
            }
            if rep(atual) > rep(clube) {
                return 0;
            }
            if tempo(pessoa) < limite(pessoa) && rep(read_u32(pessoa + 0x39) as usize) > 0x1482 {
                return 0;
            }
        }
    }
    if read_u32(pessoa + 0x24) != 0 && cpp4_b(pessoa + 0x28) == 5 {
        return 0;
    }
    let ficha = read_u32(pessoa + 0x69) as usize;
    if (ficha == 0 || read_i16(ficha + 0xa) > 0x128e) && rep(clube) < 0x8ca {
        return 0;
    }
    if cpp4_b(pessoa + 0x22) > 0x4b && rep(clube) < 0xcb2 {
        return 0;
    }
    if read_u32(pessoa + 0x61) == 0 {
        let pendente: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_IS_RETIREMENT_PENDING);
        if pendente(read_u32(0xb7434c) as usize, pessoa) != 0 {
            return 0;
        }
    }
    if rep(clube) < 0x9c4 && read_u32(pessoa + 0x1a) == read_u32(clube + 0x53) && read_i16(read_u32(pessoa + 0x69) as usize + 8) < 0x5dc
        && read_u32(pessoa + 0x39) == 0 && read_u32(pessoa + 0x24) == 0 {
        return 1;
    }
    if selecao(clube) != 0 && read_u32(pessoa + 0x1a) == read_u32(clube + 0x53) {
        if read_i16(read_u32(pessoa + 0x69) as usize + 0xa) < 0x1a5e {
            return 1;
        }
        if read_u32(pessoa + 0x39) == 0 {
            return 1;
        }
        if tempo(pessoa) > limite(pessoa) {
            let r = rep(read_u32(pessoa + 0x39) as usize);
            if r < 0x1e46 && (r as i32 + 0x2ee) < rep(clube) as i32 {
                return 1;
            }
        }
    }
    if rep(clube) > 0x1e46 {
        let vista = cpp4_reputacao_vista(clube);
        let atual = read_u32(pessoa + 0x39) as usize;
        if atual != 0 {
            let dele = cpp4_reputacao_vista(atual);
            let teto = (cpp4_b(pessoa + 0x59) as i8 as i16).wrapping_mul(25) as i32 + dele as i32 + 0xfa;
            if vista as i32 > teto {
                return 1;
            }
        }
    }
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> f64 = core::mem::transmute(FN_STAFF_INTEREST_IN_JOB_AT_CLUB);
    let nota = interesse(read_u32(read_u32(0xde1f54) as usize + 0x856) as usize, pessoa, clube, 0, 5, 0);
    if nota >= nota_minima { 1 } else { 0 } // fcomp: abaixo (ou sem ordem) é 0
}

/// Comparador: a rotina só lê (o interesse 0x844780 sem a saída das razões): pela devolução.
unsafe extern "thiscall" fn comparar_serve_para_o_cargo(this: usize, pessoa: usize, clube: usize, nota_minima: f64) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8>(1015)(this, pessoa, clube, nota_minima);
    let n = serve_para_o_cargo(this, pessoa, clube, nota_minima);
    PLACAR.registrar(r == n, || format!("serve_para_o_cargo(pessoa {:x}, clube {:x}, {}) original={} rust={}", pessoa, clube, nota_minima, r, n));
    r
}

/// { 0x4cb990 } (stdcall, ret 8, contract_manager.cpp): `staff_can_take_job(cargo, pessoa)`: pela tabela de salto
/// 0x4cbaf4 (cargo − 6, 0..9; fora: 0); 7, 12 e 14: 0; pessoa nula: 0 (o regen de não jogador 0x7ab9f2 pergunta sempre
/// com nula). 6: com 30 anos (+0x18) ou mais, não jogador (+0x69), acima de 36, ou com +0x57 abaixo de 10 e +0x2f do
/// jogador 12 ou mais: 1; senão, não jogador no cargo 8, 6 ou 5. 8: de 30 anos, não jogador ou acima de 36 ou (+0x2f ≥ 12
/// e +0x57 < 10) fora dos cargos 5 e 10: 1; senão como o 6. 9: não jogador no cargo 9. 10: não jogador no cargo 10 ou com
/// +0x28 = 10. 11: jogador (+0x61). 13 e 15: jogador e não jogador, ou acima de 34, ou de 30 anos com +0x2f ≥ 12.
unsafe extern "stdcall" fn pode_assumir_o_cargo(cargo: u32, pessoa: usize) -> u32 {
    let k = (cargo as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if pessoa == 0 {
        crate::comparacao::ramo("cpp_4 pessoa nula");
        return 0; // pessoa nula
    }
    let p = pessoa;
    let idade = cpp4_b(p + 0x18) as i8;
    let nao_jogador = read_u32(p + 0x69);
    let funcao = cpp4_b(p + 0x3d);
    let forca = || cpp4_b(read_u32(p + 0x61) as usize + 0x2f) as i8; // +0x2f do jogador
    // o fim comum (0x4cb9db): não jogador nos cargos 8, 6 ou 5
    let comum = || nao_jogador != 0 && (funcao == 8 || funcao == 6 || funcao == 5);
    let r = match k {
        0 => {
            if idade >= 0x1e {
                if nao_jogador != 0 || idade > 0x24 {
                    return 1;
                }
                if (cpp4_b(p + 0x57) as i8) < 0xa && forca() >= 0xc {
                    return 1;
                }
            }
            comum()
        }
        2 => {
            if idade < 0x1e {
                comum()
            } else if nao_jogador == 0 && idade <= 0x24 && (forca() < 0xc || cpp4_b(p + 0x57) as i8 >= 0xa) {
                comum()
            } else if funcao == 5 || funcao == 0xa {
                comum()
            } else {
                true
            }
        }
        3 => nao_jogador != 0 && funcao == 9,
        4 => nao_jogador != 0 && (funcao == 0xa || cpp4_b(p + 0x28) == 0xa),
        5 => read_u32(p + 0x61) != 0,
        _ => {
            // 13 e 15
            let jogador = read_u32(p + 0x61) as usize;
            jogador != 0 && (nao_jogador != 0 || idade > 0x22 || (idade >= 0x1e && cpp4_b(jogador + 0x2f) as i8 >= 0xc))
        }
    };
    r as u32
}

unsafe extern "stdcall" fn comparar_pode_assumir_o_cargo(cargo: u32, pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(u32, usize) -> u32>(1016)(cargo, pessoa);
    let n = pode_assumir_o_cargo(cargo, pessoa);
    PLACAR.registrar(r == n, || format!("pode_assumir_o_cargo(cargo {}, pessoa {:x}) original={} rust={}", cargo as u8 as i8, pessoa, r, n));
    r
}

/// Os dias (0..400) desde a data (dia +0, ano +2 sem sinal) até hoje ([0xae2c90]), com o ano de 365 dias.
unsafe fn cpp4_dias_desde(dia: usize, ano: usize) -> i32 {
    let hoje = core::ptr::read_unaligned(0xae2c92 as *const u16) as i32;
    let a = core::ptr::read_unaligned(ano as *const u16) as i32;
    let d = (hoje - a) * 365 - read_i16(dia) as i32 + read_i16(0xae2c90) as i32;
    d.clamp(0, 400)
}

/// A queda da moral (o byte `m` da original) pela partida ruim, ou None quando a rotina sai sem mexer (algum nulo, o
/// técnico ou o auxiliar do clube +0xcf/+0xd3, ou fora do cargo 0xb). x = (nota +0xb − reputação do clube) + 50 × dias
/// no clube (desde +0x3e, até 400) + 417 × idade − 31680; com os dias do contrato (desde +0x2d) acima de 400 (nunca: o
/// teto é 400) e idade 24..31: + 250 × +0x57 + 5000; senão, com +0x22 e a reputação da nação (+0x1a +0x8e) acima de 3500
/// e menos de 32 anos: + 250 × +0x57 + 25 × +0x22 + 2500; senão pela faixa do contrato (+0x4f >> 4): 1: + 250 × +0x57 +
/// 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 × +0x3e; nesses casos no máximo 15000. y = 1 − x × (6,667e-4 se x < 0, senão
/// 6e-5), entre 0,1 e 10; m = ftol((+0x3e + 4) / y + 0,5) (byte). Dupla precisão; as constantes do .rdata.
unsafe fn cpp4_queda_da_moral(pessoa: usize, clube: usize, contrato: usize) -> Option<i8> {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return None;
    }
    if pessoa == read_u32(clube + 0xcf) as usize || pessoa == read_u32(clube + 0xd3) as usize || cpp4_b(pessoa + 0x3d) != 0xb {
        return None;
    }
    let a = cpp4_dias_desde(pessoa + 0x3e, pessoa + 0x40);
    let b = cpp4_dias_desde(contrato + 0x2d, contrato + 0x2f);
    let idade = cpp4_b(pessoa + 0x18) as i8;
    let jogador = read_u32(pessoa + 0x61) as usize;
    let dif = read_i16(jogador + 0xb) as i32 - read_i16(clube + 0x80) as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + idade as f64 * f64_em(0x968d90)) - f64_em(0x968db8);
    let c57 = cpp4_b(pessoa + 0x57) as i8 as i32;
    let teto = f32_em(0x968da8) as f64;
    let limitar = |x: f64| if x >= teto { teto } else { x };
    let al = cpp4_b(pessoa + 0x22);
    if b > 0x190 && idade > 0x17 && idade < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + f32_em(0x968db0) as f64); // nunca: b vai até 400
    } else if al != 0 && read_i16(read_u32(pessoa + 0x1a) as usize + 0x8e) > 0xdac && idade < 0x20 {
        x = limitar(x + (c57 * 250 + al as i32 * 25) as f64 + f32_em(0x968dac) as f64);
    } else {
        match (cpp4_b(contrato + 0x4f) >> 4) as i8 {
            1 => x = limitar(x + (c57 * 250) as f64 + f32_em(0x968db0) as f64),
            2 => x = limitar(x + (c57 * 125) as f64 + f32_em(0x968dac) as f64),
            3 => x = limitar(x + (cpp4_b(contrato + 0x3e) as i8 as i32 * 1000) as f64),
            _ => {}
        }
    }
    let y = if !(x >= f64_em(0x9689a0)) { x * f64_em(0x968da0) } else { x * f64_em(0x968d98) };
    let mut y = f64_em(0x967890) - y;
    if !(y >= f64_em(0x967880)) {
        y = f64_em(0x967880);
    } else if y > f64_em(0x9688b0) {
        y = f64_em(0x9688b0);
    }
    let d = (cpp4_b(contrato + 0x3e) as i8 as i32 + 4) as f64;
    Some(ftol(d / y + f64_em(0x968968)) as u8 as i8)
}

/// A moral (+0x44) menos `m`, entre −100 e 100 (a soma em byte quando cabe).
fn cpp4_moral_nova(moral: i8, m: i8) -> i8 {
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

/// O contrato principal da pessoa no CONTRACT_MANAGER global e o clube dele, como a original os refaz (a ficha
/// [0xae1080] gravada); sem contrato o clube é nulo.
unsafe fn cpp4_clube_do_contrato(pessoa: usize) -> usize {
    let c = if read_u32(pessoa) as i32 >= read_u32(0xae2364) as i32 {
        0
    } else {
        let k = read_u32((read_u32(0xae2d0c) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(0x4f))) as i32;
        if k < 0 { 0 } else { (read_u32(read_u32(0xae1050) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) }
    };
    core::ptr::write_unaligned(0xae1080 as *mut u32, c as u32);
    if c == 0 { 0 } else { (read_u32(0xae23b4) as usize).wrapping_add((read_u32(c + 4) as usize).wrapping_mul(0x245)) }
}

/// A janela de transferências aberta para o clube do contrato da pessoa: o índice da nação-chave (0x669340 da nação do
/// clube; sem nação, o byte baixo do ponteiro do clube que a original lê da vaga do argumento) dá as duas datas da
/// tabela 0xb63d8a (72 bytes por nação): hoje depois da primeira e antes da segunda.
unsafe fn cpp4_janela_aberta(pessoa: usize, clube: usize) -> bool {
    let c1 = cpp4_clube_do_contrato(pessoa);
    let k = if read_u32(c1 + 0x53) != 0 {
        let c2 = cpp4_clube_do_contrato(pessoa);
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
        indice(read_u32(c2 + 0x53) as usize) as u8 as i8
    } else {
        // ponytail: a original lê o byte baixo da vaga do argumento `clube` (o ponteiro), não um índice
        clube as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = 0xb63d8ausize + (k as usize) * 0x48;
    let dia = read_i16(0xae2c90);
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    let ano_de = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let depois = (dia > read_i16(t) && ano == ano_de(t + 2)) || ano > ano_de(t + 2);
    let antes = (dia < read_i16(t + 0xf) && ano == ano_de(t + 0x11)) || ano < ano_de(t + 0x11);
    depois && antes
}

/// { 0x4cb250 } (thiscall, ret 0xc, contract_manager.cpp): `contract_manager_lower_match_morale(pessoa, clube,
/// contrato)`: a queda `m` (cpp4_queda_da_moral). Machucado (0x62c400), emprestado (contrato de empréstimo), aposentando
/// (0x7a7eb0), menor de 18 ou com +0x49: apaga as queixas 4 e 0x10 (0x8674e0). Senão, com m > 5 e a moral − m abaixo de
/// −30 − +0x59: acima de 21 anos, mais forte (+0xb) que o clube e acima de 2250, ou de potencial (+0xd) acima da
/// reputação − 1500 e de 500 com sorteio(+0x57) ≠ 0 e +0x3d negativo: na janela de transferências do clube do contrato,
/// a queixa 4 (0x8674e0 e 0x867700 com 4); no principal (0x4cbb60 = 1), nota acima de 1250, mais de 23 anos e
/// sorteio(+0x59 + 5) = 0: a queixa 0x10; até 21 anos, no principal com sorteio(+0x57) > 5, mais forte que o clube e
/// acima de 1250: a queixa 0x10. Por fim a felicidade no contrato (+0x44) cai m, entre −100 e 100.
unsafe extern "thiscall" fn baixar_moral_pela_partida(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let Some(m) = cpp4_queda_da_moral(pessoa, clube, contrato) else {
        return;
    };
    let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let aposentando: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let emprestado = || {
        if read_u32(pessoa) as i32 >= read_u32(0xae2364) as i32 {
            return false;
        }
        let k = read_u32((read_u32(0xae2d0c) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(0x4f)) + 4) as i32;
        k >= 0 && (read_u32(read_u32(0xae1050) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let idade = cpp4_b(pessoa + 0x18) as i8;
    if lesao(read_u32(0xb63d28) as usize, pessoa) != 0 || emprestado() || aposentando(read_u32(0xb7434c) as usize, pessoa) != 0
        || idade < 0x12 || cpp4_b(contrato + 0x49) != 0 {
        if read_u32(contrato + 0x45) & 4 != 0 {
            apagar(contrato, 4);
        }
        if cpp4_b(contrato + 0x45) & 0x10 != 0 {
            apagar(contrato, 0x10);
        }
    } else if m > 5 && (cpp4_b(contrato + 0x44) as i8 as i32 - m as i32) < -30 - cpp4_b(pessoa + 0x59) as i8 as i32 {
        let jogador = read_u32(pessoa + 0x61) as usize;
        if idade > 0x15 {
            let nota = read_i16(jogador + 0xb);
            if nota > read_i16(clube + 0x80) && nota > 0x8ca {
                if cpp4_janela_aberta(pessoa, clube) {
                    crate::comparacao::ramo("cpp_4 queixa na janela");
                    apagar(contrato, 4);
                    marcar(contrato, 4, 0);
                }
            } else {
                let mut feito = false;
                if lugar(this, pessoa, clube) == 1 && read_i16(jogador + 0xb) > 0x4e2 && idade > 0x17
                    && sortear(cpp4_b(pessoa + 0x59) as i8 as i32 + 5) == 0 {
                    apagar(contrato, 4);
                    marcar(contrato, 0x10, 0);
                    feito = true;
                }
                if !feito {
                    let pot = read_i16(jogador + 0xd);
                    if (pot as i32) > read_i16(clube + 0x80) as i32 - 0x5dc && pot > 0x1f4
                        && sortear(cpp4_b(pessoa + 0x57) as i8 as i32) != 0 && (cpp4_b(contrato + 0x3d) as i8) < 0
                        && cpp4_janela_aberta(pessoa, clube) {
                        apagar(contrato, 4);
                        marcar(contrato, 4, 0);
                    }
                }
            }
        } else if lugar(this, pessoa, clube) == 1 && sortear(cpp4_b(pessoa + 0x57) as i8 as i32) > 5
            && read_i16(jogador + 0xb) > read_i16(clube + 0x80) && read_i16(jogador + 0xb) > 0x4e2 {
            apagar(contrato, 4);
            marcar(contrato, 0x10, 0);
        }
    }
    let p = (contrato + 0x44) as *mut i8;
    *p = cpp4_moral_nova(*p, m); // a moral cai
}

/// Comparador: as queixas (0x867700) sorteiam e podem abrir pedido de transferência: só a original; o plano (a queda
/// `m`, calculada antes, sem efeito) diz a moral +0x44 que ela deixa (as queixas não a gravam) ou que ela não mexe.
unsafe extern "thiscall" fn comparar_baixar_moral_pela_partida(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1017);
    if contrato == 0 {
        return o(this, pessoa, clube, contrato);
    }
    let antes = cpp4_b(contrato + 0x44) as i8;
    let plano = cpp4_queda_da_moral(pessoa, clube, contrato);
    // apagar uma queixa (0x8674e0) recalcula o contrato (0x867240), que pode regravar a moral: com queixa marcada antes
    // ou queda acima de 5 (os ramos que apagam sempre), só a original
    if cpp4_b(contrato + 0x45) & 0x14 != 0 || plano.is_some_and(|m| m > 5) {
        return o(this, pessoa, clube, contrato);
    }
    o(this, pessoa, clube, contrato);
    let depois = cpp4_b(contrato + 0x44) as i8;
    let esperada = plano.map_or(antes, |m| cpp4_moral_nova(antes, m));
    PLACAR.registrar(depois == esperada, || format!("baixar_moral_pela_partida(queda {:?}) moral {} -> original {} plano {}", plano, antes, depois, esperada));
}

// ---------- 251a leva: os contratos: tirar o contrato principal, reenviar as ofertas guardadas e o mês do fim da temporada do clube (contract_manager.cpp) ----------


unsafe fn cpo2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A vaga do contrato principal da pessoa: o registro de 79 bytes dela em [0xae2d0c] (+0 o índice do contrato, −1 sem).
unsafe fn cpo2_vaga(pessoa: usize) -> usize {
    read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f))
}

/// { 0x4d0b50 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract(pessoa)`: com o contrato
/// principal (índice no registro de 79 bytes da pessoa), libera a entrada dele (+0 = −1 no vetor de 0x50 bytes do
/// CONTRACT_MANAGER), põe −1 na vaga da pessoa e refaz a folha de salários (0x598370) das finanças (0x5a0590) do clube
/// do contrato (+4). Sem contrato: nada.
unsafe extern "thiscall" fn tirar_o_contrato_principal(this: usize, pessoa: usize) {
    let indice = cpo2_i(cpo2_vaga(pessoa));
    if indice == -1 {
        return;
    }
    let e = read_usize(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let id_clube = read_usize(e + 4);
    cpo2_por(e, usize::MAX);
    let clube = read_usize(0xae23b4).wrapping_add(id_clube.wrapping_mul(0x245));
    crate::comparacao::ramo("cpo_2 tirar contrato");
    cpo2_por(cpo2_vaga(pessoa), usize::MAX); // a pessoa fica sem contrato
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let f = financas_do_clube(read_usize(0xae2a38), clube);
    if f != 0 {
        let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FINANCE_UPDATE_WAGE_BILL);
        folha(f);
    }
}

/// { 0x4cf4a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::resubmit_held_offers(id)`: percorre a lista
/// das ofertas guardadas (+0x38) e, para cada uma da pessoa `id` (+0): com a transferência (+0x59, 0x8c61e0 em
/// [0xde1f54]) cujo clube (+0x24) é o da oferta (+0x5d), põe o estado 0xf em 8 e reenvia a oferta (0x4cc490 com a
/// transferência, o clube, o byte +0x61, 1, 1, 0); sem transferência (+0x59 = −1) reenvia com −1, −1. Tira a oferta da
/// lista (0x68d1f0) e conta. O byte +0x61 vai empurrado no registrador inteiro (os bits de cima do que estava nele:
/// o ponteiro lido ou −1). Devolve quantas.
unsafe extern "thiscall" fn reenviar_ofertas_guardadas(this: usize, id: usize) -> u32 {
    let lista = read_usize(this + 0x38);
    if lista == 0 {
        return 0;
    }
    cpo2_por(lista + 0xa, read_usize(lista + 2));
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
                let p = achar(read_usize(0xde1f54), t);
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

/// { 0x4d45f0 } (stdcall, ret 4, contract_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `contract_manager_season_end_month(clube)`: a data do fim da temporada do clube (Data() 0x54a110 e 0x669bd0 com 0),
/// mais 10 dias (0x54c5b0), e o mês dela (0x54cfb0). Devolve o eax da 0x54cfb0 (o mês 0..11).
unsafe extern "stdcall" fn mes_do_fim_da_temporada_do_clube(clube: usize) -> u32 {
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

/// Comparador: lado a lado pela vaga da pessoa e pela entrada do contrato (a folha 0x598370 refeita duas vezes dá o
/// mesmo, pelo contrato já tirado).
unsafe extern "thiscall" fn comparar_tirar_o_contrato_principal(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(970);
    let vaga = cpo2_vaga(pessoa);
    let indice = cpo2_i(vaga);
    if indice == -1 {
        return o(this, pessoa);
    }
    let e = read_usize(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let reg = [(vaga, 4), (e, 8)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || tirar_o_contrato_principal(this, pessoa));
    PLACAR.registrar(igual, || format!("tirar_o_contrato_principal(pessoa {:x}, contrato {}) {:x?} x {:x?}", read_usize(pessoa), indice, mo, mr));
}

/// Comparador: a original reenvia (0x4cc490, ofertas e transferências) e tira as ofertas da lista (libera): roda só ela
/// e confere o plano feito antes: devolve quantas ofertas da pessoa havia na lista.
unsafe extern "thiscall" fn comparar_reenviar_ofertas_guardadas(this: usize, id: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(971);
    let lista = read_usize(this + 0x38);
    if lista == 0 {
        return o(this, id);
    }
    // o plano: a original recomeça o cursor (+0xa) antes de percorrer
    cpo2_por(lista + 0xa, read_usize(lista + 2));
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
    PLACAR.registrar(r == esperado, || format!("reenviar_ofertas_guardadas(id {:x}) devolveu {}, esperava {}", id, r, esperado));
    r
}

/// Comparador: pelo valor (datas e a 0x669bd0 só calculam).
unsafe extern "stdcall" fn comparar_mes_do_fim_da_temporada_do_clube(clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(972)(clube);
    let n = mes_do_fim_da_temporada_do_clube(clube);
    PLACAR.registrar(r == n, || format!("mes_do_fim_da_temporada_do_clube({:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 248a leva: os contratos da comissão e as datas de decisão: achar a vaga de contrato, dar o contrato à pessoa nova e marcar a data da decisão (contract_manager.cpp) ----------


unsafe fn cpl4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A caixa "cannot create space" do CONTRACT_MANAGER: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de
/// contract_manager.cpp 0x9a8190, `linha`, `texto`) em 0xae24d0; o título "Memory Allocation Error" (0x9a8108)
/// traduzido (0x66f4e0) em 0xae28bc, o corpo 0x9a7ff8 com o texto em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está).
unsafe fn cpl4_sem_espaco(linha: u32, texto: usize) {
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

/// { 0x4c79a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::get_free_staff_contract_space(so_no_fim)`:
/// a vaga do vetor de contratos (registros de 0x50 bytes em [this], capacidade +4, conta +8). Sem `so_no_fim`, a
/// primeira vaga vazia (pessoa +0 = −1); senão (ou sem vaga vazia) a do fim: cheio, cresce de 200 em 200 (realloc
/// 0x945667, ou malloc 0x944e46 sem vetor; sem memória, caixa 0x175 e −1) e a conta sobe. Devolve o índice. Os dois
/// chamadores (a criação dos contratos 0x4c7560 e a 0x4c7950) passam 1.
unsafe extern "thiscall" fn vaga_livre_de_contrato(this: usize, so_no_fim: u32) -> i32 {
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
        cpl4_por(this + 4, cap);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(p, cap.wrapping_mul(0x50) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(cap.wrapping_mul(0x50) as u32)
        };
        cpl4_por(this, q);
        if q == 0 {
            cpl4_sem_espaco(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = read_usize(this + 8).wrapping_add(1);
    cpl4_por(this + 8, n); // mais uma vaga
    (n as i32).wrapping_sub(1)
}

/// { 0x4c7950 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::add_staff_contract(pessoa, valor)`: a vaga
/// do fim (0x4c79a0 com 1); sem vaga, nada. Liga a pessoa à vaga (o índice +0 do registro de 0x4f bytes dela em
/// [0xae2d0c]), põe o id dela no contrato (+0) e gera o contrato inicial (0x85e960 com o valor). Pela contratação
/// de técnico (0x68f9c0, MANAGER_MANAGER::daily_update) e pelo regen que assina com um clube (0x7a6f10). Devolve o
/// eax da original (o da 0x85e960, ou o índice negativo); os chamadores não o leem.
unsafe extern "thiscall" fn acrescentar_contrato_da_pessoa(this: usize, pessoa: usize, valor: u32) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_FREE_STAFF_CONTRACT_SPACE);
    let i = vaga(this, 1);
    if i < 0 {
        return i as u32;
    }
    crate::comparacao::ramo("cpl_4 contrato da pessoa");
    cpl4_por(read_usize(0xae2d0c).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)), i as usize); // a pessoa aponta a vaga
    let c = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
    cpl4_por(c, read_usize(pessoa));
    let gerar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GENERATE_INITIAL_CONTRACT);
    gerar(c, pessoa, valor) // o valor da pessoa
}

/// As datas dos lembretes (tipo 2) que a data de decisão de tipo 1 leva antes dela: com o registro de jogador
/// (+0x61), L = moral do contrato (+0x44) + 100 (8 bits; o contrato pela tabela [0xae2d0c], como a 0x4cf5a0
/// inline: sem contrato a original lê o endereço 0x44 e cai, como aqui); L ≤ 0x50 sem lembrete; L > 0x96 um a
/// L × 90 / 200 dias antes; e um a L / 4 dias antes (sem o registro de jogador, L = 100). As datas pela
/// Date::sub_days (0x54c8e0), na ordem da original.
unsafe fn cpl4_lembretes(this: usize, pessoa: usize, data: usize, tipo: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if tipo != 1 {
        return (v, k);
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut l: u8 = 100;
    if read_usize(pessoa + 0x61) != 0 {
        let id = read_usize(pessoa) as i32;
        let c = if pessoa == 0 || id >= read_usize(0xae2364) as i32 {
            0
        } else {
            let i = read_usize(read_usize(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f))) as i32;
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

/// { 0x4cd6d0 } (thiscall, ret 0x10, contract_manager.cpp): `CONTRACT_MANAGER::add_decision_date(pessoa, data, fatia,
/// tipo)`: acrescenta ao vetor de 14 bytes [this+0xc] (capacidade +0x10, conta +0x14; cresce de 100 em 100 quando a
/// conta passa da capacidade − 4, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa 0x103e e nada) a entrada
/// (id da pessoa +0, data +4, fatia do dia +0xc, tipo +0xd). O tipo 1 leva antes os lembretes de tipo 2
/// (`cpl4_lembretes`), sem fatia (+0xc fica como está). Pelas propostas: a decisão da pessoa (0x4cd230, 0x4ce4e0).
/// As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
unsafe extern "thiscall" fn acrescentar_data_de_decisao(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    crate::comparacao::ramo("cpl_4 data de decisao");
    if read_usize(this + 0x14) as i32 > (read_usize(this + 0x10) as i32).wrapping_sub(4) {
        let cap = read_usize(this + 0x10).wrapping_add(100);
        cpl4_por(this + 0x10, cap);
        let p = read_usize(this + 0xc);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
            realocar(p, cap.wrapping_mul(14) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
            alocar(cap.wrapping_mul(14) as u32)
        };
        cpl4_por(this + 0xc, q);
        if q == 0 {
            cpl4_sem_espaco(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = cpl4_lembretes(this, pessoa, data, tipo as u8);
    for d in &lembretes[..k] {
        let n = read_usize(this + 0x14);
        let e = read_usize(this + 0xc).wrapping_add(n.wrapping_mul(14));
        cpl4_por(e, read_usize(pessoa));
        core::ptr::write_unaligned((e + 4) as *mut [u8; 8], *d);
        *((e + 0xd) as *mut u8) = 2;
        cpl4_por(this + 0x14, n.wrapping_add(1));
    }
    let n = read_usize(this + 0x14);
    let e = read_usize(this + 0xc).wrapping_add(n.wrapping_mul(14));
    cpl4_por(e, read_usize(pessoa));
    *((e + 0xc) as *mut u8) = fatia as u8;
    core::ptr::write_unaligned((e + 4) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    *((e + 0xd) as *mut u8) = tipo as u8;
    cpl4_por(this + 0x14, n.wrapping_add(1)); // a data da decisão
}

// ponytail: as três realocam (e a 0x4c7950 gera o contrato pela 0x85e960); não rodam duas vezes. O comparador monta
// o plano antes, roda só a original e confere; o modo Rust e a carreira provam as cópias.

/// Comparador: a vaga esperada (a primeira vazia, ou a do fim com a conta + 1 e a capacidade + 200 se cheio).
unsafe extern "thiscall" fn comparar_vaga_livre_de_contrato(this: usize, so_no_fim: u32) -> i32 {
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
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> i32>(913)(this, so_no_fim);
    if r == -1 && read_usize(this) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let (er, en, ecap) = if livre >= 0 { (livre, n, cap) } else { (n as i32, n.wrapping_add(1), if n == cap { cap.wrapping_add(200) } else { cap }) };
    PLACAR.registrar(r == er && read_usize(this + 8) == en && read_usize(this + 4) == ecap, || {
        format!("vaga_livre_de_contrato({}) devolveu {} (esperava {}), conta {} -> {} (esperava {}), capacidade {} -> {} (esperava {})",
            so_no_fim, r, er, n, read_usize(this + 8), en, cap, read_usize(this + 4), ecap)
    });
    r
}

/// Comparador: a vaga é a do fim (a conta de antes); confere a ligação da pessoa, o id no contrato e a conta.
unsafe extern "thiscall" fn comparar_acrescentar_contrato_da_pessoa(this: usize, pessoa: usize, valor: u32) -> u32 {
    let n = read_usize(this + 8);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(914)(this, pessoa, valor);
    if read_usize(this) == 0 {
        return r; // sem memória
    }
    let id = read_usize(pessoa);
    let ligada = read_usize(read_usize(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f)));
    let no_contrato = read_usize(read_usize(this).wrapping_add(n.wrapping_mul(0x50)));
    PLACAR.registrar(ligada == n && no_contrato == id && read_usize(this + 8) == n.wrapping_add(1), || {
        format!("acrescentar_contrato_da_pessoa(pessoa {}, valor {}) vaga {} (esperava {}), id no contrato {}, conta {} -> {}",
            id, valor, ligada, n, no_contrato, n, read_usize(this + 8))
    });
    r
}

/// Comparador: os lembretes e a entrada esperados antes da original; confere a conta e cada entrada nova.
unsafe extern "thiscall" fn comparar_acrescentar_data_de_decisao(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    let n = read_usize(this + 0x14);
    let (lembretes, k) = cpl4_lembretes(this, pessoa, data, tipo as u8);
    let d = core::ptr::read_unaligned(data as *const [u8; 8]);
    let id = read_usize(pessoa);
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(915)(this, pessoa, data, fatia, tipo);
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
        format!("acrescentar_data_de_decisao(pessoa {}, fatia {}, tipo {}) conta {} -> {} (esperava {}), lembretes {:?}, entradas {:?}",
            id, fatia as u8, tipo as u8, n, read_usize(this + 0x14), n + k + 1, &lembretes[..k],
            (0..=k).map(|j| core::ptr::read_unaligned(e(j) as *const [u8; 14])).collect::<Vec<_>>())
    });
}

// ---------- 247a leva: contratos e suspensão: as cláusulas de rebaixamento do clube rebaixado, tirar a oferta de contrato da pessoa ao clube e se o jogador está suspenso para a partida ----------


/// O contrato (registro de 80 bytes em [this], pelo índice +0 do registro de 79 bytes em [0xae2d0c]) da pessoa `p`
/// com o clube `clube` (+4 = id do clube) e a cláusula de rebaixamento ainda por acionar (+0x1f = 1); senão 0.
unsafe fn cpk3_clausula(this: usize, clube: usize, p: usize) -> usize {
    if p == 0 || read_usize(p) as i32 >= read_usize(0xae2364) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(0xae2d0c).wrapping_add(read_usize(p).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    let c = (i as usize).wrapping_mul(80).wrapping_add(read_usize(this));
    if c == 0 || read_usize(c + 4) != read_usize(clube) || *((c + 0x1f) as *const u8) != 1 {
        return 0;
    }
    c
}

/// O clube e o parceiro (time A/B, 0x540a50 ativo): o clube B não aciona (o A cuida dos dois). Clube nulo ou B:
/// nenhum.
unsafe fn cpk3_clubes(clube: usize) -> [usize; 2] {
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

/// { 0x4cd030 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::trigger_relegation_clauses(clube,
/// liga)`: no clube rebaixado e no parceiro (time A/B), cada um dos 50 da comissão e elenco (+0xd7) com a cláusula de
/// rebaixamento por acionar no contrato com o clube (+0x1f = 1) passa a 2 e sai a notícia (0x8e3db0 sobre
/// [0xde1f54] com o tipo 3).
unsafe extern "thiscall" fn acionar_clausulas_de_rebaixamento(this: usize, clube: usize, liga: usize) {
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_POST_RELEASE_CLAUSE_ACTIVATED_NEWS);
    for c in cpk3_clubes(clube) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = read_usize(c + 0xd7 + k * 4);
            let contrato = cpk3_clausula(this, c, p);
            if contrato != 0 {
                crate::comparacao::ramo("cpk_3 clausula de rebaixamento");
                *((contrato + 0x1f) as *mut u8) = 2; // a cláusula acionada
                noticia(read_usize(0xde1f54), p, liga, 3, contrato);
            }
        }
    }
}

/// A primeira oferta de contrato da lista `l` (sem mexer no cursor) da pessoa `p` (+0) ao clube `c` (+4).
unsafe fn cpk3_oferta(l: usize, p: usize, c: usize) -> Option<usize> {
    cpd1_itens(l).into_iter().find(|&e| read_usize(e) == p && read_usize(e + 4) == c)
}

/// { 0x4cca70 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer(pessoa, clube)`: a
/// primeira oferta da lista (+0x34) da pessoa ao clube (ids): devolve a reserva de registro (+0x63 ≠ −1: 0x7a81a0
/// sobre [0xb7434c] com +0x67, +0x6b), tira a marca de contrato acertado se ela estava acertada (+0x50 = 2: 0x8c5430),
/// sai da lista (0x68d1f0) e as pendentes da pessoa (+8 do registro de 79 bytes em [0xae2d0c]) descem uma.
unsafe extern "thiscall" fn tirar_oferta_de_contrato(this: usize, pessoa: *const u32, clube: *const u32) {
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
        devolver(read_usize(0xb7434c), reserva, read_usize(e + 0x67), read_usize(e + 0x6b));
    }
    if *((e + 0x50) as *const u8) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
        desmarcar(read_usize(0xde1f54), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    tirar(l, e);
    let n = (read_usize(0xae2d0c).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *mut u8;
    *n = (*n).wrapping_sub(1); // uma pendente a menos
}

/// { 0x55d270 } (thiscall, ret 0x10, discipline.cpp): `Discipline::is_player_suspended(jogador, competição, extra,
/// partida)`: jogador fora das pessoas ([0xae2364]) ou competição (id em `*comp`, vetor [0xadadfc]) inexistente: 0.
/// Suspenso pelo tribunal geral ([[this]], 0x7e85b0 com 1): 1. Senão, pelo tribunal da competição (+0x50, ≠ −1, com
/// 0): 1, e se o clube dele (+0x39) é de humano (0x600a50 com 0, 1, 0, 0), ele sai da vaga que ocupa entre as 20 de
/// +0x1e0. Senão 0.
unsafe extern "thiscall" fn esta_suspenso(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    if jogador != 0 && read_usize(jogador) as i32 >= read_usize(0xae2364) as i32 {
        return 0;
    }
    let v = read_usize(0xadadfc);
    if v == 0 {
        return 0;
    }
    let c = read_usize(v.wrapping_add((*comp as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_RULINGBODY_IS_PLAYER_BANNED);
    let tribunais = read_usize(this);
    if julgar(read_usize(tribunais), jogador, 1, extra, partida) != 0 {
        return 1; // a suspensão geral
    }
    let t = *((c + 0x50) as *const i8);
    if t == -1 {
        return 0;
    }
    let b = read_usize(tribunais.wrapping_add((t as isize as usize).wrapping_mul(4)));
    if b == 0 || julgar(b, jogador, 0, extra, partida) == 0 { // sem a suspensão da competição
        return 0;
    }
    crate::comparacao::ramo("cpk_3 suspenso pela competicao");
    let clube = read_usize(jogador + 0x39);
    if clube != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if humano(clube, 0, 1, 0, 0) != 0 {
            if let Some(s) = (0..20).map(|k| clube + 0x1e0 + k * 4).find(|&s| read_usize(s) == jogador) {
                *(s as *mut usize) = 0; // sai da vaga
            }
        }
    }
    1
}

/// Comparador: os contratos que a cópia acionaria (antes); a original aciona e dá as notícias (não roda duas vezes);
/// confere que todos ficaram com 2.
unsafe extern "thiscall" fn comparar_acionar_clausulas_de_rebaixamento(this: usize, clube: usize, liga: usize) {
    let mut alvos = Vec::new();
    for c in cpk3_clubes(clube) {
        if c != 0 {
            alvos.extend((0..50).map(|k| cpk3_clausula(this, c, read_usize(c + 0xd7 + k * 4))).filter(|&x| x != 0));
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(889)(this, clube, liga);
    let fora: Vec<usize> = alvos.iter().copied().filter(|&x| *((x + 0x1f) as *const u8) != 2).collect();
    PLACAR.registrar(fora.is_empty(), || format!("acionar_clausulas_de_rebaixamento({:x}) {} contratos, sem acionar {:x?}", clube, alvos.len(), fora));
}

/// Comparador: a lista esperada (sem a primeira oferta da pessoa ao clube) e as pendentes antes; a original tira (e
/// libera o item: nada dele é lido depois); confere os itens e o byte.
unsafe extern "thiscall" fn comparar_tirar_oferta_de_contrato(this: usize, pessoa: *const u32, clube: *const u32) {
    let l = read_usize(this + 0x34);
    if l == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(890)(this, pessoa, clube);
    }
    let n = (read_usize(0xae2d0c).wrapping_add((*pessoa as usize).wrapping_mul(79)) + 8) as *const u8;
    let alvo = cpk3_oferta(l, *pessoa as usize, *clube as usize);
    let esperado: Vec<usize> = cpd1_itens(l).into_iter().filter(|&e| Some(e) != alvo).collect();
    let pendentes = if alvo.is_some() { (*n).wrapping_sub(1) } else { *n };
    orig::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(890)(this, pessoa, clube);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *n == pendentes, || format!("tirar_oferta_de_contrato({}, {}) itens {} (esperava {}), pendentes {} (esperava {})", *pessoa, *clube, depois.len(), esperado.len(), *n, pendentes));
}

/// Comparador: lado a lado pelas 20 vagas (+0x1e0) do clube do jogador (a 0x7e85b0 e a 0x600a50 só leem).
unsafe extern "thiscall" fn comparar_esta_suspenso(this: usize, jogador: usize, comp: *const i32, extra: u32, partida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32, usize) -> u32>(891);
    let clube = if jogador != 0 { read_usize(jogador + 0x39) } else { 0 };
    let regioes: &[(usize, usize)] = if clube != 0 { &[(clube + 0x1e0, 80)] } else { &[] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || o(this, jogador, comp, extra, partida), || esta_suspenso(this, jogador, comp, extra, partida));
    PLACAR.registrar(igual && r == n, || format!("esta_suspenso({:x}, {}) original={} rust={} {:x?} x {:x?}", jogador, *comp, r, n, mo, mr));
    r
}

// ---------- 246a leva: as listas de pendências: soltar as suspensões guardadas, tirar as ofertas pendentes da pessoa e marcar a decisão depois da transferência acertada ----------


unsafe fn cpj2_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    f(l)
}

unsafe fn cpj2_tirar(l: usize, e: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    f(l, e);
}

/// O contador de suspensões guardadas (+0x1c) do registro de 29 bytes do id `id` em [this+0x14].
unsafe fn cpj2_guardadas(this: usize, id: usize) -> *mut i8 {
    (read_usize(this + 0x14).wrapping_add(id.wrapping_mul(29)) + 0x1c) as *mut i8
}

/// { 0x5567d0 } (thiscall, ret 4, discipline.cpp): `Discipline::drop_held_bans(pessoa)`: da lista das suspensões
/// guardadas (+0x1c; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item do id `*pessoa` enquanto o contador
/// do registro dele (+0x1c, com sinal) for positivo, descontando um a cada item.
unsafe extern "thiscall" fn soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 && *n > 0 {
        if read_usize(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            *n = (*n).wrapping_sub(1); // uma a menos
        }
        e = cpj2_proximo(l);
    }
}

/// O byte de ofertas pendentes (+8) do registro de 79 bytes da pessoa em [0xae2d0c].
unsafe fn cpj2_pendentes(id: usize) -> *mut u8 {
    (read_usize(0xae2d0c).wrapping_add(id.wrapping_mul(79)) + 8) as *mut u8
}

/// { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
unsafe extern "thiscall" fn tirar_ofertas_pendentes(this: usize, pessoa: *const u32) {
    let l = read_usize(this + 0x34);
    if l == 0 || *cpj2_pendentes(*pessoa as usize) == 0 {
        return;
    }
    let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_AGREED_CONTRACT_FLAG);
    desmarcar(read_usize(0xde1f54), pessoa);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 {
        if read_usize(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            let p = cpj2_pendentes(*pessoa as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // a última pendente
            }
        }
        e = cpj2_proximo(l);
    }
}

/// A primeira oferta da lista `l` (sem mexer no cursor) da pessoa (+0) e do clube (+4) da oferta de transferência e
/// ainda sem decisão (+0x62 = 0).
unsafe fn cpj2_oferta_a_marcar(l: usize, oferta: usize) -> Option<usize> {
    cpd1_itens(l).into_iter().find(|&e| read_usize(e) == read_usize(oferta + 0xc) && read_usize(e + 4) == read_usize(oferta + 8) && *((e + 0x62) as *const u8) == 0)
}

/// Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
unsafe fn cpj2_acima_do_clube(oferta: usize) -> u32 {
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(0x6e));
    let hab = *((read_usize(pessoa + 0x61) + 0xb) as *const i16) as i32 / 500;
    let clube = read_usize(0xae23b4).wrapping_add(read_usize(oferta + 8).wrapping_mul(0x245));
    (hab > *((clube + 0x80) as *const i16) as i32) as u32
}

/// { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
unsafe extern "thiscall" fn marcar_decisao_da_transferencia(this: usize, oferta: usize) -> u32 {
    let l = read_usize(this + 0x34);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 {
        if read_usize(e) == read_usize(oferta + 0xc) && read_usize(e + 4) == read_usize(oferta + 8) && *((e + 0x62) as *const u8) == 0 {
            *((e + 0x62) as *mut u8) = 1; // a oferta vale
            let agendar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_OFFER_DECISION);
            agendar(this, read_usize(oferta + 0xc) as u32, read_usize(oferta + 0x20) as u32);
            break;
        }
        e = cpj2_proximo(l);
    }
    cpj2_acima_do_clube(oferta)
}

/// Comparador: o plano da cópia (os itens do id que saem até o contador zerar) antes; a original tira; confere os
/// itens que ficaram e o contador.
unsafe extern "thiscall" fn comparar_soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *n;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta > 0 && read_usize(e) as u32 == *pessoa {
            resta -= 1;
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(867)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *n == resta, || format!("soltar_suspensoes_guardadas({}) itens {} -> {} (esperava {}), contador {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *n, resta));
}

/// Comparador: o mesmo plano pelo byte de pendentes; a original tira (e desmarca); confere os itens e o byte.
unsafe extern "thiscall" fn comparar_tirar_ofertas_pendentes(this: usize, pessoa: *const u32) {
    let l = read_usize(this + 0x34);
    if l == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, *const u32)>(868)(this, pessoa);
    }
    let p = cpj2_pendentes(*pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *p;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta != 0 && read_usize(e) as u32 == *pessoa {
            resta = resta.wrapping_sub(1);
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(868)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *p == resta, || format!("tirar_ofertas_pendentes({}) itens {} -> {} (esperava {}), pendentes {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *p, resta));
}

/// Comparador: a oferta que a cópia marcaria antes; a original marca e agenda (a 0x4cd230 pode acrescentar na lista
/// das decisões); confere a marca e o valor.
unsafe extern "thiscall" fn comparar_marcar_decisao_da_transferencia(this: usize, oferta: usize) -> u32 {
    let alvo = cpj2_oferta_a_marcar(read_usize(this + 0x34), oferta);
    let n = cpj2_acima_do_clube(oferta);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(869)(this, oferta);
    let marcada = alvo.map_or(true, |e| *((e + 0x62) as *const u8) == 1);
    PLACAR.registrar(marcada && r == n, || format!("marcar_decisao_da_transferencia({:x}) alvo {:x?} marcada={} original={} rust={}", oferta, alvo, marcada, r, n));
    r
}

// ---------- 245a leva: tirar as ofertas guardadas da pessoa, guardar as suspensões do emprestado e preencher a vaga da comissão ----------


/// { 0x4cf560 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_held_offers(pessoa)`: da lista das
/// ofertas guardadas (+0x38; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item cujo +0 é o id `pessoa`
/// (0x68d1b0 percorre). Sem lista: nada.
unsafe extern "thiscall" fn tirar_ofertas_guardadas(this: usize, pessoa: u32) {
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

/// A suspensão do tipo `t` (vetor de tipos [this], 4 bytes cada) vale no clube do empréstimo? Pela competição do tipo
/// (+0x17, −1 = nenhuma) igual à nação do clube (+0x53, id em +0); sem competição, pelo +0x1b (−1: vale) igual ao
/// [+0x71] da nação. Clube sem nação: não vale.
unsafe fn cpi5_vale_no_clube(this: usize, t: i8, clube: usize) -> bool {
    let tipo = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nacao = read_usize(clube + 0x53);
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
    let m = read_usize(nacao + 0x71);
    m != 0 && read_usize(m) as i32 == x // a do +0x71 da nação
}

/// O clube do empréstimo (+4 da ficha de contrato, clubes de 0x245 bytes em [0xae23b4]) e o registro de suspensões
/// do jogador (29 bytes em [this+0x14]); as vagas (5 bytes: tipo e o resto) que não valem lá, menos a 4ª (índice 3,
/// que o original pula).
unsafe fn cpi5_vagas_a_guardar(this: usize, ficha: usize, jogador: usize) -> (usize, Vec<u8>) {
    let clube = read_usize(0xae23b4).wrapping_add(read_usize(ficha + 4).wrapping_mul(0x245));
    let reg = read_usize(this + 0x14).wrapping_add(read_usize(jogador).wrapping_mul(29));
    let mut v = Vec::new();
    for k in 0..4u8 {
        let t = *((reg + 5 * k as usize) as *const i8);
        if k == 3 || t == -1 {
            continue;
        }
        if !cpi5_vale_no_clube(this, t, clube) {
            v.push(k);
        }
    }
    (reg, v)
}

/// { 0x5563f0 } (thiscall, ret 4, discipline.cpp): `Discipline::hold_bans_for_loan(jogador)`: com a ficha de contrato
/// do jogador (0x4cf6d0 sobre [0xae1050]), cada suspensão do registro dele (vagas 0 a 2) que não vale no clube do
/// empréstimo vai para a lista das guardadas (+0x1c, 0x68d0d0 com 10 bytes: o id, a vaga e os 5 bytes dela), a vaga
/// fica −1 e o contador +0x1c do registro sobe. Sem ficha: a original (caixa "Error").
unsafe extern "thiscall" fn guardar_suspensoes_do_emprestado(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let ficha = ficha_de(read_usize(0xae1050), jogador as *const i32);
    if ficha == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador); // caixa: só a original
    }
    let (reg, vagas) = cpi5_vagas_a_guardar(this, ficha, jogador);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    for k in vagas {
        let vaga = reg + 5 * k as usize;
        let mut r = [0u8; 10];
        r[..4].copy_from_slice(&(read_usize(jogador) as u32).to_le_bytes());
        r[4] = k; // a vaga
        r[5..].copy_from_slice(core::slice::from_raw_parts(vaga as *const u8, 5));
        acrescentar(this + 0x1c, r.as_ptr(), 10);
        core::ptr::write_unaligned(vaga as *mut u32, u32::MAX);
        *((vaga + 4) as *mut u8) = 0xff;
        *((reg + 0x1c) as *mut u8) = (*((reg + 0x1c) as *const u8)).wrapping_add(1);
    }
}

/// O cargo que a vaga `cargo` (byte com sinal) procura: 6, 8, 9 e 10 o mesmo; 13 procura 6 e 15 procura 8 (o byte do
/// argumento é trocado); os outros, nenhum.
fn cpi5_cargo_procurado(cargo: u32) -> Option<u32> {
    match cargo as u8 as i8 {
        6 | 8 | 9 | 10 => Some(cargo),
        13 => Some((cargo & !0xff) | 6),
        15 => Some((cargo & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// A primeira pessoa (de [0xae23bc], 0x6e bytes, [0xae2364] delas) sem clube (+0x39), no cargo (+0x3d), com registro
/// (+0x69) e da nação (+0x1a) do clube (+0x53).
unsafe fn cpi5_desempregado(clube: usize, cargo: u8) -> Option<usize> {
    let n = read_usize(0xae2364) as i32;
    let base = read_usize(0xae23bc);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        read_usize(p + 0x39) == 0 && *((p + 0x3d) as *const u8) == cargo && read_usize(p + 0x69) != 0 && read_usize(p + 0x1a) == read_usize(clube + 0x53)
    })
}

/// { 0x8cb6d0 } (stdcall, ret 8, transfer_manager.cpp): `fill_vacated_staff_job(clube, cargo)`: a vaga da comissão
/// que ficou livre é preenchida pela primeira pessoa desempregada da nação do clube no cargo procurado (6, 8, 9 e 10;
/// 13 procura 6, 15 procura 8): 0x53bd10 com o cargo (o dword do argumento), a data de hoje [0xae2c90], gravar e sem
/// reduzir. Clube nulo: a original (caixa "Error").
unsafe extern "stdcall" fn preencher_vaga_da_comissao(clube: usize, cargo: u32) {
    if clube == 0 {
        return orig::<unsafe extern "stdcall" fn(usize, u32)>(860)(clube, cargo); // caixa: só a original
    }
    let Some(c) = cpi5_cargo_procurado(cargo) else { return };
    if let Some(p) = cpi5_desempregado(clube, c as u8) {
        let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
        incluir(clube, p, c, 0xae2c90, 1, 0);
    }
}

/// Comparador: o plano antes (a 0x68d1f0 libera o item tirado, que não pode mais ser lido); a original tira; confere
/// que ficaram os itens de antes menos os da pessoa, na mesma ordem.
unsafe extern "thiscall" fn comparar_tirar_ofertas_guardadas(this: usize, pessoa: u32) {
    let l = read_usize(this + 0x38);
    let antes: Vec<usize> = if l != 0 { cpd1_itens(l) } else { Vec::new() };
    let esperado: Vec<usize> = antes.iter().copied().filter(|&e| read_usize(e) as u32 != pessoa).collect();
    orig::<unsafe extern "thiscall" fn(usize, u32)>(858)(this, pessoa);
    let depois: Vec<usize> = if l != 0 { cpd1_itens(l) } else { Vec::new() };
    PLACAR.registrar(depois == esperado, || format!("tirar_ofertas_guardadas({:x}) {} itens, esperava {}, ficaram {}", pessoa, antes.len(), esperado.len(), depois.len()));
}

/// Comparador: o plano da cópia (as vagas que não valem no clube do empréstimo) antes; a original guarda; confere o
/// registro do jogador: as vagas do plano em −1, as outras iguais e o contador somado.
unsafe extern "thiscall" fn comparar_guardar_suspensoes_do_emprestado(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let ficha = ficha_de(read_usize(0xae1050), jogador as *const i32);
    if ficha == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador);
    }
    let (reg, vagas) = cpi5_vagas_a_guardar(this, ficha, jogador);
    let mut esperado = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    for &k in &vagas {
        esperado[5 * k as usize..5 * k as usize + 5].fill(0xff);
    }
    esperado[0x1c] = esperado[0x1c].wrapping_add(vagas.len() as u8);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador);
    let depois = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    PLACAR.registrar(depois == esperado, || format!("guardar_suspensoes_do_emprestado({:x}) vagas {:?} {:x?} x {:x?}", jogador, vagas, depois, esperado));
}

/// Comparador: a escolha da cópia antes; sem ninguém, só a original. Com a pessoa, original e cópia lado a lado no
/// clube (0x245 bytes) e na pessoa (0x6e), que é tudo o que a 0x53bd10 grava: ela pode recusar (casa ocupada, faixa
/// dos humanos; com mais ligas acontece) ou gravar na seleção (+0x24), e a pessoa não fica em +0x39.
unsafe extern "stdcall" fn comparar_preencher_vaga_da_comissao(clube: usize, cargo: u32) {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32)>(860);
    let escolhida = if clube != 0 { cpi5_cargo_procurado(cargo).and_then(|c| cpi5_desempregado(clube, c as u8)) } else { None };
    let Some(p) = escolhida else {
        crate::comparacao::ramo("preencher_vaga_da_comissao sem ninguem");
        return o(clube, cargo);
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(clube, 0x245), (p, 0x6e)], || o(clube, cargo),
        || preencher_vaga_da_comissao(clube, cargo));
    if read_usize(p + 0x39) != clube {
        crate::comparacao::ramo("preencher_vaga_da_comissao sem entrar no clube");
    }
    PLACAR.registrar(igual, || format!("preencher_vaga_da_comissao({:x}, {}) pessoa {:x} {:x?} x {:x?}", clube, cargo as u8, p, mo, mr));
}

// ---------- 244a leva: o lugar do jogador no elenco e as subidas e descidas entre o principal e os reservas (contract_manager.cpp) ----------


/// O contrato da pessoa como a 0x4cbb60 acha por dentro (sem chamar a 0x4cf5a0): pessoa não nula, id abaixo de
/// [0xae2364], índice da tabela [0xae2d0c] (79 bytes por pessoa) não negativo; registro de 80 bytes em [objeto].
unsafe fn cph2_contrato(objeto: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = read_u32(pessoa) as i32;
    if id >= read_u32(0xae2364) as i32 {
        return 0;
    }
    let i = read_u32((read_u32(0xae2d0c) as usize).wrapping_add((id as usize).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    (read_u32(objeto) as usize).wrapping_add((i as usize).wrapping_mul(80))
}

/// { 0x4cbb60 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_placement(pessoa, clube)`: o
/// lugar da pessoa no `clube`: 0 nenhum, 1 reservas, 2 principal, 3 no time B ligado, 4 no clube principal com time B.
/// Só para os cargos de jogador (+0x3d diferente de 12 e 0x53b620). No próprio clube (+0x39) que pode ter reservas
/// (0x542740): 2 com o bit 0 de +0x5e, senão 1. O clube parceiro (0x540a50 com 1; o byte de saída é a vaga da
/// pessoa, que fica com o ponteiro dela se a 0x540a50 não gravar): sendo A e `clube` o próprio, 4; sendo B e `clube` o
/// parceiro A, 3. Senão pelo contrato (0x4cbb60 acha por dentro em [0xae1050], gravado em [0xae1080]): o clube do
/// contrato, que não é o próprio nem o parceiro já visto; com o bit 6 de +0x35 do contrato (achado de novo em [this]):
/// no `clube` com reservas, 1 com o bit e 2 sem; ou com o parceiro (0x540a50) sendo o `clube` (ou o do contrato o
/// `clube`), 3 com o bit e 4 sem. O jogo lê só o al.
unsafe extern "thiscall" fn lugar_no_elenco(this: usize, pessoa: usize, clube: usize) -> u8 {
    let tipo = *((pessoa + 0x3d) as *const i8);
    let tipo_de_jogador: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    if tipo == 12 || tipo_de_jogador(tipo) == 0 {
        return 0;
    }
    let reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let proprio = read_u32(pessoa + 0x39) as usize;
    if proprio != 0 && proprio == clube && tipo_de_jogador(*((pessoa + 0x3d) as *const i8)) != 0 && reservas(clube) != 0 {
        return if *((pessoa + 0x5e) as *const u8) & 1 != 0 { 2 } else { 1 }; // principal ou reservas
    }
    let mut vaga = pessoa as u32; // a vaga do argumento: a 0x540a50 pode não gravar
    let mut b = 0usize;
    if proprio != 0 {
        b = parceiro(proprio, &mut vaga, 1);
        if b != 0 {
            if vaga != 0 {
                if clube == read_u32(pessoa + 0x39) as usize {
                    return 4;
                }
            } else if clube == b {
                return 3;
            }
        }
    }
    let c = cph2_contrato(read_u32(0xae1050) as usize, pessoa);
    *(0xae1080 as *mut usize) = c; // a ficha do contrato
    if c == 0 {
        return 0;
    }
    let i = read_u32(c + 4) as usize;
    let outro = (read_u32(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if outro == 0 || outro == read_u32(pessoa + 0x39) as usize || outro == b {
        return 0;
    }
    let c2 = cph2_contrato(this, pessoa);
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

/// A 0x4cbb60 pelo endereço (passa pelo desvio) e a troca 0x4cbd20 (com 0 na saída do novo clube).
unsafe fn cph2_mover(this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let l = lugar(this, pessoa, clube);
    if l != de[0] && l != de[1] {
        return 0;
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TOGGLE_SQUAD_PLACEMENT);
    trocar(this, pessoa, clube, 0)
}

/// { 0x4cc0b0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_first_team(pessoa, clube)`: nos
/// reservas (1) ou no time B (3), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
unsafe extern "thiscall" fn subir_para_o_principal(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_mover(this, pessoa, clube, [1, 3]) // reservas ou time B
}

/// { 0x4cc0f0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_reserves(pessoa, clube)`: no
/// principal (2) ou no clube com time B (4), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
unsafe extern "thiscall" fn descer_para_os_reservas(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_mover(this, pessoa, clube, [2, 4]) // principal ou clube com time B
}

/// Comparador: lado a lado pela ficha [0xae1080] (a vaga da saída é o argumento na pilha de cada uma).
unsafe extern "thiscall" fn comparar_lugar_no_elenco(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(829);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, pessoa, clube), || lugar_no_elenco(this, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("lugar_no_elenco({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador das duas trocas: o lugar (0x4cbb60, pelo desvio) decide antes; fora dos lugares da troca, lado a lado
/// pela ficha [0xae1080] (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
unsafe fn cph2_comparar(i: usize, nome: &str, this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    let ficha = read_u32(0xae1080);
    let l = lugar(this, pessoa, clube);
    *(0xae1080 as *mut u32) = ficha;
    if l == de[0] || l == de[1] {
        return o(this, pessoa, clube);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, pessoa, clube), || cph2_mover(this, pessoa, clube, de));
    PLACAR.registrar(igual && r == n, || format!("{}({:x}, {:x}) lugar {} original={} rust={} {:x?} x {:x?}", nome, pessoa, clube, l, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_subir_para_o_principal(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_comparar(830, "subir_para_o_principal", this, pessoa, clube, [1, 3])
}

unsafe extern "thiscall" fn comparar_descer_para_os_reservas(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_comparar(831, "descer_para_os_reservas", this, pessoa, clube, [2, 4])
}

// ---------- 244a leva: a cópia do contrato de pessoal, a moral depois da boa partida e tirar as conversas de contrato (contract_manager.cpp) ----------



/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cph3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x4cc810 } (thiscall, ret 4, contract_manager.cpp): `STAFF_CONTRACT::operator=`: copia o contrato (0x50 bytes) de
/// `origem`: quatro duplas, três duplas (+0x10), cinco bytes (+0x1c), +0x21, as datas de início (+0x25) e do fim
/// (+0x2d) pela 0x417c10, o byte de campos de bits +0x35 em três passos (6 bits, o 7º, o 8º: no fim é o de `origem`),
/// +0x36, +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e e o +0x4f em dois passos (nibble
/// baixo e depois o resto). Devolve `this`.
unsafe extern "thiscall" fn copiar_contrato_de_pessoal(this: usize, origem: usize) -> usize {
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
    cph3_data(this + 0x25, origem + 0x25);
    cph3_data(this + 0x2d, origem + 0x2d);
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

/// { 0x4cb0a0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_raise_match_morale(pessoa, clube,
/// contrato)`: com os três e o registro de jogador (+0x61): x = (nota +0xb − reputação do clube +0x80) + 417 × +0x18
/// − 11680; y = 1 + x × (6,6e-5 se x < 0, senão 6,6667e-5), entre 0,01 e 2; o ganho = ftol((+0x3f + 5) / y), no máximo
/// 10. O teto: 100 (50 com a queixa 2), −50 por queixa 1, 8, 0x20, 0x40, 0x80, 0x100 enquanto o teto não passa de −50
/// para baixo. A moral (+0x44) soma o ganho entre −100 e 100 e fica no teto. Sem +0x49: com ganho > 5 e moral + ganho
/// > 0, ou felicidade (+0x3d) ≥ 75 − ganho, apaga as queixas 4 e 0x10 (0x8674e0). Por fim confere a retirada do pedido
/// (0x867240 com 0). Constantes do .rdata; o x87 em dupla precisão, como as outras do contrato.
unsafe extern "stdcall" fn subir_moral_pela_partida(pessoa: usize, clube: usize, contrato: usize) {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return;
    }
    let jogador = read_u32(pessoa + 0x61) as usize;
    if jogador == 0 {
        return;
    }
    let a = core::ptr::read_unaligned((jogador + 0xb) as *const i16) as i32 - core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    let x = (a as f64 + read_u8(pessoa + 0x18) as i8 as f64 * f64_em(0x968d90)) - f64_em(0x968d88);
    let mut y = (if x < f64_em(0x9689a0) { x * f64_em(0x968d80) } else { x * f64_em(0x968d78) }) + f64_em(0x967890);
    if y < f64_em(0x967898) {
        y = f64_em(0x967898);
    } else if y > f64_em(0x968950) {
        y = f64_em(0x968950);
    }
    let felicidade = (read_u8(contrato + 0x3f) as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut ganho = ftol(felicidade / y) as u8 as i8;
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
    if read_u8(contrato + 0x49) == 0 {
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

/// { 0x4ca1c0 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::remove_talks_records(pessoa, tecnico,
/// clube)`: no vetor de conversas ([this+0x28], [this+0x2c] registros de 21 bytes), cada um da pessoa (+0 = id) e,
/// quando dados, do clube (+4) e do técnico (+8) recebe por cima o último (5 duplas e um byte), a contagem desce e o
/// mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x4dc970).
unsafe extern "thiscall" fn tirar_conversas_de_contrato(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
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

unsafe extern "thiscall" fn comparar_copiar_contrato_de_pessoal(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(832);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x50)], || o(this, origem), || copiar_contrato_de_pessoal(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_contrato_de_pessoal {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: a 0x8674e0 e a 0x867240 mexem nas queixas, nos vínculos e na transferência (não podem rodar duas
/// vezes): só a original; a prova é a carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (153.222 chamadas); comparar pedia separar a moral das chamadas do fim.
unsafe extern "stdcall" fn comparar_subir_moral_pela_partida(pessoa: usize, clube: usize, contrato: usize) {
    orig::<unsafe extern "stdcall" fn(usize, usize, usize)>(833)(pessoa, clube, contrato)
}

unsafe extern "thiscall" fn comparar_tirar_conversas_de_contrato(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(834);
    let n = (*((this + 0x2c) as *const i32)).max(0) as usize;
    let reg = [(this + 0x28, 8), (read_u32(this + 0x28) as usize, n * 21)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, tecnico, clube), || tirar_conversas_de_contrato(this, pessoa, tecnico, clube));
    PLACAR.registrar(igual, || format!("tirar_conversas_de_contrato({:x}, {:x}, {:x}) {:x?} x {:x?}", pessoa, tecnico, clube, mo, mr));
}

// ---------- 190a leva: o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto (contract_manager.cpp, shortlist_manager.cpp, human_manager.cpp) ----------

/// A entrada de 79 bytes da pessoa `*id` na tabela [0xae2d0c] (em +4 a vaga do empréstimo, −1 sem).
unsafe fn entrada_do_emprestimo(id: *const i32) -> usize {
    (*(0xae2d0c as *const usize) as isize + *id as isize * 79) as usize
}

/// { 0x4d0bd0 } (thiscall, ret 4, contract_manager.cpp): libera a vaga do contrato de empréstimo da
/// pessoa, se ela tiver uma: −1 no começo da vaga (80 bytes cada, em [gerente]) e na entrada da pessoa.
unsafe extern "thiscall" fn tirar_contrato_de_emprestimo(gerente: usize, id: *const i32) {
    let e = entrada_do_emprestimo(id);
    let vaga = core::ptr::read_unaligned((e + 4) as *const i32);
    if vaga != -1 {
        *((*(gerente as *const usize) as isize + vaga as isize * 80) as *mut i32) = -1;
        core::ptr::write_unaligned((entrada_do_emprestimo(id) + 4) as *mut i32, -1);
    }
}

/// A lista de observados do clube `*clube` (9 bytes por clube em [gerente+0xdc]: +0 as vagas, +4 a marca).
unsafe fn lista_do_clube(gerente: usize, clube: *const i32) -> usize {
    (*((gerente + 0xdc) as *const usize) as isize + *clube as isize * 9) as usize
}

/// { 0x84aeb0 } (thiscall, ret 8, shortlist_manager.cpp): com a lista do clube ativa (marca 1), tira a
/// marca de lance (+0x17 = −1) das 40 vagas (0x25 bytes cada) cujo alvo (+0xf) é o jogador `*jogador`.
unsafe extern "thiscall" fn desmarcar_lances_pelo_jogador(gerente: usize, clube: *const i32, jogador: *const i32) {
    let l = lista_do_clube(gerente, clube);
    if *((l + 4) as *const u8) != 1 {
        return;
    }
    for i in 0..40 {
        let v = core::ptr::read_unaligned(l as *const usize) + i * 0x25;
        if core::ptr::read_unaligned((v + 0xf) as *const i32) == *jogador {
            core::ptr::write_unaligned((v + 0x17) as *mut i32, -1);
        }
    }
}

/// { 0x600e90 } (cdecl, human_manager.cpp): quem responde pelo técnico do clube: o auxiliar (+0xd3)
/// se houver; senão, dos cinco da comissão (+0x19f) com dados (+0x69), o de maior reputação (+0xa,
/// ficando o primeiro no empate); 0 se nenhum.
unsafe extern "C" fn tecnico_substituto(clube: usize) -> usize {
    let auxiliar = core::ptr::read_unaligned((clube + 0xd3) as *const usize);
    if auxiliar != 0 {
        return auxiliar;
    }
    let reputacao = |p: usize| core::ptr::read_unaligned((core::ptr::read_unaligned((p + 0x69) as *const usize) + 0xa) as *const i16);
    let mut melhor = 0usize;
    for k in 0..5 {
        let c = core::ptr::read_unaligned((clube + 0x19f + k * 4) as *const usize);
        if c == 0 || core::ptr::read_unaligned((c + 0x69) as *const usize) == 0 {
            continue;
        }
        if melhor == 0 || reputacao(c) > reputacao(melhor) {
            melhor = c;
        }
    }
    melhor
}

unsafe extern "thiscall" fn comparar_tirar_contrato_de_emprestimo(gerente: usize, id: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(442);
    let e = entrada_do_emprestimo(id);
    let mut reg = vec![(e + 4, 4)];
    let vaga = core::ptr::read_unaligned((e + 4) as *const i32);
    if vaga != -1 {
        reg.push(((*(gerente as *const usize) as isize + vaga as isize * 80) as usize, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || tirar_contrato_de_emprestimo(gerente, id));
    PLACAR.registrar(igual, || format!("tirar_contrato_de_emprestimo({}) {:x?} x {:x?}", *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_desmarcar_lances_pelo_jogador(gerente: usize, clube: *const i32, jogador: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, *const i32)>(443);
    let l = lista_do_clube(gerente, clube);
    let reg = if *((l + 4) as *const u8) == 1 { vec![(core::ptr::read_unaligned(l as *const usize), 40 * 0x25)] } else { vec![(l, 9)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, clube, jogador), || desmarcar_lances_pelo_jogador(gerente, clube, jogador));
    PLACAR.registrar(igual, || format!("desmarcar_lances_pelo_jogador({}, {}) {:x?} x {:x?}", *clube, *jogador, mo, mr));
}

unsafe extern "C" fn comparar_tecnico_substituto(clube: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize) -> usize>(444)(clube);
    let n = tecnico_substituto(clube);
    PLACAR.registrar(r == n, || format!("tecnico_substituto({:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

// ---------- 179a leva: as ofertas de contrato e a oferta de transferência fechada ----------

/// Percorre a lista do gerente de contratos (+0x34, cursor em +0xa voltando ao começo +2, 0x68d1b0)
/// até a oferta que `achou` aceita; 0 sem lista ou sem nenhuma.
unsafe fn procurar_oferta(gerente: usize, achou: impl Fn(usize) -> bool) -> usize {
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

/// { 0x4cc9e0 } (thiscall, ret 8, contract_manager.cpp): a oferta de contrato da pessoa `pessoa`
/// (+0) para o clube `clube` (+4), ou 0.
unsafe extern "thiscall" fn oferta_de_contrato(gerente: usize, pessoa: i32, clube: i32) -> usize {
    procurar_oferta(gerente, |o| *(o as *const i32) == pessoa && *((o + 4) as *const i32) == clube)
}

/// { 0x4cca30 } (thiscall, ret 4, contract_manager.cpp): a oferta acertada (+0x50 = 2) da pessoa, ou 0.
unsafe extern "thiscall" fn oferta_acertada(gerente: usize, pessoa: i32) -> usize {
    procurar_oferta(gerente, |o| *(o as *const i32) == pessoa && *((o + 0x50) as *const u8) == 2)
}

/// { 0x8d0d90 } (stdcall, transfer_offer.cpp): a oferta de transferência está encerrada? Estados
/// (+0x2c) 2, 3, 4, 0x10 e 0x14 sim; 0xf só com +0x2e a partir de 3; os demais não.
unsafe extern "system" fn oferta_fechada(o: usize) -> u32 {
    match *((o + 0x2c) as *const u8) {
        2 | 3 | 4 | 0x10 | 0x14 => 1,
        0xf => (*((o + 0x2e) as *const u8) >= 3) as u32,
        _ => 0,
    }
}

/// As regiões que as buscas mexem: o cursor da lista do gerente.
unsafe fn regioes_da_lista(gerente: usize) -> Vec<(usize, usize)> {
    let lista = *((gerente + 0x34) as *const usize);
    if lista == 0 { Vec::new() } else { vec![(lista, 0xe)] }
}

unsafe extern "thiscall" fn comparar_oferta_de_contrato(gerente: usize, pessoa: i32, clube: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> usize>(409);
    let reg = regioes_da_lista(gerente);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, pessoa, clube), || oferta_de_contrato(gerente, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("oferta_de_contrato({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_oferta_acertada(gerente: usize, pessoa: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(410);
    let reg = regioes_da_lista(gerente);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, pessoa), || oferta_acertada(gerente, pessoa));
    PLACAR.registrar(igual && r == n, || format!("oferta_acertada({}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

unsafe extern "system" fn comparar_oferta_fechada(o: usize) -> u32 {
    let r = orig::<unsafe extern "system" fn(usize) -> u32>(411)(o);
    let n = oferta_fechada(o);
    PLACAR.registrar(r == n, || format!("oferta_fechada(estado {:#x}, {:#x}) original={} rust={}", *((o + 0x2c) as *const u8), *((o + 0x2e) as *const u8), r, n));
    r
}

