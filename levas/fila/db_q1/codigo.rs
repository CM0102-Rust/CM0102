// ---------- %%LEVA%%a leva: o alcance da reputação, a reputação vista por alguém e se a pessoa é estrangeira (Database.cpp) ----------

/// { 0x5408c0 } (cdecl, Database.cpp): de que distância `quem_ve` olha a `pessoa`, o que escolhe a
/// reputação: 0 = mesmo clube ou clubes da mesma nação (+0x39, clube +0x53) → a nacional; 2 = mesma
/// nação (+0x1a) ou não estrangeiro nela (0x53d980) → a atual; 3 = não estrangeiro na nação do clube
/// da pessoa, ou reputação de jogador (+0x61, +9) + reputação do clube de quem vê (+0x80) ≥ 14000 →
/// a média; 4 (sem quem vê, quem vê entre os 16 últimos ids = técnicos humanos, sem dados ou sem
/// clube) → a mundial. Só o al é devolvido (quem chama usa al).
unsafe extern "C" fn database_get_reputation_scope(pessoa: usize, quem_ve: usize) -> u8 {
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
unsafe extern "C" fn database_get_staff_reputation_for_viewer(pessoa: usize, quem_ve: usize, nao_jogador: u32) -> u16 {
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
unsafe extern "C" fn database_is_staff_foreign(pessoa: usize, nacao: usize, grupos: u32) -> u32 {
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
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
        let contrato = contract_of(*(0xae1050 as *const usize), pessoa);
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

unsafe extern "C" fn comparar_database_get_reputation_scope(pessoa: usize, quem_ve: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(%%I0%%)(pessoa, quem_ve);
    let n = database_get_reputation_scope(pessoa, quem_ve);
    PLACAR.registrar(r == n, || format!("database_get_reputation_scope({:x}, {:x}) original={} rust={}", pessoa, quem_ve, r, n));
    r
}

unsafe extern "C" fn comparar_database_get_staff_reputation_for_viewer(pessoa: usize, quem_ve: usize, nao_jogador: u32) -> u16 {
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u16>(%%I1%%)(pessoa, quem_ve, nao_jogador);
    let n = database_get_staff_reputation_for_viewer(pessoa, quem_ve, nao_jogador);
    PLACAR.registrar(r == n, || format!("database_get_staff_reputation_for_viewer({:x}, {:x}, {}) original={} rust={}", pessoa, quem_ve, nao_jogador, r, n));
    r
}

unsafe extern "C" fn comparar_database_is_staff_foreign(pessoa: usize, nacao: usize, grupos: u32) -> u32 {
    // a 0x4cf5a0 e o Date (num Date local) só leem: os dois lados podem rodar
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u32>(%%I2%%)(pessoa, nacao, grupos);
    let n = database_is_staff_foreign(pessoa, nacao, grupos);
    PLACAR.registrar(r == n, || format!("database_is_staff_foreign({:x}, {:x}, {}) original={} rust={}", pessoa, nacao, grupos, r, n));
    r
}
