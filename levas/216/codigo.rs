// ---------- 216a leva: se a liga tem rebaixamento, a data da rodada do modelo da liga e a importância do jogo da liga (league.cpp) ----------

/// { 0x6847b0 } (thiscall, league.cpp): 1 se a liga tem vagas de rebaixamento (byte +0xc1 não nulo).
unsafe extern "thiscall" fn liga_tem_rebaixamento(liga: usize) -> u32 {
    (*((liga + 0xc1) as *const u8) != 0) as u32 // tem rebaixamento
}

/// { 0x68a160 } (cdecl, league.cpp): a data (Date::Date, 0x549f70, num Date local) de `dia`/`mes`/
/// (`anos` + `base`)/`semana` no registro `i` (palavra com sinal, 0x41 bytes) de `rodadas`: dia do ano
/// na palavra +0, ano − `base` na palavra +2, `codigo` no byte +4 e `valor` em +0x3d. Devolve o registro.
unsafe extern "C" fn data_da_rodada_da_liga(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u16; 4];
    construir(&mut d, dia, mes, anos.wrapping_add(base), semana);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    core::ptr::write_unaligned(r as *mut u16, d[0]);
    core::ptr::write_unaligned((r + 2) as *mut u16, d[1].wrapping_sub(base as u16)); // ano relativo
    *((r + 4) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((r + 0x3d) as *mut u32, valor);
    r
}

/// { 0x68a3e0 } (thiscall, ret 0xc, league.cpp): a importância do jogo da `partida` para o clube do
/// `lado` (0 = casa, +0x1c; senão +0x20): (byte +0x51 da liga − 10) / 4 + 10. Grava 0xff em `saida`;
/// com o clube na tabela (+0xb1, linhas de 59 bytes, +0x3e clubes) e mais de 10 rodadas (+0xa9): +1
/// com +0x34 da partida abaixo de 3; senão, linha sem marca (+0x37 = 0xff), +1 e a margem do último
/// dia (0x68a4d0, que pode trocar a importância) em `saida`. Só o al é devolvido (quem chama usa al).
unsafe extern "thiscall" fn importancia_do_jogo_da_liga(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    let margem: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x68a4d0usize);
    *saida = 0xff;
    let clube = *((partida + if lado as u8 == 0 { 0x1c } else { 0x20 }) as *const usize);
    let mut imp = ((*((liga + 0x51) as *const i8) as i32 - 10) / 4) as u8;
    imp = imp.wrapping_add(10); // importância base
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let linha = |i: i16| (*((liga + 0xb1) as *const usize)).wrapping_add((i as isize * 59) as usize);
    // ponytail: a original testa i == −1 depois da busca; i começa em 0 e para antes de `clubes`, nunca é −1
    let mut i = 0i16;
    loop {
        if i >= clubes {
            return imp;
        }
        if *(linha(i) as *const usize) == clube {
            break;
        }
        i = i.wrapping_add(1);
    }
    if core::ptr::read_unaligned((liga + 0xa9) as *const i16) <= 10 {
        return imp;
    }
    if core::ptr::read_unaligned((partida + 0x34) as *const i16) < 3 {
        return imp.wrapping_add(1);
    }
    let r = linha(i);
    if *((r + 0x37) as *const u8) != 0xff {
        return imp;
    }
    let mut local = imp.wrapping_add(1);
    *saida = margem(liga, r, &mut local);
    local
}

unsafe extern "thiscall" fn comparar_liga_tem_rebaixamento(liga: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(545)(liga);
    let n = liga_tem_rebaixamento(liga);
    PLACAR.registrar(r == n, || format!("liga_tem_rebaixamento original={} rust={}", r, n));
    r
}

unsafe extern "C" fn comparar_data_da_rodada_da_liga(rodadas: usize, i: u32, dia: u32, mes: u32, anos: u32, semana: u32, codigo: u32, base: u32, valor: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(546);
    let r = rodadas.wrapping_add((i as i16 as isize * 0x41) as usize);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x41)], || o(rodadas, i, dia, mes, anos, semana, codigo, base, valor),
        || data_da_rodada_da_liga(rodadas, i, dia, mes, anos, semana, codigo, base, valor));
    PLACAR.registrar(igual && a == b, || format!("data_da_rodada_da_liga({}, {}/{} +{}) {:x?} x {:x?}", i as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "thiscall" fn comparar_importancia_do_jogo_da_liga(liga: usize, partida: usize, lado: u32, saida: *mut u8) -> u8 {
    // a 0x68a4d0 só lê a liga e grava na local e na `saida`: pode rodar nos dois lados
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u8>(547);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 1)], || o(liga, partida, lado, saida),
        || importancia_do_jogo_da_liga(liga, partida, lado, saida));
    PLACAR.registrar(igual && r == n, || format!("importancia_do_jogo_da_liga original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 216a leva: as cópias do recordista, do registro de partida e do registro de sequência dos recordes do clube (club_records.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local.
unsafe fn copiar_data_do_recorde(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x448490 } (thiscall, ret 4, club_records.cpp): copia o recordista (0x18 bytes) de `origem`:
/// quatro palavras duplas de identificação (+0..+0xc) e a data em +0x10. Devolve `this`.
unsafe extern "thiscall" fn copiar_recordista(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    d(0); // identificação
    d(4);
    d(8);
    d(0xc);
    copiar_data_do_recorde(this + 0x10, origem + 0x10);
    this
}

/// { 0x448110 } (thiscall, ret 4, club_records.cpp): copia o registro de partida (0x1f bytes) de
/// `origem`: a data em +0, depois +8 (dupla), +0xc e +0xe (palavras), +0x10 (byte), +0x11 e +0x15
/// (duplas), +0x19 e +0x1a (bytes) e +0x1b (dupla). Devolve `this`.
unsafe extern "thiscall" fn copiar_registro_de_partida(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    copiar_data_do_recorde(this, origem);
    d(8); // adversário
    w(0xc);
    w(0xe);
    b(0x10);
    d(0x11);
    d(0x15);
    b(0x19);
    b(0x1a);
    d(0x1b);
    this
}

/// { 0x448170 } (thiscall, ret 4, club_records.cpp): copia o registro de sequência (0x14 bytes) de
/// `origem`: o número de jogos (+0) e as datas de início (+4) e fim (+0xc). Devolve `this`.
unsafe extern "thiscall" fn copiar_registro_de_sequencia(this: usize, origem: usize) -> usize {
    *(this as *mut u32) = *(origem as *const u32); // jogos
    copiar_data_do_recorde(this + 4, origem + 4);
    copiar_data_do_recorde(this + 0xc, origem + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_copiar_recordista(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(548);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x18)], || o(this, origem), || copiar_recordista(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recordista {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_registro_de_partida(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(549);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1f)], || o(this, origem), || copiar_registro_de_partida(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_registro_de_partida {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_registro_de_sequencia(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(550);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this, origem), || copiar_registro_de_sequencia(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_registro_de_sequencia {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 216a leva: a partida assistida por algum humano, o apelido dado pelo humano e a cópia da aposentadoria pendente (human_manager.cpp, player_regen.cpp) ----------

/// { 0x5fe420 } (thiscall, ret 4, human_manager.cpp): 1 se algum dos 16 técnicos humanos ativos
/// (registros de 110 bytes em [0xae23bc], a partir de [0xae2364] − 16, +0x5f não nulo) escolheu
/// assistir a `partida` (0x5fe290); senão 0. Partida nula: a original (caixa de erro e 0).
unsafe extern "thiscall" fn humano_assiste_a_partida(this: usize, partida: usize) -> u32 {
    if partida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(551)(this, partida);
    }
    let assiste: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5fe290usize);
    for k in 0..16i32 {
        let i = k.wrapping_add(*(0xae2364 as *const i32)).wrapping_sub(0x10);
        let h = (*(0xae23bc as *const usize)).wrapping_add((i as isize * 110) as usize);
        if *((h + 0x5f) as *const u8) != 0 && assiste(this, partida, h) != 0 {
            return 1;
        }
    }
    0 // ninguém assiste
}

/// O índice (0..[this+4]) do técnico humano `humano` (ou o da vez, [0xb74c26 + 0x300 × [0xb77c7a]])
/// para a 0x600740: None se não há humano; Some(None) fora da faixa (caso de erro da original).
unsafe fn indice_do_humano_do_apelido(this: usize, humano: usize) -> Option<Option<usize>> {
    let h = if humano != 0 {
        humano
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return None;
    }
    let i = (*(h as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Some(None);
    }
    Some(Some(i as usize))
}

/// { 0x600740 } (thiscall, ret 8, human_manager.cpp): 1 se o técnico humano `humano` (0 = o da vez)
/// deu apelido à pessoa de id [`pessoa`]: busca binária (0x946464 com 0x601a10, itens de 0x1e bytes)
/// na lista +0xa6e3 (quantos em +0xa6eb) do registro dele (0xa6ef bytes em [this]). Pessoa ou humano
/// nulos, ou lista vazia: 0. Humano fora da faixa: a original (caixa de erro e 0).
unsafe extern "thiscall" fn humano_deu_apelido(this: usize, pessoa: usize, humano: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let i = match indice_do_humano_do_apelido(this, humano) {
        None => return 0,
        Some(None) => return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(552)(this, pessoa, humano),
        Some(Some(i)) => i,
    };
    let buscar: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = (*(this as *const usize)).wrapping_add(i.wrapping_mul(0xa6ef));
    let lista = core::ptr::read_unaligned((r + 0xa6e3) as *const usize);
    // a chave: o id e um byte 0 (a 0x601a10 só compara o id)
    let chave = [*(pessoa as *const u32), 0u32];
    let achou = if lista == 0 { 0 } else { (buscar(chave.as_ptr(), lista, core::ptr::read_unaligned((r + 0xa6eb) as *const u32), 0x1e, 0x601a10) != 0) as u32 };
    achou // apelido
}

/// { 0x7a8780 } (thiscall, ret 4, player_regen.cpp): copia a aposentadoria pendente (0x1a bytes) de
/// `origem`: o id da pessoa (+0) e do clube (+4), as datas em +8 e +0x10 (0x417c10, com o retorno numa
/// local), o motivo (+0x18) e a marca de anunciada (+0x19). Devolve `this`.
unsafe extern "thiscall" fn copiar_aposentadoria_pendente(this: usize, origem: usize) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    *(this as *mut u32) = *(origem as *const u32);
    *((this + 4) as *mut u32) = *((origem + 4) as *const u32);
    copiar(this + 8, &mut local, origem + 8);
    copiar(this + 0x10, &mut local, origem + 0x10);
    *((this + 0x18) as *mut u8) = *((origem + 0x18) as *const u8); // motivo
    *((this + 0x19) as *mut u8) = *((origem + 0x19) as *const u8);
    this
}

unsafe extern "thiscall" fn comparar_humano_assiste_a_partida(this: usize, partida: usize) -> u32 {
    // a 0x5fe290 só lê: pode rodar nos dois lados
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(551);
    if partida == 0 {
        return o(this, partida);
    }
    let r = o(this, partida);
    let n = humano_assiste_a_partida(this, partida);
    PLACAR.registrar(r == n, || format!("humano_assiste_a_partida original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_humano_deu_apelido(this: usize, pessoa: usize, humano: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(552);
    if pessoa != 0 && matches!(indice_do_humano_do_apelido(this, humano), Some(None)) {
        return o(this, pessoa, humano);
    }
    let r = o(this, pessoa, humano);
    let n = humano_deu_apelido(this, pessoa, humano);
    PLACAR.registrar(r == n, || format!("humano_deu_apelido original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_copiar_aposentadoria_pendente(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(553);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x1a)], || o(this, origem), || copiar_aposentadoria_pendente(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_aposentadoria_pendente {:x?} x {:x?}", mo, mr));
    r
}
