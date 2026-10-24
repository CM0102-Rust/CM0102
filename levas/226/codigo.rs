// ---------- 226a leva: a forma do adjetivo por idioma, o atributo de treinador da pessoa e a nota com a habilidade reduzida (Database.cpp) ----------

/// As formas do adjetivo de 0x545a60 (tabelas de salto 0x545b2c/0x545b34, 0x545b58/0x545b60, 0x545b84,
/// 0x545c10, 0x545c9c e 0x545d28), índice 1..35. Os idiomas 3, 6, 7, 8 e 10 dão a mesma linha.
const FORMAS_DO_ADJETIVO_COMUM: [u8; 35] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const FORMAS_DO_ADJETIVO_4: [u8; 35] = [
    2, 3, 0, 2, 2, 3, 0, 0, 2, 0, 0, 0, 1, 1, 0, 1, 2, 3, 0, 3, 0, 0, 0, 2, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0,
];
const FORMAS_DO_ADJETIVO_5: [u8; 35] = [
    2, 3, 2, 2, 1, 3, 2, 1, 2, 2, 3, 1, 3, 3, 1, 3, 2, 3, 2, 4, 1, 1, 1, 2, 2, 1, 3, 2, 1, 1, 1, 1, 3, 2, 2,
];
const FORMAS_DO_ADJETIVO_9: [u8; 35] = [
    1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2, 0, 2, 3, 1, 2, 2, 3, 1, 3, 0, 0, 0, 1, 1, 0, 2, 1, 0, 0, 0, 0, 1, 1, 0,
];

/// { 0x545a60 } (cdecl, Database.cpp): a forma (0..4) do adjetivo para o `indice` (byte com sinal,
/// 1..35) no `idioma` (0x66f4d0; os chamadores só chamam nos idiomas que flexionam). Idioma sem tabela
/// ou índice fora: 0xff. Sem chamadas e sem caixa.
unsafe extern "C" fn forma_do_adjetivo(indice: u32, idioma: u32) -> u8 {
    let linha = match idioma {
        3 | 6 | 7 | 8 | 10 => &FORMAS_DO_ADJETIVO_COMUM,
        4 => &FORMAS_DO_ADJETIVO_4,
        5 => &FORMAS_DO_ADJETIVO_5,
        9 => &FORMAS_DO_ADJETIVO_9,
        _ => return 0xff,
    };
    let i = (indice as u8 as i8 as i32).wrapping_sub(1) as u32; // o ja da original compara sem sinal
    if i > 0x22 {
        return 0xff;
    }
    linha[i as usize]
}

/// { 0x543f30 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`: 0x543fd0 com o
/// não jogador dela (+0x69) e o `atributo` inteiro como veio. Pessoa nula ou atributo (byte com sinal)
/// fora de 1..12: a original (caixa "Error" e −1).
unsafe extern "C" fn atributo_do_nao_jogador(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) {
        return orig::<unsafe extern "C" fn(usize, u32) -> u8>(614)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543fd0usize);
    let nao_jogador = core::ptr::read_unaligned((pessoa + 0x69) as *const usize);
    calcular(pessoa, nao_jogador, atributo)
}

/// { 0x5442f0 } (cdecl, Database.cpp): a nota de 1 a 20 (`nota_do_atributo`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador` × 0,1 ([0x967880]); a de 0x544380 com a habilidade
/// reduzida (os goleiros de 0x543540). Termina no `_ftol`.
unsafe extern "C" fn nota_relativa_com_habilidade_reduzida(jogador: usize, valor: u32) -> u32 {
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as f64;
    let v = (valor as u8 as i8) as f64;
    nota_do_atributo(habilidade * f64_em(0x967880) + (v + v)) as u32
}

unsafe extern "C" fn comparar_forma_do_adjetivo(indice: u32, idioma: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(u32, u32) -> u8>(613);
    let r = o(indice, idioma);
    let n = forma_do_adjetivo(indice, idioma);
    PLACAR.registrar(r == n, || format!("forma_do_adjetivo({}, idioma {}) original={} rust={}", indice as u8 as i8, idioma, r, n));
    r
}

unsafe extern "C" fn comparar_atributo_do_nao_jogador(pessoa: usize, atributo: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u8>(614);
    let r = o(pessoa, atributo);
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) {
        return r; // caixa: só a original
    }
    let n = atributo_do_nao_jogador(pessoa, atributo);
    PLACAR.registrar(r == n, || format!("atributo_do_nao_jogador({:x}, {}) original={} rust={}", pessoa, a, r, n));
    r
}

unsafe extern "C" fn comparar_nota_relativa_com_habilidade_reduzida(jogador: usize, valor: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(615);
    let r = o(jogador, valor);
    let n = nota_relativa_com_habilidade_reduzida(jogador, valor);
    PLACAR.registrar(r == n, || format!("nota_relativa_com_habilidade_reduzida({:x}, {}) original={} rust={}", jogador, valor as u8 as i8, r, n));
    r
}

// ---------- 226a leva: o primeiro nome pelo peso, as cores da competição e se o clube tem vaga no cargo (Database.cpp) ----------

/// { 0x539b90 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) pelo peso.
/// Nação com mais de 15 primeiros nomes (`indice_de_nomes_da_nacao` +0x18): 3 sorteios na faixa dela,
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com
/// até 15: sorteio na tabela inteira ([0xae2398], no máximo 0x7fff). Só o rand() da CRT.
unsafe extern "C" fn primeiro_nome_pelo_peso(nacao: usize) -> usize {
    let linha = |i: i32| (*(0xae23f0 as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao != 0 {
        let e = indice_de_nomes_da_nacao(nacao);
        let quantos = *((e + 0x18) as *const i32);
        if quantos > 0xf {
            let inicio = *(e as *const i32);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 sorteios pelo peso
                let i = sorteio_escalado(quantos).wrapping_add(inicio);
                let peso = *((linha(i) + 0x3b) as *const i8);
                if peso > melhor {
                    melhor = peso;
                    escolhido = i;
                }
            }
            return linha(escolhido);
        }
    }
    linha(sorteio_escalado(total_de_nomes_limitado(0xae2398)))
}

/// { 0x53b7c0 } (cdecl, Database.cpp): as cores da `competicao` em pixels (0x5e4800 com os bytes +0x37,
/// +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que começam com [0xae3184] e [0xaebe04]. As
/// cores vêm do registro +0x5d (+0x90 e +0x94 dele) ou, sem ele, da própria competição (+0x61 e +0x65);
/// cor nula deixa o padrão. Devolve 1. Competição nula: a original (padrões, caixa "Error" e 0).
unsafe extern "C" fn cores_da_competicao(competicao: usize, frente: *mut u16, fundo: *mut u16) -> u32 {
    if competicao == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(617)(competicao, frente, fundo);
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
unsafe fn vaga_no_cargo(clube: usize, d: usize, n: usize, pessoa: usize) -> bool {
    (0..n).any(|i| {
        let v = core::ptr::read_unaligned((clube + d + 4 * i) as *const usize);
        v == 0 || v == pessoa
    })
}

/// { 0x53b9a0 } (cdecl, Database.cpp): 1 se o `clube` tem vaga para a `pessoa` no `cargo` (byte com
/// sinal, tabela de salto 0x53bcc8): 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3
/// (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 e 13: +0xcf / +0xd3 livre ou dela e vaga no elenco; 15:
/// vaga em +0x19f e no elenco. Clube ou pessoa nulos, cargo 14 ou fora de 1..15: a original (caixa e 0).
unsafe extern "C" fn clube_tem_vaga_para_cargo(clube: usize, cargo: u32, pessoa: usize) -> u32 {
    let c = cargo as u8 as i8;
    if clube == 0 || pessoa == 0 || !(1..=15).contains(&c) || c == 14 {
        return orig::<unsafe extern "C" fn(usize, u32, usize) -> u32>(618)(clube, cargo, pessoa);
    }
    let vaga = |d: usize, n: usize| vaga_no_cargo(clube, d, n, pessoa);
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

unsafe extern "C" fn comparar_primeiro_nome_pelo_peso(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados e a cópia não conta
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(616);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| primeiro_nome_pelo_peso(nacao)));
    PLACAR.registrar(igual && r == n, || format!("primeiro_nome_pelo_peso({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_cores_da_competicao(competicao: usize, frente: *mut u16, fundo: *mut u16) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(617);
    if competicao == 0 {
        return o(competicao, frente, fundo); // caixa: só a original
    }
    let regioes = [(frente as usize, 2), (fundo as usize, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(competicao, frente, fundo),
        || cores_da_competicao(competicao, frente, fundo));
    PLACAR.registrar(igual && r == n, || format!("cores_da_competicao({:x}) original={} rust={} {:x?} x {:x?}", competicao, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_clube_tem_vaga_para_cargo(clube: usize, cargo: u32, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, usize) -> u32>(618);
    let r = o(clube, cargo, pessoa);
    let c = cargo as u8 as i8;
    if clube == 0 || pessoa == 0 || !(1..=15).contains(&c) || c == 14 {
        return r; // caixa: só a original
    }
    let n = clube_tem_vaga_para_cargo(clube, cargo, pessoa);
    PLACAR.registrar(r == n, || format!("clube_tem_vaga_para_cargo({:x}, {}, {:x}) original={} rust={}", clube, c, pessoa, r, n));
    r
}

// ---------- 226a leva: a entrada da pessoa no clube (Database.cpp) ----------

/// Põe a `pessoa` na primeira das `n` casas vazias a partir de `clube + d`; false se não há.
unsafe fn ocupar_casa_do_clube(clube: usize, d: usize, n: usize, pessoa: usize) -> bool {
    let casa = |i: usize| (clube + d + 4 * i) as *mut usize;
    match (0..n).find(|&i| core::ptr::read_unaligned(casa(i)) == 0) {
        Some(i) => {
            core::ptr::write_unaligned(casa(i), pessoa);
            true
        }
        None => false,
    }
}

/// { 0x53bd10 } (cdecl, Database.cpp): põe a `pessoa` no `clube` no `cargo` (byte, tabela de salto
/// 0x53c14c). Pessoa da faixa dos humanos (id ≥ [0xae2364] − 16) só no cargo 5. Com `data`: as 8 bytes
/// dela vão para a pessoa (+0x29 se o clube é seleção, id ≥ [0xae235c] − 2 × [0xae2350]; senão +0x3e;
/// 0x417c10 copia +2, +0 e +4 e repete numa local que ninguém lê). Com `reduzir`: cargos 11, 12, 13 e 15
/// viram 11, os outros 0. Jogador (+0x61) com habilidade atual (+5) > 85 num clube sem técnico humano
/// (0x600a50 com 0, 1, 0, 0): 6 vira 13 e 8 vira 15. Casas: 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3;
/// 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 +0xcf e o elenco (sem elenco: 0
/// com o +0xcf já gravado); 13 +0xd3 e o elenco (sem elenco fica 6); 15 +0x19f e o elenco (sem elenco
/// fica 8); 0 nenhuma; 14 e fora: 0. Com `gravar`: clube e cargo na pessoa (+0x24/+0x28 seleção,
/// +0x39/+0x3d clube). Devolve 1. Clube ou pessoa nulos: a original (caixa "Error" e 0).
unsafe extern "C" fn incluir_pessoa_no_clube(clube: usize, pessoa: usize, cargo: u32, data: usize, gravar: u32, reduzir: u32) -> u32 {
    if clube == 0 || pessoa == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(619)(clube, pessoa, cargo, data, gravar, reduzir);
    }
    let gi = |a: usize| *(a as *const i32);
    let selecao = || gi(clube) >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2));
    if gi(pessoa) >= gi(0xae2364).wrapping_sub(16) && cargo as u8 != 5 {
        return 0;
    }
    if data != 0 {
        let d = if selecao() { pessoa + 0x29 } else { pessoa + 0x3e };
        core::ptr::write_unaligned((d + 2) as *mut u16, core::ptr::read_unaligned((data + 2) as *const u16));
        core::ptr::write_unaligned(d as *mut u16, core::ptr::read_unaligned(data as *const u16));
        core::ptr::write_unaligned((d + 4) as *mut u32, core::ptr::read_unaligned((data + 4) as *const u32));
    }
    let mut c = cargo as u8;
    if reduzir != 0 {
        c = if matches!(c, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    if core::ptr::read_unaligned((pessoa + 0x61) as *const usize) != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) == 0 {
            let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
            if core::ptr::read_unaligned((jogador + 5) as *const i16) > 0x55 {
                if c == 6 {
                    c = 0xd;
                } else if c == 8 {
                    c = 0xf;
                }
            }
        }
    }
    let ocupar = |d: usize, n: usize| ocupar_casa_do_clube(clube, d, n, pessoa);
    let unica = |d: usize| {
        let p = (clube + d) as *mut usize;
        if core::ptr::read_unaligned(p) != 0 {
            return false;
        }
        core::ptr::write_unaligned(p, pessoa);
        true
    };
    let ok = match c as i8 {
        0 => true,
        1 => unica(0xbf),
        2..=4 => ocupar(0xc3, 3),
        5 => unica(0xcf),
        6 | 7 => unica(0xd3),
        8 => ocupar(0x19f, 5),
        9 => ocupar(0x1b3, 7),
        10 => ocupar(0x1cf, 3),
        11 => ocupar(0xd7, 50),
        12 => unica(0xcf) && ocupar(0xd7, 50),
        13 => {
            if !unica(0xd3) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 6; // sem vaga no elenco: fica só na casa +0xd3
            }
            true
        }
        15 => {
            if !ocupar(0x19f, 5) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 8; // sem vaga no elenco: fica só em +0x19f
            }
            true
        }
        _ => false, // 14 e fora de 0..15
    };
    if !ok {
        return 0;
    }
    if gravar != 0 {
        if selecao() {
            core::ptr::write_unaligned((pessoa + 0x24) as *mut usize, clube);
            *((pessoa + 0x28) as *mut u8) = c;
        } else {
            core::ptr::write_unaligned((pessoa + 0x39) as *mut usize, clube);
            *((pessoa + 0x3d) as *mut u8) = c; // clube e cargo do clube
        }
    }
    1
}

unsafe extern "C" fn comparar_incluir_pessoa_no_clube(clube: usize, pessoa: usize, cargo: u32, data: usize, gravar: u32, reduzir: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(619);
    if clube == 0 || pessoa == 0 {
        return o(clube, pessoa, cargo, data, gravar, reduzir); // caixa: só a original
    }
    // grava só no clube (581 bytes) e na pessoa (110); 0x600a50 só consulta
    let regioes = [(clube, 0x245), (pessoa, 0x6e)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(clube, pessoa, cargo, data, gravar, reduzir),
        || incluir_pessoa_no_clube(clube, pessoa, cargo, data, gravar, reduzir));
    PLACAR.registrar(igual && r == n, || format!("incluir_pessoa_no_clube({:x}, {:x}, {}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        clube, pessoa, cargo as u8, data, gravar, reduzir, r, n, mo, mr));
    r
}
