// ---------- %%LEVA%%a leva: as regras da partida e o amistoso: o alcance das regras de escalação da partida, o motivo de um jogador não poder jogar e a vaga do amistoso marcado ----------

unsafe fn cpk4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x8d2fa0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::get_fixture_rules_scope(partida, tipo,
/// nação, continente)`: competição da partida (+0x14) de reservas ([0x9cf684]): a nação do mandante (+0x1c, +0x53), o
/// continente dela (+0x71, ou 0) e tipo 1. Amistoso (0x5c6710 sobre [0xae2c74]): 0, 0 e 0xff. Senão, pela nação-chave
/// da competição (0x669470, com sinal): a nação da tabela 0xb63d60 (72 bytes cada), o continente da competição (+0x59)
/// e tipo 4 (chaves 0 e 1), 2 (3 a 7) ou 1.
unsafe extern "thiscall" fn get_fixture_rules_scope(_this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let comp = cpk4_d(partida + 0x14);
    if cpk4_d(comp) == cpk4_d(0x9cf684) {
        let n = cpk4_d(cpk4_d(partida + 0x1c) + 0x53);
        *nacao = n;
        *continente = if n != 0 { cpk4_d(n + 0x71) } else { 0 };
        *tipo = 1;
        return;
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    if amistoso(cpk4_d(0xae2c74), comp) != 0 {
        *nacao = 0;
        *continente = 0;
        *tipo = 0xff;
        return;
    }
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    let k = chave(comp) as i8;
    *nacao = cpk4_d((0xb63d60isize + k as isize * 72) as usize);
    *continente = cpk4_d(cpk4_d(partida + 0x14) + 0x59);
    *tipo = match k {
        0 | 1 => 4,
        3..=7 => 2,
        _ => {
            crate::comparison::branch("cpk_4 nation scope");
            1 // a nação da competição
        }
    };
}

/// { 0x8cf5e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_ineligibility_code(jogador, partida)`:
/// mandante (+0x1c) que não é seleção (0x53b900) e jogador ausente sem licença (AWOL: 0x418f60 sobre [0xad9c6c], com 0,
/// que tira a marca de quem está sem clube): 9. Senão, pelo alcance das regras (0x8d2fa0; 0xff e 4: 0): a primeira das
/// 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo (+0x12) responde (virtual
/// +0x18 com jogador e partida). Nenhuma: 0.
unsafe extern "thiscall" fn get_ineligibility_code(this: usize, jogador: usize, partida: usize) -> u8 {
    if partida != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(cpk4_d(partida + 0x1c)) == 0 {
            let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
            if ausente(cpk4_d(0xad9c6c), jogador, 0) != 0 {
                return 9;
            }
        }
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(0x8d2fa0usize);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff || tipo == 4 {
        return 0;
    }
    for k in 0..28 {
        let r = cpk4_d(this + 0x8ac + k * 4);
        let casa = if tipo == 2 {
            cpk4_d(r + 0xe) == continente && *((r + 0x12) as *const u8) == 2
        } else {
            cpk4_d(r + 0xa) == nacao && *((r + 0x12) as *const u8) == tipo // a regra da nação
        };
        if casa {
            crate::comparison::branch("cpk_4 match rule");
            let respond: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(cpk4_d(cpk4_d(r) + 0x18));
            return respond(r, jogador, partida);
        }
    }
    0
}

/// { 0x5c6740 } (thiscall, ret 0x10, friendly.cpp): `Friendly::find_scheduled_friendly_slot(dia, ano, clube,
/// adversário)`: na lista dos amistosos (+0x34cc; o cursor volta ao começo), o primeiro registro do clube (+0) com uma
/// vaga (25 bytes a partir de +9, [+8] delas, com sinal) do dia (+0), ano (+2) e adversário (+0x10): a vaga. Senão 0.
unsafe extern "thiscall" fn find_scheduled_friendly_slot(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    // ponytail: a Data local da original (0x549ef0) só guarda o dia e o ano em 16 bits (o bissexto não é lido)
    let (dia, ano) = (dia as u16, ano as u16);
    let l = cpk4_d(this + 0x34cc);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpk4_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(l);
    while e != 0 {
        if cpk4_d(e) == clube {
            for i in 0..(*((e + 8) as *const i8)).max(0) as usize {
                let v = e + 25 * i + 9;
                if *((v + 2) as *const u16) == ano && *(v as *const u16) == dia && cpk4_d(v + 0x10) == adversario {
                    crate::comparison::branch("cpk_4 friendly slot");
                    return e + 25 * i + 9; // a vaga i
                }
            }
        }
        e = proximo(l);
    }
    0
}

/// Comparador: lado a lado pelas três saídas (a 0x669470 e a 0x5c6710 só leem).
unsafe extern "thiscall" fn compare_get_fixture_rules_scope(this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize)>(%%I0%%);
    let regioes = [(tipo as usize, 1), (nacao as usize, 4), (continente as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regioes, || o(this, partida, tipo, nacao, continente), || get_fixture_rules_scope(this, partida, tipo, nacao, continente));
    SCOREBOARD.record(igual, || format!("get_fixture_rules_scope({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparador: pelo valor. Jogador com a marca de ausente (0x418f60 com 1, que só lê): só a original (com 0 ela tira
/// a marca de quem está sem clube e a segunda chamada veria outro estado).
unsafe extern "thiscall" fn compare_get_ineligibility_code(this: usize, jogador: usize, partida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I1%%);
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    if ausente(cpk4_d(0xad9c6c), jogador, 1) != 0 {
        return o(this, jogador, partida);
    }
    let r = o(this, jogador, partida);
    let n = get_ineligibility_code(this, jogador, partida);
    SCOREBOARD.record(r == n, || format!("get_ineligibility_code({:x}, {:x}) original={} rust={}", jogador, partida, r, n));
    r
}

/// Comparador: pelo valor (as duas voltam o cursor da lista ao começo e param no mesmo registro).
unsafe extern "thiscall" fn compare_find_scheduled_friendly_slot(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize>(%%I2%%)(this, dia, ano, clube, adversario);
    let n = find_scheduled_friendly_slot(this, dia, ano, clube, adversario);
    SCOREBOARD.record(r == n, || format!("find_scheduled_friendly_slot({}/{}, {:x}, {:x}) original={:#x} rust={:#x}", dia as u16, ano as u16, clube, adversario, r, n));
    r
}
