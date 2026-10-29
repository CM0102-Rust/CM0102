// ---------- %%LEVA%%a leva: as vagas de técnico e o agendamento das ofertas: abrir as vagas que faltam, a melhor nota de candidato a técnico e agendar a oferta (manager_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpn5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpn5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O `clube` já tem vaga na lista [this+8] (+0 conta, cursor +0xa recomeçado de +2, iterador 0x68d1b0; +0xe da vaga o
/// id do clube)? `None`: a lista acabou antes da conta (a caixa da original).
unsafe fn cpn5_tem_vaga(this: usize, clube: usize) -> Option<bool> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cpn5_d(this + 8);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpn5_d(l + 2));
    if cpn5_w(cpn5_d(this + 8)) <= 0 {
        return Some(false);
    }
    let mut j = 0i32;
    loop {
        let v = proximo(cpn5_d(this + 8));
        if v == 0 {
            return None;
        }
        if cpn5_d(v + 0xe) == cpn5_d(clube) {
            crate::comparacao::ramo("cpn_5 clube com vaga aberta");
            return Some(true);
        }
        j += 1;
        if j >= cpn5_w(cpn5_d(this + 8)) as i32 {
            return Some(false);
        }
    }
}

/// O `clube` está sem técnico (0x53ca90), pode ser dirigido (0x544820), não é o time B de um par (0x540a50 com 1
/// achando o parceiro com 0 em `eh_a`) e não tem vaga aberta? `None`: a caixa da lista.
unsafe fn cpn5_falta_vaga(this: usize, clube: usize, eh_a: &mut u32) -> Option<bool> {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    if tecnico(clube) != 0 || dirigivel(clube) == 0 {
        return Some(false);
    }
    if parceiro(clube, eh_a, 1) != 0 && *eh_a == 0 {
        return Some(false);
    }
    cpn5_tem_vaga(this, clube).map(|tem| !tem)
}

/// { 0x69d060 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::fix_missing_jobs()`: cada clube
/// ([0xae23b4], 0x245 bytes, [0xae235c] relido a cada volta) sem técnico, dirigível, que não é time B e sem vaga
/// aberta (`cpn5_falta_vaga`) ganha o aviso "Job not found for %s" (nome +4) em [0xde1f64] e a vaga (0x69ba50 sem
/// técnico anterior). A lista que acaba antes da conta: a caixa "Error" (linha 0x311e) e a rotina acaba. Sem SEH, sem
/// sorteio; a local do parceiro começa em 0 uma vez só e só é lida quando a 0x540a50 o acha (e a grava).
unsafe extern "thiscall" fn abrir_vagas_que_faltam(this: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x69ba50usize);
    let mut eh_a = 0u32;
    let mut clube = cpn5_d(0xae23b4);
    let mut i = 0i32;
    while i < cpn5_d(0xae235c) as i32 {
        match cpn5_falta_vaga(this, clube, &mut eh_a) {
            None => {
                aviso_em(0x9f18a4, 0x311e);
                return;
            }
            Some(true) => {
                formatar(0xde1f64, 0xa05298, clube + 4);
                abrir(this, clube, 0);
            }
            Some(false) => {}
        }
        i += 1;
        clube += 0x245;
    }
}

/// { 0x69c540 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::best_manager_candidate_score(clube,
/// atual)`: a maior nota de técnico (0x69d1b0 com 0 e o cargo 5) entre as pessoas ([0xae23bc], 0x6e bytes,
/// [0xae2364] relido a cada volta) com ficha da comissão (+0x69), das 16 últimas só as humanas (+0x5f), que não são o
/// `atual`, sem as marcas 0x30 no byte +0xb do registro de 79 bytes ([0xae2d0c]), sem contrato de empréstimo (0x4cf6d0),
/// que não são candidatas de outra vaga (0x69fc60), com nação (+0x1a) e o clube com nação (+0x53): de outra nação só
/// com reputação (+0x69 → +0xa) a partir de 3750; de outro continente (+0x75) com +0x69 → +0xc a partir de 4250 e o
/// clube com reputação (+0x80) a partir de 4750; jogador (+0x61) com clube (+0x39) e cargo 11, 12, 13 ou 15 não pode ter
/// contrato com o clube (0x4cf5f0) marcado em +0x49; e aceita para o cargo (0x69f810 com 3,5). Ninguém: −10000.
unsafe extern "thiscall" fn melhor_nota_de_candidato_a_tecnico(this: usize, clube: usize, atual: usize) -> i32 {
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let candidato: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x69fc60usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let serve: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(0x69f810usize);
    let nota_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x69d1b0usize);
    let mut melhor = -10000i32;
    let mut p = cpn5_d(0xae23bc);
    let mut i = 0i32;
    while i < cpn5_d(0xae2364) as i32 {
        let reg79 = cpn5_d(0xae2d0c).wrapping_add((i as usize).wrapping_mul(0x4f));
        let pula = 'f: {
            if cpn5_d(p + 0x69) == 0 {
                break 'f true;
            }
            if i >= (cpn5_d(0xae2364) as i32).wrapping_sub(0x10) && cpn5_b(p + 0x5f) == 0 {
                break 'f true;
            }
            if atual == p {
                crate::comparacao::ramo("cpn_5 o tecnico atual fica de fora");
                break 'f true; // o atual fica de fora
            }
            if cpn5_b(reg79 + 0xb) & 0x30 != 0 || emprestimo(cpn5_d(0xae1050), p) != 0 || candidato(this, p) != 0 {
                break 'f true;
            }
            let (nc, np) = (cpn5_d(clube + 0x53), cpn5_d(p + 0x1a));
            if nc != np && cpn5_w(cpn5_d(p + 0x69) + 0xa) < 0xea6 {
                break 'f true;
            }
            if nc == 0 || np == 0 {
                break 'f true;
            }
            if cpn5_b(nc + 0x75) != cpn5_b(np + 0x75) && (cpn5_w(cpn5_d(p + 0x69) + 0xc) < 0x109a || cpn5_w(clube + 0x80) < 0x128e) {
                break 'f true;
            }
            if cpn5_d(p + 0x61) != 0 && cpn5_d(p + 0x39) != 0 && matches!(cpn5_b(p + 0x3d), 0xb | 0xc | 0xd | 0xf) {
                let k = contrato(cpn5_d(0xae1050), p, clube);
                if k != 0 && cpn5_b(k + 0x49) != 0 {
                    break 'f true;
                }
            }
            serve(this, p, clube, 3.5) == 0
        };
        if !pula {
            let s = nota_de(this, p, clube, 0, 5);
            if s > melhor {
                melhor = s;
            }
        }
        i += 1;
        p += 0x6e;
    }
    melhor
}

/// O plano do agendamento: a data da oferta (hoje [0xae2c90] + `dias`; com `dias` ≥ 50 a original mostra a caixa e
/// usa 10) com a fatia [0xae2c88] + `fatias` trazida para 0..2 (cada 3 fatias, mais um dia), e o balde da lista: os
/// dias desde a data base (+0x84d; ainda sem ano em +0x84f, hoje) mais o começo (+0x84c), módulo 50 de 50 a 127, × 3 +
/// fatia, nas listas de 14 bytes a partir de +0x18. Só lê (0x54c5b0 e 0x54cdb0 sobre cópias das datas).
unsafe fn cpn5_plano(this: usize, dias: u8, fatias: u8) -> ([u8; 8], i8, usize) {
    let somar: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let desde: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let base = if cpn5_w(this + 0x84f) == 0 { 0xae2c90 } else { this + 0x84d };
    let (b0, b1) = (cpn5_d(base) as u32, cpn5_d(base + 4) as u32);
    let dias = if dias as i8 >= 0x32 { 10 } else { dias as i8 };
    let (mut data, mut tmp) = ([0u8; 8], [0u8; 8]);
    // a 0x54c5b0 lê os dias em 16 bits (a original empurra o registro com lixo nos bits altos)
    let r = somar(0xae2c90, tmp.as_mut_ptr(), dias as i16 as i32 as u32);
    core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    let mut fatia = cpn5_b(0xae2c88).wrapping_add(fatias) as i8;
    while fatia > 2 {
        fatia -= 3;
        let r = somar(data.as_ptr() as usize, tmp.as_mut_ptr(), 1);
        core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    }
    let mut c = (desde(data.as_ptr() as usize, b0, b1) as u8).wrapping_add(cpn5_b(this + 0x84c));
    if c as i8 >= 0x32 {
        let q = (c as i8 as i32 as u32) / 50;
        c = c.wrapping_add((q as u8).wrapping_mul(0xce)); // − 50 × q
    }
    let balde = (c as i8 as i32).wrapping_mul(3).wrapping_add(fatia as i32);
    (data, fatia, this.wrapping_add(0x18).wrapping_add((balde as usize).wrapping_mul(14)))
}

/// { 0x8c6220 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::schedule_offer(oferta, dias, fatias)`:
/// sem data base (ano +0x84f zerado), ela vira hoje (0x417c10 de [0xae2c90] em +0x84d) e a fatia +0x855 a de hoje
/// ([0xae2c88]); `dias` ≥ 50: a caixa "Error" (linha 0x18b9) e 10 dias. A oferta ganha a data (+0x12) e a fatia (+0x1a)
/// do plano (`cpn5_plano`) e o +0x20 dela (4 bytes) entra no balde (0x68d0d0, que aloca); devolve o que a 0x68d0d0
/// devolve. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn agendar_oferta(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    crate::comparacao::ramo("cpn_5 agendar oferta");
    if cpn5_w(this + 0x84f) == 0 {
        let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
        let mut tmp = [0u8; 8];
        copiar(this + 0x84d, tmp.as_mut_ptr(), 0xae2c90);
        *((this + 0x855) as *mut u8) = cpn5_b(0xae2c88);
    }
    if dias as u8 as i8 >= 0x32 {
        aviso_em(0xa8d514, 0x18b9);
    }
    // ponytail: as datas são feitas no plano sobre cópias (0x54c5b0, 0x54cdb0 e 0x417c10 só leem e copiam) e gravadas
    // de uma vez; a original grava a data e a fatia a cada volta da fatia, com o mesmo fim
    let (data, fatia, balde) = cpn5_plano(this, dias as u8, fatias as u8);
    core::ptr::copy_nonoverlapping(data.as_ptr(), (oferta + 0x12) as *mut u8, 8);
    *((oferta + 0x1a) as *mut i8) = fatia;
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    acrescentar(balde, oferta + 0x20, 4)
}

/// Comparador: sem vaga a abrir (o plano pelo mesmo teste, com o cursor da lista devolvido), lado a lado pelo cursor
/// da lista; com vaga a abrir (ou a caixa), só a original.
unsafe extern "thiscall" fn comparar_abrir_vagas_que_faltam(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let l = cpn5_d(this + 8);
    let cursor = cpn5_d(l + 0xa);
    let mut eh_a = 0u32;
    let mut clube = cpn5_d(0xae23b4);
    let mut vazio = true;
    for _ in 0..(cpn5_d(0xae235c) as i32).max(0) {
        if cpn5_falta_vaga(this, clube, &mut eh_a) != Some(false) {
            vazio = false;
            break;
        }
        clube += 0x245;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    if !vazio {
        // ponytail: a 0x69ba50 abre a vaga (aloca, sorteia o candidato): não roda duas vezes; o modo Rust e a carreira provam
        return o(this);
    }
    let reg = [(l + 0xa, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || abrir_vagas_que_faltam(this));
    PLACAR.registrar(igual, || format!("abrir_vagas_que_faltam (nenhuma a abrir) {:x?} x {:x?}", mo, mr));
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x69d1b0 e a 0x69f810 podem sortear), pela nota e
/// pelo cursor da lista de vagas (a 0x69fc60 o devolve).
unsafe extern "thiscall" fn comparar_melhor_nota_de_candidato_a_tecnico(this: usize, clube: usize, atual: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I1%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((cpn5_d(this + 8) + 0xa, 4));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, atual), || crate::aleatorio::sem_contar(|| melhor_nota_de_candidato_a_tecnico(this, clube, atual)));
    PLACAR.registrar(igual && r == n, || format!("melhor_nota_de_candidato_a_tecnico(clube {}, atual {:x}) original={} rust={} {:x?} x {:x?}", cpn5_d(clube), atual, r, n, mo, mr));
    r
}

/// Comparador: a original roda (a 0x68d0d0 aloca o nó) e o resultado é conferido com o plano feito antes: a data
/// (+0x12) e a fatia (+0x1a) da oferta, a conta do balde mais 1 e a data base (+0x84d, 8 bytes) igual a hoje quando
/// não havia.
unsafe extern "thiscall" fn comparar_agendar_oferta(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    let sem_base = cpn5_w(this + 0x84f) == 0;
    let (data, fatia, balde) = cpn5_plano(this, dias as u8, fatias as u8);
    let conta = cpn5_w(balde);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I2%%)(this, oferta, dias, fatias);
    let gravada = core::slice::from_raw_parts((oferta + 0x12) as *const u8, 8);
    let base_ok = !sem_base || core::slice::from_raw_parts((this + 0x84d) as *const u8, 8) == core::slice::from_raw_parts(0xae2c90 as *const u8, 8);
    let ok = gravada == &data[..] && *((oferta + 0x1a) as *const i8) == fatia && cpn5_w(balde) == conta.wrapping_add(1) && base_ok;
    PLACAR.registrar(ok, || format!("agendar_oferta(dias {}, fatias {}): data {:x?} (plano {:x?}), fatia {} (plano {}), balde {:x} conta {} -> {}, base {}",
        dias as u8 as i8, fatias as u8 as i8, gravada, data, *((oferta + 0x1a) as *const i8), fatia, balde, conta, cpn5_w(balde), base_ok));
    r
}
