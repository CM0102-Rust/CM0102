// ---------- %%LEVA%%a leva: as permissões de trabalho do começo do jogo, a contagem da cláusula e o prêmio por jogo do jogador (transfer_manager.cpp) ----------

unsafe fn cps2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x8c6e60 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH só da data local): `TRANSFER_MANAGER::
/// init_work_permits()`: pela criação do jogo (0x8bba30). Para cada pessoa: com mais de 13 jogos pela seleção (+0x22),
/// 0x3fff na palavra +3 do registro de 8 bytes ([this+0x86e] + i × 8); com mais de 5, 0x1fff. Jogador (+0x3d = 0xb) de
/// clube (+0x39) que precisa de permissão de trabalho (0x8c7560 sem ignorar a dada): o ano mais antigo na nação do
/// clube pela carreira (0x7c3e40, um grupo de 6 entradas de 0x2b bytes por temporada; para no primeiro clube de outra
/// nação), começando pelo ano de +0x40. Menos de 5 anos até o ano de hoje ([0xae2c92]): a permissão (0x8c7070, sem
/// ordenar; antes a original leva a data +0x3e de 52 em 52 semanas até a data [0xdd6ec8], sem usar o resultado); senão a
/// segunda nacionalidade (+0x1e) vira a nação do clube. No fim ordena as permissões (qsort 0x945333 com 0x8e55c0).
unsafe extern "thiscall" fn init_work_permits(this: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(0x54c770usize);
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let carreira: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let mut d = [0u8; 8];
    data_padrao(d.as_mut_ptr() as usize);
    let mut i = 0usize;
    let mut p = cps2_d(0xae23bc);
    while (i as i32) < cps2_d(0xae2364) as i32 {
        let jogos = cps2_b(p + 0x22);
        let marca = (cps2_d(this + 0x86e).wrapping_add(i.wrapping_mul(8)) + 3) as *mut u16;
        if jogos > 0xd {
            core::ptr::write_unaligned(marca, 0x3fff);
        } else if jogos > 5 {
            core::ptr::write_unaligned(marca, 0x1fff);
        }
        let clube = cps2_d(p + 0x39);
        if clube != 0 && cps2_b(p + 0x3d) == 0xb && precisa(this, p, clube, 0) != 0 {
            let mut ano = cps2_w(p + 0x40);
            let mut conta = 0i32;
            let t = carreira(cps2_d(0xb74350), p, &mut conta);
            let (mut k, mut e) = (0i32, t + 2);
            while k < conta {
                let id = cps2_d(e) as i32;
                if id >= 0 && id < cps2_d(0xae235c) as i32 {
                    let c = cps2_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245));
                    if cps2_d(c + 0x53) != cps2_d(cps2_d(p + 0x39) + 0x53) {
                        break; // clube de outra nação
                    }
                    let a = cps2_w(e - 2);
                    if a < ano { // cps2: o ano mais antigo
                        crate::comparison::branch("cps_2 ano mais antigo");
                        ano = a;
                    }
                }
                k += 6;
                e += 0x102;
            }
            if t != 0 {
                liberar(t);
            }
            if (cps2_w(0xae2c92) as i32) - (ano as i32) < 5 { // cps2: menos de cinco anos na nação
                // a data +0x3e levada de 52 em 52 semanas até a data [0xdd6ec8] (a original não usa o resultado)
                let (mut t1, mut t2, mut t3) = ([0u8; 8], [0u8; 8], [0u8; 8]);
                let dp = d.as_mut_ptr() as usize;
                copiar(dp, t1.as_mut_ptr() as usize, p + 0x3e);
                loop {
                    let (ah, dh) = (cps2_w(0xdd6eca), cps2_w(dp + 2));
                    if !(ah > dh || (ah == dh && (cps2_w(0xdd6ec8) as i16) > (cps2_w(dp) as i16))) {
                        break;
                    }
                    let r = mais_semanas(dp, t2.as_mut_ptr() as usize, 0x34);
                    copiar(dp, t3.as_mut_ptr() as usize, r);
                }
                crate::comparison::branch("cps_2 permissao pela carreira");
                // ponytail: a original empurra o registrador do ano com os 16 bits altos de um ponteiro; a 0x8c7070
                // só grava a palavra baixa
                dar(this, p, cps2_d(cps2_d(p + 0x39) + 0x53), ano as u32, 1, 0);
            } else {
                crate::comparison::branch("cps_2 segunda nacionalidade");
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, cps2_d(cps2_d(p + 0x39) + 0x53)); // cps2: a segunda nacionalidade
            }
        }
        i += 1;
        p += 0x6e;
    }
    qsort(cps2_d(this + 0x93a), cps2_d(this + 0x93e) as u32, 0xe, 0x8e55c0);
}

/// { 0x8c3e50 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_clause_count(pessoa, tipo)`: no tipo 4,
/// os jogos pela seleção (+0x22) menos os da época do acordo (byte +7 do registro de 8 bytes [this+0x86e] + id × 8);
/// senão os jogos pelo clube (+0x3b) do contrato (0x4cf5a0, pedido duas vezes como na original), ou 0 sem contrato.
unsafe extern "thiscall" fn get_clause_count(this: usize, pessoa: usize, tipo: u32) -> u16 {
    if tipo as u8 == 4 {
        let r = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8));
        return (cps2_b(pessoa + 0x22) as u16).wrapping_sub(cps2_b(r + 7) as u16);
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    if contract_of(cps2_d(0xae1050), pessoa) == 0 {
        return 0;
    }
    cps2_w(contract_of(cps2_d(0xae1050), pessoa) + 0x3b)
}

/// { 0x8d0ea0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_first_team_appearance_money(pessoa)`:
/// com o bit 2 do registro de 8 bytes ([this+0x86e] + id × 8), busca o prêmio por jogo da pessoa no vetor [this+0x88c]
/// (conta +0x894; busca binária 0x946464 com 0x8b4e00, registros de 13 bytes) e o devolve; sem achar, tira o bit (soma
/// 0xfe ao byte) e devolve 0.
unsafe extern "thiscall" fn find_first_team_appearance_money(this: usize, pessoa: usize) -> usize {
    let id = cps2_d(pessoa);
    if cps2_b(cps2_d(this + 0x86e).wrapping_add(id.wrapping_mul(8))) & 2 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = id as i32;
    let r = bsearch(&chave, cps2_d(this + 0x88c), cps2_d(this + 0x894), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    let m = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8)) as *mut u8;
    *m = (*m).wrapping_add(0xfe);
    0
}

// ponytail: cresce o vetor das permissões (0x8c7070) e grava em todas as pessoas: só a original; a cópia leva os ramos
// (roda uma vez, na criação do jogo).
unsafe extern "thiscall" fn comparar_init_work_permits(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_get_clause_count(this: usize, pessoa: usize, tipo: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(%%I1%%)(this, pessoa, tipo);
    let n = get_clause_count(this, pessoa, tipo);
    SCOREBOARD.record(r == n, || format!("get_clause_count(pessoa {}, tipo {}) original={} rust={}", cps2_d(pessoa), tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelo byte do registro da pessoa, e pela devolução.
unsafe extern "thiscall" fn comparar_find_first_team_appearance_money(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let m = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(m, 1)], || o(this, pessoa), || find_first_team_appearance_money(this, pessoa));
    SCOREBOARD.record(igual && r == n, || format!("find_first_team_appearance_money(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", cps2_d(pessoa), r, n, mo, mr));
    r
}
