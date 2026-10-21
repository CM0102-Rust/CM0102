// ---------- %%LEVA%%a leva: as regras da rodada no jogo da copa, a chave do jogo e o país da excursão (Cup.cpp, friendly.cpp) ----------

/// A rodada `r` da copa: registro de 104 bytes em [copa+0xa3].
unsafe fn rodada_da_copa(copa: usize, r: i32) -> usize {
    (*((copa + 0xa3) as *const usize) as isize + r as isize * 104) as usize
}

/// A linha da chave `k` da rodada `r` da copa: [[copa+0xad] + 4·r] + 53·k.
unsafe fn linha_da_chave(copa: usize, r: i32, k: i32) -> usize {
    (*((*((copa + 0xad) as *const usize) as isize + r as isize * 4) as *const usize) as isize + k as isize * 53) as usize
}

/// O registro de 6 bytes da vaga `v` da copa: [copa+0xa7] + 6·v.
unsafe fn vaga_da_copa(copa: usize, v: i16) -> usize {
    (*((copa + 0xa7) as *const usize) as isize + v as isize * 6) as usize
}

/// { 0x521d60 } (thiscall, ret 8, Cup.cpp): passa as regras da rodada do jogo (palavra +0xf da rodada
/// [jogo+0x34]) para as marcas do jogo (+0x38, OU). Com a marca 0x800 (decide quem passa no empate), o
/// nível (byte +4 da vaga, com sinal) dos dois lados da chave `chave` decide: mandante menor tira 0x800 e
/// põe 0x1000; maior mantém 0x800; igual tira 0x800 e põe 1.
unsafe extern "thiscall" fn regras_da_rodada_no_jogo(copa: usize, jogo: usize, chave: i32) {
    let r = *((jogo + 0x34) as *const i16) as i32;
    let marcas = (jogo + 0x38) as *mut u16;
    let m = core::ptr::read_unaligned(marcas) | core::ptr::read_unaligned((rodada_da_copa(copa, r) + 0xf) as *const u16);
    core::ptr::write_unaligned(marcas, m);
    if m & 0x800 == 0 {
        return;
    }
    let linha = linha_da_chave(copa, r, chave);
    let nivel = |o: usize| *((vaga_da_copa(copa, core::ptr::read_unaligned((linha + o) as *const i16)) + 4) as *const i8);
    let (a, b) = (nivel(0), nivel(2));
    let m = if a < b { (m ^ 0x800) | 0x1000 } else if a > b { m | 0x800 } else { (m ^ 0x800) | 1 };
    core::ptr::write_unaligned(marcas, m);
}

/// { 0x519c90 } (thiscall, ret 4, Cup.cpp): a chave da rodada do jogo (+0x34) em que está o clube
/// mandante do jogo (+0xc), olhando os dois lados de cada chave (53 bytes; vaga negativa é vazia); -1 se
/// nenhuma. O número de chaves é a palavra +0x1a da rodada. Jogo nulo: a original (caixa de erro).
unsafe extern "thiscall" fn chave_do_jogo_na_copa(copa: usize, jogo: usize) -> i32 {
    if jogo == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I1%%)(copa, jogo);
    }
    let r = *((jogo + 0x34) as *const i16) as i32;
    let n = core::ptr::read_unaligned((rodada_da_copa(copa, r) + 0x1a) as *const i16) as i32;
    let mandante = *((jogo + 0xc) as *const usize);
    for i in 0..n {
        let linha = linha_da_chave(copa, r, i);
        for o in [0, 2] {
            let v = core::ptr::read_unaligned((linha + o) as *const i16);
            if v >= 0 && *(core::ptr::read_unaligned(vaga_da_copa(copa, v) as *const usize) as *const usize) == mandante {
                return i;
            }
        }
    }
    -1
}

/// { 0x5c4680 } (thiscall, ret 4, friendly.cpp): sorteia o país da excursão do clube. Pela reputação
/// (+0x80) o alcance: ≥ 8000 qualquer país, ≥ 6000 e ≥ 4000 o mesmo continente (+0x71), ≥ 2000 o mesmo
/// continente e a mesma região (+0x75); abaixo, nenhum (0). Candidatos: os outros países (lista
/// [0xae23a8], 0x122 bytes, [0xae2350] países) com +0x86 ≥ 20, +0x7e = 1, +0x84 ≤ 2 e +0x8e ≥ 5; cada um
/// tira 0x90cfc0(100) e fica o maior (estrito, comparado como byte com sinal).
unsafe extern "thiscall" fn pais_da_excursao(_this: usize, clube: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let proprio = *((clube + 0x53) as *const usize);
    let alcance: i8 = match *((clube + 0x80) as *const i16) {
        8000.. => 5,
        6000.. => 4,
        4000.. => 3,
        2000.. => 2,
        _ => return 0,
    };
    let mut escolhido = 0usize;
    let mut melhor: i8 = -1;
    if *(0xae2350 as *const i32) <= 0 {
        return 0;
    }
    let mut p = *(0xae23a8 as *const usize);
    let mut i: i16 = 0;
    loop {
        let candidato = p != proprio
            && *((p + 0x86) as *const i16) >= 0x14
            && *((p + 0x7e) as *const u8) == 1
            && *((p + 0x84) as *const i8) <= 2
            && *((p + 0x8e) as *const i16) >= 5;
        if candidato {
            let continente = || *((p + 0x71) as *const u32) == *((proprio + 0x71) as *const u32);
            let perto = match alcance {
                2 => *((p + 0x75) as *const u8) == *((proprio + 0x75) as *const u8) && continente(),
                5.. => true,
                _ => continente(),
            };
            if perto {
                let r = sorteio(100) as i8;
                if r > melhor {
                    melhor = r;
                    escolhido = p;
                }
            }
        }
        i = i.wrapping_add(1);
        p += 0x122;
        if i as i32 >= *(0xae2350 as *const i32) {
            return escolhido;
        }
    }
}

unsafe extern "thiscall" fn comparar_regras_da_rodada_no_jogo(copa: usize, jogo: usize, chave: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(jogo + 0x38, 2)], || o(copa, jogo, chave),
        || regras_da_rodada_no_jogo(copa, jogo, chave));
    PLACAR.registrar(igual, || format!("regras_da_rodada_no_jogo(chave {}) {:x?} x {:x?}", chave, mo, mr));
}

unsafe extern "thiscall" fn comparar_chave_do_jogo_na_copa(copa: usize, jogo: usize) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I1%%)(copa, jogo);
    if jogo == 0 {
        return r;
    }
    let n = chave_do_jogo_na_copa(copa, jogo);
    PLACAR.registrar(r == n, || format!("chave_do_jogo_na_copa original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_pais_da_excursao(this: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pais_da_excursao(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pais_da_excursao({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
