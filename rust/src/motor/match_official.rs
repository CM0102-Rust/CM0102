// ---------- vigesima quarta leva: lances de bola parada / arbitragem ----------

/// { 0x6e9470 }: inicia o objeto (motor, pessoa, valor): sem pessoa, 5000; senao o primeiro de
/// 7750, 7250, 6750, 6250, 5750 cuja palavra 0x1e da pessoa passa de sorteio(350, 300, ...), ou 5250.
unsafe extern "thiscall" fn iniciar_rigor(this: usize, motor: u32, pessoa: usize) {
    *(this as *mut u32) = motor;
    *((this + 4) as *mut u32) = pessoa as u32;
    let v = (this + 8) as *mut u16;
    if pessoa == 0 {
        *v = 0x1388;
        return;
    }
    for (n, valor) in [(0x15e, 0x1e46u16), (0x12c, 0x1c52), (0xfa, 0x1a5e), (0xc8, 0x186a), (0x96, 0x1676)] {
        let r = sortear(n);
        if *((pessoa + 0x1e) as *const i16) as i32 > r {
            *v = valor;
            return;
        }
    }
    *v = 0x1482;
}

/// { 0x6bab90 } (match_eng.cpp): o lance registrado continua valendo? 0 sem lance; com o dono da
/// bola (0xf582) do outro lado que o jogador do lance, a bola passando a linha do lance (0x475c)
/// encerra o lance (limpa e da 0); nos demais casos, 1.
unsafe extern "thiscall" fn lance_valendo(this: usize) -> u8 {
    let p = *((this + 0x4761) as *const usize);
    if byte(this, 0x475a) == 0 || p == 0 {
        return 0;
    }
    let l = byte(p, 0x27);
    let dono = *((this + 0xf582) as *const usize);
    if dono == 0 || byte(dono, 0x27) as i32 != (l == 0) as i32 {
        return 1;
    }
    let (y, linha) = (byte(this, 0x8ea8), byte(this, 0x475c));
    if (y > linha && l == 1) || (y < linha && l == 0) {
        limpar_lance(this);
        return 0;
    }
    1
}

/// { 0x6e9540 }: decide se o lance vira falta (ou similar) para o jogador: nao com lance valendo, nem
/// com papel 1 na area; a pessoa (0x4) pode anular (sorteio(0x28 + 15) == 0, -250 no valor 0x8);
/// na area, 3 em 4 anula; senao +175 e, pelo codigo (0x25: 1 em 20; outros: 1 em 155 com 0x8ea9 > 1),
/// grava 4 e sorteio(0x14 do jogador) + 1 nas saidas.
unsafe extern "thiscall" fn decidir_falta(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let motor = *(this as *const usize);
    let valendo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6bab90usize);
    if valendo(motor) != 0 {
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    let na_area: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
    if tabela(motor, j) & 1 != 0 && na_area(j, lado(j) as u8) as u8 != 0 {
        return 0;
    }
    let pessoa = *((this + 4) as *const usize);
    let v = (this + 8) as *mut u16;
    if pessoa != 0 && sortear(byte(pessoa, 0x28) as i32 + 0xf) == 0 {
        *v = (*v).wrapping_add(0xff06);
        return 0;
    }
    if na_area(j, lado(j) as u8) as u8 != 0 && sortear(4) != 0 {
        return 0;
    }
    *v = (*v).wrapping_add(0xaf);
    let marca = if codigo == 0x25 { sortear(0x14) != 0 } else { sortear(0x9b) == 0 && byte(motor, 0x8ea9) > 1 };
    if !marca {
        return 0;
    }
    let r = sortear(byte(j, 0x14) as u8 as i32);
    *valor = (r as u8).wrapping_add(1);
    *tipo = 4;
    1
}

// ---------- vigesima quinta leva: cartoes ----------

/// Cartao amarelo: o primeiro grava `x` em 0x25 (0x24 = 1, tipo 2, peso 2,0); o segundo, em 0x26
/// (0x24 + 1, tipo 3, peso 3,5).
unsafe fn amarelo(j: usize, tipo: *mut u8, x: i8, peso: *mut f32) {
    let n = (j + 0x24) as *mut u8;
    if *n != 0 {
        *n = (*n).wrapping_add(1);
        *tipo = 3;
        *peso = 3.5;
        *((j + 0x26) as *mut i8) = x;
    } else {
        *n = 1;
        *tipo = 2;
        *peso = 2.0;
        *((j + 0x25) as *mut i8) = x;
    }
}

/// Desconto na disposicao pelo cartao: ftol(c x c') ou, com 0x8ed0 / 11 acima de 45, ftol(c'' / (0x8ed0 / 11) x c').
unsafe fn desconto_do_cartao(motor: usize, j: usize) {
    let q = (*((motor + 0x8ed0) as *const i16) as i32 / 11) as i16;
    let base = if q > 0x2d { f32_em(0x96d64c) as f64 / q as f64 } else { f32_em(0x968e70) as f64 };
    let d = (j + 0x35) as *mut i16;
    *d = (*d).wrapping_add(ftol(base * f32_em(0x96d648) as f64) as i16);
}

/// Contador de faltas do lado no motor (0xf5c9 + 54 x lado).
unsafe fn contar_falta(motor: usize, j: usize) {
    let c = (motor as isize + 0xf5c9 + 54 * lado(j) as isize) as *mut u8;
    *c = (*c).wrapping_add(1);
}

unsafe fn na_area_do_jogador(j: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
    f(j, lado(j) as u8) as u8 != 0
}

/// { 0x6e9630 }: punicao da falta (tipo 1): vermelho direto (-1000, tipo 4), amarelo pela
/// severidade contra sorteios, ou so -25; com `flag` (ou papel 1), sorteio(3000): acima de 475
/// vermelho (-750, tipo 6, 9 no registro), acima de 50 (ou na area) amarelo com -375.
unsafe extern "thiscall" fn punir_falta(this: usize, j: usize, tipo: *mut u8, x: i8, severidade: i8, flag: u8, peso: *mut f32) {
    *peso = 1.5;
    let motor = *(this as *const usize);
    contar_falta(motor, j);
    *((j + 0x14) as *mut u8) = (*((j + 0x14) as *const u8)).wrapping_add(1);
    if *tipo != 1 || byte(motor, 0x8eb3) != 0 {
        return;
    }
    let d = (j + 0x35) as *mut i16;
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x6c38c0usize);
    if flag == 0 && tabela(motor, j) & 1 == 0 {
        if severidade as i32 > sortear(0xf) + 9 {
            *d = (*d).wrapping_add(-1000);
            *tipo = 4;
            *peso = 5.0;
        } else if severidade as i32 > sortear(5) + 3 {
            desconto_do_cartao(motor, j);
            amarelo(j, tipo, x, peso);
        } else {
            *d = (*d).wrapping_add(-25);
        }
        return;
    }
    let r = sortear(0xbb8) as i16;
    if r > 0x1db {
        *d = (*d).wrapping_add(-750);
        *tipo = 6;
        *peso = 5.0;
        *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
        return;
    }
    if r <= 0x32 && !na_area_do_jogador(j) {
        return;
    }
    *d = (*d).wrapping_add(-375);
    amarelo(j, tipo, x, peso);
}

/// { 0x6ea4e0 }: como `punir_falta`, para outro tipo de lance: inativo, peso 10; tipo 7, peso
/// 1,25; com `flag`, sorteio(3000) acima de 375 e vermelho (tipo 5), senao amarelo com desconto
/// (acima de 50, ou na area); sem `flag`, vermelho pela severidade (marca 0x16 com `expulsar`),
/// amarelo contra sorteio(6) ou -25. Conta a falta no jogador e 0x15 no outro.
unsafe extern "thiscall" fn punir_lance(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, severidade: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    *peso = 1.5;
    if byte(j, 0x19) < 0 {
        *peso = 10.0;
        return;
    }
    let motor = *(this as *const usize);
    let d = (j + 0x35) as *mut i16;
    match *tipo {
        7 => {
            *peso = 1.25;
            contar_falta(motor, j);
        }
        1 if byte(motor, 0x8eb3) == 0 => {
            contar_falta(motor, j);
            if flag != 0 {
                let r = sortear(0xbb8) as i16;
                if r > 0x177 {
                    *d = (*d).wrapping_add(-750);
                    *tipo = 5;
                    *peso = 5.0;
                    *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
                } else if r > 0x32 || na_area_do_jogador(j) {
                    desconto_do_cartao(motor, j);
                    amarelo(j, tipo, x, peso);
                }
            } else if severidade as i32 > sortear(0xf) + 7 {
                *d = (*d).wrapping_add(-1000);
                *tipo = 4;
                *peso = 5.0;
                *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut i8) = x;
                if expulsar != 0 {
                    *((j + 0x16) as *mut u8) = 1;
                }
            } else if severidade as i32 > sortear(6) {
                desconto_do_cartao(motor, j);
                amarelo(j, tipo, x, peso);
            } else {
                *d = (*d).wrapping_add(-25);
            }
        }
        _ => return,
    }
    *((j + 0x14) as *mut u8) = (*((j + 0x14) as *const u8)).wrapping_add(1);
    *((outro + 0x15) as *mut u8) = (*((outro + 0x15) as *const u8)).wrapping_add(1);
}

// ---------- vigesima setima leva: impedimento (match_official.cpp) ----------

/// { 0x6ea760 } (match_official.cpp): procura um atacante do `lado` em impedimento no lance para a
/// casa de `y` (linha do passe em `linha`). Devolve o jogador (ajustando o valor do arbitro em 0x8)
/// ou 0. Sem lance valendo; `y` alem da linha e nao atras da ultima linha defensiva (0x8eaf para o
/// lado 0, 0x8eb0 para o 1); com `passador`, ele na area do outro lado e jogadores alem da linha.
/// A pessoa do arbitro pode deixar passar (sorteio(0x29 + 25) == 0, -75). Por jogador, conforme a
/// distancia a linha (2, 1, 0 casas ou alem), sorteios contra o byte 0x107.
unsafe extern "thiscall" fn procurar_impedimento(this: usize, lado_b: i8, linha: i8, y: i8, passador: usize) -> usize {
    let motor = *(this as *const usize);
    let valendo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6bab90usize);
    if valendo(motor) != 0 {
        return 0;
    }
    let (l0, l1) = (byte(motor, 0x8eaf), byte(motor, 0x8eb0));
    match lado_b {
        0 if !(y > linha && y >= l0) => return 0,
        1 if !(y < linha && y <= l1) => return 0,
        _ => {}
    }
    if passador != 0 {
        let alem: unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8 = core::mem::transmute(0x6cb700usize);
        if alem(motor, linha, lado_b, 0) == 0 {
            return 0;
        }
    }
    let v = (this + 8) as *mut i16;
    let pessoa = *((this + 4) as *const usize);
    if pessoa != 0 && sortear(byte(pessoa, 0x29) as i32 + 0x19) == 0 {
        *v = (*v).wrapping_add(-75);
        return 0;
    }
    let na_area: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
    for k in 0..20 {
        let p = jogador_do_bloco(motor, lado_b, k);
        if byte(p, 0x19) < 0 || p == passador {
            continue;
        }
        if passador != 0 {
            if na_area(passador, (lado_b == 0) as u8) as u8 == 0 {
                continue;
            }
            let dist = f6f09b0(p, 4, y);
            if ((sortear(4) + 1) as f64) < dist as f64 || dist.is_nan() {
                continue;
            }
        }
        let (py, a) = (byte(p, 0x103) as i32, byte(p, 0x107) as i32);
        let (linha_d, sinal, perto) = match lado_b {
            0 => (l0 as i32, 1, -25i16),
            1 => (l1 as i32, -1, -15i16),
            _ => continue,
        };
        if py == linha_d - 2 * sinal && sortear(6 * a + 500) == 0 {
            *v = (*v).wrapping_add(perto);
            return p;
        }
        if py == linha_d - sinal && sortear(ftol((a as f64) * (a as f64) * (a as f64) + f64_em(0x968890))) == 0 {
            *v = (*v).wrapping_add(-15);
            return p;
        }
        if py == linha_d && sortear(3) != 0 && a < sortear(0x12) {
            *v = (*v).wrapping_add(15);
            return p;
        }
        if (py - linha_d) * sinal > 0 && a < sortear(0x64) {
            return p;
        }
    }
    0
}

/// { 0x6cfd70 } (match_eng.cpp): `procurar_impedimento` do arbitro do motor (this + 0xf5b2).
unsafe extern "thiscall" fn motor_procurar_impedimento(this: usize, lado_b: i8, linha: i8, y: i8, passador: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(0x6ea760usize);
    f(this + 0xf5b2, lado_b, linha, y, passador)
}

// ---------- vigesima oitava leva: cartao pela gravidade ----------

/// Grava o codigo do cartao no registro do jogador: 0x25 com (0x23 + 0x24) == 1 (ou <= 1, `ate_um`),
/// 0x26 com outro total diferente de zero.
unsafe fn registrar_cartao(j: usize, codigo: u8, ate_um: bool) {
    let t = byte(j, 0x23) as i32 + byte(j, 0x24) as i32;
    if t == 1 || (ate_um && t <= 1) {
        *((j + 0x25) as *mut u8) = codigo;
    } else if t != 0 {
        *((j + 0x26) as *mut u8) = codigo;
    }
}

/// { 0x6eab60 } (match_official.cpp): cartao pela gravidade do lance (1..6): sorteio(0x14 do
/// jogador) ajustado pela pessoa do arbitro (0x25) e pela gravidade; segundo amarelo possivel com
/// 0x24; vermelho (-1000, tipo 4) contra sorteio(15) + 7; senao amarelo (desconto, tipo 2/3) contra
/// sorteio(3 x 0x24 + 5) + 2, ou so tipo 9. O codigo do registro fica no lugar do argumento `j`
/// (sem gravidade conhecida, o byte baixo do ponteiro).
unsafe extern "thiscall" fn cartao_pela_gravidade(this: usize, j: usize, tipo: *mut u8, gravidade: u8, flag: u8) {
    let mut nota = sortear(byte(j, 0x14) as u8 as i32) as u8;
    let pessoa = *((this + 4) as *const usize);
    if pessoa != 0 {
        let a = byte(pessoa, 0x25);
        nota = nota.wrapping_add(if a > 18 { 2 } else if a > 12 { 1 } else if a > 6 { 0 } else if a > 2 { 0xff } else { 0xfe });
    }
    let (soma, codigo) = match gravidade {
        1 | 6 => (4, 5),
        2 => (10, 8),
        3 => (7, 1),
        4 | 5 => (100, 12),
        _ => (0, j as u8),
    };
    nota = nota.wrapping_add(soma);
    if byte(j, 0x24) != 0 && (gravidade == 1 || gravidade == 6) && sortear(byte(pessoa, 0x24) as i32) != 0 {
        *tipo = 9;
        return;
    }
    let ativo = byte(j, 0x19) >= 0 && flag == 0;
    if gravidade != 1 && nota as i8 as i32 > sortear(0xf) + 7 {
        if ativo {
            let d = (j + 0x35) as *mut i16;
            *d = (*d).wrapping_add(-1000);
            *tipo = 4;
            let t = byte(j, 0x23) as i32 + byte(j, 0x24) as i32;
            *((j + if t == 0 { 0x25 } else { 0x26 }) as *mut u8) = codigo;
        } else if codigo == 12 || codigo == 8 {
            registrar_cartao(j, codigo, true);
        }
        return;
    }
    let r = sortear(3 * byte(j, 0x24) as i32 + 5);
    if nota as i8 as i32 > r + 2 {
        if !ativo {
            return;
        }
        desconto_do_cartao(*(this as *const usize), j);
        let n = (j + 0x24) as *mut u8;
        if *n != 0 {
            *n = (*n).wrapping_add(1);
            *tipo = 3;
        } else {
            *n = 1;
            *tipo = 2;
        }
        registrar_cartao(j, codigo, false);
    } else if ativo {
        *tipo = 9;
    }
}

/// { 0x6cfd40 } (match_eng.cpp): `cartao_pela_gravidade` do arbitro do motor (this + 0xf5b2).
unsafe extern "thiscall" fn motor_cartao_pela_gravidade(this: usize, j: usize, tipo: *mut u8, gravidade: u8, flag: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8) = core::mem::transmute(0x6eab60usize);
    f(this + 0xf5b2, j, tipo, gravidade, flag)
}

// ---------- septuagesima leva: decisao do arbitro na falta (0x6e9820, 3 KB) ----------

/// { 0x6e9820 } (match_man.cpp): o arbitro decide a falta `tipo` do jogador `j` sobre `outro`.
/// Devolve 1 ou 7 (cartao dado; 0 = nada) e escreve o cartao (4, 5, 7, 10, 11), a gravidade e a
/// marca `vantagem`. `this`: +0 motor, +4 arbitro (ou 0), +8 paciencia (palavra). Fator de rigor
/// (dobra, 1,0 sem arbitro): pela severidade (+0x25), pelo modo do lado (+0x1ad do motor) e o
/// estilo (+0x27), pela vantagem e pela diferenca de placar (0xf5c9 + 54 x lado), limitado a
/// 0,1..10. Cada tipo de falta sorteia contra o fator; depois a gravidade sobe pela dureza do
/// jogador (+0x14) e cai pelo minuto (0x8ed0 / 11). Com 0x6cb650, aplica a gravidade em `outro`
/// (0x716450, 0x7164b0) e guarda `j` em +0x1a6 dele.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn decisao_da_falta(
    this: usize,
    j: usize,
    outro: usize,
    tipo: i8,
    quem: i8,
    cartao: *mut u8,
    grav: *mut i8,
    vantagem: *mut u8,
) -> u8 {
    *grav = 0;
    let mut res = 0u8;
    let m = *(this as *const usize);
    if m == 0 {
        return 0;
    }
    let parado: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6bab90usize);
    if parado(m) != 0 {
        return 0;
    }
    let paciencia = (this + 8) as *mut u16;
    let arb = *((this + 4) as *const usize);
    if arb != 0 && sortear(byte(arb, 0x28) as i32 + 0xf) == 0 {
        *paciencia = (*paciencia).wrapping_add(0xff06);
        return 0;
    }
    let minuto = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
    *paciencia = (*paciencia).wrapping_add(0xaf);
    let faixa_l: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x6f5b20usize);
    let dura = |t: i8| t == 0x2a || t == 0xc || t == 0x15;
    if arb != 0 && dura(tipo) && faixa_l(j, 0) == 0 && faixa_l(j, 1) == 0 && sortear(50 - byte(arb, 0x24) as i32) == 0 {
        *paciencia = (*paciencia).wrapping_add(0x19);
        return 0;
    }
    if (faixa_l(j, 0) != 0 || faixa_l(j, 1) != 0) && (dura(tipo) || tipo == 0x2e) && sortear(3) != 0 {
        return 0;
    }
    // vantagem: (20, ou 5 na faixa do proprio lado) x (+0x5c da ficha de `outro`)^1,5 / (1 ou 3)
    let a = if faixa_l(j, byte(j, 0x27)) != 0 { 5.0 } else { 20.0 };
    let b = byte(*((outro + 0x69) as *const usize), 0x5c) as f64;
    let div = if *((outro + 0x1a6) as *const usize) != j { 1.0 } else { 3.0 };
    *vantagem = (sortear(ftol(a * pow_do_jogo(b, f64_em(0x968940)) / div)) == 0) as u8;
    let palavra = |p: usize, d: i16| *((p + 0x31) as *mut i16) = (*((p + 0x31) as *const i16)).wrapping_add(d);
    if *vantagem != 0 {
        palavra(outro, -4);
    }
    let mut fator = 1.0f64;
    let mut rigor = 0i8;
    if arb != 0 {
        let sev = byte(arb, 0x25);
        fator = if sev > 10 { sev as f64 * f64_em(0x967880) } else { (sev as i32 - 10) as f64 * f64_em(0x967888) + f64_em(0x967890) };
        let lado_j = byte(j, 0x27);
        let modo = *((m as isize + lado_j as isize + 0x1ad) as *const i8) as f64;
        fator += modo / (byte(arb, 0x27) as f64 * f64_em(0x968968) + f64_em(0x968930));
        if byte(j, 0x24) != 0 {
            let x = sortear(21 - sev as i32);
            *grav = (*grav).wrapping_add((x / -5) as i8);
        }
        if *vantagem != 0 {
            let y = sortear(byte(arb, 0x28) as i32);
            fator *= f64_em(0x968940) - y as f64 * f64_em(0x967888);
        }
        rigor = match sev {
            20 => 2,
            18..=i8::MAX => 1,
            10..=19 => 0,
            5..=9 => -1,
            _ => (sev > 2) as i8 - 3,
        };
        let placar = *((m + 0xf5c9 + 54 * lado_j as usize) as *const u8) as i32;
        if placar > byte(arb, 0x27) as i32 + 5 {
            *paciencia = (*paciencia).wrapping_add(0xff6a);
            rigor = rigor.wrapping_add(1);
            let z = sortear(4);
            fator *= (f64_em(0x969000) - z as f64) * f64_em(0x967880) + f64_em(0x967890);
        }
    }
    let partida = *((m + 0x4792) as *const usize);
    if partida != 0 {
        let comp = *((partida + 0x14) as *const usize);
        let amistoso = *(0xae2c74 as *const usize);
        if comp != 0 && amistoso != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
            if f(amistoso, comp) != 0 {
                rigor = rigor.wrapping_sub(1);
            }
        }
    }
    if f64_em(0x967880) > fator {
        fator = 0.1;
    } else if f64_em(0x9688b0) < fator || fator.is_nan() {
        fator = 10.0;
    }
    let dureza = *((j + 0x14) as *const u8) as i32;
    let somar = |v: i32| *grav = (*grav).wrapping_add(v as i8);
    let pelo_minuto = |limite: i8| {
        if minuto < limite {
            *grav = sortear(*grav as i32) as i8;
        }
    };
    // ajuste final pela ficha de `outro` (+0x5c): 1 -> `um`; >= 6 -> +1 abaixo de 10; senao `senao`
    let pela_ficha = |um: i32, senao: i32| {
        if *vantagem != 0 && outro != 0 {
            let c = byte(*((outro + 0x69) as *const usize), 0x5c);
            if c == 1 {
                *grav = (*grav).wrapping_add(um as i8);
            } else if c >= 6 {
                if c < 10 {
                    *grav = (*grav).wrapping_add(1);
                }
            } else {
                *grav = (*grav).wrapping_add(senao as i8);
            }
        }
    };
    let teto = |v: f64, k: f64| if v > k { v } else { k };
    match tipo {
        0x2a => {
            palavra(j, -4);
            if sortear(ftol(fator * f64_em(0x969000))) != 0 {
                res = 1;
                somar(sortear(dureza) + 4);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        0x2b => {
            palavra(j, -3);
            if sortear(ftol(fator * f64_em(0x969558))) != 0 {
                res = 7;
                somar(sortear(dureza) + 3);
                *cartao = 4;
                pelo_minuto(15);
            }
        }
        0x2c => {
            let v = teto(fator * f64_em(0x9688b0), f64_em(0x968ff8));
            palavra(j, -12);
            if sortear(ftol(v)) != 0 {
                res = 1;
                somar(sortear(dureza + 20) + rigor as i32 + 9);
                *cartao = 10;
                pela_ficha(3, 2);
            }
        }
        0x2d => {
            palavra(j, -4);
            if sortear(ftol(fator * f64_em(0x968ea8))) != 0 {
                if faixa_l(j, byte(j, 0x27)) == 0 || sortear(0x12) == 0 {
                    res = 1;
                    somar(sortear(dureza) + rigor as i32 + 4);
                    *cartao = 7;
                }
                pelo_minuto(6);
            }
        }
        0x2e => {
            palavra(j, -1);
            let r = sortear(ftol(fator * f64_em(0x969000)));
            if r != 0 || (outro != 0 && tabela_do_jogador(outro) == 1 && sortear(100) != 0) {
                res = 1;
                somar(sortear(dureza) + rigor as i32 + 1);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        0x2f | 0x60 => {
            let (v, d) = if tipo == 0x2f {
                (teto(fator * f64_em(0x9688b0), f64_em(0x9688a8)), -9)
            } else {
                (fator * f64_em(0x968ea8), -25)
            };
            palavra(j, d);
            if sortear(ftol(v)) != 0 {
                res = 1;
                let y = sortear(if tipo == 0x2f { dureza } else { dureza + 20 });
                somar(y + rigor as i32 + 9);
                *cartao = 5;
                pela_ficha(5, 3);
            }
        }
        0x5f => {
            palavra(j, -12);
            if sortear(ftol(fator * f64_em(0x9688b0))) != 0 {
                res = 1;
                somar(sortear(dureza + 7) + rigor as i32 + 3);
                *cartao = 5;
            }
        }
        0x61 => {
            palavra(j, -25);
            if sortear(ftol(fator + fator)) != 0 {
                res = 1;
                somar(sortear(dureza) + rigor as i32 + 20);
                *cartao = 5;
            }
        }
        0x30 | 0x31 | 0x32 => {
            let (k, d, extra, mais) = match tipo {
                0x30 => (0x968878, -100, 0, 20),
                0x31 => (0x968908, -200, 10, 20),
                _ => (0x968908, -60, 0, 18),
            };
            palavra(j, d);
            if sortear(ftol(fator * f64_em(k))) != 0 {
                res = 1;
                somar(sortear(dureza + extra) + rigor as i32 + mais);
                *cartao = 11;
            }
        }
        0x13 | 0x21 | 0x22 => {
            let segue = sortear(ftol(f64_em(0x96d520) / fator)) == 0
                || (quem == 0x6c && sortear(if *vantagem != 0 { 2 } else { 6 }) == 0);
            if segue && (faixa_l(j, byte(j, 0x27)) == 0 || sortear(ftol(f64_em(0x969550) / fator)) == 0) {
                res = 1;
                let a = sortear(dureza);
                let b = sortear(5);
                somar(a + b + rigor as i32);
                *cartao = 7;
                pela_ficha(5, 3);
            }
        }
        0x23 | 0x0c | 0x24 => {
            if sortear(ftol(f64_em(0x968f50) / fator)) == 0 {
                res = if tipo == 0x0c { 7 } else { 1 };
                somar(sortear(dureza) + 1);
                *cartao = 4;
                if tipo == 0x24 {
                    pela_ficha(3, 2);
                }
            }
        }
        0x15 | 0x38 => {
            let r = sortear(ftol(f64_em(0x968f50) / fator));
            if r == 0 || (outro != 0 && tabela_do_jogador(outro) == 1 && sortear(5) == 0) {
                res = 1;
                somar(sortear(dureza) + 1);
                *cartao = 4;
                pelo_minuto(15);
            }
        }
        0x29 => {
            if sortear(ftol(f64_em(0x9688a8) / fator)) == 0 {
                res = 1;
                somar(sortear(dureza) + 2);
                *cartao = 7;
                pelo_minuto(10);
            }
        }
        0x01 => {
            if sortear(ftol(f64_em(0x968910) / fator)) == 0 {
                res = 1;
                somar(sortear(dureza) + 1);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        _ => {}
    }
    let aplica: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(0x6cb650usize);
    if aplica(m, tipo) != 0 {
        let multiplicar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(0x716450usize);
        let teste: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(0x7164b0usize);
        multiplicar(outro, (*grav as f64 * f64_em(0x968918) + f64_em(0x967890)) as f32);
        if teste(outro, 1, 0) != 0 {
            *((outro + 0xff) as *mut u8) = (*((outro + 0xff) as *const u8)).wrapping_add(1);
            if teste(outro, 1, 0) != 0 {
                *((outro + 0x1a6) as *mut usize) = j;
            }
        }
    }
    res
}

