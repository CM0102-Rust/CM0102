// ---------- 223a leva: a ordem do histórico de competições, o vetor de atributos do jogador e se a nação é britânica ou a Irlanda (Database.cpp) ----------

/// { 0x540780 } (cdecl, Database.cpp): comparação do qsort das linhas do histórico de competições.
/// Linha sem competição (+4) vai para o fim (a sem e b com: 1; a com e b sem: −1; as duas sem: 0);
/// depois pelo id da competição (a − b); na mesma, pela temporada (+8, palavra) da mais nova para a
/// mais velha (b − a), a não ser que uma delas seja −1 (0).
unsafe extern "C" fn ordem_do_historico_de_competicao(a: usize, b: usize) -> i32 {
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let ca = ler(a, 4);
    if ca == 0 {
        return (ler(b, 4) != 0) as i32;
    }
    let cb = ler(b, 4);
    if cb == 0 {
        return -1;
    }
    let d = (*(ca as *const i32)).wrapping_sub(*(cb as *const i32));
    if d != 0 {
        return d; // pelo id da competição
    }
    let ta = core::ptr::read_unaligned((a + 8) as *const i16);
    let tb = core::ptr::read_unaligned((b + 8) as *const i16);
    if ta == -1 || tb == -1 {
        return 0;
    }
    tb as i32 - ta as i32 // a temporada mais nova primeiro
}

/// As 54 primeiras células do vetor de atributos (0x53a610), na ordem em que a original grava:
/// (deslocamento no registro de jogador, peso). Peso 0 = o zero do começo.
const CELULAS_DO_VETOR: [(u8, u8); 54] = [
    (0x0f, 0), (0x10, 0), (0x11, 0), (0x12, 0), (0x13, 0), (0x14, 0), (0x15, 0), (0x16, 0), (0x1a, 0),
    (0x17, 0), (0x18, 0), (0x19, 0),
    (0x30, 0), (0x3b, 0), (0x1e, 0), (0x23, 2), (0x24, 0), (0x26, 2), (0x27, 2), (0x2a, 1), (0x2b, 2),
    (0x31, 2), (0x32, 2), (0x33, 2), (0x35, 0), (0x37, 2), (0x38, 2), (0x39, 0), (0x3a, 0), (0x3e, 2),
    (0x41, 0), (0x43, 2),
    (0x28, 2), (0x22, 2), (0x29, 0), (0x40, 2), (0x3d, 0), (0x3c, 0), (0x36, 0), (0x2e, 0), (0x2f, 0),
    (0x25, 0), (0x1f, 0), (0x20, 0), (0x21, 0), (0x1b, 2), (0x1c, 0), (0x1d, 0), (0x2c, 0), (0x2d, 0),
    (0x42, 0), (0x3f, 2), (0x44, 2), (0x34, 0),
];

/// { 0x53a610 } (cdecl, Database.cpp): estende os bytes do registro de `jogador` num vetor de 63
/// células de 3 bytes (palavra com sinal + peso): zera os 63 pesos; células 0..53 pela tabela
/// `CELULAS_DO_VETOR` (posições 0..8, lados 9..11, depois os atributos, peso 2 nos que contam e 1 no
/// +0x2a); a célula 54 é o byte +0x58 da `pessoa`. Os fins de grupo vão às saídas não nulas:
/// posições 9, lados 0xc, ponderados 0x20. Devolve o vetor (eax).
unsafe extern "C" fn vetor_de_atributos_do_jogador(jogador: usize, pessoa: usize, vetor: usize, fim_dos_lados: *mut u8,
                                                   fim_das_posicoes: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    for i in 0..0x3f {
        *((vetor + 3 * i + 2) as *mut u8) = 0;
    }
    for (i, &(d, peso)) in CELULAS_DO_VETOR.iter().enumerate() {
        core::ptr::write_unaligned((vetor + 3 * i) as *mut i16, *((jogador + d as usize) as *const i8) as i16);
        *((vetor + 3 * i + 2) as *mut u8) = peso;
        match i {
            8 if !fim_das_posicoes.is_null() => *fim_das_posicoes = 9,
            11 if !fim_dos_lados.is_null() => *fim_dos_lados = 0xc,
            31 if !fim_dos_ponderados.is_null() => *fim_dos_ponderados = 0x20,
            _ => {}
        }
    }
    core::ptr::write_unaligned((vetor + 3 * 54) as *mut i16, *((pessoa + 0x58) as *const i8) as i16);
    vetor
}

/// { 0x5446d0 } (cdecl, Database.cpp): 1 se a `nacao` (não nula) é Inglaterra, Escócia, Irlanda, País
/// de Gales ou Irlanda do Norte (a regra das nações da casa).
unsafe extern "C" fn nacao_britanica_ou_irlanda(nacao: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    let id = g(nacao);
    (id == g(0x9cf2e4) || id == g(0x9cf45c) || id == g(0x9cf2dc) || id == g(0x9cf510) || id == g(0x9cf3e4)) as u32
}

unsafe extern "C" fn comparar_ordem_do_historico_de_competicao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(586)(a, b);
    let n = ordem_do_historico_de_competicao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_historico_de_competicao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

unsafe extern "C" fn comparar_vetor_de_atributos_do_jogador(jogador: usize, pessoa: usize, vetor: usize, fim_dos_lados: *mut u8,
                                                            fim_das_posicoes: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize>(587);
    let mut regioes = vec![(vetor, 0x3f * 3)];
    for p in [fim_dos_lados, fim_das_posicoes, fim_dos_ponderados] {
        if !p.is_null() {
            regioes.push((p as usize, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes,
        || o(jogador, pessoa, vetor, fim_dos_lados, fim_das_posicoes, fim_dos_ponderados),
        || vetor_de_atributos_do_jogador(jogador, pessoa, vetor, fim_dos_lados, fim_das_posicoes, fim_dos_ponderados));
    PLACAR.registrar(igual && r == n, || format!("vetor_de_atributos_do_jogador({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", jogador, pessoa, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_nacao_britanica_ou_irlanda(nacao: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(588)(nacao);
    let n = nacao_britanica_ou_irlanda(nacao);
    PLACAR.registrar(r == n, || format!("nacao_britanica_ou_irlanda({:x}) original={} rust={}", nacao, r, n));
    r
}

// ---------- 223a leva: se o clube é basco e se a pessoa está na lista de transferências do clube ou na de empréstimos (Database.cpp) ----------

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
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(589)(clube);
    let n = clube_basco(clube);
    PLACAR.registrar(r == n, || format!("clube_basco({:x}) original={} rust={}", clube, r, n));
    r
}

unsafe extern "C" fn comparar_pessoa_listada_pelo_clube(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0, 0x4cf5f0 e 0x600a50 (clube não
    // nulo) só leem
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(590);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_pelo_clube(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_pelo_clube({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_pessoa_listada_para_emprestimo(pessoa: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(591);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_para_emprestimo(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_para_emprestimo({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

// ---------- 223a leva: se a pessoa está na lista a pedido, a nota de um atributo relativo e se o clube pode ter time reserva (Database.cpp) ----------

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
        return orig::<unsafe extern "C" fn(usize) -> u32>(594)(clube);
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
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(592);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_a_pedido(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_a_pedido({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_nota_relativa_do_atributo(jogador: usize, valor: u32) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u32>(593)(jogador, valor);
    let n = nota_relativa_do_atributo(jogador, valor);
    PLACAR.registrar(r == n, || format!("nota_relativa_do_atributo({:x}, {}) original={} rust={}", jogador, valor as u8 as i8, r, n));
    r
}

unsafe extern "C" fn comparar_clube_pode_ter_reservas(clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(594);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // 0x540a50 zera a marca +0x244 do clube quando recusa o parceiro: a região volta entre os lados
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube),
        || clube_pode_ter_reservas(clube));
    PLACAR.registrar(igual && r == n, || format!("clube_pode_ter_reservas({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
