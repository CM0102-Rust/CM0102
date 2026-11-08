// ---------- %%LEVA%%a leva: o sorteio do primeiro e do segundo nome por nação e o nome da pessoa formatado (Database.cpp) ----------

/// rand() da CRT (0x946a04) escalado a `n`: rand × n / 0x8000. As rotinas de nomes fazem a conta em
/// 64 bits (0x945e40 multiplica, 0x945e80 divide com sinal) e ficam com os 32 bits de baixo.
unsafe fn sorteio_escalado(n: i32) -> i32 {
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    ((rand() as i64 * n as i64) / 0x8000) as i32
}

/// O total de nomes da global `a` ([0xae2398] primeiros, [0xae239c] segundos), no máximo 0x7fff.
unsafe fn total_de_nomes_limitado(a: usize) -> i32 {
    let t = *(a as *const i32);
    if t > 0x7fff { 0x7fff } else { t }
}

/// O índice de nomes da nação `n` (id em +0): [0xae2338] + id × 36 (+0 primeiro nome de onde começam os
/// primeiros nomes, +4 o dos segundos, +0x18 quantos primeiros, +0x1c quantos segundos).
unsafe fn indice_de_nomes_da_nacao(n: usize) -> usize {
    (*(0xae2338 as *const usize)).wrapping_add((*(n as *const i32) as usize).wrapping_mul(36))
}

/// A nação que empresta nomes a cada grupo de língua (nação +0x77, 1..32): tabelas de salto 0x53a060 e
/// 0x53a528 (iguais). O grupo 17 (0) sorteia entre [0x9cf484] e [0x9cf404].
const NACOES_DO_GRUPO_DE_LINGUA: [usize; 32] = [
    0x9cf204, 0x9cf458, 0x9cf448, 0x9cf298, 0x9cf2ac, 0x9cf2b8, 0x9cf338, 0x9cf2e4, 0x9cf2fc, 0x9cf300, 0x9cf314,
    0x9cf31c, 0x9cf35c, 0x9cf344, 0x9cf360, 0x9cf36c, 0, 0x9cf430, 0x9cf434, 0x9cf444, 0x9cf448, 0x9cf408,
    0x9cf2ac, 0x9cf488, 0x9cf4dc, 0x9cf270, 0x9cf3b8, 0x9cf428, 0x9cf470, 0x9cf4a8, 0x9cf348, 0x9cf2c0,
];

/// O trecho comum de 0x539c60 (0x539df0..) e 0x53a1b0 (0x53a2be..) para nação com até 100 nomes e
/// grupo de língua: a nação do grupo (tabela) se ela tem ≥ sorteio(20) + 15 nomes em `conta`; senão até
/// 40 nações sorteadas ([0xae2350], 0x122 bytes cada em [0xae23a8]) até achar uma do mesmo grupo com
/// mais de 5 primeiros nomes (+0x18, nos dois casos). Devolve (quantos em `conta`, primeiro em `inicio`)
/// se a nação escolhida tem ≥ 5 nomes; senão `None` (sorteio na tabela inteira).
unsafe fn nacao_que_empresta_nomes(nacao: usize, conta: usize, inicio: usize) -> Option<(i32, i32)> {
    let g = |a: usize| *(a as *const i32);
    let nacoes = *(0xae23a8 as *const usize);
    let nacao_de = |id: i32| nacoes.wrapping_add((id as usize).wrapping_mul(0x122));
    let grupo = *((nacao + 0x77) as *const i8);
    let k = (grupo as i32).wrapping_sub(1) as u32;
    let mut esi = 0usize;
    let mut direto = false;
    if k <= 0x1f {
        let id = if k == 16 {
            if sorteio_escalado(2) != 0 { g(0x9cf484) } else { g(0x9cf404) }
        } else {
            g(NACOES_DO_GRUPO_DE_LINGUA[k as usize])
        };
        esi = nacao_de(id);
        if esi != 0 {
            let minimo = sorteio_escalado(0x14).wrapping_add(0xf);
            direto = g(indice_de_nomes_da_nacao(esi) + conta) >= minimo;
        }
    }
    if !direto {
        for _ in 0..40 {
            esi = nacao_de(sorteio_escalado(g(0xae2350)));
            if *((esi + 0x77) as *const i8) == grupo && g(indice_de_nomes_da_nacao(esi) + 0x18) > 5 {
                break;
            }
        }
        if esi == 0 {
            return None;
        }
    }
    let e = indice_de_nomes_da_nacao(esi);
    let quantos = g(e + conta);
    if quantos < 5 {
        return None;
    }
    Some((quantos, g(e + inicio)))
}

/// Nome aceito no sorteio de 0x539c60: nulo ou vazio sim; uma letra só ou com ponto não.
unsafe fn nome_aceito(nome: usize) -> bool {
    if nome == 0 || *(nome as *const u8) == 0 {
        return true;
    }
    let s = core::ffi::CStr::from_ptr(nome as *const core::ffi::c_char).to_bytes();
    s.len() != 1 && !s.contains(&b'.')
}

/// { 0x539c60 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) para a `nacao`.
/// Sem nação: sorteio na tabela inteira. Nação com mais de 100 primeiros nomes: 20 sorteios na faixa
/// dela, fica o de maior peso (+0x3b, com sinal) entre os aceitos (`nome_aceito`). Com até 100: grupo de
/// língua 0 sorteia na tabela inteira, mas devolve a linha da tabela dos SEGUNDOS nomes ([0xae23f4],
/// assim na original); senão `nacao_que_empresta_nomes`. Só o rand() da CRT (não o 0x90cfc0).
unsafe extern "C" fn database_pick_first_name_for_nation(nacao: usize) -> usize {
    let linha = |t: usize, i: i32| (*(t as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao == 0 {
        return linha(0xae23f0, sorteio_escalado(total_de_nomes_limitado(0xae2398)));
    }
    let e = indice_de_nomes_da_nacao(nacao);
    let quantos = *((e + 0x18) as *const i32);
    if quantos > 100 {
        let inicio = *(e as *const i32);
        let (mut melhor, mut escolhido) = (-1i8, 0i32);
        for _ in 0..20 { // 20 sorteios do primeiro nome
            let i = sorteio_escalado(quantos).wrapping_add(inicio);
            let nome = linha(0xae23f0, i);
            let peso = *((nome + 0x3b) as *const i8);
            if peso > melhor && nome_aceito(nome) {
                melhor = peso;
                escolhido = i;
            }
        }
        return linha(0xae23f0, escolhido);
    }
    if *((nacao + 0x77) as *const u8) == 0 {
        return linha(0xae23f4, sorteio_escalado(total_de_nomes_limitado(0xae2398)));
    }
    match nacao_que_empresta_nomes(nacao, 0x18, 0) {
        Some((n, inicio)) => linha(0xae23f0, sorteio_escalado(n).wrapping_add(inicio)),
        None => linha(0xae23f0, sorteio_escalado(total_de_nomes_limitado(0xae2398))),
    }
}

/// { 0x53a1b0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) para a `nacao`.
/// Como `database_pick_first_name_for_nation` com os segundos nomes (+4, +0x1c, [0xae239c]), sem filtrar os nomes
/// no sorteio de 20 (só o maior peso).
unsafe extern "C" fn database_pick_second_name_for_nation(nacao: usize) -> usize {
    let linha = |i: i32| (*(0xae23f4 as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao == 0 {
        return linha(sorteio_escalado(total_de_nomes_limitado(0xae239c)));
    }
    let e = indice_de_nomes_da_nacao(nacao);
    let quantos = *((e + 0x1c) as *const i32);
    if quantos > 100 {
        let inicio = *((e + 4) as *const i32);
        let (mut melhor, mut escolhido) = (-1i8, 0i32);
        for _ in 0..20 { // 20 sorteios do segundo nome
            let i = sorteio_escalado(quantos).wrapping_add(inicio);
            let peso = *((linha(i) + 0x3b) as *const i8);
            if peso > melhor {
                melhor = peso;
                escolhido = i;
            }
        }
        return linha(escolhido);
    }
    if *((nacao + 0x77) as *const u8) == 0 {
        return linha(sorteio_escalado(total_de_nomes_limitado(0xae239c)));
    }
    match nacao_que_empresta_nomes(nacao, 0x1c, 4) {
        Some((n, inicio)) => linha(sorteio_escalado(n).wrapping_add(inicio)),
        None => linha(sorteio_escalado(total_de_nomes_limitado(0xae239c))),
    }
}

/// Texto não nulo e não vazio.
unsafe fn tem_texto(p: usize) -> bool {
    p != 0 && *(p as *const u8) != 0
}

/// strcpy (a original copia com rep movsd/movsb o texto e o zero final).
unsafe fn copiar_nome_formatado(destino: usize, origem: usize) {
    let mut i = 0;
    loop {
        let b = *((origem + i) as *const u8);
        *((destino + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// { 0x53dba0 } (cdecl, Database.cpp): o nome da `pessoa` em `destino` no `estilo` (byte, 1..7), com
/// nome (+4), sobrenome (+8) e nome comum (+0xc, ou o apelido dado pelo humano: 0x600740/0x6005e0 em
/// [0xb63c98]). 1 "nome sobrenome", 2 "N.sobrenome", 3 "sobrenome, nome", 4 "sobrenome, N", 5 só um
/// nome, 6 iniciais, 7 como o 1 sem preferir o comum; sempre com recuos (o que houver). Devolve 1.
/// Pessoa ou destino nulos, tamanho (i16) < 0x69 ou estilo fora de 1..7: a original (caixa e 0).
unsafe extern "C" fn database_format_staff_name(pessoa: usize, destino: usize, tamanho: u32, estilo: u32) -> u32 {
    let e = estilo as u8;
    if pessoa == 0 || destino == 0 || (tamanho as i16) < 0x69 || !(1..=7).contains(&e) {
        return orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I2%%)(pessoa, destino, tamanho, estilo);
    }
    let deu_apelido: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x600740usize);
    let apelido: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x6005e0usize);
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let ler = |d: usize| core::ptr::read_unaligned((pessoa + d) as *const usize);
    let (nome, sobrenome) = (ler(4), ler(8));
    let apelidos = *(0xb63c98 as *const usize);
    let comum = if apelidos != 0 && deu_apelido(apelidos, pessoa, 0) != 0 { apelido(apelidos, pessoa, 0) } else { ler(0xc) };
    *(destino as *mut u8) = 0;
    let (c, n, s) = (tem_texto(comum), tem_texto(nome), tem_texto(sobrenome));
    let letra = |p: usize| *(p as *const i8) as i32;
    let copiar = |p: usize| copiar_nome_formatado(destino, p);
    // os recuos da original voltam a testar o comum depois do sobrenome: sem efeito (já era vazio)
    match e {
        1 | 7 => {
            if e == 1 && c {
                copiar(comum);
            } else if n && s {
                fmt(destino, 0x98f0b8, nome, sobrenome); // "%s %s"
            } else if s {
                copiar(sobrenome);
            } else if c {
                copiar(comum);
            } else if n {
                copiar(nome);
            }
        }
        2 | 3 | 4 => {
            if c {
                copiar(comum);
            } else if n && s {
                match e {
                    2 => fmt(destino, 0x9b7410, letra(nome), sobrenome), // "%c.%s"
                    3 => fmt(destino, 0x98f6ac, sobrenome, nome),        // "%s, %s"
                    _ => fmt(destino, 0x9b7408, sobrenome, letra(nome)), // "%s, %c"
                };
            } else if s {
                copiar(sobrenome);
            } else if n {
                copiar(nome);
            }
        }
        5 => {
            if c {
                copiar(comum);
            } else if s {
                copiar(sobrenome);
            } else if n {
                copiar(nome);
            }
        }
        _ => {
            // 6: iniciais
            if c {
                fmt(destino, 0x99ec1c, letra(comum)); // "%c"
            } else if n && s {
                fmt(destino, 0x9b7400, letra(nome), letra(sobrenome)); // "%c%c"
            } else if s {
                fmt(destino, 0x99ec1c, letra(sobrenome));
            } else if n {
                fmt(destino, 0x99ec1c, letra(nome));
            }
        }
    }
    1
}

unsafe extern "C" fn comparar_database_pick_first_name_for_nation(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_first_name_for_nation(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_first_name_for_nation({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_pick_second_name_for_nation(nacao: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| database_pick_second_name_for_nation(nacao)));
    PLACAR.registrar(igual && r == n, || format!("database_pick_second_name_for_nation({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_database_format_staff_name(pessoa: usize, destino: usize, tamanho: u32, estilo: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I2%%);
    if pessoa == 0 || destino == 0 || (tamanho as i16) < 0x69 || !(1..=7).contains(&(estilo as u8)) {
        return o(pessoa, destino, tamanho, estilo); // caixa: só a original
    }
    // o destino tem pelo menos 0x69 bytes (a rotina exige); 0x600740 e 0x6005e0 só leem
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(destino, 0x69)], || o(pessoa, destino, tamanho, estilo),
        || database_format_staff_name(pessoa, destino, tamanho, estilo));
    PLACAR.registrar(igual && r == n, || format!("database_format_staff_name({:x}, estilo {}) original={} rust={} {:x?} x {:x?}", pessoa, estilo as u8, r, n, mo, mr));
    r
}
