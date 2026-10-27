// ---------- %%LEVA%%a leva: a copa: os mandantes da rodada e o prêmio da rodada (Cup.cpp) ----------
// O prêmio usa a escuta do item cpb_1 (cpb1_*), que vem antes na fila.

/// Troca mandante e visitante no confronto `t` (0x35 bytes): os pares de palavras +0/+2, +0xa/+0xc e +0xe/+0x10.
unsafe fn cpb3_inverter(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xa, 0xc), (0xe, 0x10)] {
        let (x, y) = (cpb1_u16(t + a), cpb1_u16(t + b));
        core::ptr::write_unaligned((t + a) as *mut u16, y);
        core::ptr::write_unaligned((t + b) as *mut u16, x);
    }
}

/// Os confrontos [início, fim) que a 0x51e870 percorre: um só (`laco`) ou, com −1, os da rodada (+0x1a).
unsafe fn cpb3_faixa(copa: usize, rodada: u32, laco: u32) -> (i16, i16) {
    if laco as u16 != 0xffff {
        (laco as i16, (laco as u16).wrapping_add(1) as i16)
    } else {
        (0, cpb1_i16(cpb1_em(cpb1_us(copa + 0xa3), rodada as i16 as i32, 0x68) + 0x1a))
    }
}

/// { 0x51e870 } (thiscall ret 8, Cup.cpp): `Cup::set_home_teams_for_round(rodada, laco)`: em um confronto (`laco`) ou em
/// todos os da rodada (−1), com os dois times definidos, inverte mandante e visitante pela regra da rodada (+0xb):
/// bit 1, se o segundo clube é o de divisão mais alta (posição 0x90 da vtable, club_in_higher_division); bit 8, se o
/// nível (+4 do time) do primeiro é menor que o do segundo; senão, com a nação da copa (+0x24) ≠ −1, se o segundo clube
/// é dessa nação (+0x69 → id).
unsafe extern "thiscall" fn copa_mandantes_da_rodada(copa: usize, rodada: u32, laco: u32) {
    let di = rodada as i16 as i32;
    let (ini, fim) = cpb3_faixa(copa, rodada, laco);
    let regra = cpb1_u16(cpb1_em(cpb1_us(copa + 0xa3), di, 0x68) + 0xb);
    if regra & 1 == 0 && regra & 8 == 0 && cpb1_u32(copa + 0x24) == u32::MAX {
        return;
    }
    for k in ini as i32..fim as i32 {
        let t = cpb1_em(cpb1_us(cpb1_em(cpb1_us(copa + 0xad), di, 4)), k, 0x35);
        let (a, b) = (cpb1_i16(t), cpb1_i16(t + 2));
        if a == -1 || b == -1 {
            continue;
        }
        let time = |x: i16| cpb1_em(cpb1_us(copa + 0xa7), x as i32, 6);
        let inverter = if regra & 1 != 0 {
            let (ca, cb) = (cpb1_u32(time(a)), cpb1_u32(time(b)));
            let f = cpb1_us(cpb1_us(copa) + 0x90);
            let maior = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(f)(copa, ca, cb);
            maior != 0 && maior == cb // o de divisão mais alta
        } else if regra & 8 != 0 {
            (cpb1_u8(time(a) + 4) as i8) < (cpb1_u8(time(b) + 4) as i8) // o nível
        } else {
            let cb = cpb1_us(time(b));
            cb != 0 && cpb1_us(cb + 0x69) != 0 && cpb1_u32(cpb1_us(cb + 0x69)) == cpb1_u32(copa + 0x24) // a nação
        };
        if inverter {
            cpb3_inverter(t);
        }
    }
}

unsafe extern "thiscall" fn comparar_copa_mandantes_da_rodada(copa: usize, rodada: u32, laco: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I0%%);
    let (ini, fim) = cpb3_faixa(copa, rodada, laco);
    let mut reg = Vec::new();
    if ini < fim {
        let base = cpb1_us(cpb1_em(cpb1_us(copa + 0xad), rodada as i16 as i32, 4));
        reg.push((cpb1_em(base, ini as i32, 0x35), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(copa, rodada, laco), || copa_mandantes_da_rodada(copa, rodada, laco));
    PLACAR.registrar(igual, || format!("copa_mandantes_da_rodada(rodada {}, laco {}) {:x?} x {:x?}", rodada as i16, laco as i16, mo, mr));
}

/// { 0x522240 } (thiscall ret 0x10, Cup.cpp): `Cup::pay_round_prize_money(vencedor, perdedor, rodada, partida)`: o
/// prêmio da rodada (+0x60 o do vencedor, +0x64 o do perdedor); −1: o fundo (+0x28) / (rodadas − rodada) / confrontos
/// (+0x1a), e sem o do perdedor 3/4 para o vencedor e 1/4 para o perdedor. O vencedor recebe a notícia (0x48e3e0:
/// clube, valor de 64 bits, −1, a rodada seguinte +0x6f se não é a última nem 0x8c, partida, o desfecho +5 do time, 3
/// se era 1 na partida 0x136) e o dinheiro (0x5999a0 nas finanças 0x5a0590); o perdedor só o dinheiro; o fundo perde
/// os dois. O desfecho vai sem os bits altos (a original empurra o dword de uma variável de que só grava o byte; a
/// notícia lê o byte).
unsafe extern "thiscall" fn copa_premio_da_rodada(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let mut desfecho = cpb1_u8(vencedor + 5);
    if cpb1_u16(partida + 0x30) == 0x136 && desfecho == 1 {
        desfecho = 3;
    }
    let r = rodada as i16 as i32;
    let entrada = cpb1_em(cpb1_us(copa + 0xa3), r, 0x68);
    let mut ao_vencedor = cpb1_u32(entrada + 0x60) as i32;
    let mut ao_perdedor = cpb1_u32(entrada + 0x64) as i32;
    if ao_vencedor == -1 {
        let q = (cpb1_u32(copa + 0x28) as i32 / (cpb1_i16(copa + 0x3c) as i32 - r)) / cpb1_i16(entrada + 0x1a) as i32; // do fundo
        if ao_perdedor == -1 {
            ao_vencedor = q.wrapping_mul(3) / 4;
            ao_perdedor = q / 4;
        } else {
            ao_vencedor = q;
        }
    }
    let mut seguinte = 0u32;
    if r + 1 < cpb1_i16(copa + 0x3c) as i32 {
        let w = cpb1_u16(entrada + 0x6f);
        if w != 0x8c {
            seguinte = (entrada as u32 & 0xffff_0000) | w as u32;
        }
    }
    let financas = |clube: usize| {
        core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> usize>(0x5a0590)(cpb1_us(0xae2a38), cpb1_u32(clube))
    };
    let pagar = |clube: usize, valor: i32| {
        let fin = financas(clube);
        cpb1_chamar(0x5999a0, fin, &[valor as u32], &[!0], || {
            core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(0x5999a0)(fin, valor as u32)
        });
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, (cpb1_u32(copa + 0x28) as i32).wrapping_sub(valor)); // o fundo
    };
    if ao_vencedor != 0 {
        let a = [cpb1_u32(vencedor), ao_vencedor as u32, (ao_vencedor >> 31) as u32, u32::MAX, seguinte, partida as u32, desfecho as u32];
        cpb1_chamar(0x48e3e0, copa, &a, &[!0, !0, !0, !0, !0, !0, 0xff], || {
            core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(0x48e3e0)(
                copa, a[0], a[1], a[2], a[3], a[4], a[5], a[6])
        });
        pagar(vencedor, ao_vencedor);
    }
    if ao_perdedor != 0 {
        pagar(perdedor, ao_perdedor);
    }
}

unsafe extern "thiscall" fn comparar_copa_premio_da_rodada(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(%%I1%%);
    let sitios = [(0x52230b, cpb1_t7::<0x48e3e0> as *const () as usize), (0x522322, cpb1_t1::<0x5999a0> as *const () as usize), (0x522343, cpb1_t1::<0x5999a0> as *const () as usize)];
    let (_, ok, msg) = cpb1_comparar(&sitios, &[], &[(copa + 0x28, 4)], || o(copa, vencedor, perdedor, rodada, partida),
        || copa_premio_da_rodada(copa, vencedor, perdedor, rodada, partida));
    PLACAR.registrar(ok, || format!("copa_premio_da_rodada(rodada {}) {}", rodada as i16, msg));
}
