// ---------- %%LEVA%%a leva: se a pessoa está na lista a pedido, a nota de um atributo relativo e se o clube pode ter time reserva (Database.cpp) ----------

/// { 0x544600 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências a pedido (vínculo +0x4e, bit 8). Sem clube, o do contrato (0x4cf5a0,
/// guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
unsafe extern "C" fn pessoa_listada_a_pedido(pessoa: usize, clube: usize) -> u32 {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    if pessoa == 0 {
        return 0;
    }
    let mut clube = clube;
    if clube == 0 {
        let c = contrato_de(*(0xae1050 as *const usize), pessoa);
        *(0xae1080 as *mut usize) = c;
        if c == 0 {
            return 0;
        }
        let id = core::ptr::read_unaligned((c + 4) as *const u32) as usize;
        clube = (*(0xae23b4 as *const usize)).wrapping_add(id.wrapping_mul(581));
        if clube == 0 {
            return 0;
        }
    }
    let v = vinculo(*(0xae1050 as *const usize), pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 8 != 0) as u32
}

/// { 0x544380 } (cdecl, Database.cpp): a nota de 1 a 20 (`nota_do_atributo`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador`. Termina no `_ftol` (eax inteiro, 1..20).
unsafe extern "C" fn nota_relativa_do_atributo(jogador: usize, valor: u32) -> u32 {
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as i32;
    nota_do_atributo((habilidade + 2 * (valor as u8 as i8 as i32)) as f64) as u32
}

/// { 0x542740 } (cdecl, Database.cpp): 1 se o `clube` pode ter time reserva. Não: seleções (id ≥
/// [0xae235c] − [0xae2350]), clubes com parceiro A/B (0x540a50 com nulo e 1); na faixa das segundas
/// seleções (id ≥ [0xae235c] − 2 × [0xae2350]), nação Espanha, Noruega, [0x9cf360], Brasil ou
/// Argentina ou continente (+0x71) [0x9cfa18] ou [0x9cfa08]; nos demais, nação Espanha, Alemanha ou
/// Portugal. Clube nulo: caixa "Error" e 0 (a original).
unsafe extern "C" fn clube_pode_ter_reservas(clube: usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(clube);
    }
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let g = |a: usize| *(a as *const u32);
    let gi = |a: usize| *(a as *const i32);
    let id = gi(clube);
    if id >= gi(0xae235c).wrapping_sub(gi(0xae2350)) {
        return 0; // seleção
    }
    if parceiro(clube, 0, 1) != 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 {
        return 1;
    }
    let n = g(nacao);
    if id >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2)) {
        if [0x9cf488, 0x9cf408, 0x9cf360, 0x9cf260, 0x9cf218].iter().any(|&a| n == g(a)) {
            return 0;
        }
        let continente = core::ptr::read_unaligned((nacao + 0x71) as *const usize);
        if continente != 0 && (g(continente) == g(0x9cfa18) || g(continente) == g(0x9cfa08)) {
            return 0;
        }
        return 1;
    }
    if n == g(0x9cf488) || n == g(0x9cf314) || n == g(0x9cf434) {
        return 0; // Espanha, Alemanha, Portugal
    }
    1 // pode
}

unsafe extern "C" fn comparar_pessoa_listada_a_pedido(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0 e 0x4cf5f0 só leem
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_a_pedido(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_a_pedido({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_nota_relativa_do_atributo(jogador: usize, valor: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u32>(%%I1%%)(jogador, valor);
    let n = nota_relativa_do_atributo(jogador, valor);
    PLACAR.registrar(r == n, || format!("nota_relativa_do_atributo({:x}, {}) original={} rust={}", jogador, valor as u8 as i8, r, n));
    r
}

unsafe extern "C" fn comparar_clube_pode_ter_reservas(clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(%%I2%%);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // 0x540a50 zera a marca +0x244 do clube quando recusa o parceiro: a região volta entre os lados
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube),
        || clube_pode_ter_reservas(clube));
    PLACAR.registrar(igual && r == n, || format!("clube_pode_ter_reservas({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
