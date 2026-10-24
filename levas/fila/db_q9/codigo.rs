// ---------- %%LEVA%%a leva: o ajuste do moral, o vetor de atributos do não jogador e as marcas de elenco da pessoa (Database.cpp) ----------

/// { 0x544710 } (cdecl, Database.cpp): soma `delta` (byte com sinal) ao moral (+0x45) do jogador
/// (pessoa +0x61), amortecido perto das pontas: subindo acima de 10, se aleatorio(21) < moral + delta
/// soma só aleatorio(delta + 1); descendo abaixo de 10, se aleatorio((jogador +0x2f + pessoa +0x5a +
/// pessoa +0x58) / 3) > moral + delta soma aleatorio(delta − 1). Prende em 1..20 (byte com sinal); com
/// `limitar` e delta > 0, o vínculo com o clube (0x4cf5f0, pessoa +0x39) com +0x45 não nulo impede
/// subir. Devolve 1 se o moral mudou. Delta 0: 0, sem ler nada.
unsafe extern "C" fn ajustar_moral_da_pessoa(pessoa: usize, delta: u32, limitar: u32) -> u32 {
    let aleatorio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let d = delta as u8 as i8;
    if d == 0 {
        return 0;
    }
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    let moral = *((jogador + 0x45) as *const i8);
    let soma = moral as i32 + d as i32;
    let simples = moral.wrapping_add(d);
    let mut novo = if d > 0 {
        if soma > 10 && aleatorio(0x15) < soma { moral.wrapping_add(aleatorio(d as i32 + 1) as i8) } else { simples }
    } else if soma < 10 {
        let b = |a: usize| *(a as *const i8) as i32;
        let teto = (b(jogador + 0x2f) + b(pessoa + 0x5a) + b(pessoa + 0x58)) / 3;
        if aleatorio(teto) > soma { moral.wrapping_add(aleatorio(d as i32 - 1) as i8) } else { simples }
    } else {
        simples
    };
    novo = if novo < 1 { 1 } else if novo > 20 { 20 } else { novo };
    if limitar != 0 && d > 0 {
        let v = vinculo(*(0xae1050 as *const usize), pessoa, core::ptr::read_unaligned((pessoa + 0x39) as *const usize));
        if v != 0 && core::ptr::read_unaligned((v + 0x45) as *const u32) != 0 && novo >= moral {
            novo = moral;
        }
    }
    *((jogador + 0x45) as *mut i8) = novo;
    (novo != moral) as u32
}

/// Os bytes do não jogador (+0x0e..+0x22) na ordem das 21 células do vetor de 0x53a8b0 (13 antes do
/// corte, 8 depois).
const ORDEM_DO_NAO_JOGADOR: [usize; 21] = [
    0x0e, 0x0f, 0x12, 0x13, 0x14, 0x15, 0x16, 0x1a, 0x1c, 0x1d, 0x1f, 0x20, 0x22, 0x11, 0x10, 0x17, 0x18, 0x19, 0x1b,
    0x1e, 0x21,
];

/// { 0x53a8b0 } (cdecl, Database.cpp): estende (movsx) os atributos do `nao_jogador` para os 16 bits de
/// baixo das 21 células de 3 bytes do `vetor` (o 3º byte de cada uma fica como está) e grava 13 (o
/// corte) em `fim` se não nulo. A pessoa não é usada. eax na saída: os 16 bits altos do ponteiro do não
/// jogador e o último atributo estendido (os chamadores não usam).
unsafe extern "C" fn vetor_de_atributos_do_nao_jogador(nao_jogador: usize, _pessoa: usize, vetor: usize, fim: *mut u8) -> u32 {
    let mut ultimo = 0u16;
    for (k, &d) in ORDEM_DO_NAO_JOGADOR.iter().enumerate() {
        if k == 13 && !fim.is_null() {
            *fim = 0xd;
        }
        ultimo = *((nao_jogador + d) as *const i8) as i16 as u16;
        core::ptr::write_unaligned((vetor + 3 * k) as *mut u16, ultimo);
    }
    (nao_jogador as u32 & 0xffff_0000) | ultimo as u32
}

/// { 0x540980 } (cdecl, Database.cpp): o `elenco` (byte) nas marcas de elenco da pessoa (+0x5e): com
/// bits de clube (& 3) mantém os de seleção (& 0xc), senão mantém os de clube; depois acrescenta. A
/// original grava duas vezes (a máscara e o resultado). Devolve a pessoa (eax).
unsafe extern "C" fn marcar_elenco_da_pessoa(pessoa: usize, elenco: u32) -> usize {
    let e = elenco as u8;
    let p = (pessoa + 0x5e) as *mut u8;
    let mantem = if e & 3 != 0 { *p & 0xc } else { *p & 3 };
    *p = mantem;
    *p = mantem | e;
    pessoa
}

unsafe extern "C" fn comparar_ajustar_moral_da_pessoa(pessoa: usize, delta: u32, limitar: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I0%%);
    if delta as u8 == 0 {
        let r = o(pessoa, delta, limitar);
        PLACAR.registrar(r == 0, || format!("ajustar_moral_da_pessoa({:x}, 0) original={}", pessoa, r));
        return r;
    }
    // sorteia (0x90cfc0): o estado do gerador volta entre os lados e a cópia não conta os sorteios
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((core::ptr::read_unaligned((pessoa + 0x61) as *const usize) + 0x45, 1));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, delta, limitar),
        || crate::aleatorio::sem_contar(|| ajustar_moral_da_pessoa(pessoa, delta, limitar)));
    PLACAR.registrar(igual && r == n, || format!("ajustar_moral_da_pessoa({:x}, {}, {}) original={} rust={} {:x?} x {:x?}", pessoa, delta as u8 as i8, limitar, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_vetor_de_atributos_do_nao_jogador(nao_jogador: usize, pessoa: usize, vetor: usize, fim: *mut u8) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32>(%%I1%%);
    let mut regioes = vec![(vetor, 21 * 3)];
    if !fim.is_null() {
        regioes.push((fim as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(nao_jogador, pessoa, vetor, fim),
        || vetor_de_atributos_do_nao_jogador(nao_jogador, pessoa, vetor, fim));
    PLACAR.registrar(igual && r == n, || format!("vetor_de_atributos_do_nao_jogador({:x}) original={:x} rust={:x} {:x?} x {:x?}", nao_jogador, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_marcar_elenco_da_pessoa(pessoa: usize, elenco: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pessoa + 0x5e, 1)], || o(pessoa, elenco),
        || marcar_elenco_da_pessoa(pessoa, elenco));
    PLACAR.registrar(igual && r == n, || format!("marcar_elenco_da_pessoa({:x}, {:x}) {:x?} x {:x?}", pessoa, elenco as u8, mo, mr));
    r
}
