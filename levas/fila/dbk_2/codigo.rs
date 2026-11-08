// ---------- %%LEVA%%a leva: a criação do jogo: tirar a pessoa do clube e os olheiros do clube com os do time parceiro (Database.cpp) ----------

/// { 0x53c190 } (cdecl, Database.cpp): `database_remove_staff_from_club(clube, pessoa, data, desligar, como_jogador)`.
/// Na criação (os cortes de 0x541e60, 0x544970 e 0x524d70) e na carreira (transferências, demissões). Seleção = id do
/// clube ≥ [0xae235c] − 2 × [0xae2350]. Com `data`: os 8 bytes dela vão para a pessoa (+0x29 na seleção, senão +0x3e;
/// 0x417c10 copia +2, +0 e +4 e repete numa local que ninguém lê), antes de olhar o cargo. Cargo = pessoa +0x28 (seleção)
/// ou +0x3d (byte); com `como_jogador`, 11, 12, 13 e 15 viram 11 e os outros 0. Casas (tabela de salto 0x53c54c): 0
/// nenhuma; 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf e, na seleção principal (id < [0xae235c] − [0xae2350]), o +0xcf da sub-21
/// (0x53b4f0 com a nação +0x53) se for a mesma pessoa; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 elenco
/// +0xd7 (50); 12 +0xcf e o elenco; 13 +0xd3 e o elenco; 15 +0x19f e o elenco; 14, acima de 15 ou negativo: 0. A casa
/// que tem a pessoa (a primeira) é zerada; pessoa fora da casa: 0 (nas 12, 13 e 15 a primeira casa já zerada fica).
/// Com `desligar`: zera o clube na pessoa (+0x24 na seleção, +0x39 no clube). Devolve 1. Clube nulo: 0. Pessoa nula: a
/// original (caixa "Error" e 0).
unsafe extern "C" fn database_remove_staff_from_club(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    if clube == 0 {
        return 0;
    }
    if pessoa == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%)(clube, pessoa, data, desligar, como_jogador);
    }
    let gi = |a: usize| *(a as *const i32);
    let selecao = || gi(clube) >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2));
    if data != 0 {
        let d = if selecao() { pessoa + 0x29 } else { pessoa + 0x3e };
        core::ptr::write_unaligned((d + 2) as *mut u16, core::ptr::read_unaligned((data + 2) as *const u16));
        core::ptr::write_unaligned(d as *mut u16, core::ptr::read_unaligned(data as *const u16));
        core::ptr::write_unaligned((d + 4) as *mut u32, core::ptr::read_unaligned((data + 4) as *const u32));
    }
    let mut cargo = if selecao() { *((pessoa + 0x28) as *const u8) } else { *((pessoa + 0x3d) as *const u8) };
    if como_jogador != 0 {
        cargo = if matches!(cargo, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    let rd = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    let zerar = |d: usize| core::ptr::write_unaligned((clube + d) as *mut usize, 0);
    // a primeira das `k` casas a partir de +d que tem a pessoa, zerada; false se nenhuma tem
    let tirar_da_casa = |d: usize, k: usize| match (0..k).find(|&j| rd(d + 4 * j) == pessoa) {
        Some(j) => {
            zerar(d + 4 * j);
            true
        }
        None => false,
    };
    let tirou = match cargo {
        0 => true,
        1 => tirar_da_casa(0xbf, 1),
        2..=4 => tirar_da_casa(0xc3, 3),
        5 => {
            let ok = tirar_da_casa(0xcf, 1);
            let id = gi(clube);
            let n = gi(0xae235c);
            if ok && id >= n.wrapping_sub(gi(0xae2350).wrapping_mul(2)) && id < n.wrapping_sub(gi(0xae2350)) {
                let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
                let s = sub21(rd(0x53));
                if core::ptr::read_unaligned((s + 0xcf) as *const usize) == pessoa {
                    core::ptr::write_unaligned((s + 0xcf) as *mut usize, 0); // o treinador da sub-21
                }
            }
            ok
        }
        6 | 7 => tirar_da_casa(0xd3, 1),
        8 => tirar_da_casa(0x19f, 5),
        9 => tirar_da_casa(0x1b3, 7),
        10 => tirar_da_casa(0x1cf, 3),
        11 => tirar_da_casa(0xd7, 50),
        12 => tirar_da_casa(0xcf, 1) && tirar_da_casa(0xd7, 50),
        13 => tirar_da_casa(0xd3, 1) && tirar_da_casa(0xd7, 50),
        15 => tirar_da_casa(0x19f, 5) && tirar_da_casa(0xd7, 50),
        _ => false,
    };
    if !tirou {
        return 0;
    }
    if desligar != 0 {
        if selecao() {
            core::ptr::write_unaligned((pessoa + 0x24) as *mut usize, 0);
        } else {
            core::ptr::write_unaligned((pessoa + 0x39) as *mut usize, 0); // desliga a pessoa do clube
        }
    }
    1
}

/// { 0x53cd60 } (cdecl, Database.cpp): `database_get_club_scouts(clube, vetor)`: os 7 olheiros (+0x1b3) do `clube` no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro): as 7
/// casas como estão. Com parceiro: os não nulos do clube e depois os do parceiro até 7; o resto zerado só quando os
/// dois juntos não enchem as 7. Contador de 8 bits na original (no máximo 7). Devolve 1. Clube nulo: a original
/// (caixa "Error" e 0). Na carreira, pela 0x7f7270 (as listas dos olheiros, nas transferências e contratações).
unsafe extern "C" fn database_get_club_scouts(clube: usize, vetor: *mut usize) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(%%I1%%)(clube, vetor);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p == 0 {
        for j in 0..7 {
            core::ptr::write_unaligned(vetor.add(j), rd(clube + 0x1b3 + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..7 {
        let o = rd(clube + 0x1b3 + 4 * j);
        if o != 0 {
            core::ptr::write_unaligned(vetor.add(n), o);
            n += 1;
        }
    }
    for j in 0..7 {
        if n >= 7 {
            return 1;
        }
        let o = rd(p + 0x1b3 + 4 * j); // os olheiros do parceiro
        if o != 0 {
            core::ptr::write_unaligned(vetor.add(n), o);
            n += 1;
        }
    }
    for k in n..7 {
        core::ptr::write_unaligned(vetor.add(k), 0);
    }
    1
}

unsafe extern "C" fn comparar_database_remove_staff_from_club(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%);
    if clube == 0 || pessoa == 0 {
        return o(clube, pessoa, data, desligar, como_jogador); // 0 ou caixa: só a original
    }
    // o clube, a pessoa e, na seleção principal, a sub-21 (a única outra casa que a rotina zera)
    let mut regioes = vec![(clube, 581), (pessoa, 110)];
    let (id, n, nacoes) = (*(clube as *const i32), *(0xae235c as *const i32), *(0xae2350 as *const i32));
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if id >= n.wrapping_sub(nacoes.wrapping_mul(2)) && id < n.wrapping_sub(nacoes) && nacao != 0 {
        let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
        let s = sub21(nacao);
        if s != clube {
            regioes.push((s, 581));
        }
    }
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(clube, pessoa, data, desligar, como_jogador),
        || database_remove_staff_from_club(clube, pessoa, data, desligar, como_jogador));
    SCOREBOARD.record(igual && r == m, || format!("database_remove_staff_from_club({:x}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        clube, pessoa, data != 0, desligar, r, m, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_get_club_scouts(clube: usize, vetor: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(%%I1%%);
    if clube == 0 {
        return o(clube, vetor); // caixa: só a original
    }
    // o vetor e a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(vetor as usize, 28), (clube + 0x244, 1)],
        || o(clube, vetor), || database_get_club_scouts(clube, vetor));
    SCOREBOARD.record(igual && r == n, || format!("database_get_club_scouts({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
