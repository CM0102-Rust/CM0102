// ---------- %%LEVA%%a leva: os regens: a nota do doador de dados de não jogador, a ordem das aposentadorias e a ordem da fila de regens (player_regen.cpp) ----------

unsafe fn cpq9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq9_b(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpq9_w(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

unsafe fn cpq9_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// { 0x7a7150 } (stdcall, ret 4, player_regen.cpp): `player_regen_score_non_player_candidate(pessoa)`: a nota da pessoa
/// como doadora dos dados de não jogador (pela 0x7a6e60, a cada pessoa não jogadora). −10000 com +0x69, com os bits
/// 0x30 do +0xb do registro de 79 bytes ([0xae2d0c]) ou abaixo de 31 anos (+0x18). Senão sorteia 150 × (idade − 31);
/// com clube (+0x39) de reputação (+0x80) e jogador (+0x61), + 250 × (habilidade atual +0xb ÷ reputação); com
/// jogador, + sorteio(⌊(+0x2f ÷ 2)³⌋) (pow 0x945ff0 e _ftol), +100 com +0xd > 7500 e +50 com +0xb > 7500; os
/// atributos + 5 × (3 × (+0x5d + +0x5a) + 2 × (+0x59 + +0x58) + +0x56) + ⌊+0x5b²⌋ + ⌊+0x57²⌋. Com clube: sorteio da
/// nota (+75 se o clube não tem +0xcf); sem clube: + (+0x22)², e a nota dobra se o +0xd do jogador passa de um
/// sorteio(10000) (lê o jogador sem conferir, como a original). Quente (todas as pessoas a cada chamada): sem contador.
unsafe extern "stdcall" fn player_regen_score_non_player_candidate(pessoa: usize) -> i32 {
    if cpq9_d(pessoa + 0x69) != 0 {
        return -10000;
    }
    let id = cpq9_d(pessoa);
    if *((cpq9_d(0xae2d0c).wrapping_add(id.wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
        return -10000;
    }
    let idade = cpq9_b(pessoa + 0x18);
    if idade < 31 {
        return -10000;
    }
    let mut nota = random((idade - 31) * 150); // a idade
    let clube = cpq9_d(pessoa + 0x39);
    if clube != 0 {
        let injury_word_0c = cpq9_w(clube + 0x80);
        let jogador = cpq9_d(pessoa + 0x61);
        if injury_word_0c != 0 && jogador != 0 {
            nota = nota.wrapping_add((cpq9_w(jogador + 0xb) / injury_word_0c).wrapping_mul(250));
        }
    }
    let jogador = cpq9_d(pessoa + 0x61);
    if jogador != 0 {
        let v = cpq9_b(jogador + 0x2f) / 2;
        nota = nota.wrapping_add(random(match_pl_ftol(game_pow(v as f64, 3.0))));
        if cpq9_w(jogador + 0xd) > 7500 {
            nota = nota.wrapping_add(100);
        }
        if cpq9_w(jogador + 0xb) > 7500 {
            nota = nota.wrapping_add(50);
        }
    }
    let quadrados = match_pl_ftol(game_pow(cpq9_b(pessoa + 0x5b) as f64, 2.0)).wrapping_add(match_pl_ftol(game_pow(cpq9_b(pessoa + 0x57) as f64, 2.0)));
    let soma = 3 * (cpq9_b(pessoa + 0x5d) + cpq9_b(pessoa + 0x5a)) + 2 * (cpq9_b(pessoa + 0x59) + cpq9_b(pessoa + 0x58)) + cpq9_b(pessoa + 0x56);
    nota = nota.wrapping_add(soma * 5).wrapping_add(quadrados);
    let clube = cpq9_d(pessoa + 0x39);
    if clube != 0 {
        if cpq9_d(clube + 0xcf) == 0 {
            nota = nota.wrapping_add(75);
        }
        return random(nota);
    }
    let b = *((pessoa + 0x22) as *const u8) as i32;
    nota = nota.wrapping_add(b * b);
    let r = random(10000); // sem clube
    if cpq9_w(cpq9_d(pessoa + 0x61) + 0xd) > r {
        nota = nota.wrapping_add(nota);
    }
    random(nota)
}

/// A data que vale numa entrada da lista de aposentadorias: a do anúncio (+8: dia +0, ano +2) se ainda não passou da
/// data do jogo ([0xae2c90] dia, [0xae2c92] ano: o ano sem sinal, o dia com sinal), senão a da aposentadoria (+0x10).
unsafe fn entry_effective_date(e: usize) -> usize {
    let (dia, ano) = (cpq9_w(0xae2c90), cpq9_u(0xae2c92));
    let a = cpq9_u(e + 0xa);
    if a < ano || (a == ano && cpq9_w(e + 8) < dia) {
        e + 0x10
    } else {
        e + 8
    }
}

/// { 0x7ae2a0 } (cdecl, player_regen.cpp): `player_regen_compare_retirements(a, b)`: o qsort da lista de aposentadorias
/// (registros de 0x1a bytes; pela 0x7a59e0 todo dia, que para na primeira entrada futura, e pelas listas iniciais
/// 0x7a5790/0x7a58f0 e pela 0x7a8690): pela data que vale (entry_effective_date), o ano sem sinal e o dia com sinal;
/// no empate pelo id (+0). Sem `a`, −1; sem `b`, 1. Quente (qsort): sem contador.
unsafe extern "C" fn player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (da, db) = (entry_effective_date(a), entry_effective_date(b));
    let (aa, ab) = (cpq9_u(da + 2), cpq9_u(db + 2));
    if aa != ab {
        return aa as i32 - ab as i32; // o ano mais cedo primeiro
    }
    let (ja, jb) = (cpq9_w(da), cpq9_w(db));
    if ja == jb {
        return (cpq9_d(a) as i32).wrapping_sub(cpq9_d(b) as i32);
    }
    ja - jb
}

/// { 0x7ae240 } (cdecl, player_regen.cpp): `player_regen_compare_queue_entries(a, b)`: o qsort da fila de regens
/// (registros de 0xc bytes, no fim da 0x7a59e0 todo dia; a 0x7a65d0 cria os regens das entradas vencidas): pelo ano
/// (+6, sem sinal), depois pelo dia (+4, com sinal), depois pelo id (+0). Sem `a`, −1; sem `b`, 1. Quente (qsort):
/// sem contador.
unsafe extern "C" fn player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (aa, ab) = (cpq9_u(a + 6), cpq9_u(b + 6));
    if aa > ab {
        return 1; // o ano mais tarde depois
    }
    if aa < ab {
        return -1;
    }
    let (ja, jb) = (cpq9_w(a + 4), cpq9_w(b + 4));
    if ja == jb {
        return (cpq9_d(a) as i32).wrapping_sub(cpq9_d(b) as i32);
    }
    ja - jb
}

/// Comparador: lado a lado pelo estado do sorteio (a cópia sem contar), e a nota.
unsafe extern "stdcall" fn compare_player_regen_score_non_player_candidate(pessoa: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> i32>(%%I0%%);
    let reg = crate::rng::STATE_REGIONS.to_vec();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(pessoa),
        || crate::rng::without_counting(|| player_regen_score_non_player_candidate(pessoa)));
    SCOREBOARD.record(igual && r == n, || format!("player_regen_score_non_player_candidate({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn compare_player_regen_compare_retirements(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = player_regen_compare_retirements(a, b);
    SCOREBOARD.record(r == n, || format!("player_regen_compare_retirements({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn compare_player_regen_compare_queue_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = player_regen_compare_queue_entries(a, b);
    SCOREBOARD.record(r == n, || format!("player_regen_compare_queue_entries({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
