// ---------- %%LEVA%%a leva: a escalação guardada do clube: tirar o jogador da escalação, trocar duas vagas e a cópia para desfazer (tactics.cpp) ----------

unsafe fn cpx9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpx9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx9_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A linha do clube na tabela tática (0x8982b0, stdcall(clube, humano); −1 sem linha).
unsafe fn cpx9_linha(clube: usize, humano: u32) -> i32 {
    let f: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    f(clube, humano)
}

/// O começo da linha `l` (0x91 bytes cada, a partir de [this+0x601]).
unsafe fn cpx9_inicio(this: usize, l: i32) -> usize {
    cpx9_d(this + 0x601).wrapping_add(l.wrapping_mul(0x91) as usize)
}

/// As 16 cópias para desfazer (0x5c bytes a partir de this+0x30: +0 clube, +4 time, +8 a cópia; conta em +0x5f0): a do
/// clube e do time some (+8 = 0) e a conta desce.
unsafe fn cpx9_soltar_copias(this: usize, clube: usize, time: u32) {
    if cpx9_b(this + 0x5f0) == 0 {
        return;
    }
    for k in 0..0x10usize {
        let e = this + 0x30 + k * 0x5c;
        if cpx9_d(e + 8) != 0 && cpx9_d(e) == clube && cpx9_d(e + 4) as u32 == time {
            cpx9_pd(e + 8, 0);
            *((this + 0x5f0) as *mut u8) = cpx9_b(this + 0x5f0).wrapping_sub(1);
        }
    }
}

/// As regiões da escalação do clube: a linha dele na tabela tática (se houver) e as cópias para desfazer com a conta.
unsafe fn cpx9_regioes(this: usize, clube: usize, time: u32) -> Vec<(usize, usize)> {
    let mut reg = vec![(this + 0x30, 0x5c1)];
    let l = cpx9_linha(clube, time);
    if l != -1 {
        reg.push((cpx9_inicio(this, l), 0x91));
    }
    reg
}

/// { 0x896c60 } (thiscall, ret 0x10, tactics.cpp): `Tactics::remove_player_from_selection(clube, pessoa, time,
/// papeis)`, pelos contratos (troca de lugar no elenco, empréstimo e o fim dele), pelas saídas do elenco, pela escalação
/// automática (0x6dbc50), pelas seleções, pela aposentadoria e pelas transferências. Sem linha tática (0x8982b0): 0. As
/// 20 vagas da escalação (+2 da linha, ids) com a pessoa (id em +0) viram −1. Com `papeis`: as duas vagas extras (+0x56,
/// +0x5a: 0x897dc0 e 0x897ee0) e os batedores das cinco bolas paradas nos dois lados (0x897990 e 0x897bb0) que são ela
/// ficam vazios. As cópias para desfazer do clube e do time somem. Devolve 1 se a tirou de uma vaga ou de bola parada.
/// Clube ou pessoa nulos: a original (caixa "Error", 0).
unsafe extern "thiscall" fn tirar_da_escalacao(this: usize, clube: usize, pessoa: usize, time: u32, papeis: u32) -> u32 {
    if clube == 0 || pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%)(this, clube, pessoa, time, papeis); // caixa: só a original
    }
    let l = cpx9_linha(clube, time);
    if l == -1 {
        return 0;
    }
    let mut achou = 0;
    for k in 0..20usize { // cpx9: as vagas da escalação
        let vaga = cpx9_inicio(this, l) + 2 + 4 * k;
        if cpx9_i(vaga) == cpx9_i(pessoa) {
            crate::comparacao::ramo(if k < 11 { "cpx_9 tirou titular" } else { "cpx_9 tirou reserva" });
            cpx9_pd(vaga, u32::MAX); // cpx9: a vaga fica vazia
            achou = 1;
        }
    }
    if papeis != 0 {
        let da_linha: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x897dc0usize);
        let extra: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x897ee0usize);
        // ponytail: a original empurra o tipo e o lado com sobra nos bytes altos (o deslocamento da linha e a pilha);
        // as chamadas leem só o byte baixo, e a cópia passa os valores limpos
        let batedor: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x897990usize);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x897bb0usize);
        for qual in 0..2u32 {
            if da_linha(this, clube, qual, time) == pessoa {
                extra(this, clube, qual, 0, time);
            }
        }
        for tipo in 0..5u32 {
            for lado in 0..2u32 {
                if batedor(this, clube, tipo, lado, time) == pessoa {
                    crate::comparacao::ramo("cpx_9 batedor tirado");
                    marcar(this, clube, tipo, lado, 0, time);
                    achou = 1;
                }
            }
        }
    }
    cpx9_soltar_copias(this, clube, time);
    achou
}

/// Comparador: lado a lado pela devolução, pela linha do clube na tabela tática (as vagas, as extras e os batedores) e
/// pelas cópias para desfazer com a conta. Clube ou pessoa nulos (caixa): só a original.
unsafe extern "thiscall" fn comparar_tirar_da_escalacao(this: usize, clube: usize, pessoa: usize, time: u32, papeis: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%);
    if clube == 0 || pessoa == 0 {
        return o(this, clube, pessoa, time, papeis);
    }
    let reg = cpx9_regioes(this, clube, time);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, pessoa, time, papeis), || tirar_da_escalacao(this, clube, pessoa, time, papeis));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_escalacao(clube {:#x}, pessoa {:#x}, time {}, papeis {}) original={} {:x?} rust={} {:x?}", clube, pessoa, time, papeis, r, mo, n, mr));
    r
}

/// { 0x8973e0 } (thiscall, ret 0x10, tactics.cpp): `Tactics::swap_selection_slots(clube, a, b, time)`, pela tela do
/// elenco (0x46835e). Vagas `a` e `b` (bytes com sinal, 0..19): troca os ids das duas vagas da escalação (+2 + 4 × vaga
/// da linha tática, 0x90ba90 com 4 bytes) e as cópias para desfazer do clube e do time somem; devolve 1. Sem linha
/// (0x8982b0): 0. Clube nulo ou vaga fora: a original (caixa "Error", 0).
unsafe extern "thiscall" fn trocar_vagas_da_escalacao(this: usize, clube: usize, a: u32, b: u32, time: u32) -> u32 {
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(%%I1%%)(this, clube, a, b, time); // caixa: só a original
    }
    let l = cpx9_linha(clube, time);
    if l == -1 {
        return 0;
    }
    let trocar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x90ba90usize);
    let inicio = cpx9_inicio(this, l);
    trocar(inicio + 2 + 4 * va as usize, inicio + 2 + 4 * vb as usize, 4);
    cpx9_soltar_copias(this, clube, time);
    1
}

/// Comparador: lado a lado pela devolução, pela linha tática do clube e pelas cópias para desfazer. Clube nulo ou vaga
/// fora (caixa): só a original.
unsafe extern "thiscall" fn comparar_trocar_vagas_da_escalacao(this: usize, clube: usize, a: u32, b: u32, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(%%I1%%);
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return o(this, clube, a, b, time);
    }
    let reg = cpx9_regioes(this, clube, time);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, a, b, time), || trocar_vagas_da_escalacao(this, clube, a, b, time));
    PLACAR.registrar(igual && r == n, || format!("trocar_vagas_da_escalacao(clube {:#x}, {}, {}, time {}) original={} {:x?} rust={} {:x?}", clube, va, vb, time, r, mo, n, mr));
    r
}

/// { 0x897250 } (thiscall, ret 8, tactics.cpp): `Tactics::has_selection_undo(clube, time)`, pela tela do elenco
/// (0x458a58). Com cópias (+0x5f0) e o humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]): a cópia dele (índice
/// [0xae2364] − id − 1, de 0 a 15) existe (+8) e é deste clube (+0) e time (+4): 1; senão 0.
unsafe extern "thiscall" fn tem_copia_da_escalacao(this: usize, clube: usize, time: u32) -> u32 {
    if cpx9_b(this + 0x5f0) == 0 {
        return 0;
    }
    let t = cpx9_d((cpx9_w(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize);
    if t == 0 {
        return 0;
    }
    let k = cpx9_i(0xae2364).wrapping_sub(cpx9_i(t)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        return 0;
    }
    let e = this + 0x30 + k as usize * 0x5c;
    (cpx9_d(e + 8) != 0 && cpx9_d(e) == clube && cpx9_d(e + 4) as u32 == time) as u32
}

/// Comparador: a devolução (rotina pura).
unsafe extern "thiscall" fn comparar_tem_copia_da_escalacao(this: usize, clube: usize, time: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%)(this, clube, time);
    let n = tem_copia_da_escalacao(this, clube, time);
    PLACAR.registrar(r == n, || format!("tem_copia_da_escalacao(clube {:#x}, time {}) original={} rust={}", clube, time, r, n));
    r
}
