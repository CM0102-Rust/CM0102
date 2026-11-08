// ---------- %%LEVA%%a leva: o contrato assinado e posto no vetor dos contratos (contract_manager.cpp) ----------

unsafe fn cqg1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg1_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqg1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqg1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqg1_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqg1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqg1_clube(id: usize) -> usize {
    cqg1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O registro de 79 bytes da pessoa `id` em [0xae2d0c] (+0 o índice do contrato no vetor).
unsafe fn contract_manager_staff_state_record(id: usize) -> usize {
    cqg1_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f))
}

/// A cópia do contrato de 0x50 bytes de `o` para `d`, campo a campo como a original (as datas +0x25 e +0x2d pela
/// 0x417c10; os campos de bits de +0x35 e o nibble de +0x4f, mesclados em passos, acabam iguais à origem).
unsafe fn copy_contract(d: usize, o: usize) {
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    for k in (0..0x1c).step_by(4) {
        cqg1_pd(d + k, cqg1_d(o + k));
    }
    for k in 0x1c..0x21 {
        cqg1_pb(d + k, cqg1_b(o + k));
    }
    cqg1_pd(d + 0x21, cqg1_d(o + 0x21));
    let mut tmp = [0u8; 8];
    copiar_data(d + 0x25, tmp.as_mut_ptr(), o + 0x25);
    copiar_data(d + 0x2d, tmp.as_mut_ptr(), o + 0x2d);
    let s = cqg1_b(o + 0x35);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ cqg1_b(d + 0x35)) & 0x3f) ^ cqg1_b(d + 0x35);
    let al = ((((s << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl;
    cqg1_pb(d + 0x35, ((al ^ s) & 0x7f) ^ s);
    cqg1_pd(d + 0x36, cqg1_d(o + 0x36));
    cqg1_pb(d + 0x3a, cqg1_b(o + 0x3a));
    cqg1_pw(d + 0x3b, cqg1_u(o + 0x3b));
    for k in [0x3dusize, 0x3e, 0x3f] {
        cqg1_pb(d + k, cqg1_b(o + k));
    }
    cqg1_pd(d + 0x40, cqg1_d(o + 0x40));
    cqg1_pb(d + 0x44, cqg1_b(o + 0x44));
    cqg1_pd(d + 0x45, cqg1_d(o + 0x45));
    cqg1_pb(d + 0x49, cqg1_b(o + 0x49));
    cqg1_pd(d + 0x4a, cqg1_d(o + 0x4a));
    cqg1_pb(d + 0x4e, cqg1_b(o + 0x4e));
    let s = cqg1_b(o + 0x4f);
    let a = ((cqg1_b(d + 0x4f) ^ s) & 0xf) ^ cqg1_b(d + 0x4f);
    cqg1_pb(d + 0x4f, ((a ^ s) & 0xf) ^ s);
}

/// O tipo do contrato (+0x3d) pelo cargo (nibble alto de +0x4f): 1 → 75, 2 → 50, 3 → 40, 4, 5, 6 e 8 → 0; os outros
/// não mexem (a tabela de saltos da original).
unsafe fn set_contract_type_from_job(c: usize) {
    match cqg1_b(c + 0x4f) >> 4 {
        1 => cqg1_pb(c + 0x3d, 0x4b),
        2 => cqg1_pb(c + 0x3d, 0x32),
        3 => cqg1_pb(c + 0x3d, 0x28),
        4 | 5 | 6 | 8 => cqg1_pb(c + 0x3d, 0),
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
unsafe extern "thiscall" fn sign_contract(this: usize, contrato: usize, cargo: u32) -> u32 {
    let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
    let tem_vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x53b9a0usize);
    let noticia_sem_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d64e0usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let olheiros: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7f7270usize);
    let f8b2880: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b2880usize);
    let elenco: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let noticia_teste: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4dade0usize);
    let f90d860: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x90d860usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, i32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c770usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let marcar_elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x540980usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let zerar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x866ee0usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
    let c = contrato;
    let clube = cqg1_clube(cqg1_d(c + 4));
    let pessoa = cqg1_d(0xae23bc).wrapping_add(cqg1_d(c).wrapping_mul(0x6e));
    let emprestimo = || cqg1_b(c + 0x35) & 0x3f == 5;
    if cqg1_d(pessoa + 0x39) == 0 && !emprestimo() && incluir(clube, pessoa, cargo, 0xae2c90, 1, 0) == 0 {
        return 0;
    }
    if cargo as u8 != cqg1_b(pessoa + 0x3d) && !emprestimo() && tem_vaga(cqg1_d(pessoa + 0x39), cargo, pessoa) == 0 {
        noticia_sem_vaga(this, pessoa, cqg1_d(pessoa + 0x39));
        return 0;
    }
    if cqg1_d(pessoa + 0x39) != 0 && humano(cqg1_d(pessoa + 0x39), 1, 1, 0, 0) != 0 {
        olheiros(cqg1_d(0xb74448), cqg1_d(cqg1_d(pessoa + 0x39) + 0xcf));
        f8b2880(cqg1_d(0xde1f44), cqg1_d(pessoa + 0x39), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if emprestimo() {
        crate::comparison::branch("cqg_1 empréstimo");
        if (elenco(clube, 0, 0) as i32) + 1 > 0x32 {
            noticia_teste(this, pessoa, clube);
            return 0;
        }
        f90d860(cqg1_d(0xde2f70), cqg1_clube(cqg1_d(c + 4)), 0);
        incluir(cqg1_clube(cqg1_d(c + 4)), pessoa, 0xb, 0xae2c90, 1, 0);
        let n = dias_ate(c + 0x2d, cqg1_d(c + 0x25) as u32, cqg1_d(c + 0x29) as u32);
        let fim = mais(0xae2c90, tmp2.as_mut_ptr(), n); // cqg1: o fim pela duração
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), fim);
        cqg1_pb(c + 0x4f, (cqg1_b(c + 0x4f) & 0xf) | 0x80);
        copiar_data(c + 0x25, tmp.as_mut_ptr(), 0xae2c90);
        marcar_elenco(pessoa, 1);
    }
    // o fim no mínimo 4 semanas depois do início
    let (ini_dia, ini_ano) = (cqg1_w(c + 0x25), cqg1_u(c + 0x27));
    let fim_ano = cqg1_u(c + 0x2f);
    if ini_ano > fim_ano || (ini_ano == fim_ano && !(ini_dia < cqg1_w(c + 0x2d))) {
        crate::comparison::branch("cqg_1 fim em 4 semanas");
        let p = mais_semanas(c + 0x25, tmp2.as_mut_ptr(), 4); // cqg1: as 4 semanas
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), p);
    }
    copiar_data(pessoa + 0x46, tmp.as_mut_ptr(), c + 0x2d);
    let idx = cqg1_d(contract_manager_staff_state_record(cqg1_d(c))) as i32;
    if idx > -1 {
        let off = (idx as usize).wrapping_mul(0x50);
        let r = cqg1_d(this).wrapping_add(off);
        if cqg1_d(r + 4) == cqg1_d(c + 4) { // cqg1: o mesmo clube
            crate::comparison::branch("cqg_1 mesmo clube");
            let r = || cqg1_d(this).wrapping_add(off);
            cqg1_pb(c + 0x3a, cqg1_b(r() + 0x3a));
            for k in [0x3dusize, 0x3e, 0x3f, 0x44] {
                cqg1_pb(c + k, cqg1_b(r() + k));
            }
            cqg1_pd(c + 0x45, cqg1_d(r() + 0x45));
            cqg1_pb(c + 0x4e, cqg1_b(r() + 0x4e));
            cqg1_pb(c + 0x49, 0);
            cqg1_pd(c + 0x4a, u32::MAX as usize);
            let a = cqg1_b(c + 0x4f);
            cqg1_pb(c + 0x4f, ((cqg1_b(r() + 0x4f) ^ a) & 0xf) ^ a);
            let a = cqg1_b(c + 0x35);
            cqg1_pb(c + 0x35, ((((cqg1_b(r() + 0x35) << 1) as i8 >> 1) as u8 ^ a) & 0x40) ^ a);
            cqg1_pw(c + 0x3b, cqg1_u(r() + 0x3b));
            let nacao = cqg1_d(clube + 0x53);
            if nacao != 0 && cqg1_d(nacao) == cqg1_d(0x9cf434) && comunitaria(pessoa, 0) == 0 {
                let p = mais_dias(0xae2c90, tmp2.as_mut_ptr(), 0x64);
                let (dia, ano) = (cqg1_w(p), cqg1_u(p + 2));
                let depois = ano > cqg1_u(pessoa + 0x40) || (ano == cqg1_u(pessoa + 0x40) && dia > cqg1_w(pessoa + 0x3e));
                if depois {
                    let primeira = cqg1_d(pessoa + 0x1a);
                    if (primeira != 0 && cqg1_d(primeira) == cqg1_d(0x9cf260)) || sorteio(3) == 0 {
                        cqg1_pd(pessoa + 0x1e, cqg1_d(0xae23a8).wrapping_add(cqg1_d(0x9cf434).wrapping_mul(0x122)));
                    }
                }
            }
        } else {
            set_contract_type_from_job(c);
            cqg1_pd(c + 0x4a, u32::MAX as usize);
            cqg1_pb(c + 0x49, 0);
            zerar(c, pessoa, cqg1_clube(cqg1_d(c + 4)));
            cqg1_pw(c + 0x3b, 0);
        }
        copy_contract(cqg1_d(this).wrapping_add(off), c);
    } else {
        let conta = cqg1_d(this + 8) as i32;
        let mut i = 0i32;
        while i < conta {
            let r = cqg1_d(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if cqg1_d(r) as i32 == -1 { // cqg1: a vaga livre
                crate::comparison::branch("cqg_1 vaga livre");
                set_contract_type_from_job(c);
                cqg1_pb(c + 0x49, 0);
                cqg1_pd(c + 0x4a, u32::MAX as usize);
                zerar(c, pessoa, cqg1_clube(cqg1_d(c + 4)));
                copy_contract(r, c);
                cqg1_pd(contract_manager_staff_state_record(cqg1_d(r)), i as u32 as usize);
                let f = financas(cqg1_d(0xae2a38), cqg1_clube(cqg1_d(c + 4)));
                if f != 0 {
                    folha(f);
                }
                return 1;
            }
            i += 1;
        }
        if conta >= cqg1_d(this + 4) as i32 {
            let cap = (cqg1_d(this + 4) as u32).wrapping_add(0x64);
            cqg1_pd(this + 4, cap as usize);
            let p = if cqg1_d(this) == 0 { malloc(cap.wrapping_mul(0x50)) } else { realocar(cqg1_d(this), cap.wrapping_mul(0x50)) };
            cqg1_pd(this, p);
            if p == 0 {
                show_error_at(0x9a8190, 0x1637);
            }
        }
        set_contract_type_from_job(c);
        cqg1_pb(c + 0x49, 0);
        cqg1_pd(c + 0x4a, u32::MAX as usize);
        zerar(c, pessoa, cqg1_clube(cqg1_d(c + 4)));
        cqg1_pw(c + 0x3b, 0);
        cqg1_pd(contract_manager_staff_state_record(cqg1_d(c)), cqg1_d(this + 8));
        let n = cqg1_d(this + 8);
        let r = cqg1_d(this).wrapping_add(n.wrapping_mul(0x50));
        cqg1_pd(this + 8, n.wrapping_add(1));
        copy_contract(r, c);
    }
    crate::comparison::branch("cqg_1 valor na pessoa");
    cqg1_pd(pessoa + 0x4e, cqg1_d(c + 0xc)); // cqg1: o valor do contrato na pessoa
    if cqg1_b(c + 0x35) & 0x3f == 2 && cqg1_d(pessoa + 0x61) != 0 {
        moral(pessoa, 0x14, 0);
    }
    let f = financas(cqg1_d(0xae2a38), cqg1_clube(cqg1_d(c + 4)));
    if f != 0 {
        folha(f);
    }
    if cargo as u8 == cqg1_b(pessoa + 0x3d) {
        return 1;
    }
    if tirar(cqg1_d(pessoa + 0x39), pessoa, 0, 1, 0) == 0 {
        show_error_at(0x9a8190, 0x1672);
        return 0;
    }
    if incluir(cqg1_clube(cqg1_d(c + 4)), pessoa, cargo, pessoa + 0x3e, 1, 0) != 0 {
        return 1;
    }
    show_error_at(0x9a8190, 0x167a);
    0
}

/// Comparador da 0x4cf720: só a original (inclui e tira a pessoa dos clubes, realoca o vetor dos contratos, entrega
/// notícias, sorteia a segunda nacionalidade, recalcula a folha).
unsafe extern "thiscall" fn compare_sign_contract(this: usize, contrato: usize, cargo: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, contrato, cargo) // ponytail: só a original
}
