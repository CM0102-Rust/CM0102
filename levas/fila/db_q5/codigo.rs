// ---------- %%LEVA%%a leva: se o clube é basco e se a pessoa está na lista de transferências do clube ou na de empréstimos (Database.cpp) ----------

/// Os 23 clubes bascos de 0x53d7f0 (ponteiros fixos da área 0x9d00a4..), na ordem da original.
const BASQUE_CLUBS: [usize; 23] = [
    0x9d0120, 0x9d0134, 0x9d02e4, 0x9d02e8, 0x9d02ec, 0x9d02f0, 0x9d02f4, 0x9d02f8, 0x9d02fc, 0x9d0304, 0x9d0308,
    0x9d030c, 0x9d0310, 0x9d0314, 0x9d0318, 0x9d031c, 0x9d0320, 0x9d0324, 0x9d0328, 0x9d032c, 0x9d0330, 0x9d0334,
    0x9d0338,
];

/// { 0x53d7f0 } (cdecl, Database.cpp): 1 se o `clube` é da Espanha ([0x9cf488], nação +0x53) e o id
/// dele é um dos 23 clubes bascos (`BASQUE_CLUBS`). Clube ou nação nulos: 0.
unsafe extern "C" fn database_is_basque_club(clube: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    if clube == 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 || g(nacao) != g(0x9cf488) {
        return 0; // fora da Espanha
    }
    let id = g(clube);
    BASQUE_CLUBS.iter().any(|&a| id == g(a)) as u32
}

/// O vínculo (0x4cf5f0) da `pessoa` (não nula) com o `clube` e o clube usado: sem clube, o do contrato
/// (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
/// Sem contrato ou clube: (0, 0). Comum a 0x5444a0 e 0x544590 (e 0x544520, 0x544600).
unsafe fn contract_link_for_club(pessoa: usize, clube: usize) -> (usize, usize) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mut clube = clube;
    if clube == 0 {
        let c = contract_of(*(0xae1050 as *const usize), pessoa);
        *(0xae1080 as *mut usize) = c;
        if c == 0 {
            return (0, 0);
        }
        let id = core::ptr::read_unaligned((c + 4) as *const u32) as usize;
        clube = (*(0xae23b4 as *const usize)).wrapping_add(id.wrapping_mul(581));
        if clube == 0 {
            return (0, 0);
        }
    }
    (contract_bond(*(0xae1050 as *const usize), pessoa, clube), clube)
}

/// { 0x5444a0 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de transferências pelo clube (vínculo +0x4e, bit 1), a não ser que o clube
/// não seja de técnico humano (0x600a50 com 1, 1, 0, 0) e a função no clube da pessoa (+0x3d) seja 0xf.
unsafe extern "C" fn staff_is_transfer_listed(pessoa: usize, clube: usize) -> u32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if pessoa == 0 {
        return 0;
    }
    let (v, clube) = contract_link_for_club(pessoa, clube);
    if v == 0 || *((v + 0x4e) as *const u8) & 1 == 0 {
        return 0;
    }
    if humano(clube, 1, 1, 0, 0) == 0 && *((pessoa + 0x3d) as *const u8) == 0xf {
        return 0;
    }
    1
}

/// { 0x544590 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de empréstimos (vínculo +0x4e, bit 2).
unsafe extern "C" fn staff_is_loan_listed(pessoa: usize, clube: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let (v, _) = contract_link_for_club(pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 2 != 0) as u32
}

unsafe extern "C" fn comparar_database_is_basque_club(clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(clube);
    let n = database_is_basque_club(clube);
    SCOREBOARD.record(r == n, || format!("database_is_basque_club({:x}) original={} rust={}", clube, r, n));
    r
}

unsafe extern "C" fn comparar_staff_is_transfer_listed(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0, 0x4cf5f0 e 0x600a50 (clube não
    // nulo) só leem
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(pessoa, clube),
        || staff_is_transfer_listed(pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_transfer_listed({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_staff_is_loan_listed(pessoa: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(pessoa, clube),
        || staff_is_loan_listed(pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("staff_is_loan_listed({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}
