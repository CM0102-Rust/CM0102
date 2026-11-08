// ---------- %%LEVA%%a leva: quem serve para o cargo e a moral depois da partida ruim: o candidato a técnico, o cargo que a pessoa pode assumir e baixar a moral pela partida (manager_manager.cpp, contract_manager.cpp) ----------

unsafe fn cpp4_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn read_u8_volatile(a: usize) -> u8 {
    // ponytail: volátil porque a original lê [0 + 0x53] quando falta o clube do contrato (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}

/// A reputação "vista" de um clube para o técnico: com nação (+0x53), o peso dela (+0x85) × reputação (+0x80) × 0,05
/// (0x967888); sem, a reputação × 0,65 (0x96a5d0); truncadas (16 bits) e, acima de 0x203a, 0x203a − (excesso × −1,1
/// truncado).
unsafe fn perceived_club_reputation(clube: usize) -> i16 {
    let r = cpp4_w(clube + 0x80) as i32;
    let nacao = cpp4_d(clube + 0x53) as usize;
    let v = if nacao != 0 {
        match_pl_ftol((read_u8_volatile(nacao + 0x85) as i8 as i32 * r) as f64 * read_f64_at(0x967888))
    } else {
        match_pl_ftol(r as f64 * read_f64_at(0x96a5d0))
    } as i16;
    if v > 0x203a {
        (0x203a - match_pl_ftol((v as i32 - 0x203a) as f64 * read_f64_at(0x96d278))) as i16
    } else {
        v
    }
}

/// { 0x69f810 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::is_staff_suitable_for_job(pessoa, clube,
/// nota_minima)`: sem ficha de não jogador (+0x69) ou sem clube: 0. Os vetos (0): reputação de outra nação (+0xc) acima
/// de 4250 de outra nação com peso (+0x85) abaixo de 10; doméstica (+8) acima de 6250, técnico (+0x3d = 5) de outra
/// nação e clube abaixo de 5750; seleção que já é uma das três dele (+0x65 +0x10..+0x18); jogador (0xb) de nota
/// (+0x61 +0xb) acima de 4750 num clube abaixo de 6750, com contrato (0x4cf5a0) de faixa 1..3 (+0x4f >> 4), menos de
/// 180 dias, +0x3f acima de 1, +0x3e zero, menor de 35 com +0x3d positivo, ou de outra nação em clube abaixo de 7250
/// que não é o dele; técnico de clube numa divisão melhor (+0x57 +0x69), ou mais forte, ou recente (dias desde +0x3e
/// abaixo de (+0x59 + 21) × 10) num clube acima de 5250; técnico de seleção (+0x24, +0x28 = 5); clube abaixo de 2250
/// (com reputação +0xa acima de 4750 ou sem ficha); +0x22 acima de 75 num clube abaixo de 3250; não jogador com a
/// aposentadoria pendente (0x7ae350). Os sim (1): clube abaixo de 2500 da nação dele, doméstica abaixo de 1500, sem
/// clube nem seleção; seleção da nação dele com reputação abaixo de 6750, sem clube, ou com o clube dele mais fraco
/// (abaixo de 7750 e 750 abaixo da seleção) há tempo; clube acima de 7750 com a reputação vista acima da do clube dele
/// + 25 × +0x59 + 250. Senão, o interesse dele no cargo (0x844780, float) não abaixo de `nota_minima`.
unsafe extern "thiscall" fn is_staff_suitable_for_job(_this: usize, pessoa: usize, clube: usize, nota_minima: f64) -> u8 {
    let ficha = cpp4_d(pessoa + 0x69) as usize;
    if ficha == 0 || clube == 0 {
        return 0;
    }
    let rep = |c: usize| cpp4_w(c + 0x80);
    if cpp4_w(ficha + 0xc) > 0x109a {
        let (n1, n2) = (cpp4_d(pessoa + 0x1a) as usize, cpp4_d(clube + 0x53) as usize);
        if n1 != 0 && n2 != 0 && n1 != n2 && (read_u8_volatile(n2 + 0x85) as i8) < 0xa {
            return 0;
        }
    }
    if cpp4_w(ficha + 8) > 0x186a && cpp4_d(pessoa + 0x1a) != cpp4_d(clube + 0x53) && read_u8_volatile(pessoa + 0x3d) == 5 && rep(clube) < 0x1676 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        let s = cpp4_d(pessoa + 0x65) as usize;
        if s != 0 && (cpp4_d(s + 0x10) as usize == clube || cpp4_d(s + 0x14) as usize == clube || cpp4_d(s + 0x18) as usize == clube) {
            return 0;
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let tempo = |p: usize| dias(0xae2c90, cpp4_d(p + 0x3e), cpp4_d(p + 0x42));
    let limit = |p: usize| (read_u8_volatile(p + 0x59) as i8 as i32 + 0x15) * 10;
    if read_u8_volatile(pessoa + 0x3d) == 0xb {
        let jogador = cpp4_d(pessoa + 0x61) as usize;
        if jogador != 0 && cpp4_w(jogador + 0xb) > 0x128e && rep(clube) < 0x1a5e {
            let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contract_of(cpp4_d(0xae1050) as usize, pessoa);
            if c != 0 {
                let band = read_u8_volatile(c + 0x4f) >> 4;
                if band != 0 && band <= 3 {
                    return 0;
                }
                if dias(0xae2c90, cpp4_d(c + 0x25), cpp4_d(c + 0x29)) < 0xb4 {
                    return 0;
                }
                if read_u8_volatile(c + 0x3f) as i8 > 1 || read_u8_volatile(c + 0x3e) == 0 {
                    return 0;
                }
                if (read_u8_volatile(pessoa + 0x18) as i8) < 0x23 && read_u8_volatile(c + 0x3d) as i8 > 0 {
                    return 0;
                }
                if cpp4_d(pessoa + 0x1a) != cpp4_d(clube + 0x53) && rep(clube) < 0x1c52 && cpp4_d(pessoa + 0x39) as usize != clube {
                    return 0;
                }
            }
        }
    }
    if read_u8_volatile(pessoa + 0x3d) == 5 {
        let liga = cpp4_d(clube + 0x57) as usize;
        let atual = cpp4_d(pessoa + 0x39) as usize;
        if liga != 0 && atual != 0 && cpp4_d(atual + 0x57) != 0 {
            let liga2 = cpp4_d(atual + 0x57) as usize;
            if cpp4_w(liga + 0x69) < cpp4_w(liga2 + 0x69) {
                return 0; // o clube dele está numa divisão melhor
            }
            if liga == liga2 && (rep(clube) as i32 - 0x4e2) < rep(atual) as i32 {
                return 0;
            }
            if rep(atual) > rep(clube) {
                return 0;
            }
            if tempo(pessoa) < limit(pessoa) && rep(cpp4_d(pessoa + 0x39) as usize) > 0x1482 {
                return 0;
            }
        }
    }
    if cpp4_d(pessoa + 0x24) != 0 && read_u8_volatile(pessoa + 0x28) == 5 {
        return 0;
    }
    let ficha = cpp4_d(pessoa + 0x69) as usize;
    if (ficha == 0 || cpp4_w(ficha + 0xa) > 0x128e) && rep(clube) < 0x8ca {
        return 0;
    }
    if read_u8_volatile(pessoa + 0x22) > 0x4b && rep(clube) < 0xcb2 {
        return 0;
    }
    if cpp4_d(pessoa + 0x61) == 0 {
        let pendente: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7ae350usize);
        if pendente(cpp4_d(0xb7434c) as usize, pessoa) != 0 {
            return 0;
        }
    }
    if rep(clube) < 0x9c4 && cpp4_d(pessoa + 0x1a) == cpp4_d(clube + 0x53) && cpp4_w(cpp4_d(pessoa + 0x69) as usize + 8) < 0x5dc
        && cpp4_d(pessoa + 0x39) == 0 && cpp4_d(pessoa + 0x24) == 0 {
        return 1;
    }
    if selecao(clube) != 0 && cpp4_d(pessoa + 0x1a) == cpp4_d(clube + 0x53) {
        if cpp4_w(cpp4_d(pessoa + 0x69) as usize + 0xa) < 0x1a5e {
            return 1;
        }
        if cpp4_d(pessoa + 0x39) == 0 {
            return 1;
        }
        if tempo(pessoa) > limit(pessoa) {
            let r = rep(cpp4_d(pessoa + 0x39) as usize);
            if r < 0x1e46 && (r as i32 + 0x2ee) < rep(clube) as i32 {
                return 1;
            }
        }
    }
    if rep(clube) > 0x1e46 {
        let vista = perceived_club_reputation(clube);
        let atual = cpp4_d(pessoa + 0x39) as usize;
        if atual != 0 {
            let dele = perceived_club_reputation(atual);
            let teto = (read_u8_volatile(pessoa + 0x59) as i8 as i16).wrapping_mul(25) as i32 + dele as i32 + 0xfa;
            if vista as i32 > teto {
                return 1;
            }
        }
    }
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> f64 = core::mem::transmute(0x844780usize);
    let nota = interesse(cpp4_d(cpp4_d(0xde1f54) as usize + 0x856) as usize, pessoa, clube, 0, 5, 0);
    if nota >= nota_minima { 1 } else { 0 } // fcomp: abaixo (ou sem ordem) é 0
}

/// Comparador: a rotina só lê (o interesse 0x844780 sem a saída das razões): pela devolução.
unsafe extern "thiscall" fn comparar_is_staff_suitable_for_job(this: usize, pessoa: usize, clube: usize, nota_minima: f64) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8>(%%I0%%)(this, pessoa, clube, nota_minima);
    let n = is_staff_suitable_for_job(this, pessoa, clube, nota_minima);
    PLACAR.record(r == n, || format!("is_staff_suitable_for_job(pessoa {:x}, clube {:x}, {}) original={} rust={}", pessoa, clube, nota_minima, r, n));
    r
}

/// { 0x4cb990 } (stdcall, ret 8, contract_manager.cpp): `staff_can_take_job(cargo, pessoa)`: pela tabela de salto
/// 0x4cbaf4 (cargo − 6, 0..9; fora: 0); 7, 12 e 14: 0; pessoa nula: 0 (o regen de não jogador 0x7ab9f2 pergunta sempre
/// com nula). 6: com 30 anos (+0x18) ou mais, não jogador (+0x69), acima de 36, ou com +0x57 abaixo de 10 e +0x2f do
/// jogador 12 ou mais: 1; senão, não jogador no cargo 8, 6 ou 5. 8: de 30 anos, não jogador ou acima de 36 ou (+0x2f ≥ 12
/// e +0x57 < 10) fora dos cargos 5 e 10: 1; senão como o 6. 9: não jogador no cargo 9. 10: não jogador no cargo 10 ou com
/// +0x28 = 10. 11: jogador (+0x61). 13 e 15: jogador e não jogador, ou acima de 34, ou de 30 anos com +0x2f ≥ 12.
unsafe extern "stdcall" fn staff_can_take_job(cargo: u32, pessoa: usize) -> u32 {
    let k = (cargo as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if pessoa == 0 {
        crate::comparison::branch("cpp_4 pessoa nula");
        return 0; // pessoa nula
    }
    let p = pessoa;
    let idade = read_u8_volatile(p + 0x18) as i8;
    let nao_jogador = cpp4_d(p + 0x69);
    let funcao = read_u8_volatile(p + 0x3d);
    let forca = || read_u8_volatile(cpp4_d(p + 0x61) as usize + 0x2f) as i8; // +0x2f do jogador
    // o fim comum (0x4cb9db): não jogador nos cargos 8, 6 ou 5
    let comum = || nao_jogador != 0 && (funcao == 8 || funcao == 6 || funcao == 5);
    let r = match k {
        0 => {
            if idade >= 0x1e {
                if nao_jogador != 0 || idade > 0x24 {
                    return 1;
                }
                if (read_u8_volatile(p + 0x57) as i8) < 0xa && forca() >= 0xc {
                    return 1;
                }
            }
            comum()
        }
        2 => {
            if idade < 0x1e {
                comum()
            } else if nao_jogador == 0 && idade <= 0x24 && (forca() < 0xc || read_u8_volatile(p + 0x57) as i8 >= 0xa) {
                comum()
            } else if funcao == 5 || funcao == 0xa {
                comum()
            } else {
                true
            }
        }
        3 => nao_jogador != 0 && funcao == 9,
        4 => nao_jogador != 0 && (funcao == 0xa || read_u8_volatile(p + 0x28) == 0xa),
        5 => cpp4_d(p + 0x61) != 0,
        _ => {
            // 13 e 15
            let jogador = cpp4_d(p + 0x61) as usize;
            jogador != 0 && (nao_jogador != 0 || idade > 0x22 || (idade >= 0x1e && read_u8_volatile(jogador + 0x2f) as i8 >= 0xc))
        }
    };
    r as u32
}

unsafe extern "stdcall" fn comparar_staff_can_take_job(cargo: u32, pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(%%I1%%)(cargo, pessoa);
    let n = staff_can_take_job(cargo, pessoa);
    PLACAR.record(r == n, || format!("staff_can_take_job(cargo {}, pessoa {:x}) original={} rust={}", cargo as u8 as i8, pessoa, r, n));
    r
}

/// Os dias (0..400) desde a data (dia +0, ano +2 sem sinal) até hoje ([0xae2c90]), com o ano de 365 dias.
unsafe fn contract_manager_days_since_date(dia: usize, ano: usize) -> i32 {
    let hoje = core::ptr::read_unaligned(0xae2c92 as *const u16) as i32;
    let a = core::ptr::read_unaligned(ano as *const u16) as i32;
    let d = (hoje - a) * 365 - cpp4_w(dia) as i32 + cpp4_w(0xae2c90) as i32;
    d.clamp(0, 400)
}

/// A queda da moral (o byte `m` da original) pela partida ruim, ou None quando a rotina sai sem mexer (algum nulo, o
/// técnico ou o auxiliar do clube +0xcf/+0xd3, ou fora do cargo 0xb). x = (nota +0xb − reputação do clube) + 50 × dias
/// no clube (desde +0x3e, até 400) + 417 × idade − 31680; com os dias do contrato (desde +0x2d) acima de 400 (nunca: o
/// teto é 400) e idade 24..31: + 250 × +0x57 + 5000; senão, com +0x22 e a reputação da nação (+0x1a +0x8e) acima de 3500
/// e menos de 32 anos: + 250 × +0x57 + 25 × +0x22 + 2500; senão pela faixa do contrato (+0x4f >> 4): 1: + 250 × +0x57 +
/// 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 × +0x3e; nesses casos no máximo 15000. y = 1 − x × (6,667e-4 se x < 0, senão
/// 6e-5), entre 0,1 e 10; m = ftol((+0x3e + 4) / y + 0,5) (byte). Dupla precisão; as constantes do .rdata.
unsafe fn match_morale_drop(pessoa: usize, clube: usize, contrato: usize) -> Option<i8> {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return None;
    }
    if pessoa == cpp4_d(clube + 0xcf) as usize || pessoa == cpp4_d(clube + 0xd3) as usize || read_u8_volatile(pessoa + 0x3d) != 0xb {
        return None;
    }
    let a = contract_manager_days_since_date(pessoa + 0x3e, pessoa + 0x40);
    let b = contract_manager_days_since_date(contrato + 0x2d, contrato + 0x2f);
    let idade = read_u8_volatile(pessoa + 0x18) as i8;
    let jogador = cpp4_d(pessoa + 0x61) as usize;
    let dif = cpp4_w(jogador + 0xb) as i32 - cpp4_w(clube + 0x80) as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + idade as f64 * read_f64_at(0x968d90)) - read_f64_at(0x968db8);
    let c57 = read_u8_volatile(pessoa + 0x57) as i8 as i32;
    let teto = read_f32_at(0x968da8) as f64;
    let limitar = |x: f64| if x >= teto { teto } else { x };
    let al = read_u8_volatile(pessoa + 0x22);
    if b > 0x190 && idade > 0x17 && idade < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + read_f32_at(0x968db0) as f64); // nunca: b vai até 400
    } else if al != 0 && cpp4_w(cpp4_d(pessoa + 0x1a) as usize + 0x8e) > 0xdac && idade < 0x20 {
        x = limitar(x + (c57 * 250 + al as i32 * 25) as f64 + read_f32_at(0x968dac) as f64);
    } else {
        match (read_u8_volatile(contrato + 0x4f) >> 4) as i8 {
            1 => x = limitar(x + (c57 * 250) as f64 + read_f32_at(0x968db0) as f64),
            2 => x = limitar(x + (c57 * 125) as f64 + read_f32_at(0x968dac) as f64),
            3 => x = limitar(x + (read_u8_volatile(contrato + 0x3e) as i8 as i32 * 1000) as f64),
            _ => {}
        }
    }
    let y = if !(x >= read_f64_at(0x9689a0)) { x * read_f64_at(0x968da0) } else { x * read_f64_at(0x968d98) };
    let mut y = read_f64_at(0x967890) - y;
    if !(y >= read_f64_at(0x967880)) {
        y = read_f64_at(0x967880);
    } else if y > read_f64_at(0x9688b0) {
        y = read_f64_at(0x9688b0);
    }
    let d = (read_u8_volatile(contrato + 0x3e) as i8 as i32 + 4) as f64;
    Some(match_pl_ftol(d / y + read_f64_at(0x968968)) as u8 as i8)
}

/// A moral (+0x44) menos `m`, entre −100 e 100 (a soma em byte quando cabe).
fn lowered_morale(moral: i8, m: i8) -> i8 {
    let menos = m.wrapping_neg();
    let s = moral as i32 + menos as i32;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        moral.wrapping_add(menos)
    }
}

/// O contrato principal da pessoa no CONTRACT_MANAGER global e o clube dele, como a original os refaz (a ficha
/// [0xae1080] gravada); sem contrato o clube é nulo.
unsafe fn club_of_main_contract(pessoa: usize) -> usize {
    let c = if cpp4_d(pessoa) as i32 >= cpp4_d(0xae2364) as i32 {
        0
    } else {
        let k = cpp4_d((cpp4_d(0xae2d0c) as usize).wrapping_add((cpp4_d(pessoa) as usize).wrapping_mul(0x4f))) as i32;
        if k < 0 { 0 } else { (cpp4_d(cpp4_d(0xae1050) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) }
    };
    core::ptr::write_unaligned(0xae1080 as *mut u32, c as u32);
    if c == 0 { 0 } else { (cpp4_d(0xae23b4) as usize).wrapping_add((cpp4_d(c + 4) as usize).wrapping_mul(0x245)) }
}

/// A janela de transferências aberta para o clube do contrato da pessoa: o índice da nação-chave (0x669340 da nação do
/// clube; sem nação, o byte baixo do ponteiro do clube que a original lê da vaga do argumento) dá as duas datas da
/// tabela 0xb63d8a (72 bytes por nação): hoje depois da primeira e antes da segunda.
unsafe fn transfer_window_open(pessoa: usize, clube: usize) -> bool {
    let c1 = club_of_main_contract(pessoa);
    let k = if cpp4_d(c1 + 0x53) != 0 {
        let c2 = club_of_main_contract(pessoa);
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669340usize);
        indice(cpp4_d(c2 + 0x53) as usize) as u8 as i8
    } else {
        // ponytail: a original lê o byte baixo da vaga do argumento `clube` (o ponteiro), não um índice
        clube as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = 0xb63d8ausize + (k as usize) * 0x48;
    let dia = cpp4_w(0xae2c90);
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    let ano_de = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let depois = (dia > cpp4_w(t) && ano == ano_de(t + 2)) || ano > ano_de(t + 2);
    let antes = (dia < cpp4_w(t + 0xf) && ano == ano_de(t + 0x11)) || ano < ano_de(t + 0x11);
    depois && antes
}

/// { 0x4cb250 } (thiscall, ret 0xc, contract_manager.cpp): `contract_manager_lower_match_morale(pessoa, clube,
/// contrato)`: a queda `m` (match_morale_drop). Machucado (0x62c400), emprestado (contrato de empréstimo), aposentando
/// (0x7a7eb0), menor de 18 ou com +0x49: apaga as queixas 4 e 0x10 (0x8674e0). Senão, com m > 5 e a moral − m abaixo de
/// −30 − +0x59: acima de 21 anos, mais forte (+0xb) que o clube e acima de 2250, ou de potencial (+0xd) acima da
/// reputação − 1500 e de 500 com sorteio(+0x57) ≠ 0 e +0x3d negativo: na janela de transferências do clube do contrato,
/// a queixa 4 (0x8674e0 e 0x867700 com 4); no principal (0x4cbb60 = 1), nota acima de 1250, mais de 23 anos e
/// sorteio(+0x59 + 5) = 0: a queixa 0x10; até 21 anos, no principal com sorteio(+0x57) > 5, mais forte que o clube e
/// acima de 1250: a queixa 0x10. Por fim a moral (+0x44) cai m, entre −100 e 100.
unsafe extern "thiscall" fn contract_manager_lower_match_morale(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let Some(m) = match_morale_drop(pessoa, clube, contrato) else {
        return;
    };
    let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let aposentando: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let emprestado = || {
        if cpp4_d(pessoa) as i32 >= cpp4_d(0xae2364) as i32 {
            return false;
        }
        let k = cpp4_d((cpp4_d(0xae2d0c) as usize).wrapping_add((cpp4_d(pessoa) as usize).wrapping_mul(0x4f)) + 4) as i32;
        k >= 0 && (cpp4_d(cpp4_d(0xae1050) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let idade = read_u8_volatile(pessoa + 0x18) as i8;
    if lesao(cpp4_d(0xb63d28) as usize, pessoa) != 0 || emprestado() || aposentando(cpp4_d(0xb7434c) as usize, pessoa) != 0
        || idade < 0x12 || read_u8_volatile(contrato + 0x49) != 0 {
        if cpp4_d(contrato + 0x45) & 4 != 0 {
            apagar(contrato, 4);
        }
        if read_u8_volatile(contrato + 0x45) & 0x10 != 0 {
            apagar(contrato, 0x10);
        }
    } else if m > 5 && (read_u8_volatile(contrato + 0x44) as i8 as i32 - m as i32) < -30 - read_u8_volatile(pessoa + 0x59) as i8 as i32 {
        let jogador = cpp4_d(pessoa + 0x61) as usize;
        if idade > 0x15 {
            let nota = cpp4_w(jogador + 0xb);
            if nota > cpp4_w(clube + 0x80) && nota > 0x8ca {
                if transfer_window_open(pessoa, clube) {
                    crate::comparison::branch("cpp_4 queixa na janela");
                    apagar(contrato, 4);
                    marcar(contrato, 4, 0);
                }
            } else {
                let mut feito = false;
                if lugar(this, pessoa, clube) == 1 && cpp4_w(jogador + 0xb) > 0x4e2 && idade > 0x17
                    && sortear(read_u8_volatile(pessoa + 0x59) as i8 as i32 + 5) == 0 {
                    apagar(contrato, 4);
                    marcar(contrato, 0x10, 0);
                    feito = true;
                }
                if !feito {
                    let pot = cpp4_w(jogador + 0xd);
                    if (pot as i32) > cpp4_w(clube + 0x80) as i32 - 0x5dc && pot > 0x1f4
                        && sortear(read_u8_volatile(pessoa + 0x57) as i8 as i32) != 0 && (read_u8_volatile(contrato + 0x3d) as i8) < 0
                        && transfer_window_open(pessoa, clube) {
                        apagar(contrato, 4);
                        marcar(contrato, 4, 0);
                    }
                }
            }
        } else if lugar(this, pessoa, clube) == 1 && sortear(read_u8_volatile(pessoa + 0x57) as i8 as i32) > 5
            && cpp4_w(jogador + 0xb) > cpp4_w(clube + 0x80) && cpp4_w(jogador + 0xb) > 0x4e2 {
            apagar(contrato, 4);
            marcar(contrato, 0x10, 0);
        }
    }
    let p = (contrato + 0x44) as *mut i8;
    *p = lowered_morale(*p, m); // a moral cai
}

/// Comparador: as queixas (0x867700) sorteiam e podem abrir pedido de transferência: só a original; o plano (a queda
/// `m`, calculada antes, sem efeito) diz a moral +0x44 que ela deixa (as queixas não a gravam) ou que ela não mexe.
unsafe extern "thiscall" fn comparar_contract_manager_lower_match_morale(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I2%%);
    if contrato == 0 {
        return o(this, pessoa, clube, contrato);
    }
    let antes = read_u8_volatile(contrato + 0x44) as i8;
    let plano = match_morale_drop(pessoa, clube, contrato);
    // apagar uma queixa (0x8674e0) recalcula o contrato (0x867240), que pode regravar a moral: com queixa marcada antes
    // ou queda acima de 5 (os ramos que apagam sempre), só a original
    if read_u8_volatile(contrato + 0x45) & 0x14 != 0 || plano.is_some_and(|m| m > 5) {
        return o(this, pessoa, clube, contrato);
    }
    o(this, pessoa, clube, contrato);
    let depois = read_u8_volatile(contrato + 0x44) as i8;
    let esperada = plano.map_or(antes, |m| lowered_morale(antes, m));
    PLACAR.record(depois == esperada, || format!("contract_manager_lower_match_morale(queda {:?}) moral {} -> original {} plano {}", plano, antes, depois, esperada));
}
