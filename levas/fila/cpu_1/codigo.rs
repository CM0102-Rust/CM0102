// ---------- %%LEVA%%a leva: as linhas da nação-chave: a linha de uma nação, a linha do clube e o ano da temporada do clube (key_nation.cpp) ----------

unsafe fn cpu1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x669340 } (cdecl, key_nation.cpp): `key_nation_index_of_nation(nação)`: a primeira linha da tabela das 35
/// nações-chave (0xb63d60, 72 bytes, nação em +0) com a nação dada (sem conferir a nula), ou 4. Pela moral do jogo
/// 0x4cb250, pelas aposentadorias 0x7a7310, pelo amistoso livre 0x5c6b70, pelos prêmios do dia das nações e pelas vagas
/// da UEFA. TapeCall a cada jogador: sem contador.
unsafe extern "C" fn key_nation_index_of_nation(nacao: usize) -> u8 {
    for k in 0..0x23usize {
        if cpu1_d(0xb63d60 + 72 * k) == nacao {
            return k as u8; // cpu1: a linha da nacao dada
        }
    }
    4
}

unsafe extern "C" fn comparar_key_nation_index_of_nation(nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(nacao);
    let n = key_nation_index_of_nation(nacao);
    PLACAR.record(r == n, || format!("key_nation_index_of_nation(nação {:x}) original={} rust={}", nacao, r, n));
    r
}

/// { 0x669360 } (cdecl, key_nation.cpp): `key_nation_index_of_club(clube)`. Clube que não é seleção (0x53b900), com
/// nação (+0x53) escolhida para jogar (+0x11c da nação): a linha da nação na tabela, ou a caixa "Error" (linha 0x26d) e
/// 0. Senão (a original chama 0x53b900 de novo): seleção, 1; a primeira linha cujo continente (+4) é o da nação do clube
/// (+0x71), ou 3 (sem nação, 3). Pelas aposentadorias 0x7a7310, pelo adiamento pelo tempo 0x923630, pelos dias até a
/// temporada 0x59b4c0, pelo contrato desejado 0x85fc50, pelo ano da temporada 0x669950 e outras. Sem contador.
unsafe extern "C" fn key_nation_index_of_club(clube: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) == 0 {
        let nacao = cpu1_d(clube + 0x53);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) != 0 {
            for k in 0..0x23usize {
                if cpu1_d(0xb63d60 + 72 * k) == nacao {
                    return k as u8; // cpu1: a linha da nacao escolhida
                }
            }
            show_error_at(0x9eaff4, 0x26d);
            return 0;
        }
    }
    if selecao(clube) != 0 {
        return 1;
    }
    let nacao = cpu1_d(clube + 0x53); // cpu1: a nacao do continente
    for k in 0..0x23usize {
        if nacao != 0 && cpu1_d(0xb63d64 + 72 * k) == cpu1_d(nacao + 0x71) {
            return k as u8; // cpu1: a linha do continente
        }
    }
    3
}

/// Se o clube cai na caixa "Error" da 0x669360 (não é seleção, nação escolhida fora da tabela).
unsafe fn hits_key_nation_error(clube: usize) -> bool {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nacao = cpu1_d(clube + 0x53);
    selecao(clube) == 0
        && nacao != 0
        && *((nacao + 0x11c) as *const u8) != 0
        && !(0..0x23usize).any(|k| cpu1_d(0xb63d60 + 72 * k) == nacao)
}

unsafe extern "C" fn comparar_key_nation_index_of_club(clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I1%%);
    if hits_key_nation_error(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = key_nation_index_of_club(clube);
    PLACAR.record(r == n, || format!("key_nation_index_of_club(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x669950 } (cdecl, key_nation.cpp): `key_nation_get_club_season_year(clube)`: o ano de hoje ([0xae2c92]); para
/// clube que não é seleção, pela linha do clube (0x669360): com o dword +0x17 da linha não zero ou o dia de início
/// (+0x15, sem sinal) abaixo de 60, e o dia de hoje ([0xae2c90], com sinal) antes do dia de início, o ano anterior. A
/// original devolve só os 16 bits baixos com sentido (os altos são a sobra da vaga empurrada). Pelos registros do clube
/// 0x444930/0x447430/0x448ad0 e pela história do jogador 0x7c3e40/0x7c5fe0.
unsafe extern "C" fn key_nation_get_club_season_year(clube: usize) -> u16 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    let ano = *(0xae2c92 as *const u16);
    if clube != 0 && selecao(clube) == 0 {
        let k = linha(clube) as i8 as isize;
        let e = (0xb63d60isize + 72 * k) as usize;
        let inicio = core::ptr::read_unaligned((e + 0x15) as *const u16);
        if (cpu1_d(e + 0x17) != 0 || inicio < 0x3c) && (*(0xae2c90 as *const i16) as i32) < inicio as i32 {
            return ano.wrapping_sub(1); // cpu1: o ano anterior
        }
    }
    ano
}

unsafe extern "C" fn comparar_key_nation_get_club_season_year(clube: usize) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u16>(%%I2%%);
    if clube != 0 && hits_key_nation_error(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = key_nation_get_club_season_year(clube);
    PLACAR.record(r == n, || format!("key_nation_get_club_season_year(clube {:x}) original={} rust={}", clube, r, n));
    r
}
