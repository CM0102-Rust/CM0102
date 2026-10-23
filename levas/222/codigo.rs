// ---------- 222a leva: o alcance da reputação, a reputação vista por alguém e se a pessoa é estrangeira (Database.cpp) ----------

/// { 0x5408c0 } (cdecl, Database.cpp): de que distância `quem_ve` olha a `pessoa`, o que escolhe a
/// reputação: 0 = mesmo clube ou clubes da mesma nação (+0x39, clube +0x53) → a nacional; 2 = mesma
/// nação (+0x1a) ou não estrangeiro nela (0x53d980) → a atual; 3 = não estrangeiro na nação do clube
/// da pessoa, ou reputação de jogador (+0x61, +9) + reputação do clube de quem vê (+0x80) ≥ 14000 →
/// a média; 4 (sem quem vê, quem vê entre os 16 últimos ids = técnicos humanos, sem dados ou sem
/// clube) → a mundial. Só o al é devolvido (quem chama usa al).
unsafe extern "C" fn escopo_da_reputacao(pessoa: usize, quem_ve: usize) -> u8 {
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    if quem_ve == 0 {
        return 4;
    }
    let clube = ler(pessoa, 0x39);
    if clube != 0 {
        if clube == ler(quem_ve, 0x39) {
            return 0;
        }
        let outro = ler(quem_ve, 0x39);
        if outro != 0 && ler(clube, 0x53) == ler(outro, 0x53) {
            return 0;
        }
    }
    let nacao = ler(pessoa, 0x1a);
    if nacao == ler(quem_ve, 0x1a) {
        return 2; // mesma nação
    }
    if estrangeira(quem_ve, nacao, 1) == 0 {
        return 2;
    }
    let clube = ler(pessoa, 0x39);
    if clube != 0 && estrangeira(quem_ve, ler(clube, 0x53), 1) == 0 {
        return 3;
    }
    if *(quem_ve as *const i32) >= (*(0xae2364 as *const i32)).wrapping_sub(16) {
        return 4;
    }
    let jogador = ler(pessoa, 0x61);
    if jogador == 0 {
        return 4;
    }
    let clube_de_quem_ve = ler(quem_ve, 0x39);
    if clube_de_quem_ve == 0 {
        return 4;
    }
    let soma = core::ptr::read_unaligned((jogador + 9) as *const i16) as i32 + core::ptr::read_unaligned((clube_de_quem_ve + 0x80) as *const i16) as i32;
    if soma >= 0x36b0 {
        3
    } else {
        4
    }
}

/// { 0x5407e0 } (cdecl, Database.cpp): a reputação da `pessoa` vista por `quem_ve`, pelo alcance
/// (0x5408c0): dos dados de jogador (+0x61: atual +9, nacional +0xb, mundial +0xd) ou, com
/// `nao_jogador`, dos de não jogador (+0x69: +8, +0xa, +0xc). 4 → mundial, 2 → atual, 0 → nacional,
/// 3 → metade da mundial + metade da atual, outro → metade da nacional + metade da atual (divisões
/// truncadas). Só o ax é devolvido.
unsafe extern "C" fn reputacao_vista_por(pessoa: usize, quem_ve: usize, nao_jogador: u32) -> u16 {
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x5408c0usize);
    let e = escopo(pessoa, quem_ve);
    let (dados, atual) = if nao_jogador != 0 {
        (core::ptr::read_unaligned((pessoa + 0x69) as *const usize), 8)
    } else {
        (core::ptr::read_unaligned((pessoa + 0x61) as *const usize), 9)
    };
    // r(0) atual, r(2) nacional, r(4) mundial
    let r = |d: usize| core::ptr::read_unaligned((dados + atual + d) as *const i16) as i32;
    (match e {
        4 => r(4),
        2 => r(0), // escopo 2: a atual
        0 => r(2),
        3 => r(4) / 2 + r(0) / 2,
        _ => r(2) / 2 + r(0) / 2,
    }) as u16
}

/// { 0x53d980 } (cdecl, Database.cpp): 1 se a `pessoa` é estrangeira na `nacao` (nula: 1). Não é com
/// a nação na primeira (+0x1a) ou segunda nacionalidade (+0x1e). Com `grupos`, também não é: entre
/// Inglaterra, Escócia, País de Gales, Irlanda do Norte e Irlanda; entre Austrália e [0x9cf3f4];
/// em [0x9cf360] com a nação [0x9cf4f0]; e em Portugal quem não é da UE (0x53d740) com contrato
/// (0x4cf5a0) que começou até 1/1/1998 (Date 0x549f70; contrato +0x25 dia, +0x27 ano).
unsafe extern "C" fn pessoa_e_estrangeira(pessoa: usize, nacao: usize, grupos: u32) -> u32 {
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let g = |a: usize| *(a as *const u32);
    if nacao == 0 {
        return 1;
    }
    let n1 = ler(pessoa, 0x1a);
    if n1 == nacao {
        return 0;
    }
    let n2 = ler(pessoa, 0x1e);
    if n2 == nacao {
        return 0;
    }
    if grupos == 0 {
        return 1;
    }
    let id = g(nacao);
    // Inglaterra, Escócia, País de Gales, Irlanda do Norte e Irlanda
    let britanica = |x: u32| x == g(0x9cf2e4) || x == g(0x9cf45c) || x == g(0x9cf510) || x == g(0x9cf2dc) || x == g(0x9cf3e4);
    if britanica(id) {
        if britanica(g(n1)) {
            return 0;
        }
        if n2 != 0 && britanica(g(n2)) {
            return 0;
        }
    }
    if n1 != 0 {
        let (x, y) = (g(0x9cf224), g(0x9cf3f4)); // Austrália e o par dela
        if (id == x || id == y) && (g(n1) == x || g(n1) == y) {
            return 0;
        }
    }
    if id == g(0x9cf360) && n1 != 0 && g(n1) == g(0x9cf4f0) {
        return 0;
    }
    if id == g(0x9cf434) && comunitaria(pessoa, 0) == 0 {
        let contrato = contrato_de(*(0xae1050 as *const usize), pessoa);
        if contrato != 0 {
            let mut d = [0u16; 4];
            let data = construir(&mut d, 1, 0, 0x7ce, 0xffff_ffff) as *const u16; // 1/1/1998
            let (dia, ano) = (core::ptr::read_unaligned(data), core::ptr::read_unaligned(data.add(1)));
            let ano_do_contrato = core::ptr::read_unaligned((contrato + 0x27) as *const u16);
            if ano > ano_do_contrato {
                return 0;
            }
            if ano == ano_do_contrato && dia as i16 > core::ptr::read_unaligned((contrato + 0x25) as *const i16) {
                return 0;
            }
        }
    }
    1 // estrangeira
}

unsafe extern "C" fn comparar_escopo_da_reputacao(pessoa: usize, quem_ve: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(577)(pessoa, quem_ve);
    let n = escopo_da_reputacao(pessoa, quem_ve);
    PLACAR.registrar(r == n, || format!("escopo_da_reputacao({:x}, {:x}) original={} rust={}", pessoa, quem_ve, r, n));
    r
}

unsafe extern "C" fn comparar_reputacao_vista_por(pessoa: usize, quem_ve: usize, nao_jogador: u32) -> u16 {
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u16>(578)(pessoa, quem_ve, nao_jogador);
    let n = reputacao_vista_por(pessoa, quem_ve, nao_jogador);
    PLACAR.registrar(r == n, || format!("reputacao_vista_por({:x}, {:x}, {}) original={} rust={}", pessoa, quem_ve, nao_jogador, r, n));
    r
}

unsafe extern "C" fn comparar_pessoa_e_estrangeira(pessoa: usize, nacao: usize, grupos: u32) -> u32 {
    // a 0x4cf5a0 e o Date (num Date local) só leem: os dois lados podem rodar
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u32>(579)(pessoa, nacao, grupos);
    let n = pessoa_e_estrangeira(pessoa, nacao, grupos);
    PLACAR.registrar(r == n, || format!("pessoa_e_estrangeira({:x}, {:x}, {}) original={} rust={}", pessoa, nacao, grupos, r, n));
    r
}

// ---------- 222a leva: a idade da pessoa, se a nação tem transferência livre e se a pessoa é da UE (Database.cpp) ----------

/// { 0x53ab90 } (cdecl, Database.cpp): a idade da `pessoa` em anos completos na `data` (palavra +0 dia
/// do ano, +2 ano): ano − ano de nascimento (+0x12), menos 1 se o dia de nascimento (+0x10) ainda não
/// chegou. Pessoa nula: caixa "Error" e 0xff (a original). Só o ax é devolvido (quem chama usa al).
unsafe extern "C" fn idade_da_pessoa(pessoa: usize, data: *const u16) -> u16 {
    if pessoa == 0 {
        return orig::<unsafe extern "C" fn(usize, *const u16) -> u16>(580)(pessoa, data);
    }
    let dia_nasc = core::ptr::read_unaligned((pessoa + 0x10) as *const i16) as i32;
    let anos = core::ptr::read_unaligned(data.add(1)).wrapping_sub(core::ptr::read_unaligned((pessoa + 0x12) as *const u16));
    if dia_nasc - core::ptr::read_unaligned(data as *const i16) as i32 > 0 {
        anos.wrapping_sub(1) // ainda não fez aniversário
    } else {
        anos
    }
}

/// { 0x544670 } (cdecl, Database.cpp): 1 se a `nacao` tem transferência livre: com as regras novas da
/// FIFA ([0xae233c] = 1), no Brasil ou numa nação da Europa (continente +0x71, [0x9cfa10]) fora de
/// Polônia, Turquia, Croácia e Rússia.
unsafe extern "C" fn nacao_com_transferencia_livre(nacao: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    if *(0xae233c as *const u8) == 1 {
        return 1;
    }
    let id = g(nacao);
    if id == g(0x9cf260) {
        return 1; // Brasil
    }
    let continente = core::ptr::read_unaligned((nacao + 0x71) as *const usize);
    if continente == 0 {
        return 0;
    }
    if id == g(0x9cf430) || id == g(0x9cf4dc) || id == g(0x9cf2ac) || id == g(0x9cf448) {
        return 0;
    }
    (g(continente) == g(0x9cfa10)) as u32 // Europa
}

/// { 0x53d740 } (cdecl, Database.cpp): 1 se a `pessoa` conta como da UE para um clube da
/// `nacao_do_clube`: nacionalidade (+0x1a) ou segunda (+0x1e) com a marca de UE (nação +0x7f = 2) ou
/// Suíça ([0x9cf4ac]); na Alemanha, qualquer europeu (nação +0x71 = o continente [0x9cfa10] na tabela
/// [0xae23a4], 198 bytes); em [0x9cf360], todos; na Dinamarca, [0x9cf348], Noruega e [0x9cf3a0].
unsafe extern "C" fn pessoa_e_comunitaria(pessoa: usize, nacao_do_clube: usize) -> u32 {
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let g = |a: usize| *(a as *const u32);
    let n = ler(pessoa, 0x1a);
    if *((n + 0x7f) as *const u8) == 2 { return 1; } // nacionalidade da UE
    let id = g(n);
    if id == g(0x9cf4ac) {
        return 1;
    }
    let n2 = ler(pessoa, 0x1e);
    if n2 != 0 && *((n2 + 0x7f) as *const u8) == 2 {
        return 1;
    }
    if nacao_do_clube == 0 {
        return 0;
    }
    let c = g(nacao_do_clube);
    if c == g(0x9cf314) {
        let europa = (*(0xae23a4 as *const usize)).wrapping_add((g(0x9cfa10) as usize).wrapping_mul(0xc6));
        if ler(n, 0x71) == europa {
            return 1;
        }
    }
    if c == g(0x9cf360) {
        return 1;
    }
    if c == g(0x9cf2c0) && (id == g(0x9cf348) || id == g(0x9cf408) || id == g(0x9cf3a0)) {
        return 1;
    }
    0
}

unsafe extern "C" fn comparar_idade_da_pessoa(pessoa: usize, data: *const u16) -> u16 {
    let o = orig::<unsafe extern "C" fn(usize, *const u16) -> u16>(580);
    if pessoa == 0 {
        return o(pessoa, data); // caixa: só a original
    }
    let r = o(pessoa, data);
    let n = idade_da_pessoa(pessoa, data);
    PLACAR.registrar(r == n, || format!("idade_da_pessoa({:x}) original={} rust={}", pessoa, r, n));
    r
}

unsafe extern "C" fn comparar_nacao_com_transferencia_livre(nacao: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(581)(nacao);
    let n = nacao_com_transferencia_livre(nacao);
    PLACAR.registrar(r == n, || format!("nacao_com_transferencia_livre({:x}) original={} rust={}", nacao, r, n));
    r
}

unsafe extern "C" fn comparar_pessoa_e_comunitaria(pessoa: usize, nacao_do_clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u32>(582)(pessoa, nacao_do_clube);
    let n = pessoa_e_comunitaria(pessoa, nacao_do_clube);
    PLACAR.registrar(r == n, || format!("pessoa_e_comunitaria({:x}, {:x}) original={} rust={}", pessoa, nacao_do_clube, r, n));
    r
}

// ---------- 222a leva: se a pessoa está na lista de transferências e os atributos de jogador de 1 a 20 (Database.cpp) ----------

/// { 0x544520 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências, pelo clube ou a pedido (vínculo +0x4e, bits 1 e 8). Sem clube, o do
/// contrato (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id do clube na tabela [0xae23b4], 581
/// bytes por clube).
unsafe extern "C" fn pessoa_listada_para_transferencia(pessoa: usize, clube: usize) -> u32 {
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
    (v != 0 && *((v + 0x4e) as *const u8) & 9 != 0) as u32
}

/// A nota de 1 a 20 de um atributo relativo (de 0x543540, 0x544380 e 0x5442f0): x = v × 0,05 + 10;
/// (x² × 0,1 + x) × 1/3 + 0,5, entre 1 e 20, truncada (`_ftol`).
unsafe fn nota_do_atributo(v: f64) -> u8 {
    let x = v * f64_em(0x967888) + f64_em(0x9688b0);
    let z = (x * x * f64_em(0x967880) + x) * f64_em(0x968a80) + f64_em(0x968968); // arredonda
    let (minimo, maximo) = (f64_em(0x967890), f64_em(0x968ea8));
    ftol(if z > minimo {
        if z < maximo {
            z
        } else {
            maximo
        }
    } else {
        minimo
    }) as u8
}

/// { 0x543540 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador de 1 a 20. Uns são o byte do
/// registro de `jogador`; os relativos dão a nota (`nota_do_atributo`) de 2 × byte + habilidade atual
/// (+5), ou de 2 × byte + habilidade × 0,1 conforme o goleiro (+0xf: = 20 ou < 15); o 10 é o byte
/// +0x58 da `pessoa`; uns vão a 0x544380 (ou 0x5442f0). Sem pessoa, sem jogador ou atributo fora:
/// caixa "Error" e 0xff (a original). Só o al é devolvido.
unsafe extern "C" fn atributo_calculado_do_jogador(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(584)(pessoa, jogador, atributo);
    }
    // ponytail: a original empurra o registrador inteiro com o byte embaixo; as duas só leem o byte
    let relativo: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x544380usize);
    let relativo_goleiro: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(0x5442f0usize);
    let b = |d: usize| *((jogador + d) as *const i8);
    let byte = |d: usize| b(d) as u8;
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as i32;
    let nota = |d: usize| nota_do_atributo((2 * b(d) as i32 + habilidade) as f64);
    let nota_h = |d: usize| nota_do_atributo(habilidade as f64 * f64_em(0x967880) + (2 * b(d) as i32) as f64);
    let goleiro = b(0xf);
    match a {
        1 => byte(0x1b),
        2 => byte(0x1c),
        3 => byte(0x1d),
        4 => nota(0x1e),
        5 => byte(0x1f),
        6 => byte(0x20),
        7 => if goleiro == 20 { nota_h(0x43) } else { nota(0x43) },
        8 => if goleiro == 20 { nota_h(0x23) } else { nota(0x23) },
        9 => nota(0x24),
        10 => *((pessoa + 0x58) as *const u8),
        11 => if goleiro == 20 { nota_h(0x26) } else { nota(0x26) },
        12 => if goleiro == 20 { relativo_goleiro(jogador, byte(0x27) as u32) } else { relativo(jogador, byte(0x27) as u32) },
        13 => byte(0x28),
        14 => if goleiro < 15 { nota_h(0x2a) } else { nota(0x2a) },
        15 => nota(0x2b),
        16 => byte(0x2f),
        17 => byte(0x2e),
        18 => relativo(jogador, byte(0x31) as u32),
        19 => if goleiro == 20 { nota_h(0x32) } else { nota(0x32) },
        20 => if goleiro == 20 { nota_h(0x33) } else { nota(0x33) },
        21 => byte(0x36),
        22 => nota(0x37),
        23 => nota(0x39),
        24 => if goleiro < 15 { nota_h(0x3a) } else { relativo(jogador, byte(0x3a) as u32) },
        25 => byte(0x29),
        26 => byte(0x3c),
        27 => byte(0x3d),
        28 => relativo(jogador, byte(0x3e) as u32),
        29 => byte(0x3f),
        30 => byte(0x40),
        _ => byte(0x44),
    }
}

/// { 0x5431e0 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador da `pessoa`: 0x543540 com os
/// dados de jogador (+0x61). Sem pessoa ou atributo fora: caixa "Error" e 0xff (a original).
unsafe extern "C" fn atributo_da_pessoa(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return orig::<unsafe extern "C" fn(usize, u32) -> u8>(585)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543540usize);
    // a original empurra o argumento inteiro, como veio
    calcular(pessoa, core::ptr::read_unaligned((pessoa + 0x61) as *const usize), atributo)
}

unsafe extern "C" fn comparar_pessoa_listada_para_transferencia(pessoa: usize, clube: usize) -> u32 {
    // sem clube, as duas gravam o contrato em [0xae1080]; 0x4cf5a0 e 0x4cf5f0 só leem
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(583);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(pessoa, clube),
        || pessoa_listada_para_transferencia(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("pessoa_listada_para_transferencia({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_atributo_calculado_do_jogador(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(584);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return o(pessoa, jogador, atributo); // caixa: só a original
    }
    let r = o(pessoa, jogador, atributo);
    let n = atributo_calculado_do_jogador(pessoa, jogador, atributo);
    PLACAR.registrar(r == n, || format!("atributo_calculado_do_jogador({:x}, {}) original={} rust={}", jogador, a, r, n));
    r
}

unsafe extern "C" fn comparar_atributo_da_pessoa(pessoa: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u8>(585);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return o(pessoa, atributo); // caixa: só a original
    }
    let r = o(pessoa, atributo);
    let n = atributo_da_pessoa(pessoa, atributo);
    PLACAR.registrar(r == n, || format!("atributo_da_pessoa({:x}, {}) original={} rust={}", pessoa, a, r, n));
    r
}
