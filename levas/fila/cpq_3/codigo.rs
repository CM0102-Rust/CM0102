// ---------- %%LEVA%%a leva: os registros de carreira: o acesso, o rebaixamento e a classificação para a fase seguinte (staff_records.cpp) ----------

unsafe fn cpq3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpq3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpq3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpq3_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpq3_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpq3_somar(a: usize, v: i32) {
    cpq3_pi(a, cpq3_i(a).wrapping_add(v));
}

/// A constante double do exe em `a`.
unsafe fn cpq3_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Uma entrada de histórico na pilha: a data (0x54a110) com a data do jogo copiada (0x417c10).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
unsafe fn cpq3_entrada() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    nova(e.as_mut_ptr());
    copia(e.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    e
}

unsafe fn cpq3_por(e: &mut [u8; 0x40], o: usize, v: u32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// A entrada `e` no fim da lista do arquivo `arq` (FILE_LLIST, 0x58e340) da pessoa ou do clube `id` (o começo e o fim
/// em `base` + id × 8). Sem `gravar` (a cópia pelo comparador, depois da original), nada.
unsafe fn cpq3_historico(gravar: bool, arq: usize, base: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
    let p = base.wrapping_add(id.wrapping_mul(8));
    f(arq, p, p + 4, e.as_ptr());
}

/// O histórico da pessoa (o arquivo [this], as listas em [this+0xc]).
unsafe fn cpq3_da_pessoa(gravar: bool, this: usize, pessoa: usize, e: &[u8; 0x40]) {
    cpq3_historico(gravar, cpq3_d(this), cpq3_d(this + 0xc), cpq3_d(pessoa), e);
}

/// O histórico do clube (o arquivo [this+4], as listas em [this+0x10]) com o tipo e a competição.
unsafe fn cpq3_do_clube(gravar: bool, this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = cpq3_entrada();
    e[8] = tipo;
    cpq3_por(&mut e, 9, cpq3_d(comp) as u32);
    cpq3_historico(gravar, cpq3_d(this + 4), cpq3_d(this + 0x10), cpq3_d(clube), &e);
}

/// A entrada da pessoa: o clube (+8), o tipo (+0xc) e a competição (+0xd).
unsafe fn cpq3_entrada_do_clube(clube: usize, tipo: u8, comp: u32) -> [u8; 0x40] {
    let mut e = cpq3_entrada();
    cpq3_por(&mut e, 8, cpq3_d(clube) as u32);
    e[0xc] = tipo;
    cpq3_por(&mut e, 0xd, comp);
    e
}

/// Os jogadores do elenco (50 vagas em +0xd7) que contam nos registros: com registro de jogador (+0x61), id abaixo de
/// [0xae2364] e que não são o técnico.
unsafe fn cpq3_elenco(clube: usize) -> impl Iterator<Item = usize> {
    (0..50usize).filter_map(move |k| {
        let p = cpq3_d(clube + 0xd7 + 4 * k);
        if p == 0 || cpq3_d(p + 0x61) == 0 || cpq3_i(p) >= cpq3_i(0xae2364) || p == cpq3_d(clube + 0xcf) {
            None
        } else {
            Some(p)
        }
    })
}

unsafe fn cpq3_contrato(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cpq3_d(0xae1050), p, clube)
}

/// O contrato `c` do jogador `p` vai a renovação (a 0x4cda70 roda): com o gerente de transferências, o contrato com
/// mais de 300 dias (desde +0x25) e o jogador na lista do clube (0x849ce0, item +0xf = id dele). Só leitura.
unsafe fn cpq3_vai_renovar(clube: usize, p: usize, c: usize) -> bool {
    let gt = cpq3_d(0xde1f54);
    if gt == 0 {
        return false;
    }
    let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    if dias(0xae2c90, cpq3_d(c + 0x25), cpq3_d(c + 0x29)) <= 0x12c {
        return false;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let e = entrada(cpq3_d(gt + 0x856), clube, p, 0, 0);
    e != 0 && cpq3_d(e + 0xf) == cpq3_d(p)
}

/// A oferta de renovação do contrato (0x4cda70 do gerente de contratos: o jogador, sem clube nem termos, vaga −1,
/// fator 1,0, modo 1).
unsafe fn cpq3_renovar(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    f(cpq3_d(0xae1050), p, 0, 0, -1, 0, 0x3ff0_0000, 1);
}

unsafe fn cpq3_moral(p: usize, delta: i32) {
    let f: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(0x544710usize);
    f(p, delta, 1);
}

/// A nota do não jogador em `a` sobe ftol(x × peso × 10/61) e fica no máximo entre x × peso (16 bits) e ela mesma,
/// presa a 10000.
unsafe fn cpq3_nota_do_acesso(a: usize, x: i32, peso: i32) {
    cpq3_pw(a, cpq3_w(a).wrapping_add(ftol(x.wrapping_mul(peso) as f64 * cpq3_k(0x96ff88)) as i16));
    let minimo = (x as i16).wrapping_mul(peso as i16);
    let v = cpq3_w(a);
    cpq3_pw(a, if minimo > v { minimo } else if v > 0x2710 { 0x2710 } else { v });
}

/// O acesso; `gravar` falso só na cópia pelo comparador (sem os históricos em arquivo).
unsafe fn cpq3_acesso(this: usize, clube: usize, comp: usize, peso: u32, gravar: bool) {
    if clube == 0 || comp == 0 {
        aviso_em(0xa83b78, 0x301);
        return;
    }
    let e = cpq3_entrada_do_clube(clube, 8, cpq3_d(comp) as u32);
    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_add(500));
    if cpq3_w(clube + 0x80) > 0x2710 {
        cpq3_pw(clube + 0x80, 0x2710);
    }
    let v = cpq3_i(clube + 0x7b);
    if v < 0x249f0 && v < cpq3_i(clube + 0x73).wrapping_add(500) {
        cpq3_pi(clube + 0x7b, v.wrapping_add(cpq3_w(comp + 0x69) as i32 * 10));
    }
    if cpq3_d(clube + 0xcf) != 0 {
        let tec = cpq3_d(clube + 0xcf);
        if cpq3_d(tec + 0x69) != 0 {
            let objeto = cpq3_d(cpq3_d(0xadadfc).wrapping_add(cpq3_d(comp).wrapping_mul(4)));
            let mut pela = comp;
            if objeto != 0 && cpq3_i(objeto + 0x1c) > -1 {
                let cc = cpq3_d(0xae23d0).wrapping_add((cpq3_i(objeto + 0x1c) as usize).wrapping_mul(0x6b));
                pela = if cc != 0 { cc } else { comp };
            }
            let (p1, p2) = if cpq3_b(objeto + 0x42) as i8 != 1 {
                (400, 400)
            } else {
                let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
                (200, if estrangeira(tec, cpq3_d(clube + 0x53), 1) != 0 { 200 } else { 400 })
            };
            let x = cpq3_w(pela + 0x69) as i32;
            let nj = || cpq3_d(cpq3_d(clube + 0xcf) + 0x69);
            crate::comparacao::ramo("cpq_3 acesso do tecnico");
            cpq3_nota_do_acesso(nj() + 0xa, x, 400);
            cpq3_nota_do_acesso(nj() + 8, x, p2);
            cpq3_nota_do_acesso(nj() + 0xc, x, p1);
        }
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xcf), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, cpq3_d(clube + 0xcf), 1);
        if r != 0 {
            let pontos = (cpq3_w(comp + 0x69) as i32).wrapping_mul(peso as u8 as i8 as i32).wrapping_mul(2500) / 100;
            cpq3_somar(r + 0xb6, pontos);
            cpq3_somar(r + 0xba, pontos);
            cpq3_somar(r + 0xca, pontos);
        }
    }
    if cpq3_d(clube + 0xbf) != 0 {
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xbf), &e);
    }
    for p in cpq3_elenco(clube) {
        cpq3_moral(p, 0xf);
        let c = cpq3_contrato(p, clube);
        if c == 0 {
            continue;
        }
        let v = cpq3_b(c + 0x44) as i8;
        let s = v as i32 + 0x4b;
        *((c + 0x44) as *mut u8) = if s > 0x64 { 0x64 } else if s < -0x64 { 0x9c } else { v.wrapping_add(0x4b) as u8 };
        let tipo = cpq3_b(c + 0x4f) >> 4;
        if (tipo == 0 || (tipo > 3 && tipo != 5)) || !cpq3_vai_renovar(clube, p, c) {
            continue;
        }
        crate::comparacao::ramo("cpq_3 acesso renova");
        cpq3_renovar(p);
    }
    cpq3_do_clube(gravar, this, clube, 8, comp);
}

/// { 0x869480 } (thiscall, ret 0xc, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_promotion_record(clube,
/// comp, peso)`: a reputação do clube (+0x80) sobe 500 (até 10000) e +0x7b sobe 10 × a importância da competição
/// (+0x69) se está abaixo de 150000 e de +0x73 + 500. O técnico (+0xcf) com não jogador: as notas +0xa, +8 e +0xc sobem
/// pela importância da competição (a do registro de competição de clube +0x1c dela, se há) × 400, × 200/400 (estrangeiro
/// na liga com +0x42 = 1, 0x53d980) e × 200/400 (liga com +0x42 = 1) × 10/61, com o mínimo de importância × peso. O
/// histórico do técnico e do presidente (tipo 8), os pontos do técnico (0x86f5b0: +0xb6, +0xba, +0xca somam importância
/// × peso × 25), o moral +15 de cada jogador do elenco (0x544710), o vínculo do contrato (+0x44) +75 (−100..100) e a
/// renovação (0x4cda70) dos contratos dos tipos 1, 2, 3 e 5 que vencem; por fim o histórico do clube (tipo 8). Clube ou
/// competição nulos: a caixa "Error" (linha 0x301).
unsafe extern "thiscall" fn add_promotion_record(this: usize, clube: usize, comp: usize, peso: u32) {
    crate::comparacao::ramo("cpq_3 acesso");
    cpq3_acesso(this, clube, comp, peso, true)
}

/// As regiões que o acesso ou o rebaixamento mexem (sem os históricos): o estado do sorteador (o moral sorteia), o clube,
/// o não jogador do técnico, o registro de pontos dele (criado antes, como a original faria), o moral (+0x45) e as
/// reputações (+9..+0xe) de cada jogador do elenco e o vínculo (+0x44) do contrato dele. `None` se algum contrato vai a
/// renovação (a 0x4cda70 faz a oferta: não roda duas vezes; no acesso os tipos 1, 2, 3 e 5, no rebaixamento 1 e 2).
unsafe fn cpq3_regioes(this: usize, clube: usize, acesso: bool) -> Option<Vec<(usize, usize)>> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube, 0x245));
    let tec = cpq3_d(clube + 0xcf);
    if tec != 0 {
        if cpq3_d(tec + 0x69) != 0 {
            reg.push((cpq3_d(tec + 0x69), 0x10));
        }
        if acesso {
            let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
            let r = registro(this, tec, 1);
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    for p in cpq3_elenco(clube) {
        let j = cpq3_d(p + 0x61);
        reg.extend([(j + 9, 6), (j + 0x45, 1)]);
        let c = cpq3_contrato(p, clube);
        if c != 0 {
            let tipo = cpq3_b(c + 0x4f) >> 4;
            let renova = if acesso { tipo != 0 && (tipo <= 3 || tipo == 5) } else { tipo == 1 || tipo == 2 };
            if renova && cpq3_vai_renovar(clube, p, c) {
                return None;
            }
            reg.push((c + 0x44, 1));
        }
    }
    Some(reg)
}

/// Comparador: lado a lado pelas regiões (a cópia sem os históricos em arquivo, que a original já gravou, e sem contar
/// os sorteios); com renovação de contrato ou clube/competição nulos, só a original.
unsafe extern "thiscall" fn comparar_add_promotion_record(this: usize, clube: usize, comp: usize, peso: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%);
    let reg = if clube != 0 && comp != 0 { cpq3_regioes(this, clube, true) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp, peso),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp, peso),
        || crate::aleatorio::sem_contar(|| cpq3_acesso(this, clube, comp, peso, false)));
    PLACAR.registrar(igual, || format!("add_promotion_record(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}

/// A nota do não jogador em `a` soma `v` (16 bits) e fica presa a 1..10000.
unsafe fn cpq3_nota_do_rebaixamento(a: usize, v: i16) {
    cpq3_pw(a, cpq3_w(a).wrapping_add(v));
    let n = cpq3_w(a);
    cpq3_pw(a, if n < 1 { 1 } else if n > 0x2710 { 0x2710 } else { n });
}

/// As reputações (+0xb atual, +9 na nação, +0xd mundial) do registro de jogador `j` × `k`.
unsafe fn cpq3_reputacoes(j: usize, k: f64) {
    for o in [0xb, 9, 0xd] {
        cpq3_pw(j + o, ftol(cpq3_w(j + o) as i32 as f64 * k) as i16);
    }
}

unsafe fn cpq3_rebaixamento(this: usize, clube: usize, comp: usize, gravar: bool) {
    if clube == 0 || comp == 0 {
        aviso_em(0xa83b78, 0x476);
        return;
    }
    let e = cpq3_entrada_do_clube(clube, 9, cpq3_d(comp) as u32);
    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_sub(500));
    if cpq3_w(clube + 0x80) < 0x2ee {
        cpq3_pw(clube + 0x80, 0x2ee);
    }
    if cpq3_d(clube + 0xcf) != 0 {
        let tec = cpq3_d(clube + 0xcf);
        if cpq3_d(tec + 0x69) != 0 {
            let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
            let fator: i16 = if estrangeira(tec, cpq3_d(clube + 0x53), 1) != 0 { -8 } else { -16 };
            let nj = || cpq3_d(cpq3_d(clube + 0xcf) + 0x69);
            let x = || cpq3_w(comp + 0x69);
            crate::comparacao::ramo("cpq_3 rebaixamento do tecnico");
            cpq3_nota_do_rebaixamento(nj() + 0xa, x().wrapping_mul(-16));
            cpq3_nota_do_rebaixamento(nj() + 8, x().wrapping_mul(fator));
            cpq3_nota_do_rebaixamento(nj() + 0xc, x().wrapping_mul(-8));
        }
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xcf), &e);
    }
    if cpq3_d(clube + 0xbf) != 0 {
        cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xbf), &e);
    }
    for p in cpq3_elenco(clube) {
        cpq3_moral(p, -0xf);
        let c = cpq3_contrato(p, clube);
        if c == 0 {
            continue;
        }
        let j = cpq3_d(p + 0x61);
        match cpq3_b(c + 0x4f) >> 4 {
            1 | 2 => {
                if j != 0 {
                    cpq3_reputacoes(j, cpq3_k(0x9689d8)); // 0,9
                }
                if cpq3_vai_renovar(clube, p, c) {
                    crate::comparacao::ramo("cpq_3 rebaixamento renova");
                    cpq3_renovar(p);
                }
            }
            3..=5 => {
                if j != 0 {
                    cpq3_reputacoes(j, cpq3_k(0x9689d0)); // 0,95
                }
            }
            _ => {}
        }
    }
    cpq3_do_clube(gravar, this, clube, 9, comp);
}

/// { 0x86a1c0 } (thiscall, ret 8, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_relegation_record(clube,
/// comp)`: a reputação do clube (+0x80) cai 500 (mínimo 750). O técnico com não jogador: as notas +0xa (−16), +8 (−8
/// estrangeiro, 0x53d980, ou −16) e +0xc (−8) × a importância da competição (+0x69), presas a 1..10000; o histórico do
/// técnico e do presidente (tipo 9). Cada jogador do elenco: moral −15 (0x544710) e, pelo tipo do contrato (+0x4f >> 4):
/// 1 e 2, reputações × 0,9 e a renovação (0x4cda70) se vence; 3 a 5, reputações × 0,95. Por fim o histórico do clube
/// (tipo 9). Clube ou competição nulos: a caixa "Error" (linha 0x476).
unsafe extern "thiscall" fn add_relegation_record(this: usize, clube: usize, comp: usize) {
    crate::comparacao::ramo("cpq_3 rebaixamento");
    cpq3_rebaixamento(this, clube, comp, true)
}

/// Comparador: como o do acesso (sem o registro de pontos, que o rebaixamento não usa).
unsafe extern "thiscall" fn comparar_add_relegation_record(this: usize, clube: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let reg = if clube != 0 && comp != 0 { cpq3_regioes(this, clube, false) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp),
        || crate::aleatorio::sem_contar(|| cpq3_rebaixamento(this, clube, comp, false)));
    PLACAR.registrar(igual, || format!("add_relegation_record(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

unsafe fn cpq3_classificacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32, gravar: bool) {
    if clube == 0 {
        aviso_em(0xa83b78, 0x703);
        return;
    }
    let mut e = cpq3_entrada_do_clube(clube, 3, if comp != 0 { cpq3_d(comp) as u32 } else { u32::MAX });
    cpq3_por(&mut e, 0x11, fase as u16 as i16 as i32 as u32);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0870usize);
    cpq3_por(&mut e, 0x15, base(rodada) as u16 as i16 as i32 as u32);
    if cpq3_d(clube + 0xcf) == 0 {
        return;
    }
    cpq3_da_pessoa(gravar, this, cpq3_d(clube + 0xcf), &e);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let r = registro(this, cpq3_d(clube + 0xcf), 1);
    if r == 0 {
        return;
    }
    // ponytail: a original faz pow(x, 2,0) (0x945ff0): x × x, exato para a importância (16 bits)
    let x = cpq3_w(comp + 0x69) as f64;
    let pontos = ftol(x * x * (peso as u8 as i8 as i32) as f64);
    cpq3_somar(r + 0xb6, pontos);
    cpq3_somar(r + 0xca, pontos);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        crate::comparacao::ramo("cpq_3 classificado selecao");
        cpq3_somar(r + 0xbe, pontos);
    } else {
        crate::comparacao::ramo("cpq_3 classificado clube");
        cpq3_somar(r + 0xba, pontos);
    }
}

/// { 0x86bdd0 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_qualified_for_round_record(
/// clube, comp, fase, rodada, peso)`: com técnico (+0xcf), o histórico dele (tipo 3: clube, competição ou −1, a fase com
/// sinal e a rodada base 0x4b0870) e os pontos (0x86f5b0): importância² × peso somados em +0xb6, +0xca e, na seleção
/// (0x53b900), +0xbe, ou no clube +0xba. Clube nulo: a caixa "Error" (linha 0x703).
unsafe extern "thiscall" fn add_qualified_for_round_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    cpq3_classificacao(this, clube, comp, fase, rodada, peso, true)
}

/// Comparador: lado a lado pelo registro de pontos do técnico (criado antes, como a original faria), a cópia sem o
/// histórico em arquivo; sem clube ou sem técnico, só a original.
unsafe extern "thiscall" fn comparar_add_qualified_for_round_record(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I2%%);
    if clube == 0 || cpq3_d(clube + 0xcf) == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let r = registro(this, cpq3_d(clube + 0xcf), 1);
    if r == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, clube, comp, fase, rodada, peso),
        || cpq3_classificacao(this, clube, comp, fase, rodada, peso, false));
    PLACAR.registrar(igual, || format!("add_qualified_for_round_record(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}
