// ---------- %%LEVA%%a leva: as lesões: sortear a lesão recorrente, recomeçar o histórico de lesões e reiniciar o físico do jogador (injury.cpp) ----------

unsafe fn cpm5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 31 bytes do físico da pessoa `id` em [this] (+0x12 o tipo da lesão recorrente, 0xff sem; +0x13 o
/// nível; +0x14/+0x18 o começo e o último deslocamento do histórico de lesões).
unsafe fn injury_history_record(this: usize, id: usize) -> usize {
    cpm5_d(this).wrapping_add(id.wrapping_mul(31))
}

unsafe fn cpm5_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O que a 0x62e500 decide depois de sortear.
#[derive(Clone, Copy, PartialEq, Debug)]
enum RecurringInjuryOutcome {
    Nada,
    Marcada,
    Limpar(usize),
    Tratar(usize),
}

/// O plano da 0x62e500 (os sorteios e as marcas do registro; o resto fica para quem chama): 1 em 20 (sorteio 0x14) e
/// a lesão `tipo` podendo voltar ([0x9e7ae0 + 13 × tipo]), o registro sem recorrente fica com ela (+0x12 = tipo,
/// +0x13 = 0). Dos registros com recorrente (+0x12 ≠ 0xff), o de maior nota (sorteio 0x65 − 5 × nível − pessoa +0x2d
/// do jogador; 100 sem jogador, bytes com sinal), parando no primeiro com 95 ou mais. Achado: jogador de clube (+0x39)
/// de reputação (+0x80) acima de 4250, clube sem humano (0x600a50) e 1 em 3 (sorteio 3): tratar; senão limpar a marca
/// dele.
unsafe fn plan_recurring_injury(this: usize, registro: usize, lesao: *const u8) -> RecurringInjuryOutcome {
    if cpm5_b(registro + 0x12) != 0xff || cpm5_d((0x9e7ae0isize + *lesao as i8 as isize * 13) as usize) == 0 {
        return RecurringInjuryOutcome::Nada;
    }
    let s = cpm5_sorteio(0x14);
    if s != 0 { // 1 em 20
        return RecurringInjuryOutcome::Nada;
    }
    crate::comparison::branch("cpm_5 lesao recorrente");
    *((registro + 0x13) as *mut u8) = s as u8;
    *((registro + 0x12) as *mut u8) = *lesao;
    if *(0xae2364 as *const i32) <= 0 {
        return RecurringInjuryOutcome::Marcada;
    }
    let (mut melhor, mut escolhido, mut pessoa) = (0i8, 0usize, 0usize);
    let mut r = cpm5_d(this);
    let mut i = 0i32;
    while i < *(0xae2364 as *const i32) {
        if cpm5_b(r + 0x12) != 0xff {
            let p = cpm5_d(0xae23bc).wrapping_add(i as usize * 0x6e);
            let v = cpm5_sorteio(0x65) as u8;
            let jogador = cpm5_d(p + 0x61);
            let nota = if jogador == 0 { 0x64 } else { v.wrapping_add(cpm5_b(r + 0x13).wrapping_mul(0xfb)).wrapping_sub(cpm5_b(jogador + 0x2d)) };
            if nota as i8 > melhor {
                (melhor, escolhido, pessoa) = (nota as i8, r, p);
            }
            if melhor >= 0x5f { // forte o bastante: para
                crate::comparison::branch("cpm_5 recorrente forte");
                break;
            }
        }
        i += 1;
        r += 0x1f;
    }
    if escolhido == 0 {
        return RecurringInjuryOutcome::Marcada;
    }
    let clube = cpm5_d(pessoa + 0x39);
    if cpm5_d(pessoa + 0x61) != 0 && clube != 0 && core::ptr::read_unaligned((clube + 0x80) as *const i16) > 0x109a {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) == 0 && cpm5_sorteio(3) == 0 && pessoa != 0 {
            return RecurringInjuryOutcome::Tratar(pessoa);
        }
    }
    RecurringInjuryOutcome::Limpar(escolhido)
}

/// { 0x62e500 } (thiscall, ret 8, injury.cpp): `Injury::roll_recurring_injury(registro, lesao)`: o plano
/// (`plan_recurring_injury`); tratar: a lesão nova da pessoa (0x62d250 com 0x4e, 0x4f ou 0x4d pelo tipo recorrente do registro
/// dela, 0xff fora das pessoas ou sem jogador; o tipo vai no byte baixo do argumento `registro`, como a original);
/// limpar: o registro escolhido sem recorrente (+0x12 = 0xff, +0x13 = 0).
unsafe extern "thiscall" fn roll_recurring_injury(this: usize, registro: usize, lesao: *const u8) {
    match plan_recurring_injury(this, registro, lesao) {
        RecurringInjuryOutcome::Tratar(p) => {
            let id = cpm5_d(p) as i32;
            let tipo = if id < *(0xae2364 as *const i32) && cpm5_d(p + 0x61) != 0 {
                cpm5_b(injury_history_record(this, id as usize) + 0x12)
            } else {
                0xff
            };
            let nova: u8 = match tipo {
                0x0b | 0x11 | 0x17 | 0x1e | 0x23 | 0x29 => 0x4e,
                0x0c | 0x0d | 0x12 | 0x14 | 0x19..=0x1c | 0x1f | 0x24 | 0x25 | 0x2c | 0x2d | 0x3b => 0x4f,
                _ => 0x4d,
            };
            let injure: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x62d250usize);
            injure(this, (registro as u32 & 0xffff_ff00) | nova as u32, 0, p, 0, 1, -1, 0, 0);
        }
        RecurringInjuryOutcome::Limpar(r) => {
            *((r + 0x12) as *mut u8) = 0xff;
            *((r + 0x13) as *mut u8) = 0;
        }
        _ => {}
    }
}

/// { 0x631f90 } (thiscall, ret 4, injury.cpp): `Injury::reset_player_history(pessoa)`: o histórico de lesões da pessoa
/// (registro de 31 bytes do id em [this]) fica sem o último (0x58f110 sobre [this+0xc]: +0x18 = −1). Devolve o que a
/// 0x58f110 devolve. Pessoa nula: a original (caixa fatal).
unsafe extern "thiscall" fn reset_player_history(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, pessoa); // caixa fatal: só a original
    }
    let r = injury_history_record(this, cpm5_d(pessoa));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x58f110usize);
    sem_ultimo(cpm5_d(this + 0xc), r + 0x14, r + 0x18)
}

/// { 0x62bfa0 } (thiscall, ret 4, injury.cpp): `Injury::reset_player(id)`: tira da lista [this+0x28] (do começo: o
/// cursor +0xa volta a +2) cada item do id (0x68d1f0); com lesão (+0xe ≠ −1), termina (0x62e700, sem notícia);
/// recomeça o histórico (0x631f90); o registro de 31 bytes recomeça: data de hoje (0x417c10 de [0xae2c90]), condição
/// e forma 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1 (+0x14,
/// +0x18), +0x1d 0xff. O bit 0 do +0xc do registro de 79 bytes ([0xae2d0c]) cai. Devolve o endereço desse byte.
unsafe extern "thiscall" fn injury_reset_player(this: usize, id: u32) -> usize {
    crate::comparison::branch("cpm_5 reiniciar jogador");
    if cpm5_d(this + 0x28) != 0 {
        let l = cpm5_d(this + 0x28);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpm5_d(l + 2));
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
        let mut e = proximo(cpm5_d(this + 0x28));
        while e != 0 {
            if cpm5_d(e) as u32 == id {
                tirar(cpm5_d(this + 0x28), e);
            }
            e = proximo(cpm5_d(this + 0x28));
        }
    }
    let r = injury_history_record(this, id as usize);
    let pessoa = cpm5_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if cpm5_d(r + 0xe) as i32 != -1 {
        let terminar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62e700usize);
        terminar(this, pessoa, 0);
    }
    let recomecar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x631f90usize);
    recomecar(this, cpm5_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut rascunho = [0u8; 8];
    copiar(r, rascunho.as_mut_ptr(), 0xae2c90); // a data de hoje
    core::ptr::write_unaligned((r + 8) as *mut u16, 10000);
    core::ptr::write_unaligned((r + 0xa) as *mut u16, 10000);
    core::ptr::write_unaligned((r + 0xc) as *mut i16, 0); // sem desgaste
    core::ptr::write_unaligned((r + 0xe) as *mut i32, -1);
    *((r + 0x12) as *mut u8) = 0xff;
    core::ptr::write_unaligned((r + 0x14) as *mut i32, -1);
    core::ptr::write_unaligned((r + 0x18) as *mut i32, -1);
    *((r + 0x1d) as *mut u8) = 0xff;
    let g = cpm5_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    *(g as *mut u8) &= 0xfe;
    g
}

/// Comparador: a 0x62d250 (tratar) não pode rodar duas vezes: o plano antes, com o estado do sorteio guardado (sem
/// contar nem rastrear) e as marcas do registro devolvidas; a original roda. Confere as marcas do registro e do
/// escolhido e, fora do tratar, o estado do sorteio igual ao do plano (os mesmos sorteios).
unsafe extern "thiscall" fn compare_roll_recurring_injury(this: usize, registro: usize, lesao: *const u8) {
    let estado = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let marcas = (cpm5_b(registro + 0x12), cpm5_b(registro + 0x13));
    let plano = crate::rng::pause_trace(|| crate::rng::without_counting(|| plan_recurring_injury(this, registro, lesao)));
    let depois_do_plano = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    estado.write_back();
    *((registro + 0x12) as *mut u8) = marcas.0;
    *((registro + 0x13) as *mut u8) = marcas.1;
    let tipo = *lesao;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8)>(%%I0%%)(this, registro, lesao);
    let agora = (cpm5_b(registro + 0x12), cpm5_b(registro + 0x13));
    let mesmo_sorteio = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS) == depois_do_plano;
    let ok = match plano {
        RecurringInjuryOutcome::Nada => agora == marcas && mesmo_sorteio,
        RecurringInjuryOutcome::Marcada => agora == (tipo, 0) && mesmo_sorteio,
        RecurringInjuryOutcome::Limpar(r) => {
            (cpm5_b(r + 0x12), cpm5_b(r + 0x13)) == (0xff, 0) && (r == registro || agora == (tipo, 0)) && mesmo_sorteio
        }
        RecurringInjuryOutcome::Tratar(_) => true, // a 0x62d250 sorteia e grava o resto
    };
    SCOREBOARD.record(ok, || {
        format!("roll_recurring_injury(tipo {}) plano {:x?}, marcas {:?} -> {:?}, mesmo sorteio={}", tipo, plano, marcas, agora, mesmo_sorteio)
    });
}

/// Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn compare_reset_player_history(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = injury_history_record(this, cpm5_d(pessoa));
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r + 0x14, 8)], || o(this, pessoa), || reset_player_history(this, pessoa));
    SCOREBOARD.record(igual && a == b, || format!("reset_player_history({}) {:x?} x {:x?}", cpm5_d(pessoa), mo, mr));
    a
}

/// Comparador: a 0x68d1f0 libera o item tirado e a 0x62e700 grava o histórico no arquivo: o plano antes (os itens
/// que ficam, o byte do registro de 79 bytes, os bytes do registro que não mudam); a original roda; confere.
unsafe extern "thiscall" fn compare_injury_reset_player(this: usize, id: u32) -> usize {
    let l = cpm5_d(this + 0x28);
    let esperado: Vec<usize> = if l != 0 { list_items_capped(l).into_iter().filter(|&e| cpm5_d(e) as u32 != id).collect() } else { Vec::new() };
    let r = injury_history_record(this, id as usize);
    let g = cpm5_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xc;
    let (byte, nivel, b1c) = (cpm5_b(g) & 0xfe, cpm5_b(r + 0x13), cpm5_b(r + 0x1c));
    let ret = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, id);
    let itens = if l != 0 { list_items_capped(l) } else { Vec::new() };
    let w = |o: usize| core::ptr::read_unaligned((r + o) as *const i16);
    let dd = |o: usize| cpm5_d(r + o) as i32;
    let ok = itens == esperado && ret == g && cpm5_b(g) == byte
        && cpm5_d(r) == cpm5_d(0xae2c90) && cpm5_d(r + 4) == cpm5_d(0xae2c94)
        && w(8) == 10000 && w(0xa) == 10000 && w(0xc) == 0 && dd(0xe) == -1 && cpm5_b(r + 0x12) == 0xff
        && cpm5_b(r + 0x13) == nivel && dd(0x14) == -1 && dd(0x18) == -1 && cpm5_b(r + 0x1c) == b1c && cpm5_b(r + 0x1d) == 0xff;
    SCOREBOARD.record(ok, || {
        let v: Vec<u8> = (0..31).map(|o| cpm5_b(r + o)).collect();
        format!("injury_reset_player({}) itens {} (esperava {}), byte {:#x} (esperava {:#x}), registro {:02x?}", id, itens.len(), esperado.len(), cpm5_b(g), byte, v)
    });
    ret
}
