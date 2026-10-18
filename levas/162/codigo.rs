// ---------- centesima sexagesima segunda leva: as vagas da tabela tatica (tactics.cpp) ----------

/// A linha `linha` da tabela tática do objeto `t` ([t+0x601], 0x91 bytes por linha: palavra da
/// formação e 20 vagas de 4 bytes a partir de +2).
unsafe fn linha_da_tabela(t: usize, linha: i32) -> usize {
    core::ptr::read_unaligned((t + 0x601) as *const usize).wrapping_add((linha as usize).wrapping_mul(0x91))
}

/// { 0x896a90 } (thiscall, ret 0x10, tactics.cpp): grava `*pessoa` na vaga `vaga` 0..19 da linha do
/// `clube` (0x8982b0 com `humano`; sem linha, 0) e apaga, entre as 16 instruções pendentes
/// (+0x30, 0x5c bytes: clube, humano, ativa), as ativas desse clube e `humano`, descontando o
/// contador +0x5f0. Devolve 1. `clube` ou `pessoa` nulos, vaga fora: a original (caixa de erro).
unsafe extern "thiscall" fn gravar_vaga_tatica(t: usize, clube: usize, vaga: i8, pessoa: *const u32, humano: u32) -> u32 {
    if clube == 0 || !(0..=19).contains(&vaga) || pessoa.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(358)(t, clube, vaga, pessoa, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned((linha_da_tabela(t, l) + 2 + 4 * vaga as usize) as *mut u32, *pessoa);
    let pendentes = (t + 0x5f0) as *mut u8;
    if *pendentes != 0 {
        for k in 0..16 {
            let e = t + 0x30 + 0x5c * k;
            let ativa = (e + 8) as *mut u32;
            if *ativa != 0 && *(e as *const usize) == clube && *((e + 4) as *const u32) == humano {
                *ativa = 0;
                *pendentes = (*pendentes).wrapping_sub(1);
            }
        }
    }
    1
}

/// { 0x8968f0 } (thiscall, ret 0xc, tactics.cpp): a pessoa (em [0xae23bc], 0x6e bytes) da vaga `vaga`
/// 0..19 da linha do `clube`; 0 sem linha ou vaga vazia (negativa). Vaga com pessoa sem ficha
/// (+0x61), ou, numa seleção (0x53b900), pessoa sem nação (+0x24) ou de outra nação (+0x53): a vaga
/// vira −1 e devolve 0. `clube` nulo ou vaga fora: a original (caixa de erro).
unsafe extern "thiscall" fn pessoa_da_vaga_tatica(t: usize, clube: usize, vaga: i8, humano: u32) -> usize {
    if clube == 0 || !(0..=19).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(359)(t, clube, vaga, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    let casa = (linha_da_tabela(t, l) + 2 + 4 * vaga as usize) as *mut i32;
    let id = core::ptr::read_unaligned(casa);
    if id <= -1 {
        return 0;
    }
    let pessoas = *(0xae23bc as *const usize);
    let p = pessoas.wrapping_add((id as usize).wrapping_mul(0x6e));
    if core::ptr::read_unaligned((p + 0x61) as *const usize) == 0 {
        core::ptr::write_unaligned(casa, -1);
        return 0;
    }
    if selecao(clube) != 0 {
        let nacao = core::ptr::read_unaligned((p + 0x24) as *const usize);
        if nacao == 0 || core::ptr::read_unaligned((nacao + 0x53) as *const u32) != core::ptr::read_unaligned((clube + 0x53) as *const u32) {
            core::ptr::write_unaligned(casa, -1);
            return 0;
        }
    }
    p
}

/// { 0x8967d0 } (thiscall, ret 0xc, tactics.cpp): grava a formação `formacao` (palavra) na linha do
/// `clube`; 0 sem linha, senão 1. `clube` nulo ou formação fora de 0..[+0x5fd]+[+0x5f5]−1: a
/// original (caixa de erro).
unsafe extern "thiscall" fn gravar_formacao_tatica(t: usize, clube: usize, formacao: i16, humano: u32) -> u32 {
    let limite = core::ptr::read_unaligned((t + 0x5fd) as *const i32)
        .wrapping_add(core::ptr::read_unaligned((t + 0x5f5) as *const i32))
        .wrapping_sub(1);
    if clube == 0 || formacao < 0 || formacao as i32 > limite {
        return orig::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(360)(t, clube, formacao, humano);
    }
    let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let l = linha_tatica(clube, humano);
    if l == -1 {
        return 0;
    }
    core::ptr::write_unaligned(linha_da_tabela(t, l) as *mut i16, formacao);
    1
}

/// As regiões que as três podem tocar: a linha do clube na tabela (se houver) e as instruções
/// pendentes com o contador (+0x30..+0x5f1).
unsafe fn regioes_da_tabela(t: usize, clube: usize, humano: u32) -> Vec<(usize, usize)> {
    let mut r = vec![(t + 0x30, 0x5c1)];
    if clube != 0 {
        let linha_tatica: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
        let l = linha_tatica(clube, humano);
        if l != -1 {
            r.push((linha_da_tabela(t, l), 0x91));
        }
    }
    r
}

unsafe extern "thiscall" fn comparar_gravar_vaga_tatica(t: usize, clube: usize, vaga: i8, pessoa: *const u32, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(358);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, vaga, pessoa, humano), || gravar_vaga_tatica(t, clube, vaga, pessoa, humano));
    PLACAR.registrar(igual && r == v, || format!("gravar_vaga_tatica({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, vaga, humano, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_pessoa_da_vaga_tatica(t: usize, clube: usize, vaga: i8, humano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(359);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, vaga, humano), || pessoa_da_vaga_tatica(t, clube, vaga, humano));
    PLACAR.registrar(igual && r == v, || format!("pessoa_da_vaga_tatica({:#x}, {:#x}, {}, {}) original={:#x} {:x?} rust={:#x} {:x?}", t, clube, vaga, humano, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_gravar_formacao_tatica(t: usize, clube: usize, formacao: i16, humano: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(360);
    let reg = regioes_da_tabela(t, clube, humano);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, formacao, humano), || gravar_formacao_tatica(t, clube, formacao, humano));
    PLACAR.registrar(igual && r == v, || format!("gravar_formacao_tatica({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, formacao, humano, r, mo, v, mr));
    r
}
