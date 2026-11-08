// ---------- %%LEVA%%a leva: o primeiro nome pelo peso, as cores da competição e se o clube tem vaga no cargo (Database.cpp) ----------

/// { 0x539b90 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) pelo peso.
/// Nação com mais de 15 primeiros nomes (`nation_name_index` +0x18): 3 sorteios na faixa dela,
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum pass_ball). Sem nação ou com
/// até 15: sorteio na tabela inteira ([0xae2398], no máximo 0x7fff). Só o rand() da CRT.
unsafe extern "C" fn database_pick_first_name_by_weight(nacao: usize) -> usize {
    let linha = |i: i32| (*(0xae23f0 as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao != 0 {
        let e = nation_name_index(nacao);
        let quantos = *((e + 0x18) as *const i32);
        if quantos > 0xf {
            let inicio = *(e as *const i32);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 sorteios pelo peso
                let i = scaled_crt_rand(quantos).wrapping_add(inicio);
                let peso = *((linha(i) + 0x3b) as *const i8);
                if peso > melhor {
                    melhor = peso;
                    escolhido = i;
                }
            }
            return linha(escolhido);
        }
    }
    linha(scaled_crt_rand(name_count_capped(0xae2398)))
}

/// { 0x53b7c0 } (cdecl, Database.cpp): as cores da `competicao` em pixels (0x5e4800 com os bytes +0x37,
/// +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que começam com [0xae3184] e [0xaebe04]. As
/// cores vêm do registro +0x5d (+0x90 e +0x94 dele) ou, sem ele, da própria competição (+0x61 e +0x65);
/// cor nula deixa o padrão. Devolve 1. Competição nula: a original (padrões, caixa "Error" e 0).
unsafe extern "C" fn database_get_comp_colours(competicao: usize, frente: *mut u16, fundo: *mut u16) -> u32 {
    if competicao == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(%%I1%%)(competicao, frente, fundo);
    }
    // ponytail: a original empurra dwords com só o byte de baixo escrito (o resto é lixo de pilha);
    // a 0x5e4800 faz & 0xff nos três, então os bytes estendidos dão o mesmo pixel (como em 0x53b640)
    let cor: unsafe extern "C" fn(u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5e4800usize);
    core::ptr::write_unaligned(frente, *(0xae3184 as *const u16));
    core::ptr::write_unaligned(fundo, *(0xaebe04 as *const u16));
    let r = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let dono = r(competicao, 0x5d);
    let (a, b) = if dono != 0 { (r(dono, 0x90), r(dono, 0x94)) } else { (r(competicao, 0x61), r(competicao, 0x65)) };
    let pixel = |c: usize| {
        let byte = |d: usize| *((c + d) as *const u8) as u32;
        cor(byte(0x37), byte(0x38), byte(0x39), 0) as u16
    };
    if a != 0 {
        core::ptr::write_unaligned(frente, pixel(a));
    }
    if b != 0 {
        core::ptr::write_unaligned(fundo, pixel(b));
    }
    1
}

/// Há vaga para a `pessoa` nas `n` casas a partir de `clube + d`: alguma vazia ou já com ela.
unsafe fn has_slot_for(clube: usize, d: usize, n: usize, pessoa: usize) -> bool {
    (0..n).any(|i| {
        let v = core::ptr::read_unaligned((clube + d + 4 * i) as *const usize);
        v == 0 || v == pessoa
    })
}

/// { 0x53b9a0 } (cdecl, Database.cpp): 1 se o `clube` tem vaga para a `pessoa` no `cargo` (byte com
/// sinal, tabela de salto 0x53bcc8): 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3
/// (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 e 13: +0xcf / +0xd3 livre ou dela e vaga no elenco; 15:
/// vaga em +0x19f e no elenco. Clube ou pessoa nulos, cargo 14 ou fora de 1..15: a original (caixa e 0).
unsafe extern "C" fn database_club_has_room_for_job(clube: usize, cargo: u32, pessoa: usize) -> u32 {
    let c = cargo as u8 as i8;
    if clube == 0 || pessoa == 0 || !(1..=15).contains(&c) || c == 14 {
        return original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I2%%)(clube, cargo, pessoa);
    }
    let vaga = |d: usize, n: usize| has_slot_for(clube, d, n, pessoa);
    let tem = match c {
        1 => vaga(0xbf, 1),
        2..=4 => vaga(0xc3, 3),
        5 => vaga(0xcf, 1),
        6 | 7 => vaga(0xd3, 1),
        8 => vaga(0x19f, 5),
        9 => vaga(0x1b3, 7),
        10 => vaga(0x1cf, 3),
        11 => vaga(0xd7, 50),
        12 => vaga(0xcf, 1) && vaga(0xd7, 50),
        13 => vaga(0xd3, 1) && vaga(0xd7, 50),
        _ => vaga(0x19f, 5) && vaga(0xd7, 50), // 15
    };
    tem as u32
}

unsafe extern "C" fn comparar_database_pick_first_name_by_weight(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados e a cópia não conta
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_first_name_by_weight(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_first_name_by_weight({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_get_comp_colours(competicao: usize, frente: *mut u16, fundo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(%%I1%%);
    if competicao == 0 {
        return o(competicao, frente, fundo); // caixa: só a original
    }
    let regioes = [(frente as usize, 2), (fundo as usize, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(competicao, frente, fundo),
        || database_get_comp_colours(competicao, frente, fundo));
    PLACAR.registrar(igual && r == n, || format!("database_get_comp_colours({:x}) original={} rust={} {:x?} x {:x?}", competicao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_club_has_room_for_job(clube: usize, cargo: u32, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I2%%);
    let r = o(clube, cargo, pessoa);
    let c = cargo as u8 as i8;
    if clube == 0 || pessoa == 0 || !(1..=15).contains(&c) || c == 14 {
        return r; // caixa: só a original
    }
    let n = database_club_has_room_for_job(clube, cargo, pessoa);
    PLACAR.registrar(r == n, || format!("database_club_has_room_for_job({:x}, {}, {:x}) original={} rust={}", clube, c, pessoa, r, n));
    r
}
