// ---------- 235a leva: acrescentar às preferências: a pessoa favorita e a rejeitada do clube e a pessoa favorita da pessoa (Database.cpp) ----------

unsafe fn dbq1_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A lista de três dwords em `lista` tem `quem`?
unsafe fn dbq1_tem(lista: usize, quem: usize) -> bool {
    (0..3).any(|k| dbq1_u32(lista + 4 * k) == quem)
}

/// O miolo das rotinas de acrescentar às preferências: já na lista (`ja`), nada; na lista `outra` (o oposto: favoritos
/// x rejeitados), tira de lá a primeira ocorrência e para; senão entra na primeira vaga de `lista`; sem vaga, só com
/// `forcar`: sorteia (0x90cfc0, 3) a vaga que troca (0, 1 ou 2).
unsafe fn dbq1_acrescentar(lista: usize, outra: usize, quem: usize, ja: bool, forcar: u32) {
    if ja {
        return;
    }
    if let Some(k) = (0..3).find(|&k| dbq1_u32(outra + 4 * k) == quem) {
        *((outra + 4 * k) as *mut usize) = 0;
        return;
    }
    if let Some(k) = (0..3).find(|&k| dbq1_u32(lista + 4 * k) == 0) {
        *((lista + 4 * k) as *mut usize) = quem;
        return;
    }
    if forcar == 0 {
        return;
    }
    let k = sortear(3);
    if (0..3).contains(&k) {
        *((lista + 4 * k as usize) as *mut usize) = quem;
    }
}

/// O estado do sorteio e as 6 pessoas (favoritas e rejeitadas) a partir de `base` (o clube +0x9b ou as preferências
/// +0x1c); `base` nulo: só o sorteio.
unsafe fn dbq1_regioes(base: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if base != 0 {
        reg.push((base, 24));
    }
    reg
}

/// { 0x547870 } (cdecl, Database.cpp): `club_add_favoured_staff(clube, pessoa, forcar)`: põe a pessoa entre as três
/// favoritas do clube (+0x9b/+0x9f/+0xa3): já favorita, nada; rejeitada (+0xa7..+0xaf), deixa de ser e para; senão a
/// primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Clube ou pessoa nulos: nada. Nos registros de clubes
/// (0x445f45) e de pessoas (0x86dc30).
unsafe extern "C" fn clube_acrescentar_pessoa_favorita(clube: usize, pessoa: usize, forcar: u32) {
    if clube == 0 || pessoa == 0 {
        return;
    }
    dbq1_acrescentar(clube + 0x9b, clube + 0xa7, pessoa, dbq1_tem(clube + 0x9b, pessoa), forcar) // as favoritas do clube
}

unsafe extern "C" fn comparar_clube_acrescentar_pessoa_favorita(clube: usize, pessoa: usize, forcar: u32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32)>(721);
    let reg = dbq1_regioes(if clube != 0 { clube + 0x9b } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, pessoa, forcar),
        || crate::aleatorio::sem_contar(|| clube_acrescentar_pessoa_favorita(clube, pessoa, forcar)));
    PLACAR.registrar(igual, || format!("clube_acrescentar_pessoa_favorita({:x}, {:x}, {}) {:x?} x {:x?}", clube, pessoa, forcar, mo, mr));
}

/// { 0x547940 } (cdecl, Database.cpp): `club_add_disliked_staff(clube, pessoa, forcar)`: põe a pessoa entre as três
/// rejeitadas do clube (+0xa7/+0xab/+0xaf): já rejeitada, nada; favorita (+0x9b..+0xa3), deixa de ser e para; senão a
/// primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Nas audiências das disputas (0x569ed0), na mídia
/// (0x755340) e em 0x7daf50 (do motor de partidas).
unsafe extern "C" fn clube_acrescentar_pessoa_rejeitada(clube: usize, pessoa: usize, forcar: u32) {
    if clube == 0 || pessoa == 0 {
        return;
    }
    dbq1_acrescentar(clube + 0xa7, clube + 0x9b, pessoa, dbq1_tem(clube + 0xa7, pessoa), forcar) // as rejeitadas do clube
}

unsafe extern "C" fn comparar_clube_acrescentar_pessoa_rejeitada(clube: usize, pessoa: usize, forcar: u32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32)>(722);
    let reg = dbq1_regioes(if clube != 0 { clube + 0x9b } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, pessoa, forcar),
        || crate::aleatorio::sem_contar(|| clube_acrescentar_pessoa_rejeitada(clube, pessoa, forcar)));
    PLACAR.registrar(igual, || format!("clube_acrescentar_pessoa_rejeitada({:x}, {:x}, {}) {:x?} x {:x?}", clube, pessoa, forcar, mo, mr));
}

/// { 0x547a10 } (cdecl, Database.cpp): `staff_add_favoured_staff(pessoa, outra, forcar)`: põe `outra` entre as três
/// pessoas favoritas da pessoa (preferências +0x65: +0x1c/+0x20/+0x24): já favorita, nada; rejeitada (+0x28..+0x30),
/// deixa de ser e para; senão a primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Pessoa, preferências ou
/// outra nulas: nada. Nas relações do elenco (0x5d6d10, diária), registros de clubes (0x445f45), volta da lesão
/// (0x630ee0), mídia e outras.
unsafe extern "C" fn acrescentar_pessoa_favorita(pessoa: usize, outra: usize, forcar: u32) {
    if pessoa == 0 {
        return;
    }
    let pref = dbq1_u32(pessoa + 0x65);
    if pref == 0 || outra == 0 {
        return;
    }
    dbq1_acrescentar(pref + 0x1c, pref + 0x28, outra, dbq1_tem(pref + 0x1c, outra), forcar) // as favoritas da pessoa
}

unsafe extern "C" fn comparar_acrescentar_pessoa_favorita(pessoa: usize, outra: usize, forcar: u32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32)>(723);
    let pref = if pessoa != 0 { dbq1_u32(pessoa + 0x65) } else { 0 };
    let reg = dbq1_regioes(if pref != 0 { pref + 0x1c } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, outra, forcar),
        || crate::aleatorio::sem_contar(|| acrescentar_pessoa_favorita(pessoa, outra, forcar)));
    PLACAR.registrar(igual, || format!("acrescentar_pessoa_favorita({:x}, {:x}, {}) {:x?} x {:x?}", pessoa, outra, forcar, mo, mr));
}

// ---------- 235a leva: o clube rejeitado da pessoa, o esfriar das relações pessoais e a lista das pessoas rejeitadas (Database.cpp) ----------

unsafe fn dbq2_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O estado do sorteio e 24 bytes a partir de `base` (nulo: só o sorteio).
unsafe fn dbq2_regioes(base: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if base != 0 {
        reg.push((base, 24));
    }
    reg
}

/// { 0x547e70 } (cdecl, Database.cpp): `staff_add_disliked_club(pessoa, clube, forcar)`: põe o clube entre os três
/// rejeitados da pessoa (preferências +0x65: +0x10/+0x14/+0x18). O clube já rejeita a pessoa (+0xa7..+0xaf): nada;
/// o clube é favorito (+4..+0xc): deixa de ser e para; senão a primeira vaga (sem conferir se já está entre os
/// rejeitados, como a original); sem vaga, com `forcar`, a vaga sorteada (0x90cfc0, 3). Pessoa, preferências ou clube
/// nulos: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0, duas chamadas), 0x5a5980, MANAGER_MANAGER
/// (0x69c030) e outras.
unsafe extern "C" fn acrescentar_clube_rejeitado(pessoa: usize, clube: usize, forcar: u32) {
    if pessoa == 0 {
        return;
    }
    let pref = dbq2_u32(pessoa + 0x65);
    if pref == 0 || clube == 0 {
        return;
    }
    if (0..3).any(|k| dbq2_u32(clube + 0xa7 + 4 * k) == pessoa) {
        return;
    }
    if let Some(k) = (0..3).find(|&k| dbq2_u32(pref + 4 + 4 * k) == clube) {
        *((pref + 4 + 4 * k) as *mut usize) = 0;
        return;
    }
    let lista = pref + 0x10; // os clubes rejeitados
    if let Some(k) = (0..3).find(|&k| dbq2_u32(lista + 4 * k) == 0) {
        *((lista + 4 * k) as *mut usize) = clube;
        return;
    }
    if forcar == 0 {
        return;
    }
    let k = sortear(3);
    if (0..3).contains(&k) {
        *((lista + 4 * k as usize) as *mut usize) = clube;
    }
}

unsafe extern "C" fn comparar_acrescentar_clube_rejeitado(pessoa: usize, clube: usize, forcar: u32) {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32)>(724);
    let pref = if pessoa != 0 { dbq2_u32(pessoa + 0x65) } else { 0 };
    let reg = dbq2_regioes(if pref != 0 { pref + 4 } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, clube, forcar),
        || crate::aleatorio::sem_contar(|| acrescentar_clube_rejeitado(pessoa, clube, forcar)));
    PLACAR.registrar(igual, || format!("acrescentar_clube_rejeitado({:x}, {:x}, {}) {:x?} x {:x?}", pessoa, clube, forcar, mo, mr));
}

/// { 0x547f90 } (cdecl, Database.cpp): `staff_fade_staff_relationships(pessoa)`: cada uma das três pessoas favoritas
/// (preferências +0x1c..+0x24) presente sai com o sorteio (0x90cfc0) de (+0x59 da pessoa, com sinal) + 15 dando 0;
/// cada uma das três rejeitadas (+0x28..+0x30), com o de 35 − (+0x5d). Um sorteio por vaga ocupada, na ordem.
/// Pessoa ou preferências nulas: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0).
unsafe extern "C" fn esmaecer_relacoes(pessoa: usize) {
    if pessoa == 0 || dbq2_u32(pessoa + 0x65) == 0 {
        return;
    }
    let pref = dbq2_u32(pessoa + 0x65);
    for k in 0..3 {
        let v = pref + 0x1c + 4 * k;
        if dbq2_u32(v) != 0 && sortear(*((pessoa + 0x59) as *const i8) as i32 + 15) == 0 { // as favoritas
            *(v as *mut usize) = 0;
        }
    }
    for k in 0..3 {
        let v = pref + 0x28 + 4 * k;
        if dbq2_u32(v) != 0 && sortear(35 - *((pessoa + 0x5d) as *const i8) as i32) == 0 { // as rejeitadas
            *(v as *mut usize) = 0;
        }
    }
}

unsafe extern "C" fn comparar_esmaecer_relacoes(pessoa: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(725);
    let pref = if pessoa != 0 { dbq2_u32(pessoa + 0x65) } else { 0 };
    let reg = dbq2_regioes(if pref != 0 { pref + 0x1c } else { 0 });
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa),
        || crate::aleatorio::sem_contar(|| esmaecer_relacoes(pessoa)));
    PLACAR.registrar(igual, || format!("esmaecer_relacoes({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// { 0x5480f0 } (cdecl, Database.cpp): `staff_get_disliked_staff(pessoa, saida, n)`: zera o byte `*n` (sempre, antes
/// de conferir a pessoa) e copia para `saida` as pessoas rejeitadas presentes (preferências +0x28/+0x2c/+0x30), na
/// ordem, contando em `*n`. Pessoa, preferências ou saída nulas: só o zero. Devolve `n` (o eax da original); os
/// chamadores leem só o byte. No diário das pessoas (0x5ce2d0, duas chamadas), MANAGER_MANAGER (0x693850), mídia e
/// telas de pessoas.
unsafe extern "C" fn pessoas_rejeitadas(pessoa: usize, saida: usize, n: usize) -> usize {
    let cont = n as *mut i8;
    *cont = 0;
    if pessoa == 0 || dbq2_u32(pessoa + 0x65) == 0 || saida == 0 {
        return n;
    }
    let pref = dbq2_u32(pessoa + 0x65);
    for k in 0..3 {
        let p = dbq2_u32(pref + 0x28 + 4 * k); // as rejeitadas
        if p != 0 {
            *((saida as isize + 4 * *cont as isize) as *mut usize) = p;
            *cont = (*cont).wrapping_add(1);
        }
    }
    n
}

unsafe extern "C" fn comparar_pessoas_rejeitadas(pessoa: usize, saida: usize, n: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> usize>(726);
    let mut reg = vec![(n, 1)];
    if saida != 0 {
        reg.push((saida, 12));
    }
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, saida, n), || pessoas_rejeitadas(pessoa, saida, n));
    PLACAR.registrar(igual && r == m, || format!("pessoas_rejeitadas({:x}, {:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, saida, n, r, m, mo, mr));
    r
}

// ---------- 235a leva: a máscara da posição natural, quantos do elenco a têm e o companheiro favorito escolhido (Database.cpp) ----------

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
unsafe extern "C" fn mascara_da_posicao_natural(pessoa: usize) -> u32 {
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
            m |= bit; // o lateral, o zagueiro...
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

unsafe extern "C" fn comparar_mascara_da_posicao_natural(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(727)(pessoa);
    let n = mascara_da_posicao_natural(pessoa);
    PLACAR.registrar(r == n, || format!("mascara_da_posicao_natural({:x}) original={:x} rust={:x}", pessoa, r, n));
    r
}

/// { 0x548150 } (cdecl, Database.cpp): `club_count_squad_with_position_mask(clube, mascara)`: quantos dos 50 do elenco
/// (+0xd7) com jogador (+0x61) têm a máscara da posição natural (0x5481a0, pelo endereço do jogo) igual a `mascara`.
/// Só `al` (os três chamadores, 0x758010 e 0x758200, leem o byte). Na mídia.
unsafe extern "C" fn contar_elenco_com_mascara(clube: usize, mascara: u32) -> u8 {
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

unsafe extern "C" fn comparar_contar_elenco_com_mascara(clube: usize, mascara: u32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> u8>(728)(clube, mascara);
    let n = contar_elenco_com_mascara(clube, mascara);
    PLACAR.registrar(r == n, || format!("contar_elenco_com_mascara({:x}, {:x}) original={} rust={}", clube, mascara, r, n));
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
unsafe extern "C" fn escolher_companheiro_favorito(pessoa: usize, clube: usize) -> usize {
    if pessoa == 0 || clube == 0 || dbq3_u32(0xb74350) == 0 {
        return 0;
    }
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let mascara_de: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5481a0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
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
        let contrato = contrato_de(dbq3_u32(0xae1050), o, clube);
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

unsafe extern "C" fn comparar_escolher_companheiro_favorito(pessoa: usize, clube: usize) -> usize {
    // as chamadas (contrato, média, 0x8cca10) vão pelo endereço do jogo: o estado do sorteio volta entre os lados e a
    // cópia não conta, se alguma delas sortear
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(729);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(pessoa, clube),
        || crate::aleatorio::sem_contar(|| escolher_companheiro_favorito(pessoa, clube)));
    PLACAR.registrar(igual && r == n, || format!("escolher_companheiro_favorito({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

// ---------- 235a leva: as regras por nação: o clube fora da Argentina, fora da Argentina, França e Croácia, e a nação da Melanésia (Database.cpp) ----------

/// O id (+0) da nação do `clube` (+0x53); clube ou nação nulos: None.
unsafe fn dbr_id_da_nacao_do_clube(clube: usize) -> Option<u32> {
    if clube == 0 {
        return None;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if nacao == 0 {
        return None;
    }
    Some(*(nacao as *const u32))
}

/// { 0x5487b0 } (cdecl, Database.cpp): `club_nation_is_not_argentina(clube)`: 1 se a nação do clube não tem o id
/// da chave Argentina 0x9cf218; clube ou nação nulos: 0. A resposta vai em `al` e o resto do eax é o do id (`setne
/// al` depois do `mov eax, [eax]`), como na original. Na carreira: a disciplina (0x55e300), o diário dos técnicos
/// (manager_manager 0x6ab800) e as notícias (0x782f70).
unsafe extern "C" fn clube_fora_da_argentina(clube: usize) -> u32 {
    match dbr_id_da_nacao_do_clube(clube) {
        None => 0,
        Some(id) => (id & 0xffff_ff00) | (id != *(0x9cf218 as *const u32)) as u32, // a Argentina
    }
}

unsafe extern "C" fn comparar_clube_fora_da_argentina(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(730)(clube);
    let n = clube_fora_da_argentina(clube);
    PLACAR.registrar(r == n, || format!("clube_fora_da_argentina({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// { 0x5487d0 } (cdecl, Database.cpp): `club_nation_not_argentina_france_croatia(clube)`: 1 se a nação do clube não
/// tem o id de Argentina 0x9cf218, França 0x9cf300 nem Croácia 0x9cf2ac; clube ou nação nulos: 0. Em `al`, o resto
/// do eax é o do id. Na carreira, pelo diário dos técnicos (manager_manager 0x6ab800).
unsafe extern "C" fn clube_fora_de_argentina_franca_croacia(clube: usize) -> u32 {
    match dbr_id_da_nacao_do_clube(clube) {
        None => 0,
        Some(id) => {
            let chaves = [0x9cf218usize, 0x9cf300, 0x9cf2ac]; // Argentina, França, Croácia
            (id & 0xffff_ff00) | (!chaves.iter().any(|&k| *(k as *const u32) == id)) as u32
        }
    }
}

unsafe extern "C" fn comparar_clube_fora_de_argentina_franca_croacia(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(731)(clube);
    let n = clube_fora_de_argentina_franca_croacia(clube);
    PLACAR.registrar(r == n, || format!("clube_fora_de_argentina_franca_croacia({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

/// 1 se o id (+0) da `nacao` (não nula; a original não confere) é o de uma das `chaves`, senão 0 (o eax inteiro).
unsafe fn dbr_nacao_entre(nacao: usize, chaves: &[usize]) -> u32 {
    let id = *(nacao as *const u32);
    chaves.iter().any(|&k| *(k as *const u32) == id) as u32
}

/// { 0x53d900 } (cdecl, Database.cpp): `database_is_melanesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (as melanésias), senão 0. Nas copas da Oceania
/// (oceania_club_champ 0x798220, oceania_nations 0x799e30).
unsafe extern "C" fn nacao_da_melanesia(nacao: usize) -> u32 {
    dbr_nacao_entre(nacao, &[0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500]) // melanésias
}

unsafe extern "C" fn comparar_nacao_da_melanesia(nacao: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(732)(nacao);
    let n = nacao_da_melanesia(nacao);
    PLACAR.registrar(r == n, || format!("nacao_da_melanesia({:x}) original={} rust={}", nacao, r, n));
    r
}

// ---------- 235a leva: a nação da Polinésia, o nome do cargo de treino e os anos entre duas datas em texto (Database.cpp) ----------

/// { 0x53d940 } (cdecl, Database.cpp): `database_is_polynesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (as polinésias), senão 0 (a nação não é conferida). Nas
/// copas da Oceania (oceania_club_champ 0x798220, oceania_nations 0x799e30).
unsafe extern "C" fn nacao_da_polinesia(nacao: usize) -> u32 {
    let id = *(nacao as *const u32);
    let chaves = [0x9cf4d0usize, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514]; // polinésias
    chaves.iter().any(|&k| *(k as *const u32) == id) as u32
}

unsafe extern "C" fn comparar_nacao_da_polinesia(nacao: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(733)(nacao);
    let n = nacao_da_polinesia(nacao);
    PLACAR.registrar(r == n, || format!("nacao_da_polinesia({:x}) original={} rust={}", nacao, r, n));
    r
}

/// { 0x53ac40 } (cdecl, Database.cpp): `database_get_coaching_job_name(cargo, genero)`: o nome do cargo (byte com
/// sinal) no texto fixo 0xae1980, pela tabela de saltos 0x53acc8: 5 e 12 "Manager", 6 e 13 "Assistant Manager",
/// 8 e 15 "Coach" (traduzidos pela 0x66f4e0); os demais, texto vazio. Com `genero` não nulo e o idioma (0x66f4d0)
/// 4, grava nele 4 (técnico, auxiliar) ou 1 (treinador). Devolve 0xae1980. Na regeneração (player_regen 0x7a9650).
unsafe extern "C" fn nome_do_cargo_de_treino(cargo: u32, genero: usize) -> usize {
    let (texto, marca) = match (cargo as i8 as i32).wrapping_sub(5) {
        0 | 7 => (0x9b6440usize, 4u8), // "Manager"
        1 | 8 => (0x9b642c, 4),        // "Assistant Manager"
        3 | 10 => (0x9b6448, 1),       // "Coach"
        _ => {
            *(0xae1980 as *mut u8) = 0;
            return 0xae1980;
        }
    };
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    formatar(0xae1980, texto);
    if genero != 0 && idioma() == 4 {
        *(genero as *mut u8) = marca;
    }
    0xae1980
}

unsafe extern "C" fn comparar_nome_do_cargo_de_treino(cargo: u32, genero: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(u32, usize) -> usize>(734);
    // o texto (os nomes traduzidos cabem em 64) e o byte do gênero
    let com = [(0xae1980usize, 64usize), (genero, 1)];
    let regioes = if genero != 0 { &com[..] } else { &com[..1] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || o(cargo, genero), || nome_do_cargo_de_treino(cargo, genero));
    PLACAR.registrar(igual && r == n, || format!("nome_do_cargo_de_treino({}) original={:x} rust={:x} {:x?} x {:x?}", cargo as i8, r, n, mo, mr));
    r
}

/// { 0x540340 } (cdecl, Database.cpp): `database_format_years_between_dates(fim, inicio, texto, tamanho)`: os anos
/// completos entre as datas (+0 dia do ano, +2 ano, palavras): ano(fim) − ano(início), menos 1 se o dia do início é
/// maior (com sinal); escrito em `texto` com "%d" (0x98806c) como palavra sem sinal (negativo vira 65535). Devolve 1.
/// Texto nulo, tamanho < 5 ou uma das datas nula: a original (caixa e 0). Nas telas de recordes do clube e das
/// competições (0x44aa20, 0x4a5690, 0x4a7f50).
unsafe extern "C" fn anos_entre_datas_em_texto(fim: usize, inicio: usize, texto: usize, tamanho: i32) -> u32 {
    if texto == 0 || tamanho < 5 || fim == 0 || inicio == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(735)(fim, inicio, texto, tamanho);
    }
    let w = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const i16);
    let mut anos = w(fim, 2).wrapping_sub(w(inicio, 2));
    if w(inicio, 0) > w(fim, 0) {
        anos = anos.wrapping_sub(1); // o aniversário ainda não chegou
    }
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    fmt(texto, 0x98806c, anos as u16 as u32);
    1
}

unsafe extern "C" fn comparar_anos_entre_datas_em_texto(fim: usize, inicio: usize, texto: usize, tamanho: i32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(735);
    if texto == 0 || tamanho < 5 || fim == 0 || inicio == 0 {
        return o(fim, inicio, texto, tamanho); // caixa: só a original
    }
    // "%d" de uma palavra: até 6 bytes com o zero
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 6)], || o(fim, inicio, texto, tamanho),
        || anos_entre_datas_em_texto(fim, inicio, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("anos_entre_datas_em_texto({:x}, {:x}) original={} rust={} {:x?} x {:x?}", fim, inicio, r, n, mo, mr));
    r
}

// ---------- 235a leva: as ordens dos clubes pelo nome curto e das pessoas pelo nome completo, e se a pessoa confere com os nomes (Database.cpp) ----------

/// A comparação de textos sem maiúsculas 0x90c1d0(a, b, 1, 0) que as ordens do Database.cpp usam.
unsafe fn dbr_comparar_textos(a: *const u8, b: *const u8) -> i32 {
    let textos: unsafe extern "C" fn(*const u8, *const u8, i32, u32) -> i32 = core::mem::transmute(0x90c1d0usize);
    textos(a, b, 1, 0)
}

/// { 0x540080 } (cdecl, Database.cpp): `database_compare_clubs_by_short_name(a, b)`: comparação do qsort de
/// ponteiros de clube pelo nome curto (+0x38) na comparação de textos 0x90c1d0 (sem maiúsculas); clube nulo vale o
/// texto vazio. A original copia os dois nomes para a pilha antes de comparar; a cópia compara no lugar (a 0x90c1d0
/// só lê). Nas telas de clubes e de busca, no Setup (0x822260) e na semeadura da UEFA (0x906410).
unsafe extern "C" fn ordem_dos_clubes_pelo_nome_curto(a: usize, b: usize) -> i32 {
    let nome = |p: usize| {
        let c = *(p as *const usize);
        if c == 0 { b"\0".as_ptr() } else { (c + 0x38) as *const u8 } // o nome curto
    };
    dbr_comparar_textos(nome(a), nome(b))
}

unsafe extern "C" fn comparar_ordem_dos_clubes_pelo_nome_curto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(736)(a, b);
    let n = ordem_dos_clubes_pelo_nome_curto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_clubes_pelo_nome_curto({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x540270 } (cdecl, Database.cpp): `database_compare_staff_by_full_name(a, b)`: comparação do qsort de
/// ponteiros de pessoa pelo nome no estilo 1 ("nome sobrenome", 0x53dba0 em 0x7d0 bytes) na comparação de textos
/// 0x90c1d0; pessoa nula vale o texto vazio. No menu (0x7612c0) e no gerente de transferências (0x8e55f0).
unsafe extern "C" fn ordem_das_pessoas_pelo_nome_completo(a: usize, b: usize) -> i32 {
    let formatar: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (*(a as *const usize), *(b as *const usize));
    if pa != 0 {
        formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 1); // "nome sobrenome"
    }
    if pb != 0 {
        formatar(pb, nb.as_mut_ptr() as usize, 0x7d0, 1);
    }
    dbr_comparar_textos(na.as_ptr(), nb.as_ptr())
}

unsafe extern "C" fn comparar_ordem_das_pessoas_pelo_nome_completo(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(737)(a, b);
    let n = ordem_das_pessoas_pelo_nome_completo(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_pessoas_pelo_nome_completo({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53e990 } (cdecl, Database.cpp): `database_staff_matches_names(pessoa, nome, sobrenome, comum, data)`: 1 se a
/// pessoa, que tem de ter registro de jogador (+0x61) ou de não jogador (+0x69), confere: cada nome não nulo (+4, +8,
/// +0xc) tem o id (+0x33) pedido e, com `data`, a data de nascimento (+0x10 dia, +0x12 ano) é a mesma (0x4a7f20);
/// senão 0. Pessoa nula: a original (caixa e 0). Telas, regeneração e transferências (~137 chamadores).
unsafe extern "C" fn pessoa_confere_com_os_nomes(pessoa: usize, nome: u32, sobrenome: u32, comum: u32, data: usize) -> u32 {
    if pessoa == 0 {
        return orig::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(738)(pessoa, nome, sobrenome, comum, data);
    }
    let ler = |d: usize| core::ptr::read_unaligned((pessoa + d) as *const usize);
    if ler(0x61) == 0 && ler(0x69) == 0 {
        return 0;
    }
    for (d, id) in [(4, nome), (8, sobrenome), (0xc, comum)] {
        let n = ler(d);
        if n != 0 && core::ptr::read_unaligned((n + 0x33) as *const u32) != id {
            return 0; // o nome não confere
        }
    }
    if data != 0 {
        let w = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const u16);
        if w(pessoa, 0x12) != w(data, 2) || w(pessoa, 0x10) != w(data, 0) {
            return 0; // outra data de nascimento
        }
    }
    1
}

unsafe extern "C" fn comparar_pessoa_confere_com_os_nomes(pessoa: usize, nome: u32, sobrenome: u32, comum: u32, data: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(738);
    if pessoa == 0 {
        return o(pessoa, nome, sobrenome, comum, data); // caixa: só a original
    }
    let r = o(pessoa, nome, sobrenome, comum, data);
    let n = pessoa_confere_com_os_nomes(pessoa, nome, sobrenome, comum, data);
    PLACAR.registrar(r == n, || format!("pessoa_confere_com_os_nomes({:x}, {:x}, {:x}, {:x}, {:x}) original={} rust={}", pessoa, nome, sobrenome, comum, data, r, n));
    r
}

// ---------- 235a leva: as ordens dos clubes pela latitude e pela longitude da cidade do estádio (Database.cpp) ----------

/// A ordem dos clubes `ca` e `cb` (não nulos) pela coordenada f64 em `d` da cidade (+0x38) do estádio (+0x69); sem
/// estádio ou sem cidade, a de `ca` vale [0x9689a0] e a de `cb` vale 0. −1 se a de `ca` é menor (ou NaN: o `fcom`
/// com C0), 1 se maior, 0 se igual.
unsafe fn dbr_ordem_pela_coordenada(ca: usize, cb: usize, d: usize) -> i32 {
    let cidade = |c: usize| {
        let e = core::ptr::read_unaligned((c + 0x69) as *const usize);
        if e == 0 { 0 } else { core::ptr::read_unaligned((e + 0x38) as *const usize) }
    };
    let (xa, xb) = (cidade(ca), cidade(cb));
    let x = if xa != 0 { core::ptr::read_unaligned((xa + d) as *const f64) } else { f64_em(0x9689a0) };
    let y = if xb != 0 { core::ptr::read_unaligned((xb + d) as *const f64) } else { 0.0 };
    if !(x >= y) {
        -1
    } else if x > y {
        1
    } else {
        0
    }
}

/// { 0x53fd40 } (cdecl, Database.cpp): `database_compare_clubs_by_latitude(a, b)`: comparação do qsort de ponteiros
/// de clube pela latitude (+0x23, f64) da cidade do estádio, a menor primeiro. Clube nulo: a original (caixa e 0).
/// Nas copas sorteadas por região (eng_auto_cup 0x56b730, cro_a1, fin_prm, ita_ser_a, nor_prm, por_prm, spa_first,
/// swe_cup).
unsafe extern "C" fn ordem_dos_clubes_pela_latitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (*(a as *const usize), *(b as *const usize));
    if ca == 0 || cb == 0 {
        return orig::<unsafe extern "C" fn(usize, usize) -> i32>(739)(a, b);
    }
    dbr_ordem_pela_coordenada(ca, cb, 0x23) // a latitude
}

unsafe extern "C" fn comparar_ordem_dos_clubes_pela_latitude(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(739);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // caixa: só a original
    }
    let r = o(a, b);
    let n = ordem_dos_clubes_pela_latitude(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_clubes_pela_latitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53fe50 } (cdecl, Database.cpp): `database_compare_clubs_by_longitude(a, b)`: a mesma comparação pela
/// longitude (+0x2b, f64). Clube nulo: a original (caixa e 0). Nas copas sorteadas por região (bel_first, hol_cup,
/// ita_c_cup, spa_first, tur_first).
unsafe extern "C" fn ordem_dos_clubes_pela_longitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (*(a as *const usize), *(b as *const usize));
    if ca == 0 || cb == 0 {
        return orig::<unsafe extern "C" fn(usize, usize) -> i32>(740)(a, b);
    }
    dbr_ordem_pela_coordenada(ca, cb, 0x2b) // a longitude
}

unsafe extern "C" fn comparar_ordem_dos_clubes_pela_longitude(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(740);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return o(a, b); // caixa: só a original
    }
    let r = o(a, b);
    let n = ordem_dos_clubes_pela_longitude(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_clubes_pela_longitude({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
