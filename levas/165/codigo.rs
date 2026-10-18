// ---------- centesima sexagesima quinta leva: as vagas extras da tatica, as idades da selecao e o dado do tecnico humano ----------

/// { 0x897ee0 } (thiscall, ret 0x10, tactics.cpp): as duas vagas extras (+0x56 e +0x5a) da linha do
/// `clube` na tabela tática (0x8982b0 com `humano`; sem linha, 0): com `pessoa`, `*pessoa` na vaga
/// `qual` 0/1; sem, a vaga vira −1 e, na 0, a 1 desce para a 0 e fica −1. Devolve 1. `clube` nulo ou
/// `qual` fora: a original (caixa de erro).
unsafe extern "thiscall" fn vaga_extra_tatica(t: usize, clube: usize, qual: i8, pessoa: *const u32, humano: u32) -> u32 {
    if clube == 0 || !(0..=1).contains(&qual) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(367)(t, clube, qual, pessoa, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    let linha = linha_da_tabela(t, l);
    let casa = (linha + 0x56 + 4 * qual as usize) as *mut u32;
    if !pessoa.is_null() {
        crate::comparacao::ramo("vaga extra gravada");
        core::ptr::write_unaligned(casa, *pessoa);
        return 1;
    }
    crate::comparacao::ramo("vaga extra apagada");
    core::ptr::write_unaligned(casa, u32::MAX);
    if qual == 0 {
        let segunda = core::ptr::read_unaligned((linha + 0x5a) as *const u32);
        core::ptr::write_unaligned((linha + 0x56) as *mut u32, segunda);
        core::ptr::write_unaligned((linha + 0x5a) as *mut u32, u32::MAX);
    }
    1
}

/// { 0x76d430 } (stdcall, ret 0x10, national_teams.cpp): as idades limite (`*minima`, `*maxima`) de
/// uma seleção na competição `comp`: 23/23 nas competições olímpicas e de base ([comp+0x14] com id
/// numa das 9 constantes 0x9cf78c…0x9cf964); 24/22 com `tipo` 1 ou acima de 2; senão 26/22.
/// Ponteiros nulos: a original (caixa de erro).
unsafe extern "stdcall" fn idades_da_selecao(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    if minima.is_null() || maxima.is_null() {
        return orig::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(368)(comp, tipo, minima, maxima);
    }
    if comp != 0 {
        let c = *((comp + 0x14) as *const usize);
        if c != 0 {
            let id = *(c as *const i32);
            if [0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964usize]
                .iter()
                .any(|&a| *(a as *const i32) == id)
            {
                *minima = 0x17;
                *maxima = 0x17;
                return;
            }
        }
    }
    if tipo != 0 && tipo != 2 {
        *minima = 0x18;
        *maxima = 0x16;
    } else {
        *minima = 0x1a;
        *maxima = 0x16;
    }
}

/// { 0x5fdd30 } (thiscall, ret 8, human_manager.cpp): grava `valor` em +0x9a65 do registro do técnico
/// humano `tecnico` (sem ele, o da vez em [0xb74c26 + 0x300 × [0xb77c7a]]) entre os humanos `this`
/// ([this], 0xa6ef bytes cada; o índice é o id − [0xae2364] + 16, até [this+4] − 1). Devolve 1.
/// Sem técnico ou índice fora: a original (caixa de erro).
unsafe extern "thiscall" fn gravar_dado_do_humano(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let mut p = tecnico;
    if p.is_null() {
        let vez = *(0xb77c7a as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    if p.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369)(t, valor, tecnico);
    }
    let i = (*p).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369)(t, valor, tecnico);
    }
    crate::comparacao::ramo("dado do humano gravado");
    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a65) as *mut u32, valor);
    1
}

unsafe extern "thiscall" fn comparar_vaga_extra_tatica(t: usize, clube: usize, qual: i8, pessoa: *const u32, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(367);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, qual, pessoa, humano), || vaga_extra_tatica(t, clube, qual, pessoa, humano));
    PLACAR.registrar(igual && r == v, || format!("vaga_extra_tatica({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, qual, humano, r, mo, v, mr));
    r
}

unsafe extern "stdcall" fn comparar_idades_da_selecao(comp: usize, tipo: i8, minima: *mut u8, maxima: *mut u8) {
    let o = orig::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(368);
    let reg = if minima.is_null() || maxima.is_null() { vec![] } else { vec![(minima as usize, 1), (maxima as usize, 1)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(comp, tipo, minima, maxima), || idades_da_selecao(comp, tipo, minima, maxima));
    PLACAR.registrar(igual, || format!("idades_da_selecao({:#x}, {}) {:x?} x {:x?}", comp, tipo, mo, mr));
}

unsafe extern "thiscall" fn comparar_gravar_dado_do_humano(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(369);
    let mut p = tecnico;
    if p.is_null() {
        let vez = *(0xb77c7a as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    let reg = if p.is_null() {
        vec![]
    } else {
        let i = (*p).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
        if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) { vec![] } else { vec![(*(t as *const usize) + i as usize * 0xa6ef + 0x9a65, 4)] }
    };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, valor, tecnico), || gravar_dado_do_humano(t, valor, tecnico));
    PLACAR.registrar(igual && r == v, || format!("gravar_dado_do_humano({:#x}, {:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, valor, tecnico, r, mo, v, mr));
    r
}
