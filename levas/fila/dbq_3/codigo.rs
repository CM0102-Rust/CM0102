// ---------- %%LEVA%%a leva: a máscara da posição natural, quantos do elenco a têm e o companheiro favorito escolhido (Database.cpp) ----------

unsafe fn dbq3_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbq3_i8(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn dbq3_i16(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x5481a0 } (cdecl, Database.cpp): `staff_get_natural_position_mask(pessoa)`: a máscara das posições em que o
/// jogador (pessoa +0x61) é natural (20). Sem pessoa ou jogador: 0; goleiro (+0xf) 20: 1. Senão, pelos bytes com sinal
/// do jogador: +0x18 (0x80; 0x180 com +0x19 > 10), +0x17 (0x800; mais 0x400 com +0x19 > 10), +0x19 (0x700), +0x10 (2),
/// +0x11 (4), +0x12 (0x18), +0x13 (0x38), +0x14 (0x30), +0x15 (0x40; mais 0x700 se nenhum de +0x10..+0x14 é 20). Sem
/// nenhum: a primeira posição de +0x10 a +0x14 com 15 ou mais dá 2, 4, 0x18, 0x38 ou 0x30; +0x15 com 15 ou mais dá
/// 0x740 (0x40 se algum de +0x10..+0x14 é 20, o que ali não acontece); senão 0x10. A original guarda três bytes numa
/// local (gravados antes de lidos) e um no próprio argumento; a cópia não. Chamada em 37 pontos: escolha do companheiro
/// favorito (0x548da0, no diário das pessoas), motor de partidas (0x6b7f20), mídia (0x758010/0x758200, 0x72c1f0),
/// prêmios (0x768cf0) e telas de pessoas.
unsafe extern "C" fn staff_get_natural_position_mask(pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let j = dbq3_u32(pessoa + 0x61);
    if j == 0 {
        return 0;
    }
    let b = |k: usize| dbq3_i8(j + k);
    if b(0xf) == 20 {
        return 1;
    }
    let mut m = 0u32;
    if b(0x18) == 20 {
        m = if b(0x19) > 10 { 0x180 } else { 0x80 };
    }
    if b(0x17) == 20 {
        m |= if b(0x19) > 10 { 0xc00 } else { 0x800 };
    }
    if b(0x19) == 20 {
        m |= 0x700;
    }
    for (k, bit) in [(0x10, 2), (0x11, 4), (0x12, 0x18), (0x13, 0x38), (0x14, 0x30)] {
        if b(k) == 20 {
            m |= bit; // o throw_in, o zagueiro...
        }
    }
    let algum_20 = (0x10..=0x14).any(|k| b(k) == 20);
    if b(0x15) == 20 {
        m |= 0x40;
        if !algum_20 {
            m |= 0x700;
        }
    }
    if m != 0 {
        return m;
    }
    for (k, r) in [(0x10, 2), (0x11, 4), (0x12, 0x18), (0x13, 0x38), (0x14, 0x30)] {
        if b(k) >= 15 {
            return r;
        }
    }
    if b(0x15) >= 15 {
        return if algum_20 { 0x40 } else { 0x740 };
    }
    0x10
}

unsafe extern "C" fn comparar_staff_get_natural_position_mask(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(pessoa);
    let n = staff_get_natural_position_mask(pessoa);
    PLACAR.registrar(r == n, || format!("staff_get_natural_position_mask({:x}) original={:x} rust={:x}", pessoa, r, n));
    r
}

/// { 0x548150 } (cdecl, Database.cpp): `club_count_squad_with_position_mask(clube, mascara)`: quantos dos 50 do elenco
/// (+0xd7) com jogador (+0x61) têm a máscara da posição natural (0x5481a0, pelo endereço do jogo) igual a `mascara`.
/// Só `al` (os três chamadores, 0x758010 e 0x758200, leem o byte). Na mídia.
unsafe extern "C" fn club_count_squad_with_position_mask(clube: usize, mascara: u32) -> u8 {
    let mascara_de: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    let mut n = 0u8;
    for k in 0..50 {
        let p = dbq3_u32(clube + 0xd7 + 4 * k);
        if p != 0 && dbq3_u32(p + 0x61) != 0 && mascara_de(p) == mascara { // os iguais
            n = n.wrapping_add(1);
        }
    }
    n
}

unsafe extern "C" fn comparar_club_count_squad_with_position_mask(clube: usize, mascara: u32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u8>(%%I1%%)(clube, mascara);
    let n = club_count_squad_with_position_mask(clube, mascara);
    PLACAR.registrar(r == n, || format!("club_count_squad_with_position_mask({:x}, {:x}) original={} rust={}", clube, mascara, r, n));
    r
}

/// { 0x548da0 } (cdecl, Database.cpp): `staff_pick_favoured_team_mate(pessoa, clube)`: o colega do elenco do clube
/// que a pessoa escolhe para favorito, ou 0. Precisa da pessoa, do clube e de [0xb74350]; id % 6 = r: +0x59 ≥ r + 6,
/// +0x57 ≤ 15 − r e a idade (0x54cdb0 em 0xae2c90 com +0x3e/+0x42) ≥ id % 28 + 14. Cada um dos 50 (+0xd7) com jogador,
/// id abaixo de [0xae2364], não a própria pessoa, (ids somados) % 3 ≠ 0, não rejeitado pela pessoa (+0x28..+0x30), com
/// +0x5a ≥ 11 ou máscara (0x5481a0) diferente da da pessoa, com contrato (0x4cf5f0 em [0xae1050]), reputação (+0xb do
/// jogador) acima de 4250 e de (+0x80 do clube) − 750, fora da 0x8cca10 ([0xde1f54], 0) e com +5 do jogador acima de
/// 100 leva nota: a reputação; +500/+250 se a média 0x7c1e30(6, 17, 6,5), arredondada a float, passa a razão de
/// [0xb74350] (+0x10/+0x18, ou 6,8) + 0,2/+0,1, quando a 0x7c1e30(6, 1, 6,5) passa de 5; +1250 se já favorito; +75/+50
/// por +0x21 do jogador (20 / acima de 15); +250/−100 por +0x5b (acima de 15 / abaixo de 6); + (ids somados) % 500;
/// +750/+300/+150 pelo nível do contrato (+0x4f >> 4 = 1/2/3); compatriota (+0x1a) de pessoa de fora do país do clube
/// (+0x53): +1750 com o contrato +0x45 & 0x20020, senão +250 (idade abaixo de 350 e +0x18 abaixo de 24) ou +125. Fica o
/// de nota mais alta (estritamente acima de −10000 e do anterior). As chamadas pelo endereço do jogo. Locais todas
/// gravadas antes de lidas (a original usa o próprio argumento como cursor). No diário das pessoas
/// (game_daily_staff_update 0x5ce2d0) e na tela da pessoa (0x87dd80).
unsafe extern "C" fn staff_pick_favoured_team_mate(pessoa: usize, clube: usize) -> usize {
    if pessoa == 0 || clube == 0 || dbq3_u32(0xb74350) == 0 {
        return 0;
    }
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let mascara_de: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let id = dbq3_u32(pessoa) as i32;
    let r6 = id % 6;
    if (dbq3_i8(pessoa + 0x59) as i32) < r6 + 6 || dbq3_i8(pessoa + 0x57) as i32 > 15 - r6 {
        return 0;
    }
    if idade(0xae2c90, dbq3_u32(pessoa + 0x3e), dbq3_u32(pessoa + 0x42)) < id % 28 + 14 {
        return 0;
    }
    let (mut melhor, mut nota_melhor) = (0usize, -10000i32);
    for k in 0..50 {
        let o = dbq3_u32(clube + 0xd7 + 4 * k);
        if o == 0 || dbq3_u32(o + 0x61) == 0 {
            continue;
        }
        let oid = dbq3_u32(o) as i32;
        if oid >= dbq3_u32(0xae2364) as i32 || o == pessoa || oid.wrapping_add(id) % 3 == 0 {
            continue;
        }
        let pref = dbq3_u32(pessoa + 0x65);
        if pref != 0 && (0..3).any(|j| dbq3_u32(pref + 0x28 + 4 * j) == o) {
            continue;
        }
        if dbq3_i8(o + 0x5a) < 11 && mascara_de(o) == mascara_de(pessoa) {
            continue;
        }
        let contrato = contract_of(dbq3_u32(0xae1050), o, clube);
        if contrato == 0 {
            continue;
        }
        let rep = dbq3_i16(dbq3_u32(o + 0x61) + 0xb) as i32;
        if rep <= 0x109a || rep <= dbq3_i16(clube + 0x80) as i32 - 0x2ee {
            continue;
        }
        if fora(dbq3_u32(0xde1f54), o, 0) != 0 || dbq3_i16(dbq3_u32(o + 0x61) + 5) <= 100 {
            continue;
        }
        let l0 = media(dbq3_u32(0xb74350), dbq3_u32(o + 0x61), 6, 0x11, 6.5) as f32 as f64; // fstp dword
        let m1 = media(dbq3_u32(0xb74350), dbq3_u32(o + 0x61), 6, 1, 6.5);
        let dados = dbq3_u32(o + 0x61);
        let mut nota = dbq3_i16(dados + 0xb) as i32;
        if m1 > f32_em(0x968990) as f64 {
            let g = dbq3_u32(0xb74350);
            let razao = || {
                let d = f64_em(g + 0x18);
                if d == 0.0 || d.is_nan() { f64_em(0x968998) } else { f64_em(g + 0x10) / d }
            };
            // fcompp: C0 (abaixo ou sem ordem) soma
            if !(razao() + f64_em(0x968918) >= l0) {
                nota += 500;
            } else if !(razao() + f64_em(0x967880) >= l0) {
                nota += 250;
            }
        }
        if pref != 0 && (0..3).any(|j| dbq3_u32(pref + 0x1c + 4 * j) == o) {
            nota += 1250; // já favorito
        }
        let c = dbq3_i8(dados + 0x21);
        if c == 20 {
            nota += 75;
        } else if c > 15 {
            nota += 50;
        }
        let a = dbq3_i8(o + 0x5b);
        if a > 15 {
            nota += 250;
        } else if a < 6 {
            nota -= 100;
        }
        nota += oid.wrapping_add(id) % 500; // o desempate pelos ids
        match *((contrato + 0x4f) as *const u8) >> 4 {
            1 => nota += 750,
            2 => nota += 300,
            3 => nota += 150,
            _ => {}
        }
        let nacao = dbq3_u32(pessoa + 0x1a);
        if nacao != dbq3_u32(clube + 0x53) && dbq3_u32(o + 0x1a) == nacao {
            if dbq3_u32(contrato + 0x45) & 0x20020 != 0 {
                nota += 1750;
            } else if idade(0xae2c90, dbq3_u32(pessoa + 0x3e), dbq3_u32(pessoa + 0x42)) < 350 && dbq3_i8(pessoa + 0x18) < 24 {
                nota += 250;
            } else {
                nota += 125;
            }
        }
        if nota > nota_melhor {
            melhor = o;
            nota_melhor = nota;
        }
    }
    melhor
}

unsafe extern "C" fn comparar_staff_pick_favoured_team_mate(pessoa: usize, clube: usize) -> usize {
    // as chamadas (contrato, média, 0x8cca10) vão pelo endereço do jogo: o estado do sorteio volta entre os lados e a
    // cópia não conta, se alguma delas sortear
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(pessoa, clube),
        || crate::aleatorio::sem_contar(|| staff_pick_favoured_team_mate(pessoa, clube)));
    PLACAR.registrar(igual && r == n, || format!("staff_pick_favoured_team_mate({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}
