// ---------- %%LEVA%%a leva: o nome da seleção, o nome do atributo do jogador e o texto das posições (Database.cpp) ----------

unsafe fn dbu2_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbu2_vazio(t: usize) -> bool {
    *(t as *const u8) == 0
}

/// strcat (os `repne scasb` e os `rep movsd`/`rep movsb` da original).
unsafe fn dbu2_juntar(destino: usize, origem: usize) {
    let mut d = destino;
    while *(d as *const u8) != 0 {
        d += 1;
    }
    let mut k = 0;
    loop {
        let c = *((origem + k) as *const u8);
        *((d + k) as *mut u8) = c;
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// { 0x542100 } (cdecl, Database.cpp): `club_get_national_team_name(clube, variante, texto, tamanho, forma, aspas)`:
/// o nome do clube de seleção em `texto` pela forma (byte, 1..4; fora disso não escreve). O nome é o de +0x38 com o
/// gênero +0x52 (byte com sinal), na forma 3 o de +4 com o gênero +0x37. Fora das seleções (id < [0xae235c] −
/// 2 × [0xae2350]): variante 0 copia o nome (forma 4: "Main Squad"); senão "Reserves" (formas 1 e 3), "Reserve" (2),
/// "Reserve Squad" (4). Seleção com variante: "B" ou "'B'" (com `aspas`), na forma 4 "B Squad"/"'B' Squad". Seleção
/// principal (id < clubes − nações): copia o nome (forma 4: "Squad"). As de base: sub-21 ou sub-23 pelo byte
/// [[0xb74318] + 0x36] + id da nação (+0x53), "U21"/"U23" (1), "Under 21"/"Under 23" (2), "Under 21's"/"Under 23's"
/// (3), "Under 21 Squad"/"Under 23 Squad" (4). Os formatos vão ao 0x66f4e0 com (gênero, gênero, nome). Devolve 1.
/// Tamanho < 0x47 ou clube nulo (com forma 1..4): a original (caixa). Nas telas do clube, da seleção, do técnico, na
/// escalação da partida (0x725f70, em Rust), no hall da fama, nas impressões e no Setup.
unsafe extern "C" fn nome_da_selecao(clube: usize, variante: u32, texto: usize, tamanho: i32, forma: u32, aspas: u32) -> u32 {
    let f = forma as u8 as i8;
    if tamanho < 0x47 || ((1..=4).contains(&f) && clube == 0) {
        return orig::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(%%I0%%)(clube, variante, texto, tamanho, forma, aspas);
    }
    if !(1..=4).contains(&f) {
        return 1;
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let id = dbu2_u32(clube) as i32;
    let nacoes = dbu2_u32(0xae2350) as i32;
    let clubes = dbu2_u32(0xae235c) as i32;
    let (nome, g) = if f == 3 { (clube + 4, *((clube + 0x37) as *const i8) as i32) } else { (clube + 0x38, *((clube + 0x52) as *const i8) as i32) };
    let formato = if id < clubes.wrapping_sub(nacoes.wrapping_mul(2)) {
        if variante == 0 {
            if f == 4 { Some(0x9b7634) } else { None } // Main Squad
        } else {
            Some([0x9b784c, 0x9b77ac, 0x9b784c, 0x9b770c][f as usize - 1]) // Reserves, Reserve, Reserves, Reserve Squad
        }
    } else if variante != 0 {
        Some(match (f == 4, aspas != 0) {
            (false, false) => 0x9b7814, // B
            (false, true) => 0x9b7830,  // 'B'
            (true, false) => 0x9b76c8,  // B Squad
            (true, true) => 0x9b76e8,   // 'B' Squad
        })
    } else if id < clubes.wrapping_sub(nacoes) {
        if f == 4 { Some(0x9b7658) } else { None } // Squad
    } else {
        let nacao = dbu2_u32(dbu2_u32(clube + 0x53));
        let sub23 = *(nacao.wrapping_add(dbu2_u32(dbu2_u32(0xb74318) + 0x36)) as *const u8) != 0;
        Some(match (f, sub23) {
            (1, false) => 0x9b7734, // U21
            (1, true) => 0x9b7750,  // U23
            (2, false) => 0x9b776c, // Under 21 (dbu)
            (2, true) => 0x9b778c,  // Under 23
            (3, false) => 0x9b77cc, // Under 21's
            (3, true) => 0x9b77f0,  // Under 23's
            (_, false) => 0x9b7678, // Under 21 Squad
            (_, true) => 0x9b76a0,  // Under 23 Squad
        })
    };
    match formato {
        Some(s) => {
            formatar(texto, s, g, g, nome);
        }
        None => {
            *(texto as *mut u8) = 0;
            dbu2_juntar(texto, nome);
        }
    }
    1
}

unsafe extern "C" fn comparar_nome_da_selecao(clube: usize, variante: u32, texto: usize, tamanho: i32, forma: u32, aspas: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(%%I0%%);
    let f = forma as u8 as i8;
    if tamanho < 0x47 || ((1..=4).contains(&f) && clube == 0) {
        return o(clube, variante, texto, tamanho, forma, aspas); // caixa: só a original
    }
    let n = (tamanho as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(clube, variante, texto, tamanho, forma, aspas),
        || nome_da_selecao(clube, variante, texto, tamanho, forma, aspas));
    PLACAR.registrar(igual && r == x, || format!("nome_da_selecao({:x}, {}, forma {}, {}) original={} rust={} {:x?} x {:x?}", clube, variante, f, aspas, r, x, mo, mr));
    r
}

/// Os nomes dos 31 atributos de jogador da 0x5428a0 (imediatos do código, pelos endereços no exe): (abreviado, leva o
/// vazio 0xad9c64, inteiro, leva o vazio).
const DBU2_ATRIBUTOS: [(usize, bool, usize, bool); 31] = [
    (0x9b7b60, false, 0x9b7b50, false), // Acc / Acceleration
    (0x9b7af4, false, 0x9b7ae8, false), // Agg / Aggression
    (0x9b7ae4, false, 0x9b7adc, false), // Agi / Agility
    (0x9b7ad8, false, 0x9b7ac8, false), // Ant / Anticipation
    (0x9b7ac4, false, 0x9b7a98, true),  // Bal / Balance
    (0x9b7a94, false, 0x9b7a8c, false), // Bra / Bravery
    (0x9b7a88, false, 0x9b7a7c, false), // Cre / Creativity
    (0x9b7a78, false, 0x9b7a6c, false), // Cro / Crossing
    (0x9b7b30, true, 0x9b7b24, false),  // Dec / Decisions
    (0x9b7a68, false, 0x9b7a58, false), // Det / Determination
    (0x9b7a54, false, 0x9b7a48, false), // Dri / Dribbling
    (0x9b78d4, true, 0x9b78c8, false),  // Fin / Finishing
    (0x9b7a44, false, 0x9b7a3c, false), // Fla / Flair
    (0x9b7a38, false, 0x9b7a2c, false), // Han / Handling
    (0x9b7a28, false, 0x9b7a20, false), // Hea / Heading
    (0x98ea64, false, 0x9b7a14, false), // Inf / Influence
    (0x9b7a10, false, 0x9b7a08, false), // Jum / Jumping
    (0x9b7b04, true, 0x9b7af8, false),  // Lon / Long Shots
    (0x9b79d8, true, 0x9b79b0, true),   // Mar / Marking
    (0x9b7984, true, 0x9b7974, false),  // Off / Off The Ball
    (0x9b7970, false, 0x9b7968, false), // Pac / Pace
    (0x9b7964, false, 0x9b795c, false), // Pas / Passing
    (0x9b7958, false, 0x9b794c, false), // Pos / Positioning
    (0x9b792c, true, 0x9b7920, false),  // Ref / Reflexes
    (0x9b7900, true, 0x9b78f4, false),  // Set / Set Pieces
    (0x9b78c4, false, 0x9b78bc, false), // Sta / Stamina
    (0x9b78b8, false, 0x9b78ac, false), // Str / Strength
    (0x9b78a8, false, 0x9b789c, false), // Tac / Tackling
    (0x9b7898, false, 0x9b788c, false), // Tea / Teamwork
    (0x9b7888, false, 0x9b787c, false), // Tec / Technique
    (0x9b7878, false, 0x9b786c, false), // Wor / Work Rate
];

/// { 0x5428a0 } (cdecl, Database.cpp; o mapa parte a rotina em 0x542f48, que é só o meio dela: a fronteira real vai
/// até 0x5431e0): `database_get_player_attribute_name(atributo, texto, tamanho, abreviado)`: zera o texto e escreve o
/// nome traduzido (0x66f4e0) do atributo de jogador (1..0x1f, byte com sinal), abreviado (`abreviado` não nulo) ou
/// inteiro; devolve 1. Atributo fora da faixa, texto nulo ou tamanho (curto com sinal) < 100: a original (caixa e 0).
/// Nas telas do clube, de busca, da pessoa e de treino.
unsafe extern "C" fn nome_do_atributo_do_jogador(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let x = atributo as u8 as i8;
    if !(1..=0x1f).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I1%%)(atributo, texto, tamanho, abreviado);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    *(texto as *mut u8) = 0;
    let (c, cv, l, lv) = DBU2_ATRIBUTOS[x as usize - 1];
    let (s, vazio) = if abreviado != 0 { (c, cv) } else { (l, lv) }; // (dbu)
    if vazio {
        formatar(texto, s, 0xad9c64usize);
    } else {
        formatar(texto, s);
    }
    1
}

unsafe extern "C" fn comparar_nome_do_atributo_do_jogador(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I1%%);
    let x = atributo as u8 as i8;
    if !(1..=0x1f).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return o(atributo, texto, tamanho, abreviado); // caixa: só a original
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito): os nomes cabem neles
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 100)], || o(atributo, texto, tamanho, abreviado),
        || nome_do_atributo_do_jogador(atributo, texto, tamanho, abreviado));
    PLACAR.registrar(igual && r == n, || format!("nome_do_atributo_do_jogador({}, {}) original={} rust={} {:x?} x {:x?}", x, abreviado, r, n, mo, mr));
    r
}

/// Uma posição no texto da 0x53ee50: forma 1 o abreviado (com o vazio 0xad9c64), 2 e 3 o inteiro, com a barra antes
/// quando o texto já tem algo; as outras formas, a letra só com o texto ainda vazio. O traduzido passa pelo texto
/// global [0xde1f64] (0x66f4e0), como na original.
unsafe fn dbu2_posicao(buf: usize, f: u8, curto: usize, longo: usize, longo_vazio: bool, letra: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    match f {
        1 => {
            if !dbu2_vazio(buf) {
                dbu2_juntar(buf, 0x9b7630); // "/"
            }
            formatar(0xde1f64, curto, 0xad9c64usize);
            dbu2_juntar(buf, 0xde1f64);
        }
        2 | 3 => {
            if !dbu2_vazio(buf) {
                dbu2_juntar(buf, 0x9b7630);
            }
            if longo_vazio {
                formatar(0xde1f64, longo, 0xad9c64usize);
            } else {
                formatar(0xde1f64, longo);
            }
            dbu2_juntar(buf, 0xde1f64);
        }
        _ => {
            if dbu2_vazio(buf) {
                dbu2_juntar(buf, letra);
            }
        }
    }
}

/// As posições da 0x53ee50 (abreviado, inteiro, o inteiro leva o vazio, letra), imediatos do código.
const DBU2_GOLEIRO: (usize, usize, bool, usize) = (0x9b7610, 0x9b7600, false, 0x9b760c); // GK, Goalkeeper, A
const DBU2_LIBERO: (usize, usize, bool, usize) = (0x9b75e4, 0x9b75dc, false, 0x98b3f0); // SW, Sweeper, B
const DBU2_DEFENSOR: (usize, usize, bool, usize) = (0x9b75c0, 0x9b75b0, false, 0x9b75bc); // D, Defender, C
const DBU2_VOLANTE: (usize, usize, bool, usize) = (0x9b7588, 0x9b756c, false, 0x9b7584); // DM, Defensive Midfielder, D
const DBU2_MEIA_ATACANTE: (usize, usize, bool, usize) = (0x9b7544, 0x9b7528, false, 0x9b7540); // AM, Attacking Midfielder, F
const DBU2_MEIA: (usize, usize, bool, usize) = (0x9b7508, 0x9b74f8, false, 0x9b7504); // M, Midfielder, E
const DBU2_ATACANTE: (usize, usize, bool, usize) = (0x9b74dc, 0x9b74d0, false, 0x9b74d8); // S, Striker, H
const DBU2_AVANTE: (usize, usize, bool, usize) = (0x9b74b4, 0x9b7484, true, 0x9b74b0); // F, Forward<%s>, G

/// { 0x53ee50 } (cdecl, Database.cpp): `database_format_player_positions(jogador, texto, tamanho, forma)`: o texto das
/// posições do jogador (as notas +0xf..+0x1a, bytes com sinal) pela forma (byte): 1 abreviado ("D/DM RC"), 2 inteiro
/// com os lados entre parênteses, 3 inteiro sem os lados, 4 e outras as letras. A linha: com a nota mínima de 15 a 10
/// (para quando achar algo): goleiro (+0xf), líbero (+0x10), defensor (+0x11), volante (+0x12) ou meia-atacante (+0x14
/// e +0x13) ou meia (+0x13), e atacante (+0x15; "avante" se também +0x14, +0x17, +0x18 ou +0x1a). Goleiro na nota:
/// só a linha. Nada até 10: meia-atacante, e com +0x14 ≥ 15 a original GRAVA 15 em +0x13 do jogador (a cópia também).
/// Os lados (nota ≥ 15): direito (+0x17), esquerdo (+0x18), centro (+0x19); forma 1 abreviados juntos, 2 inteiros com
/// barra, 4 as letras A, B, C só com os lados vazios. Fim: forma 2 "%s (%s)", as outras "%s %s" ou só a linha.
/// Devolve 1. Os textos traduzidos passam pelo global [0xde1f64] (o mesmo uso da original). Texto ou jogador nulos ou
/// tamanho (curto com sinal) < 30: a original (caixa). As locais (linha e lados, 2 KB cada) são gravadas antes de lidas.
/// Nas telas do clube, de busca, da pessoa, de tática e de treino e nas impressões.
unsafe extern "C" fn texto_das_posicoes(jogador: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    if texto == 0 || jogador == 0 || (tamanho as u16 as i16) < 0x1e {
        return orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I2%%)(jogador, texto, tamanho, forma);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    *(texto as *mut u8) = 0;
    let f = forma as u8;
    let nota = |o: usize| *((jogador + o) as *const i8) as i32;
    let mut linha = [0u8; 0x7d0];
    let mut lados = [0u8; 0x7c0];
    let buf = linha.as_mut_ptr() as usize;
    let pos = |p: (usize, usize, bool, usize)| dbu2_posicao(buf, f, p.0, p.1, p.2, p.3);
    let mut minimo: u8 = 0xf;
    loop {
        *(buf as *mut u8) = 0;
        let t = minimo as i32;
        if nota(0xf) >= t {
            pos(DBU2_GOLEIRO);
        }
        if nota(0x10) >= t {
            pos(DBU2_LIBERO);
        }
        if nota(0x11) >= t {
            pos(DBU2_DEFENSOR);
        }
        if nota(0x12) >= t {
            pos(DBU2_VOLANTE);
        } else if nota(0x14) >= t && nota(0x13) >= t {
            pos(DBU2_MEIA_ATACANTE);
        } else if nota(0x13) >= t {
            pos(DBU2_MEIA);
        }
        if nota(0x15) >= t {
            if nota(0x14) >= t || nota(0x17) >= t || nota(0x18) >= t || nota(0x1a) >= t {
                pos(DBU2_AVANTE);
            } else {
                pos(DBU2_ATACANTE);
            }
        }
        if nota(0xf) >= t {
            // goleiro: só a linha
            dbu2_juntar(texto, buf);
            return 1;
        }
        minimo = minimo.wrapping_sub(1);
        if !dbu2_vazio(buf) || minimo < 0xa {
            break;
        }
    }
    if dbu2_vazio(buf) {
        if nota(0x14) >= 0xf {
            *((jogador + 0x13) as *mut u8) = 0xf; // a original grava no jogador
        }
        if f == 2 || f == 3 {
            formatar(0xde1f64, DBU2_MEIA_ATACANTE.1);
        } else {
            formatar(0xde1f64, DBU2_MEIA_ATACANTE.0, 0xad9c64usize);
        }
        dbu2_juntar(buf, 0xde1f64);
    }
    let lado = lados.as_mut_ptr() as usize;
    let mut volta: u8 = 0xf;
    loop {
        *(lado as *mut u8) = 0;
        for (o, curto, longo, letra) in [(0x17, 0x9b746c, 0x9b7464, 0x9b760c), (0x18, 0x9b744c, 0x9b7444, 0x98b3f0), (0x19, 0x9b7428, 0x9b7420, 0x9b75bc)] {
            if nota(o) < 0xf {
                continue;
            }
            match f {
                1 => {
                    formatar(0xde1f64, curto, 0xad9c64usize); // R, L, C
                    dbu2_juntar(lado, 0xde1f64);
                }
                2 => {
                    if !dbu2_vazio(lado) {
                        dbu2_juntar(lado, 0x9b7630);
                    }
                    formatar(0xde1f64, longo); // Right, Left, Centre
                    dbu2_juntar(lado, 0xde1f64);
                }
                4 => {
                    if dbu2_vazio(lado) {
                        dbu2_juntar(lado, letra);
                    }
                }
                _ => {}
            }
        }
        // a original repete a volta (de 15 a 10) enquanto os lados ficam vazios; as notas são as mesmas
        volta = volta.wrapping_sub(1);
        if !dbu2_vazio(lado) || volta < 0xa {
            break;
        }
    }
    if f == 2 && !dbu2_vazio(lado) {
        sprintf(texto, 0x9a3d8c, buf, lado); // "%s (%s)" (dbu)
    } else if !dbu2_vazio(lado) {
        sprintf(texto, 0x98f0b8, buf, lado); // "%s %s"
    } else {
        dbu2_juntar(texto, buf);
    }
    1
}

unsafe extern "C" fn comparar_texto_das_posicoes(jogador: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I2%%);
    if texto == 0 || jogador == 0 || (tamanho as u16 as i16) < 0x1e {
        return o(jogador, texto, tamanho, forma); // caixa: só a original
    }
    // o texto, as notas do jogador (a original pode gravar +0x13) e o começo do texto global [0xde1f64]
    let n = (tamanho as u16 as usize).min(128);
    let reg = [(texto, n), (jogador + 0xf, 12), (0xde1f64, 64)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(jogador, texto, tamanho, forma),
        || texto_das_posicoes(jogador, texto, tamanho, forma));
    PLACAR.registrar(igual && r == x, || format!("texto_das_posicoes({:x}, forma {}) original={} rust={} {:x?} x {:x?}", jogador, forma as u8, r, x, mo, mr));
    r
}
