// ---------- 242a leva: as seleções: o fim das convocações de uma competição e a poda dos registros de convocação (national_teams.cpp) ----------

unsafe fn cpd1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Os itens da lista `l` (0x68d0d0: começo em +2, nó = item +0 e seguinte +6), sem mexer no cursor (+0xa).
unsafe fn cpd1_itens(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpd1_d(l + 2);
    while no != 0 && v.len() < 100_000 {
        v.push(cpd1_d(no));
        no = cpd1_d(no + 6);
    }
    v
}

/// O fim das convocações da lista [this] (as duas rotinas da 0x774f30/0x774f90): do começo (cursor +0xa = +2),
/// cada registro que `passa` recebe em +0x10 a palavra +0 (o dia) de amanhã (0x54c5b0 com 1 sobre [0xae2c90]).
unsafe fn cpd1_encerrar(this: usize, passa: impl Fn(usize) -> bool) {
    let l = cpd1_d(this);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpd1_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> *const u16 = core::mem::transmute(0x54c5b0usize);
    let mut r = proximo(cpd1_d(this));
    while r != 0 {
        if passa(r) {
            let mut data = [0u8; 8];
            let dia = *mais_dias(0xae2c90, data.as_mut_ptr(), 1);
            core::ptr::write_unaligned((r + 0x10) as *mut u16, dia); // o dia de amanhã
        }
        r = proximo(cpd1_d(this));
    }
}

/// { 0x774f30 } (thiscall, ret 4, national_teams.cpp): `end_duty_for_competition(comp)`: na lista de registros
/// de convocação [this], os da competição (+4 igual a *comp) terminam amanhã (+0x10). Sem SEH, sem caixa, sem
/// sorteio; a local é a data que a 0x54c5b0 grava.
unsafe extern "thiscall" fn encerrar_convocacoes_da_competicao(this: usize, comp: usize) {
    cpd1_encerrar(this, |r| cpd1_d(r + 4) == cpd1_d(comp) /* a competição */)
}

/// { 0x774f90 } (thiscall, ret 8, national_teams.cpp): `end_duty_for_nation_in_competition(nacao, comp)`: como a
/// 0x774f30, só os registros da competição (+4) e da nação (+0 igual a *nacao).
unsafe extern "thiscall" fn encerrar_convocacoes_da_nacao(this: usize, nacao: usize, comp: usize) {
    cpd1_encerrar(this, |r| cpd1_d(r + 4) == cpd1_d(comp) && cpd1_d(r) == cpd1_d(nacao) /* a nação */)
}

/// As regiões do fim das convocações: o cursor da lista e o +0x10 de cada registro.
unsafe fn cpd1_regioes_encerrar(this: usize) -> Vec<(usize, usize)> {
    let l = cpd1_d(this);
    if l == 0 {
        return Vec::new();
    }
    let mut reg = vec![(l + 0xa, 4)];
    reg.extend(cpd1_itens(l).into_iter().filter(|&r| r != 0).map(|r| (r + 0x10, 2)));
    reg
}

unsafe extern "thiscall" fn comparar_encerrar_convocacoes_da_competicao(this: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(788);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, comp), || encerrar_convocacoes_da_competicao(this, comp));
    PLACAR.registrar(igual, || format!("encerrar_convocacoes_da_competicao({:x}, {:x}) {:x?} x {:x?}", this, comp, mo, mr));
}

unsafe extern "thiscall" fn comparar_encerrar_convocacoes_da_nacao(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(789);
    let reg = cpd1_regioes_encerrar(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao, comp), || encerrar_convocacoes_da_nacao(this, nacao, comp));
    PLACAR.registrar(igual, || format!("encerrar_convocacoes_da_nacao({:x}, {:x}, {:x}) {:x?} x {:x?}", this, nacao, comp, mo, mr));
}

/// O número de registros de convocação por clube da 0x7774b0: [0xae235c] − 2 × [0xae2350].
unsafe fn cpd1_quantos() -> i32 {
    (*(0xae235c as *const i32)).wrapping_sub((*(0xae2350 as *const i32)).wrapping_mul(2))
}

/// { 0x7774b0 } (thiscall, national_teams.cpp): `prune_club_call_up_records()`: cada registro de clube (+0x20,
/// 6 bytes: +0 vetor de entradas de 9 bytes, +4 marca, +5 quantas, byte com sinal) conta as entradas já vencidas
/// (0x456770: a data delas antes de [0xae2c90]; a rotina supõe que ficam no começo), desce as outras por cima
/// (0x777590) e diminui a contagem; zerada com vetor, solta o vetor (0x9452ca) e zera +4 e +0. Sem SEH, sem caixa,
/// sem sorteio; a local (o vencido) é zerada antes de ler.
unsafe extern "thiscall" fn podar_registros_de_convocacao(this: usize) {
    let n = cpd1_quantos();
    let mut rec = cpd1_d(this + 0x20);
    if n <= 0 {
        return;
    }
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x777590usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let conta = |rec: usize| *((rec + 5) as *const i8);
    for _ in 0..n {
        let mut vencidas: i8 = 0;
        if conta(rec) > 0 {
            let mut k: i8 = 0;
            loop {
                let e = cpd1_d(rec).wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                if antes(e, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0 {
                    vencidas = vencidas.wrapping_add(1); // uma vencida
                }
                k = k.wrapping_add(1);
                if k >= conta(rec) {
                    break;
                }
            }
        }
        if vencidas < conta(rec) {
            let mut k = vencidas;
            loop {
                crate::comparacao::ramo("convocações descer as que ficam");
                let v = cpd1_d(rec);
                let de = v.wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                let para = v.wrapping_add(9usize.wrapping_mul((k as i32 - vencidas as i32) as usize));
                copiar(para, de);
                k = k.wrapping_add(1);
                if k >= conta(rec) {
                    break;
                }
            }
        }
        let resto = conta(rec).wrapping_sub(vencidas);
        *((rec + 5) as *mut i8) = resto;
        if resto == 0 && cpd1_d(rec) != 0 {
            crate::comparacao::ramo("convocações soltar o vetor");
            free(cpd1_d(rec));
            *((rec + 4) as *mut u8) = 0;
            core::ptr::write_unaligned(rec as *mut usize, 0);
        }
        rec += 6;
    }
}

/// O registro `rec` da 0x7774b0 terminaria sem entradas e com vetor (a original solta o vetor)? Escrito à parte da
/// cópia, para o comparador.
unsafe fn cpd1_soltaria(rec: usize) -> bool {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let n = *((rec + 5) as *const i8);
    let v = cpd1_d(rec);
    let vencidas = (0..n.max(0) as usize).filter(|&k| antes(v + 9 * k, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) != 0).count();
    v != 0 && n.wrapping_sub(vencidas as i8) == 0
}

/// Comparador: os registros (6 bytes cada) e os vetores de entradas. Algum vetor a soltar (0x9452ca; soltar duas
/// vezes): só a original.
unsafe extern "thiscall" fn comparar_podar_registros_de_convocacao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(790);
    let n = cpd1_quantos().max(0) as usize;
    let recs = cpd1_d(this + 0x20);
    if n == 0 || recs == 0 || (0..n).any(|i| cpd1_soltaria(recs + 6 * i)) {
        return o(this);
    }
    let mut reg = vec![(recs, 6 * n)];
    for i in 0..n {
        let (v, k) = (cpd1_d(recs + 6 * i), *((recs + 6 * i + 5) as *const i8));
        if v != 0 && k > 0 {
            reg.push((v, 9 * k as usize));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || podar_registros_de_convocacao(this));
    PLACAR.registrar(igual, || format!("podar_registros_de_convocacao({:x}) {:x?} x {:x?}", this, mo, mr));
}

// ---------- 242a leva: as seleções: pôr e tirar do elenco e dispensar os acima da idade do sub-21 (national_teams.cpp) ----------

unsafe fn cpd2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O time da nação para o tipo de convocação: 0 e 1 a seleção principal (0x53b3d0), os outros o sub-21 (0x53b4f0).
unsafe fn cpd2_time(nacao: usize, tipo: u8) -> usize {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    if tipo == 0 || tipo == 1 { principal(nacao) } else { sub21(nacao) }
}

/// Tira a pessoa `p` da tática do time (0x896c60 de [0xde1d08], com 1 e com 0 no 3º argumento), como as originais
/// fazem antes de esvaziar a vaga.
unsafe fn cpd2_tirar_da_tatica(time: usize, p: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    marcar(*(0xde1d08 as *const usize), time, p, 1, 1);
    marcar(*(0xde1d08 as *const usize), time, p, 0, 1);
}

/// A pessoa sai da seleção: sem time (+0x24), sem a marca +0x28 e só com os bits de clube (& 3) em +0x5e.
unsafe fn cpd2_limpar_pessoa(p: usize) {
    core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    *((p + 0x28) as *mut u8) = 0; // a marca de convocado
    *((p + 0x5e) as *mut u8) &= 3;
}

/// { 0x776250 } (thiscall, ret 0xc, national_teams.cpp): `add_player_to_squad(nacao, tipo, p)`: a primeira vaga
/// livre das 50 do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe `p`; sem vaga, nada. A pessoa ganha o
/// time (+0x24), a data de hoje em +0x29 (0x417c10 de [0xae2c90]), a marca 0xb em +0x28 e o elenco (0x540980) 4
/// (tipo 0 e 2) ou 8. Sem SEH, sem caixa, sem sorteio; a local é a cópia da data que a 0x417c10 devolve.
unsafe extern "thiscall" fn por_no_elenco_da_selecao(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let tipo = tipo as u8;
    let time = cpd2_time(nacao, tipo);
    let elenco: u32 = if tipo == 0 || tipo == 2 { 4 } else { 8 };
    let vaga = match (0..50usize).find(|&c| cpd2_d(time + 0xd7 + 4 * c) == 0) {
        Some(c) => time + 0xd7 + 4 * c,
        None => return,
    };
    core::ptr::write_unaligned(vaga as *mut usize, p); // a vaga
    core::ptr::write_unaligned((p + 0x24) as *mut usize, time);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut data = [0u8; 8];
    copia(p + 0x29, data.as_mut_ptr(), 0xae2c90);
    *((p + 0x28) as *mut u8) = 0xb;
    let marcar_elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    marcar_elenco(p, elenco);
}

/// As regiões de pôr e tirar do elenco: as 50 vagas do time e a pessoa (+0x24 a +0x5e).
unsafe fn cpd2_regioes(time: usize, p: usize) -> [(usize, usize); 2] {
    [(time + 0xd7, 200), (p + 0x24, 0x3b)]
}

unsafe extern "thiscall" fn comparar_por_no_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(791);
    let time = cpd2_time(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = cpd2_regioes(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || por_no_elenco_da_selecao(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("por_no_elenco_da_selecao({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// { 0x7762f0 } (thiscall, ret 0xc, national_teams.cpp): `remove_player_from_squad(nacao, tipo, p)`: tira `p` da
/// tática do time (0x896c60, duas vezes) e, se ele está numa das 50 vagas (+0xd7), esvazia a vaga e limpa a
/// pessoa (+0x24, +0x28, +0x5e & 3). Sem SEH, sem sorteio, sem locais; caixa só com time ou pessoa nulos (0x896c60).
unsafe extern "thiscall" fn tirar_do_elenco_da_selecao(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let time = cpd2_time(nacao, tipo as u8);
    cpd2_tirar_da_tatica(time, p);
    if let Some(c) = (0..50usize).find(|&c| cpd2_d(time + 0xd7 + 4 * c) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * c) as *mut usize, 0);
        cpd2_limpar_pessoa(p);
    }
}

/// Comparador: as vagas e a pessoa. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha e não
/// muda nada, então a tática fica fora das regiões. Time ou pessoa nulos (a caixa da 0x896c60): só a original.
unsafe extern "thiscall" fn comparar_tirar_do_elenco_da_selecao(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(792);
    let time = cpd2_time(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = cpd2_regioes(time, p);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao, tipo, p), || tirar_do_elenco_da_selecao(this, nacao, tipo, p));
    PLACAR.registrar(igual, || format!("tirar_do_elenco_da_selecao({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id (+0 da nação).
unsafe fn cpd2_registro(this: usize, nacao: usize) -> usize {
    cpd2_d(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize)
}

/// { 0x76d880 } (thiscall, ret 4, national_teams.cpp): `release_overage_u21_players(nacao)`: depois da data do
/// registro da nação (+0x62/+0x66; 0x456740 sobre [0xae2c90]), cada uma das 50 vagas do sub-21 (0x53b4f0) com
/// pessoa de idade (+0x18, byte com sinal) acima de 21 sai da tática (0x896c60, duas vezes), é limpa (+0x24, +0x28,
/// +0x5e & 3) e a vaga esvazia; por fim zera o byte +0x6b do registro. Sem SEH, sem sorteio; a local é o `this`.
unsafe extern "thiscall" fn dispensar_acima_da_idade_do_sub21(this: usize, nacao: usize) {
    let rec = cpd2_registro(this, nacao);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    if depois(0xae2c90, cpd2_d(rec + 0x62) as u32, cpd2_d(rec + 0x66) as u32) == 0 {
        return;
    }
    crate::comparacao::ramo("sub-21 dispensar acima da idade");
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let time = sub21(nacao);
    for c in 0..50usize {
        let vaga = time + 0xd7 + 4 * c;
        let p = cpd2_d(vaga);
        if p != 0 && *((p + 0x18) as *const i8) > 0x15 { // acima de 21 anos
            cpd2_tirar_da_tatica(time, p);
            cpd2_limpar_pessoa(p);
            core::ptr::write_unaligned(vaga as *mut usize, 0);
        }
    }
    *((cpd2_registro(this, nacao) + 0x6b) as *mut u8) = 0;
}

/// Comparador: o byte +0x6b do registro, as vagas do sub-21 e as pessoas delas (+0x24 a +0x5e). A 0x896c60 da
/// cópia não acha mais as pessoas que a original já tirou (fica fora das regiões, como na 0x7762f0). Sub-21 nulo:
/// só a original.
unsafe extern "thiscall" fn comparar_dispensar_acima_da_idade_do_sub21(this: usize, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(793);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let time = sub21(nacao);
    if time == 0 {
        return o(this, nacao);
    }
    let mut reg = vec![(cpd2_registro(this, nacao) + 0x6b, 1), (time + 0xd7, 200)];
    reg.extend((0..50usize).map(|c| cpd2_d(time + 0xd7 + 4 * c)).filter(|&p| p != 0).map(|p| (p + 0x24, 0x3b)));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, nacao), || dispensar_acima_da_idade_do_sub21(this, nacao));
    PLACAR.registrar(igual, || format!("dispensar_acima_da_idade_do_sub21({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

// ---------- 242a leva: as seleções: a competição atual de cada nação e a convocação do substituto (national_teams.cpp) ----------

unsafe fn cpd3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpd3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Avisa o gerenciador dos técnicos ([0xb67a44], se existe) da seleção principal da nação (0x6a3770 com 1).
unsafe fn cpd3_avisar_tecnicos(nacao: usize) {
    let g = *(0xb67a44 as *const usize);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
        avisar(g, principal(nacao), 1);
    }
}

/// { 0x775000 } (thiscall, ret 4, national_teams.cpp): `clear_current_competition(nacao)`: a competição atual do
/// registro da nação ([this+4] + 0x6e × id, +4) vira −1; havendo a seguinte ([this+8][id] ≠ −1), ela passa a ser a
/// atual, a seguinte vira −1 e o gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
unsafe extern "thiscall" fn limpar_competicao_atual(this: usize, nacao: usize) {
    let id = *(nacao as *const i32);
    let atual = cpd3_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4;
    core::ptr::write_unaligned(atual as *mut i32, -1);
    let seguinte = cpd3_d(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    let v = *(seguinte as *const i32);
    if v != -1 {
        crate::comparacao::ramo("seleção a competição seguinte");
        core::ptr::write_unaligned(atual as *mut i32, v); // a seguinte vira a atual
        *(seguinte as *mut i32) = -1;
        cpd3_avisar_tecnicos(nacao); // o aviso da competição seguinte
    }
}

/// Comparador: a competição atual e a seguinte; com a seguinte (o aviso 0x6a3770 sorteia e mexe nos técnicos), só a
/// original.
unsafe extern "thiscall" fn comparar_limpar_competicao_atual(this: usize, nacao: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(794);
    let id = *(nacao as *const i32);
    let seguinte = cpd3_d(this + 8).wrapping_add((id as usize).wrapping_mul(4));
    if *(seguinte as *const i32) != -1 {
        return o(this, nacao);
    }
    let reg = [(cpd3_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4, 4), (seguinte, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao), || limpar_competicao_atual(this, nacao));
    PLACAR.registrar(igual, || format!("limpar_competicao_atual({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}

/// A competição `v` (−1: nenhuma) acabou para a 0x775070? Sem `fim_de_fase`, quando +0x59 da competição
/// ([0xae23d0] + 107 × v) é nulo; com ele, quando é a fase [0xae23a4] + 0xc6 × [0x9cfa10].
unsafe fn cpd3_acabou(v: i32, fim_de_fase: bool) -> bool {
    let c = cpd3_d((*(0xae23d0 as *const usize)).wrapping_add(v.wrapping_mul(107) as usize) + 0x59);
    if fim_de_fase {
        c == (*(0xae23a4 as *const usize)).wrapping_add((*(0x9cfa10 as *const i32)).wrapping_mul(0xc6) as usize)
    } else {
        c == 0
    }
}

/// { 0x775070 } (thiscall, ret 8, national_teams.cpp): `release_competitions_of_continent(continente, fim_de_fase)`:
/// a cada nação ([0xae23a8], 0x122 bytes) do continente (+0x71), a competição atual (registro +4) e a seguinte
/// ([this+8][i]) que acabaram (cpd3_acabou) viram −1; se a atual ficou −1 e há seguinte, a seguinte passa a atual e o
/// gerenciador dos técnicos é avisado (0x6a3770). Sem SEH, sem caixa, sem locais.
unsafe extern "thiscall" fn liberar_competicoes_do_continente(this: usize, continente: usize, fim_de_fase: u32) {
    let fim = fim_de_fase as u8 != 0;
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        let nacao = (*(0xae23a8 as *const usize)).wrapping_add(0x122 * i as usize);
        if cpd3_d(nacao + 0x71) == continente {
            let atual = cpd3_d(this + 4) + 0x6e * i as usize + 4;
            let v = *(atual as *const i32);
            if v != -1 && cpd3_acabou(v, fim) {
                *(atual as *mut i32) = -1; // a atual acabou
            }
            let seguinte = cpd3_d(this + 8) + 4 * i as usize;
            let v = *(seguinte as *const i32);
            if v != -1 && cpd3_acabou(v, fim) {
                *(seguinte as *mut i32) = -1;
            }
            let v = *((cpd3_d(this + 8) + 4 * i as usize) as *const i32);
            if v != -1 && *((cpd3_d(this + 4) + 0x6e * i as usize + 4) as *const i32) == -1 {
                crate::comparacao::ramo("seleção continente a seguinte vira a atual");
                *((cpd3_d(this + 4) + 0x6e * i as usize + 4) as *mut i32) = v;
                *((cpd3_d(this + 8) + 4 * i as usize) as *mut i32) = -1;
                cpd3_avisar_tecnicos(nacao); // o aviso do continente
            }
        }
        i += 1;
    }
}

/// Comparador: o aviso 0x6a3770 sorteia e mexe nos técnicos (não pode rodar duas vezes): só a original; a prova é a
/// carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (4 chamadas na carreira); comparar pedia separar a decisão do aviso.
unsafe extern "thiscall" fn comparar_liberar_competicoes_do_continente(this: usize, continente: usize, fim_de_fase: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(795)(this, continente, fim_de_fase)
}

/// { 0x775f90 } (thiscall, ret 0x10, national_teams.cpp): `call_up_replacement(nacao, tipo, nivel, saindo)`: no
/// trecho do nível (0x76d810) na reserva do tipo ([this+0xc], sub-21 [this+0x10]; começo +8/+0x1d do registro mais
/// as vagas dos níveis anteriores, quantos +0xd/+0x22), o primeiro que não é `saindo`, tem ficha (+0x61), gosta do
/// clube (0x547d30), está bem (0x62c400), não é regen marcado (0x7a7eb0) fora da principal, é da nação (time +0x24,
/// nacionalidades +0x1a/+0x1e, +0x22, bit 2 do registro de 79 bytes), não está suspenso (0x55d270) e joga numa das
/// posições de `saindo` (0x5b7d20). Sem time vai direto; convocado (+0x28 = 0xb) para outro tipo (2 com 0x5409b0,
/// senão o bit 3 de +0x5e) só depois da data do tipo (+0x3a; 0x456770) e, sendo o 0, se o clube deixa (0x600a50):
/// sai de lá (0x7762f0). Entra (0x776250), a vaga do tipo (+0x45) desce 1 e sai a notícia (0x76fd10). Ninguém: zera
/// +0x45. Sem SEH, sem sorteio; a local do tipo do outro elenco é gravada antes de ler (só o byte vale).
unsafe extern "thiscall" fn convocar_substituto(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    let posicoes: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let regen: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x7762f0usize);
    let por: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x776250usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x76fd10usize);
    let mascara = posicoes(cpd3_d(saindo + 0x61)) as u8;
    let t = tipo as u8 as i8;
    let nivel = nivel as u8 as i8;
    let rec = cpd3_d(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize);
    let sub = (t == 2) as usize;
    let o = 0x15 * sub;
    let mut inicio = cpd3_d(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        inicio = inicio.wrapping_add(cpd3_b(rec + o + 0xe + 4 * j) as i32);
    }
    let fim = inicio.wrapping_add(cpd3_b(rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize) + 0xd) as i32);
    let reserva = cpd3_d(this + 4 * sub + 0xc);
    let vaga = rec.wrapping_add((20 * t as i32) as usize) + 0x45;
    let mut k = inicio;
    while k < fim {
        let q = cpd3_d(reserva.wrapping_add((4 * k) as usize));
        k += 1;
        if q == saindo || cpd3_d(q + 0x61) == 0 || nao_gosta(q, principal(cpd3_d(q + 0x1a))) != 0 {
            continue;
        }
        if estado(*(0xb63d28 as *const usize), q) != 0 {
            continue;
        }
        if *(0xb7434c as *const usize) != 0 && regen(q) != 0 && t != 0 {
            continue;
        }
        let time = cpd3_d(q + 0x24);
        if time != 0 && cpd3_d(time + 0x53) != nacao {
            continue;
        }
        if cpd3_d(q + 0x1a) != nacao {
            let registro = (*(0xae2d0c as *const usize)).wrapping_add((*(q as *const i32)).wrapping_mul(0x4f) as usize);
            if time != 0 || cpd3_b(q + 0x22) != 0 || cpd3_b(registro + 0xc) & 2 != 0 || cpd3_d(q + 0x1e) != nacao {
                continue;
            }
        }
        let comp = (*(0xae23d0 as *const usize)).wrapping_add((*(0x9cf79c as *const i32)).wrapping_mul(107) as usize);
        if suspenso(*(0xae28f0 as *const usize), q, comp, 1, 0) != 0 {
            continue;
        }
        if posicoes(cpd3_d(q + 0x61)) as u8 & mascara & 0x7f == 0 { // as posições de quem sai
            continue;
        }
        let time = cpd3_d(q + 0x24);
        if time != 0 {
            if cpd3_b(q + 0x28) != 0xb {
                continue;
            }
            let outro: i8 = if segunda(time) != 0 { 2 } else { ((cpd3_b(q + 0x5e) >> 3) & 1) as i8 };
            if outro == t {
                continue;
            }
            let data = rec.wrapping_add((20 * outro as i32) as usize) + 0x3a;
            if antes(data, *(0xae2c90 as *const u32), *(0xae2c94 as *const u32)) == 0 {
                continue;
            }
            if outro == 0 && humano(time, 0, 1, 0, 0) == 0 {
                continue;
            }
            crate::comparacao::ramo("substituto sai do outro elenco");
            tirar(this, nacao, outro as u8 as u32, q);
        }
        crate::comparacao::ramo("substituto convocado");
        por(this, nacao, tipo, q);
        *(vaga as *mut u8) = cpd3_b(vaga).wrapping_sub(1); // uma vaga a menos
        noticia(this, q, nacao, tipo);
        return;
    }
    *(vaga as *mut u8) = 0;
}

/// Comparador: a entrada no elenco sai com a notícia (0x76fd10) e a saída do outro mexe na tática: só a original
/// (a 0x776250 e a 0x7762f0 que ela chama passam pelos comparadores delas).
// ponytail: só a original; a prova é a carreira (140 chamadas) e o ramo da cópia no modo Rust.
unsafe extern "thiscall" fn comparar_convocar_substituto(this: usize, nacao: usize, tipo: u32, nivel: u32, saindo: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(796)(this, nacao, tipo, nivel, saindo)
}

// ---------- 242a leva: as seleções: as reservas de convocáveis de cada nação e as duas ordens delas (national_teams.cpp) ----------

unsafe fn cpd4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpd4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpd4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A nota da pessoa da vaga `q` (ponteiro para a pessoa): o curto +9 da ficha (+0x61).
unsafe fn cpd4_nota(q: usize) -> i16 {
    cpd4_w(cpd4_d(cpd4_d(q) + 0x61) + 9)
}

/// O começo do trecho do nível `nivel` na reserva (`o` 0: a principal, 0x15: o sub-21): o +8 do registro mais as
/// vagas (+0xe) dos níveis anteriores.
unsafe fn cpd4_inicio(rec: usize, o: usize, nivel: i8) -> i32 {
    let mut v = cpd4_d(rec + o + 8) as i32;
    for j in 0..nivel.max(0) as usize {
        v = v.wrapping_add(cpd4_b(rec + o + 0xe + 4 * j) as i32);
    }
    v
}

/// Põe a pessoa `p` na reserva do registro `rec` (`sub` 0: a principal em [this+0xc]; 1: o sub-21 em [this+0x10],
/// campos 0x15 adiante), no trecho do nível dela (0x76d810). Com vaga (+0xd < +0xe), no fim do trecho, e a menor nota
/// (+0xf) desce se for o caso; cheio, só com nota acima da menor: entra na posição pela nota (decrescente, depois das
/// iguais), empurrando o último para fora, e a menor nota vira a do último. Trecho cheio: ordena (qsort 0x945333 com
/// 0x778ed0).
unsafe fn cpd4_reservar(this: usize, p: usize, rec: usize, sub: u32) {
    let nivel_de: unsafe extern "stdcall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x76d810usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let o = 0x15 * sub as usize;
    let nivel = nivel_de(p, rec, sub) as i8;
    let c = rec.wrapping_add(o).wrapping_add((4 * nivel as i32) as usize);
    let reserva = || cpd4_d(this + 0xc + 4 * sub as usize);
    let vaga = |k: i32| reserva().wrapping_add((4 * k) as usize);
    let nota = cpd4_w(cpd4_d(p + 0x61) + 9);
    if cpd4_b(c + 0xd) < cpd4_b(c + 0xe) {
        let ini = cpd4_inicio(rec, o, nivel);
        core::ptr::write_unaligned(vaga(ini.wrapping_add(cpd4_b(c + 0xd) as i32)) as *mut usize, p);
        *((c + 0xd) as *mut u8) = cpd4_b(c + 0xd).wrapping_add(1);
        if nota < cpd4_w(c + 0xf) {
            core::ptr::write_unaligned((c + 0xf) as *mut i16, nota); // a menor nota
        }
    } else if nota > cpd4_w(c + 0xf) {
        crate::comparacao::ramo("reservas cheias entra pela nota");
        let ini = cpd4_inicio(rec, o, nivel);
        let ultimo = (cpd4_b(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(2);
        let mut pos = ini;
        let mut j = ultimo;
        while j >= ini {
            if nota < cpd4_nota(vaga(j)) {
                pos = j + 1; // depois das de nota maior
                break;
            }
            j -= 1;
        }
        let mut j = ultimo;
        while j >= pos {
            core::ptr::write_unaligned(vaga(j + 1) as *mut usize, cpd4_d(vaga(j)));
            j -= 1;
        }
        core::ptr::write_unaligned(vaga(pos) as *mut usize, p);
        let ult = (cpd4_b(c + 0xe) as i32).wrapping_add(ini).wrapping_sub(1);
        core::ptr::write_unaligned((c + 0xf) as *mut i16, cpd4_nota(vaga(ult)));
    }
    let vagas = cpd4_b(c + 0xe);
    if cpd4_b(c + 0xd) == vagas {
        qsort(vaga(cpd4_inicio(rec, o, nivel)), vagas as u32, 4, 0x778ed0);
    }
}

/// { 0x76abf0 } (thiscall, national_teams.cpp): `build_selection_pools()`: zera as contagens (+0xd, +0x22) e põe
/// 10000 nas menores notas (+0xf, +0x24) dos 4 níveis de cada registro de nação ([this+4], 0x6e bytes); depois, cada
/// pessoa ([0xae23bc], 0x6e bytes) com nação (+0x1a), ficha (+0x61), mais de 16 anos (+0x18), sem seleção ou já
/// convocada (+0x24, +0x28 = 0xb), na posição de jogador (0x53b620 de +0x3d) e com clube (+0x39) entra na reserva
/// da nação (cpd4_reservar) e, até 21 anos com menos de 5 jogos (+0x22), na do sub-21; e de novo pela segunda
/// nacionalidade (+0x1e: sem jogos pela primeira, outra e não a [0x9cf434]). Sem SEH, sem caixa, sem sorteio; as
/// locais são gravadas antes de ler (da nota guardada em 32 bits só vale o curto).
unsafe extern "thiscall" fn montar_reservas_das_selecoes(this: usize) {
    let mut i = 0i32;
    while i < *(0xae2350 as *const i32) {
        for k in 0..4usize {
            let r = cpd4_d(this + 4) + 0x6e * i as usize + 4 * k;
            *((r + 0xd) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0xf) as *mut i16, 10000);
            *((r + 0x22) as *mut u8) = 0;
            core::ptr::write_unaligned((r + 0x24) as *mut i16, 10000);
        }
        i += 1;
    }
    let jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    let mut p = *(0xae23bc as *const usize);
    let mut n = 0i32;
    while n < *(0xae2364 as *const i32) {
        let pode = cpd4_d(p + 0x1a) != 0
            && cpd4_d(p + 0x61) != 0
            && *((p + 0x18) as *const i8) > 0x10 // mais de 16 anos
            && (cpd4_d(p + 0x24) == 0 || cpd4_b(p + 0x28) == 0xb)
            && jogador(cpd4_b(p + 0x3d) as u32) != 0
            && cpd4_d(p + 0x39) != 0;
        if pode {
            for k in 0..2 {
                let id = if k == 0 {
                    *(cpd4_d(p + 0x1a) as *const i32)
                } else {
                    let n2 = cpd4_d(p + 0x1e);
                    if n2 == 0 || cpd4_b(p + 0x22) != 0 || n2 == cpd4_d(p + 0x1a) || *(n2 as *const i32) == *(0x9cf434 as *const i32) {
                        continue;
                    }
                    crate::comparacao::ramo("reservas segunda nacionalidade");
                    *(n2 as *const i32)
                };
                let rec = cpd4_d(this + 4).wrapping_add(id.wrapping_mul(0x6e) as usize);
                cpd4_reservar(this, p, rec, 0);
                if *((p + 0x18) as *const i8) > 0x15 || cpd4_b(p + 0x22) >= 5 { // o sub-21
                    continue;
                }
                cpd4_reservar(this, p, rec, 1);
            }
        }
        n += 1;
        p += 0x6e;
    }
}

/// Comparador: os registros das nações e, em cada reserva (principal e sub-21), o trecho dos 4 níveis (a soma das
/// vagas +0xe a partir do começo +8).
unsafe extern "thiscall" fn comparar_montar_reservas_das_selecoes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(797);
    let n = (*(0xae2350 as *const i32)).max(0) as usize;
    let recs = cpd4_d(this + 4);
    let mut reg = vec![(recs, 0x6e * n)];
    for i in 0..n {
        let rec = recs + 0x6e * i;
        for sub in 0..2usize {
            let o = 0x15 * sub;
            let total: usize = (0..4).map(|k| cpd4_b(rec + o + 0xe + 4 * k) as usize).sum();
            if total > 0 {
                reg.push((cpd4_d(this + 0xc + 4 * sub).wrapping_add(4 * cpd4_d(rec + o + 8)), 4 * total));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || montar_reservas_das_selecoes(this));
    PLACAR.registrar(igual, || format!("montar_reservas_das_selecoes({:x}) {:x?} x {:x?}", this, mo, mr));
}

/// { 0x778ed0 } (cdecl, national_teams.cpp): a ordem das reservas para o qsort da 0x76abf0: pela nota (+9 da ficha
/// +0x61) decrescente (nota de b menos a de a).
unsafe extern "C" fn ordem_das_reservas(a: usize, b: usize) -> i32 {
    (cpd4_nota(b) as i32) - cpd4_nota(a) as i32 // decrescente
}

unsafe extern "C" fn comparar_ordem_das_reservas(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(798)(a, b);
    let n = ordem_das_reservas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_reservas({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x778ef0 } (cdecl, national_teams.cpp): a ordem das pessoas pelo clube (+0x39) para os qsort das seleções
/// (0x76c120, 0x7706c0, 0x7771c0): sem clube depois; clubes diferentes pelo id; no mesmo, o de elenco 4 (+0x5e)
/// antes do de elenco 8.
unsafe extern "C" fn ordem_pelo_clube(a: usize, b: usize) -> i32 {
    let (x, y) = (cpd4_d(a), cpd4_d(b));
    let cx = if x != 0 { cpd4_d(x + 0x39) } else { 0 };
    let cy = if y != 0 { cpd4_d(y + 0x39) } else { 0 };
    if cx == 0 {
        return (cy != 0) as i32;
    }
    if cy == 0 {
        return -1;
    }
    if cx != cy {
        return (*(cx as *const i32)).wrapping_sub(*(cy as *const i32)); // o id do clube
    }
    let (mx, my) = (cpd4_b(x + 0x5e), cpd4_b(y + 0x5e));
    if mx & 4 != 0 && my & 8 != 0 {
        return -1;
    }
    if mx & 8 != 0 && my & 4 != 0 {
        return 1;
    }
    0
}

unsafe extern "C" fn comparar_ordem_pelo_clube(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(799)(a, b);
    let n = ordem_pelo_clube(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pelo_clube({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 242a leva: o adversário do amistoso e o da excursão (friendly.cpp) ----------

unsafe fn cpe1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpe1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpe1_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// O clube de id `id` na tabela [0xae23b4] (registros de 0x245 bytes).
unsafe fn cpe1_clube(id: usize) -> usize {
    cpe1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O candidato `c` já tem o candidato `clube` (pelo id, +0) na lista de adversários (+0xa)? Volta o cursor da lista
/// ao começo (+0xa = +2) e percorre com a 0x68d1b0, como as duas originais.
unsafe fn cpe1_na_lista(c: usize, clube: usize) -> bool {
    let l = cpe1_d(c + 0xa);
    if l == 0 {
        return false;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpe1_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    loop {
        let x = proximo(cpe1_d(c + 0xa));
        if x == 0 {
            return false;
        }
        if cpe1_d(x) == cpe1_d(clube) {
            return true;
        }
    }
}

/// Fora pela data: no ano do amistoso ([0xae2c90], com sinal, contra +0x34e8 do Friendly, sem sinal) o clube `op`
/// com +0x1db igual ao float de [0x9cf6f8] (comparado como inteiro).
unsafe fn cpe1_fora_pela_data(this: usize, op: usize) -> bool {
    *(0xae2c90 as *const i16) as i32 == core::ptr::read_unaligned((this + 0x34e8) as *const u16) as i32
        && cpe1_d(op + 0x1db) as u32 == *(0x9cf6f8 as *const u32)
}

/// Os clubes são rivais (+0xb3, +0xb7, +0xbb, nos dois sentidos)?
unsafe fn cpe1_rivais(a: usize, b: usize) -> bool {
    [0xb3usize, 0xb7, 0xbb].iter().any(|&o| a == cpe1_d(b + o) || b == cpe1_d(a + o))
}

/// As nações (+0x53) dos clubes são rivais (+0x110, +0x114, +0x118, nos dois sentidos)?
unsafe fn cpe1_nacoes_rivais(a: usize, b: usize) -> bool {
    let (na, nb) = (cpe1_d(a + 0x53), cpe1_d(b + 0x53));
    [0x110usize, 0x114, 0x118].iter().any(|&o| na == cpe1_d(nb + o) || nb == cpe1_d(na + o))
}

unsafe fn cpe1_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x5c4cb0 } (thiscall, ret 0x14, friendly.cpp): `Friendly::find_friendly_opponent(clube, lista, n, faixa,
/// excluir)`: a faixa (byte com sinal) presa em 1..=20; busca (bsearch 0x946464 com 0x5c5270, pelo byte +5) na lista
/// de candidatos (0x16 bytes, `n`) um da faixa, subindo a faixa até achar (a partir de 20: o começo da lista). Dali
/// até o penúltimo candidato (o último fica de fora, como na original), pula: o próprio clube, os do vetor `excluir`
/// (4 ponteiros, pelo id), os que já têm o clube na lista (+0xa, o cursor volta ao começo), os fora pela data, os de
/// faixas cruzadas (+0xe/+0x10, −1 = qualquer), os rivais, os de nações rivais (meio a meio: sorteio(2) = 0), os da
/// mesma cidade (+0x57) e os acima da faixa do sorteio (+4 > sorteio(3) + 3). Pela reputação (+0x80) do clube: abaixo
/// de 2000 a mesma nação (+0x53); de 2000 a 3999 o mesmo +0x75 da nação; de 4000 a 7999 o mesmo +0x71; com o +4 do
/// clube acima de sorteio(2), a mesma nação. Fica com o candidato quando sorteio(5) = 1; nenhum: 0. Sem SEH, sem caixa.
unsafe extern "thiscall" fn adversario_do_amistoso(this: usize, clube: usize, lista: usize, n: u32, faixa: u32, excluir: usize) -> usize {
    let mut faixa = faixa as i8;
    if faixa < 1 {
        faixa = 1;
    } else if faixa > 0x14 {
        faixa = 0x14;
    }
    let cp = cpe1_clube(cpe1_d(clube));
    let mut chave = [0u8; 0x16];
    chave[5] = faixa as u8;
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut c;
    loop {
        c = bsearch(chave.as_ptr() as usize, lista, n, 0x16, 0x5c5270);
        if chave[5] as i8 >= 0x14 {
            c = lista;
        }
        chave[5] = chave[5].wrapping_add(1);
        if c != 0 {
            break;
        }
    }
    let ultimo = lista.wrapping_add((n as usize).wrapping_mul(0x16)).wrapping_sub(0x16);
    while c < ultimo {
        'proximo: {
            let id = cpe1_d(c);
            let op = cpe1_clube(id);
            if cp == op {
                break 'proximo;
            }
            if excluir != 0 {
                for k in 0..4usize {
                    let e = cpe1_d(excluir + 4 * k);
                    if e != 0 && id == cpe1_d(e) {
                        break 'proximo;
                    }
                }
            }
            if cpe1_na_lista(c, clube) || cpe1_fora_pela_data(this, op) {
                break 'proximo;
            }
            let (fc, fcl) = (cpe1_w(c + 0xe), cpe1_w(clube + 0xe));
            if fc != -1 && fcl != -1 && (fc > cpe1_w(clube + 0x10) || fcl > cpe1_w(c + 0x10)) {
                break 'proximo;
            }
            if cpe1_rivais(cp, op) || (cpe1_nacoes_rivais(cp, op) && cpe1_sorteio(2) == 0) {
                break 'proximo;
            }
            if cpe1_d(cp + 0x57) == cpe1_d(op + 0x57) {
                break 'proximo;
            }
            if cpe1_b(c + 4) > (cpe1_sorteio(3) as i8).wrapping_add(3) {
                break 'proximo;
            }
            let rep = cpe1_w(cp + 0x80);
            let (nc, no) = (cpe1_d(cp + 0x53), cpe1_d(op + 0x53));
            let longe = if rep < 2000 {
                no != nc
            } else if rep < 4000 {
                cpe1_b(no + 0x75) != cpe1_b(nc + 0x75)
            } else if rep < 8000 {
                cpe1_d(no + 0x71) != cpe1_d(nc + 0x71)
            } else {
                false
            };
            if longe {
                break 'proximo;
            }
            if cpe1_b(clube + 4) as i32 > cpe1_sorteio(2) && no != nc {
                break 'proximo;
            }
            if cpe1_sorteio(5) == 1 { // o adversário do amistoso
                return c;
            }
        }
        c = c.wrapping_add(0x16);
    }
    0
}

/// As regiões dos dois comparadores: o estado do sorteio e o cursor (+0xa) da lista de cada candidato.
unsafe fn cpe1_regioes(candidatos: impl Iterator<Item = usize>) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for c in candidatos.filter(|&c| c != 0) {
        let l = cpe1_d(c + 0xa);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_adversario_do_amistoso(this: usize, clube: usize, lista: usize, n: u32, faixa: u32, excluir: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(800);
    let reg = cpe1_regioes((0..(n as i32).max(0) as usize).map(|i| lista + 0x16 * i));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, lista, n, faixa, excluir),
        || crate::aleatorio::sem_contar(|| adversario_do_amistoso(this, clube, lista, n, faixa, excluir)));
    PLACAR.registrar(igual && r == m, || format!("adversario_do_amistoso({:x}, {:x}, n {}, faixa {}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, lista, n, faixa as i8, r, m, mo, mr));
    r
}

/// { 0x5c5040 } (thiscall, ret 0x14, friendly.cpp): `Friendly::find_tour_opponent(clube, ordenados, n, faixa_max,
/// nacao)`: busca (bsearch 0x946464 com 0x5c5290, pelo +6) no vetor de `n` ponteiros para candidatos, ordenado pela
/// nação, um da nação (o id, [nacao]) e volta ao primeiro dela. Dali até o penúltimo: o primeiro cujo clube (+0x53)
/// não é da nação encerra (0); pula os acima da faixa (+5 > faixa_max), os fora pela data, o próprio clube, os que já
/// têm o clube na lista (+0xa), os rivais, os de nações rivais (sorteio(2) = 0), os da mesma cidade (+0x57) e os acima
/// da faixa do sorteio (+4 > sorteio(3) + 3); fica com o candidato quando sorteio(2) = 1. Nenhum: 0. Sem SEH, sem caixa.
unsafe extern "thiscall" fn adversario_da_excursao(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let cp = cpe1_clube(cpe1_d(clube));
    let mut chave = [0u8; 0x16];
    chave[5] = 0xff;
    core::ptr::write_unaligned(chave.as_mut_ptr().add(6) as *mut usize, cpe1_d(nacao));
    let pchave = chave.as_ptr() as usize;
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut s = bsearch(&pchave as *const usize as usize, ordenados, n, 4, 0x5c5290);
    if s == 0 {
        return 0;
    }
    if s > ordenados {
        loop {
            let anterior = cpe1_d(s - 4);
            s -= 4;
            if cpe1_d(anterior + 6) != cpe1_d(nacao) {
                s += 4;
                break;
            }
            if s <= ordenados {
                break;
            }
        }
    }
    let ultimo = ordenados.wrapping_add((n as usize).wrapping_mul(4)).wrapping_sub(4);
    while s < ultimo {
        let c = cpe1_d(s);
        let op = cpe1_clube(cpe1_d(c));
        if cpe1_d(op + 0x53) != nacao {
            return 0;
        }
        'proximo: {
            if cpe1_b(c + 5) > faixa_max as i8 || cpe1_fora_pela_data(this, op) || cp == op || cpe1_na_lista(c, clube) {
                break 'proximo;
            }
            if cpe1_rivais(cp, op) || (cpe1_nacoes_rivais(cp, op) && cpe1_sorteio(2) == 0) {
                break 'proximo;
            }
            if cpe1_d(cp + 0x57) == cpe1_d(op + 0x57) {
                break 'proximo;
            }
            if cpe1_b(c + 4) > (cpe1_sorteio(3) as i8).wrapping_add(3) {
                break 'proximo;
            }
            if cpe1_sorteio(2) == 1 { // o adversário da excursão
                return c;
            }
        }
        s += 4;
    }
    0
}

unsafe extern "thiscall" fn comparar_adversario_da_excursao(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(801);
    let reg = cpe1_regioes((0..(n as i32).max(0) as usize).map(|i| cpe1_d(ordenados + 4 * i)));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, ordenados, n, faixa_max, nacao),
        || crate::aleatorio::sem_contar(|| adversario_da_excursao(this, clube, ordenados, n, faixa_max, nacao)));
    PLACAR.registrar(igual && r == m, || format!("adversario_da_excursao({:x}, {:x}, n {}, faixa {}, nacao {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, ordenados, n, faixa_max as i8, nacao, r, m, mo, mr));
    r
}

// ---------- 242a leva: o resultado na tabela da liga e os pontos do resultado (league.cpp) ----------

unsafe fn cpe2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpe2_mais1(a: usize) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(1);
}

unsafe fn cpe2_somar(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, core::ptr::read_unaligned(a as *const i16).wrapping_add(v));
}

/// Linhas da tabela: 0x3b bytes; +0 clube, +5 confrontos (entradas de 14), +0x11..+0x1a os totais, +0x1f..+0x28 em
/// casa (jogos, vitórias, empates, derrotas, gols pró, contra, pontos), +0x2d..+0x36 fora.
const CPE2_LINHA: usize = 0x3b;

/// { 0x685f60 } (thiscall, ret 0x18, league.cpp): `League::apply_result_to_table(partida, tabela, n, marcar,
/// pts_casa*, pts_fora*)`: acha as linhas do mandante (+0x1c) e do visitante (+0x20) entre as `n` (curto) da tabela
/// (para no primeiro clube que não é nenhum dos dois depois de achar ambos) e soma o jogo: jogos (+0x1f/+0x2d), gols
/// (+0x23/+0x25 e +0x31/+0x33: o placar de +0x47/+0x48 quando ≠ 0xff, senão o de +0x43/+0x44). O resultado pelo placar
/// +0x43/+0x44; empatado: pela prorrogação (+0x38 bit 2, +0x47/+0x48; `marcar` grava 3 em +0x3d/+0x3e) e pelos pênaltis
/// (bits 0x20/1, +0x49/+0x4a; vitória do mandante conta como empate dele; 5 com o 0x20, senão 4); empate puro com os
/// pontos de empate da liga (+0xc3). Os pontos de vitória pela virtual +0x90 (0x68aad0) da competição de tipo 2
/// ([0xadadfc], +0x43) ou da própria liga, lida a cada chamada. Refaz os totais das duas linhas e devolve os pontos
/// (+0x19) em `pts_casa`/`pts_fora` (0 sem a linha). Com confrontos na primeira linha (+5), acha o adversário em cada
/// uma e soma o confronto (0x6869a0). Sem SEH, sem sorteio, sem caixa, locais todas gravadas antes de lidas.
unsafe extern "thiscall" fn aplicar_resultado_na_tabela(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let mut pontuador = liga;
    let id = cpe2_d(liga + 4);
    if id != 0 && cpe2_d(0xadadfc) != 0 {
        let comp = cpe2_d(cpe2_d(0xadadfc).wrapping_add(cpe2_d(id).wrapping_mul(4)));
        if *((comp + 0x43) as *const u8) == 2 {
            pontuador = comp;
        }
    }
    let n = n as i16;
    let linha = |k: i16| tab.wrapping_add(CPE2_LINHA.wrapping_mul(k as i32 as usize));
    let (mut casa, mut fora): (i16, i16) = (-1, -1);
    let mut k: i16 = 0;
    if n > 0 {
        loop {
            let c = cpe2_d(linha(k));
            if c == cpe2_d(p + 0x1c) {
                casa = k;
            } else if c == cpe2_d(p + 0x20) {
                fora = k;
            } else if casa != -1 && fora != -1 {
                break;
            }
            k = k.wrapping_add(1);
            if k >= n {
                break;
            }
        }
    }
    let pb = |o: usize| *((p + o) as *const i8);
    let noventa = || *((p + 0x47) as *const u8) != 0xff;
    if casa > -1 {
        let r = linha(casa);
        cpe2_mais1(r + 0x1f);
        let (pro, contra) = if noventa() { (pb(0x47), pb(0x48)) } else { (pb(0x43), pb(0x44)) };
        cpe2_somar(r + 0x23, pro as i16);
        cpe2_somar(r + 0x25, contra as i16);
    }
    if fora > -1 {
        let r = linha(fora);
        cpe2_mais1(r + 0x2d);
        let (pro, contra) = if noventa() { (pb(0x48), pb(0x47)) } else { (pb(0x44), pb(0x43)) };
        cpe2_somar(r + 0x31, pro as i16);
        cpe2_somar(r + 0x33, contra as i16);
    }
    let pontos = |clube: usize| -> i16 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(cpe2_d(cpe2_d(pontuador) + 0x90));
        f(pontuador, clube, p) as u8 as i8 as i16
    };
    // vence o mandante: +0x20 e os pontos (+0x27) dele, derrota (+0x30) do visitante; perde: +0x22 e +0x2e com os pontos (+0x35)
    let vence_casa = |vitoria: usize| {
        if casa > -1 {
            let r = linha(casa);
            cpe2_mais1(r + vitoria);
            cpe2_somar(r + 0x27, pontos(cpe2_d(p + 0x1c))); // os pontos do mandante
        }
        if fora > -1 {
            cpe2_mais1(linha(fora) + 0x30);
        }
    };
    let vence_fora = |vitoria: usize| {
        if casa > -1 {
            cpe2_mais1(linha(casa) + 0x22);
        }
        if fora > -1 {
            let r = linha(fora);
            cpe2_mais1(r + vitoria);
            cpe2_somar(r + 0x35, pontos(cpe2_d(p + 0x20)));
        }
    };
    let (gc, gf) = (pb(0x43), pb(0x44));
    let flags = core::ptr::read_unaligned((p + 0x38) as *const u16);
    let penaltis = if *((p + 0x38) as *const u8) & 0x20 != 0 { 5u8 } else { 4 };
    if gc > gf {
        vence_casa(0x20);
    } else if gc < gf {
        vence_fora(0x2e);
    } else if flags & 2 != 0 && pb(0x47) > pb(0x48) {
        vence_casa(0x20);
        if marcar != 0 {
            *((p + 0x3d) as *mut u8) = 3;
        }
    } else if flags & 2 != 0 && pb(0x47) < pb(0x48) {
        vence_fora(0x2e);
        if marcar != 0 {
            *((p + 0x3e) as *mut u8) = 3;
        }
    } else if flags & 0x21 != 0 && pb(0x49) > pb(0x4a) {
        vence_casa(0x21);
        if marcar != 0 {
            *((p + 0x3d) as *mut u8) = penaltis;
        }
    } else if flags & 0x21 != 0 && pb(0x49) < pb(0x4a) {
        vence_fora(0x2f);
        if marcar != 0 {
            *((p + 0x3e) as *mut u8) = penaltis;
        }
    } else if flags & 0x21 == 0 {
        let empate = *((liga + 0xc3) as *const i8) as i16; // o empate puro
        if casa > -1 {
            cpe2_mais1(linha(casa) + 0x21);
            cpe2_somar(linha(casa) + 0x27, empate);
        }
        if fora > -1 {
            cpe2_mais1(linha(fora) + 0x2f);
            cpe2_somar(linha(fora) + 0x35, empate);
        }
    }
    let totais = |r: usize| -> i16 {
        for (t, f, c) in [(0x11usize, 0x2d, 0x1f), (0x12, 0x2e, 0x20), (0x13, 0x2f, 0x21), (0x14, 0x30, 0x22)] {
            *((r + t) as *mut u8) = (*((r + f) as *const u8)).wrapping_add(*((r + c) as *const u8));
        }
        for (t, f, c) in [(0x15usize, 0x31, 0x23), (0x17, 0x33, 0x25), (0x19, 0x35, 0x27)] {
            let v = core::ptr::read_unaligned((r + f) as *const i16).wrapping_add(core::ptr::read_unaligned((r + c) as *const i16));
            core::ptr::write_unaligned((r + t) as *mut i16, v);
        }
        core::ptr::read_unaligned((r + 0x19) as *const i16)
    };
    for (k, pts) in [(casa, pts_casa), (fora, pts_fora)] {
        let v = if k > -1 { totais(linha(k)) } else { 0 };
        if !pts.is_null() {
            core::ptr::write_unaligned(pts, v);
        }
    }
    if cpe2_d(tab + 5) != 0 {
        let m = n as i32 - 1;
        let (mut ia, mut ib) = (-1i32, -1i32);
        let entrada = |linha_: i16, k: i16| cpe2_d(cpe2_d(linha(linha_) + 5).wrapping_add(14 * k as i32 as usize));
        let mut k: i16 = 0;
        if m > 0 {
            loop {
                if casa > -1 && entrada(casa, k) == cpe2_d(p + 0x20) {
                    ia = k as i32;
                    if ib > -1 {
                        break;
                    }
                }
                if fora > -1 && entrada(fora, k) == cpe2_d(p + 0x1c) {
                    ib = k as i32;
                    if ia > -1 {
                        break;
                    }
                }
                k = k.wrapping_add(1);
                if k as i32 >= m {
                    break;
                }
            }
        }
        let confronto: unsafe extern "thiscall" fn(usize, usize, i32, i32, i8, i8) = core::mem::transmute(0x6869a0usize);
        confronto(liga, tab, casa as i32, ia, gc, gf);
        confronto(liga, tab, fora as i32, ib, gf, gc);
    }
}

/// As regiões da 0x685f60: as `n` linhas, os confrontos de cada linha ((n − 1) entradas de 14, se a primeira os tem),
/// +0x3d/+0x3e da partida e os dois pontos devolvidos.
unsafe fn cpe2_regioes(p: usize, tab: usize, n: u32, pts_casa: *mut i16, pts_fora: *mut i16) -> Vec<(usize, usize)> {
    let n = (n as i16).max(0) as usize;
    let mut reg = vec![(p + 0x3d, 2)];
    if tab != 0 && n > 0 {
        reg.push((tab, CPE2_LINHA * n));
        if cpe2_d(tab + 5) != 0 && n > 1 {
            for i in 0..n {
                let h = cpe2_d(tab + CPE2_LINHA * i + 5);
                if h != 0 {
                    reg.push((h, 14 * (n - 1)));
                }
            }
        }
    }
    for pts in [pts_casa, pts_fora] {
        if !pts.is_null() {
            reg.push((pts as usize, 2));
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_aplicar_resultado_na_tabela(liga: usize, p: usize, tab: usize, n: u32, marcar: u32, pts_casa: *mut i16, pts_fora: *mut i16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut i16, *mut i16)>(802);
    let reg = cpe2_regioes(p, tab, n, pts_casa, pts_fora);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(liga, p, tab, n, marcar, pts_casa, pts_fora),
        || aplicar_resultado_na_tabela(liga, p, tab, n, marcar, pts_casa, pts_fora));
    PLACAR.registrar(igual, || format!("aplicar_resultado_na_tabela({:x}, partida {:x}, {:x}, n {}, marcar {}) {:x?} x {:x?}",
        liga, p, tab, n as i16, marcar, mo, mr));
}

/// { 0x68aad0 } (thiscall, ret 8, league.cpp): `League::points_for_result(clube, partida)`: os pontos de vitória
/// (+0xc2) se o clube venceu (mandante +0x1c com +0x43 > +0x44, visitante +0x20 com menos), senão 0; empatado: pela
/// prorrogação (+0xab bit 2, +0x47/+0x48, os de vitória), pelos pênaltis (bits 0x21, +0x49/+0x4a: os de empate +0xc3
/// a quem venceu, 0 a quem perdeu) ou os de empate. Só o al é escrito: os 3 bytes altos do eax são os da partida.
unsafe extern "thiscall" fn pontos_do_resultado(liga: usize, clube: usize, p: usize) -> u32 {
    let pb = |o: usize| *((p + o) as *const i8);
    let (casa, fora) = (cpe2_d(p + 0x1c), cpe2_d(p + 0x20));
    let venceu = |a: i8, b: i8| (clube == casa && a > b) || (clube == fora && a < b);
    let ganho = |o: usize| *((liga + o) as *const u8);
    let flags = *((liga + 0xab) as *const u8);
    let v = if pb(0x43) != pb(0x44) {
        if venceu(pb(0x43), pb(0x44)) { ganho(0xc2) } else { 0 } // os pontos da vitória
    } else if flags & 2 != 0 && pb(0x47) != pb(0x48) {
        if venceu(pb(0x47), pb(0x48)) { ganho(0xc2) } else { 0 }
    } else if flags & 0x21 != 0 && pb(0x49) != pb(0x4a) {
        if venceu(pb(0x49), pb(0x4a)) { ganho(0xc3) } else { 0 }
    } else {
        ganho(0xc3)
    };
    (p as u32 & 0xffff_ff00) | v as u32
}

comparar_valor!(comparar_pontos_do_resultado, pontos_do_resultado, 803, (clube: usize, p: usize) -> u32);

// ---------- 242a leva: a ordem pelo confronto guardado, a ordem das datas e a linha do clube na tabela da liga (league.cpp) ----------

unsafe fn cpe3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpe3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x683410 } (cdecl, league.cpp): `league_compare_table_rows_h2h(a, b)` (callback do qsort da 0x6827d0 na regra
/// 4 e da 0x6471a0 da Série B italiana): linha vazia empata; a comparação própria [0xb67a00] quando não dá 0 (chamada
/// de novo); os pontos (+0x19 da coluna [[0xb679fc]+0xad], b − a); os dois sem jogos (+0x11): o id dos clubes (a − b);
/// a posição do confronto guardada em +4 (b − a); por fim o nome dos clubes (+4, strcmp: −1, 0 ou 1).
unsafe extern "C" fn ordem_pela_posicao_do_confronto(a: usize, b: usize) -> i32 {
    let col = cpe3_d(cpe3_d(0xb679fc) + 0xad);
    if cpe3_d(a) == 0 || cpe3_d(b) == 0 {
        return 0;
    }
    if cpe3_d(0xb67a00) != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(cpe3_d(0xb67a00));
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(cpe3_d(0xb67a00));
            return f(a, b);
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (cpe3_w(a + o + 0x19), cpe3_w(b + o + 0x19));
    if pa != pb {
        return pb as i32 - pa as i32;
    }
    if *((a + o + 0x11) as *const u8) == 0 && *((b + o + 0x11) as *const u8) == 0 {
        return (cpe3_d(cpe3_d(a)) as i32).wrapping_sub(cpe3_d(cpe3_d(b)) as i32);
    }
    let dif = *((b + 4) as *const i8) as i32 - *((a + 4) as *const i8) as i32; // a posição do confronto
    if dif != 0 {
        return dif;
    }
    let (mut x, mut y) = (cpe3_d(a) + 4, cpe3_d(b) + 4);
    loop {
        let (cx, cy) = (*(x as *const u8), *(y as *const u8));
        if cx != cy {
            return if cx < cy { -1 } else { 1 };
        }
        if cx == 0 {
            return 0;
        }
        x += 1;
        y += 1;
    }
}

unsafe extern "C" fn comparar_ordem_pela_posicao_do_confronto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(804)(a, b);
    let n = ordem_pela_posicao_do_confronto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_posicao_do_confronto({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x685cf0 } (cdecl, league.cpp): `league_compare_dates(a, b)` (callback do bsearch da 0x685640 na tabela de
/// datas da liga): o ano (+2, sem sinal) e depois o dia (+0, com sinal): −1, 0 ou 1.
unsafe extern "C" fn ordem_das_datas_da_liga(a: usize, b: usize) -> i32 {
    let (ya, yb) = (cpe3_w(a + 2) as u16, cpe3_w(b + 2) as u16);
    if ya < yb { // o ano antes
        return -1;
    }
    if ya > yb {
        return 1;
    }
    let (da, db) = (cpe3_w(a), cpe3_w(b));
    if da < db {
        -1
    } else {
        (da > db) as i32
    }
}

unsafe extern "C" fn comparar_ordem_das_datas_da_liga(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(805)(a, b);
    let n = ordem_das_datas_da_liga(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_datas_da_liga({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x68ab70 } (thiscall, ret 8, league.cpp): `League::find_club_row(clube, linhas)`: a linha (0x3b bytes) do
/// clube entre as +0x3e (curto) de `linhas` (nula: a tabela da liga, +0xb1), ou 0.
unsafe extern "thiscall" fn linha_do_clube(liga: usize, clube: usize, linhas: usize) -> usize {
    let t = if linhas == 0 { cpe3_d(liga + 0xb1) } else { linhas };
    let n = cpe3_w(liga + 0x3e);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if cpe3_d(r) == clube { // a linha do clube
                return r;
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_linha_do_clube, linha_do_clube, 806, (clube: usize, linhas: usize) -> usize);

/// { 0x68abd0 } (thiscall, ret 4, league.cpp; virtual +0x80 das ligas): `League::contains_club(clube)`: 1 se o
/// clube tem linha na tabela da liga (+0xb1, +0x3e linhas; a linha no endereço 0 conta como não), senão 0.
unsafe extern "thiscall" fn liga_tem_o_clube(liga: usize, clube: usize) -> u32 {
    let t = cpe3_d(liga + 0xb1);
    let n = cpe3_w(liga + 0x3e);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if cpe3_d(r) == clube {
                return (r != 0) as u32; // a liga tem o clube
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_liga_tem_o_clube, liga_tem_o_clube, 807, (clube: usize) -> u32);
