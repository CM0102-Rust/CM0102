// ---------- %%LEVA%%a leva: se o clube é basco e se a pessoa está na lista de transferências do clube ou na de empréstimos (Database.cpp) ----------

/// Os 23 clubes bascos de 0x53d7f0 (ponteiros fixos da área 0x9d00a4..), na ordem da original.
const CLUBES_BASCOS: [usize; 23] = [
    0x9d0120, 0x9d0134, 0x9d02e4, 0x9d02e8, 0x9d02ec, 0x9d02f0, 0x9d02f4, 0x9d02f8, 0x9d02fc, 0x9d0304, 0x9d0308,
    0x9d030c, 0x9d0310, 0x9d0314, 0x9d0318, 0x9d031c, 0x9d0320, 0x9d0324, 0x9d0328, 0x9d032c, 0x9d0330, 0x9d0334,
    0x9d0338,
];

/// { 0x53d7f0 } (cdecl, Database.cpp): 1 se o `clube` é da Espanha ([0x9cf488], nação +0x53) e o id
/// dele é um dos 23 clubes bascos (`CLUBES_BASCOS`). Clube ou nação nulos: 0.
unsafe extern "C" fn clube_basco(clube: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    if clube == 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 || g(nacao) != g(0x9cf488) {
        return 0; // fora da Espanha
    }
    let id = g(clube);
    CLUBES_BASCOS.iter().any(|&a| id == g(a)) as u32
}

/// O vínculo (0x4cf5f0) da `pessoa` (não nula) com o `clube` e o clube usado: sem clube, o do contrato
/// (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
/// Sem contrato ou clube: (0, 0). Comum a 0x5444a0 e 0x544590 (e 0x544520, 0x544600).
unsafe fn vinculo_da_lista(pessoa: usize, clube: usize) -> (usize, usize) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mut clube = clube;
    if clube == 0 {
        let c = contrato_de(*(0xae1050 as *const usize), pessoa);
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
    (vinculo(*(0xae1050 as *const usize), pessoa, clube), clube)
}

/// { 0x5444a0 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de transferências pelo clube (vínculo +0x4e, bit 1), a não ser que o clube
/// não seja de técnico humano (0x600a50 com 1, 1, 0, 0) e a função no clube da pessoa (+0x3d) seja 0xf.
unsafe extern "C" fn pessoa_listada_pelo_clube(pessoa: usize, clube: usize) -> u32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if pessoa == 0 {
        return 0;
    }
    let (v, clube) = vinculo_da_lista(pessoa, clube);
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
unsafe extern "C" fn pessoa_listada_para_emprestimo(pessoa: usize, clube: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let (v, _) = vinculo_da_lista(pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 2 != 0) as u32
}

unsafe extern "C" fn comparar_clube_basco(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(clube);
    let n = clube_basco(clube);
    PLACAR.registrar(r == n, || format!("clube_basco({:x}) original={} rust={}", clube, r, n));
    r
}

unsafe extern "C" fn comparar_pessoa_listada_pelo_clube(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0, 0x4cf5f0 e 0x600a50 (clube não
    // nulo) só leem
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_pelo_clube(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_pelo_clube({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_pessoa_listada_para_emprestimo(pessoa: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_para_emprestimo(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_para_emprestimo({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}
