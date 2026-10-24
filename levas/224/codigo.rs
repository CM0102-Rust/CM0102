// ---------- 224a leva: o sorteio do primeiro e do segundo nome por nação e o nome da pessoa formatado (Database.cpp) ----------

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
unsafe extern "C" fn primeiro_nome_da_nacao(nacao: usize) -> usize {
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
/// Como `primeiro_nome_da_nacao` com os segundos nomes (+4, +0x1c, [0xae239c]), sem filtrar os nomes
/// no sorteio de 20 (só o maior peso).
unsafe extern "C" fn segundo_nome_da_nacao(nacao: usize) -> usize {
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
unsafe extern "C" fn formatar_nome_da_pessoa(pessoa: usize, destino: usize, tamanho: u32, estilo: u32) -> u32 {
    let e = estilo as u8;
    if pessoa == 0 || destino == 0 || (tamanho as i16) < 0x69 || !(1..=7).contains(&e) {
        return orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(597)(pessoa, destino, tamanho, estilo);
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

unsafe extern "C" fn comparar_primeiro_nome_da_nacao(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(595);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| primeiro_nome_da_nacao(nacao)));
    PLACAR.registrar(igual && r == n, || format!("primeiro_nome_da_nacao({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_segundo_nome_da_nacao(nacao: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(596);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| segundo_nome_da_nacao(nacao)));
    PLACAR.registrar(igual && r == n, || format!("segundo_nome_da_nacao({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_formatar_nome_da_pessoa(pessoa: usize, destino: usize, tamanho: u32, estilo: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(597);
    if pessoa == 0 || destino == 0 || (tamanho as i16) < 0x69 || !(1..=7).contains(&(estilo as u8)) {
        return o(pessoa, destino, tamanho, estilo); // caixa: só a original
    }
    // o destino tem pelo menos 0x69 bytes (a rotina exige); 0x600740 e 0x6005e0 só leem
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(destino, 0x69)], || o(pessoa, destino, tamanho, estilo),
        || formatar_nome_da_pessoa(pessoa, destino, tamanho, estilo));
    PLACAR.registrar(igual && r == n, || format!("formatar_nome_da_pessoa({:x}, estilo {}) original={} rust={} {:x?} x {:x?}", pessoa, estilo as u8, r, n, mo, mr));
    r
}

// ---------- 224a leva: as cores do uniforme do clube, o técnico do clube e o nome do clube em inglês (Database.cpp) ----------

/// { 0x53b640 } (cdecl, Database.cpp): as cores do `uniforme` (byte: 1 casa +0x83/+0x87, 2 fora
/// +0x8b/+0x8f, 3 terceiro +0x93/+0x97; 2 e 3 sem uma das duas cores: as de casa trocadas) do `clube`
/// em pixels (0x5e4800 com os bytes +0x37, +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que
/// começam com [0xaebdf0] e [0xae3184]. Outro uniforme: só os padrões. Devolve 1. Clube nulo: a
/// original (padrões, caixa "Error" e 0).
unsafe extern "C" fn cores_do_uniforme_do_clube(clube: usize, uniforme: u32, frente: *mut u16, fundo: *mut u16) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(598)(clube, uniforme, frente, fundo);
    }
    // ponytail: a original empurra dwords com só o byte de baixo escrito (o resto é lixo de pilha);
    // a 0x5e4800 faz & 0xff nos três, então os bytes estendidos dão o mesmo pixel
    let cor: unsafe extern "C" fn(u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5e4800usize);
    core::ptr::write_unaligned(frente, *(0xaebdf0 as *const u16));
    core::ptr::write_unaligned(fundo, *(0xae3184 as *const u16));
    let r = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    let (a, b) = match uniforme as u8 {
        1 => (r(0x83), r(0x87)), // casa
        2 | 3 => {
            let d = if uniforme as u8 == 2 { 0x8b } else { 0x93 };
            let (a, b) = (r(d), r(d + 4));
            if a == 0 || b == 0 { (r(0x87), r(0x83)) } else { (a, b) }
        }
        _ => return 1,
    };
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

/// { 0x53ca90 } (cdecl, Database.cpp): o técnico (+0xcf) do `clube`; sem ele, o do parceiro A/B
/// (0x540a50 com 0 e 1); sem os dois, clube comum dá 0 e seleção (id ≥ [0xae235c] − [0xae2350]) repete
/// com a seleção da nação dela (0x53b3d0 com +0x53, a sub-21 chega na principal). Clube nulo (no começo
/// ou vindo da 0x53b3d0): a original com nulo (caixa "Error" e 0).
unsafe extern "C" fn tecnico_do_clube_ou_selecao(clube: usize) -> usize {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let selecao_da_nacao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let tecnico = |c: usize| core::ptr::read_unaligned((c + 0xcf) as *const usize);
    let mut c = clube;
    loop {
        if c == 0 {
            return orig::<unsafe extern "C" fn(usize) -> usize>(599)(0);
        }
        let t = tecnico(c);
        if t != 0 {
            return t; // o técnico do clube
        }
        let p = parceiro(c, 0, 1);
        if p != 0 && tecnico(p) != 0 {
            return tecnico(p);
        }
        if *(c as *const i32) < (*(0xae235c as *const i32)).wrapping_sub(*(0xae2350 as *const i32)) {
            return 0;
        }
        c = selecao_da_nacao(core::ptr::read_unaligned((c + 0x53) as *const usize));
    }
}

/// { 0x545880 } (cdecl, Database.cpp): o nome do `clube` em inglês. Idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20): pelo mapa
/// de ids [0xae23fc] se houver, senão pelo id se < quantos (+0), senão 0.
unsafe extern "C" fn nome_ingles_do_clube(clube: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return clube.wrapping_add(4); // idioma 0 ou 1
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(600)(clube);
    }
    let linhas = *((t + 0x20) as *const usize);
    let id = *(clube as *const i32);
    let mapa = *(0xae23fc as *const usize);
    if mapa != 0 {
        let i = *((mapa.wrapping_add((id as usize).wrapping_mul(4))) as *const i32);
        return linhas.wrapping_add((i as usize).wrapping_mul(79));
    }
    if id < *(t as *const i32) {
        linhas.wrapping_add((id as usize).wrapping_mul(79))
    } else {
        0
    }
}

unsafe extern "C" fn comparar_cores_do_uniforme_do_clube(clube: usize, uniforme: u32, frente: *mut u16, fundo: *mut u16) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(598);
    if clube == 0 {
        return o(clube, uniforme, frente, fundo); // caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(frente as usize, 2), (fundo as usize, 2)],
        || o(clube, uniforme, frente, fundo), || cores_do_uniforme_do_clube(clube, uniforme, frente, fundo));
    PLACAR.registrar(igual && r == n, || format!("cores_do_uniforme_do_clube({:x}, {}) original={} rust={} {:x?} x {:x?}", clube, uniforme as u8, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_tecnico_do_clube_ou_selecao(clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(599);
    if clube == 0 {
        return o(clube); // caixa: só a original
    }
    // 0x540a50 zera a marca +0x244 do clube quando recusa o parceiro (e a recusa se repete com ela zerada):
    // a região volta entre os lados
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x244, 1)], || o(clube),
        || tecnico_do_clube_ou_selecao(clube));
    PLACAR.registrar(igual && r == n, || format!("tecnico_do_clube_ou_selecao({:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_nome_ingles_do_clube(clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(600);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(clube); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(clube);
    let n = nome_ingles_do_clube(clube);
    PLACAR.registrar(r == n, || format!("nome_ingles_do_clube({:x}) idioma {} original={:x} rust={:x}", clube, idioma, r, n));
    r
}

// ---------- 224a leva: o ajuste do moral, o vetor de atributos do não jogador e as marcas de elenco da pessoa (Database.cpp) ----------

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
    let o = orig::<unsafe extern "C" fn(usize, u32, u32) -> u32>(601);
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
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32>(602);
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
    let o = orig::<unsafe extern "C" fn(usize, u32) -> usize>(603);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pessoa + 0x5e, 1)], || o(pessoa, elenco),
        || marcar_elenco_da_pessoa(pessoa, elenco));
    PLACAR.registrar(igual && r == n, || format!("marcar_elenco_da_pessoa({:x}, {:x}) {:x?} x {:x?}", pessoa, elenco as u8, mo, mr));
    r
}
