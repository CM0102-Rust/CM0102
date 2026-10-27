// ---------- %%LEVA%%a leva: as seleções: a competição atual de cada nação e a convocação do substituto (national_teams.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
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
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, continente, fim_de_fase)
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
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(%%I2%%)(this, nacao, tipo, nivel, saindo)
}
