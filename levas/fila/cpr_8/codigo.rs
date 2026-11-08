// ---------- %%LEVA%%a leva: as aposentadorias: quem se aposenta no começo do jogo, quem se aposenta pela idade e a lista inicial (player_regen.cpp) ----------

unsafe fn cpr8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpr8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpr8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpr8_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpr8_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) e, pelo clube dele (registro de 0x245 bytes pelo +4; sem
/// contrato, 0), o contrato da pessoa com esse clube (0x4cf5f0), que a rotina devolve.
unsafe fn cpr8_contratos(pessoa: usize) -> usize {
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = de(cpr8_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpr8_d(0xae23b4).wrapping_add(cpr8_d(c + 4).wrapping_mul(0x245)) } else { 0 };
    let com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    com(cpr8_d(0xae1050), pessoa, clube)
}

/// O estatuto do contrato com o clube (+0x3d); sem ele, −100 (0x9c).
unsafe fn cpr8_estatuto(c2: usize) -> i8 {
    if c2 == 0 {
        0x9cu8 as i8
    } else {
        cpr8_b(c2 + 0x3d)
    }
}

/// O limite de idade de quem não é jogador, pelo tipo do cargo (+0x3d; sem ele +0x28; sem os dois, como a original,
/// o byte baixo do ponteiro da saída, que ela lê da vaga errada da pilha): 1, 2 e 4: 70 + sorteio(10); 8 e 9:
/// 60 + sorteio(5); os outros: 61 + sorteio(5) com clube (+0x39) ou seleção (+0x24), senão 55 + sorteio(10). Byte.
unsafe fn cpr8_limite_sem_jogador(pessoa: usize, saida: usize) -> i8 {
    let mut t = cpr8_b(pessoa + 0x3d);
    if t == 0 {
        t = cpr8_b(pessoa + 0x28);
        if t == 0 {
            t = saida as u8 as i8; // ponytail: a original lê aqui o byte baixo do argumento `saida`
        }
    }
    let (n, base) = match t {
        1 | 2 | 4 => (10, 0x46u8),
        8 | 9 => (5, 0x3c),
        _ => {
            if cpr8_d(pessoa + 0x39) != 0 || cpr8_d(pessoa + 0x24) != 0 {
                (5, 0x3d)
            } else {
                (10, 0x37)
            }
        }
    };
    (sortear(n) as u8).wrapping_add(base) as i8
}

/// O limite de idade do jogador pela ficha: 34 com +0xf ≥ 15; senão 30 com +0x15 ≥ 15; senão 31 com +0x14 < 15;
/// senão 30 (bytes com sinal).
unsafe fn cpr8_limite_do_jogador(j: usize) -> i8 {
    if cpr8_b(j + 0xf) >= 15 {
        0x22
    } else if cpr8_b(j + 0x15) >= 15 {
        0x1e
    } else if cpr8_b(j + 0x14) < 15 {
        0x1f
    } else {
        0x1e
    }
}

/// A saída de quem se aposenta: o motivo (+0x18), +0x19 = 0, o id (+0) e o id do clube (+4; −1 sem clube).
unsafe fn cpr8_preencher(pessoa: usize, saida: usize, motivo: u8) {
    *((saida + 0x18) as *mut u8) = motivo;
    *((saida + 0x19) as *mut u8) = 0;
    core::ptr::write_unaligned(saida as *mut u32, cpr8_d(pessoa) as u32);
    let c = cpr8_d(pessoa + 0x39);
    core::ptr::write_unaligned((saida + 4) as *mut u32, if c != 0 { cpr8_d(c) as u32 } else { u32::MAX });
}

/// Amanhã (dia + 1, só a palavra) nas duas datas da saída (+8 e +0x10; ano em +0xa e +0x12).
unsafe fn cpr8_amanha(saida: usize) {
    let (dia, ano) = (cpr8_u(0xae2c90).wrapping_add(1), cpr8_u(0xae2c92));
    cpr8_pw(saida + 8, dia);
    cpr8_pw(saida + 0xa, ano);
    cpr8_pw(saida + 0x10, dia);
    cpr8_pw(saida + 0x12, ano);
}

/// Sem nação-chave válida na aposentadoria pela idade: hoje + sorteio(20) nas duas datas; passando de 365, o dia
/// sorteio(10) do ano seguinte.
unsafe fn cpr8_fora_por_idade(saida: usize) {
    let a = (sortear(20) as u32).wrapping_add(cpr8_d(0xae2c90) as u32) as u16;
    let ano = cpr8_u(0xae2c92);
    if a as i16 > 0x16d {
        let d = sortear(10) as u16;
        cpr8_pw(saida + 0x10, d);
        cpr8_pw(saida + 0x12, ano.wrapping_add(1));
        cpr8_pw(saida + 8, d);
        cpr8_pw(saida + 0xa, ano.wrapping_add(1));
    } else {
        cpr8_pw(saida + 8, a);
        cpr8_pw(saida + 0xa, ano);
        cpr8_pw(saida + 0x10, a);
        cpr8_pw(saida + 0x12, ano);
    }
}

/// As datas da aposentadoria na saída (+8 o anúncio, +0x10 a aposentadoria; só o dia e o ano, o +4 de cada data fica
/// como está), pela nação-chave do clube (0x669360) ou, sem clube, da nação (+0x1a, 0x669340); sem os dois, amanhã.
/// Índice 0..0x22: o anúncio em hoje + 1 + sorteio(95) (passando de 365, o dia sorteio(60) do ano seguinte); a
/// aposentadoria no fim de temporada da nação (+0x15 do registro de 72 bytes em 0xb63d60) + sorteio(30) (passando de
/// 365, sorteio(30) do ano seguinte; com +0x1d, um ano a mais); a aposentadoria antes do anúncio fica no anúncio, e a
/// menos de 45 dias dele (0x54cdb0) o anúncio vai para ela. Índice fora: `fora` (cada rotina tem a sua).
unsafe fn cpr8_datas(pessoa: usize, saida: usize, fora: unsafe fn(usize)) {
    let clube = cpr8_d(pessoa + 0x39);
    let k = if clube != 0 {
        let pelo_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
        pelo_clube(clube) as u8 as i8
    } else {
        let nacao = cpr8_d(pessoa + 0x1a);
        if nacao == 0 {
            return cpr8_amanha(saida);
        }
        let pela_nacao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669340usize);
        pela_nacao(nacao) as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return fora(saida);
    }
    crate::comparacao::ramo("cpr_8 datas pela nacao");
    let ano = cpr8_u(0xae2c92);
    let a = (sortear(95) as u32).wrapping_add(cpr8_d(0xae2c90) as u32).wrapping_add(1) as u16;
    let (dia1, ano1) = if a as i16 > 0x16d { (sortear(60) as u16, ano.wrapping_add(1)) } else { (a, ano) };
    cpr8_pw(saida + 0xa, ano1);
    cpr8_pw(saida + 8, dia1);
    let t = 0xb63d60usize.wrapping_add((k as usize) * 0x48);
    let d2 = (sortear(30) as u16).wrapping_add(cpr8_u(t + 0x15)); // o fim de temporada da nação
    cpr8_pw(saida + 0x10, d2);
    let mais_um = *((t + 0x1d) as *const u8) != 0;
    if d2 as i16 > 0x16d {
        cpr8_pw(saida + 0x10, sortear(30) as u16);
        cpr8_pw(saida + 0x12, ano.wrapping_add(if mais_um { 2 } else { 1 }));
    } else {
        cpr8_pw(saida + 0x12, if mais_um { ano.wrapping_add(1) } else { ano });
    }
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t8 = [0u8; 8];
    let (da, aa) = (cpr8_u(saida + 0x12), cpr8_u(saida + 0xa));
    if da < aa || (da == aa && cpr8_w(saida + 0x10) < cpr8_w(saida + 8)) {
        copia(saida + 0x10, t8.as_mut_ptr(), saida + 8);
        return;
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    if dias(saida + 0x10, cpr8_d(saida + 8) as u32, cpr8_d(saida + 0xc) as u32) >= 0x2d {
        return;
    }
    copia(saida + 8, t8.as_mut_ptr(), saida + 0x10);
}

/// { 0x7a79c0 } (thiscall sem ler o this, ret 8, player_regen.cpp): `player_regen_check_initial_retirement(pessoa,
/// saida)`: no jogo novo (pela 0x7a58f0), a pessoa com clube (+0x39) se aposenta nesta temporada? O contrato
/// (cpr8_contratos) e o estatuto (cpr8_estatuto). Sem ficha de jogador (+0x61): sem ficha de técnico (+0x69) sempre;
/// com ela, passando do limite (cpr8_limite_sem_jogador); motivo 0x20. Jogador: passando do limite da ficha
/// (cpr8_limite_do_jogador), v = ⌊⌊1 − (idade − limite) × 0,1⌋ × habilidade (+0xb)⌋ (16 bits); acima de 45 anos,
/// motivo 1. Clube sem competição (+0x57; sem clube, nação sem +0x11c): sorteio(3) ≠ 0 aposenta, senão só estatuto
/// negativo com clube. Senão: v ≥ 5000 ou +5 > 80, não; +5 < 10 ou v < 1500, sim; com clube, só estatuto negativo;
/// sem clube, motivo 2 com sorteio(25 × +0x3c) = 0. Quem se aposenta: a saída (cpr8_preencher) e as datas
/// (cpr8_datas; sem nação-chave válida, amanhã); devolve 1.
unsafe extern "thiscall" fn player_regen_check_initial_retirement(_this: usize, pessoa: usize, saida: usize) -> u8 {
    if cpr8_d(pessoa + 0x39) == 0 {
        return 0;
    }
    let c2 = cpr8_contratos(pessoa);
    let estatuto = cpr8_estatuto(c2);
    let idade = cpr8_b(pessoa + 0x18);
    let j = cpr8_d(pessoa + 0x61);
    let motivo: u8 = 'm: {
        if j == 0 {
            if cpr8_d(pessoa + 0x69) != 0 && idade <= cpr8_limite_sem_jogador(pessoa, saida) {
                return 0;
            }
            break 'm 0x20;
        }
        let lim = cpr8_limite_do_jogador(j);
        if idade <= lim {
            return 0;
        }
        let f = ftol(f64_em(0x967890) - (idade as i32 - lim as i32) as f64 * f64_em(0x967880)) as i16;
        let v = ftol(f as f64 * cpr8_w(j + 0xb) as f64) as i16;
        if idade > 0x2d {
            break 'm 1;
        }
        let clube = cpr8_d(pessoa + 0x39);
        let sorteia = if clube != 0 {
            cpr8_d(clube + 0x57) == 0
        } else {
            let n = cpr8_d(pessoa + 0x1a);
            n != 0 && *((n + 0x11c) as *const u8) == 0
        };
        if sorteia {
            if sortear(3) != 0 {
                break 'm 1;
            }
            if clube == 0 || estatuto >= 0 {
                return 0;
            }
            break 'm 1;
        }
        if v >= 0x1388 {
            return 0;
        }
        let hab = cpr8_w(j + 5);
        if hab > 0x50 {
            return 0; // cpr8: a habilidade alta segura o jogador
        }
        if hab < 0xa || v < 0x5dc {
            crate::comparacao::ramo("cpr_8 inicio habilidade");
            break 'm 1;
        }
        if clube != 0 {
            if estatuto < 0 {
                break 'm 1;
            }
            return 0;
        }
        if sortear(25 * cpr8_b(j + 0x3c) as i32) != 0 {
            return 0;
        }
        2
    };
    cpr8_preencher(pessoa, saida, motivo);
    cpr8_datas(pessoa, saida, cpr8_amanha);
    1
}

/// { 0x7a7310 } (thiscall sem ler o this, ret 8, player_regen.cpp): `player_regen_check_retirement(pessoa, saida)`:
/// na busca de novas aposentadorias (0x7a5790), a pessoa se aposenta? Sem ficha de jogador: como na 0x7a79c0 (motivo
/// 0x20). Jogador acima do limite da ficha: acima de 33 anos, o limite cai 1 sem contrato com o clube, ou com +0x45 no
/// contrato e +0x15/+0x14 da ficha = 20 ou o nibble alto de +0x4f = 7; com clube e contrato de salário (+0xc) acima de
/// 750 × (+0x59 + 10) até depois deste ano (+0x2f), cai 1 (e mais 1 com +0x59 < 6). v = ⌊(1 − (idade − limite) ×
/// 0,075) × habilidade (+0xb)⌋ (16 bits). Motivo 1: acima de 45; acima de 34 sem clube e desempregado há mais de 65
/// dias (desde +0x3e); 3 anos acima do limite, mais de 31 e +0x3e do contrato > 5; 4 acima: sem clube; ou contrato a
/// menos de 300 dias do fim (0x85fa00), estatuto abaixo de 50 e técnico (+0x69) ou +0x57 > sorteio(20) com o
/// potencial × 0,85 acima da habilidade atual (+7 e +5). Depois v > 5500, não. Clube em competição fora do jogo
/// ([[0xadadfc] + id × 4] = 0, ou sem +0x57; sem clube, nação sem +0x11c): sorteio(3) = 0 aposenta; com clube e
/// estatuto negativo também; senão só com v < 3500. Senão como na 0x7a79c0 (+5 > 100 não; +5 < 10 ou v < 1500 sim;
/// com clube só estatuto negativo; sem clube motivo 2 com sorteio(25 × +0x3c) = 0). Quem se aposenta: a saída e as
/// datas (sem nação-chave válida, cpr8_fora_por_idade); devolve 1.
unsafe extern "thiscall" fn player_regen_check_retirement(_this: usize, pessoa: usize, saida: usize) -> u8 {
    let c2 = cpr8_contratos(pessoa);
    let estatuto = cpr8_estatuto(c2);
    let idade = cpr8_b(pessoa + 0x18);
    let j = cpr8_d(pessoa + 0x61);
    let motivo: u8 = 'm: {
        if j == 0 {
            if cpr8_d(pessoa + 0x69) != 0 && idade <= cpr8_limite_sem_jogador(pessoa, saida) {
                return 0;
            }
            break 'm 0x20;
        }
        let mut lim = cpr8_limite_do_jogador(j);
        if idade <= lim {
            return 0;
        }
        if idade > 0x21 {
            if c2 == 0 {
                lim = lim.wrapping_sub(1);
            } else if cpr8_d(c2 + 0x45) != 0
                && (cpr8_b(j + 0x15) == 0x14 || cpr8_b(j + 0x14) == 0x14 || *((c2 + 0x4f) as *const u8) & 0xf0 == 0x70)
            {
                lim = lim.wrapping_sub(1);
            }
        }
        if cpr8_d(pessoa + 0x39) != 0 && c2 != 0 {
            let x = cpr8_b(pessoa + 0x59) as i32;
            if (cpr8_d(c2 + 0xc) as i32) > (x + 10).wrapping_mul(750) && cpr8_u(c2 + 0x2f) > cpr8_u(0xae2c92) {
                lim = lim.wrapping_sub(1);
                if x < 6 {
                    lim = lim.wrapping_sub(1);
                }
            }
        }
        let v = ftol((f64_em(0x967890) - (idade as i32 - lim as i32) as f64 * f64_em(0x96a730)) * cpr8_w(j + 0xb) as f64) as i16;
        if idade > 0x2d {
            break 'm 1;
        }
        if idade > 0x22 && cpr8_d(pessoa + 0x39) == 0 {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            if dias(0xae2c90, cpr8_d(pessoa + 0x3e) as u32, cpr8_d(pessoa + 0x42) as u32) > 0x41 {
                break 'm 1;
            }
        }
        let (i, l) = (idade as i32, lim as i32);
        if i > l + 2 && idade > 0x1f && c2 != 0 && cpr8_b(c2 + 0x3e) > 5 {
            break 'm 1;
        }
        if i > l + 3 {
            if cpr8_d(pessoa + 0x39) == 0 {
                break 'm 1;
            }
            let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x85fa00usize);
            if c2 != 0 && (fim(c2) as i16) < 300 && estatuto < 0x32 {
                if cpr8_d(pessoa + 0x69) != 0 {
                    break 'm 1;
                }
                if cpr8_b(pessoa + 0x57) as i32 > sortear(20) {
                    let (cur, pot) = (cpr8_w(j + 5) as f64, cpr8_w(j + 7) as f64);
                    if !(pot * f64_em(0x969520) <= cur) {
                        break 'm 1;
                    }
                }
            }
        }
        if v > 0x157c {
            return 0;
        }
        let clube = cpr8_d(pessoa + 0x39);
        let sorteia = if clube != 0 {
            let lg = cpr8_d(clube + 0x57);
            lg == 0 || cpr8_d(cpr8_d(0xadadfc).wrapping_add(cpr8_d(lg).wrapping_mul(4))) == 0
        } else {
            let n = cpr8_d(pessoa + 0x1a);
            n != 0 && *((n + 0x11c) as *const u8) == 0
        };
        if sorteia {
            crate::comparacao::ramo("cpr_8 fora do jogo");
            if sortear(3) == 0 {
                break 'm 1; // cpr8: um em três se aposenta
            }
            if cpr8_d(pessoa + 0x39) != 0 && estatuto < 0 {
                break 'm 1;
            }
            if v >= 0xdac {
                return 0;
            }
            break 'm 1;
        }
        let cur = cpr8_w(j + 5);
        if cur > 0x64 {
            return 0;
        }
        if cur < 0xa || v < 0x5dc {
            break 'm 1;
        }
        if clube != 0 {
            if estatuto < 0 {
                break 'm 1;
            }
            return 0;
        }
        if sortear(25 * cpr8_b(j + 0x3c) as i32) != 0 {
            return 0;
        }
        2
    };
    cpr8_preencher(pessoa, saida, motivo);
    cpr8_datas(pessoa, saida, cpr8_fora_por_idade);
    1
}

/// { 0x7a58f0 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::build_initial_retirement_list`: no jogo novo, cada pessoa (0x6e bytes em [0xae23bc], até [0xae2364],
/// relida a cada passo) de id abaixo de [0xae2364] − 16, sem os bits 0x30 do +0xb do registro de 79 bytes
/// ([0xae2d0c]) e sem contrato (0x4cf5a0) com +0x49 é testada (0x7a79c0); quem se aposenta entra na lista
/// (0x7a8510). No fim o qsort da lista ([this+4], conta +8, 26 bytes, 0x7ae2a0).
unsafe extern "thiscall" fn build_initial_retirement_list(this: usize) {
    crate::comparacao::ramo("cpr_8 lista inicial");
    // ponytail: a original constrói as duas datas da entrada (0x54a110) e deixa o resto como estava na pilha; a
    // 0x7a79c0 grava tudo o que a 0x7a8510 copia antes de devolver 1 (a terceira data local nunca é lida: fica de fora)
    let mut e = [0u8; 0x1a];
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    padrao(e.as_mut_ptr().add(8));
    padrao(e.as_mut_ptr().add(0x10));
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let testar: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x7a79c0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x7a8510usize);
    let mut p = cpr8_d(0xae23bc);
    let mut i = 0i32;
    while i < cpr8_d(0xae2364) as i32 {
        let id = cpr8_d(p) as i32;
        if id < (cpr8_d(0xae2364) as i32).wrapping_sub(0x10)
            && *((cpr8_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 == 0
        {
            let c = de(cpr8_d(0xae1050), p);
            if (c == 0 || *((c + 0x49) as *const u8) == 0) && testar(this, p, e.as_mut_ptr()) != 0 {
                acrescentar(this, e.as_ptr());
            }
        }
        i += 1;
        p = p.wrapping_add(0x6e);
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpr8_d(this + 4), cpr8_d(this + 8) as u32, 0x1a, 0x7ae2a0);
}

/// As regiões que as duas decisões mudam: o sorteio, [0xae1080], a saída e a data estática da 0x54cdb0.
unsafe fn cpr8_regioes(saida: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&[(0xae1080, 4), (saida, 0x1a), (0xae2470, 8), (0xae2418, 1)]);
    reg
}

/// Comparador: lado a lado pelo sorteio, [0xae1080], a saída e a data estática (a cópia sem contar), e a devolução.
unsafe extern "thiscall" fn comparar_player_regen_check_initial_retirement(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&cpr8_regioes(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_initial_retirement(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_initial_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparador: como o da 0x7a79c0.
unsafe extern "thiscall" fn comparar_player_regen_check_retirement(this: usize, pessoa: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&cpr8_regioes(saida), || o(this, pessoa, saida),
        || crate::aleatorio::sem_contar(|| player_regen_check_retirement(this, pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("player_regen_check_retirement(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ponytail: a lista inicial acrescenta (realloc) e sorteia em cada teste: não roda duas vezes. Só a original; o modo
// Rust e a carreira provam a cópia.
unsafe extern "thiscall" fn comparar_build_initial_retirement_list(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
