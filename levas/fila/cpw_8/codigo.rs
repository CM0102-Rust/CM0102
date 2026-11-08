// ---------- %%LEVA%%a leva: o físico do dia do jogador (a forma e a condição de jogo até hoje), a faixa de confiança da diretoria na liga e as notas da diretoria (injury.cpp, manager_manager.cpp) ----------

unsafe fn cpw8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw8_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpw8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpw8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A data `destino` recebe `origem` (0x417c10, com a devolução numa local).
unsafe fn manager_manager_copy_date(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copiar(destino, t.as_mut_ptr(), origem);
}

/// A diferença entre hoje ([0xae2c90]) e a data `d` (0x54cc30): os dias (+0, com sinal) mais 365 × os anos (+2).
unsafe fn days_since(d: usize) -> i32 {
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let mut r = [0u8; 8];
    diferenca(0xae2c90, r.as_mut_ptr(), cpw8_d(d) as u32, cpw8_d(d + 4) as u32);
    let p = r.as_ptr() as usize;
    (cpw8_uw(p + 2) as i32).wrapping_mul(365).wrapping_add(cpw8_w(p) as i32)
}

/// O ensaio do comparador: a cópia roda sem o fim da lesão (0x62e700: histórico em arquivo e notícia), que não pode
/// rodar duas vezes; chegando lá, marca `END_INJURY_REACHED` e o comparador fica com a original sozinha.
static mut END_INJURY_DRY_RUN: bool = false;
static mut END_INJURY_REACHED: bool = false;

unsafe fn end_injury(this: usize, pessoa: usize) {
    if END_INJURY_DRY_RUN {
        END_INJURY_REACHED = true;
        return;
    }
    let terminar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62e700usize);
    terminar(this, pessoa, 1);
}

/// A estática local da guarda [0xb63d10] (bit `bit`): na primeira vez, Data() (0x54a110) e o destrutor vazio no atexit.
unsafe fn init_static_date(bit: u8, data: usize, destrutor: usize) {
    let g = 0xb63d10 as *mut u8;
    if *g & bit == 0 {
        *g |= bit;
        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
        padrao(data);
        let atexit: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x94627ausize);
        atexit(destrutor);
    }
}

/// O trecho da forma acima de `base`: (x − base) × a constante `k` (f64), truncado a 16 bits com sinal, mais `base`.
unsafe fn scale_above_base(x: i32, base: i32, k: usize) -> i32 {
    (match_pl_ftol(x.wrapping_sub(base) as f64 * read_f64_at(k)) as i16 as i32).wrapping_add(base)
}

/// { 0x62c680 } (thiscall, ret 4, injury.cpp, quadro SEH só das estáticas locais): `Injury::update_player_fitness(
/// pessoa)`: uma vez por dia (a data +0 do registro de 31 bytes da pessoa em [this] já é hoje: nada), pela condição de
/// jogo (0x62c510), pelo estado da lesão (0x62c400), pelo dia das lesões (0x62b7b0) e pelas notícias da volta (0x62e010):
/// toda pessoa que joga, todo dia em que alguém pergunta. T = dias desde a última atualização (0x54cc30). Lesionada
/// (+0xe na tabela de 0x30 bytes em [this+4]): pelos dias desde o começo (+0x20) contra +0x2a e +0x28, os dias parados
/// (perda), em recuperação ou livres, com o fim da lesão (0x62e700) quando passa de +0x28. Sem lesão: dias livres, salvo
/// na seleção com data futura (0x76d780/0x76e040) ou no clube fora da temporada (0x669bd0, guardado em 0xb63d2c/0xb63d30
/// com as datas 0xb63d18/0xb63d20): férias (o byte +0xa do registro de 79 bytes sobe com sorteio(+0x5b / 3 + 2) zero, até
/// 50) e a volta da pré-temporada (15 dias, ou o meio de um sorteio(2) com forma × condição abaixo de 80%). A forma (+8):
/// recuperação de 1000 − 12 × (+0x1e − natural (+0x34 da ficha), 0..100) por dia livre, perda de 66 por dia parado, teto
/// pela forma natural, trechos acima de 7000/9000/9850 encolhidos (0x96af30, 0x96bad0, 0x968968, 0x968918, 0x967880) e −55
/// por dia de férias; a condição de jogo (+0xa) sobe 500 × dias livres e 250 × em recuperação pelo fator do cansaço
/// (+0xc).
/// Grava as duas e a data de hoje.
unsafe extern "thiscall" fn update_player_fitness(this: usize, pessoa: usize) {
    init_static_date(1, 0xb63d08, 0x62ce20);
    init_static_date(2, 0xb63d18, 0x62ce10);
    init_static_date(4, 0xb63d20, 0x62ce00);
    if pessoa == 0 {
        return;
    }
    let id = cpw8_i(pessoa);
    if id >= cpw8_i(0xae2364) {
        return;
    }
    let ficha = cpw8_d(pessoa + 0x61);
    if ficha == 0 {
        return;
    }
    let rec = cpw8_d(this).wrapping_add((id as u32).wrapping_mul(31) as usize);
    if cpw8_uw(rec + 2) == cpw8_uw(0xae2c92) && cpw8_w(rec) == cpw8_w(0xae2c90) {
        return; // já atualizado hoje
    }
    let t = days_since(rec);
    let atual = cpw8_i(rec + 0xe);
    if atual > cpw8_i(this + 8) || atual < -1 {
        core::ptr::write_unaligned((rec + 0xe) as *mut i32, -1);
    }
    // os dias livres (l38), em recuperação (l3c), parados (l34) e de férias (l30)
    let (mut l3c, mut l38, mut l34, mut l30) = (0i32, 0i32, 0i32, 0i32);
    let i = cpw8_i(rec + 0xe);
    if i != -1 {
        let inj = cpw8_d(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
        let e = days_since(inj + 0x20);
        let s = cpw8_w(inj + 0x2a) as i32;
        if e < s {
            l34 = t;
        } else {
            let f = cpw8_w(inj + 0x28) as i32;
            if e < f {
                if t < s.wrapping_add(e) {
                    l3c = e - s;
                    l34 = t - (e - s);
                } else {
                    l3c = t;
                }
            } else if t > e - s {
                l38 = t - f;
                l3c = (e - s) - l38;
                l34 = f - l3c;
                end_injury(this, pessoa);
            } else {
                l38 = t;
                end_injury(this, pessoa);
            }
        }
    } else {
        l38 = t;
        let selecao = cpw8_d(pessoa + 0x24);
        let mut ocupado = false;
        if selecao != 0 {
            let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x76d780usize);
            let k = classe(selecao, (*((pessoa + 0x5e) as *const u8) & 8) as u32);
            let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x76e040usize);
            // ponytail: a original empurra o argumento com o al por cima (o resto é o ponteiro da pessoa); só o byte conta
            let d = campo(cpw8_d(0xb74318), cpw8_d(selecao + 0x53), (pessoa as u32 & 0xffff_ff00) | k as u32);
            let (ano, hoje) = (cpw8_uw(d + 2), cpw8_uw(0xae2c92));
            ocupado = if ano > hoje {
                true
            } else if ano < hoje {
                false
            } else {
                cpw8_w(d) > cpw8_w(0xae2c90)
            };
        }
        let clube = cpw8_d(pessoa + 0x39);
        if clube != 0 && !ocupado {
            if clube != cpw8_d(0xb63d2c) || cpw8_uw(0xb63d0a) != cpw8_uw(0xae2c92) || cpw8_w(0xb63d08) != cpw8_w(0xae2c90) {
                let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);
                let r = na_temporada(clube, 0xb63d18, 0xb63d20);
                *(0xb63d30 as *mut u32) = (r == 0) as u32;
                *(0xb63d2c as *mut usize) = cpw8_d(pessoa + 0x39);
                manager_manager_copy_date(0xb63d08, 0xae2c90);
            }
            if cpw8_i(0xb63d30) != 0 {
                l38 = 0;
                let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                let u6 = dias_ate(0xae2c90, cpw8_d(0xb63d20) as u32, cpw8_d(0xb63d24) as u32) as u16;
                let u7 = dias_ate(0xb63d18, cpw8_d(0xae2c90) as u32, cpw8_d(0xae2c94) as u32) as u16;
                if u6 < u7 {
                    let lim = if (u6 as i32) < t { u6 as i32 } else { t };
                    l30 = lim;
                    l3c = t - lim;
                    for _ in 0..lim {
                        let g = cpw8_d(0xae2d0c).wrapping_add((cpw8_i(pessoa) as u32).wrapping_mul(0x4f) as usize) + 0xa;
                        if *(g as *const u8) >= 0x32 {
                            break;
                        }
                        if cpw8_sortear(cpw8_b(pessoa + 0x5b) as i32 / 3 + 2) == 0 {
                            *(g as *mut u8) = (*(g as *const u8)).wrapping_add(1);
                        }
                    }
                } else {
                    let mut x = u6 as i32 - (u7 as i32 + u6 as i32) / 2;
                    if x < 0 {
                        x = 0;
                    }
                    if t < x {
                        x = t;
                    }
                    l3c = x;
                    l34 = t - x;
                }
                if u7 < 0xf {
                    l38 = 0xf - u7 as i32;
                    l3c -= l38;
                    if l3c < 0 {
                        l3c = 0;
                    }
                } else {
                    let (f, mc) = (cpw8_w(rec + 8) as i32, cpw8_w(rec + 0xa) as i32);
                    if mc.wrapping_mul(f) / 10000 < 8000 { // cpw8: a forma abaixo de 80% na pré-temporada
                        crate::comparacao::ramo("cpw_8 sorteio da forma");
                        let r = cpw8_sortear(2);
                        l38 = (r + l3c) / 2;
                        l3c -= l38;
                    }
                }
            }
        }
    }
    // a forma
    let f = cpw8_w(rec + 8) as i32;
    let natural = cpw8_b(ficha + 0x34);
    let mut v = cpw8_b(rec + 0x1e).wrapping_sub(natural);
    if v < 0 {
        v = 0;
    } else if v > 100 {
        v = 100;
    }
    let s = (1000 - 12 * v as i32) as i16 as i32; // cpw8: a recuperação por dia livre
    let metade = s / 2;
    let n = natural as i32;
    let piso = ((n * 25 / 20 + 15) * 100) as i16 as i32;
    if l34 != 0 {
        crate::comparacao::ramo("cpw_8 perda");
    }
    let mut x = f.wrapping_sub(66i32.wrapping_mul(l34)); // cpw8: a perda por dia parado
    if x < piso {
        x = 4000;
    }
    let m = n * 16 / 20 + 0x54;
    let mut teto = m.wrapping_mul(100);
    if (teto as i16 as i32) < x {
        teto = x;
    }
    x = x.wrapping_add((metade as i16 as i32).wrapping_mul(l3c));
    let teto16 = teto as i16 as i32;
    if x > teto16 {
        x = teto16;
    }
    if l38 != 0 {
        crate::comparacao::ramo("cpw_8 recuperacao");
    }
    x = x.wrapping_add(s.wrapping_mul(l38));
    if f > 0x267a {
        x = scale_above_base(x, f, 0x967880);
    } else if f > 0x2328 {
        x = scale_above_base(x, f, 0x968918);
        if x > 0x267a {
            x = scale_above_base(x, 0x267a, 0x968968);
        }
    } else if x > 0x1b58 {
        x = scale_above_base(x, 0x1b58, 0x96af30);
        if x > 0x2328 {
            x = scale_above_base(x, 0x2328, 0x96bad0);
        }
        if x > 0x267a {
            x = scale_above_base(x, 0x267a, 0x968968);
        }
    }
    if x > 10000 {
        x = 10000;
    }
    let mut forma = x.wrapping_sub(55i32.wrapping_mul(l30));
    if forma < piso {
        forma = 5000;
    }
    let mut e2 = m;
    if (m as i16 as i32) < forma {
        e2 = forma;
    }
    let e16 = e2 as i16 as i32;
    if forma > e16 {
        forma = e16;
    }
    // a condição de jogo, pelo fator do cansaço (+0xc)
    let cansaco = cpw8_w(rec + 0xc);
    let mc = cpw8_w(rec + 0xa) as i32;
    let teto_k = read_f64_at(0x9689d0);
    let k = if cansaco > 1000 {
        let kv = read_f64_at(0x968dc0) - cansaco as f64 * read_f64_at(0x96a608);
        if !(kv > teto_k) {
            kv
        } else if l3c > 0 {
            teto_k
        } else {
            kv
        }
    } else if l3c > 0 {
        teto_k
    } else {
        read_f64_at(0x967890)
    };
    let r1 = match_pl_ftol(l38.wrapping_mul(500) as f64 * k);
    let r2 = match_pl_ftol(l3c.wrapping_mul(250) as f64 * k);
    let mut cond = r2.wrapping_add(mc).wrapping_add(r1);
    if cond > 10000 {
        cond = 10000;
    }
    core::ptr::write_unaligned((rec + 8) as *mut i16, forma as i16);
    core::ptr::write_unaligned((rec + 0xa) as *mut i16, cond as i16);
    manager_manager_copy_date(rec, 0xae2c90);
}

/// Comparador lado a lado: o registro de 31 bytes da pessoa, o byte +0xa do registro de 79 bytes, as estáticas e o
/// guardado do clube (0xb63d08..0xb63d34) e o sorteio. A cópia roda em ensaio, sem o fim da lesão (0x62e700): quando
/// ela chega lá, fica a original sozinha (já rodou primeiro). Na primeira vez (estáticas por criar, com atexit) e sem
/// registro (pessoa nula, id fora da tabela, sem ficha): só a original.
unsafe extern "thiscall" fn comparar_update_player_fitness(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    if *(0xb63d10 as *const u8) & 7 != 7 || pessoa == 0 || cpw8_i(pessoa) < 0 || cpw8_i(pessoa) >= cpw8_i(0xae2364) || cpw8_d(pessoa + 0x61) == 0 {
        return o(this, pessoa); // ponytail: as estáticas e o atexit da primeira vez não podem rodar duas vezes
    }
    let id = cpw8_i(pessoa) as usize;
    let rec = cpw8_d(this).wrapping_add(id.wrapping_mul(31));
    let geral = cpw8_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f));
    let e = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(rec, 31), (geral + 0xa, 1), (0xb63d08, 0x2c), e[0], e[1]];
    END_INJURY_REACHED = false;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || {
        END_INJURY_DRY_RUN = true;
        crate::aleatorio::sem_contar(|| update_player_fitness(this, pessoa));
        END_INJURY_DRY_RUN = false;
    });
    if END_INJURY_REACHED {
        crate::comparacao::ramo("cpw_8 fim da lesao so na original");
        return;
    }
    PLACAR.registrar(igual, || format!("update_player_fitness(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// { 0x690bd0 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_league_confidence_band(clube)`: a
/// confiança da diretoria na liga (palavra com sinal +0x12 do registro de 0x49 bytes do clube em [this]) em faixas:
/// abaixo de −2000: 0; −1000: 1; −500: 2; 500: 3; 1000: 4; 2000: 5; senão 6. Pelo texto da confiança (0x690d80).
/// Só o al conta (o resto do eax é o que sobrou da conta do endereço).
unsafe extern "thiscall" fn get_league_confidence_band(this: usize, clube: usize) -> u8 {
    let v = cpw8_w(cpw8_d(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize) + 0x12);
    if v < -2000 {
        0
    } else if v < -1000 {
        1
    } else if v < -500 {
        2
    } else if v < 500 {
        3
    } else if v < 1000 {
        4
    } else if v < 2000 {
        5
    } else {
        6
    }
}

unsafe extern "thiscall" fn comparar_get_league_confidence_band(this: usize, clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, clube);
    let n = get_league_confidence_band(this, clube);
    PLACAR.registrar(r == n, || format!("get_league_confidence_band(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x69f7c0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::set_board_ratings(clube, valor)`: as quatro
/// notas da diretoria (+6, +8, +0xc e +0xa do registro de 0x49 bytes do clube em [this], palavras, nessa ordem) recebem
/// o valor (16 bits). Pela configuração inicial dos jogadores (0x7a46a6).
unsafe extern "thiscall" fn set_board_ratings(this: usize, clube: usize, valor: u32) {
    for off in [6usize, 8, 0xc, 0xa] {
        let r = cpw8_d(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize);
        core::ptr::write_unaligned((r + off) as *mut u16, valor as u16);
    }
}

unsafe extern "thiscall" fn comparar_set_board_ratings(this: usize, clube: usize, valor: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%);
    let r = cpw8_d(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x49)], || o(this, clube, valor), || set_board_ratings(this, clube, valor));
    PLACAR.registrar(igual, || format!("set_board_ratings(clube {:x}, {:x}) {:x?} x {:x?}", clube, valor, mo, mr));
}
